1415a0730: 55  push rbp
1415a0731: 56  push rsi
1415a0732: 4881ec88000000  sub rsp, 0x88
1415a0739: 488dac2480000000  lea rbp, [rsp + 0x80]
1415a0741: 48c74500feffffff  mov qword ptr [rbp], 0xfffffffffffffffe
1415a0749: 4889ce  mov rsi, rcx
1415a074c: 0fb602  movzx eax, byte ptr [rdx]
1415a074f: 83f804  cmp eax, 4
1415a0752: 488955f0  mov qword ptr [rbp - 0x10], rdx
1415a0756: 7460  je 0x1415a07b8
1415a0758: 83f805  cmp eax, 5
1415a075b: 0f85c2000000  jne 0x1415a0823
1415a0761: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a0765: 488945b0  mov qword ptr [rbp - 0x50], rax
1415a0769: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a076d: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1415a0771: c645ff00  mov byte ptr [rbp - 1], 0
1415a0775: c645fe01  mov byte ptr [rbp - 2], 1
1415a0779: 488d55a0  lea rdx, [rbp - 0x60]
1415a077d: 4889f1  mov rcx, rsi
1415a0780: e88b5b0100  call 0x1415b6310
1415a0785: b101  mov cl, 1
1415a0787: 31c0  xor eax, eax
1415a0789: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a078d: 410fb610  movzx edx, byte ptr [r8]
1415a0791: 83fa04  cmp edx, 4
1415a0794: 7457  je 0x1415a07ed
1415a0796: 83fa05  cmp edx, 5
1415a0799: 0f85bd000000  jne 0x1415a085c
1415a079f: 84c0  test al, al
1415a07a1: 0f84d2000000  je 0x1415a0879
1415a07a7: 4983c008  add r8, 8
1415a07ab: 4c89c1  mov rcx, r8
1415a07ae: e88d872fff  call 0x140898f40
1415a07b3: e9c1000000  jmp 0x1415a0879
1415a07b8: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a07bc: 488945d0  mov qword ptr [rbp - 0x30], rax
1415a07c0: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a07c4: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
1415a07c8: c645ff01  mov byte ptr [rbp - 1], 1
1415a07cc: c645fe00  mov byte ptr [rbp - 2], 0
1415a07d0: 488d55c0  lea rdx, [rbp - 0x40]
1415a07d4: 4889f1  mov rcx, rsi
1415a07d7: e8c4e10000  call 0x1415ae9a0
1415a07dc: b001  mov al, 1
1415a07de: 31c9  xor ecx, ecx
1415a07e0: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a07e4: 410fb610  movzx edx, byte ptr [r8]
1415a07e8: 83fa04  cmp edx, 4
1415a07eb: 75a9  jne 0x1415a0796
1415a07ed: 84c9  test cl, cl
1415a07ef: 0f8484000000  je 0x1415a0879
1415a07f5: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a07f9: 498b5018  mov rdx, qword ptr [r8 + 0x18]
1415a07fd: 48894de0  mov qword ptr [rbp - 0x20], rcx
1415a0801: e86a852eff  call 0x140888d70
1415a0806: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a080a: 488b5008  mov rdx, qword ptr [rax + 8]
1415a080e: 4885d2  test rdx, rdx
1415a0811: 7466  je 0x1415a0879
1415a0813: 48c1e205  shl rdx, 5
1415a0817: 41b808000000  mov r8d, 8
1415a081d: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a0821: eb51  jmp 0x1415a0874
1415a0823: c645ff01  mov byte ptr [rbp - 1], 1
1415a0827: c645fe01  mov byte ptr [rbp - 2], 1
1415a082b: 4c8d05668a6d01  lea r8, [rip + 0x16d8a66]
1415a0832: 4889d1  mov rcx, rdx
1415a0835: 488d55ef  lea rdx, [rbp - 0x11]
1415a0839: e822923501  call 0x1428f9a60
1415a083e: 488906  mov qword ptr [rsi], rax
1415a0841: c6462202  mov byte ptr [rsi + 0x22], 2
1415a0845: b001  mov al, 1
1415a0847: b101  mov cl, 1
1415a0849: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a084d: 410fb610  movzx edx, byte ptr [r8]
1415a0851: 83fa04  cmp edx, 4
1415a0854: 0f853cffffff  jne 0x1415a0796
1415a085a: eb91  jmp 0x1415a07ed
1415a085c: 80fa03  cmp dl, 3
1415a085f: 7218  jb 0x1415a0879
1415a0861: 498b5008  mov rdx, qword ptr [r8 + 8]
1415a0865: 4885d2  test rdx, rdx
1415a0868: 740f  je 0x1415a0879
1415a086a: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a086e: 41b801000000  mov r8d, 1
1415a0874: e8a73307ff  call 0x140613c20
1415a0879: 4889f0  mov rax, rsi
1415a087c: 4881c488000000  add rsp, 0x88
1415a0883: 5e  pop rsi
1415a0884: 5d  pop rbp
1415a0885: c3  ret 
1415a0886: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415a0890: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a0895: 55  push rbp
1415a0896: 56  push rsi
1415a0897: 4883ec28  sub rsp, 0x28
1415a089b: 488daa80000000  lea rbp, [rdx + 0x80]
1415a08a2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a08a6: 488b5008  mov rdx, qword ptr [rax + 8]
1415a08aa: 4885d2  test rdx, rdx
1415a08ad: 7413  je 0x1415a08c2
1415a08af: 48c1e205  shl rdx, 5
1415a08b3: 41b808000000  mov r8d, 8
1415a08b9: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a08bd: e85e3307ff  call 0x140613c20
1415a08c2: 90  nop 
1415a08c3: 4883c428  add rsp, 0x28
1415a08c7: 5e  pop rsi
1415a08c8: 5d  pop rbp
1415a08c9: c3  ret 
1415a08ca: 660f1f440000  nop word ptr [rax + rax]
1415a08d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a08d5: 55  push rbp
1415a08d6: 56  push rsi
1415a08d7: 4883ec28  sub rsp, 0x28
1415a08db: 488daa80000000  lea rbp, [rdx + 0x80]
1415a08e2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a08e6: 0fb600  movzx eax, byte ptr [rax]
1415a08e9: 83f804  cmp eax, 4
1415a08ec: 741a  je 0x1415a0908
1415a08ee: 83f805  cmp eax, 5
1415a08f1: 7522  jne 0x1415a0915
1415a08f3: f645ff01  test byte ptr [rbp - 1], 1
1415a08f7: 7415  je 0x1415a090e
1415a08f9: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a08fd: 4883c108  add rcx, 8
1415a0901: e83a862fff  call 0x140898f40
1415a0906: eb06  jmp 0x1415a090e
1415a0908: f645fe01  test byte ptr [rbp - 2], 1
1415a090c: 7512  jne 0x1415a0920
1415a090e: 4883c428  add rsp, 0x28
1415a0912: 5e  pop rsi
1415a0913: 5d  pop rbp
1415a0914: c3  ret 
1415a0915: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a0919: e872d12bff  call 0x14085da90
1415a091e: ebee  jmp 0x1415a090e
1415a0920: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a0924: 4883c108  add rcx, 8
1415a0928: e8638a2eff  call 0x140889390
1415a092d: ebdf  jmp 0x1415a090e
