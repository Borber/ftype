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

## 速度测试

1.  文件一, `WIIUCH051.rar` 
```shell
PS D:\app\ftype> ls D:\down\Compressed\WIIUCH051.rar


    目录: D:\down\Compressed


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----         2022/7/12     17:17    15971118168 WIIUCH051.rar


PS D:\app\ftype> Measure-Command {.\ftype.exe fake txt D:\down\Compressed\WIIUCH051.rar }


Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 122
Ticks             : 1227853
TotalDays         : 1.42112615740741E-06
TotalHours        : 3.41070277777778E-05
TotalMinutes      : 0.00204642166666667
TotalSeconds      : 0.1227853
TotalMilliseconds : 122.7853



PS D:\app\ftype> Measure-Command {.\ftype.exe restore D:\down\Compressed\WIIUCH051.txt }


Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 32
Ticks             : 325362
TotalDays         : 3.76576388888889E-07
TotalHours        : 9.03783333333333E-06
TotalMinutes      : 0.00054227
TotalSeconds      : 0.0325362
TotalMilliseconds : 32.5362
```

2. 文件二   `1.7z `

```shell
PS D:\app\ftype> ls D:\down\Compressed\1.7z


    目录: D:\down\Compressed


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a----         2022/2/14     17:49           2414 1.7z


PS D:\app\ftype> Measure-Command {.\ftype.exe fake txt D:\down\Compressed\1.7z }


Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 102
Ticks             : 1028260
TotalDays         : 1.19011574074074E-06
TotalHours        : 2.85627777777778E-05
TotalMinutes      : 0.00171376666666667
TotalSeconds      : 0.102826
TotalMilliseconds : 102.826



PS D:\app\ftype> Measure-Command {.\ftype.exe restore D:\down\Compressed\1.txt }


Days              : 0
Hours             : 0
Minutes           : 0
Seconds           : 0
Milliseconds      : 31
Ticks             : 313821
TotalDays         : 3.6321875E-07
TotalHours        : 8.71725E-06
TotalMinutes      : 0.000523035
TotalSeconds      : 0.0313821
TotalMilliseconds : 31.3821


```

|    文件名     | 文件大小 | 伪装时间 | 还原时间 |
| :-----------: | :------: | :------: | :------: |
| WIIUCH051.rar |  14.8G   | 122毫秒  |  32毫秒  |
|     1.7z      |    3K    | 102毫秒  |  31毫秒  |



**伪装和还原与文件大小无关** 
