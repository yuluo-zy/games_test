0x140afadb0 push     rsi
0x140afadb1 push     rdi
0x140afadb2 push     rbx
0x140afadb3 sub      rsp, 0x30
0x140afadb7 test     cl, cl
0x140afadb9 je       0x140afae0c
0x140afadbb mov      eax, dword ptr [rdx + 0x1c]
0x140afadbe mov      rcx, qword ptr [rdx + 0x10]
0x140afadc2 mov      dword ptr [rcx], eax
0x140afadc4 mov      rsi, qword ptr [rdx]
0x140afadc7 mov      rdi, qword ptr [rsi + 0x30]
0x140afadcb mov      rbx, qword ptr [rsi + 0x40]
0x140afadcf cmp      rdi, qword ptr [rsi + 0x20]
0x140afadd3 jne      0x140afade5
0x140afadd5 lea      rcx, [rsi + 0x20]
0x140afadd9 lea      rdx, [rip + 0x213f368]
0x140afade0 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afade5 mov      rax, qword ptr [rsi + 0x28]
0x140afade9 mov      rcx, rdi
0x140afadec shl      rcx, 5
0x140afadf0 mov      dword ptr [rax + rcx], 4
0x140afadf7 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afadfc inc      rdi
0x140afadff mov      qword ptr [rsi + 0x30], rdi
0x140afae03 inc      qword ptr [rsi + 0x40]
0x140afae07 jmp      0x140afaec1 ; 
0x140afae0c mov      eax, dword ptr [r8 + 0x1c]
0x140afae10 mov      rcx, qword ptr [r8 + 0x10]
0x140afae14 mov      dword ptr [rcx], eax
0x140afae16 mov      rcx, qword ptr [r8]
0x140afae19 cmp      qword ptr [rcx + 0x10], 3
0x140afae1e jb       0x140afaec1
0x140afae24 mov      r8, qword ptr [rcx + 8]
0x140afae28 mov      r10, qword ptr [r8 + 0x50]
0x140afae2c test     r10, r10
0x140afae2f je       0x140afaec1
0x140afae35 cmp      byte ptr [rcx + 0x18], 2
0x140afae39 jne      0x140afae55
0x140afae3b mov      rax, qword ptr [r8 + 0x58]
0x140afae3f inc      rax
0x140afae42 mov      rdx, rax
0x140afae45 or       rdx, r10
0x140afae48 shr      rdx, 0x20
0x140afae4c je       0x140afae70
0x140afae4e xor      edx, edx
0x140afae50 div      r10
0x140afae53 jmp      0x140afae75 ; 
0x140afae55 mov      rdx, qword ptr [r8 + 0x58]
0x140afae59 cmp      rdx, r10
0x140afae5c jb       0x140afae79
0x140afae5e lea      r8, [rip + 0x1fea7d3]
0x140afae65 mov      rcx, rdx
0x140afae68 mov      rdx, r10
0x140afae6b call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afae70 xor      edx, edx
0x140afae72 div      r10d
0x140afae75 mov      qword ptr [r8 + 0x58], rdx
0x140afae79 mov      r8, qword ptr [r8 + 0x48]
0x140afae7d shl      rdx, 4
0x140afae81 mov      eax, dword ptr [r8 + rdx]
0x140afae85 mov      byte ptr [rcx + 0x18], 2
0x140afae89 cmp      eax, 0x15
0x140afae8c je       0x140afaec1
0x140afae8e add      r8, rdx
0x140afae91 mov      ecx, dword ptr [r8 + 0xc]
0x140afae95 mov      dword ptr [rsp + 0x28], ecx
0x140afae99 mov      rcx, qword ptr [r8 + 4]
0x140afae9d mov      qword ptr [rsp + 0x20], rcx
0x140afaea2 mov      ecx, dword ptr [r9 + 0x1c]
0x140afaea6 mov      rdx, qword ptr [r9 + 0x10]
0x140afaeaa mov      dword ptr [rdx], ecx
0x140afaeac mov      rcx, qword ptr [r9]
0x140afaeaf mov      dword ptr [rcx], eax
0x140afaeb1 mov      rax, qword ptr [rsp + 0x20]
0x140afaeb6 mov      qword ptr [rcx + 4], rax
0x140afaeba mov      eax, dword ptr [rsp + 0x28]
0x140afaebe mov      dword ptr [rcx + 0xc], eax
0x140afaec1 add      rsp, 0x30
0x140afaec5 pop      rbx
0x140afaec6 pop      rdi
0x140afaec7 pop      rsi
0x140afaec8 ret      