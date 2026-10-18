1400d4610: push rbp
1400d4611: push rsi
1400d4612: push rdi
1400d4613: sub rsp, 0x340
1400d461a: lea rbp, [rsp + 0x80]
1400d4622: mov qword ptr [rbp + 0x2b8], 0xfffffffffffffffe
1400d462d: mov rdi, rdx
1400d4630: mov rsi, rcx
1400d4633: movups xmm0, xmmword ptr [rdx + 0x60]
1400d4637: movaps xmmword ptr [rbp + 0x2a0], xmm0
1400d463e: movups xmm0, xmmword ptr [rdx + 0x50]
1400d4642: movaps xmmword ptr [rbp + 0x290], xmm0
1400d4649: movups xmm0, xmmword ptr [rdx + 0x40]
1400d464d: movaps xmmword ptr [rbp + 0x280], xmm0
1400d4654: movups xmm0, xmmword ptr [rdx]
1400d4657: movups xmm1, xmmword ptr [rdx + 0x10]
1400d465b: movups xmm2, xmmword ptr [rdx + 0x20]
1400d465f: movups xmm3, xmmword ptr [rdx + 0x30]
1400d4663: movaps xmmword ptr [rbp + 0x270], xmm3
1400d466a: movaps xmmword ptr [rbp + 0x260], xmm2
1400d4671: movaps xmmword ptr [rbp + 0x250], xmm1
1400d4678: movaps xmmword ptr [rbp + 0x240], xmm0
1400d467f: movups xmm0, xmmword ptr [rdx + 0xd0]
1400d4686: movaps xmmword ptr [rbp + 0x230], xmm0
1400d468d: movups xmm0, xmmword ptr [rdx + 0xc0]
1400d4694: movaps xmmword ptr [rbp + 0x220], xmm0
1400d469b: movups xmm0, xmmword ptr [rdx + 0xb0]
1400d46a2: movaps xmmword ptr [rbp + 0x210], xmm0
1400d46a9: movups xmm0, xmmword ptr [rdx + 0x70]
1400d46ad: movups xmm1, xmmword ptr [rdx + 0x80]
1400d46b4: movups xmm2, xmmword ptr [rdx + 0x90]
1400d46bb: movups xmm3, xmmword ptr [rdx + 0xa0]
1400d46c2: movaps xmmword ptr [rbp + 0x200], xmm3
1400d46c9: movaps xmmword ptr [rbp + 0x1f0], xmm2
1400d46d0: movaps xmmword ptr [rbp + 0x1e0], xmm1
1400d46d7: movaps xmmword ptr [rbp + 0x1d0], xmm0
1400d46de: movups xmm0, xmmword ptr [rdx + 0x140]
1400d46e5: movaps xmmword ptr [rbp + 0x1c0], xmm0
1400d46ec: movups xmm0, xmmword ptr [rdx + 0x130]
1400d46f3: movaps xmmword ptr [rbp + 0x1b0], xmm0
1400d46fa: movups xmm0, xmmword ptr [rdx + 0x120]
1400d4701: movaps xmmword ptr [rbp + 0x1a0], xmm0
1400d4708: movups xmm0, xmmword ptr [rdx + 0xe0]
1400d470f: movups xmm1, xmmword ptr [rdx + 0xf0]
1400d4716: movups xmm2, xmmword ptr [rdx + 0x100]
1400d471d: movups xmm3, xmmword ptr [rdx + 0x110]
1400d4724: movaps xmmword ptr [rbp + 0x190], xmm3
1400d472b: movaps xmmword ptr [rbp + 0x180], xmm2
1400d4732: movaps xmmword ptr [rbp + 0x170], xmm1
1400d4739: movaps xmmword ptr [rbp + 0x160], xmm0
1400d4740: movups xmm0, xmmword ptr [rdx + 0x1b0]
1400d4747: movaps xmmword ptr [rbp + 0x150], xmm0
1400d474e: movups xmm0, xmmword ptr [rdx + 0x1a0]
1400d4755: movaps xmmword ptr [rbp + 0x140], xmm0
1400d475c: movups xmm0, xmmword ptr [rdx + 0x190]
1400d4763: movaps xmmword ptr [rbp + 0x130], xmm0
1400d476a: movups xmm0, xmmword ptr [rdx + 0x150]
1400d4771: movups xmm1, xmmword ptr [rdx + 0x160]
1400d4778: movups xmm2, xmmword ptr [rdx + 0x170]
1400d477f: movups xmm3, xmmword ptr [rdx + 0x180]
1400d4786: movaps xmmword ptr [rbp + 0x120], xmm3
1400d478d: movaps xmmword ptr [rbp + 0x110], xmm2
1400d4794: movaps xmmword ptr [rbp + 0x100], xmm1
1400d479b: movaps xmmword ptr [rbp + 0xf0], xmm0
1400d47a2: movups xmm0, xmmword ptr [rdx + 0x220]
1400d47a9: movaps xmmword ptr [rbp + 0xe0], xmm0
1400d47b0: movups xmm0, xmmword ptr [rdx + 0x210]
1400d47b7: movaps xmmword ptr [rbp + 0xd0], xmm0
1400d47be: movups xmm0, xmmword ptr [rdx + 0x200]
1400d47c5: movaps xmmword ptr [rbp + 0xc0], xmm0
1400d47cc: movups xmm0, xmmword ptr [rdx + 0x1c0]
1400d47d3: movups xmm1, xmmword ptr [rdx + 0x1d0]
1400d47da: movups xmm2, xmmword ptr [rdx + 0x1e0]
1400d47e1: movups xmm3, xmmword ptr [rdx + 0x1f0]
1400d47e8: movaps xmmword ptr [rbp + 0xb0], xmm3
1400d47ef: movaps xmmword ptr [rbp + 0xa0], xmm2
1400d47f6: movaps xmmword ptr [rbp + 0x90], xmm1
1400d47fd: movaps xmmword ptr [rbp + 0x80], xmm0
1400d4804: movups xmm0, xmmword ptr [rdx + 0x290]
1400d480b: movaps xmmword ptr [rbp + 0x70], xmm0
1400d480f: movups xmm0, xmmword ptr [rdx + 0x280]
1400d4816: movaps xmmword ptr [rbp + 0x60], xmm0
1400d481a: movups xmm0, xmmword ptr [rdx + 0x270]
1400d4821: movaps xmmword ptr [rbp + 0x50], xmm0
1400d4825: movups xmm0, xmmword ptr [rdx + 0x230]
1400d482c: movups xmm1, xmmword ptr [rdx + 0x240]
1400d4833: movups xmm2, xmmword ptr [rdx + 0x250]
1400d483a: movups xmm3, xmmword ptr [rdx + 0x260]
1400d4841: movaps xmmword ptr [rbp + 0x40], xmm3
1400d4845: movaps xmmword ptr [rbp + 0x30], xmm2
1400d4849: movaps xmmword ptr [rbp + 0x20], xmm1
1400d484d: movaps xmmword ptr [rbp + 0x10], xmm0
1400d4851: movups xmm0, xmmword ptr [rdx + 0x300]
1400d4858: movaps xmmword ptr [rbp], xmm0
1400d485c: movups xmm0, xmmword ptr [rdx + 0x2f0]
1400d4863: movaps xmmword ptr [rbp - 0x10], xmm0
1400d4867: movups xmm0, xmmword ptr [rdx + 0x2e0]
1400d486e: movaps xmmword ptr [rbp - 0x20], xmm0
1400d4872: movups xmm0, xmmword ptr [rdx + 0x2a0]
1400d4879: movups xmm1, xmmword ptr [rdx + 0x2b0]
1400d4880: movups xmm2, xmmword ptr [rdx + 0x2c0]
1400d4887: movups xmm3, xmmword ptr [rdx + 0x2d0]
1400d488e: movaps xmmword ptr [rbp - 0x30], xmm3
1400d4892: movaps xmmword ptr [rbp - 0x40], xmm2
1400d4896: movaps xmmword ptr [rbp - 0x50], xmm1
1400d489a: movaps xmmword ptr [rbp - 0x60], xmm0
1400d489e: movzx eax, byte ptr [rip + 0x3560364]
1400d48a5: mov ecx, 0x310
1400d48aa: mov edx, 8
1400d48af: call 0x140613c10
1400d48b4: test rax, rax
1400d48b7: je 0x1400d4b41
1400d48bd: lea r11, [rdi + 0x70]
1400d48c1: lea r10, [rdi + 0xe0]
1400d48c8: lea r9, [rdi + 0x150]
1400d48cf: lea r8, [rdi + 0x1c0]
1400d48d6: lea rdx, [rdi + 0x230]
1400d48dd: lea rcx, [rdi + 0x2a0]
1400d48e4: movups xmm0, xmmword ptr [rdi + 0x60]
1400d48e8: movups xmmword ptr [rax + 0x60], xmm0
1400d48ec: movups xmm0, xmmword ptr [rdi + 0x50]
1400d48f0: movups xmmword ptr [rax + 0x50], xmm0
1400d48f4: movups xmm0, xmmword ptr [rdi + 0x40]
1400d48f8: movups xmmword ptr [rax + 0x40], xmm0
1400d48fc: movups xmm0, xmmword ptr [rdi]
1400d48ff: movups xmm1, xmmword ptr [rdi + 0x10]
1400d4903: movups xmm2, xmmword ptr [rdi + 0x20]
1400d4907: movups xmm3, xmmword ptr [rdi + 0x30]
1400d490b: movups xmmword ptr [rax + 0x30], xmm3
1400d490f: movups xmmword ptr [rax + 0x20], xmm2
1400d4913: movups xmmword ptr [rax + 0x10], xmm1
1400d4917: movups xmmword ptr [rax], xmm0
1400d491a: movups xmm0, xmmword ptr [r11]
1400d491e: movups xmm1, xmmword ptr [r11 + 0x10]
1400d4923: movups xmm2, xmmword ptr [r11 + 0x20]
1400d4928: movups xmm3, xmmword ptr [r11 + 0x30]
1400d492d: movups xmmword ptr [rax + 0x70], xmm0
1400d4931: movups xmmword ptr [rax + 0x80], xmm1
1400d4938: movups xmmword ptr [rax + 0x90], xmm2
1400d493f: movups xmmword ptr [rax + 0xa0], xmm3
1400d4946: movups xmm0, xmmword ptr [r11 + 0x40]
1400d494b: movups xmmword ptr [rax + 0xb0], xmm0
1400d4952: movups xmm0, xmmword ptr [r11 + 0x50]
1400d4957: movups xmmword ptr [rax + 0xc0], xmm0
1400d495e: movups xmm0, xmmword ptr [r11 + 0x60]
1400d4963: movups xmmword ptr [rax + 0xd0], xmm0
1400d496a: movups xmm0, xmmword ptr [r10 + 0x60]
1400d496f: movups xmmword ptr [rax + 0x140], xmm0
1400d4976: movups xmm0, xmmword ptr [r10 + 0x50]
1400d497b: movups xmmword ptr [rax + 0x130], xmm0
1400d4982: movups xmm0, xmmword ptr [r10 + 0x40]
1400d4987: movups xmmword ptr [rax + 0x120], xmm0
1400d498e: movups xmm0, xmmword ptr [r10]
1400d4992: movups xmm1, xmmword ptr [r10 + 0x10]
1400d4997: movups xmm2, xmmword ptr [r10 + 0x20]
1400d499c: movups xmm3, xmmword ptr [r10 + 0x30]
1400d49a1: movups xmmword ptr [rax + 0x110], xmm3
1400d49a8: movups xmmword ptr [rax + 0x100], xmm2
1400d49af: movups xmmword ptr [rax + 0xf0], xmm1
1400d49b6: movups xmmword ptr [rax + 0xe0], xmm0
1400d49bd: movups xmm0, xmmword ptr [r9]
1400d49c1: movups xmm1, xmmword ptr [r9 + 0x10]
1400d49c6: movups xmm2, xmmword ptr [r9 + 0x20]
1400d49cb: movups xmm3, xmmword ptr [r9 + 0x30]
1400d49d0: movups xmmword ptr [rax + 0x150], xmm0
1400d49d7: movups xmmword ptr [rax + 0x160], xmm1
1400d49de: movups xmmword ptr [rax + 0x170], xmm2
1400d49e5: movups xmmword ptr [rax + 0x180], xmm3
1400d49ec: movups xmm0, xmmword ptr [r9 + 0x40]
1400d49f1: movups xmmword ptr [rax + 0x190], xmm0
1400d49f8: movups xmm0, xmmword ptr [r9 + 0x50]
1400d49fd: movups xmmword ptr [rax + 0x1a0], xmm0
1400d4a04: movups xmm0, xmmword ptr [r9 + 0x60]
1400d4a09: movups xmmword ptr [rax + 0x1b0], xmm0
1400d4a10: movups xmm0, xmmword ptr [r8]
1400d4a14: movups xmm1, xmmword ptr [r8 + 0x10]
1400d4a19: movups xmm2, xmmword ptr [r8 + 0x20]
1400d4a1e: movups xmm3, xmmword ptr [r8 + 0x30]
1400d4a23: movups xmmword ptr [rax + 0x1c0], xmm0
1400d4a2a: movups xmmword ptr [rax + 0x1d0], xmm1
1400d4a31: movups xmmword ptr [rax + 0x1e0], xmm2
1400d4a38: movups xmmword ptr [rax + 0x1f0], xmm3
1400d4a3f: movups xmm0, xmmword ptr [r8 + 0x40]
1400d4a44: movups xmmword ptr [rax + 0x200], xmm0
1400d4a4b: movups xmm0, xmmword ptr [r8 + 0x60]
1400d4a50: movups xmmword ptr [rax + 0x220], xmm0
1400d4a57: movups xmm0, xmmword ptr [r8 + 0x50]
1400d4a5c: movups xmmword ptr [rax + 0x210], xmm0
1400d4a63: movups xmm0, xmmword ptr [rdx + 0x60]
1400d4a67: movups xmmword ptr [rax + 0x290], xmm0
1400d4a6e: movups xmm0, xmmword ptr [rdx + 0x50]
1400d4a72: movups xmmword ptr [rax + 0x280], xmm0
1400d4a79: movups xmm0, xmmword ptr [rdx + 0x40]
1400d4a7d: movups xmmword ptr [rax + 0x270], xmm0
1400d4a84: movups xmm0, xmmword ptr [rdx]
1400d4a87: movups xmm1, xmmword ptr [rdx + 0x10]
1400d4a8b: movups xmm2, xmmword ptr [rdx + 0x20]
1400d4a8f: movups xmm3, xmmword ptr [rdx + 0x30]
1400d4a93: movups xmmword ptr [rax + 0x260], xmm3
1400d4a9a: movups xmmword ptr [rax + 0x250], xmm2
1400d4aa1: movups xmmword ptr [rax + 0x240], xmm1
1400d4aa8: movups xmmword ptr [rax + 0x230], xmm0
1400d4aaf: movups xmm0, xmmword ptr [rcx]
1400d4ab2: movups xmm1, xmmword ptr [rcx + 0x10]
1400d4ab6: movups xmm2, xmmword ptr [rcx + 0x20]
1400d4aba: movups xmm3, xmmword ptr [rcx + 0x30]
1400d4abe: movups xmmword ptr [rax + 0x2a0], xmm0
1400d4ac5: movups xmmword ptr [rax + 0x2b0], xmm1
1400d4acc: movups xmmword ptr [rax + 0x2c0], xmm2
1400d4ad3: movups xmmword ptr [rax + 0x2d0], xmm3
1400d4ada: movups xmm0, xmmword ptr [rcx + 0x40]
1400d4ade: movups xmmword ptr [rax + 0x2e0], xmm0
1400d4ae5: movups xmm0, xmmword ptr [rcx + 0x50]
1400d4ae9: movups xmmword ptr [rax + 0x2f0], xmm0
1400d4af0: movups xmm0, xmmword ptr [rcx + 0x60]
1400d4af4: movups xmmword ptr [rax + 0x300], xmm0
1400d4afb: mov qword ptr [rsi + 8], 7
1400d4b03: mov qword ptr [rsi + 0x10], rax
1400d4b07: mov qword ptr [rsi + 0x18], 7
1400d4b0f: mov qword ptr [rsi + 0x20], 0
1400d4b17: mov qword ptr [rsi + 0x28], 8
1400d4b1f: xorps xmm0, xmm0
1400d4b22: movups xmmword ptr [rsi + 0x30], xmm0
1400d4b26: movabs rax, 0x8000000000000000
1400d4b30: mov qword ptr [rsi], rax
1400d4b33: mov rax, rsi
1400d4b36: add rsp, 0x340
1400d4b3d: pop rdi
1400d4b3e: pop rsi
1400d4b3f: pop rbp
1400d4b40: ret 
1400d4b41: mov ecx, 8
1400d4b46: mov edx, 0x310
1400d4b4b: call 0x1428d9000
1400d4b50: ud2 
1400d4b52: nop word ptr cs:[rax + rax]
1400d4b60: mov qword ptr [rsp + 0x10], rdx
1400d4b65: push rbp
1400d4b66: push rsi
1400d4b67: push rdi
1400d4b68: sub rsp, 0x20
1400d4b6c: lea rbp, [rdx + 0x80]
1400d4b73: lea rcx, [rbp - 0x60]
1400d4b77: call 0x140029880
1400d4b7c: lea rcx, [rbp + 0x10]
1400d4b80: call 0x140029880
1400d4b85: lea rcx, [rbp + 0x80]
1400d4b8c: call 0x140029880
1400d4b91: lea rcx, [rbp + 0xf0]
1400d4b98: call 0x140029880
1400d4b9d: lea rcx, [rbp + 0x160]
1400d4ba4: call 0x140029880
1400d4ba9: lea rcx, [rbp + 0x1d0]
1400d4bb0: call 0x140029880
1400d4bb5: lea rcx, [rbp + 0x240]
1400d4bbc: call 0x140029880
1400d4bc1: nop 
1400d4bc2: add rsp, 0x20
1400d4bc6: pop rdi
1400d4bc7: pop rsi
1400d4bc8: pop rbp
1400d4bc9: ret 
