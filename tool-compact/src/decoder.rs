use anyhow::{anyhow, Result};

use crate::parser::{
    find_call_end,
    parse_call_body,
};

use crate::{
    ToolCall,
    ToolDef,
};

pub fn decode_calls(
    text: &str,
    tools: &[ToolDef],
) -> Result<Vec<ToolCall>>
{
    let mut result = Vec::new();

    let mut pos = 0;

    while let Some(marker) =
        text[pos..].find("<<")
    {

        let start = pos + marker;

        let end =
            find_call_end(
                text,
                start + 2,
            )
            .ok_or_else(|| {
                anyhow!("unterminated call")
            })?;

        let content =
            &text[start+2..end];

        let (
            name,
            arguments
        ) =
            parse_call_body(
                content
            )?;

        if !tools.iter()
            .any(|t| t.name == name)
        {
            return Err(
                anyhow!("unknown tool")
            );
        }

        result.push(
            ToolCall {
                name,
                arguments,
            }
        );

        pos = end + 2;
    }

    Ok(result)
}