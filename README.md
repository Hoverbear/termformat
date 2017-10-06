# termformat

[![Build Status](https://travis-ci.org/Hoverbear/termformat.svg?branch=master)](https://travis-ci.org/Hoverbear/termformat)
[![Crates.io](https://img.shields.io/crates/v/termformat.svg)](https://crates.io/crates/termformat)

```
$ termformat --help
termformat 0.0.1
Andrew Hobden <andrew@hoverbear.org>
A simple command line terminal text formatter. Reads from parameter or stdin.

USAGE:
    termformat [FLAGS] [OPTIONS] [content]

FLAGS:
    -b, --bold         Make text bold.
    -h, --help         Prints help information
    -i, --italics      Make text italics.
    -u, --underline    Make text underline.
    -V, --version      Prints version information

OPTIONS:
        --bg <background>    Background color. [values: black, blue, bright_blue,
                             bright_black, bright_cyan, bright_green,
                             bright_magenta, bright_red, bright_white,
                             bright_yellow, cyan, green, magenta,
                             red, white, yellow]
        --fg <foreground>    Foreground color. [values: black, blue, bright_blue,
                             bright_black, bright_cyan, bright_green,
                             bright_magenta, bright_red, bright_white,
                             bright_yellow, cyan, green, magenta,
                             red, white, yellow]

ARGS:
    <content>    The content to format.
```

## Install

```
$ cargo install termformat
```
