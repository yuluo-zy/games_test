0x1409b9080: push rbp
0x1409b9081: push rsi
0x1409b9082: push rdi
0x1409b9083: push rbx
0x1409b9084: sub rsp, 0x338
0x1409b908b: lea rbp, [rsp + 0x80]
0x1409b9093: mov qword ptr [rbp + 0x2b0], 0xfffffffffffffffe
0x1409b909e: mov rsi, r8
0x1409b90a1: mov rdi, rdx
0x1409b90a4: mov rdx, rcx
0x1409b90a7: lea rcx, [rbp - 0x58]
0x1409b90ab: mov r8d, 0xf8
0x1409b90b1: call 0x142923c00
0x1409b90b6: mov qword ptr [rbp + 0xa0], rdi
0x1409b90bd: mov qword ptr [rbp + 0xa8], rsi
0x1409b90c4: lea rcx, [rsi + 0x38]
0x1409b90c8: mov byte ptr [rbp + 0x2af], 1
0x1409b90cf: movabs rdx, 0x98df08dcfa7ea5ba
0x1409b90d9: movabs r8, 0x67b117a60e632bc3
0x1409b90e3: call 0x1400dd6f0
0x1409b90e8: test rax, rax
0x1409b90eb: je 0x1409b936e
0x1409b90f1: cmp qword ptr [rsi + 0x218], rdx
0x1409b90f8: jbe 0x1409b936e
0x1409b90fe: mov rax, qword ptr [rsi + 0x210]
0x1409b9105: mov rcx, qword ptr [rax + rdx*8]
0x1409b9109: test rcx, rcx
0x1409b910c: je 0x1409b936e
0x1409b9112: mov rax, qword ptr [rsi + 0x1e0]
0x1409b9119: not rcx
0x1409b911c: lea rcx, [rcx + rcx*2]
0x1409b9120: shl rcx, 5
0x1409b9124: cmp qword ptr [rax + rcx + 0x20], 0
0x1409b912a: je 0x1409b936e
0x1409b9130: add rax, rcx
0x1409b9133: mov rbx, qword ptr [rax + 0x10]
0x1409b9137: lea rsi, [rbp + 0xb0]
0x1409b913e: lea rdx, [rbp - 0x58]
0x1409b9142: mov r8d, 0xf8
0x1409b9148: mov rcx, rsi
0x1409b914b: call 0x142923c00
0x1409b9150: movabs rax, 0xd47acc582783e1a8
0x1409b915a: mov qword ptr [rbp + 0x2a0], rax
0x1409b9161: mov dword ptr [rbp + 0x1a8], 2
0x1409b916b: lea rax, [rbp + 0x1a8]
0x1409b9172: mov qword ptr [rsp + 0x20], rax
0x1409b9177: lea rcx, [rbp + 0x2a0]
0x1409b917e: movabs r8, 0xff277bb82867f70e
0x1409b9188: mov rdx, rsi
0x1409b918b: mov r9b, 0x1d
0x1409b918e: call 0x140908610
0x1409b9193: lea rsi, [rbp + 0x1a8]
0x1409b919a: lea rdi, [rbp + 0xb0]
0x1409b91a1: mov r8d, 0xf8
0x1409b91a7: mov rcx, rsi
0x1409b91aa: mov rdx, rdi
0x1409b91ad: call 0x142923c00
0x1409b91b2: movabs rax, 0x394df4b2308658c8
0x1409b91bc: mov qword ptr [rbp + 0x2a0], rax
0x1409b91c3: mov dword ptr [rbp + 0xb0], 2
0x1409b91cd: mov qword ptr [rsp + 0x20], rdi
0x1409b91d2: lea rcx, [rbp + 0x2a0]
0x1409b91d9: movabs r8, 0xff2778b82867f1f5
0x1409b91e3: mov rdx, rsi
0x1409b91e6: mov r9b, 0x1d
0x1409b91e9: call 0x140908610
0x1409b91ee: lea rsi, [rbp + 0xb0]
0x1409b91f5: lea rdi, [rbp + 0x1a8]
0x1409b91fc: mov r8d, 0xf8
0x1409b9202: mov rcx, rsi
0x1409b9205: mov rdx, rdi
0x1409b9208: call 0x142923c00
0x1409b920d: movabs rax, 0xa9ffa026a69075eb
0x1409b9217: mov qword ptr [rbp + 0x2a0], rax
0x1409b921e: mov dword ptr [rbp + 0x1a8], 2
0x1409b9228: mov qword ptr [rsp + 0x20], rdi
0x1409b922d: lea rcx, [rbp + 0x2a0]
0x1409b9234: movabs r8, 0xff276ab82867da2b
0x1409b923e: mov rdx, rsi
0x1409b9241: mov r9b, 0x1d
0x1409b9244: call 0x140908610
0x1409b9249: lea rsi, [rbp + 0x1a8]
0x1409b9250: lea rdi, [rbp + 0xb0]
0x1409b9257: mov r8d, 0xf8
0x1409b925d: mov rcx, rsi
0x1409b9260: mov rdx, rdi
0x1409b9263: call 0x142923c00
0x1409b9268: movabs rax, 0x78624d43dedde98a
0x1409b9272: mov qword ptr [rbp + 0x2a0], rax
0x1409b9279: mov dword ptr [rbp + 0xb0], 2
0x1409b9283: mov qword ptr [rsp + 0x20], rdi
0x1409b9288: lea rcx, [rbp + 0x2a0]
0x1409b928f: movabs r8, 0xff276cb82867dd91
0x1409b9299: mov rdx, rsi
0x1409b929c: mov r9b, 0x1d
0x1409b929f: call 0x140908610
0x1409b92a4: lea rcx, [rbp + 0xb0]
0x1409b92ab: lea rdx, [rbp + 0x1a8]
0x1409b92b2: mov r8d, 0xf8
0x1409b92b8: call 0x142923c00
0x1409b92bd: cmp byte ptr [rbp + 0x18e], 8
0x1409b92c4: je 0x1409b92ed
0x1409b92c6: movss xmm3, dword ptr [rbx + 8]
0x1409b92cb: lea rcx, [rbp + 0x168]
0x1409b92d2: mov edx, 0xe8
0x1409b92d7: add rdx, qword ptr [rbp + 0x198]
0x1409b92de: movabs r8, 0x3cc185d80517a5cb
0x1409b92e8: call 0x14096ba30
0x1409b92ed: lea rcx, [rbp + 0x1a8]
0x1409b92f4: lea rdx, [rbp + 0xb0]
0x1409b92fb: mov r8d, 0xf8
0x1409b9301: call 0x142923c00
0x1409b9306: cmp byte ptr [rbp + 0x286], 8
0x1409b930d: je 0x1409b9336
0x1409b930f: movss xmm3, dword ptr [rbx + 0x10]
0x1409b9314: lea rcx, [rbp + 0x260]
0x1409b931b: mov edx, 0xe8
0x1409b9320: add rdx, qword ptr [rbp + 0x290]
0x1409b9327: movabs r8, 0x1986361629359548
0x1409b9331: call 0x14096ba30
0x1409b9336: lea rsi, [rbp + 0xb0]
0x1409b933d: lea rdx, [rbp + 0x1a8]
0x1409b9344: mov r8d, 0xf8
0x1409b934a: mov rcx, rsi
0x1409b934d: call 0x142923c00
0x1409b9352: mov byte ptr [rbp + 0x2af], 0
0x1409b9359: mov rcx, rsi
0x1409b935c: call 0x140906f40
0x1409b9361: nop 
0x1409b9362: add rsp, 0x338
0x1409b9369: pop rbx
0x1409b936a: pop rdi
0x1409b936b: pop rsi
0x1409b936c: pop rbp
0x1409b936d: ret 
0x1409b936e: mov byte ptr [rbp + 0x2af], 1
0x1409b9375: lea rcx, [rip + 0x2105e54]
0x1409b937c: call 0x1428d9310
0x1409b9381: ud2 
0x1409b9383: nop word ptr cs:[rax + rax]
0x1409b9390: mov qword ptr [rsp + 0x10], rdx
0x1409b9395: push rbp
0x1409b9396: push rsi
0x1409b9397: push rdi
0x1409b9398: push rbx
0x1409b9399: sub rsp, 0x28
0x1409b939d: lea rbp, [rdx + 0x80]
0x1409b93a4: mov byte ptr [rbp + 0x2af], 0
0x1409b93ab: lea rcx, [rbp + 0xb0]
0x1409b93b2: call 0x1408935c0
0x1409b93b7: mov byte ptr [rbp + 0x2af], 0
0x1409b93be: add rsp, 0x28
0x1409b93c2: pop rbx
0x1409b93c3: pop rdi
0x1409b93c4: pop rsi
0x1409b93c5: pop rbp
0x1409b93c6: ret 
0x1409b93c7: nop word ptr [rax + rax]
0x1409b93d0: mov qword ptr [rsp + 0x10], rdx
0x1409b93d5: push rbp
0x1409b93d6: push rsi
0x1409b93d7: push rdi
0x1409b93d8: push rbx
0x1409b93d9: sub rsp, 0x28
0x1409b93dd: lea rbp, [rdx + 0x80]
0x1409b93e4: cmp byte ptr [rbp + 0x2af], 0
0x1409b93eb: jne 0x1409b93f6
0x1409b93ed: add rsp, 0x28
0x1409b93f1: pop rbx
0x1409b93f2: pop rdi
0x1409b93f3: pop rsi
0x1409b93f4: pop rbp
0x1409b93f5: ret 
0x1409b93f6: lea rcx, [rbp - 0x58]
0x1409b93fa: call 0x1408935c0
0x1409b93ff: jmp 0x1409b93ed
0x1409b9401: nop word ptr cs:[rax + rax]
0x1409b9410: mov qword ptr [rsp + 0x10], rdx
0x1409b9415: push rbp
0x1409b9416: push rsi
0x1409b9417: push rdi
0x1409b9418: push rbx
0x1409b9419: sub rsp, 0x28
0x1409b941d: lea rbp, [rdx + 0x80]
0x1409b9424: mov byte ptr [rbp + 0x2af], 0
0x1409b942b: lea rcx, [rbp + 0x1a8]
0x1409b9432: call 0x1408935c0
0x1409b9437: mov byte ptr [rbp + 0x2af], 0
0x1409b943e: add rsp, 0x28
0x1409b9442: pop rbx
0x1409b9443: pop rdi
0x1409b9444: pop rsi
0x1409b9445: pop rbp
0x1409b9446: ret 
0x1409b9447: nop word ptr [rax + rax]
0x1409b9450: mov qword ptr [rsp + 0x10], rdx
0x1409b9455: push rbp
0x1409b9456: push rsi
0x1409b9457: push rdi
0x1409b9458: push rbx
0x1409b9459: sub rsp, 0x28
0x1409b945d: lea rbp, [rdx + 0x80]
0x1409b9464: mov byte ptr [rbp + 0x2af], 0
0x1409b946b: lea rcx, [rbp + 0xb0]
0x1409b9472: call 0x1408935c0
0x1409b9477: mov byte ptr [rbp + 0x2af], 0
0x1409b947e: add rsp, 0x28
0x1409b9482: pop rbx
0x1409b9483: pop rdi
0x1409b9484: pop rsi
0x1409b9485: pop rbp
0x1409b9486: ret 
0x1409b9487: nop word ptr [rax + rax]
0x1409b9490: mov qword ptr [rsp + 0x10], rdx
0x1409b9495: push rbp
0x1409b9496: push rsi
0x1409b9497: push rdi
0x1409b9498: push rbx
0x1409b9499: sub rsp, 0x28
0x1409b949d: lea rbp, [rdx + 0x80]
0x1409b94a4: mov byte ptr [rbp + 0x2af], 0
0x1409b94ab: lea rcx, [rbp + 0x1a8]
0x1409b94b2: call 0x1408935c0
0x1409b94b7: mov byte ptr [rbp + 0x2af], 0
0x1409b94be: add rsp, 0x28
0x1409b94c2: pop rbx
0x1409b94c3: pop rdi
0x1409b94c4: pop rsi
0x1409b94c5: pop rbp
0x1409b94c6: ret 
0x1409b94c7: nop word ptr [rax + rax]
0x1409b94d0: mov qword ptr [rsp + 0x10], rdx
0x1409b94d5: push rbp
0x1409b94d6: push rsi
0x1409b94d7: push rdi
0x1409b94d8: push rbx
0x1409b94d9: sub rsp, 0x28
0x1409b94dd: lea rbp, [rdx + 0x80]
0x1409b94e4: mov byte ptr [rbp + 0x2af], 0
0x1409b94eb: lea rcx, [rbp + 0xb0]
0x1409b94f2: call 0x1408935c0
0x1409b94f7: mov byte ptr [rbp + 0x2af], 0
0x1409b94fe: add rsp, 0x28
0x1409b9502: pop rbx
0x1409b9503: pop rdi
0x1409b9504: pop rsi
0x1409b9505: pop rbp
0x1409b9506: ret 
0x1409b9507: nop word ptr [rax + rax]
0x1409b9510: mov qword ptr [rsp + 0x10], rdx
0x1409b9515: push rbp
0x1409b9516: push rsi
0x1409b9517: push rdi
0x1409b9518: push rbx
0x1409b9519: sub rsp, 0x28
0x1409b951d: lea rbp, [rdx + 0x80]
0x1409b9524: mov byte ptr [rbp + 0x2af], 0
0x1409b952b: lea rcx, [rbp + 0x1a8]
0x1409b9532: call 0x1408935c0
0x1409b9537: mov byte ptr [rbp + 0x2af], 0
0x1409b953e: add rsp, 0x28
0x1409b9542: pop rbx
0x1409b9543: pop rdi
0x1409b9544: pop rsi
0x1409b9545: pop rbp
0x1409b9546: ret 
