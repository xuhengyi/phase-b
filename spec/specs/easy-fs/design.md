## Context

`easy-fs` 的源码内部已经天然分成块设备、块缓存、磁盘布局、位图分配、文件系统管理器和 VFS/inode 层。第一轮为了完成全量基线，仍然先保留一个 crate-level spec。

## Key Decisions

- current truth 现在同时保留聚合层 `easy-fs` 和拆分后的 `easy-fs-storage`、`easy-fs-vfs`
- `easy-fs` 自身收缩为聚合 spec，不再重复子 capability 的细节 Requirement
- 把后续更细的拆分留在 storage/VFS 两层之下继续讨论

## Constraints

- 所有数据结构都建立在固定块大小与磁盘布局不变量之上
- 上层章节内核通过 `FSManager` 和文件对象接入它，而不是直接操作所有内部模块

## Follow-up Split Notes

- 已完成第一版 split：`easy-fs-storage` 与 `easy-fs-vfs`
- 后续若要继续细拆，可在 storage 层内再区分 block/cache/layout，在 VFS 层内再区分 inode 与文件句柄
