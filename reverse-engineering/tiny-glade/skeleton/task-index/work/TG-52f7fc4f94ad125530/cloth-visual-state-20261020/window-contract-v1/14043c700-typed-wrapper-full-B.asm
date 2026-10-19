14043c700: 56 push rsi
14043c701: 57 push rdi
14043c702: 53 push rbx
14043c703: 4881ec40020000 sub rsp, 0x240
14043c70a: 4889d7 mov rdi, rdx
14043c70d: 4889ce mov rsi, rcx
14043c710: e8cbe40600 call 0x1404aabe0
14043c715: bb01000000 mov ebx, 1
14043c71a: f00fc19fb0050000 lock xadd dword ptr [rdi + 0x5b0], ebx
14043c722: f60601 test byte ptr [rsi], 1
14043c725: 0f846c010000 je 0x14043c897
14043c72b: 488d5608 lea rdx, [rsi + 8]
14043c72f: 4c8d4650 lea r8, [rsi + 0x50]
14043c733: 895c2420 mov dword ptr [rsp + 0x20], ebx
14043c737: 488d8c2440010000 lea rcx, [rsp + 0x140]
14043c73f: 4989f9 mov r9, rdi
14043c742: e859f1faff call 0x1403eb8a0
14043c747: 0f10842440010000 movups xmm0, xmmword ptr [rsp + 0x140]
14043c74f: 0f108c2450010000 movups xmm1, xmmword ptr [rsp + 0x150]
14043c757: 0f10942460010000 movups xmm2, xmmword ptr [rsp + 0x160]
14043c75f: 0f109c2470010000 movups xmm3, xmmword ptr [rsp + 0x170]
14043c767: 0f295c2470 movaps xmmword ptr [rsp + 0x70], xmm3
14043c76c: 0f29542460 movaps xmmword ptr [rsp + 0x60], xmm2
14043c771: 0f294c2450 movaps xmmword ptr [rsp + 0x50], xmm1
14043c776: 0f29442440 movaps xmmword ptr [rsp + 0x40], xmm0
14043c77b: 488d942480000000 lea rdx, [rsp + 0x80]
14043c783: 0f10842480010000 movups xmm0, xmmword ptr [rsp + 0x180]
14043c78b: 0f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm0
14043c793: 0f10842490010000 movups xmm0, xmmword ptr [rsp + 0x190]
14043c79b: 0f29842490000000 movaps xmmword ptr [rsp + 0x90], xmm0
14043c7a3: 4c8d8424a0000000 lea r8, [rsp + 0xa0]
14043c7ab: 0f108424a0010000 movups xmm0, xmmword ptr [rsp + 0x1a0]
14043c7b3: 0f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm0
14043c7bb: 0f108424b0010000 movups xmm0, xmmword ptr [rsp + 0x1b0]
14043c7c3: 0f298424b0000000 movaps xmmword ptr [rsp + 0xb0], xmm0
14043c7cb: 4c8d8c24c0000000 lea r9, [rsp + 0xc0]
14043c7d3: 0f108424c0010000 movups xmm0, xmmword ptr [rsp + 0x1c0]
14043c7db: 0f298424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm0
14043c7e3: 0f108424d0010000 movups xmm0, xmmword ptr [rsp + 0x1d0]
14043c7eb: 0f298424d0000000 movaps xmmword ptr [rsp + 0xd0], xmm0
14043c7f3: 488d8424e0000000 lea rax, [rsp + 0xe0]
14043c7fb: 0f108424e0010000 movups xmm0, xmmword ptr [rsp + 0x1e0]
14043c803: 0f108c24f0010000 movups xmm1, xmmword ptr [rsp + 0x1f0]
14043c80b: 0f10942400020000 movups xmm2, xmmword ptr [rsp + 0x200]
14043c813: 0f109c2410020000 movups xmm3, xmmword ptr [rsp + 0x210]
14043c81b: 0f298c24f0000000 movaps xmmword ptr [rsp + 0xf0], xmm1
14043c823: 0f298424e0000000 movaps xmmword ptr [rsp + 0xe0], xmm0
14043c82b: 488d8c2400010000 lea rcx, [rsp + 0x100]
14043c833: 0f299c2410010000 movaps xmmword ptr [rsp + 0x110], xmm3
14043c83b: 0f29942400010000 movaps xmmword ptr [rsp + 0x100], xmm2
14043c843: 4c8d942420010000 lea r10, [rsp + 0x120]
14043c84b: 0f10842430020000 movups xmm0, xmmword ptr [rsp + 0x230]
14043c853: 0f29842430010000 movaps xmmword ptr [rsp + 0x130], xmm0
14043c85b: 0f10842420020000 movups xmm0, xmmword ptr [rsp + 0x220]
14043c863: 0f29842420010000 movaps xmmword ptr [rsp + 0x120], xmm0
14043c86b: 4c89542430 mov qword ptr [rsp + 0x30], r10
14043c870: 48894c2428 mov qword ptr [rsp + 0x28], rcx
14043c875: 4889442420 mov qword ptr [rsp + 0x20], rax
14043c87a: 488d4c2440 lea rcx, [rsp + 0x40]
14043c87f: e84cf73a00 call 0x1407ebfd0
14043c884: 899e80010000 mov dword ptr [rsi + 0x180], ebx
14043c88a: 31c0 xor eax, eax
14043c88c: 4881c440020000 add rsp, 0x240
14043c893: 5b pop rbx
14043c894: 5f pop rdi
14043c895: 5e pop rsi
14043c896: c3 ret 
14043c897: 488d0d72f75902 lea rcx, [rip + 0x259f772]
14043c89e: 4c8d0533f85902 lea r8, [rip + 0x259f833]
14043c8a5: ba59000000 mov edx, 0x59
14043c8aa: e881ca4902 call 0x1428d9330
14043c8af: cc int3 
