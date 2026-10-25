
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141d867b0: 41 57                       	pushq	%r15
141d867b2: 41 56                       	pushq	%r14
141d867b4: 41 55                       	pushq	%r13
141d867b6: 41 54                       	pushq	%r12
141d867b8: 56                          	pushq	%rsi
141d867b9: 57                          	pushq	%rdi
141d867ba: 55                          	pushq	%rbp
141d867bb: 53                          	pushq	%rbx
141d867bc: 48 81 ec 98 01 00 00        	subq	$0x198, %rsp            # imm = 0x198
141d867c3: 44 0f 29 94 24 80 01 00 00  	movaps	%xmm10, 0x180(%rsp)
141d867cc: 44 0f 29 8c 24 70 01 00 00  	movaps	%xmm9, 0x170(%rsp)
141d867d5: 44 0f 29 84 24 60 01 00 00  	movaps	%xmm8, 0x160(%rsp)
141d867de: 0f 29 bc 24 50 01 00 00     	movaps	%xmm7, 0x150(%rsp)
141d867e6: 0f 29 b4 24 40 01 00 00     	movaps	%xmm6, 0x140(%rsp)
141d867ee: 48 8b 19                    	movq	(%rcx), %rbx
141d867f1: 4c 8b 71 08                 	movq	0x8(%rcx), %r14
141d867f5: 44 8b 49 10                 	movl	0x10(%rcx), %r9d
141d867f9: 8b 41 14                    	movl	0x14(%rcx), %eax
141d867fc: 49 8d b6 a0 01 00 00        	leaq	0x1a0(%r14), %rsi
141d86803: 49 8d be f8 00 00 00        	leaq	0xf8(%r14), %rdi
141d8680a: 48 8d 8c 24 a0 00 00 00     	leaq	0xa0(%rsp), %rcx
141d86812: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
141d86816: 4c 89 f2                    	movq	%r14, %rdx
141d86819: 49 89 d8                    	movq	%rbx, %r8
141d8681c: e8 cf 83 58 00              	callq	0x14230ebf0 <.text+0x230dbf0>
141d86821: 4c 89 74 24 60              	movq	%r14, 0x60(%rsp)
141d86826: 4c 89 b4 24 80 00 00 00     	movq	%r14, 0x80(%rsp)
141d8682e: 48 89 b4 24 88 00 00 00     	movq	%rsi, 0x88(%rsp)
141d86836: 48 89 bc 24 90 00 00 00     	movq	%rdi, 0x90(%rsp)
141d8683e: 48 89 5c 24 78              	movq	%rbx, 0x78(%rsp)
141d86843: 48 89 9c 24 98 00 00 00     	movq	%rbx, 0x98(%rsp)
141d8684b: 48 8b 84 24 a0 00 00 00     	movq	0xa0(%rsp), %rax
141d86853: 48 8b 8c 24 a8 00 00 00     	movq	0xa8(%rsp), %rcx
141d8685b: 48 8b 94 24 c0 00 00 00     	movq	0xc0(%rsp), %rdx
141d86863: 48 89 54 24 70              	movq	%rdx, 0x70(%rsp)
141d86868: 48 8b 94 24 d0 00 00 00     	movq	0xd0(%rsp), %rdx
141d86870: 48 89 54 24 58              	movq	%rdx, 0x58(%rsp)
141d86875: 48 8b 94 24 e0 00 00 00     	movq	0xe0(%rsp), %rdx
141d8687d: 48 89 54 24 50              	movq	%rdx, 0x50(%rsp)
141d86882: 8b 94 24 ec 00 00 00        	movl	0xec(%rsp), %edx
141d86889: 89 54 24 6c                 	movl	%edx, 0x6c(%rsp)
141d8688d: 48 8b 94 24 f0 00 00 00     	movq	0xf0(%rsp), %rdx
141d86895: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
141d8689a: 48 8b 94 24 f8 00 00 00     	movq	0xf8(%rsp), %rdx
141d868a2: 48 89 54 24 38              	movq	%rdx, 0x38(%rsp)
141d868a7: 48 8b 94 24 00 01 00 00     	movq	0x100(%rsp), %rdx
141d868af: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
141d868b4: 4c 8b 94 24 08 01 00 00     	movq	0x108(%rsp), %r10
141d868bc: 4c 8b b4 24 18 01 00 00     	movq	0x118(%rsp), %r14
141d868c4: 8b 94 24 24 01 00 00        	movl	0x124(%rsp), %edx
141d868cb: 89 54 24 4c                 	movl	%edx, 0x4c(%rsp)
141d868cf: 4c 8b 9c 24 28 01 00 00     	movq	0x128(%rsp), %r11
141d868d7: 4c 8b 84 24 30 01 00 00     	movq	0x130(%rsp), %r8
141d868df: 0f b6 94 24 38 01 00 00     	movzbl	0x138(%rsp), %edx
141d868e7: 88 54 24 2f                 	movb	%dl, 0x2f(%rsp)
141d868eb: f3 0f 10 05 11 f0 b9 00     	movss	0xb9f011(%rip), %xmm0   # 0x142925904
141d868f3: f3 0f 10 0d 35 14 1e 01     	movss	0x11e1435(%rip), %xmm1  # 0x142f67d30
141d868fb: 0f 28 15 3e 14 1e 01        	movaps	0x11e143e(%rip), %xmm2  # 0x142f67d40
141d86902: f3 0f 10 1d 66 ef b9 00     	movss	0xb9ef66(%rip), %xmm3   # 0x142925870
141d8690a: f3 0f 10 25 be ef b9 00     	movss	0xb9efbe(%rip), %xmm4   # 0x1429258d0
141d86912: f3 0f 10 2d 9e d3 cf 00     	movss	0xcfd39e(%rip), %xmm5   # 0x142a83cb8
141d8691a: f6 44 24 2f 01              	testb	$0x1, 0x2f(%rsp)
141d8691f: 74 4f                       	je	0x141d86970 <.text+0x1d85970>
141d86921: 4d 39 d8                    	cmpq	%r11, %r8
141d86924: 0f 85 4a 02 00 00           	jne	0x141d86b74 <.text+0x1d85b74>
141d8692a: 48 39 c8                    	cmpq	%rcx, %rax
141d8692d: 48 8b 74 24 78              	movq	0x78(%rsp), %rsi
141d86932: 0f 84 2d 05 00 00           	je	0x141d86e65 <.text+0x1d85e65>
141d86938: 48 8b 54 24 60              	movq	0x60(%rsp), %rdx
141d8693d: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
141d86944: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141d86950: 44 8b 00                    	movl	(%rax), %r8d
141d86953: 4f 8d 04 c0                 	leaq	(%r8,%r8,8), %r8
141d86957: 4e 8b 5c c2 10              	movq	0x10(%rdx,%r8,8), %r11
141d8695c: 4d 85 db                    	testq	%r11, %r11
141d8695f: 75 62                       	jne	0x141d869c3 <.text+0x1d859c3>
141d86961: 48 83 c0 04                 	addq	$0x4, %rax
141d86965: 48 39 c8                    	cmpq	%rcx, %rax
141d86968: 75 e6                       	jne	0x141d86950 <.text+0x1d85950>
141d8696a: e9 f6 04 00 00              	jmp	0x141d86e65 <.text+0x1d85e65>
141d8696f: 90                          	nop
141d86970: 4d 39 d8                    	cmpq	%r11, %r8
141d86973: 0f 85 f4 02 00 00           	jne	0x141d86c6d <.text+0x1d85c6d>
141d86979: 48 39 c8                    	cmpq	%rcx, %rax
141d8697c: 48 8b 7c 24 78              	movq	0x78(%rsp), %rdi
141d86981: 0f 84 de 04 00 00           	je	0x141d86e65 <.text+0x1d85e65>
141d86987: 48 8b 54 24 60              	movq	0x60(%rsp), %rdx
141d8698c: 48 8b 92 00 01 00 00        	movq	0x100(%rdx), %rdx
141d86993: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141d869a0: 44 8b 00                    	movl	(%rax), %r8d
141d869a3: 4f 8d 04 80                 	leaq	(%r8,%r8,4), %r8
141d869a7: 49 c1 e0 05                 	shlq	$0x5, %r8
141d869ab: 4e 8b 5c 02 58              	movq	0x58(%rdx,%r8), %r11
141d869b0: 4d 85 db                    	testq	%r11, %r11
141d869b3: 75 65                       	jne	0x141d86a1a <.text+0x1d85a1a>
141d869b5: 48 83 c0 04                 	addq	$0x4, %rax
141d869b9: 48 39 c8                    	cmpq	%rcx, %rax
141d869bc: 75 e2                       	jne	0x141d869a0 <.text+0x1d859a0>
141d869be: e9 a2 04 00 00              	jmp	0x141d86e65 <.text+0x1d85e65>
141d869c3: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
141d869c7: 4c 8b 8e 10 01 00 00        	movq	0x110(%rsi), %r9
141d869ce: 4c 8b 42 40                 	movq	0x40(%rdx), %r8
141d869d2: 4d 39 c8                    	cmpq	%r9, %r8
141d869d5: 0f 86 b1 00 00 00           	jbe	0x141d86a8c <.text+0x1d85a8c>
141d869db: 4c 8b 52 38                 	movq	0x38(%rdx), %r10
141d869df: 4f 8b 0c ca                 	movq	(%r10,%r9,8), %r9
141d869e3: 4d 85 c9                    	testq	%r9, %r9
141d869e6: 0f 84 a0 00 00 00           	je	0x141d86a8c <.text+0x1d85a8c>
141d869ec: 4c 8b 52 18                 	movq	0x18(%rdx), %r10
141d869f0: 49 f7 d1                    	notq	%r9
141d869f3: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141d869f7: 49 c1 e1 04                 	shlq	$0x4, %r9
141d869fb: 4f 8b 4c 0a 10              	movq	0x10(%r10,%r9), %r9
141d86a00: 4c 89 4c 24 40              	movq	%r9, 0x40(%rsp)
141d86a05: 4c 8b 8e 18 01 00 00        	movq	0x118(%rsi), %r9
141d86a0c: 4d 39 c8                    	cmpq	%r9, %r8
141d86a0f: 0f 87 8c 00 00 00           	ja	0x141d86aa1 <.text+0x1d85aa1>
141d86a15: e9 c3 00 00 00              	jmp	0x141d86add <.text+0x1d85add>
141d86a1a: 4c 01 c2                    	addq	%r8, %rdx
141d86a1d: 44 8b 82 94 00 00 00        	movl	0x94(%rdx), %r8d
141d86a24: 4c 8b 4c 24 60              	movq	0x60(%rsp), %r9
141d86a29: 4d 8b 89 a8 01 00 00        	movq	0x1a8(%r9), %r9
141d86a30: 4b 8d 34 c0                 	leaq	(%r8,%r8,8), %rsi
141d86a34: 4d 8d 04 f1                 	leaq	(%r9,%rsi,8), %r8
141d86a38: 4c 8b 97 10 01 00 00        	movq	0x110(%rdi), %r10
141d86a3f: 4d 8b 4c f1 40              	movq	0x40(%r9,%rsi,8), %r9
141d86a44: 4d 39 d1                    	cmpq	%r10, %r9
141d86a47: 0f 86 2f 01 00 00           	jbe	0x141d86b7c <.text+0x1d85b7c>
141d86a4d: 49 8b 70 38                 	movq	0x38(%r8), %rsi
141d86a51: 4e 8b 14 d6                 	movq	(%rsi,%r10,8), %r10
141d86a55: 4d 85 d2                    	testq	%r10, %r10
141d86a58: 0f 84 1e 01 00 00           	je	0x141d86b7c <.text+0x1d85b7c>
141d86a5e: 49 8b 70 18                 	movq	0x18(%r8), %rsi
141d86a62: 49 f7 d2                    	notq	%r10
141d86a65: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
141d86a69: 49 c1 e2 04                 	shlq	$0x4, %r10
141d86a6d: 4e 8b 54 16 10              	movq	0x10(%rsi,%r10), %r10
141d86a72: 4c 89 54 24 40              	movq	%r10, 0x40(%rsp)
141d86a77: 4c 8b 97 18 01 00 00        	movq	0x118(%rdi), %r10
141d86a7e: 4d 39 d1                    	cmpq	%r10, %r9
141d86a81: 0f 87 0a 01 00 00           	ja	0x141d86b91 <.text+0x1d85b91>
141d86a87: e9 41 01 00 00              	jmp	0x141d86bcd <.text+0x1d85bcd>
141d86a8c: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
141d86a95: 4c 8b 8e 18 01 00 00        	movq	0x118(%rsi), %r9
141d86a9c: 4d 39 c8                    	cmpq	%r9, %r8
141d86a9f: 76 3c                       	jbe	0x141d86add <.text+0x1d85add>
141d86aa1: 4c 8b 52 38                 	movq	0x38(%rdx), %r10
141d86aa5: 4f 8b 0c ca                 	movq	(%r10,%r9,8), %r9
141d86aa9: 4d 85 c9                    	testq	%r9, %r9
141d86aac: 74 2f                       	je	0x141d86add <.text+0x1d85add>
141d86aae: 4c 8b 52 18                 	movq	0x18(%rdx), %r10
141d86ab2: 49 f7 d1                    	notq	%r9
141d86ab5: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141d86ab9: 49 c1 e1 04                 	shlq	$0x4, %r9
141d86abd: 4f 8b 4c 0a 10              	movq	0x10(%r10,%r9), %r9
141d86ac2: 4c 89 4c 24 38              	movq	%r9, 0x38(%rsp)
141d86ac7: 4c 8b 96 20 01 00 00        	movq	0x120(%rsi), %r10
141d86ace: 4c 8b 4a 38                 	movq	0x38(%rdx), %r9
141d86ad2: 4d 39 d0                    	cmpq	%r10, %r8
141d86ad5: 77 1f                       	ja	0x141d86af6 <.text+0x1d85af6>
141d86ad7: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
141d86adb: eb 3d                       	jmp	0x141d86b1a <.text+0x1d85b1a>
141d86add: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
141d86ae6: 4c 8b 96 20 01 00 00        	movq	0x120(%rsi), %r10
141d86aed: 4c 8b 4a 38                 	movq	0x38(%rdx), %r9
141d86af1: 4d 39 d0                    	cmpq	%r10, %r8
141d86af4: 76 e1                       	jbe	0x141d86ad7 <.text+0x1d85ad7>
141d86af6: 4f 8b 04 d1                 	movq	(%r9,%r10,8), %r8
141d86afa: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
141d86afe: 4d 85 c0                    	testq	%r8, %r8
141d86b01: 74 17                       	je	0x141d86b1a <.text+0x1d85b1a>
141d86b03: 49 f7 d0                    	notq	%r8
141d86b06: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
141d86b0a: 49 c1 e0 04                 	shlq	$0x4, %r8
141d86b0e: 4e 8b 44 02 10              	movq	0x10(%rdx,%r8), %r8
141d86b13: 4c 89 44 24 30              	movq	%r8, 0x30(%rsp)
141d86b18: eb 09                       	jmp	0x141d86b23 <.text+0x1d85b23>
141d86b1a: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141d86b23: 48 83 c0 04                 	addq	$0x4, %rax
141d86b27: 4c 8b 86 28 01 00 00        	movq	0x128(%rsi), %r8
141d86b2e: 4c 8b 96 30 01 00 00        	movq	0x130(%rsi), %r10
141d86b35: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
141d86b39: 49 f7 d0                    	notq	%r8
141d86b3c: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
141d86b40: 49 c1 e0 04                 	shlq	$0x4, %r8
141d86b44: 4a 8b 74 02 10              	movq	0x10(%rdx,%r8), %rsi
141d86b49: 48 89 74 24 58              	movq	%rsi, 0x58(%rsp)
141d86b4e: 4e 8b 44 02 28              	movq	0x28(%rdx,%r8), %r8
141d86b53: 4c 89 44 24 50              	movq	%r8, 0x50(%rsp)
141d86b58: 4f 8b 04 d1                 	movq	(%r9,%r10,8), %r8
141d86b5c: 49 f7 d0                    	notq	%r8
141d86b5f: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
141d86b63: 49 c1 e0 04                 	shlq	$0x4, %r8
141d86b67: 4e 8b 54 02 10              	movq	0x10(%rdx,%r8), %r10
141d86b6c: 4e 8b 74 02 28              	movq	0x28(%rdx,%r8), %r14
141d86b71: 45 31 c0                    	xorl	%r8d, %r8d
141d86b74: 44 89 c3                    	movl	%r8d, %ebx
141d86b77: e9 02 01 00 00              	jmp	0x141d86c7e <.text+0x1d85c7e>
141d86b7c: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
141d86b85: 4c 8b 97 18 01 00 00        	movq	0x118(%rdi), %r10
141d86b8c: 4d 39 d1                    	cmpq	%r10, %r9
141d86b8f: 76 3c                       	jbe	0x141d86bcd <.text+0x1d85bcd>
141d86b91: 49 8b 70 38                 	movq	0x38(%r8), %rsi
141d86b95: 4e 8b 14 d6                 	movq	(%rsi,%r10,8), %r10
141d86b99: 4d 85 d2                    	testq	%r10, %r10
141d86b9c: 74 2f                       	je	0x141d86bcd <.text+0x1d85bcd>
141d86b9e: 49 8b 70 18                 	movq	0x18(%r8), %rsi
141d86ba2: 49 f7 d2                    	notq	%r10
141d86ba5: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
141d86ba9: 49 c1 e2 04                 	shlq	$0x4, %r10
141d86bad: 4e 8b 54 16 10              	movq	0x10(%rsi,%r10), %r10
141d86bb2: 4c 89 54 24 38              	movq	%r10, 0x38(%rsp)
141d86bb7: 48 8b b7 20 01 00 00        	movq	0x120(%rdi), %rsi
141d86bbe: 4d 8b 50 38                 	movq	0x38(%r8), %r10
141d86bc2: 49 39 f1                    	cmpq	%rsi, %r9
141d86bc5: 77 1f                       	ja	0x141d86be6 <.text+0x1d85be6>
141d86bc7: 4d 8b 40 18                 	movq	0x18(%r8), %r8
141d86bcb: eb 3d                       	jmp	0x141d86c0a <.text+0x1d85c0a>
141d86bcd: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
141d86bd6: 48 8b b7 20 01 00 00        	movq	0x120(%rdi), %rsi
141d86bdd: 4d 8b 50 38                 	movq	0x38(%r8), %r10
141d86be1: 49 39 f1                    	cmpq	%rsi, %r9
141d86be4: 76 e1                       	jbe	0x141d86bc7 <.text+0x1d85bc7>
141d86be6: 4d 8b 0c f2                 	movq	(%r10,%rsi,8), %r9
141d86bea: 4d 8b 40 18                 	movq	0x18(%r8), %r8
141d86bee: 4d 85 c9                    	testq	%r9, %r9
141d86bf1: 74 17                       	je	0x141d86c0a <.text+0x1d85c0a>
141d86bf3: 49 f7 d1                    	notq	%r9
141d86bf6: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141d86bfa: 49 c1 e1 04                 	shlq	$0x4, %r9
141d86bfe: 4f 8b 4c 08 10              	movq	0x10(%r8,%r9), %r9
141d86c03: 4c 89 4c 24 30              	movq	%r9, 0x30(%rsp)
141d86c08: eb 09                       	jmp	0x141d86c13 <.text+0x1d85c13>
141d86c0a: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141d86c13: 48 83 c0 04                 	addq	$0x4, %rax
141d86c17: 4c 8b 8f 28 01 00 00        	movq	0x128(%rdi), %r9
141d86c1e: 48 8b b7 30 01 00 00        	movq	0x130(%rdi), %rsi
141d86c25: 4f 8b 0c ca                 	movq	(%r10,%r9,8), %r9
141d86c29: 49 f7 d1                    	notq	%r9
141d86c2c: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141d86c30: 49 c1 e1 04                 	shlq	$0x4, %r9
141d86c34: 4b 8b 7c 08 10              	movq	0x10(%r8,%r9), %rdi
141d86c39: 48 89 7c 24 58              	movq	%rdi, 0x58(%rsp)
141d86c3e: 4f 8b 4c 08 28              	movq	0x28(%r8,%r9), %r9
141d86c43: 4c 89 4c 24 50              	movq	%r9, 0x50(%rsp)
141d86c48: 4d 8b 0c f2                 	movq	(%r10,%rsi,8), %r9
141d86c4c: 49 f7 d1                    	notq	%r9
141d86c4f: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
141d86c53: 49 c1 e1 04                 	shlq	$0x4, %r9
141d86c57: 4f 8b 54 08 10              	movq	0x10(%r8,%r9), %r10
141d86c5c: 4f 8b 74 08 28              	movq	0x28(%r8,%r9), %r14
141d86c61: 48 8b 52 50                 	movq	0x50(%rdx), %rdx
141d86c65: 48 89 54 24 70              	movq	%rdx, 0x70(%rsp)
141d86c6a: 45 31 c0                    	xorl	%r8d, %r8d
141d86c6d: 4c 89 c2                    	movq	%r8, %rdx
141d86c70: 48 c1 e2 04                 	shlq	$0x4, %rdx
141d86c74: 4c 8b 4c 24 70              	movq	0x70(%rsp), %r9
141d86c79: 41 8b 5c 11 08              	movl	0x8(%r9,%rdx), %ebx
141d86c7e: 4c 8d 3c 5b                 	leaq	(%rbx,%rbx,2), %r15
141d86c82: 48 8b 54 24 38              	movq	0x38(%rsp), %rdx
141d86c87: 4e 8d 24 fa                 	leaq	(%rdx,%r15,8), %r12
141d86c8b: 49 c1 e7 04                 	shlq	$0x4, %r15
141d86c8f: 4c 03 7c 24 40              	addq	0x40(%rsp), %r15
141d86c94: 48 8b 54 24 50              	movq	0x50(%rsp), %rdx
141d86c99: 4c 8d 0c 9a                 	leaq	(%rdx,%rbx,4), %r9
141d86c9d: 49 8d 3c 9a                 	leaq	(%r10,%rbx,4), %rdi
141d86ca1: 49 8d 14 9e                 	leaq	(%r14,%rbx,4), %rdx
141d86ca5: 48 c1 e3 04                 	shlq	$0x4, %rbx
141d86ca9: 48 8b 74 24 30              	movq	0x30(%rsp), %rsi
141d86cae: 4c 8d 2c 1e                 	leaq	(%rsi,%rbx), %r13
141d86cb2: 48 03 5c 24 58              	addq	0x58(%rsp), %rbx
141d86cb7: 41 8b 75 00                 	movl	(%r13), %esi
141d86cbb: f3 41 0f 10 7f 18           	movss	0x18(%r15), %xmm7
141d86cc1: 40 f6 c6 01                 	testb	$0x1, %sil
141d86cc5: 75 09                       	jne	0x141d86cd0 <.text+0x1d85cd0>
141d86cc7: 0f 28 f7                    	movaps	%xmm7, %xmm6
141d86cca: eb 0a                       	jmp	0x141d86cd6 <.text+0x1d85cd6>
141d86ccc: 0f 1f 40 00                 	nopl	(%rax)
141d86cd0: f3 41 0f 10 75 0c           	movss	0xc(%r13), %xmm6
141d86cd6: f3 0f 5c f7                 	subss	%xmm7, %xmm6
141d86cda: 0f 28 fe                    	movaps	%xmm6, %xmm7
141d86cdd: f3 0f 59 fe                 	mulss	%xmm6, %xmm7
141d86ce1: f3 45 0f 7e 4d 04           	movq	0x4(%r13), %xmm9
141d86ce7: f3 45 0f 7e 57 10           	movq	0x10(%r15), %xmm10
141d86ced: 66 44 0f 6e c6              	movd	%esi, %xmm8
141d86cf2: 66 45 0f 70 c0 50           	pshufd	$0x50, %xmm8, %xmm8     # xmm8 = xmm8[0,0,1,1]
141d86cf8: 66 41 0f 72 f0 1f           	pslld	$0x1f, %xmm8
141d86cfe: 66 41 0f 72 e0 1f           	psrad	$0x1f, %xmm8
141d86d04: 66 45 0f db c8              	pand	%xmm8, %xmm9
141d86d09: 66 45 0f df c2              	pandn	%xmm10, %xmm8
141d86d0e: 66 45 0f eb c1              	por	%xmm9, %xmm8
141d86d13: 45 0f 5c c2                 	subps	%xmm10, %xmm8
141d86d17: 45 0f 28 c8                 	movaps	%xmm8, %xmm9
141d86d1b: 45 0f 59 c8                 	mulps	%xmm8, %xmm9
141d86d1f: f3 45 0f 16 d1              	movshdup	%xmm9, %xmm10           # xmm10 = xmm9[1,1,3,3]
141d86d24: f3 45 0f 58 d1              	addss	%xmm9, %xmm10
141d86d29: f3 44 0f 58 d7              	addss	%xmm7, %xmm10
141d86d2e: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
141d86d32: f3 45 0f 51 ca              	sqrtss	%xmm10, %xmm9
141d86d37: 0f 28 f8                    	movaps	%xmm0, %xmm7
141d86d3a: f3 41 0f 5e f9              	divss	%xmm9, %xmm7
141d86d3f: 66 0f 7e fd                 	movd	%xmm7, %ebp
141d86d43: 85 ed                       	testl	%ebp, %ebp
141d86d45: 41 0f 99 c5                 	setns	%r13b
141d86d49: 89 ee                       	movl	%ebp, %esi
141d86d4b: 81 e6 ff ff ff 7f           	andl	$0x7fffffff, %esi       # imm = 0x7FFFFFFF
141d86d51: 81 c6 00 00 80 ff           	addl	$0xff800000, %esi       # imm = 0xFF800000
141d86d57: 81 fe 00 00 00 7f           	cmpl	$0x7f000000, %esi       # imm = 0x7F000000
141d86d5d: 40 0f 92 c6                 	setb	%sil
141d86d61: 44 20 ee                    	andb	%r13b, %sil
141d86d64: ff cd                       	decl	%ebp
141d86d66: 81 fd ff ff 7f 00           	cmpl	$0x7fffff, %ebp         # imm = 0x7FFFFF
141d86d6c: 41 0f 92 c5                 	setb	%r13b
141d86d70: 41 08 f5                    	orb	%sil, %r13b
141d86d73: f3 44 0f 12 cf              	movsldup	%xmm7, %xmm9            # xmm9 = xmm7[0,0,2,2]
141d86d78: 45 0f 59 c8                 	mulps	%xmm8, %xmm9
141d86d7c: f3 0f 59 fe                 	mulss	%xmm6, %xmm7
141d86d80: f3 0f 59 f9                 	mulss	%xmm1, %xmm7
141d86d84: 44 0f 59 ca                 	mulps	%xmm2, %xmm9
141d86d88: 75 03                       	jne	0x141d86d8d <.text+0x1d85d8d>
141d86d8a: 0f 57 ff                    	xorps	%xmm7, %xmm7
141d86d8d: 49 ff c0                    	incq	%r8
141d86d90: 41 0f b6 f5                 	movzbl	%r13b, %esi
141d86d94: 66 0f 6e f6                 	movd	%esi, %xmm6
141d86d98: 66 0f 70 f6 50              	pshufd	$0x50, %xmm6, %xmm6     # xmm6 = xmm6[0,0,1,1]
141d86d9d: 66 0f 72 f6 1f              	pslld	$0x1f, %xmm6
141d86da2: 66 0f 72 e6 1f              	psrad	$0x1f, %xmm6
141d86da7: 66 41 0f db f1              	pand	%xmm9, %xmm6
141d86dac: 8b 6c 24 6c                 	movl	0x6c(%rsp), %ebp
141d86db0: 41 89 29                    	movl	%ebp, (%r9)
141d86db3: 66 0f d6 33                 	movq	%xmm6, (%rbx)
141d86db7: f3 0f 11 7b 08              	movss	%xmm7, 0x8(%rbx)
141d86dbc: f3 41 0f 10 74 24 10        	movss	0x10(%r12), %xmm6
141d86dc3: f3 0f 58 f0                 	addss	%xmm0, %xmm6
141d86dc7: 8b 74 24 4c                 	movl	0x4c(%rsp), %esi
141d86dcb: 89 32                       	movl	%esi, (%rdx)
141d86dcd: f3 0f 11 37                 	movss	%xmm6, (%rdi)
141d86dd1: 41 8b 34 24                 	movl	(%r12), %esi
141d86dd5: 4c 8d 25 3c 12 1e 01        	leaq	0x11e123c(%rip), %r12   # 0x142f68018
141d86ddc: 49 63 34 b4                 	movslq	(%r12,%rsi,4), %rsi
141d86de0: 4c 01 e6                    	addq	%r12, %rsi
141d86de3: ff e6                       	jmpq	*%rsi
141d86de5: 41 89 29                    	movl	%ebp, (%r9)
141d86de8: c7 43 08 00 00 00 00        	movl	$0x0, 0x8(%rbx)
141d86def: 48 c7 03 00 00 00 00        	movq	$0x0, (%rbx)
141d86df6: 44 8b 4c 24 4c              	movl	0x4c(%rsp), %r9d
141d86dfb: 44 89 0a                    	movl	%r9d, (%rdx)
141d86dfe: c7 07 00 00 00 00           	movl	$0x0, (%rdi)
141d86e04: e9 11 fb ff ff              	jmp	0x141d8691a <.text+0x1d8591a>
141d86e09: 41 0f 2e 67 14              	ucomiss	0x14(%r15), %xmm4
141d86e0e: 0f 86 06 fb ff ff           	jbe	0x141d8691a <.text+0x1d8591a>
141d86e14: f3 0f 10 73 04              	movss	0x4(%rbx), %xmm6
141d86e19: f3 0f 5f f5                 	maxss	%xmm5, %xmm6
141d86e1d: 41 89 29                    	movl	%ebp, (%r9)
141d86e20: f3 0f 11 73 04              	movss	%xmm6, 0x4(%rbx)
141d86e25: e9 f0 fa ff ff              	jmp	0x141d8691a <.text+0x1d8591a>
141d86e2a: 41 89 29                    	movl	%ebp, (%r9)
141d86e2d: c7 43 04 ae 47 61 3f        	movl	$0x3f6147ae, 0x4(%rbx)  # imm = 0x3F6147AE
141d86e34: e9 e1 fa ff ff              	jmp	0x141d8691a <.text+0x1d8591a>
141d86e39: 41 89 29                    	movl	%ebp, (%r9)
141d86e3c: f3 0f 10 33                 	movss	(%rbx), %xmm6
141d86e40: f3 0f 59 f3                 	mulss	%xmm3, %xmm6
141d86e44: f3 0f 11 33                 	movss	%xmm6, (%rbx)
141d86e48: f3 0f 10 73 08              	movss	0x8(%rbx), %xmm6
141d86e4d: f3 0f 59 f3                 	mulss	%xmm3, %xmm6
141d86e51: f3 0f 11 73 08              	movss	%xmm6, 0x8(%rbx)
141d86e56: 41 89 29                    	movl	%ebp, (%r9)
141d86e59: c7 43 04 ae 47 61 bf        	movl	$0xbf6147ae, 0x4(%rbx)  # imm = 0xBF6147AE
141d86e60: e9 b5 fa ff ff              	jmp	0x141d8691a <.text+0x1d8591a>
141d86e65: 0f 28 b4 24 40 01 00 00     	movaps	0x140(%rsp), %xmm6
141d86e6d: 0f 28 bc 24 50 01 00 00     	movaps	0x150(%rsp), %xmm7
141d86e75: 44 0f 28 84 24 60 01 00 00  	movaps	0x160(%rsp), %xmm8
141d86e7e: 44 0f 28 8c 24 70 01 00 00  	movaps	0x170(%rsp), %xmm9
141d86e87: 44 0f 28 94 24 80 01 00 00  	movaps	0x180(%rsp), %xmm10
141d86e90: 48 81 c4 98 01 00 00        	addq	$0x198, %rsp            # imm = 0x198
141d86e97: 5b                          	popq	%rbx
141d86e98: 5d                          	popq	%rbp
141d86e99: 5f                          	popq	%rdi
141d86e9a: 5e                          	popq	%rsi
141d86e9b: 41 5c                       	popq	%r12
141d86e9d: 41 5d                       	popq	%r13
141d86e9f: 41 5e                       	popq	%r14
141d86ea1: 41 5f                       	popq	%r15
141d86ea3: c3                          	retq
