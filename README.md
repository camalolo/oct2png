# oct2png

Extracts base64-encoded binary data from piped input and writes it to a file.

## Usage

```
<command> | oct2png <output_path>
```

The tool reads stdin, finds the `base64,` marker, decodes everything after it, and writes the raw bytes to `<output_path>`.

## Examples

```sh
cat data.txt | oct2png image.png
curl -s http://example.com | oct2png output.png
echo "data:image/png;base64,iVBOR..." | oct2png out.png
```

## Build

```sh
cargo build --release
```

Binary output: `target/release/oct2png` (or `target/release/oct2png.exe` on Windows).

## Installation

```sh
cargo install --path .
```
