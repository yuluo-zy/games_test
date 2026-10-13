//! spv-diff: run two compute SPIR-V modules (oracle vs candidate) with identical seeded inputs
//! on a Vulkan device (lavapipe) and compare every writable output.
//!
//! usage: spv-diff <a.spv> <b.spv> <entry> [profile ...]
//! stdout: `SKIP <reason>` or one `RES ...` line per output binding and profile, then `DONE`.

use ash::vk;
use rspirv::dr::{Instruction, Module, Operand};
use rspirv::spirv::{Decoration, Dim, Op, StorageClass};
use std::collections::HashMap;
use std::ffi::CString;

const BUF_BYTES: usize = 64 * 1024;
const IMG_DIM: u32 = 64;
const PUSH_BYTES: u32 = 128;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Chan {
    F32,
    F16,
    Norm8,
    Norm16,
    Int32,
    Int16,
    Int8,
    Packed,
}

#[derive(Clone, Copy, Debug)]
struct Fmt {
    vk: vk::Format,
    texel: usize,
    chan: Chan,
}

#[derive(Clone, Debug)]
enum Kind {
    Sampler,
    Sampled(Fmt),
    Storage(Fmt),
    Ubo,
    Ssbo,
}

#[derive(Clone, Copy, Debug)]
struct Shape {
    d3: bool,
    layers: u32, // array layers (2D array) or depth (3D); 1 for plain 2D
    arrayed: bool,
    ms: bool, // 4-sample sampled image (Texture2DMS)
    count: u32, // descriptor array length
}

#[derive(Clone, Debug)]
struct Res {
    addr_offsets: Vec<(u32, u32)>, // (byte offset, array stride or 0) of *addr* members in a UBO/SSBO block
    shape: Shape,
    set: u32,
    binding: u32,
    name: String,
    kind: Kind,
}

struct Reflection {
    has_shared: bool,
    ins: Vec<(u32, char, u32, bool)>,  // location, kind f/u/i, components, flat
    outs: Vec<(u32, char, u32)>,
    res: Vec<Res>,
    local: [u32; 3],
    push_addr_offsets: Vec<u32>,
    phys: bool,
}

fn rng_next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
fn rnd01(s: &mut u64) -> f32 {
    (rng_next(s) >> 40) as f32 / (1u64 << 24) as f32
}

fn id_of(op: &Operand) -> Option<u32> {
    if let Operand::IdRef(i) = op { Some(*i) } else { None }
}
fn lit(op: &Operand) -> Option<u32> {
    if let Operand::LiteralInt32(i) = op { Some(*i) } else { None }
}

fn image_fmt(code: u32, sampled_int: Option<bool>) -> Fmt {
    use vk::Format as F;
    let f = |vk, texel, chan| Fmt { vk, texel, chan };
    match code {
        1 => f(F::R32G32B32A32_SFLOAT, 16, Chan::F32),
        2 => f(F::R16G16B16A16_SFLOAT, 8, Chan::F16),
        3 => f(F::R32_SFLOAT, 4, Chan::F32),
        4 => f(F::R8G8B8A8_UNORM, 4, Chan::Norm8),
        5 => f(F::R8G8B8A8_SNORM, 4, Chan::Norm8),
        6 => f(F::R32G32_SFLOAT, 8, Chan::F32),
        7 => f(F::R16G16_SFLOAT, 4, Chan::F16),
        8 => f(F::B10G11R11_UFLOAT_PACK32, 4, Chan::Packed),
        9 => f(F::R16_SFLOAT, 2, Chan::F16),
        10 => f(F::R16G16B16A16_UNORM, 8, Chan::Norm16),
        11 => f(F::A2B10G10R10_UNORM_PACK32, 4, Chan::Packed),
        12 => f(F::R16G16_UNORM, 4, Chan::Norm16),
        13 => f(F::R8G8_UNORM, 2, Chan::Norm8),
        14 => f(F::R16_UNORM, 2, Chan::Norm16),
        15 => f(F::R8_UNORM, 1, Chan::Norm8),
        16 => f(F::R16G16B16A16_SNORM, 8, Chan::Norm16),
        17 => f(F::R16G16_SNORM, 4, Chan::Norm16),
        18 => f(F::R8G8_SNORM, 2, Chan::Norm8),
        19 => f(F::R16_SNORM, 2, Chan::Norm16),
        20 => f(F::R8_SNORM, 1, Chan::Norm8),
        21 => f(F::R32G32B32A32_SINT, 16, Chan::Int32),
        22 => f(F::R16G16B16A16_SINT, 8, Chan::Int16),
        23 => f(F::R8G8B8A8_SINT, 4, Chan::Int8),
        24 => f(F::R32_SINT, 4, Chan::Int32),
        25 => f(F::R32G32_SINT, 8, Chan::Int32),
        26 => f(F::R16G16_SINT, 4, Chan::Int16),
        27 => f(F::R8G8_SINT, 2, Chan::Int8),
        28 => f(F::R16_SINT, 2, Chan::Int16),
        29 => f(F::R8_SINT, 1, Chan::Int8),
        30 => f(F::R32G32B32A32_UINT, 16, Chan::Int32),
        31 => f(F::R16G16B16A16_UINT, 8, Chan::Int16),
        32 => f(F::R8G8B8A8_UINT, 4, Chan::Int8),
        33 => f(F::R32_UINT, 4, Chan::Int32),
        34 => f(F::R32G32_UINT, 8, Chan::Int32),
        35 => f(F::R16G16_UINT, 4, Chan::Int16),
        36 => f(F::R8G8_UINT, 2, Chan::Int8),
        37 => f(F::R16_UINT, 2, Chan::Int16),
        38 => f(F::R8_UINT, 1, Chan::Int8),
        _ => match sampled_int {
            Some(true) => f(F::R32G32B32A32_UINT, 16, Chan::Int32),
            _ => f(F::R32G32B32A32_SFLOAT, 16, Chan::F32),
        },
    }
}

fn numeric(types: &HashMap<u32, &Instruction>, id: u32) -> Option<(char, u32)> {
    let t = types.get(&id)?;
    match t.class.opcode {
        Op::TypeFloat => Some(('f', 1)),
        Op::TypeInt => Some((if lit(&t.operands[1]) == Some(1) { 'i' } else { 'u' }, 1)),
        Op::TypeVector => numeric(types, id_of(&t.operands[0])?).map(|(k, _)| (k, lit(&t.operands[1]).unwrap_or(1))),
        _ => None,
    }
}

