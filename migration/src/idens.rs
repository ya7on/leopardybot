use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Player {
    Table,
    TelegramId,
    Score,
}

#[derive(Iden)]
pub enum Quiz {
    Table,
    Id,
    Text,
    CorrectOption,
    Option2,
    Option3,
    Option4,
    Explanation,
}

#[derive(Iden)]
pub enum Chat {
    Table,
    Id,
}

#[derive(Iden)]
pub enum Game {
    Table,
    Id,
    ChatId,
    Active,
    GameMode,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Poll {
    Table,
    Id,
    MessageId,
    GameId,
    CorrectOptionId,
    CloseDate,
    Handled,
}

#[derive(Iden)]
pub enum PlayerPollAnswer {
    Table,
    PlayerId,
    PollId,
    IsCorrect,
}

#[derive(Iden)]
pub enum PlayerPlayedQuiz {
    Table,
    PlayerId,
    QuizId,
}
