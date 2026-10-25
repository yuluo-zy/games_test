140439e50: 56 push rsi
140439e51: 57 push rdi
140439e52: 53 push rbx
140439e53: 4881ec40020000 sub rsp, 0x240
140439e5a: 4889d7 mov rdi, rdx
140439e5d: 4889ce mov rsi, rcx
140439e60: e87b0d0700 call 0x1404aabe0
140439e65: bb01000000 mov ebx, 1
140439e6a: f00fc19fb0050000 lock xadd dword ptr [rdi + 0x5b0], ebx
140439e72: f60601 test byte ptr [rsi], 1
140439e75: 0f846c010000 je 0x140439fe7
140439e7b: 488d5608 lea rdx, [rsi + 8]
140439e7f: 4c8d4650 lea r8, [rsi + 0x50]
140439e83: 895c2420 mov dword ptr [rsp + 0x20], ebx
140439e87: 488d8c2440010000 lea rcx, [rsp + 0x140]
140439e8f: 4989f9 mov r9, rdi
140439e92: e8091afbff call 0x1403eb8a0
140439e97: 0f10842440010000 movups xmm0, xmmword ptr [rsp + 0x140]
140439e9f: 0f108c2450010000 movups xmm1, xmmword ptr [rsp + 0x150]
140439ea7: 0f10942460010000 movups xmm2, xmmword ptr [rsp + 0x160]
140439eaf: 0f109c2470010000 movups xmm3, xmmword ptr [rsp + 0x170]
140439eb7: 0f295c2470 movaps xmmword ptr [rsp + 0x70], xmm3
140439ebc: 0f29542460 movaps xmmword ptr [rsp + 0x60], xmm2
140439ec1: 0f294c2450 movaps xmmword ptr [rsp + 0x50], xmm1
140439ec6: 0f29442440 movaps xmmword ptr [rsp + 0x40], xmm0
140439ecb: 488d942480000000 lea rdx, [rsp + 0x80]
140439ed3: 0f10842480010000 movups xmm0, xmmword ptr [rsp + 0x180]
140439edb: 0f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm0
140439ee3: 0f10842490010000 movups xmm0, xmmword ptr [rsp + 0x190]
140439eeb: 0f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm0
140439ef3: 4c8d8424a0000000 lea r8, [rsp + 0xa0]
140439efb: 0f108424a0010000 movups xmm0, xmmword ptr [rsp + 0x1a0]
140439f03: 0f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm0
140439f0b: 0f108424b0010000 movups xmm0, xmmword ptr [rsp + 0x1b0]
140439f13: 0f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm0
140439f1b: 4c8d8c24c0000000 lea r9, [rsp + 0xc0]
140439f23: 0f108424c0010000 movups xmm0, xmmword ptr [rsp + 0x1c0]
140439f2b: 0f298424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm0
140439f33: 0f108424d0010000 movups xmm0, xmmword ptr [rsp + 0x1d0]
140439f3b: 0f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm0
140439f43: 488d8424e0000000 lea rax, [rsp + 0xe0]
140439f4b: 0f108424e0010000 movups xmm0, xmmword ptr [rsp + 0x1e0]
140439f53: 0f108c24f0010000 movups xmm1, xmmword ptr [rsp + 0x1f0]
140439f5b: 0f10942400020000 movups xmm2, xmmword ptr [rsp + 0x200]
140439f63: 0f109c2410020000 movups xmm3, xmmword ptr [rsp + 0x210]
140439f6b: 0f298c24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm1
140439f73: 0f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm0
140439f7b: 488d8c2400010000 lea rcx, [rsp + 0x100]
140439f83: 0f299c2410010000 movaps xmmword ptr [rsp + 0x110], xmm3
140439f8b: 0f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm2
140439f93: 4c8d942420010000 lea r10, [rsp + 0x120]
140439f9b: 0f10842430020000 movups xmm0, xmmword ptr [rsp + 0x230]
140439fa3: 0f29842430010000 movaps xmmword ptr [rsp + 0x130], xmm0
140439fab: 0f10842420020000 movups xmm0, xmmword ptr [rsp + 0x220]
140439fb3: 0f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm0
140439fbb: 4c89542430 mov qword ptr [rsp + 0x30], r10
140439fc0: 48894c2428 mov qword ptr [rsp + 0x28], rcx
140439fc5: 4889442420 mov qword ptr [rsp + 0x20], rax
140439fca: 488d4c2440 lea rcx, [rsp + 0x40]
140439fcf: e8dc813800 call 0x1407c21b0
140439fd4: 899e80010000 mov dword ptr [rsi + 0x180], ebx
140439fda: 31c0 xor eax, eax
140439fdc: 4881c440020000 add rsp, 0x240
140439fe3: 5b pop rbx
140439fe4: 5f pop rdi
140439fe5: 5e pop rsi
140439fe6: c3 ret 
140439fe7: 488d0d22205a02 lea rcx, [rip + 0x25a2022]
140439fee: 4c8d05e3205a02 lea r8, [rip + 0x25a20e3]
140439ff5: ba59000000 mov edx, 0x59
140439ffa: e831f34902 call 0x1428d9330
140439fff: cc int3 
