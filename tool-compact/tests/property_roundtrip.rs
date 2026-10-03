use proptest::prelude::*;

proptest! {

#[test]
fn arbitrary_titles(
    title in "[a-zA-Z]{1,30}"
) {

let text = format!(
r#"<<create_calendar_event{{"title":"{}","start":"2026-10-05"}}>>"#,
title
);

let result =
    decode_calls(
      &text,
      &tools()
    );

prop_assert!(
    result.is_ok()
);

}

}