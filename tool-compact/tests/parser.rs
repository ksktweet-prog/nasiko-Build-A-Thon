use nasiko_tool_compact::*;

#[test]
fn parses_single_call() {

    let tools = vec![
        ToolDef{
            name:"send_email".into(),
            description:None,
            parameters:None,
        }
    ];

    let text = r#"
<<send_email{
 "to":["a@test.com"]
}>>
"#;

    let calls =
        decode_calls(
            text,
            &tools,
        ).unwrap();

    assert_eq!(
        calls.len(),
        1
    );
}