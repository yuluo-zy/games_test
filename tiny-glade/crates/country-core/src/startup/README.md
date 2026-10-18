# startup：游戏启动与会话初始化

本目录把配置、磁盘资产和会话请求转为 ECS 资源、资产句柄和加载任务。2026-10-18 已按工程正确性重新整理；代码注释以中文职责说明为主，原逆向地址和旧结论保留在历史证据中。本轮有意调整旧设计，不据此宣称原版等价。

原审查与修复状态见 [REVIEW.md](REVIEW.md)，变更与验证记录见 [修复报告](../../../../reports/startup-refactor-2026-10-18/README.md)。

## 启动阶段

原来的单一 Schedule 混合了初始化、请求处理和运行系统，现在拆为三个表：

| 阶段 | 注册入口 | 建议宿主位置 | 责任 |
|---|---|---|---|
| 资源装配 | `initialize_core_resources` | 构建 App 时 | 注册共享资源、事件和已有域存储，保留现有配置与会话请求 |
| 一次性启动 | `register_startup_systems<B>` | Startup | 首次扫描会话列表、初始化音乐；重复运行不会重建已有音乐状态 |
| 会话请求 | `register_session_systems` | PreUpdate | 先运行命令器并提交 Commands，再消费存在的 NextSession |
| 每帧更新 | `register_update_systems<B, S>` | Update | 事件驱动的会话刷新，以及历史、存档、输入、地形、墙、生态、音频等已有系统 |

`register_core_startup<B, S>(world, startup, session, update)` 是上述装配的便捷入口；`add_game_startup_systems` 使用相同的四参数形式，并把花园/ivy 网格初始化加入启动表。**这是对旧二参数 API 的明确调整**；同一表不再承担三个生命周期。

宿主必须按 startup 一次 → session 每帧 → update 每帧的顺序运行；图形设备、资产库、相机、天气、时钟等域资源和事件轮转仍由宿主提供。真实 [tiny-glade 宿主](../../../tiny-glade/src/run_game.rs) 采用自身的 App 调度装配，没有调用这个便捷入口；不要再额外叠加注册一遍。

## 会话请求与完成

`load_game_session` 只准备加载，不把 JSON 解析或路径选择当作场景已经就绪：

1. 没有 NextSession 时直接跳过；None 请求只移除一次。已有加载尚未完成时保留后续请求，不覆盖当前动作和信号。
2. 选择最近会话或指定 ID，检查可加载状态与历史文件。新建时从 demo/full 模板选择起始历史；第一次用 intro，以后排序并循环其他目录。没有候选目录返回错误。
3. 新存档的 meta 和 snapshots 在同分区临时目录中准备，全部成功后 rename 发布。复制失败自动清理，不再忽略错误、留下半成品会话。新会话的历史来源仍是选中模板文件。
4. 从主题资产载入 GladeSettings。demo 优先读取当前 World 的 GameConfig，并覆盖主题、颜色和落叶设置；进程级 setter 只保留为旧宿主兼容入口。
5. 准备成功后更新 ActiveSession、配置和 AppMode，移动 on_load 动作到 LoadGameManager，移除请求并发送会话选择事件。失败保留请求及其动作，不提前发成功通知。

`OnLoadEntry::Batch` 拥有完整命令，`OnLoadEntry::Fence` 拥有 Arc 信号；取消、复制和重试不再依赖把整数还原为 Arc。旧式裸地址 fence 被明确拒绝，其他尚未知含义的旧命令交给宿主处理，不能静默丢弃。

`make_post_load_commands` 保持动作顺序，忽略终止标记之后的内容，最终模式切换放在末尾。世界实际应用完成后，管理器的 `finish_after_world_applied` 按游标消费动作；失败重试不重复入队已完成命令，所有可能失败的动作成功后才切换模式和释放 fence。ECS 加载器的 Loaded 分支消费同一组拥有型动作，并通过 LoadingGameComplete 通知宿主处理模式；成功后清空活动请求，避免重复通知。未接入的域动作会在任何通知或信号释放前被拒绝。

