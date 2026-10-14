# -*- coding: utf-8 -*-
"""render-nani 域章节填充（tg_rn1）：把 B 阶段（B_i_api/B_i_infra/B_i_nani/B_n_world/B_n_sys/
B_a4_compute/B_a4_draw/B_d_cb_a/B_d_cb_b/B_d_gap/B_x_out）、wp1/wp3 合同与 reconstruction 源码
（地址标注）中的既有结论映射进 17 个骨架章（chapters/render/{pass_builder,nani}/**）。
新反编译数 = 0（全部函数已有证据；PLAYBOOK §5 不求穷尽）。
运行：python -I fill_render_nani.py（在证据目录下）。
"""
import re, os, sys

BASE = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters/render"

# ---------------------------------------------------------------- 文件作用
PURPOSE = {
 "pass_builder/bind_resources.rs.md": "BindGameResources trait：把颜色库（ColorLibrarySsbo）与软件 BVH 组（RtTriangles/RtTriangleMeta/RtBvh + WallColorIds）绑定进 ComputePass / DrawBuilder（O：0x1405896e0 / 0x1415fc230 / 0x1416286f0 / 0x1416289d0；B_i_api §3 复核槽名与哈希）。",
 "pass_builder/compute_pass_builder.rs.md": "ComputePass：400B compute pass 声明层（bindings / access_ranges / external_bindings / PassBuilder / UniformBinder / 着色器注册表 / 设备 / PassResources）+ dispatch 家族（模式 0..4）+ bind_texture（RG/受管/外部路径）。O 锚点见 src 头注。",
 "pass_builder/draw_builder.rs.md": "DrawBuilder：0xf8B 按值消费的 draw 声明器（NoDropOnlyTake<DrawPacket> + Option<DynamicConstantsBinding> + &mut RasterPass + Arc<CompiledShaderPipeline> + flag@+0xf0），draw 种类 0/1/2/3/4/5/7/8，终结操作无返回值。",
 "pass_builder/draw_list.rs.md": "DrawListItem / EntityDrawParams：CPU draw-list 项（0x80B 元组 = 0x50B params 首字段 + vao + shader + i32 排序键 + 模板参考 + 回调 fn；无 PDB 独立过程，全部内联进宿主 pass）。",
 "pass_builder/draw_stream.rs.md": "DrawPacket（0xb8B 绘制包，种类判别@+0x40）/ RasterPass（~0x3e8B 光栅 pass：附件数组 + RenderPass 去重缓存）/ DrawStream（无过程空壳）+ TextureBindingKey（u64：0x4000_0000_0000_0000|id，视图异或 calculate_hash）。",
 "pass_builder/framebuffer_binding.rs.md": "ColorAttachment（0x20B 枚举：tag0 引用@+8 / tag1 视图字@+4 引用@+0x18；访问 0x36/0x2e）/ DepthAttachment（tag0 可写 0x2f / tag1 只读 0x17 / 其它无深度；+0x10 清除 u32、+0x14 f32）。",
 "pass_builder/pass_resource_access.rs.md": "SSBO 访问登记三件套（read/write/write_concurrent_access_buffer<T>，53/37/9 实例 = 3 个泛型体）+ 纹理访问 trait（TextureAccess：RG 已导入 / 受管 Handle / TexturePreset / WriteTextureUnchecked，as_texture 返回 3 标签枚举）。",
 "pass_builder/pass_resources.rs.md": "PassResources：PassBuilder::render 的 UserData（56B = 7 个帧级资源指针；new_from 按 7 个 128 位 TypeId 查 ECS world 资源表）。",
 "pass_builder/shader_uri.rs.md": "ShaderUri 解析（三次 memrchr：':'→entry、'/'→文件名、'.'→stem，无点 unwrap）+ AsComputeShaderSource trait（&str/String 通用实现 + 18 个游戏实现：9 个 nani cull shaders + DoF×5 + bin_lights + GaussBlur + TgsrMain + GenerateStrokes）。",
 "pass_builder/uniform_binder.rs.md": "UniformBinder：uniform 块映射内存 + 按 64 位名字哈希查偏移写值（四道守卫静默返回；bool 写 u32；越界 slice_start_index_len_fail）。",
 "nani/buffers.rs.md": "nani 绘制参数：MAX_DRAWS_PER_LIST 27 项表（0x142aa14c0，与 max_instances 两张表逐值相同）+ NaniDrawListParams（0x70B = 4 Vec<u32> + 4 u32 汇总）+ NaniDrawListOffsetDefaults（0x1b0B = [u32;108]）+ 7 个 SSBO 载体。",
 "nani/draw_ext.rs.md": "NaniDrawExt：DrawBuilder 的 nani GPU-driven 间接绘制扩展（nani_draw 0x140904ae0，1428B：subset 读锁 → list_index=kind*4|变体 → 三个间接 SSBO 绑定 → count/raw 分派）。",
 "nani/gpu_world.rs.md": "GpuWorld：CPU↔GPU 实例桥（27×0xc0 RwLock<Option<GpuWorldSubset>> 槽 + total_instance_count + Box<NaniDrawListParams> + 两个 TypedStorageBuffer）；OwnedGpuWorldSubset = 每 subset 的稀疏实例仓库（IndexCondenser + 紧凑 key 数组 + 类型化数据缓冲 + min/max 原子）。",
 "nani/instance_data.rs.md": "每类 nani 实例的 GPU 布局类型：元素尺寸 O（0x30/0x40/0x60/0x70 分组），字段布局 U（归 nani_update_* 与 shader 侧）；CurvedHalfTimberInstanceData 三个构造器逐字节 O；WindowGlassFlags / NaniUberInstanceFlags / CurvedHalfTimberCaps 位域。",
 "nani/mesh_atlas.rs.md": "MeshAtlas / MeshAtlasPart：把多个 Mesh 合并进一张大网格（≤0x40 part；属性名集一致断言；重复名 panic；包围半径）。",
 "nani/systems.rs.md": "nani ECS 系统族：nani_maintain_instances（删→建→改三阶段）+ 20 个 nani_update_*（组件→实例载荷打包 + update_instance_data）+ nani_flush_instance_updates（27 槽冲刷 + 计数滚动）。CPU 侧全部 O。",
 "nani/trimesh.rs.md": "Trimesh / TrimeshBuilder：CPU 三角网格增量构造（≤64 三角形 meshlet 切块、属性对齐降序、build 打包 GPU 缓冲并算包围球）。",
}

# ---------------------------------------------------------------- 逐函数：EXACT[file][name] = (作用, 结论, 证据)
E = {}

def exact(f, name, desc, concl, ev): E.setdefault(f, {})[name] = (desc, concl, ev)

PB = "pass_builder"
# ---- bind_resources
exact(f"{PB}/bind_resources.rs.md", "country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>",
 "登记 ColorLibrarySsbo 读访问：read_access_buffer<ColorLibrarySsbo>（access 0x1a，组 1）→ 注册表命中 hash 0x52ec6a17beb8fef0 追加 AccessRange；**无槽表**",
 "O", "B_i_api §3；B_a4_compute §2；wp1/contracts/bind_resources.md §2；src/bind_resources.rs 头注（0x1405896e0/0x1415fc070 ICF 同体）")
exact(f"{PB}/bind_resources.rs.md", "country_core::render::pass_builder::bind_resources::impl$0::bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>",
 "同上，但经 HashMap<&str,PassResourceSlot>::rustc_entry 槽表（已占槽不重复登记）；access 0x1f",
 "O", "B_i_api §3；B_a4_draw §2；wp1/contracts/bind_resources.md §2（0x1416286f0）")
exact(f"{PB}/bind_resources.rs.md", "country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::compute_pass_builder::ComputePass>",
 "取 RtTriangles/RtBvh 缓冲（SsboId 库查 + 名字断言 + unwrap）→ 4 槽 read_access_buffer（Tri/Meta/Bvh/WallColorId，0x1a）→ 2×write_uniform<u32>（Tri/Bvh 缓冲字）；ComputePass 版**不**写 CPU 缓冲",
 "O", "B_i_api §1.3/§3；B_a4_compute §2；wp1/contracts/bind_resources.md §3（0x1415fc230）")
exact(f"{PB}/bind_resources.rs.md", "country_core::render::pass_builder::bind_resources::impl$0::bind_ray_tracing<country_core::render::pass_builder::draw_builder::DrawBuilder>",
 "同上 + WallColorId 回读（read_storage_buffer<WallColorIdSsbo> 槽 \"WallColorIds\"，xxh3=0xaf42315d263aa22a）；DrawBuilder 版写 CPU 缓冲（+0x198/+0xa0）供 CPU 侧使用",
 "O", "B_i_api §1.2/§2（0x1416289d0，4167B）；B_a4_draw §2；wp1/contracts/bind_resources.md §3")
# ---- pass_resources
exact(f"{PB}/pass_resources.rs.md", "country_core::render::pass_builder::pass_resources::PassResources::new_from",
 "按 7 个 128 位 TypeId（A..G）查 world 资源表；B 可选（缺→0）、其余缺失 Option::unwrap；56B 布局 +0 着色器库/+8 纹理库/+0x10 SSBO 库/+0x28 Mutex<DynamicConstants>，+0x18/+0x20/+0x30 类型 U",
 "O", "B_a4_compute §2/§3.6；wp1/contracts/pass_resources.md；src/pass_resources.rs 头注（0x140909310）")
# ---- uniform_binder
exact(f"{PB}/uniform_binder.rs.md", "country_core::render::pass_builder::uniform_binder::UniformBinder::new",
 "反射 +0xd8==0（无 uniform 块）→ None（niche 标签 0xa）；否则 DynamicConstants::allocate(size=reflection[+0x128]) → into_uniform_buffer，记录 set/绑定号到 +0x28",
 "O", "B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b420）")
exact(f"{PB}/uniform_binder.rs.md", "country_core::render::pass_builder::uniform_binder::UniformBinder::write_optional_uniform",
 "同 write_uniform 守卫；值由闭包**惰性**求（命中后才调用）；返回 bool",
 "O", "B_a4_compute §2（0x14096bd20 系）")

# ---- draw_stream
exact(f"{PB}/draw_stream.rs.md", "country_core::render::pass_builder::draw_stream::DrawPacket::new",
 "0xb8B 绘制包：+0x40 种类 u32=0x1_0000_0003（3 变体 + 计数）、+0x70 HashMap<BindingIndex,GpuOwned<DescriptorSetBinding>>、+0xa0 shader、+0xa8 光栅状态、+0xac blend 1.0、+0xb0 variant=0",
 "O", "B_a4_draw §1/§2；wp1/contracts/draw_stream.md §1（0x140867850）")
exact(f"{PB}/draw_stream.rs.md", "country_core::render::pass_builder::draw_stream::TextureBindingKey::with_view",
 "u64 键 = 0x4000_0000_0000_0000|id；有视图时 ^ calculate_hash(view)（0x140867940）",
 "O", "B_a4_draw §1；wp1/contracts/draw_stream.md §4")
exact(f"{PB}/draw_stream.rs.md", "country_core::render::pass_builder::draw_stream::impl$8::to_texture_binding_key",
 "&Texture → 恒 unreachable!(\"Raw textures must not be used as rg resources\")（draw_stream.rs:204:9）",
 "O", "B_a4_draw §1；wp1/contracts/draw_stream.md §4（0x1408eefd0）")
exact(f"{PB}/draw_stream.rs.md", "country_core::render::pass_builder::draw_stream::RasterPass::new_impl",
 "颜色附件 ≤5（ArrayVec 溢出 panic）→ 每附件 add_resource_access(id, 0x36/0x2e, 1)（tag 决定）；深度 0x2f/0x17 组 1；PassResources::new_from + 2 描述符克隆 + 两张空表",
 "O", "B_a4_draw §2；wp1/contracts/draw_stream.md §2（0x140867970）")
exact(f"{PB}/draw_stream.rs.md", "country_core::render::pass_builder::draw_stream::RasterPass::render",
 "置已渲染 → build_render_pass_desc（上限 8 附件）→ 深度 clear_input → HashMap<RenderPassDesc,RenderPass> 去重（命中克隆/未命中 create_render_pass+插入）→ PassBuilder::render 提交",
 "O", "B_a4_draw §2；wp1/contracts/draw_stream.md §2（0x140868370）")
# ---- framebuffer_binding
exact(f"{PB}/framebuffer_binding.rs.md", "country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc",
 "21B：ptr = *(self + 8 + (tag==1)*0x10); return ptr+4（两变体都指向 {id:u32@0, TextureDesc@4}，引用偏移不同）",
 "O", "B_a4_draw §1（0x1409092f0）")
