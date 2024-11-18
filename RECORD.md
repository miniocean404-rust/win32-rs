# 坑

## 中英文

windows api 中类似 `FindWindowA`, `FindWindowW` W 开头的可以获取中文和英文的窗口标题的句柄，但是 A 开头的只能获取英文的标题句柄
`s!` 宏用于 A(utf-8) `w!` 用于 W(utf-16) issues:https://github.com/microsoft/windows-rs/issues/2091