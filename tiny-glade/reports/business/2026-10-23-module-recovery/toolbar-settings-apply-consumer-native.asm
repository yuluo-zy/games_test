
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1404de8c0: 00 00                       	addb	%al, (%rax)
1404de8c2: c6 00 01                    	movb	$0x1, (%rax)
1404de8c5: 31 c0                       	xorl	%eax, %eax
1404de8c7: b1 01                       	movb	$0x1, %cl
1404de8c9: f0                          	lock
1404de8ca: 0f b0 0d c0 62 15 03        	cmpxchgb	%cl, 0x31562c0(%rip)    # 0x143634b91
1404de8d1: 0f 85 72 04 00 00           	jne	0x1404ded49 <.text+0x4ddd49>
1404de8d7: 48 8b 05 42 db 14 03        	movq	0x314db42(%rip), %rax   # 0x14362c420
1404de8de: 48 8b 00                    	movq	(%rax), %rax
1404de8e1: 48 d1 e0                    	shlq	%rax
1404de8e4: 48 85 c0                    	testq	%rax, %rax
1404de8e7: 0f 85 6d 04 00 00           	jne	0x1404ded5a <.text+0x4ddd5a>
1404de8ed: 31 c0                       	xorl	%eax, %eax
1404de8ef: 0f b6 0d 9c 62 15 03        	movzbl	0x315629c(%rip), %ecx   # 0x143634b92
1404de8f6: 84 c9                       	testb	%cl, %cl
1404de8f8: 0f 85 12 0c 00 00           	jne	0x1404df510 <.text+0x4de510>
1404de8fe: 0f b6 1d 8e 62 15 03        	movzbl	0x315628e(%rip), %ebx   # 0x143634b93
1404de905: c6 05 87 62 15 03 00        	movb	$0x0, 0x3156287(%rip)   # 0x143634b93
1404de90c: 84 c0                       	testb	%al, %al
1404de90e: 75 16                       	jne	0x1404de926 <.text+0x4dd926>
1404de910: 48 8b 05 09 db 14 03        	movq	0x314db09(%rip), %rax   # 0x14362c420
1404de917: 48 8b 00                    	movq	(%rax), %rax
1404de91a: 48 d1 e0                    	shlq	%rax
1404de91d: 48 85 c0                    	testq	%rax, %rax
1404de920: 0f 85 6f 04 00 00           	jne	0x1404ded95 <.text+0x4ddd95>
1404de926: 31 c0                       	xorl	%eax, %eax
1404de928: 86 05 63 62 15 03           	xchgb	%al, 0x3156263(%rip)    # 0x143634b91
1404de92e: 3c 02                       	cmpb	$0x2, %al
1404de930: 0f 84 3f 04 00 00           	je	0x1404ded75 <.text+0x4ddd75>
1404de936: 84 db                       	testb	%bl, %bl
1404de938: 0f 84 87 ae ff ff           	je	0x1404d97c5 <.text+0x4d87c5>
1404de93e: 48 8b 8d 88 14 00 00        	movq	0x1488(%rbp), %rcx
1404de945: 48 ba 40 7c d4 87 e7 56 7b 3d       	movabsq	$0x3d7b56e787d47c40, %rdx # imm = 0x3D7B56E787D47C40
1404de94f: 49 b8 c7 bf 43 8d 0a b1 b0 67       	movabsq	$0x67b0b10a8d43bfc7, %r8 # imm = 0x67B0B10A8D43BFC7
1404de959: e8 92 ed bf ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1404de95e: 48 85 c0                    	testq	%rax, %rax
1404de961: 4c 8b 85 78 14 00 00        	movq	0x1478(%rbp), %r8
1404de968: 0f 84 35 08 00 00           	je	0x1404df1a3 <.text+0x4de1a3>
1404de96e: 49 39 90 48 02 00 00        	cmpq	%rdx, 0x248(%r8)
1404de975: 0f 86 28 08 00 00           	jbe	0x1404df1a3 <.text+0x4de1a3>
1404de97b: 49 8b 80 40 02 00 00        	movq	0x240(%r8), %rax
1404de982: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1404de986: 48 85 c9                    	testq	%rcx, %rcx
1404de989: 0f 84 14 08 00 00           	je	0x1404df1a3 <.text+0x4de1a3>
1404de98f: 49 8b 80 10 02 00 00        	movq	0x210(%r8), %rax
1404de996: 48 f7 d1                    	notq	%rcx
1404de999: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1404de99d: 48 c1 e1 05                 	shlq	$0x5, %rcx
1404de9a1: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1404de9a7: 0f 84 f6 07 00 00           	je	0x1404df1a3 <.text+0x4de1a3>
1404de9ad: 48 01 c8                    	addq	%rcx, %rax
1404de9b0: 48 8b 70 10                 	movq	0x10(%rax), %rsi
1404de9b4: 41 8b 80 e0 05 00 00        	movl	0x5e0(%r8), %eax
1404de9bb: 48 8b 8d 88 14 00 00        	movq	0x1488(%rbp), %rcx
1404de9c2: 48 ba cf de 92 1b 58 89 d1 2e       	movabsq	$0x2ed189581b92decf, %rdx # imm = 0x2ED189581B92DECF
1404de9cc: 49 b8 38 5f 06 96 93 d9 d3 15       	movabsq	$0x15d3d99396065f38, %r8 # imm = 0x15D3D99396065F38
1404de9d6: e8 15 ed bf ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1404de9db: 48 85 c0                    	testq	%rax, %rax
1404de9de: 4c 8b 85 78 14 00 00        	movq	0x1478(%rbp), %r8
1404de9e5: 0f 84 c9 07 00 00           	je	0x1404df1b4 <.text+0x4de1b4>
1404de9eb: 49 39 90 48 02 00 00        	cmpq	%rdx, 0x248(%r8)
1404de9f2: 0f 86 bc 07 00 00           	jbe	0x1404df1b4 <.text+0x4de1b4>
1404de9f8: 49 8b 80 40 02 00 00        	movq	0x240(%r8), %rax
1404de9ff: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1404dea03: 48 85 c9                    	testq	%rcx, %rcx
1404dea06: 0f 84 a8 07 00 00           	je	0x1404df1b4 <.text+0x4de1b4>
1404dea0c: 49 8b 80 10 02 00 00        	movq	0x210(%r8), %rax
1404dea13: 48 f7 d1                    	notq	%rcx
1404dea16: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1404dea1a: 48 c1 e1 05                 	shlq	$0x5, %rcx
1404dea1e: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1404dea24: 0f 84 8a 07 00 00           	je	0x1404df1b4 <.text+0x4de1b4>
1404dea2a: f6 c3 02                    	testb	$0x2, %bl
1404dea2d: 74 23                       	je	0x1404dea52 <.text+0x4dda52>
1404dea2f: 48 01 c8                    	addq	%rcx, %rax
1404dea32: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1404dea36: e8 65 92 4a 00              	callq	0x140987ca0 <.text+0x986ca0>
1404dea3b: 44 0f b6 86 a8 00 00 00     	movzbl	0xa8(%rsi), %r8d
1404dea43: 48 8b 8d 20 14 00 00        	movq	0x1420(%rbp), %rcx
1404dea4a: 48 89 c2                    	movq	%rax, %rdx
1404dea4d: e8 4e fe 72 00              	callq	0x140c0e8a0 <.text+0xc0d8a0>
1404dea52: f6 c3 05                    	testb	$0x5, %bl
1404dea55: 0f 84 65 01 00 00           	je	0x1404debc0 <.text+0x4ddbc0>
1404dea5b: 31 ff                       	xorl	%edi, %edi
1404dea5d: 83 7e 68 01                 	cmpl	$0x1, 0x68(%rsi)
1404dea61: 75 12                       	jne	0x1404dea75 <.text+0x4dda75>
1404dea63: 66 0f 10 46 6c              	movupd	0x6c(%rsi), %xmm0
1404dea68: 66 0f 29 85 e0 13 00 00     	movapd	%xmm0, 0x13e0(%rbp)
1404dea70: bf 01 00 00 00              	movl	$0x1, %edi
1404dea75: 0f b6 5e 7c                 	movzbl	0x7c(%rsi), %ebx
1404dea79: 44 0f b6 76 7d              	movzbl	0x7d(%rsi), %r14d
1404dea7e: 4c 8b 7e 60                 	movq	0x60(%rsi), %r15
1404dea82: 48 8b 46 38                 	movq	0x38(%rsi), %rax
1404dea86: 48 89 c1                    	movq	%rax, %rcx
1404dea89: 48 f7 d9                    	negq	%rcx
1404dea8c: 4c 8d 85 30 0c 00 00        	leaq	0xc30(%rbp), %r8
1404dea93: 70 2d                       	jo	0x1404deac2 <.text+0x4ddac2>
1404dea95: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
1404dea9f: 48 39 c8                    	cmpq	%rcx, %rax
1404deaa2: 74 32                       	je	0x1404dead6 <.text+0x4ddad6>
1404deaa4: 48 8d 56 38                 	leaq	0x38(%rsi), %rdx
1404deaa8: 48 8d 8d 00 06 00 00        	leaq	0x600(%rbp), %rcx
1404deaaf: e8 dc d2 89 00              	callq	0x140d7bd90 <.text+0xd7ad90>
1404deab4: 48 8b 85 00 06 00 00        	movq	0x600(%rbp), %rax
1404deabb: 4c 8d 85 30 0c 00 00        	leaq	0xc30(%rbp), %r8
1404deac2: 0f 10 46 50                 	movups	0x50(%rsi), %xmm0
1404deac6: 48 8d 8d 08 06 00 00        	leaq	0x608(%rbp), %rcx
1404deacd: f3 0f 6f 09                 	movdqu	(%rcx), %xmm1
1404dead1: 66 0f 7f 4d c0              	movdqa	%xmm1, -0x40(%rbp)
1404dead6: 48 89 85 38 0c 00 00        	movq	%rax, 0xc38(%rbp)
1404deadd: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
1404deae2: 48 8d 85 38 0c 00 00        	leaq	0xc38(%rbp), %rax
1404deae9: f3 0f 7f 48 08              	movdqu	%xmm1, 0x8(%rax)
1404deaee: 0f 11 85 50 0c 00 00        	movups	%xmm0, 0xc50(%rbp)
1404deaf5: 4c 89 bd 60 0c 00 00        	movq	%r15, 0xc60(%rbp)
1404deafc: 89 bd 68 0c 00 00           	movl	%edi, 0xc68(%rbp)
1404deb02: 66 0f 28 85 e0 13 00 00     	movapd	0x13e0(%rbp), %xmm0
1404deb0a: 66 0f 11 40 34              	movupd	%xmm0, 0x34(%rax)
1404deb0f: 88 9d 7c 0c 00 00           	movb	%bl, 0xc7c(%rbp)
1404deb15: 44 88 b5 7d 0c 00 00        	movb	%r14b, 0xc7d(%rbp)
1404deb1c: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
1404deb26: 48 89 85 30 0c 00 00        	movq	%rax, 0xc30(%rbp)
1404deb2d: 48 8d 8d 00 06 00 00        	leaq	0x600(%rbp), %rcx
1404deb34: 48 8b 95 18 14 00 00        	movq	0x1418(%rbp), %rdx
1404deb3b: e8 d0 6e ee ff              	callq	0x1403c5a10 <.text+0x3c4a10>
1404deb40: 48 8b 85 00 06 00 00        	movq	0x600(%rbp), %rax
1404deb47: 48 b9 07 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff9, %rcx # imm = 0x8000000000000007
1404deb51: 48 39 c8                    	cmpq	%rcx, %rax
1404deb54: 74 6a                       	je	0x1404debc0 <.text+0x4ddbc0>
1404deb56: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
1404deb60: 48 31 c8                    	xorq	%rcx, %rax
1404deb63: 48 83 f8 07                 	cmpq	$0x7, %rax
1404deb67: b9 03 00 00 00              	movl	$0x3, %ecx
1404deb6c: 48 0f 43 c1                 	cmovaeq	%rcx, %rax
1404deb70: 48 83 f8 04                 	cmpq	$0x4, %rax
1404deb74: 74 14                       	je	0x1404deb8a <.text+0x4ddb8a>
1404deb76: 48 83 f8 03                 	cmpq	$0x3, %rax
1404deb7a: 75 44                       	jne	0x1404debc0 <.text+0x4ddbc0>
1404deb7c: 48 8d 8d 00 06 00 00        	leaq	0x600(%rbp), %rcx
1404deb83: e8 88 21 11 00              	callq	0x1405f0d10 <.text+0x5efd10>
1404deb88: eb 36                       	jmp	0x1404debc0 <.text+0x4ddbc0>
1404deb8a: 48 8b 95 08 06 00 00        	movq	0x608(%rbp), %rdx
1404deb91: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
1404deb9b: 48 39 c2                    	cmpq	%rax, %rdx
1404deb9e: 7c 20                       	jl	0x1404debc0 <.text+0x4ddbc0>
1404deba0: 48 85 d2                    	testq	%rdx, %rdx
1404deba3: 74 1b                       	je	0x1404debc0 <.text+0x4ddbc0>
1404deba5: 48 8b 8d 10 06 00 00        	movq	0x610(%rbp), %rcx
1404debac: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1404debb2: e8 69 50 13 00              	callq	0x140613c20 <.text+0x612c20>
1404debb7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1404debc0: 48 8b 8d 88 14 00 00        	movq	0x1488(%rbp), %rcx
1404debc7: 48 ba a0 1c bf 3b bb 4f 55 0a       	movabsq	$0xa554fbb3bbf1ca0, %rdx # imm = 0xA554FBB3BBF1CA0
1404debd1: 49 b8 9d 8e bb 86 a9 93 b9 29       	movabsq	$0x29b993a986bb8e9d, %r8 # imm = 0x29B993A986BB8E9D
1404debdb: e8 10 eb bf ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1404debe0: 48 85 c0                    	testq	%rax, %rax
1404debe3: 4c 8b 85 78 14 00 00        	movq	0x1478(%rbp), %r8
1404debea: 0f 84 ed 05 00 00           	je	0x1404df1dd <.text+0x4de1dd>
1404debf0: 49 39 90 48 02 00 00        	cmpq	%rdx, 0x248(%r8)
1404debf7: 0f 86 e0 05 00 00           	jbe	0x1404df1dd <.text+0x4de1dd>
1404debfd: 49 8b 80 40 02 00 00        	movq	0x240(%r8), %rax
1404dec04: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1404dec08: 48 85 c9                    	testq	%rcx, %rcx
1404dec0b: 0f 84 cc 05 00 00           	je	0x1404df1dd <.text+0x4de1dd>
1404dec11: 49 8b 80 10 02 00 00        	movq	0x210(%r8), %rax
1404dec18: 48 f7 d1                    	notq	%rcx
1404dec1b: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1404dec1f: 48 c1 e1 05                 	shlq	$0x5, %rcx
1404dec23: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1404dec29: 0f 84 ae 05 00 00           	je	0x1404df1dd <.text+0x4de1dd>
1404dec2f: 48 01 c8                    	addq	%rcx, %rax
1404dec32: 48 8b 78 10                 	movq	0x10(%rax), %rdi
1404dec36: 41 8b 88 e0 05 00 00        	movl	0x5e0(%r8), %ecx
1404dec3d: 89 48 5c                    	movl	%ecx, 0x5c(%rax)
1404dec40: 48 8b 5f 30                 	movq	0x30(%rdi), %rbx
1404dec44: 4c 8b 77 40                 	movq	0x40(%rdi), %r14
1404dec48: 48 3b 5f 20                 	cmpq	0x20(%rdi), %rbx
1404dec4c: 75 10                       	jne	0x1404dec5e <.text+0x4ddc5e>
1404dec4e: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1404dec52: 48 8d 15 cf 44 51 02        	leaq	0x25144cf(%rip), %rdx   # 0x1429f3128
1404dec59: e8 12 3c 08 00              	callq	0x140562870 <.text+0x561870>
1404dec5e: 48 8b 47 28                 	movq	0x28(%rdi), %rax
