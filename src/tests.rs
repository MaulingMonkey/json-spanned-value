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



#[test] fn duplicate_keys_allow() {
    let json = "{\"a\": 1, \"a\": 2}";
    let settings = Settings { allow_duplicate_keys: true, ..Settings::default() };
    let o : spanned::Object = from_str_with_settings(json, &settings).unwrap();
    let a = o.get("a").unwrap().as_number().unwrap().as_u64().unwrap();
    assert!(a == 1 || a == 2);
}

#[test] fn duplicate_keys_deny() {
    let json = "{\"a\": 1, \"a\": 2}";
    let settings = Settings { allow_duplicate_keys: false, ..Settings::default() };
    let err = from_str_with_settings::<spanned::Object>(json, &settings).unwrap_err();
    let until_err = &json[..err.offset_within(json).unwrap_or(json.len()-1)];
    assert!(until_err.ends_with("\"a\""), "until_err: {:?}", until_err);
}



#[test] fn trailing_object_commas_allow() {
    let json = "{\"a\": 1, \"b\": 2, }";
    let settings = Settings { allow_trailing_comma: true, ..Settings::default() };
    let o : spanned::Object = from_str_with_settings(json, &settings).unwrap();
    assert_eq!(1, o.get("a").unwrap().as_number().unwrap().as_u64().unwrap());
    assert_eq!(2, o.get("b").unwrap().as_number().unwrap().as_u64().unwrap());
}

#[test] fn trailing_object_commas_deny() {
    let json = "{\"a\": 1, \"b\": 2, }";
    let settings = Settings { allow_trailing_comma: false, ..Settings::default() };
    let err = from_str_with_settings::<spanned::Object>(json, &settings).unwrap_err();
    let until_err = &json[..err.offset_within(json).unwrap_or(json.len()-1)];
    assert!(until_err.ends_with("\"b\": 2, "), "until_err: {:?}", until_err);
}

#[test] fn trailing_array_commas_allow() {
    let json = "[1, 2,]";
    let settings = Settings { allow_trailing_comma: true, ..Settings::default() };
    let a : spanned::Array = from_str_with_settings(json, &settings).unwrap();
    assert_eq!(1, a[0].as_number().unwrap().as_u64().unwrap());
    assert_eq!(2, a[1].as_number().unwrap().as_u64().unwrap());
}

#[test] fn trailing_array_commas_deny() {
    let json = "[1, 2,]";
    let settings = Settings { allow_trailing_comma: false, ..Settings::default() };
    let err = from_str_with_settings::<spanned::Object>(json, &settings).unwrap_err();
    let _until_err = &json[..err.offset_within(json).unwrap_or(json.len()-1)];
    // assert_eq!(_until_err, "["); // Funky enough error location that I'd rather not assert it, in case it's made saner
}



#[test] fn single_line_comments_allow() {
    let json = "{\n    \"a\": 1, // a allow_comments\n    \"b\": 2 // another comment\n}";
    let settings = Settings { allow_comments: true, ..Settings::default() };
    let o : spanned::Object = from_str_with_settings(json, &settings).unwrap();
    assert_eq!(1, o.get("a").unwrap().as_number().unwrap().as_u64().unwrap());
    assert_eq!(2, o.get("b").unwrap().as_number().unwrap().as_u64().unwrap());
}

#[test] fn single_line_comments_deny() {
    let json = "{\n    \"a\": 1, // a comment\n    \"b\": 2 // another comment\n}";
    let settings = Settings { allow_comments: false, ..Settings::default() };
    let err = from_str_with_settings::<spanned::Object>(json, &settings).unwrap_err();
    let until_err = &json[..err.offset_within(json).unwrap_or(json.len()-1)];
    assert!(until_err.ends_with("\"a\": 1, "), "until_err: {:?}", until_err);
}

#[test] fn multi_line_comments_allow() {
    let json = "{\"a\": 1, /* comment */ \"b\": 2}";
    let settings = Settings { allow_comments: true, ..Settings::default() };
    let o : spanned::Object = from_str_with_settings(json, &settings).unwrap();
    assert_eq!(1, o.get("a").unwrap().as_number().unwrap().as_u64().unwrap());
    assert_eq!(2, o.get("b").unwrap().as_number().unwrap().as_u64().unwrap());
}

