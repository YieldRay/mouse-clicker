# Mouse Clicker

跨平台鼠标连点器

## 配置文件

应用程序会自动在以下位置保存配置文件：

- **Windows**: `%APPDATA%\mouse-clicker\settings.json`
- **macOS**: `~/Library/Application Support/mouse-clicker/settings.json`
- **Linux**: `~/.config/mouse-clicker/settings.json`

配置文件包含：

- 点击间隔设置
- 鼠标按键选择
- 热键配置
- 点击次数限制

## 故障排除

### macOS 权限问题

如果在 macOS 上点击计数器增加但实际没有点击效果：

1. 检查是否已授予辅助功能权限
2. 重启应用程序
3. 如果问题持续，请在系统设置中移除并重新添加权限

### 热键不响应

1. 确保选择的功能键没有被其他应用程序占用
2. 尝试选择不同的功能键
3. 重启应用程序
