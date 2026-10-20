141734605: 41833e0d                 cmp dword ptr [r14], 0xd
141734609: 7434                     je 0x14173463f
14173460b: 4180791802               cmp byte ptr [r9 + 0x18], 2
141734610: 722d                     jb 0x14173463f
141734612: 498b5120                 mov rdx, qword ptr [r9 + 0x20]
141734616: f682b900000001           test byte ptr [rdx + 0xb9], 1
14173461d: 7420                     je 0x14173463f
14173461f: 4180791903               cmp byte ptr [r9 + 0x19], 3
141734624: 7519                     jne 0x14173463f
141734626: 488d942490000000         lea rdx, [rsp + 0x90]
14173462e: 418932                   mov dword ptr [r10], esi
141734631: 4889cf                   mov rdi, rcx
141734634: 4889c1                   mov rcx, rax
141734637: e8341c35ff               call 0x140a86270
14173463c: 4889f9                   mov rcx, rdi