#[test] fn multi_line_comments_deny() {
    let json = "{\"a\": 1, /* comment */ \"b\": 2}";
    let settings = Settings { allow_comments: false, ..Settings::default() };
    let err = from_str_with_settings::<spanned::Object>(json, &settings).unwrap_err();
    let until_err = &json[..err.offset_within(json).unwrap_or(json.len()-1)];
    assert!(until_err.ends_with("\"a\": 1, "), "until_err: {:?}", until_err);
}



#[test] fn comment_spam_allow() {
    let json = "{\"a\": 1, /*comment*//**//**/// asdf\n//asdf\n//asdf\n/* comment */ \"b\": 2}";
    let settings = Settings { allow_comments: true, ..Settings::default() };
    let o : spanned::Object = from_str_with_settings(json, &settings).unwrap();
    assert_eq!(1, o.get("a").unwrap().as_number().unwrap().as_u64().unwrap());
    assert_eq!(2, o.get("b").unwrap().as_number().unwrap().as_u64().unwrap());
}

#[test] fn comment_spam_after_trailing_comma() {
    let json = "{\"a\": 1, \"b\": 2, /*comment*//**//**/// asdf\n//asdf\n//asdf\n/* comment */ }";
    let settings = Settings { allow_comments: true, allow_trailing_comma: true, ..Settings::default() };
    let o : spanned::Object = from_str_with_settings(json, &settings).unwrap();
    assert_eq!(1, o.get("a").unwrap().as_number().unwrap().as_u64().unwrap());
    assert_eq!(2, o.get("b").unwrap().as_number().unwrap().as_u64().unwrap());
}

#[test] fn not_a_comment() {
    let json = "{\"a\": 1, \"not_a_comment\": \"/*comment*//**//**/// asdf\\n//asdf\\n//asdf\\n/* comment */\", \"b\": 2 }";
    let settings = Settings { allow_comments: true, allow_trailing_comma: true, ..Settings::default() };
    let o : spanned::Object = from_str_with_settings(json, &settings).unwrap();
    assert_eq!(1, o.get("a").unwrap().as_number().unwrap().as_u64().unwrap());
    assert_eq!(2, o.get("b").unwrap().as_number().unwrap().as_u64().unwrap());
    assert_eq!("/*comment*//**//**/// asdf\n//asdf\n//asdf\n/* comment */", o.get("not_a_comment").unwrap().as_string().unwrap());
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



#[test] fn stream_numbers() {
    let json = "1 2 3 "; // FIXME: last span slightly off without trailing space (4..4 instead of 4..5)
    for (expected, actual) in [1,2,3].iter().copied().zip(StreamDeserializer::<&str, spanned::Number>::new(json).into_iter()) {
        let actual = actual.unwrap();
        let actual_str = &json[actual.range()];
        assert_eq!(expected.to_string(), actual_str);
        assert_eq!(expected.to_string(), actual.into_inner().to_string());
    }
}

#[test] fn stream_arrays() {
    let json = "[1] [2] [3]";
    for (expected, actual) in ["[1]","[2]","[3]"].iter().copied().zip(StreamDeserializer::<&str, spanned::Array>::new(json).into_iter()) {
        let actual = actual.unwrap();
        let actual_str = &json[actual.range()];
        assert_eq!(expected.to_string(), actual_str);
        assert_eq!(expected.to_string(), format!("{:?}", actual.into_inner()));
    }
}

#[test] fn stream_objects() {
    let json = "{\"a\":1} {\"b\":2} {\"c\":3}";
    for ((expected_obj, expected_key, expected_val), actual_obj) in [("{\"a\":1}", "a", 1), ("{\"b\":2}", "b", 2), ("{\"c\":3}", "c", 3)].iter().copied().zip(StreamDeserializer::<&str, spanned::Object>::new(json).into_iter()) {
        let actual_obj = actual_obj.unwrap();

        assert_eq!(expected_obj, &json[actual_obj.range()]);

        for (actual_key, actual_val) in actual_obj.into_iter() {
            assert_eq!(&json[actual_key.range()], format!("{:?}", expected_key));
            assert_eq!(&json[actual_val.range()], format!("{:?}", expected_val));
            assert_eq!(expected_key, actual_key.into_inner());
            assert_eq!(expected_val, actual_val.as_number().unwrap().as_u64().unwrap());
        }
    }
}
