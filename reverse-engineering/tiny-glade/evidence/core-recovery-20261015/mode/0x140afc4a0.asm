0x140afc4a0 push     rsi
0x140afc4a1 sub      rsp, 0x20
0x140afc4a5 mov      rax, rcx
0x140afc4a8 lea      rcx, [r8 - 4]
0x140afc4ac cmp      rcx, 7
0x140afc4b0 ja       0x140afc566
0x140afc4b6 lea      r9, [rip + 0x1fe9ccf]
0x140afc4bd movsxd   rcx, dword ptr [r9 + rcx*4]
0x140afc4c1 add      rcx, r9
0x140afc4c4 jmp      rcx
0x140afc4c6 cmp      dword ptr [rdx], 0x656e6f4e
0x140afc4cc je       0x140afc552
0x140afc4d2 cmp      dword ptr [rdx], 0x6c6c6157
0x140afc4d8 je       0x140afc558
0x140afc4da cmp      dword ptr [rdx], 0x666f6f52
0x140afc4e0 jne      0x140afc566
0x140afc4e6 mov      byte ptr [rax + 1], 2
0x140afc4ea jmp      0x140afc55c ; 
0x140afc4ec movabs   rcx, 0x696f507269617453
0x140afc4f6 xor      rcx, qword ptr [rdx]
0x140afc4f9 movzx    r9d, word ptr [rdx + 8]
0x140afc4fe xor      r9, 0x746e
0x140afc505 or       r9, rcx
0x140afc508 jne      0x140afc566
0x140afc50a mov      byte ptr [rax + 1], 4
0x140afc50e jmp      0x140afc55c ; 
0x140afc510 movabs   rcx, 0x74756c436f636544
0x140afc51a xor      rcx, qword ptr [rdx]
0x140afc51d movabs   r9, 0x72657474756c436f
0x140afc527 xor      r9, qword ptr [rdx + 3]
0x140afc52b or       r9, rcx
0x140afc52e jne      0x140afc566
0x140afc530 mov      byte ptr [rax + 1], 3
0x140afc534 jmp      0x140afc55c ; 
0x140afc536 mov      ecx, 0x72726554
0x140afc53b xor      ecx, dword ptr [rdx]
0x140afc53d mov      r9d, 0x6e696172
0x140afc543 xor      r9d, dword ptr [rdx + 3]
0x140afc547 or       r9d, ecx
0x140afc54a jne      0x140afc566
0x140afc54c mov      byte ptr [rax + 1], 5
0x140afc550 jmp      0x140afc55c ; 
0x140afc552 mov      byte ptr [rax + 1], 0
0x140afc556 jmp      0x140afc55c ; 
0x140afc558 mov      byte ptr [rax + 1], 1
0x140afc55c xor      ecx, ecx
0x140afc55e mov      byte ptr [rax], cl
0x140afc560 add      rsp, 0x20
0x140afc564 pop      rsi
0x140afc565 ret      
0x140afc566 lea      r10, [rip + 0x1fe960b]
0x140afc56d mov      r9d, 6
0x140afc573 mov      rcx, rdx
0x140afc576 mov      rdx, r8
0x140afc579 mov      r8, r10
0x140afc57c mov      rsi, rax
0x140afc57f call     0x1428f9e20 ; serde::de::Error::unknown_variant::h81e52211f9e120e7
0x140afc584 mov      rcx, rax
0x140afc587 mov      rax, rsi
0x140afc58a mov      qword ptr [rsi + 8], rcx
0x140afc58e mov      cl, 1
0x140afc590 jmp      0x140afc55e ; 