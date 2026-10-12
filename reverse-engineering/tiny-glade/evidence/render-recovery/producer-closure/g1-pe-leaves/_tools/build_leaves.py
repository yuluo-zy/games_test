#!/usr/bin/env python3
# Build pe-leaves.json + grouped .asm from _dump.json and authored conclusions.
import json, io, os, struct

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.dirname(HERE)
dump = json.load(io.open(os.path.join(OUT, '_dump.json'), encoding='utf-8'))

# va(int) -> (conclusion, confidence, category)
C = {}
def c(va, txt, conf, cat): C[int(va, 16) if isinstance(va, str) else va] = (txt, conf, cat)

# ---------------- raw::barrier ----------------
c(0x140bd6650, "SynchronizationScope1::new: builds empty scope. Three Vec fields {cap@+0x00,ptr@+0x08,len@+0x10} at offsets 0x00,0x18,0x30 each set to empty (cap=0, ptr=dangling 8, len=0); accumulating mask u64@+0x48 := 0x200000000001 (bit0 and bit45). Direct byte fact; field naming (vec/mask) per SYNTHESIS.", "high", "barrier")
c(0x140bd6950, "impl$5::reset: mask@+0x48 := 0x200000000001; zeroes the three vec length fields (+0x10,+0x28,+0x40). Does not touch ptr/cap (no dealloc). Direct byte fact.", "high", "barrier")
c(0x140bd6980, "impl$5::is_empty: returns (len@+0x10==0) && (len@+0x28==0) && (len@+0x40==0). Confirms the three Vec len offsets.", "high", "barrier")
c(0x140bd65e0, "ImageBarrier::new: out(rcx)=this; writes u64 rdx@+0x00 (image/handle), u32 5th-arg@+0x08, qword 0xffffffffffffffff@+0x0c (prev-index sentinel -1 spanning 0x0c..0x13), u8 r8b@+0x14, u8 r9b@+0x15, u8 0@+0x16. Struct ~0x17B. Field semantics inferred; offsets/args exact.", "medium", "barrier")
c(0x140bd6610, "BufferBarrier::new: out(rcx)=this; u64 rdx@+0x00, qword -1@+0x08 (prev-index sentinel), u8 r8b@+0x10, u8 r9b@+0x11. ~0x13B.", "high", "barrier")
c(0x140bd6630, "image_aspect_mask_from_format(fmt): if (fmt-0x7c) <= 6 -> table@0x142b0ffb8[(fmt-0x7c)] else 1. Table bytes 02000000 02000000 02000000 04000000 06000000 06000000 06000000 => fmt 0x7c/0x7d/0x7e->2(DEPTH), 0x7f->4(STENCIL), 0x80/0x81/0x82->6(DEPTH|STENCIL); default 1(COLOR). Returns VkImageAspectFlags.", "high", "barrier")

