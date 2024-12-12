use olinker::Ollama;

#[tokio::test]
/// This test needs a model named "mario_copy" to work
async fn test_delete_model() {
    let ollama = Ollama::default();

    ollama.delete_model("granite-code-copy".into()).await.unwrap();
}
