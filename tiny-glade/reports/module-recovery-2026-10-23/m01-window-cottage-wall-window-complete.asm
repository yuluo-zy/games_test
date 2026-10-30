
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407ebfd0: 55                          	pushq	%rbp
1407ebfd1: 41 57                       	pushq	%r15
1407ebfd3: 41 56                       	pushq	%r14
1407ebfd5: 41 55                       	pushq	%r13
1407ebfd7: 41 54                       	pushq	%r12
1407ebfd9: 56                          	pushq	%rsi
1407ebfda: 57                          	pushq	%rdi
1407ebfdb: 53                          	pushq	%rbx
1407ebfdc: 48 81 ec e8 03 00 00        	subq	$0x3e8, %rsp            # imm = 0x3E8
1407ebfe3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407ebfeb: 44 0f 29 bd 50 03 00 00     	movaps	%xmm15, 0x350(%rbp)
1407ebff3: 44 0f 29 b5 40 03 00 00     	movaps	%xmm14, 0x340(%rbp)
1407ebffb: 44 0f 29 ad 30 03 00 00     	movaps	%xmm13, 0x330(%rbp)
1407ec003: 44 0f 29 a5 20 03 00 00     	movaps	%xmm12, 0x320(%rbp)
1407ec00b: 44 0f 29 9d 10 03 00 00     	movaps	%xmm11, 0x310(%rbp)
1407ec013: 44 0f 29 95 00 03 00 00     	movaps	%xmm10, 0x300(%rbp)
1407ec01b: 44 0f 29 8d f0 02 00 00     	movaps	%xmm9, 0x2f0(%rbp)
1407ec023: 44 0f 29 85 e0 02 00 00     	movaps	%xmm8, 0x2e0(%rbp)
1407ec02b: 0f 29 bd d0 02 00 00        	movaps	%xmm7, 0x2d0(%rbp)
1407ec032: 0f 29 b5 c0 02 00 00        	movaps	%xmm6, 0x2c0(%rbp)
1407ec039: 48 c7 85 b8 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2b8(%rbp)
1407ec044: 4c 89 cf                    	movq	%r9, %rdi
1407ec047: 4d 89 c7                    	movq	%r8, %r15
1407ec04a: 48 89 d3                    	movq	%rdx, %rbx
1407ec04d: 49 89 ce                    	movq	%rcx, %r14
1407ec050: e8 ab a9 42 00              	callq	0x140c16a00 <.text+0xc15a00>
1407ec055: 88 85 a7 02 00 00           	movb	%al, 0x2a7(%rbp)
1407ec05b: 84 c0                       	testb	%al, %al
1407ec05d: 0f 84 3a 02 00 00           	je	0x1407ec29d <.text+0x7eb29d>
1407ec063: 41 bc 56 00 00 00           	movl	$0x56, %r12d
1407ec069: b9 56 00 00 00              	movl	$0x56, %ecx
1407ec06e: e8 fd f9 ff ff              	callq	0x1407eba70 <.text+0x7eaa70>
1407ec073: 48 8d 35 ce e2 29 02        	leaq	0x229e2ce(%rip), %rsi   # 0x142a8a348
1407ec07a: 48 83 f8 01                 	cmpq	$0x1, %rax
1407ec07e: 0f 85 a1 00 00 00           	jne	0x1407ec125 <.text+0x7eb125>
1407ec084: 49 89 d1                    	movq	%rdx, %r9
1407ec087: 48 85 d2                    	testq	%rdx, %rdx
1407ec08a: 74 36                       	je	0x1407ec0c2 <.text+0x7eb0c2>
1407ec08c: 49 83 f9 56                 	cmpq	$0x56, %r9
1407ec090: 73 2e                       	jae	0x1407ec0c0 <.text+0x7eb0c0>
1407ec092: 48 8d 05 af e2 29 02        	leaq	0x229e2af(%rip), %rax   # 0x142a8a348
1407ec099: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407ec09e: 7f 22                       	jg	0x1407ec0c2 <.text+0x7eb0c2>
1407ec0a0: 48 8d 05 71 e3 29 02        	leaq	0x229e371(%rip), %rax   # 0x142a8a418
1407ec0a7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407ec0ac: 48 8d 0d 95 e2 29 02        	leaq	0x229e295(%rip), %rcx   # 0x142a8a348
1407ec0b3: ba 56 00 00 00              	movl	$0x56, %edx
1407ec0b8: 45 31 c0                    	xorl	%r8d, %r8d
1407ec0bb: e8 80 d8 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407ec0c0: 75 de                       	jne	0x1407ec0a0 <.text+0x7eb0a0>
1407ec0c2: 4c 89 c9                    	movq	%r9, %rcx
1407ec0c5: e8 a6 f9 ff ff              	callq	0x1407eba70 <.text+0x7eaa70>
1407ec0ca: 48 83 f8 01                 	cmpq	$0x1, %rax
1407ec0ce: 75 55                       	jne	0x1407ec125 <.text+0x7eb125>
1407ec0d0: 49 89 d0                    	movq	%rdx, %r8
1407ec0d3: 49 83 c0 02                 	addq	$0x2, %r8
1407ec0d7: 74 39                       	je	0x1407ec112 <.text+0x7eb112>
1407ec0d9: 49 83 f8 56                 	cmpq	$0x56, %r8
1407ec0dd: 73 31                       	jae	0x1407ec110 <.text+0x7eb110>
1407ec0df: 48 8d 05 62 e2 29 02        	leaq	0x229e262(%rip), %rax   # 0x142a8a348
1407ec0e6: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407ec0eb: 7f 25                       	jg	0x1407ec112 <.text+0x7eb112>
1407ec0ed: 48 8d 05 3c e3 29 02        	leaq	0x229e33c(%rip), %rax   # 0x142a8a430
1407ec0f4: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407ec0f9: 48 8d 0d 48 e2 29 02        	leaq	0x229e248(%rip), %rcx   # 0x142a8a348
1407ec100: ba 56 00 00 00              	movl	$0x56, %edx
1407ec105: 41 b9 56 00 00 00           	movl	$0x56, %r9d
1407ec10b: e8 30 d8 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407ec110: 75 db                       	jne	0x1407ec0ed <.text+0x7eb0ed>
1407ec112: 41 bc 54 00 00 00           	movl	$0x54, %r12d
1407ec118: 49 29 d4                    	subq	%rdx, %r12
1407ec11b: 48 8d 35 26 e2 29 02        	leaq	0x229e226(%rip), %rsi   # 0x142a8a348
1407ec122: 4c 01 c6                    	addq	%r8, %rsi
1407ec125: 4c 8d 05 46 e4 29 02        	leaq	0x229e446(%rip), %r8    # 0x142a8a572
1407ec12c: 4c 8d 2d fb e3 29 02        	leaq	0x229e3fb(%rip), %r13   # 0x142a8a52e
1407ec133: 4c 89 b5 a8 02 00 00        	movq	%r14, 0x2a8(%rbp)
1407ec13a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407ec140: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407ec145: 85 c0                       	testl	%eax, %eax
1407ec147: 78 17                       	js	0x1407ec160 <.text+0x7eb160>
1407ec149: 49 ff c8                    	decq	%r8
1407ec14c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407ec14f: 75 63                       	jne	0x1407ec1b4 <.text+0x7eb1b4>
1407ec151: eb 7c                       	jmp	0x1407ec1cf <.text+0x7eb1cf>
1407ec153: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407ec160: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407ec165: 80 f9 c0                    	cmpb	$-0x40, %cl
1407ec168: 7d 23                       	jge	0x1407ec18d <.text+0x7eb18d>
1407ec16a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407ec16f: 80 fa c0                    	cmpb	$-0x40, %dl
1407ec172: 7d 22                       	jge	0x1407ec196 <.text+0x7eb196>
1407ec174: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407ec179: 49 83 c0 fc                 	addq	$-0x4, %r8
1407ec17d: 41 83 e1 07                 	andl	$0x7, %r9d
1407ec181: 41 c1 e1 06                 	shll	$0x6, %r9d
1407ec185: 83 e2 3f                    	andl	$0x3f, %edx
1407ec188: 44 09 ca                    	orl	%r9d, %edx
1407ec18b: eb 10                       	jmp	0x1407ec19d <.text+0x7eb19d>
1407ec18d: 49 83 c0 fe                 	addq	$-0x2, %r8
1407ec191: 83 e1 1f                    	andl	$0x1f, %ecx
1407ec194: eb 0f                       	jmp	0x1407ec1a5 <.text+0x7eb1a5>
1407ec196: 49 83 c0 fd                 	addq	$-0x3, %r8
1407ec19a: 83 e2 0f                    	andl	$0xf, %edx
1407ec19d: c1 e2 06                    	shll	$0x6, %edx
1407ec1a0: 83 e1 3f                    	andl	$0x3f, %ecx
1407ec1a3: 09 d1                       	orl	%edx, %ecx
1407ec1a5: c1 e1 06                    	shll	$0x6, %ecx
1407ec1a8: 24 3f                       	andb	$0x3f, %al
1407ec1aa: 0f b6 c0                    	movzbl	%al, %eax
1407ec1ad: 09 c8                       	orl	%ecx, %eax
1407ec1af: 83 f8 5c                    	cmpl	$0x5c, %eax
1407ec1b2: 74 1b                       	je	0x1407ec1cf <.text+0x7eb1cf>
1407ec1b4: 83 f8 2f                    	cmpl	$0x2f, %eax
1407ec1b7: 74 16                       	je	0x1407ec1cf <.text+0x7eb1cf>
1407ec1b9: 4d 39 e8                    	cmpq	%r13, %r8
1407ec1bc: 75 82                       	jne	0x1407ec140 <.text+0x7eb140>
1407ec1be: 49 89 de                    	movq	%rbx, %r14
1407ec1c1: 48 89 fb                    	movq	%rdi, %rbx
1407ec1c4: 4c 89 ff                    	movq	%r15, %rdi
1407ec1c7: 41 bf 44 00 00 00           	movl	$0x44, %r15d
1407ec1cd: eb 56                       	jmp	0x1407ec225 <.text+0x7eb225>
1407ec1cf: 4c 8d 2d 58 e3 29 02        	leaq	0x229e358(%rip), %r13   # 0x142a8a52e
1407ec1d6: 4d 29 e8                    	subq	%r13, %r8
1407ec1d9: 49 ff c0                    	incq	%r8
1407ec1dc: 74 32                       	je	0x1407ec210 <.text+0x7eb210>
1407ec1de: 49 83 f8 44                 	cmpq	$0x44, %r8
1407ec1e2: 73 2a                       	jae	0x1407ec20e <.text+0x7eb20e>
1407ec1e4: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1407ec1e9: 7f 25                       	jg	0x1407ec210 <.text+0x7eb210>
1407ec1eb: 48 8d 05 06 e2 29 02        	leaq	0x229e206(%rip), %rax   # 0x142a8a3f8
1407ec1f2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407ec1f7: 48 8d 0d 30 e3 29 02        	leaq	0x229e330(%rip), %rcx   # 0x142a8a52e
1407ec1fe: ba 44 00 00 00              	movl	$0x44, %edx
1407ec203: 41 b9 44 00 00 00           	movl	$0x44, %r9d
1407ec209: e8 32 d7 0e 02              	callq	0x1428d9940 <.text+0x28d8940>
1407ec20e: 75 db                       	jne	0x1407ec1eb <.text+0x7eb1eb>
1407ec210: 49 89 de                    	movq	%rbx, %r14
1407ec213: 48 89 fb                    	movq	%rdi, %rbx
1407ec216: 4c 89 ff                    	movq	%r15, %rdi
1407ec219: 41 bf 44 00 00 00           	movl	$0x44, %r15d
1407ec21f: 4d 29 c7                    	subq	%r8, %r15
1407ec222: 4d 01 c5                    	addq	%r8, %r13
1407ec225: e8 06 bf 42 00              	callq	0x140c18130 <.text+0xc17130>
1407ec22a: 48 89 c1                    	movq	%rax, %rcx
1407ec22d: 48 8b 00                    	movq	(%rax), %rax
1407ec230: 48 83 f8 01                 	cmpq	$0x1, %rax
1407ec234: 0f 85 1b 2a 00 00           	jne	0x1407eec55 <.text+0x7edc55>
1407ec23a: 48 83 c1 08                 	addq	$0x8, %rcx
1407ec23e: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407ec242: 0f 85 68 2a 00 00           	jne	0x1407eecb0 <.text+0x7edcb0>
1407ec248: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407ec24f: 48 89 8d 78 02 00 00        	movq	%rcx, 0x278(%rbp)
1407ec256: 48 83 c1 08                 	addq	$0x8, %rcx
1407ec25a: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1407ec25f: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407ec268: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407ec271: 48 89 f2                    	movq	%rsi, %rdx
1407ec274: 4d 89 e0                    	movq	%r12, %r8
1407ec277: 4d 89 e9                    	movq	%r13, %r9
1407ec27a: e8 c1 aa 42 00              	callq	0x140c16d40 <.text+0xc15d40>
1407ec27f: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1407ec283: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1407ec28a: 48 ff 00                    	incq	(%rax)
1407ec28d: 49 89 ff                    	movq	%rdi, %r15
1407ec290: 48 89 df                    	movq	%rbx, %rdi
1407ec293: 4c 89 f3                    	movq	%r14, %rbx
1407ec296: 4c 8b b5 a8 02 00 00        	movq	0x2a8(%rbp), %r14
1407ec29d: 4c 89 f1                    	movq	%r14, %rcx
1407ec2a0: e8 cb a7 33 00              	callq	0x140b26a70 <.text+0xb25a70>
1407ec2a5: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1407ec2ac: 48 89 95 e8 00 00 00        	movq	%rdx, 0xe8(%rbp)
1407ec2b3: 48 8b 13                    	movq	(%rbx), %rdx
1407ec2b6: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ec2bd: e8 3e ee 22 00              	callq	0x140a1b100 <.text+0xa1a100>
1407ec2c2: 48 8b 8d d8 03 00 00        	movq	0x3d8(%rbp), %rcx
1407ec2c9: 48 8b 95 d0 03 00 00        	movq	0x3d0(%rbp), %rdx
1407ec2d0: 4c 8b ad 10 02 00 00        	movq	0x210(%rbp), %r13
1407ec2d7: 4c 8b 85 18 02 00 00        	movq	0x218(%rbp), %r8
1407ec2de: 0f b7 85 28 02 00 00        	movzwl	0x228(%rbp), %eax
1407ec2e5: 4c 8b 8d 30 02 00 00        	movq	0x230(%rbp), %r9
1407ec2ec: 4c 8b 17                    	movq	(%rdi), %r10
1407ec2ef: 4c 89 55 18                 	movq	%r10, 0x18(%rbp)
1407ec2f3: 49 83 c2 20                 	addq	$0x20, %r10
1407ec2f7: 4c 89 55 20                 	movq	%r10, 0x20(%rbp)
1407ec2fb: 44 8b 52 1c                 	movl	0x1c(%rdx), %r10d
1407ec2ff: 44 89 95 94 00 00 00        	movl	%r10d, 0x94(%rbp)
1407ec306: 4c 8b 12                    	movq	(%rdx), %r10
1407ec309: 4c 89 55 30                 	movq	%r10, 0x30(%rbp)
1407ec30d: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
1407ec311: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
1407ec315: 49 8b 17                    	movq	(%r15), %rdx
1407ec318: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
1407ec31c: 48 8b 09                    	movq	(%rcx), %rcx
1407ec31f: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
1407ec323: 44 0f 28 2d d5 db 29 02     	movaps	0x229dbd5(%rip), %xmm13 # 0x142a89f00
1407ec32b: 44 0f 28 3d ed db 29 02     	movaps	0x229dbed(%rip), %xmm15 # 0x142a89f20
1407ec333: 4c 89 85 f0 00 00 00        	movq	%r8, 0xf0(%rbp)
1407ec33a: 4c 89 8d b8 00 00 00        	movq	%r9, 0xb8(%rbp)
1407ec341: 4d 85 c9                    	testq	%r9, %r9
1407ec344: 0f 84 3e 27 00 00           	je	0x1407eea88 <.text+0x7eda88>
1407ec34a: 66 85 c0                    	testw	%ax, %ax
1407ec34d: 74 21                       	je	0x1407ec370 <.text+0x7eb370>
1407ec34f: 4d 85 ed                    	testq	%r13, %r13
1407ec352: 0f 84 30 27 00 00           	je	0x1407eea88 <.text+0x7eda88>
1407ec358: 8d 48 ff                    	leal	-0x1(%rax), %ecx
1407ec35b: 21 c1                       	andl	%eax, %ecx
1407ec35d: 89 8d 8c 01 00 00           	movl	%ecx, 0x18c(%rbp)
1407ec363: eb 4f                       	jmp	0x1407ec3b4 <.text+0x7eb3b4>
1407ec365: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407ec370: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1407ec377: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407ec380: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
1407ec384: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1407ec388: 49 81 c5 80 f6 ff ff        	addq	$-0x980, %r13           # imm = 0xF680
1407ec38f: 48 83 c1 10                 	addq	$0x10, %rcx
1407ec393: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1407ec398: 74 e6                       	je	0x1407ec380 <.text+0x7eb380>
1407ec39a: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1407ec3a1: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1407ec3a6: 29 c1                       	subl	%eax, %ecx
1407ec3a8: f7 d0                       	notl	%eax
1407ec3aa: 89 c2                       	movl	%eax, %edx
1407ec3ac: 21 ca                       	andl	%ecx, %edx
1407ec3ae: 89 95 8c 01 00 00           	movl	%edx, 0x18c(%rbp)
1407ec3b4: f3 0f bc c0                 	tzcntl	%eax, %eax
1407ec3b8: 48 f7 d8                    	negq	%rax
1407ec3bb: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
1407ec3c2: 4a 8d 14 28                 	leaq	(%rax,%r13), %rdx
1407ec3c6: 48 81 c2 70 ff ff ff        	addq	$-0x90, %rdx
1407ec3cd: 49 8b 84 05 68 ff ff ff     	movq	-0x98(%r13,%rax), %rax
1407ec3d5: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407ec3dc: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ec3e3: e8 88 ec 22 00              	callq	0x140a1b070 <.text+0xa1a070>
1407ec3e8: 48 ff 8d b8 00 00 00        	decq	0xb8(%rbp)
1407ec3ef: 4c 8b a5 10 02 00 00        	movq	0x210(%rbp), %r12
1407ec3f6: 4c 8b bd 18 02 00 00        	movq	0x218(%rbp), %r15
1407ec3fd: 44 0f b7 b5 28 02 00 00     	movzwl	0x228(%rbp), %r14d
1407ec405: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1407ec40c: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1407ec410: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
1407ec417: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1407ec41b: 48 83 c0 50                 	addq	$0x50, %rax
1407ec41f: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1407ec423: 4c 89 ad b0 00 00 00        	movq	%r13, 0xb0(%rbp)
1407ec42a: 48 83 7d 40 00              	cmpq	$0x0, 0x40(%rbp)
1407ec42f: 0f 84 28 26 00 00           	je	0x1407eea5d <.text+0x7eda5d>
1407ec435: 66 45 85 f6                 	testw	%r14w, %r14w
1407ec439: 74 15                       	je	0x1407ec450 <.text+0x7eb450>
1407ec43b: 41 8d 46 ff                 	leal	-0x1(%r14), %eax
1407ec43f: 44 21 f0                    	andl	%r14d, %eax
1407ec442: 4d 85 e4                    	testq	%r12, %r12
1407ec445: 75 34                       	jne	0x1407ec47b <.text+0x7eb47b>
1407ec447: e9 11 26 00 00              	jmp	0x1407eea5d <.text+0x7eda5d>
1407ec44c: 0f 1f 40 00                 	nopl	(%rax)
1407ec450: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1407ec455: 66 44 0f d7 f0              	pmovmskb	%xmm0, %r14d
1407ec45a: 49 83 c4 c0                 	addq	$-0x40, %r12
1407ec45e: 49 83 c7 10                 	addq	$0x10, %r15
1407ec462: 41 81 fe ff ff 00 00        	cmpl	$0xffff, %r14d          # imm = 0xFFFF
1407ec469: 74 e5                       	je	0x1407ec450 <.text+0x7eb450>
1407ec46b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1407ec470: 44 29 f1                    	subl	%r14d, %ecx
1407ec473: 41 f7 d6                    	notl	%r14d
1407ec476: 44 89 f0                    	movl	%r14d, %eax
1407ec479: 21 c8                       	andl	%ecx, %eax
1407ec47b: 44 89 f1                    	movl	%r14d, %ecx
1407ec47e: f3 0f bc c9                 	tzcntl	%ecx, %ecx
1407ec482: 41 89 c6                    	movl	%eax, %r14d
1407ec485: c1 e1 02                    	shll	$0x2, %ecx
1407ec488: 4c 89 e0                    	movq	%r12, %rax
1407ec48b: 48 29 c8                    	subq	%rcx, %rax
1407ec48e: 8b 48 fc                    	movl	-0x4(%rax), %ecx
1407ec491: 89 8d 10 02 00 00           	movl	%ecx, 0x210(%rbp)
1407ec497: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1407ec49b: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407ec4a0: 0f 84 98 26 00 00           	je	0x1407eeb3e <.text+0x7edb3e>
1407ec4a6: 48 89 8d 78 02 00 00        	movq	%rcx, 0x278(%rbp)
1407ec4ad: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1407ec4b1: 48 8d 95 10 02 00 00        	leaq	0x210(%rbp), %rdx
1407ec4b8: e8 13 4a dc ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1407ec4bd: 48 ff 4d 40                 	decq	0x40(%rbp)
1407ec4c1: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1407ec4c5: 48 8b 71 30                 	movq	0x30(%rcx), %rsi
1407ec4c9: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1407ec4cd: 49 89 cb                    	movq	%rcx, %r11
1407ec4d0: 49 21 c3                    	andq	%rax, %r11
1407ec4d3: 48 c1 e8 39                 	shrq	$0x39, %rax
1407ec4d7: 66 0f 6e c0                 	movd	%eax, %xmm0
1407ec4db: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407ec4df: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407ec4e4: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407ec4e9: 48 8d 46 b0                 	leaq	-0x50(%rsi), %rax
1407ec4ed: 45 31 c0                    	xorl	%r8d, %r8d
1407ec4f0: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
1407ec4f7: f3 42 0f 6f 0c 1e           	movdqu	(%rsi,%r11), %xmm1
1407ec4fd: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407ec501: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407ec505: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407ec50a: 45 85 c9                    	testl	%r9d, %r9d
1407ec50d: 74 31                       	je	0x1407ec540 <.text+0x7eb540>
1407ec50f: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407ec514: 4d 01 da                    	addq	%r11, %r10
1407ec517: 49 21 ca                    	andq	%rcx, %r10
1407ec51a: 49 f7 da                    	negq	%r10
1407ec51d: 4b 8d 3c 92                 	leaq	(%r10,%r10,4), %rdi
1407ec521: 48 c1 e7 04                 	shlq	$0x4, %rdi
1407ec525: 3b 14 38                    	cmpl	(%rax,%rdi), %edx
1407ec528: 74 46                       	je	0x1407ec570 <.text+0x7eb570>
1407ec52a: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407ec52e: 66 45 21 ca                 	andw	%r9w, %r10w
1407ec532: 45 89 d1                    	movl	%r10d, %r9d
1407ec535: 75 d8                       	jne	0x1407ec50f <.text+0x7eb50f>
1407ec537: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407ec540: 66 0f 74 0d 48 94 13 02     	pcmpeqb	0x2139448(%rip), %xmm1  # 0x142925990
1407ec548: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407ec54d: 45 85 c9                    	testl	%r9d, %r9d
1407ec550: 0f 85 e8 25 00 00           	jne	0x1407eeb3e <.text+0x7edb3e>
1407ec556: 4d 01 c3                    	addq	%r8, %r11
1407ec559: 49 83 c3 10                 	addq	$0x10, %r11
1407ec55d: 49 83 c0 10                 	addq	$0x10, %r8
1407ec561: 49 21 cb                    	andq	%rcx, %r11
1407ec564: eb 91                       	jmp	0x1407ec4f7 <.text+0x7eb4f7>
1407ec566: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407ec570: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1407ec577: e8 f4 a3 33 00              	callq	0x140b26970 <.text+0xb25970>
1407ec57c: 88 85 80 02 00 00           	movb	%al, 0x280(%rbp)
1407ec582: 0f b6 85 80 02 00 00        	movzbl	0x280(%rbp), %eax
1407ec589: 88 85 8f 02 00 00           	movb	%al, 0x28f(%rbp)
1407ec58f: 3c 01                       	cmpb	$0x1, %al
1407ec591: 0f 87 93 fe ff ff           	ja	0x1407ec42a <.text+0x7eb42a>
1407ec597: 48 8b 5d 18                 	movq	0x18(%rbp), %rbx
1407ec59b: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
1407ec5a0: 0f 84 a9 25 00 00           	je	0x1407eeb4f <.text+0x7edb4f>
1407ec5a6: 48 8d 04 3e                 	leaq	(%rsi,%rdi), %rax
1407ec5aa: 48 89 85 98 02 00 00        	movq	%rax, 0x298(%rbp)
1407ec5b1: 48 8d 04 3e                 	leaq	(%rsi,%rdi), %rax
1407ec5b5: 48 83 c0 b8                 	addq	$-0x48, %rax
1407ec5b9: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1407ec5c0: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
1407ec5c4: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1407ec5cb: e8 a0 47 dc ff              	callq	0x1405b0d70 <.text+0x5afd70>
1407ec5d0: 4c 8b 2b                    	movq	(%rbx), %r13
1407ec5d3: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1407ec5d7: 48 89 ca                    	movq	%rcx, %rdx
1407ec5da: 48 21 c2                    	andq	%rax, %rdx
1407ec5dd: 48 c1 e8 39                 	shrq	$0x39, %rax
1407ec5e1: 66 0f 6e c0                 	movd	%eax, %xmm0
1407ec5e5: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407ec5e9: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407ec5ee: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407ec5f3: 49 8d 85 58 fd ff ff        	leaq	-0x2a8(%r13), %rax
1407ec5fa: 4c 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %r8
1407ec601: 45 31 c9                    	xorl	%r9d, %r9d
1407ec604: f3 41 0f 6f 4c 15 00        	movdqu	(%r13,%rdx), %xmm1
1407ec60b: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407ec60f: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407ec613: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1407ec618: 45 85 d2                    	testl	%r10d, %r10d
1407ec61b: 74 28                       	je	0x1407ec645 <.text+0x7eb645>
1407ec61d: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1407ec622: 49 01 d3                    	addq	%rdx, %r11
1407ec625: 49 21 cb                    	andq	%rcx, %r11
1407ec628: 49 f7 db                    	negq	%r11
1407ec62b: 49 69 fb a8 02 00 00        	imulq	$0x2a8, %r11, %rdi      # imm = 0x2A8
1407ec632: 4c 3b 04 38                 	cmpq	(%rax,%rdi), %r8
1407ec636: 74 33                       	je	0x1407ec66b <.text+0x7eb66b>
1407ec638: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1407ec63c: 66 45 21 d3                 	andw	%r10w, %r11w
1407ec640: 45 89 da                    	movl	%r11d, %r10d
1407ec643: 75 d8                       	jne	0x1407ec61d <.text+0x7eb61d>
1407ec645: 66 0f 74 0d 43 93 13 02     	pcmpeqb	0x2139343(%rip), %xmm1  # 0x142925990
1407ec64d: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
1407ec652: 45 85 d2                    	testl	%r10d, %r10d
1407ec655: 0f 85 f4 24 00 00           	jne	0x1407eeb4f <.text+0x7edb4f>
1407ec65b: 4c 01 ca                    	addq	%r9, %rdx
1407ec65e: 48 83 c2 10                 	addq	$0x10, %rdx
1407ec662: 49 83 c1 10                 	addq	$0x10, %r9
1407ec666: 48 21 ca                    	andq	%rcx, %rdx
1407ec669: eb 99                       	jmp	0x1407ec604 <.text+0x7eb604>
1407ec66b: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ec672: 8a 58 fc                    	movb	-0x4(%rax), %bl
1407ec675: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1407ec67c: 0f b6 8d 80 02 00 00        	movzbl	0x280(%rbp), %ecx
1407ec683: e8 c8 6b 04 00              	callq	0x140833250 <.text+0x832250>
1407ec688: 48 8b b5 e8 00 00 00        	movq	0xe8(%rbp), %rsi
1407ec68f: 89 d9                       	movl	%ebx, %ecx
1407ec691: 89 c2                       	movl	%eax, %edx
1407ec693: e8 f8 95 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407ec698: 88 85 98 01 00 00           	movb	%al, 0x198(%rbp)
1407ec69e: 88 95 99 01 00 00           	movb	%dl, 0x199(%rbp)
1407ec6a4: 48 89 f1                    	movq	%rsi, %rcx
1407ec6a7: 48 8d 95 98 01 00 00        	leaq	0x198(%rbp), %rdx
1407ec6ae: e8 dd 08 a7 00              	callq	0x14125cf90 <.text+0x125bf90>
1407ec6b3: 48 85 c0                    	testq	%rax, %rax
1407ec6b6: 0f 84 46 26 00 00           	je	0x1407eed02 <.text+0x7edd02>
1407ec6bc: 0f 10 40 20                 	movups	0x20(%rax), %xmm0
1407ec6c0: 0f 10 48 30                 	movups	0x30(%rax), %xmm1
1407ec6c4: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
1407ec6cb: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
1407ec6d2: 8b 85 94 00 00 00           	movl	0x94(%rbp), %eax
1407ec6d8: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
1407ec6dc: 89 01                       	movl	%eax, (%rcx)
1407ec6de: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
1407ec6e5: e8 36 fe 0c 00              	callq	0x1408bc520 <.text+0x8bb520>
1407ec6ea: 49 89 d1                    	movq	%rdx, %r9
1407ec6ed: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
1407ec6f4: 0f b6 49 ff                 	movzbl	-0x1(%rcx), %ecx
1407ec6f8: 88 4c 24 20                 	movb	%cl, 0x20(%rsp)
1407ec6fc: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1407ec700: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
1407ec707: 49 89 c0                    	movq	%rax, %r8
1407ec70a: e8 a1 91 03 00              	callq	0x1408258b0 <.text+0x8248b0>
1407ec70f: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
1407ec716: 48 8b 95 98 02 00 00        	movq	0x298(%rbp), %rdx
1407ec71d: 0f b6 42 fc                 	movzbl	-0x4(%rdx), %eax
1407ec721: fe c8                       	decb	%al
1407ec723: 3c 03                       	cmpb	$0x3, %al
1407ec725: 0f 83 48 26 00 00           	jae	0x1407eed73 <.text+0x7edd73>
1407ec72b: 4e 8d 04 2f                 	leaq	(%rdi,%r13), %r8
1407ec72f: 4a 8d 0c 2f                 	leaq	(%rdi,%r13), %rcx
1407ec733: 48 81 c1 60 fd ff ff        	addq	$-0x2a0, %rcx           # imm = 0xFD60
1407ec73a: 48 89 8d 68 02 00 00        	movq	%rcx, 0x268(%rbp)
1407ec741: 0f b6 4a ff                 	movzbl	-0x1(%rdx), %ecx
1407ec745: 88 8d f8 01 00 00           	movb	%cl, 0x1f8(%rbp)
1407ec74b: 8b 4a e0                    	movl	-0x20(%rdx), %ecx
1407ec74e: 89 8d 90 01 00 00           	movl	%ecx, 0x190(%rbp)
1407ec754: 0f b6 c0                    	movzbl	%al, %eax
1407ec757: 48 8d 0d da e6 29 02        	leaq	0x229e6da(%rip), %rcx   # 0x142a8ae38
1407ec75e: 48 63 3c 81                 	movslq	(%rcx,%rax,4), %rdi
1407ec762: 4c 89 85 00 02 00 00        	movq	%r8, 0x200(%rbp)
1407ec769: 41 83 78 b8 01              	cmpl	$0x1, -0x48(%r8)
1407ec76e: 4c 8b 85 78 02 00 00        	movq	0x278(%rbp), %r8
1407ec775: 0f 85 a9 03 00 00           	jne	0x1407ecb24 <.text+0x7ebb24>
1407ec77b: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
1407ec782: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
1407ec786: 4c 8d 0d 5b de 29 02        	leaq	0x229de5b(%rip), %r9    # 0x142a8a5e8
1407ec78d: e8 8e e2 22 00              	callq	0x140a1aa20 <.text+0xa19a20>
1407ec792: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ec799: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
1407ec79d: 0f 83 a9 00 00 00           	jae	0x1407ec84c <.text+0x7eb84c>
1407ec7a3: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1407ec7aa: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407ec7ad: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407ec7b0: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1407ec7b4: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407ec7b8: 0f 28 da                    	movaps	%xmm2, %xmm3
1407ec7bb: 66 0f 15 da                 	unpckhpd	%xmm2, %xmm3            # xmm3 = xmm3[1],xmm2[1]
1407ec7bf: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407ec7c3: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
1407ec7c7: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
1407ec7cb: 0f 5c d3                    	subps	%xmm3, %xmm2
1407ec7ce: 0f 28 1d db 8e 29 02        	movaps	0x2298edb(%rip), %xmm3  # 0x142a856b0
1407ec7d5: 0f 59 d3                    	mulps	%xmm3, %xmm2
1407ec7d8: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407ec7db: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407ec7de: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
1407ec7e2: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407ec7e6: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407ec7e9: f3 0f 58 cb                 	addss	%xmm3, %xmm1
1407ec7ed: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407ec7f1: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407ec7f5: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407ec7f8: 0f 58 ca                    	addps	%xmm2, %xmm1
1407ec7fb: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407ec7fe: f3 0f 10 25 3a 45 29 02     	movss	0x229453a(%rip), %xmm4  # 0x142a80d40
1407ec806: 0f 59 d4                    	mulps	%xmm4, %xmm2
1407ec809: 0f 28 d8                    	movaps	%xmm0, %xmm3
1407ec80c: 0f c6 d8 c9                 	shufps	$0xc9, %xmm0, %xmm3     # xmm3 = xmm3[1,2],xmm0[0,3]
1407ec810: 0f 59 dc                    	mulps	%xmm4, %xmm3
1407ec813: 0f c6 d2 d2                 	shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
1407ec817: 0f 5c da                    	subps	%xmm2, %xmm3
1407ec81a: 0f 58 c0                    	addps	%xmm0, %xmm0
1407ec81d: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407ec821: 0f 59 c3                    	mulps	%xmm3, %xmm0
1407ec824: 0f 58 c1                    	addps	%xmm1, %xmm0
1407ec827: f2 0f 10 8d 20 01 00 00     	movsd	0x120(%rbp), %xmm1
1407ec82f: 0f 58 c8                    	addps	%xmm0, %xmm1
1407ec832: 0f 13 8d 20 01 00 00        	movlps	%xmm1, 0x120(%rbp)
1407ec839: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407ec83c: f3 0f 58 85 28 01 00 00     	addss	0x128(%rbp), %xmm0
1407ec844: f3 0f 11 85 28 01 00 00     	movss	%xmm0, 0x128(%rbp)
1407ec84c: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ec853: 8a 58 fc                    	movb	-0x4(%rax), %bl
1407ec856: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1407ec85d: 0f b6 8d 80 02 00 00        	movzbl	0x280(%rbp), %ecx
1407ec864: e8 e7 69 04 00              	callq	0x140833250 <.text+0x832250>
1407ec869: 48 8b b5 e8 00 00 00        	movq	0xe8(%rbp), %rsi
1407ec870: 89 d9                       	movl	%ebx, %ecx
1407ec872: 89 c2                       	movl	%eax, %edx
1407ec874: e8 17 94 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407ec879: 88 85 98 01 00 00           	movb	%al, 0x198(%rbp)
1407ec87f: 88 95 99 01 00 00           	movb	%dl, 0x199(%rbp)
1407ec885: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
1407ec88a: 0f 84 dc 22 00 00           	je	0x1407eeb6c <.text+0x7edb6c>
1407ec890: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
1407ec894: 48 8d 95 98 01 00 00        	leaq	0x198(%rbp), %rdx
1407ec89b: e8 20 b5 a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407ec8a0: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
1407ec8a7: 48 89 8d a8 02 00 00        	movq	%rcx, 0x2a8(%rbp)
1407ec8ae: 48 8d 0d 83 e5 29 02        	leaq	0x229e583(%rip), %rcx   # 0x142a8ae38
1407ec8b5: 48 01 cf                    	addq	%rcx, %rdi
1407ec8b8: 48 8b 0e                    	movq	(%rsi), %rcx
1407ec8bb: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
1407ec8bf: 49 89 d0                    	movq	%rdx, %r8
1407ec8c2: 49 21 c0                    	andq	%rax, %r8
1407ec8c5: 48 c1 e8 39                 	shrq	$0x39, %rax
1407ec8c9: 66 0f 6e c0                 	movd	%eax, %xmm0
1407ec8cd: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407ec8d1: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407ec8d6: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407ec8db: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407ec8df: 44 0f b6 8d 98 01 00 00     	movzbl	0x198(%rbp), %r9d
1407ec8e7: 44 0f b6 95 99 01 00 00     	movzbl	0x199(%rbp), %r10d
1407ec8ef: 45 31 db                    	xorl	%r11d, %r11d
1407ec8f2: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407ec8f8: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407ec8fc: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407ec900: 66 0f d7 f2                 	pmovmskb	%xmm2, %esi
1407ec904: 85 f6                       	testl	%esi, %esi
1407ec906: 74 73                       	je	0x1407ec97b <.text+0x7eb97b>
1407ec908: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407ec90c: 48 89 bd 90 02 00 00        	movq	%rdi, 0x290(%rbp)
1407ec913: 75 2a                       	jne	0x1407ec93f <.text+0x7eb93f>
1407ec915: f3 0f bc fe                 	tzcntl	%esi, %edi
1407ec919: 4c 01 c7                    	addq	%r8, %rdi
1407ec91c: 48 21 d7                    	andq	%rdx, %rdi
1407ec91f: 48 f7 df                    	negq	%rdi
1407ec922: 48 8d 1c ff                 	leaq	(%rdi,%rdi,8), %rbx
1407ec926: 80 3c d8 1a                 	cmpb	$0x1a, (%rax,%rbx,8)
1407ec92a: 74 76                       	je	0x1407ec9a2 <.text+0x7eb9a2>
1407ec92c: 8d 7e ff                    	leal	-0x1(%rsi), %edi
1407ec92f: 66 21 f7                    	andw	%si, %di
1407ec932: 89 fe                       	movl	%edi, %esi
1407ec934: 48 8b bd 90 02 00 00        	movq	0x290(%rbp), %rdi
1407ec93b: 75 d8                       	jne	0x1407ec915 <.text+0x7eb915>
1407ec93d: eb 3c                       	jmp	0x1407ec97b <.text+0x7eb97b>
1407ec93f: f3 0f bc fe                 	tzcntl	%esi, %edi
1407ec943: 4c 01 c7                    	addq	%r8, %rdi
1407ec946: 48 21 d7                    	andq	%rdx, %rdi
1407ec949: 48 f7 df                    	negq	%rdi
1407ec94c: 48 8d 1c ff                 	leaq	(%rdi,%rdi,8), %rbx
1407ec950: 44 0f b6 2c d8              	movzbl	(%rax,%rbx,8), %r13d
1407ec955: 41 80 fd 1a                 	cmpb	$0x1a, %r13b
1407ec959: 74 0f                       	je	0x1407ec96a <.text+0x7eb96a>
1407ec95b: 45 38 e9                    	cmpb	%r13b, %r9b
1407ec95e: 75 0a                       	jne	0x1407ec96a <.text+0x7eb96a>
1407ec960: 48 8d 1c d8                 	leaq	(%rax,%rbx,8), %rbx
1407ec964: 44 3a 53 01                 	cmpb	0x1(%rbx), %r10b
1407ec968: 74 38                       	je	0x1407ec9a2 <.text+0x7eb9a2>
1407ec96a: 8d 7e ff                    	leal	-0x1(%rsi), %edi
1407ec96d: 66 21 f7                    	andw	%si, %di
1407ec970: 89 fe                       	movl	%edi, %esi
1407ec972: 48 8b bd 90 02 00 00        	movq	0x290(%rbp), %rdi
1407ec979: 75 c4                       	jne	0x1407ec93f <.text+0x7eb93f>
1407ec97b: 66 0f 74 0d 0d 90 13 02     	pcmpeqb	0x213900d(%rip), %xmm1  # 0x142925990
1407ec983: 66 0f d7 f1                 	pmovmskb	%xmm1, %esi
1407ec987: 85 f6                       	testl	%esi, %esi
1407ec989: 0f 85 dd 21 00 00           	jne	0x1407eeb6c <.text+0x7edb6c>
1407ec98f: 4d 01 d8                    	addq	%r11, %r8
1407ec992: 49 83 c0 10                 	addq	$0x10, %r8
1407ec996: 49 83 c3 10                 	addq	$0x10, %r11
1407ec99a: 49 21 d0                    	andq	%rdx, %r8
1407ec99d: e9 50 ff ff ff              	jmp	0x1407ec8f2 <.text+0x7eb8f2>
1407ec9a2: 48 8d 04 ff                 	leaq	(%rdi,%rdi,8), %rax
1407ec9a6: 48 8b 7c c1 d8              	movq	-0x28(%rcx,%rax,8), %rdi
1407ec9ab: f0                          	lock
1407ec9ac: 48 ff 07                    	incq	(%rdi)
1407ec9af: 0f 8e c2 24 00 00           	jle	0x1407eee77 <.text+0x7ede77>
1407ec9b5: 48 89 bd 98 01 00 00        	movq	%rdi, 0x198(%rbp)
1407ec9bc: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ec9c3: 40 8a 70 fc                 	movb	-0x4(%rax), %sil
1407ec9c7: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1407ec9ce: c6 85 b7 02 00 00 01        	movb	$0x1, 0x2b7(%rbp)
1407ec9d5: 0f b6 8d 80 02 00 00        	movzbl	0x280(%rbp), %ecx
1407ec9dc: e8 6f 68 04 00              	callq	0x140833250 <.text+0x832250>
1407ec9e1: 48 8b 9d e8 00 00 00        	movq	0xe8(%rbp), %rbx
1407ec9e8: c6 85 b7 02 00 00 01        	movb	$0x1, 0x2b7(%rbp)
1407ec9ef: 89 f1                       	movl	%esi, %ecx
1407ec9f1: 89 c2                       	movl	%eax, %edx
1407ec9f3: e8 98 92 33 00              	callq	0x140b25c90 <.text+0xb24c90>
1407ec9f8: 88 85 50 01 00 00           	movb	%al, 0x150(%rbp)
1407ec9fe: 88 95 51 01 00 00           	movb	%dl, 0x151(%rbp)
1407eca04: 48 83 7b 48 00              	cmpq	$0x0, 0x48(%rbx)
1407eca09: 0f 84 ce 21 00 00           	je	0x1407eebdd <.text+0x7edbdd>
1407eca0f: 48 8d 4b 50                 	leaq	0x50(%rbx), %rcx
1407eca13: c6 85 b7 02 00 00 01        	movb	$0x1, 0x2b7(%rbp)
1407eca1a: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407eca21: e8 9a b3 a0 00              	callq	0x1411f7dc0 <.text+0x11f6dc0>
1407eca26: 48 8b 4b 30                 	movq	0x30(%rbx), %rcx
1407eca2a: 48 8b 53 38                 	movq	0x38(%rbx), %rdx
1407eca2e: 49 89 d0                    	movq	%rdx, %r8
1407eca31: 49 21 c0                    	andq	%rax, %r8
1407eca34: 48 c1 e8 39                 	shrq	$0x39, %rax
1407eca38: 66 0f 6e c0                 	movd	%eax, %xmm0
1407eca3c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407eca40: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407eca45: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407eca4a: 48 8d 41 b8                 	leaq	-0x48(%rcx), %rax
1407eca4e: 44 0f b6 8d 50 01 00 00     	movzbl	0x150(%rbp), %r9d
1407eca56: 44 0f b6 95 51 01 00 00     	movzbl	0x151(%rbp), %r10d
1407eca5e: 31 db                       	xorl	%ebx, %ebx
1407eca60: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1407eca66: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407eca6a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407eca6e: 66 0f d7 f2                 	pmovmskb	%xmm2, %esi
1407eca72: 85 f6                       	testl	%esi, %esi
1407eca74: 0f 84 81 00 00 00           	je	0x1407ecafb <.text+0x7ebafb>
1407eca7a: 41 80 f9 1a                 	cmpb	$0x1a, %r9b
1407eca7e: 75 31                       	jne	0x1407ecab1 <.text+0x7ebab1>
1407eca80: 49 89 dd                    	movq	%rbx, %r13
1407eca83: f3 0f bc de                 	tzcntl	%esi, %ebx
1407eca87: 4c 01 c3                    	addq	%r8, %rbx
1407eca8a: 48 21 d3                    	andq	%rdx, %rbx
1407eca8d: 48 f7 db                    	negq	%rbx
1407eca90: 4c 8d 1c db                 	leaq	(%rbx,%rbx,8), %r11
1407eca94: 42 80 3c d8 1a              	cmpb	$0x1a, (%rax,%r11,8)
1407eca99: 0f 84 8c 02 00 00           	je	0x1407ecd2b <.text+0x7ebd2b>
1407eca9f: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407ecaa3: 66 41 21 f3                 	andw	%si, %r11w
1407ecaa7: 44 89 de                    	movl	%r11d, %esi
1407ecaaa: 4c 89 eb                    	movq	%r13, %rbx
1407ecaad: 75 d1                       	jne	0x1407eca80 <.text+0x7eba80>
1407ecaaf: eb 4a                       	jmp	0x1407ecafb <.text+0x7ebafb>
1407ecab1: 48 89 9d 08 02 00 00        	movq	%rbx, 0x208(%rbp)
1407ecab8: f3 0f bc de                 	tzcntl	%esi, %ebx
1407ecabc: 4c 01 c3                    	addq	%r8, %rbx
1407ecabf: 48 21 d3                    	andq	%rdx, %rbx
1407ecac2: 48 f7 db                    	negq	%rbx
1407ecac5: 4c 8d 2c db                 	leaq	(%rbx,%rbx,8), %r13
1407ecac9: 46 0f b6 1c e8              	movzbl	(%rax,%r13,8), %r11d
1407ecace: 41 80 fb 1a                 	cmpb	$0x1a, %r11b
1407ecad2: 74 13                       	je	0x1407ecae7 <.text+0x7ebae7>
1407ecad4: 45 38 d9                    	cmpb	%r11b, %r9b
1407ecad7: 75 0e                       	jne	0x1407ecae7 <.text+0x7ebae7>
1407ecad9: 4e 8d 1c e8                 	leaq	(%rax,%r13,8), %r11
1407ecadd: 45 3a 53 01                 	cmpb	0x1(%r11), %r10b
1407ecae1: 0f 84 44 02 00 00           	je	0x1407ecd2b <.text+0x7ebd2b>
1407ecae7: 44 8d 5e ff                 	leal	-0x1(%rsi), %r11d
1407ecaeb: 66 41 21 f3                 	andw	%si, %r11w
1407ecaef: 44 89 de                    	movl	%r11d, %esi
1407ecaf2: 48 8b 9d 08 02 00 00        	movq	0x208(%rbp), %rbx
1407ecaf9: 75 bd                       	jne	0x1407ecab8 <.text+0x7ebab8>
1407ecafb: 66 0f 74 0d 8d 8e 13 02     	pcmpeqb	0x2138e8d(%rip), %xmm1  # 0x142925990
1407ecb03: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1407ecb08: 45 85 db                    	testl	%r11d, %r11d
1407ecb0b: 0f 85 cc 20 00 00           	jne	0x1407eebdd <.text+0x7edbdd>
1407ecb11: 49 01 d8                    	addq	%rbx, %r8
1407ecb14: 49 83 c0 10                 	addq	$0x10, %r8
1407ecb18: 48 83 c3 10                 	addq	$0x10, %rbx
1407ecb1c: 49 21 d0                    	andq	%rdx, %r8
1407ecb1f: e9 3c ff ff ff              	jmp	0x1407eca60 <.text+0x7eba60>
1407ecb24: 80 7a da 02                 	cmpb	$0x2, -0x26(%rdx)
1407ecb28: 0f 83 62 22 00 00           	jae	0x1407eed90 <.text+0x7edd90>
1407ecb2e: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1407ecb35: f2 0f 10 38                 	movsd	(%rax), %xmm7
1407ecb39: f2 0f 11 bd 00 01 00 00     	movsd	%xmm7, 0x100(%rbp)
1407ecb41: f3 0f 10 b5 d0 00 00 00     	movss	0xd0(%rbp), %xmm6
1407ecb49: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1407ecb50: e8 eb db 33 00              	callq	0x140b2a740 <.text+0xb29740>
1407ecb55: f3 0f 59 35 8b d5 16 02     	mulss	0x216d58b(%rip), %xmm6  # 0x14295a0e8
1407ecb5d: 89 85 10 02 00 00           	movl	%eax, 0x210(%rbp)
1407ecb63: f3 0f 11 85 14 02 00 00     	movss	%xmm0, 0x214(%rbp)
1407ecb6b: f3 0f 58 b5 00 01 00 00     	addss	0x100(%rbp), %xmm6
1407ecb73: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ecb7a: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407ecb7d: e8 2e 53 4a 00              	callq	0x140c91eb0 <.text+0xc90eb0>
1407ecb82: f3 0f 11 85 00 01 00 00     	movss	%xmm0, 0x100(%rbp)
1407ecb8a: 0f 13 bd 78 01 00 00        	movlps	%xmm7, 0x178(%rbp)
1407ecb91: f3 0f 10 b5 d0 00 00 00     	movss	0xd0(%rbp), %xmm6
1407ecb99: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1407ecba0: e8 9b db 33 00              	callq	0x140b2a740 <.text+0xb29740>
1407ecba5: f3 0f 59 35 c3 8c 13 02     	mulss	0x2138cc3(%rip), %xmm6  # 0x142925870
1407ecbad: 89 85 10 02 00 00           	movl	%eax, 0x210(%rbp)
1407ecbb3: f3 0f 11 85 14 02 00 00     	movss	%xmm0, 0x214(%rbp)
1407ecbbb: f3 0f 58 b5 78 01 00 00     	addss	0x178(%rbp), %xmm6
1407ecbc3: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ecbca: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407ecbcd: e8 de 52 4a 00              	callq	0x140c91eb0 <.text+0xc90eb0>
1407ecbd2: f3 0f 11 85 78 01 00 00     	movss	%xmm0, 0x178(%rbp)
1407ecbda: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ecbe1: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1407ecbe8: 4c 8b 85 68 02 00 00        	movq	0x268(%rbp), %r8
1407ecbef: e8 9c 54 4a 00              	callq	0x140c92090 <.text+0xc91090>
1407ecbf4: f3 44 0f 10 8d 10 02 00 00  	movss	0x210(%rbp), %xmm9
1407ecbfd: f3 44 0f 10 85 18 02 00 00  	movss	0x218(%rbp), %xmm8
1407ecc06: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ecc0d: 48 8d 95 78 01 00 00        	leaq	0x178(%rbp), %rdx
1407ecc14: 4c 8b 85 68 02 00 00        	movq	0x268(%rbp), %r8
1407ecc1b: e8 70 54 4a 00              	callq	0x140c92090 <.text+0xc91090>
1407ecc20: f3 44 0f 10 95 10 02 00 00  	movss	0x210(%rbp), %xmm10
1407ecc29: f3 0f 10 b5 18 02 00 00     	movss	0x218(%rbp), %xmm6
1407ecc31: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1407ecc35: 41 0f 14 c0                 	unpcklps	%xmm8, %xmm0            # xmm0 = xmm0[0],xmm8[0],xmm0[1],xmm8[1]
1407ecc39: 0f 28 15 a0 8c 13 02        	movaps	0x2138ca0(%rip), %xmm2  # 0x1429258e0
1407ecc40: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407ecc43: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407ecc47: 0f 14 ce                    	unpcklps	%xmm6, %xmm1            # xmm1 = xmm1[0],xmm6[0],xmm1[1],xmm6[1]
1407ecc4a: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407ecc4d: 0f 58 c8                    	addps	%xmm0, %xmm1
1407ecc50: 0f 13 8d 98 00 00 00        	movlps	%xmm1, 0x98(%rbp)
1407ecc57: f3 0f 16 d7                 	movshdup	%xmm7, %xmm2            # xmm2 = xmm7[1,1,3,3]
1407ecc5b: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1407ecc62: 48 8d 95 98 00 00 00        	leaq	0x98(%rbp), %rdx
1407ecc69: e8 72 01 4b 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407ecc6e: 41 0f 14 f2                 	unpcklps	%xmm10, %xmm6           # xmm6 = xmm6[0],xmm10[0],xmm6[1],xmm10[1]
1407ecc72: 45 0f 14 c1                 	unpcklps	%xmm9, %xmm8            # xmm8 = xmm8[0],xmm9[0],xmm8[1],xmm9[1]
1407ecc76: 41 0f 5c f0                 	subps	%xmm8, %xmm6
1407ecc7a: 0f 28 c6                    	movaps	%xmm6, %xmm0
1407ecc7d: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407ecc80: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407ecc84: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407ecc88: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407ecc8b: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1407ecc8f: f3 0f 10 0d 6d 8c 13 02     	movss	0x2138c6d(%rip), %xmm1  # 0x142925904
1407ecc97: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407ecc9b: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407ecc9e: 0f 57 05 4b 8c 13 02        	xorps	0x2138c4b(%rip), %xmm0  # 0x1429258f0
1407ecca5: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1407ecca8: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407eccab: 0f 13 85 50 01 00 00        	movlps	%xmm0, 0x150(%rbp)
1407eccb2: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407eccb9: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407eccc0: e8 fb 00 4b 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407eccc5: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ecccc: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407eccd3: e8 f8 44 4a 00              	callq	0x140c911d0 <.text+0xc901d0>
1407eccd8: 0f 28 85 10 02 00 00        	movaps	0x210(%rbp), %xmm0
1407eccdf: 8b 85 a0 01 00 00           	movl	0x1a0(%rbp), %eax
1407ecce5: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1407eccec: 89 41 08                    	movl	%eax, 0x8(%rcx)
1407eccef: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1407eccf6: 48 89 01                    	movq	%rax, (%rcx)
1407eccf9: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1407ecd00: 48 b8 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rax # imm = 0x3F8000003F800000
1407ecd0a: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
1407ecd0e: c7 41 14 00 00 80 3f        	movl	$0x3f800000, 0x14(%rcx) # imm = 0x3F800000
1407ecd15: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ecd1c: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
1407ecd20: 0f 82 7d fa ff ff           	jb	0x1407ec7a3 <.text+0x7eb7a3>
1407ecd26: e9 21 fb ff ff              	jmp	0x1407ec84c <.text+0x7eb84c>
1407ecd2b: 48 8d 04 db                 	leaq	(%rbx,%rbx,8), %rax
1407ecd2f: 4c 8b 44 c1 d8              	movq	-0x28(%rcx,%rax,8), %r8
1407ecd34: f0                          	lock
1407ecd35: 49 ff 00                    	incq	(%r8)
1407ecd38: 0f 8e 39 21 00 00           	jle	0x1407eee77 <.text+0x7ede77>
1407ecd3e: 4c 89 85 c0 01 00 00        	movq	%r8, 0x1c0(%rbp)
1407ecd45: f2 0f 10 85 20 01 00 00     	movsd	0x120(%rbp), %xmm0
1407ecd4d: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1407ecd51: f3 0f 10 85 28 01 00 00     	movss	0x128(%rbp), %xmm0
1407ecd59: f3 0f 11 85 64 02 00 00     	movss	%xmm0, 0x264(%rbp)
1407ecd61: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ecd68: 83 78 b8 01                 	cmpl	$0x1, -0x48(%rax)
1407ecd6c: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407ecd6f: 44 0f 28 a5 10 01 00 00     	movaps	0x110(%rbp), %xmm12
1407ecd77: 4c 8b ad b0 00 00 00        	movq	0xb0(%rbp), %r13
1407ecd7e: 4c 89 85 08 02 00 00        	movq	%r8, 0x208(%rbp)
1407ecd85: 0f 87 ba 00 00 00           	ja	0x1407ece45 <.text+0x7ebe45>
1407ecd8b: c6 85 a6 02 00 00 01        	movb	$0x1, 0x2a6(%rbp)
1407ecd92: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ecd99: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
1407ecda0: e8 bb b8 4a 00              	callq	0x140c98660 <.text+0xc97660>
1407ecda5: f3 0f 10 8d 18 02 00 00     	movss	0x218(%rbp), %xmm1
1407ecdad: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407ecdb0: f3 0f 59 05 d8 38 17 02     	mulss	0x21738d8(%rip), %xmm0  # 0x142960690
1407ecdb8: 0f c6 c1 40                 	shufps	$0x40, %xmm1, %xmm0     # xmm0 = xmm0[0,0],xmm1[0,1]
1407ecdbc: 0f 59 05 ed c3 29 02        	mulps	0x229c3ed(%rip), %xmm0  # 0x142a891b0
1407ecdc3: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1407ecdc7: 41 0f 59 d4                 	mulps	%xmm12, %xmm2
1407ecdcb: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1407ecdcf: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407ecdd3: 0f 28 da                    	movaps	%xmm2, %xmm3
1407ecdd6: 66 0f 15 da                 	unpckhpd	%xmm2, %xmm3            # xmm3 = xmm3[1],xmm2[1]
1407ecdda: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407ecdde: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
1407ecde2: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
1407ecde6: 0f 5c d3                    	subps	%xmm3, %xmm2
1407ecde9: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407ecdec: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407ecdf0: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407ecdf3: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
1407ecdf7: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407ecdfb: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407ecdfe: f3 0f 58 cb                 	addss	%xmm3, %xmm1
1407ece02: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407ece06: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407ece0a: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1407ece0e: 0f 58 ca                    	addps	%xmm2, %xmm1
1407ece11: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1407ece15: 41 0f c6 d4 d2              	shufps	$0xd2, %xmm12, %xmm2    # xmm2 = xmm2[2,0],xmm12[1,3]
1407ece1a: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407ece1d: 0f c6 c0 d2                 	shufps	$0xd2, %xmm0, %xmm0     # xmm0 = xmm0[2,0,1,3]
1407ece21: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407ece25: 0f 5c d0                    	subps	%xmm0, %xmm2
1407ece28: 0f c6 d2 d2                 	shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
1407ece2c: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1407ece30: 41 0f 58 c4                 	addps	%xmm12, %xmm0
1407ece34: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407ece38: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407ece3b: 0f 58 c1                    	addps	%xmm1, %xmm0
1407ece3e: 4c 8b 85 08 02 00 00        	movq	0x208(%rbp), %r8
1407ece45: 0f 28 4d 60                 	movaps	0x60(%rbp), %xmm1
1407ece49: 0f 58 c8                    	addps	%xmm0, %xmm1
1407ece4c: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407ece4f: f3 0f 58 85 64 02 00 00     	addss	0x264(%rbp), %xmm0
1407ece57: 0f 13 8d 10 02 00 00        	movlps	%xmm1, 0x210(%rbp)
1407ece5e: f3 0f 11 85 18 02 00 00     	movss	%xmm0, 0x218(%rbp)
1407ece66: 44 0f 29 65 e0              	movaps	%xmm12, -0x20(%rbp)
1407ece6b: c6 85 a6 02 00 00 00        	movb	$0x0, 0x2a6(%rbp)
1407ece72: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
1407ece76: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407ece7b: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
1407ece82: 48 89 fa                    	movq	%rdi, %rdx
1407ece85: 4c 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %r9
1407ece8c: e8 1f 84 03 00              	callq	0x1408252b0 <.text+0x8242b0>
1407ece91: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
1407ece98: 48 8b 02                    	movq	(%rdx), %rax
1407ece9b: 48 0b 42 08                 	orq	0x8(%rdx), %rax
1407ece9f: 0f 85 17 1e 00 00           	jne	0x1407eecbc <.text+0x7edcbc>
1407ecea5: 48 c7 42 08 00 00 00 00     	movq	$0x0, 0x8(%rdx)
1407ecead: 48 c7 02 01 00 00 00        	movq	$0x1, (%rdx)
1407eceb4: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1407ecebb: 0f 28 8d 20 01 00 00        	movaps	0x120(%rbp), %xmm1
1407ecec2: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1407ecec9: 0f 29 52 30                 	movaps	%xmm2, 0x30(%rdx)
1407ececd: 0f 29 4a 20                 	movaps	%xmm1, 0x20(%rdx)
1407eced1: 0f 29 42 10                 	movaps	%xmm0, 0x10(%rdx)
1407eced5: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407ecedc: 48 05 b0 00 00 00           	addq	$0xb0, %rax
1407ecee2: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1407ecee9: 66 0f 6f 85 10 01 00 00     	movdqa	0x110(%rbp), %xmm0
1407ecef1: 66 0f 6f 8d 20 01 00 00     	movdqa	0x120(%rbp), %xmm1
1407ecef9: 66 0f 6f 95 30 01 00 00     	movdqa	0x130(%rbp), %xmm2
1407ecf01: 66 0f 7f 85 c0 01 00 00     	movdqa	%xmm0, 0x1c0(%rbp)
1407ecf09: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
1407ecf11: 66 0f 7f 95 e0 01 00 00     	movdqa	%xmm2, 0x1e0(%rbp)
1407ecf19: 48 8d 85 13 02 00 00        	leaq	0x213(%rbp), %rax
1407ecf20: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
1407ecf27: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1407ecf2e: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
1407ecf35: 48 8b 8d 17 02 00 00        	movq	0x217(%rbp), %rcx
1407ecf3c: 48 89 8d 9f 00 00 00        	movq	%rcx, 0x9f(%rbp)
1407ecf43: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
1407ecf4a: 48 8b b2 c0 00 00 00        	movq	0xc0(%rdx), %rsi
1407ecf51: 48 3b b2 b0 00 00 00        	cmpq	0xb0(%rdx), %rsi
1407ecf58: 48 8b bd 90 02 00 00        	movq	0x290(%rbp), %rdi
1407ecf5f: 75 13                       	jne	0x1407ecf74 <.text+0x7ebf74>
1407ecf61: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1407ecf68: 48 8d 15 b1 d6 29 02        	leaq	0x229d6b1(%rip), %rdx   # 0x142a8a620
1407ecf6f: e8 3c e4 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407ecf74: 4c 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %r9
1407ecf7b: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407ecf82: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407ecf89: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407ecf91: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407ecf9a: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407ecfa3: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407ecfab: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1407ecfb2: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407ecfb7: 8b 95 08 01 00 00           	movl	0x108(%rbp), %edx
1407ecfbd: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407ecfc1: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407ecfc9: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
1407ecfd0: 0f 10 8d 20 02 00 00        	movups	0x220(%rbp), %xmm1
1407ecfd7: 0f 10 95 30 02 00 00        	movups	0x230(%rbp), %xmm2
1407ecfde: 0f 10 9d 3c 02 00 00        	movups	0x23c(%rbp), %xmm3
1407ecfe5: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407ecfea: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407ecfef: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407ecff4: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407ecff9: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407ed004: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1407ed008: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407ed010: 8b 55 78                    	movl	0x78(%rbp), %edx
1407ed013: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407ed01a: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407ed025: 48 8b 55 7c                 	movq	0x7c(%rbp), %rdx
1407ed029: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407ed031: 8b 95 84 00 00 00           	movl	0x84(%rbp), %edx
1407ed037: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407ed03e: 48 89 bc 08 a0 00 00 00     	movq	%rdi, 0xa0(%rax,%rcx)
1407ed046: 48 c7 84 08 a8 00 00 00 1d 00 00 00 	movq	$0x1d, 0xa8(%rax,%rcx)
1407ed052: 48 8d 15 a7 d5 29 02        	leaq	0x229d5a7(%rip), %rdx   # 0x142a8a600
1407ed059: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407ed061: 48 c7 84 08 b8 00 00 00 1e 00 00 00 	movq	$0x1e, 0xb8(%rax,%rcx)
1407ed06d: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1407ed074: 0f 28 8d d0 01 00 00        	movaps	0x1d0(%rbp), %xmm1
1407ed07b: 0f 28 95 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm2
1407ed082: 0f 29 94 08 e0 00 00 00     	movaps	%xmm2, 0xe0(%rax,%rcx)
1407ed08a: 0f 29 8c 08 d0 00 00 00     	movaps	%xmm1, 0xd0(%rax,%rcx)
1407ed092: 0f 29 84 08 c0 00 00 00     	movaps	%xmm0, 0xc0(%rax,%rcx)
1407ed09a: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407ed0a5: 8b 95 90 01 00 00           	movl	0x190(%rbp), %edx
1407ed0ab: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407ed0b2: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407ed0bd: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
1407ed0c4: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407ed0cc: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1407ed0d3: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407ed0db: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407ed0e3: 0f b6 95 8a 00 00 00        	movzbl	0x8a(%rbp), %edx
1407ed0ea: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407ed0f1: 0f b7 95 88 00 00 00        	movzwl	0x88(%rbp), %edx
1407ed0f8: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407ed100: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407ed10b: f3 0f 6f 85 98 01 00 00     	movdqu	0x198(%rbp), %xmm0
1407ed113: 0f 10 8d a8 01 00 00        	movups	0x1a8(%rbp), %xmm1
1407ed11a: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407ed123: 0f 11 8c 08 2c 01 00 00     	movups	%xmm1, 0x12c(%rax,%rcx)
1407ed12b: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407ed133: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
1407ed13a: 4c 8b 85 9f 00 00 00        	movq	0x9f(%rbp), %r8
1407ed141: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407ed149: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407ed151: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407ed159: 0f b6 95 f8 01 00 00        	movzbl	0x1f8(%rbp), %edx
1407ed160: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407ed167: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407ed171: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407ed178: 4c 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %r8
1407ed17f: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407ed187: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407ed18f: 48 ff c6                    	incq	%rsi
1407ed192: 49 89 b1 c0 00 00 00        	movq	%rsi, 0xc0(%r9)
1407ed199: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ed1a0: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
1407ed1a4: 0f 83 0b 04 00 00           	jae	0x1407ed5b5 <.text+0x7ec5b5>
1407ed1aa: 48 8b 95 98 02 00 00        	movq	0x298(%rbp), %rdx
1407ed1b1: 0f b6 5a fc                 	movzbl	-0x4(%rdx), %ebx
1407ed1b5: 8d 43 ff                    	leal	-0x1(%rbx), %eax
1407ed1b8: 3c 03                       	cmpb	$0x3, %al
1407ed1ba: 0f 83 5a 1c 00 00           	jae	0x1407eee1a <.text+0x7ede1a>
1407ed1c0: 0f b6 c0                    	movzbl	%al, %eax
1407ed1c3: 48 8d 0d 7a dc 29 02        	leaq	0x229dc7a(%rip), %rcx   # 0x142a8ae44
1407ed1ca: 48 63 3c 81                 	movslq	(%rcx,%rax,4), %rdi
1407ed1ce: 48 8d 85 13 02 00 00        	leaq	0x213(%rbp), %rax
1407ed1d5: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
1407ed1dc: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1407ed1e3: 48 8d 4a e8                 	leaq	-0x18(%rdx), %rcx
1407ed1e7: 48 8d 15 ba d4 29 02        	leaq	0x229d4ba(%rip), %rdx   # 0x142a8a6a8
1407ed1ee: e8 bd c1 04 00              	callq	0x1408393b0 <.text+0x8383b0>
1407ed1f3: 8b 00                       	movl	(%rax), %eax
1407ed1f5: 89 85 90 02 00 00           	movl	%eax, 0x290(%rbp)
1407ed1fb: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1407ed202: 8b 41 1c                    	movl	0x1c(%rcx), %eax
1407ed205: 89 85 60 01 00 00           	movl	%eax, 0x160(%rbp)
1407ed20b: f3 0f 6f 41 0c              	movdqu	0xc(%rcx), %xmm0
1407ed210: 66 0f 7f 85 50 01 00 00     	movdqa	%xmm0, 0x150(%rbp)
1407ed218: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
1407ed21f: 48 8b 8d 17 02 00 00        	movq	0x217(%rbp), %rcx
1407ed226: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
1407ed22d: 48 89 8d 9f 00 00 00        	movq	%rcx, 0x9f(%rbp)
1407ed234: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407ed23b: 48 8b b0 c0 00 00 00        	movq	0xc0(%rax), %rsi
1407ed242: 48 3b b0 b0 00 00 00        	cmpq	0xb0(%rax), %rsi
1407ed249: 75 13                       	jne	0x1407ed25e <.text+0x7ec25e>
1407ed24b: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1407ed252: 48 8d 15 67 d4 29 02        	leaq	0x229d467(%rip), %rdx   # 0x142a8a6c0
1407ed259: e8 52 e1 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407ed25e: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407ed262: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1407ed266: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1407ed26a: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407ed26e: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407ed271: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1407ed275: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407ed279: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407ed27d: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1407ed281: 0f 5c ca                    	subps	%xmm2, %xmm1
1407ed284: 0f 28 15 35 cc 29 02        	movaps	0x229cc35(%rip), %xmm2  # 0x142a89ec0
1407ed28b: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407ed28e: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1407ed292: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407ed295: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407ed299: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407ed29d: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407ed2a0: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407ed2a4: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407ed2a8: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407ed2ac: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407ed2b0: 0f 58 c1                    	addps	%xmm1, %xmm0
1407ed2b3: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407ed2b7: f3 0f 10 1d 95 10 29 02     	movss	0x2291095(%rip), %xmm3  # 0x142a7e354
1407ed2bf: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407ed2c2: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1407ed2c6: 41 0f c6 d4 c9              	shufps	$0xc9, %xmm12, %xmm2    # xmm2 = xmm2[1,2],xmm12[0,3]
1407ed2cb: 0f 59 d3                    	mulps	%xmm3, %xmm2
1407ed2ce: 0f c6 c9 d2                 	shufps	$0xd2, %xmm1, %xmm1     # xmm1 = xmm1[2,0,1,3]
1407ed2d2: 0f 5c d1                    	subps	%xmm1, %xmm2
1407ed2d5: 41 0f 28 dc                 	movaps	%xmm12, %xmm3
1407ed2d9: 41 0f 58 dc                 	addps	%xmm12, %xmm3
1407ed2dd: 0f c6 db ff                 	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
1407ed2e1: 0f 59 da                    	mulps	%xmm2, %xmm3
1407ed2e4: 0f 58 d8                    	addps	%xmm0, %xmm3
1407ed2e7: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
1407ed2eb: 0f 5c c3                    	subps	%xmm3, %xmm0
1407ed2ee: 0f 12 db                    	movhlps	%xmm3, %xmm3            # xmm3 = xmm3[1,1]
1407ed2f1: f3 0f 10 8d 64 02 00 00     	movss	0x264(%rbp), %xmm1
1407ed2f9: f3 0f 5c cb                 	subss	%xmm3, %xmm1
1407ed2fd: 48 8d 05 40 db 29 02        	leaq	0x229db40(%rip), %rax   # 0x142a8ae44
1407ed304: 48 01 c7                    	addq	%rax, %rdi
1407ed307: 48 69 d3 e8 03 00 00        	imulq	$0x3e8, %rbx, %rdx      # imm = 0x3E8
1407ed30e: 48 03 95 78 02 00 00        	addq	0x278(%rbp), %rdx
1407ed315: 4c 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %r9
1407ed31c: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407ed323: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407ed32a: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407ed332: 48 c7 44 08 10 01 00 00 00  	movq	$0x1, 0x10(%rax,%rcx)
1407ed33b: 48 89 54 08 18              	movq	%rdx, 0x18(%rax,%rcx)
1407ed340: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407ed349: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407ed351: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1407ed358: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407ed35d: 8b 95 08 01 00 00           	movl	0x108(%rbp), %edx
1407ed363: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407ed367: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407ed36f: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1407ed376: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1407ed37d: 0f 10 a5 30 02 00 00        	movups	0x230(%rbp), %xmm4
1407ed384: 0f 10 ad 3c 02 00 00        	movups	0x23c(%rbp), %xmm5
1407ed38b: 0f 11 54 08 44              	movups	%xmm2, 0x44(%rax,%rcx)
1407ed390: 0f 11 5c 08 54              	movups	%xmm3, 0x54(%rax,%rcx)
1407ed395: 0f 11 64 08 64              	movups	%xmm4, 0x64(%rax,%rcx)
1407ed39a: 0f 11 6c 08 70              	movups	%xmm5, 0x70(%rax,%rcx)
1407ed39f: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407ed3aa: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1407ed3ae: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407ed3b6: 8b 55 78                    	movl	0x78(%rbp), %edx
1407ed3b9: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407ed3c0: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407ed3cb: 48 8b 55 7c                 	movq	0x7c(%rbp), %rdx
1407ed3cf: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407ed3d7: 8b 95 84 00 00 00           	movl	0x84(%rbp), %edx
1407ed3dd: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407ed3e4: 48 89 bc 08 a0 00 00 00     	movq	%rdi, 0xa0(%rax,%rcx)
1407ed3ec: 48 c7 84 08 a8 00 00 00 23 00 00 00 	movq	$0x23, 0xa8(%rax,%rcx)
1407ed3f8: 48 8d 15 4d 3e 43 02        	leaq	0x2433e4d(%rip), %rdx   # 0x142c2124c
1407ed3ff: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407ed407: 48 c7 84 08 b8 00 00 00 24 00 00 00 	movq	$0x24, 0xb8(%rax,%rcx)
1407ed413: 44 0f 29 a4 08 c0 00 00 00  	movaps	%xmm12, 0xc0(%rax,%rcx)
1407ed41c: 0f 13 84 08 d0 00 00 00     	movlps	%xmm0, 0xd0(%rax,%rcx)
1407ed424: f3 0f 11 8c 08 d8 00 00 00  	movss	%xmm1, 0xd8(%rax,%rcx)
1407ed42d: 8b 95 60 01 00 00           	movl	0x160(%rbp), %edx
1407ed433: 89 94 08 ec 00 00 00        	movl	%edx, 0xec(%rax,%rcx)
1407ed43a: 0f 28 85 50 01 00 00        	movaps	0x150(%rbp), %xmm0
1407ed441: 0f 11 84 08 dc 00 00 00     	movups	%xmm0, 0xdc(%rax,%rcx)
1407ed449: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407ed454: 8b 95 90 02 00 00           	movl	0x290(%rbp), %edx
1407ed45a: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407ed461: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407ed46c: 0f 10 85 98 01 00 00        	movups	0x198(%rbp), %xmm0
1407ed473: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407ed47b: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
1407ed482: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407ed48a: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407ed492: 0f b6 95 8a 00 00 00        	movzbl	0x8a(%rbp), %edx
1407ed499: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407ed4a0: 0f b7 95 88 00 00 00        	movzwl	0x88(%rbp), %edx
1407ed4a7: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407ed4af: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407ed4ba: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1407ed4c1: 0f 10 8d d0 01 00 00        	movups	0x1d0(%rbp), %xmm1
1407ed4c8: 0f 11 8c 08 2c 01 00 00     	movups	%xmm1, 0x12c(%rax,%rcx)
1407ed4d0: 0f 11 84 08 1c 01 00 00     	movups	%xmm0, 0x11c(%rax,%rcx)
1407ed4d8: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407ed4e0: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
1407ed4e7: 4c 8b 85 9f 00 00 00        	movq	0x9f(%rbp), %r8
1407ed4ee: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407ed4f6: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407ed4fe: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407ed506: 0f b6 95 f8 01 00 00        	movzbl	0x1f8(%rbp), %edx
1407ed50d: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407ed514: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407ed51e: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407ed525: 4c 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %r8
1407ed52c: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407ed534: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407ed53c: 48 ff c6                    	incq	%rsi
1407ed53f: 49 89 b1 c0 00 00 00        	movq	%rsi, 0xc0(%r9)
1407ed546: f3 0f 10 85 d0 00 00 00     	movss	0xd0(%rbp), %xmm0
1407ed54e: f3 0f 11 85 90 02 00 00     	movss	%xmm0, 0x290(%rbp)
1407ed556: f3 44 0f 10 95 dc 00 00 00  	movss	0xdc(%rbp), %xmm10
1407ed55f: f3 44 0f 5c 95 d8 00 00 00  	subss	0xd8(%rbp), %xmm10
1407ed568: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ed56f: 83 78 b8 00                 	cmpl	$0x0, -0x48(%rax)
1407ed573: 75 62                       	jne	0x1407ed5d7 <.text+0x7ec5d7>
1407ed575: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ed57c: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407ed580: 83 f8 01                    	cmpl	$0x1, %eax
1407ed583: 0f 84 b7 14 00 00           	je	0x1407eea40 <.text+0x7eda40>
1407ed589: 83 f8 02                    	cmpl	$0x2, %eax
1407ed58c: 0f 84 91 14 00 00           	je	0x1407eea23 <.text+0x7eda23>
1407ed592: 83 f8 03                    	cmpl	$0x3, %eax
1407ed595: 0f 85 c4 18 00 00           	jne	0x1407eee5f <.text+0x7ede5f>
1407ed59b: f3 0f 10 85 90 02 00 00     	movss	0x290(%rbp), %xmm0
1407ed5a3: f3 0f 58 05 55 06 20 02     	addss	0x2200655(%rip), %xmm0  # 0x1429edc00
1407ed5ab: f3 0f 11 85 90 02 00 00     	movss	%xmm0, 0x290(%rbp)
1407ed5b3: eb 22                       	jmp	0x1407ed5d7 <.text+0x7ec5d7>
1407ed5b5: f3 0f 10 85 d0 00 00 00     	movss	0xd0(%rbp), %xmm0
1407ed5bd: f3 0f 11 85 90 02 00 00     	movss	%xmm0, 0x290(%rbp)
1407ed5c5: f3 44 0f 10 95 dc 00 00 00  	movss	0xdc(%rbp), %xmm10
1407ed5ce: f3 44 0f 5c 95 d8 00 00 00  	subss	0xd8(%rbp), %xmm10
1407ed5d7: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1407ed5de: 48 8d 15 33 d1 29 02        	leaq	0x229d133(%rip), %rdx   # 0x142a8a718
1407ed5e5: e8 76 5c 04 00              	callq	0x140833260 <.text+0x832260>
1407ed5ea: f2 0f 10 30                 	movsd	(%rax), %xmm6
1407ed5ee: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407ed5f5: 48 8b b0 d8 00 00 00        	movq	0xd8(%rax), %rsi
1407ed5fc: 48 3b b0 c8 00 00 00        	cmpq	0xc8(%rax), %rsi
1407ed603: 75 1a                       	jne	0x1407ed61f <.text+0x7ec61f>
1407ed605: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407ed60c: 48 8d 88 c8 00 00 00        	leaq	0xc8(%rax), %rcx
1407ed613: 48 8d 15 16 d1 29 02        	leaq	0x229d116(%rip), %rdx   # 0x142a8a730
1407ed61a: e8 e1 5f d7 ff              	callq	0x140563600 <.text+0x562600>
1407ed61f: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
1407ed626: 48 8b 82 d0 00 00 00        	movq	0xd0(%rdx), %rax
1407ed62d: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
1407ed631: 0f 13 34 88                 	movlps	%xmm6, (%rax,%rcx,4)
1407ed635: f3 0f 10 85 90 02 00 00     	movss	0x290(%rbp), %xmm0
1407ed63d: f3 0f 11 44 88 08           	movss	%xmm0, 0x8(%rax,%rcx,4)
1407ed643: f3 44 0f 11 54 88 0c        	movss	%xmm10, 0xc(%rax,%rcx,4)
1407ed64a: c6 44 88 10 00              	movb	$0x0, 0x10(%rax,%rcx,4)
1407ed64f: 48 ff c6                    	incq	%rsi
1407ed652: 48 89 b2 d8 00 00 00        	movq	%rsi, 0xd8(%rdx)
1407ed659: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ed660: 8b 48 e4                    	movl	-0x1c(%rax), %ecx
1407ed663: e8 d8 8b 4a 00              	callq	0x140c96240 <.text+0xc95240>
1407ed668: 48 89 c7                    	movq	%rax, %rdi
1407ed66b: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ed672: 0f b6 70 fc                 	movzbl	-0x4(%rax), %esi
1407ed676: 48 83 fe 02                 	cmpq	$0x2, %rsi
1407ed67a: 0f 85 e7 01 00 00           	jne	0x1407ed867 <.text+0x7ec867>
1407ed680: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407ed68a: 48 01 c7                    	addq	%rax, %rdi
1407ed68d: 48 89 f8                    	movq	%rdi, %rax
1407ed690: 48 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rcx # imm = 0xE7037ED1A0B428DB
1407ed69a: 48 31 c8                    	xorq	%rcx, %rax
1407ed69d: 48 f7 e7                    	mulq	%rdi
1407ed6a0: 31 d0                       	xorl	%edx, %eax
1407ed6a2: c1 e8 1f                    	shrl	$0x1f, %eax
1407ed6a5: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
1407ed6ac: 80 bd 80 02 00 00 01        	cmpb	$0x1, 0x280(%rbp)
1407ed6b3: 0f 84 c6 01 00 00           	je	0x1407ed87f <.text+0x7ec87f>
1407ed6b9: be 02 00 00 00              	movl	$0x2, %esi
1407ed6be: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
1407ed6c2: bb 03 00 00 00              	movl	$0x3, %ebx
1407ed6c7: f3 44 0f 10 b5 d0 00 00 00  	movss	0xd0(%rbp), %xmm14
1407ed6d0: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1407ed6d7: 48 8d 15 12 db 43 02        	leaq	0x243db12(%rip), %rdx   # 0x142c2b1f0
1407ed6de: e8 7d 5b 04 00              	callq	0x140833260 <.text+0x832260>
1407ed6e3: f3 0f 10 38                 	movss	(%rax), %xmm7
1407ed6e7: f3 44 0f 10 58 04           	movss	0x4(%rax), %xmm11
1407ed6ed: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1407ed6f4: e8 47 d0 33 00              	callq	0x140b2a740 <.text+0xb29740>
1407ed6f9: 89 85 44 01 00 00           	movl	%eax, 0x144(%rbp)
1407ed6ff: 0f 28 f0                    	movaps	%xmm0, %xmm6
1407ed702: 45 0f 28 c6                 	movaps	%xmm14, %xmm8
1407ed706: f3 44 0f 59 05 d9 c9 16 02  	mulss	0x216c9d9(%rip), %xmm8  # 0x14295a0e8
1407ed70f: 8b 85 44 01 00 00           	movl	0x144(%rbp), %eax
1407ed715: 89 85 10 02 00 00           	movl	%eax, 0x210(%rbp)
1407ed71b: f3 0f 11 85 14 02 00 00     	movss	%xmm0, 0x214(%rbp)
1407ed723: f3 41 0f 58 f8              	addss	%xmm8, %xmm7
1407ed728: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ed72f: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407ed732: e8 e9 47 4a 00              	callq	0x140c91f20 <.text+0xc90f20>
1407ed737: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407ed73a: f3 0f 2a ce                 	cvtsi2ss	%esi, %xmm1
1407ed73e: f3 44 0f 59 05 49 2f 17 02  	mulss	0x2172f49(%rip), %xmm8  # 0x142960690
1407ed747: f3 45 0f 58 c3              	addss	%xmm11, %xmm8
1407ed74c: f3 44 0f 5e f1              	divss	%xmm1, %xmm14
1407ed751: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1407ed75c: 48 89 9d 18 02 00 00        	movq	%rbx, 0x218(%rbp)
1407ed763: 8b 85 44 01 00 00           	movl	0x144(%rbp), %eax
1407ed769: 89 85 20 02 00 00           	movl	%eax, 0x220(%rbp)
1407ed76f: f3 0f 11 b5 24 02 00 00     	movss	%xmm6, 0x224(%rbp)
1407ed777: f3 0f 11 85 28 02 00 00     	movss	%xmm0, 0x228(%rbp)
1407ed77f: f3 44 0f 11 85 2c 02 00 00  	movss	%xmm8, 0x22c(%rbp)
1407ed788: f3 44 0f 11 b5 30 02 00 00  	movss	%xmm14, 0x230(%rbp)
1407ed791: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
1407ed798: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
1407ed79f: c7 85 40 02 00 00 00 00 00 00       	movl	$0x0, 0x240(%rbp)
1407ed7a9: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407ed7b0: 48 8d 95 10 02 00 00        	leaq	0x210(%rbp), %rdx
1407ed7b7: 4c 8d 05 72 cb 29 02        	leaq	0x229cb72(%rip), %r8    # 0x142a8a330
1407ed7be: e8 dd 34 a7 00              	callq	0x141260ca0 <.text+0x125fca0>
1407ed7c3: 4c 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %r8
1407ed7ca: 4c 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %r9
1407ed7d1: f3 44 0f 11 4c 24 48        	movss	%xmm9, 0x48(%rsp)
1407ed7d8: 48 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %rax
1407ed7df: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1407ed7e4: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407ed7eb: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1407ed7f0: 8b 85 90 01 00 00           	movl	0x190(%rbp), %eax
1407ed7f6: 89 44 24 28                 	movl	%eax, 0x28(%rsp)
1407ed7fa: 0f b6 85 f8 01 00 00        	movzbl	0x1f8(%rbp), %eax
1407ed801: 88 44 24 20                 	movb	%al, 0x20(%rsp)
1407ed805: c7 44 24 50 00 00 00 00     	movl	$0x0, 0x50(%rsp)
1407ed80d: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
1407ed814: 48 8b 95 70 02 00 00        	movq	0x270(%rbp), %rdx
1407ed81b: 4c 89 85 68 02 00 00        	movq	%r8, 0x268(%rbp)
1407ed822: e8 69 21 00 00              	callq	0x1407ef990 <.text+0x7ee990>
1407ed827: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407ed82e: 48 85 c0                    	testq	%rax, %rax
1407ed831: 0f 84 85 06 00 00           	je	0x1407edebc <.text+0x7ecebc>
1407ed837: 48 c1 e0 04                 	shlq	$0x4, %rax
1407ed83b: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407ed83f: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407ed845: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1407ed84c: e8 cf 63 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407ed851: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ed858: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
1407ed85c: 0f 83 6b 06 00 00           	jae	0x1407edecd <.text+0x7ececd>
1407ed862: e9 64 0a 00 00              	jmp	0x1407ee2cb <.text+0x7ed2cb>
1407ed867: 80 bd 80 02 00 00 01        	cmpb	$0x1, 0x280(%rbp)
1407ed86e: 0f 85 02 0e 00 00           	jne	0x1407ee676 <.text+0x7ed676>
1407ed874: 48 c7 85 48 01 00 00 00 00 00 00    	movq	$0x0, 0x148(%rbp)
1407ed87f: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ed886: 83 78 b8 01                 	cmpl	$0x1, -0x48(%rax)
1407ed88a: 0f 87 48 06 00 00           	ja	0x1407eded8 <.text+0x7eced8>
1407ed890: 0f 28 85 c0 00 00 00        	movaps	0xc0(%rbp), %xmm0
1407ed897: 0f 28 8d d0 00 00 00        	movaps	0xd0(%rbp), %xmm1
1407ed89e: 0f 29 8d 60 01 00 00        	movaps	%xmm1, 0x160(%rbp)
1407ed8a5: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1407ed8ac: c7 85 64 01 00 00 ae 47 21 3f       	movl	$0x3f2147ae, 0x164(%rbp) # imm = 0x3F2147AE
1407ed8b6: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407ed8ba: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1407ed8be: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1407ed8c2: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407ed8c6: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407ed8c9: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1407ed8cd: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407ed8d1: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407ed8d5: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1407ed8d9: 0f 5c ca                    	subps	%xmm2, %xmm1
1407ed8dc: 0f 28 15 dd f7 1d 02        	movaps	0x21df7dd(%rip), %xmm2  # 0x1429cd0c0
1407ed8e3: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407ed8e6: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1407ed8ea: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407ed8ed: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407ed8f1: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407ed8f5: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407ed8f8: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407ed8fc: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407ed900: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407ed904: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407ed908: 0f 58 c1                    	addps	%xmm1, %xmm0
1407ed90b: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407ed90f: f3 0f 10 1d ed 7f 13 02     	movss	0x2137fed(%rip), %xmm3  # 0x142925904
1407ed917: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407ed91a: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1407ed91e: 41 0f c6 d4 c9              	shufps	$0xc9, %xmm12, %xmm2    # xmm2 = xmm2[1,2],xmm12[0,3]
1407ed923: 0f 59 d3                    	mulps	%xmm3, %xmm2
1407ed926: 0f c6 c9 d2                 	shufps	$0xd2, %xmm1, %xmm1     # xmm1 = xmm1[2,0,1,3]
1407ed92a: 0f 5c d1                    	subps	%xmm1, %xmm2
1407ed92d: 41 0f 28 f4                 	movaps	%xmm12, %xmm6
1407ed931: 41 0f 58 f4                 	addps	%xmm12, %xmm6
1407ed935: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1407ed939: 0f 59 f2                    	mulps	%xmm2, %xmm6
1407ed93c: 0f 58 f0                    	addps	%xmm0, %xmm6
1407ed93f: 0f 28 fe                    	movaps	%xmm6, %xmm7
1407ed942: 66 0f 15 fe                 	unpckhpd	%xmm6, %xmm7            # xmm7 = xmm7[1],xmm6[1]
1407ed946: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407ed949: f3 0f 59 05 7f c5 29 02     	mulss	0x229c57f(%rip), %xmm0  # 0x142a89ed0
1407ed951: f3 0f 59 05 17 7f 13 02     	mulss	0x2137f17(%rip), %xmm0  # 0x142925870
1407ed959: f3 0f 58 85 64 02 00 00     	addss	0x264(%rbp), %xmm0
1407ed961: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407ed964: 0f 59 0d 75 c5 29 02        	mulps	0x229c575(%rip), %xmm1  # 0x142a89ee0
1407ed96b: 0f 59 0d 6e 7f 13 02        	mulps	0x2137f6e(%rip), %xmm1  # 0x1429258e0
1407ed972: 0f 58 4d 60                 	addps	0x60(%rbp), %xmm1
1407ed976: 0f 13 4d 00                 	movlps	%xmm1, (%rbp)
1407ed97a: f3 0f 11 45 08              	movss	%xmm0, 0x8(%rbp)
1407ed97f: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1407ed986: e8 45 ac 89 ff              	callq	0x1400885d0 <.text+0x875d0>
1407ed98b: 48 89 c6                    	movq	%rax, %rsi
1407ed98e: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ed995: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407ed99c: e8 bf ac 4a 00              	callq	0x140c98660 <.text+0xc97660>
1407ed9a1: f3 0f 10 85 14 02 00 00     	movss	0x214(%rbp), %xmm0
1407ed9a9: f3 0f 58 05 df 2c 17 02     	addss	0x2172cdf(%rip), %xmm0  # 0x142960690
1407ed9b1: f3 0f 10 8d 18 02 00 00     	movss	0x218(%rbp), %xmm1
1407ed9b9: f3 0f 58 0d af 7e 13 02     	addss	0x2137eaf(%rip), %xmm1  # 0x142925870
1407ed9c1: f3 0f 10 95 10 02 00 00     	movss	0x210(%rbp), %xmm2
1407ed9c9: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
1407ed9cc: 0f 58 15 1d c5 29 02        	addps	0x229c51d(%rip), %xmm2  # 0x142a89ef0
1407ed9d3: 0f 13 95 c0 01 00 00        	movlps	%xmm2, 0x1c0(%rbp)
1407ed9da: f3 0f 11 8d c8 01 00 00     	movss	%xmm1, 0x1c8(%rbp)
1407ed9e2: f3 0f 11 7c 24 28           	movss	%xmm7, 0x28(%rsp)
1407ed9e8: f3 0f 11 74 24 20           	movss	%xmm6, 0x20(%rsp)
1407ed9ee: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ed9f5: 89 f2                       	movl	%esi, %edx
1407ed9f7: 49 89 e8                    	movq	%rbp, %r8
1407ed9fa: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
1407eda01: e8 da 8c a9 00              	callq	0x1412866e0 <.text+0x12856e0>
1407eda06: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407eda0d: 48 8d b0 e0 00 00 00        	leaq	0xe0(%rax), %rsi
1407eda14: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407eda1b: 48 8b 98 f0 00 00 00        	movq	0xf0(%rax), %rbx
1407eda22: 48 3b 98 e0 00 00 00        	cmpq	0xe0(%rax), %rbx
1407eda29: 75 0f                       	jne	0x1407eda3a <.text+0x7eca3a>
1407eda2b: 48 89 f1                    	movq	%rsi, %rcx
1407eda2e: 48 8d 15 13 cd 29 02        	leaq	0x229cd13(%rip), %rdx   # 0x142a8a748
1407eda35: e8 b6 db 03 00              	callq	0x14082b5f0 <.text+0x82a5f0>
1407eda3a: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
1407eda41: 48 8b 82 e8 00 00 00        	movq	0xe8(%rdx), %rax
1407eda48: 48 8d 0c 5b                 	leaq	(%rbx,%rbx,2), %rcx
1407eda4c: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407eda50: f3 0f 6f 85 10 02 00 00     	movdqu	0x210(%rbp), %xmm0
1407eda58: f3 0f 6f 8d 20 02 00 00     	movdqu	0x220(%rbp), %xmm1
1407eda60: f3 0f 6f 95 30 02 00 00     	movdqu	0x230(%rbp), %xmm2
1407eda68: f3 0f 7f 54 08 20           	movdqu	%xmm2, 0x20(%rax,%rcx)
1407eda6e: f3 0f 7f 4c 08 10           	movdqu	%xmm1, 0x10(%rax,%rcx)
1407eda74: f3 0f 7f 04 08              	movdqu	%xmm0, (%rax,%rcx)
1407eda79: 48 ff c3                    	incq	%rbx
1407eda7c: 48 89 9a f0 00 00 00        	movq	%rbx, 0xf0(%rdx)
1407eda83: 80 bd 48 01 00 00 00        	cmpb	$0x0, 0x148(%rbp)
1407eda8a: 0f 84 51 01 00 00           	je	0x1407edbe1 <.text+0x7ecbe1>
1407eda90: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1407eda97: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407eda9e: e8 bd ab 4a 00              	callq	0x140c98660 <.text+0xc97660>
1407edaa3: f3 44 0f 10 85 98 01 00 00  	movss	0x198(%rbp), %xmm8
1407edaac: f3 44 0f 58 05 7b f5 1d 02  	addss	0x21df57b(%rip), %xmm8  # 0x1429cd030
1407edab5: f3 44 0f 11 85 98 01 00 00  	movss	%xmm8, 0x198(%rbp)
1407edabe: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407edac5: 48 8d 95 50 01 00 00        	leaq	0x150(%rbp), %rdx
1407edacc: e8 8f ab 4a 00              	callq	0x140c98660 <.text+0xc97660>
1407edad1: f3 0f 10 85 18 02 00 00     	movss	0x218(%rbp), %xmm0
1407edad9: f3 0f 10 15 8f 7d 13 02     	movss	0x2137d8f(%rip), %xmm2  # 0x142925870
1407edae1: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407edae5: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407edae8: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407edaec: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
1407edaf0: f3 0f 58 8d 64 02 00 00     	addss	0x264(%rbp), %xmm1
1407edaf8: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1407edafc: 0f 28 d6                    	movaps	%xmm6, %xmm2
1407edaff: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407edb02: 0f 59 15 d7 7d 13 02        	mulps	0x2137dd7(%rip), %xmm2  # 0x1429258e0
1407edb09: 0f 58 55 60                 	addps	0x60(%rbp), %xmm2
1407edb0d: 0f 13 55 f0                 	movlps	%xmm2, -0x10(%rbp)
1407edb11: f3 0f 11 4d f8              	movss	%xmm1, -0x8(%rbp)
1407edb16: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1407edb1d: e8 ae aa 89 ff              	callq	0x1400885d0 <.text+0x875d0>
1407edb22: f3 0f 10 85 9c 01 00 00     	movss	0x19c(%rbp), %xmm0
1407edb2a: f3 0f 58 05 3a 16 1f 02     	addss	0x21f163a(%rip), %xmm0  # 0x1429df16c
1407edb32: f3 44 0f 11 85 c0 01 00 00  	movss	%xmm8, 0x1c0(%rbp)
1407edb3b: f3 0f 11 85 c4 01 00 00     	movss	%xmm0, 0x1c4(%rbp)
1407edb43: c7 85 c8 01 00 00 00 00 00 3f       	movl	$0x3f000000, 0x1c8(%rbp) # imm = 0x3F000000
1407edb4d: f3 0f 11 7c 24 28           	movss	%xmm7, 0x28(%rsp)
1407edb53: f3 0f 11 74 24 20           	movss	%xmm6, 0x20(%rsp)
1407edb59: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407edb60: 89 c2                       	movl	%eax, %edx
1407edb62: 4c 8d 45 f0                 	leaq	-0x10(%rbp), %r8
1407edb66: 4c 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %r9
1407edb6d: e8 6e 8b a9 00              	callq	0x1412866e0 <.text+0x12856e0>
1407edb72: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407edb79: 48 8b 98 f0 00 00 00        	movq	0xf0(%rax), %rbx
1407edb80: 48 3b 98 e0 00 00 00        	cmpq	0xe0(%rax), %rbx
1407edb87: 75 0f                       	jne	0x1407edb98 <.text+0x7ecb98>
1407edb89: 48 89 f1                    	movq	%rsi, %rcx
1407edb8c: 48 8d 15 cd cb 29 02        	leaq	0x229cbcd(%rip), %rdx   # 0x142a8a760
1407edb93: e8 58 da 03 00              	callq	0x14082b5f0 <.text+0x82a5f0>
1407edb98: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
1407edb9f: 48 8b 82 e8 00 00 00        	movq	0xe8(%rdx), %rax
1407edba6: 48 8d 0c 5b                 	leaq	(%rbx,%rbx,2), %rcx
1407edbaa: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407edbae: f3 0f 6f 85 10 02 00 00     	movdqu	0x210(%rbp), %xmm0
1407edbb6: f3 0f 6f 8d 20 02 00 00     	movdqu	0x220(%rbp), %xmm1
1407edbbe: f3 0f 6f 95 30 02 00 00     	movdqu	0x230(%rbp), %xmm2
1407edbc6: f3 0f 7f 54 08 20           	movdqu	%xmm2, 0x20(%rax,%rcx)
1407edbcc: f3 0f 7f 4c 08 10           	movdqu	%xmm1, 0x10(%rax,%rcx)
1407edbd2: f3 0f 7f 04 08              	movdqu	%xmm0, (%rax,%rcx)
1407edbd7: 48 ff c3                    	incq	%rbx
1407edbda: 48 89 9a f0 00 00 00        	movq	%rbx, 0xf0(%rdx)
1407edbe1: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407edbe8: 0f b6 40 fc                 	movzbl	-0x4(%rax), %eax
1407edbec: fe c8                       	decb	%al
1407edbee: 3c 03                       	cmpb	$0x3, %al
1407edbf0: 0f 83 3e 12 00 00           	jae	0x1407eee34 <.text+0x7ede34>
1407edbf6: 0f b6 c0                    	movzbl	%al, %eax
1407edbf9: 48 8d 0d 50 d2 29 02        	leaq	0x229d250(%rip), %rcx   # 0x142a8ae50
1407edc00: 48 63 1c 81                 	movslq	(%rcx,%rax,4), %rbx
1407edc04: 66 0f 6f 85 10 01 00 00     	movdqa	0x110(%rbp), %xmm0
1407edc0c: 66 0f 6f 8d 20 01 00 00     	movdqa	0x120(%rbp), %xmm1
1407edc14: 66 0f 6f 95 30 01 00 00     	movdqa	0x130(%rbp), %xmm2
1407edc1c: 66 0f 7f 95 e0 01 00 00     	movdqa	%xmm2, 0x1e0(%rbp)
1407edc24: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
1407edc2c: 66 0f 7f 85 c0 01 00 00     	movdqa	%xmm0, 0x1c0(%rbp)
1407edc34: 48 8d 85 13 02 00 00        	leaq	0x213(%rbp), %rax
1407edc3b: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
1407edc42: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1407edc49: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
1407edc50: 48 8b 8d 17 02 00 00        	movq	0x217(%rbp), %rcx
1407edc57: 48 89 8d 7f 01 00 00        	movq	%rcx, 0x17f(%rbp)
1407edc5e: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1407edc65: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407edc6c: 48 8b b0 c0 00 00 00        	movq	0xc0(%rax), %rsi
1407edc73: 48 3b b0 b0 00 00 00        	cmpq	0xb0(%rax), %rsi
1407edc7a: 75 13                       	jne	0x1407edc8f <.text+0x7ecc8f>
1407edc7c: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1407edc83: 48 8d 15 7e cb 29 02        	leaq	0x229cb7e(%rip), %rdx   # 0x142a8a808
1407edc8a: e8 21 d7 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407edc8f: 48 8d 05 ba d1 29 02        	leaq	0x229d1ba(%rip), %rax   # 0x142a8ae50
1407edc96: 48 01 c3                    	addq	%rax, %rbx
1407edc99: 4c 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %r9
1407edca0: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407edca7: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407edcae: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407edcb6: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407edcbf: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407edcc8: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407edcd0: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1407edcd4: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407edcd9: 8b 55 78                    	movl	0x78(%rbp), %edx
1407edcdc: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407edce0: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407edce8: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
1407edcef: 0f 10 8d 20 02 00 00        	movups	0x220(%rbp), %xmm1
1407edcf6: 0f 10 95 30 02 00 00        	movups	0x230(%rbp), %xmm2
1407edcfd: 0f 10 9d 3c 02 00 00        	movups	0x23c(%rbp), %xmm3
1407edd04: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407edd09: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407edd0e: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407edd13: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407edd18: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407edd23: 48 8b 55 7c                 	movq	0x7c(%rbp), %rdx
1407edd27: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407edd2f: 8b 95 84 00 00 00           	movl	0x84(%rbp), %edx
1407edd35: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407edd3c: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407edd47: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1407edd4e: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407edd56: 8b 95 90 00 00 00           	movl	0x90(%rbp), %edx
1407edd5c: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407edd63: 48 89 9c 08 a0 00 00 00     	movq	%rbx, 0xa0(%rax,%rcx)
1407edd6b: 48 c7 84 08 a8 00 00 00 2e 00 00 00 	movq	$0x2e, 0xa8(%rax,%rcx)
1407edd77: 48 8d 15 82 c8 29 02        	leaq	0x229c882(%rip), %rdx   # 0x142a8a600
1407edd7e: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407edd86: 48 c7 84 08 b8 00 00 00 1e 00 00 00 	movq	$0x1e, 0xb8(%rax,%rcx)
1407edd92: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1407edd99: 0f 28 8d d0 01 00 00        	movaps	0x1d0(%rbp), %xmm1
1407edda0: 0f 28 95 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm2
1407edda7: 0f 29 94 08 e0 00 00 00     	movaps	%xmm2, 0xe0(%rax,%rcx)
1407eddaf: 0f 29 8c 08 d0 00 00 00     	movaps	%xmm1, 0xd0(%rax,%rcx)
1407eddb7: 0f 29 84 08 c0 00 00 00     	movaps	%xmm0, 0xc0(%rax,%rcx)
1407eddbf: c7 84 08 f0 00 00 00 01 00 00 00    	movl	$0x1, 0xf0(%rax,%rcx)
1407eddca: 8b 95 90 01 00 00           	movl	0x190(%rbp), %edx
1407eddd0: 89 94 08 f4 00 00 00        	movl	%edx, 0xf4(%rax,%rcx)
1407eddd7: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407edde2: 48 8b 95 a8 00 00 00        	movq	0xa8(%rbp), %rdx
1407edde9: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407eddf1: 0f 10 85 98 00 00 00        	movups	0x98(%rbp), %xmm0
1407eddf8: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407ede00: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407ede08: 0f b6 95 ff 00 00 00        	movzbl	0xff(%rbp), %edx
1407ede0f: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407ede16: 0f b7 95 fd 00 00 00        	movzwl	0xfd(%rbp), %edx
1407ede1d: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407ede25: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407ede30: 0f 10 85 98 01 00 00        	movups	0x198(%rbp), %xmm0
1407ede37: 0f 10 8d a8 01 00 00        	movups	0x1a8(%rbp), %xmm1
1407ede3e: 0f 11 84 08 1c 01 00 00     	movups	%xmm0, 0x11c(%rax,%rcx)
1407ede46: 0f 11 8c 08 2c 01 00 00     	movups	%xmm1, 0x12c(%rax,%rcx)
1407ede4e: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407ede56: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407ede5d: 4c 8b 85 7f 01 00 00        	movq	0x17f(%rbp), %r8
1407ede64: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407ede6c: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407ede74: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407ede7c: 0f b6 95 f8 01 00 00        	movzbl	0x1f8(%rbp), %edx
1407ede83: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407ede8a: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407ede94: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1407ede9b: 4c 8b 85 07 01 00 00        	movq	0x107(%rbp), %r8
1407edea2: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407edeaa: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407edeb2: 48 ff c6                    	incq	%rsi
1407edeb5: 49 89 b1 c0 00 00 00        	movq	%rsi, 0xc0(%r9)
1407edebc: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407edec3: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
1407edec7: 0f 82 fe 03 00 00           	jb	0x1407ee2cb <.text+0x7ed2cb>
1407edecd: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407eded4: 0f b6 70 fc                 	movzbl	-0x4(%rax), %esi
1407eded8: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1407ededf: 0f 28 8d 20 01 00 00        	movaps	0x120(%rbp), %xmm1
1407edee6: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1407edeed: 0f 29 95 30 02 00 00        	movaps	%xmm2, 0x230(%rbp)
1407edef4: 0f 29 8d 20 02 00 00        	movaps	%xmm1, 0x220(%rbp)
1407edefb: 0f 29 85 10 02 00 00        	movaps	%xmm0, 0x210(%rbp)
1407edf02: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1407edf06: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407edf0b: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407edf12: 89 f2                       	movl	%esi, %edx
1407edf14: 44 0f b6 85 80 02 00 00     	movzbl	0x280(%rbp), %r8d
1407edf1c: 4c 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %r9
1407edf23: e8 88 18 00 00              	callq	0x1407ef7b0 <.text+0x7ee7b0>
1407edf28: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407edf2f: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407edf36: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1407edf3d: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1407edf44: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1407edf4b: 48 85 c0                    	testq	%rax, %rax
1407edf4e: 0f 84 40 03 00 00           	je	0x1407ee294 <.text+0x7ed294>
1407edf54: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407edf58: 48 c1 e0 04                 	shlq	$0x4, %rax
1407edf5c: 48 8b 9d 70 02 00 00        	movq	0x270(%rbp), %rbx
1407edf63: 48 01 d8                    	addq	%rbx, %rax
1407edf66: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1407edf6d: e9 6d 01 00 00              	jmp	0x1407ee0df <.text+0x7ed0df>
1407edf72: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1407edf80: 48 83 c3 30                 	addq	$0x30, %rbx
1407edf84: 44 0f 14 c7                 	unpcklps	%xmm7, %xmm8            # xmm8 = xmm8[0],xmm7[0],xmm8[1],xmm7[1]
1407edf88: 44 0f 58 c6                 	addps	%xmm6, %xmm8
1407edf8c: 48 8d 05 c9 ce 29 02        	leaq	0x229cec9(%rip), %rax   # 0x142a8ae5c
1407edf93: 49 01 c5                    	addq	%rax, %r13
1407edf96: 4c 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %r9
1407edf9d: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407edfa4: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407edfab: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407edfb3: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407edfbc: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407edfc5: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407edfcd: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407edfd5: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407edfe0: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407edfeb: 4c 89 ac 08 a0 00 00 00     	movq	%r13, 0xa0(%rax,%rcx)
1407edff3: 48 c7 84 08 a8 00 00 00 2c 00 00 00 	movq	$0x2c, 0xa8(%rax,%rcx)
1407edfff: 48 8d 15 fe 31 43 02        	leaq	0x24331fe(%rip), %rdx   # 0x142c21204
1407ee006: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407ee00e: 48 c7 84 08 b8 00 00 00 17 00 00 00 	movq	$0x17, 0xb8(%rax,%rcx)
1407ee01a: 44 0f 29 9c 08 c0 00 00 00  	movaps	%xmm11, 0xc0(%rax,%rcx)
1407ee023: 44 0f 13 84 08 d0 00 00 00  	movlps	%xmm8, 0xd0(%rax,%rcx)
1407ee02c: f3 44 0f 11 8c 08 d8 00 00 00       	movss	%xmm9, 0xd8(%rax,%rcx)
1407ee036: 0f 28 85 10 02 00 00        	movaps	0x210(%rbp), %xmm0
1407ee03d: 0f 11 84 08 dc 00 00 00     	movups	%xmm0, 0xdc(%rax,%rcx)
1407ee045: 8b 95 20 02 00 00           	movl	0x220(%rbp), %edx
1407ee04b: 89 94 08 ec 00 00 00        	movl	%edx, 0xec(%rax,%rcx)
1407ee052: c7 84 08 f0 00 00 00 00 00 00 00    	movl	$0x0, 0xf0(%rax,%rcx)
1407ee05d: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407ee068: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407ee070: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407ee07b: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407ee083: 48 8b 95 98 01 00 00        	movq	0x198(%rbp), %rdx
1407ee08a: 4c 8b 85 9f 01 00 00        	movq	0x19f(%rbp), %r8
1407ee091: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407ee099: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407ee0a1: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407ee0a9: 0f b6 95 f8 01 00 00        	movzbl	0x1f8(%rbp), %edx
1407ee0b0: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407ee0b7: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407ee0c1: 48 ff c6                    	incq	%rsi
1407ee0c4: 49 89 b1 c0 00 00 00        	movq	%rsi, 0xc0(%r9)
1407ee0cb: 48 3b 9d 68 02 00 00        	cmpq	0x268(%rbp), %rbx
1407ee0d2: 4c 8b ad b0 00 00 00        	movq	0xb0(%rbp), %r13
1407ee0d9: 0f 84 b5 01 00 00           	je	0x1407ee294 <.text+0x7ed294>
1407ee0df: 44 0f 28 1b                 	movaps	(%rbx), %xmm11
1407ee0e3: f3 0f 10 7b 14              	movss	0x14(%rbx), %xmm7
1407ee0e8: f3 0f 58 3d c8 8b 22 02     	addss	0x2228bc8(%rip), %xmm7  # 0x142a16cb8
1407ee0f0: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
1407ee0f4: 41 0f c6 cb d2              	shufps	$0xd2, %xmm11, %xmm1    # xmm1 = xmm1[2,0],xmm11[1,3]
1407ee0f9: 0f 59 0d 10 be 29 02        	mulps	0x229be10(%rip), %xmm1  # 0x142a89f10
1407ee100: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1407ee104: 41 0f c6 c3 c9              	shufps	$0xc9, %xmm11, %xmm0    # xmm0 = xmm0[1,2],xmm11[0,3]
1407ee109: 41 0f 59 c7                 	mulps	%xmm15, %xmm0
1407ee10d: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407ee117: 4c 8d 04 07                 	leaq	(%rdi,%rax), %r8
1407ee11b: 4c 89 c0                    	movq	%r8, %rax
1407ee11e: 48 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rcx # imm = 0xE7037ED1A0B428DB
1407ee128: 48 31 c8                    	xorq	%rcx, %rax
1407ee12b: 49 f7 e0                    	mulq	%r8
1407ee12e: 48 89 d1                    	movq	%rdx, %rcx
1407ee131: 48 31 c1                    	xorq	%rax, %rcx
1407ee134: 48 89 c8                    	movq	%rcx, %rax
1407ee137: ba 03 00 00 00              	movl	$0x3, %edx
1407ee13c: 48 f7 e2                    	mulq	%rdx
1407ee13f: 48 85 c9                    	testq	%rcx, %rcx
1407ee142: 75 4b                       	jne	0x1407ee18f <.text+0x7ed18f>
1407ee144: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407ee14e: 48 01 c7                    	addq	%rax, %rdi
1407ee151: 48 b9 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rcx # imm = 0xA0761D6478BD642F
1407ee15b: 49 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %r9 # imm = 0xE7037ED1A0B428DB
1407ee165: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407ee170: 49 01 c8                    	addq	%rcx, %r8
1407ee173: 48 89 f8                    	movq	%rdi, %rax
1407ee176: 4c 31 c8                    	xorq	%r9, %rax
1407ee179: 48 f7 e7                    	mulq	%rdi
1407ee17c: 48 01 cf                    	addq	%rcx, %rdi
1407ee17f: 48 31 c2                    	xorq	%rax, %rdx
1407ee182: 74 ec                       	je	0x1407ee170 <.text+0x7ed170>
1407ee184: 48 89 d0                    	movq	%rdx, %rax
1407ee187: b9 03 00 00 00              	movl	$0x3, %ecx
1407ee18c: 48 f7 e1                    	mulq	%rcx
1407ee18f: 4c 89 c7                    	movq	%r8, %rdi
1407ee192: 48 8d 43 1c                 	leaq	0x1c(%rbx), %rax
1407ee196: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
1407ee19a: 41 0f 59 d3                 	mulps	%xmm11, %xmm2
1407ee19e: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
1407ee1a2: f3 0f 58 da                 	addss	%xmm2, %xmm3
1407ee1a6: 0f 28 e2                    	movaps	%xmm2, %xmm4
1407ee1a9: 66 0f 15 e2                 	unpckhpd	%xmm2, %xmm4            # xmm4 = xmm4[1],xmm2[1]
1407ee1ad: f3 0f 58 e3                 	addss	%xmm3, %xmm4
1407ee1b1: 0f c6 e4 00                 	shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
1407ee1b5: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
1407ee1b9: 0f 5c d4                    	subps	%xmm4, %xmm2
1407ee1bc: 41 0f 59 d5                 	mulps	%xmm13, %xmm2
1407ee1c0: 41 0f 28 db                 	movaps	%xmm11, %xmm3
1407ee1c4: 41 0f 59 dd                 	mulps	%xmm13, %xmm3
1407ee1c8: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
1407ee1cc: f3 0f 58 e3                 	addss	%xmm3, %xmm4
1407ee1d0: 0f 12 db                    	movhlps	%xmm3, %xmm3            # xmm3 = xmm3[1,1]
1407ee1d3: f3 0f 58 dc                 	addss	%xmm4, %xmm3
1407ee1d7: f3 0f 58 db                 	addss	%xmm3, %xmm3
1407ee1db: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
1407ee1df: 41 0f 59 db                 	mulps	%xmm11, %xmm3
1407ee1e3: 0f 58 da                    	addps	%xmm2, %xmm3
1407ee1e6: 0f 5c c1                    	subps	%xmm1, %xmm0
1407ee1e9: 41 0f 28 f3                 	movaps	%xmm11, %xmm6
1407ee1ed: 41 0f 58 f3                 	addps	%xmm11, %xmm6
1407ee1f1: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1407ee1f5: 0f 59 f0                    	mulps	%xmm0, %xmm6
1407ee1f8: 0f 58 f3                    	addps	%xmm3, %xmm6
1407ee1fb: f3 44 0f 10 43 10           	movss	0x10(%rbx), %xmm8
1407ee201: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
1407ee205: 66 44 0f 15 ce              	unpckhpd	%xmm6, %xmm9            # xmm9 = xmm9[1],xmm6[1]
1407ee20a: f3 44 0f 58 4b 18           	addss	0x18(%rbx), %xmm9
1407ee210: 48 8d 0d 45 cc 29 02        	leaq	0x229cc45(%rip), %rcx   # 0x142a8ae5c
1407ee217: 4c 63 2c 91                 	movslq	(%rcx,%rdx,4), %r13
1407ee21b: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1407ee21f: 66 0f 7f 85 10 02 00 00     	movdqa	%xmm0, 0x210(%rbp)
1407ee227: 8b 40 10                    	movl	0x10(%rax), %eax
1407ee22a: 89 85 20 02 00 00           	movl	%eax, 0x220(%rbp)
1407ee230: 48 8d 85 c3 01 00 00        	leaq	0x1c3(%rbp), %rax
1407ee237: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
1407ee23e: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1407ee245: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407ee24c: 48 8b 8d c7 01 00 00        	movq	0x1c7(%rbp), %rcx
1407ee253: 48 89 8d 9f 01 00 00        	movq	%rcx, 0x19f(%rbp)
1407ee25a: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
1407ee261: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407ee268: 48 8b b0 c0 00 00 00        	movq	0xc0(%rax), %rsi
1407ee26f: 48 3b b0 b0 00 00 00        	cmpq	0xb0(%rax), %rsi
1407ee276: 0f 85 04 fd ff ff           	jne	0x1407edf80 <.text+0x7ecf80>
1407ee27c: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1407ee283: 48 8d 15 36 c6 29 02        	leaq	0x229c636(%rip), %rdx   # 0x142a8a8c0
1407ee28a: e8 21 d1 03 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407ee28f: e9 ec fc ff ff              	jmp	0x1407edf80 <.text+0x7ecf80>
1407ee294: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1407ee29b: 48 85 c0                    	testq	%rax, %rax
1407ee29e: 74 1a                       	je	0x1407ee2ba <.text+0x7ed2ba>
1407ee2a0: 48 c1 e0 04                 	shlq	$0x4, %rax
1407ee2a4: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407ee2a8: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407ee2ae: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1407ee2b5: e8 66 59 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407ee2ba: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ee2c1: 83 78 b8 01                 	cmpl	$0x1, -0x48(%rax)
1407ee2c5: 0f 87 5f e1 ff ff           	ja	0x1407ec42a <.text+0x7eb42a>
1407ee2cb: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ee2d2: 8b 48 e4                    	movl	-0x1c(%rax), %ecx
1407ee2d5: e8 66 7f 4a 00              	callq	0x140c96240 <.text+0xc95240>
1407ee2da: 48 89 c6                    	movq	%rax, %rsi
1407ee2dd: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ee2e4: 69 40 e4 b6 aa aa aa        	imull	$0xaaaaaab6, -0x1c(%rax), %eax # imm = 0xAAAAAAB6
1407ee2eb: 05 4b 88 56 55              	addl	$0x5556884b, %eax       # imm = 0x5556884B
1407ee2f0: 3d 55 55 55 55              	cmpl	$0x55555555, %eax       # imm = 0x55555555
1407ee2f5: 0f 87 aa 00 00 00           	ja	0x1407ee3a5 <.text+0x7ed3a5>
1407ee2fb: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ee302: 80 78 fc 03                 	cmpb	$0x3, -0x4(%rax)
1407ee306: 0f 83 99 00 00 00           	jae	0x1407ee3a5 <.text+0x7ed3a5>
1407ee30c: 48 8b bd f0 01 00 00        	movq	0x1f0(%rbp), %rdi
1407ee313: 0f b6 05 ef 68 e4 02        	movzbl	0x2e468ef(%rip), %eax   # 0x143634c09
1407ee31a: b9 50 00 00 00              	movl	$0x50, %ecx
1407ee31f: ba 10 00 00 00              	movl	$0x10, %edx
1407ee324: e8 e7 58 e2 ff              	callq	0x140613c10 <.text+0x612c10>
1407ee329: 48 85 c0                    	testq	%rax, %rax
1407ee32c: 0f 84 1c 0b 00 00           	je	0x1407eee4e <.text+0x7ede4e>
1407ee332: 48 89 c6                    	movq	%rax, %rsi
1407ee335: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1407ee33c: 0f 28 8d 20 01 00 00        	movaps	0x120(%rbp), %xmm1
1407ee343: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1407ee34a: 0f 29 50 20                 	movaps	%xmm2, 0x20(%rax)
1407ee34e: 0f 29 48 10                 	movaps	%xmm1, 0x10(%rax)
1407ee352: 0f 29 00                    	movaps	%xmm0, (%rax)
1407ee355: 48 89 78 30                 	movq	%rdi, 0x30(%rax)
1407ee359: f3 0f 10 85 90 02 00 00     	movss	0x290(%rbp), %xmm0
1407ee361: f3 0f 11 40 38              	movss	%xmm0, 0x38(%rax)
1407ee366: f3 44 0f 11 50 3c           	movss	%xmm10, 0x3c(%rax)
1407ee36c: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1407ee373: 89 46 40                    	movl	%eax, 0x40(%rsi)
1407ee376: 48 8b bd a8 02 00 00        	movq	0x2a8(%rbp), %rdi
1407ee37d: 48 8b 8f 10 01 00 00        	movq	0x110(%rdi), %rcx
1407ee384: 48 85 c9                    	testq	%rcx, %rcx
1407ee387: 74 10                       	je	0x1407ee399 <.text+0x7ed399>
1407ee389: ba 50 00 00 00              	movl	$0x50, %edx
1407ee38e: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407ee394: e8 87 58 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407ee399: 48 89 b7 10 01 00 00        	movq	%rsi, 0x110(%rdi)
1407ee3a0: e9 29 03 00 00              	jmp	0x1407ee6ce <.text+0x7ed6ce>
1407ee3a5: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407ee3af: 48 01 c6                    	addq	%rax, %rsi
1407ee3b2: 48 89 f0                    	movq	%rsi, %rax
1407ee3b5: 48 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rcx # imm = 0xE7037ED1A0B428DB
1407ee3bf: 48 31 c8                    	xorq	%rcx, %rax
1407ee3c2: 48 f7 e6                    	mulq	%rsi
1407ee3c5: 31 d0                       	xorl	%edx, %eax
1407ee3c7: c1 e8 1e                    	shrl	$0x1e, %eax
1407ee3ca: 83 f8 01                    	cmpl	$0x1, %eax
1407ee3cd: 0f 85 fb 02 00 00           	jne	0x1407ee6ce <.text+0x7ed6ce>
1407ee3d3: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ee3da: 0f b6 50 fc                 	movzbl	-0x4(%rax), %edx
1407ee3de: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1407ee3e5: 0f 28 8d 20 01 00 00        	movaps	0x120(%rbp), %xmm1
1407ee3ec: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1407ee3f3: 0f 29 95 30 02 00 00        	movaps	%xmm2, 0x230(%rbp)
1407ee3fa: 0f 29 8d 20 02 00 00        	movaps	%xmm1, 0x220(%rbp)
1407ee401: 0f 29 85 10 02 00 00        	movaps	%xmm0, 0x210(%rbp)
1407ee408: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1407ee40c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407ee411: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407ee418: 44 0f b6 85 80 02 00 00     	movzbl	0x280(%rbp), %r8d
1407ee420: 4c 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %r9
1407ee427: e8 84 13 00 00              	callq	0x1407ef7b0 <.text+0x7ee7b0>
1407ee42c: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
1407ee433: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1407ee43a: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1407ee441: 48 85 c0                    	testq	%rax, %rax
1407ee444: 0f 84 57 02 00 00           	je	0x1407ee6a1 <.text+0x7ed6a1>
1407ee44a: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407ee44e: 48 c1 e0 04                 	shlq	$0x4, %rax
1407ee452: 48 8b 9d 70 02 00 00        	movq	0x270(%rbp), %rbx
1407ee459: 48 01 d8                    	addq	%rbx, %rax
1407ee45c: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1407ee463: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407ee46a: 48 05 f8 00 00 00           	addq	$0xf8, %rax
1407ee470: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1407ee477: 31 ff                       	xorl	%edi, %edi
1407ee479: e9 89 00 00 00              	jmp	0x1407ee507 <.text+0x7ed507>
1407ee47e: 66 90                       	nop
1407ee480: 48 83 c3 30                 	addq	$0x30, %rbx
1407ee484: 48 ff c7                    	incq	%rdi
1407ee487: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
1407ee48e: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
1407ee495: 4a 8d 0c ad 00 00 00 00     	leaq	(,%r13,4), %rcx
1407ee49d: 4c 01 e9                    	addq	%r13, %rcx
1407ee4a0: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407ee4a4: c6 04 08 01                 	movb	$0x1, (%rax,%rcx)
1407ee4a8: 4c 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %r8
1407ee4af: 4c 89 44 08 08              	movq	%r8, 0x8(%rax,%rcx)
1407ee4b4: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1407ee4bb: 0f 28 8d d0 01 00 00        	movaps	0x1d0(%rbp), %xmm1
1407ee4c2: 0f 28 95 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm2
1407ee4c9: 0f 29 44 08 10              	movaps	%xmm0, 0x10(%rax,%rcx)
1407ee4ce: 0f 29 4c 08 20              	movaps	%xmm1, 0x20(%rax,%rcx)
1407ee4d3: 0f 29 54 08 30              	movaps	%xmm2, 0x30(%rax,%rcx)
1407ee4d8: 4c 8b 85 80 02 00 00        	movq	0x280(%rbp), %r8
1407ee4df: 4c 89 44 08 40              	movq	%r8, 0x40(%rax,%rcx)
1407ee4e4: 4c 8b 85 68 02 00 00        	movq	0x268(%rbp), %r8
1407ee4eb: 4c 89 44 08 48              	movq	%r8, 0x48(%rax,%rcx)
1407ee4f0: 49 ff c5                    	incq	%r13
1407ee4f3: 4c 89 aa 08 01 00 00        	movq	%r13, 0x108(%rdx)
1407ee4fa: 48 3b 9d 00 02 00 00        	cmpq	0x200(%rbp), %rbx
1407ee501: 0f 84 9a 01 00 00           	je	0x1407ee6a1 <.text+0x7ed6a1>
1407ee507: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
1407ee50b: 66 0f 6f 4b 10              	movdqa	0x10(%rbx), %xmm1
1407ee510: 66 0f 6f 53 20              	movdqa	0x20(%rbx), %xmm2
1407ee515: 66 0f 7f 95 30 02 00 00     	movdqa	%xmm2, 0x230(%rbp)
1407ee51d: 66 0f 7f 8d 20 02 00 00     	movdqa	%xmm1, 0x220(%rbp)
1407ee525: 66 0f 7f 85 10 02 00 00     	movdqa	%xmm0, 0x210(%rbp)
1407ee52d: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1407ee534: 89 85 a8 01 00 00           	movl	%eax, 0x1a8(%rbp)
1407ee53a: 48 8d 05 97 c3 29 02        	leaq	0x229c397(%rip), %rax   # 0x142a8a8d8
1407ee541: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
1407ee548: 48 c7 85 a0 01 00 00 0a 00 00 00    	movq	$0xa, 0x1a0(%rbp)
1407ee553: 48 89 bd b0 01 00 00        	movq	%rdi, 0x1b0(%rbp)
1407ee55a: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1407ee561: e8 0a e5 a9 00              	callq	0x14128ca70 <.text+0x128ba70>
1407ee566: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1407ee56d: 48 b8 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rax # imm = 0xA0761D6478BD642F
1407ee577: 4c 8d 04 06                 	leaq	(%rsi,%rax), %r8
1407ee57b: 4c 89 c0                    	movq	%r8, %rax
1407ee57e: 48 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %rcx # imm = 0xE7037ED1A0B428DB
1407ee588: 48 31 c8                    	xorq	%rcx, %rax
1407ee58b: 49 f7 e0                    	mulq	%r8
1407ee58e: 48 89 d1                    	movq	%rdx, %rcx
1407ee591: 48 31 c1                    	xorq	%rax, %rcx
1407ee594: 48 89 c8                    	movq	%rcx, %rax
1407ee597: ba 06 00 00 00              	movl	$0x6, %edx
1407ee59c: 48 f7 e2                    	mulq	%rdx
1407ee59f: 48 01 c9                    	addq	%rcx, %rcx
1407ee5a2: 48 8d 04 49                 	leaq	(%rcx,%rcx,2), %rax
1407ee5a6: 48 83 f8 03                 	cmpq	$0x3, %rax
1407ee5aa: 77 4f                       	ja	0x1407ee5fb <.text+0x7ed5fb>
1407ee5ac: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407ee5b6: 48 01 c6                    	addq	%rax, %rsi
1407ee5b9: 48 b9 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rcx # imm = 0xA0761D6478BD642F
1407ee5c3: 49 b9 db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %r9 # imm = 0xE7037ED1A0B428DB
1407ee5cd: 0f 1f 00                    	nopl	(%rax)
1407ee5d0: 48 89 f0                    	movq	%rsi, %rax
1407ee5d3: 4c 31 c8                    	xorq	%r9, %rax
1407ee5d6: 48 f7 e6                    	mulq	%rsi
1407ee5d9: 49 01 c8                    	addq	%rcx, %r8
1407ee5dc: 48 31 c2                    	xorq	%rax, %rdx
1407ee5df: 48 8d 04 12                 	leaq	(%rdx,%rdx), %rax
1407ee5e3: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407ee5e7: 48 01 ce                    	addq	%rcx, %rsi
1407ee5ea: 48 83 f8 04                 	cmpq	$0x4, %rax
1407ee5ee: 72 e0                       	jb	0x1407ee5d0 <.text+0x7ed5d0>
1407ee5f0: 48 89 d0                    	movq	%rdx, %rax
1407ee5f3: b9 06 00 00 00              	movl	$0x6, %ecx
1407ee5f8: 48 f7 e1                    	mulq	%rcx
1407ee5fb: 4c 89 c6                    	movq	%r8, %rsi
1407ee5fe: 48 89 95 f8 01 00 00        	movq	%rdx, 0x1f8(%rbp)
1407ee605: 66 0f 6f 85 10 02 00 00     	movdqa	0x210(%rbp), %xmm0
1407ee60d: 66 0f 6f 8d 20 02 00 00     	movdqa	0x220(%rbp), %xmm1
1407ee615: 66 0f 6f 95 30 02 00 00     	movdqa	0x230(%rbp), %xmm2
1407ee61d: 66 0f 7f 95 e0 01 00 00     	movdqa	%xmm2, 0x1e0(%rbp)
1407ee625: 66 0f 7f 8d d0 01 00 00     	movdqa	%xmm1, 0x1d0(%rbp)
1407ee62d: 66 0f 7f 85 c0 01 00 00     	movdqa	%xmm0, 0x1c0(%rbp)
1407ee635: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407ee63c: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1407ee643: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407ee64a: 4c 8b a8 08 01 00 00        	movq	0x108(%rax), %r13
1407ee651: 4c 3b a8 f8 00 00 00        	cmpq	0xf8(%rax), %r13
1407ee658: 0f 85 22 fe ff ff           	jne	0x1407ee480 <.text+0x7ed480>
1407ee65e: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1407ee665: 48 8d 15 7c c2 29 02        	leaq	0x229c27c(%rip), %rdx   # 0x142a8a8e8
1407ee66c: e8 8f e3 f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407ee671: e9 0a fe ff ff              	jmp	0x1407ee480 <.text+0x7ed480>
1407ee676: 40 80 fe 03                 	cmpb	$0x3, %sil
1407ee67a: 0f 85 6e 03 00 00           	jne	0x1407ee9ee <.text+0x7ed9ee>
1407ee680: 48 c7 85 48 01 00 00 00 00 00 00    	movq	$0x0, 0x148(%rbp)
1407ee68b: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1407ee692: 83 78 b8 02                 	cmpl	$0x2, -0x48(%rax)
1407ee696: 0f 83 31 f8 ff ff           	jae	0x1407edecd <.text+0x7ececd>
1407ee69c: e9 2a fc ff ff              	jmp	0x1407ee2cb <.text+0x7ed2cb>
1407ee6a1: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
1407ee6a8: 48 85 c0                    	testq	%rax, %rax
1407ee6ab: 74 1a                       	je	0x1407ee6c7 <.text+0x7ed6c7>
1407ee6ad: 48 c1 e0 04                 	shlq	$0x4, %rax
1407ee6b1: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407ee6b5: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407ee6bb: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1407ee6c2: e8 59 55 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407ee6c7: 4c 8b ad b0 00 00 00        	movq	0xb0(%rbp), %r13
1407ee6ce: 80 bd 48 01 00 00 00        	cmpb	$0x0, 0x148(%rbp)
1407ee6d5: 0f 84 4f dd ff ff           	je	0x1407ec42a <.text+0x7eb42a>
1407ee6db: 48 81 85 b8 01 00 00 f8 00 00 00    	addq	$0xf8, 0x1b8(%rbp)
1407ee6e6: f3 0f 10 b5 d0 00 00 00     	movss	0xd0(%rbp), %xmm6
1407ee6ee: f3 0f 59 35 ae 15 29 02     	mulss	0x22915ae(%rip), %xmm6  # 0x142a7fca4
1407ee6f6: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1407ee6fd: 89 85 10 02 00 00           	movl	%eax, 0x210(%rbp)
1407ee703: 48 8d 05 f6 c1 29 02        	leaq	0x229c1f6(%rip), %rax   # 0x142a8a900
1407ee70a: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1407ee711: 48 c7 85 20 02 00 00 0d 00 00 00    	movq	$0xd, 0x220(%rbp)
1407ee71c: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ee723: e8 08 e5 a9 00              	callq	0x14128cc30 <.text+0x128bc30>
1407ee728: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1407ee72f: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ee736: 8b 70 e0                    	movl	-0x20(%rax), %esi
1407ee739: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407ee740: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1407ee747: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407ee74e: 48 8b b8 08 01 00 00        	movq	0x108(%rax), %rdi
1407ee755: 48 3b b8 f8 00 00 00        	cmpq	0xf8(%rax), %rdi
1407ee75c: 75 13                       	jne	0x1407ee771 <.text+0x7ed771>
1407ee75e: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
1407ee765: 48 8d 15 a4 c1 29 02        	leaq	0x229c1a4(%rip), %rdx   # 0x142a8a910
1407ee76c: e8 8f e2 f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407ee771: 0f 28 05 68 aa 29 02        	movaps	0x229aa68(%rip), %xmm0  # 0x142a891e0
1407ee778: f3 0f 10 c6                 	movss	%xmm6, %xmm0            # xmm0 = xmm6[0],xmm0[1,2,3]
1407ee77c: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407ee77f: 0f c6 15 a9 b7 29 02 24     	shufps	$0x24, 0x229b7a9(%rip), %xmm2 # xmm2 = xmm2[0,1],mem[2,0]
                                                                        # 0x142a89f30
