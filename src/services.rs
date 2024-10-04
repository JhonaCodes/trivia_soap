use actix_web::{web,Responder, HttpResponse};
use reqwest::Client as HttpClient;
use crate::{ApiResponse, QueryParams, ServiceResponse};

pub async fn get_questions( http_client: web::Data<HttpClient>, query: web::Query<QueryParams>, ) -> impl Responder {
    let url = format!( "https://opentdb.com/api.php?amount={}&category={}&difficulty={}",
        query.amount,
        query.category.as_deref().unwrap_or(""),
        query.difficulty.as_deref().unwrap_or("")
    );

    match http_client.get(&url).send().await {
        Ok(response) => {
            match response.json::<ApiResponse>().await {
                Ok(trivia_response) => {
                    let service_response = ServiceResponse {
                        success: trivia_response.response_code == 0,
                        count: trivia_response.results.len(),
                        questions: trivia_response.results,
                    };
                    HttpResponse::Ok().json(service_response)
                },
                Err(_) => HttpResponse::InternalServerError().json(ServiceResponse {
                    success: false,
                    count: 0,
                    questions: vec![],
                }),
            }
        },
        Err(_) => HttpResponse::InternalServerError().json(ServiceResponse {
            success: false,
            count: 0,
            questions: vec![],
        }),
    }
}