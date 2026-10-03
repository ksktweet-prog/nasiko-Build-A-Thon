use anyhow::Result;
use serde_json::Value;

use crate::{CompactTools, ToolDef};

pub fn encode_tools(tools: &[ToolDef]) -> Result<CompactTools> {
    let mut out = String::new();

    for tool in tools {
        out.push_str("@tool ");
        out.push_str(&tool.name);
        out.push('(');

        if let Some(schema) = &tool.parameters {
            render_schema(schema, &mut out)?;
        }

        out.push_str(")\n");
    }

    Ok(CompactTools { rendered: out })
}

fn render_schema(schema: &Value, out: &mut String) -> Result<()> {
    let required = schema["required"]
        .as_array()
        .cloned()
        .unwrap_or_default();

    let props = match schema["properties"].as_object() {
        Some(p) => p,
        None => return Ok(()),
    };

    let mut first = true;

    for (name, field) in props {
        if !first {
            out.push_str(", ");
        }

        first = false;

        let req = required.iter().any(|v| v.as_str() == Some(name));

        out.push_str(name);

        if !req {
            out.push('?');
        }

        out.push(':');

        out.push_str(&field_type(field));
    }

    Ok(())
}

fn field_type(field: &Value) -> String {
    if let Some(enums) = field["enum"].as_array() {
        return enums
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<_>>()
            .join("|");
    }

    match field["type"].as_str() {
        Some("string") => "str".into(),
        Some("integer") => "int".into(),
        Some("number") => "num".into(),
        Some("boolean") => "bool".into(),

        Some("array") => {
            let inner = field_type(&field["items"]);
            format!("[{}]", inner)
        }

        Some("object") => "obj".into(),

        _ => "unknown".into(),
    }
}