# ---------------- raw::managed::handle ----------------
c(0x140bd6b00, "impl$5::clone (RawManagedHandle::clone): rdx=src, rcx=dst(returned). r8=inner=[src+8]; `lock inc dword [inner+8]` (atomic refcount++). Then copies src->dst: store@0, inner@8, u32 checksum@0x10, u16 device_id@0x14, u8 resource_type@0x16. NO allocation. Confirms refcount lives at inner+0x08.", "high", "handle")
c(0x140bd6b30, "impl$7::eq: al = ([a+8]==[b+8]) && (qword[a+0x0f]==qword[b+0x0f]). The two 8-byte compares together cover bytes 0x08..0x16 = resource(inner) + resource_checksum@0x10 + device_id@0x14 + resource_type@0x16. Field @0x00 (store) and 0x00..0x07 NOT compared.", "high", "handle")
c(0x140bd69a0, "impl$3::fmt: Debug for RawManagedHandle. struct name str='RawManagedHandle'; fields: resource=[self+8], resource_checksum=[self+0x10], resource_type=[self+0x16], device_id=[self+0x14]. Real field names recovered from .rdata strings.", "high", "handle")
c(0x140bd6b50, "impl$36::fmt: Debug newtype 'RuntimeDeviceId' (tuple field1 of u16@+0x14). Name string from .rdata.", "high", "handle")
c(0x140bd6b90, "impl$44::fmt: Debug newtype 'ManagedResourceChecksum' (tuple field1 of checksum@+0x10). Name string from .rdata.", "high", "handle")
c(0x1408fcd10, "impl$57::fmt: enum Debug by discriminant byte[rcx] via name table @0x142aaa198/0x142aaa1b0 = {0:Untracked,1:Tracked,2:MarkedUnused}. Managed-resource tracking state enum (3 variants).", "high", "handle")
c(0x140bd6bd0, "impl$51::fmt: enum Debug by discriminant byte[rcx] via name table @0x142b10168/0x142b101a8 = {0:Texture,1:Buffer,2:Swapchain,3:TextureView,4:RenderPass,5:Sampler,6:PersistentDescriptorSet,7:Framebuffer}. This is the managed store resource-type tag; 8 consecutive variants (0..7).", "high", "handle")
c(0x141621b00, "impl$15::fmt<ManagedTexture>: Debug for the ManagedTexture handle specialization. format! of 'rhapsody::raw::managed::texture::ManagedTexture' (len 0x2f) then debug_struct field 'raw'. Diagnostic; body fully decoded.", "medium", "handle")

# ---------------- raw::buffer ----------------
c(0x140bd4de0, "BufferDesc::new(size=rdx,u32 usage=r8d,u8 memloc=r9b): assert size>0 (panic 'assertion failed: size > 0' @0x142b0f830); writes u64 size@0, u32 usage@8, u16 alignment@0xc:=0, u8 memory_location@0xe:=r9b. BufferDesc=16B {size@0,usage@8,alignment@0xc,memory_location@0xe}.", "high", "buffer")
c(0x140bd4e20, "BufferDesc::new_gpu_only(size,usage): same as new but memory_location@0xe := 1.", "high", "buffer")
c(0x140bd4e60, "BufferDesc::new_cpu_to_gpu(size,usage): memory_location@0xe := 2.", "high", "buffer")
c(0x140bd4ea0, "BufferDesc::new_cpu_to_gpu_low_prio(size,usage): memory_location@0xe := 3.", "high", "buffer")
c(0x140bd4ee0, "BufferDesc::new_gpu_to_cpu(size,usage): memory_location@0xe := 4. So BufferMemoryLocation enum: 1=GpuOnly,2=CpuToGpu,3=CpuToGpuLowPrio,4=GpuToCpu (0=custom via ::new).", "high", "buffer")
c(0x1415a6090, "impl$10::fmt: Debug for BufferDesc. struct 'BufferDesc'; fields 'size','usage','memory_location','alignment'. Confirms the 4 fields+names.", "high", "buffer")
c(0x140bd51a0, "impl$4::byte_count: if [rcx]!=0 then return qword [rcx+8] else return 0. (Option-like accessor; not BufferDesc itself.)", "medium", "buffer")
c(0x1406868b0, "impl$3::byte_count: returns qword [rcx+8].", "high", "buffer")

