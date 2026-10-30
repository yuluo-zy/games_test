
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140811fc0: 41 57                       	pushq	%r15
140811fc2: 41 56                       	pushq	%r14
140811fc4: 41 55                       	pushq	%r13
140811fc6: 41 54                       	pushq	%r12
140811fc8: 56                          	pushq	%rsi
140811fc9: 57                          	pushq	%rdi
140811fca: 55                          	pushq	%rbp
140811fcb: 53                          	pushq	%rbx
140811fcc: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140811fd3: 66 44 0f 7f 8c 24 f0 00 00 00       	movdqa	%xmm9, 0xf0(%rsp)
140811fdd: 44 0f 29 84 24 e0 00 00 00  	movaps	%xmm8, 0xe0(%rsp)
140811fe6: 0f 29 bc 24 d0 00 00 00     	movaps	%xmm7, 0xd0(%rsp)
140811fee: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
140811ff6: 48 8b 1a                    	movq	(%rdx), %rbx
140811ff9: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140811ffd: 4c 8b 03                    	movq	(%rbx), %r8
140812000: 31 f6                       	xorl	%esi, %esi
140812002: 4d 89 c1                    	movq	%r8, %r9
140812005: 4c 2b 4a 18                 	subq	0x18(%rdx), %r9
140812009: 4c 0f 42 ce                 	cmovbq	%rsi, %r9
14081200d: 4c 2b 42 38                 	subq	0x38(%rdx), %r8
140812011: 4c 0f 42 c6                 	cmovbq	%rsi, %r8
140812015: 4b 8d 04 49                 	leaq	(%r9,%r9,2), %rax
140812019: 48 c1 e0 04                 	shlq	$0x4, %rax
14081201d: 48 03 42 08                 	addq	0x8(%rdx), %rax
140812021: 4c 8b 52 10                 	movq	0x10(%rdx), %r10
140812025: 4c 8b 5a 30                 	movq	0x30(%rdx), %r11
140812029: 4d 29 ca                    	subq	%r9, %r10
14081202c: 4c 0f 42 d6                 	cmovbq	%rsi, %r10
140812030: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140812036: 49 0f 42 c1                 	cmovbq	%r9, %rax
14081203a: 4b 8d 3c 40                 	leaq	(%r8,%r8,2), %rdi
14081203e: 48 c1 e7 04                 	shlq	$0x4, %rdi
140812042: 48 03 7a 28                 	addq	0x28(%rdx), %rdi
140812046: 4d 29 c3                    	subq	%r8, %r11
140812049: 4c 0f 42 de                 	cmovbq	%rsi, %r11
14081204d: 49 0f 42 f9                 	cmovbq	%r9, %rdi
140812051: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
140812055: 4c 29 da                    	subq	%r11, %rdx
140812058: 4c 29 d2                    	subq	%r10, %rdx
14081205b: 48 89 9c 24 b8 00 00 00     	movq	%rbx, 0xb8(%rsp)
140812063: 48 89 13                    	movq	%rdx, (%rbx)
140812066: 4b 8d 14 52                 	leaq	(%r10,%r10,2), %rdx
14081206a: 48 c1 e2 04                 	shlq	$0x4, %rdx
14081206e: 48 01 c2                    	addq	%rax, %rdx
140812071: 48 89 94 24 a8 00 00 00     	movq	%rdx, 0xa8(%rsp)
140812079: 4b 8d 14 5b                 	leaq	(%r11,%r11,2), %rdx
14081207d: 48 c1 e2 04                 	shlq	$0x4, %rdx
140812081: 48 01 fa                    	addq	%rdi, %rdx
140812084: 48 89 94 24 a0 00 00 00     	movq	%rdx, 0xa0(%rsp)
14081208c: 8b 69 1c                    	movl	0x1c(%rcx), %ebp
14081208f: 4c 8b 39                    	movq	(%rcx), %r15
140812092: 4c 8b 69 10                 	movq	0x10(%rcx), %r13
140812096: 4c 8d 64 24 70              	leaq	0x70(%rsp), %r12
14081209b: 66 0f 6f 35 dd af 1b 02     	movdqa	0x21bafdd(%rip), %xmm6  # 0x1429cd080
1408120a3: 66 0f 6f 3d e5 af 1b 02     	movdqa	0x21bafe5(%rip), %xmm7  # 0x1429cd090
1408120ab: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
1408120af: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
1408120b4: 89 6c 24 3c                 	movl	%ebp, 0x3c(%rsp)
1408120b8: 4c 89 bc 24 98 00 00 00     	movq	%r15, 0x98(%rsp)
1408120c0: 4c 89 ac 24 90 00 00 00     	movq	%r13, 0x90(%rsp)
1408120c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1408120d0: 48 85 c0                    	testq	%rax, %rax
1408120d3: 74 1b                       	je	0x1408120f0 <.text+0x8110f0>
1408120d5: 48 3b 84 24 a8 00 00 00     	cmpq	0xa8(%rsp), %rax
1408120dd: 74 11                       	je	0x1408120f0 <.text+0x8110f0>
1408120df: 48 8d 58 30                 	leaq	0x30(%rax), %rbx
1408120e3: 48 89 7c 24 28              	movq	%rdi, 0x28(%rsp)
1408120e8: 48 89 c6                    	movq	%rax, %rsi
1408120eb: eb 2b                       	jmp	0x140812118 <.text+0x811118>
1408120ed: 0f 1f 00                    	nopl	(%rax)
1408120f0: 48 85 ff                    	testq	%rdi, %rdi
1408120f3: 0f 84 1a 03 00 00           	je	0x140812413 <.text+0x811413>
1408120f9: 48 3b bc 24 a0 00 00 00     	cmpq	0xa0(%rsp), %rdi
140812101: 0f 84 0c 03 00 00           	je	0x140812413 <.text+0x811413>
140812107: 48 8d 47 30                 	leaq	0x30(%rdi), %rax
14081210b: 31 db                       	xorl	%ebx, %ebx
14081210d: 48 89 fe                    	movq	%rdi, %rsi
140812110: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140812115: 48 89 c7                    	movq	%rax, %rdi
140812118: 48 8b 84 24 b8 00 00 00     	movq	0xb8(%rsp), %rax
140812120: 48 ff 00                    	incq	(%rax)
140812123: 41 89 6d 00                 	movl	%ebp, (%r13)
140812127: 48 8b 56 18                 	movq	0x18(%rsi), %rdx
14081212b: 4c 89 f9                    	movq	%r15, %rcx
14081212e: e8 ed c2 20 00              	callq	0x140a1e420 <.text+0xa1d420>
140812133: 49 89 c6                    	movq	%rax, %r14
140812136: 48 89 d8                    	movq	%rbx, %rax
140812139: 4d 85 f6                    	testq	%r14, %r14
14081213c: 74 92                       	je	0x1408120d0 <.text+0x8110d0>
14081213e: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
140812142: 48 8b 7c 24 28              	movq	0x28(%rsp), %rdi
140812147: 48 89 d8                    	movq	%rbx, %rax
14081214a: 48 85 c9                    	testq	%rcx, %rcx
14081214d: 74 81                       	je	0x1408120d0 <.text+0x8110d0>
14081214f: 48 89 9c 24 b0 00 00 00     	movq	%rbx, 0xb0(%rsp)
140812157: 4c 8b 6e 08                 	movq	0x8(%rsi), %r13
14081215b: 48 8d 04 49                 	leaq	(%rcx,%rcx,2), %rax
14081215f: 48 8d 3c 85 00 00 00 00     	leaq	(,%rax,4), %rdi
140812167: 4c 01 ef                    	addq	%r13, %rdi
14081216a: 4d 89 f7                    	movq	%r14, %r15
14081216d: 49 83 c7 60                 	addq	$0x60, %r15
140812171: e9 da 00 00 00              	jmp	0x140812250 <.text+0x811250>
140812176: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140812180: 49 21 c0                    	andq	%rax, %r8
140812183: f3 42 0f 6f 0c 03           	movdqu	(%rbx,%r8), %xmm1
140812189: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14081218d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140812191: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140812196: 45 85 c9                    	testl	%r9d, %r9d
140812199: 74 35                       	je	0x1408121d0 <.text+0x8111d0>
14081219b: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1408121a0: 4d 01 c2                    	addq	%r8, %r10
1408121a3: 49 21 c2                    	andq	%rax, %r10
1408121a6: 49 f7 da                    	negq	%r10
1408121a9: 4f 8d 14 92                 	leaq	(%r10,%r10,4), %r10
1408121ad: 49 c1 e2 04                 	shlq	$0x4, %r10
1408121b1: 42 3b 2c 11                 	cmpl	(%rcx,%r10), %ebp
1408121b5: 74 39                       	je	0x1408121f0 <.text+0x8111f0>
1408121b7: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1408121bb: 66 45 21 ca                 	andw	%r9w, %r10w
1408121bf: 45 89 d1                    	movl	%r10d, %r9d
1408121c2: 75 d7                       	jne	0x14081219b <.text+0x81119b>
1408121c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408121d0: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
1408121d5: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1408121da: 45 85 c9                    	testl	%r9d, %r9d
1408121dd: 0f 85 fd 01 00 00           	jne	0x1408123e0 <.text+0x8113e0>
1408121e3: 49 01 d0                    	addq	%rdx, %r8
1408121e6: 49 83 c0 10                 	addq	$0x10, %r8
1408121ea: 48 83 c2 10                 	addq	$0x10, %rdx
1408121ee: eb 90                       	jmp	0x140812180 <.text+0x811180>
1408121f0: 42 0f b6 44 13 da           	movzbl	-0x26(%rbx,%r10), %eax
1408121f6: 8d 48 fe                    	leal	-0x2(%rax), %ecx
1408121f9: 48 ff c8                    	decq	%rax
1408121fc: 80 f9 03                    	cmpb	$0x3, %cl
1408121ff: b9 00 00 00 00              	movl	$0x0, %ecx
140812204: 48 0f 43 c1                 	cmovaeq	%rcx, %rax
140812208: 48 85 c0                    	testq	%rax, %rax
14081220b: 0f 85 38 02 00 00           	jne	0x140812449 <.text+0x811449>
140812211: 4c 01 d3                    	addq	%r10, %rbx
140812214: f2 0f 10 46 20              	movsd	0x20(%rsi), %xmm0
140812219: f2 41 0f 10 4d 04           	movsd	0x4(%r13), %xmm1
14081221f: 0f 58 c8                    	addps	%xmm0, %xmm1
140812222: 0f 13 4c 24 30              	movlps	%xmm1, 0x30(%rsp)
140812227: 66 0f 6e 53 d0              	movd	-0x30(%rbx), %xmm2
14081222c: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
140812231: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140812236: e8 a5 ab 48 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14081223b: 8b 44 24 48                 	movl	0x48(%rsp), %eax
14081223f: 89 43 d4                    	movl	%eax, -0x2c(%rbx)
140812242: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
140812247: 48 89 43 cc                 	movq	%rax, -0x34(%rbx)
14081224b: e9 90 01 00 00              	jmp	0x1408123e0 <.text+0x8113e0>
140812250: 41 8b 6d 00                 	movl	(%r13), %ebp
140812254: 4c 89 f9                    	movq	%r15, %rcx
140812257: 89 ea                       	movl	%ebp, %edx
140812259: e8 a2 6f ab ff              	callq	0x1402c9200 <.text+0x2c8200>
14081225e: 49 83 7e 48 00              	cmpq	$0x0, 0x48(%r14)
140812263: 0f 84 77 01 00 00           	je	0x1408123e0 <.text+0x8113e0>
140812269: f3 41 0f 6f 46 50           	movdqu	0x50(%r14), %xmm0
14081226f: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
140812274: 66 0f ef ce                 	pxor	%xmm6, %xmm1
140812278: 66 0f 7f 4c 24 40           	movdqa	%xmm1, 0x40(%rsp)
14081227e: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
140812283: 66 0f ef cf                 	pxor	%xmm7, %xmm1
140812287: 66 0f 7f 4c 24 50           	movdqa	%xmm1, 0x50(%rsp)
14081228d: 66 0f 7f 44 24 60           	movdqa	%xmm0, 0x60(%rsp)
140812293: 45 0f 29 04 24              	movaps	%xmm8, (%r12)
140812298: 49 c7 44 24 10 00 00 00 00  	movq	$0x0, 0x10(%r12)
1408122a1: 89 6c 24 30                 	movl	%ebp, 0x30(%rsp)
1408122a5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1408122ab: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1408122b0: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
1408122b5: e8 a6 3e 8f ff              	callq	0x140106160 <.text+0x105160>
1408122ba: 4c 8b 4c 24 70              	movq	0x70(%rsp), %r9
1408122bf: 49 c1 e1 38                 	shlq	$0x38, %r9
1408122c3: 4c 0b 4c 24 78              	orq	0x78(%rsp), %r9
1408122c8: 48 8b 4c 24 50              	movq	0x50(%rsp), %rcx
1408122cd: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
1408122d2: 4c 31 c8                    	xorq	%r9, %rax
1408122d5: 4c 8b 44 24 40              	movq	0x40(%rsp), %r8
1408122da: 49 01 c8                    	addq	%rcx, %r8
1408122dd: 48 8b 54 24 48              	movq	0x48(%rsp), %rdx
1408122e2: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1408122e6: 48 01 c2                    	addq	%rax, %rdx
1408122e9: 4c 31 c1                    	xorq	%r8, %rcx
1408122ec: 48 c1 c0 10                 	rolq	$0x10, %rax
1408122f0: 48 31 d0                    	xorq	%rdx, %rax
1408122f3: 49 c1 c0 20                 	rolq	$0x20, %r8
1408122f7: 48 01 ca                    	addq	%rcx, %rdx
1408122fa: 49 01 c0                    	addq	%rax, %r8
1408122fd: 48 c1 c1 11                 	rolq	$0x11, %rcx
140812301: 48 31 d1                    	xorq	%rdx, %rcx
140812304: 48 c1 c0 15                 	rolq	$0x15, %rax
140812308: 48 c1 c2 20                 	rolq	$0x20, %rdx
14081230c: 4c 31 c0                    	xorq	%r8, %rax
14081230f: 4d 31 c8                    	xorq	%r9, %r8
140812312: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
140812319: 49 01 c8                    	addq	%rcx, %r8
14081231c: 48 01 c2                    	addq	%rax, %rdx
14081231f: 48 c1 c1 0d                 	rolq	$0xd, %rcx
140812323: 4c 31 c1                    	xorq	%r8, %rcx
140812326: 48 c1 c0 10                 	rolq	$0x10, %rax
14081232a: 49 c1 c0 20                 	rolq	$0x20, %r8
14081232e: 48 31 d0                    	xorq	%rdx, %rax
140812331: 48 01 ca                    	addq	%rcx, %rdx
140812334: 48 c1 c1 11                 	rolq	$0x11, %rcx
140812338: 49 01 c0                    	addq	%rax, %r8
14081233b: 48 31 d1                    	xorq	%rdx, %rcx
14081233e: 48 c1 c0 15                 	rolq	$0x15, %rax
140812342: 4c 31 c0                    	xorq	%r8, %rax
140812345: 48 c1 c2 20                 	rolq	$0x20, %rdx
140812349: 49 01 c8                    	addq	%rcx, %r8
14081234c: 48 01 c2                    	addq	%rax, %rdx
14081234f: 48 c1 c1 0d                 	rolq	$0xd, %rcx
140812353: 4c 31 c1                    	xorq	%r8, %rcx
140812356: 48 c1 c0 10                 	rolq	$0x10, %rax
14081235a: 49 c1 c0 20                 	rolq	$0x20, %r8
14081235e: 48 31 d0                    	xorq	%rdx, %rax
140812361: 48 01 ca                    	addq	%rcx, %rdx
140812364: 48 c1 c1 11                 	rolq	$0x11, %rcx
140812368: 49 01 c0                    	addq	%rax, %r8
14081236b: 48 31 d1                    	xorq	%rdx, %rcx
14081236e: 48 c1 c0 15                 	rolq	$0x15, %rax
140812372: 4c 31 c0                    	xorq	%r8, %rax
140812375: 48 c1 c2 20                 	rolq	$0x20, %rdx
140812379: 49 01 c8                    	addq	%rcx, %r8
14081237c: 48 01 c2                    	addq	%rax, %rdx
14081237f: 48 c1 c1 0d                 	rolq	$0xd, %rcx
140812383: 4c 31 c1                    	xorq	%r8, %rcx
140812386: 48 c1 c0 10                 	rolq	$0x10, %rax
14081238a: 48 31 d0                    	xorq	%rdx, %rax
14081238d: 48 01 ca                    	addq	%rcx, %rdx
140812390: 48 c1 c1 11                 	rolq	$0x11, %rcx
140812394: 48 c1 c0 15                 	rolq	$0x15, %rax
140812398: 49 89 d0                    	movq	%rdx, %r8
14081239b: 49 c1 c0 20                 	rolq	$0x20, %r8
14081239f: 49 31 c8                    	xorq	%rcx, %r8
1408123a2: 49 31 c0                    	xorq	%rax, %r8
1408123a5: 49 31 d0                    	xorq	%rdx, %r8
1408123a8: 4c 89 c1                    	movq	%r8, %rcx
1408123ab: 48 c1 e9 39                 	shrq	$0x39, %rcx
1408123af: 49 8b 5e 30                 	movq	0x30(%r14), %rbx
1408123b3: 49 8b 46 38                 	movq	0x38(%r14), %rax
1408123b7: 66 0f 6e c1                 	movd	%ecx, %xmm0
1408123bb: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1408123bf: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1408123c4: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1408123c9: 48 8d 4b b0                 	leaq	-0x50(%rbx), %rcx
1408123cd: 31 d2                       	xorl	%edx, %edx
1408123cf: e9 ac fd ff ff              	jmp	0x140812180 <.text+0x811180>
1408123d4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1408123e0: 49 83 c5 0c                 	addq	$0xc, %r13
1408123e4: 49 39 fd                    	cmpq	%rdi, %r13
1408123e7: 0f 85 63 fe ff ff           	jne	0x140812250 <.text+0x811250>
1408123ed: 48 8b 7c 24 28              	movq	0x28(%rsp), %rdi
1408123f2: 48 8b 84 24 b0 00 00 00     	movq	0xb0(%rsp), %rax
1408123fa: 8b 6c 24 3c                 	movl	0x3c(%rsp), %ebp
1408123fe: 4c 8b bc 24 98 00 00 00     	movq	0x98(%rsp), %r15
140812406: 4c 8b ac 24 90 00 00 00     	movq	0x90(%rsp), %r13
14081240e: e9 bd fc ff ff              	jmp	0x1408120d0 <.text+0x8110d0>
140812413: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
14081241b: 0f 28 bc 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm7
140812423: 44 0f 28 84 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm8
14081242c: 44 0f 28 8c 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm9
140812435: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
14081243c: 5b                          	popq	%rbx
14081243d: 5d                          	popq	%rbp
14081243e: 5f                          	popq	%rdi
14081243f: 5e                          	popq	%rsi
140812440: 41 5c                       	popq	%r12
140812442: 41 5d                       	popq	%r13
140812444: 41 5e                       	popq	%r14
140812446: 41 5f                       	popq	%r15
140812448: c3                          	retq
140812449: 48 83 c0 fe                 	addq	$-0x2, %rax
14081244d: 48 83 f8 02                 	cmpq	$0x2, %rax
140812451: 73 18                       	jae	0x14081246b <.text+0x81146b>
140812453: 48 8d 0d 5e c9 27 02        	leaq	0x227c95e(%rip), %rcx   # 0x142a8edb8
14081245a: 4c 8d 05 7f c9 27 02        	leaq	0x227c97f(%rip), %r8    # 0x142a8ede0
140812461: ba 28 00 00 00              	movl	$0x28, %edx
140812466: e8 c5 6f 0c 02              	callq	0x1428d9430 <.text+0x28d8430>
14081246b: 48 8d 05 1e c9 27 02        	leaq	0x227c91e(%rip), %rax   # 0x142a8ed90
140812472: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
140812477: 48 c7 44 24 48 01 00 00 00  	movq	$0x1, 0x48(%rsp)
140812480: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140812485: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
14081248a: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14081248e: f3 0f 7f 44 24 58           	movdqu	%xmm0, 0x58(%rsp)
140812494: 48 8d 15 05 c9 27 02        	leaq	0x227c905(%rip), %rdx   # 0x142a8eda0
14081249b: 48 8d 4c 24 40              	leaq	0x40(%rsp), %rcx
1408124a0: e8 eb 6e 0c 02              	callq	0x1428d9390 <.text+0x28d8390>
1408124a5: cc                          	int3
1408124a6: cc                          	int3
1408124a7: cc                          	int3
1408124a8: cc                          	int3
1408124a9: cc                          	int3
1408124aa: cc                          	int3
1408124ab: cc                          	int3
1408124ac: cc                          	int3
1408124ad: cc                          	int3
1408124ae: cc                          	int3
1408124af: cc                          	int3
1408124b0: 48 83 ec 68                 	subq	$0x68, %rsp
1408124b4: 48 89 d0                    	movq	%rdx, %rax
1408124b7: 80 39 1a                    	cmpb	$0x1a, (%rcx)
1408124ba: 75 19                       	jne	0x1408124d5 <.text+0x8114d5>
1408124bc: 48 8d 15 8f c9 27 02        	leaq	0x227c98f(%rip), %rdx   # 0x142a8ee52
1408124c3: 41 b8 16 00 00 00           	movl	$0x16, %r8d
1408124c9: 48 89 c1                    	movq	%rax, %rcx
1408124cc: 48 83 c4 68                 	addq	$0x68, %rsp
1408124d0: e9 bb 46 57 00              	jmp	0x140d86b90 <.text+0xd85b90>
1408124d5: 48 89 4c 24 60              	movq	%rcx, 0x60(%rsp)
1408124da: 48 ff c1                    	incq	%rcx
1408124dd: 48 8d 15 3c c9 27 02        	leaq	0x227c93c(%rip), %rdx   # 0x142a8ee20
1408124e4: 48 89 54 24 50              	movq	%rdx, 0x50(%rsp)
1408124e9: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
1408124ee: 48 89 54 24 48              	movq	%rdx, 0x48(%rsp)
1408124f3: 48 8d 15 51 c9 27 02        	leaq	0x227c951(%rip), %rdx   # 0x142a8ee4b
1408124fa: 48 89 54 24 38              	movq	%rdx, 0x38(%rsp)
1408124ff: 48 8d 15 fa c8 27 02        	leaq	0x227c8fa(%rip), %rdx   # 0x142a8ee00
140812506: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
14081250b: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140812510: 48 c7 44 24 40 07 00 00 00  	movq	$0x7, 0x40(%rsp)
140812519: 48 c7 44 24 20 04 00 00 00  	movq	$0x4, 0x20(%rsp)
140812522: 48 8d 15 17 c9 27 02        	leaq	0x227c917(%rip), %rdx   # 0x142a8ee40
140812529: 4c 8d 0d 17 c9 27 02        	leaq	0x227c917(%rip), %r9    # 0x142a8ee47
140812530: 41 b8 07 00 00 00           	movl	$0x7, %r8d
140812536: 48 89 c1                    	movq	%rax, %rcx
140812539: e8 62 47 57 00              	callq	0x140d86ca0 <.text+0xd85ca0>
14081253e: 90                          	nop
14081253f: 48 83 c4 68                 	addq	$0x68, %rsp
140812543: c3                          	retq
140812544: cc                          	int3
140812545: cc                          	int3
140812546: cc                          	int3
140812547: cc                          	int3
140812548: cc                          	int3
140812549: cc                          	int3
14081254a: cc                          	int3
14081254b: cc                          	int3
14081254c: cc                          	int3
14081254d: cc                          	int3
14081254e: cc                          	int3
14081254f: cc                          	int3
140812550: 48 89 d0                    	movq	%rdx, %rax
140812553: 0f b6 09                    	movzbl	(%rcx), %ecx
140812556: 48 8d 15 fb d2 27 02        	leaq	0x227d2fb(%rip), %rdx   # 0x142a8f858
14081255d: 4c 8b 04 ca                 	movq	(%rdx,%rcx,8), %r8
140812561: 4c 8d 0d c0 d3 27 02        	leaq	0x227d3c0(%rip), %r9    # 0x142a8f928
140812568: 49 63 14 89                 	movslq	(%r9,%rcx,4), %rdx
14081256c: 4c 01 ca                    	addq	%r9, %rdx
14081256f: 48 89 c1                    	movq	%rax, %rcx
140812572: e9 19 46 57 00              	jmp	0x140d86b90 <.text+0xd85b90>
140812577: cc                          	int3
140812578: cc                          	int3
140812579: cc                          	int3
14081257a: cc                          	int3
14081257b: cc                          	int3
14081257c: cc                          	int3
14081257d: cc                          	int3
14081257e: cc                          	int3
14081257f: cc                          	int3
140812580: 48 83 ec 38                 	subq	$0x38, %rsp
140812584: 48 89 d0                    	movq	%rdx, %rax
140812587: 48 8b 11                    	movq	(%rcx), %rdx
14081258a: 48 85 d2                    	testq	%rdx, %rdx
14081258d: 74 1e                       	je	0x1408125ad <.text+0x8115ad>
14081258f: 83 fa 01                    	cmpl	$0x1, %edx
140812592: 75 4e                       	jne	0x1408125e2 <.text+0x8115e2>
140812594: 48 8d 15 51 ca 27 02        	leaq	0x227ca51(%rip), %rdx   # 0x142a8efec
14081259b: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1408125a1: 48 89 c1                    	movq	%rax, %rcx
1408125a4: 48 83 c4 38                 	addq	$0x38, %rsp
1408125a8: e9 e3 45 57 00              	jmp	0x140d86b90 <.text+0xd85b90>
1408125ad: 48 83 c1 08                 	addq	$0x8, %rcx
1408125b1: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
1408125b6: 48 8d 0d 0b ca 27 02        	leaq	0x227ca0b(%rip), %rcx   # 0x142a8efc8
1408125bd: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1408125c2: 48 8d 15 1f ca 27 02        	leaq	0x227ca1f(%rip), %rdx   # 0x142a8efe8
1408125c9: 4c 8d 4c 24 30              	leaq	0x30(%rsp), %r9
1408125ce: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1408125d4: 48 89 c1                    	movq	%rax, %rcx
1408125d7: e8 f4 4c 57 00              	callq	0x140d872d0 <.text+0xd862d0>
1408125dc: 90                          	nop
1408125dd: 48 83 c4 38                 	addq	$0x38, %rsp
1408125e1: c3                          	retq
1408125e2: 48 8d 15 d2 c9 27 02        	leaq	0x227c9d2(%rip), %rdx   # 0x142a8efbb
1408125e9: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1408125ef: 48 89 c1                    	movq	%rax, %rcx
1408125f2: 48 83 c4 38                 	addq	$0x38, %rsp
1408125f6: e9 95 45 57 00              	jmp	0x140d86b90 <.text+0xd85b90>
1408125fb: cc                          	int3
1408125fc: cc                          	int3
1408125fd: cc                          	int3
1408125fe: cc                          	int3
1408125ff: cc                          	int3
