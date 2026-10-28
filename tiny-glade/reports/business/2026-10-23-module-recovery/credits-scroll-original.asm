
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dfb6d0: 81 7a 58 c3 00 00 00        	cmpl	$0xc3, 0x58(%rdx)
141dfb6d7: 75 17                       	jne	0x141dfb6f0 <.text+0x1dfa6f0>
141dfb6d9: f3 0f 10 42 04              	movss	0x4(%rdx), %xmm0
141dfb6de: f3 0f 59 05 b2 2e e2 00     	mulss	0xe22eb2(%rip), %xmm0   # 0x142c1e598
141dfb6e6: f3 0f 58 41 08              	addss	0x8(%rcx), %xmm0
141dfb6eb: f3 0f 11 41 08              	movss	%xmm0, 0x8(%rcx)
141dfb6f0: c3                          	retq
141dfb6f1: cc                          	int3
141dfb6f2: cc                          	int3
141dfb6f3: cc                          	int3
141dfb6f4: cc                          	int3
141dfb6f5: cc                          	int3
141dfb6f6: cc                          	int3
141dfb6f7: cc                          	int3
141dfb6f8: cc                          	int3
141dfb6f9: cc                          	int3
141dfb6fa: cc                          	int3
141dfb6fb: cc                          	int3
141dfb6fc: cc                          	int3
141dfb6fd: cc                          	int3
141dfb6fe: cc                          	int3
141dfb6ff: cc                          	int3
141dfb700: 56                          	pushq	%rsi
141dfb701: 57                          	pushq	%rdi
141dfb702: 53                          	pushq	%rbx
141dfb703: 48 83 ec 30                 	subq	$0x30, %rsp
141dfb707: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
141dfb70c: 48 89 d7                    	movq	%rdx, %rdi
141dfb70f: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dfb712: f3 41 0f 59 40 44           	mulss	0x44(%r8), %xmm0
141dfb718: f3 41 0f 58 40 48           	addss	0x48(%r8), %xmm0
141dfb71e: 48 89 ce                    	movq	%rcx, %rsi
141dfb721: 49 8b 40 18                 	movq	0x18(%r8), %rax
141dfb725: 0f b6 00                    	movzbl	(%rax), %eax
141dfb728: a8 02                       	testb	$0x2, %al
141dfb72a: 74 05                       	je	0x141dfb731 <.text+0x1dfa731>
141dfb72c: f3 0f 11 47 10              	movss	%xmm0, 0x10(%rdi)
141dfb731: f3 41 0f 10 70 4c           	movss	0x4c(%r8), %xmm6
141dfb737: f3 0f 10 4f 08              	movss	0x8(%rdi), %xmm1
141dfb73c: a8 04                       	testb	$0x4, %al
141dfb73e: 75 74                       	jne	0x141dfb7b4 <.text+0x1dfa7b4>
141dfb740: 48 8d 5f 08                 	leaq	0x8(%rdi), %rbx
141dfb744: 48 8d 4f 0c                 	leaq	0xc(%rdi), %rcx
141dfb748: f3 0f 10 47 0c              	movss	0xc(%rdi), %xmm0
141dfb74d: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
141dfb751: f3 0f 58 c8                 	addss	%xmm0, %xmm1
141dfb755: f3 0f 11 4f 08              	movss	%xmm1, 0x8(%rdi)
141dfb75a: f3 0f 10 4f 14              	movss	0x14(%rdi), %xmm1
141dfb75f: f3 0f 10 1d 9d a1 b2 00     	movss	0xb2a19d(%rip), %xmm3   # 0x142925904
141dfb767: 0f 28 d6                    	movaps	%xmm6, %xmm2
141dfb76a: e8 01 4f e9 fe              	callq	0x140c90670 <.text+0xc8f670>
141dfb76f: f3 0f 10 47 08              	movss	0x8(%rdi), %xmm0
141dfb774: f3 0f 10 4f 18              	movss	0x18(%rdi), %xmm1
141dfb779: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
141dfb77c: 76 17                       	jbe	0x141dfb795 <.text+0x1dfa795>
141dfb77e: f3 0f 10 1d 6a 2f b3 00     	movss	0xb32f6a(%rip), %xmm3   # 0x14292e6f0
141dfb786: 48 89 d9                    	movq	%rbx, %rcx
141dfb789: 0f 28 d6                    	movaps	%xmm6, %xmm2
141dfb78c: e8 df 4e e9 fe              	callq	0x140c90670 <.text+0xc8f670>
141dfb791: f3 0f 10 03                 	movss	(%rbx), %xmm0
141dfb795: f3 0f 10 4f 1c              	movss	0x1c(%rdi), %xmm1
141dfb79a: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
141dfb79d: 76 3d                       	jbe	0x141dfb7dc <.text+0x1dfa7dc>
141dfb79f: f3 0f 10 1d 49 2f b3 00     	movss	0xb32f49(%rip), %xmm3   # 0x14292e6f0
141dfb7a7: 48 89 d9                    	movq	%rbx, %rcx
141dfb7aa: 0f 28 d6                    	movaps	%xmm6, %xmm2
141dfb7ad: e8 be 4e e9 fe              	callq	0x140c90670 <.text+0xc8f670>
141dfb7b2: eb 28                       	jmp	0x141dfb7dc <.text+0x1dfa7dc>
141dfb7b4: f3 0f 10 57 10              	movss	0x10(%rdi), %xmm2
141dfb7b9: f3 0f 5c d0                 	subss	%xmm0, %xmm2
141dfb7bd: f3 0f 58 ca                 	addss	%xmm2, %xmm1
141dfb7c1: f3 0f 5f 35 a7 55 b5 00     	maxss	0xb555a7(%rip), %xmm6   # 0x142950d70
141dfb7c9: f3 0f 11 4f 08              	movss	%xmm1, 0x8(%rdi)
141dfb7ce: f3 0f 5e d6                 	divss	%xmm6, %xmm2
141dfb7d2: f3 0f 11 57 0c              	movss	%xmm2, 0xc(%rdi)
141dfb7d7: f3 0f 11 47 10              	movss	%xmm0, 0x10(%rdi)
141dfb7dc: 0f 57 c0                    	xorps	%xmm0, %xmm0
141dfb7df: 0f 11 06                    	movups	%xmm0, (%rsi)
141dfb7e2: 48 89 f0                    	movq	%rsi, %rax
141dfb7e5: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
141dfb7ea: 48 83 c4 30                 	addq	$0x30, %rsp
141dfb7ee: 5b                          	popq	%rbx
141dfb7ef: 5f                          	popq	%rdi
141dfb7f0: 5e                          	popq	%rsi
141dfb7f1: c3                          	retq
141dfb7f2: cc                          	int3
141dfb7f3: cc                          	int3
141dfb7f4: cc                          	int3
141dfb7f5: cc                          	int3
141dfb7f6: cc                          	int3
141dfb7f7: cc                          	int3
141dfb7f8: cc                          	int3
141dfb7f9: cc                          	int3
141dfb7fa: cc                          	int3
141dfb7fb: cc                          	int3
141dfb7fc: cc                          	int3
141dfb7fd: cc                          	int3
141dfb7fe: cc                          	int3
141dfb7ff: cc                          	int3
141dfb800: 55                          	pushq	%rbp
141dfb801: 41 57                       	pushq	%r15
141dfb803: 41 56                       	pushq	%r14
141dfb805: 41 55                       	pushq	%r13
141dfb807: 41 54                       	pushq	%r12
141dfb809: 56                          	pushq	%rsi
141dfb80a: 57                          	pushq	%rdi
141dfb80b: 53                          	pushq	%rbx
141dfb80c: 48 83 ec 78                 	subq	$0x78, %rsp
141dfb810: 48 8d 6c 24 70              	leaq	0x70(%rsp), %rbp
141dfb815: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
141dfb81d: 83 39 01                    	cmpl	$0x1, (%rcx)
141dfb820: 0f 85 5d 01 00 00           	jne	0x141dfb983 <.text+0x1dfa983>
141dfb826: 48 89 d6                    	movq	%rdx, %rsi
141dfb829: 8b 79 04                    	movl	0x4(%rcx), %edi
141dfb82c: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141dfb830: e8 eb 6a d9 fe              	callq	0x140b92320 <.text+0xb91320>
141dfb835: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
141dfb839: e8 e2 72 c8 fe              	callq	0x140a82b20 <.text+0xa81b20>
141dfb83e: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141dfb842: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
141dfb846: 41 89 f8                    	movl	%edi, %r8d
141dfb849: e8 62 68 d9 fe              	callq	0x140b920b0 <.text+0xb910b0>
141dfb84e: 4c 8d 0d a3 a2 18 01        	leaq	0x118a2a3(%rip), %r9    # 0x142f85af8
141dfb855: 48 8d 7d e8                 	leaq	-0x18(%rbp), %rdi
141dfb859: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141dfb85d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141dfb863: 48 89 f9                    	movq	%rdi, %rcx
141dfb866: e8 d5 6a d9 fe              	callq	0x140b92340 <.text+0xb91340>
141dfb86b: 48 8b 5d d8                 	movq	-0x28(%rbp), %rbx
141dfb86f: 4c 8b 75 e0                 	movq	-0x20(%rbp), %r14
141dfb873: 4d 85 f6                    	testq	%r14, %r14
141dfb876: 74 65                       	je	0x141dfb8dd <.text+0x1dfa8dd>
141dfb878: 4c 8d 7b 08                 	leaq	0x8(%rbx), %r15
141dfb87c: 49 bc fe ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffe, %r12 # imm = 0x7FFFFFFFFFFFFFFE
141dfb886: 49 bd 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %r13 # imm = 0x8000000000000002
141dfb890: eb 17                       	jmp	0x141dfb8a9 <.text+0x1dfa8a9>
141dfb892: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141dfb8a0: 49 83 c7 20                 	addq	$0x20, %r15
141dfb8a4: 49 ff ce                    	decq	%r14
141dfb8a7: 74 34                       	je	0x141dfb8dd <.text+0x1dfa8dd>
141dfb8a9: 49 8b 57 f8                 	movq	-0x8(%r15), %rdx
141dfb8ad: 4a 8d 0c 22                 	leaq	(%rdx,%r12), %rcx
141dfb8b1: 48 83 f9 03                 	cmpq	$0x3, %rcx
141dfb8b5: 0f 92 c0                    	setb	%al
141dfb8b8: 48 83 f9 01                 	cmpq	$0x1, %rcx
141dfb8bc: 0f 95 c1                    	setne	%cl
141dfb8bf: 4c 39 ea                    	cmpq	%r13, %rdx
141dfb8c2: 7c dc                       	jl	0x141dfb8a0 <.text+0x1dfa8a0>
141dfb8c4: 20 c8                       	andb	%cl, %al
141dfb8c6: 75 d8                       	jne	0x141dfb8a0 <.text+0x1dfa8a0>
141dfb8c8: 48 85 d2                    	testq	%rdx, %rdx
141dfb8cb: 74 d3                       	je	0x141dfb8a0 <.text+0x1dfa8a0>
141dfb8cd: 49 8b 0f                    	movq	(%r15), %rcx
141dfb8d0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141dfb8d6: e8 45 83 81 fe              	callq	0x140613c20 <.text+0x612c20>
141dfb8db: eb c3                       	jmp	0x141dfb8a0 <.text+0x1dfa8a0>
141dfb8dd: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
141dfb8e1: 48 85 d2                    	testq	%rdx, %rdx
141dfb8e4: 74 12                       	je	0x141dfb8f8 <.text+0x1dfa8f8>
141dfb8e6: 48 c1 e2 05                 	shlq	$0x5, %rdx
141dfb8ea: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141dfb8f0: 48 89 d9                    	movq	%rbx, %rcx
141dfb8f3: e8 28 83 81 fe              	callq	0x140613c20 <.text+0x612c20>
141dfb8f8: f3 0f 10 46 08              	movss	0x8(%rsi), %xmm0
141dfb8fd: f3 0f 10 4e 0c              	movss	0xc(%rsi), %xmm1
141dfb902: f3 0f 10 15 66 9f b2 00     	movss	0xb29f66(%rip), %xmm2   # 0x142925870
141dfb90a: f3 0f 10 5e 10              	movss	0x10(%rsi), %xmm3
141dfb90f: f3 0f 59 da                 	mulss	%xmm2, %xmm3
141dfb913: f3 0f 59 56 14              	mulss	0x14(%rsi), %xmm2
141dfb918: 0f 28 e0                    	movaps	%xmm0, %xmm4
141dfb91b: f3 0f 5c e3                 	subss	%xmm3, %xmm4
141dfb91f: 0f 28 e9                    	movaps	%xmm1, %xmm5
141dfb922: f3 0f 5c ea                 	subss	%xmm2, %xmm5
141dfb926: f3 0f 58 d8                 	addss	%xmm0, %xmm3
141dfb92a: f3 0f 58 d1                 	addss	%xmm1, %xmm2
141dfb92e: 48 8b 0e                    	movq	(%rsi), %rcx
141dfb931: 66 0f 7e e2                 	movd	%xmm4, %edx
141dfb935: 66 0f 7e e8                 	movd	%xmm5, %eax
141dfb939: 48 c1 e0 20                 	shlq	$0x20, %rax
141dfb93d: 48 09 c2                    	orq	%rax, %rdx
141dfb940: f3 0f 5c dc                 	subss	%xmm4, %xmm3
141dfb944: f3 0f 5c d5                 	subss	%xmm5, %xmm2
141dfb948: 66 41 0f 7e d8              	movd	%xmm3, %r8d
141dfb94d: 66 0f 7e d0                 	movd	%xmm2, %eax
141dfb951: 48 c1 e0 20                 	shlq	$0x20, %rax
141dfb955: 49 09 c0                    	orq	%rax, %r8
141dfb958: f3 0f 10 5e 18              	movss	0x18(%rsi), %xmm3
141dfb95d: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
141dfb962: e8 a9 3b d9 fe              	callq	0x140b8f510 <.text+0xb8e510>
141dfb967: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
141dfb96b: 48 85 d2                    	testq	%rdx, %rdx
141dfb96e: 74 13                       	je	0x141dfb983 <.text+0x1dfa983>


