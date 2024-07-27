# AnkiMaker

---

![Status](https://img.shields.io/badge/status-active-success.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

[English](README.md)
[简体中文](README_CN.md)

---

## 软件说明

AnkiMaker 是一个为 [Anki](https://apps.ankiweb.net/) 生成抽认卡的工具。

AnkiMaker 可以通过 `.toml` 文件生成 Anki 能导入的 `.txt` 卡片。

## 配置

### default mode

你可以在你的 `.toml` 文件中编辑你的卡片。一个 `.toml` 文件对应着一个牌组。

接下来是一个示例文件（使用 `ankimaker --default filename.toml` 生成）

```toml
[info]
deck = ""
mode = "default"
notetype = ""

[content]
paragraph = []
```

这些都是必不可少的字段。

你可以看看[一个可以被 AnkiMaker 使用的 `.toml` 文件](https://gitee.com/lalala-233/AnkiCards/blob/main/New/成语.toml)来了解大概的格式。你也可以用一些其他的 toml 写法，比如将 `"""` 换成 `"`。

default mode 几乎适用于所有的 Anki 卡组。

### poem mode

> 如果你不想用 Anki 背古诗，请跳过这段

如果你想用 Anki 背古诗，怎么办？

我的想法是，将每句诗作为一张卡片的核心要素，辅以注释、翻译、词义等内容。

但是除了古诗本身，其他的内容收集起来较为麻烦。所以如果只有古诗，怎么用 Anki 背呢？

```text
「众里寻他千百度」的下一句是什么？
```

还是给你题目和段落，让你背古诗内容？

我的做法如下。

```text
__________众里寻他千百度，__________
```

我会严格地让自己在心里默写，并判断正误，只要有错就选「再来」。

如果每一句都这么拆，那也太麻烦了。

所以，AnkiMaker 的 `poem mode` 就是用来干这个的。

一个 `poem mode` 的 `.toml` 示例文件如下：

```toml
[info]
notetype = ""
deck = ""
mode = "poem"
title = ""
author = ""
dynasty = ""

[content]
paragraph = []
```

其中，`dynasty`（朝代）和 `author`（作者）是可选的。

你可以看看[一个可以被 AnkiMaker 使用的 `.toml` 文件](https://gitee.com/lalala-233/AnkiCards/blob/main/New/古诗文/将进酒.toml)来了解大概的格式。

## 前提条件

首先，建议使用最新版本的 Anki 来导入 AnkiMaker 生成的卡片。

如果你使用的是一个过旧的版本（<=2.1.54），那么卡片可能无法正确导入。

其次，你必须在 Anki 中设定好相关的笔记模板，并创建相应的牌组（使用 `-o` 则可以不用创建牌组）。

## 使用方法

使用以下命令运行 AnkiMaker：

```shell
cargo r --release -- <PATH>...
```

例如，如果您想从 `file1.toml` 和 `file2.toml` 中生成卡片，可以运行：

```shell
cargo r --release -- file1.toml file2.toml
```

这将会生成在文件的目录下 `file1.txt` 和 `file2.txt`，你可以用 Anki 的导入功能来导入这些文件。

你也可以使用 `--default` 或 `--poem` 参数来生成一个 `default.toml` 文件：

```shell
cargo r --release -- --default default.toml
```

你还可以使用 `-o` 或者 `--output` 参数来指定输出文件的名字：

```shell
cargo r --release -- file1.toml file2.toml -o output.txt
```

## 贡献

欢迎贡献代码！如果您发现 bug 或有新功能请求，请随时打开 issue 或提交 pull 请求。

## 作者

- [@lalala-233](https://github.com/lalala-233) - 想法和初步工作

您也可以查看参与该项目的[贡献者列表](https://github.com/lalala-233/AnkiMaker/contributors)。
