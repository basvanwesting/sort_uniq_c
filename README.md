Efficient version of "sort | uniq -c" with some output options. \
Output order is word, count. Sorted by descending count

```
Usage: sort_uniq_c [OPTIONS] [FILE]

Arguments:
  [FILE]  The path to the file to read, use - to read from stdin (must not be a tty) [default: -]

Options:
  -d, --delimiter <DELIMITER>  Optional output delimiter, default to human readable aligned text output
  -h, --help                   Print help
```
