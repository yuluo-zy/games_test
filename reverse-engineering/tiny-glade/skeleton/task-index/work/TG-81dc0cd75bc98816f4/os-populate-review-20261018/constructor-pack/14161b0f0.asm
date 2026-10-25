14161b0f0: push rbp
14161b0f1: push r15
14161b0f3: push r14
14161b0f5: push r12
14161b0f7: push rsi
14161b0f8: push rdi
14161b0f9: push rbx
14161b0fa: sub rsp, 0x130
14161b101: lea rbp, [rsp + 0x80]
14161b109: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14161b114: mov rdi, rdx
14161b117: mov rsi, rcx
14161b11a: test r8, r8
14161b11d: je 0x14161b15c
14161b11f: mov rbx, r9
14161b122: mov rax, r9
14161b125: and rax, 0x1f
14161b129: mov qword ptr [rbp - 0x40], rax
14161b12d: jne 0x14161b64b
14161b133: mov r15, r8
14161b136: mov rax, r8
14161b139: and rax, 3
14161b13d: mov qword ptr [rbp - 0x40], rax
14161b141: jne 0x14161b65c
14161b147: cmp rbx, 0x320
14161b14e: jae 0x14161b6f7
14161b154: xor r12d, r12d
14161b157: jmp 0x14161b4a0
14161b15c: movzx eax, byte ptr [rip + 0x2019aa6]
14161b163: mov ebx, 0x300
14161b168: mov ecx, 0x300
14161b16d: mov edx, 4
14161b172: call 0x140613c10
14161b177: test rax, rax
14161b17a: je 0x14161b761
14161b180: mov r15, rax
14161b183: xorps xmm0, xmm0
14161b186: movups xmmword ptr [rax], xmm0
14161b189: movsd xmm1, qword ptr [rip + 0x13b219f]
14161b191: movsd qword ptr [rax + 0x10], xmm1
14161b196: mov dword ptr [rax + 0x18], 0xbf800000
14161b19d: movups xmmword ptr [rax + 0x1c], xmm0
14161b1a1: mov dword ptr [rax + 0x2c], 0
14161b1a8: movsd qword ptr [rax + 0x30], xmm1
14161b1ad: mov dword ptr [rax + 0x38], 0xbf800000
14161b1b4: movups xmmword ptr [rax + 0x3c], xmm0
14161b1b8: mov dword ptr [rax + 0x4c], 0
14161b1bf: movsd qword ptr [rax + 0x50], xmm1
14161b1c4: mov dword ptr [rax + 0x58], 0xbf800000
14161b1cb: movups xmmword ptr [rax + 0x5c], xmm0
14161b1cf: mov dword ptr [rax + 0x6c], 0
14161b1d6: movsd qword ptr [rax + 0x70], xmm1
14161b1db: mov dword ptr [rax + 0x78], 0xbf800000
14161b1e2: movups xmmword ptr [rax + 0x7c], xmm0
14161b1e6: mov dword ptr [rax + 0x8c], 0
14161b1f0: movsd qword ptr [rax + 0x90], xmm1
14161b1f8: mov dword ptr [rax + 0x98], 0xbf800000
14161b202: movups xmmword ptr [rax + 0x9c], xmm0
14161b209: mov dword ptr [rax + 0xac], 0
14161b213: movsd qword ptr [rax + 0xb0], xmm1
14161b21b: mov dword ptr [rax + 0xb8], 0xbf800000
14161b225: movups xmmword ptr [rax + 0xbc], xmm0
14161b22c: mov dword ptr [rax + 0xcc], 0
14161b236: movsd qword ptr [rax + 0xd0], xmm1
14161b23e: mov dword ptr [rax + 0xd8], 0xbf800000
14161b248: movups xmmword ptr [rax + 0xdc], xmm0
14161b24f: mov dword ptr [rax + 0xec], 0
14161b259: movsd qword ptr [rax + 0xf0], xmm1
14161b261: mov dword ptr [rax + 0xf8], 0xbf800000
14161b26b: movups xmmword ptr [rax + 0xfc], xmm0
14161b272: mov dword ptr [rax + 0x10c], 0
14161b27c: movsd qword ptr [rax + 0x110], xmm1
14161b284: mov dword ptr [rax + 0x118], 0xbf800000
14161b28e: movups xmmword ptr [rax + 0x11c], xmm0
14161b295: mov dword ptr [rax + 0x12c], 0
14161b29f: movsd qword ptr [rax + 0x130], xmm1
14161b2a7: mov dword ptr [rax + 0x138], 0xbf800000
14161b2b1: movups xmmword ptr [rax + 0x13c], xmm0
14161b2b8: mov dword ptr [rax + 0x14c], 0
14161b2c2: movsd qword ptr [rax + 0x150], xmm1
14161b2ca: mov dword ptr [rax + 0x158], 0xbf800000
14161b2d4: movups xmmword ptr [rax + 0x15c], xmm0
14161b2db: mov dword ptr [rax + 0x16c], 0
14161b2e5: movsd qword ptr [rax + 0x170], xmm1
14161b2ed: mov dword ptr [rax + 0x178], 0xbf800000
14161b2f7: movups xmmword ptr [rax + 0x17c], xmm0
14161b2fe: mov dword ptr [rax + 0x18c], 0
14161b308: movsd qword ptr [rax + 0x190], xmm1
14161b310: mov dword ptr [rax + 0x198], 0xbf800000
14161b31a: movups xmmword ptr [rax + 0x19c], xmm0
14161b321: mov dword ptr [rax + 0x1ac], 0
14161b32b: movsd qword ptr [rax + 0x1b0], xmm1
14161b333: mov dword ptr [rax + 0x1b8], 0xbf800000
14161b33d: movups xmmword ptr [rax + 0x1bc], xmm0
14161b344: mov dword ptr [rax + 0x1cc], 0
14161b34e: movsd qword ptr [rax + 0x1d0], xmm1
14161b356: mov dword ptr [rax + 0x1d8], 0xbf800000
14161b360: movups xmmword ptr [rax + 0x1dc], xmm0
14161b367: mov dword ptr [rax + 0x1ec], 0
14161b371: movsd qword ptr [rax + 0x1f0], xmm1
14161b379: mov dword ptr [rax + 0x1f8], 0xbf800000
14161b383: movups xmmword ptr [rax + 0x1fc], xmm0
14161b38a: mov dword ptr [rax + 0x20c], 0
14161b394: movsd qword ptr [rax + 0x210], xmm1
14161b39c: mov dword ptr [rax + 0x218], 0xbf800000
14161b3a6: movups xmmword ptr [rax + 0x21c], xmm0
14161b3ad: mov dword ptr [rax + 0x22c], 0
14161b3b7: movsd qword ptr [rax + 0x230], xmm1
14161b3bf: mov dword ptr [rax + 0x238], 0xbf800000
14161b3c9: movups xmmword ptr [rax + 0x23c], xmm0
14161b3d0: mov dword ptr [rax + 0x24c], 0
14161b3da: movsd qword ptr [rax + 0x250], xmm1
14161b3e2: mov dword ptr [rax + 0x258], 0xbf800000
14161b3ec: movups xmmword ptr [rax + 0x25c], xmm0
14161b3f3: mov dword ptr [rax + 0x26c], 0
14161b3fd: movsd qword ptr [rax + 0x270], xmm1
14161b405: mov dword ptr [rax + 0x278], 0xbf800000
14161b40f: movups xmmword ptr [rax + 0x27c], xmm0
14161b416: mov dword ptr [rax + 0x28c], 0
14161b420: movsd qword ptr [rax + 0x290], xmm1
14161b428: mov dword ptr [rax + 0x298], 0xbf800000
14161b432: movups xmmword ptr [rax + 0x29c], xmm0
14161b439: mov dword ptr [rax + 0x2ac], 0
14161b443: movsd qword ptr [rax + 0x2b0], xmm1
14161b44b: mov dword ptr [rax + 0x2b8], 0xbf800000
14161b455: movups xmmword ptr [rax + 0x2bc], xmm0
14161b45c: mov dword ptr [rax + 0x2cc], 0
14161b466: movsd qword ptr [rax + 0x2d0], xmm1
14161b46e: mov dword ptr [rax + 0x2d8], 0xbf800000
14161b478: movups xmmword ptr [rax + 0x2dc], xmm0
14161b47f: mov dword ptr [rax + 0x2ec], 0
14161b489: movsd qword ptr [rax + 0x2f0], xmm1
14161b491: mov eax, 0xbf800000
14161b496: mov qword ptr [r15 + 0x2f8], rax
14161b49d: mov r12b, 1
14161b4a0: mov qword ptr [rbp + 0x98], r15
14161b4a7: mov byte ptr [rbp + 0xa7], r12b
14161b4ae: lea rcx, [rbp + 0x10]
14161b4b2: mov edx, 0x300
14161b4b7: mov r8d, 0x22
14161b4bd: mov r9b, 2
14161b4c0: call 0x140bd4de0
14161b4c5: mov qword ptr [rbp + 0x98], r15
14161b4cc: mov byte ptr [rbp + 0xa7], r12b
14161b4d3: lea r14, [rbp + 0x60]
14161b4d7: lea rdx, [rbp + 0x10]
14161b4db: mov r8d, 4
14161b4e1: mov rcx, r14
14161b4e4: call 0x140bd4f20
14161b4e9: mov qword ptr [rbp + 0x98], r15
14161b4f0: mov byte ptr [rbp + 0xa7], r12b
14161b4f7: mov qword ptr [rsp + 0x30], rbx
14161b4fc: mov qword ptr [rsp + 0x28], r15
14161b501: mov qword ptr [rsp + 0x20], r14
14161b506: lea r8, [rip + 0x16645b3]
14161b50d: lea rcx, [rbp - 0x40]
14161b511: mov r9d, 0x10
14161b517: mov rdx, rdi
14161b51a: call 0x1416a2dc0
14161b51f: cmp dword ptr [rbp - 0x40], 0xb
14161b523: jne 0x14161b686
14161b529: mov rax, qword ptr [rbp - 0x18]
14161b52d: mov qword ptr [rbp + 0x80], rax
14161b534: movups xmm0, xmmword ptr [rbp - 0x38]
14161b538: movups xmm1, xmmword ptr [rbp - 0x28]
14161b53c: movaps xmmword ptr [rbp + 0x70], xmm1
14161b540: movaps xmmword ptr [rbp + 0x60], xmm0
14161b544: test r12b, r12b
14161b547: je 0x14161b55c
14161b549: mov edx, 0x300
14161b54e: mov r8d, 4
14161b554: mov rcx, r15
14161b557: call 0x140613c20
14161b55c: lea rcx, [rbp + 0x60]
14161b560: mov rdx, rdi
14161b563: call 0x140bd4cd0
14161b568: test rax, rax
14161b56b: je 0x14161b6e6
14161b571: cmp rdx, qword ptr [rbp + 0x78]
14161b575: jb 0x14161b5d0
14161b577: mov rcx, rdx
14161b57a: and rcx, 0x1f
14161b57e: mov qword ptr [rbp - 0x40], rcx
14161b582: jne 0x14161b5ed
14161b584: mov rcx, rax
14161b587: and rcx, 3
14161b58b: mov qword ptr [rbp - 0x40], rcx
14161b58f: jne 0x14161b61c
14161b595: mov rcx, qword ptr [rbp + 0x80]
14161b59c: mov qword ptr [rsi + 0x30], rcx
14161b5a0: movaps xmm0, xmmword ptr [rbp + 0x60]
14161b5a4: movaps xmm1, xmmword ptr [rbp + 0x70]
14161b5a8: movups xmmword ptr [rsi + 0x20], xmm1
14161b5ac: movups xmmword ptr [rsi + 0x10], xmm0
14161b5b0: shr rdx, 5
14161b5b4: mov qword ptr [rsi], rax
14161b5b7: mov qword ptr [rsi + 8], rdx
14161b5bb: mov rax, rsi
14161b5be: add rsp, 0x130
14161b5c5: pop rbx
14161b5c6: pop rdi
14161b5c7: pop rsi
14161b5c8: pop r12
14161b5ca: pop r14
14161b5cc: pop r15
14161b5ce: pop rbp
14161b5cf: ret 
14161b5d0: lea rcx, [rip + 0x1664c19]
14161b5d7: lea r8, [rip + 0x1664c4a]
14161b5de: mov edx, 0x37
14161b5e3: call 0x1428d9430
14161b5e8: jmp 0x14161b77e
14161b5ed: mov qword ptr [rbp + 0x10], 0
14161b5f5: lea rax, [rip + 0x1664c4c]
14161b5fc: mov qword ptr [rsp + 0x20], rax
14161b601: lea r8, [rip + 0x1664c38]
14161b608: lea rdx, [rbp - 0x40]
14161b60c: lea r9, [rbp + 0x10]
14161b610: xor ecx, ecx
14161b612: call 0x1428d9596
14161b617: jmp 0x14161b77e
14161b61c: mov qword ptr [rbp + 0x10], 0
14161b624: lea rax, [rip + 0x1664c35]
14161b62b: mov qword ptr [rsp + 0x20], rax
14161b630: lea r8, [rip + 0x1664c09]
14161b637: lea rdx, [rbp - 0x40]
14161b63b: lea r9, [rbp + 0x10]
14161b63f: xor ecx, ecx
14161b641: call 0x1428d9596
14161b646: jmp 0x14161b77e
14161b64b: mov qword ptr [rbp + 0x10], 0
14161b653: lea rax, [rip + 0x16ae8ee]
14161b65a: jmp 0x14161b66b
14161b65c: mov qword ptr [rbp + 0x10], 0
14161b664: lea rax, [rip + 0x16ae8f5]
14161b66b: mov qword ptr [rsp + 0x20], rax
14161b670: lea r8, [rip + 0x16ae8c9]
14161b677: lea rdx, [rbp - 0x40]
14161b67b: lea r9, [rbp + 0x10]
14161b67f: xor ecx, ecx
14161b681: call 0x1428d9596
14161b686: mov dword ptr [rbp + 0x94], r12d
14161b68d: mov qword ptr [rbp - 0x48], r15
14161b691: movups xmm0, xmmword ptr [rbp]
14161b695: movaps xmmword ptr [rbp + 0x50], xmm0
14161b699: movups xmm0, xmmword ptr [rbp - 0x40]
14161b69d: movups xmm1, xmmword ptr [rbp - 0x30]
14161b6a1: movups xmm2, xmmword ptr [rbp - 0x20]
14161b6a5: movups xmm3, xmmword ptr [rbp - 0x10]
14161b6a9: movaps xmmword ptr [rbp + 0x40], xmm3
14161b6ad: movaps xmmword ptr [rbp + 0x30], xmm2
14161b6b1: movaps xmmword ptr [rbp + 0x20], xmm1
14161b6b5: movaps xmmword ptr [rbp + 0x10], xmm0
14161b6b9: lea rax, [rip + 0x1664ae8]
14161b6c0: mov qword ptr [rsp + 0x20], rax
14161b6c5: lea rcx, [rip + 0x166341c]
14161b6cc: lea r9, [rip + 0x16633f5]
14161b6d3: lea r8, [rbp + 0x10]
14161b6d7: mov edx, 0x2b
14161b6dc: call 0x1428d9760
14161b6e1: jmp 0x14161b77e
14161b6e6: lea rcx, [rip + 0x1664aeb]
14161b6ed: call 0x1428d9310
14161b6f2: jmp 0x14161b77e
14161b6f7: lea rax, [rip + 0x148268e]
14161b6fe: mov qword ptr [rbp - 0x40], rax
14161b702: mov qword ptr [rbp - 0x38], 0x46
14161b70a: lea rax, [rbp - 0x40]
14161b70e: mov qword ptr [rbp + 0x60], rax
14161b712: lea rax, [rip - 0x1511889]
14161b719: mov qword ptr [rbp + 0x68], rax
14161b71d: lea rax, [rip + 0x16640a4]
14161b724: mov qword ptr [rbp + 0x10], rax
14161b728: mov qword ptr [rbp + 0x18], 1
14161b730: mov qword ptr [rbp + 0x30], 0
14161b738: lea rax, [rbp + 0x60]
14161b73c: mov qword ptr [rbp + 0x20], rax
14161b740: mov qword ptr [rbp + 0x28], 1
14161b748: mov byte ptr [rbp + 0xa7], 0
14161b74f: lea rdx, [rip + 0x1664082]
14161b756: lea rcx, [rbp + 0x10]
14161b75a: call 0x1428d9390
14161b75f: jmp 0x14161b77e
14161b761: mov byte ptr [rbp + 0xa7], 0
14161b768: lea r8, [rip + 0x1663341]
14161b76f: mov ecx, 4
14161b774: mov edx, 0x300
14161b779: call 0x1428d8fe3
14161b77e: ud2 
14161b780: mov qword ptr [rsp + 0x10], rdx
14161b785: push rbp
14161b786: push r15
14161b788: push r14
14161b78a: push r12
14161b78c: push rsi
14161b78d: push rdi
14161b78e: push rbx
14161b78f: sub rsp, 0x40
14161b793: lea rbp, [rdx + 0x80]
14161b79a: mov rsi, qword ptr [rbp - 0x48]
14161b79e: mov qword ptr [rbp + 0x98], rsi
14161b7a5: mov edi, dword ptr [rbp + 0x94]
14161b7ab: mov byte ptr [rbp + 0xa7], dil
14161b7b2: lea rcx, [rbp + 0x10]
14161b7b6: call 0x1407cea70
14161b7bb: mov qword ptr [rbp + 0x98], rsi
14161b7c2: mov byte ptr [rbp + 0xa7], dil
14161b7c9: add rsp, 0x40
14161b7cd: pop rbx
14161b7ce: pop rdi
14161b7cf: pop rsi
14161b7d0: pop r12
14161b7d2: pop r14
14161b7d4: pop r15
14161b7d6: pop rbp
14161b7d7: ret 
14161b7d8: nop dword ptr [rax + rax]
14161b7e0: mov qword ptr [rsp + 0x10], rdx
14161b7e5: push rbp
14161b7e6: push r15
14161b7e8: push r14
14161b7ea: push r12
14161b7ec: push rsi
14161b7ed: push rdi
14161b7ee: push rbx
14161b7ef: sub rsp, 0x40
14161b7f3: lea rbp, [rdx + 0x80]
14161b7fa: test byte ptr [rbp + 0xa7], 1
14161b801: je 0x14161b81a
14161b803: mov rcx, qword ptr [rbp + 0x98]
14161b80a: mov edx, 0x300
14161b80f: mov r8d, 4
14161b815: call 0x140613c20
14161b81a: nop 
14161b81b: add rsp, 0x40
14161b81f: pop rbx
14161b820: pop rdi
14161b821: pop rsi
14161b822: pop r12
14161b824: pop r14
14161b826: pop r15
14161b828: pop rbp
14161b829: ret 
14161b82a: nop word ptr [rax + rax]
14161b830: mov qword ptr [rsp + 0x10], rdx
14161b835: push rbp
14161b836: push r15
14161b838: push r14
14161b83a: push r12
14161b83c: push rsi
14161b83d: push rdi
14161b83e: push rbx
14161b83f: sub rsp, 0x40
14161b843: lea rbp, [rdx + 0x80]
14161b84a: lea rcx, [rbp + 0x60]
14161b84e: call 0x140be24b0
14161b853: nop 
14161b854: add rsp, 0x40
14161b858: pop rbx
14161b859: pop rdi
14161b85a: pop rsi
14161b85b: pop r12
14161b85d: pop r14
14161b85f: pop r15
14161b861: pop rbp
14161b862: ret 
