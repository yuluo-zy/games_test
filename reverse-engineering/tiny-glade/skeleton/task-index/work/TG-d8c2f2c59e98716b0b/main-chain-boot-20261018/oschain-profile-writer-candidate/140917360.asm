140917360: push rbp
140917361: push r14
140917363: push rsi
140917364: push rdi
140917365: push rbx
140917366: sub rsp, 0x160
14091736d: lea rbp, [rsp + 0x80]
140917375: mov qword ptr [rbp + 0xd8], 0xfffffffffffffffe
140917380: mov rsi, rcx
140917383: call 0x140b9db30
140917388: mov r8, rdx
14091738b: mov qword ptr [rbp + 0x18], 6
140917393: lea r9, [rip + 0x219e7ae]
14091739a: mov qword ptr [rbp - 8], r9
14091739e: mov qword ptr [rbp], 4
1409173a6: mov qword ptr [rbp + 8], 1
1409173ae: mov qword ptr [rbp + 0x10], 0
1409173b6: mov word ptr [rbp + 0x20], 0
1409173bc: mov dword ptr [rbp - 0x10], 0
1409173c3: lea rcx, [rbp - 0x10]
1409173c7: mov qword ptr [rsp + 0x40], rcx
1409173cc: mov qword ptr [rsp + 0x38], 6
1409173d5: mov qword ptr [rsp + 0x30], 0
1409173de: mov qword ptr [rsp + 0x28], 1
1409173e7: mov qword ptr [rsp + 0x20], 4
1409173f0: lea rcx, [rbp + 0x58]
1409173f4: mov rdx, rax
1409173f7: call 0x140ac66a0
1409173fc: movzx eax, byte ptr [rbp + 0x70]
140917400: mov rcx, qword ptr [rbp + 0x58]
140917404: cmp al, 2
140917406: je 0x14091763e
14091740c: movups xmm0, xmmword ptr [rbp + 0x60]
140917410: movups xmmword ptr [rbp + 0xb0], xmm0
140917417: mov edx, dword ptr [rbp + 0x71]
14091741a: mov dword ptr [rbp + 0xc1], edx
140917420: mov edx, dword ptr [rbp + 0x74]
140917423: mov dword ptr [rbp + 0xc4], edx
140917429: mov qword ptr [rbp + 0xa8], rcx
140917430: mov byte ptr [rbp + 0xc0], al
140917436: mov byte ptr [rbp + 0xd7], 1
14091743d: lea rcx, [rbp - 0x10]
140917441: call 0x140bcf6f0
140917446: mov qword ptr [rbp - 0x20], 0
14091744e: mov qword ptr [rbp - 0x30], 1
140917456: mov qword ptr [rbp - 0x28], 0x80
14091745e: mov byte ptr [rbp + 0xd7], 1
140917465: lea rcx, [rbp + 0x58]
140917469: lea rdx, [rbp - 0x30]
14091746d: lea r9, [rbp - 0x10]
140917471: mov r8, rsi
140917474: call 0x14169cce0
140917479: cmp byte ptr [rbp + 0x58], 0x32
14091747d: jne 0x140917657
140917483: mov rax, qword ptr [rbp + 0x60]
140917487: mov qword ptr [rbp + 0xc8], rax
14091748e: mov rdx, qword ptr [rbp + 0x68]
140917492: mov r8, qword ptr [rbp + 0x70]
140917496: lea rcx, [rbp + 0xa8]
14091749d: mov qword ptr [rbp + 0xa0], rdx
1409174a4: call 0x140914c90
1409174a9: mov rsi, rax
1409174ac: test rax, rax
1409174af: je 0x140917575
1409174b5: mov rdi, qword ptr [rbp + 0xb0]
1409174bc: mov r14, qword ptr [rbp + 0xb8]
1409174c3: mov qword ptr [rbp - 0x30], rsi
1409174c7: mov rcx, rsi
1409174ca: call 0x1415a4cb0
1409174cf: mov bl, al
1409174d1: mov byte ptr [rbp + 0xd5], 1
1409174d8: lea rcx, [rbp + 0x58]
1409174dc: mov rdx, rdi
1409174df: mov r8, r14
1409174e2: call 0x140d61b00
1409174e7: movups xmm0, xmmword ptr [rbp + 0x58]
1409174eb: movups xmm1, xmmword ptr [rbp + 0x68]
1409174ef: movaps xmmword ptr [rbp], xmm1
1409174f3: movaps xmmword ptr [rbp - 0x10], xmm0
1409174f7: mov qword ptr [rbp + 0x10], rsi
1409174fb: mov byte ptr [rbp + 0xd5], 0
140917502: lea rdx, [rbp - 0x10]
140917506: mov ecx, ebx
140917508: call 0x140b83460
14091750d: mov rcx, rax
140917510: call 0x1428a9590
140917515: mov rsi, rax
140917518: lea rdi, [rbp + 0xb0]
14091751f: mov rdx, qword ptr [rbp + 0xc8]
140917526: test rdx, rdx
140917529: je 0x14091753d
14091752b: mov r8d, 1
140917531: mov rcx, qword ptr [rbp + 0xa0]
140917538: call 0x140613c20
14091753d: mov rcx, rdi
140917540: call 0x140b83ed0
140917545: mov rdx, qword ptr [rbp + 0xb8]
14091754c: test rdx, rdx
14091754f: je 0x140917563
140917551: mov rcx, qword ptr [rbp + 0xb0]
140917558: mov r8d, 1
14091755e: call 0x140613c20
140917563: mov rcx, qword ptr [rbp + 0xa8]
14091756a: call qword ptr [rip + 0x287ae80]
140917570: jmp 0x140917646
140917575: mov rdx, qword ptr [rbp + 0xc8]
14091757c: test rdx, rdx
14091757f: je 0x140917593
140917581: mov r8d, 1
140917587: mov rcx, qword ptr [rbp + 0xa0]
14091758e: call 0x140613c20
140917593: movups xmm0, xmmword ptr [rbp + 0xa8]
14091759a: movups xmm1, xmmword ptr [rbp + 0xb8]
1409175a1: movaps xmmword ptr [rbp - 0x20], xmm1
1409175a5: movaps xmmword ptr [rbp - 0x30], xmm0
1409175a9: mov byte ptr [rbp + 0xd6], 1
1409175b0: call 0x140b9db30
1409175b5: mov r8, rdx
1409175b8: mov byte ptr [rbp + 0xd6], 1
1409175bf: mov qword ptr [rsp + 0x20], 0xb
1409175c8: lea r9, [rip + 0x219544d]
1409175cf: lea rcx, [rbp - 0x10]
1409175d3: mov rdx, rax
1409175d6: call 0x140d62810
1409175db: mov byte ptr [rbp + 0xd6], 0
1409175e2: lea rcx, [rbp + 0x58]
1409175e6: lea rdx, [rbp - 0x30]
1409175ea: lea r8, [rbp - 0x10]
1409175ee: call 0x14096b270
1409175f3: movzx eax, byte ptr [rbp + 0x78]
1409175f7: mov rcx, qword ptr [rbp + 0x58]
1409175fb: cmp al, 2
1409175fd: jne 0x140917609
1409175ff: call qword ptr [rip + 0x287adeb]
140917605: xor esi, esi
140917607: jmp 0x140917646
140917609: mov rdx, qword ptr [rbp + 0x70]
14091760d: mov qword ptr [rbp + 8], rdx
140917611: movups xmm0, xmmword ptr [rbp + 0x60]
140917615: movups xmmword ptr [rbp - 8], xmm0
140917619: mov edx, dword ptr [rbp + 0x79]
14091761c: mov dword ptr [rbp + 0x11], edx
14091761f: mov edx, dword ptr [rbp + 0x7c]
140917622: mov dword ptr [rbp + 0x14], edx
140917625: mov qword ptr [rbp - 0x10], rcx
140917629: mov byte ptr [rbp + 0x10], al
14091762c: mov byte ptr [rbp + 0xd7], 0
140917633: lea rcx, [rbp - 0x10]
140917637: call 0x1428a9680
14091763c: jmp 0x140917643
14091763e: call 0x1428a9590
140917643: mov rsi, rax
140917646: mov rax, rsi
140917649: add rsp, 0x160
140917650: pop rbx
140917651: pop rdi
140917652: pop rsi
140917653: pop r14
140917655: pop rbp
140917656: ret 
140917657: mov rax, qword ptr [rbp + 0x98]
14091765e: mov qword ptr [rbp + 0x30], rax
140917662: movups xmm0, xmmword ptr [rbp + 0x58]
140917666: movups xmm1, xmmword ptr [rbp + 0x68]
14091766a: movups xmm2, xmmword ptr [rbp + 0x78]
14091766e: movups xmm3, xmmword ptr [rbp + 0x88]
140917675: movaps xmmword ptr [rbp + 0x20], xmm3
140917679: movaps xmmword ptr [rbp + 0x10], xmm2
14091767d: movaps xmmword ptr [rbp], xmm1
140917681: movaps xmmword ptr [rbp - 0x10], xmm0
140917685: lea rax, [rip + 0x219557c]
14091768c: mov qword ptr [rsp + 0x20], rax
140917691: lea rcx, [rip + 0x2194978]
140917698: lea r9, [rip + 0x21949c1]
14091769f: lea r8, [rbp - 0x10]
1409176a3: mov edx, 0x2b
1409176a8: call 0x1428d9760
1409176ad: ud2 
1409176af: nop 
1409176b0: mov qword ptr [rsp + 0x10], rdx
1409176b5: push rbp
1409176b6: push r14
1409176b8: push rsi
1409176b9: push rdi
1409176ba: push rbx
1409176bb: sub rsp, 0x50
1409176bf: lea rbp, [rdx + 0x80]
1409176c6: cmp qword ptr [rbp + 0xc8], 0
1409176ce: je 0x1409176e9
1409176d0: mov r8d, 1
1409176d6: mov rcx, qword ptr [rbp + 0xa0]
1409176dd: mov rdx, qword ptr [rbp + 0xc8]
1409176e4: call 0x140613c20
1409176e9: mov byte ptr [rbp + 0xd7], 1
1409176f0: add rsp, 0x50
1409176f4: pop rbx
1409176f5: pop rdi
1409176f6: pop rsi
1409176f7: pop r14
1409176f9: pop rbp
1409176fa: ret 
1409176fb: nop dword ptr [rax + rax]
140917700: mov qword ptr [rsp + 0x10], rdx
140917705: push rbp
140917706: push r14
140917708: push rsi
140917709: push rdi
14091770a: push rbx
14091770b: sub rsp, 0x50
14091770f: lea rbp, [rdx + 0x80]
140917716: cmp byte ptr [rbp + 0xd5], 0
14091771d: je 0x140917728
14091771f: lea rcx, [rbp - 0x30]
140917723: call 0x1405e7a00
140917728: nop 
140917729: add rsp, 0x50
14091772d: pop rbx
14091772e: pop rdi
14091772f: pop rsi
140917730: pop r14
140917732: pop rbp
140917733: ret 
140917734: nop word ptr cs:[rax + rax]
140917740: mov qword ptr [rsp + 0x10], rdx
140917745: push rbp
140917746: push r14
140917748: push rsi
140917749: push rdi
14091774a: push rbx
14091774b: sub rsp, 0x50
14091774f: lea rbp, [rdx + 0x80]
140917756: cmp byte ptr [rbp + 0xd6], 0
14091775d: je 0x14091777a
14091775f: lea rcx, [rbp - 0x30]
140917763: call 0x1405ecc10
140917768: mov byte ptr [rbp + 0xd7], 0
14091776f: add rsp, 0x50
140917773: pop rbx
140917774: pop rdi
140917775: pop rsi
140917776: pop r14
140917778: pop rbp
140917779: ret 
14091777a: mov byte ptr [rbp + 0xd7], 0
140917781: add rsp, 0x50
140917785: pop rbx
140917786: pop rdi
140917787: pop rsi
140917788: pop r14
14091778a: pop rbp
14091778b: ret 
14091778c: nop dword ptr [rax]
140917790: mov qword ptr [rsp + 0x10], rdx
140917795: push rbp
140917796: push r14
140917798: push rsi
140917799: push rdi
14091779a: push rbx
14091779b: sub rsp, 0x50
14091779f: lea rbp, [rdx + 0x80]
1409177a6: mov rdx, qword ptr [rbp + 0xb8]
1409177ad: test rdx, rdx
1409177b0: je 0x1409177c4
1409177b2: mov rcx, qword ptr [rbp + 0xb0]
1409177b9: mov r8d, 1
1409177bf: call 0x140613c20
1409177c4: mov rcx, qword ptr [rbp + 0xa8]
1409177cb: call qword ptr [rip + 0x287ac1f]
1409177d1: nop 
1409177d2: add rsp, 0x50
1409177d6: pop rbx
1409177d7: pop rdi
1409177d8: pop rsi
1409177d9: pop r14
1409177db: pop rbp
1409177dc: ret 
1409177dd: nop dword ptr [rax]
1409177e0: mov qword ptr [rsp + 0x10], rdx
1409177e5: push rbp
1409177e6: push r14
1409177e8: push rsi
1409177e9: push rdi
1409177ea: push rbx
1409177eb: sub rsp, 0x50
1409177ef: lea rbp, [rdx + 0x80]
1409177f6: cmp byte ptr [rbp + 0xd7], 0
1409177fd: jne 0x14091780a
1409177ff: add rsp, 0x50
140917803: pop rbx
140917804: pop rdi
140917805: pop rsi
140917806: pop r14
140917808: pop rbp
140917809: ret 
14091780a: lea rcx, [rbp + 0xa8]
140917811: call 0x1405ecc10
140917816: jmp 0x1409177ff
140917818: nop dword ptr [rax + rax]
140917820: mov qword ptr [rsp + 0x10], rdx
140917825: push rbp
140917826: push r14
140917828: push rsi
140917829: push rdi
14091782a: push rbx
14091782b: sub rsp, 0x50
14091782f: lea rbp, [rdx + 0x80]
140917836: lea rcx, [rbp - 0x10]
14091783a: call 0x140914e30
14091783f: mov byte ptr [rbp + 0xd7], 1
140917846: add rsp, 0x50
14091784a: pop rbx
14091784b: pop rdi
14091784c: pop rsi
14091784d: pop r14
14091784f: pop rbp
140917850: ret 
