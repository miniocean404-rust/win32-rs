# 内存扫描

## 坑

1. dll 注入 64 位则注入 64 位应用程序会生效，32 位同理

## 文章

1. 设计游戏外挂\_blibli: https://github.com/miniocean404-rust/ac-esp
1. 植物大战僵尸 c++ 修改 dll 注入：https://www.bilibili.com/opus/978917454469136406
1. https://www.cnblogs.com/Timesi/p/17328291.html
1. CE 指针偏移查找：https://www.cnblogs.com/LyShark/p/16490201.html
1. Rust 中的进程注入参考（是人家想别人帮助的文章）: https://users.rust-lang.org/t/process-injection-in-rust-help/115314

# 坑

## 中英文

windows api 中类似 `FindWindowA`, `FindWindowW` W 开头的可以获取中文和英文的窗口标题的句柄，但是 A 开头的只能获取英文的标题句柄
`s!` 宏用于 A(utf-8) `w!` 用于 W(utf-16) issues:https://github.com/microsoft/windows-rs/issues/2091
