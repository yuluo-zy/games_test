# 合同、交付与审查门槛

按当前角色读模板即可，不需要通读全部模板。任务阶段记录在交付说明；既有 handoff schema 不变。

## 合同与证明义务

合同每项记：statement、version、classification（observed/inferred/unknown）、evidence 文件与精确 pointer、影响的 producer/consumer、gap。ABI/native 内存事实与语义 Rust 接口分开；未知字段不能填零。声明 dependency API/锁/fork/features、失败分支、float 顺序/NaN/溢出/RNG，以及适用的通知、dirty/tick、owner、回放、删除。

证明义务表每行含：obligation ID、原版行为与版本、evidence/pointer、Rust 位置、目标消费者、作者解释、review 结果、validation 范围或缺口。优化可抽象指令，但不能漏副作用。缺失事实阻止接受影响它的行为，不必阻止合同独立的另一任务。

交付应含候选 patch/base hashes、合同、义务表、版本适配、原证据引用、依赖快照、review、授权后才生成的 validation、handoff。freeze/check 仅保证清单列举文件在冻结后未改；`semantic_acceptance` 永远 false，不保证闭包完整。review/validation 自己引用 manifest SHA，修改后生成新清单与新范围结论。

## 独立审查与输出

先读冻结清单、原始事实和关键指令，再看作者解释；不继承作者完整推理。重点核对身份/版本/ABI、公开依赖边界、guards/字段/副作用、实际 producer→consumer→replay→removal，以及反例。作者 overlay 不作为唯一依据。必要脚本用 reviewer 的唯一 worker。

审查结论是 `needs_evidence / changes_required / passed_scoped`；无 findings 只表示本次范围内审查未发现问题，不等于原版通过。报告含 reviewer actor、target、manifest SHA、实际读取的范围/证据、findings 与 unresolved。finding 含 severity（blocker/major/minor）、函数/文件位置、违反义务、原证据、消费者影响和反例或最小补证。

blocker 包括错误目标/DB/oracle、关键 ABI 或输入猜造、公共库重写、漏影响结果的通知/清理/历史顺序、用旧依赖报告验收、桥接层注入期望业务结果。对证据冲突保留双方，不以 LLM 投票解决。

## 验证与整合

暂停验证时只能保留候选或 passed_scoped。未来运行测试时分别记录编译/API 检查、目标版限定原码差分、真实 World 链与冻结链验收；支持和不支持的结论写明。case 与桥接边界来自原版分支，审查者提供作者 fixture 之外的反例；不为低影响改动机械添加镜像测试。

报告绑定 source/contract/dependency/harness/bridge/case/tool/platform hashes；变化影响相应报告。容差不能看到失败后任意扩大，保留已有 ARM 1 ULP 未关闭问题。检查部署/整合权限、patch base 与现有锁，共享 Cargo/lib/注册/CLI 只由整合 owner 应用；整合后报告不能沿用未包含新源的旧 snapshot。

原生行为无法在当前平台或 ABI 下执行时明确该缺口，不用编译替代差分。验收不超出运行过的目标版本和消费者范围。

## 既有 ledger

依 `native-module-handoff` 使用 `target_version / module / versions / facts / links / dependencies / rust_files`。link 只有 unknown、evidence_ready、implemented_unverified、verified；verified 需目标版 runtime 与 verification 事实，无残留 gaps。审查通过不能升级。

将交接保存为项目内的工作 ledger，不改任务索引生成的原 handoff。使用已有审计器：

```sh
TG_PY=/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python
TG_AUDIT=/Users/yuluo/.codex/skills/native-module-handoff/scripts/audit_handoff.py
"$TG_PY" "$TG_AUDIT" WORK_DIR/handoff.json PROJECT_SKELETON_ROOT WORK_DIR/handoff.audit.json
```

索引 handoff 的引用以 skeleton 为 root；新 ledger 以实际证据路径基准选择 root（skeleton 或项目根），证据与 rust_files 都须相对此 root、不得越界。审计器只检查引用、格式、指纹与未支持 verified 标记，不能证明原事实是真或闭包完整。