# ---- shader_uri
exact(f"{PB}/shader_uri.rs.md", "country_core::render::pass_builder::shader_uri::ShaderUri::parse",
 "48B sret = 3×&str 对：rfind(':')→path/entry（无冒号 entry=None）、rfind('/')→文件名、rfind('.')→stem；无点 → unwrap panic",
 "O", "B_i_api；wp1/contracts/shader_uri.md §1；src/shader_uri.rs（0x140968960；纠正旧 wp1 的 whole/name/entry 语义）")
exact(f"{PB}/shader_uri.rs.md", "country_core::render::pass_builder::shader_uri::impl$1::source_hash<ref$<str$> >",
 "calculate_hash(&(self_str, &()))：SipHash-1-3 零密钥，str 写字节后补 0xff",
 "O", "B_n_sys cull_shaders §1；wp1/contracts/shader_uri.md §2（0x1408a4300）")
exact(f"{PB}/shader_uri.rs.md", "country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<alloc::string::String>",
 "ShaderUri::parse → ComputePipelineDesc{path: String, entry: \"cs_main\", defines: []}.with_entry(uri.entry)",
 "O", "B_i_api §3.3；wp1/contracts/shader_uri.md §2（0x140056d30）")
exact(f"{PB}/shader_uri.rs.md", "country_core::render::pass_builder::shader_uri::impl$1::as_compute_pipeline<ref$<str$> >",
 "同上（&str 实例；0x1405cf0f0，ICF 同体）",
 "O", "B_i_api §3.3；wp1/contracts/shader_uri.md §2")
# ---- nani buffers
exact("nani/buffers.rs.md", "country_core::render::nani::buffers::nani_calculate_draw_list_params",
 "27 kind × 4 list 前缀和：cmd_offsets += T[k]；draws 钳制（!FORCE_FINE && (k>=2||FORCE_NO_FINE) → 0x40）；fine=draws>>5（掩码 **0x3fe** 非 0x3ff）；batch=(fine>>10)+(fine&0x3fe!=0)；循环体机器码展开 4 份（0x1408bb0bc 系）",
 "O", "B_n_world §1/§2.8/§2.9；B_i_nani §2.1（LISTS_PER_KIND=4、108 项、总 3788800/71936/2048/8）；wp3/contracts/buffers.md（0x1408baed0；与 default_value 是**两个独立函数**）")
exact("nani/buffers.rs.md", "country_core::render::nani::buffers::impl$9::default_value",
 "NaniDrawListOffsetDefaults（[u32;108]，0x1b0B）：offsets[4k+j] = base + j*T[k]，base += 4*T[k]；跳表 0x142aa1618",
 "O", "B_n_world §1/§2.9（0x1408bb650；wp3 合同称两者反编译重叠 = Ghidra 跳表误读，已纠正）")

# ---------------------------------------------------------------- 家族规则
FAMILY = []  # (file, regex, (desc, concl, ev))

def fam(f, pat, desc, concl, ev): FAMILY.append((f, re.compile(pat), (desc, concl, ev)))

# ---- pass_resource_access 家族
PA = f"{PB}/pass_resource_access.rs.md"
fam(PA, r"::read_access_buffer<", "同一泛型（53 实例，ICF）：AssetSsboLibrary::get_rg_buffer<T>（资源号=句柄+0x10）→ add_resource_access(access 0x1a, 组 1) → 绑定数≥0x100 unwrap → 追加 0x44B RenderPassBinding（标签 3）；**不经注册表**",
 "O", "B_a4_compute §3.1/§3.3；wp1/contracts/pass_resource_access.md §1（0x1402b00e0/0x140970620 系）")
fam(PA, r"::write_access_buffer<", "同一泛型（37 实例）：同 read，访问 0x32/0x37、组 0、标签 4（0x140bd6c60），返回追加前下标",
 "O", "B_a4_compute §3.1；wp1/contracts/pass_resource_access.md §1（0x140973720 系）")
fam(PA, r"::write_access_buffer_concurrent<", "同一泛型（9 实例）：同 write，同步组 = 调用方实参",
 "O", "B_a4_compute §3.1；wp1/contracts/pass_resource_access.md §1（0x140975c20 系）")
fam(PA, r"::impl\$0::as_texture$", "TexturePreset 的 TextureAccess：as_texture 返回 3 标签枚举（0=RG {extent,id,desc}、1=受管、2=按名）；dims=extent_2d",
 "O", "B_i_api §3.2；B_a4_compute §3.4（0x140a59800/0x140a59860）")
fam(PA, r"::impl\$0::dims$", "TexturePreset：恒 TextureDesc::extent_2d（36B ImportedTexture 带 desc）",
 "O", "B_i_api §3.2（0x140a59860）")
fam(PA, r"::impl\$2::as_texture$", "受管 &Handle<Texture>：标签 1 {extent, 图像视图(get_vk_image_view unwrap), 句柄克隆}",
 "O", "B_a4_compute §3.4（0x1408ef010）")
fam(PA, r"::impl\$2::dims$", "受管 Handle：extent = 受管纹理 desc",
 "O", "B_a4_compute §3.4（0x1408ef170）")
fam(PA, r"::impl\$3::as_texture$", "RG 已导入纹理（&RgTexture/&RgHandle 共用）：标签 0",
 "O", "B_a4_compute §3.4；B_i_api §3.2（0x1409248b0）")
fam(PA, r"::impl\$3::dims$", "RG 已导入：extent_2d(desc@+0x18)",
 "O", "B_a4_compute §3.4（0x1409248f0）")
fam(PA, r"::impl\$6::as_texture$", "WriteTextureUnchecked：与 impl$3 机器码**相同地址**（ICF 合并，0x1409248b0）",
 "O", "B_a4_compute §3.4；B_i_api §3.2")
fam(PA, r"::impl\$6::dims$", "WriteTextureUnchecked：同 impl$3（ICF 同体）",
 "O", "B_a4_compute §3.4（0x1409248f0）")

# ---- compute_pass_builder 家族
CP = f"{PB}/compute_pass_builder.rs.md"
fam(CP, r"::ComputePass::new<", "同一泛型（14 实例）：ShaderUri hash → PassResources::new_from（7 资源）→ get_compute_by_name（未命中 as_compute_pipeline+ComputeShaderProgram::new+add_compute 缓存，命中 clone）→ 设备 unwrap → UniformBinder::new → bind_color_library；返回 400B",
 "O", "B_a4_compute §2/§3；wp1/contracts/compute_pass_builder.md §2（0x1405876a0/String、0x140587eb0/&str、nani cull ×5 0x1415fcf10 系；2311B 带 PERM_* 变体与 1943B 不同形态）")
fam(CP, r"::impl\$4::bind_texture_to_compute_pass<", "RG 路径（O）：注册表未命中 → 只写维度 uniform（键 name+1，闭包惰性）；命中 → assert set==0 → 取纹理 → 绑定数检查 → add_resource_access(res, access, write?0:1) → Ref::bind_view（整块 TextureViewDesc，+0x3c 资源号、+0x40 写标志）→ AccessRange → 维度 uniform；外部路径（受管/FromDisk）见 U",
 "O", "B_i_api §3.2；B_a4_compute §3.4（0x140905260 系 8 实例：Preset 0x140363db0、&Texture 0x1408fcd40、&Handle 0x140155400、&mut Handle 0x140155d80、ReadOnlyHandle 0x14096f090 等）")
fam(CP, r"::ComputePass::write_storage_buffer<", "同一泛型（34 实例）：write_access_buffer<T>(access 0x32)",
 "O", "B_a4_compute §2（0x1416044c0 系/0x141607cf0 系）")
fam(CP, r"::ComputePass::write_storage_buffer_concurrent<", "同一泛型（9 实例）：write_access_buffer_concurrent<T>（0x37，组=调用方；原版实例如 NaniDrawListSsbo 0x141608450 访问码 0x37 实测）",
 "O", "B_i_api §1.4；B_a4_compute §2（0x141608450 系）")
fam(CP, r"::ComputePass::write_to_cpu_mapped_storage_buffer<", "同一泛型（3 实例）：write_access_buffer<T>（0x32）后按 hash 解析 resource_index 追加 access_range",
 "O", "B_a4_compute §2；wp1/contracts/compute_pass_builder.md §6（0x141609770）")
fam(CP, r"::impl\$3::read_storage_buffer<", "同一泛型（25 实例）：read_access_buffer<T>(access 0x1a)，再查注册表追加范围",
 "O", "B_a4_compute §2/§3.3；wp1/contracts/compute_pass_builder.md §6（0x14160c290 系）")
fam(CP, r"::ComputePass::dispatch_with<", "同一泛型（9 实例）：模式 3，闭包 400B env 装箱 + 函数指针入记录",
 "O", "B_a4_compute §2/§3.5；wp1/contracts/compute_pass_builder.md §5（0x14160a2e0）")
fam(CP, r"::ComputePass::dispatch_indirect_with<", "同一泛型（6 实例）：模式 4 = 模式 3 + 从 args+0x10 读 indirect resource_index + add_resource_access",
 "O", "B_a4_compute §2/§3.5（0x14160b850）")

# ---- draw_builder 家族
DB = f"{PB}/draw_builder.rs.md"
fam(DB, r"::impl\$2::read_storage_buffer<", "同一泛型（43 实例）：槽表 entry（空位 read_access_buffer：access 0x1f、组 1）→ 名字表 → packet.accesses.push",
 "O", "B_a4_draw §2（0x141629e90 系；WALL_COLOR_ID_SLOT_NAME=\"WallColorIds\" 0x14162c290）")
fam(DB, r"::impl\$3::bind_texture_to_draw_builder<", "RG 路径按 TextureBindingKey 去重槽 + 登记访问/绑定；Preset 允许写（组 0）；FromDisk/Raw：assert!(!access.is_write())，Raw 另 assert!(view.is_none())，get_vk_image_view 后 {0,view,5} 绑定；末尾 with_suffix_dims 维度 uniform；受管句柄写访问 panic \"Trying to bind {:?} for writing, but it's not writable\"",
 "O", "B_i_api §3.2；B_a4_draw §2（0x1403728a0/0x1408ee800/0x1408fd6c0/0x140908610/0x140908c50/0x14096f650）")
fam(DB, r"::impl\$2::optional_uniform<", "同 uniform：未命中静默、块外越界 slice_start_index_len_fail、bool 以 u32 写",
 "O", "B_a4_draw §2（0x141629a20/f32、0x141629b90/Mat4、0x141629d20/[u32;2]）")

# ---- shader_uri 家族（cull_shaders + 各 compute pass 实现）
SU = f"{PB}/shader_uri.rs.md"
fam(SU, r"cull_shaders::impl\$(10|14|18|22|26|30|34|38|42)::as_compute_pipeline$",
 "NaniCullAndBucket 族 as_compute_pipeline（9 个同构）：ComputePipelineDesc{path: 各变体 URI, entry: \"cs_main\", defines: [(\"PERM_oc_pass\", i32 Display)]}",
 "O", "B_n_sys cull_shaders §1（0x141623120..0x141625590；路径长 29/39/37/34/42/47/37/35/42）")
fam(SU, r"cull_shaders::impl\$(30|34|38|42)::source_hash$",
 "source_hash = calculate_hash(&(\"<path>:cs_main\", &i32))（SipHash-1-3 零密钥；串+0xff+4 字节）",
 "O", "B_n_sys cull_shaders §1（0x141624770/0x141624c10/0x1416250c0/0x141625560）")
fam(SU, r"systems::render_loop::(?:compute_passes::)?(?:dof::impl\$(?:3|7|11|15|19)|bin_lights::impl\$2|rasterize_terrain_stroke::impl\$8|taa::impl\$1|vermeer::impl\$7)::as_compute_pipeline$",
 "游戏 compute pass 结构体（DoF×5/BinLights/GaussBlur/TgsrMain/GenerateStrokes）的 as_compute_pipeline：同构（path=各自 URI、entry=cs_main 或 cs_halfres(_all)、defines 经 pipeline_defines 注入 PERM_*）",
 "O", "B_i_api §3.3/§5.6；wp1/contracts/shader_uri.md §3（URI 表；0x140a5d4f0..0x1408aab70）")
fam(SU, r"systems::render_loop::(?:compute_passes::)?(?:dof::impl\$(?:3|7|11|15|19)|bin_lights::impl\$2|rasterize_terrain_stroke::impl\$8|taa::impl\$1|vermeer::impl\$7)::source_hash$",
 "source_hash 同构 = calculate_hash(&(path:entry, &self))",
 "O", "B_n_sys cull_shaders §1 同法；wp1/contracts/shader_uri.md §2")

