
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1421be370: 55                          	pushq	%rbp
1421be371: 41 57                       	pushq	%r15
1421be373: 41 56                       	pushq	%r14
1421be375: 41 55                       	pushq	%r13
1421be377: 41 54                       	pushq	%r12
1421be379: 56                          	pushq	%rsi
1421be37a: 57                          	pushq	%rdi
1421be37b: 53                          	pushq	%rbx
1421be37c: 48 81 ec 38 03 00 00        	subq	$0x338, %rsp            # imm = 0x338
1421be383: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1421be38b: 0f 29 b5 a0 02 00 00        	movaps	%xmm6, 0x2a0(%rbp)
1421be392: 48 c7 85 98 02 00 00 fe ff ff ff    	movq	$-0x2, 0x298(%rbp)
1421be39d: 48 89 cf                    	movq	%rcx, %rdi
1421be3a0: 48 8d 0d 3d bf e3 00        	leaq	0xe3bf3d(%rip), %rcx    # 0x142ffa2e4
1421be3a7: ba 14 00 00 00              	movl	$0x14, %edx
1421be3ac: e8 cf f0 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421be3b1: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
1421be3b8: 48 89 95 c0 00 00 00        	movq	%rdx, 0xc0(%rbp)
1421be3bf: 4c 8d 6f 28                 	leaq	0x28(%rdi), %r13
1421be3c3: 48 8d 9d b8 00 00 00        	leaq	0xb8(%rbp), %rbx
1421be3ca: 48 89 9d 00 01 00 00        	movq	%rbx, 0x100(%rbp)
1421be3d1: 4c 8d 25 b8 ba f4 fd        	leaq	-0x20b4548(%rip), %r12  # 0x140109e90 <.text+0x108e90>
1421be3d8: 4c 89 a5 08 01 00 00        	movq	%r12, 0x108(%rbp)
1421be3df: 4c 89 ad 10 01 00 00        	movq	%r13, 0x110(%rbp)
1421be3e6: 48 8d 05 93 1a a4 fe        	leaq	-0x15be56d(%rip), %rax  # 0x140bffe80 <.text+0xbfee80>
1421be3ed: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421be3f4: 48 8d 05 05 bf e3 00        	leaq	0xe3bf05(%rip), %rax    # 0x142ffa300
1421be3fb: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1421be402: 48 c7 85 f8 01 00 00 02 00 00 00    	movq	$0x2, 0x1f8(%rbp)
1421be40d: 48 8d 85 00 01 00 00        	leaq	0x100(%rbp), %rax
1421be414: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1421be41b: 48 c7 85 08 02 00 00 02 00 00 00    	movq	$0x2, 0x208(%rbp)
1421be426: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1421be431: 48 8d b5 70 01 00 00        	leaq	0x170(%rbp), %rsi
1421be438: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421be43f: 48 89 f1                    	movq	%rsi, %rcx
1421be442: e8 49 bb bb fe              	callq	0x140d79f90 <.text+0xd78f90>
1421be447: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1421be44e: 48 89 f2                    	movq	%rsi, %rdx
1421be451: e8 3a d9 bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421be456: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
1421be45d: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1421be464: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
1421be46b: 0f 11 85 10 02 00 00        	movups	%xmm0, 0x210(%rbp)
1421be472: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
1421be47d: 48 8d 05 44 25 d0 00        	leaq	0xd02544(%rip), %rax    # 0x142ec09c8
1421be484: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1421be48b: 48 c7 85 00 02 00 00 09 00 00 00    	movq	$0x9, 0x200(%rbp)
1421be496: 66 c7 85 08 02 00 00 00 03  	movw	$0x300, 0x208(%rbp)     # imm = 0x300
1421be49f: c6 85 0a 02 00 00 04        	movb	$0x4, 0x20a(%rbp)
1421be4a6: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
1421be4b0: 48 b8 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %rax # imm = 0x3FA6666600000000
1421be4ba: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1421be4c1: 66 c7 85 38 02 00 00 02 00  	movw	$0x2, 0x238(%rbp)
1421be4ca: 66 c7 85 40 02 00 00 02 00  	movw	$0x2, 0x240(%rbp)
1421be4d3: c7 85 48 02 00 00 00 00 00 00       	movl	$0x0, 0x248(%rbp)
1421be4dd: 66 c7 85 5c 02 00 00 01 00  	movw	$0x1, 0x25c(%rbp)
1421be4e6: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
1421be4ed: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1421be4f4: 48 85 d2                    	testq	%rdx, %rdx
1421be4f7: 74 12                       	je	0x1421be50b <.text+0x21bd50b>
1421be4f9: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1421be500: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421be506: e8 15 57 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421be50b: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1421be512: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421be519: e8 b2 dd 00 00              	callq	0x1421cc2d0 <.text+0x21cb2d0>
1421be51e: c7 85 60 01 00 00 00 00 16 44       	movl	$0x44160000, 0x160(%rbp) # imm = 0x44160000
1421be528: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
1421be52f: 0f 10 8d 10 01 00 00        	movups	0x110(%rbp), %xmm1
1421be536: 0f 10 95 20 01 00 00        	movups	0x120(%rbp), %xmm2
1421be53d: 0f 10 9d 30 01 00 00        	movups	0x130(%rbp), %xmm3
1421be544: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
1421be54b: 0f 29 8d 00 02 00 00        	movaps	%xmm1, 0x200(%rbp)
1421be552: 0f 29 95 10 02 00 00        	movaps	%xmm2, 0x210(%rbp)
1421be559: 0f 29 9d 20 02 00 00        	movaps	%xmm3, 0x220(%rbp)
1421be560: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1421be567: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
1421be56e: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1421be575: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
1421be57c: 8b 85 60 01 00 00           	movl	0x160(%rbp), %eax
1421be582: 89 85 50 02 00 00           	movl	%eax, 0x250(%rbp)
1421be588: 48 8b 85 64 01 00 00        	movq	0x164(%rbp), %rax
1421be58f: 48 89 85 54 02 00 00        	movq	%rax, 0x254(%rbp)
1421be596: 8b 85 6c 01 00 00           	movl	0x16c(%rbp), %eax
1421be59c: 89 85 5c 02 00 00           	movl	%eax, 0x25c(%rbp)
1421be5a2: 0f b6 05 60 66 47 01        	movzbl	0x1476660(%rip), %eax   # 0x143634c09
1421be5a9: b9 70 00 00 00              	movl	$0x70, %ecx
1421be5ae: ba 08 00 00 00              	movl	$0x8, %edx
1421be5b3: e8 58 56 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421be5b8: 48 85 c0                    	testq	%rax, %rax
1421be5bb: 0f 84 59 2a 00 00           	je	0x1421c101a <.text+0x21c001a>
1421be5c1: 49 89 c6                    	movq	%rax, %r14
1421be5c4: 0f 28 85 50 02 00 00        	movaps	0x250(%rbp), %xmm0
1421be5cb: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1421be5cf: 0f 28 85 40 02 00 00        	movaps	0x240(%rbp), %xmm0
1421be5d6: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1421be5da: 0f 28 85 30 02 00 00        	movaps	0x230(%rbp), %xmm0
1421be5e1: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1421be5e5: 0f 28 85 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm0
1421be5ec: 0f 28 8d 00 02 00 00        	movaps	0x200(%rbp), %xmm1
1421be5f3: 0f 28 95 10 02 00 00        	movaps	0x210(%rbp), %xmm2
1421be5fa: 0f 28 9d 20 02 00 00        	movaps	0x220(%rbp), %xmm3
1421be601: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421be605: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421be609: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421be60d: 0f 11 00                    	movups	%xmm0, (%rax)
1421be610: 48 89 45 00                 	movq	%rax, (%rbp)
1421be614: 48 8d 05 55 20 d0 00        	leaq	0xd02055(%rip), %rax    # 0x142ec0670
1421be61b: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
1421be61f: c6 85 97 02 00 00 01        	movb	$0x1, 0x297(%rbp)
1421be626: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
1421be62a: 48 89 fa                    	movq	%rdi, %rdx
1421be62d: e8 5e c3 a1 fe              	callq	0x140bda990 <.text+0xbd9990>
1421be632: 44 8b 7d 18                 	movl	0x18(%rbp), %r15d
1421be636: 48 8d 05 03 c3 e3 00        	leaq	0xe3c303(%rip), %rax    # 0x142ffa940
1421be63d: 4a 63 0c b8                 	movslq	(%rax,%r15,4), %rcx
1421be641: 48 01 c1                    	addq	%rax, %rcx
1421be644: ff e1                       	jmpq	*%rcx
1421be646: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1421be651: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1421be65c: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1421be667: c6 85 82 02 00 00 01        	movb	$0x1, 0x282(%rbp)
1421be66e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421be677: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421be67e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421be684: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421be68a: 31 d2                       	xorl	%edx, %edx
1421be68c: e8 4f 6a 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421be691: 4c 8d 2d 30 23 d0 00        	leaq	0xd02330(%rip), %r13    # 0x142ec09c8
1421be698: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421be69f: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
1421be6a6: c7 04 33 e2 9d 8c 20        	movl	$0x208c9de2, (%rbx,%rsi) # imm = 0x208C9DE2
1421be6ad: 48 83 c6 04                 	addq	$0x4, %rsi
1421be6b1: 48 89 b5 80 01 00 00        	movq	%rsi, 0x180(%rbp)
1421be6b8: c6 85 82 02 00 00 01        	movb	$0x1, 0x282(%rbp)
1421be6bf: 48 8d 0d 1a bd e3 00        	leaq	0xe3bd1a(%rip), %rcx    # 0x142ffa3e0
1421be6c6: ba 18 00 00 00              	movl	$0x18, %edx
1421be6cb: e8 b0 ed 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421be6d0: 48 89 d7                    	movq	%rdx, %rdi
1421be6d3: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1421be6da: 48 29 f1                    	subq	%rsi, %rcx
1421be6dd: 48 39 d1                    	cmpq	%rdx, %rcx
1421be6e0: 0f 82 6c 27 00 00           	jb	0x1421c0e52 <.text+0x21bfe52>
1421be6e6: 4d 89 f4                    	movq	%r14, %r12
1421be6e9: 48 01 f3                    	addq	%rsi, %rbx
1421be6ec: 48 89 d9                    	movq	%rbx, %rcx
1421be6ef: 48 89 c2                    	movq	%rax, %rdx
1421be6f2: 49 89 f8                    	movq	%rdi, %r8
1421be6f5: e8 06 55 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421be6fa: 48 8d 04 37                 	leaq	(%rdi,%rsi), %rax
1421be6fe: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1421be705: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421be70c: 0f 10 85 70 01 00 00        	movups	0x170(%rbp), %xmm0
1421be713: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1421be71a: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421be721: 48 8d b5 00 01 00 00        	leaq	0x100(%rbp), %rsi
1421be728: 48 89 f2                    	movq	%rsi, %rdx
1421be72b: e8 60 d6 bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421be730: 48 8b 9d f0 01 00 00        	movq	0x1f0(%rbp), %rbx
1421be737: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1421be73e: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1421be745: 4c 8b b5 00 02 00 00        	movq	0x200(%rbp), %r14
1421be74c: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1421be753: 48 85 d2                    	testq	%rdx, %rdx
1421be756: 74 12                       	je	0x1421be76a <.text+0x21bd76a>
1421be758: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1421be75f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421be765: e8 b6 54 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421be76a: 41 83 ff 02                 	cmpl	$0x2, %r15d
1421be76e: 0f 84 5a 15 00 00           	je	0x1421bfcce <.text+0x21becce>
1421be774: 41 83 ff 03                 	cmpl	$0x3, %r15d
1421be778: 0f 85 f0 19 00 00           	jne	0x1421c016e <.text+0x21bf16e>
1421be77e: 48 8d 45 1c                 	leaq	0x1c(%rbp), %rax
1421be782: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1421be786: 0f b6 05 7c 64 47 01        	movzbl	0x147647c(%rip), %eax   # 0x143634c09
1421be78d: b9 70 00 00 00              	movl	$0x70, %ecx
1421be792: ba 08 00 00 00              	movl	$0x8, %edx
1421be797: e8 74 54 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421be79c: 48 85 c0                    	testq	%rax, %rax
1421be79f: 48 89 9d 60 02 00 00        	movq	%rbx, 0x260(%rbp)
1421be7a6: 0f 84 88 2b 00 00           	je	0x1421c1334 <.text+0x21c0334>
1421be7ac: 48 89 c7                    	movq	%rax, %rdi
1421be7af: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421be7b6: 4c 89 68 08                 	movq	%r13, 0x8(%rax)
1421be7ba: 48 c7 40 10 09 00 00 00     	movq	$0x9, 0x10(%rax)
1421be7c2: 66 c7 40 18 00 03           	movw	$0x300, 0x18(%rax)      # imm = 0x300
1421be7c8: c6 40 1a 04                 	movb	$0x4, 0x1a(%rax)
1421be7cc: 48 89 58 20                 	movq	%rbx, 0x20(%rax)
1421be7d0: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1421be7d7: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
1421be7db: 4c 89 77 30                 	movq	%r14, 0x30(%rdi)
1421be7df: c7 47 38 00 00 00 00        	movl	$0x0, 0x38(%rdi)
1421be7e6: 48 b8 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %rax # imm = 0x3FA6666600000000
1421be7f0: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
1421be7f4: 66 c7 47 48 02 00           	movw	$0x2, 0x48(%rdi)
1421be7fa: 66 c7 47 50 02 00           	movw	$0x2, 0x50(%rdi)
1421be800: c7 47 58 01 00 00 00        	movl	$0x1, 0x58(%rdi)
1421be807: 0f 28 35 d2 8f 7a 00        	movaps	0x7a8fd2(%rip), %xmm6   # 0x1429677e0
1421be80e: 0f 11 77 5c                 	movups	%xmm6, 0x5c(%rdi)
1421be812: 66 c7 47 6c 01 00           	movw	$0x1, 0x6c(%rdi)
1421be818: c6 47 6e 00                 	movb	$0x0, 0x6e(%rdi)
1421be81c: 48 89 bd c0 01 00 00        	movq	%rdi, 0x1c0(%rbp)
1421be823: 4c 8d 25 36 1f d0 00        	leaq	0xd01f36(%rip), %r12    # 0x142ec0760
1421be82a: 4c 89 a5 c8 01 00 00        	movq	%r12, 0x1c8(%rbp)
1421be831: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1421be83c: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1421be847: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1421be852: c6 85 8d 02 00 00 01        	movb	$0x1, 0x28d(%rbp)
1421be859: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421be862: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421be869: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421be86f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421be875: 31 d2                       	xorl	%edx, %edx
1421be877: e8 64 68 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421be87c: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421be883: 4c 8b b5 80 01 00 00        	movq	0x180(%rbp), %r14
1421be88a: 42 c7 04 33 e2 9d 8c 20     	movl	$0x208c9de2, (%rbx,%r14) # imm = 0x208C9DE2
1421be892: 49 83 c6 04                 	addq	$0x4, %r14
1421be896: 4c 89 b5 80 01 00 00        	movq	%r14, 0x180(%rbp)
1421be89d: c6 85 8d 02 00 00 01        	movb	$0x1, 0x28d(%rbp)
1421be8a4: 48 8d 0d 09 bc e3 00        	leaq	0xe3bc09(%rip), %rcx    # 0x142ffa4b4
1421be8ab: ba 14 00 00 00              	movl	$0x14, %edx
1421be8b0: e8 cb eb 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421be8b5: 49 89 d7                    	movq	%rdx, %r15
1421be8b8: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1421be8bf: 4c 29 f1                    	subq	%r14, %rcx
1421be8c2: 48 39 d1                    	cmpq	%rdx, %rcx
1421be8c5: 0f 82 cc 26 00 00           	jb	0x1421c0f97 <.text+0x21bff97>
1421be8cb: 4a 8d 0c 33                 	leaq	(%rbx,%r14), %rcx
1421be8cf: 48 89 c2                    	movq	%rax, %rdx
1421be8d2: 4d 89 f8                    	movq	%r15, %r8
1421be8d5: e8 26 53 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421be8da: 4d 01 fe                    	addq	%r15, %r14
1421be8dd: 4c 89 b5 80 01 00 00        	movq	%r14, 0x180(%rbp)
1421be8e4: 48 8d 45 68                 	leaq	0x68(%rbp), %rax
1421be8e8: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1421be8ef: 48 8d 05 4a cd 63 ff        	leaq	-0x9c32b6(%rip), %rax   # 0x1417fb640 <.text+0x17fa640>
1421be8f6: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1421be8fd: 48 8d 05 cc bb e3 00        	leaq	0xe3bbcc(%rip), %rax    # 0x142ffa4d0
1421be904: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1421be90b: 48 c7 85 f8 01 00 00 02 00 00 00    	movq	$0x2, 0x1f8(%rbp)
1421be916: 48 89 b5 00 02 00 00        	movq	%rsi, 0x200(%rbp)
1421be91d: 48 c7 85 08 02 00 00 01 00 00 00    	movq	$0x1, 0x208(%rbp)
1421be928: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1421be933: c6 85 8d 02 00 00 01        	movb	$0x1, 0x28d(%rbp)
1421be93a: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1421be93e: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421be945: e8 46 b6 bb fe              	callq	0x140d79f90 <.text+0xd78f90>
1421be94a: 4c 8b 7d c8                 	movq	-0x38(%rbp), %r15
1421be94e: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1421be952: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1421be959: 48 8b 75 d8                 	movq	-0x28(%rbp), %rsi
1421be95d: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1421be964: 4c 29 f0                    	subq	%r14, %rax
1421be967: 48 39 f0                    	cmpq	%rsi, %rax
1421be96a: 0f 82 68 26 00 00           	jb	0x1421c0fd8 <.text+0x21bffd8>
1421be970: 4c 01 f3                    	addq	%r14, %rbx
1421be973: 48 89 d9                    	movq	%rbx, %rcx
1421be976: 48 8b 9d 68 02 00 00        	movq	0x268(%rbp), %rbx
1421be97d: 48 89 da                    	movq	%rbx, %rdx
1421be980: 49 89 f0                    	movq	%rsi, %r8
1421be983: e8 78 52 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421be988: 49 01 f6                    	addq	%rsi, %r14
1421be98b: 4c 89 b5 80 01 00 00        	movq	%r14, 0x180(%rbp)
1421be992: 4d 85 ff                    	testq	%r15, %r15
1421be995: 74 11                       	je	0x1421be9a8 <.text+0x21bd9a8>
1421be997: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421be99d: 48 89 d9                    	movq	%rbx, %rcx
1421be9a0: 4c 89 fa                    	movq	%r15, %rdx
1421be9a3: e8 78 52 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421be9a8: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1421be9af: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421be9b6: 0f 10 85 70 01 00 00        	movups	0x170(%rbp), %xmm0
1421be9bd: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1421be9c4: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421be9cb: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421be9d2: e8 b9 d3 bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421be9d7: 49 be 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %r14 # imm = 0x3FA6666600000000
1421be9e1: 4c 8b bd f0 01 00 00        	movq	0x1f0(%rbp), %r15
1421be9e8: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1421be9ef: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1421be9f6: 48 8b b5 00 02 00 00        	movq	0x200(%rbp), %rsi
1421be9fd: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1421bea04: 48 85 d2                    	testq	%rdx, %rdx
1421bea07: 74 12                       	je	0x1421bea1b <.text+0x21bda1b>
1421bea09: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1421bea10: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421bea16: e8 05 52 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421bea1b: 8b 85 b8 00 00 00           	movl	0xb8(%rbp), %eax
1421bea21: 89 85 90 00 00 00           	movl	%eax, 0x90(%rbp)
1421bea27: 0f b6 85 bc 00 00 00        	movzbl	0xbc(%rbp), %eax
1421bea2e: 88 85 94 00 00 00           	movb	%al, 0x94(%rbp)
1421bea34: 8b 85 f0 01 00 00           	movl	0x1f0(%rbp), %eax
1421bea3a: 89 85 70 01 00 00           	movl	%eax, 0x170(%rbp)
1421bea40: 0f b7 85 f4 01 00 00        	movzwl	0x1f4(%rbp), %eax
1421bea47: 66 89 85 74 01 00 00        	movw	%ax, 0x174(%rbp)
1421bea4e: 8b 85 00 01 00 00           	movl	0x100(%rbp), %eax
1421bea54: 89 85 80 00 00 00           	movl	%eax, 0x80(%rbp)
1421bea5a: 0f b7 85 04 01 00 00        	movzwl	0x104(%rbp), %eax
1421bea61: 66 89 85 84 00 00 00        	movw	%ax, 0x84(%rbp)
1421bea68: 0f b6 05 9a 61 47 01        	movzbl	0x147619a(%rip), %eax   # 0x143634c09
1421bea6f: b9 70 00 00 00              	movl	$0x70, %ecx
1421bea74: ba 08 00 00 00              	movl	$0x8, %edx
1421bea79: e8 92 51 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bea7e: 48 85 c0                    	testq	%rax, %rax
1421bea81: 0f 84 19 29 00 00           	je	0x1421c13a0 <.text+0x21c03a0>
1421bea87: 48 89 c3                    	movq	%rax, %rbx
1421bea8a: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421bea91: 4c 89 68 08                 	movq	%r13, 0x8(%rax)
1421bea95: 48 c7 40 10 09 00 00 00     	movq	$0x9, 0x10(%rax)
1421bea9d: 66 c7 40 18 00 03           	movw	$0x300, 0x18(%rax)      # imm = 0x300
1421beaa3: c6 40 1a 04                 	movb	$0x4, 0x1a(%rax)
1421beaa7: 8b 85 90 00 00 00           	movl	0x90(%rbp), %eax
1421beaad: 89 43 1b                    	movl	%eax, 0x1b(%rbx)
1421beab0: 0f b6 85 94 00 00 00        	movzbl	0x94(%rbp), %eax
1421beab7: 88 43 1f                    	movb	%al, 0x1f(%rbx)
1421beaba: 4c 89 7b 20                 	movq	%r15, 0x20(%rbx)
1421beabe: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
1421beac5: 48 89 43 28                 	movq	%rax, 0x28(%rbx)
1421beac9: 48 89 73 30                 	movq	%rsi, 0x30(%rbx)
1421beacd: c7 43 38 00 00 00 00        	movl	$0x0, 0x38(%rbx)
1421bead4: 4c 89 73 40                 	movq	%r14, 0x40(%rbx)
1421bead8: 66 c7 43 48 02 00           	movw	$0x2, 0x48(%rbx)
1421beade: 8b 85 70 01 00 00           	movl	0x170(%rbp), %eax
1421beae4: 89 43 4a                    	movl	%eax, 0x4a(%rbx)
1421beae7: 0f b7 85 74 01 00 00        	movzwl	0x174(%rbp), %eax
1421beaee: 66 89 43 4e                 	movw	%ax, 0x4e(%rbx)
1421beaf2: 66 c7 43 50 02 00           	movw	$0x2, 0x50(%rbx)
1421beaf8: 0f b7 85 84 00 00 00        	movzwl	0x84(%rbp), %eax
1421beaff: 66 89 43 56                 	movw	%ax, 0x56(%rbx)
1421beb03: 8b 85 80 00 00 00           	movl	0x80(%rbp), %eax
1421beb09: 89 43 52                    	movl	%eax, 0x52(%rbx)
1421beb0c: c7 43 58 01 00 00 00        	movl	$0x1, 0x58(%rbx)
1421beb13: 0f 11 73 5c                 	movups	%xmm6, 0x5c(%rbx)
1421beb17: 66 c7 43 6c 01 00           	movw	$0x1, 0x6c(%rbx)
1421beb1d: c6 43 6e 00                 	movb	$0x0, 0x6e(%rbx)
1421beb21: 48 89 9d b8 00 00 00        	movq	%rbx, 0xb8(%rbp)
1421beb28: 4c 89 a5 c0 00 00 00        	movq	%r12, 0xc0(%rbp)
1421beb2f: 0f b6 05 d3 60 47 01        	movzbl	0x14760d3(%rip), %eax   # 0x143634c09
1421beb36: b9 10 00 00 00              	movl	$0x10, %ecx
1421beb3b: ba 04 00 00 00              	movl	$0x4, %edx
1421beb40: e8 cb 50 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421beb45: 48 85 c0                    	testq	%rax, %rax
1421beb48: 0f 84 99 28 00 00           	je	0x1421c13e7 <.text+0x21c03e7>
1421beb4e: 48 89 c6                    	movq	%rax, %rsi
1421beb51: 48 b8 03 00 00 00 00 00 c8 41       	movabsq	$0x41c8000000000003, %rax # imm = 0x41C8000000000003
1421beb5b: 48 89 06                    	movq	%rax, (%rsi)
1421beb5e: 48 c7 46 08 03 00 00 00     	movq	$0x3, 0x8(%rsi)
1421beb66: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
1421beb6d: 4c 8d 35 b4 17 d0 00        	leaq	0xd017b4(%rip), %r14    # 0x142ec0328
1421beb74: 4c 89 b5 78 01 00 00        	movq	%r14, 0x178(%rbp)
1421beb7b: 0f b6 05 87 60 47 01        	movzbl	0x1476087(%rip), %eax   # 0x143634c09
1421beb82: b9 10 00 00 00              	movl	$0x10, %ecx
1421beb87: ba 04 00 00 00              	movl	$0x4, %edx
1421beb8c: e8 7f 50 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421beb91: 48 85 c0                    	testq	%rax, %rax
1421beb94: 0f 94 85 68 02 00 00        	sete	0x268(%rbp)
1421beb9b: 0f 84 5e 28 00 00           	je	0x1421c13ff <.text+0x21c03ff>
1421beba1: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
1421beba8: 48 b9 03 00 00 00 00 00 a0 40       	movabsq	$0x40a0000000000003, %rcx # imm = 0x40A0000000000003
1421bebb2: 48 89 48 08                 	movq	%rcx, 0x8(%rax)
1421bebb6: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1421bebbd: 4c 89 b5 08 01 00 00        	movq	%r14, 0x108(%rbp)
1421bebc4: 0f 28 45 00                 	movaps	(%rbp), %xmm0
1421bebc8: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
1421bebcf: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bebd6: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bebdd: e8 4e bb 00 00              	callq	0x1421ca730 <.text+0x21c9730>
1421bebe2: 0f b6 05 20 60 47 01        	movzbl	0x1476020(%rip), %eax   # 0x143634c09
1421bebe9: b9 48 00 00 00              	movl	$0x48, %ecx
1421bebee: ba 08 00 00 00              	movl	$0x8, %edx
1421bebf3: e8 18 50 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bebf8: 48 85 c0                    	testq	%rax, %rax
1421bebfb: 0f 84 20 28 00 00           	je	0x1421c1421 <.text+0x21c0421>
1421bec01: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421bec08: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421bec0c: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421bec13: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421bec1a: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421bec21: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421bec28: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421bec2c: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421bec30: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421bec34: 0f 11 00                    	movups	%xmm0, (%rax)
1421bec37: 48 89 b5 00 01 00 00        	movq	%rsi, 0x100(%rbp)
1421bec3e: 4c 89 b5 08 01 00 00        	movq	%r14, 0x108(%rbp)
1421bec45: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421bec4c: 48 8d 35 b5 18 d0 00        	leaq	0xd018b5(%rip), %rsi    # 0x142ec0508
1421bec53: 48 89 b5 18 01 00 00        	movq	%rsi, 0x118(%rbp)
1421bec5a: c6 85 89 02 00 00 00        	movb	$0x0, 0x289(%rbp)
1421bec61: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bec68: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bec6f: e8 4c 07 8a ff              	callq	0x141a5f3c0 <.text+0x1a5e3c0>
1421bec74: 0f b6 05 8e 5f 47 01        	movzbl	0x1475f8e(%rip), %eax   # 0x143634c09
1421bec7b: b9 40 00 00 00              	movl	$0x40, %ecx
1421bec80: ba 08 00 00 00              	movl	$0x8, %edx
1421bec85: e8 86 4f 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bec8a: 48 85 c0                    	testq	%rax, %rax
1421bec8d: 0f 84 b0 27 00 00           	je	0x1421c1443 <.text+0x21c0443>
1421bec93: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421bec9a: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421beca1: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421beca8: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421becaf: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421becb3: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421becb7: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421becbb: 0f 11 00                    	movups	%xmm0, (%rax)
1421becbe: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
1421becc5: 4c 89 a5 08 01 00 00        	movq	%r12, 0x108(%rbp)
1421beccc: 48 89 9d 10 01 00 00        	movq	%rbx, 0x110(%rbp)
1421becd3: 4c 89 a5 18 01 00 00        	movq	%r12, 0x118(%rbp)
1421becda: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1421bece1: 48 8d 05 d8 14 d0 00        	leaq	0xd014d8(%rip), %rax    # 0x142ec01c0
1421bece8: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1421becef: c6 85 96 02 00 00 00        	movb	$0x0, 0x296(%rbp)
1421becf6: c6 85 95 02 00 00 00        	movb	$0x0, 0x295(%rbp)
1421becfd: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bed04: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bed0b: e8 c0 bb 00 00              	callq	0x1421ca8d0 <.text+0x21c98d0>
1421bed10: 0f b6 05 f2 5e 47 01        	movzbl	0x1475ef2(%rip), %eax   # 0x143634c09
1421bed17: b9 48 00 00 00              	movl	$0x48, %ecx
1421bed1c: ba 08 00 00 00              	movl	$0x8, %edx
1421bed21: e8 ea 4e 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bed26: 48 85 c0                    	testq	%rax, %rax
1421bed29: 0f 85 01 14 00 00           	jne	0x1421c0130 <.text+0x21bf130>
1421bed2f: b9 08 00 00 00              	movl	$0x8, %ecx
1421bed34: ba 48 00 00 00              	movl	$0x48, %edx
1421bed39: e8 c2 a2 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421bed3e: e9 20 27 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421bed43: 0f b6 05 bf 5e 47 01        	movzbl	0x1475ebf(%rip), %eax   # 0x143634c09
1421bed4a: b9 20 00 00 00              	movl	$0x20, %ecx
1421bed4f: ba 08 00 00 00              	movl	$0x8, %edx
1421bed54: e8 b7 4e 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bed59: 48 85 c0                    	testq	%rax, %rax
1421bed5c: 0f 84 bc 23 00 00           	je	0x1421c111e <.text+0x21c011e>
1421bed62: 0f 10 05 67 b6 e3 00        	movups	0xe3b667(%rip), %xmm0   # 0x142ffa3d0
1421bed69: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1421bed6d: 0f 10 05 4c b6 e3 00        	movups	0xe3b64c(%rip), %xmm0   # 0x142ffa3c0
1421bed74: 0f 11 00                    	movups	%xmm0, (%rax)
1421bed77: 0f 10 45 20                 	movups	0x20(%rbp), %xmm0
1421bed7b: 48 c7 85 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rbp)
1421bed86: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1421bed8d: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1421bed94: 48 c7 85 10 01 00 00 01 00 00 00    	movq	$0x1, 0x110(%rbp)
1421bed9f: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
1421beda6: 48 8d 0d 59 b7 e3 00        	leaq	0xe3b759(%rip), %rcx    # 0x142ffa506
1421bedad: ba 20 00 00 00              	movl	$0x20, %edx
1421bedb2: e8 c9 e6 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421bedb7: 48 bb 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %rbx # imm = 0x3FA6666600000000
1421bedc1: 4c 8d 3d 00 1c d0 00        	leaq	0xd01c00(%rip), %r15    # 0x142ec09c8
1421bedc8: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
1421bedcf: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1421bedd6: 48 89 95 c8 01 00 00        	movq	%rdx, 0x1c8(%rbp)
1421beddd: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1421bede4: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1421bedeb: 4c 89 a5 78 01 00 00        	movq	%r12, 0x178(%rbp)
1421bedf2: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
1421bedf9: 4c 89 a5 88 01 00 00        	movq	%r12, 0x188(%rbp)
1421bee00: 48 8d 05 21 b7 e3 00        	leaq	0xe3b721(%rip), %rax    # 0x142ffa528
1421bee07: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1421bee0e: 48 c7 85 f8 01 00 00 02 00 00 00    	movq	$0x2, 0x1f8(%rbp)
1421bee19: 48 89 b5 00 02 00 00        	movq	%rsi, 0x200(%rbp)
1421bee20: 48 c7 85 08 02 00 00 02 00 00 00    	movq	$0x2, 0x208(%rbp)
1421bee2b: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1421bee36: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
1421bee3d: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421bee44: e8 47 b1 bb fe              	callq	0x140d79f90 <.text+0xd78f90>
1421bee49: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
1421bee50: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1421bee57: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1421bee5e: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1421bee65: 48 8b bd c8 00 00 00        	movq	0xc8(%rbp), %rdi
1421bee6c: be 01 00 00 00              	movl	$0x1, %esi
1421bee71: ba 01 00 00 00              	movl	$0x1, %edx
1421bee76: e9 3b 01 00 00              	jmp	0x1421befb6 <.text+0x21bdfb6>
1421bee7b: 4c 8b 45 20                 	movq	0x20(%rbp), %r8
1421bee7f: 4c 8b 4d 28                 	movq	0x28(%rbp), %r9
1421bee83: 8b 57 58                    	movl	0x58(%rdi), %edx
1421bee86: c6 85 97 02 00 00 01        	movb	$0x1, 0x297(%rbp)
1421bee8d: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421bee94: e8 37 48 00 00              	callq	0x1421c36d0 <.text+0x21c26d0>
1421bee99: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1421beea0: 48 89 c8                    	movq	%rcx, %rax
1421beea3: 48 f7 d8                    	negq	%rax
1421beea6: 0f 80 9a f7 ff ff           	jo	0x1421be646 <.text+0x21bd646>
1421beeac: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
1421beeb3: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1421beeba: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1421beec1: 0f 10 85 80 01 00 00        	movups	0x180(%rbp), %xmm0
1421beec8: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1421beecf: 0f 10 85 70 01 00 00        	movups	0x170(%rbp), %xmm0
1421beed6: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1421beedd: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
1421beee4: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1421beeeb: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
1421beef2: 48 8d 0d 0d b6 e3 00        	leaq	0xe3b60d(%rip), %rcx    # 0x142ffa506
1421beef9: ba 20 00 00 00              	movl	$0x20, %edx
1421beefe: e8 7d e5 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421bef03: 4c 8d 3d be 1a d0 00        	leaq	0xd01abe(%rip), %r15    # 0x142ec09c8
1421bef0a: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1421bef0e: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
1421bef12: 48 8d 85 18 01 00 00        	leaq	0x118(%rbp), %rax
1421bef19: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1421bef1d: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
1421bef24: 4c 89 a5 c0 00 00 00        	movq	%r12, 0xc0(%rbp)
1421bef2b: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1421bef32: 4c 89 a5 d0 00 00 00        	movq	%r12, 0xd0(%rbp)
1421bef39: 48 8d 05 e8 b5 e3 00        	leaq	0xe3b5e8(%rip), %rax    # 0x142ffa528
1421bef40: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1421bef47: 48 c7 85 f8 01 00 00 02 00 00 00    	movq	$0x2, 0x1f8(%rbp)
1421bef52: 48 89 9d 00 02 00 00        	movq	%rbx, 0x200(%rbp)
1421bef59: 48 c7 85 08 02 00 00 02 00 00 00    	movq	$0x2, 0x208(%rbp)
1421bef64: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1421bef6f: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1421bef76: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421bef7d: e8 0e b0 bb fe              	callq	0x140d79f90 <.text+0xd78f90>
1421bef82: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1421bef89: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1421bef90: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
1421bef97: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1421bef9e: 48 8b bd d0 01 00 00        	movq	0x1d0(%rbp), %rdi
1421befa5: 48 bb 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %rbx # imm = 0x3FA6666600000000
1421befaf: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1421befb6: 48 c1 e6 05                 	shlq	$0x5, %rsi
1421befba: 48 8b 8d e0 01 00 00        	movq	0x1e0(%rbp), %rcx
1421befc1: 48 01 ce                    	addq	%rcx, %rsi
1421befc4: 0f 57 c0                    	xorps	%xmm0, %xmm0
1421befc7: 0f 11 85 be 00 00 00        	movups	%xmm0, 0xbe(%rbp)
1421befce: c7 85 ce 00 00 00 00 00 00 00       	movl	$0x0, 0xce(%rbp)
1421befd8: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
1421befe3: 48 c7 85 f8 01 00 00 08 00 00 00    	movq	$0x8, 0x1f8(%rbp)
1421befee: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
1421beff9: 66 c7 85 08 02 00 00 02 00  	movw	$0x2, 0x208(%rbp)
1421bf002: 66 c7 85 10 02 00 00 02 00  	movw	$0x2, 0x210(%rbp)
1421bf00b: 0f 10 85 b8 00 00 00        	movups	0xb8(%rbp), %xmm0
1421bf012: 0f 11 85 12 02 00 00        	movups	%xmm0, 0x212(%rbp)
1421bf019: 48 8b 85 c2 00 00 00        	movq	0xc2(%rbp), %rax
1421bf020: 48 89 85 1c 02 00 00        	movq	%rax, 0x21c(%rbp)
1421bf027: 8b 85 ca 00 00 00           	movl	0xca(%rbp), %eax
1421bf02d: 89 85 24 02 00 00           	movl	%eax, 0x224(%rbp)
1421bf033: 8b 85 ce 00 00 00           	movl	0xce(%rbp), %eax
1421bf039: 89 85 28 02 00 00           	movl	%eax, 0x228(%rbp)
1421bf03f: c7 85 2c 02 00 00 00 00 80 7f       	movl	$0x7f800000, 0x22c(%rbp) # imm = 0x7F800000
1421bf049: 66 c7 85 30 02 00 00 00 00  	movw	$0x0, 0x230(%rbp)
1421bf052: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
1421bf059: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
1421bf060: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
1421bf067: 48 89 95 80 01 00 00        	movq	%rdx, 0x180(%rbp)
1421bf06e: 48 89 b5 88 01 00 00        	movq	%rsi, 0x188(%rbp)
1421bf075: c6 85 7e 02 00 00 01        	movb	$0x1, 0x27e(%rbp)
1421bf07c: c6 85 7d 02 00 00 01        	movb	$0x1, 0x27d(%rbp)
1421bf083: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1421bf08a: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1421bf091: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
1421bf098: e8 03 95 00 00              	callq	0x1421c85a0 <.text+0x21c75a0>
1421bf09d: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1421bf0a4: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1421bf0ab: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1421bf0b2: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1421bf0b9: 48 89 bd 80 01 00 00        	movq	%rdi, 0x180(%rbp)
1421bf0c0: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf0c7: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1421bf0ce: e8 bd cc bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421bf0d3: 4c 8b a5 f0 01 00 00        	movq	0x1f0(%rbp), %r12
1421bf0da: 4c 8b ad f8 01 00 00        	movq	0x1f8(%rbp), %r13
1421bf0e1: 48 8b bd 00 02 00 00        	movq	0x200(%rbp), %rdi
1421bf0e8: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1421bf0ef: 48 85 d2                    	testq	%rdx, %rdx
1421bf0f2: 74 12                       	je	0x1421bf106 <.text+0x21be106>
1421bf0f4: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1421bf0fb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421bf101: e8 1a 4b 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421bf106: 0f b6 05 fc 5a 47 01        	movzbl	0x1475afc(%rip), %eax   # 0x143634c09
1421bf10d: b9 30 00 00 00              	movl	$0x30, %ecx
1421bf112: ba 08 00 00 00              	movl	$0x8, %edx
1421bf117: e8 f4 4a 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf11c: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1421bf120: 48 85 c0                    	testq	%rax, %rax
1421bf123: 4c 89 a5 a8 00 00 00        	movq	%r12, 0xa8(%rbp)
1421bf12a: 4c 89 6d 58                 	movq	%r13, 0x58(%rbp)
1421bf12e: 0f 84 fa 1e 00 00           	je	0x1421c102e <.text+0x21c002e>
1421bf134: 0f b6 05 ce 5a 47 01        	movzbl	0x1475ace(%rip), %eax   # 0x143634c09
1421bf13b: b9 70 00 00 00              	movl	$0x70, %ecx
1421bf140: ba 08 00 00 00              	movl	$0x8, %edx
1421bf145: e8 c6 4a 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf14a: 48 85 c0                    	testq	%rax, %rax
1421bf14d: 0f 84 fd 1e 00 00           	je	0x1421c1050 <.text+0x21c0050>
1421bf153: 48 89 c6                    	movq	%rax, %rsi
1421bf156: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421bf15d: 4c 89 78 08                 	movq	%r15, 0x8(%rax)
1421bf161: 48 c7 40 10 09 00 00 00     	movq	$0x9, 0x10(%rax)
1421bf169: 66 c7 40 18 00 03           	movw	$0x300, 0x18(%rax)      # imm = 0x300
1421bf16f: c6 40 1a 04                 	movb	$0x4, 0x1a(%rax)
1421bf173: 4c 89 60 20                 	movq	%r12, 0x20(%rax)
1421bf177: 4c 89 68 28                 	movq	%r13, 0x28(%rax)
1421bf17b: 48 89 78 30                 	movq	%rdi, 0x30(%rax)
1421bf17f: c7 40 38 00 00 00 00        	movl	$0x0, 0x38(%rax)
1421bf186: 48 89 58 40                 	movq	%rbx, 0x40(%rax)
1421bf18a: 66 c7 40 48 02 00           	movw	$0x2, 0x48(%rax)
1421bf190: 66 c7 40 50 02 00           	movw	$0x2, 0x50(%rax)
1421bf196: c7 40 58 00 00 00 00        	movl	$0x0, 0x58(%rax)
1421bf19d: 66 c7 40 6c 01 00           	movw	$0x1, 0x6c(%rax)
1421bf1a3: c6 40 6e 00                 	movb	$0x0, 0x6e(%rax)
1421bf1a7: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
1421bf1ae: 4c 8d 3d ab 15 d0 00        	leaq	0xd015ab(%rip), %r15    # 0x142ec0760
1421bf1b5: 4c 89 bd c0 00 00 00        	movq	%r15, 0xc0(%rbp)
1421bf1bc: 0f b6 05 46 5a 47 01        	movzbl	0x1475a46(%rip), %eax   # 0x143634c09
1421bf1c3: b9 10 00 00 00              	movl	$0x10, %ecx
1421bf1c8: ba 04 00 00 00              	movl	$0x4, %edx
1421bf1cd: e8 3e 4a 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf1d2: 48 85 c0                    	testq	%rax, %rax
1421bf1d5: 0f 94 c3                    	sete	%bl
1421bf1d8: 0f 84 86 1e 00 00           	je	0x1421c1064 <.text+0x21c0064>
1421bf1de: 48 89 c7                    	movq	%rax, %rdi
1421bf1e1: 4d 89 f5                    	movq	%r14, %r13
1421bf1e4: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
1421bf1eb: 48 b8 03 00 00 00 00 00 a0 40       	movabsq	$0x40a0000000000003, %rax # imm = 0x40A0000000000003
1421bf1f5: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1421bf1f9: 48 89 bd 70 01 00 00        	movq	%rdi, 0x170(%rbp)
1421bf200: 4c 8d 25 21 11 d0 00        	leaq	0xd01121(%rip), %r12    # 0x142ec0328
1421bf207: 4c 89 a5 78 01 00 00        	movq	%r12, 0x178(%rbp)
1421bf20e: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
1421bf215: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1421bf21c: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
1421bf223: 0f 10 8d 10 01 00 00        	movups	0x110(%rbp), %xmm1
1421bf22a: 0f 10 95 20 01 00 00        	movups	0x120(%rbp), %xmm2
1421bf231: 0f 10 9d 30 01 00 00        	movups	0x130(%rbp), %xmm3
1421bf238: 0f 29 9d 20 02 00 00        	movaps	%xmm3, 0x220(%rbp)
1421bf23f: 0f 29 95 10 02 00 00        	movaps	%xmm2, 0x210(%rbp)
1421bf246: 0f 29 8d 00 02 00 00        	movaps	%xmm1, 0x200(%rbp)
1421bf24d: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
1421bf254: 0f b6 05 ae 59 47 01        	movzbl	0x14759ae(%rip), %eax   # 0x143634c09
1421bf25b: b9 48 00 00 00              	movl	$0x48, %ecx
1421bf260: ba 08 00 00 00              	movl	$0x8, %edx
1421bf265: e8 a6 49 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf26a: 48 85 c0                    	testq	%rax, %rax
1421bf26d: 0f 84 0b 1e 00 00           	je	0x1421c107e <.text+0x21c007e>
1421bf273: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421bf27a: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421bf27e: 0f 28 85 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm0
1421bf285: 0f 28 8d 00 02 00 00        	movaps	0x200(%rbp), %xmm1
1421bf28c: 0f 28 95 10 02 00 00        	movaps	0x210(%rbp), %xmm2
1421bf293: 0f 28 9d 20 02 00 00        	movaps	0x220(%rbp), %xmm3
1421bf29a: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421bf29e: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421bf2a2: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421bf2a6: 0f 11 00                    	movups	%xmm0, (%rax)
1421bf2a9: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
1421bf2ad: 48 89 31                    	movq	%rsi, (%rcx)
1421bf2b0: 4c 89 79 08                 	movq	%r15, 0x8(%rcx)
1421bf2b4: 48 89 79 10                 	movq	%rdi, 0x10(%rcx)
1421bf2b8: 4c 89 61 18                 	movq	%r12, 0x18(%rcx)
1421bf2bc: 48 89 41 20                 	movq	%rax, 0x20(%rcx)
1421bf2c0: 48 8d 35 41 12 d0 00        	leaq	0xd01241(%rip), %rsi    # 0x142ec0508
1421bf2c7: 48 89 71 28                 	movq	%rsi, 0x28(%rcx)
1421bf2cb: 48 c7 85 c0 01 00 00 03 00 00 00    	movq	$0x3, 0x1c0(%rbp)
1421bf2d6: 48 89 8d c8 01 00 00        	movq	%rcx, 0x1c8(%rbp)
1421bf2dd: 48 c7 85 d0 01 00 00 03 00 00 00    	movq	$0x3, 0x1d0(%rbp)
1421bf2e8: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
1421bf2f3: 48 c7 85 c0 00 00 00 01 00 00 00    	movq	$0x1, 0xc0(%rbp)
1421bf2fe: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
1421bf309: c6 85 86 02 00 00 01        	movb	$0x1, 0x286(%rbp)
1421bf310: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421bf319: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
1421bf320: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421bf326: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421bf32c: 31 d2                       	xorl	%edx, %edx
1421bf32e: e8 ad 5d 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421bf333: 4c 8b b5 c0 00 00 00        	movq	0xc0(%rbp), %r14
1421bf33a: 48 8b bd c8 00 00 00        	movq	0xc8(%rbp), %rdi
1421bf341: 41 c7 04 3e e2 9d 8c 20     	movl	$0x208c9de2, (%r14,%rdi) # imm = 0x208C9DE2
1421bf349: 48 83 c7 04                 	addq	$0x4, %rdi
1421bf34d: 48 89 bd c8 00 00 00        	movq	%rdi, 0xc8(%rbp)
1421bf354: c6 85 86 02 00 00 01        	movb	$0x1, 0x286(%rbp)
1421bf35b: 48 8d 0d 7e b0 e3 00        	leaq	0xe3b07e(%rip), %rcx    # 0x142ffa3e0
1421bf362: ba 18 00 00 00              	movl	$0x18, %edx
1421bf367: e8 14 e1 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421bf36c: 48 89 d3                    	movq	%rdx, %rbx
1421bf36f: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
1421bf376: 48 29 f9                    	subq	%rdi, %rcx
1421bf379: 48 39 d1                    	cmpq	%rdx, %rcx
1421bf37c: 0f 82 11 1b 00 00           	jb	0x1421c0e93 <.text+0x21bfe93>
1421bf382: 49 01 fe                    	addq	%rdi, %r14
1421bf385: 4c 89 f1                    	movq	%r14, %rcx
1421bf388: 48 89 c2                    	movq	%rax, %rdx
1421bf38b: 49 89 d8                    	movq	%rbx, %r8
1421bf38e: e8 6d 48 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421bf393: 48 8d 04 3b                 	leaq	(%rbx,%rdi), %rax
1421bf397: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1421bf39e: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1421bf3a5: 0f 10 85 b8 00 00 00        	movups	0xb8(%rbp), %xmm0
1421bf3ac: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
1421bf3b3: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf3ba: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1421bf3c1: e8 ca c9 bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421bf3c6: 4c 8b b5 f0 01 00 00        	movq	0x1f0(%rbp), %r14
1421bf3cd: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1421bf3d4: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
1421bf3db: 48 8b 9d 00 02 00 00        	movq	0x200(%rbp), %rbx
1421bf3e2: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1421bf3e9: 48 85 d2                    	testq	%rdx, %rdx
1421bf3ec: 74 12                       	je	0x1421bf400 <.text+0x21be400>
1421bf3ee: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1421bf3f5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421bf3fb: e8 20 48 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421bf400: 0f b6 05 02 58 47 01        	movzbl	0x1475802(%rip), %eax   # 0x143634c09
1421bf407: b9 70 00 00 00              	movl	$0x70, %ecx
1421bf40c: ba 08 00 00 00              	movl	$0x8, %edx
1421bf411: e8 fa 47 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf416: 48 85 c0                    	testq	%rax, %rax
1421bf419: 0f 84 79 1c 00 00           	je	0x1421c1098 <.text+0x21c0098>
1421bf41f: 48 89 c7                    	movq	%rax, %rdi
1421bf422: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421bf429: 48 8d 05 98 15 d0 00        	leaq	0xd01598(%rip), %rax    # 0x142ec09c8
1421bf430: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1421bf434: 48 c7 47 10 09 00 00 00     	movq	$0x9, 0x10(%rdi)
1421bf43c: 66 c7 47 18 00 03           	movw	$0x300, 0x18(%rdi)      # imm = 0x300
1421bf442: c6 47 1a 04                 	movb	$0x4, 0x1a(%rdi)
1421bf446: 4c 89 77 20                 	movq	%r14, 0x20(%rdi)
1421bf44a: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
1421bf451: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
1421bf455: 48 89 5f 30                 	movq	%rbx, 0x30(%rdi)
1421bf459: c7 47 38 00 00 00 00        	movl	$0x0, 0x38(%rdi)
1421bf460: 48 b8 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %rax # imm = 0x3FA6666600000000
1421bf46a: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
1421bf46e: 66 c7 47 48 02 00           	movw	$0x2, 0x48(%rdi)
1421bf474: 66 c7 47 50 02 00           	movw	$0x2, 0x50(%rdi)
1421bf47a: c7 47 58 01 00 00 00        	movl	$0x1, 0x58(%rdi)
1421bf481: 0f 28 05 58 83 7a 00        	movaps	0x7a8358(%rip), %xmm0   # 0x1429677e0
1421bf488: 0f 11 47 5c                 	movups	%xmm0, 0x5c(%rdi)
1421bf48c: 66 c7 47 6c 01 00           	movw	$0x1, 0x6c(%rdi)
1421bf492: c6 47 6e 00                 	movb	$0x0, 0x6e(%rdi)
1421bf496: 48 89 bd 90 00 00 00        	movq	%rdi, 0x90(%rbp)
1421bf49d: 4c 89 bd 98 00 00 00        	movq	%r15, 0x98(%rbp)
1421bf4a4: 0f b6 05 5e 57 47 01        	movzbl	0x147575e(%rip), %eax   # 0x143634c09
1421bf4ab: b9 10 00 00 00              	movl	$0x10, %ecx
1421bf4b0: ba 04 00 00 00              	movl	$0x4, %edx
1421bf4b5: e8 56 47 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf4ba: 48 85 c0                    	testq	%rax, %rax
1421bf4bd: 0f 84 f0 1b 00 00           	je	0x1421c10b3 <.text+0x21c00b3>
1421bf4c3: 48 89 c3                    	movq	%rax, %rbx
1421bf4c6: 48 b8 03 00 00 00 00 00 c8 41       	movabsq	$0x41c8000000000003, %rax # imm = 0x41C8000000000003
1421bf4d0: 48 89 03                    	movq	%rax, (%rbx)
1421bf4d3: 48 c7 43 08 03 00 00 00     	movq	$0x3, 0x8(%rbx)
1421bf4db: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
1421bf4e2: 4c 89 a5 88 00 00 00        	movq	%r12, 0x88(%rbp)
1421bf4e9: 0f b6 05 19 57 47 01        	movzbl	0x1475719(%rip), %eax   # 0x143634c09
1421bf4f0: b9 10 00 00 00              	movl	$0x10, %ecx
1421bf4f5: ba 04 00 00 00              	movl	$0x4, %edx
1421bf4fa: e8 11 47 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf4ff: 48 85 c0                    	testq	%rax, %rax
1421bf502: 0f 94 85 a0 00 00 00        	sete	0xa0(%rbp)
1421bf509: 0f 84 bf 1b 00 00           	je	0x1421c10ce <.text+0x21c00ce>
1421bf50f: 49 89 c6                    	movq	%rax, %r14
1421bf512: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
1421bf519: 48 b8 03 00 00 00 00 00 a0 40       	movabsq	$0x40a0000000000003, %rax # imm = 0x40A0000000000003
1421bf523: 49 89 46 08                 	movq	%rax, 0x8(%r14)
1421bf527: 4c 89 75 68                 	movq	%r14, 0x68(%rbp)
1421bf52b: 4c 89 65 70                 	movq	%r12, 0x70(%rbp)
1421bf52f: 4c 89 ad b8 00 00 00        	movq	%r13, 0xb8(%rbp)
1421bf536: 48 8d 05 33 11 d0 00        	leaq	0xd01133(%rip), %rax    # 0x142ec0670
1421bf53d: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1421bf544: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1421bf54b: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1421bf552: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1421bf559: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
1421bf560: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf567: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1421bf56e: e8 cd d6 89 ff              	callq	0x141a5cc40 <.text+0x1a5bc40>
1421bf573: 0f b6 05 8f 56 47 01        	movzbl	0x147568f(%rip), %eax   # 0x143634c09
1421bf57a: b9 48 00 00 00              	movl	$0x48, %ecx
1421bf57f: ba 08 00 00 00              	movl	$0x8, %edx
1421bf584: e8 87 46 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf589: 48 85 c0                    	testq	%rax, %rax
1421bf58c: 0f 84 50 1b 00 00           	je	0x1421c10e2 <.text+0x21c00e2>
1421bf592: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421bf599: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421bf59d: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421bf5a4: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421bf5ab: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421bf5b2: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421bf5b9: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421bf5bd: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421bf5c1: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421bf5c5: 0f 11 00                    	movups	%xmm0, (%rax)
1421bf5c8: 4c 89 b5 70 01 00 00        	movq	%r14, 0x170(%rbp)
1421bf5cf: 4c 89 a5 78 01 00 00        	movq	%r12, 0x178(%rbp)
1421bf5d6: 4c 89 ad 80 01 00 00        	movq	%r13, 0x180(%rbp)
1421bf5dd: 48 8d 0d 8c 10 d0 00        	leaq	0xd0108c(%rip), %rcx    # 0x142ec0670
1421bf5e4: 48 89 8d 88 01 00 00        	movq	%rcx, 0x188(%rbp)
1421bf5eb: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1421bf5f2: 48 89 b5 98 01 00 00        	movq	%rsi, 0x198(%rbp)
1421bf5f9: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf600: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1421bf607: e8 c4 b2 00 00              	callq	0x1421ca8d0 <.text+0x21c98d0>
1421bf60c: 0f b6 05 f6 55 47 01        	movzbl	0x14755f6(%rip), %eax   # 0x143634c09
1421bf613: b9 48 00 00 00              	movl	$0x48, %ecx
1421bf618: ba 08 00 00 00              	movl	$0x8, %edx
1421bf61d: e8 ee 45 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf622: 48 85 c0                    	testq	%rax, %rax
1421bf625: 0f 84 cb 1a 00 00           	je	0x1421c10f6 <.text+0x21c00f6>
1421bf62b: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421bf632: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421bf636: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421bf63d: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421bf644: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421bf64b: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421bf652: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421bf656: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421bf65a: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421bf65e: 0f 11 00                    	movups	%xmm0, (%rax)
1421bf661: 48 89 9d 70 01 00 00        	movq	%rbx, 0x170(%rbp)
1421bf668: 4c 89 a5 78 01 00 00        	movq	%r12, 0x178(%rbp)
1421bf66f: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1421bf676: 48 89 b5 88 01 00 00        	movq	%rsi, 0x188(%rbp)
1421bf67d: c6 85 8c 02 00 00 00        	movb	$0x0, 0x28c(%rbp)
1421bf684: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf68b: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1421bf692: e8 29 fd 89 ff              	callq	0x141a5f3c0 <.text+0x1a5e3c0>
1421bf697: 0f b6 05 6b 55 47 01        	movzbl	0x147556b(%rip), %eax   # 0x143634c09
1421bf69e: b9 40 00 00 00              	movl	$0x40, %ecx
1421bf6a3: ba 08 00 00 00              	movl	$0x8, %edx
1421bf6a8: e8 63 45 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf6ad: 48 85 c0                    	testq	%rax, %rax
1421bf6b0: 0f 84 54 1a 00 00           	je	0x1421c110a <.text+0x21c010a>
1421bf6b6: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421bf6bd: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421bf6c4: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421bf6cb: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421bf6d2: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421bf6d6: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421bf6da: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421bf6de: 0f 11 00                    	movups	%xmm0, (%rax)
1421bf6e1: 48 89 bd 70 01 00 00        	movq	%rdi, 0x170(%rbp)
1421bf6e8: 4c 89 bd 78 01 00 00        	movq	%r15, 0x178(%rbp)
1421bf6ef: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1421bf6f6: 48 8d 05 c3 0a d0 00        	leaq	0xd00ac3(%rip), %rax    # 0x142ec01c0
1421bf6fd: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1421bf704: c6 85 90 02 00 00 00        	movb	$0x0, 0x290(%rbp)
1421bf70b: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf712: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
1421bf719: e8 12 b0 00 00              	callq	0x1421ca730 <.text+0x21c9730>
1421bf71e: 0f b6 05 e4 54 47 01        	movzbl	0x14754e4(%rip), %eax   # 0x143634c09
1421bf725: b9 48 00 00 00              	movl	$0x48, %ecx
1421bf72a: ba 08 00 00 00              	movl	$0x8, %edx
1421bf72f: e8 dc 44 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf734: 48 85 c0                    	testq	%rax, %rax
1421bf737: 0f 85 f3 09 00 00           	jne	0x1421c0130 <.text+0x21bf130>
1421bf73d: b9 08 00 00 00              	movl	$0x8, %ecx
1421bf742: ba 48 00 00 00              	movl	$0x48, %edx
1421bf747: e8 b4 98 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421bf74c: e9 12 1d 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421bf751: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1421bf75c: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1421bf767: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1421bf772: c6 85 7f 02 00 00 01        	movb	$0x1, 0x27f(%rbp)
1421bf779: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421bf782: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421bf789: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421bf78f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421bf795: 31 d2                       	xorl	%edx, %edx
1421bf797: e8 44 59 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421bf79c: 49 bf 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %r15 # imm = 0x3FA6666600000000
1421bf7a6: 4c 8d 25 1b 12 d0 00        	leaq	0xd0121b(%rip), %r12    # 0x142ec09c8
1421bf7ad: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421bf7b4: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
1421bf7bb: c7 04 33 e2 9c 85 20        	movl	$0x20859ce2, (%rbx,%rsi) # imm = 0x20859CE2
1421bf7c2: 48 83 c6 04                 	addq	$0x4, %rsi
1421bf7c6: 48 89 b5 80 01 00 00        	movq	%rsi, 0x180(%rbp)
1421bf7cd: c6 85 7f 02 00 00 01        	movb	$0x1, 0x27f(%rbp)
1421bf7d4: 48 8d 0d 15 ad e3 00        	leaq	0xe3ad15(%rip), %rcx    # 0x142ffa4f0
1421bf7db: ba 16 00 00 00              	movl	$0x16, %edx
1421bf7e0: e8 9b dc 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421bf7e5: 48 89 d7                    	movq	%rdx, %rdi
1421bf7e8: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1421bf7ef: 48 29 f1                    	subq	%rsi, %rcx
1421bf7f2: 48 39 d1                    	cmpq	%rdx, %rcx
1421bf7f5: 0f 82 d9 16 00 00           	jb	0x1421c0ed4 <.text+0x21bfed4>
1421bf7fb: 48 01 f3                    	addq	%rsi, %rbx
1421bf7fe: 48 89 d9                    	movq	%rbx, %rcx
1421bf801: 48 89 c2                    	movq	%rax, %rdx
1421bf804: 49 89 f8                    	movq	%rdi, %r8
1421bf807: e8 f4 43 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421bf80c: 48 8d 04 37                 	leaq	(%rdi,%rsi), %rax
1421bf810: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1421bf817: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421bf81e: 0f 10 85 70 01 00 00        	movups	0x170(%rbp), %xmm0
1421bf825: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1421bf82c: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf833: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bf83a: e8 51 c5 bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421bf83f: 48 8b 9d f0 01 00 00        	movq	0x1f0(%rbp), %rbx
1421bf846: 4c 8b ad f8 01 00 00        	movq	0x1f8(%rbp), %r13
1421bf84d: 48 8b bd 00 02 00 00        	movq	0x200(%rbp), %rdi
1421bf854: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1421bf85b: 48 85 d2                    	testq	%rdx, %rdx
1421bf85e: 74 12                       	je	0x1421bf872 <.text+0x21be872>
1421bf860: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1421bf867: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421bf86d: e8 ae 43 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421bf872: 0f b6 05 90 53 47 01        	movzbl	0x1475390(%rip), %eax   # 0x143634c09
1421bf879: b9 70 00 00 00              	movl	$0x70, %ecx
1421bf87e: ba 08 00 00 00              	movl	$0x8, %edx
1421bf883: e8 88 43 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf888: 48 85 c0                    	testq	%rax, %rax
1421bf88b: 0f 84 f7 18 00 00           	je	0x1421c1188 <.text+0x21c0188>
1421bf891: 48 89 c6                    	movq	%rax, %rsi
1421bf894: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421bf89b: 4c 89 60 08                 	movq	%r12, 0x8(%rax)
1421bf89f: 48 c7 40 10 09 00 00 00     	movq	$0x9, 0x10(%rax)
1421bf8a7: 66 c7 40 18 00 03           	movw	$0x300, 0x18(%rax)      # imm = 0x300
1421bf8ad: c6 40 1a 04                 	movb	$0x4, 0x1a(%rax)
1421bf8b1: 48 89 58 20                 	movq	%rbx, 0x20(%rax)
1421bf8b5: 4c 89 68 28                 	movq	%r13, 0x28(%rax)
1421bf8b9: 48 89 78 30                 	movq	%rdi, 0x30(%rax)
1421bf8bd: c7 40 38 00 00 00 00        	movl	$0x0, 0x38(%rax)
1421bf8c4: 4c 89 78 40                 	movq	%r15, 0x40(%rax)
1421bf8c8: 66 c7 40 48 02 00           	movw	$0x2, 0x48(%rax)
1421bf8ce: 66 c7 40 50 02 00           	movw	$0x2, 0x50(%rax)
1421bf8d4: c7 40 58 01 00 00 00        	movl	$0x1, 0x58(%rax)
1421bf8db: 0f 28 05 3e 03 d0 00        	movaps	0xd0033e(%rip), %xmm0   # 0x142ebfc20
1421bf8e2: 0f 11 40 5c                 	movups	%xmm0, 0x5c(%rax)
1421bf8e6: 66 c7 40 6c 01 00           	movw	$0x1, 0x6c(%rax)
1421bf8ec: c6 40 6e 00                 	movb	$0x0, 0x6e(%rax)
1421bf8f0: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1421bf8f7: 48 8d 3d 62 0e d0 00        	leaq	0xd00e62(%rip), %rdi    # 0x142ec0760
1421bf8fe: 48 89 bd 78 01 00 00        	movq	%rdi, 0x178(%rbp)
1421bf905: 0f b6 05 fd 52 47 01        	movzbl	0x14752fd(%rip), %eax   # 0x143634c09
1421bf90c: b9 10 00 00 00              	movl	$0x10, %ecx
1421bf911: ba 04 00 00 00              	movl	$0x4, %edx
1421bf916: e8 f5 42 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf91b: 48 85 c0                    	testq	%rax, %rax
1421bf91e: 0f 94 85 70 02 00 00        	sete	0x270(%rbp)
1421bf925: 0f 84 7f 18 00 00           	je	0x1421c11aa <.text+0x21c01aa>
1421bf92b: 48 b9 03 00 00 00 00 00 c8 41       	movabsq	$0x41c8000000000003, %rcx # imm = 0x41C8000000000003
1421bf935: 48 89 08                    	movq	%rcx, (%rax)
1421bf938: 48 c7 40 08 03 00 00 00     	movq	$0x3, 0x8(%rax)
1421bf940: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1421bf947: 48 8d 05 da 09 d0 00        	leaq	0xd009da(%rip), %rax    # 0x142ec0328
1421bf94e: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1421bf955: 4c 89 b5 10 01 00 00        	movq	%r14, 0x110(%rbp)
1421bf95c: 48 8d 05 0d 0d d0 00        	leaq	0xd00d0d(%rip), %rax    # 0x142ec0670
1421bf963: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421bf96a: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf971: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bf978: e8 43 fa 89 ff              	callq	0x141a5f3c0 <.text+0x1a5e3c0>
1421bf97d: 0f b6 05 85 52 47 01        	movzbl	0x1475285(%rip), %eax   # 0x143634c09
1421bf984: b9 40 00 00 00              	movl	$0x40, %ecx
1421bf989: ba 08 00 00 00              	movl	$0x8, %edx
1421bf98e: e8 7d 42 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bf993: 48 85 c0                    	testq	%rax, %rax
1421bf996: 0f 84 57 18 00 00           	je	0x1421c11f3 <.text+0x21c01f3>
1421bf99c: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421bf9a3: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421bf9aa: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421bf9b1: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421bf9b8: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421bf9bc: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421bf9c0: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421bf9c4: 0f 11 00                    	movups	%xmm0, (%rax)
1421bf9c7: 48 89 b5 00 01 00 00        	movq	%rsi, 0x100(%rbp)
1421bf9ce: 48 89 bd 08 01 00 00        	movq	%rdi, 0x108(%rbp)
1421bf9d5: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421bf9dc: 48 8d 05 dd 07 d0 00        	leaq	0xd007dd(%rip), %rax    # 0x142ec01c0
1421bf9e3: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421bf9ea: c6 85 97 02 00 00 00        	movb	$0x0, 0x297(%rbp)
1421bf9f1: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bf9f8: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bf9ff: e8 2c ad 00 00              	callq	0x1421ca730 <.text+0x21c9730>
1421bfa04: 0f b6 05 fe 51 47 01        	movzbl	0x14751fe(%rip), %eax   # 0x143634c09
1421bfa0b: b9 48 00 00 00              	movl	$0x48, %ecx
1421bfa10: ba 08 00 00 00              	movl	$0x8, %edx
1421bfa15: e8 f6 41 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bfa1a: 48 85 c0                    	testq	%rax, %rax
1421bfa1d: 0f 84 f8 17 00 00           	je	0x1421c121b <.text+0x21c021b>
1421bfa23: 48 89 c7                    	movq	%rax, %rdi
1421bfa26: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1421bfa2d: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
1421bfa31: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421bfa38: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421bfa3f: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421bfa46: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421bfa4d: 0f 11 5f 30                 	movups	%xmm3, 0x30(%rdi)
1421bfa51: 0f 11 57 20                 	movups	%xmm2, 0x20(%rdi)
1421bfa55: 0f 11 4f 10                 	movups	%xmm1, 0x10(%rdi)
1421bfa59: 0f 11 07                    	movups	%xmm0, (%rdi)
1421bfa5c: 48 8d 35 a5 0a d0 00        	leaq	0xd00aa5(%rip), %rsi    # 0x142ec0508
1421bfa63: e9 c9 13 00 00              	jmp	0x1421c0e31 <.text+0x21bfe31>
1421bfa68: 4c 8b 45 20                 	movq	0x20(%rbp), %r8
1421bfa6c: 4c 8b 4d 28                 	movq	0x28(%rbp), %r9
1421bfa70: 8b 57 58                    	movl	0x58(%rdi), %edx
1421bfa73: 48 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rsi # imm = 0x8000000000000000
1421bfa7d: 48 89 b5 f8 00 00 00        	movq	%rsi, 0xf8(%rbp)
1421bfa84: c6 85 94 02 00 00 01        	movb	$0x1, 0x294(%rbp)
1421bfa8b: c6 85 93 02 00 00 01        	movb	$0x1, 0x293(%rbp)
1421bfa92: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bfa99: e8 32 3c 00 00              	callq	0x1421c36d0 <.text+0x21c26d0>
1421bfa9e: 49 bf 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %r15 # imm = 0x3FA6666600000000
1421bfaa8: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1421bfaaf: 48 89 c1                    	movq	%rax, %rcx
1421bfab2: 48 f7 d9                    	negq	%rcx
1421bfab5: 0f 80 85 16 00 00           	jo	0x1421c1140 <.text+0x21c0140>
1421bfabb: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
1421bfac2: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1421bfac9: 48 c1 e2 05                 	shlq	$0x5, %rdx
1421bfacd: 48 01 ca                    	addq	%rcx, %rdx
1421bfad0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1421bfad3: 0f 11 85 be 00 00 00        	movups	%xmm0, 0xbe(%rbp)
1421bfada: c7 85 ce 00 00 00 00 00 00 00       	movl	$0x0, 0xce(%rbp)
1421bfae4: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
1421bfaef: 48 c7 85 f8 01 00 00 08 00 00 00    	movq	$0x8, 0x1f8(%rbp)
1421bfafa: 48 c7 85 00 02 00 00 00 00 00 00    	movq	$0x0, 0x200(%rbp)
1421bfb05: 66 c7 85 08 02 00 00 02 00  	movw	$0x2, 0x208(%rbp)
1421bfb0e: 66 c7 85 10 02 00 00 02 00  	movw	$0x2, 0x210(%rbp)
1421bfb17: 0f 10 85 b8 00 00 00        	movups	0xb8(%rbp), %xmm0
1421bfb1e: 0f 11 85 12 02 00 00        	movups	%xmm0, 0x212(%rbp)
1421bfb25: 4c 8b 85 c2 00 00 00        	movq	0xc2(%rbp), %r8
1421bfb2c: 4c 89 85 1c 02 00 00        	movq	%r8, 0x21c(%rbp)
1421bfb33: 44 8b 85 ca 00 00 00        	movl	0xca(%rbp), %r8d
1421bfb3a: 44 89 85 24 02 00 00        	movl	%r8d, 0x224(%rbp)
1421bfb41: 44 8b 85 ce 00 00 00        	movl	0xce(%rbp), %r8d
1421bfb48: 44 89 85 28 02 00 00        	movl	%r8d, 0x228(%rbp)
1421bfb4f: c7 85 2c 02 00 00 00 00 80 7f       	movl	$0x7f800000, 0x22c(%rbp) # imm = 0x7F800000
1421bfb59: 66 c7 85 30 02 00 00 00 00  	movw	$0x0, 0x230(%rbp)
1421bfb62: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
1421bfb69: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1421bfb70: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421bfb77: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
1421bfb7e: 48 89 b5 f8 00 00 00        	movq	%rsi, 0xf8(%rbp)
1421bfb85: c6 85 94 02 00 00 01        	movb	$0x1, 0x294(%rbp)
1421bfb8c: c6 85 93 02 00 00 01        	movb	$0x1, 0x293(%rbp)
1421bfb93: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421bfb9a: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bfba1: 4c 8d 85 f0 01 00 00        	leaq	0x1f0(%rbp), %r8
1421bfba8: e8 f3 89 00 00              	callq	0x1421c85a0 <.text+0x21c75a0>
1421bfbad: 0f b6 05 55 50 47 01        	movzbl	0x1475055(%rip), %eax   # 0x143634c09
1421bfbb4: b9 1c 00 00 00              	movl	$0x1c, %ecx
1421bfbb9: ba 01 00 00 00              	movl	$0x1, %edx
1421bfbbe: e8 4d 40 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bfbc3: 48 85 c0                    	testq	%rax, %rax
1421bfbc6: 0f 84 9a 15 00 00           	je	0x1421c1166 <.text+0x21c0166>
1421bfbcc: 48 89 c7                    	movq	%rax, %rdi
1421bfbcf: 0f 10 05 46 a8 e3 00        	movups	0xe3a846(%rip), %xmm0   # 0x142ffa41c
1421bfbd6: 0f 11 40 0c                 	movups	%xmm0, 0xc(%rax)
1421bfbda: 0f 10 05 2f a8 e3 00        	movups	0xe3a82f(%rip), %xmm0   # 0x142ffa410
1421bfbe1: 0f 11 00                    	movups	%xmm0, (%rax)
1421bfbe4: 0f b6 05 1e 50 47 01        	movzbl	0x147501e(%rip), %eax   # 0x143634c09
1421bfbeb: b9 70 00 00 00              	movl	$0x70, %ecx
1421bfbf0: ba 08 00 00 00              	movl	$0x8, %edx
1421bfbf5: e8 16 40 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bfbfa: 48 85 c0                    	testq	%rax, %rax
1421bfbfd: 0f 84 bb 15 00 00           	je	0x1421c11be <.text+0x21c01be>
1421bfc03: 48 89 c6                    	movq	%rax, %rsi
1421bfc06: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421bfc0d: 48 8d 05 b4 0d d0 00        	leaq	0xd00db4(%rip), %rax    # 0x142ec09c8
1421bfc14: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1421bfc18: 48 c7 46 10 09 00 00 00     	movq	$0x9, 0x10(%rsi)
1421bfc20: 66 c7 46 18 00 03           	movw	$0x300, 0x18(%rsi)      # imm = 0x300
1421bfc26: c6 46 1a 04                 	movb	$0x4, 0x1a(%rsi)
1421bfc2a: 48 c7 46 20 1c 00 00 00     	movq	$0x1c, 0x20(%rsi)
1421bfc32: 48 89 7e 28                 	movq	%rdi, 0x28(%rsi)
1421bfc36: 48 c7 46 30 1c 00 00 00     	movq	$0x1c, 0x30(%rsi)
1421bfc3e: c7 46 38 00 00 00 00        	movl	$0x0, 0x38(%rsi)
1421bfc45: 4c 89 7e 40                 	movq	%r15, 0x40(%rsi)
1421bfc49: 66 c7 46 48 02 00           	movw	$0x2, 0x48(%rsi)
1421bfc4f: 66 c7 46 50 02 00           	movw	$0x2, 0x50(%rsi)
1421bfc55: c7 46 58 00 00 00 00        	movl	$0x0, 0x58(%rsi)
1421bfc5c: 66 c7 46 6c 01 00           	movw	$0x1, 0x6c(%rsi)
1421bfc62: c6 46 6e 00                 	movb	$0x0, 0x6e(%rsi)
1421bfc66: 48 89 b5 b8 00 00 00        	movq	%rsi, 0xb8(%rbp)
1421bfc6d: 48 8d 05 ec 0a d0 00        	leaq	0xd00aec(%rip), %rax    # 0x142ec0760
1421bfc74: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1421bfc7b: 48 8b 7d 28                 	movq	0x28(%rbp), %rdi
1421bfc7f: 48 85 ff                    	testq	%rdi, %rdi
1421bfc82: 0f 88 51 15 00 00           	js	0x1421c11d9 <.text+0x21c01d9>
1421bfc88: 4c 89 b5 68 02 00 00        	movq	%r14, 0x268(%rbp)
1421bfc8f: 48 8b 5d 20                 	movq	0x20(%rbp), %rbx
1421bfc93: 0f 84 34 05 00 00           	je	0x1421c01cd <.text+0x21bf1cd>
1421bfc99: 0f b6 05 69 4f 47 01        	movzbl	0x1474f69(%rip), %eax   # 0x143634c09
1421bfca0: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1421bfca6: ba 01 00 00 00              	movl	$0x1, %edx
1421bfcab: 48 89 f9                    	movq	%rdi, %rcx
1421bfcae: e8 5d 3f 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bfcb3: 48 85 c0                    	testq	%rax, %rax
1421bfcb6: 0f 84 20 15 00 00           	je	0x1421c11dc <.text+0x21c01dc>
1421bfcbc: 49 89 c6                    	movq	%rax, %r14
1421bfcbf: 49 bf 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %r15 # imm = 0x3FA6666600000000
1421bfcc9: e9 05 05 00 00              	jmp	0x1421c01d3 <.text+0x21bf1d3>
1421bfcce: 48 89 9d 60 02 00 00        	movq	%rbx, 0x260(%rbp)
1421bfcd5: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1421bfce0: 48 c7 85 78 01 00 00 01 00 00 00    	movq	$0x1, 0x178(%rbp)
1421bfceb: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
1421bfcf6: c6 85 81 02 00 00 01        	movb	$0x1, 0x281(%rbp)
1421bfcfd: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421bfd06: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421bfd0d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421bfd13: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421bfd19: 31 d2                       	xorl	%edx, %edx
1421bfd1b: e8 c0 53 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421bfd20: 49 bf 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %r15 # imm = 0x3FA6666600000000
1421bfd2a: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421bfd31: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
1421bfd38: c7 04 33 e2 9d 8c 20        	movl	$0x208c9de2, (%rbx,%rsi) # imm = 0x208C9DE2
1421bfd3f: 48 83 c6 04                 	addq	$0x4, %rsi
1421bfd43: 48 89 b5 80 01 00 00        	movq	%rsi, 0x180(%rbp)
1421bfd4a: c6 85 81 02 00 00 01        	movb	$0x1, 0x281(%rbp)
1421bfd51: 48 8d 0d 47 a7 e3 00        	leaq	0xe3a747(%rip), %rcx    # 0x142ffa49f
1421bfd58: ba 15 00 00 00              	movl	$0x15, %edx
1421bfd5d: e8 1e d7 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421bfd62: 48 89 d7                    	movq	%rdx, %rdi
1421bfd65: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
1421bfd6c: 48 29 f1                    	subq	%rsi, %rcx
1421bfd6f: 48 39 d1                    	cmpq	%rdx, %rcx
1421bfd72: 0f 82 de 11 00 00           	jb	0x1421c0f56 <.text+0x21bff56>
1421bfd78: 48 01 f3                    	addq	%rsi, %rbx
1421bfd7b: 48 89 d9                    	movq	%rbx, %rcx
1421bfd7e: 48 89 c2                    	movq	%rax, %rdx
1421bfd81: 49 89 f8                    	movq	%rdi, %r8
1421bfd84: e8 77 3e 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421bfd89: 48 8d 04 37                 	leaq	(%rdi,%rsi), %rax
1421bfd8d: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1421bfd94: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421bfd9b: 0f 10 85 70 01 00 00        	movups	0x170(%rbp), %xmm0
1421bfda2: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1421bfda9: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bfdb0: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bfdb7: e8 d4 bf bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421bfdbc: 48 8b 9d f0 01 00 00        	movq	0x1f0(%rbp), %rbx
1421bfdc3: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1421bfdca: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1421bfdd1: 48 8b b5 00 02 00 00        	movq	0x200(%rbp), %rsi
1421bfdd8: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1421bfddf: 48 85 d2                    	testq	%rdx, %rdx
1421bfde2: 74 12                       	je	0x1421bfdf6 <.text+0x21bedf6>
1421bfde4: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1421bfdeb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421bfdf1: e8 2a 3e 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421bfdf6: 0f b6 05 0c 4e 47 01        	movzbl	0x1474e0c(%rip), %eax   # 0x143634c09
1421bfdfd: b9 70 00 00 00              	movl	$0x70, %ecx
1421bfe02: ba 08 00 00 00              	movl	$0x8, %edx
1421bfe07: e8 04 3e 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bfe0c: 48 85 c0                    	testq	%rax, %rax
1421bfe0f: 0f 84 5c 15 00 00           	je	0x1421c1371 <.text+0x21c0371>
1421bfe15: 48 89 c7                    	movq	%rax, %rdi
1421bfe18: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421bfe1f: 4c 89 68 08                 	movq	%r13, 0x8(%rax)
1421bfe23: 48 c7 40 10 09 00 00 00     	movq	$0x9, 0x10(%rax)
1421bfe2b: 66 c7 40 18 00 03           	movw	$0x300, 0x18(%rax)      # imm = 0x300
1421bfe31: c6 40 1a 04                 	movb	$0x4, 0x1a(%rax)
1421bfe35: 48 89 58 20                 	movq	%rbx, 0x20(%rax)
1421bfe39: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
1421bfe40: 48 89 47 28                 	movq	%rax, 0x28(%rdi)
1421bfe44: 48 89 77 30                 	movq	%rsi, 0x30(%rdi)
1421bfe48: c7 47 38 00 00 00 00        	movl	$0x0, 0x38(%rdi)
1421bfe4f: 4c 89 7f 40                 	movq	%r15, 0x40(%rdi)
1421bfe53: 66 c7 47 48 02 00           	movw	$0x2, 0x48(%rdi)
1421bfe59: 66 c7 47 50 02 00           	movw	$0x2, 0x50(%rdi)
1421bfe5f: c7 47 58 01 00 00 00        	movl	$0x1, 0x58(%rdi)
1421bfe66: 0f 28 35 73 79 7a 00        	movaps	0x7a7973(%rip), %xmm6   # 0x1429677e0
1421bfe6d: 0f 11 77 5c                 	movups	%xmm6, 0x5c(%rdi)
1421bfe71: 66 c7 47 6c 01 00           	movw	$0x1, 0x6c(%rdi)
1421bfe77: c6 47 6e 00                 	movb	$0x0, 0x6e(%rdi)
1421bfe7b: 48 89 bd c0 01 00 00        	movq	%rdi, 0x1c0(%rbp)
1421bfe82: 4c 89 fe                    	movq	%r15, %rsi
1421bfe85: 4c 8d 3d d4 08 d0 00        	leaq	0xd008d4(%rip), %r15    # 0x142ec0760
1421bfe8c: 4c 89 bd c8 01 00 00        	movq	%r15, 0x1c8(%rbp)
1421bfe93: 0f b6 05 6f 4d 47 01        	movzbl	0x1474d6f(%rip), %eax   # 0x143634c09
1421bfe9a: b9 70 00 00 00              	movl	$0x70, %ecx
1421bfe9f: ba 08 00 00 00              	movl	$0x8, %edx
1421bfea4: e8 67 3d 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bfea9: 48 85 c0                    	testq	%rax, %rax
1421bfeac: 0f 84 da 14 00 00           	je	0x1421c138c <.text+0x21c038c>
1421bfeb2: 48 89 c3                    	movq	%rax, %rbx
1421bfeb5: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421bfebc: 4c 89 68 08                 	movq	%r13, 0x8(%rax)
1421bfec0: 48 c7 40 10 09 00 00 00     	movq	$0x9, 0x10(%rax)
1421bfec8: 66 c7 40 18 00 03           	movw	$0x300, 0x18(%rax)      # imm = 0x300
1421bfece: c6 40 1a 04                 	movb	$0x4, 0x1a(%rax)
1421bfed2: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1421bfed9: 48 89 43 20                 	movq	%rax, 0x20(%rbx)
1421bfedd: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1421bfee4: 48 89 43 28                 	movq	%rax, 0x28(%rbx)
1421bfee8: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
1421bfeec: c7 43 38 00 00 00 00        	movl	$0x0, 0x38(%rbx)
1421bfef3: 48 89 73 40                 	movq	%rsi, 0x40(%rbx)
1421bfef7: 66 c7 43 48 02 00           	movw	$0x2, 0x48(%rbx)
1421bfefd: 66 c7 43 50 02 00           	movw	$0x2, 0x50(%rbx)
1421bff03: c7 43 58 01 00 00 00        	movl	$0x1, 0x58(%rbx)
1421bff0a: 0f 11 73 5c                 	movups	%xmm6, 0x5c(%rbx)
1421bff0e: 66 c7 43 6c 01 00           	movw	$0x1, 0x6c(%rbx)
1421bff14: c6 43 6e 00                 	movb	$0x0, 0x6e(%rbx)
1421bff18: 48 89 9d b8 00 00 00        	movq	%rbx, 0xb8(%rbp)
1421bff1f: 4c 89 bd c0 00 00 00        	movq	%r15, 0xc0(%rbp)
1421bff26: 0f b6 05 dc 4c 47 01        	movzbl	0x1474cdc(%rip), %eax   # 0x143634c09
1421bff2d: b9 10 00 00 00              	movl	$0x10, %ecx
1421bff32: ba 04 00 00 00              	movl	$0x4, %edx
1421bff37: e8 d4 3c 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bff3c: 48 85 c0                    	testq	%rax, %rax
1421bff3f: 0f 84 76 14 00 00           	je	0x1421c13bb <.text+0x21c03bb>
1421bff45: 48 89 c6                    	movq	%rax, %rsi
1421bff48: 48 b8 03 00 00 00 00 00 c8 41       	movabsq	$0x41c8000000000003, %rax # imm = 0x41C8000000000003
1421bff52: 48 89 06                    	movq	%rax, (%rsi)
1421bff55: 48 c7 46 08 03 00 00 00     	movq	$0x3, 0x8(%rsi)
1421bff5d: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
1421bff64: 4c 8d 35 bd 03 d0 00        	leaq	0xd003bd(%rip), %r14    # 0x142ec0328
1421bff6b: 4c 89 b5 78 01 00 00        	movq	%r14, 0x178(%rbp)
1421bff72: 0f b6 05 90 4c 47 01        	movzbl	0x1474c90(%rip), %eax   # 0x143634c09
1421bff79: b9 10 00 00 00              	movl	$0x10, %ecx
1421bff7e: ba 04 00 00 00              	movl	$0x4, %edx
1421bff83: e8 88 3c 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bff88: 48 85 c0                    	testq	%rax, %rax
1421bff8b: 0f 94 85 e0 01 00 00        	sete	0x1e0(%rbp)
1421bff92: 0f 84 3e 14 00 00           	je	0x1421c13d6 <.text+0x21c03d6>
1421bff98: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
1421bff9f: 48 b9 03 00 00 00 00 00 a0 40       	movabsq	$0x40a0000000000003, %rcx # imm = 0x40A0000000000003
1421bffa9: 48 89 48 08                 	movq	%rcx, 0x8(%rax)
1421bffad: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1421bffb4: 4c 89 b5 08 01 00 00        	movq	%r14, 0x108(%rbp)
1421bffbb: 4c 89 a5 10 01 00 00        	movq	%r12, 0x110(%rbp)
1421bffc2: 48 8d 05 a7 06 d0 00        	leaq	0xd006a7(%rip), %rax    # 0x142ec0670
1421bffc9: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421bffd0: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421bffd7: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421bffde: e8 4d a7 00 00              	callq	0x1421ca730 <.text+0x21c9730>
1421bffe3: 0f b6 05 1f 4c 47 01        	movzbl	0x1474c1f(%rip), %eax   # 0x143634c09
1421bffea: b9 48 00 00 00              	movl	$0x48, %ecx
1421bffef: ba 08 00 00 00              	movl	$0x8, %edx
1421bfff4: e8 17 3c 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421bfff9: 48 85 c0                    	testq	%rax, %rax
1421bfffc: 0f 84 0e 14 00 00           	je	0x1421c1410 <.text+0x21c0410>
1421c0002: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421c0009: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421c000d: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c0014: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421c001b: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421c0022: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421c0029: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c002d: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c0031: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c0035: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0038: 48 89 b5 00 01 00 00        	movq	%rsi, 0x100(%rbp)
1421c003f: 4c 89 b5 08 01 00 00        	movq	%r14, 0x108(%rbp)
1421c0046: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421c004d: 48 8d 35 b4 04 d0 00        	leaq	0xd004b4(%rip), %rsi    # 0x142ec0508
1421c0054: 48 89 b5 18 01 00 00        	movq	%rsi, 0x118(%rbp)
1421c005b: c6 85 8a 02 00 00 00        	movb	$0x0, 0x28a(%rbp)
1421c0062: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c0069: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421c0070: e8 4b f3 89 ff              	callq	0x141a5f3c0 <.text+0x1a5e3c0>
1421c0075: 0f b6 05 8d 4b 47 01        	movzbl	0x1474b8d(%rip), %eax   # 0x143634c09
1421c007c: b9 40 00 00 00              	movl	$0x40, %ecx
1421c0081: ba 08 00 00 00              	movl	$0x8, %edx
1421c0086: e8 85 3b 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c008b: 48 85 c0                    	testq	%rax, %rax
1421c008e: 0f 84 9e 13 00 00           	je	0x1421c1432 <.text+0x21c0432>
1421c0094: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c009b: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421c00a2: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421c00a9: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421c00b0: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c00b4: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c00b8: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c00bc: 0f 11 00                    	movups	%xmm0, (%rax)
1421c00bf: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
1421c00c6: 4c 89 bd 08 01 00 00        	movq	%r15, 0x108(%rbp)
1421c00cd: 48 89 9d 10 01 00 00        	movq	%rbx, 0x110(%rbp)
1421c00d4: 4c 89 bd 18 01 00 00        	movq	%r15, 0x118(%rbp)
1421c00db: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1421c00e2: 48 8d 05 d7 00 d0 00        	leaq	0xd000d7(%rip), %rax    # 0x142ec01c0
1421c00e9: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
1421c00f0: c6 85 96 02 00 00 00        	movb	$0x0, 0x296(%rbp)
1421c00f7: c6 85 95 02 00 00 00        	movb	$0x0, 0x295(%rbp)
1421c00fe: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c0105: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421c010c: e8 bf a7 00 00              	callq	0x1421ca8d0 <.text+0x21c98d0>
1421c0111: 0f b6 05 f1 4a 47 01        	movzbl	0x1474af1(%rip), %eax   # 0x143634c09
1421c0118: b9 48 00 00 00              	movl	$0x48, %ecx
1421c011d: ba 08 00 00 00              	movl	$0x8, %edx
1421c0122: e8 e9 3a 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0127: 48 85 c0                    	testq	%rax, %rax
1421c012a: 0f 84 24 13 00 00           	je	0x1421c1454 <.text+0x21c0454>
1421c0130: 48 89 c7                    	movq	%rax, %rdi
1421c0133: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1421c013a: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
1421c013e: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c0145: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421c014c: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421c0153: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421c015a: 0f 11 5f 30                 	movups	%xmm3, 0x30(%rdi)
1421c015e: 0f 11 57 20                 	movups	%xmm2, 0x20(%rdi)
1421c0162: 0f 11 4f 10                 	movups	%xmm1, 0x10(%rdi)
1421c0166: 0f 11 07                    	movups	%xmm0, (%rdi)
1421c0169: e9 c3 0c 00 00              	jmp	0x1421c0e31 <.text+0x21bfe31>
1421c016e: 0f b6 05 94 4a 47 01        	movzbl	0x1474a94(%rip), %eax   # 0x143634c09
1421c0175: b9 10 00 00 00              	movl	$0x10, %ecx
1421c017a: ba 04 00 00 00              	movl	$0x4, %edx
1421c017f: e8 8c 3a 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0184: 48 85 c0                    	testq	%rax, %rax
1421c0187: 0f 84 bb 11 00 00           	je	0x1421c1348 <.text+0x21c0348>
1421c018d: 48 89 c7                    	movq	%rax, %rdi
1421c0190: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
1421c0197: 48 c7 40 08 03 00 00 00     	movq	$0x3, 0x8(%rax)
1421c019f: 48 8d 04 5d 00 00 00 00     	leaq	(,%rbx,2), %rax
1421c01a7: 48 85 c0                    	testq	%rax, %rax
1421c01aa: 74 15                       	je	0x1421c01c1 <.text+0x21bf1c1>
1421c01ac: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421c01b2: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1421c01b9: 48 89 da                    	movq	%rbx, %rdx
1421c01bc: e8 5f 3a 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421c01c1: 48 8d 35 60 01 d0 00        	leaq	0xd00160(%rip), %rsi    # 0x142ec0328
1421c01c8: e9 3e 0c 00 00              	jmp	0x1421c0e0b <.text+0x21bfe0b>
1421c01cd: 41 be 01 00 00 00           	movl	$0x1, %r14d
1421c01d3: 4c 89 f1                    	movq	%r14, %rcx
1421c01d6: 48 89 da                    	movq	%rbx, %rdx
1421c01d9: 49 89 f8                    	movq	%rdi, %r8
1421c01dc: e8 1f 3a 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421c01e1: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
1421c01ec: 48 8d 05 d5 07 d0 00        	leaq	0xd007d5(%rip), %rax    # 0x142ec09c8
1421c01f3: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1421c01fa: 48 c7 85 00 02 00 00 09 00 00 00    	movq	$0x9, 0x200(%rbp)
1421c0205: 66 c7 85 08 02 00 00 00 03  	movw	$0x300, 0x208(%rbp)     # imm = 0x300
1421c020e: c6 85 0a 02 00 00 04        	movb	$0x4, 0x20a(%rbp)
1421c0215: 48 89 bd 10 02 00 00        	movq	%rdi, 0x210(%rbp)
1421c021c: 4c 89 b5 18 02 00 00        	movq	%r14, 0x218(%rbp)
1421c0223: 48 89 bd 20 02 00 00        	movq	%rdi, 0x220(%rbp)
1421c022a: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
1421c0234: 4c 89 bd 30 02 00 00        	movq	%r15, 0x230(%rbp)
1421c023b: 66 c7 85 38 02 00 00 02 00  	movw	$0x2, 0x238(%rbp)
1421c0244: 66 c7 85 40 02 00 00 02 00  	movw	$0x2, 0x240(%rbp)
1421c024d: c7 85 48 02 00 00 00 00 00 00       	movl	$0x0, 0x248(%rbp)
1421c0257: 66 c7 85 5c 02 00 00 01 00  	movw	$0x1, 0x25c(%rbp)
1421c0260: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
1421c0267: 48 8d bd 00 01 00 00        	leaq	0x100(%rbp), %rdi
1421c026e: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421c0275: 48 89 f9                    	movq	%rdi, %rcx
1421c0278: e8 c3 cf 89 ff              	callq	0x141a5d240 <.text+0x1a5c240>
1421c027d: 0f 10 45 30                 	movups	0x30(%rbp), %xmm0
1421c0281: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
1421c028c: 0f 11 85 20 01 00 00        	movups	%xmm0, 0x120(%rbp)
1421c0293: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
1421c029a: 0f 10 8d 30 01 00 00        	movups	0x130(%rbp), %xmm1
1421c02a1: 0f 10 95 40 01 00 00        	movups	0x140(%rbp), %xmm2
1421c02a8: 0f 10 9d 50 01 00 00        	movups	0x150(%rbp), %xmm3
1421c02af: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
1421c02b6: 0f 29 8d 20 02 00 00        	movaps	%xmm1, 0x220(%rbp)
1421c02bd: 0f 29 95 30 02 00 00        	movaps	%xmm2, 0x230(%rbp)
1421c02c4: 0f 29 9d 40 02 00 00        	movaps	%xmm3, 0x240(%rbp)
1421c02cb: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1421c02d2: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1421c02d9: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
1421c02e0: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1421c02e7: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
1421c02ee: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1421c02f5: 0f 10 85 20 01 00 00        	movups	0x120(%rbp), %xmm0
1421c02fc: 0f 29 85 10 02 00 00        	movaps	%xmm0, 0x210(%rbp)
1421c0303: 83 bd f0 01 00 00 05        	cmpl	$0x5, 0x1f0(%rbp)
1421c030a: 72 4d                       	jb	0x1421c0359 <.text+0x21bf359>
1421c030c: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1421c0313: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1421c031a: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1421c0321: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1421c0328: 48 8b 00                    	movq	(%rax), %rax
1421c032b: 48 85 c0                    	testq	%rax, %rax
1421c032e: 74 09                       	je	0x1421c0339 <.text+0x21bf339>
1421c0330: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1421c0337: ff d0                       	callq	*%rax
1421c0339: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1421c0340: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1421c0347: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1421c034b: 48 85 d2                    	testq	%rdx, %rdx
1421c034e: 74 09                       	je	0x1421c0359 <.text+0x21bf359>
1421c0350: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1421c0354: e8 c7 38 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421c0359: 48 c7 85 f0 01 00 00 05 00 00 00    	movq	$0x5, 0x1f0(%rbp)
1421c0364: 48 c7 85 f8 01 00 00 01 00 00 00    	movq	$0x1, 0x1f8(%rbp)
1421c036f: 48 8d 05 ba a0 e3 00        	leaq	0xe3a0ba(%rip), %rax    # 0x142ffa430
1421c0376: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1421c037d: 0f 28 85 10 02 00 00        	movaps	0x210(%rbp), %xmm0
1421c0384: 0f 28 8d 20 02 00 00        	movaps	0x220(%rbp), %xmm1
1421c038b: 0f 28 95 30 02 00 00        	movaps	0x230(%rbp), %xmm2
1421c0392: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
1421c0399: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
1421c03a0: 0f 29 95 40 01 00 00        	movaps	%xmm2, 0x140(%rbp)
1421c03a7: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1421c03ae: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1421c03b5: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1421c03bc: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
1421c03c3: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1421c03ca: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421c03d1: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
1421c03d8: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421c03df: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1421c03e6: 0f 57 c0                    	xorps	%xmm0, %xmm0
1421c03e9: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1421c03f0: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1421c03f7: 0f b6 05 0b 48 47 01        	movzbl	0x147480b(%rip), %eax   # 0x143634c09
1421c03fe: b9 68 00 00 00              	movl	$0x68, %ecx
1421c0403: ba 08 00 00 00              	movl	$0x8, %edx
1421c0408: e8 03 38 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c040d: 48 85 c0                    	testq	%rax, %rax
1421c0410: 0f 84 f1 0d 00 00           	je	0x1421c1207 <.text+0x21c0207>
1421c0416: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1421c041d: 48 89 48 60                 	movq	%rcx, 0x60(%rax)
1421c0421: 0f 28 85 50 01 00 00        	movaps	0x150(%rbp), %xmm0
1421c0428: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1421c042c: 0f 28 85 40 01 00 00        	movaps	0x140(%rbp), %xmm0
1421c0433: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1421c0437: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1421c043e: 0f 28 8d 10 01 00 00        	movaps	0x110(%rbp), %xmm1
1421c0445: 0f 28 95 20 01 00 00        	movaps	0x120(%rbp), %xmm2
1421c044c: 0f 28 9d 30 01 00 00        	movaps	0x130(%rbp), %xmm3
1421c0453: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c0457: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c045b: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c045f: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0462: 48 89 b5 f0 01 00 00        	movq	%rsi, 0x1f0(%rbp)
1421c0469: 48 8d 0d f0 02 d0 00        	leaq	0xd002f0(%rip), %rcx    # 0x142ec0760
1421c0470: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
1421c0477: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
1421c047e: 48 8d 05 93 ff cf 00        	leaq	0xcfff93(%rip), %rax    # 0x142ec0418
1421c0485: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1421c048c: c6 85 8e 02 00 00 01        	movb	$0x1, 0x28e(%rbp)
1421c0493: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
1421c049a: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421c04a1: e8 1a ef 89 ff              	callq	0x141a5f3c0 <.text+0x1a5e3c0>
1421c04a6: c6 85 92 02 00 00 01        	movb	$0x1, 0x292(%rbp)
1421c04ad: c6 85 91 02 00 00 01        	movb	$0x1, 0x291(%rbp)
1421c04b4: 48 8d 0d 29 9e e3 00        	leaq	0xe39e29(%rip), %rcx    # 0x142ffa2e4
1421c04bb: ba 14 00 00 00              	movl	$0x14, %edx
1421c04c0: e8 bb cf 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421c04c5: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1421c04cc: 48 89 95 c8 01 00 00        	movq	%rdx, 0x1c8(%rbp)
1421c04d3: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1421c04da: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1421c04e1: 4c 89 a5 08 01 00 00        	movq	%r12, 0x108(%rbp)
1421c04e8: 4c 89 ad 10 01 00 00        	movq	%r13, 0x110(%rbp)
1421c04ef: 48 8d 05 8a f9 a3 fe        	leaq	-0x15c0676(%rip), %rax  # 0x140bffe80 <.text+0xbfee80>
1421c04f6: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
1421c04fd: 48 8d 05 6c 9f e3 00        	leaq	0xe39f6c(%rip), %rax    # 0x142ffa470
1421c0504: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1421c050b: 48 c7 85 f8 01 00 00 02 00 00 00    	movq	$0x2, 0x1f8(%rbp)
1421c0516: 48 89 bd 00 02 00 00        	movq	%rdi, 0x200(%rbp)
1421c051d: 48 c7 85 08 02 00 00 02 00 00 00    	movq	$0x2, 0x208(%rbp)
1421c0528: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
1421c0533: c6 85 92 02 00 00 01        	movb	$0x1, 0x292(%rbp)
1421c053a: c6 85 91 02 00 00 01        	movb	$0x1, 0x291(%rbp)
1421c0541: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1421c0545: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421c054c: e8 3f 9a bb fe              	callq	0x140d79f90 <.text+0xd78f90>
1421c0551: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
1421c0555: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1421c055c: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1421c0560: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1421c0567: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c056e: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1421c0575: e8 16 b8 bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421c057a: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
1421c0581: 48 89 85 15 01 00 00        	movq	%rax, 0x115(%rbp)
1421c0588: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c058f: 0f 11 85 05 01 00 00        	movups	%xmm0, 0x105(%rbp)
1421c0596: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
1421c059d: 48 85 d2                    	testq	%rdx, %rdx
1421c05a0: 48 8d 35 21 04 d0 00        	leaq	0xd00421(%rip), %rsi    # 0x142ec09c8
1421c05a7: 74 12                       	je	0x1421c05bb <.text+0x21bf5bb>
1421c05a9: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1421c05b0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421c05b6: e8 65 36 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421c05bb: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
1421c05c6: 48 89 b5 f8 01 00 00        	movq	%rsi, 0x1f8(%rbp)
1421c05cd: 48 c7 85 00 02 00 00 09 00 00 00    	movq	$0x9, 0x200(%rbp)
1421c05d8: 66 c7 85 08 02 00 00 00 03  	movw	$0x300, 0x208(%rbp)     # imm = 0x300
1421c05e1: c6 85 0a 02 00 00 04        	movb	$0x4, 0x20a(%rbp)
1421c05e8: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
1421c05ef: 48 8b 85 0d 01 00 00        	movq	0x10d(%rbp), %rax
1421c05f6: 48 8b 8d 15 01 00 00        	movq	0x115(%rbp), %rcx
1421c05fd: 0f 11 85 0b 02 00 00        	movups	%xmm0, 0x20b(%rbp)
1421c0604: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1421c060b: 48 89 8d 20 02 00 00        	movq	%rcx, 0x220(%rbp)
1421c0612: c7 85 28 02 00 00 00 00 00 00       	movl	$0x0, 0x228(%rbp)
1421c061c: 4c 89 bd 30 02 00 00        	movq	%r15, 0x230(%rbp)
1421c0623: 66 c7 85 38 02 00 00 02 00  	movw	$0x2, 0x238(%rbp)
1421c062c: 66 c7 85 40 02 00 00 02 00  	movw	$0x2, 0x240(%rbp)
1421c0635: c7 85 48 02 00 00 01 00 00 00       	movl	$0x1, 0x248(%rbp)
1421c063f: 0f 28 35 9a 71 7a 00        	movaps	0x7a719a(%rip), %xmm6   # 0x1429677e0
1421c0646: 0f 11 b5 4c 02 00 00        	movups	%xmm6, 0x24c(%rbp)
1421c064d: 66 c7 85 5c 02 00 00 01 00  	movw	$0x1, 0x25c(%rbp)
1421c0656: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
1421c065d: c6 85 92 02 00 00 01        	movb	$0x1, 0x292(%rbp)
1421c0664: c6 85 91 02 00 00 01        	movb	$0x1, 0x291(%rbp)
1421c066b: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1421c0672: 48 8d 95 f0 01 00 00        	leaq	0x1f0(%rbp), %rdx
1421c0679: e8 52 bc 00 00              	callq	0x1421cc2d0 <.text+0x21cb2d0>
1421c067e: c7 85 60 01 00 00 00 00 16 44       	movl	$0x44160000, 0x160(%rbp) # imm = 0x44160000
1421c0688: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
1421c068f: 0f 10 8d 10 01 00 00        	movups	0x110(%rbp), %xmm1
1421c0696: 0f 10 95 20 01 00 00        	movups	0x120(%rbp), %xmm2
1421c069d: 0f 10 9d 30 01 00 00        	movups	0x130(%rbp), %xmm3
1421c06a4: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
1421c06ab: 0f 29 8d 00 02 00 00        	movaps	%xmm1, 0x200(%rbp)
1421c06b2: 0f 29 95 10 02 00 00        	movaps	%xmm2, 0x210(%rbp)
1421c06b9: 0f 29 9d 20 02 00 00        	movaps	%xmm3, 0x220(%rbp)
1421c06c0: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1421c06c7: 0f 29 85 30 02 00 00        	movaps	%xmm0, 0x230(%rbp)
1421c06ce: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1421c06d5: 0f 29 85 40 02 00 00        	movaps	%xmm0, 0x240(%rbp)
1421c06dc: 8b 85 60 01 00 00           	movl	0x160(%rbp), %eax
1421c06e2: 89 85 50 02 00 00           	movl	%eax, 0x250(%rbp)
1421c06e8: 48 8b 85 64 01 00 00        	movq	0x164(%rbp), %rax
1421c06ef: 48 89 85 54 02 00 00        	movq	%rax, 0x254(%rbp)
1421c06f6: 8b 85 6c 01 00 00           	movl	0x16c(%rbp), %eax
1421c06fc: 89 85 5c 02 00 00           	movl	%eax, 0x25c(%rbp)
1421c0702: 0f b6 05 00 45 47 01        	movzbl	0x1474500(%rip), %eax   # 0x143634c09
1421c0709: b9 70 00 00 00              	movl	$0x70, %ecx
1421c070e: ba 08 00 00 00              	movl	$0x8, %edx
1421c0713: e8 f8 34 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0718: 48 85 c0                    	testq	%rax, %rax
1421c071b: 0f 84 0e 0b 00 00           	je	0x1421c122f <.text+0x21c022f>
1421c0721: 48 89 c7                    	movq	%rax, %rdi
1421c0724: 0f 28 85 50 02 00 00        	movaps	0x250(%rbp), %xmm0
1421c072b: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1421c072f: 0f 28 85 40 02 00 00        	movaps	0x240(%rbp), %xmm0
1421c0736: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1421c073a: 0f 28 85 30 02 00 00        	movaps	0x230(%rbp), %xmm0
1421c0741: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1421c0745: 0f 28 85 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm0
1421c074c: 0f 28 8d 00 02 00 00        	movaps	0x200(%rbp), %xmm1
1421c0753: 0f 28 95 10 02 00 00        	movaps	0x210(%rbp), %xmm2
1421c075a: 0f 28 9d 20 02 00 00        	movaps	0x220(%rbp), %xmm3
1421c0761: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c0765: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c0769: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c076d: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0770: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
1421c0774: 48 8d 05 f5 fe cf 00        	leaq	0xcffef5(%rip), %rax    # 0x142ec0670
1421c077b: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
1421c077f: 0f b6 05 83 44 47 01        	movzbl	0x1474483(%rip), %eax   # 0x143634c09
1421c0786: b9 30 00 00 00              	movl	$0x30, %ecx
1421c078b: ba 08 00 00 00              	movl	$0x8, %edx
1421c0790: e8 7b 34 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0795: 48 85 c0                    	testq	%rax, %rax
1421c0798: 41 0f 94 c6                 	sete	%r14b
1421c079c: 44 88 b5 e0 01 00 00        	movb	%r14b, 0x1e0(%rbp)
1421c07a3: 0f 84 9a 0a 00 00           	je	0x1421c1243 <.text+0x21c0243>
1421c07a9: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1421c07b0: 0f 10 85 b8 00 00 00        	movups	0xb8(%rbp), %xmm0
1421c07b7: 0f 10 8d c8 00 00 00        	movups	0xc8(%rbp), %xmm1
1421c07be: 0f 10 95 d8 00 00 00        	movups	0xd8(%rbp), %xmm2
1421c07c5: 0f 10 9d e8 00 00 00        	movups	0xe8(%rbp), %xmm3
1421c07cc: 0f 29 9d 20 02 00 00        	movaps	%xmm3, 0x220(%rbp)
1421c07d3: 0f 29 95 10 02 00 00        	movaps	%xmm2, 0x210(%rbp)
1421c07da: 0f 29 8d 00 02 00 00        	movaps	%xmm1, 0x200(%rbp)
1421c07e1: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
1421c07e8: 0f b6 05 1a 44 47 01        	movzbl	0x147441a(%rip), %eax   # 0x143634c09
1421c07ef: b9 40 00 00 00              	movl	$0x40, %ecx
1421c07f4: ba 08 00 00 00              	movl	$0x8, %edx
1421c07f9: e8 12 34 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c07fe: 48 85 c0                    	testq	%rax, %rax
1421c0801: 0f 84 5e 0a 00 00           	je	0x1421c1265 <.text+0x21c0265>
1421c0807: 48 89 c6                    	movq	%rax, %rsi
1421c080a: 0f 28 85 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm0
1421c0811: 0f 28 8d 00 02 00 00        	movaps	0x200(%rbp), %xmm1
1421c0818: 0f 28 95 10 02 00 00        	movaps	0x210(%rbp), %xmm2
1421c081f: 0f 28 9d 20 02 00 00        	movaps	0x220(%rbp), %xmm3
1421c0826: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c082a: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c082e: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c0832: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0835: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1421c083c: 4c 8d 3d 7d f9 cf 00        	leaq	0xcff97d(%rip), %r15    # 0x142ec01c0
1421c0843: 4c 89 bd c8 01 00 00        	movq	%r15, 0x1c8(%rbp)
1421c084a: 0f b6 05 b8 43 47 01        	movzbl	0x14743b8(%rip), %eax   # 0x143634c09
1421c0851: b9 10 00 00 00              	movl	$0x10, %ecx
1421c0856: ba 04 00 00 00              	movl	$0x4, %edx
1421c085b: e8 b0 33 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0860: 48 85 c0                    	testq	%rax, %rax
1421c0863: 0f 94 85 60 02 00 00        	sete	0x260(%rbp)
1421c086a: 0f 84 09 0a 00 00           	je	0x1421c1279 <.text+0x21c0279>
1421c0870: 48 89 c3                    	movq	%rax, %rbx
1421c0873: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
1421c087a: 49 bc 03 00 00 00 00 00 a0 40       	movabsq	$0x40a0000000000003, %r12 # imm = 0x40A0000000000003
1421c0884: 4c 89 60 08                 	movq	%r12, 0x8(%rax)
1421c0888: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1421c088f: 4c 8d 2d 92 fa cf 00        	leaq	0xcffa92(%rip), %r13    # 0x142ec0328
1421c0896: 4c 89 ad 08 01 00 00        	movq	%r13, 0x108(%rbp)
1421c089d: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1421c08a4: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1421c08ab: 0f 10 85 70 01 00 00        	movups	0x170(%rbp), %xmm0
1421c08b2: 0f 10 8d 80 01 00 00        	movups	0x180(%rbp), %xmm1
1421c08b9: 0f 10 95 90 01 00 00        	movups	0x190(%rbp), %xmm2
1421c08c0: 0f 10 9d a0 01 00 00        	movups	0x1a0(%rbp), %xmm3
1421c08c7: 0f 29 9d 20 02 00 00        	movaps	%xmm3, 0x220(%rbp)
1421c08ce: 0f 29 95 10 02 00 00        	movaps	%xmm2, 0x210(%rbp)
1421c08d5: 0f 29 8d 00 02 00 00        	movaps	%xmm1, 0x200(%rbp)
1421c08dc: 0f 29 85 f0 01 00 00        	movaps	%xmm0, 0x1f0(%rbp)
1421c08e3: 0f b6 05 1f 43 47 01        	movzbl	0x147431f(%rip), %eax   # 0x143634c09
1421c08ea: b9 48 00 00 00              	movl	$0x48, %ecx
1421c08ef: ba 08 00 00 00              	movl	$0x8, %edx
1421c08f4: e8 17 33 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c08f9: 48 85 c0                    	testq	%rax, %rax
1421c08fc: 0f 84 8b 09 00 00           	je	0x1421c128d <.text+0x21c028d>
1421c0902: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421c0909: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421c090d: 0f 28 85 f0 01 00 00        	movaps	0x1f0(%rbp), %xmm0
1421c0914: 0f 28 8d 00 02 00 00        	movaps	0x200(%rbp), %xmm1
1421c091b: 0f 28 95 10 02 00 00        	movaps	0x210(%rbp), %xmm2
1421c0922: 0f 28 9d 20 02 00 00        	movaps	0x220(%rbp), %xmm3
1421c0929: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c092d: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c0931: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c0935: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0938: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1421c093f: 48 89 31                    	movq	%rsi, (%rcx)
1421c0942: 4c 89 79 08                 	movq	%r15, 0x8(%rcx)
1421c0946: 48 89 59 10                 	movq	%rbx, 0x10(%rcx)
1421c094a: 4c 89 69 18                 	movq	%r13, 0x18(%rcx)
1421c094e: 48 89 41 20                 	movq	%rax, 0x20(%rcx)
1421c0952: 48 8d 35 af fb cf 00        	leaq	0xcffbaf(%rip), %rsi    # 0x142ec0508
1421c0959: 48 89 71 28                 	movq	%rsi, 0x28(%rcx)
1421c095d: 48 c7 45 68 03 00 00 00     	movq	$0x3, 0x68(%rbp)
1421c0965: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
1421c0969: 48 c7 45 78 03 00 00 00     	movq	$0x3, 0x78(%rbp)
1421c0971: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
1421c097c: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
1421c0987: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
1421c0992: c6 85 83 02 00 00 01        	movb	$0x1, 0x283(%rbp)
1421c0999: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c09a2: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1421c09a9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1421c09af: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c09b5: 31 d2                       	xorl	%edx, %edx
1421c09b7: e8 24 47 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c09bc: 4c 8b bd c8 01 00 00        	movq	0x1c8(%rbp), %r15
1421c09c3: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
1421c09ca: 41 c7 04 1f e2 9d 8c 20     	movl	$0x208c9de2, (%r15,%rbx) # imm = 0x208C9DE2
1421c09d2: 48 83 c3 04                 	addq	$0x4, %rbx
1421c09d6: 48 89 9d d0 01 00 00        	movq	%rbx, 0x1d0(%rbp)
1421c09dd: c6 85 83 02 00 00 01        	movb	$0x1, 0x283(%rbp)
1421c09e4: 48 8d 0d a5 9a e3 00        	leaq	0xe39aa5(%rip), %rcx    # 0x142ffa490
1421c09eb: ba 0f 00 00 00              	movl	$0xf, %edx
1421c09f0: e8 8b ca 9d fe              	callq	0x140b9d480 <.text+0xb9c480>
1421c09f5: 49 89 d6                    	movq	%rdx, %r14
1421c09f8: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
1421c09ff: 48 29 d9                    	subq	%rbx, %rcx
1421c0a02: 48 39 d1                    	cmpq	%rdx, %rcx
1421c0a05: 0f 82 0a 05 00 00           	jb	0x1421c0f15 <.text+0x21bff15>
1421c0a0b: 49 01 df                    	addq	%rbx, %r15
1421c0a0e: 4c 89 f9                    	movq	%r15, %rcx
1421c0a11: 48 89 c2                    	movq	%rax, %rdx
1421c0a14: 4d 89 f0                    	movq	%r14, %r8
1421c0a17: e8 e4 31 76 00              	callq	0x142923c00 <.text+0x2922c00>
1421c0a1c: 49 8d 04 1e                 	leaq	(%r14,%rbx), %rax
1421c0a20: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1421c0a27: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421c0a2e: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1421c0a35: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1421c0a3c: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c0a43: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421c0a4a: e8 41 b3 bb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
1421c0a4f: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1421c0a56: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
1421c0a5d: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1421c0a64: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1421c0a6b: 4c 8b b5 00 02 00 00        	movq	0x200(%rbp), %r14
1421c0a72: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
1421c0a79: 48 85 d2                    	testq	%rdx, %rdx
1421c0a7c: 49 bf 00 00 00 00 66 66 a6 3f       	movabsq	$0x3fa6666600000000, %r15 # imm = 0x3FA6666600000000
1421c0a86: 74 12                       	je	0x1421c0a9a <.text+0x21bfa9a>
1421c0a88: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1421c0a8f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1421c0a95: e8 86 31 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421c0a9a: 0f b6 05 68 41 47 01        	movzbl	0x1474168(%rip), %eax   # 0x143634c09
1421c0aa1: b9 70 00 00 00              	movl	$0x70, %ecx
1421c0aa6: ba 08 00 00 00              	movl	$0x8, %edx
1421c0aab: e8 60 31 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0ab0: 48 85 c0                    	testq	%rax, %rax
1421c0ab3: 0f 84 e8 07 00 00           	je	0x1421c12a1 <.text+0x21c02a1>
1421c0ab9: 48 89 c3                    	movq	%rax, %rbx
1421c0abc: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1421c0ac3: 48 8d 05 fe fe cf 00        	leaq	0xcffefe(%rip), %rax    # 0x142ec09c8
1421c0aca: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
1421c0ace: 48 c7 43 10 09 00 00 00     	movq	$0x9, 0x10(%rbx)
1421c0ad6: 66 c7 43 18 00 03           	movw	$0x300, 0x18(%rbx)      # imm = 0x300
1421c0adc: c6 43 1a 04                 	movb	$0x4, 0x1a(%rbx)
1421c0ae0: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1421c0ae7: 48 89 43 20                 	movq	%rax, 0x20(%rbx)
1421c0aeb: 48 8b 85 60 02 00 00        	movq	0x260(%rbp), %rax
1421c0af2: 48 89 43 28                 	movq	%rax, 0x28(%rbx)
1421c0af6: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
1421c0afa: c7 43 38 00 00 00 00        	movl	$0x0, 0x38(%rbx)
1421c0b01: 4c 89 7b 40                 	movq	%r15, 0x40(%rbx)
1421c0b05: 66 c7 43 48 02 00           	movw	$0x2, 0x48(%rbx)
1421c0b0b: 66 c7 43 50 02 00           	movw	$0x2, 0x50(%rbx)
1421c0b11: c7 43 58 01 00 00 00        	movl	$0x1, 0x58(%rbx)
1421c0b18: 0f 11 73 5c                 	movups	%xmm6, 0x5c(%rbx)
1421c0b1c: 66 c7 43 6c 01 00           	movw	$0x1, 0x6c(%rbx)
1421c0b22: c6 43 6e 00                 	movb	$0x0, 0x6e(%rbx)
1421c0b26: 48 89 5d e0                 	movq	%rbx, -0x20(%rbp)
1421c0b2a: 48 8d 05 2f fc cf 00        	leaq	0xcffc2f(%rip), %rax    # 0x142ec0760
1421c0b31: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1421c0b35: 0f b6 05 cd 40 47 01        	movzbl	0x14740cd(%rip), %eax   # 0x143634c09
1421c0b3c: b9 10 00 00 00              	movl	$0x10, %ecx
1421c0b41: ba 04 00 00 00              	movl	$0x4, %edx
1421c0b46: e8 c5 30 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0b4b: 48 85 c0                    	testq	%rax, %rax
1421c0b4e: 0f 84 61 07 00 00           	je	0x1421c12b5 <.text+0x21c02b5>
1421c0b54: 49 89 c6                    	movq	%rax, %r14
1421c0b57: 48 b8 03 00 00 00 00 00 c8 41       	movabsq	$0x41c8000000000003, %rax # imm = 0x41C8000000000003
1421c0b61: 49 89 06                    	movq	%rax, (%r14)
1421c0b64: 49 c7 46 08 03 00 00 00     	movq	$0x3, 0x8(%r14)
1421c0b6c: 4c 89 b5 90 00 00 00        	movq	%r14, 0x90(%rbp)
1421c0b73: 4c 89 ad 98 00 00 00        	movq	%r13, 0x98(%rbp)
1421c0b7a: 0f b6 05 88 40 47 01        	movzbl	0x1474088(%rip), %eax   # 0x143634c09
1421c0b81: b9 10 00 00 00              	movl	$0x10, %ecx
1421c0b86: ba 04 00 00 00              	movl	$0x4, %edx
1421c0b8b: e8 80 30 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0b90: 48 85 c0                    	testq	%rax, %rax
1421c0b93: 0f 94 85 70 02 00 00        	sete	0x270(%rbp)
1421c0b9a: 0f 84 30 07 00 00           	je	0x1421c12d0 <.text+0x21c02d0>
1421c0ba0: 49 89 c7                    	movq	%rax, %r15
1421c0ba3: 48 c7 00 03 00 00 00        	movq	$0x3, (%rax)
1421c0baa: 4c 89 60 08                 	movq	%r12, 0x8(%rax)
1421c0bae: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1421c0bb5: 4c 89 ad 88 00 00 00        	movq	%r13, 0x88(%rbp)
1421c0bbc: 48 89 bd c0 01 00 00        	movq	%rdi, 0x1c0(%rbp)
1421c0bc3: 48 8d 05 a6 fa cf 00        	leaq	0xcffaa6(%rip), %rax    # 0x142ec0670
1421c0bca: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1421c0bd1: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1421c0bd5: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421c0bdc: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
1421c0be0: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
1421c0be7: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c0bee: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421c0bf5: e8 46 c0 89 ff              	callq	0x141a5cc40 <.text+0x1a5bc40>
1421c0bfa: 0f b6 05 08 40 47 01        	movzbl	0x1474008(%rip), %eax   # 0x143634c09
1421c0c01: b9 48 00 00 00              	movl	$0x48, %ecx
1421c0c06: ba 08 00 00 00              	movl	$0x8, %edx
1421c0c0b: e8 00 30 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0c10: 48 85 c0                    	testq	%rax, %rax
1421c0c13: 0f 84 cb 06 00 00           	je	0x1421c12e4 <.text+0x21c02e4>
1421c0c19: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421c0c20: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421c0c24: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c0c2b: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421c0c32: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421c0c39: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421c0c40: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c0c44: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c0c48: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c0c4c: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0c4f: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
1421c0c56: 4c 89 ad 08 01 00 00        	movq	%r13, 0x108(%rbp)
1421c0c5d: 48 89 bd 10 01 00 00        	movq	%rdi, 0x110(%rbp)
1421c0c64: 48 8d 0d 05 fa cf 00        	leaq	0xcffa05(%rip), %rcx    # 0x142ec0670
1421c0c6b: 48 89 8d 18 01 00 00        	movq	%rcx, 0x118(%rbp)
1421c0c72: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1421c0c79: 48 89 b5 28 01 00 00        	movq	%rsi, 0x128(%rbp)
1421c0c80: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c0c87: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421c0c8e: e8 3d 9c 00 00              	callq	0x1421ca8d0 <.text+0x21c98d0>
1421c0c93: 0f b6 05 6f 3f 47 01        	movzbl	0x1473f6f(%rip), %eax   # 0x143634c09
1421c0c9a: b9 48 00 00 00              	movl	$0x48, %ecx
1421c0c9f: ba 08 00 00 00              	movl	$0x8, %edx
1421c0ca4: e8 67 2f 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0ca9: 48 85 c0                    	testq	%rax, %rax
1421c0cac: 48 8d 3d 0d f5 cf 00        	leaq	0xcff50d(%rip), %rdi    # 0x142ec01c0
1421c0cb3: 0f 84 3f 06 00 00           	je	0x1421c12f8 <.text+0x21c02f8>
1421c0cb9: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
1421c0cc0: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1421c0cc4: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c0ccb: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421c0cd2: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421c0cd9: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421c0ce0: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c0ce4: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c0ce8: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c0cec: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0cef: 4c 89 b5 00 01 00 00        	movq	%r14, 0x100(%rbp)
1421c0cf6: 4c 89 ad 08 01 00 00        	movq	%r13, 0x108(%rbp)
1421c0cfd: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421c0d04: 48 89 b5 18 01 00 00        	movq	%rsi, 0x118(%rbp)
1421c0d0b: c6 85 8b 02 00 00 00        	movb	$0x0, 0x28b(%rbp)
1421c0d12: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c0d19: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421c0d20: e8 9b e6 89 ff              	callq	0x141a5f3c0 <.text+0x1a5e3c0>
1421c0d25: 0f b6 05 dd 3e 47 01        	movzbl	0x1473edd(%rip), %eax   # 0x143634c09
1421c0d2c: b9 40 00 00 00              	movl	$0x40, %ecx
1421c0d31: ba 08 00 00 00              	movl	$0x8, %edx
1421c0d36: e8 d5 2e 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0d3b: 48 85 c0                    	testq	%rax, %rax
1421c0d3e: 4c 8b a5 68 02 00 00        	movq	0x268(%rbp), %r12
1421c0d45: 0f 84 c1 05 00 00           	je	0x1421c130c <.text+0x21c030c>
1421c0d4b: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c0d52: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421c0d59: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421c0d60: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421c0d67: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1421c0d6b: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1421c0d6f: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1421c0d73: 0f 11 00                    	movups	%xmm0, (%rax)
1421c0d76: 48 89 9d 00 01 00 00        	movq	%rbx, 0x100(%rbp)
1421c0d7d: 48 8d 0d dc f9 cf 00        	leaq	0xcff9dc(%rip), %rcx    # 0x142ec0760
1421c0d84: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
1421c0d8b: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1421c0d92: 48 89 bd 18 01 00 00        	movq	%rdi, 0x118(%rbp)
1421c0d99: c6 85 8f 02 00 00 00        	movb	$0x0, 0x28f(%rbp)
1421c0da0: 48 8d 8d f0 01 00 00        	leaq	0x1f0(%rbp), %rcx
1421c0da7: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1421c0dae: e8 7d 99 00 00              	callq	0x1421ca730 <.text+0x21c9730>
1421c0db3: 0f b6 05 4f 3e 47 01        	movzbl	0x1473e4f(%rip), %eax   # 0x143634c09
1421c0dba: b9 48 00 00 00              	movl	$0x48, %ecx
1421c0dbf: ba 08 00 00 00              	movl	$0x8, %edx
1421c0dc4: e8 47 2e 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c0dc9: 48 85 c0                    	testq	%rax, %rax
1421c0dcc: 0f 84 4e 05 00 00           	je	0x1421c1320 <.text+0x21c0320>
1421c0dd2: 48 89 c7                    	movq	%rax, %rdi
1421c0dd5: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1421c0ddc: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
1421c0de0: 0f 10 85 f0 01 00 00        	movups	0x1f0(%rbp), %xmm0
1421c0de7: 0f 10 8d 00 02 00 00        	movups	0x200(%rbp), %xmm1
1421c0dee: 0f 10 95 10 02 00 00        	movups	0x210(%rbp), %xmm2
1421c0df5: 0f 10 9d 20 02 00 00        	movups	0x220(%rbp), %xmm3
1421c0dfc: 0f 11 5f 30                 	movups	%xmm3, 0x30(%rdi)
1421c0e00: 0f 11 57 20                 	movups	%xmm2, 0x20(%rdi)
1421c0e04: 0f 11 4f 10                 	movups	%xmm1, 0x10(%rdi)
1421c0e08: 0f 11 07                    	movups	%xmm0, (%rdi)
1421c0e0b: 4c 89 a5 68 02 00 00        	movq	%r12, 0x268(%rbp)
1421c0e12: 4c 89 e1                    	movq	%r12, %rcx
1421c0e15: e8 a6 89 bf fe              	callq	0x140db97c0 <.text+0xdb87c0>
1421c0e1a: ba 70 00 00 00              	movl	$0x70, %edx
1421c0e1f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1421c0e25: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1421c0e2c: e8 ef 2d 45 fe              	callq	0x140613c20 <.text+0x612c20>
1421c0e31: 48 89 f8                    	movq	%rdi, %rax
1421c0e34: 48 89 f2                    	movq	%rsi, %rdx
1421c0e37: 0f 28 b5 a0 02 00 00        	movaps	0x2a0(%rbp), %xmm6
1421c0e3e: 48 81 c4 38 03 00 00        	addq	$0x338, %rsp            # imm = 0x338
1421c0e45: 5b                          	popq	%rbx
1421c0e46: 5f                          	popq	%rdi
1421c0e47: 5e                          	popq	%rsi
1421c0e48: 41 5c                       	popq	%r12
1421c0e4a: 41 5d                       	popq	%r13
1421c0e4c: 41 5e                       	popq	%r14
1421c0e4e: 41 5f                       	popq	%r15
1421c0e50: 5d                          	popq	%rbp
1421c0e51: c3                          	retq
1421c0e52: c6 85 82 02 00 00 01        	movb	$0x1, 0x282(%rbp)
1421c0e59: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c0e62: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421c0e69: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c0e6f: 48 89 f2                    	movq	%rsi, %rdx
1421c0e72: 49 89 f8                    	movq	%rdi, %r8
1421c0e75: 48 89 c6                    	movq	%rax, %rsi
1421c0e78: e8 63 42 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c0e7d: 48 89 f0                    	movq	%rsi, %rax
1421c0e80: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421c0e87: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
1421c0e8e: e9 53 d8 ff ff              	jmp	0x1421be6e6 <.text+0x21bd6e6>
1421c0e93: c6 85 86 02 00 00 01        	movb	$0x1, 0x286(%rbp)
1421c0e9a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c0ea3: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
1421c0eaa: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c0eb0: 48 89 fa                    	movq	%rdi, %rdx
1421c0eb3: 49 89 d8                    	movq	%rbx, %r8
1421c0eb6: 48 89 c7                    	movq	%rax, %rdi
1421c0eb9: e8 22 42 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c0ebe: 48 89 f8                    	movq	%rdi, %rax
1421c0ec1: 4c 8b b5 c0 00 00 00        	movq	0xc0(%rbp), %r14
1421c0ec8: 48 8b bd c8 00 00 00        	movq	0xc8(%rbp), %rdi
1421c0ecf: e9 ae e4 ff ff              	jmp	0x1421bf382 <.text+0x21be382>
1421c0ed4: c6 85 7f 02 00 00 01        	movb	$0x1, 0x27f(%rbp)
1421c0edb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c0ee4: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421c0eeb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c0ef1: 48 89 f2                    	movq	%rsi, %rdx
1421c0ef4: 49 89 f8                    	movq	%rdi, %r8
1421c0ef7: 48 89 c6                    	movq	%rax, %rsi
1421c0efa: e8 e1 41 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c0eff: 48 89 f0                    	movq	%rsi, %rax
1421c0f02: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421c0f09: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
1421c0f10: e9 e6 e8 ff ff              	jmp	0x1421bf7fb <.text+0x21be7fb>
1421c0f15: c6 85 83 02 00 00 01        	movb	$0x1, 0x283(%rbp)
1421c0f1c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c0f25: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1421c0f2c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c0f32: 48 89 da                    	movq	%rbx, %rdx
1421c0f35: 4d 89 f0                    	movq	%r14, %r8
1421c0f38: 48 89 c3                    	movq	%rax, %rbx
1421c0f3b: e8 a0 41 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c0f40: 48 89 d8                    	movq	%rbx, %rax
1421c0f43: 4c 8b bd c8 01 00 00        	movq	0x1c8(%rbp), %r15
1421c0f4a: 48 8b 9d d0 01 00 00        	movq	0x1d0(%rbp), %rbx
1421c0f51: e9 b5 fa ff ff              	jmp	0x1421c0a0b <.text+0x21bfa0b>
1421c0f56: c6 85 81 02 00 00 01        	movb	$0x1, 0x281(%rbp)
1421c0f5d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c0f66: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421c0f6d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c0f73: 48 89 f2                    	movq	%rsi, %rdx
1421c0f76: 49 89 f8                    	movq	%rdi, %r8
1421c0f79: 48 89 c6                    	movq	%rax, %rsi
1421c0f7c: e8 5f 41 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c0f81: 48 89 f0                    	movq	%rsi, %rax
1421c0f84: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421c0f8b: 48 8b b5 80 01 00 00        	movq	0x180(%rbp), %rsi
1421c0f92: e9 e1 ed ff ff              	jmp	0x1421bfd78 <.text+0x21bed78>
1421c0f97: c6 85 8d 02 00 00 01        	movb	$0x1, 0x28d(%rbp)
1421c0f9e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c0fa7: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421c0fae: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c0fb4: 4c 89 f2                    	movq	%r14, %rdx
1421c0fb7: 4d 89 f8                    	movq	%r15, %r8
1421c0fba: 48 89 c3                    	movq	%rax, %rbx
1421c0fbd: e8 1e 41 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c0fc2: 48 89 d8                    	movq	%rbx, %rax
1421c0fc5: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421c0fcc: 4c 8b b5 80 01 00 00        	movq	0x180(%rbp), %r14
1421c0fd3: e9 f3 d8 ff ff              	jmp	0x1421be8cb <.text+0x21bd8cb>
1421c0fd8: 4c 89 bd e0 01 00 00        	movq	%r15, 0x1e0(%rbp)
1421c0fdf: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1421c0fe8: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1421c0fef: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1421c0ff5: 4c 89 f2                    	movq	%r14, %rdx
1421c0ff8: 49 89 f0                    	movq	%rsi, %r8
1421c0ffb: e8 e0 40 6e 00              	callq	0x1428a50e0 <.text+0x28a40e0>
1421c1000: 48 8b 9d 78 01 00 00        	movq	0x178(%rbp), %rbx
1421c1007: 4c 8b b5 80 01 00 00        	movq	0x180(%rbp), %r14
1421c100e: 4c 8b bd e0 01 00 00        	movq	0x1e0(%rbp), %r15
1421c1015: e9 56 d9 ff ff              	jmp	0x1421be970 <.text+0x21bd970>
1421c101a: b9 08 00 00 00              	movl	$0x8, %ecx
1421c101f: ba 70 00 00 00              	movl	$0x70, %edx
1421c1024: e8 d7 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1029: e9 35 04 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c102e: c6 85 88 02 00 00 01        	movb	$0x1, 0x288(%rbp)
1421c1035: c6 85 87 02 00 00 01        	movb	$0x1, 0x287(%rbp)
1421c103c: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1041: ba 30 00 00 00              	movl	$0x30, %edx
1421c1046: e8 b5 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c104b: e9 13 04 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1050: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1055: ba 70 00 00 00              	movl	$0x70, %edx
1421c105a: e8 a1 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c105f: e9 ff 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1064: 88 9d 68 02 00 00           	movb	%bl, 0x268(%rbp)
1421c106a: b9 04 00 00 00              	movl	$0x4, %ecx
1421c106f: ba 10 00 00 00              	movl	$0x10, %edx
1421c1074: e8 87 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1079: e9 e5 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c107e: 88 9d 68 02 00 00           	movb	%bl, 0x268(%rbp)
1421c1084: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1089: ba 48 00 00 00              	movl	$0x48, %edx
1421c108e: e8 6d 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1093: e9 cb 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1098: 4c 89 b5 68 02 00 00        	movq	%r14, 0x268(%rbp)
1421c109f: b9 08 00 00 00              	movl	$0x8, %ecx
1421c10a4: ba 70 00 00 00              	movl	$0x70, %edx
1421c10a9: e8 52 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c10ae: e9 b0 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c10b3: c6 85 8c 02 00 00 01        	movb	$0x1, 0x28c(%rbp)
1421c10ba: b9 04 00 00 00              	movl	$0x4, %ecx
1421c10bf: ba 10 00 00 00              	movl	$0x10, %edx
1421c10c4: e8 37 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c10c9: e9 95 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c10ce: b9 04 00 00 00              	movl	$0x4, %ecx
1421c10d3: ba 10 00 00 00              	movl	$0x10, %edx
1421c10d8: e8 23 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c10dd: e9 81 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c10e2: b9 08 00 00 00              	movl	$0x8, %ecx
1421c10e7: ba 48 00 00 00              	movl	$0x48, %edx
1421c10ec: e8 0f 7f 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c10f1: e9 6d 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c10f6: b9 08 00 00 00              	movl	$0x8, %ecx
1421c10fb: ba 48 00 00 00              	movl	$0x48, %edx
1421c1100: e8 fb 7e 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1105: e9 59 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c110a: b9 08 00 00 00              	movl	$0x8, %ecx
1421c110f: ba 40 00 00 00              	movl	$0x40, %edx
1421c1114: e8 e7 7e 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1119: e9 45 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c111e: c6 85 97 02 00 00 01        	movb	$0x1, 0x297(%rbp)
1421c1125: 4c 8d 05 04 8f e3 00        	leaq	0xe38f04(%rip), %r8     # 0x142ffa030
1421c112c: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1131: ba 20 00 00 00              	movl	$0x20, %edx
1421c1136: e8 a8 7e 71 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1421c113b: e9 23 03 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1140: 48 89 b5 f8 00 00 00        	movq	%rsi, 0xf8(%rbp)
1421c1147: c6 85 94 02 00 00 01        	movb	$0x1, 0x294(%rbp)
1421c114e: c6 85 93 02 00 00 01        	movb	$0x1, 0x293(%rbp)
1421c1155: 48 8d 0d 9c 92 e3 00        	leaq	0xe3929c(%rip), %rcx    # 0x142ffa3f8
1421c115c: e8 af 81 71 00              	callq	0x1428d9310 <.text+0x28d8310>
1421c1161: e9 fd 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1166: c6 85 8e 02 00 00 01        	movb	$0x1, 0x28e(%rbp)
1421c116d: 4c 8d 05 1c f8 cf 00        	leaq	0xcff81c(%rip), %r8     # 0x142ec0990
1421c1174: b9 01 00 00 00              	movl	$0x1, %ecx
1421c1179: ba 1c 00 00 00              	movl	$0x1c, %edx
1421c117e: e8 60 7e 71 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1421c1183: e9 db 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1188: 4c 89 ad 70 02 00 00        	movq	%r13, 0x270(%rbp)
1421c118f: 48 89 9d 68 02 00 00        	movq	%rbx, 0x268(%rbp)
1421c1196: b9 08 00 00 00              	movl	$0x8, %ecx
1421c119b: ba 70 00 00 00              	movl	$0x70, %edx
1421c11a0: e8 5b 7e 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c11a5: e9 b9 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c11aa: b9 04 00 00 00              	movl	$0x4, %ecx
1421c11af: ba 10 00 00 00              	movl	$0x10, %edx
1421c11b4: e8 47 7e 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c11b9: e9 a5 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c11be: 48 89 bd 68 02 00 00        	movq	%rdi, 0x268(%rbp)
1421c11c5: b9 08 00 00 00              	movl	$0x8, %ecx
1421c11ca: ba 70 00 00 00              	movl	$0x70, %edx
1421c11cf: e8 2c 7e 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c11d4: e9 8a 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c11d9: 45 31 ff                    	xorl	%r15d, %r15d
1421c11dc: 4c 8d 05 ad f7 cf 00        	leaq	0xcff7ad(%rip), %r8     # 0x142ec0990
1421c11e3: 4c 89 f9                    	movq	%r15, %rcx
1421c11e6: 48 89 fa                    	movq	%rdi, %rdx
1421c11e9: e8 f5 7d 71 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1421c11ee: e9 70 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c11f3: b9 08 00 00 00              	movl	$0x8, %ecx
1421c11f8: ba 40 00 00 00              	movl	$0x40, %edx
1421c11fd: e8 fe 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1202: e9 5c 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1207: b9 08 00 00 00              	movl	$0x8, %ecx
1421c120c: ba 68 00 00 00              	movl	$0x68, %edx
1421c1211: e8 ea 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1216: e9 48 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c121b: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1220: ba 48 00 00 00              	movl	$0x48, %edx
1421c1225: e8 d6 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c122a: e9 34 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c122f: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1234: ba 70 00 00 00              	movl	$0x70, %edx
1421c1239: e8 c2 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c123e: e9 20 02 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1243: c6 85 85 02 00 00 01        	movb	$0x1, 0x285(%rbp)
1421c124a: c6 85 84 02 00 00 01        	movb	$0x1, 0x284(%rbp)
1421c1251: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1256: ba 30 00 00 00              	movl	$0x30, %edx
1421c125b: e8 a0 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1260: e9 fe 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1265: b9 08 00 00 00              	movl	$0x8, %ecx
1421c126a: ba 40 00 00 00              	movl	$0x40, %edx
1421c126f: e8 8c 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1274: e9 ea 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1279: b9 04 00 00 00              	movl	$0x4, %ecx
1421c127e: ba 10 00 00 00              	movl	$0x10, %edx
1421c1283: e8 78 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1288: e9 d6 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c128d: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1292: ba 48 00 00 00              	movl	$0x48, %edx
1421c1297: e8 64 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c129c: e9 c2 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c12a1: b9 08 00 00 00              	movl	$0x8, %ecx
1421c12a6: ba 70 00 00 00              	movl	$0x70, %edx
1421c12ab: e8 50 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c12b0: e9 ae 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c12b5: c6 85 8b 02 00 00 01        	movb	$0x1, 0x28b(%rbp)
1421c12bc: b9 04 00 00 00              	movl	$0x4, %ecx
1421c12c1: ba 10 00 00 00              	movl	$0x10, %edx
1421c12c6: e8 35 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c12cb: e9 93 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c12d0: b9 04 00 00 00              	movl	$0x4, %ecx
1421c12d5: ba 10 00 00 00              	movl	$0x10, %edx
1421c12da: e8 21 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c12df: e9 7f 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c12e4: b9 08 00 00 00              	movl	$0x8, %ecx
1421c12e9: ba 48 00 00 00              	movl	$0x48, %edx
1421c12ee: e8 0d 7d 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c12f3: e9 6b 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c12f8: b9 08 00 00 00              	movl	$0x8, %ecx
1421c12fd: ba 48 00 00 00              	movl	$0x48, %edx
1421c1302: e8 f9 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1307: e9 57 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c130c: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1311: ba 40 00 00 00              	movl	$0x40, %edx
1421c1316: e8 e5 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c131b: e9 43 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1320: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1325: ba 48 00 00 00              	movl	$0x48, %edx
1421c132a: e8 d1 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c132f: e9 2f 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1334: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1339: ba 70 00 00 00              	movl	$0x70, %edx
1421c133e: e8 bd 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1343: e9 1b 01 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1348: 48 89 9d 60 02 00 00        	movq	%rbx, 0x260(%rbp)
1421c134f: c6 85 96 02 00 00 01        	movb	$0x1, 0x296(%rbp)
1421c1356: c6 85 95 02 00 00 01        	movb	$0x1, 0x295(%rbp)
1421c135d: b9 04 00 00 00              	movl	$0x4, %ecx
1421c1362: ba 10 00 00 00              	movl	$0x10, %edx
1421c1367: e8 94 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c136c: e9 f2 00 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c1371: 48 89 9d 68 02 00 00        	movq	%rbx, 0x268(%rbp)
1421c1378: b9 08 00 00 00              	movl	$0x8, %ecx
1421c137d: ba 70 00 00 00              	movl	$0x70, %edx
1421c1382: e8 79 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1387: e9 d7 00 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c138c: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1391: ba 70 00 00 00              	movl	$0x70, %edx
1421c1396: e8 65 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c139b: e9 c3 00 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c13a0: 4c 89 bd e0 01 00 00        	movq	%r15, 0x1e0(%rbp)
1421c13a7: b9 08 00 00 00              	movl	$0x8, %ecx
1421c13ac: ba 70 00 00 00              	movl	$0x70, %edx
1421c13b1: e8 4a 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c13b6: e9 a8 00 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c13bb: c6 85 8a 02 00 00 01        	movb	$0x1, 0x28a(%rbp)
1421c13c2: b9 04 00 00 00              	movl	$0x4, %ecx
1421c13c7: ba 10 00 00 00              	movl	$0x10, %edx
1421c13cc: e8 2f 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c13d1: e9 8d 00 00 00              	jmp	0x1421c1463 <.text+0x21c0463>
1421c13d6: b9 04 00 00 00              	movl	$0x4, %ecx
1421c13db: ba 10 00 00 00              	movl	$0x10, %edx
1421c13e0: e8 1b 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c13e5: eb 7c                       	jmp	0x1421c1463 <.text+0x21c0463>
1421c13e7: c6 85 89 02 00 00 01        	movb	$0x1, 0x289(%rbp)
1421c13ee: b9 04 00 00 00              	movl	$0x4, %ecx
1421c13f3: ba 10 00 00 00              	movl	$0x10, %edx
1421c13f8: e8 03 7c 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c13fd: eb 64                       	jmp	0x1421c1463 <.text+0x21c0463>
1421c13ff: b9 04 00 00 00              	movl	$0x4, %ecx
1421c1404: ba 10 00 00 00              	movl	$0x10, %edx
1421c1409: e8 f2 7b 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c140e: eb 53                       	jmp	0x1421c1463 <.text+0x21c0463>
1421c1410: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1415: ba 48 00 00 00              	movl	$0x48, %edx
1421c141a: e8 e1 7b 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c141f: eb 42                       	jmp	0x1421c1463 <.text+0x21c0463>
1421c1421: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1426: ba 48 00 00 00              	movl	$0x48, %edx
1421c142b: e8 d0 7b 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1430: eb 31                       	jmp	0x1421c1463 <.text+0x21c0463>
1421c1432: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1437: ba 40 00 00 00              	movl	$0x40, %edx
1421c143c: e8 bf 7b 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1441: eb 20                       	jmp	0x1421c1463 <.text+0x21c0463>
1421c1443: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1448: ba 40 00 00 00              	movl	$0x40, %edx
1421c144d: e8 ae 7b 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1452: eb 0f                       	jmp	0x1421c1463 <.text+0x21c0463>
1421c1454: b9 08 00 00 00              	movl	$0x8, %ecx
1421c1459: ba 48 00 00 00              	movl	$0x48, %edx
1421c145e: e8 9d 7b 71 00              	callq	0x1428d9000 <.text+0x28d8000>
1421c1463: 0f 0b                       	ud2
