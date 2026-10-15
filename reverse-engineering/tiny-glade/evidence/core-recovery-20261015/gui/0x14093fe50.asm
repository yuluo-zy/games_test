0x14093fe50 push     rbp
0x14093fe51 push     r15
0x14093fe53 push     r14
0x14093fe55 push     r12
0x14093fe57 push     rsi
0x14093fe58 push     rdi
0x14093fe59 push     rbx
0x14093fe5a sub      rsp, 0x40
0x14093fe5e lea      rbp, [rsp + 0x40]
0x14093fe63 mov      qword ptr [rbp - 8], 0xfffffffffffffffe
0x14093fe6b mov      r15, r8
0x14093fe6e test     r8, r8
0x14093fe71 js       0x14093ff4c
0x14093fe77 mov      edi, r9d
0x14093fe7a mov      rsi, rcx
0x14093fe7d je       0x14093feac
0x14093fe7f mov      r14, rdx
0x14093fe82 movzx    eax, byte ptr [rip + 0x2cf4d80]
0x14093fe89 mov      ebx, 1
0x14093fe8e mov      edx, 1
0x14093fe93 mov      rcx, r15
0x14093fe96 call     0x140613c10
0x14093fe9b test     rax, rax
0x14093fe9e je       0x14093ff4e
0x14093fea4 mov      r12, rax
0x14093fea7 mov      rdx, r14
0x14093feaa jmp      0x14093feb2
0x14093feac mov      r12d, 1
0x14093feb2 mov      rcx, r12
0x14093feb5 mov      r8, r15
0x14093feb8 call     0x142923c00
0x14093febd mov      rcx, qword ptr [rsi + 0xb0]
0x14093fec4 cmp      rcx, 4
0x14093fec8 ja       0x14093ff60
0x14093fece shl      rcx, 5
0x14093fed2 lea      rbx, [rsi + rcx]
0x14093fed6 add      rbx, 0x10
0x14093feda mov      rdx, qword ptr [rsi + rcx + 0x10]
0x14093fedf mov      rax, rdx
0x14093fee2 neg      rax
0x14093fee5 jo       0x14093fef8
0x14093fee7 jae      0x14093fef8
0x14093fee9 mov      rcx, qword ptr [rbx + 8]
0x14093feed mov      r8d, 1
0x14093fef3 call     0x140613c20
0x14093fef8 mov      qword ptr [rbx], r15
0x14093fefb mov      qword ptr [rbx + 8], r12
0x14093feff mov      qword ptr [rbx + 0x10], r15
0x14093ff03 mov      dword ptr [rbx + 0x18], edi
0x14093ff06 mov      dword ptr [rbx + 0x1c], 0x40a00000
0x14093ff0d mov      rcx, qword ptr [rsi + 0xb0]
0x14093ff14 lea      rax, [rcx + 1]
0x14093ff18 movabs   rdx, 0xcccccccccccccccd
0x14093ff22 mul      rdx
0x14093ff25 shr      rdx, 2
0x14093ff29 lea      rax, [rdx + rdx*4]
0x14093ff2d neg      rax
0x14093ff30 add      rax, rcx
0x14093ff33 inc      rax
0x14093ff36 mov      qword ptr [rsi + 0xb0], rax
0x14093ff3d add      rsp, 0x40
0x14093ff41 pop      rbx
0x14093ff42 pop      rdi
0x14093ff43 pop      rsi
0x14093ff44 pop      r12
0x14093ff46 pop      r14
0x14093ff48 pop      r15
0x14093ff4a pop      rbp
0x14093ff4b ret      
0x14093ff4c xor      ebx, ebx
0x14093ff4e lea      r8, [rip + 0x217256b]
0x14093ff55 mov      rcx, rbx
0x14093ff58 mov      rdx, r15
0x14093ff5b call     0x1428d8fe3
0x14093ff60 mov      qword ptr [rbp - 0x18], r12
0x14093ff64 mov      qword ptr [rbp - 0x10], r15
0x14093ff68 lea      r8, [rip + 0x21725c9]
0x14093ff6f mov      edx, 5
0x14093ff74 call     0x1428d9518