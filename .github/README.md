
# sierpinski-triangle

A rust cli tool to generate a sierpinski triangle from the chaos game.

## Usage

```
Usage: sierpinski-triangle.exe [OPTIONS] --size <SIZE>

Options:
  -s, --size <SIZE>
  -q, --quality <QUALITY>                    [default: 4000000]
  -d, --output-directory <OUTPUT_DIRECTORY>  [default: ./]
  -h, --help                                 Print help
  -V, --version                              Print version
```


## Examples

This will generate a sierpinski triangle of 4000x4000 pixels and 800_000_000 points
```bash
./sierpinski-triangle.exe -s 4000 -q 8000000000 -d ./output
```
or
```bash
cargo run --release -- -s 4000 -q 8000000000 -d ./output
```


## Screenshots

![Render 4000x4000_800000000](https://github.com/formal-pancake/sierpinski-triangle/blob/main/.github/assets/render_4000x4000_800000000.png)

