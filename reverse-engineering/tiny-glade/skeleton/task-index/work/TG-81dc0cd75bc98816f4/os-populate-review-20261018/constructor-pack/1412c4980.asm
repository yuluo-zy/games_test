1412c4980: push rbp
1412c4981: push rsi
1412c4982: sub rsp, 0xd8
1412c4989: lea rbp, [rsp + 0x80]
1412c4991: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c4999: mov rsi, rcx
1412c499c: lea rcx, [rbp - 0x58]
1412c49a0: xor r8d, r8d
1412c49a3: call 0x141619ab0
1412c49a8: mov qword ptr [rbp - 0x60], 0
1412c49b0: movzx eax, byte ptr [rip + 0x2370252]
1412c49b7: mov ecx, 0x30
1412c49bc: mov edx, 8
1412c49c1: call 0x140613c10
1412c49c6: test rax, rax
1412c49c9: je 0x1412c4a7f
1412c49cf: movups xmm0, xmmword ptr [rbp - 0x60]
1412c49d3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c49d7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c49db: movups xmmword ptr [rax + 0x20], xmm2
1412c49df: movups xmmword ptr [rax + 0x10], xmm1
1412c49e3: movups xmmword ptr [rax], xmm0
1412c49e6: lea rcx, [rip + 0x17da010]
1412c49ed: mov qword ptr [rbp], rcx
1412c49f1: mov qword ptr [rbp + 8], 0x53
1412c49f9: mov qword ptr [rbp + 0x10], rax
1412c49fd: lea rax, [rip + 0x196874c]
1412c4a04: mov qword ptr [rbp + 0x18], rax
1412c4a08: mov qword ptr [rbp + 0x30], 0
1412c4a10: mov byte ptr [rbp + 0x46], 5
1412c4a14: mov qword ptr [rbp + 0x20], rcx
1412c4a18: mov qword ptr [rbp + 0x28], 0x53
1412c4a20: movzx eax, byte ptr [rip + 0x23701e2]
1412c4a27: mov ecx, 0x40
1412c4a2c: mov edx, 8
1412c4a31: call 0x140613c10
1412c4a36: test rax, rax
1412c4a39: je 0x1412c4a90
1412c4a3b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c4a3f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c4a43: movups xmm2, xmmword ptr [rbp + 0x30]
1412c4a47: movups xmm3, xmmword ptr [rbp + 0x40]
1412c4a4b: movups xmmword ptr [rax + 0x30], xmm3
1412c4a4f: movups xmmword ptr [rax + 0x20], xmm2
1412c4a53: movups xmmword ptr [rax + 0x10], xmm1
1412c4a57: movups xmmword ptr [rax], xmm0
1412c4a5a: movabs rdx, 0x45a92b71a96e360f
1412c4a64: mov rcx, rsi
1412c4a67: mov r8, rax
1412c4a6a: call 0x140969bf0
1412c4a6f: cmp rax, 1
1412c4a73: je 0x1412c4aa1
1412c4a75: add rsp, 0xd8
1412c4a7c: pop rsi
1412c4a7d: pop rbp
1412c4a7e: ret 
1412c4a7f: mov ecx, 8
1412c4a84: mov edx, 0x30
1412c4a89: call 0x1428d9000
1412c4a8e: jmp 0x1412c4a9f
1412c4a90: mov ecx, 8
1412c4a95: mov edx, 0x40
1412c4a9a: call 0x1428d9000
1412c4a9f: ud2 
1412c4aa1: mov rax, rdx
1412c4aa4: add rdx, 0x10
1412c4aa8: mov rsi, rdx
1412c4aab: cmp qword ptr [rax + 0x18], 0x53
1412c4ab0: jne 0x1412c4b2e
1412c4ab2: mov rdx, qword ptr [rsi]
1412c4ab5: lea rcx, [rip + 0x17d9f41]
1412c4abc: mov r8d, 0x53
1412c4ac2: call 0x142923bf0
1412c4ac7: test eax, eax
1412c4ac9: jne 0x1412c4b2e
1412c4acb: lea rax, [rip + 0x1968bce]
1412c4ad2: mov qword ptr [rbp - 0x30], rax
1412c4ad6: lea rax, [rip - 0xd9123d]
1412c4add: mov qword ptr [rbp - 0x28], rax
1412c4ae1: mov rax, rbp
1412c4ae4: mov qword ptr [rbp - 0x20], rax
1412c4ae8: lea rax, [rip - 0x11bac5f]
1412c4aef: mov qword ptr [rbp - 0x18], rax
1412c4af3: lea rax, [rip + 0x1968b2e]
1412c4afa: mov qword ptr [rbp + 0x10], rax
1412c4afe: mov qword ptr [rbp + 0x18], 3
1412c4b06: mov qword ptr [rbp + 0x30], 0
1412c4b0e: lea rax, [rbp - 0x30]
1412c4b12: mov qword ptr [rbp + 0x20], rax
1412c4b16: mov qword ptr [rbp + 0x28], 2
1412c4b1e: lea rdx, [rip + 0x1968b33]
1412c4b25: lea rcx, [rbp + 0x10]
1412c4b29: call 0x1428d9390
1412c4b2e: lea rax, [rip + 0x1968b6b]
1412c4b35: mov qword ptr [rbp + 0x10], rax
1412c4b39: lea rax, [rip - 0xd912a0]
1412c4b40: mov qword ptr [rbp + 0x18], rax
1412c4b44: mov qword ptr [rbp + 0x20], rsi
1412c4b48: lea rax, [rip - 0x11bacbf]
1412c4b4f: mov qword ptr [rbp + 0x28], rax
1412c4b53: mov rcx, rbp
1412c4b56: mov qword ptr [rbp + 0x30], rcx
1412c4b5a: mov qword ptr [rbp + 0x38], rax
1412c4b5e: lea rax, [rip + 0x1968a03]
1412c4b65: mov qword ptr [rbp - 0x30], rax
1412c4b69: mov qword ptr [rbp - 0x28], 3
1412c4b71: mov qword ptr [rbp - 0x10], 0
1412c4b79: lea rax, [rbp + 0x10]
1412c4b7d: mov qword ptr [rbp - 0x20], rax
1412c4b81: mov qword ptr [rbp - 0x18], 3
1412c4b89: lea rdx, [rip + 0x1968a40]
1412c4b90: lea rcx, [rbp - 0x30]
1412c4b94: call 0x1428d9390
1412c4b99: int3 
1412c4b9a: nop word ptr [rax + rax]
1412c4ba0: mov qword ptr [rsp + 0x10], rdx
1412c4ba5: push rbp
1412c4ba6: push rsi
1412c4ba7: sub rsp, 0x28
1412c4bab: lea rbp, [rdx + 0x80]
1412c4bb2: lea rcx, [rbp - 0x58]
1412c4bb6: call 0x140be24b0
1412c4bbb: nop 
1412c4bbc: add rsp, 0x28
1412c4bc0: pop rsi
1412c4bc1: pop rbp
1412c4bc2: ret 
1412c4bc3: nop word ptr cs:[rax + rax]
1412c4bd0: mov qword ptr [rsp + 0x10], rdx
1412c4bd5: push rbp
1412c4bd6: push rsi
1412c4bd7: sub rsp, 0x28
1412c4bdb: lea rbp, [rdx + 0x80]
1412c4be2: lea rcx, [rbp + 0x10]
1412c4be6: call 0x1400c04b0
1412c4beb: nop 
1412c4bec: add rsp, 0x28
1412c4bf0: pop rsi
1412c4bf1: pop rbp
1412c4bf2: ret 
