# color SSBO 与 lush 两项 CPU 失败：只读根因

只读定位 `/private/tmp/tg-business17-country_core-tests-20261021.log` 两项失败，不修改生产、候选、claim，不运行 Cargo。证据、当前源 hash、原 PE/PDB records、96 B 原指令切片及原资产内容见同目录 `INPUTS_AND_NATIVE_VALUES.json`。

## 1. lush：测试未读到真实资产，原定 read-error 分支正常运行

日志 line2880：`glade_defaults_and_saved_image_have_distinct_slots` 在原122行期待 `lush`，实际为 `default`。`load_session_grading` 的 I/O Err 分支明确产生 default，而 JSON 成功分支直接搬 `current_grading_lut`。这不是数值算法差异。

本机真实文件 `/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/assets/glade/summer/grading.json` 确实是 name=lush、strength=.4、ev_shift=.3、contrast=1.05、saturation=1.02；与测试完整期望一致。旧测试从编译期 manifest `games_test/tiny-glade/crates/country-core` 上溯3层，落到 `games_test/ljxsj_92385/Tiny Glade`，该路径不存在。当前 root 已在同文件测试加入 `TG_INSTALL_ROOT`，与 `/private/tmp/tg-cpu-test-install-root-20261021.json` after=5b86731d...一致，本 worker 不并写它。工作进程未设置该 env，不代表 root 后续执行命令的 env。

root 下一步只需以实际安装目录为 TG_INSTALL_ROOT 重跑该测试。生产已有 SessionMeta::assets_root→grading_json_path→read/deserialize→三槽默认→saved仅覆盖槽1/2的真实链；缺文件保持 default、坏JSON仍 fatal，不应为了测试改此业务，也不该把 lush 烘成默认。

精确文件任务 `TG-4fb3e73e6c3418824d`，live show **claim=NULL**；root 已唯一写六项测试路径支持，worker 未领取、未改写。已有原入口是1408bf9c0，旧完整原ASM `evidence/round12/color-startup/load_glade_grading-original.asm`。本次结论主要由真实读取路径及原资产直接闭合，未因旧章节标O便宣称当前原执行对照通过。

## 2. color SSBO：单个旧测试期望与已修正真实短名冲突

日志 line1341：`color_ssbo_short_names_hash_to_the_shader_binding_ssbo_ids` 第466行期待 `HalftimberColorIds`，实际 `HalftimberColorIdSsbo`。现生产 StorageElement::NAME 为后者；draw_callbacks_a / draw_callbacks_b 的半木标签同样使用后者和 ID `0x0cf3b7d1045fdc98`。CPU 颜色数组资源 `HalftimberColorIds` 与 GPU 元素 `HalftimberColorIdSsbo` 是不同类型，不能把 CPU 数组名推广成所有 GPU 槽名。

独立对 f507 原 RNE 确认：

- 原 0x142c815de 的**21 B literal** 是 `HalftimberColorIdSsbo`，字节 `48616c6674696d626572436f6c6f7249645373626f`。原 read_storage_buffer 实例为 **141630190/756 B，PDB1638:112176**，其 prefix 向哈希槽查询传此 pointer 和长度0x15。
- 原真正 `AssetSsboLibrary::get_rg_buffer<HalftimberColorIdSsbo>` 是 **1412ca520/810 B，PDB1579:118664**；原 movabs 的立即数字节位于 **1412ca53b**，值同 `0x0cf3b7d1045fdc98`。因此现 source 注释把141630190称get_rg_buffer是地址身份措辞不精确，但所用槽名和ID本身正确。
- 当前 `SsboId::of<T>()` 取 `NameHash::compute(T::NAME)`，draw tag 的库查询也取同槽；`NameHash` 复用既有 xxhash-rust，不需要改哈希算法。root 同次日志中独立 `halftimber_slot_hash_is_the_observed_exe_immediate` 已通过。这是 root 已执行结果，不是本 worker 新跑。

最窄修复方向：只把失败测试的半木短名期望改成 `HalftimberColorIdSsbo`，同时把该测试“槽名全是复数”的陈旧注释更正为半木例外；生产 const/Hash/注册/读槽以及断言严格度均保留。相邻类型头注释同样仍写旧复数，可以只做证据明确的注释纠错；不是必要算法改动。候选必须等中央协调 source owner，当前只读报告不产 foreign patch。

精确文件任务 `TG-2b6b4af01a4999b89e`，live owner=`main-chain-frame-20261018`，旧合法 work 见 live receipt。其它 draw/library 文件只作为消费者核对，不取得写权。

## 验收界线

这两项不能合并归因：lush 是实际安装根路径问题，color 是旧字符串期望问题。当前没有引入算法 fallback、改容差、跳测试或重建新注册库；根因文档不等于测试已恢复。由 root 完成单个测试窄修及真实 env 重跑后报告结果。
