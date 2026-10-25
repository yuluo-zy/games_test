1412c05d0: push rbp
1412c05d1: push rsi
1412c05d2: sub rsp, 0xe8
1412c05d9: lea rbp, [rsp + 0x80]
1412c05e1: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412c05e9: mov rsi, rcx
1412c05ec: lea rcx, [rbp - 0x58]
1412c05f0: xor r8d, r8d
1412c05f3: call 0x14161e5a0
1412c05f8: movzx eax, byte ptr [rip + 0x237460a]
1412c05ff: mov ecx, 0x38
1412c0604: mov edx, 8
1412c0609: call 0x140613c10
1412c060e: test rax, rax
1412c0611: je 0x1412c06cf
1412c0617: mov rcx, qword ptr [rbp - 0x28]
1412c061b: mov qword ptr [rax + 0x30], rcx
1412c061f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c0623: movups xmm1, xmmword ptr [rbp - 0x48]
1412c0627: movups xmm2, xmmword ptr [rbp - 0x38]
1412c062b: movups xmmword ptr [rax + 0x20], xmm2
1412c062f: movups xmmword ptr [rax + 0x10], xmm1
1412c0633: movups xmmword ptr [rax], xmm0
1412c0636: lea rcx, [rip + 0x17dc904]
1412c063d: mov qword ptr [rbp + 0x10], rcx
1412c0641: mov qword ptr [rbp + 0x18], 0x34
1412c0649: mov qword ptr [rbp + 0x20], rax
1412c064d: lea rax, [rip + 0x196c514]
1412c0654: mov qword ptr [rbp + 0x28], rax
1412c0658: mov qword ptr [rbp + 0x40], 0
1412c0660: mov byte ptr [rbp + 0x56], 5
1412c0664: mov qword ptr [rbp + 0x30], rcx
1412c0668: mov qword ptr [rbp + 0x38], 0x34
1412c0670: movzx eax, byte ptr [rip + 0x2374592]
1412c0677: mov ecx, 0x40
1412c067c: mov edx, 8
1412c0681: call 0x140613c10
1412c0686: test rax, rax
1412c0689: je 0x1412c06e0
1412c068b: movups xmm0, xmmword ptr [rbp + 0x20]
1412c068f: movups xmm1, xmmword ptr [rbp + 0x30]
1412c0693: movups xmm2, xmmword ptr [rbp + 0x40]
1412c0697: movups xmm3, xmmword ptr [rbp + 0x50]
1412c069b: movups xmmword ptr [rax + 0x30], xmm3
1412c069f: movups xmmword ptr [rax + 0x20], xmm2
1412c06a3: movups xmmword ptr [rax + 0x10], xmm1
1412c06a7: movups xmmword ptr [rax], xmm0
1412c06aa: movabs rdx, 0x584226dee325ca2c
1412c06b4: mov rcx, rsi
1412c06b7: mov r8, rax
1412c06ba: call 0x140969bf0
1412c06bf: cmp rax, 1
1412c06c3: je 0x1412c06f1
1412c06c5: add rsp, 0xe8
1412c06cc: pop rsi
1412c06cd: pop rbp
1412c06ce: ret 
1412c06cf: mov ecx, 8
1412c06d4: mov edx, 0x38
1412c06d9: call 0x1428d9000
1412c06de: jmp 0x1412c06ef
1412c06e0: mov ecx, 8
1412c06e5: mov edx, 0x40
1412c06ea: call 0x1428d9000
1412c06ef: ud2 
1412c06f1: mov rax, rdx
1412c06f4: add rdx, 0x10
1412c06f8: mov rsi, rdx
1412c06fb: cmp qword ptr [rax + 0x18], 0x34
1412c0700: jne 0x1412c077f
1412c0702: mov rdx, qword ptr [rsi]
1412c0705: lea rcx, [rip + 0x17dc835]
1412c070c: mov r8d, 0x34
1412c0712: call 0x142923bf0
1412c0717: test eax, eax
1412c0719: jne 0x1412c077f
1412c071b: lea rax, [rip + 0x196da26]
1412c0722: mov qword ptr [rbp - 0x20], rax
1412c0726: lea rax, [rip - 0xd8ce8d]
1412c072d: mov qword ptr [rbp - 0x18], rax
1412c0731: lea rax, [rbp + 0x10]
1412c0735: mov qword ptr [rbp - 0x10], rax
1412c0739: lea rax, [rip - 0x11b68b0]
1412c0740: mov qword ptr [rbp - 8], rax
1412c0744: lea rax, [rip + 0x196cedd]
1412c074b: mov qword ptr [rbp + 0x20], rax
1412c074f: mov qword ptr [rbp + 0x28], 3
1412c0757: mov qword ptr [rbp + 0x40], 0
1412c075f: lea rax, [rbp - 0x20]
1412c0763: mov qword ptr [rbp + 0x30], rax
1412c0767: mov qword ptr [rbp + 0x38], 2
1412c076f: lea rdx, [rip + 0x196cee2]
1412c0776: lea rcx, [rbp + 0x20]
1412c077a: call 0x1428d9390
1412c077f: lea rax, [rip + 0x196d9c2]
1412c0786: mov qword ptr [rbp + 0x20], rax
1412c078a: lea rax, [rip - 0xd8cef1]
1412c0791: mov qword ptr [rbp + 0x28], rax
1412c0795: mov qword ptr [rbp + 0x30], rsi
1412c0799: lea rax, [rip - 0x11b6910]
1412c07a0: mov qword ptr [rbp + 0x38], rax
1412c07a4: lea rcx, [rbp + 0x10]
1412c07a8: mov qword ptr [rbp + 0x40], rcx
1412c07ac: mov qword ptr [rbp + 0x48], rax
1412c07b0: lea rax, [rip + 0x196cdb1]
1412c07b7: mov qword ptr [rbp - 0x20], rax
1412c07bb: mov qword ptr [rbp - 0x18], 3
1412c07c3: mov qword ptr [rbp], 0
1412c07cb: lea rax, [rbp + 0x20]
1412c07cf: mov qword ptr [rbp - 0x10], rax
1412c07d3: mov qword ptr [rbp - 8], 3
1412c07db: lea rdx, [rip + 0x196cdee]
1412c07e2: lea rcx, [rbp - 0x20]
1412c07e6: call 0x1428d9390
1412c07eb: int3 
1412c07ec: nop dword ptr [rax]
1412c07f0: mov qword ptr [rsp + 0x10], rdx
1412c07f5: push rbp
1412c07f6: push rsi
1412c07f7: sub rsp, 0x28
1412c07fb: lea rbp, [rdx + 0x80]
1412c0802: lea rcx, [rbp - 0x48]
1412c0806: call 0x140be24b0
1412c080b: nop 
1412c080c: add rsp, 0x28
1412c0810: pop rsi
1412c0811: pop rbp
1412c0812: ret 
1412c0813: nop word ptr cs:[rax + rax]
1412c0820: mov qword ptr [rsp + 0x10], rdx
1412c0825: push rbp
1412c0826: push rsi
1412c0827: sub rsp, 0x28
1412c082b: lea rbp, [rdx + 0x80]
1412c0832: lea rcx, [rbp + 0x20]
1412c0836: call 0x1400c04b0
1412c083b: nop 
1412c083c: add rsp, 0x28
1412c0840: pop rsi
1412c0841: pop rbp
1412c0842: ret 
