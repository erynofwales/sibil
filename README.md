Sibil
=====

A Scheme interpreter.

## TODO

A whole ton of stuff. Very much a work in progress. And this list is by no means
exhaustive, though I will continue to add to it as I think of things...

### `sibil` — The actual binary

- [ ] Environments
- [ ] REPL
- [ ] Standard library stuff

### `sibillexer` — The lexer

- [x] Make the Lexer Peekable
- [x] Use character iterators (`str.chars()`) instead of my own code to iterate
  through characters in the input
- [ ] Rational numbers
- [ ] Complex numbers
- [ ] Quasiquotes (this is not context free apparently, so I expect Badness)
- [ ] Proper error handling

### `sibilparser` — The parser

- [ ] Proper error handling

### `sibiltypes` — The type library

- [ ] Bools
- [ ] Chars
- [ ] Complex numbers
- [ ] Addition of disparate types of numbers
- [ ] Subtraction of numbers
- [ ] Multiplication of numbers
- [ ] Division of numbers

## Authors

- Eryn Wells <eryn@erynwells.me>
