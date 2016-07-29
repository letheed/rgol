# Conway's game of life


## Usage

Generate an empty map, edit it and play it like so:

``` sh
rgol genmap 40 40 > map
nano map
rgol play map
```

Inside the repository, you can use `cargo run --release --` in stead of `rgol`.


## Man

<pre>
SYNOPSIS
    rgol genmap N_ROW N_COL [-s]
    rgol play FILE [TICK_MS]

DESCRIPTION
    genmap Prints an empty map.

    play   Plays the game.
           CTRL-c exits cleanly and restores your terminal.

OPTIONS
    -s, --space
           Adds spaces to the map.
           Makes the map almost isotropic with monospaced fonts.

MAPS
    Maps must be rectangular.
    Whitespace is ignored. ‘·’ (U+00B7 MIDDLE DOT) is a dead cell. Anything else is a living cell.
</pre>
