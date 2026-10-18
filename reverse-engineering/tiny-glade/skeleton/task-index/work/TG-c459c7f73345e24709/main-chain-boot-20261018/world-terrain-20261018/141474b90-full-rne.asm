141474b90: push rbp
141474b91: push r15
141474b93: push r14
141474b95: push r13
141474b97: push r12
141474b99: push rsi
141474b9a: push rdi
141474b9b: push rbx
141474b9c: sub rsp, 0x1b8
141474ba3: lea rbp, [rsp + 0x80]
141474bab: mov qword ptr [rbp + 0x130], 0xfffffffffffffffe
141474bb6: mov qword ptr [rbp + 0x128], r8
141474bbd: mov r15, qword ptr [rcx]
141474bc0: lea r12, [rbp + 0x68]
141474bc4: lea r13, [rbp - 0x42]
141474bc8: mov sil, 2
141474bcb: movzx ebx, dl
141474bce: lea rdi, [rip + 0x17f1043] ; 0x142c65c18 b'9\xf0\x80\xfeH\xf0\x80\xfe\xc6\xef\x80\xfe\xcb\xef\x80\xfe\xd0\xef\x80\xfe\xd5\xef\x80\xfe\xda\xef\x80\xfe\xdf\xef\x80\xfe'
141474bd5: movsxd rax, dword ptr [rdi + rbx*4]
141474bd9: add rax, rdi
141474bdc: jmp rax
141474bde: mov sil, 3
141474be1: jmp 0x141474c60
141474be3: mov sil, 4
141474be6: jmp 0x141474c60
141474be8: mov sil, 5
141474beb: jmp 0x141474c60
141474bed: mov sil, 6
141474bf0: jmp 0x141474c60
141474bf2: mov sil, 7
141474bf5: jmp 0x141474c60
141474bf7: mov sil, 8
141474bfa: jmp 0x141474c60
141474bfc: mov sil, 9
141474bff: jmp 0x141474c60
141474c01: mov sil, 0xa
141474c04: jmp 0x141474c60
141474c06: mov sil, 0xb
141474c09: jmp 0x141474c60
141474c0b: mov sil, 0xc
141474c0e: jmp 0x141474c60
141474c10: mov sil, 0xd
141474c13: jmp 0x141474c60
141474c15: mov sil, 0xe
141474c18: jmp 0x141474c60
141474c1a: mov sil, 0xf
141474c1d: jmp 0x141474c60
141474c1f: mov sil, 0x10
141474c22: jmp 0x141474c60
141474c24: mov sil, 0x11
141474c27: jmp 0x141474c60
141474c29: mov sil, 0x12
141474c2c: jmp 0x141474c60
141474c2e: mov sil, 0x13
141474c31: jmp 0x141474c60
141474c33: mov sil, 0x14
141474c36: jmp 0x141474c60
141474c38: mov sil, 0x15
141474c3b: jmp 0x141474c60
141474c3d: mov sil, 0x16
141474c40: jmp 0x141474c60
141474c42: mov sil, 0x17
141474c45: jmp 0x141474c60
141474c47: mov sil, 0x18
141474c4a: jmp 0x141474c60
141474c4c: mov sil, 0x19
141474c4f: jmp 0x141474c60
141474c51: mov sil, 1
141474c54: nop word ptr cs:[rax + rax]
141474c60: lea rcx, [rbp + 0x30]
141474c64: mov edx, ebx
141474c66: call 0x140a59890 ; country_core::resources::texture_presets::impl$1::from
141474c6b: mov r14, qword ptr [r15]
141474c6e: add r14, 0x10
141474c72: lea rcx, [rbp + 0x50]
141474c76: call 0x140c9cdb0 ; utils::name_hash::NameHash::debug_name
141474c7b: mov r9, rdx
141474c7e: movups xmm0, xmmword ptr [rbp + 0x30]
141474c82: movups xmm1, xmmword ptr [rbp + 0x40]
141474c86: movaps xmmword ptr [rbp + 0xd0], xmm1
141474c8d: movaps xmmword ptr [rbp + 0xc0], xmm0
141474c94: mov qword ptr [rbp + 0x18], 1
141474c9c: lea rcx, [rbp + 0x18]
141474ca0: mov qword ptr [rsp + 0x28], rcx
141474ca5: lea rcx, [rbp + 0xc0]
141474cac: mov qword ptr [rsp + 0x20], rcx
141474cb1: lea rcx, [rbp + 0x60]
141474cb5: mov rdx, r14
141474cb8: mov r8, rax
141474cbb: call 0x1416a71f0 ; rhapsody::raw::device::GraphicsDevice::create_texture<enum2$<rhapsody::raw::texture::TextureInitialData> >
141474cc0: cmp dword ptr [rbp + 0x60], 0xb
141474cc4: jne 0x141474deb
141474cca: mov rax, qword ptr [r12 + 0x30]
141474ccf: mov qword ptr [rbp + 0xf0], rax
141474cd6: movups xmm0, xmmword ptr [r12]
141474cdb: movups xmm1, xmmword ptr [r12 + 0x10]
141474ce1: movups xmm2, xmmword ptr [r12 + 0x20]
141474ce7: movaps xmmword ptr [rbp + 0xe0], xmm2
141474cee: movaps xmmword ptr [rbp + 0xd0], xmm1
141474cf5: movaps xmmword ptr [rbp + 0xc0], xmm0
141474cfc: movups xmm3, xmmword ptr [rbp + 0x10a]
141474d03: movups xmmword ptr [r13 + 0x4a], xmm3
141474d08: movaps xmm3, xmmword ptr [rbp + 0x100]
141474d0f: movups xmmword ptr [r13 + 0x40], xmm3
141474d14: mov rax, qword ptr [rbp + 0xf0]
141474d1b: mov qword ptr [r13 + 0x30], rax
141474d1f: mov rax, qword ptr [rbp + 0xf8]
141474d26: mov qword ptr [r13 + 0x38], rax
141474d2a: movups xmmword ptr [r13 + 0x20], xmm2
141474d2f: movups xmmword ptr [r13 + 0x10], xmm1
141474d34: movups xmmword ptr [r13], xmm0
141474d39: movups xmm0, xmmword ptr [r13 + 0x4a]
141474d3e: movups xmmword ptr [rbp + 0xaa], xmm0
141474d45: movups xmm0, xmmword ptr [r13 + 0x40]
141474d4a: movaps xmmword ptr [rbp + 0xa0], xmm0
141474d51: movups xmm0, xmmword ptr [r13]
141474d56: movups xmm1, xmmword ptr [r13 + 0x10]
141474d5b: movups xmm2, xmmword ptr [r13 + 0x20]
141474d60: mov rax, qword ptr [r13 + 0x30]
141474d64: mov qword ptr [rbp + 0x90], rax
141474d6b: mov rax, qword ptr [r13 + 0x38]
141474d6f: mov qword ptr [rbp + 0x98], rax
141474d76: movaps xmmword ptr [rbp + 0x80], xmm2
141474d7d: movaps xmmword ptr [rbp + 0x70], xmm1
141474d81: movaps xmmword ptr [rbp + 0x60], xmm0
141474d85: mov byte ptr [rbp + 0xba], 7
141474d8c: lea rcx, [rbp + 0xc0]
141474d93: lea r9, [rbp + 0x60]
141474d97: mov rdx, qword ptr [rbp + 0x128]
141474d9e: mov r8d, ebx
141474da1: call 0x141525760 ; hashbrown::map::HashMap::insert<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState,std::hash::random::RandomState,alloc::alloc::Global>
141474da6: cmp byte ptr [rbp + 0xf6], 7
141474dad: je 0x141474dc7
141474daf: lea rcx, [rbp + 0xc0]
141474db6: call 0x14003bb80 ; zune_jpeg::upsampler::upsample_no_op
141474dbb: lea rcx, [rbp + 0xc0]
141474dc2: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
141474dc7: movzx ebx, sil
141474dcb: movsxd rax, dword ptr [rdi + rbx*4]
141474dcf: add rax, rdi
141474dd2: mov sil, 2
141474dd5: jmp rax
141474dd7: add rsp, 0x1b8
141474dde: pop rbx
141474ddf: pop rdi
141474de0: pop rsi
141474de1: pop r12
141474de3: pop r13
141474de5: pop r14
141474de7: pop r15
141474de9: pop rbp
141474dea: ret 
141474deb: movups xmm0, xmmword ptr [rbp + 0xa0]
141474df2: movaps xmmword ptr [rbp + 0x100], xmm0
141474df9: movups xmm0, xmmword ptr [rbp + 0x60]
141474dfd: movups xmm1, xmmword ptr [rbp + 0x70]
141474e01: movups xmm2, xmmword ptr [rbp + 0x80]
141474e08: movups xmm3, xmmword ptr [rbp + 0x90]
141474e0f: movaps xmmword ptr [rbp + 0xf0], xmm3
141474e16: movaps xmmword ptr [rbp + 0xe0], xmm2
141474e1d: movaps xmmword ptr [rbp + 0xd0], xmm1
141474e24: movaps xmmword ptr [rbp + 0xc0], xmm0
141474e2b: lea rax, [rip + 0x17f05fe] ; 0x142c65430 b'\xf0S\xc6B\x01\x00\x00\x00:\x00\x00\x00\x00\x00\x00\x00Z\x00\x00\x00\x16\x00\x00\x00crates/c'
141474e32: mov qword ptr [rsp + 0x20], rax
141474e37: lea rcx, [rip + 0x17f0072] ; 0x142c64eb0 b'called `Result::unwrap()` on an '
141474e3e: lea r9, [rip + 0x17f009b] ; 0x142c64ee0 b'p\xea|@\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x00\xa5GA\x01\x00\x00\x00'
141474e45: lea r8, [rbp + 0xc0]
141474e4c: mov edx, 0x2b
141474e51: call 0x1428d9760 ; core::result::unwrap_failed
141474e56: ud2 
141474e58: nop dword ptr [rax + rax]
141474e60: mov qword ptr [rsp + 0x10], rdx
141474e65: push rbp
141474e66: push r15
141474e68: push r14
141474e6a: push r13
141474e6c: push r12
141474e6e: push rsi
141474e6f: push rdi
141474e70: push rbx
141474e71: sub rsp, 0x38
141474e75: lea rbp, [rdx + 0x80]
141474e7c: lea rcx, [rbp + 0xc0]
141474e83: call 0x1407cea70 ; core::ptr::drop_in_place<enum2$<rhapsody::raw::error::BackendError> >
141474e88: nop 
141474e89: add rsp, 0x38
141474e8d: pop rbx
141474e8e: pop rdi
141474e8f: pop rsi
141474e90: pop r12
141474e92: pop r13
141474e94: pop r14
141474e96: pop r15
141474e98: pop rbp
141474e99: ret 
141474e9a: nop word ptr [rax + rax]
141474ea0: mov qword ptr [rsp + 0x10], rdx
141474ea5: push rbp
141474ea6: push r15
141474ea8: push r14
141474eaa: push r13
141474eac: push r12
141474eae: push rsi
141474eaf: push rdi
141474eb0: push rbx
141474eb1: sub rsp, 0x38
141474eb5: lea rbp, [rdx + 0x80]
141474ebc: lea rcx, [rbp + 0xc0]
141474ec3: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
141474ec8: nop 
141474ec9: add rsp, 0x38
141474ecd: pop rbx
141474ece: pop rdi
141474ecf: pop rsi
141474ed0: pop r12
141474ed2: pop r13
141474ed4: pop r14
141474ed6: pop r15
141474ed8: pop rbp
141474ed9: ret 
