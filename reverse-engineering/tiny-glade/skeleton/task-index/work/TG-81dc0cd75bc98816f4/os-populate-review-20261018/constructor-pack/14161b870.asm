14161b870: push rbp
14161b871: push r15
14161b873: push r14
14161b875: push r12
14161b877: push rsi
14161b878: push rdi
14161b879: push rbx
14161b87a: sub rsp, 0x130
14161b881: lea rbp, [rsp + 0x80]
14161b889: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161b894: mov rdi, rdx
14161b897: mov rsi, rcx
14161b89a: test r8, r8
14161b89d: je 0x14161b8d9
14161b89f: mov rbx, r9
14161b8a2: mov rax, r9
14161b8a5: and rax, 3
14161b8a9: mov qword ptr [rbp - 0x40], rax
14161b8ad: jne 0x14161bafe
14161b8b3: mov r15, r8
14161b8b6: mov rax, r8
14161b8b9: and rax, 3
14161b8bd: mov qword ptr [rbp - 0x40], rax
14161b8c1: jne 0x14161bb0f
14161b8c7: cmp rbx, 0x80004
14161b8ce: jae 0x14161bbaa
14161b8d4: xor r12d, r12d
14161b8d7: jmp 0x14161b953
14161b8d9: movzx eax, byte ptr [rip + 0x2019329]
14161b8e0: mov ecx, 0x80000
14161b8e5: mov edx, 4
14161b8ea: call 0x140613c10
14161b8ef: test rax, rax
14161b8f2: je 0x14161bc14
14161b8f8: mov r15, rax
14161b8fb: mov eax, 0x1c
14161b900: xorps xmm0, xmm0
14161b903: nop word ptr cs:[rax + rax]
14161b910: movups xmmword ptr [r15 + rax*4 - 0x70], xmm0
14161b916: movups xmmword ptr [r15 + rax*4 - 0x60], xmm0
14161b91c: movups xmmword ptr [r15 + rax*4 - 0x50], xmm0
14161b922: movups xmmword ptr [r15 + rax*4 - 0x40], xmm0
14161b928: movups xmmword ptr [r15 + rax*4 - 0x30], xmm0
14161b92e: movups xmmword ptr [r15 + rax*4 - 0x20], xmm0
14161b934: movups xmmword ptr [r15 + rax*4 - 0x10], xmm0
14161b93a: movups xmmword ptr [r15 + rax*4], xmm0
14161b93f: add rax, 0x20
14161b943: cmp rax, 0x2001c
14161b949: jne 0x14161b910
14161b94b: mov r12b, 1
14161b94e: mov ebx, 0x80000
14161b953: mov qword ptr [rbp + 0x98], r15
14161b95a: mov byte ptr [rbp + 0xa7], r12b
14161b961: lea rcx, [rbp + 0x10]
14161b965: mov edx, 0x80000
14161b96a: mov r8d, 0x22
14161b970: mov r9b, 4
14161b973: call 0x140bd4de0
14161b978: mov qword ptr [rbp + 0x98], r15
14161b97f: mov byte ptr [rbp + 0xa7], r12b
14161b986: lea r14, [rbp + 0x60]
14161b98a: lea rdx, [rbp + 0x10]
14161b98e: mov r8d, 4
14161b994: mov rcx, r14
14161b997: call 0x140bd4f20
14161b99c: mov qword ptr [rbp + 0x98], r15
14161b9a3: mov byte ptr [rbp + 0xa7], r12b
14161b9aa: mov qword ptr [rsp + 0x30], rbx
14161b9af: mov qword ptr [rsp + 0x28], r15
14161b9b4: mov qword ptr [rsp + 0x20], r14
14161b9b9: lea r8, [rip + 0x1664313]
14161b9c0: lea rcx, [rbp - 0x40]
14161b9c4: mov r9d, 0xd
14161b9ca: mov rdx, rdi
14161b9cd: call 0x1416a2dc0
14161b9d2: cmp dword ptr [rbp - 0x40], 0xb
14161b9d6: jne 0x14161bb39
14161b9dc: mov rax, qword ptr [rbp - 0x18]
14161b9e0: mov qword ptr [rbp + 0x80], rax
14161b9e7: movups xmm0, xmmword ptr [rbp - 0x38]
14161b9eb: movups xmm1, xmmword ptr [rbp - 0x28]
14161b9ef: movaps xmmword ptr [rbp + 0x70], xmm1
14161b9f3: movaps xmmword ptr [rbp + 0x60], xmm0
14161b9f7: test r12b, r12b
14161b9fa: je 0x14161ba0f
14161b9fc: mov edx, 0x80000
14161ba01: mov r8d, 4
14161ba07: mov rcx, r15
14161ba0a: call 0x140613c20
14161ba0f: lea rcx, [rbp + 0x60]
14161ba13: mov rdx, rdi
14161ba16: call 0x140bd4cd0
14161ba1b: test rax, rax
14161ba1e: je 0x14161bb99
14161ba24: cmp rdx, qword ptr [rbp + 0x78]
14161ba28: jb 0x14161ba83
14161ba2a: mov rcx, rdx
14161ba2d: and rcx, 3
14161ba31: mov qword ptr [rbp - 0x40], rcx
14161ba35: jne 0x14161baa0
14161ba37: mov rcx, rax
14161ba3a: and rcx, 3
14161ba3e: mov qword ptr [rbp - 0x40], rcx
14161ba42: jne 0x14161bacf
14161ba48: mov rcx, qword ptr [rbp + 0x80]
14161ba4f: mov qword ptr [rsi + 0x30], rcx
14161ba53: movaps xmm0, xmmword ptr [rbp + 0x60]
14161ba57: movaps xmm1, xmmword ptr [rbp + 0x70]
14161ba5b: movups xmmword ptr [rsi + 0x20], xmm1
14161ba5f: movups xmmword ptr [rsi + 0x10], xmm0
14161ba63: shr rdx, 2
14161ba67: mov qword ptr [rsi], rax
14161ba6a: mov qword ptr [rsi + 8], rdx
14161ba6e: mov rax, rsi
14161ba71: add rsp, 0x130
14161ba78: pop rbx
14161ba79: pop rdi
14161ba7a: pop rsi
14161ba7b: pop r12
14161ba7d: pop r14
14161ba7f: pop r15
14161ba81: pop rbp
14161ba82: ret 
14161ba83: lea rcx, [rip + 0x1664766]
14161ba8a: lea r8, [rip + 0x1664797]
14161ba91: mov edx, 0x37
14161ba96: call 0x1428d9430
14161ba9b: jmp 0x14161bc31
14161baa0: mov qword ptr [rbp + 0x10], 0
14161baa8: lea rax, [rip + 0x1664799]
14161baaf: mov qword ptr [rsp + 0x20], rax
14161bab4: lea r8, [rip + 0x1664785]
14161babb: lea rdx, [rbp - 0x40]
14161babf: lea r9, [rbp + 0x10]
14161bac3: xor ecx, ecx
14161bac5: call 0x1428d9596
14161baca: jmp 0x14161bc31
14161bacf: mov qword ptr [rbp + 0x10], 0
14161bad7: lea rax, [rip + 0x1664782]
14161bade: mov qword ptr [rsp + 0x20], rax
14161bae3: lea r8, [rip + 0x1664756]
14161baea: lea rdx, [rbp - 0x40]
14161baee: lea r9, [rbp + 0x10]
14161baf2: xor ecx, ecx
14161baf4: call 0x1428d9596
14161baf9: jmp 0x14161bc31
14161bafe: mov qword ptr [rbp + 0x10], 0
14161bb06: lea rax, [rip + 0x16ae43b]
14161bb0d: jmp 0x14161bb1e
14161bb0f: mov qword ptr [rbp + 0x10], 0
14161bb17: lea rax, [rip + 0x16ae442]
14161bb1e: mov qword ptr [rsp + 0x20], rax
14161bb23: lea r8, [rip + 0x16ae416]
14161bb2a: lea rdx, [rbp - 0x40]
14161bb2e: lea r9, [rbp + 0x10]
14161bb32: xor ecx, ecx
14161bb34: call 0x1428d9596
14161bb39: mov dword ptr [rbp + 0x94], r12d
14161bb40: mov qword ptr [rbp - 0x48], r15
14161bb44: movups xmm0, xmmword ptr [rbp]
14161bb48: movaps xmmword ptr [rbp + 0x50], xmm0
14161bb4c: movups xmm0, xmmword ptr [rbp - 0x40]
14161bb50: movups xmm1, xmmword ptr [rbp - 0x30]
14161bb54: movups xmm2, xmmword ptr [rbp - 0x20]
14161bb58: movups xmm3, xmmword ptr [rbp - 0x10]
14161bb5c: movaps xmmword ptr [rbp + 0x40], xmm3
14161bb60: movaps xmmword ptr [rbp + 0x30], xmm2
14161bb64: movaps xmmword ptr [rbp + 0x20], xmm1
14161bb68: movaps xmmword ptr [rbp + 0x10], xmm0
14161bb6c: lea rax, [rip + 0x1664635]
14161bb73: mov qword ptr [rsp + 0x20], rax
14161bb78: lea rcx, [rip + 0x1662f69]
14161bb7f: lea r9, [rip + 0x1662f42]
14161bb86: lea r8, [rbp + 0x10]
14161bb8a: mov edx, 0x2b
14161bb8f: call 0x1428d9760
14161bb94: jmp 0x14161bc31
14161bb99: lea rcx, [rip + 0x1664638]
14161bba0: call 0x1428d9310
14161bba5: jmp 0x14161bc31
14161bbaa: lea rax, [rip + 0x1482718]
14161bbb1: mov qword ptr [rbp - 0x40], rax
14161bbb5: mov qword ptr [rbp - 0x38], 0x3a
14161bbbd: lea rax, [rbp - 0x40]
14161bbc1: mov qword ptr [rbp + 0x60], rax
14161bbc5: lea rax, [rip - 0x1511d3c]
14161bbcc: mov qword ptr [rbp + 0x68], rax
14161bbd0: lea rax, [rip + 0x1663bf1]
14161bbd7: mov qword ptr [rbp + 0x10], rax
14161bbdb: mov qword ptr [rbp + 0x18], 1
14161bbe3: mov qword ptr [rbp + 0x30], 0
14161bbeb: lea rax, [rbp + 0x60]
14161bbef: mov qword ptr [rbp + 0x20], rax
14161bbf3: mov qword ptr [rbp + 0x28], 1
14161bbfb: mov byte ptr [rbp + 0xa7], 0
14161bc02: lea rdx, [rip + 0x1663bcf]
14161bc09: lea rcx, [rbp + 0x10]
14161bc0d: call 0x1428d9390
14161bc12: jmp 0x14161bc31
14161bc14: mov byte ptr [rbp + 0xa7], 0
14161bc1b: lea r8, [rip + 0x1662e8e]
14161bc22: mov ecx, 4
14161bc27: mov edx, 0x80000
14161bc2c: call 0x1428d8fe3
14161bc31: ud2 
14161bc33: nop word ptr cs:[rax + rax]
14161bc40: mov qword ptr [rsp + 0x10], rdx
14161bc45: push rbp
14161bc46: push r15
14161bc48: push r14
14161bc4a: push r12
14161bc4c: push rsi
14161bc4d: push rdi
14161bc4e: push rbx
14161bc4f: sub rsp, 0x40
14161bc53: lea rbp, [rdx + 0x80]
14161bc5a: mov rsi, qword ptr [rbp - 0x48]
14161bc5e: mov qword ptr [rbp + 0x98], rsi
14161bc65: mov edi, dword ptr [rbp + 0x94]
14161bc6b: mov byte ptr [rbp + 0xa7], dil
14161bc72: lea rcx, [rbp + 0x10]
14161bc76: call 0x1407cea70
14161bc7b: mov qword ptr [rbp + 0x98], rsi
14161bc82: mov byte ptr [rbp + 0xa7], dil
14161bc89: add rsp, 0x40
14161bc8d: pop rbx
14161bc8e: pop rdi
14161bc8f: pop rsi
14161bc90: pop r12
14161bc92: pop r14
14161bc94: pop r15
14161bc96: pop rbp
14161bc97: ret 
14161bc98: nop dword ptr [rax + rax]
14161bca0: mov qword ptr [rsp + 0x10], rdx
14161bca5: push rbp
14161bca6: push r15
14161bca8: push r14
14161bcaa: push r12
14161bcac: push rsi
14161bcad: push rdi
14161bcae: push rbx
14161bcaf: sub rsp, 0x40
14161bcb3: lea rbp, [rdx + 0x80]
14161bcba: test byte ptr [rbp + 0xa7], 1
14161bcc1: je 0x14161bcda
14161bcc3: mov rcx, qword ptr [rbp + 0x98]
14161bcca: mov edx, 0x80000
14161bccf: mov r8d, 4
14161bcd5: call 0x140613c20
14161bcda: nop 
14161bcdb: add rsp, 0x40
14161bcdf: pop rbx
14161bce0: pop rdi
14161bce1: pop rsi
14161bce2: pop r12
14161bce4: pop r14
14161bce6: pop r15
14161bce8: pop rbp
14161bce9: ret 
14161bcea: nop word ptr [rax + rax]
14161bcf0: mov qword ptr [rsp + 0x10], rdx
14161bcf5: push rbp
14161bcf6: push r15
14161bcf8: push r14
14161bcfa: push r12
14161bcfc: push rsi
14161bcfd: push rdi
14161bcfe: push rbx
14161bcff: sub rsp, 0x40
14161bd03: lea rbp, [rdx + 0x80]
14161bd0a: lea rcx, [rbp + 0x60]
14161bd0e: call 0x140be24b0
14161bd13: nop 
14161bd14: add rsp, 0x40
14161bd18: pop rbx
14161bd19: pop rdi
14161bd1a: pop rsi
14161bd1b: pop r12
14161bd1d: pop r14
14161bd1f: pop r15
14161bd21: pop rbp
14161bd22: ret 
