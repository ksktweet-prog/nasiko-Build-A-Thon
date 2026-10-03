use nasiko_tool_compact::*;

#[test]
fn streaming_split()
{
    let tools = vec![
        ToolDef{
            name:"send_email".into(),
            description:None,
            parameters:None,
        }
    ];

    let mut decoder =
        StreamDecoder::new();

    let a =
        decoder.push_chunk(
            "<<send_",
            &tools,
        ).unwrap();

    assert!(a.is_empty());

    let b =
        decoder.push_chunk(
            r#"email{"to":["a"]}>>"#,
            &tools,
        ).unwrap();

    assert_eq!(
        b.len(),
        1
    );
}