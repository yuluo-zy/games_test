14093bd80: push rbp
14093bd81: push rsi
14093bd82: push rdi
14093bd83: sub rsp, 0x90
14093bd8a: lea rbp, [rsp + 0x80]
14093bd92: mov qword ptr [rbp + 8], 0xfffffffffffffffe
14093bd9a: mov rdi, rdx
14093bd9d: mov rsi, rcx
14093bda0: xor ecx, ecx
14093bda2: call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > > >
14093bda7: test rax, rax
14093bdaa: je 0x14093be45
14093bdb0: movups xmm0, xmmword ptr [rax]
14093bdb3: inc qword ptr [rax]
14093bdb6: movups xmm1, xmmword ptr [rip + 0x2323a83] ; 0x142c5f840 b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00Pw\x88@\x01\x00\x00\x00\x88\x01\x00\x00\x00\x00\x00\x00'
14093bdbd: movaps xmmword ptr [rbp - 0x50], xmm1
14093bdc1: movups xmm1, xmmword ptr [rip + 0x2323a68] ; 0x142c5f830 b' \xf8\xc5B\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
14093bdc8: movaps xmmword ptr [rbp - 0x60], xmm1
14093bdcc: movaps xmmword ptr [rbp - 0x40], xmm0
14093bdd0: lea r8, [rbp - 0x60]
14093bdd4: mov rcx, rdi
14093bdd7: xor edx, edx
14093bdd9: call 0x141474b90 ; core::iter::adapters::map::impl$2::fold<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,hashbrown::map::impl$82::extend::closure_env$0<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl$1::create_preset_textures::closure_env$0> > > >
14093bdde: movaps xmm0, xmmword ptr [rbp - 0x60]
14093bde2: movaps xmm1, xmmword ptr [rbp - 0x50]
14093bde6: movaps xmm2, xmmword ptr [rbp - 0x40]
14093bdea: movaps xmmword ptr [rbp - 0x10], xmm2
14093bdee: movaps xmmword ptr [rbp - 0x20], xmm1
14093bdf2: movaps xmmword ptr [rbp - 0x30], xmm0
14093bdf6: mov eax, dword ptr [rsi + 0x1c]
14093bdf9: mov rcx, qword ptr [rsi + 0x10]
14093bdfd: mov dword ptr [rcx], eax
14093bdff: mov rdx, qword ptr [rsi]
14093be02: lea rcx, [rdx + 0x50]
14093be06: add rdx, 0x70
14093be0a: mov r8d, 0x68
14093be10: mov r9d, 0x10
14093be16: mov qword ptr [rbp], rcx
14093be1a: call 0x14004eed0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global>
14093be1f: movaps xmm0, xmmword ptr [rbp - 0x30]
14093be23: movaps xmm1, xmmword ptr [rbp - 0x20]
14093be27: movaps xmm2, xmmword ptr [rbp - 0x10]
14093be2b: mov rax, qword ptr [rbp]
14093be2f: movups xmmword ptr [rax + 0x20], xmm2
14093be33: movups xmmword ptr [rax + 0x10], xmm1
14093be37: movups xmmword ptr [rax], xmm0
14093be3a: add rsp, 0x90
14093be41: pop rdi
14093be42: pop rsi
14093be43: pop rbp
14093be44: ret 
14093be45: lea rcx, [rip + 0x22fec54] ; 0x142c3aaa0 b'P\xaa\xc3B\x01\x00\x00\x00O\x00\x00\x00\x00\x00\x00\x00\x14\x01\x00\x00\x19\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00'
14093be4c: call 0x1428d80d0 ; std::thread::local::panic_access_error
14093be51: int3 
14093be52: nop word ptr cs:[rax + rax]
14093be60: mov qword ptr [rsp + 0x10], rdx
14093be65: push rbp
14093be66: push rsi
14093be67: push rdi
14093be68: sub rsp, 0x20
14093be6c: lea rbp, [rdx + 0x80]
14093be73: lea rcx, [rbp - 0x60]
14093be77: mov r8d, 0x68
14093be7d: mov r9d, 0x10
14093be83: lea rdx, [rbp - 0x40]
14093be87: call 0x14004eed0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global>
14093be8c: nop 
14093be8d: add rsp, 0x20
14093be91: pop rdi
14093be92: pop rsi
14093be93: pop rbp
14093be94: ret 
14093be95: nop word ptr cs:[rax + rax]
14093bea0: mov qword ptr [rsp + 0x10], rdx
14093bea5: push rbp
14093bea6: push rsi
14093bea7: push rdi
14093bea8: sub rsp, 0x20
14093beac: lea rbp, [rdx + 0x80]
14093beb3: movaps xmm0, xmmword ptr [rbp - 0x30]
14093beb7: movaps xmm1, xmmword ptr [rbp - 0x20]
14093bebb: movaps xmm2, xmmword ptr [rbp - 0x10]
14093bebf: mov rax, qword ptr [rbp]
14093bec3: movups xmmword ptr [rax + 0x20], xmm2
14093bec7: movups xmmword ptr [rax + 0x10], xmm1
14093becb: movups xmmword ptr [rax], xmm0
14093bece: add rsp, 0x20
14093bed2: pop rdi
14093bed3: pop rsi
14093bed4: pop rbp
14093bed5: ret 
