
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412b6420: 55                          	pushq	%rbp
1412b6421: 41 57                       	pushq	%r15
1412b6423: 41 56                       	pushq	%r14
1412b6425: 41 55                       	pushq	%r13
1412b6427: 41 54                       	pushq	%r12
1412b6429: 56                          	pushq	%rsi
1412b642a: 57                          	pushq	%rdi
1412b642b: 53                          	pushq	%rbx
1412b642c: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
1412b6433: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412b643b: 66 0f 7f bd 30 01 00 00     	movdqa	%xmm7, 0x130(%rbp)
1412b6443: 66 0f 7f b5 20 01 00 00     	movdqa	%xmm6, 0x120(%rbp)
1412b644b: 48 c7 85 18 01 00 00 fe ff ff ff    	movq	$-0x2, 0x118(%rbp)
1412b6456: 4c 89 8d e0 00 00 00        	movq	%r9, 0xe0(%rbp)
1412b645d: 4c 89 c3                    	movq	%r8, %rbx
1412b6460: 49 89 d6                    	movq	%rdx, %r14
1412b6463: 48 89 cf                    	movq	%rcx, %rdi
1412b6466: e8 d5 72 a9 ff              	callq	0x140d4d740 <.text+0xd4c740>
1412b646b: 48 89 c6                    	movq	%rax, %rsi
1412b646e: f6 00 01                    	testb	$0x1, (%rax)
1412b6471: 4c 89 b5 88 00 00 00        	movq	%r14, 0x88(%rbp)
1412b6478: 74 11                       	je	0x1412b648b <.text+0x12b548b>
1412b647a: 48 89 f1                    	movq	%rsi, %rcx
1412b647d: 48 83 c1 08                 	addq	$0x8, %rcx
1412b6481: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1412b6485: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
1412b6489: eb 1b                       	jmp	0x1412b64a6 <.text+0x12b54a6>
1412b648b: e8 70 e9 aa ff              	callq	0x140d64e00 <.text+0xd63e00>
1412b6490: 48 89 f1                    	movq	%rsi, %rcx
1412b6493: 48 83 c1 08                 	addq	$0x8, %rcx
1412b6497: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1412b649e: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1412b64a2: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
1412b64a6: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1412b64aa: 4c 89 01                    	movq	%r8, (%rcx)
1412b64ad: 0f 10 05 7c 59 97 01        	movups	0x197597c(%rip), %xmm0  # 0x142c2be30
1412b64b4: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
1412b64b8: f3 0f 6f 05 60 59 97 01     	movdqu	0x1975960(%rip), %xmm0  # 0x142c2be20
1412b64c0: 66 0f 7f 45 30              	movdqa	%xmm0, 0x30(%rbp)
1412b64c5: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1412b64c9: 48 89 55 58                 	movq	%rdx, 0x58(%rbp)
1412b64cd: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
1412b64d4: 48 8b 13                    	movq	(%rbx), %rdx
1412b64d7: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b64de: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1412b64e2: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
1412b64e6: e8 d5 7d 76 ff              	callq	0x140a1e2c0 <.text+0xa1d2c0>
1412b64eb: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1412b64f2: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
1412b64f6: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
1412b64fd: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1412b6501: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1412b6508: 48 8b 75 d0                 	movq	-0x30(%rbp), %rsi
1412b650c: 4c 8b 65 d8                 	movq	-0x28(%rbp), %r12
1412b6510: 0f b7 4d e8                 	movzwl	-0x18(%rbp), %ecx
1412b6514: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
1412b651b: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1412b651f: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1412b6526: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
1412b652a: 4c 8b 6d 00                 	movq	(%rbp), %r13
1412b652e: 48 8b 0f                    	movq	(%rdi), %rcx
1412b6531: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
1412b6538: 48 8b 00                    	movq	(%rax), %rax
1412b653b: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1412b6542: 66 0f 6e 35 4e e1 67 01     	movd	0x167e14e(%rip), %xmm6  # 0x142934698
1412b654a: f3 0f 7e 3d ae 58 97 01     	movq	0x19758ae(%rip), %xmm7  # 0x142c2be00
1412b6552: 0f b7 7d 10                 	movzwl	0x10(%rbp), %edi
1412b6556: 48 8b 45 18                 	movq	0x18(%rbp), %rax
1412b655a: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
1412b6561: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1412b6568: 48 8b 00                    	movq	(%rax), %rax
1412b656b: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
1412b6572: eb 54                       	jmp	0x1412b65c8 <.text+0x12b55c8>
1412b6574: 66 0f 6f c6                 	movdqa	%xmm6, %xmm0
1412b6578: f3 0f 7e 8d d4 00 00 00     	movq	0xd4(%rbp), %xmm1
1412b6580: 66 0f 6e d0                 	movd	%eax, %xmm2
1412b6584: 66 0f 70 d2 50              	pshufd	$0x50, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,1]
1412b6589: 66 0f 72 f2 1f              	pslld	$0x1f, %xmm2
1412b658e: 66 0f 72 e2 1f              	psrad	$0x1f, %xmm2
1412b6593: 66 0f db ca                 	pand	%xmm2, %xmm1
1412b6597: 66 0f df d7                 	pandn	%xmm7, %xmm2
1412b659b: 66 0f eb d1                 	por	%xmm1, %xmm2
1412b659f: 66 0f d6 55 70              	movq	%xmm2, 0x70(%rbp)
1412b65a4: 66 0f 7e 45 78              	movd	%xmm0, 0x78(%rbp)
1412b65a9: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b65b0: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
1412b65b7: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
1412b65bb: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
1412b65bf: 4c 8d 4d 70                 	leaq	0x70(%rbp), %r9
1412b65c3: e8 48 f1 fa ff              	callq	0x141265710 <.text+0x1264710>
1412b65c8: f6 85 b0 00 00 00 01        	testb	$0x1, 0xb0(%rbp)
1412b65cf: 74 3f                       	je	0x1412b6610 <.text+0x12b5610>
1412b65d1: 48 85 f6                    	testq	%rsi, %rsi
1412b65d4: 74 15                       	je	0x1412b65eb <.text+0x12b55eb>
1412b65d6: 48 83 bd 08 01 00 00 00     	cmpq	$0x0, 0x108(%rbp)
1412b65de: 75 3f                       	jne	0x1412b661f <.text+0x12b561f>
1412b65e0: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
1412b65eb: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
1412b65f2: 48 85 c0                    	testq	%rax, %rax
1412b65f5: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1412b65fc: 0f 85 d0 00 00 00           	jne	0x1412b66d2 <.text+0x12b56d2>
1412b6602: 31 c0                       	xorl	%eax, %eax
1412b6604: 48 85 db                    	testq	%rbx, %rbx
1412b6607: 75 65                       	jne	0x1412b666e <.text+0x12b566e>
1412b6609: e9 5e 02 00 00              	jmp	0x1412b686c <.text+0x12b586c>
1412b660e: 66 90                       	nop
1412b6610: 48 85 f6                    	testq	%rsi, %rsi
1412b6613: 74 42                       	je	0x1412b6657 <.text+0x12b5657>
1412b6615: 48 83 bd 08 01 00 00 00     	cmpq	$0x0, 0x108(%rbp)
1412b661d: 74 2d                       	je	0x1412b664c <.text+0x12b564c>
1412b661f: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1412b6626: 66 85 d2                    	testw	%dx, %dx
1412b6629: 0f 84 e1 00 00 00           	je	0x1412b6710 <.text+0x12b5710>
1412b662f: 8d 4a ff                    	leal	-0x1(%rdx), %ecx
1412b6632: 21 d1                       	andl	%edx, %ecx
1412b6634: 48 ff 8d 08 01 00 00        	decq	0x108(%rbp)
1412b663b: 89 d0                       	movl	%edx, %eax
1412b663d: 48 89 f2                    	movq	%rsi, %rdx
1412b6640: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
1412b6647: e9 04 01 00 00              	jmp	0x1412b6750 <.text+0x12b5750>
1412b664c: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
1412b6657: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1412b665e: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
1412b6665: 48 85 db                    	testq	%rbx, %rbx
1412b6668: 0f 84 fe 01 00 00           	je	0x1412b686c <.text+0x12b586c>
1412b666e: 48 85 d2                    	testq	%rdx, %rdx
1412b6671: 0f 84 f5 01 00 00           	je	0x1412b686c <.text+0x12b586c>
1412b6677: 66 85 ff                    	testw	%di, %di
1412b667a: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1412b6681: 74 0d                       	je	0x1412b6690 <.text+0x12b5690>
1412b6683: 8d 4f ff                    	leal	-0x1(%rdi), %ecx
1412b6686: 21 f9                       	andl	%edi, %ecx
1412b6688: eb 30                       	jmp	0x1412b66ba <.text+0x12b56ba>
1412b668a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412b6690: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
1412b6696: 66 0f d7 f8                 	pmovmskb	%xmm0, %edi
1412b669a: 48 81 c3 00 fb ff ff        	addq	$-0x500, %rbx           # imm = 0xFB00
1412b66a1: 49 83 c5 10                 	addq	$0x10, %r13
1412b66a5: 81 ff ff ff 00 00           	cmpl	$0xffff, %edi           # imm = 0xFFFF
1412b66ab: 74 e3                       	je	0x1412b6690 <.text+0x12b5690>
1412b66ad: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1412b66b2: 29 f8                       	subl	%edi, %eax
1412b66b4: f7 d7                       	notl	%edi
1412b66b6: 89 f9                       	movl	%edi, %ecx
1412b66b8: 21 c1                       	andl	%eax, %ecx
1412b66ba: 48 ff ca                    	decq	%rdx
1412b66bd: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
1412b66c4: 89 f8                       	movl	%edi, %eax
1412b66c6: 48 89 da                    	movq	%rbx, %rdx
1412b66c9: 89 cf                       	movl	%ecx, %edi
1412b66cb: 31 f6                       	xorl	%esi, %esi
1412b66cd: e9 7e 00 00 00              	jmp	0x1412b6750 <.text+0x12b5750>
1412b66d2: 48 8b 70 30                 	movq	0x30(%rax), %rsi
1412b66d6: 48 8b 48 48                 	movq	0x48(%rax), %rcx
1412b66da: 66 0f 6f 06                 	movdqa	(%rsi), %xmm0
1412b66de: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
1412b66e2: f7 d2                       	notl	%edx
1412b66e4: 4c 8d 66 10                 	leaq	0x10(%rsi), %r12
1412b66e8: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1412b66ef: 48 85 c9                    	testq	%rcx, %rcx
1412b66f2: 0f 84 50 01 00 00           	je	0x1412b6848 <.text+0x12b5848>
1412b66f8: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
1412b6703: 66 85 d2                    	testw	%dx, %dx
1412b6706: 0f 85 23 ff ff ff           	jne	0x1412b662f <.text+0x12b562f>
1412b670c: 0f 1f 40 00                 	nopl	(%rax)
1412b6710: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
1412b6716: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1412b671a: 48 81 c6 00 fb ff ff        	addq	$-0x500, %rsi           # imm = 0xFB00
1412b6721: 49 83 c4 10                 	addq	$0x10, %r12
1412b6725: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1412b672a: 74 e4                       	je	0x1412b6710 <.text+0x12b5710>
1412b672c: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1412b6731: 29 c1                       	subl	%eax, %ecx
1412b6733: f7 d0                       	notl	%eax
1412b6735: 89 c2                       	movl	%eax, %edx
1412b6737: 21 ca                       	andl	%ecx, %edx
1412b6739: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
1412b6740: 48 ff 8d 08 01 00 00        	decq	0x108(%rbp)
1412b6747: 48 89 f2                    	movq	%rsi, %rdx
1412b674a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412b6750: f3 0f bc c0                 	tzcntl	%eax, %eax
1412b6754: 48 f7 d8                    	negq	%rax
1412b6757: 4c 8d 3c 80                 	leaq	(%rax,%rax,4), %r15
1412b675b: 49 c1 e7 04                 	shlq	$0x4, %r15
1412b675f: 49 89 d6                    	movq	%rdx, %r14
1412b6762: 42 8b 54 3a b0              	movl	-0x50(%rdx,%r15), %edx
1412b6767: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b676e: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
1412b6775: 89 95 a8 00 00 00           	movl	%edx, 0xa8(%rbp)
1412b677b: e8 10 56 87 ff              	callq	0x140b2bd90 <.text+0xb2ad90>
1412b6780: 84 c0                       	testb	%al, %al
1412b6782: 0f 84 40 fe ff ff           	je	0x1412b65c8 <.text+0x12b55c8>
1412b6788: 4b 8d 0c 3e                 	leaq	(%r14,%r15), %rcx
1412b678c: 48 83 c1 b8                 	addq	$-0x48, %rcx
1412b6790: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b6797: 48 8d 15 42 57 97 01        	leaq	0x1975742(%rip), %rdx   # 0x142c2bee0
1412b679e: e8 cd cc 57 ff              	callq	0x140833470 <.text+0x832470>
1412b67a3: f3 0f 7e 00                 	movq	(%rax), %xmm0
1412b67a7: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
1412b67ae: 48 8b 41 08                 	movq	0x8(%rcx), %rax
1412b67b2: 48 69 49 10 50 01 00 00     	imulq	$0x150, 0x10(%rcx), %rcx # imm = 0x150
1412b67ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412b67c0: 48 85 c9                    	testq	%rcx, %rcx
1412b67c3: 74 3b                       	je	0x1412b6800 <.text+0x12b5800>
1412b67c5: f3 0f 7e 88 28 01 00 00     	movq	0x128(%rax), %xmm1
1412b67cd: 48 05 50 01 00 00           	addq	$0x150, %rax            # imm = 0x150
1412b67d3: 66 0f 76 c8                 	pcmpeqd	%xmm0, %xmm1
1412b67d7: 66 0f 70 c9 50              	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
1412b67dc: 66 0f 50 d1                 	movmskpd	%xmm1, %edx
1412b67e0: 48 81 c1 b0 fe ff ff        	addq	$-0x150, %rcx           # imm = 0xFEB0
1412b67e7: 83 fa 03                    	cmpl	$0x3, %edx
1412b67ea: 75 d4                       	jne	0x1412b67c0 <.text+0x12b57c0>
1412b67ec: e9 d7 fd ff ff              	jmp	0x1412b65c8 <.text+0x12b55c8>
1412b67f1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1412b6800: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
1412b6808: 44 8b 85 a8 00 00 00        	movl	0xa8(%rbp), %r8d
1412b680f: 44 89 45 d0                 	movl	%r8d, -0x30(%rbp)
1412b6813: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
1412b681a: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
1412b6821: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
1412b6828: e8 53 43 76 ff              	callq	0x140a1ab80 <.text+0xa19b80>
1412b682d: 8b 85 d0 00 00 00           	movl	0xd0(%rbp), %eax
1412b6833: 85 c0                       	testl	%eax, %eax
1412b6835: 0f 84 39 fd ff ff           	je	0x1412b6574 <.text+0x12b5574>
1412b683b: 66 0f 6e 85 dc 00 00 00     	movd	0xdc(%rbp), %xmm0
1412b6843: e9 30 fd ff ff              	jmp	0x1412b6578 <.text+0x12b5578>
1412b6848: 48 89 95 00 01 00 00        	movq	%rdx, 0x100(%rbp)
1412b684f: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
1412b685a: 31 c0                       	xorl	%eax, %eax
1412b685c: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
1412b6863: 48 85 db                    	testq	%rbx, %rbx
1412b6866: 0f 85 02 fe ff ff           	jne	0x1412b666e <.text+0x12b566e>
1412b686c: 48 8b 7d 30                 	movq	0x30(%rbp), %rdi
1412b6870: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
1412b6874: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
1412b6878: 4d 85 c0                    	testq	%r8, %r8
1412b687b: 4c 89 85 90 00 00 00        	movq	%r8, 0x90(%rbp)
1412b6882: 74 3e                       	je	0x1412b68c2 <.text+0x12b58c2>
1412b6884: 49 8d 40 01                 	leaq	0x1(%r8), %rax
1412b6888: b9 28 00 00 00              	movl	$0x28, %ecx
1412b688d: 48 f7 e1                    	mulq	%rcx
1412b6890: 48 83 c0 0f                 	addq	$0xf, %rax
1412b6894: 48 83 e0 f0                 	andq	$-0x10, %rax
1412b6898: 49 8d 0c 00                 	leaq	(%r8,%rax), %rcx
1412b689c: 48 83 c1 11                 	addq	$0x11, %rcx
1412b68a0: 48 89 8d e8 00 00 00        	movq	%rcx, 0xe8(%rbp)
1412b68a7: 48 89 f9                    	movq	%rdi, %rcx
1412b68aa: 48 29 c1                    	subq	%rax, %rcx
1412b68ad: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
1412b68b4: b8 10 00 00 00              	movl	$0x10, %eax
1412b68b9: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1412b68c0: eb 0b                       	jmp	0x1412b68cd <.text+0x12b58cd>
1412b68c2: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
1412b68cd: 48 8b 8d 80 00 00 00        	movq	0x80(%rbp), %rcx
1412b68d4: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1412b68db: 4c 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %r8
1412b68e2: 4c 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %r9
1412b68e9: 4c 8b 6d 48                 	movq	0x48(%rbp), %r13
1412b68ed: 4c 8d 77 10                 	leaq	0x10(%rdi), %r14
1412b68f1: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1412b68f5: f7 d0                       	notl	%eax
1412b68f7: 45 8b 51 1c                 	movl	0x1c(%r9), %r10d
1412b68fb: 44 89 95 08 01 00 00        	movl	%r10d, 0x108(%rbp)
1412b6902: 49 8b 19                    	movq	(%r9), %rbx
1412b6905: 4d 8b 49 10                 	movq	0x10(%r9), %r9
1412b6909: 4c 89 8d a8 00 00 00        	movq	%r9, 0xa8(%rbp)
1412b6910: 48 8b 12                    	movq	(%rdx), %rdx
1412b6913: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
1412b691a: 8b 51 1c                    	movl	0x1c(%rcx), %edx
1412b691d: 89 95 b0 00 00 00           	movl	%edx, 0xb0(%rbp)
1412b6923: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1412b6927: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
1412b692e: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1412b6935: 8b 51 1c                    	movl	0x1c(%rcx), %edx
1412b6938: 89 95 b8 00 00 00           	movl	%edx, 0xb8(%rbp)
1412b693e: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1412b6942: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
1412b6949: 49 8b 08                    	movq	(%r8), %rcx
1412b694c: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
1412b6953: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
1412b6957: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
1412b695e: 4c 8d a5 d0 00 00 00        	leaq	0xd0(%rbp), %r12
1412b6965: 49 83 ed 01                 	subq	$0x1, %r13
1412b6969: 0f 82 6f 01 00 00           	jb	0x1412b6ade <.text+0x12b5ade>
1412b696f: 90                          	nop
1412b6970: 66 85 c0                    	testw	%ax, %ax
1412b6973: 74 0b                       	je	0x1412b6980 <.text+0x12b5980>
1412b6975: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
1412b6979: 41 21 c7                    	andl	%eax, %r15d
1412b697c: eb 2c                       	jmp	0x1412b69aa <.text+0x12b59aa>
1412b697e: 66 90                       	nop
1412b6980: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
1412b6985: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1412b6989: 48 81 c7 80 fd ff ff        	addq	$-0x280, %rdi           # imm = 0xFD80
1412b6990: 49 83 c6 10                 	addq	$0x10, %r14
1412b6994: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1412b6999: 74 e5                       	je	0x1412b6980 <.text+0x12b5980>
1412b699b: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1412b69a0: 29 c1                       	subl	%eax, %ecx
1412b69a2: f7 d0                       	notl	%eax
1412b69a4: 41 89 c7                    	movl	%eax, %r15d
1412b69a7: 41 21 cf                    	andl	%ecx, %r15d
1412b69aa: f3 0f bc c0                 	tzcntl	%eax, %eax
1412b69ae: 48 f7 d8                    	negq	%rax
1412b69b1: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1412b69b5: f3 0f 6f 74 c7 d8           	movdqu	-0x28(%rdi,%rax,8), %xmm6
1412b69bb: 48 8b 4c c7 d8              	movq	-0x28(%rdi,%rax,8), %rcx
1412b69c0: 8b 74 c7 e8                 	movl	-0x18(%rdi,%rax,8), %esi
1412b69c4: f3 0f 6f 44 c7 ec           	movdqu	-0x14(%rdi,%rax,8), %xmm0
1412b69ca: 66 0f 7f 45 70              	movdqa	%xmm0, 0x70(%rbp)
1412b69cf: 48 83 f9 03                 	cmpq	$0x3, %rcx
1412b69d3: 0f 84 05 01 00 00           	je	0x1412b6ade <.text+0x12b5ade>
1412b69d9: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
1412b69e0: 89 f2                       	movl	%esi, %edx
1412b69e2: e8 a9 53 87 ff              	callq	0x140b2bd90 <.text+0xb2ad90>
1412b69e7: 84 c0                       	testb	%al, %al
1412b69e9: 0f 84 3f 01 00 00           	je	0x1412b6b2e <.text+0x12b5b2e>
1412b69ef: 8b 85 08 01 00 00           	movl	0x108(%rbp), %eax
1412b69f5: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
1412b69fc: 89 01                       	movl	%eax, (%rcx)
1412b69fe: 89 b5 d0 00 00 00           	movl	%esi, 0xd0(%rbp)
1412b6a04: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1412b6a0b: 4c 89 e2                    	movq	%r12, %rdx
1412b6a0e: e8 bd a4 2f ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1412b6a13: 48 89 d9                    	movq	%rbx, %rcx
1412b6a16: 48 89 c2                    	movq	%rax, %rdx
1412b6a19: 4d 89 e0                    	movq	%r12, %r8
1412b6a1c: e8 ff dd fa ff              	callq	0x141264820 <.text+0x1263820>
1412b6a21: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
1412b6a25: 48 85 d2                    	testq	%rdx, %rdx
1412b6a28: 74 0f                       	je	0x1412b6a39 <.text+0x12b5a39>
1412b6a2a: f0                          	lock
1412b6a2b: 48 ff 0a                    	decq	(%rdx)
1412b6a2e: 75 09                       	jne	0x1412b6a39 <.text+0x12b5a39>
1412b6a30: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1412b6a34: e8 67 47 57 ff              	callq	0x14082b1a0 <.text+0x82a1a0>
1412b6a39: f3 0f 7f 75 c8              	movdqu	%xmm6, -0x38(%rbp)
1412b6a3e: 89 75 d8                    	movl	%esi, -0x28(%rbp)
1412b6a41: 48 c7 45 c0 04 00 00 00     	movq	$0x4, -0x40(%rbp)
1412b6a49: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1412b6a50: e8 1b ff 7c ff              	callq	0x140a86970 <.text+0xa85970>
1412b6a55: 34 01                       	xorb	$0x1, %al
1412b6a57: 48 8d 55 74                 	leaq	0x74(%rbp), %rdx
1412b6a5b: 8b 4a 08                    	movl	0x8(%rdx), %ecx
1412b6a5e: 89 8d d8 00 00 00           	movl	%ecx, 0xd8(%rbp)
1412b6a64: 48 8b 0a                    	movq	(%rdx), %rcx
1412b6a67: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
1412b6a6e: 88 85 dc 00 00 00           	movb	%al, 0xdc(%rbp)
1412b6a74: 8b 85 b0 00 00 00           	movl	0xb0(%rbp), %eax
1412b6a7a: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
1412b6a81: 89 01                       	movl	%eax, (%rcx)
1412b6a83: 8b 85 b8 00 00 00           	movl	0xb8(%rbp), %eax
1412b6a89: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1412b6a90: 89 01                       	movl	%eax, (%rcx)
1412b6a92: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1412b6a99: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1412b6a9e: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1412b6aa5: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1412b6aaa: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
1412b6ab1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1412b6ab6: 48 c7 44 24 28 00 00 00 00  	movq	$0x0, 0x28(%rsp)
1412b6abf: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1412b6ac3: 31 d2                       	xorl	%edx, %edx
1412b6ac5: 4d 89 e0                    	movq	%r12, %r8
1412b6ac8: 4c 8b 4d 68                 	movq	0x68(%rbp), %r9
1412b6acc: e8 5f 03 fc ff              	callq	0x141276e30 <.text+0x1275e30>
1412b6ad1: 44 89 f8                    	movl	%r15d, %eax
1412b6ad4: 49 83 ed 01                 	subq	$0x1, %r13
1412b6ad8: 0f 83 92 fe ff ff           	jae	0x1412b6970 <.text+0x12b5970>
1412b6ade: 48 83 bd 90 00 00 00 00     	cmpq	$0x0, 0x90(%rbp)
1412b6ae6: 74 24                       	je	0x1412b6b0c <.text+0x12b5b0c>
1412b6ae8: 48 83 bd e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rbp)
1412b6af0: 74 1a                       	je	0x1412b6b0c <.text+0x12b5b0c>
1412b6af2: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1412b6af9: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1412b6b00: 4c 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %r8
1412b6b07: e8 14 d1 35 ff              	callq	0x140613c20 <.text+0x612c20>
1412b6b0c: 0f 28 b5 20 01 00 00        	movaps	0x120(%rbp), %xmm6
1412b6b13: 0f 28 bd 30 01 00 00        	movaps	0x130(%rbp), %xmm7
1412b6b1a: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
1412b6b21: 5b                          	popq	%rbx
1412b6b22: 5f                          	popq	%rdi
1412b6b23: 5e                          	popq	%rsi
1412b6b24: 41 5c                       	popq	%r12
1412b6b26: 41 5d                       	popq	%r13
1412b6b28: 41 5e                       	popq	%r14
1412b6b2a: 41 5f                       	popq	%r15
1412b6b2c: 5d                          	popq	%rbp
1412b6b2d: c3                          	retq
1412b6b2e: 48 8d 0d 0b 53 97 01        	leaq	0x197530b(%rip), %rcx   # 0x142c2be40
1412b6b35: 4c 8d 05 8c 53 97 01        	leaq	0x197538c(%rip), %r8    # 0x142c2bec8
1412b6b3c: ba 52 00 00 00              	movl	$0x52, %edx
1412b6b41: e8 ea 28 62 01              	callq	0x1428d9430 <.text+0x28d8430>
1412b6b46: 0f 0b                       	ud2
