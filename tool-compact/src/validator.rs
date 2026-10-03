use anyhow::{anyhow, Result};
use serde_json::Value;

use crate::{ToolCall, ToolDef};

pub fn validate_call(
    call: &ToolCall,
    tools: &[ToolDef],
) -> Result<()> {
    let tool = tools
        .iter()
        .find(|t| t.name == call.name)
        .ok_or_else(|| anyhow!("unknown_tool"))?;

    let schema = tool
        .parameters
        .as_ref()
        .ok_or_else(|| anyhow!("missing schema"))?;

    validate_schema(
        &call.arguments,
        schema,
    )
}

fn validate_schema(
    args: &Value,
    schema: &Value,
) -> Result<()> {
    let obj = args
        .as_object()
        .ok_or_else(|| anyhow!("arguments must be object"))?;

    if let Some(required) = schema["required"].as_array() {
        for field in required {
            let field = field.as_str().unwrap();

            if !obj.contains_key(field) {
                return Err(anyhow!("missing field"));
            }
        }
    }

    if let Some(props) = schema["properties"].as_object() {
        for (name, field_schema) in props {
            if let Some(value) = obj.get(name) {
                validate_value(
                    value,
                    field_schema,
                )?;
            }
        }
    }

    Ok(())
}

fn validate_value(
    value: &Value,
    schema: &Value,
) -> Result<()> {
    if let Some(enums) = schema["enum"].as_array() {
        if !enums.contains(value) {
            return Err(anyhow!("invalid enum"));
        }
        return Ok(());
    }

    match schema["type"].as_str() {
        Some("string") if value.is_string() => Ok(()),

        Some("integer")
            if value.is_i64() || value.is_u64() =>
        {
            Ok(())
        }

        Some("number") if value.is_number() => Ok(()),

        Some("boolean") if value.is_boolean() => Ok(()),

        Some("array") if value.is_array() => Ok(()),

        Some("object") if value.is_object() => Ok(()),

        Some(_) => Err(anyhow!("type mismatch")),

        None => Ok(()),
    }
}