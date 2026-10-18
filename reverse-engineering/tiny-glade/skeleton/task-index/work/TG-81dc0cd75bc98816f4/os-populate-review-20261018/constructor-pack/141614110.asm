141614110: push rbp
141614111: push r15
141614113: push r14
141614115: push r12
141614117: push rsi
141614118: push rdi
141614119: push rbx
14161411a: sub rsp, 0x120
141614121: lea rbp, [rsp + 0x80]
141614129: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141614134: mov rdi, rdx
141614137: mov rsi, rcx
14161413a: test r8, r8
14161413d: je 0x141614179
14161413f: mov rbx, r9
141614142: mov rax, r9
141614145: and rax, 0x1f
141614149: mov qword ptr [rbp - 0x40], rax
14161414d: jne 0x1416142be
141614153: mov r15, r8
141614156: mov rax, r8
141614159: and rax, 3
14161415d: mov qword ptr [rbp - 0x40], rax
141614161: jne 0x1416142cf
141614167: cmp rbx, 0x6820
14161416e: jae 0x141614359
141614174: xor r12d, r12d
141614177: jmp 0x1416141f5
141614179: movzx eax, byte ptr [rip + 0x2020a89]
141614180: mov ecx, 0x6800
141614185: mov edx, 4
14161418a: call 0x140613c10
14161418f: test rax, rax
141614192: je 0x1416143c3
141614198: mov r15, rax
14161419b: mov eax, 0x34
1416141a0: xorps xmm0, xmm0
1416141a3: mov rcx, qword ptr [rip + 0x149375e]
1416141aa: mov edx, dword ptr [rip + 0x1493760]
1416141b0: movups xmmword ptr [r15 + rax - 0x34], xmm0
1416141b6: mov dword ptr [r15 + rax - 0x24], 0x3f800000
1416141bf: mov qword ptr [r15 + rax - 0x20], rcx
1416141c4: mov dword ptr [r15 + rax - 0x18], edx
1416141c9: movups xmmword ptr [r15 + rax - 0x14], xmm0
1416141cf: mov dword ptr [r15 + rax - 4], 0x3f800000
1416141d8: mov qword ptr [r15 + rax], rcx
1416141dc: mov dword ptr [r15 + rax + 8], edx
1416141e1: add rax, 0x40
1416141e5: cmp rax, 0x6834
1416141eb: jne 0x1416141b0
1416141ed: mov r12b, 1
1416141f0: mov ebx, 0x6800
1416141f5: mov qword ptr [rbp + 0x88], r15
1416141fc: mov byte ptr [rbp + 0x97], r12b
141614203: lea rcx, [rbp + 0x10]
141614207: mov edx, 0x6800
14161420c: mov r8d, 0x22
141614212: mov r9b, 1
141614215: call 0x140bd4de0
14161421a: mov qword ptr [rbp + 0x88], r15
141614221: mov byte ptr [rbp + 0x97], r12b
141614228: lea r14, [rbp + 0x68]
14161422c: lea rdx, [rbp + 0x10]
141614230: mov r8d, 4
141614236: mov rcx, r14
141614239: call 0x140bd4f20
14161423e: mov qword ptr [rbp + 0x88], r15
141614245: mov byte ptr [rbp + 0x97], r12b
14161424c: mov qword ptr [rsp + 0x30], rbx
141614251: mov qword ptr [rsp + 0x28], r15
141614256: mov qword ptr [rsp + 0x20], r14
14161425b: lea r8, [rip + 0x166bcc6]
141614262: lea rcx, [rbp - 0x40]
141614266: mov r9d, 0xd
14161426c: mov rdx, rdi
14161426f: call 0x1416a2dc0
141614274: cmp dword ptr [rbp - 0x40], 0xb
141614278: jne 0x1416142f9
14161427a: mov rax, qword ptr [rbp - 0x18]
14161427e: mov qword ptr [rsi + 0x20], rax
141614282: movups xmm0, xmmword ptr [rbp - 0x38]
141614286: movups xmm1, xmmword ptr [rbp - 0x28]
14161428a: movups xmmword ptr [rsi + 0x10], xmm1
14161428e: movups xmmword ptr [rsi], xmm0
141614291: test r12b, r12b
141614294: je 0x1416142a9
141614296: mov edx, 0x6800
14161429b: mov r8d, 4
1416142a1: mov rcx, r15
1416142a4: call 0x140613c20
1416142a9: mov rax, rsi
1416142ac: add rsp, 0x120
1416142b3: pop rbx
1416142b4: pop rdi
1416142b5: pop rsi
1416142b6: pop r12
1416142b8: pop r14
1416142ba: pop r15
1416142bc: pop rbp
1416142bd: ret 
1416142be: mov qword ptr [rbp + 0x10], 0
1416142c6: lea rax, [rip + 0x16b5c7b]
1416142cd: jmp 0x1416142de
1416142cf: mov qword ptr [rbp + 0x10], 0
1416142d7: lea rax, [rip + 0x16b5c82]
1416142de: mov qword ptr [rsp + 0x20], rax
1416142e3: lea r8, [rip + 0x16b5c56]
1416142ea: lea rdx, [rbp - 0x40]
1416142ee: lea r9, [rbp + 0x10]
1416142f2: xor ecx, ecx
1416142f4: call 0x1428d9596
1416142f9: mov dword ptr [rbp + 0x84], r12d
141614300: mov qword ptr [rbp + 0x78], r15
141614304: movups xmm0, xmmword ptr [rbp]
141614308: movaps xmmword ptr [rbp + 0x50], xmm0
14161430c: movups xmm0, xmmword ptr [rbp - 0x40]
141614310: movups xmm1, xmmword ptr [rbp - 0x30]
141614314: movups xmm2, xmmword ptr [rbp - 0x20]
141614318: movups xmm3, xmmword ptr [rbp - 0x10]
14161431c: movaps xmmword ptr [rbp + 0x40], xmm3
141614320: movaps xmmword ptr [rbp + 0x30], xmm2
141614324: movaps xmmword ptr [rbp + 0x20], xmm1
141614328: movaps xmmword ptr [rbp + 0x10], xmm0
14161432c: lea rax, [rip + 0x166be75]
141614333: mov qword ptr [rsp + 0x20], rax
141614338: lea rcx, [rip + 0x166a7a9]
14161433f: lea r9, [rip + 0x166a782]
141614346: lea r8, [rbp + 0x10]
14161434a: mov edx, 0x2b
14161434f: call 0x1428d9760
141614354: jmp 0x1416143e0
141614359: lea rax, [rip + 0x148a53d]
141614360: mov qword ptr [rbp - 0x40], rax
141614364: mov qword ptr [rbp - 0x38], 0x54
14161436c: lea rax, [rbp - 0x40]
141614370: mov qword ptr [rbp + 0x68], rax
141614374: lea rax, [rip - 0x150a4eb]
14161437b: mov qword ptr [rbp + 0x70], rax
14161437f: lea rax, [rip + 0x166b442]
141614386: mov qword ptr [rbp + 0x10], rax
14161438a: mov qword ptr [rbp + 0x18], 1
141614392: mov qword ptr [rbp + 0x30], 0
14161439a: lea rax, [rbp + 0x68]
14161439e: mov qword ptr [rbp + 0x20], rax
1416143a2: mov qword ptr [rbp + 0x28], 1
1416143aa: mov byte ptr [rbp + 0x97], 0
1416143b1: lea rdx, [rip + 0x166b420]
1416143b8: lea rcx, [rbp + 0x10]
1416143bc: call 0x1428d9390
1416143c1: jmp 0x1416143e0
1416143c3: mov byte ptr [rbp + 0x97], 0
1416143ca: lea r8, [rip + 0x166a6df]
1416143d1: mov ecx, 4
1416143d6: mov edx, 0x6800
1416143db: call 0x1428d8fe3
1416143e0: ud2 
1416143e2: nop word ptr cs:[rax + rax]
1416143f0: mov qword ptr [rsp + 0x10], rdx
1416143f5: push rbp
1416143f6: push r15
1416143f8: push r14
1416143fa: push r12
1416143fc: push rsi
1416143fd: push rdi
1416143fe: push rbx
1416143ff: sub rsp, 0x40
141614403: lea rbp, [rdx + 0x80]
14161440a: mov rsi, qword ptr [rbp + 0x78]
14161440e: mov qword ptr [rbp + 0x88], rsi
141614415: mov edi, dword ptr [rbp + 0x84]
14161441b: mov byte ptr [rbp + 0x97], dil
141614422: lea rcx, [rbp + 0x10]
141614426: call 0x1407cea70
14161442b: mov qword ptr [rbp + 0x88], rsi
141614432: mov byte ptr [rbp + 0x97], dil
141614439: add rsp, 0x40
14161443d: pop rbx
14161443e: pop rdi
14161443f: pop rsi
141614440: pop r12
141614442: pop r14
141614444: pop r15
141614446: pop rbp
141614447: ret 
141614448: nop dword ptr [rax + rax]
141614450: mov qword ptr [rsp + 0x10], rdx
141614455: push rbp
141614456: push r15
141614458: push r14
14161445a: push r12
14161445c: push rsi
14161445d: push rdi
14161445e: push rbx
14161445f: sub rsp, 0x40
141614463: lea rbp, [rdx + 0x80]
14161446a: test byte ptr [rbp + 0x97], 1
141614471: je 0x14161448a
141614473: mov rcx, qword ptr [rbp + 0x88]
14161447a: mov edx, 0x6800
14161447f: mov r8d, 4
141614485: call 0x140613c20
14161448a: nop 
14161448b: add rsp, 0x40
14161448f: pop rbx
141614490: pop rdi
141614491: pop rsi
141614492: pop r12
141614494: pop r14
141614496: pop r15
141614498: pop rbp
141614499: ret 
