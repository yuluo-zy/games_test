141611380: push rbp
141611381: push r15
141611383: push r14
141611385: push r12
141611387: push rsi
141611388: push rdi
141611389: push rbx
14161138a: mov eax, 0x20d0
14161138f: call 0x14289aa40
141611394: sub rsp, rax
141611397: lea rbp, [rsp + 0x80]
14161139f: mov qword ptr [rbp + 0x2048], 0xfffffffffffffffe
1416113aa: mov rdi, rdx
1416113ad: mov rsi, rcx
1416113b0: test r8, r8
1416113b3: je 0x1416113f7
1416113b5: mov rbx, r9
1416113b8: mov rax, r9
1416113bb: and rax, 0x1fff
1416113c1: mov qword ptr [rbp + 0x1fc8], rax
1416113c8: jne 0x141611532
1416113ce: mov r15, r8
1416113d1: mov rax, r8
1416113d4: and rax, 3
1416113d8: mov qword ptr [rbp + 0x1fc8], rax
1416113df: jne 0x141611543
1416113e5: cmp rbx, 0x4000
1416113ec: jae 0x1416115e2
1416113f2: xor r12d, r12d
1416113f5: jmp 0x141611453
1416113f7: mov byte ptr [rbp + 0x2047], 0
1416113fe: lea rcx, [rbp - 0x40]
141611402: call 0x140b31d10
141611407: test byte ptr [rbp - 0x40], 1
14161140b: je 0x14161144b
14161140d: movzx eax, byte ptr [rip + 0x20237f5]
141611414: mov ebx, 0x2000
141611419: mov ecx, 0x2000
14161141e: mov edx, 4
141611423: call 0x140613c10
141611428: test rax, rax
14161142b: je 0x14161165e
141611431: mov r15, rax
141611434: lea rdx, [rbp - 0x3c]
141611438: mov r8d, 0x2000
14161143e: mov rcx, rax
141611441: call 0x142923c00
141611446: mov r12b, 1
141611449: jmp 0x141611453
14161144b: xor r12d, r12d
14161144e: xor ebx, ebx
141611450: xor r15d, r15d
141611453: mov qword ptr [rbp + 0x2038], r15
14161145a: mov byte ptr [rbp + 0x2047], r12b
141611461: lea rcx, [rbp - 0x40]
141611465: mov edx, 0x2000
14161146a: mov r8d, 0x12
141611470: mov r9b, 1
141611473: call 0x140bd4de0
141611478: mov qword ptr [rbp + 0x2038], r15
14161147f: mov byte ptr [rbp + 0x2047], r12b
141611486: lea r14, [rbp + 0x2018]
14161148d: lea rdx, [rbp - 0x40]
141611491: mov r8d, 4
141611497: mov rcx, r14
14161149a: call 0x140bd4f20
14161149f: mov qword ptr [rbp + 0x2038], r15
1416114a6: mov byte ptr [rbp + 0x2047], r12b
1416114ad: mov qword ptr [rsp + 0x30], rbx
1416114b2: mov qword ptr [rsp + 0x28], r15
1416114b7: mov qword ptr [rsp + 0x20], r14
1416114bc: lea r8, [rip + 0x166e70b]
1416114c3: lea rcx, [rbp + 0x1fc8]
1416114ca: mov r9d, 0x15
1416114d0: mov rdx, rdi
1416114d3: call 0x1416a2dc0
1416114d8: cmp dword ptr [rbp + 0x1fc8], 0xb
1416114df: jne 0x141611570
1416114e5: mov rax, qword ptr [rbp + 0x1ff0]
1416114ec: mov qword ptr [rsi + 0x20], rax
1416114f0: movups xmm0, xmmword ptr [rbp + 0x1fd0]
1416114f7: movups xmm1, xmmword ptr [rbp + 0x1fe0]
1416114fe: movups xmmword ptr [rsi + 0x10], xmm1
141611502: movups xmmword ptr [rsi], xmm0
141611505: test r12b, r12b
141611508: je 0x14161151d
14161150a: mov edx, 0x2000
14161150f: mov r8d, 4
141611515: mov rcx, r15
141611518: call 0x140613c20
14161151d: mov rax, rsi
141611520: add rsp, 0x20d0
141611527: pop rbx
141611528: pop rdi
141611529: pop rsi
14161152a: pop r12
14161152c: pop r14
14161152e: pop r15
141611530: pop rbp
141611531: ret 
141611532: mov qword ptr [rbp - 0x40], 0
14161153a: lea rax, [rip + 0x16b8a07]
141611541: jmp 0x141611552
141611543: mov qword ptr [rbp - 0x40], 0
14161154b: lea rax, [rip + 0x16b8a0e]
141611552: mov qword ptr [rsp + 0x20], rax
141611557: lea r8, [rip + 0x16b89e2]
14161155e: lea rdx, [rbp + 0x1fc8]
141611565: lea r9, [rbp - 0x40]
141611569: xor ecx, ecx
14161156b: call 0x1428d9596
141611570: mov qword ptr [rbp + 0x2030], r12
141611577: mov qword ptr [rbp + 0x2028], r15
14161157e: movups xmm0, xmmword ptr [rbp + 0x2008]
141611585: movaps xmmword ptr [rbp], xmm0
141611589: movups xmm0, xmmword ptr [rbp + 0x1fc8]
141611590: movups xmm1, xmmword ptr [rbp + 0x1fd8]
141611597: movups xmm2, xmmword ptr [rbp + 0x1fe8]
14161159e: movups xmm3, xmmword ptr [rbp + 0x1ff8]
1416115a5: movaps xmmword ptr [rbp - 0x10], xmm3
1416115a9: movaps xmmword ptr [rbp - 0x20], xmm2
1416115ad: movaps xmmword ptr [rbp - 0x30], xmm1
1416115b1: movaps xmmword ptr [rbp - 0x40], xmm0
1416115b5: lea rax, [rip + 0x166ebec]
1416115bc: mov qword ptr [rsp + 0x20], rax
1416115c1: lea rcx, [rip + 0x166d520]
1416115c8: lea r9, [rip + 0x166d4f9]
1416115cf: lea r8, [rbp - 0x40]
1416115d3: mov edx, 0x2b
1416115d8: call 0x1428d9760
1416115dd: jmp 0x14161167b
1416115e2: lea rax, [rip + 0x148c38f]
1416115e9: mov qword ptr [rbp + 0x1fc8], rax
1416115f0: mov qword ptr [rbp + 0x1fd0], 0x53
1416115fb: lea rax, [rbp + 0x1fc8]
141611602: mov qword ptr [rbp + 0x2018], rax
141611609: lea rax, [rip - 0x1507780]
141611610: mov qword ptr [rbp + 0x2020], rax
141611617: lea rax, [rip + 0x166e1aa]
14161161e: mov qword ptr [rbp - 0x40], rax
141611622: mov qword ptr [rbp - 0x38], 1
14161162a: mov qword ptr [rbp - 0x20], 0
141611632: lea rax, [rbp + 0x2018]
141611639: mov qword ptr [rbp - 0x30], rax
14161163d: mov qword ptr [rbp - 0x28], 1
141611645: mov byte ptr [rbp + 0x2047], 0
14161164c: lea rdx, [rip + 0x166e185]
141611653: lea rcx, [rbp - 0x40]
141611657: call 0x1428d9390
14161165c: jmp 0x14161167b
14161165e: mov byte ptr [rbp + 0x2047], 0
141611665: lea r8, [rip + 0x166d444]
14161166c: mov ecx, 4
141611671: mov edx, 0x2000
141611676: call 0x1428d8fe3
14161167b: ud2 
14161167d: nop dword ptr [rax]
141611680: mov qword ptr [rsp + 0x10], rdx
141611685: push rbp
141611686: push r15
141611688: push r14
14161168a: push r12
14161168c: push rsi
14161168d: push rdi
14161168e: push rbx
14161168f: sub rsp, 0x40
141611693: lea rbp, [rdx + 0x80]
14161169a: mov rsi, qword ptr [rbp + 0x2028]
1416116a1: mov qword ptr [rbp + 0x2038], rsi
1416116a8: mov rdi, qword ptr [rbp + 0x2030]
1416116af: mov byte ptr [rbp + 0x2047], dil
1416116b6: lea rcx, [rbp - 0x40]
1416116ba: call 0x1407cea70
1416116bf: mov qword ptr [rbp + 0x2038], rsi
1416116c6: mov byte ptr [rbp + 0x2047], dil
1416116cd: add rsp, 0x40
1416116d1: pop rbx
1416116d2: pop rdi
1416116d3: pop rsi
1416116d4: pop r12
1416116d6: pop r14
1416116d8: pop r15
1416116da: pop rbp
1416116db: ret 
1416116dc: nop dword ptr [rax]
1416116e0: mov qword ptr [rsp + 0x10], rdx
1416116e5: push rbp
1416116e6: push r15
1416116e8: push r14
1416116ea: push r12
1416116ec: push rsi
1416116ed: push rdi
1416116ee: push rbx
1416116ef: sub rsp, 0x40
1416116f3: lea rbp, [rdx + 0x80]
1416116fa: test byte ptr [rbp + 0x2047], 1
141611701: je 0x14161171a
141611703: mov rcx, qword ptr [rbp + 0x2038]
14161170a: mov edx, 0x2000
14161170f: mov r8d, 4
141611715: call 0x140613c20
14161171a: nop 
14161171b: add rsp, 0x40
14161171f: pop rbx
141611720: pop rdi
141611721: pop rsi
141611722: pop r12
141611724: pop r14
141611726: pop r15
141611728: pop rbp
141611729: ret 
