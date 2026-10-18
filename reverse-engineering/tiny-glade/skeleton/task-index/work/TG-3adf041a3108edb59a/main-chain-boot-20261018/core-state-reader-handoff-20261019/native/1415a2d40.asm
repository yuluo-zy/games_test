1415a2d40: 55  push rbp
1415a2d41: 56  push rsi
1415a2d42: 4881ec88000000  sub rsp, 0x88
1415a2d49: 488dac2480000000  lea rbp, [rsp + 0x80]
1415a2d51: 48c74500feffffff  mov qword ptr [rbp], 0xfffffffffffffffe
1415a2d59: 4889ce  mov rsi, rcx
1415a2d5c: 0fb602  movzx eax, byte ptr [rdx]
1415a2d5f: 83f804  cmp eax, 4
1415a2d62: 488955f0  mov qword ptr [rbp - 0x10], rdx
1415a2d66: 7460  je 0x1415a2dc8
1415a2d68: 83f805  cmp eax, 5
1415a2d6b: 0f85c2000000  jne 0x1415a2e33
1415a2d71: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a2d75: 488945b0  mov qword ptr [rbp - 0x50], rax
1415a2d79: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a2d7d: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1415a2d81: c645ff00  mov byte ptr [rbp - 1], 0
1415a2d85: c645fe01  mov byte ptr [rbp - 2], 1
1415a2d89: 488d55a0  lea rdx, [rbp - 0x60]
1415a2d8d: 4889f1  mov rcx, rsi
1415a2d90: e8eb5e0100  call 0x1415b8c80
1415a2d95: b101  mov cl, 1
1415a2d97: 31c0  xor eax, eax
1415a2d99: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a2d9d: 410fb610  movzx edx, byte ptr [r8]
1415a2da1: 83fa04  cmp edx, 4
1415a2da4: 7457  je 0x1415a2dfd
1415a2da6: 83fa05  cmp edx, 5
1415a2da9: 0f85bd000000  jne 0x1415a2e6c
1415a2daf: 84c0  test al, al
1415a2db1: 0f84d2000000  je 0x1415a2e89
1415a2db7: 4983c008  add r8, 8
1415a2dbb: 4c89c1  mov rcx, r8
1415a2dbe: e87d612fff  call 0x140898f40
1415a2dc3: e9c1000000  jmp 0x1415a2e89
1415a2dc8: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415a2dcc: 488945d0  mov qword ptr [rbp - 0x30], rax
1415a2dd0: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415a2dd4: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
1415a2dd8: c645ff01  mov byte ptr [rbp - 1], 1
1415a2ddc: c645fe00  mov byte ptr [rbp - 2], 0
1415a2de0: 488d55c0  lea rdx, [rbp - 0x40]
1415a2de4: 4889f1  mov rcx, rsi
1415a2de7: e814a60000  call 0x1415ad400
1415a2dec: b001  mov al, 1
1415a2dee: 31c9  xor ecx, ecx
1415a2df0: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a2df4: 410fb610  movzx edx, byte ptr [r8]
1415a2df8: 83fa04  cmp edx, 4
1415a2dfb: 75a9  jne 0x1415a2da6
1415a2dfd: 84c9  test cl, cl
1415a2dff: 0f8484000000  je 0x1415a2e89
1415a2e05: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a2e09: 498b5018  mov rdx, qword ptr [r8 + 0x18]
1415a2e0d: 48894de0  mov qword ptr [rbp - 0x20], rcx
1415a2e11: e85a5f2eff  call 0x140888d70
1415a2e16: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a2e1a: 488b5008  mov rdx, qword ptr [rax + 8]
1415a2e1e: 4885d2  test rdx, rdx
1415a2e21: 7466  je 0x1415a2e89
1415a2e23: 48c1e205  shl rdx, 5
1415a2e27: 41b808000000  mov r8d, 8
1415a2e2d: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a2e31: eb51  jmp 0x1415a2e84
1415a2e33: c645ff01  mov byte ptr [rbp - 1], 1
1415a2e37: c645fe01  mov byte ptr [rbp - 2], 1
1415a2e3b: 4c8d05b6666d01  lea r8, [rip + 0x16d66b6]
1415a2e42: 4889d1  mov rcx, rdx
1415a2e45: 488d55ef  lea rdx, [rbp - 0x11]
1415a2e49: e8126c3501  call 0x1428f9a60
1415a2e4e: 48894608  mov qword ptr [rsi + 8], rax
1415a2e52: c60601  mov byte ptr [rsi], 1
1415a2e55: b001  mov al, 1
1415a2e57: b101  mov cl, 1
1415a2e59: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
1415a2e5d: 410fb610  movzx edx, byte ptr [r8]
1415a2e61: 83fa04  cmp edx, 4
1415a2e64: 0f853cffffff  jne 0x1415a2da6
1415a2e6a: eb91  jmp 0x1415a2dfd
1415a2e6c: 80fa03  cmp dl, 3
1415a2e6f: 7218  jb 0x1415a2e89
1415a2e71: 498b5008  mov rdx, qword ptr [r8 + 8]
1415a2e75: 4885d2  test rdx, rdx
1415a2e78: 740f  je 0x1415a2e89
1415a2e7a: 498b4810  mov rcx, qword ptr [r8 + 0x10]
1415a2e7e: 41b801000000  mov r8d, 1
1415a2e84: e8970d07ff  call 0x140613c20
1415a2e89: 4889f0  mov rax, rsi
1415a2e8c: 4881c488000000  add rsp, 0x88
1415a2e93: 5e  pop rsi
1415a2e94: 5d  pop rbp
1415a2e95: c3  ret 
1415a2e96: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415a2ea0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a2ea5: 55  push rbp
1415a2ea6: 56  push rsi
1415a2ea7: 4883ec28  sub rsp, 0x28
1415a2eab: 488daa80000000  lea rbp, [rdx + 0x80]
1415a2eb2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a2eb6: 488b5008  mov rdx, qword ptr [rax + 8]
1415a2eba: 4885d2  test rdx, rdx
1415a2ebd: 7413  je 0x1415a2ed2
1415a2ebf: 48c1e205  shl rdx, 5
1415a2ec3: 41b808000000  mov r8d, 8
1415a2ec9: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415a2ecd: e84e0d07ff  call 0x140613c20
1415a2ed2: 90  nop 
1415a2ed3: 4883c428  add rsp, 0x28
1415a2ed7: 5e  pop rsi
1415a2ed8: 5d  pop rbp
1415a2ed9: c3  ret 
1415a2eda: 660f1f440000  nop word ptr [rax + rax]
1415a2ee0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a2ee5: 55  push rbp
1415a2ee6: 56  push rsi
1415a2ee7: 4883ec28  sub rsp, 0x28
1415a2eeb: 488daa80000000  lea rbp, [rdx + 0x80]
1415a2ef2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a2ef6: 0fb600  movzx eax, byte ptr [rax]
1415a2ef9: 83f804  cmp eax, 4
1415a2efc: 741a  je 0x1415a2f18
1415a2efe: 83f805  cmp eax, 5
1415a2f01: 7522  jne 0x1415a2f25
1415a2f03: f645ff01  test byte ptr [rbp - 1], 1
1415a2f07: 7415  je 0x1415a2f1e
1415a2f09: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a2f0d: 4883c108  add rcx, 8
1415a2f11: e82a602fff  call 0x140898f40
1415a2f16: eb06  jmp 0x1415a2f1e
1415a2f18: f645fe01  test byte ptr [rbp - 2], 1
1415a2f1c: 7512  jne 0x1415a2f30
1415a2f1e: 4883c428  add rsp, 0x28
1415a2f22: 5e  pop rsi
1415a2f23: 5d  pop rbp
1415a2f24: c3  ret 
1415a2f25: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a2f29: e862ab2bff  call 0x14085da90
1415a2f2e: ebee  jmp 0x1415a2f1e
1415a2f30: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
1415a2f34: 4883c108  add rcx, 8
1415a2f38: e853642eff  call 0x140889390
1415a2f3d: ebdf  jmp 0x1415a2f1e
