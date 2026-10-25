0x1408cdbd0 4883ec28 sub rsp, 0x28
0x1408cdbd4 4489c0 mov eax, r8d
0x1408cdbd7 48394110 cmp qword ptr [rcx + 0x10], rax
0x1408cdbdb 761c jbe 0x1408cdbf9
0x1408cdbdd 488b4908 mov rcx, qword ptr [rcx + 8]
0x1408cdbe1 4885c9 test rcx, rcx
0x1408cdbe4 7413 je 0x1408cdbf9
0x1408cdbe6 488d0480 lea rax, [rax + rax*4]
0x1408cdbea 3954c120 cmp dword ptr [rcx + rax*8 + 0x20], edx
0x1408cdbee 7509 jne 0x1408cdbf9
0x1408cdbf0 488d04c1 lea rax, [rcx + rax*8]
0x1408cdbf4 4883c428 add rsp, 0x28
0x1408cdbf8 c3 ret
0x1408cdbf9 488d0de0fc3902 lea rcx, [rip + 0x239fce0]
0x1408cdc00 4c8d05a9611d02 lea r8, [rip + 0x21d61a9]
0x1408cdc07 ba18000000 mov edx, 0x18
0x1408cdc0c e814ab0002 call 0x1428d8725
0x1408cdc11 cc int3
