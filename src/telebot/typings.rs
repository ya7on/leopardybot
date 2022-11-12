use serde::Deserialize;

pub mod input {
    use super::*;

    #[derive(Debug, Deserialize)]
    pub struct Update {
        pub update_id: usize,
        pub message: Option<output::Message>,
        pub poll: Option<output::Poll>,
        pub poll_answer: Option<output::PollAnswer>,
    }
}

pub mod output {
    use super::*;
    use serde::Serialize;

    #[derive(Debug, Deserialize, PartialEq)]
    pub enum ChatType {
        #[serde(rename = "private")]
        Private,
        #[serde(rename = "group")]
        Group,
        #[serde(rename = "supergroup")]
        Supergroup,
        #[serde(rename = "channel")]
        Channel,
    }

    #[derive(Debug, Deserialize)]
    pub struct User {
        pub id: isize,
    }

    #[derive(Debug, Deserialize)]
    pub struct Chat {
        pub id: isize,
        #[serde(rename = "type")]
        pub chat_type: ChatType,
    }

    #[derive(Debug, Deserialize)]
    pub struct PollOption {
        pub text: String,
        pub voter_count: usize,
    }

    #[derive(Debug, Deserialize)]
    pub struct PollAnswer {
        pub poll_id: String,
        pub user: User,
        pub option_ids: Vec<usize>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Poll {
        pub id: String,
        pub question: String,
        pub options: Vec<PollOption>,
        pub total_voter_count: usize,
        pub is_closed: bool,
        pub correct_option_id: Option<usize>,
        pub open_period: Option<usize>,
        pub close_date: Option<usize>,
    }

    #[derive(Debug, Deserialize)]
    pub struct Message {
        pub message_id: usize,
        pub from: Option<User>,
        pub chat: Chat,
        pub text: Option<String>,
        pub poll: Option<Poll>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BotCommand {
        pub command: String,
        pub description: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct WebhookInfo {
        pub url: String,
    }
}
