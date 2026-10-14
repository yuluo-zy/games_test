# resources/logo_animation.rs

> 文件作用：动画 logo 资源与系统：5 阶段揭示状态机（progress[5]@+0..+0x14 + durations[5]@+0x14..+0x28），阶段 0 播 FMOD event:/UI/Logo/Scribble、阶段 2 播 event:/UI/Logo/Branches；wait/finish 握手（+0x0c 等待进度、+0x10 完成揭示）；startup_animated_logo 加载 tiny_glade_logo/logo_branches JSON 网格与 raster.hlsl 着色器。

函数总计 9：业务 7 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| ``country_core::resources::logo_animation::animate_logo`` | 0x140b1a6b0 | 0 | 系统 | O：Update 系统 (ResMut<LogoAnimation>, Res<Time>, Res<AudioCore>)：找首个进度<1 阶段 i，stage[i]=clamp(stage[i]+dt/duration[i],0,1)；阶段 0 起播 Scribble、阶段 2 起播 Branches（串 0x142ae945b/0x142ae9472） | session-save\decomp\decomp-0x140b1a6b0.txt  |
| ``country_core::resources::logo_animation::LogoAnimation::play`` | 0x140b19d00 | 38 | 音频 | O：play = 清零 +0x00..+0x14（5 阶段进度） | session-save\decomp\decomp-0x140b19d00.txt  |
| ``country_core::resources::logo_animation::LogoAnimation::is_waiting`` | 0x140b19d10 | 45 | 谓词/校验 | O：is_waiting = (+0x0c)>0.0（movss+ucomiss+seta） | session-save\decomp\decomp-0x140b19d10.txt  |
| ``country_core::resources::logo_animation::LogoAnimation::is_finished`` | 0x140b19d20 | 49 | 谓词/校验 | O：is_finished = (+0x10)>=1.0（常量@0x142ae8b10；实代码 0x140b19d22，v2 地址按对齐） | session-save\DIGEST.md  |
| ``country_core::resources::logo_animation::LogoAnimation::finish_wait`` | 0x140b19d30 | 53 | 生命周期控制 | O：finish_wait = +0x0c:=1.0f（实代码 0x140b19d33；v1 名/地址整体错位一位） | session-save\DIGEST.md  |
| ``country_core::resources::logo_animation::impl$2::build`` | 0x140b19d40 | 81 | 构造/注册/生成 | O：Plugin::build = insert_resource(LogoAnimation 默认：progress 全 0 + durations{const×4, 0.5f}) + Startup startup_animated_logo + Update animate_logo | session-save\DIGEST.md  |
| ``country_core::resources::logo_animation::startup_animated_logo`` | 0x140b19d90 | 93 | 生命周期控制 | O：Startup 系统：load JSON 网格 tiny_glade_logo/logo_branches + 着色器 logo_animation.raster.hlsl/logo_branches_animation.raster.hlsl + spawn (KeepOnLevelUnload, DrawableMeshBundle, DesignatedRenderPass, DontCastShadow) | session-save\decomp\decomp-0x140b19d90.txt  |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