fn reflect(m: &Module, entry: &str) -> Result<Reflection, String> {
    let mut types: HashMap<u32, &Instruction> = HashMap::new();
    for i in &m.types_global_values {
        if let Some(r) = i.result_id {
            types.insert(r, i);
        }
    }
    let mut deco: HashMap<(u32, u32), Vec<u32>> = HashMap::new(); // (id, decoration) -> args
    let mut member_off: HashMap<(u32, u32), u32> = HashMap::new();
    for a in &m.annotations {
        match a.class.opcode {
            Op::Decorate => {
                let t = id_of(&a.operands[0]).unwrap();
                if let Operand::Decoration(d) = a.operands[1] {
                    deco.insert((t, d as u32), a.operands[2..].iter().filter_map(lit).collect());
                }
            }
            Op::MemberDecorate => {
                let t = id_of(&a.operands[0]).unwrap();
                let mem = lit(&a.operands[1]).unwrap();
                if let Operand::Decoration(Decoration::Offset) = a.operands[2] {
                    member_off.insert((t, mem), lit(&a.operands[3]).unwrap());
                }
            }
            _ => {}
        }
    }
    let mut names: HashMap<u32, String> = HashMap::new();
    let mut member_names: HashMap<(u32, u32), String> = HashMap::new();
    for d in &m.debug_names {
        match d.class.opcode {
            Op::Name => {
                if let (Some(t), Operand::LiteralString(s)) = (id_of(&d.operands[0]), &d.operands[1]) {
                    names.insert(t, s.clone());
                }
            }
            Op::MemberName => {
                if let (Some(t), Some(mem), Operand::LiteralString(s)) =
                    (id_of(&d.operands[0]), lit(&d.operands[1]), &d.operands[2])
                {
                    member_names.insert((t, mem), s.clone());
                }
            }
            _ => {}
        }
    }
    let mut local = [1, 1, 1];
    for e in &m.execution_modes {
        if let (Some(_), Operand::ExecutionMode(rspirv::spirv::ExecutionMode::LocalSize)) = (id_of(&e.operands[0]), &e.operands[1]) {
            local = [lit(&e.operands[2]).unwrap(), lit(&e.operands[3]).unwrap(), lit(&e.operands[4]).unwrap()];
        }
    }
    let _ = entry;
    let caps_phys = m.capabilities.iter().any(|c| matches!(c.operands[0], Operand::Capability(rspirv::spirv::Capability::PhysicalStorageBufferAddresses)));
    let mut res = Vec::new();
    let (mut ins, mut outs) = (Vec::new(), Vec::new());
    let mut push_addr_offsets = Vec::new();
    for v in &m.types_global_values {
        if v.class.opcode != Op::Variable {
            continue;
        }
        let sc = if let Operand::StorageClass(s) = v.operands[0] { s } else { continue };
        if matches!(sc, StorageClass::Input | StorageClass::Output) {
            let vid = v.result_id.unwrap();
            if let Some(loc) = deco.get(&(vid, Decoration::Location as u32)).and_then(|a| a.first().copied()) {
                let ptr = types[&v.result_type.unwrap()];
                if let Some((k, n)) = numeric(&types, id_of(&ptr.operands[1]).unwrap()) {
                    if sc == StorageClass::Input { ins.push((loc, k, n, deco.contains_key(&(vid, Decoration::Flat as u32)))); } else { outs.push((loc, k, n)); }
                }
            }
            continue;
        }
        if !matches!(sc, StorageClass::UniformConstant | StorageClass::Uniform | StorageClass::StorageBuffer | StorageClass::PushConstant) {
            continue;
        }
        let vid = v.result_id.unwrap();
        let ptr = types[&v.result_type.unwrap()];
        let mut pointee = id_of(&ptr.operands[1]).unwrap();
        if sc == StorageClass::PushConstant {
            for mem in 0..64u32 {
                if let Some(n) = member_names.get(&(pointee, mem)) {
                    if n.contains("addr") {
                        if let Some(o) = member_off.get(&(pointee, mem)) {
                            push_addr_offsets.push(*o);
                        }
                    }
                }
            }
            continue;
        }
        let set = deco.get(&(vid, Decoration::DescriptorSet as u32)).and_then(|a| a.first().copied()).unwrap_or(0);
        let binding = match deco.get(&(vid, Decoration::Binding as u32)).and_then(|a| a.first().copied()) {
            Some(b) => b,
            None => continue,
        };
        let name = names.get(&vid).cloned().unwrap_or_default();
        let mut t = types[&pointee];
        let mut count = 1u32;
        if matches!(t.class.opcode, Op::TypeRuntimeArray | Op::TypeArray) {
            count = if t.class.opcode == Op::TypeArray {
                id_of(&t.operands[1]).and_then(|c| types.get(&c)).and_then(|c| lit(&c.operands[0])).unwrap_or(1024)
            } else { 1024 };
            pointee = id_of(&t.operands[0]).unwrap();
            t = types[&pointee];
        }
        let mut shape = Shape { d3: false, layers: 1, arrayed: false, ms: false, count };
        let mut addr_offsets: Vec<(u32, u32)> = Vec::new();
        pointee = t.result_id.unwrap();
        let kind = match t.class.opcode {
            Op::TypeSampler => Kind::Sampler,
            Op::TypeImage => {
                let sampled_ty = types[&id_of(&t.operands[0]).unwrap()];
                let sampled_int = Some(sampled_ty.class.opcode == Op::TypeInt);
                let dim = if let Operand::Dim(d) = t.operands[1] { d } else { Dim::Dim2D };
                let arrayed = lit(&t.operands[3]).unwrap_or(0);
                let ms = lit(&t.operands[4]).unwrap_or(0);
                let sampled_flag = lit(&t.operands[5]).unwrap_or(1);
                if !(dim == Dim::Dim2D || dim == Dim::Dim3D) || (dim == Dim::Dim3D && arrayed != 0) || (ms != 0 && (sampled_flag == 2 || arrayed != 0 || dim != Dim::Dim2D)) {
                    return Err(format!("unsupported image dim {dim:?} arrayed={arrayed} ms={ms} ({name})"));
                }
                if ms != 0 { shape.ms = true; }
                if dim == Dim::Dim3D { shape.d3 = true; shape.layers = 4; }
                if arrayed != 0 { shape.arrayed = true; shape.layers = 4; }
                let sampled = lit(&t.operands[5]).unwrap_or(1);
                let code = if let Operand::ImageFormat(f) = t.operands[6] { f as u32 } else { 0 };
                let fmt = image_fmt(code, sampled_int);
                if sampled == 2 { Kind::Storage(fmt) } else { Kind::Sampled(image_fmt(0, sampled_int)) }
            }
            Op::TypeStruct => {
                for mem in 0..128u32 {
                    let off = match member_off.get(&(pointee, mem)) { Some(o) => *o, None => continue };
                    if member_names.get(&(pointee, mem)).map(|n| n.contains("addr")).unwrap_or(false) { addr_offsets.push((off, 0)); continue; }
                    // runtime array / array of structs holding device addresses (e.g. nani_instance_data)
                    let mt = match types[&pointee].operands.get(mem as usize).and_then(id_of).and_then(|i| types.get(&i)) { Some(t) => *t, None => continue };
                    if matches!(mt.class.opcode, Op::TypeRuntimeArray | Op::TypeArray) {
                        let (aid, eid) = (mt.result_id.unwrap(), id_of(&mt.operands[0]).unwrap());
                        let stride = deco.get(&(aid, Decoration::ArrayStride as u32)).and_then(|a| a.first().copied()).unwrap_or(0);
                        for em in 0..128u32 {
                            if member_names.get(&(eid, em)).map(|n| n.contains("addr")).unwrap_or(false) {
                                if let Some(eo) = member_off.get(&(eid, em)) { addr_offsets.push((off + eo, stride)); }
                            }
                        }
                    }
                }
                if sc == StorageClass::StorageBuffer || deco.contains_key(&(pointee, Decoration::BufferBlock as u32)) {
                    Kind::Ssbo
                } else {
                    Kind::Ubo
                }
            }
            op => return Err(format!("unsupported resource type {op:?} ({name})")),
        };
        t = types[&pointee];
        let _ = t;
        res.push(Res { addr_offsets, shape, set, binding, name, kind });
    }
    ins.sort();
    outs.sort();
    let has_shared = m.types_global_values.iter().any(|v| v.class.opcode == Op::Variable && matches!(v.operands[0], Operand::StorageClass(StorageClass::Workgroup)));
    Ok(Reflection { has_shared, ins, outs, res, local, push_addr_offsets, phys: caps_phys })
}

struct Gpu {
    _entry: ash::Entry,
    instance: ash::Instance,
    dev: ash::Device,
    mem_props: vk::PhysicalDeviceMemoryProperties,
    queue: vk::Queue,
    pool: vk::CommandPool,
}

impl Gpu {
    fn new() -> Gpu {
        unsafe {
            let entry = ash::Entry::load().expect("load vulkan");
            let app = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);
            let instance = entry.create_instance(&vk::InstanceCreateInfo::default().application_info(&app), None).expect("instance");
            let pd = instance.enumerate_physical_devices().unwrap()[0];
            let mem_props = instance.get_physical_device_memory_properties(pd);
            let prio = [1.0f32];
            let qci = [vk::DeviceQueueCreateInfo::default().queue_family_index(0).queue_priorities(&prio)];
            let mut v13 = vk::PhysicalDeviceVulkan13Features::default().dynamic_rendering(true);
            let mut v12 = vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true).runtime_descriptor_array(true).descriptor_binding_partially_bound(true).shader_sampled_image_array_non_uniform_indexing(true).shader_storage_image_array_non_uniform_indexing(true).shader_uniform_buffer_array_non_uniform_indexing(true).shader_int8(true);
            let mut f2 = vk::PhysicalDeviceFeatures2::default().features(
                vk::PhysicalDeviceFeatures::default()
                    .robust_buffer_access(true)
                    .fragment_stores_and_atomics(true)
                    .vertex_pipeline_stores_and_atomics(true)
                    .shader_int64(true)
                    .shader_int16(true)
                    .shader_storage_image_extended_formats(true)
                    .shader_storage_image_write_without_format(true)
                    .shader_storage_image_read_without_format(true)
                    .sample_rate_shading(true),
            ).push_next(&mut v12).push_next(&mut v13);
            let dev = instance.create_device(pd, &vk::DeviceCreateInfo::default().queue_create_infos(&qci).push_next(&mut f2), None).expect("device");
            let queue = dev.get_device_queue(0, 0);
            let pool = dev.create_command_pool(&vk::CommandPoolCreateInfo::default().queue_family_index(0).flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER), None).unwrap();
            Gpu { _entry: entry, instance, dev, mem_props, queue, pool }
        }
    }
    fn mem_type(&self, bits: u32, flags: vk::MemoryPropertyFlags) -> u32 {
        (0..self.mem_props.memory_type_count)
            .find(|i| bits & (1 << i) != 0 && self.mem_props.memory_types[*i as usize].property_flags.contains(flags))
            .expect("memory type")
    }
    fn buffer(&self, size: usize) -> (vk::Buffer, vk::DeviceMemory, *mut u8) {
        unsafe {
            let b = self.dev.create_buffer(
                &vk::BufferCreateInfo::default().size(size as u64).usage(
                    vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC
                        | vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                ), None).unwrap();
            let req = self.dev.get_buffer_memory_requirements(b);
            let mut flags = vk::MemoryAllocateFlagsInfo::default().flags(vk::MemoryAllocateFlags::DEVICE_ADDRESS);
            let mem = self.dev.allocate_memory(
                &vk::MemoryAllocateInfo::default().allocation_size(req.size)
                    .memory_type_index(self.mem_type(req.memory_type_bits, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT))
                    .push_next(&mut flags), None).unwrap();
            self.dev.bind_buffer_memory(b, mem, 0).unwrap();
            let p = self.dev.map_memory(mem, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty()).unwrap() as *mut u8;
            (b, mem, p)
        }
    }
    fn image(&self, fmt: vk::Format, sh: Shape) -> (vk::Image, vk::ImageView) {
        unsafe {
            let img = self.dev.create_image(
                &vk::ImageCreateInfo::default().image_type(if sh.d3 { vk::ImageType::TYPE_3D } else { vk::ImageType::TYPE_2D }).format(fmt)
                    .extent(vk::Extent3D { width: IMG_DIM, height: IMG_DIM, depth: if sh.d3 { sh.layers } else { 1 } }).mip_levels(1).array_layers(if sh.arrayed { sh.layers } else { 1 })
                    .samples(if sh.ms { vk::SampleCountFlags::TYPE_4 } else { vk::SampleCountFlags::TYPE_1 }).tiling(vk::ImageTiling::OPTIMAL)
                    .usage(if sh.ms { vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::COLOR_ATTACHMENT } else { vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST })
                    .initial_layout(vk::ImageLayout::UNDEFINED), None).expect("image");
            let req = self.dev.get_image_memory_requirements(img);
            let mem = self.dev.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size)
                .memory_type_index(self.mem_type(req.memory_type_bits, vk::MemoryPropertyFlags::empty())), None).unwrap();
            self.dev.bind_image_memory(img, mem, 0).unwrap();
            let view = self.dev.create_image_view(
                &vk::ImageViewCreateInfo::default().image(img).view_type(if sh.d3 { vk::ImageViewType::TYPE_3D } else if sh.arrayed { vk::ImageViewType::TYPE_2D_ARRAY } else { vk::ImageViewType::TYPE_2D }).format(fmt)
                    .subresource_range(vk::ImageSubresourceRange { aspect_mask: vk::ImageAspectFlags::COLOR, base_mip_level: 0, level_count: 1, base_array_layer: 0, layer_count: if sh.arrayed { sh.layers } else { 1 } }), None).unwrap();
            (img, view)
        }
    }
}

