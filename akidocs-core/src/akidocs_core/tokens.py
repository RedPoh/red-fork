from dataclasses import dataclass


@dataclass
class Text:
    content: str


@dataclass
class Italic:
    content: str


@dataclass
class Header:
    level: int
    content: str


@dataclass
class Paragraph:
    content: str


Token = Header | Paragraph
InlineToken = Text | Italic