# ---- uniform_binder 家族
UB = f"{PB}/uniform_binder.rs.md"
fam(UB, r"::UniformBinder::write_uniform<", "同一泛型（10 实例，写宽不同）：守卫①反射 valid ②表非空 ③名字哈希（64 位直接作 hashbrown 哈希）命中取偏移 ④偏移≤len；写 u32/i32/f32/bool→u32 4B、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B；越界 slice_start_index_len_fail；守卫失败静默返回",
 "O", "B_a4_compute §2/§3.3；wp1/contracts/uniform_binder.md §1（0x14096b570..0x14096bc10）")
fam(UB, r"::UniformBinder::write_optional_uniform<", "同一泛型（4 实例）：同守卫；值由闭包惰性求；返回 bool",
 "O", "B_a4_compute §2（0x14096bd20/0x14096be40/0x14096bf50/0x14048f750）")

# ---- gpu_world 家族
GW = "nani/gpu_world.rs.md"
fam(GW, r"::impl\$36::register_required_components<", "同一泛型（19 实例，1776B 系）：bevy 派生展开 —— ComponentsRegistrator::register_component<InstanceDataMarker<T>>（纯 ECS 胶水，无游戏数值）",
 "O", "B_n_world §1/§7（0x141572d80 系）")
fam(GW, r"::impl\$14::type_name<", "同一泛型（21 实例，13B）：返回类型名字符串（vtable +0x18）",
 "O", "B_n_world §1/§2.6（0x141572130 系）")
fam(GW, r"::impl\$14::as_any<", "同一泛型（21 实例，11B）：Any 降级（vtable +0x20；as_any 与 as_any_mut 同址 ICF）",
 "O", "B_n_world §1/§2.6（0x141572280 系）")
fam(GW, r"::impl\$14::as_any_mut<", "同一泛型（21 实例）：AnyMut 降级（vtable +0x28）",
 "O", "B_n_world §1/§2.6（0x141572280 系同址）")
fam(GW, r"::impl\$14::gpu_flush<", "同一泛型（21 实例，9B 薄封装）：ShadowedBuffer<T> 冲刷标记（vtable +0x30）",
 "O", "B_n_world §1（0x1415723d0 系）")
fam(GW, r"::impl\$14::copy<", "同一泛型（21 实例，按元素尺寸 0x40/0x30/0x70/0x60 分 4 组）：数据 Vec 内元素复制（copy(dst,src) 目的在前）+ CAS min/max",
 "O", "B_n_world §1/§2.6（0x1415724f0/0x1415725b0/0x141572660/0x141572760）")
fam(GW, r"::OwnedGpuWorldSubset::update_instance_data<", "同一泛型（21 实例，O）：fast-path compact[index]==key.id 直命中，否则 HashMap 查找回写 key.index（缺 → unwrap）；写紧凑 key（Vec3 + f16 radius + mesh_index）与 T 数据；CAS min/max",
 "O", "B_n_world §1/§2.2；B_n_sys §3（0x14156db40 系/0x14156fff0 系）")

# ---- systems 逐函数（B_n_sys 表）
SYS = "nani/systems.rs.md"
def sysx(name, desc, ev_extra=""):
    exact(SYS, f"country_core::render::nani::systems::{name}", desc, "O", f"B_n_sys §1/§4{ev_extra}；wp3/contracts/systems.md §3；src/nani/systems.rs")

sysx("nani_update_curved_halftimber", "CurvedHalfTimber 打包（kind 0x18，0x40 载荷，**&mut 组件直写**触发 tick）：选色 direct!=0→HalftimberColorIds[key]（越界 panic）/PublicWalls.roofs→Roof(byte@+0x40==2→u32@+0x44)/ids；写 data[3]（pad_0c）；无 cap 掩码；position=data[0..3]/sort=1.0", "；§4 curved")
sysx("nani_update_stone_floor", "StoneFloor 打包（kind 9）：Roof(0x58B)/Floor(0x28B)/RoofAnimation(8B 缺省 1.0)；Roof 与 Floor 都缺 → unwrap；circle_tangent_u0（Circle2d::get_tangent_at_u(0.0) 0x140c97660）U=0", "；§4 stone_floor")
sysx("nani_maintain_instances", "实例维护三阶段（**先删→后建→再改**）：RemovedComponents→Local map remove_entry（缺→unwrap，行 51:41）→remove_instance；新增 Query<Entity,&NaniInstanceDesc,&Transform> Without<NaniActiveInstance> → MeshAtlasLibrary get_by_name（Named 需 part.subset==kind，缺→panic 行 63:21 文案）→ extend_no_data（closure$1 0x141565950 收 12B {u32,u32,kind u8,[u8;3]}）→ Local HashMap insert + Commands Insert<NaniActiveInstance>；容量满 → warn_once(\"Instance limit reached…\")；Changed<NaniInstanceDesc> → parts.len()==1 断言 → update_instance_mesh", "；§2 maintain")
sysx("nani_update_solid_vertex_color", "纯色顶点色（kind 3，0x40）：通用 12 字仿射（Mat3::from_quat 各列×scale + 平移）；w12=SemiTransparent 在场、w13=FlatLighting 在场", "")
sysx("nani_update_solid_vertex_color_window", "窗纯色（kind 4）：w12=Semi、w13=ColorId.0", "")
sysx("nani_update_solid_albedo_map", "贴图（kind 0xf）：w12=[0,1,8,2,3,4,5,6,7][NaniSolidTexture byte]（表 0x142fd38c8）", "")
sysx("nani_update_solid_albedo_map_emissive", "发光贴图（kind 0x10，机器码与 albedo_map **同体**）", "")
sysx("nani_update_preplaced_flower", "预置花（kind 0xe，载荷 **0x30**）：TerrainRockMaskSsbo 掩码（u/v=min(511,trunc(clamp01(c/130+0.5)*512))，位 1→隐藏）/水采样>0.01/4 路径栅格 clamp01 和>0.1 → 隐藏；隐藏载荷=单位旋转、scale 1、平移(0,-100,0)、sort_key 0", "；§4 preplaced_flower")
sysx("nani_update_plant", "植物（kind 0xa，0x30）：position/sort_key 覆写且保持 mesh_index（ECS 测试）", "")
sysx("nani_update_solid_vertex_color_window_glass", "窗玻璃（kind 5）：ColorId|semi<<31（ColorId bit31→unwrap \"value out of bounds for field WindowGlassFlags.color_id\"）、窗光 f32、AlbedoMap 在场、bindless 下标（缺 name → unwrap）", "")
sysx("nani_update_solid_vertex_color_bricks", "砖（kind 6）：pack_2x16f_uint(f2,f3)（0x140c90df0 = f16(a)|f16(b)<<16）", "")
sysx("nani_update_gothic_window_bricks", "哥特窗砖（kind 7，0x70 载荷）", "")
sysx("nani_update_wren", "鹪鹩（kind 0x11，0x70 载荷）", "")
sysx("nani_update_lantern_emissive", "灯笼发光（kind 0x12）", "")
sysx("nani_update_plaster", "石膏（kind 0x13，0x30）：position=(0,0,0)/sort_key=1.0；断言 w.simplified_curve_2d.is_valid()（文案 O）", "")
sysx("nani_update_cloth", "布料（kind 0x14）：+ClothlineCloth + NormalMap + Res<AssetTextureLibrary>", "")
sysx("nani_update_smoke", "烟（kind 0x15）", "")
sysx("nani_update_balcony_door", "阳台门（kind 0x16）", "")
sysx("nani_update_uber", "Uber（kind 0x17）：flags = albedo*1 + semi*2 + 4；+Option<SemiTransparent> + Option<AlbedoMap> + Res<AssetTextureLibrary>", "")
sysx("nani_update_velocity_only", "速度（kind 0x19，0x60）：当前 + 上一帧（PrevTransform）两个仿射；position/sort 取当前 Transform", "")
sysx("nani_update_stairs_floor", "楼梯地板（kind 0x1a，0x30）：9 字拷自 StairsFloorMeshChunk+8 + 3 零字；position=0/sort=1.0", "")
sysx("nani_flush_instance_updates", "冲刷（0x142080640，1053B）：27 槽（stride 0xc0），仅 Owned 调 ShadowedBuffer<GpuInstance>::gpu_flush(device) + 数据缓冲 vtable+0x30；末尾 +0x1448=+0x1440; +0x1440=0；参数顺序（Res<GraphicsDeviceResource> 位置）为 I", "；§5")

# ---------------------------------------------------------------- 内联/无符号列表
INLINE = {}  # (file, name) -> 注释

def inl(f, name, txt): INLINE[(f, name)] = txt

# bind_resources
inl(f"{PB}/bind_resources.rs.md", "country_core::render::pass_builder::bind_resources::impl$0::bind_shadowmap",
 "占位 shadowmap 绑定（inline 12 宿主；无独立过程；与 bind_dummy_shadowmap 相关）—— I")
# compute_pass_builder 内联
for nm, txt in {
 "ComputeDispatchApi::dispatch_1d": "1D 组数（线程总数模式 1）—— I（内联，转 dispatch_impl）",
 "ComputeDispatchApi::dispatch_2d": "2D 组数 —— I（内联）",
 "ComputeDispatchApi::push_constants": "push constants 提交（转 ComputeExec 接缝）—— I",
 "ComputeDispatchApi::track_gpu_lifetime": "GPU 生命周期跟踪 —— I",
 "ComputeDispatchIndirectApi::dispatch": "间接分派（经 ComputeExec）—— I",
 "ComputeDispatchIndirectApi::push_constants": "间接路径 push constants —— I",
 "ComputePass::dispatch": "dispatch 薄封装（模式 0）—— I",
 "ComputePass::dispatch_3d": "dispatch_3d 薄封装（模式 0）—— I",
 "ComputePass::dispatch_impl": "共享 dispatch 实现（inline 132 宿主）：(mode,args,closure) 包装进 closure_env_0（400B）→ PassBuilder::render —— I（O 本体在 0x14169af70 渲染回调）",
 "ComputePass::new_minimal": "最小构造（无着色器/无颜色库）—— I",
 "ComputePass::push_constants": "push constants（行 415）—— I",
 "ComputePass::read_draw_indirect_cmd_buffer": "读间接命令缓冲（行 407）—— I",
 "ComputePass::read_write_texture": "读写纹理绑定（行 161；内联进 44/102/238/500 宿主）—— I",
 "ComputePass::write_draw_indirect_cmd_buffer": "写间接命令缓冲（行 394）—— I",
 "ComputePass::write_texture": "写纹理绑定（行 171）—— I",
 "impl$3::read_storage_buffer": "read_storage_buffer<T> 泛型声明（内联，宿主为 25 个实例）—— O（实例见上）",
 "impl$3::read_texture": "读纹理（行 704）—— I",
 "impl$3::read_texture_view": "读纹理视图（行 718；stencil 视图 aspect_mask(4)）—— I",
 "impl$3::uniform": "uniform<C,T> 必选写（inline 155 宿主；无 ==8 守卫）—— O（与 optional_uniform 同逻辑）",
}.items(): inl(f"{PB}/compute_pass_builder.rs.md", f"country_core::render::pass_builder::compute_pass_builder::{nm}", txt)
# draw_builder 内联
for nm, txt in {
 "DrawBuilder::stencil_ref": "模板参考值（inline 11）—— I",
 "DrawBuilder::push_constants": "push constants（inline 1）—— I",
 "DrawBuilder::bind_dummy_shadowmap": "绑定占位 shadowmap（inline 1；对应 bind_shadowmap）—— I",
 "DrawBuilder::draw_instanced": "draw_instanced（inline 13）—— I（经 draw_instanced_w_gl_draw_flag 的薄封装）",
 "DrawBuilder::draw_indirect": "draw_indirect（inline 1）—— I",
 "DrawBuilder::draw_indexed_indirect": "draw_indexed_indirect（inline 11）—— I（经 draw_indexed_indirect_raw 薄封装）",
 "DrawBuilder::get_indirect_args_buffer": "取间接参数缓冲句柄（inline 2）—— I",
 "impl$2::uniform": "uniform<C,T> 必选写（inline 155 宿主）—— O（同 optional_uniform）",
 "impl$2::read_texture": "读纹理绑定（行 524）—— I",
 "impl$2::read_texture_view": "读纹理视图（行 538）—— I",
 "impl$2::read_storage_buffer": "read_storage_buffer<T> 泛型（行 553；43 个实例）—— O（实例见上）",
 "NoDropOnlyTake::take": "取走型辅助：防止误 drop（首 u64 niche，2=已取走）—— I",
 "NoDropOnlyTake::try_take": "try 取走 —— I",
}.items(): inl(f"{PB}/draw_builder.rs.md", f"country_core::render::pass_builder::draw_builder::{nm}", txt)
# draw_stream 内联
for nm, txt in {
 "DrawPacket::variant": "variant 读取（行 122）—— I",
 "DrawPacket::stencil_ref": "stencil_ref 读取（行 127）—— I",
 "impl$2::to_texture_binding_key": "Handle<Texture> → 0x4000_0000_0000_0000|id（行 161）—— I（取自 bind 函数体）",
 "impl$3::to_texture_binding_key": "ReadOnlyHandle → 同上（行 167）—— I",
 "impl$4::to_texture_binding_key": "TexturePreset → 同上（行 174）—— I",
 "RasterPass::get_render_pass": "取/建 HashMap<RenderPassDesc,RenderPass> 缓存中的本 pass RenderPass（inline 1）—— I",
 "RasterPass::new": "new → new_impl（inline 42）—— O（见 new_impl）",
}.items(): inl(f"{PB}/draw_stream.rs.md", f"country_core::render::pass_builder::draw_stream::{nm}", txt)
# framebuffer_binding 内联
for nm, txt in {
 "DepthAttachment::layer": "层字段读取（inline 1）—— I（+4 = 层，O）",
 "ColorAttachment::handle_mut": "纹理句柄可变访问（inline 4）—— I",
 "RasterFramebufferBinding::new_color_depth": "旧 API：机器码无对应物（B_a4_draw 已删 5 槽/color_count 字段）—— 已删除",
 "RasterFramebufferBinding::new_color": "旧 API：同上 —— 已删除",
 "RasterFramebufferBinding::new_depth": "旧 API：同上 —— 已删除",
 "impl$4::from": "附件转换（行 171）—— I",
 "impl$5::from": "附件转换（行 179）—— I",
 "impl$6::from": "附件转换（行 185）—— I",
}.items(): inl(f"{PB}/framebuffer_binding.rs.md", f"country_core::render::pass_builder::framebuffer_binding::{nm}", txt)
# draw_list
inl(f"{PB}/draw_list.rs.md", "country_core::render::pass_builder::draw_list::DrawListItem::to_draw_packet",
 "内联 22 宿主：DrawListItem（0x70B，首字段 EntityDrawParams 0x50B）+ 回调 fn@+0x70（元组 0x80B，driftsort 稳定排序）；元素布局 O（0x1409bc8f0..0x1409bc97c 逐字段），to_draw_packet 具体形态 I—— O/I")
