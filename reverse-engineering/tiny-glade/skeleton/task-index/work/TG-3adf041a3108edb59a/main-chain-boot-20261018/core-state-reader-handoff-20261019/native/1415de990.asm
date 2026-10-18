1415de990: 55  push rbp
1415de991: 4157  push r15
1415de993: 4156  push r14
1415de995: 4155  push r13
1415de997: 4154  push r12
1415de999: 56  push rsi
1415de99a: 57  push rdi
1415de99b: 53  push rbx
1415de99c: 4881ec28010000  sub rsp, 0x128
1415de9a3: 488dac2480000000  lea rbp, [rsp + 0x80]
1415de9ab: 48c785a0000000feffffff  mov qword ptr [rbp + 0xa0], 0xfffffffffffffffe
1415de9b6: 4989cc  mov r12, rcx
1415de9b9: 0fb602  movzx eax, byte ptr [rdx]
1415de9bc: 83f806  cmp eax, 6
1415de9bf: 488955e0  mov qword ptr [rbp - 0x20], rdx
1415de9c3: 488945d8  mov qword ptr [rbp - 0x28], rax
1415de9c7: 7531  jne 0x1415de9fa
1415de9c9: c645f00d  mov byte ptr [rbp - 0x10], 0xd
1415de9cd: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415de9d4: 488d15adb86901  lea rdx, [rip + 0x169b8ad]
1415de9db: 4c8d057eb66901  lea r8, [rip + 0x169b67e]
1415de9e2: 488d4df0  lea rcx, [rbp - 0x10]
1415de9e6: e815b02e01  call 0x1428c9a00
1415de9eb: 4989442408  mov qword ptr [r12 + 8], rax
1415de9f0: 41c6042401  mov byte ptr [r12], 1
1415de9f5: e9a8020000  jmp 0x1415deca2
1415de9fa: 83f805  cmp eax, 5
1415de9fd: 0f85cf020000  jne 0x1415decd2
1415dea03: 0f104208  movups xmm0, xmmword ptr [rdx + 8]
1415dea07: 0f2945a0  movaps xmmword ptr [rbp - 0x60], xmm0
1415dea0b: 488b4218  mov rax, qword ptr [rdx + 0x18]
1415dea0f: 488945e8  mov qword ptr [rbp - 0x18], rax
1415dea13: 488945b0  mov qword ptr [rbp - 0x50], rax
1415dea17: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415dea1e: 488d4df0  lea rcx, [rbp - 0x10]
1415dea22: 488d55a0  lea rdx, [rbp - 0x60]
1415dea26: e8a5965aff  call 0x140b880d0
1415dea2b: 4c8d6d71  lea r13, [rbp + 0x71]
1415dea2f: c6859e00000038  mov byte ptr [rbp + 0x9e], 0x38
1415dea36: 4c8d75c0  lea r14, [rbp - 0x40]
1415dea3a: 4c8d7df0  lea r15, [rbp - 0x10]
1415dea3e: 488d1dabd26901  lea rbx, [rip + 0x169d2ab]
1415dea45: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415dea50: 4c89f1  mov rcx, r14
1415dea53: 4c89fa  mov rdx, r15
1415dea56: e8f5d9e5ff  call 0x14143c450
1415dea5b: 488b45c0  mov rax, qword ptr [rbp - 0x40]
1415dea5f: 4885c0  test rax, rax
1415dea62: 0f84b7010000  je 0x1415dec1f
1415dea68: 488b4dd0  mov rcx, qword ptr [rbp - 0x30]
1415dea6c: 488d1449  lea rdx, [rcx + rcx*2]
1415dea70: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415dea78: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415dea80: 4c894d60  mov qword ptr [rbp + 0x60], r9
1415dea84: 488bbcd078010000  mov rdi, qword ptr [rax + rdx*8 + 0x178]
1415dea8c: 48c1e105  shl rcx, 5
1415dea90: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415dea94: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415dea99: 0f294570  movaps xmmword ptr [rbp + 0x70], xmm0
1415dea9d: 0f298d80000000  movaps xmmword ptr [rbp + 0x80], xmm1
1415deaa4: 4c89c0  mov rax, r8
1415deaa7: 48f7d8  neg rax
1415deaaa: 0f806f010000  jo 0x1415dec1f
1415deab0: 4c898590000000  mov qword ptr [rbp + 0x90], r8
1415deab7: 0fb64538  movzx eax, byte ptr [rbp + 0x38]
1415deabb: 4883f806  cmp rax, 6
1415deabf: 0f87f1000000  ja 0x1415debb6
1415deac5: 48630483  movsxd rax, dword ptr [rbx + rax*4]
1415deac9: 4801d8  add rax, rbx
1415deacc: ffe0  jmp rax
1415deace: 0f284570  movaps xmm0, xmmword ptr [rbp + 0x70]
1415dead2: 0f288d80000000  movaps xmm1, xmmword ptr [rbp + 0x80]
1415dead9: 488d4538  lea rax, [rbp + 0x38]
1415deadd: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415deae1: 0f1100  movups xmmword ptr [rax], xmm0
1415deae4: 40b601  mov sil, 1
1415deae7: 4883ff02  cmp rdi, 2
1415deaeb: 488b4d60  mov rcx, qword ptr [rbp + 0x60]
1415deaef: 750e  jne 0x1415deaff
1415deaf1: 0fb701  movzx eax, word ptr [rcx]
1415deaf4: 663b058c024f01  cmp ax, word ptr [rip + 0x14f028c]
1415deafb: 400f95c6  setne sil
1415deaff: 488b9590000000  mov rdx, qword ptr [rbp + 0x90]
1415deb06: 4885d2  test rdx, rdx
1415deb09: 740b  je 0x1415deb16
1415deb0b: 41b801000000  mov r8d, 1
1415deb11: e80a5103ff  call 0x140613c20
1415deb16: 4084f6  test sil, sil
1415deb19: 0f84a5000000  je 0x1415debc4
1415deb1f: 0fb64538  movzx eax, byte ptr [rbp + 0x38]
1415deb23: c6453806  mov byte ptr [rbp + 0x38], 6
1415deb27: 3c06  cmp al, 6
1415deb29: 0f843f010000  je 0x1415dec6e
1415deb2f: 884570  mov byte ptr [rbp + 0x70], al
1415deb32: 488d4538  lea rax, [rbp + 0x38]
1415deb36: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415deb3a: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415deb3e: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415deb43: 410f114500  movups xmmword ptr [r13], xmm0
1415deb48: 488d4d70  lea rcx, [rbp + 0x70]
1415deb4c: e8ef4dfcff  call 0x1415a3940
1415deb51: 4885c0  test rax, rax
1415deb54: 0f84f6feffff  je 0x1415dea50
1415deb5a: e920010000  jmp 0x1415dec7f
1415deb5f: 488b5540  mov rdx, qword ptr [rbp + 0x40]
1415deb63: 4885d2  test rdx, rdx
1415deb66: 0f8462ffffff  je 0x1415deace
1415deb6c: 488b4d48  mov rcx, qword ptr [rbp + 0x48]
1415deb70: 41b801000000  mov r8d, 1
1415deb76: e8a55003ff  call 0x140613c20
1415deb7b: e94effffff  jmp 0x1415deace
1415deb80: 488b4d48  mov rcx, qword ptr [rbp + 0x48]
1415deb84: 488b5550  mov rdx, qword ptr [rbp + 0x50]
1415deb88: 48894d68  mov qword ptr [rbp + 0x68], rcx
1415deb8c: e8dfa12aff  call 0x140888d70
1415deb91: 488b5540  mov rdx, qword ptr [rbp + 0x40]
1415deb95: 4885d2  test rdx, rdx
1415deb98: 0f8430ffffff  je 0x1415deace
1415deb9e: 48c1e205  shl rdx, 5
1415deba2: 41b808000000  mov r8d, 8
1415deba8: 488b4d68  mov rcx, qword ptr [rbp + 0x68]
1415debac: e86f5003ff  call 0x140613c20
1415debb1: e918ffffff  jmp 0x1415deace
1415debb6: 488d4d40  lea rcx, [rbp + 0x40]
1415debba: e881a32bff  call 0x140898f40
1415debbf: e90affffff  jmp 0x1415deace
1415debc4: 80bd9e00000038  cmp byte ptr [rbp + 0x9e], 0x38
1415debcb: 0f8573010000  jne 0x1415ded44
1415debd1: 0fb64538  movzx eax, byte ptr [rbp + 0x38]
1415debd5: c6453806  mov byte ptr [rbp + 0x38], 6
1415debd9: 3c06  cmp al, 6
1415debdb: 0f84d8000000  je 0x1415decb9
1415debe1: 884570  mov byte ptr [rbp + 0x70], al
1415debe4: 488d4538  lea rax, [rbp + 0x38]
1415debe8: 0f104001  movups xmm0, xmmword ptr [rax + 1]
1415debec: 0f104810  movups xmm1, xmmword ptr [rax + 0x10]
1415debf0: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415debf5: 410f114500  movups xmmword ptr [r13], xmm0
1415debfa: 4c89f1  mov rcx, r14
1415debfd: 488d5570  lea rdx, [rbp + 0x70]
1415dec01: e82ac11b00  call 0x14179ad30
1415dec06: f645c001  test byte ptr [rbp - 0x40], 1
1415dec0a: 0f85bc000000  jne 0x1415deccc
1415dec10: 0fb645c1  movzx eax, byte ptr [rbp - 0x3f]
1415dec14: 88859e000000  mov byte ptr [rbp + 0x9e], al
1415dec1a: e931feffff  jmp 0x1415dea50
1415dec1f: 0fb6859e000000  movzx eax, byte ptr [rbp + 0x9e]
1415dec26: 3c38  cmp al, 0x38
1415dec28: 7513  jne 0x1415dec3d
1415dec2a: 488d0df2b96901  lea rcx, [rip + 0x169b9f2]
1415dec31: ba02000000  mov edx, 2
1415dec36: e885af3101  call 0x1428f9bc0
1415dec3b: eb42  jmp 0x1415dec7f
1415dec3d: 48837d3000  cmp qword ptr [rbp + 0x30], 0
1415dec42: 0f85d9000000  jne 0x1415ded21
1415dec48: 4188442401  mov byte ptr [r12 + 1], al
1415dec4d: 31c0  xor eax, eax
1415dec4f: 41880424  mov byte ptr [r12], al
1415dec53: 488d4df0  lea rcx, [rbp - 0x10]
1415dec57: e894912aff  call 0x140887df0
1415dec5c: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415dec63: 488d4d38  lea rcx, [rbp + 0x38]
1415dec67: e8c4f72aff  call 0x14088e430
1415dec6c: eb34  jmp 0x1415deca2
1415dec6e: 488d0df3b56901  lea rcx, [rip + 0x169b5f3]
1415dec75: ba10000000  mov edx, 0x10
1415dec7a: e881b33101  call 0x1428fa000
1415dec7f: 4989442408  mov qword ptr [r12 + 8], rax
1415dec84: 41c6042401  mov byte ptr [r12], 1
1415dec89: 488d4df0  lea rcx, [rbp - 0x10]
1415dec8d: e85e912aff  call 0x140887df0
1415dec92: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415dec99: 488d4d38  lea rcx, [rbp + 0x38]
1415dec9d: e88ef72aff  call 0x14088e430
1415deca2: 4c89e0  mov rax, r12
1415deca5: 4881c428010000  add rsp, 0x128
1415decac: 5b  pop rbx
1415decad: 5f  pop rdi
1415decae: 5e  pop rsi
1415decaf: 415c  pop r12
1415decb1: 415d  pop r13
1415decb3: 415e  pop r14
1415decb5: 415f  pop r15
1415decb7: 5d  pop rbp
1415decb8: c3  ret 
1415decb9: 488d0da8b56901  lea rcx, [rip + 0x169b5a8]
1415decc0: ba10000000  mov edx, 0x10
1415decc5: e836b33101  call 0x1428fa000
1415decca: ebb3  jmp 0x1415dec7f
1415deccc: 488b45c8  mov rax, qword ptr [rbp - 0x38]
1415decd0: ebad  jmp 0x1415dec7f
1415decd2: 0f1002  movups xmm0, xmmword ptr [rdx]
1415decd5: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
1415decd9: 0f294d00  movaps xmmword ptr [rbp], xmm1
1415decdd: 0f2945f0  movaps xmmword ptr [rbp - 0x10], xmm0
1415dece1: 488d4d70  lea rcx, [rbp + 0x70]
1415dece5: 488d55f0  lea rdx, [rbp - 0x10]
1415dece9: e822af2e01  call 0x1428c9c10
1415decee: 488d1593b56901  lea rdx, [rip + 0x169b593]
1415decf5: 4c8d0564b36901  lea r8, [rip + 0x169b364]
1415decfc: 488d4d70  lea rcx, [rbp + 0x70]
1415ded00: e8fbac2e01  call 0x1428c9a00
1415ded05: 4989442408  mov qword ptr [r12 + 8], rax
1415ded0a: 41c6042401  mov byte ptr [r12], 1
1415ded0f: c6859f00000001  mov byte ptr [rbp + 0x9f], 1
1415ded16: 488d4df0  lea rcx, [rbp - 0x10]
1415ded1a: e871ed27ff  call 0x14085da90
1415ded1f: eb81  jmp 0x1415deca2
1415ded21: 488d1570b36901  lea rdx, [rip + 0x169b370]
1415ded28: 4c8d0531b36901  lea r8, [rip + 0x169b331]
1415ded2f: 488b4de8  mov rcx, qword ptr [rbp - 0x18]
1415ded33: e8e8af3101  call 0x1428f9d20
1415ded38: 4989442408  mov qword ptr [r12 + 8], rax
1415ded3d: b001  mov al, 1
1415ded3f: e90bffffff  jmp 0x1415dec4f
1415ded44: 488d0dd8b86901  lea rcx, [rip + 0x169b8d8]
1415ded4b: ba02000000  mov edx, 2
1415ded50: e85bb03101  call 0x1428f9db0
1415ded55: e925ffffff  jmp 0x1415dec7f
1415ded5a: 660f1f440000  nop word ptr [rax + rax]
1415ded60: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ded65: 55  push rbp
1415ded66: 4157  push r15
1415ded68: 4156  push r14
1415ded6a: 4155  push r13
1415ded6c: 4154  push r12
1415ded6e: 56  push rsi
1415ded6f: 57  push rdi
1415ded70: 53  push rbx
1415ded71: 4883ec28  sub rsp, 0x28
1415ded75: 488daa80000000  lea rbp, [rdx + 0x80]
1415ded7c: 488b45d8  mov rax, qword ptr [rbp - 0x28]
1415ded80: 83c0fb  add eax, -5
1415ded83: 3c02  cmp al, 2
1415ded85: 7209  jb 0x1415ded90
1415ded87: f6859f00000001  test byte ptr [rbp + 0x9f], 1
1415ded8e: 7411  je 0x1415deda1
1415ded90: 4883c428  add rsp, 0x28
1415ded94: 5b  pop rbx
1415ded95: 5f  pop rdi
1415ded96: 5e  pop rsi
1415ded97: 415c  pop r12
1415ded99: 415d  pop r13
1415ded9b: 415e  pop r14
1415ded9d: 415f  pop r15
1415ded9f: 5d  pop rbp
1415deda0: c3  ret 
1415deda1: 488b4de0  mov rcx, qword ptr [rbp - 0x20]
1415deda5: e8e6ec27ff  call 0x14085da90
1415dedaa: ebe4  jmp 0x1415ded90
1415dedac: 0f1f4000  nop dword ptr [rax]
1415dedb0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415dedb5: 55  push rbp
1415dedb6: 4157  push r15
1415dedb8: 4156  push r14
1415dedba: 4155  push r13
1415dedbc: 4154  push r12
1415dedbe: 56  push rsi
1415dedbf: 57  push rdi
1415dedc0: 53  push rbx
1415dedc1: 4883ec28  sub rsp, 0x28
1415dedc5: 488daa80000000  lea rbp, [rdx + 0x80]
1415dedcc: 488b5540  mov rdx, qword ptr [rbp + 0x40]
1415dedd0: 4885d2  test rdx, rdx
1415dedd3: 7413  je 0x1415dede8
1415dedd5: 48c1e205  shl rdx, 5
1415dedd9: 41b808000000  mov r8d, 8
1415deddf: 488b4d68  mov rcx, qword ptr [rbp + 0x68]
1415dede3: e8384e03ff  call 0x140613c20
1415dede8: 90  nop 
1415dede9: 4883c428  add rsp, 0x28
1415deded: 5b  pop rbx
1415dedee: 5f  pop rdi
1415dedef: 5e  pop rsi
1415dedf0: 415c  pop r12
1415dedf2: 415d  pop r13
1415dedf4: 415e  pop r14
1415dedf6: 415f  pop r15
1415dedf8: 5d  pop rbp
1415dedf9: c3  ret 
1415dedfa: 660f1f440000  nop word ptr [rax + rax]
1415dee00: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415dee05: 55  push rbp
1415dee06: 4157  push r15
1415dee08: 4156  push r14
1415dee0a: 4155  push r13
1415dee0c: 4154  push r12
1415dee0e: 56  push rsi
1415dee0f: 57  push rdi
1415dee10: 53  push rbx
1415dee11: 4883ec28  sub rsp, 0x28
1415dee15: 488daa80000000  lea rbp, [rdx + 0x80]
1415dee1c: 0f284570  movaps xmm0, xmmword ptr [rbp + 0x70]
1415dee20: 0f288d80000000  movaps xmm1, xmmword ptr [rbp + 0x80]
1415dee27: 488d4538  lea rax, [rbp + 0x38]
1415dee2b: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415dee2f: 0f1100  movups xmmword ptr [rax], xmm0
1415dee32: 4883bd9000000000  cmp qword ptr [rbp + 0x90], 0
1415dee3a: 7416  je 0x1415dee52
1415dee3c: 41b801000000  mov r8d, 1
1415dee42: 488b4d60  mov rcx, qword ptr [rbp + 0x60]
1415dee46: 488b9590000000  mov rdx, qword ptr [rbp + 0x90]
1415dee4d: e8ce4d03ff  call 0x140613c20
1415dee52: 90  nop 
1415dee53: 4883c428  add rsp, 0x28
1415dee57: 5b  pop rbx
1415dee58: 5f  pop rdi
1415dee59: 5e  pop rsi
1415dee5a: 415c  pop r12
1415dee5c: 415d  pop r13
1415dee5e: 415e  pop r14
1415dee60: 415f  pop r15
1415dee62: 5d  pop rbp
1415dee63: c3  ret 
1415dee64: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415dee70: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415dee75: 55  push rbp
1415dee76: 4157  push r15
1415dee78: 4156  push r14
1415dee7a: 4155  push r13
1415dee7c: 4154  push r12
1415dee7e: 56  push rsi
1415dee7f: 57  push rdi
1415dee80: 53  push rbx
1415dee81: 4883ec28  sub rsp, 0x28
1415dee85: 488daa80000000  lea rbp, [rdx + 0x80]
1415dee8c: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415dee93: 488d4df0  lea rcx, [rbp - 0x10]
1415dee97: e864b62aff  call 0x14088a500
1415dee9c: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415deea3: 4883c428  add rsp, 0x28
1415deea7: 5b  pop rbx
1415deea8: 5f  pop rdi
1415deea9: 5e  pop rsi
1415deeaa: 415c  pop r12
1415deeac: 415d  pop r13
1415deeae: 415e  pop r14
1415deeb0: 415f  pop r15
1415deeb2: 5d  pop rbp
1415deeb3: c3  ret 
1415deeb4: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415deec0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415deec5: 55  push rbp
1415deec6: 4157  push r15
1415deec8: 4156  push r14
1415deeca: 4155  push r13
1415deecc: 4154  push r12
1415deece: 56  push rsi
1415deecf: 57  push rdi
1415deed0: 53  push rbx
1415deed1: 4883ec28  sub rsp, 0x28
1415deed5: 488daa80000000  lea rbp, [rdx + 0x80]
1415deedc: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415deee3: 488d4d38  lea rcx, [rbp + 0x38]
1415deee7: e844f52aff  call 0x14088e430
1415deeec: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415deef3: 4883c428  add rsp, 0x28
1415deef7: 5b  pop rbx
1415deef8: 5f  pop rdi
1415deef9: 5e  pop rsi
1415deefa: 415c  pop r12
1415deefc: 415d  pop r13
1415deefe: 415e  pop r14
1415def00: 415f  pop r15
1415def02: 5d  pop rbp
1415def03: c3  ret 
1415def04: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415def10: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415def15: 55  push rbp
1415def16: 4157  push r15
1415def18: 4156  push r14
1415def1a: 4155  push r13
1415def1c: 4154  push r12
1415def1e: 56  push rsi
1415def1f: 57  push rdi
1415def20: 53  push rbx
1415def21: 4883ec28  sub rsp, 0x28
1415def25: 488daa80000000  lea rbp, [rdx + 0x80]
1415def2c: 488d4df0  lea rcx, [rbp - 0x10]
1415def30: e85beb27ff  call 0x14085da90
1415def35: c6859f00000001  mov byte ptr [rbp + 0x9f], 1
1415def3c: 4883c428  add rsp, 0x28
1415def40: 5b  pop rbx
1415def41: 5f  pop rdi
1415def42: 5e  pop rsi
1415def43: 415c  pop r12
1415def45: 415d  pop r13
1415def47: 415e  pop r14
1415def49: 415f  pop r15
1415def4b: 5d  pop rbp
1415def4c: c3  ret 
1415def4d: 0f1f00  nop dword ptr [rax]
1415def50: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415def55: 55  push rbp
1415def56: 4157  push r15
1415def58: 4156  push r14
1415def5a: 4155  push r13
1415def5c: 4154  push r12
1415def5e: 56  push rsi
1415def5f: 57  push rdi
1415def60: 53  push rbx
1415def61: 4883ec28  sub rsp, 0x28
1415def65: 488daa80000000  lea rbp, [rdx + 0x80]
1415def6c: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415def73: 488d4d38  lea rcx, [rbp + 0x38]
1415def77: e8b4f42aff  call 0x14088e430
1415def7c: c6859f00000000  mov byte ptr [rbp + 0x9f], 0
1415def83: 4883c428  add rsp, 0x28
1415def87: 5b  pop rbx
1415def88: 5f  pop rdi
1415def89: 5e  pop rsi
1415def8a: 415c  pop r12
1415def8c: 415d  pop r13
1415def8e: 415e  pop r14
1415def90: 415f  pop r15
1415def92: 5d  pop rbp
1415def93: c3  ret 
