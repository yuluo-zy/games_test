
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412df330: 55                          	pushq	%rbp
1412df331: 56                          	pushq	%rsi
1412df332: 57                          	pushq	%rdi
1412df333: 53                          	pushq	%rbx
1412df334: 48 83 ec 48                 	subq	$0x48, %rsp
1412df338: 48 8d 6c 24 40              	leaq	0x40(%rsp), %rbp
1412df33d: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
1412df345: 48 89 d7                    	movq	%rdx, %rdi
1412df348: 48 8b 71 08                 	movq	0x8(%rcx), %rsi
1412df34c: 48 85 f6                    	testq	%rsi, %rsi
1412df34f: 74 41                       	je	0x1412df392 <.text+0x12de392>
1412df351: 4c 8b 07                    	movq	(%rdi), %r8
1412df354: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1412df358: 48 8b 06                    	movq	(%rsi), %rax
1412df35b: 48 8b 5e 10                 	movq	0x10(%rsi), %rbx
1412df35f: 48 29 d8                    	subq	%rbx, %rax
1412df362: 48 83 f8 27                 	cmpq	$0x27, %rax
1412df366: 77 45                       	ja	0x1412df3ad <.text+0x12de3ad>
1412df368: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
1412df36c: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
1412df370: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1412df379: 41 b8 28 00 00 00           	movl	$0x28, %r8d
1412df37f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1412df385: 48 89 f1                    	movq	%rsi, %rcx
1412df388: 48 89 da                    	movq	%rbx, %rdx
1412df38b: e8 d0 a6 5c 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1412df390: eb 76                       	jmp	0x1412df408 <.text+0x12de408>
1412df392: 48 8b 71 10                 	movq	0x10(%rcx), %rsi
1412df396: 4c 8b 07                    	movq	(%rdi), %r8
1412df399: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1412df39d: 48 8b 06                    	movq	(%rsi), %rax
1412df3a0: 48 8b 5e 10                 	movq	0x10(%rsi), %rbx
1412df3a4: 48 29 d8                    	subq	%rbx, %rax
1412df3a7: 48 83 f8 27                 	cmpq	$0x27, %rax
1412df3ab: 76 33                       	jbe	0x1412df3e0 <.text+0x12de3e0>
1412df3ad: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1412df3b1: 48 8d 0d 68 77 ff ff        	leaq	-0x8898(%rip), %rcx     # 0x1412d6b20 <.text+0x12d5b20>
1412df3b8: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
1412df3bc: 4c 89 44 18 08              	movq	%r8, 0x8(%rax,%rbx)
1412df3c1: 48 89 54 18 10              	movq	%rdx, 0x10(%rax,%rbx)
1412df3c6: 0f 10 47 10                 	movups	0x10(%rdi), %xmm0
1412df3ca: 0f 11 44 18 18              	movups	%xmm0, 0x18(%rax,%rbx)
1412df3cf: 48 83 c3 28                 	addq	$0x28, %rbx
1412df3d3: 48 89 5e 10                 	movq	%rbx, 0x10(%rsi)
1412df3d7: 48 83 c4 48                 	addq	$0x48, %rsp
1412df3db: 5b                          	popq	%rbx
1412df3dc: 5f                          	popq	%rdi
1412df3dd: 5e                          	popq	%rsi
1412df3de: 5d                          	popq	%rbp
1412df3df: c3                          	retq
1412df3e0: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
1412df3e4: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
1412df3e8: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1412df3f1: 41 b8 28 00 00 00           	movl	$0x28, %r8d
1412df3f7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1412df3fd: 48 89 f1                    	movq	%rsi, %rcx
1412df400: 48 89 da                    	movq	%rbx, %rdx
1412df403: e8 58 a6 5c 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1412df408: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
1412df40c: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
1412df410: eb 9b                       	jmp	0x1412df3ad <.text+0x12de3ad>
1412df412: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1412df420: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412df425: 55                          	pushq	%rbp
1412df426: 56                          	pushq	%rsi
1412df427: 57                          	pushq	%rdi
1412df428: 53                          	pushq	%rbx
1412df429: 48 83 ec 28                 	subq	$0x28, %rsp
1412df42d: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
1412df431: 48 83 7d f8 00              	cmpq	$0x0, -0x8(%rbp)
1412df436: 74 1e                       	je	0x1412df456 <.text+0x12de456>
1412df438: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1412df43c: 48 8d 04 c9                 	leaq	(%rcx,%rcx,8), %rax
1412df440: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1412df444: 48 01 ca                    	addq	%rcx, %rdx
1412df447: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412df44d: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1412df451: e8 ca 47 33 ff              	callq	0x140613c20 <.text+0x612c20>
1412df456: 90                          	nop
1412df457: 48 83 c4 28                 	addq	$0x28, %rsp
1412df45b: 5b                          	popq	%rbx
1412df45c: 5f                          	popq	%rdi
1412df45d: 5e                          	popq	%rsi
1412df45e: 5d                          	popq	%rbp
1412df45f: c3                          	retq
1412df460: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412df465: 55                          	pushq	%rbp
1412df466: 56                          	pushq	%rsi
1412df467: 57                          	pushq	%rdi
1412df468: 53                          	pushq	%rbx
1412df469: 48 83 ec 28                 	subq	$0x28, %rsp
1412df46d: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
1412df471: 48 83 7d f8 00              	cmpq	$0x0, -0x8(%rbp)
1412df476: 74 1e                       	je	0x1412df496 <.text+0x12de496>
1412df478: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1412df47c: 48 8d 04 c9                 	leaq	(%rcx,%rcx,8), %rax
1412df480: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1412df484: 48 01 ca                    	addq	%rcx, %rdx
1412df487: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1412df48d: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1412df491: e8 8a 47 33 ff              	callq	0x140613c20 <.text+0x612c20>
1412df496: 90                          	nop
1412df497: 48 83 c4 28                 	addq	$0x28, %rsp
1412df49b: 5b                          	popq	%rbx
1412df49c: 5f                          	popq	%rdi
1412df49d: 5e                          	popq	%rsi
1412df49e: 5d                          	popq	%rbp
1412df49f: c3                          	retq
