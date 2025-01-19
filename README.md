# ROT13

[ROT13][1] is a simple letter substitution cipher that replaces a letter with
the 13th letter after it in the alphabet. ROT13 is a special case of the [Caesar
Cipher][2].

## Features

- Supports both uppercase and lowercase letters
- Preserves non-alphabetic characters
- Accepts input from both command line arguments and stdin
- Implements symmetric encryption (applying ROT13 twice returns the original
  text)

## Command Line Usage

```text
$ echo "Hello, World!" | rot13
Uryyb, Jbeyq!
```

Or directly as an argument:

```text
$ rot13 "Hello, World!"
Uryyb, Jbeyq!
```

[1]: https://en.wikipedia.org/wiki/ROT13
[2]: https://en.wikipedia.org/wiki/Caesar_cipher
