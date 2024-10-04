use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub category: String,
    pub difficulty: String,
    pub question: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub response_code: i32,
    pub results: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub success: bool,
    pub count: usize,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub category: Option<String>,
    pub difficulty: Option<String>,
    pub amount: usize,
}