/// The 16 input profiles. Each yields a (float, int) view of the same intent for a given word index:
///  0 unit floats / ints<64      1 small ints (floats scaled)   2 mixed                 3 signed [-1,1]
///  4 moderate floats [0,6)      5 moderate ints <8             6 IDENTITY 4x4 blocks   7 all zero
///  8 all one                    9 max values (u32::MAX, 3e38) 10 powers of two        11 sparse (75% zero)
/// 12 near-constant 0.45..0.55  13 sequence i%64               14 large floats <1e4    15 tiny signed
fn gen_word(profile: u32, idx: usize, r: f32, r2: f32) -> (f32, u32) {
    match profile % 16 {
        0 => (r, (r * 64.0) as u32),
        1 => (r * 64.0, (r * 4.0) as u32),
        2 => if idx % 2 == 0 { (r * 4.0, (r * 1024.0) as u32) } else { (r * 16.0, (r * 16.0) as u32) },
        3 => (r * 2.0 - 1.0, (r * 16.0) as u32),
        4 => (r * 6.0, (r * 32.0) as u32),
        5 => (r * 8.0, (r * 8.0) as u32),
        6 => { let on = matches!(idx % 16, 0 | 5 | 10 | 15); (if on { 1.0 } else { 0.0 }, on as u32) }
        7 => (0.0, 0),
        8 => (1.0, 1),
        9 => (3.0e38, 0xFFFF), // floats at the top of the range; ints large but loop-safe
        10 => { let k = (r2 * 9.0) as i32; (if idx % 3 == 0 { -(2f32.powi(k - 4)) } else { 2f32.powi(k - 4) }, 1u32 << (k as u32)) }
        11 => if r2 < 0.75 { (0.0, 0) } else { (r, (r * 8.0) as u32) },
        12 => (0.45 + r * 0.1, (r * 2.0) as u32),
        13 => ((idx % 64) as f32 / 64.0, (idx % 64) as u32),
        14 => (r * 1.0e4, (r * 65536.0) as u32),
        _ => ((r * 2.0 - 1.0) * 1.0e-6, ((r * 3.0) as i32 - 1) as u32),
    }
}

/// Coverage-search profiles (>= 16): every 32-bit word is independently a small integer (counts, indices,
/// flags), a unit float, a mid float, or zero, so one buffer can carry both count fields and float data.
/// s = (profile-16) % 16: bits 0-1 pick the share of integer words, bits 2-3 the integer range.
/// Profiles 16..32 are the "well formed" mixes, 32.. add zeros/large values (see diff_common.CORE_PROFILES).
fn gen_mixed(profile: u32, r: f32, r2: f32) -> u32 {
    let s = (profile - 16) % 16;
    let wild = profile >= 32;
    let int_share = [0.25f32, 0.5, 0.75, 0.9][(s & 3) as usize];
    let range = [2u32, 4, 16, 128][((s >> 2) & 3) as usize];
    if r2 < int_share { (r * range as f32) as u32 + if wild && r < 0.1 { 0x7FFF_FFF0 } else { 0 } }
    else if r2 < int_share + (1.0 - int_share) * 0.6 { (r * 4.0 - 1.0).to_bits() }
    else if wild && r < 0.3 { 0 } else { (r * 64.0).to_bits() }
}

fn fill_bytes(chan: Chan, bytes: &mut [u8], profile: u32, seed: &mut u64) {
    if profile >= 16 && matches!(chan, Chan::F32 | Chan::Int32 | Chan::Packed) {
        for w in bytes.chunks_exact_mut(4) {
            let (r, r2) = (rnd01(seed), rnd01(seed));
            w.copy_from_slice(&gen_mixed(profile, r, r2).to_le_bytes());
        }
        return;
    }
    let unit = match chan { Chan::F32 | Chan::Int32 | Chan::Packed => 4, Chan::F16 | Chan::Norm16 | Chan::Int16 => 2, _ => 1 };
    let mut i = 0;
    while i + unit <= bytes.len() {
        let (r, r2) = (rnd01(seed), rnd01(seed));
        let (f, n) = gen_word(profile, i / 4, r, r2);
        match chan {
            Chan::F32 => bytes[i..i + 4].copy_from_slice(&f.to_le_bytes()),
            Chan::F16 => bytes[i..i + 2].copy_from_slice(&half_bits(f.clamp(-65000.0, 65000.0)).to_le_bytes()),
            Chan::Int32 | Chan::Packed => bytes[i..i + 4].copy_from_slice(&n.to_le_bytes()),
            Chan::Int16 => bytes[i..i + 2].copy_from_slice(&(n as u16).to_le_bytes()),
            Chan::Norm16 => bytes[i..i + 2].copy_from_slice(&((f.clamp(0.0, 1.0) * 65535.0) as u16).to_le_bytes()),
            Chan::Int8 => bytes[i] = n as u8,
            Chan::Norm8 => bytes[i] = (f.clamp(0.0, 1.0) * 255.0) as u8,
        }
        i += unit;
    }
}

/// Buffers have no declared type; choose the integer view for the index-like profiles.
fn buffer_chan(profile: u32) -> Chan {
    if matches!(profile % 16, 1 | 5 | 13) { Chan::Int32 } else { Chan::F32 }
}

fn half_bits(v: f32) -> u16 {
    let b = v.to_bits();
    let sign = ((b >> 16) & 0x8000) as u16;
    let exp = ((b >> 23) & 0xff) as i32 - 127 + 15;
    let man = b & 0x7fffff;
    if exp <= 0 { sign } else if exp >= 31 { sign | 0x7bff } else { sign | ((exp as u16) << 10) | ((man >> 13) as u16) }
}
fn half_to_f32(h: u16) -> f32 {
    let s = ((h >> 15) & 1) as u32;
    let e = ((h >> 10) & 0x1f) as i32;
    let m = (h & 0x3ff) as u32;
    let v = if e == 0 { (m as f32) * 2f32.powi(-24) } else if e == 31 { f32::NAN } else { (1.0 + m as f32 / 1024.0) * 2f32.powi(e - 15) };
    if s == 1 { -v } else { v }
}

fn words_close(a: u32, b: u32) -> bool {
    if a == b { return true; }
    let (fa, fb) = (f32::from_bits(a), f32::from_bits(b));
    if fa.is_nan() && fb.is_nan() { return true; }
    if !fa.is_finite() || !fb.is_finite() { return false; }
    (fa - fb).abs() <= 2e-4 * fa.abs().max(fb.abs()).max(1.0)
}

fn compare(chan: Chan, a: &[u8], b: &[u8]) -> (usize, Option<usize>) {
    let mut mism = 0;
    let mut first = None;
    let unit = match chan { Chan::F32 | Chan::Int32 | Chan::Packed => 4, Chan::F16 | Chan::Norm16 | Chan::Int16 => 2, _ => 1 };
    let mut i = 0;
    while i + unit <= a.len() {
        let ok = match chan {
            Chan::F32 => words_close(u32::from_le_bytes(a[i..i + 4].try_into().unwrap()), u32::from_le_bytes(b[i..i + 4].try_into().unwrap())),
            Chan::F16 => {
                let (x, y) = (half_to_f32(u16::from_le_bytes([a[i], a[i + 1]])), half_to_f32(u16::from_le_bytes([b[i], b[i + 1]])));
                (x.is_nan() && y.is_nan()) || x == y || (x - y).abs() <= 2e-3 * x.abs().max(y.abs()).max(1.0)
            }
            Chan::Norm8 => (a[i] as i32 - b[i] as i32).abs() <= 1,
            Chan::Norm16 => (u16::from_le_bytes([a[i], a[i + 1]]) as i32 - u16::from_le_bytes([b[i], b[i + 1]]) as i32).abs() <= 1,
            _ => a[i..i + unit] == b[i..i + unit],
        };
        if !ok { mism += 1; if first.is_none() { first = Some(i); } }
        i += unit;
    }
    (mism, first)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || (args.len() < 4 && args[1] != "--reflect-io") {
        eprintln!("usage: spv-diff a.spv b.spv entry [profile...]");
        std::process::exit(2);
    }
    if args[1] == "--reflect-io" {
        let m = rspirv::dr::load_bytes(&std::fs::read(&args[2]).unwrap()).expect("parse");
        match reflect(&m, "") {
            Ok(r) => {
                for (l, k, n, f) in &r.ins { println!("IN {} {} {} {}", l, k, n, *f as u8); }
                for (l, k, n) in &r.outs { println!("OUT {} {} {}", l, k, n); }
            }
            Err(e) => println!("SKIP {e}"),
        }
        return;
    }
    if args[1] == "--vs" {
        let (ab, bb) = (std::fs::read(&args[2]).unwrap(), std::fs::read(&args[3]).unwrap());
        let profiles: Vec<u32> = if args.len() > 5 { args[5..].iter().map(|s| s.parse().unwrap()).collect() } else { vec![0, 1, 2, 3] };
        let (ca, cb) = match (rewrite_vs(&ab), rewrite_vs(&bb)) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(e), _) | (_, Err(e)) => { println!("SKIP rewrite: {e}"); return; }
        };
        if let Ok(dir) = std::env::var("SPVDIFF_DUMP") { std::fs::write(format!("{dir}/a.rewritten.spv"), &ca.spirv).ok(); std::fs::write(format!("{dir}/b.rewritten.spv"), &cb.spirv).ok(); }
        if ca.kinds != cb.kinds { println!("IFACE_MISMATCH a={:?} b={:?}", ca.kinds, cb.kinds); println!("DONE"); return; }
        let ma = rspirv::dr::load_bytes(&ab).expect("parse a");
        let mut refl = match reflect(&ma, &args[4]) { Ok(r) => r, Err(e) => { println!("SKIP {e}"); return; } };
        refl.res.push(Res { addr_offsets: vec![], shape: Shape { d3: false, layers: 1, arrayed: false, ms: false, count: 1 }, set: CAPTURE_SET, binding: 0, name: "__capture".into(), kind: Kind::Ssbo });
        let gpu = Gpu::new();
        let vsm = VsMode { ins: refl.ins.iter().map(|x| (x.0, x.1, x.2, x.3)).collect(), kinds: ca.kinds.clone(), stride: ca.stride };
        let (xa, xb) = if std::env::var("SPVDIFF_NOREWRITE").is_ok() { (ab.clone(), bb.clone()) } else { (ca.spirv.clone(), cb.spirv.clone()) };
        unsafe { run(&gpu, &xa, &xb, &args[4], &refl, &profiles, None, Some(&vsm)) }
        println!("DONE");
        return;
    }
    let (gfx_vs, args) = if args[1] == "--gfx" {
        let vs = std::fs::read(&args[2]).unwrap();
        let mut a2 = vec![args[0].clone()];
        a2.extend_from_slice(&args[3..]);
        (Some(vs), a2)
    } else { (None, args) };
    let a_bytes = std::fs::read(&args[1]).unwrap();
    let b_bytes = std::fs::read(&args[2]).unwrap();
    let entry = args[3].clone();
    let profiles: Vec<u32> = if args.len() > 4 { args[4..].iter().map(|s| s.parse().unwrap()).collect() } else { vec![0, 1, 2, 3] };
    let ma = rspirv::dr::load_bytes(&a_bytes).expect("parse a");
    let refl = match reflect(&ma, &entry) { Ok(r) => r, Err(e) => { println!("SKIP {e}"); return; } };
    if std::env::var("SPVDIFF_TRACE").is_ok() { eprintln!("reflected {} resources", refl.res.len()); }
    let gpu = Gpu::new();
    let gfx = gfx_vs.map(|vs| Gfx { vs, outs: refl.outs.clone() });
    unsafe { run(&gpu, &a_bytes, &b_bytes, &entry, &refl, &profiles, gfx.as_ref(), None) }
    println!("DONE");
}

