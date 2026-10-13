# shader 工具链与嵌入调试信息发现（B 版本）

日期：2026-10-13。数据源：B 发行 `compiled-shaders/` 404 个 `.bin` 的 SPIR-V 字符串。

## 编译命令（从 SPIR-V 内嵌字符串直接读出，Observation）

```text
dxc-cl-option: /tmp/.tmpqJ1ad3 -E cs_main -T cs_6_4 -spirv -WX -Ges -fspv-target-env=vulkan1.1 -HV 2021 -Zi -Fo /tmp/.tmpopgr37 -Qembed_debug
```

含义：**DirectX Shader Compiler (dxc)**，shader model `6_4`，输出 SPIR-V，`-WX`（警告即错误），`-Ges`（SPIR-V 速度优化），`-fspv-target-env=vulkan1.1`，HLSL 语言版本 `-HV 2021`，`-Zi`（生成调试信息），`-Qembed_debug`（把调试信息嵌入 SPIR-V）。

## 后果：SPIR-V 内嵌源码路径与名字（Observation）

- 404 个 `.bin` → **419 个唯一原始源码路径**（含 68 个 `.inc.hlsl` include），即整个 `shaders/` 源码树文件清单可无损提取（见 `embedded-shader-source-paths.txt`）。
- 目录结构直接可见：`shaders/dof/`、`shaders/gi/`、`shaders/grass/`、`shaders/water/`、`shaders/blit/`、`shaders/deferred/`、`shaders/inc/{common,samplers,frame_constants,uv,ray_cone,debug_utils,tracked_buffer,material_eval,...}.hlsl`、`shaders/inc/material/{solid,grass,plant,canopy}_material.inc.hlsl` 等。
- 存在 `../` 相对 include（如 `shaders/inc/material/eval/../canopy_material.inc.hlsl`）。
- 源码级变量/资源名保留：`coc_tex_dims`、`coc_tex`、`rt_bl_bvh`、`rt_triangles`、`instance_indirection_buffer`、`frame_constants`、`spdGlobalAtomic` 等。

## 对 HLSL 逆向的意义

- 这不是"无名字盲推"：dxc `-Zi -Qembed_debug` 意味着每个 shader 的绑定名、局部变量名、函数名、源码文件与行号信息可提取。
- 工具链已知且可复现：可用同一 `dxc -T cs_6_4 -spirv -HV 2021 -Zi -Qembed_debug` 重编译重建的 HLSL，用 SPIR-V 哈希/反射做差分校验。
- 共享 include（common/frame_constants/samplers/material_eval/…）内容需跨引用它的所有 shader 一致重建。

## 证据文件

- `embedded-shader-source-paths.txt`：419 个嵌入源码路径（含 68 include）。
- `catalog.json`：260 唯一 pass / 35 族 / 绑定细节。
- 复用 `parse_shader_catalog.py` 可扩展为"名字+路径+include 图"提取器。
