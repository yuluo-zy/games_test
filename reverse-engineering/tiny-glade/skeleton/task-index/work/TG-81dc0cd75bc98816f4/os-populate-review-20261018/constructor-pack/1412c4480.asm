1412c4480: push rbp
1412c4481: push rsi
1412c4482: sub rsp, 0xd8
1412c4489: lea rbp, [rsp + 0x80]
1412c4491: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c4499: mov rsi, rcx
1412c449c: lea rcx, [rbp - 0x58]
1412c44a0: xor r8d, r8d
1412c44a3: call 0x141618fd0
1412c44a8: mov qword ptr [rbp - 0x60], 0
1412c44b0: movzx eax, byte ptr [rip + 0x2370752]
1412c44b7: mov ecx, 0x30
1412c44bc: mov edx, 8
1412c44c1: call 0x140613c10
1412c44c6: test rax, rax
1412c44c9: je 0x1412c457f
1412c44cf: movups xmm0, xmmword ptr [rbp - 0x60]
1412c44d3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c44d7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c44db: movups xmmword ptr [rax + 0x20], xmm2
1412c44df: movups xmmword ptr [rax + 0x10], xmm1
1412c44e3: movups xmmword ptr [rax], xmm0
1412c44e6: lea rcx, [rip + 0x17d961f]
1412c44ed: mov qword ptr [rbp], rcx
1412c44f1: mov qword ptr [rbp + 8], 0x50
1412c44f9: mov qword ptr [rbp + 0x10], rax
1412c44fd: lea rax, [rip + 0x1968bdc]
1412c4504: mov qword ptr [rbp + 0x18], rax
1412c4508: mov qword ptr [rbp + 0x30], 0
1412c4510: mov byte ptr [rbp + 0x46], 5
1412c4514: mov qword ptr [rbp + 0x20], rcx
1412c4518: mov qword ptr [rbp + 0x28], 0x50
1412c4520: movzx eax, byte ptr [rip + 0x23706e2]
1412c4527: mov ecx, 0x40
1412c452c: mov edx, 8
1412c4531: call 0x140613c10
1412c4536: test rax, rax
1412c4539: je 0x1412c4590
1412c453b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c453f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c4543: movups xmm2, xmmword ptr [rbp + 0x30]
1412c4547: movups xmm3, xmmword ptr [rbp + 0x40]
1412c454b: movups xmmword ptr [rax + 0x30], xmm3
1412c454f: movups xmmword ptr [rax + 0x20], xmm2
1412c4553: movups xmmword ptr [rax + 0x10], xmm1
1412c4557: movups xmmword ptr [rax], xmm0
1412c455a: movabs rdx, 0xf7c8c9c2eb9e16fb
1412c4564: mov rcx, rsi
1412c4567: mov r8, rax
1412c456a: call 0x140969bf0
1412c456f: cmp rax, 1
1412c4573: je 0x1412c45a1
1412c4575: add rsp, 0xd8
1412c457c: pop rsi
1412c457d: pop rbp
1412c457e: ret 
1412c457f: mov ecx, 8
1412c4584: mov edx, 0x30
1412c4589: call 0x1428d9000
1412c458e: jmp 0x1412c459f
1412c4590: mov ecx, 8
1412c4595: mov edx, 0x40
1412c459a: call 0x1428d9000
1412c459f: ud2 
1412c45a1: mov rax, rdx
1412c45a4: add rdx, 0x10
1412c45a8: mov rsi, rdx
1412c45ab: cmp qword ptr [rax + 0x18], 0x50
1412c45b0: jne 0x1412c462e
1412c45b2: mov rdx, qword ptr [rsi]
1412c45b5: lea rcx, [rip + 0x17d9550]
1412c45bc: mov r8d, 0x50
1412c45c2: call 0x142923bf0
1412c45c7: test eax, eax
1412c45c9: jne 0x1412c462e
1412c45cb: lea rax, [rip + 0x196923e]
1412c45d2: mov qword ptr [rbp - 0x30], rax
1412c45d6: lea rax, [rip - 0xd90d3d]
1412c45dd: mov qword ptr [rbp - 0x28], rax
1412c45e1: mov rax, rbp
1412c45e4: mov qword ptr [rbp - 0x20], rax
1412c45e8: lea rax, [rip - 0x11ba75f]
1412c45ef: mov qword ptr [rbp - 0x18], rax
1412c45f3: lea rax, [rip + 0x196902e]
1412c45fa: mov qword ptr [rbp + 0x10], rax
1412c45fe: mov qword ptr [rbp + 0x18], 3
1412c4606: mov qword ptr [rbp + 0x30], 0
1412c460e: lea rax, [rbp - 0x30]
1412c4612: mov qword ptr [rbp + 0x20], rax
1412c4616: mov qword ptr [rbp + 0x28], 2
1412c461e: lea rdx, [rip + 0x1969033]
1412c4625: lea rcx, [rbp + 0x10]
1412c4629: call 0x1428d9390
1412c462e: lea rax, [rip + 0x19691db]
1412c4635: mov qword ptr [rbp + 0x10], rax
1412c4639: lea rax, [rip - 0xd90da0]
1412c4640: mov qword ptr [rbp + 0x18], rax
1412c4644: mov qword ptr [rbp + 0x20], rsi
1412c4648: lea rax, [rip - 0x11ba7bf]
1412c464f: mov qword ptr [rbp + 0x28], rax
1412c4653: mov rcx, rbp
1412c4656: mov qword ptr [rbp + 0x30], rcx
1412c465a: mov qword ptr [rbp + 0x38], rax
1412c465e: lea rax, [rip + 0x1968f03]
1412c4665: mov qword ptr [rbp - 0x30], rax
1412c4669: mov qword ptr [rbp - 0x28], 3
1412c4671: mov qword ptr [rbp - 0x10], 0
1412c4679: lea rax, [rbp + 0x10]
1412c467d: mov qword ptr [rbp - 0x20], rax
1412c4681: mov qword ptr [rbp - 0x18], 3
1412c4689: lea rdx, [rip + 0x1968f40]
1412c4690: lea rcx, [rbp - 0x30]
1412c4694: call 0x1428d9390
1412c4699: int3 
1412c469a: nop word ptr [rax + rax]
1412c46a0: mov qword ptr [rsp + 0x10], rdx
1412c46a5: push rbp
1412c46a6: push rsi
1412c46a7: sub rsp, 0x28
1412c46ab: lea rbp, [rdx + 0x80]
1412c46b2: lea rcx, [rbp - 0x58]
1412c46b6: call 0x140be24b0
1412c46bb: nop 
1412c46bc: add rsp, 0x28
1412c46c0: pop rsi
1412c46c1: pop rbp
1412c46c2: ret 
1412c46c3: nop word ptr cs:[rax + rax]
1412c46d0: mov qword ptr [rsp + 0x10], rdx
1412c46d5: push rbp
1412c46d6: push rsi
1412c46d7: sub rsp, 0x28
1412c46db: lea rbp, [rdx + 0x80]
1412c46e2: lea rcx, [rbp + 0x10]
1412c46e6: call 0x1400c04b0
1412c46eb: nop 
1412c46ec: add rsp, 0x28
1412c46f0: pop rsi
1412c46f1: pop rbp
1412c46f2: ret 
