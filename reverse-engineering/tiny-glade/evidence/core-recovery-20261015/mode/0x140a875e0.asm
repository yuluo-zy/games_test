0x140a875e0 push     r15
0x140a875e2 push     r14
0x140a875e4 push     r12
0x140a875e6 push     rsi
0x140a875e7 push     rdi
0x140a875e8 push     rbx
0x140a875e9 sub      rsp, 0x48
0x140a875ed mov      r14, r8
0x140a875f0 mov      rdi, rdx
0x140a875f3 mov      rbx, qword ptr [rcx]
0x140a875f6 mov      rsi, qword ptr [rdx]
0x140a875f9 mov      rcx, rbx
0x140a875fc mov      rdx, rsi
0x140a875ff call     0x140a825e0 ; core::cmp::PartialEq::ne::hbdc199c1652682d8 (.llvm.11027640352236353829)
0x140a87604 test     al, al
0x140a87606 je       0x140a87687
0x140a87608 movups   xmm0, xmmword ptr [rbx]
0x140a8760b movaps   xmmword ptr [rsp + 0x20], xmm0
0x140a87610 movups   xmm0, xmmword ptr [rsi]
0x140a87613 movaps   xmmword ptr [rsp + 0x30], xmm0
0x140a87618 mov      eax, dword ptr [r14 + 0x1c]
0x140a8761c mov      rcx, qword ptr [r14 + 0x10]
0x140a87620 mov      dword ptr [rcx], eax
0x140a87622 mov      r14, qword ptr [r14]
0x140a87625 mov      r15, qword ptr [r14 + 0x30]
0x140a87629 mov      r12, qword ptr [r14 + 0x40]
0x140a8762d cmp      r15, qword ptr [r14 + 0x20]
0x140a87631 jne      0x140a87643
0x140a87633 lea      rcx, [r14 + 0x20]
0x140a87637 lea      rdx, [rip + 0x21b2b0a]
0x140a8763e call     0x140562ab0 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::hd115b64d00308ce7 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::heb430bc807eb3344
0x140a87643 mov      rax, qword ptr [r14 + 0x28]
0x140a87647 lea      rcx, [r15 + r15*4]
0x140a8764b movaps   xmm0, xmmword ptr [rsp + 0x20]
0x140a87650 movaps   xmm1, xmmword ptr [rsp + 0x30]
0x140a87655 movups   xmmword ptr [rax + rcx*8 + 0x10], xmm1
0x140a8765a movups   xmmword ptr [rax + rcx*8], xmm0
0x140a8765e mov      qword ptr [rax + rcx*8 + 0x20], r12
0x140a87663 inc      r15
0x140a87666 mov      qword ptr [r14 + 0x30], r15
0x140a8766a inc      qword ptr [r14 + 0x40]
0x140a8766e movups   xmm0, xmmword ptr [rbx]
0x140a87671 movaps   xmmword ptr [rsp + 0x20], xmm0
0x140a87676 mov      eax, dword ptr [rdi + 0x1c]
0x140a87679 mov      rcx, qword ptr [rdi + 0x10]
0x140a8767d mov      dword ptr [rcx], eax
0x140a8767f movaps   xmm0, xmmword ptr [rsp + 0x20]
0x140a87684 movups   xmmword ptr [rsi], xmm0
0x140a87687 add      rsp, 0x48
0x140a8768b pop      rbx
0x140a8768c pop      rdi
0x140a8768d pop      rsi
0x140a8768e pop      r12
0x140a87690 pop      r14
0x140a87692 pop      r15
0x140a87694 ret      