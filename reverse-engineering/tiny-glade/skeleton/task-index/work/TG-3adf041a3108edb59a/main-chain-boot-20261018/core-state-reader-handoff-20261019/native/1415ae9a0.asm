1415ae9a0: 55  push rbp
1415ae9a1: 4157  push r15
1415ae9a3: 4156  push r14
1415ae9a5: 4154  push r12
1415ae9a7: 56  push rsi
1415ae9a8: 57  push rdi
1415ae9a9: 53  push rbx
1415ae9aa: 4881ecc0000000  sub rsp, 0xc0
1415ae9b1: 488dac2480000000  lea rbp, [rsp + 0x80]
1415ae9b9: 48c74538feffffff  mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
1415ae9c1: 4889ce  mov rsi, rcx
1415ae9c4: 4c8b7210  mov r14, qword ptr [rdx + 0x10]
1415ae9c8: 488d4d10  lea rcx, [rbp + 0x10]
1415ae9cc: e8af965dff  call 0x140b88080
1415ae9d1: 4c8b6518  mov r12, qword ptr [rbp + 0x18]
1415ae9d5: 488b7d28  mov rdi, qword ptr [rbp + 0x28]
1415ae9d9: 4939fc  cmp r12, rdi
1415ae9dc: 7464  je 0x1415aea42
1415ae9de: 498d5c2420  lea rbx, [r12 + 0x20]
1415ae9e3: 48895d18  mov qword ptr [rbp + 0x18], rbx
1415ae9e7: 410fb60424  movzx eax, byte ptr [r12]
1415ae9ec: 3c06  cmp al, 6
1415ae9ee: 7455  je 0x1415aea45
1415ae9f0: 8845f0  mov byte ptr [rbp - 0x10], al
1415ae9f3: 410f10442401  movups xmm0, xmmword ptr [r12 + 1]
1415ae9f9: 410f104c2410  movups xmm1, xmmword ptr [r12 + 0x10]
1415ae9ff: 0f1145f1  movups xmmword ptr [rbp - 0xf], xmm0
1415aea03: 0f114d00  movups xmmword ptr [rbp], xmm1
1415aea07: 488d05da096e01  lea rax, [rip + 0x16e09da]
1415aea0e: 4889442420  mov qword ptr [rsp + 0x20], rax
1415aea13: 48c744242802000000  mov qword ptr [rsp + 0x28], 2
1415aea1c: 4c8d05b5096e01  lea r8, [rip + 0x16e09b5]
1415aea23: 488d4dc8  lea rcx, [rbp - 0x38]
1415aea27: 488d55f0  lea rdx, [rbp - 0x10]
1415aea2b: 41b90e000000  mov r9d, 0xe
1415aea31: e80a35ffff  call 0x1415a1f40
1415aea36: 837dc801  cmp dword ptr [rbp - 0x38], 1
1415aea3a: 7568  jne 0x1415aeaa4
1415aea3c: 488b45d0  mov rax, qword ptr [rbp - 0x30]
1415aea40: eb18  jmp 0x1415aea5a
1415aea42: 4c89e3  mov rbx, r12
1415aea45: 488d150cc56c01  lea rdx, [rip + 0x16cc50c]
1415aea4c: 4c8d050db66c01  lea r8, [rip + 0x16cb60d]
1415aea53: 31c9  xor ecx, ecx
1415aea55: e8c6b23401  call 0x1428f9d20
1415aea5a: 488906  mov qword ptr [rsi], rax
1415aea5d: c6462202  mov byte ptr [rsi + 0x22], 2
1415aea61: 4829df  sub rdi, rbx
1415aea64: 48c1ef05  shr rdi, 5
1415aea68: 4889d9  mov rcx, rbx
1415aea6b: 4889fa  mov rdx, rdi
1415aea6e: e8fda22dff  call 0x140888d70
1415aea73: 488b5520  mov rdx, qword ptr [rbp + 0x20]
1415aea77: 4885d2  test rdx, rdx
1415aea7a: 7413  je 0x1415aea8f
1415aea7c: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415aea80: 48c1e205  shl rdx, 5
1415aea84: 41b808000000  mov r8d, 8
1415aea8a: e8915106ff  call 0x140613c20
1415aea8f: 4889f0  mov rax, rsi
1415aea92: 4881c4c0000000  add rsp, 0xc0
1415aea99: 5b  pop rbx
1415aea9a: 5f  pop rdi
1415aea9b: 5e  pop rsi
1415aea9c: 415c  pop r12
1415aea9e: 415e  pop r14
1415aeaa0: 415f  pop r15
1415aeaa2: 5d  pop rbp
1415aeaa3: c3  ret 
1415aeaa4: f20f1045cc  movsd xmm0, qword ptr [rbp - 0x34]
1415aeaa9: f20f1145e8  movsd qword ptr [rbp - 0x18], xmm0
1415aeaae: 4839fb  cmp rbx, rdi
1415aeab1: 7417  je 0x1415aeaca
1415aeab3: 4c8b7de8  mov r15, qword ptr [rbp - 0x18]
1415aeab7: 498d5c2440  lea rbx, [r12 + 0x40]
1415aeabc: 48895d18  mov qword ptr [rbp + 0x18], rbx
1415aeac0: 410fb6442420  movzx eax, byte ptr [r12 + 0x20]
1415aeac6: 3c06  cmp al, 6
1415aeac8: 751d  jne 0x1415aeae7
1415aeaca: 488d1587c46c01  lea rdx, [rip + 0x16cc487]
1415aead1: 4c8d0588b56c01  lea r8, [rip + 0x16cb588]
1415aead8: b901000000  mov ecx, 1
1415aeadd: e83eb23401  call 0x1428f9d20
1415aeae2: e973ffffff  jmp 0x1415aea5a
1415aeae7: 8845c8  mov byte ptr [rbp - 0x38], al
1415aeaea: 410f10442421  movups xmm0, xmmword ptr [r12 + 0x21]
1415aeaf0: 410f104c2430  movups xmm1, xmmword ptr [r12 + 0x30]
1415aeaf6: 0f1145c9  movups xmmword ptr [rbp - 0x37], xmm0
1415aeafa: 0f114dd8  movups xmmword ptr [rbp - 0x28], xmm1
1415aeafe: 488d05c3356e01  lea rax, [rip + 0x16e35c3]
1415aeb05: 4889442420  mov qword ptr [rsp + 0x20], rax
1415aeb0a: 48c744242806000000  mov qword ptr [rsp + 0x28], 6
1415aeb13: 4c8d050e366e01  lea r8, [rip + 0x16e360e]
1415aeb1a: 488d4df0  lea rcx, [rbp - 0x10]
1415aeb1e: 488d55c8  lea rdx, [rbp - 0x38]
1415aeb22: 41b914000000  mov r9d, 0x14
1415aeb28: e80320ffff  call 0x1415a0b30
1415aeb2d: 0fb64d0a  movzx ecx, byte ptr [rbp + 0xa]
1415aeb31: 488b45f0  mov rax, qword ptr [rbp - 0x10]
1415aeb35: 80f902  cmp cl, 2
1415aeb38: 0f841cffffff  je 0x1415aea5a
1415aeb3e: 0fb75508  movzx edx, word ptr [rbp + 8]
1415aeb42: 668955c0  mov word ptr [rbp - 0x40], dx
1415aeb46: 0f1045f8  movups xmm0, xmmword ptr [rbp - 8]
1415aeb4a: 0f2945b0  movaps xmmword ptr [rbp - 0x50], xmm0
1415aeb4e: 8b550b  mov edx, dword ptr [rbp + 0xb]
1415aeb51: 895530  mov dword ptr [rbp + 0x30], edx
1415aeb54: 0fb6550f  movzx edx, byte ptr [rbp + 0xf]
1415aeb58: 885534  mov byte ptr [rbp + 0x34], dl
1415aeb5b: 4829df  sub rdi, rbx
1415aeb5e: 753b  jne 0x1415aeb9b
1415aeb60: 0fb755c0  movzx edx, word ptr [rbp - 0x40]
1415aeb64: 66895620  mov word ptr [rsi + 0x20], dx
1415aeb68: 0f2845b0  movaps xmm0, xmmword ptr [rbp - 0x50]
1415aeb6c: 0f114610  movups xmmword ptr [rsi + 0x10], xmm0
1415aeb70: 8b5530  mov edx, dword ptr [rbp + 0x30]
1415aeb73: 895623  mov dword ptr [rsi + 0x23], edx
1415aeb76: 0fb65534  movzx edx, byte ptr [rbp + 0x34]
1415aeb7a: 885627  mov byte ptr [rsi + 0x27], dl
1415aeb7d: 4c893e  mov qword ptr [rsi], r15
1415aeb80: 48894608  mov qword ptr [rsi + 8], rax
1415aeb84: 884e22  mov byte ptr [rsi + 0x22], cl
1415aeb87: 48c1ef05  shr rdi, 5
1415aeb8b: 4889d9  mov rcx, rbx
1415aeb8e: 4889fa  mov rdx, rdi
1415aeb91: e8daa12dff  call 0x140888d70
1415aeb96: e9d8feffff  jmp 0x1415aea73
1415aeb9b: 488d15aeb46c01  lea rdx, [rip + 0x16cb4ae]
1415aeba2: 4c8d05b7b46c01  lea r8, [rip + 0x16cb4b7]
1415aeba9: 4c89f1  mov rcx, r14
1415aebac: e86fb13401  call 0x1428f9d20
1415aebb1: 488906  mov qword ptr [rsi], rax
1415aebb4: b102  mov cl, 2
1415aebb6: ebcc  jmp 0x1415aeb84
1415aebb8: 0f1f840000000000  nop dword ptr [rax + rax]
1415aebc0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415aebc5: 55  push rbp
1415aebc6: 4157  push r15
1415aebc8: 4156  push r14
1415aebca: 4154  push r12
1415aebcc: 56  push rsi
1415aebcd: 57  push rdi
1415aebce: 53  push rbx
1415aebcf: 4883ec30  sub rsp, 0x30
1415aebd3: 488daa80000000  lea rbp, [rdx + 0x80]
1415aebda: 488b5520  mov rdx, qword ptr [rbp + 0x20]
1415aebde: 4885d2  test rdx, rdx
1415aebe1: 7413  je 0x1415aebf6
1415aebe3: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415aebe7: 48c1e205  shl rdx, 5
1415aebeb: 41b808000000  mov r8d, 8
1415aebf1: e82a5006ff  call 0x140613c20
1415aebf6: 90  nop 
1415aebf7: 4883c430  add rsp, 0x30
1415aebfb: 5b  pop rbx
1415aebfc: 5f  pop rdi
1415aebfd: 5e  pop rsi
1415aebfe: 415c  pop r12
1415aec00: 415e  pop r14
1415aec02: 415f  pop r15
1415aec04: 5d  pop rbp
1415aec05: c3  ret 
1415aec06: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415aec10: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415aec15: 55  push rbp
1415aec16: 4157  push r15
1415aec18: 4156  push r14
1415aec1a: 4154  push r12
1415aec1c: 56  push rsi
1415aec1d: 57  push rdi
1415aec1e: 53  push rbx
1415aec1f: 4883ec30  sub rsp, 0x30
1415aec23: 488daa80000000  lea rbp, [rdx + 0x80]
1415aec2a: 488b5520  mov rdx, qword ptr [rbp + 0x20]
1415aec2e: 4885d2  test rdx, rdx
1415aec31: 7413  je 0x1415aec46
1415aec33: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415aec37: 48c1e205  shl rdx, 5
1415aec3b: 41b808000000  mov r8d, 8
1415aec41: e8da4f06ff  call 0x140613c20
1415aec46: 90  nop 
1415aec47: 4883c430  add rsp, 0x30
1415aec4b: 5b  pop rbx
1415aec4c: 5f  pop rdi
1415aec4d: 5e  pop rsi
1415aec4e: 415c  pop r12
1415aec50: 415e  pop r14
1415aec52: 415f  pop r15
1415aec54: 5d  pop rbp
1415aec55: c3  ret 
1415aec56: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415aec60: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415aec65: 55  push rbp
1415aec66: 4157  push r15
1415aec68: 4156  push r14
1415aec6a: 4154  push r12
1415aec6c: 56  push rsi
1415aec6d: 57  push rdi
1415aec6e: 53  push rbx
1415aec6f: 4883ec30  sub rsp, 0x30
1415aec73: 488daa80000000  lea rbp, [rdx + 0x80]
1415aec7a: 488d4d10  lea rcx, [rbp + 0x10]
1415aec7e: e8ddb82dff  call 0x14088a560
1415aec83: 90  nop 
1415aec84: 4883c430  add rsp, 0x30
1415aec88: 5b  pop rbx
1415aec89: 5f  pop rdi
1415aec8a: 5e  pop rsi
1415aec8b: 415c  pop r12
1415aec8d: 415e  pop r14
1415aec8f: 415f  pop r15
1415aec91: 5d  pop rbp
1415aec92: c3  ret 
