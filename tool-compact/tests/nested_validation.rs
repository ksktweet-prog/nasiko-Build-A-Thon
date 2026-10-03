use serde_json::json;

#[test]
fn validate_nested_object() {
    let schema = json!({
        "type":"object",
        "properties":{
            "user":{
                "type":"object",
                "properties":{
                    "name":{
                        "type":"string"
                    }
                },
                "required":["name"]
            }
        },
        "required":["user"]
    });

    let valid = json!({
        "user":{
            "name":"john"
        }
    });

    assert!(
        validate_schema(
            &valid,
            &schema
        ).is_ok()
    );
}

#[test]
fn missing_nested_required() {
    let schema = json!({
        "type":"object",
        "properties":{
            "user":{
                "type":"object",
                "properties":{
                    "name":{"type":"string"}
                },
                "required":["name"]
            }
        },
        "required":["user"]
    });

    let invalid = json!({
        "user":{}
    });

    assert!(
        validate_schema(
            &invalid,
            &schema
        ).is_err()
    );
}