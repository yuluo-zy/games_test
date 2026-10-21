0x140fc9230: push rbp
0x140fc9231: push r14
0x140fc9233: push rsi
0x140fc9234: push rdi
0x140fc9235: push rbx
0x140fc9236: sub rsp, 0xd0
0x140fc923d: lea rbp, [rsp + 0x80]
0x140fc9245: movaps xmmword ptr [rbp + 0x40], xmm6
0x140fc9249: mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
0x140fc9251: mov rsi, rcx
0x140fc9254: mov rbx, qword ptr [rcx]
0x140fc9257: cmp byte ptr [rbx], 0
0x140fc925a: je 0x140fc92f8
0x140fc9260: lea rcx, [rip + 0x1beac46]
0x140fc9267: mov rdi, rdx
0x140fc926a: mov edx, 0xe
0x140fc926f: call 0x140b9d480
0x140fc9274: mov qword ptr [rbp - 0x18], rax
0x140fc9278: mov qword ptr [rbp - 0x10], rdx
0x140fc927c: lea rax, [rbp - 0x18]
0x140fc9280: mov qword ptr [rbp - 0x40], rax
0x140fc9284: lea rax, [rip - 0xebf3fb]
0x140fc928b: mov qword ptr [rbp - 0x38], rax
0x140fc928f: lea rax, [rip + 0x1beac2a]
0x140fc9296: mov qword ptr [rbp], rax
0x140fc929a: mov qword ptr [rbp + 8], 2
0x140fc92a2: lea rax, [rbp - 0x40]
0x140fc92a6: mov qword ptr [rbp + 0x10], rax
0x140fc92aa: mov qword ptr [rbp + 0x18], 1
0x140fc92b2: mov qword ptr [rbp + 0x20], 0
0x140fc92ba: lea rcx, [rbp - 0x30]
0x140fc92be: mov rdx, rbp
0x140fc92c1: call 0x140d79f90
0x140fc92c6: mov r8, qword ptr [rbp - 0x28]
0x140fc92ca: mov r9, qword ptr [rbp - 0x20]
0x140fc92ce: lea rcx, [rbp - 0x18]
0x140fc92d2: mov rdx, rdi
0x140fc92d5: mov qword ptr [rbp + 0x30], r8
0x140fc92d9: call 0x14068bfc0
0x140fc92de: mov rdx, qword ptr [rbp - 0x30]
0x140fc92e2: test rdx, rdx
0x140fc92e5: je 0x140fc930e
0x140fc92e7: mov r8d, 1
0x140fc92ed: mov rcx, qword ptr [rbp + 0x30]
0x140fc92f1: call 0x140613c20
0x140fc92f6: jmp 0x140fc930e
0x140fc92f8: lea r8, [rip + 0x1beb459]
0x140fc92ff: lea rcx, [rbp - 0x18]
0x140fc9303: mov r9d, 6
0x140fc9309: call 0x14068bfc0
0x140fc930e: mov rax, qword ptr [rbp - 8]
0x140fc9312: mov qword ptr [rbp + 0x10], rax
0x140fc9316: movups xmm0, xmmword ptr [rbp - 0x18]
0x140fc931a: movaps xmmword ptr [rbp], xmm0
0x140fc931e: lea rdi, [rbp - 0x58]
0x140fc9322: mov rdx, rbp
0x140fc9325: mov rcx, rdi
0x140fc9328: mov r8b, 0x13
0x140fc932b: call 0x14067e6d0
0x140fc9330: mov rax, qword ptr [rsi + 8]
0x140fc9334: movss xmm6, dword ptr [rax]
0x140fc9338: mulss xmm6, dword ptr [rip + 0x195c590]
0x140fc9340: mov rcx, rdi
0x140fc9343: mov dl, 1
0x140fc9345: call 0x140f9dba0
0x140fc934a: mov rcx, rax
0x140fc934d: movaps xmm1, xmm6
0x140fc9350: call 0x140fac150
0x140fc9355: movdqu xmm0, xmmword ptr [rbp - 0x58]
0x140fc935a: movdqa xmmword ptr [rbp], xmm0
0x140fc935f: mov rax, qword ptr [rbp - 0x48]
0x140fc9363: mov qword ptr [rbp + 0x10], rax
0x140fc9367: mov rdi, qword ptr [rbp]
0x140fc936b: cmp qword ptr [rdi + 0x1b8], 0
0x140fc9373: je 0x140fc94b6
0x140fc9379: lea rdx, [rbp + 8]
0x140fc937d: lea rcx, [rdi + 0x1c0]
0x140fc9384: call 0x1405b0d70
0x140fc9389: mov rcx, qword ptr [rdi + 0x1a0]
0x140fc9390: mov rdx, qword ptr [rdi + 0x1a8]
0x140fc9397: mov r8, rdx
0x140fc939a: and r8, rax
0x140fc939d: shr rax, 0x39
0x140fc93a1: movd xmm0, eax
0x140fc93a5: punpcklbw xmm0, xmm0
0x140fc93a9: pshuflw xmm0, xmm0, 0
0x140fc93ae: pshufd xmm0, xmm0, 0
0x140fc93b3: lea rax, [rcx - 0xc0]
0x140fc93ba: xor r9d, r9d
0x140fc93bd: mov r10, qword ptr [rbp + 8]
0x140fc93c1: pcmpeqd xmm1, xmm1
0x140fc93c5: movdqu xmm2, xmmword ptr [rcx + r8]
0x140fc93cb: movdqa xmm3, xmm2
0x140fc93cf: pcmpeqb xmm3, xmm0
0x140fc93d3: pmovmskb r11d, xmm3
0x140fc93d8: test r11d, r11d
0x140fc93db: je 0x140fc9410
0x140fc93dd: tzcnt edi, r11d
0x140fc93e2: add rdi, r8
0x140fc93e5: and rdi, rdx
0x140fc93e8: neg rdi
0x140fc93eb: lea rdi, [rdi + rdi*2]
0x140fc93ef: shl rdi, 6
0x140fc93f3: cmp r10, qword ptr [rax + rdi]
0x140fc93f7: je 0x140fc9432
0x140fc93f9: lea edi, [r11 - 1]
0x140fc93fd: and di, r11w
0x140fc9401: mov r11d, edi
0x140fc9404: jne 0x140fc93dd
0x140fc9406: nop word ptr cs:[rax + rax]
0x140fc9410: pcmpeqb xmm2, xmm1
0x140fc9414: pmovmskb r11d, xmm2
0x140fc9419: test r11d, r11d
0x140fc941c: jne 0x140fc94b6
0x140fc9422: add r8, r9
0x140fc9425: add r8, 0x10
0x140fc9429: add r9, 0x10
0x140fc942d: and r8, rdx
0x140fc9430: jmp 0x140fc93c5
0x140fc9432: test byte ptr [rcx + rdi - 2], 0x30
0x140fc9437: je 0x140fc94d8
0x140fc943d: cmp byte ptr [rbx], 0
0x140fc9440: je 0x140fc94ce
0x140fc9446: mov rax, qword ptr [rsi + 0x10]
0x140fc944a: mov ecx, dword ptr [rax + 0x1c]
0x140fc944d: mov rdx, qword ptr [rax + 0x10]
0x140fc9451: mov dword ptr [rdx], ecx
0x140fc9453: mov rax, qword ptr [rax]
0x140fc9456: mov byte ptr [rax + 0x206], 1
0x140fc945d: mov rax, qword ptr [rsi + 0x18]
0x140fc9461: mov ecx, dword ptr [rax + 0x1c]
0x140fc9464: mov rdx, qword ptr [rax + 0x10]
0x140fc9468: mov dword ptr [rdx], ecx
0x140fc946a: mov rdi, qword ptr [rax]
0x140fc946d: mov rbx, qword ptr [rdi + 0x30]
0x140fc9471: mov r14, qword ptr [rdi + 0x40]
0x140fc9475: cmp rbx, qword ptr [rdi + 0x20]
0x140fc9479: jne 0x140fc948b
0x140fc947b: lea rcx, [rdi + 0x20]
0x140fc947f: lea rdx, [rip + 0x1bebdea]
0x140fc9486: call 0x140562870
0x140fc948b: mov rax, qword ptr [rdi + 0x28]
0x140fc948f: mov qword ptr [rax + rbx*8], r14
0x140fc9493: inc rbx
0x140fc9496: mov qword ptr [rdi + 0x30], rbx
0x140fc949a: inc qword ptr [rdi + 0x40]
0x140fc949e: mov rax, qword ptr [rsi + 0x20]
0x140fc94a2: mov ecx, dword ptr [rax + 0x1c]
0x140fc94a5: mov rdx, qword ptr [rax + 0x10]
0x140fc94a9: mov dword ptr [rdx], ecx
0x140fc94ab: mov rax, qword ptr [rax]
0x140fc94ae: mov dword ptr [rax], 2
0x140fc94b4: jmp 0x140fc94d8
0x140fc94b6: lea rcx, [rip + 0x1a4dd4b]
0x140fc94bd: lea r8, [rip + 0x1a4eb14]
0x140fc94c4: mov edx, 0x16
0x140fc94c9: call 0x1428d9330
0x140fc94ce: mov rax, qword ptr [rsi + 0x28]
0x140fc94d2: mov rax, qword ptr [rax]
0x140fc94d5: inc qword ptr [rax]
0x140fc94d8: movaps xmm6, xmmword ptr [rbp + 0x40]
0x140fc94dc: add rsp, 0xd0
0x140fc94e3: pop rbx
0x140fc94e4: pop rdi
0x140fc94e5: pop rsi
0x140fc94e6: pop r14
0x140fc94e8: pop rbp
0x140fc94e9: ret 
0x140fc94ea: nop word ptr [rax + rax]
0x140fc94f0: mov qword ptr [rsp + 0x10], rdx
0x140fc94f5: push rbp
0x140fc94f6: push r14
0x140fc94f8: push rsi
0x140fc94f9: push rdi
0x140fc94fa: push rbx
0x140fc94fb: sub rsp, 0x30
0x140fc94ff: lea rbp, [rdx + 0x80]
0x140fc9506: movaps xmmword ptr [rsp + 0x20], xmm6
0x140fc950b: mov rdx, qword ptr [rbp - 0x30]
0x140fc950f: test rdx, rdx
0x140fc9512: je 0x140fc9523
0x140fc9514: mov r8d, 1
0x140fc951a: mov rcx, qword ptr [rbp + 0x30]
0x140fc951e: call 0x140613c20
0x140fc9523: movaps xmm6, xmmword ptr [rsp + 0x20]
0x140fc9528: add rsp, 0x30
0x140fc952c: pop rbx
0x140fc952d: pop rdi
0x140fc952e: pop rsi
0x140fc952f: pop r14
0x140fc9531: pop rbp
0x140fc9532: ret 
