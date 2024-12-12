use base64::Engine;
use tokio_stream::StreamExt;

use olinker::{
    generation::{
        chat::{request::ChatMessageRequest, ChatMessage},
        images::Image,
    },
    Ollama,
};

#[allow(dead_code)]
const PROMPT: &str = "Why is the sky blue?";

#[tokio::test]
async fn test_send_chat_messages_stream() {
    let ollama = Ollama::default();

    let messages = vec![ChatMessage::user(PROMPT.to_string())];
    let mut res = ollama
        .send_chat_messages_stream(ChatMessageRequest::new(
            "llama3.2:latest".to_string(),
            messages,
        ))
        .await
        .unwrap();

    let mut done = false;
    while let Some(res) = res.next().await {
        let res = res.unwrap();
        dbg!(&res);
        if res.done {
            done = true;
            break;
        }
    }

    assert!(done);
}

#[tokio::test]
async fn test_send_chat_messages() {
    let ollama = Ollama::default();

    let messages = vec![ChatMessage::user(PROMPT.to_string())];
    let res = ollama
        .send_chat_messages(ChatMessageRequest::new(
            "llama3.2:latest".to_string(),
            messages,
        ))
        .await
        .unwrap();
    dbg!(&res);

    assert!(res.done);
}

const IMAGE_URL: &str = "https://images.pexels.com/photos/1054655/pexels-photo-1054655.jpeg";

#[tokio::test]
async fn test_send_chat_messages_with_images() {
    let ollama = Ollama::default();

    let bytes = reqwest::get(IMAGE_URL)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    let base64 = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let image = Image::from_base64(&base64);

    let messages =
        vec![ChatMessage::user("What can we see in this image?".to_string()).add_image(image)];
    let res = ollama
        .send_chat_messages(ChatMessageRequest::new(
            "llava:latest".to_string(),
            messages,
        ))
        .await
        .unwrap();
    dbg!(&res);

    assert!(res.done);
}
