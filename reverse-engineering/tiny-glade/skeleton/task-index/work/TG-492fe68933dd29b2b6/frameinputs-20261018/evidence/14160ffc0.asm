14160ffc0: push rbp
14160ffc1: push r15
14160ffc3: push r14
14160ffc5: push r12
14160ffc7: push rsi
14160ffc8: push rdi
14160ffc9: push rbx
14160ffca: sub rsp, 0x100
14160ffd1: lea rbp, [rsp + 0x80]
14160ffd9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
14160ffe1: test r8, r8
14160ffe4: je 0x1416100de
14160ffea: mov r14, r9
14160ffed: mov rax, r9
14160fff0: and rax, 3
14160fff4: mov qword ptr [rbp + 0x18], rax
14160fff8: jne 0x1416100a3
14160fffe: mov rbx, r8
141610001: mov rax, r8
141610004: and rax, 3
141610008: mov qword ptr [rbp + 0x18], rax
14161000c: jne 0x1416100b4
141610012: mov rdi, rdx
141610015: mov rsi, rcx
141610018: lea r15, [rbp - 0x40]
14161001c: mov rcx, r15
14161001f: mov rdx, r14
141610022: mov r8d, 0x22
141610028: mov r9b, 1
14161002b: call 0x140bd4de0
141610030: lea r12, [rbp + 0x68]
141610034: mov r8d, 4
14161003a: mov rcx, r12
14161003d: mov rdx, r15
141610040: call 0x140bd4f20
141610045: mov qword ptr [rsp + 0x30], r14
14161004a: mov qword ptr [rsp + 0x28], rbx
14161004f: mov qword ptr [rsp + 0x20], r12
141610054: lea r8, [rip + 0x1670048]
14161005b: lea rcx, [rbp + 0x18]
14161005f: mov r9d, 0x16
141610065: mov rdx, rdi
141610068: call 0x1416a2dc0
14161006d: cmp dword ptr [rbp + 0x18], 0xb
141610071: jne 0x141610155
141610077: mov rax, qword ptr [rbp + 0x40]
14161007b: mov qword ptr [rsi + 0x20], rax
14161007f: movups xmm0, xmmword ptr [rbp + 0x20]
141610083: movups xmm1, xmmword ptr [rbp + 0x30]
141610087: movups xmmword ptr [rsi + 0x10], xmm1
14161008b: movups xmmword ptr [rsi], xmm0
14161008e: mov rax, rsi
141610091: add rsp, 0x100
141610098: pop rbx
141610099: pop rdi
14161009a: pop rsi
14161009b: pop r12
14161009d: pop r14
14161009f: pop r15
1416100a1: pop rbp
1416100a2: ret 
1416100a3: mov qword ptr [rbp - 0x40], 0
1416100ab: lea rax, [rip + 0x16b9e96]
1416100b2: jmp 0x1416100c3
1416100b4: mov qword ptr [rbp - 0x40], 0
1416100bc: lea rax, [rip + 0x16b9e9d]
1416100c3: mov qword ptr [rsp + 0x20], rax
1416100c8: lea r8, [rip + 0x16b9e71]
1416100cf: lea rdx, [rbp + 0x18]
1416100d3: lea r9, [rbp - 0x40]
1416100d7: xor ecx, ecx
1416100d9: call 0x1428d9596
1416100de: lea rax, [rip + 0x148d5ff]
1416100e5: mov qword ptr [rbp + 0x68], rax
1416100e9: mov qword ptr [rbp + 0x70], 0x3b
1416100f1: lea rax, [rip + 0x166ffc8]
1416100f8: mov qword ptr [rbp + 0x18], rax
1416100fc: lea rax, [rip - 0x10dc863]
141610103: mov qword ptr [rbp + 0x20], rax
141610107: lea rax, [rbp + 0x68]
14161010b: mov qword ptr [rbp + 0x28], rax
14161010f: lea rax, [rip - 0x1506286]
141610116: mov qword ptr [rbp + 0x30], rax
14161011a: lea rax, [rip + 0x166f60f]
141610121: mov qword ptr [rbp - 0x40], rax
141610125: mov qword ptr [rbp - 0x38], 3
14161012d: mov qword ptr [rbp - 0x20], 0
141610135: lea rax, [rbp + 0x18]
141610139: mov qword ptr [rbp - 0x30], rax
14161013d: mov qword ptr [rbp - 0x28], 2
141610145: lea rdx, [rip + 0x166f63c]
14161014c: lea rcx, [rbp - 0x40]
141610150: call 0x1428d9390
141610155: movups xmm0, xmmword ptr [rbp + 0x58]
141610159: movaps xmmword ptr [rbp], xmm0
14161015d: movups xmm0, xmmword ptr [rbp + 0x18]
141610161: movups xmm1, xmmword ptr [rbp + 0x28]
141610165: movups xmm2, xmmword ptr [rbp + 0x38]
141610169: movups xmm3, xmmword ptr [rbp + 0x48]
14161016d: movaps xmmword ptr [rbp - 0x10], xmm3
141610171: movaps xmmword ptr [rbp - 0x20], xmm2
141610175: movaps xmmword ptr [rbp - 0x30], xmm1
141610179: movaps xmmword ptr [rbp - 0x40], xmm0
14161017d: lea rax, [rip + 0x1670024]
141610184: mov qword ptr [rsp + 0x20], rax
141610189: lea rcx, [rip + 0x166e958]
141610190: lea r9, [rip + 0x166e931]
141610197: lea r8, [rbp - 0x40]
14161019b: mov edx, 0x2b
1416101a0: call 0x1428d9760
1416101a5: ud2 
1416101a7: nop word ptr [rax + rax]
1416101b0: mov qword ptr [rsp + 0x10], rdx
1416101b5: push rbp
1416101b6: push r15
1416101b8: push r14
1416101ba: push r12
1416101bc: push rsi
1416101bd: push rdi
1416101be: push rbx
1416101bf: sub rsp, 0x40
1416101c3: lea rbp, [rdx + 0x80]
1416101ca: lea rcx, [rbp - 0x40]
1416101ce: call 0x1407cea70
1416101d3: nop 
1416101d4: add rsp, 0x40
1416101d8: pop rbx
1416101d9: pop rdi
1416101da: pop rsi
1416101db: pop r12
1416101dd: pop r14
1416101df: pop r15
1416101e1: pop rbp
1416101e2: ret 