# pass_resource_access 内联（7 项，都在章节内联列表里？章节只列了 as_texture 等；检查）
for nm, txt in {
 "impl$1::as_texture": "TextureFromDisk / 只读句柄族 as_texture（行 54；外部路径，视图必须 None）—— I（O 记录布局见 B_i_api §3.2）",
 "impl$1::dims": "同上族 dims（行 58）—— I",
 "impl$4::as_texture": "行 98 族 as_texture（&RgHandle/&ReadOnlyHandle 目标改为 RG）—— I（B_i_api §3.2）",
 "impl$4::dims": "行 105 族 dims —— I",
 "impl$5::as_texture": "行 113 族 as_texture（另一只读/受管族）—— I",
 "impl$5::dims": "行 120 族 dims —— I",
 "ssbo_to_rg_handle": "内联辅助（56 宿主）：&T → rhapsody Handle；真实签名 U—— U",
}.items(): inl(f"{PB}/pass_resource_access.rs.md", f"country_core::render::pass_builder::pass_resource_access::{nm}", txt)
# shader_uri 内联
for nm, txt in {
 "impl$1::source_hash": "source_hash<&str> 泛型（行 48）—— O（见实例）",
 "impl$2::source_hash": "通用 T 实现（行 61，inline 1）—— I",
 "impl$2::as_compute_pipeline": "通用 T 实现（行 65）—— I",
 "impl$3::as_shader_def_value": "AsShaderDefValue：编译期特性值（行 75）—— I",
 "impl$4::as_shader_def_value": "AsShaderDefValue（行 81）—— I",
 "ComputeShaderSourceHash::compute": "计算源码 hash 的辅助（inline 22；calculate_hash 转发）—— I",
}.items(): inl(f"{PB}/shader_uri.rs.md", f"country_core::render::pass_builder::shader_uri::{nm}", txt)
for nm in ["impl$10","impl$14","impl$18","impl$22","impl$26"]:
    inl(f"{PB}/shader_uri.rs.md", f"country_core::render::nani::cull_shaders::{nm}::source_hash", "ICF/内联（as_compute_pipeline 同址取 hash）—— O")
for nm, src in [("impl$2","bin_lights"),("impl$8","rasterize_terrain_stroke"),("impl$1","taa"),("impl$7","vermeer"),("impl$15","dof"),("impl$19","dof")]:
    inl(f"{PB}/shader_uri.rs.md", f"country_core::systems::render_loop::{'compute_passes::' if src!='vermeer' else ''}{src}::{nm}::source_hash", "同构 source_hash —— O")
# uniform_binder 内联
for nm, txt in {
 "UniformBinder::write_uniform": "write_uniform 泛型（行 37，10 个实例）—— O（见实例）",
 "UniformBinder::write_optional_uniform": "write_optional_uniform 泛型（行 53，4 个实例）—— O（见实例）",
}.items(): inl(f"{PB}/uniform_binder.rs.md", f"country_core::render::pass_builder::uniform_binder::{nm}", txt)
# nani buffers 内联
for nm, txt in {
 "GpuInstance::new": "GpuInstance（紧凑实例记录）构造 —— I（字段 U：B_n_world U 清单）",
 "GpuInstance::set_mesh_atlas_part": "写 mesh atlas part 字段 —— I（字段 U）",
 "NaniDrawListParams::list_draw_merge_window_range": "draw-list 合并窗口范围（内联；语义由名+draw 列表接线推断）—— I",
 "NaniDrawListParams::list_draw_merge_window_subtotal_range": "合并窗口小计范围 —— I",
}.items(): inl("nani/buffers.rs.md", f"country_core::render::nani::buffers::{nm}", txt)
# gpu_world 内联（31 项）
for nm, txt in {
 "impl$2::from": "从 u8 → GpuWorldSubsetKind 转换—— I",
 "GpuWorldSubsetKind::shader": "kind→shader 名—— **无机器码依据**（B_n_world 已删 shader/try_from_shader）—— U",
 "GpuWorldSubsetKind::max_instances": "max_instances[kind]（三张常量表 0x142aba790/0x142c77010/0x142aa14c0 逐值相同）—— O（数值在 buffers.rs）",
 "GpuWorldSubsetKind::use_fine_grained_sorting": "FORCE_FINE(0x143634d9c)||(kind<2 && !FORCE_NO_FINE(0x143634d9d)) 判定 —— O（语义 I）",
 "GpuWorldSubsetKind::max_draws_per_list": "MAX_DRAWS_PER_LIST[kind]（0x142aa14c0）—— O（27 值在 buffers.rs）",
 "GpuWorldSubsetKind::draw_list_index": "kind → draw-list 索引（= kind*4，变体位由 nani_draw 加）—— I",
 "GpuWorldSubsetKind::try_from_shader": "shader 名→kind —— **无机器码依据**（已删）—— U",
 "GpuWorldSubsetKind::gpu_world_subset_desc": "subset 描述（owned/external/owned_custom 已内联）—— U（字段未恢复）",
 "impl$4::gpu_world_subset_desc::external": "External 描述 —— U",
 "impl$4::gpu_world_subset_desc::owned": "Owned 描述（容量 = max_instances[kind]，I）—— I/U",
 "impl$4::gpu_world_subset_desc::owned_custom": "自定义 Owned 描述 —— U",
 "impl$36::register_required_components": "InstanceDataMarker<T> 组件注册（行 338 泛型）—— O（19 个实例见上）",
 "GpuWorld::get_subset": "取 subset 读锁（内联；PDB 仅 LF_FUNC_ID）—— I（形态）",
 "GpuWorld::owned_subset_exclusive_mut": "免锁 &mut OwnedGpuWorldSubset 访问器（非 Owned → panic \"Subset {:?} is not owned\"）—— I",
 "GpuWorld::create_instances_no_data": "create_instances 的无数据薄包装（行 468）—— I",
 "GpuWorld::assign_subset": "把 GpuWorldSubset 写入槽（startup_nani 接线点）—— I",
 "GpuWorld::update_mesh_atlas_slices": "mesh atlas 切片更新（行 501）—— I（语义由名+mesh_atlas 接线推断）",
 "GpuWorld::flush_instance_updates": "= nani_flush_instance_updates 的 GpuWorld 侧：roll_instance_counts + flush_instance_buffers（O 事实见 0x142080640）—— O/I",
 "GpuWorld::clear": "清空（行 539）—— I",
 "GpuWorld::mesh_atlas_info_buf": "NaniMeshAtlasPartInfo 缓冲访问器（+0x1458）—— O（布局）",
 "GpuWorld::draw_list_info_buf": "NaniDrawListInfo 缓冲访问器（+0x1480）—— O（布局）",
 "GpuWorld::try_get_subset_mut": "try_write 读锁变体 —— I",
 "GpuWorld::list_index_to_cmd_offset": "list_index→cmd 偏移（= params.max_instance_offsets[list]，nani_draw 0x140904c98 内联读第二个 Vec）—— O（B_i_nani §2.2 纠错）",
 "GpuWorldSubset::atlas_mesh_count": "atlas 网格数（行 606）—— I",
 "OwnedGpuWorldSubset::new": "以 max_instances[kind] 个默认元素创建（I：构造器内联 21 处）—— I（容量 O）",
 "OwnedGpuWorldSubset::extend": "批量扩展（行 634；extend_no_data 的薄包装）—— I",
 "OwnedGpuWorldSubset::instance_count": "实例数读取（行 727）—— I",
 "OwnedGpuWorldSubset::gpu_flush": "缓冲写后标记 GPU 侧脏（inline 1）—— I",
 "OwnedGpuWorldSubset::clear": "重置计数与索引（行 768）—— I",
 "OwnedGpuWorldSubset::remove_instance": "swap-remove（内联转发 IndexCondenser::remove + 数据缓冲 vtable+0x50）—— O（转发点）",
 "NaniInstanceData::new": "类型擦除数据缓冲构造（行 817）—— I",
}.items(): inl(GW, f"country_core::render::nani::gpu_world::{nm}", txt)
# instance_data 内联
for nm, txt in {
 "WindowGlassFlags::color_id_or_err": "modular_bitfield 派生（color_id 位域读取；bit31 越界 unwrap 文案 O）—— I",
 "WindowGlassFlags::set_color_id": "位域写 —— I",
 "WindowGlassFlags::set_color_id_checked": "位域检查写 —— I",
 "WindowGlassFlags::with_color_id": "位域 builder —— I",
 "WindowGlassFlags::is_semitransparent_or_err": "semi 位读取 —— I",
 "WindowGlassFlags::set_is_semitransparent": "位域写 —— I",
 "WindowGlassFlags::set_is_semitransparent_checked": "检查写 —— I",
 "WindowGlassFlags::with_is_semitransparent": "builder —— I",
 "NaniUberInstanceFlags::has_albedo_texture_or_err": "albedo 位读取 —— I",
 "NaniUberInstanceFlags::is_semitransparent_or_err": "semi 位读取 —— I",
 "NaniUberInstanceFlags::with_is_semitransparent": "builder —— I",
 "NaniUberInstanceFlags::receive_contact_shadows_or_err": "contact shadow 位读取 —— I",
 "NaniUberInstanceFlags::pad_or_err": "pad 位读取 —— I",
 "CurvedHalfTimberCaps::to_u32": "端帽位域打包（行 322）—— I",
 "CurvedHalfTimberInstanceData::curved_halftimber_mesh_atlas_part_idx": "PDB 仅 LF_FUNC_ID（内联残留）；**无机器码依据，reconstruction 已删除**—— U",
}.items(): inl("nani/instance_data.rs.md", f"country_core::render::nani::instance_data::{nm}", txt)
# mesh_atlas 内联
inl("nani/mesh_atlas.rs.md", "country_core::render::nani::mesh_atlas::MeshAtlas::clear", "清空图集（行 141）—— I")
# trimesh 内联（无；glue 3 项：builder::closure$0 ×2 + attrib::closure$0，派生胶水）

