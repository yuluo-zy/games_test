1412bd5f0: push rbp
1412bd5f1: push rsi
1412bd5f2: sub rsp, 0xd8
1412bd5f9: lea rbp, [rsp + 0x80]
1412bd601: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bd609: mov rsi, rcx
1412bd60c: lea rcx, [rbp - 0x58]
1412bd610: xor r8d, r8d
1412bd613: call 0x1416148e0
1412bd618: movzx eax, byte ptr [rip + 0x23775ea]
1412bd61f: mov ecx, 0x28
1412bd624: mov edx, 8
1412bd629: call 0x140613c10
1412bd62e: test rax, rax
1412bd631: je 0x1412bd6e7
1412bd637: mov rcx, qword ptr [rbp - 0x38]
1412bd63b: mov qword ptr [rax + 0x20], rcx
1412bd63f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bd643: movups xmm1, xmmword ptr [rbp - 0x48]
1412bd647: movups xmmword ptr [rax + 0x10], xmm1
1412bd64b: movups xmmword ptr [rax], xmm0
1412bd64e: lea rcx, [rip + 0x17e1707]
1412bd655: mov qword ptr [rbp], rcx
1412bd659: mov qword ptr [rbp + 8], 0x48
1412bd661: mov qword ptr [rbp + 0x10], rax
1412bd665: lea rax, [rip + 0x196f0d4]
1412bd66c: mov qword ptr [rbp + 0x18], rax
1412bd670: mov qword ptr [rbp + 0x30], 0
1412bd678: mov byte ptr [rbp + 0x46], 5
1412bd67c: mov qword ptr [rbp + 0x20], rcx
1412bd680: mov qword ptr [rbp + 0x28], 0x48
1412bd688: movzx eax, byte ptr [rip + 0x237757a]
1412bd68f: mov ecx, 0x40
1412bd694: mov edx, 8
1412bd699: call 0x140613c10
1412bd69e: test rax, rax
1412bd6a1: je 0x1412bd6f8
1412bd6a3: movups xmm0, xmmword ptr [rbp + 0x10]
1412bd6a7: movups xmm1, xmmword ptr [rbp + 0x20]
1412bd6ab: movups xmm2, xmmword ptr [rbp + 0x30]
1412bd6af: movups xmm3, xmmword ptr [rbp + 0x40]
1412bd6b3: movups xmmword ptr [rax + 0x30], xmm3
1412bd6b7: movups xmmword ptr [rax + 0x20], xmm2
1412bd6bb: movups xmmword ptr [rax + 0x10], xmm1
1412bd6bf: movups xmmword ptr [rax], xmm0
1412bd6c2: movabs rdx, 0x564703083181a121
1412bd6cc: mov rcx, rsi
1412bd6cf: mov r8, rax
1412bd6d2: call 0x140969bf0
1412bd6d7: cmp rax, 1
1412bd6db: je 0x1412bd709
1412bd6dd: add rsp, 0xd8
1412bd6e4: pop rsi
1412bd6e5: pop rbp
1412bd6e6: ret 
1412bd6e7: mov ecx, 8
1412bd6ec: mov edx, 0x28
1412bd6f1: call 0x1428d9000
1412bd6f6: jmp 0x1412bd707
1412bd6f8: mov ecx, 8
1412bd6fd: mov edx, 0x40
1412bd702: call 0x1428d9000
1412bd707: ud2 
1412bd709: mov rax, rdx
1412bd70c: add rdx, 0x10
1412bd710: mov rsi, rdx
1412bd713: cmp qword ptr [rax + 0x18], 0x48
1412bd718: jne 0x1412bd796
1412bd71a: mov rdx, qword ptr [rsi]
1412bd71d: lea rcx, [rip + 0x17e1638]
1412bd724: mov r8d, 0x48
1412bd72a: call 0x142923bf0
1412bd72f: test eax, eax
1412bd731: jne 0x1412bd796
1412bd733: lea rax, [rip + 0x19706ae]
1412bd73a: mov qword ptr [rbp - 0x30], rax
1412bd73e: lea rax, [rip - 0xd89ea5]
1412bd745: mov qword ptr [rbp - 0x28], rax
1412bd749: mov rax, rbp
1412bd74c: mov qword ptr [rbp - 0x20], rax
1412bd750: lea rax, [rip - 0x11b38c7]
1412bd757: mov qword ptr [rbp - 0x18], rax
1412bd75b: lea rax, [rip + 0x196fec6]
1412bd762: mov qword ptr [rbp + 0x10], rax
1412bd766: mov qword ptr [rbp + 0x18], 3
1412bd76e: mov qword ptr [rbp + 0x30], 0
1412bd776: lea rax, [rbp - 0x30]
1412bd77a: mov qword ptr [rbp + 0x20], rax
1412bd77e: mov qword ptr [rbp + 0x28], 2
1412bd786: lea rdx, [rip + 0x196fecb]
1412bd78d: lea rcx, [rbp + 0x10]
1412bd791: call 0x1428d9390
1412bd796: lea rax, [rip + 0x197064b]
1412bd79d: mov qword ptr [rbp + 0x10], rax
1412bd7a1: lea rax, [rip - 0xd89f08]
1412bd7a8: mov qword ptr [rbp + 0x18], rax
1412bd7ac: mov qword ptr [rbp + 0x20], rsi
1412bd7b0: lea rax, [rip - 0x11b3927]
1412bd7b7: mov qword ptr [rbp + 0x28], rax
1412bd7bb: mov rcx, rbp
1412bd7be: mov qword ptr [rbp + 0x30], rcx
1412bd7c2: mov qword ptr [rbp + 0x38], rax
1412bd7c6: lea rax, [rip + 0x196fd9b]
1412bd7cd: mov qword ptr [rbp - 0x30], rax
1412bd7d1: mov qword ptr [rbp - 0x28], 3
1412bd7d9: mov qword ptr [rbp - 0x10], 0
1412bd7e1: lea rax, [rbp + 0x10]
1412bd7e5: mov qword ptr [rbp - 0x20], rax
1412bd7e9: mov qword ptr [rbp - 0x18], 3
1412bd7f1: lea rdx, [rip + 0x196fdd8]
1412bd7f8: lea rcx, [rbp - 0x30]
1412bd7fc: call 0x1428d9390
1412bd801: int3 
1412bd802: nop word ptr cs:[rax + rax]
1412bd810: mov qword ptr [rsp + 0x10], rdx
1412bd815: push rbp
1412bd816: push rsi
1412bd817: sub rsp, 0x28
1412bd81b: lea rbp, [rdx + 0x80]
1412bd822: lea rcx, [rbp - 0x58]
1412bd826: call 0x140be24b0
1412bd82b: nop 
1412bd82c: add rsp, 0x28
1412bd830: pop rsi
1412bd831: pop rbp
1412bd832: ret 
1412bd833: nop word ptr cs:[rax + rax]
1412bd840: mov qword ptr [rsp + 0x10], rdx
1412bd845: push rbp
1412bd846: push rsi
1412bd847: sub rsp, 0x28
1412bd84b: lea rbp, [rdx + 0x80]
1412bd852: lea rcx, [rbp + 0x10]
1412bd856: call 0x1400c04b0
1412bd85b: nop 
1412bd85c: add rsp, 0x28
1412bd860: pop rsi
1412bd861: pop rbp
1412bd862: ret 
