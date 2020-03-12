
# btwn

Command line app to filter lines from stdin based on a range expression like '1...3'.

I built this because it's easier to use than awkward `head` and `tail` combinations.

```bash
$ btwn --help
Usage: btwn <range>

Filters lines based on the given range expression

Options:
  --help            display usage information

Examples:
  '3' - line 3 only
  '2..6' - lines 2 to 6 exclusive
  '2...6' - lines 2 to 6 inclusive
  '3..' - lines 3 onwards
  '..4' - lines 1 to 4 exclusive
  ```

  ## Installation

  Clone the repo and run

  ```bash
  cargo install --path .
  ```

  Alternately grab a release from the github release page.


## Examples


```bash
$ cat test.txt
a
b
c
d
e
f
g
h
```

Lines 2 to 5 (exclusive)

```bash
$ cat test.txt | btwn 2..5
b
c
d
```

Lines 2 to 5 (inclusive)

```bash
$ cat test.txt | btwn 2...5
b
c
d
e
```

Line 2 onwards

```bash
$ cat test.txt | btwn 2..
b
c
d
e
f
g
h
```

Line 1 to line 4 (inclusive)

```bash
$ cat test.txt | btwn ...4
a
b
c
d
```