# ---------------------------------------------------------------- 逐函数（其它 exact）
# compute_pass_builder 非泛型业务
for nm, txt in {
 "ComputePass::bind_external_raw": ("复制调用方 8×u64 原始描述符到 0x48B 外部记录（命中覆写绑定号；未命中丢弃）", "O", "B_a4_compute §2（0x14096e540）"),
 "ComputePass::bind_external_storage_buffer": ("assert set → GraphicsDevice::get_vk_buffer（None → unwrap）→ 0x48B 记录 {binding, tag 5, vk, 0, !0, 句柄克隆, 0}；不登记 RG 访问", "O", "B_a4_compute §2（0x14096e2a0）"),
 "ComputePass::read_rg_storage_buffer": ("注册表查找（键=句柄 hash，值={set,resource}）→ assert_eq!(set,0) → add_resource_access(resource, 0x1a, 1) → 绑定数≥0x100 unwrap → 0x44B RenderPassBinding 追加 → access_range；未命中整体跳过", "O", "B_a4_compute §2/§3.3（0x14096dac0）"),
 "ComputePass::write_rg_storage_buffer": ("同 read，访问 0x37、组 0", "O", "B_a4_compute §2（0x14096dd60）"),
 "ComputePass::write_rg_storage_buffer_concurrent": ("同 write，同步组=调用方", "O", "B_a4_compute §2（0x14096e000）"),
 "ComputePass::dispatch_indirect_raw": ("add_resource_access(buf, 0x02, 1) → 模式 2 render", "O", "B_a4_compute §2/§3.5（0x14096e880）"),
 "ComputePass::dispatch_1d": ("置 1D 组数（线程总数）→ 模式 1 render", "O", "B_a4_compute §2（0x14096eaf0）"),
 "ComputePass::dispatch_2d": ("置 2D 组数 → 模式 1（缺省维=1）", "O", "B_a4_compute §2（0x14096ec80）"),
 "ComputePass::dispatch_groups": ("置线程组数 → 模式 0", "O", "B_a4_compute §2（0x14096ee10）"),
 "ComputeDispatchApi::dispatch_3d": ("立即模式：local size 三项为 0 → div_by_zero（**先于屏障**）；屏障；ceil 除法；锁内计数+1；经 device+0x360 vkCmdDispatch；device+0xaba==1 先 full barrier", "O", "B_a4_compute §2/§3.5（0x14096efa0）"),
}.items():
    exact(f"{PB}/compute_pass_builder.rs.md", f"country_core::render::pass_builder::compute_pass_builder::{nm}", *txt)

# draw_builder 非泛型业务
for nm, txt in {
 "DrawBuilder::new": ("写 variant@+0xb0；取 shader 反射（缺失 unwrap）；**仅当反射有 uniform 块**才 DynamicConstants::allocate + insert uniform 绑定，否则 None@+0xde==8；写 +0xe0 pass、+0xe8 Arc、+0xf0=0", "O", "B_a4_draw §2（0x140905be0）"),
 "DrawBuilder::read_rg_storage_buffer": ("名字表空/未命中 → 原样返回；set!=0 → assert_failed<SetIndex>；add_resource_access(0x24, 组 1)；绑定≤0x100", "O", "B_a4_draw §2（0x140906150）"),
 "DrawBuilder::bind_external_storage_buffer": ("{vk_buffer,0,u64::MAX}+句柄克隆 insert；get_vk_buffer None → unwrap", "O", "B_a4_draw §2（0x140906400）"),
 "DrawBuilder::write_rg_storage_buffer": ("同 read，访问 0x37、组 0（两函数体其余相同）", "O", "B_a4_draw §1/§2（0x1409067e0）"),
 "DrawBuilder::draw_no_vao": ("draw 种类 0：直接快照 push，不写间接参数", "O", "B_a4_draw §2（0x140906a90）"),
 "DrawBuilder::draw_no_vao_indexed": ("种类 1", "O", "B_a4_draw §2（0x140906cb0）"),
 "DrawBuilder::draw": ("不改种类；快照 push（具体参数由闭包录制期解释）", "O", "B_a4_draw §2（0x140906f40）"),
 "DrawBuilder::draw_multi_instanced": ("种类 4：+0x44 起点/+0x4c 数（Vec<[u32;2]>）", "O", "B_a4_draw §2（0x140907050）"),
 "DrawBuilder::draw_instanced_from_placement_data": ("placement 表查找（缺失 expect \"This Handle for PlacementDataSSBO doesn't have a corresponding buffer\"）→ bind_external_storage_buffer(0x30e6778252531a83) → draw_instanced_w_gl_draw_flag", "O", "B_a4_draw §2（0x140907280）"),
 "DrawBuilder::draw_instanced_w_gl_draw_flag": ("种类 3：+0x44 = gl_draw_flag（DrawPacket::new 缺省 flag=1）", "O", "B_a4_draw §2（0x140907490）"),
 "DrawBuilder::multi_draw_indirect": ("种类 5：AssetSsboLibrary::get_rg_buffer<DrawIndirectBuffer>；add_resource_access(2,组1)；+0x44/+0x50/+0x58", "O", "B_a4_draw §2（0x1409076b0）"),
 "DrawBuilder::multi_draw_indexed_indirect": ("种类 7：get_rg_buffer<DrawIndexedIndirectBuffer>；静态段表 DAT_142aaaa78+idx*0x14 读 4B（INDEXED_INDIRECT_INDEX_COUNTS 24 项，B_i_api §2 逐项复核）；+0x48=idx*0x14、+0x60=1", "O", "B_a4_draw §2；B_i_api §2（0x140907990）"),
 "DrawBuilder::draw_indexed_indirect_raw": ("种类 7：从间接缓冲记录 +0x10 读 resource_index；add_resource_access(2,组1)", "O", "B_a4_draw §2（0x140907cc0）"),
 "DrawBuilder::draw_indexed_indirect_count": ("种类 8：两个间接缓冲各 add_resource_access；+0x44/+0x48/+0x50/+0x58/+0x60/+0x68", "O", "B_a4_draw §2（0x140907f90）"),
 "DrawBuilder::discard": ("丢弃 builder（所有 GpuOwned 状态置 2；不 push 记录）", "O", "B_a4_draw §2（0x1409082c0）"),
 "DrawBuilder::draw_no_index_buffer": ("种类 2", "O", "B_a4_draw §2（0x1409083f0）"),
}.items():
    exact(f"{PB}/draw_builder.rs.md", f"country_core::render::pass_builder::draw_builder::{nm}", *txt)

# gpu_world 非泛型业务
for nm, txt in {
 "GpuWorld::new": ("nani_calculate_draw_list_params() → NaniDrawListInfo 缓冲（初始数据=克隆的 max_instance_offsets）→ 27×RwLock<Option> 槽 → 两个 TypedStorageBuffer（+0x1458/+0x1480）→ 计数清零；带 device 参数", "O", "B_n_world §1/§3；B_i_nani §2；wp3/contracts/gpu_world.md §1（0x1409917d0）"),
 "GpuWorld::owned_subset_mut": ("按 kind 取槽写锁 → MappedRwLockWriteGuard<OwnedGpuWorldSubset>；External → panic \"Subset {:?} is not owned\"，空槽 unwrap", "O", "B_n_world §1/§5；B_i_nani §1（0x140991c50）"),
 "GpuWorld::create_instances<country_core::resources::placement_data::PlacementDataSsbo,country_core::systems::prefab::create_prefab_gpu_instances::closure_env$0>": ("单态化（PlacementDataSsbo）：count=min(cull.len,data.len)；容量满 → warn-once 跳过；i=next_index++；HashMap insert + Vec push；CAS min/max；类型化数据缓冲 type-id 断言（panic 文案 \"The instance data for {kind:?} is not {T} but {actual}\"）；末尾 total_instance_count += cull.len()", "O", "B_n_world §1/§2.4/§2.5/§2.6（0x14156d0d0）"),
 "GpuWorld::update_instance_mesh": ("&mut self 免锁 → owned_subset_exclusive_mut → 写紧凑 key +0xe（u16 mesh_index）", "O", "B_n_world §1（0x140991d60）"),
 "GpuWorld::remove_instance": ("owned_subset_mut(key.kind)（key+8 kind 字节）→ IndexCondenser::remove → 命中 swap-remove（positions 与数据同步、CAS 收缩 min/max）；未命中 log trace \"Failed to remove GPU instance {key:?}\"", "O", "B_n_world §1/§2（0x140991f60）"),
 "GpuWorld::import_to_rg": ("遍历 27 槽（+0xa0..+0x14e0 step 0xc0），仅 Owned 调数据缓冲 import_to_rg（vtable +0x40）", "O", "B_n_world §1/§2.6/§5（0x1409921c0）"),
 "GpuWorld::export_from_rg": ("对称（vtable +0x48）", "O", "B_n_world §1/§5（0x140992220）"),
 "OwnedGpuWorldSubset::extend_no_data": ("同 create_instances 的 a–h 路径但数据由调用方另写；回调每实例一次（收 12B {u32,u32,kind u8}）；无 hash 表（External）跳过查找；容量 warn-once 跳过非 panic", "O", "B_n_world §1/§2（0x140992280）"),
 "impl$14::rg_handle": ("返回 RG 资源句柄（43B；vtable +0x38）", "O", "B_n_world §1（0x141572410）"),
 "impl$14::import_to_rg": ("导入 RG（82B；vtable +0x40）", "O", "B_n_world §1（0x141572440）"),
 "impl$14::export_from_rg": ("导出 RG（73B；vtable +0x48）", "O", "B_n_world §1（0x1415724a0）"),
 "_::impl$0::deserialize::impl$0::visit_str<enum2$<ron::error::Error> >": ("GpuWorldSubsetKind 的 serde Deserialize 派生（RON 字符串→变体，27 路匹配；非法 → Error::Message）；I（体量一致，未逐字节比对）", "O", "B_n_world §1/§2.1（0x141572870）"),
}.items():
    exact(GW, f"country_core::render::nani::gpu_world::{nm}", *txt)

# instance_data 业务
for nm, txt in {
 "UberInstanceData::marker": ("pub const fn marker()（1B ret，与其它 ZST 函数 ICF 合并）返回 InstanceDataMarker<Uber…>", "O", "B_n_world §1（0x14003bb80）"),
 "impl$122::clone_behavior": ("Component Clone 派生薄封装（13B）", "O", "B_n_world §1（0x140990af0）"),
 "impl$122::register_required_components": ("InstanceDataMarker<T> 组件注册（bevy 派生；行 284）", "O", "B_n_world §1（0x140990a80）"),
 "CurvedHalfTimberInstanceData::new_curved_xy": ("0x40B 逐偏移写入：+0x10=start-half_len*0.5、+0x14=mid+half_len*0.5、+0x18=start、+0x1c=end、+0x28=细分计数值、+0x30=next_power_of_two(ceil((|y1-y0|*x1+1)/0.6))（clz 循环）；+0x30 细分等级 ilog2(next_pow2(…)).clamp(2,5)", "O", "B_n_world §1/§2.10（0x14098f690）"),
 "CurvedHalfTimberInstanceData::new_curved_xz": ("xz 平面对称构造（363B，同 0x40B 布局）", "O", "B_n_world §1（0x14098f800）"),
 "CurvedHalfTimberInstanceData::new_straight": ("+0x20 = caps_count*2 + (has_cap?2:1)；+0x24 = 0x3dc28f5c00000001；+0x2c = 0x100000000（打包字段语义 U）", "O", "B_n_world §1/§2.10（0x14098f970）"),
}.items():
    exact("nani/instance_data.rs.md", f"country_core::render::nani::instance_data::{nm}", *txt)

# mesh_atlas 业务
for nm, txt in {
 "MeshAtlasPart::new": ("空构造（3B 薄构造，O）", "O", "wp3/contracts/mesh_atlas.md §1（0x140833250）"),
 "MeshAtlas::add": ("8 步（O）：indices 空→panic \"invalid mesh\"；Vertex_Position 包围半径 sqrt(max(x²+y²+z²))；parts.len()<0x40 断言；base_vertex=现有元素数；首个 mesh 直接 clone 否则索引+base 后比较排序属性名集（不同 → Err anyhow \"Mesh attribute mismatch…\"，不回滚）→ 追加 MeshAtlasPart{index_offset,index_count,radius,0}；重复名 → panic \"Duplicate atlas mesh name\"", "O", "B_n_world；wp3/contracts/mesh_atlas.md §2（0x140a36200）"),
 "impl$10::clone_behavior": ("Component 派生薄封装（3B；与 is_exclusive<FunctionSystem> 同址 ICF，0x140002370）", "O", "wp3/contracts/mesh_atlas.md §3；render-loop DIGEST §5"),
}.items():
    exact("nani/mesh_atlas.rs.md", f"country_core::render::nani::mesh_atlas::{nm}", *txt)