1407ee787: 41 0f 28 f4                 	movaps	%xmm12, %xmm6
1407ee78b: 41 0f 59 f4                 	mulps	%xmm12, %xmm6
1407ee78f: f3 0f 16 ce                 	movshdup	%xmm6, %xmm1            # xmm1 = xmm6[1,1,3,3]
1407ee793: f3 0f 58 ce                 	addss	%xmm6, %xmm1
1407ee797: 0f 28 de                    	movaps	%xmm6, %xmm3
1407ee79a: 66 0f 15 de                 	unpckhpd	%xmm6, %xmm3            # xmm3 = xmm3[1],xmm6[1]
1407ee79e: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407ee7a2: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
1407ee7a6: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1407ee7aa: 0f 5c f3                    	subps	%xmm3, %xmm6
1407ee7ad: 0f 59 d6                    	mulps	%xmm6, %xmm2
1407ee7b0: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407ee7b4: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407ee7b7: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
1407ee7bb: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407ee7bf: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407ee7c2: f3 0f 58 cb                 	addss	%xmm3, %xmm1
1407ee7c6: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407ee7ca: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407ee7ce: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1407ee7d2: 0f 58 ca                    	addps	%xmm2, %xmm1
1407ee7d5: 41 0f 28 fc                 	movaps	%xmm12, %xmm7
1407ee7d9: 41 0f c6 fc d2              	shufps	$0xd2, %xmm12, %xmm7    # xmm7 = xmm7[2,0],xmm12[1,3]
1407ee7de: 0f 28 d7                    	movaps	%xmm7, %xmm2
1407ee7e1: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407ee7e4: 0f c6 c0 d2                 	shufps	$0xd2, %xmm0, %xmm0     # xmm0 = xmm0[2,0,1,3]
1407ee7e8: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407ee7ec: 0f 5c d0                    	subps	%xmm0, %xmm2
1407ee7ef: 0f c6 d2 d2                 	shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
1407ee7f3: 45 0f 28 cc                 	movaps	%xmm12, %xmm9
1407ee7f7: 45 0f 58 cc                 	addps	%xmm12, %xmm9
1407ee7fb: 45 0f c6 c9 ff              	shufps	$0xff, %xmm9, %xmm9     # xmm9 = xmm9[3,3,3,3]
1407ee800: 41 0f 59 d1                 	mulps	%xmm9, %xmm2
1407ee804: 0f 58 d1                    	addps	%xmm1, %xmm2
1407ee807: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
1407ee80b: 0f 58 c2                    	addps	%xmm2, %xmm0
1407ee80e: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407ee811: f3 0f 58 95 64 02 00 00     	addss	0x264(%rbp), %xmm2
1407ee819: 48 8b 9d a8 02 00 00        	movq	0x2a8(%rbp), %rbx
1407ee820: 48 8b 83 00 01 00 00        	movq	0x100(%rbx), %rax
1407ee827: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
1407ee82b: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407ee82f: 66 c7 04 08 02 00           	movw	$0x2, (%rax,%rcx)
1407ee835: 89 74 08 04                 	movl	%esi, 0x4(%rax,%rcx)
1407ee839: 44 0f 11 64 08 10           	movups	%xmm12, 0x10(%rax,%rcx)
1407ee83f: 0f 13 44 08 20              	movlps	%xmm0, 0x20(%rax,%rcx)
1407ee844: f3 0f 11 54 08 28           	movss	%xmm2, 0x28(%rax,%rcx)
1407ee84a: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
1407ee851: f3 0f 6f 42 0c              	movdqu	0xc(%rdx), %xmm0
1407ee856: f3 0f 7f 44 08 2c           	movdqu	%xmm0, 0x2c(%rax,%rcx)
1407ee85c: 8b 52 1c                    	movl	0x1c(%rdx), %edx
1407ee85f: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407ee863: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
1407ee86a: 48 89 54 08 40              	movq	%rdx, 0x40(%rax,%rcx)
1407ee86f: 48 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %rdx
1407ee876: 48 89 54 08 48              	movq	%rdx, 0x48(%rax,%rcx)
1407ee87b: 48 ff c7                    	incq	%rdi
1407ee87e: 48 89 bb 08 01 00 00        	movq	%rdi, 0x108(%rbx)
1407ee885: f3 44 0f 10 85 d0 00 00 00  	movss	0xd0(%rbp), %xmm8
1407ee88e: f3 44 0f 59 05 05 8f 29 02  	mulss	0x2298f05(%rip), %xmm8  # 0x142a8779c
1407ee897: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1407ee89e: 89 85 10 02 00 00           	movl	%eax, 0x210(%rbp)
1407ee8a4: 48 8d 05 7d c0 29 02        	leaq	0x229c07d(%rip), %rax   # 0x142a8a928
1407ee8ab: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1407ee8b2: 48 c7 85 20 02 00 00 0c 00 00 00    	movq	$0xc, 0x220(%rbp)
1407ee8bd: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407ee8c4: e8 67 e3 a9 00              	callq	0x14128cc30 <.text+0x128bc30>
1407ee8c9: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
1407ee8d0: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1407ee8d7: 8b 70 e0                    	movl	-0x20(%rax), %esi
1407ee8da: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407ee8e1: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1407ee8e8: 48 8b bb 08 01 00 00        	movq	0x108(%rbx), %rdi
1407ee8ef: 48 3b bb f8 00 00 00        	cmpq	0xf8(%rbx), %rdi
1407ee8f6: 75 13                       	jne	0x1407ee90b <.text+0x7ed90b>
1407ee8f8: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
1407ee8ff: 48 8d 15 32 c0 29 02        	leaq	0x229c032(%rip), %rdx   # 0x142a8a938
1407ee906: e8 f5 e0 f9 ff              	callq	0x14078ca00 <.text+0x78ba00>
1407ee90b: 0f 28 05 ce a8 29 02        	movaps	0x229a8ce(%rip), %xmm0  # 0x142a891e0
1407ee912: f3 41 0f 10 c0              	movss	%xmm8, %xmm0            # xmm0 = xmm8[0],xmm0[1,2,3]
1407ee917: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407ee91a: 0f c6 0d 0e b6 29 02 24     	shufps	$0x24, 0x229b60e(%rip), %xmm1 # xmm1 = xmm1[0,1],mem[2,0]
                                                                        # 0x142a89f30
