# tg-text：真实规则与本轮 CPU 闭合范围

本轮实现 FontTable + ShapeStream/RawShapeStream 可调用 CPU 链，使用原 serde 与 RON fork。完整 tg-text 未完成：原 lib.rs 的 TextRenderer、TextFragment、glyph cache、atlas 与 GPU upload 未实现，没有 renderer 成功 stub。

## Observed

| 规则 | 原依据 | 实现 | 比较与结果 |
|---|---|---|---|
| 字体用途枚举为 Label/MenuButton/TextEditor/Hand；PDB 类型名称为 FontFamily | 原 font_table.rs 命名实体、0x141e5d390 enum visitor、发行 assets/fonts/table.ron | font_table::FontFamily | 静态分支核对；发行配置以原 RON fork 成功读取 |
| 字体表按角色查找，缺失返回 None，不自行回退到另一用途 | 0x140b91fa0；HashMap 展开本身排除 | FontTable::get | 静态返回零/条目指针核对；候选缺失测试 |
| 默认 scale=1；scaled 保留 family 的借用，两个 f32 输入各乘 scale，不 clamp | 0x140b06780、0x140b92090；LLVM PDB module 625 | default_scale、FontTableEntry::scaled | RNE native default=1；21 组按 bits 对照零差异，含负scale/NaN/Inf/±0 |
| ShapeStream 新建为空 Vec；builder append 保留顺序；is_empty 看长度 | module 626/1186 原过程；0x140a82b20、0x140b920b0、0x140b92160 | new/fill_color/corner_radius/image/is_empty | 原 new、is_empty 执行；builder 静态核对，append 原执行 |
| 原形状 RON 有 FillColor(u32)、Image(source:string,scaling:enum)、CornerRadius(f32) | 0x140fad360、0x140fad7f0，发行 UI .ron | serde 派生 ShapeStreamItem + transparent Vec；ImageSource 复用 String::deserialize | 静态 shape visitor + 原发行样例读取；未执行原 serde ABI |
| ImageSource 的 RON路径调用 deserialize_str，再 From<String>，不是未标记 string/u32 二选一 | 0x140fad7f0→0x141e43360 原RON deserialize_str；0x140681f40 From<String> 同址别名 | ImageSource::Deserialize 只接收字符串；Rust From<u32> builder 保留 | 静态调用核对；RON numeric source 失败测试（候选） |
| scaling 值 Stretch/FitHeight/BlurExtend/Nine=0/1/2/3 | 0x140fad490 原字符串 visitor；append 的 bit packing | ImageScaling | 原 append 每种scale和7种index执行，零差异 |
| FillColor 原字值不改变，输出 [0,color] 两个 u32 | 0x140b92340 + RawShapeStream::encode 内联 | RawShapeStream::append FillColor分支 | 5 原色值案例零差异 |
| Image 输出 tag1、index低16位左移8、scaling左移24按位或，超16位index静默截断 | 同上；Image::with_scaling 内联 | append Image分支 | 28 indexed +4 named 原案例零差异 |
| CornerRadius 的输出 tag=2，clamp 0..255、截断整数、放bits8..15；NaN及负值编码0，+Inf编码255 | 原0x140b92340；常量0x14293ac70=255.0 | append CornerRadius分支复用 f32::clamp 与 Rust cast | 12原radius边界案例逐word零差异 |
| named image 按遇到顺序调用resolver；indexed image不调用resolver；append保留既有prefix | 原0x140b92340动态vtable调用与执行 | append闭包接口 | 51组整体word/call列表核对，其中有empty和混合prefix |
| encode_no_images仍能编码indexed image；遇named source走不可达panic | 0x140b92220调用append，vtable resolver指向0x140b92300，原panic文本 internal error: entered unreachable code | encode_no_images | 静态失败分支；候选数值图成功/名称图panic测试；原panic未执行 |
| RawShapeStream clear仅清长度，保留存储容量/地址；new为空 | 原0x140b92590、0x140b92320 | Vec::clear、Vec::default | 原clear/new执行；候选clear后复用测试 |

