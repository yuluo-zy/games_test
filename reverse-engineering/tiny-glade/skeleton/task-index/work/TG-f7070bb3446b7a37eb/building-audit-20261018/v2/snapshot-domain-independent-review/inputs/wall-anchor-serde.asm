1416fa3a0: 55  push rbp
1416fa3a1: 4157  push r15
1416fa3a3: 4156  push r14
1416fa3a5: 56  push rsi
1416fa3a6: 57  push rdi
1416fa3a7: 53  push rbx
1416fa3a8: 4881ecc8000000  sub rsp, 0xc8
1416fa3af: 488dac2480000000  lea rbp, [rsp + 0x80]
1416fa3b7: 440f294530  movaps xmmword ptr [rbp + 0x30], xmm8
1416fa3bc: 0f297d20  movaps xmmword ptr [rbp + 0x20], xmm7
1416fa3c0: 0f297510  movaps xmmword ptr [rbp + 0x10], xmm6
1416fa3c4: 48c74508feffffff  mov qword ptr [rbp + 8], 0xfffffffffffffffe
1416fa3cc: 4889d7  mov rdi, rdx
1416fa3cf: 4889ce  mov rsi, rcx
1416fa3d2: f30f104208  movss xmm0, dword ptr [rdx + 8]
1416fa3d7: 0f57f6  xorps xmm6, xmm6
1416fa3da: 0f2ec6  ucomiss xmm0, xmm6
1416fa3dd: f30f107a10  movss xmm7, dword ptr [rdx + 0x10]
1416fa3e2: f3440f10420c  movss xmm8, dword ptr [rdx + 0xc]
1416fa3e8: 4c8b3a  mov r15, qword ptr [rdx]
1416fa3eb: 440fb67219  movzx r14d, byte ptr [rdx + 0x19]
1416fa3f0: 0fb65a1a  movzx ebx, byte ptr [rdx + 0x1a]
1416fa3f4: 48b80000000000000080  movabs rax, 0x8000000000000000
1416fa3fe: 488945d0  mov qword ptr [rbp - 0x30], rax
1416fa402: 48c745e800000000  mov qword ptr [rbp - 0x18], 0
1416fa40a: 48c745f800000000  mov qword ptr [rbp - 8], 0
1416fa412: 0f85fe000000  jne 0x1416fa516
1416fa418: 0f8af8000000  jp 0x1416fa516
1416fa41e: 440fc2c600  cmpeqps xmm8, xmm6
1416fa423: 0fc2fe00  cmpeqps xmm7, xmm6
1416fa427: 410f54f8  andps xmm7, xmm8
1416fa42b: 660f7ef8  movd eax, xmm7
1416fa42f: f30f104714  movss xmm0, dword ptr [rdi + 0x14]
1416fa434: 0f2ec6  ucomiss xmm0, xmm6
1416fa437: 0f9bc1  setnp cl
1416fa43a: 0f94c2  sete dl
1416fa43d: 20ca  and dl, cl
1416fa43f: 84d0  test al, dl
1416fa441: 7527  jne 0x1416fa46a
1416fa443: 4c8d4f0c  lea r9, [rdi + 0xc]
1416fa447: c6450701  mov byte ptr [rbp + 7], 1
1416fa44b: 488d154c7c5901  lea rdx, [rip + 0x1597c4c]
1416fa452: 488d4dd0  lea rcx, [rbp - 0x30]
1416fa456: 41b809000000  mov r8d, 9
1416fa45c: e80f87e9ff  call 0x141592b70
1416fa461: 4885c0  test rax, rax
1416fa464: 0f85d3000000  jne 0x1416fa53d
1416fa46a: 4c8d4f18  lea r9, [rdi + 0x18]
1416fa46e: c6450701  mov byte ptr [rbp + 7], 1
1416fa472: 488d152e7c5901  lea rdx, [rip + 0x1597c2e]
1416fa479: 488d4dd0  lea rcx, [rbp - 0x30]
1416fa47d: 41b806000000  mov r8d, 6
1416fa483: e8d8d9e9ff  call 0x141597e60
1416fa488: 4885c0  test rax, rax
1416fa48b: 0f85ac000000  jne 0x1416fa53d
1416fa491: 4d85ff  test r15, r15
1416fa494: 7426  je 0x1416fa4bc
1416fa496: c6450701  mov byte ptr [rbp + 7], 1
1416fa49a: 488d150c7c5901  lea rdx, [rip + 0x1597c0c]
1416fa4a1: 488d4dd0  lea rcx, [rbp - 0x30]
1416fa4a5: 41b808000000  mov r8d, 8
1416fa4ab: 4989f9  mov r9, rdi
1416fa4ae: e84df1e9ff  call 0x141599600
1416fa4b3: 4885c0  test rax, rax
1416fa4b6: 0f8581000000  jne 0x1416fa53d
1416fa4bc: 4584f6  test r14b, r14b
1416fa4bf: 7423  je 0x1416fa4e4
1416fa4c1: 4c8d4f19  lea r9, [rdi + 0x19]
1416fa4c5: c6450701  mov byte ptr [rbp + 7], 1
1416fa4c9: 488d15e57b5901  lea rdx, [rip + 0x1597be5]
1416fa4d0: 488d4dd0  lea rcx, [rbp - 0x30]
1416fa4d4: 41b80e000000  mov r8d, 0xe
1416fa4da: e8a1ebe9ff  call 0x141599080
1416fa4df: 4885c0  test rax, rax
1416fa4e2: 7559  jne 0x1416fa53d
1416fa4e4: 84db  test bl, bl
1416fa4e6: 0f849e000000  je 0x1416fa58a
1416fa4ec: 0f1045d0  movups xmm0, xmmword ptr [rbp - 0x30]
1416fa4f0: 0f104de0  movups xmm1, xmmword ptr [rbp - 0x20]
1416fa4f4: 0f1055f0  movups xmm2, xmmword ptr [rbp - 0x10]
1416fa4f8: 0f2955c0  movaps xmmword ptr [rbp - 0x40], xmm2
1416fa4fc: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
1416fa500: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1416fa504: c6450700  mov byte ptr [rbp + 7], 0
1416fa508: 488d55a0  lea rdx, [rbp - 0x60]
1416fa50c: 4889f1  mov rcx, rsi
1416fa50f: e8acbf48ff  call 0x140b864c0
1416fa514: eb54  jmp 0x1416fa56a
1416fa516: 4c8d4f08  lea r9, [rdi + 8]
1416fa51a: c6450701  mov byte ptr [rbp + 7], 1
1416fa51e: 488d15637b5901  lea rdx, [rip + 0x1597b63]
1416fa525: 488d4dd0  lea rcx, [rbp - 0x30]
1416fa529: 41b816000000  mov r8d, 0x16
1416fa52f: e8ecb4e9ff  call 0x141595a20
1416fa534: 4885c0  test rax, rax
1416fa537: 0f84e1feffff  je 0x1416fa41e
1416fa53d: 488d4de8  lea rcx, [rbp - 0x18]
1416fa541: 48894608  mov qword ptr [rsi + 8], rax
1416fa545: c60606  mov byte ptr [rsi], 6
1416fa548: e8f3e919ff  call 0x140898f40
1416fa54d: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
1416fa551: 4889d0  mov rax, rdx
1416fa554: 48f7d8  neg rax
1416fa557: 7011  jo 0x1416fa56a
1416fa559: 730f  jae 0x1416fa56a
1416fa55b: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1416fa55f: 41b801000000  mov r8d, 1
1416fa565: e8b696f1fe  call 0x140613c20
1416fa56a: 4889f0  mov rax, rsi
1416fa56d: 0f287510  movaps xmm6, xmmword ptr [rbp + 0x10]
1416fa571: 0f287d20  movaps xmm7, xmmword ptr [rbp + 0x20]
1416fa575: 440f284530  movaps xmm8, xmmword ptr [rbp + 0x30]
1416fa57a: 4881c4c8000000  add rsp, 0xc8
1416fa581: 5b  pop rbx
1416fa582: 5f  pop rdi
1416fa583: 5e  pop rsi
1416fa584: 415e  pop r14
1416fa586: 415f  pop r15
1416fa588: 5d  pop rbp
1416fa589: c3  ret
1416fa58a: 4883c71a  add rdi, 0x1a
1416fa58e: c6450701  mov byte ptr [rbp + 7], 1
1416fa592: 488d152a7b5901  lea rdx, [rip + 0x1597b2a]
1416fa599: 488d4dd0  lea rcx, [rbp - 0x30]
1416fa59d: 41b804000000  mov r8d, 4
1416fa5a3: 4989f9  mov r9, rdi
1416fa5a6: e8d596e9ff  call 0x141593c80
1416fa5ab: 4885c0  test rax, rax
1416fa5ae: 758d  jne 0x1416fa53d
1416fa5b0: e937ffffff  jmp 0x1416fa4ec
1416fa5b5: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
1416fa5c0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416fa5c5: 55  push rbp
1416fa5c6: 4157  push r15
1416fa5c8: 4156  push r14
1416fa5ca: 56  push rsi
1416fa5cb: 57  push rdi
1416fa5cc: 53  push rbx
1416fa5cd: 4883ec58  sub rsp, 0x58
1416fa5d1: 488daa80000000  lea rbp, [rdx + 0x80]
1416fa5d8: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1416fa5de: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1416fa5e3: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1416fa5e8: 488b55d0  mov rdx, qword ptr [rbp - 0x30]
1416fa5ec: 488d045500000000  lea rax, [rdx*2]
1416fa5f4: 4885c0  test rax, rax
1416fa5f7: 740f  je 0x1416fa608
1416fa5f9: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1416fa5fd: 41b801000000  mov r8d, 1
1416fa603: e81896f1fe  call 0x140613c20
1416fa608: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1416fa60d: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1416fa612: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1416fa618: 4883c458  add rsp, 0x58
1416fa61c: 5b  pop rbx
1416fa61d: 5f  pop rdi
1416fa61e: 5e  pop rsi
1416fa61f: 415e  pop r14
1416fa621: 415f  pop r15
1416fa623: 5d  pop rbp
1416fa624: c3  ret
1416fa625: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
1416fa630: 4889542410  mov qword ptr [rsp + 0x10], rdx
1416fa635: 55  push rbp
1416fa636: 4157  push r15
1416fa638: 4156  push r14
1416fa63a: 56  push rsi
1416fa63b: 57  push rdi
1416fa63c: 53  push rbx
1416fa63d: 4883ec58  sub rsp, 0x58
1416fa641: 488daa80000000  lea rbp, [rdx + 0x80]
1416fa648: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1416fa64e: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1416fa653: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1416fa658: 807d0700  cmp byte ptr [rbp + 7], 0
1416fa65c: 751d  jne 0x1416fa67b
1416fa65e: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1416fa663: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1416fa668: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1416fa66e: 4883c458  add rsp, 0x58
1416fa672: 5b  pop rbx
1416fa673: 5f  pop rdi
1416fa674: 5e  pop rsi
1416fa675: 415e  pop r14
1416fa677: 415f  pop r15
1416fa679: 5d  pop rbp
1416fa67a: c3  ret
1416fa67b: 488d4dd0  lea rcx, [rbp - 0x30]
1416fa67f: e8dc3416ff  call 0x14085db60
1416fa684: ebd8  jmp 0x1416fa65e
