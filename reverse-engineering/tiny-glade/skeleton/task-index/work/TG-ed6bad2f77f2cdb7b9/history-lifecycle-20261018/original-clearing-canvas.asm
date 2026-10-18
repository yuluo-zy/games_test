14092df50: push rbp
14092df51: push r15
14092df53: push r14
14092df55: push rsi
14092df56: push rdi
14092df57: push rbx
14092df58: sub rsp, 0x1c8
14092df5f: lea rbp, [rsp + 0x80]
14092df67: mov qword ptr [rbp + 0x140], 0xfffffffffffffffe
14092df72: mov bl, r9b
14092df75: mov rsi, rcx
14092df78: mov qword ptr [rbp + 0x120], rdx
14092df7f: mov qword ptr [rbp + 0x128], r8
14092df86: lea rax, [rbp + 0x120]
14092df8d: mov qword ptr [rbp + 0x110], rax
14092df94: lea rax, [rip - 0x82410b]
14092df9b: mov qword ptr [rbp + 0x118], rax
14092dfa2: lea rax, [rip + 0x2181ea7]
14092dfa9: mov qword ptr [rbp - 0x60], rax
14092dfad: mov qword ptr [rbp - 0x58], 2
14092dfb5: lea rax, [rbp + 0x110]
14092dfbc: mov qword ptr [rbp - 0x50], rax
14092dfc0: mov qword ptr [rbp - 0x48], 1
14092dfc8: mov qword ptr [rbp - 0x40], 0
14092dfd0: lea rcx, [rbp + 0x60]
14092dfd4: lea rdx, [rbp - 0x60]
14092dfd8: call 0x140d79f90
14092dfdd: mov rcx, qword ptr [rbp + 0x68]
14092dfe1: mov rax, qword ptr [rbp + 0x70]
14092dfe5: mov qword ptr [rbp + 0x130], rcx
14092dfec: mov qword ptr [rbp - 0x60], rcx
14092dff0: mov qword ptr [rbp - 0x58], rax
14092dff4: lea rcx, [rbp - 0x60]
14092dff8: call 0x1405bdda0
14092dffd: mov rdi, rax
14092e000: mov r15, qword ptr [rbp + 0x1a8]
14092e007: movzx r14d, byte ptr [rbp + 0x1a0]
14092e00f: mov rdx, qword ptr [rbp + 0x60]
14092e013: test rdx, rdx
14092e016: je 0x14092e02a
14092e018: mov r8d, 1
14092e01e: mov rcx, qword ptr [rbp + 0x130]
14092e025: call 0x140613c20
14092e02a: mov eax, dword ptr [r15 + 0x18]
14092e02e: mov ecx, dword ptr [r15 + 0x2c]
14092e032: mov dword ptr [rbp + 0xa4], ecx
14092e038: movups xmm0, xmmword ptr [r15 + 0x1c]
14092e03d: movups xmmword ptr [rbp + 0x94], xmm0
14092e044: mov ecx, dword ptr [r15 + 0x30]
14092e048: movups xmm0, xmmword ptr [r15 + 0x34]
14092e04d: movups xmmword ptr [rbp + 0xac], xmm0
14092e054: mov edx, dword ptr [r15 + 0x44]
14092e058: mov dword ptr [rbp + 0xbc], edx
14092e05e: mov edx, dword ptr [r15 + 0x48]
14092e062: movups xmm0, xmmword ptr [r15 + 0x4c]
14092e067: movups xmmword ptr [rbp + 0xc4], xmm0
14092e06e: mov r8d, dword ptr [r15 + 0x5c]
14092e072: mov dword ptr [rbp + 0xd4], r8d
14092e079: mov qword ptr [rbp + 0x60], 0
14092e081: mov qword ptr [rbp + 0x68], 8
14092e089: mov qword ptr [rbp + 0x70], 0
14092e091: mov r8, qword ptr [r15]
14092e094: mov qword ptr [rbp + 0x78], r8
14092e098: movups xmm0, xmmword ptr [r15 + 8]
14092e09d: movups xmmword ptr [rbp + 0x80], xmm0
14092e0a4: mov dword ptr [rbp + 0x90], eax
14092e0aa: mov dword ptr [rbp + 0xa8], ecx
14092e0b0: mov dword ptr [rbp + 0xc0], edx
14092e0b6: mov byte ptr [rbp + 0xd8], 1
14092e0bd: mov byte ptr [rbp + 0xd9], bl
14092e0c3: mov byte ptr [rbp + 0xe8], 4
14092e0ca: mov byte ptr [rbp + 0xe9], r14b
14092e0d1: movabs rax, 0x4302000043020000
14092e0db: mov qword ptr [rbp + 0xec], rax
14092e0e2: mov word ptr [rbp + 0x108], 0x100
14092e0eb: lea r14, [rsi + 0x60]
