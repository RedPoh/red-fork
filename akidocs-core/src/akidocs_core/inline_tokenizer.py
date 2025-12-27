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
        if text[i : i + len(delim)] == delim:
            claimed_by_longer = False
            for check_delim, _ in DELIMITERS:
                if len(check_delim) <= len(delim):
                    continue
                if text[i : i + len(check_delim)] != check_delim:
                    continue
                close = _find_closing(text, check_delim, i + len(check_delim))
                if close != -1:
                    claimed_by_longer = True
                    break

            if not claimed_by_longer:
                return i

        skipped_nested_section = False
        for check_delim, _ in DELIMITERS:
            if check_delim == delim:
                continue
            if text[i : i + len(check_delim)] != check_delim:
                continue

            close = _find_closing(text, check_delim, i + len(check_delim))
            if close != -1:
                i = close + len(check_delim)
                skipped_nested_section = True
                break

        if not skipped_nested_section:
            i += 1

    return -1


def tokenize_inline(
    text: str, inherited_styles: frozenset[Style] = frozenset()
) -> list[InlineText]:
    tokens: list[InlineText] = []
    text_buffer = ""
    i = 0

    while i < len(text):
        found_delimiter = False
        failed_opening_len = 0

        for delim, styles in DELIMITERS:
            if text[i : i + len(delim)] != delim:
                continue

            end = _find_closing(text, delim, i + len(delim))
            if end == -1:
                failed_opening_len = max(failed_opening_len, len(delim))
                continue

            # Reject if closer is within the range a longer delimiter claimed
            if end + len(delim) <= i + failed_opening_len:
                continue

            if text_buffer:
                tokens.append(InlineText(content=text_buffer, styles=inherited_styles))
                text_buffer = ""

            inner_content = text[i + len(delim) : end]
            combined_styles = inherited_styles | styles

            if inner_content:
                inner_tokens = tokenize_inline(inner_content, combined_styles)
                tokens.extend(inner_tokens)
            else:
                tokens.append(InlineText(content="", styles=combined_styles))

            i = end + len(delim)
            found_delimiter = True
            break

        if not found_delimiter:
            text_buffer += text[i]
            i += 1

    if text_buffer:
        tokens.append(InlineText(content=text_buffer, styles=inherited_styles))

    return tokens
