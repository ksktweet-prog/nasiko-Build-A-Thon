#[test]
fn converts_to_openai() {

    let call = ToolCall {
        name:"send_email".into(),
        arguments:json!({
            "to":["a@test.com"]
        })
    };

    let openai =
        call.to_openai(
            "call_1".into()
        ).unwrap();

    assert_eq!(
        openai.function.name,
        "send_email"
    );

    assert!(
        openai
         .function
         .arguments
         .contains("a@test.com")
    );
}