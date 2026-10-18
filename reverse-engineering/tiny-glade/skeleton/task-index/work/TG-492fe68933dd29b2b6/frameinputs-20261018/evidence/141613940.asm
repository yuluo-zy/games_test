141613940: push rbp
141613941: push r15
141613943: push r14
141613945: push r12
141613947: push rsi
141613948: push rdi
141613949: push rbx
14161394a: sub rsp, 0x100
141613951: lea rbp, [rsp + 0x80]
141613959: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141613961: test r8, r8
141613964: je 0x141613a79
14161396a: mov rbx, r9
14161396d: mov rdi, rdx
141613970: mov rsi, rcx
141613973: movabs rcx, 0xcccccccccccccccd
14161397d: mov rax, r9
141613980: mul rcx
141613983: shr rdx, 2
141613987: and rdx, 0xfffffffffffffffc
14161398b: lea rax, [rdx + rdx*4]
14161398f: mov rcx, r9
141613992: sub rcx, rax
141613995: mov qword ptr [rbp + 0x18], rcx
141613999: jne 0x141613a3e
14161399f: mov r14, r8
1416139a2: mov rax, r8
1416139a5: and rax, 3
1416139a9: mov qword ptr [rbp + 0x18], rax
1416139ad: jne 0x141613a4f
1416139b3: lea r15, [rbp - 0x40]
1416139b7: mov rcx, r15
1416139ba: mov rdx, rbx
1416139bd: mov r8d, 0x122
1416139c3: mov r9b, 1
1416139c6: call 0x140bd4de0
1416139cb: lea r12, [rbp + 0x68]
1416139cf: mov r8d, 4
1416139d5: mov rcx, r12
1416139d8: mov rdx, r15
1416139db: call 0x140bd4f20
1416139e0: mov qword ptr [rsp + 0x30], rbx
1416139e5: mov qword ptr [rsp + 0x28], r14
1416139ea: mov qword ptr [rsp + 0x20], r12
1416139ef: lea r8, [rip + 0x166c636]
1416139f6: lea rcx, [rbp + 0x18]
1416139fa: mov r9d, 0x10
141613a00: mov rdx, rdi
141613a03: call 0x1416a2dc0
141613a08: cmp dword ptr [rbp + 0x18], 0xb
141613a0c: jne 0x141613af0
141613a12: mov rax, qword ptr [rbp + 0x40]
141613a16: mov qword ptr [rsi + 0x20], rax
141613a1a: movups xmm0, xmmword ptr [rbp + 0x20]
141613a1e: movups xmm1, xmmword ptr [rbp + 0x30]
141613a22: movups xmmword ptr [rsi + 0x10], xmm1
141613a26: movups xmmword ptr [rsi], xmm0
141613a29: mov rax, rsi
141613a2c: add rsp, 0x100
141613a33: pop rbx
141613a34: pop rdi
141613a35: pop rsi
141613a36: pop r12
141613a38: pop r14
141613a3a: pop r15
141613a3c: pop rbp
141613a3d: ret 
141613a3e: mov qword ptr [rbp - 0x40], 0
141613a46: lea rax, [rip + 0x16b64fb]
141613a4d: jmp 0x141613a5e
141613a4f: mov qword ptr [rbp - 0x40], 0
141613a57: lea rax, [rip + 0x16b6502]
141613a5e: mov qword ptr [rsp + 0x20], rax
141613a63: lea r8, [rip + 0x16b64d6]
141613a6a: lea rdx, [rbp + 0x18]
141613a6e: lea r9, [rbp - 0x40]
141613a72: xor ecx, ecx
141613a74: call 0x1428d9596
141613a79: lea rax, [rip + 0x1489fcf]
141613a80: mov qword ptr [rbp + 0x68], rax
141613a84: mov qword ptr [rbp + 0x70], 0x35
141613a8c: lea rax, [rip + 0x166c5ad]
141613a93: mov qword ptr [rbp + 0x18], rax
141613a97: lea rax, [rip - 0x10e01fe]
141613a9e: mov qword ptr [rbp + 0x20], rax
141613aa2: lea rax, [rbp + 0x68]
141613aa6: mov qword ptr [rbp + 0x28], rax
141613aaa: lea rax, [rip - 0x1509c21]
141613ab1: mov qword ptr [rbp + 0x30], rax
141613ab5: lea rax, [rip + 0x166bc74]
141613abc: mov qword ptr [rbp - 0x40], rax
141613ac0: mov qword ptr [rbp - 0x38], 3
141613ac8: mov qword ptr [rbp - 0x20], 0
141613ad0: lea rax, [rbp + 0x18]
141613ad4: mov qword ptr [rbp - 0x30], rax
141613ad8: mov qword ptr [rbp - 0x28], 2
141613ae0: lea rdx, [rip + 0x166bca1]
141613ae7: lea rcx, [rbp - 0x40]
141613aeb: call 0x1428d9390
141613af0: movups xmm0, xmmword ptr [rbp + 0x58]
141613af4: movaps xmmword ptr [rbp], xmm0
141613af8: movups xmm0, xmmword ptr [rbp + 0x18]
141613afc: movups xmm1, xmmword ptr [rbp + 0x28]
141613b00: movups xmm2, xmmword ptr [rbp + 0x38]
141613b04: movups xmm3, xmmword ptr [rbp + 0x48]
141613b08: movaps xmmword ptr [rbp - 0x10], xmm3
141613b0c: movaps xmmword ptr [rbp - 0x20], xmm2
141613b10: movaps xmmword ptr [rbp - 0x30], xmm1
141613b14: movaps xmmword ptr [rbp - 0x40], xmm0
141613b18: lea rax, [rip + 0x166c689]
141613b1f: mov qword ptr [rsp + 0x20], rax
141613b24: lea rcx, [rip + 0x166afbd]
141613b2b: lea r9, [rip + 0x166af96]
141613b32: lea r8, [rbp - 0x40]
141613b36: mov edx, 0x2b
141613b3b: call 0x1428d9760
141613b40: ud2 
141613b42: nop word ptr cs:[rax + rax]
141613b50: mov qword ptr [rsp + 0x10], rdx
141613b55: push rbp
141613b56: push r15
141613b58: push r14
141613b5a: push r12
141613b5c: push rsi
141613b5d: push rdi
141613b5e: push rbx
141613b5f: sub rsp, 0x40
141613b63: lea rbp, [rdx + 0x80]
141613b6a: lea rcx, [rbp - 0x40]
141613b6e: call 0x1407cea70
141613b73: nop 
141613b74: add rsp, 0x40
141613b78: pop rbx
141613b79: pop rdi
141613b7a: pop rsi
141613b7b: pop r12
141613b7d: pop r14
141613b7f: pop r15
141613b81: pop rbp
141613b82: ret 