B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f9cd6d: c7 00 00 00 00 00           	movl	$0x0, (%rax)
140f9cd73: 0f 28 05 46 18 c1 01        	movaps	0x1c11846(%rip), %xmm0  # 0x142bae5c0
140f9cd7a: 0f 11 40 08                 	movups	%xmm0, 0x8(%rax)
140f9cd7e: f3 0f 7e 05 4a 18 c1 01     	movq	0x1c1184a(%rip), %xmm0  # 0x142bae5d0
140f9cd86: 66 0f d6 40 18              	movq	%xmm0, 0x18(%rax)


B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141dfa41a: f3 41 0f 10 37              	movss	(%r15), %xmm6
141dfa41f: c6 85 e7 00 00 00 01        	movb	$0x1, 0xe7(%rbp)
141dfa426: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
141dfa42a: b2 01                       	movb	$0x1, %dl
141dfa42c: e8 ef 27 1a ff              	callq	0x140f9cc20 <.text+0xf9bc20>
141dfa431: f3 0f 11 70 1c              	movss	%xmm6, 0x1c(%rax)
141dfa436: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
141dfa43a: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
141dfa43e: 66 0f 6f 45 e0              	movdqa	-0x20(%rbp), %xmm0
141dfa443: 66 0f 7f 45 c0              	movdqa	%xmm0, -0x40(%rbp)
141dfa448: c6 85 e7 00 00 00 01        	movb	$0x1, 0xe7(%rbp)
141dfa44f: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
141dfa453: e8 58 26 1a ff              	callq	0x140f9cab0 <.text+0xf9bab0>
141dfa458: f3 0f 10 70 08              	movss	0x8(%rax), %xmm6


