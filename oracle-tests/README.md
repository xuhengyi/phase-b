# oracle-tests

这里保存的是可以安装到 Phase B 候选工作区中的基础 crate 单元测试。

当前设计采用“可安装 unit tests”而不是单独的外部 integration harness，原因有两点：

1. 大部分基础 crate 的现有测试已经以 unit test 形式存在，迁移成本更低；
2. `kernel-alloc` 这类需要白盒访问的 crate，天然更适合走 unit test + seam。

## 当前内容

- `unit/<crate>/src/tests.rs`
  - 可复制到候选工作区 `<crate>/src/tests.rs`
- `unit/kernel-context/src/foreign/tests.rs`
  - `kernel-context` 的 feature 子模块测试
- `install_oracle_tests.sh`
  - 将这些测试复制到指定 trial workspace
- `run_oracle_tests.sh`
  - 安装测试并按固定矩阵运行

## 为什么说这些测试是“泛化版本”

这些测试在迁移时做了两类收敛：

1. 尽量避免直接绑定私有字段、内部容器或当前实现的命名细节；
2. 对无法完全黑盒化的少数 crate，显式定义最小 seam，而不是默认候选实现必须照抄当前内部结构。

目前唯一显式要求 test seam 的基础 crate 是 `kernel-alloc`。

测试矩阵见 [TEST_MATRIX.md](/home/xu-hy22/rCore-Tutorial-in-single-workspace/phase-b/oracle-tests/TEST_MATRIX.md)。
