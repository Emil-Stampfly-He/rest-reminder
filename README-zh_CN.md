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
rest-reminder.exe --log-to d
```
`--log-to d`表示日志文件会被存入`D:\`。当然，你也可以选择存入C盘:
```aiignore
rest-reminder.exe --log-to c
```
你可能会需要调整管理员权限。

### 2. 从源代码中重新构建
如果你希望自己DIY，首先确保自己已经安装了Rust和Cargo。

```aiignore
git clone https://github.com/Emil-Stampfly-He/rest-reminder
```
日志存放位置相关代码块：
```Rust
#[derive(Debug, Clone)]
pub enum LogLocation {
    C,
    D,
}

impl LogLocation {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "c" => LogLocation::C,
            _ => LogLocation::D,
        }
    }
}

fn get_log_path(location: LogLocation) -> PathBuf {
    match location {
        LogLocation::C => PathBuf::from("C:\\focus_log.txt"),
        LogLocation::D => PathBuf::from("D:\\focus_log.txt"),
    }
}
```
工作时长相关代码行：
```Rust
const WORKING_TIME: u64 = 10;
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