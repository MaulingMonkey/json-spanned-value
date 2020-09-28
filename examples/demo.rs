use json_spanned_value as jsv;

fn main() {
    let text = "{ \"null\": null, \"string\": \"string\", \"number\": 123, \"array\": [1, 2, 3], \"object\": { \"key\": \"value\" }, \"value\": \"asdf\" }";
    let _parsed : jsv::Value = jsv::from_str(text).unwrap();
}
