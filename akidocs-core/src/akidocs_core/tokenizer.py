def tokenize(text: str) -> list:
    if text == "":
        return []

    blocks = text.split("\n\n")
    tokens = []

    for block in blocks:
        block = block.strip()
        if block == "":
            continue

        if block.startswith("#"):
            level = 0
            for char in block:
                if char == "#":
                    level += 1
                else:
                    break
            content = block[level:].strip()
            tokens.append({"type": "header", "level": level, "content": content})
        else:
            tokens.append({"type": "paragraph", "content": block})

    return tokens
