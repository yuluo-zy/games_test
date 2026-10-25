1415b9db0: 55  push rbp
1415b9db1: 4157  push r15
1415b9db3: 4156  push r14
1415b9db5: 4155  push r13
1415b9db7: 4154  push r12
1415b9db9: 56  push rsi
1415b9dba: 57  push rdi
1415b9dbb: 53  push rbx
1415b9dbc: 4881ec48010000  sub rsp, 0x148
1415b9dc3: 488dac2480000000  lea rbp, [rsp + 0x80]
1415b9dcb: 440f2985b0000000  movaps xmmword ptr [rbp + 0xb0], xmm8
1415b9dd3: 0f29bda0000000  movaps xmmword ptr [rbp + 0xa0], xmm7
1415b9dda: 0f29b590000000  movaps xmmword ptr [rbp + 0x90], xmm6
1415b9de1: 48c78588000000feffffff  mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
1415b9dec: 4989cc  mov r12, rcx
1415b9def: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415b9df3: 48894518  mov qword ptr [rbp + 0x18], rax
1415b9df7: 488d5db0  lea rbx, [rbp - 0x50]
1415b9dfb: 4889d9  mov rcx, rbx
1415b9dfe: e8cde25cff  call 0x140b880d0
1415b9e03: 4c8d6d31  lea r13, [rbp + 0x31]
1415b9e07: c7858400000000000000  mov dword ptr [rbp + 0x84], 0
1415b9e11: 0f57f6  xorps xmm6, xmm6
1415b9e14: b001  mov al, 1
1415b9e16: 898580000000  mov dword ptr [rbp + 0x80], eax
1415b9e1c: 4c8d7d58  lea r15, [rbp + 0x58]
1415b9e20: 488d3dbd156c01  lea rdi, [rip + 0x16c15bd]
1415b9e27: 660f1f840000000000  nop word ptr [rax + rax]
1415b9e30: 4c89f9  mov rcx, r15
1415b9e33: 4889da  mov rdx, rbx
1415b9e36: e81526e8ff  call 0x14143c450
1415b9e3b: 488b4558  mov rax, qword ptr [rbp + 0x58]
1415b9e3f: 4885c0  test rax, rax
1415b9e42: 0f847e020000  je 0x1415ba0c6
1415b9e48: 488b4d68  mov rcx, qword ptr [rbp + 0x68]
1415b9e4c: 488d1449  lea rdx, [rcx + rcx*2]
1415b9e50: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415b9e58: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415b9e60: 4c894d70  mov qword ptr [rbp + 0x70], r9
1415b9e64: 4c8bb4d078010000  mov r14, qword ptr [rax + rdx*8 + 0x178]
1415b9e6c: 48c1e105  shl rcx, 5
1415b9e70: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415b9e74: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415b9e79: 0f294530  movaps xmmword ptr [rbp + 0x30], xmm0
1415b9e7d: 0f294d40  movaps xmmword ptr [rbp + 0x40], xmm1
1415b9e81: 4c89c0  mov rax, r8
1415b9e84: 48f7d8  neg rax
1415b9e87: 0f8039020000  jo 0x1415ba0c6
1415b9e8d: 4c894578  mov qword ptr [rbp + 0x78], r8
1415b9e91: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415b9e95: 4883f806  cmp rax, 6
1415b9e99: 0f873c010000  ja 0x1415b9fdb
1415b9e9f: 48630487  movsxd rax, dword ptr [rdi + rax*4]
1415b9ea3: 4801f8  add rax, rdi
1415b9ea6: ffe0  jmp rax
1415b9ea8: 0f284530  movaps xmm0, xmmword ptr [rbp + 0x30]
1415b9eac: 0f284d40  movaps xmm1, xmmword ptr [rbp + 0x40]
1415b9eb0: 488d45f8  lea rax, [rbp - 8]
1415b9eb4: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415b9eb8: 0f1100  movups xmmword ptr [rax], xmm0
1415b9ebb: 4983fe0b  cmp r14, 0xb
1415b9ebf: 0f848b000000  je 0x1415b9f50
1415b9ec5: 40b602  mov sil, 2
1415b9ec8: 4983fe06  cmp r14, 6
1415b9ecc: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415b9ed0: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415b9ed4: 7523  jne 0x1415b9ef9
1415b9ed6: 8b01  mov eax, dword ptr [rcx]
1415b9ed8: 33052b565101  xor eax, dword ptr [rip + 0x151562b]
1415b9ede: 440fb74104  movzx r8d, word ptr [rcx + 4]
1415b9ee3: 6644330522565101  xor r8w, word ptr [rip + 0x1515622]
1415b9eeb: 450fb7c0  movzx r8d, r8w
1415b9eef: 4109c0  or r8d, eax
1415b9ef2: 400f95c6  setne sil
1415b9ef6: 4000f6  add sil, sil
1415b9ef9: 4885d2  test rdx, rdx
1415b9efc: 740b  je 0x1415b9f09
1415b9efe: 41b801000000  mov r8d, 1
1415b9f04: e8179d05ff  call 0x140613c20
1415b9f09: 4080fe02  cmp sil, 2
1415b9f0d: 0f85d6000000  jne 0x1415b9fe9
1415b9f13: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415b9f17: c645f806  mov byte ptr [rbp - 8], 6
1415b9f1b: 3c06  cmp al, 6
1415b9f1d: 0f84f6010000  je 0x1415ba119
1415b9f23: 884530  mov byte ptr [rbp + 0x30], al
1415b9f26: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415b9f2a: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415b9f2e: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415b9f33: 410f114500  movups xmmword ptr [r13], xmm0
1415b9f38: 488d4d30  lea rcx, [rbp + 0x30]
1415b9f3c: e8ff99feff  call 0x1415a3940
1415b9f41: 4885c0  test rax, rax
1415b9f44: 0f84e6feffff  je 0x1415b9e30
1415b9f4a: e9db010000  jmp 0x1415ba12a
1415b9f4f: 90  nop 
1415b9f50: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415b9f54: 488b01  mov rax, qword ptr [rcx]
1415b9f57: 488b5103  mov rdx, qword ptr [rcx + 3]
1415b9f5b: 483305ad555101  xor rax, qword ptr [rip + 0x15155ad]
1415b9f62: 483315a9555101  xor rdx, qword ptr [rip + 0x15155a9]
1415b9f69: 4809c2  or rdx, rax
1415b9f6c: 0f94c0  sete al
1415b9f6f: 40b602  mov sil, 2
1415b9f72: 4028c6  sub sil, al
1415b9f75: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415b9f79: 4885d2  test rdx, rdx
1415b9f7c: 0f857cffffff  jne 0x1415b9efe
1415b9f82: eb85  jmp 0x1415b9f09
1415b9f84: 488b5500  mov rdx, qword ptr [rbp]
1415b9f88: 4885d2  test rdx, rdx
1415b9f8b: 0f8417ffffff  je 0x1415b9ea8
1415b9f91: 488b4d08  mov rcx, qword ptr [rbp + 8]
1415b9f95: 41b801000000  mov r8d, 1
1415b9f9b: e8809c05ff  call 0x140613c20
1415b9fa0: e903ffffff  jmp 0x1415b9ea8
1415b9fa5: 488b4d08  mov rcx, qword ptr [rbp + 8]
1415b9fa9: 488b5510  mov rdx, qword ptr [rbp + 0x10]
1415b9fad: 48894d28  mov qword ptr [rbp + 0x28], rcx
1415b9fb1: e8baed2cff  call 0x140888d70
1415b9fb6: 488b5500  mov rdx, qword ptr [rbp]
1415b9fba: 4885d2  test rdx, rdx
1415b9fbd: 0f84e5feffff  je 0x1415b9ea8
1415b9fc3: 48c1e205  shl rdx, 5
1415b9fc7: 41b808000000  mov r8d, 8
1415b9fcd: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415b9fd1: e84a9c05ff  call 0x140613c20
1415b9fd6: e9cdfeffff  jmp 0x1415b9ea8
1415b9fdb: 488d4d00  lea rcx, [rbp]
1415b9fdf: e85cef2dff  call 0x140898f40
1415b9fe4: e9bffeffff  jmp 0x1415b9ea8
1415b9fe9: 400fb6c6  movzx eax, sil
1415b9fed: 83f801  cmp eax, 1
1415b9ff0: 755c  jne 0x1415ba04e
1415b9ff2: f6858000000001  test byte ptr [rbp + 0x80], 1
1415b9ff9: 0f84da010000  je 0x1415ba1d9
1415b9fff: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415ba003: c645f806  mov byte ptr [rbp - 8], 6
1415ba007: 3c06  cmp al, 6
1415ba009: 0f846d010000  je 0x1415ba17c
1415ba00f: 884530  mov byte ptr [rbp + 0x30], al
1415ba012: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415ba016: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415ba01a: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415ba01f: 410f114500  movups xmmword ptr [r13], xmm0
1415ba024: 4c89f9  mov rcx, r15
1415ba027: 488d5530  lea rdx, [rbp + 0x30]
1415ba02b: e84003feff  call 0x14159a370
1415ba030: f6455801  test byte ptr [rbp + 0x58], 1
1415ba034: 0f853c010000  jne 0x1415ba176
1415ba03a: f30f10755c  movss xmm6, dword ptr [rbp + 0x5c]
1415ba03f: c7858000000000000000  mov dword ptr [rbp + 0x80], 0
1415ba049: e9e2fdffff  jmp 0x1415b9e30
1415ba04e: 83bd8400000000  cmp dword ptr [rbp + 0x84], 0
1415ba055: 0f8568010000  jne 0x1415ba1c3
1415ba05b: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415ba05f: c645f806  mov byte ptr [rbp - 8], 6
1415ba063: 3c06  cmp al, 6
1415ba065: 0f8424010000  je 0x1415ba18f
1415ba06b: 884530  mov byte ptr [rbp + 0x30], al
1415ba06e: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415ba072: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415ba076: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415ba07b: 410f114500  movups xmmword ptr [r13], xmm0
1415ba080: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415ba089: 41b904000000  mov r9d, 4
1415ba08f: 4c89f9  mov rcx, r15
1415ba092: 488d5530  lea rdx, [rbp + 0x30]
1415ba096: 4c8d05abd36b01  lea r8, [rip + 0x16bd3ab]
1415ba09d: e86e99feff  call 0x1415a3a10
1415ba0a2: f6455801  test byte ptr [rbp + 0x58], 1
1415ba0a6: 0f85ca000000  jne 0x1415ba176
1415ba0ac: f30f107d5c  movss xmm7, dword ptr [rbp + 0x5c]
1415ba0b1: f3440f104560  movss xmm8, dword ptr [rbp + 0x60]
1415ba0b7: c7858400000001000000  mov dword ptr [rbp + 0x84], 1
1415ba0c1: e96afdffff  jmp 0x1415b9e30
1415ba0c6: f6858400000001  test byte ptr [rbp + 0x84], 1
1415ba0cd: 7437  je 0x1415ba106
1415ba0cf: f3440f114520  movss dword ptr [rbp + 0x20], xmm8
1415ba0d5: f30f117524  movss dword ptr [rbp + 0x24], xmm6
1415ba0da: 48837df000  cmp qword ptr [rbp - 0x10], 0
1415ba0df: 0f85bd000000  jne 0x1415ba1a2
1415ba0e5: 488b4520  mov rax, qword ptr [rbp + 0x20]
1415ba0e9: f3410f117c2404  movss dword ptr [r12 + 4], xmm7
1415ba0f0: 31c9  xor ecx, ecx
1415ba0f2: 4989442408  mov qword ptr [r12 + 8], rax
1415ba0f7: 41890c24  mov dword ptr [r12], ecx
1415ba0fb: 488d4db0  lea rcx, [rbp - 0x50]
1415ba0ff: e8ecdc2cff  call 0x140887df0
1415ba104: eb3a  jmp 0x1415ba140
1415ba106: 488d0df30e6c01  lea rcx, [rip + 0x16c0ef3]
1415ba10d: ba06000000  mov edx, 6
1415ba112: e8a9fa3301  call 0x1428f9bc0
1415ba117: eb11  jmp 0x1415ba12a
1415ba119: 488d0d48016c01  lea rcx, [rip + 0x16c0148]
1415ba120: ba10000000  mov edx, 0x10
1415ba125: e8d6fe3301  call 0x1428fa000
1415ba12a: 4989442408  mov qword ptr [r12 + 8], rax
1415ba12f: 41c7042401000000  mov dword ptr [r12], 1
1415ba137: 488d4db0  lea rcx, [rbp - 0x50]
1415ba13b: e8b0dc2cff  call 0x140887df0
1415ba140: 488d4df8  lea rcx, [rbp - 8]
1415ba144: e8e7422dff  call 0x14088e430
1415ba149: 4c89e0  mov rax, r12
1415ba14c: 0f28b590000000  movaps xmm6, xmmword ptr [rbp + 0x90]
1415ba153: 0f28bda0000000  movaps xmm7, xmmword ptr [rbp + 0xa0]
1415ba15a: 440f2885b0000000  movaps xmm8, xmmword ptr [rbp + 0xb0]
1415ba162: 4881c448010000  add rsp, 0x148
1415ba169: 5b  pop rbx
1415ba16a: 5f  pop rdi
1415ba16b: 5e  pop rsi
1415ba16c: 415c  pop r12
1415ba16e: 415d  pop r13
1415ba170: 415e  pop r14
1415ba172: 415f  pop r15
1415ba174: 5d  pop rbp
1415ba175: c3  ret 
1415ba176: 488b4560  mov rax, qword ptr [rbp + 0x60]
1415ba17a: ebae  jmp 0x1415ba12a
1415ba17c: 488d0de5006c01  lea rcx, [rip + 0x16c00e5]
1415ba183: ba10000000  mov edx, 0x10
1415ba188: e873fe3301  call 0x1428fa000
1415ba18d: eb9b  jmp 0x1415ba12a
1415ba18f: 488d0dd2006c01  lea rcx, [rip + 0x16c00d2]
1415ba196: ba10000000  mov edx, 0x10
1415ba19b: e860fe3301  call 0x1428fa000
1415ba1a0: eb88  jmp 0x1415ba12a
1415ba1a2: 488d15effe6b01  lea rdx, [rip + 0x16bfeef]
1415ba1a9: 4c8d05b0fe6b01  lea r8, [rip + 0x16bfeb0]
1415ba1b0: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
1415ba1b4: e867fb3301  call 0x1428f9d20
1415ba1b9: b901000000  mov ecx, 1
1415ba1be: e92fffffff  jmp 0x1415ba0f2
1415ba1c3: 488d0d360e6c01  lea rcx, [rip + 0x16c0e36]
1415ba1ca: ba06000000  mov edx, 6
1415ba1cf: e8dcfb3301  call 0x1428f9db0
1415ba1d4: e951ffffff  jmp 0x1415ba12a
1415ba1d9: 488d0d260e6c01  lea rcx, [rip + 0x16c0e26]
1415ba1e0: ba0b000000  mov edx, 0xb
1415ba1e5: e8c6fb3301  call 0x1428f9db0
1415ba1ea: e93bffffff  jmp 0x1415ba12a
1415ba1ef: 90  nop 
1415ba1f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ba1f5: 55  push rbp
1415ba1f6: 4157  push r15
1415ba1f8: 4156  push r14
1415ba1fa: 4155  push r13
1415ba1fc: 4154  push r12
1415ba1fe: 56  push rsi
1415ba1ff: 57  push rdi
1415ba200: 53  push rbx
1415ba201: 4883ec58  sub rsp, 0x58
1415ba205: 488daa80000000  lea rbp, [rdx + 0x80]
1415ba20c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415ba212: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415ba217: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415ba21c: 488b5500  mov rdx, qword ptr [rbp]
1415ba220: 4885d2  test rdx, rdx
1415ba223: 7413  je 0x1415ba238
1415ba225: 48c1e205  shl rdx, 5
1415ba229: 41b808000000  mov r8d, 8
1415ba22f: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415ba233: e8e89905ff  call 0x140613c20
1415ba238: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415ba23d: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415ba242: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415ba248: 4883c458  add rsp, 0x58
1415ba24c: 5b  pop rbx
1415ba24d: 5f  pop rdi
1415ba24e: 5e  pop rsi
1415ba24f: 415c  pop r12
1415ba251: 415d  pop r13
1415ba253: 415e  pop r14
1415ba255: 415f  pop r15
1415ba257: 5d  pop rbp
1415ba258: c3  ret 
1415ba259: 0f1f8000000000  nop dword ptr [rax]
1415ba260: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ba265: 55  push rbp
1415ba266: 4157  push r15
1415ba268: 4156  push r14
1415ba26a: 4155  push r13
1415ba26c: 4154  push r12
1415ba26e: 56  push rsi
1415ba26f: 57  push rdi
1415ba270: 53  push rbx
1415ba271: 4883ec58  sub rsp, 0x58
1415ba275: 488daa80000000  lea rbp, [rdx + 0x80]
1415ba27c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415ba282: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415ba287: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415ba28c: 0f284530  movaps xmm0, xmmword ptr [rbp + 0x30]
1415ba290: 0f284d40  movaps xmm1, xmmword ptr [rbp + 0x40]
1415ba294: 488d45f8  lea rax, [rbp - 8]
1415ba298: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415ba29c: 0f1100  movups xmmword ptr [rax], xmm0
1415ba29f: 48837d7800  cmp qword ptr [rbp + 0x78], 0
1415ba2a4: 7413  je 0x1415ba2b9
1415ba2a6: 41b801000000  mov r8d, 1
1415ba2ac: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415ba2b0: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415ba2b4: e8679905ff  call 0x140613c20
1415ba2b9: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415ba2be: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415ba2c3: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415ba2c9: 4883c458  add rsp, 0x58
1415ba2cd: 5b  pop rbx
1415ba2ce: 5f  pop rdi
1415ba2cf: 5e  pop rsi
1415ba2d0: 415c  pop r12
1415ba2d2: 415d  pop r13
1415ba2d4: 415e  pop r14
1415ba2d6: 415f  pop r15
1415ba2d8: 5d  pop rbp
1415ba2d9: c3  ret 
1415ba2da: 660f1f440000  nop word ptr [rax + rax]
1415ba2e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ba2e5: 55  push rbp
1415ba2e6: 4157  push r15
1415ba2e8: 4156  push r14
1415ba2ea: 4155  push r13
1415ba2ec: 4154  push r12
1415ba2ee: 56  push rsi
1415ba2ef: 57  push rdi
1415ba2f0: 53  push rbx
1415ba2f1: 4883ec58  sub rsp, 0x58
1415ba2f5: 488daa80000000  lea rbp, [rdx + 0x80]
1415ba2fc: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415ba302: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415ba307: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415ba30c: 488d4db0  lea rcx, [rbp - 0x50]
1415ba310: e8eb012dff  call 0x14088a500
1415ba315: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415ba31a: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415ba31f: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415ba325: 4883c458  add rsp, 0x58
1415ba329: 5b  pop rbx
1415ba32a: 5f  pop rdi
1415ba32b: 5e  pop rsi
1415ba32c: 415c  pop r12
1415ba32e: 415d  pop r13
1415ba330: 415e  pop r14
1415ba332: 415f  pop r15
1415ba334: 5d  pop rbp
1415ba335: c3  ret 
1415ba336: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415ba340: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ba345: 55  push rbp
1415ba346: 4157  push r15
1415ba348: 4156  push r14
1415ba34a: 4155  push r13
1415ba34c: 4154  push r12
1415ba34e: 56  push rsi
1415ba34f: 57  push rdi
1415ba350: 53  push rbx
1415ba351: 4883ec58  sub rsp, 0x58
1415ba355: 488daa80000000  lea rbp, [rdx + 0x80]
1415ba35c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415ba362: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415ba367: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415ba36c: 488d4df8  lea rcx, [rbp - 8]
1415ba370: e8bb402dff  call 0x14088e430
1415ba375: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415ba37a: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415ba37f: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415ba385: 4883c458  add rsp, 0x58
1415ba389: 5b  pop rbx
1415ba38a: 5f  pop rdi
1415ba38b: 5e  pop rsi
1415ba38c: 415c  pop r12
1415ba38e: 415d  pop r13
1415ba390: 415e  pop r14
1415ba392: 415f  pop r15
1415ba394: 5d  pop rbp
1415ba395: c3  ret 
1415ba396: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415ba3a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415ba3a5: 55  push rbp
1415ba3a6: 4157  push r15
1415ba3a8: 4156  push r14
1415ba3aa: 4155  push r13
1415ba3ac: 4154  push r12
1415ba3ae: 56  push rsi
1415ba3af: 57  push rdi
1415ba3b0: 53  push rbx
1415ba3b1: 4883ec58  sub rsp, 0x58
1415ba3b5: 488daa80000000  lea rbp, [rdx + 0x80]
1415ba3bc: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415ba3c2: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415ba3c7: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415ba3cc: 488d4df8  lea rcx, [rbp - 8]
1415ba3d0: e85b402dff  call 0x14088e430
1415ba3d5: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415ba3da: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415ba3df: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415ba3e5: 4883c458  add rsp, 0x58
1415ba3e9: 5b  pop rbx
1415ba3ea: 5f  pop rdi
1415ba3eb: 5e  pop rsi
1415ba3ec: 415c  pop r12
1415ba3ee: 415d  pop r13
1415ba3f0: 415e  pop r14
1415ba3f2: 415f  pop r15
1415ba3f4: 5d  pop rbp
1415ba3f5: c3  ret 
