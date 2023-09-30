# AnkiMaker

---

![Status](https://img.shields.io/badge/status-active-success.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

[English](README.md)
[简体中文](README_CN.md)

---

## About

AnkiMaker is a tool that generates flashcards for [Anki](https://apps.ankiweb.net/).

As of now, AnkiMaker can only generate flashcards for Chinese classical poetry and can export them in .txt format that can be imported into Anki.

[An example file](https://github.com/lalala-233/AnkiCards/blob/main/New/将进酒.toml) (written in Chinese) that can be used by AnkiMaker.

## Prerequisites

Please make sure you are using the latest version of Anki to import the flashcards generated by AnkiMaker. Flashcards may not be imported correctly if you are using an older version (<=2.1.54).

## Usage

To use AnkiMaker, run the following command:

```shell
cargo r --bin ankimaker --release <FILE_PATH>...
```

For example, if you want to generate flashcards from file1.toml and file2.toml, run:

```shell
cargo r --bin ankimaker --release file1.toml file2.toml
```

You can also use the `default` argument to generate a `default.toml` file:

```shell
cargo r --bin ankimaker --release default
```

## Todo

- [] Implement a GUI

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request if you find a bug or have a feature request.

## Authors

- [@lalala-233](https://github.com/lalala-233) - Idea & Initial work

You can also check the [list of contributors](https://github.com/lalala-233/AnkiMaker/contributors) who participated in this project.
