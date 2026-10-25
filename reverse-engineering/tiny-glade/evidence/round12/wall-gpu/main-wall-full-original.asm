
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140757800: 55                          	pushq	%rbp
140757801: 41 57                       	pushq	%r15
140757803: 41 56                       	pushq	%r14
140757805: 41 55                       	pushq	%r13
140757807: 41 54                       	pushq	%r12
140757809: 56                          	pushq	%rsi
14075780a: 57                          	pushq	%rdi
14075780b: 53                          	pushq	%rbx
14075780c: 48 81 ec 68 05 00 00        	subq	$0x568, %rsp            # imm = 0x568
140757813: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14075781b: 44 0f 29 bd d0 04 00 00     	movaps	%xmm15, 0x4d0(%rbp)
140757823: 44 0f 29 b5 c0 04 00 00     	movaps	%xmm14, 0x4c0(%rbp)
14075782b: 66 44 0f 7f ad b0 04 00 00  	movdqa	%xmm13, 0x4b0(%rbp)
140757834: 66 44 0f 7f a5 a0 04 00 00  	movdqa	%xmm12, 0x4a0(%rbp)
14075783d: 66 44 0f 7f 9d 90 04 00 00  	movdqa	%xmm11, 0x490(%rbp)
140757846: 44 0f 29 95 80 04 00 00     	movaps	%xmm10, 0x480(%rbp)
14075784e: 44 0f 29 8d 70 04 00 00     	movaps	%xmm9, 0x470(%rbp)
140757856: 44 0f 29 85 60 04 00 00     	movaps	%xmm8, 0x460(%rbp)
14075785e: 0f 29 bd 50 04 00 00        	movaps	%xmm7, 0x450(%rbp)
140757865: 66 0f 7f b5 40 04 00 00     	movdqa	%xmm6, 0x440(%rbp)
14075786d: 48 c7 85 38 04 00 00 fe ff ff ff    	movq	$-0x2, 0x438(%rbp)
140757878: 4d 89 ce                    	movq	%r9, %r14
14075787b: 4c 89 85 18 04 00 00        	movq	%r8, 0x418(%rbp)
140757882: 48 89 95 c8 02 00 00        	movq	%rdx, 0x2c8(%rbp)
140757889: 48 8b 85 90 05 00 00        	movq	0x590(%rbp), %rax
140757890: 48 8b 11                    	movq	(%rcx), %rdx
140757893: 48 89 95 f8 03 00 00        	movq	%rdx, 0x3f8(%rbp)
14075789a: 48 8b 51 20                 	movq	0x20(%rcx), %rdx
14075789e: 48 89 95 a8 02 00 00        	movq	%rdx, 0x2a8(%rbp)
1407578a5: 48 8b 79 40                 	movq	0x40(%rcx), %rdi
1407578a9: 48 8b 59 48                 	movq	0x48(%rcx), %rbx
1407578ad: 48 8b 49 58                 	movq	0x58(%rcx), %rcx
1407578b1: 48 89 8d 20 02 00 00        	movq	%rcx, 0x220(%rbp)
1407578b8: 48 8b 08                    	movq	(%rax), %rcx
1407578bb: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1407578c2: f3 0f 6f 40 20              	movdqu	0x20(%rax), %xmm0
1407578c7: f3 0f 6f 48 30              	movdqu	0x30(%rax), %xmm1
1407578cc: f3 0f 6f 50 40              	movdqu	0x40(%rax), %xmm2
1407578d1: f3 0f 6f 58 50              	movdqu	0x50(%rax), %xmm3
1407578d6: 66 0f 7f 5d 60              	movdqa	%xmm3, 0x60(%rbp)
1407578db: 66 0f 7f 55 50              	movdqa	%xmm2, 0x50(%rbp)
1407578e0: 66 0f 7f 4d 40              	movdqa	%xmm1, 0x40(%rbp)
1407578e5: 66 0f 7f 45 30              	movdqa	%xmm0, 0x30(%rbp)
1407578ea: e8 11 f1 4b 00              	callq	0x140c16a00 <.text+0xc15a00>
1407578ef: 88 85 31 04 00 00           	movb	%al, 0x431(%rbp)
1407578f5: 84 c0                       	testb	%al, %al
1407578f7: 0f 84 0f 02 00 00           	je	0x140757b0c <.text+0x756b0c>
1407578fd: 41 bf 39 00 00 00           	movl	$0x39, %r15d
140757903: b9 39 00 00 00              	movl	$0x39, %ecx
140757908: e8 03 fa ff ff              	callq	0x140757310 <.text+0x756310>
14075790d: 4c 8d 25 e7 50 32 02        	leaq	0x23250e7(%rip), %r12   # 0x142a7c9fb
140757914: 48 83 f8 01                 	cmpq	$0x1, %rax
140757918: 0f 85 a1 00 00 00           	jne	0x1407579bf <.text+0x7569bf>
14075791e: 49 89 d1                    	movq	%rdx, %r9
140757921: 48 85 d2                    	testq	%rdx, %rdx
140757924: 74 36                       	je	0x14075795c <.text+0x75695c>
140757926: 49 83 f9 39                 	cmpq	$0x39, %r9
14075792a: 73 2e                       	jae	0x14075795a <.text+0x75695a>
14075792c: 48 8d 05 c8 50 32 02        	leaq	0x23250c8(%rip), %rax   # 0x142a7c9fb
140757933: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140757938: 7f 22                       	jg	0x14075795c <.text+0x75695c>
14075793a: 48 8d 05 6f 51 32 02        	leaq	0x232516f(%rip), %rax   # 0x142a7cab0
140757941: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140757946: 48 8d 0d ae 50 32 02        	leaq	0x23250ae(%rip), %rcx   # 0x142a7c9fb
14075794d: ba 39 00 00 00              	movl	$0x39, %edx
140757952: 45 31 c0                    	xorl	%r8d, %r8d
140757955: e8 e6 1f 18 02              	callq	0x1428d9940 <.text+0x28d8940>
14075795a: 75 de                       	jne	0x14075793a <.text+0x75693a>
14075795c: 4c 89 c9                    	movq	%r9, %rcx
14075795f: e8 ac f9 ff ff              	callq	0x140757310 <.text+0x756310>
140757964: 48 83 f8 01                 	cmpq	$0x1, %rax
140757968: 75 55                       	jne	0x1407579bf <.text+0x7569bf>
14075796a: 49 89 d0                    	movq	%rdx, %r8
14075796d: 49 83 c0 02                 	addq	$0x2, %r8
140757971: 74 39                       	je	0x1407579ac <.text+0x7569ac>
140757973: 49 83 f8 39                 	cmpq	$0x39, %r8
140757977: 73 31                       	jae	0x1407579aa <.text+0x7569aa>
140757979: 48 8d 05 7b 50 32 02        	leaq	0x232507b(%rip), %rax   # 0x142a7c9fb
140757980: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140757985: 7f 25                       	jg	0x1407579ac <.text+0x7569ac>
140757987: 48 8d 05 3a 51 32 02        	leaq	0x232513a(%rip), %rax   # 0x142a7cac8
14075798e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140757993: 48 8d 0d 61 50 32 02        	leaq	0x2325061(%rip), %rcx   # 0x142a7c9fb
14075799a: ba 39 00 00 00              	movl	$0x39, %edx
14075799f: 41 b9 39 00 00 00           	movl	$0x39, %r9d
1407579a5: e8 96 1f 18 02              	callq	0x1428d9940 <.text+0x28d8940>
1407579aa: 75 db                       	jne	0x140757987 <.text+0x756987>
1407579ac: 41 bf 37 00 00 00           	movl	$0x37, %r15d
1407579b2: 49 29 d7                    	subq	%rdx, %r15
1407579b5: 4c 8d 25 3f 50 32 02        	leaq	0x232503f(%rip), %r12   # 0x142a7c9fb
1407579bc: 4d 01 c4                    	addq	%r8, %r12
1407579bf: 4c 8d 05 30 52 32 02        	leaq	0x2325230(%rip), %r8    # 0x142a7cbf6
1407579c6: 4c 8d 2d f3 51 32 02        	leaq	0x23251f3(%rip), %r13   # 0x142a7cbc0
1407579cd: 0f 1f 00                    	nopl	(%rax)
1407579d0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407579d5: 85 c0                       	testl	%eax, %eax
1407579d7: 78 17                       	js	0x1407579f0 <.text+0x7569f0>
1407579d9: 49 ff c8                    	decq	%r8
1407579dc: 83 f8 5c                    	cmpl	$0x5c, %eax
1407579df: 75 63                       	jne	0x140757a44 <.text+0x756a44>
1407579e1: eb 72                       	jmp	0x140757a55 <.text+0x756a55>
1407579e3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407579f0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407579f5: 80 f9 c0                    	cmpb	$-0x40, %cl
1407579f8: 7d 23                       	jge	0x140757a1d <.text+0x756a1d>
1407579fa: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407579ff: 80 fa c0                    	cmpb	$-0x40, %dl
140757a02: 7d 22                       	jge	0x140757a26 <.text+0x756a26>
140757a04: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
140757a09: 49 83 c0 fc                 	addq	$-0x4, %r8
140757a0d: 41 83 e1 07                 	andl	$0x7, %r9d
140757a11: 41 c1 e1 06                 	shll	$0x6, %r9d
140757a15: 83 e2 3f                    	andl	$0x3f, %edx
140757a18: 44 09 ca                    	orl	%r9d, %edx
140757a1b: eb 10                       	jmp	0x140757a2d <.text+0x756a2d>
140757a1d: 49 83 c0 fe                 	addq	$-0x2, %r8
140757a21: 83 e1 1f                    	andl	$0x1f, %ecx
140757a24: eb 0f                       	jmp	0x140757a35 <.text+0x756a35>
140757a26: 49 83 c0 fd                 	addq	$-0x3, %r8
140757a2a: 83 e2 0f                    	andl	$0xf, %edx
140757a2d: c1 e2 06                    	shll	$0x6, %edx
140757a30: 83 e1 3f                    	andl	$0x3f, %ecx
140757a33: 09 d1                       	orl	%edx, %ecx
140757a35: c1 e1 06                    	shll	$0x6, %ecx
140757a38: 24 3f                       	andb	$0x3f, %al
140757a3a: 0f b6 c0                    	movzbl	%al, %eax
140757a3d: 09 c8                       	orl	%ecx, %eax
140757a3f: 83 f8 5c                    	cmpl	$0x5c, %eax
140757a42: 74 11                       	je	0x140757a55 <.text+0x756a55>
140757a44: 83 f8 2f                    	cmpl	$0x2f, %eax
140757a47: 74 0c                       	je	0x140757a55 <.text+0x756a55>
140757a49: 4d 39 e8                    	cmpq	%r13, %r8
140757a4c: 75 82                       	jne	0x1407579d0 <.text+0x7569d0>
140757a4e: be 36 00 00 00              	movl	$0x36, %esi
140757a53: eb 4c                       	jmp	0x140757aa1 <.text+0x756aa1>
140757a55: 4c 8d 2d 64 51 32 02        	leaq	0x2325164(%rip), %r13   # 0x142a7cbc0
140757a5c: 4d 29 e8                    	subq	%r13, %r8
140757a5f: 49 ff c0                    	incq	%r8
140757a62: 74 32                       	je	0x140757a96 <.text+0x756a96>
140757a64: 49 83 f8 36                 	cmpq	$0x36, %r8
140757a68: 73 2a                       	jae	0x140757a94 <.text+0x756a94>
140757a6a: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
140757a6f: 7f 25                       	jg	0x140757a96 <.text+0x756a96>
140757a71: 48 8d 05 18 50 32 02        	leaq	0x2325018(%rip), %rax   # 0x142a7ca90
140757a78: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140757a7d: 48 8d 0d 3c 51 32 02        	leaq	0x232513c(%rip), %rcx   # 0x142a7cbc0
140757a84: ba 36 00 00 00              	movl	$0x36, %edx
140757a89: 41 b9 36 00 00 00           	movl	$0x36, %r9d
140757a8f: e8 ac 1e 18 02              	callq	0x1428d9940 <.text+0x28d8940>
140757a94: 75 db                       	jne	0x140757a71 <.text+0x756a71>
140757a96: be 36 00 00 00              	movl	$0x36, %esi
140757a9b: 4c 29 c6                    	subq	%r8, %rsi
140757a9e: 4d 01 c5                    	addq	%r8, %r13
140757aa1: e8 8a 06 4c 00              	callq	0x140c18130 <.text+0xc17130>
140757aa6: 48 89 c1                    	movq	%rax, %rcx
140757aa9: 48 8b 00                    	movq	(%rax), %rax
140757aac: 48 83 f8 01                 	cmpq	$0x1, %rax
140757ab0: 0f 85 04 22 00 00           	jne	0x140759cba <.text+0x758cba>
140757ab6: 48 83 c1 08                 	addq	$0x8, %rcx
140757aba: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140757abe: 0f 85 f8 22 00 00           	jne	0x140759dbc <.text+0x758dbc>
140757ac4: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140757acb: 48 89 8d 20 04 00 00        	movq	%rcx, 0x420(%rbp)
140757ad2: 48 83 c1 08                 	addq	$0x8, %rcx
140757ad6: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
140757adb: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140757ae4: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140757aed: 4c 89 e2                    	movq	%r12, %rdx
140757af0: 4d 89 f8                    	movq	%r15, %r8
140757af3: 4d 89 e9                    	movq	%r13, %r9
140757af6: e8 45 f2 4b 00              	callq	0x140c16d40 <.text+0xc15d40>
140757afb: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140757b02: 48 8b 85 20 04 00 00        	movq	0x420(%rbp), %rax
140757b09: 48 ff 00                    	incq	(%rax)
140757b0c: 4c 89 b5 e8 03 00 00        	movq	%r14, 0x3e8(%rbp)
140757b13: 48 89 9d f8 00 00 00        	movq	%rbx, 0xf8(%rbp)
140757b1a: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
140757b21: 48 8b 8d 58 05 00 00        	movq	0x558(%rbp), %rcx
140757b28: 48 8b 01                    	movq	(%rcx), %rax
140757b2b: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
140757b2f: 4c 8b 10                    	movq	(%rax), %r10
140757b32: 45 31 c9                    	xorl	%r9d, %r9d
140757b35: 4d 89 d3                    	movq	%r10, %r11
140757b38: 4c 2b 5a 18                 	subq	0x18(%rdx), %r11
140757b3c: 4d 0f 42 d9                 	cmovbq	%r9, %r11
140757b40: 4c 2b 52 38                 	subq	0x38(%rdx), %r10
140757b44: 4d 0f 42 d1                 	cmovbq	%r9, %r10
140757b48: 4c 89 d9                    	movq	%r11, %rcx
140757b4b: 48 c1 e1 04                 	shlq	$0x4, %rcx
140757b4f: 48 03 4a 08                 	addq	0x8(%rdx), %rcx
140757b53: 4c 8b 42 10                 	movq	0x10(%rdx), %r8
140757b57: 4d 29 d8                    	subq	%r11, %r8
140757b5a: 4d 0f 42 c1                 	cmovbq	%r9, %r8
140757b5e: bf 08 00 00 00              	movl	$0x8, %edi
140757b63: 48 0f 42 cf                 	cmovbq	%rdi, %rcx
140757b67: 4d 89 d3                    	movq	%r10, %r11
140757b6a: 49 c1 e3 04                 	shlq	$0x4, %r11
140757b6e: 4c 03 5a 28                 	addq	0x28(%rdx), %r11
140757b72: 48 8b 72 30                 	movq	0x30(%rdx), %rsi
140757b76: 4c 29 d6                    	subq	%r10, %rsi
140757b79: 49 0f 42 f1                 	cmovbq	%r9, %rsi
140757b7d: 4c 0f 42 df                 	cmovbq	%rdi, %r11
140757b81: 4e 8d 0c 06                 	leaq	(%rsi,%r8), %r9
140757b85: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
140757b89: 4c 29 ca                    	subq	%r9, %rdx
140757b8c: 48 89 10                    	movq	%rdx, (%rax)
140757b8f: 49 c1 e0 04                 	shlq	$0x4, %r8
140757b93: 49 01 c8                    	addq	%rcx, %r8
140757b96: 48 c1 e6 04                 	shlq	$0x4, %rsi
140757b9a: 4c 01 de                    	addq	%r11, %rsi
140757b9d: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
140757ba4: 48 89 8d f8 02 00 00        	movq	%rcx, 0x2f8(%rbp)
140757bab: 4c 89 85 00 03 00 00        	movq	%r8, 0x300(%rbp)
140757bb2: 4c 89 9d 08 03 00 00        	movq	%r11, 0x308(%rbp)
140757bb9: 48 89 b5 10 03 00 00        	movq	%rsi, 0x310(%rbp)
140757bc0: 4c 89 8d 18 03 00 00        	movq	%r9, 0x318(%rbp)
140757bc7: 4c 8d 05 ca 4d 32 02        	leaq	0x2324dca(%rip), %r8    # 0x142a7c998
140757bce: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
140757bd5: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140757bdc: e8 ef ba aa 00              	callq	0x1412036d0 <.text+0x12026d0>
140757be1: 48 8b 85 60 05 00 00        	movq	0x560(%rbp), %rax
140757be8: 48 8b 18                    	movq	(%rax), %rbx
140757beb: 48 8b 40 08                 	movq	0x8(%rax), %rax
140757bef: 48 8b 13                    	movq	(%rbx), %rdx
140757bf2: 31 c9                       	xorl	%ecx, %ecx
140757bf4: 49 89 d0                    	movq	%rdx, %r8
140757bf7: 4c 2b 40 18                 	subq	0x18(%rax), %r8
140757bfb: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
140757bff: 48 2b 50 38                 	subq	0x38(%rax), %rdx
140757c03: 48 0f 42 d1                 	cmovbq	%rcx, %rdx
140757c07: 4d 89 c7                    	movq	%r8, %r15
140757c0a: 49 c1 e7 04                 	shlq	$0x4, %r15
140757c0e: 4c 03 78 08                 	addq	0x8(%rax), %r15
140757c12: 4c 8b 48 10                 	movq	0x10(%rax), %r9
140757c16: 4d 29 c1                    	subq	%r8, %r9
140757c19: 4c 0f 42 c9                 	cmovbq	%rcx, %r9
140757c1d: 4c 0f 42 ff                 	cmovbq	%rdi, %r15
140757c21: 49 89 d6                    	movq	%rdx, %r14
140757c24: 49 c1 e6 04                 	shlq	$0x4, %r14
140757c28: 4c 03 70 28                 	addq	0x28(%rax), %r14
140757c2c: 4c 8b 60 30                 	movq	0x30(%rax), %r12
140757c30: 49 29 d4                    	subq	%rdx, %r12
140757c33: 4c 0f 42 e1                 	cmovbq	%rcx, %r12
140757c37: 4c 0f 42 f7                 	cmovbq	%rdi, %r14
140757c3b: 48 8b 40 40                 	movq	0x40(%rax), %rax
140757c3f: 4c 29 e0                    	subq	%r12, %rax
140757c42: 4c 29 c8                    	subq	%r9, %rax
140757c45: 48 89 03                    	movq	%rax, (%rbx)
140757c48: 49 c1 e1 04                 	shlq	$0x4, %r9
140757c4c: 4d 01 f9                    	addq	%r15, %r9
140757c4f: 4c 89 8d 20 04 00 00        	movq	%r9, 0x420(%rbp)
140757c56: 49 c1 e4 04                 	shlq	$0x4, %r12
140757c5a: 4d 01 f4                    	addq	%r14, %r12
140757c5d: eb 16                       	jmp	0x140757c75 <.text+0x756c75>
140757c5f: 90                          	nop
140757c60: 48 8b 85 b8 03 00 00        	movq	0x3b8(%rbp), %rax
140757c67: 4c 89 2c f8                 	movq	%r13, (%rax,%rdi,8)
140757c6b: 48 ff c7                    	incq	%rdi
140757c6e: 48 89 bd c0 03 00 00        	movq	%rdi, 0x3c0(%rbp)
140757c75: 4c 89 f1                    	movq	%r14, %rcx
140757c78: 4c 89 fa                    	movq	%r15, %rdx
140757c7b: 48 85 d2                    	testq	%rdx, %rdx
140757c7e: 75 20                       	jne	0x140757ca0 <.text+0x756ca0>
140757c80: 48 85 c9                    	testq	%rcx, %rcx
140757c83: 0f 84 ab 00 00 00           	je	0x140757d34 <.text+0x756d34>
140757c89: 4c 39 e1                    	cmpq	%r12, %rcx
140757c8c: 0f 84 a2 00 00 00           	je	0x140757d34 <.text+0x756d34>
140757c92: 4c 8d 71 10                 	leaq	0x10(%rcx), %r14
140757c96: 4c 89 f7                    	movq	%r14, %rdi
140757c99: 31 f6                       	xorl	%esi, %esi
140757c9b: 48 89 ca                    	movq	%rcx, %rdx
140757c9e: eb 13                       	jmp	0x140757cb3 <.text+0x756cb3>
140757ca0: 48 3b 95 20 04 00 00        	cmpq	0x420(%rbp), %rdx
140757ca7: 74 3b                       	je	0x140757ce4 <.text+0x756ce4>
140757ca9: 4c 8d 7a 10                 	leaq	0x10(%rdx), %r15
140757cad: 48 89 cf                    	movq	%rcx, %rdi
140757cb0: 4c 89 fe                    	movq	%r15, %rsi
140757cb3: 48 ff 03                    	incq	(%rbx)
140757cb6: 4c 8b 6a 08                 	movq	0x8(%rdx), %r13
140757cba: c6 85 33 04 00 00 01        	movb	$0x1, 0x433(%rbp)
140757cc1: 48 8b 8d f8 03 00 00        	movq	0x3f8(%rbp), %rcx
140757cc8: 4c 89 ea                    	movq	%r13, %rdx
140757ccb: 4c 8d 05 76 47 4c 02        	leaq	0x24c4776(%rip), %r8    # 0x142c1c448
140757cd2: e8 19 a2 33 00              	callq	0x140a91ef0 <.text+0xa90ef0>
140757cd7: 48 89 f9                    	movq	%rdi, %rcx
140757cda: 48 89 f2                    	movq	%rsi, %rdx
140757cdd: 48 85 c0                    	testq	%rax, %rax
140757ce0: 75 0e                       	jne	0x140757cf0 <.text+0x756cf0>
140757ce2: eb 97                       	jmp	0x140757c7b <.text+0x756c7b>
140757ce4: 45 31 ff                    	xorl	%r15d, %r15d
140757ce7: 48 85 c9                    	testq	%rcx, %rcx
140757cea: 75 9d                       	jne	0x140757c89 <.text+0x756c89>
140757cec: eb 46                       	jmp	0x140757d34 <.text+0x756d34>
140757cee: 66 90                       	nop
140757cf0: 48 8b bd c0 03 00 00        	movq	0x3c0(%rbp), %rdi
140757cf7: 48 3b bd b0 03 00 00        	cmpq	0x3b0(%rbp), %rdi
140757cfe: 0f 85 5c ff ff ff           	jne	0x140757c60 <.text+0x756c60>
140757d04: c6 85 33 04 00 00 01        	movb	$0x1, 0x433(%rbp)
140757d0b: 48 c7 44 24 20 08 00 00 00  	movq	$0x8, 0x20(%rsp)
140757d14: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140757d1a: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140757d20: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
140757d27: 48 89 fa                    	movq	%rdi, %rdx
140757d2a: e8 b1 00 15 02              	callq	0x1428a7de0 <.text+0x28a6de0>
140757d2f: e9 2c ff ff ff              	jmp	0x140757c60 <.text+0x756c60>
140757d34: 48 8b 95 18 04 00 00        	movq	0x418(%rbp), %rdx
140757d3b: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140757d3e: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140757d42: 89 85 94 03 00 00           	movl	%eax, 0x394(%rbp)
140757d48: 48 89 8d 10 02 00 00        	movq	%rcx, 0x210(%rbp)
140757d4f: 89 01                       	movl	%eax, (%rcx)
140757d51: 48 8b 02                    	movq	(%rdx), %rax
140757d54: 48 8b 38                    	movq	(%rax), %rdi
140757d57: 48 89 85 10 04 00 00        	movq	%rax, 0x410(%rbp)
140757d5e: 48 8b 58 18                 	movq	0x18(%rax), %rbx
140757d62: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
140757d66: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140757d6a: f7 d1                       	notl	%ecx
140757d6c: 4c 8d 7f 10                 	leaq	0x10(%rdi), %r15
140757d70: 4c 8d 35 e1 4e 32 02        	leaq	0x2324ee1(%rip), %r14   # 0x142a7cc58
140757d77: eb 1e                       	jmp	0x140757d97 <.text+0x756d97>
140757d79: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140757d80: 48 8b 85 b8 03 00 00        	movq	0x3b8(%rbp), %rax
140757d87: 4e 89 2c e0                 	movq	%r13, (%rax,%r12,8)
140757d8b: 49 ff c4                    	incq	%r12
140757d8e: 4c 89 a5 c0 03 00 00        	movq	%r12, 0x3c0(%rbp)
140757d95: 89 f1                       	movl	%esi, %ecx
140757d97: 48 85 db                    	testq	%rbx, %rbx
140757d9a: 0f 84 d0 00 00 00           	je	0x140757e70 <.text+0x756e70>
140757da0: 66 85 c9                    	testw	%cx, %cx
140757da3: 74 0b                       	je	0x140757db0 <.text+0x756db0>
140757da5: 8d 71 ff                    	leal	-0x1(%rcx), %esi
140757da8: 21 ce                       	andl	%ecx, %esi
140757daa: eb 2d                       	jmp	0x140757dd9 <.text+0x756dd9>
140757dac: 0f 1f 40 00                 	nopl	(%rax)
140757db0: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
140757db5: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140757db9: 48 81 c7 80 f9 ff ff        	addq	$-0x680, %rdi           # imm = 0xF980
140757dc0: 49 83 c7 10                 	addq	$0x10, %r15
140757dc4: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140757dca: 74 e4                       	je	0x140757db0 <.text+0x756db0>
140757dcc: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140757dd1: 29 c8                       	subl	%ecx, %eax
140757dd3: f7 d1                       	notl	%ecx
140757dd5: 89 ce                       	movl	%ecx, %esi
140757dd7: 21 c6                       	andl	%eax, %esi
140757dd9: f3 0f bc c1                 	tzcntl	%ecx, %eax
140757ddd: 48 f7 d8                    	negq	%rax
140757de0: 4c 6b e0 68                 	imulq	$0x68, %rax, %r12
140757de4: 4e 8b 6c 27 98              	movq	-0x68(%rdi,%r12), %r13
140757de9: c6 85 33 04 00 00 01        	movb	$0x1, 0x433(%rbp)
140757df0: 48 8b 8d f8 03 00 00        	movq	0x3f8(%rbp), %rcx
140757df7: 4c 89 ea                    	movq	%r13, %rdx
140757dfa: 4d 89 f0                    	movq	%r14, %r8
140757dfd: e8 ee a0 33 00              	callq	0x140a91ef0 <.text+0xa90ef0>
140757e02: 48 ff cb                    	decq	%rbx
140757e05: 89 f1                       	movl	%esi, %ecx
140757e07: 48 85 c0                    	testq	%rax, %rax
140757e0a: 74 8b                       	je	0x140757d97 <.text+0x756d97>
140757e0c: 4a 8d 0c 27                 	leaq	(%rdi,%r12), %rcx
140757e10: 48 83 c1 a0                 	addq	$-0x60, %rcx
140757e14: 66 0f 6e 88 8c 02 00 00     	movd	0x28c(%rax), %xmm1
140757e1c: 83 b8 58 02 00 00 02        	cmpl	$0x2, 0x258(%rax)
140757e23: 41 0f 95 c0                 	setne	%r8b
140757e27: c6 85 33 04 00 00 01        	movb	$0x1, 0x433(%rbp)
140757e2e: e8 cd f4 82 00              	callq	0x140f87300 <.text+0xf86300>
140757e33: 89 f1                       	movl	%esi, %ecx
140757e35: 84 c0                       	testb	%al, %al
140757e37: 0f 84 5a ff ff ff           	je	0x140757d97 <.text+0x756d97>
140757e3d: 4c 8b a5 c0 03 00 00        	movq	0x3c0(%rbp), %r12
140757e44: 4c 3b a5 b0 03 00 00        	cmpq	0x3b0(%rbp), %r12
140757e4b: 0f 85 2f ff ff ff           	jne	0x140757d80 <.text+0x756d80>
140757e51: c6 85 33 04 00 00 01        	movb	$0x1, 0x433(%rbp)
140757e58: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
140757e5f: 48 8d 15 0a 4e 32 02        	leaq	0x2324e0a(%rip), %rdx   # 0x142a7cc70
140757e66: e8 05 aa e0 ff              	callq	0x140562870 <.text+0x561870>
140757e6b: e9 10 ff ff ff              	jmp	0x140757d80 <.text+0x756d80>
140757e70: 48 8b 85 b8 03 00 00        	movq	0x3b8(%rbp), %rax
140757e77: 48 89 85 08 04 00 00        	movq	%rax, 0x408(%rbp)
140757e7e: 48 8b 95 c0 03 00 00        	movq	0x3c0(%rbp), %rdx
140757e85: 48 83 fa 01                 	cmpq	$0x1, %rdx
140757e89: 4c 8b a5 f8 03 00 00        	movq	0x3f8(%rbp), %r12
140757e90: 0f 87 88 1e 00 00           	ja	0x140759d1e <.text+0x758d1e>
140757e96: 4c 8b 95 e8 03 00 00        	movq	0x3e8(%rbp), %r10
140757e9d: 48 8b 8d 88 05 00 00        	movq	0x588(%rbp), %rcx
140757ea4: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
140757eaf: 48 c7 85 b8 01 00 00 04 00 00 00    	movq	$0x4, 0x1b8(%rbp)
140757eba: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
140757ec5: 48 8b 85 08 04 00 00        	movq	0x408(%rbp), %rax
140757ecc: 48 8d 1c d0                 	leaq	(%rax,%rdx,8), %rbx
140757ed0: 48 8b 09                    	movq	(%rcx), %rcx
140757ed3: 48 89 8d d0 02 00 00        	movq	%rcx, 0x2d0(%rbp)
140757eda: 48 8b 95 c8 02 00 00        	movq	0x2c8(%rbp), %rdx
140757ee1: 48 8b 0a                    	movq	(%rdx), %rcx
140757ee4: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
140757eeb: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140757eef: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140757ef6: 4c 8b 85 50 05 00 00        	movq	0x550(%rbp), %r8
140757efd: 49 8b 08                    	movq	(%r8), %rcx
140757f00: 48 89 8d c8 00 00 00        	movq	%rcx, 0xc8(%rbp)
140757f07: 49 8b 48 08                 	movq	0x8(%r8), %rcx
140757f0b: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
140757f12: 41 8b 48 14                 	movl	0x14(%r8), %ecx
140757f16: 89 8d 8c 03 00 00           	movl	%ecx, 0x38c(%rbp)
140757f1c: 8b 4a 1c                    	movl	0x1c(%rdx), %ecx
140757f1f: 89 8d a4 02 00 00           	movl	%ecx, 0x2a4(%rbp)
140757f25: 4c 8b 85 78 05 00 00        	movq	0x578(%rbp), %r8
140757f2c: 41 8b 48 1c                 	movl	0x1c(%r8), %ecx
140757f30: 89 8d 9c 02 00 00           	movl	%ecx, 0x29c(%rbp)
140757f36: 49 8b 10                    	movq	(%r8), %rdx
140757f39: 49 8b 48 10                 	movq	0x10(%r8), %rcx
140757f3d: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
140757f44: 4c 8b 8d 80 05 00 00        	movq	0x580(%rbp), %r9
140757f4b: 41 8b 49 1c                 	movl	0x1c(%r9), %ecx
140757f4f: 89 8d a0 02 00 00           	movl	%ecx, 0x2a0(%rbp)
140757f55: 4d 8b 01                    	movq	(%r9), %r8
140757f58: 49 8b 49 10                 	movq	0x10(%r9), %rcx
140757f5c: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
140757f63: 41 8b 4a 1c                 	movl	0x1c(%r10), %ecx
140757f67: 89 8d 88 03 00 00           	movl	%ecx, 0x388(%rbp)
140757f6d: 49 8b 0a                    	movq	(%r10), %rcx
140757f70: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
140757f77: 49 8b 4a 10                 	movq	0x10(%r10), %rcx
140757f7b: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
140757f82: 4c 8b 95 68 05 00 00        	movq	0x568(%rbp), %r10
140757f89: 41 8b 4a 1c                 	movl	0x1c(%r10), %ecx
140757f8d: 89 8d 90 03 00 00           	movl	%ecx, 0x390(%rbp)
140757f93: 4d 8b 0a                    	movq	(%r10), %r9
140757f96: 49 8b 4a 10                 	movq	0x10(%r10), %rcx
140757f9a: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
140757fa1: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140757fa5: 0f 57 ff                    	xorps	%xmm7, %xmm7
140757fa8: f3 44 0f 10 3d bf 48 32 02  	movss	0x23248bf(%rip), %xmm15 # 0x142a7c870
140757fb1: f3 44 0f 10 05 b6 d8 1c 02  	movss	0x21cd8b6(%rip), %xmm8  # 0x142925870
140757fba: 44 0f 28 35 9e 48 32 02     	movaps	0x232489e(%rip), %xmm14 # 0x142a7c860
140757fc2: 44 0f 28 15 96 2c 1e 02     	movaps	0x21e2c96(%rip), %xmm10 # 0x14293ac60
140757fca: 48 8b 8d b0 03 00 00        	movq	0x3b0(%rbp), %rcx
140757fd1: 48 89 8d 08 02 00 00        	movq	%rcx, 0x208(%rbp)
140757fd8: 4d 8d 74 24 20              	leaq	0x20(%r12), %r14
140757fdd: 48 89 95 c8 02 00 00        	movq	%rdx, 0x2c8(%rbp)
140757fe4: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
140757fe8: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
140757fef: 48 8b 8d 10 04 00 00        	movq	0x410(%rbp), %rcx
140757ff6: 48 8d 49 20                 	leaq	0x20(%rcx), %rcx
140757ffa: 48 89 8d d8 02 00 00        	movq	%rcx, 0x2d8(%rbp)
140758001: 4c 89 85 f0 01 00 00        	movq	%r8, 0x1f0(%rbp)
140758008: 49 8d 48 20                 	leaq	0x20(%r8), %rcx
14075800c: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
140758013: 4c 89 8d e8 03 00 00        	movq	%r9, 0x3e8(%rbp)
14075801a: 49 8d 49 20                 	leaq	0x20(%r9), %rcx
14075801e: 48 89 8d 18 02 00 00        	movq	%rcx, 0x218(%rbp)
140758025: 49 89 c7                    	movq	%rax, %r15
140758028: 48 89 85 a8 03 00 00        	movq	%rax, 0x3a8(%rbp)
14075802f: 48 89 9d 00 02 00 00        	movq	%rbx, 0x200(%rbp)
140758036: 4c 89 b5 e8 01 00 00        	movq	%r14, 0x1e8(%rbp)
14075803d: 49 39 df                    	cmpq	%rbx, %r15
140758040: 0f 84 7a 1b 00 00           	je	0x140759bc0 <.text+0x758bc0>
140758046: 49 8b 37                    	movq	(%r15), %rsi
140758049: 48 89 b5 28 04 00 00        	movq	%rsi, 0x428(%rbp)
140758050: 49 83 7c 24 18 00           	cmpq	$0x0, 0x18(%r12)
140758056: 0f 84 53 1b 00 00           	je	0x140759baf <.text+0x758baf>
14075805c: 49 83 c7 08                 	addq	$0x8, %r15
140758060: 4c 89 f1                    	movq	%r14, %rcx
140758063: 48 8d 95 28 04 00 00        	leaq	0x428(%rbp), %rdx
14075806a: e8 01 8d e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14075806f: 49 8b 3c 24                 	movq	(%r12), %rdi
140758073: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
140758078: 48 89 ca                    	movq	%rcx, %rdx
14075807b: 48 21 c2                    	andq	%rax, %rdx
14075807e: 48 c1 e8 39                 	shrq	$0x39, %rax
140758082: 66 0f 6e c0                 	movd	%eax, %xmm0
140758086: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14075808a: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14075808f: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140758094: 48 8d 87 58 fd ff ff        	leaq	-0x2a8(%rdi), %rax
14075809b: 45 31 c0                    	xorl	%r8d, %r8d
14075809e: f3 0f 6f 0c 17              	movdqu	(%rdi,%rdx), %xmm1
1407580a3: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407580a7: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407580ab: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407580b0: 45 85 c9                    	testl	%r9d, %r9d
1407580b3: 74 2b                       	je	0x1407580e0 <.text+0x7570e0>
1407580b5: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407580ba: 49 01 d2                    	addq	%rdx, %r10
1407580bd: 49 21 ca                    	andq	%rcx, %r10
1407580c0: 49 f7 da                    	negq	%r10
1407580c3: 49 69 da a8 02 00 00        	imulq	$0x2a8, %r10, %rbx      # imm = 0x2A8
1407580ca: 48 3b 34 18                 	cmpq	(%rax,%rbx), %rsi
1407580ce: 74 40                       	je	0x140758110 <.text+0x757110>
1407580d0: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407580d4: 66 45 21 ca                 	andw	%r9w, %r10w
1407580d8: 45 89 d1                    	movl	%r10d, %r9d
1407580db: 75 d8                       	jne	0x1407580b5 <.text+0x7570b5>
1407580dd: 0f 1f 00                    	nopl	(%rax)
1407580e0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1407580e4: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407580e9: 45 85 c9                    	testl	%r9d, %r9d
1407580ec: 0f 85 bd 1a 00 00           	jne	0x140759baf <.text+0x758baf>
1407580f2: 4c 01 c2                    	addq	%r8, %rdx
1407580f5: 48 83 c2 10                 	addq	$0x10, %rdx
1407580f9: 49 83 c0 10                 	addq	$0x10, %r8
1407580fd: 48 21 ca                    	andq	%rcx, %rdx
140758100: eb 9c                       	jmp	0x14075809e <.text+0x75709e>
140758102: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140758110: 4c 89 bd d0 01 00 00        	movq	%r15, 0x1d0(%rbp)
140758117: 48 c7 85 b0 02 00 00 00 00 00 00    	movq	$0x0, 0x2b0(%rbp)
140758122: 48 c7 85 b8 02 00 00 04 00 00 00    	movq	$0x4, 0x2b8(%rbp)
14075812d: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
140758138: 8b 85 9c 02 00 00           	movl	0x29c(%rbp), %eax
14075813e: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
140758145: 89 01                       	movl	%eax, (%rcx)
140758147: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
14075814e: 4c 8b 70 30                 	movq	0x30(%rax), %r14
140758152: 4c 8b 78 40                 	movq	0x40(%rax), %r15
140758156: 4c 3b 70 20                 	cmpq	0x20(%rax), %r14
14075815a: 75 1a                       	jne	0x140758176 <.text+0x757176>
14075815c: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140758163: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
14075816a: 48 8d 15 57 43 4c 02        	leaq	0x24c4357(%rip), %rdx   # 0x142c1c4c8
140758171: e8 ba a4 e0 ff              	callq	0x140562630 <.text+0x561630>
140758176: 48 8b 95 c8 02 00 00        	movq	0x2c8(%rbp), %rdx
14075817d: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140758181: 4c 89 f1                    	movq	%r14, %rcx
140758184: 48 c1 e1 04                 	shlq	$0x4, %rcx
140758188: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
14075818c: 48 89 74 08 08              	movq	%rsi, 0x8(%rax,%rcx)
140758191: 49 ff c6                    	incq	%r14
140758194: 4c 89 72 30                 	movq	%r14, 0x30(%rdx)
140758198: 48 ff 42 40                 	incq	0x40(%rdx)
14075819c: 48 8d 34 1f                 	leaq	(%rdi,%rbx), %rsi
1407581a0: 48 8d 04 1f                 	leaq	(%rdi,%rbx), %rax
1407581a4: 48 05 60 fd ff ff           	addq	$-0x2a0, %rax           # imm = 0xFD60
1407581aa: 48 89 85 20 04 00 00        	movq	%rax, 0x420(%rbp)
1407581b1: 80 7e f6 00                 	cmpb	$0x0, -0xa(%rsi)
1407581b5: 0f 85 b3 11 00 00           	jne	0x14075936e <.text+0x75836e>
1407581bb: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407581c2: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
1407581c9: 4c 89 e1                    	movq	%r12, %rcx
1407581cc: e8 df a0 33 00              	callq	0x140a922b0 <.text+0xa912b0>
1407581d1: 48 89 c2                    	movq	%rax, %rdx
1407581d4: 48 c1 ea 20                 	shrq	$0x20, %rdx
1407581d8: 48 89 b5 18 04 00 00        	movq	%rsi, 0x418(%rbp)
1407581df: 75 34                       	jne	0x140758215 <.text+0x757215>
1407581e1: 48 8d 4e f5                 	leaq	-0xb(%rsi), %rcx
1407581e5: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
1407581ec: e8 3f b0 0d 00              	callq	0x140833230 <.text+0x832230>
1407581f1: 48 89 85 f0 03 00 00        	movq	%rax, 0x3f0(%rbp)
1407581f8: 48 8b 85 f0 03 00 00        	movq	0x3f0(%rbp), %rax
1407581ff: 00 c0                       	addb	%al, %al
140758201: 48 89 85 f0 03 00 00        	movq	%rax, 0x3f0(%rbp)
140758208: 45 31 ed                    	xorl	%r13d, %r13d
14075820b: 45 31 e4                    	xorl	%r12d, %r12d
14075820e: 31 ff                       	xorl	%edi, %edi
140758210: e9 65 01 00 00              	jmp	0x14075837a <.text+0x75737a>
140758215: 41 89 c1                    	movl	%eax, %r9d
140758218: 48 8b b5 f8 00 00 00        	movq	0xf8(%rbp), %rsi
14075821f: 4c 3b 4e 10                 	cmpq	0x10(%rsi), %r9
140758223: 0f 83 c6 1b 00 00           	jae	0x140759def <.text+0x758def>
140758229: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
14075822d: 48 89 c2                    	movq	%rax, %rdx
140758230: 48 c1 ea 20                 	shrq	$0x20, %rdx
140758234: 4f 8d 04 89                 	leaq	(%r9,%r9,4), %r8
140758238: 42 39 14 81                 	cmpl	%edx, (%rcx,%r8,4)
14075823c: 4c 8b 9d 00 01 00 00        	movq	0x100(%rbp), %r11
140758243: 0f 85 a6 1b 00 00           	jne	0x140759def <.text+0x758def>
140758249: 4a 8d 0c 81                 	leaq	(%rcx,%r8,4), %rcx
14075824d: 44 8b 41 04                 	movl	0x4(%rcx), %r8d
140758251: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
140758257: 4d 39 d0                    	cmpq	%r10, %r8
14075825a: 0f 84 8f 1b 00 00           	je	0x140759def <.text+0x758def>
140758260: 45 31 c9                    	xorl	%r9d, %r9d
140758263: 4d 39 43 38                 	cmpq	%r8, 0x38(%r11)
140758267: 0f 86 92 1b 00 00           	jbe	0x140759dff <.text+0x758dff>
14075826d: 49 8b 53 28                 	movq	0x28(%r11), %rdx
140758271: 45 89 c2                    	movl	%r8d, %r10d
140758274: 41 c1 ea 06                 	shrl	$0x6, %r10d
140758278: 4a 8b 14 d2                 	movq	(%rdx,%r10,8), %rdx
14075827c: 4c 0f a3 c2                 	btq	%r8, %rdx
140758280: 0f 83 79 1b 00 00           	jae	0x140759dff <.text+0x758dff>
140758286: 8b 41 0c                    	movl	0xc(%rcx), %eax
140758289: 48 8b 96 a8 01 00 00        	movq	0x1a8(%rsi), %rdx
140758290: 4c 8d 04 c0                 	leaq	(%rax,%rax,8), %r8
140758294: 49 8b 83 10 01 00 00        	movq	0x110(%r11), %rax
14075829b: 4a 39 44 c2 40              	cmpq	%rax, 0x40(%rdx,%r8,8)
1407582a0: 76 2e                       	jbe	0x1407582d0 <.text+0x7572d0>
1407582a2: 4a 8d 14 c2                 	leaq	(%rdx,%r8,8), %rdx
1407582a6: 4c 8b 42 38                 	movq	0x38(%rdx), %r8
1407582aa: 49 8b 04 c0                 	movq	(%r8,%rax,8), %rax
1407582ae: 48 85 c0                    	testq	%rax, %rax
1407582b1: 48 8b b5 18 04 00 00        	movq	0x418(%rbp), %rsi
1407582b8: 74 21                       	je	0x1407582db <.text+0x7572db>
1407582ba: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
1407582be: 48 f7 d0                    	notq	%rax
1407582c1: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407582c5: 48 c1 e0 04                 	shlq	$0x4, %rax
1407582c9: 48 8b 44 02 10              	movq	0x10(%rdx,%rax), %rax
1407582ce: eb 0d                       	jmp	0x1407582dd <.text+0x7572dd>
1407582d0: 31 c0                       	xorl	%eax, %eax
1407582d2: 48 8b b5 18 04 00 00        	movq	0x418(%rbp), %rsi
1407582d9: eb 02                       	jmp	0x1407582dd <.text+0x7572dd>
1407582db: 31 c0                       	xorl	%eax, %eax
1407582dd: 8b 49 10                    	movl	0x10(%rcx), %ecx
1407582e0: 48 6b f9 58                 	imulq	$0x58, %rcx, %rdi
1407582e4: 48 01 c7                    	addq	%rax, %rdi
1407582e7: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
1407582ee: 48 89 f9                    	movq	%rdi, %rcx
1407582f1: e8 ba b0 18 00              	callq	0x1408e33b0 <.text+0x8e23b0>
1407582f6: 41 89 c5                    	movl	%eax, %r13d
1407582f9: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140758300: 48 89 f9                    	movq	%rdi, %rcx
140758303: e8 48 af 18 00              	callq	0x1408e3250 <.text+0x8e2250>
140758308: 84 c0                       	testb	%al, %al
14075830a: 75 31                       	jne	0x14075833d <.text+0x75733d>
14075830c: 48 8d 4f 3d                 	leaq	0x3d(%rdi), %rcx
140758310: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140758317: e8 14 af 0d 00              	callq	0x140833230 <.text+0x832230>
14075831c: 84 c0                       	testb	%al, %al
14075831e: 74 1d                       	je	0x14075833d <.text+0x75733d>
140758320: 48 8d 4e f9                 	leaq	-0x7(%rsi), %rcx
140758324: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
14075832b: e8 40 d3 16 00              	callq	0x1408c5670 <.text+0x8c4670>
140758330: 00 c0                       	addb	%al, %al
140758332: fe c0                       	incb	%al
140758334: 48 89 85 f0 03 00 00        	movq	%rax, 0x3f0(%rbp)
14075833b: eb 0b                       	jmp	0x140758348 <.text+0x757348>
14075833d: 48 c7 85 f0 03 00 00 00 00 00 00    	movq	$0x0, 0x3f0(%rbp)
140758348: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
14075834f: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140758356: 48 89 fa                    	movq	%rdi, %rdx
140758359: e8 82 b0 18 00              	callq	0x1408e33e0 <.text+0x8e23e0>
14075835e: 4c 8b a5 f0 02 00 00        	movq	0x2f0(%rbp), %r12
140758365: 4c 8b b5 f8 02 00 00        	movq	0x2f8(%rbp), %r14
14075836c: 48 8b 85 00 03 00 00        	movq	0x300(%rbp), %rax
140758373: 48 89 85 d0 03 00 00        	movq	%rax, 0x3d0(%rbp)
14075837a: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140758381: 48 8d 8d 28 04 00 00        	leaq	0x428(%rbp), %rcx
140758388: e8 43 02 93 ff              	callq	0x1400885d0 <.text+0x875d0>
14075838d: 48 69 c0 21 80 b2 04        	imulq	$0x4b28021, %rax, %rax  # imm = 0x4B28021
140758394: 48 03 86 a8 fd ff ff        	addq	-0x258(%rsi), %rax
14075839b: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1407583a2: 4c 8b be f8 fd ff ff        	movq	-0x208(%rsi), %r15
1407583a9: 4d 85 ff                    	testq	%r15, %r15
1407583ac: 4c 89 a5 c8 03 00 00        	movq	%r12, 0x3c8(%rbp)
1407583b3: 0f 84 f3 00 00 00           	je	0x1407584ac <.text+0x7574ac>
1407583b9: 48 8b b6 f0 fd ff ff        	movq	-0x210(%rsi), %rsi
1407583c0: 4a 8d 1c bd 00 00 00 00     	leaq	(,%r15,4), %rbx
1407583c8: 0f b6 05 3a c8 ed 02        	movzbl	0x2edc83a(%rip), %eax   # 0x143634c09
1407583cf: ba 04 00 00 00              	movl	$0x4, %edx
1407583d4: 48 89 d9                    	movq	%rbx, %rcx
1407583d7: e8 34 b8 eb ff              	callq	0x140613c10 <.text+0x612c10>
1407583dc: 48 85 c0                    	testq	%rax, %rax
1407583df: 0f 84 78 1b 00 00           	je	0x140759f5d <.text+0x758f5d>
1407583e5: 49 83 ff 07                 	cmpq	$0x7, %r15
1407583e9: 48 89 85 00 04 00 00        	movq	%rax, 0x400(%rbp)
1407583f0: 76 28                       	jbe	0x14075841a <.text+0x75741a>
1407583f2: 4e 8d 04 b8                 	leaq	(%rax,%r15,4), %r8
1407583f6: 48 8d 4e 04                 	leaq	0x4(%rsi), %rcx
1407583fa: 4b 8d 14 7f                 	leaq	(%r15,%r15,2), %rdx
1407583fe: 48 8d 14 96                 	leaq	(%rsi,%rdx,4), %rdx
140758402: 48 83 c2 fc                 	addq	$-0x4, %rdx
140758406: 48 39 d0                    	cmpq	%rdx, %rax
140758409: 0f 92 c2                    	setb	%dl
14075840c: 4c 39 c1                    	cmpq	%r8, %rcx
14075840f: 0f 92 c1                    	setb	%cl
140758412: 84 ca                       	testb	%cl, %dl
140758414: 0f 84 a0 00 00 00           	je	0x1407584ba <.text+0x7574ba>
14075841a: 45 31 d2                    	xorl	%r10d, %r10d
14075841d: 4d 89 f8                    	movq	%r15, %r8
140758420: 4c 89 d1                    	movq	%r10, %rcx
140758423: 49 83 e0 03                 	andq	$0x3, %r8
140758427: 74 2e                       	je	0x140758457 <.text+0x757457>
140758429: 4b 8d 0c 52                 	leaq	(%r10,%r10,2), %rcx
14075842d: 48 8d 14 8e                 	leaq	(%rsi,%rcx,4), %rdx
140758431: 48 83 c2 04                 	addq	$0x4, %rdx
140758435: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
140758439: 45 31 c9                    	xorl	%r9d, %r9d
14075843c: 4c 89 d1                    	movq	%r10, %rcx
14075843f: 90                          	nop
140758440: 66 42 0f 6e 04 8a           	movd	(%rdx,%r9,4), %xmm0
140758446: 66 0f 7e 04 88              	movd	%xmm0, (%rax,%rcx,4)
14075844b: 48 ff c1                    	incq	%rcx
14075844e: 49 83 c1 03                 	addq	$0x3, %r9
140758452: 4d 39 c8                    	cmpq	%r9, %r8
140758455: 75 e9                       	jne	0x140758440 <.text+0x757440>
140758457: 4d 29 fa                    	subq	%r15, %r10
14075845a: 49 83 fa fc                 	cmpq	$-0x4, %r10
14075845e: 0f 87 cc 00 00 00           	ja	0x140758530 <.text+0x757530>
140758464: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
140758468: 48 8d 14 96                 	leaq	(%rsi,%rdx,4), %rdx
14075846c: 48 83 c2 28                 	addq	$0x28, %rdx
140758470: f3 0f 10 42 dc              	movss	-0x24(%rdx), %xmm0
140758475: f3 0f 11 04 88              	movss	%xmm0, (%rax,%rcx,4)
14075847a: f3 0f 10 42 e8              	movss	-0x18(%rdx), %xmm0
14075847f: f3 0f 11 44 88 04           	movss	%xmm0, 0x4(%rax,%rcx,4)
140758485: f3 0f 10 42 f4              	movss	-0xc(%rdx), %xmm0
14075848a: f3 0f 11 44 88 08           	movss	%xmm0, 0x8(%rax,%rcx,4)
140758490: 66 0f 6e 02                 	movd	(%rdx), %xmm0
140758494: 66 0f 7e 44 88 0c           	movd	%xmm0, 0xc(%rax,%rcx,4)
14075849a: 48 83 c1 04                 	addq	$0x4, %rcx
14075849e: 48 83 c2 30                 	addq	$0x30, %rdx
1407584a2: 49 39 cf                    	cmpq	%rcx, %r15
1407584a5: 75 c9                       	jne	0x140758470 <.text+0x757470>
1407584a7: e9 84 00 00 00              	jmp	0x140758530 <.text+0x757530>
1407584ac: b8 04 00 00 00              	movl	$0x4, %eax
1407584b1: 48 89 85 00 04 00 00        	movq	%rax, 0x400(%rbp)
1407584b8: eb 7d                       	jmp	0x140758537 <.text+0x757537>
1407584ba: 4d 89 fa                    	movq	%r15, %r10
1407584bd: 49 83 e2 f8                 	andq	$-0x8, %r10
1407584c1: 48 8d 4e 58                 	leaq	0x58(%rsi), %rcx
1407584c5: 31 d2                       	xorl	%edx, %edx
1407584c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407584d0: 66 0f 6e 41 ac              	movd	-0x54(%rcx), %xmm0
1407584d5: 66 0f 6e 49 b8              	movd	-0x48(%rcx), %xmm1
1407584da: 66 0f 62 c1                 	punpckldq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1407584de: 66 0f 6e 49 c4              	movd	-0x3c(%rcx), %xmm1
1407584e3: 66 0f 6e 51 d0              	movd	-0x30(%rcx), %xmm2
1407584e8: 66 0f 62 ca                 	punpckldq	%xmm2, %xmm1    # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
1407584ec: 66 0f 6c c1                 	punpcklqdq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0]
1407584f0: 66 0f 6e 09                 	movd	(%rcx), %xmm1
1407584f4: 66 0f 6e 51 f4              	movd	-0xc(%rcx), %xmm2
1407584f9: 66 0f 62 d1                 	punpckldq	%xmm1, %xmm2    # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
1407584fd: 66 0f 6e 49 e8              	movd	-0x18(%rcx), %xmm1
140758502: 66 0f 6e 59 dc              	movd	-0x24(%rcx), %xmm3
140758507: 66 0f 62 d9                 	punpckldq	%xmm1, %xmm3    # xmm3 = xmm3[0],xmm1[0],xmm3[1],xmm1[1]
14075850b: 66 0f 6c da                 	punpcklqdq	%xmm2, %xmm3    # xmm3 = xmm3[0],xmm2[0]
14075850f: f3 0f 7f 04 90              	movdqu	%xmm0, (%rax,%rdx,4)
140758514: f3 0f 7f 5c 90 10           	movdqu	%xmm3, 0x10(%rax,%rdx,4)
14075851a: 48 83 c2 08                 	addq	$0x8, %rdx
14075851e: 48 83 c1 60                 	addq	$0x60, %rcx
140758522: 49 39 d2                    	cmpq	%rdx, %r10
140758525: 75 a9                       	jne	0x1407584d0 <.text+0x7574d0>
140758527: 4d 39 d7                    	cmpq	%r10, %r15
14075852a: 0f 85 ed fe ff ff           	jne	0x14075841d <.text+0x75741d>
140758530: 48 8b b5 18 04 00 00        	movq	0x418(%rbp), %rsi
140758537: 4c 89 bd d8 03 00 00        	movq	%r15, 0x3d8(%rbp)
14075853e: 48 8b 86 48 fe ff ff        	movq	-0x1b8(%rsi), %rax
140758545: 4c 8d 24 85 00 00 00 00     	leaq	(,%rax,4), %r12
14075854d: 48 89 85 a0 03 00 00        	movq	%rax, 0x3a0(%rbp)
140758554: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140758558: 0f 85 2a 1a 00 00           	jne	0x140759f88 <.text+0x758f88>
14075855e: 41 bf 00 00 00 00           	movl	$0x0, %r15d
140758564: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14075856e: 49 39 c4                    	cmpq	%rax, %r12
140758571: 0f 87 14 1a 00 00           	ja	0x140759f8b <.text+0x758f8b>
140758577: 48 8b 9e 40 fe ff ff        	movq	-0x1c0(%rsi), %rbx
14075857e: 4d 85 e4                    	testq	%r12, %r12
140758581: 74 33                       	je	0x1407585b6 <.text+0x7575b6>
140758583: 0f b6 05 7f c6 ed 02        	movzbl	0x2edc67f(%rip), %eax   # 0x143634c09
14075858a: 41 bf 04 00 00 00           	movl	$0x4, %r15d
140758590: ba 04 00 00 00              	movl	$0x4, %edx
140758595: 4c 89 e1                    	movq	%r12, %rcx
140758598: e8 73 b6 eb ff              	callq	0x140613c10 <.text+0x612c10>
14075859d: 48 85 c0                    	testq	%rax, %rax
1407585a0: 0f 84 e5 19 00 00           	je	0x140759f8b <.text+0x758f8b>
1407585a6: 48 8b 8d a0 03 00 00        	movq	0x3a0(%rbp), %rcx
1407585ad: 48 89 8d e0 02 00 00        	movq	%rcx, 0x2e0(%rbp)
1407585b4: eb 10                       	jmp	0x1407585c6 <.text+0x7575c6>
1407585b6: b8 04 00 00 00              	movl	$0x4, %eax
1407585bb: 48 c7 85 e0 02 00 00 00 00 00 00    	movq	$0x0, 0x2e0(%rbp)
1407585c6: 48 89 85 e0 03 00 00        	movq	%rax, 0x3e0(%rbp)
1407585cd: 48 89 c1                    	movq	%rax, %rcx
1407585d0: 48 89 da                    	movq	%rbx, %rdx
1407585d3: 4d 89 e0                    	movq	%r12, %r8
1407585d6: e8 25 b6 1c 02              	callq	0x142923c00 <.text+0x2922c00>
1407585db: 48 85 ff                    	testq	%rdi, %rdi
1407585de: 0f 84 cc 00 00 00           	je	0x1407586b0 <.text+0x7576b0>
1407585e4: 48 89 f9                    	movq	%rdi, %rcx
1407585e7: e8 94 ad 18 00              	callq	0x1408e3380 <.text+0x8e2380>
1407585ec: 84 c0                       	testb	%al, %al
1407585ee: 0f 84 bc 00 00 00           	je	0x1407586b0 <.text+0x7576b0>
1407585f4: 48 8d 4f 3d                 	leaq	0x3d(%rdi), %rcx
1407585f8: e8 33 ac 0d 00              	callq	0x140833230 <.text+0x832230>
1407585fd: 84 c0                       	testb	%al, %al
1407585ff: 0f 85 ab 00 00 00           	jne	0x1407586b0 <.text+0x7576b0>
140758605: 48 83 bd a0 03 00 00 00     	cmpq	$0x0, 0x3a0(%rbp)
14075860d: 0f 84 9d 00 00 00           	je	0x1407586b0 <.text+0x7576b0>
140758613: 49 83 c4 fc                 	addq	$-0x4, %r12
140758617: 48 8b 85 e0 03 00 00        	movq	0x3e0(%rbp), %rax
14075861e: 49 83 fc 1b                 	cmpq	$0x1b, %r12
140758622: 76 50                       	jbe	0x140758674 <.text+0x757674>
140758624: 49 c1 ec 02                 	shrq	$0x2, %r12
140758628: 49 ff c4                    	incq	%r12
14075862b: 4c 89 e1                    	movq	%r12, %rcx
14075862e: 48 83 e1 f8                 	andq	$-0x8, %rcx
140758632: 4c 8b 85 e0 03 00 00        	movq	0x3e0(%rbp), %r8
140758639: 49 8d 04 88                 	leaq	(%r8,%rcx,4), %rax
14075863d: 31 d2                       	xorl	%edx, %edx
14075863f: 90                          	nop
140758640: 41 0f 10 04 90              	movups	(%r8,%rdx,4), %xmm0
140758645: 41 0f 10 4c 90 10           	movups	0x10(%r8,%rdx,4), %xmm1
14075864b: 41 0f 58 c6                 	addps	%xmm14, %xmm0
14075864f: 41 0f 58 ce                 	addps	%xmm14, %xmm1
140758653: 41 0f 5f c2                 	maxps	%xmm10, %xmm0
140758657: 41 0f 5f ca                 	maxps	%xmm10, %xmm1
14075865b: 41 0f 11 04 90              	movups	%xmm0, (%r8,%rdx,4)
140758660: 41 0f 11 4c 90 10           	movups	%xmm1, 0x10(%r8,%rdx,4)
140758666: 48 83 c2 08                 	addq	$0x8, %rdx
14075866a: 48 39 d1                    	cmpq	%rdx, %rcx
14075866d: 75 d1                       	jne	0x140758640 <.text+0x757640>
14075866f: 49 39 cc                    	cmpq	%rcx, %r12
140758672: 74 3c                       	je	0x1407586b0 <.text+0x7576b0>
140758674: 48 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %rcx
14075867b: 48 8b 95 a0 03 00 00        	movq	0x3a0(%rbp), %rdx
140758682: 48 8d 0c 91                 	leaq	(%rcx,%rdx,4), %rcx
140758686: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140758690: f3 0f 10 00                 	movss	(%rax), %xmm0
140758694: f3 41 0f 58 c7              	addss	%xmm15, %xmm0
140758699: f3 41 0f 5f c0              	maxss	%xmm8, %xmm0
14075869e: f3 0f 11 00                 	movss	%xmm0, (%rax)
1407586a2: 48 8d 50 04                 	leaq	0x4(%rax), %rdx
1407586a6: 48 89 d0                    	movq	%rdx, %rax
1407586a9: 48 39 ca                    	cmpq	%rcx, %rdx
1407586ac: 75 e2                       	jne	0x140758690 <.text+0x757690>
1407586ae: 66 90                       	nop
1407586b0: 48 8b 96 f0 fd ff ff        	movq	-0x210(%rsi), %rdx
1407586b7: 48 8b 86 f8 fd ff ff        	movq	-0x208(%rsi), %rax
1407586be: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407586c2: 4c 8d 04 82                 	leaq	(%rdx,%rax,4), %r8
1407586c6: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407586cd: 4c 8d 0d bc 04 4c 02        	leaq	0x24c04bc(%rip), %r9    # 0x142c18b90
1407586d4: e8 07 23 b6 ff              	callq	0x1402ba9e0 <.text+0x2b99e0>
1407586d9: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
1407586e0: 48 8d 95 30 02 00 00        	leaq	0x230(%rbp), %rdx
1407586e7: 45 31 c0                    	xorl	%r8d, %r8d
1407586ea: e8 31 12 aa 00              	callq	0x1411f9920 <.text+0x11f8920>
1407586ef: 31 c0                       	xorl	%eax, %eax
1407586f1: 48 3b 85 f0 02 00 00        	cmpq	0x2f0(%rbp), %rax
1407586f8: 0f 80 22 18 00 00           	jo	0x140759f20 <.text+0x758f20>
1407586fe: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
140758705: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14075870c: f3 0f 6f 85 f0 02 00 00     	movdqu	0x2f0(%rbp), %xmm0
140758714: f3 0f 6f 8d 00 03 00 00     	movdqu	0x300(%rbp), %xmm1
14075871c: f3 0f 6f 95 10 03 00 00     	movdqu	0x310(%rbp), %xmm2
140758724: 66 0f 7f 95 60 01 00 00     	movdqa	%xmm2, 0x160(%rbp)
14075872c: 66 0f 7f 8d 50 01 00 00     	movdqa	%xmm1, 0x150(%rbp)
140758734: 66 0f 7f 85 40 01 00 00     	movdqa	%xmm0, 0x140(%rbp)
14075873c: 41 b4 02                    	movb	$0x2, %r12b
14075873f: 45 84 ed                    	testb	%r13b, %r13b
140758742: 0f 84 95 00 00 00           	je	0x1407587dd <.text+0x7577dd>
140758748: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
14075874f: e8 cc 1e 3d 00              	callq	0x140b2a620 <.text+0xb29620>
140758754: 66 0f 7e 85 98 03 00 00     	movd	%xmm0, 0x398(%rbp)
14075875c: 48 8d 4e f9                 	leaq	-0x7(%rsi), %rcx
140758760: e8 0b cf 16 00              	callq	0x1408c5670 <.text+0x8c4670>
140758765: 41 88 c4                    	movb	%al, %r12b
140758768: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
14075876f: e8 5c 1e 3d 00              	callq	0x140b2a5d0 <.text+0xb295d0>
140758774: 48 89 c1                    	movq	%rax, %rcx
140758777: e8 c4 6f aa 00              	callq	0x1411ff740 <.text+0x11fe740>
14075877c: f6 85 c8 03 00 00 01        	testb	$0x1, 0x3c8(%rbp)
140758783: 0f 84 f1 17 00 00           	je	0x140759f7a <.text+0x758f7a>
140758789: 49 89 c5                    	movq	%rax, %r13
14075878c: 48 89 c1                    	movq	%rax, %rcx
14075878f: 4c 89 f2                    	movq	%r14, %rdx
140758792: e8 39 0f 54 00              	callq	0x140c996d0 <.text+0xc986d0>
140758797: 66 44 0f 6f d8              	movdqa	%xmm0, %xmm11
14075879c: 49 ff c6                    	incq	%r14
14075879f: 4c 89 e9                    	movq	%r13, %rcx
1407587a2: 4c 89 f2                    	movq	%r14, %rdx
1407587a5: e8 26 0f 54 00              	callq	0x140c996d0 <.text+0xc986d0>
1407587aa: 66 44 0f 6f e0              	movdqa	%xmm0, %xmm12
1407587af: 4c 89 e9                    	movq	%r13, %rcx
1407587b2: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1407587b9: e8 12 0f 54 00              	callq	0x140c996d0 <.text+0xc986d0>
1407587be: 66 44 0f 6f e8              	movdqa	%xmm0, %xmm13
1407587c3: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1407587ca: 48 ff c2                    	incq	%rdx
1407587cd: 4c 89 e9                    	movq	%r13, %rcx
1407587d0: e8 fb 0e 54 00              	callq	0x140c996d0 <.text+0xc986d0>
1407587d5: 66 0f 7e 85 9c 03 00 00     	movd	%xmm0, 0x39c(%rbp)
1407587dd: 4c 8d 7e f9                 	leaq	-0x7(%rsi), %r15
1407587e1: c6 85 34 04 00 00 00        	movb	$0x0, 0x434(%rbp)
1407587e8: 4c 89 f9                    	movq	%r15, %rcx
1407587eb: e8 80 ce 16 00              	callq	0x1408c5670 <.text+0x8c4670>
1407587f0: 84 c0                       	testb	%al, %al
1407587f2: 0f 84 c7 00 00 00           	je	0x1407588bf <.text+0x7578bf>
1407587f8: 8b 85 94 03 00 00           	movl	0x394(%rbp), %eax
1407587fe: 48 8b 8d 10 02 00 00        	movq	0x210(%rbp), %rcx
140758805: 89 01                       	movl	%eax, (%rcx)
140758807: 48 8b b5 28 04 00 00        	movq	0x428(%rbp), %rsi
14075880e: 48 89 b5 f0 02 00 00        	movq	%rsi, 0x2f0(%rbp)
140758815: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
14075881c: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140758823: e8 48 85 e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140758828: 48 89 c3                    	movq	%rax, %rbx
14075882b: 48 c1 eb 39                 	shrq	$0x39, %rbx
14075882f: 48 8b 95 10 04 00 00        	movq	0x410(%rbp), %rdx
140758836: 48 8b 0a                    	movq	(%rdx), %rcx
140758839: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
14075883d: 49 89 d0                    	movq	%rdx, %r8
140758840: 49 21 c0                    	andq	%rax, %r8
140758843: 66 0f 6e c3                 	movd	%ebx, %xmm0
140758847: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14075884b: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140758850: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140758855: 4c 8d 49 98                 	leaq	-0x68(%rcx), %r9
140758859: 45 31 d2                    	xorl	%r10d, %r10d
14075885c: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140758862: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140758866: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14075886a: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14075886f: 45 85 db                    	testl	%r11d, %r11d
140758872: 74 29                       	je	0x14075889d <.text+0x75789d>
140758874: f3 45 0f bc f3              	tzcntl	%r11d, %r14d
140758879: 4d 01 c6                    	addq	%r8, %r14
14075887c: 49 21 d6                    	andq	%rdx, %r14
14075887f: 49 f7 de                    	negq	%r14
140758882: 4d 6b f6 68                 	imulq	$0x68, %r14, %r14
140758886: 4b 39 34 31                 	cmpq	%rsi, (%r9,%r14)
14075888a: 0f 84 f2 00 00 00           	je	0x140758982 <.text+0x757982>
140758890: 45 8d 73 ff                 	leal	-0x1(%r11), %r14d
140758894: 66 45 21 de                 	andw	%r11w, %r14w
140758898: 45 89 f3                    	movl	%r14d, %r11d
14075889b: 75 d7                       	jne	0x140758874 <.text+0x757874>
14075889d: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1407588a1: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1407588a6: 45 85 db                    	testl	%r11d, %r11d
1407588a9: 0f 85 e3 00 00 00           	jne	0x140758992 <.text+0x757992>
1407588af: 4d 01 d0                    	addq	%r10, %r8
1407588b2: 49 83 c0 10                 	addq	$0x10, %r8
1407588b6: 49 83 c2 10                 	addq	$0x10, %r10
1407588ba: 49 21 d0                    	andq	%rdx, %r8
1407588bd: eb 9d                       	jmp	0x14075885c <.text+0x75785c>
1407588bf: 8b 85 94 03 00 00           	movl	0x394(%rbp), %eax
1407588c5: 48 8b 8d 10 02 00 00        	movq	0x210(%rbp), %rcx
1407588cc: 89 01                       	movl	%eax, (%rcx)
1407588ce: 48 8b b5 28 04 00 00        	movq	0x428(%rbp), %rsi
1407588d5: 48 89 b5 f0 02 00 00        	movq	%rsi, 0x2f0(%rbp)
1407588dc: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
1407588e3: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
1407588ea: e8 81 84 e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
1407588ef: 48 89 c3                    	movq	%rax, %rbx
1407588f2: 48 c1 eb 39                 	shrq	$0x39, %rbx
1407588f6: 48 8b 8d 10 04 00 00        	movq	0x410(%rbp), %rcx
1407588fd: 48 8b 11                    	movq	(%rcx), %rdx
140758900: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
140758904: 49 89 c8                    	movq	%rcx, %r8
140758907: 49 21 c0                    	andq	%rax, %r8
14075890a: 66 0f 6e c3                 	movd	%ebx, %xmm0
14075890e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140758912: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140758917: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14075891c: 4c 8d 4a 98                 	leaq	-0x68(%rdx), %r9
140758920: 45 31 d2                    	xorl	%r10d, %r10d
140758923: f3 42 0f 6f 0c 02           	movdqu	(%rdx,%r8), %xmm1
140758929: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14075892d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140758931: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
140758936: 45 85 db                    	testl	%r11d, %r11d
140758939: 74 25                       	je	0x140758960 <.text+0x757960>
14075893b: f3 45 0f bc f3              	tzcntl	%r11d, %r14d
140758940: 4d 01 c6                    	addq	%r8, %r14
140758943: 49 21 ce                    	andq	%rcx, %r14
140758946: 49 f7 de                    	negq	%r14
140758949: 4d 6b f6 68                 	imulq	$0x68, %r14, %r14
14075894d: 4b 39 34 31                 	cmpq	%rsi, (%r9,%r14)
140758951: 74 37                       	je	0x14075898a <.text+0x75798a>
140758953: 45 8d 73 ff                 	leal	-0x1(%r11), %r14d
140758957: 66 45 21 de                 	andw	%r11w, %r14w
14075895b: 45 89 f3                    	movl	%r14d, %r11d
14075895e: 75 db                       	jne	0x14075893b <.text+0x75793b>
140758960: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140758964: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
140758969: 45 85 db                    	testl	%r11d, %r11d
14075896c: 0f 85 4e 01 00 00           	jne	0x140758ac0 <.text+0x757ac0>
140758972: 4d 01 d0                    	addq	%r10, %r8
140758975: 49 83 c0 10                 	addq	$0x10, %r8
140758979: 49 83 c2 10                 	addq	$0x10, %r10
14075897d: 49 21 c8                    	andq	%rcx, %r8
140758980: eb a1                       	jmp	0x140758923 <.text+0x757923>
140758982: 4c 01 f1                    	addq	%r14, %rcx
140758985: e9 08 01 00 00              	jmp	0x140758a92 <.text+0x757a92>
14075898a: 4c 01 f2                    	addq	%r14, %rdx
14075898d: e9 2a 02 00 00              	jmp	0x140758bbc <.text+0x757bbc>
140758992: 49 89 c6                    	movq	%rax, %r14
140758995: 48 8b 85 10 04 00 00        	movq	0x410(%rbp), %rax
14075899c: 48 83 78 10 00              	cmpq	$0x0, 0x10(%rax)
1407589a1: 0f 84 8c 11 00 00           	je	0x140759b33 <.text+0x758b33>
1407589a7: c6 85 34 04 00 00 00        	movb	$0x0, 0x434(%rbp)
1407589ae: 48 8d 8d f8 02 00 00        	leaq	0x2f8(%rbp), %rcx
1407589b5: e8 96 e4 82 00              	callq	0x140f86e50 <.text+0xf85e50>
1407589ba: 48 89 b5 f0 02 00 00        	movq	%rsi, 0x2f0(%rbp)
1407589c1: 48 8b 8d 10 04 00 00        	movq	0x410(%rbp), %rcx
1407589c8: 48 8b 01                    	movq	(%rcx), %rax
1407589cb: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1407589cf: 4c 89 f2                    	movq	%r14, %rdx
1407589d2: 48 21 ca                    	andq	%rcx, %rdx
1407589d5: f3 0f 6f 04 10              	movdqu	(%rax,%rdx), %xmm0
1407589da: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
1407589df: 45 85 c0                    	testl	%r8d, %r8d
1407589e2: 0f 84 01 11 00 00           	je	0x140759ae9 <.text+0x758ae9>
1407589e8: f3 45 0f bc c0              	tzcntl	%r8d, %r8d
1407589ed: 49 01 d0                    	addq	%rdx, %r8
1407589f0: 49 21 c8                    	andq	%rcx, %r8
1407589f3: 42 0f b6 14 00              	movzbl	(%rax,%r8), %edx
1407589f8: 84 d2                       	testb	%dl, %dl
1407589fa: 0f 89 81 11 00 00           	jns	0x140759b81 <.text+0x758b81>
140758a00: 4d 8d 48 f0                 	leaq	-0x10(%r8), %r9
140758a04: 49 21 c9                    	andq	%rcx, %r9
140758a07: 42 88 1c 00                 	movb	%bl, (%rax,%r8)
140758a0b: 42 88 5c 08 10              	movb	%bl, 0x10(%rax,%r9)
140758a10: 49 f7 d8                    	negq	%r8
140758a13: 4d 6b c0 68                 	imulq	$0x68, %r8, %r8
140758a17: 4a 8d 0c 00                 	leaq	(%rax,%r8), %rcx
140758a1b: 80 e2 01                    	andb	$0x1, %dl
140758a1e: 0f b6 d2                    	movzbl	%dl, %edx
140758a21: 4c 8b 8d 10 04 00 00        	movq	0x410(%rbp), %r9
140758a28: 49 29 51 10                 	subq	%rdx, 0x10(%r9)
140758a2c: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
140758a33: 4a 89 54 00 f8              	movq	%rdx, -0x8(%rax,%r8)
140758a38: 0f 10 85 40 03 00 00        	movups	0x340(%rbp), %xmm0
140758a3f: 42 0f 11 44 00 e8           	movups	%xmm0, -0x18(%rax,%r8)
140758a45: 0f 10 85 30 03 00 00        	movups	0x330(%rbp), %xmm0
140758a4c: 42 0f 11 44 00 d8           	movups	%xmm0, -0x28(%rax,%r8)
140758a52: f3 0f 6f 85 f0 02 00 00     	movdqu	0x2f0(%rbp), %xmm0
140758a5a: f3 0f 6f 8d 00 03 00 00     	movdqu	0x300(%rbp), %xmm1
140758a62: f3 0f 6f 95 10 03 00 00     	movdqu	0x310(%rbp), %xmm2
140758a6a: f3 0f 6f 9d 20 03 00 00     	movdqu	0x320(%rbp), %xmm3
140758a72: f3 42 0f 7f 5c 00 c8        	movdqu	%xmm3, -0x38(%rax,%r8)
140758a79: f3 42 0f 7f 54 00 b8        	movdqu	%xmm2, -0x48(%rax,%r8)
140758a80: f3 42 0f 7f 4c 00 a8        	movdqu	%xmm1, -0x58(%rax,%r8)
140758a87: f3 42 0f 7f 44 00 98        	movdqu	%xmm0, -0x68(%rax,%r8)
140758a8e: 49 ff 41 18                 	incq	0x18(%r9)
140758a92: 48 83 c1 a0                 	addq	$-0x60, %rcx
140758a96: c6 85 34 04 00 00 00        	movb	$0x0, 0x434(%rbp)
140758a9d: e8 0e e4 82 00              	callq	0x140f86eb0 <.text+0xf85eb0>
140758aa2: 49 89 c6                    	movq	%rax, %r14
140758aa5: 45 0f 28 ce                 	movaps	%xmm14, %xmm9
140758aa9: 45 0f 28 f0                 	movaps	%xmm8, %xmm14
140758aad: 45 0f 28 c7                 	movaps	%xmm15, %xmm8
140758ab1: c7 85 c8 03 00 00 00 00 00 00       	movl	$0x0, 0x3c8(%rbp)
140758abb: e9 89 02 00 00              	jmp	0x140758d49 <.text+0x757d49>
140758ac0: 49 89 c6                    	movq	%rax, %r14
140758ac3: 48 8b 85 10 04 00 00        	movq	0x410(%rbp), %rax
140758aca: 48 83 78 10 00              	cmpq	$0x0, 0x10(%rax)
140758acf: 0f 84 85 10 00 00           	je	0x140759b5a <.text+0x758b5a>
140758ad5: c6 85 34 04 00 00 00        	movb	$0x0, 0x434(%rbp)
140758adc: 48 8d 8d f8 02 00 00        	leaq	0x2f8(%rbp), %rcx
140758ae3: e8 68 e3 82 00              	callq	0x140f86e50 <.text+0xf85e50>
140758ae8: 48 89 b5 f0 02 00 00        	movq	%rsi, 0x2f0(%rbp)
140758aef: 48 8b 8d 10 04 00 00        	movq	0x410(%rbp), %rcx
140758af6: 48 8b 01                    	movq	(%rcx), %rax
140758af9: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
140758afd: 4c 89 f2                    	movq	%r14, %rdx
140758b00: 48 21 ca                    	andq	%rcx, %rdx
140758b03: f3 0f 6f 04 10              	movdqu	(%rax,%rdx), %xmm0
140758b08: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
140758b0d: 45 85 c0                    	testl	%r8d, %r8d
140758b10: 0f 84 f8 0f 00 00           	je	0x140759b0e <.text+0x758b0e>
140758b16: f3 45 0f bc c0              	tzcntl	%r8d, %r8d
140758b1b: 49 01 d0                    	addq	%rdx, %r8
140758b1e: 49 21 c8                    	andq	%rcx, %r8
140758b21: 46 0f b6 0c 00              	movzbl	(%rax,%r8), %r9d
140758b26: 45 84 c9                    	testb	%r9b, %r9b
140758b29: 0f 89 69 10 00 00           	jns	0x140759b98 <.text+0x758b98>
140758b2f: 49 8d 50 f0                 	leaq	-0x10(%r8), %rdx
140758b33: 48 21 ca                    	andq	%rcx, %rdx
140758b36: 42 88 1c 00                 	movb	%bl, (%rax,%r8)
140758b3a: 88 5c 10 10                 	movb	%bl, 0x10(%rax,%rdx)
140758b3e: 49 f7 d8                    	negq	%r8
140758b41: 49 6b c8 68                 	imulq	$0x68, %r8, %rcx
140758b45: 48 8d 14 08                 	leaq	(%rax,%rcx), %rdx
140758b49: 41 80 e1 01                 	andb	$0x1, %r9b
140758b4d: 45 0f b6 c1                 	movzbl	%r9b, %r8d
140758b51: 4c 8b 8d 10 04 00 00        	movq	0x410(%rbp), %r9
140758b58: 4d 29 41 10                 	subq	%r8, 0x10(%r9)
140758b5c: 4c 8b 85 50 03 00 00        	movq	0x350(%rbp), %r8
140758b63: 4c 89 44 08 f8              	movq	%r8, -0x8(%rax,%rcx)
140758b68: 0f 10 85 40 03 00 00        	movups	0x340(%rbp), %xmm0
140758b6f: 0f 11 44 08 e8              	movups	%xmm0, -0x18(%rax,%rcx)
140758b74: 0f 10 85 30 03 00 00        	movups	0x330(%rbp), %xmm0
140758b7b: 0f 11 44 08 d8              	movups	%xmm0, -0x28(%rax,%rcx)
140758b80: f3 0f 6f 85 f0 02 00 00     	movdqu	0x2f0(%rbp), %xmm0
140758b88: f3 0f 6f 8d 00 03 00 00     	movdqu	0x300(%rbp), %xmm1
140758b90: f3 0f 6f 95 10 03 00 00     	movdqu	0x310(%rbp), %xmm2
140758b98: f3 0f 6f 9d 20 03 00 00     	movdqu	0x320(%rbp), %xmm3
140758ba0: f3 0f 7f 5c 08 c8           	movdqu	%xmm3, -0x38(%rax,%rcx)
140758ba6: f3 0f 7f 54 08 b8           	movdqu	%xmm2, -0x48(%rax,%rcx)
140758bac: f3 0f 7f 4c 08 a8           	movdqu	%xmm1, -0x58(%rax,%rcx)
140758bb2: f3 0f 7f 44 08 98           	movdqu	%xmm0, -0x68(%rax,%rcx)
140758bb8: 49 ff 41 18                 	incq	0x18(%r9)
140758bbc: 48 83 c2 a0                 	addq	$-0x60, %rdx
140758bc0: c6 85 34 04 00 00 00        	movb	$0x0, 0x434(%rbp)
140758bc7: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140758bce: e8 7d e3 82 00              	callq	0x140f86f50 <.text+0xf85f50>
140758bd3: 0f 10 85 40 03 00 00        	movups	0x340(%rbp), %xmm0
140758bda: 0f 29 85 80 02 00 00        	movaps	%xmm0, 0x280(%rbp)
140758be1: 0f 10 85 30 03 00 00        	movups	0x330(%rbp), %xmm0
140758be8: 0f 29 85 70 02 00 00        	movaps	%xmm0, 0x270(%rbp)
140758bef: f3 0f 6f 85 f0 02 00 00     	movdqu	0x2f0(%rbp), %xmm0
140758bf7: f3 0f 6f 8d 00 03 00 00     	movdqu	0x300(%rbp), %xmm1
140758bff: f3 0f 6f 95 10 03 00 00     	movdqu	0x310(%rbp), %xmm2
140758c07: f3 0f 6f 9d 20 03 00 00     	movdqu	0x320(%rbp), %xmm3
140758c0f: 66 0f 7f 9d 60 02 00 00     	movdqa	%xmm3, 0x260(%rbp)
140758c17: 66 0f 7f 95 50 02 00 00     	movdqa	%xmm2, 0x250(%rbp)
140758c1f: 66 0f 7f 8d 40 02 00 00     	movdqa	%xmm1, 0x240(%rbp)
140758c27: 66 0f 7f 85 30 02 00 00     	movdqa	%xmm0, 0x230(%rbp)
140758c2f: c6 85 34 04 00 00 01        	movb	$0x1, 0x434(%rbp)
140758c36: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
140758c3d: e8 be e2 82 00              	callq	0x140f86f00 <.text+0xf85f00>
140758c42: f3 0f 10 85 98 03 00 00     	movss	0x398(%rbp), %xmm0
140758c4a: f3 0f 11 85 f0 02 00 00     	movss	%xmm0, 0x2f0(%rbp)
140758c52: 66 44 0f 7e 9d f4 02 00 00  	movd	%xmm11, 0x2f4(%rbp)
140758c5b: 66 44 0f 7e a5 f8 02 00 00  	movd	%xmm12, 0x2f8(%rbp)
140758c64: 66 44 0f 7e ad fc 02 00 00  	movd	%xmm13, 0x2fc(%rbp)
140758c6d: 66 0f 6e 85 9c 03 00 00     	movd	0x39c(%rbp), %xmm0
140758c75: 66 0f 7e 85 00 03 00 00     	movd	%xmm0, 0x300(%rbp)
140758c7d: 44 88 a5 04 03 00 00        	movb	%r12b, 0x304(%rbp)
140758c84: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
140758c8b: c6 85 34 04 00 00 01        	movb	$0x1, 0x434(%rbp)
140758c92: 48 8b 8d d0 02 00 00        	movq	0x2d0(%rbp), %rcx
140758c99: 48 89 4c 24 50              	movq	%rcx, 0x50(%rsp)
140758c9e: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140758ca5: 48 89 4c 24 48              	movq	%rcx, 0x48(%rsp)
140758caa: 48 8b 8d f0 03 00 00        	movq	0x3f0(%rbp), %rcx
140758cb1: 88 4c 24 40                 	movb	%cl, 0x40(%rsp)
140758cb5: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140758cba: 48 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %rax
140758cc1: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140758cc6: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
140758ccd: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140758cd2: 48 8b 85 e0 03 00 00        	movq	0x3e0(%rbp), %rax
140758cd9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140758cde: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
140758ce5: 4c 8b 85 00 04 00 00        	movq	0x400(%rbp), %r8
140758cec: 4c 8b 8d d8 03 00 00        	movq	0x3d8(%rbp), %r9
140758cf3: e8 48 c8 ab 00              	callq	0x141215540 <.text+0x1214540>
140758cf8: 48 8b 85 18 04 00 00        	movq	0x418(%rbp), %rax
140758cff: 66 0f 6e 48 ec              	movd	-0x14(%rax), %xmm1
140758d04: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
140758d08: 41 0f 95 c0                 	setne	%r8b
140758d0c: c6 85 34 04 00 00 01        	movb	$0x1, 0x434(%rbp)
140758d13: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
140758d1a: e8 e1 e5 82 00              	callq	0x140f87300 <.text+0xf86300>
140758d1f: c6 85 34 04 00 00 01        	movb	$0x1, 0x434(%rbp)
140758d26: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
140758d2d: e8 7e e1 82 00              	callq	0x140f86eb0 <.text+0xf85eb0>
140758d32: 49 89 c6                    	movq	%rax, %r14
140758d35: 45 0f 28 ce                 	movaps	%xmm14, %xmm9
140758d39: 45 0f 28 f0                 	movaps	%xmm8, %xmm14
140758d3d: 45 0f 28 c7                 	movaps	%xmm15, %xmm8
140758d41: b0 01                       	movb	$0x1, %al
140758d43: 89 85 c8 03 00 00           	movl	%eax, 0x3c8(%rbp)
140758d49: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
140758d54: 48 c7 85 18 01 00 00 04 00 00 00    	movq	$0x4, 0x118(%rbp)
140758d5f: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140758d6a: c6 85 36 04 00 00 01        	movb	$0x1, 0x436(%rbp)
140758d71: 4c 89 f9                    	movq	%r15, %rcx
140758d74: e8 f7 c8 16 00              	callq	0x1408c5670 <.text+0x8c4670>
140758d79: 48 85 ff                    	testq	%rdi, %rdi
140758d7c: 0f 95 c1                    	setne	%cl
140758d7f: 84 c1                       	testb	%al, %cl
140758d81: 0f 84 ed 00 00 00           	je	0x140758e74 <.text+0x757e74>
140758d87: c6 85 36 04 00 00 01        	movb	$0x1, 0x436(%rbp)
140758d8e: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140758d92: e8 d9 dc 3c 00              	callq	0x140b26a70 <.text+0xb25a70>
140758d97: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
140758d9e: 48 89 95 f8 02 00 00        	movq	%rdx, 0x2f8(%rbp)
140758da5: 48 8b 85 18 04 00 00        	movq	0x418(%rbp), %rax
140758dac: f3 0f 6f 80 58 fe ff ff     	movdqu	-0x1a8(%rax), %xmm0
140758db4: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
140758dbb: c6 85 36 04 00 00 01        	movb	$0x1, 0x436(%rbp)
140758dc2: 48 8d 85 10 01 00 00        	leaq	0x110(%rbp), %rax
140758dc9: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140758dce: 4c 89 74 24 60              	movq	%r14, 0x60(%rsp)
140758dd3: 48 89 7c 24 58              	movq	%rdi, 0x58(%rsp)
140758dd8: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
140758ddf: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
140758de4: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140758deb: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140758df0: 48 8b 85 00 04 00 00        	movq	0x400(%rbp), %rax
140758df7: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140758dfc: f3 0f 7f 44 24 30           	movdqu	%xmm0, 0x30(%rsp)
140758e02: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
140758e09: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140758e0e: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
140758e15: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140758e1a: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
140758e21: 4c 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %r8
140758e28: 4c 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %r9
140758e2f: e8 fc ff ab 00              	callq	0x141218e30 <.text+0x1217e30>
140758e34: c6 85 36 04 00 00 01        	movb	$0x1, 0x436(%rbp)
140758e3b: 48 89 f9                    	movq	%rdi, %rcx
140758e3e: e8 6d a5 18 00              	callq	0x1408e33b0 <.text+0x8e23b0>
140758e43: 84 c0                       	testb	%al, %al
140758e45: 74 2d                       	je	0x140758e74 <.text+0x757e74>
140758e47: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
140758e4e: c6 85 36 04 00 00 01        	movb	$0x1, 0x436(%rbp)
140758e55: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
140758e5a: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
140758e61: 4c 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %r8
140758e68: 4c 8b 8d d0 02 00 00        	movq	0x2d0(%rbp), %r9
140758e6f: e8 6c ed ab 00              	callq	0x141217be0 <.text+0x1216be0>
140758e74: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140758e7b: 48 89 85 d0 03 00 00        	movq	%rax, 0x3d0(%rbp)
140758e82: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
140758e89: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
140758e90: 48 8b bd 20 01 00 00        	movq	0x120(%rbp), %rdi
140758e97: 48 8b 9d 28 04 00 00        	movq	0x428(%rbp), %rbx
140758e9e: 8b 85 a0 02 00 00           	movl	0x2a0(%rbp), %eax
140758ea4: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140758eab: 89 01                       	movl	%eax, (%rcx)
140758ead: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140758eb4: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140758eb8: 4c 8b 68 40                 	movq	0x40(%rax), %r13
140758ebc: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140758ec0: 75 13                       	jne	0x140758ed5 <.text+0x757ed5>
140758ec2: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
140758ec9: 48 8d 15 f8 35 4c 02        	leaq	0x24c35f8(%rip), %rdx   # 0x142c1c4c8
140758ed0: e8 db 9b e0 ff              	callq	0x140562ab0 <.text+0x561ab0>
140758ed5: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
140758edc: 48 8b 42 28                 	movq	0x28(%rdx), %rax
140758ee0: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
140758ee4: 4c 8b 85 d0 03 00 00        	movq	0x3d0(%rbp), %r8
140758eeb: 4c 89 04 c8                 	movq	%r8, (%rax,%rcx,8)
140758eef: 4c 8b 85 28 02 00 00        	movq	0x228(%rbp), %r8
140758ef6: 4c 89 44 c8 08              	movq	%r8, 0x8(%rax,%rcx,8)
140758efb: 48 89 7c c8 10              	movq	%rdi, 0x10(%rax,%rcx,8)
140758f00: 48 89 5c c8 18              	movq	%rbx, 0x18(%rax,%rcx,8)
140758f05: 4c 89 6c c8 20              	movq	%r13, 0x20(%rax,%rcx,8)
140758f0a: 48 ff c6                    	incq	%rsi
140758f0d: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
140758f11: 48 ff 42 40                 	incq	0x40(%rdx)
140758f15: 41 80 3f 01                 	cmpb	$0x1, (%r15)
140758f19: 75 5a                       	jne	0x140758f75 <.text+0x757f75>
140758f1b: 48 8b 85 18 04 00 00        	movq	0x418(%rbp), %rax
140758f22: f3 0f 6f 80 58 fe ff ff     	movdqu	-0x1a8(%rax), %xmm0
140758f2a: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
140758f31: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
140758f38: 4c 89 74 24 40              	movq	%r14, 0x40(%rsp)
140758f3d: f3 0f 7f 44 24 30           	movdqu	%xmm0, 0x30(%rsp)
140758f43: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140758f4a: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140758f4f: 48 8b 85 00 04 00 00        	movq	0x400(%rbp), %rax
140758f56: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140758f5b: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
140758f62: 4c 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %r8
140758f69: 4c 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %r9
140758f70: e8 7b 92 ac 00              	callq	0x1412221f0 <.text+0x12211f0>
140758f75: 48 8b 85 f0 03 00 00        	movq	0x3f0(%rbp), %rax
140758f7c: ff c8                       	decl	%eax
140758f7e: a8 fd                       	testb	$-0x3, %al
140758f80: 75 39                       	jne	0x140758fbb <.text+0x757fbb>
140758f82: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
140758f89: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
140758f90: 4c 89 74 24 28              	movq	%r14, 0x28(%rsp)
140758f95: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
140758f9c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140758fa1: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
140758fa8: 4c 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %r8
140758faf: 4c 8b 8d 20 02 00 00        	movq	0x220(%rbp), %r9
140758fb6: e8 55 ad ac 00              	callq	0x141223d10 <.text+0x1222d10>
140758fbb: 48 8b 8d 28 04 00 00        	movq	0x428(%rbp), %rcx
140758fc2: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
140758fc9: e8 c2 f7 0f 00              	callq	0x140858790 <.text+0x857790>
140758fce: 89 c7                       	movl	%eax, %edi
140758fd0: 48 8b 85 18 04 00 00        	movq	0x418(%rbp), %rax
140758fd7: 48 8b 98 58 fe ff ff        	movq	-0x1a8(%rax), %rbx
140758fde: 4c 8b b8 60 fe ff ff        	movq	-0x1a0(%rax), %r15
140758fe5: 66 0f ef f6                 	pxor	%xmm6, %xmm6
140758fe9: 41 80 fc 02                 	cmpb	$0x2, %r12b
140758fed: 75 67                       	jne	0x140759056 <.text+0x758056>
140758fef: 48 8d 48 f5                 	leaq	-0xb(%rax), %rcx
140758ff3: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
140758ffa: e8 31 a2 0d 00              	callq	0x140833230 <.text+0x832230>
140758fff: 66 0f 6e 35 6d 38 32 02     	movd	0x232386d(%rip), %xmm6  # 0x142a7c874
140759007: 84 c0                       	testb	%al, %al
140759009: 48 8b 85 18 04 00 00        	movq	0x418(%rbp), %rax
140759010: 74 44                       	je	0x140759056 <.text+0x758056>
140759012: f3 0f 10 40 ec              	movss	-0x14(%rax), %xmm0
140759017: f3 0f 58 05 59 38 32 02     	addss	0x2323859(%rip), %xmm0  # 0x142a7c878
14075901f: f3 0f 5e 05 55 38 32 02     	divss	0x2323855(%rip), %xmm0  # 0x142a7c87c
140759027: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14075902b: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14075902f: f3 0f 10 05 cd c8 1c 02     	movss	0x21cc8cd(%rip), %xmm0  # 0x142925904
140759037: f3 0f 5d c1                 	minss	%xmm1, %xmm0
14075903b: 0f 28 c8                    	movaps	%xmm0, %xmm1
14075903e: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
140759042: f3 0f 58 c0                 	addss	%xmm0, %xmm0
140759046: f3 0f 10 35 2a 40 27 02     	movss	0x227402a(%rip), %xmm6  # 0x1429cd078
14075904e: f3 0f 5c f0                 	subss	%xmm0, %xmm6
140759052: f3 0f 59 f1                 	mulss	%xmm1, %xmm6
140759056: f3 0f 10 78 d4              	movss	-0x2c(%rax), %xmm7
14075905b: f3 0f 10 85 98 03 00 00     	movss	0x398(%rbp), %xmm0
140759063: f3 0f 11 85 f0 02 00 00     	movss	%xmm0, 0x2f0(%rbp)
14075906b: 66 44 0f 7e 9d f4 02 00 00  	movd	%xmm11, 0x2f4(%rbp)
140759074: 66 44 0f 7e a5 f8 02 00 00  	movd	%xmm12, 0x2f8(%rbp)
14075907d: 66 44 0f 7e ad fc 02 00 00  	movd	%xmm13, 0x2fc(%rbp)
140759086: f3 0f 10 85 9c 03 00 00     	movss	0x39c(%rbp), %xmm0
14075908e: f3 0f 11 85 00 03 00 00     	movss	%xmm0, 0x300(%rbp)
140759096: 44 88 a5 04 03 00 00        	movb	%r12b, 0x304(%rbp)
14075909d: 0f b6 45 7a                 	movzbl	0x7a(%rbp), %eax
1407590a1: 48 8d 8d f8 02 00 00        	leaq	0x2f8(%rbp), %rcx
1407590a8: 88 41 0f                    	movb	%al, 0xf(%rcx)
1407590ab: 0f b7 45 78                 	movzwl	0x78(%rbp), %eax
1407590af: 66 89 41 0d                 	movw	%ax, 0xd(%rcx)
1407590b3: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
1407590ba: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
1407590c1: e8 3a 15 3d 00              	callq	0x140b2a600 <.text+0xb29600>
1407590c6: 31 f6                       	xorl	%esi, %esi
1407590c8: 48 85 c0                    	testq	%rax, %rax
1407590cb: 74 53                       	je	0x140759120 <.text+0x758120>
1407590cd: f6 00 01                    	testb	$0x1, (%rax)
1407590d0: 75 4e                       	jne	0x140759120 <.text+0x758120>
1407590d2: 66 0f 6e 50 0c              	movd	0xc(%rax), %xmm2
1407590d7: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
1407590de: f3 0f 10 05 9a 37 32 02     	movss	0x232379a(%rip), %xmm0  # 0x142a7c880
1407590e6: f3 0f 10 0d 96 37 32 02     	movss	0x2323796(%rip), %xmm1  # 0x142a7c884
1407590ee: e8 ad 73 53 00              	callq	0x140c904a0 <.text+0xc8f4a0>
1407590f3: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407590f6: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1407590fa: f3 44 0f 10 3d 01 c8 1c 02  	movss	0x21cc801(%rip), %xmm15 # 0x142925904
140759103: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
140759107: f3 0f 5d c1                 	minss	%xmm1, %xmm0
14075910b: f3 44 0f 5c f8              	subss	%xmm0, %xmm15
140759110: f3 44 0f 59 3d 6f 37 32 02  	mulss	0x232376f(%rip), %xmm15 # 0x142a7c888
140759119: be 01 00 00 00              	movl	$0x1, %esi
14075911e: eb 00                       	jmp	0x140759120 <.text+0x758120>
140759120: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
140759127: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
14075912e: e8 cd 14 3d 00              	callq	0x140b2a600 <.text+0xb29600>
140759133: 31 c9                       	xorl	%ecx, %ecx
140759135: 48 85 c0                    	testq	%rax, %rax
140759138: 74 5a                       	je	0x140759194 <.text+0x758194>
14075913a: f6 00 01                    	testb	$0x1, (%rax)
14075913d: 48 8b 95 00 04 00 00        	movq	0x400(%rbp), %rdx
140759144: 75 55                       	jne	0x14075919b <.text+0x75819b>
140759146: 66 0f 6e 50 0c              	movd	0xc(%rax), %xmm2
14075914b: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
140759152: f3 0f 10 05 26 37 32 02     	movss	0x2323726(%rip), %xmm0  # 0x142a7c880
14075915a: f3 0f 10 0d 16 3f 27 02     	movss	0x2273f16(%rip), %xmm1  # 0x1429cd078
140759162: e8 39 73 53 00              	callq	0x140c904a0 <.text+0xc8f4a0>
140759167: 0f 57 c9                    	xorps	%xmm1, %xmm1
14075916a: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14075916e: f3 0f 10 05 8e c7 1c 02     	movss	0x21cc78e(%rip), %xmm0  # 0x142925904
140759176: 0f 28 d0                    	movaps	%xmm0, %xmm2
140759179: f3 0f 5d d1                 	minss	%xmm1, %xmm2
14075917d: f3 0f 5c c2                 	subss	%xmm2, %xmm0
140759181: f3 0f 59 05 03 37 32 02     	mulss	0x2323703(%rip), %xmm0  # 0x142a7c88c
140759189: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14075918d: b9 01 00 00 00              	movl	$0x1, %ecx
140759192: eb 00                       	jmp	0x140759194 <.text+0x758194>
140759194: 48 8b 95 00 04 00 00        	movq	0x400(%rbp), %rdx
14075919b: 48 8b 85 18 04 00 00        	movq	0x418(%rbp), %rax
1407591a2: 66 0f 6e 48 e4              	movd	-0x1c(%rax), %xmm1
1407591a7: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
1407591ae: 89 8c 24 90 00 00 00        	movl	%ecx, 0x90(%rsp)
1407591b5: 89 b4 24 80 00 00 00        	movl	%esi, 0x80(%rsp)
1407591bc: 4c 89 74 24 58              	movq	%r14, 0x58(%rsp)
1407591c1: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
1407591c8: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1407591cd: 48 8b 85 f0 03 00 00        	movq	0x3f0(%rbp), %rax
1407591d4: 88 44 24 48                 	movb	%al, 0x48(%rsp)
1407591d8: 4c 89 7c 24 40              	movq	%r15, 0x40(%rsp)
1407591dd: 66 0f 7e 8c 24 a0 00 00 00  	movd	%xmm1, 0xa0(%rsp)
1407591e6: f3 0f 11 84 24 98 00 00 00  	movss	%xmm0, 0x98(%rsp)
1407591ef: f3 44 0f 11 bc 24 88 00 00 00       	movss	%xmm15, 0x88(%rsp)
1407591f9: 48 8d 85 f0 02 00 00        	leaq	0x2f0(%rbp), %rax
140759200: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
140759205: f3 0f 11 7c 24 70           	movss	%xmm7, 0x70(%rsp)
14075920b: 48 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %rax
140759212: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
140759217: 66 0f 7e 74 24 60           	movd	%xmm6, 0x60(%rsp)
14075921d: 48 89 5c 24 38              	movq	%rbx, 0x38(%rsp)
140759222: 48 8b 85 d8 03 00 00        	movq	0x3d8(%rbp), %rax
140759229: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14075922e: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
140759233: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14075923a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14075923f: c6 84 24 a8 00 00 00 00     	movb	$0x0, 0xa8(%rsp)
140759247: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14075924e: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140759255: 41 89 f8                    	movl	%edi, %r8d
140759258: 4c 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %r9
14075925f: e8 fc 04 01 00              	callq	0x140769760 <.text+0x768760>
140759264: 80 bd c8 03 00 00 00        	cmpb	$0x0, 0x3c8(%rbp)
14075926b: 4c 8b a5 f8 03 00 00        	movq	0x3f8(%rbp), %r12
140759272: 0f 57 ff                    	xorps	%xmm7, %xmm7
140759275: 48 8b b5 d8 03 00 00        	movq	0x3d8(%rbp), %rsi
14075927c: 74 5c                       	je	0x1407592da <.text+0x7582da>
14075927e: f6 85 30 02 00 00 01        	testb	$0x1, 0x230(%rbp)
140759285: 0f 84 1c 08 00 00           	je	0x140759aa7 <.text+0x758aa7>
14075928b: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
140759292: 48 85 c0                    	testq	%rax, %rax
140759295: 74 1a                       	je	0x1407592b1 <.text+0x7582b1>
140759297: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
14075929e: 48 c1 e0 02                 	shlq	$0x2, %rax
1407592a2: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1407592a6: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407592ac: e8 6f a9 eb ff              	callq	0x140613c20 <.text+0x612c20>
1407592b1: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1407592b8: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
1407592bf: 48 85 c0                    	testq	%rax, %rax
1407592c2: 74 16                       	je	0x1407592da <.text+0x7582da>
1407592c4: 48 8b 09                    	movq	(%rcx), %rcx
1407592c7: 48 c1 e0 02                 	shlq	$0x2, %rax
1407592cb: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1407592cf: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407592d5: e8 46 a9 eb ff              	callq	0x140613c20 <.text+0x612c20>
1407592da: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
1407592e1: 48 85 d2                    	testq	%rdx, %rdx
1407592e4: 45 0f 28 f8                 	movaps	%xmm8, %xmm15
1407592e8: 74 16                       	je	0x140759300 <.text+0x758300>
1407592ea: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1407592f1: 48 c1 e2 03                 	shlq	$0x3, %rdx
1407592f5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407592fb: e8 20 a9 eb ff              	callq	0x140613c20 <.text+0x612c20>
140759300: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
140759307: 48 85 d2                    	testq	%rdx, %rdx
14075930a: 45 0f 28 c6                 	movaps	%xmm14, %xmm8
14075930e: 74 16                       	je	0x140759326 <.text+0x758326>
140759310: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
140759317: 48 c1 e2 02                 	shlq	$0x2, %rdx
14075931b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140759321: e8 fa a8 eb ff              	callq	0x140613c20 <.text+0x612c20>
140759326: 48 8b 95 e0 02 00 00        	movq	0x2e0(%rbp), %rdx
14075932d: 48 85 d2                    	testq	%rdx, %rdx
140759330: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140759334: 45 0f 28 f1                 	movaps	%xmm9, %xmm14
140759338: 74 16                       	je	0x140759350 <.text+0x758350>
14075933a: 48 c1 e2 02                 	shlq	$0x2, %rdx
14075933e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140759344: 48 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %rcx
14075934b: e8 d0 a8 eb ff              	callq	0x140613c20 <.text+0x612c20>
140759350: 48 85 f6                    	testq	%rsi, %rsi
140759353: 74 19                       	je	0x14075936e <.text+0x75836e>
140759355: 48 c1 e6 02                 	shlq	$0x2, %rsi
140759359: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075935f: 48 8b 8d 00 04 00 00        	movq	0x400(%rbp), %rcx
140759366: 48 89 f2                    	movq	%rsi, %rdx
140759369: e8 b2 a8 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075936e: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
140759375: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
14075937c: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
140759383: e8 78 8a 33 00              	callq	0x140a91e00 <.text+0xa90e00>
140759388: 48 89 c2                    	movq	%rax, %rdx
14075938b: 48 c1 ea 20                 	shrq	$0x20, %rdx
14075938f: 0f 85 0b 01 00 00           	jne	0x1407594a0 <.text+0x7584a0>
140759395: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
1407593a0: 48 c7 85 48 01 00 00 04 00 00 00    	movq	$0x4, 0x148(%rbp)
1407593ab: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407593b2: 0f 11 38                    	movups	%xmm7, (%rax)
1407593b5: 48 c7 85 60 01 00 00 04 00 00 00    	movq	$0x4, 0x160(%rbp)
1407593c0: 0f 11 78 18                 	movups	%xmm7, 0x18(%rax)
1407593c4: 48 c7 85 78 01 00 00 04 00 00 00    	movq	$0x4, 0x178(%rbp)
1407593cf: 0f 11 78 30                 	movups	%xmm7, 0x30(%rax)
1407593d3: 48 c7 85 90 01 00 00 04 00 00 00    	movq	$0x4, 0x190(%rbp)
1407593de: 48 c7 85 98 01 00 00 00 00 00 00    	movq	$0x0, 0x198(%rbp)
1407593e9: c6 85 a0 01 00 00 00        	movb	$0x0, 0x1a0(%rbp)
1407593f0: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
1407593f7: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407593fe: 48 8b 95 20 04 00 00        	movq	0x420(%rbp), %rdx
140759405: e8 c6 0a 19 00              	callq	0x1408e9ed0 <.text+0x8e8ed0>
14075940a: 4c 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %r14
140759411: 4c 8d 3d 8d 15 4c 02        	leaq	0x24c158d(%rip), %r15   # 0x142c1a9a5
140759418: 8b 85 88 03 00 00           	movl	0x388(%rbp), %eax
14075941e: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
140759425: 89 01                       	movl	%eax, (%rcx)
140759427: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
14075942e: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
140759435: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
14075943c: e8 7f c5 ac 00              	callq	0x1412259c0 <.text+0x12249c0>
140759441: 48 89 c7                    	movq	%rax, %rdi
140759444: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
14075944b: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
140759452: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140759459: e8 b2 30 16 00              	callq	0x1408bc510 <.text+0x8bb510>
14075945e: f6 85 f0 02 00 00 01        	testb	$0x1, 0x2f0(%rbp)
140759465: 0f 84 f8 01 00 00           	je	0x140759663 <.text+0x758663>
14075946b: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
140759472: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140759479: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
140759480: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
140759487: e8 44 f1 92 ff              	callq	0x1400885d0 <.text+0x875d0>
14075948c: 0d 00 00 00 01              	orl	$0x1000000, %eax        # imm = 0x1000000
140759491: e9 d3 01 00 00              	jmp	0x140759669 <.text+0x758669>
140759496: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407594a0: 89 c1                       	movl	%eax, %ecx
1407594a2: 48 8b bd d0 00 00 00        	movq	0xd0(%rbp), %rdi
1407594a9: 48 3b 4f 10                 	cmpq	0x10(%rdi), %rcx
1407594ad: 4c 8b 9d c8 00 00 00        	movq	0xc8(%rbp), %r11
1407594b4: 0f 83 91 09 00 00           	jae	0x140759e4b <.text+0x758e4b>
1407594ba: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1407594be: 48 c1 e8 20                 	shrq	$0x20, %rax
1407594c2: 4c 8d 0c 89                 	leaq	(%rcx,%rcx,4), %r9
1407594c6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1407594cc: 42 39 04 8a                 	cmpl	%eax, (%rdx,%r9,4)
1407594d0: 0f 85 14 09 00 00           	jne	0x140759dea <.text+0x758dea>
1407594d6: 4e 8d 0c 8a                 	leaq	(%rdx,%r9,4), %r9
1407594da: 41 8b 51 04                 	movl	0x4(%r9), %edx
1407594de: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
1407594e4: 4c 39 d2                    	cmpq	%r10, %rdx
1407594e7: 0f 84 fd 08 00 00           	je	0x140759dea <.text+0x758dea>
1407594ed: 49 39 53 38                 	cmpq	%rdx, 0x38(%r11)
1407594f1: 0f 86 e9 08 00 00           	jbe	0x140759de0 <.text+0x758de0>
1407594f7: 4d 8b 43 28                 	movq	0x28(%r11), %r8
1407594fb: 41 89 d2                    	movl	%edx, %r10d
1407594fe: 41 c1 ea 06                 	shrl	$0x6, %r10d
140759502: 4f 8b 04 d0                 	movq	(%r8,%r10,8), %r8
140759506: 49 0f a3 d0                 	btq	%rdx, %r8
14075950a: 0f 83 d0 08 00 00           	jae	0x140759de0 <.text+0x758de0>
140759510: 41 8b 41 0c                 	movl	0xc(%r9), %eax
140759514: 41 8b 71 10                 	movl	0x10(%r9), %esi
140759518: 48 8b 8f a8 01 00 00        	movq	0x1a8(%rdi), %rcx
14075951f: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
140759523: 49 8b 93 10 01 00 00        	movq	0x110(%r11), %rdx
14075952a: 4d 8b 83 18 01 00 00        	movq	0x118(%r11), %r8
140759531: 4c 8b 4c c1 18              	movq	0x18(%rcx,%rax,8), %r9
140759536: 48 8b 44 c1 38              	movq	0x38(%rcx,%rax,8), %rax
14075953b: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14075953f: 48 f7 d1                    	notq	%rcx
140759542: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
140759546: 48 c1 e2 04                 	shlq	$0x4, %rdx
14075954a: 4d 8b 54 11 28              	movq	0x28(%r9,%rdx), %r10
14075954f: 4a 8b 04 c0                 	movq	(%rax,%r8,8), %rax
140759553: 48 f7 d0                    	notq	%rax
140759556: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14075955a: 48 c1 e0 04                 	shlq	$0x4, %rax
14075955e: 49 8b 5c 01 28              	movq	0x28(%r9,%rax), %rbx
140759563: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
140759567: 48 c1 e1 04                 	shlq	$0x4, %rcx
14075956b: 49 03 4c 11 10              	addq	0x10(%r9,%rdx), %rcx
140759570: 4c 8d 04 b5 00 00 00 00     	leaq	(,%rsi,4), %r8
140759578: 4d 03 44 11 20              	addq	0x20(%r9,%rdx), %r8
14075957d: 4c 89 85 a8 03 00 00        	movq	%r8, 0x3a8(%rbp)
140759584: 48 6b fe 68                 	imulq	$0x68, %rsi, %rdi
140759588: 49 03 7c 01 10              	addq	0x10(%r9,%rax), %rdi
14075958d: 8b 85 8c 03 00 00           	movl	0x38c(%rbp), %eax
140759593: 41 89 04 b2                 	movl	%eax, (%r10,%rsi,4)
140759597: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
14075959e: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
1407595a5: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
1407595ac: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
1407595b3: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
1407595ba: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
1407595c1: e8 6a f2 0f 00              	callq	0x140858830 <.text+0x857830>
1407595c6: 8b 85 8c 03 00 00           	movl	0x38c(%rbp), %eax
1407595cc: 89 04 b3                    	movl	%eax, (%rbx,%rsi,4)
1407595cf: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
1407595d6: 48 89 f9                    	movq	%rdi, %rcx
1407595d9: 48 8b 95 20 04 00 00        	movq	0x420(%rbp), %rdx
1407595e0: e8 eb 08 19 00              	callq	0x1408e9ed0 <.text+0x8e8ed0>
1407595e5: 8b 85 88 03 00 00           	movl	0x388(%rbp), %eax
1407595eb: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
1407595f2: 89 01                       	movl	%eax, (%rcx)
1407595f4: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407595fb: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
140759602: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
140759609: e8 b2 c3 ac 00              	callq	0x1412259c0 <.text+0x12249c0>
14075960e: 48 89 c3                    	movq	%rax, %rbx
140759611: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
140759618: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
14075961f: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140759626: e8 e5 2e 16 00              	callq	0x1408bc510 <.text+0x8bb510>
14075962b: f6 85 f0 02 00 00 01        	testb	$0x1, 0x2f0(%rbp)
140759632: 0f 84 88 03 00 00           	je	0x1407599c0 <.text+0x7589c0>
140759638: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
14075963f: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140759646: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
14075964d: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
140759654: e8 77 ef 92 ff              	callq	0x1400885d0 <.text+0x875d0>
140759659: 0d 00 00 00 01              	orl	$0x1000000, %eax        # imm = 0x1000000
14075965e: e9 63 03 00 00              	jmp	0x1407599c6 <.text+0x7589c6>
140759663: 8b 85 f4 02 00 00           	movl	0x2f4(%rbp), %eax
140759669: 89 47 58                    	movl	%eax, 0x58(%rdi)
14075966c: 48 c7 47 50 00 00 00 00     	movq	$0x0, 0x50(%rdi)
140759674: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
14075967b: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
140759682: e8 59 ea 18 00              	callq	0x1408e80e0 <.text+0x8e70e0>
140759687: 48 83 c7 40                 	addq	$0x40, %rdi
14075968b: 48 8d 0c d2                 	leaq	(%rdx,%rdx,8), %rcx
14075968f: 48 8d 0c 88                 	leaq	(%rax,%rcx,4), %rcx
140759693: 48 c7 85 f0 02 00 00 00 00 00 00    	movq	$0x0, 0x2f0(%rbp)
14075969e: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
1407596a9: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
1407596b0: 48 89 8d 78 03 00 00        	movq	%rcx, 0x378(%rbp)
1407596b7: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
1407596be: 48 89 f9                    	movq	%rdi, %rcx
1407596c1: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
1407596c8: 4c 8d 05 b9 35 32 02        	leaq	0x23235b9(%rip), %r8    # 0x142a7cc88
1407596cf: e8 9c 87 aa 00              	callq	0x141201e70 <.text+0x1200e70>
1407596d4: 8b 85 90 03 00 00           	movl	0x390(%rbp), %eax
1407596da: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
1407596e1: 89 01                       	movl	%eax, (%rcx)
1407596e3: 48 8b 85 e8 03 00 00        	movq	0x3e8(%rbp), %rax
1407596ea: 48 8b 70 30                 	movq	0x30(%rax), %rsi
1407596ee: 48 8b 78 40                 	movq	0x40(%rax), %rdi
1407596f2: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
1407596f6: 75 1a                       	jne	0x140759712 <.text+0x758712>
1407596f8: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
1407596ff: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
140759706: 48 8d 15 1b 3b 4c 02        	leaq	0x24c3b1b(%rip), %rdx   # 0x142c1d228
14075970d: e8 5e 91 e0 ff              	callq	0x140562870 <.text+0x561870>
140759712: 48 8b 8d e8 03 00 00        	movq	0x3e8(%rbp), %rcx
140759719: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14075971d: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
140759721: 48 ff c6                    	incq	%rsi
140759724: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
140759728: 48 ff 41 40                 	incq	0x40(%rcx)
14075972c: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
140759733: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
14075973a: 48 8d 15 37 11 4c 02        	leaq	0x24c1137(%rip), %rdx   # 0x142c1a878
140759741: e8 fa 04 ab 00              	callq	0x141209c40 <.text+0x1208c40>
140759746: 48 89 c3                    	movq	%rax, %rbx
140759749: 48 85 c0                    	testq	%rax, %rax
14075974c: 0f 84 76 06 00 00           	je	0x140759dc8 <.text+0x758dc8>
140759752: 4c 89 bd 30 02 00 00        	movq	%r15, 0x230(%rbp)
140759759: 48 c7 85 38 02 00 00 3a 00 00 00    	movq	$0x3a, 0x238(%rbp)
140759764: 48 89 df                    	movq	%rbx, %rdi
140759767: 48 83 c7 10                 	addq	$0x10, %rdi
14075976b: 48 83 7b 18 3a              	cmpq	$0x3a, 0x18(%rbx)
140759770: 0f 85 0b 06 00 00           	jne	0x140759d81 <.text+0x758d81>
140759776: 48 8b 17                    	movq	(%rdi), %rdx
140759779: 41 b8 3a 00 00 00           	movl	$0x3a, %r8d
14075977f: 4c 89 f9                    	movq	%r15, %rcx
140759782: e8 69 a4 1c 02              	callq	0x142923bf0 <.text+0x2922bf0>
140759787: 85 c0                       	testl	%eax, %eax
140759789: 0f 85 f2 05 00 00           	jne	0x140759d81 <.text+0x758d81>
14075978f: 48 8b 0b                    	movq	(%rbx), %rcx
140759792: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140759796: 48 8b 40 18                 	movq	0x18(%rax), %rax
14075979a: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
1407597a1: ff d0                       	callq	*%rax
1407597a3: 48 89 c7                    	movq	%rax, %rdi
1407597a6: 48 85 c0                    	testq	%rax, %rax
1407597a9: 0f 84 19 06 00 00           	je	0x140759dc8 <.text+0x758dc8>
1407597af: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1407597b6: 48 89 85 90 02 00 00        	movq	%rax, 0x290(%rbp)
1407597bd: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
1407597c4: 0f 29 85 80 02 00 00        	movaps	%xmm0, 0x280(%rbp)
1407597cb: 0f 10 85 80 01 00 00        	movups	0x180(%rbp), %xmm0
1407597d2: 0f 29 85 70 02 00 00        	movaps	%xmm0, 0x270(%rbp)
1407597d9: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1407597e0: f3 0f 6f 8d 50 01 00 00     	movdqu	0x150(%rbp), %xmm1
1407597e8: f3 0f 6f 95 60 01 00 00     	movdqu	0x160(%rbp), %xmm2
1407597f0: f3 0f 6f 9d 70 01 00 00     	movdqu	0x170(%rbp), %xmm3
1407597f8: 66 0f 7f 9d 60 02 00 00     	movdqa	%xmm3, 0x260(%rbp)
140759800: 66 0f 7f 95 50 02 00 00     	movdqa	%xmm2, 0x250(%rbp)
140759808: 66 0f 7f 8d 40 02 00 00     	movdqa	%xmm1, 0x240(%rbp)
140759810: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
140759817: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
14075981e: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140759825: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
14075982c: 0f 29 85 a0 00 00 00        	movaps	%xmm0, 0xa0(%rbp)
140759833: 0f b6 05 cf b3 ed 02        	movzbl	0x2edb3cf(%rip), %eax   # 0x143634c09
14075983a: b9 20 00 00 00              	movl	$0x20, %ecx
14075983f: ba 08 00 00 00              	movl	$0x8, %edx
140759844: e8 c7 a3 eb ff              	callq	0x140613c10 <.text+0x612c10>
140759849: 48 85 c0                    	testq	%rax, %rax
14075984c: 0f 84 b6 06 00 00           	je	0x140759f08 <.text+0x758f08>
140759852: 48 89 c1                    	movq	%rax, %rcx
140759855: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
14075985c: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
140759864: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
14075986c: c6 40 18 00                 	movb	$0x0, 0x18(%rax)
140759870: 4c 8b 4f 60                 	movq	0x60(%rdi), %r9
140759874: eb 0c                       	jmp	0x140759882 <.text+0x758882>
140759876: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140759880: f3 90                       	pause
140759882: 49 8b 41 08                 	movq	0x8(%r9), %rax
140759886: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140759890: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140759894: 74 ea                       	je	0x140759880 <.text+0x758880>
140759896: 48 85 c0                    	testq	%rax, %rax
140759899: 0f 88 35 04 00 00           	js	0x140759cd4 <.text+0x758cd4>
14075989f: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
1407598a3: f0                          	lock
1407598a4: 49 0f b1 51 08              	cmpxchgq	%rdx, 0x8(%r9)
1407598a9: 75 e5                       	jne	0x140759890 <.text+0x758890>
1407598ab: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407598b2: c6 85 30 04 00 00 01        	movb	$0x1, 0x430(%rbp)
1407598b9: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1407598be: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
1407598c5: 4c 8d 85 a0 00 00 00        	leaq	0xa0(%rbp), %r8
1407598cc: e8 2f ef 0f 00              	callq	0x140858800 <.text+0x857800>
1407598d1: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
1407598d8: 48 8d 8d f8 02 00 00        	leaq	0x2f8(%rbp), %rcx
1407598df: 48 89 81 88 00 00 00        	movq	%rax, 0x88(%rcx)
1407598e6: 0f 28 85 80 02 00 00        	movaps	0x280(%rbp), %xmm0
1407598ed: 0f 11 41 78                 	movups	%xmm0, 0x78(%rcx)
1407598f1: 0f 28 85 70 02 00 00        	movaps	0x270(%rbp), %xmm0
1407598f8: 0f 11 41 68                 	movups	%xmm0, 0x68(%rcx)
1407598fc: 0f 28 85 30 02 00 00        	movaps	0x230(%rbp), %xmm0
140759903: 0f 28 8d 40 02 00 00        	movaps	0x240(%rbp), %xmm1
14075990a: 0f 28 95 50 02 00 00        	movaps	0x250(%rbp), %xmm2
140759911: 66 0f 6f 9d 60 02 00 00     	movdqa	0x260(%rbp), %xmm3
140759919: f3 0f 7f 59 58              	movdqu	%xmm3, 0x58(%rcx)
14075991e: 0f 11 51 48                 	movups	%xmm2, 0x48(%rcx)
140759922: 0f 11 49 38                 	movups	%xmm1, 0x38(%rcx)
140759926: 0f 11 41 28                 	movups	%xmm0, 0x28(%rcx)
14075992a: 0f 10 85 10 01 00 00        	movups	0x110(%rbp), %xmm0
140759931: f3 0f 6f 8d 20 01 00 00     	movdqu	0x120(%rbp), %xmm1
140759939: f3 0f 6f 95 30 01 00 00     	movdqu	0x130(%rbp), %xmm2
140759941: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
140759948: 66 0f 7f 8d 00 03 00 00     	movdqa	%xmm1, 0x300(%rbp)
140759950: 66 0f 7f 95 10 03 00 00     	movdqa	%xmm2, 0x310(%rbp)
140759958: c6 85 30 04 00 00 00        	movb	$0x0, 0x430(%rbp)
14075995f: 48 8d 4d 78                 	leaq	0x78(%rbp), %rcx
140759963: 48 8b 95 70 05 00 00        	movq	0x570(%rbp), %rdx
14075996a: 4c 8d 85 f0 02 00 00        	leaq	0x2f0(%rbp), %r8
140759971: 4c 8d 0d b8 33 32 02        	leaq	0x23233b8(%rip), %r9    # 0x142a7cd30
140759978: e8 d3 5b ab 00              	callq	0x14120f550 <.text+0x120e550>
14075997d: 4c 8b 85 98 00 00 00        	movq	0x98(%rbp), %r8
140759984: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14075998b: 8b 8d a4 02 00 00           	movl	0x2a4(%rbp), %ecx
140759991: 89 08                       	movl	%ecx, (%rax)
140759993: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
14075999a: c6 85 35 04 00 00 00        	movb	$0x0, 0x435(%rbp)
1407599a1: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1407599a8: e8 c3 2f 79 00              	callq	0x140eec970 <.text+0xeeb970>
1407599ad: 48 8b 9d 00 02 00 00        	movq	0x200(%rbp), %rbx
1407599b4: 4c 8b bd d0 01 00 00        	movq	0x1d0(%rbp), %r15
1407599bb: e9 7d e6 ff ff              	jmp	0x14075803d <.text+0x75703d>
1407599c0: 8b 85 f4 02 00 00           	movl	0x2f4(%rbp), %eax
1407599c6: 89 43 58                    	movl	%eax, 0x58(%rbx)
1407599c9: 48 c7 43 50 00 00 00 00     	movq	$0x0, 0x50(%rbx)
1407599d1: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
1407599d8: 48 89 f9                    	movq	%rdi, %rcx
1407599db: e8 00 e7 18 00              	callq	0x1408e80e0 <.text+0x8e70e0>
1407599e0: 48 83 c3 40                 	addq	$0x40, %rbx
1407599e4: 48 8d 0c d2                 	leaq	(%rdx,%rdx,8), %rcx
1407599e8: 48 8d 0c 88                 	leaq	(%rax,%rcx,4), %rcx
1407599ec: 48 c7 85 f0 02 00 00 00 00 00 00    	movq	$0x0, 0x2f0(%rbp)
1407599f7: 48 c7 85 30 03 00 00 00 00 00 00    	movq	$0x0, 0x330(%rbp)
140759a02: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
140759a09: 48 89 8d 78 03 00 00        	movq	%rcx, 0x378(%rbp)
140759a10: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
140759a17: 48 89 d9                    	movq	%rbx, %rcx
140759a1a: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140759a21: 4c 8d 05 60 32 32 02        	leaq	0x2323260(%rip), %r8    # 0x142a7cc88
140759a28: e8 43 84 aa 00              	callq	0x141201e70 <.text+0x1200e70>
140759a2d: 8b 85 90 03 00 00           	movl	0x390(%rbp), %eax
140759a33: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
140759a3a: 89 01                       	movl	%eax, (%rcx)
140759a3c: 48 8b 85 e8 03 00 00        	movq	0x3e8(%rbp), %rax
140759a43: 48 8b 70 30                 	movq	0x30(%rax), %rsi
140759a47: 48 8b 78 40                 	movq	0x40(%rax), %rdi
140759a4b: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
140759a4f: 75 1a                       	jne	0x140759a6b <.text+0x758a6b>
140759a51: c6 85 37 04 00 00 00        	movb	$0x0, 0x437(%rbp)
140759a58: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
140759a5f: 48 8d 15 c2 37 4c 02        	leaq	0x24c37c2(%rip), %rdx   # 0x142c1d228
140759a66: e8 05 8e e0 ff              	callq	0x140562870 <.text+0x561870>
140759a6b: 48 c1 ad a8 03 00 00 20     	shrq	$0x20, 0x3a8(%rbp)
140759a73: 48 8b 8d e8 03 00 00        	movq	0x3e8(%rbp), %rcx
140759a7a: 48 8b 41 28                 	movq	0x28(%rcx), %rax
140759a7e: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
140759a82: 48 ff c6                    	incq	%rsi
140759a85: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
140759a89: 48 ff 41 40                 	incq	0x40(%rcx)
140759a8d: 48 8b 9d 00 02 00 00        	movq	0x200(%rbp), %rbx
140759a94: 4c 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %r14
140759a9b: 4c 8b bd d0 01 00 00        	movq	0x1d0(%rbp), %r15
140759aa2: e9 96 e5 ff ff              	jmp	0x14075803d <.text+0x75703d>
140759aa7: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
140759aae: 48 85 c0                    	testq	%rax, %rax
140759ab1: 74 1a                       	je	0x140759acd <.text+0x758acd>
140759ab3: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
140759aba: 48 c1 e0 02                 	shlq	$0x2, %rax
140759abe: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
140759ac2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140759ac8: e8 53 a1 eb ff              	callq	0x140613c20 <.text+0x612c20>
140759acd: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
140759ad4: 48 8d 8d 58 02 00 00        	leaq	0x258(%rbp), %rcx
140759adb: 48 85 c0                    	testq	%rax, %rax
140759ade: 0f 85 e0 f7 ff ff           	jne	0x1407592c4 <.text+0x7582c4>
140759ae4: e9 f1 f7 ff ff              	jmp	0x1407592da <.text+0x7582da>
140759ae9: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140759aef: 4c 01 ca                    	addq	%r9, %rdx
140759af2: 48 21 ca                    	andq	%rcx, %rdx
140759af5: f3 0f 6f 04 10              	movdqu	(%rax,%rdx), %xmm0
140759afa: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
140759aff: 49 83 c1 10                 	addq	$0x10, %r9
140759b03: 45 85 c0                    	testl	%r8d, %r8d
140759b06: 0f 85 dc ee ff ff           	jne	0x1407589e8 <.text+0x7579e8>
140759b0c: eb e1                       	jmp	0x140759aef <.text+0x758aef>
140759b0e: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140759b14: 4c 01 ca                    	addq	%r9, %rdx
140759b17: 48 21 ca                    	andq	%rcx, %rdx
140759b1a: f3 0f 6f 04 10              	movdqu	(%rax,%rdx), %xmm0
140759b1f: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
140759b24: 49 83 c1 10                 	addq	$0x10, %r9
140759b28: 45 85 c0                    	testl	%r8d, %r8d
140759b2b: 0f 85 e5 ef ff ff           	jne	0x140758b16 <.text+0x757b16>
140759b31: eb e1                       	jmp	0x140759b14 <.text+0x758b14>
140759b33: c6 85 34 04 00 00 00        	movb	$0x0, 0x434(%rbp)
140759b3a: ba 01 00 00 00              	movl	$0x1, %edx
140759b3f: 48 8b 8d 10 04 00 00        	movq	0x410(%rbp), %rcx
140759b46: 4c 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %r8
140759b4d: 41 b1 01                    	movb	$0x1, %r9b
140759b50: e8 6b e5 15 02              	callq	0x1428b80c0 <.text+0x28b70c0>
140759b55: e9 4d ee ff ff              	jmp	0x1407589a7 <.text+0x7579a7>
140759b5a: c6 85 34 04 00 00 00        	movb	$0x0, 0x434(%rbp)
140759b61: ba 01 00 00 00              	movl	$0x1, %edx
140759b66: 48 8b 8d 10 04 00 00        	movq	0x410(%rbp), %rcx
140759b6d: 4c 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %r8
140759b74: 41 b1 01                    	movb	$0x1, %r9b
140759b77: e8 44 e5 15 02              	callq	0x1428b80c0 <.text+0x28b70c0>
140759b7c: e9 54 ef ff ff              	jmp	0x140758ad5 <.text+0x757ad5>
140759b81: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
140759b85: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
140759b89: f3 44 0f bc c2              	tzcntl	%edx, %r8d
140759b8e: 42 0f b6 14 00              	movzbl	(%rax,%r8), %edx
140759b93: e9 68 ee ff ff              	jmp	0x140758a00 <.text+0x757a00>
140759b98: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
140759b9c: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
140759ba0: f3 44 0f bc c2              	tzcntl	%edx, %r8d
140759ba5: 46 0f b6 0c 00              	movzbl	(%rax,%r8), %r9d
140759baa: e9 80 ef ff ff              	jmp	0x140758b2f <.text+0x757b2f>
140759baf: 48 8d 0d 42 30 32 02        	leaq	0x2323042(%rip), %rcx   # 0x142a7cbf8
140759bb6: e8 55 f7 17 02              	callq	0x1428d9310 <.text+0x28d8310>
140759bbb: e9 dd 03 00 00              	jmp	0x140759f9d <.text+0x758f9d>
140759bc0: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
140759bc7: 48 85 d2                    	testq	%rdx, %rdx
140759bca: 74 16                       	je	0x140759be2 <.text+0x758be2>
140759bcc: 48 c1 e2 03                 	shlq	$0x3, %rdx
140759bd0: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140759bd6: 48 8b 8d 08 04 00 00        	movq	0x408(%rbp), %rcx
140759bdd: e8 3e a0 eb ff              	callq	0x140613c20 <.text+0x612c20>
140759be2: 48 8b 95 b0 01 00 00        	movq	0x1b0(%rbp), %rdx
140759be9: 48 85 d2                    	testq	%rdx, %rdx
140759bec: 74 16                       	je	0x140759c04 <.text+0x758c04>
140759bee: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140759bf5: 48 c1 e2 04                 	shlq	$0x4, %rdx
140759bf9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140759bff: e8 1c a0 eb ff              	callq	0x140613c20 <.text+0x612c20>
140759c04: 80 bd 31 04 00 00 00        	cmpb	$0x0, 0x431(%rbp)
140759c0b: 74 4b                       	je	0x140759c58 <.text+0x758c58>
140759c0d: e8 1e e5 4b 00              	callq	0x140c18130 <.text+0xc17130>
140759c12: 48 89 c1                    	movq	%rax, %rcx
140759c15: 48 8b 00                    	movq	(%rax), %rax
140759c18: 48 83 f8 01                 	cmpq	$0x1, %rax
140759c1c: 0f 85 d1 00 00 00           	jne	0x140759cf3 <.text+0x758cf3>
140759c22: 48 83 c1 08                 	addq	$0x8, %rcx
140759c26: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
140759c2d: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140759c31: 0f 85 85 01 00 00           	jne	0x140759dbc <.text+0x758dbc>
140759c37: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140759c3e: 48 89 8d f8 03 00 00        	movq	%rcx, 0x3f8(%rbp)
140759c45: 48 83 c1 08                 	addq	$0x8, %rcx
140759c49: e8 a2 d4 4b 00              	callq	0x140c170f0 <.text+0xc160f0>
140759c4e: 48 8b 85 f8 03 00 00        	movq	0x3f8(%rbp), %rax
140759c55: 48 ff 00                    	incq	(%rax)
140759c58: 0f 28 b5 40 04 00 00        	movaps	0x440(%rbp), %xmm6
140759c5f: 0f 28 bd 50 04 00 00        	movaps	0x450(%rbp), %xmm7
140759c66: 44 0f 28 85 60 04 00 00     	movaps	0x460(%rbp), %xmm8
140759c6e: 44 0f 28 8d 70 04 00 00     	movaps	0x470(%rbp), %xmm9
140759c76: 44 0f 28 95 80 04 00 00     	movaps	0x480(%rbp), %xmm10
140759c7e: 44 0f 28 9d 90 04 00 00     	movaps	0x490(%rbp), %xmm11
140759c86: 44 0f 28 a5 a0 04 00 00     	movaps	0x4a0(%rbp), %xmm12
140759c8e: 44 0f 28 ad b0 04 00 00     	movaps	0x4b0(%rbp), %xmm13
140759c96: 44 0f 28 b5 c0 04 00 00     	movaps	0x4c0(%rbp), %xmm14
140759c9e: 44 0f 28 bd d0 04 00 00     	movaps	0x4d0(%rbp), %xmm15
140759ca6: 48 81 c4 68 05 00 00        	addq	$0x568, %rsp            # imm = 0x568
140759cad: 5b                          	popq	%rbx
140759cae: 5f                          	popq	%rdi
140759caf: 5e                          	popq	%rsi
140759cb0: 41 5c                       	popq	%r12
140759cb2: 41 5d                       	popq	%r13
140759cb4: 41 5e                       	popq	%r14
140759cb6: 41 5f                       	popq	%r15
140759cb8: 5d                          	popq	%rbp
140759cb9: c3                          	retq
140759cba: 48 85 c0                    	testq	%rax, %rax
140759cbd: 75 53                       	jne	0x140759d12 <.text+0x758d12>
140759cbf: 31 d2                       	xorl	%edx, %edx
140759cc1: e8 fa 78 19 02              	callq	0x1428f15c0 <.text+0x28f05c0>
140759cc6: 48 89 c1                    	movq	%rax, %rcx
140759cc9: 48 85 c0                    	testq	%rax, %rax
140759ccc: 0f 85 e8 dd ff ff           	jne	0x140757aba <.text+0x756aba>
140759cd2: eb 3e                       	jmp	0x140759d12 <.text+0x758d12>
140759cd4: c6 85 32 04 00 00 01        	movb	$0x1, 0x432(%rbp)
140759cdb: 48 8d 0d 76 35 4c 02        	leaq	0x24c3576(%rip), %rcx   # 0x142c1d258
140759ce2: 48 8d 15 cf 35 4c 02        	leaq	0x24c35cf(%rip), %rdx   # 0x142c1d2b8
140759ce9: e8 12 fd 14 02              	callq	0x1428a9a00 <.text+0x28a8a00>
140759cee: e9 aa 02 00 00              	jmp	0x140759f9d <.text+0x758f9d>
140759cf3: 48 85 c0                    	testq	%rax, %rax
140759cf6: 75 1a                       	jne	0x140759d12 <.text+0x758d12>
140759cf8: 31 d2                       	xorl	%edx, %edx
140759cfa: e8 c1 78 19 02              	callq	0x1428f15c0 <.text+0x28f05c0>
140759cff: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
140759d06: 48 89 c1                    	movq	%rax, %rcx
140759d09: 48 85 c0                    	testq	%rax, %rax
140759d0c: 0f 85 1b ff ff ff           	jne	0x140759c2d <.text+0x758c2d>
140759d12: 48 8d 0d 8f f7 4b 02        	leaq	0x24bf78f(%rip), %rcx   # 0x142c194a8
140759d19: e8 b2 e3 17 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140759d1e: 48 83 fa 15                 	cmpq	$0x15, %rdx
140759d22: 0f 83 77 02 00 00           	jae	0x140759f9f <.text+0x758f9f>
140759d28: 4c 8b 9d 08 04 00 00        	movq	0x408(%rbp), %r11
140759d2f: 49 8d 04 d3                 	leaq	(%r11,%rdx,8), %rax
140759d33: 49 8d 4b 08                 	leaq	0x8(%r11), %rcx
140759d37: ba 08 00 00 00              	movl	$0x8, %edx
140759d3c: eb 17                       	jmp	0x140759d55 <.text+0x758d55>
140759d3e: 4d 89 d9                    	movq	%r11, %r9
140759d41: 4d 89 01                    	movq	%r8, (%r9)
140759d44: 48 83 c1 08                 	addq	$0x8, %rcx
140759d48: 48 83 c2 08                 	addq	$0x8, %rdx
140759d4c: 48 39 c1                    	cmpq	%rax, %rcx
140759d4f: 0f 84 64 02 00 00           	je	0x140759fb9 <.text+0x758fb9>
140759d55: 4c 8b 51 f8                 	movq	-0x8(%rcx), %r10
140759d59: 4c 8b 01                    	movq	(%rcx), %r8
140759d5c: 4d 39 d0                    	cmpq	%r10, %r8
140759d5f: 73 e3                       	jae	0x140759d44 <.text+0x758d44>
140759d61: 49 89 d1                    	movq	%rdx, %r9
140759d64: 4f 89 14 0b                 	movq	%r10, (%r11,%r9)
140759d68: 49 83 f9 08                 	cmpq	$0x8, %r9
140759d6c: 74 d0                       	je	0x140759d3e <.text+0x758d3e>
140759d6e: 4f 8b 54 0b f0              	movq	-0x10(%r11,%r9), %r10
140759d73: 49 83 c1 f8                 	addq	$-0x8, %r9
140759d77: 4d 39 d0                    	cmpq	%r10, %r8
140759d7a: 72 e8                       	jb	0x140759d64 <.text+0x758d64>
140759d7c: 4d 01 d9                    	addq	%r11, %r9
140759d7f: eb c0                       	jmp	0x140759d41 <.text+0x758d41>
140759d81: 48 c7 85 f0 02 00 00 00 00 00 00    	movq	$0x0, 0x2f0(%rbp)
140759d8c: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
140759d93: 48 8d 05 c6 0a 4c 02        	leaq	0x24c0ac6(%rip), %rax   # 0x142c1a860
140759d9a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140759d9f: 48 8d 95 30 02 00 00        	leaq	0x230(%rbp), %rdx
140759da6: 4c 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %r9
140759dad: 31 c9                       	xorl	%ecx, %ecx
140759daf: 49 89 f8                    	movq	%rdi, %r8
140759db2: e8 8a 0d 17 02              	callq	0x1428cab41 <.text+0x28c9b41>
140759db7: e9 e1 01 00 00              	jmp	0x140759f9d <.text+0x758f9d>
140759dbc: 48 8d 0d 55 f7 4b 02        	leaq	0x24bf755(%rip), %rcx   # 0x142c19518
140759dc3: e8 88 f4 17 02              	callq	0x1428d9250 <.text+0x28d8250>
140759dc8: c6 85 35 04 00 00 01        	movb	$0x1, 0x435(%rbp)
140759dcf: 48 8d 0d 6a 2e 32 02        	leaq	0x2322e6a(%rip), %rcx   # 0x142a7cc40
140759dd6: e8 35 f5 17 02              	callq	0x1428d9310 <.text+0x28d8310>
140759ddb: e9 bd 01 00 00              	jmp	0x140759f9d <.text+0x758f9d>
140759de0: 48 89 95 a8 03 00 00        	movq	%rdx, 0x3a8(%rbp)
140759de7: 45 31 c0                    	xorl	%r8d, %r8d
140759dea: 48 89 c2                    	movq	%rax, %rdx
140759ded: eb 62                       	jmp	0x140759e51 <.text+0x758e51>
140759def: 48 c1 e2 20                 	shlq	$0x20, %rdx
140759df3: 4c 09 ca                    	orq	%r9, %rdx
140759df6: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140759dfc: 48 89 d0                    	movq	%rdx, %rax
140759dff: 44 89 8d f0 02 00 00        	movl	%r9d, 0x2f0(%rbp)
140759e06: 44 89 85 f4 02 00 00        	movl	%r8d, 0x2f4(%rbp)
140759e0d: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
140759e14: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140759e1b: 48 8d 05 ee 2d 32 02        	leaq	0x2322dee(%rip), %rax   # 0x142a7cc10
140759e22: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140759e27: 48 8d 0d a2 2b 32 02        	leaq	0x2322ba2(%rip), %rcx   # 0x142a7c9d0
140759e2e: 4c 8d 0d 7b 2b 32 02        	leaq	0x2322b7b(%rip), %r9    # 0x142a7c9b0
140759e35: 4c 8d 85 f0 02 00 00        	leaq	0x2f0(%rbp), %r8
140759e3c: ba 2b 00 00 00              	movl	$0x2b, %edx
140759e41: e8 1a f9 17 02              	callq	0x1428d9760 <.text+0x28d8760>
140759e46: e9 52 01 00 00              	jmp	0x140759f9d <.text+0x758f9d>
140759e4b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140759e51: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
140759e58: 48 c1 e0 20                 	shlq	$0x20, %rax
140759e5c: 4c 09 c0                    	orq	%r8, %rax
140759e5f: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140759e66: 48 c1 e2 20                 	shlq	$0x20, %rdx
140759e6a: 48 09 ca                    	orq	%rcx, %rdx
140759e6d: 48 89 95 48 01 00 00        	movq	%rdx, 0x148(%rbp)
140759e74: 48 8d 85 28 04 00 00        	leaq	0x428(%rbp), %rax
140759e7b: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140759e82: 48 8d 05 37 b0 3d 00        	leaq	0x3db037(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
140759e89: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
140759e90: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140759e97: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
140759e9e: 48 8d 05 9b d8 ff ff        	leaq	-0x2765(%rip), %rax     # 0x140757740 <.text+0x756740>
140759ea5: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
140759eac: 48 8d 05 45 2e 32 02        	leaq	0x2322e45(%rip), %rax   # 0x142a7ccf8
140759eb3: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
140759eba: 48 c7 85 f8 02 00 00 02 00 00 00    	movq	$0x2, 0x2f8(%rbp)
140759ec5: 48 c7 85 10 03 00 00 00 00 00 00    	movq	$0x0, 0x310(%rbp)
140759ed0: 48 8d 85 30 02 00 00        	leaq	0x230(%rbp), %rax
140759ed7: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
140759ede: 48 c7 85 08 03 00 00 02 00 00 00    	movq	$0x2, 0x308(%rbp)
140759ee9: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140759ef0: 48 8d 15 21 2e 32 02        	leaq	0x2322e21(%rip), %rdx   # 0x142a7cd18
140759ef7: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140759efe: e8 8d f4 17 02              	callq	0x1428d9390 <.text+0x28d8390>
140759f03: e9 95 00 00 00              	jmp	0x140759f9d <.text+0x758f9d>
140759f08: c6 85 32 04 00 00 01        	movb	$0x1, 0x432(%rbp)
140759f0f: b9 08 00 00 00              	movl	$0x8, %ecx
140759f14: ba 20 00 00 00              	movl	$0x20, %edx
140759f19: e8 e2 f0 17 02              	callq	0x1428d9000 <.text+0x28d8000>
140759f1e: eb 7d                       	jmp	0x140759f9d <.text+0x758f9d>
140759f20: f3 0f 6f 85 f8 02 00 00     	movdqu	0x2f8(%rbp), %xmm0
140759f28: 66 0f 7f 85 30 02 00 00     	movdqa	%xmm0, 0x230(%rbp)
140759f30: 48 8d 05 41 f4 4b 02        	leaq	0x24bf441(%rip), %rax   # 0x142c19378
140759f37: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140759f3c: 48 8d 0d 85 ec 4b 02        	leaq	0x24bec85(%rip), %rcx   # 0x142c18bc8
140759f43: 4c 8d 0d 5e ec 4b 02        	leaq	0x24bec5e(%rip), %r9    # 0x142c18ba8
140759f4a: 4c 8d 85 30 02 00 00        	leaq	0x230(%rbp), %r8
140759f51: ba 2b 00 00 00              	movl	$0x2b, %edx
140759f56: e8 05 f8 17 02              	callq	0x1428d9760 <.text+0x28d8760>
140759f5b: eb 40                       	jmp	0x140759f9d <.text+0x758f9d>
140759f5d: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
140759f64: 4c 8d 05 2d 2a 32 02        	leaq	0x2322a2d(%rip), %r8    # 0x142a7c998
140759f6b: b9 04 00 00 00              	movl	$0x4, %ecx
140759f70: 48 89 da                    	movq	%rbx, %rdx
140759f73: e8 6b f0 17 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140759f78: eb 23                       	jmp	0x140759f9d <.text+0x758f9d>
140759f7a: 48 8d 0d a7 2c 32 02        	leaq	0x2322ca7(%rip), %rcx   # 0x142a7cc28
140759f81: e8 8a f3 17 02              	callq	0x1428d9310 <.text+0x28d8310>
140759f86: eb 15                       	jmp	0x140759f9d <.text+0x758f9d>
140759f88: 45 31 ff                    	xorl	%r15d, %r15d
140759f8b: 4c 8d 05 de ff 4b 02        	leaq	0x24bffde(%rip), %r8    # 0x142c19f70
140759f92: 4c 89 f9                    	movq	%r15, %rcx
140759f95: 4c 89 e2                    	movq	%r12, %rdx
140759f98: e8 46 f0 17 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140759f9d: 0f 0b                       	ud2
140759f9f: c6 85 33 04 00 00 01        	movb	$0x1, 0x433(%rbp)
140759fa6: 4c 8d 85 f0 02 00 00        	leaq	0x2f0(%rbp), %r8
140759fad: 48 8b 8d 08 04 00 00        	movq	0x408(%rbp), %rcx
140759fb4: e8 a7 ed 1f 00              	callq	0x140958d60 <.text+0x957d60>
140759fb9: 48 8b 85 b8 03 00 00        	movq	0x3b8(%rbp), %rax
140759fc0: 48 89 85 08 04 00 00        	movq	%rax, 0x408(%rbp)
140759fc7: 48 8b 95 c0 03 00 00        	movq	0x3c0(%rbp), %rdx
140759fce: 48 83 fa 02                 	cmpq	$0x2, %rdx
140759fd2: 73 0c                       	jae	0x140759fe0 <.text+0x758fe0>
140759fd4: 4c 8b a5 f8 03 00 00        	movq	0x3f8(%rbp), %r12
140759fdb: e9 b6 de ff ff              	jmp	0x140757e96 <.text+0x756e96>
140759fe0: b8 01 00 00 00              	movl	$0x1, %eax
140759fe5: 4c 8b a5 f8 03 00 00        	movq	0x3f8(%rbp), %r12
140759fec: 4c 8b 95 e8 03 00 00        	movq	0x3e8(%rbp), %r10
140759ff3: 4c 8b 85 08 04 00 00        	movq	0x408(%rbp), %r8
140759ffa: 49 8b 0c c0                 	movq	(%r8,%rax,8), %rcx
140759ffe: 49 3b 4c c0 f8              	cmpq	-0x8(%r8,%rax,8), %rcx
14075a003: 74 0d                       	je	0x14075a012 <.text+0x759012>
14075a005: 48 ff c0                    	incq	%rax
14075a008: 48 39 c2                    	cmpq	%rax, %rdx
14075a00b: 75 ed                       	jne	0x140759ffa <.text+0x758ffa>
14075a00d: e9 8b de ff ff              	jmp	0x140757e9d <.text+0x756e9d>
14075a012: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
14075a016: 48 39 d1                    	cmpq	%rdx, %rcx
14075a019: 73 33                       	jae	0x14075a04e <.text+0x75904e>
14075a01b: 4c 8b 8d 08 04 00 00        	movq	0x408(%rbp), %r9
14075a022: eb 08                       	jmp	0x14075a02c <.text+0x75902c>
14075a024: 48 ff c1                    	incq	%rcx
14075a027: 48 39 d1                    	cmpq	%rdx, %rcx
14075a02a: 73 14                       	jae	0x14075a040 <.text+0x759040>
14075a02c: 4d 8b 04 c9                 	movq	(%r9,%rcx,8), %r8
14075a030: 4d 3b 44 c1 f8              	cmpq	-0x8(%r9,%rax,8), %r8
14075a035: 74 ed                       	je	0x14075a024 <.text+0x759024>
14075a037: 4d 89 04 c1                 	movq	%r8, (%r9,%rax,8)
14075a03b: 48 ff c0                    	incq	%rax
14075a03e: eb e4                       	jmp	0x14075a024 <.text+0x759024>
14075a040: 48 8b 8d b8 03 00 00        	movq	0x3b8(%rbp), %rcx
14075a047: 48 89 8d 08 04 00 00        	movq	%rcx, 0x408(%rbp)
14075a04e: 48 89 85 c0 03 00 00        	movq	%rax, 0x3c0(%rbp)
14075a055: 48 89 c2                    	movq	%rax, %rdx
14075a058: e9 40 de ff ff              	jmp	0x140757e9d <.text+0x756e9d>
14075a05d: 0f 1f 00                    	nopl	(%rax)
14075a060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a065: 55                          	pushq	%rbp
14075a066: 41 57                       	pushq	%r15
14075a068: 41 56                       	pushq	%r14
14075a06a: 41 55                       	pushq	%r13
14075a06c: 41 54                       	pushq	%r12
14075a06e: 56                          	pushq	%rsi
14075a06f: 57                          	pushq	%rdi
14075a070: 53                          	pushq	%rbx
14075a071: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a078: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a07f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a088: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a091: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075a09b: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075a0a5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075a0af: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a0b8: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a0c1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a0ca: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a0d2: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a0db: 48 8b 85 f8 03 00 00        	movq	0x3f8(%rbp), %rax
14075a0e2: 48 ff 00                    	incq	(%rax)
14075a0e5: 0f 28 b4 24 40 01 00 00     	movaps	0x140(%rsp), %xmm6
14075a0ed: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a0f5: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a0fe: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a107: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a110: 44 0f 28 9c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm11
14075a119: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
14075a122: 44 0f 28 ac 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm13
14075a12b: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a134: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a13d: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a144: 5b                          	popq	%rbx
14075a145: 5f                          	popq	%rdi
14075a146: 5e                          	popq	%rsi
14075a147: 41 5c                       	popq	%r12
14075a149: 41 5d                       	popq	%r13
14075a14b: 41 5e                       	popq	%r14
14075a14d: 41 5f                       	popq	%r15
14075a14f: 5d                          	popq	%rbp
14075a150: c3                          	retq
14075a151: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14075a160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a165: 55                          	pushq	%rbp
14075a166: 41 57                       	pushq	%r15
14075a168: 41 56                       	pushq	%r14
14075a16a: 41 55                       	pushq	%r13
14075a16c: 41 54                       	pushq	%r12
14075a16e: 56                          	pushq	%rsi
14075a16f: 57                          	pushq	%rdi
14075a170: 53                          	pushq	%rbx
14075a171: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a178: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a17f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a188: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a191: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075a19b: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075a1a5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075a1af: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a1b8: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a1c1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a1ca: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a1d2: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a1db: 48 83 bd d0 03 00 00 00     	cmpq	$0x0, 0x3d0(%rbp)
14075a1e3: 74 1d                       	je	0x14075a202 <.text+0x759202>
14075a1e5: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
14075a1ec: 48 c1 e2 06                 	shlq	$0x6, %rdx
14075a1f0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075a1f6: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
14075a1fd: e8 1e 9a eb ff              	callq	0x140613c20 <.text+0x612c20>
14075a202: c6 85 36 04 00 00 00        	movb	$0x0, 0x436(%rbp)
14075a209: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a212: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a21a: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a223: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a22c: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a235: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075a23f: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075a249: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075a253: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a25c: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a265: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a26c: 5b                          	popq	%rbx
14075a26d: 5f                          	popq	%rdi
14075a26e: 5e                          	popq	%rsi
14075a26f: 41 5c                       	popq	%r12
14075a271: 41 5d                       	popq	%r13
14075a273: 41 5e                       	popq	%r14
14075a275: 41 5f                       	popq	%r15
14075a277: 5d                          	popq	%rbp
14075a278: c3                          	retq
14075a279: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14075a280: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a285: 55                          	pushq	%rbp
14075a286: 41 57                       	pushq	%r15
14075a288: 41 56                       	pushq	%r14
14075a28a: 41 55                       	pushq	%r13
14075a28c: 41 54                       	pushq	%r12
14075a28e: 56                          	pushq	%rsi
14075a28f: 57                          	pushq	%rdi
14075a290: 53                          	pushq	%rbx
14075a291: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a298: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a29f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a2a8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a2b1: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075a2bb: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075a2c5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075a2cf: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a2d8: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a2e1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a2ea: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a2f2: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a2fb: 48 8b 85 20 04 00 00        	movq	0x420(%rbp), %rax
14075a302: 48 ff 00                    	incq	(%rax)
14075a305: 0f 28 b4 24 40 01 00 00     	movaps	0x140(%rsp), %xmm6
14075a30d: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a315: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a31e: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a327: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a330: 44 0f 28 9c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm11
14075a339: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
14075a342: 44 0f 28 ac 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm13
14075a34b: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a354: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a35d: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a364: 5b                          	popq	%rbx
14075a365: 5f                          	popq	%rdi
14075a366: 5e                          	popq	%rsi
14075a367: 41 5c                       	popq	%r12
14075a369: 41 5d                       	popq	%r13
14075a36b: 41 5e                       	popq	%r14
14075a36d: 41 5f                       	popq	%r15
14075a36f: 5d                          	popq	%rbp
14075a370: c3                          	retq
14075a371: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14075a380: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a385: 55                          	pushq	%rbp
14075a386: 41 57                       	pushq	%r15
14075a388: 41 56                       	pushq	%r14
14075a38a: 41 55                       	pushq	%r13
14075a38c: 41 54                       	pushq	%r12
14075a38e: 56                          	pushq	%rsi
14075a38f: 57                          	pushq	%rdi
14075a390: 53                          	pushq	%rbx
14075a391: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a398: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a39f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a3a8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a3b1: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075a3bb: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075a3c5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075a3cf: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a3d8: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a3e1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a3ea: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a3f2: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a3fb: 80 bd 30 04 00 00 00        	cmpb	$0x0, 0x430(%rbp)
14075a402: 74 13                       	je	0x14075a417 <.text+0x759417>
14075a404: c6 85 32 04 00 00 00        	movb	$0x0, 0x432(%rbp)
14075a40b: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
14075a412: e8 09 0f 9f ff              	callq	0x14014b320 <.text+0x14a320>
14075a417: c6 85 32 04 00 00 00        	movb	$0x0, 0x432(%rbp)
14075a41e: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a427: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a42f: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a438: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a441: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a44a: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075a454: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075a45e: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075a468: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a471: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a47a: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a481: 5b                          	popq	%rbx
14075a482: 5f                          	popq	%rdi
14075a483: 5e                          	popq	%rsi
14075a484: 41 5c                       	popq	%r12
14075a486: 41 5d                       	popq	%r13
14075a488: 41 5e                       	popq	%r14
14075a48a: 41 5f                       	popq	%r15
14075a48c: 5d                          	popq	%rbp
14075a48d: c3                          	retq
14075a48e: 66 90                       	nop
14075a490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a495: 55                          	pushq	%rbp
14075a496: 41 57                       	pushq	%r15
14075a498: 41 56                       	pushq	%r14
14075a49a: 41 55                       	pushq	%r13
14075a49c: 41 54                       	pushq	%r12
14075a49e: 56                          	pushq	%rsi
14075a49f: 57                          	pushq	%rdi
14075a4a0: 53                          	pushq	%rbx
14075a4a1: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a4a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a4af: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a4b8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a4c1: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075a4cb: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075a4d5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075a4df: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a4e8: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a4f1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a4fa: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a502: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a50b: 80 bd 36 04 00 00 00        	cmpb	$0x0, 0x436(%rbp)
14075a512: 0f 84 9e 00 00 00           	je	0x14075a5b6 <.text+0x7595b6>
14075a518: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
14075a51f: 48 85 d2                    	testq	%rdx, %rdx
14075a522: 74 16                       	je	0x14075a53a <.text+0x75953a>
14075a524: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
14075a52b: 48 c1 e2 06                 	shlq	$0x6, %rdx
14075a52f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075a535: e8 e6 96 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075a53a: 8b 85 c8 03 00 00           	movl	0x3c8(%rbp), %eax
14075a540: 88 85 34 04 00 00           	movb	%al, 0x434(%rbp)
14075a546: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a54f: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a557: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a560: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a569: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a572: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075a57c: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075a586: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075a590: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a599: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a5a2: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a5a9: 5b                          	popq	%rbx
14075a5aa: 5f                          	popq	%rdi
14075a5ab: 5e                          	popq	%rsi
14075a5ac: 41 5c                       	popq	%r12
14075a5ae: 41 5d                       	popq	%r13
14075a5b0: 41 5e                       	popq	%r14
14075a5b2: 41 5f                       	popq	%r15
14075a5b4: 5d                          	popq	%rbp
14075a5b5: c3                          	retq
14075a5b6: 8b 85 c8 03 00 00           	movl	0x3c8(%rbp), %eax
14075a5bc: 88 85 34 04 00 00           	movb	%al, 0x434(%rbp)
14075a5c2: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a5cb: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a5d3: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a5dc: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a5e5: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a5ee: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075a5f8: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075a602: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075a60c: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a615: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a61e: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a625: 5b                          	popq	%rbx
14075a626: 5f                          	popq	%rdi
14075a627: 5e                          	popq	%rsi
14075a628: 41 5c                       	popq	%r12
14075a62a: 41 5d                       	popq	%r13
14075a62c: 41 5e                       	popq	%r14
14075a62e: 41 5f                       	popq	%r15
14075a630: 5d                          	popq	%rbp
14075a631: c3                          	retq
14075a632: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14075a640: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a645: 55                          	pushq	%rbp
14075a646: 41 57                       	pushq	%r15
14075a648: 41 56                       	pushq	%r14
14075a64a: 41 55                       	pushq	%r13
14075a64c: 41 54                       	pushq	%r12
14075a64e: 56                          	pushq	%rsi
14075a64f: 57                          	pushq	%rdi
14075a650: 53                          	pushq	%rbx
14075a651: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a658: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a65f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a668: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a671: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075a67b: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075a685: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075a68f: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a698: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a6a1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a6aa: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a6b2: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a6bb: f6 85 34 04 00 00 01        	testb	$0x1, 0x434(%rbp)
14075a6c2: 74 7c                       	je	0x14075a740 <.text+0x759740>
14075a6c4: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
14075a6cb: e8 b0 cb ff ff              	callq	0x140757280 <.text+0x756280>
14075a6d0: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a6d9: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a6e1: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a6ea: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a6f3: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a6fc: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075a706: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075a710: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075a71a: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a723: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a72c: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a733: 5b                          	popq	%rbx
14075a734: 5f                          	popq	%rdi
14075a735: 5e                          	popq	%rsi
14075a736: 41 5c                       	popq	%r12
14075a738: 41 5d                       	popq	%r13
14075a73a: 41 5e                       	popq	%r14
14075a73c: 41 5f                       	popq	%r15
14075a73e: 5d                          	popq	%rbp
14075a73f: c3                          	retq
14075a740: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a749: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a751: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a75a: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a763: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a76c: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075a776: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075a780: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075a78a: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a793: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a79c: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a7a3: 5b                          	popq	%rbx
14075a7a4: 5f                          	popq	%rdi
14075a7a5: 5e                          	popq	%rsi
14075a7a6: 41 5c                       	popq	%r12
14075a7a8: 41 5d                       	popq	%r13
14075a7aa: 41 5e                       	popq	%r14
14075a7ac: 41 5f                       	popq	%r15
14075a7ae: 5d                          	popq	%rbp
14075a7af: c3                          	retq
14075a7b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a7b5: 55                          	pushq	%rbp
14075a7b6: 41 57                       	pushq	%r15
14075a7b8: 41 56                       	pushq	%r14
14075a7ba: 41 55                       	pushq	%r13
14075a7bc: 41 54                       	pushq	%r12
14075a7be: 56                          	pushq	%rsi
14075a7bf: 57                          	pushq	%rdi
14075a7c0: 53                          	pushq	%rbx
14075a7c1: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a7c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a7cf: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a7d8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a7e1: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075a7eb: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075a7f5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075a7ff: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a808: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a811: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a81a: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a822: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a82b: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
14075a832: e8 b9 7c e7 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
14075a837: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a840: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a848: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a851: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a85a: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a863: 44 0f 28 9c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm11
14075a86c: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
14075a875: 44 0f 28 ac 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm13
14075a87e: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a887: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a890: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a897: 5b                          	popq	%rbx
14075a898: 5f                          	popq	%rdi
14075a899: 5e                          	popq	%rsi
14075a89a: 41 5c                       	popq	%r12
14075a89c: 41 5d                       	popq	%r13
14075a89e: 41 5e                       	popq	%r14
14075a8a0: 41 5f                       	popq	%r15
14075a8a2: 5d                          	popq	%rbp
14075a8a3: c3                          	retq
14075a8a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14075a8b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a8b5: 55                          	pushq	%rbp
14075a8b6: 41 57                       	pushq	%r15
14075a8b8: 41 56                       	pushq	%r14
14075a8ba: 41 55                       	pushq	%r13
14075a8bc: 41 54                       	pushq	%r12
14075a8be: 56                          	pushq	%rsi
14075a8bf: 57                          	pushq	%rdi
14075a8c0: 53                          	pushq	%rbx
14075a8c1: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a8c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a8cf: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a8d8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a8e1: 44 0f 29 ac 24 d0 00 00 00  	movaps	%xmm13, 0xd0(%rsp)
14075a8ea: 44 0f 29 a4 24 e0 00 00 00  	movaps	%xmm12, 0xe0(%rsp)
14075a8f3: 44 0f 29 9c 24 f0 00 00 00  	movaps	%xmm11, 0xf0(%rsp)
14075a8fc: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075a905: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075a90e: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075a917: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075a91f: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075a928: 48 83 bd e0 02 00 00 00     	cmpq	$0x0, 0x2e0(%rbp)
14075a930: 74 1d                       	je	0x14075a94f <.text+0x75994f>
14075a932: 48 8b 95 e0 02 00 00        	movq	0x2e0(%rbp), %rdx
14075a939: 48 c1 e2 02                 	shlq	$0x2, %rdx
14075a93d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075a943: 48 8b 8d e0 03 00 00        	movq	0x3e0(%rbp), %rcx
14075a94a: e8 d1 92 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075a94f: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075a958: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075a960: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075a969: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075a972: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075a97b: 44 0f 28 9c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm11
14075a984: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
14075a98d: 44 0f 28 ac 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm13
14075a996: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075a99f: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075a9a8: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075a9af: 5b                          	popq	%rbx
14075a9b0: 5f                          	popq	%rdi
14075a9b1: 5e                          	popq	%rsi
14075a9b2: 41 5c                       	popq	%r12
14075a9b4: 41 5d                       	popq	%r13
14075a9b6: 41 5e                       	popq	%r14
14075a9b8: 41 5f                       	popq	%r15
14075a9ba: 5d                          	popq	%rbp
14075a9bb: c3                          	retq
14075a9bc: 0f 1f 40 00                 	nopl	(%rax)
14075a9c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075a9c5: 55                          	pushq	%rbp
14075a9c6: 41 57                       	pushq	%r15
14075a9c8: 41 56                       	pushq	%r14
14075a9ca: 41 55                       	pushq	%r13
14075a9cc: 41 54                       	pushq	%r12
14075a9ce: 56                          	pushq	%rsi
14075a9cf: 57                          	pushq	%rdi
14075a9d0: 53                          	pushq	%rbx
14075a9d1: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075a9d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075a9df: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075a9e8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075a9f1: 44 0f 29 ac 24 d0 00 00 00  	movaps	%xmm13, 0xd0(%rsp)
14075a9fa: 44 0f 29 a4 24 e0 00 00 00  	movaps	%xmm12, 0xe0(%rsp)
14075aa03: 44 0f 29 9c 24 f0 00 00 00  	movaps	%xmm11, 0xf0(%rsp)
14075aa0c: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075aa15: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075aa1e: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075aa27: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075aa2f: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075aa38: 48 83 bd d8 03 00 00 00     	cmpq	$0x0, 0x3d8(%rbp)
14075aa40: 74 1d                       	je	0x14075aa5f <.text+0x759a5f>
14075aa42: 48 8b 95 d8 03 00 00        	movq	0x3d8(%rbp), %rdx
14075aa49: 48 c1 e2 02                 	shlq	$0x2, %rdx
14075aa4d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075aa53: 48 8b 8d 00 04 00 00        	movq	0x400(%rbp), %rcx
14075aa5a: e8 c1 91 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075aa5f: c6 85 37 04 00 00 01        	movb	$0x1, 0x437(%rbp)
14075aa66: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075aa6f: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075aa77: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075aa80: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075aa89: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075aa92: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075aa9c: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075aaa6: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075aab0: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075aab9: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075aac2: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075aac9: 5b                          	popq	%rbx
14075aaca: 5f                          	popq	%rdi
14075aacb: 5e                          	popq	%rsi
14075aacc: 41 5c                       	popq	%r12
14075aace: 41 5d                       	popq	%r13
14075aad0: 41 5e                       	popq	%r14
14075aad2: 41 5f                       	popq	%r15
14075aad4: 5d                          	popq	%rbp
14075aad5: c3                          	retq
14075aad6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14075aae0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075aae5: 55                          	pushq	%rbp
14075aae6: 41 57                       	pushq	%r15
14075aae8: 41 56                       	pushq	%r14
14075aaea: 41 55                       	pushq	%r13
14075aaec: 41 54                       	pushq	%r12
14075aaee: 56                          	pushq	%rsi
14075aaef: 57                          	pushq	%rdi
14075aaf0: 53                          	pushq	%rbx
14075aaf1: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075aaf8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075aaff: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075ab08: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075ab11: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075ab1b: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075ab25: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075ab2f: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075ab38: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075ab41: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075ab4a: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075ab52: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075ab5b: 80 bd 32 04 00 00 00        	cmpb	$0x0, 0x432(%rbp)
14075ab62: 0f 84 a9 00 00 00           	je	0x14075ac11 <.text+0x759c11>
14075ab68: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14075ab6f: 48 85 c0                    	testq	%rax, %rax
14075ab72: 74 1a                       	je	0x14075ab8e <.text+0x759b8e>
14075ab74: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14075ab7b: 48 c1 e0 05                 	shlq	$0x5, %rax
14075ab7f: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14075ab83: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075ab89: e8 92 90 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075ab8e: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
14075ab95: e8 86 07 9f ff              	callq	0x14014b320 <.text+0x14a320>
14075ab9a: c6 85 35 04 00 00 00        	movb	$0x0, 0x435(%rbp)
14075aba1: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075abaa: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075abb2: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075abbb: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075abc4: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075abcd: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075abd7: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075abe1: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075abeb: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075abf4: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075abfd: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075ac04: 5b                          	popq	%rbx
14075ac05: 5f                          	popq	%rdi
14075ac06: 5e                          	popq	%rsi
14075ac07: 41 5c                       	popq	%r12
14075ac09: 41 5d                       	popq	%r13
14075ac0b: 41 5e                       	popq	%r14
14075ac0d: 41 5f                       	popq	%r15
14075ac0f: 5d                          	popq	%rbp
14075ac10: c3                          	retq
14075ac11: c6 85 35 04 00 00 00        	movb	$0x0, 0x435(%rbp)
14075ac18: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075ac21: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075ac29: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075ac32: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075ac3b: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075ac44: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075ac4e: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075ac58: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075ac62: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075ac6b: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075ac74: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075ac7b: 5b                          	popq	%rbx
14075ac7c: 5f                          	popq	%rdi
14075ac7d: 5e                          	popq	%rsi
14075ac7e: 41 5c                       	popq	%r12
14075ac80: 41 5d                       	popq	%r13
14075ac82: 41 5e                       	popq	%r14
14075ac84: 41 5f                       	popq	%r15
14075ac86: 5d                          	popq	%rbp
14075ac87: c3                          	retq
14075ac88: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14075ac90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075ac95: 55                          	pushq	%rbp
14075ac96: 41 57                       	pushq	%r15
14075ac98: 41 56                       	pushq	%r14
14075ac9a: 41 55                       	pushq	%r13
14075ac9c: 41 54                       	pushq	%r12
14075ac9e: 56                          	pushq	%rsi
14075ac9f: 57                          	pushq	%rdi
14075aca0: 53                          	pushq	%rbx
14075aca1: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075aca8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075acaf: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075acb8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075acc1: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075accb: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075acd5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075acdf: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075ace8: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075acf1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075acfa: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075ad02: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075ad0b: 0f b6 9d 35 04 00 00        	movzbl	0x435(%rbp), %ebx
14075ad12: f6 c3 01                    	testb	$0x1, %bl
14075ad15: 0f 84 82 00 00 00           	je	0x14075ad9d <.text+0x759d9d>
14075ad1b: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
14075ad22: e8 f9 05 9f ff              	callq	0x14014b320 <.text+0x14a320>
14075ad27: 88 9d 37 04 00 00           	movb	%bl, 0x437(%rbp)
14075ad2d: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075ad36: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075ad3e: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075ad47: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075ad50: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075ad59: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075ad63: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075ad6d: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075ad77: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075ad80: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075ad89: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075ad90: 5b                          	popq	%rbx
14075ad91: 5f                          	popq	%rdi
14075ad92: 5e                          	popq	%rsi
14075ad93: 41 5c                       	popq	%r12
14075ad95: 41 5d                       	popq	%r13
14075ad97: 41 5e                       	popq	%r14
14075ad99: 41 5f                       	popq	%r15
14075ad9b: 5d                          	popq	%rbp
14075ad9c: c3                          	retq
14075ad9d: 88 9d 37 04 00 00           	movb	%bl, 0x437(%rbp)
14075ada3: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075adac: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075adb4: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075adbd: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075adc6: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075adcf: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075add9: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075ade3: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075aded: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075adf6: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075adff: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075ae06: 5b                          	popq	%rbx
14075ae07: 5f                          	popq	%rdi
14075ae08: 5e                          	popq	%rsi
14075ae09: 41 5c                       	popq	%r12
14075ae0b: 41 5d                       	popq	%r13
14075ae0d: 41 5e                       	popq	%r14
14075ae0f: 41 5f                       	popq	%r15
14075ae11: 5d                          	popq	%rbp
14075ae12: c3                          	retq
14075ae13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14075ae20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075ae25: 55                          	pushq	%rbp
14075ae26: 41 57                       	pushq	%r15
14075ae28: 41 56                       	pushq	%r14
14075ae2a: 41 55                       	pushq	%r13
14075ae2c: 41 54                       	pushq	%r12
14075ae2e: 56                          	pushq	%rsi
14075ae2f: 57                          	pushq	%rdi
14075ae30: 53                          	pushq	%rbx
14075ae31: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075ae38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075ae3f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075ae48: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075ae51: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075ae5b: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075ae65: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075ae6f: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075ae78: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075ae81: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075ae8a: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075ae92: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075ae9b: f6 85 37 04 00 00 01        	testb	$0x1, 0x437(%rbp)
14075aea2: 0f 84 92 00 00 00           	je	0x14075af3a <.text+0x759f3a>
14075aea8: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
14075aeaf: 48 85 c0                    	testq	%rax, %rax
14075aeb2: 74 1a                       	je	0x14075aece <.text+0x759ece>
14075aeb4: 48 8b 8d b8 02 00 00        	movq	0x2b8(%rbp), %rcx
14075aebb: 48 c1 e0 05                 	shlq	$0x5, %rax
14075aebf: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14075aec3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075aec9: e8 52 8d eb ff              	callq	0x140613c20 <.text+0x612c20>
14075aece: 0f 28 b4 24 40 01 00 00     	movaps	0x140(%rsp), %xmm6
14075aed6: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075aede: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075aee7: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075aef0: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075aef9: 44 0f 28 9c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm11
14075af02: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
14075af0b: 44 0f 28 ac 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm13
14075af14: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075af1d: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075af26: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075af2d: 5b                          	popq	%rbx
14075af2e: 5f                          	popq	%rdi
14075af2f: 5e                          	popq	%rsi
14075af30: 41 5c                       	popq	%r12
14075af32: 41 5d                       	popq	%r13
14075af34: 41 5e                       	popq	%r14
14075af36: 41 5f                       	popq	%r15
14075af38: 5d                          	popq	%rbp
14075af39: c3                          	retq
14075af3a: 0f 28 b4 24 40 01 00 00     	movaps	0x140(%rsp), %xmm6
14075af42: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075af4a: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075af53: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075af5c: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075af65: 44 0f 28 9c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm11
14075af6e: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
14075af77: 44 0f 28 ac 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm13
14075af80: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075af89: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075af92: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075af99: 5b                          	popq	%rbx
14075af9a: 5f                          	popq	%rdi
14075af9b: 5e                          	popq	%rsi
14075af9c: 41 5c                       	popq	%r12
14075af9e: 41 5d                       	popq	%r13
14075afa0: 41 5e                       	popq	%r14
14075afa2: 41 5f                       	popq	%r15
14075afa4: 5d                          	popq	%rbp
14075afa5: c3                          	retq
14075afa6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14075afb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075afb5: 55                          	pushq	%rbp
14075afb6: 41 57                       	pushq	%r15
14075afb8: 41 56                       	pushq	%r14
14075afba: 41 55                       	pushq	%r13
14075afbc: 41 54                       	pushq	%r12
14075afbe: 56                          	pushq	%rsi
14075afbf: 57                          	pushq	%rdi
14075afc0: 53                          	pushq	%rbx
14075afc1: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075afc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075afcf: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075afd8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075afe1: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075afeb: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075aff5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075afff: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075b008: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075b011: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075b01a: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075b022: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075b02b: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
14075b032: 48 85 d2                    	testq	%rdx, %rdx
14075b035: 74 16                       	je	0x14075b04d <.text+0x75a04d>
14075b037: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075b03b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075b041: 48 8b 8d 08 04 00 00        	movq	0x408(%rbp), %rcx
14075b048: e8 d3 8b eb ff              	callq	0x140613c20 <.text+0x612c20>
14075b04d: 48 8b 95 b0 01 00 00        	movq	0x1b0(%rbp), %rdx
14075b054: 48 85 d2                    	testq	%rdx, %rdx
14075b057: 74 16                       	je	0x14075b06f <.text+0x75a06f>
14075b059: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
14075b060: 48 c1 e2 04                 	shlq	$0x4, %rdx
14075b064: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075b06a: e8 b1 8b eb ff              	callq	0x140613c20 <.text+0x612c20>
14075b06f: c6 85 33 04 00 00 00        	movb	$0x0, 0x433(%rbp)
14075b076: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075b07f: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075b087: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075b090: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075b099: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075b0a2: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075b0ac: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075b0b6: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075b0c0: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075b0c9: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075b0d2: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075b0d9: 5b                          	popq	%rbx
14075b0da: 5f                          	popq	%rdi
14075b0db: 5e                          	popq	%rsi
14075b0dc: 41 5c                       	popq	%r12
14075b0de: 41 5d                       	popq	%r13
14075b0e0: 41 5e                       	popq	%r14
14075b0e2: 41 5f                       	popq	%r15
14075b0e4: 5d                          	popq	%rbp
14075b0e5: c3                          	retq
14075b0e6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14075b0f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075b0f5: 55                          	pushq	%rbp
14075b0f6: 41 57                       	pushq	%r15
14075b0f8: 41 56                       	pushq	%r14
14075b0fa: 41 55                       	pushq	%r13
14075b0fc: 41 54                       	pushq	%r12
14075b0fe: 56                          	pushq	%rsi
14075b0ff: 57                          	pushq	%rdi
14075b100: 53                          	pushq	%rbx
14075b101: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075b108: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075b10f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075b118: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075b121: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075b12b: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075b135: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075b13f: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075b148: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075b151: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075b15a: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075b162: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075b16b: 80 bd 33 04 00 00 00        	cmpb	$0x0, 0x433(%rbp)
14075b172: 0f 84 92 00 00 00           	je	0x14075b20a <.text+0x75a20a>
14075b178: 48 8b 95 b0 03 00 00        	movq	0x3b0(%rbp), %rdx
14075b17f: 48 85 d2                    	testq	%rdx, %rdx
14075b182: 74 16                       	je	0x14075b19a <.text+0x75a19a>
14075b184: 48 8b 8d b8 03 00 00        	movq	0x3b8(%rbp), %rcx
14075b18b: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075b18f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075b195: e8 86 8a eb ff              	callq	0x140613c20 <.text+0x612c20>
14075b19a: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075b1a3: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075b1ab: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075b1b4: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075b1bd: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075b1c6: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075b1d0: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075b1da: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075b1e4: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075b1ed: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075b1f6: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075b1fd: 5b                          	popq	%rbx
14075b1fe: 5f                          	popq	%rdi
14075b1ff: 5e                          	popq	%rsi
14075b200: 41 5c                       	popq	%r12
14075b202: 41 5d                       	popq	%r13
14075b204: 41 5e                       	popq	%r14
14075b206: 41 5f                       	popq	%r15
14075b208: 5d                          	popq	%rbp
14075b209: c3                          	retq
14075b20a: 66 0f 6f b4 24 40 01 00 00  	movdqa	0x140(%rsp), %xmm6
14075b213: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075b21b: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075b224: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075b22d: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075b236: 66 44 0f 6f 9c 24 f0 00 00 00       	movdqa	0xf0(%rsp), %xmm11
14075b240: 66 44 0f 6f a4 24 e0 00 00 00       	movdqa	0xe0(%rsp), %xmm12
14075b24a: 66 44 0f 6f ac 24 d0 00 00 00       	movdqa	0xd0(%rsp), %xmm13
14075b254: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075b25d: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075b266: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075b26d: 5b                          	popq	%rbx
14075b26e: 5f                          	popq	%rdi
14075b26f: 5e                          	popq	%rsi
14075b270: 41 5c                       	popq	%r12
14075b272: 41 5d                       	popq	%r13
14075b274: 41 5e                       	popq	%r14
14075b276: 41 5f                       	popq	%r15
14075b278: 5d                          	popq	%rbp
14075b279: c3                          	retq
14075b27a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14075b280: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075b285: 55                          	pushq	%rbp
14075b286: 41 57                       	pushq	%r15
14075b288: 41 56                       	pushq	%r14
14075b28a: 41 55                       	pushq	%r13
14075b28c: 41 54                       	pushq	%r12
14075b28e: 56                          	pushq	%rsi
14075b28f: 57                          	pushq	%rdi
14075b290: 53                          	pushq	%rbx
14075b291: 48 81 ec 58 01 00 00        	subq	$0x158, %rsp            # imm = 0x158
14075b298: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075b29f: 44 0f 29 bc 24 b0 00 00 00  	movaps	%xmm15, 0xb0(%rsp)
14075b2a8: 44 0f 29 b4 24 c0 00 00 00  	movaps	%xmm14, 0xc0(%rsp)
14075b2b1: 66 44 0f 7f ac 24 d0 00 00 00       	movdqa	%xmm13, 0xd0(%rsp)
14075b2bb: 66 44 0f 7f a4 24 e0 00 00 00       	movdqa	%xmm12, 0xe0(%rsp)
14075b2c5: 66 44 0f 7f 9c 24 f0 00 00 00       	movdqa	%xmm11, 0xf0(%rsp)
14075b2cf: 44 0f 29 94 24 00 01 00 00  	movaps	%xmm10, 0x100(%rsp)
14075b2d8: 44 0f 29 8c 24 10 01 00 00  	movaps	%xmm9, 0x110(%rsp)
14075b2e1: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
14075b2ea: 0f 29 bc 24 30 01 00 00     	movaps	%xmm7, 0x130(%rsp)
14075b2f2: 66 0f 7f b4 24 40 01 00 00  	movdqa	%xmm6, 0x140(%rsp)
14075b2fb: 80 bd 31 04 00 00 00        	cmpb	$0x0, 0x431(%rbp)
14075b302: 74 3d                       	je	0x14075b341 <.text+0x75a341>
14075b304: e8 27 ce 4b 00              	callq	0x140c18130 <.text+0xc17130>
14075b309: 48 89 c6                    	movq	%rax, %rsi
14075b30c: 48 8b 00                    	movq	(%rax), %rax
14075b30f: 48 83 f8 01                 	cmpq	$0x1, %rax
14075b313: 0f 85 94 00 00 00           	jne	0x14075b3ad <.text+0x75a3ad>
14075b319: 48 83 c6 08                 	addq	$0x8, %rsi
14075b31d: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
14075b324: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
14075b328: 0f 85 ad 00 00 00           	jne	0x14075b3db <.text+0x75a3db>
14075b32e: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
14075b335: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
14075b339: e8 b2 bd 4b 00              	callq	0x140c170f0 <.text+0xc160f0>
14075b33e: 48 ff 06                    	incq	(%rsi)
14075b341: 0f 28 b4 24 40 01 00 00     	movaps	0x140(%rsp), %xmm6
14075b349: 0f 28 bc 24 30 01 00 00     	movaps	0x130(%rsp), %xmm7
14075b351: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
14075b35a: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
14075b363: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
14075b36c: 44 0f 28 9c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm11
14075b375: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
14075b37e: 44 0f 28 ac 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm13
14075b387: 44 0f 28 b4 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm14
14075b390: 44 0f 28 bc 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm15
14075b399: 48 81 c4 58 01 00 00        	addq	$0x158, %rsp            # imm = 0x158
14075b3a0: 5b                          	popq	%rbx
14075b3a1: 5f                          	popq	%rdi
14075b3a2: 5e                          	popq	%rsi
14075b3a3: 41 5c                       	popq	%r12
14075b3a5: 41 5d                       	popq	%r13
14075b3a7: 41 5e                       	popq	%r14
14075b3a9: 41 5f                       	popq	%r15
14075b3ab: 5d                          	popq	%rbp
14075b3ac: c3                          	retq
14075b3ad: 48 85 c0                    	testq	%rax, %rax
14075b3b0: 75 1d                       	jne	0x14075b3cf <.text+0x75a3cf>
14075b3b2: 48 89 f1                    	movq	%rsi, %rcx
14075b3b5: 31 d2                       	xorl	%edx, %edx
14075b3b7: e8 04 62 19 02              	callq	0x1428f15c0 <.text+0x28f05c0>
14075b3bc: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
14075b3c3: 48 89 c6                    	movq	%rax, %rsi
14075b3c6: 48 85 c0                    	testq	%rax, %rax
14075b3c9: 0f 85 55 ff ff ff           	jne	0x14075b324 <.text+0x75a324>
14075b3cf: 48 8d 0d d2 e0 4b 02        	leaq	0x24be0d2(%rip), %rcx   # 0x142c194a8
14075b3d6: e8 f5 cc 17 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14075b3db: 48 8d 0d 36 e1 4b 02        	leaq	0x24be136(%rip), %rcx   # 0x142c19518
14075b3e2: e8 69 de 17 02              	callq	0x1428d9250 <.text+0x28d8250>
14075b3e7: cc                          	int3
