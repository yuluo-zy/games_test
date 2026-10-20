140b067d0: push r15
140b067d2: push r14
140b067d4: push rsi
140b067d5: push rdi
140b067d6: push rbx
140b067d7: sub rsp, 0x40
140b067db: movaps xmmword ptr [rsp + 0x30], xmm7
140b067e0: movaps xmmword ptr [rsp + 0x20], xmm6
140b067e5: mov rdi, rdx
140b067e8: mov rsi, rcx
140b067eb: mov r15, qword ptr [rdx + 0x10]
140b067ef: cmp r15, 1
140b067f3: jne 0x140b06814
140b067f5: mov rax, qword ptr [rdi + 8]
140b067f9: mov ecx, dword ptr [rax + 0x4c]
140b067fc: mov dword ptr [rsi + 0x40], ecx
140b067ff: movups xmm0, xmmword ptr [rax + 0xc]
140b06803: movups xmm1, xmmword ptr [rax + 0x1c]
140b06807: movups xmm2, xmmword ptr [rax + 0x2c]
140b0680b: movups xmm3, xmmword ptr [rax + 0x3c]
140b0680f: jmp 0x140b06ac7
140b06814: movss xmm6, dword ptr [rip + 0x1e1f0e8]
140b0681c: movaps xmm0, xmm2
140b0681f: movaps xmm1, xmm6
140b06822: call 0x142923f10 ; 
140b06827: test r15, r15
140b0682a: je 0x140b06aef
140b06830: movaps xmm1, xmm0
140b06833: addss xmm1, xmm6
140b06837: xorps xmm3, xmm3
140b0683a: movaps xmm2, xmm0
140b0683d: cmpltss xmm2, xmm3
140b06842: andps xmm1, xmm2
140b06845: andnps xmm2, xmm0
140b06848: orps xmm2, xmm1
140b0684b: mov rbx, qword ptr [rdi + 8]
140b0684f: movss xmm1, dword ptr [rbx + 8]
140b06854: mov rax, r15
140b06857: shl rax, 4
140b0685b: lea rdx, [rax + rax*4]
140b0685f: lea r14, [rbx + rdx]
140b06863: ucomiss xmm1, xmm2
140b06866: jbe 0x140b0689f
140b06868: cmp r14, 0x50
140b0686c: je 0x140b06aff
140b06872: movss xmm0, dword ptr [r14 - 0x48]
140b06878: addss xmm0, dword ptr [rip + 0x1e53864]
140b06880: lea rdi, [r14 - 0x44]
140b06884: call 0x140c904a0 ; utils::inverse_lerp
140b06889: xorps xmm1, xmm1
140b0688c: ucomiss xmm1, xmm0
140b0688f: seta al
140b06892: maxss xmm1, xmm0
140b06896: ucomiss xmm1, dword ptr [rip + 0x1e1f067]
140b0689d: jmp 0x140b068e9
140b0689f: ucomiss xmm2, xmm1
140b068a2: jb 0x140b068b2
140b068a4: movss xmm3, dword ptr [rbx + 0x58]
140b068a9: ucomiss xmm3, xmm2
140b068ac: jae 0x140b069f3
140b068b2: cmp r15, 2
140b068b6: jne 0x140b0699a
140b068bc: cmp r14, 0x50
140b068c0: je 0x140b06b0b
140b068c6: movss xmm0, dword ptr [r14 - 0x48]
140b068cc: addss xmm1, xmm6
140b068d0: lea rdi, [r14 - 0x44]
140b068d4: call 0x140c904a0 ; utils::inverse_lerp
140b068d9: xorps xmm1, xmm1
140b068dc: ucomiss xmm1, xmm0
140b068df: seta al
140b068e2: maxss xmm1, xmm0
140b068e6: ucomiss xmm1, xmm6
140b068e9: movd ecx, xmm0
140b068ed: seta dl
140b068f0: and ecx, 0x7fffffff
140b068f6: cmp ecx, 0x7f800000
140b068fc: setl cl
140b068ff: or cl, al
140b06901: or cl, dl
140b06903: je 0x140b06ab2
140b06909: lea r8, [rbx + 0xc]
140b0690d: minss xmm6, xmm1
140b06911: mov rcx, rsi
140b06914: mov rdx, rdi
140b06917: movaps xmm3, xmm6
140b0691a: call 0x140a97650 ; country_core::render::atmosphere::SunSkyColors::lerp
140b0691f: movss xmm0, dword ptr [rbx + 0x40]
140b06924: mulss xmm0, xmm6
140b06928: movsd xmm2, qword ptr [rbx + 0x44]
140b0692d: movsldup xmm1, xmm6
140b06931: mulps xmm1, xmm2
140b06934: movss xmm3, dword ptr [r14 - 4]
140b0693a: movss xmm2, dword ptr [rbx + 0x4c]
140b0693f: subss xmm2, xmm3
140b06943: mulss xmm2, xmm6
140b06947: addss xmm2, xmm3
140b0694b: movss xmm3, dword ptr [rbx + 0x3c]
140b06950: movaps xmm4, xmmword ptr [rip + 0x1e2dc59]
140b06957: movss xmm4, xmm3
140b0695b: movss xmm3, dword ptr [r14 - 0x14]
140b06961: movss xmm5, dword ptr [r14 - 0x10]
140b06967: movaps xmm7, xmm3
140b0696a: unpcklps xmm7, xmm6
140b0696d: shufps xmm7, xmm7, 0x54
140b06971: shufps xmm4, xmm4, 0x54
140b06975: subps xmm4, xmm7
140b06978: unpcklps xmm6, xmm5
140b0697b: movhps xmm6, qword ptr [r14 - 0xc]
140b06980: mulps xmm6, xmm4
140b06983: unpcklps xmm3, xmm0
140b06986: movlhps xmm3, xmm1
140b06989: addps xmm3, xmm6
140b0698c: movups xmmword ptr [rsi + 0x30], xmm3
140b06990: movss dword ptr [rsi + 0x40], xmm2
140b06995: jmp 0x140b06ad6
140b0699a: lea r15, [rbx + 0xa0]
140b069a1: lea rax, [rbx + 0x58]
140b069a5: mov ecx, 0xa0
140b069aa: sub rcx, rdx
140b069ad: xor edx, edx
140b069af: jmp 0x140b069d1
140b069b1: nop word ptr cs:[rax + rax]
140b069c0: add rdx, -0x50
140b069c4: add rax, 0x50
140b069c8: cmp rcx, rdx
140b069cb: je 0x140b068bc
140b069d1: movss xmm0, dword ptr [rax]
140b069d5: ucomiss xmm2, xmm0
140b069d8: jb 0x140b069c0
140b069da: movss xmm3, dword ptr [rax + 0x50]
140b069df: ucomiss xmm3, xmm2
140b069e2: jb 0x140b069c0
140b069e4: sub r15, rdx
140b069e7: add rax, -8
140b069eb: mov rbx, rax
140b069ee: movaps xmm1, xmm0
140b069f1: jmp 0x140b069f7
140b069f3: lea r15, [rbx + 0x50]
140b069f7: lea rdi, [rbx + 0xc]
140b069fb: movaps xmm0, xmm1
140b069fe: movaps xmm1, xmm3
140b06a01: call 0x140c904a0 ; utils::inverse_lerp
140b06a06: xorps xmm1, xmm1
140b06a09: ucomiss xmm1, xmm0
140b06a0c: seta al
140b06a0f: maxss xmm1, xmm0
140b06a13: ucomiss xmm1, dword ptr [rip + 0x1e1eeea]
140b06a1a: movd ecx, xmm0
140b06a1e: seta dl
140b06a21: and ecx, 0x7fffffff
140b06a27: cmp ecx, 0x7f800000
140b06a2d: setl cl
140b06a30: or cl, al
140b06a32: or cl, dl
140b06a34: je 0x140b06ab2
140b06a36: lea r8, [r15 + 0xc]
140b06a3a: minss xmm6, xmm1
140b06a3e: mov rcx, rsi
140b06a41: mov rdx, rdi
140b06a44: movaps xmm3, xmm6
140b06a47: call 0x140a97650 ; country_core::render::atmosphere::SunSkyColors::lerp
140b06a4c: movss xmm0, dword ptr [r15 + 0x40]
140b06a52: mulss xmm0, xmm6
140b06a56: movsd xmm2, qword ptr [r15 + 0x44]
140b06a5c: movsldup xmm1, xmm6
140b06a60: mulps xmm1, xmm2
140b06a63: movss xmm3, dword ptr [rbx + 0x4c]
140b06a68: movss xmm2, dword ptr [r15 + 0x4c]
140b06a6e: subss xmm2, xmm3
140b06a72: mulss xmm2, xmm6
140b06a76: addss xmm2, xmm3
140b06a7a: movss xmm3, dword ptr [r15 + 0x3c]
140b06a80: movaps xmm4, xmmword ptr [rip + 0x1e2db29]
140b06a87: movss xmm4, xmm3
140b06a8b: movss xmm3, dword ptr [rbx + 0x3c]
140b06a90: movss xmm5, dword ptr [rbx + 0x40]
140b06a95: movaps xmm7, xmm3
140b06a98: unpcklps xmm7, xmm6
140b06a9b: shufps xmm7, xmm7, 0x54
140b06a9f: shufps xmm4, xmm4, 0x54
140b06aa3: subps xmm4, xmm7
140b06aa6: unpcklps xmm6, xmm5
140b06aa9: movhps xmm6, qword ptr [rbx + 0x44]
140b06aad: jmp 0x140b06980
140b06ab2: mov eax, dword ptr [rdi + 0x40]
140b06ab5: mov dword ptr [rsi + 0x40], eax
140b06ab8: movups xmm0, xmmword ptr [rdi]
140b06abb: movups xmm1, xmmword ptr [rdi + 0x10]
140b06abf: movups xmm2, xmmword ptr [rdi + 0x20]
140b06ac3: movups xmm3, xmmword ptr [rdi + 0x30]
140b06ac7: movups xmmword ptr [rsi + 0x30], xmm3
140b06acb: movups xmmword ptr [rsi + 0x20], xmm2
140b06acf: movups xmmword ptr [rsi + 0x10], xmm1
140b06ad3: movups xmmword ptr [rsi], xmm0
140b06ad6: mov rax, rsi
140b06ad9: movaps xmm6, xmmword ptr [rsp + 0x20]
140b06ade: movaps xmm7, xmmword ptr [rsp + 0x30]
140b06ae3: add rsp, 0x40
140b06ae7: pop rbx
140b06ae8: pop rdi
140b06ae9: pop rsi
140b06aea: pop r14
140b06aec: pop r15
140b06aee: ret 
140b06aef: lea r8, [rip + 0x1fe0b4a]
140b06af6: xor ecx, ecx
140b06af8: xor edx, edx
140b06afa: call 0x1428d9518 ; core::panicking::panic_bounds_check
140b06aff: lea rcx, [rip + 0x1fe0b6a]
140b06b06: call 0x1428d9310 ; core::option::unwrap_failed
140b06b0b: lea rcx, [rip + 0x1fe0b46]
140b06b12: call 0x1428d9310 ; core::option::unwrap_failed
140b06b17: int3 
