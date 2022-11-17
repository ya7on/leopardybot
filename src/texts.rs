use crate::error::Result;
use std::fmt::Write;

pub struct TextFormatter;

impl TextFormatter {
    pub fn round_over() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "<b>Раунд закончился</b> ⌛️",)?;
        writeln!(writer, "Кто не успел, тот опоздал!")?;
        Ok(writer)
    }

    pub fn game_over() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "🏁 <b>Игра окончена</b> 🏁")?;
        writeln!(writer, "Отлично сыграно!")?;
        writeln!(writer)?;
        writeln!(writer, "↩️ /play - сыграть в новую игру")?;
        Ok(writer)
    }

    pub fn new_group_chat() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "Привет всем в этом чате!")?;
        Ok(writer)
    }

    pub fn start() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "<b>Привет!</b> 👋")?;
        writeln!(writer)?;
        writeln!(writer, "Введи /help чтобы узнать все команды")?;
        Ok(writer)
    }

    pub fn help() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "<b>Квиз бот</b> 🧠")?;
        writeln!(writer)?;
        writeln!(writer, "🕹 <b>Режимы игры</b>:")?;
        writeln!(writer, "👉 В личной переписке с ботом новые вопросы будут присылаться по мере того, как вы отвечаете в викторинах")?;
        writeln!(
            writer,
            "👉 В групповых чатах время на ответ ограничено и игра поделена на несколько раундов"
        )?;
        writeln!(writer)?;
        writeln!(writer, "💬 <b>Команды</b>:")?;
        writeln!(writer, "🎮 /play - сыграть в игру")?;
        writeln!(writer, "📕 /help - помощь")?;
        Ok(writer)
    }

    pub fn group_game_already_started() -> Result<String> {
        let mut writer = String::new();
        writeln!(
            writer,
            "<b>Игра уже запущена! 🤔 Дождитесь следующего раунда</b> ⏳"
        )?;
        Ok(writer)
    }

    pub fn single_game_already_started() -> Result<String> {
        let mut writer = String::new();
        writeln!(
            writer,
            "<b>Вы уже играете! 🤔 Ответьте на предыдущий вопрос чтобы перейти к следующему</b> 👆"
        )?;
        writeln!(
            writer,
            "Если вы очистили чат с ботом или по какой-то другой причине не можете найти сообщение с викториной, введите /restart"
        )?;
        Ok(writer)
    }

    pub fn cannot_find_new_quiz() -> Result<String> {
        let mut writer = String::new();
        writeln!(writer, "<b>Новые вопросы закончились 😰</b>")?;
        write!(
            writer,
            "Вы ответили на все доступные <i>на данный момент</i> викторины. "
        )?;
        write!(writer, "Следующую викторину вы, вероятно, уже видели. ")?;
        write!(
            writer,
            "Мы ежедневно добавляем новые вопросы, но вы играете в них быстрее 😁"
        )?;
        Ok(writer)
    }
}
