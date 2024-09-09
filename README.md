### Terminal interface for rendering and simulating sorting algorithms
Utilises the [ratatui.rs](https://docs.rs/ratatui/latest/ratatui/) crate to render and sort bar charts  
Supports bogosort, bubble sort, insertion sort, merge sort and quick sort

```console
$ sorts_tui --help
Sorts TUI: terminal interface for rendering and simulating sorting algorithms

Usage: sorts_tui.exe [OPTIONS] <SORT_TYPE>

Arguments:
  <SORT_TYPE>  Sort algorithm to use [possible values: bogo, bubble, insertion, merge, quick]

Options:
  -n, --quantity <QUANTITY>    Number of items to sort (2 - 150) [default: 50]
  -t, --tick-rate <TICK_RATE>  How often interface reloads (in milliseconds) [default: 100]
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```
