1408ccc90: push r15
1408ccc92: push r14
1408ccc94: push r13
1408ccc96: push r12
1408ccc98: push rsi
1408ccc99: push rdi
1408ccc9a: push rbp
1408ccc9b: push rbx
1408ccc9c: sub rsp, 0x1f8
1408ccca3: movaps xmmword ptr [rsp + 0x1e0], xmm14
1408cccac: movaps xmmword ptr [rsp + 0x1d0], xmm13
1408cccb5: movaps xmmword ptr [rsp + 0x1c0], xmm12
1408cccbe: movaps xmmword ptr [rsp + 0x1b0], xmm11
1408cccc7: movaps xmmword ptr [rsp + 0x1a0], xmm10
1408cccd0: movaps xmmword ptr [rsp + 0x190], xmm9
1408cccd9: movaps xmmword ptr [rsp + 0x180], xmm8
1408ccce2: movaps xmmword ptr [rsp + 0x170], xmm7
1408cccea: movaps xmmword ptr [rsp + 0x160], xmm6
1408cccf2: movaps xmm6, xmm3
1408cccf5: mov qword ptr [rsp + 0x60], rcx
1408cccfa: movabs rax, 0x20b040607030100
1408ccd04: mov qword ptr [rsp + 0xd8], rax
1408ccd0c: mov word ptr [rsp + 0xe0], 0xd0c
1408ccd16: lea r15, [rdx + 0x20]
1408ccd1a: lea rdi, [rdx + 0x3d80]
1408ccd21: movss xmm7, dword ptr [r8 + 8]
1408ccd27: movsd xmm8, qword ptr [r8]
1408ccd2c: movsd xmm9, qword ptr [r8 + 0xc]
1408ccd32: movss xmm10, dword ptr [r8 + 0x14]
1408ccd38: mov r12, qword ptr [rdx + 8]
1408ccd3c: mov rax, qword ptr [rdx + 0x10]
1408ccd40: mov qword ptr [rsp + 0x68], rax
1408ccd45: mov esi, 2
1408ccd4a: movss xmm12, dword ptr [rip + 0x2058bb1]
1408ccd53: mov ebp, 0x10
1408ccd58: movaps xmm11, xmmword ptr [rip + 0x2071360]
1408ccd60: lea r13, [rsp + 0xa0]
1408ccd68: lea rbx, [rsp + 0x120]
1408ccd70: jmp 0x1408ccdc5
1408ccd72: nop word ptr cs:[rax + rax]
1408ccd80: lea rax, [r12 + rcx*8]
1408ccd84: movsldup xmm13, xmm12
1408ccd89: mulps xmm13, xmm9
1408ccd8d: movaps xmm14, xmm10
1408ccd91: mulss xmm14, xmm12
1408ccd96: addps xmm13, xmm8
1408ccd9a: addss xmm14, xmm7
1408ccd9f: mov rcx, qword ptr [rax + 0x10]
1408ccda3: mov qword ptr [rsp + 0x90], rcx
1408ccdab: movups xmm0, xmmword ptr [rax]
1408ccdae: movaps xmmword ptr [rsp + 0x80], xmm0
1408ccdb6: xor esi, esi
1408ccdb8: inc rbp
1408ccdbb: cmp rbp, 0x1a
1408ccdbf: je 0x1408ccf13
1408ccdc5: movzx eax, byte ptr [rsp + rbp + 0xc8]
1408ccdcd: imul rax, rax, 0x390
1408ccdd4: lea r14, [r15 + rax]
1408ccdd8: lea rdx, [r15 + rax]
1408ccddc: add rdx, 0x30
1408ccde0: movaps xmmword ptr [rsp + 0xa0], xmm11
1408ccde9: movlps qword ptr [rsp + 0xb0], xmm8
1408ccdf2: movss dword ptr [rsp + 0xb8], xmm7
1408ccdfb: movlps qword ptr [rsp + 0x70], xmm9
1408cce01: movss dword ptr [rsp + 0x78], xmm10
1408cce08: movss dword ptr [rsp + 0x50], xmm6
1408cce0e: movss dword ptr [rsp + 0x54], xmm12
1408cce15: mov dword ptr [rsp + 0x58], 0
1408cce1d: mov word ptr [rsp + 0x5c], 0x101
1408cce24: mov qword ptr [rsp + 0xf8], 0
1408cce30: mov dword ptr [rsp + 0x108], 0
1408cce3b: mov dword ptr [rsp + 0x114], 0
1408cce46: mov qword ptr [rsp + 0xe8], 0
1408cce52: lea rax, [rsp + 0xe8]
1408cce5a: mov qword ptr [rsp + 0x48], rax
1408cce5f: lea rax, [rsp + 0x54]
1408cce64: mov qword ptr [rsp + 0x40], rax
1408cce69: lea rax, [rip + 0x21d6bd0]
1408cce70: mov qword ptr [rsp + 0x38], rax
1408cce75: lea rax, [rsp + 0x50]
1408cce7a: mov qword ptr [rsp + 0x30], rax
1408cce7f: lea rax, [rsp + 0x70]
1408cce84: mov qword ptr [rsp + 0x28], rax
1408cce89: mov qword ptr [rsp + 0x20], r13
1408cce8e: mov rcx, rbx
1408cce91: mov r8, rdi
1408cce94: mov r9, r14
1408cce97: call 0x140b947d0 ; rapier3d::pipeline::query_pipeline::QueryPipeline::cast_shape
1408cce9c: cmp byte ptr [rsp + 0x15c], 4
1408ccea4: je 0x1408ccdb8
1408cceaa: mov edx, dword ptr [rsp + 0x120]
1408cceb1: mov r8d, dword ptr [rsp + 0x124]
1408cceb9: movss xmm12, dword ptr [rsp + 0x128]
1408ccec3: mov rcx, r14
1408ccec6: call 0x140b93940 ; rapier3d::geometry::collider_set::ColliderSet::get_mut_internal
1408ccecb: test rax, rax
1408ccece: je 0x1408ccfaf
1408cced4: mov rax, qword ptr [rax]
1408cced7: mov ecx, eax
1408cced9: cmp qword ptr [rsp + 0x68], rcx
1408ccede: jbe 0x1408ccefb
1408ccee0: test r12, r12
1408ccee3: je 0x1408ccefb
1408ccee5: shr rax, 0x20
1408ccee9: or eax, 1
1408cceec: lea rcx, [rcx + rcx*4]
1408ccef0: cmp dword ptr [r12 + rcx*8 + 0x20], eax
1408ccef5: je 0x1408ccd80
1408ccefb: lea rcx, [rip + 0x23a09de]
1408ccf02: lea r8, [rip + 0x21d6cc7]
1408ccf09: mov edx, 0x18
1408ccf0e: call 0x1428d8725 ; std::panicking::begin_panic<ref$<str$> >
1408ccf13: mov rcx, qword ptr [rsp + 0x90]
1408ccf1b: mov rax, qword ptr [rsp + 0x60]
1408ccf20: mov qword ptr [rax + 0x10], rcx
1408ccf24: movaps xmm0, xmmword ptr [rsp + 0x80]
1408ccf2c: movups xmmword ptr [rax], xmm0
1408ccf2f: movlps qword ptr [rax + 0x18], xmm13
1408ccf34: movss dword ptr [rax + 0x20], xmm14
1408ccf3a: mov qword ptr [rax + 0x24], 0
1408ccf42: mov dword ptr [rax + 0x2c], 0
1408ccf49: mov dword ptr [rax + 0x30], esi
1408ccf4c: movaps xmm6, xmmword ptr [rsp + 0x160]
1408ccf54: movaps xmm7, xmmword ptr [rsp + 0x170]
1408ccf5c: movaps xmm8, xmmword ptr [rsp + 0x180]
1408ccf65: movaps xmm9, xmmword ptr [rsp + 0x190]
1408ccf6e: movaps xmm10, xmmword ptr [rsp + 0x1a0]
1408ccf77: movaps xmm11, xmmword ptr [rsp + 0x1b0]
1408ccf80: movaps xmm12, xmmword ptr [rsp + 0x1c0]
1408ccf89: movaps xmm13, xmmword ptr [rsp + 0x1d0]
1408ccf92: movaps xmm14, xmmword ptr [rsp + 0x1e0]
1408ccf9b: add rsp, 0x1f8
1408ccfa2: pop rbx
1408ccfa3: pop rbp
1408ccfa4: pop rdi
1408ccfa5: pop rsi
1408ccfa6: pop r12
1408ccfa8: pop r13
1408ccfaa: pop r14
1408ccfac: pop r15
1408ccfae: ret 
1408ccfaf: lea rcx, [rip + 0x21d6c02]
1408ccfb6: call 0x1428d9310 ; core::option::unwrap_failed
1408ccfbb: int3 
