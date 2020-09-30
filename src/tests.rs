use super::*;
use std::collections::BTreeMap;
use serde::Deserialize;

fn do_test_obj(json: &str, expected: Vec<(&str, &str, fn(&Value) -> bool)>) {
    let expected = expected.into_iter().map(|(k, v, v2)| (k, (v, v2))).collect::<BTreeMap<_, _>>();
    let parsed : Value = super::from_str(json).unwrap();
    for (key, val) in parsed.into_object().unwrap() {
        let key_raw = json.get(key.start .. key.end).unwrap_or_else(|| panic!("Unable to fetch key_raw for key: {:?}", key));
        let val_raw = json.get(val.start .. val.end).unwrap_or_else(|| panic!("Unable to fetch val_raw for val: {:?}", val));
        let (expected_raw_val, expected_val) = expected.get(key.as_str()).unwrap_or_else(|| panic!("Did not expect key: {:?}", key));

        assert_eq!(key_raw, format!("{:?}", key));
        assert_eq!(val_raw, *expected_raw_val);
        assert!(expected_val(&val), "Failed to match condition for val: {:?}", val);
    }
}

#[test] fn object_no_whitespace() {
    do_test_obj("{\"null\":null,\"string\":\"string\\nwith\\nescapes\",\"number\":123,\"array\":[1,2,3],\"object\":{\"key\":\"value\",\"key2\":\"value2\"}}", vec![
        ("null",    "null",                                     |v| v.is_null()),
        ("string",  "\"string\\nwith\\nescapes\"",              |v| v.as_string() == Some("string\nwith\nescapes")),
        ("number",  "123",                                      |v| v.as_number().and_then(|n| n.as_u64()) == Some(123)),
        ("array",   "[1,2,3]",                                  |v| v.as_array().map(|a| a.len()) == Some(3)),
        ("object",  "{\"key\":\"value\",\"key2\":\"value2\"}",  |v| v.as_object().map(|o| o.len()) == Some(2)),
    ]);
}

#[test] fn object_more_whitespace() {
    do_test_obj("{  \"null\"  :  null  ,  \"string\"  :  \"string\\nwith\\nescapes\"  ,  \"number\"  :  123  ,  \"array\"  :  [1,2,3]  ,  \"object\"  :  {\"key\":\"value\",\"key2\":\"value2\"}}", vec![
        ("null",    "null",                                     |v| v.is_null()),
        ("string",  "\"string\\nwith\\nescapes\"",              |v| v.as_string() == Some("string\nwith\nescapes")),
        ("number",  "123",                                      |v| v.as_number().and_then(|n| n.as_u64()) == Some(123)),
        ("array",   "[1,2,3]",                                  |v| v.as_array().map(|a| a.len()) == Some(3)),
        ("object",  "{\"key\":\"value\",\"key2\":\"value2\"}",  |v| v.is_object()),
    ]);
}

#[test] fn object_even_more_whitespace() {
    do_test_obj("{  \"  null  \"  :  null  ,  \"  string  \"  :  \"  string\\nwith\\nescapes  \"  ,  \"  number  \"  :  123  ,  \"  array  \"  :  [  1  ,  2  ,  3  ]  ,  \"  object  \"  :  {  \"  key  \"  :  \"  value  \"  ,  \"  key2  \"  :  \"  value2  \"  }  }", vec![
        ("  null  ",    "null",                                 |v| v.is_null()),
        ("  string  ",  "\"  string\\nwith\\nescapes  \"",      |v| v.as_string() == Some("  string\nwith\nescapes  ")),
        ("  number  ",  "123",                                  |v| v.as_number().and_then(|n| n.as_u64()) == Some(123)),
        ("  array  ",   "[  1  ,  2  ,  3  ]",                  |v| v.as_array().map(|a| a.len()) == Some(3)),
        ("  object  ",  "{  \"  key  \"  :  \"  value  \"  ,  \"  key2  \"  :  \"  value2  \"  }",  |v| v.as_object().map(|o| o.len()) == Some(2)),
    ]);
}

#[test] fn struct_plain() {
    #[allow(dead_code)] #[derive(Deserialize)] struct Plain {
        null:   (),
        string: String,
        number: serde_json::Number,
        array:  Vec<Value>,
        object: Map<Spanned<String>, spanned::Value>,
    }

    let json = "{\"null\":null,\"string\":\"string\\nwith\\nescapes\",\"number\":123,\"array\":[1,2,3],\"object\":{\"key\":\"value\",\"key2\":\"value2\"}}";
    let _parsed1 : Plain            = super::from_str(json).unwrap();
    let _parsed2 : Spanned<Plain>   = super::from_str(json).unwrap();
}

#[test] fn struct_annotated() {
    #[allow(dead_code)] #[derive(Deserialize)] struct Annotated {
        null:   spanned::Null,
        string: spanned::String,
        number: spanned::Number,
        array:  spanned::Array,
        object: spanned::Object,
    }

    let json = "{\"null\":null,\"string\":\"string\\nwith\\nescapes\",\"number\":123,\"array\":[1,2,3],\"object\":{\"key\":\"value\",\"key2\":\"value2\"}}";
    let _parsed1 : Annotated            = super::from_str(json).unwrap();
    let _parsed2 : Spanned<Annotated>   = super::from_str(json).unwrap();
}
