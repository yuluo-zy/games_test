
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407cf210: 55                          	pushq	%rbp
1407cf211: 41 57                       	pushq	%r15
1407cf213: 41 56                       	pushq	%r14
1407cf215: 41 55                       	pushq	%r13
1407cf217: 41 54                       	pushq	%r12
1407cf219: 56                          	pushq	%rsi
1407cf21a: 57                          	pushq	%rdi
1407cf21b: 53                          	pushq	%rbx
1407cf21c: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
1407cf223: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407cf22b: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
1407cf233: 4c 89 4d b8                 	movq	%r9, -0x48(%rbp)
1407cf237: 4c 89 45 b0                 	movq	%r8, -0x50(%rbp)
1407cf23b: 48 89 55 a8                 	movq	%rdx, -0x58(%rbp)
1407cf23f: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
1407cf247: 48 c7 45 e0 08 00 00 00     	movq	$0x8, -0x20(%rbp)
1407cf24f: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
1407cf257: 48 c7 45 c8 03 00 00 00     	movq	$0x3, -0x38(%rbp)
1407cf25f: c6 45 d2 09                 	movb	$0x9, -0x2e(%rbp)
1407cf263: 66 c7 45 d0 00 01           	movw	$0x100, -0x30(%rbp)     # imm = 0x100
1407cf269: 44 8b 69 1c                 	movl	0x1c(%rcx), %r13d
1407cf26d: 4c 8b 31                    	movq	(%rcx), %r14
1407cf270: 48 8b 71 10                 	movq	0x10(%rcx), %rsi
1407cf274: 31 c0                       	xorl	%eax, %eax
1407cf276: 4c 8d 3d 0b 8d 2b 02        	leaq	0x22b8d0b(%rip), %r15   # 0x142a87f88
1407cf27d: 4c 8d 65 d8                 	leaq	-0x28(%rbp), %r12
1407cf281: eb 23                       	jmp	0x1407cf2a6 <.text+0x7ce2a6>
1407cf283: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407cf290: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
1407cf294: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1407cf298: 48 89 0c d8                 	movq	%rcx, (%rax,%rbx,8)
1407cf29c: 48 ff c3                    	incq	%rbx
1407cf29f: 48 89 5d e8                 	movq	%rbx, -0x18(%rbp)
1407cf2a3: 48 89 f8                    	movq	%rdi, %rax
1407cf2a6: 48 83 f8 03                 	cmpq	$0x3, %rax
1407cf2aa: 74 3b                       	je	0x1407cf2e7 <.text+0x7ce2e7>
1407cf2ac: 48 8d 78 01                 	leaq	0x1(%rax), %rdi
1407cf2b0: 48 89 7d c0                 	movq	%rdi, -0x40(%rbp)
1407cf2b4: 0f b6 54 05 d0              	movzbl	-0x30(%rbp,%rax), %edx
1407cf2b9: 44 89 2e                    	movl	%r13d, (%rsi)
1407cf2bc: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf2c0: 4c 89 f1                    	movq	%r14, %rcx
1407cf2c3: e8 58 e9 0f 00              	callq	0x1408cdc20 <.text+0x8ccc20>
1407cf2c8: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1407cf2cc: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
1407cf2d0: 48 8b 5d e8                 	movq	-0x18(%rbp), %rbx
1407cf2d4: 48 3b 5d d8                 	cmpq	-0x28(%rbp), %rbx
1407cf2d8: 75 b6                       	jne	0x1407cf290 <.text+0x7ce290>
1407cf2da: 4c 89 e1                    	movq	%r12, %rcx
1407cf2dd: 4c 89 fa                    	movq	%r15, %rdx
1407cf2e0: e8 8b 35 d9 ff              	callq	0x140562870 <.text+0x561870>
1407cf2e5: eb a9                       	jmp	0x1407cf290 <.text+0x7ce290>
1407cf2e7: 48 8b 55 a8                 	movq	-0x58(%rbp), %rdx
1407cf2eb: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1407cf2ee: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407cf2f2: 89 01                       	movl	%eax, (%rcx)
1407cf2f4: 48 8b 0a                    	movq	(%rdx), %rcx
1407cf2f7: 48 83 c1 18                 	addq	$0x18, %rcx
1407cf2fb: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf2ff: e8 5c 35 ac 00              	callq	0x141292860 <.text+0x1291860>
1407cf304: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1407cf308: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
1407cf30c: 48 8b 75 e8                 	movq	-0x18(%rbp), %rsi
1407cf310: 48 3b 75 d8                 	cmpq	-0x28(%rbp), %rsi
1407cf314: 75 10                       	jne	0x1407cf326 <.text+0x7ce326>
1407cf316: 48 8d 15 23 8c 2b 02        	leaq	0x22b8c23(%rip), %rdx   # 0x142a87f40
1407cf31d: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
1407cf321: e8 4a 35 d9 ff              	callq	0x140562870 <.text+0x561870>
1407cf326: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
1407cf32a: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1407cf32e: 48 89 0c f0                 	movq	%rcx, (%rax,%rsi,8)
1407cf332: 48 ff c6                    	incq	%rsi
1407cf335: 48 89 75 e8                 	movq	%rsi, -0x18(%rbp)
1407cf339: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
1407cf33d: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1407cf340: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407cf344: 89 01                       	movl	%eax, (%rcx)
1407cf346: 48 8b 0a                    	movq	(%rdx), %rcx
1407cf349: 48 83 c1 18                 	addq	$0x18, %rcx
1407cf34d: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf351: e8 0a 35 ac 00              	callq	0x141292860 <.text+0x1291860>
1407cf356: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1407cf35a: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
1407cf35e: 48 8b 75 e8                 	movq	-0x18(%rbp), %rsi
1407cf362: 48 3b 75 d8                 	cmpq	-0x28(%rbp), %rsi
1407cf366: 75 10                       	jne	0x1407cf378 <.text+0x7ce378>
1407cf368: 48 8d 15 e9 8b 2b 02        	leaq	0x22b8be9(%rip), %rdx   # 0x142a87f58
1407cf36f: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
1407cf373: e8 f8 34 d9 ff              	callq	0x140562870 <.text+0x561870>
1407cf378: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
1407cf37c: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1407cf380: 48 89 0c f0                 	movq	%rcx, (%rax,%rsi,8)
1407cf384: 48 ff c6                    	incq	%rsi
1407cf387: 48 89 75 e8                 	movq	%rsi, -0x18(%rbp)
1407cf38b: 48 89 75 d0                 	movq	%rsi, -0x30(%rbp)
1407cf38f: 0f 10 45 d8                 	movups	-0x28(%rbp), %xmm0
1407cf393: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1407cf397: c6 45 ff 00                 	movb	$0x0, -0x1(%rbp)
1407cf39b: 4c 8d 05 ce 8b 2b 02        	leaq	0x22b8bce(%rip), %r8    # 0x142a87f70
1407cf3a2: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
1407cf3a6: 48 8b 4d b8                 	movq	-0x48(%rbp), %rcx
1407cf3aa: e8 c1 e7 a9 00              	callq	0x14126db70 <.text+0x126cb70>
1407cf3af: 90                          	nop
1407cf3b0: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
1407cf3b7: 5b                          	popq	%rbx
1407cf3b8: 5f                          	popq	%rdi
1407cf3b9: 5e                          	popq	%rsi
1407cf3ba: 41 5c                       	popq	%r12
1407cf3bc: 41 5d                       	popq	%r13
1407cf3be: 41 5e                       	popq	%r14
1407cf3c0: 41 5f                       	popq	%r15
1407cf3c2: 5d                          	popq	%rbp
1407cf3c3: c3                          	retq
1407cf3c4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407cf3d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407cf3d5: 55                          	pushq	%rbp
1407cf3d6: 41 57                       	pushq	%r15
1407cf3d8: 41 56                       	pushq	%r14
1407cf3da: 41 55                       	pushq	%r13
1407cf3dc: 41 54                       	pushq	%r12
1407cf3de: 56                          	pushq	%rsi
1407cf3df: 57                          	pushq	%rdi
1407cf3e0: 53                          	pushq	%rbx
1407cf3e1: 48 83 ec 28                 	subq	$0x28, %rsp
1407cf3e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407cf3ec: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1407cf3f0: f0                          	lock
1407cf3f1: 48 ff 08                    	decq	(%rax)
1407cf3f4: 75 0d                       	jne	0x1407cf403 <.text+0x7ce403>
1407cf3f6: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf3fa: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1407cf3fe: e8 3d dc 38 00              	callq	0x140b5d040 <.text+0xb5c040>
1407cf403: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf407: 48 83 c4 28                 	addq	$0x28, %rsp
1407cf40b: 5b                          	popq	%rbx
1407cf40c: 5f                          	popq	%rdi
1407cf40d: 5e                          	popq	%rsi
1407cf40e: 41 5c                       	popq	%r12
1407cf410: 41 5d                       	popq	%r13
1407cf412: 41 5e                       	popq	%r14
1407cf414: 41 5f                       	popq	%r15
1407cf416: 5d                          	popq	%rbp
1407cf417: c3                          	retq
1407cf418: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407cf420: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407cf425: 55                          	pushq	%rbp
1407cf426: 41 57                       	pushq	%r15
1407cf428: 41 56                       	pushq	%r14
1407cf42a: 41 55                       	pushq	%r13
1407cf42c: 41 54                       	pushq	%r12
1407cf42e: 56                          	pushq	%rsi
1407cf42f: 57                          	pushq	%rdi
1407cf430: 53                          	pushq	%rbx
1407cf431: 48 83 ec 28                 	subq	$0x28, %rsp
1407cf435: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407cf43c: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1407cf440: f0                          	lock
1407cf441: 48 ff 08                    	decq	(%rax)
1407cf444: 75 0d                       	jne	0x1407cf453 <.text+0x7ce453>
1407cf446: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf44a: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1407cf44e: e8 ed db 38 00              	callq	0x140b5d040 <.text+0xb5c040>
1407cf453: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf457: 48 83 c4 28                 	addq	$0x28, %rsp
1407cf45b: 5b                          	popq	%rbx
1407cf45c: 5f                          	popq	%rdi
1407cf45d: 5e                          	popq	%rsi
1407cf45e: 41 5c                       	popq	%r12
1407cf460: 41 5d                       	popq	%r13
1407cf462: 41 5e                       	popq	%r14
1407cf464: 41 5f                       	popq	%r15
1407cf466: 5d                          	popq	%rbp
1407cf467: c3                          	retq
1407cf468: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407cf470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407cf475: 55                          	pushq	%rbp
1407cf476: 41 57                       	pushq	%r15
1407cf478: 41 56                       	pushq	%r14
1407cf47a: 41 55                       	pushq	%r13
1407cf47c: 41 54                       	pushq	%r12
1407cf47e: 56                          	pushq	%rsi
1407cf47f: 57                          	pushq	%rdi
1407cf480: 53                          	pushq	%rbx
1407cf481: 48 83 ec 28                 	subq	$0x28, %rsp
1407cf485: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407cf48c: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1407cf490: f0                          	lock
1407cf491: 48 ff 08                    	decq	(%rax)
1407cf494: 75 0d                       	jne	0x1407cf4a3 <.text+0x7ce4a3>
1407cf496: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf49a: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1407cf49e: e8 9d db 38 00              	callq	0x140b5d040 <.text+0xb5c040>
1407cf4a3: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
1407cf4a7: 48 83 c4 28                 	addq	$0x28, %rsp
1407cf4ab: 5b                          	popq	%rbx
1407cf4ac: 5f                          	popq	%rdi
1407cf4ad: 5e                          	popq	%rsi
1407cf4ae: 41 5c                       	popq	%r12
1407cf4b0: 41 5d                       	popq	%r13
1407cf4b2: 41 5e                       	popq	%r14
1407cf4b4: 41 5f                       	popq	%r15
1407cf4b6: 5d                          	popq	%rbp
1407cf4b7: c3                          	retq
1407cf4b8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407cf4c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407cf4c5: 55                          	pushq	%rbp
1407cf4c6: 41 57                       	pushq	%r15
1407cf4c8: 41 56                       	pushq	%r14
1407cf4ca: 41 55                       	pushq	%r13
1407cf4cc: 41 54                       	pushq	%r12
1407cf4ce: 56                          	pushq	%rsi
1407cf4cf: 57                          	pushq	%rdi
1407cf4d0: 53                          	pushq	%rbx
1407cf4d1: 48 83 ec 28                 	subq	$0x28, %rsp
1407cf4d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407cf4dc: 80 7d ff 00                 	cmpb	$0x0, -0x1(%rbp)
1407cf4e0: 75 11                       	jne	0x1407cf4f3 <.text+0x7ce4f3>
1407cf4e2: 48 83 c4 28                 	addq	$0x28, %rsp
1407cf4e6: 5b                          	popq	%rbx
1407cf4e7: 5f                          	popq	%rdi
1407cf4e8: 5e                          	popq	%rsi
1407cf4e9: 41 5c                       	popq	%r12
1407cf4eb: 41 5d                       	popq	%r13
1407cf4ed: 41 5e                       	popq	%r14
1407cf4ef: 41 5f                       	popq	%r15
1407cf4f1: 5d                          	popq	%rbp
1407cf4f2: c3                          	retq
1407cf4f3: 48 8d 4d d8                 	leaq	-0x28(%rbp), %rcx
1407cf4f7: e8 b4 5d e0 ff              	callq	0x1405d52b0 <.text+0x5d42b0>
1407cf4fc: eb e4                       	jmp	0x1407cf4e2 <.text+0x7ce4e2>
