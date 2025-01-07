# cnbc-parser

A scaper to parse "Stocks to watch on T+1" on CNBC.

Latest Version: `v0.1.0` - [Release Notes](https://github.com/Harshil-Jani/cnbc-parser/releases/tag/v0.1.0)

# Installation

For Linux and MacOS only. (For Windows, Do it yourself, I don't have plans to support Windows)

Run the following command in your terminal to install cnbc-parser

```
curl -sSL https://raw.githubusercontent.com/Harshil-Jani/cnbc-parser/main/install.sh | bash
```

For Development Purposes

```
git clone https://github.com/Harshil-Jani/cnbc-parser.git
cd cnbc-parser
cargo build --release
cargo install --path .
```

# Usage

```
cnbc-parser <URL> <optional : filename to save data>
```

# Contributing

Contributions are welcome. Please open an issue before making a PR. I am flexible with adding new features or being pointed out to bugs.

# License

This project is licensed under the **GNU GENERAL PUBLIC LICENSE**. See the LICENSE file for more details.
