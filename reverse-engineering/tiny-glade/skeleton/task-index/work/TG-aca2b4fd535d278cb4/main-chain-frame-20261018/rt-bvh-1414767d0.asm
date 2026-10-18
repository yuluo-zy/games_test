00000001414767d0 push      rbp
00000001414767d1 push      r15
00000001414767d3 push      r14
00000001414767d5 push      r13
00000001414767d7 push      r12
00000001414767d9 push      rsi
00000001414767da push      rdi
00000001414767db push      rbx
00000001414767dc sub       rsp, 0x98
00000001414767e3 lea       rbp, [rsp + 0x80]
00000001414767eb movaps    xmmword ptr [rbp], xmm9
00000001414767f0 movaps    xmmword ptr [rbp - 0x10], xmm8
00000001414767f5 movaps    xmmword ptr [rbp - 0x20], xmm7
00000001414767f9 movaps    xmmword ptr [rbp - 0x30], xmm6
00000001414767fd mov       qword ptr [rbp - 0x38], 0xfffffffffffffffe
0000000141476805 mov       r8, qword ptr [rcx]
0000000141476808 mov       r10, qword ptr [rcx + 8]
000000014147680c mov       rax, qword ptr [rdx]
000000014147680f mov       qword ptr [rbp - 0x40], rax
0000000141476813 mov       r9, qword ptr [rdx + 8]
0000000141476817 cmp       r8, r10
000000014147681a je        0x141476923
0000000141476820 mov       r12, qword ptr [rdx + 0x10]
0000000141476824 mov       rax, qword ptr [rcx + 0x10]
0000000141476828 sub       r10, r8
000000014147682b shr       r10, 2
000000014147682f xor       r13d, r13d
0000000141476832 mov       qword ptr [rbp - 0x60], r8
0000000141476836 mov       qword ptr [rbp - 0x58], r10
000000014147683a mov       qword ptr [rbp - 0x50], rax
000000014147683e nop       
0000000141476840 mov       qword ptr [rbp - 0x48], r9
0000000141476844 mov       ecx, dword ptr [r8 + r13*4]
0000000141476848 mov       rdx, qword ptr [rax + 0x10]
000000014147684c cmp       rdx, rcx
000000014147684f jbe       0x141476950
0000000141476855 mov       rax, qword ptr [rax + 8]
0000000141476859 lea       rcx, [rcx + rcx*2]
000000014147685d shl       rcx, 4
0000000141476861 movaps    xmm8, xmmword ptr [rax + rcx]
0000000141476866 movaps    xmm7, xmmword ptr [rax + rcx + 0x10]
000000014147686b movaps    xmm6, xmmword ptr [rax + rcx + 0x20]
0000000141476870 movss     xmm9, dword ptr [rax + rcx + 8]
0000000141476877 subps     xmm7, xmm8
000000014147687b movaps    xmm0, xmm7
000000014147687e call      0x1408b9f20
0000000141476883 mov       esi, eax
0000000141476885 movaps    xmm0, xmm6
0000000141476888 subss     xmm0, xmm8
000000014147688d call      0x1408b9f20
0000000141476892 mov       di, ax
0000000141476895 movshdup  xmm0, xmm7
0000000141476899 call      0x1408b9f20
000000014147689e mov       ebx, eax
00000001414768a0 subps     xmm6, xmm8
00000001414768a4 movshdup  xmm0, xmm6
00000001414768a8 call      0x1408b9f20
00000001414768ad mov       r14w, ax
00000001414768b1 movhlps   xmm7, xmm7
00000001414768b4 movaps    xmm0, xmm7
00000001414768b7 call      0x1408b9f20
00000001414768bc mov       r15d, eax
00000001414768bf movhlps   xmm6, xmm6
00000001414768c2 movaps    xmm0, xmm6
00000001414768c5 call      0x1408b9f20
00000001414768ca shl       esi, 0x10
00000001414768cd movzx     ecx, di
00000001414768d0 or        ecx, esi
00000001414768d2 shl       ebx, 0x10
00000001414768d5 movzx     edx, r14w
00000001414768d9 or        edx, ebx
00000001414768db shl       r15d, 0x10
00000001414768df movzx     eax, ax
00000001414768e2 or        eax, r15d
00000001414768e5 mov       r9, qword ptr [rbp - 0x48]
00000001414768e9 lea       r8, [r9 + r9*2]
00000001414768ed movlps    qword ptr [r12 + r8*8], xmm8
00000001414768f2 movss     dword ptr [r12 + r8*8 + 8], xmm9
00000001414768f9 mov       dword ptr [r12 + r8*8 + 0xc], ecx
00000001414768fe mov       dword ptr [r12 + r8*8 + 0x10], edx
0000000141476903 mov       dword ptr [r12 + r8*8 + 0x14], eax
0000000141476908 inc       r9
000000014147690b inc       r13
000000014147690e mov       r10, qword ptr [rbp - 0x58]
0000000141476912 cmp       r13, r10
0000000141476915 mov       r8, qword ptr [rbp - 0x60]
0000000141476919 mov       rax, qword ptr [rbp - 0x50]
000000014147691d jne       0x141476840
0000000141476923 mov       rax, qword ptr [rbp - 0x40]
0000000141476927 mov       qword ptr [rax], r9
000000014147692a movaps    xmm6, xmmword ptr [rbp - 0x30]
000000014147692e movaps    xmm7, xmmword ptr [rbp - 0x20]
0000000141476932 movaps    xmm8, xmmword ptr [rbp - 0x10]
0000000141476937 movaps    xmm9, xmmword ptr [rbp]
000000014147693c add       rsp, 0x98
0000000141476943 pop       rbx
0000000141476944 pop       rdi
0000000141476945 pop       rsi
0000000141476946 pop       r12
0000000141476948 pop       r13
000000014147694a pop       r14
000000014147694c pop       r15
000000014147694e pop       rbp
000000014147694f ret       
0000000141476950 lea       r8, [rip + 0x17eecc9]
0000000141476957 call      0x1428d9518
000000014147695c ud2       
000000014147695e nop       
0000000141476960 mov       qword ptr [rsp + 0x10], rdx
0000000141476965 push      rbp
0000000141476966 push      r15
0000000141476968 push      r14
000000014147696a push      r13
000000014147696c push      r12
000000014147696e push      rsi
000000014147696f push      rdi
0000000141476970 push      rbx
0000000141476971 sub       rsp, 0x68
0000000141476975 lea       rbp, [rdx + 0x80]
000000014147697c movaps    xmmword ptr [rsp + 0x20], xmm9
0000000141476982 movaps    xmmword ptr [rsp + 0x30], xmm8
0000000141476988 movaps    xmmword ptr [rsp + 0x40], xmm7
000000014147698d movaps    xmmword ptr [rsp + 0x50], xmm6
0000000141476992 mov       rax, qword ptr [rbp - 0x40]
0000000141476996 mov       rcx, qword ptr [rbp - 0x48]
000000014147699a mov       qword ptr [rax], rcx
000000014147699d movaps    xmm6, xmmword ptr [rsp + 0x50]
00000001414769a2 movaps    xmm7, xmmword ptr [rsp + 0x40]
00000001414769a7 movaps    xmm8, xmmword ptr [rsp + 0x30]
00000001414769ad movaps    xmm9, xmmword ptr [rsp + 0x20]
00000001414769b3 add       rsp, 0x68
00000001414769b7 pop       rbx
00000001414769b8 pop       rdi
00000001414769b9 pop       rsi
00000001414769ba pop       r12
00000001414769bc pop       r13
00000001414769be pop       r14
00000001414769c0 pop       r15
00000001414769c2 pop       rbp
00000001414769c3 ret       