B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c90670: 56                          	pushq	%rsi
140c90671: 48 83 ec 40                 	subq	$0x40, %rsp
140c90675: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140c9067a: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
140c9067f: 0f 28 c2                    	movaps	%xmm2, %xmm0
140c90682: 0f 28 f1                    	movaps	%xmm1, %xmm6
140c90685: 48 89 ce                    	movq	%rcx, %rsi
140c90688: f3 0f 10 39                 	movss	(%rcx), %xmm7
140c9068c: 0f 57 05 5d 52 c9 01        	xorps	0x1c9525d(%rip), %xmm0  # 0x1429258f0
140c90693: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
140c90697: e8 24 38 c9 01              	callq	0x142923ec0 <.text+0x2922ec0>
140c9069c: f3 0f 10 0d 60 52 c9 01     	movss	0x1c95260(%rip), %xmm1  # 0x142925904
140c906a4: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140c906a8: f3 0f 5c f7                 	subss	%xmm7, %xmm6
140c906ac: f3 0f 59 f1                 	mulss	%xmm1, %xmm6
140c906b0: f3 0f 58 f7                 	addss	%xmm7, %xmm6
140c906b4: f3 0f 11 36                 	movss	%xmm6, (%rsi)
140c906b8: 0f 28 c6                    	movaps	%xmm6, %xmm0
140c906bb: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140c906c0: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140c906c5: 48 83 c4 40                 	addq	$0x40, %rsp
140c906c9: 5e                          	popq	%rsi
140c906ca: c3                          	retq

RNE mapped data 0x142bae5c0: 0000c8c300000000000000000000b4420000c8c3ffff7f7f
f32: -400, 0, 0, 90, -400, f32::MAX
