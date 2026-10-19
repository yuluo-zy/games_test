1407ee6f6: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee6fd: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ee703: 488d05f6c12902 lea rax, [rip + 0x229c1f6]
1407ee70a: 48898518020000 mov qword ptr [rbp + 0x218], rax
1407ee711: 48c785200200000d000000 mov qword ptr [rbp + 0x220], 0xd
1407ee71c: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ee723: e808e5a900 call 0x14128cc30 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
1407ee728: 48898580020000 mov qword ptr [rbp + 0x280], rax
1407ee72f: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee736: 8b70e0 mov esi, dword ptr [rax - 0x20]
1407ee739: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
1407ee740: 488985f8010000 mov qword ptr [rbp + 0x1f8], rax
1407ee747: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
1407ee74e: 488bb808010000 mov rdi, qword ptr [rax + 0x108]
1407ee755: 483bb8f8000000 cmp rdi, qword ptr [rax + 0xf8]
1407ee75c: 7513 jne 0x1407ee771
1407ee75e: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407ee765: 488d15a4c12902 lea rdx, [rip + 0x229c1a4]
1407ee76c: e88fe2f9ff call 0x14078ca00 ; alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::affine3a::Affine3A,u32,array$<f32,2> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc
1407ee771: 0f280568aa2902 movaps xmm0, xmmword ptr [rip + 0x229aa68]
1407ee778: f30f10c6 movss xmm0, xmm6
1407ee77c: 0f28d0 movaps xmm2, xmm0
1407ee77f: 0fc615a9b7290224 shufps xmm2, xmmword ptr [rip + 0x229b7a9], 0x24
1407ee787: 410f28f4 movaps xmm6, xmm12
1407ee78b: 410f59f4 mulps xmm6, xmm12
1407ee78f: f30f16ce movshdup xmm1, xmm6
1407ee793: f30f58ce addss xmm1, xmm6
1407ee797: 0f28de movaps xmm3, xmm6
1407ee79a: 660f15de unpckhpd xmm3, xmm6
1407ee79e: f30f58d9 addss xmm3, xmm1
1407ee7a2: 0fc6db00 shufps xmm3, xmm3, 0
1407ee7a6: 0fc6f6ff shufps xmm6, xmm6, 0xff
1407ee7aa: 0f5cf3 subps xmm6, xmm3
1407ee7ad: 0f59d6 mulps xmm2, xmm6
1407ee7b0: 410f28cc movaps xmm1, xmm12
1407ee7b4: 0f59c8 mulps xmm1, xmm0
1407ee7b7: f30f16d9 movshdup xmm3, xmm1
1407ee7bb: f30f58d9 addss xmm3, xmm1
1407ee7bf: 0f12c9 movhlps xmm1, xmm1
1407ee7c2: f30f58cb addss xmm1, xmm3
1407ee7c6: f30f58c9 addss xmm1, xmm1
1407ee7ca: 0fc6c900 shufps xmm1, xmm1, 0
1407ee7ce: 410f59cc mulps xmm1, xmm12
1407ee7d2: 0f58ca addps xmm1, xmm2
1407ee7d5: 410f28fc movaps xmm7, xmm12
1407ee7d9: 410fc6fcd2 shufps xmm7, xmm12, 0xd2
1407ee7de: 0f28d7 movaps xmm2, xmm7
1407ee7e1: 0f59d0 mulps xmm2, xmm0
1407ee7e4: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407ee7e8: 410f59c4 mulps xmm0, xmm12
1407ee7ec: 0f5cd0 subps xmm2, xmm0
1407ee7ef: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
1407ee7f3: 450f28cc movaps xmm9, xmm12
1407ee7f7: 450f58cc addps xmm9, xmm12
1407ee7fb: 450fc6c9ff shufps xmm9, xmm9, 0xff
1407ee800: 410f59d1 mulps xmm2, xmm9
1407ee804: 0f58d1 addps xmm2, xmm1
1407ee807: 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
1407ee80b: 0f58c2 addps xmm0, xmm2
1407ee80e: 0f12d2 movhlps xmm2, xmm2
1407ee811: f30f589564020000 addss xmm2, dword ptr [rbp + 0x264]
1407ee819: 488b9da8020000 mov rbx, qword ptr [rbp + 0x2a8]
1407ee820: 488b8300010000 mov rax, qword ptr [rbx + 0x100]
1407ee827: 488d0cbf lea rcx, [rdi + rdi*4]
1407ee82b: 48c1e104 shl rcx, 4
1407ee82f: 66c704080200 mov word ptr [rax + rcx], 2
1407ee835: 89740804 mov dword ptr [rax + rcx + 4], esi
1407ee839: 440f11640810 movups xmmword ptr [rax + rcx + 0x10], xmm12
1407ee83f: 0f13440820 movlps qword ptr [rax + rcx + 0x20], xmm0
1407ee844: f30f11540828 movss dword ptr [rax + rcx + 0x28], xmm2
1407ee84a: 488d9520010000 lea rdx, [rbp + 0x120]
1407ee851: f30f6f420c movdqu xmm0, xmmword ptr [rdx + 0xc]
1407ee856: f30f7f44082c movdqu xmmword ptr [rax + rcx + 0x2c], xmm0
1407ee85c: 8b521c mov edx, dword ptr [rdx + 0x1c]
1407ee85f: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ee863: 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
1407ee86a: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407ee86f: 488b95f8010000 mov rdx, qword ptr [rbp + 0x1f8]
1407ee876: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407ee87b: 48ffc7 inc rdi
1407ee87e: 4889bb08010000 mov qword ptr [rbx + 0x108], rdi
1407ee885: f3440f1085d0000000 movss xmm8, dword ptr [rbp + 0xd0]
1407ee88e: f3440f5905058f2902 mulss xmm8, dword ptr [rip + 0x2298f05]
1407ee897: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee89e: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ee8a4: 488d057dc02902 lea rax, [rip + 0x229c07d]
1407ee8ab: 48898518020000 mov qword ptr [rbp + 0x218], rax
1407ee8b2: 48c785200200000c000000 mov qword ptr [rbp + 0x220], 0xc
1407ee8bd: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ee8c4: e867e3a900 call 0x14128cc30 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
1407ee8c9: 48898578020000 mov qword ptr [rbp + 0x278], rax
1407ee8d0: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee8d7: 8b70e0 mov esi, dword ptr [rax - 0x20]
1407ee8da: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
1407ee8e1: 48898580020000 mov qword ptr [rbp + 0x280], rax
1407ee8e8: 488bbb08010000 mov rdi, qword ptr [rbx + 0x108]
1407ee8ef: 483bbbf8000000 cmp rdi, qword ptr [rbx + 0xf8]
1407ee8f6: 7513 jne 0x1407ee90b
1407ee8f8: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407ee8ff: 488d1532c02902 lea rdx, [rip + 0x229c032]
1407ee906: e8f5e0f9ff call 0x14078ca00 ; alloc::raw_vec::RawVec::grow_one<tuple$<glam::f32::affine3a::Affine3A,u32,array$<f32,2> >,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_decorator::decorator_visual::flag_connections::CandidateInfo,alloc
1407ee90b: 0f2805cea82902 movaps xmm0, xmmword ptr [rip + 0x229a8ce]
1407ee912: f3410f10c0 movss xmm0, xmm8
1407ee917: 0f28c8 movaps xmm1, xmm0
1407ee91a: 0fc60d0eb6290224 shufps xmm1, xmmword ptr [rip + 0x229b60e], 0x24
1407ee922: 0f59f1 mulps xmm6, xmm1
1407ee925: 410f28cc movaps xmm1, xmm12
1407ee929: 0f59c8 mulps xmm1, xmm0
1407ee92c: f30f16d1 movshdup xmm2, xmm1
1407ee930: f30f58d1 addss xmm2, xmm1
1407ee934: 0f12c9 movhlps xmm1, xmm1
1407ee937: f30f58ca addss xmm1, xmm2
1407ee93b: f30f58c9 addss xmm1, xmm1
1407ee93f: 0fc6c900 shufps xmm1, xmm1, 0
1407ee943: 410f59cc mulps xmm1, xmm12
1407ee947: 0f58ce addps xmm1, xmm6
1407ee94a: 0f59f8 mulps xmm7, xmm0
1407ee94d: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407ee951: 410f59c4 mulps xmm0, xmm12
1407ee955: 0f5cf8 subps xmm7, xmm0
1407ee958: 0fc6ffd2 shufps xmm7, xmm7, 0xd2
1407ee95c: 440f59cf mulps xmm9, xmm7
1407ee960: 440f58c9 addps xmm9, xmm1
1407ee964: 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
1407ee968: 410f58c1 addps xmm0, xmm9
1407ee96c: 450f12c9 movhlps xmm9, xmm9
1407ee970: f30f108d64020000 movss xmm1, dword ptr [rbp + 0x264]
1407ee978: f3410f58c9 addss xmm1, xmm9
1407ee97d: 4c8b85a8020000 mov r8, qword ptr [rbp + 0x2a8]
1407ee984: 498b8000010000 mov rax, qword ptr [r8 + 0x100]
1407ee98b: 488d0cbf lea rcx, [rdi + rdi*4]
1407ee98f: 48c1e104 shl rcx, 4
1407ee993: 66c704080201 mov word ptr [rax + rcx], 0x102
1407ee999: 89740804 mov dword ptr [rax + rcx + 4], esi
1407ee99d: 440f11640810 movups xmmword ptr [rax + rcx + 0x10], xmm12
1407ee9a3: 0f13440820 movlps qword ptr [rax + rcx + 0x20], xmm0
1407ee9a8: f30f114c0828 movss dword ptr [rax + rcx + 0x28], xmm1
1407ee9ae: 488d9520010000 lea rdx, [rbp + 0x120]
1407ee9b5: f30f6f420c movdqu xmm0, xmmword ptr [rdx + 0xc]
1407ee9ba: f30f7f44082c movdqu xmmword ptr [rax + rcx + 0x2c], xmm0
1407ee9c0: 8b521c mov edx, dword ptr [rdx + 0x1c]
1407ee9c3: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ee9c7: 488b9578020000 mov rdx, qword ptr [rbp + 0x278]
1407ee9ce: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407ee9d3: 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
1407ee9da: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407ee9df: 48ffc7 inc rdi
1407ee9e2: 4989b808010000 mov qword ptr [r8 + 0x108], rdi
1407ee9e9: e93cdaffff jmp 0x1407ec42a ; unknown
