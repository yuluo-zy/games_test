1415d3c70: 55  push rbp
1415d3c71: 4156  push r14
1415d3c73: 56  push rsi
1415d3c74: 57  push rdi
1415d3c75: 53  push rbx
1415d3c76: 4881ecb0000000  sub rsp, 0xb0
1415d3c7d: 488dac2480000000  lea rbp, [rsp + 0x80]
1415d3c85: 48c74528feffffff  mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
1415d3c8d: 4889ce  mov rsi, rcx
1415d3c90: 0f104218  movups xmm0, xmmword ptr [rdx + 0x18]
1415d3c94: 0f104a28  movups xmm1, xmmword ptr [rdx + 0x28]
1415d3c98: 0f294d00  movaps xmmword ptr [rbp], xmm1
1415d3c9c: 0f2945f0  movaps xmmword ptr [rbp - 0x10], xmm0
1415d3ca0: 4c8b0a  mov r9, qword ptr [rdx]
1415d3ca3: 488b4a08  mov rcx, qword ptr [rdx + 8]
1415d3ca7: 488b5210  mov rdx, qword ptr [rdx + 0x10]
1415d3cab: 4883fa05  cmp rdx, 5
1415d3caf: 0f85c8000000  jne 0x1415d3d7d
1415d3cb5: 8b01  mov eax, dword ptr [rcx]
1415d3cb7: 33057fe46b01  xor eax, dword ptr [rip + 0x16be47f]
1415d3cbd: 440fb64104  movzx r8d, byte ptr [rcx + 4]
1415d3cc2: 44320577e46b01  xor r8b, byte ptr [rip + 0x16be477]
1415d3cc9: 450fb6c0  movzx r8d, r8b
1415d3ccd: 4109c0  or r8d, eax
1415d3cd0: 742d  je 0x1415d3cff
1415d3cd2: 8b01  mov eax, dword ptr [rcx]
1415d3cd4: 330567e46b01  xor eax, dword ptr [rip + 0x16be467]
1415d3cda: 440fb64104  movzx r8d, byte ptr [rcx + 4]
1415d3cdf: 4432055fe46b01  xor r8b, byte ptr [rip + 0x16be45f]
1415d3ce6: 450fb6c0  movzx r8d, r8b
1415d3cea: 4109c0  or r8d, eax
1415d3ced: 0f858a000000  jne 0x1415d3d7d
1415d3cf3: b301  mov bl, 1
1415d3cf5: 4531f6  xor r14d, r14d
1415d3cf8: 4d85c9  test r9, r9
1415d3cfb: 750c  jne 0x1415d3d09
1415d3cfd: eb18  jmp 0x1415d3d17
1415d3cff: 31db  xor ebx, ebx
1415d3d01: 4531f6  xor r14d, r14d
1415d3d04: 4d85c9  test r9, r9
1415d3d07: 740e  je 0x1415d3d17
1415d3d09: 41b801000000  mov r8d, 1
1415d3d0f: 4c89ca  mov rdx, r9
1415d3d12: e809ff03ff  call 0x140613c20
1415d3d17: 0f2845f0  movaps xmm0, xmmword ptr [rbp - 0x10]
1415d3d1b: 0f284d00  movaps xmm1, xmmword ptr [rbp]
1415d3d1f: 0f294db0  movaps xmmword ptr [rbp - 0x50], xmm1
1415d3d23: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1415d3d27: 4584f6  test r14b, r14b
1415d3d2a: 7416  je 0x1415d3d42
1415d3d2c: 48897e08  mov qword ptr [rsi + 8], rdi
1415d3d30: c60602  mov byte ptr [rsi], 2
1415d3d33: c6452700  mov byte ptr [rbp + 0x27], 0
1415d3d37: 488d4da0  lea rcx, [rbp - 0x60]
1415d3d3b: e8f0a62bff  call 0x14088e430
1415d3d40: eb2a  jmp 0x1415d3d6c
1415d3d42: 0f2845f0  movaps xmm0, xmmword ptr [rbp - 0x10]
1415d3d46: 0f284d00  movaps xmm1, xmmword ptr [rbp]
1415d3d4a: 0f114de0  movups xmmword ptr [rbp - 0x20], xmm1
1415d3d4e: 0f1145d0  movups xmmword ptr [rbp - 0x30], xmm0
1415d3d52: 881e  mov byte ptr [rsi], bl
1415d3d54: 0f1045c9  movups xmm0, xmmword ptr [rbp - 0x37]
1415d3d58: 0f104dd9  movups xmm1, xmmword ptr [rbp - 0x27]
1415d3d5c: 0f114601  movups xmmword ptr [rsi + 1], xmm0
1415d3d60: 0f114e11  movups xmmword ptr [rsi + 0x11], xmm1
1415d3d64: 488b45e8  mov rax, qword ptr [rbp - 0x18]
1415d3d68: 48894620  mov qword ptr [rsi + 0x20], rax
1415d3d6c: 4889f0  mov rax, rsi
1415d3d6f: 4881c4b0000000  add rsp, 0xb0
1415d3d76: 5b  pop rbx
1415d3d77: 5f  pop rdi
1415d3d78: 5e  pop rsi
1415d3d79: 415e  pop r14
1415d3d7b: 5d  pop rbp
1415d3d7c: c3  ret 
1415d3d7d: 4c894d18  mov qword ptr [rbp + 0x18], r9
1415d3d81: 4c8d05c0e36b01  lea r8, [rip + 0x16be3c0]
1415d3d88: 41b902000000  mov r9d, 2
1415d3d8e: 48894d10  mov qword ptr [rbp + 0x10], rcx
1415d3d92: e889603201  call 0x1428f9e20
1415d3d97: 4889c7  mov rdi, rax
1415d3d9a: 41b601  mov r14b, 1
1415d3d9d: 4c8b4d18  mov r9, qword ptr [rbp + 0x18]
1415d3da1: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415d3da5: 4d85c9  test r9, r9
1415d3da8: 0f855bffffff  jne 0x1415d3d09
1415d3dae: e964ffffff  jmp 0x1415d3d17
1415d3db3: 666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415d3dc0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415d3dc5: 55  push rbp
1415d3dc6: 4156  push r14
1415d3dc8: 56  push rsi
1415d3dc9: 57  push rdi
1415d3dca: 53  push rbx
1415d3dcb: 4883ec20  sub rsp, 0x20
1415d3dcf: 488daa80000000  lea rbp, [rdx + 0x80]
1415d3dd6: 48837d1800  cmp qword ptr [rbp + 0x18], 0
1415d3ddb: 7413  je 0x1415d3df0
1415d3ddd: 41b801000000  mov r8d, 1
1415d3de3: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415d3de7: 488b5518  mov rdx, qword ptr [rbp + 0x18]
1415d3deb: e830fe03ff  call 0x140613c20
1415d3df0: c6452701  mov byte ptr [rbp + 0x27], 1
1415d3df4: 4883c420  add rsp, 0x20
1415d3df8: 5b  pop rbx
1415d3df9: 5f  pop rdi
1415d3dfa: 5e  pop rsi
1415d3dfb: 415e  pop r14
1415d3dfd: 5d  pop rbp
1415d3dfe: c3  ret 
1415d3dff: 90  nop 
1415d3e00: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415d3e05: 55  push rbp
1415d3e06: 4156  push r14
1415d3e08: 56  push rsi
1415d3e09: 57  push rdi
1415d3e0a: 53  push rbx
1415d3e0b: 4883ec20  sub rsp, 0x20
1415d3e0f: 488daa80000000  lea rbp, [rdx + 0x80]
1415d3e16: 807d2700  cmp byte ptr [rbp + 0x27], 0
1415d3e1a: 7409  je 0x1415d3e25
1415d3e1c: 488d4df0  lea rcx, [rbp - 0x10]
1415d3e20: e80ba62bff  call 0x14088e430
1415d3e25: 90  nop 
1415d3e26: 4883c420  add rsp, 0x20
1415d3e2a: 5b  pop rbx
1415d3e2b: 5f  pop rdi
1415d3e2c: 5e  pop rsi
1415d3e2d: 415e  pop r14
1415d3e2f: 5d  pop rbp
1415d3e30: c3  ret 
