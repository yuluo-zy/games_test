1412c67b0: push rbp
1412c67b1: push rsi
1412c67b2: sub rsp, 0xd8
1412c67b9: lea rbp, [rsp + 0x80]
1412c67c1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c67c9: mov rsi, rcx
1412c67cc: lea rcx, [rbp - 0x58]
1412c67d0: xor r8d, r8d
1412c67d3: call 0x141614110
1412c67d8: movzx eax, byte ptr [rip + 0x236e42a]
1412c67df: mov ecx, 0x28
1412c67e4: mov edx, 8
1412c67e9: call 0x140613c10
1412c67ee: test rax, rax
1412c67f1: je 0x1412c68a7
1412c67f7: mov rcx, qword ptr [rbp - 0x38]
1412c67fb: mov qword ptr [rax + 0x20], rcx
1412c67ff: movups xmm0, xmmword ptr [rbp - 0x58]
1412c6803: movups xmm1, xmmword ptr [rbp - 0x48]
1412c6807: movups xmmword ptr [rax + 0x10], xmm1
1412c680b: movups xmmword ptr [rax], xmm0
1412c680e: lea rcx, [rip + 0x17d8088]
1412c6815: mov qword ptr [rbp], rcx
1412c6819: mov qword ptr [rbp + 8], 0x54
1412c6821: mov qword ptr [rbp + 0x10], rax
1412c6825: lea rax, [rip + 0x1966bc4]
1412c682c: mov qword ptr [rbp + 0x18], rax
1412c6830: mov qword ptr [rbp + 0x30], 0
1412c6838: mov byte ptr [rbp + 0x46], 5
1412c683c: mov qword ptr [rbp + 0x20], rcx
1412c6840: mov qword ptr [rbp + 0x28], 0x54
1412c6848: movzx eax, byte ptr [rip + 0x236e3ba]
1412c684f: mov ecx, 0x40
1412c6854: mov edx, 8
1412c6859: call 0x140613c10
1412c685e: test rax, rax
1412c6861: je 0x1412c68b8
1412c6863: movups xmm0, xmmword ptr [rbp + 0x10]
1412c6867: movups xmm1, xmmword ptr [rbp + 0x20]
1412c686b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c686f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c6873: movups xmmword ptr [rax + 0x30], xmm3
1412c6877: movups xmmword ptr [rax + 0x20], xmm2
1412c687b: movups xmmword ptr [rax + 0x10], xmm1
1412c687f: movups xmmword ptr [rax], xmm0
1412c6882: movabs rdx, 0xda2386d830255aac
1412c688c: mov rcx, rsi
1412c688f: mov r8, rax
1412c6892: call 0x140969bf0
1412c6897: cmp rax, 1
1412c689b: je 0x1412c68c9
1412c689d: add rsp, 0xd8
1412c68a4: pop rsi
1412c68a5: pop rbp
1412c68a6: ret 
1412c68a7: mov ecx, 8
1412c68ac: mov edx, 0x28
1412c68b1: call 0x1428d9000
1412c68b6: jmp 0x1412c68c7
1412c68b8: mov ecx, 8
1412c68bd: mov edx, 0x40
1412c68c2: call 0x1428d9000
1412c68c7: ud2 
1412c68c9: mov rax, rdx
1412c68cc: add rdx, 0x10
1412c68d0: mov rsi, rdx
1412c68d3: cmp qword ptr [rax + 0x18], 0x54
1412c68d8: jne 0x1412c6956
1412c68da: mov rdx, qword ptr [rsi]
1412c68dd: lea rcx, [rip + 0x17d7fb9]
1412c68e4: mov r8d, 0x54
1412c68ea: call 0x142923bf0
1412c68ef: test eax, eax
1412c68f1: jne 0x1412c6956
1412c68f3: lea rax, [rip + 0x1967576]
1412c68fa: mov qword ptr [rbp - 0x30], rax
1412c68fe: lea rax, [rip - 0xd93065]
1412c6905: mov qword ptr [rbp - 0x28], rax
1412c6909: mov rax, rbp
1412c690c: mov qword ptr [rbp - 0x20], rax
1412c6910: lea rax, [rip - 0x11bca87]
1412c6917: mov qword ptr [rbp - 0x18], rax
1412c691b: lea rax, [rip + 0x1966d06]
1412c6922: mov qword ptr [rbp + 0x10], rax
1412c6926: mov qword ptr [rbp + 0x18], 3
1412c692e: mov qword ptr [rbp + 0x30], 0
1412c6936: lea rax, [rbp - 0x30]
1412c693a: mov qword ptr [rbp + 0x20], rax
1412c693e: mov qword ptr [rbp + 0x28], 2
1412c6946: lea rdx, [rip + 0x1966d0b]
1412c694d: lea rcx, [rbp + 0x10]
1412c6951: call 0x1428d9390
1412c6956: lea rax, [rip + 0x1967513]
1412c695d: mov qword ptr [rbp + 0x10], rax
1412c6961: lea rax, [rip - 0xd930c8]
1412c6968: mov qword ptr [rbp + 0x18], rax
1412c696c: mov qword ptr [rbp + 0x20], rsi
1412c6970: lea rax, [rip - 0x11bcae7]
1412c6977: mov qword ptr [rbp + 0x28], rax
1412c697b: mov rcx, rbp
1412c697e: mov qword ptr [rbp + 0x30], rcx
1412c6982: mov qword ptr [rbp + 0x38], rax
1412c6986: lea rax, [rip + 0x1966bdb]
1412c698d: mov qword ptr [rbp - 0x30], rax
1412c6991: mov qword ptr [rbp - 0x28], 3
1412c6999: mov qword ptr [rbp - 0x10], 0
1412c69a1: lea rax, [rbp + 0x10]
1412c69a5: mov qword ptr [rbp - 0x20], rax
1412c69a9: mov qword ptr [rbp - 0x18], 3
1412c69b1: lea rdx, [rip + 0x1966c18]
1412c69b8: lea rcx, [rbp - 0x30]
1412c69bc: call 0x1428d9390
1412c69c1: int3 
1412c69c2: nop word ptr cs:[rax + rax]
1412c69d0: mov qword ptr [rsp + 0x10], rdx
1412c69d5: push rbp
1412c69d6: push rsi
1412c69d7: sub rsp, 0x28
1412c69db: lea rbp, [rdx + 0x80]
1412c69e2: lea rcx, [rbp - 0x58]
1412c69e6: call 0x140be24b0
1412c69eb: nop 
1412c69ec: add rsp, 0x28
1412c69f0: pop rsi
1412c69f1: pop rbp
1412c69f2: ret 
1412c69f3: nop word ptr cs:[rax + rax]
1412c6a00: mov qword ptr [rsp + 0x10], rdx
1412c6a05: push rbp
1412c6a06: push rsi
1412c6a07: sub rsp, 0x28
1412c6a0b: lea rbp, [rdx + 0x80]
1412c6a12: lea rcx, [rbp + 0x10]
1412c6a16: call 0x1400c04b0
1412c6a1b: nop 
1412c6a1c: add rsp, 0x28
1412c6a20: pop rsi
1412c6a21: pop rbp
1412c6a22: ret 
