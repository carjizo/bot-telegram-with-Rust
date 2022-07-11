use dotenv::dotenv;
use frankenstein::api_params::SendPhotoParams;
use frankenstein::api_params::SendVideoParams;
use frankenstein::AsyncTelegramApi;
use frankenstein::GetUpdatesParams;
use frankenstein::Message;
use frankenstein::SendMessageParams;
use frankenstein::{AsyncApi, UpdateContent};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let id: String = env::var("CHAT_ID").expect("CHAT_ID not set");
    // let chat_id: i64 = id.parse().unwrap();

    let token: String = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api: AsyncApi = AsyncApi::new(&token);

    let update_params_builder = GetUpdatesParams::builder();
    let mut update_params = update_params_builder.clone().build();

    loop {
        let result = api.get_updates(&update_params).await;
        println!("result: {:?}", result);

        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        if let Some(text) = message.text.clone() {
                            if text == "Hola" {
                                let api_clone = api.clone();

                                tokio::spawn(async move {
                                    process_message(message, api_clone).await;
                                });

                                update_params = update_params_builder
                                    .clone()
                                    .offset(update.update_id + 1)
                                    .build();
                            } else if text == "foto" {
                                let api_clone = api.clone();
                                let id: String = env::var("CHAT_ID").expect("CHAT_ID not set");
                                let chat_id: i64 = id.parse().unwrap();
                                let file = std::path::PathBuf::from("./photo.jpg");
                                let params = SendPhotoParams::builder()
                                    .chat_id(chat_id)
                                    .photo(file)
                                    .build();
                                match api.send_photo(&params).await {
                                    Ok(response) => {
                                        println!("Photo was uploaded {:?}", response);
                                    }
                                    Err(error) => {
                                        eprintln!("Failed to upload photo: {:?}", error);
                                    }
                                }

                                update_params = update_params_builder
                                    .clone()
                                    .offset(update.update_id + 1)
                                    .build();
                            } else if text == "video" {
                                let api_clone = api.clone();
                                let id: String = env::var("CHAT_ID").expect("CHAT_ID not set");
                                let chat_id: i64 = id.parse().unwrap();
                                let file = std::path::PathBuf::from("./video.mp4");
                                let params = SendVideoParams::builder()
                                    .chat_id(chat_id)
                                    .video(file)
                                    .build();
                                match api.send_video(&params).await {
                                    Ok(response) => {
                                        println!("Photo was uploaded {:?}", response);
                                    }
                                    Err(error) => {
                                        eprintln!("Failed to upload photo: {:?}", error);
                                    }
                                }

                                update_params = update_params_builder
                                    .clone()
                                    .offset(update.update_id + 1)
                                    .build();
                            }
                        }
                    }
                }
            }
            Err(error) => {
                println!("Failed to get updates: {:?}", error);
            }
        }
    }
}

async fn process_message(message: Message, api: AsyncApi) {
    let send_message_params = SendMessageParams::builder()
        .chat_id(message.chat.id)
        .text("hello")
        .reply_to_message_id(message.message_id)
        .build();

    if let Err(err) = api.send_message(&send_message_params).await {
        println!("Failed to send message: {:?}", err);
    }
}