struct VsMode {
    ins: Vec<(u32, char, u32, bool)>,
    kinds: Vec<(u32, char)>,
    stride: u32,
}

const VS_VERTS: u32 = 64;

struct Gfx {
    vs: Vec<u8>,
    outs: Vec<(u32, char, u32)>,
}

unsafe fn run(gpu: &Gpu, a: &[u8], b: &[u8], entry: &str, refl: &Reflection, profiles: &[u32], gfx: Option<&Gfx>, vsm: Option<&VsMode>) {
    let stages = if vsm.is_some() { vk::ShaderStageFlags::VERTEX } else if gfx.is_some() { vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT } else { vk::ShaderStageFlags::COMPUTE };
    let d = &gpu.dev;
    let max_set = refl.res.iter().map(|r| r.set).max().unwrap_or(0);
    let mut layouts = Vec::new();
    for s in 0..=max_set {
        let binds: Vec<_> = refl.res.iter().filter(|r| r.set == s).map(|r| {
            let ty = match r.kind {
                Kind::Sampler => vk::DescriptorType::SAMPLER,
                Kind::Sampled(_) => vk::DescriptorType::SAMPLED_IMAGE,
                Kind::Storage(_) => vk::DescriptorType::STORAGE_IMAGE,
                Kind::Ubo => vk::DescriptorType::UNIFORM_BUFFER,
                Kind::Ssbo => vk::DescriptorType::STORAGE_BUFFER,
            };
            vk::DescriptorSetLayoutBinding::default().binding(r.binding).descriptor_type(ty).descriptor_count(r.shape.count).stage_flags(stages)
        }).collect();
        layouts.push(d.create_descriptor_set_layout(&vk::DescriptorSetLayoutCreateInfo::default().bindings(&binds), None).unwrap());
    }
    let pcr = [vk::PushConstantRange::default().stage_flags(stages).offset(0).size(PUSH_BYTES)];
    let pl = d.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().set_layouts(&layouts).push_constant_ranges(&pcr), None).unwrap();
    let sizes = [
        vk::DescriptorPoolSize { ty: vk::DescriptorType::SAMPLER, descriptor_count: 16384 },
        vk::DescriptorPoolSize { ty: vk::DescriptorType::SAMPLED_IMAGE, descriptor_count: 16384 },
        vk::DescriptorPoolSize { ty: vk::DescriptorType::STORAGE_IMAGE, descriptor_count: 16384 },
        vk::DescriptorPoolSize { ty: vk::DescriptorType::UNIFORM_BUFFER, descriptor_count: 16384 },
        vk::DescriptorPoolSize { ty: vk::DescriptorType::STORAGE_BUFFER, descriptor_count: 16384 },
    ];
    let dpool = d.create_descriptor_pool(&vk::DescriptorPoolCreateInfo::default().max_sets(16).pool_sizes(&sizes), None).unwrap();
    let sets = d.allocate_descriptor_sets(&vk::DescriptorSetAllocateInfo::default().descriptor_pool(dpool).set_layouts(&layouts)).unwrap();

    // resources
    struct Live { res: Res, buf: Option<(vk::Buffer, *mut u8)>, img: Option<(vk::Image, vk::ImageView, vk::Buffer, *mut u8)>, region: vk::BufferImageCopy }
    let mut live: Vec<Live> = Vec::new();
    let mut infos_buf: Vec<(usize, vk::DescriptorBufferInfo)> = Vec::new();
    let mut infos_img: Vec<(usize, Vec<vk::DescriptorImageInfo>)> = Vec::new();
    for r in &refl.res {
        let mut l = Live { res: r.clone(), buf: None, img: None, region: vk::BufferImageCopy::default() };
        match r.kind {
            Kind::Ubo | Kind::Ssbo => { let (b, _m, p) = gpu.buffer(BUF_BYTES); l.buf = Some((b, p)); infos_buf.push((live.len(), vk::DescriptorBufferInfo { buffer: b, offset: 0, range: BUF_BYTES as u64 })); }
            Kind::Sampled(f) | Kind::Storage(f) => {
                let (img, view) = gpu.image(f.vk, r.shape);
                let (sb, _sm, sp) = gpu.buffer(IMG_DIM as usize * IMG_DIM as usize * f.texel * r.shape.layers as usize);
                l.img = Some((img, view, sb, sp));
                l.region = vk::BufferImageCopy::default()
                    .image_subresource(vk::ImageSubresourceLayers { aspect_mask: vk::ImageAspectFlags::COLOR, mip_level: 0, base_array_layer: 0, layer_count: if r.shape.arrayed { r.shape.layers } else { 1 } })
                    .image_extent(vk::Extent3D { width: IMG_DIM, height: IMG_DIM, depth: if r.shape.d3 { r.shape.layers } else { 1 } });
                infos_img.push((live.len(), vec![vk::DescriptorImageInfo { sampler: vk::Sampler::null(), image_view: view, image_layout: vk::ImageLayout::GENERAL }; r.shape.count as usize]));
            }
            Kind::Sampler => {
                let n = r.name.to_lowercase();
                let filt = if n.contains("nn") || n.contains("nearest") { vk::Filter::NEAREST } else { vk::Filter::LINEAR };
                let mut sci = vk::SamplerCreateInfo::default().mag_filter(filt).min_filter(filt)
                    .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE).address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE).address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE).max_lod(16.0);
                if n.contains("shadow") { sci = sci.compare_enable(true).compare_op(vk::CompareOp::LESS_OR_EQUAL); }
                let s = d.create_sampler(&sci, None).unwrap();
                infos_img.push((live.len(), vec![vk::DescriptorImageInfo { sampler: s, image_view: vk::ImageView::null(), image_layout: vk::ImageLayout::UNDEFINED }; r.shape.count as usize]));
            }
        }
        live.push(l);
    }
    let vbufs: Vec<(u32, vk::Buffer, *mut u8, char)> = vsm.map(|v| v.ins.iter().map(|(l, k, _, _)| { let (b, _m, p) = gpu.buffer(16 * VS_VERTS as usize); (*l, b, p, *k) }).collect()).unwrap_or_default();
    let (phys_buf, _pm, phys_ptr) = gpu.buffer(BUF_BYTES);
    let phys_addr = d.get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(phys_buf));
    let mut writes = Vec::new();
    for (i, bi) in &infos_buf {
        let r = &live[*i].res;
        writes.push(vk::WriteDescriptorSet::default().dst_set(sets[r.set as usize]).dst_binding(r.binding)
            .descriptor_type(if matches!(r.kind, Kind::Ubo) { vk::DescriptorType::UNIFORM_BUFFER } else { vk::DescriptorType::STORAGE_BUFFER })
            .buffer_info(std::slice::from_ref(bi)));
    }
    for (i, ii) in &infos_img {
        let r = &live[*i].res;
        let ty = match r.kind { Kind::Sampler => vk::DescriptorType::SAMPLER, Kind::Sampled(_) => vk::DescriptorType::SAMPLED_IMAGE, _ => vk::DescriptorType::STORAGE_IMAGE };
        writes.push(vk::WriteDescriptorSet::default().dst_set(sets[r.set as usize]).dst_binding(r.binding).descriptor_type(ty).image_info(ii));
    }
    d.update_descriptor_sets(&writes, &[]);

    let rt_fmts: Vec<vk::Format> = gfx.map(|g| g.outs.iter().map(|(_, k, _)| match k { 'u' => vk::Format::R32G32B32A32_UINT, 'i' => vk::Format::R32G32B32A32_SINT, _ => vk::Format::R32G32B32A32_SFLOAT }).collect()).unwrap_or_default();
    let mk_module = |bytes: &[u8]| {
        let words: Vec<u32> = bytes.chunks(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect();
        d.create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&words), None).expect("shader module")
    };
    let mk_pipe = |bytes: &[u8]| -> vk::Pipeline {
        if let Some(v) = vsm {
            let vsm_mod = mk_module(bytes);
            let vn = CString::new("vs_main").unwrap();
            let stages_ci = [vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(vsm_mod).name(&vn)];
            let binds: Vec<_> = v.ins.iter().map(|(l, _, _, _)| vk::VertexInputBindingDescription { binding: *l, stride: 16, input_rate: vk::VertexInputRate::VERTEX }).collect();
            let attrs: Vec<_> = v.ins.iter().map(|(l, k, _, _)| vk::VertexInputAttributeDescription { location: *l, binding: *l, format: match k { 'u' => vk::Format::R32G32B32A32_UINT, 'i' => vk::Format::R32G32B32A32_SINT, _ => vk::Format::R32G32B32A32_SFLOAT }, offset: 0 }).collect();
            let vi = vk::PipelineVertexInputStateCreateInfo::default().vertex_binding_descriptions(&binds).vertex_attribute_descriptions(&attrs);
            let ia = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::POINT_LIST);
            let vp = [vk::Viewport { x: 0.0, y: 0.0, width: IMG_DIM as f32, height: IMG_DIM as f32, min_depth: 0.0, max_depth: 1.0 }];
            let sc = [vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: vk::Extent2D { width: IMG_DIM, height: IMG_DIM } }];
            let vps = vk::PipelineViewportStateCreateInfo::default().viewports(&vp).scissors(&sc);
            let rs = vk::PipelineRasterizationStateCreateInfo::default().rasterizer_discard_enable(true).polygon_mode(vk::PolygonMode::FILL).line_width(1.0);
            let ms = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
            let mut rendering = vk::PipelineRenderingCreateInfo::default();
            let ci = vk::GraphicsPipelineCreateInfo::default().stages(&stages_ci).vertex_input_state(&vi).input_assembly_state(&ia)
                .viewport_state(&vps).rasterization_state(&rs).multisample_state(&ms).layout(pl).push_next(&mut rendering);
            return d.create_graphics_pipelines(vk::PipelineCache::null(), &[ci], None).map_err(|e| e.1).expect("vs pipeline")[0];
        }
        if let Some(g) = gfx {
            let (vsm, fsm) = (mk_module(&g.vs), mk_module(bytes));
            let (vn, fnm) = (CString::new("vs_main").unwrap(), CString::new(entry).unwrap());
            let stages_ci = [
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(vsm).name(&vn),
                vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(fsm).name(&fnm),
            ];
            let vi = vk::PipelineVertexInputStateCreateInfo::default();
            let ia = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
            let vp = [vk::Viewport { x: 0.0, y: 0.0, width: IMG_DIM as f32, height: IMG_DIM as f32, min_depth: 0.0, max_depth: 1.0 }];
            let sc = [vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: vk::Extent2D { width: IMG_DIM, height: IMG_DIM } }];
            let vps = vk::PipelineViewportStateCreateInfo::default().viewports(&vp).scissors(&sc);
            let rs = vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL).cull_mode(vk::CullModeFlags::NONE).line_width(1.0);
            let ms = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_1);
            let ds = vk::PipelineDepthStencilStateCreateInfo::default().depth_test_enable(true).depth_write_enable(true).depth_compare_op(vk::CompareOp::ALWAYS);
            let atts: Vec<_> = rt_fmts.iter().map(|_| vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA)).collect();
            let cb = vk::PipelineColorBlendStateCreateInfo::default().attachments(&atts);
            let mut rendering = vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&rt_fmts).depth_attachment_format(vk::Format::D32_SFLOAT);
            let ci = vk::GraphicsPipelineCreateInfo::default().stages(&stages_ci).vertex_input_state(&vi).input_assembly_state(&ia)
                .viewport_state(&vps).rasterization_state(&rs).multisample_state(&ms).depth_stencil_state(&ds).color_blend_state(&cb)
                .layout(pl).push_next(&mut rendering);
            return d.create_graphics_pipelines(vk::PipelineCache::null(), &[ci], None).map_err(|e| e.1).expect("graphics pipeline")[0];
        }
        let sm = mk_module(bytes);
        let name = CString::new(entry).unwrap();
        let stage = vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::COMPUTE).module(sm).name(&name);
        d.create_compute_pipelines(vk::PipelineCache::null(), &[vk::ComputePipelineCreateInfo::default().stage(stage).layout(pl)], None).map_err(|e| e.1).expect("pipeline")[0]
    };
    if std::env::var("SPVDIFF_TRACE").is_ok() { eprintln!("resources ready, building pipelines"); }
    let (pa, pb) = (mk_pipe(a), mk_pipe(b));
    if std::env::var("SPVDIFF_TRACE").is_ok() { eprintln!("pipelines built"); }
    let cmd = d.allocate_command_buffers(&vk::CommandBufferAllocateInfo::default().command_pool(gpu.pool).level(vk::CommandBufferLevel::PRIMARY).command_buffer_count(1)).unwrap()[0];
    let fence = d.create_fence(&vk::FenceCreateInfo::default(), None).unwrap();
    let rts: Vec<(vk::Image, vk::ImageView, vk::Buffer, *mut u8, usize)> = rt_fmts.iter().map(|f| {
        let texel = 16;
        let img = d.create_image(&vk::ImageCreateInfo::default().image_type(vk::ImageType::TYPE_2D).format(*f)
            .extent(vk::Extent3D { width: IMG_DIM, height: IMG_DIM, depth: 1 }).mip_levels(1).array_layers(1).samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL).usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC).initial_layout(vk::ImageLayout::UNDEFINED), None).expect("rt image");
        let req = d.get_image_memory_requirements(img);
        let mem = d.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(gpu.mem_type(req.memory_type_bits, vk::MemoryPropertyFlags::empty())), None).unwrap();
        d.bind_image_memory(img, mem, 0).unwrap();
        let view = d.create_image_view(&vk::ImageViewCreateInfo::default().image(img).view_type(vk::ImageViewType::TYPE_2D).format(*f)
            .subresource_range(vk::ImageSubresourceRange { aspect_mask: vk::ImageAspectFlags::COLOR, base_mip_level: 0, level_count: 1, base_array_layer: 0, layer_count: 1 }), None).unwrap();
        let (sb, _sm, sp) = gpu.buffer(IMG_DIM as usize * IMG_DIM as usize * texel);
        (img, view, sb, sp, IMG_DIM as usize * IMG_DIM as usize * texel)
    }).collect();
    let depth: Option<(vk::Image, vk::ImageView, vk::Buffer, *mut u8)> = if gfx.is_some() {
        let img = d.create_image(&vk::ImageCreateInfo::default().image_type(vk::ImageType::TYPE_2D).format(vk::Format::D32_SFLOAT)
            .extent(vk::Extent3D { width: IMG_DIM, height: IMG_DIM, depth: 1 }).mip_levels(1).array_layers(1).samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL).usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC).initial_layout(vk::ImageLayout::UNDEFINED), None).expect("depth image");
        let req = d.get_image_memory_requirements(img);
        let mem = d.allocate_memory(&vk::MemoryAllocateInfo::default().allocation_size(req.size).memory_type_index(gpu.mem_type(req.memory_type_bits, vk::MemoryPropertyFlags::empty())), None).unwrap();
        d.bind_image_memory(img, mem, 0).unwrap();
        let view = d.create_image_view(&vk::ImageViewCreateInfo::default().image(img).view_type(vk::ImageViewType::TYPE_2D).format(vk::Format::D32_SFLOAT)
            .subresource_range(vk::ImageSubresourceRange { aspect_mask: vk::ImageAspectFlags::DEPTH, base_mip_level: 0, level_count: 1, base_array_layer: 0, layer_count: 1 }), None).unwrap();
        let (sb, _sm, sp) = gpu.buffer(IMG_DIM as usize * IMG_DIM as usize * 4);
        Some((img, view, sb, sp))
    } else { None };
    let drange = vk::ImageSubresourceRange { aspect_mask: vk::ImageAspectFlags::DEPTH, base_mip_level: 0, level_count: 1, base_array_layer: 0, layer_count: 1 };
    let dregion = vk::BufferImageCopy::default().image_subresource(vk::ImageSubresourceLayers { aspect_mask: vk::ImageAspectFlags::DEPTH, mip_level: 0, base_array_layer: 0, layer_count: 1 }).image_extent(vk::Extent3D { width: IMG_DIM, height: IMG_DIM, depth: 1 });
    let dbarrier = |from: vk::ImageLayout, to: vk::ImageLayout| {
        vk::ImageMemoryBarrier::default().image(depth.unwrap().0).old_layout(from).new_layout(to).subresource_range(drange)
            .src_access_mask(vk::AccessFlags::MEMORY_WRITE | vk::AccessFlags::MEMORY_READ).dst_access_mask(vk::AccessFlags::MEMORY_WRITE | vk::AccessFlags::MEMORY_READ)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED).dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    };
    let rng_base = 0x9E3779B97F4A7C15u64;
    // 1-D shaders with groupshared memory (prefix sums) index it by the *global* id: one group only.
    let (gx, gy) = if refl.local[1] == 1 && refl.has_shared { (1, 1) } else if refl.local[1] == 1 { ((4096 + refl.local[0] - 1) / refl.local[0], 1) } else { ((64 + refl.local[0] - 1) / refl.local[0], (64 + refl.local[1] - 1) / refl.local[1]) };

    let submit = |f: &dyn Fn(vk::CommandBuffer)| {
        d.reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty()).unwrap();
        d.begin_command_buffer(cmd, &vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)).unwrap();
        f(cmd);
        d.end_command_buffer(cmd).unwrap();
        d.reset_fences(&[fence]).unwrap();
        let cb = [cmd];
        d.queue_submit(gpu.queue, &[vk::SubmitInfo::default().command_buffers(&cb)], fence).unwrap();
        d.wait_for_fences(&[fence], true, u64::MAX).unwrap();
    };
    let range = vk::ImageSubresourceRange { aspect_mask: vk::ImageAspectFlags::COLOR, base_mip_level: 0, level_count: 1, base_array_layer: 0, layer_count: vk::REMAINING_ARRAY_LAYERS };
    let layers = vk::ImageSubresourceLayers { aspect_mask: vk::ImageAspectFlags::COLOR, mip_level: 0, base_array_layer: 0, layer_count: 1 };
    let region = vk::BufferImageCopy::default().image_subresource(layers).image_extent(vk::Extent3D { width: IMG_DIM, height: IMG_DIM, depth: 1 });
    let barrier = |img: vk::Image, from: vk::ImageLayout, to: vk::ImageLayout| {
        vk::ImageMemoryBarrier::default().image(img).old_layout(from).new_layout(to).subresource_range(range)
            .src_access_mask(vk::AccessFlags::MEMORY_WRITE | vk::AccessFlags::MEMORY_READ).dst_access_mask(vk::AccessFlags::MEMORY_WRITE | vk::AccessFlags::MEMORY_READ)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED).dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
    };

    let any_ms = live.iter().any(|l| l.res.shape.ms);
    let msfill: Option<(vk::Pipeline, vk::Pipeline, vk::PipelineLayout)> = if any_ms {
        let (vp, fp, up) = (std::env::var("SPVDIFF_MS_VS"), std::env::var("SPVDIFF_MS_FS_F"), std::env::var("SPVDIFF_MS_FS_U"));
        let (vp, fp, up) = match (vp, fp, up) { (Ok(a), Ok(b), Ok(c)) => (a, b, c), _ => { println!("SKIP multisampled input needs SPVDIFF_MS_VS/_FS_F/_FS_U"); return; } };
        let fpl = d.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default().push_constant_ranges(&[vk::PushConstantRange::default().stage_flags(vk::ShaderStageFlags::FRAGMENT).offset(0).size(4)]), None).unwrap();
        let vsm_mod = mk_module(&std::fs::read(vp).unwrap());
        let mk = |fs_path: &str, fmt: vk::Format, entry_name: &str| -> vk::Pipeline {
            let fsm = mk_module(&std::fs::read(fs_path).unwrap());
            let (vn, en) = (CString::new("vs_main").unwrap(), CString::new(entry_name).unwrap());
            let st = [vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::VERTEX).module(vsm_mod).name(&vn), vk::PipelineShaderStageCreateInfo::default().stage(vk::ShaderStageFlags::FRAGMENT).module(fsm).name(&en)];
            let vi = vk::PipelineVertexInputStateCreateInfo::default();
            let ia = vk::PipelineInputAssemblyStateCreateInfo::default().topology(vk::PrimitiveTopology::TRIANGLE_LIST);
            let vp_ = [vk::Viewport { x: 0.0, y: 0.0, width: IMG_DIM as f32, height: IMG_DIM as f32, min_depth: 0.0, max_depth: 1.0 }];
            let sc = [vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: vk::Extent2D { width: IMG_DIM, height: IMG_DIM } }];
            let vps = vk::PipelineViewportStateCreateInfo::default().viewports(&vp_).scissors(&sc);
            let rs = vk::PipelineRasterizationStateCreateInfo::default().polygon_mode(vk::PolygonMode::FILL).cull_mode(vk::CullModeFlags::NONE).line_width(1.0);
            let ms = vk::PipelineMultisampleStateCreateInfo::default().rasterization_samples(vk::SampleCountFlags::TYPE_4).sample_shading_enable(true).min_sample_shading(1.0);
            let atts = [vk::PipelineColorBlendAttachmentState::default().color_write_mask(vk::ColorComponentFlags::RGBA)];
            let cb = vk::PipelineColorBlendStateCreateInfo::default().attachments(&atts);
            let fm = [fmt];
            let mut rendering = vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&fm);
            let ci = vk::GraphicsPipelineCreateInfo::default().stages(&st).vertex_input_state(&vi).input_assembly_state(&ia).viewport_state(&vps).rasterization_state(&rs).multisample_state(&ms).color_blend_state(&cb).layout(fpl).push_next(&mut rendering);
            d.create_graphics_pipelines(vk::PipelineCache::null(), &[ci], None).map_err(|e| e.1).expect("ms fill pipeline")[0]
        };
        Some((mk(&fp, vk::Format::R32G32B32A32_SFLOAT, "fs_f"), mk(&up, vk::Format::R32G32B32A32_UINT, "fs_u"), fpl))
    } else { None };

    for &profile in profiles {
        let mut init: Vec<Vec<u8>> = Vec::new();
        let mut seed = rng_base ^ (profile as u64 * 0x1234567);
        for l in &live {
            let (len, chan) = match l.res.kind { Kind::Ubo | Kind::Ssbo => (BUF_BYTES, Chan::F32), Kind::Sampled(f) | Kind::Storage(f) => (IMG_DIM as usize * IMG_DIM as usize * f.texel * l.res.shape.layers as usize, f.chan), Kind::Sampler => { init.push(Vec::new()); continue } };
            let mut bytes = vec![0u8; len];
            let c = if matches!(l.res.kind, Kind::Ubo | Kind::Ssbo) { buffer_chan(profile) } else { chan };
            fill_bytes(c, &mut bytes, profile, &mut seed);
            for (o, stride) in &l.res.addr_offsets {
                let mut pos = *o as usize;
                while pos + 8 <= bytes.len() {
                    bytes[pos..pos + 8].copy_from_slice(&phys_addr.to_le_bytes());
                    if *stride == 0 { break; }
                    pos += *stride as usize;
                }
            }
            init.push(bytes);
        }
        for (_, _, p, k) in &vbufs {
            let mut vb = vec![0u8; 16 * VS_VERTS as usize];
            fill_bytes(if *k == 'f' { Chan::F32 } else { Chan::Int32 }, &mut vb, profile, &mut seed);
            std::ptr::copy_nonoverlapping(vb.as_ptr(), *p, vb.len());
        }
        let mut pbytes = vec![0u8; BUF_BYTES];
        fill_bytes(Chan::F32, &mut pbytes, 0, &mut seed);
        let mut push = [0u8; PUSH_BYTES as usize];
        for w in 0..(PUSH_BYTES as usize / 4) {
            let (r, r2) = (rnd01(&mut seed), rnd01(&mut seed));
            let g = gen_word(profile, w, r, r2).1;
            // push constants are mostly counts/offsets: keep loop bounds finite
            let v = match profile % 16 { 7 => 0, 8 => 1, 9 => 65535, 6 | 13 => g % 64, _ => 1 + g % 63 };
            push[w * 4..w * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        for o in &refl.push_addr_offsets { push[*o as usize..*o as usize + 8].copy_from_slice(&phys_addr.to_le_bytes()); }

        let mut outs: Vec<Vec<Vec<u8>>> = Vec::new(); // [run][binding]
        for run in 0..2 {
            std::ptr::copy_nonoverlapping(pbytes.as_ptr(), phys_ptr, BUF_BYTES);
            for (i, l) in live.iter().enumerate() {
                if let Some((_, p)) = l.buf { std::ptr::copy_nonoverlapping(init[i].as_ptr(), p, BUF_BYTES); }
                if let Some((_, _, _, p)) = l.img { std::ptr::copy_nonoverlapping(init[i].as_ptr(), p, init[i].len()); }
            }
            if let Some(ms) = &msfill {
                let prof = profile;
                submit(&|c| {
                    for l in &live {
                        if let (Some((img, view, _, _)), true) = (l.img, l.res.shape.ms) {
                            let uint = match l.res.kind { Kind::Sampled(f) => matches!(f.vk, vk::Format::R32G32B32A32_UINT | vk::Format::R32G32B32A32_SINT), _ => false };
                            d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[barrier(img, vk::ImageLayout::UNDEFINED, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)]);
                            let att = [vk::RenderingAttachmentInfo::default().image_view(view).image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL).load_op(vk::AttachmentLoadOp::CLEAR).store_op(vk::AttachmentStoreOp::STORE).clear_value(vk::ClearValue { color: vk::ClearColorValue { uint32: [0; 4] } })];
                            d.cmd_begin_rendering(c, &vk::RenderingInfo::default().render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: vk::Extent2D { width: IMG_DIM, height: IMG_DIM } }).layer_count(1).color_attachments(&att));
                            d.cmd_bind_pipeline(c, vk::PipelineBindPoint::GRAPHICS, if uint { ms.1 } else { ms.0 });
                            d.cmd_push_constants(c, ms.2, vk::ShaderStageFlags::FRAGMENT, 0, &prof.to_le_bytes());
                            d.cmd_draw(c, 3, 1, 0, 0);
                            d.cmd_end_rendering(c);
                            d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[barrier(img, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, vk::ImageLayout::GENERAL)]);
                        }
                    }
                });
            }
            submit(&|c| {
                for l in &live {
                    if let (Some((img, _, sb, _)), false) = (l.img, l.res.shape.ms) {
                        d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[barrier(img, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL)]);
                        d.cmd_copy_buffer_to_image(c, sb, img, vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[l.region]);
                        d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[barrier(img, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::GENERAL)]);
                    }
                }
            });
            submit(&|c| {
                let bp = if gfx.is_some() || vsm.is_some() { vk::PipelineBindPoint::GRAPHICS } else { vk::PipelineBindPoint::COMPUTE };
                if gfx.is_some() {
                    for rt in &rts {
                        d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[barrier(rt.0, vk::ImageLayout::UNDEFINED, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)]);
                    }
                    let atts: Vec<_> = rts.iter().map(|rt| vk::RenderingAttachmentInfo::default().image_view(rt.1).image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                        .load_op(vk::AttachmentLoadOp::CLEAR).store_op(vk::AttachmentStoreOp::STORE).clear_value(vk::ClearValue { color: vk::ClearColorValue { uint32: [0; 4] } })).collect();
                    d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[dbarrier(vk::ImageLayout::UNDEFINED, vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)]);
                    let datt = vk::RenderingAttachmentInfo::default().image_view(depth.unwrap().1).image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
                        .load_op(vk::AttachmentLoadOp::CLEAR).store_op(vk::AttachmentStoreOp::STORE).clear_value(vk::ClearValue { depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 } });
                    let ri = vk::RenderingInfo::default().render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: vk::Extent2D { width: IMG_DIM, height: IMG_DIM } }).layer_count(1).color_attachments(&atts).depth_attachment(&datt);

                    d.cmd_begin_rendering(c, &ri);
                }
                if vsm.is_some() {
                    d.cmd_begin_rendering(c, &vk::RenderingInfo::default().render_area(vk::Rect2D { offset: vk::Offset2D { x: 0, y: 0 }, extent: vk::Extent2D { width: IMG_DIM, height: IMG_DIM } }).layer_count(1));
                }
                d.cmd_bind_pipeline(c, bp, if run == 0 { pa } else { pb });
                d.cmd_bind_descriptor_sets(c, bp, pl, 0, &sets, &[]);
                d.cmd_push_constants(c, pl, stages, 0, &push);
                if vsm.is_some() {
                    for (l, b, _, _) in &vbufs { d.cmd_bind_vertex_buffers(c, *l, &[*b], &[0]); }
                    d.cmd_draw(c, VS_VERTS, 1, 0, 0);
                    d.cmd_end_rendering(c);
                } else if gfx.is_some() {
                    d.cmd_draw(c, 3, 1, 0, 0);
                    d.cmd_end_rendering(c);
                    d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[dbarrier(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL, vk::ImageLayout::TRANSFER_SRC_OPTIMAL)]);
                    d.cmd_copy_image_to_buffer(c, depth.unwrap().0, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, depth.unwrap().2, &[dregion]);
                    for rt in &rts {
                        d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::ALL_COMMANDS, vk::DependencyFlags::empty(), &[], &[], &[barrier(rt.0, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, vk::ImageLayout::TRANSFER_SRC_OPTIMAL)]);
                        d.cmd_copy_image_to_buffer(c, rt.0, vk::ImageLayout::TRANSFER_SRC_OPTIMAL, rt.2, &[region]);
                    }
                } else {
                    d.cmd_dispatch(c, gx, gy, 1);
                }
                let mb = [vk::MemoryBarrier::default().src_access_mask(vk::AccessFlags::MEMORY_WRITE).dst_access_mask(vk::AccessFlags::MEMORY_READ | vk::AccessFlags::TRANSFER_READ)];
                d.cmd_pipeline_barrier(c, vk::PipelineStageFlags::ALL_COMMANDS, vk::PipelineStageFlags::TRANSFER | vk::PipelineStageFlags::HOST, vk::DependencyFlags::empty(), &mb, &[], &[]);
                for l in &live {
                    if let (Some((img, _, sb, _)), false) = (l.img, l.res.shape.ms) { d.cmd_copy_image_to_buffer(c, img, vk::ImageLayout::GENERAL, sb, &[l.region]); }
                }
            });
            let mut o = Vec::new();
            for (i, l) in live.iter().enumerate() {
                if let Some((_, p)) = l.buf { o.push(std::slice::from_raw_parts(p, BUF_BYTES).to_vec()); }
                else if let Some((_, _, _, p)) = l.img { o.push(std::slice::from_raw_parts(p, init[i].len()).to_vec()); }
                else { o.push(Vec::new()); }
            }
            for rt in &rts { o.push(std::slice::from_raw_parts(rt.3, rt.4).to_vec()); }
            if let Some(dp) = &depth { o.push(std::slice::from_raw_parts(dp.3, IMG_DIM as usize * IMG_DIM as usize * 4).to_vec()); }
            outs.push(o);
        }
        for (k, rt) in rts.iter().enumerate() {
            let i = live.len() + k;
            let chan = if gfx.unwrap().outs[k].1 == 'f' { Chan::F32 } else { Chan::Int32 };
            let (mism, first) = compare(chan, &outs[0][i], &outs[1][i]);
            let nz = |o: &Vec<u8>| o.iter().filter(|x| **x != 0).count();
            let fs = first.map(|o| { let w = |v: &Vec<u8>| u32::from_le_bytes(v[o..o + 4].try_into().unwrap()); format!("px{}:{}:{}", o / 16, f32::from_bits(w(&outs[0][i])), f32::from_bits(w(&outs[1][i]))) }).unwrap_or_default();
            println!("RES profile={} set=9 binding={} name=rt{} kind=rt mismatch={} changed_a={} changed_b={} first={}", profile, gfx.unwrap().outs[k].0, gfx.unwrap().outs[k].0, mism, nz(&outs[0][i]), nz(&outs[1][i]), fs);
            let _ = rt;
        }
        if depth.is_some() {
            let i = live.len() + rts.len();
            let (mism, first) = compare(Chan::F32, &outs[0][i], &outs[1][i]);
            let ch = |o: &Vec<u8>| o.chunks(4).filter(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]) != 0x3F800000).count();
            let fs = first.map(|o| { let w = |v: &Vec<u8>| u32::from_le_bytes(v[o..o + 4].try_into().unwrap()); format!("px{}:{}:{}", o / 4, f32::from_bits(w(&outs[0][i])), f32::from_bits(w(&outs[1][i]))) }).unwrap_or_default();
            println!("RES profile={} set=9 binding=99 name=depth kind=depth mismatch={} changed_a={} changed_b={} first={}", profile, mism, ch(&outs[0][i]), ch(&outs[1][i]), fs);
        }
        for (i, l) in live.iter().enumerate() {
            let writable = matches!(l.res.kind, Kind::Ssbo | Kind::Storage(_));
            if !writable { continue; }
            let chan = match l.res.kind { Kind::Storage(f) => f.chan, _ => Chan::Int32 };
            let (mism, first) = if let (Some(v), "__capture") = (vsm, l.res.name.as_str()) {
                compare_cap(&v.kinds, v.stride as usize, VS_VERTS as usize, &outs[0][i], &outs[1][i])
            } else { compare(if matches!(l.res.kind, Kind::Ssbo) { Chan::F32 } else { chan }, &outs[0][i], &outs[1][i]) };
            let ch_a = outs[0][i].iter().zip(&init[i]).filter(|(x, y)| x != y).count();
            let ch_b = outs[1][i].iter().zip(&init[i]).filter(|(x, y)| x != y).count();
            let fs = first.map(|o| { let w = |v: &Vec<u8>| if o + 4 <= v.len() { u32::from_le_bytes(v[o..o + 4].try_into().unwrap()) } else { 0 }; let (x, y) = (w(&outs[0][i]), w(&outs[1][i])); format!("{}:{:08x}/{:08x}({:e}/{:e})", o, x, y, f32::from_bits(x), f32::from_bits(y)) }).unwrap_or_default();
            println!("RES profile={} set={} binding={} name={} kind={} mismatch={} changed_a={} changed_b={} first={}", profile, l.res.set, l.res.binding, l.res.name,
                if matches!(l.res.kind, Kind::Ssbo) { "ssbo" } else { "image" }, mism, ch_a, ch_b, fs);
        }
    }
    let _ = (&gpu.instance, pa, pb);
}

