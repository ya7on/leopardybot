use crate::error::{Error, Result};
use std::fmt::Write;

pub struct TextFormatter;

impl TextFormatter {
    pub fn round_over() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "<b>Раунд закончился</b>")
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        Ok(writer)
    }

    pub fn game_over() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "<b>Игра окончена</b>")
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        writeln!(writer)
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        writeln!(writer, "/play - сыграть в новую игру")
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        Ok(writer)
    }

    pub fn new_chat() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "Привет всем в этом чате")
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        Ok(writer)
    }

    pub fn help() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "<b>Квиз бот</b>")
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        writeln!(writer)
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        writeln!(writer, "/play - сыграть в игру")
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        writeln!(writer, "/help - помощь")
            .map_err(|err| Error::SerializationError(format!("Cannot write line. {}", err)))?;
        Ok(writer)
    }
}