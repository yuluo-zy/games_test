1416fcda0: push r15
1416fcda2: push r14
1416fcda4: push r12
1416fcda6: push rsi
1416fcda7: push rdi
1416fcda8: push rbx
1416fcda9: sub rsp, 0xb8
1416fcdb0: mov edi, 1
1416fcdb5: lock xadd dword ptr [rdx + 0x5b0], edi
1416fcdbd: test byte ptr [rcx], 1
1416fcdc0: je 0x1416fd02e
1416fcdc6: mov rsi, rcx
1416fcdc9: mov rax, qword ptr [rcx + 8]
1416fcdcd: cmp qword ptr [rdx + 0x218], rax
1416fcdd4: jbe 0x1416fcf6d
1416fcdda: mov rcx, qword ptr [rdx + 0x210]
1416fcde1: mov rcx, qword ptr [rcx + rax*8]
1416fcde5: test rcx, rcx
1416fcde8: je 0x1416fcf6d
1416fcdee: mov rax, qword ptr [rdx + 0x1e0]
1416fcdf5: not rcx
1416fcdf8: lea rcx, [rcx + rcx*2]
1416fcdfc: shl rcx, 5
1416fce00: cmp qword ptr [rax + rcx + 0x20], 0
1416fce06: je 0x1416fcf6d
1416fce0c: add rax, rcx
1416fce0f: mov rcx, qword ptr [rax + 0x10]
1416fce13: mov r8d, dword ptr [rdx + 0x5b0]
1416fce1a: mov r8, qword ptr [rsi + 0x10]
1416fce1e: cmp qword ptr [rdx + 0x218], r8
1416fce25: jbe 0x1416fcf8b
1416fce2b: mov r9, qword ptr [rdx + 0x210]
1416fce32: mov r9, qword ptr [r9 + r8*8]
1416fce36: test r9, r9
1416fce39: je 0x1416fcf8b
1416fce3f: mov r8, qword ptr [rdx + 0x1e0]
1416fce46: not r9
1416fce49: lea r10, [r9 + r9*2]
1416fce4d: shl r10, 5
1416fce51: cmp qword ptr [r8 + r10 + 0x20], 0
1416fce57: je 0x1416fcf8b
1416fce5d: mov r9d, dword ptr [rsi + 0x158]
1416fce64: add r8, r10
1416fce67: mov r10, qword ptr [r8 + 0x10]
1416fce6b: mov r11d, dword ptr [rdx + 0x5b0]
1416fce72: mov r11, qword ptr [rsi + 0x18]
1416fce76: cmp qword ptr [rdx + 0x218], r11
1416fce7d: jbe 0x1416fcfa9
1416fce83: mov rbx, qword ptr [rdx + 0x210]
1416fce8a: mov rbx, qword ptr [rbx + r11*8]
1416fce8e: test rbx, rbx
1416fce91: je 0x1416fcfa9
1416fce97: mov r11, qword ptr [rdx + 0x1e0]
1416fce9e: not rbx
1416fcea1: lea rbx, [rbx + rbx*2]
1416fcea5: shl rbx, 5
1416fcea9: cmp qword ptr [r11 + rbx + 0x20], 0
1416fceaf: je 0x1416fcfa9
1416fceb5: add r11, rbx
1416fceb8: lea rbx, [r8 + 0x5c]
1416fcebc: add r8, 0x58
1416fcec0: lea r14, [rax + 0x5c]
1416fcec4: add rax, 0x58
1416fcec8: mov r15, qword ptr [r11 + 0x10]
1416fcecc: lea r12, [r11 + 0x58]
1416fced0: add r11, 0x5c
1416fced4: mov edx, dword ptr [rdx + 0x5b0]
1416fceda: mov qword ptr [rsp + 0x58], rcx
1416fcedf: mov qword ptr [rsp + 0x60], rax
1416fcee4: mov qword ptr [rsp + 0x68], r14
1416fcee9: mov dword ptr [rsp + 0x70], r9d
1416fceee: mov dword ptr [rsp + 0x74], edi
1416fcef2: lea rdx, [rsp + 0x78]
1416fcef7: mov qword ptr [rsp + 0x78], r10
1416fcefc: mov qword ptr [rsp + 0x80], r8
1416fcf04: mov qword ptr [rsp + 0x88], rbx
1416fcf0c: mov dword ptr [rsp + 0x90], r9d
1416fcf14: mov dword ptr [rsp + 0x94], edi
1416fcf1b: lea r8, [rsp + 0x98]
1416fcf23: mov qword ptr [rsp + 0x98], r15
1416fcf2b: mov qword ptr [rsp + 0xa0], r12
1416fcf33: mov qword ptr [rsp + 0xa8], r11
1416fcf3b: mov dword ptr [rsp + 0xb0], r9d
1416fcf43: mov dword ptr [rsp + 0xb4], edi
1416fcf4a: lea rcx, [rsp + 0x58]
1416fcf4f: call 0x140912a40
1416fcf54: mov dword ptr [rsi + 0x158], edi
1416fcf5a: xor eax, eax
1416fcf5c: add rsp, 0xb8
1416fcf63: pop rbx
1416fcf64: pop rdi
1416fcf65: pop rsi
1416fcf66: pop r12
1416fcf68: pop r14
1416fcf6a: pop r15
1416fcf6c: ret 
1416fcf6d: add rsi, 0xc0
1416fcf74: lea rax, [rip + 0x1595976]
1416fcf7b: mov qword ptr [rsp + 0x28], rax
1416fcf80: mov qword ptr [rsp + 0x30], 0x2d
1416fcf89: jmp 0x1416fcfc5
1416fcf8b: add rsi, 0xc0
1416fcf92: lea rax, [rip + 0x15958b3]
1416fcf99: mov qword ptr [rsp + 0x28], rax
1416fcf9e: mov qword ptr [rsp + 0x30], 0x32
1416fcfa7: jmp 0x1416fcfc5
1416fcfa9: add rsi, 0xc0
1416fcfb0: lea rax, [rip + 0x15958e1]
1416fcfb7: mov qword ptr [rsp + 0x28], rax
1416fcfbc: mov qword ptr [rsp + 0x30], 0x59
1416fcfc5: mov qword ptr [rsp + 0x38], rsi
1416fcfca: lea rax, [rip - 0x16c1501]
1416fcfd1: mov qword ptr [rsp + 0x40], rax
1416fcfd6: lea rax, [rsp + 0x28]
1416fcfdb: mov qword ptr [rsp + 0x48], rax
1416fcfe0: lea rax, [rip - 0x15f3157]
1416fcfe7: mov qword ptr [rsp + 0x50], rax
1416fcfec: lea rax, [rip + 0x159547d]
1416fcff3: mov qword ptr [rsp + 0x58], rax
1416fcff8: mov qword ptr [rsp + 0x60], 2
1416fd001: mov qword ptr [rsp + 0x78], 0
1416fd00a: lea rax, [rsp + 0x38]
1416fd00f: mov qword ptr [rsp + 0x68], rax
1416fd014: mov qword ptr [rsp + 0x70], 2
1416fd01d: lea rdx, [rip + 0x159585c]
1416fd024: lea rcx, [rsp + 0x58]
1416fd029: call 0x1428d9390
1416fd02e: lea rcx, [rip + 0x1595a8b]
1416fd035: lea r8, [rip + 0x1595ae4]
1416fd03c: mov edx, 0x59
1416fd041: call 0x1428d9330
1416fd046: int3 