// ---------------------------------------------------------------------------------------------
// Vertex-shader capture: rewrite a VS so that, right before every OpReturn of its entry function,
// all outputs (gl_Position + Location outputs) are bit-copied into a storage buffer at
// [gl_VertexIndex * stride + slot]. Both oracle and candidate get the same rewrite.

use rspirv::binary::Assemble;
use rspirv::spirv::{BuiltIn, Word};

pub struct Capture {
    pub spirv: Vec<u8>,
    pub kinds: Vec<(u32, char)>, // (sort key, kind f/u/i) per slot
    pub stride: u32,
}

fn rewrite_vs(bytes: &[u8]) -> Result<Capture, String> {
    let mut m = rspirv::dr::load_bytes(bytes).map_err(|e| format!("{e:?}"))?;
    let mut bound = m.header.as_ref().unwrap().bound;
    let mut fresh = || { let b = bound; bound += 1; b };
    let types: HashMap<u32, Instruction> = m.types_global_values.iter().filter_map(|i| i.result_id.map(|r| (r, i.clone()))).collect();
    // decorations
    let mut loc: HashMap<u32, u32> = HashMap::new();
    let mut builtin: HashMap<u32, u32> = HashMap::new();
    for a in &m.annotations {
        if a.class.opcode == Op::Decorate {
            let t = id_of(&a.operands[0]).unwrap();
            match a.operands[1] {
                Operand::Decoration(Decoration::Location) => { loc.insert(t, lit(&a.operands[2]).unwrap()); }
                Operand::Decoration(Decoration::BuiltIn) => { if let Operand::BuiltIn(b) = a.operands[2] { builtin.insert(t, b as u32); } }
                _ => {}
            }
        }
    }
    let entry_fn = m.entry_points.iter().find(|e| matches!(e.operands[0], Operand::ExecutionModel(rspirv::spirv::ExecutionModel::Vertex)))
        .and_then(|e| id_of(&e.operands[1])).ok_or("no vertex entry point")?;
    // outputs to capture
    struct Out { var: u32, ty: u32, key: u32, kind: char, comps: u32, elem: u32, rows: u32, coltype: u32 }
    let mut outs: Vec<Out> = Vec::new();
    let mut vertex_index_var = None;
    for v in &m.types_global_values {
        if v.class.opcode != Op::Variable { continue; }
        let sc = if let Operand::StorageClass(s) = v.operands[0] { s } else { continue };
        let vid = v.result_id.unwrap();
        if sc == StorageClass::Input && builtin.get(&vid) == Some(&(BuiltIn::VertexIndex as u32)) { vertex_index_var = Some(vid); }
        if sc != StorageClass::Output { continue; }
        let key = if let Some(b) = builtin.get(&vid) {
            if *b != BuiltIn::Position as u32 { continue; }
            0
        } else if let Some(l) = loc.get(&vid) { 1 + l } else { continue };
        let ptr = &types[&v.result_type.unwrap()];
        let pointee = id_of(&ptr.operands[1]).unwrap();
        let t = &types[&pointee];
        let (mut rows, mut coltype) = (0u32, 0u32);
        let (elem, comps) = match t.class.opcode {
            Op::TypeVector => (id_of(&t.operands[0]).unwrap(), lit(&t.operands[1]).unwrap()),
            Op::TypeMatrix => {
                coltype = id_of(&t.operands[0]).unwrap();
                let cv = &types[&coltype];
                rows = lit(&cv.operands[1]).unwrap();
                (id_of(&cv.operands[0]).unwrap(), lit(&t.operands[1]).unwrap() * rows)
            }
            Op::TypeFloat | Op::TypeInt => (pointee, 1),
            op => return Err(format!("unsupported output type {op:?} at key {key}")),
        };
        let et = &types[&elem];
        let kind = match et.class.opcode { Op::TypeFloat => 'f', Op::TypeInt => if lit(&et.operands[1]) == Some(1) { 'i' } else { 'u' }, _ => return Err("odd elem".into()) };
        if et.class.opcode == Op::TypeFloat && lit(&et.operands[0]) != Some(32) { return Err("non-32-bit output".into()); }
        outs.push(Out { var: vid, ty: pointee, key, kind, comps, elem, rows, coltype });
    }
    outs.sort_by_key(|o| o.key);
    // uint type
    let uint = m.types_global_values.iter().find(|i| i.class.opcode == Op::TypeInt && lit(&i.operands[0]) == Some(32) && lit(&i.operands[1]) == Some(0)).and_then(|i| i.result_id);
    let mut new_types: Vec<Instruction> = Vec::new();
    let uint = match uint { Some(u) => u, None => { let u = fresh(); new_types.push(Instruction::new(Op::TypeInt, None, Some(u), vec![Operand::LiteralInt32(32), Operand::LiteralInt32(0)])); u } };
    let stride: u32 = outs.iter().map(|o| o.comps).sum();
    let mut kinds = Vec::new();
    for o in &outs { for _ in 0..o.comps { kinds.push((o.key, o.kind)); } }
    // storage buffer + vertex index
    let (rarr, st, ptr_sb, ptr_uint_sb, ptr_in_uint, ssbo) = (fresh(), fresh(), fresh(), fresh(), fresh(), fresh());
    new_types.push(Instruction::new(Op::TypeRuntimeArray, None, Some(rarr), vec![Operand::IdRef(uint)]));
    new_types.push(Instruction::new(Op::TypeStruct, None, Some(st), vec![Operand::IdRef(rarr)]));
    new_types.push(Instruction::new(Op::TypePointer, None, Some(ptr_sb), vec![Operand::StorageClass(StorageClass::StorageBuffer), Operand::IdRef(st)]));
    new_types.push(Instruction::new(Op::TypePointer, None, Some(ptr_uint_sb), vec![Operand::StorageClass(StorageClass::StorageBuffer), Operand::IdRef(uint)]));
    new_types.push(Instruction::new(Op::Variable, Some(ptr_sb), Some(ssbo), vec![Operand::StorageClass(StorageClass::StorageBuffer)]));
    let vi_var = match vertex_index_var {
        Some(v) => v,
        None => {
            new_types.push(Instruction::new(Op::TypePointer, None, Some(ptr_in_uint), vec![Operand::StorageClass(StorageClass::Input), Operand::IdRef(uint)]));
            let v = fresh();
            new_types.push(Instruction::new(Op::Variable, Some(ptr_in_uint), Some(v), vec![Operand::StorageClass(StorageClass::Input)]));
            m.annotations.push(Instruction::new(Op::Decorate, None, None, vec![Operand::IdRef(v), Operand::Decoration(Decoration::BuiltIn), Operand::BuiltIn(BuiltIn::VertexIndex)]));
            for e in m.entry_points.iter_mut() { e.operands.push(Operand::IdRef(v)); }
            v
        }
    };
    m.annotations.push(Instruction::new(Op::Decorate, None, None, vec![Operand::IdRef(rarr), Operand::Decoration(Decoration::ArrayStride), Operand::LiteralInt32(4)]));
    m.annotations.push(Instruction::new(Op::MemberDecorate, None, None, vec![Operand::IdRef(st), Operand::LiteralInt32(0), Operand::Decoration(Decoration::Offset), Operand::LiteralInt32(0)]));
    m.annotations.push(Instruction::new(Op::Decorate, None, None, vec![Operand::IdRef(st), Operand::Decoration(Decoration::Block)]));
    m.annotations.push(Instruction::new(Op::Decorate, None, None, vec![Operand::IdRef(ssbo), Operand::Decoration(Decoration::DescriptorSet), Operand::LiteralInt32(CAPTURE_SET)]));
    m.annotations.push(Instruction::new(Op::Decorate, None, None, vec![Operand::IdRef(ssbo), Operand::Decoration(Decoration::Binding), Operand::LiteralInt32(0)]));
    for e in m.entry_points.iter_mut() { if m.header.as_ref().map(|h| h.version() >= (1, 4)).unwrap_or(false) { e.operands.push(Operand::IdRef(ssbo)); } }
    // constants for slot offsets and stride
    let mut consts: HashMap<u32, Word> = HashMap::new();
    let mut need = |v: u32, new_types: &mut Vec<Instruction>, fresh: &mut dyn FnMut() -> Word| -> Word {
        *consts.entry(v).or_insert_with(|| { let c = fresh(); new_types.push(Instruction::new(Op::Constant, Some(uint), Some(c), vec![Operand::LiteralInt32(v)])); c })
    };
    let c_zero = need(0, &mut new_types, &mut fresh);
    let c_stride = need(stride.max(1), &mut new_types, &mut fresh);
    let mut slot_consts = Vec::new();
    for s in 0..stride { slot_consts.push(need(s, &mut new_types, &mut fresh)); }
    // the epilogue: built once, spliced before every OpReturn of the entry function
    let mut epi: Vec<Instruction> = Vec::new();
    let vid = fresh();
    epi.push(Instruction::new(Op::Load, Some(uint), Some(vid), vec![Operand::IdRef(vi_var)]));
    let base = fresh();
    epi.push(Instruction::new(Op::IMul, Some(uint), Some(base), vec![Operand::IdRef(vid), Operand::IdRef(c_stride)]));
    let mut slot = 0usize;
    for o in &outs {
        let val = fresh();
        epi.push(Instruction::new(Op::Load, Some(o.ty), Some(val), vec![Operand::IdRef(o.var)]));
        for c in 0..o.comps {
            let comp = if o.comps == 1 { val } else if o.rows > 0 {
                let col = fresh();
                epi.push(Instruction::new(Op::CompositeExtract, Some(o.coltype), Some(col), vec![Operand::IdRef(val), Operand::LiteralInt32(c / o.rows)]));
                let x = fresh();
                epi.push(Instruction::new(Op::CompositeExtract, Some(o.elem), Some(x), vec![Operand::IdRef(col), Operand::LiteralInt32(c % o.rows)]));
                x
            } else {
                let x = fresh();
                epi.push(Instruction::new(Op::CompositeExtract, Some(o.elem), Some(x), vec![Operand::IdRef(val), Operand::LiteralInt32(c)]));
                x
            };
            let as_uint = if o.kind == 'u' { comp } else { let b = fresh(); epi.push(Instruction::new(Op::Bitcast, Some(uint), Some(b), vec![Operand::IdRef(comp)])); b };
            let idx = fresh();
            epi.push(Instruction::new(Op::IAdd, Some(uint), Some(idx), vec![Operand::IdRef(base), Operand::IdRef(slot_consts[slot])]));
            let p = fresh();
            epi.push(Instruction::new(Op::AccessChain, Some(ptr_uint_sb), Some(p), vec![Operand::IdRef(ssbo), Operand::IdRef(c_zero), Operand::IdRef(idx)]));
            epi.push(Instruction::new(Op::Store, None, None, vec![Operand::IdRef(p), Operand::IdRef(as_uint)]));
            slot += 1;
        }
    }
    let mut spliced = 0;
    for f in m.functions.iter_mut() {
        if f.def.as_ref().and_then(|d| d.result_id) != Some(entry_fn) { continue; }
        for b in f.blocks.iter_mut() {
            if let Some(pos) = b.instructions.iter().rposition(|i| i.class.opcode == Op::Return) {
                // result ids must be unique per splice: clone only for the first return
                if spliced > 0 { return Err("multiple returns in entry".into()); }
                for (k, i) in epi.iter().enumerate() { b.instructions.insert(pos + k, i.clone()); }
                spliced += 1;
            }
        }
    }
    if spliced == 0 { return Err("no OpReturn in entry".into()); }
    m.types_global_values.extend(new_types);
    m.header.as_mut().unwrap().bound = bound;
    let words = m.assemble();
    Ok(Capture { spirv: words.iter().flat_map(|w| w.to_le_bytes()).collect(), kinds, stride })
}

