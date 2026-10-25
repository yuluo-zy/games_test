1412c5880: push rbp
1412c5881: push rsi
1412c5882: sub rsp, 0xd8
1412c5889: lea rbp, [rsp + 0x80]
1412c5891: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c5899: mov rsi, rcx
1412c589c: lea rcx, [rbp - 0x58]
1412c58a0: xor r8d, r8d
1412c58a3: call 0x141610db0
1412c58a8: movzx eax, byte ptr [rip + 0x236f35a]
1412c58af: mov ecx, 0x28
1412c58b4: mov edx, 8
1412c58b9: call 0x140613c10
1412c58be: test rax, rax
1412c58c1: je 0x1412c5977
1412c58c7: mov rcx, qword ptr [rbp - 0x38]
1412c58cb: mov qword ptr [rax + 0x20], rcx
1412c58cf: movups xmm0, xmmword ptr [rbp - 0x58]
1412c58d3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c58d7: movups xmmword ptr [rax + 0x10], xmm1
1412c58db: movups xmmword ptr [rax], xmm0
1412c58de: lea rcx, [rip + 0x17d8b1d]
1412c58e5: mov qword ptr [rbp], rcx
1412c58e9: mov qword ptr [rbp + 8], 0x55
1412c58f1: mov qword ptr [rbp + 0x10], rax
1412c58f5: lea rax, [rip + 0x19679a4]
1412c58fc: mov qword ptr [rbp + 0x18], rax
1412c5900: mov qword ptr [rbp + 0x30], 0
1412c5908: mov byte ptr [rbp + 0x46], 5
1412c590c: mov qword ptr [rbp + 0x20], rcx
1412c5910: mov qword ptr [rbp + 0x28], 0x55
1412c5918: movzx eax, byte ptr [rip + 0x236f2ea]
1412c591f: mov ecx, 0x40
1412c5924: mov edx, 8
1412c5929: call 0x140613c10
1412c592e: test rax, rax
1412c5931: je 0x1412c5988
1412c5933: movups xmm0, xmmword ptr [rbp + 0x10]
1412c5937: movups xmm1, xmmword ptr [rbp + 0x20]
1412c593b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c593f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c5943: movups xmmword ptr [rax + 0x30], xmm3
1412c5947: movups xmmword ptr [rax + 0x20], xmm2
1412c594b: movups xmmword ptr [rax + 0x10], xmm1
1412c594f: movups xmmword ptr [rax], xmm0
1412c5952: movabs rdx, 0x392675f5c9079eea
1412c595c: mov rcx, rsi
1412c595f: mov r8, rax
1412c5962: call 0x140969bf0
1412c5967: cmp rax, 1
1412c596b: je 0x1412c5999
1412c596d: add rsp, 0xd8
1412c5974: pop rsi
1412c5975: pop rbp
1412c5976: ret 
1412c5977: mov ecx, 8
1412c597c: mov edx, 0x28
1412c5981: call 0x1428d9000
1412c5986: jmp 0x1412c5997
1412c5988: mov ecx, 8
1412c598d: mov edx, 0x40
1412c5992: call 0x1428d9000
1412c5997: ud2 
1412c5999: mov rax, rdx
1412c599c: add rdx, 0x10
1412c59a0: mov rsi, rdx
1412c59a3: cmp qword ptr [rax + 0x18], 0x55
1412c59a8: jne 0x1412c5a26
1412c59aa: mov rdx, qword ptr [rsi]
1412c59ad: lea rcx, [rip + 0x17d8a4e]
1412c59b4: mov r8d, 0x55
1412c59ba: call 0x142923bf0
1412c59bf: test eax, eax
1412c59c1: jne 0x1412c5a26
1412c59c3: lea rax, [rip + 0x19680ee]
1412c59ca: mov qword ptr [rbp - 0x30], rax
1412c59ce: lea rax, [rip - 0xd92135]
1412c59d5: mov qword ptr [rbp - 0x28], rax
1412c59d9: mov rax, rbp
1412c59dc: mov qword ptr [rbp - 0x20], rax
1412c59e0: lea rax, [rip - 0x11bbb57]
1412c59e7: mov qword ptr [rbp - 0x18], rax
1412c59eb: lea rax, [rip + 0x1967c36]
1412c59f2: mov qword ptr [rbp + 0x10], rax
1412c59f6: mov qword ptr [rbp + 0x18], 3
1412c59fe: mov qword ptr [rbp + 0x30], 0
1412c5a06: lea rax, [rbp - 0x30]
1412c5a0a: mov qword ptr [rbp + 0x20], rax
1412c5a0e: mov qword ptr [rbp + 0x28], 2
1412c5a16: lea rdx, [rip + 0x1967c3b]
1412c5a1d: lea rcx, [rbp + 0x10]
1412c5a21: call 0x1428d9390
1412c5a26: lea rax, [rip + 0x196808b]
1412c5a2d: mov qword ptr [rbp + 0x10], rax
1412c5a31: lea rax, [rip - 0xd92198]
1412c5a38: mov qword ptr [rbp + 0x18], rax
1412c5a3c: mov qword ptr [rbp + 0x20], rsi
1412c5a40: lea rax, [rip - 0x11bbbb7]
1412c5a47: mov qword ptr [rbp + 0x28], rax
1412c5a4b: mov rcx, rbp
1412c5a4e: mov qword ptr [rbp + 0x30], rcx
1412c5a52: mov qword ptr [rbp + 0x38], rax
1412c5a56: lea rax, [rip + 0x1967b0b]
1412c5a5d: mov qword ptr [rbp - 0x30], rax
1412c5a61: mov qword ptr [rbp - 0x28], 3
1412c5a69: mov qword ptr [rbp - 0x10], 0
1412c5a71: lea rax, [rbp + 0x10]
1412c5a75: mov qword ptr [rbp - 0x20], rax
1412c5a79: mov qword ptr [rbp - 0x18], 3
1412c5a81: lea rdx, [rip + 0x1967b48]
1412c5a88: lea rcx, [rbp - 0x30]
1412c5a8c: call 0x1428d9390
1412c5a91: int3 
1412c5a92: nop word ptr cs:[rax + rax]
1412c5aa0: mov qword ptr [rsp + 0x10], rdx
1412c5aa5: push rbp
1412c5aa6: push rsi
1412c5aa7: sub rsp, 0x28
1412c5aab: lea rbp, [rdx + 0x80]
1412c5ab2: lea rcx, [rbp - 0x58]
1412c5ab6: call 0x140be24b0
1412c5abb: nop 
1412c5abc: add rsp, 0x28
1412c5ac0: pop rsi
1412c5ac1: pop rbp
1412c5ac2: ret 
1412c5ac3: nop word ptr cs:[rax + rax]
1412c5ad0: mov qword ptr [rsp + 0x10], rdx
1412c5ad5: push rbp
1412c5ad6: push rsi
1412c5ad7: sub rsp, 0x28
1412c5adb: lea rbp, [rdx + 0x80]
1412c5ae2: lea rcx, [rbp + 0x10]
1412c5ae6: call 0x1400c04b0
1412c5aeb: nop 
1412c5aec: add rsp, 0x28
1412c5af0: pop rsi
1412c5af1: pop rbp
1412c5af2: ret 
