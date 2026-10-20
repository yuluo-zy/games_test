
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ee800: 41 57                       	pushq	%r15
1408ee802: 41 56                       	pushq	%r14
1408ee804: 41 55                       	pushq	%r13
1408ee806: 41 54                       	pushq	%r12
1408ee808: 56                          	pushq	%rsi
1408ee809: 57                          	pushq	%rdi
1408ee80a: 55                          	pushq	%rbp
1408ee80b: 53                          	pushq	%rbx
1408ee80c: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1408ee813: 44 89 cd                    	movl	%r9d, %ebp
1408ee816: 4c 89 c7                    	movq	%r8, %rdi
1408ee819: 48 89 d6                    	movq	%rdx, %rsi
1408ee81c: 48 89 cb                    	movq	%rcx, %rbx
1408ee81f: 48 89 8c 24 98 00 00 00     	movq	%rcx, 0x98(%rsp)
1408ee827: 44 89 c9                    	movl	%r9d, %ecx
1408ee82a: e8 e1 25 32 00              	callq	0x140c10e10 <.text+0xc0fe10>
1408ee82f: 84 c0                       	testb	%al, %al
1408ee831: 0f 85 00 07 00 00           	jne	0x1408eef37 <.text+0x8edf37>
1408ee837: 4c 8b b6 e8 00 00 00        	movq	0xe8(%rsi), %r14
1408ee83e: 49 83 be b0 00 00 00 00     	cmpq	$0x0, 0xb0(%r14)
1408ee846: 0f 84 b2 00 00 00           	je	0x1408ee8fe <.text+0x8ed8fe>
1408ee84c: 4c 8b ac 24 50 01 00 00     	movq	0x150(%rsp), %r13
1408ee854: 49 89 f8                    	movq	%rdi, %r8
1408ee857: 49 c1 e8 39                 	shrq	$0x39, %r8
1408ee85b: 49 8b 86 98 00 00 00        	movq	0x98(%r14), %rax
1408ee862: 49 8b 8e a0 00 00 00        	movq	0xa0(%r14), %rcx
1408ee869: 48 89 ca                    	movq	%rcx, %rdx
1408ee86c: 48 21 fa                    	andq	%rdi, %rdx
1408ee86f: 66 41 0f 6e c0              	movd	%r8d, %xmm0
1408ee874: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408ee878: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408ee87d: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408ee882: 4c 8d 40 f0                 	leaq	-0x10(%rax), %r8
1408ee886: 45 31 c9                    	xorl	%r9d, %r9d
1408ee889: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408ee88d: f3 0f 6f 14 10              	movdqu	(%rax,%rdx), %xmm2
1408ee892: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408ee896: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408ee89a: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1408ee89f: 45 85 d2                    	testl	%r10d, %r10d
1408ee8a2: 74 3c                       	je	0x1408ee8e0 <.text+0x8ed8e0>
1408ee8a4: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1408ee8a9: 49 01 d3                    	addq	%rdx, %r11
1408ee8ac: 49 21 cb                    	andq	%rcx, %r11
1408ee8af: 4d 89 df                    	movq	%r11, %r15
1408ee8b2: 49 c1 e7 04                 	shlq	$0x4, %r15
1408ee8b6: 4d 89 c4                    	movq	%r8, %r12
1408ee8b9: 4d 29 fc                    	subq	%r15, %r12
1408ee8bc: 49 39 3c 24                 	cmpq	%rdi, (%r12)
1408ee8c0: 0f 84 0a 01 00 00           	je	0x1408ee9d0 <.text+0x8ed9d0>
1408ee8c6: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1408ee8ca: 66 45 21 d3                 	andw	%r10w, %r11w
1408ee8ce: 45 89 da                    	movl	%r11d, %r10d
1408ee8d1: 75 d1                       	jne	0x1408ee8a4 <.text+0x8ed8a4>
1408ee8d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1408ee8e0: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408ee8e4: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1408ee8e9: 45 85 d2                    	testl	%r10d, %r10d
1408ee8ec: 75 10                       	jne	0x1408ee8fe <.text+0x8ed8fe>
1408ee8ee: 4c 01 ca                    	addq	%r9, %rdx
1408ee8f1: 48 83 c2 10                 	addq	$0x10, %rdx
1408ee8f5: 49 83 c1 10                 	addq	$0x10, %r9
1408ee8f9: 48 21 ca                    	andq	%rcx, %rdx
1408ee8fc: eb 8f                       	jmp	0x1408ee88d <.text+0x8ed88d>
1408ee8fe: 48 89 f9                    	movq	%rdi, %rcx
1408ee901: e8 4a 8d 39 00              	callq	0x140c87650 <.text+0xc86650>
1408ee906: 80 be de 00 00 00 08        	cmpb	$0x8, 0xde(%rsi)
1408ee90d: 0f 84 70 05 00 00           	je	0x1408eee83 <.text+0x8ede83>
1408ee913: 49 83 be e8 00 00 00 00     	cmpq	$0x0, 0xe8(%r14)
1408ee91b: 0f 84 62 05 00 00           	je	0x1408eee83 <.text+0x8ede83>
1408ee921: 49 83 be 30 01 00 00 00     	cmpq	$0x0, 0x130(%r14)
1408ee929: 0f 84 54 05 00 00           	je	0x1408eee83 <.text+0x8ede83>
1408ee92f: 49 89 c1                    	movq	%rax, %r9
1408ee932: 49 c1 e9 39                 	shrq	$0x39, %r9
1408ee936: 49 8b 8e 18 01 00 00        	movq	0x118(%r14), %rcx
1408ee93d: 49 8b 96 20 01 00 00        	movq	0x120(%r14), %rdx
1408ee944: 49 89 d0                    	movq	%rdx, %r8
1408ee947: 49 21 c0                    	andq	%rax, %r8
1408ee94a: 66 41 0f 6e c1              	movd	%r9d, %xmm0
1408ee94f: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408ee953: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408ee958: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408ee95d: 4c 8d 49 f0                 	leaq	-0x10(%rcx), %r9
1408ee961: 45 31 d2                    	xorl	%r10d, %r10d
1408ee964: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408ee968: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1408ee96e: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408ee972: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408ee976: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
1408ee97b: 45 85 db                    	testl	%r11d, %r11d
1408ee97e: 74 2e                       	je	0x1408ee9ae <.text+0x8ed9ae>
1408ee980: f3 41 0f bc fb              	tzcntl	%r11d, %edi
1408ee985: 4c 01 c7                    	addq	%r8, %rdi
1408ee988: 48 21 d7                    	andq	%rdx, %rdi
1408ee98b: 49 89 fe                    	movq	%rdi, %r14
1408ee98e: 49 c1 e6 04                 	shlq	$0x4, %r14
1408ee992: 4d 89 cf                    	movq	%r9, %r15
1408ee995: 4d 29 f7                    	subq	%r14, %r15
1408ee998: 49 39 07                    	cmpq	%rax, (%r15)
1408ee99b: 0f 84 20 02 00 00           	je	0x1408eebc1 <.text+0x8edbc1>
1408ee9a1: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
1408ee9a5: 66 44 21 df                 	andw	%r11w, %di
1408ee9a9: 41 89 fb                    	movl	%edi, %r11d
1408ee9ac: 75 d2                       	jne	0x1408ee980 <.text+0x8ed980>
1408ee9ae: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408ee9b2: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1408ee9b7: 45 85 db                    	testl	%r11d, %r11d
1408ee9ba: 0f 85 c3 04 00 00           	jne	0x1408eee83 <.text+0x8ede83>
1408ee9c0: 4d 01 d0                    	addq	%r10, %r8
1408ee9c3: 49 83 c0 10                 	addq	$0x10, %r8
1408ee9c7: 49 83 c2 10                 	addq	$0x10, %r10
1408ee9cb: 49 21 d0                    	andq	%rdx, %r8
1408ee9ce: eb 98                       	jmp	0x1408ee968 <.text+0x8ed968>
1408ee9d0: 49 f7 db                    	negq	%r11
1408ee9d3: 49 c1 e3 04                 	shlq	$0x4, %r11
1408ee9d7: 42 8b 4c 18 f8              	movl	-0x8(%rax,%r11), %ecx
1408ee9dc: 46 8b 7c 18 fc              	movl	-0x4(%rax,%r11), %r15d
1408ee9e1: 89 4c 24 34                 	movl	%ecx, 0x34(%rsp)
1408ee9e5: 85 c9                       	testl	%ecx, %ecx
1408ee9e7: 0f 85 aa 04 00 00           	jne	0x1408eee97 <.text+0x8ede97>
1408ee9ed: 4c 8b 33                    	movq	(%rbx), %r14
1408ee9f0: 41 8b 06                    	movl	(%r14), %eax
1408ee9f3: 89 44 24 2c                 	movl	%eax, 0x2c(%rsp)
1408ee9f7: 49 8d 4e 04                 	leaq	0x4(%r14), %rcx
1408ee9fb: e8 90 6d 2e 00              	callq	0x140bd5790 <.text+0xbd4790>
1408eea00: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
1408eea08: 48 8b 9e e0 00 00 00        	movq	0xe0(%rsi), %rbx
1408eea0f: 41 8b 0e                    	movl	(%r14), %ecx
1408eea12: e8 39 48 f4 ff              	callq	0x140833250 <.text+0x832250>
1408eea17: 89 c0                       	movl	%eax, %eax
1408eea19: 49 be 00 00 00 00 00 00 00 40       	movabsq	$0x4000000000000000, %r14 # imm = 0x4000000000000000
1408eea23: 49 09 c6                    	orq	%rax, %r14
1408eea26: 41 83 7d 00 02              	cmpl	$0x2, (%r13)
1408eea2b: 44 89 7c 24 30              	movl	%r15d, 0x30(%rsp)
1408eea30: 74 0b                       	je	0x1408eea3d <.text+0x8eda3d>
1408eea32: 4c 89 e9                    	movq	%r13, %rcx
1408eea35: e8 b6 d0 b6 00              	callq	0x14145baf0 <.text+0x145aaf0>
1408eea3a: 49 31 c6                    	xorq	%rax, %r14
1408eea3d: 4c 89 74 24 38              	movq	%r14, 0x38(%rsp)
1408eea42: 4c 8d 63 50                 	leaq	0x50(%rbx), %r12
1408eea46: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1408eea4b: 4c 89 e1                    	movq	%r12, %rcx
1408eea4e: e8 1d 23 cc ff              	callq	0x1405b0d70 <.text+0x5afd70>
1408eea53: 49 89 c7                    	movq	%rax, %r15
1408eea56: 49 89 c0                    	movq	%rax, %r8
1408eea59: 49 c1 e8 39                 	shrq	$0x39, %r8
1408eea5d: 48 8b 43 30                 	movq	0x30(%rbx), %rax
1408eea61: 48 8b 4b 38                 	movq	0x38(%rbx), %rcx
1408eea65: 48 89 ca                    	movq	%rcx, %rdx
1408eea68: 4c 21 fa                    	andq	%r15, %rdx
1408eea6b: 4c 89 84 24 90 00 00 00     	movq	%r8, 0x90(%rsp)
1408eea73: 66 41 0f 6e c0              	movd	%r8d, %xmm0
1408eea78: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408eea7c: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408eea81: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408eea86: 4c 8d 40 f0                 	leaq	-0x10(%rax), %r8
1408eea8a: 45 31 c9                    	xorl	%r9d, %r9d
1408eea8d: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408eea91: f3 0f 6f 14 10              	movdqu	(%rax,%rdx), %xmm2
1408eea96: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408eea9a: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408eea9e: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1408eeaa3: 45 85 d2                    	testl	%r10d, %r10d
1408eeaa6: 74 4d                       	je	0x1408eeaf5 <.text+0x8edaf5>
1408eeaa8: 48 89 9c 24 80 00 00 00     	movq	%rbx, 0x80(%rsp)
1408eeab0: 89 eb                       	movl	%ebp, %ebx
1408eeab2: 4d 89 fd                    	movq	%r15, %r13
1408eeab5: 4d 89 cf                    	movq	%r9, %r15
1408eeab8: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1408eeabd: 49 01 d3                    	addq	%rdx, %r11
1408eeac0: 49 21 cb                    	andq	%rcx, %r11
1408eeac3: 4d 89 d9                    	movq	%r11, %r9
1408eeac6: 49 c1 e1 04                 	shlq	$0x4, %r9
1408eeaca: 4c 89 c5                    	movq	%r8, %rbp
1408eeacd: 4d 29 c8                    	subq	%r9, %r8
1408eead0: 4d 39 30                    	cmpq	%r14, (%r8)
1408eead3: 74 47                       	je	0x1408eeb1c <.text+0x8edb1c>
1408eead5: 45 8d 42 ff                 	leal	-0x1(%r10), %r8d
1408eead9: 66 45 21 d0                 	andw	%r10w, %r8w
1408eeadd: 45 89 c2                    	movl	%r8d, %r10d
1408eeae0: 4d 89 f9                    	movq	%r15, %r9
1408eeae3: 4d 89 ef                    	movq	%r13, %r15
1408eeae6: 49 89 e8                    	movq	%rbp, %r8
1408eeae9: 89 dd                       	movl	%ebx, %ebp
1408eeaeb: 48 8b 9c 24 80 00 00 00     	movq	0x80(%rsp), %rbx
1408eeaf3: 75 bb                       	jne	0x1408eeab0 <.text+0x8edab0>
1408eeaf5: 4d 89 c2                    	movq	%r8, %r10
1408eeaf8: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408eeafc: 66 44 0f d7 c2              	pmovmskb	%xmm2, %r8d
1408eeb01: 45 85 c0                    	testl	%r8d, %r8d
1408eeb04: 75 25                       	jne	0x1408eeb2b <.text+0x8edb2b>
1408eeb06: 4c 01 ca                    	addq	%r9, %rdx
1408eeb09: 48 83 c2 10                 	addq	$0x10, %rdx
1408eeb0d: 49 83 c1 10                 	addq	$0x10, %r9
1408eeb11: 48 21 ca                    	andq	%rcx, %rdx
1408eeb14: 4d 89 d0                    	movq	%r10, %r8
1408eeb17: e9 75 ff ff ff              	jmp	0x1408eea91 <.text+0x8eda91>
1408eeb1c: 49 f7 db                    	negq	%r11
1408eeb1f: 49 c1 e3 04                 	shlq	$0x4, %r11
1408eeb23: 4c 01 d8                    	addq	%r11, %rax
1408eeb26: e9 22 02 00 00              	jmp	0x1408eed4d <.text+0x8edd4d>
1408eeb2b: 48 83 7b 40 00              	cmpq	$0x0, 0x40(%rbx)
1408eeb30: 0f 84 b2 03 00 00           	je	0x1408eeee8 <.text+0x8edee8>
1408eeb36: 4c 8d ab 90 00 00 00        	leaq	0x90(%rbx), %r13
1408eeb3d: 89 e9                       	movl	%ebp, %ecx
1408eeb3f: e8 cc 22 32 00              	callq	0x140c10e10 <.text+0xc0fe10>
1408eeb44: 89 e9                       	movl	%ebp, %ecx
1408eeb46: 84 c0                       	testb	%al, %al
1408eeb48: 0f 84 aa 00 00 00           	je	0x1408eebf8 <.text+0x8edbf8>
1408eeb4e: e8 2d d0 74 ff              	callq	0x14003bb80 <.text+0x3ab80>
1408eeb53: 4c 89 e9                    	movq	%r13, %rcx
1408eeb56: 44 8b 64 24 2c              	movl	0x2c(%rsp), %r12d
1408eeb5b: 44 89 e2                    	movl	%r12d, %edx
1408eeb5e: 41 89 e8                    	movl	%ebp, %r8d
1408eeb61: 45 31 c9                    	xorl	%r9d, %r9d
1408eeb64: e8 27 df 0e 00              	callq	0x1409dca90 <.text+0x9dba90>
1408eeb69: 44 89 64 24 28              	movl	%r12d, 0x28(%rsp)
1408eeb6e: 48 8b 84 24 50 01 00 00     	movq	0x150(%rsp), %rax
1408eeb76: 0f 10 00                    	movups	(%rax), %xmm0
1408eeb79: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1408eeb7d: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1408eeb81: 0f 10 58 2c                 	movups	0x2c(%rax), %xmm3
1408eeb85: 0f 11 9c 24 cc 00 00 00     	movups	%xmm3, 0xcc(%rsp)
1408eeb8d: 0f 29 94 24 c0 00 00 00     	movaps	%xmm2, 0xc0(%rsp)
1408eeb95: 0f 29 8c 24 b0 00 00 00     	movaps	%xmm1, 0xb0(%rsp)
1408eeb9d: 0f 29 84 24 a0 00 00 00     	movaps	%xmm0, 0xa0(%rsp)
1408eeba5: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1408eebaa: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
1408eebaf: 4c 8d 84 24 a0 00 00 00     	leaq	0xa0(%rsp), %r8
1408eebb7: e8 74 80 2e 00              	callq	0x140bd6c30 <.text+0xbd5c30>
1408eebbc: e9 a8 00 00 00              	jmp	0x1408eec69 <.text+0x8edc69>
1408eebc1: 48 f7 df                    	negq	%rdi
1408eebc4: 48 c1 e7 04                 	shlq	$0x4, %rdi
1408eebc8: 8b 7c 39 f8                 	movl	-0x8(%rcx,%rdi), %edi
1408eebcc: 48 8b 0b                    	movq	(%rbx), %rcx
1408eebcf: 48 83 c1 04                 	addq	$0x4, %rcx
1408eebd3: e8 b8 6b 2e 00              	callq	0x140bd5790 <.text+0xbd4790>
1408eebd8: 48 8b 96 c0 00 00 00        	movq	0xc0(%rsi), %rdx
1408eebdf: 48 39 fa                    	cmpq	%rdi, %rdx
1408eebe2: 0f 82 34 03 00 00           	jb	0x1408eef1c <.text+0x8edf1c>
1408eebe8: 48 8b 8e b8 00 00 00        	movq	0xb8(%rsi), %rcx
1408eebef: 48 89 04 39                 	movq	%rax, (%rcx,%rdi)
1408eebf3: e9 8b 02 00 00              	jmp	0x1408eee83 <.text+0x8ede83>
1408eebf8: e8 83 cf 74 ff              	callq	0x14003bb80 <.text+0x3ab80>
1408eebfd: 4c 89 e9                    	movq	%r13, %rcx
1408eec00: 44 8b 64 24 2c              	movl	0x2c(%rsp), %r12d
1408eec05: 44 89 e2                    	movl	%r12d, %edx
1408eec08: 41 89 e8                    	movl	%ebp, %r8d
1408eec0b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1408eec11: e8 7a de 0e 00              	callq	0x1409dca90 <.text+0x9dba90>
1408eec16: 44 89 64 24 28              	movl	%r12d, 0x28(%rsp)
1408eec1b: 48 8b 84 24 50 01 00 00     	movq	0x150(%rsp), %rax
1408eec23: 0f 10 00                    	movups	(%rax), %xmm0
1408eec26: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1408eec2a: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1408eec2e: 0f 10 58 2c                 	movups	0x2c(%rax), %xmm3
1408eec32: 0f 11 9c 24 cc 00 00 00     	movups	%xmm3, 0xcc(%rsp)
1408eec3a: 0f 29 94 24 c0 00 00 00     	movaps	%xmm2, 0xc0(%rsp)
1408eec42: 0f 29 8c 24 b0 00 00 00     	movaps	%xmm1, 0xb0(%rsp)
1408eec4a: 0f 29 84 24 a0 00 00 00     	movaps	%xmm0, 0xa0(%rsp)
1408eec52: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1408eec57: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
1408eec5c: 4c 8d 84 24 a0 00 00 00     	leaq	0xa0(%rsp), %r8
1408eec64: e8 97 7f 2e 00              	callq	0x140bd6c00 <.text+0xbd5c00>
1408eec69: 4c 8b 6b 28                 	movq	0x28(%rbx), %r13
1408eec6d: 49 81 fd 00 01 00 00        	cmpq	$0x100, %r13            # imm = 0x100
1408eec74: 0f 83 21 03 00 00           	jae	0x1408eef9b <.text+0x8edf9b>
1408eec7a: 48 8d 4b 18                 	leaq	0x18(%rbx), %rcx
1408eec7e: 4c 3b 29                    	cmpq	(%rcx), %r13
1408eec81: 75 0c                       	jne	0x1408eec8f <.text+0x8edc8f>
1408eec83: 48 8d 15 46 86 1b 02        	leaq	0x21b8646(%rip), %rdx   # 0x142aa72d0
1408eec8a: e8 c1 14 27 00              	callq	0x140b60150 <.text+0xb5f150>
1408eec8f: 48 8b 43 20                 	movq	0x20(%rbx), %rax
1408eec93: 4c 89 e9                    	movq	%r13, %rcx
1408eec96: 48 c1 e1 06                 	shlq	$0x6, %rcx
1408eec9a: 4a 8d 0c a9                 	leaq	(%rcx,%r13,4), %rcx
1408eec9e: 8b 54 24 78                 	movl	0x78(%rsp), %edx
1408eeca2: 89 54 08 40                 	movl	%edx, 0x40(%rax,%rcx)
1408eeca6: 0f 10 44 24 38              	movups	0x38(%rsp), %xmm0
1408eecab: f3 0f 6f 4c 24 48           	movdqu	0x48(%rsp), %xmm1
1408eecb1: f3 0f 6f 54 24 58           	movdqu	0x58(%rsp), %xmm2
1408eecb7: f3 0f 6f 5c 24 68           	movdqu	0x68(%rsp), %xmm3
1408eecbd: f3 0f 7f 5c 08 30           	movdqu	%xmm3, 0x30(%rax,%rcx)
1408eecc3: f3 0f 7f 54 08 20           	movdqu	%xmm2, 0x20(%rax,%rcx)
1408eecc9: f3 0f 7f 4c 08 10           	movdqu	%xmm1, 0x10(%rax,%rcx)
1408eeccf: 0f 11 04 08                 	movups	%xmm0, (%rax,%rcx)
1408eecd3: 49 8d 45 01                 	leaq	0x1(%r13), %rax
1408eecd7: 48 89 43 28                 	movq	%rax, 0x28(%rbx)
1408eecdb: 48 8b 4b 30                 	movq	0x30(%rbx), %rcx
1408eecdf: 48 8b 43 38                 	movq	0x38(%rbx), %rax
1408eece3: 49 21 c7                    	andq	%rax, %r15
1408eece6: f3 42 0f 6f 04 39           	movdqu	(%rcx,%r15), %xmm0
1408eecec: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1408eecf0: 85 d2                       	testl	%edx, %edx
1408eecf2: 0f 84 cc 01 00 00           	je	0x1408eeec4 <.text+0x8edec4>
1408eecf8: f3 0f bc d2                 	tzcntl	%edx, %edx
1408eecfc: 4c 01 fa                    	addq	%r15, %rdx
1408eecff: 48 21 c2                    	andq	%rax, %rdx
1408eed02: 44 0f b6 04 11              	movzbl	(%rcx,%rdx), %r8d
1408eed07: 45 84 c0                    	testb	%r8b, %r8b
1408eed0a: 0f 89 f6 01 00 00           	jns	0x1408eef06 <.text+0x8edf06>
1408eed10: 4c 8d 4a f0                 	leaq	-0x10(%rdx), %r9
1408eed14: 49 21 c1                    	andq	%rax, %r9
1408eed17: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
1408eed1f: 88 04 11                    	movb	%al, (%rcx,%rdx)
1408eed22: 42 88 44 09 10              	movb	%al, 0x10(%rcx,%r9)
1408eed27: 48 c1 e2 04                 	shlq	$0x4, %rdx
1408eed2b: 41 80 e0 01                 	andb	$0x1, %r8b
1408eed2f: 41 0f b6 c0                 	movzbl	%r8b, %eax
1408eed33: 48 29 43 40                 	subq	%rax, 0x40(%rbx)
1408eed37: 48 89 c8                    	movq	%rcx, %rax
1408eed3a: 48 29 d0                    	subq	%rdx, %rax
1408eed3d: 48 f7 da                    	negq	%rdx
1408eed40: 4c 89 74 11 f0              	movq	%r14, -0x10(%rcx,%rdx)
1408eed45: 44 88 68 f8                 	movb	%r13b, -0x8(%rax)
1408eed49: 48 ff 43 48                 	incq	0x48(%rbx)
1408eed4d: 44 8b 74 24 30              	movl	0x30(%rsp), %r14d
1408eed52: 0f b6 68 f8                 	movzbl	-0x8(%rax), %ebp
1408eed56: 48 8b 5e 20                 	movq	0x20(%rsi), %rbx
1408eed5a: 48 3b 5e 10                 	cmpq	0x10(%rsi), %rbx
1408eed5e: 75 10                       	jne	0x1408eed70 <.text+0x8edd70>
1408eed60: 48 8d 4e 10                 	leaq	0x10(%rsi), %rcx
1408eed64: 48 8d 15 35 85 1b 02        	leaq	0x21b8535(%rip), %rdx   # 0x142aa72a0
1408eed6b: e8 00 41 c7 ff              	callq	0x140562e70 <.text+0x561e70>
1408eed70: 48 8b 46 18                 	movq	0x18(%rsi), %rax
1408eed74: 44 89 34 d8                 	movl	%r14d, (%rax,%rbx,8)
1408eed78: 40 88 6c d8 04              	movb	%bpl, 0x4(%rax,%rbx,8)
1408eed7d: 48 ff c3                    	incq	%rbx
1408eed80: 48 89 5e 20                 	movq	%rbx, 0x20(%rsi)
1408eed84: 48 8b 9e e8 00 00 00        	movq	0xe8(%rsi), %rbx
1408eed8b: 48 89 f9                    	movq	%rdi, %rcx
1408eed8e: e8 bd 88 39 00              	callq	0x140c87650 <.text+0xc86650>
1408eed93: 80 be de 00 00 00 08        	cmpb	$0x8, 0xde(%rsi)
1408eed9a: 0f 84 e3 00 00 00           	je	0x1408eee83 <.text+0x8ede83>
1408eeda0: 48 83 bb e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rbx)
1408eeda8: 0f 84 d5 00 00 00           	je	0x1408eee83 <.text+0x8ede83>
1408eedae: 48 83 bb 30 01 00 00 00     	cmpq	$0x0, 0x130(%rbx)
1408eedb6: 0f 84 c7 00 00 00           	je	0x1408eee83 <.text+0x8ede83>
1408eedbc: 49 89 c1                    	movq	%rax, %r9
1408eedbf: 49 c1 e9 39                 	shrq	$0x39, %r9
1408eedc3: 48 8b 8b 18 01 00 00        	movq	0x118(%rbx), %rcx
1408eedca: 48 8b 93 20 01 00 00        	movq	0x120(%rbx), %rdx
1408eedd1: 49 89 d0                    	movq	%rdx, %r8
1408eedd4: 49 21 c0                    	andq	%rax, %r8
1408eedd7: 66 41 0f 6e c1              	movd	%r9d, %xmm0
1408eeddc: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408eede0: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408eede5: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408eedea: 4c 8d 49 f0                 	leaq	-0x10(%rcx), %r9
1408eedee: 45 31 d2                    	xorl	%r10d, %r10d
1408eedf1: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1408eedf5: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1408eedfb: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1408eedff: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1408eee03: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
1408eee08: 45 85 db                    	testl	%r11d, %r11d
1408eee0b: 74 2a                       	je	0x1408eee37 <.text+0x8ede37>
1408eee0d: f3 41 0f bc fb              	tzcntl	%r11d, %edi
1408eee12: 4c 01 c7                    	addq	%r8, %rdi
1408eee15: 48 21 d7                    	andq	%rdx, %rdi
1408eee18: 48 89 fb                    	movq	%rdi, %rbx
1408eee1b: 48 c1 e3 04                 	shlq	$0x4, %rbx
1408eee1f: 4d 89 ce                    	movq	%r9, %r14
1408eee22: 49 29 de                    	subq	%rbx, %r14
1408eee25: 49 39 06                    	cmpq	%rax, (%r14)
1408eee28: 74 2b                       	je	0x1408eee55 <.text+0x8ede55>
1408eee2a: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
1408eee2e: 66 44 21 df                 	andw	%r11w, %di
1408eee32: 41 89 fb                    	movl	%edi, %r11d
1408eee35: 75 d6                       	jne	0x1408eee0d <.text+0x8ede0d>
1408eee37: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1408eee3b: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1408eee40: 45 85 db                    	testl	%r11d, %r11d
1408eee43: 75 3e                       	jne	0x1408eee83 <.text+0x8ede83>
1408eee45: 4d 01 d0                    	addq	%r10, %r8
1408eee48: 49 83 c0 10                 	addq	$0x10, %r8
1408eee4c: 49 83 c2 10                 	addq	$0x10, %r10
1408eee50: 49 21 d0                    	andq	%rdx, %r8
1408eee53: eb a0                       	jmp	0x1408eedf5 <.text+0x8eddf5>
1408eee55: 48 f7 df                    	negq	%rdi
1408eee58: 48 c1 e7 04                 	shlq	$0x4, %rdi
1408eee5c: 8b 4c 39 f8                 	movl	-0x8(%rcx,%rdi), %ecx
1408eee60: 48 8b 96 c0 00 00 00        	movq	0xc0(%rsi), %rdx
1408eee67: 48 39 ca                    	cmpq	%rcx, %rdx
1408eee6a: 0f 82 bb 00 00 00           	jb	0x1408eef2b <.text+0x8edf2b>
1408eee70: 48 8b 86 b8 00 00 00        	movq	0xb8(%rsi), %rax
1408eee77: 48 8b 94 24 88 00 00 00     	movq	0x88(%rsp), %rdx
1408eee7f: 48 89 14 08                 	movq	%rdx, (%rax,%rcx)
1408eee83: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1408eee8a: 5b                          	popq	%rbx
1408eee8b: 5d                          	popq	%rbp
1408eee8c: 5f                          	popq	%rdi
1408eee8d: 5e                          	popq	%rsi
1408eee8e: 41 5c                       	popq	%r12
1408eee90: 41 5d                       	popq	%r13
1408eee92: 41 5e                       	popq	%r14
1408eee94: 41 5f                       	popq	%r15
1408eee96: c3                          	retq
1408eee97: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
1408eeea0: 48 8d 05 e1 83 1b 02        	leaq	0x21b83e1(%rip), %rax   # 0x142aa7288
1408eeea7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408eeeac: 4c 8d 05 7d 82 1b 02        	leaq	0x21b827d(%rip), %r8    # 0x142aa7130
1408eeeb3: 48 8d 54 24 34              	leaq	0x34(%rsp), %rdx
1408eeeb8: 4c 8d 4c 24 38              	leaq	0x38(%rsp), %r9
1408eeebd: 31 c9                       	xorl	%ecx, %ecx
1408eeebf: e8 8b aa fb 01              	callq	0x1428a994f <.text+0x28a894f>
1408eeec4: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1408eeeca: 4d 01 c7                    	addq	%r8, %r15
1408eeecd: 49 21 c7                    	andq	%rax, %r15
1408eeed0: f3 42 0f 6f 04 39           	movdqu	(%rcx,%r15), %xmm0
1408eeed6: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1408eeeda: 49 83 c0 10                 	addq	$0x10, %r8
1408eeede: 85 d2                       	testl	%edx, %edx
1408eeee0: 0f 85 12 fe ff ff           	jne	0x1408eecf8 <.text+0x8edcf8>
1408eeee6: eb e2                       	jmp	0x1408eeeca <.text+0x8edeca>
1408eeee8: 48 8d 4b 30                 	leaq	0x30(%rbx), %rcx
1408eeeec: ba 01 00 00 00              	movl	$0x1, %edx
1408eeef1: 4d 89 e0                    	movq	%r12, %r8
1408eeef4: 41 b1 01                    	movb	$0x1, %r9b
1408eeef7: e8 54 b3 fb 01              	callq	0x1428aa250 <.text+0x28a9250>
1408eeefc: 4c 8b 74 24 38              	movq	0x38(%rsp), %r14
1408eef01: e9 30 fc ff ff              	jmp	0x1408eeb36 <.text+0x8edb36>
1408eef06: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
1408eef0a: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1408eef0e: f3 0f bc d2                 	tzcntl	%edx, %edx
1408eef12: 44 0f b6 04 11              	movzbl	(%rcx,%rdx), %r8d
1408eef17: e9 f4 fd ff ff              	jmp	0x1408eed10 <.text+0x8edd10>
1408eef1c: 4c 8d 05 9d 6c 1c 02        	leaq	0x21c6c9d(%rip), %r8    # 0x142ab5bc0
1408eef23: 48 89 f9                    	movq	%rdi, %rcx
1408eef26: e8 05 a9 fe 01              	callq	0x1428d9830 <.text+0x28d8830>
1408eef2b: 4c 8d 05 8e 6c 1c 02        	leaq	0x21c6c8e(%rip), %r8    # 0x142ab5bc0
1408eef32: e8 f9 a8 fe 01              	callq	0x1428d9830 <.text+0x28d8830>
1408eef37: 48 8d 84 24 98 00 00 00     	leaq	0x98(%rsp), %rax
1408eef3f: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
1408eef47: 48 8d 05 72 de ff ff        	leaq	-0x218e(%rip), %rax     # 0x1408ecdc0 <.text+0x8ebdc0>
1408eef4e: 48 89 84 24 a8 00 00 00     	movq	%rax, 0xa8(%rsp)
1408eef56: 48 8d 05 b3 82 1b 02        	leaq	0x21b82b3(%rip), %rax   # 0x142aa7210
1408eef5d: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1408eef62: 48 c7 44 24 40 02 00 00 00  	movq	$0x2, 0x40(%rsp)
1408eef6b: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
1408eef74: 48 8d 84 24 a0 00 00 00     	leaq	0xa0(%rsp), %rax
1408eef7c: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
1408eef81: 48 c7 44 24 50 01 00 00 00  	movq	$0x1, 0x50(%rsp)
1408eef8a: 48 8d 15 27 83 1b 02        	leaq	0x21b8327(%rip), %rdx   # 0x142aa72b8
1408eef91: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1408eef96: e8 f5 a3 fe 01              	callq	0x1428d9390 <.text+0x28d8390>
1408eef9b: 48 8d 05 76 81 1b 02        	leaq	0x21b8176(%rip), %rax   # 0x142aa7118
1408eefa2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408eefa7: 48 8d 0d 82 7a 1b 02        	leaq	0x21b7a82(%rip), %rcx   # 0x142aa6a30
1408eefae: 4c 8d 0d eb 7a 1b 02        	leaq	0x21b7aeb(%rip), %r9    # 0x142aa6aa0
1408eefb5: 4c 8d 84 24 a0 00 00 00     	leaq	0xa0(%rsp), %r8
1408eefbd: ba 2b 00 00 00              	movl	$0x2b, %edx
1408eefc2: e8 99 a7 fe 01              	callq	0x1428d9760 <.text+0x28d8760>
1408eefc7: cc                          	int3
