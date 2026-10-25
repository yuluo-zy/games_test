1400d37d0: push rbp
1400d37d1: push r15
1400d37d3: push r14
1400d37d5: push rsi
1400d37d6: push rdi
1400d37d7: push rbx
1400d37d8: sub rsp, 0x1b8
1400d37df: lea rbp, [rsp + 0x80]
1400d37e7: movaps xmmword ptr [rbp + 0x120], xmm6
1400d37ee: mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
1400d37f9: mov r15, rdx
1400d37fc: mov rsi, rcx
1400d37ff: mov qword ptr [rbp - 0x58], 0
1400d3807: mov qword ptr [rbp - 0x38], 0
1400d380f: mov qword ptr [rbp - 0x30], 8
1400d3817: mov qword ptr [rbp - 0x28], 0
1400d381f: mov qword ptr [rbp - 0x20], 0x10
1400d3827: xorps xmm6, xmm6
1400d382a: movups xmmword ptr [rbp - 0x18], xmm6
1400d382e: mov qword ptr [rbp - 8], 0x10
1400d3836: movups xmmword ptr [rbp], xmm6
1400d383a: mov qword ptr [rbp + 0x10], 0x10
1400d3842: movups xmmword ptr [rbp + 0x18], xmm6
1400d3846: mov qword ptr [rbp + 0x28], 0x10
1400d384e: movups xmmword ptr [rbp + 0x30], xmm6
1400d3852: mov qword ptr [rbp + 0x40], 0x10
1400d385a: movups xmmword ptr [rbp + 0x48], xmm6
1400d385e: mov dword ptr [rbp + 0x58], 0
1400d3865: movabs rdi, 0x8000000000000000
1400d386f: mov qword ptr [rbp + 0x60], rdi
1400d3873: lea rbx, [rip + 0x28575f6]
1400d387a: mov qword ptr [rbp + 0x68], rbx
1400d387e: mov qword ptr [rbp + 0x70], 0x29
1400d3886: mov qword ptr [rbp + 0x78], 0x10
1400d388e: movups xmmword ptr [rbp + 0x80], xmm6
1400d3895: mov qword ptr [rbp + 0x90], 0x10
1400d38a0: movups xmmword ptr [rbp + 0x98], xmm6
1400d38a7: mov qword ptr [rbp + 0xa8], 0x10
1400d38b2: movups xmmword ptr [rbp + 0xb0], xmm6
1400d38b9: mov qword ptr [rbp + 0xc0], 0x10
1400d38c4: movups xmmword ptr [rbp + 0xc8], xmm6
1400d38cb: mov qword ptr [rbp + 0xd8], 0x10
1400d38d6: movups xmmword ptr [rbp + 0xe0], xmm6
1400d38dd: mov dword ptr [rbp + 0xf0], 0
1400d38e7: mov dword ptr [rbp + 0xf8], 0
1400d38f1: mov word ptr [rbp + 0xfc], 1
1400d38fa: mov dword ptr [rbp + 0x100], 0
1400d3904: movzx eax, byte ptr [rip + 0x35612fe]
1400d390b: mov ecx, 0x160
1400d3910: mov edx, 8
1400d3915: call 0x140613c10
1400d391a: test rax, rax
1400d391d: mov qword ptr [rbp + 0x108], r15
1400d3924: je 0x1400d3b13
1400d392a: mov qword ptr [rax], 0
1400d3931: mov qword ptr [rax + 0x20], 0
1400d3939: mov qword ptr [rax + 0x28], 8
1400d3941: mov qword ptr [rax + 0x30], 0
1400d3949: mov qword ptr [rax + 0x38], 0x10
1400d3951: movups xmmword ptr [rax + 0x40], xmm6
1400d3955: mov qword ptr [rax + 0x50], 0x10
1400d395d: movups xmmword ptr [rax + 0x58], xmm6
1400d3961: mov qword ptr [rax + 0x68], 0x10
1400d3969: movups xmmword ptr [rax + 0x70], xmm6
1400d396d: mov qword ptr [rax + 0x80], 0x10
1400d3978: movups xmmword ptr [rax + 0x88], xmm6
1400d397f: mov qword ptr [rax + 0x98], 0x10
1400d398a: movups xmmword ptr [rax + 0xa0], xmm6
1400d3991: mov dword ptr [rax + 0xb0], 0
1400d399b: mov qword ptr [rax + 0xb8], rdi
1400d39a2: mov qword ptr [rax + 0xc0], rbx
1400d39a9: mov qword ptr [rax + 0xc8], 0x29
1400d39b4: mov qword ptr [rax + 0xd0], 0x10
1400d39bf: movups xmmword ptr [rax + 0xd8], xmm6
1400d39c6: mov qword ptr [rax + 0xe8], 0x10
1400d39d1: movups xmmword ptr [rax + 0xf0], xmm6
1400d39d8: mov qword ptr [rax + 0x100], 0x10
1400d39e3: movups xmmword ptr [rax + 0x108], xmm6
1400d39ea: mov qword ptr [rax + 0x118], 0x10
1400d39f5: movups xmmword ptr [rax + 0x120], xmm6
1400d39fc: mov qword ptr [rax + 0x130], 0x10
1400d3a07: movups xmmword ptr [rax + 0x138], xmm6
1400d3a0e: mov dword ptr [rax + 0x148], 0
1400d3a18: mov dword ptr [rax + 0x150], 0
1400d3a22: mov word ptr [rax + 0x154], 1
1400d3a2b: mov qword ptr [rbp + 0x110], rax
1400d3a32: mov dword ptr [rax + 0x158], 0
1400d3a3c: xor eax, eax
1400d3a3e: cmp rax, qword ptr [r15]
1400d3a41: jno 0x1400d3a69
1400d3a43: lea rdi, [r15 + 0x30]
1400d3a47: mov rbx, qword ptr [r15 + 0x30]
1400d3a4b: mov r14d, 0x28
1400d3a51: cmp rbx, qword ptr [r15 + 0x20]
1400d3a55: jne 0x1400d3a8d
1400d3a57: lea rcx, [r15 + 0x20]
1400d3a5b: lea rdx, [rip + 0x28666d6]
1400d3a62: call 0x1407404d0
1400d3a67: jmp 0x1400d3a8d
1400d3a69: lea rdi, [r15 + 0x58]
1400d3a6d: mov rbx, qword ptr [r15 + 0x58]
1400d3a71: mov r14d, 0x50
1400d3a77: cmp rbx, qword ptr [r15 + 0x48]
1400d3a7b: jne 0x1400d3a8d
1400d3a7d: lea rcx, [r15 + 0x48]
1400d3a81: lea rdx, [rip + 0x2866698]
1400d3a88: call 0x1407404d0
1400d3a8d: mov r8, qword ptr [rbp + 0x108]
1400d3a94: mov rax, qword ptr [r8 + r14]
1400d3a98: mov rcx, rbx
1400d3a9b: shl rcx, 4
1400d3a9f: mov rdx, qword ptr [rbp + 0x110]
1400d3aa6: mov qword ptr [rax + rcx], rdx
1400d3aaa: lea rdx, [rip + 0x28665af]
1400d3ab1: mov qword ptr [rax + rcx + 8], rdx
1400d3ab6: inc rbx
1400d3ab9: mov qword ptr [rdi], rbx
1400d3abc: movups xmm0, xmmword ptr [r8 + 0x60]
1400d3ac1: movups xmmword ptr [rsi + 0x60], xmm0
1400d3ac5: movups xmm0, xmmword ptr [r8 + 0x50]
1400d3aca: movups xmmword ptr [rsi + 0x50], xmm0
1400d3ace: movups xmm0, xmmword ptr [r8 + 0x40]
1400d3ad3: movups xmmword ptr [rsi + 0x40], xmm0
1400d3ad7: movups xmm0, xmmword ptr [r8]
1400d3adb: movups xmm1, xmmword ptr [r8 + 0x10]
1400d3ae0: movups xmm2, xmmword ptr [r8 + 0x20]
1400d3ae5: movups xmm3, xmmword ptr [r8 + 0x30]
1400d3aea: movups xmmword ptr [rsi + 0x30], xmm3
1400d3aee: movups xmmword ptr [rsi + 0x20], xmm2
1400d3af2: movups xmmword ptr [rsi + 0x10], xmm1
1400d3af6: movups xmmword ptr [rsi], xmm0
1400d3af9: mov rax, rsi
1400d3afc: movaps xmm6, xmmword ptr [rbp + 0x120]
1400d3b03: add rsp, 0x1b8
1400d3b0a: pop rbx
1400d3b0b: pop rdi
1400d3b0c: pop rsi
1400d3b0d: pop r14
1400d3b0f: pop r15
1400d3b11: pop rbp
1400d3b12: ret 
1400d3b13: mov ecx, 8
1400d3b18: mov edx, 0x160
1400d3b1d: call 0x1428d9000
1400d3b22: ud2 
1400d3b24: nop word ptr cs:[rax + rax]
1400d3b30: mov qword ptr [rsp + 0x10], rdx
1400d3b35: push rbp
1400d3b36: push r15
1400d3b38: push r14
1400d3b3a: push rsi
1400d3b3b: push rdi
1400d3b3c: push rbx
1400d3b3d: sub rsp, 0x38
1400d3b41: lea rbp, [rdx + 0x80]
1400d3b48: movaps xmmword ptr [rsp + 0x20], xmm6
1400d3b4d: lea rcx, [rbp - 0x58]
1400d3b51: call 0x1405e5e70
1400d3b56: movaps xmm6, xmmword ptr [rsp + 0x20]
1400d3b5b: add rsp, 0x38
1400d3b5f: pop rbx
1400d3b60: pop rdi
1400d3b61: pop rsi
1400d3b62: pop r14
1400d3b64: pop r15
1400d3b66: pop rbp
1400d3b67: ret 
1400d3b68: nop dword ptr [rax + rax]
1400d3b70: mov qword ptr [rsp + 0x10], rdx
1400d3b75: push rbp
1400d3b76: push r15
1400d3b78: push r14
1400d3b7a: push rsi
1400d3b7b: push rdi
1400d3b7c: push rbx
1400d3b7d: sub rsp, 0x38
1400d3b81: lea rbp, [rdx + 0x80]
1400d3b88: movaps xmmword ptr [rsp + 0x20], xmm6
1400d3b8d: mov rcx, qword ptr [rbp + 0x108]
1400d3b94: call 0x140029880
1400d3b99: movaps xmm6, xmmword ptr [rsp + 0x20]
1400d3b9e: add rsp, 0x38
1400d3ba2: pop rbx
1400d3ba3: pop rdi
1400d3ba4: pop rsi
1400d3ba5: pop r14
1400d3ba7: pop r15
1400d3ba9: pop rbp
1400d3baa: ret 
1400d3bab: nop dword ptr [rax + rax]
1400d3bb0: mov qword ptr [rsp + 0x10], rdx
1400d3bb5: push rbp
1400d3bb6: push r15
1400d3bb8: push r14
1400d3bba: push rsi
1400d3bbb: push rdi
1400d3bbc: push rbx
1400d3bbd: sub rsp, 0x38
1400d3bc1: lea rbp, [rdx + 0x80]
1400d3bc8: movaps xmmword ptr [rsp + 0x20], xmm6
1400d3bcd: lea rdx, [rip + 0x286648c]
1400d3bd4: mov rcx, qword ptr [rbp + 0x110]
1400d3bdb: call 0x14002e900
1400d3be0: movaps xmm6, xmmword ptr [rsp + 0x20]
1400d3be5: add rsp, 0x38
1400d3be9: pop rbx
1400d3bea: pop rdi
1400d3beb: pop rsi
1400d3bec: pop r14
1400d3bee: pop r15
1400d3bf0: pop rbp
1400d3bf1: ret 
1400d3bf2: nop word ptr cs:[rax + rax]
1400d3c00: mov qword ptr [rsp + 0x10], rdx
1400d3c05: push rbp
1400d3c06: push r15
1400d3c08: push r14
1400d3c0a: push rsi
1400d3c0b: push rdi
1400d3c0c: push rbx
1400d3c0d: sub rsp, 0x38
1400d3c11: lea rbp, [rdx + 0x80]
1400d3c18: movaps xmmword ptr [rsp + 0x20], xmm6
1400d3c1d: lea rdx, [rip + 0x286643c]
1400d3c24: mov rcx, qword ptr [rbp + 0x110]
1400d3c2b: call 0x14002e900
1400d3c30: movaps xmm6, xmmword ptr [rsp + 0x20]
1400d3c35: add rsp, 0x38
1400d3c39: pop rbx
1400d3c3a: pop rdi
1400d3c3b: pop rsi
1400d3c3c: pop r14
1400d3c3e: pop r15
1400d3c40: pop rbp
1400d3c41: ret 
