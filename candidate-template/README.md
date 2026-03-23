# candidate-template

这个目录是 Phase B 候选工作区模板，不是可直接运行的完整实现。

当前模板包含：

- 与上游一致的 workspace 结构；
- 各 crate 的 `Cargo.toml`、`README.md`、`build.rs` 等非实现文件；
- 章节 crate、基础 crate 的空源码树；
- 直接迁移的 `user` 和 `xtask`；
- host 测试所需的最小 seam 说明。

当前模板刻意不包含：

- 基础 crate 的现有实现；
- 章节内核的现有实现；
- `spec/` 规格输入；
- 已安装的 oracle tests。

如果你要开始一次新实验，优先从上一级运行：

```bash
./phase-b/new_trial.sh <trial-name>
```

如果你只想拿到一个完全裸的模板副本，可以使用：

```bash
./phase-b/new_trial.sh <trial-name> --bare
```

seam 约束见 [SEAMS.md](/home/xu-hy22/Graduation/phase-b/candidate-template/SEAMS.md)。
