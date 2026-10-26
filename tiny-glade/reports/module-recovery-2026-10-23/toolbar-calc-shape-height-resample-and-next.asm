
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14146afe0: 55                          	pushq	%rbp
14146afe1: 41 57                       	pushq	%r15
14146afe3: 41 56                       	pushq	%r14
14146afe5: 41 55                       	pushq	%r13
14146afe7: 41 54                       	pushq	%r12
14146afe9: 56                          	pushq	%rsi
14146afea: 57                          	pushq	%rdi
14146afeb: 53                          	pushq	%rbx
14146afec: 48 81 ec 98 01 00 00        	subq	$0x198, %rsp            # imm = 0x198
14146aff3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14146affb: 44 0f 29 bd 00 01 00 00     	movaps	%xmm15, 0x100(%rbp)
14146b003: 44 0f 29 b5 f0 00 00 00     	movaps	%xmm14, 0xf0(%rbp)
14146b00b: 44 0f 29 ad e0 00 00 00     	movaps	%xmm13, 0xe0(%rbp)
14146b013: 44 0f 29 a5 d0 00 00 00     	movaps	%xmm12, 0xd0(%rbp)
14146b01b: 44 0f 29 9d c0 00 00 00     	movaps	%xmm11, 0xc0(%rbp)
14146b023: 44 0f 29 95 b0 00 00 00     	movaps	%xmm10, 0xb0(%rbp)
14146b02b: 44 0f 29 8d a0 00 00 00     	movaps	%xmm9, 0xa0(%rbp)
14146b033: 44 0f 29 85 90 00 00 00     	movaps	%xmm8, 0x90(%rbp)
14146b03b: 0f 29 bd 80 00 00 00        	movaps	%xmm7, 0x80(%rbp)
14146b042: 0f 29 75 70                 	movaps	%xmm6, 0x70(%rbp)
14146b046: 48 c7 45 68 fe ff ff ff     	movq	$-0x2, 0x68(%rbp)
14146b04e: 48 8b 7a 10                 	movq	0x10(%rdx), %rdi
14146b052: 48 83 ff 01                 	cmpq	$0x1, %rdi
14146b056: 0f 86 d7 03 00 00           	jbe	0x14146b433 <.text+0x146a433>
14146b05c: 0f 28 f2                    	movaps	%xmm2, %xmm6
14146b05f: 0f 57 c0                    	xorps	%xmm0, %xmm0
14146b062: 0f 2e d0                    	ucomiss	%xmm0, %xmm2
14146b065: 75 06                       	jne	0x14146b06d <.text+0x146a06d>
14146b067: 0f 8b de 03 00 00           	jnp	0x14146b44b <.text+0x146a44b>
14146b06d: 48 89 d6                    	movq	%rdx, %rsi
14146b070: f3 0f 10 4a 30              	movss	0x30(%rdx), %xmm1
14146b075: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
14146b078: 75 06                       	jne	0x14146b080 <.text+0x146a080>
14146b07a: 0f 8b e3 03 00 00           	jnp	0x14146b463 <.text+0x146a463>
14146b080: 48 8d 04 bd 00 00 00 00     	leaq	(,%rdi,4), %rax
14146b088: 4c 8d 3c 40                 	leaq	(%rax,%rax,2), %r15
14146b08c: 48 b8 aa aa aa aa aa aa aa 0a       	movabsq	$0xaaaaaaaaaaaaaaa, %rax # imm = 0xAAAAAAAAAAAAAAA
14146b096: 48 39 c7                    	cmpq	%rax, %rdi
14146b099: 0f 87 dc 03 00 00           	ja	0x14146b47b <.text+0x146a47b>
14146b09f: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
14146b0a3: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
14146b0aa: 49 8b 19                    	movq	(%r9), %rbx
14146b0ad: 4c 8b 28                    	movq	(%rax), %r13
14146b0b0: 0f b6 05 52 9b 1c 02        	movzbl	0x21c9b52(%rip), %eax   # 0x143634c09
14146b0b7: 41 bc 04 00 00 00           	movl	$0x4, %r12d
14146b0bd: ba 04 00 00 00              	movl	$0x4, %edx
14146b0c2: 4c 89 f9                    	movq	%r15, %rcx
14146b0c5: e8 46 8b 1a ff              	callq	0x140613c10 <.text+0x612c10>
14146b0ca: 48 85 c0                    	testq	%rax, %rax
14146b0cd: 0f 84 ab 03 00 00           	je	0x14146b47e <.text+0x146a47e>
14146b0d3: 49 89 c6                    	movq	%rax, %r14
14146b0d6: 48 89 7d 40                 	movq	%rdi, 0x40(%rbp)
14146b0da: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14146b0de: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
14146b0e6: 48 89 75 28                 	movq	%rsi, 0x28(%rbp)
14146b0ea: 48 8b 76 08                 	movq	0x8(%rsi), %rsi
14146b0ee: f3 44 0f 10 06              	movss	(%rsi), %xmm8
14146b0f3: f3 0f 10 7e 04              	movss	0x4(%rsi), %xmm7
14146b0f8: c6 45 67 01                 	movb	$0x1, 0x67(%rbp)
14146b0fc: 48 89 d9                    	movq	%rbx, %rcx
14146b0ff: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14146b103: 0f 28 d7                    	movaps	%xmm7, %xmm2
14146b106: e8 35 e5 57 ff              	callq	0x1409e9640 <.text+0x9e8640>
14146b10b: f3 41 0f 10 4d 3c           	movss	0x3c(%r13), %xmm1
14146b111: 0f 28 d0                    	movaps	%xmm0, %xmm2
14146b114: f3 0f c2 d0 03              	cmpunordss	%xmm0, %xmm2
14146b119: 0f 28 da                    	movaps	%xmm2, %xmm3
14146b11c: 0f 54 d9                    	andps	%xmm1, %xmm3
14146b11f: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14146b123: 0f 55 d1                    	andnps	%xmm1, %xmm2
14146b126: 0f 56 d3                    	orps	%xmm3, %xmm2
14146b129: f3 45 0f 11 06              	movss	%xmm8, (%r14)
14146b12e: f3 41 0f 11 56 04           	movss	%xmm2, 0x4(%r14)
14146b134: f3 41 0f 11 7e 08           	movss	%xmm7, 0x8(%r14)
14146b13a: 48 c7 45 50 01 00 00 00     	movq	$0x1, 0x50(%rbp)
14146b142: 48 8d 04 fe                 	leaq	(%rsi,%rdi,8), %rax
14146b146: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14146b14a: b0 01                       	movb	$0x1, %al
14146b14c: f3 44 0f 10 1d af a7 4b 01  	movss	0x14ba7af(%rip), %xmm11 # 0x142925904
14146b155: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
14146b159: f3 44 0f 10 35 7e ef 4e 01  	movss	0x14eef7e(%rip), %xmm14 # 0x14295a0e0
14146b162: f3 44 0f 10 3d 79 ef 4e 01  	movss	0x14eef79(%rip), %xmm15 # 0x14295a0e4
14146b16b: eb 0c                       	jmp	0x14146b179 <.text+0x146a179>
14146b16d: 0f 1f 00                    	nopl	(%rax)
14146b170: 31 c0                       	xorl	%eax, %eax
14146b172: 4c 89 e1                    	movq	%r12, %rcx
14146b175: 48 8b 75 30                 	movq	0x30(%rbp), %rsi
14146b179: 48 3b 75 38                 	cmpq	0x38(%rbp), %rsi
14146b17d: 0f 84 01 02 00 00           	je	0x14146b384 <.text+0x146a384>
14146b183: 4c 8d 66 08                 	leaq	0x8(%rsi), %r12
14146b187: a8 01                       	testb	$0x1, %al
14146b189: 74 15                       	je	0x14146b1a0 <.text+0x146a1a0>
14146b18b: 4c 3b 65 38                 	cmpq	0x38(%rbp), %r12
14146b18f: 0f 84 ef 01 00 00           	je	0x14146b384 <.text+0x146a384>
14146b195: 48 8d 46 10                 	leaq	0x10(%rsi), %rax
14146b199: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14146b19d: eb 0b                       	jmp	0x14146b1aa <.text+0x146a1aa>
14146b19f: 90                          	nop
14146b1a0: 4c 89 65 30                 	movq	%r12, 0x30(%rbp)
14146b1a4: 49 89 f4                    	movq	%rsi, %r12
14146b1a7: 48 89 ce                    	movq	%rcx, %rsi
14146b1aa: f3 41 0f 10 04 24           	movss	(%r12), %xmm0
14146b1b0: f3 41 0f 10 4c 24 04        	movss	0x4(%r12), %xmm1
14146b1b7: f3 0f 5c 06                 	subss	(%rsi), %xmm0
14146b1bb: f3 0f 5c 4e 04              	subss	0x4(%rsi), %xmm1
14146b1c0: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14146b1c4: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
14146b1c8: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14146b1cc: 0f 57 c0                    	xorps	%xmm0, %xmm0
14146b1cf: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
14146b1d3: f3 0f 5f 05 fd ee 4e 01     	maxss	0x14eeefd(%rip), %xmm0  # 0x14295a0d8
14146b1db: 0f 28 ce                    	movaps	%xmm6, %xmm1
14146b1de: f3 0f 5e c8                 	divss	%xmm0, %xmm1
14146b1e2: f3 41 0f 5d cb              	minss	%xmm11, %xmm1
14146b1e7: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
14146b1eb: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14146b1ef: e8 ac 8d 4b 01              	callq	0x142923fa0 <.text+0x2922fa0>
14146b1f4: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
14146b1f9: 48 89 c1                    	movq	%rax, %rcx
14146b1fc: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
14146b200: 0f 28 c8                    	movaps	%xmm0, %xmm1
14146b203: f3 0f 5c 0d d1 ee 4e 01     	subss	0x14eeed1(%rip), %xmm1  # 0x14295a0dc
14146b20b: f3 48 0f 2c f9              	cvttss2si	%xmm1, %rdi
14146b210: 48 21 cf                    	andq	%rcx, %rdi
14146b213: 48 09 c7                    	orq	%rax, %rdi
14146b216: 41 0f 2e c5                 	ucomiss	%xmm13, %xmm0
14146b21a: b8 00 00 00 00              	movl	$0x0, %eax
14146b21f: 48 0f 42 f8                 	cmovbq	%rax, %rdi
14146b223: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
14146b227: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
14146b22e: 48 0f 47 f8                 	cmovaq	%rax, %rdi
14146b232: 48 ff c7                    	incq	%rdi
14146b235: 48 83 ff 03                 	cmpq	$0x3, %rdi
14146b239: b8 02 00 00 00              	movl	$0x2, %eax
14146b23e: 48 0f 42 f8                 	cmovbq	%rax, %rdi
14146b242: 48 85 ff                    	testq	%rdi, %rdi
14146b245: 78 19                       	js	0x14146b260 <.text+0x146a260>
14146b247: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
14146b24b: f3 4c 0f 2a d7              	cvtsi2ss	%rdi, %xmm10
14146b250: eb 2a                       	jmp	0x14146b27c <.text+0x146a27c>
14146b252: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14146b260: 48 89 f8                    	movq	%rdi, %rax
14146b263: 48 d1 e8                    	shrq	%rax
14146b266: 89 f9                       	movl	%edi, %ecx
14146b268: 83 e1 01                    	andl	$0x1, %ecx
14146b26b: 48 09 c1                    	orq	%rax, %rcx
14146b26e: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
14146b272: f3 4c 0f 2a d1              	cvtsi2ss	%rcx, %xmm10
14146b277: f3 45 0f 58 d2              	addss	%xmm10, %xmm10
14146b27c: f3 45 0f 58 d7              	addss	%xmm15, %xmm10
14146b281: 41 be 01 00 00 00           	movl	$0x1, %r14d
14146b287: eb 48                       	jmp	0x14146b2d1 <.text+0x146a2d1>
14146b289: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14146b290: 49 ff c6                    	incq	%r14
14146b293: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
14146b297: f3 41 0f c2 c1 03           	cmpunordss	%xmm9, %xmm0
14146b29d: 0f 28 c8                    	movaps	%xmm0, %xmm1
14146b2a0: 41 0f 54 cc                 	andps	%xmm12, %xmm1
14146b2a4: f3 45 0f 5f e1              	maxss	%xmm9, %xmm12
14146b2a9: 41 0f 55 c4                 	andnps	%xmm12, %xmm0
14146b2ad: 0f 56 c1                    	orps	%xmm1, %xmm0
14146b2b0: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14146b2b4: 4b 8d 0c 7f                 	leaq	(%r15,%r15,2), %rcx
14146b2b8: f3 44 0f 11 04 88           	movss	%xmm8, (%rax,%rcx,4)
14146b2be: f3 0f 11 44 88 04           	movss	%xmm0, 0x4(%rax,%rcx,4)
14146b2c4: f3 0f 11 7c 88 08           	movss	%xmm7, 0x8(%rax,%rcx,4)
14146b2ca: 49 ff c7                    	incq	%r15
14146b2cd: 4c 89 7d 50                 	movq	%r15, 0x50(%rbp)
14146b2d1: 49 39 fe                    	cmpq	%rdi, %r14
14146b2d4: 0f 83 96 fe ff ff           	jae	0x14146b170 <.text+0x146a170>
14146b2da: 4d 85 f6                    	testq	%r14, %r14
14146b2dd: 78 11                       	js	0x14146b2f0 <.text+0x146a2f0>
14146b2df: 0f 57 ff                    	xorps	%xmm7, %xmm7
14146b2e2: f3 49 0f 2a fe              	cvtsi2ss	%r14, %xmm7
14146b2e7: eb 22                       	jmp	0x14146b30b <.text+0x146a30b>
14146b2e9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14146b2f0: 4c 89 f0                    	movq	%r14, %rax
14146b2f3: 48 d1 e8                    	shrq	%rax
14146b2f6: 44 89 f1                    	movl	%r14d, %ecx
14146b2f9: 83 e1 01                    	andl	$0x1, %ecx
14146b2fc: 48 09 c1                    	orq	%rax, %rcx
14146b2ff: 0f 57 ff                    	xorps	%xmm7, %xmm7
14146b302: f3 48 0f 2a f9              	cvtsi2ss	%rcx, %xmm7
14146b307: f3 0f 58 ff                 	addss	%xmm7, %xmm7
14146b30b: f3 41 0f 5e fa              	divss	%xmm10, %xmm7
14146b310: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
14146b314: f3 0f 5c c7                 	subss	%xmm7, %xmm0
14146b318: f3 0f 10 0e                 	movss	(%rsi), %xmm1
14146b31c: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
14146b320: f3 0f 59 46 04              	mulss	0x4(%rsi), %xmm0
14146b325: f3 45 0f 10 04 24           	movss	(%r12), %xmm8
14146b32b: f3 44 0f 59 c7              	mulss	%xmm7, %xmm8
14146b330: f3 44 0f 58 c1              	addss	%xmm1, %xmm8
14146b335: f3 41 0f 59 7c 24 04        	mulss	0x4(%r12), %xmm7
14146b33c: f3 0f 58 f8                 	addss	%xmm0, %xmm7
14146b340: c6 45 67 01                 	movb	$0x1, 0x67(%rbp)
14146b344: 48 89 d9                    	movq	%rbx, %rcx
14146b347: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14146b34b: 0f 28 d7                    	movaps	%xmm7, %xmm2
14146b34e: e8 ed e2 57 ff              	callq	0x1409e9640 <.text+0x9e8640>
14146b353: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
14146b357: f3 45 0f 10 65 3c           	movss	0x3c(%r13), %xmm12
14146b35d: 4c 8b 7d 50                 	movq	0x50(%rbp), %r15
14146b361: 4c 3b 7d 40                 	cmpq	0x40(%rbp), %r15
14146b365: 0f 85 25 ff ff ff           	jne	0x14146b290 <.text+0x146a290>
14146b36b: c6 45 67 01                 	movb	$0x1, 0x67(%rbp)
14146b36f: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14146b373: 48 8d 15 36 85 7f 01        	leaq	0x17f8536(%rip), %rdx   # 0x142c638b0
14146b37a: e8 01 3e 73 ff              	callq	0x140b9f180 <.text+0xb9e180>
14146b37f: e9 0c ff ff ff              	jmp	0x14146b290 <.text+0x146a290>
14146b384: 48 8b 45 50                 	movq	0x50(%rbp), %rax
14146b388: 48 89 45 00                 	movq	%rax, (%rbp)
14146b38c: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
14146b390: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
14146b394: c6 45 67 00                 	movb	$0x0, 0x67(%rbp)
14146b398: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
14146b39c: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
14146b3a0: e8 8b de ff ff              	callq	0x141469230 <.text+0x1468230>
14146b3a5: 31 c0                       	xorl	%eax, %eax
14146b3a7: 48 3b 45 a8                 	cmpq	-0x58(%rbp), %rax
14146b3ab: 0f 80 df 00 00 00           	jo	0x14146b490 <.text+0x146a490>
14146b3b1: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
14146b3b5: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14146b3b9: 48 89 48 30                 	movq	%rcx, 0x30(%rax)
14146b3bd: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
14146b3c1: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
14146b3c5: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
14146b3c9: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14146b3cd: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14146b3d1: 0f 11 00                    	movups	%xmm0, (%rax)
14146b3d4: 0f 28 75 70                 	movaps	0x70(%rbp), %xmm6
14146b3d8: 0f 28 bd 80 00 00 00        	movaps	0x80(%rbp), %xmm7
14146b3df: 44 0f 28 85 90 00 00 00     	movaps	0x90(%rbp), %xmm8
14146b3e7: 44 0f 28 8d a0 00 00 00     	movaps	0xa0(%rbp), %xmm9
14146b3ef: 44 0f 28 95 b0 00 00 00     	movaps	0xb0(%rbp), %xmm10
14146b3f7: 44 0f 28 9d c0 00 00 00     	movaps	0xc0(%rbp), %xmm11
14146b3ff: 44 0f 28 a5 d0 00 00 00     	movaps	0xd0(%rbp), %xmm12
14146b407: 44 0f 28 ad e0 00 00 00     	movaps	0xe0(%rbp), %xmm13
14146b40f: 44 0f 28 b5 f0 00 00 00     	movaps	0xf0(%rbp), %xmm14
14146b417: 44 0f 28 bd 00 01 00 00     	movaps	0x100(%rbp), %xmm15
14146b41f: 48 81 c4 98 01 00 00        	addq	$0x198, %rsp            # imm = 0x198
14146b426: 5b                          	popq	%rbx
14146b427: 5f                          	popq	%rdi
14146b428: 5e                          	popq	%rsi
14146b429: 41 5c                       	popq	%r12
14146b42b: 41 5d                       	popq	%r13
14146b42d: 41 5e                       	popq	%r14
14146b42f: 41 5f                       	popq	%r15
14146b431: 5d                          	popq	%rbp
14146b432: c3                          	retq
14146b433: 48 8d 0d c6 82 7f 01        	leaq	0x17f82c6(%rip), %rcx   # 0x142c63700
14146b43a: 4c 8d 05 df 83 7f 01        	leaq	0x17f83df(%rip), %r8    # 0x142c63820
14146b441: ba 28 00 00 00              	movl	$0x28, %edx
14146b446: e8 e5 df 46 01              	callq	0x1428d9430 <.text+0x28d8430>
14146b44b: 48 8d 0d e6 83 7f 01        	leaq	0x17f83e6(%rip), %rcx   # 0x142c63838
14146b452: 4c 8d 05 0f 84 7f 01        	leaq	0x17f840f(%rip), %r8    # 0x142c63868
14146b459: ba 2b 00 00 00              	movl	$0x2b, %edx
14146b45e: e8 cd df 46 01              	callq	0x1428d9430 <.text+0x28d8430>
14146b463: 48 8d 0d 16 83 7f 01        	leaq	0x17f8316(%rip), %rcx   # 0x142c63780
14146b46a: 4c 8d 05 0f 84 7f 01        	leaq	0x17f840f(%rip), %r8    # 0x142c63880
14146b471: ba 24 00 00 00              	movl	$0x24, %edx
14146b476: e8 b5 df 46 01              	callq	0x1428d9430 <.text+0x28d8430>
14146b47b: 45 31 e4                    	xorl	%r12d, %r12d
14146b47e: 4c 8d 05 13 84 7f 01        	leaq	0x17f8413(%rip), %r8    # 0x142c63898
14146b485: 4c 89 e1                    	movq	%r12, %rcx
14146b488: 4c 89 fa                    	movq	%r15, %rdx
14146b48b: e8 53 db 46 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14146b490: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
14146b494: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14146b498: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
14146b49c: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14146b4a0: 48 8d 05 29 12 00 00        	leaq	0x1229(%rip), %rax      # 0x14146c6d0 <.text+0x146b6d0>
14146b4a7: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14146b4ab: 48 8b 45 28                 	movq	0x28(%rbp), %rax
14146b4af: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14146b4b3: 48 8d 05 06 c0 e8 ff        	leaq	-0x173ffa(%rip), %rax   # 0x1412f74c0 <.text+0x12f64c0>
14146b4ba: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14146b4be: 48 8d 05 f3 85 7f 01        	leaq	0x17f85f3(%rip), %rax   # 0x142c63ab8
14146b4c5: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14146b4c9: 48 c7 45 f8 02 00 00 00     	movq	$0x2, -0x8(%rbp)
14146b4d1: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
14146b4d9: 48 8d 45 40                 	leaq	0x40(%rbp), %rax
14146b4dd: 48 89 45 00                 	movq	%rax, (%rbp)
14146b4e1: 48 c7 45 08 02 00 00 00     	movq	$0x2, 0x8(%rbp)
14146b4e9: 48 8d 15 e8 85 7f 01        	leaq	0x17f85e8(%rip), %rdx   # 0x142c63ad8
14146b4f0: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
14146b4f4: e8 97 de 46 01              	callq	0x1428d9390 <.text+0x28d8390>
14146b4f9: cc                          	int3
14146b4fa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14146b500: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14146b505: 55                          	pushq	%rbp
14146b506: 41 57                       	pushq	%r15
14146b508: 41 56                       	pushq	%r14
14146b50a: 41 55                       	pushq	%r13
14146b50c: 41 54                       	pushq	%r12
14146b50e: 56                          	pushq	%rsi
14146b50f: 57                          	pushq	%rdi
14146b510: 53                          	pushq	%rbx
14146b511: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14146b518: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14146b51f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14146b525: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14146b52b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14146b531: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14146b537: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14146b53d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14146b543: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14146b54c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14146b555: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14146b55d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14146b565: 80 7d 67 00                 	cmpb	$0x0, 0x67(%rbp)
14146b569: 74 20                       	je	0x14146b58b <.text+0x146a58b>
14146b56b: 48 8b 45 40                 	movq	0x40(%rbp), %rax
14146b56f: 48 85 c0                    	testq	%rax, %rax
14146b572: 74 17                       	je	0x14146b58b <.text+0x146a58b>
14146b574: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14146b578: 48 c1 e0 02                 	shlq	$0x2, %rax
14146b57c: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14146b580: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14146b586: e8 95 86 1a ff              	callq	0x140613c20 <.text+0x612c20>
14146b58b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14146b593: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14146b59b: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14146b5a4: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14146b5ad: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14146b5b3: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14146b5b9: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14146b5bf: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14146b5c5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14146b5cb: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14146b5d1: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14146b5d8: 5b                          	popq	%rbx
14146b5d9: 5f                          	popq	%rdi
14146b5da: 5e                          	popq	%rsi
14146b5db: 41 5c                       	popq	%r12
14146b5dd: 41 5d                       	popq	%r13
14146b5df: 41 5e                       	popq	%r14
14146b5e1: 41 5f                       	popq	%r15
14146b5e3: 5d                          	popq	%rbp
14146b5e4: c3                          	retq
14146b5e5: cc                          	int3
14146b5e6: cc                          	int3
14146b5e7: cc                          	int3
14146b5e8: cc                          	int3
14146b5e9: cc                          	int3
14146b5ea: cc                          	int3
14146b5eb: cc                          	int3
14146b5ec: cc                          	int3
14146b5ed: cc                          	int3
14146b5ee: cc                          	int3
14146b5ef: cc                          	int3
14146b5f0: 55                          	pushq	%rbp
14146b5f1: 41 57                       	pushq	%r15
14146b5f3: 41 56                       	pushq	%r14
14146b5f5: 41 55                       	pushq	%r13
14146b5f7: 41 54                       	pushq	%r12
14146b5f9: 56                          	pushq	%rsi
14146b5fa: 57                          	pushq	%rdi
14146b5fb: 53                          	pushq	%rbx
14146b5fc: 48 81 ec 78 01 00 00        	subq	$0x178, %rsp            # imm = 0x178
14146b603: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14146b60b: 44 0f 29 ad e0 00 00 00     	movaps	%xmm13, 0xe0(%rbp)
14146b613: 44 0f 29 a5 d0 00 00 00     	movaps	%xmm12, 0xd0(%rbp)
14146b61b: 44 0f 29 9d c0 00 00 00     	movaps	%xmm11, 0xc0(%rbp)
14146b623: 44 0f 29 95 b0 00 00 00     	movaps	%xmm10, 0xb0(%rbp)
14146b62b: 44 0f 29 8d a0 00 00 00     	movaps	%xmm9, 0xa0(%rbp)
14146b633: 44 0f 29 85 90 00 00 00     	movaps	%xmm8, 0x90(%rbp)
14146b63b: 0f 29 bd 80 00 00 00        	movaps	%xmm7, 0x80(%rbp)
14146b642: 0f 29 75 70                 	movaps	%xmm6, 0x70(%rbp)
14146b646: 48 c7 45 68 fe ff ff ff     	movq	$-0x2, 0x68(%rbp)
14146b64e: 48 8b 7a 10                 	movq	0x10(%rdx), %rdi
14146b652: 48 83 ff 01                 	cmpq	$0x1, %rdi
14146b656: 0f 86 71 03 00 00           	jbe	0x14146b9cd <.text+0x146a9cd>
14146b65c: 0f 28 f2                    	movaps	%xmm2, %xmm6
14146b65f: 0f 57 c0                    	xorps	%xmm0, %xmm0
14146b662: 0f 2e d0                    	ucomiss	%xmm0, %xmm2
14146b665: 75 06                       	jne	0x14146b66d <.text+0x146a66d>
14146b667: 0f 8b 78 03 00 00           	jnp	0x14146b9e5 <.text+0x146a9e5>
14146b66d: 48 89 d6                    	movq	%rdx, %rsi
14146b670: f3 0f 10 4a 30              	movss	0x30(%rdx), %xmm1
14146b675: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
14146b678: 75 06                       	jne	0x14146b680 <.text+0x146a680>
14146b67a: 0f 8b 7d 03 00 00           	jnp	0x14146b9fd <.text+0x146a9fd>
14146b680: 48 8d 04 bd 00 00 00 00     	leaq	(,%rdi,4), %rax
14146b688: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
14146b68c: 48 b8 aa aa aa aa aa aa aa 0a       	movabsq	$0xaaaaaaaaaaaaaaa, %rax # imm = 0xAAAAAAAAAAAAAAA
14146b696: 48 39 c7                    	cmpq	%rax, %rdi
14146b699: 0f 87 76 03 00 00           	ja	0x14146ba15 <.text+0x146aa15>
14146b69f: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
14146b6a3: 0f b6 05 5f 95 1c 02        	movzbl	0x21c955f(%rip), %eax   # 0x143634c09
14146b6aa: 41 bf 04 00 00 00           	movl	$0x4, %r15d
14146b6b0: ba 04 00 00 00              	movl	$0x4, %edx
14146b6b5: 4c 89 f1                    	movq	%r14, %rcx
14146b6b8: e8 53 85 1a ff              	callq	0x140613c10 <.text+0x612c10>
14146b6bd: 48 85 c0                    	testq	%rax, %rax
14146b6c0: 0f 84 52 03 00 00           	je	0x14146ba18 <.text+0x146aa18>
14146b6c6: 48 89 c3                    	movq	%rax, %rbx
14146b6c9: 48 89 7d 40                 	movq	%rdi, 0x40(%rbp)
14146b6cd: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14146b6d1: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
14146b6d9: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
14146b6dd: 48 8b 76 08                 	movq	0x8(%rsi), %rsi
14146b6e1: f2 0f 10 06                 	movsd	(%rsi), %xmm0
14146b6e5: f2 0f 11 45 00              	movsd	%xmm0, (%rbp)
14146b6ea: c6 45 67 01                 	movb	$0x1, 0x67(%rbp)
14146b6ee: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
14146b6f2: 48 89 ea                    	movq	%rbp, %rdx
14146b6f5: 0f 57 ff                    	xorps	%xmm7, %xmm7
14146b6f8: 0f 57 d2                    	xorps	%xmm2, %xmm2
14146b6fb: e8 e0 16 83 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14146b700: 8b 45 c0                    	movl	-0x40(%rbp), %eax
14146b703: 89 43 08                    	movl	%eax, 0x8(%rbx)
14146b706: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
14146b70a: 48 89 03                    	movq	%rax, (%rbx)
14146b70d: 48 c7 45 50 01 00 00 00     	movq	$0x1, 0x50(%rbp)
14146b715: 48 8d 04 fe                 	leaq	(%rsi,%rdi,8), %rax
14146b719: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14146b71d: b0 01                       	movb	$0x1, %al
14146b71f: f3 44 0f 10 05 b0 e9 4e 01  	movss	0x14ee9b0(%rip), %xmm8  # 0x14295a0d8
14146b728: f3 44 0f 10 0d d3 a1 4b 01  	movss	0x14ba1d3(%rip), %xmm9  # 0x142925904
14146b731: f3 44 0f 10 15 a2 e9 4e 01  	movss	0x14ee9a2(%rip), %xmm10 # 0x14295a0dc
14146b73a: f3 44 0f 10 1d 9d e9 4e 01  	movss	0x14ee99d(%rip), %xmm11 # 0x14295a0e0
14146b743: f3 44 0f 10 25 98 e9 4e 01  	movss	0x14ee998(%rip), %xmm12 # 0x14295a0e4
14146b74c: 4c 8d 75 b8                 	leaq	-0x48(%rbp), %r14
14146b750: 49 89 ef                    	movq	%rbp, %r15
14146b753: eb 14                       	jmp	0x14146b769 <.text+0x146a769>
14146b755: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14146b760: 31 c0                       	xorl	%eax, %eax
14146b762: 4c 89 e9                    	movq	%r13, %rcx
14146b765: 48 8b 75 30                 	movq	0x30(%rbp), %rsi
14146b769: 48 3b 75 38                 	cmpq	0x38(%rbp), %rsi
14146b76d: 0f 84 bc 01 00 00           	je	0x14146b92f <.text+0x146a92f>
14146b773: 4c 8d 6e 08                 	leaq	0x8(%rsi), %r13
14146b777: a8 01                       	testb	$0x1, %al
14146b779: 74 15                       	je	0x14146b790 <.text+0x146a790>
14146b77b: 4c 3b 6d 38                 	cmpq	0x38(%rbp), %r13
14146b77f: 0f 84 aa 01 00 00           	je	0x14146b92f <.text+0x146a92f>
14146b785: 48 8d 46 10                 	leaq	0x10(%rsi), %rax
14146b789: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14146b78d: eb 0b                       	jmp	0x14146b79a <.text+0x146a79a>
14146b78f: 90                          	nop
14146b790: 4c 89 6d 30                 	movq	%r13, 0x30(%rbp)
14146b794: 49 89 f5                    	movq	%rsi, %r13
14146b797: 48 89 ce                    	movq	%rcx, %rsi
14146b79a: f3 41 0f 10 45 00           	movss	(%r13), %xmm0
14146b7a0: f3 41 0f 10 4d 04           	movss	0x4(%r13), %xmm1
14146b7a6: f3 0f 5c 06                 	subss	(%rsi), %xmm0
14146b7aa: f3 0f 5c 4e 04              	subss	0x4(%rsi), %xmm1
14146b7af: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14146b7b3: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
14146b7b7: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14146b7bb: 0f 57 c0                    	xorps	%xmm0, %xmm0
14146b7be: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
14146b7c2: f3 41 0f 5f c0              	maxss	%xmm8, %xmm0
14146b7c7: 0f 28 ce                    	movaps	%xmm6, %xmm1
14146b7ca: f3 0f 5e c8                 	divss	%xmm0, %xmm1
14146b7ce: f3 41 0f 5d c9              	minss	%xmm9, %xmm1
14146b7d3: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
14146b7d7: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14146b7db: e8 c0 87 4b 01              	callq	0x142923fa0 <.text+0x2922fa0>
14146b7e0: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
14146b7e5: 48 89 c1                    	movq	%rax, %rcx
14146b7e8: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
14146b7ec: 0f 28 c8                    	movaps	%xmm0, %xmm1
14146b7ef: f3 41 0f 5c ca              	subss	%xmm10, %xmm1
14146b7f4: f3 48 0f 2c f9              	cvttss2si	%xmm1, %rdi
14146b7f9: 48 21 cf                    	andq	%rcx, %rdi
14146b7fc: 48 09 c7                    	orq	%rax, %rdi
14146b7ff: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
14146b802: b8 00 00 00 00              	movl	$0x0, %eax
14146b807: 48 0f 42 f8                 	cmovbq	%rax, %rdi
14146b80b: 41 0f 2e c3                 	ucomiss	%xmm11, %xmm0
14146b80f: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
14146b816: 48 0f 47 f8                 	cmovaq	%rax, %rdi
14146b81a: 48 ff c7                    	incq	%rdi
14146b81d: 48 83 ff 03                 	cmpq	$0x3, %rdi
14146b821: b8 02 00 00 00              	movl	$0x2, %eax
14146b826: 48 0f 42 f8                 	cmovbq	%rax, %rdi
14146b82a: 48 85 ff                    	testq	%rdi, %rdi
14146b82d: 78 11                       	js	0x14146b840 <.text+0x146a840>
14146b82f: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
14146b833: f3 4c 0f 2a ef              	cvtsi2ss	%rdi, %xmm13
14146b838: eb 22                       	jmp	0x14146b85c <.text+0x146a85c>
14146b83a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14146b840: 48 89 f8                    	movq	%rdi, %rax
14146b843: 48 d1 e8                    	shrq	%rax
14146b846: 89 f9                       	movl	%edi, %ecx
14146b848: 83 e1 01                    	andl	$0x1, %ecx
14146b84b: 48 09 c1                    	orq	%rax, %rcx
14146b84e: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
14146b852: f3 4c 0f 2a e9              	cvtsi2ss	%rcx, %xmm13
14146b857: f3 45 0f 58 ed              	addss	%xmm13, %xmm13
14146b85c: f3 45 0f 58 ec              	addss	%xmm12, %xmm13
14146b861: bb 01 00 00 00              	movl	$0x1, %ebx
14146b866: eb 29                       	jmp	0x14146b891 <.text+0x146a891>
14146b868: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14146b870: 48 ff c3                    	incq	%rbx
14146b873: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14146b877: 4b 8d 0c 64                 	leaq	(%r12,%r12,2), %rcx
14146b87b: 8b 55 c0                    	movl	-0x40(%rbp), %edx
14146b87e: 89 54 88 08                 	movl	%edx, 0x8(%rax,%rcx,4)
14146b882: 48 8b 55 b8                 	movq	-0x48(%rbp), %rdx
14146b886: 48 89 14 88                 	movq	%rdx, (%rax,%rcx,4)
14146b88a: 49 ff c4                    	incq	%r12
14146b88d: 4c 89 65 50                 	movq	%r12, 0x50(%rbp)
14146b891: 48 39 fb                    	cmpq	%rdi, %rbx
14146b894: 0f 83 c6 fe ff ff           	jae	0x14146b760 <.text+0x146a760>
14146b89a: 48 85 db                    	testq	%rbx, %rbx
14146b89d: 78 11                       	js	0x14146b8b0 <.text+0x146a8b0>
14146b89f: 0f 57 c0                    	xorps	%xmm0, %xmm0
14146b8a2: f3 48 0f 2a c3              	cvtsi2ss	%rbx, %xmm0
14146b8a7: eb 21                       	jmp	0x14146b8ca <.text+0x146a8ca>
14146b8a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14146b8b0: 48 89 d8                    	movq	%rbx, %rax
14146b8b3: 48 d1 e8                    	shrq	%rax
14146b8b6: 89 d9                       	movl	%ebx, %ecx
14146b8b8: 83 e1 01                    	andl	$0x1, %ecx
14146b8bb: 48 09 c1                    	orq	%rax, %rcx
14146b8be: 0f 57 c0                    	xorps	%xmm0, %xmm0
14146b8c1: f3 48 0f 2a c1              	cvtsi2ss	%rcx, %xmm0
14146b8c6: f3 0f 58 c0                 	addss	%xmm0, %xmm0
14146b8ca: f3 41 0f 5e c5              	divss	%xmm13, %xmm0
14146b8cf: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
14146b8d3: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14146b8d7: f2 0f 10 16                 	movsd	(%rsi), %xmm2
14146b8db: f2 41 0f 10 5d 00           	movsd	(%r13), %xmm3
14146b8e1: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14146b8e5: 0f 59 ca                    	mulps	%xmm2, %xmm1
14146b8e8: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14146b8ec: 0f 59 c3                    	mulps	%xmm3, %xmm0
14146b8ef: 0f 58 c1                    	addps	%xmm1, %xmm0
14146b8f2: 0f 13 45 00                 	movlps	%xmm0, (%rbp)
14146b8f6: c6 45 67 01                 	movb	$0x1, 0x67(%rbp)
14146b8fa: 0f 57 d2                    	xorps	%xmm2, %xmm2
14146b8fd: 4c 89 f1                    	movq	%r14, %rcx
14146b900: 4c 89 fa                    	movq	%r15, %rdx
14146b903: e8 d8 14 83 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14146b908: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
14146b90c: 4c 3b 65 40                 	cmpq	0x40(%rbp), %r12
14146b910: 0f 85 5a ff ff ff           	jne	0x14146b870 <.text+0x146a870>
14146b916: c6 45 67 01                 	movb	$0x1, 0x67(%rbp)
14146b91a: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14146b91e: 48 8d 15 8b 7f 7f 01        	leaq	0x17f7f8b(%rip), %rdx   # 0x142c638b0
14146b925: e8 56 38 73 ff              	callq	0x140b9f180 <.text+0xb9e180>
14146b92a: e9 41 ff ff ff              	jmp	0x14146b870 <.text+0x146a870>
14146b92f: 48 8b 45 50                 	movq	0x50(%rbp), %rax
14146b933: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14146b937: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
14146b93b: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
14146b93f: c6 45 67 00                 	movb	$0x0, 0x67(%rbp)
14146b943: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
14146b947: 48 89 ea                    	movq	%rbp, %rdx
14146b94a: e8 e1 d8 ff ff              	callq	0x141469230 <.text+0x1468230>
14146b94f: 31 c0                       	xorl	%eax, %eax
14146b951: 48 3b 45 b8                 	cmpq	-0x48(%rbp), %rax
14146b955: 0f 80 cf 00 00 00           	jo	0x14146ba2a <.text+0x146aa2a>
14146b95b: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14146b95f: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14146b963: 48 89 48 30                 	movq	%rcx, 0x30(%rax)
14146b967: 0f 10 45 b8                 	movups	-0x48(%rbp), %xmm0
14146b96b: 0f 10 4d c8                 	movups	-0x38(%rbp), %xmm1
14146b96f: 0f 10 55 d8                 	movups	-0x28(%rbp), %xmm2
14146b973: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14146b977: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14146b97b: 0f 11 00                    	movups	%xmm0, (%rax)
14146b97e: 0f 28 75 70                 	movaps	0x70(%rbp), %xmm6
14146b982: 0f 28 bd 80 00 00 00        	movaps	0x80(%rbp), %xmm7
14146b989: 44 0f 28 85 90 00 00 00     	movaps	0x90(%rbp), %xmm8
14146b991: 44 0f 28 8d a0 00 00 00     	movaps	0xa0(%rbp), %xmm9
14146b999: 44 0f 28 95 b0 00 00 00     	movaps	0xb0(%rbp), %xmm10
14146b9a1: 44 0f 28 9d c0 00 00 00     	movaps	0xc0(%rbp), %xmm11
14146b9a9: 44 0f 28 a5 d0 00 00 00     	movaps	0xd0(%rbp), %xmm12
14146b9b1: 44 0f 28 ad e0 00 00 00     	movaps	0xe0(%rbp), %xmm13
14146b9b9: 48 81 c4 78 01 00 00        	addq	$0x178, %rsp            # imm = 0x178
14146b9c0: 5b                          	popq	%rbx
14146b9c1: 5f                          	popq	%rdi
14146b9c2: 5e                          	popq	%rsi
14146b9c3: 41 5c                       	popq	%r12
14146b9c5: 41 5d                       	popq	%r13
14146b9c7: 41 5e                       	popq	%r14
14146b9c9: 41 5f                       	popq	%r15
14146b9cb: 5d                          	popq	%rbp
14146b9cc: c3                          	retq
14146b9cd: 48 8d 0d 2c 7d 7f 01        	leaq	0x17f7d2c(%rip), %rcx   # 0x142c63700
14146b9d4: 4c 8d 05 45 7e 7f 01        	leaq	0x17f7e45(%rip), %r8    # 0x142c63820
14146b9db: ba 28 00 00 00              	movl	$0x28, %edx
14146b9e0: e8 4b da 46 01              	callq	0x1428d9430 <.text+0x28d8430>
14146b9e5: 48 8d 0d 4c 7e 7f 01        	leaq	0x17f7e4c(%rip), %rcx   # 0x142c63838
14146b9ec: 4c 8d 05 75 7e 7f 01        	leaq	0x17f7e75(%rip), %r8    # 0x142c63868
14146b9f3: ba 2b 00 00 00              	movl	$0x2b, %edx
14146b9f8: e8 33 da 46 01              	callq	0x1428d9430 <.text+0x28d8430>
14146b9fd: 48 8d 0d 7c 7d 7f 01        	leaq	0x17f7d7c(%rip), %rcx   # 0x142c63780
14146ba04: 4c 8d 05 75 7e 7f 01        	leaq	0x17f7e75(%rip), %r8    # 0x142c63880
14146ba0b: ba 24 00 00 00              	movl	$0x24, %edx
14146ba10: e8 1b da 46 01              	callq	0x1428d9430 <.text+0x28d8430>
14146ba15: 45 31 ff                    	xorl	%r15d, %r15d
14146ba18: 4c 8d 05 79 7e 7f 01        	leaq	0x17f7e79(%rip), %r8    # 0x142c63898
14146ba1f: 4c 89 f9                    	movq	%r15, %rcx
14146ba22: 4c 89 f2                    	movq	%r14, %rdx
14146ba25: e8 b9 d5 46 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14146ba2a: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
14146ba2e: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
14146ba32: 48 8d 45 a0                 	leaq	-0x60(%rbp), %rax
14146ba36: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14146ba3a: 48 8d 05 8f 0c 00 00        	leaq	0xc8f(%rip), %rax       # 0x14146c6d0 <.text+0x146b6d0>
14146ba41: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14146ba45: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14146ba49: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14146ba4d: 48 8d 05 6c ba e8 ff        	leaq	-0x174594(%rip), %rax   # 0x1412f74c0 <.text+0x12f64c0>
14146ba54: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14146ba58: 48 8d 05 59 80 7f 01        	leaq	0x17f8059(%rip), %rax   # 0x142c63ab8
14146ba5f: 48 89 45 00                 	movq	%rax, (%rbp)
14146ba63: 48 c7 45 08 02 00 00 00     	movq	$0x2, 0x8(%rbp)
14146ba6b: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
14146ba73: 48 8d 45 40                 	leaq	0x40(%rbp), %rax
14146ba77: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14146ba7b: 48 c7 45 18 02 00 00 00     	movq	$0x2, 0x18(%rbp)
14146ba83: 48 8d 15 4e 80 7f 01        	leaq	0x17f804e(%rip), %rdx   # 0x142c63ad8
14146ba8a: 48 89 e9                    	movq	%rbp, %rcx
14146ba8d: e8 fe d8 46 01              	callq	0x1428d9390 <.text+0x28d8390>
14146ba92: cc                          	int3
14146ba93: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14146baa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14146baa5: 55                          	pushq	%rbp
14146baa6: 41 57                       	pushq	%r15
14146baa8: 41 56                       	pushq	%r14
14146baaa: 41 55                       	pushq	%r13
14146baac: 41 54                       	pushq	%r12
14146baae: 56                          	pushq	%rsi
14146baaf: 57                          	pushq	%rdi
14146bab0: 53                          	pushq	%rbx
14146bab1: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
14146bab8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14146babf: 44 0f 29 6c 24 20           	movaps	%xmm13, 0x20(%rsp)
14146bac5: 44 0f 29 64 24 30           	movaps	%xmm12, 0x30(%rsp)
14146bacb: 44 0f 29 5c 24 40           	movaps	%xmm11, 0x40(%rsp)
14146bad1: 44 0f 29 54 24 50           	movaps	%xmm10, 0x50(%rsp)
14146bad7: 44 0f 29 4c 24 60           	movaps	%xmm9, 0x60(%rsp)
14146badd: 44 0f 29 44 24 70           	movaps	%xmm8, 0x70(%rsp)
14146bae3: 0f 29 bc 24 80 00 00 00     	movaps	%xmm7, 0x80(%rsp)
14146baeb: 0f 29 b4 24 90 00 00 00     	movaps	%xmm6, 0x90(%rsp)
14146baf3: 80 7d 67 00                 	cmpb	$0x0, 0x67(%rbp)
14146baf7: 74 20                       	je	0x14146bb19 <.text+0x146ab19>
14146baf9: 48 8b 45 40                 	movq	0x40(%rbp), %rax
14146bafd: 48 85 c0                    	testq	%rax, %rax
14146bb00: 74 17                       	je	0x14146bb19 <.text+0x146ab19>
14146bb02: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14146bb06: 48 c1 e0 02                 	shlq	$0x2, %rax
14146bb0a: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14146bb0e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14146bb14: e8 07 81 1a ff              	callq	0x140613c20 <.text+0x612c20>
14146bb19: 0f 28 b4 24 90 00 00 00     	movaps	0x90(%rsp), %xmm6
14146bb21: 0f 28 bc 24 80 00 00 00     	movaps	0x80(%rsp), %xmm7
14146bb29: 44 0f 28 44 24 70           	movaps	0x70(%rsp), %xmm8
14146bb2f: 44 0f 28 4c 24 60           	movaps	0x60(%rsp), %xmm9
14146bb35: 44 0f 28 54 24 50           	movaps	0x50(%rsp), %xmm10
14146bb3b: 44 0f 28 5c 24 40           	movaps	0x40(%rsp), %xmm11
14146bb41: 44 0f 28 64 24 30           	movaps	0x30(%rsp), %xmm12
14146bb47: 44 0f 28 6c 24 20           	movaps	0x20(%rsp), %xmm13
14146bb4d: 48 81 c4 a8 00 00 00        	addq	$0xa8, %rsp
14146bb54: 5b                          	popq	%rbx
14146bb55: 5f                          	popq	%rdi
14146bb56: 5e                          	popq	%rsi
14146bb57: 41 5c                       	popq	%r12
14146bb59: 41 5d                       	popq	%r13
14146bb5b: 41 5e                       	popq	%r14
14146bb5d: 41 5f                       	popq	%r15
14146bb5f: 5d                          	popq	%rbp
14146bb60: c3                          	retq
14146bb61: cc                          	int3
14146bb62: cc                          	int3
14146bb63: cc                          	int3
14146bb64: cc                          	int3
14146bb65: cc                          	int3
14146bb66: cc                          	int3
14146bb67: cc                          	int3
14146bb68: cc                          	int3
14146bb69: cc                          	int3
14146bb6a: cc                          	int3
14146bb6b: cc                          	int3
14146bb6c: cc                          	int3
14146bb6d: cc                          	int3
14146bb6e: cc                          	int3
14146bb6f: cc                          	int3
