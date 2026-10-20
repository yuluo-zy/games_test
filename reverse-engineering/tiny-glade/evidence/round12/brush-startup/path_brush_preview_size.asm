
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b1c010: 41 57                       	pushq	%r15
140b1c012: 41 56                       	pushq	%r14
140b1c014: 41 55                       	pushq	%r13
140b1c016: 41 54                       	pushq	%r12
140b1c018: 56                          	pushq	%rsi
140b1c019: 57                          	pushq	%rdi
140b1c01a: 55                          	pushq	%rbp
140b1c01b: 53                          	pushq	%rbx
140b1c01c: 48 83 ec 40                 	subq	$0x40, %rsp
140b1c020: 48 8b 01                    	movq	(%rcx), %rax
140b1c023: 8b 00                       	movl	(%rax), %eax
140b1c025: 83 c0 f9                    	addl	$-0x7, %eax
140b1c028: 83 f8 01                    	cmpl	$0x1, %eax
140b1c02b: 0f 87 2e 02 00 00           	ja	0x140b1c25f <.text+0xb1b25f>
140b1c031: 48 8b 1a                    	movq	(%rdx), %rbx
140b1c034: 48 8b 42 08                 	movq	0x8(%rdx), %rax
140b1c038: f2 0f 10 42 10              	movsd	0x10(%rdx), %xmm0
140b1c03d: 48 8b 93 f8 00 00 00        	movq	0xf8(%rbx), %rdx
140b1c044: 4c 8b 8b 00 01 00 00        	movq	0x100(%rbx), %r9
140b1c04b: 44 0f b6 93 08 01 00 00     	movzbl	0x108(%rbx), %r10d
140b1c053: 45 84 d2                    	testb	%r10b, %r10b
140b1c056: 0f 84 8c 00 00 00           	je	0x140b1c0e8 <.text+0xb1b0e8>
140b1c05c: 4c 8d 5c 24 10              	leaq	0x10(%rsp), %r11
140b1c061: 48 8d 74 24 18              	leaq	0x18(%rsp), %rsi
140b1c066: 48 8d 7c 24 08              	leaq	0x8(%rsp), %rdi
140b1c06b: 4e 8d 3c 8d 00 00 00 00     	leaq	(,%r9,4), %r15
140b1c073: 48 89 d1                    	movq	%rdx, %rcx
140b1c076: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b1c080: 4d 85 ff                    	testq	%r15, %r15
140b1c083: 0f 84 6c 01 00 00           	je	0x140b1c1f5 <.text+0xb1b1f5>
140b1c089: 44 8b 31                    	movl	(%rcx), %r14d
140b1c08c: 48 83 c1 04                 	addq	$0x4, %rcx
140b1c090: 4c 8b a0 a8 01 00 00        	movq	0x1a8(%rax), %r12
140b1c097: 4f 8d 2c f6                 	leaq	(%r14,%r14,8), %r13
140b1c09b: 4f 8b 74 ec 10              	movq	0x10(%r12,%r13,8), %r14
140b1c0a0: 49 83 c7 fc                 	addq	$-0x4, %r15
140b1c0a4: 4d 85 f6                    	testq	%r14, %r14
140b1c0a7: 74 d7                       	je	0x140b1c080 <.text+0xb1b080>
140b1c0a9: 4f 8d 3c ec                 	leaq	(%r12,%r13,8), %r15
140b1c0ad: 48 8b 9b 10 01 00 00        	movq	0x110(%rbx), %rbx
140b1c0b4: 4d 8b 67 18                 	movq	0x18(%r15), %r12
140b1c0b8: 4d 8b 7f 38                 	movq	0x38(%r15), %r15
140b1c0bc: 49 8b 1c df                 	movq	(%r15,%rbx,8), %rbx
140b1c0c0: 48 f7 d3                    	notq	%rbx
140b1c0c3: 4c 8d 2c 5b                 	leaq	(%rbx,%rbx,2), %r13
140b1c0c7: 49 c1 e5 04                 	shlq	$0x4, %r13
140b1c0cb: 4f 8b 7c 2c 28              	movq	0x28(%r12,%r13), %r15
140b1c0d0: 4b 8b 5c 2c 10              	movq	0x10(%r12,%r13), %rbx
140b1c0d5: 4f 8b 64 2c 20              	movq	0x20(%r12,%r13), %r12
140b1c0da: 48 89 1c 24                 	movq	%rbx, (%rsp)
140b1c0de: 4c 89 64 24 08              	movq	%r12, 0x8(%rsp)
140b1c0e3: e9 bc 00 00 00              	jmp	0x140b1c1a4 <.text+0xb1b1a4>
140b1c0e8: 4c 8d 5c 24 30              	leaq	0x30(%rsp), %r11
140b1c0ed: 48 8d 74 24 38              	leaq	0x38(%rsp), %rsi
140b1c0f2: 48 8d 7c 24 28              	leaq	0x28(%rsp), %rdi
140b1c0f7: 4e 8d 3c 8d 00 00 00 00     	leaq	(,%r9,4), %r15
140b1c0ff: 48 89 d1                    	movq	%rdx, %rcx
140b1c102: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b1c110: 4d 85 ff                    	testq	%r15, %r15
140b1c113: 0f 84 dc 00 00 00           	je	0x140b1c1f5 <.text+0xb1b1f5>
140b1c119: 44 8b 31                    	movl	(%rcx), %r14d
140b1c11c: 48 83 c1 04                 	addq	$0x4, %rcx
140b1c120: 4c 8b a0 00 01 00 00        	movq	0x100(%rax), %r12
140b1c127: 4f 8d 2c b6                 	leaq	(%r14,%r14,4), %r13
140b1c12b: 49 c1 e5 05                 	shlq	$0x5, %r13
140b1c12f: 4f 8b 74 2c 58              	movq	0x58(%r12,%r13), %r14
140b1c134: 49 83 c7 fc                 	addq	$-0x4, %r15
140b1c138: 4d 85 f6                    	testq	%r14, %r14
140b1c13b: 74 d3                       	je	0x140b1c110 <.text+0xb1b110>
140b1c13d: 4d 01 ec                    	addq	%r13, %r12
140b1c140: 45 8b bc 24 94 00 00 00     	movl	0x94(%r12), %r15d
140b1c148: 4c 8b a8 a8 01 00 00        	movq	0x1a8(%rax), %r13
140b1c14f: 4f 8d 3c ff                 	leaq	(%r15,%r15,8), %r15
140b1c153: 48 8b 9b 10 01 00 00        	movq	0x110(%rbx), %rbx
140b1c15a: 4b 8b 6c fd 18              	movq	0x18(%r13,%r15,8), %rbp
140b1c15f: 4f 8b 7c fd 38              	movq	0x38(%r13,%r15,8), %r15
140b1c164: 49 8b 1c df                 	movq	(%r15,%rbx,8), %rbx
140b1c168: 48 f7 d3                    	notq	%rbx
140b1c16b: 4d 8b 7c 24 50              	movq	0x50(%r12), %r15
140b1c170: 4c 8d 24 5b                 	leaq	(%rbx,%rbx,2), %r12
140b1c174: 49 c1 e4 04                 	shlq	$0x4, %r12
140b1c178: 45 8b 7f 08                 	movl	0x8(%r15), %r15d
140b1c17c: 4b 8d 1c 7f                 	leaq	(%r15,%r15,2), %rbx
140b1c180: 48 c1 e3 04                 	shlq	$0x4, %rbx
140b1c184: 4a 03 5c 25 10              	addq	0x10(%rbp,%r12), %rbx
140b1c189: 49 c1 e7 02                 	shlq	$0x2, %r15
140b1c18d: 4e 8b 6c 25 20              	movq	0x20(%rbp,%r12), %r13
140b1c192: 4d 01 fd                    	addq	%r15, %r13
140b1c195: 4e 03 7c 25 28              	addq	0x28(%rbp,%r12), %r15
140b1c19a: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140b1c19f: 4c 89 6c 24 28              	movq	%r13, 0x28(%rsp)
140b1c1a4: 4d 89 3b                    	movq	%r15, (%r11)
140b1c1a7: 0f 13 06                    	movlps	%xmm0, (%rsi)
140b1c1aa: 48 8b 77 08                 	movq	0x8(%rdi), %rsi
140b1c1ae: 44 8b 5f 14                 	movl	0x14(%rdi), %r11d
140b1c1b2: bf 01 00 00 00              	movl	$0x1, %edi
140b1c1b7: 4a 8d 14 8a                 	leaq	(%rdx,%r9,4), %rdx
140b1c1bb: 45 84 d2                    	testb	%r10b, %r10b
140b1c1be: 75 48                       	jne	0x140b1c208 <.text+0xb1b208>
140b1c1c0: 4c 39 f7                    	cmpq	%r14, %rdi
140b1c1c3: 0f 85 96 00 00 00           	jne	0x140b1c25f <.text+0xb1b25f>
140b1c1c9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b1c1d0: 48 39 d1                    	cmpq	%rdx, %rcx
140b1c1d3: 74 5c                       	je	0x140b1c231 <.text+0xb1b231>
140b1c1d5: 44 8b 09                    	movl	(%rcx), %r9d
140b1c1d8: 48 83 c1 04                 	addq	$0x4, %rcx
140b1c1dc: 4c 8b 90 00 01 00 00        	movq	0x100(%rax), %r10
140b1c1e3: 4f 8d 0c 89                 	leaq	(%r9,%r9,4), %r9
140b1c1e7: 49 c1 e1 05                 	shlq	$0x5, %r9
140b1c1eb: 4b 83 7c 0a 58 00           	cmpq	$0x0, 0x58(%r10,%r9)
140b1c1f1: 74 dd                       	je	0x140b1c1d0 <.text+0xb1b1d0>
140b1c1f3: eb 6a                       	jmp	0x140b1c25f <.text+0xb1b25f>
140b1c1f5: 45 31 db                    	xorl	%r11d, %r11d
140b1c1f8: 45 31 f6                    	xorl	%r14d, %r14d
140b1c1fb: 31 ff                       	xorl	%edi, %edi
140b1c1fd: 31 db                       	xorl	%ebx, %ebx
140b1c1ff: 4a 8d 14 8a                 	leaq	(%rdx,%r9,4), %rdx
140b1c203: 45 84 d2                    	testb	%r10b, %r10b
140b1c206: 74 b8                       	je	0x140b1c1c0 <.text+0xb1b1c0>
140b1c208: 4c 39 f7                    	cmpq	%r14, %rdi
140b1c20b: 75 52                       	jne	0x140b1c25f <.text+0xb1b25f>
140b1c20d: 0f 1f 00                    	nopl	(%rax)
140b1c210: 48 39 d1                    	cmpq	%rdx, %rcx
140b1c213: 74 1c                       	je	0x140b1c231 <.text+0xb1b231>
140b1c215: 44 8b 09                    	movl	(%rcx), %r9d
140b1c218: 48 83 c1 04                 	addq	$0x4, %rcx
140b1c21c: 4c 8b 90 a8 01 00 00        	movq	0x1a8(%rax), %r10
140b1c223: 4f 8d 0c c9                 	leaq	(%r9,%r9,8), %r9
140b1c227: 4b 83 7c ca 10 00           	cmpq	$0x0, 0x10(%r10,%r9,8)
140b1c22d: 74 e1                       	je	0x140b1c210 <.text+0xb1b210>
140b1c22f: eb 2e                       	jmp	0x140b1c25f <.text+0xb1b25f>
140b1c231: 48 85 db                    	testq	%rbx, %rbx
140b1c234: 74 29                       	je	0x140b1c25f <.text+0xb1b25f>
140b1c236: 49 8b 00                    	movq	(%r8), %rax
140b1c239: f3 0f 10 00                 	movss	(%rax), %xmm0
140b1c23d: f3 0f 59 05 bb 13 f7 01     	mulss	0x1f713bb(%rip), %xmm0  # 0x142a8d600
140b1c245: f3 0f 11 43 1c              	movss	%xmm0, 0x1c(%rbx)
140b1c24a: f3 0f 11 43 24              	movss	%xmm0, 0x24(%rbx)
140b1c24f: f3 0f 5f 05 ad 96 e0 01     	maxss	0x1e096ad(%rip), %xmm0  # 0x142925904
140b1c257: 44 89 1e                    	movl	%r11d, (%rsi)
140b1c25a: f3 0f 11 43 20              	movss	%xmm0, 0x20(%rbx)
140b1c25f: 48 83 c4 40                 	addq	$0x40, %rsp
140b1c263: 5b                          	popq	%rbx
140b1c264: 5d                          	popq	%rbp
140b1c265: 5f                          	popq	%rdi
140b1c266: 5e                          	popq	%rsi
140b1c267: 41 5c                       	popq	%r12
140b1c269: 41 5d                       	popq	%r13
140b1c26b: 41 5e                       	popq	%r14
140b1c26d: 41 5f                       	popq	%r15
140b1c26f: c3                          	retq
