1415c3f20: push rbp
1415c3f21: push r15
1415c3f23: push r14
1415c3f25: push r13
1415c3f27: push r12
1415c3f29: push rsi
1415c3f2a: push rdi
1415c3f2b: push rbx
1415c3f2c: sub rsp, 0x188
1415c3f33: lea rbp, [rsp + 0x80]
1415c3f3b: movaps xmmword ptr [rbp + 0xf0], xmm10
1415c3f43: movaps xmmword ptr [rbp + 0xe0], xmm9
1415c3f4b: movaps xmmword ptr [rbp + 0xd0], xmm8
1415c3f53: movaps xmmword ptr [rbp + 0xc0], xmm7
1415c3f5a: movaps xmmword ptr [rbp + 0xb0], xmm6
1415c3f61: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
1415c3f6c: mov rsi, rcx
1415c3f6f: mov rax, qword ptr [rdx + 0x10]
1415c3f73: mov qword ptr [rbp - 0x58], rax
1415c3f77: lea rbx, [rbp - 0x48]
1415c3f7b: mov rcx, rbx
1415c3f7e: call 0x140b880d0
1415c3f83: lea rdi, [rbp + 0x31]
1415c3f87: mov al, 1
1415c3f89: mov dword ptr [rbp + 0x60], eax
1415c3f8c: mov dword ptr [rbp + 0x80], 0
1415c3f96: lea r15, [rbp + 0x88]
1415c3f9d: lea r12, [rbp + 0x30]
1415c3fa1: lea r14, [rip + 0x16b76a0]
1415c3fa8: mov dword ptr [rbp + 0x84], 0
1415c3fb2: mov dword ptr [rbp + 0x7c], 0
1415c3fb9: mov dword ptr [rbp + 0x64], 0
1415c3fc0: mov dword ptr [rbp + 0x78], 0
1415c3fc7: mov dword ptr [rbp + 0x74], 0
1415c3fce: nop 
1415c3fd0: mov rcx, r15
1415c3fd3: mov rdx, rbx
1415c3fd6: call 0x14143c450
1415c3fdb: mov rax, qword ptr [rbp + 0x88]
1415c3fe2: test rax, rax
1415c3fe5: je 0x1415c4386
1415c3feb: mov rcx, qword ptr [rbp + 0x98]
1415c3ff2: lea rdx, [rcx + rcx*2]
1415c3ff6: mov r8, qword ptr [rax + rdx*8 + 0x168]
1415c3ffe: mov r9, qword ptr [rax + rdx*8 + 0x170]
1415c4006: mov qword ptr [rbp + 0x58], r9
1415c400a: mov r13, qword ptr [rax + rdx*8 + 0x178]
1415c4012: shl rcx, 5
1415c4016: movups xmm0, xmmword ptr [rax + rcx]
1415c401a: movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415c401f: movaps xmmword ptr [rbp + 0x30], xmm0
1415c4023: movaps xmmword ptr [rbp + 0x40], xmm1
1415c4027: mov qword ptr [rbp + 0x68], r8
1415c402b: mov rax, r8
1415c402e: neg rax
1415c4031: jo 0x1415c4386
1415c4037: movzx eax, byte ptr [rbp]
1415c403b: cmp rax, 6
1415c403f: ja 0x1415c419b
1415c4045: movsxd rax, dword ptr [r14 + rax*4]
1415c4049: add rax, r14
1415c404c: jmp rax
1415c404e: movaps xmm0, xmmword ptr [rbp + 0x30]
1415c4052: movaps xmm1, xmmword ptr [rbp + 0x40]
1415c4056: lea rax, [rbp]
1415c405a: movups xmmword ptr [rax + 0x10], xmm1
1415c405e: movups xmmword ptr [rax], xmm0
1415c4061: mov rcx, r15
1415c4064: mov rdx, qword ptr [rbp + 0x58]
1415c4068: mov r8, r13
1415c406b: call 0x1416fb440
1415c4070: mov rdx, qword ptr [rbp + 0x68]
1415c4074: test rdx, rdx
1415c4077: je 0x1415c4088
1415c4079: mov r8d, 1
1415c407f: mov rcx, qword ptr [rbp + 0x58]
1415c4083: call 0x140613c20
1415c4088: cmp byte ptr [rbp + 0x88], 0
1415c408f: jne 0x1415c4419
1415c4095: movzx eax, byte ptr [rbp + 0x89]
1415c409c: cmp eax, 6
1415c409f: jne 0x1415c40d9
1415c40a1: movzx eax, byte ptr [rbp]
1415c40a5: mov byte ptr [rbp], 6
1415c40a9: cmp al, 6
1415c40ab: je 0x1415c4435
1415c40b1: mov byte ptr [rbp + 0x30], al
1415c40b4: movups xmm0, xmmword ptr [rbx + 0x49]
1415c40b8: movups xmm1, xmmword ptr [rbx + 0x58]
1415c40bc: movups xmmword ptr [rdi + 0xf], xmm1
1415c40c0: movups xmmword ptr [rdi], xmm0
1415c40c3: mov rcx, r12
1415c40c6: call 0x1415a3940
1415c40cb: test rax, rax
1415c40ce: je 0x1415c3fd0
1415c40d4: jmp 0x1415c447f
1415c40d9: mov eax, eax
1415c40db: lea rcx, [rip + 0x16b7582]
1415c40e2: movsxd rax, dword ptr [rcx + rax*4]
1415c40e6: add rax, rcx
1415c40e9: jmp rax
1415c40eb: cmp dword ptr [rbp + 0x74], 0
1415c40ef: jne 0x1415c45ae
1415c40f5: movzx eax, byte ptr [rbp]
1415c40f9: mov byte ptr [rbp], 6
1415c40fd: cmp al, 6
1415c40ff: je 0x1415c44d8
1415c4105: mov byte ptr [rbp + 0x30], al
1415c4108: movups xmm0, xmmword ptr [rbx + 0x49]
1415c410c: movups xmm1, xmmword ptr [rbx + 0x58]
1415c4110: movups xmmword ptr [rdi + 0xf], xmm1
1415c4114: movups xmmword ptr [rdi], xmm0
1415c4117: mov rcx, r15
1415c411a: mov rdx, r12
1415c411d: call 0x14159a370
1415c4122: test byte ptr [rbp + 0x88], 1
1415c4129: jne 0x1415c4419
1415c412f: movss xmm9, dword ptr [rbp + 0x8c]
1415c4138: mov dword ptr [rbp + 0x74], 1
1415c413f: jmp 0x1415c3fd0
1415c4144: mov rdx, qword ptr [rbp + 8]
1415c4148: test rdx, rdx
1415c414b: je 0x1415c404e
1415c4151: mov rcx, qword ptr [rbp + 0x10]
1415c4155: mov r8d, 1
1415c415b: call 0x140613c20
1415c4160: jmp 0x1415c404e
1415c4165: mov rcx, qword ptr [rbp + 0x10]
1415c4169: mov rdx, qword ptr [rbp + 0x18]
1415c416d: mov qword ptr [rbp + 0x28], rcx
1415c4171: call 0x140888d70
1415c4176: mov rdx, qword ptr [rbp + 8]
1415c417a: test rdx, rdx
1415c417d: je 0x1415c404e
1415c4183: shl rdx, 5
1415c4187: mov r8d, 8
1415c418d: mov rcx, qword ptr [rbp + 0x28]
1415c4191: call 0x140613c20
1415c4196: jmp 0x1415c404e
1415c419b: lea rcx, [rbp + 8]
1415c419f: call 0x140898f40
1415c41a4: jmp 0x1415c404e
1415c41a9: cmp dword ptr [rbp + 0x84], 0
1415c41b0: jne 0x1415c456c
1415c41b6: movzx eax, byte ptr [rbp]
1415c41ba: mov byte ptr [rbp], 6
1415c41be: cmp al, 6
1415c41c0: je 0x1415c44eb
1415c41c6: mov byte ptr [rbp + 0x30], al
1415c41c9: movups xmm0, xmmword ptr [rbx + 0x49]
1415c41cd: movups xmm1, xmmword ptr [rbx + 0x58]
1415c41d1: movups xmmword ptr [rdi + 0xf], xmm1
1415c41d5: movups xmmword ptr [rdi], xmm0
1415c41d8: mov rcx, r15
1415c41db: mov rdx, r12
1415c41de: call 0x14159a370
1415c41e3: test byte ptr [rbp + 0x88], 1
1415c41ea: jne 0x1415c4419
1415c41f0: movss xmm7, dword ptr [rbp + 0x8c]
1415c41f8: mov dword ptr [rbp + 0x84], 1
1415c4202: jmp 0x1415c3fd0
1415c4207: test byte ptr [rbp + 0x60], 1
1415c420b: je 0x1415c4582
1415c4211: movzx eax, byte ptr [rbp]
1415c4215: mov byte ptr [rbp], 6
1415c4219: cmp al, 6
1415c421b: je 0x1415c4514
1415c4221: mov byte ptr [rbp + 0x30], al
1415c4224: movups xmm0, xmmword ptr [rbx + 0x49]
1415c4228: movups xmm1, xmmword ptr [rbx + 0x58]
1415c422c: movups xmmword ptr [rdi + 0xf], xmm1
1415c4230: movups xmmword ptr [rdi], xmm0
1415c4233: mov r8d, 3
1415c4239: mov rcx, r15
1415c423c: mov rdx, r12
1415c423f: call 0x14159da80
1415c4244: test byte ptr [rbp + 0x88], 1
1415c424b: jne 0x1415c4419
1415c4251: mov eax, dword ptr [rbp + 0x8c]
1415c4257: mov dword ptr [rbp + 0x54], eax
1415c425a: mov rax, qword ptr [rbp + 0x90]
1415c4261: mov qword ptr [rbp - 0x50], rax
1415c4265: mov al, 1
1415c4267: mov dword ptr [rbp + 0x64], eax
1415c426a: mov dword ptr [rbp + 0x60], 0
1415c4271: jmp 0x1415c3fd0
1415c4276: cmp dword ptr [rbp + 0x7c], 0
1415c427a: jne 0x1415c45c4
1415c4280: movzx eax, byte ptr [rbp]
1415c4284: mov byte ptr [rbp], 6
1415c4288: cmp al, 6
1415c428a: je 0x1415c44fe
1415c4290: mov byte ptr [rbp + 0x30], al
1415c4293: movups xmm0, xmmword ptr [rbx + 0x49]
1415c4297: movups xmm1, xmmword ptr [rbx + 0x58]
1415c429b: movups xmmword ptr [rdi + 0xf], xmm1
1415c429f: movups xmmword ptr [rdi], xmm0
1415c42a2: mov rcx, r15
1415c42a5: mov rdx, r12
1415c42a8: call 0x14159a370
1415c42ad: test byte ptr [rbp + 0x88], 1
1415c42b4: jne 0x1415c4419
1415c42ba: movss xmm8, dword ptr [rbp + 0x8c]
1415c42c3: mov dword ptr [rbp + 0x7c], 1
1415c42ca: jmp 0x1415c3fd0
1415c42cf: cmp dword ptr [rbp + 0x78], 0
1415c42d3: jne 0x1415c4556
1415c42d9: movzx eax, byte ptr [rbp]
1415c42dd: mov byte ptr [rbp], 6
1415c42e1: cmp al, 6
1415c42e3: je 0x1415c452a
1415c42e9: mov byte ptr [rbp + 0x30], al
1415c42ec: movups xmm0, xmmword ptr [rbx + 0x49]
1415c42f0: movups xmm1, xmmword ptr [rbx + 0x58]
1415c42f4: movups xmmword ptr [rdi + 0xf], xmm1
1415c42f8: movups xmmword ptr [rdi], xmm0
1415c42fb: mov rcx, r15
1415c42fe: mov rdx, r12
1415c4301: call 0x14159a370
1415c4306: test byte ptr [rbp + 0x88], 1
1415c430d: jne 0x1415c4419
1415c4313: movss xmm10, dword ptr [rbp + 0x8c]
1415c431c: mov dword ptr [rbp + 0x78], 1
1415c4323: jmp 0x1415c3fd0
1415c4328: cmp dword ptr [rbp + 0x80], 0
1415c432f: jne 0x1415c4598
1415c4335: movzx eax, byte ptr [rbp]
1415c4339: mov byte ptr [rbp], 6
1415c433d: cmp al, 6
1415c433f: je 0x1415c4540
1415c4345: mov byte ptr [rbp + 0x30], al
1415c4348: movups xmm0, xmmword ptr [rbx + 0x49]
1415c434c: movups xmm1, xmmword ptr [rbx + 0x58]
1415c4350: movups xmmword ptr [rdi + 0xf], xmm1
1415c4354: movups xmmword ptr [rdi], xmm0
1415c4357: mov rcx, r15
1415c435a: mov rdx, r12
1415c435d: call 0x14159a370
1415c4362: test byte ptr [rbp + 0x88], 1
1415c4369: jne 0x1415c4419
1415c436f: movss xmm6, dword ptr [rbp + 0x8c]
1415c4377: mov dword ptr [rbp + 0x80], 1
1415c4381: jmp 0x1415c3fd0
1415c4386: test byte ptr [rbp + 0x74], 1
1415c438a: je 0x1415c4422
1415c4390: test byte ptr [rbp + 0x78], 1
1415c4394: je 0x1415c4448
1415c439a: test byte ptr [rbp + 0x64], 1
1415c439e: je 0x1415c445b
1415c43a4: test byte ptr [rbp + 0x7c], 1
1415c43a8: je 0x1415c446e
1415c43ae: movss dword ptr [rbp + 0x20], xmm10
1415c43b4: mov eax, dword ptr [rbp + 0x54]
1415c43b7: mov dword ptr [rbp + 0x24], eax
1415c43ba: cmp qword ptr [rbp - 8], 0
1415c43bf: jne 0x1415c45da
1415c43c5: test byte ptr [rbp + 0x80], 1
1415c43cc: jne 0x1415c43d1
1415c43ce: xorps xmm6, xmm6
1415c43d1: mov rax, qword ptr [rbp + 0x20]
1415c43d5: test byte ptr [rbp + 0x84], 1
1415c43dc: jne 0x1415c43e1
1415c43de: xorps xmm7, xmm7
1415c43e1: movss dword ptr [rsi + 4], xmm9
1415c43e7: mov qword ptr [rsi + 8], rax
1415c43eb: mov rax, qword ptr [rbp - 0x50]
1415c43ef: mov qword ptr [rsi + 0x10], rax
1415c43f3: movss dword ptr [rsi + 0x18], xmm8
1415c43f9: mov dword ptr [rsi + 0x1c], 0x42340000
1415c4400: movss dword ptr [rsi + 0x20], xmm7
1415c4405: movss dword ptr [rsi + 0x24], xmm6
1415c440a: xor eax, eax
1415c440c: mov dword ptr [rsi], eax
1415c440e: lea rcx, [rbp - 0x48]
1415c4412: call 0x140887df0
1415c4417: jmp 0x1415c4492
1415c4419: mov rax, qword ptr [rbp + 0x90]
1415c4420: jmp 0x1415c447f
1415c4422: lea rcx, [rip + 0x16b61ae]
1415c4429: mov edx, 3
1415c442e: call 0x1428f9bc0
1415c4433: jmp 0x1415c447f
1415c4435: lea rcx, [rip + 0x16b5e2c]
1415c443c: mov edx, 0x10
1415c4441: call 0x1428fa000
1415c4446: jmp 0x1415c447f
1415c4448: lea rcx, [rip + 0x16b6df1]
1415c444f: mov edx, 5
1415c4454: call 0x1428f9bc0
1415c4459: jmp 0x1415c447f
1415c445b: lea rcx, [rip + 0x16b5ece]
1415c4462: mov edx, 8
1415c4467: call 0x1428f9bc0
1415c446c: jmp 0x1415c447f
1415c446e: lea rcx, [rip + 0x16b6dd0]
1415c4475: mov edx, 0x15
1415c447a: call 0x1428f9bc0
1415c447f: mov qword ptr [rsi + 8], rax
1415c4483: mov dword ptr [rsi], 1
1415c4489: lea rcx, [rbp - 0x48]
1415c448d: call 0x140887df0
1415c4492: lea rcx, [rbp]
1415c4496: call 0x14088e430
1415c449b: mov rax, rsi
1415c449e: movaps xmm6, xmmword ptr [rbp + 0xb0]
1415c44a5: movaps xmm7, xmmword ptr [rbp + 0xc0]
1415c44ac: movaps xmm8, xmmword ptr [rbp + 0xd0]
1415c44b4: movaps xmm9, xmmword ptr [rbp + 0xe0]
1415c44bc: movaps xmm10, xmmword ptr [rbp + 0xf0]
1415c44c4: add rsp, 0x188
1415c44cb: pop rbx
1415c44cc: pop rdi
1415c44cd: pop rsi
1415c44ce: pop r12
1415c44d0: pop r13
1415c44d2: pop r14
1415c44d4: pop r15
1415c44d6: pop rbp
1415c44d7: ret 
1415c44d8: lea rcx, [rip + 0x16b5d89]
1415c44df: mov edx, 0x10
1415c44e4: call 0x1428fa000
1415c44e9: jmp 0x1415c447f
1415c44eb: lea rcx, [rip + 0x16b5d76]
1415c44f2: mov edx, 0x10
1415c44f7: call 0x1428fa000
1415c44fc: jmp 0x1415c447f
1415c44fe: lea rcx, [rip + 0x16b5d63]
1415c4505: mov edx, 0x10
1415c450a: call 0x1428fa000
1415c450f: jmp 0x1415c447f
1415c4514: lea rcx, [rip + 0x16b5d4d]
1415c451b: mov edx, 0x10
1415c4520: call 0x1428fa000
1415c4525: jmp 0x1415c447f
1415c452a: lea rcx, [rip + 0x16b5d37]
1415c4531: mov edx, 0x10
1415c4536: call 0x1428fa000
1415c453b: jmp 0x1415c447f
1415c4540: lea rcx, [rip + 0x16b5d21]
1415c4547: mov edx, 0x10
1415c454c: call 0x1428fa000
1415c4551: jmp 0x1415c447f
1415c4556: lea rcx, [rip + 0x16b6ce3]
1415c455d: mov edx, 5
1415c4562: call 0x1428f9db0
1415c4567: jmp 0x1415c447f
1415c456c: lea rcx, [rip + 0x16b6ce7]
1415c4573: mov edx, 0xc
1415c4578: call 0x1428f9db0
1415c457d: jmp 0x1415c447f
1415c4582: lea rcx, [rip + 0x16b5da7]
1415c4589: mov edx, 8
1415c458e: call 0x1428f9db0
1415c4593: jmp 0x1415c447f
1415c4598: lea rcx, [rip + 0x16b6cc7]
1415c459f: mov edx, 0xa
1415c45a4: call 0x1428f9db0
1415c45a9: jmp 0x1415c447f
1415c45ae: lea rcx, [rip + 0x16b6022]
1415c45b5: mov edx, 3
1415c45ba: call 0x1428f9db0
1415c45bf: jmp 0x1415c447f
1415c45c4: lea rcx, [rip + 0x16b6c7a]
1415c45cb: mov edx, 0x15
1415c45d0: call 0x1428f9db0
1415c45d5: jmp 0x1415c447f
1415c45da: lea rdx, [rip + 0x16b5ab7]
1415c45e1: lea r8, [rip + 0x16b5a78]
1415c45e8: mov rcx, qword ptr [rbp - 0x58]
1415c45ec: call 0x1428f9d20
1415c45f1: mov qword ptr [rsi + 8], rax
1415c45f5: mov eax, 1
1415c45fa: jmp 0x1415c440c
1415c45ff: nop 
1415c4600: mov qword ptr [rsp + 0x10], rdx
1415c4605: push rbp
1415c4606: push r15
1415c4608: push r14
1415c460a: push r13
1415c460c: push r12
1415c460e: push rsi
1415c460f: push rdi
1415c4610: push rbx
1415c4611: sub rsp, 0x78
1415c4615: lea rbp, [rdx + 0x80]
1415c461c: movaps xmmword ptr [rsp + 0x20], xmm10
1415c4622: movaps xmmword ptr [rsp + 0x30], xmm9
1415c4628: movaps xmmword ptr [rsp + 0x40], xmm8
1415c462e: movaps xmmword ptr [rsp + 0x50], xmm7
1415c4633: movaps xmmword ptr [rsp + 0x60], xmm6
1415c4638: mov rdx, qword ptr [rbp + 8]
1415c463c: test rdx, rdx
1415c463f: je 0x1415c4654
1415c4641: shl rdx, 5
1415c4645: mov r8d, 8
1415c464b: mov rcx, qword ptr [rbp + 0x28]
1415c464f: call 0x140613c20
1415c4654: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c4659: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c465e: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c4664: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c466a: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c4670: add rsp, 0x78
1415c4674: pop rbx
1415c4675: pop rdi
1415c4676: pop rsi
1415c4677: pop r12
1415c4679: pop r13
1415c467b: pop r14
1415c467d: pop r15
1415c467f: pop rbp
1415c4680: ret 
1415c4681: nop word ptr cs:[rax + rax]
1415c4690: mov qword ptr [rsp + 0x10], rdx
1415c4695: push rbp
1415c4696: push r15
1415c4698: push r14
1415c469a: push r13
1415c469c: push r12
1415c469e: push rsi
1415c469f: push rdi
1415c46a0: push rbx
1415c46a1: sub rsp, 0x78
1415c46a5: lea rbp, [rdx + 0x80]
1415c46ac: movaps xmmword ptr [rsp + 0x20], xmm10
1415c46b2: movaps xmmword ptr [rsp + 0x30], xmm9
1415c46b8: movaps xmmword ptr [rsp + 0x40], xmm8
1415c46be: movaps xmmword ptr [rsp + 0x50], xmm7
1415c46c3: movaps xmmword ptr [rsp + 0x60], xmm6
1415c46c8: movaps xmm0, xmmword ptr [rbp + 0x30]
1415c46cc: movaps xmm1, xmmword ptr [rbp + 0x40]
1415c46d0: lea rax, [rbp]
1415c46d4: movups xmmword ptr [rax + 0x10], xmm1
1415c46d8: movups xmmword ptr [rax], xmm0
1415c46db: mov byte ptr [rbp + 0xa7], 1
1415c46e2: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c46e7: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c46ec: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c46f2: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c46f8: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c46fe: add rsp, 0x78
1415c4702: pop rbx
1415c4703: pop rdi
1415c4704: pop rsi
1415c4705: pop r12
1415c4707: pop r13
1415c4709: pop r14
1415c470b: pop r15
1415c470d: pop rbp
1415c470e: ret 
1415c470f: nop 
1415c4710: mov qword ptr [rsp + 0x10], rdx
1415c4715: push rbp
1415c4716: push r15
1415c4718: push r14
1415c471a: push r13
1415c471c: push r12
1415c471e: push rsi
1415c471f: push rdi
1415c4720: push rbx
1415c4721: sub rsp, 0x78
1415c4725: lea rbp, [rdx + 0x80]
1415c472c: movaps xmmword ptr [rsp + 0x20], xmm10
1415c4732: movaps xmmword ptr [rsp + 0x30], xmm9
1415c4738: movaps xmmword ptr [rsp + 0x40], xmm8
1415c473e: movaps xmmword ptr [rsp + 0x50], xmm7
1415c4743: movaps xmmword ptr [rsp + 0x60], xmm6
1415c4748: cmp qword ptr [rbp + 0x68], 0
1415c474d: je 0x1415c4762
1415c474f: mov r8d, 1
1415c4755: mov rcx, qword ptr [rbp + 0x58]
1415c4759: mov rdx, qword ptr [rbp + 0x68]
1415c475d: call 0x140613c20
1415c4762: mov byte ptr [rbp + 0xa7], 0
1415c4769: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c476e: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c4773: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c4779: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c477f: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c4785: add rsp, 0x78
1415c4789: pop rbx
1415c478a: pop rdi
1415c478b: pop rsi
1415c478c: pop r12
1415c478e: pop r13
1415c4790: pop r14
1415c4792: pop r15
1415c4794: pop rbp
1415c4795: ret 
1415c4796: nop word ptr cs:[rax + rax]
1415c47a0: mov qword ptr [rsp + 0x10], rdx
1415c47a5: push rbp
1415c47a6: push r15
1415c47a8: push r14
1415c47aa: push r13
1415c47ac: push r12
1415c47ae: push rsi
1415c47af: push rdi
1415c47b0: push rbx
1415c47b1: sub rsp, 0x78
1415c47b5: lea rbp, [rdx + 0x80]
1415c47bc: movaps xmmword ptr [rsp + 0x20], xmm10
1415c47c2: movaps xmmword ptr [rsp + 0x30], xmm9
1415c47c8: movaps xmmword ptr [rsp + 0x40], xmm8
1415c47ce: movaps xmmword ptr [rsp + 0x50], xmm7
1415c47d3: movaps xmmword ptr [rsp + 0x60], xmm6
1415c47d8: cmp byte ptr [rbp + 0xa7], 0
1415c47df: je 0x1415c4828
1415c47e1: cmp qword ptr [rbp + 0x68], 0
1415c47e6: je 0x1415c47fb
1415c47e8: mov r8d, 1
1415c47ee: mov rcx, qword ptr [rbp + 0x58]
1415c47f2: mov rdx, qword ptr [rbp + 0x68]
1415c47f6: call 0x140613c20
1415c47fb: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c4800: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c4805: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c480b: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c4811: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c4817: add rsp, 0x78
1415c481b: pop rbx
1415c481c: pop rdi
1415c481d: pop rsi
1415c481e: pop r12
1415c4820: pop r13
1415c4822: pop r14
1415c4824: pop r15
1415c4826: pop rbp
1415c4827: ret 
1415c4828: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c482d: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c4832: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c4838: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c483e: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c4844: add rsp, 0x78
1415c4848: pop rbx
1415c4849: pop rdi
1415c484a: pop rsi
1415c484b: pop r12
1415c484d: pop r13
1415c484f: pop r14
1415c4851: pop r15
1415c4853: pop rbp
1415c4854: ret 
1415c4855: nop word ptr cs:[rax + rax]
1415c4860: mov qword ptr [rsp + 0x10], rdx
1415c4865: push rbp
1415c4866: push r15
1415c4868: push r14
1415c486a: push r13
1415c486c: push r12
1415c486e: push rsi
1415c486f: push rdi
1415c4870: push rbx
1415c4871: sub rsp, 0x78
1415c4875: lea rbp, [rdx + 0x80]
1415c487c: movaps xmmword ptr [rsp + 0x20], xmm10
1415c4882: movaps xmmword ptr [rsp + 0x30], xmm9
1415c4888: movaps xmmword ptr [rsp + 0x40], xmm8
1415c488e: movaps xmmword ptr [rsp + 0x50], xmm7
1415c4893: movaps xmmword ptr [rsp + 0x60], xmm6
1415c4898: lea rcx, [rbp - 0x48]
1415c489c: call 0x14088a500
1415c48a1: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c48a6: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c48ab: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c48b1: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c48b7: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c48bd: add rsp, 0x78
1415c48c1: pop rbx
1415c48c2: pop rdi
1415c48c3: pop rsi
1415c48c4: pop r12
1415c48c6: pop r13
1415c48c8: pop r14
1415c48ca: pop r15
1415c48cc: pop rbp
1415c48cd: ret 
1415c48ce: nop 
1415c48d0: mov qword ptr [rsp + 0x10], rdx
1415c48d5: push rbp
1415c48d6: push r15
1415c48d8: push r14
1415c48da: push r13
1415c48dc: push r12
1415c48de: push rsi
1415c48df: push rdi
1415c48e0: push rbx
1415c48e1: sub rsp, 0x78
1415c48e5: lea rbp, [rdx + 0x80]
1415c48ec: movaps xmmword ptr [rsp + 0x20], xmm10
1415c48f2: movaps xmmword ptr [rsp + 0x30], xmm9
1415c48f8: movaps xmmword ptr [rsp + 0x40], xmm8
1415c48fe: movaps xmmword ptr [rsp + 0x50], xmm7
1415c4903: movaps xmmword ptr [rsp + 0x60], xmm6
1415c4908: lea rcx, [rbp]
1415c490c: call 0x14088e430
1415c4911: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c4916: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c491b: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c4921: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c4927: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c492d: add rsp, 0x78
1415c4931: pop rbx
1415c4932: pop rdi
1415c4933: pop rsi
1415c4934: pop r12
1415c4936: pop r13
1415c4938: pop r14
1415c493a: pop r15
1415c493c: pop rbp
1415c493d: ret 
1415c493e: nop 
1415c4940: mov qword ptr [rsp + 0x10], rdx
1415c4945: push rbp
1415c4946: push r15
1415c4948: push r14
1415c494a: push r13
1415c494c: push r12
1415c494e: push rsi
1415c494f: push rdi
1415c4950: push rbx
1415c4951: sub rsp, 0x78
1415c4955: lea rbp, [rdx + 0x80]
1415c495c: movaps xmmword ptr [rsp + 0x20], xmm10
1415c4962: movaps xmmword ptr [rsp + 0x30], xmm9
1415c4968: movaps xmmword ptr [rsp + 0x40], xmm8
1415c496e: movaps xmmword ptr [rsp + 0x50], xmm7
1415c4973: movaps xmmword ptr [rsp + 0x60], xmm6
1415c4978: lea rcx, [rbp]
1415c497c: call 0x14088e430
1415c4981: movaps xmm6, xmmword ptr [rsp + 0x60]
1415c4986: movaps xmm7, xmmword ptr [rsp + 0x50]
1415c498b: movaps xmm8, xmmword ptr [rsp + 0x40]
1415c4991: movaps xmm9, xmmword ptr [rsp + 0x30]
1415c4997: movaps xmm10, xmmword ptr [rsp + 0x20]
1415c499d: add rsp, 0x78
1415c49a1: pop rbx
1415c49a2: pop rdi
1415c49a3: pop rsi
1415c49a4: pop r12
1415c49a6: pop r13
1415c49a8: pop r14
1415c49aa: pop r15
1415c49ac: pop rbp
1415c49ad: ret 
