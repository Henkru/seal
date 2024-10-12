# seal

A command-line tool to streamline encrypting secrets with [Age](https://age-encryption.org/)
and generate corresponding QR codes.

```bash
$ seal --help

Usage: seal [OPTIONS] <OUTPUT> [INPUT]

Arguments:
  <OUTPUT>  Write the result QR code to file at path OUTPUT. Pass - for stdout
  [INPUT]   Path to a file to read from. Defaults to stdin [default: -]

Options:
  -l, --label <LABEL>
          Optional footer label for the QR code.
  -f, --font-size <FONT_SIZE>
          Optional font size for the label. [default: 40]
  -a, --age-output <AGE_OUTPUT>
          Write the armored content to the file at path AGE_OUTPUT.
  -r, --recipient <RECIPIENT>
          Encrypt to the specified RECIPIENT. May be repeated.
  -R, --recipient-file <RECIPIENT_FILE>
          Encrypt to the recipients listed at RECIPIENT_FILE. May be repeated.
      --age-bin <AGE_BIN>
          Overwrite the age binary path. [default: rage]
  -h, --help
          Print help
  -V, --version
          Print version

# Generate QR code with label
$ seal -R ./recipients --label "hello secret" hello.png hello.txt

# Write the armored .age file as well
$ seal -R ./recipients -a hello.age hello.png hello.txt

# Read input from stdin
$ echo "Hello Age!" | seal -R ./recipients hello.png

# Pipe the output qr code (e.g. for printing)
$ seal -R ./recipients - hello.txt | lp
```
