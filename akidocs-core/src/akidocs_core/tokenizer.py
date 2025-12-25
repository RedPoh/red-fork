def tokenize(text: str) -> list:
    if text == "":
        return []
    return [{"type": "paragraph", "content": text}]
