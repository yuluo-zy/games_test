
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ac80e0: 55                          	pushq	%rbp
140ac80e1: 41 56                       	pushq	%r14
140ac80e3: 56                          	pushq	%rsi
140ac80e4: 57                          	pushq	%rdi
140ac80e5: 53                          	pushq	%rbx
140ac80e6: 48 81 ec a0 00 00 00        	subq	$0xa0, %rsp
140ac80ed: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140ac80f5: 48 c7 45 18 fe ff ff ff     	movq	$-0x2, 0x18(%rbp)
140ac80fd: 48 8b 7d 78                 	movq	0x78(%rbp), %rdi
140ac8101: 4c 8b 31                    	movq	(%rcx), %r14
140ac8104: 41 83 3e 06                 	cmpl	$0x6, (%r14)
140ac8108: 75 2a                       	jne	0x140ac8134 <.text+0xac7134>
140ac810a: 41 0f b6 46 04              	movzbl	0x4(%r14), %eax
140ac810f: 8d 48 ff                    	leal	-0x1(%rax), %ecx
140ac8112: 80 f9 02                    	cmpb	$0x2, %cl
140ac8115: 73 1d                       	jae	0x140ac8134 <.text+0xac7134>
140ac8117: 4c 89 ce                    	movq	%r9, %rsi
140ac811a: 48 8b 5d 70                 	movq	0x70(%rbp), %rbx
140ac811e: 3c 02                       	cmpb	$0x2, %al
140ac8120: 75 32                       	jne	0x140ac8154 <.text+0xac7154>
140ac8122: 48 8b 12                    	movq	(%rdx), %rdx
140ac8125: 48 83 c2 10                 	addq	$0x10, %rdx
140ac8129: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140ac812d: e8 be 0b 1d 00              	callq	0x140c98cf0 <.text+0xc97cf0>
140ac8132: eb 38                       	jmp	0x140ac816c <.text+0xac716c>
140ac8134: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140ac8137: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140ac813b: 89 01                       	movl	%eax, (%rcx)
140ac813d: 48 8b 07                    	movq	(%rdi), %rax
140ac8140: c7 00 00 00 00 00           	movl	$0x0, (%rax)
140ac8146: 48 81 c4 a0 00 00 00        	addq	$0xa0, %rsp
140ac814d: 5b                          	popq	%rbx
140ac814e: 5f                          	popq	%rdi
140ac814f: 5e                          	popq	%rsi
140ac8150: 41 5e                       	popq	%r14
140ac8152: 5d                          	popq	%rbp
140ac8153: c3                          	retq
140ac8154: 49 8b 10                    	movq	(%r8), %rdx
140ac8157: 48 83 c2 10                 	addq	$0x10, %rdx
140ac815b: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140ac815f: f3 0f 10 15 01 4f f0 01     	movss	0x1f04f01(%rip), %xmm2  # 0x1429cd068
140ac8167: e8 84 fb 1c 00              	callq	0x140c97cf0 <.text+0xc96cf0>
140ac816c: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140ac8171: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140ac8175: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
140ac8179: f3 0f 10 15 ef d6 e5 01     	movss	0x1e5d6ef(%rip), %xmm2  # 0x142925870
140ac8181: 49 89 f1                    	movq	%rsi, %r9
140ac8184: e8 57 2e 9a 00              	callq	0x14146afe0 <.text+0x1469fe0>
140ac8189: 48 8b 75 b0                 	movq	-0x50(%rbp), %rsi
140ac818d: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
140ac8191: 48 85 c0                    	testq	%rax, %rax
140ac8194: 74 47                       	je	0x140ac81dd <.text+0xac71dd>
140ac8196: 48 8d 0c 85 00 00 00 00     	leaq	(,%rax,4), %rcx
140ac819e: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140ac81a2: 0f 57 d2                    	xorps	%xmm2, %xmm2
140ac81a5: 31 d2                       	xorl	%edx, %edx
140ac81a7: 0f 57 c9                    	xorps	%xmm1, %xmm1
140ac81aa: 0f 28 c2                    	movaps	%xmm2, %xmm0
140ac81ad: 0f 1f 00                    	nopl	(%rax)
140ac81b0: f3 0f 10 5c 16 04           	movss	0x4(%rsi,%rdx), %xmm3
140ac81b6: f3 0f c2 c0 03              	cmpunordss	%xmm0, %xmm0
140ac81bb: 0f 28 e0                    	movaps	%xmm0, %xmm4
140ac81be: 0f 54 e3                    	andps	%xmm3, %xmm4
140ac81c1: f3 0f 58 cb                 	addss	%xmm3, %xmm1
140ac81c5: f3 0f 5f da                 	maxss	%xmm2, %xmm3
140ac81c9: 0f 55 c3                    	andnps	%xmm3, %xmm0
140ac81cc: 0f 56 c4                    	orps	%xmm4, %xmm0
140ac81cf: 48 83 c2 0c                 	addq	$0xc, %rdx
140ac81d3: 0f 28 d0                    	movaps	%xmm0, %xmm2
140ac81d6: 48 39 d1                    	cmpq	%rdx, %rcx
140ac81d9: 75 d5                       	jne	0x140ac81b0 <.text+0xac71b0>
140ac81db: eb 06                       	jmp	0x140ac81e3 <.text+0xac71e3>
140ac81dd: 0f 57 c9                    	xorps	%xmm1, %xmm1
140ac81e0: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ac81e3: 41 8b 16                    	movl	(%r14), %edx
140ac81e6: 83 fa 06                    	cmpl	$0x6, %edx
140ac81e9: 41 0f 94 c0                 	sete	%r8b
140ac81ed: 41 0f b6 4e 04              	movzbl	0x4(%r14), %ecx
140ac81f2: 80 f9 02                    	cmpb	$0x2, %cl
140ac81f5: 41 0f 94 c1                 	sete	%r9b
140ac81f9: 45 84 c8                    	testb	%r9b, %r8b
140ac81fc: 74 1b                       	je	0x140ac8219 <.text+0xac7219>
140ac81fe: f3 0f 10 15 8a c4 e6 01     	movss	0x1e6c48a(%rip), %xmm2  # 0x142934690
140ac8206: 48 8b 0b                    	movq	(%rbx), %rcx
140ac8209: 83 79 08 02                 	cmpl	$0x2, 0x8(%rcx)
140ac820d: 75 2d                       	jne	0x140ac823c <.text+0xac723c>
140ac820f: 80 39 00                    	cmpb	$0x0, (%rcx)
140ac8212: 75 45                       	jne	0x140ac8259 <.text+0xac7259>
140ac8214: 0f 57 c9                    	xorps	%xmm1, %xmm1
140ac8217: eb 45                       	jmp	0x140ac825e <.text+0xac725e>
140ac8219: 83 fa 06                    	cmpl	$0x6, %edx
140ac821c: 0f 85 77 01 00 00           	jne	0x140ac8399 <.text+0xac7399>
140ac8222: 80 f9 01                    	cmpb	$0x1, %cl
140ac8225: 0f 85 6e 01 00 00           	jne	0x140ac8399 <.text+0xac7399>
140ac822b: f3 0f 10 15 d1 c3 e6 01     	movss	0x1e6c3d1(%rip), %xmm2  # 0x142934604
140ac8233: 48 8b 0b                    	movq	(%rbx), %rcx
140ac8236: 83 79 08 02                 	cmpl	$0x2, 0x8(%rcx)
140ac823a: 74 d3                       	je	0x140ac820f <.text+0xac720f>
140ac823c: 0f 57 db                    	xorps	%xmm3, %xmm3
140ac823f: f3 48 0f 2a d8              	cvtsi2ss	%rax, %xmm3
140ac8244: f3 0f 5e cb                 	divss	%xmm3, %xmm1
140ac8248: f3 0f 58 ca                 	addss	%xmm2, %xmm1
140ac824c: f3 0f 58 0d fc 6f fb 01     	addss	0x1fb6ffc(%rip), %xmm1  # 0x142a7f250
140ac8254: 0f 28 d1                    	movaps	%xmm1, %xmm2
140ac8257: eb 09                       	jmp	0x140ac8262 <.text+0xac7262>
140ac8259: f3 0f 10 49 04              	movss	0x4(%rcx), %xmm1
140ac825e: f3 0f 58 d1                 	addss	%xmm1, %xmm2
140ac8262: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
140ac8269: f3 0f 10 1d df 6f fb 01     	movss	0x1fb6fdf(%rip), %xmm3  # 0x142a7f250
140ac8271: 0f 28 c8                    	movaps	%xmm0, %xmm1
140ac8274: f3 0f 58 cb                 	addss	%xmm3, %xmm1
140ac8278: f3 0f 5c d1                 	subss	%xmm1, %xmm2
140ac827c: f3 0f 10 25 ac 4d f0 01     	movss	0x1f04dac(%rip), %xmm4  # 0x1429cd030
140ac8284: f3 0f 5f e2                 	maxss	%xmm2, %xmm4
140ac8288: f3 0f 10 0d 70 5b fb 01     	movss	0x1fb5b70(%rip), %xmm1  # 0x142a7de00
140ac8290: f3 0f 5d cc                 	minss	%xmm4, %xmm1
140ac8294: 8b 53 1c                    	movl	0x1c(%rbx), %edx
140ac8297: 4c 8b 43 10                 	movq	0x10(%rbx), %r8
140ac829b: 41 89 10                    	movl	%edx, (%r8)
140ac829e: f3 0f 11 49 40              	movss	%xmm1, 0x40(%rcx)
140ac82a3: 48 8b 0b                    	movq	(%rbx), %rcx
140ac82a6: f3 0f 10 51 3c              	movss	0x3c(%rcx), %xmm2
140ac82ab: 0f 57 e4                    	xorps	%xmm4, %xmm4
140ac82ae: f3 0f c2 e2 00              	cmpeqss	%xmm2, %xmm4
140ac82b3: f3 0f 10 2d 05 6f f1 01     	movss	0x1f16f05(%rip), %xmm5  # 0x1429df1c0
140ac82bb: 0f 54 ec                    	andps	%xmm4, %xmm5
140ac82be: 0f 55 e2                    	andnps	%xmm2, %xmm4
140ac82c1: 0f 56 e5                    	orps	%xmm5, %xmm4
140ac82c4: 8b 5f 1c                    	movl	0x1c(%rdi), %ebx
140ac82c7: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
140ac82cb: 41 89 1e                    	movl	%ebx, (%r14)
140ac82ce: 48 8b 3f                    	movq	(%rdi), %rdi
140ac82d1: f3 0f 11 67 08              	movss	%xmm4, 0x8(%rdi)
140ac82d6: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140ac82da: f3 0f 58 cb                 	addss	%xmm3, %xmm1
140ac82de: 80 3f 00                    	cmpb	$0x0, (%rdi)
140ac82e1: 75 05                       	jne	0x140ac82e8 <.text+0xac72e8>
140ac82e3: 0f 28 c1                    	movaps	%xmm1, %xmm0
140ac82e6: eb 05                       	jmp	0x140ac82ed <.text+0xac72ed>
140ac82e8: f3 0f 10 47 04              	movss	0x4(%rdi), %xmm0
140ac82ed: 48 8b 00                    	movq	(%rax), %rax
140ac82f0: f3 0f 10 50 28              	movss	0x28(%rax), %xmm2
140ac82f5: f3 0f 10 1d ab c2 e6 01     	movss	0x1e6c2ab(%rip), %xmm3  # 0x1429345a8
140ac82fd: e8 de 81 1c 00              	callq	0x140c904e0 <.text+0xc8f4e0>
140ac8302: 41 89 1e                    	movl	%ebx, (%r14)
140ac8305: c7 07 01 00 00 00           	movl	$0x1, (%rdi)
140ac830b: f3 0f 11 47 04              	movss	%xmm0, 0x4(%rdi)
140ac8310: f3 0f 5c 47 08              	subss	0x8(%rdi), %xmm0
140ac8315: 41 89 1e                    	movl	%ebx, (%r14)
140ac8318: f3 0f 11 47 0c              	movss	%xmm0, 0xc(%rdi)
140ac831d: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
140ac8321: 48 85 c0                    	testq	%rax, %rax
140ac8324: 74 16                       	je	0x140ac833c <.text+0xac733c>
140ac8326: 48 c1 e0 02                 	shlq	$0x2, %rax
140ac832a: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
140ac832e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac8334: 48 89 f1                    	movq	%rsi, %rcx
140ac8337: e8 e4 b8 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140ac833c: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
140ac8340: 48 85 d2                    	testq	%rdx, %rdx
140ac8343: 74 13                       	je	0x140ac8358 <.text+0xac7358>
140ac8345: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
140ac8349: 48 c1 e2 02                 	shlq	$0x2, %rdx
140ac834d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac8353: e8 c8 b8 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140ac8358: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140ac835c: 48 85 d2                    	testq	%rdx, %rdx
140ac835f: 74 13                       	je	0x140ac8374 <.text+0xac7374>
140ac8361: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140ac8365: 48 c1 e2 03                 	shlq	$0x3, %rdx
140ac8369: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac836f: e8 ac b8 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140ac8374: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140ac8378: 48 85 d2                    	testq	%rdx, %rdx
140ac837b: 0f 84 c5 fd ff ff           	je	0x140ac8146 <.text+0xac7146>
140ac8381: 48 8b 4d 00                 	movq	(%rbp), %rcx
140ac8385: 48 c1 e2 02                 	shlq	$0x2, %rdx
140ac8389: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac838f: e8 8c b8 b4 ff              	callq	0x140613c20 <.text+0x612c20>
140ac8394: e9 ad fd ff ff              	jmp	0x140ac8146 <.text+0xac7146>
140ac8399: 48 8d 0d 40 73 01 02        	leaq	0x2017340(%rip), %rcx   # 0x142adf6e0
140ac83a0: 4c 8d 05 09 77 01 02        	leaq	0x2017709(%rip), %r8    # 0x142adfab0
140ac83a7: ba 28 00 00 00              	movl	$0x28, %edx
140ac83ac: e8 7f 10 e1 01              	callq	0x1428d9430 <.text+0x28d8430>
140ac83b1: 0f 0b                       	ud2
140ac83b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140ac83c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac83c5: 55                          	pushq	%rbp
140ac83c6: 41 56                       	pushq	%r14
140ac83c8: 56                          	pushq	%rsi
140ac83c9: 57                          	pushq	%rdi
140ac83ca: 53                          	pushq	%rbx
140ac83cb: 48 83 ec 30                 	subq	$0x30, %rsp
140ac83cf: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac83d6: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140ac83da: e8 11 a1 b0 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140ac83df: 90                          	nop
140ac83e0: 48 83 c4 30                 	addq	$0x30, %rsp
140ac83e4: 5b                          	popq	%rbx
140ac83e5: 5f                          	popq	%rdi
140ac83e6: 5e                          	popq	%rsi
140ac83e7: 41 5e                       	popq	%r14
140ac83e9: 5d                          	popq	%rbp
140ac83ea: c3                          	retq
140ac83eb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140ac83f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac83f5: 55                          	pushq	%rbp
140ac83f6: 41 56                       	pushq	%r14
140ac83f8: 56                          	pushq	%rsi
140ac83f9: 57                          	pushq	%rdi
140ac83fa: 53                          	pushq	%rbx
140ac83fb: 48 83 ec 30                 	subq	$0x30, %rsp
140ac83ff: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac8406: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140ac840a: e8 11 36 68 ff              	callq	0x14014ba20 <.text+0x14aa20>
140ac840f: 90                          	nop
140ac8410: 48 83 c4 30                 	addq	$0x30, %rsp
140ac8414: 5b                          	popq	%rbx
140ac8415: 5f                          	popq	%rdi
140ac8416: 5e                          	popq	%rsi
140ac8417: 41 5e                       	popq	%r14
140ac8419: 5d                          	popq	%rbp
140ac841a: c3                          	retq