# trimesh 业务
for nm, txt in {
 "TrimeshChunk::index_count": ("movzx eax, word [rcx+8]：u16 索引数（5B）", "O", "B_n_world §1（0x14129dbc0）"),
 "Trimesh::builder": ("把索引缓冲切成 ≤64 三角形 meshlet（块内顶点去重成局部 u8 下标；Vec::with_capacity(0x40/0xc0)）；旧骨架完全缺失的主体", "O", "B_n_world §1/§2.9（0x14129dbd0）"),
 "TrimeshBuilder::positions": ("置 positions 属性下标（= 当前 attribs.len()）→ attrib<Vec3>(\"positions\", …)（98B）", "O", "B_n_world §1/§2.9（0x14129e890）"),
 "TrimeshBuilder::attrib<glam::f32::vec3::Vec3>": ("0x18B 记录 {data ptr+byte_len, elem_size u16, align u8}（**无属性名**；+0x12=对齐=4；按对齐降序强制）；data 空/不足 expected → panic；vertex_count 首个定值后续必须相等 assert_failed<usize,usize>", "O", "B_n_world §1/§2.9（0x140905080/0x1412a90c0 两份 ICF）"),
 "TrimeshBuilder::build": ("打包每块\"三角形+逐顶点属性\"进 GPU 缓冲（usage 0x20020），算包围球，device_address 回写；TrimeshChunk{address,index_count:u16,…} 0x20B", "O", "B_n_world §1/§2.9（0x14129e900）"),
}.items():
    exact("nani/trimesh.rs.md", f"country_core::render::nani::trimesh::{nm}", *txt)

# draw_ext
exact("nani/draw_ext.rs.md", "country_core::render::nani::draw_ext::impl$0::nani_draw",
 "9 步（O）：取 GpuWorld（type-hash 0x662978458699a5ef；缺→unwrap）→ subset 读锁（槽 None→unwrap）→ draw_count（Owned/External 变体 u64）→ list_index=kind*4|PASS_VARIANT_TABLE[pass_variant(+0xf0)]（0x302010000→[0,0,1,2,3]）→ list_index_to_cmd_offset（越界 panic_bounds_check）→ get_rg_buffer<NaniDrawListLength/NaniDrawList> + read_storage_buffer<NaniInstanceIndirectionSsbo> +（句柄有则）read_rg_storage_buffer(0x2106412d631790c6) → 偏移 cmd_slot*20 → count_path=FORCE_FINE(0x143634d9c)||(kind<=1&&!FORCE_NO_FINE(0x143634d9d)) 选 draw_indexed_indirect_count（max=MAX_DRAWS_PER_LIST[kind]，表 0x142aab288）或 _raw → 释放读锁",
 "O", "B_n_sys draw_ext §1（0x140904ae0）；B_i_nani §2；wp3/contracts/draw_ext.md")

# ---------------------------------------------------------------- glue 说明
GLUE_NOTE = {
 f"{PB}/bind_resources.rs.md": "无 glue（内联 1：bind_shadowmap）。",
 f"{PB}/compute_pass_builder.rs.md": "6 项 glue = dispatch_impl::closure_env$0 的 4 个嵌套 closure + bind_texture_to_compute_pass::closure$3（维度 uniform 闭包，均属业务函数的惰性求值回调，不单独计业务）。",
 f"{PB}/draw_builder.rs.md": "7 项 glue = impl$0::drop（**业务性 drop**，O：未 draw/discard 且非 panic → panic \"The DrawBuilder for {name} was not drawn or discarded.\"，0x14089a140）+ bind_external_storage_buffer::closure$0 + read_storage_buffer::closure$0 + bind_texture_to_draw_builder::closure$0/4/5/7（维度/惰性闭包）。",
 f"{PB}/draw_list.rs.md": "无 glue。",
 f"{PB}/draw_stream.rs.md": "14 项 glue：impl$9::drop（0x140896d80 = **DrawPassRenderCheck::drop**，O：!rendered && !panicking → panic \"RasterPass dropped without a call to render()\"）+ new_impl/render/get_render_pass 的 9 个 closure + impl$14/17/22 的 default/eq 派生。",
 f"{PB}/framebuffer_binding.rs.md": "无 glue。",
 f"{PB}/pass_resource_access.rs.md": "无 glue。",
 f"{PB}/pass_resources.rs.md": "无 glue。",
 f"{PB}/shader_uri.rs.md": "6 项 glue = impl$18::fmt（0x140939420 Debug，O）+ impl$5::hash/impl$7::eq（ShaderUri 作 HashMap 键）+ taa/vermeer/rasterize_terrain_stroke 的 impl$2/8/9::hash（ShaderDefValue 派生）。",
 f"{PB}/uniform_binder.rs.md": "1 项 glue = write_optional_uniform::closure$0（惰性值闭包）。",
 "nani/buffers.rs.md": "无 glue（内联 4：GpuInstance×2 + merge-window×2）。",
 "nani/draw_ext.rs.md": "无 glue。",
 "nani/gpu_world.rs.md": "19 项 glue：impl$17::fmt（0x140992a60 = **GpuWorldSubsetKind 的 Debug**，O：27 个变体名自 0x142aba868 长度表+0x142aba940 跳表逐个读出）+ impl$44::fmt（0x140a241d0，**U**：PDB 归属 gpu_world 但机器码是 debug_tuple_field1_finish(\"InstanceKey\")，与 12B InstanceKey 对不上，疑为 IndexCondenser::Condex 的 Debug 或 ICF 别名）+ 派生 eq/hash/next/deserialize/closure（gpu_world_subset_desc 的 3 个 closure 等）。",
 "nani/instance_data.rs.md": "1 项 glue = impl$118::clone（Component Clone 派生）。",
 "nani/mesh_atlas.rs.md": "6 项 glue = default×2 + fmt + add 的 3 个 closure（半径/属性比较闭包）。",
 "nani/systems.rs.md": "7 项 glue = impl$0::default（NaniActiveInstanceCollection Local 缺省）+ maintain 的 closure$0/$1（0x141565950 = extend_no_data 回调，把 12B 记录存入 Option，O）+ curved_halftimber/stone_floor 的 4 个打包闭包。",
 "nani/trimesh.rs.md": "3 项 glue = builder::closure$0 ×2 + attrib::closure$0（meshlet 去重/属性比较闭包）。",
}

