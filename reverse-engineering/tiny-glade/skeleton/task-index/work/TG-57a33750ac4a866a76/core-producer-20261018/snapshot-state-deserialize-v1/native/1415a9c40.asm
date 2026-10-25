1415a9c40: 55  push rbp
1415a9c41: 4157  push r15
1415a9c43: 4156  push r14
1415a9c45: 56  push rsi
1415a9c46: 57  push rdi
1415a9c47: 53  push rbx
1415a9c48: 4881ec98000000  sub rsp, 0x98
1415a9c4f: 488dac2480000000  lea rbp, [rsp + 0x80]
1415a9c57: 0f297500  movaps xmmword ptr [rbp], xmm6
1415a9c5b: 48c745f8feffffff  mov qword ptr [rbp - 8], 0xfffffffffffffffe
1415a9c63: 4889ce  mov rsi, rcx
1415a9c66: 4c8b7210  mov r14, qword ptr [rdx + 0x10]
1415a9c6a: 488d4dc8  lea rcx, [rbp - 0x38]
1415a9c6e: e80de45dff  call 0x140b88080
1415a9c73: 4c8b7dd0  mov r15, qword ptr [rbp - 0x30]
1415a9c77: 488b7de0  mov rdi, qword ptr [rbp - 0x20]
1415a9c7b: 4939ff  cmp r15, rdi
1415a9c7e: 743e  je 0x1415a9cbe
1415a9c80: 498d5f20  lea rbx, [r15 + 0x20]
1415a9c84: 48895dd0  mov qword ptr [rbp - 0x30], rbx
1415a9c88: 410fb607  movzx eax, byte ptr [r15]
1415a9c8c: 3c06  cmp al, 6
1415a9c8e: 7431  je 0x1415a9cc1
1415a9c90: 8845a8  mov byte ptr [rbp - 0x58], al
1415a9c93: 410f104701  movups xmm0, xmmword ptr [r15 + 1]
1415a9c98: 410f104f10  movups xmm1, xmmword ptr [r15 + 0x10]
1415a9c9d: 0f1145a9  movups xmmword ptr [rbp - 0x57], xmm0
1415a9ca1: 0f114db8  movups xmmword ptr [rbp - 0x48], xmm1
1415a9ca5: 488d4de8  lea rcx, [rbp - 0x18]
1415a9ca9: 488d55a8  lea rdx, [rbp - 0x58]
1415a9cad: e8be06ffff  call 0x14159a370
1415a9cb2: 837de801  cmp dword ptr [rbp - 0x18], 1
1415a9cb6: 756d  jne 0x1415a9d25
1415a9cb8: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415a9cbc: eb18  jmp 0x1415a9cd6
1415a9cbe: 4c89fb  mov rbx, r15
1415a9cc1: 488d15e8046d01  lea rdx, [rip + 0x16d04e8]
1415a9cc8: 4c8d0591036d01  lea r8, [rip + 0x16d0391]
1415a9ccf: 31c9  xor ecx, ecx
1415a9cd1: e84a003501  call 0x1428f9d20
1415a9cd6: 48894608  mov qword ptr [rsi + 8], rax
1415a9cda: c70601000000  mov dword ptr [rsi], 1
1415a9ce0: 4829df  sub rdi, rbx
1415a9ce3: 48c1ef05  shr rdi, 5
1415a9ce7: 4889d9  mov rcx, rbx
1415a9cea: 4889fa  mov rdx, rdi
1415a9ced: e87ef02dff  call 0x140888d70
1415a9cf2: 488b55d8  mov rdx, qword ptr [rbp - 0x28]
1415a9cf6: 4885d2  test rdx, rdx
1415a9cf9: 7413  je 0x1415a9d0e
1415a9cfb: 488b4dc8  mov rcx, qword ptr [rbp - 0x38]
1415a9cff: 48c1e205  shl rdx, 5
1415a9d03: 41b808000000  mov r8d, 8
1415a9d09: e8129f06ff  call 0x140613c20
1415a9d0e: 4889f0  mov rax, rsi
1415a9d11: 0f287500  movaps xmm6, xmmword ptr [rbp]
1415a9d15: 4881c498000000  add rsp, 0x98
1415a9d1c: 5b  pop rbx
1415a9d1d: 5f  pop rdi
1415a9d1e: 5e  pop rsi
1415a9d1f: 415e  pop r14
1415a9d21: 415f  pop r15
1415a9d23: 5d  pop rbp
1415a9d24: c3  ret 
1415a9d25: 4839fb  cmp rbx, rdi
1415a9d28: 7416  je 0x1415a9d40
1415a9d2a: f30f1075ec  movss xmm6, dword ptr [rbp - 0x14]
1415a9d2f: 498d5f40  lea rbx, [r15 + 0x40]
1415a9d33: 48895dd0  mov qword ptr [rbp - 0x30], rbx
1415a9d37: 410fb64720  movzx eax, byte ptr [r15 + 0x20]
1415a9d3c: 3c06  cmp al, 6
1415a9d3e: 751d  jne 0x1415a9d5d
1415a9d40: 488d1569046d01  lea rdx, [rip + 0x16d0469]
1415a9d47: 4c8d0512036d01  lea r8, [rip + 0x16d0312]
1415a9d4e: b901000000  mov ecx, 1
1415a9d53: e8c8ff3401  call 0x1428f9d20
1415a9d58: e979ffffff  jmp 0x1415a9cd6
1415a9d5d: 8845a8  mov byte ptr [rbp - 0x58], al
1415a9d60: 410f104721  movups xmm0, xmmword ptr [r15 + 0x21]
1415a9d65: 410f104f30  movups xmm1, xmmword ptr [r15 + 0x30]
1415a9d6a: 0f1145a9  movups xmmword ptr [rbp - 0x57], xmm0
1415a9d6e: 0f114db8  movups xmmword ptr [rbp - 0x48], xmm1
1415a9d72: 488d4de8  lea rcx, [rbp - 0x18]
1415a9d76: 488d55a8  lea rdx, [rbp - 0x58]
1415a9d7a: e8f105ffff  call 0x14159a370
1415a9d7f: 837de801  cmp dword ptr [rbp - 0x18], 1
1415a9d83: 0f842fffffff  je 0x1415a9cb8
1415a9d89: 4829df  sub rdi, rbx
1415a9d8c: 7527  jne 0x1415a9db5
1415a9d8e: f30f1045ec  movss xmm0, dword ptr [rbp - 0x14]
1415a9d93: f30f117604  movss dword ptr [rsi + 4], xmm6
1415a9d98: f30f114608  movss dword ptr [rsi + 8], xmm0
1415a9d9d: 31c0  xor eax, eax
1415a9d9f: 8906  mov dword ptr [rsi], eax
1415a9da1: 48c1ef05  shr rdi, 5
1415a9da5: 4889d9  mov rcx, rbx
1415a9da8: 4889fa  mov rdx, rdi
1415a9dab: e8c0ef2dff  call 0x140888d70
1415a9db0: e93dffffff  jmp 0x1415a9cf2
1415a9db5: 488d1594026d01  lea rdx, [rip + 0x16d0294]
1415a9dbc: 4c8d059d026d01  lea r8, [rip + 0x16d029d]
1415a9dc3: 4c89f1  mov rcx, r14
1415a9dc6: e855ff3401  call 0x1428f9d20
1415a9dcb: 48894608  mov qword ptr [rsi + 8], rax
1415a9dcf: b801000000  mov eax, 1
1415a9dd4: ebc9  jmp 0x1415a9d9f
1415a9dd6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415a9de0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a9de5: 55  push rbp
1415a9de6: 4157  push r15
1415a9de8: 4156  push r14
1415a9dea: 56  push rsi
1415a9deb: 57  push rdi
1415a9dec: 53  push rbx
1415a9ded: 4883ec38  sub rsp, 0x38
1415a9df1: 488daa80000000  lea rbp, [rdx + 0x80]
1415a9df8: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415a9dfd: 488b55d8  mov rdx, qword ptr [rbp - 0x28]
1415a9e01: 4885d2  test rdx, rdx
1415a9e04: 7413  je 0x1415a9e19
1415a9e06: 488b4dc8  mov rcx, qword ptr [rbp - 0x38]
1415a9e0a: 48c1e205  shl rdx, 5
1415a9e0e: 41b808000000  mov r8d, 8
1415a9e14: e8079e06ff  call 0x140613c20
1415a9e19: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415a9e1e: 4883c438  add rsp, 0x38
1415a9e22: 5b  pop rbx
1415a9e23: 5f  pop rdi
1415a9e24: 5e  pop rsi
1415a9e25: 415e  pop r14
1415a9e27: 415f  pop r15
1415a9e29: 5d  pop rbp
1415a9e2a: c3  ret 
1415a9e2b: 0f1f440000  nop dword ptr [rax + rax]
1415a9e30: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a9e35: 55  push rbp
1415a9e36: 4157  push r15
1415a9e38: 4156  push r14
1415a9e3a: 56  push rsi
1415a9e3b: 57  push rdi
1415a9e3c: 53  push rbx
1415a9e3d: 4883ec38  sub rsp, 0x38
1415a9e41: 488daa80000000  lea rbp, [rdx + 0x80]
1415a9e48: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415a9e4d: 488b55d8  mov rdx, qword ptr [rbp - 0x28]
1415a9e51: 4885d2  test rdx, rdx
1415a9e54: 7413  je 0x1415a9e69
1415a9e56: 488b4dc8  mov rcx, qword ptr [rbp - 0x38]
1415a9e5a: 48c1e205  shl rdx, 5
1415a9e5e: 41b808000000  mov r8d, 8
1415a9e64: e8b79d06ff  call 0x140613c20
1415a9e69: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415a9e6e: 4883c438  add rsp, 0x38
1415a9e72: 5b  pop rbx
1415a9e73: 5f  pop rdi
1415a9e74: 5e  pop rsi
1415a9e75: 415e  pop r14
1415a9e77: 415f  pop r15
1415a9e79: 5d  pop rbp
1415a9e7a: c3  ret 
1415a9e7b: 0f1f440000  nop dword ptr [rax + rax]
1415a9e80: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415a9e85: 55  push rbp
1415a9e86: 4157  push r15
1415a9e88: 4156  push r14
1415a9e8a: 56  push rsi
1415a9e8b: 57  push rdi
1415a9e8c: 53  push rbx
1415a9e8d: 4883ec38  sub rsp, 0x38
1415a9e91: 488daa80000000  lea rbp, [rdx + 0x80]
1415a9e98: 0f29742420  movaps xmmword ptr [rsp + 0x20], xmm6
1415a9e9d: 488d4dc8  lea rcx, [rbp - 0x38]
1415a9ea1: e8ba062eff  call 0x14088a560
1415a9ea6: 0f28742420  movaps xmm6, xmmword ptr [rsp + 0x20]
1415a9eab: 4883c438  add rsp, 0x38
1415a9eaf: 5b  pop rbx
1415a9eb0: 5f  pop rdi
1415a9eb1: 5e  pop rsi
1415a9eb2: 415e  pop r14
1415a9eb4: 415f  pop r15
1415a9eb6: 5d  pop rbp
1415a9eb7: c3  ret 
