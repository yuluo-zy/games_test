0x140a48ba0 push     r14
0x140a48ba2 push     rsi
0x140a48ba3 push     rdi
0x140a48ba4 push     rbx
0x140a48ba5 sub      rsp, 0x58
0x140a48ba9 mov      ecx, dword ptr [rcx]
0x140a48bab mov      r14d, 0xa
0x140a48bb1 lea      rax, [rip + 0x2085993] ; @0x142ace54b '0001020304050607080910111213141516171819202122232425262728293031323334353637383940414243444546474849'
0x140a48bb8 cmp      ecx, 0x2710
0x140a48bbe jb       0x140a48c6f
0x140a48bc4 mov      r10d, 0xa
0x140a48bca mov      r8d, 0xd1b71759
0x140a48bd0 mov      r9d, ecx
0x140a48bd3 imul     r9, r8
0x140a48bd7 shr      r9, 0x2d
0x140a48bdb imul     r11d, r9d, 0x2710
0x140a48be2 mov      esi, ecx
0x140a48be4 sub      esi, r11d
0x140a48be7 imul     r11d, esi, 0x147b
0x140a48bee shr      r11d, 0x13
0x140a48bf2 imul     edi, r11d, 0x64
0x140a48bf6 sub      esi, edi
0x140a48bf8 movzx    esi, si
0x140a48bfb lea      r14, [r10 - 4]
0x140a48bff movzx    r11d, word ptr [rax + r11*2]
0x140a48c04 mov      word ptr [rsp + r10 + 0x2c], r11w
0x140a48c0a movzx    r11d, word ptr [rax + rsi*2]
0x140a48c0f mov      word ptr [rsp + r10 + 0x2e], r11w
0x140a48c15 mov      r10, r14
0x140a48c18 cmp      ecx, 0x5f5e0ff
0x140a48c1e mov      ecx, r9d
0x140a48c21 ja       0x140a48bd0
0x140a48c23 mov      ecx, r9d
0x140a48c26 cmp      r9d, 0x63
0x140a48c2a jbe      0x140a48c5a
0x140a48c2c movzx    r8d, cx
0x140a48c30 shr      r8d, 2
0x140a48c34 imul     r8d, r8d, 0x147b
0x140a48c3b shr      r8d, 0x11
0x140a48c3f imul     r9d, r8d, 0x64
0x140a48c43 sub      ecx, r9d
0x140a48c46 movzx    ecx, cx
0x140a48c49 movzx    ecx, word ptr [rax + rcx*2]
0x140a48c4d mov      word ptr [rsp + r14 + 0x2e], cx
0x140a48c53 add      r14, -2
0x140a48c57 mov      rcx, r8
0x140a48c5a mov      rsi, qword ptr [rdx]
0x140a48c5d cmp      ecx, 0xa
0x140a48c60 jae      0x140a48c7d
0x140a48c62 or       cl, 0x30
0x140a48c65 mov      byte ptr [rsp + r14 + 0x2f], cl
0x140a48c6a dec      r14
0x140a48c6d jmp      0x140a48c8b ; 
0x140a48c6f mov      r9d, ecx
0x140a48c72 mov      ecx, r9d
0x140a48c75 cmp      r9d, 0x63
0x140a48c79 ja       0x140a48c2c
0x140a48c7b jmp      0x140a48c5a ; 
0x140a48c7d movzx    eax, word ptr [rax + rcx*2]
0x140a48c81 mov      word ptr [rsp + r14 + 0x2e], ax
0x140a48c87 add      r14, -2
0x140a48c8b mov      edi, 0xa
0x140a48c90 sub      rdi, r14
0x140a48c93 mov      rax, qword ptr [rsi]
0x140a48c96 mov      rbx, qword ptr [rsi + 0x10]
0x140a48c9a sub      rax, rbx
0x140a48c9d cmp      rax, rdi
0x140a48ca0 jb       0x140a48ccc
0x140a48ca2 lea      rdx, [rsp + r14]
0x140a48ca6 add      rdx, 0x30
0x140a48caa mov      rcx, qword ptr [rsi + 8]
0x140a48cae add      rcx, rbx
0x140a48cb1 mov      r8, rdi
0x140a48cb4 call     0x142923c00 ; memcpy
0x140a48cb9 add      rbx, rdi
0x140a48cbc mov      qword ptr [rsi + 0x10], rbx
0x140a48cc0 xor      eax, eax
0x140a48cc2 add      rsp, 0x58
0x140a48cc6 pop      rbx
0x140a48cc7 pop      rdi
0x140a48cc8 pop      rsi
0x140a48cc9 pop      r14
0x140a48ccb ret      
0x140a48ccc mov      qword ptr [rsp + 0x20], 1
0x140a48cd5 mov      r9d, 1
0x140a48cdb mov      rcx, rsi
0x140a48cde mov      rdx, rbx
0x140a48ce1 mov      r8, rdi
0x140a48ce4 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a48ce9 mov      rbx, qword ptr [rsi + 0x10]
0x140a48ced jmp      0x140a48ca2 ; 