def tokenize(text: str) -> list:
    if text == "":
        return []

    if text.startswith("#"):
        content = text.lstrip("#").strip()
        return [{"type": "header", "level": 1, "content": content}]

    return [{"type": "paragraph", "content": text}]
