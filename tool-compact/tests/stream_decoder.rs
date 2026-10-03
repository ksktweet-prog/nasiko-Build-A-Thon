#[test]
fn chunked_streaming() {

    let mut decoder =
        StreamDecoder::new();

    let _ =
        decoder.push_chunk(
            "<<se",
            &tools(),
        );

    let _ =
        decoder.push_chunk(
            "nd_email{",
            &tools(),
        );

    let _ =
        decoder.push_chunk(
            r#""to":["a@x.com"]"#,
            &tools(),
        );

    let calls =
        decoder.push_chunk(
            "}>>",
            &tools(),
        ).unwrap();

    assert_eq!(
        calls.len(),
        1
    );
}