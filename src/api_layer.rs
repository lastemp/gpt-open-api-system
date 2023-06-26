use crate::Choice;
use crate::CompletionRequest;
use crate::CompletionResponse;
use crate::Message;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;

pub async fn generate_message_response(
    completion_request: CompletionRequest,
) -> std::result::Result<String, reqwest::Error> {
    let api_key: String =
        String::from("Bearer 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");
    let api_url: String = String::from("https://api.openai.com/v1/chat/completions");
    let mut message_content = String::from("");

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_key)
        .json(&completion_request)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            println!("server not responding");
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let a = String::from(""); //Default value.
                    let b: Vec<Choice> = Vec::new();
                    let c = Message {
                        role: String::from(""),
                        content: Some(String::from("")),
                    };

                    let my_output = response.json::<CompletionResponse>().await?;
                    let _id = &my_output.id.as_ref().unwrap_or(&a);
                    let _choices = &my_output.choices.as_ref().unwrap_or(&b);

                    if _id.len() > 0 && _choices.len() > 0 {
                        let _message = _choices[0].message.as_ref().unwrap_or(&c);
                        let _content = _message.content.as_ref().unwrap_or(&a);
                        message_content = _content.to_string();
                    }
                }
                s => println!("Received response status: {:?}", s),
            }
        }
    };

    Ok(message_content)
}
