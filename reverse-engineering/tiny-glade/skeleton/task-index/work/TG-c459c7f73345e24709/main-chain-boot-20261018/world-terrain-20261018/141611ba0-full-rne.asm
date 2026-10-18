141611ba0: push rbp
141611ba1: push r15
141611ba3: push r14
141611ba5: push r12
141611ba7: push rsi
141611ba8: push rdi
141611ba9: push rbx
141611baa: sub rsp, 0x100
141611bb1: lea rbp, [rsp + 0x80]
141611bb9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141611bc1: mov rbx, r8
141611bc4: mov rdi, rdx
141611bc7: mov rsi, rcx
141611bca: test r8, r8
141611bcd: je 0x141611c2b
141611bcf: movabs rcx, 0xaaaaaaaaaaaaaaab
141611bd9: mov rax, r9
141611bdc: mul rcx
141611bdf: shr rdx, 1
141611be2: and rdx, 0xfffffffffffffff0
141611be6: lea rax, [rdx + rdx*2]
141611bea: mov rcx, r9
141611bed: sub rcx, rax
141611bf0: mov qword ptr [rbp - 0x40], rcx
141611bf4: jne 0x141611cbc
141611bfa: mov rax, rbx
141611bfd: and rax, 3
141611c01: mov qword ptr [rbp - 0x40], rax
141611c05: jne 0x141611ccd
141611c0b: cmp r9, 0x6cfb10
141611c12: jae 0x141611d49
141611c18: mov eax, r9d
141611c1b: mov r12d, 0xaaaaaaab
141611c21: imul r12, rax
141611c25: shr r12, 0x25
141611c29: jmp 0x141611c2b
141611c2b: lea r14, [rbp + 0x10]
141611c2f: mov edx, 0x6cfae0
141611c34: mov rcx, r14
141611c37: mov r8d, 0x22
141611c3d: mov r9b, 1
141611c40: call 0x140bd4de0 ; rhapsody::raw::buffer::BufferDesc::new
141611c45: lea r15, [rbp + 0x68]
141611c49: mov r8d, 4
141611c4f: mov rcx, r15
141611c52: mov rdx, r14
141611c55: call 0x140bd4f20 ; rhapsody::raw::buffer::BufferDesc::alignment
141611c5a: shl r12, 4
141611c5e: lea rax, [r12 + r12*2]
141611c62: mov qword ptr [rsp + 0x30], rax
141611c67: mov qword ptr [rsp + 0x28], rbx
141611c6c: mov qword ptr [rsp + 0x20], r15
141611c71: lea r8, [rip + 0x166df6b] ; 0x142c7fbe3 b'Rocky Terrain VerticesColorLibra'
141611c78: lea rcx, [rbp - 0x40]
141611c7c: mov r9d, 0x16
141611c82: mov rdx, rdi
141611c85: call 0x1416a2dc0 ; rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
141611c8a: cmp dword ptr [rbp - 0x40], 0xb
141611c8e: jne 0x141611cf7
141611c90: mov rax, qword ptr [rbp - 0x18]
141611c94: mov qword ptr [rsi + 0x20], rax
141611c98: movups xmm0, xmmword ptr [rbp - 0x38]
141611c9c: movups xmm1, xmmword ptr [rbp - 0x28]
141611ca0: movups xmmword ptr [rsi + 0x10], xmm1
141611ca4: movups xmmword ptr [rsi], xmm0
141611ca7: mov rax, rsi
141611caa: add rsp, 0x100
141611cb1: pop rbx
141611cb2: pop rdi
141611cb3: pop rsi
141611cb4: pop r12
141611cb6: pop r14
141611cb8: pop r15
141611cba: pop rbp
141611cbb: ret 
141611cbc: mov qword ptr [rbp + 0x10], 0
141611cc4: lea rax, [rip + 0x16b827d] ; 0x142cc9f48 b'\xf0\x9e\xccB\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x05\x00\x00\x00\xf0\x9e\xccB\x01\x00\x00\x00'
141611ccb: jmp 0x141611cdc
141611ccd: mov qword ptr [rbp + 0x10], 0
141611cd5: lea rax, [rip + 0x16b8284] ; 0x142cc9f60 b'\xf0\x9e\xccB\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x1b\x00\x00\x00\x05\x00\x00\x00no entry'
141611cdc: mov qword ptr [rsp + 0x20], rax
141611ce1: lea r8, [rip + 0x16b8258] ; 0x142cc9f40 b'\x00\x00\x00\x00\x00\x00\x00\x00\xf0\x9e\xccB\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x05\x00\x00\x00'
141611ce8: lea rdx, [rbp - 0x40]
141611cec: lea r9, [rbp + 0x10]
141611cf0: xor ecx, ecx
141611cf2: call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
141611cf7: movups xmm0, xmmword ptr [rbp]
141611cfb: movaps xmmword ptr [rbp + 0x50], xmm0
141611cff: movups xmm0, xmmword ptr [rbp - 0x40]
141611d03: movups xmm1, xmmword ptr [rbp - 0x30]
141611d07: movups xmm2, xmmword ptr [rbp - 0x20]
141611d0b: movups xmm3, xmmword ptr [rbp - 0x10]
141611d0f: movaps xmmword ptr [rbp + 0x40], xmm3
141611d13: movaps xmmword ptr [rbp + 0x30], xmm2
141611d17: movaps xmmword ptr [rbp + 0x20], xmm1
141611d1b: movaps xmmword ptr [rbp + 0x10], xmm0
141611d1f: lea rax, [rip + 0x166e482] ; 0x142c801a8 b'`\xf7\xc7B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00\x94\x00\x00\x00\x0e\x00\x00\x00`\xf7\xc7B\x01\x00\x00\x00'
141611d26: mov qword ptr [rsp + 0x20], rax
141611d2b: lea rcx, [rip + 0x166cdb6] ; 0x142c7eae8 b'called `Result::unwrap()` on an '
141611d32: lea r9, [rip + 0x166cd8f] ; 0x142c7eac8 b'p\xea|@\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x90\xf1`A\x01\x00\x00\x00'
141611d39: lea r8, [rbp + 0x10]
141611d3d: mov edx, 0x2b
141611d42: call 0x1428d9760 ; core::result::unwrap_failed
141611d47: ud2 
141611d49: lea rax, [rip + 0x148cfce] ; 0x142a9ed1e b'country_core::resources::rocky_t'
141611d50: mov qword ptr [rbp - 0x40], rax
141611d54: mov qword ptr [rbp - 0x38], 0x3e
141611d5c: lea rax, [rbp - 0x40]
141611d60: mov qword ptr [rbp + 0x68], rax
141611d64: lea rax, [rip - 0x1507edb] ; 0x140109e90 b'I\x89\xd0H\x8b\x01H\x8bQ\x08H\x89\xc1\xe9N\xe8\xc7\x00\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc'
141611d6b: mov qword ptr [rbp + 0x70], rax
141611d6f: lea rax, [rip + 0x166da52] ; 0x142c7f7c8 b'\xa0\xf7\xc7B\x01\x00\x00\x00(\x00\x00\x00\x00\x00\x00\x00`\xf7\xc7B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00'
141611d76: mov qword ptr [rbp + 0x10], rax
141611d7a: mov qword ptr [rbp + 0x18], 1
141611d82: mov qword ptr [rbp + 0x30], 0
141611d8a: lea rax, [rbp + 0x68]
141611d8e: mov qword ptr [rbp + 0x20], rax
141611d92: mov qword ptr [rbp + 0x28], 1
141611d9a: lea rdx, [rip + 0x166da37] ; 0x142c7f7d8 b'`\xf7\xc7B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00e\x00\x00\x00\r\x00\x00\x00Velocity'
141611da1: lea rcx, [rbp + 0x10]
141611da5: call 0x1428d9390 ; core::panicking::panic_fmt
141611daa: int3 
141611dab: nop dword ptr [rax + rax]
141611db0: mov qword ptr [rsp + 0x10], rdx
141611db5: push rbp
141611db6: push r15
141611db8: push r14
141611dba: push r12
141611dbc: push rsi
141611dbd: push rdi
141611dbe: push rbx
141611dbf: sub rsp, 0x40
141611dc3: lea rbp, [rdx + 0x80]
141611dca: lea rcx, [rbp + 0x10]
141611dce: call 0x1407cea70 ; core::ptr::drop_in_place<enum2$<rhapsody::raw::error::BackendError> >
141611dd3: nop 
141611dd4: add rsp, 0x40
141611dd8: pop rbx
141611dd9: pop rdi
141611dda: pop rsi
141611ddb: pop r12
141611ddd: pop r14
141611ddf: pop r15
141611de1: pop rbp
141611de2: ret 
