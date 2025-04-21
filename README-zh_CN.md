# 休息提醒工具（目前仅限 Windows 用户）

这是一个基于 Rust 的小型 Windows 应用程序，用于监控 IntelliJ IDEA 或 RustRover 是否正在运行，并在连续使用 1 小时后提醒你休息。它还会记录你的专注工作时长到文件中。

## 功能特点

- 监控特定 IDE 进程（例如 `idea64.exe`、`rustrover64.exe`）
- 跟踪专注工作时长
- 在连续工作 1 小时后显示阻塞系统弹窗提醒休息
- 记录开始/结束时间和工作时长到文件
- 允许用户通过命令行选项配置日志文件保存位置

## 工作示例

> ![截图](Screenshot.png)

## 工作原理

1. 程序检查是否正在运行 IntelliJ IDEA 或 RustRover，当然，你也可以添加其他进程。
2. 如果检测到其中任何一个，它会开始计时。
3. 如果 1 小时（你可以手动设置时间，允许更短或更长的时间）过去，而 IDE 仍未关闭，程序将弹出一个系统提示框提醒你休息。
4. 工作会话记录到文件中以供存档。

## 安装

有两种方式安装此应用程序。

### 1. 从 Releases 下载
如果你不打算更改任何东西，建议直接从 Releases 页面下载。在下载后，运行以下命令：

```aiignore
rest-reminder.exe --log-to <file_location> --time <time>
```
如果你没有指定`focus_log.txt`的文件存放位置，程序会自动指定`D:\`为存放位置。如果时间参数未被指定，则程序会默认设置
为3600秒（1小时）。对于时间参数，你需要按照秒数指定，而不是分钟或小时数。

**注意：不要在文件地址后加上`focus_log.txt`的后缀！** 例如：
* "D:\\": 允许
* "D:\\focus_log.txt": **不被允许**
* "D:\\name": **不被允许**，因为地址最后没有加上 `\`


### 2. 从源代码中重新构建
如果你希望自己DIY，首先确保自己已经安装了Rust和Cargo。

```aiignore
git clone https://github.com/Emil-Stampfly-He/rest-reminder
```
被监听的进程相关代码块：
```Rust
pub fn run_rest_reminder(log_location: LogLocation) {
    let target_software = vec!["idea64.exe", "rustrover64.exe"];
    // Omitted
}
```
修改完毕后，确保程序能够被正常编译，然后运行：
```aiignore
cargo build --release
```
最终的.exe文件会出现在 `\target\release`目录当中。