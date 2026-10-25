14159fb30: 55  push rbp
14159fb31: 56  push rsi
14159fb32: 4881ec88000000  sub rsp, 0x88
14159fb39: 488dac2480000000  lea rbp, [rsp + 0x80]
14159fb41: 48c74500feffffff  mov qword ptr [rbp], 0xfffffffffffffffe
14159fb49: 4889ce  mov rsi, rcx
14159fb4c: 0fb602  movzx eax, byte ptr [rdx]
14159fb4f: 83f804  cmp eax, 4
14159fb52: 488955f0  mov qword ptr [rbp - 0x10], rdx
14159fb56: 7460  je 0x14159fbb8
14159fb58: 83f805  cmp eax, 5
14159fb5b: 0f85c2000000  jne 0x14159fc23
14159fb61: 488b4218  mov rax, qword ptr [rdx + 0x18]
14159fb65: 488945b0  mov qword ptr [rbp - 0x50], rax
14159fb69: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
14159fb6d: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
14159fb71: c645ff00  mov byte ptr [rbp - 1], 0
14159fb75: c645fe01  mov byte ptr [rbp - 2], 1
14159fb79: 488d55a0  lea rdx, [rbp - 0x60]
14159fb7d: 4889f1  mov rcx, rsi
14159fb80: e82ba20100  call 0x1415b9db0
14159fb85: b101  mov cl, 1
14159fb87: 31c0  xor eax, eax
14159fb89: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
14159fb8d: 410fb610  movzx edx, byte ptr [r8]
14159fb91: 83fa04  cmp edx, 4
14159fb94: 7457  je 0x14159fbed
14159fb96: 83fa05  cmp edx, 5
14159fb99: 0f85c0000000  jne 0x14159fc5f
14159fb9f: 84c0  test al, al
14159fba1: 0f84d5000000  je 0x14159fc7c
14159fba7: 4983c008  add r8, 8
14159fbab: 4c89c1  mov rcx, r8
14159fbae: e88d932fff  call 0x140898f40
14159fbb3: e9c4000000  jmp 0x14159fc7c
14159fbb8: 488b4218  mov rax, qword ptr [rdx + 0x18]
14159fbbc: 488945d0  mov qword ptr [rbp - 0x30], rax
14159fbc0: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
14159fbc4: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
14159fbc8: c645ff01  mov byte ptr [rbp - 1], 1
14159fbcc: c645fe00  mov byte ptr [rbp - 2], 0
14159fbd0: 488d55c0  lea rdx, [rbp - 0x40]
14159fbd4: 4889f1  mov rcx, rsi
14159fbd7: e8c4dd0000  call 0x1415ad9a0
14159fbdc: b001  mov al, 1
14159fbde: 31c9  xor ecx, ecx
14159fbe0: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
14159fbe4: 410fb610  movzx edx, byte ptr [r8]
14159fbe8: 83fa04  cmp edx, 4
14159fbeb: 75a9  jne 0x14159fb96
14159fbed: 84c9  test cl, cl
14159fbef: 0f8487000000  je 0x14159fc7c
14159fbf5: 498b4810  mov rcx, qword ptr [r8 + 0x10]
14159fbf9: 498b5018  mov rdx, qword ptr [r8 + 0x18]
14159fbfd: 48894de0  mov qword ptr [rbp - 0x20], rcx
14159fc01: e86a912eff  call 0x140888d70
14159fc06: 488b45f0  mov rax, qword ptr [rbp - 0x10]
14159fc0a: 488b5008  mov rdx, qword ptr [rax + 8]
14159fc0e: 4885d2  test rdx, rdx
14159fc11: 7469  je 0x14159fc7c
14159fc13: 48c1e205  shl rdx, 5
14159fc17: 41b808000000  mov r8d, 8
14159fc1d: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
14159fc21: eb54  jmp 0x14159fc77
14159fc23: c645ff01  mov byte ptr [rbp - 1], 1
14159fc27: c645fe01  mov byte ptr [rbp - 2], 1
14159fc2b: 4c8d05a6956d01  lea r8, [rip + 0x16d95a6]
14159fc32: 4889d1  mov rcx, rdx
14159fc35: 488d55ef  lea rdx, [rbp - 0x11]
14159fc39: e8229e3501  call 0x1428f9a60
14159fc3e: 48894608  mov qword ptr [rsi + 8], rax
14159fc42: c70601000000  mov dword ptr [rsi], 1
14159fc48: b001  mov al, 1
14159fc4a: b101  mov cl, 1
14159fc4c: 4c8b45f0  mov r8, qword ptr [rbp - 0x10]
14159fc50: 410fb610  movzx edx, byte ptr [r8]
14159fc54: 83fa04  cmp edx, 4
14159fc57: 0f8539ffffff  jne 0x14159fb96
14159fc5d: eb8e  jmp 0x14159fbed
14159fc5f: 80fa03  cmp dl, 3
14159fc62: 7218  jb 0x14159fc7c
14159fc64: 498b5008  mov rdx, qword ptr [r8 + 8]
14159fc68: 4885d2  test rdx, rdx
14159fc6b: 740f  je 0x14159fc7c
14159fc6d: 498b4810  mov rcx, qword ptr [r8 + 0x10]
14159fc71: 41b801000000  mov r8d, 1
14159fc77: e8a43f07ff  call 0x140613c20
14159fc7c: 4889f0  mov rax, rsi
14159fc7f: 4881c488000000  add rsp, 0x88
14159fc86: 5e  pop rsi
14159fc87: 5d  pop rbp
14159fc88: c3  ret 
14159fc89: 0f1f8000000000  nop dword ptr [rax]
14159fc90: 4889542410  mov qword ptr [rsp + 0x10], rdx
14159fc95: 55  push rbp
14159fc96: 56  push rsi
14159fc97: 4883ec28  sub rsp, 0x28
14159fc9b: 488daa80000000  lea rbp, [rdx + 0x80]
14159fca2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
14159fca6: 488b5008  mov rdx, qword ptr [rax + 8]
14159fcaa: 4885d2  test rdx, rdx
14159fcad: 7413  je 0x14159fcc2
14159fcaf: 48c1e205  shl rdx, 5
14159fcb3: 41b808000000  mov r8d, 8
14159fcb9: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
14159fcbd: e85e3f07ff  call 0x140613c20
14159fcc2: 90  nop 
14159fcc3: 4883c428  add rsp, 0x28
14159fcc7: 5e  pop rsi
14159fcc8: 5d  pop rbp
14159fcc9: c3  ret 
14159fcca: 660f1f440000  nop word ptr [rax + rax]
14159fcd0: 4889542410  mov qword ptr [rsp + 0x10], rdx
14159fcd5: 55  push rbp
14159fcd6: 56  push rsi
14159fcd7: 4883ec28  sub rsp, 0x28
14159fcdb: 488daa80000000  lea rbp, [rdx + 0x80]
14159fce2: 488b45f0  mov rax, qword ptr [rbp - 0x10]
14159fce6: 0fb600  movzx eax, byte ptr [rax]
14159fce9: 83f804  cmp eax, 4
14159fcec: 741a  je 0x14159fd08
14159fcee: 83f805  cmp eax, 5
14159fcf1: 7522  jne 0x14159fd15
14159fcf3: f645ff01  test byte ptr [rbp - 1], 1
14159fcf7: 7415  je 0x14159fd0e
14159fcf9: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
14159fcfd: 4883c108  add rcx, 8
14159fd01: e83a922fff  call 0x140898f40
14159fd06: eb06  jmp 0x14159fd0e
14159fd08: f645fe01  test byte ptr [rbp - 2], 1
14159fd0c: 7512  jne 0x14159fd20
14159fd0e: 4883c428  add rsp, 0x28
14159fd12: 5e  pop rsi
14159fd13: 5d  pop rbp
14159fd14: c3  ret 
14159fd15: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
14159fd19: e872dd2bff  call 0x14085da90
14159fd1e: ebee  jmp 0x14159fd0e
14159fd20: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
14159fd24: 4883c108  add rcx, 8
14159fd28: e863962eff  call 0x140889390
14159fd2d: ebdf  jmp 0x14159fd0e
