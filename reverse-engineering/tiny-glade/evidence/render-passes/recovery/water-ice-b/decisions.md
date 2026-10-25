# water-ice-b 判定记录 (2026-10-13)

## 裁决
- **变体约定**：raster 双变体（b903 全量 / f0ad 精简或空 PS 桩）无法由单源复现 —— 单源编译出的 PS 绑定集等于 b903。以 b903 为重建目标，f0ad PS 绑定差记残差（同 F-E/F-W 约定）。
- **$Globals 绑定**：所有主文件显式 `cbuffer $Globals : register(b13,space0)`，`current_view_index` 作 member 0；不 include inc/current_view.hlsl（其文件级全局会使 dxc 生成 b14 第二个 $Globals）。
- **capabilities**：deferred_rt / ice_refl_resolve / ice_refr_resolve 目标含 StorageImageWriteWithoutFormat，安装 dxc 对已知格式 r11g11b10f 存储图不再发该 cap（ice_refl_resolve 额外多发 ReadWithoutFormat）—— dxc 版本差异，记录不追。
- **冰/水 RT 射线条目**：ice_refl_march→ice_deferred_rt 射线 = 字0 像素 / 字1-3 低位=世界 dir 半浮 / 高位=反射 dir；water_plane PS→deferred_rt 射线 = 像素 + octahedral tangent + 长度 + depth|flags。
- **O/I → U**：软件 BVH 的 morton 槽解码、water_surface/ice_plane PS 内层循环为紧凑等价实现（spirv-cross 已内联，逐条转录不现实）。

## 残差
- 直方图全部差异（dxc 版本 + inc 闭包漂移）；f0ad PS 桩绑定差；WriteWithoutFormat cap 1-off。
