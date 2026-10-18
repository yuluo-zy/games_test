141737ae0: push rbp
141737ae1: push rsi
141737ae2: push rdi
141737ae3: push rbx
141737ae4: sub rsp, 0x2a8
141737aeb: lea rbp, [rsp + 0x80]
141737af3: mov qword ptr [rbp + 0x220], 0xfffffffffffffffe
141737afe: mov rsi, rcx
141737b01: movups xmm0, xmmword ptr [rdx + 0x60]
141737b05: movaps xmmword ptr [rbp + 0x70], xmm0
141737b09: movups xmm0, xmmword ptr [rdx + 0x50]
141737b0d: movaps xmmword ptr [rbp + 0x60], xmm0
141737b11: movups xmm0, xmmword ptr [rdx + 0x40]
141737b15: movaps xmmword ptr [rbp + 0x50], xmm0
141737b19: movups xmm0, xmmword ptr [rdx]
141737b1c: movups xmm1, xmmword ptr [rdx + 0x10]
141737b20: movups xmm2, xmmword ptr [rdx + 0x20]
141737b24: movups xmm3, xmmword ptr [rdx + 0x30]
141737b28: movaps xmmword ptr [rbp + 0x40], xmm3
141737b2c: movaps xmmword ptr [rbp + 0x30], xmm2
141737b30: movaps xmmword ptr [rbp + 0x20], xmm1
141737b34: movaps xmmword ptr [rbp + 0x10], xmm0
141737b38: movzx eax, byte ptr [rip + 0x1efd0ca]
141737b3f: mov ecx, 0x150
141737b44: mov edx, 8
141737b49: call 0x140613c10
141737b4e: test rax, rax
141737b51: je 0x141737e87
141737b57: mov qword ptr [rbp + 0x218], rax
141737b5e: xorps xmm0, xmm0
141737b61: movups xmmword ptr [rbp + 0xf8], xmm0
141737b68: movups xmmword ptr [rbp + 0x110], xmm0
141737b6f: movups xmmword ptr [rbp + 0x128], xmm0
141737b76: movups xmmword ptr [rbp + 0x140], xmm0
141737b7d: movups xmmword ptr [rbp + 0x158], xmm0
141737b84: mov dword ptr [rbp + 0x168], 0
141737b8e: movups xmmword ptr [rbp + 0x190], xmm0
141737b95: movups xmmword ptr [rbp + 0x1a8], xmm0
141737b9c: movups xmmword ptr [rbp + 0x1c0], xmm0
141737ba3: movups xmmword ptr [rbp + 0x1d8], xmm0
141737baa: movups xmmword ptr [rbp + 0x1f0], xmm0
141737bb1: mov dword ptr [rbp + 0x200], 0
141737bbb: mov qword ptr [rbp + 0x80], 0
141737bc6: mov qword ptr [rbp + 0xd8], 0
141737bd1: mov qword ptr [rbp + 0xe0], 8
141737bdc: mov qword ptr [rbp + 0xe8], 0
141737be7: mov qword ptr [rbp + 0xf0], 0x10
141737bf2: mov qword ptr [rbp + 0x108], 0x10
141737bfd: mov qword ptr [rbp + 0x120], 0x10
141737c08: mov qword ptr [rbp + 0x138], 0x10
141737c13: mov qword ptr [rbp + 0x150], 0x10
141737c1e: movabs rbx, 0x8000000000000000
141737c28: mov qword ptr [rbp + 0x170], rbx
141737c2f: lea rax, [rip + 0x1595897]
141737c36: mov qword ptr [rbp + 0x178], rax
141737c3d: mov qword ptr [rbp + 0x180], 0x44
