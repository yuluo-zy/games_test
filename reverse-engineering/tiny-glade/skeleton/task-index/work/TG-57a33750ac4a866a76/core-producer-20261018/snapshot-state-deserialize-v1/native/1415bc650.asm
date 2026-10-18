1415bc650: 55  push rbp
1415bc651: 4157  push r15
1415bc653: 4156  push r14
1415bc655: 4155  push r13
1415bc657: 4154  push r12
1415bc659: 56  push rsi
1415bc65a: 57  push rdi
1415bc65b: 53  push rbx
1415bc65c: 4881ec58010000  sub rsp, 0x158
1415bc663: 488dac2480000000  lea rbp, [rsp + 0x80]
1415bc66b: 440f2985c0000000  movaps xmmword ptr [rbp + 0xc0], xmm8
1415bc673: 0f29bdb0000000  movaps xmmword ptr [rbp + 0xb0], xmm7
1415bc67a: 0f29b5a0000000  movaps xmmword ptr [rbp + 0xa0], xmm6
1415bc681: 48c78598000000feffffff  mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
1415bc68c: 4889ce  mov rsi, rcx
1415bc68f: 488b4210  mov rax, qword ptr [rdx + 0x10]
1415bc693: 488945b0  mov qword ptr [rbp - 0x50], rax
1415bc697: 488d5db8  lea rbx, [rbp - 0x48]
1415bc69b: 4889d9  mov rcx, rbx
1415bc69e: e82dba5cff  call 0x140b880d0
1415bc6a3: 4c8d6d31  lea r13, [rbp + 0x31]
1415bc6a7: c7859400000000000000  mov dword ptr [rbp + 0x94], 0
1415bc6b1: 0f57f6  xorps xmm6, xmm6
1415bc6b4: b001  mov al, 1
1415bc6b6: 898584000000  mov dword ptr [rbp + 0x84], eax
1415bc6bc: 4c8d7d60  lea r15, [rbp + 0x60]
1415bc6c0: 4c8d6530  lea r12, [rbp + 0x30]
1415bc6c4: 488d3da1ed6b01  lea rdi, [rip + 0x16beda1]
1415bc6cb: c7859000000000000000  mov dword ptr [rbp + 0x90], 0
1415bc6d5: 66662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415bc6e0: 4c89f9  mov rcx, r15
1415bc6e3: 4889da  mov rdx, rbx
1415bc6e6: e865fde7ff  call 0x14143c450
1415bc6eb: 488b4560  mov rax, qword ptr [rbp + 0x60]
1415bc6ef: 4885c0  test rax, rax
1415bc6f2: 0f8448030000  je 0x1415bca40
1415bc6f8: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
1415bc6fc: 488d1449  lea rdx, [rcx + rcx*2]
1415bc700: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
1415bc708: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
1415bc710: 4c898d88000000  mov qword ptr [rbp + 0x88], r9
1415bc717: 4c8bb4d078010000  mov r14, qword ptr [rax + rdx*8 + 0x178]
1415bc71f: 48c1e105  shl rcx, 5
1415bc723: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
1415bc727: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
1415bc72c: 0f294530  movaps xmmword ptr [rbp + 0x30], xmm0
1415bc730: 0f294d40  movaps xmmword ptr [rbp + 0x40], xmm1
1415bc734: 4c89c0  mov rax, r8
1415bc737: 48f7d8  neg rax
1415bc73a: 0f8000030000  jo 0x1415bca40
1415bc740: 4c894578  mov qword ptr [rbp + 0x78], r8
1415bc744: 0fb64500  movzx eax, byte ptr [rbp]
1415bc748: 4883f806  cmp rax, 6
1415bc74c: 0f876f010000  ja 0x1415bc8c1
1415bc752: 48630487  movsxd rax, dword ptr [rdi + rax*4]
1415bc756: 4801f8  add rax, rdi
1415bc759: ffe0  jmp rax
1415bc75b: 0f284530  movaps xmm0, xmmword ptr [rbp + 0x30]
1415bc75f: 0f284d40  movaps xmm1, xmmword ptr [rbp + 0x40]
1415bc763: 488d4500  lea rax, [rbp]
1415bc767: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415bc76b: 0f1100  movups xmmword ptr [rax], xmm0
1415bc76e: 4983fe02  cmp r14, 2
1415bc772: 747c  je 0x1415bc7f0
1415bc774: 4983fe0b  cmp r14, 0xb
1415bc778: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415bc77c: 7442  je 0x1415bc7c0
1415bc77e: 4983fe0a  cmp r14, 0xa
1415bc782: 0f8583000000  jne 0x1415bc80b
1415bc788: 488b8d88000000  mov rcx, qword ptr [rbp + 0x88]
1415bc78f: 488b01  mov rax, qword ptr [rcx]
1415bc792: 483305d62d5101  xor rax, qword ptr [rip + 0x1512dd6]
1415bc799: 0fb74908  movzx ecx, word ptr [rcx + 8]
1415bc79d: 66330dd32d5101  xor cx, word ptr [rip + 0x1512dd3]
1415bc7a4: 0fb7c9  movzx ecx, cx
1415bc7a7: 4809c1  or rcx, rax
1415bc7aa: 755f  jne 0x1415bc80b
1415bc7ac: 4531f6  xor r14d, r14d
1415bc7af: 4885d2  test rdx, rdx
1415bc7b2: 755f  jne 0x1415bc813
1415bc7b4: eb6f  jmp 0x1415bc825
1415bc7b6: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415bc7c0: 488b8d88000000  mov rcx, qword ptr [rbp + 0x88]
1415bc7c7: 488b01  mov rax, qword ptr [rcx]
1415bc7ca: 4833053e2d5101  xor rax, qword ptr [rip + 0x1512d3e]
1415bc7d1: 488b4903  mov rcx, qword ptr [rcx + 3]
1415bc7d5: 48330d362d5101  xor rcx, qword ptr [rip + 0x1512d36]
1415bc7dc: 4809c1  or rcx, rax
1415bc7df: 752a  jne 0x1415bc80b
1415bc7e1: 41b602  mov r14b, 2
1415bc7e4: 4885d2  test rdx, rdx
1415bc7e7: 752a  jne 0x1415bc813
1415bc7e9: eb3a  jmp 0x1415bc825
1415bc7eb: 0f1f440000  nop dword ptr [rax + rax]
1415bc7f0: 488b8588000000  mov rax, qword ptr [rbp + 0x88]
1415bc7f7: 0fb700  movzx eax, word ptr [rax]
1415bc7fa: 663b05782d5101  cmp ax, word ptr [rip + 0x1512d78]
1415bc801: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415bc805: 0f84c4000000  je 0x1415bc8cf
1415bc80b: 41b603  mov r14b, 3
1415bc80e: 4885d2  test rdx, rdx
1415bc811: 7412  je 0x1415bc825
1415bc813: 41b801000000  mov r8d, 1
1415bc819: 488b8d88000000  mov rcx, qword ptr [rbp + 0x88]
1415bc820: e8fb7305ff  call 0x140613c20
1415bc825: 4180fe03  cmp r14b, 3
1415bc829: 0f85b1000000  jne 0x1415bc8e0
1415bc82f: 0fb64500  movzx eax, byte ptr [rbp]
1415bc833: c6450006  mov byte ptr [rbp], 6
1415bc837: 3c06  cmp al, 6
1415bc839: 0f8464020000  je 0x1415bcaa3
1415bc83f: 884530  mov byte ptr [rbp + 0x30], al
1415bc842: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415bc846: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415bc84a: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415bc84f: 410f114500  movups xmmword ptr [r13], xmm0
1415bc854: 4c89e1  mov rcx, r12
1415bc857: e8e470feff  call 0x1415a3940
1415bc85c: 4885c0  test rax, rax
1415bc85f: 0f847bfeffff  je 0x1415bc6e0
1415bc865: e95d020000  jmp 0x1415bcac7
1415bc86a: 488b5508  mov rdx, qword ptr [rbp + 8]
1415bc86e: 4885d2  test rdx, rdx
1415bc871: 0f84e4feffff  je 0x1415bc75b
1415bc877: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415bc87b: 41b801000000  mov r8d, 1
1415bc881: e89a7305ff  call 0x140613c20
1415bc886: e9d0feffff  jmp 0x1415bc75b
1415bc88b: 488b4d10  mov rcx, qword ptr [rbp + 0x10]
1415bc88f: 488b5518  mov rdx, qword ptr [rbp + 0x18]
1415bc893: 48894d28  mov qword ptr [rbp + 0x28], rcx
1415bc897: e8d4c42cff  call 0x140888d70
1415bc89c: 488b5508  mov rdx, qword ptr [rbp + 8]
1415bc8a0: 4885d2  test rdx, rdx
1415bc8a3: 0f84b2feffff  je 0x1415bc75b
1415bc8a9: 48c1e205  shl rdx, 5
1415bc8ad: 41b808000000  mov r8d, 8
1415bc8b3: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415bc8b7: e8647305ff  call 0x140613c20
1415bc8bc: e99afeffff  jmp 0x1415bc75b
1415bc8c1: 488d4d08  lea rcx, [rbp + 8]
1415bc8c5: e876c62dff  call 0x140898f40
1415bc8ca: e98cfeffff  jmp 0x1415bc75b
1415bc8cf: 41b601  mov r14b, 1
1415bc8d2: 4885d2  test rdx, rdx
1415bc8d5: 0f8538ffffff  jne 0x1415bc813
1415bc8db: e945ffffff  jmp 0x1415bc825
1415bc8e0: 410fb6c6  movzx eax, r14b
1415bc8e4: 83f802  cmp eax, 2
1415bc8e7: 0f8480000000  je 0x1415bc96d
1415bc8ed: 83f801  cmp eax, 1
1415bc8f0: 0f85d2000000  jne 0x1415bc9c8
1415bc8f6: 83bd9400000000  cmp dword ptr [rbp + 0x94], 0
1415bc8fd: 0f8565020000  jne 0x1415bcb68
1415bc903: 0fb64500  movzx eax, byte ptr [rbp]
1415bc907: c6450006  mov byte ptr [rbp], 6
1415bc90b: 3c06  cmp al, 6
1415bc90d: 0f8416020000  je 0x1415bcb29
1415bc913: 884530  mov byte ptr [rbp + 0x30], al
1415bc916: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415bc91a: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415bc91e: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415bc923: 410f114500  movups xmmword ptr [r13], xmm0
1415bc928: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415bc931: 41b904000000  mov r9d, 4
1415bc937: 4c89f9  mov rcx, r15
1415bc93a: 4c89e2  mov rdx, r12
1415bc93d: 4c8d0504ab6b01  lea r8, [rip + 0x16bab04]
1415bc944: e8c770feff  call 0x1415a3a10
1415bc949: f6456001  test byte ptr [rbp + 0x60], 1
1415bc94d: 0f85bd010000  jne 0x1415bcb10
1415bc953: f3440f104564  movss xmm8, dword ptr [rbp + 0x64]
1415bc959: f30f107d68  movss xmm7, dword ptr [rbp + 0x68]
1415bc95e: c7859400000001000000  mov dword ptr [rbp + 0x94], 1
1415bc968: e973fdffff  jmp 0x1415bc6e0
1415bc96d: f6858400000001  test byte ptr [rbp + 0x84], 1
1415bc974: 0f84d8010000  je 0x1415bcb52
1415bc97a: 0fb64500  movzx eax, byte ptr [rbp]
1415bc97e: c6450006  mov byte ptr [rbp], 6
1415bc982: 3c06  cmp al, 6
1415bc984: 0f848c010000  je 0x1415bcb16
1415bc98a: 884530  mov byte ptr [rbp + 0x30], al
1415bc98d: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415bc991: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415bc995: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415bc99a: 410f114500  movups xmmword ptr [r13], xmm0
1415bc99f: 4c89f9  mov rcx, r15
1415bc9a2: 4c89e2  mov rdx, r12
1415bc9a5: e8c6d9fdff  call 0x14159a370
1415bc9aa: f6456001  test byte ptr [rbp + 0x60], 1
1415bc9ae: 0f855c010000  jne 0x1415bcb10
1415bc9b4: f30f107564  movss xmm6, dword ptr [rbp + 0x64]
1415bc9b9: c7858400000000000000  mov dword ptr [rbp + 0x84], 0
1415bc9c3: e918fdffff  jmp 0x1415bc6e0
1415bc9c8: 83bd9000000000  cmp dword ptr [rbp + 0x90], 0
1415bc9cf: 0f85a9010000  jne 0x1415bcb7e
1415bc9d5: 0fb64500  movzx eax, byte ptr [rbp]
1415bc9d9: c6450006  mov byte ptr [rbp], 6
1415bc9dd: 3c06  cmp al, 6
1415bc9df: 0f8457010000  je 0x1415bcb3c
1415bc9e5: 884530  mov byte ptr [rbp + 0x30], al
1415bc9e8: 0f104349  movups xmm0, xmmword ptr [rbx + 0x49]
1415bc9ec: 0f104b58  movups xmm1, xmmword ptr [rbx + 0x58]
1415bc9f0: 410f114d0f  movups xmmword ptr [r13 + 0xf], xmm1
1415bc9f5: 410f114500  movups xmmword ptr [r13], xmm0
1415bc9fa: 48c744242002000000  mov qword ptr [rsp + 0x20], 2
1415bca03: 41b90e000000  mov r9d, 0xe
1415bca09: 4c89f9  mov rcx, r15
1415bca0c: 4c89e2  mov rdx, r12
1415bca0f: 4c8d0542ad6b01  lea r8, [rip + 0x16bad42]
1415bca16: e8e570feff  call 0x1415a3b00
1415bca1b: f6456001  test byte ptr [rbp + 0x60], 1
1415bca1f: 0f85eb000000  jne 0x1415bcb10
1415bca25: 8b4564  mov eax, dword ptr [rbp + 0x64]
1415bca28: 89455c  mov dword ptr [rbp + 0x5c], eax
1415bca2b: 8b4568  mov eax, dword ptr [rbp + 0x68]
1415bca2e: 894558  mov dword ptr [rbp + 0x58], eax
1415bca31: c7859000000001000000  mov dword ptr [rbp + 0x90], 1
1415bca3b: e9a0fcffff  jmp 0x1415bc6e0
1415bca40: f6859000000001  test byte ptr [rbp + 0x90], 1
1415bca47: 7447  je 0x1415bca90
1415bca49: f6859400000001  test byte ptr [rbp + 0x94], 1
1415bca50: 7464  je 0x1415bcab6
1415bca52: 8b4558  mov eax, dword ptr [rbp + 0x58]
1415bca55: 894520  mov dword ptr [rbp + 0x20], eax
1415bca58: f3440f114524  movss dword ptr [rbp + 0x24], xmm8
1415bca5e: 48837df800  cmp qword ptr [rbp - 8], 0
1415bca63: 0f852b010000  jne 0x1415bcb94
1415bca69: 488b4520  mov rax, qword ptr [rbp + 0x20]
1415bca6d: 8b4d5c  mov ecx, dword ptr [rbp + 0x5c]
1415bca70: 894e04  mov dword ptr [rsi + 4], ecx
1415bca73: 48894608  mov qword ptr [rsi + 8], rax
1415bca77: f30f117e10  movss dword ptr [rsi + 0x10], xmm7
1415bca7c: f30f117614  movss dword ptr [rsi + 0x14], xmm6
1415bca81: 31c0  xor eax, eax
1415bca83: 8906  mov dword ptr [rsi], eax
1415bca85: 488d4db8  lea rcx, [rbp - 0x48]
1415bca89: e862b32cff  call 0x140887df0
1415bca8e: eb4a  jmp 0x1415bcada
1415bca90: 488d0db1e56b01  lea rcx, [rip + 0x16be5b1]
1415bca97: ba0a000000  mov edx, 0xa
1415bca9c: e81fd13301  call 0x1428f9bc0
1415bcaa1: eb24  jmp 0x1415bcac7
1415bcaa3: 488d0dbed76b01  lea rcx, [rip + 0x16bd7be]
1415bcaaa: ba10000000  mov edx, 0x10
1415bcaaf: e84cd53301  call 0x1428fa000
1415bcab4: eb11  jmp 0x1415bcac7
1415bcab6: 488d0d95e56b01  lea rcx, [rip + 0x16be595]
1415bcabd: ba02000000  mov edx, 2
1415bcac2: e8f9d03301  call 0x1428f9bc0
1415bcac7: 48894608  mov qword ptr [rsi + 8], rax
1415bcacb: c70601000000  mov dword ptr [rsi], 1
1415bcad1: 488d4db8  lea rcx, [rbp - 0x48]
1415bcad5: e816b32cff  call 0x140887df0
1415bcada: 488d4d00  lea rcx, [rbp]
1415bcade: e84d192dff  call 0x14088e430
1415bcae3: 4889f0  mov rax, rsi
1415bcae6: 0f28b5a0000000  movaps xmm6, xmmword ptr [rbp + 0xa0]
1415bcaed: 0f28bdb0000000  movaps xmm7, xmmword ptr [rbp + 0xb0]
1415bcaf4: 440f2885c0000000  movaps xmm8, xmmword ptr [rbp + 0xc0]
1415bcafc: 4881c458010000  add rsp, 0x158
1415bcb03: 5b  pop rbx
1415bcb04: 5f  pop rdi
1415bcb05: 5e  pop rsi
1415bcb06: 415c  pop r12
1415bcb08: 415d  pop r13
1415bcb0a: 415e  pop r14
1415bcb0c: 415f  pop r15
1415bcb0e: 5d  pop rbp
1415bcb0f: c3  ret 
1415bcb10: 488b4568  mov rax, qword ptr [rbp + 0x68]
1415bcb14: ebb1  jmp 0x1415bcac7
1415bcb16: 488d0d4bd76b01  lea rcx, [rip + 0x16bd74b]
1415bcb1d: ba10000000  mov edx, 0x10
1415bcb22: e8d9d43301  call 0x1428fa000
1415bcb27: eb9e  jmp 0x1415bcac7
1415bcb29: 488d0d38d76b01  lea rcx, [rip + 0x16bd738]
1415bcb30: ba10000000  mov edx, 0x10
1415bcb35: e8c6d43301  call 0x1428fa000
1415bcb3a: eb8b  jmp 0x1415bcac7
1415bcb3c: 488d0d25d76b01  lea rcx, [rip + 0x16bd725]
1415bcb43: ba10000000  mov edx, 0x10
1415bcb48: e8b3d43301  call 0x1428fa000
1415bcb4d: e975ffffff  jmp 0x1415bcac7
1415bcb52: 488d0dade46b01  lea rcx, [rip + 0x16be4ad]
1415bcb59: ba0b000000  mov edx, 0xb
1415bcb5e: e84dd23301  call 0x1428f9db0
1415bcb63: e95fffffff  jmp 0x1415bcac7
1415bcb68: 488d0de3e46b01  lea rcx, [rip + 0x16be4e3]
1415bcb6f: ba02000000  mov edx, 2
1415bcb74: e837d23301  call 0x1428f9db0
1415bcb79: e949ffffff  jmp 0x1415bcac7
1415bcb7e: 488d0dc3e46b01  lea rcx, [rip + 0x16be4c3]
1415bcb85: ba0a000000  mov edx, 0xa
1415bcb8a: e821d23301  call 0x1428f9db0
1415bcb8f: e933ffffff  jmp 0x1415bcac7
1415bcb94: 488d15fdd46b01  lea rdx, [rip + 0x16bd4fd]
1415bcb9b: 4c8d05bed46b01  lea r8, [rip + 0x16bd4be]
1415bcba2: 488b4db0  mov rcx, qword ptr [rbp - 0x50]
1415bcba6: e875d13301  call 0x1428f9d20
1415bcbab: 48894608  mov qword ptr [rsi + 8], rax
1415bcbaf: b801000000  mov eax, 1
1415bcbb4: e9cafeffff  jmp 0x1415bca83
1415bcbb9: 0f1f8000000000  nop dword ptr [rax]
1415bcbc0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bcbc5: 55  push rbp
1415bcbc6: 4157  push r15
1415bcbc8: 4156  push r14
1415bcbca: 4155  push r13
1415bcbcc: 4154  push r12
1415bcbce: 56  push rsi
1415bcbcf: 57  push rdi
1415bcbd0: 53  push rbx
1415bcbd1: 4883ec58  sub rsp, 0x58
1415bcbd5: 488daa80000000  lea rbp, [rdx + 0x80]
1415bcbdc: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bcbe2: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bcbe7: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bcbec: 488b5508  mov rdx, qword ptr [rbp + 8]
1415bcbf0: 4885d2  test rdx, rdx
1415bcbf3: 7413  je 0x1415bcc08
1415bcbf5: 48c1e205  shl rdx, 5
1415bcbf9: 41b808000000  mov r8d, 8
1415bcbff: 488b4d28  mov rcx, qword ptr [rbp + 0x28]
1415bcc03: e8187005ff  call 0x140613c20
1415bcc08: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bcc0d: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bcc12: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bcc18: 4883c458  add rsp, 0x58
1415bcc1c: 5b  pop rbx
1415bcc1d: 5f  pop rdi
1415bcc1e: 5e  pop rsi
1415bcc1f: 415c  pop r12
1415bcc21: 415d  pop r13
1415bcc23: 415e  pop r14
1415bcc25: 415f  pop r15
1415bcc27: 5d  pop rbp
1415bcc28: c3  ret 
1415bcc29: 0f1f8000000000  nop dword ptr [rax]
1415bcc30: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bcc35: 55  push rbp
1415bcc36: 4157  push r15
1415bcc38: 4156  push r14
1415bcc3a: 4155  push r13
1415bcc3c: 4154  push r12
1415bcc3e: 56  push rsi
1415bcc3f: 57  push rdi
1415bcc40: 53  push rbx
1415bcc41: 4883ec58  sub rsp, 0x58
1415bcc45: 488daa80000000  lea rbp, [rdx + 0x80]
1415bcc4c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bcc52: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bcc57: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bcc5c: 0f284530  movaps xmm0, xmmword ptr [rbp + 0x30]
1415bcc60: 0f284d40  movaps xmm1, xmmword ptr [rbp + 0x40]
1415bcc64: 488d4500  lea rax, [rbp]
1415bcc68: 0f114810  movups xmmword ptr [rax + 0x10], xmm1
1415bcc6c: 0f1100  movups xmmword ptr [rax], xmm0
1415bcc6f: 48837d7800  cmp qword ptr [rbp + 0x78], 0
1415bcc74: 7416  je 0x1415bcc8c
1415bcc76: 41b801000000  mov r8d, 1
1415bcc7c: 488b8d88000000  mov rcx, qword ptr [rbp + 0x88]
1415bcc83: 488b5578  mov rdx, qword ptr [rbp + 0x78]
1415bcc87: e8946f05ff  call 0x140613c20
1415bcc8c: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bcc91: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bcc96: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bcc9c: 4883c458  add rsp, 0x58
1415bcca0: 5b  pop rbx
1415bcca1: 5f  pop rdi
1415bcca2: 5e  pop rsi
1415bcca3: 415c  pop r12
1415bcca5: 415d  pop r13
1415bcca7: 415e  pop r14
1415bcca9: 415f  pop r15
1415bccab: 5d  pop rbp
1415bccac: c3  ret 
1415bccad: 0f1f00  nop dword ptr [rax]
1415bccb0: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bccb5: 55  push rbp
1415bccb6: 4157  push r15
1415bccb8: 4156  push r14
1415bccba: 4155  push r13
1415bccbc: 4154  push r12
1415bccbe: 56  push rsi
1415bccbf: 57  push rdi
1415bccc0: 53  push rbx
1415bccc1: 4883ec58  sub rsp, 0x58
1415bccc5: 488daa80000000  lea rbp, [rdx + 0x80]
1415bcccc: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bccd2: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bccd7: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bccdc: 488d4db8  lea rcx, [rbp - 0x48]
1415bcce0: e81bd82cff  call 0x14088a500
1415bcce5: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bccea: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bccef: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bccf5: 4883c458  add rsp, 0x58
1415bccf9: 5b  pop rbx
1415bccfa: 5f  pop rdi
1415bccfb: 5e  pop rsi
1415bccfc: 415c  pop r12
1415bccfe: 415d  pop r13
1415bcd00: 415e  pop r14
1415bcd02: 415f  pop r15
1415bcd04: 5d  pop rbp
1415bcd05: c3  ret 
1415bcd06: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415bcd10: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bcd15: 55  push rbp
1415bcd16: 4157  push r15
1415bcd18: 4156  push r14
1415bcd1a: 4155  push r13
1415bcd1c: 4154  push r12
1415bcd1e: 56  push rsi
1415bcd1f: 57  push rdi
1415bcd20: 53  push rbx
1415bcd21: 4883ec58  sub rsp, 0x58
1415bcd25: 488daa80000000  lea rbp, [rdx + 0x80]
1415bcd2c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bcd32: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bcd37: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bcd3c: 488d4d00  lea rcx, [rbp]
1415bcd40: e8eb162dff  call 0x14088e430
1415bcd45: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bcd4a: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bcd4f: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bcd55: 4883c458  add rsp, 0x58
1415bcd59: 5b  pop rbx
1415bcd5a: 5f  pop rdi
1415bcd5b: 5e  pop rsi
1415bcd5c: 415c  pop r12
1415bcd5e: 415d  pop r13
1415bcd60: 415e  pop r14
1415bcd62: 415f  pop r15
1415bcd64: 5d  pop rbp
1415bcd65: c3  ret 
1415bcd66: 662e0f1f840000000000  nop word ptr cs:[rax + rax]
1415bcd70: 4889542410  mov qword ptr [rsp + 0x10], rdx
1415bcd75: 55  push rbp
1415bcd76: 4157  push r15
1415bcd78: 4156  push r14
1415bcd7a: 4155  push r13
1415bcd7c: 4154  push r12
1415bcd7e: 56  push rsi
1415bcd7f: 57  push rdi
1415bcd80: 53  push rbx
1415bcd81: 4883ec58  sub rsp, 0x58
1415bcd85: 488daa80000000  lea rbp, [rdx + 0x80]
1415bcd8c: 440f29442420  movaps xmmword ptr [rsp + 0x20], xmm8
1415bcd92: 0f297c2430  movaps xmmword ptr [rsp + 0x30], xmm7
1415bcd97: 0f29742440  movaps xmmword ptr [rsp + 0x40], xmm6
1415bcd9c: 488d4d00  lea rcx, [rbp]
1415bcda0: e88b162dff  call 0x14088e430
1415bcda5: 0f28742440  movaps xmm6, xmmword ptr [rsp + 0x40]
1415bcdaa: 0f287c2430  movaps xmm7, xmmword ptr [rsp + 0x30]
1415bcdaf: 440f28442420  movaps xmm8, xmmword ptr [rsp + 0x20]
1415bcdb5: 4883c458  add rsp, 0x58
1415bcdb9: 5b  pop rbx
1415bcdba: 5f  pop rdi
1415bcdbb: 5e  pop rsi
1415bcdbc: 415c  pop r12
1415bcdbe: 415d  pop r13
1415bcdc0: 415e  pop r14
1415bcdc2: 415f  pop r15
1415bcdc4: 5d  pop rbp
1415bcdc5: c3  ret 
