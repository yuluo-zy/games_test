
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412fafb0: 55                          	pushq	%rbp
1412fafb1: 41 57                       	pushq	%r15
1412fafb3: 41 56                       	pushq	%r14
1412fafb5: 41 55                       	pushq	%r13
1412fafb7: 41 54                       	pushq	%r12
1412fafb9: 56                          	pushq	%rsi
1412fafba: 57                          	pushq	%rdi
1412fafbb: 53                          	pushq	%rbx
1412fafbc: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1412fafc3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412fafcb: 48 c7 45 60 fe ff ff ff     	movq	$-0x2, 0x60(%rbp)
1412fafd3: 4d 89 c6                    	movq	%r8, %r14
1412fafd6: 48 89 d3                    	movq	%rdx, %rbx
1412fafd9: 48 89 ce                    	movq	%rcx, %rsi
1412fafdc: 4c 8b 4a 28                 	movq	0x28(%rdx), %r9
1412fafe0: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1412fafe4: 4c 8d 45 5f                 	leaq	0x5f(%rbp), %r8
1412fafe8: e8 c3 e2 17 00              	callq	0x1414792b0 <.text+0x14782b0>
1412fafed: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1412faff1: 48 83 f8 03                 	cmpq	$0x3, %rax
1412faff5: 74 1d                       	je	0x1412fb014 <.text+0x12fa014>
1412faff7: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1412faffb: 48 89 4d 00                 	movq	%rcx, (%rbp)
1412fafff: 0f 10 45 30                 	movups	0x30(%rbp), %xmm0
1412fb003: 0f 10 4d 40                 	movups	0x40(%rbp), %xmm1
1412fb007: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
1412fb00b: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1412fb00f: 83 f8 02                    	cmpl	$0x2, %eax
1412fb012: 75 1c                       	jne	0x1412fb030 <.text+0x12fa030>
1412fb014: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
1412fb01b: 48 c7 46 08 08 00 00 00     	movq	$0x8, 0x8(%rsi)
1412fb023: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
1412fb02b: e9 65 01 00 00              	jmp	0x1412fb195 <.text+0x12fa195>
1412fb030: 48 8b 4d 00                 	movq	(%rbp), %rcx
1412fb034: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
1412fb038: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
1412fb03c: 0f 28 4d f0                 	movaps	-0x10(%rbp), %xmm1
1412fb040: 0f 11 4d 40                 	movups	%xmm1, 0x40(%rbp)
1412fb044: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
1412fb048: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1412fb04c: 0f b6 05 b6 9b 33 02        	movzbl	0x2339bb6(%rip), %eax   # 0x143634c09
1412fb053: b9 c0 00 00 00              	movl	$0xc0, %ecx
1412fb058: ba 08 00 00 00              	movl	$0x8, %edx
1412fb05d: e8 ae 8b 31 ff              	callq	0x140613c10 <.text+0x612c10>
1412fb062: 48 85 c0                    	testq	%rax, %rax
1412fb065: 0f 84 41 01 00 00           	je	0x1412fb1ac <.text+0x12fa1ac>
1412fb06b: 48 89 c7                    	movq	%rax, %rdi
1412fb06e: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
1412fb072: 0f 10 4d 38                 	movups	0x38(%rbp), %xmm1
1412fb076: 0f 10 55 48                 	movups	0x48(%rbp), %xmm2
1412fb07a: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1412fb07e: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1412fb082: 0f 11 00                    	movups	%xmm0, (%rax)
1412fb085: 48 c7 45 10 04 00 00 00     	movq	$0x4, 0x10(%rbp)
1412fb08d: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1412fb091: 48 c7 45 20 01 00 00 00     	movq	$0x1, 0x20(%rbp)
1412fb099: 0f 10 03                    	movups	(%rbx), %xmm0
1412fb09c: 0f 10 4b 10                 	movups	0x10(%rbx), %xmm1
1412fb0a0: 0f 10 53 20                 	movups	0x20(%rbx), %xmm2
1412fb0a4: 0f 29 55 00                 	movaps	%xmm2, (%rbp)
1412fb0a8: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
1412fb0ac: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1412fb0b0: 4c 8d 6d 30                 	leaq	0x30(%rbp), %r13
1412fb0b4: bb 01 00 00 00              	movl	$0x1, %ebx
1412fb0b9: 4c 8d 75 28                 	leaq	0x28(%rbp), %r14
1412fb0bd: 4c 8d 7d e0                 	leaq	-0x20(%rbp), %r15
1412fb0c1: 4c 8d 65 5f                 	leaq	0x5f(%rbp), %r12
1412fb0c5: eb 32                       	jmp	0x1412fb0f9 <.text+0x12fa0f9>
1412fb0c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1412fb0d0: 48 8d 04 5b                 	leaq	(%rbx,%rbx,2), %rax
1412fb0d4: 48 c1 e0 04                 	shlq	$0x4, %rax
1412fb0d8: 0f 10 45 28                 	movups	0x28(%rbp), %xmm0
1412fb0dc: 0f 10 4d 38                 	movups	0x38(%rbp), %xmm1
1412fb0e0: 0f 10 55 48                 	movups	0x48(%rbp), %xmm2
1412fb0e4: 0f 11 54 07 20              	movups	%xmm2, 0x20(%rdi,%rax)
1412fb0e9: 0f 11 4c 07 10              	movups	%xmm1, 0x10(%rdi,%rax)
1412fb0ee: 0f 11 04 07                 	movups	%xmm0, (%rdi,%rax)
1412fb0f2: 48 ff c3                    	incq	%rbx
1412fb0f5: 48 89 5d 20                 	movq	%rbx, 0x20(%rbp)
1412fb0f9: 4c 8b 4d 08                 	movq	0x8(%rbp), %r9
1412fb0fd: 4c 89 f1                    	movq	%r14, %rcx
1412fb100: 4c 89 fa                    	movq	%r15, %rdx
1412fb103: 4d 89 e0                    	movq	%r12, %r8
1412fb106: e8 a5 e1 17 00              	callq	0x1414792b0 <.text+0x14782b0>
1412fb10b: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1412fb10f: 48 83 f8 03                 	cmpq	$0x3, %rax
1412fb113: 74 71                       	je	0x1412fb186 <.text+0x12fa186>
1412fb115: 49 8b 4d 20                 	movq	0x20(%r13), %rcx
1412fb119: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
1412fb11d: 41 0f 10 45 00              	movups	(%r13), %xmm0
1412fb122: 41 0f 10 4d 10              	movups	0x10(%r13), %xmm1
1412fb127: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
1412fb12b: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1412fb12f: 83 f8 02                    	cmpl	$0x2, %eax
1412fb132: 74 52                       	je	0x1412fb186 <.text+0x12fa186>
1412fb134: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1412fb138: 49 89 4d 20                 	movq	%rcx, 0x20(%r13)
1412fb13c: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
1412fb140: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
1412fb144: 41 0f 11 4d 10              	movups	%xmm1, 0x10(%r13)
1412fb149: 41 0f 11 45 00              	movups	%xmm0, (%r13)
1412fb14e: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1412fb152: 48 3b 5d 10                 	cmpq	0x10(%rbp), %rbx
1412fb156: 0f 85 74 ff ff ff           	jne	0x1412fb0d0 <.text+0x12fa0d0>
1412fb15c: 48 c7 44 24 20 30 00 00 00  	movq	$0x30, 0x20(%rsp)
1412fb165: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1412fb16b: 41 b9 08 00 00 00           	movl	$0x8, %r9d
1412fb171: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1412fb175: 48 89 da                    	movq	%rbx, %rdx
1412fb178: e8 e3 e8 5a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1412fb17d: 48 8b 7d 18                 	movq	0x18(%rbp), %rdi
1412fb181: e9 4a ff ff ff              	jmp	0x1412fb0d0 <.text+0x12fa0d0>
1412fb186: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1412fb18a: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1412fb18e: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1412fb192: 0f 11 06                    	movups	%xmm0, (%rsi)
1412fb195: 48 89 f0                    	movq	%rsi, %rax
1412fb198: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1412fb19f: 5b                          	popq	%rbx
1412fb1a0: 5f                          	popq	%rdi
1412fb1a1: 5e                          	popq	%rsi
1412fb1a2: 41 5c                       	popq	%r12
1412fb1a4: 41 5d                       	popq	%r13
1412fb1a6: 41 5e                       	popq	%r14
1412fb1a8: 41 5f                       	popq	%r15
1412fb1aa: 5d                          	popq	%rbp
1412fb1ab: c3                          	retq
1412fb1ac: b9 08 00 00 00              	movl	$0x8, %ecx
1412fb1b1: ba c0 00 00 00              	movl	$0xc0, %edx
1412fb1b6: 4d 89 f0                    	movq	%r14, %r8
1412fb1b9: e8 25 de 5d 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1412fb1be: 0f 0b                       	ud2
1412fb1c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412fb1c5: 55                          	pushq	%rbp
1412fb1c6: 41 57                       	pushq	%r15
1412fb1c8: 41 56                       	pushq	%r14
1412fb1ca: 41 55                       	pushq	%r13
1412fb1cc: 41 54                       	pushq	%r12
1412fb1ce: 56                          	pushq	%rsi
1412fb1cf: 57                          	pushq	%rdi
1412fb1d0: 53                          	pushq	%rbx
1412fb1d1: 48 83 ec 28                 	subq	$0x28, %rsp
1412fb1d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412fb1dc: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1412fb1e0: e8 9b f4 e4 fe              	callq	0x14014a680 <.text+0x149680>
1412fb1e5: 90                          	nop
1412fb1e6: 48 83 c4 28                 	addq	$0x28, %rsp
1412fb1ea: 5b                          	popq	%rbx
1412fb1eb: 5f                          	popq	%rdi
1412fb1ec: 5e                          	popq	%rsi
1412fb1ed: 41 5c                       	popq	%r12
1412fb1ef: 41 5d                       	popq	%r13
1412fb1f1: 41 5e                       	popq	%r14
1412fb1f3: 41 5f                       	popq	%r15
1412fb1f5: 5d                          	popq	%rbp
1412fb1f6: c3                          	retq
1412fb1f7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1412fb200: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412fb205: 55                          	pushq	%rbp
1412fb206: 41 57                       	pushq	%r15
1412fb208: 41 56                       	pushq	%r14
1412fb20a: 41 55                       	pushq	%r13
1412fb20c: 41 54                       	pushq	%r12
1412fb20e: 56                          	pushq	%rsi
1412fb20f: 57                          	pushq	%rdi
1412fb210: 53                          	pushq	%rbx
1412fb211: 48 83 ec 28                 	subq	$0x28, %rsp
1412fb215: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412fb21c: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
1412fb220: e8 5b f4 e4 fe              	callq	0x14014a680 <.text+0x149680>
1412fb225: 90                          	nop
1412fb226: 48 83 c4 28                 	addq	$0x28, %rsp
1412fb22a: 5b                          	popq	%rbx
1412fb22b: 5f                          	popq	%rdi
1412fb22c: 5e                          	popq	%rsi
1412fb22d: 41 5c                       	popq	%r12
1412fb22f: 41 5d                       	popq	%r13
1412fb231: 41 5e                       	popq	%r14
1412fb233: 41 5f                       	popq	%r15
1412fb235: 5d                          	popq	%rbp
1412fb236: c3                          	retq
1412fb237: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1412fb240: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412fb245: 55                          	pushq	%rbp
1412fb246: 41 57                       	pushq	%r15
1412fb248: 41 56                       	pushq	%r14
1412fb24a: 41 55                       	pushq	%r13
1412fb24c: 41 54                       	pushq	%r12
1412fb24e: 56                          	pushq	%rsi
1412fb24f: 57                          	pushq	%rdi
1412fb250: 53                          	pushq	%rbx
1412fb251: 48 83 ec 28                 	subq	$0x28, %rsp
1412fb255: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412fb25c: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1412fb260: e8 db e4 59 ff              	callq	0x140899740 <.text+0x898740>
1412fb265: 90                          	nop
1412fb266: 48 83 c4 28                 	addq	$0x28, %rsp
1412fb26a: 5b                          	popq	%rbx
1412fb26b: 5f                          	popq	%rdi
1412fb26c: 5e                          	popq	%rsi
1412fb26d: 41 5c                       	popq	%r12
1412fb26f: 41 5d                       	popq	%r13
1412fb271: 41 5e                       	popq	%r14
1412fb273: 41 5f                       	popq	%r15
1412fb275: 5d                          	popq	%rbp
1412fb276: c3                          	retq
