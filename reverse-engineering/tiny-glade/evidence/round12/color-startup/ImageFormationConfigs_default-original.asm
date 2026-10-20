
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408ae7f0: 55                          	pushq	%rbp
1408ae7f1: 41 57                       	pushq	%r15
1408ae7f3: 41 56                       	pushq	%r14
1408ae7f5: 41 54                       	pushq	%r12
1408ae7f7: 56                          	pushq	%rsi
1408ae7f8: 57                          	pushq	%rdi
1408ae7f9: 53                          	pushq	%rbx
1408ae7fa: 48 83 ec 40                 	subq	$0x40, %rsp
1408ae7fe: 48 8d 6c 24 40              	leaq	0x40(%rsp), %rbp
1408ae803: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
1408ae80b: 48 89 ce                    	movq	%rcx, %rsi
1408ae80e: 0f b6 05 f4 63 d8 02        	movzbl	0x2d863f4(%rip), %eax   # 0x143634c09
1408ae815: b9 07 00 00 00              	movl	$0x7, %ecx
1408ae81a: ba 01 00 00 00              	movl	$0x1, %edx
1408ae81f: e8 ec 53 d6 ff              	callq	0x140613c10 <.text+0x612c10>
1408ae824: 48 85 c0                    	testq	%rax, %rax
1408ae827: 0f 84 3b 01 00 00           	je	0x1408ae968 <.text+0x8ad968>
1408ae82d: 49 89 c7                    	movq	%rax, %r15
1408ae830: 8b 3d bc 83 23 02           	movl	0x22383bc(%rip), %edi   # 0x142ae6bf2
1408ae836: 89 78 03                    	movl	%edi, 0x3(%rax)
1408ae839: 44 8b 35 af 83 23 02        	movl	0x22383af(%rip), %r14d  # 0x142ae6bef
1408ae840: 44 89 30                    	movl	%r14d, (%rax)
1408ae843: 0f b6 05 bf 63 d8 02        	movzbl	0x2d863bf(%rip), %eax   # 0x143634c09
1408ae84a: b9 07 00 00 00              	movl	$0x7, %ecx
1408ae84f: ba 01 00 00 00              	movl	$0x1, %edx
1408ae854: e8 b7 53 d6 ff              	callq	0x140613c10 <.text+0x612c10>
1408ae859: 48 85 c0                    	testq	%rax, %rax
1408ae85c: 0f 84 1c 01 00 00           	je	0x1408ae97e <.text+0x8ad97e>
1408ae862: 49 89 c4                    	movq	%rax, %r12
1408ae865: 89 78 03                    	movl	%edi, 0x3(%rax)
1408ae868: 44 89 30                    	movl	%r14d, (%rax)
1408ae86b: 0f b6 1d 1e 65 d8 02        	movzbl	0x2d8651e(%rip), %ebx   # 0x143634d90
1408ae872: 0f b6 05 90 63 d8 02        	movzbl	0x2d86390(%rip), %eax   # 0x143634c09
1408ae879: b9 07 00 00 00              	movl	$0x7, %ecx
1408ae87e: ba 01 00 00 00              	movl	$0x1, %edx
1408ae883: e8 88 53 d6 ff              	callq	0x140613c10 <.text+0x612c10>
1408ae888: 48 85 c0                    	testq	%rax, %rax
1408ae88b: 0f 84 09 01 00 00           	je	0x1408ae99a <.text+0x8ad99a>
1408ae891: 89 78 03                    	movl	%edi, 0x3(%rax)
1408ae894: 44 89 30                    	movl	%r14d, (%rax)
1408ae897: 48 c7 46 30 00 00 00 00     	movq	$0x0, 0x30(%rsi)
1408ae89f: 48 c7 46 35 00 00 00 00     	movq	$0x0, 0x35(%rsi)
1408ae8a7: 48 c7 46 70 00 00 00 00     	movq	$0x0, 0x70(%rsi)
1408ae8af: 48 c7 46 75 00 00 00 00     	movq	$0x0, 0x75(%rsi)
1408ae8b7: 48 c7 06 07 00 00 00        	movq	$0x7, (%rsi)
1408ae8be: 4c 89 7e 08                 	movq	%r15, 0x8(%rsi)
1408ae8c2: 48 c7 46 10 07 00 00 00     	movq	$0x7, 0x10(%rsi)
1408ae8ca: 0f 28 05 5f d5 1e 02        	movaps	0x21ed55f(%rip), %xmm0  # 0x142a9be30
1408ae8d1: 0f 11 46 18                 	movups	%xmm0, 0x18(%rsi)
1408ae8d5: c6 46 28 00                 	movb	$0x0, 0x28(%rsi)
1408ae8d9: c6 46 3d 01                 	movb	$0x1, 0x3d(%rsi)
1408ae8dd: 48 c7 46 40 07 00 00 00     	movq	$0x7, 0x40(%rsi)
1408ae8e5: 4c 89 66 48                 	movq	%r12, 0x48(%rsi)
1408ae8e9: 48 c7 46 50 07 00 00 00     	movq	$0x7, 0x50(%rsi)
1408ae8f1: 0f 11 46 58                 	movups	%xmm0, 0x58(%rsi)
1408ae8f5: c6 46 68 00                 	movb	$0x0, 0x68(%rsi)
1408ae8f9: c6 46 7d 01                 	movb	$0x1, 0x7d(%rsi)
1408ae8fd: 48 c7 86 80 00 00 00 07 00 00 00    	movq	$0x7, 0x80(%rsi)
1408ae908: 48 89 86 88 00 00 00        	movq	%rax, 0x88(%rsi)
1408ae90f: 48 c7 86 90 00 00 00 07 00 00 00    	movq	$0x7, 0x90(%rsi)
1408ae91a: 0f 11 86 98 00 00 00        	movups	%xmm0, 0x98(%rsi)
1408ae921: c6 86 a8 00 00 00 00        	movb	$0x0, 0xa8(%rsi)
1408ae928: f2 0f 10 05 10 d5 1e 02     	movsd	0x21ed510(%rip), %xmm0  # 0x142a9be40
1408ae930: f2 0f 11 86 b0 00 00 00     	movsd	%xmm0, 0xb0(%rsi)
1408ae938: c7 86 b8 00 00 00 00 00 00 00       	movl	$0x0, 0xb8(%rsi)
1408ae942: c6 86 bc 00 00 00 00        	movb	$0x0, 0xbc(%rsi)
1408ae949: 88 9e bd 00 00 00           	movb	%bl, 0xbd(%rsi)
1408ae94f: c6 86 c0 00 00 00 01        	movb	$0x1, 0xc0(%rsi)
1408ae956: 48 89 f0                    	movq	%rsi, %rax
1408ae959: 48 83 c4 40                 	addq	$0x40, %rsp
1408ae95d: 5b                          	popq	%rbx
1408ae95e: 5f                          	popq	%rdi
1408ae95f: 5e                          	popq	%rsi
1408ae960: 41 5c                       	popq	%r12
1408ae962: 41 5e                       	popq	%r14
1408ae964: 41 5f                       	popq	%r15
1408ae966: 5d                          	popq	%rbp
1408ae967: c3                          	retq
1408ae968: 4c 8d 05 71 7e 23 02        	leaq	0x2237e71(%rip), %r8    # 0x142ae67e0
1408ae96f: b9 01 00 00 00              	movl	$0x1, %ecx
1408ae974: ba 07 00 00 00              	movl	$0x7, %edx
1408ae979: e8 65 a6 02 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408ae97e: 4c 89 7d f0                 	movq	%r15, -0x10(%rbp)
1408ae982: 4c 8d 05 57 7e 23 02        	leaq	0x2237e57(%rip), %r8    # 0x142ae67e0
1408ae989: b9 01 00 00 00              	movl	$0x1, %ecx
1408ae98e: ba 07 00 00 00              	movl	$0x7, %edx
1408ae993: e8 4b a6 02 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408ae998: eb 1e                       	jmp	0x1408ae9b8 <.text+0x8ad9b8>
1408ae99a: 4c 89 65 e8                 	movq	%r12, -0x18(%rbp)
1408ae99e: 4c 89 7d f0                 	movq	%r15, -0x10(%rbp)
1408ae9a2: 4c 8d 05 37 7e 23 02        	leaq	0x2237e37(%rip), %r8    # 0x142ae67e0
1408ae9a9: b9 01 00 00 00              	movl	$0x1, %ecx
1408ae9ae: ba 07 00 00 00              	movl	$0x7, %edx
1408ae9b3: e8 2b a6 02 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408ae9b8: 0f 0b                       	ud2
1408ae9ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408ae9c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408ae9c5: 55                          	pushq	%rbp
1408ae9c6: 41 57                       	pushq	%r15
1408ae9c8: 41 56                       	pushq	%r14
1408ae9ca: 41 54                       	pushq	%r12
1408ae9cc: 56                          	pushq	%rsi
1408ae9cd: 57                          	pushq	%rdi
1408ae9ce: 53                          	pushq	%rbx
1408ae9cf: 48 83 ec 20                 	subq	$0x20, %rsp
1408ae9d3: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
1408ae9d7: ba 07 00 00 00              	movl	$0x7, %edx
1408ae9dc: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408ae9e2: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1408ae9e6: e8 35 52 d6 ff              	callq	0x140613c20 <.text+0x612c20>
1408ae9eb: 90                          	nop
1408ae9ec: 48 83 c4 20                 	addq	$0x20, %rsp
1408ae9f0: 5b                          	popq	%rbx
1408ae9f1: 5f                          	popq	%rdi
1408ae9f2: 5e                          	popq	%rsi
1408ae9f3: 41 5c                       	popq	%r12
1408ae9f5: 41 5e                       	popq	%r14
1408ae9f7: 41 5f                       	popq	%r15
1408ae9f9: 5d                          	popq	%rbp
1408ae9fa: c3                          	retq
1408ae9fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1408aea00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408aea05: 55                          	pushq	%rbp
1408aea06: 41 57                       	pushq	%r15
1408aea08: 41 56                       	pushq	%r14
1408aea0a: 41 54                       	pushq	%r12
1408aea0c: 56                          	pushq	%rsi
1408aea0d: 57                          	pushq	%rdi
1408aea0e: 53                          	pushq	%rbx
1408aea0f: 48 83 ec 20                 	subq	$0x20, %rsp
1408aea13: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
1408aea17: ba 07 00 00 00              	movl	$0x7, %edx
1408aea1c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408aea22: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1408aea26: e8 f5 51 d6 ff              	callq	0x140613c20 <.text+0x612c20>
1408aea2b: ba 07 00 00 00              	movl	$0x7, %edx
1408aea30: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1408aea36: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1408aea3a: e8 e1 51 d6 ff              	callq	0x140613c20 <.text+0x612c20>
1408aea3f: 90                          	nop
1408aea40: 48 83 c4 20                 	addq	$0x20, %rsp
1408aea44: 5b                          	popq	%rbx
1408aea45: 5f                          	popq	%rdi
1408aea46: 5e                          	popq	%rsi
1408aea47: 41 5c                       	popq	%r12
1408aea49: 41 5e                       	popq	%r14
1408aea4b: 41 5f                       	popq	%r15
1408aea4d: 5d                          	popq	%rbp
1408aea4e: c3                          	retq
