# T3 调查：符号名与泛型签名的来源与真伪

- 目标：`D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb`（274,710,528 B）与 PE `workspace/input/tiny-glade.exe`（SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`）。
- 方法：PDB 字节级搜索 + `llvm-pdbutil` 全库直方图 + 生成脚本数据流追踪。**只读**，未改动任何受限路径。
- 产物：`t3-names/{statistics.json, storage-form-evidence.txt, natvis-extract.txt, raw_symbols.txt, m476_symbols.txt}`。

## 0. 父级复核（逐字节验证）

| 断言 | 复核方式 | 结果 |
|---|---|---|
| 泛型名**逐字节**存在于 PDB | `grep -a -c "queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo" tiny_glade.pdb` | **2** ✓ |
| 同上（另一例） | `grep -a -c "get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources"` | **8** ✓ |
| 原始 legacy 符号并存 | `grep -a -c "_ZN8rhapsody3raw6submit55_"` | **1** ✓ |
| offset `31700481` 处原文 | 直接 seek 读取 | `GraphicsDevice::queue_submit<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global>,rhapsody::rg::executor::impl$7::submit_pending_cbs::closure` ✓ |
| `public-symbols.json[demangled]` 有损 | 统计含真泛型 `<>` 的条数 | **0 / 101,405**；含 `$` 转义的 **49,348** ✓ |
| workgraph 泛型名来自 PDB 而非 cxxfilt | 统计含 `<` 的 66 条中带 `$` 的条数 | **57**（PDB 存储风格）✓ |

## 1. 核心结论：**泛型名是原始 PDB 数据，可信**

### 证据链
1. **数据流**：`workgraph/functions.json` 的 `names` = `native-procedures.json[name]` ∪ `public-symbols.json[demangled]`。
   前者由 `scripts/render-skeleton/inventory.py` 解析 `llvm-pdbutil dump --symbols --modi=N` 输出（正则抓反引号内名字）。
   **`llvm-pdbutil` 不做去修饰** —— 它的 `--publics` 输出仍是原始 `_ZN`，所以 `--symbols` 打印的就是 **PDB 里存的字节**。
2. **铁证**：任务里那两个名字**逐字节**存在于 PDB 中（含结尾 `\x00`）。389 个已缓存 module dump 中 **0 个**含裸 `_ZN`，**352 个**含 `impl$/closure_env$`。
3. **关键反证**：`queue_submit` 的**原始 legacy 符号**是
   `_ZN8rhapsody3raw6submit55_$LT$impl$u20$rhapsody..raw..device..GraphicsDevice$GT$12queue_submit17h8a0ca96aebdf4411E`
   —— **路径里根本没有泛型**。而存储名却有 `<Vec<SubmitInfo,Global>,…closure_env$0>`。
   ⇒ 该名**不是由 `_ZN` 去修饰来的**，而是**编译器依带类型调试信息生成的"漂亮名"**。
   旁证：`llvm-cxxfilt` 对同一 `_ZN` 只得到 `rhapsody::raw::submit::_$LT$…$GT$::queue_submit::h8a0ca96aebdf4411`（无语意、留 `$`）。

⇒ **泛型名反映本二进制真实的单态化实例，属原始数据。**

### ⚠️ 语义边界（重要，勿越界使用）
它编码的是 **「路径 + 具体泛型实参 + `{impl#N}`/`{closure#N}` 判别号」**，
**不是函数签名** —— **参数类型与返回类型不编码**。`impl$7` / `closure_env$0` 是与源码绑定的**构建期编号**。

## 2. ⚠️ 需要降级的既有证据

**唯一被"重建"的是 `public-symbols.json[demangled]`** —— 它是脚本用 `llvm-cxxfilt` 对裸 `_ZN` 做的**有损**转换（保留 `$LT$/$u20$`、**丢泛型**）。

- 101,405 条中**含真泛型 `<…>` 的为 0**。
- ⇒ **凡引用该字段的类型结论都应降级。**

但 **workgraph 的 66 个含 `<` 的名字无一属 cxxfilt 风格**（51/66 是 PDB 存储风格）⇒ **workgraph 的泛型名全部来自原始 PDB，不受此影响。**

## 3. PDB 符号分类统计（`--symbols`/`--ids` 全库直方图）

| 记录 | 数量 |
|---|---:|
| `S_PUB32` | **101,405** |
| `S_GPROC32` | **62,480** |
| `S_LPROC32` | **22,832**（共 85,312 过程，与 `S_FRAMEPROC` 85,308 一致） |
| `S_INLINESITE` | **3,224,901** |
| `S_LOCAL` | **101,046** |
| `S_INLINEES` | 66,944 |
| `LF_FUNC_ID` | **83,429** |
| `LF_MFUNC_ID` | 35 |
| `LF_STRING_ID` | 43,625 |

**是否编码签名**：
- `_ZN`（`S_PUB32` 底层符号）编码**路径 + 泛型实参**，**不编码参数/返回签名**；
- `S_GPROC32`/`S_LPROC32` 的存储名 = 编译器漂亮名；
- `LF_FUNC_ID`/`LF_MFUNC_ID` 存去修饰名、**无签名**；
- `S_INLINESITE` 仅引用 `LF_FUNC_ID` 索引。

## 4. v0 修饰子集：**不存在**

全部 101,405 个 public 中 `_R…` 仅 **2 条**（`_RTC_Initialize` / `_RTC_Terminate`，**MSVC 运行期检查**，非 Rust v0）。
⇒ **全库无任何 v0 符号** —— "v0 编码完整签名"这条金矿**不存在**。

## 5. natvis：与游戏类型无关

3 个文件只描述 **windows-rs** 类型：`windows_core::array::Array<*>`、`RefCount`；`windows_strings::{HSTRING, PSTR/PCSTR, PWSTR/PCWSTR, RefCount}`；`windows_result::{Error, HRESULT, ErrorInfo}`。
含字段名/表达式（如 `HSTRING` 的 `header()->len/count/flags`），但**零** rhapsody / country-core 类型。

## 6. legacy `17h<hash>` 跨构建比对：**当前无法落地**

理论上：固定 rustc 版本下 hash 是路径 + 类型的确定函数，可跨构建比对。
**但** `skeleton/A|B|C` 仅有 `Cargo.toml`/`Cargo.lock`，**无 src、无编译符号、零个 `17h` 串**。要利用需**真正编译这三个骨架**。

## 7. `credits-crates.txt` vs 依赖

credits **563 条 ⊂ Cargo.lock 802 包**；**239 个包只出现在 lock 不在 credits**（bindgen/clang-sys/平台 shim 等构建期依赖）。
判定符号归属其实**不必靠 credits**：module 名（`rhapsody-*`/`country_core-*`）已直接给出原始 crate。

## 8. 仍不可得

- **完整的 Rust 参数/返回签名** —— 本构建**无 v0**，且 `_ZN` 路径不含签名；只能得**泛型实例**与路径。
- **依赖内部实现**（按项目规则无需逆向，直接复用）。
- **游戏自有类型的字段布局** —— natvis 不覆盖；须靠 **TPI 类型流**（T1 的任务）。
- PDB 类型流原始记录本题未展开。

## 9. 对"可实现性"的影响

| 问题 | 本题后的状态 |
|---|---|
| 泛型实例信息可信吗？ | **可信**（原始 PDB 字节） |
| 能从中得到函数签名吗？ | **不能** —— 只编码泛型实参与路径 |
| 有 v0 金矿吗？ | **没有** |
| 哪里需要降级？ | **`public-symbols.json[demangled]`**（有损，0 泛型） |
| 剩下哪条路？ | **TPI 类型流（T1）** 与 **调用点 ABI（T2）** |
