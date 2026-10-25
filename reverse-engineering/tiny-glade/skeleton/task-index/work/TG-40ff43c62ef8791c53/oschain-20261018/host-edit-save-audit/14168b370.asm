14168b370: push rbp
14168b371: push rsi
14168b372: push rdi
14168b373: push rbx
14168b374: sub rsp, 0x298
14168b37b: lea rbp, [rsp + 0x80]
14168b383: mov qword ptr [rbp + 0x210], 0xfffffffffffffffe
14168b38e: mov rsi, rcx
14168b391: movzx eax, byte ptr [rip + 0x1fa9871]
14168b398: mov ecx, 0xe0
14168b39d: mov edx, 8
14168b3a2: call 0x140613c10
14168b3a7: test rax, rax
14168b3aa: je 0x14168b782
14168b3b0: mov qword ptr [rbp + 0x208], rax
14168b3b7: xorps xmm0, xmm0
14168b3ba: movups xmmword ptr [rbp + 0xe8], xmm0
14168b3c1: movups xmmword ptr [rbp + 0x100], xmm0
14168b3c8: movups xmmword ptr [rbp + 0x118], xmm0
14168b3cf: movups xmmword ptr [rbp + 0x130], xmm0
14168b3d6: movups xmmword ptr [rbp + 0x148], xmm0
14168b3dd: mov dword ptr [rbp + 0x158], 0
14168b3e7: movups xmmword ptr [rbp + 0x180], xmm0
14168b3ee: movups xmmword ptr [rbp + 0x198], xmm0
14168b3f5: movups xmmword ptr [rbp + 0x1b0], xmm0
14168b3fc: movups xmmword ptr [rbp + 0x1c8], xmm0
14168b403: movups xmmword ptr [rbp + 0x1e0], xmm0
14168b40a: mov dword ptr [rbp + 0x1f0], 0
14168b414: mov qword ptr [rbp + 0x88], 0
14168b41f: mov qword ptr [rbp + 0xc8], 0
14168b42a: mov qword ptr [rbp + 0xd0], 8
14168b435: mov qword ptr [rbp + 0xd8], 0
14168b440: mov qword ptr [rbp + 0xe0], 0x10
14168b44b: mov qword ptr [rbp + 0xf8], 0x10
14168b456: mov qword ptr [rbp + 0x110], 0x10
14168b461: mov qword ptr [rbp + 0x128], 0x10
14168b46c: mov qword ptr [rbp + 0x140], 0x10
14168b477: movabs rbx, 0x8000000000000000
14168b481: mov qword ptr [rbp + 0x160], rbx
14168b488: lea rax, [rip + 0x15faea1]
14168b48f: mov qword ptr [rbp + 0x168], rax
14168b496: mov qword ptr [rbp + 0x170], 0x50
14168b4a1: mov qword ptr [rbp + 0x178], 0x10
14168b4ac: mov qword ptr [rbp + 0x190], 0x10
14168b4b7: mov qword ptr [rbp + 0x1a8], 0x10
14168b4c2: mov qword ptr [rbp + 0x1c0], 0x10
14168b4cd: mov qword ptr [rbp + 0x1d8], 0x10
14168b4d8: mov dword ptr [rbp + 0x1f8], 0
14168b4e2: mov word ptr [rbp + 0x1fc], 1
14168b4eb: mov dword ptr [rbp + 0x200], 0
14168b4f5: movzx eax, byte ptr [rip + 0x1fa970d]
14168b4fc: mov ecx, 0x180
14168b501: mov edx, 8
14168b506: call 0x140613c10
14168b50b: test rax, rax
14168b50e: je 0x14168b791
14168b514: mov rdi, rax
14168b517: lea rdx, [rbp + 0x88]
14168b51e: mov r8d, 0x180
14168b524: mov rcx, rax
14168b527: call 0x142923c00
14168b52c: lea r8, [rip + 0x15faa15]
14168b533: lea rcx, [rbp + 0x18]
14168b537: mov rdx, rdi
14168b53a: call 0x140ce4b10
14168b53f: xorps xmm0, xmm0
14168b542: movups xmmword ptr [rbp + 0xe8], xmm0
14168b549: movups xmmword ptr [rbp + 0x100], xmm0
14168b550: movups xmmword ptr [rbp + 0x118], xmm0
14168b557: movups xmmword ptr [rbp + 0x130], xmm0
14168b55e: movups xmmword ptr [rbp + 0x148], xmm0
14168b565: mov dword ptr [rbp + 0x158], 0
14168b56f: movups xmmword ptr [rbp + 0x180], xmm0
14168b576: movups xmmword ptr [rbp + 0x198], xmm0
14168b57d: movups xmmword ptr [rbp + 0x1b0], xmm0
14168b584: movups xmmword ptr [rbp + 0x1c8], xmm0
14168b58b: movups xmmword ptr [rbp + 0x1e0], xmm0
14168b592: mov dword ptr [rbp + 0x1f0], 0
14168b59c: mov qword ptr [rbp + 0x88], 0
14168b5a7: mov qword ptr [rbp + 0xc8], 0
14168b5b2: mov qword ptr [rbp + 0xd0], 8
14168b5bd: mov qword ptr [rbp + 0xd8], 0
14168b5c8: mov qword ptr [rbp + 0xe0], 0x10
14168b5d3: mov qword ptr [rbp + 0xf8], 0x10
14168b5de: mov qword ptr [rbp + 0x110], 0x10
14168b5e9: mov qword ptr [rbp + 0x128], 0x10
14168b5f4: mov qword ptr [rbp + 0x140], 0x10
14168b5ff: mov qword ptr [rbp + 0x160], rbx
14168b606: lea rax, [rip + 0x15facbd]
14168b60d: mov qword ptr [rbp + 0x168], rax
14168b614: mov qword ptr [rbp + 0x170], 0x66
14168b61f: mov qword ptr [rbp + 0x178], 0x10
14168b62a: mov qword ptr [rbp + 0x190], 0x10
14168b635: mov qword ptr [rbp + 0x1a8], 0x10
14168b640: mov qword ptr [rbp + 0x1c0], 0x10
14168b64b: mov qword ptr [rbp + 0x1d8], 0x10
14168b656: mov dword ptr [rbp + 0x1f8], 0
14168b660: mov word ptr [rbp + 0x1fc], 1
14168b669: mov dword ptr [rbp + 0x200], 0
14168b673: movzx eax, byte ptr [rip + 0x1fa958f]
14168b67a: mov ecx, 0x180
14168b67f: mov edx, 8
14168b684: call 0x140613c10
14168b689: test rax, rax
14168b68c: je 0x14168b7a2
14168b692: mov rdi, rax
14168b695: lea rdx, [rbp + 0x88]
14168b69c: mov r8d, 0x180
14168b6a2: mov rcx, rax
14168b6a5: call 0x142923c00
14168b6aa: lea r8, [rip + 0x15fa7df]
14168b6b1: lea rcx, [rbp - 0x58]
14168b6b5: mov rdx, rdi
14168b6b8: call 0x140ce4b10
14168b6bd: movups xmm0, xmmword ptr [rbp + 0x78]
14168b6c1: mov rax, qword ptr [rbp + 0x208]
14168b6c8: movups xmmword ptr [rax + 0x60], xmm0
14168b6cc: movups xmm0, xmmword ptr [rbp + 0x68]
14168b6d0: movups xmmword ptr [rax + 0x50], xmm0
14168b6d4: movups xmm0, xmmword ptr [rbp + 0x58]
14168b6d8: movups xmmword ptr [rax + 0x40], xmm0
14168b6dc: movups xmm0, xmmword ptr [rbp + 0x18]
14168b6e0: movups xmm1, xmmword ptr [rbp + 0x28]
14168b6e4: movups xmm2, xmmword ptr [rbp + 0x38]
14168b6e8: movups xmm3, xmmword ptr [rbp + 0x48]
14168b6ec: movups xmmword ptr [rax + 0x30], xmm3
14168b6f0: movups xmmword ptr [rax + 0x20], xmm2
14168b6f4: movups xmmword ptr [rax + 0x10], xmm1
14168b6f8: movups xmmword ptr [rax], xmm0
14168b6fb: movups xmm0, xmmword ptr [rbp - 0x58]
14168b6ff: movups xmm1, xmmword ptr [rbp - 0x48]
14168b703: movups xmm2, xmmword ptr [rbp - 0x38]
14168b707: movups xmm3, xmmword ptr [rbp - 0x28]
14168b70b: movups xmmword ptr [rax + 0x70], xmm0
14168b70f: movups xmmword ptr [rax + 0x80], xmm1
14168b716: movups xmmword ptr [rax + 0x90], xmm2
14168b71d: movups xmmword ptr [rax + 0xa0], xmm3
14168b724: movups xmm0, xmmword ptr [rbp - 0x18]
14168b728: movups xmmword ptr [rax + 0xb0], xmm0
14168b72f: movups xmm0, xmmword ptr [rbp - 8]
14168b733: movups xmmword ptr [rax + 0xc0], xmm0
14168b73a: movups xmm0, xmmword ptr [rbp + 8]
14168b73e: movups xmmword ptr [rax + 0xd0], xmm0
14168b745: mov qword ptr [rsi + 8], 2
14168b74d: mov qword ptr [rsi + 0x10], rax
14168b751: mov qword ptr [rsi + 0x18], 2
14168b759: mov qword ptr [rsi + 0x20], 0
14168b761: mov qword ptr [rsi + 0x28], 8
14168b769: xorps xmm0, xmm0
14168b76c: movups xmmword ptr [rsi + 0x30], xmm0
14168b770: mov qword ptr [rsi], rbx
14168b773: mov rax, rsi
14168b776: add rsp, 0x298
14168b77d: pop rbx
14168b77e: pop rdi
14168b77f: pop rsi
14168b780: pop rbp
14168b781: ret 
14168b782: mov ecx, 8
14168b787: mov edx, 0xe0
14168b78c: call 0x1428d9000
14168b791: mov ecx, 8
14168b796: mov edx, 0x180
14168b79b: call 0x1428d9000
14168b7a0: jmp 0x14168b7b1
14168b7a2: mov ecx, 8
14168b7a7: mov edx, 0x180
14168b7ac: call 0x1428d9000
14168b7b1: ud2 
14168b7b3: nop word ptr cs:[rax + rax]
14168b7c0: mov qword ptr [rsp + 0x10], rdx
14168b7c5: push rbp
14168b7c6: push rsi
14168b7c7: push rdi
14168b7c8: push rbx
14168b7c9: sub rsp, 0x28
14168b7cd: lea rbp, [rdx + 0x80]
14168b7d4: lea rcx, [rbp + 0x88]
14168b7db: call 0x1405f3e70
14168b7e0: nop 
14168b7e1: add rsp, 0x28
14168b7e5: pop rbx
14168b7e6: pop rdi
14168b7e7: pop rsi
14168b7e8: pop rbp
14168b7e9: ret 
14168b7ea: nop word ptr [rax + rax]
14168b7f0: mov qword ptr [rsp + 0x10], rdx
14168b7f5: push rbp
14168b7f6: push rsi
14168b7f7: push rdi
14168b7f8: push rbx
14168b7f9: sub rsp, 0x28
14168b7fd: lea rbp, [rdx + 0x80]
14168b804: mov edx, 0xe0
14168b809: mov r8d, 8
14168b80f: mov rcx, qword ptr [rbp + 0x208]
14168b816: call 0x140613c20
14168b81b: nop 
14168b81c: add rsp, 0x28
14168b820: pop rbx
14168b821: pop rdi
14168b822: pop rsi
14168b823: pop rbp
14168b824: ret 
14168b825: nop word ptr cs:[rax + rax]
14168b830: mov qword ptr [rsp + 0x10], rdx
14168b835: push rbp
14168b836: push rsi
14168b837: push rdi
14168b838: push rbx
14168b839: sub rsp, 0x28
14168b83d: lea rbp, [rdx + 0x80]
14168b844: lea rcx, [rbp + 0x88]
14168b84b: call 0x1405f3e70
14168b850: nop 
14168b851: add rsp, 0x28
14168b855: pop rbx
14168b856: pop rdi
14168b857: pop rsi
14168b858: pop rbp
14168b859: ret 
14168b85a: nop word ptr [rax + rax]
14168b860: mov qword ptr [rsp + 0x10], rdx
14168b865: push rbp
14168b866: push rsi
14168b867: push rdi
14168b868: push rbx
14168b869: sub rsp, 0x28
14168b86d: lea rbp, [rdx + 0x80]
14168b874: lea rcx, [rbp + 0x18]
14168b878: call 0x140029880
14168b87d: nop 
14168b87e: add rsp, 0x28
14168b882: pop rbx
14168b883: pop rdi
14168b884: pop rsi
14168b885: pop rbp
14168b886: ret 
