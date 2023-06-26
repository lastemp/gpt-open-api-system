mod api_layer;

use actix_web::{get, http, post, web, App, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::str;

#[derive(Deserialize)]
struct CompletionData {
    user_message: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Choice {
    pub index: u8,
    pub message: Option<Message>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct CompletionResponse {
    pub id: Option<String>,
    pub object: String,
    pub created: u32,
    pub choices: Option<Vec<Choice>>,
    pub usage: Usage,
}

#[derive(Serialize)]
struct UserResponseData {
    user_response: String,
}

// We define the model to use
const MODEL: &str = "gpt-3.5-turbo";

#[get("/")]
async fn index() -> impl Responder {
    format!("")
}

#[post("/processmessage")]
async fn process_message(
    confirmation_data: web::Json<CompletionData>,
    //data: web::Data<String>,
) -> impl Responder {
    let mut _messages: Vec<Message> = Vec::new();

    let input_message = &confirmation_data.user_message;
    let input_message = input_message.to_vec();

    let system_message: Message = Message {
        role: String::from("system"),
        content: Some(String::from("You are a helpful assistant.")),
    };
    _messages.push(system_message);

    if input_message.len() > 0 {
        for my_message in input_message.iter() {
            if my_message.replace(" ", "").trim().len() > 0 {
                let user_message: Message = Message {
                    role: String::from("user"),
                    content: Some(my_message.to_string()),
                };
                _messages.push(user_message);
            }
        }
    }

    let valid_data = {
        if _messages.len() > 1 {
            true
        } else {
            false
        }
    };

    if !valid_data {
        let response_data = UserResponseData {
            user_response: String::from("user_message is empty"),
        };
        return (
            web::Json(response_data),
            http::StatusCode::INTERNAL_SERVER_ERROR,
        );
    }

    let completion_request = CompletionRequest {
        model: MODEL.to_string(),
        messages: _messages,
    };

    let xy = tokio::spawn(async move {
        // Process each request concurrently.
        let _content = api_layer::generate_message_response(completion_request).await;
        let message_content: String = match _content {
            Ok(a) => a,
            Err(e) => String::from(""),
        };
        return message_content;
    });

    let message_content: String = match xy.await {
        Ok(a) => a,
        Err(e) => String::from(""),
    };
    let response_data = UserResponseData {
        user_response: message_content,
    };

    (web::Json(response_data), http::StatusCode::OK)
}

#[actix_web::main]
async fn main() {
    // get env vars
    dotenv().ok();
    let server_addr = env::var("SERVER_ADDR").expect("SERVER_ADDR is not set in .env file");
    /*
    let gpt_open_api_key =
        env::var("GPT_OPEN_API_KEY").expect("GPT_OPEN_API_KEY is not set in .env file");
    */
    let mut http_server_status = String::from("[info] ActixWebHttpServer - Listening for HTTP on ");

    //let shared_data = web::Data::new(shared_data);

    http_server_status.push_str(&server_addr);

    let server = match HttpServer::new(move || {
        App::new()
            //.app_data(shared_data.clone())
            .service(index)
            .service(process_message)
    })
    .bind(server_addr)
    {
        Ok(s) => {
            println!("{:?}", http_server_status);
            s
        }
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };

    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
}
