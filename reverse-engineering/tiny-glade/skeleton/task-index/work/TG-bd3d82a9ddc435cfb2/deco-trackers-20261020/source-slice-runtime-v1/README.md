# 限定同源编译方案（待07审边界、root独跑）

此 harness 用 `#[path]` 直接包含三份完整实际候选文件；不复制/改写其 impl，不链接旧 country-core rlib。`country_core` 命名空间由当前生产源按 byte span 原文提取的 WallId、DecoratorId、DecoratorStorageAddr 定义和 impl 提供。module scaffolding / 测试观察资源是 harness 自身，不假冒任何原几何/音频/SSBO host。

`SOURCE_SPANS.json` 冻结每个来源整文件 SHA、byte 范围、行范围和未改切片 SHA；原 serde import 也精确提取。`DEPENDENCY_INPUTS.json` 冻结候选 SHA、当前 workspace manifest/lock。实际依赖是现锁 Bevy ECS0.16.0/std 和 serde1.0.200/derive；该三候选已不使用 glam，不增加无用依赖。`WORKSPACE_LOCK_INPUT.snapshot` 是初始 workspace lock 原物；初始 harness Cargo.lock 为相同完整副本，root 可仅在该隔离目录 offline 解析裁剪。

以下命令由 root 在全队单 Cargo 约束下执行。先做锁准备和身份验证；任何新 registry/git name/version/source/checksum 即拒绝，不下载或新选版本。这里未执行 Cargo，也未把 rustfmt 结果当类型检查。

```sh
python3 verify_inputs.py
cargo metadata --offline --format-version 1 --manifest-path Cargo.toml > metadata.json
python3 verify_inputs.py --prepared-lock
cargo test --offline --locked --manifest-path Cargo.toml --target-dir ../source-slice-target-v1
```

运行工作目录就是本 harness。root 执行后另冻实际裁剪 lock、完整诊断和退出码；如果 offline resolver 改选原 lock 以外身份，停在 lock 验证，不强造替身。若源在并发整合中变化，先审新来源与 span，再更新限定输入，不编辑测试依赖定义来过编译。

验证覆盖：真实候选 HRTB/token 生命周期的 Rust 类型检查；核心状态迁移与原 OrderedEq 测试；在真实 Bevy World/Schedule 中调用两 ResMut protocol adapters 并观察 current/Removed。最后一项只是协议 fixture，不是原 PublicWalls/geometry/audio 行为。

未覆盖：完整 country-core/system-decorator/App 构建、原七参/四参完整 assembly system、SSBO几何/音效、GPU、B原版运行差分、C、性能。source candidate仍为 implemented_unverified，直到 root 在此边界实际执行并由07复核结果。
