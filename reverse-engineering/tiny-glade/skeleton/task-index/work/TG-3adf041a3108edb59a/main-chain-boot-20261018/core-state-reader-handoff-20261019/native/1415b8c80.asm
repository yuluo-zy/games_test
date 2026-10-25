1415b8c80: 55  push rbp
1415b8c81: 4157  push r15
1415b8c83: 4156  push r14
1415b8c85: 4155  push r13
1415b8c87: 4154  push r12
1415b8c89: 56  push rsi
1415b8c8a: 57  push rdi
1415b8c8b: 53  push rbx
1415b8c8c: 4881ecf8000000  sub rsp, 0xf8
1415b8c93: 488dac2480000000  lea rbp, [rsp + 0x80]
1415b8c9b: 48c74570feffffff  mov qword ptr [rbp + 0x70], 0xfffffffffffffffe
1415b8ca3: 4989cc  mov r12, rcx
1415b8ca6: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415b8caa: 48894528  mov qword ptr [rbp + 0x28], rax
1415b8cae: 488d5da8  lea rbx, [rbp - 0x58]
1415b8cb2: 4889d9  mov rcx, rbx
1415b8cb5: e816f45cff  call 0x140b880d0
1415b8cba: 4c8d6d31  lea r13, [rbp + 0x31]
1415b8cbe: c7456800000000  mov dword ptr [rbp + 0x68], 0
1415b8cc5: 4c8d7d10  lea r15, [rbp + 0x10]
1415b8cc9: 488d3dc0266c01  lea rdi, [rip + 0x16c26c0]
1415b8cd0: 4c89f9  mov rcx, r15
1415b8cd3: 4889da  mov rdx, rbx
1415b8cd6: e87537e8ff  call 0x14143c450
1415b8cdb: 488b4510  mov rax, qword ptr [rbp + 0x10]
1415b8cdf: 4885c0  test rax, rax
1415b8ce2: 0f849b010000  je 0x1415b8e83
1415b8ce8: 488b4d20  mov rcx, qword ptr [rbp + 0x20]
1415b8cec: 488d1449  lea rdx, [rcx + rcx*2]
1415b8cf0: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415b8cf8: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415b8d00: 4c894d50  mov qword ptr [rbp + 0x50], r9
1415b8d04: 4c8bb4d078010000  mov r14, qword ptr [rax + rdx*8 + 0x178]
1415b8d0c: 48c1e105  shl rcx, 5
1415b8d10: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415b8d14: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415b8d19: 0f294530  movaps xmmword ptr [rbp + 0x30], xmm0
1415b8d1d: 0f294d40  movaps xmmword ptr [rbp + 0x40], xmm1
1415b8d21: 4c89c0  mov rax, r8
1415b8d24: 48f7d8  neg rax
1415b8d27: 0f8056010000  jo 0x1415b8e83
1415b8d2d: 4c894560  mov qword ptr [rbp + 0x60], r8
1415b8d31: 0fb645f0  movzx eax, byte ptr [rbp - 0x10]
1415b8d35: 4883f806  cmp rax, 6
1415b8d39: 0f87e0000000  ja 0x1415b8e1f
1415b8d3f: 48630487  movsxd rax, dword ptr [rdi + rax*4]
1415b8d43: 4801f8  add rax, rdi
1415b8d46: ffe0  jmp rax
1415b8d48: 0f284530  movaps xmm0, xmmword ptr [rbp + 0x30]
1415b8d4c: 0f284d40  movaps xmm1, xmmword ptr [rbp + 0x40]
1415b8d50: 488d45f0  lea rax, [rbp - 0x10]
1415b8d54: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b8d58: 0f1100  movups xmmword ptr [rax], xmm0
1415b8d5b: 40b601  mov sil, 1
1415b8d5e: 4983fe01  cmp r14, 1
1415b8d62: 488b4d50  mov rcx, qword ptr [rbp + 0x50]
1415b8d66: 7507  jne 0x1415b8d6f
1415b8d68: 803978  cmp byte ptr [rcx], 0x78
1415b8d6b: 400f95c6  setne sil
1415b8d6f: 488b5560  mov rdx, qword ptr [rbp + 0x60]
1415b8d73: 4885d2  test rdx, rdx
1415b8d76: 740b  je 0x1415b8d83
1415b8d78: 41b801000000  mov r8d, 1
1415b8d7e: e89dae05ff  call 0x140613c20
1415b8d83: 4084f6  test sil, sil
1415b8d86: 0f84a1000000  je 0x1415b8e2d
1415b8d8c: 0fb645f0  movzx eax, byte ptr [rbp - 0x10]
1415b8d90: c645f006  mov byte ptr [rbp - 0x10], 6
1415b8d94: 3c06  cmp al, 6
1415b8d96: 0f8425010000  je 0x1415b8ec1
1415b8d9c: 884530  mov byte ptr [rbp + 0x30], al
1415b8d9f: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415b8da3: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415b8da7: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415b8dac: 410f114500  movups xmmword ptr [r13], xmm0
1415b8db1: 488d4d30  lea rcx, [rbp + 0x30]
1415b8db5: e886abfeff  call 0x1415a3940
1415b8dba: 4885c0  test rax, rax
1415b8dbd: 0f840dffffff  je 0x1415b8cd0
1415b8dc3: e90a010000  jmp 0x1415b8ed2
1415b8dc8: 488b55f8  mov rdx, qword ptr [rbp - 8]
1415b8dcc: 4885d2  test rdx, rdx
1415b8dcf: 0f8473ffffff  je 0x1415b8d48
1415b8dd5: 488b4d00  mov rcx, qword ptr [rbp]
1415b8dd9: 41b801000000  mov r8d, 1
1415b8ddf: e83cae05ff  call 0x140613c20
1415b8de4: e95fffffff  jmp 0x1415b8d48
1415b8de9: 488b4d00  mov rcx, qword ptr [rbp]
1415b8ded: 488b5508  mov rdx, qword ptr [rbp + 8]
1415b8df1: 48894d58  mov qword ptr [rbp + 0x58], rcx
1415b8df5: e876ff2cff  call 0x140888d70
1415b8dfa: 488b55f8  mov rdx, qword ptr [rbp - 8]
1415b8dfe: 4885d2  test rdx, rdx
1415b8e01: 0f8441ffffff  je 0x1415b8d48
1415b8e07: 48c1e205  shl rdx, 5
1415b8e0b: 41b808000000  mov r8d, 8
1415b8e11: 488b4d58  mov rcx, qword ptr [rbp + 0x58]
1415b8e15: e806ae05ff  call 0x140613c20
1415b8e1a: e929ffffff  jmp 0x1415b8d48
1415b8e1f: 488d4df8  lea rcx, [rbp - 8]
1415b8e23: e818012eff  call 0x140898f40
1415b8e28: e91bffffff  jmp 0x1415b8d48
1415b8e2d: f6456801  test byte ptr [rbp + 0x68], 1
1415b8e31: 0f850a010000  jne 0x1415b8f41
1415b8e37: 0fb645f0  movzx eax, byte ptr [rbp - 0x10]
1415b8e3b: c645f006  mov byte ptr [rbp - 0x10], 6
1415b8e3f: 3c06  cmp al, 6
1415b8e41: 0f84be000000  je 0x1415b8f05
1415b8e47: 884530  mov byte ptr [rbp + 0x30], al
1415b8e4a: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415b8e4e: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415b8e52: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415b8e57: 410f114500  movups xmmword ptr [r13], xmm0
1415b8e5c: 4c89f9  mov rcx, r15
1415b8e5f: 488d5530  lea rdx, [rbp + 0x30]
1415b8e63: e81813feff  call 0x14159a180
1415b8e68: f6451001  test byte ptr [rbp + 0x10], 1
1415b8e6c: 0f85a6000000  jne 0x1415b8f18
1415b8e72: 0fb64511  movzx eax, byte ptr [rbp + 0x11]
1415b8e76: 88456f  mov byte ptr [rbp + 0x6f], al
1415b8e79: b001  mov al, 1
1415b8e7b: 894568  mov dword ptr [rbp + 0x68], eax
1415b8e7e: e94dfeffff  jmp 0x1415b8cd0
1415b8e83: f6456801  test byte ptr [rbp + 0x68], 1
1415b8e87: 7425  je 0x1415b8eae
1415b8e89: 48837de800  cmp qword ptr [rbp - 0x18], 0
1415b8e8e: 0f858a000000  jne 0x1415b8f1e
1415b8e94: 0fb6456f  movzx eax, byte ptr [rbp + 0x6f]
1415b8e98: 4188442401  mov byte ptr [r12 + 1], al
1415b8e9d: 31c0  xor eax, eax
1415b8e9f: 41880424  mov byte ptr [r12], al
1415b8ea3: 488d4da8  lea rcx, [rbp - 0x58]
1415b8ea7: e844ef2cff  call 0x140887df0
1415b8eac: eb37  jmp 0x1415b8ee5
1415b8eae: 488d0dd3126c01  lea rcx, [rip + 0x16c12d3]
1415b8eb5: ba01000000  mov edx, 1
1415b8eba: e8010d3401  call 0x1428f9bc0
1415b8ebf: eb11  jmp 0x1415b8ed2
1415b8ec1: 488d0da0136c01  lea rcx, [rip + 0x16c13a0]
1415b8ec8: ba10000000  mov edx, 0x10
1415b8ecd: e82e113401  call 0x1428fa000
1415b8ed2: 4989442408  mov qword ptr [r12 + 8], rax
1415b8ed7: 41c6042401  mov byte ptr [r12], 1
1415b8edc: 488d4da8  lea rcx, [rbp - 0x58]
1415b8ee0: e80bef2cff  call 0x140887df0
1415b8ee5: 488d4df0  lea rcx, [rbp - 0x10]
1415b8ee9: e842552dff  call 0x14088e430
1415b8eee: 4c89e0  mov rax, r12
1415b8ef1: 4881c4f8000000  add rsp, 0xf8
1415b8ef8: 5b  pop rbx
1415b8ef9: 5f  pop rdi
1415b8efa: 5e  pop rsi
1415b8efb: 415c  pop r12
1415b8efd: 415d  pop r13
1415b8eff: 415e  pop r14
1415b8f01: 415f  pop r15
1415b8f03: 5d  pop rbp
1415b8f04: c3  ret 
1415b8f05: 488d0d5c136c01  lea rcx, [rip + 0x16c135c]
1415b8f0c: ba10000000  mov edx, 0x10
1415b8f11: e8ea103401  call 0x1428fa000
1415b8f16: ebba  jmp 0x1415b8ed2
1415b8f18: 488b4518  mov rax, qword ptr [rbp + 0x18]
1415b8f1c: ebb4  jmp 0x1415b8ed2
1415b8f1e: 488d1573116c01  lea rdx, [rip + 0x16c1173]
1415b8f25: 4c8d0534116c01  lea r8, [rip + 0x16c1134]
1415b8f2c: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415b8f30: e8eb0d3401  call 0x1428f9d20
1415b8f35: 4989442408  mov qword ptr [r12 + 8], rax
1415b8f3a: b001  mov al, 1
1415b8f3c: e95effffff  jmp 0x1415b8e9f
1415b8f41: 488d0d40126c01  lea rcx, [rip + 0x16c1240]
1415b8f48: ba01000000  mov edx, 1
1415b8f4d: e85e0e3401  call 0x1428f9db0
1415b8f52: e97bffffff  jmp 0x1415b8ed2
1415b8f57: 660f1f840000000000  nop word ptr [rax + rax]
1415b8f60: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b8f65: 55  push rbp
1415b8f66: 4157  push r15
1415b8f68: 4156  push r14
1415b8f6a: 4155  push r13
1415b8f6c: 4154  push r12
1415b8f6e: 56  push rsi
1415b8f6f: 57  push rdi
1415b8f70: 53  push rbx
1415b8f71: 4883ec28  sub rsp, 0x28
1415b8f75: 488daa80000000  lea rbp, [rdx + 0x80]
1415b8f7c: 488b55f8  mov rdx, qword ptr [rbp - 8]
1415b8f80: 4885d2  test rdx, rdx
1415b8f83: 7413  je 0x1415b8f98
1415b8f85: 48c1e205  shl rdx, 5
1415b8f89: 41b808000000  mov r8d, 8
1415b8f8f: 488b4d58  mov rcx, qword ptr [rbp + 0x58]
1415b8f93: e888ac05ff  call 0x140613c20
1415b8f98: 90  nop 
1415b8f99: 4883c428  add rsp, 0x28
1415b8f9d: 5b  pop rbx
1415b8f9e: 5f  pop rdi
1415b8f9f: 5e  pop rsi
1415b8fa0: 415c  pop r12
1415b8fa2: 415d  pop r13
1415b8fa4: 415e  pop r14
1415b8fa6: 415f  pop r15
1415b8fa8: 5d  pop rbp
1415b8fa9: c3  ret 
1415b8faa: 660f1f440000  nop word ptr [rax + rax]
1415b8fb0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b8fb5: 55  push rbp
1415b8fb6: 4157  push r15
1415b8fb8: 4156  push r14
1415b8fba: 4155  push r13
1415b8fbc: 4154  push r12
1415b8fbe: 56  push rsi
1415b8fbf: 57  push rdi
1415b8fc0: 53  push rbx
1415b8fc1: 4883ec28  sub rsp, 0x28
1415b8fc5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b8fcc: 0f284530  movaps xmm0, xmmword ptr [rbp + 0x30]
1415b8fd0: 0f284d40  movaps xmm1, xmmword ptr [rbp + 0x40]
1415b8fd4: 488d45f0  lea rax, [rbp - 0x10]
1415b8fd8: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b8fdc: 0f1100  movups xmmword ptr [rax], xmm0
1415b8fdf: 48837d6000  cmp qword ptr [rbp + 0x60], 0
1415b8fe4: 7413  je 0x1415b8ff9
1415b8fe6: 41b801000000  mov r8d, 1
1415b8fec: 488b4d50  mov rcx, qword ptr [rbp + 0x50]
1415b8ff0: 488b5560  mov rdx, qword ptr [rbp + 0x60]
1415b8ff4: e827ac05ff  call 0x140613c20
1415b8ff9: 90  nop 
1415b8ffa: 4883c428  add rsp, 0x28
1415b8ffe: 5b  pop rbx
1415b8fff: 5f  pop rdi
1415b9000: 5e  pop rsi
1415b9001: 415c  pop r12
1415b9003: 415d  pop r13
1415b9005: 415e  pop r14
1415b9007: 415f  pop r15
1415b9009: 5d  pop rbp
1415b900a: c3  ret 
1415b900b: 0f1f440000  nop dword ptr [rax + rax]
1415b9010: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b9015: 55  push rbp
1415b9016: 4157  push r15
1415b9018: 4156  push r14
1415b901a: 4155  push r13
1415b901c: 4154  push r12
1415b901e: 56  push rsi
1415b901f: 57  push rdi
1415b9020: 53  push rbx
1415b9021: 4883ec28  sub rsp, 0x28
1415b9025: 488daa80000000  lea rbp, [rdx + 0x80]
1415b902c: 488d4da8  lea rcx, [rbp - 0x58]
1415b9030: e8cb142dff  call 0x14088a500
1415b9035: 90  nop 
1415b9036: 4883c428  add rsp, 0x28
1415b903a: 5b  pop rbx
1415b903b: 5f  pop rdi
1415b903c: 5e  pop rsi
1415b903d: 415c  pop r12
1415b903f: 415d  pop r13
1415b9041: 415e  pop r14
1415b9043: 415f  pop r15
1415b9045: 5d  pop rbp
1415b9046: c3  ret 
1415b9047: 660f1f840000000000  nop word ptr [rax + rax]
1415b9050: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b9055: 55  push rbp
1415b9056: 4157  push r15
1415b9058: 4156  push r14
1415b905a: 4155  push r13
1415b905c: 4154  push r12
1415b905e: 56  push rsi
1415b905f: 57  push rdi
1415b9060: 53  push rbx
1415b9061: 4883ec28  sub rsp, 0x28
1415b9065: 488daa80000000  lea rbp, [rdx + 0x80]
1415b906c: 488d4df0  lea rcx, [rbp - 0x10]
1415b9070: e8bb532dff  call 0x14088e430
1415b9075: 90  nop 
1415b9076: 4883c428  add rsp, 0x28
1415b907a: 5b  pop rbx
1415b907b: 5f  pop rdi
1415b907c: 5e  pop rsi
1415b907d: 415c  pop r12
1415b907f: 415d  pop r13
1415b9081: 415e  pop r14
1415b9083: 415f  pop r15
1415b9085: 5d  pop rbp
1415b9086: c3  ret 
1415b9087: 660f1f840000000000  nop word ptr [rax + rax]
1415b9090: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415b9095: 55  push rbp
1415b9096: 4157  push r15
1415b9098: 4156  push r14
1415b909a: 4155  push r13
1415b909c: 4154  push r12
1415b909e: 56  push rsi
1415b909f: 57  push rdi
1415b90a0: 53  push rbx
1415b90a1: 4883ec28  sub rsp, 0x28
1415b90a5: 488daa80000000  lea rbp, [rdx + 0x80]
1415b90ac: 488d4df0  lea rcx, [rbp - 0x10]
1415b90b0: e87b532dff  call 0x14088e430
1415b90b5: 90  nop 
1415b90b6: 4883c428  add rsp, 0x28
1415b90ba: 5b  pop rbx
1415b90bb: 5f  pop rdi
1415b90bc: 5e  pop rsi
1415b90bd: 415c  pop r12
1415b90bf: 415d  pop r13
1415b90c1: 415e  pop r14
1415b90c3: 415f  pop r15
1415b90c5: 5d  pop rbp
1415b90c6: c3  ret 
