# R1：`grading_lut` 的 `.cube` 加载（渲染推进第 1 项）

日期：2026-10-14。对应 `RENDER_GAP_ANALYSIS.md` §3.1 / §7 第 2 步。标注：O 实测 / I 推断 / U 未测量。

## 做了什么
- 新增 `resources/grading_lut_cube.rs`（纯逻辑，不触设备）：`parse_rgb`（0x140b01cc0）、`parse_cube`（0x140b01050 的解析主体）。
- `resources/grading_lut.rs`：`GradingLut` 由 `Gap` 占位改为真实字段 `{name: String, size: u32, texture: Texture}`（O 偏移见文档）；
  `GradingLut::load_cube<D: TextureDevice>(device, path, name)` 接上 `read_to_string → parse_cube → new_3d(..,0x7b).sampled() → create_texture("grading lut")`。
- `rhapsody_types.rs`：新增 `TextureDesc::new_3d`（O 0x140bd5310，与 `new_2d` 逐字节对照）。
- `lib.rs`：用 `#[path]` 直接挂载共享基础库 `utils/src/rgb9e5.rs`（`pack_rgb9e5`，机器码已核对，不复制）。
- 测试台 `l0-harness` 挂载以上文件：全量 **804 通过 / 0 失败**（原 785；新增 grading_lut_cube 14、grading_lut 4、共享 rgb9e5 1）。
- `grading_lut.rs` 里 `todo!/unimplemented!` 7 → 5：仍未做 `ColorGradingState::flattened`、`GradingLut::load_unwrapped`、
  `lut_name_to_asset_name`、`asset_name`、`GradingLuts::load`（后两者依赖未反编译的闭包 0x140ae5bb0）。

## 新证据（O，机器码核对）
1. **0x1400f9c30 是 `str::trim`**（PDB：`trim_matches<trim::closure_env>`），不是恒等函数。反编译器只保留返回的指针、丢了 `rdx`（长度）。
   汇编 140b01285/01289 把 `rax/rdx` 存为行指针/长度，`test rdx,rdx` 判空，后续所有长度比较用 trim 后长度。
   `LUT_3D_SIZE` 参数同样先 `trim`（140b014ab），所以标准写法 `LUT_3D_SIZE 33` 合法。**若按反编译文本直译会得到“所有标准 .cube 都报 InvalidDigit”的错误实现。**
2. **行数不足 `size³` 不报错**：`DIGEST` 记的“越界 panic 0x1428d9518”是写入前的冗余下标检查，不可达（已更正 DIGEST）。
3. 字符串（取自 EXE，DIGEST 里给的是 fat-pointer 描述符地址）：`Parsing DOMAIN_MIN`、`Parsing DOMAIN_MAX`、`Parsing LUT_3D_SIZE`、
   `DOMAIN_MIN must be 0 0 0`、`DOMAIN_MAX must be 1 1 1`、`1D LUTs are not supported`、
   `Too many entries in the LUT. Bailing before an overrun`、`Parsing line {n}: {line}`（片段 `"Parsing line "` + `": "`）、
   `Invalid RGB triplet`（0x142ae6df8，DIGEST 未列）、纹理名 `grading lut`。
4. `LUT_3D_SIZE` 缺省值 `0x1f`（`uStack_70 = 0x1f`）；`size³` 为回绕乘；`resize` 在 `size³ < len` 时会截断。
5. 纹理格式 `0x7b` = `VK_FORMAT_E5B9G9R9_UFLOAT_PACK32`；`0x1417c66b0` 的 PDB 名就是 `utils::rgb9e5::pack_rgb9e5`，共享 `utils` 已有对应实现。

## 未解决 / 偏离（U）
- `TextureDesc` 没有“维度码”字段：`new_2d` 写 `+0x1e=2`、`new_3d` 写 `+0x1e=4`（O），本结构体没建模（加字段会波及
  `pass_kit/test_env.rs:223` 等不带 `..Default` 的字面量）。调用方只能靠 `extent[2]` 区分。
- 分词器 0x140b00920（PDB 名 `str::iter::next`）取 `split_whitespace`（I）；未逐字节核对其空白表是否与 std 完全一致。
- `parse_rgb` 的 `ParseFloatError` 文案由 std 决定，测试里按当前 std 文案断言（`invalid float literal`）。
- 整个 crate 仍未整体编译（见 RENDER_GAP_ANALYSIS §0）；本次只在隔离测试台里编译了新增/改动的文件。
