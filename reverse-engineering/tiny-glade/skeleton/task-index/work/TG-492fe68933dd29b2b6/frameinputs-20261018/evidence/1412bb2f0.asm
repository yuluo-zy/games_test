1412bb2f0: push rbp
1412bb2f1: push rsi
1412bb2f2: sub rsp, 0xd8
1412bb2f9: lea rbp, [rsp + 0x80]
1412bb301: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bb309: mov rsi, rcx
1412bb30c: shl r9, 2
1412bb310: lea r9, [r9 + r9*4]
1412bb314: lea rcx, [rbp - 0x58]
1412bb318: call 0x141613940
1412bb31d: movzx eax, byte ptr [rip + 0x23798e5]
1412bb324: mov ecx, 0x28
1412bb329: mov edx, 8
1412bb32e: call 0x140613c10
1412bb333: test rax, rax
1412bb336: je 0x1412bb3ec
1412bb33c: mov rcx, qword ptr [rbp - 0x38]
1412bb340: mov qword ptr [rax + 0x20], rcx
1412bb344: movups xmm0, xmmword ptr [rbp - 0x58]
1412bb348: movups xmm1, xmmword ptr [rbp - 0x48]
1412bb34c: movups xmmword ptr [rax + 0x10], xmm1
1412bb350: movups xmmword ptr [rax], xmm0
1412bb353: lea rcx, [rip + 0x17e26f5]
1412bb35a: mov qword ptr [rbp], rcx
1412bb35e: mov qword ptr [rbp + 8], 0x35
1412bb366: mov qword ptr [rbp + 0x10], rax
1412bb36a: lea rax, [rip + 0x19710bf]
1412bb371: mov qword ptr [rbp + 0x18], rax
1412bb375: mov qword ptr [rbp + 0x30], 0
1412bb37d: mov byte ptr [rbp + 0x46], 5
1412bb381: mov qword ptr [rbp + 0x20], rcx
1412bb385: mov qword ptr [rbp + 0x28], 0x35
1412bb38d: movzx eax, byte ptr [rip + 0x2379875]
1412bb394: mov ecx, 0x40
1412bb399: mov edx, 8
1412bb39e: call 0x140613c10
1412bb3a3: test rax, rax
1412bb3a6: je 0x1412bb3fd
1412bb3a8: movups xmm0, xmmword ptr [rbp + 0x10]
1412bb3ac: movups xmm1, xmmword ptr [rbp + 0x20]
1412bb3b0: movups xmm2, xmmword ptr [rbp + 0x30]
1412bb3b4: movups xmm3, xmmword ptr [rbp + 0x40]
1412bb3b8: movups xmmword ptr [rax + 0x30], xmm3
1412bb3bc: movups xmmword ptr [rax + 0x20], xmm2
1412bb3c0: movups xmmword ptr [rax + 0x10], xmm1
1412bb3c4: movups xmmword ptr [rax], xmm0
1412bb3c7: movabs rdx, 0xd92427ff18235c77
1412bb3d1: mov rcx, rsi
1412bb3d4: mov r8, rax
1412bb3d7: call 0x140969bf0
1412bb3dc: cmp rax, 1
1412bb3e0: je 0x1412bb40e
1412bb3e2: add rsp, 0xd8
1412bb3e9: pop rsi
1412bb3ea: pop rbp
1412bb3eb: ret 
1412bb3ec: mov ecx, 8
1412bb3f1: mov edx, 0x28
1412bb3f6: call 0x1428d9000
1412bb3fb: jmp 0x1412bb40c
1412bb3fd: mov ecx, 8
1412bb402: mov edx, 0x40
1412bb407: call 0x1428d9000
1412bb40c: ud2 
1412bb40e: mov rax, rdx
1412bb411: add rdx, 0x10
1412bb415: mov rsi, rdx
1412bb418: cmp qword ptr [rax + 0x18], 0x35
1412bb41d: jne 0x1412bb49b
1412bb41f: mov rdx, qword ptr [rsi]
1412bb422: lea rcx, [rip + 0x17e2626]
1412bb429: mov r8d, 0x35
1412bb42f: call 0x142923bf0
1412bb434: test eax, eax
1412bb436: jne 0x1412bb49b
1412bb438: lea rax, [rip + 0x1972561]
1412bb43f: mov qword ptr [rbp - 0x30], rax
1412bb443: lea rax, [rip - 0xd87baa]
1412bb44a: mov qword ptr [rbp - 0x28], rax
1412bb44e: mov rax, rbp
1412bb451: mov qword ptr [rbp - 0x20], rax
1412bb455: lea rax, [rip - 0x11b15cc]
1412bb45c: mov qword ptr [rbp - 0x18], rax
1412bb460: lea rax, [rip + 0x19721c1]
1412bb467: mov qword ptr [rbp + 0x10], rax
1412bb46b: mov qword ptr [rbp + 0x18], 3
1412bb473: mov qword ptr [rbp + 0x30], 0
1412bb47b: lea rax, [rbp - 0x30]
1412bb47f: mov qword ptr [rbp + 0x20], rax
1412bb483: mov qword ptr [rbp + 0x28], 2
1412bb48b: lea rdx, [rip + 0x19721c6]
1412bb492: lea rcx, [rbp + 0x10]
1412bb496: call 0x1428d9390
1412bb49b: lea rax, [rip + 0x19724fe]
1412bb4a2: mov qword ptr [rbp + 0x10], rax
1412bb4a6: lea rax, [rip - 0xd87c0d]
1412bb4ad: mov qword ptr [rbp + 0x18], rax
1412bb4b1: mov qword ptr [rbp + 0x20], rsi
1412bb4b5: lea rax, [rip - 0x11b162c]
1412bb4bc: mov qword ptr [rbp + 0x28], rax
1412bb4c0: mov rcx, rbp
1412bb4c3: mov qword ptr [rbp + 0x30], rcx
1412bb4c7: mov qword ptr [rbp + 0x38], rax
1412bb4cb: lea rax, [rip + 0x1972096]
1412bb4d2: mov qword ptr [rbp - 0x30], rax
1412bb4d6: mov qword ptr [rbp - 0x28], 3
1412bb4de: mov qword ptr [rbp - 0x10], 0
1412bb4e6: lea rax, [rbp + 0x10]
1412bb4ea: mov qword ptr [rbp - 0x20], rax
1412bb4ee: mov qword ptr [rbp - 0x18], 3
1412bb4f6: lea rdx, [rip + 0x19720d3]
1412bb4fd: lea rcx, [rbp - 0x30]
1412bb501: call 0x1428d9390
1412bb506: int3 
1412bb507: nop word ptr [rax + rax]
1412bb510: mov qword ptr [rsp + 0x10], rdx
1412bb515: push rbp
1412bb516: push rsi
1412bb517: sub rsp, 0x28
1412bb51b: lea rbp, [rdx + 0x80]
1412bb522: lea rcx, [rbp - 0x58]
1412bb526: call 0x140be24b0
1412bb52b: nop 
1412bb52c: add rsp, 0x28
1412bb530: pop rsi
1412bb531: pop rbp
1412bb532: ret 
1412bb533: nop word ptr cs:[rax + rax]
1412bb540: mov qword ptr [rsp + 0x10], rdx
1412bb545: push rbp
1412bb546: push rsi
1412bb547: sub rsp, 0x28
1412bb54b: lea rbp, [rdx + 0x80]
1412bb552: lea rcx, [rbp + 0x10]
1412bb556: call 0x1400c04b0
1412bb55b: nop 
1412bb55c: add rsp, 0x28
1412bb560: pop rsi
1412bb561: pop rbp
1412bb562: ret 
