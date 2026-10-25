1412bdaf0: push rbp
1412bdaf1: push rsi
1412bdaf2: sub rsp, 0xe8
1412bdaf9: lea rbp, [rsp + 0x80]
1412bdb01: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412bdb09: mov rsi, rcx
1412bdb0c: lea rcx, [rbp - 0x58]
1412bdb10: xor r8d, r8d
1412bdb13: call 0x14161d1f0
1412bdb18: movzx eax, byte ptr [rip + 0x23770ea]
1412bdb1f: mov ecx, 0x38
1412bdb24: mov edx, 8
1412bdb29: call 0x140613c10
1412bdb2e: test rax, rax
1412bdb31: je 0x1412bdbef
1412bdb37: mov rcx, qword ptr [rbp - 0x28]
1412bdb3b: mov qword ptr [rax + 0x30], rcx
1412bdb3f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bdb43: movups xmm1, xmmword ptr [rbp - 0x48]
1412bdb47: movups xmm2, xmmword ptr [rbp - 0x38]
1412bdb4b: movups xmmword ptr [rax + 0x20], xmm2
1412bdb4f: movups xmmword ptr [rax + 0x10], xmm1
1412bdb53: movups xmmword ptr [rax], xmm0
1412bdb56: lea rcx, [rip + 0x17e094f]
1412bdb5d: mov qword ptr [rbp + 0x10], rcx
1412bdb61: mov qword ptr [rbp + 0x18], 0x49
1412bdb69: mov qword ptr [rbp + 0x20], rax
1412bdb6d: lea rax, [rip + 0x196ec3c]
1412bdb74: mov qword ptr [rbp + 0x28], rax
1412bdb78: mov qword ptr [rbp + 0x40], 0
1412bdb80: mov byte ptr [rbp + 0x56], 5
1412bdb84: mov qword ptr [rbp + 0x30], rcx
1412bdb88: mov qword ptr [rbp + 0x38], 0x49
1412bdb90: movzx eax, byte ptr [rip + 0x2377072]
1412bdb97: mov ecx, 0x40
1412bdb9c: mov edx, 8
1412bdba1: call 0x140613c10
1412bdba6: test rax, rax
1412bdba9: je 0x1412bdc00
1412bdbab: movups xmm0, xmmword ptr [rbp + 0x20]
1412bdbaf: movups xmm1, xmmword ptr [rbp + 0x30]
1412bdbb3: movups xmm2, xmmword ptr [rbp + 0x40]
1412bdbb7: movups xmm3, xmmword ptr [rbp + 0x50]
1412bdbbb: movups xmmword ptr [rax + 0x30], xmm3
1412bdbbf: movups xmmword ptr [rax + 0x20], xmm2
1412bdbc3: movups xmmword ptr [rax + 0x10], xmm1
1412bdbc7: movups xmmword ptr [rax], xmm0
1412bdbca: movabs rdx, 0x802ff23dee42ab47
1412bdbd4: mov rcx, rsi
1412bdbd7: mov r8, rax
1412bdbda: call 0x140969bf0
1412bdbdf: cmp rax, 1
1412bdbe3: je 0x1412bdc11
1412bdbe5: add rsp, 0xe8
1412bdbec: pop rsi
1412bdbed: pop rbp
1412bdbee: ret 
1412bdbef: mov ecx, 8
1412bdbf4: mov edx, 0x38
1412bdbf9: call 0x1428d9000
1412bdbfe: jmp 0x1412bdc0f
1412bdc00: mov ecx, 8
1412bdc05: mov edx, 0x40
1412bdc0a: call 0x1428d9000
1412bdc0f: ud2 
1412bdc11: mov rax, rdx
1412bdc14: add rdx, 0x10
1412bdc18: mov rsi, rdx
1412bdc1b: cmp qword ptr [rax + 0x18], 0x49
1412bdc20: jne 0x1412bdc9f
1412bdc22: mov rdx, qword ptr [rsi]
1412bdc25: lea rcx, [rip + 0x17e0880]
1412bdc2c: mov r8d, 0x49
1412bdc32: call 0x142923bf0
1412bdc37: test eax, eax
1412bdc39: jne 0x1412bdc9f
1412bdc3b: lea rax, [rip + 0x196fbfe]
1412bdc42: mov qword ptr [rbp - 0x20], rax
1412bdc46: lea rax, [rip - 0xd8a3ad]
1412bdc4d: mov qword ptr [rbp - 0x18], rax
1412bdc51: lea rax, [rbp + 0x10]
1412bdc55: mov qword ptr [rbp - 0x10], rax
1412bdc59: lea rax, [rip - 0x11b3dd0]
1412bdc60: mov qword ptr [rbp - 8], rax
1412bdc64: lea rax, [rip + 0x196f9bd]
1412bdc6b: mov qword ptr [rbp + 0x20], rax
1412bdc6f: mov qword ptr [rbp + 0x28], 3
1412bdc77: mov qword ptr [rbp + 0x40], 0
1412bdc7f: lea rax, [rbp - 0x20]
1412bdc83: mov qword ptr [rbp + 0x30], rax
1412bdc87: mov qword ptr [rbp + 0x38], 2
1412bdc8f: lea rdx, [rip + 0x196f9c2]
1412bdc96: lea rcx, [rbp + 0x20]
1412bdc9a: call 0x1428d9390
1412bdc9f: lea rax, [rip + 0x196fb9a]
1412bdca6: mov qword ptr [rbp + 0x20], rax
1412bdcaa: lea rax, [rip - 0xd8a411]
1412bdcb1: mov qword ptr [rbp + 0x28], rax
1412bdcb5: mov qword ptr [rbp + 0x30], rsi
1412bdcb9: lea rax, [rip - 0x11b3e30]
1412bdcc0: mov qword ptr [rbp + 0x38], rax
1412bdcc4: lea rcx, [rbp + 0x10]
1412bdcc8: mov qword ptr [rbp + 0x40], rcx
1412bdccc: mov qword ptr [rbp + 0x48], rax
1412bdcd0: lea rax, [rip + 0x196f891]
1412bdcd7: mov qword ptr [rbp - 0x20], rax
1412bdcdb: mov qword ptr [rbp - 0x18], 3
1412bdce3: mov qword ptr [rbp], 0
1412bdceb: lea rax, [rbp + 0x20]
1412bdcef: mov qword ptr [rbp - 0x10], rax
1412bdcf3: mov qword ptr [rbp - 8], 3
1412bdcfb: lea rdx, [rip + 0x196f8ce]
1412bdd02: lea rcx, [rbp - 0x20]
1412bdd06: call 0x1428d9390
1412bdd0b: int3 
1412bdd0c: nop dword ptr [rax]
1412bdd10: mov qword ptr [rsp + 0x10], rdx
1412bdd15: push rbp
1412bdd16: push rsi
1412bdd17: sub rsp, 0x28
1412bdd1b: lea rbp, [rdx + 0x80]
1412bdd22: lea rcx, [rbp - 0x48]
1412bdd26: call 0x140be24b0
1412bdd2b: nop 
1412bdd2c: add rsp, 0x28
1412bdd30: pop rsi
1412bdd31: pop rbp
1412bdd32: ret 
1412bdd33: nop word ptr cs:[rax + rax]
1412bdd40: mov qword ptr [rsp + 0x10], rdx
1412bdd45: push rbp
1412bdd46: push rsi
1412bdd47: sub rsp, 0x28
1412bdd4b: lea rbp, [rdx + 0x80]
1412bdd52: lea rcx, [rbp + 0x20]
1412bdd56: call 0x1400c04b0
1412bdd5b: nop 
1412bdd5c: add rsp, 0x28
1412bdd60: pop rsi
1412bdd61: pop rbp
1412bdd62: ret 
