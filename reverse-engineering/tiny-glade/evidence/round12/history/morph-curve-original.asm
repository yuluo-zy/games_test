
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140952570: 55                          	pushq	%rbp
140952571: 41 57                       	pushq	%r15
140952573: 41 56                       	pushq	%r14
140952575: 41 55                       	pushq	%r13
140952577: 41 54                       	pushq	%r12
140952579: 56                          	pushq	%rsi
14095257a: 57                          	pushq	%rdi
14095257b: 53                          	pushq	%rbx
14095257c: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
140952583: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14095258b: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
140952593: 4c 89 c3                    	movq	%r8, %rbx
140952596: 49 89 d5                    	movq	%rdx, %r13
140952599: 48 89 cf                    	movq	%rcx, %rdi
14095259c: f3 0f 10 5d 70              	movss	0x70(%rbp), %xmm3
1409525a1: 0f 57 c0                    	xorps	%xmm0, %xmm0
1409525a4: 0f 2e d8                    	ucomiss	%xmm0, %xmm3
1409525a7: 75 6b                       	jne	0x140952614 <.text+0x951614>
1409525a9: 7a 69                       	jp	0x140952614 <.text+0x951614>
1409525ab: 48 8b 73 10                 	movq	0x10(%rbx), %rsi
1409525af: 4c 8d 34 f5 00 00 00 00     	leaq	(,%rsi,8), %r14
1409525b7: 45 31 e4                    	xorl	%r12d, %r12d
1409525ba: 48 89 f0                    	movq	%rsi, %rax
1409525bd: 48 c1 e8 3d                 	shrq	$0x3d, %rax
1409525c1: 0f 85 62 01 00 00           	jne	0x140952729 <.text+0x951729>
1409525c7: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
1409525d1: 49 39 c6                    	cmpq	%rax, %r14
1409525d4: 0f 87 4f 01 00 00           	ja	0x140952729 <.text+0x951729>
1409525da: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
1409525de: 4d 85 f6                    	testq	%r14, %r14
1409525e1: 48 89 7d e0                 	movq	%rdi, -0x20(%rbp)
1409525e5: 4c 89 6d e8                 	movq	%r13, -0x18(%rbp)
1409525e9: 74 55                       	je	0x140952640 <.text+0x951640>
1409525eb: 0f b6 05 17 26 ce 02        	movzbl	0x2ce2617(%rip), %eax   # 0x143634c09
1409525f2: 41 bc 04 00 00 00           	movl	$0x4, %r12d
1409525f8: ba 04 00 00 00              	movl	$0x4, %edx
1409525fd: 4c 89 f1                    	movq	%r14, %rcx
140952600: e8 0b 16 cc ff              	callq	0x140613c10 <.text+0x612c10>
140952605: 48 85 c0                    	testq	%rax, %rax
140952608: 0f 84 1b 01 00 00           	je	0x140952729 <.text+0x951729>
14095260e: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
140952612: eb 39                       	jmp	0x14095264d <.text+0x95164d>
140952614: 48 8d 05 e5 1d 16 02        	leaq	0x2161de5(%rip), %rax   # 0x142ab4400
14095261b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140952620: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
140952624: 4c 89 f1                    	movq	%r14, %rcx
140952627: 48 89 da                    	movq	%rbx, %rdx
14095262a: 4d 89 c8                    	movq	%r9, %r8
14095262d: e8 0e 44 b1 00              	callq	0x141466a40 <.text+0x1465a40>
140952632: 4c 89 e9                    	movq	%r13, %rcx
140952635: 48 89 fa                    	movq	%rdi, %rdx
140952638: 4d 89 f0                    	movq	%r14, %r8
14095263b: e9 cf 00 00 00              	jmp	0x14095270f <.text+0x95170f>
140952640: b8 04 00 00 00              	movl	$0x4, %eax
140952645: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
14095264d: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140952651: 48 89 c1                    	movq	%rax, %rcx
140952654: 4c 89 fa                    	movq	%r15, %rdx
140952657: 4d 89 f0                    	movq	%r14, %r8
14095265a: e8 a1 15 fd 01              	callq	0x142923c00 <.text+0x2922c00>
14095265f: 48 8b 7b 28                 	movq	0x28(%rbx), %rdi
140952663: 4c 8d 34 bd 00 00 00 00     	leaq	(,%rdi,4), %r14
14095266b: 45 31 ed                    	xorl	%r13d, %r13d
14095266e: 48 89 f8                    	movq	%rdi, %rax
140952671: 48 c1 e8 3e                 	shrq	$0x3e, %rax
140952675: 0f 85 c0 00 00 00           	jne	0x14095273b <.text+0x95173b>
14095267b: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140952685: 49 39 c6                    	cmpq	%rax, %r14
140952688: 0f 87 ad 00 00 00           	ja	0x14095273b <.text+0x95173b>
14095268e: 4c 8b 7b 20                 	movq	0x20(%rbx), %r15
140952692: 4d 85 f6                    	testq	%r14, %r14
140952695: 74 2b                       	je	0x1409526c2 <.text+0x9516c2>
140952697: 0f b6 05 6b 25 ce 02        	movzbl	0x2ce256b(%rip), %eax   # 0x143634c09
14095269e: 41 bd 04 00 00 00           	movl	$0x4, %r13d
1409526a4: ba 04 00 00 00              	movl	$0x4, %edx
1409526a9: 4c 89 f1                    	movq	%r14, %rcx
1409526ac: e8 5f 15 cc ff              	callq	0x140613c10 <.text+0x612c10>
1409526b1: 48 85 c0                    	testq	%rax, %rax
1409526b4: 0f 84 81 00 00 00           	je	0x14095273b <.text+0x95173b>
1409526ba: 49 89 c4                    	movq	%rax, %r12
1409526bd: 49 89 fd                    	movq	%rdi, %r13
1409526c0: eb 09                       	jmp	0x1409526cb <.text+0x9516cb>
1409526c2: 41 bc 04 00 00 00           	movl	$0x4, %r12d
1409526c8: 45 31 ed                    	xorl	%r13d, %r13d
1409526cb: 4c 89 e1                    	movq	%r12, %rcx
1409526ce: 4c 89 fa                    	movq	%r15, %rdx
1409526d1: 4d 89 f0                    	movq	%r14, %r8
1409526d4: e8 27 15 fd 01              	callq	0x142923c00 <.text+0x2922c00>
1409526d9: f3 0f 10 43 30              	movss	0x30(%rbx), %xmm0
1409526de: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1409526e2: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
1409526e6: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1409526ea: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1409526ee: 48 89 75 b8                 	movq	%rsi, -0x48(%rbp)
1409526f2: 4c 89 6d c0                 	movq	%r13, -0x40(%rbp)
1409526f6: 4c 89 65 c8                 	movq	%r12, -0x38(%rbp)
1409526fa: 48 89 7d d0                 	movq	%rdi, -0x30(%rbp)
1409526fe: f3 0f 11 45 d8              	movss	%xmm0, -0x28(%rbp)
140952703: 4c 8d 45 a8                 	leaq	-0x58(%rbp), %r8
140952707: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14095270b: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
14095270f: e8 7c 79 15 00              	callq	0x140aaa090 <.text+0xaa9090>
140952714: 90                          	nop
140952715: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14095271c: 5b                          	popq	%rbx
14095271d: 5f                          	popq	%rdi
14095271e: 5e                          	popq	%rsi
14095271f: 41 5c                       	popq	%r12
140952721: 41 5d                       	popq	%r13
140952723: 41 5e                       	popq	%r14
140952725: 41 5f                       	popq	%r15
140952727: 5d                          	popq	%rbp
140952728: c3                          	retq
