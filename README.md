# FGEN

A file generator that reads a file with instructions and generates a new file based on those instructions.

## Usage

```bash
fgen <input_file> <output_file>
```

### Language

The FGEN language is a simple language that defines the file. Here is a simple example:

```
TEXT     1       Hello world
ESC      \n
RAND     20      00,FF
```

This will generate a file with the following content (in hexdump):

```
00000000  48 65 6c 6c 6f 20 77 6f  72 6c 64 0a 5e 33 85 13  |Hello world.^3..|
00000010  3f 64 90 85 44 fa 52 1b  4f d3 42 ce 3d 39 99 c9  |?d..D.R.O.B.=9..|
00000020
```

You can see it has the text "Hello world" a new line and then 20 random bytes.

### Instructions

- `TEXT` - Writes text to the file `TEXT <REPEAT> <TEXT>`
- `ESC` - Writes an escape character `ESC <CHAR>`
- `RAND` - Writes random bytes to the file `RAND <REPEAT> <RANGE>`
- `FILE` - Writes the content of a file to the file `FILE <REPEAT> <FILE>`
- `HEX` - Writes a hex string to the file `HEX <REPEAT> <HEX>`

### Comments

Comments are defined by `#` and are ignored by the parser.

## Licence

This project is licenced under the GNU General Public Licence v3.0. See the [LICENCE](LICENCE) file for more information.
