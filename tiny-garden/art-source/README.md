# 美术源文件工作区

此目录保存可编辑制作资料，不作为安装包运行时资产目录。正式美术按 [制作计划](../docs/美术资产制作计划.md) 逐批交付。

当前包含 `tools/build-window-blockout.py`：在 Blender 后台生成第一扇基础窗的分块灰盒模板，含固定四角、可伸长直段、深色窗内和 `opening-bottom` 锚点。不是最终模型，未自动修改生产资产清单。

```powershell
blender --background --python art-source/tools/build-window-blockout.py
```

此旧脚本此前因 Blender 不在 PATH 而未执行。2026-10-09 已找到 `C:/Program Files/Blender Foundation/Blender 5.2/blender.exe`，实际版本为 5.2.2 LTS；新管线与已执行结果如下。两个脚本均拒绝在交互会话中清空场景，且拒绝覆盖已有版本。

生成后先手动审查，必要时优化 Bevel 造成的三角形，再将清单 `window-basic` 更新为 `blockout`、记录真实文件路径及作者；通过完整格式与引擎验收后才能标 `approved`。不能仅凭脚本运行成功直接批准。

底部枢轴位于外框最低点；洞口下沿锚点为导出后 `(0, 0.09, 0)`。外框范围约 1.08×1.38m，洞口 0.9×1.2m。运行时必须对齐洞口锚点，而不是直接把外框最低点放到洞口下沿。

## 已执行的首批房屋资产

当前有效版本为 [house-kit-v4.blend](models/house-kit-v4.blend)，生产简报见[制作简报](房屋资产首批制作简报.md)。Blender 场景包括四种模块、每种的独立隐藏 LOD 集合、1.8m 尺度参考、预览副本、相机和灯光。LOD 是可编辑简化源，不代表游戏已启用切换。

- [生成脚本](tools/build-house-kit.py)：原创建模、米制重复 UV、四套纹理烘焙、预算/闭合组件检查、GLB 与 kit.json 同源导出。
- 输出：`assets/themes/warm-stone/house-kit-v4/`；4 GLB + 12 PNG + kit.json + Blender 验证数据 + Khronos 验证报告。
- 新版本窗枢轴直接位于洞口底部 `(0,0,0)`，不是旧 blockout 的外框最低点；门/烟囱也是底部中心。采用新独立合同，不能混用旧模板锚点。
- UV 使用有意重叠的 1m 重复纹理，不是唯一展开图集；导出 V 方向与 GLB 保持一致。
- `validation.json` 是 Blender 静态检查；`gltf-validation.json` 是 Khronos 验证与 GLB/kit 坐标、UV 一致性检查；视觉和性能限制见[验收报告](../docs/房屋资产首批验收.md)。

```powershell
cd D:\game\tiny-garden
# 不覆盖 v4，下一次写入新版本；无需 Blender 在 PATH。
& 'C:/Program Files/Blender Foundation/Blender 5.2/blender.exe' --background --factory-startup --python-exit-code 1 --python art-source/tools/build-house-kit.py -- --revision v5
```

`--python-exit-code 1` 很重要：Blender 默认可能在 Python 异常后仍返回 0。生成器不自动发布清单；美术人员修改 `.blend` 后仍需重新导出同源编译件并验收，不能只替换 GLB 而保留旧 kit.json。

手工修改源文件后的[统一重导出工具](tools/export-house-kit.py)：

```powershell
& 'C:/Program Files/Blender Foundation/Blender 5.2/blender.exe' --background --factory-startup --python-exit-code 1 --python art-source/tools/export-house-kit.py -- --source art-source/models/house-kit-v4.blend --revision v5
# 只核查源，不生成新文件：在上面命令末尾添加 --check-only。
```

重导出保留原源文件，应用导出副本的修饰器、输出同源 GLB / kit.json / 贴图并保存新版本源。需保留模块集合名、对象 `material_role`、原点与 UV 合同，不接受随意变换后不应用缩放的源。它不是任意 GLB 的通用转换器。

独立格式验证器使用 `gltf-validator@2.0.0-dev.3.10`（安装在本机临时工具目录，不加入游戏运行时依赖）：

```powershell
node art-source/tools/validate-house-kit.cjs 'C:/Users/liyu/AppData/Local/Temp/tinygarden-validator-20261009/node_modules/gltf-validator' v4
# 新版初次验证可加 --report 生成报告；已有报告不会覆盖。
```

制作迭代：v1 烘焙成功但静态检查中止；v2 初始模型；v3 调整木板和 UV；v4 导出切线后通过格式门。早期版本不是当前运行时资源，保留本地排查资料，不发布。
