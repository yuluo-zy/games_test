# systems/render_loop/render_world.rs

> 文件作用：帧渲染入口 `render_world`（H 层，0x1404ec110，54 个 pass 逐条调用序）。详见 `evidence/country-core-audit/B_s_h.md` H 层。

## 全局门控（render-gap §3.4，2026-10-14 补充）

| 全局 | 指向 | 语义（O 地址 / I 含义） | 用法（O） |
|---|---|---|---|
| `*0x14362c0d8`（qword，.data 初值） | `0x143634da0` | `freeze_occlusion_culling` 调试开关 | 0x1404ec5b1 `mov rax,[0x14362c0d8]; cmp byte ptr [rax],0; jne skip`——非 0 跳过 `generate_wall_draw_lists_pass2` + `gpu_generate_draw_lists` |
| `*0x14362c098`（qword，.data 初值） | `0x143634d90` | 进程级 demo 开关（同 `lut_gallery`/`gpu_lut`/`vermeer`） | 0x1404eca6c 同形读——非 0 跳过折射/水面 pass 链（demo 下语义 I） |

- 两者都是**指针全局**（`.data` 存 qword 指针 → 指向 `.data/.bss` 字节标志），与 `CENTERED_PATH`（`*0x14362c308`→`0x143638fac`）同形。
- reconstruction `render_world.rs:236/238` 的"语义 U"注释可更新为 `freeze_occlusion_culling` / `demo`。
