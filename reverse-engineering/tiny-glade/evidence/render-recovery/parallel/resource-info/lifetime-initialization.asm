D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14048da00: 55                          	pushq	%rbp
14048da01: 41 57                       	pushq	%r15
14048da03: 41 56                       	pushq	%r14
14048da05: 41 55                       	pushq	%r13
14048da07: 41 54                       	pushq	%r12
14048da09: 56                          	pushq	%rsi
14048da0a: 57                          	pushq	%rdi
14048da0b: 53                          	pushq	%rbx
14048da0c: 48 83 ec 48                 	subq	$0x48, %rsp
14048da10: 48 8d 6c 24 40              	leaq	0x40(%rsp), %rbp
14048da15: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14048da1d: 48 8b 31                    	movq	(%rcx), %rsi
14048da20: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14048da24: 4c 8b 02                    	movq	(%rdx), %r8
14048da27: 4c 89 45 e0                 	movq	%r8, -0x20(%rbp)
14048da2b: 4c 8b 42 08                 	movq	0x8(%rdx), %r8
14048da2f: 4c 89 45 f0                 	movq	%r8, -0x10(%rbp)
14048da33: 48 39 c6                    	cmpq	%rax, %rsi
14048da36: 0f 84 b2 00 00 00           	je	0x14048daee <.text+0x48caee>
14048da3c: 48 8b 79 10                 	movq	0x10(%rcx), %rdi
14048da40: 48 8b 5a 10                 	movq	0x10(%rdx), %rbx
14048da44: 48 29 f0                    	subq	%rsi, %rax
14048da47: 48 c1 e8 03                 	shrq	$0x3, %rax
14048da4b: 49 be 39 8e e3 38 8e e3 38 8e       	movabsq	$-0x71c71c71c71c71c7, %r14 # imm = 0x8E38E38E38E38E39
14048da55: 4c 0f af f0                 	imulq	%rax, %r14
14048da59: 45 31 ff                    	xorl	%r15d, %r15d
14048da5c: 4c 8d 65 fc                 	leaq	-0x4(%rbp), %r12
14048da60: 4c 8d 6d ec                 	leaq	-0x14(%rbp), %r13
14048da64: eb 5d                       	jmp	0x14048dac3 <.text+0x48cac3>
14048da66: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14048da70: b9 00 00 00 00              	movl	$0x0, %ecx
14048da75: 4d 89 e0                    	movq	%r12, %r8
14048da78: ba 00 00 00 00              	movl	$0x0, %edx
14048da7d: 4d 85 c9                    	testq	%r9, %r9
14048da80: 0f 85 84 00 00 00           	jne	0x14048db0a <.text+0x48cb0a>
14048da86: 41 89 10                    	movl	%edx, (%r8)
14048da89: 8b 55 fc                    	movl	-0x4(%rbp), %edx
14048da8c: 44 8b 45 ec                 	movl	-0x14(%rbp), %r8d
14048da90: 4c 8b 55 f0                 	movq	-0x10(%rbp), %r10
14048da94: 4f 8d 0c 52                 	leaq	(%r10,%r10,2), %r9
14048da98: 42 89 0c cb                 	movl	%ecx, (%rbx,%r9,8)
14048da9c: 42 89 44 cb 04              	movl	%eax, 0x4(%rbx,%r9,8)
14048daa1: 42 89 54 cb 08              	movl	%edx, 0x8(%rbx,%r9,8)
14048daa6: 46 89 44 cb 0c              	movl	%r8d, 0xc(%rbx,%r9,8)
14048daab: 4a c7 44 cb 10 ff ff ff ff  	movq	$-0x1, 0x10(%rbx,%r9,8)
14048dab4: 49 ff c2                    	incq	%r10
14048dab7: 4c 89 55 f0                 	movq	%r10, -0x10(%rbp)
14048dabb: 49 ff c7                    	incq	%r15
14048dabe: 4d 39 f7                    	cmpq	%r14, %r15
14048dac1: 74 2b                       	je	0x14048daee <.text+0x48caee>
14048dac3: 4b 8d 04 ff                 	leaq	(%r15,%r15,8), %rax
14048dac7: 4c 8b 0c c6                 	movq	(%rsi,%rax,8), %r9
14048dacb: 49 8d 41 ff                 	leaq	-0x1(%r9), %rax
14048dacf: 48 83 f8 02                 	cmpq	$0x2, %rax
14048dad3: 73 9b                       	jae	0x14048da70 <.text+0x48ca70>
14048dad5: 31 c9                       	xorl	%ecx, %ecx
14048dad7: e8 64 70 74 00              	callq	0x140bd4b40 <.text+0xbd3b40>
14048dadc: 8b 0f                       	movl	(%rdi), %ecx
14048dade: 8b 57 04                    	movl	0x4(%rdi), %edx
14048dae1: 89 4d fc                    	movl	%ecx, -0x4(%rbp)
14048dae4: b9 01 00 00 00              	movl	$0x1, %ecx
14048dae9: 4d 89 e8                    	movq	%r13, %r8
14048daec: eb 98                       	jmp	0x14048da86 <.text+0x48ca86>
14048daee: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
14048daf2: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14048daf6: 48 89 08                    	movq	%rcx, (%rax)
14048daf9: 48 83 c4 48                 	addq	$0x48, %rsp
14048dafd: 5b                          	popq	%rbx
14048dafe: 5f                          	popq	%rdi
14048daff: 5e                          	popq	%rsi
14048db00: 41 5c                       	popq	%r12
14048db02: 41 5d                       	popq	%r13
14048db04: 41 5e                       	popq	%r14
14048db06: 41 5f                       	popq	%r15
14048db08: 5d                          	popq	%rbp
14048db09: c3                          	retq
14048db0a: 48 8d 0d bc 07 55 02        	leaq	0x25507bc(%rip), %rcx   # 0x1429de2cd
14048db11: 4c 8d 05 e0 07 55 02        	leaq	0x25507e0(%rip), %r8    # 0x1429de2f8
14048db18: ba 28 00 00 00              	movl	$0x28, %edx
14048db1d: e8 0e b9 44 02              	callq	0x1428d9430 <.text+0x28d8430>
14048db22: 0f 0b                       	ud2
