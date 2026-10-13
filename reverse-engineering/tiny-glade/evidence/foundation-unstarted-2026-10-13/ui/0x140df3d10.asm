
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140df3d10: 55                          	pushq	%rbp
140df3d11: 41 57                       	pushq	%r15
140df3d13: 41 56                       	pushq	%r14
140df3d15: 56                          	pushq	%rsi
140df3d16: 57                          	pushq	%rdi
140df3d17: 53                          	pushq	%rbx
140df3d18: 48 83 ec 48                 	subq	$0x48, %rsp
140df3d1c: 48 8d 6c 24 40              	leaq	0x40(%rsp), %rbp
140df3d21: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
140df3d29: 48 89 ce                    	movq	%rcx, %rsi
140df3d2c: c6 45 ff 00                 	movb	$0x0, -0x1(%rbp)
140df3d30: 48 8d 79 18                 	leaq	0x18(%rcx), %rdi
140df3d34: c6 45 fe 01                 	movb	$0x1, -0x2(%rbp)
140df3d38: 4c 8d 45 ff                 	leaq	-0x1(%rbp), %r8
140df3d3c: 48 89 f9                    	movq	%rdi, %rcx
140df3d3f: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
140df3d43: e8 f8 c1 c5 00              	callq	0x141a4ff40 <.text+0x1a4ef40>
140df3d48: 80 7d ff 00                 	cmpb	$0x0, -0x1(%rbp)
140df3d4c: 74 1e                       	je	0x140df3d6c <.text+0xdf2d6c>
140df3d4e: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140df3d52: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140df3d56: 48 85 d2                    	testq	%rdx, %rdx
140df3d59: 74 72                       	je	0x140df3dcd <.text+0xdf2dcd>
140df3d5b: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140df3d5f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140df3d65: e8 b6 fe 81 ff              	callq	0x140613c20 <.text+0x612c20>
140df3d6a: eb 61                       	jmp	0x140df3dcd <.text+0xdf2dcd>
140df3d6c: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140df3d70: 4c 8b 30                    	movq	(%rax), %r14
140df3d73: 48 8b 48 08                 	movq	0x8(%rax), %rcx
140df3d77: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140df3d7b: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140df3d7f: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
140df3d83: 4c 8b 78 18                 	movq	0x18(%rax), %r15
140df3d87: 48 8b 5e 28                 	movq	0x28(%rsi), %rbx
140df3d8b: 48 3b 5e 18                 	cmpq	0x18(%rsi), %rbx
140df3d8f: 75 0f                       	jne	0x140df3da0 <.text+0xdf2da0>
140df3d91: 48 8d 15 c0 50 d8 01        	leaq	0x1d850c0(%rip), %rdx   # 0x142b78e58
140df3d98: 48 89 f9                    	movq	%rdi, %rcx
140df3d9b: e8 90 ee 76 ff              	callq	0x140562c30 <.text+0x561c30>
140df3da0: 48 8b 46 20                 	movq	0x20(%rsi), %rax
140df3da4: 48 89 d9                    	movq	%rbx, %rcx
140df3da7: 48 c1 e1 05                 	shlq	$0x5, %rcx
140df3dab: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
140df3daf: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
140df3db3: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
140df3db8: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140df3dbc: 48 89 54 08 10              	movq	%rdx, 0x10(%rax,%rcx)
140df3dc1: 4c 89 7c 08 18              	movq	%r15, 0x18(%rax,%rcx)
140df3dc6: 48 ff c3                    	incq	%rbx
140df3dc9: 48 89 5e 28                 	movq	%rbx, 0x28(%rsi)
140df3dcd: 48 83 c4 48                 	addq	$0x48, %rsp
140df3dd1: 5b                          	popq	%rbx
140df3dd2: 5f                          	popq	%rdi
140df3dd3: 5e                          	popq	%rsi
140df3dd4: 41 5e                       	popq	%r14
140df3dd6: 41 5f                       	popq	%r15
140df3dd8: 5d                          	popq	%rbp
140df3dd9: c3                          	retq
