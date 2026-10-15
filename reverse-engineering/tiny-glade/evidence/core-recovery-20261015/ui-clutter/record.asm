0x1409a9160  push     r15
0x1409a9162  push     r14
0x1409a9164  push     r13
0x1409a9166  push     r12
0x1409a9168  push     rsi
0x1409a9169  push     rdi
0x1409a916a  push     rbp
0x1409a916b  push     rbx
0x1409a916c  sub      rsp, 0x48
0x1409a9170  mov      r14, qword ptr [rdx]
0x1409a9173  mov      r8, qword ptr [rdx + 8]
0x1409a9177  mov      r10, qword ptr [r14]
0x1409a917a  xor      r9d, r9d
0x1409a917d  mov      rdx, r10
0x1409a9180  sub      rdx, qword ptr [r8 + 0x18]
0x1409a9184  cmovb    rdx, r9
0x1409a9188  sub      r10, qword ptr [r8 + 0x38]
0x1409a918c  cmovb    r10, r9
0x1409a9190  mov      rax, rdx
0x1409a9193  shl      rax, 4
0x1409a9197  add      rax, qword ptr [r8 + 8]
0x1409a919b  mov      r15, qword ptr [r8 + 0x10]
0x1409a919f  sub      r15, rdx
0x1409a91a2  cmovb    r15, r9
0x1409a91a6  mov      r11d, 8
0x1409a91ac  cmovb    rax, r11
0x1409a91b0  mov      rdx, r10
0x1409a91b3  shl      rdx, 4
0x1409a91b7  add      rdx, qword ptr [r8 + 0x28]
0x1409a91bb  mov      rsi, qword ptr [r8 + 0x30]
0x1409a91bf  sub      rsi, r10
0x1409a91c2  cmovb    rsi, r9
0x1409a91c6  cmovb    rdx, r11
0x1409a91ca  mov      r8, qword ptr [r8 + 0x40]
0x1409a91ce  sub      r8, rsi
0x1409a91d1  sub      r8, r15
0x1409a91d4  mov      qword ptr [r14], r8
0x1409a91d7  shl      r15, 4
0x1409a91db  add      r15, rax
0x1409a91de  shl      rsi, 4
0x1409a91e2  add      rsi, rdx
0x1409a91e5  mov      qword ptr [rsp + 0x28], rsi
0x1409a91ea  mov      ebp, dword ptr [rcx + 0x1c]
0x1409a91ed  mov      rsi, qword ptr [rcx]
0x1409a91f0  mov      r13, qword ptr [rcx + 0x10]
0x1409a91f4  test     rax, rax
0x1409a91f7  je       0x1409a9210
0x1409a91f9  nop      dword ptr [rax]
0x1409a9200  cmp      rax, r15
0x1409a9203  je       0x1409a9210
0x1409a9205  lea      r12, [rax + 0x10]
0x1409a9209  mov      rdi, rdx
0x1409a920c  jmp      0x1409a922a
0x1409a920e  nop      
0x1409a9210  test     rdx, rdx
0x1409a9213  je       0x1409a9299
0x1409a9219  cmp      rdx, qword ptr [rsp + 0x28]
0x1409a921e  je       0x1409a9299
0x1409a9220  lea      rdi, [rdx + 0x10]
0x1409a9224  xor      r12d, r12d
0x1409a9227  mov      rax, rdx
0x1409a922a  inc      qword ptr [r14]
0x1409a922d  mov      dword ptr [r13], ebp
0x1409a9231  mov      rbx, qword ptr [rax + 8]
0x1409a9235  movzx    eax, byte ptr [rip + 0x2c8b9cd]
0x1409a923c  mov      ecx, 0x18
0x1409a9241  mov      edx, 8
0x1409a9246  call     0x140613c10
0x1409a924b  test     rax, rax
0x1409a924e  je       0x1409a92aa
0x1409a9250  mov      qword ptr [rax], 1
0x1409a9257  mov      qword ptr [rax + 8], 1
0x1409a925f  mov      qword ptr [rax + 0x10], rbx
0x1409a9263  mov      qword ptr [rsp + 0x38], rax
0x1409a9268  mov      qword ptr [rsp + 0x30], 0xd
0x1409a9271  mov      rcx, rsi
0x1409a9274  lea      rdx, [rsp + 0x30]
0x1409a9279  lea      r8, [rip + 0x2114710]
0x1409a9280  call     0x140a84ec0
0x1409a9285  mov      rdx, rdi
0x1409a9288  mov      rax, r12
0x1409a928b  test     rax, rax
0x1409a928e  jne      0x1409a9200
0x1409a9294  jmp      0x1409a9210
0x1409a9299  add      rsp, 0x48
0x1409a929d  pop      rbx
0x1409a929e  pop      rbp
0x1409a929f  pop      rdi
0x1409a92a0  pop      rsi
0x1409a92a1  pop      r12
0x1409a92a3  pop      r13
0x1409a92a5  pop      r14
0x1409a92a7  pop      r15
0x1409a92a9  ret      
0x1409a92aa  mov      ecx, 8
0x1409a92af  mov      edx, 0x18
0x1409a92b4  call     0x1428d9000
0x1409a92b9  int3     
