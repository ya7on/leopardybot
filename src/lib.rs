#[macro_use]
extern crate log;

pub mod api;
pub mod conf;
pub mod db;
pub mod entities;
pub mod error;
pub mod game {
    pub mod base;
    pub mod chat;
    pub mod poll;
    pub mod question;
    pub mod typings;
}
pub mod job;
pub mod seeder;
pub mod server;
pub mod telebot {
    pub mod client;
    pub mod typings;
}
