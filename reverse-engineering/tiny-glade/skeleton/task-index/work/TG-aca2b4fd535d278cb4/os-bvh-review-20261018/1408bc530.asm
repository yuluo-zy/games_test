1408bc530: push rbp
1408bc531: push r14
1408bc533: push rsi
1408bc534: push rdi
1408bc535: push rbx
1408bc536: sub rsp, 0x40
1408bc53a: lea rbp, [rsp + 0x40]
1408bc53f: mov qword ptr [rbp - 8], 0xfffffffffffffffe
1408bc547: mov rdi, qword ptr [rcx]
1408bc54a: lea rsi, [rdi + 0x10]
1408bc54e: mov eax, dword ptr [rdi + 0x10]
1408bc551: cmp eax, 0x3ffffffd
1408bc556: ja 0x1408bc5b2
1408bc558: lea ecx, [rax + 1]
1408bc55b: lock cmpxchg dword ptr [rsi], ecx
1408bc55f: jne 0x1408bc5b2
1408bc561: movzx eax, byte ptr [rdi + 0x18]
1408bc565: test al, al
1408bc567: jne 0x1408bc5c2
1408bc569: movzx r14d, byte ptr [rdi + 0xa0]
1408bc571: mov rbx, qword ptr [rdi + 0x20]
1408bc575: movzx eax, byte ptr [rdi + 0xa8]
1408bc57c: mov edx, 0xffffffff
1408bc581: lock xadd dword ptr [rdi + 0x10], edx
1408bc586: dec edx
1408bc588: mov ecx, edx
1408bc58a: and ecx, 0xbfffffff
1408bc590: neg ecx
1408bc592: jo 0x1408bc5f8
1408bc594: cmp r14b, 2
1408bc598: sete cl
1408bc59b: neg rbx
1408bc59e: seto dl
1408bc5a1: and dl, cl
1408bc5a3: xor al, 1
1408bc5a5: and al, dl
1408bc5a7: add rsp, 0x40
1408bc5ab: pop rbx
1408bc5ac: pop rdi
1408bc5ad: pop rsi
1408bc5ae: pop r14
1408bc5b0: pop rbp
1408bc5b1: ret 
1408bc5b2: mov rcx, rsi
1408bc5b5: call 0x1428d8a30
1408bc5ba: movzx eax, byte ptr [rdi + 0x18]
1408bc5be: test al, al
1408bc5c0: je 0x1408bc569
1408bc5c2: add rdi, 0x20
1408bc5c6: mov qword ptr [rbp - 0x18], rdi
1408bc5ca: mov qword ptr [rbp - 0x10], rsi
1408bc5ce: lea rax, [rip + 0x21e43eb]
1408bc5d5: mov qword ptr [rsp + 0x20], rax
1408bc5da: lea rcx, [rip + 0x21e3dd7]
1408bc5e1: lea r9, [rip + 0x21e3e00]
1408bc5e8: lea r8, [rbp - 0x18]
1408bc5ec: mov edx, 0x2b
1408bc5f1: call 0x1428d9760
1408bc5f6: ud2 
1408bc5f8: mov rcx, rsi
1408bc5fb: mov esi, eax
1408bc5fd: call 0x1428d8d40
1408bc602: mov eax, esi
1408bc604: jmp 0x1408bc594
1408bc606: nop word ptr cs:[rax + rax]
1408bc610: mov qword ptr [rsp + 0x10], rdx
1408bc615: push rbp
1408bc616: push r14
1408bc618: push rsi
1408bc619: push rdi
1408bc61a: push rbx
1408bc61b: sub rsp, 0x30
1408bc61f: lea rbp, [rdx + 0x40]
1408bc623: mov rcx, qword ptr [rbp - 0x10]
1408bc627: mov edx, 0xffffffff
1408bc62c: lock xadd dword ptr [rcx], edx
1408bc630: dec edx
1408bc632: mov eax, edx
1408bc634: and eax, 0xbfffffff
1408bc639: neg eax
1408bc63b: jo 0x1408bc648
1408bc63d: add rsp, 0x30
1408bc641: pop rbx
1408bc642: pop rdi
1408bc643: pop rsi
1408bc644: pop r14
1408bc646: pop rbp
1408bc647: ret 
1408bc648: call 0x1428d8d40
1408bc64d: jmp 0x1408bc63d
