# RESTORE — external-dep（workspace 依赖缺口）

> restorer-external-dep，2026-10-17。按 `planning/2026-10-17-unimplemented-ledger.md` §3
> external-dep（5 条）补齐 workspace 依赖。身份以原 B 锁
> `provenance/original-Cargo.lock`（与 `reverse-engineering/tiny-glade/skeleton/B/Cargo.lock`
> diff 完全一致）为准；不用最新版覆盖。不手写任何依赖实现。

## 编译现状

- `cargo check -p toolbar`：**0 错误 / 0 警告**（本会话，联网在线补齐后，未被其他组在途错误阻断）。
- `cargo metadata`（全 workspace 解析）：**通过**，锁一致。
- 修改文件：`games_test/tiny-glade/Cargo.toml`（[workspace.dependencies]）、
  `games_test/tiny-glade/crates/systems/toolbar/Cargo.toml`（[dependencies]）、
  `games_test/tiny-glade/Cargo.lock`（仅新增条目 + toolbar/image 依赖面，无既有条目改版本）。

## 每条处置（已补入 / 版本 / 阻塞）

| 依赖 | B 锁版本 | 处置 |
|---|---|---|
| `taffy` | 0.3.13 | **已补入** workspace.dependencies `=0.3.13` + toolbar `taffy.workspace = true`；锁内 `taffy 0.3.13` checksum `82870da0…588c9` 与 B 一致（deps arrayvec/grid/num-traits 0.2.19/slotmap 同 B）。无阻塞。 |
| `tiny_http` | 0.12.0 | **已补入** workspace.dependencies `=0.12.0` + toolbar `tiny_http.workspace = true`；锁内 `tiny_http 0.12.0` checksum `389915df…64cdc82` 与 B 一致（deps ascii/chunked_transfer/httpdate/log 同 B）。无阻塞。 |
| `image` | 0.25.1 | **已补入** workspace.dependencies `=0.25.1`（`default-features=false`，features `["png","jpeg","tiff"]`，还原 B 解析面 {png, tiff, zune-core, zune-jpeg}）+ toolbar `image.workspace = true`；锁内 `image 0.25.1` checksum `fd54d660…bce3d11` 与 B 一致，deps 现含 tiff 后与 B 完全同面。无阻塞。 |
| `open` | 5.0.0 | **已补入** workspace.dependencies `=5.0.0` + toolbar `open.workspace = true`；锁内 `open 5.0.0` checksum `cfabf192…98cef8` 与 B 一致。无阻塞。 |
| `webbrowser` | 0.8.10 | **已补入** workspace.dependencies `=0.8.10` + toolbar `webbrowser.workspace = true`；锁内 `webbrowser 0.8.10` checksum `fd222aa3…a85b6` 与 B 一致。无阻塞。 |
| `hotwatch` | 0.4.6（git h3r2tic rev 7576ad4a…）| **已具备**（workspace.dependencies 既有 git 声明，B 锁同一 commit）；本次为 toolbar 补 `hotwatch.workspace = true` 引用。无阻塞。 |
| rapier 后端 | rapier3d 0.22.0（git PounceLight rev 7b5ece73…）| **已具备**（workspace.dependencies 既有 git 声明，锁内 0.22.0 与 B 一致）；`maintain_stone_column_colliders` 的后端属 country-core `native-physics` feature（`dep:rapier3d`）暴露面，为 **core-task**（core 提供 rapier 后端），非纯 workspace 依赖缺口。wall-constructor 依赖 country-core，无需自增 rapier。 |
| bytemuck | 1.16.0 | 提示项：**已入 workspace**（`=1.16.0`, derive），非缺口。锁内 1.16.0 与 B 一致。 |

## 硬标准说明

- 直接依赖身份（version/source/checksum）**全部与 B 锁一致**：taffy/tiny_http/image/open/webbrowser/hotwatch/rapier3d/bytemuck。
- 本次新增锁条目：taffy、tiny_http、grid 0.10.0、ascii 1.1.0、chunked_transfer 1.5.0、
  tiff 0.9.1、jpeg-decoder 0.3.2、weezl 0.1.12（后二者为 tiff 传递解析）。
- **登记传递解析漂移（非阻塞）**：`jpeg-decoder 0.3.2`（B 为 0.3.0）、`weezl 0.1.12`（B 为 0.1.8）——
  cargo 对 tiff 0.9.1 的 `jpeg-decoder "0.3"` / `weezl "0.1"` 取当前最新；与 workspace 既有 31 个
  传递版本漂移（如 gpu-allocator 0.27.0 vs B 0.26.0）同模式，属既有规范。若需字节级锁面一致，
  须手工改锁并防 `cargo update` 回弹，交由集成 owner 裁决。
- 锁改动仅新增（git diff 0 删除、+92 行）；未改既有条目的 version/source/checksum。

## 阻塞 / 代案

- **无阻塞**。联网 crates.io 在线可得（taffy 0.3.13 / tiny_http 0.12.0 / grid 0.10.0 /
  ascii 1.1.0 / chunked_transfer 1.5.0 / tiff 0.9.1 均已拉取入缓存）。
- 登记：rapier 后端交付权归 core-task（country-core native-physics）；外部依赖面已就绪。

## 总账更新

- `planning/2026-10-17-unimplemented-ledger.md` §3 六行全部处置：taffy/tiny_http/image/open/webbrowser
  **已补入**；hotwatch/rapier/bytemuck 已具备非缺口；rapier 行动作建议改为 core-task。
