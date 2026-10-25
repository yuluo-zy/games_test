1412bfba0: push rbp
1412bfba1: push rsi
1412bfba2: push rdi
1412bfba3: sub rsp, 0x140
1412bfbaa: lea rbp, [rsp + 0x80]
1412bfbb2: mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
1412bfbbd: mov rsi, rcx
1412bfbc0: lea rcx, [rbp - 0x58]
1412bfbc4: xor r8d, r8d
1412bfbc7: call 0x1412e4a90
1412bfbcc: movzx eax, byte ptr [rip + 0x2375036]
1412bfbd3: mov ecx, 0x90
1412bfbd8: mov edx, 8
1412bfbdd: call 0x140613c10
1412bfbe2: test rax, rax
1412bfbe5: je 0x1412bfcb2
1412bfbeb: mov rdi, rax
1412bfbee: lea rdx, [rbp - 0x58]
1412bfbf2: mov r8d, 0x90
1412bfbf8: mov rcx, rax
1412bfbfb: call 0x142923c00
1412bfc00: lea rax, [rip + 0x17de7c6]
1412bfc07: mov qword ptr [rbp + 0x68], rax
1412bfc0b: mov qword ptr [rbp + 0x70], 0x35
1412bfc13: mov qword ptr [rbp + 0x78], rdi
1412bfc17: lea rcx, [rip + 0x196ce6a]
1412bfc1e: mov qword ptr [rbp + 0x80], rcx
1412bfc25: mov qword ptr [rbp + 0x98], 0
1412bfc30: mov byte ptr [rbp + 0xae], 5
1412bfc37: mov qword ptr [rbp + 0x88], rax
1412bfc3e: mov qword ptr [rbp + 0x90], 0x35
1412bfc49: movzx eax, byte ptr [rip + 0x2374fb9]
1412bfc50: mov ecx, 0x40
1412bfc55: mov edx, 8
1412bfc5a: call 0x140613c10
1412bfc5f: test rax, rax
1412bfc62: je 0x1412bfcc3
1412bfc64: movups xmm0, xmmword ptr [rbp + 0x78]
1412bfc68: movups xmm1, xmmword ptr [rbp + 0x88]
1412bfc6f: movups xmm2, xmmword ptr [rbp + 0x98]
1412bfc76: movups xmm3, xmmword ptr [rbp + 0xa8]
1412bfc7d: movups xmmword ptr [rax + 0x30], xmm3
1412bfc81: movups xmmword ptr [rax + 0x20], xmm2
1412bfc85: movups xmmword ptr [rax + 0x10], xmm1
1412bfc89: movups xmmword ptr [rax], xmm0
1412bfc8c: movabs rdx, 0x13d939590d705099
1412bfc96: mov rcx, rsi
1412bfc99: mov r8, rax
1412bfc9c: call 0x140969bf0
1412bfca1: cmp rax, 1
1412bfca5: je 0x1412bfcd4
1412bfca7: add rsp, 0x140
1412bfcae: pop rdi
1412bfcaf: pop rsi
1412bfcb0: pop rbp
1412bfcb1: ret 
1412bfcb2: mov ecx, 8
1412bfcb7: mov edx, 0x90
1412bfcbc: call 0x1428d9000
1412bfcc1: jmp 0x1412bfcd2
1412bfcc3: mov ecx, 8
1412bfcc8: mov edx, 0x40
1412bfccd: call 0x1428d9000
1412bfcd2: ud2 
1412bfcd4: mov rax, rdx
1412bfcd7: add rdx, 0x10
1412bfcdb: mov rsi, rdx
1412bfcde: cmp qword ptr [rax + 0x18], 0x35
1412bfce3: jne 0x1412bfd72
1412bfce9: mov rdx, qword ptr [rsi]
1412bfcec: lea rcx, [rip + 0x17de6da]
1412bfcf3: mov r8d, 0x35
1412bfcf9: call 0x142923bf0
1412bfcfe: test eax, eax
1412bfd00: jne 0x1412bfd72
1412bfd02: lea rax, [rip + 0x196dadf]
1412bfd09: mov qword ptr [rbp + 0x38], rax
1412bfd0d: lea rax, [rip - 0xd8c474]
1412bfd14: mov qword ptr [rbp + 0x40], rax
1412bfd18: lea rax, [rbp + 0x68]
1412bfd1c: mov qword ptr [rbp + 0x48], rax
1412bfd20: lea rax, [rip - 0x11b5e97]
1412bfd27: mov qword ptr [rbp + 0x50], rax
1412bfd2b: lea rax, [rip + 0x196d8f6]
1412bfd32: mov qword ptr [rbp + 0x78], rax
1412bfd36: mov qword ptr [rbp + 0x80], 3
1412bfd41: mov qword ptr [rbp + 0x98], 0
1412bfd4c: lea rax, [rbp + 0x38]
1412bfd50: mov qword ptr [rbp + 0x88], rax
1412bfd57: mov qword ptr [rbp + 0x90], 2
1412bfd62: lea rdx, [rip + 0x196d8ef]
1412bfd69: lea rcx, [rbp + 0x78]
1412bfd6d: call 0x1428d9390
1412bfd72: lea rax, [rip + 0x196da6f]
1412bfd79: mov qword ptr [rbp + 0x78], rax
1412bfd7d: lea rax, [rip - 0xd8c4e4]
1412bfd84: mov qword ptr [rbp + 0x80], rax
1412bfd8b: mov qword ptr [rbp + 0x88], rsi
1412bfd92: lea rax, [rip - 0x11b5f09]
1412bfd99: mov qword ptr [rbp + 0x90], rax
1412bfda0: lea rcx, [rbp + 0x68]
1412bfda4: mov qword ptr [rbp + 0x98], rcx
1412bfdab: mov qword ptr [rbp + 0xa0], rax
1412bfdb2: lea rax, [rip + 0x196d7af]
1412bfdb9: mov qword ptr [rbp + 0x38], rax
1412bfdbd: mov qword ptr [rbp + 0x40], 3
1412bfdc5: mov qword ptr [rbp + 0x58], 0
1412bfdcd: lea rax, [rbp + 0x78]
1412bfdd1: mov qword ptr [rbp + 0x48], rax
1412bfdd5: mov qword ptr [rbp + 0x50], 3
1412bfddd: lea rdx, [rip + 0x196d7ec]
1412bfde4: lea rcx, [rbp + 0x38]
1412bfde8: call 0x1428d9390
1412bfded: int3 
1412bfdee: nop 
1412bfdf0: mov qword ptr [rsp + 0x10], rdx
1412bfdf5: push rbp
1412bfdf6: push rsi
1412bfdf7: push rdi
1412bfdf8: sub rsp, 0x20
1412bfdfc: lea rbp, [rdx + 0x80]
1412bfe03: lea rcx, [rbp - 0x58]
1412bfe07: call 0x14087f110
1412bfe0c: nop 
1412bfe0d: add rsp, 0x20
1412bfe11: pop rdi
1412bfe12: pop rsi
1412bfe13: pop rbp
1412bfe14: ret 
1412bfe15: nop word ptr cs:[rax + rax]
1412bfe20: mov qword ptr [rsp + 0x10], rdx
1412bfe25: push rbp
1412bfe26: push rsi
1412bfe27: push rdi
1412bfe28: sub rsp, 0x20
1412bfe2c: lea rbp, [rdx + 0x80]
1412bfe33: lea rcx, [rbp + 0x78]
1412bfe37: call 0x1400c04b0
1412bfe3c: nop 
1412bfe3d: add rsp, 0x20
1412bfe41: pop rdi
1412bfe42: pop rsi
1412bfe43: pop rbp
1412bfe44: ret 
