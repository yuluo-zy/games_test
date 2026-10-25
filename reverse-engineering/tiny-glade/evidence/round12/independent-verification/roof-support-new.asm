
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412c2650: 55                          	pushq	%rbp
1412c2651: 56                          	pushq	%rsi
1412c2652: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1412c2659: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1412c2661: 48 c7 45 50 fe ff ff ff     	movq	$-0x2, 0x50(%rbp)
1412c2669: 48 89 ce                    	movq	%rcx, %rsi
1412c266c: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
1412c2670: 45 31 c0                    	xorl	%r8d, %r8d
1412c2673: e8 18 7f 35 00              	callq	0x14161a590 <.text+0x1619590>
1412c2678: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
1412c2680: 0f b6 05 82 25 37 02        	movzbl	0x2372582(%rip), %eax   # 0x143634c09
1412c2687: b9 30 00 00 00              	movl	$0x30, %ecx
1412c268c: ba 08 00 00 00              	movl	$0x8, %edx
1412c2691: e8 7a 15 35 ff              	callq	0x140613c10 <.text+0x612c10>
1412c2696: 48 85 c0                    	testq	%rax, %rax
1412c2699: 0f 84 b0 00 00 00           	je	0x1412c274f <.text+0x12c174f>
1412c269f: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
1412c26a3: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
1412c26a7: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
1412c26ab: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1412c26af: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1412c26b3: 0f 11 00                    	movups	%xmm0, (%rax)
1412c26b6: 48 8d 0d 51 b3 7d 01        	leaq	0x17db351(%rip), %rcx   # 0x142a9da0e
1412c26bd: 48 89 4d 00                 	movq	%rcx, (%rbp)
1412c26c1: 48 c7 45 08 41 00 00 00     	movq	$0x41, 0x8(%rbp)
1412c26c9: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1412c26cd: 48 8d 05 6c a7 96 01        	leaq	0x196a76c(%rip), %rax   # 0x142c2ce40
1412c26d4: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1412c26d8: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
1412c26e0: c6 45 46 05                 	movb	$0x5, 0x46(%rbp)
1412c26e4: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1412c26e8: 48 c7 45 28 41 00 00 00     	movq	$0x41, 0x28(%rbp)
1412c26f0: 0f b6 05 12 25 37 02        	movzbl	0x2372512(%rip), %eax   # 0x143634c09
1412c26f7: b9 40 00 00 00              	movl	$0x40, %ecx
1412c26fc: ba 08 00 00 00              	movl	$0x8, %edx
1412c2701: e8 0a 15 35 ff              	callq	0x140613c10 <.text+0x612c10>
1412c2706: 48 85 c0                    	testq	%rax, %rax
1412c2709: 74 55                       	je	0x1412c2760 <.text+0x12c1760>
1412c270b: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1412c270f: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1412c2713: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
1412c2717: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
1412c271b: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1412c271f: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1412c2723: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1412c2727: 0f 11 00                    	movups	%xmm0, (%rax)
1412c272a: 48 ba ad fe ec f5 98 35 77 83       	movabsq	$-0x7c88ca670a130153, %rdx # imm = 0x83773598F5ECFEAD
1412c2734: 48 89 f1                    	movq	%rsi, %rcx
1412c2737: 49 89 c0                    	movq	%rax, %r8
1412c273a: e8 b1 74 6a ff              	callq	0x140969bf0 <.text+0x968bf0>
1412c273f: 48 83 f8 01                 	cmpq	$0x1, %rax
1412c2743: 74 2c                       	je	0x1412c2771 <.text+0x12c1771>
1412c2745: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1412c274c: 5e                          	popq	%rsi
1412c274d: 5d                          	popq	%rbp
1412c274e: c3                          	retq
1412c274f: b9 08 00 00 00              	movl	$0x8, %ecx
1412c2754: ba 30 00 00 00              	movl	$0x30, %edx
1412c2759: e8 a2 68 61 01              	callq	0x1428d9000 <.text+0x28d8000>
1412c275e: eb 0f                       	jmp	0x1412c276f <.text+0x12c176f>
1412c2760: b9 08 00 00 00              	movl	$0x8, %ecx
1412c2765: ba 40 00 00 00              	movl	$0x40, %edx
1412c276a: e8 91 68 61 01              	callq	0x1428d9000 <.text+0x28d8000>
1412c276f: 0f 0b                       	ud2
1412c2771: 48 89 d0                    	movq	%rdx, %rax
1412c2774: 48 83 c2 10                 	addq	$0x10, %rdx
1412c2778: 48 89 d6                    	movq	%rdx, %rsi
1412c277b: 48 83 78 18 41              	cmpq	$0x41, 0x18(%rax)
1412c2780: 75 7c                       	jne	0x1412c27fe <.text+0x12c17fe>
1412c2782: 48 8b 16                    	movq	(%rsi), %rdx
1412c2785: 48 8d 0d 82 b2 7d 01        	leaq	0x17db282(%rip), %rcx   # 0x142a9da0e
1412c278c: 41 b8 41 00 00 00           	movl	$0x41, %r8d
1412c2792: e8 59 14 66 01              	callq	0x142923bf0 <.text+0x2922bf0>
1412c2797: 85 c0                       	testl	%eax, %eax
1412c2799: 75 63                       	jne	0x1412c27fe <.text+0x12c17fe>
1412c279b: 48 8d 05 56 b3 96 01        	leaq	0x196b356(%rip), %rax   # 0x142c2daf8
1412c27a2: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1412c27a6: 48 8d 05 f3 10 27 ff        	leaq	-0xd8ef0d(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
1412c27ad: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1412c27b1: 48 89 e8                    	movq	%rbp, %rax
1412c27b4: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1412c27b8: 48 8d 05 d1 76 e4 fe        	leaq	-0x11b892f(%rip), %rax  # 0x140109e90 <.text+0x108e90>
1412c27bf: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1412c27c3: 48 8d 05 5e ae 96 01        	leaq	0x196ae5e(%rip), %rax   # 0x142c2d628
1412c27ca: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1412c27ce: 48 c7 45 18 03 00 00 00     	movq	$0x3, 0x18(%rbp)
1412c27d6: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
1412c27de: 48 8d 45 d0                 	leaq	-0x30(%rbp), %rax
1412c27e2: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
1412c27e6: 48 c7 45 28 02 00 00 00     	movq	$0x2, 0x28(%rbp)
1412c27ee: 48 8d 15 63 ae 96 01        	leaq	0x196ae63(%rip), %rdx   # 0x142c2d658
1412c27f5: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1412c27f9: e8 92 6b 61 01              	callq	0x1428d9390 <.text+0x28d8390>
1412c27fe: 48 8d 05 f3 b2 96 01        	leaq	0x196b2f3(%rip), %rax   # 0x142c2daf8
1412c2805: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1412c2809: 48 8d 05 90 10 27 ff        	leaq	-0xd8ef70(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
1412c2810: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1412c2814: 48 89 75 20                 	movq	%rsi, 0x20(%rbp)
1412c2818: 48 8d 05 71 76 e4 fe        	leaq	-0x11b898f(%rip), %rax  # 0x140109e90 <.text+0x108e90>
1412c281f: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
1412c2823: 48 89 e9                    	movq	%rbp, %rcx
1412c2826: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
1412c282a: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1412c282e: 48 8d 05 33 ad 96 01        	leaq	0x196ad33(%rip), %rax   # 0x142c2d568
1412c2835: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
1412c2839: 48 c7 45 d8 03 00 00 00     	movq	$0x3, -0x28(%rbp)
1412c2841: 48 c7 45 f0 00 00 00 00     	movq	$0x0, -0x10(%rbp)
1412c2849: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
1412c284d: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1412c2851: 48 c7 45 e8 03 00 00 00     	movq	$0x3, -0x18(%rbp)
1412c2859: 48 8d 15 70 ad 96 01        	leaq	0x196ad70(%rip), %rdx   # 0x142c2d5d0
1412c2860: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1412c2864: e8 27 6b 61 01              	callq	0x1428d9390 <.text+0x28d8390>
1412c2869: cc                          	int3
1412c286a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1412c2870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412c2875: 55                          	pushq	%rbp
1412c2876: 56                          	pushq	%rsi
1412c2877: 48 83 ec 28                 	subq	$0x28, %rsp
1412c287b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412c2882: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
1412c2886: e8 25 fc 91 ff              	callq	0x140be24b0 <.text+0xbe14b0>
1412c288b: 90                          	nop
1412c288c: 48 83 c4 28                 	addq	$0x28, %rsp
1412c2890: 5e                          	popq	%rsi
1412c2891: 5d                          	popq	%rbp
1412c2892: c3                          	retq
1412c2893: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1412c28a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1412c28a5: 55                          	pushq	%rbp
1412c28a6: 56                          	pushq	%rsi
1412c28a7: 48 83 ec 28                 	subq	$0x28, %rsp
1412c28ab: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1412c28b2: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1412c28b6: e8 f5 db df fe              	callq	0x1400c04b0 <.text+0xbf4b0>
1412c28bb: 90                          	nop
1412c28bc: 48 83 c4 28                 	addq	$0x28, %rsp
1412c28c0: 5e                          	popq	%rsi
1412c28c1: 5d                          	popq	%rbp
1412c28c2: c3                          	retq
