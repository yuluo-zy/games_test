141610820: push rbp
141610821: push r15
141610823: push r14
141610825: push r12
141610827: push rsi
141610828: push rdi
141610829: push rbx
14161082a: sub rsp, 0x120
141610831: lea rbp, [rsp + 0x80]
141610839: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141610844: mov rdi, rdx
141610847: mov rsi, rcx
14161084a: test r8, r8
14161084d: je 0x141610886
14161084f: mov rbx, r9
141610852: mov rax, r9
141610855: and rax, 0xf
141610859: mov qword ptr [rbp - 0x40], rax
14161085d: jne 0x14161097f
141610863: mov r15, r8
141610866: mov rax, r8
141610869: and rax, 3
14161086d: mov qword ptr [rbp - 0x40], rax
141610871: jne 0x141610990
141610877: cmp rbx, 0x20
14161087b: jae 0x141610a1a
141610881: xor r12d, r12d
141610884: jmp 0x1416108b6
141610886: movzx eax, byte ptr [rip + 0x202437c]
14161088d: mov ebx, 0x10
141610892: mov ecx, 0x10
141610897: mov edx, 4
14161089c: call 0x140613c10
1416108a1: test rax, rax
1416108a4: je 0x141610a84
1416108aa: mov r15, rax
1416108ad: xorps xmm0, xmm0
1416108b0: movups xmmword ptr [rax], xmm0
1416108b3: mov r12b, 1
1416108b6: mov qword ptr [rbp + 0x88], r15
1416108bd: mov byte ptr [rbp + 0x97], r12b
1416108c4: lea rcx, [rbp + 0x10]
1416108c8: mov edx, 0x10
1416108cd: mov r8d, 0x122
1416108d3: mov r9b, 1
1416108d6: call 0x140bd4de0
1416108db: mov qword ptr [rbp + 0x88], r15
1416108e2: mov byte ptr [rbp + 0x97], r12b
1416108e9: lea r14, [rbp + 0x68]
1416108ed: lea rdx, [rbp + 0x10]
1416108f1: mov r8d, 4
1416108f7: mov rcx, r14
1416108fa: call 0x140bd4f20
1416108ff: mov qword ptr [rbp + 0x88], r15
141610906: mov byte ptr [rbp + 0x97], r12b
14161090d: mov qword ptr [rsp + 0x30], rbx
141610912: mov qword ptr [rsp + 0x28], r15
141610917: mov qword ptr [rsp + 0x20], r14
14161091c: lea r8, [rip + 0x166f39e]
141610923: lea rcx, [rbp - 0x40]
141610927: mov r9d, 0x12
14161092d: mov rdx, rdi
141610930: call 0x1416a2dc0
141610935: cmp dword ptr [rbp - 0x40], 0xb
141610939: jne 0x1416109ba
14161093b: mov rax, qword ptr [rbp - 0x18]
14161093f: mov qword ptr [rsi + 0x20], rax
141610943: movups xmm0, xmmword ptr [rbp - 0x38]
141610947: movups xmm1, xmmword ptr [rbp - 0x28]
14161094b: movups xmmword ptr [rsi + 0x10], xmm1
14161094f: movups xmmword ptr [rsi], xmm0
141610952: test r12b, r12b
141610955: je 0x14161096a
141610957: mov edx, 0x10
14161095c: mov r8d, 4
141610962: mov rcx, r15
141610965: call 0x140613c20
14161096a: mov rax, rsi
14161096d: add rsp, 0x120
141610974: pop rbx
141610975: pop rdi
141610976: pop rsi
141610977: pop r12
141610979: pop r14
14161097b: pop r15
14161097d: pop rbp
14161097e: ret 
14161097f: mov qword ptr [rbp + 0x10], 0
141610987: lea rax, [rip + 0x16b95ba]
14161098e: jmp 0x14161099f
141610990: mov qword ptr [rbp + 0x10], 0
141610998: lea rax, [rip + 0x16b95c1]
14161099f: mov qword ptr [rsp + 0x20], rax
1416109a4: lea r8, [rip + 0x16b9595]
1416109ab: lea rdx, [rbp - 0x40]
1416109af: lea r9, [rbp + 0x10]
1416109b3: xor ecx, ecx
1416109b5: call 0x1428d9596
1416109ba: mov dword ptr [rbp + 0x84], r12d
1416109c1: mov qword ptr [rbp + 0x78], r15
1416109c5: movups xmm0, xmmword ptr [rbp]
1416109c9: movaps xmmword ptr [rbp + 0x50], xmm0
1416109cd: movups xmm0, xmmword ptr [rbp - 0x40]
1416109d1: movups xmm1, xmmword ptr [rbp - 0x30]
1416109d5: movups xmm2, xmmword ptr [rbp - 0x20]
1416109d9: movups xmm3, xmmword ptr [rbp - 0x10]
1416109dd: movaps xmmword ptr [rbp + 0x40], xmm3
1416109e1: movaps xmmword ptr [rbp + 0x30], xmm2
1416109e5: movaps xmmword ptr [rbp + 0x20], xmm1
1416109e9: movaps xmmword ptr [rbp + 0x10], xmm0
1416109ed: lea rax, [rip + 0x166f7b4]
1416109f4: mov qword ptr [rsp + 0x20], rax
1416109f9: lea rcx, [rip + 0x166e0e8]
141610a00: lea r9, [rip + 0x166e0c1]
141610a07: lea r8, [rbp + 0x10]
141610a0b: mov edx, 0x2b
141610a10: call 0x1428d9760
141610a15: jmp 0x141610aa1
141610a1a: lea rax, [rip + 0x148d741]
141610a21: mov qword ptr [rbp - 0x40], rax
141610a25: mov qword ptr [rbp - 0x38], 0x47
141610a2d: lea rax, [rbp - 0x40]
141610a31: mov qword ptr [rbp + 0x68], rax
141610a35: lea rax, [rip - 0x1506bac]
141610a3c: mov qword ptr [rbp + 0x70], rax
141610a40: lea rax, [rip + 0x166ed81]
141610a47: mov qword ptr [rbp + 0x10], rax
141610a4b: mov qword ptr [rbp + 0x18], 1
141610a53: mov qword ptr [rbp + 0x30], 0
141610a5b: lea rax, [rbp + 0x68]
141610a5f: mov qword ptr [rbp + 0x20], rax
141610a63: mov qword ptr [rbp + 0x28], 1
141610a6b: mov byte ptr [rbp + 0x97], 0
141610a72: lea rdx, [rip + 0x166ed5f]
141610a79: lea rcx, [rbp + 0x10]
141610a7d: call 0x1428d9390
141610a82: jmp 0x141610aa1
141610a84: mov byte ptr [rbp + 0x97], 0
141610a8b: lea r8, [rip + 0x166e01e]
141610a92: mov ecx, 4
141610a97: mov edx, 0x10
141610a9c: call 0x1428d8fe3
141610aa1: ud2 
141610aa3: nop word ptr cs:[rax + rax]
141610ab0: mov qword ptr [rsp + 0x10], rdx
141610ab5: push rbp
141610ab6: push r15
141610ab8: push r14
141610aba: push r12
141610abc: push rsi
141610abd: push rdi
141610abe: push rbx
141610abf: sub rsp, 0x40
141610ac3: lea rbp, [rdx + 0x80]
141610aca: mov rsi, qword ptr [rbp + 0x78]
141610ace: mov qword ptr [rbp + 0x88], rsi
141610ad5: mov edi, dword ptr [rbp + 0x84]
141610adb: mov byte ptr [rbp + 0x97], dil
141610ae2: lea rcx, [rbp + 0x10]
141610ae6: call 0x1407cea70
141610aeb: mov qword ptr [rbp + 0x88], rsi
141610af2: mov byte ptr [rbp + 0x97], dil
141610af9: add rsp, 0x40
141610afd: pop rbx
141610afe: pop rdi
141610aff: pop rsi
141610b00: pop r12
141610b02: pop r14
141610b04: pop r15
141610b06: pop rbp
141610b07: ret 
141610b08: nop dword ptr [rax + rax]
141610b10: mov qword ptr [rsp + 0x10], rdx
141610b15: push rbp
141610b16: push r15
141610b18: push r14
141610b1a: push r12
141610b1c: push rsi
141610b1d: push rdi
141610b1e: push rbx
141610b1f: sub rsp, 0x40
141610b23: lea rbp, [rdx + 0x80]
141610b2a: test byte ptr [rbp + 0x97], 1
141610b31: je 0x141610b4a
141610b33: mov rcx, qword ptr [rbp + 0x88]
141610b3a: mov edx, 0x10
141610b3f: mov r8d, 4
141610b45: call 0x140613c20
141610b4a: nop 
141610b4b: add rsp, 0x40
141610b4f: pop rbx
141610b50: pop rdi
141610b51: pop rsi
141610b52: pop r12
141610b54: pop r14
141610b56: pop r15
141610b58: pop rbp
141610b59: ret 
