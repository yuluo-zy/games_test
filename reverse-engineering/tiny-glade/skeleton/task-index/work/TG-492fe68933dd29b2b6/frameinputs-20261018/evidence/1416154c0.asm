1416154c0: push rbp
1416154c1: push r15
1416154c3: push r14
1416154c5: push r12
1416154c7: push rsi
1416154c8: push rdi
1416154c9: push rbx
1416154ca: sub rsp, 0x280
1416154d1: lea rbp, [rsp + 0x80]
1416154d9: mov qword ptr [rbp + 0x1f8], 0xfffffffffffffffe
1416154e4: mov rdi, rdx
1416154e7: mov rsi, rcx
1416154ea: test r8, r8
1416154ed: je 0x141615504
1416154ef: mov rbx, r9
1416154f2: cmp r9, 2
1416154f6: jae 0x1416156c1
1416154fc: mov r15, r8
1416154ff: xor r12d, r12d
141615502: jmp 0x14161555e
141615504: mov byte ptr [rbp + 0x1f7], 0
14161550b: lea rcx, [rbp - 0x40]
14161550f: call 0x1408bb650
141615514: test byte ptr [rbp - 0x40], 1
141615518: je 0x141615558
14161551a: movzx eax, byte ptr [rip + 0x201f6e8]
141615521: mov ecx, 0x1b0
141615526: mov edx, 4
14161552b: call 0x140613c10
141615530: test rax, rax
141615533: je 0x14161573d
141615539: mov r15, rax
14161553c: lea rdx, [rbp - 0x3c]
141615540: mov r8d, 0x1b0
141615546: mov rcx, rax
141615549: call 0x142923c00
14161554e: mov ebx, 1
141615553: mov r12b, 1
141615556: jmp 0x14161555e
141615558: xor r12d, r12d
14161555b: xor r15d, r15d
14161555e: mov qword ptr [rbp + 0x1e8], r15
141615565: mov byte ptr [rbp + 0x1f7], r12b
14161556c: lea rcx, [rbp - 0x40]
141615570: mov edx, 0x1b0
141615575: mov r8d, 0x12
14161557b: mov r9b, 1
14161557e: call 0x140bd4de0
141615583: mov qword ptr [rbp + 0x1e8], r15
14161558a: mov byte ptr [rbp + 0x1f7], r12b
141615591: lea r14, [rbp + 0x1c8]
141615598: lea rdx, [rbp - 0x40]
14161559c: mov r8d, 4
1416155a2: mov rcx, r14
1416155a5: call 0x140bd4f20
1416155aa: imul rax, rbx, 0x1b0
1416155b1: mov qword ptr [rbp + 0x1e8], r15
1416155b8: mov byte ptr [rbp + 0x1f7], r12b
1416155bf: mov qword ptr [rsp + 0x30], rax
1416155c4: mov qword ptr [rsp + 0x28], r15
1416155c9: mov qword ptr [rsp + 0x20], r14
1416155ce: lea r8, [rip + 0x166a243]
1416155d5: lea rcx, [rbp + 0x178]
1416155dc: mov r9d, 0x1a
1416155e2: mov rdx, rdi
1416155e5: call 0x1416a2dc0
1416155ea: cmp dword ptr [rbp + 0x178], 0xb
1416155f1: jne 0x14161564f
1416155f3: mov rax, qword ptr [rbp + 0x1a0]
1416155fa: mov qword ptr [rsi + 0x20], rax
1416155fe: movups xmm0, xmmword ptr [rbp + 0x180]
141615605: movups xmm1, xmmword ptr [rbp + 0x190]
14161560c: movups xmmword ptr [rsi + 0x10], xmm1
141615610: movups xmmword ptr [rsi], xmm0
141615613: test r12b, r12b
141615616: je 0x14161563c
141615618: mov edx, 0x1b0
14161561d: mov r8d, 4
141615623: mov rcx, r15
141615626: add rsp, 0x280
14161562d: pop rbx
14161562e: pop rdi
14161562f: pop rsi
141615630: pop r12
141615632: pop r14
141615634: pop r15
141615636: pop rbp
141615637: jmp 0x140613c20
14161563c: nop 
14161563d: add rsp, 0x280
141615644: pop rbx
141615645: pop rdi
141615646: pop rsi
141615647: pop r12
141615649: pop r14
14161564b: pop r15
14161564d: pop rbp
14161564e: ret 
14161564f: mov qword ptr [rbp + 0x1e0], r12
141615656: mov qword ptr [rbp + 0x1d8], r15
14161565d: movups xmm0, xmmword ptr [rbp + 0x1b8]
141615664: movaps xmmword ptr [rbp], xmm0
141615668: movups xmm0, xmmword ptr [rbp + 0x178]
14161566f: movups xmm1, xmmword ptr [rbp + 0x188]
141615676: movups xmm2, xmmword ptr [rbp + 0x198]
14161567d: movups xmm3, xmmword ptr [rbp + 0x1a8]
141615684: movaps xmmword ptr [rbp - 0x10], xmm3
141615688: movaps xmmword ptr [rbp - 0x20], xmm2
14161568c: movaps xmmword ptr [rbp - 0x30], xmm1
141615690: movaps xmmword ptr [rbp - 0x40], xmm0
141615694: lea rax, [rip + 0x166ab0d]
14161569b: mov qword ptr [rsp + 0x20], rax
1416156a0: lea rcx, [rip + 0x1669441]
1416156a7: lea r9, [rip + 0x166941a]
1416156ae: lea r8, [rbp - 0x40]
1416156b2: mov edx, 0x2b
1416156b7: call 0x1428d9760
1416156bc: jmp 0x14161575a
1416156c1: lea rax, [rip + 0x1487922]
1416156c8: mov qword ptr [rbp + 0x178], rax
1416156cf: mov qword ptr [rbp + 0x180], 0x3f
1416156da: lea rax, [rbp + 0x178]
1416156e1: mov qword ptr [rbp + 0x1c8], rax
1416156e8: lea rax, [rip - 0x150b85f]
1416156ef: mov qword ptr [rbp + 0x1d0], rax
1416156f6: lea rax, [rip + 0x166a0cb]
1416156fd: mov qword ptr [rbp - 0x40], rax
141615701: mov qword ptr [rbp - 0x38], 1
141615709: mov qword ptr [rbp - 0x20], 0
141615711: lea rax, [rbp + 0x1c8]
141615718: mov qword ptr [rbp - 0x30], rax
14161571c: mov qword ptr [rbp - 0x28], 1
141615724: mov byte ptr [rbp + 0x1f7], 0
14161572b: lea rdx, [rip + 0x166a0a6]
141615732: lea rcx, [rbp - 0x40]
141615736: call 0x1428d9390
14161573b: jmp 0x14161575a
14161573d: mov byte ptr [rbp + 0x1f7], 0
141615744: lea r8, [rip + 0x1669365]
14161574b: mov ecx, 4
141615750: mov edx, 0x1b0
141615755: call 0x1428d8fe3
14161575a: ud2 
14161575c: nop dword ptr [rax]
141615760: mov qword ptr [rsp + 0x10], rdx
141615765: push rbp
141615766: push r15
141615768: push r14
14161576a: push r12
14161576c: push rsi
14161576d: push rdi
14161576e: push rbx
14161576f: sub rsp, 0x40
141615773: lea rbp, [rdx + 0x80]
14161577a: mov rsi, qword ptr [rbp + 0x1d8]
141615781: mov qword ptr [rbp + 0x1e8], rsi
141615788: mov rdi, qword ptr [rbp + 0x1e0]
14161578f: mov byte ptr [rbp + 0x1f7], dil
141615796: lea rcx, [rbp - 0x40]
14161579a: call 0x1407cea70
14161579f: mov qword ptr [rbp + 0x1e8], rsi
1416157a6: mov byte ptr [rbp + 0x1f7], dil
1416157ad: add rsp, 0x40
1416157b1: pop rbx
1416157b2: pop rdi
1416157b3: pop rsi
1416157b4: pop r12
1416157b6: pop r14
1416157b8: pop r15
1416157ba: pop rbp
1416157bb: ret 
1416157bc: nop dword ptr [rax]
1416157c0: mov qword ptr [rsp + 0x10], rdx
1416157c5: push rbp
1416157c6: push r15
1416157c8: push r14
1416157ca: push r12
1416157cc: push rsi
1416157cd: push rdi
1416157ce: push rbx
1416157cf: sub rsp, 0x40
1416157d3: lea rbp, [rdx + 0x80]
1416157da: test byte ptr [rbp + 0x1f7], 1
1416157e1: je 0x1416157fa
1416157e3: mov rcx, qword ptr [rbp + 0x1e8]
1416157ea: mov edx, 0x1b0
1416157ef: mov r8d, 4
1416157f5: call 0x140613c20
1416157fa: nop 
1416157fb: add rsp, 0x40
1416157ff: pop rbx
141615800: pop rdi
141615801: pop rsi
141615802: pop r12
141615804: pop r14
141615806: pop r15
141615808: pop rbp
141615809: ret 
