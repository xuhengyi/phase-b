# 基础 crate 测试矩阵

| crate | 命令 | 主要验证内容 |
| --- | --- | --- |
| `rcore-console` | `cargo test -p rcore-console` | 控制台输出、日志、全局初始化 |
| `linker` | `cargo test -p linker` | 链接脚本、镜像布局、应用枚举 |
| `signal-defs` | `cargo test -p signal-defs` | 信号常量、类型边界 |
| `kernel-context` | `cargo test -p kernel-context --features foreign` | 上下文访问器、foreign portal、host stub |
| `signal` | `cargo test -p signal` | trait 契约与结果语义 |
| `signal-impl` | `cargo test -p signal-impl` | handler 安装、默认动作、sigreturn |
| `rcore-task-manage` | `cargo test -p rcore-task-manage --features \"proc thread\"` | ID、关系管理、线程等待 |
| `sync` | `cargo test -p sync` | 互斥、条件变量、信号量 |
| `kernel-vm` | `cargo test -p kernel-vm` | 映射、克隆、共享映射 |
| `kernel-alloc` | `cargo test -p kernel-alloc` | 分配、回收、地址不重叠 |
| `easy-fs` | `cargo test -p easy-fs` | 文件系统创建、目录、读写 |
| `syscall` | `cargo test -p syscall --features user` | user API、常量、类型 |
| `syscall` | `cargo test -p syscall --features kernel` | kernel feature 下的编译/测试入口 |

基础 crate 全部通过后，再执行：

```bash
cargo pretest
```
