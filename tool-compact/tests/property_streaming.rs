use proptest::prelude::*;

proptest! {

#[test]
fn split_at_any_position(
    split in 1usize..30usize
) {

    let tools = vec![
        ToolDef {
            name:"send_email".into(),
            description:None,
            parameters:None,
        }
    ];

    let call =
       r#"<<send_email{"to":["a"]}>>"#;

    let pos =
      split.min(
        call.len()-1
      );

    let mut decoder =
        StreamDecoder::new();

    let _ =
        decoder.push_chunk(
            &call[..pos],
            &tools,
        );

    let result =
        decoder.push_chunk(
            &call[pos..],
            &tools,
        ).unwrap();

    prop_assert_eq!(
        result.len(),
        1
    );
}

}