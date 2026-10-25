1408ca1c0: push rbp
1408ca1c1: push r14
1408ca1c3: push rsi
1408ca1c4: push rdi
1408ca1c5: push rbx
1408ca1c6: sub rsp, 0xf0
1408ca1cd: lea rbp, [rsp + 0x80]
1408ca1d5: mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
1408ca1dd: mov rdi, rdx
1408ca1e0: mov rsi, rcx
1408ca1e3: movzx eax, byte ptr [rip + 0x2d6aa1f] ; 0x143634c09 b''
1408ca1ea: mov ecx, 0x13
1408ca1ef: mov edx, 1
1408ca1f4: call 0x140613c10
1408ca1f9: test rax, rax
1408ca1fc: je 0x1408ca52a
1408ca202: mov r14, rax
1408ca205: movups xmm0, xmmword ptr [rip + 0x21d9026] ; 0x142aa3232 b'/gauss_blur.cs.hlsl\x00\x00\x00\xb1.\xaaB\x01\x00\x00\x00V\x00'
1408ca20c: movups xmmword ptr [rax], xmm0
1408ca20f: mov dword ptr [rax + 0xf], 0x6c736c68
1408ca216: movzx eax, byte ptr [rip + 0x2d6a9ec] ; 0x143634c09 b''
1408ca21d: mov ecx, 7
1408ca222: mov edx, 1
1408ca227: call 0x140613c10
1408ca22c: test rax, rax
1408ca22f: je 0x1408ca540
1408ca235: mov rbx, rax
1408ca238: mov eax, dword ptr [rip + 0x21fa345] ; 0x142ac4583 b'mainFailed opening compiled shad'
1408ca23e: mov dword ptr [rbx + 3], eax
1408ca241: mov eax, dword ptr [rip + 0x21fa339] ; 0x142ac4580 b'cs_mainFailed opening compiled s'
1408ca247: mov dword ptr [rbx], eax
1408ca249: mov qword ptr [rbp + 0x18], 0x13
1408ca251: mov qword ptr [rbp + 0x20], r14
1408ca255: mov qword ptr [rbp + 0x28], 0x13
1408ca25d: mov qword ptr [rbp + 0x30], 7
1408ca265: mov qword ptr [rbp + 0x38], rbx
1408ca269: mov qword ptr [rbp + 0x40], 7
1408ca271: mov qword ptr [rbp + 0x48], 0
1408ca279: mov qword ptr [rbp + 0x50], 8
1408ca281: mov qword ptr [rbp + 0x58], 0
1408ca289: movzx eax, byte ptr [rip + 0x2d6a979] ; 0x143634c09 b''
1408ca290: mov ecx, 7
1408ca295: mov edx, 1
1408ca29a: call 0x140613c10
1408ca29f: test rax, rax
1408ca2a2: je 0x1408ca55c
1408ca2a8: mov r14, rax
1408ca2ab: mov dword ptr [rax + 3], 0x6e69616d
1408ca2b2: mov dword ptr [rax], 0x6d5f7363
1408ca2b8: mov edx, 7
1408ca2bd: mov r8d, 1
1408ca2c3: mov rcx, rbx
1408ca2c6: call 0x140613c20
1408ca2cb: mov qword ptr [rbp + 0x30], 7
1408ca2d3: mov qword ptr [rbp + 0x38], r14
1408ca2d7: mov qword ptr [rbp + 0x40], 7
1408ca2df: movups xmm0, xmmword ptr [rbp + 0x18]
1408ca2e3: movups xmm1, xmmword ptr [rbp + 0x48]
1408ca2e7: movaps xmmword ptr [rbp - 0x30], xmm0
1408ca2eb: movaps xmmword ptr [rbp], xmm1
1408ca2ef: mov rax, qword ptr [rbp + 0x58]
1408ca2f3: mov qword ptr [rbp + 0x10], rax
1408ca2f7: mov rax, qword ptr [rbp + 0x28]
1408ca2fb: mov qword ptr [rbp - 0x20], rax
1408ca2ff: mov rax, qword ptr [rbp + 0x30]
1408ca303: mov qword ptr [rbp - 0x18], rax
1408ca307: mov rax, qword ptr [rbp + 0x38]
1408ca30b: mov qword ptr [rbp - 0x10], rax
1408ca30f: mov rax, qword ptr [rbp + 0x40]
1408ca313: mov qword ptr [rbp - 8], rax
1408ca317: movzx eax, byte ptr [rip + 0x2d6a8eb] ; 0x143634c09 b''
1408ca31e: mov ecx, 8
1408ca323: mov edx, 1
1408ca328: call 0x140613c10
1408ca32d: test rax, rax
1408ca330: je 0x1408ca574
1408ca336: movabs rcx, 0x746d665f4d524550
1408ca340: mov qword ptr [rbp + 0x60], rax
1408ca344: mov qword ptr [rax], rcx
1408ca347: mov qword ptr [rbp - 0x38], rdi
1408ca34b: lea rax, [rbp - 0x38]
1408ca34f: mov qword ptr [rbp - 0x48], rax
1408ca353: lea rax, [rip - 0x396a2a] ; 0x140533930 b'I\x89\xd0H\x8b\x01H\x8b\x08H\x8bP\x08\xe9\xaeM\x85\x00\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc\xcc'
1408ca35a: mov qword ptr [rbp - 0x40], rax
1408ca35e: lea rax, [rip + 0x21eb32b] ; 0x142ab5690 b'\x88V\xabB\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00\x88V\xabB\x01\x00\x00\x00\x01\x00\x00\x00\x00\x00\x00\x00'
1408ca365: mov qword ptr [rbp + 0x18], rax
1408ca369: mov qword ptr [rbp + 0x20], 2
1408ca371: lea rax, [rbp - 0x48]
1408ca375: mov qword ptr [rbp + 0x28], rax
1408ca379: mov qword ptr [rbp + 0x30], 1
1408ca381: mov qword ptr [rbp + 0x38], 0
1408ca389: lea rcx, [rbp - 0x60]
1408ca38d: lea rdx, [rbp + 0x18]
1408ca391: call 0x140d79f90
1408ca396: lea rbx, [rbp]
1408ca39a: mov qword ptr [rbp + 0x18], 8
1408ca3a2: mov rax, qword ptr [rbp + 0x60]
1408ca3a6: mov qword ptr [rbp + 0x20], rax
1408ca3aa: mov qword ptr [rbp + 0x28], 8
1408ca3b2: movups xmm0, xmmword ptr [rbp - 0x60]
1408ca3b6: movups xmmword ptr [rbp + 0x30], xmm0
1408ca3ba: mov rax, qword ptr [rbp - 0x50]
1408ca3be: mov qword ptr [rbp + 0x40], rax
1408ca3c2: mov r14, qword ptr [rbp + 0x10]
1408ca3c6: cmp r14, qword ptr [rbp]
1408ca3ca: jne 0x1408ca3db
1408ca3cc: lea rdx, [rip + 0x21d8e75] ; 0x142aa3248 b'\xb1.\xaaB\x01\x00\x00\x00V\x00\x00\x00\x00\x00\x00\x003\x00\x00\x00\x01\x00\x00\x00\xdb\x0fI?\x00\x00\x80('
1408ca3d3: mov rcx, rbx
1408ca3d6: call 0x1416a7d30
1408ca3db: mov rax, qword ptr [rbp + 8]
1408ca3df: lea rcx, [r14 + r14*2]
1408ca3e3: shl rcx, 4
1408ca3e7: movups xmm0, xmmword ptr [rbp + 0x18]
1408ca3eb: movups xmm1, xmmword ptr [rbp + 0x28]
1408ca3ef: movups xmm2, xmmword ptr [rbp + 0x38]
1408ca3f3: movups xmmword ptr [rax + rcx + 0x20], xmm2
1408ca3f8: movups xmmword ptr [rax + rcx + 0x10], xmm1
1408ca3fd: movups xmmword ptr [rax + rcx], xmm0
1408ca401: inc r14
1408ca404: mov qword ptr [rbp + 0x10], r14
1408ca408: movzx eax, byte ptr [rip + 0x2d6a7fa] ; 0x143634c09 b''
1408ca40f: mov ecx, 7
1408ca414: mov edx, 1
1408ca419: call 0x140613c10
1408ca41e: test rax, rax
1408ca421: je 0x1408ca58c
1408ca427: mov r14, rax
1408ca42a: mov dword ptr [rax + 3], 0x79745f4d
1408ca431: mov dword ptr [rax], 0x4d524550
1408ca437: cmp byte ptr [rdi + 0x10], 0
1408ca43b: movzx eax, byte ptr [rip + 0x2d6a7c7] ; 0x143634c09 b''
1408ca442: je 0x1408ca469
1408ca444: mov edi, 6
1408ca449: mov ecx, 6
1408ca44e: mov edx, 1
1408ca453: call 0x140613c10
1408ca458: test rax, rax
1408ca45b: je 0x1408ca5a4
1408ca461: mov word ptr [rax + 4], 0x3474
1408ca467: jmp 0x1408ca48a
1408ca469: mov edi, 5
1408ca46e: mov ecx, 5
1408ca473: mov edx, 1
1408ca478: call 0x140613c10
1408ca47d: test rax, rax
1408ca480: je 0x1408ca5c0
1408ca486: mov byte ptr [rax + 4], 0x74
1408ca48a: mov dword ptr [rax], 0x616f6c66
1408ca490: mov qword ptr [rbp + 0x18], 7
1408ca498: mov qword ptr [rbp + 0x20], r14
1408ca49c: mov qword ptr [rbp + 0x28], 7
1408ca4a4: mov qword ptr [rbp + 0x30], rdi
1408ca4a8: mov qword ptr [rbp + 0x38], rax
1408ca4ac: mov qword ptr [rbp + 0x40], rdi
1408ca4b0: mov rdi, qword ptr [rbp + 0x10]
1408ca4b4: cmp rdi, qword ptr [rbp]
1408ca4b8: jne 0x1408ca4c9
1408ca4ba: lea rdx, [rip + 0x21d8d87] ; 0x142aa3248 b'\xb1.\xaaB\x01\x00\x00\x00V\x00\x00\x00\x00\x00\x00\x003\x00\x00\x00\x01\x00\x00\x00\xdb\x0fI?\x00\x00\x80('
1408ca4c1: mov rcx, rbx
1408ca4c4: call 0x1416a7d30
1408ca4c9: mov rax, qword ptr [rbp + 8]
1408ca4cd: lea rcx, [rdi + rdi*2]
1408ca4d1: shl rcx, 4
1408ca4d5: movups xmm0, xmmword ptr [rbp + 0x18]
1408ca4d9: movups xmm1, xmmword ptr [rbp + 0x28]
1408ca4dd: movups xmm2, xmmword ptr [rbp + 0x38]
1408ca4e1: movups xmmword ptr [rax + rcx + 0x20], xmm2
1408ca4e6: movups xmmword ptr [rax + rcx + 0x10], xmm1
1408ca4eb: movups xmmword ptr [rax + rcx], xmm0
1408ca4ef: inc rdi
1408ca4f2: mov qword ptr [rbp + 0x10], rdi
1408ca4f6: mov qword ptr [rsi + 0x40], rdi
1408ca4fa: movaps xmm0, xmmword ptr [rbp - 0x30]
1408ca4fe: movaps xmm1, xmmword ptr [rbp - 0x20]
1408ca502: movaps xmm2, xmmword ptr [rbp - 0x10]
1408ca506: movaps xmm3, xmmword ptr [rbp]
1408ca50a: movups xmmword ptr [rsi + 0x30], xmm3
1408ca50e: movups xmmword ptr [rsi + 0x20], xmm2
1408ca512: movups xmmword ptr [rsi + 0x10], xmm1
1408ca516: movups xmmword ptr [rsi], xmm0
1408ca519: mov rax, rsi
1408ca51c: add rsp, 0xf0
1408ca523: pop rbx
1408ca524: pop rdi
1408ca525: pop rsi
1408ca526: pop r14
1408ca528: pop rbp
1408ca529: ret 
1408ca52a: lea r8, [rip + 0x21fa14f] ; 0x142ac4680 b'0F\xacB\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00/home/h3'
1408ca531: mov ecx, 1
1408ca536: mov edx, 0x13
1408ca53b: call 0x1428d8fe3
1408ca540: mov qword ptr [rbp + 0x60], r14
1408ca544: lea r8, [rip + 0x21fa135] ; 0x142ac4680 b'0F\xacB\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00/home/h3'
1408ca54b: mov ecx, 1
1408ca550: mov edx, 7
1408ca555: call 0x1428d8fe3
1408ca55a: jmp 0x1408ca5da
1408ca55c: lea r8, [rip + 0x21fa11d] ; 0x142ac4680 b'0F\xacB\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00/home/h3'
1408ca563: mov ecx, 1
1408ca568: mov edx, 7
1408ca56d: call 0x1428d8fe3
1408ca572: jmp 0x1408ca5da
1408ca574: lea r8, [rip + 0x21d87c5] ; 0x142aa2d40 b'\xf0,\xaaB\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00\xe0(\x83@\x01\x00\x00\x00'
1408ca57b: mov ecx, 1
1408ca580: mov edx, 8
1408ca585: call 0x1428d8fe3
1408ca58a: jmp 0x1408ca5da
1408ca58c: lea r8, [rip + 0x21d87ad] ; 0x142aa2d40 b'\xf0,\xaaB\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00\xe0(\x83@\x01\x00\x00\x00'
1408ca593: mov ecx, 1
1408ca598: mov edx, 7
1408ca59d: call 0x1428d8fe3
1408ca5a2: jmp 0x1408ca5da
1408ca5a4: mov qword ptr [rbp + 0x60], r14
1408ca5a8: lea r8, [rip + 0x21d8791] ; 0x142aa2d40 b'\xf0,\xaaB\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00\xe0(\x83@\x01\x00\x00\x00'
1408ca5af: mov ecx, 1
1408ca5b4: mov edx, 6
1408ca5b9: call 0x1428d8fe3
1408ca5be: jmp 0x1408ca5da
1408ca5c0: mov qword ptr [rbp + 0x60], r14
1408ca5c4: lea r8, [rip + 0x21d8775] ; 0x142aa2d40 b'\xf0,\xaaB\x01\x00\x00\x00J\x00\x00\x00\x00\x00\x00\x00\xa2\x00\x00\x00\x19\x00\x00\x00\xe0(\x83@\x01\x00\x00\x00'
1408ca5cb: mov ecx, 1
1408ca5d0: mov edx, 5
1408ca5d5: call 0x1428d8fe3
1408ca5da: ud2 
1408ca5dc: nop dword ptr [rax]
1408ca5e0: mov qword ptr [rsp + 0x10], rdx
1408ca5e5: push rbp
1408ca5e6: push r14
1408ca5e8: push rsi
1408ca5e9: push rdi
1408ca5ea: push rbx
1408ca5eb: sub rsp, 0x20
1408ca5ef: lea rbp, [rdx + 0x80]
1408ca5f6: mov edx, 0x13
1408ca5fb: mov r8d, 1
1408ca601: mov rcx, qword ptr [rbp + 0x60]
1408ca605: call 0x140613c20
1408ca60a: nop 
1408ca60b: add rsp, 0x20
1408ca60f: pop rbx
1408ca610: pop rdi
1408ca611: pop rsi
1408ca612: pop r14
1408ca614: pop rbp
1408ca615: ret 
1408ca616: nop word ptr cs:[rax + rax]
1408ca620: mov qword ptr [rsp + 0x10], rdx
1408ca625: push rbp
1408ca626: push r14
1408ca628: push rsi
1408ca629: push rdi
1408ca62a: push rbx
1408ca62b: sub rsp, 0x20
1408ca62f: lea rbp, [rdx + 0x80]
1408ca636: lea rcx, [rbp + 0x18]
1408ca63a: call 0x1405fe2e0
1408ca63f: nop 
1408ca640: add rsp, 0x20
1408ca644: pop rbx
1408ca645: pop rdi
1408ca646: pop rsi
1408ca647: pop r14
1408ca649: pop rbp
1408ca64a: ret 
1408ca64b: nop dword ptr [rax + rax]
1408ca650: mov qword ptr [rsp + 0x10], rdx
1408ca655: push rbp
1408ca656: push r14
1408ca658: push rsi
1408ca659: push rdi
1408ca65a: push rbx
1408ca65b: sub rsp, 0x20
1408ca65f: lea rbp, [rdx + 0x80]
1408ca666: lea rcx, [rbp - 0x30]
1408ca66a: call 0x1405fe2e0
1408ca66f: nop 
1408ca670: add rsp, 0x20
1408ca674: pop rbx
1408ca675: pop rdi
1408ca676: pop rsi
1408ca677: pop r14
1408ca679: pop rbp
1408ca67a: ret 
1408ca67b: nop dword ptr [rax + rax]
1408ca680: mov qword ptr [rsp + 0x10], rdx
1408ca685: push rbp
1408ca686: push r14
1408ca688: push rsi
1408ca689: push rdi
1408ca68a: push rbx
1408ca68b: sub rsp, 0x20
1408ca68f: lea rbp, [rdx + 0x80]
1408ca696: mov edx, 7
1408ca69b: mov r8d, 1
1408ca6a1: mov rcx, qword ptr [rbp + 0x60]
1408ca6a5: call 0x140613c20
1408ca6aa: nop 
1408ca6ab: add rsp, 0x20
1408ca6af: pop rbx
1408ca6b0: pop rdi
1408ca6b1: pop rsi
1408ca6b2: pop r14
1408ca6b4: pop rbp
1408ca6b5: ret 
1408ca6b6: nop word ptr cs:[rax + rax]
1408ca6c0: mov qword ptr [rsp + 0x10], rdx
1408ca6c5: push rbp
1408ca6c6: push r14
1408ca6c8: push rsi
1408ca6c9: push rdi
1408ca6ca: push rbx
1408ca6cb: sub rsp, 0x20
1408ca6cf: lea rbp, [rdx + 0x80]
1408ca6d6: mov edx, 8
1408ca6db: mov r8d, 1
1408ca6e1: mov rcx, qword ptr [rbp + 0x60]
1408ca6e5: call 0x140613c20
1408ca6ea: nop 
1408ca6eb: add rsp, 0x20
1408ca6ef: pop rbx
1408ca6f0: pop rdi
1408ca6f1: pop rsi
1408ca6f2: pop r14
1408ca6f4: pop rbp
1408ca6f5: ret 
1408ca6f6: nop word ptr cs:[rax + rax]
1408ca700: mov qword ptr [rsp + 0x10], rdx
1408ca705: push rbp
1408ca706: push r14
1408ca708: push rsi
1408ca709: push rdi
1408ca70a: push rbx
1408ca70b: sub rsp, 0x20
1408ca70f: lea rbp, [rdx + 0x80]
1408ca716: lea rcx, [rbp + 0x18]
1408ca71a: call 0x1405ed460
1408ca71f: nop 
1408ca720: add rsp, 0x20
1408ca724: pop rbx
1408ca725: pop rdi
1408ca726: pop rsi
1408ca727: pop r14
1408ca729: pop rbp
1408ca72a: ret 
1408ca72b: nop dword ptr [rax + rax]
1408ca730: mov qword ptr [rsp + 0x10], rdx
1408ca735: push rbp
1408ca736: push r14
1408ca738: push rsi
1408ca739: push rdi
1408ca73a: push rbx
1408ca73b: sub rsp, 0x20
1408ca73f: lea rbp, [rdx + 0x80]
1408ca746: lea rcx, [rbp + 0x18]
1408ca74a: call 0x1405ed460
1408ca74f: nop 
1408ca750: add rsp, 0x20
1408ca754: pop rbx
1408ca755: pop rdi
1408ca756: pop rsi
1408ca757: pop r14
1408ca759: pop rbp
1408ca75a: ret 
