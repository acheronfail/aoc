# Advent of Code

A little CLI app that automatically fetches the Advent of Code puzzle for the given year and day, starts a watch loop
which records answers for part 1 and part 2 of each puzzle, and then optionally submits it once stopped.

## Usage

```
aoc
Callum Oz <acheronfail@gmail.com>

USAGE:
    aoc <year> <day>

ARGS:
    <year>
    <day>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

## TODO

* restructure as workspace crate, with each sub-crate being a year
* each year will be a lib, and have examples which use the lib, and solve each day