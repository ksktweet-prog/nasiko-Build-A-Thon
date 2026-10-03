use anyhow::Result;
use tiktoken_rs::o200k_base;

pub fn count_tokens(
    text: &str
) -> Result<usize> {

    let bpe =
        o200k_base()?;

    Ok(
       bpe
         .encode_with_special_tokens(
            text
         )
         .len()
    )
}