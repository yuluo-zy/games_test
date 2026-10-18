1415b3890: 55  push rbp
1415b3891: 4157  push r15
1415b3893: 4156  push r14
1415b3895: 4155  push r13
1415b3897: 4154  push r12
1415b3899: 56  push rsi
1415b389a: 57  push rdi
1415b389b: 53  push rbx
1415b389c: 4881ec98000000  sub rsp, 0x98
1415b38a3: 488dac2480000000  lea rbp, [rsp + 0x80]
1415b38ab: 48c74510feffffff  mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
1415b38b3: 4889ce  mov rsi, rcx
1415b38b6: 4c8b7210  mov r14, qword ptr [rdx + 0x10]
1415b38ba: 488d4dd8  lea rcx, [rbp - 0x28]
1415b38be: e8bd475dff  call 0x140b88080
1415b38c3: 4c8b65e0  mov r12, qword ptr [rbp - 0x20]
1415b38c7: 488b7df0  mov rdi, qword ptr [rbp - 0x10]
1415b38cb: 4939fc  cmp r12, rdi
1415b38ce: 7457  je 0x1415b3927
1415b38d0: 498d5c2420  lea rbx, [r12 + 0x20]
1415b38d5: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415b38d9: 410fb60424  movzx eax, byte ptr [r12]
1415b38de: 3c06  cmp al, 6
1415b38e0: 7448  je 0x1415b392a
1415b38e2: 8845b8  mov byte ptr [rbp - 0x48], al
1415b38e5: 410f10442401  movups xmm0, xmmword ptr [r12 + 1]
1415b38eb: 410f104c2410  movups xmm1, xmmword ptr [r12 + 0x10]
1415b38f1: 0f1145b9  movups xmmword ptr [rbp - 0x47], xmm0
1415b38f5: 0f114dc8  movups xmmword ptr [rbp - 0x38], xmm1
1415b38f9: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415b3902: 4c8d054f3e6c01  lea r8, [rip + 0x16c3e4f]
1415b3909: 4889e9  mov rcx, rbp
1415b390c: 488d55b8  lea rdx, [rbp - 0x48]
1415b3910: 41b90e000000  mov r9d, 0xe
1415b3916: e8e501ffff  call 0x1415a3b00
1415b391b: 837d0001  cmp dword ptr [rbp], 1
1415b391f: 756d  jne 0x1415b398e
1415b3921: 488b4508  mov rax, qword ptr [rbp + 8]
1415b3925: eb18  jmp 0x1415b393f
1415b3927: 4c89e3  mov rbx, r12
1415b392a: 488d1507776c01  lea rdx, [rip + 0x16c7707]
1415b3931: 4c8d0528676c01  lea r8, [rip + 0x16c6728]
1415b3938: 31c9  xor ecx, ecx
1415b393a: e8e1633401  call 0x1428f9d20
1415b393f: 48894608  mov qword ptr [rsi + 8], rax
1415b3943: c70601000000  mov dword ptr [rsi], 1
1415b3949: 4829df  sub rdi, rbx
1415b394c: 48c1ef05  shr rdi, 5
1415b3950: 4889d9  mov rcx, rbx
1415b3953: 4889fa  mov rdx, rdi
1415b3956: e815542dff  call 0x140888d70
1415b395b: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415b395f: 4885d2  test rdx, rdx
1415b3962: 7413  je 0x1415b3977
1415b3964: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415b3968: 48c1e205  shl rdx, 5
1415b396c: 41b808000000  mov r8d, 8
1415b3972: e8a90206ff  call 0x140613c20
1415b3977: 4889f0  mov rax, rsi
1415b397a: 4881c498000000  add rsp, 0x98
1415b3981: 5b  pop rbx
1415b3982: 5f  pop rdi
1415b3983: 5e  pop rsi
1415b3984: 415c  pop r12
1415b3986: 415d  pop r13
1415b3988: 415e  pop r14
1415b398a: 415f  pop r15
1415b398c: 5d  pop rbp
1415b398d: c3  ret 
1415b398e: 4839fb  cmp rbx, rdi
1415b3991: 741a  je 0x1415b39ad
1415b3993: 8b4d04  mov ecx, dword ptr [rbp + 4]
1415b3996: 448b6d08  mov r13d, dword ptr [rbp + 8]
1415b399a: 498d5c2440  lea rbx, [r12 + 0x40]
1415b399f: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415b39a3: 410fb6442420  movzx eax, byte ptr [r12 + 0x20]
1415b39a9: 3c06  cmp al, 6
1415b39ab: 751d  jne 0x1415b39ca
1415b39ad: 488d1584766c01  lea rdx, [rip + 0x16c7684]
1415b39b4: 4c8d05a5666c01  lea r8, [rip + 0x16c66a5]
1415b39bb: b901000000  mov ecx, 1
1415b39c0: e85b633401  call 0x1428f9d20
1415b39c5: e975ffffff  jmp 0x1415b393f
1415b39ca: 894dfc  mov dword ptr [rbp - 4], ecx
1415b39cd: 8845b8  mov byte ptr [rbp - 0x48], al
1415b39d0: 410f10442421  movups xmm0, xmmword ptr [r12 + 0x21]
1415b39d6: 410f104c2430  movups xmm1, xmmword ptr [r12 + 0x30]
1415b39dc: 0f1145b9  movups xmmword ptr [rbp - 0x47], xmm0
1415b39e0: 0f114dc8  movups xmmword ptr [rbp - 0x38], xmm1
1415b39e4: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415b39ed: 4c8d05543a6c01  lea r8, [rip + 0x16c3a54]
1415b39f4: 4889e9  mov rcx, rbp
1415b39f7: 488d55b8  lea rdx, [rbp - 0x48]
1415b39fb: 41b904000000  mov r9d, 4
1415b3a01: e80a00ffff  call 0x1415a3a10
1415b3a06: 837d0001  cmp dword ptr [rbp], 1
1415b3a0a: 0f8411ffffff  je 0x1415b3921
1415b3a10: f20f104504  movsd xmm0, qword ptr [rbp + 4]
1415b3a15: f20f1145b0  movsd qword ptr [rbp - 0x50], xmm0
1415b3a1a: 4c8b7db0  mov r15, qword ptr [rbp - 0x50]
1415b3a1e: 4839fb  cmp rbx, rdi
1415b3a21: 7413  je 0x1415b3a36
1415b3a23: 498d5c2460  lea rbx, [r12 + 0x60]
1415b3a28: 48895de0  mov qword ptr [rbp - 0x20], rbx
1415b3a2c: 410fb6442440  movzx eax, byte ptr [r12 + 0x40]
1415b3a32: 3c06  cmp al, 6
1415b3a34: 7540  jne 0x1415b3a76
1415b3a36: 0f57c0  xorps xmm0, xmm0
1415b3a39: 4829df  sub rdi, rbx
1415b3a3c: 756f  jne 0x1415b3aad
1415b3a3e: 4c89f8  mov rax, r15
1415b3a41: 48c1e820  shr rax, 0x20
1415b3a45: 49c1e720  shl r15, 0x20
1415b3a49: 4d09ef  or r15, r13
1415b3a4c: 8b4dfc  mov ecx, dword ptr [rbp - 4]
1415b3a4f: 894e04  mov dword ptr [rsi + 4], ecx
1415b3a52: 4c897e08  mov qword ptr [rsi + 8], r15
1415b3a56: 894610  mov dword ptr [rsi + 0x10], eax
1415b3a59: f30f114614  movss dword ptr [rsi + 0x14], xmm0
1415b3a5e: 31c0  xor eax, eax
1415b3a60: 8906  mov dword ptr [rsi], eax
1415b3a62: 48c1ef05  shr rdi, 5
1415b3a66: 4889d9  mov rcx, rbx
1415b3a69: 4889fa  mov rdx, rdi
1415b3a6c: e8ff522dff  call 0x140888d70
1415b3a71: e9e5feffff  jmp 0x1415b395b
1415b3a76: 8845b8  mov byte ptr [rbp - 0x48], al
1415b3a79: 410f10442441  movups xmm0, xmmword ptr [r12 + 0x41]
1415b3a7f: 410f104c2450  movups xmm1, xmmword ptr [r12 + 0x50]
1415b3a85: 0f1145b9  movups xmmword ptr [rbp - 0x47], xmm0
1415b3a89: 0f114dc8  movups xmmword ptr [rbp - 0x38], xmm1
1415b3a8d: 4889e9  mov rcx, rbp
1415b3a90: 488d55b8  lea rdx, [rbp - 0x48]
1415b3a94: e8d768feff  call 0x14159a370
1415b3a99: 837d0001  cmp dword ptr [rbp], 1
1415b3a9d: 0f847efeffff  je 0x1415b3921
1415b3aa3: f30f104504  movss xmm0, dword ptr [rbp + 4]
1415b3aa8: 4829df  sub rdi, rbx
1415b3aab: 7491  je 0x1415b3a3e
1415b3aad: 488d159c656c01  lea rdx, [rip + 0x16c659c]
1415b3ab4: 4c8d05a5656c01  lea r8, [rip + 0x16c65a5]
1415b3abb: 4c89f1  mov rcx, r14
1415b3abe: e85d623401  call 0x1428f9d20
1415b3ac3: 48894608  mov qword ptr [rsi + 8], rax
1415b3ac7: b801000000  mov eax, 1
1415b3acc: eb92  jmp 0x1415b3a60
1415b3ace: 6690  nop 
1415b3ad0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b3ad5: 55  push rbp
1415b3ad6: 4157  push r15
1415b3ad8: 4156  push r14
1415b3ada: 4155  push r13
1415b3adc: 4154  push r12
1415b3ade: 56  push rsi
1415b3adf: 57  push rdi
1415b3ae0: 53  push rbx
1415b3ae1: 4883ec28  sub rsp, 0x28
1415b3ae5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b3aec: 488d4dd8  lea rcx, [rbp - 0x28]
1415b3af0: e86b6a2dff  call 0x14088a560
1415b3af5: 90  nop 
1415b3af6: 4883c428  add rsp, 0x28
1415b3afa: 5b  pop rbx
1415b3afb: 5f  pop rdi
1415b3afc: 5e  pop rsi
1415b3afd: 415c  pop r12
1415b3aff: 415d  pop r13
1415b3b01: 415e  pop r14
1415b3b03: 415f  pop r15
1415b3b05: 5d  pop rbp
1415b3b06: c3  ret 
1415b3b07: 660f1f840000000000  nop word ptr [rax + rax]
1415b3b10: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b3b15: 55  push rbp
1415b3b16: 4157  push r15
1415b3b18: 4156  push r14
1415b3b1a: 4155  push r13
1415b3b1c: 4154  push r12
1415b3b1e: 56  push rsi
1415b3b1f: 57  push rdi
1415b3b20: 53  push rbx
1415b3b21: 4883ec28  sub rsp, 0x28
1415b3b25: 488daa80000000  lea rbp, [rdx + 0x80]
1415b3b2c: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415b3b30: 4885d2  test rdx, rdx
1415b3b33: 7413  je 0x1415b3b48
1415b3b35: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415b3b39: 48c1e205  shl rdx, 5
1415b3b3d: 41b808000000  mov r8d, 8
1415b3b43: e8d80006ff  call 0x140613c20
1415b3b48: 90  nop 
1415b3b49: 4883c428  add rsp, 0x28
1415b3b4d: 5b  pop rbx
1415b3b4e: 5f  pop rdi
1415b3b4f: 5e  pop rsi
1415b3b50: 415c  pop r12
1415b3b52: 415d  pop r13
1415b3b54: 415e  pop r14
1415b3b56: 415f  pop r15
1415b3b58: 5d  pop rbp
1415b3b59: c3  ret 
1415b3b5a: 660f1f440000  nop word ptr [rax + rax]
1415b3b60: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b3b65: 55  push rbp
1415b3b66: 4157  push r15
1415b3b68: 4156  push r14
1415b3b6a: 4155  push r13
1415b3b6c: 4154  push r12
1415b3b6e: 56  push rsi
1415b3b6f: 57  push rdi
1415b3b70: 53  push rbx
1415b3b71: 4883ec28  sub rsp, 0x28
1415b3b75: 488daa80000000  lea rbp, [rdx + 0x80]
1415b3b7c: 488b55e8  mov rdx, qword ptr [rbp - 0x18]
1415b3b80: 4885d2  test rdx, rdx
1415b3b83: 7413  je 0x1415b3b98
1415b3b85: 488b4dd8  mov rcx, qword ptr [rbp - 0x28]
1415b3b89: 48c1e205  shl rdx, 5
1415b3b8d: 41b808000000  mov r8d, 8
1415b3b93: e8880006ff  call 0x140613c20
1415b3b98: 90  nop 
1415b3b99: 4883c428  add rsp, 0x28
1415b3b9d: 5b  pop rbx
1415b3b9e: 5f  pop rdi
1415b3b9f: 5e  pop rsi
1415b3ba0: 415c  pop r12
1415b3ba2: 415d  pop r13
1415b3ba4: 415e  pop r14
1415b3ba6: 415f  pop r15
1415b3ba8: 5d  pop rbp
1415b3ba9: c3  ret 
