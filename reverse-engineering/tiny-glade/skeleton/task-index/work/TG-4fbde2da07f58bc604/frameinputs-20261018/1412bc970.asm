1412bc970: push rbp
1412bc971: push rsi
1412bc972: sub rsp, 0xd8
1412bc979: lea rbp, [rsp + 0x80]
1412bc981: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bc989: mov rsi, rcx
1412bc98c: lea rcx, [rbp - 0x58]
1412bc990: xor r8d, r8d
1412bc993: call 0x141616120
1412bc998: mov qword ptr [rbp - 0x60], 0
1412bc9a0: movzx eax, byte ptr [rip + 0x2378262]
1412bc9a7: mov ecx, 0x30
1412bc9ac: mov edx, 8
1412bc9b1: call 0x140613c10
1412bc9b6: test rax, rax
1412bc9b9: je 0x1412bca6f
1412bc9bf: movups xmm0, xmmword ptr [rbp - 0x60]
1412bc9c3: movups xmm1, xmmword ptr [rbp - 0x50]
1412bc9c7: movups xmm2, xmmword ptr [rbp - 0x40]
1412bc9cb: movups xmmword ptr [rax + 0x20], xmm2
1412bc9cf: movups xmmword ptr [rax + 0x10], xmm1
1412bc9d3: movups xmmword ptr [rax], xmm0
1412bc9d6: lea rcx, [rip + 0x17e21aa]
1412bc9dd: mov qword ptr [rbp], rcx
1412bc9e1: mov qword ptr [rbp + 8], 0x3d
1412bc9e9: mov qword ptr [rbp + 0x10], rax
1412bc9ed: lea rax, [rip + 0x196fc34]
1412bc9f4: mov qword ptr [rbp + 0x18], rax
1412bc9f8: mov qword ptr [rbp + 0x30], 0
1412bca00: mov byte ptr [rbp + 0x46], 5
1412bca04: mov qword ptr [rbp + 0x20], rcx
1412bca08: mov qword ptr [rbp + 0x28], 0x3d
1412bca10: movzx eax, byte ptr [rip + 0x23781f2]
1412bca17: mov ecx, 0x40
1412bca1c: mov edx, 8
1412bca21: call 0x140613c10
1412bca26: test rax, rax
1412bca29: je 0x1412bca80
1412bca2b: movups xmm0, xmmword ptr [rbp + 0x10]
1412bca2f: movups xmm1, xmmword ptr [rbp + 0x20]
1412bca33: movups xmm2, xmmword ptr [rbp + 0x30]
1412bca37: movups xmm3, xmmword ptr [rbp + 0x40]
1412bca3b: movups xmmword ptr [rax + 0x30], xmm3
1412bca3f: movups xmmword ptr [rax + 0x20], xmm2
1412bca43: movups xmmword ptr [rax + 0x10], xmm1
1412bca47: movups xmmword ptr [rax], xmm0
1412bca4a: movabs rdx, 0x9e3cfa7f2993f406
1412bca54: mov rcx, rsi
1412bca57: mov r8, rax
1412bca5a: call 0x140969bf0
1412bca5f: cmp rax, 1
1412bca63: je 0x1412bca91
1412bca65: add rsp, 0xd8
1412bca6c: pop rsi
1412bca6d: pop rbp
1412bca6e: ret 
1412bca6f: mov ecx, 8
1412bca74: mov edx, 0x30
1412bca79: call 0x1428d9000
1412bca7e: jmp 0x1412bca8f
1412bca80: mov ecx, 8
1412bca85: mov edx, 0x40
1412bca8a: call 0x1428d9000
1412bca8f: ud2 
1412bca91: mov rax, rdx
1412bca94: add rdx, 0x10
1412bca98: mov rsi, rdx
1412bca9b: cmp qword ptr [rax + 0x18], 0x3d
1412bcaa0: jne 0x1412bcb1e
1412bcaa2: mov rdx, qword ptr [rsi]
1412bcaa5: lea rcx, [rip + 0x17e20db]
1412bcaac: mov r8d, 0x3d
1412bcab2: call 0x142923bf0
1412bcab7: test eax, eax
1412bcab9: jne 0x1412bcb1e
1412bcabb: lea rax, [rip + 0x19710c6]
1412bcac2: mov qword ptr [rbp - 0x30], rax
1412bcac6: lea rax, [rip - 0xd8922d]
1412bcacd: mov qword ptr [rbp - 0x28], rax
1412bcad1: mov rax, rbp
1412bcad4: mov qword ptr [rbp - 0x20], rax
1412bcad8: lea rax, [rip - 0x11b2c4f]
1412bcadf: mov qword ptr [rbp - 0x18], rax
1412bcae3: lea rax, [rip + 0x1970b3e]
1412bcaea: mov qword ptr [rbp + 0x10], rax
1412bcaee: mov qword ptr [rbp + 0x18], 3
1412bcaf6: mov qword ptr [rbp + 0x30], 0
1412bcafe: lea rax, [rbp - 0x30]
1412bcb02: mov qword ptr [rbp + 0x20], rax
1412bcb06: mov qword ptr [rbp + 0x28], 2
1412bcb0e: lea rdx, [rip + 0x1970b43]
1412bcb15: lea rcx, [rbp + 0x10]
1412bcb19: call 0x1428d9390
1412bcb1e: lea rax, [rip + 0x1971063]
1412bcb25: mov qword ptr [rbp + 0x10], rax
1412bcb29: lea rax, [rip - 0xd89290]
1412bcb30: mov qword ptr [rbp + 0x18], rax
1412bcb34: mov qword ptr [rbp + 0x20], rsi
1412bcb38: lea rax, [rip - 0x11b2caf]
1412bcb3f: mov qword ptr [rbp + 0x28], rax
1412bcb43: mov rcx, rbp
1412bcb46: mov qword ptr [rbp + 0x30], rcx
1412bcb4a: mov qword ptr [rbp + 0x38], rax
1412bcb4e: lea rax, [rip + 0x1970a13]
1412bcb55: mov qword ptr [rbp - 0x30], rax
1412bcb59: mov qword ptr [rbp - 0x28], 3
1412bcb61: mov qword ptr [rbp - 0x10], 0
1412bcb69: lea rax, [rbp + 0x10]
1412bcb6d: mov qword ptr [rbp - 0x20], rax
1412bcb71: mov qword ptr [rbp - 0x18], 3
1412bcb79: lea rdx, [rip + 0x1970a50]
1412bcb80: lea rcx, [rbp - 0x30]
1412bcb84: call 0x1428d9390
1412bcb89: int3 
1412bcb8a: nop word ptr [rax + rax]
1412bcb90: mov qword ptr [rsp + 0x10], rdx
1412bcb95: push rbp
1412bcb96: push rsi
1412bcb97: sub rsp, 0x28
1412bcb9b: lea rbp, [rdx + 0x80]
1412bcba2: lea rcx, [rbp - 0x58]
1412bcba6: call 0x140be24b0
1412bcbab: nop 
1412bcbac: add rsp, 0x28
1412bcbb0: pop rsi
1412bcbb1: pop rbp
1412bcbb2: ret 
1412bcbb3: nop word ptr cs:[rax + rax]
1412bcbc0: mov qword ptr [rsp + 0x10], rdx
1412bcbc5: push rbp
1412bcbc6: push rsi
1412bcbc7: sub rsp, 0x28
1412bcbcb: lea rbp, [rdx + 0x80]
1412bcbd2: lea rcx, [rbp + 0x10]
1412bcbd6: call 0x1400c04b0
1412bcbdb: nop 
1412bcbdc: add rsp, 0x28
1412bcbe0: pop rsi
1412bcbe1: pop rbp
1412bcbe2: ret 
