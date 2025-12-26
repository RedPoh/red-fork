from akidocs_core.tokens import InlineToken, Italic, Text


def tokenize_inline(text: str) -> list[InlineToken]:
    tokens = []
    current = ""
    i = 0

    while i < len(text):
        if text[i] == "*":
            if current:
                tokens.append(Text(content=current))
                current = ""

            # Find closing *
            end = text.find("*", i + 1)
            if end != -1:
                tokens.append(Italic(content=text[i + 1 : end]))
                i = end + 1
            else:
                current += text[i]
                i += 1
        else:
            current += text[i]
            i += 1

    if current:
        tokens.append(Text(content=current))

    return tokens
