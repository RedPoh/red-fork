use akidocs_core_rs::tokenizer::tokenize;

#[test]
fn test_empty_string_returns_empty_list() {
    assert_eq!(tokenize(""), vec![]);
}
