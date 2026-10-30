
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408238a0: 55                          	pushq	%rbp
1408238a1: 41 57                       	pushq	%r15
1408238a3: 41 56                       	pushq	%r14
1408238a5: 41 55                       	pushq	%r13
1408238a7: 41 54                       	pushq	%r12
1408238a9: 56                          	pushq	%rsi
1408238aa: 57                          	pushq	%rdi
1408238ab: 53                          	pushq	%rbx
1408238ac: 48 81 ec a8 03 00 00        	subq	$0x3a8, %rsp            # imm = 0x3A8
1408238b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408238bb: 48 c7 85 20 03 00 00 fe ff ff ff    	movq	$-0x2, 0x320(%rbp)
1408238c6: 49 89 d6                    	movq	%rdx, %r14
1408238c9: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
1408238d0: 8b 50 1c                    	movl	0x1c(%rax), %edx
1408238d3: 4c 8b 50 10                 	movq	0x10(%rax), %r10
1408238d7: 89 95 18 03 00 00           	movl	%edx, 0x318(%rbp)
1408238dd: 4c 89 95 f8 02 00 00        	movq	%r10, 0x2f8(%rbp)
1408238e4: 41 89 12                    	movl	%edx, (%r10)
1408238e7: 4c 8b 38                    	movq	(%rax), %r15
1408238ea: 41 0f b6 87 82 02 00 00     	movzbl	0x282(%r15), %eax
1408238f2: 41 c6 87 82 02 00 00 00     	movb	$0x0, 0x282(%r15)
1408238fa: 85 c0                       	testl	%eax, %eax
1408238fc: 0f 84 b3 05 00 00           	je	0x140823eb5 <.text+0x822eb5>
140823902: 48 8b 95 a0 03 00 00        	movq	0x3a0(%rbp), %rdx
140823909: 83 f8 01                    	cmpl	$0x1, %eax
14082390c: 0f 85 a7 03 00 00           	jne	0x140823cb9 <.text+0x822cb9>
140823912: 4c 89 85 c8 02 00 00        	movq	%r8, 0x2c8(%rbp)
140823919: 4c 89 8d c0 02 00 00        	movq	%r9, 0x2c0(%rbp)
140823920: 48 8b 11                    	movq	(%rcx), %rdx
140823923: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140823927: e8 d4 81 30 00              	callq	0x140b2bb00 <.text+0xb2ab00>
14082392c: 0f b6 45 a0                 	movzbl	-0x60(%rbp), %eax
140823930: 88 85 e0 02 00 00           	movb	%al, 0x2e0(%rbp)
140823936: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
14082393a: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
140823941: 48 8b 75 b0                 	movq	-0x50(%rbp), %rsi
140823945: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
140823949: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
140823950: 48 8b 7d c0                 	movq	-0x40(%rbp), %rdi
140823954: 4c 8b 6d c8                 	movq	-0x38(%rbp), %r13
140823958: 0f b7 45 d8                 	movzwl	-0x28(%rbp), %eax
14082395c: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
140823960: 48 89 8d 08 03 00 00        	movq	%rcx, 0x308(%rbp)
140823967: 4c 89 bd f0 02 00 00        	movq	%r15, 0x2f0(%rbp)
14082396e: 49 8d 4f 18                 	leaq	0x18(%r15), %rcx
140823972: 48 89 8d d0 02 00 00        	movq	%rcx, 0x2d0(%rbp)
140823979: 49 8b 0e                    	movq	(%r14), %rcx
14082397c: 48 89 8d d8 02 00 00        	movq	%rcx, 0x2d8(%rbp)
140823983: 48 83 fe 02                 	cmpq	$0x2, %rsi
140823987: 0f 84 63 01 00 00           	je	0x140823af0 <.text+0x822af0>
14082398d: eb 71                       	jmp	0x140823a00 <.text+0x822a00>
14082398f: 90                          	nop
140823990: be 02 00 00 00              	movl	$0x2, %esi
140823995: 41 89 c4                    	movl	%eax, %r12d
140823998: 48 c7 85 00 03 00 00 00 00 00 00    	movq	$0x0, 0x300(%rbp)
1408239a3: 41 be 02 00 00 00           	movl	$0x2, %r14d
1408239a9: 8b 85 18 03 00 00           	movl	0x318(%rbp), %eax
1408239af: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1408239b6: 89 01                       	movl	%eax, (%rcx)
1408239b8: 48 8b 8d d0 02 00 00        	movq	0x2d0(%rbp), %rcx
1408239bf: 4c 89 f2                    	movq	%r14, %rdx
1408239c2: 49 89 d8                    	movq	%rbx, %r8
1408239c5: e8 66 95 1f 00              	callq	0x140a1cf30 <.text+0xa1bf30>
1408239ca: 49 89 c7                    	movq	%rax, %r15
1408239cd: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
1408239d4: 4c 89 f2                    	movq	%r14, %rdx
1408239d7: 49 89 d8                    	movq	%rbx, %r8
1408239da: e8 e1 88 1f 00              	callq	0x140a1c2c0 <.text+0xa1b2c0>
1408239df: 48 85 c0                    	testq	%rax, %rax
1408239e2: 0f 84 36 0c 00 00           	je	0x14082461e <.text+0x82361e>
1408239e8: 4c 89 f9                    	movq	%r15, %rcx
1408239eb: 48 89 c2                    	movq	%rax, %rdx
1408239ee: e8 bd 39 a3 00              	callq	0x1412573b0 <.text+0x12563b0>
1408239f3: 44 89 e0                    	movl	%r12d, %eax
1408239f6: 48 83 fe 02                 	cmpq	$0x2, %rsi
1408239fa: 0f 84 f0 00 00 00           	je	0x140823af0 <.text+0x822af0>
140823a00: 48 85 ff                    	testq	%rdi, %rdi
140823a03: 0f 84 80 00 00 00           	je	0x140823a89 <.text+0x822a89>
140823a09: 48 83 bd 08 03 00 00 00     	cmpq	$0x0, 0x308(%rbp)
140823a11: 0f 84 b9 00 00 00           	je	0x140823ad0 <.text+0x822ad0>
140823a17: 66 85 c0                    	testw	%ax, %ax
140823a1a: 74 14                       	je	0x140823a30 <.text+0x822a30>
140823a1c: 44 8d 60 ff                 	leal	-0x1(%rax), %r12d
140823a20: 41 21 c4                    	andl	%eax, %r12d
140823a23: eb 36                       	jmp	0x140823a5b <.text+0x822a5b>
140823a25: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140823a30: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
140823a36: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140823a3a: 48 81 c7 80 f6 ff ff        	addq	$-0x980, %rdi           # imm = 0xF680
140823a41: 49 83 c5 10                 	addq	$0x10, %r13
140823a45: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140823a4a: 74 e4                       	je	0x140823a30 <.text+0x822a30>
140823a4c: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140823a51: 29 c1                       	subl	%eax, %ecx
140823a53: f7 d0                       	notl	%eax
140823a55: 41 89 c4                    	movl	%eax, %r12d
140823a58: 41 21 cc                    	andl	%ecx, %r12d
140823a5b: 48 ff 8d 08 03 00 00        	decq	0x308(%rbp)
140823a62: f3 0f bc c0                 	tzcntl	%eax, %eax
140823a66: 48 f7 d8                    	negq	%rax
140823a69: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
140823a70: 48 8d 0c 07                 	leaq	(%rdi,%rax), %rcx
140823a74: 48 81 c1 68 ff ff ff        	addq	$-0x98, %rcx
140823a7b: e8 20 88 1f 00              	callq	0x140a1c2a0 <.text+0xa1b2a0>
140823a80: 48 83 f8 03                 	cmpq	$0x3, %rax
140823a84: 75 5f                       	jne	0x140823ae5 <.text+0x822ae5>
140823a86: 44 89 e0                    	movl	%r12d, %eax
140823a89: 40 f6 c6 01                 	testb	$0x1, %sil
140823a8d: 74 52                       	je	0x140823ae1 <.text+0x822ae1>
140823a8f: 41 be 01 00 00 00           	movl	$0x1, %r14d
140823a95: bf 00 00 00 00              	movl	$0x0, %edi
140823a9a: 48 83 bd 10 03 00 00 00     	cmpq	$0x0, 0x310(%rbp)
140823aa2: b9 00 00 00 00              	movl	$0x0, %ecx
140823aa7: 48 89 8d 10 03 00 00        	movq	%rcx, 0x310(%rbp)
140823aae: 74 40                       	je	0x140823af0 <.text+0x822af0>
140823ab0: 41 89 c4                    	movl	%eax, %r12d
140823ab3: 31 ff                       	xorl	%edi, %edi
140823ab5: 48 c7 85 10 03 00 00 00 00 00 00    	movq	$0x0, 0x310(%rbp)
140823ac0: e9 e4 fe ff ff              	jmp	0x1408239a9 <.text+0x8229a9>
140823ac5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140823ad0: 48 c7 85 08 03 00 00 00 00 00 00    	movq	$0x0, 0x308(%rbp)
140823adb: 40 f6 c6 01                 	testb	$0x1, %sil
140823adf: 75 ae                       	jne	0x140823a8f <.text+0x822a8f>
140823ae1: 31 ff                       	xorl	%edi, %edi
140823ae3: eb 0b                       	jmp	0x140823af0 <.text+0x822af0>
140823ae5: 49 89 c6                    	movq	%rax, %r14
140823ae8: 48 89 d3                    	movq	%rdx, %rbx
140823aeb: e9 b9 fe ff ff              	jmp	0x1408239a9 <.text+0x8229a9>
140823af0: 48 83 bd 00 03 00 00 00     	cmpq	$0x0, 0x300(%rbp)
140823af8: 0f 95 c1                    	setne	%cl
140823afb: 84 8d e0 02 00 00           	testb	%cl, 0x2e0(%rbp)
140823b01: 0f 85 89 fe ff ff           	jne	0x140823990 <.text+0x822990>
140823b07: 8b bd 18 03 00 00           	movl	0x318(%rbp), %edi
140823b0d: 48 8b 9d f8 02 00 00        	movq	0x2f8(%rbp), %rbx
140823b14: 89 3b                       	movl	%edi, (%rbx)
140823b16: 4c 8b bd f0 02 00 00        	movq	0x2f0(%rbp), %r15
140823b1d: 49 8d 8f 30 02 00 00        	leaq	0x230(%r15), %rcx
140823b24: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
140823b2b: 48 8b 30                    	movq	(%rax), %rsi
140823b2e: 48 89 f2                    	movq	%rsi, %rdx
140823b31: e8 7a f4 a3 00              	callq	0x141262fb0 <.text+0x1261fb0>
140823b36: 0f 10 46 20                 	movups	0x20(%rsi), %xmm0
140823b3a: 41 0f 11 87 50 02 00 00     	movups	%xmm0, 0x250(%r15)
140823b42: 89 3b                       	movl	%edi, (%rbx)
140823b44: 49 8d 8f d0 01 00 00        	leaq	0x1d0(%r15), %rcx
140823b4b: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
140823b52: 48 8b 30                    	movq	(%rax), %rsi
140823b55: 48 89 f2                    	movq	%rsi, %rdx
140823b58: e8 b3 f6 a3 00              	callq	0x141263210 <.text+0x1262210>
140823b5d: 0f 10 46 20                 	movups	0x20(%rsi), %xmm0
140823b61: 41 0f 11 87 f0 01 00 00     	movups	%xmm0, 0x1f0(%r15)
140823b69: 89 3b                       	movl	%edi, (%rbx)
140823b6b: 49 8d 8f 00 02 00 00        	leaq	0x200(%r15), %rcx
140823b72: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
140823b79: 48 8b 30                    	movq	(%rax), %rsi
140823b7c: 48 89 f2                    	movq	%rsi, %rdx
140823b7f: e8 7c ef a3 00              	callq	0x141262b00 <.text+0x1261b00>
140823b84: 0f 10 46 20                 	movups	0x20(%rsi), %xmm0
140823b88: 41 0f 11 87 20 02 00 00     	movups	%xmm0, 0x220(%r15)
140823b90: 89 3b                       	movl	%edi, (%rbx)
140823b92: 49 8d b7 60 02 00 00        	leaq	0x260(%r15), %rsi
140823b99: 48 8b 85 98 03 00 00        	movq	0x398(%rbp), %rax
140823ba0: 48 8b 10                    	movq	(%rax), %rdx
140823ba3: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140823ba7: e8 f4 f9 00 00              	callq	0x1408335a0 <.text+0x8325a0>
140823bac: 49 8b 87 68 02 00 00        	movq	0x268(%r15), %rax
140823bb3: 48 85 c0                    	testq	%rax, %rax
140823bb6: 74 1d                       	je	0x140823bd5 <.text+0x822bd5>
140823bb8: 49 8b 8f 70 02 00 00        	movq	0x270(%r15), %rcx
140823bbf: 48 89 c2                    	movq	%rax, %rdx
140823bc2: 48 c1 e2 07                 	shlq	$0x7, %rdx
140823bc6: 48 8d 14 c2                 	leaq	(%rdx,%rax,8), %rdx
140823bca: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140823bd0: e8 4b 00 df ff              	callq	0x140613c20 <.text+0x612c20>
140823bd5: f3 0f 6f 45 a0              	movdqu	-0x60(%rbp), %xmm0
140823bda: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140823bde: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140823be2: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
140823be6: 8b 85 18 03 00 00           	movl	0x318(%rbp), %eax
140823bec: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
140823bf3: 89 01                       	movl	%eax, (%rcx)
140823bf5: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
140823bfc: 48 8b 08                    	movq	(%rax), %rcx
140823bff: 48 8b 41 08                 	movq	0x8(%rcx), %rax
140823c03: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
140823c07: 49 8b 57 10                 	movq	0x10(%r15), %rdx
140823c0b: 4c 39 c2                    	cmpq	%r8, %rdx
140823c0e: 72 07                       	jb	0x140823c17 <.text+0x822c17>
140823c10: 4d 89 47 10                 	movq	%r8, 0x10(%r15)
140823c14: 4c 89 c2                    	movq	%r8, %rdx
140823c17: 48 85 d2                    	testq	%rdx, %rdx
140823c1a: 0f 84 61 02 00 00           	je	0x140823e81 <.text+0x822e81>
140823c20: 49 8b 4f 08                 	movq	0x8(%r15), %rcx
140823c24: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140823c2a: 4c 8d 15 47 d6 26 02        	leaq	0x226d647(%rip), %r10   # 0x142a91278
140823c31: 49 89 d3                    	movq	%rdx, %r11
140823c34: eb 35                       	jmp	0x140823c6b <.text+0x822c6b>
140823c36: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140823c40: be 05 00 00 00              	movl	$0x5, %esi
140823c45: 4a 89 74 09 f8              	movq	%rsi, -0x8(%rcx,%r9)
140823c4a: 66 0f 6f 45 a0              	movdqa	-0x60(%rbp), %xmm0
140823c4f: f3 42 0f 7f 04 09           	movdqu	%xmm0, (%rcx,%r9)
140823c55: 48 8b 75 b0                 	movq	-0x50(%rbp), %rsi
140823c59: 4a 89 74 09 10              	movq	%rsi, 0x10(%rcx,%r9)
140823c5e: 49 83 c1 20                 	addq	$0x20, %r9
140823c62: 49 ff cb                    	decq	%r11
140823c65: 0f 84 16 02 00 00           	je	0x140823e81 <.text+0x822e81>
140823c6b: 4a 8b 7c 08 f8              	movq	-0x8(%rax,%r9), %rdi
140823c70: 48 8d 5f fd                 	leaq	-0x3(%rdi), %rbx
140823c74: 48 83 fb 03                 	cmpq	$0x3, %rbx
140823c78: be 03 00 00 00              	movl	$0x3, %esi
140823c7d: 48 0f 43 de                 	cmovaeq	%rsi, %rbx
140823c81: 49 63 1c 9a                 	movslq	(%r10,%rbx,4), %rbx
140823c85: 4c 01 d3                    	addq	%r10, %rbx
140823c88: ff e3                       	jmpq	*%rbx
140823c8a: be 04 00 00 00              	movl	$0x4, %esi
140823c8f: eb b4                       	jmp	0x140823c45 <.text+0x822c45>
140823c91: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140823ca0: 4a 8b 74 08 10              	movq	0x10(%rax,%r9), %rsi
140823ca5: 48 89 75 b0                 	movq	%rsi, -0x50(%rbp)
140823ca9: f3 42 0f 6f 04 08           	movdqu	(%rax,%r9), %xmm0
140823caf: 66 0f 7f 45 a0              	movdqa	%xmm0, -0x60(%rbp)
140823cb4: 48 89 fe                    	movq	%rdi, %rsi
140823cb7: eb 8c                       	jmp	0x140823c45 <.text+0x822c45>
140823cb9: 41 80 bf 81 02 00 00 00     	cmpb	$0x0, 0x281(%r15)
140823cc1: 0f 84 ee 01 00 00           	je	0x140823eb5 <.text+0x822eb5>
140823cc7: 4c 89 85 c8 02 00 00        	movq	%r8, 0x2c8(%rbp)
140823cce: 4c 89 8d c0 02 00 00        	movq	%r9, 0x2c0(%rbp)
140823cd5: 4c 89 bd f0 02 00 00        	movq	%r15, 0x2f0(%rbp)
140823cdc: 48 8b 02                    	movq	(%rdx), %rax
140823cdf: 48 89 85 78 02 00 00        	movq	%rax, 0x278(%rbp)
140823ce6: 0f b6 40 48                 	movzbl	0x48(%rax), %eax
140823cea: 88 85 1e 03 00 00           	movb	%al, 0x31e(%rbp)
140823cf0: 48 8b 11                    	movq	(%rcx), %rdx
140823cf3: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140823cf7: 48 89 95 10 03 00 00        	movq	%rdx, 0x310(%rbp)
140823cfe: e8 7d 7e 30 00              	callq	0x140b2bb80 <.text+0xb2ab80>
140823d03: 48 8b 55 a0                 	movq	-0x60(%rbp), %rdx
140823d07: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
140823d0b: 48 89 85 00 03 00 00        	movq	%rax, 0x300(%rbp)
140823d12: 48 8b 7d b0                 	movq	-0x50(%rbp), %rdi
140823d16: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
140823d1a: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
140823d21: 4c 8b 6d c0                 	movq	-0x40(%rbp), %r13
140823d25: 4c 8b 7d c8                 	movq	-0x38(%rbp), %r15
140823d29: 0f b7 75 d8                 	movzwl	-0x28(%rbp), %esi
140823d2d: 4c 8b 65 e0                 	movq	-0x20(%rbp), %r12
140823d31: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
140823d35: 89 85 d0 02 00 00           	movl	%eax, 0x2d0(%rbp)
140823d3b: 49 8b 06                    	movq	(%r14), %rax
140823d3e: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
140823d45: 49 8b 46 10                 	movq	0x10(%r14), %rax
140823d49: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
140823d50: 48 83 ff 05                 	cmpq	$0x5, %rdi
140823d54: 0f 84 b6 00 00 00           	je	0x140823e10 <.text+0x822e10>
140823d5a: 4d 85 ed                    	testq	%r13, %r13
140823d5d: 0f 84 7e 00 00 00           	je	0x140823de1 <.text+0x822de1>
140823d63: 4d 85 e4                    	testq	%r12, %r12
140823d66: 74 76                       	je	0x140823dde <.text+0x822dde>
140823d68: 66 85 f6                    	testw	%si, %si
140823d6b: 74 13                       	je	0x140823d80 <.text+0x822d80>
140823d6d: 48 89 d3                    	movq	%rdx, %rbx
140823d70: 44 8d 76 ff                 	leal	-0x1(%rsi), %r14d
140823d74: 41 21 f6                    	andl	%esi, %r14d
140823d77: eb 35                       	jmp	0x140823dae <.text+0x822dae>
140823d79: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140823d80: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
140823d85: 66 0f d7 f0                 	pmovmskb	%xmm0, %esi
140823d89: 49 81 c5 80 f6 ff ff        	addq	$-0x980, %r13           # imm = 0xF680
140823d90: 49 83 c7 10                 	addq	$0x10, %r15
140823d94: 81 fe ff ff 00 00           	cmpl	$0xffff, %esi           # imm = 0xFFFF
140823d9a: 74 e4                       	je	0x140823d80 <.text+0x822d80>
140823d9c: 48 89 d3                    	movq	%rdx, %rbx
140823d9f: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140823da4: 29 f0                       	subl	%esi, %eax
140823da6: f7 d6                       	notl	%esi
140823da8: 41 89 f6                    	movl	%esi, %r14d
140823dab: 41 21 c6                    	andl	%eax, %r14d
140823dae: 49 ff cc                    	decq	%r12
140823db1: f3 0f bc c6                 	tzcntl	%esi, %eax
140823db5: 48 f7 d8                    	negq	%rax
140823db8: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
140823dbf: 49 8b 8c 05 68 ff ff ff     	movq	-0x98(%r13,%rax), %rcx
140823dc7: e8 54 87 09 00              	callq	0x1408bc520 <.text+0x8bb520>
140823dcc: 48 83 f8 03                 	cmpq	$0x3, %rax
140823dd0: 0f 85 9d 00 00 00           	jne	0x140823e73 <.text+0x822e73>
140823dd6: 44 89 f6                    	movl	%r14d, %esi
140823dd9: 48 89 da                    	movq	%rbx, %rdx
140823ddc: eb 03                       	jmp	0x140823de1 <.text+0x822de1>
140823dde: 45 31 e4                    	xorl	%r12d, %r12d
140823de1: 48 8d 47 fd                 	leaq	-0x3(%rdi), %rax
140823de5: 41 be 03 00 00 00           	movl	$0x3, %r14d
140823deb: 41 bd 00 00 00 00           	movl	$0x0, %r13d
140823df1: 48 83 f8 01                 	cmpq	$0x1, %rax
140823df5: 76 19                       	jbe	0x140823e10 <.text+0x822e10>
140823df7: 45 31 ed                    	xorl	%r13d, %r13d
140823dfa: 48 89 d3                    	movq	%rdx, %rbx
140823dfd: 4c 8b 85 08 03 00 00        	movq	0x308(%rbp), %r8
140823e04: 48 89 fa                    	movq	%rdi, %rdx
140823e07: eb 27                       	jmp	0x140823e30 <.text+0x822e30>
140823e09: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140823e10: 48 8d 42 fd                 	leaq	-0x3(%rdx), %rax
140823e14: 48 83 f8 01                 	cmpq	$0x1, %rax
140823e18: 0f 86 b6 02 00 00           	jbe	0x1408240d4 <.text+0x8230d4>
140823e1e: bb 03 00 00 00              	movl	$0x3, %ebx
140823e23: 41 be 05 00 00 00           	movl	$0x5, %r14d
140823e29: 4c 8b 85 00 03 00 00        	movq	0x300(%rbp), %r8
140823e30: 8b 85 d0 02 00 00           	movl	0x2d0(%rbp), %eax
140823e36: 48 8b 8d e0 02 00 00        	movq	0x2e0(%rbp), %rcx
140823e3d: 89 01                       	movl	%eax, (%rcx)
140823e3f: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
140823e46: e8 95 8f 1f 00              	callq	0x140a1cde0 <.text+0xa1bde0>
140823e4b: 49 89 d0                    	movq	%rdx, %r8
140823e4e: 4c 89 f7                    	movq	%r14, %rdi
140823e51: 48 89 da                    	movq	%rbx, %rdx
140823e54: 48 85 c0                    	testq	%rax, %rax
140823e57: 0f 84 f3 fe ff ff           	je	0x140823d50 <.text+0x822d50>
140823e5d: 48 89 c1                    	movq	%rax, %rcx
140823e60: 4c 89 c2                    	movq	%r8, %rdx
140823e63: e8 98 3a a3 00              	callq	0x141257900 <.text+0x1256900>
140823e68: 4c 89 f7                    	movq	%r14, %rdi
140823e6b: 48 89 da                    	movq	%rbx, %rdx
140823e6e: e9 dd fe ff ff              	jmp	0x140823d50 <.text+0x822d50>
140823e73: 49 89 d0                    	movq	%rdx, %r8
140823e76: 44 89 f6                    	movl	%r14d, %esi
140823e79: 49 89 fe                    	movq	%rdi, %r14
140823e7c: 48 89 c2                    	movq	%rax, %rdx
140823e7f: eb af                       	jmp	0x140823e30 <.text+0x822e30>
140823e81: 48 c1 e2 05                 	shlq	$0x5, %rdx
140823e85: 48 01 c2                    	addq	%rax, %rdx
140823e88: 49 c1 e0 05                 	shlq	$0x5, %r8
140823e8c: 49 01 c0                    	addq	%rax, %r8
140823e8f: 4c 8d 0d ea 80 40 02        	leaq	0x24080ea(%rip), %r9    # 0x142c2bf80
140823e96: 4c 89 f9                    	movq	%r15, %rcx
140823e99: e8 72 a8 a3 00              	callq	0x14125e710 <.text+0x125d710>
140823e9e: 8b 85 18 03 00 00           	movl	0x318(%rbp), %eax
140823ea4: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
140823eab: 89 01                       	movl	%eax, (%rcx)
140823ead: 41 c6 87 81 02 00 00 01     	movb	$0x1, 0x281(%r15)
140823eb5: 41 80 bf 80 02 00 00 00     	cmpb	$0x0, 0x280(%r15)
140823ebd: 0f 84 fc 01 00 00           	je	0x1408240bf <.text+0x8230bf>
140823ec3: 8b 85 18 03 00 00           	movl	0x318(%rbp), %eax
140823ec9: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
140823ed0: 89 01                       	movl	%eax, (%rcx)
140823ed2: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140823ed6: e8 a5 07 00 00              	callq	0x140824680 <.text+0x823680>
140823edb: 49 8d 4f 18                 	leaq	0x18(%r15), %rcx
140823edf: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140823ee3: e8 78 78 1f 00              	callq	0x140a1b760 <.text+0xa1a760>
140823ee8: 49 8d 8f d0 01 00 00        	leaq	0x1d0(%r15), %rcx
140823eef: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
140823ef6: e8 15 f3 a3 00              	callq	0x141263210 <.text+0x1262210>
140823efb: f3 0f 6f 85 90 01 00 00     	movdqu	0x190(%rbp), %xmm0
140823f03: f3 41 0f 7f 87 f0 01 00 00  	movdqu	%xmm0, 0x1f0(%r15)
140823f0c: 49 8d 8f 00 02 00 00        	leaq	0x200(%r15), %rcx
140823f13: 48 8d 95 a0 01 00 00        	leaq	0x1a0(%rbp), %rdx
140823f1a: e8 e1 eb a3 00              	callq	0x141262b00 <.text+0x1261b00>
140823f1f: f3 0f 6f 85 c0 01 00 00     	movdqu	0x1c0(%rbp), %xmm0
140823f27: f3 41 0f 7f 87 20 02 00 00  	movdqu	%xmm0, 0x220(%r15)
140823f30: 49 8d 8f 30 02 00 00        	leaq	0x230(%r15), %rcx
140823f37: 48 8d 95 d0 01 00 00        	leaq	0x1d0(%rbp), %rdx
140823f3e: e8 6d f0 a3 00              	callq	0x141262fb0 <.text+0x1261fb0>
140823f43: f3 0f 6f 85 f0 01 00 00     	movdqu	0x1f0(%rbp), %xmm0
140823f4b: f3 41 0f 7f 87 50 02 00 00  	movdqu	%xmm0, 0x250(%r15)
140823f54: 48 8d 95 00 02 00 00        	leaq	0x200(%rbp), %rdx
140823f5b: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
140823f62: e8 39 f6 00 00              	callq	0x1408335a0 <.text+0x8325a0>
140823f67: 49 8d b7 60 02 00 00        	leaq	0x260(%r15), %rsi
140823f6e: 49 8b 87 68 02 00 00        	movq	0x268(%r15), %rax
140823f75: 48 85 c0                    	testq	%rax, %rax
140823f78: 74 1d                       	je	0x140823f97 <.text+0x822f97>
140823f7a: 49 8b 8f 70 02 00 00        	movq	0x270(%r15), %rcx
140823f81: 48 89 c2                    	movq	%rax, %rdx
140823f84: 48 c1 e2 07                 	shlq	$0x7, %rdx
140823f88: 48 8d 14 c2                 	leaq	(%rdx,%rax,8), %rdx
140823f8c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140823f92: e8 89 fc de ff              	callq	0x140613c20 <.text+0x612c20>
140823f97: f3 0f 6f 85 80 02 00 00     	movdqu	0x280(%rbp), %xmm0
140823f9f: 0f 10 8d 90 02 00 00        	movups	0x290(%rbp), %xmm1
140823fa6: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140823faa: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
140823fae: 4c 8b 45 b0                 	movq	-0x50(%rbp), %r8
140823fb2: 49 8b 57 10                 	movq	0x10(%r15), %rdx
140823fb6: 4c 39 c2                    	cmpq	%r8, %rdx
140823fb9: 72 07                       	jb	0x140823fc2 <.text+0x822fc2>
140823fbb: 4d 89 47 10                 	movq	%r8, 0x10(%r15)
140823fbf: 4c 89 c2                    	movq	%r8, %rdx
140823fc2: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
140823fc6: 48 85 d2                    	testq	%rdx, %rdx
140823fc9: 0f 84 a0 00 00 00           	je	0x14082406f <.text+0x82306f>
140823fcf: 49 8b 4f 08                 	movq	0x8(%r15), %rcx
140823fd3: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140823fd9: 4c 8d 15 a8 d2 26 02        	leaq	0x226d2a8(%rip), %r10   # 0x142a91288
140823fe0: 49 89 d3                    	movq	%rdx, %r11
140823fe3: eb 38                       	jmp	0x14082401d <.text+0x82301d>
140823fe5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140823ff0: be 05 00 00 00              	movl	$0x5, %esi
140823ff5: 4a 89 74 09 f8              	movq	%rsi, -0x8(%rcx,%r9)
140823ffa: 66 0f 6f 85 80 02 00 00     	movdqa	0x280(%rbp), %xmm0
140824002: f3 42 0f 7f 04 09           	movdqu	%xmm0, (%rcx,%r9)
140824008: 48 8b b5 90 02 00 00        	movq	0x290(%rbp), %rsi
14082400f: 4a 89 74 09 10              	movq	%rsi, 0x10(%rcx,%r9)
140824014: 49 83 c1 20                 	addq	$0x20, %r9
140824018: 49 ff cb                    	decq	%r11
14082401b: 74 52                       	je	0x14082406f <.text+0x82306f>
14082401d: 4a 8b 7c 08 f8              	movq	-0x8(%rax,%r9), %rdi
140824022: 48 8d 5f fd                 	leaq	-0x3(%rdi), %rbx
140824026: 48 83 fb 03                 	cmpq	$0x3, %rbx
14082402a: be 03 00 00 00              	movl	$0x3, %esi
14082402f: 48 0f 43 de                 	cmovaeq	%rsi, %rbx
140824033: 49 63 1c 9a                 	movslq	(%r10,%rbx,4), %rbx
140824037: 4c 01 d3                    	addq	%r10, %rbx
14082403a: ff e3                       	jmpq	*%rbx
14082403c: be 04 00 00 00              	movl	$0x4, %esi
140824041: eb b2                       	jmp	0x140823ff5 <.text+0x822ff5>
140824043: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140824050: 4a 8b 74 08 10              	movq	0x10(%rax,%r9), %rsi
140824055: 48 89 b5 90 02 00 00        	movq	%rsi, 0x290(%rbp)
14082405c: f3 42 0f 6f 04 08           	movdqu	(%rax,%r9), %xmm0
140824062: 66 0f 7f 85 80 02 00 00     	movdqa	%xmm0, 0x280(%rbp)
14082406a: 48 89 fe                    	movq	%rdi, %rsi
14082406d: eb 86                       	jmp	0x140823ff5 <.text+0x822ff5>
14082406f: 48 c1 e2 05                 	shlq	$0x5, %rdx
140824073: 48 01 c2                    	addq	%rax, %rdx
140824076: 49 c1 e0 05                 	shlq	$0x5, %r8
14082407a: 49 01 c0                    	addq	%rax, %r8
14082407d: 4c 8d 0d fc 7e 40 02        	leaq	0x2407efc(%rip), %r9    # 0x142c2bf80
140824084: 4c 89 f9                    	movq	%r15, %rcx
140824087: e8 84 a6 a3 00              	callq	0x14125e710 <.text+0x125d710>
14082408c: 0f b6 85 22 02 00 00        	movzbl	0x222(%rbp), %eax
140824093: 41 88 87 82 02 00 00        	movb	%al, 0x282(%r15)
14082409a: 0f b6 85 20 02 00 00        	movzbl	0x220(%rbp), %eax
1408240a1: 41 88 87 80 02 00 00        	movb	%al, 0x280(%r15)
1408240a8: 0f b6 85 21 02 00 00        	movzbl	0x221(%rbp), %eax
1408240af: 41 88 87 81 02 00 00        	movb	%al, 0x281(%r15)
1408240b6: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1408240ba: e8 81 30 dd ff              	callq	0x1405f7140 <.text+0x5f6140>
1408240bf: 90                          	nop
1408240c0: 48 81 c4 a8 03 00 00        	addq	$0x3a8, %rsp            # imm = 0x3A8
1408240c7: 5b                          	popq	%rbx
1408240c8: 5f                          	popq	%rdi
1408240c9: 5e                          	popq	%rsi
1408240ca: 41 5c                       	popq	%r12
1408240cc: 41 5d                       	popq	%r13
1408240ce: 41 5e                       	popq	%r14
1408240d0: 41 5f                       	popq	%r15
1408240d2: 5d                          	popq	%rbp
1408240d3: c3                          	retq
1408240d4: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1408240d8: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
1408240df: e8 1c 7a 30 00              	callq	0x140b2bb00 <.text+0xb2ab00>
1408240e4: 0f b6 45 a0                 	movzbl	-0x60(%rbp), %eax
1408240e8: 88 85 1f 03 00 00           	movb	%al, 0x31f(%rbp)
1408240ee: 48 8b 55 a8                 	movq	-0x58(%rbp), %rdx
1408240f2: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
1408240f6: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
1408240fd: 48 8b 5d b8                 	movq	-0x48(%rbp), %rbx
140824101: 48 8b 7d c0                 	movq	-0x40(%rbp), %rdi
140824105: 4c 8b 75 c8                 	movq	-0x38(%rbp), %r14
140824109: 0f b7 45 d8                 	movzwl	-0x28(%rbp), %eax
14082410d: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140824114: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140824118: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
14082411f: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
140824126: 48 83 c0 18                 	addq	$0x18, %rax
14082412a: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
140824131: eb 17                       	jmp	0x14082414a <.text+0x82314a>
140824133: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140824140: 4c 89 eb                    	movq	%r13, %rbx
140824143: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
14082414a: 48 83 bd 08 03 00 00 02     	cmpq	$0x2, 0x308(%rbp)
140824152: 0f 84 0f 01 00 00           	je	0x140824267 <.text+0x823267>
140824158: 48 85 ff                    	testq	%rdi, %rdi
14082415b: 0f 84 a4 00 00 00           	je	0x140824205 <.text+0x823205>
140824161: 48 83 bd b8 02 00 00 00     	cmpq	$0x0, 0x2b8(%rbp)
140824169: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
140824170: 0f 84 d4 00 00 00           	je	0x14082424a <.text+0x82324a>
140824176: 66 85 c9                    	testw	%cx, %cx
140824179: 74 15                       	je	0x140824190 <.text+0x823190>
14082417b: 48 89 d6                    	movq	%rdx, %rsi
14082417e: 44 8d 79 ff                 	leal	-0x1(%rcx), %r15d
140824182: 41 21 cf                    	andl	%ecx, %r15d
140824185: eb 37                       	jmp	0x1408241be <.text+0x8231be>
140824187: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140824190: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
140824195: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
140824199: 48 81 c7 80 f6 ff ff        	addq	$-0x980, %rdi           # imm = 0xF680
1408241a0: 49 83 c6 10                 	addq	$0x10, %r14
1408241a4: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
1408241aa: 74 e4                       	je	0x140824190 <.text+0x823190>
1408241ac: 48 89 d6                    	movq	%rdx, %rsi
1408241af: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1408241b4: 29 c8                       	subl	%ecx, %eax
1408241b6: f7 d1                       	notl	%ecx
1408241b8: 41 89 cf                    	movl	%ecx, %r15d
1408241bb: 41 21 c7                    	andl	%eax, %r15d
1408241be: 48 ff 8d b8 02 00 00        	decq	0x2b8(%rbp)
1408241c5: f3 0f bc c1                 	tzcntl	%ecx, %eax
1408241c9: 48 f7 d8                    	negq	%rax
1408241cc: 4c 69 e8 98 00 00 00        	imulq	$0x98, %rax, %r13
1408241d3: 4a 8d 0c 2f                 	leaq	(%rdi,%r13), %rcx
1408241d7: 48 81 c1 68 ff ff ff        	addq	$-0x98, %rcx
1408241de: e8 bd 80 1f 00              	callq	0x140a1c2a0 <.text+0xa1b2a0>
1408241e3: 48 83 f8 03                 	cmpq	$0x3, %rax
1408241e7: 0f 85 1f 02 00 00           	jne	0x14082440c <.text+0x82340c>
1408241ed: 44 89 f9                    	movl	%r15d, %ecx
1408241f0: 48 89 f2                    	movq	%rsi, %rdx
1408241f3: 48 89 8d e8 02 00 00        	movq	%rcx, 0x2e8(%rbp)
1408241fa: f6 85 08 03 00 00 01        	testb	$0x1, 0x308(%rbp)
140824201: 75 19                       	jne	0x14082421c <.text+0x82321c>
140824203: eb 60                       	jmp	0x140824265 <.text+0x823265>
140824205: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
14082420c: 48 89 8d e8 02 00 00        	movq	%rcx, 0x2e8(%rbp)
140824213: f6 85 08 03 00 00 01        	testb	$0x1, 0x308(%rbp)
14082421a: 74 49                       	je	0x140824265 <.text+0x823265>
14082421c: 41 bc 01 00 00 00           	movl	$0x1, %r12d
140824222: bf 00 00 00 00              	movl	$0x0, %edi
140824227: b9 00 00 00 00              	movl	$0x0, %ecx
14082422c: 48 85 db                    	testq	%rbx, %rbx
14082422f: 74 39                       	je	0x14082426a <.text+0x82326a>
140824231: 31 ff                       	xorl	%edi, %edi
140824233: 48 c7 85 10 03 00 00 00 00 00 00    	movq	$0x0, 0x310(%rbp)
14082423e: 48 89 95 00 03 00 00        	movq	%rdx, 0x300(%rbp)
140824245: 48 89 de                    	movq	%rbx, %rsi
140824248: eb 59                       	jmp	0x1408242a3 <.text+0x8232a3>
14082424a: 48 c7 85 b8 02 00 00 00 00 00 00    	movq	$0x0, 0x2b8(%rbp)
140824255: 48 89 8d e8 02 00 00        	movq	%rcx, 0x2e8(%rbp)
14082425c: f6 85 08 03 00 00 01        	testb	$0x1, 0x308(%rbp)
140824263: 75 b7                       	jne	0x14082421c <.text+0x82321c>
140824265: 31 ff                       	xorl	%edi, %edi
140824267: 48 89 d9                    	movq	%rbx, %rcx
14082426a: 48 85 d2                    	testq	%rdx, %rdx
14082426d: 0f 95 c0                    	setne	%al
140824270: 84 85 1f 03 00 00           	testb	%al, 0x31f(%rbp)
140824276: 0f 84 be 01 00 00           	je	0x14082443a <.text+0x82343a>
14082427c: 48 89 8d 10 03 00 00        	movq	%rcx, 0x310(%rbp)
140824283: 48 89 d6                    	movq	%rdx, %rsi
140824286: b8 02 00 00 00              	movl	$0x2, %eax
14082428b: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
140824292: 48 c7 85 00 03 00 00 00 00 00 00    	movq	$0x0, 0x300(%rbp)
14082429d: 41 bc 02 00 00 00           	movl	$0x2, %r12d
1408242a3: 48 8b 9d b0 02 00 00        	movq	0x2b0(%rbp), %rbx
1408242aa: 8b 85 d0 02 00 00           	movl	0x2d0(%rbp), %eax
1408242b0: 48 8b 8d e0 02 00 00        	movq	0x2e0(%rbp), %rcx
1408242b7: 89 01                       	movl	%eax, (%rcx)
1408242b9: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
1408242c0: 4c 89 e2                    	movq	%r12, %rdx
1408242c3: 49 89 d8                    	movq	%rbx, %r8
1408242c6: e8 15 8b 1f 00              	callq	0x140a1cde0 <.text+0xa1bde0>
1408242cb: 48 85 c0                    	testq	%rax, %rax
1408242ce: 0f 84 56 03 00 00           	je	0x14082462a <.text+0x82362a>
1408242d4: 49 89 c7                    	movq	%rax, %r15
1408242d7: 49 89 d5                    	movq	%rdx, %r13
1408242da: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1408242e1: 4c 89 e2                    	movq	%r12, %rdx
1408242e4: 49 89 d8                    	movq	%rbx, %r8
1408242e7: e8 d4 7f 1f 00              	callq	0x140a1c2c0 <.text+0xa1b2c0>
1408242ec: 48 85 c0                    	testq	%rax, %rax
1408242ef: 48 0f 45 f0                 	cmovneq	%rax, %rsi
1408242f3: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
1408242fa: 4c 89 ea                    	movq	%r13, %rdx
1408242fd: 4d 89 e0                    	movq	%r12, %r8
140824300: 49 89 cc                    	movq	%rcx, %r12
140824303: 48 89 9d b0 02 00 00        	movq	%rbx, 0x2b0(%rbp)
14082430a: 49 89 d9                    	movq	%rbx, %r9
14082430d: e8 1e 71 1f 00              	callq	0x140a1b430 <.text+0xa1a430>
140824312: 4c 89 f9                    	movq	%r15, %rcx
140824315: 48 89 f2                    	movq	%rsi, %rdx
140824318: 4d 89 e0                    	movq	%r12, %r8
14082431b: e8 20 2b a3 00              	callq	0x141256e40 <.text+0x1255e40>
140824320: 4c 8b ad 10 03 00 00        	movq	0x310(%rbp), %r13
140824327: 4c 89 eb                    	movq	%r13, %rbx
14082432a: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
140824331: 80 bd 1e 03 00 00 00        	cmpb	$0x0, 0x31e(%rbp)
140824338: 48 8d b5 38 02 00 00        	leaq	0x238(%rbp), %rsi
14082433f: 0f 84 05 fe ff ff           	je	0x14082414a <.text+0x82314a>
140824345: 49 8b 47 30                 	movq	0x30(%r15), %rax
140824349: 49 8b 4f 38                 	movq	0x38(%r15), %rcx
14082434d: 48 01 c1                    	addq	%rax, %rcx
140824350: 48 ff c1                    	incq	%rcx
140824353: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
140824357: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
14082435b: f7 d2                       	notl	%edx
14082435d: 4d 8b 47 48                 	movq	0x48(%r15), %r8
140824361: 49 83 c7 60                 	addq	$0x60, %r15
140824365: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
14082436c: 48 83 c0 10                 	addq	$0x10, %rax
140824370: 48 89 85 88 02 00 00        	movq	%rax, 0x288(%rbp)
140824377: 48 89 8d 90 02 00 00        	movq	%rcx, 0x290(%rbp)
14082437e: 66 89 95 98 02 00 00        	movw	%dx, 0x298(%rbp)
140824385: 4c 89 85 a0 02 00 00        	movq	%r8, 0x2a0(%rbp)
14082438c: 4c 89 bd a8 02 00 00        	movq	%r15, 0x2a8(%rbp)
140824393: 48 8d 9d 30 02 00 00        	leaq	0x230(%rbp), %rbx
14082439a: 48 89 d9                    	movq	%rbx, %rcx
14082439d: 4c 89 e2                    	movq	%r12, %rdx
1408243a0: e8 5b 6e 1f 00              	callq	0x140a1b200 <.text+0xa1a200>
1408243a5: 48 83 bd 38 02 00 00 00     	cmpq	$0x0, 0x238(%rbp)
1408243ad: 4c 8d bd 50 02 00 00        	leaq	0x250(%rbp), %r15
1408243b4: 75 23                       	jne	0x1408243d9 <.text+0x8233d9>
1408243b6: e9 85 fd ff ff              	jmp	0x140824140 <.text+0x823140>
1408243bb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1408243c0: 48 89 d9                    	movq	%rbx, %rcx
1408243c3: 4c 89 e2                    	movq	%r12, %rdx
1408243c6: e8 35 6e 1f 00              	callq	0x140a1b200 <.text+0xa1a200>
1408243cb: 48 83 bd 38 02 00 00 00     	cmpq	$0x0, 0x238(%rbp)
1408243d3: 0f 84 67 fd ff ff           	je	0x140824140 <.text+0x823140>
1408243d9: 48 8b 46 10                 	movq	0x10(%rsi), %rax
1408243dd: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1408243e4: f3 0f 6f 06                 	movdqu	(%rsi), %xmm0
1408243e8: 66 0f 7f 85 50 02 00 00     	movdqa	%xmm0, 0x250(%rbp)
1408243f0: 4c 89 f9                    	movq	%r15, %rcx
1408243f3: e8 d8 41 86 ff              	callq	0x1400885d0 <.text+0x875d0>
1408243f8: 80 78 47 01                 	cmpb	$0x1, 0x47(%rax)
1408243fc: 75 c2                       	jne	0x1408243c0 <.text+0x8233c0>
1408243fe: 4c 89 f9                    	movq	%r15, %rcx
140824401: e8 da 6d 1f 00              	callq	0x140a1b1e0 <.text+0xa1a1e0>
140824406: c6 40 47 00                 	movb	$0x0, 0x47(%rax)
14082440a: eb b4                       	jmp	0x1408243c0 <.text+0x8233c0>
14082440c: 49 89 c4                    	movq	%rax, %r12
14082440f: 48 89 b5 00 03 00 00        	movq	%rsi, 0x300(%rbp)
140824416: 4a 8d 34 2f                 	leaq	(%rdi,%r13), %rsi
14082441a: 48 81 c6 70 ff ff ff        	addq	$-0x90, %rsi
140824421: 44 89 f9                    	movl	%r15d, %ecx
140824424: 48 89 8d e8 02 00 00        	movq	%rcx, 0x2e8(%rbp)
14082442b: 48 89 9d 10 03 00 00        	movq	%rbx, 0x310(%rbp)
140824432: 48 89 d3                    	movq	%rdx, %rbx
140824435: e9 70 fe ff ff              	jmp	0x1408242aa <.text+0x8232aa>
14082443a: 48 8b 95 c8 02 00 00        	movq	0x2c8(%rbp), %rdx
140824441: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140824444: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140824448: 89 01                       	movl	%eax, (%rcx)
14082444a: 48 8b 32                    	movq	(%rdx), %rsi
14082444d: 4c 8b bd f0 02 00 00        	movq	0x2f0(%rbp), %r15
140824454: 49 8d 97 30 02 00 00        	leaq	0x230(%r15), %rdx
14082445b: 48 89 f1                    	movq	%rsi, %rcx
14082445e: e8 4d eb a3 00              	callq	0x141262fb0 <.text+0x1261fb0>
140824463: 41 0f 10 87 50 02 00 00     	movups	0x250(%r15), %xmm0
14082446b: 0f 11 46 20                 	movups	%xmm0, 0x20(%rsi)
14082446f: 48 8b 95 c0 02 00 00        	movq	0x2c0(%rbp), %rdx
140824476: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140824479: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14082447d: 89 01                       	movl	%eax, (%rcx)
14082447f: 48 8b 32                    	movq	(%rdx), %rsi
140824482: 49 8d 97 d0 01 00 00        	leaq	0x1d0(%r15), %rdx
140824489: 48 89 f1                    	movq	%rsi, %rcx
14082448c: e8 7f ed a3 00              	callq	0x141263210 <.text+0x1262210>
140824491: 41 0f 10 87 f0 01 00 00     	movups	0x1f0(%r15), %xmm0
140824499: 0f 11 46 20                 	movups	%xmm0, 0x20(%rsi)
14082449d: 48 8b 95 90 03 00 00        	movq	0x390(%rbp), %rdx
1408244a4: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1408244a7: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1408244ab: 89 01                       	movl	%eax, (%rcx)
1408244ad: 48 8b 32                    	movq	(%rdx), %rsi
1408244b0: 49 8d 97 00 02 00 00        	leaq	0x200(%r15), %rdx
1408244b7: 48 89 f1                    	movq	%rsi, %rcx
1408244ba: e8 41 e6 a3 00              	callq	0x141262b00 <.text+0x1261b00>
1408244bf: 41 0f 10 87 20 02 00 00     	movups	0x220(%r15), %xmm0
1408244c7: 0f 11 46 20                 	movups	%xmm0, 0x20(%rsi)
1408244cb: 48 8b 95 98 03 00 00        	movq	0x398(%rbp), %rdx
1408244d2: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1408244d5: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1408244d9: 89 01                       	movl	%eax, (%rcx)
1408244db: 48 8b 32                    	movq	(%rdx), %rsi
1408244de: 49 8d 97 60 02 00 00        	leaq	0x260(%r15), %rdx
1408244e5: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1408244e9: e8 b2 f0 00 00              	callq	0x1408335a0 <.text+0x8325a0>
1408244ee: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1408244f2: 48 85 c0                    	testq	%rax, %rax
1408244f5: 74 1a                       	je	0x140824511 <.text+0x823511>
1408244f7: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
1408244fb: 48 89 c2                    	movq	%rax, %rdx
1408244fe: 48 c1 e2 07                 	shlq	$0x7, %rdx
140824502: 48 8d 14 c2                 	leaq	(%rdx,%rax,8), %rdx
140824506: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14082450c: e8 0f f7 de ff              	callq	0x140613c20 <.text+0x612c20>
140824511: f3 0f 6f 45 a0              	movdqu	-0x60(%rbp), %xmm0
140824516: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
14082451a: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
14082451e: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
140824522: 48 8b 8d a0 03 00 00        	movq	0x3a0(%rbp), %rcx
140824529: 8b 41 1c                    	movl	0x1c(%rcx), %eax
14082452c: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140824530: 89 01                       	movl	%eax, (%rcx)
140824532: 49 8b 47 08                 	movq	0x8(%r15), %rax
140824536: 4d 8b 47 10                 	movq	0x10(%r15), %r8
14082453a: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
140824541: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
140824545: 4c 39 c2                    	cmpq	%r8, %rdx
140824548: 72 07                       	jb	0x140824551 <.text+0x823551>
14082454a: 4c 89 41 10                 	movq	%r8, 0x10(%rcx)
14082454e: 4c 89 c2                    	movq	%r8, %rdx
140824551: 48 85 d2                    	testq	%rdx, %rdx
140824554: 0f 84 7f 00 00 00           	je	0x1408245d9 <.text+0x8235d9>
14082455a: 4c 8b 71 08                 	movq	0x8(%rcx), %r14
14082455e: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140824564: 4c 8d 15 fd cc 26 02        	leaq	0x226ccfd(%rip), %r10   # 0x142a91268
14082456b: 49 89 d3                    	movq	%rdx, %r11
14082456e: eb 27                       	jmp	0x140824597 <.text+0x823597>
140824570: be 05 00 00 00              	movl	$0x5, %esi
140824575: 4b 89 74 0e f8              	movq	%rsi, -0x8(%r14,%r9)
14082457a: 66 0f 6f 45 a0              	movdqa	-0x60(%rbp), %xmm0
14082457f: f3 43 0f 7f 04 0e           	movdqu	%xmm0, (%r14,%r9)
140824585: 48 8b 75 b0                 	movq	-0x50(%rbp), %rsi
140824589: 4b 89 74 0e 10              	movq	%rsi, 0x10(%r14,%r9)
14082458e: 49 83 c1 20                 	addq	$0x20, %r9
140824592: 49 ff cb                    	decq	%r11
140824595: 74 42                       	je	0x1408245d9 <.text+0x8235d9>
140824597: 4a 8b 7c 08 f8              	movq	-0x8(%rax,%r9), %rdi
14082459c: 48 8d 5f fd                 	leaq	-0x3(%rdi), %rbx
1408245a0: 48 83 fb 03                 	cmpq	$0x3, %rbx
1408245a4: be 03 00 00 00              	movl	$0x3, %esi
1408245a9: 48 0f 43 de                 	cmovaeq	%rsi, %rbx
1408245ad: 49 63 1c 9a                 	movslq	(%r10,%rbx,4), %rbx
1408245b1: 4c 01 d3                    	addq	%r10, %rbx
1408245b4: ff e3                       	jmpq	*%rbx
1408245b6: be 04 00 00 00              	movl	$0x4, %esi
1408245bb: eb b8                       	jmp	0x140824575 <.text+0x823575>
1408245bd: 0f 1f 00                    	nopl	(%rax)
1408245c0: 4a 8b 74 08 10              	movq	0x10(%rax,%r9), %rsi
1408245c5: 48 89 75 b0                 	movq	%rsi, -0x50(%rbp)
1408245c9: f3 42 0f 6f 04 08           	movdqu	(%rax,%r9), %xmm0
1408245cf: 66 0f 7f 45 a0              	movdqa	%xmm0, -0x60(%rbp)
1408245d4: 48 89 fe                    	movq	%rdi, %rsi
1408245d7: eb 9c                       	jmp	0x140824575 <.text+0x823575>
1408245d9: 48 c1 e2 05                 	shlq	$0x5, %rdx
1408245dd: 48 01 c2                    	addq	%rax, %rdx
1408245e0: 49 c1 e0 05                 	shlq	$0x5, %r8
1408245e4: 49 01 c0                    	addq	%rax, %r8
1408245e7: 4c 8d 0d 92 79 40 02        	leaq	0x2407992(%rip), %r9    # 0x142c2bf80
1408245ee: e8 1d a1 a3 00              	callq	0x14125e710 <.text+0x125d710>
1408245f3: 48 8b 95 b0 03 00 00        	movq	0x3b0(%rbp), %rdx
1408245fa: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1408245fd: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140824601: 89 01                       	movl	%eax, (%rcx)
140824603: 48 8b 0a                    	movq	(%rdx), %rcx
140824606: e8 05 ac 0d 00              	callq	0x1408ff210 <.text+0x8fe210>
14082460b: 41 80 bf 80 02 00 00 00     	cmpb	$0x0, 0x280(%r15)
140824613: 0f 85 aa f8 ff ff           	jne	0x140823ec3 <.text+0x822ec3>
140824619: e9 a1 fa ff ff              	jmp	0x1408240bf <.text+0x8230bf>
14082461e: 48 8d 0d 03 cc 26 02        	leaq	0x226cc03(%rip), %rcx   # 0x142a91228
140824625: e8 e6 4c 0b 02              	callq	0x1428d9310 <.text+0x28d8310>
14082462a: 48 8d 0d 0f cc 26 02        	leaq	0x226cc0f(%rip), %rcx   # 0x142a91240
140824631: e8 da 4c 0b 02              	callq	0x1428d9310 <.text+0x28d8310>
140824636: cc                          	int3
140824637: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140824640: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140824645: 55                          	pushq	%rbp
140824646: 41 57                       	pushq	%r15
140824648: 41 56                       	pushq	%r14
14082464a: 41 55                       	pushq	%r13
14082464c: 41 54                       	pushq	%r12
14082464e: 56                          	pushq	%rsi
14082464f: 57                          	pushq	%rdi
140824650: 53                          	pushq	%rbx
140824651: 48 83 ec 28                 	subq	$0x28, %rsp
140824655: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14082465c: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140824660: e8 db 2a dd ff              	callq	0x1405f7140 <.text+0x5f6140>
140824665: 90                          	nop
140824666: 48 83 c4 28                 	addq	$0x28, %rsp
14082466a: 5b                          	popq	%rbx
14082466b: 5f                          	popq	%rdi
14082466c: 5e                          	popq	%rsi
14082466d: 41 5c                       	popq	%r12
14082466f: 41 5d                       	popq	%r13
140824671: 41 5e                       	popq	%r14
140824673: 41 5f                       	popq	%r15
140824675: 5d                          	popq	%rbp
140824676: c3                          	retq
140824677: cc                          	int3
140824678: cc                          	int3
140824679: cc                          	int3
14082467a: cc                          	int3
14082467b: cc                          	int3
14082467c: cc                          	int3
14082467d: cc                          	int3
14082467e: cc                          	int3
14082467f: cc                          	int3
