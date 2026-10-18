# B CustomCursor 真实 buffer→事件→OS 写面

本文件族 `TG-b3ebb2d11bdaa18cfe` 已依法领取，owner `frameinputs-20261018`；parent 审核合同后明确授权合入这两份 core 源码，已应用并保存快照。关联 canonical `resources/mouse_capture.rs` 的旧 claim `TG-6159182f7b5484b1b8` 仍为同一 agent 之前的 `oschain-20261018`，不抢其他 owner。`game_loop.rs` 当前归 boot owner，本文给出其后续精确接线要求。

## 原证与布局

原 B RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。`native-manifest.json` 有完整 byte constructor、get_rg、CreatedCmd get_last/drain 实例。复用完整 OS 证据：`TG-aff540cfd24b989ccb/mainchain-20261018/evidence/{event_loop,main_loop}.asm`；callback `TG-40ff43c62ef8791c53/oschain-20261018/os-callback.asm`。

直接读取安装目录两份原 `_custom_cursor.raster.hlsl_*.bin`，逐字节匹配 retained oracle；从原容器 length prefix 按完整 SPIR-V 边界解析，保留 module SHA、OpMemberName、OpMemberDecorate Offset、类型引用到 `original-cursor-shader-layout.json`。两个原 vertex shader 都是：

```text
CustomCursor { pos: float2 @0, enabled: uint @8, size_px: float @12 }
```

总长 16B、对齐 4。不是 ID/颜色/占位字节。`custom_cursor_pass` 的 graph key 保留原 0xd0f0827e7ff22799；canonical short NAME 从 constructor rdata142c7f920 精确读出 `CustomCursorSsbo`，16 字节。

`new_storage_buffer@1416146c0`：元素 16B、BUFFER_LEN=1、TypedStorageBuffer40B；desc size16、usage0x22、memory2、alignment4；None 不上传初始数据。Some 时按顺序断言 len%16==0、ptr%4==0、len<32（允许恰好16）。不能复用 generic uniform constructor（其 usage0x12/memory1）或 MappedStorageBuffer 的 None 清零策略。

## 原启动末端

`init_populate@14090a532` 注册该实际类型。`a5a9` 调真实 mapped_slice_mut；必须 nonnull、ptr%4==0、len%16==0、len==16。`a617` 写事件 payload8B 指针、`a61b` EventId8B，stride16。没有写这 16B buffer 的初始化值。

事件的 canonical 名为 `country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd`。`main_loop@1404daa37` 通过同名 `get_last_event@1402b4910` drain 两个 World 本地事件数组，最后指针非零时转发 OS GameEvent tag6。`run_event_loop@14026e6ac` 保存它，callback 的 captured pointer cell 随后指向它。

候选保留事件 8B typed NonNull 的实际语义，安全 API 不提供裸指针解引用。Send/Sync 只允许传地址；实际 OS 写者必须满足真实 owner 存活和 host/GPU 同步，不能凭该 Copy 事件认为它有资源所有权。

## 原 OS 写面与当前错误

当前 `OsCursorVisibilityCmd::Custom(u32)` 将 payload 描述为光标 ID，错误。原 `run_event_loop@14026f910/917/937` 保存 payload 并原位写入映射 +0xc；callback @140557ea8/eb4 用 `movss` 再写 +0xc。它是 **f32 size_px**。候选将 variant 改为 `Custom(f32)`，derive PartialEq，不再为包含浮点的 enum derive Eq。

原 CursorMoved uncaptured 分支 `140557e57..eb4`：

1. 原 capture state tag==2 时进入绝对光标分支；其他状态按原捕获分支处理，不能无条件写绝对位置。
2. 映射 pointer 存在才写。位置 f64×2 转 f32×2，写 +0；suppression bits==0 生成 enabled u32 写 +8；保存的 size_px f32 写 +12。
3. 接着才向游戏转发 absolute CursorMoved。

原 Suppress/Unsuppress 改 OR/AND-NOT 来源位；在 custom mode 下，来源位跨越零的边界时写 mapped.enabled。Standard/Custom 模式切换仍按完整原分支分别决定硬件 cursor 显隐。size 浮点保存值与模式是两个状态，原 Standard 分支没有把保存的 size 清零；不要在接线时自行发明替代状态更新。

## 当前架构接线与生命周期要求

不改 rhapsody。已有真实接口：`tiny_glade::render_host::BufferHost::managed_buffer(&core Buffer)->Option<ManagedBuffer>`，其返回对象强持实际 buffer 的 Arc，后者保有映射 MemoryBlock/Device；render owner 已确认。

GameLoop 消费 CreatedCmd 必须按**当前 World**中 canonical `AssetSsboLibrary::get_buffer::<CustomCursorSsbo>()` 拿 core Buffer，经当时所属 BufferHost 按代际/owner 校验并取得 ManagedBuffer，确认其 persistent_host_ptr 与事件指针一致，然后保留 lease，才能建立唯一 OS writer。不能用 raw pointer 重建 Arc，不能从临时 CPU Vec 造事件，不能通过另一 App 的同序号 buffer 认领映射。换 App 时先退旧 writer 再换新 lease。

双 App 还要求在实际 handoff 点重置全部 World 本地 EventCursor：两个 App 的 EventId 从各自的0起，沿用 loading App 的 last_count 会漏 gameplay App 的初始化 CreatedCmd/OS 请求。boot 已确认此薄宿主修复需求，当前未改冻结 `game_loop.rs`。

## 候选 API 与未合边界

- `candidate-custom_cursor_pass.rs`：真实16B Pod元素、Typed40专用 memory2 constructor，以及明确只恢复原第72项的 `create_custom_cursor_ssbo(&AssetSsboLibrary,&dyn GraphicsDevice)->CustomCursorSsboCreatedCmd`，真正创建、查库、映射、校验并返回真实指针。该 helper 不发送假成功，也不冒充前71项。
- `candidate-mouse_capture.rs`：原8B typed pointer event和 Custom(f32)，其余现逻辑不改。
- 两份 core 候选已由 parent 审核后授权合入生产，尚未在完整启动注册/发送事件，未改 GameLoop；其实际 managed lease/唯一 writer/硬件 cursor 原分支待 boot 按此合同连接。不能因此宣称整个 CustomCursor 或全72启动已经打通。

rustfmt 解析通过，无 Cargo/GPU。当前生产 hash/快照已写入 candidate-manifest.json 的 applied sources，证据在本 workdir，运行验证待 parent 统一执行。
