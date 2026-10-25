# 像素验证图片（lavapipe，480x270，整帧测试读回）

来源：`cargo test -p tiny-glade --features native-audio --test original_gameplay_chain shipped_session`
（设 `TG_PIXEL_OUT=xxx.ppm` 可自行导出；画面经原版截图链路 `shared_texture_capture` 读回）。

- `world-frame-lavapipe-480x270.png` / `-960x540.png`：修复后的世界画面（秋季树林、满屏草地、一段弧形墙）。草地偏粉为未闭合问题，排查中。
- `02-world-black-before-fix.png`：修复前的世界画面，几乎全黑（根因：默认光栅状态为全 0，模板测试把所有片元挡掉）。
- `01-grading-reference-lut-strip.png`：GradingReference 读回，一条 16 格调色 LUT，用于确认调色/输出链路。

注意：lavapipe 的存储缓冲上限为 128 MiB，超出部分着色器读数不可信；像素结果仅作功能核验，不是最终画质依据。

## 更新（草地颜色修复后）
- `03-world-after-grass-color-fix-*.png`：补上 OnEnter(Game) 按会话主题加载 colors.json 后的画面，草地恢复秋季暖黄褐。
- 仍有两处小块花屏（左侧约 (2..35, 90..100)、右中约 (355..381, 108..126)），已二分到 `InstancedGlDrawFlag` 类 draw，根因未闭合。
