1409ebe80: 4883ec28 sub rsp, 0x28
1409ebe84: 488b4110 mov rax, qword ptr [rcx + 0x10]
1409ebe88: 4839d0 cmp rax, rdx
1409ebe8b: 7677 jbe 0x1409ebf04
1409ebe8d: 488b4108 mov rax, qword ptr [rcx + 8]
1409ebe91: 488d0c52 lea rcx, [rdx + rdx*2]
1409ebe95: 488b44c808 mov rax, qword ptr [rax + rcx*8 + 8]
1409ebe9a: 4889c1 mov rcx, rax
1409ebe9d: 48d1e9 shr rcx, 1
1409ebea0: 48ba5555555555555555 movabs rdx, 0x5555555555555555
1409ebeaa: 4821ca and rdx, rcx
1409ebead: 4829d0 sub rax, rdx
1409ebeb0: 48b93333333333333333 movabs rcx, 0x3333333333333333
1409ebeba: 4889c2 mov rdx, rax
1409ebebd: 4821ca and rdx, rcx
1409ebec0: 48c1e802 shr rax, 2
1409ebec4: 4821c8 and rax, rcx
1409ebec7: 4801d0 add rax, rdx
1409ebeca: 4889c1 mov rcx, rax
1409ebecd: 48c1e904 shr rcx, 4
1409ebed1: 4801c1 add rcx, rax
1409ebed4: 48b80f0f0f0f0f0f0f0f movabs rax, 0xf0f0f0f0f0f0f0f
1409ebede: 4821c8 and rax, rcx
1409ebee1: 48b90101010101010101 movabs rcx, 0x101010101010101
1409ebeeb: 480fafc8 imul rcx, rax
1409ebeef: 48c1e938 shr rcx, 0x38
1409ebef3: f30f2ac1 cvtsi2ss xmm0, ecx
1409ebef7: f30f5905018e0d02 mulss xmm0, dword ptr [rip + 0x20d8e01]
1409ebeff: 4883c428 add rsp, 0x28
1409ebf03: c3 ret 
1409ebf04: 4c8d0515930d02 lea r8, [rip + 0x20d9315]
1409ebf0b: 4889d1 mov rcx, rdx
1409ebf0e: 4889c2 mov rdx, rax
1409ebf11: e802d6ee01 call 0x1428d9518 ; core::panicking::panic_bounds_check
1409ebf16: cc int3 
