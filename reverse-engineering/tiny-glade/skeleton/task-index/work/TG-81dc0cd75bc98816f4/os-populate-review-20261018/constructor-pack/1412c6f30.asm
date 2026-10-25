1412c6f30: push rbp
1412c6f31: push rsi
1412c6f32: sub rsp, 0xe8
1412c6f39: lea rbp, [rsp + 0x80]
1412c6f41: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c6f49: mov rsi, rcx
1412c6f4c: lea rcx, [rbp - 0x58]
1412c6f50: xor r8d, r8d
1412c6f53: call 0x14161fb20
1412c6f58: movzx eax, byte ptr [rip + 0x236dcaa]
1412c6f5f: mov ecx, 0x38
1412c6f64: mov edx, 8
1412c6f69: call 0x140613c10
1412c6f6e: test rax, rax
1412c6f71: je 0x1412c702f
1412c6f77: mov rcx, qword ptr [rbp - 0x28]
1412c6f7b: mov qword ptr [rax + 0x30], rcx
1412c6f7f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c6f83: movups xmm1, xmmword ptr [rbp - 0x48]
1412c6f87: movups xmm2, xmmword ptr [rbp - 0x38]
1412c6f8b: movups xmmword ptr [rax + 0x20], xmm2
1412c6f8f: movups xmmword ptr [rax + 0x10], xmm1
1412c6f93: movups xmmword ptr [rax], xmm0
1412c6f96: lea rcx, [rip + 0x17d66c0]
1412c6f9d: mov qword ptr [rbp + 0x10], rcx
1412c6fa1: mov qword ptr [rbp + 0x18], 0x41
1412c6fa9: mov qword ptr [rbp + 0x20], rax
1412c6fad: lea rax, [rip + 0x19664e4]
1412c6fb4: mov qword ptr [rbp + 0x28], rax
1412c6fb8: mov qword ptr [rbp + 0x40], 0
1412c6fc0: mov byte ptr [rbp + 0x56], 5
1412c6fc4: mov qword ptr [rbp + 0x30], rcx
1412c6fc8: mov qword ptr [rbp + 0x38], 0x41
1412c6fd0: movzx eax, byte ptr [rip + 0x236dc32]
1412c6fd7: mov ecx, 0x40
1412c6fdc: mov edx, 8
1412c6fe1: call 0x140613c10
1412c6fe6: test rax, rax
1412c6fe9: je 0x1412c7040
1412c6feb: movups xmm0, xmmword ptr [rbp + 0x20]
1412c6fef: movups xmm1, xmmword ptr [rbp + 0x30]
1412c6ff3: movups xmm2, xmmword ptr [rbp + 0x40]
1412c6ff7: movups xmm3, xmmword ptr [rbp + 0x50]
1412c6ffb: movups xmmword ptr [rax + 0x30], xmm3
1412c6fff: movups xmmword ptr [rax + 0x20], xmm2
1412c7003: movups xmmword ptr [rax + 0x10], xmm1
1412c7007: movups xmmword ptr [rax], xmm0
1412c700a: movabs rdx, 0x9fa335d19383656b
1412c7014: mov rcx, rsi
1412c7017: mov r8, rax
1412c701a: call 0x140969bf0
1412c701f: cmp rax, 1
1412c7023: je 0x1412c7051
1412c7025: add rsp, 0xe8
1412c702c: pop rsi
1412c702d: pop rbp
1412c702e: ret 
1412c702f: mov ecx, 8
1412c7034: mov edx, 0x38
1412c7039: call 0x1428d9000
1412c703e: jmp 0x1412c704f
1412c7040: mov ecx, 8
1412c7045: mov edx, 0x40
1412c704a: call 0x1428d9000
1412c704f: ud2 
1412c7051: mov rax, rdx
1412c7054: add rdx, 0x10
1412c7058: mov rsi, rdx
1412c705b: cmp qword ptr [rax + 0x18], 0x41
1412c7060: jne 0x1412c70df
1412c7062: mov rdx, qword ptr [rsi]
1412c7065: lea rcx, [rip + 0x17d65f1]
1412c706c: mov r8d, 0x41
1412c7072: call 0x142923bf0
1412c7077: test eax, eax
1412c7079: jne 0x1412c70df
1412c707b: lea rax, [rip + 0x1966f8e]
1412c7082: mov qword ptr [rbp - 0x20], rax
1412c7086: lea rax, [rip - 0xd937ed]
1412c708d: mov qword ptr [rbp - 0x18], rax
1412c7091: lea rax, [rbp + 0x10]
1412c7095: mov qword ptr [rbp - 0x10], rax
1412c7099: lea rax, [rip - 0x11bd210]
1412c70a0: mov qword ptr [rbp - 8], rax
1412c70a4: lea rax, [rip + 0x196657d]
1412c70ab: mov qword ptr [rbp + 0x20], rax
1412c70af: mov qword ptr [rbp + 0x28], 3
1412c70b7: mov qword ptr [rbp + 0x40], 0
1412c70bf: lea rax, [rbp - 0x20]
1412c70c3: mov qword ptr [rbp + 0x30], rax
1412c70c7: mov qword ptr [rbp + 0x38], 2
1412c70cf: lea rdx, [rip + 0x1966582]
1412c70d6: lea rcx, [rbp + 0x20]
1412c70da: call 0x1428d9390
1412c70df: lea rax, [rip + 0x1966f2a]
1412c70e6: mov qword ptr [rbp + 0x20], rax
1412c70ea: lea rax, [rip - 0xd93851]
1412c70f1: mov qword ptr [rbp + 0x28], rax
1412c70f5: mov qword ptr [rbp + 0x30], rsi
1412c70f9: lea rax, [rip - 0x11bd270]
1412c7100: mov qword ptr [rbp + 0x38], rax
1412c7104: lea rcx, [rbp + 0x10]
1412c7108: mov qword ptr [rbp + 0x40], rcx
1412c710c: mov qword ptr [rbp + 0x48], rax
1412c7110: lea rax, [rip + 0x1966451]
1412c7117: mov qword ptr [rbp - 0x20], rax
1412c711b: mov qword ptr [rbp - 0x18], 3
1412c7123: mov qword ptr [rbp], 0
1412c712b: lea rax, [rbp + 0x20]
1412c712f: mov qword ptr [rbp - 0x10], rax
1412c7133: mov qword ptr [rbp - 8], 3
1412c713b: lea rdx, [rip + 0x196648e]
1412c7142: lea rcx, [rbp - 0x20]
1412c7146: call 0x1428d9390
1412c714b: int3 
1412c714c: nop dword ptr [rax]
1412c7150: mov qword ptr [rsp + 0x10], rdx
1412c7155: push rbp
1412c7156: push rsi
1412c7157: sub rsp, 0x28
1412c715b: lea rbp, [rdx + 0x80]
1412c7162: lea rcx, [rbp - 0x48]
1412c7166: call 0x140be24b0
1412c716b: nop 
1412c716c: add rsp, 0x28
1412c7170: pop rsi
1412c7171: pop rbp
1412c7172: ret 
1412c7173: nop word ptr cs:[rax + rax]
1412c7180: mov qword ptr [rsp + 0x10], rdx
1412c7185: push rbp
1412c7186: push rsi
1412c7187: sub rsp, 0x28
1412c718b: lea rbp, [rdx + 0x80]
1412c7192: lea rcx, [rbp + 0x20]
1412c7196: call 0x1400c04b0
1412c719b: nop 
1412c719c: add rsp, 0x28
1412c71a0: pop rsi
1412c71a1: pop rbp
1412c71a2: ret 
