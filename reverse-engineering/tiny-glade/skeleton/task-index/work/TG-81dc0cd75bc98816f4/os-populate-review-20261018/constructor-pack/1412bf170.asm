1412bf170: push rbp
1412bf171: push rsi
1412bf172: sub rsp, 0xd8
1412bf179: lea rbp, [rsp + 0x80]
1412bf181: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bf189: mov rsi, rcx
1412bf18c: lea rcx, [rbp - 0x58]
1412bf190: xor r8d, r8d
1412bf193: call 0x14161a760
1412bf198: mov qword ptr [rbp - 0x60], 0
1412bf1a0: movzx eax, byte ptr [rip + 0x2375a62]
1412bf1a7: mov ecx, 0x30
1412bf1ac: mov edx, 8
1412bf1b1: call 0x140613c10
1412bf1b6: test rax, rax
1412bf1b9: je 0x1412bf26f
1412bf1bf: movups xmm0, xmmword ptr [rbp - 0x60]
1412bf1c3: movups xmm1, xmmword ptr [rbp - 0x50]
1412bf1c7: movups xmm2, xmmword ptr [rbp - 0x40]
1412bf1cb: movups xmmword ptr [rax + 0x20], xmm2
1412bf1cf: movups xmmword ptr [rax + 0x10], xmm1
1412bf1d3: movups xmmword ptr [rax], xmm0
1412bf1d6: lea rcx, [rip + 0x17df689]
1412bf1dd: mov qword ptr [rbp], rcx
1412bf1e1: mov qword ptr [rbp + 8], 0x37
1412bf1e9: mov qword ptr [rbp + 0x10], rax
1412bf1ed: lea rax, [rip + 0x196d7b4]
1412bf1f4: mov qword ptr [rbp + 0x18], rax
1412bf1f8: mov qword ptr [rbp + 0x30], 0
1412bf200: mov byte ptr [rbp + 0x46], 5
1412bf204: mov qword ptr [rbp + 0x20], rcx
1412bf208: mov qword ptr [rbp + 0x28], 0x37
1412bf210: movzx eax, byte ptr [rip + 0x23759f2]
1412bf217: mov ecx, 0x40
1412bf21c: mov edx, 8
1412bf221: call 0x140613c10
1412bf226: test rax, rax
1412bf229: je 0x1412bf280
1412bf22b: movups xmm0, xmmword ptr [rbp + 0x10]
1412bf22f: movups xmm1, xmmword ptr [rbp + 0x20]
1412bf233: movups xmm2, xmmword ptr [rbp + 0x30]
1412bf237: movups xmm3, xmmword ptr [rbp + 0x40]
1412bf23b: movups xmmword ptr [rax + 0x30], xmm3
1412bf23f: movups xmmword ptr [rax + 0x20], xmm2
1412bf243: movups xmmword ptr [rax + 0x10], xmm1
1412bf247: movups xmmword ptr [rax], xmm0
1412bf24a: movabs rdx, 0x13c9c799697a906c
1412bf254: mov rcx, rsi
1412bf257: mov r8, rax
1412bf25a: call 0x140969bf0
1412bf25f: cmp rax, 1
1412bf263: je 0x1412bf291
1412bf265: add rsp, 0xd8
1412bf26c: pop rsi
1412bf26d: pop rbp
1412bf26e: ret 
1412bf26f: mov ecx, 8
1412bf274: mov edx, 0x30
1412bf279: call 0x1428d9000
1412bf27e: jmp 0x1412bf28f
1412bf280: mov ecx, 8
1412bf285: mov edx, 0x40
1412bf28a: call 0x1428d9000
1412bf28f: ud2 
1412bf291: mov rax, rdx
1412bf294: add rdx, 0x10
1412bf298: mov rsi, rdx
1412bf29b: cmp qword ptr [rax + 0x18], 0x37
1412bf2a0: jne 0x1412bf31e
1412bf2a2: mov rdx, qword ptr [rsi]
1412bf2a5: lea rcx, [rip + 0x17df5ba]
1412bf2ac: mov r8d, 0x37
1412bf2b2: call 0x142923bf0
1412bf2b7: test eax, eax
1412bf2b9: jne 0x1412bf31e
1412bf2bb: lea rax, [rip + 0x196ea26]
1412bf2c2: mov qword ptr [rbp - 0x30], rax
1412bf2c6: lea rax, [rip - 0xd8ba2d]
1412bf2cd: mov qword ptr [rbp - 0x28], rax
1412bf2d1: mov rax, rbp
1412bf2d4: mov qword ptr [rbp - 0x20], rax
1412bf2d8: lea rax, [rip - 0x11b544f]
1412bf2df: mov qword ptr [rbp - 0x18], rax
1412bf2e3: lea rax, [rip + 0x196e33e]
1412bf2ea: mov qword ptr [rbp + 0x10], rax
1412bf2ee: mov qword ptr [rbp + 0x18], 3
1412bf2f6: mov qword ptr [rbp + 0x30], 0
1412bf2fe: lea rax, [rbp - 0x30]
1412bf302: mov qword ptr [rbp + 0x20], rax
1412bf306: mov qword ptr [rbp + 0x28], 2
1412bf30e: lea rdx, [rip + 0x196e343]
1412bf315: lea rcx, [rbp + 0x10]
1412bf319: call 0x1428d9390
1412bf31e: lea rax, [rip + 0x196e9c3]
1412bf325: mov qword ptr [rbp + 0x10], rax
1412bf329: lea rax, [rip - 0xd8ba90]
1412bf330: mov qword ptr [rbp + 0x18], rax
1412bf334: mov qword ptr [rbp + 0x20], rsi
1412bf338: lea rax, [rip - 0x11b54af]
1412bf33f: mov qword ptr [rbp + 0x28], rax
1412bf343: mov rcx, rbp
1412bf346: mov qword ptr [rbp + 0x30], rcx
1412bf34a: mov qword ptr [rbp + 0x38], rax
1412bf34e: lea rax, [rip + 0x196e213]
1412bf355: mov qword ptr [rbp - 0x30], rax
1412bf359: mov qword ptr [rbp - 0x28], 3
1412bf361: mov qword ptr [rbp - 0x10], 0
1412bf369: lea rax, [rbp + 0x10]
1412bf36d: mov qword ptr [rbp - 0x20], rax
1412bf371: mov qword ptr [rbp - 0x18], 3
1412bf379: lea rdx, [rip + 0x196e250]
1412bf380: lea rcx, [rbp - 0x30]
1412bf384: call 0x1428d9390
1412bf389: int3 
1412bf38a: nop word ptr [rax + rax]
1412bf390: mov qword ptr [rsp + 0x10], rdx
1412bf395: push rbp
1412bf396: push rsi
1412bf397: sub rsp, 0x28
1412bf39b: lea rbp, [rdx + 0x80]
1412bf3a2: lea rcx, [rbp - 0x58]
1412bf3a6: call 0x140be24b0
1412bf3ab: nop 
1412bf3ac: add rsp, 0x28
1412bf3b0: pop rsi
1412bf3b1: pop rbp
1412bf3b2: ret 
1412bf3b3: nop word ptr cs:[rax + rax]
1412bf3c0: mov qword ptr [rsp + 0x10], rdx
1412bf3c5: push rbp
1412bf3c6: push rsi
1412bf3c7: sub rsp, 0x28
1412bf3cb: lea rbp, [rdx + 0x80]
1412bf3d2: lea rcx, [rbp + 0x10]
1412bf3d6: call 0x1400c04b0
1412bf3db: nop 
1412bf3dc: add rsp, 0x28
1412bf3e0: pop rsi
1412bf3e1: pop rbp
1412bf3e2: ret 
