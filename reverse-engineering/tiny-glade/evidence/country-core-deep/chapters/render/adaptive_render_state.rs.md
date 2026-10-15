# render/adaptive_render_state.rs

> 文件作用：**自适应渲染状态**——派生 Clone 的标记/配置类型（`AdaptiveRenderState`）。本章节内容极少（仅 1 个派生胶水），无独立业务过程。

## 派生胶水（1 项：clone 等）

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- 该模块只有 derive(Clone) 胶水（SOURCE_MAP 记录：0 业务 / 1 派生），无机器码独立过程。
- 关联语义（I，render-gap/render-passes 未单独建档）：自适应分辨率状态在 `render`（0x1404e8010）的 OOM 降分辨率路径被写（`scale += -0.1f32`、降级状态资源首 u64:=2，见 render_frame 章 / B_s_h H 层）；`reduced_scale`/`out_of_video_memory` 是 render_frame.rs 的辅助，非本文件。
- 结论：本文件不承载渲染算法；真实自适应逻辑在 `systems/render_loop/render_frame.rs`（`recompile_at_reduced_scale`）。

