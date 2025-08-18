# Mouse Clicker

跨平台鼠标连点器

## 下载

从 [Releases](../../releases) 页面下载适合平台的版本：

- **Windows**: `mouse-clicker-windows-x64.exe` (64 位) 或 `mouse-clicker-windows-arm64.exe` (ARM64)
- **macOS**: `mouse-clicker-macos-x64` (Intel) 或 `mouse-clicker-macos-arm64` (Apple Silicon)
- **Linux**: `mouse-clicker-linux-x64` (64 位) 或 `mouse-clicker-linux-arm64` (ARM64)

## 配置文件

- **Windows**: `%APPDATA%\mouse-clicker\settings.json`
- **macOS**: `~/Library/Application Support/mouse-clicker/settings.json`
- **Linux**: `~/.config/mouse-clicker/settings.json`

## 系统要求

### Linux 依赖

在某些 Linux 发行版上，可能需要安装额外的依赖：

```bash
# Ubuntu/Debian
sudo apt-get install libxdo3 libxcb1 libxkbcommon0 libgtk-3-0

# CentOS/RHEL/Fedora
sudo yum install libxdo libxcb libxkbcommon gtk3
```

## 故障排除

### macOS 权限问题

如果在 macOS 上点击计数器增加但实际没有点击效果：

1. 检查是否已授予辅助功能权限
2. 重启应用程序

### Linux 权限问题

如果在 Linux 上遇到权限问题：

1. 确保安装了必要的系统库
2. 检查 X11 或 Wayland 显示服务器配置

## 其它

Inspired by <https://github.com/lalakii/MouseClickTool>
