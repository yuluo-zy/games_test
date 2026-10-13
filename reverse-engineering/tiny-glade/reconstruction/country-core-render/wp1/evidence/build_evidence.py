import hashlib, json
from pathlib import Path

root = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade")
skeleton_root = root / "skeleton/B/crates/country-core/src/render/pass_builder"
dec_dir = root / "evidence/render-passes/pass-builder"

mods = ["compute_pass_builder","pass_resource_access","draw_builder","bind_resources","pass_resources","uniform_binder","framebuffer_binding","draw_list","draw_stream","shader_uri"]
suffixes = [".rs.pdb.json",".rs.structures.json.gz",".rs.enriched.json.gz",".rs.dependencies.json",".rs.project-evidence.json.gz"]

def sha256(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

bundles = {}
for m in mods:
    for s in suffixes:
        p = skeleton_root / f"{m}.rs{s}"
        if p.exists():
            bundles[f"{m}.rs{s}"] = {"path": str(p), "sha256": sha256(p)}
        else:
            bundles[f"{m}.rs{s}"] = {"path": str(p), "sha256": None, "missing": True}

# decompiles used
decs = sorted(p.name for p in dec_dir.glob("dec*_0x*.c"))
dec_sha = {}
for d in decs:
    p = dec_dir / d
    dec_sha[d] = sha256(p)

# addresses of primary native procs analyzed
addresses = {
    "ComputePass::new<String>": "0x1405876a0",
    "ComputePass::read_rg_storage_buffer": "0x14096dac0",
    "ComputePass::write_rg_storage_buffer": "0x14096dd60",
    "ComputePass::write_rg_storage_buffer_concurrent": "0x14096e000",
    "ComputePass::bind_external_storage_buffer": "0x14096e2a0",
    "ComputePass::bind_external_raw": "0x14096e540",
    "ComputePass::dispatch_indirect_raw": "0x14096e880",
    "ComputePass::dispatch_1d": "0x14096eaf0",
    "ComputePass::dispatch_2d": "0x14096ec80",
    "ComputePass::dispatch_groups": "0x14096ee10",
    "ComputeDispatchApi::dispatch_3d": "0x14096efa0",
    "ComputePass::dispatch_with<cull_and_bucket>": "0x14160a2e0",
    "ComputePass::dispatch_indirect_with<group_draw_cmd_prefix_sum>": "0x14160b850",
    "ComputePass::write_to_cpu_mapped_storage_buffer<MouseDepthValueSsbo>": "0x141609770",
    "ComputePass::read_storage_buffer<Cobblestone2RasterStagingSsbo>": "0x140588650",
    "pass_resource_access::read_access_buffer<AtomicCounterBuffer>": "0x140970620",
    "pass_resource_access::write_access_buffer<NaniDrawListLengthSsbo>": "0x140975920",
    "pass_resource_access::write_access_buffer_concurrent<NaniNextVisibilityBitmaskSsbo>": "0x1409764a0",
    "DrawBuilder::new": "0x140905be0",
    "DrawBuilder::read_rg_storage_buffer": "0x140906150",
    "DrawBuilder::bind_external_storage_buffer": "0x140906400",
    "DrawBuilder::draw": "0x140906f40",
    "DrawBuilder::draw_multi_instanced": "0x140907050",
    "DrawBuilder::draw_instanced_from_placement_data": "0x140907280",
    "DrawBuilder::draw_instanced_w_gl_draw_flag": "0x140907490",
    "DrawBuilder::multi_draw_indirect": "0x1409076b0",
    "DrawBuilder::multi_draw_indexed_indirect": "0x140907990",
    "DrawBuilder::draw_indexed_indirect_raw": "0x140907cc0",
    "DrawBuilder::draw_indexed_indirect_count": "0x140907f90",
    "DrawBuilder::discard": "0x1409082c0",
    "DrawBuilder::optional_uniform<f32>": "0x141629a20",
    "PassResources::new_from": "0x140909310",
    "ColorAttachment::desc": "0x1409092f0",
    "UniformBinder::write_uniform<Vec3>": "0x14096b660",
    "UniformBinder::write_optional_uniform<[u32;2]>": "0x14096be40",
    "RasterPass::new_impl": "0x140867970",
    "RasterPass::render": "0x140868370",
    "DrawPacket::new": "0x140867850",
    "TextureBindingKey::with_view": "0x140867940",
    "ShaderUri::parse": "0x140968960",
    "bind_resources::bind_color_library<DrawBuilder>": "0x1416286f0",
    "bind_resources::bind_ray_tracing<DrawBuilder>": "0x1416289d0",
    "rhapsody::PassBuilder::render<screenshot>": "0x140533d90",
    "rhapsody::PassBuilder::add_resource_access": "0x1409dca90",
}

# procedure instance counts per generic (from enriched.json) - quick scan
instance_counts = {}
for m in mods:
    import gzip
    p = skeleton_root / f"{m}.rs.enriched.json.gz"
    with gzip.open(p,"rt") as f:
        data = json.load(f)
    funcs = data.get("functions", [])
    named = {}
    for fn in funcs:
        ci = fn.get("compiled_instances", [])
        named[fn.get("name")] = len(ci)
    instance_counts[m] = {"function_count": len(funcs), "with_compiled_instances": sum(1 for v in named.values() if v>0)}

evidence = {
    "target": {
        "version": "B",
        "exe": {"path": str(root/"evidence/render-passes"), "sha256": "cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323"},
        "pdb": {"sha256": "2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911"},
    },
    "method": (
        "业务逻辑：Ghidra 只读反编译（55 个既有 dec*.c）+ PDB 骨架 bundle（structures/enriched/pdb/dependencies）+ "
        "rhapsody-rebuild API 边界。机器码差分仅用于裁决数值/边界歧义，不作实现手段。"
    ),
    "skeleton_bundles": bundles,
    "decompiles_used": dec_sha,
    "primary_addresses": addresses,
    "instance_counts": instance_counts,
    "judgments": [
        "ComputePass 尺寸 400B、按值 builder 返回：由 read_rg_storage_buffer 末端 memcpy(in_RCX,in_RDX,400) 直接观测（O）。",
        "RenderPassBinding 0x44B 布局 = 4×16B + 4B：由 0x14096dac0 写回序列直接观测；字段语义归 rhapsody::rg::pass_api（复用边界）。",
        "access_range 8B = {u32 resource_index, u8 access_index}：由 0x14096dac0 写 u32@+0 + u8@+4 观测；元素类型名为 Range<ResourceAccessIndex>。",
        "hashbrown 注册表 {+0x98 桶, +0xa0 掩码, +0xb0 计数}，键=句柄 hash u64，值={u32 set, u32 resource}：由多处组查找/assert_failed<SetIndex> 观测。",
        "dispatch 模式 2/3/4 与 draw 模式 1/3/4/5/7/8 为机器码判别值；Rust 枚举名未恢复（typed gap），不臆造名字。",
        "PassResources 7 hash 常量直接观测（0x140909310）；资源名映射 U。",
        "ShaderUri 48B=3×&str：由 parse 三处 memrchr + 6×8B 写回观测。",
        "DrawBuilder 0xf8B：new 末端 memcpy(in_RCX,local_298,0xb8)+扩展写回观测。",
        "RasterPass ~0x3e8B：new_impl 写至 +0x3e4；render 置 +0x3e4 已渲染标记。",
        "DrawPacket +0xac=0x3f800000（blend 1.0）与 RandomState 由 DrawPacket::new 观测。",
        "PassBuilder::render/add_resource_access 属 rhapsody 复用依赖：参照 reconstruction/rhapsody-rebuild 的候选 API，不重实现。",
    ],
    "gaps": [
        "UniformBinder 精确尺寸；ComputePass +0x48..+0x130 字段；DrawBuilder +0x60..+0xb0 字段。",
        "PassResources 资源名映射（hash→资源名）。",
        "ShaderUri source_hash 的 hash 算法。",
        "dispatch/draw 模式判别值 1/2/3/4/5/7/8 → Rust 枚举名。",
        "RenderPassBinding 各字段语义（归 rhapsody）。",
        "ColorAttachment::desc 真实逻辑（反编译为空占位，被内联）。",
        "DrawPacket/DrawStream 精确尺寸与 DrawStream 方法集。",
        "DrawBuilder::draw_instanced_from_placement_data 的 placement map 值与 GlobalId 语义。",
        "AsShaderDefValue 的 def 编码与两 16-hex 变体对选择规则。",
    ],
}
Path("evidence.json").write_text(json.dumps(evidence, indent=2, ensure_ascii=False))
print("written evidence.json")
print("instance_counts:", json.dumps(instance_counts)[:600])