# ---------------- raw::texture ----------------
c(0x140bd5790, "TextureDesc::extent_2d: returns qword [rcx+0xc] = packed (width u32@0xc, height u32@0x10).", "high", "texture")
c(0x140bd5430, "TextureDesc::format(u32): copy self, set u32 format@0x08 := arg.", "high", "texture")
c(0x140bd5340, "TextureDesc::usage(u32): copy self, set u32 usage@0x00 := arg.", "high", "texture")
c(0x140bd5370, "TextureDesc::sampled: self.usage@0x00 |= 0x4 (VK_IMAGE_USAGE_SAMPLED_BIT). Returns copy.", "high", "texture")
c(0x140bd5390, "TextureDesc::storage: usage@0x00 |= 0x8 (VK_IMAGE_USAGE_STORAGE_BIT).", "high", "texture")
c(0x140bd53b0, "TextureDesc::transfer_dst: usage@0x00 |= 0x2 (VK_IMAGE_USAGE_TRANSFER_DST_BIT).", "high", "texture")
c(0x140bd53d0, "TextureDesc::color_attachment: usage@0x00 |= 0x10 (VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT).", "high", "texture")
c(0x140bd53f0, "TextureDesc::depth_stencil_attachment: usage@0x00 |= 0x20 (VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT).", "high", "texture")
c(0x140bd5410, "TextureDesc::sampled_with_different_format: usage@0x00 (u64) |= 0x10800000004 => bits 2 + 35 + 40.", "high", "texture")
c(0x140bd5460, "TextureDesc::mip_levels(u8): copy self, set byte mip_levels@0x1c := arg. New precise field offset (mip_levels@0x1c).", "high", "texture")
c(0x140bd55a0, "TextureDesc::sample_count(u8): copy self, set byte sample_count@0x1d := arg. New precise field offset.", "high", "texture")
c(0x140bd5270, "TextureDesc::new({w,h,d?}, kind=r8b, fmtarg=r9d): zero u64@0; u32@8:=r9d; w,h from [rdx]->@0xc,@0x10; u32@0x14:=[rdx+8]; array_elements@0x18:=1; u16@0x1c:=0x0101 (mip=1,samples=1); kind@0x1e:=r8b.", "high", "texture")
c(0x140bd52a0, "TextureDesc::new_2d(packed_wh edx, format r8d): u64@0=0; format@0x08=r8d; w=edx@0xc; h=edx>>32@0x10; depth@0x14=1; array_elements@0x18=1; mip@0x1c=1; samples@0x1d=1; kind@0x1e=2 (Tex2d).", "high", "texture")
c(0x140bd52e0, "TextureDesc::new_2d_array(packed_wh edx, format r9d, array_elements r8d): ...; @0x18=r8d; u16@0x1c=0x0101; kind@0x1e=3 (Tex2dArray).", "high", "texture")
c(0x140bd5310, "TextureDesc::new_3d({w,h,d}, format r8d): kind@0x1e=4 (Tex3d); depth from [rdx+8]@0x14.", "high", "texture")
c(0x140bd54a0, "TextureDesc::all_mip_levels: computes full mip chain length from extent (log2). Decoded body available.", "medium", "texture")
c(0x140bd5730, "TextureDesc::half_res: halves width/height (max(1,...)). Decoded body available.", "medium", "texture")
c(0x1408edb50, "impl$14::fmt (etc, 4 instances): enum Debug by discriminant via name table = {0:Tex1d,1:Tex1dArray,2:Tex2d,3:Tex2dArray,4:Tex3d,5:Cube,6:CubeArray}. This is TextureDesc.kind; matches kind values 2/3/4 used by the constructors.", "high", "texture")
c(0x1408eda30, "impl$28::fmt: Debug for TextureDesc (struct 'TextureDesc' + kind variant names). Diagnostic.", "medium", "texture")
c(0x140bd57a0, "TextureViewDesc::new: zeroes six dwords at 0x00,0x08,0x10,0x18,0x20,0x28 (six 8-byte Option-like {disc@X,val@X+4} fields), qword@0x30=0, byte@0x38=0. TextureViewDesc ~0x3cB.", "high", "texture")
c(0x140bd57e0, "TextureViewDesc::view_type(u32): copy; set disc@0x00=1, val@0x04:=arg.", "high", "texture")
c(0x140bd5820, "TextureViewDesc::usage(u32): copy; set disc@0x08=1, val@0x0c:=arg.", "high", "texture")
c(0x140bd5860, "TextureViewDesc::format(u32): copy; set disc@0x10=1, val@0x14:=arg.", "high", "texture")
c(0x140bd58a0, "TextureViewDesc::aspect_mask(u32): copy; set disc@0x18=1, val@0x1c:=arg.", "high", "texture")
c(0x140bd5920, "TextureViewDesc::level_count(u32,u32): copy; set u32@0x20:=arg1, u32@0x24:=arg2.", "medium", "texture")
c(0x140bd59a0, "TextureViewDesc::layer_count(u32,u32): copy; set u32@0x28:=arg1, u32@0x2c:=arg2.", "medium", "texture")
c(0x140bd58e0, "TextureViewDesc::base_mip_level(u32): copy; set u32@0x30:=arg.", "high", "texture")
c(0x140bd5960, "TextureViewDesc::base_layer(u32): copy; set u32@0x34:=arg.", "high", "texture")
c(0x140bd59f0, "TextureViewDesc::bind_individual_mips(bool): copy 0x3c bytes; set byte@0x38:=arg.", "high", "texture")
c(0x140bd5a40, "impl$4::from: assert r8>0; {disc@0:=3, ptr@8:=rdx, len@0x10:=r8}. Builds a variant with tag 3.", "medium", "texture")

