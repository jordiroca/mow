# mow

A lightweight Rust application to remove whitespace from stdin, depending on the executable's name (`mow`, `lmow`, `rmow`).

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Additional Information](#additional-information)
- [License](#license)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/) (1.66 or newer)

### Building from Source

Clone the repository and navigate into the project directory:

```bash
git clone https://github.com/jordiroca/mow.git
cd mow
```

Build the binaries:

```bash
rustc mow.rs
cp mow lmow
cp mow rmow

# Copy the programs under your PATH:

sudo mv mow lmow rmow /usr/local/bin/
```

## Usage

Use the respective binary to mow input from the command line.

Examples:

### mow

Remove leading and trailing whitespace:

```bash
echo -e "  hello \n world  " | mow
```

### lmow

Remove leading whitespace:

```bash
echo -e "  hello \n world  " | lmow
```

### rmow

Remove trailing whitespace:

```bash
echo -e "  hello \n world  " | rmow
```

Note: Ensure that the name of the executable binary (`mow`, `lmow`, `rmow`) matches the desired mowming operation.

## Additional Information

### Contributing

Feel free to submit PRs, issues, or any other feedback! You'll be welcomed and appreciated.

### Support

For issues using `mow`, please create a [GitHub issue](https://github.com/jordiroca/mow/issues).

## License

See [LICENSE](LICENSE) for more information.
