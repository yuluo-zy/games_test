0x1404e8010: push rbp
0x1404e8011: push r15
0x1404e8013: push r14
0x1404e8015: push r13
0x1404e8017: push r12
0x1404e8019: push rsi
0x1404e801a: push rdi
0x1404e801b: push rbx
0x1404e801c: mov eax, 0x1208
0x1404e8021: call 0x14289aa40
0x1404e8026: sub rsp, rax
0x1404e8029: lea rbp, [rsp + 0x80]
0x1404e8031: movaps xmmword ptr [rbp + 0x1170], xmm11
0x1404e8039: movaps xmmword ptr [rbp + 0x1160], xmm10
0x1404e8041: movaps xmmword ptr [rbp + 0x1150], xmm9
0x1404e8049: movaps xmmword ptr [rbp + 0x1140], xmm8
0x1404e8051: movaps xmmword ptr [rbp + 0x1130], xmm7
0x1404e8058: movaps xmmword ptr [rbp + 0x1120], xmm6
0x1404e805f: mov qword ptr [rbp + 0x1118], 0xfffffffffffffffe
0x1404e806a: mov qword ptr [rbp + 0x10e0], r9
0x1404e8071: mov qword ptr [rbp + 0x10a8], r8
0x1404e8078: mov rsi, rdx
0x1404e807b: mov r12, rcx
0x1404e807e: call 0x140c16a00
0x1404e8083: test al, al
0x1404e8085: mov qword ptr [rbp + 0x10d8], rsi
0x1404e808c: je 0x1404e80ea
0x1404e808e: mov esi, 0x1f
0x1404e8093: mov ecx, 0x1f
0x1404e8098: call 0x1404e7b60
0x1404e809d: lea rbx, [rip + 0x2503850]
0x1404e80a4: cmp rax, 1
0x1404e80a8: jne 0x1404e8155
0x1404e80ae: mov r9, rdx
0x1404e80b1: test rdx, rdx
0x1404e80b4: je 0x1404e80f3
0x1404e80b6: cmp r9, 0x1f
0x1404e80ba: jae 0x1404e80f1
0x1404e80bc: lea rax, [rip + 0x2503831]
0x1404e80c3: cmp byte ptr [r9 + rax], 0xbf
0x1404e80c8: jg 0x1404e80f3
0x1404e80ca: lea rax, [rip + 0x25038c7]
0x1404e80d1: mov qword ptr [rsp + 0x20], rax
0x1404e80d6: lea rcx, [rip + 0x2503817]
0x1404e80dd: mov edx, 0x1f
0x1404e80e2: xor r8d, r8d
0x1404e80e5: call 0x1428d9940
0x1404e80ea: xor eax, eax
0x1404e80ec: jmp 0x1404e82b8
0x1404e80f1: jne 0x1404e80ca
0x1404e80f3: mov rcx, r9
0x1404e80f6: call 0x1404e7b60
0x1404e80fb: cmp rax, 1
0x1404e80ff: jne 0x1404e8155
0x1404e8101: mov r8, rdx
0x1404e8104: add r8, 2
0x1404e8108: je 0x1404e8143
0x1404e810a: cmp r8, 0x1f
0x1404e810e: jae 0x1404e8141
0x1404e8110: lea rax, [rip + 0x25037dd]
0x1404e8117: cmp byte ptr [r8 + rax], 0xbf
0x1404e811c: jg 0x1404e8143
0x1404e811e: lea rax, [rip + 0x250388b]
0x1404e8125: mov qword ptr [rsp + 0x20], rax
0x1404e812a: lea rcx, [rip + 0x25037c3]
0x1404e8131: mov edx, 0x1f
0x1404e8136: mov r9d, 0x1f
0x1404e813c: call 0x1428d9940
0x1404e8141: jne 0x1404e811e
0x1404e8143: mov esi, 0x1d
0x1404e8148: sub rsi, rdx
0x1404e814b: lea rbx, [rip + 0x25037a2]
0x1404e8152: add rbx, r8
0x1404e8155: lea r8, [rip + 0x25039f8]
0x1404e815c: lea r15, [rip + 0x25039cd]
0x1404e8163: nop word ptr cs:[rax + rax]
0x1404e8170: movsx eax, byte ptr [r8 - 1]
0x1404e8175: test eax, eax
0x1404e8177: js 0x1404e8190
0x1404e8179: dec r8
0x1404e817c: cmp eax, 0x5c
0x1404e817f: jne 0x1404e81e4
0x1404e8181: jmp 0x1404e81f5
0x1404e8183: nop word ptr cs:[rax + rax]
0x1404e8190: movzx ecx, byte ptr [r8 - 2]
0x1404e8195: cmp cl, 0xc0
0x1404e8198: jge 0x1404e81bd
0x1404e819a: movzx edx, byte ptr [r8 - 3]
0x1404e819f: cmp dl, 0xc0
0x1404e81a2: jge 0x1404e81c6
0x1404e81a4: movzx r9d, byte ptr [r8 - 4]
0x1404e81a9: add r8, -4
0x1404e81ad: and r9d, 7
0x1404e81b1: shl r9d, 6
0x1404e81b5: and edx, 0x3f
0x1404e81b8: or edx, r9d
0x1404e81bb: jmp 0x1404e81cd
0x1404e81bd: add r8, -2
0x1404e81c1: and ecx, 0x1f
0x1404e81c4: jmp 0x1404e81d5
0x1404e81c6: add r8, -3
0x1404e81ca: and edx, 0xf
0x1404e81cd: shl edx, 6
0x1404e81d0: and ecx, 0x3f
0x1404e81d3: or ecx, edx
0x1404e81d5: shl ecx, 6
0x1404e81d8: and al, 0x3f
0x1404e81da: movzx eax, al
0x1404e81dd: or eax, ecx
0x1404e81df: cmp eax, 0x5c
0x1404e81e2: je 0x1404e81f5
0x1404e81e4: cmp eax, 0x2f
0x1404e81e7: je 0x1404e81f5
0x1404e81e9: cmp r8, r15
0x1404e81ec: jne 0x1404e8170
0x1404e81ee: mov edi, 0x24
0x1404e81f3: jmp 0x1404e8241
0x1404e81f5: lea r15, [rip + 0x2503934]
0x1404e81fc: sub r8, r15
0x1404e81ff: inc r8
0x1404e8202: je 0x1404e8236
0x1404e8204: cmp r8, 0x24
0x1404e8208: jae 0x1404e8234
0x1404e820a: cmp byte ptr [r8 + r15], 0xbf
0x1404e820f: jg 0x1404e8236
0x1404e8211: lea rax, [rip + 0x2503760]
0x1404e8218: mov qword ptr [rsp + 0x20], rax
0x1404e821d: lea rcx, [rip + 0x250390c]
0x1404e8224: mov edx, 0x24
0x1404e8229: mov r9d, 0x24
0x1404e822f: call 0x1428d9940
0x1404e8234: jne 0x1404e8211
0x1404e8236: mov edi, 0x24
0x1404e823b: sub rdi, r8
0x1404e823e: add r15, r8
0x1404e8241: call 0x140c18130
0x1404e8246: mov rcx, rax
0x1404e8249: mov rax, qword ptr [rax]
0x1404e824c: cmp rax, 1
0x1404e8250: jne 0x1404ea582
0x1404e8256: add rcx, 8
0x1404e825a: cmp qword ptr [rcx], 0
0x1404e825e: jne 0x1404ea5a6
0x1404e8264: mov qword ptr [rcx], 0xffffffffffffffff
0x1404e826b: mov qword ptr [rbp + 0x1100], rcx
0x1404e8272: add rcx, 8
0x1404e8276: mov qword ptr [rsp + 0x20], rdi
0x1404e827b: mov qword ptr [rsp + 0x30], 0
0x1404e8284: mov qword ptr [rsp + 0x28], 1
0x1404e828d: mov rdx, rbx
0x1404e8290: mov r8, rsi
0x1404e8293: mov r9, r15
0x1404e8296: call 0x140c16d40
0x1404e829b: mov rcx, qword ptr [rbp + 0x1100]
0x1404e82a2: inc qword ptr [rcx]
0x1404e82a5: mov qword ptr [rbp + 0x1088], rax
0x1404e82ac: mov eax, 1
0x1404e82b1: mov rsi, qword ptr [rbp + 0x10d8]
0x1404e82b8: mov qword ptr [rbp + 0x1080], rax
0x1404e82bf: lea rbx, [rsi + 0x38]
0x1404e82c3: movabs rdx, 0x2ed189581b92decf
0x1404e82cd: movabs r8, 0x15d3d99396065f38
0x1404e82d7: mov rcx, rbx
0x1404e82da: call 0x1400dd6f0
0x1404e82df: test rax, rax
0x1404e82e2: je 0x1404ea5b2
0x1404e82e8: cmp qword ptr [rsi + 0x218], rdx
0x1404e82ef: jbe 0x1404ea5b2
0x1404e82f5: mov rax, qword ptr [rsi + 0x210]
0x1404e82fc: mov rcx, qword ptr [rax + rdx*8]
0x1404e8300: test rcx, rcx
0x1404e8303: je 0x1404ea5b2
0x1404e8309: mov rax, qword ptr [rsi + 0x1e0]
0x1404e8310: not rcx
0x1404e8313: lea rcx, [rcx + rcx*2]
0x1404e8317: shl rcx, 5
0x1404e831b: cmp qword ptr [rax + rcx + 0x20], 0
0x1404e8321: je 0x1404ea5b2
0x1404e8327: add rax, rcx
0x1404e832a: mov rax, qword ptr [rax + 0x10]
0x1404e832e: mov rdi, qword ptr [rax]
0x1404e8331: lock inc qword ptr [rdi]
0x1404e8335: jle 0x1404ea7cd
0x1404e833b: mov qword ptr [rbp + 0x10a0], rdi
0x1404e8342: movabs rdx, 0x3d7b56e787d47c40
0x1404e834c: movabs r8, 0x67b0b10a8d43bfc7
0x1404e8356: mov rcx, rbx
0x1404e8359: call 0x1400dd6f0
0x1404e835e: test rax, rax
0x1404e8361: je 0x1404ea5c3
0x1404e8367: cmp qword ptr [rsi + 0x218], rdx
0x1404e836e: jbe 0x1404ea5c3
0x1404e8374: mov rax, qword ptr [rsi + 0x210]
0x1404e837b: mov rcx, qword ptr [rax + rdx*8]
0x1404e837f: test rcx, rcx
0x1404e8382: je 0x1404ea5c3
0x1404e8388: mov rax, qword ptr [rsi + 0x1e0]
0x1404e838f: not rcx
0x1404e8392: lea rcx, [rcx + rcx*2]
0x1404e8396: shl rcx, 5
0x1404e839a: cmp qword ptr [rax + rcx + 0x20], 0
0x1404e83a0: je 0x1404ea5c3
0x1404e83a6: add rax, rcx
0x1404e83a9: mov rsi, qword ptr [rax + 0x10]
0x1404e83ad: xor r15d, r15d
0x1404e83b0: cmp dword ptr [rsi + 0x68], 1
0x1404e83b4: jne 0x1404e83c9
0x1404e83b6: movupd xmm0, xmmword ptr [rsi + 0x6c]
0x1404e83bb: movapd xmmword ptr [rbp + 0xb00], xmm0
0x1404e83c3: mov r15d, 1
0x1404e83c9: mov eax, dword ptr [rsi + 0x94]
0x1404e83cf: mov dword ptr [rbp + 0x10fc], eax
0x1404e83d5: movzx r14d, byte ptr [rsi + 0x7c]
0x1404e83da: movzx r13d, byte ptr [rsi + 0x7d]
0x1404e83df: mov rax, qword ptr [rsi + 0x60]
0x1404e83e3: mov qword ptr [rbp + 0x10b8], rax
0x1404e83ea: mov rcx, qword ptr [rsi + 0x38]
0x1404e83ee: mov rax, rcx
0x1404e83f1: neg rax
0x1404e83f4: movabs rax, 0x8000000000000001
0x1404e83fe: jo 0x1404e8425
0x1404e8400: cmp rcx, rax
0x1404e8403: jne 0x1404e840e
0x1404e8405: mov qword ptr [rbp + 0x1100], rcx
0x1404e840c: jmp 0x1404e844c
0x1404e840e: lea rdx, [rsi + 0x38]
0x1404e8412: lea rcx, [rbp + 0xd20]
0x1404e8419: call 0x140d7bd90
0x1404e841e: mov rcx, qword ptr [rbp + 0xd20]
0x1404e8425: mov qword ptr [rbp + 0x1100], rcx
0x1404e842c: movups xmm6, xmmword ptr [rsi + 0x50]
0x1404e8430: mov rax, qword ptr [rbp + 0xd28]
0x1404e8437: mov qword ptr [rbp + 0x10f0], rax
0x1404e843e: mov rax, qword ptr [rbp + 0xd30]
0x1404e8445: mov qword ptr [rbp + 0xaf0], rax
0x1404e844c: movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8454: movapd xmmword ptr [rbp + 0x90], xmm0
0x1404e845c: mov eax, dword ptr [rsi + 0x80]
0x1404e8462: mov dword ptr [rbp + 0x10c8], eax
0x1404e8468: movss xmm7, dword ptr [rsi + 0x84]
0x1404e8470: mov eax, dword ptr [rsi + 0x88]
0x1404e8476: mov dword ptr [rbp + 0x10d0], eax
0x1404e847c: movss xmm8, dword ptr [rsi + 0x8c]
0x1404e8485: movsd xmm9, qword ptr [rsi + 0xa8]
0x1404e848e: movzx eax, byte ptr [rsi + 0xc3]
0x1404e8495: mov byte ptr [rbp + 0x10c0], al
0x1404e849b: movups xmm10, xmmword ptr [rsi + 0x98]
0x1404e84a3: lea r8, [rip + 0x2503596]
0x1404e84aa: lea rcx, [rbp + 0x310]
0x1404e84b1: mov rdx, rsi
0x1404e84b4: call 0x1402b8400
0x1404e84b9: mov qword ptr [rbp + 0xaf8], r12
0x1404e84c0: cmp byte ptr [rsi + 0xac], 1
0x1404e84c7: mov qword ptr [rbp + 0x1090], rdi
0x1404e84ce: mov qword ptr [rbp + 0x1098], rbx
0x1404e84d5: jne 0x1404e84f1
0x1404e84d7: movupd xmm0, xmmword ptr [rsi + 0xad]
0x1404e84df: movapd xmmword ptr [rbp + 0x530], xmm0
0x1404e84e7: mov al, 1
0x1404e84e9: mov dword ptr [rbp + 0x10e8], eax
0x1404e84ef: jmp 0x1404e84fb
0x1404e84f1: mov dword ptr [rbp + 0x10e8], 0
0x1404e84fb: movss xmm11, dword ptr [rsi + 0xbd]
0x1404e8504: mov r12b, byte ptr [rsi + 0xc1]
0x1404e850b: mov dil, byte ptr [rsi + 0xc2]
0x1404e8512: lea rdx, [rsi + 0x18]
0x1404e8516: mov ebx, dword ptr [rsi + 0x30]
0x1404e8519: lea r8, [rip + 0x2503588]
0x1404e8520: lea rcx, [rbp + 0xb00]
0x1404e8527: call 0x1402b7890
0x1404e852c: mov dword ptr [rbp + 0xb18], ebx
0x1404e8532: mov eax, dword ptr [rsi + 0x90]
0x1404e8538: mov ecx, dword ptr [rbp + 0x10fc]
0x1404e853e: mov dword ptr [rbp + 0xdb4], ecx
0x1404e8544: mov rcx, qword ptr [rbp + 0x1100]
0x1404e854b: mov qword ptr [rbp + 0xd58], rcx
0x1404e8552: mov rcx, qword ptr [rbp + 0x10f0]
0x1404e8559: mov qword ptr [rbp + 0xd60], rcx
0x1404e8560: mov rcx, qword ptr [rbp + 0xaf0]
0x1404e8567: mov qword ptr [rbp + 0xd68], rcx
0x1404e856e: movaps xmmword ptr [rbp + 0xd70], xmm6
0x1404e8575: mov rcx, qword ptr [rbp + 0x10b8]
0x1404e857c: mov qword ptr [rbp + 0xd80], rcx
0x1404e8583: mov dword ptr [rbp + 0xd88], r15d
0x1404e858a: movaps xmm0, xmmword ptr [rbp + 0x90]
0x1404e8591: movups xmmword ptr [rbp + 0xd8c], xmm0
0x1404e8598: mov byte ptr [rbp + 0xd9c], r14b
0x1404e859f: mov byte ptr [rbp + 0xd9d], r13b
0x1404e85a6: mov ecx, dword ptr [rbp + 0x10c8]
0x1404e85ac: mov dword ptr [rbp + 0xda0], ecx
0x1404e85b2: movss dword ptr [rbp + 0xda4], xmm7
0x1404e85ba: mov ecx, dword ptr [rbp + 0x10d0]
0x1404e85c0: mov dword ptr [rbp + 0xda8], ecx
0x1404e85c6: movss dword ptr [rbp + 0xdac], xmm8
0x1404e85cf: movss dword ptr [rbp + 0xdc8], xmm9
0x1404e85d8: movzx ecx, byte ptr [rbp + 0x10c0]
0x1404e85df: mov byte ptr [rbp + 0xde3], cl
0x1404e85e5: movups xmmword ptr [rbp + 0xdb8], xmm10
0x1404e85ed: mov rcx, qword ptr [rbp + 0x320]
0x1404e85f4: mov qword ptr [rbp + 0xd30], rcx
0x1404e85fb: movups xmm0, xmmword ptr [rbp + 0x310]
0x1404e8602: movaps xmmword ptr [rbp + 0xd20], xmm0
0x1404e8609: mov ecx, dword ptr [rbp + 0x10e8]
0x1404e860f: mov byte ptr [rbp + 0xdcc], cl
0x1404e8615: movaps xmm0, xmmword ptr [rbp + 0x530]
0x1404e861c: movups xmmword ptr [rbp + 0xdcd], xmm0
0x1404e8623: movss dword ptr [rbp + 0xddd], xmm11
0x1404e862c: mov byte ptr [rbp + 0xde1], r12b
0x1404e8633: mov byte ptr [rbp + 0xde2], dil
0x1404e863a: movupd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8642: movups xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8649: movupd xmmword ptr [rbp + 0xd38], xmm0
0x1404e8651: movups xmmword ptr [rbp + 0xd48], xmm1
0x1404e8658: mov dword ptr [rbp + 0xdb0], eax
0x1404e865e: movups xmm8, xmmword ptr [rsi + 0xf8]
0x1404e8666: movss xmm7, dword ptr [rsi + 0xf0]
0x1404e866e: movzx ebx, byte ptr [rsi + 0xf4]
0x1404e8675: movsd xmm11, qword ptr [rsi + 0xe8]
0x1404e867e: mov rdi, qword ptr [rsi + 0xe0]
0x1404e8685: movss xmm6, dword ptr [rsi + 0x11c]
0x1404e868d: movups xmm9, xmmword ptr [rsi + 0x108]
0x1404e8695: movss xmm10, dword ptr [rsi + 0x118]
0x1404e869e: xor eax, eax
0x1404e86a0: cmp rax, qword ptr [rsi + 0xc8]
0x1404e86a7: movzx r14d, byte ptr [rsi + 0x120]
0x1404e86af: movzx r15d, byte ptr [rsi + 0x121]
0x1404e86b7: jno 0x1404e86cc
0x1404e86b9: movabs rax, 0x8000000000000000
0x1404e86c3: mov qword ptr [rbp + 0xb00], rax
0x1404e86ca: jmp 0x1404e86e2
0x1404e86cc: add rsi, 0xc8
0x1404e86d3: lea rcx, [rbp + 0xb00]
0x1404e86da: mov rdx, rsi
0x1404e86dd: call 0x140d7bd90
0x1404e86e2: lea rcx, [rbp + 0x788]
0x1404e86e9: lea rsi, [rbp + 0xd20]
0x1404e86f0: mov r8d, 0xc8
0x1404e86f6: mov rdx, rsi
0x1404e86f9: call 0x142923c00
0x1404e86fe: movups xmmword ptr [rbp + 0x880], xmm8
0x1404e8706: movlps qword ptr [rbp + 0x870], xmm11
0x1404e870e: movss dword ptr [rbp + 0x878], xmm7
0x1404e8716: mov byte ptr [rbp + 0x87c], bl
0x1404e871c: mov qword ptr [rbp + 0x868], rdi
0x1404e8723: movups xmmword ptr [rbp + 0x890], xmm9
0x1404e872b: movss dword ptr [rbp + 0x8a0], xmm10
0x1404e8734: movss dword ptr [rbp + 0x8a4], xmm6
0x1404e873c: mov byte ptr [rbp + 0x8a8], r14b
0x1404e8743: mov byte ptr [rbp + 0x8a9], r15b
0x1404e874a: movupd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8752: movupd xmmword ptr [rbp + 0x850], xmm0
0x1404e875a: mov rax, qword ptr [rbp + 0xb10]
0x1404e8761: mov qword ptr [rbp + 0x860], rax
0x1404e8768: movzx eax, byte ptr [rbp + 0x848]
0x1404e876f: mov rcx, qword ptr [rip + 0x314386a]
0x1404e8776: mov byte ptr [rcx], al
0x1404e8778: call 0x140c16a00
0x1404e877d: mov byte ptr [rbp + 0x10b8], al
0x1404e8783: mov dil, byte ptr [rbp + 0x1208]
0x1404e878a: cmp dil, 2
0x1404e878e: sete r14b
0x1404e8792: lea rcx, [rbp + 0x90]
0x1404e8799: call 0x1409ba340
0x1404e879e: mov byte ptr [rbp + 0x1117], 1
0x1404e87a5: lea rdx, [rbp + 0x90]
0x1404e87ac: mov rcx, qword ptr [rbp + 0x10d8]
0x1404e87b3: call 0x1409ba540
0x1404e87b8: mov r15, qword ptr [rbp + 0x10e0]
0x1404e87bf: mov byte ptr [rbp + 0x1117], 1
0x1404e87c6: lea rcx, [rbp + 0x90]
0x1404e87cd: call 0x1409ba4b0
0x1404e87d2: mov rcx, qword ptr [rbp + 0x11f8]
0x1404e87d9: mov rax, qword ptr [rbp + 0x1090]
0x1404e87e0: add rax, 0x10
0x1404e87e4: mov qword ptr [rbp + 0x1100], rax
0x1404e87eb: test rcx, rcx
0x1404e87ee: je 0x1404e882c
0x1404e87f0: mov byte ptr [rbp + 0x1117], 1
0x1404e87f7: call 0x140c0e7e0
0x1404e87fc: mov ebx, eax
0x1404e87fe: mov byte ptr [rbp + 0x1117], 1
0x1404e8805: mov rcx, qword ptr [rbp + 0x11f8]
0x1404e880c: mov rdx, qword ptr [rbp + 0x1100]
0x1404e8813: call 0x140c0ea90
0x1404e8818: mov dword ptr [rbp + 0x1074], ebx
0x1404e881e: mov qword ptr [rbp + 0x1078], rax
0x1404e8825: mov eax, 1
0x1404e882a: jmp 0x1404e882e
0x1404e882c: xor eax, eax
0x1404e882e: mov r13, qword ptr [rbp + 0x11f0]
0x1404e8835: mov dword ptr [rbp + 0x1070], eax
0x1404e883b: test dil, dil
0x1404e883e: mov rbx, qword ptr [rbp + 0x10d8]
0x1404e8845: je 0x1404e8962
0x1404e884b: mov rdi, qword ptr [rbp + 0x1200]
0x1404e8852: lea r12, [r15 + 0xd0]
0x1404e8859: lea rdx, [rbp + 0x90]
0x1404e8860: mov r8d, 0x1f8
0x1404e8866: mov rcx, rsi
0x1404e8869: call 0x142923c00
0x1404e886e: movupd xmm0, xmmword ptr [rbp + 0x1070]
0x1404e8876: movapd xmmword ptr [rbp + 0x530], xmm0
0x1404e887e: mov rax, qword ptr [rbp + 0xaf8]
0x1404e8885: mov qword ptr [rbp + 0x1000], rax
0x1404e888c: lea r9, [r13 + 0x18]
0x1404e8890: lea rax, [r13 + 0x78]
0x1404e8894: mov qword ptr [rbp + 0xb00], rbx
0x1404e889b: mov r8, qword ptr [rbp + 0x1100]
0x1404e88a2: mov qword ptr [rbp + 0xb08], r8
0x1404e88a9: mov qword ptr [rbp + 0x10c8], rax
0x1404e88b0: mov qword ptr [rbp + 0xb10], rax
0x1404e88b7: lea rax, [rbp + 0x1000]
0x1404e88be: mov qword ptr [rbp + 0xb18], rax
0x1404e88c5: mov qword ptr [rbp + 0xb20], rsi
0x1404e88cc: lea rax, [rbp + 0x788]
0x1404e88d3: mov qword ptr [rbp + 0xb28], rax
0x1404e88da: lea rax, [rbp + 0x530]
0x1404e88e1: mov qword ptr [rbp + 0xb30], rax
0x1404e88e8: mov qword ptr [rbp + 0xb38], rdi
0x1404e88ef: lea rax, [rbp + 0xb00]
0x1404e88f6: mov qword ptr [rsp + 0x20], rax
0x1404e88fb: lea rcx, [rbp + 0x310]
0x1404e8902: mov qword ptr [rbp + 0x10c0], r12
0x1404e8909: mov rdx, r12
0x1404e890c: mov r12, r9
0x1404e890f: call 0x140161cd0
0x1404e8914: mov byte ptr [rbp + 0x1117], 0
0x1404e891b: lea rcx, [rbp + 0xd20]
0x1404e8922: call 0x140609f60
0x1404e8927: xor eax, eax
0x1404e8929: cmp rax, qword ptr [rbp + 0x310]
0x1404e8930: setno byte ptr [rbp + 0x10fc]
0x1404e8937: jo 0x1404e8a3d
0x1404e893d: lea rcx, [rbp + 0x530]
0x1404e8944: lea rdx, [rbp + 0x310]
0x1404e894b: mov r8d, 0x218
0x1404e8951: call 0x142923c00
0x1404e8956: mov r12d, dword ptr [rbp + 0x528]
0x1404e895d: jmp 0x1404e9536
0x1404e8962: test eax, eax
0x1404e8964: je 0x1404ea617
0x1404e896a: mov rdi, qword ptr [rbp + 0x1078]
0x1404e8971: lea rsi, [r15 + 0xd0]
0x1404e8978: lea rdx, [rbp + 0x90]
0x1404e897f: mov r8d, 0x1f8
0x1404e8985: lea r12, [rbp + 0xb00]
0x1404e898c: mov rcx, r12
0x1404e898f: call 0x142923c00
0x1404e8994: mov qword ptr [rbp + 0x530], rdi
0x1404e899b: lea r9, [r13 + 0x18]
0x1404e899f: lea rax, [r13 + 0x78]
0x1404e89a3: mov qword ptr [rbp + 0x310], rbx
0x1404e89aa: mov r8, qword ptr [rbp + 0x1100]
0x1404e89b1: mov qword ptr [rbp + 0x318], r8
0x1404e89b8: mov qword ptr [rbp + 0x320], rax
0x1404e89bf: lea rax, [rbp + 0x530]
0x1404e89c6: mov qword ptr [rbp + 0x328], rax
0x1404e89cd: mov qword ptr [rbp + 0x330], r12
0x1404e89d4: lea rax, [rbp + 0x310]
0x1404e89db: mov qword ptr [rsp + 0x20], rax
0x1404e89e0: lea rcx, [rbp + 0xd20]
0x1404e89e7: mov rdx, rsi
0x1404e89ea: call 0x1401647b0
0x1404e89ef: mov byte ptr [rbp + 0x1117], 0
0x1404e89f6: lea rcx, [rbp + 0xb00]
0x1404e89fd: call 0x140609f60
0x1404e8a02: xor eax, eax
0x1404e8a04: cmp rax, qword ptr [rbp + 0xd20]
0x1404e8a0b: jo 0x1404ea63b
0x1404e8a11: lea rcx, [rbp + 0x530]
0x1404e8a18: lea rdx, [rbp + 0xd20]
0x1404e8a1f: mov r8d, 0x218
0x1404e8a25: call 0x142923c00
0x1404e8a2a: mov r12d, dword ptr [rbp + 0xf38]
0x1404e8a31: mov byte ptr [rbp + 0x10fc], 1
0x1404e8a38: jmp 0x1404e9536
0x1404e8a3d: mov byte ptr [rbp + 0x1117], 0
0x1404e8a44: movabs rdx, 0xa4c02661e67b048b
0x1404e8a4e: movabs r8, 0xeae761e6aa2a67ec
0x1404e8a58: mov rcx, qword ptr [rbp + 0x1098]
0x1404e8a5f: call 0x1400dd6f0
0x1404e8a64: test rax, rax
0x1404e8a67: mov rcx, qword ptr [rbp + 0x10d8]
0x1404e8a6e: je 0x1404ea5e0
0x1404e8a74: cmp qword ptr [rcx + 0x218], rdx
0x1404e8a7b: jbe 0x1404ea5e0
0x1404e8a81: mov rax, qword ptr [rcx + 0x210]
0x1404e8a88: mov rax, qword ptr [rax + rdx*8]
0x1404e8a8c: test rax, rax
0x1404e8a8f: je 0x1404ea5e0
0x1404e8a95: mov rsi, qword ptr [rcx + 0x1e0]
0x1404e8a9c: not rax
0x1404e8a9f: lea rax, [rax + rax*2]
0x1404e8aa3: shl rax, 5
0x1404e8aa7: cmp qword ptr [rsi + rax + 0x20], 0
0x1404e8aad: je 0x1404ea5e0
0x1404e8ab3: add rsi, rax
0x1404e8ab6: mov rdi, qword ptr [rsi + 0x10]
0x1404e8aba: mov ebx, dword ptr [rcx + 0x5b0]
0x1404e8ac0: movss xmm0, dword ptr [rdi + 0x18]
0x1404e8ac5: add rdi, 0x18
0x1404e8ac9: addss xmm0, dword ptr [rip + 0x2502c27]
0x1404e8ad1: movss dword ptr [rbp + 0x10b4], xmm0
0x1404e8ad9: mov rax, qword ptr [rip + 0x31438c8]
0x1404e8ae0: mov rax, qword ptr [rax]
0x1404e8ae3: test rax, rax
0x1404e8ae6: je 0x1404e8c93
0x1404e8aec: mov qword ptr [rbp + 0x530], rdi
0x1404e8af3: lea rax, [rip + 0x8a3e16]
0x1404e8afa: mov qword ptr [rbp + 0x538], rax
0x1404e8b01: lea rcx, [rbp + 0x10b4]
0x1404e8b08: mov qword ptr [rbp + 0x540], rcx
0x1404e8b0f: mov qword ptr [rbp + 0x548], rax
0x1404e8b16: mov qword ptr [rbp + 0xb00], 0
0x1404e8b21: mov qword ptr [rbp + 0xb08], 1
0x1404e8b2c: mov qword ptr [rbp + 0xb10], 0
0x1404e8b37: mov qword ptr [rbp + 0xb18], 1
0x1404e8b42: mov qword ptr [rbp + 0xb20], 0
0x1404e8b4d: mov qword ptr [rbp + 0xb28], 0x20
0x1404e8b58: mov byte ptr [rbp + 0xb30], 3
0x1404e8b5f: mov qword ptr [rbp + 0xb38], 0
0x1404e8b6a: mov qword ptr [rbp + 0xb40], 1
0x1404e8b75: mov qword ptr [rbp + 0xb48], 0
0x1404e8b80: mov qword ptr [rbp + 0xb50], 1
0x1404e8b8b: mov qword ptr [rbp + 0xb58], 1
0x1404e8b96: mov qword ptr [rbp + 0xb60], 0x20
0x1404e8ba1: mov byte ptr [rbp + 0xb68], 3
0x1404e8ba8: mov byte ptr [rbp + 0x1117], 0
0x1404e8baf: lea rcx, [rip + 0x250311a]
0x1404e8bb6: call 0x14067dd80
0x1404e8bbb: movupd xmm0, xmmword ptr [rax]
0x1404e8bbf: mov eax, dword ptr [rax + 0x10]
0x1404e8bc2: mov qword ptr [rbp + 0xd50], 1
0x1404e8bcd: lea rcx, [rip + 0x2502f9c]
0x1404e8bd4: mov qword ptr [rbp + 0xd58], rcx
0x1404e8bdb: mov qword ptr [rbp + 0xd60], 0x17
0x1404e8be6: lea rdx, [rip + 0x25030c3]
0x1404e8bed: mov qword ptr [rbp + 0xd70], rdx
0x1404e8bf4: mov qword ptr [rbp + 0xd78], 2
0x1404e8bff: lea rdx, [rbp + 0x530]
0x1404e8c06: mov qword ptr [rbp + 0xd80], rdx
0x1404e8c0d: mov qword ptr [rbp + 0xd88], 2
0x1404e8c18: lea rdx, [rbp + 0xb00]
0x1404e8c1f: mov qword ptr [rbp + 0xd90], rdx
0x1404e8c26: mov qword ptr [rbp + 0xd98], 2
0x1404e8c31: mov qword ptr [rbp + 0xd20], 0
0x1404e8c3c: mov qword ptr [rbp + 0xd28], rcx
0x1404e8c43: mov qword ptr [rbp + 0xd30], 0x17
0x1404e8c4e: mov qword ptr [rbp + 0xd38], 0
0x1404e8c59: movupd xmmword ptr [rbp + 0xd40], xmm0
0x1404e8c61: mov dword ptr [rbp + 0xd68], 1
0x1404e8c6b: mov dword ptr [rbp + 0xd6c], eax
0x1404e8c71: mov byte ptr [rbp + 0x1117], 0
0x1404e8c78: lea rcx, [rbp + 0x1000]
0x1404e8c7f: lea rdx, [rbp + 0xd20]
0x1404e8c86: call 0x140d4a330
0x1404e8c8b: movss xmm0, dword ptr [rbp + 0x10b4]
0x1404e8c93: xorps xmm1, xmm1
0x1404e8c96: ucomiss xmm1, xmm0
0x1404e8c99: jae 0x1404ea672
0x1404e8c9f: mov dword ptr [rsi + 0x5c], ebx
0x1404e8ca2: movss dword ptr [rdi], xmm0
0x1404e8ca6: mov byte ptr [rbp + 0x1117], 0
0x1404e8cad: mov rcx, r12
0x1404e8cb0: call 0x140be76f0
0x1404e8cb5: call 0x140d4d740
0x1404e8cba: mov rsi, rax
0x1404e8cbd: test byte ptr [rax], 1
0x1404e8cc0: je 0x1404e8cd3
0x1404e8cc2: mov rcx, rsi
0x1404e8cc5: add rcx, 8
0x1404e8cc9: mov rax, qword ptr [rsi + 8]
0x1404e8ccd: mov rdx, qword ptr [rsi + 0x10]
0x1404e8cd1: jmp 0x1404e8cf5
0x1404e8cd3: mov byte ptr [rbp + 0x1117], 0
0x1404e8cda: call 0x140d64e00
0x1404e8cdf: mov rcx, rsi
0x1404e8ce2: add rcx, 8
0x1404e8ce6: mov qword ptr [rsi], 1
0x1404e8ced: mov qword ptr [rsi + 8], rax
0x1404e8cf1: mov qword ptr [rsi + 0x10], rdx
0x1404e8cf5: lea r8, [rax + 1]
0x1404e8cf9: mov qword ptr [rcx], r8
0x1404e8cfc: movups xmm6, xmmword ptr [rip + 0x2502ded]
0x1404e8d03: movaps xmmword ptr [rbp + 0xb10], xmm6
0x1404e8d0a: movups xmm7, xmmword ptr [rip + 0x2502dcf]
0x1404e8d11: movaps xmmword ptr [rbp + 0xb00], xmm7
0x1404e8d18: mov qword ptr [rbp + 0xb20], rax
0x1404e8d1f: mov qword ptr [rbp + 0xb28], rdx
0x1404e8d26: call 0x140d4d740
0x1404e8d2b: mov rsi, rax
0x1404e8d2e: test byte ptr [rax], 1
0x1404e8d31: je 0x1404e8d44
0x1404e8d33: mov rax, rsi
0x1404e8d36: add rax, 8
0x1404e8d3a: mov rbx, qword ptr [rsi + 8]
0x1404e8d3e: mov rdi, qword ptr [rsi + 0x10]
0x1404e8d42: jmp 0x1404e8d65
0x1404e8d44: call 0x140d64e00
0x1404e8d49: mov rbx, rax
0x1404e8d4c: mov rdi, rdx
0x1404e8d4f: mov rax, rsi
0x1404e8d52: add rax, 8
0x1404e8d56: mov qword ptr [rsi], 1
0x1404e8d5d: mov qword ptr [rsi + 8], rbx
0x1404e8d61: mov qword ptr [rsi + 0x10], rdx
0x1404e8d65: lea rcx, [rbx + 1]
0x1404e8d69: mov qword ptr [rax], rcx
0x1404e8d6c: movaps xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8d73: movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8d7a: movaps xmm2, xmmword ptr [rbp + 0xb20]
0x1404e8d81: movaps xmmword ptr [rbp + 0xd40], xmm2
0x1404e8d88: movaps xmmword ptr [rbp + 0xb30], xmm7
0x1404e8d8f: movaps xmmword ptr [rbp + 0xb40], xmm6
0x1404e8d96: movaps xmmword ptr [rbp + 0xb00], xmm0
0x1404e8d9d: movaps xmmword ptr [rbp + 0xb10], xmm1
0x1404e8da4: movaps xmmword ptr [rbp + 0xb20], xmm2
0x1404e8dab: lea rsi, [r15 + 0x160]
0x1404e8db2: cmp qword ptr [r15 + 0x160], 0
0x1404e8dba: je 0x1404e8e44
0x1404e8dc0: mov qword ptr [rbp + 0x10e8], rbx
0x1404e8dc7: mov qword ptr [rbp + 0x10d0], rdi
0x1404e8dce: lea rdx, [r15 + 0x180]
0x1404e8dd5: mov r8d, 0x68
0x1404e8ddb: mov r9d, 0x10
0x1404e8de1: mov qword ptr [rbp + 0x10f0], rsi
0x1404e8de8: mov rcx, rsi
0x1404e8deb: call 0x14004ceb0
0x1404e8df0: mov r15, qword ptr [rbp + 0x10e0]
0x1404e8df7: mov rdx, qword ptr [r15 + 0x198]
0x1404e8dfe: test rdx, rdx
0x1404e8e01: mov rdi, qword ptr [rbp + 0x10d0]
0x1404e8e08: mov rbx, qword ptr [rbp + 0x10e8]
0x1404e8e0f: mov rsi, qword ptr [rbp + 0x10f0]
0x1404e8e16: je 0x1404e8e44
0x1404e8e18: mov rax, rdx
0x1404e8e1b: shl rax, 4
0x1404e8e1f: add rdx, rax
0x1404e8e22: add rdx, 0x21
0x1404e8e26: je 0x1404e8e44
0x1404e8e28: mov rcx, qword ptr [r15 + 0x190]
0x1404e8e2f: neg rax
0x1404e8e32: add rcx, rax
0x1404e8e35: add rcx, -0x10
0x1404e8e39: mov r8d, 0x10
0x1404e8e3f: call 0x140613c20
0x1404e8e44: movaps xmm0, xmmword ptr [rbp + 0xb40]
0x1404e8e4b: movups xmmword ptr [rsi + 0x40], xmm0
0x1404e8e4f: movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8e57: movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8e5e: movapd xmm2, xmmword ptr [rbp + 0xb20]
0x1404e8e66: movapd xmm3, xmmword ptr [rbp + 0xb30]
0x1404e8e6e: movupd xmmword ptr [rsi + 0x30], xmm3
0x1404e8e73: movupd xmmword ptr [rsi + 0x20], xmm2
0x1404e8e78: movups xmmword ptr [rsi + 0x10], xmm1
0x1404e8e7c: movupd xmmword ptr [rsi], xmm0
0x1404e8e80: mov qword ptr [r15 + 0x1b0], rbx
0x1404e8e87: mov qword ptr [r15 + 0x1b8], rdi
0x1404e8e8e: call 0x140d4d740
0x1404e8e93: mov rsi, rax
0x1404e8e96: test byte ptr [rax], 1
0x1404e8e99: je 0x1404e8eac
0x1404e8e9b: mov rcx, rsi
0x1404e8e9e: add rcx, 8
0x1404e8ea2: mov rax, qword ptr [rsi + 8]
0x1404e8ea6: mov rdx, qword ptr [rsi + 0x10]
0x1404e8eaa: jmp 0x1404e8ece
0x1404e8eac: mov byte ptr [rbp + 0x1117], 0
0x1404e8eb3: call 0x140d64e00
0x1404e8eb8: mov rcx, rsi
0x1404e8ebb: add rcx, 8
0x1404e8ebf: mov qword ptr [rsi], 1
0x1404e8ec6: mov qword ptr [rsi + 8], rax
0x1404e8eca: mov qword ptr [rsi + 0x10], rdx
0x1404e8ece: lea r8, [rax + 1]
0x1404e8ed2: mov qword ptr [rcx], r8
0x1404e8ed5: movaps xmmword ptr [rbp + 0xb10], xmm6
0x1404e8edc: movaps xmmword ptr [rbp + 0xb00], xmm7
0x1404e8ee3: mov qword ptr [rbp + 0xb20], rax
0x1404e8eea: mov qword ptr [rbp + 0xb28], rdx
0x1404e8ef1: call 0x140d4d740
0x1404e8ef6: mov rsi, rax
0x1404e8ef9: test byte ptr [rax], 1
0x1404e8efc: je 0x1404e8f0f
0x1404e8efe: mov rax, rsi
0x1404e8f01: add rax, 8
0x1404e8f05: mov rbx, qword ptr [rsi + 8]
0x1404e8f09: mov rdi, qword ptr [rsi + 0x10]
0x1404e8f0d: jmp 0x1404e8f30
0x1404e8f0f: call 0x140d64e00
0x1404e8f14: mov rbx, rax
0x1404e8f17: mov rdi, rdx
0x1404e8f1a: mov rax, rsi
0x1404e8f1d: add rax, 8
0x1404e8f21: mov qword ptr [rsi], 1
0x1404e8f28: mov qword ptr [rsi + 8], rbx
0x1404e8f2c: mov qword ptr [rsi + 0x10], rdx
0x1404e8f30: lea rcx, [rbx + 1]
0x1404e8f34: mov qword ptr [rax], rcx
0x1404e8f37: movaps xmm0, xmmword ptr [rbp + 0xb00]
0x1404e8f3e: movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e8f45: movaps xmm2, xmmword ptr [rbp + 0xb20]
0x1404e8f4c: movaps xmmword ptr [rbp + 0xd40], xmm2
0x1404e8f53: movaps xmmword ptr [rbp + 0xb30], xmm7
0x1404e8f5a: movaps xmmword ptr [rbp + 0xb40], xmm6
0x1404e8f61: movaps xmmword ptr [rbp + 0xb00], xmm0
0x1404e8f68: movaps xmmword ptr [rbp + 0xb10], xmm1
0x1404e8f6f: movaps xmmword ptr [rbp + 0xb20], xmm2
0x1404e8f76: lea rsi, [r15 + 0x250]
0x1404e8f7d: cmp qword ptr [r15 + 0x250], 0
0x1404e8f85: je 0x1404e900f
0x1404e8f8b: mov qword ptr [rbp + 0x10e8], rbx
0x1404e8f92: mov qword ptr [rbp + 0x10d0], rdi
0x1404e8f99: lea rdx, [r15 + 0x270]
0x1404e8fa0: mov r8d, 0x68
0x1404e8fa6: mov r9d, 0x10
0x1404e8fac: mov qword ptr [rbp + 0x10f0], rsi
0x1404e8fb3: mov rcx, rsi
0x1404e8fb6: call 0x14004ceb0
0x1404e8fbb: mov r15, qword ptr [rbp + 0x10e0]
0x1404e8fc2: mov rdx, qword ptr [r15 + 0x288]
0x1404e8fc9: test rdx, rdx
0x1404e8fcc: mov rdi, qword ptr [rbp + 0x10d0]
0x1404e8fd3: mov rbx, qword ptr [rbp + 0x10e8]
0x1404e8fda: mov rsi, qword ptr [rbp + 0x10f0]
0x1404e8fe1: je 0x1404e900f
0x1404e8fe3: mov rax, rdx
0x1404e8fe6: shl rax, 4
0x1404e8fea: add rdx, rax
0x1404e8fed: add rdx, 0x21
0x1404e8ff1: je 0x1404e900f
0x1404e8ff3: mov rcx, qword ptr [r15 + 0x280]
0x1404e8ffa: neg rax
0x1404e8ffd: add rcx, rax
0x1404e9000: add rcx, -0x10
0x1404e9004: mov r8d, 0x10
0x1404e900a: call 0x140613c20
0x1404e900f: movaps xmm0, xmmword ptr [rbp + 0xb40]
0x1404e9016: movups xmmword ptr [rsi + 0x40], xmm0
0x1404e901a: movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404e9022: movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404e9029: movapd xmm2, xmmword ptr [rbp + 0xb20]
0x1404e9031: movapd xmm3, xmmword ptr [rbp + 0xb30]
0x1404e9039: movupd xmmword ptr [rsi + 0x30], xmm3
0x1404e903e: movupd xmmword ptr [rsi + 0x20], xmm2
0x1404e9043: movups xmmword ptr [rsi + 0x10], xmm1
0x1404e9047: movupd xmmword ptr [rsi], xmm0
0x1404e904b: mov qword ptr [r15 + 0x2a0], rbx
0x1404e9052: mov qword ptr [r15 + 0x2a8], rdi
0x1404e9059: mov byte ptr [rbp + 0x1117], 0
0x1404e9060: mov rcx, qword ptr [rbp + 0x1100]
0x1404e9067: call 0x140bf8e60
0x1404e906c: mov byte ptr [rbp + 0x1117], 0
0x1404e9073: movabs rdx, 0x3335386a5fbcee80
0x1404e907d: movabs r8, 0x8f2a59f31a65ec60
0x1404e9087: mov rcx, qword ptr [rbp + 0x1098]
0x1404e908e: call 0x1400dd6f0
0x1404e9093: test rax, rax
0x1404e9096: mov r8, qword ptr [rbp + 0x10d8]
0x1404e909d: je 0x1404e90ea
0x1404e909f: cmp qword ptr [r8 + 0x218], rdx
0x1404e90a6: jbe 0x1404e90ea
0x1404e90a8: mov rax, qword ptr [r8 + 0x210]
0x1404e90af: mov rcx, qword ptr [rax + rdx*8]
0x1404e90b3: test rcx, rcx
0x1404e90b6: je 0x1404e90ea
0x1404e90b8: mov rax, qword ptr [r8 + 0x1e0]
0x1404e90bf: not rcx
0x1404e90c2: lea rcx, [rcx + rcx*2]
0x1404e90c6: shl rcx, 5
0x1404e90ca: cmp qword ptr [rax + rcx + 0x20], 0
0x1404e90d0: je 0x1404e90ea
0x1404e90d2: add rax, rcx
0x1404e90d5: mov rcx, qword ptr [rax + 0x10]
0x1404e90d9: mov edx, dword ptr [r8 + 0x5b0]
0x1404e90e0: mov dword ptr [rax + 0x5c], edx
0x1404e90e3: mov qword ptr [rcx], 2
0x1404e90ea: mov rax, qword ptr [rip + 0x31432b7]
0x1404e90f1: mov rax, qword ptr [rax]
0x1404e90f4: cmp rax, 3
0x1404e90f8: jb 0x1404e91cb
0x1404e90fe: mov byte ptr [rbp + 0x1117], 0
0x1404e9105: lea rcx, [rip + 0x2502c2c]
0x1404e910c: call 0x14067dd80
0x1404e9111: movupd xmm0, xmmword ptr [rax]
0x1404e9115: mov eax, dword ptr [rax + 0x10]
0x1404e9118: mov qword ptr [rbp + 0xd50], 3
0x1404e9123: lea rcx, [rip + 0x2502a46]
0x1404e912a: mov qword ptr [rbp + 0xd58], rcx
0x1404e9131: mov qword ptr [rbp + 0xd60], 0x17
0x1404e913c: lea rdx, [rip + 0x2502be5]
0x1404e9143: mov qword ptr [rbp + 0xd70], rdx
0x1404e914a: mov qword ptr [rbp + 0xd78], 1
0x1404e9155: mov qword ptr [rbp + 0xd80], 8
0x1404e9160: mov qword ptr [rbp + 0xd20], 0
0x1404e916b: xorps xmm1, xmm1
0x1404e916e: movups xmmword ptr [rbp + 0xd88], xmm1
0x1404e9175: mov qword ptr [rbp + 0xd28], rcx
0x1404e917c: mov qword ptr [rbp + 0xd30], 0x17
0x1404e9187: mov qword ptr [rbp + 0xd38], 0
0x1404e9192: movupd xmmword ptr [rbp + 0xd40], xmm0
0x1404e919a: mov dword ptr [rbp + 0xd68], 1
0x1404e91a4: mov dword ptr [rbp + 0xd6c], eax
0x1404e91aa: mov byte ptr [rbp + 0x1117], 0
0x1404e91b1: lea rcx, [rbp + 0x1000]
0x1404e91b8: lea rdx, [rbp + 0xd20]
0x1404e91bf: call 0x140d4a330
0x1404e91c4: mov r8, qword ptr [rbp + 0x10d8]
0x1404e91cb: mov dword ptr [rbp + 0x1000], 0
0x1404e91d5: mov qword ptr [rbp + 0xb00], 0
0x1404e91e0: mov qword ptr [rbp + 0xb08], 0x10
0x1404e91eb: xorpd xmm0, xmm0
0x1404e91ef: movupd xmmword ptr [rbp + 0xb10], xmm0
0x1404e91f7: mov qword ptr [rbp + 0xb20], 0x10
0x1404e9202: movupd xmmword ptr [rbp + 0xb28], xmm0
0x1404e920a: mov qword ptr [rbp + 0xb38], 0x10
0x1404e9215: movupd xmmword ptr [rbp + 0xb40], xmm0
0x1404e921d: mov qword ptr [rbp + 0xb50], 0x10
0x1404e9228: movupd xmmword ptr [rbp + 0xb58], xmm0
0x1404e9230: mov qword ptr [rbp + 0xb68], 0x10
0x1404e923b: movupd xmmword ptr [rbp + 0xb70], xmm0
0x1404e9243: mov qword ptr [rbp + 0xb80], 0x10
0x1404e924e: movupd xmmword ptr [rbp + 0xb88], xmm0
0x1404e9256: mov qword ptr [rbp + 0xb98], 0x10
0x1404e9261: movupd xmmword ptr [rbp + 0xba0], xmm0
0x1404e9269: mov qword ptr [rbp + 0xbb0], 0x10
0x1404e9274: movupd xmmword ptr [rbp + 0xbb8], xmm0
0x1404e927c: mov qword ptr [rbp + 0xbc8], 0x10
0x1404e9287: movupd xmmword ptr [rbp + 0xbd0], xmm0
0x1404e928f: mov qword ptr [rbp + 0xbe0], 0x10
0x1404e929a: movupd xmmword ptr [rbp + 0xbe8], xmm0
0x1404e92a2: mov qword ptr [rbp + 0xbf8], 0x10
0x1404e92ad: movupd xmmword ptr [rbp + 0xc00], xmm0
0x1404e92b5: mov qword ptr [rbp + 0xc10], 0x10
0x1404e92c0: movupd xmmword ptr [rbp + 0xc18], xmm0
0x1404e92c8: mov qword ptr [rbp + 0xc28], 0x10
0x1404e92d3: movupd xmmword ptr [rbp + 0xc30], xmm0
0x1404e92db: mov qword ptr [rbp + 0xc40], 0x10
0x1404e92e6: movupd xmmword ptr [rbp + 0xc48], xmm0
0x1404e92ee: mov qword ptr [rbp + 0xc58], 0x10
0x1404e92f9: movupd xmmword ptr [rbp + 0xc60], xmm0
0x1404e9301: mov qword ptr [rbp + 0xc70], 0x10
0x1404e930c: movupd xmmword ptr [rbp + 0xc78], xmm0
0x1404e9314: mov qword ptr [rbp + 0xc88], 0x10
0x1404e931f: movupd xmmword ptr [rbp + 0xc90], xmm0
0x1404e9327: mov qword ptr [rbp + 0xca0], 0x10
0x1404e9332: movupd xmmword ptr [rbp + 0xca8], xmm0
0x1404e933a: mov qword ptr [rbp + 0xcb8], 0x10
0x1404e9345: movupd xmmword ptr [rbp + 0xcc0], xmm0
0x1404e934d: mov qword ptr [rbp + 0xcd0], 0x10
0x1404e9358: movupd xmmword ptr [rbp + 0xcd8], xmm0
0x1404e9360: mov qword ptr [rbp + 0xce8], 0x10
0x1404e936b: mov qword ptr [rbp + 0xcf0], 0
0x1404e9376: movabs rax, 0x2000000020
0x1404e9380: mov qword ptr [rbp + 0x9c0], rax
0x1404e9387: mov qword ptr [rbp + 0x530], r8
0x1404e938e: mov r8, qword ptr [rbp + 0x1100]
0x1404e9395: mov qword ptr [rbp + 0x538], r8
0x1404e939c: mov rax, qword ptr [rbp + 0x10c8]
0x1404e93a3: mov qword ptr [rbp + 0x540], rax
0x1404e93aa: lea rax, [rbp + 0x9c0]
0x1404e93b1: mov qword ptr [rbp + 0x548], rax
0x1404e93b8: lea rax, [rbp + 0xb00]
0x1404e93bf: mov qword ptr [rbp + 0x550], rax
0x1404e93c6: lea rax, [rbp + 0x788]
0x1404e93cd: mov qword ptr [rbp + 0x558], rax
0x1404e93d4: lea rax, [rbp + 0x1000]
0x1404e93db: mov qword ptr [rbp + 0x560], rax
0x1404e93e2: mov qword ptr [rbp + 0x568], 0
0x1404e93ed: lea rax, [rbp + 0x530]
0x1404e93f4: mov qword ptr [rsp + 0x20], rax
0x1404e93f9: lea rcx, [rbp + 0xd20]
0x1404e9400: mov rdx, qword ptr [rbp + 0x10c0]
0x1404e9407: mov r9, r12
0x1404e940a: call 0x140161cd0
0x1404e940f: mov byte ptr [rbp + 0x1117], 0
0x1404e9416: lea rcx, [rbp + 0xb00]
0x1404e941d: call 0x140609f60
0x1404e9422: xor eax, eax
0x1404e9424: cmp rax, qword ptr [rbp + 0xd20]
0x1404e942b: jo 0x1404ea6ea
0x1404e9431: lea rcx, [rbp + 0xb00]
0x1404e9438: lea rdx, [rbp + 0xd20]
0x1404e943f: mov r8d, 0x220
0x1404e9445: call 0x142923c00
0x1404e944a: mov rax, qword ptr [rip + 0x3142f57]
0x1404e9451: mov rax, qword ptr [rax]
0x1404e9454: cmp rax, 3
0x1404e9458: jb 0x1404e9516
0x1404e945e: lea rcx, [rip + 0x250297b]
0x1404e9465: call 0x14067dd80
0x1404e946a: movupd xmm0, xmmword ptr [rax]
0x1404e946e: mov eax, dword ptr [rax + 0x10]
0x1404e9471: mov qword ptr [rbp + 0xd50], 3
0x1404e947c: lea rcx, [rip + 0x25026ed]
0x1404e9483: mov qword ptr [rbp + 0xd58], rcx
0x1404e948a: mov qword ptr [rbp + 0xd60], 0x17
0x1404e9495: lea rdx, [rip + 0x2502934]
0x1404e949c: mov qword ptr [rbp + 0xd70], rdx
0x1404e94a3: mov qword ptr [rbp + 0xd78], 1
0x1404e94ae: mov qword ptr [rbp + 0xd80], 8
0x1404e94b9: mov qword ptr [rbp + 0xd20], 0
0x1404e94c4: xorps xmm1, xmm1
0x1404e94c7: movups xmmword ptr [rbp + 0xd88], xmm1
0x1404e94ce: mov qword ptr [rbp + 0xd28], rcx
0x1404e94d5: mov qword ptr [rbp + 0xd30], 0x17
0x1404e94e0: mov qword ptr [rbp + 0xd38], 0
0x1404e94eb: movupd xmmword ptr [rbp + 0xd40], xmm0
0x1404e94f3: mov dword ptr [rbp + 0xd68], 1
0x1404e94fd: mov dword ptr [rbp + 0xd6c], eax
0x1404e9503: lea rcx, [rbp + 0x1000]
0x1404e950a: lea rdx, [rbp + 0xd20]
0x1404e9511: call 0x140d4a330
0x1404e9516: lea rcx, [rbp + 0x530]
0x1404e951d: lea rdx, [rbp + 0xb00]
0x1404e9524: mov r8d, 0x218
0x1404e952a: call 0x142923c00
0x1404e952f: mov r12d, dword ptr [rbp + 0xd18]
0x1404e9536: lea rcx, [rbp + 0xb00]
0x1404e953d: lea rdx, [rbp + 0x530]
0x1404e9544: mov r8d, 0x218
0x1404e954a: call 0x142923c00
0x1404e954f: mov esi, dword ptr [rip + 0x3141b93]
0x1404e9555: cmp esi, 0x3b9aca00
0x1404e955b: jne 0x1404e9571
0x1404e955d: xor edi, edi
0x1404e955f: xor esi, esi
0x1404e9561: mov al, 2
0x1404e9563: and r14b, byte ptr [rbp + 0x10b8]
0x1404e956a: jne 0x1404e95ad
0x1404e956c: jmp 0x1404e96b0
0x1404e9571: mov rbx, qword ptr [rip + 0x314b638]
0x1404e9578: mov byte ptr [rbp + 0x1110], 1
0x1404e957f: call 0x140d635e0
0x1404e9584: mov byte ptr [rbp + 0x1110], 1
0x1404e958b: mov rcx, rax
0x1404e958e: mov r8, rbx
0x1404e9591: mov r9d, esi
0x1404e9594: call 0x140d63b00
0x1404e9599: mov rdi, rax
0x1404e959c: mov esi, edx
0x1404e959e: mov al, 2
0x1404e95a0: and r14b, byte ptr [rbp + 0x10b8]
0x1404e95a7: je 0x1404e96b0
0x1404e95ad: mov rax, qword ptr [r13 + 0x10]
0x1404e95b1: test rax, rax
0x1404e95b4: je 0x1404e95ce
0x1404e95b6: dec rax
0x1404e95b9: mov qword ptr [r13 + 0x10], rax
0x1404e95bd: mov rcx, qword ptr [r13 + 8]
0x1404e95c1: imul rdx, rax, 0x58
0x1404e95c5: movzx eax, byte ptr [rcx + rdx + 0x4c]
0x1404e95ca: cmp al, 2
0x1404e95cc: jne 0x1404e95f1
0x1404e95ce: mov byte ptr [rbp + 0x1110], 1
0x1404e95d5: lea rcx, [rbp + 0xd20]
0x1404e95dc: mov rdx, qword ptr [rbp + 0x1100]
0x1404e95e3: call 0x140bff4a0
0x1404e95e8: movzx eax, byte ptr [rbp + 0xd6c]
0x1404e95ef: jmp 0x1404e964a
0x1404e95f1: add rcx, rdx
0x1404e95f4: movups xmm0, xmmword ptr [rcx + 0x3c]
0x1404e95f8: movups xmmword ptr [rbp + 0xd5c], xmm0
0x1404e95ff: movupd xmm0, xmmword ptr [rcx]
0x1404e9603: movups xmm1, xmmword ptr [rcx + 0x10]
0x1404e9607: movupd xmm2, xmmword ptr [rcx + 0x20]
0x1404e960c: movupd xmm3, xmmword ptr [rcx + 0x30]
0x1404e9611: movapd xmmword ptr [rbp + 0xd50], xmm3
0x1404e9619: movapd xmmword ptr [rbp + 0xd40], xmm2
0x1404e9621: movaps xmmword ptr [rbp + 0xd30], xmm1
0x1404e9628: movapd xmmword ptr [rbp + 0xd20], xmm0
0x1404e9630: mov byte ptr [rbp + 0xd6c], al
0x1404e9636: mov rdx, qword ptr [rcx + 0x4d]
0x1404e963a: mov qword ptr [rbp + 0xd6d], rdx
0x1404e9641: mov ecx, dword ptr [rcx + 0x54]
0x1404e9644: mov dword ptr [rbp + 0xd74], ecx
0x1404e964a: movups xmm0, xmmword ptr [rbp + 0xd5c]
0x1404e9651: movups xmmword ptr [rbp + 0x91c], xmm0
0x1404e9658: movapd xmm0, xmmword ptr [rbp + 0xd20]
0x1404e9660: movaps xmm1, xmmword ptr [rbp + 0xd30]
0x1404e9667: movapd xmm2, xmmword ptr [rbp + 0xd40]
0x1404e966f: movapd xmm3, xmmword ptr [rbp + 0xd50]
0x1404e9677: movapd xmmword ptr [rbp + 0x910], xmm3
0x1404e967f: movapd xmmword ptr [rbp + 0x900], xmm2
0x1404e9687: movaps xmmword ptr [rbp + 0x8f0], xmm1
0x1404e968e: movapd xmmword ptr [rbp + 0x8e0], xmm0
0x1404e9696: mov rcx, qword ptr [rbp + 0xd6d]
0x1404e969d: mov qword ptr [rbp + 0xff0], rcx
0x1404e96a4: mov ecx, dword ptr [rbp + 0xd74]
0x1404e96aa: mov dword ptr [rbp + 0xff7], ecx
0x1404e96b0: lea r9, [r13 + 0x18]
0x1404e96b4: movzx ecx, byte ptr [rbp + 0x848]
0x1404e96bb: movups xmm0, xmmword ptr [rbp + 0x91c]
0x1404e96c2: movups xmmword ptr [rbp + 0x34c], xmm0
0x1404e96c9: movapd xmm0, xmmword ptr [rbp + 0x8e0]
0x1404e96d1: movapd xmm1, xmmword ptr [rbp + 0x8f0]
0x1404e96d9: movapd xmm2, xmmword ptr [rbp + 0x900]
0x1404e96e1: movapd xmm3, xmmword ptr [rbp + 0x910]
0x1404e96e9: movapd xmmword ptr [rbp + 0x340], xmm3
0x1404e96f1: movapd xmmword ptr [rbp + 0x330], xmm2
0x1404e96f9: movapd xmmword ptr [rbp + 0x320], xmm1
0x1404e9701: movapd xmmword ptr [rbp + 0x310], xmm0
0x1404e9709: mov byte ptr [rbp + 0x35c], al
0x1404e970f: mov rax, qword ptr [rbp + 0xff0]
0x1404e9716: mov qword ptr [rbp + 0x35d], rax
0x1404e971d: mov eax, dword ptr [rbp + 0xff7]
0x1404e9723: mov dword ptr [rbp + 0x364], eax
0x1404e9729: mov byte ptr [rbp + 0x1110], 1
0x1404e9730: lea rax, [rbp + 0x310]
0x1404e9737: mov qword ptr [rsp + 0x30], rax
0x1404e973c: mov byte ptr [rsp + 0x28], cl
0x1404e9740: mov byte ptr [rsp + 0x20], cl
0x1404e9744: lea rcx, [rbp + 0xd20]
0x1404e974b: mov rdx, r15
0x1404e974e: mov r8, qword ptr [rbp + 0x1100]
0x1404e9755: call 0x140beabd0
0x1404e975a: mov dword ptr [rbp + 0x10b8], r12d
0x1404e9761: mov qword ptr [rbp + 0x10c8], rdi
0x1404e9768: mov dword ptr [rbp + 0x10c0], esi
0x1404e976e: lea rcx, [rbp + 0x310]
0x1404e9775: lea r12, [rbp + 0xd20]
0x1404e977c: mov r8d, 0x190
0x1404e9782: mov rdx, r12
0x1404e9785: call 0x142923c00
0x1404e978a: lea rdx, [rbp + 0xeb0]
0x1404e9791: lea rbx, [rbp + 0x538]
0x1404e9798: mov r8d, 0x108
0x1404e979e: mov rcx, rbx
0x1404e97a1: call 0x142923c00
0x1404e97a6: mov qword ptr [rbp + 0x530], 0
0x1404e97b1: lea rdi, [rbp + 0x298]
0x1404e97b8: xor ecx, ecx
0x1404e97ba: lea r15, [rbp + 0x290]
0x1404e97c1: lea r14, [rbp + 0x9c0]
0x1404e97c8: jmp 0x1404e97d7
0x1404e97ca: nop word ptr [rax + rax]
0x1404e97d0: mov rcx, qword ptr [rbp + 0x530]
0x1404e97d7: mov eax, dword ptr [rbp + 0x638]
0x1404e97dd: cmp rcx, rax
0x1404e97e0: je 0x1404e9a03
0x1404e97e6: mov rax, rcx
0x1404e97e9: inc rcx
0x1404e97ec: mov qword ptr [rbp + 0x530], rcx
0x1404e97f3: shl rax, 7
0x1404e97f7: mov rdx, qword ptr [rbx + rax]
0x1404e97fb: mov r8, rdx
0x1404e97fe: neg r8
0x1404e9801: jo 0x1404e97d7
0x1404e9803: movabs rcx, 0x8000000000000001
0x1404e980d: cmp rdx, rcx
0x1404e9810: je 0x1404e9a03
0x1404e9816: add rax, rbx
0x1404e9819: mov qword ptr [rbp + 0x290], rdx
0x1404e9820: mov rcx, qword ptr [rax + 0x78]
0x1404e9824: mov qword ptr [rdi + 0x70], rcx
0x1404e9828: movups xmm0, xmmword ptr [rax + 0x68]
0x1404e982c: movups xmmword ptr [rdi + 0x60], xmm0
0x1404e9830: movups xmm0, xmmword ptr [rax + 0x58]
0x1404e9834: movups xmmword ptr [rdi + 0x50], xmm0
0x1404e9838: movups xmm0, xmmword ptr [rax + 0x48]
0x1404e983c: movups xmmword ptr [rdi + 0x40], xmm0
0x1404e9840: movupd xmm0, xmmword ptr [rax + 8]
0x1404e9845: movupd xmm1, xmmword ptr [rax + 0x18]
0x1404e984a: movupd xmm2, xmmword ptr [rax + 0x28]
0x1404e984f: movupd xmm3, xmmword ptr [rax + 0x38]
0x1404e9854: movupd xmmword ptr [rdi + 0x30], xmm3
0x1404e9859: movupd xmmword ptr [rdi + 0x20], xmm2
0x1404e985e: movupd xmmword ptr [rdi + 0x10], xmm1
0x1404e9863: movupd xmmword ptr [rdi], xmm0
0x1404e9867: mov rcx, r12
0x1404e986a: mov rdx, r15
0x1404e986d: call 0x140294cf0
0x1404e9872: mov rax, qword ptr [rbp + 0xd50]
0x1404e9879: mov qword ptr [rbp + 0x1030], rax
0x1404e9880: movups xmm0, xmmword ptr [rbp + 0xd20]
0x1404e9887: movups xmm1, xmmword ptr [rbp + 0xd30]
0x1404e988e: movups xmm2, xmmword ptr [rbp + 0xd40]
0x1404e9895: movaps xmmword ptr [rbp + 0x1020], xmm2
0x1404e989c: movaps xmmword ptr [rbp + 0x1010], xmm1
0x1404e98a3: movaps xmmword ptr [rbp + 0x1000], xmm0
0x1404e98aa: movups xmm0, xmmword ptr [r12 + 0x38]
0x1404e98b0: movupd xmm1, xmmword ptr [r12 + 0x48]
0x1404e98b7: movupd xmm2, xmmword ptr [r12 + 0x58]
0x1404e98be: movupd xmm3, xmmword ptr [r12 + 0x68]
0x1404e98c5: movaps xmmword ptr [rbp + 0x960], xmm0
0x1404e98cc: movapd xmmword ptr [rbp + 0x970], xmm1
0x1404e98d4: movapd xmmword ptr [rbp + 0x980], xmm2
0x1404e98dc: movapd xmmword ptr [rbp + 0x990], xmm3
0x1404e98e4: movupd xmm0, xmmword ptr [r12 + 0x78]
0x1404e98eb: movapd xmmword ptr [rbp + 0x9a0], xmm0
0x1404e98f3: mov rax, qword ptr [r12 + 0x88]
0x1404e98fb: mov qword ptr [rbp + 0x9b0], rax
0x1404e9902: mov rsi, qword ptr [r13 + 0x10]
0x1404e9906: cmp rsi, qword ptr [r13]
0x1404e990a: jne 0x1404e991b
0x1404e990c: mov rcx, r13
0x1404e990f: lea rdx, [rip + 0x2502552]
0x1404e9916: call 0x140562f30
0x1404e991b: mov rax, qword ptr [r13 + 8]
0x1404e991f: imul rcx, rsi, 0x58
0x1404e9923: movaps xmm0, xmmword ptr [rbp + 0x960]
0x1404e992a: movaps xmm1, xmmword ptr [rbp + 0x970]
0x1404e9931: movaps xmm2, xmmword ptr [rbp + 0x980]
0x1404e9938: movapd xmm3, xmmword ptr [rbp + 0x990]
0x1404e9940: movapd xmm4, xmmword ptr [rbp + 0x9a0]
0x1404e9948: mov rdx, qword ptr [rbp + 0x9b0]
0x1404e994f: mov qword ptr [rax + rcx + 0x50], rdx
0x1404e9954: movupd xmmword ptr [rax + rcx + 0x40], xmm4
0x1404e995a: movupd xmmword ptr [rax + rcx + 0x30], xmm3
0x1404e9960: movups xmmword ptr [rax + rcx + 0x20], xmm2
0x1404e9965: movups xmmword ptr [rax + rcx + 0x10], xmm1
0x1404e996a: movups xmmword ptr [rax + rcx], xmm0
0x1404e996e: inc rsi
0x1404e9971: mov qword ptr [r13 + 0x10], rsi
0x1404e9975: mov rax, qword ptr [rbp + 0x1030]
0x1404e997c: mov qword ptr [rbp + 0xd50], rax
0x1404e9983: movapd xmm0, xmmword ptr [rbp + 0x1000]
0x1404e998b: movapd xmm1, xmmword ptr [rbp + 0x1010]
0x1404e9993: movapd xmm2, xmmword ptr [rbp + 0x1020]
0x1404e999b: movapd xmmword ptr [rbp + 0xd40], xmm2
0x1404e99a3: movapd xmmword ptr [rbp + 0xd30], xmm1
0x1404e99ab: movapd xmmword ptr [rbp + 0xd20], xmm0
0x1404e99b3: mov byte ptr [rbp + 0x110d], 0
0x1404e99ba: mov rcx, r14
0x1404e99bd: mov rdx, r12
0x1404e99c0: call 0x140bd3cb0
0x1404e99c5: mov rcx, r14
0x1404e99c8: mov rdx, qword ptr [rbp + 0x10a8]
0x1404e99cf: call 0x140bd4030
0x1404e99d4: mov rax, qword ptr [rbp + 0x9c0]
0x1404e99db: test rax, rax
0x1404e99de: je 0x1404e97d0
0x1404e99e4: mov rcx, qword ptr [rbp + 0x9c8]
0x1404e99eb: shl rax, 3
0x1404e99ef: lea rdx, [rax + rax*2]
0x1404e99f3: mov r8d, 8
0x1404e99f9: call 0x140613c20
0x1404e99fe: jmp 0x1404e97d0
0x1404e9a03: mov byte ptr [rbp + 0x1116], 1
0x1404e9a0a: mov byte ptr [rbp + 0x1115], 1
0x1404e9a11: lea rcx, [rbp + 0x530]
0x1404e9a18: call 0x1405d1490
0x1404e9a1d: mov rdi, qword ptr [rbp + 0x10e0]
0x1404e9a24: mov rbx, qword ptr [rbp + 0x10d8]
0x1404e9a2b: mov r12, qword ptr [rbp + 0x11f8]
0x1404e9a32: mov byte ptr [rbp + 0x1116], 1
0x1404e9a39: mov byte ptr [rbp + 0x1115], 1
0x1404e9a40: call 0x140d635e0
0x1404e9a45: mov qword ptr [rbp + 0xad0], rax
0x1404e9a4c: mov dword ptr [rbp + 0xad8], edx
0x1404e9a52: lea rcx, [rbp + 0xd20]
0x1404e9a59: lea rdx, [rbp + 0xb00]
0x1404e9a60: mov r8d, 0x218
0x1404e9a66: call 0x142923c00
0x1404e9a6b: mov byte ptr [rbp + 0x1028], 2
0x1404e9a72: mov byte ptr [rbp + 0x110c], 1
0x1404e9a79: lea rsi, [rbp + 0x530]
0x1404e9a80: mov rcx, rsi
0x1404e9a83: mov rdx, rbx
0x1404e9a86: call 0x140909310
0x1404e9a8b: mov byte ptr [rbp + 0x110c], 0
0x1404e9a92: mov qword ptr [rsp + 0x20], rsi
0x1404e9a97: lea rcx, [rbp - 0x48]
0x1404e9a9b: lea rdx, [rbp + 0x310]
0x1404e9aa2: lea r8, [rbp + 0xd20]
0x1404e9aa9: lea r9, [rbp + 0x1000]
0x1404e9ab0: call 0x1400fa190
0x1404e9ab5: mov r8, qword ptr [rbp + 0x490]
0x1404e9abc: mov byte ptr [rbp + 0x1116], 1
0x1404e9ac3: mov byte ptr [rbp + 0x1115], 0
0x1404e9aca: lea rcx, [rbp + 0xd20]
0x1404e9ad1: lea rdx, [rbp - 0x48]
0x1404e9ad5: call 0x140be9d00
0x1404e9ada: lea rcx, [rbp + 0x530]
0x1404e9ae1: lea rdx, [rbp + 0xd20]
0x1404e9ae8: mov r8d, 0x90
0x1404e9aee: call 0x142923c00
0x1404e9af3: movupd xmm0, xmmword ptr [rbp + 0xdb0]
0x1404e9afb: movupd xmm1, xmmword ptr [rbp + 0xdc0]
0x1404e9b03: movupd xmm2, xmmword ptr [rbp + 0xdd0]
0x1404e9b0b: movapd xmmword ptr [rbp + 0xac0], xmm2
0x1404e9b13: movapd xmmword ptr [rbp + 0xab0], xmm1
0x1404e9b1b: movapd xmmword ptr [rbp + 0xaa0], xmm0
0x1404e9b23: mov byte ptr [rbp + 0x1112], 1
0x1404e9b2a: mov byte ptr [rbp + 0x1111], 1
0x1404e9b31: lea rcx, [rbp + 0xd20]
0x1404e9b38: lea rdx, [rbp + 0xaa0]
0x1404e9b3f: mov r8d, dword ptr [rbp + 0x10b8]
0x1404e9b46: call 0x140be9c80
0x1404e9b4b: mov byte ptr [rbp + 0x1112], 1
0x1404e9b52: mov byte ptr [rbp + 0x1111], 1
0x1404e9b59: lea rcx, [rbp + 0xa28]
0x1404e9b60: lea rdx, [rbp + 0xd20]
0x1404e9b67: call 0x140bd63f0
0x1404e9b6c: lea rsi, [rdi + 0xd0]
0x1404e9b73: lea rcx, [rbp + 0xd20]
0x1404e9b7a: mov r8d, 0x90
0x1404e9b80: mov rdx, rsi
0x1404e9b83: call 0x142923c00
0x1404e9b88: lea rdx, [rbp + 0x530]
0x1404e9b8f: mov r8d, 0x90
0x1404e9b95: mov rcx, rsi
0x1404e9b98: call 0x142923c00
0x1404e9b9d: xor eax, eax
0x1404e9b9f: cmp rax, qword ptr [rbp + 0xd20]
0x1404e9ba6: jo 0x1404e9bc2
0x1404e9ba8: mov byte ptr [rbp + 0x1114], 1
0x1404e9baf: mov byte ptr [rbp + 0x1113], 1
0x1404e9bb6: lea rcx, [rbp + 0xd20]
0x1404e9bbd: call 0x1405ef210
0x1404e9bc2: test r12, r12
0x1404e9bc5: sete al
0x1404e9bc8: movzx ecx, byte ptr [rbp + 0x10fc]
0x1404e9bcf: xor cl, 1
0x1404e9bd2: mov r14b, 1
0x1404e9bd5: or cl, al
0x1404e9bd7: jne 0x1404e9c06
0x1404e9bd9: mov byte ptr [rbp + 0x1114], 1
0x1404e9be0: mov byte ptr [rbp + 0x1113], 1
0x1404e9be7: lea rcx, [rbp + 0x9c0]
0x1404e9bee: mov rdx, qword ptr [rbp + 0x1100]
0x1404e9bf5: mov r8, r12
0x1404e9bf8: call 0x140bfd9d0
0x1404e9bfd: cmp byte ptr [rbp + 0xa1e], 7
0x1404e9c04: jne 0x1404e9c0e
0x1404e9c06: xor r15d, r15d
0x1404e9c09: jmp 0x1404e9f16
0x1404e9c0e: mov rax, qword ptr [rbp + 0xa20]
0x1404e9c15: mov qword ptr [rbp + 0x1060], rax
0x1404e9c1c: movups xmm0, xmmword ptr [rbp + 0xa10]
0x1404e9c23: movaps xmmword ptr [rbp + 0x1050], xmm0
0x1404e9c2a: movups xmm0, xmmword ptr [rbp + 0xa00]
0x1404e9c31: movaps xmmword ptr [rbp + 0x1040], xmm0
0x1404e9c38: movupd xmm0, xmmword ptr [rbp + 0x9c0]
0x1404e9c40: movupd xmm1, xmmword ptr [rbp + 0x9d0]
0x1404e9c48: movupd xmm2, xmmword ptr [rbp + 0x9e0]
0x1404e9c50: movupd xmm3, xmmword ptr [rbp + 0x9f0]
0x1404e9c58: movapd xmmword ptr [rbp + 0x1030], xmm3
0x1404e9c60: movapd xmmword ptr [rbp + 0x1020], xmm2
0x1404e9c68: movapd xmmword ptr [rbp + 0x1010], xmm1
0x1404e9c70: movapd xmmword ptr [rbp + 0x1000], xmm0
0x1404e9c78: mov byte ptr [rbp + 0x110f], 1
0x1404e9c7f: mov byte ptr [rbp + 0x110e], 1
0x1404e9c86: lea rbx, [rbp + 0xd20]
0x1404e9c8d: lea rsi, [rbp + 0x1000]
0x1404e9c94: mov rcx, rbx
0x1404e9c97: mov rdx, rsi
0x1404e9c9a: call 0x140c0f120
0x1404e9c9f: mov rdi, qword ptr [rbp + 0xd20]
0x1404e9ca6: movups xmm0, xmmword ptr [rbp + 0xd48]
0x1404e9cad: movaps xmmword ptr [rbp + 0x950], xmm0
0x1404e9cb4: movups xmm0, xmmword ptr [rbp + 0xd38]
0x1404e9cbb: movaps xmmword ptr [rbp + 0x940], xmm0
0x1404e9cc2: movups xmm0, xmmword ptr [rbp + 0xd28]
0x1404e9cc9: movaps xmmword ptr [rbp + 0x930], xmm0
0x1404e9cd0: mov rax, qword ptr [rbp + 0xa58]
0x1404e9cd7: mov qword ptr [rbp + 0xa90], rax
0x1404e9cde: movupd xmm0, xmmword ptr [rbp + 0xa28]
0x1404e9ce6: movupd xmm1, xmmword ptr [rbp + 0xa38]
0x1404e9cee: movupd xmm2, xmmword ptr [rbp + 0xa48]
0x1404e9cf6: movapd xmmword ptr [rbp + 0xa80], xmm2
0x1404e9cfe: movapd xmmword ptr [rbp + 0xa70], xmm1
0x1404e9d06: movapd xmmword ptr [rbp + 0xa60], xmm0
0x1404e9d0e: mov r14, qword ptr [rbp + 0x488]
0x1404e9d15: mov byte ptr [rbp + 0x110b], 1
0x1404e9d1c: lea rcx, [rbp + 0x750]
0x1404e9d23: mov r15, qword ptr [rbp + 0x10d8]
0x1404e9d2a: mov rdx, r15
0x1404e9d2d: call 0x140909310
0x1404e9d32: mov rax, qword ptr [rbp + 0x10e0]
0x1404e9d39: lea r9, [rax + 0x1c0]
0x1404e9d40: add r13, 0x78
0x1404e9d44: movaps xmm0, xmmword ptr [rbp + 0x930]
0x1404e9d4b: movaps xmm1, xmmword ptr [rbp + 0x940]
0x1404e9d52: movaps xmm2, xmmword ptr [rbp + 0x950]
0x1404e9d59: movaps xmmword ptr [rbp + 0xfe0], xmm2
0x1404e9d60: movaps xmmword ptr [rbp + 0xfd0], xmm1
0x1404e9d67: movaps xmmword ptr [rbp + 0xfc0], xmm0
0x1404e9d6e: mov qword ptr [rbp + 0xd20], r15
0x1404e9d75: mov qword ptr [rbp + 0xd28], r14
0x1404e9d7c: mov qword ptr [rbp + 0xd30], r13
0x1404e9d83: movapd xmm0, xmmword ptr [rbp + 0xa60]
0x1404e9d8b: movapd xmm1, xmmword ptr [rbp + 0xa70]
0x1404e9d93: movapd xmm2, xmmword ptr [rbp + 0xa80]
0x1404e9d9b: movupd xmmword ptr [rbp + 0xd48], xmm0
0x1404e9da3: movupd xmmword ptr [rbp + 0xd58], xmm1
0x1404e9dab: movupd xmmword ptr [rbp + 0xd68], xmm2
0x1404e9db3: mov rax, qword ptr [rbp + 0xa90]
0x1404e9dba: mov qword ptr [rbp + 0xd78], rax
0x1404e9dc1: mov qword ptr [rbp + 0xd38], rdi
0x1404e9dc8: lea rax, [rbp + 0x788]
0x1404e9dcf: mov qword ptr [rbp + 0xd40], rax
0x1404e9dd6: mov byte ptr [rbp + 0x110b], 0
0x1404e9ddd: mov qword ptr [rsp + 0x28], rbx
0x1404e9de2: lea rax, [rbp + 0xfc0]
0x1404e9de9: mov qword ptr [rsp + 0x20], rax
0x1404e9dee: lea rcx, [rbp + 0x8b0]
0x1404e9df5: lea rdx, [rbp + 0x310]
0x1404e9dfc: lea rdi, [rbp + 0x750]
0x1404e9e03: mov r8, rdi
0x1404e9e06: call 0x1400fb5b0
0x1404e9e0b: cmp qword ptr [rbp + 0x8b0], 0
0x1404e9e13: je 0x1404ea721
0x1404e9e19: movupd xmm0, xmmword ptr [rbp + 0x8b0]
0x1404e9e21: movupd xmm1, xmmword ptr [rbp + 0x8c0]
0x1404e9e29: movupd xmm2, xmmword ptr [rbp + 0x8d0]
0x1404e9e31: movapd xmmword ptr [rbp + 0xfc0], xmm0
0x1404e9e39: movapd xmmword ptr [rbp + 0xfe0], xmm2
0x1404e9e41: movapd xmmword ptr [rbp + 0xfd0], xmm1
0x1404e9e49: cmp qword ptr [rbp + 0xfc0], 0
0x1404e9e51: je 0x1404ea72c
0x1404e9e57: mov rax, qword ptr [rbp + 0x1060]
0x1404e9e5e: mov qword ptr [rbp + 0xd80], rax
0x1404e9e65: movaps xmm0, xmmword ptr [rbp + 0x1050]
0x1404e9e6c: movaps xmmword ptr [rbp + 0xd70], xmm0
0x1404e9e73: movaps xmm0, xmmword ptr [rbp + 0x1040]
0x1404e9e7a: movaps xmmword ptr [rbp + 0xd60], xmm0
0x1404e9e81: movapd xmm0, xmmword ptr [rbp + 0x1000]
0x1404e9e89: movapd xmm1, xmmword ptr [rbp + 0x1010]
0x1404e9e91: movapd xmm2, xmmword ptr [rbp + 0x1020]
0x1404e9e99: movapd xmm3, xmmword ptr [rbp + 0x1030]
0x1404e9ea1: movapd xmmword ptr [rbp + 0xd50], xmm3
0x1404e9ea9: movapd xmmword ptr [rbp + 0xd40], xmm2
0x1404e9eb1: movapd xmmword ptr [rbp + 0xd30], xmm1
0x1404e9eb9: movapd xmmword ptr [rbp + 0xd20], xmm0
0x1404e9ec1: mov byte ptr [rbp + 0x110a], 0
0x1404e9ec8: lea rcx, [rbp + 0xd20]
0x1404e9ecf: call 0x140c0f170
0x1404e9ed4: mov rdi, rax
0x1404e9ed7: mov esi, edx
0x1404e9ed9: cmp qword ptr [rbp + 0xfc0], 0
0x1404e9ee1: je 0x1404e9f10
0x1404e9ee3: lea rdx, [rbp + 0xfe0]
0x1404e9eea: mov byte ptr [rbp + 0x110f], 0
0x1404e9ef1: mov byte ptr [rbp + 0x110e], 0
0x1404e9ef8: lea rcx, [rbp + 0xfc0]
0x1404e9eff: mov r8d, 0x48
0x1404e9f05: mov r9d, 0x10
0x1404e9f0b: call 0x14004f690
0x1404e9f10: mov r15b, 1
0x1404e9f13: xor r14d, r14d
0x1404e9f16: lea rbx, [rbp + 0xd20]
0x1404e9f1d: lea rdx, [rbp + 0x310]
0x1404e9f24: mov r8d, 0x190
0x1404e9f2a: mov rcx, rbx
0x1404e9f2d: call 0x142923c00
0x1404e9f32: mov byte ptr [rbp + 0x1114], r14b
0x1404e9f39: mov byte ptr [rbp + 0x1113], 0
0x1404e9f40: mov rcx, rbx
0x1404e9f43: call 0x140bec400
0x1404e9f48: test r15b, r15b
0x1404e9f4b: je 0x1404e9f79
0x1404e9f4d: test r12, r12
0x1404e9f50: je 0x1404ea5f8
0x1404e9f56: mov byte ptr [rbp + 0x1114], r14b
0x1404e9f5d: mov byte ptr [rbp + 0x1113], 0
0x1404e9f64: mov rcx, r12
0x1404e9f67: mov rdx, qword ptr [rbp + 0x1100]
0x1404e9f6e: mov r8, rdi
0x1404e9f71: mov r9d, esi
0x1404e9f74: call 0x140c0ef80
0x1404e9f79: mov byte ptr [rbp + 0x1114], r14b
0x1404e9f80: mov byte ptr [rbp + 0x1113], 0
0x1404e9f87: mov rcx, qword ptr [rbp + 0x1100]
0x1404e9f8e: call 0x14031ea20
0x1404e9f93: mov rdx, qword ptr [rbp + 0x1090]
0x1404e9f9a: add rdx, 0xab0
0x1404e9fa1: mov byte ptr [rbp + 0x1114], r14b
0x1404e9fa8: mov byte ptr [rbp + 0x1113], 0
0x1404e9faf: lea rcx, [rbp + 0xd20]
0x1404e9fb6: call 0x140c02da0
0x1404e9fbb: mov rax, qword ptr [rbp + 0xd30]
0x1404e9fc2: mov qword ptr [rip + 0x314abdf], rax
0x1404e9fc9: movupd xmm0, xmmword ptr [rbp + 0xd20]
0x1404e9fd1: movupd xmmword ptr [rip + 0x314abbf], xmm0
0x1404e9fd9: test r14b, r14b
0x1404e9fdc: mov rbx, qword ptr [rbp + 0x10e0]
0x1404e9fe3: je 0x1404ea00b
0x1404e9fe5: lea rcx, [rbp + 0xa28]
0x1404e9fec: call 0x14003bb80
0x1404e9ff1: mov byte ptr [rbp + 0x1112], 0
0x1404e9ff8: mov byte ptr [rbp + 0x1111], 0
0x1404e9fff: lea rcx, [rbp + 0xa28]
0x1404ea006: call 0x140be24b0
0x1404ea00b: lea rdx, [rbp + 0xac0]
0x1404ea012: mov byte ptr [rbp + 0x1109], 0
0x1404ea019: mov byte ptr [rbp + 0x1108], 0
0x1404ea020: lea rcx, [rbp + 0xaa0]
0x1404ea027: mov r8d, 0x48
0x1404ea02d: mov r9d, 0x10
0x1404ea033: call 0x14004f690
0x1404ea038: mov byte ptr [rbp + 0x1116], 0
0x1404ea03f: mov byte ptr [rbp + 0x1115], 0
0x1404ea046: mov rcx, qword ptr [rbp + 0x1100]
0x1404ea04d: call 0x140bf8f80
0x1404ea052: cmp byte ptr [rbp + 0x87a], 0
0x1404ea059: je 0x1404ea46d
0x1404ea05f: mov byte ptr [rbp + 0x1116], 0
0x1404ea066: mov byte ptr [rbp + 0x1115], 0
0x1404ea06d: lea rcx, [rbp + 0xad0]
0x1404ea074: call 0x140d63910
0x1404ea079: mov rdi, rax
0x1404ea07c: mov esi, edx
0x1404ea07e: mov byte ptr [rbp + 0x1116], 0
0x1404ea085: mov byte ptr [rbp + 0x1115], 0
0x1404ea08c: mov rcx, rbx
0x1404ea08f: mov rdx, qword ptr [rbp + 0x1100]
0x1404ea096: call 0x140beaae0
0x1404ea09b: cmp rax, 1
0x1404ea09f: jne 0x1404ea46d
0x1404ea0a5: movq xmm0, rdi
0x1404ea0aa: movq xmm2, qword ptr [rip + 0x244487e]
0x1404ea0b2: punpckldq xmm0, xmm2
0x1404ea0b6: movapd xmm3, xmmword ptr [rip + 0x2444882]
0x1404ea0be: subpd xmm0, xmm3
0x1404ea0c2: movapd xmm4, xmm0
0x1404ea0c6: unpckhpd xmm4, xmm0
0x1404ea0ca: cvtsi2sd xmm1, esi
0x1404ea0ce: addsd xmm4, xmm0
0x1404ea0d2: movsd xmm5, qword ptr [rip + 0x24b84ce]
0x1404ea0da: divsd xmm1, xmm5
0x1404ea0de: addsd xmm1, xmm4
0x1404ea0e2: movq xmm0, qword ptr [rbp + 0x10c8]
0x1404ea0ea: punpckldq xmm0, xmm2
0x1404ea0ee: subpd xmm0, xmm3
0x1404ea0f2: movapd xmm2, xmm0
0x1404ea0f6: unpckhpd xmm2, xmm0
0x1404ea0fa: addsd xmm2, xmm0
0x1404ea0fe: xorps xmm0, xmm0
0x1404ea101: cvtsi2sd xmm0, dword ptr [rbp + 0x10c0]
0x1404ea109: divsd xmm0, xmm5
0x1404ea10d: movq xmm3, rdx
0x1404ea112: punpckldq xmm3, xmmword ptr [rip + 0x2444816]
0x1404ea11a: addsd xmm0, xmm2
0x1404ea11e: subpd xmm3, xmmword ptr [rip + 0x244481a]
0x1404ea126: movapd xmm2, xmm3
0x1404ea12a: unpckhpd xmm2, xmm3
0x1404ea12e: addsd xmm2, xmm3
0x1404ea132: divsd xmm2, qword ptr [rip + 0x24b846e]
0x1404ea13a: addsd xmm0, xmm1
0x1404ea13e: mov r8, qword ptr [rip + 0x314aa93]
0x1404ea145: lea rax, [rip + 0x314aa6c]
0x1404ea14c: mov ecx, r8d
0x1404ea14f: and ecx, 3
0x1404ea152: movsd qword ptr [rax + rcx*8], xmm2
0x1404ea157: mov rcx, qword ptr [rip + 0x314aaa2]
0x1404ea15e: mov r9d, ecx
0x1404ea161: and r9d, 3
0x1404ea165: lea rax, [rcx + 1]
0x1404ea169: mov rdx, r8
0x1404ea16c: lea r10, [rip + 0x314aa6d]
0x1404ea173: movsd qword ptr [r10 + r9*8], xmm0
0x1404ea179: mov qword ptr [rip + 0x314aa80], rax
0x1404ea180: inc rdx
0x1404ea183: mov qword ptr [rip + 0x314aa4e], rdx
0x1404ea18a: jne 0x1404ea199
0x1404ea18c: movsd xmm0, qword ptr [rip + 0x2445d54]
0x1404ea194: jmp 0x1404ea235
0x1404ea199: movsd xmm0, qword ptr [rip + 0x314aa17]
0x1404ea1a1: minsd xmm0, qword ptr [rip + 0x2445d3f]
0x1404ea1a9: test r8, r8
0x1404ea1ac: je 0x1404ea235
0x1404ea1b2: movsd xmm2, qword ptr [rip + 0x314aa06]
0x1404ea1ba: movapd xmm1, xmm0
0x1404ea1be: cmpunordsd xmm1, xmm0
0x1404ea1c3: movapd xmm3, xmm1
0x1404ea1c7: andpd xmm3, xmm2
0x1404ea1cb: minsd xmm2, xmm0
0x1404ea1cf: andnpd xmm1, xmm2
0x1404ea1d3: orpd xmm1, xmm3
0x1404ea1d7: movapd xmm0, xmm1
0x1404ea1db: cmp rdx, 2
0x1404ea1df: je 0x1404ea235
0x1404ea1e1: movsd xmm2, qword ptr [rip + 0x314a9df]
0x1404ea1e9: movapd xmm0, xmm1
0x1404ea1ed: cmpunordsd xmm0, xmm1
0x1404ea1f2: movapd xmm3, xmm0
0x1404ea1f6: andpd xmm3, xmm2
0x1404ea1fa: minsd xmm2, xmm1
0x1404ea1fe: andnpd xmm0, xmm2
0x1404ea202: orpd xmm0, xmm3
0x1404ea206: cmp rdx, 3
0x1404ea20a: je 0x1404ea235
0x1404ea20c: movsd xmm1, qword ptr [rip + 0x314a9bc]
0x1404ea214: movapd xmm2, xmm0
0x1404ea218: cmpunordsd xmm2, xmm0
0x1404ea21d: movapd xmm3, xmm2
0x1404ea221: andpd xmm3, xmm1
0x1404ea225: minsd xmm1, xmm0
0x1404ea229: andnpd xmm2, xmm1
0x1404ea22d: orpd xmm2, xmm3
0x1404ea231: movapd xmm0, xmm2
0x1404ea235: test rax, rax
0x1404ea238: je 0x1404ea2dc
0x1404ea23e: movsd xmm1, qword ptr [rip + 0x314a99a]
0x1404ea246: maxsd xmm1, qword ptr [rip + 0x25014b2]
0x1404ea24e: test rcx, rcx
0x1404ea251: je 0x1404ea2e4
0x1404ea257: movsd xmm3, qword ptr [rip + 0x314a989]
0x1404ea25f: movapd xmm2, xmm1
0x1404ea263: cmpunordsd xmm2, xmm1
0x1404ea268: movapd xmm4, xmm2
0x1404ea26c: andpd xmm4, xmm3
0x1404ea270: maxsd xmm3, xmm1
0x1404ea274: andnpd xmm2, xmm3
0x1404ea278: orpd xmm2, xmm4
0x1404ea27c: movapd xmm1, xmm2
0x1404ea280: cmp rax, 2
0x1404ea284: je 0x1404ea2e4
0x1404ea286: movsd xmm3, qword ptr [rip + 0x314a962]
0x1404ea28e: movapd xmm1, xmm2
0x1404ea292: cmpunordsd xmm1, xmm2
0x1404ea297: movapd xmm4, xmm1
0x1404ea29b: andpd xmm4, xmm3
0x1404ea29f: maxsd xmm3, xmm2
0x1404ea2a3: andnpd xmm1, xmm3
0x1404ea2a7: orpd xmm1, xmm4
0x1404ea2ab: cmp rax, 3
0x1404ea2af: je 0x1404ea2e4
0x1404ea2b1: movsd xmm2, qword ptr [rip + 0x314a93f]
0x1404ea2b9: movapd xmm3, xmm1
0x1404ea2bd: cmpunordsd xmm3, xmm1
0x1404ea2c2: movapd xmm4, xmm3
0x1404ea2c6: andpd xmm4, xmm2
0x1404ea2ca: maxsd xmm2, xmm1
0x1404ea2ce: andnpd xmm3, xmm2
0x1404ea2d2: orpd xmm3, xmm4
0x1404ea2d6: movapd xmm1, xmm3
0x1404ea2da: jmp 0x1404ea2e4
0x1404ea2dc: movsd xmm1, qword ptr [rip + 0x250141c]
0x1404ea2e4: subsd xmm0, xmm1
0x1404ea2e8: xorpd xmm1, xmm1
0x1404ea2ec: maxsd xmm0, xmm1
0x1404ea2f0: movq rdx, xmm0
0x1404ea2f5: mov rcx, rdx
0x1404ea2f8: shr rcx, 0x34
0x1404ea2fc: mov r8d, ecx
0x1404ea2ff: and r8d, 0x7ff
0x1404ea306: xor r9d, r9d
0x1404ea309: cmp r8d, 0x3e0
0x1404ea310: jae 0x1404ea31a
0x1404ea312: xor r8d, r8d
0x1404ea315: jmp 0x1404ea450
0x1404ea31a: movabs r10, 0xfffffffffffff
0x1404ea324: mov r11, rdx
0x1404ea327: and r11, r10
0x1404ea32a: movabs rax, 0x10000000000000
0x1404ea334: or rax, r11
0x1404ea337: movzx r8d, r8w
0x1404ea33b: cmp r8d, 0x3ff
0x1404ea342: jae 0x1404ea3b3
0x1404ea344: add cl, 0x2d
0x1404ea347: xor edx, edx
0x1404ea349: shld rdx, rax, cl
0x1404ea34d: shl rax, cl
0x1404ea350: xor r10d, r10d
0x1404ea353: test cl, 0x40
0x1404ea356: cmovne rdx, rax
0x1404ea35a: cmovne rax, r10
0x1404ea35e: imul rcx, rdx, 0x3b9aca00
0x1404ea365: mov edx, 0x3b9aca00
0x1404ea36a: mul rdx
0x1404ea36d: add rdx, rcx
0x1404ea370: mov rcx, rdx
0x1404ea373: shr rcx, 0x20
0x1404ea377: mov r8d, edx
0x1404ea37a: mov r9d, 0x80000000
0x1404ea380: xor r9, r8
0x1404ea383: or r9, rax
0x1404ea386: setne al
0x1404ea389: shr edx, 0x1f
0x1404ea38c: mov r8d, ecx
0x1404ea38f: or r8b, al
0x1404ea392: and r8b, dl
0x1404ea395: movzx r9d, r8b
0x1404ea399: add r9d, ecx
0x1404ea39c: xor r8d, r8d
0x1404ea39f: cmp r9d, 0x3b9aca00
0x1404ea3a6: sete r8b
0x1404ea3aa: cmove r9d, r10d
0x1404ea3ae: jmp 0x1404ea450
0x1404ea3b3: cmp r8d, 0x433
0x1404ea3ba: jae 0x1404ea43a
0x1404ea3bc: lea r8, [rcx + 1]
0x1404ea3c0: mov r9b, 0x33
0x1404ea3c3: sub r9b, cl
0x1404ea3c6: mov ecx, r9d
0x1404ea3c9: shr rax, cl
0x1404ea3cc: mov ecx, r8d
0x1404ea3cf: shl rdx, cl
0x1404ea3d2: and rdx, r10
0x1404ea3d5: mov r8d, 0x3b9aca00
0x1404ea3db: mov rcx, rax
0x1404ea3de: mov rax, rdx
0x1404ea3e1: mul r8
0x1404ea3e4: shl edx, 0xc
0x1404ea3e7: mov r8, rax
0x1404ea3ea: shr r8, 0x34
0x1404ea3ee: or edx, r8d
0x1404ea3f1: movabs r9, 0xffffffffffe00
0x1404ea3fb: and r9, rax
0x1404ea3fe: movabs r10, 0x8000000000000
0x1404ea408: cmp r9, r10
0x1404ea40b: setne r9b
0x1404ea40f: shr rax, 0x33
0x1404ea413: and eax, 1
0x1404ea416: or r8b, r9b
0x1404ea419: and r8b, al
0x1404ea41c: movzx eax, r8b
0x1404ea420: add eax, edx
0x1404ea422: xor r9d, r9d
0x1404ea425: xor r8d, r8d
0x1404ea428: cmp eax, 0x3b9aca00
0x1404ea42d: sete r8b
0x1404ea431: cmovne r9d, eax
0x1404ea435: add r8, rcx
0x1404ea438: jmp 0x1404ea450
0x1404ea43a: cmp r8d, 0x43f
0x1404ea441: jae 0x1404ea793
0x1404ea447: add cl, 0xd
0x1404ea44a: shl rax, cl
0x1404ea44d: mov r8, rax
0x1404ea450: mov byte ptr [rbp + 0x1116], 0
0x1404ea457: mov byte ptr [rbp + 0x1115], 0
0x1404ea45e: mov rcx, rbx
0x1404ea461: mov rdx, qword ptr [rbp + 0x1100]
0x1404ea468: call 0x140bea680
0x1404ea46d: mov byte ptr [rbp + 0x1116], 0
0x1404ea474: mov byte ptr [rbp + 0x1115], 0
0x1404ea47b: call 0x140d635e0
0x1404ea480: mov qword ptr [rip + 0x314a729], rax
0x1404ea487: mov dword ptr [rip + 0x3140c5b], edx
0x1404ea48d: lea rcx, [rbp + 0x788]
0x1404ea494: call 0x1405fdb80
0x1404ea499: mov rdx, qword ptr [rbp + 0x850]
0x1404ea4a0: mov rax, rdx
0x1404ea4a3: neg rax
0x1404ea4a6: jo 0x1404ea4bc
0x1404ea4a8: jae 0x1404ea4bc
0x1404ea4aa: mov rcx, qword ptr [rbp + 0x858]
0x1404ea4b1: mov r8d, 1
0x1404ea4b7: call 0x140613c20
0x1404ea4bc: mov rax, qword ptr [rbp + 0x10a0]
0x1404ea4c3: lock dec qword ptr [rax]
0x1404ea4c7: jne 0x1404ea4d5
0x1404ea4c9: lea rcx, [rbp + 0x10a0]
0x1404ea4d0: call 0x140b5be90
0x1404ea4d5: cmp qword ptr [rbp + 0x1080], 0
0x1404ea4dd: je 0x1404ea526
0x1404ea4df: call 0x140c18130
0x1404ea4e4: mov rcx, rax
0x1404ea4e7: mov rax, qword ptr [rax]
0x1404ea4ea: cmp rax, 1
0x1404ea4ee: jne 0x1404ea568
0x1404ea4f0: add rcx, 8
0x1404ea4f4: cmp qword ptr [rcx], 0
0x1404ea4f8: jne 0x1404ea5a6
0x1404ea4fe: mov qword ptr [rcx], 0xffffffffffffffff
0x1404ea505: mov qword ptr [rbp + 0x10a8], rcx
0x1404ea50c: add rcx, 8
0x1404ea510: mov rdx, qword ptr [rbp + 0x1088]
0x1404ea517: call 0x140c170f0
0x1404ea51c: mov rax, qword ptr [rbp + 0x10a8]
0x1404ea523: inc qword ptr [rax]
0x1404ea526: movaps xmm6, xmmword ptr [rbp + 0x1120]
0x1404ea52d: movaps xmm7, xmmword ptr [rbp + 0x1130]
0x1404ea534: movaps xmm8, xmmword ptr [rbp + 0x1140]
0x1404ea53c: movaps xmm9, xmmword ptr [rbp + 0x1150]
0x1404ea544: movaps xmm10, xmmword ptr [rbp + 0x1160]
0x1404ea54c: movaps xmm11, xmmword ptr [rbp + 0x1170]
0x1404ea554: add rsp, 0x1208
0x1404ea55b: pop rbx
0x1404ea55c: pop rdi
0x1404ea55d: pop rsi
0x1404ea55e: pop r12
0x1404ea560: pop r13
0x1404ea562: pop r14
0x1404ea564: pop r15
0x1404ea566: pop rbp
0x1404ea567: ret 
0x1404ea568: test rax, rax
0x1404ea56b: jne 0x1404ea59a
0x1404ea56d: xor edx, edx
0x1404ea56f: call 0x1428a1640
0x1404ea574: mov rcx, rax
0x1404ea577: test rax, rax
0x1404ea57a: jne 0x1404ea4f4
0x1404ea580: jmp 0x1404ea59a
0x1404ea582: test rax, rax
0x1404ea585: jne 0x1404ea59a
0x1404ea587: xor edx, edx
0x1404ea589: call 0x1428a1640
0x1404ea58e: mov rcx, rax
0x1404ea591: test rax, rax
0x1404ea594: jne 0x1404e825a
0x1404ea59a: lea rcx, [rip + 0x244846f]
0x1404ea5a1: call 0x1428d80d0
0x1404ea5a6: lea rcx, [rip + 0x244850b]
0x1404ea5ad: call 0x1428d9250
0x1404ea5b2: lea rcx, [rip + 0x25015cf]
0x1404ea5b9: call 0x1428d9310
0x1404ea5be: jmp 0x1404ea7cd
0x1404ea5c3: lea rcx, [rip + 0x25015d6]
0x1404ea5ca: lea r8, [rip + 0x25015df]
0x1404ea5d1: mov edx, 0xa
0x1404ea5d6: call 0x1428d9330
0x1404ea5db: jmp 0x1404ea7cd
0x1404ea5e0: mov byte ptr [rbp + 0x1117], 0
0x1404ea5e7: lea rcx, [rip + 0x2501652]
0x1404ea5ee: call 0x1428d9310
0x1404ea5f3: jmp 0x1404ea7cd
0x1404ea5f8: mov byte ptr [rbp + 0x1114], r14b
0x1404ea5ff: mov byte ptr [rbp + 0x1113], 0
0x1404ea606: lea rcx, [rip + 0x2501843]
0x1404ea60d: call 0x1428d9310
0x1404ea612: jmp 0x1404ea7cd
0x1404ea617: mov byte ptr [rbp + 0x1117], 1
0x1404ea61e: lea rcx, [rip + 0x25015a3]
0x1404ea625: lea r8, [rip + 0x25015c4]
0x1404ea62c: mov edx, 0x28
0x1404ea631: call 0x1428d9330
0x1404ea636: jmp 0x1404ea7cd
0x1404ea63b: mov byte ptr [rbp + 0x1117], 0
0x1404ea642: lea rax, [rip + 0x25015df]
0x1404ea649: mov qword ptr [rsp + 0x20], rax
0x1404ea64e: lea rcx, [rip + 0x25015b3]
0x1404ea655: lea r9, [rip + 0x2501204]
0x1404ea65c: lea r8, [rbp + 0x1000]
0x1404ea663: mov edx, 0x1b
0x1404ea668: call 0x1428d9760
0x1404ea66d: jmp 0x1404ea7cd
0x1404ea672: lea rax, [rbp + 0x1070]
0x1404ea679: mov qword ptr [rbp + 0xb00], rax
0x1404ea680: lea rax, [rip - 0x26f7]
0x1404ea687: mov qword ptr [rbp + 0xb08], rax
0x1404ea68e: lea rax, [rip + 0x2501813]
0x1404ea695: mov qword ptr [rbp + 0xd20], rax
0x1404ea69c: mov qword ptr [rbp + 0xd28], 1
0x1404ea6a7: mov qword ptr [rbp + 0xd40], 0
0x1404ea6b2: lea rax, [rbp + 0xb00]
0x1404ea6b9: mov qword ptr [rbp + 0xd30], rax
0x1404ea6c0: mov qword ptr [rbp + 0xd38], 1
0x1404ea6cb: mov byte ptr [rbp + 0x1117], 0
0x1404ea6d2: lea rdx, [rip + 0x25017df]
0x1404ea6d9: lea rcx, [rbp + 0xd20]
0x1404ea6e0: call 0x1428d9390
0x1404ea6e5: jmp 0x1404ea7cd
0x1404ea6ea: mov byte ptr [rbp + 0x1117], 0
0x1404ea6f1: lea rax, [rip + 0x2501690]
0x1404ea6f8: mov qword ptr [rsp + 0x20], rax
0x1404ea6fd: lea rcx, [rip + 0x250164c]
0x1404ea704: lea r9, [rip + 0x2501155]
0x1404ea70b: lea r8, [rbp + 0x1000]
0x1404ea712: mov edx, 0x37
0x1404ea717: call 0x1428d9760
0x1404ea71c: jmp 0x1404ea7cd
0x1404ea721: mov qword ptr [rbp + 0xfc0], 0
0x1404ea72c: mov qword ptr [rbp + 0x750], rsi
0x1404ea733: lea rax, [rip - 0x274a]
0x1404ea73a: mov qword ptr [rbp + 0x758], rax
0x1404ea741: lea rax, [rip + 0x25016e0]
0x1404ea748: mov qword ptr [rbp + 0xd20], rax
0x1404ea74f: mov qword ptr [rbp + 0xd28], 1
0x1404ea75a: mov qword ptr [rbp + 0xd40], 0
0x1404ea765: mov qword ptr [rbp + 0xd30], rdi
0x1404ea76c: mov qword ptr [rbp + 0xd38], 1
0x1404ea777: mov byte ptr [rbp + 0x110a], 1
0x1404ea77e: lea rdx, [rip + 0x25016b3]
0x1404ea785: lea rcx, [rbp + 0xd20]
0x1404ea78c: call 0x1428d9390
0x1404ea791: jmp 0x1404ea7cd
0x1404ea793: lea rax, [rip + 0x250101e]
0x1404ea79a: mov qword ptr [rbp + 0xd20], rax
0x1404ea7a1: mov qword ptr [rbp + 0xd28], 0x48
0x1404ea7ac: mov byte ptr [rbp + 0x1116], 0
0x1404ea7b3: mov byte ptr [rbp + 0x1115], 0
0x1404ea7ba: lea rdx, [rip + 0x2501087]
0x1404ea7c1: lea rcx, [rbp + 0xd20]
0x1404ea7c8: call 0x1428ce090
0x1404ea7cd: ud2 
0x1404ea7cf: nop 
0x1404ea7d0: mov qword ptr [rsp + 0x10], rdx
0x1404ea7d5: push rbp
0x1404ea7d6: push r15
0x1404ea7d8: push r14
0x1404ea7da: push r13
0x1404ea7dc: push r12
0x1404ea7de: push rsi
0x1404ea7df: push rdi
0x1404ea7e0: push rbx
0x1404ea7e1: sub rsp, 0x98
0x1404ea7e8: lea rbp, [rdx + 0x80]
0x1404ea7ef: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ea7f5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ea7fb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ea801: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ea807: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ea80c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ea814: mov rax, qword ptr [rbp + 0x1100]
0x1404ea81b: inc qword ptr [rax]
0x1404ea81e: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ea826: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ea82b: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ea831: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ea837: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ea83d: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ea843: add rsp, 0x98
0x1404ea84a: pop rbx
0x1404ea84b: pop rdi
0x1404ea84c: pop rsi
0x1404ea84d: pop r12
0x1404ea84f: pop r13
0x1404ea851: pop r14
0x1404ea853: pop r15
0x1404ea855: pop rbp
0x1404ea856: ret 
0x1404ea857: nop word ptr [rax + rax]
0x1404ea860: mov qword ptr [rsp + 0x10], rdx
0x1404ea865: push rbp
0x1404ea866: push r15
0x1404ea868: push r14
0x1404ea86a: push r13
0x1404ea86c: push r12
0x1404ea86e: push rsi
0x1404ea86f: push rdi
0x1404ea870: push rbx
0x1404ea871: sub rsp, 0x98
0x1404ea878: lea rbp, [rdx + 0x80]
0x1404ea87f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ea885: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ea88b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ea891: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ea897: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ea89c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ea8a4: cmp qword ptr [rbp + 0x1080], 0
0x1404ea8ac: je 0x1404ea8dd
0x1404ea8ae: lea rax, [rbp + 0x1088]
0x1404ea8b5: mov qword ptr [rbp + 0xae8], rax
0x1404ea8bc: lea rax, [rbp + 0xae8]
0x1404ea8c3: mov qword ptr [rbp + 0xae0], rax
0x1404ea8ca: lea rcx, [rip + 0x2501047]
0x1404ea8d1: lea rdx, [rbp + 0xae0]
0x1404ea8d8: call 0x1400a2580
0x1404ea8dd: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ea8e5: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ea8ea: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ea8f0: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ea8f6: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ea8fc: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ea902: add rsp, 0x98
0x1404ea909: pop rbx
0x1404ea90a: pop rdi
0x1404ea90b: pop rsi
0x1404ea90c: pop r12
0x1404ea90e: pop r13
0x1404ea910: pop r14
0x1404ea912: pop r15
0x1404ea914: pop rbp
0x1404ea915: ret 
0x1404ea916: nop word ptr cs:[rax + rax]
0x1404ea920: mov qword ptr [rsp + 0x10], rdx
0x1404ea925: push rbp
0x1404ea926: push r15
0x1404ea928: push r14
0x1404ea92a: push r13
0x1404ea92c: push r12
0x1404ea92e: push rsi
0x1404ea92f: push rdi
0x1404ea930: push rbx
0x1404ea931: sub rsp, 0x98
0x1404ea938: lea rbp, [rdx + 0x80]
0x1404ea93f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ea945: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ea94b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ea951: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ea957: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ea95c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ea964: mov rax, qword ptr [rbp + 0x10a0]
0x1404ea96b: lock dec qword ptr [rax]
0x1404ea96f: jne 0x1404ea97d
0x1404ea971: lea rcx, [rbp + 0x10a0]
0x1404ea978: call 0x140b5be90
0x1404ea97d: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ea985: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ea98a: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ea990: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ea996: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ea99c: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ea9a2: add rsp, 0x98
0x1404ea9a9: pop rbx
0x1404ea9aa: pop rdi
0x1404ea9ab: pop rsi
0x1404ea9ac: pop r12
0x1404ea9ae: pop r13
0x1404ea9b0: pop r14
0x1404ea9b2: pop r15
0x1404ea9b4: pop rbp
0x1404ea9b5: ret 
0x1404ea9b6: nop word ptr cs:[rax + rax]
0x1404ea9c0: mov qword ptr [rsp + 0x10], rdx
0x1404ea9c5: push rbp
0x1404ea9c6: push r15
0x1404ea9c8: push r14
0x1404ea9ca: push r13
0x1404ea9cc: push r12
0x1404ea9ce: push rsi
0x1404ea9cf: push rdi
0x1404ea9d0: push rbx
0x1404ea9d1: sub rsp, 0x98
0x1404ea9d8: lea rbp, [rdx + 0x80]
0x1404ea9df: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ea9e5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ea9eb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ea9f1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ea9f7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ea9fc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eaa04: movabs rax, 0x8000000000000002
0x1404eaa0e: cmp qword ptr [rbp + 0x1100], rax
0x1404eaa15: jl 0x1404eaa3a
0x1404eaa17: cmp qword ptr [rbp + 0x1100], 0
0x1404eaa1f: je 0x1404eaa3a
0x1404eaa21: mov r8d, 1
0x1404eaa27: mov rcx, qword ptr [rbp + 0x10f0]
0x1404eaa2e: mov rdx, qword ptr [rbp + 0x1100]
0x1404eaa35: call 0x140613c20
0x1404eaa3a: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eaa42: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eaa47: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eaa4d: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eaa53: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eaa59: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eaa5f: add rsp, 0x98
0x1404eaa66: pop rbx
0x1404eaa67: pop rdi
0x1404eaa68: pop rsi
0x1404eaa69: pop r12
0x1404eaa6b: pop r13
0x1404eaa6d: pop r14
0x1404eaa6f: pop r15
0x1404eaa71: pop rbp
0x1404eaa72: ret 
0x1404eaa73: nop word ptr cs:[rax + rax]
0x1404eaa80: mov qword ptr [rsp + 0x10], rdx
0x1404eaa85: push rbp
0x1404eaa86: push r15
0x1404eaa88: push r14
0x1404eaa8a: push r13
0x1404eaa8c: push r12
0x1404eaa8e: push rsi
0x1404eaa8f: push rdi
0x1404eaa90: push rbx
0x1404eaa91: sub rsp, 0x98
0x1404eaa98: lea rbp, [rdx + 0x80]
0x1404eaa9f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eaaa5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eaaab: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eaab1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eaab7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eaabc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eaac4: lea rcx, [rbp + 0x310]
0x1404eaacb: call 0x1405d1240
0x1404eaad0: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eaad8: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eaadd: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eaae3: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eaae9: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eaaef: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eaaf5: add rsp, 0x98
0x1404eaafc: pop rbx
0x1404eaafd: pop rdi
0x1404eaafe: pop rsi
0x1404eaaff: pop r12
0x1404eab01: pop r13
0x1404eab03: pop r14
0x1404eab05: pop r15
0x1404eab07: pop rbp
0x1404eab08: ret 
0x1404eab09: nop dword ptr [rax]
0x1404eab10: mov qword ptr [rsp + 0x10], rdx
0x1404eab15: push rbp
0x1404eab16: push r15
0x1404eab18: push r14
0x1404eab1a: push r13
0x1404eab1c: push r12
0x1404eab1e: push rsi
0x1404eab1f: push rdi
0x1404eab20: push rbx
0x1404eab21: sub rsp, 0x98
0x1404eab28: lea rbp, [rdx + 0x80]
0x1404eab2f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eab35: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eab3b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eab41: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eab47: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eab4c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eab54: mov byte ptr [rbp + 0x1117], 0
0x1404eab5b: lea rcx, [rbp + 0xd20]
0x1404eab62: call 0x140609f60
0x1404eab67: mov byte ptr [rbp + 0x1117], 0
0x1404eab6e: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eab76: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eab7b: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eab81: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eab87: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eab8d: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eab93: add rsp, 0x98
0x1404eab9a: pop rbx
0x1404eab9b: pop rdi
0x1404eab9c: pop rsi
0x1404eab9d: pop r12
0x1404eab9f: pop r13
0x1404eaba1: pop r14
0x1404eaba3: pop r15
0x1404eaba5: pop rbp
0x1404eaba6: ret 
0x1404eaba7: nop word ptr [rax + rax]
0x1404eabb0: mov qword ptr [rsp + 0x10], rdx
0x1404eabb5: push rbp
0x1404eabb6: push r15
0x1404eabb8: push r14
0x1404eabba: push r13
0x1404eabbc: push r12
0x1404eabbe: push rsi
0x1404eabbf: push rdi
0x1404eabc0: push rbx
0x1404eabc1: sub rsp, 0x98
0x1404eabc8: lea rbp, [rdx + 0x80]
0x1404eabcf: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eabd5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eabdb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eabe1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eabe7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eabec: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eabf4: mov byte ptr [rbp + 0x1117], 0
0x1404eabfb: lea rcx, [rbp + 0xb00]
0x1404eac02: call 0x140609f60
0x1404eac07: mov byte ptr [rbp + 0x1117], 0
0x1404eac0e: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eac16: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eac1b: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eac21: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eac27: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eac2d: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eac33: add rsp, 0x98
0x1404eac3a: pop rbx
0x1404eac3b: pop rdi
0x1404eac3c: pop rsi
0x1404eac3d: pop r12
0x1404eac3f: pop r13
0x1404eac41: pop r14
0x1404eac43: pop r15
0x1404eac45: pop rbp
0x1404eac46: ret 
0x1404eac47: nop word ptr [rax + rax]
0x1404eac50: mov qword ptr [rsp + 0x10], rdx
0x1404eac55: push rbp
0x1404eac56: push r15
0x1404eac58: push r14
0x1404eac5a: push r13
0x1404eac5c: push r12
0x1404eac5e: push rsi
0x1404eac5f: push rdi
0x1404eac60: push rbx
0x1404eac61: sub rsp, 0x98
0x1404eac68: lea rbp, [rdx + 0x80]
0x1404eac6f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eac75: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eac7b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eac81: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eac87: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eac8c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eac94: mov byte ptr [rbp + 0x1117], 0
0x1404eac9b: lea rcx, [rbp + 0xb00]
0x1404eaca2: mov r8d, 0x68
0x1404eaca8: mov r9d, 0x10
0x1404eacae: lea rdx, [rbp + 0xb20]
0x1404eacb5: call 0x14004ceb0
0x1404eacba: mov byte ptr [rbp + 0x1117], 0
0x1404eacc1: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eacc9: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eacce: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eacd4: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eacda: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eace0: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eace6: add rsp, 0x98
0x1404eaced: pop rbx
0x1404eacee: pop rdi
0x1404eacef: pop rsi
0x1404eacf0: pop r12
0x1404eacf2: pop r13
0x1404eacf4: pop r14
0x1404eacf6: pop r15
0x1404eacf8: pop rbp
0x1404eacf9: ret 
0x1404eacfa: nop word ptr [rax + rax]
0x1404ead00: mov qword ptr [rsp + 0x10], rdx
0x1404ead05: push rbp
0x1404ead06: push r15
0x1404ead08: push r14
0x1404ead0a: push r13
0x1404ead0c: push r12
0x1404ead0e: push rsi
0x1404ead0f: push rdi
0x1404ead10: push rbx
0x1404ead11: sub rsp, 0x98
0x1404ead18: lea rbp, [rdx + 0x80]
0x1404ead1f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ead25: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ead2b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ead31: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ead37: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ead3c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ead44: mov byte ptr [rbp + 0x1117], 0
0x1404ead4b: lea rcx, [rbp + 0xb00]
0x1404ead52: mov r8d, 0x68
0x1404ead58: mov r9d, 0x10
0x1404ead5e: lea rdx, [rbp + 0xb20]
0x1404ead65: call 0x14004ceb0
0x1404ead6a: mov byte ptr [rbp + 0x1117], 0
0x1404ead71: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ead79: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ead7e: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ead84: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ead8a: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ead90: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ead96: add rsp, 0x98
0x1404ead9d: pop rbx
0x1404ead9e: pop rdi
0x1404ead9f: pop rsi
0x1404eada0: pop r12
0x1404eada2: pop r13
0x1404eada4: pop r14
0x1404eada6: pop r15
0x1404eada8: pop rbp
0x1404eada9: ret 
0x1404eadaa: nop word ptr [rax + rax]
0x1404eadb0: mov qword ptr [rsp + 0x10], rdx
0x1404eadb5: push rbp
0x1404eadb6: push r15
0x1404eadb8: push r14
0x1404eadba: push r13
0x1404eadbc: push r12
0x1404eadbe: push rsi
0x1404eadbf: push rdi
0x1404eadc0: push rbx
0x1404eadc1: sub rsp, 0x98
0x1404eadc8: lea rbp, [rdx + 0x80]
0x1404eadcf: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eadd5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eaddb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eade1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eade7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eadec: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eadf4: lea rcx, [rbp + 0xb00]
0x1404eadfb: call 0x1405dab70
0x1404eae00: mov byte ptr [rbp + 0x1117], 0
0x1404eae07: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eae0f: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eae14: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eae1a: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eae20: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eae26: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eae2c: add rsp, 0x98
0x1404eae33: pop rbx
0x1404eae34: pop rdi
0x1404eae35: pop rsi
0x1404eae36: pop r12
0x1404eae38: pop r13
0x1404eae3a: pop r14
0x1404eae3c: pop r15
0x1404eae3e: pop rbp
0x1404eae3f: ret 
0x1404eae40: mov qword ptr [rsp + 0x10], rdx
0x1404eae45: push rbp
0x1404eae46: push r15
0x1404eae48: push r14
0x1404eae4a: push r13
0x1404eae4c: push r12
0x1404eae4e: push rsi
0x1404eae4f: push rdi
0x1404eae50: push rbx
0x1404eae51: sub rsp, 0x98
0x1404eae58: lea rbp, [rdx + 0x80]
0x1404eae5f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eae65: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eae6b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eae71: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eae77: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eae7c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eae84: movzx ebx, byte ptr [rbp + 0x1111]
0x1404eae8b: movzx esi, byte ptr [rbp + 0x1112]
0x1404eae92: lea rdx, [rbp + 0xac0]
0x1404eae99: lea rcx, [rbp + 0xaa0]
0x1404eaea0: mov r8d, 0x48
0x1404eaea6: mov r9d, 0x10
0x1404eaeac: call 0x14004f690
0x1404eaeb1: mov byte ptr [rbp + 0x1109], sil
0x1404eaeb8: mov byte ptr [rbp + 0x1108], bl
0x1404eaebe: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eaec6: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eaecb: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eaed1: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eaed7: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eaedd: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eaee3: add rsp, 0x98
0x1404eaeea: pop rbx
0x1404eaeeb: pop rdi
0x1404eaeec: pop rsi
0x1404eaeed: pop r12
0x1404eaeef: pop r13
0x1404eaef1: pop r14
0x1404eaef3: pop r15
0x1404eaef5: pop rbp
0x1404eaef6: ret 
0x1404eaef7: nop word ptr [rax + rax]
0x1404eaf00: mov qword ptr [rsp + 0x10], rdx
0x1404eaf05: push rbp
0x1404eaf06: push r15
0x1404eaf08: push r14
0x1404eaf0a: push r13
0x1404eaf0c: push r12
0x1404eaf0e: push rsi
0x1404eaf0f: push rdi
0x1404eaf10: push rbx
0x1404eaf11: sub rsp, 0x98
0x1404eaf18: lea rbp, [rdx + 0x80]
0x1404eaf1f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eaf25: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eaf2b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eaf31: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eaf37: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eaf3c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eaf44: cmp byte ptr [rbp + 0x110b], 0
0x1404eaf4b: je 0x1404eaf81
0x1404eaf4d: mov byte ptr [rbp + 0x110f], 0
0x1404eaf54: mov byte ptr [rbp + 0x110e], 1
0x1404eaf5b: lea rcx, [rbp + 0x930]
0x1404eaf62: call 0x1405f9780
0x1404eaf67: mov byte ptr [rbp + 0x110f], 0
0x1404eaf6e: mov byte ptr [rbp + 0x110e], 1
0x1404eaf75: lea rcx, [rbp + 0xa60]
0x1404eaf7c: call 0x1405ee5e0
0x1404eaf81: mov byte ptr [rbp + 0x110f], 0
0x1404eaf88: mov byte ptr [rbp + 0x110e], 1
0x1404eaf8f: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eaf97: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eaf9c: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eafa2: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eafa8: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eafae: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eafb4: add rsp, 0x98
0x1404eafbb: pop rbx
0x1404eafbc: pop rdi
0x1404eafbd: pop rsi
0x1404eafbe: pop r12
0x1404eafc0: pop r13
0x1404eafc2: pop r14
0x1404eafc4: pop r15
0x1404eafc6: pop rbp
0x1404eafc7: ret 
0x1404eafc8: nop dword ptr [rax + rax]
0x1404eafd0: mov qword ptr [rsp + 0x10], rdx
0x1404eafd5: push rbp
0x1404eafd6: push r15
0x1404eafd8: push r14
0x1404eafda: push r13
0x1404eafdc: push r12
0x1404eafde: push rsi
0x1404eafdf: push rdi
0x1404eafe0: push rbx
0x1404eafe1: sub rsp, 0x98
0x1404eafe8: lea rbp, [rdx + 0x80]
0x1404eafef: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eaff5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eaffb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb001: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb007: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb00c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb014: movzx ebx, byte ptr [rbp + 0x110a]
0x1404eb01b: cmp qword ptr [rbp + 0xfc0], 0
0x1404eb023: je 0x1404eb044
0x1404eb025: lea rdx, [rbp + 0xfe0]
0x1404eb02c: lea rcx, [rbp + 0xfc0]
0x1404eb033: mov r8d, 0x48
0x1404eb039: mov r9d, 0x10
0x1404eb03f: call 0x14004f690
0x1404eb044: mov byte ptr [rbp + 0x110f], 0
0x1404eb04b: and bl, 1
0x1404eb04e: mov byte ptr [rbp + 0x110e], bl
0x1404eb054: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb05c: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb061: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb067: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb06d: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb073: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb079: add rsp, 0x98
0x1404eb080: pop rbx
0x1404eb081: pop rdi
0x1404eb082: pop rsi
0x1404eb083: pop r12
0x1404eb085: pop r13
0x1404eb087: pop r14
0x1404eb089: pop r15
0x1404eb08b: pop rbp
0x1404eb08c: ret 
0x1404eb08d: nop dword ptr [rax]
0x1404eb090: mov qword ptr [rsp + 0x10], rdx
0x1404eb095: push rbp
0x1404eb096: push r15
0x1404eb098: push r14
0x1404eb09a: push r13
0x1404eb09c: push r12
0x1404eb09e: push rsi
0x1404eb09f: push rdi
0x1404eb0a0: push rbx
0x1404eb0a1: sub rsp, 0x98
0x1404eb0a8: lea rbp, [rdx + 0x80]
0x1404eb0af: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb0b5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb0bb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb0c1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb0c7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb0cc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb0d4: movzx ebx, byte ptr [rbp + 0x110f]
0x1404eb0db: cmp byte ptr [rbp + 0x110e], 0
0x1404eb0e2: je 0x1404eb136
0x1404eb0e4: lea rcx, [rbp + 0x1000]
0x1404eb0eb: call 0x1405ffcc0
0x1404eb0f0: mov byte ptr [rbp + 0x1114], bl
0x1404eb0f6: mov byte ptr [rbp + 0x1113], 1
0x1404eb0fd: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb105: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb10a: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb110: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb116: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb11c: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb122: add rsp, 0x98
0x1404eb129: pop rbx
0x1404eb12a: pop rdi
0x1404eb12b: pop rsi
0x1404eb12c: pop r12
0x1404eb12e: pop r13
0x1404eb130: pop r14
0x1404eb132: pop r15
0x1404eb134: pop rbp
0x1404eb135: ret 
0x1404eb136: mov byte ptr [rbp + 0x1114], bl
0x1404eb13c: mov byte ptr [rbp + 0x1113], 1
0x1404eb143: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb14b: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb150: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb156: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb15c: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb162: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb168: add rsp, 0x98
0x1404eb16f: pop rbx
0x1404eb170: pop rdi
0x1404eb171: pop rsi
0x1404eb172: pop r12
0x1404eb174: pop r13
0x1404eb176: pop r14
0x1404eb178: pop r15
0x1404eb17a: pop rbp
0x1404eb17b: ret 
0x1404eb17c: nop dword ptr [rax]
0x1404eb180: mov qword ptr [rsp + 0x10], rdx
0x1404eb185: push rbp
0x1404eb186: push r15
0x1404eb188: push r14
0x1404eb18a: push r13
0x1404eb18c: push r12
0x1404eb18e: push rsi
0x1404eb18f: push rdi
0x1404eb190: push rbx
0x1404eb191: sub rsp, 0x98
0x1404eb198: lea rbp, [rdx + 0x80]
0x1404eb19f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb1a5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb1ab: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb1b1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb1b7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb1bc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb1c4: mov byte ptr [rbp + 0x1112], 0
0x1404eb1cb: mov byte ptr [rbp + 0x1111], 0
0x1404eb1d2: lea rcx, [rbp + 0xa28]
0x1404eb1d9: call 0x140be24b0
0x1404eb1de: mov byte ptr [rbp + 0x1112], 0
0x1404eb1e5: mov byte ptr [rbp + 0x1111], 0
0x1404eb1ec: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb1f4: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb1f9: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb1ff: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb205: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb20b: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb211: add rsp, 0x98
0x1404eb218: pop rbx
0x1404eb219: pop rdi
0x1404eb21a: pop rsi
0x1404eb21b: pop r12
0x1404eb21d: pop r13
0x1404eb21f: pop r14
0x1404eb221: pop r15
0x1404eb223: pop rbp
0x1404eb224: ret 
0x1404eb225: nop word ptr cs:[rax + rax]
0x1404eb230: mov qword ptr [rsp + 0x10], rdx
0x1404eb235: push rbp
0x1404eb236: push r15
0x1404eb238: push r14
0x1404eb23a: push r13
0x1404eb23c: push r12
0x1404eb23e: push rsi
0x1404eb23f: push rdi
0x1404eb240: push rbx
0x1404eb241: sub rsp, 0x98
0x1404eb248: lea rbp, [rdx + 0x80]
0x1404eb24f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb255: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb25b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb261: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb267: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb26c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb274: mov rdx, qword ptr [rbp + 0x850]
0x1404eb27b: lea rax, [rdx*2]
0x1404eb283: test rax, rax
0x1404eb286: je 0x1404eb29a
0x1404eb288: mov rcx, qword ptr [rbp + 0x858]
0x1404eb28f: mov r8d, 1
0x1404eb295: call 0x140613c20
0x1404eb29a: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb2a2: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb2a7: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb2ad: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb2b3: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb2b9: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb2bf: add rsp, 0x98
0x1404eb2c6: pop rbx
0x1404eb2c7: pop rdi
0x1404eb2c8: pop rsi
0x1404eb2c9: pop r12
0x1404eb2cb: pop r13
0x1404eb2cd: pop r14
0x1404eb2cf: pop r15
0x1404eb2d1: pop rbp
0x1404eb2d2: ret 
0x1404eb2d3: nop word ptr cs:[rax + rax]
0x1404eb2e0: mov qword ptr [rsp + 0x10], rdx
0x1404eb2e5: push rbp
0x1404eb2e6: push r15
0x1404eb2e8: push r14
0x1404eb2ea: push r13
0x1404eb2ec: push r12
0x1404eb2ee: push rsi
0x1404eb2ef: push rdi
0x1404eb2f0: push rbx
0x1404eb2f1: sub rsp, 0x98
0x1404eb2f8: lea rbp, [rdx + 0x80]
0x1404eb2ff: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb305: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb30b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb311: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb317: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb31c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb324: mov rax, qword ptr [rbp + 0x10a8]
0x1404eb32b: inc qword ptr [rax]
0x1404eb32e: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb336: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb33b: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb341: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb347: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb34d: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb353: add rsp, 0x98
0x1404eb35a: pop rbx
0x1404eb35b: pop rdi
0x1404eb35c: pop rsi
0x1404eb35d: pop r12
0x1404eb35f: pop r13
0x1404eb361: pop r14
0x1404eb363: pop r15
0x1404eb365: pop rbp
0x1404eb366: ret 
0x1404eb367: nop word ptr [rax + rax]
0x1404eb370: mov qword ptr [rsp + 0x10], rdx
0x1404eb375: push rbp
0x1404eb376: push r15
0x1404eb378: push r14
0x1404eb37a: push r13
0x1404eb37c: push r12
0x1404eb37e: push rsi
0x1404eb37f: push rdi
0x1404eb380: push rbx
0x1404eb381: sub rsp, 0x98
0x1404eb388: lea rbp, [rdx + 0x80]
0x1404eb38f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb395: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb39b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb3a1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb3a7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb3ac: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb3b4: movzx ebx, byte ptr [rbp + 0x1113]
0x1404eb3bb: test byte ptr [rbp + 0x1114], 1
0x1404eb3c2: je 0x1404eb419
0x1404eb3c4: lea rcx, [rbp + 0xa28]
0x1404eb3cb: call 0x1405ee5e0
0x1404eb3d0: mov byte ptr [rbp + 0x1112], 0
0x1404eb3d7: and bl, 1
0x1404eb3da: mov byte ptr [rbp + 0x1111], bl
0x1404eb3e0: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb3e8: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb3ed: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb3f3: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb3f9: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb3ff: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb405: add rsp, 0x98
0x1404eb40c: pop rbx
0x1404eb40d: pop rdi
0x1404eb40e: pop rsi
0x1404eb40f: pop r12
0x1404eb411: pop r13
0x1404eb413: pop r14
0x1404eb415: pop r15
0x1404eb417: pop rbp
0x1404eb418: ret 
0x1404eb419: mov byte ptr [rbp + 0x1112], 0
0x1404eb420: and bl, 1
0x1404eb423: mov byte ptr [rbp + 0x1111], bl
0x1404eb429: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb431: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb436: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb43c: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb442: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb448: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb44e: add rsp, 0x98
0x1404eb455: pop rbx
0x1404eb456: pop rdi
0x1404eb457: pop rsi
0x1404eb458: pop r12
0x1404eb45a: pop r13
0x1404eb45c: pop r14
0x1404eb45e: pop r15
0x1404eb460: pop rbp
0x1404eb461: ret 
0x1404eb462: nop word ptr cs:[rax + rax]
0x1404eb470: mov qword ptr [rsp + 0x10], rdx
0x1404eb475: push rbp
0x1404eb476: push r15
0x1404eb478: push r14
0x1404eb47a: push r13
0x1404eb47c: push r12
0x1404eb47e: push rsi
0x1404eb47f: push rdi
0x1404eb480: push rbx
0x1404eb481: sub rsp, 0x98
0x1404eb488: lea rbp, [rdx + 0x80]
0x1404eb48f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb495: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb49b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb4a1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb4a7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb4ac: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb4b4: movzx ebx, byte ptr [rbp + 0x1108]
0x1404eb4bb: cmp byte ptr [rbp + 0x1109], 0
0x1404eb4c2: je 0x1404eb519
0x1404eb4c4: lea rcx, [rbp + 0x530]
0x1404eb4cb: call 0x1405ef210
0x1404eb4d0: and bl, 1
0x1404eb4d3: mov byte ptr [rbp + 0x1116], bl
0x1404eb4d9: mov byte ptr [rbp + 0x1115], 0
0x1404eb4e0: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb4e8: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb4ed: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb4f3: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb4f9: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb4ff: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb505: add rsp, 0x98
0x1404eb50c: pop rbx
0x1404eb50d: pop rdi
0x1404eb50e: pop rsi
0x1404eb50f: pop r12
0x1404eb511: pop r13
0x1404eb513: pop r14
0x1404eb515: pop r15
0x1404eb517: pop rbp
0x1404eb518: ret 
0x1404eb519: and bl, 1
0x1404eb51c: mov byte ptr [rbp + 0x1116], bl
0x1404eb522: mov byte ptr [rbp + 0x1115], 0
0x1404eb529: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb531: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb536: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb53c: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb542: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb548: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb54e: add rsp, 0x98
0x1404eb555: pop rbx
0x1404eb556: pop rdi
0x1404eb557: pop rsi
0x1404eb558: pop r12
0x1404eb55a: pop r13
0x1404eb55c: pop r14
0x1404eb55e: pop r15
0x1404eb560: pop rbp
0x1404eb561: ret 
0x1404eb562: nop word ptr cs:[rax + rax]
0x1404eb570: mov qword ptr [rsp + 0x10], rdx
0x1404eb575: push rbp
0x1404eb576: push r15
0x1404eb578: push r14
0x1404eb57a: push r13
0x1404eb57c: push r12
0x1404eb57e: push rsi
0x1404eb57f: push rdi
0x1404eb580: push rbx
0x1404eb581: sub rsp, 0x98
0x1404eb588: lea rbp, [rdx + 0x80]
0x1404eb58f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb595: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb59b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb5a1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb5a7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb5ac: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb5b4: cmp byte ptr [rbp + 0x110c], 0
0x1404eb5bb: je 0x1404eb625
0x1404eb5bd: cmp byte ptr [rbp + 0x1028], 2
0x1404eb5c4: je 0x1404eb5d2
0x1404eb5c6: lea rcx, [rbp + 0x1000]
0x1404eb5cd: call 0x1405f9780
0x1404eb5d2: lea rcx, [rbp + 0xd20]
0x1404eb5d9: call 0x1405dab70
0x1404eb5de: mov byte ptr [rbp + 0x1116], 1
0x1404eb5e5: mov byte ptr [rbp + 0x1115], 0
0x1404eb5ec: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb5f4: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb5f9: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb5ff: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb605: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb60b: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb611: add rsp, 0x98
0x1404eb618: pop rbx
0x1404eb619: pop rdi
0x1404eb61a: pop rsi
0x1404eb61b: pop r12
0x1404eb61d: pop r13
0x1404eb61f: pop r14
0x1404eb621: pop r15
0x1404eb623: pop rbp
0x1404eb624: ret 
0x1404eb625: mov byte ptr [rbp + 0x1116], 1
0x1404eb62c: mov byte ptr [rbp + 0x1115], 0
0x1404eb633: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb63b: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb640: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb646: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb64c: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb652: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb658: add rsp, 0x98
0x1404eb65f: pop rbx
0x1404eb660: pop rdi
0x1404eb661: pop rsi
0x1404eb662: pop r12
0x1404eb664: pop r13
0x1404eb666: pop r14
0x1404eb668: pop r15
0x1404eb66a: pop rbp
0x1404eb66b: ret 
0x1404eb66c: nop dword ptr [rax]
0x1404eb670: mov qword ptr [rsp + 0x10], rdx
0x1404eb675: push rbp
0x1404eb676: push r15
0x1404eb678: push r14
0x1404eb67a: push r13
0x1404eb67c: push r12
0x1404eb67e: push rsi
0x1404eb67f: push rdi
0x1404eb680: push rbx
0x1404eb681: sub rsp, 0x98
0x1404eb688: lea rbp, [rdx + 0x80]
0x1404eb68f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb695: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb69b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb6a1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb6a7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb6ac: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb6b4: mov byte ptr [rbp + 0x110d], 1
0x1404eb6bb: lea rcx, [rbp + 0x960]
0x1404eb6c2: call 0x1405d5690
0x1404eb6c7: mov byte ptr [rbp + 0x110d], 1
0x1404eb6ce: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb6d6: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb6db: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb6e1: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb6e7: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb6ed: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb6f3: add rsp, 0x98
0x1404eb6fa: pop rbx
0x1404eb6fb: pop rdi
0x1404eb6fc: pop rsi
0x1404eb6fd: pop r12
0x1404eb6ff: pop r13
0x1404eb701: pop r14
0x1404eb703: pop r15
0x1404eb705: pop rbp
0x1404eb706: ret 
0x1404eb707: nop word ptr [rax + rax]
0x1404eb710: mov qword ptr [rsp + 0x10], rdx
0x1404eb715: push rbp
0x1404eb716: push r15
0x1404eb718: push r14
0x1404eb71a: push r13
0x1404eb71c: push r12
0x1404eb71e: push rsi
0x1404eb71f: push rdi
0x1404eb720: push rbx
0x1404eb721: sub rsp, 0x98
0x1404eb728: lea rbp, [rdx + 0x80]
0x1404eb72f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb735: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb73b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb741: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb747: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb74c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb754: mov rax, qword ptr [rbp + 0x9c0]
0x1404eb75b: test rax, rax
0x1404eb75e: je 0x1404eb77a
0x1404eb760: mov rcx, qword ptr [rbp + 0x9c8]
0x1404eb767: shl rax, 3
0x1404eb76b: lea rdx, [rax + rax*2]
0x1404eb76f: mov r8d, 8
0x1404eb775: call 0x140613c20
0x1404eb77a: mov byte ptr [rbp + 0x110d], 0
0x1404eb781: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb789: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb78e: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb794: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb79a: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb7a0: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb7a6: add rsp, 0x98
0x1404eb7ad: pop rbx
0x1404eb7ae: pop rdi
0x1404eb7af: pop rsi
0x1404eb7b0: pop r12
0x1404eb7b2: pop r13
0x1404eb7b4: pop r14
0x1404eb7b6: pop r15
0x1404eb7b8: pop rbp
0x1404eb7b9: ret 
0x1404eb7ba: nop word ptr [rax + rax]
0x1404eb7c0: mov qword ptr [rsp + 0x10], rdx
0x1404eb7c5: push rbp
0x1404eb7c6: push r15
0x1404eb7c8: push r14
0x1404eb7ca: push r13
0x1404eb7cc: push r12
0x1404eb7ce: push rsi
0x1404eb7cf: push rdi
0x1404eb7d0: push rbx
0x1404eb7d1: sub rsp, 0x98
0x1404eb7d8: lea rbp, [rdx + 0x80]
0x1404eb7df: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb7e5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb7eb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb7f1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb7f7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb7fc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb804: cmp byte ptr [rbp + 0x110d], 0
0x1404eb80b: je 0x1404eb852
0x1404eb80d: lea rcx, [rbp + 0x1000]
0x1404eb814: call 0x1405f5a40
0x1404eb819: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb821: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb826: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb82c: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb832: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb838: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb83e: add rsp, 0x98
0x1404eb845: pop rbx
0x1404eb846: pop rdi
0x1404eb847: pop rsi
0x1404eb848: pop r12
0x1404eb84a: pop r13
0x1404eb84c: pop r14
0x1404eb84e: pop r15
0x1404eb850: pop rbp
0x1404eb851: ret 
0x1404eb852: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb85a: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb85f: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb865: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb86b: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb871: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb877: add rsp, 0x98
0x1404eb87e: pop rbx
0x1404eb87f: pop rdi
0x1404eb880: pop rsi
0x1404eb881: pop r12
0x1404eb883: pop r13
0x1404eb885: pop r14
0x1404eb887: pop r15
0x1404eb889: pop rbp
0x1404eb88a: ret 
0x1404eb88b: nop dword ptr [rax + rax]
0x1404eb890: mov qword ptr [rsp + 0x10], rdx
0x1404eb895: push rbp
0x1404eb896: push r15
0x1404eb898: push r14
0x1404eb89a: push r13
0x1404eb89c: push r12
0x1404eb89e: push rsi
0x1404eb89f: push rdi
0x1404eb8a0: push rbx
0x1404eb8a1: sub rsp, 0x98
0x1404eb8a8: lea rbp, [rdx + 0x80]
0x1404eb8af: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb8b5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb8bb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb8c1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb8c7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb8cc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb8d4: movzx ebx, byte ptr [rbp + 0x1115]
0x1404eb8db: cmp byte ptr [rbp + 0x1116], 0
0x1404eb8e2: je 0x1404eb932
0x1404eb8e4: lea rcx, [rbp + 0x310]
0x1404eb8eb: call 0x1405faa90
0x1404eb8f0: and bl, 1
0x1404eb8f3: mov byte ptr [rbp + 0x1110], bl
0x1404eb8f9: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb901: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb906: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb90c: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb912: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb918: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb91e: add rsp, 0x98
0x1404eb925: pop rbx
0x1404eb926: pop rdi
0x1404eb927: pop rsi
0x1404eb928: pop r12
0x1404eb92a: pop r13
0x1404eb92c: pop r14
0x1404eb92e: pop r15
0x1404eb930: pop rbp
0x1404eb931: ret 
0x1404eb932: and bl, 1
0x1404eb935: mov byte ptr [rbp + 0x1110], bl
0x1404eb93b: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb943: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb948: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb94e: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb954: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb95a: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eb960: add rsp, 0x98
0x1404eb967: pop rbx
0x1404eb968: pop rdi
0x1404eb969: pop rsi
0x1404eb96a: pop r12
0x1404eb96c: pop r13
0x1404eb96e: pop r14
0x1404eb970: pop r15
0x1404eb972: pop rbp
0x1404eb973: ret 
0x1404eb974: nop word ptr cs:[rax + rax]
0x1404eb980: mov qword ptr [rsp + 0x10], rdx
0x1404eb985: push rbp
0x1404eb986: push r15
0x1404eb988: push r14
0x1404eb98a: push r13
0x1404eb98c: push r12
0x1404eb98e: push rsi
0x1404eb98f: push rdi
0x1404eb990: push rbx
0x1404eb991: sub rsp, 0x98
0x1404eb998: lea rbp, [rdx + 0x80]
0x1404eb99f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eb9a5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eb9ab: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eb9b1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eb9b7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eb9bc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404eb9c4: cmp byte ptr [rbp + 0x1110], 0
0x1404eb9cb: je 0x1404eba19
0x1404eb9cd: lea rcx, [rbp + 0xb00]
0x1404eb9d4: call 0x1405dab70
0x1404eb9d9: mov byte ptr [rbp + 0x1117], 0
0x1404eb9e0: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eb9e8: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eb9ed: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eb9f3: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eb9f9: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eb9ff: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eba05: add rsp, 0x98
0x1404eba0c: pop rbx
0x1404eba0d: pop rdi
0x1404eba0e: pop rsi
0x1404eba0f: pop r12
0x1404eba11: pop r13
0x1404eba13: pop r14
0x1404eba15: pop r15
0x1404eba17: pop rbp
0x1404eba18: ret 
0x1404eba19: mov byte ptr [rbp + 0x1117], 0
0x1404eba20: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404eba28: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404eba2d: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404eba33: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404eba39: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404eba3f: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404eba45: add rsp, 0x98
0x1404eba4c: pop rbx
0x1404eba4d: pop rdi
0x1404eba4e: pop rsi
0x1404eba4f: pop r12
0x1404eba51: pop r13
0x1404eba53: pop r14
0x1404eba55: pop r15
0x1404eba57: pop rbp
0x1404eba58: ret 
0x1404eba59: nop dword ptr [rax]
0x1404eba60: mov qword ptr [rsp + 0x10], rdx
0x1404eba65: push rbp
0x1404eba66: push r15
0x1404eba68: push r14
0x1404eba6a: push r13
0x1404eba6c: push r12
0x1404eba6e: push rsi
0x1404eba6f: push rdi
0x1404eba70: push rbx
0x1404eba71: sub rsp, 0x98
0x1404eba78: lea rbp, [rdx + 0x80]
0x1404eba7f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404eba85: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404eba8b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404eba91: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404eba97: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404eba9c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ebaa4: cmp byte ptr [rbp + 0x1117], 0
0x1404ebaab: je 0x1404ebaf2
0x1404ebaad: lea rcx, [rbp + 0x90]
0x1404ebab4: call 0x140609f60
0x1404ebab9: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebac1: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebac6: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebacc: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebad2: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebad8: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebade: add rsp, 0x98
0x1404ebae5: pop rbx
0x1404ebae6: pop rdi
0x1404ebae7: pop rsi
0x1404ebae8: pop r12
0x1404ebaea: pop r13
0x1404ebaec: pop r14
0x1404ebaee: pop r15
0x1404ebaf0: pop rbp
0x1404ebaf1: ret 
0x1404ebaf2: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebafa: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebaff: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebb05: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebb0b: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebb11: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebb17: add rsp, 0x98
0x1404ebb1e: pop rbx
0x1404ebb1f: pop rdi
0x1404ebb20: pop rsi
0x1404ebb21: pop r12
0x1404ebb23: pop r13
0x1404ebb25: pop r14
0x1404ebb27: pop r15
0x1404ebb29: pop rbp
0x1404ebb2a: ret 
0x1404ebb2b: nop dword ptr [rax + rax]
0x1404ebb30: mov qword ptr [rsp + 0x10], rdx
0x1404ebb35: push rbp
0x1404ebb36: push r15
0x1404ebb38: push r14
0x1404ebb3a: push r13
0x1404ebb3c: push r12
0x1404ebb3e: push rsi
0x1404ebb3f: push rdi
0x1404ebb40: push rbx
0x1404ebb41: sub rsp, 0x98
0x1404ebb48: lea rbp, [rdx + 0x80]
0x1404ebb4f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ebb55: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ebb5b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ebb61: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ebb67: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ebb6c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ebb74: lea rcx, [rbp + 0xd20]
0x1404ebb7b: call 0x1405fdb80
0x1404ebb80: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebb88: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebb8d: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebb93: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebb99: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebb9f: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebba5: add rsp, 0x98
0x1404ebbac: pop rbx
0x1404ebbad: pop rdi
0x1404ebbae: pop rsi
0x1404ebbaf: pop r12
0x1404ebbb1: pop r13
0x1404ebbb3: pop r14
0x1404ebbb5: pop r15
0x1404ebbb7: pop rbp
0x1404ebbb8: ret 
0x1404ebbb9: nop dword ptr [rax]
0x1404ebbc0: mov qword ptr [rsp + 0x10], rdx
0x1404ebbc5: push rbp
0x1404ebbc6: push r15
0x1404ebbc8: push r14
0x1404ebbca: push r13
0x1404ebbcc: push r12
0x1404ebbce: push rsi
0x1404ebbcf: push rdi
0x1404ebbd0: push rbx
0x1404ebbd1: sub rsp, 0x98
0x1404ebbd8: lea rbp, [rdx + 0x80]
0x1404ebbdf: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ebbe5: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ebbeb: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ebbf1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ebbf7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ebbfc: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ebc04: lea rcx, [rbp + 0x788]
0x1404ebc0b: call 0x1405f2450
0x1404ebc10: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebc18: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebc1d: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebc23: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebc29: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebc2f: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebc35: add rsp, 0x98
0x1404ebc3c: pop rbx
0x1404ebc3d: pop rdi
0x1404ebc3e: pop rsi
0x1404ebc3f: pop r12
0x1404ebc41: pop r13
0x1404ebc43: pop r14
0x1404ebc45: pop r15
0x1404ebc47: pop rbp
0x1404ebc48: ret 
0x1404ebc49: nop dword ptr [rax]
0x1404ebc50: mov qword ptr [rsp + 0x10], rdx
0x1404ebc55: push rbp
0x1404ebc56: push r15
0x1404ebc58: push r14
0x1404ebc5a: push r13
0x1404ebc5c: push r12
0x1404ebc5e: push rsi
0x1404ebc5f: push rdi
0x1404ebc60: push rbx
0x1404ebc61: sub rsp, 0x98
0x1404ebc68: lea rbp, [rdx + 0x80]
0x1404ebc6f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ebc75: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ebc7b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ebc81: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ebc87: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ebc8c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ebc94: mov rsi, qword ptr [rbp + 0x10e0]
0x1404ebc9b: lea rcx, [rsi + 0x190]
0x1404ebca2: lea rdx, [rsi + 0x1b0]
0x1404ebca9: mov r8d, 0x10
0x1404ebcaf: mov r9d, 0x10
0x1404ebcb5: call 0x14004c400
0x1404ebcba: movaps xmm0, xmmword ptr [rbp + 0xb40]
0x1404ebcc1: mov rax, qword ptr [rbp + 0x10f0]
0x1404ebcc8: movups xmmword ptr [rax + 0x40], xmm0
0x1404ebccc: movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404ebcd4: movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404ebcdb: movapd xmm2, xmmword ptr [rbp + 0xb20]
0x1404ebce3: movapd xmm3, xmmword ptr [rbp + 0xb30]
0x1404ebceb: movupd xmmword ptr [rax + 0x30], xmm3
0x1404ebcf0: movupd xmmword ptr [rax + 0x20], xmm2
0x1404ebcf5: movups xmmword ptr [rax + 0x10], xmm1
0x1404ebcf9: movupd xmmword ptr [rax], xmm0
0x1404ebcfd: mov rax, qword ptr [rbp + 0x10e8]
0x1404ebd04: mov qword ptr [rsi + 0x1b0], rax
0x1404ebd0b: mov rax, qword ptr [rbp + 0x10d0]
0x1404ebd12: mov qword ptr [rsi + 0x1b8], rax
0x1404ebd19: mov byte ptr [rbp + 0x1117], 0
0x1404ebd20: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebd28: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebd2d: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebd33: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebd39: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebd3f: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebd45: add rsp, 0x98
0x1404ebd4c: pop rbx
0x1404ebd4d: pop rdi
0x1404ebd4e: pop rsi
0x1404ebd4f: pop r12
0x1404ebd51: pop r13
0x1404ebd53: pop r14
0x1404ebd55: pop r15
0x1404ebd57: pop rbp
0x1404ebd58: ret 
0x1404ebd59: nop dword ptr [rax]
0x1404ebd60: mov qword ptr [rsp + 0x10], rdx
0x1404ebd65: push rbp
0x1404ebd66: push r15
0x1404ebd68: push r14
0x1404ebd6a: push r13
0x1404ebd6c: push r12
0x1404ebd6e: push rsi
0x1404ebd6f: push rdi
0x1404ebd70: push rbx
0x1404ebd71: sub rsp, 0x98
0x1404ebd78: lea rbp, [rdx + 0x80]
0x1404ebd7f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ebd85: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ebd8b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ebd91: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ebd97: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ebd9c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ebda4: mov rsi, qword ptr [rbp + 0x10e0]
0x1404ebdab: lea rcx, [rsi + 0x280]
0x1404ebdb2: lea rdx, [rsi + 0x2a0]
0x1404ebdb9: mov r8d, 0x10
0x1404ebdbf: mov r9d, 0x10
0x1404ebdc5: call 0x14004c400
0x1404ebdca: movaps xmm0, xmmword ptr [rbp + 0xb40]
0x1404ebdd1: mov rax, qword ptr [rbp + 0x10f0]
0x1404ebdd8: movups xmmword ptr [rax + 0x40], xmm0
0x1404ebddc: movapd xmm0, xmmword ptr [rbp + 0xb00]
0x1404ebde4: movaps xmm1, xmmword ptr [rbp + 0xb10]
0x1404ebdeb: movapd xmm2, xmmword ptr [rbp + 0xb20]
0x1404ebdf3: movapd xmm3, xmmword ptr [rbp + 0xb30]
0x1404ebdfb: movupd xmmword ptr [rax + 0x30], xmm3
0x1404ebe00: movupd xmmword ptr [rax + 0x20], xmm2
0x1404ebe05: movups xmmword ptr [rax + 0x10], xmm1
0x1404ebe09: movupd xmmword ptr [rax], xmm0
0x1404ebe0d: mov rax, qword ptr [rbp + 0x10e8]
0x1404ebe14: mov qword ptr [rsi + 0x2a0], rax
0x1404ebe1b: mov rax, qword ptr [rbp + 0x10d0]
0x1404ebe22: mov qword ptr [rsi + 0x2a8], rax
0x1404ebe29: mov byte ptr [rbp + 0x1117], 0
0x1404ebe30: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebe38: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebe3d: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebe43: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebe49: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebe4f: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebe55: add rsp, 0x98
0x1404ebe5c: pop rbx
0x1404ebe5d: pop rdi
0x1404ebe5e: pop rsi
0x1404ebe5f: pop r12
0x1404ebe61: pop r13
0x1404ebe63: pop r14
0x1404ebe65: pop r15
0x1404ebe67: pop rbp
0x1404ebe68: ret 
0x1404ebe69: nop dword ptr [rax]
0x1404ebe70: mov qword ptr [rsp + 0x10], rdx
0x1404ebe75: push rbp
0x1404ebe76: push r15
0x1404ebe78: push r14
0x1404ebe7a: push r13
0x1404ebe7c: push r12
0x1404ebe7e: push rsi
0x1404ebe7f: push rdi
0x1404ebe80: push rbx
0x1404ebe81: sub rsp, 0x98
0x1404ebe88: lea rbp, [rdx + 0x80]
0x1404ebe8f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ebe95: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ebe9b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ebea1: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ebea7: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ebeac: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ebeb4: mov byte ptr [rbp + 0x1117], 0
0x1404ebebb: lea rcx, [rbp + 0xb00]
0x1404ebec2: call 0x140609f60
0x1404ebec7: mov byte ptr [rbp + 0x1117], 0
0x1404ebece: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebed6: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebedb: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebee1: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebee7: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebeed: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebef3: add rsp, 0x98
0x1404ebefa: pop rbx
0x1404ebefb: pop rdi
0x1404ebefc: pop rsi
0x1404ebefd: pop r12
0x1404ebeff: pop r13
0x1404ebf01: pop r14
0x1404ebf03: pop r15
0x1404ebf05: pop rbp
0x1404ebf06: ret 
0x1404ebf07: nop word ptr [rax + rax]
0x1404ebf10: mov qword ptr [rsp + 0x10], rdx
0x1404ebf15: push rbp
0x1404ebf16: push r15
0x1404ebf18: push r14
0x1404ebf1a: push r13
0x1404ebf1c: push r12
0x1404ebf1e: push rsi
0x1404ebf1f: push rdi
0x1404ebf20: push rbx
0x1404ebf21: sub rsp, 0x98
0x1404ebf28: lea rbp, [rdx + 0x80]
0x1404ebf2f: movaps xmmword ptr [rsp + 0x30], xmm11
0x1404ebf35: movaps xmmword ptr [rsp + 0x40], xmm10
0x1404ebf3b: movaps xmmword ptr [rsp + 0x50], xmm9
0x1404ebf41: movaps xmmword ptr [rsp + 0x60], xmm8
0x1404ebf47: movaps xmmword ptr [rsp + 0x70], xmm7
0x1404ebf4c: movaps xmmword ptr [rsp + 0x80], xmm6
0x1404ebf54: lea rcx, [rbp + 0x530]
0x1404ebf5b: call 0x1405d1490
0x1404ebf60: mov byte ptr [rbp + 0x1116], 1
0x1404ebf67: mov byte ptr [rbp + 0x1115], 1
0x1404ebf6e: movaps xmm6, xmmword ptr [rsp + 0x80]
0x1404ebf76: movaps xmm7, xmmword ptr [rsp + 0x70]
0x1404ebf7b: movaps xmm8, xmmword ptr [rsp + 0x60]
0x1404ebf81: movaps xmm9, xmmword ptr [rsp + 0x50]
0x1404ebf87: movaps xmm10, xmmword ptr [rsp + 0x40]
0x1404ebf8d: movaps xmm11, xmmword ptr [rsp + 0x30]
0x1404ebf93: add rsp, 0x98
0x1404ebf9a: pop rbx
0x1404ebf9b: pop rdi
0x1404ebf9c: pop rsi
0x1404ebf9d: pop r12
0x1404ebf9f: pop r13
0x1404ebfa1: pop r14
0x1404ebfa3: pop r15
0x1404ebfa5: pop rbp
0x1404ebfa6: ret 
