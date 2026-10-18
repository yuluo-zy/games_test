140a91110: push r15
140a91112: push r14
140a91114: push r12
140a91116: push rsi
140a91117: push rdi
140a91118: push rbp
140a91119: push rbx
140a9111a: sub rsp, 0x120
140a91121: mov edi, 1
140a91126: lock xadd dword ptr [rdx + 0x5b0], edi
140a9112e: xor eax, eax
140a91130: cmp rax, qword ptr [rcx + 0x160]
140a91137: jo 0x140a914d5
140a9113d: mov rsi, rcx
140a91140: mov rax, qword ptr [rcx + 0x138]
140a91147: mov r11, qword ptr [rdx + 0x218]
140a9114e: cmp r11, rax
140a91151: jbe 0x140a913d5
140a91157: mov rbx, qword ptr [rdx + 0x210]
140a9115e: mov rax, qword ptr [rbx + rax*8]
140a91162: test rax, rax
140a91165: je 0x140a913d5
140a9116b: mov r8, qword ptr [rdx + 0x1e0]
140a91172: not rax
140a91175: lea rax, [rax + rax*2]
140a91179: shl rax, 5
140a9117d: cmp qword ptr [r8 + rax + 0x20], 0
140a91183: je 0x140a913d5
140a91189: mov rcx, qword ptr [rsi + 0x140]
140a91190: cmp r11, rcx
140a91193: jbe 0x140a913f3
140a91199: mov rcx, qword ptr [rbx + rcx*8]
140a9119d: test rcx, rcx
140a911a0: je 0x140a913f3
140a911a6: not rcx
140a911a9: lea rcx, [rcx + rcx*2]
140a911ad: shl rcx, 5
140a911b1: cmp qword ptr [r8 + rcx + 0x20], 0
140a911b7: je 0x140a913f3
140a911bd: mov r9, qword ptr [rsi + 0x148]
140a911c4: cmp r11, r9
140a911c7: jbe 0x140a91411
140a911cd: mov r9, qword ptr [rbx + r9*8]
140a911d1: test r9, r9
140a911d4: je 0x140a91411
140a911da: not r9
140a911dd: lea r9, [r9 + r9*2]
140a911e1: shl r9, 5
140a911e5: cmp qword ptr [r8 + r9 + 0x20], 0
140a911eb: je 0x140a91411
140a911f1: mov r10, qword ptr [rsi + 0x150]
140a911f8: cmp r11, r10
140a911fb: jbe 0x140a9142f
140a91201: mov r10, qword ptr [rbx + r10*8]
140a91205: test r10, r10
140a91208: je 0x140a9142f
140a9120e: not r10
140a91211: lea r10, [r10 + r10*2]
140a91215: shl r10, 5
140a91219: cmp qword ptr [r8 + r10 + 0x20], 0
140a9121f: je 0x140a9142f
140a91225: mov r14, qword ptr [rsi + 0x158]
140a9122c: cmp r11, r14
140a9122f: jbe 0x140a9144d
140a91235: mov r11, qword ptr [rbx + r14*8]
140a91239: test r11, r11
140a9123c: je 0x140a9144d
140a91242: not r11
140a91245: lea r11, [r11 + r11*2]
140a91249: shl r11, 5
140a9124d: cmp qword ptr [r8 + r11 + 0x20], 0
140a91253: je 0x140a9144d
140a91259: add rax, r8
140a9125c: mov r12, qword ptr [rax + 0x10]
140a91260: add rcx, r8
140a91263: mov r15, qword ptr [rcx + 0x10]
140a91267: add r9, r8
140a9126a: mov r14, qword ptr [r9 + 0x10]
140a9126e: add r10, r8
140a91271: mov rbx, qword ptr [r10 + 0x10]
140a91275: add r11, r8
140a91278: mov r8, qword ptr [r11 + 0x10]
140a9127c: mov qword ptr [rsp + 0x60], r12
140a91281: lea r12, [rax + 0x5c]
140a91285: add rax, 0x58
140a91289: mov qword ptr [rsp + 0x68], rax
140a9128e: mov qword ptr [rsp + 0x70], r12
140a91293: mov ebp, dword ptr [rsi + 0x130]
140a91299: mov dword ptr [rsp + 0x78], ebp
140a9129d: mov dword ptr [rsp + 0x7c], edi
140a912a1: mov qword ptr [rsp + 0x80], r15
140a912a9: lea rax, [rcx + 0x5c]
140a912ad: add rcx, 0x58
140a912b1: mov qword ptr [rsp + 0x88], rcx
140a912b9: mov qword ptr [rsp + 0x90], rax
140a912c1: mov dword ptr [rsp + 0x98], ebp
140a912c8: mov dword ptr [rsp + 0x9c], edi
140a912cf: mov qword ptr [rsp + 0xa0], r14
140a912d7: lea rax, [r9 + 0x5c]
140a912db: add r9, 0x58
140a912df: mov qword ptr [rsp + 0xa8], r9
140a912e7: mov qword ptr [rsp + 0xb0], rax
140a912ef: mov dword ptr [rsp + 0xb8], ebp
140a912f6: mov dword ptr [rsp + 0xbc], edi
140a912fd: mov qword ptr [rsp + 0xc0], rbx
140a91305: lea rax, [r10 + 0x5c]
140a91309: add r10, 0x58
140a9130d: mov qword ptr [rsp + 0xc8], r10
140a91315: mov qword ptr [rsp + 0xd0], rax
140a9131d: lea rcx, [rsi + 0x160]
140a91324: mov dword ptr [rsp + 0xd8], ebp
140a9132b: mov dword ptr [rsp + 0xdc], edi
140a91332: mov qword ptr [rsp + 0xe0], r8
140a9133a: lea rax, [r11 + 0x58]
140a9133e: add r11, 0x5c
140a91342: mov qword ptr [rsp + 0xe8], rax
140a9134a: lea rax, [rsp + 0x80]
140a91352: mov qword ptr [rsp + 0xf0], r11
140a9135a: lea r8, [rsp + 0xa0]
140a91362: mov dword ptr [rsp + 0xf8], ebp
140a91369: lea r9, [rsp + 0xc0]
140a91371: mov dword ptr [rsp + 0xfc], edi
140a91378: mov qword ptr [rsp + 0x100], rdx
140a91380: lea rdx, [rsp + 0xe0]
140a91388: mov qword ptr [rsp + 0x108], 0
140a91394: mov qword ptr [rsp + 0x110], rcx
140a9139c: lea rcx, [rsp + 0x100]
140a913a4: mov qword ptr [rsp + 0x28], rcx
140a913a9: mov qword ptr [rsp + 0x20], rdx
140a913ae: lea rcx, [rsp + 0x60]
140a913b3: mov rdx, rax
140a913b6: call 0x140a21620
140a913bb: mov dword ptr [rsi + 0x130], edi
140a913c1: xor eax, eax
140a913c3: add rsp, 0x120
140a913ca: pop rbx
140a913cb: pop rbp
140a913cc: pop rdi
140a913cd: pop rsi
140a913ce: pop r12
140a913d0: pop r14
140a913d2: pop r15
140a913d4: ret 
140a913d5: add rsi, 0x98
140a913dc: lea rax, [rip + 0x20487bd]
140a913e3: mov qword ptr [rsp + 0x30], rax
140a913e8: mov qword ptr [rsp + 0x38], 0x2c
140a913f1: jmp 0x140a91469
140a913f3: add rsi, 0x98
140a913fa: lea rax, [rip + 0x2048696]
140a91401: mov qword ptr [rsp + 0x30], rax
140a91406: mov qword ptr [rsp + 0x38], 0x3d
140a9140f: jmp 0x140a91469
140a91411: add rsi, 0x98
140a91418: lea rax, [rip + 0x20487e2]
140a9141f: mov qword ptr [rsp + 0x30], rax
140a91424: mov qword ptr [rsp + 0x38], 0x33
140a9142d: jmp 0x140a91469
140a9142f: add rsi, 0x98
140a91436: lea rax, [rip + 0x20487f7]
140a9143d: mov qword ptr [rsp + 0x30], rax
140a91442: mov qword ptr [rsp + 0x38], 0x41
140a9144b: jmp 0x140a91469
140a9144d: add rsi, 0x98
140a91454: lea rax, [rip + 0x2048771]
140a9145b: mov qword ptr [rsp + 0x30], rax
140a91460: mov qword ptr [rsp + 0x38], 0x35
140a91469: mov qword ptr [rsp + 0x40], rsi
140a9146e: lea rax, [rip - 0xa559a5]
140a91475: mov qword ptr [rsp + 0x48], rax
140a9147a: lea rax, [rsp + 0x30]
140a9147f: mov qword ptr [rsp + 0x50], rax
140a91484: lea rax, [rip - 0x9875fb]
140a9148b: mov qword ptr [rsp + 0x58], rax
140a91490: lea rax, [rip + 0x2048669]
140a91497: mov qword ptr [rsp + 0x60], rax
140a9149c: mov qword ptr [rsp + 0x68], 2
140a914a5: mov qword ptr [rsp + 0x80], 0
140a914b1: lea rax, [rsp + 0x40]
140a914b6: mov qword ptr [rsp + 0x70], rax
140a914bb: mov qword ptr [rsp + 0x78], 2
140a914c4: lea rdx, [rip + 0x20486bd]
140a914cb: lea rcx, [rsp + 0x60]
140a914d0: call 0x1428d9390
140a914d5: lea rcx, [rip + 0x2048874]
140a914dc: lea r8, [rip + 0x20488cd]
140a914e3: mov edx, 0x59
140a914e8: call 0x1428d9330
140a914ed: int3 
