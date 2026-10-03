use anyhow::{anyhow, Result};

pub fn find_call_end(
    input: &str,
    start: usize,
) -> Option<usize> {

    let mut in_string = false;
    let mut escaped = false;

    let bytes = input.as_bytes();

    let mut i = start;

    while i + 1 < bytes.len() {

        let ch = bytes[i] as char;

        if escaped {
            escaped = false;
            i += 1;
            continue;
        }

        match ch {

            '\\' => {
                escaped = true;
            }

            '"' => {
                in_string = !in_string;
            }

            _ => {}
        }

        if !in_string {

            if bytes[i] == b'>'
                && bytes[i + 1] == b'>'
            {
                return Some(i);
            }
        }

        i += 1;
    }

    None
}

pub fn parse_call_body(
    body: &str,
) -> Result<(String, serde_json::Value)>
{
    let brace =
        body.find('{')
            .ok_or_else(|| anyhow!("missing json"))?;

    let name =
        body[..brace].trim();

    if name.is_empty() {
        return Err(anyhow!("missing tool"));
    }

    let json =
        &body[brace..];

    let args =
        serde_json::from_str(json)?;

    Ok((
        name.to_string(),
        args,
    ))
}