# ---------------------------------------------------------------- 每章证据注记
NOTES = {
 f"{PB}/bind_resources.rs.md": """## 证据注记（tg_rn1）
- **哈希常量**（O 立即数）：COLOR_LIBRARY_BINDING_HASH 0x52ec6a17beb8fef0；RT_TRIANGLES 0x168ddb2c3b5f38a9 / RT_TRIANGLE_META 0x47ddebfb264383d0 / RT_BVH 0x339e76846042a271 / WALL_COLOR_ID 0xe9701a257aa8828c；uniform 键 0x1c9f2eef0ac739ac（Tri 缓冲字）/ 0xc2b7f2fdf5e21521（Bvh 缓冲字，取值来源 I）。
- **槽名纠错**（B_i_api §1.2）：墙色槽名是复数 **"WallColorIds"**（0x14162c290 的 rustc_entry 键 0xc，xxh3=0xaf42315d263aa22a），Rt 三个槽名 = "RtTriangles"/"RtTriangleMeta"/"RtBvh"（xxh3 复算吻合）。
- **两族差异**：ComputePass 版无槽表、不写 CPU 缓冲；DrawBuilder 版有 `HashMap<&str, PassResourceSlot>` 槽表、回读墙色到 CPU 缓冲。
- U：`bind_ray_tracing` 两个 u32 uniform 的取值来源（I：`Buffer+0` 低 32 位）；接入 `resources::rt::*` 真实类型前的载体（含 Vec 不满足 StorageElement: Copy）。""",
 f"{PB}/compute_pass_builder.rs.md": """## 证据注记（tg_rn1）
- **ComputePass 400B 布局**（O）：+0x00 bindings Vec<RenderPassBinding(0x44)>、+0x18 access_ranges Vec<8B>、+0x30 external_bindings Vec<0x48B>、+0x60 PassBuilder、+0x110/+0x120/+0x136 UniformBinder（+0x136==8 = 无 uniform 块）、+0x138 &着色器反射（hashbrown：键=绑定名哈希 u64，值=(SetIndex,BindingIndex)）、+0x140 &GraphicsDevice、+0x148 PassResources(56B)。方法按值消费 self（memcpy 0x190 回传）。
- **访问码**（O）：compute 读 SSBO 0x1a；写 0x37；`write_*_storage_buffer`/cpu-mapped 0x32；间接参数读 0x02；传输写 0x34；`is_write = (a-0x28)<0x10`（0x140c10e10）。同步组：读 1 / 写 0 / 并发写=调用方。
- **注册表语义**（O）：每次命中 `assert_eq!(set, 0)`；AccessRange.resource_index 存的是 **BindingIndex** 不是资源号；`*_access_buffer<T>` 完全不经注册表。
- **dispatch 模式**（O）：0 组数 / 1 线程总数 / 2 间接 / 3 闭包 / 4 间接+闭包；渲染回调体 0x14169af70（组模式 device+0xaba==1 → 屏障+计数+1+`vkCmdDispatch`；间接模式 "Resource does not exist" + `vkCmdDispatchIndirect`）。模式→Rust 枚举名 U（DrawMode 保留 u32）。
- **panic/断言**：绑定数 ≥0x100 `unwrap_failed`（在 add_resource_access 之后）；local size 0 `div_by_zero`（**先于屏障**）；注册表未命中静默跳过。
- U：`bind_texture` 受管/`TextureFromDisk` 外部绑定路径（标签 1/2 的记录布局 O 于 B_i_api §3.2，本文件只实现了 RG 路径）；`ComputePass+0x140` 确切类型（I：设备）；push constants 原始承载字段（I：自行保存字节）；`write_storage_buffer_concurrent` 原版实例 = 9 个（NaniDrawListSsbo 0x141608450 等）。""",
 f"{PB}/draw_builder.rs.md": """## 证据注记（tg_rn1）
- **DrawBuilder 0xf8B 布局**（O）：+0x00 NoDropOnlyTake<DrawPacket>（首 u64 niche，2=已取走）、+0x10 access_ranges Vec、+0x18 外部绑定 Vec、+0x40 种类 u32 + 参数区、+0xb0 variant、+0xb8 uniform 缓冲 ptr、+0xde 状态字节（==8 无 pass）、+0xe0 &mut RasterPass（+0x00 draws Vec、+0x18 bindings Vec、+0x60 名字→槽表、+0x90 PassBuilder、+0x1a8 PassResources、+0x1b8 ssbo 库、+0x1c8 placement 表、+0x1d0 Mutex<DynamicConstants>、+0x1e0 设备、+0x1f0 world、+0x1f8 视口）、+0xe8 Arc<CompiledShaderPipeline>、+0xf0 bind_common 布尔。
- **draw 种类**（O）：0 NoVao / 1 NoVaoIndexed / 2 NoIndexBuffer / 3 InstancedGlDrawFlag / 4 MultiInstanced / 5 MultiDrawIndirect / 7 IndexedIndirect(×2) / 8 IndexedIndirectCount；**终结操作，无返回值**；`discard` 置 GpuOwned=2。
- **CommonDrawFlag**（B_i_api §2）：`#[repr(u8)]` 0 Off/1 On/2..=4 ShadowCascade0..2；`bind_common` bool uniform = `flag==0`（B_i_api §1.1 纠错）。
- **静态段表** INDEXED_INDIRECT_INDEX_COUNTS 24 项（0x142aaaa78+i×0x14，B_i_api §2 逐项复核，第 10/11 项=0）。
- **panic**：Drop 未 draw/discard（0x14089a140）；&Texture→unreachable（draw_stream.rs:204:9）；绑定≥0x100；placement 缺失文案。
- U：DrawPacket.head[2]（+0/+8）、+0x28 的 Vec<u8>、+0xb1；multi_draw_indirect 各 u64 语义；RasterState 位名（只知 byte0&=0x7f、byte3=(b3&0x9f)|0x40 等用法）；带视图 bind_texture 与 write_storage_buffer_concurrent 之外 `bind_texture<&mut Handle>`（0x1408edd00）无对应 Rust 类型。""",
 f"{PB}/draw_list.rs.md": """## 证据注记（tg_rn1）
- 元素 = `(DrawListItem, fn(DrawBuilder, &EntityDrawParams, &mut World))` 元组 0x80B；回调第二实参即元素指针（`[rdx+8]` = Entity）。
- 字段表（O，0x1409bc8f0..0x1409bc97c）：+0 Option<DrawTopology>（tag u32，None=2）、+8 Entity、+0x10 shader 名 &str、+0x20 Transform 0x30B、+0x50 Handle<Vao>、+0x58 Handle<RasterShaderProgram>、+0x60 排序键 i32、+0x64/0x65 Option<u8> 模板、+0x70 回调。
- `DrawTopology` 变体名 U（tag 0/1 + f32 载荷；`Alt` = tag 1 为 I 命名）；`sort_draw_list` = `sort_by_key(i32)` 稳定（driftsort）。
- 纠错（B_d_gap §2）：旧 `DrawListItem{sort_key:u64, shader_id}` 与 `to_draw_packet` 的 `[0,0]` 头按机器码改为 `[1, vao]`；`EntityDrawParams` 是元素**首字段**。""",
 f"{PB}/draw_stream.rs.md": """## 证据注记（tg_rn1）
- **DrawPacket 0xb8B**（O）：head[2]@0、Vec<(u32 binding,u8 slot)>@0x10、Vec<u8>@0x28、种类 u32+载荷@0x40、HashMap<BindingIndex,GpuOwned<DescriptorSetBinding>>@0x70、shader@0xa0、光栅状态@0xa8、blend 1.0@0xac、variant@0xb0。
- **RasterPass ~0x3e8B**：new_impl 附件计数@+0x7c、深度标志@+0x1c、7 附件视图描述@+0xe4..、纹理句柄@+0x124、mip/层@+0x12c、已渲染@+0x3e4；render 置已渲染 → 附件描述（上限 8）→ create_render_pass 缓存 → PassBuilder::render 提交。
- **TextureBindingKey**（O）：u64 = 0x4000_0000_0000_0000|id，有 view `^ calculate_hash(view)`；`&Texture` 转换恒 panic。
- **DrawPassRenderCheck::drop**（0x140896d80）：`!rendered && !thread::panicking()` → panic。
- U：`DrawStream` 无 PDB 过程（空壳）；`get_render_pass` 形态 I；DrawPacket +0x28 Vec<u8> 语义 U。""",
 f"{PB}/framebuffer_binding.rs.md": """## 证据注记（tg_rn1）
- **ColorAttachment 0x20B**（O）：tag0 引用@+8 / tag1 视图字@+4（16B TextureViewDesc）+ 引用@+0x18；`desc()` 两变体都返回 `ptr+4`；访问类型 tag0→0x36、tag1→0x2e（组 1）。
- **DepthAttachment**（O）：tag0 可写 0x2f / tag1 只读 0x17 / 其它 = 无深度；+4 层、+8 纹理、+0x10 u32 非 0 = 清除、+0x14 f32 清除值。
- **RasterPass::new_impl** 迭代步长 = count<<5（0x20B/附件），上限 5（ArrayVec）。
- 纠错：旧 `RasterFramebufferBinding` 的 5 槽/`color_count` 字段在机器码里没有对应物（B_a4_draw 已删）；`AttachmentTexture` 是 `rhapsody::raw::texture::Texture` 被读字段（id@0、format@0xc、sample_count@0x21、extent）的最小替身。
- U：访问类型数值对应的 `vk_sync_fork::AccessType` 变体名；`RenderPassAttachmentDesc` 0x10B 内部布局（valid=format!=0，I）。""",
 f"{PB}/pass_resource_access.rs.md": """## 证据注记（tg_rn1）
- **访问码**（O，B_i_api/B_a4_compute 纠错后）：0x1a compute 读 / 0x37 写 / 0x32 cpu-mapped 写 / 0x02 间接读 / 0x34 传输写 / 0x1f DrawBuilder 读 / 0x1d 采样图像；`is_write = (a-0x28)<0x10`（0x140c10e10）。
- **三件套流程**（O）：`AssetSsboLibrary::get_rg_buffer<T>`（资源号 = 句柄+0x10）→ `add_resource_access(resource, access, group)` → 绑定数 ≥0x100 `unwrap_failed` → 追加 0x44B RenderPassBinding（read 标签 3 0x140bd6c70 / write 标签 4 0x140bd6c60 / 视图 0x140bd6c00/0x140bd6c30）→ 返回追加前下标（write 返回 u8 供调用方追加 access_range）。**SSBO 访问不经注册表**（旧版臆造的 append_resource_binding 已删）。
- **实例全集**：read 53 / write 37 / write_concurrent 9（全名单见 wp1/contracts/pass_resource_access.md §4：AtomicCounterBuffer、Nani 族 12 个、各 *Ssbo 等）。
- **TextureAccess**（O）：as_texture 返回 3 标签枚举（0 RG {extent,id,desc} / 1 受管 {extent,图像视图,句柄克隆} / 2 按名）；`dims` 恒 extent_2d。
- U：`RenderPassBinding` 判别 2 的含义；`ssbo_to_rg_handle` 真实签名；`SsboType::rg_buffer_hash`（旧名）语义迁移为 SsboId（B_i_api §1.3）。""",
 f"{PB}/pass_resources.rs.md": """## 证据注记（tg_rn1）
- **7 个 TypeId**（O，`movabs` 立即数）：A SHADER_LIBRARY 0x8597ee0727b461b3（→+0x00，必选）、B OPTIONAL 0xf0b963d163fe760f（→+0x30，**可选**）、C TEXTURE_LIBRARY 0xa68561b396784eaa（→+0x08）、D SSBO_LIBRARY 0x8226cc786e679355（→+0x10，与 bind_ray_tracing 同立即数）、E 0x874b1541877dacfd（→+0x18）、F 0x47c94cb78283a72e（→+0x28，= Mutex<DynamicConstants>，I）、G 0xb65bf3141f93e938（→+0x20）。
- 查找顺序 A,B,C,D,E,F,G；B 缺失记 0，其余缺失 `Option::unwrap_failed`（六处）。
- U：+0x18/+0x20/+0x30 的**资源真名**（角色：E/G 帧级核心，候选 frame_constants/camera/Globals/sampler set，I）；`ResourceLookup for World` = typed gap（恒 None，走原版 unwrap panic 路径）。""",
 f"{PB}/shader_uri.rs.md": """## 证据注记（tg_rn1）
- **parse 语义纠错**（B 阶段 vs 旧 wp1 合同）：三段 = `path`（':' 前全部）/ `stem`（文件名去末扩展名）/ `entry`（':' 后，**可选**）；旧合同把 entry 当必选、三段含义全错；无 '.' → `Option::unwrap` panic。
- **source_hash**（O）：SipHash-1-3 零密钥（`DefaultHasher::new()`），str 写字节后补 0xff；`calculate_hash` 0x1405bdda0 与 `render::utils::calculate_hash` 逐位一致。
- **18 个游戏实现**（O，URI 全表见 wp1/contracts/shader_uri.md §3）：9 个 `NaniCullAndBucket*`（/nani/cull_and_bucket{,_bush,_curved_halftimber,_flowers,_plaster,_roof_tile,_smoke,_stairs_floor,_wooden_plank}.cs.hlsl）+ DoF×5 + BinLights + GaussBlur + TgsrMain + GenerateStrokes；PERM 变体在 `as_compute_pipeline`（带 defs）而非 `source_hash`。
- `ComputePass::new` 的 2311B（带 PERM_* 源）与 1943B（&str）两种形态（B_i_api §3.3）。""",
 f"{PB}/uniform_binder.rs.md": """## 证据注记（tg_rn1）
- **守卫四道**（O，全部静默返回）：① reflection[+0xe8 结构]首字非 0 ② 字段表项数非 0 ③ 名字哈希（64 位直接作 hashbrown 哈希）命中取 u32 偏移 ④ `chunk_len < offset` → `slice_start_index_len_fail`。
- **写宽**（O）：u32/i32/f32/bool 4B（bool 以 `movzx eax, r9b` 写整 u32）、[u32;2]/Vec2 8B、Vec3 12B、Vec4 16B、Mat4 64B。
- `write_optional_uniform` 返回 bool = 写了（值由闭包惰性求，如纹理 extent）。
- 键由旧版 `BindingIndex(u32)` 改为 u64 名字哈希（O：查表 `cmp [..], r8` 64 位）。""",
 "nani/buffers.rs.md": """## 证据注记（tg_rn1）
- **MAX_DRAWS_PER_LIST**（O，27 值从 0x142aa14c0 读出）：8192,8192,4096,32768,8192,8192,8192,8192,262144,40960,1024,53120,9600,131072,512,8192,4096,1024,32768,65536,2048,1024,2048,131072,81920,256,32768；同一张表也是 max_instances（0x142aba790/0x142c77010 逐值相同）。
- **108 项互证**：`NaniDrawListInfo` 缓冲 0x1b0B = 108×4B = `NaniDrawListOffsetDefaults`；汇总 (3788800, 71936, 2048, 8)（B_i_nani §2.1）。
- **纠错**：`nani_calculate_draw_list_params` 与 `default_value` 是**两个独立函数**（0x1408baed0+0x77e / 0x1408bb650+0x570 不重叠；wp3 合同称 Ghidra 跳表误读为重叠）。
- 掩码是 **0x3fe** 不是 0x3ff（`test ebx, 0x3fe; setne`，照抄）。
- U：`GpuInstance` 字段；`list_draw_merge_window_range/subtotal_range` 语义（I）；7 个 SSBO 载体（NaniDrawListLength/NaniDrawList/NaniInstanceIndirection/Allocator 等）的库 `SsboType` 注册已由 B_i_nani §2 接线。""",
 "nani/draw_ext.rs.md": """## 证据注记（tg_rn1）
- **PASS_VARIANT_TABLE**（O）：立即数 0x302010000 右移 8*pass_variant 取低字节 → [0,0,1,2,3]。
- **槽名/哈希**（O）：`read_storage_buffer<NaniInstanceIndirectionSsbo>` 槽名 "NaniInstanceIndirectionSsbo"（0x142c8151d）、binding hash 0xcdf9c36a6a9c8c80；`read_rg_storage_buffer(0x2106412d631790c6, handle.resource)`；句柄 = `owned.data().rg_handle()`（External → None，draw_count 仍 U=0）。
- **count/raw 分派**（O）：count 路径偏移 +0x50/+0x58/+0x60/+0x68（cmd_slot*20 / 0x14 / list_index*4 / MAX_DRAWS_PER_LIST[kind]）；raw 路径 r8=cmd_slot*20、r9=0x14、[rsp+0x20]=draw_count；`max_draw_count` 表 0x142aab288 27×u64 实读。
- **A4 缺口**（B_i_nani §2）：`PassHost` 没有 World 资源访问器，`GpuWorld`/`AssetSsboLibrary`/`RenderGraph` 仍以显式参数传入；`draw_*` 现为终结操作所以 `nani_draw` 返回 `()`。
- U：External subset 的 draw_count 来源；Owned 的实例间接缓冲 RG 句柄（原 `OwnedIndirectionHandle` 占位已删）。""",
 "nani/gpu_world.rs.md": """## 证据注记（tg_rn1）
- **GpuWorld 布局**（O，B_n_world §3 推翻旧骨架）：+0x00..+0x1440 = 27×0xc0 `RwLock<Option<GpuWorldSubset>>`（每槽 +0x98/+0xa0 为 trait 对象 {data_ptr, vtable}）；+0x1440 `total_instance_count: u64`；+0x1448 保留（`roll_instance_counts` 写它 = 上帧计数）；+0x1450 `Box<NaniDrawListParams>`（0x70B）；+0x1458 `TypedStorageBuffer<NaniMeshAtlasPartInfo>`；+0x1480 `TypedStorageBuffer<NaniDrawListInfo>`（4B 元素，初始数据 = max_instance_offsets）。旧骨架"7 个主 subset 锁数组 / +0x14a0 params"不存在。
- **OwnedGpuWorldSubset**（O，u64 单位索引）：[0..2] Vec<ChunkId>、[3]/[4]/[6] HashMap<InstanceKey,u32>、[9]/[10] AtomicU64 min/max、[0xb]/[0xc] Vec<InstanceCullData> 紧凑 key、[0x12]/[0x13] NaniInstanceData trait 对象、[0x14] instance_count、[0x16] next_instance_index（单调无 freelist，swap-remove 维护紧凑）。
- **InstanceKey = 12B** `{id:u32, index:u32, kind:u8}`（B_n_world §2.2；16B 的是 `InstanceCullData {Vec3, f16 radius, u16 mesh_index}`）。
- **27 个变体名**（O，0x142aba868/0x142aba940 读出）：Canopy, TreeTrunk, BackgroundTree, SolidVertexColor, SolidVertexColorWindow, SolidVertexColorWindowGlass, SolidVertexColorBricks, GothicWindowBricks, RoofTile, RoofFloor, Plant, Flower, Bush, WoodenPlank, PreplacedFlower, SolidAlbedoMap, SolidAlbedoMapEmissive, Wren, LanternEmissive, Plaster, Cloth, Smoke, BalconyDoor, Uber, CurvedHalfTimber, VelocityOnly, StairsFloor；旧的 Main/Bush/Unknown9..26 全是编造。
- **容量用尽不是 panic**：warn-once 跳过（键 = "module_path! ()"+"$crate :: Level :: Warn"+消息，`Mutex was poisonned`）；`total_instance_count += cull.len()` 无条件。
- **NaniInstanceData vtable**（O）：+0x18 type_name、+0x20 as_any、+0x28 as_any_mut、+0x30 gpu_flush、+0x38 rg_handle、+0x40 import、+0x48 export、+0x50 copy；类型不符 panic 文案 "The instance data for {kind:?} is not {T} but {actual}"（0x142c76920）。
- U：`GpuWorldSubsetDesc` 字段（owned/external/owned_custom 已内联）；External subset 载荷；除 CurvedHalfTimber 外 19 个实例数据字段布局；`NaniMeshAtlasPartInfo` 真实元素大小（只确认 BUFFER_LEN=0x6c0）；`impl$44::fmt`（PDB 归属 gpu_world 实为 InstanceKey 派生 Debug 疑云）；`startup_nani`（27 subset 创建，未恢复）。""",
 "nani/instance_data.rs.md": """## 证据注记（tg_rn1）
- **元素尺寸表**（O，update_instance_data<T> 最大写偏移 + impl$14::copy 步长核对）：0x40 = Uber/SolidVertexColor/Bricks/Window/WindowGlass/SolidAlbedoMap/SolidAlbedoMapEmissive/LanternEmissive/BalconyDoor/Smoke/StoneFloor/Cloth/CurvedHalfTimber；0x30 = Plant/Plaster/StairsFloor/**PreplacedFlower**（旧骨架 0x40 错）；0x60 = VelocityOnly；0x70 = Wren/GothicWindowBricks。
- **CurvedHalfTimber 构造器**（O，0x40B 逐字节）：`new_curved_xy/xz` 细分段数 = next_power_of_two(ceil((|y1-y0|*x1+1)/0.6))（clz 循环），细分等级 = ilog2(...).clamp(2,5)；`new_straight` 的 +0x24=0x3dc28f5c00000001、+0x2c=0x100000000 打包字段语义 U。
- **位域**：WindowGlassFlags（color_id bit31 → unwrap "value out of bounds for field WindowGlassFlags.color_id"，O）、NaniUberInstanceFlags（flags=albedo*1+semi*2+4）、CurvedHalfTimberCaps —— modular_bitfield 0.11.2 派生，字段布局 U。
- 纠错：`curved_halftimber_mesh_atlas_part_idx` 只有 PDB LF_FUNC_ID（内联残留），无机器码依据，reconstruction 已删除；`PlantInstanceData::marker()` 删除（真实 marker 只有 Uber）。""",
 "nani/mesh_atlas.rs.md": """## 证据注记（tg_rn1）
- **MeshAtlas 布局**（O）：mesh 0x00（0x78B）、layouts Vec<MeshAtlasPart(16B)>@0x78、parts HashMap<AtlassedMeshName,u32>@0x90；PDB 的 `ComputedLayout` 是链接器别名，元素就是 `{index_offset, index_count, radius, 0}`。
- **add 签名**（O）：`add(&mut self, name, &Mesh) -> Result<u32, anyhow::Error>`（u32 = 新 part 序号；调用方 MeshAtlasLibrary::add 包 "invalid mesh: " 上下文）。
- 半径 = `sqrt(max(x²+y²)+z²)`（f32::max 语义）；属性判别式非 Float3 → panic "incorrect attribute type"；属性名缺失 → expect "no entry found for key"。
- **属性不一致不回滚**（索引已追加才比较）；**重复名 panic 时布局已追加** —— 两个 O 细节。
- MAX_PARTS = 0x40（一张图集最多 64 part）。""",
 "nani/systems.rs.md": """## 证据注记（tg_rn1）
- **kind ↔ 系统映射**（O，B_n_sys §1/§4）：SolidVertexColor=3、Window=4、Glass=5、Bricks=6、GothicWindowBricks=7、StoneFloor=9、Plant=0xa、PreplacedFlower=0xe、SolidAlbedoMap=0xf、Emissive=0x10、Wren=0x11、LanternEmissive=0x12、Plaster=0x13、Cloth=0x14、Smoke=0x15、BalconyDoor=0x16、Uber=0x17、CurvedHalfTimber=0x18、VelocityOnly=0x19、StairsFloor=0x1a。
- **ECS 参数面**（O，PDB FunctionSystem 单态化；全表见 wp3/contracts/systems.md §3）：maintain = Query<(Entity,&NaniInstanceDesc,&Transform), Without<NaniActiveInstance>> + Changed 查询 + RemovedComponents + Res<MeshAtlasLibrary> + ResMut<GpuWorld> + Local<NaniActiveInstanceCollection> + Commands；update 族 = Query<(&NaniActiveInstance, &Transform[, 组件]), With<InstanceDataMarker<T>>> + Res<GpuWorld>；curved_halftimber 是唯一 `&mut InstanceData` 直写（+Res<PublicWalls>+Res<HalftimberColorIds>+Query<&Roof>）；plaster/stairs_floor 用 Or<(Changed<source>, Changed<NaniActiveInstance>)>。
- **maintain 关键 O**：顺序先删后建再改；NaniInstanceDesc 24B {mesh: bit0 判别 AtlasPart(u32@+4)/Named(NameHash u64@+8), subset_kind u8@+0x10}；Named 经 get_by_name 线性找 part.subset==kind，缺 → panic（行 63:21 文案 "NaniInstanceDesc requested mesh {} and shader {:?},…"）；紧凑 key 16B = {Vec3, f16, u16 mesh_part}（旧文档 {chunk_id,u64…} 错）；flush 27 槽（stride 0xc0）。
- **update_instance_data 签名**（O，0x14156fff0 系）：`(subset, key: &mut{u32,u32}, position: &Vec3, sort_key: f32, data: &T)`；fast-path compact 命中否则 HashMap 回写 key.index。
- **U**：gap 组件（BrickColor/NaniSolidTexture/WindowLight/AlbedoMap/PrevTransform/Wren/GothicWindowBrickDeformation/CurvedHalfTimber/Roof/RoofAnimation/Floor/LanternLightParams/Plaster/ClothlineCloth/NormalMap/Smoke/StairsFloorMeshChunk）经 gap_words 读 0，注释含机器码偏移；terrain_rock_mask_words（有行时未注册 → 原版 unwrap panic，B_x_out 已补访问器）；circle_tangent_u0=0；animated_peel_lookup=None；window_light_value=0.0；`nani_flush_instance_updates` 第二参数类型（I：Res<GraphicsDeviceResource>）；External subset 的 draw_count。""",
 "nani/trimesh.rs.md": """## 证据注记（tg_rn1）
- **TrimeshBuilder 0x50B**（O）：positions_index: Option<usize>@0、chunks Vec@0x10、attribs Vec@0x28、input_max_index: isize@0x40、vertex_count@0x48；`positions` 记录 attrib 下标（[rdx+8]=attribs.len()），不是 bool。
- **attrib 0x18B 记录**（O）：{data ptr+byte_len, elem_size u16, align u8 @0x12}，**无属性名**；Vec3 = elem 0xc / align 4；强制对齐降序；四个断言（空/不足/不等）。
- **meshlet**（O）：每块 ≤64 三角形（TRIANGLES_PER_CHUNK），顶点容量 0xc0 = 64×3；三角形把 3 局部下标打包 `l0|l1<<8|l2<<16`；块内顶点去重成局部 u8 下标。
- **TrimeshChunk 0x20B**（O）：address@0（先缓冲内偏移、建缓冲后 += device_address）、index_count u16@8、…；`build` usage 0x20020；包围球。
- `BufferDeviceAddress`（rhapsody::raw::buffer::Buffer::device_address 0x140bd4c00 的占位）—— B_i_nani §4 已并入 `render/ssbo.rs::rhapsody_boundary::GraphicsDevice`。""",
}