# ---------------- raw::device ----------------
c(0x140bf84c0, "GraphicsDevice::device_uuid: returns 16-byte movups from [*(dev+0x658)+0x690]. i.e. pipelineCacheUUID from VkPhysicalDeviceProperties (physical device base = *(dev+0x658)+0x10).", "high", "device")
c(0x140bf8950, "GraphicsDevice::physical_device: returns *(dev+0x658) + 0x10 (addr of the PhysicalDevice object).", "high", "device")
c(0x140bf8960, "GraphicsDevice::debug_utils: returns &(dev+0x870) if [dev+0x870]!=0 else NULL (Option<&DebugUtils> accessor).", "medium", "device")
c(0x140bf8fa0, "GraphicsDevice::min_subgroup_size: returns (eax=[dev+0x40], edx=[dev+0x44]) — a pair of u32 read from dev.", "high", "device")
c(0x140bf8f80, "GraphicsDevice::refresh_memory_budget: if byte[dev+0x1a]!=0 then tail-call PhysicalDevice::refresh_memory_budget(&(*(dev+0x658)+0x10)); else no-op.", "high", "device")
c(0x140be23a0, "GraphicsDevice::impl$5::drop: wait_idle(); [dev+0x648]->take_out_dropped_waste(dev); lock RawMutex@dev+0x718 (parking_lot); Queue::reclaim_command_buffer_resources(&Queue@dev+0x730, dev); for each Queue in Vec@dev+0x7e8 (len@+0x7f0, stride 0xb0) reclaim; unlock. Device Drop path.", "high", "device")
c(0x1428caa40, "ResourceRegistry::image_from_raw_handle panic_cold_explicit: calls panic_explicit with msg@0x142b13cf0.", "medium", "rg")
c(0x1428caa60, "ResourceRegistry::buffer panic_cold: panic_explicit msg@0x142b13d50.", "medium", "rg")
c(0x1428caa80, "ResourceRegistry::image_view panic_cold: panic_explicit msg@0x142b13dc8.", "medium", "rg")

# ---------------- raw::swapchain ----------------
c(0x140c0f120, "TrackedSwapchainTextureLease::image: clones the embedded RawManagedHandle: `lock inc dword [lease+0x18]+8` (refcount), copies lease+0x10..0x26 into the returned ManagedTexture handle, back-ptr = &(lease+0x28). Same refcount semantics as handle::clone.", "high", "swapchain")
c(0x140c0ef70, "Swapchain::any_present_successful: returns byte [rcx+0x1c].", "high", "swapchain")
c(0x140c0e7e0, "Swapchain::format: returns u32 [rcx+0x18].", "high", "swapchain")
c(0x1418cd0f0, "impl$8::fmt: Debug for swapchain state enum; variants/fields 'SwapchainDesc','dims','vsync','exclusive','ZeroDims'. Diagnostic.", "medium", "swapchain")
c(0x1418cd1d0, "impl$9::fmt: Debug for acquire-result enum; names 'Retired','Stale','desc','reason','ZeroDims','Vulkan' (+source path string). Diagnostic.", "medium", "swapchain")

