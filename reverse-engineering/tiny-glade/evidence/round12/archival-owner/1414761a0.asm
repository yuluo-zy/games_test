
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414761a0: 55                          	pushq	%rbp
1414761a1: 41 57                       	pushq	%r15
1414761a3: 41 56                       	pushq	%r14
1414761a5: 41 55                       	pushq	%r13
1414761a7: 41 54                       	pushq	%r12
1414761a9: 56                          	pushq	%rsi
1414761aa: 57                          	pushq	%rdi
1414761ab: 53                          	pushq	%rbx
1414761ac: 48 81 ec a8 01 00 00        	subq	$0x1a8, %rsp            # imm = 0x1A8
1414761b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1414761bb: 48 c7 85 20 01 00 00 fe ff ff ff    	movq	$-0x2, 0x120(%rbp)
1414761c6: 0f 10 41 40                 	movups	0x40(%rcx), %xmm0
1414761ca: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1414761d1: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
1414761d5: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
1414761d9: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
1414761dd: 0f 10 59 30                 	movups	0x30(%rcx), %xmm3
1414761e1: 0f 29 9d 00 01 00 00        	movaps	%xmm3, 0x100(%rbp)
1414761e8: 0f 29 95 f0 00 00 00        	movaps	%xmm2, 0xf0(%rbp)
1414761ef: 0f 29 8d e0 00 00 00        	movaps	%xmm1, 0xe0(%rbp)
1414761f6: 66 0f 7f 85 d0 00 00 00     	movdqa	%xmm0, 0xd0(%rbp)
1414761fe: 4c 8b bd f0 00 00 00        	movq	0xf0(%rbp), %r15
141476205: 4d 85 ff                    	testq	%r15, %r15
141476208: 0f 84 a1 01 00 00           	je	0x1414763af <.text+0x14753af>
14147620e: 48 89 d6                    	movq	%rdx, %rsi
141476211: 0f b7 85 e8 00 00 00        	movzwl	0xe8(%rbp), %eax
141476218: 48 8d 5d a8                 	leaq	-0x58(%rbp), %rbx
14147621c: 0f 1f 40 00                 	nopl	(%rax)
141476220: 4c 8b a5 d0 00 00 00        	movq	0xd0(%rbp), %r12
141476227: 66 85 c0                    	testw	%ax, %ax
14147622a: 74 24                       	je	0x141476250 <.text+0x1475250>
14147622c: 44 8d 68 ff                 	leal	-0x1(%rax), %r13d
141476230: 41 21 c5                    	andl	%eax, %r13d
141476233: 66 44 89 ad e8 00 00 00     	movw	%r13w, 0xe8(%rbp)
14147623b: 49 ff cf                    	decq	%r15
14147623e: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
141476245: 4d 85 e4                    	testq	%r12, %r12
141476248: 75 5f                       	jne	0x1414762a9 <.text+0x14752a9>
14147624a: e9 d5 00 00 00              	jmp	0x141476324 <.text+0x1475324>
14147624f: 90                          	nop
141476250: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
141476257: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141476260: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
141476264: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
141476268: 49 81 c4 80 f6 ff ff        	addq	$-0x980, %r12           # imm = 0xF680
14147626f: 48 83 c1 10                 	addq	$0x10, %rcx
141476273: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
141476278: 74 e6                       	je	0x141476260 <.text+0x1475260>
14147627a: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
141476281: 4c 89 a5 d0 00 00 00        	movq	%r12, 0xd0(%rbp)
141476288: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14147628d: 29 c1                       	subl	%eax, %ecx
14147628f: f7 d0                       	notl	%eax
141476291: 41 89 c5                    	movl	%eax, %r13d
141476294: 41 21 cd                    	andl	%ecx, %r13d
141476297: 66 44 89 ad e8 00 00 00     	movw	%r13w, 0xe8(%rbp)
14147629f: 49 ff cf                    	decq	%r15
1414762a2: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
1414762a9: f3 0f bc c0                 	tzcntl	%eax, %eax
1414762ad: 48 f7 d8                    	negq	%rax
1414762b0: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
1414762b7: 49 8b bc 04 68 ff ff ff     	movq	-0x98(%r12,%rax), %rdi
1414762bf: 4d 8b b4 04 70 ff ff ff     	movq	-0x90(%r12,%rax), %r14
1414762c7: 49 8d 14 04                 	leaq	(%r12,%rax), %rdx
1414762cb: 48 81 c2 78 ff ff ff        	addq	$-0x88, %rdx
1414762d2: 41 b8 88 00 00 00           	movl	$0x88, %r8d
1414762d8: 48 89 d9                    	movq	%rbx, %rcx
1414762db: e8 20 d9 4a 01              	callq	0x142923c00 <.text+0x2922c00>
1414762e0: 4d 85 f6                    	testq	%r14, %r14
1414762e3: 74 42                       	je	0x141476327 <.text+0x1475327>
1414762e5: 41 b8 88 00 00 00           	movl	$0x88, %r8d
1414762eb: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1414762ef: 48 89 da                    	movq	%rbx, %rdx
1414762f2: e8 09 d9 4a 01              	callq	0x142923c00 <.text+0x2922c00>
1414762f7: 4c 89 75 40                 	movq	%r14, 0x40(%rbp)
1414762fb: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
141476303: 48 89 7d 38                 	movq	%rdi, 0x38(%rbp)
141476307: 48 89 f1                    	movq	%rsi, %rcx
14147630a: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
14147630e: e8 3d 47 6b ff              	callq	0x140b2aa50 <.text+0xb29a50>
141476313: 44 89 e8                    	movl	%r13d, %eax
141476316: 4d 85 ff                    	testq	%r15, %r15
141476319: 0f 85 01 ff ff ff           	jne	0x141476220 <.text+0x1475220>
14147631f: e9 8b 00 00 00              	jmp	0x1414763af <.text+0x14753af>
141476324: 45 31 e4                    	xorl	%r12d, %r12d
141476327: 4d 85 ff                    	testq	%r15, %r15
14147632a: 0f 84 7f 00 00 00           	je	0x1414763af <.text+0x14753af>
141476330: 48 8b b5 d8 00 00 00        	movq	0xd8(%rbp), %rsi
141476337: eb 2e                       	jmp	0x141476367 <.text+0x1475367>
141476339: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141476340: f3 41 0f bc c5              	tzcntl	%r13d, %eax
141476345: 48 f7 d8                    	negq	%rax
141476348: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
14147634f: 49 8d 0c 04                 	leaq	(%r12,%rax), %rcx
141476353: 48 81 c1 70 ff ff ff        	addq	$-0x90, %rcx
14147635a: e8 31 e7 18 ff              	callq	0x140604a90 <.text+0x603a90>
14147635f: 41 89 fd                    	movl	%edi, %r13d
141476362: 49 ff cf                    	decq	%r15
141476365: 74 48                       	je	0x1414763af <.text+0x14753af>
141476367: 66 45 85 ed                 	testw	%r13w, %r13w
14147636b: 74 13                       	je	0x141476380 <.text+0x1475380>
14147636d: 4d 85 e4                    	testq	%r12, %r12
141476370: 74 3d                       	je	0x1414763af <.text+0x14753af>
141476372: 41 8d 7d ff                 	leal	-0x1(%r13), %edi
141476376: 44 21 ef                    	andl	%r13d, %edi
141476379: eb c5                       	jmp	0x141476340 <.text+0x1475340>
14147637b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141476380: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
141476384: 66 44 0f d7 e8              	pmovmskb	%xmm0, %r13d
141476389: 49 81 c4 80 f6 ff ff        	addq	$-0x980, %r12           # imm = 0xF680
141476390: 48 83 c6 10                 	addq	$0x10, %rsi
141476394: 41 81 fd ff ff 00 00        	cmpl	$0xffff, %r13d          # imm = 0xFFFF
14147639b: 74 e3                       	je	0x141476380 <.text+0x1475380>
14147639d: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1414763a2: 44 29 e8                    	subl	%r13d, %eax
1414763a5: 41 f7 d5                    	notl	%r13d
1414763a8: 44 89 ef                    	movl	%r13d, %edi
1414763ab: 21 c7                       	andl	%eax, %edi
1414763ad: eb 91                       	jmp	0x141476340 <.text+0x1475340>
1414763af: 48 8d b5 f8 00 00 00        	leaq	0xf8(%rbp), %rsi
1414763b6: 48 8b bd 00 01 00 00        	movq	0x100(%rbp), %rdi
1414763bd: 48 85 ff                    	testq	%rdi, %rdi
1414763c0: 74 2e                       	je	0x1414763f0 <.text+0x14753f0>
1414763c2: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1414763c9: 4c 8d 47 11                 	leaq	0x11(%rdi), %r8
1414763cd: b2 ff                       	movb	$-0x1, %dl
1414763cf: e8 4c d8 4a 01              	callq	0x142923c20 <.text+0x2922c20>
1414763d4: 48 8d 47 01                 	leaq	0x1(%rdi), %rax
1414763d8: 48 89 c1                    	movq	%rax, %rcx
1414763db: 48 c1 e9 03                 	shrq	$0x3, %rcx
1414763df: 48 83 e0 f8                 	andq	$-0x8, %rax
1414763e3: 48 29 c8                    	subq	%rcx, %rax
1414763e6: 48 83 ff 08                 	cmpq	$0x8, %rdi
1414763ea: 48 0f 42 c7                 	cmovbq	%rdi, %rax
1414763ee: eb 02                       	jmp	0x1414763f2 <.text+0x14753f2>
1414763f0: 31 c0                       	xorl	%eax, %eax
1414763f2: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
1414763fd: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
141476404: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14147640b: 0f 10 06                    	movups	(%rsi), %xmm0
14147640e: 0f 10 4e 10                 	movups	0x10(%rsi), %xmm1
141476412: 0f 11 00                    	movups	%xmm0, (%rax)
141476415: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
141476419: 48 81 c4 a8 01 00 00        	addq	$0x1a8, %rsp            # imm = 0x1A8
141476420: 5b                          	popq	%rbx
141476421: 5f                          	popq	%rdi
141476422: 5e                          	popq	%rsi
141476423: 41 5c                       	popq	%r12
141476425: 41 5d                       	popq	%r13
141476427: 41 5e                       	popq	%r14
141476429: 41 5f                       	popq	%r15
14147642b: 5d                          	popq	%rbp
14147642c: c3                          	retq
14147642d: 0f 1f 00                    	nopl	(%rax)
141476430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141476435: 55                          	pushq	%rbp
141476436: 41 57                       	pushq	%r15
141476438: 41 56                       	pushq	%r14
14147643a: 41 55                       	pushq	%r13
14147643c: 41 54                       	pushq	%r12
14147643e: 56                          	pushq	%rsi
14147643f: 57                          	pushq	%rdi
141476440: 53                          	pushq	%rbx
141476441: 48 83 ec 28                 	subq	$0x28, %rsp
141476445: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14147644c: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
141476453: e8 38 a3 40 ff              	callq	0x140880790 <.text+0x87f790>
141476458: 90                          	nop
141476459: 48 83 c4 28                 	addq	$0x28, %rsp
14147645d: 5b                          	popq	%rbx
14147645e: 5f                          	popq	%rdi
14147645f: 5e                          	popq	%rsi
141476460: 41 5c                       	popq	%r12
141476462: 41 5d                       	popq	%r13
141476464: 41 5e                       	popq	%r14
141476466: 41 5f                       	popq	%r15
141476468: 5d                          	popq	%rbp
141476469: c3                          	retq