const CAPTURE_SET: u32 = 7;

fn compare_cap(kinds: &[(u32, char)], stride: usize, verts: usize, a: &[u8], b: &[u8]) -> (usize, Option<usize>) {
    let (mut mism, mut first) = (0, None);
    let mut per_slot = vec![0usize; stride];
    let mut ex: Vec<Option<(usize, u32, u32)>> = vec![None; stride];
    for v in 0..verts {
        // A vertex whose clip position is NaN (in either run) is discarded by the hardware; its other
        // outputs are never consumed (and may be OpUndef), so they are not compared.
        let nan_pos = (0..4.min(stride)).any(|k| {
            let o = (v * stride + k) * 4;
            o + 4 <= a.len() && (f32::from_bits(u32::from_le_bytes(a[o..o + 4].try_into().unwrap())).is_nan() || f32::from_bits(u32::from_le_bytes(b[o..o + 4].try_into().unwrap())).is_nan())
        });
        if nan_pos && kinds.first().map(|k| k.0) == Some(0) { continue; }
        for k in 0..stride {
            let o = (v * stride + k) * 4;
            if o + 4 > a.len() { break; }
            let (x, y) = (u32::from_le_bytes(a[o..o + 4].try_into().unwrap()), u32::from_le_bytes(b[o..o + 4].try_into().unwrap()));
            let ok = if kinds[k].1 == 'f' { words_close(x, y) } else { x == y };
            if !ok { mism += 1; per_slot[k] += 1; if ex[k].is_none() { ex[k] = Some((v, x, y)); } if first.is_none() { first = Some(o); } }
        }
    }
    if std::env::var("SPVDIFF_SLOTS").is_ok() {
        for k in 0..stride { if per_slot[k] > 0 { let (v, x, y) = ex[k].unwrap(); eprintln!("SLOT {} key={} kind={} mism={}/{} e.g. vertex {}: {:#010x}({}) vs {:#010x}({})", k, kinds[k].0, kinds[k].1, per_slot[k], verts, v, x, f32::from_bits(x), y, f32::from_bits(y)); } }
    }
    (mism, first)
}
