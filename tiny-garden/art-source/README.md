# 美术源文件工作区

此目录保存可编辑制作资料，不作为安装包运行时资产目录。正式美术按 [制作计划](../docs/美术资产制作计划.md) 逐批交付。

当前包含 `tools/build-window-blockout.py`：在 Blender 后台生成第一扇基础窗的分块灰盒模板，含固定四角、可伸长直段、深色窗内和 `opening-bottom` 锚点。不是最终模型，未自动修改生产资产清单。

```powershell
blender --background --python art-source/tools/build-window-blockout.py
```

需要已有 Blender 可执行文件。当前机器没有在 PATH 找到 Blender，因此本次只检查脚本语法，**没有执行 Blender 导出**。脚本拒绝在交互会话运行，也拒绝覆盖已有 `.blend` 或 `.glb`，避免损坏美术修改。

生成后先手动审查，必要时优化 Bevel 造成的三角形，再将清单 `window-basic` 更新为 `blockout`、记录真实文件路径及作者；通过完整格式与引擎验收后才能标 `approved`。不能仅凭脚本运行成功直接批准。

底部枢轴位于外框最低点；洞口下沿锚点为导出后 `(0, 0.09, 0)`。外框范围约 1.08×1.38m，洞口 0.9×1.2m。运行时必须对齐洞口锚点，而不是直接把外框最低点放到洞口下沿。
