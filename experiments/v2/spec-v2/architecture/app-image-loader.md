# 架构契约：用户程序镜像与装载

- Inputs: `inputs/user-tests-manifest.json`, `rustos/ch2/build.rs`, `rustos/ch7/build.rs`, `user/src/lib.rs`
- Prompt version: redo-step1-complete-os-architecture.v1
- Model/Codex version: Codex session with local `codex-cli 0.130.0`
- Date: 2026-05-09

## 目标

RustOS/COS 必须运行独立构建的用户程序。用户程序可以来自同一批测试源，也可以由生成实现自带兼容 ABI 的测试用户态工程构建；但内核不能把测试输出硬编码为 oracle，也不能把手写几段 U-mode 汇编当作完整用户程序体系。

## 用户程序 ABI

- 每个用户程序是 RISC-V 裸机用户态程序，入口符号等价于 `_start`，再调用测试 `main`。
- 用户程序通过 `ecall` 使用 `spec-v2/architecture/boot-trap-syscall.md` 中的 syscall ABI。
- 用户程序的标准输出走 `write(fd=1, buf, len)`；panic/错误输出可以走 `fd=2` 或标准输出，但必须由 syscall 进入内核。
- 用户堆通过 `brk/sbrk` 扩展；`ch4` 之后内核必须维护用户堆边界。

## ch2-ch4 app image

- `ch2/ch3` 可采用固定地址 app image：构建多个用户程序，按清单记录 base、step、数量、每个 app 的起止偏移。
- `ch4` 之后可继续使用 app image，但装载时必须建立独立地址空间，并按程序段权限或等价规则映射代码、数据、栈和 trap 上下文。
- app image 的用户程序必须由独立用户工程编译成 RISC-V ELF 或二进制，再被内核 loader 装载。
- 可接受的实现方式包括：
  - 构建脚本把用户程序 ELF/raw binary 打包进内核镜像，内核运行时按元数据装载。
  - 构建脚本生成 app table，内核运行时复制到用户地址空间。
- 不可接受的实现方式包括：
  - 内核源码中写死每个用户程序的输出。
  - 内核源码中写死一组仅用于触发 checker 的短汇编片段。
  - `test.sh` 在宿主机上直接运行用户程序或打印用户程序输出。

## ch5 进程装载

- 初始进程必须从独立用户程序镜像创建，通常为 `initproc` 或等价 init 程序。
- `exec(path)` 必须按路径查找一个已构建用户程序，并用该程序替换当前进程镜像。
- `fork` 必须创建可独立调度的子进程，父子进程看到不同返回值：父进程得到子 pid，子进程得到 `0`。
- 子进程退出后，父进程可以通过 `wait/waitpid` 取回 pid 和 exit code。

## ch6-ch8 文件系统镜像

- `ch6` 之后应提供块设备镜像或等价持久 app store，并由 QEMU 挂载给内核。
- 文件系统至少支持 base 测例需要的普通文件：
  - 通过路径打开用户程序文件并读取完整 ELF 用于 `exec`。
  - 通过路径打开普通数据文件，如 `filea`，供 `cat_filea` 和文件 I/O 测例读取。
  - 支持创建/截断/读写基础文件以通过 `filetest_simple`。
- 文件描述符表必须把 `0/1/2` 保留为标准输入、标准输出和调试/错误输出；普通文件和 pipe 从后续 fd 分配。
- `ch7/ch8` 的 pipe、signal、thread/sync 测例仍通过同一套用户程序镜像和 syscall ABI 执行。

## 构建与报告要求

每个 `generated/<impl>/<chapter>/report.md` 必须说明：

- 用户程序构建工具链和目标。
- app image 或 fs image 的格式。
- 内核如何定位一个用户程序。
- 内核如何创建用户栈、入口 PC、地址空间和初始寄存器。
- `exec` 如何查找和装载程序。
- 哪些用户程序来自共同测试清单，哪些是生成实现自带的兼容用户程序。

## Evidence

- `inputs/user-tests-manifest.json`
- `rustos/ch2/build.rs`
- `rustos/ch7/build.rs`
- `user/src/lib.rs`

## Uncertainty

本契约允许 RustOS/COS 采用不同 app image 或文件系统格式；一致性比较只看用户态可观察行为和 syscall/capability 语义。
