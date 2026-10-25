
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141ebc100: 8d 85 d8 00 00 00           	leal	0xd8(%rbp), %eax
141ebc106: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
141ebc10d: 48 8d 05 0c 78 19 fe        	leaq	-0x1e687f4(%rip), %rax  # 0x140053920 <.text+0x52920>
141ebc114: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
141ebc11b: 48 8d 05 f6 e6 0d 01        	leaq	0x10de6f6(%rip), %rax   # 0x142f9a818
141ebc122: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebc129: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
141ebc134: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
141ebc13f: 48 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %rax
141ebc146: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141ebc14d: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
141ebc158: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141ebc15c: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141ebc163: e8 c8 d3 ff ff              	callq	0x141eb9530 <.text+0x1eb8530>
141ebc168: f3 0f 6f 45 e8              	movdqu	-0x18(%rbp), %xmm0
141ebc16d: 66 0f 7f 85 f0 00 00 00     	movdqa	%xmm0, 0xf0(%rbp)
141ebc175: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
141ebc179: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
141ebc180: e9 34 05 00 00              	jmp	0x141ebc6b9 <.text+0x1ebb6b9>
141ebc185: c7 44 39 f0 00 00 00 40     	movl	$0x40000000, -0x10(%rcx,%rdi) # imm = 0x40000000
141ebc18d: 48 89 df                    	movq	%rbx, %rdi
141ebc190: e9 93 ee ff ff              	jmp	0x141ebb028 <.text+0x1eba028>
141ebc195: 48 8d 4f 18                 	leaq	0x18(%rdi), %rcx
141ebc199: e8 22 a7 c8 fe              	callq	0x140b468c0 <.text+0xb458c0>
141ebc19e: 88 c3                       	movb	%al, %bl
141ebc1a0: 4c 8d 05 22 e5 0d 01        	leaq	0x10de522(%rip), %r8    # 0x142f9a6c9
141ebc1a7: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
141ebc1ae: 48 8d 95 70 02 00 00        	leaq	0x270(%rbp), %rdx
141ebc1b5: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
141ebc1bb: e8 80 c1 0f ff              	callq	0x140fb8340 <.text+0xfb7340>
141ebc1c0: 0f b6 c3                    	movzbl	%bl, %eax
141ebc1c3: 4c 8d 0c 80                 	leaq	(%rax,%rax,4), %r9
141ebc1c7: 49 83 c1 1b                 	addq	$0x1b, %r9
141ebc1cb: 48 8d 0d d7 e4 0d 01        	leaq	0x10de4d7(%rip), %rcx   # 0x142f9a6a9
141ebc1d2: 4c 8d 05 b5 e4 0d 01        	leaq	0x10de4b5(%rip), %r8    # 0x142f9a68e
141ebc1d9: 84 c0                       	testb	%al, %al
141ebc1db: 4c 0f 45 c1                 	cmovneq	%rcx, %r8
141ebc1df: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
141ebc1e8: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
141ebc1ef: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141ebc1f6: e8 75 27 0e ff              	callq	0x140f9e970 <.text+0xf9d970>
141ebc1fb: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
141ebc202: e8 59 28 7d fe              	callq	0x14068ea60 <.text+0x68da60>
141ebc207: c7 80 a8 00 00 00 00 00 00 40       	movl	$0x40000000, 0xa8(%rax) # imm = 0x40000000
141ebc211: 48 89 f9                    	movq	%rdi, %rcx
141ebc214: e8 27 02 98 fe              	callq	0x14083c440 <.text+0x83b440>
141ebc219: 84 c0                       	testb	%al, %al
141ebc21b: 74 30                       	je	0x141ebc24d <.text+0x1ebb24d>
141ebc21d: 48 8d 0d c0 e4 0d 01        	leaq	0x10de4c0(%rip), %rcx   # 0x142f9a6e4
141ebc224: ba 18 00 00 00              	movl	$0x18, %edx
141ebc229: e8 52 12 ce fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebc22e: eb 2e                       	jmp	0x141ebc25e <.text+0x1ebb25e>
141ebc230: 48 8d 0d b9 bd b5 00        	leaq	0xb5bdb9(%rip), %rcx    # 0x142a17ff0
141ebc237: e8 d4 d0 a1 00              	callq	0x1428d9310 <.text+0x28d8310>
141ebc23c: e9 c1 06 00 00              	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc241: 48 8d 0d 00 91 cf 00        	leaq	0xcf9100(%rip), %rcx    # 0x142bb5348
141ebc248: e8 03 d0 a1 00              	callq	0x1428d9250 <.text+0x28d8250>
141ebc24d: 48 8d 0d 7f e4 0d 01        	leaq	0x10de47f(%rip), %rcx   # 0x142f9a6d3
141ebc254: ba 11 00 00 00              	movl	$0x11, %edx
141ebc259: e8 22 12 ce fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebc25e: 48 89 54 24 20              	movq	%rdx, 0x20(%rsp)
141ebc263: 48 8d 15 92 e4 0d 01        	leaq	0x10de492(%rip), %rdx   # 0x142f9a6fc
141ebc26a: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
141ebc271: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141ebc277: 49 89 c1                    	movq	%rax, %r9
141ebc27a: e8 a1 1a 10 ff              	callq	0x140fbdd20 <.text+0xfbcd20>
141ebc27f: 48 8d 15 80 e4 0d 01        	leaq	0x10de480(%rip), %rdx   # 0x142f9a706
141ebc286: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
141ebc28d: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141ebc293: 49 89 f9                    	movq	%rdi, %r9
141ebc296: e8 a5 0c 10 ff              	callq	0x140fbcf40 <.text+0xfbbf40>
141ebc29b: e8 00 79 ea fe              	callq	0x140d63ba0 <.text+0xd62ba0>
141ebc2a0: 41 89 d1                    	movl	%edx, %r9d
141ebc2a3: 48 81 c7 e8 00 00 00        	addq	$0xe8, %rdi
141ebc2aa: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
141ebc2b1: 48 89 fa                    	movq	%rdi, %rdx
141ebc2b4: 41 89 c0                    	movl	%eax, %r8d
141ebc2b7: e8 14 79 ea fe              	callq	0x140d63bd0 <.text+0xd62bd0>
141ebc2bc: 80 bd 10 01 00 00 00        	cmpb	$0x0, 0x110(%rbp)
141ebc2c3: 75 09                       	jne	0x141ebc2ce <.text+0x1ebb2ce>
141ebc2c5: 48 8b 9d 18 01 00 00        	movq	0x118(%rbp), %rbx
141ebc2cc: eb 02                       	jmp	0x141ebc2d0 <.text+0x1ebb2d0>
141ebc2ce: 31 db                       	xorl	%ebx, %ebx
141ebc2d0: 48 b9 89 88 88 88 88 88 88 88       	movabsq	$-0x7777777777777777, %rcx # imm = 0x8888888888888889
141ebc2da: 48 89 d8                    	movq	%rbx, %rax
141ebc2dd: 48 f7 e1                    	mulq	%rcx
141ebc2e0: 48 89 d7                    	movq	%rdx, %rdi
141ebc2e3: 48 c1 ef 05                 	shrq	$0x5, %rdi
141ebc2e7: 48 6b c7 3c                 	imulq	$0x3c, %rdi, %rax
141ebc2eb: 48 89 d9                    	movq	%rbx, %rcx
141ebc2ee: 48 29 c1                    	subq	%rax, %rcx
141ebc2f1: 48 89 8d 00 03 00 00        	movq	%rcx, 0x300(%rbp)
141ebc2f8: 48 8d 85 00 03 00 00        	leaq	0x300(%rbp), %rax
141ebc2ff: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
141ebc306: 4c 8d 35 b3 13 ed fe        	leaq	-0x112ec4d(%rip), %r14  # 0x140d8d6c0 <.text+0xd8c6c0>
141ebc30d: 4c 89 b5 d8 02 00 00        	movq	%r14, 0x2d8(%rbp)
141ebc314: 48 8d 05 fd e3 0d 01        	leaq	0x10de3fd(%rip), %rax   # 0x142f9a718
141ebc31b: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebc322: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
141ebc32d: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
141ebc338: 48 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %rax
141ebc33f: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141ebc346: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
141ebc351: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
141ebc355: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141ebc35c: e8 cf d1 ff ff              	callq	0x141eb9530 <.text+0x1eb8530>
141ebc361: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
141ebc365: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
141ebc36c: 48 8b 45 58                 	movq	0x58(%rbp), %rax
141ebc370: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
141ebc377: 48 b9 89 88 88 88 88 88 88 08       	movabsq	$0x888888888888889, %rcx # imm = 0x888888888888889
141ebc381: 48 89 f8                    	movq	%rdi, %rax
141ebc384: 48 f7 e1                    	mulq	%rcx
141ebc387: 48 d1 ea                    	shrq	%rdx
141ebc38a: 4c 6b c2 3c                 	imulq	$0x3c, %rdx, %r8
141ebc38e: 48 89 d8                    	movq	%rbx, %rax
141ebc391: 48 c1 e8 04                 	shrq	$0x4, %rax
141ebc395: 48 b9 81 6f 5e 4d 3c 2b 1a 09       	movabsq	$0x91a2b3c4d5e6f81, %rcx # imm = 0x91A2B3C4D5E6F81
141ebc39f: 48 f7 e1                    	mulq	%rcx
141ebc3a2: 48 89 d1                    	movq	%rdx, %rcx
141ebc3a5: 48 c1 e9 03                 	shrq	$0x3, %rcx
141ebc3a9: 48 ba ab aa aa aa aa aa aa 0a       	movabsq	$0xaaaaaaaaaaaaaab, %rdx # imm = 0xAAAAAAAAAAAAAAB
141ebc3b3: 48 89 c8                    	movq	%rcx, %rax
141ebc3b6: 48 f7 e2                    	mulq	%rdx
141ebc3b9: 48 c1 e2 03                 	shlq	$0x3, %rdx
141ebc3bd: 48 8d 04 52                 	leaq	(%rdx,%rdx,2), %rax
141ebc3c1: 48 29 c1                    	subq	%rax, %rcx
141ebc3c4: 48 ba b7 4a 89 72 06 45 2e c2       	movabsq	$-0x3dd1baf98d76b549, %rdx # imm = 0xC22E450672894AB7
141ebc3ce: 48 89 d8                    	movq	%rbx, %rax
141ebc3d1: 48 f7 e2                    	mulq	%rdx
141ebc3d4: 48 c1 ea 10                 	shrq	$0x10, %rdx
141ebc3d8: 48 89 8d a0 00 00 00        	movq	%rcx, 0xa0(%rbp)
141ebc3df: 48 89 95 98 00 00 00        	movq	%rdx, 0x98(%rbp)
141ebc3e6: 4c 29 c7                    	subq	%r8, %rdi
141ebc3e9: 48 89 7d 70                 	movq	%rdi, 0x70(%rbp)
141ebc3ed: 44 8b a5 cc 02 00 00        	movl	0x2cc(%rbp), %r12d
141ebc3f4: 4c 89 ff                    	movq	%r15, %rdi
141ebc3f7: 0f 84 b6 00 00 00           	je	0x141ebc4b3 <.text+0x1ebb4b3>
141ebc3fd: 48 8d 45 70                 	leaq	0x70(%rbp), %rax
141ebc401: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
141ebc408: 4c 89 b5 d8 02 00 00        	movq	%r14, 0x2d8(%rbp)
141ebc40f: 48 8d 85 00 03 00 00        	leaq	0x300(%rbp), %rax
141ebc416: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
141ebc41d: 48 8d 05 ac f6 17 fe        	leaq	-0x1e80954(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
141ebc424: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
141ebc42b: 48 8d 05 0e e3 0d 01        	leaq	0x10de30e(%rip), %rax   # 0x142f9a740
141ebc432: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebc439: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
141ebc444: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
141ebc44f: 48 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %rax
141ebc456: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141ebc45d: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
141ebc468: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
141ebc46c: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141ebc473: e8 b8 d0 ff ff              	callq	0x141eb9530 <.text+0x1eb8530>
141ebc478: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
141ebc47f: 48 85 d2                    	testq	%rdx, %rdx
141ebc482: 74 12                       	je	0x141ebc496 <.text+0x1ebb496>
141ebc484: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141ebc48b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebc491: e8 8a 77 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebc496: 48 8b 45 40                 	movq	0x40(%rbp), %rax
141ebc49a: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
141ebc4a1: 0f 10 45 30                 	movups	0x30(%rbp), %xmm0
141ebc4a5: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
141ebc4ac: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
141ebc4b3: 48 85 c9                    	testq	%rcx, %rcx
141ebc4b6: 0f 84 b2 00 00 00           	je	0x141ebc56e <.text+0x1ebb56e>
141ebc4bc: 48 8d 85 a0 00 00 00        	leaq	0xa0(%rbp), %rax
141ebc4c3: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
141ebc4ca: 4c 89 b5 d8 02 00 00        	movq	%r14, 0x2d8(%rbp)
141ebc4d1: 48 8d 85 00 03 00 00        	leaq	0x300(%rbp), %rax
141ebc4d8: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
141ebc4df: 48 8d 05 ea f5 17 fe        	leaq	-0x1e80a16(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
141ebc4e6: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
141ebc4ed: 48 8d 05 74 e2 0d 01        	leaq	0x10de274(%rip), %rax   # 0x142f9a768
141ebc4f4: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebc4fb: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
141ebc506: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
141ebc511: 48 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %rax
141ebc518: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141ebc51f: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
141ebc52a: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
141ebc52e: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141ebc535: e8 f6 cf ff ff              	callq	0x141eb9530 <.text+0x1eb8530>
141ebc53a: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
141ebc541: 48 85 d2                    	testq	%rdx, %rdx
141ebc544: 74 12                       	je	0x141ebc558 <.text+0x1ebb558>
141ebc546: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141ebc54d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebc553: e8 c8 76 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebc558: 48 8b 45 28                 	movq	0x28(%rbp), %rax
141ebc55c: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
141ebc563: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
141ebc567: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
141ebc56e: 48 83 bd 98 00 00 00 00     	cmpq	$0x0, 0x98(%rbp)
141ebc576: 0f 84 b1 00 00 00           	je	0x141ebc62d <.text+0x1ebb62d>
141ebc57c: 48 8d 85 98 00 00 00        	leaq	0x98(%rbp), %rax
141ebc583: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
141ebc58a: 4c 89 b5 d8 02 00 00        	movq	%r14, 0x2d8(%rbp)
141ebc591: 48 8d 85 00 03 00 00        	leaq	0x300(%rbp), %rax
141ebc598: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
141ebc59f: 48 8d 05 2a f5 17 fe        	leaq	-0x1e80ad6(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
141ebc5a6: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
141ebc5ad: 48 8d 05 dc e1 0d 01        	leaq	0x10de1dc(%rip), %rax   # 0x142f9a790
141ebc5b4: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebc5bb: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
141ebc5c6: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
141ebc5d1: 48 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %rax
141ebc5d8: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141ebc5df: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
141ebc5ea: 48 89 e9                    	movq	%rbp, %rcx
141ebc5ed: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141ebc5f4: e8 37 cf ff ff              	callq	0x141eb9530 <.text+0x1eb8530>
141ebc5f9: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
141ebc600: 48 85 d2                    	testq	%rdx, %rdx
141ebc603: 74 12                       	je	0x141ebc617 <.text+0x1ebb617>
141ebc605: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141ebc60c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebc612: e8 09 76 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebc617: 48 8b 45 10                 	movq	0x10(%rbp), %rax
141ebc61b: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
141ebc622: 0f 10 45 00                 	movups	(%rbp), %xmm0
141ebc626: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
141ebc62d: 48 8d 15 7c e1 0d 01        	leaq	0x10de17c(%rip), %rdx   # 0x142f9a7b0
141ebc634: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
141ebc63b: 4c 8d 8d 00 03 00 00        	leaq	0x300(%rbp), %r9
141ebc642: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
141ebc648: e8 c3 4d 10 ff              	callq	0x140fc1410 <.text+0xfc0410>
141ebc64d: 48 8b 95 00 03 00 00        	movq	0x300(%rbp), %rdx
141ebc654: 48 85 d2                    	testq	%rdx, %rdx
141ebc657: 74 12                       	je	0x141ebc66b <.text+0x1ebb66b>
141ebc659: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141ebc660: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebc666: e8 b5 75 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebc66b: 45 89 ee                    	movl	%r13d, %r14d
141ebc66e: e9 b5 e9 ff ff              	jmp	0x141ebb028 <.text+0x1eba028>
141ebc673: 48 8d 0d be e1 0d 01        	leaq	0x10de1be(%rip), %rcx   # 0x142f9a838
141ebc67a: ba 14 00 00 00              	movl	$0x14, %edx
141ebc67f: e8 fc 0d ce fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebc684: 49 89 d0                    	movq	%rdx, %r8
141ebc687: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
141ebc68e: 48 89 c2                    	movq	%rax, %rdx
141ebc691: e8 7a cf ff ff              	callq	0x141eb9610 <.text+0x1eb8610>
141ebc696: eb 21                       	jmp	0x141ebc6b9 <.text+0x1ebb6b9>
141ebc698: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
141ebc6a3: 48 c7 85 f8 00 00 00 01 00 00 00    	movq	$0x1, 0xf8(%rbp)
141ebc6ae: 48 c7 85 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbp)
141ebc6b9: 4c 8d 05 8c e1 0d 01        	leaq	0x10de18c(%rip), %r8    # 0x142f9a84c
141ebc6c0: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
141ebc6c7: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
141ebc6ce: 41 b9 06 00 00 00           	movl	$0x6, %r9d
141ebc6d4: e8 67 bc 0f ff              	callq	0x140fb8340 <.text+0xfb7340>
141ebc6d9: 48 8b bd f8 00 00 00        	movq	0xf8(%rbp), %rdi
141ebc6e0: 4c 8b 8d 00 01 00 00        	movq	0x100(%rbp), %r9
141ebc6e7: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
141ebc6ee: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
141ebc6f5: 49 89 f8                    	movq	%rdi, %r8
141ebc6f8: e8 03 f8 7c fe              	callq	0x14068bf00 <.text+0x68af00>
141ebc6fd: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
141ebc704: b2 01                       	movb	$0x1, %dl
141ebc706: e8 25 12 0e ff              	callq	0x140f9d930 <.text+0xf9c930>
141ebc70b: 48 89 c1                    	movq	%rax, %rcx
141ebc70e: b2 02                       	movb	$0x2, %dl
141ebc710: e8 2b 48 cd fe              	callq	0x140b90f40 <.text+0xb8ff40>
141ebc715: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
141ebc71c: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
141ebc723: f3 0f 6f 85 d0 02 00 00     	movdqu	0x2d0(%rbp), %xmm0
141ebc72b: 66 0f 7f 85 00 03 00 00     	movdqa	%xmm0, 0x300(%rbp)
141ebc733: 48 8d 8d 00 03 00 00        	leaq	0x300(%rbp), %rcx
141ebc73a: b2 01                       	movb	$0x1, %dl
141ebc73c: e8 ef 11 0e ff              	callq	0x140f9d930 <.text+0xf9c930>
141ebc741: 66 0f 6e 0d 33 91 a6 00     	movd	0xa69133(%rip), %xmm1   # 0x14292587c
141ebc749: 48 89 c1                    	movq	%rax, %rcx
141ebc74c: e8 ff f9 0e ff              	callq	0x140fac150 <.text+0xfab150>
141ebc751: 48 8b 85 10 03 00 00        	movq	0x310(%rbp), %rax
141ebc758: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
141ebc75f: 66 0f 6f 85 00 03 00 00     	movdqa	0x300(%rbp), %xmm0
141ebc767: 66 0f 7f 85 d0 02 00 00     	movdqa	%xmm0, 0x2d0(%rbp)
141ebc76f: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
141ebc776: b2 01                       	movb	$0x1, %dl
141ebc778: e8 b3 11 0e ff              	callq	0x140f9d930 <.text+0xf9c930>
141ebc77d: 48 89 c1                    	movq	%rax, %rcx
141ebc780: 31 d2                       	xorl	%edx, %edx
141ebc782: e8 f9 44 cd fe              	callq	0x140b90c80 <.text+0xb8fc80>
141ebc787: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
141ebc78e: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
141ebc795: 66 0f 6f 85 d0 02 00 00     	movdqa	0x2d0(%rbp), %xmm0
141ebc79d: 66 0f 7f 85 00 03 00 00     	movdqa	%xmm0, 0x300(%rbp)
141ebc7a5: 48 8d 8d 00 03 00 00        	leaq	0x300(%rbp), %rcx
141ebc7ac: b2 01                       	movb	$0x1, %dl
141ebc7ae: e8 7d 11 0e ff              	callq	0x140f9d930 <.text+0xf9c930>
141ebc7b3: 66 0f 6e 0d 95 45 a7 00     	movd	0xa74595(%rip), %xmm1   # 0x142930d50
141ebc7bb: 48 89 c1                    	movq	%rax, %rcx
141ebc7be: e8 fd 46 cd fe              	callq	0x140b90ec0 <.text+0xb8fec0>
141ebc7c3: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
141ebc7ca: 48 85 d2                    	testq	%rdx, %rdx
141ebc7cd: 44 8b a5 cc 02 00 00        	movl	0x2cc(%rbp), %r12d
141ebc7d4: 45 89 ee                    	movl	%r13d, %r14d
141ebc7d7: 74 0e                       	je	0x141ebc7e7 <.text+0x1ebb7e7>
141ebc7d9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebc7df: 48 89 f9                    	movq	%rdi, %rcx
141ebc7e2: e8 39 74 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebc7e7: 4c 89 ff                    	movq	%r15, %rdi
141ebc7ea: e9 39 e8 ff ff              	jmp	0x141ebb028 <.text+0x1eba028>
141ebc7ef: 45 31 ed                    	xorl	%r13d, %r13d
141ebc7f2: 4c 8d 05 af dc 0d 01        	leaq	0x10ddcaf(%rip), %r8    # 0x142f9a4a8
141ebc7f9: 4c 89 e9                    	movq	%r13, %rcx
141ebc7fc: 48 89 da                    	movq	%rbx, %rdx
141ebc7ff: e8 df c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc804: e9 f9 00 00 00              	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc809: 45 31 ed                    	xorl	%r13d, %r13d
141ebc80c: 4c 8d 05 95 dc 0d 01        	leaq	0x10ddc95(%rip), %r8    # 0x142f9a4a8
141ebc813: 4c 89 e9                    	movq	%r13, %rcx
141ebc816: 48 89 da                    	movq	%rbx, %rdx
141ebc819: e8 c5 c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc81e: e9 df 00 00 00              	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc823: 45 31 ed                    	xorl	%r13d, %r13d
141ebc826: 4c 8d 05 7b dc 0d 01        	leaq	0x10ddc7b(%rip), %r8    # 0x142f9a4a8
141ebc82d: 4c 89 e9                    	movq	%r13, %rcx
141ebc830: 48 89 da                    	movq	%rbx, %rdx
141ebc833: e8 ab c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc838: e9 c5 00 00 00              	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc83d: 45 31 ed                    	xorl	%r13d, %r13d
141ebc840: 4c 8d 05 61 dc 0d 01        	leaq	0x10ddc61(%rip), %r8    # 0x142f9a4a8
141ebc847: 4c 89 e9                    	movq	%r13, %rcx
141ebc84a: 48 89 da                    	movq	%rbx, %rdx
141ebc84d: e8 91 c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc852: e9 ab 00 00 00              	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc857: 45 31 ed                    	xorl	%r13d, %r13d
141ebc85a: 4c 8d 05 47 dc 0d 01        	leaq	0x10ddc47(%rip), %r8    # 0x142f9a4a8
141ebc861: 4c 89 e9                    	movq	%r13, %rcx
141ebc864: 4c 89 f2                    	movq	%r14, %rdx
141ebc867: e8 77 c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc86c: e9 91 00 00 00              	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc871: 45 31 ed                    	xorl	%r13d, %r13d
141ebc874: 4c 89 b5 f8 02 00 00        	movq	%r14, 0x2f8(%rbp)
141ebc87b: 4c 8d 05 26 dc 0d 01        	leaq	0x10ddc26(%rip), %r8    # 0x142f9a4a8
141ebc882: 4c 89 e9                    	movq	%r13, %rcx
141ebc885: 48 89 fa                    	movq	%rdi, %rdx
141ebc888: e8 56 c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc88d: eb 73                       	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc88f: c6 85 24 03 00 00 01        	movb	$0x1, 0x324(%rbp)
141ebc896: 4c 8d 05 0b dc 0d 01        	leaq	0x10ddc0b(%rip), %r8    # 0x142f9a4a8
141ebc89d: b9 01 00 00 00              	movl	$0x1, %ecx
141ebc8a2: ba 10 00 00 00              	movl	$0x10, %edx
141ebc8a7: e8 37 c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc8ac: eb 54                       	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc8ae: 31 ff                       	xorl	%edi, %edi
141ebc8b0: 48 89 cb                    	movq	%rcx, %rbx
141ebc8b3: c6 85 26 03 00 00 01        	movb	$0x1, 0x326(%rbp)
141ebc8ba: 4c 8d 05 cf 87 cf 00        	leaq	0xcf87cf(%rip), %r8     # 0x142bb5090
141ebc8c1: 48 89 f9                    	movq	%rdi, %rcx
141ebc8c4: 48 89 da                    	movq	%rbx, %rdx
141ebc8c7: e8 17 c7 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc8cc: eb 34                       	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc8ce: 4c 89 c3                    	movq	%r8, %rbx
141ebc8d1: 48 c7 85 b0 02 00 00 00 00 00 00    	movq	$0x0, 0x2b0(%rbp)
141ebc8dc: 4c 8d 05 dd 87 cf 00        	leaq	0xcf87dd(%rip), %r8     # 0x142bb50c0
141ebc8e3: 48 8b 8d b0 02 00 00        	movq	0x2b0(%rbp), %rcx
141ebc8ea: 48 89 da                    	movq	%rbx, %rdx
141ebc8ed: e8 f1 c6 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141ebc8f2: eb 0e                       	jmp	0x141ebc902 <.text+0x1ebb902>
141ebc8f4: 4c 8d 05 c5 87 cf 00        	leaq	0xcf87c5(%rip), %r8     # 0x142bb50c0
141ebc8fb: 31 c9                       	xorl	%ecx, %ecx
141ebc8fd: e8 e1 c6 a1 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
