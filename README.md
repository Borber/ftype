中文 | [English](README_EN.md)

<a href="https://www.jetbrains.com/?from=fake_type"><img src="./icon_CLion.svg" height="40px"/></a>
使用 **CLion** 制作。感谢 _JetBrains_ 对开源的支持！

# ftype
以极快的速度将文件伪装成另一种格式，并无损恢复它们。

执行时间与文件大小无关。

## 使用教程

```shell
ftype 0.1.1
伪装文件类型

USAGE:
    ftype.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    fake       伪装文件
    help       Print this message or the help of the given subcommand(s)
    restore    恢复文件
```

### 伪装文件

```shell
ftype.exe fake gz .\release.zip
```

文件将会被伪装为`gz`压缩文件

同理, 使用 mp4, txt 将会将文件伪装成对应类型额文件

### 恢复文件

```shell
ftype.exe restore  .\release.gz
```

文件将会被无损的恢复为原始文件