原执行报告：[native-oracle.json](native-oracle.json)。共51组append +21组scaled，另5次default/new/is_empty/clear调用；Rust的6项测试通过，不把其余候选schema测试算作原接口运行。

## Inferred / implementation choices

- scaled的两个输入以 font_size/line_height 命名，依据原cosmic-text消费者族；PDB未记录Rust真实字段类型，所以 ScaledFont是更清楚的恢复接口，不能当原ABI布局。
- builder使用显式Rust enum与Vec，表使用std HashMap；不复制原Rust niche布局或HashMap探测代码。原CPU协议的word与可见顺序保持一致。
- FontTable的family/scale字段、缺scale走1以及ShapeStream []表示由发行文件、default_scale和派生接口支撑；补查原module2778的三个schema宿主，确认fonts/family/source/scaling必填、scale缺失直接调用default_scale，以及重复这些字段进入错误分支。派生器/错误编解码仍复用serde/RON。原完整反序列化失败布局、unknown-field策略未执行验收，候选失败测试仅作重建合同校验。
- shape item的私有encode/with_scaling实现折入append分支，没有额外重写容器或通用编码器。

## Unknown / typed gaps

- 原TextFragment editor/buffer所有权、Attrs初始化/变更、dirty/redraw与prepare条件；不能拿常见GUI模型代替。
- 原CPU TextFragment布局run→glyph→primitives、cursor/selection/decoration、bounds、pixels-per-point缓存失效；本轮没有生成字形或伪度量。
- TextRenderer::get_glyph、AtlasAllocUsage、generation、atlas淘汰与空间不足路径；后续应直接复用原 cosmic-text fork 0.11.2 commit 3da2a14ea3d063875fdc55695431e74405302b0f 与 etagere 0.2.10，恢复游戏缓存/调用策略。
- GPU glyph纹理、upload/consume/render、Rhapsody与toolbar实际consumer接线；words正确不表示已执行SSBO shader或renderer。
- 原容器增长/allocator、Drop均属于依赖而未执行；没有恢复或模拟std内存分配。原serde运行、named resolver真实纹理索引生产者仍未验收。
- A/C未执行。

## 工具与持久性

Windows实际官方Python MCP SDK → ReVa transport / Ghidra engine，独占worker small-text-20261013，B patched SHA cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323。初始seed只有一字节placeholder，不能解释成函数无逻辑。按原LLVM PDB module625/626/1186精确范围和bytes SHA恢复20个本地clone函数体，并移除范围内的冲突placeholder。没有声称Rust ABI已恢复，没有修改原数据库或其他worker。

完整证据：initial-result.json、pdb-ranges.json、full-prepare-result.json、reopen-result.json。专属overlay checkpoint成功，正常停止后重开验证32/584/9字节核心函数体保持。后续assembly-result.json的指令查询passed，但job总ok=false，原因checkpoint文件atomic replace发生WinError5，失败报告保留；不能把该job宣称全成功。再后续schema-result.json查询及checkpoint成功。MCP analysis不是游戏原入口执行。

原native harness绑定RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03，在自有VirtualAlloc内保持PE代码/只读数据相对距离，仅调用已确认CPU函数。输入/output预分配，所有Vec增长分支避开；名称resolver使用显式host callback。未启动游戏、未走GPU、未调用原serde/panic/allocator。harness属于分析工具，不计作业务交付。

## 原接口完整映射：选定CPU子域

