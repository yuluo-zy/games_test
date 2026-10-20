
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1415237c0: 55                          	pushq	%rbp
1415237c1: 41 57                       	pushq	%r15
1415237c3: 41 56                       	pushq	%r14
1415237c5: 41 55                       	pushq	%r13
1415237c7: 41 54                       	pushq	%r12
1415237c9: 56                          	pushq	%rsi
1415237ca: 57                          	pushq	%rdi
1415237cb: 53                          	pushq	%rbx
1415237cc: 48 81 ec 98 02 00 00        	subq	$0x298, %rsp            # imm = 0x298
1415237d3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1415237db: 66 44 0f 7f 8d 00 02 00 00  	movdqa	%xmm9, 0x200(%rbp)
1415237e4: 44 0f 29 85 f0 01 00 00     	movaps	%xmm8, 0x1f0(%rbp)
1415237ec: 0f 29 bd e0 01 00 00        	movaps	%xmm7, 0x1e0(%rbp)
1415237f3: 0f 29 b5 d0 01 00 00        	movaps	%xmm6, 0x1d0(%rbp)
1415237fa: 48 c7 85 c8 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1c8(%rbp)
141523805: 48 89 8d e8 00 00 00        	movq	%rcx, 0xe8(%rbp)
14152380c: f3 0f 6f 02                 	movdqu	(%rdx), %xmm0
141523810: f3 0f 6f 4a 10              	movdqu	0x10(%rdx), %xmm1
141523815: f3 0f 6f 52 20              	movdqu	0x20(%rdx), %xmm2
14152381a: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
14152381e: 0f 29 9d b0 01 00 00        	movaps	%xmm3, 0x1b0(%rbp)
141523825: 66 0f 7f 95 a0 01 00 00     	movdqa	%xmm2, 0x1a0(%rbp)
14152382d: 66 0f 7f 8d 90 01 00 00     	movdqa	%xmm1, 0x190(%rbp)
141523835: 66 0f 7f 85 80 01 00 00     	movdqa	%xmm0, 0x180(%rbp)
14152383d: 48 8d b5 a0 00 00 00        	leaq	0xa0(%rbp), %rsi
141523844: 66 0f 6f 35 34 98 4a 01     	movdqa	0x14a9834(%rip), %xmm6  # 0x1429cd080
14152384c: 66 0f 6f 3d 3c 98 4a 01     	movdqa	0x14a983c(%rip), %xmm7  # 0x1429cd090
141523854: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
141523858: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
14152385d: eb 18                       	jmp	0x141523877 <.text+0x1522877>
14152385f: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
141523866: 48 29 c1                    	subq	%rax, %rcx
141523869: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14152386f: 4c 89 ca                    	movq	%r9, %rdx
141523872: e8 a9 03 0f ff              	callq	0x140613c20 <.text+0x612c20>
141523877: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
14152387e: 48 85 c9                    	testq	%rcx, %rcx
141523881: 0f 84 6f 08 00 00           	je	0x1415240f6 <.text+0x15230f6>
141523887: 0f b7 95 b0 01 00 00        	movzwl	0x1b0(%rbp), %edx
14152388e: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
141523895: 66 85 d2                    	testw	%dx, %dx
141523898: 74 26                       	je	0x1415238c0 <.text+0x15228c0>
14152389a: 44 8d 42 ff                 	leal	-0x1(%rdx), %r8d
14152389e: 41 21 d0                    	andl	%edx, %r8d
1415238a1: 66 44 89 85 b0 01 00 00     	movw	%r8w, 0x1b0(%rbp)
1415238a9: 48 ff c9                    	decq	%rcx
1415238ac: 48 89 8d b8 01 00 00        	movq	%rcx, 0x1b8(%rbp)
1415238b3: 48 85 c0                    	testq	%rax, %rax
1415238b6: 75 64                       	jne	0x14152391c <.text+0x152291c>
1415238b8: e9 39 08 00 00              	jmp	0x1415240f6 <.text+0x15230f6>
1415238bd: 0f 1f 00                    	nopl	(%rax)
1415238c0: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
1415238c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1415238d0: 66 41 0f 6f 00              	movdqa	(%r8), %xmm0
1415238d5: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1415238d9: 48 05 00 f9 ff ff           	addq	$-0x700, %rax           # imm = 0xF900
1415238df: 49 83 c0 10                 	addq	$0x10, %r8
1415238e3: 81 fa ff ff 00 00           	cmpl	$0xffff, %edx           # imm = 0xFFFF
1415238e9: 74 e5                       	je	0x1415238d0 <.text+0x15228d0>
1415238eb: 4c 89 85 a0 01 00 00        	movq	%r8, 0x1a0(%rbp)
1415238f2: 48 89 85 98 01 00 00        	movq	%rax, 0x198(%rbp)
1415238f9: 41 b8 fe ff ff ff           	movl	$0xfffffffe, %r8d       # imm = 0xFFFFFFFE
1415238ff: 41 29 d0                    	subl	%edx, %r8d
141523902: f7 d2                       	notl	%edx
141523904: 41 89 d1                    	movl	%edx, %r9d
141523907: 45 21 c1                    	andl	%r8d, %r9d
14152390a: 66 44 89 8d b0 01 00 00     	movw	%r9w, 0x1b0(%rbp)
141523912: 48 ff c9                    	decq	%rcx
141523915: 48 89 8d b8 01 00 00        	movq	%rcx, 0x1b8(%rbp)
14152391c: f3 0f bc ca                 	tzcntl	%edx, %ecx
141523920: 48 f7 d9                    	negq	%rcx
141523923: 48 6b c9 70                 	imulq	$0x70, %rcx, %rcx
141523927: 4c 8b 44 08 90              	movq	-0x70(%rax,%rcx), %r8
14152392c: 4c 8b 4c 08 98              	movq	-0x68(%rax,%rcx), %r9
141523931: 0f 10 44 08 c0              	movups	-0x40(%rax,%rcx), %xmm0
141523936: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
14152393a: 0f 10 44 08 b0              	movups	-0x50(%rax,%rcx), %xmm0
14152393f: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
141523943: 0f 10 44 08 a0              	movups	-0x60(%rax,%rcx), %xmm0
141523948: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
14152394c: 0f 10 44 08 d0              	movups	-0x30(%rax,%rcx), %xmm0
141523951: 0f 10 4c 08 e0              	movups	-0x20(%rax,%rcx), %xmm1
141523956: 0f 10 54 08 f0              	movups	-0x10(%rax,%rcx), %xmm2
14152395b: 0f 29 55 60                 	movaps	%xmm2, 0x60(%rbp)
14152395f: 0f 29 4d 50                 	movaps	%xmm1, 0x50(%rbp)
141523963: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
141523967: 49 83 f8 03                 	cmpq	$0x3, %r8
14152396b: 0f 84 85 07 00 00           	je	0x1415240f6 <.text+0x15230f6>
141523971: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
141523975: 0f 28 4d 20                 	movaps	0x20(%rbp), %xmm1
141523979: 0f 28 55 30                 	movaps	0x30(%rbp), %xmm2
14152397d: 0f 29 95 70 01 00 00        	movaps	%xmm2, 0x170(%rbp)
141523984: 0f 29 8d 60 01 00 00        	movaps	%xmm1, 0x160(%rbp)
14152398b: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
141523992: 66 0f 6f 45 40              	movdqa	0x40(%rbp), %xmm0
141523997: 66 0f 6f 4d 50              	movdqa	0x50(%rbp), %xmm1
14152399c: 66 0f 6f 55 60              	movdqa	0x60(%rbp), %xmm2
1415239a1: 66 0f 7f 95 10 01 00 00     	movdqa	%xmm2, 0x110(%rbp)
1415239a9: 66 0f 7f 8d 00 01 00 00     	movdqa	%xmm1, 0x100(%rbp)
1415239b1: 66 0f 7f 85 f0 00 00 00     	movdqa	%xmm0, 0xf0(%rbp)
1415239b9: 4c 8b ad 68 01 00 00        	movq	0x168(%rbp), %r13
1415239c0: 4d 85 ed                    	testq	%r13, %r13
1415239c3: 0f 84 f7 05 00 00           	je	0x141523fc0 <.text+0x1522fc0>
1415239c9: 4c 89 8d e0 00 00 00        	movq	%r9, 0xe0(%rbp)
1415239d0: 4c 89 85 d8 00 00 00        	movq	%r8, 0xd8(%rbp)
1415239d7: 48 8b 9d 50 01 00 00        	movq	0x150(%rbp), %rbx
1415239de: 4c 8d 63 10                 	leaq	0x10(%rbx), %r12
1415239e2: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
1415239e6: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
1415239eb: 41 f7 d7                    	notl	%r15d
1415239ee: e9 24 02 00 00              	jmp	0x141523c17 <.text+0x1522c17>
1415239f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141523a00: 49 21 c0                    	andq	%rax, %r8
141523a03: f3 42 0f 6f 0c 02           	movdqu	(%rdx,%r8), %xmm1
141523a09: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
141523a0d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
141523a11: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
141523a16: 45 85 d2                    	testl	%r10d, %r10d
141523a19: 74 25                       	je	0x141523a40 <.text+0x1522a40>
141523a1b: f3 45 0f bc da              	tzcntl	%r10d, %r11d
141523a20: 4d 01 c3                    	addq	%r8, %r11
141523a23: 49 21 c3                    	andq	%rax, %r11
141523a26: 49 f7 db                    	negq	%r11
141523a29: 4d 6b db 58                 	imulq	$0x58, %r11, %r11
141523a2d: 46 39 34 19                 	cmpl	%r14d, (%rcx,%r11)
141523a31: 74 2d                       	je	0x141523a60 <.text+0x1522a60>
141523a33: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
141523a37: 66 45 21 d3                 	andw	%r10w, %r11w
141523a3b: 45 89 da                    	movl	%r11d, %r10d
141523a3e: 75 db                       	jne	0x141523a1b <.text+0x1522a1b>
141523a40: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
141523a45: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
141523a4a: 45 85 d2                    	testl	%r10d, %r10d
141523a4d: 0f 85 ad 01 00 00           	jne	0x141523c00 <.text+0x1522c00>
141523a53: 4d 01 c8                    	addq	%r9, %r8
141523a56: 49 83 c0 10                 	addq	$0x10, %r8
141523a5a: 49 83 c1 10                 	addq	$0x10, %r9
141523a5e: eb a0                       	jmp	0x141523a00 <.text+0x1522a00>
141523a60: 42 0f b6 44 1a d2           	movzbl	-0x2e(%rdx,%r11), %eax
141523a66: 8d 48 fe                    	leal	-0x2(%rax), %ecx
141523a69: 49 89 c0                    	movq	%rax, %r8
141523a6c: 49 ff c8                    	decq	%r8
141523a6f: 80 f9 03                    	cmpb	$0x3, %cl
141523a72: 41 ba 00 00 00 00           	movl	$0x0, %r10d
141523a78: 4d 0f 43 c2                 	cmovaeq	%r10, %r8
141523a7c: 0f b6 4f d2                 	movzbl	-0x2e(%rdi), %ecx
141523a80: 4c 8d 49 ff                 	leaq	-0x1(%rcx), %r9
141523a84: 48 83 f9 02                 	cmpq	$0x2, %rcx
141523a88: 4d 0f 42 ca                 	cmovbq	%r10, %r9
141523a8c: 4d 39 c8                    	cmpq	%r9, %r8
141523a8f: 0f 85 79 01 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523a95: 4c 01 da                    	addq	%r11, %rdx
141523a98: 4c 8d 0d d1 85 74 01        	leaq	0x17485d1(%rip), %r9    # 0x142c6c070
141523a9f: 4f 63 04 81                 	movslq	(%r9,%r8,4), %r8
141523aa3: 4d 01 c8                    	addq	%r9, %r8
141523aa6: 41 ff e0                    	jmpq	*%r8
141523aa9: f3 0f 10 42 b0              	movss	-0x50(%rdx), %xmm0
141523aae: 0f 2e 47 b0                 	ucomiss	-0x50(%rdi), %xmm0
141523ab2: 0f 85 56 01 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523ab8: 0f 8a 50 01 00 00           	jp	0x141523c0e <.text+0x1522c0e>
141523abe: f3 0f 10 42 b4              	movss	-0x4c(%rdx), %xmm0
141523ac3: 0f 2e 47 b4                 	ucomiss	-0x4c(%rdi), %xmm0
141523ac7: 0f 85 41 01 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523acd: 0f 8a 3b 01 00 00           	jp	0x141523c0e <.text+0x1522c0e>
141523ad3: f3 0f 10 42 b8              	movss	-0x48(%rdx), %xmm0
141523ad8: 0f 2e 47 b8                 	ucomiss	-0x48(%rdi), %xmm0
141523adc: 0f 85 2c 01 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523ae2: 0f 8a 26 01 00 00           	jp	0x141523c0e <.text+0x1522c0e>
141523ae8: 0f b6 42 fa                 	movzbl	-0x6(%rdx), %eax
141523aec: 8d 48 c8                    	leal	-0x38(%rax), %ecx
141523aef: 80 f9 08                    	cmpb	$0x8, %cl
141523af2: 0f b6 c9                    	movzbl	%cl, %ecx
141523af5: 41 ba 06 00 00 00           	movl	$0x6, %r10d
141523afb: 41 0f 43 ca                 	cmovael	%r10d, %ecx
141523aff: 44 0f b6 47 fa              	movzbl	-0x6(%rdi), %r8d
141523b04: 45 8d 48 c8                 	leal	-0x38(%r8), %r9d
141523b08: 41 80 f9 08                 	cmpb	$0x8, %r9b
141523b0c: 45 0f b6 c9                 	movzbl	%r9b, %r9d
141523b10: 45 0f 43 ca                 	cmovael	%r10d, %r9d
141523b14: 44 38 c9                    	cmpb	%r9b, %cl
141523b17: 0f 85 f1 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b1d: 80 f9 06                    	cmpb	$0x6, %cl
141523b20: 0f 94 c1                    	sete	%cl
141523b23: 44 38 c0                    	cmpb	%r8b, %al
141523b26: 0f 95 c0                    	setne	%al
141523b29: 84 c8                       	testb	%cl, %al
141523b2b: 0f 85 dd 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b31: 0f b6 42 f8                 	movzbl	-0x8(%rdx), %eax
141523b35: 3a 47 f8                    	cmpb	-0x8(%rdi), %al
141523b38: 0f 85 d0 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b3e: 8b 42 d8                    	movl	-0x28(%rdx), %eax
141523b41: 3b 47 d8                    	cmpl	-0x28(%rdi), %eax
141523b44: 0f 85 c4 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b4a: 8b 42 dc                    	movl	-0x24(%rdx), %eax
141523b4d: 3b 47 dc                    	cmpl	-0x24(%rdi), %eax
141523b50: 0f 85 b8 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b56: 0f b6 42 f9                 	movzbl	-0x7(%rdx), %eax
141523b5a: 3a 47 f9                    	cmpb	-0x7(%rdi), %al
141523b5d: 0f 85 ab 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b63: f3 0f 10 42 e0              	movss	-0x20(%rdx), %xmm0
141523b68: 0f 2e 47 e0                 	ucomiss	-0x20(%rdi), %xmm0
141523b6c: 0f 85 9c 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b72: 0f 8a 96 00 00 00           	jp	0x141523c0e <.text+0x1522c0e>
141523b78: 0f b6 42 fc                 	movzbl	-0x4(%rdx), %eax
141523b7c: 3a 47 fc                    	cmpb	-0x4(%rdi), %al
141523b7f: 0f 85 89 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b85: 8b 42 e4                    	movl	-0x1c(%rdx), %eax
141523b88: 3b 47 e4                    	cmpl	-0x1c(%rdi), %eax
141523b8b: 0f 85 7d 00 00 00           	jne	0x141523c0e <.text+0x1522c0e>
141523b91: f3 0f 10 42 e8              	movss	-0x18(%rdx), %xmm0
141523b96: 0f 2e 47 e8                 	ucomiss	-0x18(%rdi), %xmm0
141523b9a: 75 72                       	jne	0x141523c0e <.text+0x1522c0e>
141523b9c: 7a 70                       	jp	0x141523c0e <.text+0x1522c0e>
141523b9e: 0f b6 42 fd                 	movzbl	-0x3(%rdx), %eax
141523ba2: 3a 47 fd                    	cmpb	-0x3(%rdi), %al
141523ba5: 75 67                       	jne	0x141523c0e <.text+0x1522c0e>
141523ba7: f3 0f 10 42 ec              	movss	-0x14(%rdx), %xmm0
141523bac: 0f 2e 47 ec                 	ucomiss	-0x14(%rdi), %xmm0
141523bb0: 75 5c                       	jne	0x141523c0e <.text+0x1522c0e>
141523bb2: 7a 5a                       	jp	0x141523c0e <.text+0x1522c0e>
141523bb4: 8b 42 f0                    	movl	-0x10(%rdx), %eax
141523bb7: 3b 47 f0                    	cmpl	-0x10(%rdi), %eax
141523bba: 75 52                       	jne	0x141523c0e <.text+0x1522c0e>
141523bbc: 0f b6 42 fb                 	movzbl	-0x5(%rdx), %eax
141523bc0: 3a 47 fb                    	cmpb	-0x5(%rdi), %al
141523bc3: 75 49                       	jne	0x141523c0e <.text+0x1522c0e>
141523bc5: 0f b6 42 fe                 	movzbl	-0x2(%rdx), %eax
141523bc9: 3a 47 fe                    	cmpb	-0x2(%rdi), %al
141523bcc: 75 40                       	jne	0x141523c0e <.text+0x1522c0e>
141523bce: 8b 42 f4                    	movl	-0xc(%rdx), %eax
141523bd1: 3b 47 f4                    	cmpl	-0xc(%rdi), %eax
141523bd4: 75 38                       	jne	0x141523c0e <.text+0x1522c0e>
141523bd6: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
141523bda: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
141523be1: e8 8a ba 5d ff              	callq	0x140aff670 <.text+0xafe670>
141523be6: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
141523bed: 48 89 fa                    	movq	%rdi, %rdx
141523bf0: e8 eb 65 65 ff              	callq	0x140b7a1e0 <.text+0xb791e0>
141523bf5: eb 17                       	jmp	0x141523c0e <.text+0x1522c0e>
141523bf7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141523c00: 48 83 bd 00 01 00 00 00     	cmpq	$0x0, 0x100(%rbp)
141523c08: 0f 84 ee 02 00 00           	je	0x141523efc <.text+0x1522efc>
141523c0e: 4d 85 ed                    	testq	%r13, %r13
141523c11: 0f 84 09 03 00 00           	je	0x141523f20 <.text+0x1522f20>
141523c17: 66 45 85 ff                 	testw	%r15w, %r15w
141523c1b: 75 25                       	jne	0x141523c42 <.text+0x1522c42>
141523c1d: 0f 1f 00                    	nopl	(%rax)
141523c20: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
141523c26: 66 44 0f d7 f8              	pmovmskb	%xmm0, %r15d
141523c2b: 48 81 c3 80 fa ff ff        	addq	$-0x580, %rbx           # imm = 0xFA80
141523c32: 49 83 c4 10                 	addq	$0x10, %r12
141523c36: 41 81 ff ff ff 00 00        	cmpl	$0xffff, %r15d          # imm = 0xFFFF
141523c3d: 74 e1                       	je	0x141523c20 <.text+0x1522c20>
141523c3f: 41 f7 d7                    	notl	%r15d
141523c42: 41 8d 47 ff                 	leal	-0x1(%r15), %eax
141523c46: f3 41 0f bc cf              	tzcntl	%r15d, %ecx
141523c4b: 44 21 f8                    	andl	%r15d, %eax
141523c4e: 41 89 c7                    	movl	%eax, %r15d
141523c51: 48 f7 d9                    	negq	%rcx
141523c54: 48 6b c1 58                 	imulq	$0x58, %rcx, %rax
141523c58: 48 8d 3c 03                 	leaq	(%rbx,%rax), %rdi
141523c5c: 49 ff cd                    	decq	%r13
141523c5f: 44 8b 74 03 a8              	movl	-0x58(%rbx,%rax), %r14d
141523c64: 66 0f 6f 85 10 01 00 00     	movdqa	0x110(%rbp), %xmm0
141523c6c: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
141523c71: 66 0f ef ce                 	pxor	%xmm6, %xmm1
141523c75: 66 0f 7f 4d 70              	movdqa	%xmm1, 0x70(%rbp)
141523c7a: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
141523c7f: 66 0f ef cf                 	pxor	%xmm7, %xmm1
141523c83: 66 0f 7f 8d 80 00 00 00     	movdqa	%xmm1, 0x80(%rbp)
141523c8b: 66 0f 7f 85 90 00 00 00     	movdqa	%xmm0, 0x90(%rbp)
141523c93: 44 0f 29 06                 	movaps	%xmm8, (%rsi)
141523c97: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
141523c9f: 44 89 b5 c4 01 00 00        	movl	%r14d, 0x1c4(%rbp)
141523ca6: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141523cac: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
141523cb0: 48 8d 95 c4 01 00 00        	leaq	0x1c4(%rbp), %rdx
141523cb7: e8 a4 24 be fe              	callq	0x140106160 <.text+0x105160>
141523cbc: 4c 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %r9
141523cc3: 49 c1 e1 38                 	shlq	$0x38, %r9
141523cc7: 4c 0b 8d a8 00 00 00        	orq	0xa8(%rbp), %r9
141523cce: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
141523cd5: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
141523cdc: 4c 31 c8                    	xorq	%r9, %rax
141523cdf: 4c 8b 45 70                 	movq	0x70(%rbp), %r8
141523ce3: 49 01 c8                    	addq	%rcx, %r8
141523ce6: 48 8b 55 78                 	movq	0x78(%rbp), %rdx
141523cea: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141523cee: 48 01 c2                    	addq	%rax, %rdx
141523cf1: 4c 31 c1                    	xorq	%r8, %rcx
141523cf4: 48 c1 c0 10                 	rolq	$0x10, %rax
141523cf8: 48 31 d0                    	xorq	%rdx, %rax
141523cfb: 49 c1 c0 20                 	rolq	$0x20, %r8
141523cff: 48 01 ca                    	addq	%rcx, %rdx
141523d02: 49 01 c0                    	addq	%rax, %r8
141523d05: 48 c1 c1 11                 	rolq	$0x11, %rcx
141523d09: 48 31 d1                    	xorq	%rdx, %rcx
141523d0c: 48 c1 c0 15                 	rolq	$0x15, %rax
141523d10: 48 c1 c2 20                 	rolq	$0x20, %rdx
141523d14: 4c 31 c0                    	xorq	%r8, %rax
141523d17: 4d 31 c8                    	xorq	%r9, %r8
141523d1a: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
141523d21: 49 01 c8                    	addq	%rcx, %r8
141523d24: 48 01 c2                    	addq	%rax, %rdx
141523d27: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141523d2b: 4c 31 c1                    	xorq	%r8, %rcx
141523d2e: 48 c1 c0 10                 	rolq	$0x10, %rax
141523d32: 49 c1 c0 20                 	rolq	$0x20, %r8
141523d36: 48 31 d0                    	xorq	%rdx, %rax
141523d39: 48 01 ca                    	addq	%rcx, %rdx
141523d3c: 48 c1 c1 11                 	rolq	$0x11, %rcx
141523d40: 49 01 c0                    	addq	%rax, %r8
141523d43: 48 31 d1                    	xorq	%rdx, %rcx
141523d46: 48 c1 c0 15                 	rolq	$0x15, %rax
141523d4a: 4c 31 c0                    	xorq	%r8, %rax
141523d4d: 48 c1 c2 20                 	rolq	$0x20, %rdx
141523d51: 49 01 c8                    	addq	%rcx, %r8
141523d54: 48 01 c2                    	addq	%rax, %rdx
141523d57: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141523d5b: 4c 31 c1                    	xorq	%r8, %rcx
141523d5e: 48 c1 c0 10                 	rolq	$0x10, %rax
141523d62: 49 c1 c0 20                 	rolq	$0x20, %r8
141523d66: 48 31 d0                    	xorq	%rdx, %rax
141523d69: 48 01 ca                    	addq	%rcx, %rdx
141523d6c: 48 c1 c1 11                 	rolq	$0x11, %rcx
141523d70: 49 01 c0                    	addq	%rax, %r8
141523d73: 48 31 d1                    	xorq	%rdx, %rcx
141523d76: 48 c1 c0 15                 	rolq	$0x15, %rax
141523d7a: 4c 31 c0                    	xorq	%r8, %rax
141523d7d: 48 c1 c2 20                 	rolq	$0x20, %rdx
141523d81: 49 01 c8                    	addq	%rcx, %r8
141523d84: 48 01 c2                    	addq	%rax, %rdx
141523d87: 48 c1 c1 0d                 	rolq	$0xd, %rcx
141523d8b: 4c 31 c1                    	xorq	%r8, %rcx
141523d8e: 48 c1 c0 10                 	rolq	$0x10, %rax
141523d92: 48 31 d0                    	xorq	%rdx, %rax
141523d95: 48 01 ca                    	addq	%rcx, %rdx
141523d98: 48 c1 c1 11                 	rolq	$0x11, %rcx
141523d9c: 48 c1 c0 15                 	rolq	$0x15, %rax
141523da0: 49 89 d0                    	movq	%rdx, %r8
141523da3: 49 c1 c0 20                 	rolq	$0x20, %r8
141523da7: 49 31 c8                    	xorq	%rcx, %r8
141523daa: 49 31 c0                    	xorq	%rax, %r8
141523dad: 49 31 d0                    	xorq	%rdx, %r8
141523db0: 4c 89 c1                    	movq	%r8, %rcx
141523db3: 48 c1 e9 39                 	shrq	$0x39, %rcx
141523db7: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
141523dbe: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
141523dc5: 66 0f 6e c1                 	movd	%ecx, %xmm0
141523dc9: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141523dcd: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141523dd2: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141523dd7: 48 8d 4a a8                 	leaq	-0x58(%rdx), %rcx
141523ddb: 45 31 c9                    	xorl	%r9d, %r9d
141523dde: e9 1d fc ff ff              	jmp	0x141523a00 <.text+0x1522a00>
141523de3: f3 0f 10 42 b0              	movss	-0x50(%rdx), %xmm0
141523de8: 0f 2e 47 b0                 	ucomiss	-0x50(%rdi), %xmm0
141523dec: 0f 85 1c fe ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523df2: 0f 8a 16 fe ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523df8: f3 0f 10 42 b4              	movss	-0x4c(%rdx), %xmm0
141523dfd: 0f 2e 47 b4                 	ucomiss	-0x4c(%rdi), %xmm0
141523e01: 0f 85 07 fe ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e07: 0f 8a 01 fe ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523e0d: f3 0f 10 42 c0              	movss	-0x40(%rdx), %xmm0
141523e12: 0f 2e 47 c0                 	ucomiss	-0x40(%rdi), %xmm0
141523e16: 0f 85 f2 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e1c: 0f 8a ec fd ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523e22: f3 0f 10 42 c4              	movss	-0x3c(%rdx), %xmm0
141523e27: 0f 2e 47 c4                 	ucomiss	-0x3c(%rdi), %xmm0
141523e2b: 0f 85 dd fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e31: 0f 8a d7 fd ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523e37: f3 0f 10 42 c8              	movss	-0x38(%rdx), %xmm0
141523e3c: 0f 2e 47 c8                 	ucomiss	-0x38(%rdi), %xmm0
141523e40: 0f 85 c8 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e46: 0f 8a c2 fd ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523e4c: f3 0f 10 42 cc              	movss	-0x34(%rdx), %xmm0
141523e51: 0f 2e 47 cc                 	ucomiss	-0x34(%rdi), %xmm0
141523e55: 0f 85 b3 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e5b: 0f 8a ad fd ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523e61: 44 0f b6 42 d0              	movzbl	-0x30(%rdx), %r8d
141523e66: 44 3a 47 d0                 	cmpb	-0x30(%rdi), %r8b
141523e6a: 0f 85 9e fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e70: 4c 8b 42 b8                 	movq	-0x48(%rdx), %r8
141523e74: 4c 3b 47 b8                 	cmpq	-0x48(%rdi), %r8
141523e78: 0f 85 90 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e7e: 38 c8                       	cmpb	%cl, %al
141523e80: 0f 85 88 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523e86: 0f b6 47 d1                 	movzbl	-0x2f(%rdi), %eax
141523e8a: 38 42 d1                    	cmpb	%al, -0x2f(%rdx)
141523e8d: 0f 84 55 fc ff ff           	je	0x141523ae8 <.text+0x1522ae8>
141523e93: e9 76 fd ff ff              	jmp	0x141523c0e <.text+0x1522c0e>
141523e98: f3 0f 7e 42 b0              	movq	-0x50(%rdx), %xmm0
141523e9d: f3 0f 7e 4f b0              	movq	-0x50(%rdi), %xmm1
141523ea2: 66 0f 76 c8                 	pcmpeqd	%xmm0, %xmm1
141523ea6: 66 0f 70 c1 50              	pshufd	$0x50, %xmm1, %xmm0     # xmm0 = xmm1[0,0,1,1]
141523eab: 66 0f 50 c0                 	movmskpd	%xmm0, %eax
141523eaf: 83 f8 03                    	cmpl	$0x3, %eax
141523eb2: 0f 85 56 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523eb8: f3 0f 10 42 b8              	movss	-0x48(%rdx), %xmm0
141523ebd: 0f 2e 47 b8                 	ucomiss	-0x48(%rdi), %xmm0
141523ec1: 0f 85 47 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523ec7: 0f 8a 41 fd ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523ecd: f3 0f 10 42 bc              	movss	-0x44(%rdx), %xmm0
141523ed2: 0f 2e 47 bc                 	ucomiss	-0x44(%rdi), %xmm0
141523ed6: 0f 85 32 fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523edc: 0f 8a 2c fd ff ff           	jp	0x141523c0e <.text+0x1522c0e>
141523ee2: f3 0f 10 42 c0              	movss	-0x40(%rdx), %xmm0
141523ee7: 0f 2e 47 c0                 	ucomiss	-0x40(%rdi), %xmm0
141523eeb: 0f 85 1d fd ff ff           	jne	0x141523c0e <.text+0x1522c0e>
141523ef1: 0f 8b f1 fb ff ff           	jnp	0x141523ae8 <.text+0x1522ae8>
141523ef7: e9 12 fd ff ff              	jmp	0x141523c0e <.text+0x1522c0e>
141523efc: ba 01 00 00 00              	movl	$0x1, %edx
141523f01: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
141523f08: 4c 8d 85 10 01 00 00        	leaq	0x110(%rbp), %r8
141523f0f: 41 b1 01                    	movb	$0x1, %r9b
141523f12: e8 c9 92 38 01              	callq	0x1428ad1e0 <.text+0x28ac1e0>
141523f17: e9 f2 fc ff ff              	jmp	0x141523c0e <.text+0x1522c0e>
141523f1c: 0f 1f 40 00                 	nopl	(%rax)
141523f20: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
141523f27: 48 0b 85 68 01 00 00        	orq	0x168(%rbp), %rax
141523f2e: 4c 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %r8
141523f35: 4c 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %r9
141523f3c: 0f 85 8c 00 00 00           	jne	0x141523fce <.text+0x1522fce>
141523f42: 4c 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %r9
141523f49: 4d 85 c9                    	testq	%r9, %r9
141523f4c: 74 35                       	je	0x141523f83 <.text+0x1522f83>
141523f4e: 49 8d 41 01                 	leaq	0x1(%r9), %rax
141523f52: b9 58 00 00 00              	movl	$0x58, %ecx
141523f57: 48 f7 e1                    	mulq	%rcx
141523f5a: 48 83 c0 0f                 	addq	$0xf, %rax
141523f5e: 48 83 e0 f0                 	andq	$-0x10, %rax
141523f62: 49 01 c1                    	addq	%rax, %r9
141523f65: 49 83 c1 11                 	addq	$0x11, %r9
141523f69: 74 18                       	je	0x141523f83 <.text+0x1522f83>
141523f6b: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
141523f72: 48 29 c1                    	subq	%rax, %rcx
141523f75: 41 b8 10 00 00 00           	movl	$0x10, %r8d
141523f7b: 4c 89 ca                    	movq	%r9, %rdx
141523f7e: e8 9d fc 0e ff              	callq	0x140613c20 <.text+0x612c20>
141523f83: 4c 8b 8d 58 01 00 00        	movq	0x158(%rbp), %r9
141523f8a: 4d 85 c9                    	testq	%r9, %r9
141523f8d: 0f 84 e4 f8 ff ff           	je	0x141523877 <.text+0x1522877>
141523f93: 49 8d 41 01                 	leaq	0x1(%r9), %rax
141523f97: b9 58 00 00 00              	movl	$0x58, %ecx
141523f9c: 48 f7 e1                    	mulq	%rcx
141523f9f: 48 83 c0 0f                 	addq	$0xf, %rax
141523fa3: 48 83 e0 f0                 	andq	$-0x10, %rax
141523fa7: 49 01 c1                    	addq	%rax, %r9
141523faa: 49 83 c1 11                 	addq	$0x11, %r9
141523fae: 0f 85 ab f8 ff ff           	jne	0x14152385f <.text+0x152285f>
141523fb4: e9 be f8 ff ff              	jmp	0x141523877 <.text+0x1522877>
141523fb9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
141523fc0: 48 83 bd 08 01 00 00 00     	cmpq	$0x0, 0x108(%rbp)
141523fc8: 0f 84 74 ff ff ff           	je	0x141523f42 <.text+0x1522f42>
141523fce: 0f 28 85 50 01 00 00        	movaps	0x150(%rbp), %xmm0
141523fd5: 0f 28 8d 60 01 00 00        	movaps	0x160(%rbp), %xmm1
141523fdc: 0f 28 95 70 01 00 00        	movaps	0x170(%rbp), %xmm2
141523fe3: 0f 29 55 d0                 	movaps	%xmm2, -0x30(%rbp)
141523fe7: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
141523feb: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
141523fef: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
141523ff6: 0f 28 8d 00 01 00 00        	movaps	0x100(%rbp), %xmm1
141523ffd: 0f 28 95 10 01 00 00        	movaps	0x110(%rbp), %xmm2
141524004: 48 8d 45 e0                 	leaq	-0x20(%rbp), %rax
141524008: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14152400c: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
141524010: 0f 11 00                    	movups	%xmm0, (%rax)
141524013: 0f 28 45 00                 	movaps	(%rbp), %xmm0
141524017: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
14152401e: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
141524022: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
141524029: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
14152402d: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
141524032: 66 0f 6f 55 d0              	movdqa	-0x30(%rbp), %xmm2
141524037: 0f 28 5d e0                 	movaps	-0x20(%rbp), %xmm3
14152403b: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
141524042: 66 0f 7f 95 10 01 00 00     	movdqa	%xmm2, 0x110(%rbp)
14152404a: 66 0f 7f 8d 00 01 00 00     	movdqa	%xmm1, 0x100(%rbp)
141524052: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
141524059: 48 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %rax
141524060: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141524065: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
141524069: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
141524070: e8 5b 1e 00 00              	callq	0x141525ed0 <.text+0x1524ed0>
141524075: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
141524079: 48 85 c9                    	testq	%rcx, %rcx
14152407c: 0f 84 f5 f7 ff ff           	je	0x141523877 <.text+0x1522877>
141524082: 4c 8b 4d 78                 	movq	0x78(%rbp), %r9
141524086: 4d 85 c9                    	testq	%r9, %r9
141524089: 74 2e                       	je	0x1415240b9 <.text+0x15230b9>
14152408b: 49 8d 41 01                 	leaq	0x1(%r9), %rax
14152408f: ba 58 00 00 00              	movl	$0x58, %edx
141524094: 48 f7 e2                    	mulq	%rdx
141524097: 48 83 c0 0f                 	addq	$0xf, %rax
14152409b: 48 83 e0 f0                 	andq	$-0x10, %rax
14152409f: 49 01 c1                    	addq	%rax, %r9
1415240a2: 49 83 c1 11                 	addq	$0x11, %r9
1415240a6: 74 11                       	je	0x1415240b9 <.text+0x15230b9>
1415240a8: 48 29 c1                    	subq	%rax, %rcx
1415240ab: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1415240b1: 4c 89 ca                    	movq	%r9, %rdx
1415240b4: e8 67 fb 0e ff              	callq	0x140613c20 <.text+0x612c20>
1415240b9: 4c 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %r9
1415240c0: 4d 85 c9                    	testq	%r9, %r9
1415240c3: 0f 84 ae f7 ff ff           	je	0x141523877 <.text+0x1522877>
1415240c9: 49 8d 41 01                 	leaq	0x1(%r9), %rax
1415240cd: b9 58 00 00 00              	movl	$0x58, %ecx
1415240d2: 48 f7 e1                    	mulq	%rcx
1415240d5: 48 83 c0 0f                 	addq	$0xf, %rax
1415240d9: 48 83 e0 f0                 	andq	$-0x10, %rax
1415240dd: 49 01 c1                    	addq	%rax, %r9
1415240e0: 49 83 c1 11                 	addq	$0x11, %r9
1415240e4: 0f 84 8d f7 ff ff           	je	0x141523877 <.text+0x1522877>
1415240ea: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
1415240f1: e9 70 f7 ff ff              	jmp	0x141523866 <.text+0x1522866>
1415240f6: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1415240fd: e8 4e 1c 65 ff              	callq	0x140b75d50 <.text+0xb74d50>
141524102: 4c 8b 85 80 01 00 00        	movq	0x180(%rbp), %r8
141524109: 4d 85 c0                    	testq	%r8, %r8
14152410c: 74 18                       	je	0x141524126 <.text+0x1523126>
14152410e: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
141524115: 48 85 d2                    	testq	%rdx, %rdx
141524118: 74 0c                       	je	0x141524126 <.text+0x1523126>
14152411a: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
141524121: e8 fa fa 0e ff              	callq	0x140613c20 <.text+0x612c20>
141524126: 0f 28 b5 d0 01 00 00        	movaps	0x1d0(%rbp), %xmm6
14152412d: 0f 28 bd e0 01 00 00        	movaps	0x1e0(%rbp), %xmm7
141524134: 44 0f 28 85 f0 01 00 00     	movaps	0x1f0(%rbp), %xmm8
14152413c: 44 0f 28 8d 00 02 00 00     	movaps	0x200(%rbp), %xmm9
141524144: 48 81 c4 98 02 00 00        	addq	$0x298, %rsp            # imm = 0x298
14152414b: 5b                          	popq	%rbx
14152414c: 5f                          	popq	%rdi
14152414d: 5e                          	popq	%rsi
14152414e: 41 5c                       	popq	%r12
141524150: 41 5d                       	popq	%r13
141524152: 41 5e                       	popq	%r14
141524154: 41 5f                       	popq	%r15
141524156: 5d                          	popq	%rbp
141524157: c3                          	retq
141524158: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141524160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141524165: 55                          	pushq	%rbp
141524166: 41 57                       	pushq	%r15
141524168: 41 56                       	pushq	%r14
14152416a: 41 55                       	pushq	%r13
14152416c: 41 54                       	pushq	%r12
14152416e: 56                          	pushq	%rsi
14152416f: 57                          	pushq	%rdi
141524170: 53                          	pushq	%rbx
141524171: 48 83 ec 68                 	subq	$0x68, %rsp
141524175: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14152417c: 66 44 0f 7f 4c 24 20        	movdqa	%xmm9, 0x20(%rsp)
141524183: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
141524189: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
14152418f: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
141524195: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14152419c: e8 5f 2d 35 ff              	callq	0x140876f00 <.text+0x875f00>
1415241a1: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1415241a8: e8 53 2d 35 ff              	callq	0x140876f00 <.text+0x875f00>
1415241ad: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
1415241b3: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
1415241b9: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1415241bf: 66 44 0f 6f 4c 24 20        	movdqa	0x20(%rsp), %xmm9
1415241c6: 48 83 c4 68                 	addq	$0x68, %rsp
1415241ca: 5b                          	popq	%rbx
1415241cb: 5f                          	popq	%rdi
1415241cc: 5e                          	popq	%rsi
1415241cd: 41 5c                       	popq	%r12
1415241cf: 41 5d                       	popq	%r13
1415241d1: 41 5e                       	popq	%r14
1415241d3: 41 5f                       	popq	%r15
1415241d5: 5d                          	popq	%rbp
1415241d6: c3                          	retq
1415241d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1415241e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1415241e5: 55                          	pushq	%rbp
1415241e6: 41 57                       	pushq	%r15
1415241e8: 41 56                       	pushq	%r14
1415241ea: 41 55                       	pushq	%r13
1415241ec: 41 54                       	pushq	%r12
1415241ee: 56                          	pushq	%rsi
1415241ef: 57                          	pushq	%rdi
1415241f0: 53                          	pushq	%rbx
1415241f1: 48 83 ec 68                 	subq	$0x68, %rsp
1415241f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1415241fc: 66 44 0f 7f 4c 24 20        	movdqa	%xmm9, 0x20(%rsp)
141524203: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
141524209: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
14152420f: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
141524215: 48 8d 8d 80 01 00 00        	leaq	0x180(%rbp), %rcx
14152421c: e8 ef 1e 36 ff              	callq	0x140886110 <.text+0x885110>
141524221: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
141524226: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14152422b: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
141524231: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
141524237: 48 83 c4 68                 	addq	$0x68, %rsp
14152423b: 5b                          	popq	%rbx
14152423c: 5f                          	popq	%rdi
14152423d: 5e                          	popq	%rsi
14152423e: 41 5c                       	popq	%r12
141524240: 41 5d                       	popq	%r13
141524242: 41 5e                       	popq	%r14
141524244: 41 5f                       	popq	%r15
141524246: 5d                          	popq	%rbp
141524247: c3                          	retq