# ---------------- raw::shader ----------------
c(0x140be67f0, "get_vk_blend_mode(desc): idx=(byte[desc+0x1b]>>3)&0xc (byte offset 0/4/8/12). Builds VkPipelineColorBlendAttachmentState(0x20B): blendEnable=t0[idx], srcColorFactor@4=1, dstColorFactor=t1[idx], colorOp@0xc=0, srcAlphaFactor@0x10=1, dstAlphaFactor=t2[idx], alphaOp@0x18=0, colorWriteMask@0x1c=0xf. t0@0x142b139f8=[0,1,1,1]; t1@0x142b13a08=[0,1,7,0x10]; t2@0x142b13a18=[0,1,7,0x12] (VkBlendFactor: 0=ZERO,1=ONE,7=ONE_MINUS_SRC_ALPHA,0x10=ONE_MINUS_SRC1_COLOR,0x12=ONE_MINUS_SRC1_ALPHA). idx0=blend off.", "high", "shader")
c(0x140be67c0, "RenderPassAttachmentDesc::sample_count(u8): copy; set byte@0x0c:=arg.", "high", "shader")
c(0x140be67a0, "RenderPassAttachmentDesc::clear_input: copy; set u32@0x04:=1 (tag/bool true).", "high", "shader")
c(0x140be6780, "RenderPassAttachmentDesc::new(format=edx): u32@0:=edx; qword@4:=0; byte@0xc:=1 (sample_count default 1). ~0x10B.", "high", "shader")
c(0x140be6760, "RasterPipelineDesc::new: copy 0x18B from src; set u32@0x18:=r8d.", "high", "shader")
c(0x140be6750, "impl$7::default: returns u32 constant 0x1ffc00d0 (packed default flags/discriminant).", "high", "shader")
c(0x14003bb80, "ComputePipelineDesc::new: single `ret` — no-op constructor (unit return).", "high", "shader")

# ---------------- raw::binding ----------------
c(0x140bee130, "GraphicsDevice::bind_pipeline: tail-call through device fn-table slot [rcx+0x2d8] with (cmd=[rdx+0x48], edx=[r8+0x30]=bindPoint, r8=[r8+0x28]=pipeline). vkCmdBindPipeline wrapper.", "high", "binding")
c(0x141621310, "impl$1::fmt: Debug impl (diagnostic; body decoded in asm).", "low", "binding")

