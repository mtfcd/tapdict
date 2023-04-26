# TapDict

A desktop dictionary app build with tauri. Read word from screen use OCR. Get word by mouse hovering.

## Usage

hovering mouse on a word, press Ctrl+Shift+c, a windows will popup and show the definition. You can input word by hand also.

## How it work

OCR functionality comes from [tesseract](https://github.com/antimatter15/tesseract-rs).
Definition comes from a embedded database. this database comes from: https://github.com/skywind3000/ECDICT.
If word can not be found in the database, it will lookup in [merria-webster's web api](https://www.dictionaryapi.com/).

## Building

Refer to [tauri](https://tauri.app/) install tauri.
Refer to [tesseract-sys](https://crates.io/crates/tesseract-sys/) install tesseract dependencies.
Apply a api key from [merria-webster's web api](https://www.dictionaryapi.com/) place it under src-tauri/src/utils/
Download stardict version of sqlite database file from https://github.com/skywind3000/ECDICT.

### windows

On windows for statically linked libraries add `-static` to vcpkg command.

```
vcpkg install tesseract:x64-windows-static
```

There are two system lib needed on Windows. I add it in `build.rs`.

### linux

Ubuntu and Fedora are covered in [tesseract-sys](https://crates.io/crates/tesseract-sys/), arch linux just run `pacman -S tesseract`.

### mac

`brew install tesseract`
But when you run the app, it might tell you `libarchive` not found, I haven't found a way to bundlethe library into app. But install it use brew should fix it. There maybe a convinient way in future version of tauri(https://github.com/tauri-apps/tauri/pull/5732)
