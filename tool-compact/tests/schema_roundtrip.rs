#[test]
fn schema_roundtrip() {

    let tools = vec![
        example_tool()
    ];

    let compact =
        encode_tools(
            &tools
        ).unwrap();

    let decoded =
        decode_tools(
            &compact
        ).unwrap();

    assert_eq!(
        tools[0].name,
        decoded[0].name
    );
}