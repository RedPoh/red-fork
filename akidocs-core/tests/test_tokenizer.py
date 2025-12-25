from akidocs_core.tokenizer import tokenize


def test_empty_string_returns_empty_list():
    assert tokenize("") == []