# ---------------- rg (state/recorder/executor/pass_api/resource/compiled) ----------------
c(0x140be57d0, "rg::state::impl$0::default: constructs a default RenderGraphState incl. a std RandomState HashMap (calls RandomState::new KEYS shim). Body fully decoded.", "medium", "rg")
c(0x140bece50, "ResourceSyncGroup::new: eax=1; `lock xadd dword [0x14362c230], eax` -> returns previous value of the global AtomicU32 (monotonic group-id allocator).", "high", "rg")
c(0x140bece60, "ResourceSyncGroup::is_equivalent: (u32[a]==u32[b]) — equality of the group id.", "high", "rg")
c(0x140b25c90, "PassResourceAccessType::new: `mov eax,edx; mov edx,ecx; ret` — swaps the two args (ecx<->edx). Root cause of the earlier mis-modeled field binding.", "high", "rg")
c(0x140be9690, "rg::compiled::pass_access_range_indices: `mov eax,ecx; mov edx,edx; ret` — identity, returns the two u32 args unchanged as a pair. The previously-'unavailable' 5-byte body, now solved.", "high", "rg")
c(0x140be95e0, "rg::compiled::BatchCompiler::new: constructs {cap/ptr@0..0x08 (empty vec: [0]=0,[8]=8),[0x10..0x1f]=0,[0x19..0x28]=0}. The previously-'unavailable' 30-byte body, now solved.", "high", "rg")
c(0x140833250, "rg::resource::ResourceHandle::raw: `mov eax,ecx; ret` — identity returning the u32 handle.", "high", "rg")
c(0x140bece40, "rg::recorder::PassResourceRef::resource_index: returns u32 [rcx].", "high", "rg")
c(0x140bd6c00, "Ref<Texture,GpuSrv>::bind_view: copies 0x40B TextureViewDesc from r8, sets u32@0x3c:=[rdx], byte@0x40:=0.", "high", "rg")
c(0x140bd6c30, "Ref<Texture,GpuUav>::bind_view: same but byte@0x40:=1 (Srv=0, Uav=1 discriminator).", "high", "rg")
c(0x140bd6c60, "rg::pass_api::impl$5::bind: {u32@0x04:=[rdx]; u32@0x00:=4} — variant tag 4.", "high", "rg")
c(0x140bd6c70, "Ref<Buffer,GpuSrv>::bind_uniform_buffer: {u32@0x04:=[rdx]; u32@0x00:=3} — variant tag 3.", "high", "rg")
c(0x140bd6ce0, "rg::resource::impl$4::from: copy 0x20B.", "high", "rg")
c(0x140bd6d00, "rg::resource::impl$5::from: copy 0x10B; byte@0x1e:=7 (tag 7).", "high", "rg")
c(0x140bdbcc0, "rg::init::impl$14::from: copy 0x40B; qword@0:=0x14 (tag 20).", "high", "rg")
c(0x140be9c70, "rg::executor::PassPgoDuration::new: `mov rdx,r8; mov rax,rcx; ret` — returns (rcx, r8) as a 2-field struct (args passthrough).", "medium", "rg")
c(0x1416f7850, "RenderPassApi::end_render_pass: rcx=[self]; rdx=[self+8]; rcx=[rax+0x48]; jmp [rdx+0x428] (vtable dispatch).", "high", "rg")
c(0x1416f7960, "RenderPassApi::set_line_width: if byte[*(self+8)+0xad9]!=0 then jmp through [(*(self)+0x48)+0x2f0]. Gated by a feature flag at +0xad9.", "high", "rg")
c(0x1415fc050, "rg::recorder::impl$1::render<...dispatch closure...>: push rsi; call Box<FnOnce>::call_once; return rsi. Thin closure-invoker.", "medium", "rg")
c(0x1428ee9f0, "rg::import_export::impl$1::from_owned_rg_resource panic_cold: panic_explicit.", "medium", "rg")

# ---------------- raw:: (misc) ----------------
c(0x140a9fa70, "AvailableGraphicsDeviceInitState::is_error: (byte[rcx]==1).", "high", "raw")
c(0x140c001c0, "DeviceDriverVersion::vendor_string: table lookup by vendor dword -> &str. table@0x142b17d38/0x142b17d70 = {0:NVIDIA,1:Intel,2:Intel,3:AMD,4:Qualcomm,5:Apple,6:Unknown}.", "high", "raw")
c(0x140c001e0, "DeviceDriverVersion::is_amd: (dword[rcx]==3).", "high", "raw")
c(0x140c001f0, "DeviceDriverVersion::is_apple: (dword[rcx]==5).", "high", "raw")
c(0x140c0e600, "GpuAllocator::peak_total_allocated_bytes: qword [rcx+0x680].", "high", "raw")
c(0x140c0e630, "GpuAllocator::peak_device_local_bytes: qword [rcx+0x670].", "high", "raw")
c(0x140c0e640, "GpuAllocator::peak_system_bytes: qword [rcx+0x678].", "high", "raw")
c(0x140c0e610, "GpuAllocator::peak_allocator_report: returns &(rcx+0x620) if its value !=0 else NULL (Option accessor).", "high", "raw")
c(0x14067dd80, "GpuAllocator::inner: `mov rax,rcx; ret` — identity (returns &self).", "high", "raw")
c(0x140c02280, "DynamicConstantsChunk::device_address: returns qword [rcx+0x10].", "high", "raw")
c(0x140c02290, "DynamicConstantsChunk::into_gpu_lifetime_tracker: copies [rdx+0x20],[rdx+0x30] -> out (0x20B).", "high", "raw")
c(0x140c02da0, "RenderStats::snapshot_and_reset: three 64-bit atomic xchg-0 on [rdx],[rdx+8],[rdx+0x10]; old values written to out[0..0x10]. 3 atomic u64 counters.", "high", "raw")
c(0x140be2370, "command_buffer::impl$4::drop: read [rcx]; if ==0 or ==3 -> return (no destroy); else panic. Guard against dropping a live command buffer.", "high", "raw")
c(0x140bed620, "command_buffer::impl$11::fmt: Debug for the command-buffer state enum; names include 'Initial','Recording','Recorded','Submitted','Universal' and fields 'queue','seq'. Diagnostic.", "medium", "raw")
c(0x140be6b40, "print_log_dump: `jmp leekspin_print_dump`.", "high", "raw")
c(0x140be6b50, "dump_log: if global [0x143634e40]!=0 -> RingBuffer::to_inner(&global) else unwrap_failed. Flushes the leekspin ring log.", "high", "raw")
c(0x140be2ad0, "leek::begin_rg_pass: copies 16B from rcx, appends tag byte 0x2a with payload; RingBuffer::append(global@0x143634e40, rec, 0x15). Trace marker (0x2a).", "high", "raw")
c(0x140be2b10, "leek::end_rg_pass: RingBuffer::append(global, {0x2b}, 1). Trace marker 0x2b.", "high", "raw")
c(0x140be6a90, "init_logging: logging initializer (body decoded).", "low", "raw")
c(0x140be2880, "leek::impl$1::from: UTF-8 decode of a byte slice into a 16-byte (char?) array with bounds check. Body fully decoded.", "medium", "raw")
c(0x140be2710, "leek::impl$0::fmt: Debug impl (diagnostic).", "low", "raw")
c(0x140be2990, "leek::impl$2::fmt: Debug impl (diagnostic).", "low", "raw")
c(0x140bdd3f0, "raw::error::impl$3::fmt: Debug for a boxed error chain (discriminant-based variant selection). Diagnostic.", "medium", "raw")
c(0x140082a00, "GraphicsWorkarounds serialize (serde): serializes workaround flags. Body decoded.", "medium", "raw")
c(0x14090dda0, "TempList<T>::add: list-append. Body decoded.", "medium", "raw")

