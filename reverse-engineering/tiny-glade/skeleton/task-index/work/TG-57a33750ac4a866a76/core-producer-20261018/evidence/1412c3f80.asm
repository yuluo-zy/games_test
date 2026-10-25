1412c3f80: push rbp
1412c3f81: push rsi
1412c3f82: sub rsp, 0xd8
1412c3f89: lea rbp, [rsp + 0x80]
1412c3f91: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c3f99: mov rsi, rcx
1412c3f9c: lea rcx, [rbp - 0x58]
1412c3fa0: xor r8d, r8d
1412c3fa3: call 0x1416159e0
1412c3fa8: mov qword ptr [rbp - 0x60], 0
1412c3fb0: movzx eax, byte ptr [rip + 0x2370c52]
1412c3fb7: mov ecx, 0x30
1412c3fbc: mov edx, 8
1412c3fc1: call 0x140613c10
1412c3fc6: test rax, rax
1412c3fc9: je 0x1412c407f
1412c3fcf: movups xmm0, xmmword ptr [rbp - 0x60]
1412c3fd3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c3fd7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c3fdb: movups xmmword ptr [rax + 0x20], xmm2
1412c3fdf: movups xmmword ptr [rax + 0x10], xmm1
1412c3fe3: movups xmmword ptr [rax], xmm0
1412c3fe6: lea rcx, [rip + 0x17d9554]
1412c3fed: mov qword ptr [rbp], rcx
1412c3ff1: mov qword ptr [rbp + 8], 0x43
1412c3ff9: mov qword ptr [rbp + 0x10], rax
1412c3ffd: lea rax, [rip + 0x196906c]
1412c4004: mov qword ptr [rbp + 0x18], rax
1412c4008: mov qword ptr [rbp + 0x30], 0
1412c4010: mov byte ptr [rbp + 0x46], 5
1412c4014: mov qword ptr [rbp + 0x20], rcx
1412c4018: mov qword ptr [rbp + 0x28], 0x43
1412c4020: movzx eax, byte ptr [rip + 0x2370be2]
1412c4027: mov ecx, 0x40
1412c402c: mov edx, 8
1412c4031: call 0x140613c10
1412c4036: test rax, rax
1412c4039: je 0x1412c4090
1412c403b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c403f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c4043: movups xmm2, xmmword ptr [rbp + 0x30]
1412c4047: movups xmm3, xmmword ptr [rbp + 0x40]
1412c404b: movups xmmword ptr [rax + 0x30], xmm3
1412c404f: movups xmmword ptr [rax + 0x20], xmm2
1412c4053: movups xmmword ptr [rax + 0x10], xmm1
1412c4057: movups xmmword ptr [rax], xmm0
1412c405a: movabs rdx, 0x96675839a35d7e8f
1412c4064: mov rcx, rsi
1412c4067: mov r8, rax
1412c406a: call 0x140969bf0
1412c406f: cmp rax, 1
1412c4073: je 0x1412c40a1
1412c4075: add rsp, 0xd8
1412c407c: pop rsi
1412c407d: pop rbp
1412c407e: ret 
1412c407f: mov ecx, 8
1412c4084: mov edx, 0x30
1412c4089: call 0x1428d9000
1412c408e: jmp 0x1412c409f
1412c4090: mov ecx, 8
1412c4095: mov edx, 0x40
1412c409a: call 0x1428d9000
1412c409f: ud2 
1412c40a1: mov rax, rdx
1412c40a4: add rdx, 0x10
1412c40a8: mov rsi, rdx
1412c40ab: cmp qword ptr [rax + 0x18], 0x43
1412c40b0: jne 0x1412c412e
1412c40b2: mov rdx, qword ptr [rsi]
1412c40b5: lea rcx, [rip + 0x17d9485]
1412c40bc: mov r8d, 0x43
1412c40c2: call 0x142923bf0
1412c40c7: test eax, eax
1412c40c9: jne 0x1412c412e
1412c40cb: lea rax, [rip + 0x1969dc6]
1412c40d2: mov qword ptr [rbp - 0x30], rax
1412c40d6: lea rax, [rip - 0xd9083d]
1412c40dd: mov qword ptr [rbp - 0x28], rax
1412c40e1: mov rax, rbp
1412c40e4: mov qword ptr [rbp - 0x20], rax
1412c40e8: lea rax, [rip - 0x11ba25f]
1412c40ef: mov qword ptr [rbp - 0x18], rax
1412c40f3: lea rax, [rip + 0x196952e]
1412c40fa: mov qword ptr [rbp + 0x10], rax
1412c40fe: mov qword ptr [rbp + 0x18], 3
1412c4106: mov qword ptr [rbp + 0x30], 0
1412c410e: lea rax, [rbp - 0x30]
1412c4112: mov qword ptr [rbp + 0x20], rax
1412c4116: mov qword ptr [rbp + 0x28], 2
1412c411e: lea rdx, [rip + 0x1969533]
1412c4125: lea rcx, [rbp + 0x10]
1412c4129: call 0x1428d9390
1412c412e: lea rax, [rip + 0x1969d63]
1412c4135: mov qword ptr [rbp + 0x10], rax
1412c4139: lea rax, [rip - 0xd908a0]
1412c4140: mov qword ptr [rbp + 0x18], rax
1412c4144: mov qword ptr [rbp + 0x20], rsi
1412c4148: lea rax, [rip - 0x11ba2bf]
1412c414f: mov qword ptr [rbp + 0x28], rax
1412c4153: mov rcx, rbp
1412c4156: mov qword ptr [rbp + 0x30], rcx
1412c415a: mov qword ptr [rbp + 0x38], rax
1412c415e: lea rax, [rip + 0x1969403]
1412c4165: mov qword ptr [rbp - 0x30], rax
1412c4169: mov qword ptr [rbp - 0x28], 3
1412c4171: mov qword ptr [rbp - 0x10], 0
1412c4179: lea rax, [rbp + 0x10]
1412c417d: mov qword ptr [rbp - 0x20], rax
1412c4181: mov qword ptr [rbp - 0x18], 3
1412c4189: lea rdx, [rip + 0x1969440]
1412c4190: lea rcx, [rbp - 0x30]
1412c4194: call 0x1428d9390
1412c4199: int3 
1412c419a: nop word ptr [rax + rax]
1412c41a0: mov qword ptr [rsp + 0x10], rdx
1412c41a5: push rbp
1412c41a6: push rsi
1412c41a7: sub rsp, 0x28
1412c41ab: lea rbp, [rdx + 0x80]
1412c41b2: lea rcx, [rbp - 0x58]
1412c41b6: call 0x140be24b0
1412c41bb: nop 
1412c41bc: add rsp, 0x28
1412c41c0: pop rsi
1412c41c1: pop rbp
1412c41c2: ret 
1412c41c3: nop word ptr cs:[rax + rax]
1412c41d0: mov qword ptr [rsp + 0x10], rdx
1412c41d5: push rbp
1412c41d6: push rsi
1412c41d7: sub rsp, 0x28
1412c41db: lea rbp, [rdx + 0x80]
1412c41e2: lea rcx, [rbp + 0x10]
1412c41e6: call 0x1400c04b0
1412c41eb: nop 
1412c41ec: add rsp, 0x28
1412c41f0: pop rsi
1412c41f1: pop rbp
1412c41f2: ret 
