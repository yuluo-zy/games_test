0x140afa190 push     rsi
0x140afa191 push     rdi
0x140afa192 push     rbx
0x140afa193 sub      rsp, 0x30
0x140afa197 test     cl, cl
0x140afa199 je       0x140afa1ec
0x140afa19b mov      eax, dword ptr [rdx + 0x1c]
0x140afa19e mov      rcx, qword ptr [rdx + 0x10]
0x140afa1a2 mov      dword ptr [rcx], eax
0x140afa1a4 mov      rsi, qword ptr [rdx]
0x140afa1a7 mov      rdi, qword ptr [rsi + 0x30]
0x140afa1ab mov      rbx, qword ptr [rsi + 0x40]
0x140afa1af cmp      rdi, qword ptr [rsi + 0x20]
0x140afa1b3 jne      0x140afa1c5
0x140afa1b5 lea      rcx, [rsi + 0x20]
0x140afa1b9 lea      rdx, [rip + 0x213ff88]
0x140afa1c0 call     0x140562c30 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hdc229ca0537875e6 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h7170e6483b671948
0x140afa1c5 mov      rax, qword ptr [rsi + 0x28]
0x140afa1c9 mov      rcx, rdi
0x140afa1cc shl      rcx, 5
0x140afa1d0 mov      dword ptr [rax + rcx], 4
0x140afa1d7 mov      qword ptr [rax + rcx + 0x18], rbx
0x140afa1dc inc      rdi
0x140afa1df mov      qword ptr [rsi + 0x30], rdi
0x140afa1e3 inc      qword ptr [rsi + 0x40]
0x140afa1e7 jmp      0x140afa2b0 ; 
0x140afa1ec mov      eax, dword ptr [r8 + 0x1c]
0x140afa1f0 mov      rcx, qword ptr [r8 + 0x10]
0x140afa1f4 mov      dword ptr [rcx], eax
0x140afa1f6 mov      rcx, qword ptr [r8]
0x140afa1f9 cmp      qword ptr [rcx + 0x10], 7
0x140afa1fe jb       0x140afa2b0
0x140afa204 mov      r8, qword ptr [rcx + 8]
0x140afa208 mov      r10, qword ptr [r8 + 0xd0]
0x140afa20f test     r10, r10
0x140afa212 je       0x140afa2b0
0x140afa218 cmp      byte ptr [rcx + 0x18], 6
0x140afa21c jne      0x140afa23b
0x140afa21e mov      rax, qword ptr [r8 + 0xd8]
0x140afa225 inc      rax
0x140afa228 mov      rdx, rax
0x140afa22b or       rdx, r10
0x140afa22e shr      rdx, 0x20
0x140afa232 je       0x140afa259
0x140afa234 xor      edx, edx
0x140afa236 div      r10
0x140afa239 jmp      0x140afa25e ; 
0x140afa23b mov      rdx, qword ptr [r8 + 0xd8]
0x140afa242 cmp      rdx, r10
0x140afa245 jb       0x140afa265
0x140afa247 lea      r8, [rip + 0x1feb3ea]
0x140afa24e mov      rcx, rdx
0x140afa251 mov      rdx, r10
0x140afa254 call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x140afa259 xor      edx, edx
0x140afa25b div      r10d
0x140afa25e mov      qword ptr [r8 + 0xd8], rdx
0x140afa265 mov      r8, qword ptr [r8 + 0xc8]
0x140afa26c shl      rdx, 4
0x140afa270 mov      eax, dword ptr [r8 + rdx]
0x140afa274 mov      byte ptr [rcx + 0x18], 6
0x140afa278 cmp      eax, 0x15
0x140afa27b je       0x140afa2b0
0x140afa27d add      r8, rdx
0x140afa280 mov      ecx, dword ptr [r8 + 0xc]
0x140afa284 mov      dword ptr [rsp + 0x28], ecx
0x140afa288 mov      rcx, qword ptr [r8 + 4]
0x140afa28c mov      qword ptr [rsp + 0x20], rcx
0x140afa291 mov      ecx, dword ptr [r9 + 0x1c]
0x140afa295 mov      rdx, qword ptr [r9 + 0x10]
0x140afa299 mov      dword ptr [rdx], ecx
0x140afa29b mov      rcx, qword ptr [r9]
0x140afa29e mov      dword ptr [rcx], eax
0x140afa2a0 mov      rax, qword ptr [rsp + 0x20]
0x140afa2a5 mov      qword ptr [rcx + 4], rax
0x140afa2a9 mov      eax, dword ptr [rsp + 0x28]
0x140afa2ad mov      dword ptr [rcx + 0xc], eax
0x140afa2b0 add      rsp, 0x30
0x140afa2b4 pop      rbx
0x140afa2b5 pop      rdi
0x140afa2b6 pop      rsi
0x140afa2b7 ret      