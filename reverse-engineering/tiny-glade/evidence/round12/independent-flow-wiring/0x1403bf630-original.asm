1403bf630: push rsi
1403bf631: sub rsp, 0x80
1403bf638: mov eax, 1
1403bf63d: lock xadd dword ptr [rdx + 0x5b0], eax
1403bf645: test byte ptr [rcx], 1
1403bf648: je 0x1403bf7fa
1403bf64e: mov r8, qword ptr [rcx + 8]
1403bf652: mov r10, qword ptr [rdx + 0x218]
1403bf659: cmp r10, r8
1403bf65c: jbe 0x1403bf6f0
1403bf662: mov r11, qword ptr [rdx + 0x210]
1403bf669: mov r9, qword ptr [r11 + r8*8]
1403bf66d: test r9, r9
1403bf670: je 0x1403bf6f0
1403bf672: mov r8, qword ptr [rdx + 0x1e0]
1403bf679: not r9
1403bf67c: lea r9, [r9 + r9*2]
1403bf680: shl r9, 5
1403bf684: cmp qword ptr [r8 + r9 + 0x20], 0
1403bf68a: je 0x1403bf6f0
1403bf68c: mov rsi, qword ptr [rcx + 0x10]
1403bf690: cmp r10, rsi
1403bf693: jbe 0x1403bf775
1403bf699: mov r10, qword ptr [r11 + rsi*8]
1403bf69d: test r10, r10
1403bf6a0: je 0x1403bf775
1403bf6a6: not r10
1403bf6a9: lea r10, [r10 + r10*2]
1403bf6ad: shl r10, 5
1403bf6b1: cmp qword ptr [r8 + r10 + 0x20], 0
1403bf6b7: je 0x1403bf775
1403bf6bd: add r9, r8
1403bf6c0: mov r9, qword ptr [r9 + 0x10]
1403bf6c4: add r10, r8
1403bf6c7: mov r8, qword ptr [r10 + 0x10]
1403bf6cb: mov edx, dword ptr [rdx + 0x5b0]
1403bf6d1: cmp byte ptr [r9], 2
1403bf6d5: jne 0x1403bf6df
1403bf6d7: mov dword ptr [r10 + 0x5c], eax
1403bf6db: mov byte ptr [r8], 1
1403bf6df: mov dword ptr [rcx + 0x150], eax
1403bf6e5: xor eax, eax
1403bf6e7: add rsp, 0x80
1403bf6ee: pop rsi
1403bf6ef: ret 
1403bf6f0: add rcx, 0xb8
1403bf6f7: lea rax, [rip + 0x260e419]
1403bf6fe: mov qword ptr [rsp + 0x20], rax
1403bf703: mov qword ptr [rsp + 0x28], 0x55
1403bf70c: mov qword ptr [rsp + 0x30], rcx
1403bf711: lea rax, [rip - 0x383c48]
1403bf718: mov qword ptr [rsp + 0x38], rax
1403bf71d: lea rax, [rsp + 0x20]
1403bf722: mov qword ptr [rsp + 0x40], rax
1403bf727: lea rax, [rip - 0x2b589e]
1403bf72e: mov qword ptr [rsp + 0x48], rax
1403bf733: lea rax, [rip + 0x260e45e]
1403bf73a: mov qword ptr [rsp + 0x50], rax
1403bf73f: mov qword ptr [rsp + 0x58], 2
1403bf748: mov qword ptr [rsp + 0x70], 0
1403bf751: lea rax, [rsp + 0x30]
1403bf756: mov qword ptr [rsp + 0x60], rax
1403bf75b: mov qword ptr [rsp + 0x68], 2
1403bf764: lea rdx, [rip + 0x260e4b5]
1403bf76b: lea rcx, [rsp + 0x50]
1403bf770: call 0x1428d9390 ; core::panicking::panic_fmt
1403bf775: add rcx, 0xb8
1403bf77c: lea rax, [rip + 0x260e787]
1403bf783: mov qword ptr [rsp + 0x20], rax
1403bf788: mov qword ptr [rsp + 0x28], 0x59
1403bf791: mov qword ptr [rsp + 0x30], rcx
1403bf796: lea rax, [rip - 0x383ccd]
1403bf79d: mov qword ptr [rsp + 0x38], rax
1403bf7a2: lea rax, [rsp + 0x20]
1403bf7a7: mov qword ptr [rsp + 0x40], rax
1403bf7ac: lea rax, [rip - 0x2b5923]
1403bf7b3: mov qword ptr [rsp + 0x48], rax
1403bf7b8: lea rax, [rip + 0x260e3d9]
1403bf7bf: mov qword ptr [rsp + 0x50], rax
1403bf7c4: mov qword ptr [rsp + 0x58], 2
1403bf7cd: mov qword ptr [rsp + 0x70], 0
1403bf7d6: lea rax, [rsp + 0x30]
1403bf7db: mov qword ptr [rsp + 0x60], rax
1403bf7e0: mov qword ptr [rsp + 0x68], 2
1403bf7e9: lea rdx, [rip + 0x260e4a8]
1403bf7f0: lea rcx, [rsp + 0x50]
1403bf7f5: call 0x1428d9390 ; core::panicking::panic_fmt
1403bf7fa: lea rcx, [rip + 0x260e83f]
1403bf801: lea r8, [rip + 0x260e898]
1403bf808: mov edx, 0x59
1403bf80d: call 0x1428d9330 ; core::option::expect_failed
1403bf812: int3 
