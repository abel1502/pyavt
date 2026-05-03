# PyAVT - Python bindings for the AVT virtual terminal emulator

[![PyPI version](https://img.shields.io/pypi/v/pyavt.svg)](https://pypi.org/project/pyavt/)
[![Python versions](https://img.shields.io/pypi/pyversions/pyavt.svg)](https://pypi.org/project/pyavt/)
[![Downloads](https://pepy.tech/badge/pyavt)](https://pepy.tech/project/pyavt)
[![CI](https://github.com/abel1502/pyavt/actions/workflows/CI.yml/badge.svg)](https://github.com/abel1502/pyavt)
[![Type hints](https://img.shields.io/badge/typing-typed-blue.svg)](https://peps.python.org/pep-0484/)
[![License](https://img.shields.io/pypi/l/pyavt.svg)](LICENSE)

This project is a thin wrapper around [AVT](https://github.com/asciinema/avt),
a virtual terminal emulator developed by asciinema.

## Installation

Using pip:

```bash
pip install pyavt
```

## Usage

The package exposes `avt`, `avt.parser`, `avt.terminal` and `avt.util` modules,
mirroring the underlying AVT API. The original is mostly undocumented, and so
is this wrapper (PRs with documentation are welcome). Refer to the type hints
and the source code for details.

In general, the high-level API is `avt.Vt`, which implements the actual virtual
terminal, and `avt.util.TextCollector`, which collects the plain text output
of a `Vt` with proper line unwrapping. `avt.parser.Parser` recognizes the
ANSI escape codes and converts them to `avt.terminal.Function`s for you to
process. `avt.terminal.Terminal` implements the virtual terminal sans actual
parser, if for some reason you want that.

```python
import avt

vt = avt.Vt()

# Changed line numbed, and the contents of the lines reappearing when the terminal is scrolled
lines, scrollback = vt.feed_str("Hello!\r\n")
print(lines, scrollback, vt.cursor)

lines, scrollback = vt.feed_str("\033[0;31;40mCOOL RED TEXT\033[0m\r\n")
print(lines, scrollback, vt.cursor)

# You can access individual lines and cells within them
print(vt[1][3])

for i in range(25):
    # Reminder that raw terminals treat \n as "go down one line, keep the horizontal cursor where it was"
    lines, scrollback = vt.feed_str(f"{i}\n")
    print(lines, scrollback, vt.cursor)

# You don't have to care about them though
vt.feed_str("Alright, how does it look?")
# And the single-char version reports nothing anyway
vt.feed(".")
vt.feed(".")
vt.feed(".")
vt.feed_str("\r\n")


# All lines, including those out of frame
print(vt.lines())
# Lines visible on the screen
print(vt.view())
# All lines, but only the string values (no color, etc. metadata)
print(vt.text())
# All raw inputs to the terminal
print(repr(vt.dump()))
```

```python
import avt
from avt.util import TextCollector

tc = TextCollector(avt.Vt())

tc.feed_str("Hello, world!\r\n")
tc.feed_str("This one's ")
tc.feed_str("l" + "o" * 100 + "ng\r\n")

print(tc.flush())
# ['Hello, world!', "This one's loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong"]
```

No automated tests exist yet. Report any issues you find.

## License

© 2026 Andrey Belyaev

In accordance with AVT's license, the code is distributed under the Apache License, Version 2.0.
