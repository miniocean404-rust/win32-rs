# 内存扫描

## 文章

1. https://www.cnblogs.com/Timesi/p/17328291.html
2. 植物大战僵尸 c++ 修改 dll 注入：https://www.bilibili.com/opus/978917454469136406
3. CE 指针偏移查找：https://www.cnblogs.com/LyShark/p/16490201.html

# 坑

## 中英文

windows api 中类似 `FindWindowA`, `FindWindowW` W 开头的可以获取中文和英文的窗口标题的句柄，但是 A 开头的只能获取英文的标题句柄
`s!` 宏用于 A(utf-8) `w!` 用于 W(utf-16) issues:https://github.com/microsoft/windows-rs/issues/2091

# 植物大战僵尸

144a5e48 5560
02a5b4a8 768
006a9ec0
