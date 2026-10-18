1412c4200: push rbp
1412c4201: push rsi
1412c4202: sub rsp, 0xd8
1412c4209: lea rbp, [rsp + 0x80]
1412c4211: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c4219: mov rsi, rcx
1412c421c: lea rcx, [rbp - 0x58]
1412c4220: xor r8d, r8d
1412c4223: call 0x1416150a0
1412c4228: movzx eax, byte ptr [rip + 0x23709da]
1412c422f: mov ecx, 0x28
1412c4234: mov edx, 8
1412c4239: call 0x140613c10
1412c423e: test rax, rax
1412c4241: je 0x1412c42f7
1412c4247: mov rcx, qword ptr [rbp - 0x38]
1412c424b: mov qword ptr [rax + 0x20], rcx
1412c424f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c4253: movups xmm1, xmmword ptr [rbp - 0x48]
1412c4257: movups xmmword ptr [rax + 0x10], xmm1
1412c425b: movups xmmword ptr [rax], xmm0
1412c425e: lea rcx, [rip + 0x17da847]
1412c4265: mov qword ptr [rbp], rcx
1412c4269: mov qword ptr [rbp + 8], 0x30
1412c4271: mov qword ptr [rbp + 0x10], rax
1412c4275: lea rax, [rip + 0x1968e2c]
1412c427c: mov qword ptr [rbp + 0x18], rax
1412c4280: mov qword ptr [rbp + 0x30], 0
1412c4288: mov byte ptr [rbp + 0x46], 5
1412c428c: mov qword ptr [rbp + 0x20], rcx
1412c4290: mov qword ptr [rbp + 0x28], 0x30
1412c4298: movzx eax, byte ptr [rip + 0x237096a]
1412c429f: mov ecx, 0x40
1412c42a4: mov edx, 8
1412c42a9: call 0x140613c10
1412c42ae: test rax, rax
1412c42b1: je 0x1412c4308
1412c42b3: movups xmm0, xmmword ptr [rbp + 0x10]
1412c42b7: movups xmm1, xmmword ptr [rbp + 0x20]
1412c42bb: movups xmm2, xmmword ptr [rbp + 0x30]
1412c42bf: movups xmm3, xmmword ptr [rbp + 0x40]
1412c42c3: movups xmmword ptr [rax + 0x30], xmm3
1412c42c7: movups xmmword ptr [rax + 0x20], xmm2
1412c42cb: movups xmmword ptr [rax + 0x10], xmm1
1412c42cf: movups xmmword ptr [rax], xmm0
1412c42d2: movabs rdx, 0x28351c5dbc9c80fb
1412c42dc: mov rcx, rsi
1412c42df: mov r8, rax
1412c42e2: call 0x140969bf0
1412c42e7: cmp rax, 1
1412c42eb: je 0x1412c4319
1412c42ed: add rsp, 0xd8
1412c42f4: pop rsi
1412c42f5: pop rbp
1412c42f6: ret 
1412c42f7: mov ecx, 8
1412c42fc: mov edx, 0x28
1412c4301: call 0x1428d9000
1412c4306: jmp 0x1412c4317
1412c4308: mov ecx, 8
1412c430d: mov edx, 0x40
1412c4312: call 0x1428d9000
1412c4317: ud2 
1412c4319: mov rax, rdx
1412c431c: add rdx, 0x10
1412c4320: mov rsi, rdx
1412c4323: cmp qword ptr [rax + 0x18], 0x30
1412c4328: jne 0x1412c43a6
1412c432a: mov rdx, qword ptr [rsi]
1412c432d: lea rcx, [rip + 0x17da778]
1412c4334: mov r8d, 0x30
1412c433a: call 0x142923bf0
1412c433f: test eax, eax
1412c4341: jne 0x1412c43a6
1412c4343: lea rax, [rip + 0x196978e]
1412c434a: mov qword ptr [rbp - 0x30], rax
1412c434e: lea rax, [rip - 0xd90ab5]
1412c4355: mov qword ptr [rbp - 0x28], rax
1412c4359: mov rax, rbp
1412c435c: mov qword ptr [rbp - 0x20], rax
1412c4360: lea rax, [rip - 0x11ba4d7]
1412c4367: mov qword ptr [rbp - 0x18], rax
1412c436b: lea rax, [rip + 0x19692b6]
1412c4372: mov qword ptr [rbp + 0x10], rax
1412c4376: mov qword ptr [rbp + 0x18], 3
1412c437e: mov qword ptr [rbp + 0x30], 0
1412c4386: lea rax, [rbp - 0x30]
1412c438a: mov qword ptr [rbp + 0x20], rax
1412c438e: mov qword ptr [rbp + 0x28], 2
1412c4396: lea rdx, [rip + 0x19692bb]
1412c439d: lea rcx, [rbp + 0x10]
1412c43a1: call 0x1428d9390
1412c43a6: lea rax, [rip + 0x196972b]
1412c43ad: mov qword ptr [rbp + 0x10], rax
1412c43b1: lea rax, [rip - 0xd90b18]
1412c43b8: mov qword ptr [rbp + 0x18], rax
1412c43bc: mov qword ptr [rbp + 0x20], rsi
1412c43c0: lea rax, [rip - 0x11ba537]
1412c43c7: mov qword ptr [rbp + 0x28], rax
1412c43cb: mov rcx, rbp
1412c43ce: mov qword ptr [rbp + 0x30], rcx
1412c43d2: mov qword ptr [rbp + 0x38], rax
1412c43d6: lea rax, [rip + 0x196918b]
1412c43dd: mov qword ptr [rbp - 0x30], rax
1412c43e1: mov qword ptr [rbp - 0x28], 3
1412c43e9: mov qword ptr [rbp - 0x10], 0
1412c43f1: lea rax, [rbp + 0x10]
1412c43f5: mov qword ptr [rbp - 0x20], rax
1412c43f9: mov qword ptr [rbp - 0x18], 3
1412c4401: lea rdx, [rip + 0x19691c8]
1412c4408: lea rcx, [rbp - 0x30]
1412c440c: call 0x1428d9390
1412c4411: int3 
1412c4412: nop word ptr cs:[rax + rax]
1412c4420: mov qword ptr [rsp + 0x10], rdx
1412c4425: push rbp
1412c4426: push rsi
1412c4427: sub rsp, 0x28
1412c442b: lea rbp, [rdx + 0x80]
1412c4432: lea rcx, [rbp - 0x58]
1412c4436: call 0x140be24b0
1412c443b: nop 
1412c443c: add rsp, 0x28
1412c4440: pop rsi
1412c4441: pop rbp
1412c4442: ret 
1412c4443: nop word ptr cs:[rax + rax]
1412c4450: mov qword ptr [rsp + 0x10], rdx
1412c4455: push rbp
1412c4456: push rsi
1412c4457: sub rsp, 0x28
1412c445b: lea rbp, [rdx + 0x80]
1412c4462: lea rcx, [rbp + 0x10]
1412c4466: call 0x1400c04b0
1412c446b: nop 
1412c446c: add rsp, 0x28
1412c4470: pop rsi
1412c4471: pop rbp
1412c4472: ret 
