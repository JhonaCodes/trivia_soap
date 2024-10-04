use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    category: String,
    difficulty: String,
    question: String,
    correct_answer: String,
    incorrect_answers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    response_code: i32,
    results: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServiceResponse {
    success: bool,
    count: usize,
    questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    category: Option<String>,
    difficulty: Option<String>,
    amount: usize,
}