| 原接口或派生族 | 恢复位置/处理 | 证据级别与缺口 |
|---|---|---|
| FontTable::get/default | FontTable get/Default；HashMap直接复用std | get静态，default表空的容器语义；实际原HashMap lookup未执行 |
| FontTableEntry::scaled/default_scale | font_table.rs | 21组原scaled+default执行 |
| FontFamily Eq/Hash/Deserialize | enum derive | 类型/变体名和enum visitor静态；Eq/Hash依赖derive，不恢复哈希算法 |
| FontTable/FontTableEntry Deserialize 的各visit_str/visit_map/field decoder | serde derive +原RON fork | 原module2778 schema宿主static；见schema-result.json与schema-*.c |
| ShapeStream::new/is_empty/Clone | Vec default/is_empty/derive Clone | original new/is_empty执行；Clone使用Vec/enum依赖，不单独宣称原clone执行 |
| ShapeStream::fill_color/corner_radius/image的u32与str单态化 | consuming builders | 原module626/1186物理过程；builder规则static，后续原append执行 |
| ImageSource From<u32>/From<str>/From<String>/Deserialize | enum conversions；Deserialize仅字符串 | 原转换过程与deserialize_str调用static；原容量/heap布局不作为源码接口 |
| ImageScaling Deserialize | enum derive | Stretch/FitHeight/BlurExtend/Nine精确visitor+bit值static，原append输出已运行 |
| ShapeStreamItem/ShapeStream Deserialize及Clone派生族 | serde enum+transparent Vec、derive Clone | schema与VecVisitor static；serialization encoder未观察到，不补造 |
| RawShapeStream::new/default/clear/append | shape_stream.rs | 原new/clear与51组append；增长容器由Vec依赖承担 |
| RawShapeStream::encode | 业务合并进RawShapeStream::append三个分支 | 原module626 inline0x86cf（IPIID），宿主append三个实例；所做动作是Vec::extend/push写raw words，并非words访问器、RON或另一序列化格式。输出由51组原append覆盖；原私有完整Rust参数签名未知 |
| Image::with_scaling | 合并进append Image分支的scaling<<24 | 原module626 inline0x86ce，append offsets0x7b与0xdd附近取scaling；index保持低16位再配tag1。4种scale原运行覆盖；不补造未知原Image结构 |
| ShapeStream::encode_no_images + impl image_name_to_index | raw append +不可达resolver | 原物理函数/vtable/panic字面量static；候选成功/失败测试。未执行原panic/原分配路径 |
| RawShapeStream::words | 新增可读接口，返回借用slice | 重建consumer边界；不是把它映射为原Raw::encode |

## schema字段逐项与剩余状态

| 字段 | 原static规则 | 当前代码 | 对照状态 |
|---|---|---|---|
| FontTable.fonts | 必填；重复错误（0x141e44f60，fonts readonly0x142f8ec0a） | HashMap<FontFamily,FontTableEntry> Deserialize | static确认；原serde未执行 |
| FontTableEntry.family | 必填；重复错误（0x141e4a6b0，family0x142f8ec0f） | String Deserialize | static确认；候选失败测试 |
| FontTableEntry.scale | 缺失调用0x140b06780=1.0；重复错误；提供值不加限制 | serde default_scale f32 | static default分支+原default执行，21 scaled案例 |
| ShapeStream表示 | Vec<ShapeStreamItem>序列，无额外默认shape | serde transparent Vec | 原VecVisitor+发行draw[] static；候选解析 |
| FillColor载荷 | u32 newtype | serde u32 | static newtype分支；负/溢出候选拒绝 |
| CornerRadius载荷 | f32 newtype；编码再做radius规则 | serde f32 | static newtype分支+原encoded边界 |
| Image.source | 字符串必填，重复错误（0x141e3b460，source0x142f8ec1a） | ImageSource Deserialize_str | static确认；原serde未执行 |
| Image.scaling | 必填；重复错误，没有默认Stretch分支（同host，scaling0x142f8ec20） | ImageScaling enum必填 | static确认；候选缺字段失败 |
| 未识别字段/完整错误payload/错误位置 | 尚未逐路径核实，库ignored-any展开不作为游戏实现 | 保留serde默认策略 | 未验收，明确gap；不能称整个schema零差异 |
