# AnkiMaker

---

![Status](https://img.shields.io/badge/status-active-success.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

[English](README.md)
[简体中文](README_CN.md)

---

## About

AnkiMaker is a tool that generates flashcards for [Anki](https://apps.ankiweb.net/).

AnkiMaker can generate flashcards from `.toml` files to `.txt` files which can be imported into Anki.

## Configuration

### default mode

You can edit the flashcards by editing the `.toml` file. One file is one deck.

The following is an example of a `.toml` file generated by `ankimaker --default filename.toml`

```toml
[info]
deck = ""
mode = "default"
notetype = ""

[content]
paragraph = []
```

These fields are essential.

You can look [An `.toml` file](https://github.com/lalala-233/AnkiCards/blob/main/New/成语.toml) (written in Chinese) that can be used by AnkiMaker.

You can also use some other toml grammar, such as replacing `"""` with `"`.

default mode is suitable for almost all Anki decks.

### poem mode

If you don't want to recite ancient Chinese poems in Chinese for the exam which requires you to write them without any mistakes, you needn't to read this.

If you want, read the [简体中文](README_CN.md) please.

## Prerequisites

Please make sure you are using the latest version of Anki to import the flashcards generated by AnkiMaker. Flashcards may not be imported correctly if you are using an older version (<=2.1.54).

And you need to configure the notetype you want use in Anki.

What's more, you should create the deck you want to import the flashcards into (If you use `-o`, you don't need it).

## Usage

To use AnkiMaker, run the following command:

```shell
cargo r --release -- <PATH>...
```

For example, if you want to generate flashcards from file1.toml and file2.toml, run:

```shell
cargo r --release -- file1.toml file2.toml
```

It will generate `file1.txt` and `file2.txt` in the their current directory. You can then import this file into Anki.

You can also use the `--default` argument to generate a `default.toml` file:

```shell
cargo r --release -- --default default.toml
```

You can use `-o` or `--output` argument to specify the output file name:

```shell
cargo r --release -- file1.toml file2.toml -o output.txt
```

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request if you find a bug or have a feature request.

## Authors

- [@lalala-233](https://github.com/lalala-233) - Idea & Initial work

You can also check the [list of contributors](https://github.com/lalala-233/AnkiMaker/contributors) who participated in this project.
