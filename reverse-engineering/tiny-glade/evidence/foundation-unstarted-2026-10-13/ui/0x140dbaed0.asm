
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140dbaed0: 55                          	pushq	%rbp
140dbaed1: 56                          	pushq	%rsi
140dbaed2: 57                          	pushq	%rdi
140dbaed3: 48 83 ec 50                 	subq	$0x50, %rsp
140dbaed7: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
140dbaedc: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
140dbaee4: 48 89 ce                    	movq	%rcx, %rsi
140dbaee7: 48 89 55 d0                 	movq	%rdx, -0x30(%rbp)
140dbaeeb: 4c 89 45 d8                 	movq	%r8, -0x28(%rbp)
140dbaeef: 48 89 d1                    	movq	%rdx, %rcx
140dbaef2: 4c 89 c2                    	movq	%r8, %rdx
140dbaef5: e8 96 e6 fa ff              	callq	0x140d69590 <.text+0xd68590>
140dbaefa: 48 85 c0                    	testq	%rax, %rax
140dbaefd: 74 6b                       	je	0x140dbaf6a <.text+0xdb9f6a>
140dbaeff: 89 c1                       	movl	%eax, %ecx
140dbaf01: 83 e1 03                    	andl	$0x3, %ecx
140dbaf04: 83 f9 01                    	cmpl	$0x1, %ecx
140dbaf07: 0f 85 db 00 00 00           	jne	0x140dbafe8 <.text+0xdb9fe8>
140dbaf0d: 48 89 c1                    	movq	%rax, %rcx
140dbaf10: 48 ff c9                    	decq	%rcx
140dbaf13: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
140dbaf17: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
140dbaf1b: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140dbaf1f: 48 8b 40 07                 	movq	0x7(%rax), %rax
140dbaf23: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140dbaf27: 48 8b 00                    	movq	(%rax), %rax
140dbaf2a: 48 85 c0                    	testq	%rax, %rax
140dbaf2d: 74 06                       	je	0x140dbaf35 <.text+0xdb9f35>
140dbaf2f: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140dbaf33: ff d0                       	callq	*%rax
140dbaf35: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140dbaf39: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140dbaf3d: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140dbaf41: 48 85 d2                    	testq	%rdx, %rdx
140dbaf44: 74 09                       	je	0x140dbaf4f <.text+0xdb9f4f>
140dbaf46: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140dbaf4a: e8 d1 8c 85 ff              	callq	0x140613c20 <.text+0x612c20>
140dbaf4f: ba 18 00 00 00              	movl	$0x18, %edx
140dbaf54: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140dbaf5a: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
140dbaf5e: 48 83 c4 50                 	addq	$0x50, %rsp
140dbaf62: 5f                          	popq	%rdi
140dbaf63: 5e                          	popq	%rsi
140dbaf64: 5d                          	popq	%rbp
140dbaf65: e9 b6 8c 85 ff              	jmp	0x140613c20 <.text+0x612c20>
140dbaf6a: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140dbaf74: 48 39 06                    	cmpq	%rax, (%rsi)
140dbaf77: 7e 6f                       	jle	0x140dbafe8 <.text+0xdb9fe8>
140dbaf79: 48 8d 7e 30                 	leaq	0x30(%rsi), %rdi
140dbaf7d: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
140dbaf81: 48 89 f9                    	movq	%rdi, %rcx
140dbaf84: e8 e7 83 c1 00              	callq	0x1419d3370 <.text+0x19d2370>
140dbaf89: 48 b8 05 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffb, %rax # imm = 0x8000000000000005
140dbaf93: 48 39 06                    	cmpq	%rax, (%rsi)
140dbaf96: 7c 50                       	jl	0x140dbafe8 <.text+0xdb9fe8>
140dbaf98: 48 83 7e 10 00              	cmpq	$0x0, 0x10(%rsi)
140dbaf9d: 75 49                       	jne	0x140dbafe8 <.text+0xdb9fe8>
140dbaf9f: 48 83 7e 28 00              	cmpq	$0x0, 0x28(%rsi)
140dbafa4: 75 42                       	jne	0x140dbafe8 <.text+0xdb9fe8>
140dbafa6: 48 83 7e 40 00              	cmpq	$0x0, 0x40(%rsi)
140dbafab: 75 3b                       	jne	0x140dbafe8 <.text+0xdb9fe8>
140dbafad: 48 89 7d f0                 	movq	%rdi, -0x10(%rbp)
140dbafb1: 48 8d 46 18                 	leaq	0x18(%rsi), %rax
140dbafb5: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140dbafb9: 48 89 75 e8                 	movq	%rsi, -0x18(%rbp)
140dbafbd: 48 89 f1                    	movq	%rsi, %rcx
140dbafc0: e8 ab df 83 ff              	callq	0x1405f8f70 <.text+0x5f7f70>
140dbafc5: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
140dbafc9: e8 a2 df 83 ff              	callq	0x1405f8f70 <.text+0x5f7f70>
140dbafce: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140dbafd2: e8 99 df 83 ff              	callq	0x1405f8f70 <.text+0x5f7f70>
140dbafd7: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
140dbafdb: 48 b9 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rcx # imm = 0x8000000000000004
140dbafe5: 48 89 08                    	movq	%rcx, (%rax)
140dbafe8: 48 83 c4 50                 	addq	$0x50, %rsp
140dbafec: 5f                          	popq	%rdi
140dbafed: 5e                          	popq	%rsi
140dbafee: 5d                          	popq	%rbp
140dbafef: c3                          	retq
