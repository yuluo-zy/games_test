# Blender 模型编辑工具链

安装完成，已验证实时连接、场景读取与 Python 执行。当前未转换游戏资源。

## 组件

- Blender 5.2.2 LTS，Apple Silicon，`/Applications/Blender.app`，通过 Homebrew 安装。
- MCP for Blender 2.1.9，通过 PyPI 安装在本目录的独立 Python 3.11 环境。
- Blender 插件位于 `~/Library/Application Support/Blender/5.2/scripts/addons/blender_mcp.py`，已启用。
- Codex MCP 名称 `blender`，配置位于 `~/.codex/config.toml`，修改前已保存带 blender 时间戳的备份。
- 仅本机连接 `127.0.0.1:9876`；桥接随 Blender GUI 打开启动，关闭 Blender 后停止。

## 按需使用

双击本目录 `open-blender.command`，或运行 `open -a /Applications/Blender.app`。

Skills 安装在 `~/.codex/skills`，新一轮对话可发现：

| 技能 | 用途 |
|---|---|
| blender-modeler | 网格编辑、修改器、原点与场景整理 |
| vegetation-artist | 树干、枝杈、树冠与叶片组织 |
| materials | 顶点色与材质搭建 |
| uv-workflow | 保留和调整 UV |
| retopology | 为继续编辑整理拓扑 |
| geometry-nodes | 建立可编辑曲线、参数化与分布系统 |

仅加载当前任务需要的技能。源技能库共有参考文件已补齐。
如当前聊天未显示新 MCP 工具，重新打开聊天；仍未显示时重启 Codex。

Blender 内按 N，进入 MCP for Blender 面板，可查看、启动或停止桥接。

## 来源与验证

MCP：https://github.com/ahujasid/mcp-for-blender

建模技能：https://github.com/arjun988/blender-skills

几何节点技能：https://github.com/nodecue/blender-node-skills

以上是社区工具与本次任务筛选的技能，并非官方精选目录里的 Blender 插件。

`installation.json` 记录版本与技能来源提交；`connection-verification.json` 记录实时验证结果。

如果需要重新启用插件：

```sh
/Applications/Blender.app/Contents/MacOS/Blender --background --python /Users/yuluo/Desktop/game/tools/blender/enable-addon.py
```
