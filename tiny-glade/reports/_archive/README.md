# 归档区

本目录保存已退出主索引、但保留取证价值的内容。归档只做移动，不删除；随时可按 `move-manifest-2026-10-24.tsv` 反向移回。

## 内容

| 路径 | 原位置 | 归档原因 |
|---|---|---|
| `2026-10-17-refactor-iterations/` | `reports/refactor/` | 2026-10-17 的 v3–v9 迭代记录（40 MB）。`crates/rhapsody/reports/refactor/` 保留了同一轮 3.8 GB 的完整树，此处只是过时子集，`crates/rhapsody/REFACTOR_PROGRESS.md` 引用的也是完整树。 |
| `raw-dumps/2026-10-18-obsolete-objects-cleanup/` | `reports/core-systems-resources-recovery-2026-10-18/obsolete-{r7,rust}-objects-cleanup.json` | 19 MB 的“已废弃对象”扫描转储，仓库内无任何引用。 |

## 未归档（有意保留在主索引）

- `runtime/2026-10-19-main-loop-render-recovery/latest-audit-*`：当前最新审计，被同目录 `CORE-CHAIN-CONTINUATION.md` 引用。
- `runtime/2026-10-19-main-loop-render-recovery/logs/`：被 `independent-validation/FINAL_PROVENANCE_AUDIT.json` 等按路径引用，移动会使取证路径失效。
- `foundation/2026-10-18-country-core-utils-review-fixes/clippy*.jsonl`：被同目录 `verification.json` 引用，体积仅 3 MB。

## 移回方法

`move-manifest-2026-10-24.tsv` 每行是 `旧路径<TAB>新路径`（相对 `tiny-glade/`）。反向恢复示例：

```bash
cd tiny-glade && awk -F'\t' '{print $2"\t"$1}' reports/_archive/move-manifest-2026-10-24.tsv \
  | while IFS=$'\t' read -r src dst; do mkdir -p "$(dirname "$dst")" && mv "$src" "$dst"; done
```
