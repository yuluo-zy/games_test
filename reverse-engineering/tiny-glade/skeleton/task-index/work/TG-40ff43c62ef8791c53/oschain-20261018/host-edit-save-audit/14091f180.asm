14091f180: push rbp
14091f181: push r15
14091f183: push r14
14091f185: push r13
14091f187: push r12
14091f189: push rsi
14091f18a: push rdi
14091f18b: push rbx
14091f18c: sub rsp, 0x3e8
14091f193: lea rbp, [rsp + 0x80]
14091f19b: movaps xmmword ptr [rbp + 0x350], xmm8
14091f1a3: movaps xmmword ptr [rbp + 0x340], xmm7
14091f1aa: movaps xmmword ptr [rbp + 0x330], xmm6
14091f1b1: mov qword ptr [rbp + 0x328], 0xfffffffffffffffe
14091f1bc: mov qword ptr [rbp + 0x48], rdx
14091f1c0: mov rax, qword ptr [rbp + 0x3e0]
14091f1c7: mov rdx, qword ptr [rbp + 0x3e8]
14091f1ce: mov r10, qword ptr [rbp + 0x3d8]
14091f1d5: mov r11d, dword ptr [rcx + 0x1c]
14091f1d9: mov dword ptr [rbp + 0x318], r11d
14091f1e0: mov rsi, qword ptr [rcx]
14091f1e3: mov rcx, qword ptr [rcx + 0x10]
14091f1e7: mov qword ptr [rbp + 0x258], rcx
14091f1ee: mov rcx, qword ptr [r10]
14091f1f1: mov qword ptr [rbp - 0x20], rcx
14091f1f5: mov rcx, qword ptr [r10 + 8]
14091f1f9: mov qword ptr [rbp + 0x230], rcx
14091f200: mov ecx, dword ptr [rdx + 0x1c]
14091f203: mov dword ptr [rbp + 0x148], ecx
14091f209: mov rcx, qword ptr [rdx]
14091f20c: mov qword ptr [rbp + 0x18], rcx
14091f210: mov rcx, qword ptr [rdx + 0x10]
14091f214: mov qword ptr [rbp + 0x20], rcx
14091f218: mov ecx, dword ptr [rax + 0x1c]
14091f21b: mov dword ptr [rbp + 0x14c], ecx
14091f221: mov rcx, qword ptr [rax]
14091f224: mov qword ptr [rbp + 0x28], rcx
14091f228: mov rax, qword ptr [rax + 0x10]
14091f22c: mov qword ptr [rbp + 0x30], rax
14091f230: mov rax, qword ptr [rbp + 0x3d0]
14091f237: mov rax, qword ptr [rax]
14091f23a: mov qword ptr [rbp + 0x60], rax
14091f23e: mov rax, qword ptr [rbp + 0x3f8]
14091f245: mov ecx, dword ptr [rax + 0x1c]
14091f248: mov dword ptr [rbp + 0x23c], ecx
14091f24e: mov rdx, qword ptr [rax]
14091f251: mov rax, qword ptr [rax + 0x10]
14091f255: mov qword ptr [rbp + 0x158], rax
14091f25c: mov rax, qword ptr [rbp + 0x400]
14091f263: mov ecx, dword ptr [rax + 0x1c]
14091f266: mov dword ptr [rbp + 0x13c], ecx
14091f26c: mov rcx, qword ptr [rax]
14091f26f: mov qword ptr [rbp - 0x18], rcx
14091f273: mov rax, qword ptr [rax + 0x10]
14091f277: mov qword ptr [rbp - 0x10], rax
14091f27b: mov rax, qword ptr [r9]
14091f27e: mov qword ptr [rbp + 0x1e8], rax
14091f285: mov rcx, qword ptr [rbp + 0x410]
14091f28c: mov rax, qword ptr [rcx + 8]
14091f290: test rax, rax
14091f293: mov r9, qword ptr [rcx + 0x10]
14091f297: mov qword ptr [rbp - 0x40], rax
14091f29b: mov qword ptr [rbp - 0x38], r9
14091f29f: cmove rax, r9
14091f2a3: mov qword ptr [rbp + 0x248], rax
14091f2aa: mov rax, qword ptr [r8]
14091f2ad: mov qword ptr [rbp + 0x80], rax
14091f2b4: mov rax, qword ptr [rbp + 0x408]
14091f2bb: mov r8, qword ptr [rax + 0x20]
14091f2bf: mov r9, qword ptr [rax + 0x30]
14091f2c3: mov qword ptr [rbp - 8], r9
14091f2c7: mov r9d, dword ptr [rax + 0x3c]
14091f2cb: mov dword ptr [rbp + 0x140], r9d
14091f2d2: mov r9d, dword ptr [rax + 0x1c]
14091f2d6: mov dword ptr [rbp + 0x144], r9d
14091f2dd: mov r9, qword ptr [rax]
14091f2e0: mov rax, qword ptr [rax + 0x10]
14091f2e4: mov qword ptr [rbp], rax
14091f2e8: xorps xmm6, xmm6
14091f2eb: xorps xmm7, xmm7
14091f2ee: lea r14, [rbp + 0x1b8]
14091f2f5: lea rax, [rsi + 0x20]
14091f2f9: mov qword ptr [rbp + 0x78], rax
14091f2fd: mov rax, qword ptr [rcx]
14091f300: mov qword ptr [rbp - 0x30], rax
14091f304: mov qword ptr [rbp + 0x68], r8
14091f308: lea rax, [r8 + 0x20]
14091f30c: mov qword ptr [rbp + 0x38], rax
14091f310: mov rax, qword ptr [rcx + 0x18]
14091f314: mov qword ptr [rbp - 0x28], rax
14091f318: lea rax, [rsi + 0x40]
14091f31c: mov qword ptr [rbp + 0x10], rax
14091f320: mov qword ptr [rbp + 0x70], r9
14091f324: lea rax, [r9 + 0x20]
14091f328: mov qword ptr [rbp + 0x40], rax
14091f32c: mov qword ptr [rbp + 0x150], rdx
14091f333: lea rax, [rdx + 0x10]
14091f337: mov qword ptr [rbp + 8], rax
14091f33b: xor ebx, ebx
14091f33d: movabs rdi, 0x8000000000000000
14091f347: jmp 0x14091f360
14091f349: lea rcx, [rbp + 0x1b0]
14091f350: call 0x14091baf0
14091f355: nop word ptr cs:[rax + rax]
14091f360: mov eax, dword ptr [rbp + 0x318]
14091f366: mov rcx, qword ptr [rbp + 0x258]
14091f36d: mov dword ptr [rcx], eax
14091f36f: mov rax, qword ptr [rsi + 0x18]
14091f373: test rax, rax
14091f376: je 0x140920b5c
14091f37c: mov rcx, qword ptr [rsi + 0x10]
14091f380: lea rdx, [rcx + 1]
14091f384: mov r8, qword ptr [rsi]
14091f387: mov r9, qword ptr [rsi + 8]
14091f38b: cmp rdx, r8
14091f38e: cmovb r8, rbx
14091f392: neg r8
14091f395: lea rdx, [rcx + r8]
14091f399: inc rdx
14091f39c: mov qword ptr [rsi + 0x10], rdx
14091f3a0: dec rax
14091f3a3: mov qword ptr [rsi + 0x18], rax
14091f3a7: imul rcx, rcx, 0x38
14091f3ab: mov rax, qword ptr [r9 + rcx]
14091f3af: mov r8, qword ptr [r9 + rcx + 8]
14091f3b4: mov r12, qword ptr [r9 + rcx + 0x10]
14091f3b9: mov r15, qword ptr [r9 + rcx + 0x18]
14091f3be: mov r13, qword ptr [r9 + rcx + 0x20]
14091f3c3: movups xmm0, xmmword ptr [r9 + rcx + 0x28]
14091f3c9: movaps xmmword ptr [rbp - 0x50], xmm0
14091f3cd: movabs rcx, 0x8000000000000014
14091f3d7: cmp rax, rcx
14091f3da: je 0x140920b5c
14091f3e0: mov qword ptr [rbp + 0x1b0], rax
14091f3e7: mov qword ptr [rbp + 0x1b8], r8
14091f3ee: mov qword ptr [rbp + 0x1c0], r12
14091f3f5: mov qword ptr [rbp + 0x1c8], r15
14091f3fc: mov qword ptr [rbp + 0x1d0], r13
14091f403: movaps xmm0, xmmword ptr [rbp - 0x50]
14091f407: movups xmmword ptr [r14 + 0x20], xmm0
14091f40c: mov rcx, rax
14091f40f: xor rcx, rdi
14091f412: cmp rcx, 0x14
14091f416: mov edx, 0xd
14091f41b: cmovae rcx, rdx
14091f41f: lea rdx, [rip + 0x218ef8e]
14091f426: movsxd rcx, dword ptr [rdx + rcx*4]
14091f42a: add rcx, rdx
14091f42d: jmp rcx
14091f42f: mov rax, r15
14091f432: mov r15, r8
14091f435: mov qword ptr [rbp + 0x210], rax
14091f43c: test rax, rax
14091f43f: mov qword ptr [rbp + 0x50], rax
14091f443: je 0x140920d85
14091f449: mov qword ptr [rbp + 0x90], rax
14091f450: movzx eax, byte ptr [rax + 0x10]
14091f454: test al, al
14091f456: jne 0x140920dfd
14091f45c: mov rax, qword ptr [rbp + 0x90]
14091f463: lock dec qword ptr [rax]
14091f467: jne 0x1409204bc
14091f46d: lea rcx, [rbp + 0x90]
14091f474: call 0x1402e5d80
14091f479: jmp 0x1409204bc
14091f47e: mov rcx, qword ptr [rbp + 0x248]
14091f485: mov rax, qword ptr [rcx]
14091f488: mov r13, qword ptr [rcx + 0x10]
14091f48c: sub rax, r13
14091f48f: cmp rax, 7
14091f493: ja 0x14091fdbc
14091f499: mov byte ptr [rbp + 0x327], 1
14091f4a0: mov qword ptr [rsp + 0x20], 1
14091f4a9: mov r8d, 8
14091f4af: mov r9d, 1
14091f4b5: mov rcx, qword ptr [rbp + 0x248]
14091f4bc: mov rdx, r13
14091f4bf: call 0x1428a9a60
14091f4c4: jmp 0x14091fdbc
14091f4c9: mov qword ptr [rbp + 0x308], r12
14091f4d0: mov qword ptr [rbp + 0x310], r8
14091f4d7: mov rax, qword ptr [rbp + 0x60]
14091f4db: mov rdx, qword ptr [rax]
14091f4de: add rdx, 0x10
14091f4e2: mov byte ptr [rbp + 0x322], 1
14091f4e9: lea rcx, [rbp + 0x260]
14091f4f0: call 0x1416a3620
14091f4f5: mov rax, qword ptr [rbp + 0x278]
14091f4fc: mov rcx, rax
14091f4ff: neg rcx
14091f502: jo 0x140921143
14091f508: movaps xmm8, xmmword ptr [rbp + 0x260]
14091f510: mov rdi, qword ptr [rbp + 0x270]
14091f517: mov rcx, qword ptr [rbp + 0x280]
14091f51e: mov rdx, qword ptr [rbp + 0x290]
14091f525: mov qword ptr [rbp + 0x250], rdx
14091f52c: mov rdx, qword ptr [rbp + 0x298]
14091f533: mov qword ptr [rbp + 0x88], rdx
14091f53a: imul rdx, qword ptr [rbp + 0x288], 0x38
14091f542: add rdx, rcx
14091f545: mov qword ptr [rbp + 0x260], rcx
14091f54c: mov qword ptr [rbp + 0x268], rcx
14091f553: mov qword ptr [rbp + 0x270], rax
14091f55a: mov qword ptr [rbp + 0x278], rdx
14091f561: lea rcx, [rbp + 0x1f0]
14091f568: lea rdx, [rbp + 0x260]
14091f56f: lea r8, [rip + 0x23196e2]
14091f576: call 0x1417376e0
14091f57b: mov rax, qword ptr [rbp + 0x250]
14091f582: test rax, rax
14091f585: je 0x14091f5a1
14091f587: shl rax, 3
14091f58b: lea rdx, [rax + rax*2]
14091f58f: mov r8d, 8
14091f595: mov rcx, qword ptr [rbp + 0x88]
14091f59c: call 0x140613c20
14091f5a1: mov rcx, qword ptr [rbp + 0x1f8]
14091f5a8: mov rdx, qword ptr [rbp + 0x200]
14091f5af: lea rax, [rbp + 0x240]
14091f5b6: mov qword ptr [rbp + 0x260], rax
14091f5bd: cmp rdx, 2
14091f5c1: jae 0x140920a28
14091f5c7: mov rax, qword ptr [rbp + 0x310]
14091f5ce: mov qword ptr [rbp + 0x160], rax
14091f5d5: mov rax, qword ptr [rbp + 0x308]
14091f5dc: mov qword ptr [rbp + 0x168], rax
14091f5e3: mov qword ptr [rbp + 0x170], r15
14091f5ea: mov qword ptr [rbp + 0x178], r13
14091f5f1: mov rax, qword ptr [rbp + 0x200]
14091f5f8: mov qword ptr [rbp + 0x270], rax
14091f5ff: movups xmm0, xmmword ptr [rbp + 0x1f0]
14091f606: movaps xmmword ptr [rbp + 0x260], xmm0
14091f60d: movups xmmword ptr [rbp + 0x278], xmm8
14091f615: mov qword ptr [rbp + 0x288], rdi
14091f61c: movzx eax, byte ptr [rip + 0x2d155e6]
14091f623: mov ecx, 0x80
14091f628: mov edx, 1
14091f62d: call 0x140613c10
14091f632: test rax, rax
14091f635: je 0x140921121
14091f63b: mov qword ptr [rbp + 0x210], 0x80
14091f646: mov qword ptr [rbp + 0x218], rax
14091f64d: mov qword ptr [rbp + 0x220], 0
14091f658: lea rax, [rbp + 0x210]
14091f65f: mov qword ptr [rbp + 0x90], rax
14091f666: lea rax, [rip + 0x218deb3]
14091f66d: mov qword ptr [rbp + 0x98], rax
14091f674: mov qword ptr [rbp + 0xa0], 2
14091f67f: mov qword ptr [rbp + 0xa8], 0
14091f68a: mov byte ptr [rbp + 0xb0], 0
14091f691: lea rcx, [rbp + 0x260]
14091f698: lea rdx, [rbp + 0x90]
14091f69f: call 0x1413fc1b0
14091f6a4: test rax, rax
14091f6a7: jne 0x14092108d
14091f6ad: mov rcx, qword ptr [rbp + 0x210]
14091f6b4: mov rax, qword ptr [rbp + 0x218]
14091f6bb: mov rdx, rcx
14091f6be: neg rdx
14091f6c1: jo 0x1409210e7
14091f6c7: mov rdx, qword ptr [rbp + 0x220]
14091f6ce: mov qword ptr [rbp + 0x90], rcx
14091f6d5: mov qword ptr [rbp + 0x98], rax
14091f6dc: mov qword ptr [rbp + 0xa0], rdx
14091f6e3: mov byte ptr [rbp + 0x321], 0
14091f6ea: lea rcx, [rbp + 0x160]
14091f6f1: lea rdx, [rbp + 0x90]
14091f6f8: call 0x1403108d0
14091f6fd: test rax, rax
14091f700: jne 0x1409210b0
14091f706: mov r13, qword ptr [rbp + 0x268]
14091f70d: mov rdi, qword ptr [rbp + 0x270]
14091f714: test rdi, rdi
14091f717: je 0x1409204fc
14091f71d: lea rbx, [r13 + 8]
14091f721: jmp 0x14091f73d
14091f723: nop word ptr cs:[rax + rax]
14091f730: add rbx, 0x20
14091f734: dec rdi
14091f737: je 0x1409204fc
14091f73d: mov rdx, qword ptr [rbx - 8]
14091f741: test rdx, rdx
14091f744: je 0x14091f730
14091f746: mov rcx, qword ptr [rbx]
14091f749: mov r8d, 1
14091f74f: call 0x140613c20
14091f754: jmp 0x14091f730
14091f756: mov qword ptr [rbp + 0x260], r8
14091f75d: mov qword ptr [rbp + 0x268], r12
14091f764: mov qword ptr [rbp + 0x270], r15
14091f76b: mov qword ptr [rbp + 0x278], r13
14091f772: mov rax, qword ptr [rbp + 0x60]
14091f776: mov rcx, qword ptr [rax]
14091f779: add rcx, 0x10
14091f77d: mov byte ptr [rbp + 0x31f], 1
14091f784: call 0x140bf8950
14091f789: mov byte ptr [rbp + 0x31f], 1
14091f790: lea rcx, [rbp + 0x90]
14091f797: mov rdx, rax
14091f79a: call 0x140c00a80
14091f79f: mov byte ptr [rbp + 0x31f], 0
14091f7a6: lea rcx, [rbp + 0x260]
14091f7ad: lea rdx, [rbp + 0x90]
14091f7b4: call 0x1403108d0
14091f7b9: test rax, rax
14091f7bc: movabs rdi, 0x8000000000000000
14091f7c6: je 0x140920923
14091f7cc: mov qword ptr [rbp + 0x160], rax
14091f7d3: mov rcx, qword ptr [rip + 0x2d0cbce]
14091f7da: mov rcx, qword ptr [rcx]
14091f7dd: test rcx, rcx
14091f7e0: je 0x14091f8ce
14091f7e6: lea rax, [rbp + 0x160]
14091f7ed: mov qword ptr [rbp + 0x90], rax
14091f7f4: lea rax, [rip + 0x43b4e5]
14091f7fb: mov qword ptr [rbp + 0x98], rax
14091f802: lea rcx, [rip + 0x218e877]
14091f809: call 0x14067dd80
14091f80e: movups xmm0, xmmword ptr [rax]
14091f811: mov eax, dword ptr [rax + 0x10]
14091f814: mov qword ptr [rbp + 0x290], 1
14091f81f: lea rdx, [rip + 0x218e662]
14091f826: mov qword ptr [rbp + 0x298], rdx
14091f82d: mov qword ptr [rbp + 0x2a0], 0x1c
14091f838: lea rcx, [rip + 0x218e831]
14091f83f: mov qword ptr [rbp + 0x2b0], rcx
14091f846: mov qword ptr [rbp + 0x2b8], 1
14091f851: lea rcx, [rbp + 0x90]
14091f858: mov qword ptr [rbp + 0x2c0], rcx
14091f85f: mov qword ptr [rbp + 0x2c8], 1
14091f86a: mov qword ptr [rbp + 0x2d0], 0
14091f875: mov qword ptr [rbp + 0x260], 0
14091f880: mov qword ptr [rbp + 0x268], rdx
14091f887: mov qword ptr [rbp + 0x270], 0x1c
14091f892: mov qword ptr [rbp + 0x278], 0
14091f89d: movups xmmword ptr [rbp + 0x280], xmm0
14091f8a4: mov dword ptr [rbp + 0x2a8], 1
14091f8ae: mov dword ptr [rbp + 0x2ac], eax
14091f8b4: lea rcx, [rbp + 0x240]
14091f8bb: lea rdx, [rbp + 0x260]
14091f8c2: call 0x140d4a330
14091f8c7: mov rax, qword ptr [rbp + 0x160]
14091f8ce: mov ecx, eax
14091f8d0: and ecx, 3
14091f8d3: cmp ecx, 1
14091f8d6: jne 0x140920923
14091f8dc: lea rcx, [rax - 1]
14091f8e0: mov qword ptr [rbp + 0x250], rcx
14091f8e7: mov rcx, qword ptr [rax - 1]
14091f8eb: mov qword ptr [rbp + 0x310], rcx
14091f8f2: mov rax, qword ptr [rax + 7]
14091f8f6: mov qword ptr [rbp + 0x308], rax
14091f8fd: mov rax, qword ptr [rax]
14091f900: test rax, rax
14091f903: je 0x14091f90e
14091f905: mov rcx, qword ptr [rbp + 0x310]
14091f90c: call rax
14091f90e: mov rcx, qword ptr [rbp + 0x310]
14091f915: mov rax, qword ptr [rbp + 0x308]
14091f91c: mov rdx, qword ptr [rax + 8]
14091f920: test rdx, rdx
14091f923: je 0x14091f92e
14091f925: mov r8, qword ptr [rax + 0x10]
14091f929: call 0x140613c20
14091f92e: mov edx, 0x18
14091f933: mov r8d, 8
14091f939: mov rcx, qword ptr [rbp + 0x250]
14091f940: call 0x140613c20
14091f945: jmp 0x140920923
14091f94a: mov qword ptr [rbp + 0x1f0], rax
14091f951: mov qword ptr [rbp + 0x1f8], r8
14091f958: mov qword ptr [rbp + 0x200], r12
14091f95f: mov qword ptr [rbp + 0x210], r15
14091f966: mov qword ptr [rbp + 0x218], r13
14091f96d: movaps xmm0, xmmword ptr [rbp - 0x50]
14091f971: lea rax, [rbp + 0x220]
14091f978: movups xmmword ptr [rax], xmm0
14091f97b: mov rax, qword ptr [rip + 0x2d0ca26]
14091f982: mov rax, qword ptr [rax]
14091f985: cmp rax, 3
14091f989: jb 0x14091fa70
14091f98f: lea rax, [rbp + 0x1f0]
14091f996: mov qword ptr [rbp + 0x90], rax
14091f99d: lea rax, [rip - 0x8425b4]
14091f9a4: mov qword ptr [rbp + 0x98], rax
14091f9ab: lea rcx, [rip + 0x218e616]
14091f9b2: call 0x14067dd80
14091f9b7: movups xmm0, xmmword ptr [rax]
14091f9ba: mov eax, dword ptr [rax + 0x10]
14091f9bd: mov qword ptr [rbp + 0x290], 3
14091f9c8: lea rdx, [rip + 0x218e4b9]
14091f9cf: mov qword ptr [rbp + 0x298], rdx
14091f9d6: mov qword ptr [rbp + 0x2a0], 0x1c
14091f9e1: lea rcx, [rip + 0x218e5d0]
14091f9e8: mov qword ptr [rbp + 0x2b0], rcx
14091f9ef: mov qword ptr [rbp + 0x2b8], 1
14091f9fa: lea rcx, [rbp + 0x90]
14091fa01: mov qword ptr [rbp + 0x2c0], rcx
14091fa08: mov qword ptr [rbp + 0x2c8], 1
14091fa13: mov qword ptr [rbp + 0x2d0], 0
14091fa1e: mov qword ptr [rbp + 0x260], 0
14091fa29: mov qword ptr [rbp + 0x268], rdx
14091fa30: mov qword ptr [rbp + 0x270], 0x1c
14091fa3b: mov qword ptr [rbp + 0x278], 0
14091fa46: movups xmmword ptr [rbp + 0x280], xmm0
14091fa4d: mov dword ptr [rbp + 0x2a8], 1
14091fa57: mov dword ptr [rbp + 0x2ac], eax
14091fa5d: lea rcx, [rbp + 0x240]
14091fa64: lea rdx, [rbp + 0x260]
14091fa6b: call 0x140d4a330
14091fa70: mov eax, dword ptr [rbp + 0x318]
14091fa76: mov rcx, qword ptr [rbp + 0x258]
14091fa7d: mov dword ptr [rcx], eax
14091fa7f: mov rcx, qword ptr [rbp + 0x10]
14091fa83: lea rdx, [rbp + 0x1f0]
14091fa8a: call 0x1405b18f0
14091fa8f: lea rcx, [rbp + 0x260]
14091fa96: mov rdx, qword ptr [rbp + 0x78]
14091fa9a: mov r8, rax
14091fa9d: lea r9, [rbp + 0x1f0]
14091faa4: call 0x140b77930
14091faa9: mov rdx, qword ptr [rbp + 0x260]
14091fab0: mov rax, rdx
14091fab3: neg rax
14091fab6: jo 0x140921178
14091fabc: mov r13, qword ptr [rbp + 0x278]
14091fac3: mov rdi, qword ptr [rbp + 0x280]
14091faca: jae 0x14091fade
14091facc: mov rcx, qword ptr [rbp + 0x268]
14091fad3: mov r8d, 1
14091fad9: call 0x140613c20
14091fade: mov qword ptr [rbp + 0x208], rdi
14091fae5: mov byte ptr [rbp + 0x326], 1
14091faec: call 0x140c17420
14091faf1: mov qword ptr [rbp + 0x308], rax
14091faf8: mov rcx, rax
14091fafb: add rcx, 8
14091faff: mov rdx, r13
14091fb02: call 0x140c177f0
14091fb07: mov qword ptr [rbp + 0x310], rax
14091fb0e: test rax, rax
14091fb11: je 0x140921167
14091fb17: mov qword ptr [rbp + 0x250], rdx
14091fb1e: mov rax, qword ptr [rdx]
14091fb21: test rax, rax
14091fb24: je 0x14091fb2f
14091fb26: mov rcx, qword ptr [rbp + 0x310]
14091fb2d: call rax
14091fb2f: mov rax, qword ptr [rbp + 0x250]
14091fb36: mov rdx, qword ptr [rax + 8]
14091fb3a: test rdx, rdx
14091fb3d: je 0x14091fb4f
14091fb3f: mov r8, qword ptr [rax + 0x10]
14091fb43: mov rcx, qword ptr [rbp + 0x310]
14091fb4a: call 0x140613c20
14091fb4f: mov byte ptr [rbp + 0x326], 1
14091fb56: mov rdi, qword ptr [rbp + 0x308]
14091fb5d: mov rcx, rdi
14091fb60: call 0x140c81e10
14091fb65: mov al, 1
14091fb67: lock cmpxchg byte ptr [rdi], bl
14091fb6b: jne 0x140920a4c
14091fb71: mov byte ptr [rbp + 0x326], 1
14091fb78: xor ecx, ecx
14091fb7a: call 0x140c169f0
14091fb7f: cmp rbx, qword ptr [rbp + 0x210]
14091fb86: jno 0x140920529
14091fb8c: mov rax, qword ptr [rbp + 0x208]
14091fb93: lock dec qword ptr [rax]
14091fb97: jne 0x140920770
14091fb9d: lea rcx, [rbp + 0x208]
14091fba4: call 0x140b5cdb0
14091fba9: jmp 0x140920770
14091fbae: mov rcx, qword ptr [rbp - 0x20]
14091fbb2: mov r13, qword ptr [rcx + 0xf8]
14091fbb9: mov rax, qword ptr [rcx + 0x100]
14091fbc0: lea r12, [rax*4]
14091fbc8: add r12, r13
14091fbcb: movzx r14d, byte ptr [rcx + 0x108]
14091fbd3: mov eax, 8
14091fbd8: mov qword ptr [rbp + 0x310], rax
14091fbdf: mov edi, 8
14091fbe4: xor ebx, ebx
14091fbe6: xor r15d, r15d
14091fbe9: test r14b, r14b
14091fbec: jne 0x14091fc60
14091fbee: cmp r15, rbx
14091fbf1: jne 0x14091fc3b
14091fbf3: nop word ptr cs:[rax + rax]
14091fc00: cmp r13, r12
14091fc03: je 0x1409204cb
14091fc09: mov ecx, dword ptr [r13]
14091fc0d: add r13, 4
14091fc11: mov rax, qword ptr [rbp + 0x230]
14091fc18: mov rax, qword ptr [rax + 0x100]
14091fc1f: lea rcx, [rcx + rcx*4]
14091fc23: shl rcx, 5
14091fc27: mov rbx, qword ptr [rax + rcx + 0x58]
14091fc2c: test rbx, rbx
14091fc2f: je 0x14091fc00
14091fc31: add rax, rcx
14091fc34: mov rdi, qword ptr [rax + 0x50]
14091fc38: xor r15d, r15d
14091fc3b: mov rax, r15
14091fc3e: shl rax, 4
14091fc42: add rax, rdi
14091fc45: mov rax, qword ptr [rax]
14091fc48: mov rcx, rax
14091fc4b: shr rcx, 0x20
14091fc4f: jne 0x14091fcca
14091fc51: jmp 0x1409204cb
14091fc56: nop word ptr cs:[rax + rax]
14091fc60: cmp r15, rbx
14091fc63: jne 0x14091fcaf
14091fc65: nop word ptr cs:[rax + rax]
14091fc70: cmp r13, r12
14091fc73: je 0x1409204cb
14091fc79: mov ecx, dword ptr [r13]
14091fc7d: add r13, 4
14091fc81: mov rax, qword ptr [rbp + 0x230]
14091fc88: mov rax, qword ptr [rax + 0x1a8]
14091fc8f: lea rcx, [rcx + rcx*8]
14091fc93: mov rbx, qword ptr [rax + rcx*8 + 0x10]
14091fc98: test rbx, rbx
14091fc9b: je 0x14091fc70
14091fc9d: lea rax, [rax + rcx*8]
14091fca1: mov rax, qword ptr [rax + 8]
14091fca5: mov qword ptr [rbp + 0x310], rax
14091fcac: xor r15d, r15d
14091fcaf: mov rax, qword ptr [rbp + 0x310]
14091fcb6: lea rax, [rax + r15*8]
14091fcba: mov rax, qword ptr [rax]
14091fcbd: mov rcx, rax
14091fcc0: shr rcx, 0x20
14091fcc4: je 0x1409204cb
14091fcca: mov qword ptr [rbp + 0x280], rax
14091fcd1: mov rax, qword ptr [rbp - 0x30]
14091fcd5: mov qword ptr [rbp + 0x260], rax
14091fcdc: mov rax, qword ptr [rbp - 0x40]
14091fce0: mov qword ptr [rbp + 0x268], rax
14091fce7: mov rax, qword ptr [rbp - 0x38]
14091fceb: mov qword ptr [rbp + 0x270], rax
14091fcf2: mov rax, qword ptr [rbp - 0x28]
14091fcf6: mov qword ptr [rbp + 0x278], rax
14091fcfd: mov byte ptr [rbp + 0x327], 1
14091fd04: lea rcx, [rbp + 0x260]
14091fd0b: lea rdx, [rip + 0x218e41e]
14091fd12: call 0x140d14770
14091fd17: inc r15
14091fd1a: test r14b, r14b
14091fd1d: je 0x14091fbee
14091fd23: jmp 0x14091fc60
14091fd28: test r8, r8
14091fd2b: je 0x140920923
14091fd31: jmp 0x140920d37
14091fd36: mov eax, dword ptr [rbp + 0x23c]
14091fd3c: mov rcx, qword ptr [rbp + 0x158]
14091fd43: mov dword ptr [rcx], eax
14091fd45: mov rax, qword ptr [rbp + 0x150]
14091fd4c: mov byte ptr [rax + 0x31], 0
14091fd50: jmp 0x140920923
14091fd55: mov rax, qword ptr [rbp - 8]
14091fd59: mov ecx, dword ptr [rbp + 0x140]
14091fd5f: mov dword ptr [rax], ecx
14091fd61: mov rax, qword ptr [rbp + 0x68]
14091fd65: mov rdi, qword ptr [rax + 0x30]
14091fd69: mov rbx, qword ptr [rax + 0x40]
14091fd6d: cmp rdi, qword ptr [rax + 0x20]
14091fd71: jne 0x14091fd8a
14091fd73: mov byte ptr [rbp + 0x327], 1
14091fd7a: mov rcx, qword ptr [rbp + 0x38]
14091fd7e: lea rdx, [rip + 0x23692f3]
14091fd85: call 0x140562870
14091fd8a: mov rcx, qword ptr [rbp + 0x68]
14091fd8e: mov rax, qword ptr [rcx + 0x28]
14091fd92: mov qword ptr [rax + rdi*8], rbx
14091fd96: inc rdi
14091fd99: mov qword ptr [rcx + 0x30], rdi
14091fd9d: inc qword ptr [rcx + 0x40]
14091fda1: mov rcx, qword ptr [rbp + 0x248]
14091fda8: mov rax, qword ptr [rcx]
14091fdab: mov r13, qword ptr [rcx + 0x10]
14091fdaf: sub rax, r13
14091fdb2: cmp rax, 7
14091fdb6: jbe 0x1409209c8
14091fdbc: mov rcx, qword ptr [rbp + 0x248]
14091fdc3: mov rax, qword ptr [rcx + 8]
14091fdc7: lea rdx, [rip + 0x9b7a92]
14091fdce: mov qword ptr [rax + r13], rdx
14091fdd2: add r13, 8
14091fdd6: jmp 0x1409200ca
14091fddb: mov rax, qword ptr [rbp + 0x1e8]
14091fde2: cmp byte ptr [rax + 0x38], 0
14091fde6: mov r13, r8
14091fde9: jne 0x14092082b
14091fdef: mov rcx, qword ptr [rbp + 0x1e8]
14091fdf6: mov rax, qword ptr [rcx]
14091fdf9: mov rcx, qword ptr [rcx + 0x18]
14091fdfd: movdqa xmm0, xmmword ptr [rax]
14091fe01: pmovmskb r8d, xmm0
14091fe06: not r8d
14091fe09: lea rdx, [rax + 0x10]
14091fe0d: jmp 0x14091fe34
14091fe0f: nop 
14091fe10: lea r9d, [r8 - 1]
14091fe14: and r9d, r8d
14091fe17: tzcnt r8d, r8d
14091fe1c: shl r8d, 5
14091fe20: mov r10, rax
14091fe23: sub r10, r8
14091fe26: cmp byte ptr [r10 - 8], 1
14091fe2b: mov r8d, r9d
14091fe2e: je 0x14092082b
14091fe34: sub rcx, 1
14091fe38: jb 0x1409207fa
14091fe3e: test r8w, r8w
14091fe42: jne 0x14091fe10
14091fe44: nop word ptr cs:[rax + rax]
14091fe50: movdqa xmm0, xmmword ptr [rdx]
14091fe54: pmovmskb r8d, xmm0
14091fe59: add rax, -0x200
14091fe5f: add rdx, 0x10
14091fe63: cmp r8d, 0xffff
14091fe6a: je 0x14091fe50
14091fe6c: mov r10d, 0xfffffffe
14091fe72: sub r10d, r8d
14091fe75: not r8d
14091fe78: mov r9d, r8d
14091fe7b: and r9d, r10d
14091fe7e: jmp 0x14091fe17
14091fe80: mov eax, dword ptr [rbp + 0x23c]
14091fe86: mov rcx, qword ptr [rbp + 0x158]
14091fe8d: mov dword ptr [rcx], eax
14091fe8f: mov rax, qword ptr [rbp + 0x150]
14091fe96: mov qword ptr [rax], 0
14091fe9d: mov dword ptr [rax + 8], 0
14091fea4: mov qword ptr [rax + 0x28], 0
14091feac: mov rcx, qword ptr [rbp + 8]
14091feb0: movups xmmword ptr [rcx], xmm6
14091feb3: mov dword ptr [rcx + 0x10], 0
14091feba: mov word ptr [rax + 0x30], 1
14091fec0: mov eax, dword ptr [rbp + 0x13c]
14091fec6: mov rcx, qword ptr [rbp - 0x10]
14091feca: mov dword ptr [rcx], eax
14091fecc: mov rax, qword ptr [rbp - 0x18]
14091fed0: mov dword ptr [rax + 0x108], 0
14091feda: jmp 0x140920923
14091fedf: mov qword ptr [rbp + 0x308], r12
14091fee6: mov qword ptr [rbp + 0x310], r8
14091feed: mov rbx, qword ptr [rbp + 0x1d8]
14091fef4: mov eax, dword ptr [rbp + 0x144]
14091fefa: mov rcx, qword ptr [rbp]
14091fefe: mov dword ptr [rcx], eax
14091ff00: mov rax, qword ptr [rbp + 0x70]
14091ff04: mov rdi, qword ptr [rax + 0x30]
14091ff08: mov r14, qword ptr [rax + 0x40]
14091ff0c: cmp rdi, qword ptr [rax + 0x20]
14091ff10: jne 0x14091ff22
14091ff12: mov rcx, qword ptr [rbp + 0x40]
14091ff16: lea rdx, [rip + 0x231a22b]
14091ff1d: call 0x1405629f0
14091ff22: mov rdx, qword ptr [rbp + 0x70]
14091ff26: mov rax, qword ptr [rdx + 0x28]
14091ff2a: lea rcx, [rdi + rdi*2]
14091ff2e: shl rcx, 4
14091ff32: mov r8, qword ptr [rbp + 0x310]
14091ff39: mov qword ptr [rax + rcx], r8
14091ff3d: mov r8, qword ptr [rbp + 0x308]
14091ff44: mov qword ptr [rax + rcx + 8], r8
14091ff49: mov qword ptr [rax + rcx + 0x10], r15
14091ff4e: mov qword ptr [rax + rcx + 0x18], r13
14091ff53: mov qword ptr [rax + rcx + 0x20], rbx
14091ff58: mov qword ptr [rax + rcx + 0x28], r14
14091ff5d: inc rdi
14091ff60: mov qword ptr [rdx + 0x30], rdi
14091ff64: inc qword ptr [rdx + 0x40]
14091ff68: jmp 0x140920910
14091ff6d: mov rax, qword ptr [rbp + 0x1e8]
14091ff74: cmp byte ptr [rax + 0x38], 0
14091ff78: jne 0x140920a90
14091ff7e: mov rcx, qword ptr [rbp + 0x1e8]
14091ff85: mov rax, qword ptr [rcx]
14091ff88: mov rcx, qword ptr [rcx + 0x18]
14091ff8c: movdqa xmm0, xmmword ptr [rax]
14091ff90: pmovmskb r8d, xmm0
14091ff95: not r8d
14091ff98: lea rdx, [rax + 0x10]
14091ff9c: jmp 0x14091ffc4
14091ff9e: nop 
14091ffa0: lea r9d, [r8 - 1]
14091ffa4: and r9d, r8d
14091ffa7: tzcnt r8d, r8d
14091ffac: shl r8d, 5
14091ffb0: mov r10, rax
14091ffb3: sub r10, r8
14091ffb6: cmp byte ptr [r10 - 8], 0
14091ffbb: mov r8d, r9d
14091ffbe: jne 0x140920a90
14091ffc4: sub rcx, 1
14091ffc8: jb 0x1409207c4
14091ffce: test r8w, r8w
14091ffd2: jne 0x14091ffa0
14091ffd4: nop word ptr cs:[rax + rax]
14091ffe0: movdqa xmm0, xmmword ptr [rdx]
14091ffe4: pmovmskb r8d, xmm0
14091ffe9: add rax, -0x200
14091ffef: add rdx, 0x10
14091fff3: cmp r8d, 0xffff
14091fffa: je 0x14091ffe0
14091fffc: mov r10d, 0xfffffffe
140920002: sub r10d, r8d
140920005: not r8d
140920008: mov r9d, r8d
14092000b: and r9d, r10d
14092000e: jmp 0x14091ffa7
140920010: movd xmm0, r8d
140920015: ucomiss xmm7, xmm0
140920018: ja 0x140921040
14092001e: mov edx, r8d
140920021: shr edx, 0x17
140920024: movzx eax, dl
140920027: xor ebx, ebx
140920029: cmp ax, 0x60
14092002d: jb 0x140920080
14092002f: mov edi, r8d
140920032: and edi, 0x7fffff
140920038: or edi, 0x800000
14092003e: cmp ax, 0x7f
140920042: jae 0x140920958
140920048: mov eax, edi
14092004a: lea ecx, [rdx + 0x2a]
14092004d: shl rax, cl
140920050: mov ecx, 0x3b9aca00
140920055: mul rcx
140920058: mov rcx, rax
14092005b: neg rcx
14092005e: setno cl
140920061: shr rax, 0x3f
140920065: mov r8d, edx
140920068: or r8b, cl
14092006b: and r8b, al
14092006e: movzx ebx, r8b
140920072: add ebx, edx
140920074: cmp ebx, 0x3b9aca00
14092007a: je 0x140921040
140920080: xor edi, edi
140920082: mov rcx, qword ptr [rbp + 0x248]
140920089: mov rax, qword ptr [rcx]
14092008c: mov r13, qword ptr [rcx + 0x10]
140920090: sub rax, r13
140920093: cmp rax, 0x1f
140920097: jbe 0x1409209f8
14092009d: mov rcx, qword ptr [rbp + 0x248]
1409200a4: mov rax, qword ptr [rcx + 8]
1409200a8: lea rdx, [rip + 0x9b9911]
1409200af: mov qword ptr [rax + r13], rdx
1409200b3: mov qword ptr [rax + r13 + 8], 2
1409200bc: mov qword ptr [rax + r13 + 0x10], rdi
1409200c1: mov dword ptr [rax + r13 + 0x18], ebx
1409200c6: add r13, 0x20
1409200ca: mov qword ptr [rcx + 0x10], r13
1409200ce: jmp 0x140920910
1409200d3: lea rax, [rbp + 0x272]
1409200da: movups xmmword ptr [rax - 2], xmm6
1409200de: movups xmmword ptr [rax + 0x1e], xmm6
1409200e2: mov qword ptr [rbp + 0x260], 0
1409200ed: mov qword ptr [rbp + 0x268], 8
1409200f8: mov qword ptr [rbp + 0x280], 0
140920103: mov qword ptr [rbp + 0x288], 8
14092010e: movabs rax, 0x8000000000000003
140920118: mov qword ptr [rbp + 0x2a0], rax
14092011f: mov byte ptr [rbp + 0x327], 1
140920126: mov rcx, qword ptr [rbp + 0x410]
14092012d: lea rdx, [rbp + 0x260]
140920134: lea r8, [rip + 0x218dd6d]
14092013b: call 0x1412e1400
140920140: jmp 0x140920910
140920145: mov eax, dword ptr [rbp + 0x23c]
14092014b: mov rcx, qword ptr [rbp + 0x158]
140920152: mov dword ptr [rcx], eax
140920154: mov rax, qword ptr [rbp + 0x150]
14092015b: mov byte ptr [rax + 0x31], 1
14092015f: jmp 0x140920923
140920164: mov qword ptr [rbp + 0x160], r8
14092016b: mov qword ptr [rbp + 0x168], r12
140920172: mov qword ptr [rbp + 0x170], r15
140920179: mov rax, qword ptr [rip + 0x2d0c228]
140920180: mov rax, qword ptr [rax]
140920183: cmp rax, 3
140920187: jb 0x14092027c
14092018d: lea rax, [rbp + 0x160]
140920194: mov qword ptr [rbp + 0x90], rax
14092019b: lea rax, [rip - 0x842db2]
1409201a2: mov qword ptr [rbp + 0x98], rax
1409201a9: mov byte ptr [rbp + 0x325], 1
1409201b0: lea rcx, [rip + 0x218dd99]
1409201b7: call 0x14067dd80
1409201bc: movups xmm0, xmmword ptr [rax]
1409201bf: mov eax, dword ptr [rax + 0x10]
1409201c2: mov qword ptr [rbp + 0x290], 3
1409201cd: lea rdx, [rip + 0x218dcb4]
1409201d4: mov qword ptr [rbp + 0x298], rdx
1409201db: mov qword ptr [rbp + 0x2a0], 0x1c
1409201e6: lea rcx, [rip + 0x218dd53]
1409201ed: mov qword ptr [rbp + 0x2b0], rcx
1409201f4: mov qword ptr [rbp + 0x2b8], 1
1409201ff: lea rcx, [rbp + 0x90]
140920206: mov qword ptr [rbp + 0x2c0], rcx
14092020d: mov qword ptr [rbp + 0x2c8], 1
140920218: mov qword ptr [rbp + 0x2d0], 0
140920223: mov qword ptr [rbp + 0x260], 0
14092022e: mov qword ptr [rbp + 0x268], rdx
140920235: mov qword ptr [rbp + 0x270], 0x1c
140920240: mov qword ptr [rbp + 0x278], 0
14092024b: movups xmmword ptr [rbp + 0x280], xmm0
140920252: mov dword ptr [rbp + 0x2a8], 1
14092025c: mov dword ptr [rbp + 0x2ac], eax
140920262: mov byte ptr [rbp + 0x325], 1
140920269: lea rcx, [rbp + 0x240]
140920270: lea rdx, [rbp + 0x260]
140920277: call 0x140d4a330
14092027c: mov byte ptr [rbp + 0x325], 1
140920283: mov cl, 1
140920285: call 0x140c169f0
14092028a: mov byte ptr [rbp + 0x325], 1
140920291: call 0x140c17420
140920296: mov qword ptr [rbp + 0x310], rax
14092029d: mov byte ptr [rbp + 0x324], 1
1409202a4: lea rcx, [rbp + 0x260]
1409202ab: call 0x14167fb50
1409202b0: movups xmm0, xmmword ptr [rbp + 0x2a0]
1409202b7: lea rax, [rbp + 0x96]
1409202be: movups xmmword ptr [rax + 0x40], xmm0
1409202c2: movups xmm0, xmmword ptr [rbp + 0x260]
1409202c9: movups xmm1, xmmword ptr [rbp + 0x270]
1409202d0: movups xmm2, xmmword ptr [rbp + 0x280]
1409202d7: movups xmm3, xmmword ptr [rbp + 0x290]
1409202de: movups xmmword ptr [rax + 0x30], xmm3
1409202e2: movups xmmword ptr [rax + 0x20], xmm2
1409202e6: movups xmmword ptr [rax + 0x10], xmm1
1409202ea: movups xmmword ptr [rax], xmm0
1409202ed: mov qword ptr [rbp + 0x260], 1
1409202f8: mov qword ptr [rbp + 0x268], 1
140920303: mov word ptr [rbp + 0x270], 0
14092030c: mov rax, qword ptr [rbp + 0xde]
140920313: lea rcx, [rbp + 0x272]
14092031a: mov qword ptr [rcx + 0x4e], rax
14092031e: movups xmm0, xmmword ptr [rbp + 0xd0]
140920325: movups xmmword ptr [rcx + 0x40], xmm0
140920329: movups xmm0, xmmword ptr [rbp + 0x90]
140920330: movups xmm1, xmmword ptr [rbp + 0xa0]
140920337: movups xmm2, xmmword ptr [rbp + 0xb0]
14092033e: movups xmm3, xmmword ptr [rbp + 0xc0]
140920345: movups xmmword ptr [rcx + 0x30], xmm3
140920349: movups xmmword ptr [rcx + 0x20], xmm2
14092034d: movups xmmword ptr [rcx + 0x10], xmm1
140920351: movups xmmword ptr [rcx], xmm0
140920354: movzx eax, byte ptr [rip + 0x2d148ae]
14092035b: mov ecx, 0x68
140920360: mov edx, 8
140920365: call 0x140613c10
14092036a: test rax, rax
14092036d: je 0x140921204
140920373: mov r15, rax
140920376: mov rax, qword ptr [rbp + 0x2c0]
14092037d: mov qword ptr [r15 + 0x60], rax
140920381: movups xmm0, xmmword ptr [rbp + 0x2b0]
140920388: movups xmmword ptr [r15 + 0x50], xmm0
14092038d: movups xmm0, xmmword ptr [rbp + 0x2a0]
140920394: movups xmmword ptr [r15 + 0x40], xmm0
140920399: movups xmm0, xmmword ptr [rbp + 0x260]
1409203a0: movups xmm1, xmmword ptr [rbp + 0x270]
1409203a7: movups xmm2, xmmword ptr [rbp + 0x280]
1409203ae: movups xmm3, xmmword ptr [rbp + 0x290]
1409203b5: movups xmmword ptr [r15 + 0x30], xmm3
1409203ba: movups xmmword ptr [r15 + 0x20], xmm2
1409203bf: movups xmmword ptr [r15 + 0x10], xmm1
1409203c4: movups xmmword ptr [r15], xmm0
1409203c8: mov qword ptr [rbp + 0x210], r15
1409203cf: lock inc qword ptr [r15]
1409203d3: mov rdi, qword ptr [rbp + 0x310]
1409203da: jle 0x1409213f1
1409203e0: mov qword ptr [rbp + 0x260], r15
1409203e7: movzx eax, byte ptr [rip + 0x2d1481b]
1409203ee: mov ecx, 8
1409203f3: mov edx, 8
1409203f8: call 0x140613c10
1409203fd: test rax, rax
140920400: je 0x1409211e9
140920406: mov rcx, rdi
140920409: add rcx, 8
14092040d: mov qword ptr [rax], r15
140920410: mov byte ptr [rbp + 0x323], 1
140920417: mov rdx, rax
14092041a: lea r8, [rip + 0x218db47]
140920421: call 0x140c17740
140920426: mov ecx, dword ptr [rbp + 0x318]
14092042c: mov rdx, qword ptr [rbp + 0x258]
140920433: mov dword ptr [rdx], ecx
140920435: mov rcx, qword ptr [rbp + 0x170]
14092043c: mov qword ptr [rbp + 0x270], rcx
140920443: movups xmm0, xmmword ptr [rbp + 0x160]
14092044a: movaps xmmword ptr [rbp + 0x260], xmm0
140920451: mov byte ptr [rbp + 0x323], 0
140920458: mov rcx, qword ptr [rbp + 0x78]
14092045c: lea rdx, [rbp + 0x260]
140920463: mov r8, rax
140920466: mov r9, r15
140920469: call 0x141528e40
14092046e: mov qword ptr [rbp + 0x90], rax
140920475: mov qword ptr [rbp + 0x98], rdx
14092047c: test rdx, rdx
14092047f: je 0x14092049a
140920481: lock dec qword ptr [rdx]
140920485: jne 0x14092049a
140920487: mov byte ptr [rbp + 0x323], 0
14092048e: lea rcx, [rbp + 0x98]
140920495: call 0x140b5cdb0
14092049a: mov byte ptr [rbp + 0x325], 0
1409204a1: mov rdi, qword ptr [rbp + 0x310]
1409204a8: mov rcx, rdi
1409204ab: call 0x140c81e10
1409204b0: mov al, 1
1409204b2: lock cmpxchg byte ptr [rdi], bl
1409204b6: jne 0x140920a62
1409204bc: movabs rdi, 0x8000000000000000
1409204c6: jmp 0x140920923
1409204cb: mov eax, dword ptr [rbp + 0x148]
1409204d1: mov rcx, qword ptr [rbp + 0x20]
1409204d5: mov dword ptr [rcx], eax
1409204d7: mov rax, qword ptr [rbp + 0x18]
1409204db: mov byte ptr [rax + 0x48], 0
1409204df: mov rcx, qword ptr [rbp + 0x28]
1409204e3: mov byte ptr [rcx + 5], 0
1409204e7: mov eax, dword ptr [rbp + 0x14c]
1409204ed: mov rdx, qword ptr [rbp + 0x30]
1409204f1: mov dword ptr [rdx], eax
1409204f3: mov byte ptr [rcx + 4], 0
1409204f7: jmp 0x140920910
1409204fc: mov rdx, qword ptr [rbp + 0x260]
140920503: test rdx, rdx
140920506: je 0x14092051a
140920508: shl rdx, 5
14092050c: mov r8d, 8
140920512: mov rcx, r13
140920515: call 0x140613c20
14092051a: movabs rdi, 0x8000000000000000
140920524: jmp 0x140920921
140920529: lea rax, [rbp + 0x210]
140920530: mov qword ptr [rbp + 0x58], rax
140920534: mov rcx, qword ptr [rbp + 0x208]
14092053b: lock dec qword ptr [rcx]
14092053f: jne 0x140921218
140920545: movzx ebx, byte ptr [rcx + 0x11]
140920549: mov rax, qword ptr [rcx + 0x60]
14092054d: mov qword ptr [rbp + 0x2ae], rax
140920554: movups xmm0, xmmword ptr [rcx + 0x52]
140920558: movaps xmmword ptr [rbp + 0x2a0], xmm0
14092055f: movups xmm0, xmmword ptr [rcx + 0x42]
140920563: movaps xmmword ptr [rbp + 0x290], xmm0
14092056a: movups xmm0, xmmword ptr [rcx + 0x32]
14092056e: movaps xmmword ptr [rbp + 0x280], xmm0
140920575: movups xmm0, xmmword ptr [rcx + 0x22]
140920579: movaps xmmword ptr [rbp + 0x270], xmm0
140920580: movups xmm0, xmmword ptr [rcx + 0x12]
140920584: movaps xmmword ptr [rbp + 0x260], xmm0
14092058b: cmp rcx, -1
14092058f: je 0x1409205a8
140920591: lock dec qword ptr [rcx + 8]
140920596: jne 0x1409205a8
140920598: mov edx, 0x68
14092059d: mov r8d, 8
1409205a3: call 0x140613c20
1409205a8: mov rax, qword ptr [rbp + 0x2ae]
1409205af: mov qword ptr [rbp + 0xde], rax
1409205b6: movaps xmm0, xmmword ptr [rbp + 0x2a0]
1409205bd: movaps xmmword ptr [rbp + 0xd0], xmm0
1409205c4: movaps xmm0, xmmword ptr [rbp + 0x260]
1409205cb: movaps xmm1, xmmword ptr [rbp + 0x270]
1409205d2: movaps xmm2, xmmword ptr [rbp + 0x280]
1409205d9: movaps xmm3, xmmword ptr [rbp + 0x290]
1409205e0: movaps xmmword ptr [rbp + 0xc0], xmm3
1409205e7: movaps xmmword ptr [rbp + 0xb0], xmm2
1409205ee: movaps xmmword ptr [rbp + 0xa0], xmm1
1409205f5: movaps xmmword ptr [rbp + 0x90], xmm0
1409205fc: lea rax, [rbp + 0x96]
140920603: movups xmm0, xmmword ptr [rax]
140920606: movups xmm1, xmmword ptr [rax + 0x10]
14092060a: movups xmm2, xmmword ptr [rax + 0x20]
14092060e: movups xmm3, xmmword ptr [rax + 0x30]
140920612: movaps xmmword ptr [rbp + 0x160], xmm0
140920619: movaps xmmword ptr [rbp + 0x170], xmm1
140920620: movaps xmmword ptr [rbp + 0x180], xmm2
140920627: movaps xmmword ptr [rbp + 0x190], xmm3
14092062e: movups xmm0, xmmword ptr [rax + 0x40]
140920632: movaps xmmword ptr [rbp + 0x1a0], xmm0
140920639: test bl, bl
14092063b: jne 0x140921230
140920641: movaps xmm0, xmmword ptr [rbp + 0x1a0]
140920648: movaps xmmword ptr [rbp + 0x2a0], xmm0
14092064f: movaps xmm0, xmmword ptr [rbp + 0x160]
140920656: movaps xmm1, xmmword ptr [rbp + 0x170]
14092065d: movaps xmm2, xmmword ptr [rbp + 0x180]
140920664: movaps xmm3, xmmword ptr [rbp + 0x190]
14092066b: movaps xmmword ptr [rbp + 0x290], xmm3
140920672: movaps xmmword ptr [rbp + 0x280], xmm2
140920679: movaps xmmword ptr [rbp + 0x270], xmm1
140920680: movaps xmmword ptr [rbp + 0x260], xmm0
140920687: mov rcx, qword ptr [rbp + 0x58]
14092068b: mov dword ptr [rbp + 0x90], 0
140920695: lea rdx, [rbp + 0x98]
14092069c: movups xmmword ptr [rdx], xmm6
14092069f: mov dword ptr [rbp + 0xa8], 7
1409206a9: mov word ptr [rax + 0x1e], 0
1409206af: mov qword ptr [rax + 0x16], 0
1409206b7: mov byte ptr [rbp + 0xb1], 1
1409206be: mov word ptr [rbp + 0xb3], 0x101
1409206c7: call 0x140d76ed0
1409206cc: xor ebx, ebx
1409206ce: mov r8, rdx
1409206d1: lea rcx, [rbp + 0x90]
1409206d8: mov rdx, rax
1409206db: call 0x140d59dc0
1409206e0: mov rdi, rdx
1409206e3: test al, 1
1409206e5: jne 0x1409212ff
1409206eb: movzx eax, byte ptr [rip + 0x2d14517]
1409206f2: mov ecx, 0x2000
1409206f7: mov edx, 1
1409206fc: call 0x140613c10
140920701: test rax, rax
140920704: je 0x1409212dd
14092070a: mov qword ptr [rbp + 0x90], 0x2000
140920715: mov qword ptr [rbp + 0x98], rax
14092071c: mov qword ptr [rbp + 0xa0], 0
140920727: mov byte ptr [rbp + 0xa8], 0
14092072e: mov qword ptr [rbp + 0xb0], rdi
140920735: lea rcx, [rbp + 0x260]
14092073c: lea rdx, [rbp + 0x90]
140920743: call 0x14172ed70
140920748: test rax, rax
14092074b: jne 0x1409212a6
140920751: lea rcx, [rbp + 0x90]
140920758: call 0x140892dc0
14092075d: mov byte ptr [rbp + 0x326], 0
140920764: lea rcx, [rbp + 0x260]
14092076b: call 0x140888890
140920770: mov rdx, qword ptr [rbp + 0x210]
140920777: mov rax, rdx
14092077a: neg rax
14092077d: jo 0x140920793
14092077f: jae 0x140920793
140920781: mov rcx, qword ptr [rbp + 0x218]
140920788: mov r8d, 1
14092078e: call 0x140613c20
140920793: mov rdx, qword ptr [rbp + 0x1f0]
14092079a: test rdx, rdx
14092079d: movabs rdi, 0x8000000000000000
1409207a7: je 0x140920923
1409207ad: mov rcx, qword ptr [rbp + 0x1f8]
1409207b4: mov r8d, 1
1409207ba: call 0x140613c20
1409207bf: jmp 0x140920923
1409207c4: mov byte ptr [rbp + 0x327], 1
1409207cb: mov rcx, qword ptr [rbp + 0x80]
1409207d2: call 0x1408bc530
1409207d7: test al, al
1409207d9: movabs rdi, 0x8000000000000000
1409207e3: lea r14, [rbp + 0x1b8]
1409207ea: mov ebx, 0
1409207ef: jne 0x140920923
1409207f5: jmp 0x140920a90
1409207fa: mov byte ptr [rbp + 0x327], 1
140920801: mov rcx, qword ptr [rbp + 0x80]
140920808: call 0x1408bc530
14092080d: test al, al
14092080f: movabs rdi, 0x8000000000000000
140920819: lea r14, [rbp + 0x1b8]
140920820: mov ebx, 0
140920825: jne 0x140920923
14092082b: test r13, r13
14092082e: jne 0x140920eb7
140920834: mov rax, qword ptr [rip + 0x2d0bb6d]
14092083b: mov rax, qword ptr [rax]
14092083e: cmp rax, 2
140920842: jb 0x140920923
140920848: mov byte ptr [rbp + 0x327], 1
14092084f: lea rcx, [rip + 0x218d61a]
140920856: call 0x14067dd80
14092085b: movups xmm0, xmmword ptr [rax]
14092085e: mov eax, dword ptr [rax + 0x10]
140920861: mov qword ptr [rbp + 0x290], 2
14092086c: lea rdx, [rip + 0x218d615]
140920873: mov qword ptr [rbp + 0x298], rdx
14092087a: mov qword ptr [rbp + 0x2a0], 0x1c
140920885: lea rcx, [rip + 0x218d5d4]
14092088c: mov qword ptr [rbp + 0x2b0], rcx
140920893: mov qword ptr [rbp + 0x2b8], 1
14092089e: mov qword ptr [rbp + 0x2c0], 8
1409208a9: mov qword ptr [rbp + 0x260], 0
1409208b4: lea rcx, [rbp + 0x272]
1409208bb: movups xmmword ptr [rcx + 0x56], xmm6
1409208bf: mov qword ptr [rbp + 0x268], rdx
1409208c6: mov qword ptr [rbp + 0x270], 0x1c
1409208d1: mov qword ptr [rbp + 0x278], 0
1409208dc: movups xmmword ptr [rbp + 0x280], xmm0
1409208e3: mov dword ptr [rbp + 0x2a8], 1
1409208ed: mov dword ptr [rbp + 0x2ac], eax
1409208f3: mov byte ptr [rbp + 0x327], 1
1409208fa: lea rcx, [rbp + 0x240]
140920901: lea rdx, [rbp + 0x260]
140920908: call 0x140d4a330
14092090d: nop dword ptr [rax]
140920910: movabs rdi, 0x8000000000000000
14092091a: lea r14, [rbp + 0x1b8]
140920921: xor ebx, ebx
140920923: mov rax, qword ptr [rbp + 0x1b0]
14092092a: xor rax, rdi
14092092d: cmp rax, 0x14
140920931: mov ecx, 0xd
140920936: cmovae rax, rcx
14092093a: cmp rax, 0xf
14092093e: ja 0x14091f349
140920944: mov ecx, 0xf101
140920949: bt rcx, rax
14092094d: jb 0x14091f360
140920953: jmp 0x14091f349
140920958: movzx eax, ax
14092095b: cmp eax, 0x96
140920960: jae 0x1409209b0
140920962: lea eax, [rdx + 1]
140920965: mov cl, 0x16
140920967: sub cl, dl
140920969: shr edi, cl
14092096b: mov ecx, eax
14092096d: shl r8d, cl
140920970: and r8d, 0x7fffff
140920977: imul rax, r8, 0x3b9aca00
14092097e: mov rcx, rax
140920981: shr rcx, 0x17
140920985: mov edx, eax
140920987: and edx, 0x7ffe00
14092098d: cmp edx, 0x400000
140920993: setne dl
140920996: shr eax, 0x16
140920999: and eax, 1
14092099c: mov r8d, ecx
14092099f: or r8b, dl
1409209a2: and r8b, al
1409209a5: movzx ebx, r8b
1409209a9: add ebx, ecx
1409209ab: jmp 0x140920082
1409209b0: cmp eax, 0xbf
1409209b5: jae 0x14092138d
1409209bb: mov edi, edi
1409209bd: lea ecx, [rdx + 0x2a]
1409209c0: shl rdi, cl
1409209c3: jmp 0x140920082
1409209c8: mov byte ptr [rbp + 0x327], 1
1409209cf: mov qword ptr [rsp + 0x20], 1
1409209d8: mov r8d, 8
1409209de: mov r9d, 1
1409209e4: mov rcx, qword ptr [rbp + 0x248]
1409209eb: mov rdx, r13
1409209ee: call 0x1428a9a60
1409209f3: jmp 0x14091fdbc
1409209f8: mov byte ptr [rbp + 0x327], 1
1409209ff: mov qword ptr [rsp + 0x20], 1
140920a08: mov r8d, 0x20
140920a0e: mov r9d, 1
140920a14: mov rcx, qword ptr [rbp + 0x248]
140920a1b: mov rdx, r13
140920a1e: call 0x1428a9a60
140920a23: jmp 0x14092009d
140920a28: cmp rdx, 0x15
140920a2c: jae 0x140920a78
140920a2e: mov byte ptr [rbp + 0x31e], 1
140920a35: mov r8d, 1
140920a3b: lea r9, [rbp + 0x260]
140920a42: call 0x1413395f0
140920a47: jmp 0x14091f5c7
140920a4c: mov byte ptr [rbp + 0x326], 1
140920a53: mov rcx, rdi
140920a56: xor edx, edx
140920a58: call 0x1428cdb90
140920a5d: jmp 0x14091fb71
140920a62: mov byte ptr [rbp + 0x325], 0
140920a69: mov rcx, rdi
140920a6c: xor edx, edx
140920a6e: call 0x1428cdb90
140920a73: jmp 0x1409204bc
140920a78: mov byte ptr [rbp + 0x31e], 1
140920a7f: lea r8, [rbp + 0x260]
140920a86: call 0x140b4cc10
140920a8b: jmp 0x14091f5c7
140920a90: mov eax, dword ptr [rbp + 0x318]
140920a96: mov rcx, qword ptr [rbp + 0x258]
140920a9d: mov dword ptr [rcx], eax
140920a9f: mov qword ptr [rbp + 0x268], 0x12c
140920aaa: movabs rax, 0x8000000000000003
140920ab4: mov qword ptr [rbp + 0x260], rax
140920abb: mov rax, qword ptr [rsi]
140920abe: mov rcx, qword ptr [rsi + 0x18]
140920ac2: cmp rcx, rax
140920ac5: jne 0x140920ae7
140920ac7: lea rdx, [rip + 0x218d33a]
140920ace: mov rcx, rsi
140920ad1: call 0x140b5a8f0
140920ad6: mov rax, qword ptr [rsi]
140920ad9: mov rcx, qword ptr [rsi + 0x18]
140920add: movabs rdi, 0x8000000000000000
140920ae7: mov rdx, qword ptr [rsi + 8]
140920aeb: mov r8, qword ptr [rsi + 0x10]
140920aef: dec r8
140920af2: add rax, r8
140920af5: cmovae rax, r8
140920af9: mov qword ptr [rsi + 0x10], rax
140920afd: inc rcx
140920b00: mov qword ptr [rsi + 0x18], rcx
140920b04: imul rax, rax, 0x38
140920b08: movups xmm0, xmmword ptr [rbp + 0x260]
140920b0f: movups xmm1, xmmword ptr [rbp + 0x270]
140920b16: movups xmm2, xmmword ptr [rbp + 0x280]
140920b1d: mov rcx, qword ptr [rbp + 0x290]
140920b24: mov qword ptr [rdx + rax + 0x30], rcx
140920b29: movups xmmword ptr [rdx + rax + 0x20], xmm2
140920b2e: movups xmmword ptr [rdx + rax + 0x10], xmm1
140920b33: movups xmmword ptr [rdx + rax], xmm0
140920b37: xor rdi, qword ptr [rbp + 0x1b0]
140920b3e: cmp rdi, 0x14
140920b42: mov eax, 0xd
140920b47: cmovb rax, rdi
140920b4b: cmp rax, 0xf
140920b4f: ja 0x140920b86
140920b51: mov ecx, 0xf101
140920b56: bt rcx, rax
140920b5a: jae 0x140920b86
140920b5c: movaps xmm6, xmmword ptr [rbp + 0x330]
140920b63: movaps xmm7, xmmword ptr [rbp + 0x340]
140920b6a: movaps xmm8, xmmword ptr [rbp + 0x350]
140920b72: add rsp, 0x3e8
140920b79: pop rbx
140920b7a: pop rdi
140920b7b: pop rsi
140920b7c: pop r12
140920b7e: pop r13
140920b80: pop r14
140920b82: pop r15
140920b84: pop rbp
140920b85: ret 
140920b86: lea rcx, [rbp + 0x1b0]
140920b8d: call 0x14091baf0
140920b92: jmp 0x140920b5c
140920b94: mov byte ptr [rbp + 0x327], 1
140920b9b: mov rcx, r14
140920b9e: call 0x140d76ed0
140920ba3: mov byte ptr [rbp + 0x327], 1
140920baa: mov rcx, rax
140920bad: call 0x140d61cd0
140920bb2: test rax, rax
140920bb5: je 0x14092139e
140920bbb: mov r8, rdx
140920bbe: mov byte ptr [rbp + 0x260], 1
140920bc5: mov byte ptr [rbp + 0x327], 1
140920bcc: lea rcx, [rbp + 0x260]
140920bd3: mov rdx, rax
140920bd6: call 0x140d5a150
140920bdb: test rax, rax
140920bde: jne 0x1409213bf
140920be4: mov eax, dword ptr [rbp + 0x318]
140920bea: mov rcx, qword ptr [rbp + 0x258]
140920bf1: mov dword ptr [rcx], eax
140920bf3: mov rax, qword ptr [rbp + 0x1e0]
140920bfa: mov qword ptr [rbp + 0x290], rax
140920c01: movups xmm0, xmmword ptr [rbp + 0x1b0]
140920c08: movups xmm1, xmmword ptr [rbp + 0x1c0]
140920c0f: movups xmm2, xmmword ptr [rbp + 0x1d0]
140920c16: movaps xmmword ptr [rbp + 0x280], xmm2
140920c1d: movaps xmmword ptr [rbp + 0x270], xmm1
140920c24: movaps xmmword ptr [rbp + 0x260], xmm0
140920c2b: mov rax, qword ptr [rsi]
140920c2e: mov rcx, qword ptr [rsi + 0x18]
140920c32: cmp rcx, rax
140920c35: jne 0x140920ce2
140920c3b: lea rdx, [rip + 0x218d1c6]
140920c42: mov rcx, rsi
140920c45: call 0x140b5a8f0
140920c4a: jmp 0x140920cdb
140920c4f: mov eax, dword ptr [rbp + 0x318]
140920c55: mov rcx, qword ptr [rbp + 0x258]
140920c5c: mov dword ptr [rcx], eax
140920c5e: mov rax, qword ptr [rbp + 0x1e0]
140920c65: mov qword ptr [rbp + 0x290], rax
140920c6c: mov rax, qword ptr [rbp + 0x1b0]
140920c73: mov rcx, qword ptr [rbp + 0x1b8]
140920c7a: mov rdx, qword ptr [rbp + 0x1c0]
140920c81: mov r8, qword ptr [rbp + 0x1c8]
140920c88: mov r9, qword ptr [rbp + 0x1d0]
140920c8f: mov qword ptr [rbp + 0x280], r9
140920c96: mov r9, qword ptr [rbp + 0x1d8]
140920c9d: mov qword ptr [rbp + 0x288], r9
140920ca4: mov qword ptr [rbp + 0x270], rdx
140920cab: mov qword ptr [rbp + 0x278], r8
140920cb2: mov qword ptr [rbp + 0x260], rax
140920cb9: mov qword ptr [rbp + 0x268], rcx
140920cc0: mov rax, qword ptr [rsi]
140920cc3: mov rcx, qword ptr [rsi + 0x18]
140920cc7: cmp rcx, rax
140920cca: jne 0x140920ce2
140920ccc: lea rdx, [rip + 0x218d135]
140920cd3: mov rcx, rsi
140920cd6: call 0x140b5a8f0
140920cdb: mov rax, qword ptr [rsi]
140920cde: mov rcx, qword ptr [rsi + 0x18]
140920ce2: mov rdx, qword ptr [rsi + 8]
140920ce6: mov r8, qword ptr [rsi + 0x10]
140920cea: dec r8
140920ced: add rax, r8
140920cf0: cmovae rax, r8
140920cf4: mov qword ptr [rsi + 0x10], rax
140920cf8: inc rcx
140920cfb: mov qword ptr [rsi + 0x18], rcx
140920cff: imul rax, rax, 0x38
140920d03: movaps xmm0, xmmword ptr [rbp + 0x260]
140920d0a: movaps xmm1, xmmword ptr [rbp + 0x270]
140920d11: movaps xmm2, xmmword ptr [rbp + 0x280]
140920d18: mov rcx, qword ptr [rbp + 0x290]
140920d1f: mov qword ptr [rdx + rax + 0x30], rcx
140920d24: movups xmmword ptr [rdx + rax + 0x20], xmm2
140920d29: movups xmmword ptr [rdx + rax + 0x10], xmm1
140920d2e: movups xmmword ptr [rdx + rax], xmm0
140920d32: jmp 0x140920b5c
140920d37: mov eax, dword ptr [rbp + 0x318]
140920d3d: mov rcx, qword ptr [rbp + 0x258]
140920d44: mov dword ptr [rcx], eax
140920d46: dec r8
140920d49: mov qword ptr [rbp + 0x268], r8
140920d50: movabs rax, 0x8000000000000005
140920d5a: mov qword ptr [rbp + 0x260], rax
140920d61: mov rax, qword ptr [rsi]
140920d64: mov rcx, qword ptr [rsi + 0x18]
140920d68: cmp rcx, rax
140920d6b: jne 0x140920ae7
140920d71: lea rdx, [rip + 0x218d090]
140920d78: mov rcx, rsi
140920d7b: call 0x140b5a8f0
140920d80: jmp 0x140920ad6
140920d85: mov rax, qword ptr [rbp + 0x48]
140920d89: mov rax, qword ptr [rax]
140920d8c: mov eax, dword ptr [rax]
140920d8e: lea ecx, [rax - 6]
140920d91: cmp ecx, 0xf
140920d94: setb cl
140920d97: mov eax, eax
140920d99: add rax, -0xd
140920d9d: cmp rax, 2
140920da1: setb al
140920da4: test cl, al
140920da6: je 0x140920f05
140920dac: mov eax, dword ptr [rbp + 0x318]
140920db2: mov rcx, qword ptr [rbp + 0x258]
140920db9: mov dword ptr [rcx], eax
140920dbb: mov qword ptr [rbp + 0x268], r15
140920dc2: mov qword ptr [rbp + 0x270], r12
140920dc9: mov byte ptr [rbp + 0x280], r13b
140920dd0: mov qword ptr [rbp + 0x278], 0
140920ddb: mov qword ptr [rbp + 0x260], rdi
140920de2: lea r8, [rip + 0x218d01f]
140920de9: lea rdx, [rbp + 0x260]
140920df0: mov rcx, rsi
140920df3: call 0x140b5a7a0
140920df8: jmp 0x14092100c
140920dfd: mov eax, dword ptr [rbp + 0x318]
140920e03: mov rcx, qword ptr [rbp + 0x258]
140920e0a: mov dword ptr [rcx], eax
140920e0c: mov rax, qword ptr [rbp + 0x90]
140920e13: mov qword ptr [rbp + 0x268], r15
140920e1a: mov qword ptr [rbp + 0x270], r12
140920e21: mov byte ptr [rbp + 0x280], r13b
140920e28: mov qword ptr [rbp + 0x278], rax
140920e2f: mov qword ptr [rbp + 0x260], rdi
140920e36: mov rax, qword ptr [rsi]
140920e39: mov rcx, qword ptr [rsi + 0x18]
140920e3d: cmp rcx, rax
140920e40: jne 0x140920e58
140920e42: lea rdx, [rip + 0x218cfbf]
140920e49: mov rcx, rsi
140920e4c: call 0x140b5a8f0
140920e51: mov rax, qword ptr [rsi]
140920e54: mov rcx, qword ptr [rsi + 0x18]
140920e58: mov rdx, qword ptr [rsi + 8]
140920e5c: mov r8, qword ptr [rsi + 0x10]
140920e60: dec r8
140920e63: add rax, r8
140920e66: cmovae rax, r8
140920e6a: mov qword ptr [rsi + 0x10], rax
140920e6e: inc rcx
140920e71: mov qword ptr [rsi + 0x18], rcx
140920e75: imul rax, rax, 0x38
140920e79: movups xmm0, xmmword ptr [rbp + 0x260]
140920e80: movups xmm1, xmmword ptr [rbp + 0x270]
140920e87: movups xmm2, xmmword ptr [rbp + 0x280]
140920e8e: mov rcx, qword ptr [rbp + 0x290]
140920e95: mov qword ptr [rdx + rax + 0x30], rcx
140920e9a: movups xmmword ptr [rdx + rax + 0x20], xmm2
140920e9f: movups xmmword ptr [rdx + rax + 0x10], xmm1
140920ea4: movups xmmword ptr [rdx + rax], xmm0
140920ea8: movabs rdi, 0x8000000000000000
140920eb2: jmp 0x140920b37
140920eb7: mov eax, dword ptr [rbp + 0x318]
140920ebd: mov rcx, qword ptr [rbp + 0x258]
140920ec4: mov dword ptr [rcx], eax
140920ec6: dec r13
140920ec9: mov qword ptr [rbp + 0x268], r13
140920ed0: movabs rax, 0x8000000000000003
140920eda: mov qword ptr [rbp + 0x260], rax
140920ee1: mov rax, qword ptr [rsi]
140920ee4: mov rcx, qword ptr [rsi + 0x18]
140920ee8: cmp rcx, rax
140920eeb: jne 0x140920ae7
140920ef1: lea rdx, [rip + 0x218cf10]
140920ef8: mov rcx, rsi
140920efb: call 0x140b5a8f0
140920f00: jmp 0x140920ad6
140920f05: call 0x14091ef90
140920f0a: mov r8, rdx
140920f0d: mov qword ptr [rbp + 0x160], rax
140920f14: mov rdx, qword ptr [rbp + 0x3f0]
140920f1b: mov eax, dword ptr [rdx + 0x1c]
140920f1e: mov rcx, qword ptr [rdx + 0x10]
140920f22: mov dword ptr [rcx], eax
140920f24: mov rax, qword ptr [rdx]
140920f27: mov byte ptr [rax], 2
140920f2a: mov qword ptr [rbp + 0x260], 0
140920f35: mov qword ptr [rbp + 0x268], 8
140920f40: mov qword ptr [rbp + 0x270], 0
140920f4b: movabs rax, 0x8000000000000002
140920f55: mov qword ptr [rbp + 0x278], rax
140920f5c: mov qword ptr [rbp + 0x280], r15
140920f63: mov qword ptr [rbp + 0x288], r12
140920f6a: mov byte ptr [rbp + 0x2f8], r13b
140920f71: mov byte ptr [rbp + 0x320], 1
140920f78: lea rcx, [rbp + 0x90]
140920f7f: lea rdx, [rbp + 0x260]
140920f86: call 0x141328ac0
140920f8b: mov byte ptr [rbp + 0x320], 1
140920f92: lea r8, [rip + 0x218ce87]
140920f99: lea rdx, [rbp + 0x90]
140920fa0: mov rcx, qword ptr [rbp + 0x410]
140920fa7: call 0x1412e0b90
140920fac: mov eax, dword ptr [rbp + 0x318]
140920fb2: mov rcx, qword ptr [rbp + 0x258]
140920fb9: mov dword ptr [rcx], eax
140920fbb: mov rax, qword ptr [rbp + 0x160]
140920fc2: mov qword ptr [rbp + 0x268], r15
140920fc9: mov qword ptr [rbp + 0x270], r12
140920fd0: mov byte ptr [rbp + 0x280], r13b
140920fd7: mov qword ptr [rbp + 0x278], rax
140920fde: movabs rax, 0x8000000000000000
140920fe8: mov qword ptr [rbp + 0x260], rax
140920fef: mov byte ptr [rbp + 0x320], 0
140920ff6: lea r8, [rip + 0x218ce0b]
140920ffd: lea rdx, [rbp + 0x260]
140921004: mov rcx, rsi
140921007: call 0x140b5a7a0
14092100c: mov rax, qword ptr [rbp + 0x210]
140921013: test rax, rax
140921016: je 0x140921031
140921018: lock dec qword ptr [rax]
14092101c: jne 0x140921031
14092101e: mov byte ptr [rbp + 0x327], 1
140921025: lea rcx, [rbp + 0x210]
14092102c: call 0x1402e5d80
140921031: movabs rdi, 0x8000000000000000
14092103b: jmp 0x140920b37
140921040: mov eax, 0x3b
140921045: lea rcx, [rip + 0x218c594]
14092104c: mov qword ptr [rbp + 0x260], rcx
140921053: mov qword ptr [rbp + 0x268], rax
14092105a: mov byte ptr [rbp + 0x327], 1
140921061: lea rdx, [rip + 0x218c648]
140921068: lea rcx, [rbp + 0x260]
14092106f: call 0x1428ce090
140921074: jmp 0x1409213f1
140921079: mov byte ptr [rbp + 0x327], 1
140921080: mov ecx, r8d
140921083: call 0x140d63150
140921088: jmp 0x1409213f1
14092108d: mov rsi, rax
140921090: mov rdx, qword ptr [rbp + 0x210]
140921097: test rdx, rdx
14092109a: je 0x1409210ea
14092109c: mov rcx, qword ptr [rbp + 0x218]
1409210a3: mov r8d, 1
1409210a9: call 0x140613c20
1409210ae: jmp 0x1409210ea
1409210b0: mov qword ptr [rbp + 0x90], rax
1409210b7: lea rax, [rip + 0x218d05a]
1409210be: mov qword ptr [rsp + 0x20], rax
1409210c3: lea rcx, [rip + 0x218c68e]
1409210ca: lea r9, [rip + 0x218c627]
1409210d1: lea r8, [rbp + 0x90]
1409210d8: mov edx, 0x2b
1409210dd: call 0x1428d9760
1409210e2: jmp 0x1409213f1
1409210e7: mov rsi, rax
1409210ea: mov qword ptr [rbp + 0x90], rsi
1409210f1: lea rax, [rip + 0x218d008]
1409210f8: mov qword ptr [rsp + 0x20], rax
1409210fd: lea rcx, [rip + 0x218c654]
140921104: lea r9, [rip + 0x218c6bd]
14092110b: lea r8, [rbp + 0x90]
140921112: mov edx, 0x2b
140921117: call 0x1428d9760
14092111c: jmp 0x1409213f1
140921121: mov byte ptr [rbp + 0x321], 1
140921128: lea r8, [rip + 0x218c3d9]
14092112f: mov ecx, 1
140921134: mov edx, 0x80
140921139: call 0x1428d8fe3
14092113e: jmp 0x1409213f1
140921143: mov byte ptr [rbp + 0x322], 1
14092114a: lea rcx, [rip + 0x218cf47]
140921151: lea r8, [rip + 0x218cf90]
140921158: mov edx, 0x4f
14092115d: call 0x1428d9330
140921162: jmp 0x1409213f1
140921167: lea rcx, [rip + 0x218ce72]
14092116e: call 0x1428d9310
140921173: jmp 0x1409213f1
140921178: lea rax, [rbp + 0x1f0]
14092117f: mov qword ptr [rbp + 0x90], rax
140921186: lea rax, [rip - 0x8e56bd]
14092118d: mov qword ptr [rbp + 0x98], rax
140921194: lea rax, [rip + 0x218cfdd]
14092119b: mov qword ptr [rbp + 0x260], rax
1409211a2: mov qword ptr [rbp + 0x268], 1
1409211ad: mov qword ptr [rbp + 0x280], 0
1409211b8: lea rax, [rbp + 0x90]
1409211bf: mov qword ptr [rbp + 0x270], rax
1409211c6: mov qword ptr [rbp + 0x278], 1
1409211d1: lea rdx, [rip + 0x218cfb0]
1409211d8: lea rcx, [rbp + 0x260]
1409211df: call 0x1428d9390
1409211e4: jmp 0x1409213f1
1409211e9: mov qword ptr [rbp + 0x230], r15
1409211f0: mov ecx, 8
1409211f5: mov edx, 8
1409211fa: call 0x1428d9000
1409211ff: jmp 0x1409213f1
140921204: mov ecx, 8
140921209: mov edx, 0x68
14092120e: call 0x1428d9000
140921213: jmp 0x1409213f1
140921218: mov byte ptr [rbp + 0x326], 0
14092121f: lea rcx, [rip + 0x218cdd2]
140921226: call 0x1428d9310
14092122b: jmp 0x1409213f1
140921230: movaps xmm0, xmmword ptr [rbp + 0x1a0]
140921237: movaps xmmword ptr [rbp + 0x2a0], xmm0
14092123e: movaps xmm0, xmmword ptr [rbp + 0x160]
140921245: movaps xmm1, xmmword ptr [rbp + 0x170]
14092124c: movaps xmm2, xmmword ptr [rbp + 0x180]
140921253: movaps xmm3, xmmword ptr [rbp + 0x190]
14092125a: movaps xmmword ptr [rbp + 0x290], xmm3
140921261: movaps xmmword ptr [rbp + 0x280], xmm2
140921268: movaps xmmword ptr [rbp + 0x270], xmm1
14092126f: movaps xmmword ptr [rbp + 0x260], xmm0
140921276: lea rax, [rip + 0x218cd93]
14092127d: mov qword ptr [rsp + 0x20], rax
140921282: lea rcx, [rip + 0x218c4cf]
140921289: lea r9, [rip + 0x218c4f8]
140921290: lea r8, [rbp + 0x260]
140921297: mov edx, 0x2b
14092129c: call 0x1428d9760
1409212a1: jmp 0x1409213f1
1409212a6: mov qword ptr [rbp + 0x160], rax
1409212ad: lea rax, [rip + 0x218cd8c]
1409212b4: mov qword ptr [rsp + 0x20], rax
1409212b9: lea rcx, [rip + 0x218cd68]
1409212c0: lea r9, [rip + 0x218c411]
1409212c7: lea r8, [rbp + 0x160]
1409212ce: mov edx, 0x17
1409212d3: call 0x1428d9760
1409212d8: jmp 0x1409213f1
1409212dd: mov qword ptr [rbp + 0x230], rdi
1409212e4: lea r8, [rip + 0x2365235]
1409212eb: mov ecx, 1
1409212f0: mov edx, 0x2000
1409212f5: call 0x1428d8fe3
1409212fa: jmp 0x1409213f1
1409212ff: mov qword ptr [rbp + 0x240], rdi
140921306: lea rax, [rbp + 0x58]
14092130a: mov qword ptr [rbp + 0x160], rax
140921311: lea rax, [rip - 0x8f2a28]
140921318: mov qword ptr [rbp + 0x168], rax
14092131f: lea rax, [rbp + 0x240]
140921326: mov qword ptr [rbp + 0x170], rax
14092132d: lea rax, [rip + 0x4399ac]
140921334: mov qword ptr [rbp + 0x178], rax
14092133b: lea rax, [rip + 0x218ce76]
140921342: mov qword ptr [rbp + 0x90], rax
140921349: mov qword ptr [rbp + 0x98], 2
140921354: mov qword ptr [rbp + 0xb0], 0
14092135f: lea rax, [rbp + 0x160]
140921366: mov qword ptr [rbp + 0xa0], rax
14092136d: mov qword ptr [rbp + 0xa8], 2
140921378: lea rdx, [rip + 0x218ce59]
14092137f: lea rcx, [rbp + 0x90]
140921386: call 0x1428d9390
14092138b: jmp 0x1409213f1
14092138d: mov eax, 0x48
140921392: lea rcx, [rip + 0x218c282]
140921399: jmp 0x14092104c
14092139e: mov byte ptr [rbp + 0x327], 1
1409213a5: lea rcx, [rip + 0x218cb14]
1409213ac: lea r8, [rip + 0x218cb25]
1409213b3: mov edx, 0x18
1409213b8: call 0x1428d9330
1409213bd: jmp 0x1409213f1
1409213bf: mov qword ptr [rbp + 0x260], rax
1409213c6: lea rax, [rip + 0x218cb3b]
1409213cd: mov qword ptr [rsp + 0x20], rax
1409213d2: lea rcx, [rip + 0x218cb17]
1409213d9: lea r9, [rip + 0x218c318]
1409213e0: lea r8, [rbp + 0x260]
1409213e7: mov edx, 0x18
1409213ec: call 0x1428d9760
1409213f1: ud2 
1409213f3: nop word ptr cs:[rax + rax]
140921400: mov qword ptr [rsp + 0x10], rdx
140921405: push rbp
140921406: push r15
140921408: push r14
14092140a: push r13
14092140c: push r12
14092140e: push rsi
14092140f: push rdi
140921410: push rbx
140921411: sub rsp, 0x58
140921415: lea rbp, [rdx + 0x80]
14092141c: movaps xmmword ptr [rsp + 0x20], xmm8
140921422: movaps xmmword ptr [rsp + 0x30], xmm7
140921427: movaps xmmword ptr [rsp + 0x40], xmm6
14092142c: mov byte ptr [rbp + 0x327], 1
140921433: lea rcx, [rbp + 0x260]
14092143a: call 0x140b5a520
14092143f: mov byte ptr [rbp + 0x327], 1
140921446: movaps xmm6, xmmword ptr [rsp + 0x40]
14092144b: movaps xmm7, xmmword ptr [rsp + 0x30]
140921450: movaps xmm8, xmmword ptr [rsp + 0x20]
140921456: add rsp, 0x58
14092145a: pop rbx
14092145b: pop rdi
14092145c: pop rsi
14092145d: pop r12
14092145f: pop r13
140921461: pop r14
140921463: pop r15
140921465: pop rbp
140921466: ret 
140921467: nop word ptr [rax + rax]
140921470: mov qword ptr [rsp + 0x10], rdx
140921475: push rbp
140921476: push r15
140921478: push r14
14092147a: push r13
14092147c: push r12
14092147e: push rsi
14092147f: push rdi
140921480: push rbx
140921481: sub rsp, 0x58
140921485: lea rbp, [rdx + 0x80]
14092148c: movaps xmmword ptr [rsp + 0x20], xmm8
140921492: movaps xmmword ptr [rsp + 0x30], xmm7
140921497: movaps xmmword ptr [rsp + 0x40], xmm6
14092149c: lea rcx, [rbp + 0x260]
1409214a3: call 0x140b5a520
1409214a8: movaps xmm6, xmmword ptr [rsp + 0x40]
1409214ad: movaps xmm7, xmmword ptr [rsp + 0x30]
1409214b2: movaps xmm8, xmmword ptr [rsp + 0x20]
1409214b8: add rsp, 0x58
1409214bc: pop rbx
1409214bd: pop rdi
1409214be: pop rsi
1409214bf: pop r12
1409214c1: pop r13
1409214c3: pop r14
1409214c5: pop r15
1409214c7: pop rbp
1409214c8: ret 
1409214c9: nop dword ptr [rax]
1409214d0: mov qword ptr [rsp + 0x10], rdx
1409214d5: push rbp
1409214d6: push r15
1409214d8: push r14
1409214da: push r13
1409214dc: push r12
1409214de: push rsi
1409214df: push rdi
1409214e0: push rbx
1409214e1: sub rsp, 0x58
1409214e5: lea rbp, [rdx + 0x80]
1409214ec: movaps xmmword ptr [rsp + 0x20], xmm8
1409214f2: movaps xmmword ptr [rsp + 0x30], xmm7
1409214f7: movaps xmmword ptr [rsp + 0x40], xmm6
1409214fc: mov byte ptr [rbp + 0x327], 1
140921503: lea rcx, [rbp + 0x260]
14092150a: call 0x140b5a520
14092150f: mov byte ptr [rbp + 0x327], 1
140921516: movaps xmm6, xmmword ptr [rsp + 0x40]
14092151b: movaps xmm7, xmmword ptr [rsp + 0x30]
140921520: movaps xmm8, xmmword ptr [rsp + 0x20]
140921526: add rsp, 0x58
14092152a: pop rbx
14092152b: pop rdi
14092152c: pop rsi
14092152d: pop r12
14092152f: pop r13
140921531: pop r14
140921533: pop r15
140921535: pop rbp
140921536: ret 
140921537: nop word ptr [rax + rax]
140921540: mov qword ptr [rsp + 0x10], rdx
140921545: push rbp
140921546: push r15
140921548: push r14
14092154a: push r13
14092154c: push r12
14092154e: push rsi
14092154f: push rdi
140921550: push rbx
140921551: sub rsp, 0x58
140921555: lea rbp, [rdx + 0x80]
14092155c: movaps xmmword ptr [rsp + 0x20], xmm8
140921562: movaps xmmword ptr [rsp + 0x30], xmm7
140921567: movaps xmmword ptr [rsp + 0x40], xmm6
14092156c: mov byte ptr [rbp + 0x327], 0
140921573: lea rcx, [rbp + 0x260]
14092157a: call 0x140b5a520
14092157f: mov byte ptr [rbp + 0x327], 0
140921586: movaps xmm6, xmmword ptr [rsp + 0x40]
14092158b: movaps xmm7, xmmword ptr [rsp + 0x30]
140921590: movaps xmm8, xmmword ptr [rsp + 0x20]
140921596: add rsp, 0x58
14092159a: pop rbx
14092159b: pop rdi
14092159c: pop rsi
14092159d: pop r12
14092159f: pop r13
1409215a1: pop r14
1409215a3: pop r15
1409215a5: pop rbp
1409215a6: ret 
1409215a7: nop word ptr [rax + rax]
1409215b0: mov qword ptr [rsp + 0x10], rdx
1409215b5: push rbp
1409215b6: push r15
1409215b8: push r14
1409215ba: push r13
1409215bc: push r12
1409215be: push rsi
1409215bf: push rdi
1409215c0: push rbx
1409215c1: sub rsp, 0x58
1409215c5: lea rbp, [rdx + 0x80]
1409215cc: movaps xmmword ptr [rsp + 0x20], xmm8
1409215d2: movaps xmmword ptr [rsp + 0x30], xmm7
1409215d7: movaps xmmword ptr [rsp + 0x40], xmm6
1409215dc: mov byte ptr [rbp + 0x327], 0
1409215e3: lea rcx, [rbp + 0x260]
1409215ea: call 0x140b5a520
1409215ef: mov byte ptr [rbp + 0x327], 0
1409215f6: movaps xmm6, xmmword ptr [rsp + 0x40]
1409215fb: movaps xmm7, xmmword ptr [rsp + 0x30]
140921600: movaps xmm8, xmmword ptr [rsp + 0x20]
140921606: add rsp, 0x58
14092160a: pop rbx
14092160b: pop rdi
14092160c: pop rsi
14092160d: pop r12
14092160f: pop r13
140921611: pop r14
140921613: pop r15
140921615: pop rbp
140921616: ret 
140921617: nop word ptr [rax + rax]
140921620: mov qword ptr [rsp + 0x10], rdx
140921625: push rbp
140921626: push r15
140921628: push r14
14092162a: push r13
14092162c: push r12
14092162e: push rsi
14092162f: push rdi
140921630: push rbx
140921631: sub rsp, 0x58
140921635: lea rbp, [rdx + 0x80]
14092163c: movaps xmmword ptr [rsp + 0x20], xmm8
140921642: movaps xmmword ptr [rsp + 0x30], xmm7
140921647: movaps xmmword ptr [rsp + 0x40], xmm6
14092164c: mov byte ptr [rbp + 0x327], 1
140921653: lea rcx, [rbp + 0x260]
14092165a: call 0x1405e7a00
14092165f: mov byte ptr [rbp + 0x327], 1
140921666: movaps xmm6, xmmword ptr [rsp + 0x40]
14092166b: movaps xmm7, xmmword ptr [rsp + 0x30]
140921670: movaps xmm8, xmmword ptr [rsp + 0x20]
140921676: add rsp, 0x58
14092167a: pop rbx
14092167b: pop rdi
14092167c: pop rsi
14092167d: pop r12
14092167f: pop r13
140921681: pop r14
140921683: pop r15
140921685: pop rbp
140921686: ret 
140921687: nop word ptr [rax + rax]
140921690: mov qword ptr [rsp + 0x10], rdx
140921695: push rbp
140921696: push r15
140921698: push r14
14092169a: push r13
14092169c: push r12
14092169e: push rsi
14092169f: push rdi
1409216a0: push rbx
1409216a1: sub rsp, 0x58
1409216a5: lea rbp, [rdx + 0x80]
1409216ac: movaps xmmword ptr [rsp + 0x20], xmm8
1409216b2: movaps xmmword ptr [rsp + 0x30], xmm7
1409216b7: movaps xmmword ptr [rsp + 0x40], xmm6
1409216bc: cmp byte ptr [rbp + 0x320], 0
1409216c3: je 0x1409216ff
1409216c5: mov rax, qword ptr [rbp + 0x160]
1409216cc: lock dec qword ptr [rax]
1409216d0: jne 0x1409216de
1409216d2: lea rcx, [rbp + 0x160]
1409216d9: call 0x1402e5d80
1409216de: movaps xmm6, xmmword ptr [rsp + 0x40]
1409216e3: movaps xmm7, xmmword ptr [rsp + 0x30]
1409216e8: movaps xmm8, xmmword ptr [rsp + 0x20]
1409216ee: add rsp, 0x58
1409216f2: pop rbx
1409216f3: pop rdi
1409216f4: pop rsi
1409216f5: pop r12
1409216f7: pop r13
1409216f9: pop r14
1409216fb: pop r15
1409216fd: pop rbp
1409216fe: ret 
1409216ff: movaps xmm6, xmmword ptr [rsp + 0x40]
140921704: movaps xmm7, xmmword ptr [rsp + 0x30]
140921709: movaps xmm8, xmmword ptr [rsp + 0x20]
14092170f: add rsp, 0x58
140921713: pop rbx
140921714: pop rdi
140921715: pop rsi
140921716: pop r12
140921718: pop r13
14092171a: pop r14
14092171c: pop r15
14092171e: pop rbp
14092171f: ret 
140921720: mov qword ptr [rsp + 0x10], rdx
140921725: push rbp
140921726: push r15
140921728: push r14
14092172a: push r13
14092172c: push r12
14092172e: push rsi
14092172f: push rdi
140921730: push rbx
140921731: sub rsp, 0x58
140921735: lea rbp, [rdx + 0x80]
14092173c: movaps xmmword ptr [rsp + 0x20], xmm8
140921742: movaps xmmword ptr [rsp + 0x30], xmm7
140921747: movaps xmmword ptr [rsp + 0x40], xmm6
14092174c: mov rcx, qword ptr [rbp + 0x310]
140921753: mov rax, qword ptr [rbp + 0x308]
14092175a: mov rdx, qword ptr [rax + 8]
14092175e: test rdx, rdx
140921761: je 0x140921773
140921763: mov rax, qword ptr [rbp + 0x308]
14092176a: mov r8, qword ptr [rax + 0x10]
14092176e: call 0x140613c20
140921773: mov edx, 0x18
140921778: mov r8d, 8
14092177e: mov rcx, qword ptr [rbp + 0x250]
140921785: call 0x140613c20
14092178a: mov byte ptr [rbp + 0x327], 1
140921791: movaps xmm6, xmmword ptr [rsp + 0x40]
140921796: movaps xmm7, xmmword ptr [rsp + 0x30]
14092179b: movaps xmm8, xmmword ptr [rsp + 0x20]
1409217a1: add rsp, 0x58
1409217a5: pop rbx
1409217a6: pop rdi
1409217a7: pop rsi
1409217a8: pop r12
1409217aa: pop r13
1409217ac: pop r14
1409217ae: pop r15
1409217b0: pop rbp
1409217b1: ret 
1409217b2: nop word ptr cs:[rax + rax]
1409217c0: mov qword ptr [rsp + 0x10], rdx
1409217c5: push rbp
1409217c6: push r15
1409217c8: push r14
1409217ca: push r13
1409217cc: push r12
1409217ce: push rsi
1409217cf: push rdi
1409217d0: push rbx
1409217d1: sub rsp, 0x58
1409217d5: lea rbp, [rdx + 0x80]
1409217dc: movaps xmmword ptr [rsp + 0x20], xmm8
1409217e2: movaps xmmword ptr [rsp + 0x30], xmm7
1409217e7: movaps xmmword ptr [rsp + 0x40], xmm6
1409217ec: mov byte ptr [rbp + 0x327], 1
1409217f3: lea rcx, [rbp + 0x260]
1409217fa: call 0x140b5a520
1409217ff: mov byte ptr [rbp + 0x327], 1
140921806: movaps xmm6, xmmword ptr [rsp + 0x40]
14092180b: movaps xmm7, xmmword ptr [rsp + 0x30]
140921810: movaps xmm8, xmmword ptr [rsp + 0x20]
140921816: add rsp, 0x58
14092181a: pop rbx
14092181b: pop rdi
14092181c: pop rsi
14092181d: pop r12
14092181f: pop r13
140921821: pop r14
140921823: pop r15
140921825: pop rbp
140921826: ret 
140921827: nop word ptr [rax + rax]
140921830: mov qword ptr [rsp + 0x10], rdx
140921835: push rbp
140921836: push r15
140921838: push r14
14092183a: push r13
14092183c: push r12
14092183e: push rsi
14092183f: push rdi
140921840: push rbx
140921841: sub rsp, 0x58
140921845: lea rbp, [rdx + 0x80]
14092184c: movaps xmmword ptr [rsp + 0x20], xmm8
140921852: movaps xmmword ptr [rsp + 0x30], xmm7
140921857: movaps xmmword ptr [rsp + 0x40], xmm6
14092185c: mov rax, qword ptr [rbp + 0x310]
140921863: lea rax, [rax*2]
14092186b: test rax, rax
14092186e: je 0x140921889
140921870: mov r8d, 1
140921876: mov rcx, qword ptr [rbp + 0x308]
14092187d: mov rdx, qword ptr [rbp + 0x310]
140921884: call 0x140613c20
140921889: mov byte ptr [rbp + 0x327], 1
140921890: movaps xmm6, xmmword ptr [rsp + 0x40]
140921895: movaps xmm7, xmmword ptr [rsp + 0x30]
14092189a: movaps xmm8, xmmword ptr [rsp + 0x20]
1409218a0: add rsp, 0x58
1409218a4: pop rbx
1409218a5: pop rdi
1409218a6: pop rsi
1409218a7: pop r12
1409218a9: pop r13
1409218ab: pop r14
1409218ad: pop r15
1409218af: pop rbp
1409218b0: ret 
1409218b1: nop word ptr cs:[rax + rax]
1409218c0: mov qword ptr [rsp + 0x10], rdx
1409218c5: push rbp
1409218c6: push r15
1409218c8: push r14
1409218ca: push r13
1409218cc: push r12
1409218ce: push rsi
1409218cf: push rdi
1409218d0: push rbx
1409218d1: sub rsp, 0x58
1409218d5: lea rbp, [rdx + 0x80]
1409218dc: movaps xmmword ptr [rsp + 0x20], xmm8
1409218e2: movaps xmmword ptr [rsp + 0x30], xmm7
1409218e7: movaps xmmword ptr [rsp + 0x40], xmm6
1409218ec: lea rcx, [rbp + 0x240]
1409218f3: call 0x1405e7a00
1409218f8: movaps xmm6, xmmword ptr [rsp + 0x40]
1409218fd: movaps xmm7, xmmword ptr [rsp + 0x30]
140921902: movaps xmm8, xmmword ptr [rsp + 0x20]
140921908: add rsp, 0x58
14092190c: pop rbx
14092190d: pop rdi
14092190e: pop rsi
14092190f: pop r12
140921911: pop r13
140921913: pop r14
140921915: pop r15
140921917: pop rbp
140921918: ret 
140921919: nop dword ptr [rax]
140921920: mov qword ptr [rsp + 0x10], rdx
140921925: push rbp
140921926: push r15
140921928: push r14
14092192a: push r13
14092192c: push r12
14092192e: push rsi
14092192f: push rdi
140921930: push rbx
140921931: sub rsp, 0x58
140921935: lea rbp, [rdx + 0x80]
14092193c: movaps xmmword ptr [rsp + 0x20], xmm8
140921942: movaps xmmword ptr [rsp + 0x30], xmm7
140921947: movaps xmmword ptr [rsp + 0x40], xmm6
14092194c: mov rcx, qword ptr [rbp + 0x230]
140921953: call qword ptr [rip + 0x2870a97]
140921959: movaps xmm6, xmmword ptr [rsp + 0x40]
14092195e: movaps xmm7, xmmword ptr [rsp + 0x30]
140921963: movaps xmm8, xmmword ptr [rsp + 0x20]
140921969: add rsp, 0x58
14092196d: pop rbx
14092196e: pop rdi
14092196f: pop rsi
140921970: pop r12
140921972: pop r13
140921974: pop r14
140921976: pop r15
140921978: pop rbp
140921979: ret 
14092197a: nop word ptr [rax + rax]
140921980: mov qword ptr [rsp + 0x10], rdx
140921985: push rbp
140921986: push r15
140921988: push r14
14092198a: push r13
14092198c: push r12
14092198e: push rsi
14092198f: push rdi
140921990: push rbx
140921991: sub rsp, 0x58
140921995: lea rbp, [rdx + 0x80]
14092199c: movaps xmmword ptr [rsp + 0x20], xmm8
1409219a2: movaps xmmword ptr [rsp + 0x30], xmm7
1409219a7: movaps xmmword ptr [rsp + 0x40], xmm6
1409219ac: lea rcx, [rbp + 0x160]
1409219b3: call 0x140d4cc30
1409219b8: movaps xmm6, xmmword ptr [rsp + 0x40]
1409219bd: movaps xmm7, xmmword ptr [rsp + 0x30]
1409219c2: movaps xmm8, xmmword ptr [rsp + 0x20]
1409219c8: add rsp, 0x58
1409219cc: pop rbx
1409219cd: pop rdi
1409219ce: pop rsi
1409219cf: pop r12
1409219d1: pop r13
1409219d3: pop r14
1409219d5: pop r15
1409219d7: pop rbp
1409219d8: ret 
1409219d9: nop dword ptr [rax]
1409219e0: mov qword ptr [rsp + 0x10], rdx
1409219e5: push rbp
1409219e6: push r15
1409219e8: push r14
1409219ea: push r13
1409219ec: push r12
1409219ee: push rsi
1409219ef: push rdi
1409219f0: push rbx
1409219f1: sub rsp, 0x58
1409219f5: lea rbp, [rdx + 0x80]
1409219fc: movaps xmmword ptr [rsp + 0x20], xmm8
140921a02: movaps xmmword ptr [rsp + 0x30], xmm7
140921a07: movaps xmmword ptr [rsp + 0x40], xmm6
140921a0c: mov byte ptr [rbp + 0x326], 0
140921a13: lea rcx, [rbp + 0x260]
140921a1a: call 0x140888890
140921a1f: mov byte ptr [rbp + 0x326], 0
140921a26: movaps xmm6, xmmword ptr [rsp + 0x40]
140921a2b: movaps xmm7, xmmword ptr [rsp + 0x30]
140921a30: movaps xmm8, xmmword ptr [rsp + 0x20]
140921a36: add rsp, 0x58
140921a3a: pop rbx
140921a3b: pop rdi
140921a3c: pop rsi
140921a3d: pop r12
140921a3f: pop r13
140921a41: pop r14
140921a43: pop r15
140921a45: pop rbp
140921a46: ret 
140921a47: nop word ptr [rax + rax]
140921a50: mov qword ptr [rsp + 0x10], rdx
140921a55: push rbp
140921a56: push r15
140921a58: push r14
140921a5a: push r13
140921a5c: push r12
140921a5e: push rsi
140921a5f: push rdi
140921a60: push rbx
140921a61: sub rsp, 0x58
140921a65: lea rbp, [rdx + 0x80]
140921a6c: movaps xmmword ptr [rsp + 0x20], xmm8
140921a72: movaps xmmword ptr [rsp + 0x30], xmm7
140921a77: movaps xmmword ptr [rsp + 0x40], xmm6
140921a7c: mov rax, qword ptr [rbp + 0x210]
140921a83: test rax, rax
140921a86: jne 0x140921ab0
140921a88: mov byte ptr [rbp + 0x327], 1
140921a8f: movaps xmm6, xmmword ptr [rsp + 0x40]
140921a94: movaps xmm7, xmmword ptr [rsp + 0x30]
140921a99: movaps xmm8, xmmword ptr [rsp + 0x20]
140921a9f: add rsp, 0x58
140921aa3: pop rbx
140921aa4: pop rdi
140921aa5: pop rsi
140921aa6: pop r12
140921aa8: pop r13
140921aaa: pop r14
140921aac: pop r15
140921aae: pop rbp
140921aaf: ret 
140921ab0: cmp qword ptr [rbp + 0x50], 0
140921ab5: jne 0x140921af1
140921ab7: lock dec qword ptr [rax]
140921abb: jne 0x140921ac9
140921abd: lea rcx, [rbp + 0x210]
140921ac4: call 0x1402e5d80
140921ac9: mov byte ptr [rbp + 0x327], 1
140921ad0: movaps xmm6, xmmword ptr [rsp + 0x40]
140921ad5: movaps xmm7, xmmword ptr [rsp + 0x30]
140921ada: movaps xmm8, xmmword ptr [rsp + 0x20]
140921ae0: add rsp, 0x58
140921ae4: pop rbx
140921ae5: pop rdi
140921ae6: pop rsi
140921ae7: pop r12
140921ae9: pop r13
140921aeb: pop r14
140921aed: pop r15
140921aef: pop rbp
140921af0: ret 
140921af1: mov byte ptr [rbp + 0x327], 1
140921af8: movaps xmm6, xmmword ptr [rsp + 0x40]
140921afd: movaps xmm7, xmmword ptr [rsp + 0x30]
140921b02: movaps xmm8, xmmword ptr [rsp + 0x20]
140921b08: add rsp, 0x58
140921b0c: pop rbx
140921b0d: pop rdi
140921b0e: pop rsi
140921b0f: pop r12
140921b11: pop r13
140921b13: pop r14
140921b15: pop r15
140921b17: pop rbp
140921b18: ret 
140921b19: nop dword ptr [rax]
140921b20: mov qword ptr [rsp + 0x10], rdx
140921b25: push rbp
140921b26: push r15
140921b28: push r14
140921b2a: push r13
140921b2c: push r12
140921b2e: push rsi
140921b2f: push rdi
140921b30: push rbx
140921b31: sub rsp, 0x58
140921b35: lea rbp, [rdx + 0x80]
140921b3c: movaps xmmword ptr [rsp + 0x20], xmm8
140921b42: movaps xmmword ptr [rsp + 0x30], xmm7
140921b47: movaps xmmword ptr [rsp + 0x40], xmm6
140921b4c: mov rax, qword ptr [rbp + 0x250]
140921b53: mov rdx, qword ptr [rax + 8]
140921b57: test rdx, rdx
140921b5a: je 0x140921b73
140921b5c: mov rax, qword ptr [rbp + 0x250]
140921b63: mov r8, qword ptr [rax + 0x10]
140921b67: mov rcx, qword ptr [rbp + 0x310]
140921b6e: call 0x140613c20
140921b73: movaps xmm6, xmmword ptr [rsp + 0x40]
140921b78: movaps xmm7, xmmword ptr [rsp + 0x30]
140921b7d: movaps xmm8, xmmword ptr [rsp + 0x20]
140921b83: add rsp, 0x58
140921b87: pop rbx
140921b88: pop rdi
140921b89: pop rsi
140921b8a: pop r12
140921b8c: pop r13
140921b8e: pop r14
140921b90: pop r15
140921b92: pop rbp
140921b93: ret 
140921b94: nop word ptr cs:[rax + rax]
140921ba0: mov qword ptr [rsp + 0x10], rdx
140921ba5: push rbp
140921ba6: push r15
140921ba8: push r14
140921baa: push r13
140921bac: push r12
140921bae: push rsi
140921baf: push rdi
140921bb0: push rbx
140921bb1: sub rsp, 0x58
140921bb5: lea rbp, [rdx + 0x80]
140921bbc: movaps xmmword ptr [rsp + 0x20], xmm8
140921bc2: movaps xmmword ptr [rsp + 0x30], xmm7
140921bc7: movaps xmmword ptr [rsp + 0x40], xmm6
140921bcc: lea rcx, [rbp + 0x160]
140921bd3: call 0x1405e7a00
140921bd8: mov byte ptr [rbp + 0x327], 1
140921bdf: movaps xmm6, xmmword ptr [rsp + 0x40]
140921be4: movaps xmm7, xmmword ptr [rsp + 0x30]
140921be9: movaps xmm8, xmmword ptr [rsp + 0x20]
140921bef: add rsp, 0x58
140921bf3: pop rbx
140921bf4: pop rdi
140921bf5: pop rsi
140921bf6: pop r12
140921bf8: pop r13
140921bfa: pop r14
140921bfc: pop r15
140921bfe: pop rbp
140921bff: ret 
140921c00: mov qword ptr [rsp + 0x10], rdx
140921c05: push rbp
140921c06: push r15
140921c08: push r14
140921c0a: push r13
140921c0c: push r12
140921c0e: push rsi
140921c0f: push rdi
140921c10: push rbx
140921c11: sub rsp, 0x58
140921c15: lea rbp, [rdx + 0x80]
140921c1c: movaps xmmword ptr [rsp + 0x20], xmm8
140921c22: movaps xmmword ptr [rsp + 0x30], xmm7
140921c27: movaps xmmword ptr [rsp + 0x40], xmm6
140921c2c: lea rcx, [rbp + 0x278]
140921c33: mov byte ptr [rbp + 0x324], 1
140921c3a: call 0x140888890
140921c3f: mov byte ptr [rbp + 0x324], 1
140921c46: movaps xmm6, xmmword ptr [rsp + 0x40]
140921c4b: movaps xmm7, xmmword ptr [rsp + 0x30]
140921c50: movaps xmm8, xmmword ptr [rsp + 0x20]
140921c56: add rsp, 0x58
140921c5a: pop rbx
140921c5b: pop rdi
140921c5c: pop rsi
140921c5d: pop r12
140921c5f: pop r13
140921c61: pop r14
140921c63: pop r15
140921c65: pop rbp
140921c66: ret 
140921c67: nop word ptr [rax + rax]
140921c70: mov qword ptr [rsp + 0x10], rdx
140921c75: push rbp
140921c76: push r15
140921c78: push r14
140921c7a: push r13
140921c7c: push r12
140921c7e: push rsi
140921c7f: push rdi
140921c80: push rbx
140921c81: sub rsp, 0x58
140921c85: lea rbp, [rdx + 0x80]
140921c8c: movaps xmmword ptr [rsp + 0x20], xmm8
140921c92: movaps xmmword ptr [rsp + 0x30], xmm7
140921c97: movaps xmmword ptr [rsp + 0x40], xmm6
140921c9c: mov rax, qword ptr [rbp + 0x230]
140921ca3: lock dec qword ptr [rax]
140921ca7: jne 0x140921cbc
140921ca9: mov byte ptr [rbp + 0x323], 1
140921cb0: lea rcx, [rbp + 0x260]
140921cb7: call 0x140b5cdb0
140921cbc: mov byte ptr [rbp + 0x323], 1
140921cc3: movaps xmm6, xmmword ptr [rsp + 0x40]
140921cc8: movaps xmm7, xmmword ptr [rsp + 0x30]
140921ccd: movaps xmm8, xmmword ptr [rsp + 0x20]
140921cd3: add rsp, 0x58
140921cd7: pop rbx
140921cd8: pop rdi
140921cd9: pop rsi
140921cda: pop r12
140921cdc: pop r13
140921cde: pop r14
140921ce0: pop r15
140921ce2: pop rbp
140921ce3: ret 
140921ce4: nop word ptr cs:[rax + rax]
140921cf0: mov qword ptr [rsp + 0x10], rdx
140921cf5: push rbp
140921cf6: push r15
140921cf8: push r14
140921cfa: push r13
140921cfc: push r12
140921cfe: push rsi
140921cff: push rdi
140921d00: push rbx
140921d01: sub rsp, 0x58
140921d05: lea rbp, [rdx + 0x80]
140921d0c: movaps xmmword ptr [rsp + 0x20], xmm8
140921d12: movaps xmmword ptr [rsp + 0x30], xmm7
140921d17: movaps xmmword ptr [rsp + 0x40], xmm6
140921d1c: lea rcx, [rbp + 0x90]
140921d23: call 0x1405e7a00
140921d28: movaps xmm6, xmmword ptr [rsp + 0x40]
140921d2d: movaps xmm7, xmmword ptr [rsp + 0x30]
140921d32: movaps xmm8, xmmword ptr [rsp + 0x20]
140921d38: add rsp, 0x58
140921d3c: pop rbx
140921d3d: pop rdi
140921d3e: pop rsi
140921d3f: pop r12
140921d41: pop r13
140921d43: pop r14
140921d45: pop r15
140921d47: pop rbp
140921d48: ret 
140921d49: nop dword ptr [rax]
140921d50: mov qword ptr [rsp + 0x10], rdx
140921d55: push rbp
140921d56: push r15
140921d58: push r14
140921d5a: push r13
140921d5c: push r12
140921d5e: push rsi
140921d5f: push rdi
140921d60: push rbx
140921d61: sub rsp, 0x58
140921d65: lea rbp, [rdx + 0x80]
140921d6c: movaps xmmword ptr [rsp + 0x20], xmm8
140921d72: movaps xmmword ptr [rsp + 0x30], xmm7
140921d77: movaps xmmword ptr [rsp + 0x40], xmm6
140921d7c: mov rdx, qword ptr [rbp + 0x210]
140921d83: test rdx, rdx
140921d86: je 0x140921d9a
140921d88: mov rcx, qword ptr [rbp + 0x218]
140921d8f: mov r8d, 1
140921d95: call 0x140613c20
140921d9a: mov byte ptr [rbp + 0x321], 1
140921da1: movaps xmm6, xmmword ptr [rsp + 0x40]
140921da6: movaps xmm7, xmmword ptr [rsp + 0x30]
140921dab: movaps xmm8, xmmword ptr [rsp + 0x20]
140921db1: add rsp, 0x58
140921db5: pop rbx
140921db6: pop rdi
140921db7: pop rsi
140921db8: pop r12
140921dba: pop r13
140921dbc: pop r14
140921dbe: pop r15
140921dc0: pop rbp
140921dc1: ret 
140921dc2: nop word ptr cs:[rax + rax]
140921dd0: mov qword ptr [rsp + 0x10], rdx
140921dd5: push rbp
140921dd6: push r15
140921dd8: push r14
140921dda: push r13
140921ddc: push r12
140921dde: push rsi
140921ddf: push rdi
140921de0: push rbx
140921de1: sub rsp, 0x58
140921de5: lea rbp, [rdx + 0x80]
140921dec: movaps xmmword ptr [rsp + 0x20], xmm8
140921df2: movaps xmmword ptr [rsp + 0x30], xmm7
140921df7: movaps xmmword ptr [rsp + 0x40], xmm6
140921dfc: cmp qword ptr [rbp + 0x250], 0
140921e04: je 0x140921e27
140921e06: mov rax, qword ptr [rbp + 0x250]
140921e0d: shl rax, 3
140921e11: lea rdx, [rax + rax*2]
140921e15: mov r8d, 8
140921e1b: mov rcx, qword ptr [rbp + 0x88]
140921e22: call 0x140613c20
140921e27: mov byte ptr [rbp + 0x322], 1
140921e2e: movaps xmm6, xmmword ptr [rsp + 0x40]
140921e33: movaps xmm7, xmmword ptr [rsp + 0x30]
140921e38: movaps xmm8, xmmword ptr [rsp + 0x20]
140921e3e: add rsp, 0x58
140921e42: pop rbx
140921e43: pop rdi
140921e44: pop rsi
140921e45: pop r12
140921e47: pop r13
140921e49: pop r14
140921e4b: pop r15
140921e4d: pop rbp
140921e4e: ret 
140921e4f: nop 
140921e50: mov qword ptr [rsp + 0x10], rdx
140921e55: push rbp
140921e56: push r15
140921e58: push r14
140921e5a: push r13
140921e5c: push r12
140921e5e: push rsi
140921e5f: push rdi
140921e60: push rbx
140921e61: sub rsp, 0x58
140921e65: lea rbp, [rdx + 0x80]
140921e6c: movaps xmmword ptr [rsp + 0x20], xmm8
140921e72: movaps xmmword ptr [rsp + 0x30], xmm7
140921e77: movaps xmmword ptr [rsp + 0x40], xmm6
140921e7c: lea rcx, [rbp + 0x90]
140921e83: call 0x140892dc0
140921e88: movaps xmm6, xmmword ptr [rsp + 0x40]
140921e8d: movaps xmm7, xmmword ptr [rsp + 0x30]
140921e92: movaps xmm8, xmmword ptr [rsp + 0x20]
140921e98: add rsp, 0x58
140921e9c: pop rbx
140921e9d: pop rdi
140921e9e: pop rsi
140921e9f: pop r12
140921ea1: pop r13
140921ea3: pop r14
140921ea5: pop r15
140921ea7: pop rbp
140921ea8: ret 
140921ea9: nop dword ptr [rax]
140921eb0: mov qword ptr [rsp + 0x10], rdx
140921eb5: push rbp
140921eb6: push r15
140921eb8: push r14
140921eba: push r13
140921ebc: push r12
140921ebe: push rsi
140921ebf: push rdi
140921ec0: push rbx
140921ec1: sub rsp, 0x58
140921ec5: lea rbp, [rdx + 0x80]
140921ecc: movaps xmmword ptr [rsp + 0x20], xmm8
140921ed2: movaps xmmword ptr [rsp + 0x30], xmm7
140921ed7: movaps xmmword ptr [rsp + 0x40], xmm6
140921edc: mov byte ptr [rbp + 0x321], 1
140921ee3: lea rcx, [rbp + 0x90]
140921eea: call 0x1405e88a0
140921eef: mov byte ptr [rbp + 0x321], 1
140921ef6: movaps xmm6, xmmword ptr [rsp + 0x40]
140921efb: movaps xmm7, xmmword ptr [rsp + 0x30]
140921f00: movaps xmm8, xmmword ptr [rsp + 0x20]
140921f06: add rsp, 0x58
140921f0a: pop rbx
140921f0b: pop rdi
140921f0c: pop rsi
140921f0d: pop r12
140921f0f: pop r13
140921f11: pop r14
140921f13: pop r15
140921f15: pop rbp
140921f16: ret 
140921f17: nop word ptr [rax + rax]
140921f20: mov qword ptr [rsp + 0x10], rdx
140921f25: push rbp
140921f26: push r15
140921f28: push r14
140921f2a: push r13
140921f2c: push r12
140921f2e: push rsi
140921f2f: push rdi
140921f30: push rbx
140921f31: sub rsp, 0x58
140921f35: lea rbp, [rdx + 0x80]
140921f3c: movaps xmmword ptr [rsp + 0x20], xmm8
140921f42: movaps xmmword ptr [rsp + 0x30], xmm7
140921f47: movaps xmmword ptr [rsp + 0x40], xmm6
140921f4c: mov rcx, qword ptr [rbp + 0x308]
140921f53: call 0x140161c30
140921f58: mov byte ptr [rbp + 0x326], 1
140921f5f: movaps xmm6, xmmword ptr [rsp + 0x40]
140921f64: movaps xmm7, xmmword ptr [rsp + 0x30]
140921f69: movaps xmm8, xmmword ptr [rsp + 0x20]
140921f6f: add rsp, 0x58
140921f73: pop rbx
140921f74: pop rdi
140921f75: pop rsi
140921f76: pop r12
140921f78: pop r13
140921f7a: pop r14
140921f7c: pop r15
140921f7e: pop rbp
140921f7f: ret 
140921f80: mov qword ptr [rsp + 0x10], rdx
140921f85: push rbp
140921f86: push r15
140921f88: push r14
140921f8a: push r13
140921f8c: push r12
140921f8e: push rsi
140921f8f: push rdi
140921f90: push rbx
140921f91: sub rsp, 0x58
140921f95: lea rbp, [rdx + 0x80]
140921f9c: movaps xmmword ptr [rsp + 0x20], xmm8
140921fa2: movaps xmmword ptr [rsp + 0x30], xmm7
140921fa7: movaps xmmword ptr [rsp + 0x40], xmm6
140921fac: cmp byte ptr [rbp + 0x31f], 0
140921fb3: je 0x140921ffb
140921fb5: mov rdx, qword ptr [rbp + 0x260]
140921fbc: test rdx, rdx
140921fbf: je 0x140921fd3
140921fc1: mov rcx, qword ptr [rbp + 0x268]
140921fc8: mov r8d, 1
140921fce: call 0x140613c20
140921fd3: mov byte ptr [rbp + 0x327], 1
140921fda: movaps xmm6, xmmword ptr [rsp + 0x40]
140921fdf: movaps xmm7, xmmword ptr [rsp + 0x30]
140921fe4: movaps xmm8, xmmword ptr [rsp + 0x20]
140921fea: add rsp, 0x58
140921fee: pop rbx
140921fef: pop rdi
140921ff0: pop rsi
140921ff1: pop r12
140921ff3: pop r13
140921ff5: pop r14
140921ff7: pop r15
140921ff9: pop rbp
140921ffa: ret 
140921ffb: mov byte ptr [rbp + 0x327], 1
140922002: movaps xmm6, xmmword ptr [rsp + 0x40]
140922007: movaps xmm7, xmmword ptr [rsp + 0x30]
14092200c: movaps xmm8, xmmword ptr [rsp + 0x20]
140922012: add rsp, 0x58
140922016: pop rbx
140922017: pop rdi
140922018: pop rsi
140922019: pop r12
14092201b: pop r13
14092201d: pop r14
14092201f: pop r15
140922021: pop rbp
140922022: ret 
140922023: nop word ptr cs:[rax + rax]
140922030: mov qword ptr [rsp + 0x10], rdx
140922035: push rbp
140922036: push r15
140922038: push r14
14092203a: push r13
14092203c: push r12
14092203e: push rsi
14092203f: push rdi
140922040: push rbx
140922041: sub rsp, 0x58
140922045: lea rbp, [rdx + 0x80]
14092204c: movaps xmmword ptr [rsp + 0x20], xmm8
140922052: movaps xmmword ptr [rsp + 0x30], xmm7
140922057: movaps xmmword ptr [rsp + 0x40], xmm6
14092205c: movzx ebx, byte ptr [rbp + 0x323]
140922063: test bl, 1
140922066: je 0x1409220a8
140922068: mov rax, qword ptr [rbp + 0x210]
14092206f: lock dec qword ptr [rax]
140922073: jne 0x140922081
140922075: lea rcx, [rbp + 0x210]
14092207c: call 0x140b5cdb0
140922081: mov byte ptr [rbp + 0x324], bl
140922087: movaps xmm6, xmmword ptr [rsp + 0x40]
14092208c: movaps xmm7, xmmword ptr [rsp + 0x30]
140922091: movaps xmm8, xmmword ptr [rsp + 0x20]
140922097: add rsp, 0x58
14092209b: pop rbx
14092209c: pop rdi
14092209d: pop rsi
14092209e: pop r12
1409220a0: pop r13
1409220a2: pop r14
1409220a4: pop r15
1409220a6: pop rbp
1409220a7: ret 
1409220a8: mov byte ptr [rbp + 0x324], bl
1409220ae: movaps xmm6, xmmword ptr [rsp + 0x40]
1409220b3: movaps xmm7, xmmword ptr [rsp + 0x30]
1409220b8: movaps xmm8, xmmword ptr [rsp + 0x20]
1409220be: add rsp, 0x58
1409220c2: pop rbx
1409220c3: pop rdi
1409220c4: pop rsi
1409220c5: pop r12
1409220c7: pop r13
1409220c9: pop r14
1409220cb: pop r15
1409220cd: pop rbp
1409220ce: ret 
1409220cf: nop 
1409220d0: mov qword ptr [rsp + 0x10], rdx
1409220d5: push rbp
1409220d6: push r15
1409220d8: push r14
1409220da: push r13
1409220dc: push r12
1409220de: push rsi
1409220df: push rdi
1409220e0: push rbx
1409220e1: sub rsp, 0x58
1409220e5: lea rbp, [rdx + 0x80]
1409220ec: movaps xmmword ptr [rsp + 0x20], xmm8
1409220f2: movaps xmmword ptr [rsp + 0x30], xmm7
1409220f7: movaps xmmword ptr [rsp + 0x40], xmm6
1409220fc: lea rcx, [rbp + 0x260]
140922103: call 0x140888890
140922108: mov byte ptr [rbp + 0x326], 0
14092210f: movaps xmm6, xmmword ptr [rsp + 0x40]
140922114: movaps xmm7, xmmword ptr [rsp + 0x30]
140922119: movaps xmm8, xmmword ptr [rsp + 0x20]
14092211f: add rsp, 0x58
140922123: pop rbx
140922124: pop rdi
140922125: pop rsi
140922126: pop r12
140922128: pop r13
14092212a: pop r14
14092212c: pop r15
14092212e: pop rbp
14092212f: ret 
140922130: mov qword ptr [rsp + 0x10], rdx
140922135: push rbp
140922136: push r15
140922138: push r14
14092213a: push r13
14092213c: push r12
14092213e: push rsi
14092213f: push rdi
140922140: push rbx
140922141: sub rsp, 0x58
140922145: lea rbp, [rdx + 0x80]
14092214c: movaps xmmword ptr [rsp + 0x20], xmm8
140922152: movaps xmmword ptr [rsp + 0x30], xmm7
140922157: movaps xmmword ptr [rsp + 0x40], xmm6
14092215c: cmp byte ptr [rbp + 0x321], 0
140922163: je 0x1409221a4
140922165: mov rdx, qword ptr [rbp + 0x160]
14092216c: test rdx, rdx
14092216f: je 0x140922183
140922171: mov rcx, qword ptr [rbp + 0x168]
140922178: mov r8d, 1
14092217e: call 0x140613c20
140922183: movaps xmm6, xmmword ptr [rsp + 0x40]
140922188: movaps xmm7, xmmword ptr [rsp + 0x30]
14092218d: movaps xmm8, xmmword ptr [rsp + 0x20]
140922193: add rsp, 0x58
140922197: pop rbx
140922198: pop rdi
140922199: pop rsi
14092219a: pop r12
14092219c: pop r13
14092219e: pop r14
1409221a0: pop r15
1409221a2: pop rbp
1409221a3: ret 
1409221a4: movaps xmm6, xmmword ptr [rsp + 0x40]
1409221a9: movaps xmm7, xmmword ptr [rsp + 0x30]
1409221ae: movaps xmm8, xmmword ptr [rsp + 0x20]
1409221b4: add rsp, 0x58
1409221b8: pop rbx
1409221b9: pop rdi
1409221ba: pop rsi
1409221bb: pop r12
1409221bd: pop r13
1409221bf: pop r14
1409221c1: pop r15
1409221c3: pop rbp
1409221c4: ret 
1409221c5: nop word ptr cs:[rax + rax]
1409221d0: mov qword ptr [rsp + 0x10], rdx
1409221d5: push rbp
1409221d6: push r15
1409221d8: push r14
1409221da: push r13
1409221dc: push r12
1409221de: push rsi
1409221df: push rdi
1409221e0: push rbx
1409221e1: sub rsp, 0x58
1409221e5: lea rbp, [rdx + 0x80]
1409221ec: movaps xmmword ptr [rsp + 0x20], xmm8
1409221f2: movaps xmmword ptr [rsp + 0x30], xmm7
1409221f7: movaps xmmword ptr [rsp + 0x40], xmm6
1409221fc: movzx ebx, byte ptr [rbp + 0x324]
140922203: mov rcx, qword ptr [rbp + 0x310]
14092220a: call 0x140161c30
14092220f: mov byte ptr [rbp + 0x325], bl
140922215: movaps xmm6, xmmword ptr [rsp + 0x40]
14092221a: movaps xmm7, xmmword ptr [rsp + 0x30]
14092221f: movaps xmm8, xmmword ptr [rsp + 0x20]
140922225: add rsp, 0x58
140922229: pop rbx
14092222a: pop rdi
14092222b: pop rsi
14092222c: pop r12
14092222e: pop r13
140922230: pop r14
140922232: pop r15
140922234: pop rbp
140922235: ret 
140922236: nop word ptr cs:[rax + rax]
140922240: mov qword ptr [rsp + 0x10], rdx
140922245: push rbp
140922246: push r15
140922248: push r14
14092224a: push r13
14092224c: push r12
14092224e: push rsi
14092224f: push rdi
140922250: push rbx
140922251: sub rsp, 0x58
140922255: lea rbp, [rdx + 0x80]
14092225c: movaps xmmword ptr [rsp + 0x20], xmm8
140922262: movaps xmmword ptr [rsp + 0x30], xmm7
140922267: movaps xmmword ptr [rsp + 0x40], xmm6
14092226c: lea rcx, [rbp + 0x260]
140922273: call 0x1405d8fc0
140922278: mov byte ptr [rbp + 0x31e], 0
14092227f: movaps xmm6, xmmword ptr [rsp + 0x40]
140922284: movaps xmm7, xmmword ptr [rsp + 0x30]
140922289: movaps xmm8, xmmword ptr [rsp + 0x20]
14092228f: add rsp, 0x58
140922293: pop rbx
140922294: pop rdi
140922295: pop rsi
140922296: pop r12
140922298: pop r13
14092229a: pop r14
14092229c: pop r15
14092229e: pop rbp
14092229f: ret 
1409222a0: mov qword ptr [rsp + 0x10], rdx
1409222a5: push rbp
1409222a6: push r15
1409222a8: push r14
1409222aa: push r13
1409222ac: push r12
1409222ae: push rsi
1409222af: push rdi
1409222b0: push rbx
1409222b1: sub rsp, 0x58
1409222b5: lea rbp, [rdx + 0x80]
1409222bc: movaps xmmword ptr [rsp + 0x20], xmm8
1409222c2: movaps xmmword ptr [rsp + 0x30], xmm7
1409222c7: movaps xmmword ptr [rsp + 0x40], xmm6
1409222cc: movzx ebx, byte ptr [rbp + 0x31e]
1409222d3: test bl, 1
1409222d6: je 0x14092230b
1409222d8: lea rcx, [rbp + 0x1f0]
1409222df: call 0x1405d8fc0
1409222e4: mov byte ptr [rbp + 0x322], bl
1409222ea: movaps xmm6, xmmword ptr [rsp + 0x40]
1409222ef: movaps xmm7, xmmword ptr [rsp + 0x30]
1409222f4: movaps xmm8, xmmword ptr [rsp + 0x20]
1409222fa: add rsp, 0x58
1409222fe: pop rbx
1409222ff: pop rdi
140922300: pop rsi
140922301: pop r12
140922303: pop r13
140922305: pop r14
140922307: pop r15
140922309: pop rbp
14092230a: ret 
14092230b: mov byte ptr [rbp + 0x322], bl
140922311: movaps xmm6, xmmword ptr [rsp + 0x40]
140922316: movaps xmm7, xmmword ptr [rsp + 0x30]
14092231b: movaps xmm8, xmmword ptr [rsp + 0x20]
140922321: add rsp, 0x58
140922325: pop rbx
140922326: pop rdi
140922327: pop rsi
140922328: pop r12
14092232a: pop r13
14092232c: pop r14
14092232e: pop r15
140922330: pop rbp
140922331: ret 
140922332: nop word ptr cs:[rax + rax]
140922340: mov qword ptr [rsp + 0x10], rdx
140922345: push rbp
140922346: push r15
140922348: push r14
14092234a: push r13
14092234c: push r12
14092234e: push rsi
14092234f: push rdi
140922350: push rbx
140922351: sub rsp, 0x58
140922355: lea rbp, [rdx + 0x80]
14092235c: movaps xmmword ptr [rsp + 0x20], xmm8
140922362: movaps xmmword ptr [rsp + 0x30], xmm7
140922367: movaps xmmword ptr [rsp + 0x40], xmm6
14092236c: test byte ptr [rbp + 0x322], 1
140922373: je 0x1409223c0
140922375: cmp qword ptr [rbp + 0x310], 0
14092237d: je 0x140922398
14092237f: mov r8d, 1
140922385: mov rcx, qword ptr [rbp + 0x308]
14092238c: mov rdx, qword ptr [rbp + 0x310]
140922393: call 0x140613c20
140922398: mov byte ptr [rbp + 0x327], 1
14092239f: movaps xmm6, xmmword ptr [rsp + 0x40]
1409223a4: movaps xmm7, xmmword ptr [rsp + 0x30]
1409223a9: movaps xmm8, xmmword ptr [rsp + 0x20]
1409223af: add rsp, 0x58
1409223b3: pop rbx
1409223b4: pop rdi
1409223b5: pop rsi
1409223b6: pop r12
1409223b8: pop r13
1409223ba: pop r14
1409223bc: pop r15
1409223be: pop rbp
1409223bf: ret 
1409223c0: mov byte ptr [rbp + 0x327], 1
1409223c7: movaps xmm6, xmmword ptr [rsp + 0x40]
1409223cc: movaps xmm7, xmmword ptr [rsp + 0x30]
1409223d1: movaps xmm8, xmmword ptr [rsp + 0x20]
1409223d7: add rsp, 0x58
1409223db: pop rbx
1409223dc: pop rdi
1409223dd: pop rsi
1409223de: pop r12
1409223e0: pop r13
1409223e2: pop r14
1409223e4: pop r15
1409223e6: pop rbp
1409223e7: ret 
1409223e8: nop dword ptr [rax + rax]
1409223f0: mov qword ptr [rsp + 0x10], rdx
1409223f5: push rbp
1409223f6: push r15
1409223f8: push r14
1409223fa: push r13
1409223fc: push r12
1409223fe: push rsi
1409223ff: push rdi
140922400: push rbx
140922401: sub rsp, 0x58
140922405: lea rbp, [rdx + 0x80]
14092240c: movaps xmmword ptr [rsp + 0x20], xmm8
140922412: movaps xmmword ptr [rsp + 0x30], xmm7
140922417: movaps xmmword ptr [rsp + 0x40], xmm6
14092241c: test byte ptr [rbp + 0x325], 1
140922423: je 0x14092246b
140922425: mov rdx, qword ptr [rbp + 0x160]
14092242c: test rdx, rdx
14092242f: je 0x140922443
140922431: mov rcx, qword ptr [rbp + 0x168]
140922438: mov r8d, 1
14092243e: call 0x140613c20
140922443: mov byte ptr [rbp + 0x327], 1
14092244a: movaps xmm6, xmmword ptr [rsp + 0x40]
14092244f: movaps xmm7, xmmword ptr [rsp + 0x30]
140922454: movaps xmm8, xmmword ptr [rsp + 0x20]
14092245a: add rsp, 0x58
14092245e: pop rbx
14092245f: pop rdi
140922460: pop rsi
140922461: pop r12
140922463: pop r13
140922465: pop r14
140922467: pop r15
140922469: pop rbp
14092246a: ret 
14092246b: mov byte ptr [rbp + 0x327], 1
140922472: movaps xmm6, xmmword ptr [rsp + 0x40]
140922477: movaps xmm7, xmmword ptr [rsp + 0x30]
14092247c: movaps xmm8, xmmword ptr [rsp + 0x20]
140922482: add rsp, 0x58
140922486: pop rbx
140922487: pop rdi
140922488: pop rsi
140922489: pop r12
14092248b: pop r13
14092248d: pop r14
14092248f: pop r15
140922491: pop rbp
140922492: ret 
140922493: nop word ptr cs:[rax + rax]
1409224a0: mov qword ptr [rsp + 0x10], rdx
1409224a5: push rbp
1409224a6: push r15
1409224a8: push r14
1409224aa: push r13
1409224ac: push r12
1409224ae: push rsi
1409224af: push rdi
1409224b0: push rbx
1409224b1: sub rsp, 0x58
1409224b5: lea rbp, [rdx + 0x80]
1409224bc: movaps xmmword ptr [rsp + 0x20], xmm8
1409224c2: movaps xmmword ptr [rsp + 0x30], xmm7
1409224c7: movaps xmmword ptr [rsp + 0x40], xmm6
1409224cc: cmp byte ptr [rbp + 0x326], 0
1409224d3: je 0x14092250f
1409224d5: mov rax, qword ptr [rbp + 0x208]
1409224dc: lock dec qword ptr [rax]
1409224e0: jne 0x1409224ee
1409224e2: lea rcx, [rbp + 0x208]
1409224e9: call 0x140b5cdb0
1409224ee: movaps xmm6, xmmword ptr [rsp + 0x40]
1409224f3: movaps xmm7, xmmword ptr [rsp + 0x30]
1409224f8: movaps xmm8, xmmword ptr [rsp + 0x20]
1409224fe: add rsp, 0x58
140922502: pop rbx
140922503: pop rdi
140922504: pop rsi
140922505: pop r12
140922507: pop r13
140922509: pop r14
14092250b: pop r15
14092250d: pop rbp
14092250e: ret 
14092250f: movaps xmm6, xmmword ptr [rsp + 0x40]
140922514: movaps xmm7, xmmword ptr [rsp + 0x30]
140922519: movaps xmm8, xmmword ptr [rsp + 0x20]
14092251f: add rsp, 0x58
140922523: pop rbx
140922524: pop rdi
140922525: pop rsi
140922526: pop r12
140922528: pop r13
14092252a: pop r14
14092252c: pop r15
14092252e: pop rbp
14092252f: ret 
140922530: mov qword ptr [rsp + 0x10], rdx
140922535: push rbp
140922536: push r15
140922538: push r14
14092253a: push r13
14092253c: push r12
14092253e: push rsi
14092253f: push rdi
140922540: push rbx
140922541: sub rsp, 0x58
140922545: lea rbp, [rdx + 0x80]
14092254c: movaps xmmword ptr [rsp + 0x20], xmm8
140922552: movaps xmmword ptr [rsp + 0x30], xmm7
140922557: movaps xmmword ptr [rsp + 0x40], xmm6
14092255c: mov rdx, qword ptr [rbp + 0x210]
140922563: lea rax, [rdx*2]
14092256b: test rax, rax
14092256e: je 0x140922582
140922570: mov rcx, qword ptr [rbp + 0x218]
140922577: mov r8d, 1
14092257d: call 0x140613c20
140922582: mov rdx, qword ptr [rbp + 0x1f0]
140922589: test rdx, rdx
14092258c: je 0x1409225a0
14092258e: mov rcx, qword ptr [rbp + 0x1f8]
140922595: mov r8d, 1
14092259b: call 0x140613c20
1409225a0: mov byte ptr [rbp + 0x327], 1
1409225a7: movaps xmm6, xmmword ptr [rsp + 0x40]
1409225ac: movaps xmm7, xmmword ptr [rsp + 0x30]
1409225b1: movaps xmm8, xmmword ptr [rsp + 0x20]
1409225b7: add rsp, 0x58
1409225bb: pop rbx
1409225bc: pop rdi
1409225bd: pop rsi
1409225be: pop r12
1409225c0: pop r13
1409225c2: pop r14
1409225c4: pop r15
1409225c6: pop rbp
1409225c7: ret 
1409225c8: nop dword ptr [rax + rax]
1409225d0: mov qword ptr [rsp + 0x10], rdx
1409225d5: push rbp
1409225d6: push r15
1409225d8: push r14
1409225da: push r13
1409225dc: push r12
1409225de: push rsi
1409225df: push rdi
1409225e0: push rbx
1409225e1: sub rsp, 0x58
1409225e5: lea rbp, [rdx + 0x80]
1409225ec: movaps xmmword ptr [rsp + 0x20], xmm8
1409225f2: movaps xmmword ptr [rsp + 0x30], xmm7
1409225f7: movaps xmmword ptr [rsp + 0x40], xmm6
1409225fc: cmp byte ptr [rbp + 0x327], 0
140922603: je 0x140922634
140922605: movabs rcx, 0x8000000000000000
14092260f: xor rcx, qword ptr [rbp + 0x1b0]
140922616: cmp rcx, 0x14
14092261a: mov eax, 0xd
14092261f: cmovb rax, rcx
140922623: cmp rax, 0xf
140922627: ja 0x140922655
140922629: mov ecx, 0xf101
14092262e: bt rcx, rax
140922632: jae 0x140922655
140922634: movaps xmm6, xmmword ptr [rsp + 0x40]
140922639: movaps xmm7, xmmword ptr [rsp + 0x30]
14092263e: movaps xmm8, xmmword ptr [rsp + 0x20]
140922644: add rsp, 0x58
140922648: pop rbx
140922649: pop rdi
14092264a: pop rsi
14092264b: pop r12
14092264d: pop r13
14092264f: pop r14
140922651: pop r15
140922653: pop rbp
140922654: ret 
140922655: lea rcx, [rbp + 0x1b0]
14092265c: call 0x14091baf0
140922661: jmp 0x140922634
