1404f41b0: push r14
1404f41b2: push rsi
1404f41b3: push rdi
1404f41b4: push rbx
1404f41b5: sub rsp, 0x38
1404f41b9: mov rsi, rdx
1404f41bc: mov rdi, rcx
1404f41bf: movabs rcx, 0x8000000000000000
1404f41c9: xor rcx, qword ptr [rdi]
1404f41cc: cmp rcx, 2
1404f41d0: mov eax, 2
1404f41d5: cmovb rax, rcx
1404f41d9: add rdx, 8
1404f41dd: test rax, rax
1404f41e0: je 0x1404f4320
1404f41e6: cmp rax, 1
1404f41ea: jne 0x1404f4419
1404f41f0: mov rbx, qword ptr [rsi]
1404f41f3: mov rax, qword ptr [rbx + 0x10]
1404f41f7: cmp qword ptr [rbx], rax
1404f41fa: je 0x1404f4565
1404f4200: mov rcx, qword ptr [rbx + 8]
1404f4204: mov byte ptr [rcx + rax], 0x7b
1404f4208: inc rax
1404f420b: mov qword ptr [rbx + 0x10], rax
1404f420f: lea r8, [rip + 0x24f8427]
1404f4216: mov r9d, 0x11
1404f421c: mov rcx, rsi
1404f421f: call 0x1404b6100
1404f4224: test rax, rax
1404f4227: jne 0x1404f4554
1404f422d: mov rbx, qword ptr [rsi]
1404f4230: mov rdx, qword ptr [rbx + 0x10]
1404f4234: cmp qword ptr [rbx], rdx
1404f4237: je 0x1404f45f2
1404f423d: lea r9, [rdi + 8]
1404f4241: mov rax, qword ptr [rbx + 8]
1404f4245: mov byte ptr [rax + rdx], 0x3a
1404f4249: inc rdx
1404f424c: mov qword ptr [rbx + 0x10], rdx
1404f4250: mov rbx, qword ptr [rsi]
1404f4253: mov rdx, qword ptr [rbx + 0x10]
1404f4257: cmp qword ptr [rbx], rdx
1404f425a: je 0x1404f4618
1404f4260: mov rax, qword ptr [rbx + 8]
1404f4264: mov byte ptr [rax + rdx], 0x7b
1404f4268: inc rdx
1404f426b: mov qword ptr [rbx + 0x10], rdx
1404f426f: mov qword ptr [rsp + 0x28], rsi
1404f4274: mov byte ptr [rsp + 0x30], 1
1404f4279: lea rdx, [rip + 0x24f83df]
1404f4280: lea rcx, [rsp + 0x28]
1404f4285: mov r8d, 5
1404f428b: call 0x1404baab0
1404f4290: test rax, rax
1404f4293: jne 0x1404f454a
1404f4299: lea r9, [rdi + 0xc]
1404f429d: lea rdx, [rip + 0x24f83d3]
1404f42a4: lea rcx, [rsp + 0x28]
1404f42a9: mov r8d, 0xe
1404f42af: call 0x1404bdc80
1404f42b4: test rax, rax
1404f42b7: jne 0x1404f454a
1404f42bd: lea r9, [rdi + 0x10]
1404f42c1: lea rdx, [rip + 0x24f83bd]
1404f42c8: lea rcx, [rsp + 0x28]
1404f42cd: mov r8d, 0xd
1404f42d3: call 0x1404bdc80
1404f42d8: test rax, rax
1404f42db: jne 0x1404f454a
1404f42e1: lea r9, [rdi + 0x14]
1404f42e5: lea rdx, [rip + 0x24f83a6]
1404f42ec: lea rcx, [rsp + 0x28]
1404f42f1: mov r8d, 0xc
1404f42f7: call 0x1404bdc80
1404f42fc: test rax, rax
1404f42ff: jne 0x1404f454a
1404f4305: add rdi, 0x18
1404f4309: lea rdx, [rip + 0x24f838e]
1404f4310: lea rcx, [rsp + 0x28]
1404f4315: mov r8d, 0xb
1404f431b: jmp 0x1404f4403
1404f4320: mov rbx, qword ptr [rsi]
1404f4323: mov rax, qword ptr [rbx + 0x10]
1404f4327: cmp qword ptr [rbx], rax
1404f432a: je 0x1404f4594
1404f4330: mov rcx, qword ptr [rbx + 8]
1404f4334: mov byte ptr [rcx + rax], 0x7b
1404f4338: inc rax
1404f433b: mov qword ptr [rbx + 0x10], rax
1404f433f: lea r8, [rip + 0x24f82e7]
1404f4346: mov r9d, 0x10
1404f434c: mov rcx, rsi
1404f434f: call 0x1404b6100
1404f4354: test rax, rax
1404f4357: jne 0x1404f4554
1404f435d: mov rbx, qword ptr [rsi]
1404f4360: mov rdx, qword ptr [rbx + 0x10]
1404f4364: cmp qword ptr [rbx], rdx
1404f4367: je 0x1404f4644
1404f436d: mov rax, qword ptr [rbx + 8]
1404f4371: mov byte ptr [rax + rdx], 0x3a
1404f4375: inc rdx
1404f4378: mov qword ptr [rbx + 0x10], rdx
1404f437c: mov rbx, qword ptr [rsi]
1404f437f: mov rdx, qword ptr [rbx + 0x10]
1404f4383: cmp qword ptr [rbx], rdx
1404f4386: je 0x1404f466a
1404f438c: mov rax, qword ptr [rbx + 8]
1404f4390: mov byte ptr [rax + rdx], 0x7b
1404f4394: inc rdx
1404f4397: mov qword ptr [rbx + 0x10], rdx
1404f439b: mov qword ptr [rsp + 0x28], rsi
1404f43a0: mov byte ptr [rsp + 0x30], 1
1404f43a5: lea r9, [rdi + 0x20]
1404f43a9: lea rdx, [rip + 0x24f82af]
1404f43b0: lea rcx, [rsp + 0x28]
1404f43b5: mov r8d, 5
1404f43bb: call 0x1404baab0
1404f43c0: test rax, rax
1404f43c3: jne 0x1404f454a
1404f43c9: lea r9, [rdi + 8]
1404f43cd: lea rdx, [rip + 0x24f8290]
1404f43d4: lea rcx, [rsp + 0x28]
1404f43d9: mov r8d, 0xa
1404f43df: call 0x1404bbdb0
1404f43e4: test rax, rax
1404f43e7: jne 0x1404f454a
1404f43ed: add rdi, 0x24
1404f43f1: lea rdx, [rip + 0x24f8276]
1404f43f8: lea rcx, [rsp + 0x28]
1404f43fd: mov r8d, 9
1404f4403: mov r9, rdi
1404f4406: call 0x1404bdc80
1404f440b: test rax, rax
1404f440e: jne 0x1404f454a
1404f4414: jmp 0x1404f4504
1404f4419: mov rbx, qword ptr [rsi]
1404f441c: mov rax, qword ptr [rbx + 0x10]
1404f4420: cmp qword ptr [rbx], rax
1404f4423: je 0x1404f45c3
1404f4429: mov rcx, qword ptr [rbx + 8]
1404f442d: mov byte ptr [rcx + rax], 0x7b
1404f4431: inc rax
1404f4434: mov qword ptr [rbx + 0x10], rax
1404f4438: lea r8, [rip + 0x24f820f]
1404f443f: mov r9d, 0x11
1404f4445: mov rcx, rsi
1404f4448: call 0x1404b6100
1404f444d: test rax, rax
1404f4450: jne 0x1404f4554
1404f4456: mov rbx, qword ptr [rsi]
1404f4459: mov rdx, qword ptr [rbx + 0x10]
1404f445d: cmp qword ptr [rbx], rdx
1404f4460: je 0x1404f4690
1404f4466: mov rax, qword ptr [rbx + 8]
1404f446a: mov byte ptr [rax + rdx], 0x3a
1404f446e: inc rdx
1404f4471: mov qword ptr [rbx + 0x10], rdx
1404f4475: mov rbx, qword ptr [rsi]
1404f4478: mov rdx, qword ptr [rbx + 0x10]
1404f447c: cmp qword ptr [rbx], rdx
1404f447f: je 0x1404f46b6
1404f4485: mov rax, qword ptr [rbx + 8]
1404f4489: mov byte ptr [rax + rdx], 0x7b
1404f448d: inc rdx
1404f4490: mov qword ptr [rbx + 0x10], rdx
1404f4494: mov qword ptr [rsp + 0x28], rsi
1404f4499: mov byte ptr [rsp + 0x30], 1
1404f449e: lea r9, [rdi + 0x20]
1404f44a2: lea rdx, [rip + 0x24f8200]
1404f44a9: lea rcx, [rsp + 0x28]
1404f44ae: mov r8d, 2
1404f44b4: call 0x1404baab0
1404f44b9: test rax, rax
1404f44bc: jne 0x1404f454a
1404f44c2: lea rdx, [rip + 0x24f81e2]
1404f44c9: lea rcx, [rsp + 0x28]
1404f44ce: mov r8d, 0x13
1404f44d4: mov r9, rdi
1404f44d7: call 0x1404bbdb0
1404f44dc: test rax, rax
1404f44df: jne 0x1404f454a
1404f44e1: add rdi, 0x18
1404f44e5: lea rdx, [rip + 0x24f81d2]
1404f44ec: lea rcx, [rsp + 0x28]
1404f44f1: mov r8d, 8
1404f44f7: mov r9, rdi
1404f44fa: call 0x1404bd170
1404f44ff: test rax, rax
1404f4502: jne 0x1404f454a
1404f4504: cmp byte ptr [rsp + 0x30], 0
1404f4509: je 0x1404f452c
1404f450b: mov rax, qword ptr [rsp + 0x28]
1404f4510: mov rcx, qword ptr [rax]
1404f4513: lea rdx, [rip + 0x24f7dce]
1404f451a: lea r9, [rip + 0x24f83c7]
1404f4521: mov r8d, 1
1404f4527: call 0x1402b5870
1404f452c: mov rcx, qword ptr [rsi]
1404f452f: lea rdx, [rip + 0x24f7db2]
1404f4536: lea r9, [rip + 0x24f83ab]
1404f453d: mov r8d, 1
1404f4543: call 0x1402b5870
1404f4548: xor eax, eax
1404f454a: add rsp, 0x38
1404f454e: pop rbx
1404f454f: pop rdi
1404f4550: pop rsi
1404f4551: pop r14
1404f4553: ret 
1404f4554: mov rcx, rax
1404f4557: add rsp, 0x38
1404f455b: pop rbx
1404f455c: pop rdi
1404f455d: pop rsi
1404f455e: pop r14
1404f4560: jmp 0x1428c98a0
1404f4565: mov qword ptr [rsp + 0x20], 1
1404f456e: mov r8d, 1
1404f4574: mov r9d, 1
1404f457a: mov rcx, rbx
1404f457d: mov r14, rdx
1404f4580: mov rdx, rax
1404f4583: call 0x1428a29e0
1404f4588: mov rdx, r14
1404f458b: mov rax, qword ptr [rbx + 0x10]
1404f458f: jmp 0x1404f4200
1404f4594: mov qword ptr [rsp + 0x20], 1
1404f459d: mov r8d, 1
1404f45a3: mov r9d, 1
1404f45a9: mov rcx, rbx
1404f45ac: mov r14, rdx
1404f45af: mov rdx, rax
1404f45b2: call 0x1428a29e0
1404f45b7: mov rdx, r14
1404f45ba: mov rax, qword ptr [rbx + 0x10]
1404f45be: jmp 0x1404f4330
1404f45c3: mov qword ptr [rsp + 0x20], 1
1404f45cc: mov r8d, 1
1404f45d2: mov r9d, 1
1404f45d8: mov rcx, rbx
1404f45db: mov r14, rdx
1404f45de: mov rdx, rax
1404f45e1: call 0x1428a29e0
1404f45e6: mov rdx, r14
1404f45e9: mov rax, qword ptr [rbx + 0x10]
1404f45ed: jmp 0x1404f4429
1404f45f2: mov qword ptr [rsp + 0x20], 1
1404f45fb: mov r8d, 1
1404f4601: mov r9d, 1
1404f4607: mov rcx, rbx
1404f460a: call 0x1428a29e0
1404f460f: mov rdx, qword ptr [rbx + 0x10]
1404f4613: jmp 0x1404f423d
1404f4618: mov qword ptr [rsp + 0x20], 1
1404f4621: mov r8d, 1
1404f4627: mov r14, r9
1404f462a: mov r9d, 1
1404f4630: mov rcx, rbx
1404f4633: call 0x1428a29e0
1404f4638: mov r9, r14
1404f463b: mov rdx, qword ptr [rbx + 0x10]
1404f463f: jmp 0x1404f4260
1404f4644: mov qword ptr [rsp + 0x20], 1
1404f464d: mov r8d, 1
1404f4653: mov r9d, 1
1404f4659: mov rcx, rbx
1404f465c: call 0x1428a29e0
1404f4661: mov rdx, qword ptr [rbx + 0x10]
1404f4665: jmp 0x1404f436d
1404f466a: mov qword ptr [rsp + 0x20], 1
1404f4673: mov r8d, 1
1404f4679: mov r9d, 1
1404f467f: mov rcx, rbx
1404f4682: call 0x1428a29e0
1404f4687: mov rdx, qword ptr [rbx + 0x10]
1404f468b: jmp 0x1404f438c
1404f4690: mov qword ptr [rsp + 0x20], 1
1404f4699: mov r8d, 1
1404f469f: mov r9d, 1
1404f46a5: mov rcx, rbx
1404f46a8: call 0x1428a29e0
1404f46ad: mov rdx, qword ptr [rbx + 0x10]
1404f46b1: jmp 0x1404f4466
1404f46b6: mov qword ptr [rsp + 0x20], 1
1404f46bf: mov r8d, 1
1404f46c5: mov r9d, 1
1404f46cb: mov rcx, rbx
1404f46ce: call 0x1428a29e0
1404f46d3: mov rdx, qword ptr [rbx + 0x10]
1404f46d7: jmp 0x1404f4485