1407ee922: 0f 59 f1                    	mulps	%xmm1, %xmm6
1407ee925: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407ee929: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407ee92c: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407ee930: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407ee934: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407ee937: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407ee93b: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407ee93f: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407ee943: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1407ee947: 0f 58 ce                    	addps	%xmm6, %xmm1
1407ee94a: 0f 59 f8                    	mulps	%xmm0, %xmm7
1407ee94d: 0f c6 c0 d2                 	shufps	$0xd2, %xmm0, %xmm0     # xmm0 = xmm0[2,0,1,3]
1407ee951: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407ee955: 0f 5c f8                    	subps	%xmm0, %xmm7
1407ee958: 0f c6 ff d2                 	shufps	$0xd2, %xmm7, %xmm7     # xmm7 = xmm7[2,0,1,3]
1407ee95c: 44 0f 59 cf                 	mulps	%xmm7, %xmm9
1407ee960: 44 0f 58 c9                 	addps	%xmm1, %xmm9
1407ee964: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
1407ee968: 41 0f 58 c1                 	addps	%xmm9, %xmm0
1407ee96c: 45 0f 12 c9                 	movhlps	%xmm9, %xmm9            # xmm9 = xmm9[1,1]
1407ee970: f3 0f 10 8d 64 02 00 00     	movss	0x264(%rbp), %xmm1
1407ee978: f3 41 0f 58 c9              	addss	%xmm9, %xmm1
1407ee97d: 4c 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %r8
1407ee984: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
1407ee98b: 48 8d 0c bf                 	leaq	(%rdi,%rdi,4), %rcx
1407ee98f: 48 c1 e1 04                 	shlq	$0x4, %rcx
1407ee993: 66 c7 04 08 02 01           	movw	$0x102, (%rax,%rcx)     # imm = 0x102
1407ee999: 89 74 08 04                 	movl	%esi, 0x4(%rax,%rcx)
1407ee99d: 44 0f 11 64 08 10           	movups	%xmm12, 0x10(%rax,%rcx)
1407ee9a3: 0f 13 44 08 20              	movlps	%xmm0, 0x20(%rax,%rcx)
1407ee9a8: f3 0f 11 4c 08 28           	movss	%xmm1, 0x28(%rax,%rcx)
1407ee9ae: 48 8d 95 20 01 00 00        	leaq	0x120(%rbp), %rdx
1407ee9b5: f3 0f 6f 42 0c              	movdqu	0xc(%rdx), %xmm0
1407ee9ba: f3 0f 7f 44 08 2c           	movdqu	%xmm0, 0x2c(%rax,%rcx)
1407ee9c0: 8b 52 1c                    	movl	0x1c(%rdx), %edx
1407ee9c3: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407ee9c7: 48 8b 95 78 02 00 00        	movq	0x278(%rbp), %rdx
1407ee9ce: 48 89 54 08 40              	movq	%rdx, 0x40(%rax,%rcx)
1407ee9d3: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
1407ee9da: 48 89 54 08 48              	movq	%rdx, 0x48(%rax,%rcx)
1407ee9df: 48 ff c7                    	incq	%rdi
1407ee9e2: 49 89 b8 08 01 00 00        	movq	%rdi, 0x108(%r8)
1407ee9e9: e9 3c da ff ff              	jmp	0x1407ec42a <.text+0x7eb42a>
1407ee9ee: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
1407ee9f2: 72 09                       	jb	0x1407ee9fd <.text+0x7ed9fd>
1407ee9f4: f3 44 0f 10 0d 4b f9 28 02  	movss	0x228f94b(%rip), %xmm9  # 0x142a7e348
1407ee9fd: 40 84 f6                    	testb	%sil, %sil
1407eea00: 0f 84 69 02 00 00           	je	0x1407eec6f <.text+0x7edc6f>
1407eea06: f3 44 0f 10 b5 d0 00 00 00  	movss	0xd0(%rbp), %xmm14
1407eea0f: 48 8d 5e 01                 	leaq	0x1(%rsi), %rbx
1407eea13: 48 c7 85 48 01 00 00 00 00 00 00    	movq	$0x0, 0x148(%rbp)
1407eea1e: e9 ad ec ff ff              	jmp	0x1407ed6d0 <.text+0x7ec6d0>
1407eea23: f3 0f 10 85 90 02 00 00     	movss	0x290(%rbp), %xmm0
1407eea2b: f3 0f 58 05 c5 cc 1f 02     	addss	0x21fccc5(%rip), %xmm0  # 0x1429eb6f8
1407eea33: f3 0f 11 85 90 02 00 00     	movss	%xmm0, 0x290(%rbp)
1407eea3b: e9 97 eb ff ff              	jmp	0x1407ed5d7 <.text+0x7ec5d7>
1407eea40: f3 0f 10 85 90 02 00 00     	movss	0x290(%rbp), %xmm0
1407eea48: f3 0f 58 05 98 f2 1e 02     	addss	0x21ef298(%rip), %xmm0  # 0x1429ddce8
1407eea50: f3 0f 11 85 90 02 00 00     	movss	%xmm0, 0x290(%rbp)
1407eea58: e9 7a eb ff ff              	jmp	0x1407ed5d7 <.text+0x7ec5d7>
1407eea5d: 8b 85 8c 01 00 00           	movl	0x18c(%rbp), %eax
1407eea63: 4c 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %r8
1407eea6a: 4c 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %r9
1407eea71: 4c 89 85 f0 00 00 00        	movq	%r8, 0xf0(%rbp)
1407eea78: 4c 89 8d b8 00 00 00        	movq	%r9, 0xb8(%rbp)
1407eea7f: 4d 85 c9                    	testq	%r9, %r9
1407eea82: 0f 85 c2 d8 ff ff           	jne	0x1407ec34a <.text+0x7eb34a>
1407eea88: 80 bd a7 02 00 00 00        	cmpb	$0x0, 0x2a7(%rbp)
1407eea8f: 48 8b 75 58                 	movq	0x58(%rbp), %rsi
1407eea93: 74 47                       	je	0x1407eeadc <.text+0x7edadc>
1407eea95: e8 96 96 42 00              	callq	0x140c18130 <.text+0xc17130>
1407eea9a: 48 89 c1                    	movq	%rax, %rcx
1407eea9d: 48 8b 00                    	movq	(%rax), %rax
1407eeaa0: 48 83 f8 01                 	cmpq	$0x1, %rax
1407eeaa4: 0f 85 e2 01 00 00           	jne	0x1407eec8c <.text+0x7edc8c>
1407eeaaa: 48 83 c1 08                 	addq	$0x8, %rcx
1407eeaae: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407eeab2: 0f 85 f8 01 00 00           	jne	0x1407eecb0 <.text+0x7edcb0>
1407eeab8: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407eeabf: 48 89 8d a8 02 00 00        	movq	%rcx, 0x2a8(%rbp)
1407eeac6: 48 83 c1 08                 	addq	$0x8, %rcx
1407eeaca: 48 89 f2                    	movq	%rsi, %rdx
1407eeacd: e8 1e 86 42 00              	callq	0x140c170f0 <.text+0xc160f0>
1407eead2: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407eead9: 48 ff 00                    	incq	(%rax)
1407eeadc: 0f 28 b5 c0 02 00 00        	movaps	0x2c0(%rbp), %xmm6
1407eeae3: 0f 28 bd d0 02 00 00        	movaps	0x2d0(%rbp), %xmm7
1407eeaea: 44 0f 28 85 e0 02 00 00     	movaps	0x2e0(%rbp), %xmm8
1407eeaf2: 44 0f 28 8d f0 02 00 00     	movaps	0x2f0(%rbp), %xmm9
1407eeafa: 44 0f 28 95 00 03 00 00     	movaps	0x300(%rbp), %xmm10
1407eeb02: 44 0f 28 9d 10 03 00 00     	movaps	0x310(%rbp), %xmm11
1407eeb0a: 44 0f 28 a5 20 03 00 00     	movaps	0x320(%rbp), %xmm12
1407eeb12: 44 0f 28 ad 30 03 00 00     	movaps	0x330(%rbp), %xmm13
1407eeb1a: 44 0f 28 b5 40 03 00 00     	movaps	0x340(%rbp), %xmm14
1407eeb22: 44 0f 28 bd 50 03 00 00     	movaps	0x350(%rbp), %xmm15
1407eeb2a: 48 81 c4 e8 03 00 00        	addq	$0x3e8, %rsp            # imm = 0x3E8
1407eeb31: 5b                          	popq	%rbx
1407eeb32: 5f                          	popq	%rdi
1407eeb33: 5e                          	popq	%rsi
1407eeb34: 41 5c                       	popq	%r12
1407eeb36: 41 5d                       	popq	%r13
1407eeb38: 41 5e                       	popq	%r14
1407eeb3a: 41 5f                       	popq	%r15
1407eeb3c: 5d                          	popq	%rbp
1407eeb3d: c3                          	retq
1407eeb3e: 48 8d 0d a3 27 43 02        	leaq	0x24327a3(%rip), %rcx   # 0x142c212e8
1407eeb45: e8 c6 a7 0e 02              	callq	0x1428d9310 <.text+0x28d8310>
1407eeb4a: e9 28 03 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eeb4f: 48 8d 0d e3 b5 29 02        	leaq	0x229b5e3(%rip), %rcx   # 0x142a8a139
1407eeb56: 4c 8d 05 1b ba 29 02        	leaq	0x229ba1b(%rip), %r8    # 0x142a8a578
1407eeb5d: ba 16 00 00 00              	movl	$0x16, %edx
1407eeb62: e8 c9 a7 0e 02              	callq	0x1428d9330 <.text+0x28d8330>
1407eeb67: e9 0b 03 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eeb6c: 48 8d 85 98 01 00 00        	leaq	0x198(%rbp), %rax
1407eeb73: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407eeb7a: 48 8d 05 1f ce ff ff        	leaq	-0x31e1(%rip), %rax     # 0x1407eb9a0 <.text+0x7ea9a0>
1407eeb81: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1407eeb88: 48 8d 05 59 b6 29 02        	leaq	0x229b659(%rip), %rax   # 0x142a8a1e8
1407eeb8f: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1407eeb96: 48 c7 85 18 02 00 00 01 00 00 00    	movq	$0x1, 0x218(%rbp)
1407eeba1: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407eebac: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1407eebb3: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1407eebba: 48 c7 85 28 02 00 00 01 00 00 00    	movq	$0x1, 0x228(%rbp)
1407eebc5: 48 8d 15 2c b6 29 02        	leaq	0x229b62c(%rip), %rdx   # 0x142a8a1f8
1407eebcc: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407eebd3: e8 b8 a7 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407eebd8: e9 9a 02 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eebdd: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407eebe4: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407eebeb: 48 8d 05 ae cd ff ff        	leaq	-0x3252(%rip), %rax     # 0x1407eb9a0 <.text+0x7ea9a0>
1407eebf2: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1407eebf9: 48 8d 05 e8 b5 29 02        	leaq	0x229b5e8(%rip), %rax   # 0x142a8a1e8
1407eec00: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1407eec07: 48 c7 85 18 02 00 00 01 00 00 00    	movq	$0x1, 0x218(%rbp)
1407eec12: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407eec1d: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1407eec24: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1407eec2b: 48 c7 85 28 02 00 00 01 00 00 00    	movq	$0x1, 0x228(%rbp)
1407eec36: c6 85 b7 02 00 00 01        	movb	$0x1, 0x2b7(%rbp)
1407eec3d: 48 8d 15 cc b5 29 02        	leaq	0x229b5cc(%rip), %rdx   # 0x142a8a210
1407eec44: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407eec4b: e8 40 a7 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407eec50: e9 22 02 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eec55: 48 85 c0                    	testq	%rax, %rax
1407eec58: 75 4a                       	jne	0x1407eeca4 <.text+0x7edca4>
1407eec5a: 31 d2                       	xorl	%edx, %edx
1407eec5c: e8 1f 39 10 02              	callq	0x1428f2580 <.text+0x28f1580>
1407eec61: 48 89 c1                    	movq	%rax, %rcx
1407eec64: 48 85 c0                    	testq	%rax, %rax
1407eec67: 0f 85 d1 d5 ff ff           	jne	0x1407ec23e <.text+0x7eb23e>
1407eec6d: eb 35                       	jmp	0x1407eeca4 <.text+0x7edca4>
1407eec6f: 48 8d 0d 16 c5 43 02        	leaq	0x243c516(%rip), %rcx   # 0x142c2b18c
1407eec76: 4c 8d 05 5b c5 43 02        	leaq	0x243c55b(%rip), %r8    # 0x142c2b1d8
1407eec7d: ba 23 00 00 00              	movl	$0x23, %edx
1407eec82: e8 a9 a7 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407eec87: e9 eb 01 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eec8c: 48 85 c0                    	testq	%rax, %rax
1407eec8f: 75 13                       	jne	0x1407eeca4 <.text+0x7edca4>
1407eec91: 31 d2                       	xorl	%edx, %edx
1407eec93: e8 e8 38 10 02              	callq	0x1428f2580 <.text+0x28f1580>
1407eec98: 48 89 c1                    	movq	%rax, %rcx
1407eec9b: 48 85 c0                    	testq	%rax, %rax
1407eec9e: 0f 85 0a fe ff ff           	jne	0x1407eeaae <.text+0x7edaae>
1407eeca4: 48 8d 0d 1d 27 43 02        	leaq	0x243271d(%rip), %rcx   # 0x142c213c8
1407eecab: e8 20 94 0e 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407eecb0: 48 8d 0d 81 27 43 02        	leaq	0x2432781(%rip), %rcx   # 0x142c21438
1407eecb7: e8 94 a5 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407eecbc: 48 8d 05 65 2a 2a 02        	leaq	0x22a2a65(%rip), %rax   # 0x142a91728
1407eecc3: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1407eecca: 48 c7 85 18 02 00 00 01 00 00 00    	movq	$0x1, 0x218(%rbp)
1407eecd5: 48 c7 85 20 02 00 00 08 00 00 00    	movq	$0x8, 0x220(%rbp)
1407eece0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407eece3: 0f 11 85 28 02 00 00        	movups	%xmm0, 0x228(%rbp)
1407eecea: 48 8d 15 47 2a 2a 02        	leaq	0x22a2a47(%rip), %rdx   # 0x142a91738
1407eecf1: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407eecf8: e8 93 a6 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407eecfd: e9 75 01 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eed02: 48 8d 85 98 01 00 00        	leaq	0x198(%rbp), %rax
1407eed09: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407eed10: 48 8d 05 d9 e1 a6 00        	leaq	0xa6e1d9(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
1407eed17: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1407eed1e: 48 8d 05 5b 2f 43 02        	leaq	0x2432f5b(%rip), %rax   # 0x142c21c80
1407eed25: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1407eed2c: 48 c7 85 18 02 00 00 01 00 00 00    	movq	$0x1, 0x218(%rbp)
1407eed37: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407eed42: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1407eed49: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1407eed50: 48 c7 85 28 02 00 00 01 00 00 00    	movq	$0x1, 0x228(%rbp)
1407eed5b: 48 8d 15 46 2f 43 02        	leaq	0x2432f46(%rip), %rdx   # 0x142c21ca8
1407eed62: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407eed69: e8 22 a6 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407eed6e: e9 04 01 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eed73: 48 8d 0d 5e b9 29 02        	leaq	0x229b95e(%rip), %rcx   # 0x142a8a6d8
1407eed7a: 4c 8d 05 47 bc 29 02        	leaq	0x229bc47(%rip), %r8    # 0x142a8a9c8
1407eed81: ba 28 00 00 00              	movl	$0x28, %edx
1407eed86: e8 a5 a6 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407eed8b: e9 e7 00 00 00              	jmp	0x1407eee77 <.text+0x7ede77>
1407eed90: 48 8d 85 8f 02 00 00        	leaq	0x28f(%rbp), %rax
1407eed97: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407eed9e: 48 8d 05 9b cc ff ff        	leaq	-0x3365(%rip), %rax     # 0x1407eba40 <.text+0x7eaa40>
1407eeda5: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1407eedac: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1407eedb3: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407eedba: 48 8d 05 df d0 ff ff        	leaq	-0x2f21(%rip), %rax     # 0x1407ebea0 <.text+0x7eaea0>
1407eedc1: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
1407eedc8: 48 8d 05 c1 bb 29 02        	leaq	0x229bbc1(%rip), %rax   # 0x142a8a990
1407eedcf: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1407eedd6: 48 c7 85 18 02 00 00 02 00 00 00    	movq	$0x2, 0x218(%rbp)
1407eede1: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407eedec: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1407eedf3: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1407eedfa: 48 c7 85 28 02 00 00 02 00 00 00    	movq	$0x2, 0x228(%rbp)
1407eee05: 48 8d 15 a4 bb 29 02        	leaq	0x229bba4(%rip), %rdx   # 0x142a8a9b0
1407eee0c: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1407eee13: e8 78 a5 0e 02              	callq	0x1428d9390 <.text+0x28d8390>
1407eee18: eb 5d                       	jmp	0x1407eee77 <.text+0x7ede77>
1407eee1a: 48 8d 0d b7 b8 29 02        	leaq	0x229b8b7(%rip), %rcx   # 0x142a8a6d8
1407eee21: 4c 8d 05 28 bb 29 02        	leaq	0x229bb28(%rip), %r8    # 0x142a8a950
1407eee28: ba 28 00 00 00              	movl	$0x28, %edx
1407eee2d: e8 fe a5 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407eee32: eb 43                       	jmp	0x1407eee77 <.text+0x7ede77>
1407eee34: 48 8d 0d 9d b8 29 02        	leaq	0x229b89d(%rip), %rcx   # 0x142a8a6d8
1407eee3b: 4c 8d 05 de b9 29 02        	leaq	0x229b9de(%rip), %r8    # 0x142a8a820
1407eee42: ba 28 00 00 00              	movl	$0x28, %edx
1407eee47: e8 e4 a5 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407eee4c: eb 29                       	jmp	0x1407eee77 <.text+0x7ede77>
1407eee4e: b9 10 00 00 00              	movl	$0x10, %ecx
1407eee53: ba 50 00 00 00              	movl	$0x50, %edx
1407eee58: e8 a3 a1 0e 02              	callq	0x1428d9000 <.text+0x28d8000>
1407eee5d: eb 18                       	jmp	0x1407eee77 <.text+0x7ede77>
1407eee5f: 48 8d 0d 72 b8 29 02        	leaq	0x229b872(%rip), %rcx   # 0x142a8a6d8
1407eee66: 4c 8d 05 93 b8 29 02        	leaq	0x229b893(%rip), %r8    # 0x142a8a700
1407eee6d: ba 28 00 00 00              	movl	$0x28, %edx
1407eee72: e8 b9 a5 0e 02              	callq	0x1428d9430 <.text+0x28d8430>
1407eee77: 0f 0b                       	ud2
1407eee79: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407eee80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407eee85: 55                          	pushq	%rbp
1407eee86: 41 57                       	pushq	%r15
1407eee88: 41 56                       	pushq	%r14
1407eee8a: 41 55                       	pushq	%r13
1407eee8c: 41 54                       	pushq	%r12
1407eee8e: 56                          	pushq	%rsi
1407eee8f: 57                          	pushq	%rdi
1407eee90: 53                          	pushq	%rbx
1407eee91: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407eee98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407eee9f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407eeea5: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407eeeab: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407eeeb4: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407eeebd: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407eeec6: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407eeecf: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407eeed8: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407eeee1: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407eeee9: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407eeef1: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
1407eeef8: 48 ff 00                    	incq	(%rax)
1407eeefb: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407eef03: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407eef0b: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407eef14: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407eef1d: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407eef26: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407eef2f: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407eef38: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407eef41: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407eef47: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407eef4d: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407eef54: 5b                          	popq	%rbx
1407eef55: 5f                          	popq	%rdi
1407eef56: 5e                          	popq	%rsi
1407eef57: 41 5c                       	popq	%r12
1407eef59: 41 5d                       	popq	%r13
1407eef5b: 41 5e                       	popq	%r14
1407eef5d: 41 5f                       	popq	%r15
1407eef5f: 5d                          	popq	%rbp
1407eef60: c3                          	retq
1407eef61: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407eef70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407eef75: 55                          	pushq	%rbp
1407eef76: 41 57                       	pushq	%r15
1407eef78: 41 56                       	pushq	%r14
1407eef7a: 41 55                       	pushq	%r13
1407eef7c: 41 54                       	pushq	%r12
1407eef7e: 56                          	pushq	%rsi
1407eef7f: 57                          	pushq	%rdi
1407eef80: 53                          	pushq	%rbx
1407eef81: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407eef88: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407eef8f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407eef95: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407eef9b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407eefa4: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407eefad: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407eefb6: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407eefbf: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407eefc8: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407eefd1: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407eefd9: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407eefe1: 80 bd a7 02 00 00 00        	cmpb	$0x0, 0x2a7(%rbp)
1407eefe8: 74 3a                       	je	0x1407ef024 <.text+0x7ee024>
1407eefea: e8 41 91 42 00              	callq	0x140c18130 <.text+0xc17130>
1407eefef: 48 89 c6                    	movq	%rax, %rsi
1407eeff2: 48 8b 00                    	movq	(%rax), %rax
1407eeff5: 48 83 f8 01                 	cmpq	$0x1, %rax
1407eeff9: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1407eeffd: 0f 85 87 00 00 00           	jne	0x1407ef08a <.text+0x7ee08a>
1407ef003: 48 83 c6 08                 	addq	$0x8, %rsi
1407ef007: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
1407ef00b: 0f 85 a6 00 00 00           	jne	0x1407ef0b7 <.text+0x7ee0b7>
1407ef011: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
1407ef018: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
1407ef01c: e8 cf 80 42 00              	callq	0x140c170f0 <.text+0xc160f0>
1407ef021: 48 ff 06                    	incq	(%rsi)
1407ef024: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef02c: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef034: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef03d: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef046: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef04f: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef058: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef061: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef06a: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef070: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef076: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef07d: 5b                          	popq	%rbx
1407ef07e: 5f                          	popq	%rdi
1407ef07f: 5e                          	popq	%rsi
1407ef080: 41 5c                       	popq	%r12
1407ef082: 41 5d                       	popq	%r13
1407ef084: 41 5e                       	popq	%r14
1407ef086: 41 5f                       	popq	%r15
1407ef088: 5d                          	popq	%rbp
1407ef089: c3                          	retq
1407ef08a: 48 85 c0                    	testq	%rax, %rax
1407ef08d: 75 1c                       	jne	0x1407ef0ab <.text+0x7ee0ab>
1407ef08f: 48 89 f1                    	movq	%rsi, %rcx
1407ef092: 48 89 d6                    	movq	%rdx, %rsi
1407ef095: 31 d2                       	xorl	%edx, %edx
1407ef097: e8 e4 34 10 02              	callq	0x1428f2580 <.text+0x28f1580>
1407ef09c: 48 89 f2                    	movq	%rsi, %rdx
1407ef09f: 48 89 c6                    	movq	%rax, %rsi
1407ef0a2: 48 85 c0                    	testq	%rax, %rax
1407ef0a5: 0f 85 5c ff ff ff           	jne	0x1407ef007 <.text+0x7ee007>
1407ef0ab: 48 8d 0d 16 23 43 02        	leaq	0x2432316(%rip), %rcx   # 0x142c213c8
1407ef0b2: e8 19 90 0e 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407ef0b7: 48 8d 0d 7a 23 43 02        	leaq	0x243237a(%rip), %rcx   # 0x142c21438
1407ef0be: e8 8d a1 0e 02              	callq	0x1428d9250 <.text+0x28d8250>
1407ef0c3: cc                          	int3
1407ef0c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407ef0d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407ef0d5: 55                          	pushq	%rbp
1407ef0d6: 41 57                       	pushq	%r15
1407ef0d8: 41 56                       	pushq	%r14
1407ef0da: 41 55                       	pushq	%r13
1407ef0dc: 41 54                       	pushq	%r12
1407ef0de: 56                          	pushq	%rsi
1407ef0df: 57                          	pushq	%rdi
1407ef0e0: 53                          	pushq	%rbx
1407ef0e1: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407ef0e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407ef0ef: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407ef0f5: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407ef0fb: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407ef104: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407ef10d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407ef116: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407ef11f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407ef128: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407ef131: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407ef139: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407ef141: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1407ef148: 48 ff 00                    	incq	(%rax)
1407ef14b: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef153: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef15b: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef164: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef16d: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef176: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef17f: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef188: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef191: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef197: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef19d: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef1a4: 5b                          	popq	%rbx
1407ef1a5: 5f                          	popq	%rdi
1407ef1a6: 5e                          	popq	%rsi
1407ef1a7: 41 5c                       	popq	%r12
1407ef1a9: 41 5d                       	popq	%r13
1407ef1ab: 41 5e                       	popq	%r14
1407ef1ad: 41 5f                       	popq	%r15
1407ef1af: 5d                          	popq	%rbp
1407ef1b0: c3                          	retq
1407ef1b1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407ef1c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407ef1c5: 55                          	pushq	%rbp
1407ef1c6: 41 57                       	pushq	%r15
1407ef1c8: 41 56                       	pushq	%r14
1407ef1ca: 41 55                       	pushq	%r13
1407ef1cc: 41 54                       	pushq	%r12
1407ef1ce: 56                          	pushq	%rsi
1407ef1cf: 57                          	pushq	%rdi
1407ef1d0: 53                          	pushq	%rbx
1407ef1d1: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407ef1d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407ef1df: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407ef1e5: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407ef1eb: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407ef1f4: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407ef1fd: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407ef206: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407ef20f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407ef218: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407ef221: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407ef229: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407ef231: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1407ef238: 48 85 c0                    	testq	%rax, %rax
1407ef23b: 74 1a                       	je	0x1407ef257 <.text+0x7ee257>
1407ef23d: 48 c1 e0 04                 	shlq	$0x4, %rax
1407ef241: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407ef245: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407ef24b: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1407ef252: e8 c9 49 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407ef257: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef25f: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef267: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef270: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef279: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef282: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef28b: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef294: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef29d: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef2a3: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef2a9: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef2b0: 5b                          	popq	%rbx
1407ef2b1: 5f                          	popq	%rdi
1407ef2b2: 5e                          	popq	%rsi
1407ef2b3: 41 5c                       	popq	%r12
1407ef2b5: 41 5d                       	popq	%r13
1407ef2b7: 41 5e                       	popq	%r14
1407ef2b9: 41 5f                       	popq	%r15
1407ef2bb: 5d                          	popq	%rbp
1407ef2bc: c3                          	retq
1407ef2bd: 0f 1f 00                    	nopl	(%rax)
1407ef2c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407ef2c5: 55                          	pushq	%rbp
1407ef2c6: 41 57                       	pushq	%r15
1407ef2c8: 41 56                       	pushq	%r14
1407ef2ca: 41 55                       	pushq	%r13
1407ef2cc: 41 54                       	pushq	%r12
1407ef2ce: 56                          	pushq	%rsi
1407ef2cf: 57                          	pushq	%rdi
1407ef2d0: 53                          	pushq	%rbx
1407ef2d1: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407ef2d8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407ef2df: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407ef2e5: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407ef2eb: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407ef2f4: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407ef2fd: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407ef306: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407ef30f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407ef318: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407ef321: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407ef329: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407ef331: 48 83 bd 90 01 00 00 00     	cmpq	$0x0, 0x190(%rbp)
1407ef339: 74 21                       	je	0x1407ef35c <.text+0x7ee35c>
1407ef33b: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1407ef342: 48 c1 e0 04                 	shlq	$0x4, %rax
1407ef346: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407ef34a: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407ef350: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1407ef357: e8 c4 48 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407ef35c: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef364: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef36c: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef375: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef37e: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef387: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef390: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef399: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef3a2: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef3a8: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef3ae: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef3b5: 5b                          	popq	%rbx
1407ef3b6: 5f                          	popq	%rdi
1407ef3b7: 5e                          	popq	%rsi
1407ef3b8: 41 5c                       	popq	%r12
1407ef3ba: 41 5d                       	popq	%r13
1407ef3bc: 41 5e                       	popq	%r14
1407ef3be: 41 5f                       	popq	%r15
1407ef3c0: 5d                          	popq	%rbp
1407ef3c1: c3                          	retq
1407ef3c2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1407ef3d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407ef3d5: 55                          	pushq	%rbp
1407ef3d6: 41 57                       	pushq	%r15
1407ef3d8: 41 56                       	pushq	%r14
1407ef3da: 41 55                       	pushq	%r13
1407ef3dc: 41 54                       	pushq	%r12
1407ef3de: 56                          	pushq	%rsi
1407ef3df: 57                          	pushq	%rdi
1407ef3e0: 53                          	pushq	%rbx
1407ef3e1: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407ef3e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407ef3ef: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407ef3f5: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407ef3fb: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407ef404: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407ef40d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407ef416: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407ef41f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407ef428: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407ef431: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407ef439: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407ef441: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
1407ef448: 48 85 c0                    	testq	%rax, %rax
1407ef44b: 74 1a                       	je	0x1407ef467 <.text+0x7ee467>
1407ef44d: 48 c1 e0 04                 	shlq	$0x4, %rax
1407ef451: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407ef455: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1407ef45b: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1407ef462: e8 b9 47 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407ef467: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef46f: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef477: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef480: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef489: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef492: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef49b: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef4a4: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef4ad: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef4b3: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef4b9: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef4c0: 5b                          	popq	%rbx
1407ef4c1: 5f                          	popq	%rdi
1407ef4c2: 5e                          	popq	%rsi
1407ef4c3: 41 5c                       	popq	%r12
1407ef4c5: 41 5d                       	popq	%r13
1407ef4c7: 41 5e                       	popq	%r14
1407ef4c9: 41 5f                       	popq	%r15
1407ef4cb: 5d                          	popq	%rbp
1407ef4cc: c3                          	retq
1407ef4cd: 0f 1f 00                    	nopl	(%rax)
1407ef4d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407ef4d5: 55                          	pushq	%rbp
1407ef4d6: 41 57                       	pushq	%r15
1407ef4d8: 41 56                       	pushq	%r14
1407ef4da: 41 55                       	pushq	%r13
1407ef4dc: 41 54                       	pushq	%r12
1407ef4de: 56                          	pushq	%rsi
1407ef4df: 57                          	pushq	%rdi
1407ef4e0: 53                          	pushq	%rbx
1407ef4e1: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407ef4e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407ef4ef: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407ef4f5: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407ef4fb: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407ef504: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407ef50d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407ef516: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407ef51f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407ef528: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407ef531: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407ef539: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407ef541: 0f b6 9d a6 02 00 00        	movzbl	0x2a6(%rbp), %ebx
1407ef548: f6 c3 01                    	testb	$0x1, %bl
1407ef54b: 0f 84 85 00 00 00           	je	0x1407ef5d6 <.text+0x7ee5d6>
1407ef551: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1407ef558: f0                          	lock
1407ef559: 48 ff 08                    	decq	(%rax)
1407ef55c: 75 0c                       	jne	0x1407ef56a <.text+0x7ee56a>
1407ef55e: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407ef565: e8 b6 d0 36 00              	callq	0x140b5c620 <.text+0xb5b620>
1407ef56a: 88 9d b7 02 00 00           	movb	%bl, 0x2b7(%rbp)
1407ef570: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef578: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef580: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef589: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef592: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef59b: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef5a4: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef5ad: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef5b6: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef5bc: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef5c2: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef5c9: 5b                          	popq	%rbx
1407ef5ca: 5f                          	popq	%rdi
1407ef5cb: 5e                          	popq	%rsi
1407ef5cc: 41 5c                       	popq	%r12
1407ef5ce: 41 5d                       	popq	%r13
1407ef5d0: 41 5e                       	popq	%r14
1407ef5d2: 41 5f                       	popq	%r15
1407ef5d4: 5d                          	popq	%rbp
1407ef5d5: c3                          	retq
1407ef5d6: 88 9d b7 02 00 00           	movb	%bl, 0x2b7(%rbp)
1407ef5dc: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef5e4: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef5ec: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef5f5: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef5fe: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef607: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef610: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef619: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef622: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef628: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef62e: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef635: 5b                          	popq	%rbx
1407ef636: 5f                          	popq	%rdi
1407ef637: 5e                          	popq	%rsi
1407ef638: 41 5c                       	popq	%r12
1407ef63a: 41 5d                       	popq	%r13
1407ef63c: 41 5e                       	popq	%r14
1407ef63e: 41 5f                       	popq	%r15
1407ef640: 5d                          	popq	%rbp
1407ef641: c3                          	retq
1407ef642: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1407ef650: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407ef655: 55                          	pushq	%rbp
1407ef656: 41 57                       	pushq	%r15
1407ef658: 41 56                       	pushq	%r14
1407ef65a: 41 55                       	pushq	%r13
1407ef65c: 41 54                       	pushq	%r12
1407ef65e: 56                          	pushq	%rsi
1407ef65f: 57                          	pushq	%rdi
1407ef660: 53                          	pushq	%rbx
1407ef661: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1407ef668: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407ef66f: 44 0f 29 7c 24 60           	movaps	%xmm15, 0x60(%rsp)
1407ef675: 44 0f 29 74 24 70           	movaps	%xmm14, 0x70(%rsp)
1407ef67b: 44 0f 29 ac 24 80 00 00 00  	movaps	%xmm13, 0x80(%rsp)
1407ef684: 44 0f 29 a4 24 90 00 00 00  	movaps	%xmm12, 0x90(%rsp)
1407ef68d: 44 0f 29 9c 24 a0 00 00 00  	movaps	%xmm11, 0xa0(%rsp)
1407ef696: 44 0f 29 94 24 b0 00 00 00  	movaps	%xmm10, 0xb0(%rsp)
1407ef69f: 44 0f 29 8c 24 c0 00 00 00  	movaps	%xmm9, 0xc0(%rsp)
1407ef6a8: 44 0f 29 84 24 d0 00 00 00  	movaps	%xmm8, 0xd0(%rsp)
1407ef6b1: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
1407ef6b9: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
1407ef6c1: f6 85 b7 02 00 00 01        	testb	$0x1, 0x2b7(%rbp)
1407ef6c8: 74 7f                       	je	0x1407ef749 <.text+0x7ee749>
1407ef6ca: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1407ef6d1: f0                          	lock
1407ef6d2: 48 ff 08                    	decq	(%rax)
1407ef6d5: 75 0c                       	jne	0x1407ef6e3 <.text+0x7ee6e3>
1407ef6d7: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1407ef6de: e8 3d cf 36 00              	callq	0x140b5c620 <.text+0xb5b620>
1407ef6e3: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef6eb: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef6f3: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef6fc: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef705: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef70e: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef717: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef720: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef729: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef72f: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef735: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef73c: 5b                          	popq	%rbx
1407ef73d: 5f                          	popq	%rdi
1407ef73e: 5e                          	popq	%rsi
1407ef73f: 41 5c                       	popq	%r12
1407ef741: 41 5d                       	popq	%r13
1407ef743: 41 5e                       	popq	%r14
1407ef745: 41 5f                       	popq	%r15
1407ef747: 5d                          	popq	%rbp
1407ef748: c3                          	retq
1407ef749: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
1407ef751: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
1407ef759: 44 0f 28 84 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm8
1407ef762: 44 0f 28 8c 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm9
1407ef76b: 44 0f 28 94 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm10
1407ef774: 44 0f 28 9c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm11
1407ef77d: 44 0f 28 a4 24 90 00 00 00  	movaps	0x90(%rsp), %xmm12
1407ef786: 44 0f 28 ac 24 80 00 00 00  	movaps	0x80(%rsp), %xmm13
1407ef78f: 44 0f 28 74 24 70           	movaps	0x70(%rsp), %xmm14
1407ef795: 44 0f 28 7c 24 60           	movaps	0x60(%rsp), %xmm15
1407ef79b: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1407ef7a2: 5b                          	popq	%rbx
1407ef7a3: 5f                          	popq	%rdi
1407ef7a4: 5e                          	popq	%rsi
1407ef7a5: 41 5c                       	popq	%r12
1407ef7a7: 41 5d                       	popq	%r13
1407ef7a9: 41 5e                       	popq	%r14
1407ef7ab: 41 5f                       	popq	%r15
1407ef7ad: 5d                          	popq	%rbp
1407ef7ae: c3                          	retq
