use olinker::{admin::create::CreateModelRequest, Ollama};
use tokio_stream::StreamExt;

#[ignore]
#[tokio::test]
/// This test needs a Modelfile at /tmp to work
async fn test_create_model_stream() {
    let ollama = Ollama::default();
    
    let mut res = ollama
        .create_model_stream(CreateModelRequest::path(
            "model".into(),
            "/tmp/Modelfile".into(),
        ))
        .await
        .unwrap();

    let mut done = false;
    while let Some(res) = res.next().await {
        match res {
            Ok(res) => {
                dbg!(&res.message);
                if res.message.eq("success") {
                    done = true;
                }
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    assert!(done);
}

#[ignore]
#[tokio::test]
/// This test needs a Modelfile at /tmp to work
async fn test_create_model() {
    let ollama = Ollama::default();

    let res = ollama
        .create_model(CreateModelRequest::path(
            "model".into(),
            "/tmp/Modelfile".into(),
        ))
        .await
        .unwrap();

    assert!(res.message.eq("success"));
}