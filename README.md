# TapDict

这是一个用 tauri 实现的桌面字典，使用 ocr 识别屏幕上的单词，可以鼠标选词。

[English](./english.md)

## 用法

鼠标悬停在单词上，按快捷键 Ctrl+Shift+c，就会弹出解释。
也可以打开窗口手动输入单词查询。

## 实现

通过 ocr 识别屏幕上的单词，查询一个内置的字典。
ocr 用的是 tesseract，内置字典数据来自: https://github.com/skywind3000/ECDICT

## 构建

参考[tauri](https://tauri.app/)官网安装依赖。

然后安装项目依赖，主要就是 tesseract 可以参考[tesseract-sys](https://crates.io/crates/tesseract-sys/)库的说明。

### windows

windows 上如果想要静态库的话 vcpkg 应该是要加上`-static`

```
vcpkg install tesseract:x64-windows-static
```

windows 上还依赖了两个系统库，我加在了 `build.rs` 里。

### linux

linux 上的依赖文档里说了 ubuntu 和 fedora，arch linux 里就用`pacman -S tesseract`就行。

### mac

`brew install tesseract`
mac 上运行的时候可能会缺以来`libarchive`，还没找到怎么把这个库打包到安装包里，但可以用 brew 安装。
