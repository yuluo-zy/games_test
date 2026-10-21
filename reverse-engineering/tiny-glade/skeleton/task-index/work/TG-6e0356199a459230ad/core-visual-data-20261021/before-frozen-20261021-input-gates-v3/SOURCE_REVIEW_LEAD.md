# 独立源码复核（core business lead）

结论：通过本包最窄入口/叶范围；只做源码审查与隔离 patch 精确重放，未执行 Cargo、运行游戏或 GPU。

原入口门与失败顺序已核：flag 的 ty3d 先于 cache/ID；arrow 的 ty3a→required ID→subtype23/rank1排除→required真实墙→required collision key→dst_as_wall；roof 完整 body 无额外 dst_tag 前门，先 required subtype 再21分支。没有把 puffin UTF8 字节当业务类型门。mixed actual World fixtures 的 Flag3d/Chimney3b/Door38/Arrow3a 与对应 extra tag 已逐项核，不再使用旧错误 Chimney3e 夹具；最终 flag 哈希9ddb替代304c。

四项新系统测试验证真实生产 generator 共存、真实 blueprint mesh/source、缺 ID/cache/wall 的原 required 顺序；测试仅已做源码/API复核，须 root 用锁定 Bevy0.16 编译实际运行。

arrow 原 bc703 的 GothicHatWallView/Params→bones→panel/collider 桥仍未恢复；本包保现缓存适配并公开该缺口，不声称完整 arrow visual 算法或可直接挂整宿主。其它 visual 原无 ty 门者未机械添加门。

源码：`tiny-glade/crates/systems/decorator/src/decorator_visual/chimney_roof.rs`。

原始 before：`1a65764e5e0b06125ef0572db37c9ac625b8ade0ce87943a4c83a19a8dd42908`；冻结 candidate：`37d16960704485a0e6e4bb081b5d25222464244f70c99c4ad5645eb0b4a7700b`。

从原始 before 使用 combined/original patch，`patch -p1 -F0 --batch` 在独立临时目录退出0且字节完全等于冻结候选。生产 source 哈希保持 before。完整复核记录见 `SOURCE_REPRODUCTION_LEAD.json`。
