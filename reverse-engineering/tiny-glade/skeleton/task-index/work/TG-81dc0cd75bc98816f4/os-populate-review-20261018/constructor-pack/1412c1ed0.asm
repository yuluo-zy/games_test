1412c1ed0: push rbp
1412c1ed1: push rsi
1412c1ed2: sub rsp, 0xe8
1412c1ed9: lea rbp, [rsp + 0x80]
1412c1ee1: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c1ee9: mov rsi, rcx
1412c1eec: lea rcx, [rbp - 0x58]
1412c1ef0: xor r8d, r8d
1412c1ef3: call 0x14161bd30
1412c1ef8: movzx eax, byte ptr [rip + 0x2372d0a]
1412c1eff: mov ecx, 0x38
1412c1f04: mov edx, 8
1412c1f09: call 0x140613c10
1412c1f0e: test rax, rax
1412c1f11: je 0x1412c1fcf
1412c1f17: mov rcx, qword ptr [rbp - 0x28]
1412c1f1b: mov qword ptr [rax + 0x30], rcx
1412c1f1f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c1f23: movups xmm1, xmmword ptr [rbp - 0x48]
1412c1f27: movups xmm2, xmmword ptr [rbp - 0x38]
1412c1f2b: movups xmmword ptr [rax + 0x20], xmm2
1412c1f2f: movups xmmword ptr [rax + 0x10], xmm1
1412c1f33: movups xmmword ptr [rax], xmm0
1412c1f36: lea rcx, [rip + 0x17dcb13]
1412c1f3d: mov qword ptr [rbp + 0x10], rcx
1412c1f41: mov qword ptr [rbp + 0x18], 0x5c
1412c1f49: mov qword ptr [rbp + 0x20], rax
1412c1f4d: lea rax, [rip + 0x196ae44]
1412c1f54: mov qword ptr [rbp + 0x28], rax
1412c1f58: mov qword ptr [rbp + 0x40], 0
1412c1f60: mov byte ptr [rbp + 0x56], 5
1412c1f64: mov qword ptr [rbp + 0x30], rcx
1412c1f68: mov qword ptr [rbp + 0x38], 0x5c
1412c1f70: movzx eax, byte ptr [rip + 0x2372c92]
1412c1f77: mov ecx, 0x40
1412c1f7c: mov edx, 8
1412c1f81: call 0x140613c10
1412c1f86: test rax, rax
1412c1f89: je 0x1412c1fe0
1412c1f8b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c1f8f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c1f93: movups xmm2, xmmword ptr [rbp + 0x40]
1412c1f97: movups xmm3, xmmword ptr [rbp + 0x50]
1412c1f9b: movups xmmword ptr [rax + 0x30], xmm3
1412c1f9f: movups xmmword ptr [rax + 0x20], xmm2
1412c1fa3: movups xmmword ptr [rax + 0x10], xmm1
1412c1fa7: movups xmmword ptr [rax], xmm0
1412c1faa: movabs rdx, 0xdeff5ab6a35972a
1412c1fb4: mov rcx, rsi
1412c1fb7: mov r8, rax
1412c1fba: call 0x140969bf0
1412c1fbf: cmp rax, 1
1412c1fc3: je 0x1412c1ff1
1412c1fc5: add rsp, 0xe8
1412c1fcc: pop rsi
1412c1fcd: pop rbp
1412c1fce: ret 
1412c1fcf: mov ecx, 8
1412c1fd4: mov edx, 0x38
1412c1fd9: call 0x1428d9000
1412c1fde: jmp 0x1412c1fef
1412c1fe0: mov ecx, 8
1412c1fe5: mov edx, 0x40
1412c1fea: call 0x1428d9000
1412c1fef: ud2 
1412c1ff1: mov rax, rdx
1412c1ff4: add rdx, 0x10
1412c1ff8: mov rsi, rdx
1412c1ffb: cmp qword ptr [rax + 0x18], 0x5c
1412c2000: jne 0x1412c207f
1412c2002: mov rdx, qword ptr [rsi]
1412c2005: lea rcx, [rip + 0x17dca44]
1412c200c: mov r8d, 0x5c
1412c2012: call 0x142923bf0
1412c2017: test eax, eax
1412c2019: jne 0x1412c207f
1412c201b: lea rax, [rip + 0x196bbae]
1412c2022: mov qword ptr [rbp - 0x20], rax
1412c2026: lea rax, [rip - 0xd8e78d]
1412c202d: mov qword ptr [rbp - 0x18], rax
1412c2031: lea rax, [rbp + 0x10]
1412c2035: mov qword ptr [rbp - 0x10], rax
1412c2039: lea rax, [rip - 0x11b81b0]
1412c2040: mov qword ptr [rbp - 8], rax
1412c2044: lea rax, [rip + 0x196b5dd]
1412c204b: mov qword ptr [rbp + 0x20], rax
1412c204f: mov qword ptr [rbp + 0x28], 3
1412c2057: mov qword ptr [rbp + 0x40], 0
1412c205f: lea rax, [rbp - 0x20]
1412c2063: mov qword ptr [rbp + 0x30], rax
1412c2067: mov qword ptr [rbp + 0x38], 2
1412c206f: lea rdx, [rip + 0x196b5e2]
1412c2076: lea rcx, [rbp + 0x20]
1412c207a: call 0x1428d9390
1412c207f: lea rax, [rip + 0x196bb4a]
1412c2086: mov qword ptr [rbp + 0x20], rax
1412c208a: lea rax, [rip - 0xd8e7f1]
1412c2091: mov qword ptr [rbp + 0x28], rax
1412c2095: mov qword ptr [rbp + 0x30], rsi
1412c2099: lea rax, [rip - 0x11b8210]
1412c20a0: mov qword ptr [rbp + 0x38], rax
1412c20a4: lea rcx, [rbp + 0x10]
1412c20a8: mov qword ptr [rbp + 0x40], rcx
1412c20ac: mov qword ptr [rbp + 0x48], rax
1412c20b0: lea rax, [rip + 0x196b4b1]
1412c20b7: mov qword ptr [rbp - 0x20], rax
1412c20bb: mov qword ptr [rbp - 0x18], 3
1412c20c3: mov qword ptr [rbp], 0
1412c20cb: lea rax, [rbp + 0x20]
1412c20cf: mov qword ptr [rbp - 0x10], rax
1412c20d3: mov qword ptr [rbp - 8], 3
1412c20db: lea rdx, [rip + 0x196b4ee]
1412c20e2: lea rcx, [rbp - 0x20]
1412c20e6: call 0x1428d9390
1412c20eb: int3 
1412c20ec: nop dword ptr [rax]
1412c20f0: mov qword ptr [rsp + 0x10], rdx
1412c20f5: push rbp
1412c20f6: push rsi
1412c20f7: sub rsp, 0x28
1412c20fb: lea rbp, [rdx + 0x80]
1412c2102: lea rcx, [rbp - 0x48]
1412c2106: call 0x140be24b0
1412c210b: nop 
1412c210c: add rsp, 0x28
1412c2110: pop rsi
1412c2111: pop rbp
1412c2112: ret 
1412c2113: nop word ptr cs:[rax + rax]
1412c2120: mov qword ptr [rsp + 0x10], rdx
1412c2125: push rbp
1412c2126: push rsi
1412c2127: sub rsp, 0x28
1412c212b: lea rbp, [rdx + 0x80]
1412c2132: lea rcx, [rbp + 0x20]
1412c2136: call 0x1400c04b0
1412c213b: nop 
1412c213c: add rsp, 0x28
1412c2140: pop rsi
1412c2141: pop rbp
1412c2142: ret 
