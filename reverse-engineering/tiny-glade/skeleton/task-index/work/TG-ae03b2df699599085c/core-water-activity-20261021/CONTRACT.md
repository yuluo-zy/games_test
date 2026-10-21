# 水声活动格 producer → consumer 合同（待独立复核）

本轮范围为当前 Rust 工程的 B 原证增量恢复。B/C 文件源 checksum 相同只支持这份 B 算法复用候选，不升级 C ABI/runtime；原版发行 RNE 与 PDB 身份见 native-manifest.json。原指令没有执行，native-pack/report.json 是 LLVM 身份/字节静态证据。

## 已确认复用

- `update_water_audio_grid` 完整 PDB 范围 141baefe0+2477 字节已读：profiler/borrow panic 与 unwind 复用依赖；业务主段 141baf315..511。frame 自增后令 hash=11*(f+1)，64 位选择，抖动坐标由低位映射到奇数 [-7,7]，先乘 1/16 再乘13。native 两段乘法对这些整数输入均可精确表示，当前 *0.8125 等价。
- core 唯一 `WaterAudioGrid` 已有100格、每格位置+u64 bits+两 i16 sums、变位时 wrapping 加减，update 的 Local 仅 frame。原音频消费者按 popcount/64、平均抖动中心、3D 距离与42.25衰减计算音量。当前实现已具有这些算法，不能重建镜像 grid。
- 采样窗口 XMM3=130 与第五参数0x43020000=130；当前调用130×130正确，旧头注释260不成立。canonical WorldRaster current_buffer 读 composite/scratch selector，编辑/历史/clear 共用该实例。
- 原 consumer 1409ecea0 中 min(loop.len,grid.len)、start_if_not_playing、set_position、AudioInstance Active guard、set_volume、空位 stop 均在当前 source；`WaterAudioGrid.clear_activity` 保留位置/容量，uninit_clearing 与 clear_canvas 已调用。候选不改变音频状态机和卸载策略。

## 确认缺口与最窄修复

1. **真实 producer 无法运行**：水系统取 `Res<Raycasting>`，该类型私有独占了另一 RaycastWorld；当前全 crates 没有其资源创建或插入。生产 run_game 安装的是 `Res<RaycastWorld>`。修复 water 参数借用现有唯一 RaycastWorld，继续显式借用 TerrainHeightsData，不构造第二物理世界。
2. **有限遮挡线段丢失**：141baf498..4a5 传 max_toi=1.0、mask0x32a3、origin=sample、direction=eye−sample。1408cc344..380 将相同 max_toi 先传地形（mask&0x20）再传 Rapier，地形命中短路。当前 Raycasting 硬传 f32::MAX，会把相机之后物体也当遮挡。建议由现 RaycastWorld 增加 `intersects_any_w_terrain(terrain,origin,direction,max_toi,mask)`，先 terrain.intersect_ray(ray,max_toi)，否则复用已存在的 intersects_any_with_max_toi。只在获 core world 旧 owner 窄委派后出此候选。不要改既有不带 bound 的兼容 API。
3. **demo gate 身份待证（暂停修改）**：141baefe0 在所有业务前读 `_DEV_OVERRIDE_IS_DEMO_MODE`；当前 water_systems_enabled 永远true。14362c098 是指针槽，实际标志身份/与DemoModeEnabled的等价性尚需PDB数据符号、原writer交叉确认。独立review要求暂停此项，候选不引入DemoModeEnabled、不改门禁。draw_water 同样保留未知。
4. **宿主未注册音频 owner/consumer**：实际 audio_host Startup 没有 water_ambience_audio_init_system/water_audio_init_system，Update 没有二者 consumer，WaterPlugin 仅 Option<Res<WaterAudioGrid>> gate，故水格永远不存在。root 已接受 host 接线修复；子候选不改宿主。

## 候选源范围与依赖

- 已合法领取 TG-ae03 水格文件；先出该文件候选及对应现有 inline tests。
- collision TG-f707 owner building-audit-20261018，当前只读，待 root 窄委派；本合同不给自己写权。
- core ambience TG-5ac8 owner boot-audio-lifecycle-20261019，root 已允许沿旧 work_dir 生成窄候选，但目前未发现必须改业务，无需为提交量动它。
- 新 helper 需 native-physics，与实际 tiny-glade 已有 feature 一致。system-water Cargo 若单独构建也需 root 在既有 country-core workspace dependency 上启用 native-physics（一行）；由 root 唯一 Cargo owner 处理，不引入新依赖/版本/后端。
- 重用锁定 Bevy0.16/glam0.29.3/Rapier 与现 FloatGridSampler；不恢复依赖内部、不改变 sampler 全局策略。

## 验收（由 root 执行）

- 实际 Schedule/World 使用唯一 RaycastWorld 和 canonical WaterAudioGrid：全水 raster 的帧位/抖动 sums、重复64帧覆盖、清 raster 64帧排空、clear_activity 后继续更新均一致；不依赖 unit projection helper。
- 距离阈值及水阈值 ordered/NaN 剔除保持原序；demo gate 身份未证，本轮不改。
- RaycastWorld 原生物理测试：遮挡在 sample→eye 内/端点/eye 后、不同mask；terrain短路同样严格使用 max_toi=1，方向不归一化；root 现有 bounded RaycastWorld 测试复用并增加 terrain-aware层。
- root 实际 App 注册 startup owner 后首次 Update 无缺 Raycasting 资源错误，水格 producer/ambience消费/卸载/二次会话真实闭包；native-audio 实机 FMOD 验证另记，静态/Mock结果不能宣称真实音频通过。

目前未运行 Cargo/rustc/GPU/原指令 oracle；未修改任何生产源或共享 Cargo/lib/App。

## 现有可复用 bounded 接口（完整原文，world.rs:1574..1586）

```rust
/// B @0x1408cd5b0 -> 0x1408cdf80: preserve the caller's parametric ray bound.
/// The direction is passed unchanged; physical length is `|dir| * max_toi`.
#[cfg(feature = "native-physics")]
pub fn intersects_any_with_max_toi(
    &self,
    origin: glam::Vec3,
    dir: glam::Vec3,
    max_toi: f32,
    mask: u32,
) -> bool {
    Rapier::intersects_any(self, origin, dir, max_toi, mask)
}
```

`RaycastWorld` 自身为 public Resource；实际 World 共用它，现 native-physics API 只读借用。无特征的现兼容 `intersects_any` 返回 false 只是既有未实现壳，不能用该壳当新闭包验收。候选应由实际 feature 驱动，不添加默认成功 fallback。
