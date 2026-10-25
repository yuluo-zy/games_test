1416144a0: push rbp
1416144a1: push r15
1416144a3: push r14
1416144a5: push r12
1416144a7: push rsi
1416144a8: push rdi
1416144a9: push rbx
1416144aa: sub rsp, 0x100
1416144b1: lea rbp, [rsp + 0x80]
1416144b9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1416144c1: mov rbx, r8
1416144c4: mov rdi, rdx
1416144c7: mov rsi, rcx
1416144ca: test r8, r8
1416144cd: je 0x141614562
1416144d3: mov r14, r9
1416144d6: mov rax, r9
1416144d9: and rax, 0x1f
1416144dd: mov qword ptr [rbp - 0x40], rax
1416144e1: jne 0x1416145ee
1416144e7: mov rax, rbx
1416144ea: and rax, 3
1416144ee: mov qword ptr [rbp - 0x40], rax
1416144f2: jne 0x1416145ff
1416144f8: cmp r14, 0x1837e0
1416144ff: jb 0x141614565
141614501: lea rax, [rip + 0x14899b5] ; 0x142a9debd b'country_core::resources::rocky_t'
141614508: mov qword ptr [rbp - 0x40], rax
14161450c: mov qword ptr [rbp - 0x38], 0x40
141614514: lea rax, [rbp - 0x40]
141614518: mov qword ptr [rbp + 0x68], rax
14161451c: lea rax, [rip - 0x150a693] ; 0x140109e90 b'I\x89\xd0H\x8b\x01H\x8bQ\x08H\x89\xc1\xe9N\xe8\xc7\x00\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc'
141614523: mov qword ptr [rbp + 0x70], rax
141614527: lea rax, [rip + 0x166b29a] ; 0x142c7f7c8 b'\xa0\xf7\xc7B\x01\x00\x00\x00(\x00\x00\x00\x00\x00\x00\x00`\xf7\xc7B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00'
14161452e: mov qword ptr [rbp + 0x10], rax
141614532: mov qword ptr [rbp + 0x18], 1
14161453a: mov qword ptr [rbp + 0x30], 0
141614542: lea rax, [rbp + 0x68]
141614546: mov qword ptr [rbp + 0x20], rax
14161454a: mov qword ptr [rbp + 0x28], 1
141614552: lea rdx, [rip + 0x166b27f] ; 0x142c7f7d8 b'`\xf7\xc7B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00e\x00\x00\x00\r\x00\x00\x00Velocity'
141614559: lea rcx, [rbp + 0x10]
14161455d: call 0x1428d9390 ; core::panicking::panic_fmt
141614562: xor r14d, r14d
141614565: lea r15, [rbp + 0x10]
141614569: mov edx, 0x1837c0
14161456e: mov rcx, r15
141614571: mov r8d, 0x22
141614577: mov r9b, 1
14161457a: call 0x140bd4de0 ; rhapsody::raw::buffer::BufferDesc::new
14161457f: lea r12, [rbp + 0x68]
141614583: mov r8d, 4
141614589: mov rcx, r12
14161458c: mov rdx, r15
14161458f: call 0x140bd4f20 ; rhapsody::raw::buffer::BufferDesc::alignment
141614594: mov qword ptr [rsp + 0x30], r14
141614599: mov qword ptr [rsp + 0x28], rbx
14161459e: mov qword ptr [rsp + 0x20], r12
1416145a3: lea r8, [rip + 0x166bbc3] ; 0x142c8016d b'Rocky Terrain TrianglesPlasterIn'
1416145aa: lea rcx, [rbp - 0x40]
1416145ae: mov r9d, 0x17
1416145b4: mov rdx, rdi
1416145b7: call 0x1416a2dc0 ; rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
1416145bc: cmp dword ptr [rbp - 0x40], 0xb
1416145c0: jne 0x141614629
1416145c2: mov rax, qword ptr [rbp - 0x18]
1416145c6: mov qword ptr [rsi + 0x20], rax
1416145ca: movups xmm0, xmmword ptr [rbp - 0x38]
1416145ce: movups xmm1, xmmword ptr [rbp - 0x28]
1416145d2: movups xmmword ptr [rsi + 0x10], xmm1
1416145d6: movups xmmword ptr [rsi], xmm0
1416145d9: mov rax, rsi
1416145dc: add rsp, 0x100
1416145e3: pop rbx
1416145e4: pop rdi
1416145e5: pop rsi
1416145e6: pop r12
1416145e8: pop r14
1416145ea: pop r15
1416145ec: pop rbp
1416145ed: ret 
1416145ee: mov qword ptr [rbp + 0x10], 0
1416145f6: lea rax, [rip + 0x16b594b] ; 0x142cc9f48 b'\xf0\x9e\xccB\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x05\x00\x00\x00\xf0\x9e\xccB\x01\x00\x00\x00'
1416145fd: jmp 0x14161460e
1416145ff: mov qword ptr [rbp + 0x10], 0
141614607: lea rax, [rip + 0x16b5952] ; 0x142cc9f60 b'\xf0\x9e\xccB\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x1b\x00\x00\x00\x05\x00\x00\x00no entry'
14161460e: mov qword ptr [rsp + 0x20], rax
141614613: lea r8, [rip + 0x16b5926] ; 0x142cc9f40 b'\x00\x00\x00\x00\x00\x00\x00\x00\xf0\x9e\xccB\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x1a\x00\x00\x00\x05\x00\x00\x00'
14161461a: lea rdx, [rbp - 0x40]
14161461e: lea r9, [rbp + 0x10]
141614622: xor ecx, ecx
141614624: call 0x1428d9596 ; core::panicking::assert_failed<usize,usize>
141614629: movups xmm0, xmmword ptr [rbp]
14161462d: movaps xmmword ptr [rbp + 0x50], xmm0
141614631: movups xmm0, xmmword ptr [rbp - 0x40]
141614635: movups xmm1, xmmword ptr [rbp - 0x30]
141614639: movups xmm2, xmmword ptr [rbp - 0x20]
14161463d: movups xmm3, xmmword ptr [rbp - 0x10]
141614641: movaps xmmword ptr [rbp + 0x40], xmm3
141614645: movaps xmmword ptr [rbp + 0x30], xmm2
141614649: movaps xmmword ptr [rbp + 0x20], xmm1
14161464d: movaps xmmword ptr [rbp + 0x10], xmm0
141614651: lea rax, [rip + 0x166bb50] ; 0x142c801a8 b'`\xf7\xc7B\x01\x00\x00\x00&\x00\x00\x00\x00\x00\x00\x00\x94\x00\x00\x00\x0e\x00\x00\x00`\xf7\xc7B\x01\x00\x00\x00'
141614658: mov qword ptr [rsp + 0x20], rax
14161465d: lea rcx, [rip + 0x166a484] ; 0x142c7eae8 b'called `Result::unwrap()` on an '
141614664: lea r9, [rip + 0x166a45d] ; 0x142c7eac8 b'p\xea|@\x01\x00\x00\x00P\x00\x00\x00\x00\x00\x00\x00\x08\x00\x00\x00\x00\x00\x00\x00\x90\xf1`A\x01\x00\x00\x00'
14161466b: lea r8, [rbp + 0x10]
14161466f: mov edx, 0x2b
141614674: call 0x1428d9760 ; core::result::unwrap_failed
141614679: ud2 
14161467b: nop dword ptr [rax + rax]
141614680: mov qword ptr [rsp + 0x10], rdx
141614685: push rbp
141614686: push r15
141614688: push r14
14161468a: push r12
14161468c: push rsi
14161468d: push rdi
14161468e: push rbx
14161468f: sub rsp, 0x40
141614693: lea rbp, [rdx + 0x80]
14161469a: lea rcx, [rbp + 0x10]
14161469e: call 0x1407cea70 ; core::ptr::drop_in_place<enum2$<rhapsody::raw::error::BackendError> >
1416146a3: nop 
1416146a4: add rsp, 0x40
1416146a8: pop rbx
1416146a9: pop rdi
1416146aa: pop rsi
1416146ab: pop r12
1416146ad: pop r14
1416146af: pop r15
1416146b1: pop rbp
1416146b2: ret 
