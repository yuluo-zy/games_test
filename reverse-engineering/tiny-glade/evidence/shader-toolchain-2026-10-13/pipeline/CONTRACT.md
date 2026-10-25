# shader-pipeline B 发行合同恢复

恢复源码接在 reconstruction/rhapsody-rebuild/src/shader_pipeline，真实消费者为 shader_runtime 对 CompiledShaderPipeline 的加载与 layout 生产；compiler 从共享 stage 提取 DXC 请求。公共 Speedy 0.8.6、const-fnv1a-hash 1.1.0、Rust DefaultHasher 与原 spirq fork 均直接复用。

## 原发行格式

LittleEndian Speedy 声明顺序：Vec<Stage {u32 tag, String entry, Vec<u32> SPIR-V}>、Vec<String>依赖路径、ShaderReflection。

ShaderReflection：Vec<HashMap<u32 BindingIndex, Binding>>（vector位置就是descriptor set）、Option<UniformBlock>、HashMap<String, BindingLocation>、HashMap<u64 BindingNameHash, BindingLocation>、HashMap<String, VertexInput>、Option<[u32;3] local_size>、u32 push_constant_size。UniformBlock：u32 byte_size、字符串/哈希到字段u32偏移的两张map、BindingLocation{set,binding}；不是Vulkan push constant，404样本它的绑定对应$Globals UBO。最后u32通过原raster/compute VkPushConstantRange消费者确认，不能误称flags。

Binding：kind u32 0 StorageBuffer、1 UniformBuffer、2 CombinedImageSampler、3 StorageImage、4 SampledImage、5 Sampler；array u32 0 Fixed(u32)、1 Runtime；String name。tag2不在404中，由已验证原descriptor消费者表补证；运行时agent初审修正了我最初误命名AccelerationStructure。

Type：0 Scalar、1 Vector、2 Matrix、3 Image、4 CombinedImageSampler、5 SampledImage、6 StorageImage、7 Sampler、8 SubpassData、9 Array、10 Struct、11 AccelStruct、12 DeviceAddress、13 DevicePointer。资源tag3..8/11/12在原reader对应无载荷分支，不能把public spirq的Image字段当存储字段。Scalar：0 Void、1 Boolean、2 Signed(u32字节数)、3 Unsigned(u32)、4 Float(u32)；Vector追加u32分量数；Matrix追加u32向量数、Option<AxisOrder>、Option<u64 stride>；Array为Type、Option<u32 repeat>、Option<u64 stride>；Struct为Option<String>名和Vec<Member {Option<String>名,u64 offset,Type}>；DevicePointer递归Type。尺寸委托原spirq，opaque资源在仅尺寸查询的内部投影中使用公开无尺寸载体，完整类型投影遇opaque返回None。

为回写保留原HashMap顺序而保存键值对Vec；生产消费者必须使用effective_descriptor_sets、effective_vertex_inputs或反向查询，保留最后写入覆盖。Speedy的原read_from_buffer允许未消费尾字节；当前保存trailing_bytes并回写。非法枚举、截断、UTF8失败由原Speedy返回，不臆造读取器。

## 原文件键

compiled_file_path只把source中的正斜线替换为下划线，不归一化、不去leading slash、不删除source前缀。Compute为compiled-shaders/{source}_{entry}_{hash:x}.bin，Raster没有entry字段。defines hash为每对(String,String)用DefaultHasher默认固定种子后XOR，空为0；顺序不影响、重复两次抵消，不是对Vec整体hash。

原shader_variants::get_raster_shader_variants @0x1417388a0 在原rdata 0x142ccd730构造SHADER_VARIANT_DEPTH_ONLY、值0/1。现Rust hash与发行b903e43ffb3da915/f0adff76827e73ff键对照；原macro字节和指令见raster-macro-evidence.json/raster-variants.asm.txt。

BindingNameHash::new @0x140c87660只对末尾_dims设limit=len-5并加1。原const-fnv库limit=0表示全文，所以单名_dims不能按空串hash。with_suffix_dims @0x140c87650为wrapping加1。原指令见offline symbols/局部反汇编证据。

## 证据与验证界限

完整schema探索404资产全部按声明字段消费至EOF，170 compute +234 vertex +234 fragment，不使用magic扫描。Rust tests要求同样404逐字节decode/encode回写、原SPIR-V入口对照、names/hashes/UBO字段关系、顶点尺寸、真实macro键、非法数据与未出现Type标签。实际运行结果由根集中cargo报告追加，不能把探索器结果替代Rust测试。

verify-keys.py尝试原指令Unicorn oracle，但当前Python缺unicorn，未运行，不能记为原入口差分通过。此脚本不是恢复交付，保留失败报告。模板/声明和函数覆盖不代表整个包完成。

缺口：原ShaderCompilerArtifacts::load_compiled的完整anyhow错误消息/上下文与缓存拥有关系、原描述对象所有optional字段、全shader-compiler反射/打包生产者、A/C版本和整游戏consumer未验收。新Rust API采用业务字段，无原Rust ABI声明。原crate公开来源仍未知，local不等于已证实私有唯一实现。

## 持久分析overlay

worker shader-pipeline-20261013，project D:/game/tools/reverse-runtime/shader-pipeline-20261013/project/TinyGlade.gpr；原seed及发行文件保留，没有同步旧Mac overlays或原数据库。prepare_functions按原RNE exception/public-range字节SHA限定body；未修改原型/签名/类型，Ghidra自动推导参数只作阅读辅助，不当ABI。各job事务外checkpoint已成功；最终由根统一正常stop保存。

实际集中验证：cargo test --locked --test shader_pipeline_contract 5/5通过，404完整typed字段与回写、keys和损坏边界通过；见validation.json及cargo-test-verified.log。这是B发行合同闭环，不是全crate所有函数或全原错误副作用验收。

接口分类：ComputePipelineDesc::new/with_entry与RasterPipelineDesc::new_vertex_fragment是候选便利构造，不作为原构造函数全部默认值/Option/所有权恢复。空defines只是新API选择，depth-only宏0/1由原shader_variants消费者另行追加，未把此producer复原到构造层。真正原对照合同为两个compiled_file_path、defines pair XOR键、BindingNameHash与完整发行schema读写。描述字段/entry命名不声明原作者完整结构。
