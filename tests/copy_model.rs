#[tokio::test]
/// This test needs a model named "mario" to work
async fn test_copy_model() {
    let ollama = olinker::Ollama::default();

    ollama
        .copy_model("granite-code:3b".into(), "granite-code-copy".into())
        .await
        .unwrap();
}
