140bfe270: push rbp
140bfe271: push r15
140bfe273: push r14
140bfe275: push r13
140bfe277: push r12
140bfe279: push rsi
140bfe27a: push rdi
140bfe27b: push rbx
140bfe27c: sub rsp, 0xf8
140bfe283: lea rbp, [rsp + 0x80]
140bfe28b: mov qword ptr [rbp + 0x70], 0xfffffffffffffffe
140bfe293: test byte ptr [r8], 4
140bfe297: je 0x140bfe514
140bfe29d: mov r15, r8
140bfe2a0: mov rdi, rdx
140bfe2a3: mov rsi, rcx
140bfe2a6: mov rbx, qword ptr [rbp + 0xe0]
140bfe2ad: mov rax, qword ptr [r9]
140bfe2b0: lea rcx, [rip + 0x1f19245] ; 0x142b174fc b'\xb3p\x0e\xfe\xc4m\x0e\xfe\x0fn\x0e\xfe\xaen\x0e\xfe/home/h3'
140bfe2b7: movsxd rax, dword ptr [rcx + rax*4]
140bfe2bb: add rax, rcx
140bfe2be: jmp rax
140bfe2c0: mov r14, qword ptr [rbx]
140bfe2c3: add rbx, 8
140bfe2c7: mov ecx, dword ptr [r15 + 8]
140bfe2cb: call 0x141738fe0 ; rhapsody::raw::format::vk_format_valid_aspect_flags
140bfe2d0: mov dword ptr [rbp + 0x60], eax
140bfe2d3: lea rax, [r15 + 0x1c]
140bfe2d7: add r15, 0x18
140bfe2db: mov qword ptr [rbp + 8], rbx
140bfe2df: mov qword ptr [rbp + 0x10], r14
140bfe2e3: lea rcx, [rbp + 0x60]
140bfe2e7: mov qword ptr [rbp + 0x18], rcx
140bfe2eb: mov qword ptr [rbp + 0x20], rdi
140bfe2ef: mov qword ptr [rbp + 0x28], rax
140bfe2f3: mov qword ptr [rbp + 0x30], r15
140bfe2f7: lea r8, [rbp + 8]
140bfe2fb: mov rcx, rsi
140bfe2fe: mov rdx, rdi
140bfe301: call 0x140c03fa0 ; rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::init_texture_data::closure_env$1>
140bfe306: jmp 0x140bfe4fd
140bfe30b: mov r13, qword ptr [r9 + 8]
140bfe30f: mov r12, qword ptr [r9 + 0x10]
140bfe313: mov r14d, dword ptr [r15 + 8]
140bfe317: mov ecx, r14d
140bfe31a: call 0x1417390e0 ; rhapsody::raw::format::vk_format_block_bytes
140bfe31f: mov qword ptr [rbp + 0x60], rax
140bfe323: cmp rax, 1
140bfe327: jne 0x140bfe52c
140bfe32d: mov qword ptr [rbp - 0x28], r12
140bfe331: movzx eax, byte ptr [r15 + 0x1c]
140bfe336: mov qword ptr [rbp + 0x60], rax
140bfe33a: cmp r12, rax
140bfe33d: jne 0x140bfe588
140bfe343: mov eax, dword ptr [r15 + 0x14]
140bfe347: mov dword ptr [rbp + 0x68], eax
140bfe34a: mov rax, qword ptr [r15 + 0xc]
140bfe34e: mov qword ptr [rbp + 0x60], rax
140bfe352: lea rcx, [rbp + 0x20]
140bfe356: mov word ptr [rsp + 0x20], 0x101
140bfe35d: lea rdx, [rbp + 0x60]
140bfe361: mov r8d, r12d
140bfe364: mov r9b, 1
140bfe367: call 0x140bd5a80 ; rhapsody::raw::texture::MipChainIter::new
140bfe36c: mov rax, r12
140bfe36f: shl rax, 4
140bfe373: add rax, r13
140bfe376: xorps xmm0, xmm0
140bfe379: movups xmmword ptr [rbp + 0x30], xmm0
140bfe37d: mov qword ptr [rbp + 0x40], 0
140bfe385: mov qword ptr [rbp + 8], r13
140bfe389: mov qword ptr [rbp + 0x10], rax
140bfe38d: mov qword ptr [rbp + 0x18], r12
140bfe391: lea r8, [rip + 0x1f16cf8] ; 0x142b15090 b'8P\xb1B\x01\x00\x00\x00X\x00\x00\x00\x00\x00\x00\x00\xb3\x07\x00\x00\t\x00\x00\x00'
140bfe398: lea rcx, [rbp - 0x10]
140bfe39c: lea rdx, [rbp + 8]
140bfe3a0: call 0x1418b77a0 ; alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::texture::TextureUpdateData,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::take::Take<rhapsody::raw::texture::MipChainIter>,core::slice::iter::Iter<ref$<slice2$<u8> > > >,rhapsody::raw::texture::impl$9::init_texture_data::closure_env$0> >
140bfe3a5: jmp 0x140bfe45b
140bfe3aa: mov rax, qword ptr [r9 + 8]
140bfe3ae: mov qword ptr [rbp + 0x58], rax
140bfe3b2: mov r13, qword ptr [r9 + 0x10]
140bfe3b6: mov r14d, dword ptr [r15 + 8]
140bfe3ba: lea rcx, [rbp - 0x34]
140bfe3be: mov edx, r14d
140bfe3c1: call 0x141739060 ; rhapsody::raw::format::vk_format_info
140bfe3c6: mov r9d, dword ptr [rbp - 0x30]
140bfe3ca: mov eax, dword ptr [r15 + 0x14]
140bfe3ce: mov dword ptr [rbp - 0x18], eax
140bfe3d1: mov rax, qword ptr [r15 + 0xc]
140bfe3d5: mov qword ptr [rbp - 0x20], rax
140bfe3d9: mov r12d, dword ptr [r15 + 0x18]
140bfe3dd: cmp r12d, 0x10000
140bfe3e4: jae 0x140bfe5c7
140bfe3ea: cmp r9d, 0x100
140bfe3f1: jae 0x140bfe5d0
140bfe3f7: test r13, r13
140bfe3fa: je 0x140bfe570
140bfe400: movzx r8d, byte ptr [r15 + 0x1c]
140bfe405: movzx eax, word ptr [rbp - 0x2c]
140bfe409: mov word ptr [rsp + 0x20], ax
140bfe40e: lea rcx, [rbp + 0x60]
140bfe412: lea rdx, [rbp - 0x20]
140bfe416: call 0x140bd5a80 ; rhapsody::raw::texture::MipChainIter::new
140bfe41b: mov rax, qword ptr [rbp + 0x58]
140bfe41f: mov qword ptr [rbp + 8], rax
140bfe423: mov qword ptr [rbp + 0x10], r13
140bfe427: mov rax, qword ptr [rbp + 0x60]
140bfe42b: mov qword ptr [rbp + 0x18], rax
140bfe42f: mov ecx, dword ptr [rbp + 0x68]
140bfe432: mov dword ptr [rbp + 0x20], ecx
140bfe435: mov qword ptr [rbp + 0x24], rax
140bfe439: mov dword ptr [rbp + 0x2c], ecx
140bfe43c: mov word ptr [rbp + 0x30], 0
140bfe442: mov word ptr [rbp + 0x32], r12w
140bfe447: lea r8, [rip + 0x1f16c42] ; 0x142b15090 b'8P\xb1B\x01\x00\x00\x00X\x00\x00\x00\x00\x00\x00\x00\xb3\x07\x00\x00\t\x00\x00\x00'
140bfe44e: lea rcx, [rbp - 0x10]
140bfe452: lea rdx, [rbp + 8]
140bfe456: call 0x1418b8170 ; alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::texture::TextureUpdateData,rhapsody::raw::texture::LinearInitialDataIter>
140bfe45b: mov r9, qword ptr [rbx]
140bfe45e: add rbx, 8
140bfe462: mov rcx, qword ptr [rbp - 8]
140bfe466: mov rax, qword ptr [rbp]
140bfe46a: mov qword ptr [rsp + 0x30], rax
140bfe46f: mov qword ptr [rbp + 0x58], rcx
140bfe473: mov qword ptr [rsp + 0x28], rcx
140bfe478: mov dword ptr [rsp + 0x20], r14d
140bfe47d: mov byte ptr [rsp + 0x40], 0x22
140bfe482: mov byte ptr [rsp + 0x38], 0
140bfe487: lea rcx, [rbp + 8]
140bfe48b: mov rdx, rdi
140bfe48e: mov r8, rbx
140bfe491: call 0x140bfeac0 ; rhapsody::raw::device::GraphicsDevice::update_texture_impl
140bfe496: mov rax, qword ptr [rbp + 8]
140bfe49a: cmp rax, 0xb
140bfe49e: jne 0x140bfe4b2
140bfe4a0: mov qword ptr [rsi], 0xb
140bfe4a7: mov rax, qword ptr [rbp - 0x10]
140bfe4ab: test rax, rax
140bfe4ae: jne 0x140bfe4e6
140bfe4b0: jmp 0x140bfe4fd
140bfe4b2: mov rcx, qword ptr [rbp + 0x50]
140bfe4b6: mov qword ptr [rsi + 0x48], rcx
140bfe4ba: movups xmm0, xmmword ptr [rbp + 0x10]
140bfe4be: movups xmm1, xmmword ptr [rbp + 0x20]
140bfe4c2: movups xmm2, xmmword ptr [rbp + 0x30]
140bfe4c6: movups xmm3, xmmword ptr [rbp + 0x40]
140bfe4ca: movups xmmword ptr [rsi + 0x38], xmm3
140bfe4ce: movups xmmword ptr [rsi + 0x28], xmm2
140bfe4d2: movups xmmword ptr [rsi + 0x18], xmm1
140bfe4d6: movups xmmword ptr [rsi + 8], xmm0
140bfe4da: mov qword ptr [rsi], rax
140bfe4dd: mov rax, qword ptr [rbp - 0x10]
140bfe4e1: test rax, rax
140bfe4e4: je 0x140bfe4fd
140bfe4e6: shl rax, 4
140bfe4ea: lea rdx, [rax + rax*2]
140bfe4ee: mov r8d, 8
140bfe4f4: mov rcx, qword ptr [rbp + 0x58]
140bfe4f8: call 0x140613c20 ; ?
140bfe4fd: mov rax, rsi
140bfe500: add rsp, 0xf8
140bfe507: pop rbx
140bfe508: pop rdi
140bfe509: pop rsi
140bfe50a: pop r12
140bfe50c: pop r13
140bfe50e: pop r14
140bfe510: pop r15
140bfe512: pop rbp
140bfe513: ret 
140bfe514: lea rcx, [rip + 0x1f18add] ; 0x142b16ff8 b'assertion failed: desc.u'
140bfe51b: lea r8, [rip + 0x1f18b3e] ; 0x142b17060 b';p\xb1B\x01\x00\x00\x00"\x00\x00\x00\x00\x00\x00\x00\x86\x02\x00\x00\t\x00\x00\x00'
140bfe522: mov edx, 0x43
140bfe527: call 0x1428d9430 ; core::panicking::panic
140bfe52c: lea rax, [rip + 0x1f18b65] ; 0x142b17098 b'\x90p\xb1B\x01\x00\x00\x00\x04\x00\x00\x00\x00\x00\x00\x00;p\xb1B\x01\x00\x00\x00'
140bfe533: mov qword ptr [rbp + 8], rax
140bfe537: mov qword ptr [rbp + 0x10], 1
140bfe53f: mov qword ptr [rbp + 0x18], 8
140bfe547: xorps xmm0, xmm0
140bfe54a: movups xmmword ptr [rbp + 0x20], xmm0
140bfe54e: lea rax, [rip + 0x1f18b53] ; 0x142b170a8 b';p\xb1B\x01\x00\x00\x00"\x00\x00\x00\x00\x00\x00\x00\x9a\x02\x00\x00\x11\x00\x00\x00'
140bfe555: mov qword ptr [rsp + 0x20], rax
140bfe55a: lea rdx, [rip + 0x1f171ff] ; 0x142b15760 b'\x01\x00\x00\x00\x00\x00\x00\x00\x88O\xb1B\x01\x00\x00\x00O\x00\x00\x00\x00\x00\x00\x00'
140bfe561: lea r8, [rbp + 0x60]
140bfe565: lea r9, [rbp + 8]
140bfe569: xor ecx, ecx
140bfe56b: call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
140bfe570: lea rcx, [rip + 0x1f11569] ; 0x142b0fae0 b'assertion failed: !data.'
140bfe577: lea r8, [rip + 0x1f11772] ; 0x142b0fcf0 b'\xff\xf9\xb0B\x01\x00\x00\x00"\x00\x00\x00\x00\x00\x00\x00\xea\x01\x00\x00\t\x00\x00\x00'
140bfe57e: mov edx, 0x22
140bfe583: call 0x1428d9430 ; core::panicking::panic
140bfe588: mov qword ptr [rbp + 8], 0
140bfe590: lea rax, [rip + 0x1f18b29] ; 0x142b170c0 b';p\xb1B\x01\x00\x00\x00"\x00\x00\x00\x00\x00\x00\x00\x9b\x02\x00\x00\x11\x00\x00\x00'
140bfe597: mov qword ptr [rsp + 0x20], rax
140bfe59c: lea rdx, [rbp - 0x28]
140bfe5a0: lea r8, [rbp + 0x60]
140bfe5a4: lea r9, [rbp + 8]
140bfe5a8: xor ecx, ecx
140bfe5aa: call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
140bfe5af: lea rcx, [rip + 0x1f179fa] ; 0x142b15fb0 b'internal error: entered '
140bfe5b6: lea r8, [rip + 0x1f18abb] ; 0x142b17078 b';p\xb1B\x01\x00\x00\x00"\x00\x00\x00\x00\x00\x00\x00\x88\x02\x00\x00,\x00\x00\x00'
140bfe5bd: mov edx, 0x28
140bfe5c2: call 0x1428d9430 ; core::panicking::panic
140bfe5c7: lea rax, [rip + 0x1f18b0a] ; 0x142b170d8 b';p\xb1B\x01\x00\x00\x00"\x00\x00\x00\x00\x00\x00\x00\x93\x02\x00\x004\x00\x00\x00'
140bfe5ce: jmp 0x140bfe5d7
140bfe5d0: lea rax, [rip + 0x1f18b19] ; 0x142b170f0 b';p\xb1B\x01\x00\x00\x00"\x00\x00\x00\x00\x00\x00\x00\x94\x02\x00\x000\x00\x00\x00'
140bfe5d7: mov qword ptr [rsp + 0x20], rax
140bfe5dc: lea rcx, [rip + 0x1f16ba5] ; 0x142b15188 b'called `Result::unwrap()'
140bfe5e3: lea r9, [rip + 0x1f16bce] ; 0x142b151b8 b'\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00'
140bfe5ea: lea r8, [rbp + 8]
140bfe5ee: mov edx, 0x2b
140bfe5f3: call 0x1428d9760 ; core::result::unwrap_failed
140bfe5f8: int3 
140bfe5f9: nop dword ptr [rax]
140bfe600: mov qword ptr [rsp + 0x10], rdx
140bfe605: push rbp
140bfe606: push r15
140bfe608: push r14
140bfe60a: push r13
140bfe60c: push r12
140bfe60e: push rsi
140bfe60f: push rdi
140bfe610: push rbx
140bfe611: sub rsp, 0x48
140bfe615: lea rbp, [rdx + 0x80]
140bfe61c: mov rax, qword ptr [rbp - 0x10]
140bfe620: test rax, rax
140bfe623: je 0x140bfe63c
140bfe625: shl rax, 4
140bfe629: lea rdx, [rax + rax*2]
140bfe62d: mov r8d, 8
140bfe633: mov rcx, qword ptr [rbp + 0x58]
140bfe637: call 0x140613c20 ; ?
140bfe63c: nop 
140bfe63d: add rsp, 0x48
140bfe641: pop rbx
140bfe642: pop rdi
140bfe643: pop rsi
140bfe644: pop r12
140bfe646: pop r13
140bfe648: pop r14
140bfe64a: pop r15
140bfe64c: pop rbp
140bfe64d: ret 
