1415a2940: 55  push rbp
1415a2941: 56  push rsi
1415a2942: 4881ec88000000  sub rsp, 0x88
1415a2949: 488dac2480000000  lea rbp, [rsp + 0x80]
1415a2951: 48c74500feffffff  mov qword ptr [rbp], 0xfffffffffffffffe
1415a2959: 4889ce  mov rsi, rcx
1415a295c: 0fb602  movzx eax, byte ptr [rdx]
1415a295f: 83f804  cmp eax, 4
1415a2962: 488955f0  mov qword ptr [rbp - 0x10], rdx
1415a2966: 7460  je 0x1415a29c8
1415a2968: 83f805  cmp eax, 5
1415a296b: 0f85c2000000  jne 0x1415a2a33
1415a2971: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a2975: 488945b0  mov qword ptr [rbp - 0x50], rax
1415a2979: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a297d: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1415a2981: c645ff00  mov byte ptr [rbp - 1], 0
1415a2985: c645fe01  mov byte ptr [rbp - 2], 1
1415a2989: 488d55a0  lea rdx, [rbp - 0x60]
1415a298d: 4889f1  mov rcx, rsi
1415a2990: e81bce0100  call 0x1415bf7b0
1415a2995: b101  mov cl, 1
1415a2997: 31c0  xor eax, eax
1415a2999: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a299d: 410fb610  movzx edx, byte ptr [r8]
1415a29a1: 83fa04  cmp edx, 4
1415a29a4: 7457  je 0x1415a29fd
1415a29a6: 83fa05  cmp edx, 5
1415a29a9: 0f85c0000000  jne 0x1415a2a6f
1415a29af: 84c0  test al, al
1415a29b1: 0f84d5000000  je 0x1415a2a8c
1415a29b7: 4983c008  add r8, 8
1415a29bb: 4c89c1  mov rcx, r8
1415a29be: e87d652fff  call 0x140898f40
1415a29c3: e9c4000000  jmp 0x1415a2a8c
1415a29c8: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a29cc: 488945d0  mov qword ptr [rbp - 0x30], rax
1415a29d0: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a29d4: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
1415a29d8: c645ff01  mov byte ptr [rbp - 1], 1
1415a29dc: c645fe00  mov byte ptr [rbp - 2], 0
1415a29e0: 488d55c0  lea rdx, [rbp - 0x40]
1415a29e4: 4889f1  mov rcx, rsi
1415a29e7: e804e50000  call 0x1415b0ef0
1415a29ec: b001  mov al, 1
1415a29ee: 31c9  xor ecx, ecx
1415a29f0: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a29f4: 410fb610  movzx edx, byte ptr [r8]
1415a29f8: 83fa04  cmp edx, 4
1415a29fb: 75a9  jne 0x1415a29a6
1415a29fd: 84c9  test cl, cl
1415a29ff: 0f8487000000  je 0x1415a2a8c
1415a2a05: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a2a09: 498b5018  mov rdx, qword ptr [r8 + 0x18]
1415a2a0d: 48894de0  mov qword ptr [rbp - 0x20], rcx
1415a2a11: e85a632eff  call 0x140888d70
1415a2a16: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a2a1a: 488b5008  mov rdx, qword ptr [rax + 8]
1415a2a1e: 4885d2  test rdx, rdx
1415a2a21: 7469  je 0x1415a2a8c
1415a2a23: 48c1e205  shl rdx, 5
1415a2a27: 41b808000000  mov r8d, 8
1415a2a2d: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a2a31: eb54  jmp 0x1415a2a87
1415a2a33: c645ff01  mov byte ptr [rbp - 1], 1
1415a2a37: c645fe01  mov byte ptr [rbp - 2], 1
1415a2a3b: 4c8d05766a6d01  lea r8, [rip + 0x16d6a76]
1415a2a42: 4889d1  mov rcx, rdx
1415a2a45: 488d55ef  lea rdx, [rbp - 0x11]
1415a2a49: e812703501  call 0x1428f9a60
1415a2a4e: 48894608  mov qword ptr [rsi + 8], rax
1415a2a52: c70601000000  mov dword ptr [rsi], 1
1415a2a58: b001  mov al, 1
1415a2a5a: b101  mov cl, 1
1415a2a5c: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a2a60: 410fb610  movzx edx, byte ptr [r8]
1415a2a64: 83fa04  cmp edx, 4
1415a2a67: 0f8539ffffff  jne 0x1415a29a6
1415a2a6d: eb8e  jmp 0x1415a29fd
1415a2a6f: 80fa03  cmp dl, 3
1415a2a72: 7218  jb 0x1415a2a8c
1415a2a74: 498b5008  mov rdx, qword ptr [r8 + 8]
1415a2a78: 4885d2  test rdx, rdx
1415a2a7b: 740f  je 0x1415a2a8c
1415a2a7d: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a2a81: 41b801000000  mov r8d, 1
1415a2a87: e8941107ff  call 0x140613c20
1415a2a8c: 4889f0  mov rax, rsi
1415a2a8f: 4881c488000000  add rsp, 0x88
1415a2a96: 5e  pop rsi
1415a2a97: 5d  pop rbp
1415a2a98: c3  ret 
1415a2a99: 0f1f8000000000  nop dword ptr [rax]
1415a2aa0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a2aa5: 55  push rbp
1415a2aa6: 56  push rsi
1415a2aa7: 4883ec28  sub rsp, 0x28
1415a2aab: 488daa80000000  lea rbp, [rdx + 0x80]
1415a2ab2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a2ab6: 488b5008  mov rdx, qword ptr [rax + 8]
1415a2aba: 4885d2  test rdx, rdx
1415a2abd: 7413  je 0x1415a2ad2
1415a2abf: 48c1e205  shl rdx, 5
1415a2ac3: 41b808000000  mov r8d, 8
1415a2ac9: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a2acd: e84e1107ff  call 0x140613c20
1415a2ad2: 90  nop 
1415a2ad3: 4883c428  add rsp, 0x28
1415a2ad7: 5e  pop rsi
1415a2ad8: 5d  pop rbp
1415a2ad9: c3  ret 
1415a2ada: 660f1f440000  nop word ptr [rax + rax]
1415a2ae0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a2ae5: 55  push rbp
1415a2ae6: 56  push rsi
1415a2ae7: 4883ec28  sub rsp, 0x28
1415a2aeb: 488daa80000000  lea rbp, [rdx + 0x80]
1415a2af2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a2af6: 0fb600  movzx eax, byte ptr [rax]
1415a2af9: 83f804  cmp eax, 4
1415a2afc: 741a  je 0x1415a2b18
1415a2afe: 83f805  cmp eax, 5
1415a2b01: 7522  jne 0x1415a2b25
1415a2b03: f645ff01  test byte ptr [rbp - 1], 1
1415a2b07: 7415  je 0x1415a2b1e
1415a2b09: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a2b0d: 4883c108  add rcx, 8
1415a2b11: e82a642fff  call 0x140898f40
1415a2b16: eb06  jmp 0x1415a2b1e
1415a2b18: f645fe01  test byte ptr [rbp - 2], 1
1415a2b1c: 7512  jne 0x1415a2b30
1415a2b1e: 4883c428  add rsp, 0x28
1415a2b22: 5e  pop rsi
1415a2b23: 5d  pop rbp
1415a2b24: c3  ret 
1415a2b25: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a2b29: e862af2bff  call 0x14085da90
1415a2b2e: ebee  jmp 0x1415a2b1e
1415a2b30: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a2b34: 4883c108  add rcx, 8
1415a2b38: e853682eff  call 0x140889390
1415a2b3d: ebdf  jmp 0x1415a2b1e
