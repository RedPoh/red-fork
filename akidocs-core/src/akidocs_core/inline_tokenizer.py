from akidocs_core.tokens import Bold, InlineText, Italic, Style

DELIMITERS: list[tuple[str, frozenset[Style]]] = [
    ("***", frozenset({Bold(), Italic()})),
    ("**", frozenset({Bold()})),
    ("*", frozenset({Italic()})),
]


def _find_closing(text: str, delim: str, start: int) -> int:
    """Find closing delimiter, skipping nested sections."""
    i = start
    while i < len(text):
        # Check for our closing delimiter
        if text[i : i + len(delim)] == delim:
            # Only defer to longer delimiter if it can actually close
            longer_valid = False
            for check_delim, _ in DELIMITERS:
                if len(check_delim) <= len(delim):
                    continue
                if text[i : i + len(check_delim)] != check_delim:
                    continue
                close = _find_closing(text, check_delim, i + len(check_delim))
                if close != -1:
                    longer_valid = True
                    break

            if not longer_valid:
                return i

        # Check for other delimiters to skip
        skipped = False
        for check_delim, _ in DELIMITERS:
            if check_delim == delim:
                continue
            if text[i : i + len(check_delim)] != check_delim:
                continue

            close = _find_closing(text, check_delim, i + len(check_delim))
            if close != -1:
                i = close + len(check_delim)
                skipped = True
                break

        if not skipped:
            i += 1

    return -1


def tokenize_inline(
    text: str, inherited_styles: frozenset[Style] = frozenset()
) -> list[InlineText]:
    tokens: list[InlineText] = []
    current = ""
    i = 0

    while i < len(text):
        matched = False

        for delim, styles in DELIMITERS:
            if text[i : i + len(delim)] != delim:
                continue

            end = _find_closing(text, delim, i + len(delim))
            if end == -1:
                break

            if current:
                tokens.append(InlineText(content=current, styles=inherited_styles))
                current = ""

            inner_content = text[i + len(delim) : end]
            combined_styles = inherited_styles | styles
            inner_tokens = tokenize_inline(inner_content, combined_styles)
            tokens.extend(inner_tokens)

            i = end + len(delim)
            matched = True
            break

        if not matched:
            current += text[i]
            i += 1

    if current:
        tokens.append(InlineText(content=current, styles=inherited_styles))

    return tokens
