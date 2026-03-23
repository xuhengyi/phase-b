## Context

`ch6` 是第一个把块设备、文件系统和进程系统调用整合进章节内核的版本，因此其复杂度来自子系统接线而非新的单一算法。

## Key Decisions

- `spec.md` 聚焦“文件系统接入、程序装载、文件 syscall”三类外部行为
- 具体的 MMIO 映射、块缓存细节和 FS 内部结构留给底层 crate 的 spec 与 design

## Constraints

- 运行时依赖 virtio block 设备
- 文件系统能力由 `easy-fs` 提供
- 进程与地址空间能力延续自 `ch5`

## Follow-up Split Notes

- chapter spec 不拆分
- 第二轮若引入 tests，可补强文件描述符初始化和错误路径场景
