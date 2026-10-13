# crates/rhapsody/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

crate根与leek入口包装已实现部分候选。

## 接线与验证

接线：not_established。

候选check/test/clippy；不能从根文件存在判断完整crate完成。

## 已恢复的函数和类型范围

- leek wait_submits_done和顶层接口

## 明确缺口

- 全部原root/reexports/features/原API闭合

下一闭包：全部原root/reexports/features/原API闭合

## 证据

- [reconstruction/rhapsody-rebuild/MODULE_STATUS.md](../../reconstruction/rhapsody-rebuild/MODULE_STATUS.md)
- [reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json](../../reconstruction/rhapsody-rebuild/evidence/missing-functions-completion-2026-10-13.json)
- [reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json](../../reconstruction/rhapsody-rebuild/evidence/functional-project-validation.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/rhapsody/src/leek.rs` | B | [reconstruction/rhapsody-rebuild/src/leek.rs](../../reconstruction/rhapsody-rebuild/src/leek.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/rhapsody/src/lib.rs` | B | [reconstruction/rhapsody-rebuild/src/lib.rs](../../reconstruction/rhapsody-rebuild/src/lib.rs)：pdb_structure_aligned_candidate_path; not_original_text | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/rhapsody-rebuild/src/leek.rs

[reconstruction/rhapsody-rebuild/src/leek.rs](../../reconstruction/rhapsody-rebuild/src/leek.rs)

SHA256：`9746cb7114bcc60e3ba0e4f0fdf5af6b965a52239663cf4784e721334c243c56`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 10 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 21 | 未由扫描证明；以证据限定范围为准 |
| `source` | 26 | 未由扫描证明；以证据限定范围为准 |
| `wait_submits_done` | 33 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/lib.rs

[reconstruction/rhapsody-rebuild/src/lib.rs](../../reconstruction/rhapsody-rebuild/src/lib.rs)

SHA256：`8b5c5477d805dc3b2a5872e8b5812ae5ab20423c722617e9e9b4e1e7b7200e1b`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
