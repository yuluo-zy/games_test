# ICC 原 B 尾链恢复合同 V1

## 身份与边界

- 合法 claim：TG-10fd53e1e286bb43af / render-tail-icc-20261021。索引 claim 输出默认 target C，但本合同及候选仅恢复 B；不据 A/C 推断行为。
- B RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，匹配 PDB `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。
- `native-pack/report.json`：5 个有界请求，20 个符号化点，原字节逐指令核对，无目标执行。update 的 PDB 范围 3290 B 含主 unwind 区及多个 cold cleanup funclet；旧 round13 manifest 的 2184 B 仅主 unwind 区，不能当完整 update。
- 原模块路径 `country-core/src/systems/backend/icc.rs`。必要宿主增量由 root 授权，候选不写生产、不运行 Cargo/GPU/Git。

## 原地址、真实顺序、输入输出

| 原函数/区域 | 实际合同 | 当前消费者 |
| --- | --- | --- |
| run_event_loop 0x14026e4a0；dirty 初始化 0x14026e540 | 初始化 dirty=true；OS 事件批次后仅 dirty 且 monitor Some 查询；先清 dirty | Windows 主线程显示器 tracker；窗口 owner 安装调用点 |
| query call 0x14026f613 → get_monitor_icc_profile 0x1403ac400 | Win32 monitor device 名 → CreateIC/GetICMProfile → 路径；失败 warn 不发送变更，不清已有 profile | IccProfileChanged event 的真实 OS producer |
| send 0x14026f820..84f | 仅查询成功发送 tag0x11，经原 mpsc 到 App | 宿主直送当前 App 的 canonical event；App 切换后重新查询，不复用 texture id |
| IccProfilePlugin::build 0x140a58440 | add_systems；默认资源/事件在原 App 注册层创建 | 两个独立 App 均安装插件；Update 调度边界显式宿主适配 |
| update_window_icc_profile 0x14093d830，PDB3290 B | EventReader 只用本批最后一条；先 drop 旧 Some；再解析并生成 LUT；失败 warn，保持 None；GPU 创建失败 expect("ICC LUT texture") | ActiveIccProfile canonical Resource |
| generate_icc_lut 内联 0x14093d965..db43 | Profile::new_file → Profile::new_srgb → Transform::new(RGBA_16=0x4009a，双向格式相同，Perceptual=0，flags=0) → grid → 原地变换；自动 drop transform 与两个 Profile | 真实 CPU LUT 数据，非简化矩阵/默认纹理 |
| grid collect 0x1413036c0，1690 B | 32³ 个 [u16;4]；R 最快、G 次之、B 最慢；通道 floor(i*65535/31)；alpha 255，不能改65535 | 完整 262144 B texture initial data |
| texture 0x14093db63..dbe8 | extent[32,32,32]；format0x5b=R16G16B16A16_UNORM；new_3d + sampled；name "ICC"；一次真实 upload | 当前 TextureHost，通过已有 TextureDevice 边界 |
| update 0x14093dc26..dfb8 | name=path.file_name()，缺文件名使用原 path；to_string_lossy；Some(name,texture) | 可观察 active name；RAII 保实际 owner |
| final_post 0x14090d000 / closure0x1409ce780 | TypeId 5f06b298510fb768/054e14bacecb858d 的 optional texture +0x18；Some 绑定 B_LUT_CUSTOM，None 绑定已有默认 grading asset | world_postprocess 构造 FinalPostArgs.custom_lut，不能固定 None |

## API 与 owner 合同

1. 新 canonical 模块公开 `IccProfileChanged(PathBuf)`、`ActiveIccProfile`、`IccProfilePlugin`。默认真实 None；不伪造显示器/profile。
2. 原 GraphicsDevice 创建经现工程 `IccTextureDevice(Arc<dyn TextureDevice+Send+Sync>)` 适配到当前 App 的同一个 TextureHost，由 CoreRenderBridge::install_asset_libraries 一次注入。它不是第二设备/纹理池。两个 App、两套 namespace 保持独立。
3. Active 内部 OwnedIccLut 持 name、Texture 和同一个 strong device owner；替换或 App drop 仅 release 自己的 host slot。实际在途图、TextureView、descriptor lease 持 ManagedTexture Arc，CPU slot 释放不能提前销毁 GPU 对象。
4. 插件可以在尚无 GPU runtime 的 CPU App 构建。没有事件时不访问缺失设备；有真实事件时缺设备必须明确失败，不能吞事件并报告已安装。原有效 production App 总有真实设备。
5. FinalPost 读取 canonical Active；Some 时先校验 strong TextureDevice owner 正是 frame.hosts().textures，避免跨 App id 碰撞；再校验 host 中真实 texture 的 kind/extent/format/sample_count，构造现 BindTexture::Raw。raw 路径由 RasterPassHost 导入实际 Arc 并建立 sampled access，不能用 graph logical id 冒充物理句柄。None 保原 fallback。
6. 无每帧 LCMS/文件读取/纹理创建。只有最后一个真实变更事件触发完整算法。无 wait_idle、GC预算、渲染数量或 shader 算法改动。

## 依赖与平台

- 原 Cargo.lock：lcms2=5.6.0 SHA b9d50a4d00d909da0ec023ab34ea09bf3b3b79b5a36b3e0da3a305112edaf150；lcms2-sys=4.0.1 SHA 7abb8aac1a07fd8cdffaa25461a3873a7bf7164cd20b849d44a183a1348d682f。round13/icc 的两个完整 .crate 已重新计算并一致。
- lcms2-sys4.0.1 vendor header LCMS_VERSION=2150。本机 Homebrew little-cms2 2.19.1 已安装，但不能据此称原2.15数值完全一致。候选建议固定两个 Rust 包并启用 lcms2-sys static，复用原包完整2.15算法，避免实现自写颜色变换或浮动传递依赖。root 决定/执行 Cargo registry缓存安装、lock与链接验证；我们不运行构建。
- Win32 query 沿已有 tiny-glade/icc.rs 的真实函数。查询失败与 delivered bad path 行为不同，必须分别测试。
- 原B只有Windows query证据。本机Mac不能凭monitor名称或默认sRGB路径伪造ICC；需先明确当前 CAMetalLayer colorspace/ColorSync管理，防重复显示转换。核心显式事件→LCMS→GPU→finalpost可跨平台真实执行；OS自动来源的Mac方案独立标平台适配，尚未声称等同原Windows。
- 原Win32函数已有unsafe FFI；本次canonical核心/owner/consumer不新增unsafe。Windows DC异常路径历史泄漏不在此首包扩大。

## 错误/取消/替换矩阵与验收

- 无事件/无device：不创建、不清除 active。
- 同批 valid,bad：只处理bad；先释放已有active、warn、None；不能上传valid。
- 同批 bad,valid：只处理valid；恰好一次转换与真实创建。
- delivered路径读失败/ICC解析失败/Transform失败：旧active已释放，新None；不调用GPU创建。
- GPU创建错误：保持原fatal expect；activeNone，不假成功，不保旧profile。
- 替换/世界drop：各个host slot恰好释放一次；old graph仍持强native owner至真实完成/取消。
- 跨App相同数字texture id：owner校验拒绝；不允许读到另一App纹理。
- 图构建/compile/execute失败：现graph/callback/lease取消链保持，ICC active不因一次帧取消而释放；后续可再次绑定同真实纹理。
- CPU测试：完整grid顺序/位宽/alpha；真实LCMS sRGB identity和非恒等profile；最后事件/parse失败/创建失败/replace/drop及双App owner。
- GPU验收由root：真实显式profile事件→3D0x5b upload→actual finalpost shader sampled→默认/自定义像素差异；取消后再次绘制；异步旧纹理在途替换；原30/WSI现断言不变。CPU/mock测试不代替GPU。

## 当前明确未宣称完成

本合同是原证及必要工程适配设计；尚无候选编译/CPU测试/GPU执行结论。尾链其余TAA/TGSR/Vermeer/painterly/UI/text/截图有当前实现，逐分支审计继续，不能用旧missing清单或已通过world30/WSI13外推全分支完成。
