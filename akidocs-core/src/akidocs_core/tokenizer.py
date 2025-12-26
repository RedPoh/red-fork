from akidocs_core.inline_tokenizer import tokenize_inline
from akidocs_core.tokens import Header, Paragraph, Token


def tokenize(text: str) -> list[Token]:
    text = text.replace("\r\n", "\n")

    if text == "":
        return []

    blocks = text.split("\n\n")
    tokens = []

    for block in blocks:
        block = block.strip()
        if block == "":
            continue

        if block.startswith("#"):
            stripped = block.lstrip("#")
            level = len(block) - len(stripped)
            if level <= 6 and (stripped == "" or stripped.startswith(" ")):
                content = stripped.strip()
                tokens.append(Header(level=level, content=tokenize_inline(content)))
            else:
                tokens.append(Paragraph(content=tokenize_inline(block)))
        else:
            tokens.append(Paragraph(content=tokenize_inline(block)))

    return tokens
