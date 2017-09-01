# RGOL 1 "2016-12-30" "rgol 1.0.0" "User Commands"

## NAME
rgol - Conway's game of life in Rust for terminals

## SYNOPSIS
`rgol` `genmap` *NROW* *NCOL* [`-s`]  
`rgol` `play` *FILE* [*TICK_MS*]

## DESCRIPTION

`genmap`
  Print an empty map.

`play`
  Play the game.
  Ctrl + c exits cleanly

## OPTIONS

`-s`, `--space`
  Add spaces to the map.
  Makes the map somewhat isotropic with monospaced fonts.

## MAPS
Maps must be rectangular.

Whitespace is ignored. ‘·’ (U+00B7 MIDDLE DOT) is a dead cell. Anything else is a living cell.

## AUTHOR
[Letheed][email]

## SOURCE
[Repository][repo]  
[Bug tracker][issues]

## COPYRIGHT
Copyright © 2015 Letheed. License MIT.

[email]: mailto:letheed@outlook.com
[repo]: https://github.com/letheed/rgol
[issues]: https://github.com/letheed/rgol/issues
