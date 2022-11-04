pub struct QuizPollOption {
    pub is_correct: bool,
    pub text: String,
}

pub struct QuizPoll {
    pub id: i32,
    pub text: String,
    pub options: Vec<QuizPollOption>,
    pub correct_answer_id: usize,
}
