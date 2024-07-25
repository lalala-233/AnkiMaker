# AnkiMaker

---

![Status](https://img.shields.io/badge/status-active-success.svg)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

[English](README.md)
[简体中文](README_CN.md)

---

## 软件说明

AnkiMaker 是一个为 [Anki](https://apps.ankiweb.net/) 生成抽认卡的工具。

但现在，AnkiMaker 只能为中文的古诗词生成卡片，并且只能生成可以被 Anki 导入的 .txt 格式的卡片。

[一个可以被 AnkiMaker 使用的示例文件](https://gitee.com/lalala-233/AnkiCards/blob/main/New/将进酒.toml)。

## 前提条件

建议使用最新版本的 Anki 来导入 AnkiMaker 生成的卡片。

如果你使用的是一个过旧的版本（<=2.1.54），那么卡片可能无法正确导入。

## 使用方法

使用以下命令运行 AnkiMaker：

```shell
cargo r --release -- <PATH>...
```

例如，如果您想从 `file1.toml` 和 `file2.toml` 中生成卡片，可以运行:

```shell
cargo r --release -- file1.toml file2.toml
```

你也可以使用 `--default` 参数来生成一个 `default.toml` 文件:

```shell
cargo r --release -- --default default.toml
```

## 贡献

欢迎贡献代码！如果您发现 bug 或有新功能请求，请随时打开 issue 或提交 pull 请求。

## 作者

- [@lalala-233](https://github.com/lalala-233) - 想法和初步工作

您也可以查看参与该项目的[贡献者列表](https://github.com/lalala-233/AnkiMaker/contributors)。
