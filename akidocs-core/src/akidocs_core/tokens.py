from dataclasses import dataclass


@dataclass
class Text:
    content: str


@dataclass
class Italic:
    content: str


InlineToken = Text | Italic


@dataclass
class Header:
    level: int
    content: list[InlineToken]


@dataclass
class Paragraph:
    content: list[InlineToken]


Token = Header | Paragraph
