14161cd80: push rbp
14161cd81: push r15
14161cd83: push r14
14161cd85: push r12
14161cd87: push rsi
14161cd88: push rdi
14161cd89: push rbx
14161cd8a: sub rsp, 0x130
14161cd91: lea rbp, [rsp + 0x80]
14161cd99: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161cda4: mov rdi, rdx
14161cda7: mov rsi, rcx
14161cdaa: test r8, r8
14161cdad: je 0x14161cde6
14161cdaf: mov rbx, r9
14161cdb2: mov rax, r9
14161cdb5: and rax, 0x1f
14161cdb9: mov qword ptr [rbp - 0x40], rax
14161cdbd: jne 0x14161cfc8
14161cdc3: mov r15, r8
14161cdc6: mov rax, r8
14161cdc9: and rax, 0xf
14161cdcd: mov qword ptr [rbp - 0x40], rax
14161cdd1: jne 0x14161cfd9
14161cdd7: cmp rbx, 0x40
14161cddb: jae 0x14161d074
14161cde1: xor r12d, r12d
14161cde4: jmp 0x14161ce1d
14161cde6: movzx eax, byte ptr [rip + 0x2017e1c]
14161cded: mov ebx, 0x20
14161cdf2: mov ecx, 0x20
14161cdf7: mov edx, 0x10
14161cdfc: call 0x140613c10
14161ce01: test rax, rax
14161ce04: je 0x14161d0de
14161ce0a: mov r15, rax
14161ce0d: xorps xmm0, xmm0
14161ce10: movaps xmmword ptr [rax], xmm0
14161ce13: mov dword ptr [rax + 0x10], 0xffffffff
14161ce1a: mov r12b, 1
14161ce1d: mov qword ptr [rbp + 0x98], r15
14161ce24: mov byte ptr [rbp + 0xa7], r12b
14161ce2b: lea rcx, [rbp + 0x10]
14161ce2f: mov edx, 0x20
14161ce34: mov r8d, 0x22
14161ce3a: mov r9b, 2
14161ce3d: call 0x140bd4de0
14161ce42: mov qword ptr [rbp + 0x98], r15
14161ce49: mov byte ptr [rbp + 0xa7], r12b
14161ce50: lea r14, [rbp + 0x60]
14161ce54: lea rdx, [rbp + 0x10]
14161ce58: mov r8d, 0x10
14161ce5e: mov rcx, r14
14161ce61: call 0x140bd4f20
14161ce66: mov qword ptr [rbp + 0x98], r15
14161ce6d: mov byte ptr [rbp + 0xa7], r12b
14161ce74: mov qword ptr [rsp + 0x30], rbx
14161ce79: mov qword ptr [rsp + 0x28], r15
14161ce7e: mov qword ptr [rsp + 0x20], r14
14161ce83: lea r8, [rip + 0x1662e56]
14161ce8a: lea rcx, [rbp - 0x40]
14161ce8e: mov r9d, 0x13
14161ce94: mov rdx, rdi
14161ce97: call 0x1416a2dc0
14161ce9c: cmp dword ptr [rbp - 0x40], 0xb
14161cea0: jne 0x14161d003
14161cea6: mov rax, qword ptr [rbp - 0x18]
14161ceaa: mov qword ptr [rbp + 0x80], rax
14161ceb1: movups xmm0, xmmword ptr [rbp - 0x38]
14161ceb5: movups xmm1, xmmword ptr [rbp - 0x28]
14161ceb9: movaps xmmword ptr [rbp + 0x70], xmm1
14161cebd: movaps xmmword ptr [rbp + 0x60], xmm0
14161cec1: test r12b, r12b
14161cec4: je 0x14161ced9
14161cec6: mov edx, 0x20
14161cecb: mov r8d, 0x10
14161ced1: mov rcx, r15
14161ced4: call 0x140613c20
14161ced9: lea rcx, [rbp + 0x60]
14161cedd: mov rdx, rdi
14161cee0: call 0x140bd4cd0
14161cee5: test rax, rax
14161cee8: je 0x14161d063
14161ceee: cmp rdx, qword ptr [rbp + 0x78]
14161cef2: jb 0x14161cf4d
14161cef4: mov rcx, rdx
14161cef7: and rcx, 0x1f
14161cefb: mov qword ptr [rbp - 0x40], rcx
14161ceff: jne 0x14161cf6a
14161cf01: mov rcx, rax
14161cf04: and rcx, 0xf
14161cf08: mov qword ptr [rbp - 0x40], rcx
14161cf0c: jne 0x14161cf99
14161cf12: mov rcx, qword ptr [rbp + 0x80]
14161cf19: mov qword ptr [rsi + 0x30], rcx
14161cf1d: movaps xmm0, xmmword ptr [rbp + 0x60]
14161cf21: movaps xmm1, xmmword ptr [rbp + 0x70]
14161cf25: movups xmmword ptr [rsi + 0x20], xmm1
14161cf29: movups xmmword ptr [rsi + 0x10], xmm0
14161cf2d: shr rdx, 5
14161cf31: mov qword ptr [rsi], rax
14161cf34: mov qword ptr [rsi + 8], rdx
14161cf38: mov rax, rsi
14161cf3b: add rsp, 0x130
14161cf42: pop rbx
14161cf43: pop rdi
14161cf44: pop rsi
14161cf45: pop r12
14161cf47: pop r14
14161cf49: pop r15
14161cf4b: pop rbp
14161cf4c: ret 
14161cf4d: lea rcx, [rip + 0x166329c]
14161cf54: lea r8, [rip + 0x16632cd]
14161cf5b: mov edx, 0x37
14161cf60: call 0x1428d9430
14161cf65: jmp 0x14161d0fb
14161cf6a: mov qword ptr [rbp + 0x10], 0
14161cf72: lea rax, [rip + 0x16632cf]
14161cf79: mov qword ptr [rsp + 0x20], rax
14161cf7e: lea r8, [rip + 0x16632bb]
14161cf85: lea rdx, [rbp - 0x40]
14161cf89: lea r9, [rbp + 0x10]
14161cf8d: xor ecx, ecx
14161cf8f: call 0x1428d9596
14161cf94: jmp 0x14161d0fb
14161cf99: mov qword ptr [rbp + 0x10], 0
14161cfa1: lea rax, [rip + 0x16632b8]
14161cfa8: mov qword ptr [rsp + 0x20], rax
14161cfad: lea r8, [rip + 0x166328c]
14161cfb4: lea rdx, [rbp - 0x40]
14161cfb8: lea r9, [rbp + 0x10]
14161cfbc: xor ecx, ecx
14161cfbe: call 0x1428d9596
14161cfc3: jmp 0x14161d0fb
14161cfc8: mov qword ptr [rbp + 0x10], 0
14161cfd0: lea rax, [rip + 0x16acf71]
14161cfd7: jmp 0x14161cfe8
14161cfd9: mov qword ptr [rbp + 0x10], 0
14161cfe1: lea rax, [rip + 0x16acf78]
14161cfe8: mov qword ptr [rsp + 0x20], rax
14161cfed: lea r8, [rip + 0x16acf4c]
14161cff4: lea rdx, [rbp - 0x40]
14161cff8: lea r9, [rbp + 0x10]
14161cffc: xor ecx, ecx
14161cffe: call 0x1428d9596
14161d003: mov dword ptr [rbp + 0x94], r12d
14161d00a: mov qword ptr [rbp - 0x48], r15
14161d00e: movups xmm0, xmmword ptr [rbp]
14161d012: movaps xmmword ptr [rbp + 0x50], xmm0
14161d016: movups xmm0, xmmword ptr [rbp - 0x40]
14161d01a: movups xmm1, xmmword ptr [rbp - 0x30]
14161d01e: movups xmm2, xmmword ptr [rbp - 0x20]
14161d022: movups xmm3, xmmword ptr [rbp - 0x10]
14161d026: movaps xmmword ptr [rbp + 0x40], xmm3
14161d02a: movaps xmmword ptr [rbp + 0x30], xmm2
14161d02e: movaps xmmword ptr [rbp + 0x20], xmm1
14161d032: movaps xmmword ptr [rbp + 0x10], xmm0
14161d036: lea rax, [rip + 0x166316b]
14161d03d: mov qword ptr [rsp + 0x20], rax
14161d042: lea rcx, [rip + 0x1661a9f]
14161d049: lea r9, [rip + 0x1661a78]
14161d050: lea r8, [rbp + 0x10]
14161d054: mov edx, 0x2b
14161d059: call 0x1428d9760
14161d05e: jmp 0x14161d0fb
14161d063: lea rcx, [rip + 0x166316e]
14161d06a: call 0x1428d9310
14161d06f: jmp 0x14161d0fb
14161d074: lea rax, [rip + 0x1480ee0]
14161d07b: mov qword ptr [rbp - 0x40], rax
14161d07f: mov qword ptr [rbp - 0x38], 0x6c
14161d087: lea rax, [rbp - 0x40]
14161d08b: mov qword ptr [rbp + 0x60], rax
14161d08f: lea rax, [rip - 0x1513206]
14161d096: mov qword ptr [rbp + 0x68], rax
14161d09a: lea rax, [rip + 0x1662727]
14161d0a1: mov qword ptr [rbp + 0x10], rax
14161d0a5: mov qword ptr [rbp + 0x18], 1
14161d0ad: mov qword ptr [rbp + 0x30], 0
14161d0b5: lea rax, [rbp + 0x60]
14161d0b9: mov qword ptr [rbp + 0x20], rax
14161d0bd: mov qword ptr [rbp + 0x28], 1
14161d0c5: mov byte ptr [rbp + 0xa7], 0
14161d0cc: lea rdx, [rip + 0x1662705]
14161d0d3: lea rcx, [rbp + 0x10]
14161d0d7: call 0x1428d9390
14161d0dc: jmp 0x14161d0fb
14161d0de: mov byte ptr [rbp + 0xa7], 0
14161d0e5: lea r8, [rip + 0x16619c4]
14161d0ec: mov ecx, 0x10
14161d0f1: mov edx, 0x20
14161d0f6: call 0x1428d8fe3
14161d0fb: ud2 
14161d0fd: nop dword ptr [rax]
14161d100: mov qword ptr [rsp + 0x10], rdx
14161d105: push rbp
14161d106: push r15
14161d108: push r14
14161d10a: push r12
14161d10c: push rsi
14161d10d: push rdi
14161d10e: push rbx
14161d10f: sub rsp, 0x40
14161d113: lea rbp, [rdx + 0x80]
14161d11a: mov rsi, qword ptr [rbp - 0x48]
14161d11e: mov qword ptr [rbp + 0x98], rsi
14161d125: mov edi, dword ptr [rbp + 0x94]
14161d12b: mov byte ptr [rbp + 0xa7], dil
14161d132: lea rcx, [rbp + 0x10]
14161d136: call 0x1407cea70
14161d13b: mov qword ptr [rbp + 0x98], rsi
14161d142: mov byte ptr [rbp + 0xa7], dil
14161d149: add rsp, 0x40
14161d14d: pop rbx
14161d14e: pop rdi
14161d14f: pop rsi
14161d150: pop r12
14161d152: pop r14
14161d154: pop r15
14161d156: pop rbp
14161d157: ret 
14161d158: nop dword ptr [rax + rax]
14161d160: mov qword ptr [rsp + 0x10], rdx
14161d165: push rbp
14161d166: push r15
14161d168: push r14
14161d16a: push r12
14161d16c: push rsi
14161d16d: push rdi
14161d16e: push rbx
14161d16f: sub rsp, 0x40
14161d173: lea rbp, [rdx + 0x80]
14161d17a: test byte ptr [rbp + 0xa7], 1
14161d181: je 0x14161d19a
14161d183: mov rcx, qword ptr [rbp + 0x98]
14161d18a: mov edx, 0x20
14161d18f: mov r8d, 0x10
14161d195: call 0x140613c20
14161d19a: nop 
14161d19b: add rsp, 0x40
14161d19f: pop rbx
14161d1a0: pop rdi
14161d1a1: pop rsi
14161d1a2: pop r12
14161d1a4: pop r14
14161d1a6: pop r15
14161d1a8: pop rbp
14161d1a9: ret 
14161d1aa: nop word ptr [rax + rax]
14161d1b0: mov qword ptr [rsp + 0x10], rdx
14161d1b5: push rbp
14161d1b6: push r15
14161d1b8: push r14
14161d1ba: push r12
14161d1bc: push rsi
14161d1bd: push rdi
14161d1be: push rbx
14161d1bf: sub rsp, 0x40
14161d1c3: lea rbp, [rdx + 0x80]
14161d1ca: lea rcx, [rbp + 0x60]
14161d1ce: call 0x140be24b0
14161d1d3: nop 
14161d1d4: add rsp, 0x40
14161d1d8: pop rbx
14161d1d9: pop rdi
14161d1da: pop rsi
14161d1db: pop r12
14161d1dd: pop r14
14161d1df: pop r15
14161d1e1: pop rbp
14161d1e2: ret 
