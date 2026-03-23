# trial-workspaces

这个目录用于存放每一次 Phase B 实验的独立工作区。

每个 trial 都应来自同一份 `candidate-template/`，避免前一次实验的代码或测试污染后一次实验。

建议命名方式：

- `trial-001`
- `trial-002`
- `demo-baseline`
- `demo-with-tests`

推荐通过上一级脚本创建：

```bash
./phase-b/new_trial.sh <trial-name>
```
