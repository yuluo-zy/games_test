0x140a876a0 push     rbp
0x140a876a1 push     r15
0x140a876a3 push     r14
0x140a876a5 push     r13
0x140a876a7 push     r12
0x140a876a9 push     rsi
0x140a876aa push     rdi
0x140a876ab push     rbx
0x140a876ac sub      rsp, 0x188
0x140a876b3 lea      rbp, [rsp + 0x80]
0x140a876bb movaps   xmmword ptr [rbp + 0xf0], xmm6
0x140a876c2 mov      qword ptr [rbp + 0xe8], 0xfffffffffffffffe
0x140a876cd mov      qword ptr [rbp + 0xc0], rdx
0x140a876d4 mov      rdi, qword ptr [r8 + 0x10]
0x140a876d8 lea      r15, [rdi*8]
0x140a876e0 xor      r13d, r13d
0x140a876e3 mov      rax, rdi
0x140a876e6 shr      rax, 0x3d
0x140a876ea jne      0x140a878e6
0x140a876f0 movabs   rax, 0x7ffffffffffffffc
0x140a876fa cmp      r15, rax
0x140a876fd ja       0x140a878e6
0x140a87703 mov      rbx, r8
0x140a87706 movss    xmm6, dword ptr [r8 + 0x54]
0x140a8770c mov      r12, qword ptr [r8 + 8]
0x140a87710 test     r15, r15
0x140a87713 mov      qword ptr [rbp + 0xc8], rcx
0x140a8771a mov      dword ptr [rbp + 0xdc], r9d
0x140a87721 je       0x140a8774f
0x140a87723 movzx    eax, byte ptr [rip + 0x2bad4df]
0x140a8772a mov      r13d, 4
0x140a87730 mov      edx, 4
0x140a87735 mov      rcx, r15
0x140a87738 call     0x140613c10 ; __rust_alloc
0x140a8773d test     rax, rax
0x140a87740 je       0x140a878e6
0x140a87746 mov      qword ptr [rbp + 0xe0], rdi
0x140a8774d jmp      0x140a8775f ; 
0x140a8774f mov      eax, 4
0x140a87754 mov      qword ptr [rbp + 0xe0], 0
0x140a8775f mov      qword ptr [rbp + 0xd0], rax
0x140a87766 mov      rcx, rax
0x140a87769 mov      rdx, r12
0x140a8776c mov      r8, r15
0x140a8776f call     0x142923c00 ; memcpy
0x140a87774 mov      r14, qword ptr [rbx + 0x28]
0x140a87778 lea      r12, [r14*4]
0x140a87780 xor      esi, esi
0x140a87782 mov      rax, r14
0x140a87785 shr      rax, 0x3e
0x140a87789 jne      0x140a878f8
0x140a8778f movabs   rax, 0x7ffffffffffffffc
0x140a87799 cmp      r12, rax
0x140a8779c ja       0x140a878f8
0x140a877a2 mov      r13, qword ptr [rbx + 0x20]
0x140a877a6 test     r12, r12
0x140a877a9 je       0x140a877d5
0x140a877ab movzx    eax, byte ptr [rip + 0x2bad457]
0x140a877b2 mov      esi, 4
0x140a877b7 mov      edx, 4
0x140a877bc mov      rcx, r12
0x140a877bf call     0x140613c10 ; __rust_alloc
0x140a877c4 test     rax, rax
0x140a877c7 je       0x140a878f8
0x140a877cd mov      r15, rax
0x140a877d0 mov      rsi, r14
0x140a877d3 jmp      0x140a877dd ; 
0x140a877d5 mov      r15d, 4
0x140a877db xor      esi, esi
0x140a877dd mov      rcx, r15
0x140a877e0 mov      rdx, r13
0x140a877e3 mov      r8, r12
0x140a877e6 call     0x142923c00 ; memcpy
0x140a877eb movss    xmm0, dword ptr [rbx + 0x30]
0x140a877f0 mov      ecx, dword ptr [rbx + 0x38]
0x140a877f3 lea      edx, [rcx - 2]
0x140a877f6 cmp      edx, 2
0x140a877f9 mov      eax, 2
0x140a877fe cmovae   edx, eax
0x140a87801 test     edx, edx
0x140a87803 je       0x140a87829
0x140a87805 cmp      edx, 1
0x140a87808 jne      0x140a87811
0x140a8780a mov      eax, 3
0x140a8780f jmp      0x140a87829 ; 
0x140a87811 mov      rax, qword ptr [rbx + 0x4c]
0x140a87815 mov      qword ptr [rbp + 0xb0], rax
0x140a8781c movups   xmm1, xmmword ptr [rbx + 0x3c]
0x140a87820 movaps   xmmword ptr [rbp + 0xa0], xmm1
0x140a87827 mov      eax, ecx
0x140a87829 movzx    ecx, word ptr [rbx + 0x68]
0x140a8782d movss    xmm1, dword ptr [rbx + 0x60]
0x140a87832 mov      rdx, qword ptr [rbp + 0xe0]
0x140a87839 mov      qword ptr [rbp - 0x50], rdx
0x140a8783d mov      rdx, qword ptr [rbp + 0xd0]
0x140a87844 mov      qword ptr [rbp - 0x48], rdx
0x140a87848 mov      qword ptr [rbp - 0x40], rdi
0x140a8784c mov      qword ptr [rbp - 0x38], rsi
0x140a87850 mov      qword ptr [rbp - 0x30], r15
0x140a87854 mov      qword ptr [rbp - 0x28], r14
0x140a87858 mov      rdx, qword ptr [rbp + 0xc0]
0x140a8785f mov      qword ptr [rbp + 0x20], rdx
0x140a87863 movss    dword ptr [rbp + 0x28], xmm6
0x140a87868 movss    dword ptr [rbp - 0x20], xmm0
0x140a8786d mov      dword ptr [rbp - 0x18], eax
0x140a87870 movaps   xmm0, xmmword ptr [rbp + 0xa0]
0x140a87877 movups   xmmword ptr [rbp - 0x14], xmm0
0x140a8787b mov      rax, qword ptr [rbp + 0xb0]
0x140a87882 mov      qword ptr [rbp - 4], rax
0x140a87886 movss    dword ptr [rbp + 4], xmm6
0x140a8788b movsd    xmm0, qword ptr [rbx + 0x58]
0x140a87890 movsd    qword ptr [rbp + 8], xmm0
0x140a87895 movss    dword ptr [rbp + 0x10], xmm1
0x140a8789a mov      eax, dword ptr [rbx + 0x64]
0x140a8789d mov      dword ptr [rbp + 0x14], eax
0x140a878a0 mov      word ptr [rbp + 0x18], cx
0x140a878a4 mov      eax, dword ptr [rbp + 0xdc]
0x140a878aa mov      dword ptr [rbp + 0x2c], eax
0x140a878ad movabs   rax, 0x8000000000000001
0x140a878b7 mov      qword ptr [rbp - 0x58], rax
0x140a878bb lea      rdx, [rbp - 0x58]
0x140a878bf mov      rcx, qword ptr [rbp + 0xc8]
0x140a878c6 call     0x140a82b40 ; country_core::resources::history::wall_compound_edit::CompoundEditBuilder::next::haac2dec7a73c97f5
0x140a878cb movaps   xmm6, xmmword ptr [rbp + 0xf0]
0x140a878d2 add      rsp, 0x188
0x140a878d9 pop      rbx
0x140a878da pop      rdi
0x140a878db pop      rsi
0x140a878dc pop      r12
0x140a878de pop      r13
0x140a878e0 pop      r14
0x140a878e2 pop      r15
0x140a878e4 pop      rbp
0x140a878e5 ret      
0x140a878e6 lea      r8, [rip + 0x21b128b]
0x140a878ed mov      rcx, r13
0x140a878f0 mov      rdx, r15
0x140a878f3 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a878f8 lea      r8, [rip + 0x21b1279]
0x140a878ff mov      rcx, rsi
0x140a87902 mov      rdx, r12
0x140a87905 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a8790a ud2      
0x140a8790c nop      dword ptr [rax]
0x140a87910 mov      qword ptr [rsp + 0x10], rdx
0x140a87915 push     rbp
0x140a87916 push     r15
0x140a87918 push     r14
0x140a8791a push     r13
0x140a8791c push     r12
0x140a8791e push     rsi
0x140a8791f push     rdi
0x140a87920 push     rbx
0x140a87921 sub      rsp, 0x38
0x140a87925 lea      rbp, [rdx + 0x80]
0x140a8792c movaps   xmmword ptr [rsp + 0x20], xmm6
0x140a87931 cmp      qword ptr [rbp + 0xe0], 0
0x140a87939 je       0x140a87958
0x140a8793b mov      rdx, qword ptr [rbp + 0xe0]
0x140a87942 shl      rdx, 3
0x140a87946 mov      r8d, 4
0x140a8794c mov      rcx, qword ptr [rbp + 0xd0]
0x140a87953 call     0x140613c20 ; __rust_dealloc
0x140a87958 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x140a8795d add      rsp, 0x38
0x140a87961 pop      rbx
0x140a87962 pop      rdi
0x140a87963 pop      rsi
0x140a87964 pop      r12
0x140a87966 pop      r13
0x140a87968 pop      r14
0x140a8796a pop      r15
0x140a8796c pop      rbp
0x140a8796d ret      