下游存档模块在同期接入了无有效快照时的历史恢复分支。本轮保留这些修改，完整快照和全游戏仍未在本轮验收。**本模块的请求提交成功不代表完整场景加载已实现。**

## 预制件与热重载

`PrefabWatcher` 复用 hotwatch 的平台监听和 100 ms 防抖；回调仅合并通知，文件 I/O 全部在锁外完成。接受 Write、Create、Rename、Remove 和 Rescan，支持编辑器的原子保存。`try_new` 可处理平台监听初始化失败，`new` 是保留的快捷接口。

重载先取走本批通知并构建独立配置快照；失败时保留旧库，将预制件和图集通知合并回队列，不覆盖期间新到的事件。成功后一次替换；新增和删除的名字也会通知消费者。图集独立变化时直接返回图集通知，不再重读整棵 RON 目录。

`load_prefabs_impl` 现在把完整目录视为库快照，因此一个 PrefabLibrary 应对应一个扫描根目录；成功重扫会移除已删除配置。实例消费者收到删除通知后只清理旧内容，保留引用实体以支持资源重新出现。

配置解析、继承和校验复用 asset 库。**网格、AABB 和部分音频字段仍是未接入的恢复占位**，本轮未把它们包装成完整可绘制资产。文件依赖监听仍由宿主注册；只有配置扫描不会自动建立所有监听。

## 主题音频

startup 与音频 ECS 域复用同一个 `load_theme_audio`：

- 正常主题读取自己的 audio.json 曲目；只有显式使用 halloween 目录且配置文件缺失或曲目为空时，才使用 Halloween 常量曲库。
- 参数 GladeType 使用 `GladeType::audio_id()`；IsHalloween 为独立的 0/1 开关。冬季的 player 标量仍为 0.25，不再拿来充当主题编号。
- 配置读取/解析失败返回错误，保留原曲库与参数；无效主题编号在产生副作用前拒绝。
- 新曲库创建后再替换旧曲库，清空旧播放索引与播放列表；重复加载不会积累实例。字符串切片与 String 复用同一接口，常量不再先分配临时 Vec<String>。

后端参数失败会记录参数名和值；事件实例化/停止的错误策略仍由现有 MusicPlayer/后端接口管理。本轮没有真实 FMOD 设备验收。

## 其他启动资源

| 文件 | 责任 |
|---|---|
| init_color_library | 上传默认调色板并登记 colors.json 监听，返回 CPU 资源供宿主插入 |
| init_photomode_state | 初始化拍照状态；重新进入时保留相机模式、自由相机初始化状态和焦点标记开关；未知字段不编造语义 |
| load_shaders / startup_blit | 注册 UI、通用和交换链 shader；底层 Result 入口现在真正传播编译错误；现有简便包装保留 expect 策略 |
| load_textures | 仅提供路径分量白名单和 demo/full 背景选择，不解码或上传纹理 |
| startup_garden_brush / startup_ivy | 一次性网格注册；花园完整 mesh→shader→spawn 流程使用已有宿主接口 |
| load_glade_tod_and_weather | 用 BufReader 读取时间序列；全部解析成功后替换资源，读取失败回退默认文件，坏 JSON 返回错误并保留旧状态 |
| startup_terrain | 四列分块追加 CW/CCW 索引；消除递归和回绕运算，写入前检查区间、u32 索引与平台容量，只预留一次输出空间 |

地形接口不建立顶点或 GPU 资源；无效尺寸在改变输出前 panic，调用者仍负责顶点布局。性能调整是明确减少分配、重复扫描与重复初始化；没有做帧率或吞吐基准，不报告量化加速。

## 检查命令

在 games_test/tiny-glade 下运行：

```sh
CARGO_INCREMENTAL=0 cargo test -p country-core --lib --offline --locked
CARGO_INCREMENTAL=0 cargo check -p tiny-glade --all-targets --offline --locked
CARGO_INCREMENTAL=0 cargo clippy -p country-core --lib --offline --locked
```

具体成功范围、未解决的外部构建问题和源指纹以本轮报告为准。上一轮 22 项隔离探针是修复前缺口的历史复现，不能作为当前源码验收。
