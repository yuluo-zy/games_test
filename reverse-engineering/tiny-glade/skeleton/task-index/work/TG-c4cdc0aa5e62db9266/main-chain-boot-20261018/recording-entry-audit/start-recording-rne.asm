140a86270: push rbp
140a86271: push rsi
140a86272: sub rsp, 0x88
140a86279: lea rbp, [rsp + 0x80]
140a86281: mov qword ptr [rbp], 0xfffffffffffffffe
140a86289: mov rsi, rcx
140a8628c: mov rcx, qword ptr [rdx]
140a8628f: lea rdx, [rip + 0x2052152]
140a86296: call 0x140a825e0
140a8629b: test al, al
140a8629d: je 0x140a863ec
140a862a3: mov qword ptr [rbp - 0x58], 0
140a862ab: mov qword ptr [rbp - 0x50], 8
140a862b3: mov qword ptr [rbp - 0x48], 0
140a862bb: movzx eax, byte ptr [rip + 0x2bae947]
140a862c2: mov ecx, 0x40
140a862c7: mov edx, 8
140a862cc: call 0x140613c10
140a862d1: mov qword ptr [rbp - 8], rax
140a862d5: test rax, rax
140a862d8: je 0x140a86404
140a862de: xor ecx, ecx
140a862e0: call 0x1400d9b20
140a862e5: test rax, rax
140a862e8: je 0x140a86415
140a862ee: movups xmm0, xmmword ptr [rax]
140a862f1: inc qword ptr [rax]
140a862f4: mov rax, qword ptr [rbp - 8]
140a862f8: mov qword ptr [rax], 1
140a862ff: mov qword ptr [rax + 8], 1
140a86307: movups xmm1, xmmword ptr [rip + 0x206b162]
140a8630e: movups xmmword ptr [rax + 0x10], xmm1
140a86312: movups xmm1, xmmword ptr [rip + 0x206b167]
140a86319: movups xmmword ptr [rax + 0x20], xmm1
140a8631d: movups xmmword ptr [rax + 0x30], xmm0
140a86321: mov qword ptr [rbp - 0x60], rax
140a86325: movzx eax, byte ptr [rip + 0x2bae8dd]
140a8632c: mov ecx, 0x60
140a86331: mov edx, 8
140a86336: call 0x140613c10
140a8633b: test rax, rax
140a8633e: je 0x140a86423
140a86344: mov qword ptr [rax], 1
140a8634b: mov qword ptr [rax + 8], 1
140a86353: mov qword ptr [rax + 0x10], 0
140a8635b: mov qword ptr [rax + 0x20], 0
140a86363: mov qword ptr [rax + 0x30], 0
140a8636b: mov qword ptr [rax + 0x38], 4
140a86373: xorps xmm0, xmm0
140a86376: movups xmmword ptr [rax + 0x40], xmm0
140a8637a: mov qword ptr [rax + 0x50], 4
140a86382: mov qword ptr [rbp - 0x18], rax
140a86386: mov qword ptr [rax + 0x58], 0
140a8638e: mov rax, qword ptr [rbp - 0x48]
140a86392: mov qword ptr [rbp - 0x30], rax
140a86396: movups xmm0, xmmword ptr [rbp - 0x58]
140a8639a: movaps xmmword ptr [rbp - 0x40], xmm0
140a8639e: lea rax, [rsi + 0x30]
140a863a2: mov qword ptr [rbp - 0x10], rax
140a863a6: xor eax, eax
140a863a8: mov qword ptr [rbp - 0x20], rsi
140a863ac: cmp rax, qword ptr [rsi + 0x30]
140a863b0: jo 0x140a863bb
140a863b2: mov rcx, qword ptr [rbp - 0x10]
140a863b6: call 0x14088bae0
140a863bb: mov rax, qword ptr [rbp - 0x30]
140a863bf: mov rcx, qword ptr [rbp - 0x10]
140a863c3: mov qword ptr [rcx + 0x10], rax
140a863c7: movaps xmm0, xmmword ptr [rbp - 0x40]
140a863cb: movups xmmword ptr [rcx], xmm0
140a863ce: mov rax, qword ptr [rbp - 8]
140a863d2: mov rcx, qword ptr [rbp - 0x20]
140a863d6: mov qword ptr [rcx + 0x48], rax
140a863da: mov rax, qword ptr [rbp - 0x18]
140a863de: mov qword ptr [rcx + 0x50], rax
140a863e2: add rsp, 0x88
140a863e9: pop rsi
140a863ea: pop rbp
140a863eb: ret 
140a863ec: lea rcx, [rip + 0x2052005]
140a863f3: lea r8, [rip + 0x205202e]
140a863fa: mov edx, 0x2f
140a863ff: call 0x1428d9430
140a86404: mov ecx, 8
140a86409: mov edx, 0x40
140a8640e: call 0x1428d9000
140a86413: jmp 0x140a86432
140a86415: lea rcx, [rip + 0x21b4684]
140a8641c: call 0x1428d80d0
140a86421: jmp 0x140a86432
140a86423: mov ecx, 8
140a86428: mov edx, 0x60
140a8642d: call 0x1428d9000
140a86432: ud2 
140a86434: nop word ptr cs:[rax + rax]
140a86440: mov qword ptr [rsp + 0x10], rdx
140a86445: push rbp
140a86446: push rsi
140a86447: sub rsp, 0x28
140a8644b: lea rbp, [rdx + 0x80]
140a86452: mov edx, 0x40
140a86457: mov r8d, 8
140a8645d: mov rcx, qword ptr [rbp - 8]
140a86461: call 0x140613c20
140a86466: nop 
140a86467: add rsp, 0x28
140a8646b: pop rsi
140a8646c: pop rbp
140a8646d: ret 
140a8646e: nop 
140a86470: mov qword ptr [rsp + 0x10], rdx
140a86475: push rbp
140a86476: push rsi
140a86477: sub rsp, 0x28
140a8647b: lea rbp, [rdx + 0x80]
140a86482: lea rcx, [rbp - 0x58]
140a86486: call 0x140894ab0
140a8648b: nop 
140a8648c: add rsp, 0x28
140a86490: pop rsi
140a86491: pop rbp
140a86492: ret 
140a86493: nop word ptr cs:[rax + rax]
140a864a0: mov qword ptr [rsp + 0x10], rdx
140a864a5: push rbp
140a864a6: push rsi
140a864a7: sub rsp, 0x28
140a864ab: lea rbp, [rdx + 0x80]
140a864b2: mov rax, qword ptr [rbp - 8]
140a864b6: lock dec qword ptr [rax]
140a864ba: jne 0x140a864c5
140a864bc: lea rcx, [rbp - 0x60]
140a864c0: call 0x140b5cb60
140a864c5: nop 
140a864c6: add rsp, 0x28
140a864ca: pop rsi
140a864cb: pop rbp
140a864cc: ret 
140a864cd: nop dword ptr [rax]
140a864d0: mov qword ptr [rsp + 0x10], rdx
140a864d5: push rbp
140a864d6: push rsi
140a864d7: sub rsp, 0x28
140a864db: lea rbp, [rdx + 0x80]
140a864e2: mov rax, qword ptr [rbp - 0x30]
140a864e6: mov rcx, qword ptr [rbp - 0x10]
140a864ea: mov qword ptr [rcx + 0x10], rax
140a864ee: movaps xmm0, xmmword ptr [rbp - 0x40]
140a864f2: movups xmmword ptr [rcx], xmm0
140a864f5: mov rax, qword ptr [rbp - 8]
140a864f9: mov rcx, qword ptr [rbp - 0x20]
140a864fd: mov qword ptr [rcx + 0x48], rax
140a86501: mov rax, qword ptr [rbp - 0x18]
140a86505: mov qword ptr [rcx + 0x50], rax
140a86509: add rsp, 0x28
140a8650d: pop rsi
140a8650e: pop rbp
140a8650f: ret 
