1405c0400: push rbp
1405c0401: push rsi
1405c0402: push rdi
1405c0403: push rbx
1405c0404: sub rsp, 0x238
1405c040b: lea rbp, [rsp + 0x80]
1405c0413: mov qword ptr [rbp + 0x1b0], 0xfffffffffffffffe
1405c041e: mov ebx, edx
1405c0420: mov rsi, rcx
1405c0423: xorps xmm0, xmm0
1405c0426: movups xmmword ptr [rbp + 0x20], xmm0
1405c042a: movups xmmword ptr [rbp + 0x38], xmm0
1405c042e: movups xmmword ptr [rbp + 0x50], xmm0
1405c0432: movups xmmword ptr [rbp + 0x68], xmm0
1405c0436: movups xmmword ptr [rbp + 0x80], xmm0
1405c043d: mov dword ptr [rbp + 0x90], 0
1405c0447: movups xmmword ptr [rbp + 0xb8], xmm0
1405c044e: movups xmmword ptr [rbp + 0xd0], xmm0
1405c0455: movups xmmword ptr [rbp + 0xe8], xmm0
1405c045c: movups xmmword ptr [rbp + 0x100], xmm0
1405c0463: movups xmmword ptr [rbp + 0x118], xmm0
1405c046a: mov dword ptr [rbp + 0x128], 0
1405c0474: mov qword ptr [rbp - 0x58], 0
1405c047c: mov qword ptr [rbp], 0
1405c0484: mov qword ptr [rbp + 8], 8
1405c048c: mov qword ptr [rbp + 0x10], 0
1405c0494: mov qword ptr [rbp + 0x18], 0x10
1405c049c: mov qword ptr [rbp + 0x30], 0x10
1405c04a4: mov qword ptr [rbp + 0x48], 0x10
1405c04ac: mov qword ptr [rbp + 0x60], 0x10
1405c04b4: mov qword ptr [rbp + 0x78], 0x10
1405c04bc: movabs rax, 0x8000000000000000
1405c04c6: mov qword ptr [rbp + 0x98], rax
1405c04cd: lea rax, [rip + 0x244509e]
1405c04d4: mov qword ptr [rbp + 0xa0], rax
1405c04db: mov qword ptr [rbp + 0xa8], 0x4b
1405c04e6: mov qword ptr [rbp + 0xb0], 0x10
1405c04f1: mov qword ptr [rbp + 0xc8], 0x10
1405c04fc: mov qword ptr [rbp + 0xe0], 0x10
1405c0507: mov qword ptr [rbp + 0xf8], 0x10
1405c0512: mov qword ptr [rbp + 0x110], 0x10
1405c051d: mov dword ptr [rbp + 0x130], 0
1405c0527: mov word ptr [rbp + 0x134], 1
1405c0530: mov dword ptr [rbp + 0x138], 0
1405c053a: movzx eax, byte ptr [rip + 0x30746c8]
1405c0541: mov ecx, 0x198
1405c0546: mov edx, 8
1405c054b: call 0x140613c10
1405c0550: test rax, rax
1405c0553: je 0x1405c060b
1405c0559: mov rdi, rax
1405c055c: lea rdx, [rbp - 0x58]
1405c0560: mov r8d, 0x198
1405c0566: mov rcx, rax
1405c0569: call 0x142923c00
1405c056e: lea r8, [rip + 0x2444ed3]
1405c0575: lea rcx, [rbp + 0x140]
1405c057c: mov rdx, rdi
1405c057f: call 0x140ce4b10
1405c0584: mov byte ptr [rbp - 0x58], bl
1405c0587: mov rcx, qword ptr [rip + 0x306bde2]
1405c058e: lea r8, [rip + 0x236aa03]
1405c0595: lea rdx, [rbp - 0x58]
1405c0599: call 0x140364d20
1405c059e: mov r8, rdx
1405c05a1: lea rcx, [rbp + 0x140]
1405c05a8: mov rdx, rax
1405c05ab: call 0x1400d68c0
1405c05b0: movups xmm0, xmmword ptr [rbp + 0x1a0]
1405c05b7: movups xmmword ptr [rsi + 0x60], xmm0
1405c05bb: movups xmm0, xmmword ptr [rbp + 0x190]
1405c05c2: movups xmmword ptr [rsi + 0x50], xmm0
1405c05c6: movups xmm0, xmmword ptr [rbp + 0x180]
1405c05cd: movups xmmword ptr [rsi + 0x40], xmm0
1405c05d1: movups xmm0, xmmword ptr [rbp + 0x140]
1405c05d8: movups xmm1, xmmword ptr [rbp + 0x150]
1405c05df: movups xmm2, xmmword ptr [rbp + 0x160]
1405c05e6: movups xmm3, xmmword ptr [rbp + 0x170]
1405c05ed: movups xmmword ptr [rsi + 0x30], xmm3
1405c05f1: movups xmmword ptr [rsi + 0x20], xmm2
1405c05f5: movups xmmword ptr [rsi + 0x10], xmm1
1405c05f9: movups xmmword ptr [rsi], xmm0
1405c05fc: mov rax, rsi
1405c05ff: add rsp, 0x238
1405c0606: pop rbx
1405c0607: pop rdi
1405c0608: pop rsi
1405c0609: pop rbp
1405c060a: ret 
1405c060b: mov ecx, 8
1405c0610: mov edx, 0x198
1405c0615: call 0x1428d9000
1405c061a: ud2 