# ---------------------------------------------------------------- 执行
def annotate_row(file, row):
    """row: list of cells (split by ' | '). Returns new cells."""
    cells = [c.strip() for c in row]
    # cells[0] = "| `name`"（行首的 "| " 不是分隔符），cells[1]=addr, [2]=line, [3]=作用, [4]=结论, [5]=证据
    name = cells[0].split("|")[-1].strip().strip("`")
    if file in E:
        hit_e = None
        for k, v in E[file].items():
            if name == k or name.startswith(k + "<"):
                hit_e = v
                break
        if hit_e is not None:
            desc, concl, ev = hit_e
        else:
            desc = concl = ev = None
    else:
        desc = concl = ev = None
    if desc is None:
        hit = None
        for ff, pat, d in FAMILY:
            if ff == file and pat.search(name):
                hit = d
                break
        if hit is None:
            # 未命中：显式报错
            raise SystemExit(f"UNMATCHED row in {file}: {name}")
        desc, concl, ev = hit
    cells[3] = desc
    cells[4] = concl
    cells[5] = ev
    return cells

def annotate_inline(file, line):
    m = re.match(r'^- `([^`]+)`（行 (\d+)）— (.+)$', line.strip())
    if not m:
        return line
    name = m.group(1)
    if (file, name) in INLINE:
        return f"- `{name}`（行 {m.group(2)}）— {INLINE[(file, name)]}"
    # 未命中
    raise SystemExit(f"UNMATCHED inline in {file}: {name}")

def process(fname):
    path = os.path.join(BASE, fname)
    with open(path, encoding="utf-8") as f:
        text = f.read()
    # 幂等：剥掉先前追加的"## 证据注记"段（含重复副本）
    if "## 证据注记（tg_rn1）" in text:
        text = text.split("## 证据注记（tg_rn1）")[0].rstrip("\n") + "\n"
    lines = text.splitlines(keepends=True)
    out = []
    purpose_done = False
    in_biz = False; in_inline = False; in_glue = False
    for ln in lines:
        s = ln.strip()
        if s.startswith("> 文件作用："):
            out.append(f"> 文件作用：{PURPOSE[fname]}\n")
            purpose_done = True
            continue
        if s.startswith("## 业务函数"):
            in_biz = True; in_inline = False; in_glue = False
            out.append(ln); continue
        if s.startswith("## 内联/无独立符号函数"):
            in_biz = False; in_inline = True; in_glue = False
            out.append(ln); continue
        if s.startswith("## 派生胶水"):
            in_biz = False; in_inline = False; in_glue = False
            out.append(ln)
            out.append("\n" + GLUE_NOTE.get(fname, "") + "\n")
            continue
        if s.startswith("## ") and "证据注记" not in s:
            in_biz = False; in_inline = False; in_glue = False
            out.append(ln); continue
        if in_biz and s.startswith("|"):
            if "| 函数 |" in s or s.startswith("|---"):
                out.append(ln); continue
            cells = annotate_row(fname, s.split(" | "))
            out.append(" | ".join(cells) + " |\n")
            continue
        if in_inline and s.startswith("- `"):
            out.append(annotate_inline(fname, ln.rstrip("\n")) + "\n")
            continue
        out.append(ln)
    # 追加证据注记
    out.append("\n" + NOTES[fname] + "\n")
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.writelines(out)
    print(f"OK {fname}")

FILES = [
 "pass_builder/bind_resources.rs.md",
 "pass_builder/compute_pass_builder.rs.md",
 "pass_builder/draw_builder.rs.md",
 "pass_builder/draw_list.rs.md",
 "pass_builder/draw_stream.rs.md",
 "pass_builder/framebuffer_binding.rs.md",
 "pass_builder/pass_resource_access.rs.md",
 "pass_builder/pass_resources.rs.md",
 "pass_builder/shader_uri.rs.md",
 "pass_builder/uniform_binder.rs.md",
 "nani/buffers.rs.md",
 "nani/draw_ext.rs.md",
 "nani/gpu_world.rs.md",
 "nani/instance_data.rs.md",
 "nani/mesh_atlas.rs.md",
 "nani/systems.rs.md",
 "nani/trimesh.rs.md",
]
for f in FILES:
    process(f)
print("DONE")
