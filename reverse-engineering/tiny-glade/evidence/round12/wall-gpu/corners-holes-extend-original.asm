
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14123fe30: 55                          	pushq	%rbp
14123fe31: 41 57                       	pushq	%r15
14123fe33: 41 56                       	pushq	%r14
14123fe35: 56                          	pushq	%rsi
14123fe36: 57                          	pushq	%rdi
14123fe37: 53                          	pushq	%rbx
14123fe38: 48 83 ec 68                 	subq	$0x68, %rsp
14123fe3c: 48 8d 6c 24 60              	leaq	0x60(%rsp), %rbp
14123fe41: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14123fe49: 48 89 ce                    	movq	%rcx, %rsi
14123fe4c: 48 8b 02                    	movq	(%rdx), %rax
14123fe4f: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14123fe53: 48 8b 42 08                 	movq	0x8(%rdx), %rax
14123fe57: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14123fe5b: 48 8b 7a 10                 	movq	0x10(%rdx), %rdi
14123fe5f: 48 8b 01                    	movq	(%rcx), %rax
14123fe62: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
14123fe66: 48 29 d0                    	subq	%rdx, %rax
14123fe69: 48 39 f8                    	cmpq	%rdi, %rax
14123fe6c: 0f 82 87 00 00 00           	jb	0x14123fef9 <.text+0x123eef9>
14123fe72: 48 85 ff                    	testq	%rdi, %rdi
14123fe75: 4c 8b 7d f0                 	movq	-0x10(%rbp), %r15
14123fe79: 74 44                       	je	0x14123febf <.text+0x123eebf>
14123fe7b: 48 c1 e7 02                 	shlq	$0x2, %rdi
14123fe7f: 48 8d 1c bf                 	leaq	(%rdi,%rdi,4), %rbx
14123fe83: 45 31 f6                    	xorl	%r14d, %r14d
14123fe86: 48 8d 7d d0                 	leaq	-0x30(%rbp), %rdi
14123fe8a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14123fe90: 47 0f b6 44 37 10           	movzbl	0x10(%r15,%r14), %r8d
14123fe96: 41 80 f8 05                 	cmpb	$0x5, %r8b
14123fe9a: 74 23                       	je	0x14123febf <.text+0x123eebf>
14123fe9c: 4b 8d 04 37                 	leaq	(%r15,%r14), %rax
14123fea0: 0f 10 00                    	movups	(%rax), %xmm0
14123fea3: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
14123fea7: 48 89 f1                    	movq	%rsi, %rcx
14123feaa: 48 89 fa                    	movq	%rdi, %rdx
14123fead: e8 ae 79 d4 ff              	callq	0x140f87860 <.text+0xf86860>
14123feb2: 49 83 c6 14                 	addq	$0x14, %r14
14123feb6: 4c 39 f3                    	cmpq	%r14, %rbx
14123feb9: 4c 8b 7d f0                 	movq	-0x10(%rbp), %r15
14123febd: 75 d1                       	jne	0x14123fe90 <.text+0x123ee90>
14123febf: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
14123fec3: 48 85 c0                    	testq	%rax, %rax
14123fec6: 74 23                       	je	0x14123feeb <.text+0x123eeeb>
14123fec8: 48 c1 e0 02                 	shlq	$0x2, %rax
14123fecc: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
14123fed0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14123fed6: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14123feda: 48 83 c4 68                 	addq	$0x68, %rsp
14123fede: 5b                          	popq	%rbx
14123fedf: 5f                          	popq	%rdi
14123fee0: 5e                          	popq	%rsi
14123fee1: 41 5e                       	popq	%r14
14123fee3: 41 5f                       	popq	%r15
14123fee5: 5d                          	popq	%rbp
14123fee6: e9 35 3d 3d ff              	jmp	0x140613c20 <.text+0x612c20>
14123feeb: 90                          	nop
14123feec: 48 83 c4 68                 	addq	$0x68, %rsp
14123fef0: 5b                          	popq	%rbx
14123fef1: 5f                          	popq	%rdi
14123fef2: 5e                          	popq	%rsi
14123fef3: 41 5e                       	popq	%r14
14123fef5: 41 5f                       	popq	%r15
14123fef7: 5d                          	popq	%rbp
14123fef8: c3                          	retq
14123fef9: c6 45 ff 00                 	movb	$0x0, -0x1(%rbp)
14123fefd: 48 c7 44 24 20 14 00 00 00  	movq	$0x14, 0x20(%rsp)
14123ff06: 41 b9 04 00 00 00           	movl	$0x4, %r9d
14123ff0c: 48 89 f1                    	movq	%rsi, %rcx
14123ff0f: 49 89 f8                    	movq	%rdi, %r8
14123ff12: 4c 8b 7d f0                 	movq	-0x10(%rbp), %r15
14123ff16: e8 c5 7e 66 01              	callq	0x1428a7de0 <.text+0x28a6de0>
14123ff1b: e9 5b ff ff ff              	jmp	0x14123fe7b <.text+0x123ee7b>
14123ff20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14123ff25: 55                          	pushq	%rbp
14123ff26: 41 57                       	pushq	%r15
14123ff28: 41 56                       	pushq	%r14
14123ff2a: 56                          	pushq	%rsi
14123ff2b: 57                          	pushq	%rdi
14123ff2c: 53                          	pushq	%rbx
14123ff2d: 48 83 ec 28                 	subq	$0x28, %rsp
14123ff31: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
14123ff35: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
14123ff3a: 74 21                       	je	0x14123ff5d <.text+0x123ef5d>
14123ff3c: f6 45 ff 01                 	testb	$0x1, -0x1(%rbp)
14123ff40: 75 1b                       	jne	0x14123ff5d <.text+0x123ef5d>
14123ff42: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
14123ff46: 48 c1 e0 02                 	shlq	$0x2, %rax
14123ff4a: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
14123ff4e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14123ff54: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14123ff58: e8 c3 3c 3d ff              	callq	0x140613c20 <.text+0x612c20>
14123ff5d: 90                          	nop
14123ff5e: 48 83 c4 28                 	addq	$0x28, %rsp
14123ff62: 5b                          	popq	%rbx
14123ff63: 5f                          	popq	%rdi
14123ff64: 5e                          	popq	%rsi
14123ff65: 41 5e                       	popq	%r14
14123ff67: 41 5f                       	popq	%r15
14123ff69: 5d                          	popq	%rbp
14123ff6a: c3                          	retq
14123ff6b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14123ff70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14123ff75: 55                          	pushq	%rbp
14123ff76: 41 57                       	pushq	%r15
14123ff78: 41 56                       	pushq	%r14
14123ff7a: 56                          	pushq	%rsi
14123ff7b: 57                          	pushq	%rdi
14123ff7c: 53                          	pushq	%rbx
14123ff7d: 48 83 ec 28                 	subq	$0x28, %rsp
14123ff81: 48 8d 6a 60                 	leaq	0x60(%rdx), %rbp
14123ff85: 48 83 7d e8 00              	cmpq	$0x0, -0x18(%rbp)
14123ff8a: 74 1f                       	je	0x14123ffab <.text+0x123efab>
14123ff8c: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
14123ff90: 48 8d 04 85 00 00 00 00     	leaq	(,%rax,4), %rax
14123ff98: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
14123ff9c: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14123ffa2: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14123ffa6: e8 75 3c 3d ff              	callq	0x140613c20 <.text+0x612c20>
14123ffab: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
14123ffaf: 48 83 c4 28                 	addq	$0x28, %rsp
14123ffb3: 5b                          	popq	%rbx
14123ffb4: 5f                          	popq	%rdi
14123ffb5: 5e                          	popq	%rsi
14123ffb6: 41 5e                       	popq	%r14
14123ffb8: 41 5f                       	popq	%r15
14123ffba: 5d                          	popq	%rbp
14123ffbb: c3                          	retq
