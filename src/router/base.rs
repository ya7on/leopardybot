use crate::error::{Error, Result};
use crate::telebot::client::Client;
use crate::telebot::typings::input::Update;
use crate::telebot::typings::output::{BotCommand, ChatType, Message};
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
    fn check_message(
        &self,
        update: &Update,
        expected_command: &String,
        command_scope: &CommandScope,
    ) -> Result<bool> {
        fn check_scope(message: &Message, command_scope: &CommandScope) -> bool {
            match command_scope {
                CommandScope::Any => true,
                CommandScope::PrivateChats => message.chat.chat_type == ChatType::Private,
                CommandScope::GroupChats => {
                    message.chat.chat_type == ChatType::Group
                        || message.chat.chat_type == ChatType::Supergroup
                }
            }
        }

        fn check_command(message: &Message, expected_command: &String) -> Result<bool> {
            if let Some(text) = &message.text {
                let re = Regex::new(r"(/[a-zA-Z0-9_]+)(@.+)?")
                    .map_err(|err| Error::SerializationError(format!("Invalid regex. {}", err)))?;
                if let Some(command) = re.captures(text).map(|c| c[1].to_string()) {
                    return Ok(command == *expected_command);
                }
            }
            Ok(false)
        }

        if let Some(message) = &update.message {
            return Ok(
                check_scope(message, command_scope) && check_command(message, expected_command)?
            );
        }
        Ok(false)
    }

    // TODO что будет при отмене ответа пользователя
    fn check_poll_answer(&self, update: &Update) -> Result<bool> {
        Ok(update.poll_answer.is_some())
    }

    pub fn check(&self, update: &Update) -> Result<bool> {
        match self {
            RouteMatch::Command { command, scope } => self.check_message(update, command, scope),
            RouteMatch::PollAnswer => self.check_poll_answer(update),
        }
    }
}

#[derive(Clone)]
pub struct RouteCfg {
    pub route_match: RouteMatch,
    pub handler: Box<dyn RouteHandler + Send + Sync>,
    pub description: Option<String>,
}

#[derive(Clone, Default)]
pub struct Router {
    routes: Vec<RouteCfg>,
}

impl Router {
    pub fn new() -> Self {
        Self::default()
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
