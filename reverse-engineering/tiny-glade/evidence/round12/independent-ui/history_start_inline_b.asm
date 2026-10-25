1414e182e: 41833e0d                 cmp dword ptr [r14], 0xd
1414e1832: 7434                     je 0x1414e1868
1414e1834: 4180791802               cmp byte ptr [r9 + 0x18], 2
1414e1839: 722d                     jb 0x1414e1868
1414e183b: 498b5120                 mov rdx, qword ptr [r9 + 0x20]
1414e183f: f682b900000001           test byte ptr [rdx + 0xb9], 1
1414e1846: 7420                     je 0x1414e1868
1414e1848: 4180791903               cmp byte ptr [r9 + 0x19], 3
1414e184d: 7519                     jne 0x1414e1868
1414e184f: 488d9424a0000000         lea rdx, [rsp + 0xa0]
1414e1857: 418932                   mov dword ptr [r10], esi
1414e185a: 4889cf                   mov rdi, rcx
1414e185d: 4889c1                   mov rcx, rax
1414e1860: e80b4a5aff               call 0x140a86270
1414e1865: 4889f9                   mov rcx, rdi
