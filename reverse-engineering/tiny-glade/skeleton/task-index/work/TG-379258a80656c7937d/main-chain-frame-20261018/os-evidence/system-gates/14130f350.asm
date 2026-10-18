14130f350: push rbp
14130f351: push r15
14130f353: push r14
14130f355: push rsi
14130f356: push rdi
14130f357: push rbx
14130f358: sub rsp, 0x148
14130f35f: lea rbp, [rsp + 0x80]
14130f367: movaps xmmword ptr [rbp + 0xb0], xmm6
14130f36e: mov qword ptr [rbp + 0xa8], 0xfffffffffffffffe
14130f379: mov rdi, r8
14130f37c: mov rsi, rcx
14130f37f: movzx eax, dl
14130f382: lea rcx, [rip + 0x1929d1b]
14130f389: movsxd rax, dword ptr [rcx + rax*4]
14130f38d: add rax, rcx
14130f390: jmp rax
14130f392: mov r14b, 1
14130f395: jmp 0x14130f4aa
14130f39a: mov r14b, 0x2b
14130f39d: jmp 0x14130f4aa
14130f3a2: mov r14b, 0x25
14130f3a5: jmp 0x14130f4aa
14130f3aa: mov r14b, 0x23
14130f3ad: jmp 0x14130f4aa
14130f3b2: mov r14b, 0x24
14130f3b5: jmp 0x14130f4aa
14130f3ba: mov r14b, 0x1b
14130f3bd: jmp 0x14130f4aa
14130f3c2: mov r14b, 0x22
14130f3c5: jmp 0x14130f4aa
14130f3ca: mov r14b, 0x10
14130f3cd: jmp 0x14130f4aa
14130f3d2: mov r14b, 0x2c
14130f3d5: jmp 0x14130f4aa
14130f3da: mov r14b, 0xe
14130f3dd: jmp 0x14130f4aa
14130f3e2: mov r14b, 0x21
14130f3e5: jmp 0x14130f4aa
14130f3ea: mov r14b, 0xb
14130f3ed: jmp 0x14130f4aa
14130f3f2: mov r14b, 4
14130f3f5: jmp 0x14130f4aa
14130f3fa: mov r14b, 0x28
14130f3fd: jmp 0x14130f4aa
14130f402: mov r14b, 0xc
14130f405: jmp 0x14130f4aa
14130f40a: mov r14b, 9
14130f40d: jmp 0x14130f4aa
14130f412: mov r14b, 0x19
14130f415: jmp 0x14130f4aa
14130f41a: mov r14b, 0x18
14130f41d: jmp 0x14130f4aa
14130f422: mov r14b, 2
14130f425: jmp 0x14130f4aa
14130f42a: mov r14b, 0xf
14130f42d: jmp 0x14130f4aa
14130f42f: mov r14b, 3
14130f432: jmp 0x14130f4aa
14130f434: mov r14b, 7
14130f437: jmp 0x14130f4aa
14130f439: mov r14b, 0x1f
14130f43c: jmp 0x14130f4aa
14130f43e: mov r14b, 0x11
14130f441: jmp 0x14130f4aa
14130f443: mov r14b, 0x29
14130f446: jmp 0x14130f4aa
14130f448: mov r14b, 0x14
14130f44b: jmp 0x14130f4aa
14130f44d: mov r14b, 5
14130f450: jmp 0x14130f4aa
14130f452: mov r14b, 0x15
14130f455: jmp 0x14130f4aa
14130f457: mov r14b, 0xd
14130f45a: jmp 0x14130f4aa
14130f45c: mov r14b, 6
14130f45f: jmp 0x14130f4aa
14130f461: mov r14b, 0x12
14130f464: jmp 0x14130f4aa
14130f466: mov r14b, 0xa
14130f469: jmp 0x14130f4aa
14130f46b: mov r14b, 8
14130f46e: jmp 0x14130f4aa
14130f470: mov r14b, 0x1e
14130f473: jmp 0x14130f4aa
14130f475: mov r14b, 0x20
14130f478: jmp 0x14130f4aa
14130f47a: mov r14b, 0x2d
14130f47d: jmp 0x14130f4aa
14130f47f: mov r14b, 0x16
14130f482: jmp 0x14130f4aa
14130f484: mov r14b, 0x27
14130f487: jmp 0x14130f4aa
14130f489: mov r14b, 0x13
14130f48c: jmp 0x14130f4aa
14130f48e: mov r14b, 0x2a
14130f491: jmp 0x14130f4aa
14130f493: mov r14b, 0x17
14130f496: jmp 0x14130f4aa
14130f498: mov r14b, 0x1c
14130f49b: jmp 0x14130f4aa
14130f49d: mov r14b, 0x1d
14130f4a0: jmp 0x14130f4aa
14130f4a2: mov r14b, 0x26
14130f4a5: jmp 0x14130f4aa
14130f4a7: mov r14b, 0x1a
14130f4aa: mov byte ptr [rbp + 0x20], dl
14130f4ad: mov rcx, qword ptr [rip + 0x231cebc]
14130f4b4: lea r8, [rip + 0x19508a5]
14130f4bb: lea rdx, [rbp + 0x20]
14130f4bf: call 0x140b218a0
14130f4c4: mov r8, rdx
14130f4c7: lea rcx, [rbp - 0x50]
14130f4cb: mov rdx, rax
14130f4ce: call 0x140cdd670
14130f4d3: movabs rbx, 0x8000000000000001
14130f4dd: cmp qword ptr [rbp - 0x50], rbx
14130f4e1: jne 0x14130f4ff
14130f4e3: mov qword ptr [rsi], 0
14130f4ea: mov qword ptr [rsi + 8], 8
14130f4f2: mov qword ptr [rsi + 0x10], 0
14130f4fa: jmp 0x14130f81b
14130f4ff: movups xmm0, xmmword ptr [rbp + 0x10]
14130f503: movaps xmmword ptr [rbp + 0x80], xmm0
14130f50a: movups xmm0, xmmword ptr [rbp]
14130f50e: movaps xmmword ptr [rbp + 0x70], xmm0
14130f512: movups xmm0, xmmword ptr [rbp - 0x10]
14130f516: movaps xmmword ptr [rbp + 0x60], xmm0
14130f51a: movups xmm0, xmmword ptr [rbp - 0x50]
14130f51e: movups xmm1, xmmword ptr [rbp - 0x40]
14130f522: movups xmm2, xmmword ptr [rbp - 0x30]
14130f526: movups xmm3, xmmword ptr [rbp - 0x20]
14130f52a: movaps xmmword ptr [rbp + 0x50], xmm3
14130f52e: movaps xmmword ptr [rbp + 0x40], xmm2
14130f532: movaps xmmword ptr [rbp + 0x30], xmm1
14130f536: movaps xmmword ptr [rbp + 0x20], xmm0
14130f53a: movzx eax, byte ptr [rip + 0x23256c8]
14130f541: mov ecx, 0x1c0
14130f546: mov edx, 8
14130f54b: call 0x140613c10
14130f550: test rax, rax
14130f553: je 0x14130f5c6
14130f555: movaps xmm0, xmmword ptr [rbp + 0x80]
14130f55c: movups xmmword ptr [rax + 0x60], xmm0
14130f560: movaps xmm0, xmmword ptr [rbp + 0x70]
14130f564: movups xmmword ptr [rax + 0x50], xmm0
14130f568: movaps xmm0, xmmword ptr [rbp + 0x60]
14130f56c: movups xmmword ptr [rax + 0x40], xmm0
14130f570: movaps xmm0, xmmword ptr [rbp + 0x20]
14130f574: movaps xmm1, xmmword ptr [rbp + 0x30]
14130f578: movaps xmm2, xmmword ptr [rbp + 0x40]
14130f57c: movaps xmm3, xmmword ptr [rbp + 0x50]
14130f580: movups xmmword ptr [rax + 0x30], xmm3
14130f584: movups xmmword ptr [rax + 0x20], xmm2
14130f588: movups xmmword ptr [rax + 0x10], xmm1
14130f58c: movups xmmword ptr [rax], xmm0
14130f58f: mov qword ptr [rbp + 0x90], 4
14130f59a: mov qword ptr [rbp + 0x98], rax
14130f5a1: mov qword ptr [rbp + 0xa0], 1
14130f5ac: movzx eax, r14b
14130f5b0: mov r15b, 2
14130f5b3: lea ecx, [rax - 1]
14130f5b6: lea r14, [rip + 0x1929b9f]
14130f5bd: movsxd rcx, dword ptr [r14 + rcx*4]
14130f5c1: add rcx, r14
14130f5c4: jmp rcx
14130f5c6: mov ecx, 8
14130f5cb: mov edx, 0x1c0
14130f5d0: mov r8, rdi
14130f5d3: call 0x1428d8fe3
14130f5d8: ud2 
14130f5da: mov r15b, 3
14130f5dd: jmp 0x14130f6f0
14130f5e2: mov r15b, 4
14130f5e5: jmp 0x14130f6f0
14130f5ea: mov r15b, 5
14130f5ed: jmp 0x14130f6f0
14130f5f2: mov r15b, 6
14130f5f5: jmp 0x14130f6f0
14130f5fa: mov r15b, 7
14130f5fd: jmp 0x14130f6f0
14130f602: mov r15b, 8
14130f605: jmp 0x14130f6f0
14130f60a: mov r15b, 9
14130f60d: jmp 0x14130f6f0
14130f612: mov r15b, 0xa
14130f615: jmp 0x14130f6f0
14130f61a: mov r15b, 0xb
14130f61d: jmp 0x14130f6f0
14130f622: mov r15b, 0xc
14130f625: jmp 0x14130f6f0
14130f62a: mov r15b, 0xd
14130f62d: jmp 0x14130f6f0
14130f632: mov r15b, 0xe
14130f635: jmp 0x14130f6f0
14130f63a: mov r15b, 0xf
14130f63d: jmp 0x14130f6f0
14130f642: mov r15b, 0x10
14130f645: jmp 0x14130f6f0
14130f64a: mov r15b, 0x11
14130f64d: jmp 0x14130f6f0
14130f652: mov r15b, 0x12
14130f655: jmp 0x14130f6f0
14130f65a: mov r15b, 0x13
14130f65d: jmp 0x14130f6f0
14130f662: mov r15b, 0x14
14130f665: jmp 0x14130f6f0
14130f66a: mov r15b, 0x15
14130f66d: jmp 0x14130f6f0
14130f672: mov r15b, 0x16
14130f675: jmp 0x14130f6f0
14130f677: mov r15b, 0x17
14130f67a: jmp 0x14130f6f0
14130f67c: mov r15b, 0x18
14130f67f: jmp 0x14130f6f0
14130f681: mov r15b, 0x19
14130f684: jmp 0x14130f6f0
14130f686: mov r15b, 0x1a
14130f689: jmp 0x14130f6f0
14130f68b: mov r15b, 0x1b
14130f68e: jmp 0x14130f6f0
14130f690: mov r15b, 0x1c
14130f693: jmp 0x14130f6f0
14130f695: mov r15b, 0x1d
14130f698: jmp 0x14130f6f0
14130f69a: mov r15b, 0x1e
14130f69d: jmp 0x14130f6f0
14130f69f: mov r15b, 0x1f
14130f6a2: jmp 0x14130f6f0
14130f6a4: mov r15b, 0x20
14130f6a7: jmp 0x14130f6f0
14130f6a9: mov r15b, 0x21
14130f6ac: jmp 0x14130f6f0
14130f6ae: mov r15b, 0x22
14130f6b1: jmp 0x14130f6f0
14130f6b3: mov r15b, 0x23
14130f6b6: jmp 0x14130f6f0
14130f6b8: mov r15b, 0x24
14130f6bb: jmp 0x14130f6f0
14130f6bd: mov r15b, 0x25
14130f6c0: jmp 0x14130f6f0
14130f6c2: mov r15b, 0x26
14130f6c5: jmp 0x14130f6f0
14130f6c7: mov r15b, 0x27
14130f6ca: jmp 0x14130f6f0
14130f6cc: mov r15b, 0x28
14130f6cf: jmp 0x14130f6f0
14130f6d1: mov r15b, 0x29
14130f6d4: jmp 0x14130f6f0
14130f6d6: mov r15b, 0x2a
14130f6d9: jmp 0x14130f6f0
14130f6db: mov r15b, 0x2b
14130f6de: jmp 0x14130f6f0
14130f6e0: mov r15b, 0x2c
14130f6e3: jmp 0x14130f6f0
14130f6e5: mov r15b, 0x2d
14130f6e8: nop dword ptr [rax + rax]
14130f6f0: mov byte ptr [rbp + 0x20], al
14130f6f3: mov rcx, qword ptr [rip + 0x231cc76]
14130f6fa: lea r8, [rip + 0x195065f]
14130f701: lea rdx, [rbp + 0x20]
14130f705: call 0x140b218a0
14130f70a: mov r8, rdx
14130f70d: lea rcx, [rbp - 0x50]
14130f711: mov rdx, rax
14130f714: call 0x140cdd670
14130f719: cmp qword ptr [rbp - 0x50], rbx
14130f71d: je 0x14130f7fb
14130f723: movups xmm0, xmmword ptr [rbp + 0x10]
14130f727: movaps xmmword ptr [rbp + 0x80], xmm0
14130f72e: movups xmm0, xmmword ptr [rbp]
14130f732: movaps xmmword ptr [rbp + 0x70], xmm0
14130f736: movups xmm0, xmmword ptr [rbp - 0x10]
14130f73a: movaps xmmword ptr [rbp + 0x60], xmm0
14130f73e: movups xmm0, xmmword ptr [rbp - 0x50]
14130f742: movups xmm1, xmmword ptr [rbp - 0x40]
14130f746: movups xmm2, xmmword ptr [rbp - 0x30]
14130f74a: movups xmm3, xmmword ptr [rbp - 0x20]
14130f74e: movaps xmmword ptr [rbp + 0x50], xmm3
14130f752: movaps xmmword ptr [rbp + 0x40], xmm2
14130f756: movaps xmmword ptr [rbp + 0x30], xmm1
14130f75a: movaps xmmword ptr [rbp + 0x20], xmm0
14130f75e: mov rdi, qword ptr [rbp + 0xa0]
14130f765: cmp rdi, qword ptr [rbp + 0x90]
14130f76c: jne 0x14130f792
14130f76e: mov qword ptr [rsp + 0x20], 0x70
14130f777: lea rcx, [rbp + 0x90]
14130f77e: mov r8d, 1
14130f784: mov r9d, 8
14130f78a: mov rdx, rdi
14130f78d: call 0x1428a9a60
14130f792: mov rax, qword ptr [rbp + 0x98]
14130f799: imul rcx, rdi, 0x70
14130f79d: movaps xmm0, xmmword ptr [rbp + 0x20]
14130f7a1: movaps xmm1, xmmword ptr [rbp + 0x30]
14130f7a5: movaps xmm2, xmmword ptr [rbp + 0x40]
14130f7a9: movaps xmm3, xmmword ptr [rbp + 0x50]
14130f7ad: movaps xmm4, xmmword ptr [rbp + 0x60]
14130f7b1: movaps xmm5, xmmword ptr [rbp + 0x70]
14130f7b5: movaps xmm6, xmmword ptr [rbp + 0x80]
14130f7bc: movups xmmword ptr [rax + rcx + 0x60], xmm6
14130f7c1: movups xmmword ptr [rax + rcx + 0x50], xmm5
14130f7c6: movups xmmword ptr [rax + rcx + 0x40], xmm4
14130f7cb: movups xmmword ptr [rax + rcx + 0x30], xmm3
14130f7d0: movups xmmword ptr [rax + rcx + 0x20], xmm2
14130f7d5: movups xmmword ptr [rax + rcx + 0x10], xmm1
14130f7da: movups xmmword ptr [rax + rcx], xmm0
14130f7de: inc rdi
14130f7e1: mov qword ptr [rbp + 0xa0], rdi
14130f7e8: movzx eax, r15b
14130f7ec: lea ecx, [rax - 1]
14130f7ef: movsxd rcx, dword ptr [r14 + rcx*4]
14130f7f3: add rcx, r14
14130f7f6: mov r15b, 2
14130f7f9: jmp rcx
14130f7fb: mov rax, qword ptr [rbp + 0xa0]
14130f802: mov qword ptr [rsi + 0x10], rax
14130f806: mov rax, qword ptr [rbp + 0x90]
14130f80d: mov qword ptr [rsi], rax
14130f810: mov rax, qword ptr [rbp + 0x98]
14130f817: mov qword ptr [rsi + 8], rax
14130f81b: mov rax, rsi
14130f81e: movaps xmm6, xmmword ptr [rbp + 0xb0]
14130f825: add rsp, 0x148
14130f82c: pop rbx
14130f82d: pop rdi
14130f82e: pop rsi
14130f82f: pop r14
14130f831: pop r15
14130f833: pop rbp
14130f834: ret 
14130f835: nop word ptr cs:[rax + rax]
14130f840: mov qword ptr [rsp + 0x10], rdx
14130f845: push rbp
14130f846: push r15
14130f848: push r14
14130f84a: push rsi
14130f84b: push rdi
14130f84c: push rbx
14130f84d: sub rsp, 0x38
14130f851: lea rbp, [rdx + 0x80]
14130f858: movaps xmmword ptr [rsp + 0x20], xmm6
14130f85d: lea rcx, [rbp + 0x20]
14130f861: call 0x1400d4bd0
14130f866: movaps xmm6, xmmword ptr [rsp + 0x20]
14130f86b: add rsp, 0x38
14130f86f: pop rbx
14130f870: pop rdi
14130f871: pop rsi
14130f872: pop r14
14130f874: pop r15
14130f876: pop rbp
14130f877: ret 
14130f878: nop dword ptr [rax + rax]
14130f880: mov qword ptr [rsp + 0x10], rdx
14130f885: push rbp
14130f886: push r15
14130f888: push r14
14130f88a: push rsi
14130f88b: push rdi
14130f88c: push rbx
14130f88d: sub rsp, 0x38
14130f891: lea rbp, [rdx + 0x80]
14130f898: movaps xmmword ptr [rsp + 0x20], xmm6
14130f89d: lea rcx, [rbp + 0x20]
14130f8a1: call 0x1400d4bd0
14130f8a6: movaps xmm6, xmmword ptr [rsp + 0x20]
14130f8ab: add rsp, 0x38
14130f8af: pop rbx
14130f8b0: pop rdi
14130f8b1: pop rsi
14130f8b2: pop r14
14130f8b4: pop r15
14130f8b6: pop rbp
14130f8b7: ret 
14130f8b8: nop dword ptr [rax + rax]
14130f8c0: mov qword ptr [rsp + 0x10], rdx
14130f8c5: push rbp
14130f8c6: push r15
14130f8c8: push r14
14130f8ca: push rsi
14130f8cb: push rdi
14130f8cc: push rbx
14130f8cd: sub rsp, 0x38
14130f8d1: lea rbp, [rdx + 0x80]
14130f8d8: movaps xmmword ptr [rsp + 0x20], xmm6
14130f8dd: lea rcx, [rbp + 0x90]
14130f8e4: call 0x1405e04e0
14130f8e9: movaps xmm6, xmmword ptr [rsp + 0x20]
14130f8ee: add rsp, 0x38
14130f8f2: pop rbx
14130f8f3: pop rdi
14130f8f4: pop rsi
14130f8f5: pop r14
14130f8f7: pop r15
14130f8f9: pop rbp
14130f8fa: ret 
