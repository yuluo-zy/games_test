1412bf3f0: push rbp
1412bf3f1: push rsi
1412bf3f2: sub rsp, 0xe8
1412bf3f9: lea rbp, [rsp + 0x80]
1412bf401: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412bf409: mov rsi, rcx
1412bf40c: lea rcx, [rbp - 0x58]
1412bf410: xor r8d, r8d
1412bf413: call 0x14161da30
1412bf418: movzx eax, byte ptr [rip + 0x23757ea]
1412bf41f: mov ecx, 0x38
1412bf424: mov edx, 8
1412bf429: call 0x140613c10
1412bf42e: test rax, rax
1412bf431: je 0x1412bf4ef
1412bf437: mov rcx, qword ptr [rbp - 0x28]
1412bf43b: mov qword ptr [rax + 0x30], rcx
1412bf43f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bf443: movups xmm1, xmmword ptr [rbp - 0x48]
1412bf447: movups xmm2, xmmword ptr [rbp - 0x38]
1412bf44b: movups xmmword ptr [rax + 0x20], xmm2
1412bf44f: movups xmmword ptr [rax + 0x10], xmm1
1412bf453: movups xmmword ptr [rax], xmm0
1412bf456: lea rcx, [rip + 0x17dee23]
1412bf45d: mov qword ptr [rbp + 0x10], rcx
1412bf461: mov qword ptr [rbp + 0x18], 0x49
1412bf469: mov qword ptr [rbp + 0x20], rax
1412bf46d: lea rax, [rip + 0x196d56c]
1412bf474: mov qword ptr [rbp + 0x28], rax
1412bf478: mov qword ptr [rbp + 0x40], 0
1412bf480: mov byte ptr [rbp + 0x56], 5
1412bf484: mov qword ptr [rbp + 0x30], rcx
1412bf488: mov qword ptr [rbp + 0x38], 0x49
1412bf490: movzx eax, byte ptr [rip + 0x2375772]
1412bf497: mov ecx, 0x40
1412bf49c: mov edx, 8
1412bf4a1: call 0x140613c10
1412bf4a6: test rax, rax
1412bf4a9: je 0x1412bf500
1412bf4ab: movups xmm0, xmmword ptr [rbp + 0x20]
1412bf4af: movups xmm1, xmmword ptr [rbp + 0x30]
1412bf4b3: movups xmm2, xmmword ptr [rbp + 0x40]
1412bf4b7: movups xmm3, xmmword ptr [rbp + 0x50]
1412bf4bb: movups xmmword ptr [rax + 0x30], xmm3
1412bf4bf: movups xmmword ptr [rax + 0x20], xmm2
1412bf4c3: movups xmmword ptr [rax + 0x10], xmm1
1412bf4c7: movups xmmword ptr [rax], xmm0
1412bf4ca: movabs rdx, 0xc6df81f12eb726e7
1412bf4d4: mov rcx, rsi
1412bf4d7: mov r8, rax
1412bf4da: call 0x140969bf0
1412bf4df: cmp rax, 1
1412bf4e3: je 0x1412bf511
1412bf4e5: add rsp, 0xe8
1412bf4ec: pop rsi
1412bf4ed: pop rbp
1412bf4ee: ret 
1412bf4ef: mov ecx, 8
1412bf4f4: mov edx, 0x38
1412bf4f9: call 0x1428d9000
1412bf4fe: jmp 0x1412bf50f
1412bf500: mov ecx, 8
1412bf505: mov edx, 0x40
1412bf50a: call 0x1428d9000
1412bf50f: ud2 
1412bf511: mov rax, rdx
1412bf514: add rdx, 0x10
1412bf518: mov rsi, rdx
1412bf51b: cmp qword ptr [rax + 0x18], 0x49
1412bf520: jne 0x1412bf59f
1412bf522: mov rdx, qword ptr [rsi]
1412bf525: lea rcx, [rip + 0x17ded54]
1412bf52c: mov r8d, 0x49
1412bf532: call 0x142923bf0
1412bf537: test eax, eax
1412bf539: jne 0x1412bf59f
1412bf53b: lea rax, [rip + 0x196e1fe]
1412bf542: mov qword ptr [rbp - 0x20], rax
1412bf546: lea rax, [rip - 0xd8bcad]
1412bf54d: mov qword ptr [rbp - 0x18], rax
1412bf551: lea rax, [rbp + 0x10]
1412bf555: mov qword ptr [rbp - 0x10], rax
1412bf559: lea rax, [rip - 0x11b56d0]
1412bf560: mov qword ptr [rbp - 8], rax
1412bf564: lea rax, [rip + 0x196e0bd]
1412bf56b: mov qword ptr [rbp + 0x20], rax
1412bf56f: mov qword ptr [rbp + 0x28], 3
1412bf577: mov qword ptr [rbp + 0x40], 0
1412bf57f: lea rax, [rbp - 0x20]
1412bf583: mov qword ptr [rbp + 0x30], rax
1412bf587: mov qword ptr [rbp + 0x38], 2
1412bf58f: lea rdx, [rip + 0x196e0c2]
1412bf596: lea rcx, [rbp + 0x20]
1412bf59a: call 0x1428d9390
1412bf59f: lea rax, [rip + 0x196e19a]
1412bf5a6: mov qword ptr [rbp + 0x20], rax
1412bf5aa: lea rax, [rip - 0xd8bd11]
1412bf5b1: mov qword ptr [rbp + 0x28], rax
1412bf5b5: mov qword ptr [rbp + 0x30], rsi
1412bf5b9: lea rax, [rip - 0x11b5730]
1412bf5c0: mov qword ptr [rbp + 0x38], rax
1412bf5c4: lea rcx, [rbp + 0x10]
1412bf5c8: mov qword ptr [rbp + 0x40], rcx
1412bf5cc: mov qword ptr [rbp + 0x48], rax
1412bf5d0: lea rax, [rip + 0x196df91]
1412bf5d7: mov qword ptr [rbp - 0x20], rax
1412bf5db: mov qword ptr [rbp - 0x18], 3
1412bf5e3: mov qword ptr [rbp], 0
1412bf5eb: lea rax, [rbp + 0x20]
1412bf5ef: mov qword ptr [rbp - 0x10], rax
1412bf5f3: mov qword ptr [rbp - 8], 3
1412bf5fb: lea rdx, [rip + 0x196dfce]
1412bf602: lea rcx, [rbp - 0x20]
1412bf606: call 0x1428d9390
1412bf60b: int3 
1412bf60c: nop dword ptr [rax]
1412bf610: mov qword ptr [rsp + 0x10], rdx
1412bf615: push rbp
1412bf616: push rsi
1412bf617: sub rsp, 0x28
1412bf61b: lea rbp, [rdx + 0x80]
1412bf622: lea rcx, [rbp - 0x48]
1412bf626: call 0x140be24b0
1412bf62b: nop 
1412bf62c: add rsp, 0x28
1412bf630: pop rsi
1412bf631: pop rbp
1412bf632: ret 
1412bf633: nop word ptr cs:[rax + rax]
1412bf640: mov qword ptr [rsp + 0x10], rdx
1412bf645: push rbp
1412bf646: push rsi
1412bf647: sub rsp, 0x28
1412bf64b: lea rbp, [rdx + 0x80]
1412bf652: lea rcx, [rbp + 0x20]
1412bf656: call 0x1400c04b0
1412bf65b: nop 
1412bf65c: add rsp, 0x28
1412bf660: pop rsi
1412bf661: pop rbp
1412bf662: ret 
