# duolingo_checker

A program that checks that your Duolingo words are in Anki.

## Usage

Can be found by doing:
```bash
duolingo_checker --help
```

### Example

You have a file like:
```text
Glas\tNoun\t33 minutes ago\t
Mann\tNoun\t3 months ago\t
```
which can be created by selecting all the text on the
"Words" section of the Duolingo site, putting it into
a text file, and then removing everything so it is like
the file above.

Then the words that are not in the Anki DB can be found
by doing:
```bash
duolingo_checker <Anki_DB_path> < words.txt
```
