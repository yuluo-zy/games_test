
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412feb50: 55                          	pushq	%rbp
1412feb51: 41 57                       	pushq	%r15
1412feb53: 41 56                       	pushq	%r14
1412feb55: 41 55                       	pushq	%r13
1412feb57: 41 54                       	pushq	%r12
1412feb59: 56                          	pushq	%rsi
1412feb5a: 57                          	pushq	%rdi
1412feb5b: 53                          	pushq	%rbx
1412feb5c: 48 81 ec 28 01 00 00        	subq	$0x128, %rsp            # imm = 0x128
1412feb63: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412feb6b: 66 0f 7f b5 90 00 00 00     	movdqa	%xmm6, 0x90(%rbp)
1412feb73: 48 c7 85 88 00 00 00 fe ff ff ff    	movq	$-0x2, 0x88(%rbp)
1412feb7e: 4d 89 c6                    	movq	%r8, %r14
1412feb81: 48 89 d3                    	movq	%rdx, %rbx
1412feb84: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
1412feb88: 48 8d 72 28                 	leaq	0x28(%rdx), %rsi
1412feb8c: 48 83 3a 00                 	cmpq	$0x0, (%rdx)
1412feb90: 74 17                       	je	0x1412feba9 <.text+0x12fdba9>
1412feb92: 48 89 d9                    	movq	%rbx, %rcx
1412feb95: 48 89 f2                    	movq	%rsi, %rdx
1412feb98: e8 03 4a 26 00              	callq	0x1415635a0 <.text+0x15625a0>
1412feb9d: 83 f8 01                    	cmpl	$0x1, %eax
1412feba0: 74 7f                       	je	0x1412fec21 <.text+0x12fdc21>
1412feba2: 48 c7 03 00 00 00 00        	movq	$0x0, (%rbx)
1412feba9: 0f b6 83 88 00 00 00        	movzbl	0x88(%rbx), %eax
1412febb0: 3c 02                       	cmpb	$0x2, %al
1412febb2: 74 3e                       	je	0x1412febf2 <.text+0x12fdbf2>
1412febb4: 48 8b 93 80 00 00 00        	movq	0x80(%rbx), %rdx
1412febbb: 48 85 d2                    	testq	%rdx, %rdx
1412febbe: 74 32                       	je	0x1412febf2 <.text+0x12fdbf2>
1412febc0: 44 0f b7 43 78              	movzwl	0x78(%rbx), %r8d
1412febc5: 48 8b 4b 60                 	movq	0x60(%rbx), %rcx
1412febc9: 66 45 85 c0                 	testw	%r8w, %r8w
1412febcd: 0f 84 d6 03 00 00           	je	0x1412fefa9 <.text+0x12fdfa9>
1412febd3: 45 8d 48 ff                 	leal	-0x1(%r8), %r9d
1412febd7: 45 21 c1                    	andl	%r8d, %r9d
1412febda: 66 44 89 4b 78              	movw	%r9w, 0x78(%rbx)
1412febdf: 48 ff ca                    	decq	%rdx
1412febe2: 48 89 93 80 00 00 00        	movq	%rdx, 0x80(%rbx)
1412febe9: 48 85 c9                    	testq	%rcx, %rcx
1412febec: 0f 85 05 04 00 00           	jne	0x1412feff7 <.text+0x12fdff7>
1412febf2: 48 83 7b 30 00              	cmpq	$0x0, 0x30(%rbx)
1412febf7: 0f 84 79 04 00 00           	je	0x1412ff076 <.text+0x12fe076>
1412febfd: 48 8d 73 30                 	leaq	0x30(%rbx), %rsi
1412fec01: 48 8d 53 58                 	leaq	0x58(%rbx), %rdx
1412fec05: 48 89 f1                    	movq	%rsi, %rcx
1412fec08: e8 93 49 26 00              	callq	0x1415635a0 <.text+0x15625a0>
1412fec0d: 83 f8 01                    	cmpl	$0x1, %eax
1412fec10: 74 0f                       	je	0x1412fec21 <.text+0x12fdc21>
1412fec12: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
1412fec19: 85 c0                       	testl	%eax, %eax
1412fec1b: 0f 84 55 04 00 00           	je	0x1412ff076 <.text+0x12fe076>
1412fec21: 89 d6                       	movl	%edx, %esi
1412fec23: 0f b6 05 df 5f 33 02        	movzbl	0x2335fdf(%rip), %eax   # 0x143634c09
1412fec2a: b9 10 00 00 00              	movl	$0x10, %ecx
1412fec2f: ba 04 00 00 00              	movl	$0x4, %edx
1412fec34: e8 d7 4f 31 ff              	callq	0x140613c10 <.text+0x612c10>
1412fec39: 48 85 c0                    	testq	%rax, %rax
1412fec3c: 0f 84 6a 04 00 00           	je	0x1412ff0ac <.text+0x12fe0ac>
1412fec42: 48 89 c7                    	movq	%rax, %rdi
1412fec45: 89 30                       	movl	%esi, (%rax)
1412fec47: 48 c7 45 68 04 00 00 00     	movq	$0x4, 0x68(%rbp)
1412fec4f: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1412fec53: 48 c7 45 78 01 00 00 00     	movq	$0x1, 0x78(%rbp)
1412fec5b: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1412fec5f: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1412fec65: 48 89 da                    	movq	%rbx, %rdx
1412fec68: e8 93 4f 62 01              	callq	0x142923c00 <.text+0x2922c00>
1412fec6d: ba 01 00 00 00              	movl	$0x1, %edx
1412fec72: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1412fec76: 48 89 fe                    	movq	%rdi, %rsi
1412fec79: eb 0f                       	jmp	0x1412fec8a <.text+0x12fdc8a>
1412fec7b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1412fec80: 89 1c 96                    	movl	%ebx, (%rsi,%rdx,4)
1412fec83: 48 ff c2                    	incq	%rdx
1412fec86: 48 89 55 78                 	movq	%rdx, 0x78(%rbp)
1412fec8a: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
1412fec8e: 4c 8b 7d b0                 	movq	-0x50(%rbp), %r15
1412fec92: 4d 85 ff                    	testq	%r15, %r15
1412fec95: 0f 84 9d 01 00 00           	je	0x1412fee38 <.text+0x12fde38>
1412fec9b: 4c 8b 65 d0                 	movq	-0x30(%rbp), %r12
1412fec9f: 4d 85 e4                    	testq	%r12, %r12
1412feca2: 0f 84 88 01 00 00           	je	0x1412fee30 <.text+0x12fde30>
1412feca8: 4c 8b 6d d8                 	movq	-0x28(%rbp), %r13
1412fecac: 49 8d 45 20                 	leaq	0x20(%r13), %rax
1412fecb0: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1412fecb4: 0f b7 45 c8                 	movzwl	-0x38(%rbp), %eax
1412fecb8: 48 8b 7d b8                 	movq	-0x48(%rbp), %rdi
1412fecbc: e9 b1 00 00 00              	jmp	0x1412fed72 <.text+0x12fdd72>
1412fecc1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412fecd0: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1412fecd6: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1412fecda: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1412fecde: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
1412fece3: 45 85 db                    	testl	%r11d, %r11d
1412fece6: 74 48                       	je	0x1412fed30 <.text+0x12fdd30>
1412fece8: 4c 89 ea                    	movq	%r13, %rdx
1412feceb: 49 89 f2                    	movq	%rsi, %r10
1412fecee: f3 41 0f bc f3              	tzcntl	%r11d, %esi
1412fecf3: 4c 01 c6                    	addq	%r8, %rsi
1412fecf6: 48 23 75 50                 	andq	0x50(%rbp), %rsi
1412fecfa: 48 f7 de                    	negq	%rsi
1412fecfd: 48 6b f6 58                 	imulq	$0x58, %rsi, %rsi
1412fed01: 45 89 cd                    	movl	%r9d, %r13d
1412fed04: 4c 8b 4d 40                 	movq	0x40(%rbp), %r9
1412fed08: 45 3b 2c 31                 	cmpl	(%r9,%rsi), %r13d
1412fed0c: 74 52                       	je	0x1412fed60 <.text+0x12fdd60>
1412fed0e: 45 89 e9                    	movl	%r13d, %r9d
1412fed11: 41 8d 73 ff                 	leal	-0x1(%r11), %esi
1412fed15: 66 44 21 de                 	andw	%r11w, %si
1412fed19: 41 89 f3                    	movl	%esi, %r11d
1412fed1c: 4c 89 d6                    	movq	%r10, %rsi
1412fed1f: 49 89 d5                    	movq	%rdx, %r13
1412fed22: 75 c4                       	jne	0x1412fece8 <.text+0x12fdce8>
1412fed24: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1412fed30: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1412fed34: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
1412fed39: 45 85 db                    	testl	%r11d, %r11d
1412fed3c: 0f 85 2e 02 00 00           	jne	0x1412fef70 <.text+0x12fdf70>
1412fed42: 49 01 c0                    	addq	%rax, %r8
1412fed45: 49 83 c0 10                 	addq	$0x10, %r8
1412fed49: 48 83 c0 10                 	addq	$0x10, %rax
1412fed4d: 4c 23 45 50                 	andq	0x50(%rbp), %r8
1412fed51: e9 7a ff ff ff              	jmp	0x1412fecd0 <.text+0x12fdcd0>
1412fed56: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1412fed60: 44 89 f0                    	movl	%r14d, %eax
1412fed63: 4d 85 e4                    	testq	%r12, %r12
1412fed66: 4c 89 d6                    	movq	%r10, %rsi
1412fed69: 49 89 d5                    	movq	%rdx, %r13
1412fed6c: 0f 84 be 00 00 00           	je	0x1412fee30 <.text+0x12fde30>
1412fed72: 66 85 c0                    	testw	%ax, %ax
1412fed75: 74 09                       	je	0x1412fed80 <.text+0x12fdd80>
1412fed77: 44 8d 70 ff                 	leal	-0x1(%rax), %r14d
1412fed7b: 41 21 c6                    	andl	%eax, %r14d
1412fed7e: eb 31                       	jmp	0x1412fedb1 <.text+0x12fddb1>
1412fed80: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
1412fed84: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1412fed88: 49 81 c7 80 fa ff ff        	addq	$-0x580, %r15           # imm = 0xFA80
1412fed8f: 48 83 c7 10                 	addq	$0x10, %rdi
1412fed93: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1412fed98: 74 e6                       	je	0x1412fed80 <.text+0x12fdd80>
1412fed9a: 48 89 7d b8                 	movq	%rdi, -0x48(%rbp)
1412fed9e: 4c 89 7d b0                 	movq	%r15, -0x50(%rbp)
1412feda2: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1412feda7: 29 c1                       	subl	%eax, %ecx
1412feda9: f7 d0                       	notl	%eax
1412fedab: 41 89 c6                    	movl	%eax, %r14d
1412fedae: 41 21 ce                    	andl	%ecx, %r14d
1412fedb1: 66 44 89 75 c8              	movw	%r14w, -0x38(%rbp)
1412fedb6: 49 ff cc                    	decq	%r12
1412fedb9: 4c 89 65 d0                 	movq	%r12, -0x30(%rbp)
1412fedbd: f3 0f bc c0                 	tzcntl	%eax, %eax
1412fedc1: 48 f7 d8                    	negq	%rax
1412fedc4: 48 6b c0 58                 	imulq	$0x58, %rax, %rax
1412fedc8: 41 8b 5c 07 a8              	movl	-0x58(%r15,%rax), %ebx
1412fedcd: 89 9d 84 00 00 00           	movl	%ebx, 0x84(%rbp)
1412fedd3: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
1412fedd8: 0f 84 92 01 00 00           	je	0x1412fef70 <.text+0x12fdf70>
1412fedde: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1412fede2: 48 8d 95 84 00 00 00        	leaq	0x84(%rbp), %rdx
1412fede9: e8 e2 20 2b ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1412fedee: 49 8b 4d 00                 	movq	(%r13), %rcx
1412fedf2: 4d 8b 45 08                 	movq	0x8(%r13), %r8
1412fedf6: 4c 89 45 50                 	movq	%r8, 0x50(%rbp)
1412fedfa: 49 21 c0                    	andq	%rax, %r8
1412fedfd: 48 c1 e8 39                 	shrq	$0x39, %rax
1412fee01: 66 0f 6e c0                 	movd	%eax, %xmm0
1412fee05: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1412fee09: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1412fee0e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1412fee13: 48 8d 41 a8                 	leaq	-0x58(%rcx), %rax
1412fee17: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1412fee1b: 44 8b 8d 84 00 00 00        	movl	0x84(%rbp), %r9d
1412fee22: 31 c0                       	xorl	%eax, %eax
1412fee24: e9 a7 fe ff ff              	jmp	0x1412fecd0 <.text+0x12fdcd0>
1412fee29: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1412fee30: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
1412fee38: 0f b6 45 38                 	movzbl	0x38(%rbp), %eax
1412fee3c: 3c 02                       	cmpb	$0x2, %al
1412fee3e: 0f 84 fd 00 00 00           	je	0x1412fef41 <.text+0x12fdf41>
1412fee44: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
1412fee48: 48 85 d2                    	testq	%rdx, %rdx
1412fee4b: 0f 84 f0 00 00 00           	je	0x1412fef41 <.text+0x12fdf41>
1412fee51: 44 0f b7 45 28              	movzwl	0x28(%rbp), %r8d
1412fee56: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
1412fee5a: 66 45 85 c0                 	testw	%r8w, %r8w
1412fee5e: 74 20                       	je	0x1412fee80 <.text+0x12fde80>
1412fee60: 45 8d 48 ff                 	leal	-0x1(%r8), %r9d
1412fee64: 45 21 c1                    	andl	%r8d, %r9d
1412fee67: 66 44 89 4d 28              	movw	%r9w, 0x28(%rbp)
1412fee6c: 48 ff ca                    	decq	%rdx
1412fee6f: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
1412fee73: 48 85 c9                    	testq	%rcx, %rcx
1412fee76: 75 5c                       	jne	0x1412feed4 <.text+0x12fded4>
1412fee78: e9 c4 00 00 00              	jmp	0x1412fef41 <.text+0x12fdf41>
1412fee7d: 0f 1f 00                    	nopl	(%rax)
1412fee80: 4c 8b 4d 18                 	movq	0x18(%rbp), %r9
1412fee84: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1412fee90: 66 41 0f 6f 01              	movdqa	(%r9), %xmm0
1412fee95: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
1412fee9a: 48 81 c1 00 f9 ff ff        	addq	$-0x700, %rcx           # imm = 0xF900
1412feea1: 49 83 c1 10                 	addq	$0x10, %r9
1412feea5: 41 81 f8 ff ff 00 00        	cmpl	$0xffff, %r8d           # imm = 0xFFFF
1412feeac: 74 e2                       	je	0x1412fee90 <.text+0x12fde90>
1412feeae: 4c 89 4d 18                 	movq	%r9, 0x18(%rbp)
1412feeb2: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
1412feeb6: 41 b9 fe ff ff ff           	movl	$0xfffffffe, %r9d       # imm = 0xFFFFFFFE
1412feebc: 45 29 c1                    	subl	%r8d, %r9d
1412feebf: 41 f7 d0                    	notl	%r8d
1412feec2: 45 89 c2                    	movl	%r8d, %r10d
1412feec5: 45 21 ca                    	andl	%r9d, %r10d
1412feec8: 66 44 89 55 28              	movw	%r10w, 0x28(%rbp)
1412feecd: 48 ff ca                    	decq	%rdx
1412feed0: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
1412feed4: f3 41 0f bc d0              	tzcntl	%r8d, %edx
1412feed9: 48 f7 da                    	negq	%rdx
1412feedc: 48 6b d2 70                 	imulq	$0x70, %rdx, %rdx
1412feee0: 4c 8d 04 11                 	leaq	(%rcx,%rdx), %r8
1412feee4: 49 83 c0 a0                 	addq	$-0x60, %r8
1412feee8: 48 01 d1                    	addq	%rdx, %rcx
1412feeeb: 48 83 c1 d0                 	addq	$-0x30, %rcx
1412feeef: a8 01                       	testb	$0x1, %al
1412feef1: 48 89 c8                    	movq	%rcx, %rax
1412feef4: 49 0f 45 c0                 	cmovneq	%r8, %rax
1412feef8: 4c 0f 45 c1                 	cmovneq	%rcx, %r8
1412feefc: 4d 8b 38                    	movq	(%r8), %r15
1412feeff: 49 8b 48 08                 	movq	0x8(%r8), %rcx
1412fef03: 49 8d 4c 0f 01              	leaq	0x1(%r15,%rcx), %rcx
1412fef08: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1412fef0d: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1412fef11: f7 d2                       	notl	%edx
1412fef13: 4d 8d 4f 10                 	leaq	0x10(%r15), %r9
1412fef17: 4d 8b 40 18                 	movq	0x18(%r8), %r8
1412fef1b: 4c 89 7d b0                 	movq	%r15, -0x50(%rbp)
1412fef1f: 4c 89 4d b8                 	movq	%r9, -0x48(%rbp)
1412fef23: 48 89 4d c0                 	movq	%rcx, -0x40(%rbp)
1412fef27: 66 89 55 c8                 	movw	%dx, -0x38(%rbp)
1412fef2b: 4c 89 45 d0                 	movq	%r8, -0x30(%rbp)
1412fef2f: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1412fef33: 4d 85 ff                    	testq	%r15, %r15
1412fef36: 0f 85 5f fd ff ff           	jne	0x1412fec9b <.text+0x12fdc9b>
1412fef3c: e9 f7 fe ff ff              	jmp	0x1412fee38 <.text+0x12fde38>
1412fef41: 48 83 7d e0 00              	cmpq	$0x0, -0x20(%rbp)
1412fef46: 0f 84 10 01 00 00           	je	0x1412ff05c <.text+0x12fe05c>
1412fef4c: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1412fef50: 48 8d 55 08                 	leaq	0x8(%rbp), %rdx
1412fef54: e8 47 46 26 00              	callq	0x1415635a0 <.text+0x15625a0>
1412fef59: 89 d3                       	movl	%edx, %ebx
1412fef5b: 83 f8 01                    	cmpl	$0x1, %eax
1412fef5e: 0f 85 f8 00 00 00           	jne	0x1412ff05c <.text+0x12fe05c>
1412fef64: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1412fef70: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1412fef74: 48 3b 55 68                 	cmpq	0x68(%rbp), %rdx
1412fef78: 0f 85 02 fd ff ff           	jne	0x1412fec80 <.text+0x12fdc80>
1412fef7e: 48 c7 44 24 20 04 00 00 00  	movq	$0x4, 0x20(%rsp)
1412fef87: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1412fef8d: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1412fef93: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1412fef97: e8 c4 aa 5a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1412fef9c: 48 8b 75 70                 	movq	0x70(%rbp), %rsi
1412fefa0: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
1412fefa4: e9 d7 fc ff ff              	jmp	0x1412fec80 <.text+0x12fdc80>
1412fefa9: 4c 8b 4b 68                 	movq	0x68(%rbx), %r9
1412fefad: 0f 1f 00                    	nopl	(%rax)
1412fefb0: 66 41 0f 6f 01              	movdqa	(%r9), %xmm0
1412fefb5: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
1412fefba: 48 81 c1 00 f9 ff ff        	addq	$-0x700, %rcx           # imm = 0xF900
1412fefc1: 49 83 c1 10                 	addq	$0x10, %r9
1412fefc5: 41 81 f8 ff ff 00 00        	cmpl	$0xffff, %r8d           # imm = 0xFFFF
1412fefcc: 74 e2                       	je	0x1412fefb0 <.text+0x12fdfb0>
1412fefce: 4c 89 4b 68                 	movq	%r9, 0x68(%rbx)
1412fefd2: 48 89 4b 60                 	movq	%rcx, 0x60(%rbx)
1412fefd6: 41 b9 fe ff ff ff           	movl	$0xfffffffe, %r9d       # imm = 0xFFFFFFFE
1412fefdc: 45 29 c1                    	subl	%r8d, %r9d
1412fefdf: 41 f7 d0                    	notl	%r8d
1412fefe2: 45 89 c2                    	movl	%r8d, %r10d
1412fefe5: 45 21 ca                    	andl	%r9d, %r10d
1412fefe8: 66 44 89 53 78              	movw	%r10w, 0x78(%rbx)
1412fefed: 48 ff ca                    	decq	%rdx
1412feff0: 48 89 93 80 00 00 00        	movq	%rdx, 0x80(%rbx)
1412feff7: f3 41 0f bc d0              	tzcntl	%r8d, %edx
1412feffc: 48 f7 da                    	negq	%rdx
1412fefff: 48 6b d2 70                 	imulq	$0x70, %rdx, %rdx
1412ff003: 4c 8d 04 11                 	leaq	(%rcx,%rdx), %r8
1412ff007: 49 83 c0 a0                 	addq	$-0x60, %r8
1412ff00b: 48 01 d1                    	addq	%rdx, %rcx
1412ff00e: 48 83 c1 d0                 	addq	$-0x30, %rcx
1412ff012: a8 01                       	testb	$0x1, %al
1412ff014: 48 89 c8                    	movq	%rcx, %rax
1412ff017: 49 0f 45 c0                 	cmovneq	%r8, %rax
1412ff01b: 4c 0f 45 c1                 	cmovneq	%rcx, %r8
1412ff01f: 49 8b 08                    	movq	(%r8), %rcx
1412ff022: 49 8b 50 08                 	movq	0x8(%r8), %rdx
1412ff026: 48 8d 54 11 01              	leaq	0x1(%rcx,%rdx), %rdx
1412ff02b: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
1412ff02f: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
1412ff034: 41 f7 d1                    	notl	%r9d
1412ff037: 4d 8b 40 18                 	movq	0x18(%r8), %r8
1412ff03b: 48 89 0b                    	movq	%rcx, (%rbx)
1412ff03e: 48 83 c1 10                 	addq	$0x10, %rcx
1412ff042: 48 89 4b 08                 	movq	%rcx, 0x8(%rbx)
1412ff046: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1412ff04a: 66 44 89 4b 18              	movw	%r9w, 0x18(%rbx)
1412ff04f: 4c 89 43 20                 	movq	%r8, 0x20(%rbx)
1412ff053: 48 89 43 28                 	movq	%rax, 0x28(%rbx)
1412ff057: e9 36 fb ff ff              	jmp	0x1412feb92 <.text+0x12fdb92>
1412ff05c: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1412ff060: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1412ff064: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
1412ff068: 48 89 c8                    	movq	%rcx, %rax
1412ff06b: f3 0f 6f 45 68              	movdqu	0x68(%rbp), %xmm0
1412ff070: f3 0f 7f 01                 	movdqu	%xmm0, (%rcx)
1412ff074: eb 1b                       	jmp	0x1412ff091 <.text+0x12fe091>
1412ff076: 48 8b 45 60                 	movq	0x60(%rbp), %rax
1412ff07a: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1412ff081: 48 c7 40 08 04 00 00 00     	movq	$0x4, 0x8(%rax)
1412ff089: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
1412ff091: 0f 28 b5 90 00 00 00        	movaps	0x90(%rbp), %xmm6
1412ff098: 48 81 c4 28 01 00 00        	addq	$0x128, %rsp            # imm = 0x128
1412ff09f: 5b                          	popq	%rbx
1412ff0a0: 5f                          	popq	%rdi
1412ff0a1: 5e                          	popq	%rsi
1412ff0a2: 41 5c                       	popq	%r12
1412ff0a4: 41 5d                       	popq	%r13
1412ff0a6: 41 5e                       	popq	%r14
1412ff0a8: 41 5f                       	popq	%r15
1412ff0aa: 5d                          	popq	%rbp
1412ff0ab: c3                          	retq
1412ff0ac: b9 04 00 00 00              	movl	$0x4, %ecx
1412ff0b1: ba 10 00 00 00              	movl	$0x10, %edx
1412ff0b6: 4d 89 f0                    	movq	%r14, %r8
1412ff0b9: e8 25 9f 5d 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1412ff0be: cc                          	int3
1412ff0bf: 90                          	nop
1412ff0c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412ff0c5: 55                          	pushq	%rbp
1412ff0c6: 41 57                       	pushq	%r15
1412ff0c8: 41 56                       	pushq	%r14
1412ff0ca: 41 55                       	pushq	%r13
1412ff0cc: 41 54                       	pushq	%r12
1412ff0ce: 56                          	pushq	%rsi
1412ff0cf: 57                          	pushq	%rdi
1412ff0d0: 53                          	pushq	%rbx
1412ff0d1: 48 83 ec 38                 	subq	$0x38, %rsp
1412ff0d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412ff0dc: 66 0f 7f 74 24 20           	movdqa	%xmm6, 0x20(%rsp)
1412ff0e2: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
1412ff0e6: 48 85 d2                    	testq	%rdx, %rdx
1412ff0e9: 74 13                       	je	0x1412ff0fe <.text+0x12fe0fe>
1412ff0eb: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
1412ff0ef: 48 c1 e2 02                 	shlq	$0x2, %rdx
1412ff0f3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412ff0f9: e8 22 4b 31 ff              	callq	0x140613c20 <.text+0x612c20>
1412ff0fe: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1412ff103: 48 83 c4 38                 	addq	$0x38, %rsp
1412ff107: 5b                          	popq	%rbx
1412ff108: 5f                          	popq	%rdi
1412ff109: 5e                          	popq	%rsi
1412ff10a: 41 5c                       	popq	%r12
1412ff10c: 41 5d                       	popq	%r13
1412ff10e: 41 5e                       	popq	%r14
1412ff110: 41 5f                       	popq	%r15
1412ff112: 5d                          	popq	%rbp
1412ff113: c3                          	retq
