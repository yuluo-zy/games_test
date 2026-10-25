1415a3740: 55  push rbp
1415a3741: 56  push rsi
1415a3742: 4881ec88000000  sub rsp, 0x88
1415a3749: 488dac2480000000  lea rbp, [rsp + 0x80]
1415a3751: 48c74500feffffff  mov qword ptr [rbp], 0xfffffffffffffffe
1415a3759: 4889ce  mov rsi, rcx
1415a375c: 0fb602  movzx eax, byte ptr [rdx]
1415a375f: 83f804  cmp eax, 4
1415a3762: 488955f0  mov qword ptr [rbp - 0x10], rdx
1415a3766: 7460  je 0x1415a37c8
1415a3768: 83f805  cmp eax, 5
1415a376b: 0f85c2000000  jne 0x1415a3833
1415a3771: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a3775: 488945b0  mov qword ptr [rbp - 0x50], rax
1415a3779: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a377d: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1415a3781: c645ff00  mov byte ptr [rbp - 1], 0
1415a3785: c645fe01  mov byte ptr [rbp - 2], 1
1415a3789: 488d55a0  lea rdx, [rbp - 0x60]
1415a378d: 4889f1  mov rcx, rsi
1415a3790: e8bb8e0100  call 0x1415bc650
1415a3795: b101  mov cl, 1
1415a3797: 31c0  xor eax, eax
1415a3799: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a379d: 410fb610  movzx edx, byte ptr [r8]
1415a37a1: 83fa04  cmp edx, 4
1415a37a4: 7457  je 0x1415a37fd
1415a37a6: 83fa05  cmp edx, 5
1415a37a9: 0f85c0000000  jne 0x1415a386f
1415a37af: 84c0  test al, al
1415a37b1: 0f84d5000000  je 0x1415a388c
1415a37b7: 4983c008  add r8, 8
1415a37bb: 4c89c1  mov rcx, r8
1415a37be: e87d572fff  call 0x140898f40
1415a37c3: e9c4000000  jmp 0x1415a388c
1415a37c8: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a37cc: 488945d0  mov qword ptr [rbp - 0x30], rax
1415a37d0: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a37d4: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
1415a37d8: c645ff01  mov byte ptr [rbp - 1], 1
1415a37dc: c645fe00  mov byte ptr [rbp - 2], 0
1415a37e0: 488d55c0  lea rdx, [rbp - 0x40]
1415a37e4: 4889f1  mov rcx, rsi
1415a37e7: e8a4000100  call 0x1415b3890
1415a37ec: b001  mov al, 1
1415a37ee: 31c9  xor ecx, ecx
1415a37f0: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a37f4: 410fb610  movzx edx, byte ptr [r8]
1415a37f8: 83fa04  cmp edx, 4
1415a37fb: 75a9  jne 0x1415a37a6
1415a37fd: 84c9  test cl, cl
1415a37ff: 0f8487000000  je 0x1415a388c
1415a3805: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a3809: 498b5018  mov rdx, qword ptr [r8 + 0x18]
1415a380d: 48894de0  mov qword ptr [rbp - 0x20], rcx
1415a3811: e85a552eff  call 0x140888d70
1415a3816: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a381a: 488b5008  mov rdx, qword ptr [rax + 8]
1415a381e: 4885d2  test rdx, rdx
1415a3821: 7469  je 0x1415a388c
1415a3823: 48c1e205  shl rdx, 5
1415a3827: 41b808000000  mov r8d, 8
1415a382d: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a3831: eb54  jmp 0x1415a3887
1415a3833: c645ff01  mov byte ptr [rbp - 1], 1
1415a3837: c645fe01  mov byte ptr [rbp - 2], 1
1415a383b: 4c8d05565d6d01  lea r8, [rip + 0x16d5d56]
1415a3842: 4889d1  mov rcx, rdx
1415a3845: 488d55ef  lea rdx, [rbp - 0x11]
1415a3849: e812623501  call 0x1428f9a60
1415a384e: 48894608  mov qword ptr [rsi + 8], rax
1415a3852: c70601000000  mov dword ptr [rsi], 1
1415a3858: b001  mov al, 1
1415a385a: b101  mov cl, 1
1415a385c: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a3860: 410fb610  movzx edx, byte ptr [r8]
1415a3864: 83fa04  cmp edx, 4
1415a3867: 0f8539ffffff  jne 0x1415a37a6
1415a386d: eb8e  jmp 0x1415a37fd
1415a386f: 80fa03  cmp dl, 3
1415a3872: 7218  jb 0x1415a388c
1415a3874: 498b5008  mov rdx, qword ptr [r8 + 8]
1415a3878: 4885d2  test rdx, rdx
1415a387b: 740f  je 0x1415a388c
1415a387d: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a3881: 41b801000000  mov r8d, 1
1415a3887: e8940307ff  call 0x140613c20
1415a388c: 4889f0  mov rax, rsi
1415a388f: 4881c488000000  add rsp, 0x88
1415a3896: 5e  pop rsi
1415a3897: 5d  pop rbp
1415a3898: c3  ret 
1415a3899: 0f1f8000000000  nop dword ptr [rax]
1415a38a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a38a5: 55  push rbp
1415a38a6: 56  push rsi
1415a38a7: 4883ec28  sub rsp, 0x28
1415a38ab: 488daa80000000  lea rbp, [rdx + 0x80]
1415a38b2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a38b6: 488b5008  mov rdx, qword ptr [rax + 8]
1415a38ba: 4885d2  test rdx, rdx
1415a38bd: 7413  je 0x1415a38d2
1415a38bf: 48c1e205  shl rdx, 5
1415a38c3: 41b808000000  mov r8d, 8
1415a38c9: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a38cd: e84e0307ff  call 0x140613c20
1415a38d2: 90  nop 
1415a38d3: 4883c428  add rsp, 0x28
1415a38d7: 5e  pop rsi
1415a38d8: 5d  pop rbp
1415a38d9: c3  ret 
1415a38da: 660f1f440000  nop word ptr [rax + rax]
1415a38e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a38e5: 55  push rbp
1415a38e6: 56  push rsi
1415a38e7: 4883ec28  sub rsp, 0x28
1415a38eb: 488daa80000000  lea rbp, [rdx + 0x80]
1415a38f2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a38f6: 0fb600  movzx eax, byte ptr [rax]
1415a38f9: 83f804  cmp eax, 4
1415a38fc: 741a  je 0x1415a3918
1415a38fe: 83f805  cmp eax, 5
1415a3901: 7522  jne 0x1415a3925
1415a3903: f645ff01  test byte ptr [rbp - 1], 1
1415a3907: 7415  je 0x1415a391e
1415a3909: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a390d: 4883c108  add rcx, 8
1415a3911: e82a562fff  call 0x140898f40
1415a3916: eb06  jmp 0x1415a391e
1415a3918: f645fe01  test byte ptr [rbp - 2], 1
1415a391c: 7512  jne 0x1415a3930
1415a391e: 4883c428  add rsp, 0x28
1415a3922: 5e  pop rsi
1415a3923: 5d  pop rbp
1415a3924: c3  ret 
1415a3925: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a3929: e862a12bff  call 0x14085da90
1415a392e: ebee  jmp 0x1415a391e
1415a3930: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a3934: 4883c108  add rcx, 8
1415a3938: e8535a2eff  call 0x140889390
1415a393d: ebdf  jmp 0x1415a391e
