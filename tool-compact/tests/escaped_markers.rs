use nasiko_tool_compact::*;

#[test]
fn escaped_marker_inside_json()
{
    let tools = vec![
        ToolDef{
            name:"send_email".into(),
            description:None,
            parameters:None,
        }
    ];

    let text = r#"
<<send_email{
 "body":"hello >> world"
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

    assert_eq!(
        calls[0]
            .arguments["body"],
        "hello >> world"
    );
}