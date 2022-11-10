#[macro_use]
extern crate log;

pub mod api;
pub mod router {
    pub mod base;
    pub mod help;
    pub mod play;
    pub mod poll_answer;
    pub mod start;
}
pub mod conf;
pub mod db;
pub mod entities;
pub mod error;
pub mod game {
    pub mod base;
    pub mod chat;
    pub mod player;
    pub mod player_poll_answer;
    pub mod poll;
    pub mod question;
    pub mod typings;
}
pub mod job;
pub mod seeder;
pub mod server;
pub mod texts;
pub mod telebot {
    pub mod client;
    pub mod typings;
}
