1415bf7b0: 55  push rbp
1415bf7b1: 4157  push r15
1415bf7b3: 4156  push r14
1415bf7b5: 4155  push r13
1415bf7b7: 4154  push r12
1415bf7b9: 56  push rsi
1415bf7ba: 57  push rdi
1415bf7bb: 53  push rbx
1415bf7bc: 4881ec48010000  sub rsp, 0x148
1415bf7c3: 488dac2480000000  lea rbp, [rsp + 0x80]
1415bf7cb: 440f2985b0000000  movaps xmmword ptr [rbp + 0xb0], xmm8
1415bf7d3: 0f29bda0000000  movaps xmmword ptr [rbp + 0xa0], xmm7
1415bf7da: 0f29b590000000  movaps xmmword ptr [rbp + 0x90], xmm6
1415bf7e1: 48c78588000000feffffff  mov qword ptr [rbp + 0x88], 0xfffffffffffffffe
1415bf7ec: 4989cc  mov r12, rcx
1415bf7ef: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415bf7f3: 48894518  mov qword ptr [rbp + 0x18], rax
1415bf7f7: 488d5db0  lea rbx, [rbp - 0x50]
1415bf7fb: 4889d9  mov rcx, rbx
1415bf7fe: e8cd885cff  call 0x140b880d0
1415bf803: 4c8d6d31  lea r13, [rbp + 0x31]
1415bf807: c7858400000000000000  mov dword ptr [rbp + 0x84], 0
1415bf811: 0f57f6  xorps xmm6, xmm6
1415bf814: b001  mov al, 1
1415bf816: 898580000000  mov dword ptr [rbp + 0x80], eax
1415bf81c: 4c8d7d58  lea r15, [rbp + 0x58]
1415bf820: 488d3d09bd6b01  lea rdi, [rip + 0x16bbd09]
1415bf827: 660f1f840000000000  nop word ptr [rax + rax]
1415bf830: 4c89f9  mov rcx, r15
1415bf833: 4889da  mov rdx, rbx
1415bf836: e815cce7ff  call 0x14143c450
1415bf83b: 488b4558  mov rax, qword ptr [rbp + 0x58]
1415bf83f: 4885c0  test rax, rax
1415bf842: 0f8497020000  je 0x1415bfadf
1415bf848: 488b4d68  mov rcx, qword ptr [rbp + 0x68]
1415bf84c: 488d1449  lea rdx, [rcx + rcx*2]
1415bf850: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415bf858: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415bf860: 4c894d70  mov qword ptr [rbp + 0x70], r9
1415bf864: 4c8bb4d078010000  mov r14, qword ptr [rax + rdx*8 + 0x178]
1415bf86c: 48c1e105  shl rcx, 5
1415bf870: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415bf874: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415bf879: 0f294530  movaps xmmword ptr [rbp + 0x30], xmm0
1415bf87d: 0f294d40  movaps xmmword ptr [rbp + 0x40], xmm1
1415bf881: 4c89c0  mov rax, r8
1415bf884: 48f7d8  neg rax
1415bf887: 0f8052020000  jo 0x1415bfadf
1415bf88d: 4c894578  mov qword ptr [rbp + 0x78], r8
1415bf891: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415bf895: 4883f806  cmp rax, 6
1415bf899: 0f8755010000  ja 0x1415bf9f4
1415bf89f: 48630487  movsxd rax, dword ptr [rdi + rax*4]
1415bf8a3: 4801f8  add rax, rdi
1415bf8a6: ffe0  jmp rax
1415bf8a8: 660f6f4530  movdqa xmm0, xmmword ptr [rbp + 0x30]
1415bf8ad: 660f6f4d40  movdqa xmm1, xmmword ptr [rbp + 0x40]
1415bf8b2: 488d45f8  lea rax, [rbp - 8]
1415bf8b6: f30f7f4810  movdqu xmmword ptr [rax + 0x10], xmm1
1415bf8bb: f30f7f00  movdqu xmmword ptr [rax], xmm0
1415bf8bf: 4983fe14  cmp r14, 0x14
1415bf8c3: 0f8487000000  je 0x1415bf950
1415bf8c9: 40b602  mov sil, 2
1415bf8cc: 4983fe07  cmp r14, 7
1415bf8d0: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415bf8d4: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415bf8d8: 751d  jne 0x1415bf8f7
1415bf8da: 8b01  mov eax, dword ptr [rcx]
1415bf8dc: 448b4103  mov r8d, dword ptr [rcx + 3]
1415bf8e0: 3305d0fb5001  xor eax, dword ptr [rip + 0x150fbd0]
1415bf8e6: 443305ccfb5001  xor r8d, dword ptr [rip + 0x150fbcc]
1415bf8ed: 4109c0  or r8d, eax
1415bf8f0: 400f95c6  setne sil
1415bf8f4: 4000f6  add sil, sil
1415bf8f7: 4885d2  test rdx, rdx
1415bf8fa: 740b  je 0x1415bf907
1415bf8fc: 41b801000000  mov r8d, 1
1415bf902: e8194305ff  call 0x140613c20
1415bf907: 4080fe02  cmp sil, 2
1415bf90b: 0f85f1000000  jne 0x1415bfa02
1415bf911: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415bf915: c645f806  mov byte ptr [rbp - 8], 6
1415bf919: 3c06  cmp al, 6
1415bf91b: 0f8411020000  je 0x1415bfb32
1415bf921: 884530  mov byte ptr [rbp + 0x30], al
1415bf924: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415bf928: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415bf92c: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415bf931: 410f114500  movups xmmword ptr [r13], xmm0
1415bf936: 488d4d30  lea rcx, [rbp + 0x30]
1415bf93a: e80140feff  call 0x1415a3940
1415bf93f: 4885c0  test rax, rax
1415bf942: 0f84e8feffff  je 0x1415bf830
1415bf948: e9f6010000  jmp 0x1415bfb43
1415bf94d: 0f1f00  nop dword ptr [rax]
1415bf950: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415bf954: f30f6f01  movdqu xmm0, xmmword ptr [rcx]
1415bf958: f30f6f0d5dfb5001  movdqu xmm1, xmmword ptr [rip + 0x150fb5d]
1415bf960: 660f74c8  pcmpeqb xmm1, xmm0
1415bf964: 660f6e4110  movd xmm0, dword ptr [rcx + 0x10]
1415bf969: 660f6e155cfb5001  movd xmm2, dword ptr [rip + 0x150fb5c]
1415bf971: 660f74d0  pcmpeqb xmm2, xmm0
1415bf975: 660fdbd1  pand xmm2, xmm1
1415bf979: 660fd7c2  pmovmskb eax, xmm2
1415bf97d: 3dffff0000  cmp eax, 0xffff
1415bf982: 0f94c0  sete al
1415bf985: 40b602  mov sil, 2
1415bf988: 4028c6  sub sil, al
1415bf98b: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415bf98f: 4885d2  test rdx, rdx
1415bf992: 0f8564ffffff  jne 0x1415bf8fc
1415bf998: e96affffff  jmp 0x1415bf907
1415bf99d: 488b5500  mov rdx, qword ptr [rbp]
1415bf9a1: 4885d2  test rdx, rdx
1415bf9a4: 0f84fefeffff  je 0x1415bf8a8
1415bf9aa: 488b4d08  mov rcx, qword ptr [rbp + 8]
1415bf9ae: 41b801000000  mov r8d, 1
1415bf9b4: e8674205ff  call 0x140613c20
1415bf9b9: e9eafeffff  jmp 0x1415bf8a8
1415bf9be: 488b4d08  mov rcx, qword ptr [rbp + 8]
1415bf9c2: 488b5510  mov rdx, qword ptr [rbp + 0x10]
1415bf9c6: 48894d28  mov qword ptr [rbp + 0x28], rcx
1415bf9ca: e8a1932cff  call 0x140888d70
1415bf9cf: 488b5500  mov rdx, qword ptr [rbp]
1415bf9d3: 4885d2  test rdx, rdx
1415bf9d6: 0f84ccfeffff  je 0x1415bf8a8
1415bf9dc: 48c1e205  shl rdx, 5
1415bf9e0: 41b808000000  mov r8d, 8
1415bf9e6: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415bf9ea: e8314205ff  call 0x140613c20
1415bf9ef: e9b4feffff  jmp 0x1415bf8a8
1415bf9f4: 488d4d00  lea rcx, [rbp]
1415bf9f8: e843952dff  call 0x140898f40
1415bf9fd: e9a6feffff  jmp 0x1415bf8a8
1415bfa02: 400fb6c6  movzx eax, sil
1415bfa06: 83f801  cmp eax, 1
1415bfa09: 755c  jne 0x1415bfa67
1415bfa0b: f6858000000001  test byte ptr [rbp + 0x80], 1
1415bfa12: 0f84da010000  je 0x1415bfbf2
1415bfa18: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415bfa1c: c645f806  mov byte ptr [rbp - 8], 6
1415bfa20: 3c06  cmp al, 6
1415bfa22: 0f846d010000  je 0x1415bfb95
1415bfa28: 884530  mov byte ptr [rbp + 0x30], al
1415bfa2b: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415bfa2f: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415bfa33: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415bfa38: 410f114500  movups xmmword ptr [r13], xmm0
1415bfa3d: 4c89f9  mov rcx, r15
1415bfa40: 488d5530  lea rdx, [rbp + 0x30]
1415bfa44: e827a9fdff  call 0x14159a370
1415bfa49: f6455801  test byte ptr [rbp + 0x58], 1
1415bfa4d: 0f853c010000  jne 0x1415bfb8f
1415bfa53: f30f10755c  movss xmm6, dword ptr [rbp + 0x5c]
1415bfa58: c7858000000000000000  mov dword ptr [rbp + 0x80], 0
1415bfa62: e9c9fdffff  jmp 0x1415bf830
1415bfa67: 83bd8400000000  cmp dword ptr [rbp + 0x84], 0
1415bfa6e: 0f8568010000  jne 0x1415bfbdc
1415bfa74: 0fb645f8  movzx eax, byte ptr [rbp - 8]
1415bfa78: c645f806  mov byte ptr [rbp - 8], 6
1415bfa7c: 3c06  cmp al, 6
1415bfa7e: 0f8424010000  je 0x1415bfba8
1415bfa84: 884530  mov byte ptr [rbp + 0x30], al
1415bfa87: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415bfa8b: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415bfa8f: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415bfa94: 410f114500  movups xmmword ptr [r13], xmm0
1415bfa99: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415bfaa2: 41b904000000  mov r9d, 4
1415bfaa8: 4c89f9  mov rcx, r15
1415bfaab: 488d5530  lea rdx, [rbp + 0x30]
1415bfaaf: 4c8d0592796b01  lea r8, [rip + 0x16b7992]
1415bfab6: e8553ffeff  call 0x1415a3a10
1415bfabb: f6455801  test byte ptr [rbp + 0x58], 1
1415bfabf: 0f85ca000000  jne 0x1415bfb8f
1415bfac5: f30f107d5c  movss xmm7, dword ptr [rbp + 0x5c]
1415bfaca: f3440f104560  movss xmm8, dword ptr [rbp + 0x60]
1415bfad0: c7858400000001000000  mov dword ptr [rbp + 0x84], 1
1415bfada: e951fdffff  jmp 0x1415bf830
1415bfadf: f6858400000001  test byte ptr [rbp + 0x84], 1
1415bfae6: 7437  je 0x1415bfb1f
1415bfae8: f3440f114520  movss dword ptr [rbp + 0x20], xmm8
1415bfaee: f30f117524  movss dword ptr [rbp + 0x24], xmm6
1415bfaf3: 48837df000  cmp qword ptr [rbp - 0x10], 0
1415bfaf8: 0f85bd000000  jne 0x1415bfbbb
1415bfafe: 488b4520  mov rax, qword ptr [rbp + 0x20]
1415bfb02: f3410f117c2404  movss dword ptr [r12 + 4], xmm7
1415bfb09: 31c9  xor ecx, ecx
1415bfb0b: 4989442408  mov qword ptr [r12 + 8], rax
1415bfb10: 41890c24  mov dword ptr [r12], ecx
1415bfb14: 488d4db0  lea rcx, [rbp - 0x50]
1415bfb18: e8d3822cff  call 0x140887df0
1415bfb1d: eb3a  jmp 0x1415bfb59
1415bfb1f: 488d0d82b46b01  lea rcx, [rip + 0x16bb482]
1415bfb26: ba07000000  mov edx, 7
1415bfb2b: e890a03301  call 0x1428f9bc0
1415bfb30: eb11  jmp 0x1415bfb43
1415bfb32: 488d0d2fa76b01  lea rcx, [rip + 0x16ba72f]
1415bfb39: ba10000000  mov edx, 0x10
1415bfb3e: e8bda43301  call 0x1428fa000
1415bfb43: 4989442408  mov qword ptr [r12 + 8], rax
1415bfb48: 41c7042401000000  mov dword ptr [r12], 1
1415bfb50: 488d4db0  lea rcx, [rbp - 0x50]
1415bfb54: e897822cff  call 0x140887df0
1415bfb59: 488d4df8  lea rcx, [rbp - 8]
1415bfb5d: e8cee82cff  call 0x14088e430
1415bfb62: 4c89e0  mov rax, r12
1415bfb65: 0f28b590000000  movaps xmm6, xmmword ptr [rbp + 0x90]
1415bfb6c: 0f28bda0000000  movaps xmm7, xmmword ptr [rbp + 0xa0]
1415bfb73: 440f2885b0000000  movaps xmm8, xmmword ptr [rbp + 0xb0]
1415bfb7b: 4881c448010000  add rsp, 0x148
1415bfb82: 5b  pop rbx
1415bfb83: 5f  pop rdi
1415bfb84: 5e  pop rsi
1415bfb85: 415c  pop r12
1415bfb87: 415d  pop r13
1415bfb89: 415e  pop r14
1415bfb8b: 415f  pop r15
1415bfb8d: 5d  pop rbp
1415bfb8e: c3  ret 
1415bfb8f: 488b4560  mov rax, qword ptr [rbp + 0x60]
1415bfb93: ebae  jmp 0x1415bfb43
1415bfb95: 488d0dcca66b01  lea rcx, [rip + 0x16ba6cc]
1415bfb9c: ba10000000  mov edx, 0x10
1415bfba1: e85aa43301  call 0x1428fa000
1415bfba6: eb9b  jmp 0x1415bfb43
1415bfba8: 488d0db9a66b01  lea rcx, [rip + 0x16ba6b9]
1415bfbaf: ba10000000  mov edx, 0x10
1415bfbb4: e847a43301  call 0x1428fa000
1415bfbb9: eb88  jmp 0x1415bfb43
1415bfbbb: 488d15d6a46b01  lea rdx, [rip + 0x16ba4d6]
1415bfbc2: 4c8d0597a46b01  lea r8, [rip + 0x16ba497]
1415bfbc9: 488b4d18  mov rcx, qword ptr [rbp + 0x18]
1415bfbcd: e84ea13301  call 0x1428f9d20
1415bfbd2: b901000000  mov ecx, 1
1415bfbd7: e92fffffff  jmp 0x1415bfb0b
1415bfbdc: 488d0dc5b36b01  lea rcx, [rip + 0x16bb3c5]
1415bfbe3: ba07000000  mov edx, 7
1415bfbe8: e8c3a13301  call 0x1428f9db0
1415bfbed: e951ffffff  jmp 0x1415bfb43
1415bfbf2: 488d0db6b36b01  lea rcx, [rip + 0x16bb3b6]
1415bfbf9: ba14000000  mov edx, 0x14
1415bfbfe: e8ada13301  call 0x1428f9db0
1415bfc03: e93bffffff  jmp 0x1415bfb43
1415bfc08: 0f1f840000000000  nop dword ptr [rax + rax]
1415bfc10: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bfc15: 55  push rbp
1415bfc16: 4157  push r15
1415bfc18: 4156  push r14
1415bfc1a: 4155  push r13
1415bfc1c: 4154  push r12
1415bfc1e: 56  push rsi
1415bfc1f: 57  push rdi
1415bfc20: 53  push rbx
1415bfc21: 4883ec58  sub rsp, 0x58
1415bfc25: 488daa80000000  lea rbp, [rdx + 0x80]
1415bfc2c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bfc32: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bfc37: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bfc3c: 488b5500  mov rdx, qword ptr [rbp]
1415bfc40: 4885d2  test rdx, rdx
1415bfc43: 7413  je 0x1415bfc58
1415bfc45: 48c1e205  shl rdx, 5
1415bfc49: 41b808000000  mov r8d, 8
1415bfc4f: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415bfc53: e8c83f05ff  call 0x140613c20
1415bfc58: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bfc5d: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bfc62: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bfc68: 4883c458  add rsp, 0x58
1415bfc6c: 5b  pop rbx
1415bfc6d: 5f  pop rdi
1415bfc6e: 5e  pop rsi
1415bfc6f: 415c  pop r12
1415bfc71: 415d  pop r13
1415bfc73: 415e  pop r14
1415bfc75: 415f  pop r15
1415bfc77: 5d  pop rbp
1415bfc78: c3  ret 
1415bfc79: 0f1f8000000000  nop dword ptr [rax]
1415bfc80: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bfc85: 55  push rbp
1415bfc86: 4157  push r15
1415bfc88: 4156  push r14
1415bfc8a: 4155  push r13
1415bfc8c: 4154  push r12
1415bfc8e: 56  push rsi
1415bfc8f: 57  push rdi
1415bfc90: 53  push rbx
1415bfc91: 4883ec58  sub rsp, 0x58
1415bfc95: 488daa80000000  lea rbp, [rdx + 0x80]
1415bfc9c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bfca2: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bfca7: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bfcac: 0f284530  movaps xmm0, xmmword ptr [rbp + 0x30]
1415bfcb0: 0f284d40  movaps xmm1, xmmword ptr [rbp + 0x40]
1415bfcb4: 488d45f8  lea rax, [rbp - 8]
1415bfcb8: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415bfcbc: 0f1100  movups xmmword ptr [rax], xmm0
1415bfcbf: 48837d7800  cmp qword ptr [rbp + 0x78], 0
1415bfcc4: 7413  je 0x1415bfcd9
1415bfcc6: 41b801000000  mov r8d, 1
1415bfccc: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415bfcd0: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415bfcd4: e8473f05ff  call 0x140613c20
1415bfcd9: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bfcde: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bfce3: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bfce9: 4883c458  add rsp, 0x58
1415bfced: 5b  pop rbx
1415bfcee: 5f  pop rdi
1415bfcef: 5e  pop rsi
1415bfcf0: 415c  pop r12
1415bfcf2: 415d  pop r13
1415bfcf4: 415e  pop r14
1415bfcf6: 415f  pop r15
1415bfcf8: 5d  pop rbp
1415bfcf9: c3  ret 
1415bfcfa: 660f1f440000  nop word ptr [rax + rax]
1415bfd00: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bfd05: 55  push rbp
1415bfd06: 4157  push r15
1415bfd08: 4156  push r14
1415bfd0a: 4155  push r13
1415bfd0c: 4154  push r12
1415bfd0e: 56  push rsi
1415bfd0f: 57  push rdi
1415bfd10: 53  push rbx
1415bfd11: 4883ec58  sub rsp, 0x58
1415bfd15: 488daa80000000  lea rbp, [rdx + 0x80]
1415bfd1c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bfd22: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bfd27: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bfd2c: 488d4db0  lea rcx, [rbp - 0x50]
1415bfd30: e8cba72cff  call 0x14088a500
1415bfd35: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bfd3a: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bfd3f: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bfd45: 4883c458  add rsp, 0x58
1415bfd49: 5b  pop rbx
1415bfd4a: 5f  pop rdi
1415bfd4b: 5e  pop rsi
1415bfd4c: 415c  pop r12
1415bfd4e: 415d  pop r13
1415bfd50: 415e  pop r14
1415bfd52: 415f  pop r15
1415bfd54: 5d  pop rbp
1415bfd55: c3  ret 
1415bfd56: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415bfd60: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bfd65: 55  push rbp
1415bfd66: 4157  push r15
1415bfd68: 4156  push r14
1415bfd6a: 4155  push r13
1415bfd6c: 4154  push r12
1415bfd6e: 56  push rsi
1415bfd6f: 57  push rdi
1415bfd70: 53  push rbx
1415bfd71: 4883ec58  sub rsp, 0x58
1415bfd75: 488daa80000000  lea rbp, [rdx + 0x80]
1415bfd7c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bfd82: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bfd87: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bfd8c: 488d4df8  lea rcx, [rbp - 8]
1415bfd90: e89be62cff  call 0x14088e430
1415bfd95: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bfd9a: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bfd9f: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bfda5: 4883c458  add rsp, 0x58
1415bfda9: 5b  pop rbx
1415bfdaa: 5f  pop rdi
1415bfdab: 5e  pop rsi
1415bfdac: 415c  pop r12
1415bfdae: 415d  pop r13
1415bfdb0: 415e  pop r14
1415bfdb2: 415f  pop r15
1415bfdb4: 5d  pop rbp
1415bfdb5: c3  ret 
1415bfdb6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415bfdc0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bfdc5: 55  push rbp
1415bfdc6: 4157  push r15
1415bfdc8: 4156  push r14
1415bfdca: 4155  push r13
1415bfdcc: 4154  push r12
1415bfdce: 56  push rsi
1415bfdcf: 57  push rdi
1415bfdd0: 53  push rbx
1415bfdd1: 4883ec58  sub rsp, 0x58
1415bfdd5: 488daa80000000  lea rbp, [rdx + 0x80]
1415bfddc: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bfde2: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bfde7: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bfdec: 488d4df8  lea rcx, [rbp - 8]
1415bfdf0: e83be62cff  call 0x14088e430
1415bfdf5: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bfdfa: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bfdff: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bfe05: 4883c458  add rsp, 0x58
1415bfe09: 5b  pop rbx
1415bfe0a: 5f  pop rdi
1415bfe0b: 5e  pop rsi
1415bfe0c: 415c  pop r12
1415bfe0e: 415d  pop r13
1415bfe10: 415e  pop r14
1415bfe12: 415f  pop r15
1415bfe14: 5d  pop rbp
1415bfe15: c3  ret 