# default for anything not explicitly authored
def default_concl(it):
    n = it['name']; s = it['size']
    if '::fmt' in n or '::from' in n or 'serialize' in n:
        return ("Leaf/impl fully decoded from PE bytes (semantics mostly diagnostic/formatting); see insns.", "medium")
    if s > 700:
        return ("Large function body fully decoded (not a micro-leaf); decode present in insns.", "medium")
    return ("Leaf fully decoded from PE bytes; no additional authored interpretation.", "low")

out = []
for it in dump:
    va = int(it['va'], 16)
    if va in C:
        concl, conf, cat = C[va]
    else:
        txt, conf = default_concl(it); cat = 'misc'
        concl = txt
    out.append({
        'va': it['va'], 'size': it['size'], 'name': it['name'], 'modules': it.get('module'),
        'bytes': it['bytes'], 'n_insns': it['n_decoded'],
        'category': cat, 'conclusion': concl, 'confidence': conf,
        'insns': it['insns'],
    })
json.dump({'source': 'direct PE image read (imagebase 0x140000000)',
           'exe': 'workspace/input/tiny-glade.exe',
           'sha256': 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
           'count': len(out), 'items': out},
          io.open(os.path.join(OUT, 'pe-leaves.json'), 'w', encoding='utf-8'),
          ensure_ascii=False, indent=1)

# group asm files
groups = {}
for it in out:
    groups.setdefault(it['category'], []).append(it)
for g, items in groups.items():
    lines = []
    for it in sorted(items, key=lambda x: x['va']):
        lines.append('=== %s size=%d  %s' % (it['va'], it['size'], it['name']))
        lines.append('    ; %s  [%s]' % (it['conclusion'], it['confidence']))
        for i in it['insns']:
            lines.append('    %-14s %-24s %-16s %s' % (i['a'], i['b'], i['m'], i['o'] + (i['ann'] or '')))
        lines.append('')
    io.open(os.path.join(OUT, '%s.asm' % g), 'w', encoding='utf-8').write('\n'.join(lines))
print('wrote pe-leaves.json +', len(groups), 'asm files:', sorted(groups))
