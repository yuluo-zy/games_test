
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b3f550: 55                          	pushq	%rbp
140b3f551: 56                          	pushq	%rsi
140b3f552: 57                          	pushq	%rdi
140b3f553: 53                          	pushq	%rbx
140b3f554: 48 81 ec 08 02 00 00        	subq	$0x208, %rsp            # imm = 0x208
140b3f55b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b3f563: 48 c7 85 80 01 00 00 fe ff ff ff    	movq	$-0x2, 0x180(%rbp)
140b3f56e: 48 89 ce                    	movq	%rcx, %rsi
140b3f571: 0f b6 05 91 56 af 02        	movzbl	0x2af5691(%rip), %eax   # 0x143634c09
140b3f578: b9 e0 00 00 00              	movl	$0xe0, %ecx
140b3f57d: ba 08 00 00 00              	movl	$0x8, %edx
140b3f582: e8 89 46 ad ff              	callq	0x140613c10 <.text+0x612c10>
140b3f587: 48 85 c0                    	testq	%rax, %rax
140b3f58a: 0f 84 83 02 00 00           	je	0x140b3f813 <.text+0xb3e813>
140b3f590: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
140b3f597: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b3f59a: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
140b3f59e: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
140b3f5a2: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
140b3f5a9: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
140b3f5b0: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
140b3f5b7: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
140b3f5c1: 0f 11 85 f0 00 00 00        	movups	%xmm0, 0xf0(%rbp)
140b3f5c8: 0f 11 85 08 01 00 00        	movups	%xmm0, 0x108(%rbp)
140b3f5cf: 0f 11 85 20 01 00 00        	movups	%xmm0, 0x120(%rbp)
140b3f5d6: 0f 11 85 38 01 00 00        	movups	%xmm0, 0x138(%rbp)
140b3f5dd: 0f 11 85 50 01 00 00        	movups	%xmm0, 0x150(%rbp)
140b3f5e4: c7 85 60 01 00 00 00 00 00 00       	movl	$0x0, 0x160(%rbp)
140b3f5ee: 48 c7 45 18 00 00 00 00     	movq	$0x0, 0x18(%rbp)
140b3f5f6: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
140b3f5fe: 48 c7 45 40 08 00 00 00     	movq	$0x8, 0x40(%rbp)
140b3f606: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
140b3f60e: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
140b3f616: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
140b3f61e: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
140b3f629: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
140b3f634: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
140b3f63f: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140b3f649: 48 89 9d d0 00 00 00        	movq	%rbx, 0xd0(%rbp)
140b3f650: 48 8d 05 a6 f6 fa 01        	leaq	0x1faf6a6(%rip), %rax   # 0x142aeecfd
140b3f657: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140b3f65e: 48 c7 85 e0 00 00 00 64 00 00 00    	movq	$0x64, 0xe0(%rbp)
140b3f669: 48 c7 85 e8 00 00 00 10 00 00 00    	movq	$0x10, 0xe8(%rbp)
140b3f674: 48 c7 85 00 01 00 00 10 00 00 00    	movq	$0x10, 0x100(%rbp)
140b3f67f: 48 c7 85 18 01 00 00 10 00 00 00    	movq	$0x10, 0x118(%rbp)
140b3f68a: 48 c7 85 30 01 00 00 10 00 00 00    	movq	$0x10, 0x130(%rbp)
140b3f695: 48 c7 85 48 01 00 00 10 00 00 00    	movq	$0x10, 0x148(%rbp)
140b3f6a0: c7 85 68 01 00 00 00 00 00 00       	movl	$0x0, 0x168(%rbp)
140b3f6aa: 66 c7 85 6c 01 00 00 01 00  	movw	$0x1, 0x16c(%rbp)
140b3f6b3: c7 85 70 01 00 00 00 00 00 00       	movl	$0x0, 0x170(%rbp)
140b3f6bd: 0f b6 05 45 55 af 02        	movzbl	0x2af5545(%rip), %eax   # 0x143634c09
140b3f6c4: b9 60 01 00 00              	movl	$0x160, %ecx            # imm = 0x160
140b3f6c9: ba 08 00 00 00              	movl	$0x8, %edx
140b3f6ce: e8 3d 45 ad ff              	callq	0x140613c10 <.text+0x612c10>
140b3f6d3: 48 85 c0                    	testq	%rax, %rax
140b3f6d6: 0f 84 46 01 00 00           	je	0x140b3f822 <.text+0xb3e822>
140b3f6dc: 48 89 c7                    	movq	%rax, %rdi
140b3f6df: 48 8d 55 18                 	leaq	0x18(%rbp), %rdx
140b3f6e3: 41 b8 60 01 00 00           	movl	$0x160, %r8d            # imm = 0x160
140b3f6e9: 48 89 c1                    	movq	%rax, %rcx
140b3f6ec: e8 0f 45 de 01              	callq	0x142923c00 <.text+0x2922c00>
140b3f6f1: 4c 8d 05 08 f3 fa 01        	leaq	0x1faf308(%rip), %r8    # 0x142aeea00
140b3f6f8: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b3f6fc: 48 89 fa                    	movq	%rdi, %rdx
140b3f6ff: e8 0c 54 1a 00              	callq	0x140ce4b10 <.text+0xce3b10>
140b3f704: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140b3f708: e8 23 22 ed ff              	callq	0x140a11930 <.text+0xa10930>
140b3f70d: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
140b3f711: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
140b3f718: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140b3f71c: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
140b3f720: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140b3f724: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140b3f728: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140b3f72c: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140b3f730: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140b3f734: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140b3f738: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
140b3f73c: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140b3f740: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140b3f744: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140b3f748: 0f 11 00                    	movups	%xmm0, (%rax)
140b3f74b: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140b3f74f: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140b3f753: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140b3f757: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140b3f75b: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140b3f75f: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140b3f766: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140b3f76d: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140b3f774: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140b3f778: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140b3f77f: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140b3f783: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140b3f78a: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140b3f78e: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140b3f795: 48 c7 45 20 02 00 00 00     	movq	$0x2, 0x20(%rbp)
140b3f79d: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140b3f7a1: 48 c7 45 30 02 00 00 00     	movq	$0x2, 0x30(%rbp)
140b3f7a9: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
140b3f7b1: 48 c7 45 40 08 00 00 00     	movq	$0x8, 0x40(%rbp)
140b3f7b9: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b3f7bc: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
140b3f7c0: 48 89 5d 18                 	movq	%rbx, 0x18(%rbp)
140b3f7c4: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140b3f7c8: e8 53 02 1f 00              	callq	0x140d2fa20 <.text+0xd2ea20>
140b3f7cd: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140b3f7d1: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140b3f7d5: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140b3f7d9: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140b3f7dd: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140b3f7e1: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140b3f7e5: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140b3f7e9: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140b3f7ed: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140b3f7f1: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140b3f7f5: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140b3f7f9: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140b3f7fd: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140b3f801: 0f 11 06                    	movups	%xmm0, (%rsi)
140b3f804: 48 89 f0                    	movq	%rsi, %rax
140b3f807: 48 81 c4 08 02 00 00        	addq	$0x208, %rsp            # imm = 0x208
140b3f80e: 5b                          	popq	%rbx
140b3f80f: 5f                          	popq	%rdi
140b3f810: 5e                          	popq	%rsi
140b3f811: 5d                          	popq	%rbp
140b3f812: c3                          	retq
140b3f813: b9 08 00 00 00              	movl	$0x8, %ecx
140b3f818: ba e0 00 00 00              	movl	$0xe0, %edx
140b3f81d: e8 de 97 d9 01              	callq	0x1428d9000 <.text+0x28d8000>
140b3f822: b9 08 00 00 00              	movl	$0x8, %ecx
140b3f827: ba 60 01 00 00              	movl	$0x160, %edx            # imm = 0x160
140b3f82c: e8 cf 97 d9 01              	callq	0x1428d9000 <.text+0x28d8000>
140b3f831: 0f 0b                       	ud2
140b3f833: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b3f840: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f845: 55                          	pushq	%rbp
140b3f846: 56                          	pushq	%rsi
140b3f847: 57                          	pushq	%rdi
140b3f848: 53                          	pushq	%rbx
140b3f849: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f84d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f854: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140b3f858: e8 13 66 aa ff              	callq	0x1405e5e70 <.text+0x5e4e70>
140b3f85d: 90                          	nop
140b3f85e: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f862: 5b                          	popq	%rbx
140b3f863: 5f                          	popq	%rdi
140b3f864: 5e                          	popq	%rsi
140b3f865: 5d                          	popq	%rbp
140b3f866: c3                          	retq
140b3f867: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b3f870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f875: 55                          	pushq	%rbp
140b3f876: 56                          	pushq	%rsi
140b3f877: 57                          	pushq	%rdi
140b3f878: 53                          	pushq	%rbx
140b3f879: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f87d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f884: ba e0 00 00 00              	movl	$0xe0, %edx
140b3f889: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140b3f88f: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
140b3f896: e8 85 43 ad ff              	callq	0x140613c20 <.text+0x612c20>
140b3f89b: 90                          	nop
140b3f89c: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f8a0: 5b                          	popq	%rbx
140b3f8a1: 5f                          	popq	%rdi
140b3f8a2: 5e                          	popq	%rsi
140b3f8a3: 5d                          	popq	%rbp
140b3f8a4: c3                          	retq
140b3f8a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140b3f8b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f8b5: 55                          	pushq	%rbp
140b3f8b6: 56                          	pushq	%rsi
140b3f8b7: 57                          	pushq	%rdi
140b3f8b8: 53                          	pushq	%rbx
140b3f8b9: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f8bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f8c4: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b3f8c8: e8 b3 9f 4e ff              	callq	0x140029880 <.text+0x28880>
140b3f8cd: 90                          	nop
140b3f8ce: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f8d2: 5b                          	popq	%rbx
140b3f8d3: 5f                          	popq	%rdi
140b3f8d4: 5e                          	popq	%rsi
140b3f8d5: 5d                          	popq	%rbp
140b3f8d6: c3                          	retq
140b3f8d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b3f8e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b3f8e5: 55                          	pushq	%rbp
140b3f8e6: 56                          	pushq	%rsi
140b3f8e7: 57                          	pushq	%rdi
140b3f8e8: 53                          	pushq	%rbx
140b3f8e9: 48 83 ec 28                 	subq	$0x28, %rsp
140b3f8ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3f8f4: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140b3f8f8: e8 83 9f 4e ff              	callq	0x140029880 <.text+0x28880>
140b3f8fd: 90                          	nop
140b3f8fe: 48 83 c4 28                 	addq	$0x28, %rsp
140b3f902: 5b                          	popq	%rbx
140b3f903: 5f                          	popq	%rdi
140b3f904: 5e                          	popq	%rsi
140b3f905: 5d                          	popq	%rbp
140b3f906: c3                          	retq
