import typing


__version__: str


class Cell:
    def is_default(self) -> bool: ...
    @property
    def char(self) -> str: ...
    @property
    def width(self) -> int: ...
    @property
    def pen(self) -> Pen: ...
    def set(self, ch: str, width: int, pen: Pen) -> None: ...


class Charset:
    Ascii: typing.ClassVar[Charset]
    Drawing: typing.ClassVar[Charset]
    
    def translate(self, ch: str) -> str: ...


class Color:
    class RGB:
        def __init__(self, r: int, g: int, b: int) -> None: ...
        def __getitem__(self, index: typing.Literal[0, 1, 2]) -> int: ...
    
    class Indexed:
        def __init__(self, index: int) -> None: ...
        def __getitem__(self, index: typing.Literal[0]) -> int: ...


class Line:
    def __len__(self) -> int: ...
    def is_empty(self) -> bool: ...
    @property
    def cells(self) -> list[Cell]: ...
    def chunks(self, predicate: typing.Callable[[Cell, Cell], bool]) -> list[list[Cell]]: ...
    @property
    def text(self) -> str: ...


class Pen:
    pass


class Vt:
    pass


