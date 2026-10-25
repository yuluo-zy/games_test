# B 阶段（A4 / N / D / C / S / H）并行工作简报

目标：把渲染模块里剩余的 `todo!`/占位，按**原 EXE 机器码**恢复成真实业务逻辑，并有可运行测试。

## 硬规则
1. 不重造轮子：std / bevy_ecs 0.16 / glam / hashbrown / parking_lot / serde / itertools / ash / slotmap / xxhash-rust 一律直接 `use` 真实依赖。
   原依赖里离线缓存缺失的（puffin、frozen、nohash-hasher、enum-iterator、dolly、hotwatch、shader-pipeline、vk_sync_fork、toolbar、rhapsody 本体）
   只写**最小 stub**（只复刻调用点用到的签名，不发明行为），集中放在已有 stub 模块（`lib.rs::rhapsody_contract`、`puffin_contract`、
   `shader_pipeline_contract`、`rhapsody_types.rs`、`frozen_map.rs`），已有的先复用，不要再造第二份。
2. 业务逻辑必须来自机器码：先查 PDB 过程表 `evidence/country-core-audit/data/procs.tsv`（列 addr size ? obj name），
   `grep 'country_core::<模块路径>' procs.tsv` 取你的文件对应的全部过程；泛型单态化实例先按 `norm()` 去重，只需恢复一份泛型逻辑。
   用 `python3 -I tools/country-core-audit/dis.py <0x地址>` 看反汇编（带 call 目标名、常量、字符串），`rdva.py` 读字节/fmt 片段。
   注意：PDB 里名字可能是 ICF 链接器别名（相同机器码被合并），不要直接信名字；以代码体为准。
3. 每个结论标 O（机器码观测）/ I（推断）/ U（未知）。U 不要编造，留 typed gap 并注明。
4. 功能要完整：该函数在机器码里做的所有分支、常量、断言文本、错误文本都要在 Rust 里体现；`todo!`/`unimplemented!` 不得留在你负责的文件里
   （确实依赖范围外模块的，用 stub + 注释指明依赖点）。
5. 不要 commit。回复/文档用中文。不要改你不拥有的文件；如需改共享文件（lib.rs、rhapsody_types.rs、rhapsody_contract），
   只做**追加**，并在报告里列出，由集成者合并。公共 API 签名若必须改，在报告里写清旧→新与原因。
6. 解析不可信二进制一律 `python3 -I`。

## 验证（隔离测试台，每个 agent 用自己的副本，避免互相覆盖）
```
cp -r tools/country-core-audit/l0-harness /tmp/h_<你的名字>
# 在 /tmp/h_<名字>/src/lib.rs（及 src/render/mod.rs）里用 #[path="@SRC@/..."] 挂载你负责的真实源文件，缺的外部依赖在 harness 里补最小替身
L0H_REAL=$PWD/reconstruction/country-core/src L0H_WORK=/tmp/w_<名字> CARGO_TARGET_DIR=/tmp/t_<名字> /tmp/h_<名字>/run.sh
```
（目标目录嫌占磁盘可共用 `/tmp/l0h-target`。需要 `export PATH=$HOME/.cargo/bin:$PATH`。）
未挂载的文件至少 `rustfmt --check --edition 2024 <file>` 通过。测试写在源文件的 `#[cfg(test)]` 里，
断言原机器码里的常量/文案/分支（不要只测 happy path）。

## 交付
- 在 `evidence/country-core-audit/B_<你的名字>.md` 写报告：恢复了哪些函数（地址→Rust 项）、O/I/U 清单、stub 清单、对公共 API 的改动、测试结果、遗留。
- 最终回复只给 10 行以内摘要。
