140a04cf0: push r15
140a04cf2: push r14
140a04cf4: push r13
140a04cf6: push r12
140a04cf8: push rsi
140a04cf9: push rdi
140a04cfa: push rbp
140a04cfb: push rbx
140a04cfc: sub rsp, 0x68
140a04d00: movdqa xmmword ptr [rsp + 0x50], xmm8
140a04d07: movdqa xmmword ptr [rsp + 0x40], xmm7
140a04d0d: movdqa xmmword ptr [rsp + 0x30], xmm6
140a04d13: mov rax, qword ptr [rcx]
140a04d16: mov r14, qword ptr [rax]
140a04d19: mov rcx, qword ptr [rdx]
140a04d1c: call 0x141556130
140a04d21: mov rdi, rax
140a04d24: mov rax, qword ptr [r14]
140a04d27: mov sil, 1
140a04d2a: cmp byte ptr [rax], 1
140a04d2d: jne 0x140a04e13
140a04d33: mov rbx, rdx
140a04d36: mov rcx, qword ptr [r14 + 8]
140a04d3a: mov rdx, rdi
140a04d3d: mov r8, rbx
140a04d40: call 0x141552ff0
140a04d45: test al, al
140a04d47: je 0x140a04e13
140a04d4d: mov r15, qword ptr [r14 + 0x10]
140a04d51: cmp qword ptr [r15 + 0x18], 0
140a04d56: je 0x140a04e11
140a04d5c: lea rcx, [r15 + 0x20]
140a04d60: mov rdx, rdi
140a04d63: mov r8, rbx
140a04d66: call 0x1405b1a70
140a04d6b: mov r14, qword ptr [r15]
140a04d6e: mov r15, qword ptr [r15 + 8]
140a04d72: mov r12, r15
140a04d75: and r12, rax
140a04d78: shr rax, 0x39
140a04d7c: movd xmm0, eax
140a04d80: punpcklbw xmm0, xmm0
140a04d84: pshuflw xmm0, xmm0, 0
140a04d89: pshufd xmm6, xmm0, 0
140a04d8e: lea r13, [r14 - 0x10]
140a04d92: xor ecx, ecx
140a04d94: pcmpeqd xmm7, xmm7
140a04d98: movdqu xmm8, xmmword ptr [r14 + r12]
140a04d9e: movdqa xmm0, xmm8
140a04da3: pcmpeqb xmm0, xmm6
140a04da7: pmovmskb ebp, xmm0
140a04dab: test ebp, ebp
140a04dad: je 0x140a04df3
140a04daf: mov qword ptr [rsp + 0x28], rcx
140a04db4: tzcnt eax, ebp
140a04db8: add rax, r12
140a04dbb: and rax, r15
140a04dbe: shl rax, 4
140a04dc2: mov rcx, r13
140a04dc5: sub rcx, rax
140a04dc8: call 0x141556130
140a04dcd: cmp rdx, rbx
140a04dd0: jne 0x140a04de4
140a04dd2: mov rcx, rdi
140a04dd5: mov rdx, rax
140a04dd8: mov r8, rbx
140a04ddb: call 0x142923bf0
140a04de0: test eax, eax
140a04de2: je 0x140a04e13
140a04de4: lea eax, [rbp - 1]
140a04de7: and ax, bp
140a04dea: mov ebp, eax
140a04dec: mov rcx, qword ptr [rsp + 0x28]
140a04df1: jne 0x140a04db4
140a04df3: pcmpeqb xmm8, xmm7
140a04df8: pmovmskb eax, xmm8
140a04dfd: test eax, eax
140a04dff: jne 0x140a04e11
140a04e01: add r12, rcx
140a04e04: add r12, 0x10
140a04e08: add rcx, 0x10
140a04e0c: and r12, r15
140a04e0f: jmp 0x140a04d98
140a04e11: xor esi, esi
140a04e13: mov eax, esi
140a04e15: movaps xmm6, xmmword ptr [rsp + 0x30]
140a04e1a: movaps xmm7, xmmword ptr [rsp + 0x40]
140a04e1f: movaps xmm8, xmmword ptr [rsp + 0x50]
140a04e25: add rsp, 0x68
140a04e29: pop rbx
140a04e2a: pop rbp
140a04e2b: pop rdi
140a04e2c: pop rsi
140a04e2d: pop r12
140a04e2f: pop r13
140a04e31: pop r14
140a04e33: pop r15
140a04e35: ret 
