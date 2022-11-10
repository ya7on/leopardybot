use crate::error::{Error, Result};
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::telebot::typings::output::BotCommand;
use regex::Regex;
use sea_orm::DatabaseConnection;

pub struct CommandsList {
    pub private_chats: Vec<BotCommand>,
    pub group_chats: Vec<BotCommand>,
    pub group_administrators: Vec<BotCommand>,
}

pub trait RouteClone {
    fn clone_box(&self) -> Box<dyn RouteHandler + Send + Sync>;
}

impl<T> RouteClone for T
where
    T: 'static + RouteHandler + Clone + Send + Sync,
{
    fn clone_box(&self) -> Box<dyn RouteHandler + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn RouteHandler + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[async_trait::async_trait]
pub trait RouteHandler: RouteClone {
    async fn handle(&self, db: &DatabaseConnection, client: &Client, update: &Update)
        -> Result<()>;
}

#[derive(Clone)]
pub enum CommandScope {
    Any,
    PrivateChats,
    GroupChats,
    AllChatAdministrators,
}

#[derive(Clone)]
pub enum RouteMatch {
    Command {
        command: String,
        scope: CommandScope,
    },
    PollAnswer,
}

impl RouteMatch {
    fn parse_command(text: &str) -> Result<Option<String>> {
        let re = Regex::new(r"(/[a-zA-Z0-9_]+)(@.+)?")
            .map_err(|err| Error::SerializationError(format!("Invalid regex. {}", err)))?;
        Ok(re.captures(text).map(|c| c[1].to_string()))
    }

    /// FIXME сделать как-то красивее
    pub fn check(&self, update: &Update) -> Result<bool> {
        match self {
            RouteMatch::Command { command, .. } => {
                if let Update {
                    message: Some(message),
                    ..
                } = update
                {
                    if let Some(text) = &message.text {
                        if let Some(c) = Self::parse_command(text)? {
                            return Ok(c == command.to_owned());
                        }
                    }
                }
            }
            RouteMatch::PollAnswer => {
                if update.poll_answer.is_some() {
                    return Ok(true);
                }
            }
        };
        Ok(false)
    }
}

#[derive(Clone)]
pub struct RouteCfg {
    pub route_match: RouteMatch,
    pub handler: Box<dyn RouteHandler + Send + Sync>,
    pub description: Option<String>,
}

#[derive(Clone)]
pub struct Router {
    routes: Vec<RouteCfg>,
}

impl Router {
    pub fn new() -> Self {
        Self { routes: vec![] }
    }

    pub fn register(mut self, command: RouteCfg) -> Router {
        self.routes.push(command);
        self
    }

    pub async fn handle(
        &self,
        update: &Update,
        db: &DatabaseConnection,
        client: &Client,
    ) -> Result<()> {
        for route in self.routes.iter() {
            if route.route_match.check(update)? {
                route.handler.handle(db, client, update).await?;
            }
        }
        Ok(())
    }

    pub fn list_commands(&self) -> Result<CommandsList> {
        let mut commands = CommandsList {
            private_chats: Vec::new(),
            group_chats: Vec::new(),
            group_administrators: Vec::new(),
        };

        for command in self.routes.iter() {
            if let Some(description) = &command.description {
                if let RouteMatch::Command {
                    command: command_text,
                    scope,
                } = &command.route_match
                {
                    match scope {
                        CommandScope::Any => {
                            commands.private_chats.push(BotCommand {
                                command: command_text.to_owned(),
                                description: description.clone(),
                            });
                            commands.group_chats.push(BotCommand {
                                command: command_text.to_owned(),
                                description: description.clone(),
                            });
                            commands.group_administrators.push(BotCommand {
                                command: command_text.to_owned(),
                                description: description.clone(),
                            });
                        }
                        CommandScope::PrivateChats => {
                            commands.private_chats.push(BotCommand {
                                command: command_text.to_owned(),
                                description: description.clone(),
                            });
                        }
                        CommandScope::GroupChats => {
                            commands.group_chats.push(BotCommand {
                                command: command_text.to_owned(),
                                description: description.clone(),
                            });
                        }
                        CommandScope::AllChatAdministrators => {
                            commands.group_administrators.push(BotCommand {
                                command: command_text.to_owned(),
                                description: description.clone(),
                            });
                        }
                    }
                }
            }
        }

        Ok(commands)
    }
}
