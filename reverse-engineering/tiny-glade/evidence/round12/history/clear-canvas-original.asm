
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400d25f0: 55                          	pushq	%rbp
1400d25f1: 41 57                       	pushq	%r15
1400d25f3: 41 56                       	pushq	%r14
1400d25f5: 56                          	pushq	%rsi
1400d25f6: 57                          	pushq	%rdi
1400d25f7: 53                          	pushq	%rbx
1400d25f8: 48 81 ec b8 01 00 00        	subq	$0x1b8, %rsp            # imm = 0x1B8
1400d25ff: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400d2607: 0f 29 b5 20 01 00 00        	movaps	%xmm6, 0x120(%rbp)
1400d260e: 48 c7 85 18 01 00 00 fe ff ff ff    	movq	$-0x2, 0x118(%rbp)
1400d2619: 49 89 d7                    	movq	%rdx, %r15
1400d261c: 48 89 ce                    	movq	%rcx, %rsi
1400d261f: 48 c7 45 a8 00 00 00 00     	movq	$0x0, -0x58(%rbp)
1400d2627: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
1400d262f: 48 c7 45 d0 08 00 00 00     	movq	$0x8, -0x30(%rbp)
1400d2637: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
1400d263f: 48 c7 45 e0 10 00 00 00     	movq	$0x10, -0x20(%rbp)
1400d2647: 0f 57 f6                    	xorps	%xmm6, %xmm6
1400d264a: 0f 11 75 e8                 	movups	%xmm6, -0x18(%rbp)
1400d264e: 48 c7 45 f8 10 00 00 00     	movq	$0x10, -0x8(%rbp)
1400d2656: 0f 11 75 00                 	movups	%xmm6, (%rbp)
1400d265a: 48 c7 45 10 10 00 00 00     	movq	$0x10, 0x10(%rbp)
1400d2662: 0f 11 75 18                 	movups	%xmm6, 0x18(%rbp)
1400d2666: 48 c7 45 28 10 00 00 00     	movq	$0x10, 0x28(%rbp)
1400d266e: 0f 11 75 30                 	movups	%xmm6, 0x30(%rbp)
1400d2672: 48 c7 45 40 10 00 00 00     	movq	$0x10, 0x40(%rbp)
1400d267a: 0f 11 75 48                 	movups	%xmm6, 0x48(%rbp)
1400d267e: c7 45 58 00 00 00 00        	movl	$0x0, 0x58(%rbp)
1400d2685: 48 bf 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdi # imm = 0x8000000000000000
1400d268f: 48 89 7d 60                 	movq	%rdi, 0x60(%rbp)
1400d2693: 48 8d 1d 6b 44 8d 02        	leaq	0x28d446b(%rip), %rbx   # 0x1429a6b05
1400d269a: 48 89 5d 68                 	movq	%rbx, 0x68(%rbp)
1400d269e: 48 c7 45 70 6b 00 00 00     	movq	$0x6b, 0x70(%rbp)
1400d26a6: 48 c7 45 78 10 00 00 00     	movq	$0x10, 0x78(%rbp)
1400d26ae: 0f 11 b5 80 00 00 00        	movups	%xmm6, 0x80(%rbp)
1400d26b5: 48 c7 85 90 00 00 00 10 00 00 00    	movq	$0x10, 0x90(%rbp)
1400d26c0: 0f 11 b5 98 00 00 00        	movups	%xmm6, 0x98(%rbp)
1400d26c7: 48 c7 85 a8 00 00 00 10 00 00 00    	movq	$0x10, 0xa8(%rbp)
1400d26d2: 0f 11 b5 b0 00 00 00        	movups	%xmm6, 0xb0(%rbp)
1400d26d9: 48 c7 85 c0 00 00 00 10 00 00 00    	movq	$0x10, 0xc0(%rbp)
1400d26e4: 0f 11 b5 c8 00 00 00        	movups	%xmm6, 0xc8(%rbp)
1400d26eb: 48 c7 85 d8 00 00 00 10 00 00 00    	movq	$0x10, 0xd8(%rbp)
1400d26f6: 0f 11 b5 e0 00 00 00        	movups	%xmm6, 0xe0(%rbp)
1400d26fd: c7 85 f0 00 00 00 00 00 00 00       	movl	$0x0, 0xf0(%rbp)
1400d2707: c7 85 f8 00 00 00 00 00 00 00       	movl	$0x0, 0xf8(%rbp)
1400d2711: 66 c7 85 fc 00 00 00 01 00  	movw	$0x1, 0xfc(%rbp)
1400d271a: c7 85 00 01 00 00 00 00 00 00       	movl	$0x0, 0x100(%rbp)
1400d2724: 0f b6 05 de 24 56 03        	movzbl	0x35624de(%rip), %eax   # 0x143634c09
1400d272b: b9 60 01 00 00              	movl	$0x160, %ecx            # imm = 0x160
1400d2730: ba 08 00 00 00              	movl	$0x8, %edx
1400d2735: e8 d6 14 54 00              	callq	0x140613c10 <.text+0x612c10>
1400d273a: 48 85 c0                    	testq	%rax, %rax
1400d273d: 4c 89 bd 08 01 00 00        	movq	%r15, 0x108(%rbp)
1400d2744: 0f 84 e9 01 00 00           	je	0x1400d2933 <.text+0xd1933>
1400d274a: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1400d2751: 48 c7 40 20 00 00 00 00     	movq	$0x0, 0x20(%rax)
1400d2759: 48 c7 40 28 08 00 00 00     	movq	$0x8, 0x28(%rax)
1400d2761: 48 c7 40 30 00 00 00 00     	movq	$0x0, 0x30(%rax)
1400d2769: 48 c7 40 38 10 00 00 00     	movq	$0x10, 0x38(%rax)
1400d2771: 0f 11 70 40                 	movups	%xmm6, 0x40(%rax)
1400d2775: 48 c7 40 50 10 00 00 00     	movq	$0x10, 0x50(%rax)
1400d277d: 0f 11 70 58                 	movups	%xmm6, 0x58(%rax)
1400d2781: 48 c7 40 68 10 00 00 00     	movq	$0x10, 0x68(%rax)
1400d2789: 0f 11 70 70                 	movups	%xmm6, 0x70(%rax)
1400d278d: 48 c7 80 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rax)
1400d2798: 0f 11 b0 88 00 00 00        	movups	%xmm6, 0x88(%rax)
1400d279f: 48 c7 80 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rax)
1400d27aa: 0f 11 b0 a0 00 00 00        	movups	%xmm6, 0xa0(%rax)
1400d27b1: c7 80 b0 00 00 00 00 00 00 00       	movl	$0x0, 0xb0(%rax)
1400d27bb: 48 89 b8 b8 00 00 00        	movq	%rdi, 0xb8(%rax)
1400d27c2: 48 89 98 c0 00 00 00        	movq	%rbx, 0xc0(%rax)
1400d27c9: 48 c7 80 c8 00 00 00 6b 00 00 00    	movq	$0x6b, 0xc8(%rax)
1400d27d4: 48 c7 80 d0 00 00 00 10 00 00 00    	movq	$0x10, 0xd0(%rax)
1400d27df: 0f 11 b0 d8 00 00 00        	movups	%xmm6, 0xd8(%rax)
1400d27e6: 48 c7 80 e8 00 00 00 10 00 00 00    	movq	$0x10, 0xe8(%rax)
1400d27f1: 0f 11 b0 f0 00 00 00        	movups	%xmm6, 0xf0(%rax)
1400d27f8: 48 c7 80 00 01 00 00 10 00 00 00    	movq	$0x10, 0x100(%rax)
1400d2803: 0f 11 b0 08 01 00 00        	movups	%xmm6, 0x108(%rax)
1400d280a: 48 c7 80 18 01 00 00 10 00 00 00    	movq	$0x10, 0x118(%rax)
1400d2815: 0f 11 b0 20 01 00 00        	movups	%xmm6, 0x120(%rax)
1400d281c: 48 c7 80 30 01 00 00 10 00 00 00    	movq	$0x10, 0x130(%rax)
1400d2827: 0f 11 b0 38 01 00 00        	movups	%xmm6, 0x138(%rax)
1400d282e: c7 80 48 01 00 00 00 00 00 00       	movl	$0x0, 0x148(%rax)
1400d2838: c7 80 50 01 00 00 00 00 00 00       	movl	$0x0, 0x150(%rax)
1400d2842: 66 c7 80 54 01 00 00 01 00  	movw	$0x1, 0x154(%rax)
1400d284b: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1400d2852: c7 80 58 01 00 00 00 00 00 00       	movl	$0x0, 0x158(%rax)
1400d285c: 31 c0                       	xorl	%eax, %eax
1400d285e: 49 3b 07                    	cmpq	(%r15), %rax
1400d2861: 71 26                       	jno	0x1400d2889 <.text+0xd1889>
1400d2863: 49 8d 7f 30                 	leaq	0x30(%r15), %rdi
1400d2867: 49 8b 5f 30                 	movq	0x30(%r15), %rbx
1400d286b: 41 be 28 00 00 00           	movl	$0x28, %r14d
1400d2871: 49 3b 5f 20                 	cmpq	0x20(%r15), %rbx
1400d2875: 75 36                       	jne	0x1400d28ad <.text+0xd18ad>
1400d2877: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
1400d287b: 48 8d 15 b6 78 86 02        	leaq	0x28678b6(%rip), %rdx   # 0x14293a138
1400d2882: e8 49 dc 66 00              	callq	0x1407404d0 <.text+0x73f4d0>
1400d2887: eb 24                       	jmp	0x1400d28ad <.text+0xd18ad>
1400d2889: 49 8d 7f 58                 	leaq	0x58(%r15), %rdi
1400d288d: 49 8b 5f 58                 	movq	0x58(%r15), %rbx
1400d2891: 41 be 50 00 00 00           	movl	$0x50, %r14d
1400d2897: 49 3b 5f 48                 	cmpq	0x48(%r15), %rbx
1400d289b: 75 10                       	jne	0x1400d28ad <.text+0xd18ad>
1400d289d: 49 8d 4f 48                 	leaq	0x48(%r15), %rcx
1400d28a1: 48 8d 15 78 78 86 02        	leaq	0x2867878(%rip), %rdx   # 0x14293a120
1400d28a8: e8 23 dc 66 00              	callq	0x1407404d0 <.text+0x73f4d0>
1400d28ad: 4c 8b 85 08 01 00 00        	movq	0x108(%rbp), %r8
1400d28b4: 4b 8b 04 30                 	movq	(%r8,%r14), %rax
1400d28b8: 48 89 d9                    	movq	%rbx, %rcx
1400d28bb: 48 c1 e1 04                 	shlq	$0x4, %rcx
1400d28bf: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
1400d28c6: 48 89 14 08                 	movq	%rdx, (%rax,%rcx)
1400d28ca: 48 8d 15 cf 76 86 02        	leaq	0x28676cf(%rip), %rdx   # 0x142939fa0
1400d28d1: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
1400d28d6: 48 ff c3                    	incq	%rbx
1400d28d9: 48 89 1f                    	movq	%rbx, (%rdi)
1400d28dc: 41 0f 10 40 60              	movups	0x60(%r8), %xmm0
1400d28e1: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
1400d28e5: 41 0f 10 40 50              	movups	0x50(%r8), %xmm0
1400d28ea: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
1400d28ee: 41 0f 10 40 40              	movups	0x40(%r8), %xmm0
1400d28f3: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
1400d28f7: 41 0f 10 00                 	movups	(%r8), %xmm0
1400d28fb: 41 0f 10 48 10              	movups	0x10(%r8), %xmm1
1400d2900: 41 0f 10 50 20              	movups	0x20(%r8), %xmm2
1400d2905: 41 0f 10 58 30              	movups	0x30(%r8), %xmm3
1400d290a: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
1400d290e: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
1400d2912: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
1400d2916: 0f 11 06                    	movups	%xmm0, (%rsi)
1400d2919: 48 89 f0                    	movq	%rsi, %rax
1400d291c: 0f 28 b5 20 01 00 00        	movaps	0x120(%rbp), %xmm6
1400d2923: 48 81 c4 b8 01 00 00        	addq	$0x1b8, %rsp            # imm = 0x1B8
1400d292a: 5b                          	popq	%rbx
1400d292b: 5f                          	popq	%rdi
1400d292c: 5e                          	popq	%rsi
1400d292d: 41 5e                       	popq	%r14
1400d292f: 41 5f                       	popq	%r15
1400d2931: 5d                          	popq	%rbp
1400d2932: c3                          	retq
1400d2933: b9 08 00 00 00              	movl	$0x8, %ecx
1400d2938: ba 60 01 00 00              	movl	$0x160, %edx            # imm = 0x160
1400d293d: e8 be 66 80 02              	callq	0x1428d9000 <.text+0x28d8000>
1400d2942: 0f 0b                       	ud2
1400d2944: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1400d2950: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400d2955: 55                          	pushq	%rbp
1400d2956: 41 57                       	pushq	%r15
1400d2958: 41 56                       	pushq	%r14
1400d295a: 56                          	pushq	%rsi
1400d295b: 57                          	pushq	%rdi
1400d295c: 53                          	pushq	%rbx
1400d295d: 48 83 ec 38                 	subq	$0x38, %rsp
1400d2961: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400d2968: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1400d296d: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
1400d2971: e8 fa 34 51 00              	callq	0x1405e5e70 <.text+0x5e4e70>
1400d2976: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1400d297b: 48 83 c4 38                 	addq	$0x38, %rsp
1400d297f: 5b                          	popq	%rbx
1400d2980: 5f                          	popq	%rdi
1400d2981: 5e                          	popq	%rsi
1400d2982: 41 5e                       	popq	%r14
1400d2984: 41 5f                       	popq	%r15
1400d2986: 5d                          	popq	%rbp
1400d2987: c3                          	retq
1400d2988: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1400d2990: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400d2995: 55                          	pushq	%rbp
1400d2996: 41 57                       	pushq	%r15
1400d2998: 41 56                       	pushq	%r14
1400d299a: 56                          	pushq	%rsi
1400d299b: 57                          	pushq	%rdi
1400d299c: 53                          	pushq	%rbx
1400d299d: 48 83 ec 38                 	subq	$0x38, %rsp
1400d29a1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400d29a8: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1400d29ad: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1400d29b4: e8 c7 6e f5 ff              	callq	0x140029880 <.text+0x28880>
1400d29b9: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1400d29be: 48 83 c4 38                 	addq	$0x38, %rsp
1400d29c2: 5b                          	popq	%rbx
1400d29c3: 5f                          	popq	%rdi
1400d29c4: 5e                          	popq	%rsi
1400d29c5: 41 5e                       	popq	%r14
1400d29c7: 41 5f                       	popq	%r15
1400d29c9: 5d                          	popq	%rbp
1400d29ca: c3                          	retq
1400d29cb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1400d29d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400d29d5: 55                          	pushq	%rbp
1400d29d6: 41 57                       	pushq	%r15
1400d29d8: 41 56                       	pushq	%r14
1400d29da: 56                          	pushq	%rsi
1400d29db: 57                          	pushq	%rdi
1400d29dc: 53                          	pushq	%rbx
1400d29dd: 48 83 ec 38                 	subq	$0x38, %rsp
1400d29e1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400d29e8: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1400d29ed: 48 8d 15 ac 75 86 02        	leaq	0x28675ac(%rip), %rdx   # 0x142939fa0
1400d29f4: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
1400d29fb: e8 00 bf f5 ff              	callq	0x14002e900 <.text+0x2d900>
1400d2a00: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1400d2a05: 48 83 c4 38                 	addq	$0x38, %rsp
1400d2a09: 5b                          	popq	%rbx
1400d2a0a: 5f                          	popq	%rdi
1400d2a0b: 5e                          	popq	%rsi
1400d2a0c: 41 5e                       	popq	%r14
1400d2a0e: 41 5f                       	popq	%r15
1400d2a10: 5d                          	popq	%rbp
1400d2a11: c3                          	retq
1400d2a12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1400d2a20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400d2a25: 55                          	pushq	%rbp
1400d2a26: 41 57                       	pushq	%r15
1400d2a28: 41 56                       	pushq	%r14
1400d2a2a: 56                          	pushq	%rsi
1400d2a2b: 57                          	pushq	%rdi
1400d2a2c: 53                          	pushq	%rbx
1400d2a2d: 48 83 ec 38                 	subq	$0x38, %rsp
1400d2a31: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400d2a38: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1400d2a3d: 48 8d 15 5c 75 86 02        	leaq	0x286755c(%rip), %rdx   # 0x142939fa0
1400d2a44: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
1400d2a4b: e8 b0 be f5 ff              	callq	0x14002e900 <.text+0x2d900>
1400d2a50: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1400d2a55: 48 83 c4 38                 	addq	$0x38, %rsp
1400d2a59: 5b                          	popq	%rbx
1400d2a5a: 5f                          	popq	%rdi
1400d2a5b: 5e                          	popq	%rsi
1400d2a5c: 41 5e                       	popq	%r14
1400d2a5e: 41 5f                       	popq	%r15
1400d2a60: 5d                          	popq	%rbp
1400d2a61: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403770b0: 55                          	pushq	%rbp
1403770b1: 56                          	pushq	%rsi
1403770b2: 57                          	pushq	%rdi
1403770b3: 53                          	pushq	%rbx
1403770b4: 48 81 ec a8 02 00 00        	subq	$0x2a8, %rsp            # imm = 0x2A8
1403770bb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403770c3: 48 c7 85 20 02 00 00 fe ff ff ff    	movq	$-0x2, 0x220(%rbp)
1403770ce: 48 89 ce                    	movq	%rcx, %rsi
1403770d1: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403770d4: 0f 11 85 00 01 00 00        	movups	%xmm0, 0x100(%rbp)
1403770db: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
1403770e2: 0f 11 85 30 01 00 00        	movups	%xmm0, 0x130(%rbp)
1403770e9: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
1403770f0: 0f 11 85 60 01 00 00        	movups	%xmm0, 0x160(%rbp)
1403770f7: c7 85 70 01 00 00 00 00 00 00       	movl	$0x0, 0x170(%rbp)
140377101: 0f 11 85 98 01 00 00        	movups	%xmm0, 0x198(%rbp)
140377108: 0f 11 85 b0 01 00 00        	movups	%xmm0, 0x1b0(%rbp)
14037710f: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
140377116: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
14037711d: 0f 11 85 f8 01 00 00        	movups	%xmm0, 0x1f8(%rbp)
140377124: c7 85 08 02 00 00 00 00 00 00       	movl	$0x0, 0x208(%rbp)
14037712e: 48 c7 45 10 00 00 00 00     	movq	$0x0, 0x10(%rbp)
140377136: 48 c7 85 e0 00 00 00 00 00 00 00    	movq	$0x0, 0xe0(%rbp)
140377141: 48 c7 85 e8 00 00 00 08 00 00 00    	movq	$0x8, 0xe8(%rbp)
14037714c: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
140377157: 48 c7 85 f8 00 00 00 10 00 00 00    	movq	$0x10, 0xf8(%rbp)
140377162: 48 c7 85 10 01 00 00 10 00 00 00    	movq	$0x10, 0x110(%rbp)
14037716d: 48 c7 85 28 01 00 00 10 00 00 00    	movq	$0x10, 0x128(%rbp)
140377178: 48 c7 85 40 01 00 00 10 00 00 00    	movq	$0x10, 0x140(%rbp)
140377183: 48 c7 85 58 01 00 00 10 00 00 00    	movq	$0x10, 0x158(%rbp)
14037718e: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140377198: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14037719f: 48 8d 05 af 19 65 02        	leaq	0x26519af(%rip), %rax   # 0x1429c8b55
1403771a6: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1403771ad: 48 c7 85 88 01 00 00 31 00 00 00    	movq	$0x31, 0x188(%rbp)
1403771b8: 48 c7 85 90 01 00 00 10 00 00 00    	movq	$0x10, 0x190(%rbp)
1403771c3: 48 c7 85 a8 01 00 00 10 00 00 00    	movq	$0x10, 0x1a8(%rbp)
1403771ce: 48 c7 85 c0 01 00 00 10 00 00 00    	movq	$0x10, 0x1c0(%rbp)
1403771d9: 48 c7 85 d8 01 00 00 10 00 00 00    	movq	$0x10, 0x1d8(%rbp)
1403771e4: 48 c7 85 f0 01 00 00 10 00 00 00    	movq	$0x10, 0x1f0(%rbp)
1403771ef: c7 85 10 02 00 00 00 00 00 00       	movl	$0x0, 0x210(%rbp)
1403771f9: 66 c7 85 14 02 00 00 01 00  	movw	$0x1, 0x214(%rbp)
140377202: c7 85 18 02 00 00 00 00 00 00       	movl	$0x0, 0x218(%rbp)
14037720c: 0f b6 05 f6 d9 2b 03        	movzbl	0x32bd9f6(%rip), %eax   # 0x143634c09
140377213: b9 10 02 00 00              	movl	$0x210, %ecx            # imm = 0x210
140377218: ba 08 00 00 00              	movl	$0x8, %edx
14037721d: e8 ee c9 29 00              	callq	0x140613c10 <.text+0x612c10>
140377222: 48 85 c0                    	testq	%rax, %rax
140377225: 74 45                       	je	0x14037726c <.text+0x37626c>
140377227: 48 89 c7                    	movq	%rax, %rdi
14037722a: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14037722e: 41 b8 10 02 00 00           	movl	$0x210, %r8d            # imm = 0x210
140377234: 48 89 c1                    	movq	%rax, %rcx
140377237: e8 c4 c9 5a 02              	callq	0x142923c00 <.text+0x2922c00>
14037723c: 4c 8d 05 3d 18 65 02        	leaq	0x265183d(%rip), %r8    # 0x1429c8a80
140377243: 48 8d 5d a0                 	leaq	-0x60(%rbp), %rbx
140377247: 48 89 d9                    	movq	%rbx, %rcx
14037724a: 48 89 fa                    	movq	%rdi, %rdx
14037724d: e8 be d8 96 00              	callq	0x140ce4b10 <.text+0xce3b10>
140377252: 48 89 f1                    	movq	%rsi, %rcx
140377255: 48 89 da                    	movq	%rbx, %rdx
140377258: e8 93 b3 d5 ff              	callq	0x1400d25f0 <.text+0xd15f0>
14037725d: 48 89 f0                    	movq	%rsi, %rax
140377260: 48 81 c4 a8 02 00 00        	addq	$0x2a8, %rsp            # imm = 0x2A8
140377267: 5b                          	popq	%rbx
140377268: 5f                          	popq	%rdi
140377269: 5e                          	popq	%rsi
14037726a: 5d                          	popq	%rbp
14037726b: c3                          	retq
14037726c: b9 08 00 00 00              	movl	$0x8, %ecx
140377271: ba 10 02 00 00              	movl	$0x210, %edx            # imm = 0x210
140377276: e8 85 1d 56 02              	callq	0x1428d9000 <.text+0x28d8000>
14037727b: 0f 0b                       	ud2
14037727d: 0f 1f 00                    	nopl	(%rax)
140377280: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140377285: 55                          	pushq	%rbp
140377286: 56                          	pushq	%rsi
140377287: 57                          	pushq	%rdi
140377288: 53                          	pushq	%rbx
140377289: 48 83 ec 28                 	subq	$0x28, %rsp
14037728d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140377294: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140377298: e8 33 ae 26 00              	callq	0x1405e20d0 <.text+0x5e10d0>
14037729d: 90                          	nop
14037729e: 48 83 c4 28                 	addq	$0x28, %rsp
1403772a2: 5b                          	popq	%rbx
1403772a3: 5f                          	popq	%rdi
1403772a4: 5e                          	popq	%rsi
1403772a5: 5d                          	popq	%rbp
1403772a6: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14075b810: 55                          	pushq	%rbp
14075b811: 41 57                       	pushq	%r15
14075b813: 41 56                       	pushq	%r14
14075b815: 41 55                       	pushq	%r13
14075b817: 41 54                       	pushq	%r12
14075b819: 56                          	pushq	%rsi
14075b81a: 57                          	pushq	%rdi
14075b81b: 53                          	pushq	%rbx
14075b81c: 48 81 ec 18 02 00 00        	subq	$0x218, %rsp            # imm = 0x218
14075b823: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14075b82b: 44 0f 29 a5 80 01 00 00     	movaps	%xmm12, 0x180(%rbp)
14075b833: 44 0f 29 9d 70 01 00 00     	movaps	%xmm11, 0x170(%rbp)
14075b83b: 44 0f 29 95 60 01 00 00     	movaps	%xmm10, 0x160(%rbp)
14075b843: 66 44 0f 7f 8d 50 01 00 00  	movdqa	%xmm9, 0x150(%rbp)
14075b84c: 44 0f 29 85 40 01 00 00     	movaps	%xmm8, 0x140(%rbp)
14075b854: 0f 29 bd 30 01 00 00        	movaps	%xmm7, 0x130(%rbp)
14075b85b: 66 0f 7f b5 20 01 00 00     	movdqa	%xmm6, 0x120(%rbp)
14075b863: 48 c7 85 18 01 00 00 fe ff ff ff    	movq	$-0x2, 0x118(%rbp)
14075b86e: 4c 89 4d 78                 	movq	%r9, 0x78(%rbp)
14075b872: 4c 89 45 70                 	movq	%r8, 0x70(%rbp)
14075b876: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
14075b87a: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
14075b87e: e8 7d b1 4b 00              	callq	0x140c16a00 <.text+0xc15a00>
14075b883: 41 89 c7                    	movl	%eax, %r15d
14075b886: 45 84 ff                    	testb	%r15b, %r15b
14075b889: 0f 84 1d 02 00 00           	je	0x14075baac <.text+0x75aaac>
14075b88f: 48 8d 3d 3d 16 32 02        	leaq	0x232163d(%rip), %rdi   # 0x142a7ced3
14075b896: be 3a 00 00 00              	movl	$0x3a, %esi
14075b89b: ba 3a 00 00 00              	movl	$0x3a, %edx
14075b8a0: 48 89 f9                    	movq	%rdi, %rcx
14075b8a3: e8 48 fb ff ff              	callq	0x14075b3f0 <.text+0x75a3f0>
14075b8a8: 48 83 f8 01                 	cmpq	$0x1, %rax
14075b8ac: 0f 85 aa 00 00 00           	jne	0x14075b95c <.text+0x75a95c>
14075b8b2: 49 89 d1                    	movq	%rdx, %r9
14075b8b5: 48 85 d2                    	testq	%rdx, %rdx
14075b8b8: 74 36                       	je	0x14075b8f0 <.text+0x75a8f0>
14075b8ba: 49 83 f9 3a                 	cmpq	$0x3a, %r9
14075b8be: 73 2e                       	jae	0x14075b8ee <.text+0x75a8ee>
14075b8c0: 48 8d 05 0c 16 32 02        	leaq	0x232160c(%rip), %rax   # 0x142a7ced3
14075b8c7: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14075b8cc: 7f 22                       	jg	0x14075b8f0 <.text+0x75a8f0>
14075b8ce: 48 8d 05 f3 16 32 02        	leaq	0x23216f3(%rip), %rax   # 0x142a7cfc8
14075b8d5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14075b8da: 48 8d 0d f2 15 32 02        	leaq	0x23215f2(%rip), %rcx   # 0x142a7ced3
14075b8e1: ba 3a 00 00 00              	movl	$0x3a, %edx
14075b8e6: 45 31 c0                    	xorl	%r8d, %r8d
14075b8e9: e8 52 e0 17 02              	callq	0x1428d9940 <.text+0x28d8940>
14075b8ee: 75 de                       	jne	0x14075b8ce <.text+0x75a8ce>
14075b8f0: 48 8d 3d dc 15 32 02        	leaq	0x23215dc(%rip), %rdi   # 0x142a7ced3
14075b8f7: 48 89 f9                    	movq	%rdi, %rcx
14075b8fa: 4c 89 ca                    	movq	%r9, %rdx
14075b8fd: e8 ee fa ff ff              	callq	0x14075b3f0 <.text+0x75a3f0>
14075b902: 48 83 f8 01                 	cmpq	$0x1, %rax
14075b906: 75 54                       	jne	0x14075b95c <.text+0x75a95c>
14075b908: 49 89 d0                    	movq	%rdx, %r8
14075b90b: 49 83 c0 02                 	addq	$0x2, %r8
14075b90f: 74 39                       	je	0x14075b94a <.text+0x75a94a>
14075b911: 49 83 f8 3a                 	cmpq	$0x3a, %r8
14075b915: 73 31                       	jae	0x14075b948 <.text+0x75a948>
14075b917: 48 8d 05 b5 15 32 02        	leaq	0x23215b5(%rip), %rax   # 0x142a7ced3
14075b91e: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14075b923: 7f 25                       	jg	0x14075b94a <.text+0x75a94a>
14075b925: 48 8d 05 b4 16 32 02        	leaq	0x23216b4(%rip), %rax   # 0x142a7cfe0
14075b92c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14075b931: 48 8d 0d 9b 15 32 02        	leaq	0x232159b(%rip), %rcx   # 0x142a7ced3
14075b938: ba 3a 00 00 00              	movl	$0x3a, %edx
14075b93d: 41 b9 3a 00 00 00           	movl	$0x3a, %r9d
14075b943: e8 f8 df 17 02              	callq	0x1428d9940 <.text+0x28d8940>
14075b948: 75 db                       	jne	0x14075b925 <.text+0x75a925>
14075b94a: be 38 00 00 00              	movl	$0x38, %esi
14075b94f: 48 29 d6                    	subq	%rdx, %rsi
14075b952: 48 8d 3d 7a 15 32 02        	leaq	0x232157a(%rip), %rdi   # 0x142a7ced3
14075b959: 4c 01 c7                    	addq	%r8, %rdi
14075b95c: 4c 8d 05 8b 17 32 02        	leaq	0x232178b(%rip), %r8    # 0x142a7d0ee
14075b963: 4c 8d 35 4e 17 32 02        	leaq	0x232174e(%rip), %r14   # 0x142a7d0b8
14075b96a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14075b970: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14075b975: 85 c0                       	testl	%eax, %eax
14075b977: 78 17                       	js	0x14075b990 <.text+0x75a990>
14075b979: 49 ff c8                    	decq	%r8
14075b97c: 83 f8 5c                    	cmpl	$0x5c, %eax
14075b97f: 75 63                       	jne	0x14075b9e4 <.text+0x75a9e4>
14075b981: eb 72                       	jmp	0x14075b9f5 <.text+0x75a9f5>
14075b983: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14075b990: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14075b995: 80 f9 c0                    	cmpb	$-0x40, %cl
14075b998: 7d 23                       	jge	0x14075b9bd <.text+0x75a9bd>
14075b99a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14075b99f: 80 fa c0                    	cmpb	$-0x40, %dl
14075b9a2: 7d 22                       	jge	0x14075b9c6 <.text+0x75a9c6>
14075b9a4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14075b9a9: 49 83 c0 fc                 	addq	$-0x4, %r8
14075b9ad: 41 83 e1 07                 	andl	$0x7, %r9d
14075b9b1: 41 c1 e1 06                 	shll	$0x6, %r9d
14075b9b5: 83 e2 3f                    	andl	$0x3f, %edx
14075b9b8: 44 09 ca                    	orl	%r9d, %edx
14075b9bb: eb 10                       	jmp	0x14075b9cd <.text+0x75a9cd>
14075b9bd: 49 83 c0 fe                 	addq	$-0x2, %r8
14075b9c1: 83 e1 1f                    	andl	$0x1f, %ecx
14075b9c4: eb 0f                       	jmp	0x14075b9d5 <.text+0x75a9d5>
14075b9c6: 49 83 c0 fd                 	addq	$-0x3, %r8
14075b9ca: 83 e2 0f                    	andl	$0xf, %edx
14075b9cd: c1 e2 06                    	shll	$0x6, %edx
14075b9d0: 83 e1 3f                    	andl	$0x3f, %ecx
14075b9d3: 09 d1                       	orl	%edx, %ecx
14075b9d5: c1 e1 06                    	shll	$0x6, %ecx
14075b9d8: 24 3f                       	andb	$0x3f, %al
14075b9da: 0f b6 c0                    	movzbl	%al, %eax
14075b9dd: 09 c8                       	orl	%ecx, %eax
14075b9df: 83 f8 5c                    	cmpl	$0x5c, %eax
14075b9e2: 74 11                       	je	0x14075b9f5 <.text+0x75a9f5>
14075b9e4: 83 f8 2f                    	cmpl	$0x2f, %eax
14075b9e7: 74 0c                       	je	0x14075b9f5 <.text+0x75a9f5>
14075b9e9: 4d 39 f0                    	cmpq	%r14, %r8
14075b9ec: 75 82                       	jne	0x14075b970 <.text+0x75a970>
14075b9ee: bb 36 00 00 00              	movl	$0x36, %ebx
14075b9f3: eb 4c                       	jmp	0x14075ba41 <.text+0x75aa41>
14075b9f5: 4c 8d 35 bc 16 32 02        	leaq	0x23216bc(%rip), %r14   # 0x142a7d0b8
14075b9fc: 4d 29 f0                    	subq	%r14, %r8
14075b9ff: 49 ff c0                    	incq	%r8
14075ba02: 74 32                       	je	0x14075ba36 <.text+0x75aa36>
14075ba04: 49 83 f8 36                 	cmpq	$0x36, %r8
14075ba08: 73 2a                       	jae	0x14075ba34 <.text+0x75aa34>
14075ba0a: 43 80 3c 30 bf              	cmpb	$-0x41, (%r8,%r14)
14075ba0f: 7f 25                       	jg	0x14075ba36 <.text+0x75aa36>
14075ba11: 48 8d 05 90 15 32 02        	leaq	0x2321590(%rip), %rax   # 0x142a7cfa8
14075ba18: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14075ba1d: 48 8d 0d 94 16 32 02        	leaq	0x2321694(%rip), %rcx   # 0x142a7d0b8
14075ba24: ba 36 00 00 00              	movl	$0x36, %edx
14075ba29: 41 b9 36 00 00 00           	movl	$0x36, %r9d
14075ba2f: e8 0c df 17 02              	callq	0x1428d9940 <.text+0x28d8940>
14075ba34: 75 db                       	jne	0x14075ba11 <.text+0x75aa11>
14075ba36: bb 36 00 00 00              	movl	$0x36, %ebx
14075ba3b: 4c 29 c3                    	subq	%r8, %rbx
14075ba3e: 4d 01 c6                    	addq	%r8, %r14
14075ba41: e8 ea c6 4b 00              	callq	0x140c18130 <.text+0xc17130>
14075ba46: 48 89 c1                    	movq	%rax, %rcx
14075ba49: 48 8b 00                    	movq	(%rax), %rax
14075ba4c: 48 83 f8 01                 	cmpq	$0x1, %rax
14075ba50: 0f 85 68 11 00 00           	jne	0x14075cbbe <.text+0x75bbbe>
14075ba56: 48 83 c1 08                 	addq	$0x8, %rcx
14075ba5a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14075ba5e: 0f 85 6d 12 00 00           	jne	0x14075ccd1 <.text+0x75bcd1>
14075ba64: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14075ba6b: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14075ba72: 48 83 c1 08                 	addq	$0x8, %rcx
14075ba76: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14075ba7b: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14075ba84: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14075ba8d: 48 89 fa                    	movq	%rdi, %rdx
14075ba90: 49 89 f0                    	movq	%rsi, %r8
14075ba93: 4d 89 f1                    	movq	%r14, %r9
14075ba96: e8 a5 b2 4b 00              	callq	0x140c16d40 <.text+0xc15d40>
14075ba9b: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
14075baa2: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14075baa9: 48 ff 00                    	incq	(%rax)
14075baac: 44 88 bd 16 01 00 00        	movb	%r15b, 0x116(%rbp)
14075bab3: b1 01                       	movb	$0x1, %cl
14075bab5: 31 c0                       	xorl	%eax, %eax
14075bab7: f0                          	lock
14075bab8: 0f b0 0d 01 05 ed 02        	cmpxchgb	%cl, 0x2ed0501(%rip)    # 0x14362bfc0
14075babf: 0f 85 3e 11 00 00           	jne	0x14075cc03 <.text+0x75bc03>
14075bac5: 48 8b 35 54 09 ed 02        	movq	0x2ed0954(%rip), %rsi   # 0x14362c420
14075bacc: 48 8b 06                    	movq	(%rsi), %rax
14075bacf: 48 d1 e0                    	shlq	%rax
14075bad2: 48 85 c0                    	testq	%rax, %rax
14075bad5: 0f 85 39 11 00 00           	jne	0x14075cc14 <.text+0x75bc14>
14075badb: 31 c0                       	xorl	%eax, %eax
14075badd: 0f b6 0d dd 04 ed 02        	movzbl	0x2ed04dd(%rip), %ecx   # 0x14362bfc1
14075bae4: 84 c9                       	testb	%cl, %cl
14075bae6: 0f 85 3e 11 00 00           	jne	0x14075cc2a <.text+0x75bc2a>
14075baec: 89 85 08 01 00 00           	movl	%eax, 0x108(%rbp)
14075baf2: 48 8b 0d df 04 ed 02        	movq	0x2ed04df(%rip), %rcx   # 0x14362bfd8
14075baf9: 48 85 c9                    	testq	%rcx, %rcx
14075bafc: 74 1f                       	je	0x14075bb1d <.text+0x75ab1d>
14075bafe: 48 8b 05 cb 04 ed 02        	movq	0x2ed04cb(%rip), %rax   # 0x14362bfd0
14075bb05: 48 c1 e1 04                 	shlq	$0x4, %rcx
14075bb09: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14075bb0d: 31 d2                       	xorl	%edx, %edx
14075bb0f: 90                          	nop
14075bb10: ff 4c 10 28                 	decl	0x28(%rax,%rdx)
14075bb14: 48 83 c2 30                 	addq	$0x30, %rdx
14075bb18: 48 39 d1                    	cmpq	%rdx, %rcx
14075bb1b: 75 f3                       	jne	0x14075bb10 <.text+0x75ab10>
14075bb1d: 48 8d 0d a4 04 ed 02        	leaq	0x2ed04a4(%rip), %rcx   # 0x14362bfc8
14075bb24: e8 97 53 aa 00              	callq	0x141200ec0 <.text+0x11ffec0>
14075bb29: 80 bd 08 01 00 00 00        	cmpb	$0x0, 0x108(%rbp)
14075bb30: 75 0f                       	jne	0x14075bb41 <.text+0x75ab41>
14075bb32: 48 8b 06                    	movq	(%rsi), %rax
14075bb35: 48 d1 e0                    	shlq	%rax
14075bb38: 48 85 c0                    	testq	%rax, %rax
14075bb3b: 0f 85 9c 11 00 00           	jne	0x14075ccdd <.text+0x75bcdd>
14075bb41: 31 c0                       	xorl	%eax, %eax
14075bb43: 86 05 77 04 ed 02           	xchgb	%al, 0x2ed0477(%rip)    # 0x14362bfc0
14075bb49: 3c 02                       	cmpb	$0x2, %al
14075bb4b: 0f 84 1a 11 00 00           	je	0x14075cc6b <.text+0x75bc6b>
14075bb51: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
14075bb58: 4c 8b 10                    	movq	(%rax), %r10
14075bb5b: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14075bb5f: 4d 8b 02                    	movq	(%r10), %r8
14075bb62: 31 d2                       	xorl	%edx, %edx
14075bb64: 4c 89 c0                    	movq	%r8, %rax
14075bb67: 48 2b 41 18                 	subq	0x18(%rcx), %rax
14075bb6b: 48 0f 42 c2                 	cmovbq	%rdx, %rax
14075bb6f: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
14075bb73: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
14075bb77: 49 89 c7                    	movq	%rax, %r15
14075bb7a: 49 c1 e7 04                 	shlq	$0x4, %r15
14075bb7e: 4c 03 79 08                 	addq	0x8(%rcx), %r15
14075bb82: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
14075bb86: 48 8b 71 30                 	movq	0x30(%rcx), %rsi
14075bb8a: 49 29 c3                    	subq	%rax, %r11
14075bb8d: 4c 0f 42 da                 	cmovbq	%rdx, %r11
14075bb91: 41 b9 08 00 00 00           	movl	$0x8, %r9d
14075bb97: 4d 0f 42 f9                 	cmovbq	%r9, %r15
14075bb9b: 4c 89 c0                    	movq	%r8, %rax
14075bb9e: 48 c1 e0 04                 	shlq	$0x4, %rax
14075bba2: 48 03 41 28                 	addq	0x28(%rcx), %rax
14075bba6: 4c 29 c6                    	subq	%r8, %rsi
14075bba9: 48 0f 42 f2                 	cmovbq	%rdx, %rsi
14075bbad: 49 0f 42 c1                 	cmovbq	%r9, %rax
14075bbb1: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14075bbb5: 48 29 f1                    	subq	%rsi, %rcx
14075bbb8: 4c 29 d9                    	subq	%r11, %rcx
14075bbbb: 4c 89 95 f0 00 00 00        	movq	%r10, 0xf0(%rbp)
14075bbc2: 49 89 0a                    	movq	%rcx, (%r10)
14075bbc5: 49 c1 e3 04                 	shlq	$0x4, %r11
14075bbc9: 4d 01 fb                    	addq	%r15, %r11
14075bbcc: 4c 89 9d f8 00 00 00        	movq	%r11, 0xf8(%rbp)
14075bbd3: 48 c1 e6 04                 	shlq	$0x4, %rsi
14075bbd7: 48 01 c6                    	addq	%rax, %rsi
14075bbda: 48 89 b5 88 00 00 00        	movq	%rsi, 0x88(%rbp)
14075bbe1: 48 bb 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rbx # imm = 0x8000000000000001
14075bbeb: 49 bd 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r13 # imm = 0x8000000000000000
14075bbf5: 4c 8d 75 10                 	leaq	0x10(%rbp), %r14
14075bbf9: eb 13                       	jmp	0x14075bc0e <.text+0x75ac0e>
14075bbfb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14075bc00: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14075bc07: 4c 8b bd 00 01 00 00        	movq	0x100(%rbp), %r15
14075bc0e: 4d 85 ff                    	testq	%r15, %r15
14075bc11: 74 1d                       	je	0x14075bc30 <.text+0x75ac30>
14075bc13: 4c 3b bd f8 00 00 00        	cmpq	0xf8(%rbp), %r15
14075bc1a: 74 14                       	je	0x14075bc30 <.text+0x75ac30>
14075bc1c: 49 8d 4f 10                 	leaq	0x10(%r15), %rcx
14075bc20: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
14075bc27: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
14075bc2e: eb 2f                       	jmp	0x14075bc5f <.text+0x75ac5f>
14075bc30: 48 85 c0                    	testq	%rax, %rax
14075bc33: 0f 84 de 02 00 00           	je	0x14075bf17 <.text+0x75af17>
14075bc39: 48 3b 85 88 00 00 00        	cmpq	0x88(%rbp), %rax
14075bc40: 0f 84 d1 02 00 00           	je	0x14075bf17 <.text+0x75af17>
14075bc46: 48 8d 48 10                 	leaq	0x10(%rax), %rcx
14075bc4a: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14075bc51: 48 c7 85 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbp)
14075bc5c: 49 89 c7                    	movq	%rax, %r15
14075bc5f: 49 83 c7 08                 	addq	$0x8, %r15
14075bc63: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14075bc6a: 48 ff 00                    	incq	(%rax)
14075bc6d: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14075bc71: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14075bc74: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14075bc78: 89 01                       	movl	%eax, (%rcx)
14075bc7a: 4c 8b 22                    	movq	(%rdx), %r12
14075bc7d: 49 8d 4c 24 20              	leaq	0x20(%r12), %rcx
14075bc82: 4c 89 fa                    	movq	%r15, %rdx
14075bc85: e8 e6 50 e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14075bc8a: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14075bc91: 4c 89 e2                    	movq	%r12, %rdx
14075bc94: 49 89 c0                    	movq	%rax, %r8
14075bc97: 4d 89 f9                    	movq	%r15, %r9
14075bc9a: e8 51 bb 41 00              	callq	0x140b777f0 <.text+0xb767f0>
14075bc9f: 48 8b b5 98 00 00 00        	movq	0x98(%rbp), %rsi
14075bca6: 48 89 f0                    	movq	%rsi, %rax
14075bca9: 48 f7 d8                    	negq	%rax
14075bcac: 70 0f                       	jo	0x14075bcbd <.text+0x75acbd>
14075bcae: 48 8d 85 98 00 00 00        	leaq	0x98(%rbp), %rax
14075bcb5: 0f 10 40 08                 	movups	0x8(%rax), %xmm0
14075bcb9: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
14075bcbd: 48 89 b5 90 00 00 00        	movq	%rsi, 0x90(%rbp)
14075bcc4: 66 0f 6f 45 10              	movdqa	0x10(%rbp), %xmm0
14075bcc9: 48 8d 85 98 00 00 00        	leaq	0x98(%rbp), %rax
14075bcd0: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14075bcd4: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14075bcdf: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
14075bcea: 45 31 ff                    	xorl	%r15d, %r15d
14075bced: 45 31 e4                    	xorl	%r12d, %r12d
14075bcf0: 4d 85 e4                    	testq	%r12, %r12
14075bcf3: 75 4b                       	jne	0x14075bd40 <.text+0x75ad40>
14075bcf5: eb 34                       	jmp	0x14075bd2b <.text+0x75ad2b>
14075bcf7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14075bd00: 49 8d 04 f7                 	leaq	(%r15,%rsi,8), %rax
14075bd04: 4c 89 bd a8 00 00 00        	movq	%r15, 0xa8(%rbp)
14075bd0b: 4c 89 bd b0 00 00 00        	movq	%r15, 0xb0(%rbp)
14075bd12: 4c 89 a5 b8 00 00 00        	movq	%r12, 0xb8(%rbp)
14075bd19: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14075bd20: 4c 89 ee                    	movq	%r13, %rsi
14075bd23: 4d 89 fc                    	movq	%r15, %r12
14075bd26: 4d 85 e4                    	testq	%r12, %r12
14075bd29: 75 15                       	jne	0x14075bd40 <.text+0x75ad40>
14075bd2b: 4c 89 ff                    	movq	%r15, %rdi
14075bd2e: 49 89 f4                    	movq	%rsi, %r12
14075bd31: 48 89 de                    	movq	%rbx, %rsi
14075bd34: 49 39 dc                    	cmpq	%rbx, %r12
14075bd37: 75 6c                       	jne	0x14075bda5 <.text+0x75ada5>
14075bd39: eb 7c                       	jmp	0x14075bdb7 <.text+0x75adb7>
14075bd3b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14075bd40: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14075bd47: 48 3b 85 c0 00 00 00        	cmpq	0xc0(%rbp), %rax
14075bd4e: 74 1b                       	je	0x14075bd6b <.text+0x75ad6b>
14075bd50: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
14075bd54: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
14075bd5b: 48 8b 00                    	movq	(%rax), %rax
14075bd5e: 48 89 c1                    	movq	%rax, %rcx
14075bd61: 48 c1 e9 20                 	shrq	$0x20, %rcx
14075bd65: 0f 85 d5 00 00 00           	jne	0x14075be40 <.text+0x75ae40>
14075bd6b: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14075bd72: 48 85 d2                    	testq	%rdx, %rdx
14075bd75: 74 12                       	je	0x14075bd89 <.text+0x75ad89>
14075bd77: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075bd7b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075bd81: 4c 89 e1                    	movq	%r12, %rcx
14075bd84: e8 97 7e eb ff              	callq	0x140613c20 <.text+0x612c20>
14075bd89: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14075bd94: 4c 8b a5 90 00 00 00        	movq	0x90(%rbp), %r12
14075bd9b: 31 ff                       	xorl	%edi, %edi
14075bd9d: 48 89 de                    	movq	%rbx, %rsi
14075bda0: 49 39 dc                    	cmpq	%rbx, %r12
14075bda3: 74 12                       	je	0x14075bdb7 <.text+0x75adb7>
14075bda5: 4c 89 ad 90 00 00 00        	movq	%r13, 0x90(%rbp)
14075bdac: 4c 89 e0                    	movq	%r12, %rax
14075bdaf: 48 f7 d8                    	negq	%rax
14075bdb2: 4c 89 ee                    	movq	%r13, %rsi
14075bdb5: 71 49                       	jno	0x14075be00 <.text+0x75ae00>
14075bdb7: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
14075bdbe: 48 85 c9                    	testq	%rcx, %rcx
14075bdc1: 0f 84 ef 00 00 00           	je	0x14075beb6 <.text+0x75aeb6>
14075bdc7: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14075bdce: 48 3b 85 e0 00 00 00        	cmpq	0xe0(%rbp), %rax
14075bdd5: 0f 84 b5 00 00 00           	je	0x14075be90 <.text+0x75ae90>
14075bddb: 48 8d 50 08                 	leaq	0x8(%rax), %rdx
14075bddf: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
14075bde6: 48 8b 00                    	movq	(%rax), %rax
14075bde9: 48 89 c2                    	movq	%rax, %rdx
14075bdec: 48 c1 ea 20                 	shrq	$0x20, %rdx
14075bdf0: 0f 84 9a 00 00 00           	je	0x14075be90 <.text+0x75ae90>
14075bdf6: 45 31 e4                    	xorl	%r12d, %r12d
14075bdf9: eb 48                       	jmp	0x14075be43 <.text+0x75ae43>
14075bdfb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14075be00: 4c 8b bd 98 00 00 00        	movq	0x98(%rbp), %r15
14075be07: 48 8b b5 a0 00 00 00        	movq	0xa0(%rbp), %rsi
14075be0e: 48 85 ff                    	testq	%rdi, %rdi
14075be11: 0f 84 e9 fe ff ff           	je	0x14075bd00 <.text+0x75ad00>
14075be17: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14075be1e: 48 85 d2                    	testq	%rdx, %rdx
14075be21: 0f 84 d9 fe ff ff           	je	0x14075bd00 <.text+0x75ad00>
14075be27: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075be2b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075be31: 48 89 f9                    	movq	%rdi, %rcx
14075be34: e8 e7 7d eb ff              	callq	0x140613c20 <.text+0x612c20>
14075be39: e9 c2 fe ff ff              	jmp	0x14075bd00 <.text+0x75ad00>
14075be3e: 66 90                       	nop
14075be40: 4c 89 ff                    	movq	%r15, %rdi
14075be43: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
14075be4a: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
14075be4e: f3 0f 6f 49 10              	movdqu	0x10(%rcx), %xmm1
14075be53: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14075be57: f3 0f 7f 45 10              	movdqu	%xmm0, 0x10(%rbp)
14075be5c: f3 0f 7f 4d 20              	movdqu	%xmm1, 0x20(%rbp)
14075be61: 4c 89 f1                    	movq	%r14, %rcx
14075be64: 48 8d 15 cd 12 32 02        	leaq	0x23212cd(%rip), %rdx   # 0x142a7d138
14075be6b: e8 00 89 5b 00              	callq	0x140d14770 <.text+0xd13770>
14075be70: 49 89 ff                    	movq	%rdi, %r15
14075be73: 4d 85 e4                    	testq	%r12, %r12
14075be76: 0f 85 c4 fe ff ff           	jne	0x14075bd40 <.text+0x75ad40>
14075be7c: e9 aa fe ff ff              	jmp	0x14075bd2b <.text+0x75ad2b>
14075be81: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14075be90: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
14075be97: 48 85 d2                    	testq	%rdx, %rdx
14075be9a: 74 0f                       	je	0x14075beab <.text+0x75aeab>
14075be9c: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075bea0: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075bea6: e8 75 7d eb ff              	callq	0x140613c20 <.text+0x612c20>
14075beab: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
14075beb6: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
14075bebd: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
14075bec7: 48 39 c2                    	cmpq	%rax, %rdx
14075beca: 7c 1b                       	jl	0x14075bee7 <.text+0x75aee7>
14075becc: 48 85 d2                    	testq	%rdx, %rdx
14075becf: 74 16                       	je	0x14075bee7 <.text+0x75aee7>
14075bed1: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
14075bed8: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075bedc: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075bee2: e8 39 7d eb ff              	callq	0x140613c20 <.text+0x612c20>
14075bee7: 48 85 ff                    	testq	%rdi, %rdi
14075beea: 0f 84 10 fd ff ff           	je	0x14075bc00 <.text+0x75ac00>
14075bef0: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14075bef7: 48 85 d2                    	testq	%rdx, %rdx
14075befa: 0f 84 00 fd ff ff           	je	0x14075bc00 <.text+0x75ac00>
14075bf00: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075bf04: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075bf0a: 48 89 f9                    	movq	%rdi, %rcx
14075bf0d: e8 0e 7d eb ff              	callq	0x140613c20 <.text+0x612c20>
14075bf12: e9 e9 fc ff ff              	jmp	0x14075bc00 <.text+0x75ac00>
14075bf17: 48 8b 8d 10 02 00 00        	movq	0x210(%rbp), %rcx
14075bf1e: 48 8b 01                    	movq	(%rcx), %rax
14075bf21: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
14075bf25: 4c 8b 10                    	movq	(%rax), %r10
14075bf28: 45 31 c9                    	xorl	%r9d, %r9d
14075bf2b: 4d 89 d3                    	movq	%r10, %r11
14075bf2e: 4c 2b 5a 18                 	subq	0x18(%rdx), %r11
14075bf32: 4d 0f 42 d9                 	cmovbq	%r9, %r11
14075bf36: 4c 2b 52 38                 	subq	0x38(%rdx), %r10
14075bf3a: 4d 0f 42 d1                 	cmovbq	%r9, %r10
14075bf3e: 4c 89 d9                    	movq	%r11, %rcx
14075bf41: 48 c1 e1 04                 	shlq	$0x4, %rcx
14075bf45: 48 03 4a 08                 	addq	0x8(%rdx), %rcx
14075bf49: 4c 8b 42 10                 	movq	0x10(%rdx), %r8
14075bf4d: 4d 29 d8                    	subq	%r11, %r8
14075bf50: 4d 0f 42 c1                 	cmovbq	%r9, %r8
14075bf54: 41 bb 08 00 00 00           	movl	$0x8, %r11d
14075bf5a: 49 0f 42 cb                 	cmovbq	%r11, %rcx
14075bf5e: 4c 89 d6                    	movq	%r10, %rsi
14075bf61: 48 c1 e6 04                 	shlq	$0x4, %rsi
14075bf65: 48 03 72 28                 	addq	0x28(%rdx), %rsi
14075bf69: 48 8b 7a 30                 	movq	0x30(%rdx), %rdi
14075bf6d: 4c 29 d7                    	subq	%r10, %rdi
14075bf70: 49 0f 42 f9                 	cmovbq	%r9, %rdi
14075bf74: 49 0f 42 f3                 	cmovbq	%r11, %rsi
14075bf78: 4e 8d 0c 07                 	leaq	(%rdi,%r8), %r9
14075bf7c: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
14075bf80: 4c 29 ca                    	subq	%r9, %rdx
14075bf83: 48 89 10                    	movq	%rdx, (%rax)
14075bf86: 49 c1 e0 04                 	shlq	$0x4, %r8
14075bf8a: 49 01 c8                    	addq	%rcx, %r8
14075bf8d: 48 c1 e7 04                 	shlq	$0x4, %rdi
14075bf91: 48 01 f7                    	addq	%rsi, %rdi
14075bf94: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14075bf9b: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
14075bfa2: 4c 89 85 a0 00 00 00        	movq	%r8, 0xa0(%rbp)
14075bfa9: 48 89 b5 a8 00 00 00        	movq	%rsi, 0xa8(%rbp)
14075bfb0: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
14075bfb7: 4c 89 8d b8 00 00 00        	movq	%r9, 0xb8(%rbp)
14075bfbe: 4c 8d 05 ab 0e 32 02        	leaq	0x2320eab(%rip), %r8    # 0x142a7ce70
14075bfc5: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
14075bfc9: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
14075bfd0: e8 fb 76 aa 00              	callq	0x1412036d0 <.text+0x12026d0>
14075bfd5: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14075bfd9: 48 8b 00                    	movq	(%rax), %rax
14075bfdc: 4c 8b 30                    	movq	(%rax), %r14
14075bfdf: 48 8b 40 18                 	movq	0x18(%rax), %rax
14075bfe3: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
14075bfea: 66 41 0f 6f 06              	movdqa	(%r14), %xmm0
14075bfef: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14075bff3: f7 d0                       	notl	%eax
14075bff5: 4d 8d 66 10                 	leaq	0x10(%r14), %r12
14075bff9: 48 8b 95 28 02 00 00        	movq	0x228(%rbp), %rdx
14075c000: 48 8b 0a                    	movq	(%rdx), %rcx
14075c003: 4c 8b 42 10                 	movq	0x10(%rdx), %r8
14075c007: 4c 89 85 f0 00 00 00        	movq	%r8, 0xf0(%rbp)
14075c00e: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
14075c015: 48 8d 49 20                 	leaq	0x20(%rcx), %rcx
14075c019: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
14075c01d: 48 8b 8d 30 02 00 00        	movq	0x230(%rbp), %rcx
14075c024: 48 8b 09                    	movq	(%rcx), %rcx
14075c027: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
14075c02b: 8b 4a 1c                    	movl	0x1c(%rdx), %ecx
14075c02e: 89 8d f8 00 00 00           	movl	%ecx, 0xf8(%rbp)
14075c034: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
14075c03b: 8b 51 1c                    	movl	0x1c(%rcx), %edx
14075c03e: 89 55 04                    	movl	%edx, 0x4(%rbp)
14075c041: 48 8b 11                    	movq	(%rcx), %rdx
14075c044: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
14075c048: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
14075c04c: 48 89 55 f8                 	movq	%rdx, -0x8(%rbp)
14075c050: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
14075c054: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
14075c058: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
14075c05d: 66 0f 6e 35 e3 0c 32 02     	movd	0x2320ce3(%rip), %xmm6  # 0x142a7cd48
14075c065: 44 0f 28 15 13 29 1d 02     	movaps	0x21d2913(%rip), %xmm10 # 0x14292e980
14075c06d: f3 44 0f 10 1d fe 45 20 02  	movss	0x22045fe(%rip), %xmm11 # 0x142960674
14075c076: f3 44 0f 10 25 f1 97 1c 02  	movss	0x21c97f1(%rip), %xmm12 # 0x142925870
14075c07f: e9 81 02 00 00              	jmp	0x14075c305 <.text+0x75b305>
14075c084: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14075c090: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14075c096: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14075c09a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14075c09e: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14075c0a3: 45 85 d2                    	testl	%r10d, %r10d
14075c0a6: 74 58                       	je	0x14075c100 <.text+0x75b100>
14075c0a8: 4d 8b 1f                    	movq	(%r15), %r11
14075c0ab: 89 b5 88 00 00 00           	movl	%esi, 0x88(%rbp)
14075c0b1: 4c 89 fe                    	movq	%r15, %rsi
14075c0b4: 49 89 df                    	movq	%rbx, %r15
14075c0b7: f3 41 0f bc fa              	tzcntl	%r10d, %edi
14075c0bc: 4c 01 c7                    	addq	%r8, %rdi
14075c0bf: 48 21 d7                    	andq	%rdx, %rdi
14075c0c2: 48 89 fb                    	movq	%rdi, %rbx
14075c0c5: 48 c1 e3 04                 	shlq	$0x4, %rbx
14075c0c9: 4c 89 c8                    	movq	%r9, %rax
14075c0cc: 48 29 d8                    	subq	%rbx, %rax
14075c0cf: 4c 3b 18                    	cmpq	(%rax), %r11
14075c0d2: 74 5c                       	je	0x14075c130 <.text+0x75b130>
14075c0d4: 41 8d 42 ff                 	leal	-0x1(%r10), %eax
14075c0d8: 66 44 21 d0                 	andw	%r10w, %ax
14075c0dc: 41 89 c2                    	movl	%eax, %r10d
14075c0df: 48 8b bd 00 01 00 00        	movq	0x100(%rbp), %rdi
14075c0e6: 4c 89 fb                    	movq	%r15, %rbx
14075c0e9: 49 89 f7                    	movq	%rsi, %r15
14075c0ec: 8b b5 88 00 00 00           	movl	0x88(%rbp), %esi
14075c0f2: 75 bd                       	jne	0x14075c0b1 <.text+0x75b0b1>
14075c0f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14075c100: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
14075c105: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
14075c109: 85 c0                       	testl	%eax, %eax
14075c10b: 0f 85 df 02 00 00           	jne	0x14075c3f0 <.text+0x75b3f0>
14075c111: 48 8b 45 68                 	movq	0x68(%rbp), %rax
14075c115: 49 01 c0                    	addq	%rax, %r8
14075c118: 49 83 c0 10                 	addq	$0x10, %r8
14075c11c: 48 83 c0 10                 	addq	$0x10, %rax
14075c120: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075c124: 49 21 d0                    	andq	%rdx, %r8
14075c127: e9 64 ff ff ff              	jmp	0x14075c090 <.text+0x75b090>
14075c12c: 0f 1f 40 00                 	nopl	(%rax)
14075c130: 48 f7 df                    	negq	%rdi
14075c133: 48 c1 e7 04                 	shlq	$0x4, %rdi
14075c137: f3 0f 10 44 39 f8           	movss	-0x8(%rcx,%rdi), %xmm0
14075c13d: 66 41 0f 6e 4d f0           	movd	-0x10(%r13), %xmm1
14075c143: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14075c147: f3 0f 10 50 28              	movss	0x28(%rax), %xmm2
14075c14c: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c153: 66 0f 6f de                 	movdqa	%xmm6, %xmm3
14075c157: e8 84 43 53 00              	callq	0x140c904e0 <.text+0xc8f4e0>
14075c15c: 4c 89 ff                    	movq	%r15, %rdi
14075c15f: 49 89 f7                    	movq	%rsi, %r15
14075c162: 8b b5 88 00 00 00           	movl	0x88(%rbp), %esi
14075c168: 0f 28 f8                    	movaps	%xmm0, %xmm7
14075c16b: f3 45 0f 10 45 f0           	movss	-0x10(%r13), %xmm8
14075c171: f3 41 0f 5c c0              	subss	%xmm8, %xmm0
14075c176: 41 0f 54 c2                 	andps	%xmm10, %xmm0
14075c17a: 44 0f 2e d8                 	ucomiss	%xmm0, %xmm11
14075c17e: 0f 87 18 01 00 00           	ja	0x14075c29c <.text+0x75b29c>
14075c184: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c18b: 48 89 f9                    	movq	%rdi, %rcx
14075c18e: e8 6d e4 3c 00              	callq	0x140b2a600 <.text+0xb29600>
14075c193: 48 85 c0                    	testq	%rax, %rax
14075c196: 74 11                       	je	0x14075c1a9 <.text+0x75b1a9>
14075c198: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c19f: 48 89 c1                    	movq	%rax, %rcx
14075c1a2: e8 f9 30 aa 00              	callq	0x1411ff2a0 <.text+0x11fe2a0>
14075c1a7: eb 07                       	jmp	0x14075c1b0 <.text+0x75b1b0>
14075c1a9: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075c1ac: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14075c1b0: f3 0f 11 45 38              	movss	%xmm0, 0x38(%rbp)
14075c1b5: 66 0f 7e 4d 3c              	movd	%xmm1, 0x3c(%rbp)
14075c1ba: f3 45 0f 10 45 e0           	movss	-0x20(%r13), %xmm8
14075c1c0: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c1c7: 48 89 f9                    	movq	%rdi, %rcx
14075c1ca: e8 51 e4 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075c1cf: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
14075c1d4: f3 45 0f 59 c4              	mulss	%xmm12, %xmm8
14075c1d9: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c1e0: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14075c1e7: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
14075c1eb: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14075c1ef: e8 ec 0b 54 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14075c1f4: 49 8b 17                    	movq	(%r15), %rdx
14075c1f7: 8b 85 98 00 00 00           	movl	0x98(%rbp), %eax
14075c1fd: 89 45 18                    	movl	%eax, 0x18(%rbp)
14075c200: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14075c207: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14075c20b: 8b 45 04                    	movl	0x4(%rbp), %eax
14075c20e: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
14075c212: 89 01                       	movl	%eax, (%rcx)
14075c214: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14075c218: 48 8b 58 30                 	movq	0x30(%rax), %rbx
14075c21c: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14075c220: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
14075c224: 75 1f                       	jne	0x14075c245 <.text+0x75b245>
14075c226: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c22d: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14075c231: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
14075c235: 48 8d 15 8c 02 4c 02        	leaq	0x24c028c(%rip), %rdx   # 0x142c1c4c8
14075c23c: e8 6f 68 e0 ff              	callq	0x140562ab0 <.text+0x561ab0>
14075c241: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
14075c245: 4c 8b 4d f8                 	movq	-0x8(%rbp), %r9
14075c249: 49 8b 41 28                 	movq	0x28(%r9), %rax
14075c24d: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
14075c251: 48 89 3c c8                 	movq	%rdi, (%rax,%rcx,8)
14075c255: 48 89 54 c8 08              	movq	%rdx, 0x8(%rax,%rcx,8)
14075c25a: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
14075c25e: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
14075c263: 8b 55 18                    	movl	0x18(%rbp), %edx
14075c266: 89 54 c8 18                 	movl	%edx, 0x18(%rax,%rcx,8)
14075c26a: c7 44 c8 1c 00 00 80 3f     	movl	$0x3f800000, 0x1c(%rax,%rcx,8) # imm = 0x3F800000
14075c272: c6 44 c8 20 09              	movb	$0x9, 0x20(%rax,%rcx,8)
14075c277: 8b 95 90 00 00 00           	movl	0x90(%rbp), %edx
14075c27d: 44 8b 85 93 00 00 00        	movl	0x93(%rbp), %r8d
14075c284: 44 89 44 c8 24              	movl	%r8d, 0x24(%rax,%rcx,8)
14075c289: 89 54 c8 21                 	movl	%edx, 0x21(%rax,%rcx,8)
14075c28d: 48 ff c3                    	incq	%rbx
14075c290: 49 89 59 30                 	movq	%rbx, 0x30(%r9)
14075c294: 49 ff 41 40                 	incq	0x40(%r9)
14075c298: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
14075c29c: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14075c2a3: 8b 8d f8 00 00 00           	movl	0xf8(%rbp), %ecx
14075c2a9: 89 08                       	movl	%ecx, (%rax)
14075c2ab: 49 8b 17                    	movq	(%r15), %rdx
14075c2ae: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c2b5: 48 8b 8d 00 01 00 00        	movq	0x100(%rbp), %rcx
14075c2bc: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14075c2c0: e8 0b 08 aa 00              	callq	0x1411fcad0 <.text+0x11fbad0>
14075c2c5: 45 0f 2e 45 f0              	ucomiss	-0x10(%r13), %xmm8
14075c2ca: 89 f0                       	movl	%esi, %eax
14075c2cc: 75 02                       	jne	0x14075c2d0 <.text+0x75b2d0>
14075c2ce: 7b 35                       	jnp	0x14075c305 <.text+0x75b305>
14075c2d0: 49 8b 1f                    	movq	(%r15), %rbx
14075c2d3: 48 8b 7d 60                 	movq	0x60(%rbp), %rdi
14075c2d7: 48 3b 7d 50                 	cmpq	0x50(%rbp), %rdi
14075c2db: 75 17                       	jne	0x14075c2f4 <.text+0x75b2f4>
14075c2dd: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c2e4: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
14075c2e8: 48 8d 15 31 0e 32 02        	leaq	0x2320e31(%rip), %rdx   # 0x142a7d120
14075c2ef: e8 7c 65 e0 ff              	callq	0x140562870 <.text+0x561870>
14075c2f4: 48 8b 45 58                 	movq	0x58(%rbp), %rax
14075c2f8: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14075c2fc: 48 ff c7                    	incq	%rdi
14075c2ff: 48 89 7d 60                 	movq	%rdi, 0x60(%rbp)
14075c303: 89 f0                       	movl	%esi, %eax
14075c305: 48 8b 95 08 01 00 00        	movq	0x108(%rbp), %rdx
14075c30c: 48 85 d2                    	testq	%rdx, %rdx
14075c30f: 0f 84 0d 01 00 00           	je	0x14075c422 <.text+0x75b422>
14075c315: 66 85 c0                    	testw	%ax, %ax
14075c318: 74 16                       	je	0x14075c330 <.text+0x75b330>
14075c31a: 8d 70 ff                    	leal	-0x1(%rax), %esi
14075c31d: 21 c6                       	andl	%eax, %esi
14075c31f: 48 8b bd 00 01 00 00        	movq	0x100(%rbp), %rdi
14075c326: eb 41                       	jmp	0x14075c369 <.text+0x75b369>
14075c328: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14075c330: 48 8b bd 00 01 00 00        	movq	0x100(%rbp), %rdi
14075c337: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14075c340: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
14075c346: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14075c34a: 49 81 c6 80 d5 ff ff        	addq	$-0x2a80, %r14          # imm = 0xD580
14075c351: 49 83 c4 10                 	addq	$0x10, %r12
14075c355: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14075c35a: 74 e4                       	je	0x14075c340 <.text+0x75b340>
14075c35c: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14075c361: 29 c1                       	subl	%eax, %ecx
14075c363: f7 d0                       	notl	%eax
14075c365: 89 c6                       	movl	%eax, %esi
14075c367: 21 ce                       	andl	%ecx, %esi
14075c369: 48 ff ca                    	decq	%rdx
14075c36c: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
14075c373: f3 0f bc c0                 	tzcntl	%eax, %eax
14075c377: 48 f7 d8                    	negq	%rax
14075c37a: 48 69 c0 a8 02 00 00        	imulq	$0x2a8, %rax, %rax      # imm = 0x2A8
14075c381: 4d 8d 2c 06                 	leaq	(%r14,%rax), %r13
14075c385: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
14075c38a: 74 64                       	je	0x14075c3f0 <.text+0x75b3f0>
14075c38c: 4d 8d 3c 06                 	leaq	(%r14,%rax), %r15
14075c390: 49 81 c7 58 fd ff ff        	addq	$-0x2a8, %r15           # imm = 0xFD58
14075c397: 49 8d 1c 06                 	leaq	(%r14,%rax), %rbx
14075c39b: 48 81 c3 60 fd ff ff        	addq	$-0x2a0, %rbx           # imm = 0xFD60
14075c3a2: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
14075c3a6: 4c 89 fa                    	movq	%r15, %rdx
14075c3a9: e8 c2 49 e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14075c3ae: 48 8b 0f                    	movq	(%rdi), %rcx
14075c3b1: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
14075c3b5: 49 89 d0                    	movq	%rdx, %r8
14075c3b8: 49 21 c0                    	andq	%rax, %r8
14075c3bb: 48 c1 e8 39                 	shrq	$0x39, %rax
14075c3bf: 66 0f 6e c0                 	movd	%eax, %xmm0
14075c3c3: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14075c3c7: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14075c3cc: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14075c3d1: 4c 8d 49 f0                 	leaq	-0x10(%rcx), %r9
14075c3d5: 48 c7 45 68 00 00 00 00     	movq	$0x0, 0x68(%rbp)
14075c3dd: e9 ae fc ff ff              	jmp	0x14075c090 <.text+0x75b090>
14075c3e2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14075c3f0: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14075c3f7: 8b 8d f8 00 00 00           	movl	0xf8(%rbp), %ecx
14075c3fd: 89 08                       	movl	%ecx, (%rax)
14075c3ff: 49 8b 95 58 fd ff ff        	movq	-0x2a8(%r13), %rdx
14075c406: f3 41 0f 10 55 f0           	movss	-0x10(%r13), %xmm2
14075c40c: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c413: 48 89 f9                    	movq	%rdi, %rcx
14075c416: e8 b5 06 aa 00              	callq	0x1411fcad0 <.text+0x11fbad0>
14075c41b: 89 f0                       	movl	%esi, %eax
14075c41d: e9 e3 fe ff ff              	jmp	0x14075c305 <.text+0x75b305>
14075c422: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14075c426: 48 8b 10                    	movq	(%rax), %rdx
14075c429: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c430: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075c434: e8 c7 ec 2b 00              	callq	0x140a1b100 <.text+0xa1a100>
14075c439: 48 8b 45 30                 	movq	0x30(%rbp), %rax
14075c43d: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
14075c444: f3 0f 6f 45 10              	movdqu	0x10(%rbp), %xmm0
14075c449: f3 0f 6f 4d 20              	movdqu	0x20(%rbp), %xmm1
14075c44e: 66 0f 7f 8d a0 00 00 00     	movdqa	%xmm1, 0xa0(%rbp)
14075c456: 66 0f 7f 85 90 00 00 00     	movdqa	%xmm0, 0x90(%rbp)
14075c45e: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075c465: 4c 8d 05 04 0a 32 02        	leaq	0x2320a04(%rip), %r8    # 0x142a7ce70
14075c46c: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
14075c470: 48 8d 95 90 00 00 00        	leaq	0x90(%rbp), %rdx
14075c477: e8 84 96 aa 00              	callq	0x141205b00 <.text+0x1204b00>
14075c47c: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14075c483: 8b 8d f8 00 00 00           	movl	0xf8(%rbp), %ecx
14075c489: 89 08                       	movl	%ecx, (%rax)
14075c48b: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14075c492: 48 8b 40 18                 	movq	0x18(%rax), %rax
14075c496: 48 85 c0                    	testq	%rax, %rax
14075c499: 0f 84 48 01 00 00           	je	0x14075c5e7 <.text+0x75b5e7>
14075c49f: 48 8b b5 00 01 00 00        	movq	0x100(%rbp), %rsi
14075c4a6: 48 8b 0e                    	movq	(%rsi), %rcx
14075c4a9: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
14075c4ad: 4c 8d 41 10                 	leaq	0x10(%rcx), %r8
14075c4b1: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
14075c4b5: 66 44 0f d7 d8              	pmovmskb	%xmm0, %r11d
14075c4ba: 41 f7 d3                    	notl	%r11d
14075c4bd: 4c 8b 4d 40                 	movq	0x40(%rbp), %r9
14075c4c1: 4c 8b 55 48                 	movq	0x48(%rbp), %r10
14075c4c5: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
14075c4c9: 49 c1 e2 03                 	shlq	$0x3, %r10
14075c4cd: 66 0f 76 c0                 	pcmpeqd	%xmm0, %xmm0
14075c4d1: 49 89 c6                    	movq	%rax, %r14
14075c4d4: 49 89 cf                    	movq	%rcx, %r15
14075c4d7: eb 31                       	jmp	0x14075c50a <.text+0x75b50a>
14075c4d9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14075c4e0: 48 ff c7                    	incq	%rdi
14075c4e3: 4c 8b ad 00 01 00 00        	movq	0x100(%rbp), %r13
14075c4ea: 49 89 7d 10                 	movq	%rdi, 0x10(%r13)
14075c4ee: 40 b6 ff                    	movb	$-0x1, %sil
14075c4f1: 40 88 34 19                 	movb	%sil, (%rcx,%rbx)
14075c4f5: 42 88 74 21 10              	movb	%sil, 0x10(%rcx,%r12)
14075c4fa: 48 ff c8                    	decq	%rax
14075c4fd: 49 89 45 18                 	movq	%rax, 0x18(%r13)
14075c501: 4d 85 f6                    	testq	%r14, %r14
14075c504: 0f 84 dd 00 00 00           	je	0x14075c5e7 <.text+0x75b5e7>
14075c50a: 66 45 85 db                 	testw	%r11w, %r11w
14075c50e: 75 21                       	jne	0x14075c531 <.text+0x75b531>
14075c510: 66 41 0f 6f 08              	movdqa	(%r8), %xmm1
14075c515: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
14075c51a: 49 81 c7 00 ff ff ff        	addq	$-0x100, %r15
14075c521: 49 83 c0 10                 	addq	$0x10, %r8
14075c525: 41 81 fb ff ff 00 00        	cmpl	$0xffff, %r11d          # imm = 0xFFFF
14075c52c: 74 e2                       	je	0x14075c510 <.text+0x75b510>
14075c52e: 41 f7 d3                    	notl	%r11d
14075c531: 41 8d 73 ff                 	leal	-0x1(%r11), %esi
14075c535: f3 41 0f bc db              	tzcntl	%r11d, %ebx
14075c53a: 44 21 de                    	andl	%r11d, %esi
14075c53d: 41 89 f3                    	movl	%esi, %r11d
14075c540: c1 e3 04                    	shll	$0x4, %ebx
14075c543: 4d 89 fc                    	movq	%r15, %r12
14075c546: 49 29 dc                    	subq	%rbx, %r12
14075c549: 49 ff ce                    	decq	%r14
14075c54c: 49 8b 74 24 f0              	movq	-0x10(%r12), %rsi
14075c551: 31 db                       	xorl	%ebx, %ebx
14075c553: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14075c560: 49 39 da                    	cmpq	%rbx, %r10
14075c563: 74 1b                       	je	0x14075c580 <.text+0x75b580>
14075c565: 4c 8d 6b 08                 	leaq	0x8(%rbx), %r13
14075c569: 49 3b 34 19                 	cmpq	(%r9,%rbx), %rsi
14075c56d: 4c 89 eb                    	movq	%r13, %rbx
14075c570: 75 ee                       	jne	0x14075c560 <.text+0x75b560>
14075c572: eb 8d                       	jmp	0x14075c501 <.text+0x75b501>
14075c574: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14075c580: 48 89 cb                    	movq	%rcx, %rbx
14075c583: 4c 29 e3                    	subq	%r12, %rbx
14075c586: 48 c1 fb 04                 	sarq	$0x4, %rbx
14075c58a: 4c 8d 63 f0                 	leaq	-0x10(%rbx), %r12
14075c58e: 49 21 d4                    	andq	%rdx, %r12
14075c591: f3 42 0f 6f 0c 21           	movdqu	(%rcx,%r12), %xmm1
14075c597: 66 0f 74 c8                 	pcmpeqb	%xmm0, %xmm1
14075c59b: 66 0f d7 f1                 	pmovmskb	%xmm1, %esi
14075c59f: 85 f6                       	testl	%esi, %esi
14075c5a1: 74 0b                       	je	0x14075c5ae <.text+0x75b5ae>
14075c5a3: 66 44 0f bd ee              	bsrw	%si, %r13w
14075c5a8: 41 83 f5 0f                 	xorl	$0xf, %r13d
14075c5ac: eb 05                       	jmp	0x14075c5b3 <.text+0x75b5b3>
14075c5ae: 66 41 bd 10 00              	movw	$0x10, %r13w
14075c5b3: f3 0f 6f 0c 19              	movdqu	(%rcx,%rbx), %xmm1
14075c5b8: 66 0f 74 c8                 	pcmpeqb	%xmm0, %xmm1
14075c5bc: 66 0f d7 f1                 	pmovmskb	%xmm1, %esi
14075c5c0: 81 ce 00 00 01 00           	orl	$0x10000, %esi          # imm = 0x10000
14075c5c6: f3 0f bc f6                 	tzcntl	%esi, %esi
14075c5ca: 41 01 f5                    	addl	%esi, %r13d
14075c5cd: 40 b6 80                    	movb	$-0x80, %sil
14075c5d0: 66 41 83 fd 0f              	cmpw	$0xf, %r13w
14075c5d5: 0f 86 05 ff ff ff           	jbe	0x14075c4e0 <.text+0x75b4e0>
14075c5db: 4c 8b ad 00 01 00 00        	movq	0x100(%rbp), %r13
14075c5e2: e9 0a ff ff ff              	jmp	0x14075c4f1 <.text+0x75b4f1>
14075c5e7: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
14075c5eb: 48 85 d2                    	testq	%rdx, %rdx
14075c5ee: 74 13                       	je	0x14075c603 <.text+0x75b603>
14075c5f0: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14075c5f4: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075c5f8: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075c5fe: e8 1d 76 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075c603: 4c 8b 75 58                 	movq	0x58(%rbp), %r14
14075c607: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14075c60b: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
14075c60f: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14075c616: 48 83 fa 01                 	cmpq	$0x1, %rdx
14075c61a: 0f 87 5c 06 00 00           	ja	0x14075cc7c <.text+0x75bc7c>
14075c620: 48 8b 45 78                 	movq	0x78(%rbp), %rax
14075c624: 4c 8b 45 50                 	movq	0x50(%rbp), %r8
14075c628: 4c 89 85 f0 00 00 00        	movq	%r8, 0xf0(%rbp)
14075c62f: 49 8d 14 d6                 	leaq	(%r14,%rdx,8), %rdx
14075c633: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
14075c63a: 48 8b 18                    	movq	(%rax), %rbx
14075c63d: 48 8d 43 20                 	leaq	0x20(%rbx), %rax
14075c641: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14075c648: 49 bc 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r12 # imm = 0x8000000000000001
14075c652: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
14075c656: 49 bf 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r15 # imm = 0x8000000000000000
14075c660: 4c 89 b5 f8 00 00 00        	movq	%r14, 0xf8(%rbp)
14075c667: 48 89 9d 00 01 00 00        	movq	%rbx, 0x100(%rbp)
14075c66e: 4c 3b b5 08 01 00 00        	cmpq	0x108(%rbp), %r14
14075c675: 0f 84 83 04 00 00           	je	0x14075cafe <.text+0x75bafe>
14075c67b: 49 8b 06                    	movq	(%r14), %rax
14075c67e: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075c682: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14075c686: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14075c689: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14075c68d: 89 01                       	movl	%eax, (%rcx)
14075c68f: 48 8b 32                    	movq	(%rdx), %rsi
14075c692: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14075c696: 48 8d 7d 38                 	leaq	0x38(%rbp), %rdi
14075c69a: 48 89 fa                    	movq	%rdi, %rdx
14075c69d: e8 ce 46 e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14075c6a2: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075c6a6: 48 89 f2                    	movq	%rsi, %rdx
14075c6a9: 49 89 c0                    	movq	%rax, %r8
14075c6ac: 49 89 f9                    	movq	%rdi, %r9
14075c6af: e8 3c b1 41 00              	callq	0x140b777f0 <.text+0xb767f0>
14075c6b4: 49 83 c6 08                 	addq	$0x8, %r14
14075c6b8: 48 8b 5d 18                 	movq	0x18(%rbp), %rbx
14075c6bc: 48 89 d8                    	movq	%rbx, %rax
14075c6bf: 48 f7 d8                    	negq	%rax
14075c6c2: 70 aa                       	jo	0x14075c66e <.text+0x75b66e>
14075c6c4: 48 8d 45 20                 	leaq	0x20(%rbp), %rax
14075c6c8: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14075c6cc: 48 8d 85 98 00 00 00        	leaq	0x98(%rbp), %rax
14075c6d3: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14075c6d7: 48 89 9d 90 00 00 00        	movq	%rbx, 0x90(%rbp)
14075c6de: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14075c6e9: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
14075c6f4: 31 f6                       	xorl	%esi, %esi
14075c6f6: 31 ff                       	xorl	%edi, %edi
14075c6f8: 48 85 ff                    	testq	%rdi, %rdi
14075c6fb: 75 43                       	jne	0x14075c740 <.text+0x75b740>
14075c6fd: eb 2c                       	jmp	0x14075c72b <.text+0x75b72b>
14075c6ff: 90                          	nop
14075c700: 48 8d 04 de                 	leaq	(%rsi,%rbx,8), %rax
14075c704: 48 89 b5 a8 00 00 00        	movq	%rsi, 0xa8(%rbp)
14075c70b: 48 89 b5 b0 00 00 00        	movq	%rsi, 0xb0(%rbp)
14075c712: 48 89 bd b8 00 00 00        	movq	%rdi, 0xb8(%rbp)
14075c719: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14075c720: 4c 89 fb                    	movq	%r15, %rbx
14075c723: 48 89 f7                    	movq	%rsi, %rdi
14075c726: 48 85 ff                    	testq	%rdi, %rdi
14075c729: 75 15                       	jne	0x14075c740 <.text+0x75b740>
14075c72b: 49 89 f5                    	movq	%rsi, %r13
14075c72e: 48 89 df                    	movq	%rbx, %rdi
14075c731: 4c 89 e3                    	movq	%r12, %rbx
14075c734: 4c 39 e7                    	cmpq	%r12, %rdi
14075c737: 75 6d                       	jne	0x14075c7a6 <.text+0x75b7a6>
14075c739: eb 7d                       	jmp	0x14075c7b8 <.text+0x75b7b8>
14075c73b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14075c740: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14075c747: 48 3b 85 c0 00 00 00        	cmpq	0xc0(%rbp), %rax
14075c74e: 74 1b                       	je	0x14075c76b <.text+0x75b76b>
14075c750: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
14075c754: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
14075c75b: 48 8b 00                    	movq	(%rax), %rax
14075c75e: 48 89 c1                    	movq	%rax, %rcx
14075c761: 48 c1 e9 20                 	shrq	$0x20, %rcx
14075c765: 0f 85 d5 00 00 00           	jne	0x14075c840 <.text+0x75b840>
14075c76b: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14075c772: 48 85 d2                    	testq	%rdx, %rdx
14075c775: 74 12                       	je	0x14075c789 <.text+0x75b789>
14075c777: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075c77b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075c781: 48 89 f9                    	movq	%rdi, %rcx
14075c784: e8 97 74 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075c789: 48 c7 85 a8 00 00 00 00 00 00 00    	movq	$0x0, 0xa8(%rbp)
14075c794: 48 8b bd 90 00 00 00        	movq	0x90(%rbp), %rdi
14075c79b: 45 31 ed                    	xorl	%r13d, %r13d
14075c79e: 4c 89 e3                    	movq	%r12, %rbx
14075c7a1: 4c 39 e7                    	cmpq	%r12, %rdi
14075c7a4: 74 12                       	je	0x14075c7b8 <.text+0x75b7b8>
14075c7a6: 4c 89 bd 90 00 00 00        	movq	%r15, 0x90(%rbp)
14075c7ad: 48 89 f8                    	movq	%rdi, %rax
14075c7b0: 48 f7 d8                    	negq	%rax
14075c7b3: 4c 89 fb                    	movq	%r15, %rbx
14075c7b6: 71 48                       	jno	0x14075c800 <.text+0x75b800>
14075c7b8: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
14075c7bf: 48 85 c9                    	testq	%rcx, %rcx
14075c7c2: 0f 84 ee 00 00 00           	je	0x14075c8b6 <.text+0x75b8b6>
14075c7c8: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14075c7cf: 48 3b 85 e0 00 00 00        	cmpq	0xe0(%rbp), %rax
14075c7d6: 0f 84 b4 00 00 00           	je	0x14075c890 <.text+0x75b890>
14075c7dc: 48 8d 50 08                 	leaq	0x8(%rax), %rdx
14075c7e0: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
14075c7e7: 48 8b 00                    	movq	(%rax), %rax
14075c7ea: 48 89 c2                    	movq	%rax, %rdx
14075c7ed: 48 c1 ea 20                 	shrq	$0x20, %rdx
14075c7f1: 0f 84 99 00 00 00           	je	0x14075c890 <.text+0x75b890>
14075c7f7: 31 ff                       	xorl	%edi, %edi
14075c7f9: eb 48                       	jmp	0x14075c843 <.text+0x75b843>
14075c7fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14075c800: 48 8b b5 98 00 00 00        	movq	0x98(%rbp), %rsi
14075c807: 48 8b 9d a0 00 00 00        	movq	0xa0(%rbp), %rbx
14075c80e: 4d 85 ed                    	testq	%r13, %r13
14075c811: 0f 84 e9 fe ff ff           	je	0x14075c700 <.text+0x75b700>
14075c817: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14075c81e: 48 85 d2                    	testq	%rdx, %rdx
14075c821: 0f 84 d9 fe ff ff           	je	0x14075c700 <.text+0x75b700>
14075c827: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075c82b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075c831: 4c 89 e9                    	movq	%r13, %rcx
14075c834: e8 e7 73 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075c839: e9 c2 fe ff ff              	jmp	0x14075c700 <.text+0x75b700>
14075c83e: 66 90                       	nop
14075c840: 49 89 f5                    	movq	%rsi, %r13
14075c843: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
14075c84a: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
14075c84e: f3 0f 6f 49 10              	movdqu	0x10(%rcx), %xmm1
14075c853: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14075c857: f3 0f 7f 45 10              	movdqu	%xmm0, 0x10(%rbp)
14075c85c: f3 0f 7f 4d 20              	movdqu	%xmm1, 0x20(%rbp)
14075c861: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075c865: 48 8d 15 9c 08 32 02        	leaq	0x232089c(%rip), %rdx   # 0x142a7d108
14075c86c: e8 ff 7e 5b 00              	callq	0x140d14770 <.text+0xd13770>
14075c871: 4c 89 ee                    	movq	%r13, %rsi
14075c874: 48 85 ff                    	testq	%rdi, %rdi
14075c877: 0f 85 c3 fe ff ff           	jne	0x14075c740 <.text+0x75b740>
14075c87d: e9 a9 fe ff ff              	jmp	0x14075c72b <.text+0x75b72b>
14075c882: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14075c890: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
14075c897: 48 85 d2                    	testq	%rdx, %rdx
14075c89a: 74 0f                       	je	0x14075c8ab <.text+0x75b8ab>
14075c89c: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075c8a0: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075c8a6: e8 75 73 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075c8ab: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
14075c8b6: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
14075c8bd: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
14075c8c7: 48 39 c2                    	cmpq	%rax, %rdx
14075c8ca: 7c 1b                       	jl	0x14075c8e7 <.text+0x75b8e7>
14075c8cc: 48 85 d2                    	testq	%rdx, %rdx
14075c8cf: 74 16                       	je	0x14075c8e7 <.text+0x75b8e7>
14075c8d1: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
14075c8d8: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075c8dc: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075c8e2: e8 39 73 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075c8e7: 4d 85 ed                    	testq	%r13, %r13
14075c8ea: 48 8b 9d 00 01 00 00        	movq	0x100(%rbp), %rbx
14075c8f1: 74 1e                       	je	0x14075c911 <.text+0x75b911>
14075c8f3: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14075c8fa: 48 85 d2                    	testq	%rdx, %rdx
14075c8fd: 74 12                       	je	0x14075c911 <.text+0x75b911>
14075c8ff: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075c903: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075c909: 4c 89 e9                    	movq	%r13, %rcx
14075c90c: e8 0f 73 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075c911: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14075c915: 48 8b 38                    	movq	(%rax), %rdi
14075c918: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
14075c91d: 0f 84 ca 01 00 00           	je	0x14075caed <.text+0x75baed>
14075c923: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
14075c927: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
14075c92b: e8 40 44 e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14075c930: 48 8b 37                    	movq	(%rdi), %rsi
14075c933: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
14075c937: 48 89 ca                    	movq	%rcx, %rdx
14075c93a: 48 21 c2                    	andq	%rax, %rdx
14075c93d: 48 c1 e8 39                 	shrq	$0x39, %rax
14075c941: 66 0f 6e c0                 	movd	%eax, %xmm0
14075c945: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14075c949: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14075c94e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14075c953: 48 8d 86 58 fd ff ff        	leaq	-0x2a8(%rsi), %rax
14075c95a: 4c 8b 45 38                 	movq	0x38(%rbp), %r8
14075c95e: 45 31 c9                    	xorl	%r9d, %r9d
14075c961: f3 0f 6f 0c 16              	movdqu	(%rsi,%rdx), %xmm1
14075c966: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14075c96a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14075c96e: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14075c973: 45 85 d2                    	testl	%r10d, %r10d
14075c976: 74 28                       	je	0x14075c9a0 <.text+0x75b9a0>
14075c978: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14075c97d: 49 01 d3                    	addq	%rdx, %r11
14075c980: 49 21 cb                    	andq	%rcx, %r11
14075c983: 49 f7 db                    	negq	%r11
14075c986: 49 69 fb a8 02 00 00        	imulq	$0x2a8, %r11, %rdi      # imm = 0x2A8
14075c98d: 4c 3b 04 38                 	cmpq	(%rax,%rdi), %r8
14075c991: 74 3d                       	je	0x14075c9d0 <.text+0x75b9d0>
14075c993: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14075c997: 66 45 21 d3                 	andw	%r10w, %r11w
14075c99b: 45 89 da                    	movl	%r11d, %r10d
14075c99e: 75 d8                       	jne	0x14075c978 <.text+0x75b978>
14075c9a0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
14075c9a4: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
14075c9a9: 45 85 d2                    	testl	%r10d, %r10d
14075c9ac: 0f 85 3b 01 00 00           	jne	0x14075caed <.text+0x75baed>
14075c9b2: 4c 01 ca                    	addq	%r9, %rdx
14075c9b5: 48 83 c2 10                 	addq	$0x10, %rdx
14075c9b9: 49 83 c1 10                 	addq	$0x10, %r9
14075c9bd: 48 21 ca                    	andq	%rcx, %rdx
14075c9c0: eb 9f                       	jmp	0x14075c961 <.text+0x75b961>
14075c9c2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14075c9d0: 48 8d 0c 3e                 	leaq	(%rsi,%rdi), %rcx
14075c9d4: 48 83 c1 f6                 	addq	$-0xa, %rcx
14075c9d8: e8 a3 8c 16 00              	callq	0x1408c5680 <.text+0x8c4680>
14075c9dd: 84 c0                       	testb	%al, %al
14075c9df: 0f 85 89 fc ff ff           	jne	0x14075c66e <.text+0x75b66e>
14075c9e5: 48 83 7b 18 00              	cmpq	$0x0, 0x18(%rbx)
14075c9ea: 0f 84 99 00 00 00           	je	0x14075ca89 <.text+0x75ba89>
14075c9f0: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14075c9f7: 48 8d 55 38                 	leaq	0x38(%rbp), %rdx
14075c9fb: e8 70 43 e5 ff              	callq	0x1405b0d70 <.text+0x5afd70>
14075ca00: 48 8b 0b                    	movq	(%rbx), %rcx
14075ca03: 48 8b 53 08                 	movq	0x8(%rbx), %rdx
14075ca07: 49 89 d0                    	movq	%rdx, %r8
14075ca0a: 49 21 c0                    	andq	%rax, %r8
14075ca0d: 48 c1 e8 39                 	shrq	$0x39, %rax
14075ca11: 66 0f 6e c0                 	movd	%eax, %xmm0
14075ca15: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14075ca19: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14075ca1e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14075ca23: 48 8d 41 98                 	leaq	-0x68(%rcx), %rax
14075ca27: 4c 8b 4d 38                 	movq	0x38(%rbp), %r9
14075ca2b: 45 31 d2                    	xorl	%r10d, %r10d
14075ca2e: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14075ca34: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14075ca38: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14075ca3c: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
14075ca41: 45 85 db                    	testl	%r11d, %r11d
14075ca44: 74 25                       	je	0x14075ca6b <.text+0x75ba6b>
14075ca46: f3 41 0f bc db              	tzcntl	%r11d, %ebx
14075ca4b: 4c 01 c3                    	addq	%r8, %rbx
14075ca4e: 48 21 d3                    	andq	%rdx, %rbx
14075ca51: 48 f7 db                    	negq	%rbx
14075ca54: 48 6b db 68                 	imulq	$0x68, %rbx, %rbx
14075ca58: 4c 3b 0c 18                 	cmpq	(%rax,%rbx), %r9
14075ca5c: 74 2f                       	je	0x14075ca8d <.text+0x75ba8d>
14075ca5e: 41 8d 5b ff                 	leal	-0x1(%r11), %ebx
14075ca62: 66 44 21 db                 	andw	%r11w, %bx
14075ca66: 41 89 db                    	movl	%ebx, %r11d
14075ca69: 75 db                       	jne	0x14075ca46 <.text+0x75ba46>
14075ca6b: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
14075ca6f: 66 44 0f d7 d9              	pmovmskb	%xmm1, %r11d
14075ca74: 45 85 db                    	testl	%r11d, %r11d
14075ca77: 75 10                       	jne	0x14075ca89 <.text+0x75ba89>
14075ca79: 4d 01 d0                    	addq	%r10, %r8
14075ca7c: 49 83 c0 10                 	addq	$0x10, %r8
14075ca80: 49 83 c2 10                 	addq	$0x10, %r10
14075ca84: 49 21 d0                    	andq	%rdx, %r8
14075ca87: eb a5                       	jmp	0x14075ca2e <.text+0x75ba2e>
14075ca89: 31 c0                       	xorl	%eax, %eax
14075ca8b: eb 14                       	jmp	0x14075caa1 <.text+0x75baa1>
14075ca8d: 48 01 d9                    	addq	%rbx, %rcx
14075ca90: 48 83 c1 a0                 	addq	$-0x60, %rcx
14075ca94: e8 17 a4 82 00              	callq	0x140f86eb0 <.text+0xf85eb0>
14075ca99: 48 89 c1                    	movq	%rax, %rcx
14075ca9c: e8 2f a2 82 00              	callq	0x140f86cd0 <.text+0xf85cd0>
14075caa1: 48 8d 0c 3e                 	leaq	(%rsi,%rdi), %rcx
14075caa5: 48 81 c1 60 fd ff ff        	addq	$-0x2a0, %rcx           # imm = 0xFD60
14075caac: 48 89 54 24 40              	movq	%rdx, 0x40(%rsp)
14075cab1: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
14075cab6: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
14075cabb: 48 8d 45 38                 	leaq	0x38(%rbp), %rax
14075cabf: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14075cac4: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
14075cacb: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14075cad0: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
14075cad4: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
14075cad8: 4c 8b 45 70                 	movq	0x70(%rbp), %r8
14075cadc: 4c 8b 8d 00 02 00 00        	movq	0x200(%rbp), %r9
14075cae3: e8 c8 9e ac 00              	callq	0x1412269b0 <.text+0x12259b0>
14075cae8: e9 81 fb ff ff              	jmp	0x14075c66e <.text+0x75b66e>
14075caed: 48 8d 0d fc 05 32 02        	leaq	0x23205fc(%rip), %rcx   # 0x142a7d0f0
14075caf4: e8 17 c8 17 02              	callq	0x1428d9310 <.text+0x28d8310>
14075caf9: e9 6b 01 00 00              	jmp	0x14075cc69 <.text+0x75bc69>
14075cafe: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
14075cb05: 48 85 d2                    	testq	%rdx, %rdx
14075cb08: 74 16                       	je	0x14075cb20 <.text+0x75bb20>
14075cb0a: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075cb0e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075cb14: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14075cb1b: e8 00 71 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075cb20: 80 bd 16 01 00 00 00        	cmpb	$0x0, 0x116(%rbp)
14075cb27: 74 4b                       	je	0x14075cb74 <.text+0x75bb74>
14075cb29: e8 02 b6 4b 00              	callq	0x140c18130 <.text+0xc17130>
14075cb2e: 48 89 c1                    	movq	%rax, %rcx
14075cb31: 48 8b 00                    	movq	(%rax), %rax
14075cb34: 48 83 f8 01                 	cmpq	$0x1, %rax
14075cb38: 0f 85 9a 00 00 00           	jne	0x14075cbd8 <.text+0x75bbd8>
14075cb3e: 48 83 c1 08                 	addq	$0x8, %rcx
14075cb42: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14075cb49: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14075cb4d: 0f 85 7e 01 00 00           	jne	0x14075ccd1 <.text+0x75bcd1>
14075cb53: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14075cb5a: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14075cb61: 48 83 c1 08                 	addq	$0x8, %rcx
14075cb65: e8 86 a5 4b 00              	callq	0x140c170f0 <.text+0xc160f0>
14075cb6a: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14075cb71: 48 ff 00                    	incq	(%rax)
14075cb74: 0f 28 b5 20 01 00 00        	movaps	0x120(%rbp), %xmm6
14075cb7b: 0f 28 bd 30 01 00 00        	movaps	0x130(%rbp), %xmm7
14075cb82: 44 0f 28 85 40 01 00 00     	movaps	0x140(%rbp), %xmm8
14075cb8a: 44 0f 28 8d 50 01 00 00     	movaps	0x150(%rbp), %xmm9
14075cb92: 44 0f 28 95 60 01 00 00     	movaps	0x160(%rbp), %xmm10
14075cb9a: 44 0f 28 9d 70 01 00 00     	movaps	0x170(%rbp), %xmm11
14075cba2: 44 0f 28 a5 80 01 00 00     	movaps	0x180(%rbp), %xmm12
14075cbaa: 48 81 c4 18 02 00 00        	addq	$0x218, %rsp            # imm = 0x218
14075cbb1: 5b                          	popq	%rbx
14075cbb2: 5f                          	popq	%rdi
14075cbb3: 5e                          	popq	%rsi
14075cbb4: 41 5c                       	popq	%r12
14075cbb6: 41 5d                       	popq	%r13
14075cbb8: 41 5e                       	popq	%r14
14075cbba: 41 5f                       	popq	%r15
14075cbbc: 5d                          	popq	%rbp
14075cbbd: c3                          	retq
14075cbbe: 48 85 c0                    	testq	%rax, %rax
14075cbc1: 75 34                       	jne	0x14075cbf7 <.text+0x75bbf7>
14075cbc3: 31 d2                       	xorl	%edx, %edx
14075cbc5: e8 f6 49 19 02              	callq	0x1428f15c0 <.text+0x28f05c0>
14075cbca: 48 89 c1                    	movq	%rax, %rcx
14075cbcd: 48 85 c0                    	testq	%rax, %rax
14075cbd0: 0f 85 84 ee ff ff           	jne	0x14075ba5a <.text+0x75aa5a>
14075cbd6: eb 1f                       	jmp	0x14075cbf7 <.text+0x75bbf7>
14075cbd8: 48 85 c0                    	testq	%rax, %rax
14075cbdb: 75 1a                       	jne	0x14075cbf7 <.text+0x75bbf7>
14075cbdd: 31 d2                       	xorl	%edx, %edx
14075cbdf: e8 dc 49 19 02              	callq	0x1428f15c0 <.text+0x28f05c0>
14075cbe4: 48 89 c1                    	movq	%rax, %rcx
14075cbe7: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14075cbee: 48 85 c0                    	testq	%rax, %rax
14075cbf1: 0f 85 52 ff ff ff           	jne	0x14075cb49 <.text+0x75bb49>
14075cbf7: 48 8d 0d aa c8 4b 02        	leaq	0x24bc8aa(%rip), %rcx   # 0x142c194a8
14075cbfe: e8 cd b4 17 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14075cc03: 48 8d 0d b6 f3 ec 02        	leaq	0x2ecf3b6(%rip), %rcx   # 0x14362bfc0
14075cc0a: e8 41 bb 17 02              	callq	0x1428d8750 <.text+0x28d7750>
14075cc0f: e9 b1 ee ff ff              	jmp	0x14075bac5 <.text+0x75aac5>
14075cc14: e8 97 ba 17 02              	callq	0x1428d86b0 <.text+0x28d76b0>
14075cc19: 34 01                       	xorb	$0x1, %al
14075cc1b: 0f b6 0d 9f f3 ec 02        	movzbl	0x2ecf39f(%rip), %ecx   # 0x14362bfc1
14075cc22: 84 c9                       	testb	%cl, %cl
14075cc24: 0f 84 c2 ee ff ff           	je	0x14075baec <.text+0x75aaec>
14075cc2a: 48 8d 0d 8f f3 ec 02        	leaq	0x2ecf38f(%rip), %rcx   # 0x14362bfc0
14075cc31: 48 89 8d 90 00 00 00        	movq	%rcx, 0x90(%rbp)
14075cc38: 88 85 98 00 00 00           	movb	%al, 0x98(%rbp)
14075cc3e: 48 8d 05 0b 05 32 02        	leaq	0x232050b(%rip), %rax   # 0x142a7d150
14075cc45: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14075cc4a: 48 8d 0d 57 02 32 02        	leaq	0x2320257(%rip), %rcx   # 0x142a7cea8
14075cc51: 4c 8d 0d 30 02 32 02        	leaq	0x2320230(%rip), %r9    # 0x142a7ce88
14075cc58: 4c 8d 85 90 00 00 00        	leaq	0x90(%rbp), %r8
14075cc5f: ba 2b 00 00 00              	movl	$0x2b, %edx
14075cc64: e8 f7 ca 17 02              	callq	0x1428d9760 <.text+0x28d8760>
14075cc69: 0f 0b                       	ud2
14075cc6b: 48 8d 0d 4e f3 ec 02        	leaq	0x2ecf34e(%rip), %rcx   # 0x14362bfc0
14075cc72: e8 99 bb 17 02              	callq	0x1428d8810 <.text+0x28d7810>
14075cc77: e9 d5 ee ff ff              	jmp	0x14075bb51 <.text+0x75ab51>
14075cc7c: 48 83 fa 15                 	cmpq	$0x15, %rdx
14075cc80: 0f 83 a7 00 00 00           	jae	0x14075cd2d <.text+0x75bd2d>
14075cc86: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075cc8d: 4c 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %r9
14075cc94: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14075cc9a: 4c 89 f1                    	movq	%r14, %rcx
14075cc9d: e8 7e fd ac 00              	callq	0x14122ca20 <.text+0x122ba20>
14075cca2: 4c 8b 75 58                 	movq	0x58(%rbp), %r14
14075cca6: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14075ccaa: 48 83 fa 02                 	cmpq	$0x2, %rdx
14075ccae: 0f 82 6c f9 ff ff           	jb	0x14075c620 <.text+0x75b620>
14075ccb4: b8 01 00 00 00              	movl	$0x1, %eax
14075ccb9: 4d 8b 04 c6                 	movq	(%r14,%rax,8), %r8
14075ccbd: 4d 3b 44 c6 f8              	cmpq	-0x8(%r14,%rax,8), %r8
14075ccc2: 74 32                       	je	0x14075ccf6 <.text+0x75bcf6>
14075ccc4: 48 ff c0                    	incq	%rax
14075ccc7: 48 39 c2                    	cmpq	%rax, %rdx
14075ccca: 75 ed                       	jne	0x14075ccb9 <.text+0x75bcb9>
14075cccc: e9 4f f9 ff ff              	jmp	0x14075c620 <.text+0x75b620>
14075ccd1: 48 8d 0d 40 c8 4b 02        	leaq	0x24bc840(%rip), %rcx   # 0x142c19518
14075ccd8: e8 73 c5 17 02              	callq	0x1428d9250 <.text+0x28d8250>
14075ccdd: e8 ce b9 17 02              	callq	0x1428d86b0 <.text+0x28d76b0>
14075cce2: 84 c0                       	testb	%al, %al
14075cce4: 0f 85 57 ee ff ff           	jne	0x14075bb41 <.text+0x75ab41>
14075ccea: c6 05 d0 f2 ec 02 01        	movb	$0x1, 0x2ecf2d0(%rip)   # 0x14362bfc1
14075ccf1: e9 4b ee ff ff              	jmp	0x14075bb41 <.text+0x75ab41>
14075ccf6: 4c 8d 48 01                 	leaq	0x1(%rax), %r9
14075ccfa: 49 39 d1                    	cmpq	%rdx, %r9
14075ccfd: 72 0a                       	jb	0x14075cd09 <.text+0x75bd09>
14075ccff: eb 20                       	jmp	0x14075cd21 <.text+0x75bd21>
14075cd01: 49 ff c1                    	incq	%r9
14075cd04: 49 39 d1                    	cmpq	%rdx, %r9
14075cd07: 73 14                       	jae	0x14075cd1d <.text+0x75bd1d>
14075cd09: 4f 8b 04 ce                 	movq	(%r14,%r9,8), %r8
14075cd0d: 4d 3b 44 c6 f8              	cmpq	-0x8(%r14,%rax,8), %r8
14075cd12: 74 ed                       	je	0x14075cd01 <.text+0x75bd01>
14075cd14: 4d 89 04 c6                 	movq	%r8, (%r14,%rax,8)
14075cd18: 48 ff c0                    	incq	%rax
14075cd1b: eb e4                       	jmp	0x14075cd01 <.text+0x75bd01>
14075cd1d: 4c 8b 75 58                 	movq	0x58(%rbp), %r14
14075cd21: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075cd25: 48 89 c2                    	movq	%rax, %rdx
14075cd28: e9 f3 f8 ff ff              	jmp	0x14075c620 <.text+0x75b620>
14075cd2d: c6 85 17 01 00 00 01        	movb	$0x1, 0x117(%rbp)
14075cd34: 4c 8d 85 90 00 00 00        	leaq	0x90(%rbp), %r8
14075cd3b: 4c 89 f1                    	movq	%r14, %rcx
14075cd3e: e8 4d 23 ad 00              	callq	0x14122f090 <.text+0x122e090>
14075cd43: e9 5a ff ff ff              	jmp	0x14075cca2 <.text+0x75bca2>
14075cd48: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14075cd50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075cd55: 55                          	pushq	%rbp
14075cd56: 41 57                       	pushq	%r15
14075cd58: 41 56                       	pushq	%r14
14075cd5a: 41 55                       	pushq	%r13
14075cd5c: 41 54                       	pushq	%r12
14075cd5e: 56                          	pushq	%rsi
14075cd5f: 57                          	pushq	%rdi
14075cd60: 53                          	pushq	%rbx
14075cd61: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075cd68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075cd6f: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075cd75: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075cd7b: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075cd81: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075cd88: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075cd91: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075cd99: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075cda2: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14075cda9: 48 ff 00                    	incq	(%rax)
14075cdac: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
14075cdb4: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075cdbc: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075cdc5: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14075cdcb: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075cdd1: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075cdd7: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075cddd: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075cde4: 5b                          	popq	%rbx
14075cde5: 5f                          	popq	%rdi
14075cde6: 5e                          	popq	%rsi
14075cde7: 41 5c                       	popq	%r12
14075cde9: 41 5d                       	popq	%r13
14075cdeb: 41 5e                       	popq	%r14
14075cded: 41 5f                       	popq	%r15
14075cdef: 5d                          	popq	%rbp
14075cdf0: c3                          	retq
14075cdf1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14075ce00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075ce05: 55                          	pushq	%rbp
14075ce06: 41 57                       	pushq	%r15
14075ce08: 41 56                       	pushq	%r14
14075ce0a: 41 55                       	pushq	%r13
14075ce0c: 41 54                       	pushq	%r12
14075ce0e: 56                          	pushq	%rsi
14075ce0f: 57                          	pushq	%rdi
14075ce10: 53                          	pushq	%rbx
14075ce11: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075ce18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075ce1f: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075ce25: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075ce2b: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075ce31: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075ce38: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075ce41: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075ce49: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075ce52: 48 8d 0d 67 f1 ec 02        	leaq	0x2ecf167(%rip), %rcx   # 0x14362bfc0
14075ce59: 8b 95 08 01 00 00           	movl	0x108(%rbp), %edx
14075ce5f: e8 9c 61 8e ff              	callq	0x140043000 <.text+0x42000>
14075ce64: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14075ce6d: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075ce75: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075ce7e: 66 44 0f 6f 4c 24 70        	movdqa	0x70(%rsp), %xmm9
14075ce85: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075ce8b: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075ce91: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075ce97: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075ce9e: 5b                          	popq	%rbx
14075ce9f: 5f                          	popq	%rdi
14075cea0: 5e                          	popq	%rsi
14075cea1: 41 5c                       	popq	%r12
14075cea3: 41 5d                       	popq	%r13
14075cea5: 41 5e                       	popq	%r14
14075cea7: 41 5f                       	popq	%r15
14075cea9: 5d                          	popq	%rbp
14075ceaa: c3                          	retq
14075ceab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14075ceb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075ceb5: 55                          	pushq	%rbp
14075ceb6: 41 57                       	pushq	%r15
14075ceb8: 41 56                       	pushq	%r14
14075ceba: 41 55                       	pushq	%r13
14075cebc: 41 54                       	pushq	%r12
14075cebe: 56                          	pushq	%rsi
14075cebf: 57                          	pushq	%rdi
14075cec0: 53                          	pushq	%rbx
14075cec1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075cec8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075cecf: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075ced5: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075cedb: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075cee1: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075cee8: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075cef1: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075cef9: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075cf02: 80 bd 16 01 00 00 00        	cmpb	$0x0, 0x116(%rbp)
14075cf09: 74 39                       	je	0x14075cf44 <.text+0x75bf44>
14075cf0b: e8 20 b2 4b 00              	callq	0x140c18130 <.text+0xc17130>
14075cf10: 48 89 c6                    	movq	%rax, %rsi
14075cf13: 48 8b 00                    	movq	(%rax), %rax
14075cf16: 48 83 f8 01                 	cmpq	$0x1, %rax
14075cf1a: 75 6d                       	jne	0x14075cf89 <.text+0x75bf89>
14075cf1c: 48 83 c6 08                 	addq	$0x8, %rsi
14075cf20: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14075cf27: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
14075cf2b: 0f 85 82 00 00 00           	jne	0x14075cfb3 <.text+0x75bfb3>
14075cf31: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
14075cf38: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
14075cf3c: e8 af a1 4b 00              	callq	0x140c170f0 <.text+0xc160f0>
14075cf41: 48 ff 06                    	incq	(%rsi)
14075cf44: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
14075cf4c: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075cf54: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075cf5d: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14075cf63: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075cf69: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075cf6f: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075cf75: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075cf7c: 5b                          	popq	%rbx
14075cf7d: 5f                          	popq	%rdi
14075cf7e: 5e                          	popq	%rsi
14075cf7f: 41 5c                       	popq	%r12
14075cf81: 41 5d                       	popq	%r13
14075cf83: 41 5e                       	popq	%r14
14075cf85: 41 5f                       	popq	%r15
14075cf87: 5d                          	popq	%rbp
14075cf88: c3                          	retq
14075cf89: 48 85 c0                    	testq	%rax, %rax
14075cf8c: 75 19                       	jne	0x14075cfa7 <.text+0x75bfa7>
14075cf8e: 48 89 f1                    	movq	%rsi, %rcx
14075cf91: 31 d2                       	xorl	%edx, %edx
14075cf93: e8 28 46 19 02              	callq	0x1428f15c0 <.text+0x28f05c0>
14075cf98: 48 89 c6                    	movq	%rax, %rsi
14075cf9b: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14075cfa2: 48 85 c0                    	testq	%rax, %rax
14075cfa5: 75 80                       	jne	0x14075cf27 <.text+0x75bf27>
14075cfa7: 48 8d 0d fa c4 4b 02        	leaq	0x24bc4fa(%rip), %rcx   # 0x142c194a8
14075cfae: e8 1d b1 17 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14075cfb3: 48 8d 0d 5e c5 4b 02        	leaq	0x24bc55e(%rip), %rcx   # 0x142c19518
14075cfba: e8 91 c2 17 02              	callq	0x1428d9250 <.text+0x28d8250>
14075cfbf: cc                          	int3
14075cfc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075cfc5: 55                          	pushq	%rbp
14075cfc6: 41 57                       	pushq	%r15
14075cfc8: 41 56                       	pushq	%r14
14075cfca: 41 55                       	pushq	%r13
14075cfcc: 41 54                       	pushq	%r12
14075cfce: 56                          	pushq	%rsi
14075cfcf: 57                          	pushq	%rdi
14075cfd0: 53                          	pushq	%rbx
14075cfd1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075cfd8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075cfdf: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075cfe5: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075cfeb: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075cff1: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075cff8: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075d001: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075d009: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075d012: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
14075d019: 48 85 d2                    	testq	%rdx, %rdx
14075d01c: 74 16                       	je	0x14075d034 <.text+0x75c034>
14075d01e: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075d022: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075d028: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14075d02f: e8 ec 6b eb ff              	callq	0x140613c20 <.text+0x612c20>
14075d034: c6 85 17 01 00 00 00        	movb	$0x0, 0x117(%rbp)
14075d03b: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14075d044: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075d04c: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075d055: 66 44 0f 6f 4c 24 70        	movdqa	0x70(%rsp), %xmm9
14075d05c: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075d062: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075d068: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075d06e: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075d075: 5b                          	popq	%rbx
14075d076: 5f                          	popq	%rdi
14075d077: 5e                          	popq	%rsi
14075d078: 41 5c                       	popq	%r12
14075d07a: 41 5d                       	popq	%r13
14075d07c: 41 5e                       	popq	%r14
14075d07e: 41 5f                       	popq	%r15
14075d080: 5d                          	popq	%rbp
14075d081: c3                          	retq
14075d082: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14075d090: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075d095: 55                          	pushq	%rbp
14075d096: 41 57                       	pushq	%r15
14075d098: 41 56                       	pushq	%r14
14075d09a: 41 55                       	pushq	%r13
14075d09c: 41 54                       	pushq	%r12
14075d09e: 56                          	pushq	%rsi
14075d09f: 57                          	pushq	%rdi
14075d0a0: 53                          	pushq	%rbx
14075d0a1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075d0a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075d0af: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075d0b5: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075d0bb: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075d0c1: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075d0c8: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075d0d1: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075d0d9: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075d0e2: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
14075d0e9: 48 ff 00                    	incq	(%rax)
14075d0ec: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
14075d0f4: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075d0fc: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075d105: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14075d10b: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075d111: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075d117: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075d11d: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075d124: 5b                          	popq	%rbx
14075d125: 5f                          	popq	%rdi
14075d126: 5e                          	popq	%rsi
14075d127: 41 5c                       	popq	%r12
14075d129: 41 5d                       	popq	%r13
14075d12b: 41 5e                       	popq	%r14
14075d12d: 41 5f                       	popq	%r15
14075d12f: 5d                          	popq	%rbp
14075d130: c3                          	retq
14075d131: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14075d140: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075d145: 55                          	pushq	%rbp
14075d146: 41 57                       	pushq	%r15
14075d148: 41 56                       	pushq	%r14
14075d14a: 41 55                       	pushq	%r13
14075d14c: 41 54                       	pushq	%r12
14075d14e: 56                          	pushq	%rsi
14075d14f: 57                          	pushq	%rdi
14075d150: 53                          	pushq	%rbx
14075d151: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075d158: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075d15f: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075d165: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075d16b: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075d171: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075d178: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075d181: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075d189: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075d192: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14075d199: e8 12 8e 12 00              	callq	0x140885fb0 <.text+0x884fb0>
14075d19e: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
14075d1a6: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075d1ae: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075d1b7: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14075d1bd: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075d1c3: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075d1c9: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075d1cf: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075d1d6: 5b                          	popq	%rbx
14075d1d7: 5f                          	popq	%rdi
14075d1d8: 5e                          	popq	%rsi
14075d1d9: 41 5c                       	popq	%r12
14075d1db: 41 5d                       	popq	%r13
14075d1dd: 41 5e                       	popq	%r14
14075d1df: 41 5f                       	popq	%r15
14075d1e1: 5d                          	popq	%rbp
14075d1e2: c3                          	retq
14075d1e3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14075d1f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075d1f5: 55                          	pushq	%rbp
14075d1f6: 41 57                       	pushq	%r15
14075d1f8: 41 56                       	pushq	%r14
14075d1fa: 41 55                       	pushq	%r13
14075d1fc: 41 54                       	pushq	%r12
14075d1fe: 56                          	pushq	%rsi
14075d1ff: 57                          	pushq	%rdi
14075d200: 53                          	pushq	%rbx
14075d201: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075d208: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075d20f: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075d215: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075d21b: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075d221: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075d228: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075d231: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075d239: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075d242: 80 bd 17 01 00 00 00        	cmpb	$0x0, 0x117(%rbp)
14075d249: 74 63                       	je	0x14075d2ae <.text+0x75c2ae>
14075d24b: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
14075d24f: 48 85 d2                    	testq	%rdx, %rdx
14075d252: 74 13                       	je	0x14075d267 <.text+0x75c267>
14075d254: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14075d258: 48 c1 e2 03                 	shlq	$0x3, %rdx
14075d25c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14075d262: e8 b9 69 eb ff              	callq	0x140613c20 <.text+0x612c20>
14075d267: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14075d270: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075d278: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075d281: 66 44 0f 6f 4c 24 70        	movdqa	0x70(%rsp), %xmm9
14075d288: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075d28e: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075d294: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075d29a: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075d2a1: 5b                          	popq	%rbx
14075d2a2: 5f                          	popq	%rdi
14075d2a3: 5e                          	popq	%rsi
14075d2a4: 41 5c                       	popq	%r12
14075d2a6: 41 5d                       	popq	%r13
14075d2a8: 41 5e                       	popq	%r14
14075d2aa: 41 5f                       	popq	%r15
14075d2ac: 5d                          	popq	%rbp
14075d2ad: c3                          	retq
14075d2ae: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14075d2b7: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075d2bf: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075d2c8: 66 44 0f 6f 4c 24 70        	movdqa	0x70(%rsp), %xmm9
14075d2cf: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075d2d5: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075d2db: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075d2e1: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075d2e8: 5b                          	popq	%rbx
14075d2e9: 5f                          	popq	%rdi
14075d2ea: 5e                          	popq	%rsi
14075d2eb: 41 5c                       	popq	%r12
14075d2ed: 41 5d                       	popq	%r13
14075d2ef: 41 5e                       	popq	%r14
14075d2f1: 41 5f                       	popq	%r15
14075d2f3: 5d                          	popq	%rbp
14075d2f4: c3                          	retq
14075d2f5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14075d300: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075d305: 55                          	pushq	%rbp
14075d306: 41 57                       	pushq	%r15
14075d308: 41 56                       	pushq	%r14
14075d30a: 41 55                       	pushq	%r13
14075d30c: 41 54                       	pushq	%r12
14075d30e: 56                          	pushq	%rsi
14075d30f: 57                          	pushq	%rdi
14075d310: 53                          	pushq	%rbx
14075d311: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075d318: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075d31f: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075d325: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075d32b: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075d331: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075d338: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075d341: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075d349: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075d352: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14075d359: e8 52 8c 12 00              	callq	0x140885fb0 <.text+0x884fb0>
14075d35e: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
14075d366: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075d36e: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075d377: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
14075d37d: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075d383: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075d389: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075d38f: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075d396: 5b                          	popq	%rbx
14075d397: 5f                          	popq	%rdi
14075d398: 5e                          	popq	%rsi
14075d399: 41 5c                       	popq	%r12
14075d39b: 41 5d                       	popq	%r13
14075d39d: 41 5e                       	popq	%r14
14075d39f: 41 5f                       	popq	%r15
14075d3a1: 5d                          	popq	%rbp
14075d3a2: c3                          	retq
14075d3a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14075d3b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14075d3b5: 55                          	pushq	%rbp
14075d3b6: 41 57                       	pushq	%r15
14075d3b8: 41 56                       	pushq	%r14
14075d3ba: 41 55                       	pushq	%r13
14075d3bc: 41 54                       	pushq	%r12
14075d3be: 56                          	pushq	%rsi
14075d3bf: 57                          	pushq	%rdi
14075d3c0: 53                          	pushq	%rbx
14075d3c1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14075d3c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14075d3cf: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
14075d3d5: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
14075d3db: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
14075d3e1: 66 44 0f 7f 4c 24 70        	movdqa	%xmm9, 0x70(%rsp)
14075d3e8: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14075d3f1: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
14075d3f9: 66 0f 7f b4 24 a0 00 00 00  	movdqa	%xmm6, 0xa0(%rsp)
14075d402: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
14075d409: e8 62 54 e7 ff              	callq	0x1405d2870 <.text+0x5d1870>
14075d40e: 66 0f 6f b4 24 a0 00 00 00  	movdqa	0xa0(%rsp), %xmm6
14075d417: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
14075d41f: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14075d428: 66 44 0f 6f 4c 24 70        	movdqa	0x70(%rsp), %xmm9
14075d42f: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
14075d435: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
14075d43b: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
14075d441: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
14075d448: 5b                          	popq	%rbx
14075d449: 5f                          	popq	%rdi
14075d44a: 5e                          	popq	%rsi
14075d44b: 41 5c                       	popq	%r12
14075d44d: 41 5d                       	popq	%r13
14075d44f: 41 5e                       	popq	%r14
14075d451: 41 5f                       	popq	%r15
14075d453: 5d                          	popq	%rbp
14075d454: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407706d0: 55                          	pushq	%rbp
1407706d1: 41 57                       	pushq	%r15
1407706d3: 41 56                       	pushq	%r14
1407706d5: 41 55                       	pushq	%r13
1407706d7: 41 54                       	pushq	%r12
1407706d9: 56                          	pushq	%rsi
1407706da: 57                          	pushq	%rdi
1407706db: 53                          	pushq	%rbx
1407706dc: 48 81 ec b8 01 00 00        	subq	$0x1b8, %rsp            # imm = 0x1B8
1407706e3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407706eb: 66 0f 7f bd 20 01 00 00     	movdqa	%xmm7, 0x120(%rbp)
1407706f3: 0f 29 b5 10 01 00 00        	movaps	%xmm6, 0x110(%rbp)
1407706fa: 48 c7 85 08 01 00 00 fe ff ff ff    	movq	$-0x2, 0x108(%rbp)
140770705: 4d 89 c6                    	movq	%r8, %r14
140770708: 48 89 8d f8 00 00 00        	movq	%rcx, 0xf8(%rbp)
14077070f: 4c 8b 2a                    	movq	(%rdx), %r13
140770712: 48 8b 4a 08                 	movq	0x8(%rdx), %rcx
140770716: 4d 8b 45 00                 	movq	(%r13), %r8
14077071a: 31 d2                       	xorl	%edx, %edx
14077071c: 4d 89 c2                    	movq	%r8, %r10
14077071f: 4c 2b 51 18                 	subq	0x18(%rcx), %r10
140770723: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
140770727: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
14077072b: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
14077072f: 4c 89 d0                    	movq	%r10, %rax
140770732: 48 c1 e0 04                 	shlq	$0x4, %rax
140770736: 48 03 41 08                 	addq	0x8(%rcx), %rax
14077073a: 48 8b 71 10                 	movq	0x10(%rcx), %rsi
14077073e: 48 8b 59 30                 	movq	0x30(%rcx), %rbx
140770742: 4c 29 d6                    	subq	%r10, %rsi
140770745: 48 0f 42 f2                 	cmovbq	%rdx, %rsi
140770749: 41 ba 08 00 00 00           	movl	$0x8, %r10d
14077074f: 49 0f 42 c2                 	cmovbq	%r10, %rax
140770753: 4d 89 c4                    	movq	%r8, %r12
140770756: 49 c1 e4 04                 	shlq	$0x4, %r12
14077075a: 4c 03 61 28                 	addq	0x28(%rcx), %r12
14077075e: 4c 29 c3                    	subq	%r8, %rbx
140770761: 48 0f 42 da                 	cmovbq	%rdx, %rbx
140770765: 4d 0f 42 e2                 	cmovbq	%r10, %r12
140770769: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14077076d: 48 29 d9                    	subq	%rbx, %rcx
140770770: 48 29 f1                    	subq	%rsi, %rcx
140770773: 49 89 4d 00                 	movq	%rcx, (%r13)
140770777: 48 c1 e6 04                 	shlq	$0x4, %rsi
14077077b: 48 01 c6                    	addq	%rax, %rsi
14077077e: 48 c1 e3 04                 	shlq	$0x4, %rbx
140770782: 4c 01 e3                    	addq	%r12, %rbx
140770785: 41 8b 49 1c                 	movl	0x1c(%r9), %ecx
140770789: 89 8d f4 00 00 00           	movl	%ecx, 0xf4(%rbp)
14077078f: 49 8b 09                    	movq	(%r9), %rcx
140770792: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140770799: 49 8b 49 10                 	movq	0x10(%r9), %rcx
14077079d: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
1407707a4: 49 89 ef                    	movq	%rbp, %r15
1407707a7: eb 3f                       	jmp	0x1407707e8 <.text+0x76f7e8>
1407707a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407707b0: 48 8d 78 10                 	leaq	0x10(%rax), %rdi
1407707b4: 49 ff 45 00                 	incq	(%r13)
1407707b8: 48 8b 40 08                 	movq	0x8(%rax), %rax
1407707bc: 48 89 45 00                 	movq	%rax, (%rbp)
1407707c0: 8b 85 f4 00 00 00           	movl	0xf4(%rbp), %eax
1407707c6: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1407707cd: 89 01                       	movl	%eax, (%rcx)
1407707cf: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1407707d6: 4c 89 fa                    	movq	%r15, %rdx
1407707d9: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
1407707e0: e8 9b 8b 33 00              	callq	0x140aa9380 <.text+0xaa8380>
1407707e5: 48 89 f8                    	movq	%rdi, %rax
1407707e8: 48 85 c0                    	testq	%rax, %rax
1407707eb: 74 05                       	je	0x1407707f2 <.text+0x76f7f2>
1407707ed: 48 39 f0                    	cmpq	%rsi, %rax
1407707f0: 75 be                       	jne	0x1407707b0 <.text+0x76f7b0>
1407707f2: 4d 85 e4                    	testq	%r12, %r12
1407707f5: 74 10                       	je	0x140770807 <.text+0x76f807>
1407707f7: 49 39 dc                    	cmpq	%rbx, %r12
1407707fa: 74 0b                       	je	0x140770807 <.text+0x76f807>
1407707fc: 4c 89 e0                    	movq	%r12, %rax
1407707ff: 49 83 c4 10                 	addq	$0x10, %r12
140770803: 31 ff                       	xorl	%edi, %edi
140770805: eb ad                       	jmp	0x1407707b4 <.text+0x76f7b4>
140770807: 4d 8b 3e                    	movq	(%r14), %r15
14077080a: 49 8b 46 08                 	movq	0x8(%r14), %rax
14077080e: 4d 8b 07                    	movq	(%r15), %r8
140770811: 31 c9                       	xorl	%ecx, %ecx
140770813: 4d 89 c1                    	movq	%r8, %r9
140770816: 4c 2b 48 18                 	subq	0x18(%rax), %r9
14077081a: 4c 0f 42 c9                 	cmovbq	%rcx, %r9
14077081e: 4c 2b 40 38                 	subq	0x38(%rax), %r8
140770822: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
140770826: 4b 8d 14 49                 	leaq	(%r9,%r9,2), %rdx
14077082a: 48 c1 e2 03                 	shlq	$0x3, %rdx
14077082e: 48 03 50 08                 	addq	0x8(%rax), %rdx
140770832: 4c 8b 50 10                 	movq	0x10(%rax), %r10
140770836: 4c 8b 58 30                 	movq	0x30(%rax), %r11
14077083a: 4d 29 ca                    	subq	%r9, %r10
14077083d: 4c 0f 42 d1                 	cmovbq	%rcx, %r10
140770841: 41 b9 08 00 00 00           	movl	$0x8, %r9d
140770847: 49 0f 42 d1                 	cmovbq	%r9, %rdx
14077084b: 4b 8d 34 40                 	leaq	(%r8,%r8,2), %rsi
14077084f: 48 c1 e6 03                 	shlq	$0x3, %rsi
140770853: 48 03 70 28                 	addq	0x28(%rax), %rsi
140770857: 4d 29 c3                    	subq	%r8, %r11
14077085a: 4c 0f 42 d9                 	cmovbq	%rcx, %r11
14077085e: 49 0f 42 f1                 	cmovbq	%r9, %rsi
140770862: 48 8b 40 40                 	movq	0x40(%rax), %rax
140770866: 4c 29 d8                    	subq	%r11, %rax
140770869: 4c 29 d0                    	subq	%r10, %rax
14077086c: 49 89 07                    	movq	%rax, (%r15)
14077086f: 4b 8d 04 52                 	leaq	(%r10,%r10,2), %rax
140770873: 48 8d 3c c2                 	leaq	(%rdx,%rax,8), %rdi
140770877: 4b 8d 04 5b                 	leaq	(%r11,%r11,2), %rax
14077087b: 48 8d 1c c6                 	leaq	(%rsi,%rax,8), %rbx
14077087f: eb 3f                       	jmp	0x1407708c0 <.text+0x76f8c0>
140770881: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140770890: 4c 8d 72 18                 	leaq	0x18(%rdx), %r14
140770894: 48 83 c2 08                 	addq	$0x8, %rdx
140770898: 49 ff 07                    	incq	(%r15)
14077089b: 8b 85 f4 00 00 00           	movl	0xf4(%rbp), %eax
1407708a1: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1407708a8: 89 01                       	movl	%eax, (%rcx)
1407708aa: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1407708b1: 4c 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %r8
1407708b8: e8 c3 8a 33 00              	callq	0x140aa9380 <.text+0xaa8380>
1407708bd: 4c 89 f2                    	movq	%r14, %rdx
1407708c0: 48 85 d2                    	testq	%rdx, %rdx
1407708c3: 74 05                       	je	0x1407708ca <.text+0x76f8ca>
1407708c5: 48 39 fa                    	cmpq	%rdi, %rdx
1407708c8: 75 c6                       	jne	0x140770890 <.text+0x76f890>
1407708ca: 48 85 f6                    	testq	%rsi, %rsi
1407708cd: 74 11                       	je	0x1407708e0 <.text+0x76f8e0>
1407708cf: 48 39 de                    	cmpq	%rbx, %rsi
1407708d2: 74 0c                       	je	0x1407708e0 <.text+0x76f8e0>
1407708d4: 48 89 f2                    	movq	%rsi, %rdx
1407708d7: 48 83 c6 18                 	addq	$0x18, %rsi
1407708db: 45 31 f6                    	xorl	%r14d, %r14d
1407708de: eb b4                       	jmp	0x140770894 <.text+0x76f894>
1407708e0: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1407708e7: 4c 8b 38                    	movq	(%rax), %r15
1407708ea: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1407708ee: 4d 8b 07                    	movq	(%r15), %r8
1407708f1: 31 d2                       	xorl	%edx, %edx
1407708f3: 4c 89 c0                    	movq	%r8, %rax
1407708f6: 48 2b 41 18                 	subq	0x18(%rcx), %rax
1407708fa: 48 0f 42 c2                 	cmovbq	%rdx, %rax
1407708fe: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
140770902: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
140770906: 48 8d 1c 80                 	leaq	(%rax,%rax,4), %rbx
14077090a: 48 c1 e3 03                 	shlq	$0x3, %rbx
14077090e: 48 03 59 08                 	addq	0x8(%rcx), %rbx
140770912: 4c 8b 49 10                 	movq	0x10(%rcx), %r9
140770916: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
14077091a: 49 29 c1                    	subq	%rax, %r9
14077091d: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
140770921: 41 bb 08 00 00 00           	movl	$0x8, %r11d
140770927: 49 0f 42 db                 	cmovbq	%r11, %rbx
14077092b: 4b 8d 04 80                 	leaq	(%r8,%r8,4), %rax
14077092f: 48 c1 e0 03                 	shlq	$0x3, %rax
140770933: 48 03 41 28                 	addq	0x28(%rcx), %rax
140770937: 4d 29 c2                    	subq	%r8, %r10
14077093a: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
14077093e: 49 0f 42 c3                 	cmovbq	%r11, %rax
140770942: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140770946: 4c 29 d1                    	subq	%r10, %rcx
140770949: 4c 29 c9                    	subq	%r9, %rcx
14077094c: 49 89 0f                    	movq	%rcx, (%r15)
14077094f: 4b 8d 0c 89                 	leaq	(%r9,%r9,4), %rcx
140770953: 48 8d 0c cb                 	leaq	(%rbx,%rcx,8), %rcx
140770957: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
14077095b: 4b 8d 0c 92                 	leaq	(%r10,%r10,4), %rcx
14077095f: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
140770963: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140770967: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14077096e: 4c 8b 29                    	movq	(%rcx), %r13
140770971: 49 8d 4d 20                 	leaq	0x20(%r13), %rcx
140770975: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
140770979: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140770980: 48 8b 09                    	movq	(%rcx), %rcx
140770983: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
140770987: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
14077098e: 48 83 c1 20                 	addq	$0x20, %rcx
140770992: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140770996: 48 8d 75 0f                 	leaq	0xf(%rbp), %rsi
14077099a: 66 0f 76 ff                 	pcmpeqd	%xmm7, %xmm7
14077099e: 0f 28 35 1b d7 1c 02        	movaps	0x21cd71b(%rip), %xmm6  # 0x14293e0c0
1407709a5: 4c 89 7d 68                 	movq	%r15, 0x68(%rbp)
1407709a9: 4c 89 6d d8                 	movq	%r13, -0x28(%rbp)
1407709ad: eb 17                       	jmp	0x1407709c6 <.text+0x76f9c6>
1407709af: 90                          	nop
1407709b0: 48 8b 9d b8 00 00 00        	movq	0xb8(%rbp), %rbx
1407709b7: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1407709be: 4c 8b 7d 68                 	movq	0x68(%rbp), %r15
1407709c2: 4c 8b 6d d8                 	movq	-0x28(%rbp), %r13
1407709c6: 48 85 db                    	testq	%rbx, %rbx
1407709c9: 74 25                       	je	0x1407709f0 <.text+0x76f9f0>
1407709cb: 48 3b 5d 58                 	cmpq	0x58(%rbp), %rbx
1407709cf: 74 1f                       	je	0x1407709f0 <.text+0x76f9f0>
1407709d1: 48 8d 4b 28                 	leaq	0x28(%rbx), %rcx
1407709d5: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
1407709dc: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1407709e3: eb 37                       	jmp	0x140770a1c <.text+0x76fa1c>
1407709e5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407709f0: 48 85 c0                    	testq	%rax, %rax
1407709f3: 0f 84 35 05 00 00           	je	0x140770f2e <.text+0x76ff2e>
1407709f9: 48 3b 45 e8                 	cmpq	-0x18(%rbp), %rax
1407709fd: 0f 84 2b 05 00 00           	je	0x140770f2e <.text+0x76ff2e>
140770a03: 48 8d 48 28                 	leaq	0x28(%rax), %rcx
140770a07: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
140770a0e: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
140770a19: 48 89 c3                    	movq	%rax, %rbx
140770a1c: 49 ff 07                    	incq	(%r15)
140770a1f: 48 8d 7b 18                 	leaq	0x18(%rbx), %rdi
140770a23: 8b 85 f4 00 00 00           	movl	0xf4(%rbp), %eax
140770a29: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140770a30: 89 01                       	movl	%eax, (%rcx)
140770a32: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140770a36: 48 89 fa                    	movq	%rdi, %rdx
140770a39: e8 32 03 e4 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140770a3e: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140770a45: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
140770a4c: 49 89 c0                    	movq	%rax, %r8
140770a4f: 48 89 bd f8 00 00 00        	movq	%rdi, 0xf8(%rbp)
140770a56: 49 89 f9                    	movq	%rdi, %r9
140770a59: e8 92 6d 40 00              	callq	0x140b777f0 <.text+0xb767f0>
140770a5e: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140770a65: 48 89 c8                    	movq	%rcx, %rax
140770a68: 48 f7 d8                    	negq	%rax
140770a6b: 0f 80 c2 00 00 00           	jo	0x140770b33 <.text+0x76fb33>
140770a71: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
140770a75: 48 89 5d e0                 	movq	%rbx, -0x20(%rbp)
140770a79: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
140770a80: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
140770a87: 48 8b bd 98 00 00 00        	movq	0x98(%rbp), %rdi
140770a8e: 48 85 ff                    	testq	%rdi, %rdi
140770a91: 74 7d                       	je	0x140770b10 <.text+0x76fb10>
140770a93: 48 c1 e7 03                 	shlq	$0x3, %rdi
140770a97: 45 31 f6                    	xorl	%r14d, %r14d
140770a9a: eb 0d                       	jmp	0x140770aa9 <.text+0x76faa9>
140770a9c: 0f 1f 40 00                 	nopl	(%rax)
140770aa0: 49 83 c6 08                 	addq	$0x8, %r14
140770aa4: 4c 39 f7                    	cmpq	%r14, %rdi
140770aa7: 74 67                       	je	0x140770b10 <.text+0x76fb10>
140770aa9: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
140770ab0: 4a 8b 1c 30                 	movq	(%rax,%r14), %rbx
140770ab4: 48 89 d8                    	movq	%rbx, %rax
140770ab7: 48 c1 e8 20                 	shrq	$0x20, %rax
140770abb: 74 53                       	je	0x140770b10 <.text+0x76fb10>
140770abd: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
140770ac4: 4c 8b 20                    	movq	(%rax), %r12
140770ac7: 4c 89 e1                    	movq	%r12, %rcx
140770aca: 48 89 da                    	movq	%rbx, %rdx
140770acd: e8 ce a4 57 00              	callq	0x140ceafa0 <.text+0xce9fa0>
140770ad2: 84 c0                       	testb	%al, %al
140770ad4: 74 ca                       	je	0x140770aa0 <.text+0x76faa0>
140770ad6: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
140770add: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
140770ae2: 48 8b 40 18                 	movq	0x18(%rax), %rax
140770ae6: 4c 89 65 00                 	movq	%r12, (%rbp)
140770aea: f3 0f 7f 45 08              	movdqu	%xmm0, 0x8(%rbp)
140770aef: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140770af3: 48 89 5d 20                 	movq	%rbx, 0x20(%rbp)
140770af7: 48 89 e9                    	movq	%rbp, %rcx
140770afa: 48 8d 15 c7 e1 30 02        	leaq	0x230e1c7(%rip), %rdx   # 0x142a7ecc8
140770b01: e8 6a 3c 5a 00              	callq	0x140d14770 <.text+0xd13770>
140770b06: eb 98                       	jmp	0x140770aa0 <.text+0x76faa0>
140770b08: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140770b10: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140770b14: 48 85 d2                    	testq	%rdx, %rdx
140770b17: 74 16                       	je	0x140770b2f <.text+0x76fb2f>
140770b19: 48 c1 e2 03                 	shlq	$0x3, %rdx
140770b1d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140770b23: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
140770b2a: e8 f1 30 ea ff              	callq	0x140613c20 <.text+0x612c20>
140770b2f: 48 8b 5d e0                 	movq	-0x20(%rbp), %rbx
140770b33: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
140770b38: 0f 84 e4 03 00 00           	je	0x140770f22 <.text+0x76ff22>
140770b3e: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140770b42: 48 8b 95 f8 00 00 00        	movq	0xf8(%rbp), %rdx
140770b49: e8 22 02 e4 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140770b4e: 49 8b 4d 00                 	movq	(%r13), %rcx
140770b52: 49 8b 55 08                 	movq	0x8(%r13), %rdx
140770b56: 49 89 d0                    	movq	%rdx, %r8
140770b59: 49 21 c0                    	andq	%rax, %r8
140770b5c: 48 c1 e8 39                 	shrq	$0x39, %rax
140770b60: 66 0f 6e c0                 	movd	%eax, %xmm0
140770b64: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140770b68: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140770b6d: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140770b72: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
140770b79: 45 31 c9                    	xorl	%r9d, %r9d
140770b7c: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140770b82: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140770b86: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140770b8a: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140770b8f: 45 85 d2                    	testl	%r10d, %r10d
140770b92: 74 3c                       	je	0x140770bd0 <.text+0x76fbd0>
140770b94: 4c 8b 9d f8 00 00 00        	movq	0xf8(%rbp), %r11
140770b9b: 4d 8b 1b                    	movq	(%r11), %r11
140770b9e: f3 41 0f bc fa              	tzcntl	%r10d, %edi
140770ba3: 4c 01 c7                    	addq	%r8, %rdi
140770ba6: 48 21 d7                    	andq	%rdx, %rdi
140770ba9: 48 f7 df                    	negq	%rdi
140770bac: 48 69 ff a8 02 00 00        	imulq	$0x2a8, %rdi, %rdi      # imm = 0x2A8
140770bb3: 4c 3b 1c 38                 	cmpq	(%rax,%rdi), %r11
140770bb7: 74 47                       	je	0x140770c00 <.text+0x76fc00>
140770bb9: 41 8d 7a ff                 	leal	-0x1(%r10), %edi
140770bbd: 66 44 21 d7                 	andw	%r10w, %di
140770bc1: 41 89 fa                    	movl	%edi, %r10d
140770bc4: 75 d8                       	jne	0x140770b9e <.text+0x76fb9e>
140770bc6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140770bd0: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
140770bd4: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140770bd9: 45 85 d2                    	testl	%r10d, %r10d
140770bdc: 0f 85 40 03 00 00           	jne	0x140770f22 <.text+0x76ff22>
140770be2: 4d 01 c8                    	addq	%r9, %r8
140770be5: 49 83 c0 10                 	addq	$0x10, %r8
140770be9: 49 83 c1 10                 	addq	$0x10, %r9
140770bed: 49 21 d0                    	andq	%rdx, %r8
140770bf0: eb 8a                       	jmp	0x140770b7c <.text+0x76fb7c>
140770bf2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140770c00: 48 8d 14 39                 	leaq	(%rcx,%rdi), %rdx
140770c04: 48 81 c2 60 fd ff ff        	addq	$-0x2a0, %rdx           # imm = 0xFD60
140770c0b: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
140770c16: 48 c7 85 d0 00 00 00 08 00 00 00    	movq	$0x8, 0xd0(%rbp)
140770c21: 48 c7 85 d8 00 00 00 00 00 00 00    	movq	$0x0, 0xd8(%rbp)
140770c2c: c6 85 07 01 00 00 01        	movb	$0x1, 0x107(%rbp)
140770c33: 48 8d 85 c8 00 00 00        	leaq	0xc8(%rbp), %rax
140770c3a: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140770c3f: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
140770c46: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140770c4b: 48 8b 85 c8 01 00 00        	movq	0x1c8(%rbp), %rax
140770c52: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140770c57: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140770c5e: 4d 89 e8                    	movq	%r13, %r8
140770c61: 4c 8b 4d 60                 	movq	0x60(%rbp), %r9
140770c65: e8 a6 36 ad 00              	callq	0x141244310 <.text+0x1243310>
140770c6a: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
140770c6e: 4c 8b 6b 10                 	movq	0x10(%rbx), %r13
140770c72: 49 c1 e5 06                 	shlq	$0x6, %r13
140770c76: 4d 01 fd                    	addq	%r15, %r13
140770c79: eb 1e                       	jmp	0x140770c99 <.text+0x76fc99>
140770c7b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140770c80: 49 83 c7 40                 	addq	$0x40, %r15
140770c84: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
140770c8b: 4c 89 34 f8                 	movq	%r14, (%rax,%rdi,8)
140770c8f: 48 ff c7                    	incq	%rdi
140770c92: 48 89 bd d8 00 00 00        	movq	%rdi, 0xd8(%rbp)
140770c99: 4d 39 ef                    	cmpq	%r13, %r15
140770c9c: 0f 84 fe 01 00 00           	je	0x140770ea0 <.text+0x76fea0>
140770ca2: 41 8b 4f 30                 	movl	0x30(%r15), %ecx
140770ca6: c6 85 07 01 00 00 01        	movb	$0x1, 0x107(%rbp)
140770cad: e8 9e 25 0c 00              	callq	0x140833250 <.text+0x832250>
140770cb2: c6 85 07 01 00 00 01        	movb	$0x1, 0x107(%rbp)
140770cb9: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140770cc0: 89 c2                       	movl	%eax, %edx
140770cc2: e8 99 b0 0b 00              	callq	0x14082bd60 <.text+0x82ad60>
140770cc7: f3 0f 6f 85 80 00 00 00     	movdqu	0x80(%rbp), %xmm0
140770ccf: 66 0f 7f 45 f0              	movdqa	%xmm0, -0x10(%rbp)
140770cd4: c6 85 07 01 00 00 01        	movb	$0x1, 0x107(%rbp)
140770cdb: e8 a0 ae 8c ff              	callq	0x14003bb80 <.text+0x3ab80>
140770ce0: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
140770ce4: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
140770ceb: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140770cf2: 48 8b 38                    	movq	(%rax), %rdi
140770cf5: 48 8d 45 c8                 	leaq	-0x38(%rbp), %rax
140770cf9: 0f 11 00                    	movups	%xmm0, (%rax)
140770cfc: f3 41 0f 6f 07              	movdqu	(%r15), %xmm0
140770d01: f3 41 0f 6f 4f 10           	movdqu	0x10(%r15), %xmm1
140770d07: f3 41 0f 6f 57 20           	movdqu	0x20(%r15), %xmm2
140770d0d: 41 0f 10 5f 30              	movups	0x30(%r15), %xmm3
140770d12: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140770d16: f3 0f 7f 56 20              	movdqu	%xmm2, 0x20(%rsi)
140770d1b: f3 0f 7f 4e 10              	movdqu	%xmm1, 0x10(%rsi)
140770d20: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
140770d24: c6 85 07 01 00 00 01        	movb	$0x1, 0x107(%rbp)
140770d2b: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140770d32: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
140770d39: e8 62 39 5a 00              	callq	0x140d146a0 <.text+0xd136a0>
140770d3e: 48 8b 9d 88 00 00 00        	movq	0x88(%rbp), %rbx
140770d45: 48 85 db                    	testq	%rbx, %rbx
140770d48: 75 07                       	jne	0x140770d51 <.text+0x76fd51>
140770d4a: 48 8b 9d 90 00 00 00        	movq	0x90(%rbp), %rbx
140770d51: 4c 8b b5 a0 00 00 00        	movq	0xa0(%rbp), %r14
140770d58: 48 8b 03                    	movq	(%rbx), %rax
140770d5b: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
140770d5f: 4c 29 e0                    	subq	%r12, %rax
140770d62: 48 3d d7 00 00 00           	cmpq	$0xd7, %rax
140770d68: 0f 86 02 01 00 00           	jbe	0x140770e70 <.text+0x76fe70>
140770d6e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140770d72: 48 8d 0d 57 da a9 00        	leaq	0xa9da57(%rip), %rcx    # 0x14120e7d0 <.text+0x120d7d0>
140770d79: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
140770d7d: 42 0f 11 74 20 08           	movups	%xmm6, 0x8(%rax,%r12)
140770d83: 4a c7 44 20 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r12)
140770d8c: 42 c7 44 20 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r12)
140770d95: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140770d9f: 4a 89 4c 20 24              	movq	%rcx, 0x24(%rax,%r12)
140770da4: 42 c7 44 20 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r12) # imm = 0x3F800000
140770dad: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
140770db1: 42 0f 11 44 20 30           	movups	%xmm0, 0x30(%rax,%r12)
140770db7: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
140770dbb: 4a 89 4c 20 40              	movq	%rcx, 0x40(%rax,%r12)
140770dc0: 42 c6 44 20 48 18           	movb	$0x18, 0x48(%rax,%r12)
140770dc6: 0f 10 45 00                 	movups	(%rbp), %xmm0
140770dca: f3 0f 6f 4d 10              	movdqu	0x10(%rbp), %xmm1
140770dcf: f3 0f 6f 55 20              	movdqu	0x20(%rbp), %xmm2
140770dd4: 0f 10 5d 30                 	movups	0x30(%rbp), %xmm3
140770dd8: 42 0f 11 44 20 49           	movups	%xmm0, 0x49(%rax,%r12)
140770dde: f3 42 0f 7f 4c 20 59        	movdqu	%xmm1, 0x59(%rax,%r12)
140770de5: f3 42 0f 7f 54 20 69        	movdqu	%xmm2, 0x69(%rax,%r12)
140770dec: 42 0f 11 5c 20 79           	movups	%xmm3, 0x79(%rax,%r12)
140770df2: f3 0f 6f 45 3f              	movdqu	0x3f(%rbp), %xmm0
140770df7: f3 42 0f 7f 84 20 88 00 00 00       	movdqu	%xmm0, 0x88(%rax,%r12)
140770e01: 4a 89 bc 20 98 00 00 00     	movq	%rdi, 0x98(%rax,%r12)
140770e09: 42 c6 84 20 a0 00 00 00 01  	movb	$0x1, 0xa0(%rax,%r12)
140770e12: 42 c6 84 20 a8 00 00 00 00  	movb	$0x0, 0xa8(%rax,%r12)
140770e1b: 4e 89 b4 20 b8 00 00 00     	movq	%r14, 0xb8(%rax,%r12)
140770e23: 48 8d 0d 96 f1 a9 00        	leaq	0xa9f196(%rip), %rcx    # 0x14120ffc0 <.text+0x120efc0>
140770e2a: 4a 89 8c 20 c8 00 00 00     	movq	%rcx, 0xc8(%rax,%r12)
140770e32: 49 81 c4 d8 00 00 00        	addq	$0xd8, %r12
140770e39: 4c 89 63 10                 	movq	%r12, 0x10(%rbx)
140770e3d: 48 8b bd d8 00 00 00        	movq	0xd8(%rbp), %rdi
140770e44: 48 3b bd c8 00 00 00        	cmpq	0xc8(%rbp), %rdi
140770e4b: 0f 85 2f fe ff ff           	jne	0x140770c80 <.text+0x76fc80>
140770e51: c6 85 07 01 00 00 01        	movb	$0x1, 0x107(%rbp)
140770e58: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
140770e5f: 48 8d 15 92 de 30 02        	leaq	0x230de92(%rip), %rdx   # 0x142a7ecf8
140770e66: e8 85 0f 41 00              	callq	0x140b81df0 <.text+0xb80df0>
140770e6b: e9 10 fe ff ff              	jmp	0x140770c80 <.text+0x76fc80>
140770e70: c6 85 07 01 00 00 01        	movb	$0x1, 0x107(%rbp)
140770e77: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140770e80: 41 b8 d8 00 00 00           	movl	$0xd8, %r8d
140770e86: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140770e8c: 48 89 d9                    	movq	%rbx, %rcx
140770e8f: 4c 89 e2                    	movq	%r12, %rdx
140770e92: e8 49 6f 13 02              	callq	0x1428a7de0 <.text+0x28a6de0>
140770e97: e9 d2 fe ff ff              	jmp	0x140770d6e <.text+0x76fd6e>
140770e9c: 0f 1f 40 00                 	nopl	(%rax)
140770ea0: 8b 85 f4 00 00 00           	movl	0xf4(%rbp), %eax
140770ea6: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140770ead: 89 01                       	movl	%eax, (%rcx)
140770eaf: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140770eb6: 4c 8b 00                    	movq	(%rax), %r8
140770eb9: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
140770ec0: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140770ec4: f3 0f 6f 85 c8 00 00 00     	movdqu	0xc8(%rbp), %xmm0
140770ecc: 66 0f 7f 45 00              	movdqa	%xmm0, (%rbp)
140770ed1: c6 85 07 01 00 00 00        	movb	$0x0, 0x107(%rbp)
140770ed8: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140770edf: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
140770ee6: 49 89 e9                    	movq	%rbp, %r9
140770ee9: e8 a2 b9 a8 00              	callq	0x1411fc890 <.text+0x11fb890>
140770eee: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
140770ef5: 48 89 d0                    	movq	%rdx, %rax
140770ef8: 48 f7 d8                    	negq	%rax
140770efb: 0f 80 af fa ff ff           	jo	0x1407709b0 <.text+0x76f9b0>
140770f01: 0f 83 a9 fa ff ff           	jae	0x1407709b0 <.text+0x76f9b0>
140770f07: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
140770f0e: 48 c1 e2 03                 	shlq	$0x3, %rdx
140770f12: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140770f18: e8 03 2d ea ff              	callq	0x140613c20 <.text+0x612c20>
140770f1d: e9 8e fa ff ff              	jmp	0x1407709b0 <.text+0x76f9b0>
140770f22: 48 8d 0d b7 dd 30 02        	leaq	0x230ddb7(%rip), %rcx   # 0x142a7ece0
140770f29: e8 e2 83 16 02              	callq	0x1428d9310 <.text+0x28d8310>
140770f2e: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
140770f35: 48 8b 38                    	movq	(%rax), %rdi
140770f38: 48 8b 48 08                 	movq	0x8(%rax), %rcx
140770f3c: 4c 8b 07                    	movq	(%rdi), %r8
140770f3f: 31 d2                       	xorl	%edx, %edx
140770f41: 4c 89 c0                    	movq	%r8, %rax
140770f44: 48 2b 41 18                 	subq	0x18(%rcx), %rax
140770f48: 48 0f 42 c2                 	cmovbq	%rdx, %rax
140770f4c: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
140770f50: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
140770f54: 48 8d 1c 80                 	leaq	(%rax,%rax,4), %rbx
140770f58: 48 c1 e3 03                 	shlq	$0x3, %rbx
140770f5c: 48 03 59 08                 	addq	0x8(%rcx), %rbx
140770f60: 4c 8b 49 10                 	movq	0x10(%rcx), %r9
140770f64: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
140770f68: 49 29 c1                    	subq	%rax, %r9
140770f6b: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
140770f6f: 41 bb 08 00 00 00           	movl	$0x8, %r11d
140770f75: 49 0f 42 db                 	cmovbq	%r11, %rbx
140770f79: 4b 8d 04 80                 	leaq	(%r8,%r8,4), %rax
140770f7d: 48 c1 e0 03                 	shlq	$0x3, %rax
140770f81: 48 03 41 28                 	addq	0x28(%rcx), %rax
140770f85: 4d 29 c2                    	subq	%r8, %r10
140770f88: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
140770f8c: 49 0f 42 c3                 	cmovbq	%r11, %rax
140770f90: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
140770f94: 4c 29 d1                    	subq	%r10, %rcx
140770f97: 4c 29 c9                    	subq	%r9, %rcx
140770f9a: 48 89 7d 50                 	movq	%rdi, 0x50(%rbp)
140770f9e: 48 89 0f                    	movq	%rcx, (%rdi)
140770fa1: 4b 8d 0c 89                 	leaq	(%r9,%r9,4), %rcx
140770fa5: 48 8d 0c cb                 	leaq	(%rbx,%rcx,8), %rcx
140770fa9: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
140770fad: 4b 8d 0c 92                 	leaq	(%r10,%r10,4), %rcx
140770fb1: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
140770fb5: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
140770fb9: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140770fc0: 48 8d 51 30                 	leaq	0x30(%rcx), %rdx
140770fc4: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
140770fcb: 48 83 c1 50                 	addq	$0x50, %rcx
140770fcf: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140770fd6: 4c 8d b5 80 00 00 00        	leaq	0x80(%rbp), %r14
140770fdd: eb 0c                       	jmp	0x140770feb <.text+0x76ffeb>
140770fdf: 90                          	nop
140770fe0: 48 8b 5d 78                 	movq	0x78(%rbp), %rbx
140770fe4: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140770feb: 48 85 db                    	testq	%rbx, %rbx
140770fee: 74 20                       	je	0x140771010 <.text+0x770010>
140770ff0: 48 3b 5d 60                 	cmpq	0x60(%rbp), %rbx
140770ff4: 74 1a                       	je	0x140771010 <.text+0x770010>
140770ff6: 48 8d 4b 28                 	leaq	0x28(%rbx), %rcx
140770ffa: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
140770ffe: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
140771005: eb 32                       	jmp	0x140771039 <.text+0x770039>
140771007: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140771010: 48 85 c0                    	testq	%rax, %rax
140771013: 0f 84 e7 03 00 00           	je	0x140771400 <.text+0x770400>
140771019: 48 3b 45 68                 	cmpq	0x68(%rbp), %rax
14077101d: 0f 84 dd 03 00 00           	je	0x140771400 <.text+0x770400>
140771023: 48 8d 48 28                 	leaq	0x28(%rax), %rcx
140771027: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
14077102e: 48 c7 45 78 00 00 00 00     	movq	$0x0, 0x78(%rbp)
140771036: 48 89 c3                    	movq	%rax, %rbx
140771039: 48 8b 45 50                 	movq	0x50(%rbp), %rax
14077103d: 48 ff 00                    	incq	(%rax)
140771040: 4c 8d 7b 18                 	leaq	0x18(%rbx), %r15
140771044: 8b 85 f4 00 00 00           	movl	0xf4(%rbp), %eax
14077104a: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
140771051: 89 01                       	movl	%eax, (%rcx)
140771053: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
14077105a: 4c 89 fa                    	movq	%r15, %rdx
14077105d: e8 0e fd e3 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140771062: 4c 89 f1                    	movq	%r14, %rcx
140771065: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
14077106c: 49 89 c0                    	movq	%rax, %r8
14077106f: 4d 89 f9                    	movq	%r15, %r9
140771072: e8 79 67 40 00              	callq	0x140b777f0 <.text+0xb767f0>
140771077: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14077107e: 48 89 c8                    	movq	%rcx, %rax
140771081: 48 f7 d8                    	negq	%rax
140771084: 0f 80 cc 00 00 00           	jo	0x140771156 <.text+0x770156>
14077108a: 48 89 8d e8 00 00 00        	movq	%rcx, 0xe8(%rbp)
140771091: 48 89 5d 58                 	movq	%rbx, 0x58(%rbp)
140771095: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
14077109c: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
1407710a3: 48 8b bd 98 00 00 00        	movq	0x98(%rbp), %rdi
1407710aa: 48 85 ff                    	testq	%rdi, %rdi
1407710ad: 0f 84 7d 00 00 00           	je	0x140771130 <.text+0x770130>
1407710b3: 48 c1 e7 03                 	shlq	$0x3, %rdi
1407710b7: 45 31 ed                    	xorl	%r13d, %r13d
1407710ba: eb 0d                       	jmp	0x1407710c9 <.text+0x7700c9>
1407710bc: 0f 1f 40 00                 	nopl	(%rax)
1407710c0: 49 83 c5 08                 	addq	$0x8, %r13
1407710c4: 4c 39 ef                    	cmpq	%r13, %rdi
1407710c7: 74 67                       	je	0x140771130 <.text+0x770130>
1407710c9: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
1407710d0: 4a 8b 1c 28                 	movq	(%rax,%r13), %rbx
1407710d4: 48 89 d8                    	movq	%rbx, %rax
1407710d7: 48 c1 e8 20                 	shrq	$0x20, %rax
1407710db: 74 53                       	je	0x140771130 <.text+0x770130>
1407710dd: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1407710e4: 4c 8b 20                    	movq	(%rax), %r12
1407710e7: 4c 89 e1                    	movq	%r12, %rcx
1407710ea: 48 89 da                    	movq	%rbx, %rdx
1407710ed: e8 ae 9e 57 00              	callq	0x140ceafa0 <.text+0xce9fa0>
1407710f2: 84 c0                       	testb	%al, %al
1407710f4: 74 ca                       	je	0x1407710c0 <.text+0x7700c0>
1407710f6: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1407710fd: f3 0f 6f 40 08              	movdqu	0x8(%rax), %xmm0
140771102: 48 8b 40 18                 	movq	0x18(%rax), %rax
140771106: 4c 89 65 00                 	movq	%r12, (%rbp)
14077110a: f3 0f 7f 45 08              	movdqu	%xmm0, 0x8(%rbp)
14077110f: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140771113: 48 89 5d 20                 	movq	%rbx, 0x20(%rbp)
140771117: 48 89 e9                    	movq	%rbp, %rcx
14077111a: 48 8d 15 77 db 30 02        	leaq	0x230db77(%rip), %rdx   # 0x142a7ec98
140771121: e8 4a 36 5a 00              	callq	0x140d14770 <.text+0xd13770>
140771126: eb 98                       	jmp	0x1407710c0 <.text+0x7700c0>
140771128: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140771130: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
140771137: 48 85 d2                    	testq	%rdx, %rdx
14077113a: 74 16                       	je	0x140771152 <.text+0x770152>
14077113c: 48 c1 e2 03                 	shlq	$0x3, %rdx
140771140: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140771146: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
14077114d: e8 ce 2a ea ff              	callq	0x140613c20 <.text+0x612c20>
140771152: 48 8b 5d 58                 	movq	0x58(%rbp), %rbx
140771156: 48 c7 85 c8 00 00 00 00 00 00 00    	movq	$0x0, 0xc8(%rbp)
140771161: 48 c7 85 d0 00 00 00 08 00 00 00    	movq	$0x8, 0xd0(%rbp)
14077116c: 48 c7 85 d8 00 00 00 00 00 00 00    	movq	$0x0, 0xd8(%rbp)
140771177: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14077117b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14077117f: 48 c1 e0 06                 	shlq	$0x6, %rax
140771183: 48 01 c8                    	addq	%rcx, %rax
140771186: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
14077118d: eb 21                       	jmp	0x1407711b0 <.text+0x7701b0>
14077118f: 90                          	nop
140771190: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140771197: 48 83 c1 40                 	addq	$0x40, %rcx
14077119b: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
1407711a2: 4c 89 2c f8                 	movq	%r13, (%rax,%rdi,8)
1407711a6: 48 ff c7                    	incq	%rdi
1407711a9: 48 89 bd d8 00 00 00        	movq	%rdi, 0xd8(%rbp)
1407711b0: 48 3b 8d e8 00 00 00        	cmpq	0xe8(%rbp), %rcx
1407711b7: 48 89 8d f8 00 00 00        	movq	%rcx, 0xf8(%rbp)
1407711be: 0f 84 cc 01 00 00           	je	0x140771390 <.text+0x770390>
1407711c4: 8b 49 30                    	movl	0x30(%rcx), %ecx
1407711c7: e8 84 20 0c 00              	callq	0x140833250 <.text+0x832250>
1407711cc: 4c 89 f1                    	movq	%r14, %rcx
1407711cf: 89 c2                       	movl	%eax, %edx
1407711d1: e8 8a ab 0b 00              	callq	0x14082bd60 <.text+0x82ad60>
1407711d6: f3 0f 6f 85 80 00 00 00     	movdqu	0x80(%rbp), %xmm0
1407711de: 66 0f 7f 45 f0              	movdqa	%xmm0, -0x10(%rbp)
1407711e3: e8 98 a9 8c ff              	callq	0x14003bb80 <.text+0x3ab80>
1407711e8: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
1407711ec: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
1407711f3: 49 8b 3f                    	movq	(%r15), %rdi
1407711f6: 48 8d 45 c8                 	leaq	-0x38(%rbp), %rax
1407711fa: 0f 11 00                    	movups	%xmm0, (%rax)
1407711fd: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
140771204: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140771208: f3 0f 6f 48 10              	movdqu	0x10(%rax), %xmm1
14077120d: f3 0f 6f 50 20              	movdqu	0x20(%rax), %xmm2
140771212: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
140771216: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
14077121a: f3 0f 7f 56 20              	movdqu	%xmm2, 0x20(%rsi)
14077121f: f3 0f 7f 4e 10              	movdqu	%xmm1, 0x10(%rsi)
140771224: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
140771228: 4c 89 f1                    	movq	%r14, %rcx
14077122b: 48 8b 95 a0 01 00 00        	movq	0x1a0(%rbp), %rdx
140771232: e8 69 34 5a 00              	callq	0x140d146a0 <.text+0xd136a0>
140771237: 48 8b 9d 88 00 00 00        	movq	0x88(%rbp), %rbx
14077123e: 48 85 db                    	testq	%rbx, %rbx
140771241: 75 07                       	jne	0x14077124a <.text+0x77024a>
140771243: 48 8b 9d 90 00 00 00        	movq	0x90(%rbp), %rbx
14077124a: 4c 8b ad a0 00 00 00        	movq	0xa0(%rbp), %r13
140771251: 48 8b 03                    	movq	(%rbx), %rax
140771254: 4c 8b 63 10                 	movq	0x10(%rbx), %r12
140771258: 4c 29 e0                    	subq	%r12, %rax
14077125b: 48 3d d7 00 00 00           	cmpq	$0xd7, %rax
140771261: 0f 86 fb 00 00 00           	jbe	0x140771362 <.text+0x770362>
140771267: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14077126b: 48 8d 0d 5e d5 a9 00        	leaq	0xa9d55e(%rip), %rcx    # 0x14120e7d0 <.text+0x120d7d0>
140771272: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
140771276: 42 0f 11 74 20 08           	movups	%xmm6, 0x8(%rax,%r12)
14077127c: 4a c7 44 20 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r12)
140771285: 42 c7 44 20 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r12)
14077128e: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140771298: 4a 89 4c 20 24              	movq	%rcx, 0x24(%rax,%r12)
14077129d: 42 c7 44 20 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r12) # imm = 0x3F800000
1407712a6: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
1407712aa: 42 0f 11 44 20 30           	movups	%xmm0, 0x30(%rax,%r12)
1407712b0: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
1407712b4: 4a 89 4c 20 40              	movq	%rcx, 0x40(%rax,%r12)
1407712b9: 42 c6 44 20 48 18           	movb	$0x18, 0x48(%rax,%r12)
1407712bf: 0f 10 45 00                 	movups	(%rbp), %xmm0
1407712c3: f3 0f 6f 4d 10              	movdqu	0x10(%rbp), %xmm1
1407712c8: f3 0f 6f 55 20              	movdqu	0x20(%rbp), %xmm2
1407712cd: 0f 10 5d 30                 	movups	0x30(%rbp), %xmm3
1407712d1: 42 0f 11 44 20 49           	movups	%xmm0, 0x49(%rax,%r12)
1407712d7: f3 42 0f 7f 4c 20 59        	movdqu	%xmm1, 0x59(%rax,%r12)
1407712de: f3 42 0f 7f 54 20 69        	movdqu	%xmm2, 0x69(%rax,%r12)
1407712e5: 42 0f 11 5c 20 79           	movups	%xmm3, 0x79(%rax,%r12)
1407712eb: f3 0f 6f 45 3f              	movdqu	0x3f(%rbp), %xmm0
1407712f0: f3 42 0f 7f 84 20 88 00 00 00       	movdqu	%xmm0, 0x88(%rax,%r12)
1407712fa: 4a 89 bc 20 98 00 00 00     	movq	%rdi, 0x98(%rax,%r12)
140771302: 42 c6 84 20 a0 00 00 00 02  	movb	$0x2, 0xa0(%rax,%r12)
14077130b: 42 c6 84 20 a8 00 00 00 00  	movb	$0x0, 0xa8(%rax,%r12)
140771314: 4e 89 ac 20 b8 00 00 00     	movq	%r13, 0xb8(%rax,%r12)
14077131c: 48 8d 0d 9d ec a9 00        	leaq	0xa9ec9d(%rip), %rcx    # 0x14120ffc0 <.text+0x120efc0>
140771323: 4a 89 8c 20 c8 00 00 00     	movq	%rcx, 0xc8(%rax,%r12)
14077132b: 49 81 c4 d8 00 00 00        	addq	$0xd8, %r12
140771332: 4c 89 63 10                 	movq	%r12, 0x10(%rbx)
140771336: 48 8b bd d8 00 00 00        	movq	0xd8(%rbp), %rdi
14077133d: 48 3b bd c8 00 00 00        	cmpq	0xc8(%rbp), %rdi
140771344: 0f 85 46 fe ff ff           	jne	0x140771190 <.text+0x770190>
14077134a: 48 8d 8d c8 00 00 00        	leaq	0xc8(%rbp), %rcx
140771351: 48 8d 15 58 d9 30 02        	leaq	0x230d958(%rip), %rdx   # 0x142a7ecb0
140771358: e8 93 0a 41 00              	callq	0x140b81df0 <.text+0xb80df0>
14077135d: e9 2e fe ff ff              	jmp	0x140771190 <.text+0x770190>
140771362: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14077136b: 41 b8 d8 00 00 00           	movl	$0xd8, %r8d
140771371: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140771377: 48 89 d9                    	movq	%rbx, %rcx
14077137a: 4c 89 e2                    	movq	%r12, %rdx
14077137d: e8 5e 6a 13 02              	callq	0x1428a7de0 <.text+0x28a6de0>
140771382: e9 e0 fe ff ff              	jmp	0x140771267 <.text+0x770267>
140771387: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140771390: 8b 85 f4 00 00 00           	movl	0xf4(%rbp), %eax
140771396: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
14077139d: 89 01                       	movl	%eax, (%rcx)
14077139f: 4d 8b 07                    	movq	(%r15), %r8
1407713a2: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
1407713a9: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1407713ad: f3 0f 6f 85 c8 00 00 00     	movdqu	0xc8(%rbp), %xmm0
1407713b5: 66 0f 7f 45 00              	movdqa	%xmm0, (%rbp)
1407713ba: 4c 89 f1                    	movq	%r14, %rcx
1407713bd: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1407713c4: 49 89 e9                    	movq	%rbp, %r9
1407713c7: e8 c4 b4 a8 00              	callq	0x1411fc890 <.text+0x11fb890>
1407713cc: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
1407713d3: 48 89 d0                    	movq	%rdx, %rax
1407713d6: 48 f7 d8                    	negq	%rax
1407713d9: 0f 80 01 fc ff ff           	jo	0x140770fe0 <.text+0x76ffe0>
1407713df: 0f 83 fb fb ff ff           	jae	0x140770fe0 <.text+0x76ffe0>
1407713e5: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
1407713ec: 48 c1 e2 03                 	shlq	$0x3, %rdx
1407713f0: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1407713f6: e8 25 28 ea ff              	callq	0x140613c20 <.text+0x612c20>
1407713fb: e9 e0 fb ff ff              	jmp	0x140770fe0 <.text+0x76ffe0>
140771400: 0f 28 b5 10 01 00 00        	movaps	0x110(%rbp), %xmm6
140771407: 0f 28 bd 20 01 00 00        	movaps	0x120(%rbp), %xmm7
14077140e: 48 81 c4 b8 01 00 00        	addq	$0x1b8, %rsp            # imm = 0x1B8
140771415: 5b                          	popq	%rbx
140771416: 5f                          	popq	%rdi
140771417: 5e                          	popq	%rsi
140771418: 41 5c                       	popq	%r12
14077141a: 41 5d                       	popq	%r13
14077141c: 41 5e                       	popq	%r14
14077141e: 41 5f                       	popq	%r15
140771420: 5d                          	popq	%rbp
140771421: c3                          	retq
140771422: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140771430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140771435: 55                          	pushq	%rbp
140771436: 41 57                       	pushq	%r15
140771438: 41 56                       	pushq	%r14
14077143a: 41 55                       	pushq	%r13
14077143c: 41 54                       	pushq	%r12
14077143e: 56                          	pushq	%rsi
14077143f: 57                          	pushq	%rdi
140771440: 53                          	pushq	%rbx
140771441: 48 83 ec 58                 	subq	$0x58, %rsp
140771445: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14077144c: 66 0f 7f 7c 24 30           	movdqa	%xmm7, 0x30(%rsp)
140771452: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
140771457: 48 83 bd e8 00 00 00 00     	cmpq	$0x0, 0xe8(%rbp)
14077145f: 74 1d                       	je	0x14077147e <.text+0x77047e>
140771461: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
140771468: 48 c1 e2 03                 	shlq	$0x3, %rdx
14077146c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140771472: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140771479: e8 a2 27 ea ff              	callq	0x140613c20 <.text+0x612c20>
14077147e: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140771483: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140771488: 48 83 c4 58                 	addq	$0x58, %rsp
14077148c: 5b                          	popq	%rbx
14077148d: 5f                          	popq	%rdi
14077148e: 5e                          	popq	%rsi
14077148f: 41 5c                       	popq	%r12
140771491: 41 5d                       	popq	%r13
140771493: 41 5e                       	popq	%r14
140771495: 41 5f                       	popq	%r15
140771497: 5d                          	popq	%rbp
140771498: c3                          	retq
140771499: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407714a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407714a5: 55                          	pushq	%rbp
1407714a6: 41 57                       	pushq	%r15
1407714a8: 41 56                       	pushq	%r14
1407714aa: 41 55                       	pushq	%r13
1407714ac: 41 54                       	pushq	%r12
1407714ae: 56                          	pushq	%rsi
1407714af: 57                          	pushq	%rdi
1407714b0: 53                          	pushq	%rbx
1407714b1: 48 83 ec 58                 	subq	$0x58, %rsp
1407714b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407714bc: 66 0f 7f 7c 24 30           	movdqa	%xmm7, 0x30(%rsp)
1407714c2: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1407714c7: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
1407714ce: 48 39 85 f8 00 00 00        	cmpq	%rax, 0xf8(%rbp)
1407714d5: 74 22                       	je	0x1407714f9 <.text+0x7704f9>
1407714d7: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
1407714de: 48 85 d2                    	testq	%rdx, %rdx
1407714e1: 74 16                       	je	0x1407714f9 <.text+0x7704f9>
1407714e3: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1407714ea: 48 c1 e2 03                 	shlq	$0x3, %rdx
1407714ee: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1407714f4: e8 27 27 ea ff              	callq	0x140613c20 <.text+0x612c20>
1407714f9: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1407714fe: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140771503: 48 83 c4 58                 	addq	$0x58, %rsp
140771507: 5b                          	popq	%rbx
140771508: 5f                          	popq	%rdi
140771509: 5e                          	popq	%rsi
14077150a: 41 5c                       	popq	%r12
14077150c: 41 5d                       	popq	%r13
14077150e: 41 5e                       	popq	%r14
140771510: 41 5f                       	popq	%r15
140771512: 5d                          	popq	%rbp
140771513: c3                          	retq
140771514: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140771520: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140771525: 55                          	pushq	%rbp
140771526: 41 57                       	pushq	%r15
140771528: 41 56                       	pushq	%r14
14077152a: 41 55                       	pushq	%r13
14077152c: 41 54                       	pushq	%r12
14077152e: 56                          	pushq	%rsi
14077152f: 57                          	pushq	%rdi
140771530: 53                          	pushq	%rbx
140771531: 48 83 ec 58                 	subq	$0x58, %rsp
140771535: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14077153c: 66 0f 7f 7c 24 30           	movdqa	%xmm7, 0x30(%rsp)
140771542: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
140771547: 48 83 7d 70 00              	cmpq	$0x0, 0x70(%rbp)
14077154c: 74 1a                       	je	0x140771568 <.text+0x770568>
14077154e: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140771552: 48 c1 e2 03                 	shlq	$0x3, %rdx
140771556: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14077155c: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
140771563: e8 b8 26 ea ff              	callq	0x140613c20 <.text+0x612c20>
140771568: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
14077156d: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140771572: 48 83 c4 58                 	addq	$0x58, %rsp
140771576: 5b                          	popq	%rbx
140771577: 5f                          	popq	%rdi
140771578: 5e                          	popq	%rsi
140771579: 41 5c                       	popq	%r12
14077157b: 41 5d                       	popq	%r13
14077157d: 41 5e                       	popq	%r14
14077157f: 41 5f                       	popq	%r15
140771581: 5d                          	popq	%rbp
140771582: c3                          	retq
140771583: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140771590: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140771595: 55                          	pushq	%rbp
140771596: 41 57                       	pushq	%r15
140771598: 41 56                       	pushq	%r14
14077159a: 41 55                       	pushq	%r13
14077159c: 41 54                       	pushq	%r12
14077159e: 56                          	pushq	%rsi
14077159f: 57                          	pushq	%rdi
1407715a0: 53                          	pushq	%rbx
1407715a1: 48 83 ec 58                 	subq	$0x58, %rsp
1407715a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407715ac: 66 0f 7f 7c 24 30           	movdqa	%xmm7, 0x30(%rsp)
1407715b2: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1407715b7: 80 bd 07 01 00 00 00        	cmpb	$0x0, 0x107(%rbp)
1407715be: 74 22                       	je	0x1407715e2 <.text+0x7705e2>
1407715c0: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
1407715c7: 48 85 d2                    	testq	%rdx, %rdx
1407715ca: 74 16                       	je	0x1407715e2 <.text+0x7705e2>
1407715cc: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1407715d3: 48 c1 e2 03                 	shlq	$0x3, %rdx
1407715d7: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1407715dd: e8 3e 26 ea ff              	callq	0x140613c20 <.text+0x612c20>
1407715e2: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1407715e7: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
1407715ec: 48 83 c4 58                 	addq	$0x58, %rsp
1407715f0: 5b                          	popq	%rbx
1407715f1: 5f                          	popq	%rdi
1407715f2: 5e                          	popq	%rsi
1407715f3: 41 5c                       	popq	%r12
1407715f5: 41 5d                       	popq	%r13
1407715f7: 41 5e                       	popq	%r14
1407715f9: 41 5f                       	popq	%r15
1407715fb: 5d                          	popq	%rbp
1407715fc: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a7abd0: 55                          	pushq	%rbp
140a7abd1: 41 57                       	pushq	%r15
140a7abd3: 41 56                       	pushq	%r14
140a7abd5: 41 55                       	pushq	%r13
140a7abd7: 41 54                       	pushq	%r12
140a7abd9: 56                          	pushq	%rsi
140a7abda: 57                          	pushq	%rdi
140a7abdb: 53                          	pushq	%rbx
140a7abdc: 48 81 ec b8 02 00 00        	subq	$0x2b8, %rsp            # imm = 0x2B8
140a7abe3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a7abeb: 0f 29 bd 20 02 00 00        	movaps	%xmm7, 0x220(%rbp)
140a7abf2: 0f 29 b5 10 02 00 00        	movaps	%xmm6, 0x210(%rbp)
140a7abf9: 48 c7 85 08 02 00 00 fe ff ff ff    	movq	$-0x2, 0x208(%rbp)
140a7ac04: 4c 89 85 00 02 00 00        	movq	%r8, 0x200(%rbp)
140a7ac0b: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140a7ac0f: 49 8b 01                    	movq	(%r9), %rax
140a7ac12: 4d 8b 69 08                 	movq	0x8(%r9), %r13
140a7ac16: 48 8b 10                    	movq	(%rax), %rdx
140a7ac19: 31 ff                       	xorl	%edi, %edi
140a7ac1b: 49 89 d0                    	movq	%rdx, %r8
140a7ac1e: 4d 2b 45 18                 	subq	0x18(%r13), %r8
140a7ac22: 4c 0f 42 c7                 	cmovbq	%rdi, %r8
140a7ac26: 49 2b 55 38                 	subq	0x38(%r13), %rdx
140a7ac2a: 48 0f 42 d7                 	cmovbq	%rdi, %rdx
140a7ac2e: 4d 8b 5d 10                 	movq	0x10(%r13), %r11
140a7ac32: 4d 8b 55 30                 	movq	0x30(%r13), %r10
140a7ac36: 4d 89 d1                    	movq	%r10, %r9
140a7ac39: 49 29 d1                    	subq	%rdx, %r9
140a7ac3c: 4c 0f 42 cf                 	cmovbq	%rdi, %r9
140a7ac40: 0f 96 c3                    	setbe	%bl
140a7ac43: 4d 89 df                    	movq	%r11, %r15
140a7ac46: 4d 29 c7                    	subq	%r8, %r15
140a7ac49: 4c 0f 42 ff                 	cmovbq	%rdi, %r15
140a7ac4d: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
140a7ac51: 49 8b 7d 40                 	movq	0x40(%r13), %rdi
140a7ac55: 41 0f 96 c6                 	setbe	%r14b
140a7ac59: 48 89 f9                    	movq	%rdi, %rcx
140a7ac5c: 4c 29 c9                    	subq	%r9, %rcx
140a7ac5f: 4c 29 f9                    	subq	%r15, %rcx
140a7ac62: 4c 8b a5 b8 02 00 00        	movq	0x2b8(%rbp), %r12
140a7ac69: 49 8b 34 24                 	movq	(%r12), %rsi
140a7ac6d: 48 89 b5 e8 00 00 00        	movq	%rsi, 0xe8(%rbp)
140a7ac74: 49 8b 74 24 10              	movq	0x10(%r12), %rsi
140a7ac79: 48 89 75 00                 	movq	%rsi, (%rbp)
140a7ac7d: 41 8b 74 24 1c              	movl	0x1c(%r12), %esi
140a7ac82: 89 b5 c4 00 00 00           	movl	%esi, 0xc4(%rbp)
140a7ac88: 49 8b 74 24 20              	movq	0x20(%r12), %rsi
140a7ac8d: 48 89 75 40                 	movq	%rsi, 0x40(%rbp)
140a7ac91: 49 8b 74 24 30              	movq	0x30(%r12), %rsi
140a7ac96: 48 89 75 50                 	movq	%rsi, 0x50(%rbp)
140a7ac9a: 41 8b 74 24 3c              	movl	0x3c(%r12), %esi
140a7ac9f: 89 b5 d8 00 00 00           	movl	%esi, 0xd8(%rbp)
140a7aca5: 49 8b 74 24 40              	movq	0x40(%r12), %rsi
140a7acaa: 48 89 75 30                 	movq	%rsi, 0x30(%rbp)
140a7acae: 49 8b 74 24 50              	movq	0x50(%r12), %rsi
140a7acb3: 48 89 75 38                 	movq	%rsi, 0x38(%rbp)
140a7acb7: 41 8b 74 24 5c              	movl	0x5c(%r12), %esi
140a7acbc: 89 b5 d4 00 00 00           	movl	%esi, 0xd4(%rbp)
140a7acc2: 49 8b 74 24 60              	movq	0x60(%r12), %rsi
140a7acc7: 48 89 75 10                 	movq	%rsi, 0x10(%rbp)
140a7accb: 49 8b 74 24 70              	movq	0x70(%r12), %rsi
140a7acd0: 48 89 75 20                 	movq	%rsi, 0x20(%rbp)
140a7acd4: 41 8b 74 24 7c              	movl	0x7c(%r12), %esi
140a7acd9: 89 b5 d0 00 00 00           	movl	%esi, 0xd0(%rbp)
140a7acdf: 49 8b b4 24 80 00 00 00     	movq	0x80(%r12), %rsi
140a7ace7: 48 89 75 f8                 	movq	%rsi, -0x8(%rbp)
140a7aceb: 49 8b b4 24 90 00 00 00     	movq	0x90(%r12), %rsi
140a7acf3: 48 89 75 18                 	movq	%rsi, 0x18(%rbp)
140a7acf7: 41 8b b4 24 9c 00 00 00     	movl	0x9c(%r12), %esi
140a7acff: 89 b5 cc 00 00 00           	movl	%esi, 0xcc(%rbp)
140a7ad05: 49 8b b4 24 a0 00 00 00     	movq	0xa0(%r12), %rsi
140a7ad0d: 48 89 75 f0                 	movq	%rsi, -0x10(%rbp)
140a7ad11: 49 8b b4 24 b0 00 00 00     	movq	0xb0(%r12), %rsi
140a7ad19: 48 89 75 08                 	movq	%rsi, 0x8(%rbp)
140a7ad1d: 41 8b b4 24 bc 00 00 00     	movl	0xbc(%r12), %esi
140a7ad25: 89 b5 c8 00 00 00           	movl	%esi, 0xc8(%rbp)
140a7ad2b: 4c 8b a5 c0 02 00 00        	movq	0x2c0(%rbp), %r12
140a7ad32: 49 8b 34 24                 	movq	(%r12), %rsi
140a7ad36: 48 89 75 58                 	movq	%rsi, 0x58(%rbp)
140a7ad3a: 49 8b 74 24 10              	movq	0x10(%r12), %rsi
140a7ad3f: 48 89 75 60                 	movq	%rsi, 0x60(%rbp)
140a7ad43: 41 8b 74 24 1c              	movl	0x1c(%r12), %esi
140a7ad48: 89 b5 dc 00 00 00           	movl	%esi, 0xdc(%rbp)
140a7ad4e: 49 8b 74 24 20              	movq	0x20(%r12), %rsi
140a7ad53: 48 89 75 68                 	movq	%rsi, 0x68(%rbp)
140a7ad57: 49 8b 74 24 30              	movq	0x30(%r12), %rsi
140a7ad5c: 48 89 75 70                 	movq	%rsi, 0x70(%rbp)
140a7ad60: 41 8b 74 24 3c              	movl	0x3c(%r12), %esi
140a7ad65: 89 b5 e0 00 00 00           	movl	%esi, 0xe0(%rbp)
140a7ad6b: 49 8b 74 24 40              	movq	0x40(%r12), %rsi
140a7ad70: 48 89 75 78                 	movq	%rsi, 0x78(%rbp)
140a7ad74: 49 8b 74 24 50              	movq	0x50(%r12), %rsi
140a7ad79: 45 8b 64 24 5c              	movl	0x5c(%r12), %r12d
140a7ad7e: 44 89 a5 e4 00 00 00        	movl	%r12d, 0xe4(%rbp)
140a7ad85: 4d 8b 65 08                 	movq	0x8(%r13), %r12
140a7ad89: 4d 8b 6d 28                 	movq	0x28(%r13), %r13
140a7ad8d: 48 89 08                    	movq	%rcx, (%rax)
140a7ad90: 41 84 de                    	testb	%bl, %r14b
140a7ad93: 0f 85 51 0d 00 00           	jne	0x140a7baea <.text+0xa7aaea>
140a7ad99: 48 89 d1                    	movq	%rdx, %rcx
140a7ad9c: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a7ada0: 49 01 cd                    	addq	%rcx, %r13
140a7ada3: 49 c1 e1 04                 	shlq	$0x4, %r9
140a7ada7: 4c 89 c1                    	movq	%r8, %rcx
140a7adaa: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a7adae: 49 01 cc                    	addq	%rcx, %r12
140a7adb1: 49 c1 e7 04                 	shlq	$0x4, %r15
140a7adb5: 31 c9                       	xorl	%ecx, %ecx
140a7adb7: 4d 39 c3                    	cmpq	%r8, %r11
140a7adba: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140a7adc0: 4d 0f 42 e0                 	cmovbq	%r8, %r12
140a7adc4: 4f 8d 5c 3c f0              	leaq	-0x10(%r12,%r15), %r11
140a7adc9: 49 0f 47 cb                 	cmovaq	%r11, %rcx
140a7adcd: 49 39 d2                    	cmpq	%rdx, %r10
140a7add0: 4d 0f 42 e8                 	cmovbq	%r8, %r13
140a7add4: 4f 8d 7c 0d f0              	leaq	-0x10(%r13,%r9), %r15
140a7add9: 4c 0f 46 f9                 	cmovbeq	%rcx, %r15
140a7addd: 48 89 38                    	movq	%rdi, (%rax)
140a7ade0: 48 8b 05 c1 15 bb 02        	movq	0x2bb15c1(%rip), %rax   # 0x14362c3a8
140a7ade7: 48 8b 00                    	movq	(%rax), %rax
140a7adea: 48 83 f8 02                 	cmpq	$0x2, %rax
140a7adee: 0f 86 b6 00 00 00           	jbe	0x140a7aeaa <.text+0xa79eaa>
140a7adf4: 48 8d 0d 7d c1 05 02        	leaq	0x205c17d(%rip), %rcx   # 0x142ad6f78
140a7adfb: e8 80 2f c0 ff              	callq	0x14067dd80 <.text+0x67cd80>
140a7ae00: 0f 10 00                    	movups	(%rax), %xmm0
140a7ae03: 8b 40 10                    	movl	0x10(%rax), %eax
140a7ae06: 48 c7 85 20 01 00 00 03 00 00 00    	movq	$0x3, 0x120(%rbp)
140a7ae11: 48 8d 0d 78 c1 05 02        	leaq	0x205c178(%rip), %rcx   # 0x142ad6f90
140a7ae18: 48 89 8d 28 01 00 00        	movq	%rcx, 0x128(%rbp)
140a7ae1f: 48 c7 85 30 01 00 00 23 00 00 00    	movq	$0x23, 0x130(%rbp)
140a7ae2a: 48 8d 15 07 c1 05 02        	leaq	0x205c107(%rip), %rdx   # 0x142ad6f38
140a7ae31: 48 89 95 40 01 00 00        	movq	%rdx, 0x140(%rbp)
140a7ae38: 48 c7 85 48 01 00 00 01 00 00 00    	movq	$0x1, 0x148(%rbp)
140a7ae43: 48 c7 85 50 01 00 00 08 00 00 00    	movq	$0x8, 0x150(%rbp)
140a7ae4e: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
140a7ae59: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a7ae5c: 0f 11 8d 58 01 00 00        	movups	%xmm1, 0x158(%rbp)
140a7ae63: 48 89 8d f8 00 00 00        	movq	%rcx, 0xf8(%rbp)
140a7ae6a: 48 c7 85 00 01 00 00 23 00 00 00    	movq	$0x23, 0x100(%rbp)
140a7ae75: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
140a7ae80: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
140a7ae87: c7 85 38 01 00 00 01 00 00 00       	movl	$0x1, 0x138(%rbp)
140a7ae91: 89 85 3c 01 00 00           	movl	%eax, 0x13c(%rbp)
140a7ae97: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
140a7ae9e: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140a7aea5: e8 86 f4 2c 00              	callq	0x140d4a330 <.text+0xd49330>
140a7aeaa: 45 0f b6 67 08              	movzbl	0x8(%r15), %r12d
140a7aeaf: e8 8c 28 2d 00              	callq	0x140d4d740 <.text+0xd4c740>
140a7aeb4: 49 89 c7                    	movq	%rax, %r15
140a7aeb7: f6 00 01                    	testb	$0x1, (%rax)
140a7aeba: 74 11                       	je	0x140a7aecd <.text+0xa79ecd>
140a7aebc: 4c 89 f9                    	movq	%r15, %rcx
140a7aebf: 48 83 c1 08                 	addq	$0x8, %rcx
140a7aec3: 49 8b 47 08                 	movq	0x8(%r15), %rax
140a7aec7: 49 8b 57 10                 	movq	0x10(%r15), %rdx
140a7aecb: eb 1b                       	jmp	0x140a7aee8 <.text+0xa79ee8>
140a7aecd: e8 2e 9f 2e 00              	callq	0x140d64e00 <.text+0xd63e00>
140a7aed2: 4c 89 f9                    	movq	%r15, %rcx
140a7aed5: 48 83 c1 08                 	addq	$0x8, %rcx
140a7aed9: 49 c7 07 01 00 00 00        	movq	$0x1, (%r15)
140a7aee0: 49 89 47 08                 	movq	%rax, 0x8(%r15)
140a7aee4: 49 89 57 10                 	movq	%rdx, 0x10(%r15)
140a7aee8: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140a7aeec: 4c 89 01                    	movq	%r8, (%rcx)
140a7aeef: 0f 10 35 2a bd 05 02        	movups	0x205bd2a(%rip), %xmm6  # 0x142ad6c20
140a7aef6: 0f 29 75 d0                 	movaps	%xmm6, -0x30(%rbp)
140a7aefa: 0f 10 3d 0f bd 05 02        	movups	0x205bd0f(%rip), %xmm7  # 0x142ad6c10
140a7af01: 0f 29 7d c0                 	movaps	%xmm7, -0x40(%rbp)
140a7af05: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140a7af09: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
140a7af0d: 31 c9                       	xorl	%ecx, %ecx
140a7af0f: e8 0c ec 65 ff              	callq	0x1400d9b20 <.text+0xd8b20>
140a7af14: 48 85 c0                    	testq	%rax, %rax
140a7af17: 0f 84 79 0c 00 00           	je	0x140a7bb96 <.text+0xa7ab96>
140a7af1d: 0f 10 00                    	movups	(%rax), %xmm0
140a7af20: 48 ff 00                    	incq	(%rax)
140a7af23: 0f 29 85 a0 00 00 00        	movaps	%xmm0, 0xa0(%rbp)
140a7af2a: 0f 10 05 cf 0f 1f 02        	movups	0x21f0fcf(%rip), %xmm0  # 0x142c6bf00
140a7af31: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
140a7af38: 0f 10 05 d1 0f 1f 02        	movups	0x21f0fd1(%rip), %xmm0  # 0x142c6bf10
140a7af3f: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
140a7af46: e8 f5 27 2d 00              	callq	0x140d4d740 <.text+0xd4c740>
140a7af4b: 49 89 c7                    	movq	%rax, %r15
140a7af4e: f6 00 01                    	testb	$0x1, (%rax)
140a7af51: 74 11                       	je	0x140a7af64 <.text+0xa79f64>
140a7af53: 4c 89 f9                    	movq	%r15, %rcx
140a7af56: 48 83 c1 08                 	addq	$0x8, %rcx
140a7af5a: 49 8b 47 08                 	movq	0x8(%r15), %rax
140a7af5e: 49 8b 57 10                 	movq	0x10(%r15), %rdx
140a7af62: eb 1b                       	jmp	0x140a7af7f <.text+0xa79f7f>
140a7af64: e8 97 9e 2e 00              	callq	0x140d64e00 <.text+0xd63e00>
140a7af69: 4c 89 f9                    	movq	%r15, %rcx
140a7af6c: 48 83 c1 08                 	addq	$0x8, %rcx
140a7af70: 49 c7 07 01 00 00 00        	movq	$0x1, (%r15)
140a7af77: 49 89 47 08                 	movq	%rax, 0x8(%r15)
140a7af7b: 49 89 57 10                 	movq	%rdx, 0x10(%r15)
140a7af7f: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140a7af83: 4c 89 01                    	movq	%r8, (%rcx)
140a7af86: 0f 29 b5 c0 01 00 00        	movaps	%xmm6, 0x1c0(%rbp)
140a7af8d: 0f 29 bd b0 01 00 00        	movaps	%xmm7, 0x1b0(%rbp)
140a7af94: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140a7af9b: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
140a7afa2: e8 99 27 2d 00              	callq	0x140d4d740 <.text+0xd4c740>
140a7afa7: 49 89 c7                    	movq	%rax, %r15
140a7afaa: f6 00 01                    	testb	$0x1, (%rax)
140a7afad: 74 18                       	je	0x140a7afc7 <.text+0xa79fc7>
140a7afaf: 4d 89 f8                    	movq	%r15, %r8
140a7afb2: 49 83 c0 08                 	addq	$0x8, %r8
140a7afb6: 49 8b 47 08                 	movq	0x8(%r15), %rax
140a7afba: 49 8b 4f 10                 	movq	0x10(%r15), %rcx
140a7afbe: 48 89 8d a8 01 00 00        	movq	%rcx, 0x1a8(%rbp)
140a7afc5: eb 22                       	jmp	0x140a7afe9 <.text+0xa79fe9>
140a7afc7: e8 34 9e 2e 00              	callq	0x140d64e00 <.text+0xd63e00>
140a7afcc: 4d 89 f8                    	movq	%r15, %r8
140a7afcf: 49 83 c0 08                 	addq	$0x8, %r8
140a7afd3: 49 c7 07 01 00 00 00        	movq	$0x1, (%r15)
140a7afda: 49 89 47 08                 	movq	%rax, 0x8(%r15)
140a7afde: 48 89 95 a8 01 00 00        	movq	%rdx, 0x1a8(%rbp)
140a7afe5: 49 89 57 10                 	movq	%rdx, 0x10(%r15)
140a7afe9: 48 8b 8d d8 02 00 00        	movq	0x2d8(%rbp), %rcx
140a7aff0: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140a7aff7: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
140a7affb: 49 89 10                    	movq	%rdx, (%r8)
140a7affe: 0f 28 45 c0                 	movaps	-0x40(%rbp), %xmm0
140a7b002: 0f 28 4d d0                 	movaps	-0x30(%rbp), %xmm1
140a7b006: 0f 28 55 e0                 	movaps	-0x20(%rbp), %xmm2
140a7b00a: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
140a7b011: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
140a7b018: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
140a7b01f: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
140a7b026: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
140a7b02d: 0f 28 95 a0 00 00 00        	movaps	0xa0(%rbp), %xmm2
140a7b034: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
140a7b03b: 0f 29 8d 30 01 00 00        	movaps	%xmm1, 0x130(%rbp)
140a7b042: 0f 29 95 40 01 00 00        	movaps	%xmm2, 0x140(%rbp)
140a7b049: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
140a7b050: 0f 28 8d c0 01 00 00        	movaps	0x1c0(%rbp), %xmm1
140a7b057: 0f 28 95 d0 01 00 00        	movaps	0x1d0(%rbp), %xmm2
140a7b05e: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
140a7b065: 0f 29 8d 60 01 00 00        	movaps	%xmm1, 0x160(%rbp)
140a7b06c: 0f 29 95 70 01 00 00        	movaps	%xmm2, 0x170(%rbp)
140a7b073: 0f 29 b5 90 01 00 00        	movaps	%xmm6, 0x190(%rbp)
140a7b07a: 0f 29 bd 80 01 00 00        	movaps	%xmm7, 0x180(%rbp)
140a7b081: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140a7b084: 48 8b 51 10                 	movq	0x10(%rcx), %rdx
140a7b088: 89 02                       	movl	%eax, (%rdx)
140a7b08a: 48 8b 09                    	movq	(%rcx), %rcx
140a7b08d: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
140a7b094: e8 47 fa b8 ff              	callq	0x14060aae0 <.text+0x609ae0>
140a7b099: 48 8b bd e0 02 00 00        	movq	0x2e0(%rbp), %rdi
140a7b0a0: 48 8b 9d f8 02 00 00        	movq	0x2f8(%rbp), %rbx
140a7b0a7: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140a7b0ae: 41 b8 b0 00 00 00           	movl	$0xb0, %r8d
140a7b0b4: 4c 8b b5 b8 00 00 00        	movq	0xb8(%rbp), %r14
140a7b0bb: 4c 89 f1                    	movq	%r14, %rcx
140a7b0be: e8 3d 8b ea 01              	callq	0x142923c00 <.text+0x2922c00>
140a7b0c3: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140a7b0ca: 49 89 86 b0 00 00 00        	movq	%rax, 0xb0(%r14)
140a7b0d1: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
140a7b0d8: 49 89 86 b8 00 00 00        	movq	%rax, 0xb8(%r14)
140a7b0df: 49 c7 86 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%r14)
140a7b0ea: 49 c7 86 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%r14)
140a7b0f5: 41 c6 86 e0 01 00 00 00     	movb	$0x0, 0x1e0(%r14)
140a7b0fd: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140a7b100: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
140a7b104: 89 01                       	movl	%eax, (%rcx)
140a7b106: 48 8b 07                    	movq	(%rdi), %rax
140a7b109: c6 40 10 00                 	movb	$0x0, 0x10(%rax)
140a7b10d: 8b 43 1c                    	movl	0x1c(%rbx), %eax
140a7b110: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140a7b114: 89 01                       	movl	%eax, (%rcx)
140a7b116: 48 8b 3b                    	movq	(%rbx), %rdi
140a7b119: 48 83 7f 18 00              	cmpq	$0x0, 0x18(%rdi)
140a7b11e: 74 3d                       	je	0x140a7b15d <.text+0xa7a15d>
140a7b120: 48 8b 5f 08                 	movq	0x8(%rdi), %rbx
140a7b124: 48 85 db                    	testq	%rbx, %rbx
140a7b127: 74 0e                       	je	0x140a7b137 <.text+0xa7a137>
140a7b129: 48 8b 0f                    	movq	(%rdi), %rcx
140a7b12c: 4c 8d 43 11                 	leaq	0x11(%rbx), %r8
140a7b130: b2 ff                       	movb	$-0x1, %dl
140a7b132: e8 e9 8a ea 01              	callq	0x142923c20 <.text+0x2922c20>
140a7b137: 48 c7 47 18 00 00 00 00     	movq	$0x0, 0x18(%rdi)
140a7b13f: 48 8d 43 01                 	leaq	0x1(%rbx), %rax
140a7b143: 48 89 c1                    	movq	%rax, %rcx
140a7b146: 48 c1 e9 03                 	shrq	$0x3, %rcx
140a7b14a: 48 83 e0 f8                 	andq	$-0x8, %rax
140a7b14e: 48 29 c8                    	subq	%rcx, %rax
140a7b151: 48 83 fb 08                 	cmpq	$0x8, %rbx
140a7b155: 48 0f 42 c3                 	cmovbq	%rbx, %rax
140a7b159: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
140a7b15d: 48 89 b5 a8 01 00 00        	movq	%rsi, 0x1a8(%rbp)
140a7b164: 4c 8b b5 a8 02 00 00        	movq	0x2a8(%rbp), %r14
140a7b16b: 4c 8b ad f0 02 00 00        	movq	0x2f0(%rbp), %r13
140a7b172: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140a7b176: 8b 78 1c                    	movl	0x1c(%rax), %edi
140a7b179: 48 8b 58 10                 	movq	0x10(%rax), %rbx
140a7b17d: 89 3b                       	movl	%edi, (%rbx)
140a7b17f: 48 8b 30                    	movq	(%rax), %rsi
140a7b182: 48 89 f1                    	movq	%rsi, %rcx
140a7b185: e8 a6 f5 dd ff              	callq	0x14085a730 <.text+0x859730>
140a7b18a: 45 84 e4                    	testb	%r12b, %r12b
140a7b18d: 75 0d                       	jne	0x140a7b19c <.text+0xa7a19c>
140a7b18f: 89 3b                       	movl	%edi, (%rbx)
140a7b191: 48 c7 86 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rsi)
140a7b19c: 49 8b 36                    	movq	(%r14), %rsi
140a7b19f: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140a7b1a3: 4c 8b 38                    	movq	(%rax), %r15
140a7b1a6: 49 8b 17                    	movq	(%r15), %rdx
140a7b1a9: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b1ad: 48 89 f1                    	movq	%rsi, %rcx
140a7b1b0: 41 b0 13                    	movb	$0x13, %r8b
140a7b1b3: e8 28 0d ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b1b8: 48 8b 45 50                 	movq	0x50(%rbp), %rax
140a7b1bc: 8b 8d d8 00 00 00           	movl	0xd8(%rbp), %ecx
140a7b1c2: 89 08                       	movl	%ecx, (%rax)
140a7b1c4: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a7b1c8: e8 d3 ab f9 ff              	callq	0x140a15da0 <.text+0xa14da0>
140a7b1cd: 49 8b 17                    	movq	(%r15), %rdx
140a7b1d0: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b1d4: 48 89 f1                    	movq	%rsi, %rcx
140a7b1d7: 41 b0 14                    	movb	$0x14, %r8b
140a7b1da: e8 01 0d ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b1df: 48 8b 45 38                 	movq	0x38(%rbp), %rax
140a7b1e3: 8b 8d d4 00 00 00           	movl	0xd4(%rbp), %ecx
140a7b1e9: 89 08                       	movl	%ecx, (%rax)
140a7b1eb: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140a7b1ef: e8 ac ab f9 ff              	callq	0x140a15da0 <.text+0xa14da0>
140a7b1f4: 49 8b 17                    	movq	(%r15), %rdx
140a7b1f7: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b1fb: 48 89 f1                    	movq	%rsi, %rcx
140a7b1fe: 41 b0 06                    	movb	$0x6, %r8b
140a7b201: e8 da 0c ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b206: 49 8b 17                    	movq	(%r15), %rdx
140a7b209: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b20d: 48 89 f1                    	movq	%rsi, %rcx
140a7b210: 41 b0 05                    	movb	$0x5, %r8b
140a7b213: e8 c8 0c ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b218: 49 8b 17                    	movq	(%r15), %rdx
140a7b21b: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b21f: 48 89 f1                    	movq	%rsi, %rcx
140a7b222: 41 b0 07                    	movb	$0x7, %r8b
140a7b225: e8 b6 0c ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b22a: 49 8b 17                    	movq	(%r15), %rdx
140a7b22d: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b231: 48 89 f1                    	movq	%rsi, %rcx
140a7b234: 41 b0 01                    	movb	$0x1, %r8b
140a7b237: e8 a4 0c ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b23c: 49 8b 17                    	movq	(%r15), %rdx
140a7b23f: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b243: 48 89 f1                    	movq	%rsi, %rcx
140a7b246: 41 b0 02                    	movb	$0x2, %r8b
140a7b249: e8 92 0c ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b24e: 49 8b 17                    	movq	(%r15), %rdx
140a7b251: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b255: 48 89 f1                    	movq	%rsi, %rcx
140a7b258: 41 b0 03                    	movb	$0x3, %r8b
140a7b25b: e8 80 0c ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b260: 49 8b 17                    	movq	(%r15), %rdx
140a7b263: 48 83 c2 10                 	addq	$0x10, %rdx
140a7b267: 48 89 f1                    	movq	%rsi, %rcx
140a7b26a: 41 b0 04                    	movb	$0x4, %r8b
140a7b26d: e8 6e 0c ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7b272: 48 8b 45 00                 	movq	(%rbp), %rax
140a7b276: 8b 8d c4 00 00 00           	movl	0xc4(%rbp), %ecx
140a7b27c: 89 08                       	movl	%ecx, (%rax)
140a7b27e: 48 8b 8d e8 00 00 00        	movq	0xe8(%rbp), %rcx
140a7b285: e8 16 ab f9 ff              	callq	0x140a15da0 <.text+0xa14da0>
140a7b28a: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140a7b28e: 8b 8d d0 00 00 00           	movl	0xd0(%rbp), %ecx
140a7b294: 89 08                       	movl	%ecx, (%rax)
140a7b296: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
140a7b29a: e8 01 ab f9 ff              	callq	0x140a15da0 <.text+0xa14da0>
140a7b29f: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140a7b2a3: 8b 8d cc 00 00 00           	movl	0xcc(%rbp), %ecx
140a7b2a9: 89 08                       	movl	%ecx, (%rax)
140a7b2ab: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140a7b2af: e8 ec aa f9 ff              	callq	0x140a15da0 <.text+0xa14da0>
140a7b2b4: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140a7b2b8: 8b 8d c8 00 00 00           	movl	0xc8(%rbp), %ecx
140a7b2be: 89 08                       	movl	%ecx, (%rax)
140a7b2c0: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140a7b2c4: e8 d7 aa f9 ff              	callq	0x140a15da0 <.text+0xa14da0>
140a7b2c9: 41 8b 45 1c                 	movl	0x1c(%r13), %eax
140a7b2cd: 49 8b 4d 10                 	movq	0x10(%r13), %rcx
140a7b2d1: 89 01                       	movl	%eax, (%rcx)
140a7b2d3: 49 8b 5d 00                 	movq	(%r13), %rbx
140a7b2d7: 48 89 d9                    	movq	%rbx, %rcx
140a7b2da: e8 a1 fb 02 00              	callq	0x140aaae80 <.text+0xaa9e80>
140a7b2df: 48 83 c3 30                 	addq	$0x30, %rbx
140a7b2e3: 48 89 d9                    	movq	%rbx, %rcx
140a7b2e6: e8 95 fb 02 00              	callq	0x140aaae80 <.text+0xaa9e80>
140a7b2eb: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7b2f2: e8 59 9e 02 00              	callq	0x140aa5150 <.text+0xaa4150>
140a7b2f7: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
140a7b2fe: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140a7b301: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140a7b305: 89 01                       	movl	%eax, (%rcx)
140a7b307: 48 8b 0a                    	movq	(%rdx), %rcx
140a7b30a: 48 89 8d 00 02 00 00        	movq	%rcx, 0x200(%rbp)
140a7b311: e8 9a 1c b9 ff              	callq	0x14060cfb0 <.text+0x60bfb0>
140a7b316: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140a7b31d: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140a7b324: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140a7b328: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140a7b32f: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140a7b333: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140a7b33a: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140a7b341: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140a7b348: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140a7b34f: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140a7b353: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140a7b357: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140a7b35b: 0f 11 00                    	movups	%xmm0, (%rax)
140a7b35e: 45 84 e4                    	testb	%r12b, %r12b
140a7b361: 48 8b 5d 58                 	movq	0x58(%rbp), %rbx
140a7b365: 0f 85 94 00 00 00           	jne	0x140a7b3ff <.text+0xa7a3ff>
140a7b36b: 48 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %rax
140a7b372: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a7b375: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
140a7b37c: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140a7b37f: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140a7b383: 89 0a                       	movl	%ecx, (%rdx)
140a7b385: 48 8b 08                    	movq	(%rax), %rcx
140a7b388: 48 89 8d 00 02 00 00        	movq	%rcx, 0x200(%rbp)
140a7b38f: e8 5c f9 e0 ff              	callq	0x14088acf0 <.text+0x889cf0>
140a7b394: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7b39b: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
140a7b3a2: 48 c7 41 08 08 00 00 00     	movq	$0x8, 0x8(%rcx)
140a7b3aa: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a7b3ad: 0f 11 41 10                 	movups	%xmm0, 0x10(%rcx)
140a7b3b1: 48 c7 41 20 08 00 00 00     	movq	$0x8, 0x20(%rcx)
140a7b3b9: 48 c7 41 28 00 00 00 00     	movq	$0x0, 0x28(%rcx)
140a7b3c1: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a7b3cb: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
140a7b3cf: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140a7b3d6: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140a7b3dd: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140a7b3e4: 0f 11 41 38                 	movups	%xmm0, 0x38(%rcx)
140a7b3e8: 0f 11 49 48                 	movups	%xmm1, 0x48(%rcx)
140a7b3ec: 0f 11 51 58                 	movups	%xmm2, 0x58(%rcx)
140a7b3f0: 48 c7 41 68 ff ff ff ff     	movq	$-0x1, 0x68(%rcx)
140a7b3f8: c7 41 70 00 00 00 00        	movl	$0x0, 0x70(%rcx)
140a7b3ff: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
140a7b406: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140a7b409: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140a7b40d: 89 0a                       	movl	%ecx, (%rdx)
140a7b40f: 48 8b 08                    	movq	(%rax), %rcx
140a7b412: 48 89 8d 00 02 00 00        	movq	%rcx, 0x200(%rbp)
140a7b419: e8 b2 9c e1 ff              	callq	0x1408950d0 <.text+0x8940d0>
140a7b41e: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140a7b425: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a7b42c: 48 c7 40 08 08 00 00 00     	movq	$0x8, 0x8(%rax)
140a7b434: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a7b437: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
140a7b43b: 48 c7 40 20 00 00 00 00     	movq	$0x0, 0x20(%rax)
140a7b443: c7 40 28 00 00 80 3f        	movl	$0x3f800000, 0x28(%rax) # imm = 0x3F800000
140a7b44a: 0f b6 05 b8 97 bb 02        	movzbl	0x2bb97b8(%rip), %eax   # 0x143634c09
140a7b451: b9 00 40 01 00              	movl	$0x14000, %ecx          # imm = 0x14000
140a7b456: ba 04 00 00 00              	movl	$0x4, %edx
140a7b45b: e8 b0 87 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7b460: 48 85 c0                    	testq	%rax, %rax
140a7b463: 0f 84 3e 07 00 00           	je	0x140a7bba7 <.text+0xa7aba7>
140a7b469: 48 c7 85 f0 00 00 00 00 50 00 00    	movq	$0x5000, 0xf0(%rbp) # imm = 0x5000
140a7b474: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140a7b47b: 48 c7 85 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbp)
140a7b486: 4c 8d 0d 83 dd 06 02        	leaq	0x206dd83(%rip), %r9    # 0x142ae9210
140a7b48d: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7b494: ba 00 50 00 00              	movl	$0x5000, %edx           # imm = 0x5000
140a7b499: 45 31 c0                    	xorl	%r8d, %r8d
140a7b49c: e8 7f 89 87 00              	callq	0x1412f3e20 <.text+0x12f2e20>
140a7b4a1: 48 8b bd a0 02 00 00        	movq	0x2a0(%rbp), %rdi
140a7b4a8: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140a7b4af: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140a7b4b6: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140a7b4bd: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
140a7b4c4: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a7b4c8: 8b 8d dc 00 00 00           	movl	0xdc(%rbp), %ecx
140a7b4ce: 89 08                       	movl	%ecx, (%rax)
140a7b4d0: 48 8b 13                    	movq	(%rbx), %rdx
140a7b4d3: 48 85 d2                    	testq	%rdx, %rdx
140a7b4d6: 74 13                       	je	0x140a7b4eb <.text+0xa7a4eb>
140a7b4d8: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140a7b4dc: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a7b4e0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7b4e6: e8 35 87 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7b4eb: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
140a7b4f2: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140a7b4f6: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
140a7b4fd: 0f 11 03                    	movups	%xmm0, (%rbx)
140a7b500: 48 8b 3f                    	movq	(%rdi), %rdi
140a7b503: 48 8d 15 be 2e 1b 02        	leaq	0x21b2ebe(%rip), %rdx   # 0x142c2e3c8
140a7b50a: 48 89 f9                    	movq	%rdi, %rcx
140a7b50d: e8 0e e2 ee ff              	callq	0x140969720 <.text+0x968720>
140a7b512: 48 85 c0                    	testq	%rax, %rax
140a7b515: 0f 84 57 06 00 00           	je	0x140a7bb72 <.text+0xa7ab72>
140a7b51b: 48 89 c3                    	movq	%rax, %rbx
140a7b51e: 48 8d 05 d0 1b 02 02        	leaq	0x2021bd0(%rip), %rax   # 0x142a9d0f5
140a7b525: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
140a7b52c: 48 c7 85 b8 01 00 00 32 00 00 00    	movq	$0x32, 0x1b8(%rbp)
140a7b537: 48 83 7b 18 32              	cmpq	$0x32, 0x18(%rbx)
140a7b53c: 0f 85 ca 05 00 00           	jne	0x140a7bb0c <.text+0xa7ab0c>
140a7b542: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140a7b546: 48 8d 0d a8 1b 02 02        	leaq	0x2021ba8(%rip), %rcx   # 0x142a9d0f5
140a7b54d: 41 b8 32 00 00 00           	movl	$0x32, %r8d
140a7b553: e8 98 86 ea 01              	callq	0x142923bf0 <.text+0x2922bf0>
140a7b558: 85 c0                       	testl	%eax, %eax
140a7b55a: 0f 85 ac 05 00 00           	jne	0x140a7bb0c <.text+0xa7ab0c>
140a7b560: 48 8b 0b                    	movq	(%rbx), %rcx
140a7b563: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140a7b567: ff 50 18                    	callq	*0x18(%rax)
140a7b56a: 48 85 c0                    	testq	%rax, %rax
140a7b56d: 0f 84 ff 05 00 00           	je	0x140a7bb72 <.text+0xa7ab72>
140a7b573: 48 89 c3                    	movq	%rax, %rbx
140a7b576: 4d 8b 37                    	movq	(%r15), %r14
140a7b579: 0f b6 05 89 96 bb 02        	movzbl	0x2bb9689(%rip), %eax   # 0x143634c09
140a7b580: b9 00 40 01 00              	movl	$0x14000, %ecx          # imm = 0x14000
140a7b585: ba 04 00 00 00              	movl	$0x4, %edx
140a7b58a: e8 81 86 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7b58f: 48 85 c0                    	testq	%rax, %rax
140a7b592: 0f 84 25 06 00 00           	je	0x140a7bbbd <.text+0xa7abbd>
140a7b598: 44 88 a5 e8 00 00 00        	movb	%r12b, 0xe8(%rbp)
140a7b59f: 41 b8 00 40 01 00           	movl	$0x14000, %r8d          # imm = 0x14000
140a7b5a5: 49 89 c5                    	movq	%rax, %r13
140a7b5a8: 48 89 c1                    	movq	%rax, %rcx
140a7b5ab: 31 d2                       	xorl	%edx, %edx
140a7b5ad: e8 6e 86 ea 01              	callq	0x142923c20 <.text+0x2922c20>
140a7b5b2: 48 81 7b 18 00 40 01 00     	cmpq	$0x14000, 0x18(%rbx)    # imm = 0x14000
140a7b5ba: 4c 89 ad 00 02 00 00        	movq	%r13, 0x200(%rbp)
140a7b5c1: 0f 82 0c 06 00 00           	jb	0x140a7bbd3 <.text+0xa7abd3>
140a7b5c7: 49 83 c6 10                 	addq	$0x10, %r14
140a7b5cb: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
140a7b5d6: 48 c7 85 88 00 00 00 00 40 01 00    	movq	$0x14000, 0x88(%rbp) # imm = 0x14000
140a7b5e1: 4c 8d a5 80 00 00 00        	leaq	0x80(%rbp), %r12
140a7b5e8: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140a7b5ed: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
140a7b5f6: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
140a7b5ff: 48 c7 44 24 20 00 40 01 00  	movq	$0x14000, 0x20(%rsp)    # imm = 0x14000
140a7b608: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7b60f: 4c 89 f2                    	movq	%r14, %rdx
140a7b612: 49 89 d8                    	movq	%rbx, %r8
140a7b615: 4d 89 e9                    	movq	%r13, %r9
140a7b618: e8 83 39 17 00              	callq	0x140beefa0 <.text+0xbedfa0>
140a7b61d: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140a7b624: 48 83 f8 0b                 	cmpq	$0xb, %rax
140a7b628: 48 8b 5d 68                 	movq	0x68(%rbp), %rbx
140a7b62c: 0f 85 a8 05 00 00           	jne	0x140a7bbda <.text+0xa7abda>
140a7b632: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7b637: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7b63d: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7b644: e8 d7 85 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7b649: 0f b6 05 b9 95 bb 02        	movzbl	0x2bb95b9(%rip), %eax   # 0x143634c09
140a7b650: b9 00 40 01 00              	movl	$0x14000, %ecx          # imm = 0x14000
140a7b655: ba 04 00 00 00              	movl	$0x4, %edx
140a7b65a: e8 b1 85 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7b65f: 48 85 c0                    	testq	%rax, %rax
140a7b662: 0f 84 35 06 00 00           	je	0x140a7bc9d <.text+0xa7ac9d>
140a7b668: 48 c7 85 f0 00 00 00 00 50 00 00    	movq	$0x5000, 0xf0(%rbp) # imm = 0x5000
140a7b673: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140a7b67a: 48 c7 85 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbp)
140a7b685: 4c 8d 0d 9c db 06 02        	leaq	0x206db9c(%rip), %r9    # 0x142ae9228
140a7b68c: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7b693: ba 00 50 00 00              	movl	$0x5000, %edx           # imm = 0x5000
140a7b698: 45 31 c0                    	xorl	%r8d, %r8d
140a7b69b: e8 80 87 87 00              	callq	0x1412f3e20 <.text+0x12f2e20>
140a7b6a0: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140a7b6a7: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140a7b6ae: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140a7b6b5: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
140a7b6bc: 48 8b 45 70                 	movq	0x70(%rbp), %rax
140a7b6c0: 8b 8d e0 00 00 00           	movl	0xe0(%rbp), %ecx
140a7b6c6: 89 08                       	movl	%ecx, (%rax)
140a7b6c8: 48 8b 13                    	movq	(%rbx), %rdx
140a7b6cb: 48 85 d2                    	testq	%rdx, %rdx
140a7b6ce: 74 13                       	je	0x140a7b6e3 <.text+0xa7a6e3>
140a7b6d0: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140a7b6d4: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a7b6d8: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7b6de: e8 3d 85 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7b6e3: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
140a7b6ea: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140a7b6ee: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
140a7b6f5: 0f 11 03                    	movups	%xmm0, (%rbx)
140a7b6f8: 48 8d 15 19 2c 1b 02        	leaq	0x21b2c19(%rip), %rdx   # 0x142c2e318
140a7b6ff: 48 89 f9                    	movq	%rdi, %rcx
140a7b702: e8 19 e0 ee ff              	callq	0x140969720 <.text+0x968720>
140a7b707: 48 85 c0                    	testq	%rax, %rax
140a7b70a: 0f 84 6e 04 00 00           	je	0x140a7bb7e <.text+0xa7ab7e>
140a7b710: 48 89 c3                    	movq	%rax, %rbx
140a7b713: 48 8d 05 2c 22 02 02        	leaq	0x202222c(%rip), %rax   # 0x142a9d946
140a7b71a: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
140a7b721: 48 c7 85 b8 01 00 00 32 00 00 00    	movq	$0x32, 0x1b8(%rbp)
140a7b72c: 48 83 7b 18 32              	cmpq	$0x32, 0x18(%rbx)
140a7b731: 0f 85 d5 03 00 00           	jne	0x140a7bb0c <.text+0xa7ab0c>
140a7b737: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140a7b73b: 48 8d 0d 04 22 02 02        	leaq	0x2022204(%rip), %rcx   # 0x142a9d946
140a7b742: 41 b8 32 00 00 00           	movl	$0x32, %r8d
140a7b748: e8 a3 84 ea 01              	callq	0x142923bf0 <.text+0x2922bf0>
140a7b74d: 85 c0                       	testl	%eax, %eax
140a7b74f: 0f 85 b7 03 00 00           	jne	0x140a7bb0c <.text+0xa7ab0c>
140a7b755: 48 8b 0b                    	movq	(%rbx), %rcx
140a7b758: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140a7b75c: ff 50 18                    	callq	*0x18(%rax)
140a7b75f: 48 85 c0                    	testq	%rax, %rax
140a7b762: 0f 84 16 04 00 00           	je	0x140a7bb7e <.text+0xa7ab7e>
140a7b768: 48 89 c3                    	movq	%rax, %rbx
140a7b76b: 4d 8b 37                    	movq	(%r15), %r14
140a7b76e: 0f b6 05 94 94 bb 02        	movzbl	0x2bb9494(%rip), %eax   # 0x143634c09
140a7b775: b9 00 40 01 00              	movl	$0x14000, %ecx          # imm = 0x14000
140a7b77a: ba 04 00 00 00              	movl	$0x4, %edx
140a7b77f: e8 8c 84 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7b784: 48 85 c0                    	testq	%rax, %rax
140a7b787: 0f 84 26 05 00 00           	je	0x140a7bcb3 <.text+0xa7acb3>
140a7b78d: 49 89 c5                    	movq	%rax, %r13
140a7b790: 41 b8 00 40 01 00           	movl	$0x14000, %r8d          # imm = 0x14000
140a7b796: 48 89 c1                    	movq	%rax, %rcx
140a7b799: 31 d2                       	xorl	%edx, %edx
140a7b79b: e8 80 84 ea 01              	callq	0x142923c20 <.text+0x2922c20>
140a7b7a0: 48 81 7b 18 00 40 01 00     	cmpq	$0x14000, 0x18(%rbx)    # imm = 0x14000
140a7b7a8: 4c 89 ad 00 02 00 00        	movq	%r13, 0x200(%rbp)
140a7b7af: 0f 82 14 05 00 00           	jb	0x140a7bcc9 <.text+0xa7acc9>
140a7b7b5: 49 83 c6 10                 	addq	$0x10, %r14
140a7b7b9: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
140a7b7c4: 48 c7 85 88 00 00 00 00 40 01 00    	movq	$0x14000, 0x88(%rbp) # imm = 0x14000
140a7b7cf: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140a7b7d4: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
140a7b7dd: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
140a7b7e6: 48 c7 44 24 20 00 40 01 00  	movq	$0x14000, 0x20(%rsp)    # imm = 0x14000
140a7b7ef: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7b7f6: 4c 89 f2                    	movq	%r14, %rdx
140a7b7f9: 49 89 d8                    	movq	%rbx, %r8
140a7b7fc: 4d 89 e9                    	movq	%r13, %r9
140a7b7ff: e8 9c 37 17 00              	callq	0x140beefa0 <.text+0xbedfa0>
140a7b804: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140a7b80b: 48 83 f8 0b                 	cmpq	$0xb, %rax
140a7b80f: 48 8b 5d 78                 	movq	0x78(%rbp), %rbx
140a7b813: 0f 85 b7 04 00 00           	jne	0x140a7bcd0 <.text+0xa7acd0>
140a7b819: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7b81e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7b824: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7b82b: e8 f0 83 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7b830: 0f b6 05 d2 93 bb 02        	movzbl	0x2bb93d2(%rip), %eax   # 0x143634c09
140a7b837: b9 00 40 01 00              	movl	$0x14000, %ecx          # imm = 0x14000
140a7b83c: ba 04 00 00 00              	movl	$0x4, %edx
140a7b841: e8 ca 83 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7b846: 48 85 c0                    	testq	%rax, %rax
140a7b849: 0f 84 44 05 00 00           	je	0x140a7bd93 <.text+0xa7ad93>
140a7b84f: 48 c7 85 f0 00 00 00 00 50 00 00    	movq	$0x5000, 0xf0(%rbp) # imm = 0x5000
140a7b85a: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140a7b861: 48 c7 85 00 01 00 00 00 00 00 00    	movq	$0x0, 0x100(%rbp)
140a7b86c: 4c 8d 0d cd d9 06 02        	leaq	0x206d9cd(%rip), %r9    # 0x142ae9240
140a7b873: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7b87a: ba 00 50 00 00              	movl	$0x5000, %edx           # imm = 0x5000
140a7b87f: 45 31 c0                    	xorl	%r8d, %r8d
140a7b882: e8 99 85 87 00              	callq	0x1412f3e20 <.text+0x12f2e20>
140a7b887: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140a7b88e: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140a7b895: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140a7b89c: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
140a7b8a3: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
140a7b8aa: 8b 8d e4 00 00 00           	movl	0xe4(%rbp), %ecx
140a7b8b0: 89 08                       	movl	%ecx, (%rax)
140a7b8b2: 48 8b 13                    	movq	(%rbx), %rdx
140a7b8b5: 48 85 d2                    	testq	%rdx, %rdx
140a7b8b8: 74 13                       	je	0x140a7b8cd <.text+0xa7a8cd>
140a7b8ba: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140a7b8be: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a7b8c2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7b8c8: e8 53 83 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7b8cd: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
140a7b8d4: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140a7b8d8: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
140a7b8df: 0f 11 03                    	movups	%xmm0, (%rbx)
140a7b8e2: 48 8d 15 57 29 1b 02        	leaq	0x21b2957(%rip), %rdx   # 0x142c2e240
140a7b8e9: 48 89 f9                    	movq	%rdi, %rcx
140a7b8ec: e8 2f de ee ff              	callq	0x140969720 <.text+0x968720>
140a7b8f1: 48 85 c0                    	testq	%rax, %rax
140a7b8f4: 0f 84 90 02 00 00           	je	0x140a7bb8a <.text+0xa7ab8a>
140a7b8fa: 48 89 c7                    	movq	%rax, %rdi
140a7b8fd: 48 8d 05 3a 19 02 02        	leaq	0x202193a(%rip), %rax   # 0x142a9d23e
140a7b904: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
140a7b90b: 48 c7 85 b8 01 00 00 38 00 00 00    	movq	$0x38, 0x1b8(%rbp)
140a7b916: 48 83 7f 18 38              	cmpq	$0x38, 0x18(%rdi)
140a7b91b: 0f 85 1e 02 00 00           	jne	0x140a7bb3f <.text+0xa7ab3f>
140a7b921: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140a7b925: 48 8d 0d 12 19 02 02        	leaq	0x2021912(%rip), %rcx   # 0x142a9d23e
140a7b92c: 41 b8 38 00 00 00           	movl	$0x38, %r8d
140a7b932: e8 b9 82 ea 01              	callq	0x142923bf0 <.text+0x2922bf0>
140a7b937: 85 c0                       	testl	%eax, %eax
140a7b939: 0f 85 00 02 00 00           	jne	0x140a7bb3f <.text+0xa7ab3f>
140a7b93f: 48 8b 0f                    	movq	(%rdi), %rcx
140a7b942: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140a7b946: ff 50 18                    	callq	*0x18(%rax)
140a7b949: 48 85 c0                    	testq	%rax, %rax
140a7b94c: 0f 84 38 02 00 00           	je	0x140a7bb8a <.text+0xa7ab8a>
140a7b952: 48 89 c7                    	movq	%rax, %rdi
140a7b955: 49 8b 1f                    	movq	(%r15), %rbx
140a7b958: 0f b6 05 aa 92 bb 02        	movzbl	0x2bb92aa(%rip), %eax   # 0x143634c09
140a7b95f: b9 00 40 01 00              	movl	$0x14000, %ecx          # imm = 0x14000
140a7b964: ba 04 00 00 00              	movl	$0x4, %edx
140a7b969: e8 a2 82 b9 ff              	callq	0x140613c10 <.text+0x612c10>
140a7b96e: 48 85 c0                    	testq	%rax, %rax
140a7b971: 0f 84 32 04 00 00           	je	0x140a7bda9 <.text+0xa7ada9>
140a7b977: 49 89 c6                    	movq	%rax, %r14
140a7b97a: 41 b8 00 40 01 00           	movl	$0x14000, %r8d          # imm = 0x14000
140a7b980: 48 89 c1                    	movq	%rax, %rcx
140a7b983: 31 d2                       	xorl	%edx, %edx
140a7b985: e8 96 82 ea 01              	callq	0x142923c20 <.text+0x2922c20>
140a7b98a: 48 81 7f 18 00 40 01 00     	cmpq	$0x14000, 0x18(%rdi)    # imm = 0x14000
140a7b992: 4c 89 b5 00 02 00 00        	movq	%r14, 0x200(%rbp)
140a7b999: 0f 82 20 04 00 00           	jb	0x140a7bdbf <.text+0xa7adbf>
140a7b99f: 48 83 c3 10                 	addq	$0x10, %rbx
140a7b9a3: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
140a7b9ae: 48 c7 85 88 00 00 00 00 40 01 00    	movq	$0x14000, 0x88(%rbp) # imm = 0x14000
140a7b9b9: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140a7b9be: 48 c7 44 24 38 00 00 00 00  	movq	$0x0, 0x38(%rsp)
140a7b9c7: 48 c7 44 24 30 01 00 00 00  	movq	$0x1, 0x30(%rsp)
140a7b9d0: 48 c7 44 24 20 00 40 01 00  	movq	$0x14000, 0x20(%rsp)    # imm = 0x14000
140a7b9d9: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7b9e0: 48 89 da                    	movq	%rbx, %rdx
140a7b9e3: 49 89 f8                    	movq	%rdi, %r8
140a7b9e6: 4d 89 f1                    	movq	%r14, %r9
140a7b9e9: e8 b2 35 17 00              	callq	0x140beefa0 <.text+0xbedfa0>
140a7b9ee: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140a7b9f5: 48 83 f8 0b                 	cmpq	$0xb, %rax
140a7b9f9: 44 0f b6 ad e8 00 00 00     	movzbl	0xe8(%rbp), %r13d
140a7ba01: 0f 85 bf 03 00 00           	jne	0x140a7bdc6 <.text+0xa7adc6>
140a7ba07: 48 8b 9d e8 02 00 00        	movq	0x2e8(%rbp), %rbx
140a7ba0e: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7ba13: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7ba19: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7ba20: e8 fb 81 b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7ba25: 45 84 ed                    	testb	%r13b, %r13b
140a7ba28: 75 28                       	jne	0x140a7ba52 <.text+0xa7aa52>
140a7ba2a: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
140a7ba31: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140a7ba34: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140a7ba38: 89 0a                       	movl	%ecx, (%rdx)
140a7ba3a: 48 8b 38                    	movq	(%rax), %rdi
140a7ba3d: 48 8d 4f 08                 	leaq	0x8(%rdi), %rcx
140a7ba41: e8 ba dc 0f 00              	callq	0x140b79700 <.text+0xb78700>
140a7ba46: 48 c7 47 38 00 00 00 00     	movq	$0x0, 0x38(%rdi)
140a7ba4e: c6 47 40 00                 	movb	$0x0, 0x40(%rdi)
140a7ba52: 49 8b 3f                    	movq	(%r15), %rdi
140a7ba55: 48 83 c7 10                 	addq	$0x10, %rdi
140a7ba59: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
140a7ba60: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140a7ba63: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140a7ba67: 89 01                       	movl	%eax, (%rcx)
140a7ba69: 48 89 f1                    	movq	%rsi, %rcx
140a7ba6c: 48 89 fa                    	movq	%rdi, %rdx
140a7ba6f: 41 b0 09                    	movb	$0x9, %r8b
140a7ba72: e8 69 04 ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7ba77: 48 89 f1                    	movq	%rsi, %rcx
140a7ba7a: 48 89 fa                    	movq	%rdi, %rdx
140a7ba7d: 41 b0 0b                    	movb	$0xb, %r8b
140a7ba80: e8 5b 04 ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7ba85: 48 89 f1                    	movq	%rsi, %rcx
140a7ba88: 48 89 fa                    	movq	%rdi, %rdx
140a7ba8b: 41 b0 0d                    	movb	$0xd, %r8b
140a7ba8e: e8 4d 04 ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7ba93: 48 89 f1                    	movq	%rsi, %rcx
140a7ba96: 48 89 fa                    	movq	%rdi, %rdx
140a7ba99: 41 b0 0e                    	movb	$0xe, %r8b
140a7ba9c: e8 3f 04 ec ff              	callq	0x14093bee0 <.text+0x93aee0>
140a7baa1: 48 8b 03                    	movq	(%rbx), %rax
140a7baa4: 48 85 c0                    	testq	%rax, %rax
140a7baa7: 74 41                       	je	0x140a7baea <.text+0xa7aaea>
140a7baa9: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140a7baad: 8b 53 1c                    	movl	0x1c(%rbx), %edx
140a7bab0: 89 11                       	movl	%edx, (%rcx)
140a7bab2: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140a7bab6: 48 85 c9                    	testq	%rcx, %rcx
140a7bab9: 74 2f                       	je	0x140a7baea <.text+0xa7aaea>
140a7babb: 48 8b 40 08                 	movq	0x8(%rax), %rax
140a7babf: 48 c1 e1 03                 	shlq	$0x3, %rcx
140a7bac3: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a7bac7: 31 d2                       	xorl	%edx, %edx
140a7bac9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a7bad0: 48 c7 44 10 08 00 00 00 00  	movq	$0x0, 0x8(%rax,%rdx)
140a7bad9: c7 44 10 10 00 00 00 00     	movl	$0x0, 0x10(%rax,%rdx)
140a7bae1: 48 83 c2 18                 	addq	$0x18, %rdx
140a7bae5: 48 39 d1                    	cmpq	%rdx, %rcx
140a7bae8: 75 e6                       	jne	0x140a7bad0 <.text+0xa7aad0>
140a7baea: 0f 28 b5 10 02 00 00        	movaps	0x210(%rbp), %xmm6
140a7baf1: 0f 28 bd 20 02 00 00        	movaps	0x220(%rbp), %xmm7
140a7baf8: 48 81 c4 b8 02 00 00        	addq	$0x2b8, %rsp            # imm = 0x2B8
140a7baff: 5b                          	popq	%rbx
140a7bb00: 5f                          	popq	%rdi
140a7bb01: 5e                          	popq	%rsi
140a7bb02: 41 5c                       	popq	%r12
140a7bb04: 41 5d                       	popq	%r13
140a7bb06: 41 5e                       	popq	%r14
140a7bb08: 41 5f                       	popq	%r15
140a7bb0a: 5d                          	popq	%rbp
140a7bb0b: c3                          	retq
140a7bb0c: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
140a7bb17: 48 83 c3 10                 	addq	$0x10, %rbx
140a7bb1b: 48 8d 05 5e 26 1b 02        	leaq	0x21b265e(%rip), %rax   # 0x142c2e180
140a7bb22: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7bb27: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
140a7bb2e: 4c 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %r9
140a7bb35: 31 c9                       	xorl	%ecx, %ecx
140a7bb37: 49 89 d8                    	movq	%rbx, %r8
140a7bb3a: e8 02 f0 e4 01              	callq	0x1428cab41 <.text+0x28c9b41>
140a7bb3f: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
140a7bb4a: 48 83 c7 10                 	addq	$0x10, %rdi
140a7bb4e: 48 8d 05 2b 26 1b 02        	leaq	0x21b262b(%rip), %rax   # 0x142c2e180
140a7bb55: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7bb5a: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
140a7bb61: 4c 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %r9
140a7bb68: 31 c9                       	xorl	%ecx, %ecx
140a7bb6a: 49 89 f8                    	movq	%rdi, %r8
140a7bb6d: e8 cf ef e4 01              	callq	0x1428cab41 <.text+0x28c9b41>
140a7bb72: 48 8d 0d 3f b4 05 02        	leaq	0x205b43f(%rip), %rcx   # 0x142ad6fb8
140a7bb79: e8 92 d7 e5 01              	callq	0x1428d9310 <.text+0x28d8310>
140a7bb7e: 48 8d 0d 7b b4 05 02        	leaq	0x205b47b(%rip), %rcx   # 0x142ad7000
140a7bb85: e8 86 d7 e5 01              	callq	0x1428d9310 <.text+0x28d8310>
140a7bb8a: 48 8d 0d b7 b4 05 02        	leaq	0x205b4b7(%rip), %rcx   # 0x142ad7048
140a7bb91: e8 7a d7 e5 01              	callq	0x1428d9310 <.text+0x28d8310>
140a7bb96: 48 8d 0d 03 ef 1b 02        	leaq	0x21bef03(%rip), %rcx   # 0x142c3aaa0
140a7bb9d: e8 2e c5 e5 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a7bba2: e9 dd 02 00 00              	jmp	0x140a7be84 <.text+0xa7ae84>
140a7bba7: 4c 8d 05 62 d6 06 02        	leaq	0x206d662(%rip), %r8    # 0x142ae9210
140a7bbae: b9 04 00 00 00              	movl	$0x4, %ecx
140a7bbb3: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7bbb8: e8 26 d4 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7bbbd: 4c 8d 05 0c b4 05 02        	leaq	0x205b40c(%rip), %r8    # 0x142ad6fd0
140a7bbc4: b9 04 00 00 00              	movl	$0x4, %ecx
140a7bbc9: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7bbce: e8 10 d4 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7bbd3: b8 0b 00 00 00              	movl	$0xb, %eax
140a7bbd8: eb 46                       	jmp	0x140a7bc20 <.text+0xa7ac20>
140a7bbda: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140a7bbe1: 48 89 8d f0 01 00 00        	movq	%rcx, 0x1f0(%rbp)
140a7bbe8: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
140a7bbef: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
140a7bbf6: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
140a7bbfd: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
140a7bc04: 0f 29 9d e0 01 00 00        	movaps	%xmm3, 0x1e0(%rbp)
140a7bc0b: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
140a7bc12: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
140a7bc19: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
140a7bc20: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
140a7bc27: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
140a7bc2e: 0f 28 8d c0 01 00 00        	movaps	0x1c0(%rbp), %xmm1
140a7bc35: 0f 28 95 d0 01 00 00        	movaps	0x1d0(%rbp), %xmm2
140a7bc3c: 0f 28 9d e0 01 00 00        	movaps	0x1e0(%rbp), %xmm3
140a7bc43: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
140a7bc4a: 0f 11 8d 08 01 00 00        	movups	%xmm1, 0x108(%rbp)
140a7bc51: 0f 11 95 18 01 00 00        	movups	%xmm2, 0x118(%rbp)
140a7bc58: 0f 11 9d 28 01 00 00        	movups	%xmm3, 0x128(%rbp)
140a7bc5f: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140a7bc66: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140a7bc6d: 48 8d 05 74 b3 05 02        	leaq	0x205b374(%rip), %rax   # 0x142ad6fe8
140a7bc74: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7bc79: 48 8d 0d 20 ad 05 02        	leaq	0x205ad20(%rip), %rcx   # 0x142ad69a0
140a7bc80: 4c 8d 0d 49 ad 05 02        	leaq	0x205ad49(%rip), %r9    # 0x142ad69d0
140a7bc87: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
140a7bc8e: ba 2b 00 00 00              	movl	$0x2b, %edx
140a7bc93: e8 c8 da e5 01              	callq	0x1428d9760 <.text+0x28d8760>
140a7bc98: e9 e7 01 00 00              	jmp	0x140a7be84 <.text+0xa7ae84>
140a7bc9d: 4c 8d 05 84 d5 06 02        	leaq	0x206d584(%rip), %r8    # 0x142ae9228
140a7bca4: b9 04 00 00 00              	movl	$0x4, %ecx
140a7bca9: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7bcae: e8 30 d3 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7bcb3: 4c 8d 05 5e b3 05 02        	leaq	0x205b35e(%rip), %r8    # 0x142ad7018
140a7bcba: b9 04 00 00 00              	movl	$0x4, %ecx
140a7bcbf: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7bcc4: e8 1a d3 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7bcc9: b8 0b 00 00 00              	movl	$0xb, %eax
140a7bcce: eb 46                       	jmp	0x140a7bd16 <.text+0xa7ad16>
140a7bcd0: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140a7bcd7: 48 89 8d f0 01 00 00        	movq	%rcx, 0x1f0(%rbp)
140a7bcde: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
140a7bce5: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
140a7bcec: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
140a7bcf3: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
140a7bcfa: 0f 29 9d e0 01 00 00        	movaps	%xmm3, 0x1e0(%rbp)
140a7bd01: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
140a7bd08: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
140a7bd0f: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
140a7bd16: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
140a7bd1d: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
140a7bd24: 0f 28 8d c0 01 00 00        	movaps	0x1c0(%rbp), %xmm1
140a7bd2b: 0f 28 95 d0 01 00 00        	movaps	0x1d0(%rbp), %xmm2
140a7bd32: 0f 28 9d e0 01 00 00        	movaps	0x1e0(%rbp), %xmm3
140a7bd39: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
140a7bd40: 0f 11 8d 08 01 00 00        	movups	%xmm1, 0x108(%rbp)
140a7bd47: 0f 11 95 18 01 00 00        	movups	%xmm2, 0x118(%rbp)
140a7bd4e: 0f 11 9d 28 01 00 00        	movups	%xmm3, 0x128(%rbp)
140a7bd55: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140a7bd5c: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140a7bd63: 48 8d 05 c6 b2 05 02        	leaq	0x205b2c6(%rip), %rax   # 0x142ad7030
140a7bd6a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7bd6f: 48 8d 0d 2a ac 05 02        	leaq	0x205ac2a(%rip), %rcx   # 0x142ad69a0
140a7bd76: 4c 8d 0d 53 ac 05 02        	leaq	0x205ac53(%rip), %r9    # 0x142ad69d0
140a7bd7d: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
140a7bd84: ba 2b 00 00 00              	movl	$0x2b, %edx
140a7bd89: e8 d2 d9 e5 01              	callq	0x1428d9760 <.text+0x28d8760>
140a7bd8e: e9 f1 00 00 00              	jmp	0x140a7be84 <.text+0xa7ae84>
140a7bd93: 4c 8d 05 a6 d4 06 02        	leaq	0x206d4a6(%rip), %r8    # 0x142ae9240
140a7bd9a: b9 04 00 00 00              	movl	$0x4, %ecx
140a7bd9f: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7bda4: e8 3a d2 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7bda9: 4c 8d 05 b0 b2 05 02        	leaq	0x205b2b0(%rip), %r8    # 0x142ad7060
140a7bdb0: b9 04 00 00 00              	movl	$0x4, %ecx
140a7bdb5: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7bdba: e8 24 d2 e5 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a7bdbf: b8 0b 00 00 00              	movl	$0xb, %eax
140a7bdc4: eb 46                       	jmp	0x140a7be0c <.text+0xa7ae0c>
140a7bdc6: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140a7bdcd: 48 89 8d f0 01 00 00        	movq	%rcx, 0x1f0(%rbp)
140a7bdd4: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
140a7bddb: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
140a7bde2: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
140a7bde9: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
140a7bdf0: 0f 29 9d e0 01 00 00        	movaps	%xmm3, 0x1e0(%rbp)
140a7bdf7: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
140a7bdfe: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
140a7be05: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
140a7be0c: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
140a7be13: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
140a7be1a: 0f 28 8d c0 01 00 00        	movaps	0x1c0(%rbp), %xmm1
140a7be21: 0f 28 95 d0 01 00 00        	movaps	0x1d0(%rbp), %xmm2
140a7be28: 0f 28 9d e0 01 00 00        	movaps	0x1e0(%rbp), %xmm3
140a7be2f: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
140a7be36: 0f 11 8d 08 01 00 00        	movups	%xmm1, 0x108(%rbp)
140a7be3d: 0f 11 95 18 01 00 00        	movups	%xmm2, 0x118(%rbp)
140a7be44: 0f 11 9d 28 01 00 00        	movups	%xmm3, 0x128(%rbp)
140a7be4b: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
140a7be52: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
140a7be59: 48 8d 05 18 b2 05 02        	leaq	0x205b218(%rip), %rax   # 0x142ad7078
140a7be60: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a7be65: 48 8d 0d 34 ab 05 02        	leaq	0x205ab34(%rip), %rcx   # 0x142ad69a0
140a7be6c: 4c 8d 0d 5d ab 05 02        	leaq	0x205ab5d(%rip), %r9    # 0x142ad69d0
140a7be73: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
140a7be7a: ba 2b 00 00 00              	movl	$0x2b, %edx
140a7be7f: e8 dc d8 e5 01              	callq	0x1428d9760 <.text+0x28d8760>
140a7be84: 0f 0b                       	ud2
140a7be86: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a7be90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7be95: 55                          	pushq	%rbp
140a7be96: 41 57                       	pushq	%r15
140a7be98: 41 56                       	pushq	%r14
140a7be9a: 41 55                       	pushq	%r13
140a7be9c: 41 54                       	pushq	%r12
140a7be9e: 56                          	pushq	%rsi
140a7be9f: 57                          	pushq	%rdi
140a7bea0: 53                          	pushq	%rbx
140a7bea1: 48 83 ec 68                 	subq	$0x68, %rsp
140a7bea5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7beac: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7beb1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7beb6: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a7beba: 41 b8 98 00 00 00           	movl	$0x98, %r8d
140a7bec0: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140a7bec6: 48 8d 55 e0                 	leaq	-0x20(%rbp), %rdx
140a7beca: e8 31 4a 5d ff              	callq	0x140050900 <.text+0x4f900>
140a7becf: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7bed4: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7bed9: 48 83 c4 68                 	addq	$0x68, %rsp
140a7bedd: 5b                          	popq	%rbx
140a7bede: 5f                          	popq	%rdi
140a7bedf: 5e                          	popq	%rsi
140a7bee0: 41 5c                       	popq	%r12
140a7bee2: 41 5d                       	popq	%r13
140a7bee4: 41 5e                       	popq	%r14
140a7bee6: 41 5f                       	popq	%r15
140a7bee8: 5d                          	popq	%rbp
140a7bee9: c3                          	retq
140a7beea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a7bef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7bef5: 55                          	pushq	%rbp
140a7bef6: 41 57                       	pushq	%r15
140a7bef8: 41 56                       	pushq	%r14
140a7befa: 41 55                       	pushq	%r13
140a7befc: 41 54                       	pushq	%r12
140a7befe: 56                          	pushq	%rsi
140a7beff: 57                          	pushq	%rdi
140a7bf00: 53                          	pushq	%rbx
140a7bf01: 48 83 ec 68                 	subq	$0x68, %rsp
140a7bf05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7bf0c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7bf11: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7bf16: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140a7bf1d: e8 1e a7 b5 ff              	callq	0x1405d6640 <.text+0x5d5640>
140a7bf22: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7bf27: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7bf2c: 48 83 c4 68                 	addq	$0x68, %rsp
140a7bf30: 5b                          	popq	%rbx
140a7bf31: 5f                          	popq	%rdi
140a7bf32: 5e                          	popq	%rsi
140a7bf33: 41 5c                       	popq	%r12
140a7bf35: 41 5d                       	popq	%r13
140a7bf37: 41 5e                       	popq	%r14
140a7bf39: 41 5f                       	popq	%r15
140a7bf3b: 5d                          	popq	%rbp
140a7bf3c: c3                          	retq
140a7bf3d: 0f 1f 00                    	nopl	(%rax)
140a7bf40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7bf45: 55                          	pushq	%rbp
140a7bf46: 41 57                       	pushq	%r15
140a7bf48: 41 56                       	pushq	%r14
140a7bf4a: 41 55                       	pushq	%r13
140a7bf4c: 41 54                       	pushq	%r12
140a7bf4e: 56                          	pushq	%rsi
140a7bf4f: 57                          	pushq	%rdi
140a7bf50: 53                          	pushq	%rbx
140a7bf51: 48 83 ec 68                 	subq	$0x68, %rsp
140a7bf55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7bf5c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7bf61: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7bf66: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
140a7bf6d: 41 b8 50 00 00 00           	movl	$0x50, %r8d
140a7bf73: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140a7bf79: 48 8d 95 d0 01 00 00        	leaq	0x1d0(%rbp), %rdx
140a7bf80: e8 bb 3f 5d ff              	callq	0x14004ff40 <.text+0x4ef40>
140a7bf85: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7bf8a: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7bf8f: 48 83 c4 68                 	addq	$0x68, %rsp
140a7bf93: 5b                          	popq	%rbx
140a7bf94: 5f                          	popq	%rdi
140a7bf95: 5e                          	popq	%rsi
140a7bf96: 41 5c                       	popq	%r12
140a7bf98: 41 5d                       	popq	%r13
140a7bf9a: 41 5e                       	popq	%r14
140a7bf9c: 41 5f                       	popq	%r15
140a7bf9e: 5d                          	popq	%rbp
140a7bf9f: c3                          	retq
140a7bfa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7bfa5: 55                          	pushq	%rbp
140a7bfa6: 41 57                       	pushq	%r15
140a7bfa8: 41 56                       	pushq	%r14
140a7bfaa: 41 55                       	pushq	%r13
140a7bfac: 41 54                       	pushq	%r12
140a7bfae: 56                          	pushq	%rsi
140a7bfaf: 57                          	pushq	%rdi
140a7bfb0: 53                          	pushq	%rbx
140a7bfb1: 48 83 ec 68                 	subq	$0x68, %rsp
140a7bfb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7bfbc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7bfc1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7bfc6: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
140a7bfcd: 48 85 d2                    	testq	%rdx, %rdx
140a7bfd0: 74 16                       	je	0x140a7bfe8 <.text+0xa7afe8>
140a7bfd2: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140a7bfd9: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a7bfdd: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7bfe3: e8 38 7c b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7bfe8: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7bfed: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7bff2: 48 83 c4 68                 	addq	$0x68, %rsp
140a7bff6: 5b                          	popq	%rbx
140a7bff7: 5f                          	popq	%rdi
140a7bff8: 5e                          	popq	%rsi
140a7bff9: 41 5c                       	popq	%r12
140a7bffb: 41 5d                       	popq	%r13
140a7bffd: 41 5e                       	popq	%r14
140a7bfff: 41 5f                       	popq	%r15
140a7c001: 5d                          	popq	%rbp
140a7c002: c3                          	retq
140a7c003: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a7c010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c015: 55                          	pushq	%rbp
140a7c016: 41 57                       	pushq	%r15
140a7c018: 41 56                       	pushq	%r14
140a7c01a: 41 55                       	pushq	%r13
140a7c01c: 41 54                       	pushq	%r12
140a7c01e: 56                          	pushq	%rsi
140a7c01f: 57                          	pushq	%rdi
140a7c020: 53                          	pushq	%rbx
140a7c021: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c025: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c02c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c031: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c036: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7c03b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7c041: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7c048: e8 d3 7b b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7c04d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c052: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c057: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c05b: 5b                          	popq	%rbx
140a7c05c: 5f                          	popq	%rdi
140a7c05d: 5e                          	popq	%rsi
140a7c05e: 41 5c                       	popq	%r12
140a7c060: 41 5d                       	popq	%r13
140a7c062: 41 5e                       	popq	%r14
140a7c064: 41 5f                       	popq	%r15
140a7c066: 5d                          	popq	%rbp
140a7c067: c3                          	retq
140a7c068: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a7c070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c075: 55                          	pushq	%rbp
140a7c076: 41 57                       	pushq	%r15
140a7c078: 41 56                       	pushq	%r14
140a7c07a: 41 55                       	pushq	%r13
140a7c07c: 41 54                       	pushq	%r12
140a7c07e: 56                          	pushq	%rsi
140a7c07f: 57                          	pushq	%rdi
140a7c080: 53                          	pushq	%rbx
140a7c081: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c085: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c08c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c091: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c096: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
140a7c09d: 48 85 d2                    	testq	%rdx, %rdx
140a7c0a0: 74 16                       	je	0x140a7c0b8 <.text+0xa7b0b8>
140a7c0a2: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140a7c0a9: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a7c0ad: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7c0b3: e8 68 7b b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7c0b8: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c0bd: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c0c2: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c0c6: 5b                          	popq	%rbx
140a7c0c7: 5f                          	popq	%rdi
140a7c0c8: 5e                          	popq	%rsi
140a7c0c9: 41 5c                       	popq	%r12
140a7c0cb: 41 5d                       	popq	%r13
140a7c0cd: 41 5e                       	popq	%r14
140a7c0cf: 41 5f                       	popq	%r15
140a7c0d1: 5d                          	popq	%rbp
140a7c0d2: c3                          	retq
140a7c0d3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a7c0e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c0e5: 55                          	pushq	%rbp
140a7c0e6: 41 57                       	pushq	%r15
140a7c0e8: 41 56                       	pushq	%r14
140a7c0ea: 41 55                       	pushq	%r13
140a7c0ec: 41 54                       	pushq	%r12
140a7c0ee: 56                          	pushq	%rsi
140a7c0ef: 57                          	pushq	%rdi
140a7c0f0: 53                          	pushq	%rbx
140a7c0f1: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c0f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c0fc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c101: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c106: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7c10b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7c111: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7c118: e8 03 7b b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7c11d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c122: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c127: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c12b: 5b                          	popq	%rbx
140a7c12c: 5f                          	popq	%rdi
140a7c12d: 5e                          	popq	%rsi
140a7c12e: 41 5c                       	popq	%r12
140a7c130: 41 5d                       	popq	%r13
140a7c132: 41 5e                       	popq	%r14
140a7c134: 41 5f                       	popq	%r15
140a7c136: 5d                          	popq	%rbp
140a7c137: c3                          	retq
140a7c138: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a7c140: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c145: 55                          	pushq	%rbp
140a7c146: 41 57                       	pushq	%r15
140a7c148: 41 56                       	pushq	%r14
140a7c14a: 41 55                       	pushq	%r13
140a7c14c: 41 54                       	pushq	%r12
140a7c14e: 56                          	pushq	%rsi
140a7c14f: 57                          	pushq	%rdi
140a7c150: 53                          	pushq	%rbx
140a7c151: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c155: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c15c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c161: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c166: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
140a7c16d: 48 85 d2                    	testq	%rdx, %rdx
140a7c170: 74 16                       	je	0x140a7c188 <.text+0xa7b188>
140a7c172: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140a7c179: 48 c1 e2 02                 	shlq	$0x2, %rdx
140a7c17d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7c183: e8 98 7a b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7c188: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c18d: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c192: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c196: 5b                          	popq	%rbx
140a7c197: 5f                          	popq	%rdi
140a7c198: 5e                          	popq	%rsi
140a7c199: 41 5c                       	popq	%r12
140a7c19b: 41 5d                       	popq	%r13
140a7c19d: 41 5e                       	popq	%r14
140a7c19f: 41 5f                       	popq	%r15
140a7c1a1: 5d                          	popq	%rbp
140a7c1a2: c3                          	retq
140a7c1a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a7c1b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c1b5: 55                          	pushq	%rbp
140a7c1b6: 41 57                       	pushq	%r15
140a7c1b8: 41 56                       	pushq	%r14
140a7c1ba: 41 55                       	pushq	%r13
140a7c1bc: 41 54                       	pushq	%r12
140a7c1be: 56                          	pushq	%rsi
140a7c1bf: 57                          	pushq	%rdi
140a7c1c0: 53                          	pushq	%rbx
140a7c1c1: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c1c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c1cc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c1d1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c1d6: ba 00 40 01 00              	movl	$0x14000, %edx          # imm = 0x14000
140a7c1db: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a7c1e1: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7c1e8: e8 33 7a b9 ff              	callq	0x140613c20 <.text+0x612c20>
140a7c1ed: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c1f2: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c1f7: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c1fb: 5b                          	popq	%rbx
140a7c1fc: 5f                          	popq	%rdi
140a7c1fd: 5e                          	popq	%rsi
140a7c1fe: 41 5c                       	popq	%r12
140a7c200: 41 5d                       	popq	%r13
140a7c202: 41 5e                       	popq	%r14
140a7c204: 41 5f                       	popq	%r15
140a7c206: 5d                          	popq	%rbp
140a7c207: c3                          	retq
140a7c208: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a7c210: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c215: 55                          	pushq	%rbp
140a7c216: 41 57                       	pushq	%r15
140a7c218: 41 56                       	pushq	%r14
140a7c21a: 41 55                       	pushq	%r13
140a7c21c: 41 54                       	pushq	%r12
140a7c21e: 56                          	pushq	%rsi
140a7c21f: 57                          	pushq	%rdi
140a7c220: 53                          	pushq	%rbx
140a7c221: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c225: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c22c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c231: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c236: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
140a7c23d: 41 b8 b0 00 00 00           	movl	$0xb0, %r8d
140a7c243: 48 8b b5 b8 00 00 00        	movq	0xb8(%rbp), %rsi
140a7c24a: 48 89 f1                    	movq	%rsi, %rcx
140a7c24d: e8 ae 79 ea 01              	callq	0x142923c00 <.text+0x2922c00>
140a7c252: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140a7c259: 48 89 86 b0 00 00 00        	movq	%rax, 0xb0(%rsi)
140a7c260: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
140a7c267: 48 89 86 b8 00 00 00        	movq	%rax, 0xb8(%rsi)
140a7c26e: 48 c7 86 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rsi)
140a7c279: 48 c7 86 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rsi)
140a7c284: c6 86 e0 01 00 00 00        	movb	$0x0, 0x1e0(%rsi)
140a7c28b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c290: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c295: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c299: 5b                          	popq	%rbx
140a7c29a: 5f                          	popq	%rdi
140a7c29b: 5e                          	popq	%rsi
140a7c29c: 41 5c                       	popq	%r12
140a7c29e: 41 5d                       	popq	%r13
140a7c2a0: 41 5e                       	popq	%r14
140a7c2a2: 41 5f                       	popq	%r15
140a7c2a4: 5d                          	popq	%rbp
140a7c2a5: c3                          	retq
140a7c2a6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a7c2b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c2b5: 55                          	pushq	%rbp
140a7c2b6: 41 57                       	pushq	%r15
140a7c2b8: 41 56                       	pushq	%r14
140a7c2ba: 41 55                       	pushq	%r13
140a7c2bc: 41 54                       	pushq	%r12
140a7c2be: 56                          	pushq	%rsi
140a7c2bf: 57                          	pushq	%rdi
140a7c2c0: 53                          	pushq	%rbx
140a7c2c1: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c2c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c2cc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c2d1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c2d6: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140a7c2dd: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140a7c2e4: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140a7c2e8: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140a7c2ef: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140a7c2f3: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140a7c2fa: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140a7c301: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140a7c308: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140a7c30f: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140a7c313: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140a7c317: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140a7c31b: 0f 11 00                    	movups	%xmm0, (%rax)
140a7c31e: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c323: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c328: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c32c: 5b                          	popq	%rbx
140a7c32d: 5f                          	popq	%rdi
140a7c32e: 5e                          	popq	%rsi
140a7c32f: 41 5c                       	popq	%r12
140a7c331: 41 5d                       	popq	%r13
140a7c333: 41 5e                       	popq	%r14
140a7c335: 41 5f                       	popq	%r15
140a7c337: 5d                          	popq	%rbp
140a7c338: c3                          	retq
140a7c339: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a7c340: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c345: 55                          	pushq	%rbp
140a7c346: 41 57                       	pushq	%r15
140a7c348: 41 56                       	pushq	%r14
140a7c34a: 41 55                       	pushq	%r13
140a7c34c: 41 54                       	pushq	%r12
140a7c34e: 56                          	pushq	%rsi
140a7c34f: 57                          	pushq	%rdi
140a7c350: 53                          	pushq	%rbx
140a7c351: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c355: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c35c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c361: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c366: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a7c36d: 48 c7 01 00 00 00 00        	movq	$0x0, (%rcx)
140a7c374: 48 c7 41 08 08 00 00 00     	movq	$0x8, 0x8(%rcx)
140a7c37c: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a7c37f: 0f 11 41 10                 	movups	%xmm0, 0x10(%rcx)
140a7c383: 48 c7 41 20 08 00 00 00     	movq	$0x8, 0x20(%rcx)
140a7c38b: 48 c7 41 28 00 00 00 00     	movq	$0x0, 0x28(%rcx)
140a7c393: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a7c39d: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
140a7c3a1: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140a7c3a8: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140a7c3af: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140a7c3b6: 0f 11 41 38                 	movups	%xmm0, 0x38(%rcx)
140a7c3ba: 0f 11 49 48                 	movups	%xmm1, 0x48(%rcx)
140a7c3be: 0f 11 51 58                 	movups	%xmm2, 0x58(%rcx)
140a7c3c2: 48 c7 41 68 ff ff ff ff     	movq	$-0x1, 0x68(%rcx)
140a7c3ca: c7 41 70 00 00 00 00        	movl	$0x0, 0x70(%rcx)
140a7c3d1: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c3d6: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c3db: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c3df: 5b                          	popq	%rbx
140a7c3e0: 5f                          	popq	%rdi
140a7c3e1: 5e                          	popq	%rsi
140a7c3e2: 41 5c                       	popq	%r12
140a7c3e4: 41 5d                       	popq	%r13
140a7c3e6: 41 5e                       	popq	%r14
140a7c3e8: 41 5f                       	popq	%r15
140a7c3ea: 5d                          	popq	%rbp
140a7c3eb: c3                          	retq
140a7c3ec: 0f 1f 40 00                 	nopl	(%rax)
140a7c3f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c3f5: 55                          	pushq	%rbp
140a7c3f6: 41 57                       	pushq	%r15
140a7c3f8: 41 56                       	pushq	%r14
140a7c3fa: 41 55                       	pushq	%r13
140a7c3fc: 41 54                       	pushq	%r12
140a7c3fe: 56                          	pushq	%rsi
140a7c3ff: 57                          	pushq	%rdi
140a7c400: 53                          	pushq	%rbx
140a7c401: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c405: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c40c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c411: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c416: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
140a7c41d: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140a7c424: 48 c7 40 08 08 00 00 00     	movq	$0x8, 0x8(%rax)
140a7c42c: 0f 57 c0                    	xorps	%xmm0, %xmm0
140a7c42f: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
140a7c433: 48 c7 40 20 00 00 00 00     	movq	$0x0, 0x20(%rax)
140a7c43b: c7 40 28 00 00 80 3f        	movl	$0x3f800000, 0x28(%rax) # imm = 0x3F800000
140a7c442: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c447: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c44c: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c450: 5b                          	popq	%rbx
140a7c451: 5f                          	popq	%rdi
140a7c452: 5e                          	popq	%rsi
140a7c453: 41 5c                       	popq	%r12
140a7c455: 41 5d                       	popq	%r13
140a7c457: 41 5e                       	popq	%r14
140a7c459: 41 5f                       	popq	%r15
140a7c45b: 5d                          	popq	%rbp
140a7c45c: c3                          	retq
140a7c45d: 0f 1f 00                    	nopl	(%rax)
140a7c460: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c465: 55                          	pushq	%rbp
140a7c466: 41 57                       	pushq	%r15
140a7c468: 41 56                       	pushq	%r14
140a7c46a: 41 55                       	pushq	%r13
140a7c46c: 41 54                       	pushq	%r12
140a7c46e: 56                          	pushq	%rsi
140a7c46f: 57                          	pushq	%rdi
140a7c470: 53                          	pushq	%rbx
140a7c471: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c475: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c47c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c481: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c486: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7c48d: e8 7e 53 d3 ff              	callq	0x1407b1810 <.text+0x7b0810>
140a7c492: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c497: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c49c: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c4a0: 5b                          	popq	%rbx
140a7c4a1: 5f                          	popq	%rdi
140a7c4a2: 5e                          	popq	%rsi
140a7c4a3: 41 5c                       	popq	%r12
140a7c4a5: 41 5d                       	popq	%r13
140a7c4a7: 41 5e                       	popq	%r14
140a7c4a9: 41 5f                       	popq	%r15
140a7c4ab: 5d                          	popq	%rbp
140a7c4ac: c3                          	retq
140a7c4ad: 0f 1f 00                    	nopl	(%rax)
140a7c4b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c4b5: 55                          	pushq	%rbp
140a7c4b6: 41 57                       	pushq	%r15
140a7c4b8: 41 56                       	pushq	%r14
140a7c4ba: 41 55                       	pushq	%r13
140a7c4bc: 41 54                       	pushq	%r12
140a7c4be: 56                          	pushq	%rsi
140a7c4bf: 57                          	pushq	%rdi
140a7c4c0: 53                          	pushq	%rbx
140a7c4c1: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c4c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c4cc: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c4d1: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c4d6: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7c4dd: e8 2e 53 d3 ff              	callq	0x1407b1810 <.text+0x7b0810>
140a7c4e2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c4e7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c4ec: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c4f0: 5b                          	popq	%rbx
140a7c4f1: 5f                          	popq	%rdi
140a7c4f2: 5e                          	popq	%rsi
140a7c4f3: 41 5c                       	popq	%r12
140a7c4f5: 41 5d                       	popq	%r13
140a7c4f7: 41 5e                       	popq	%r14
140a7c4f9: 41 5f                       	popq	%r15
140a7c4fb: 5d                          	popq	%rbp
140a7c4fc: c3                          	retq
140a7c4fd: 0f 1f 00                    	nopl	(%rax)
140a7c500: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a7c505: 55                          	pushq	%rbp
140a7c506: 41 57                       	pushq	%r15
140a7c508: 41 56                       	pushq	%r14
140a7c50a: 41 55                       	pushq	%r13
140a7c50c: 41 54                       	pushq	%r12
140a7c50e: 56                          	pushq	%rsi
140a7c50f: 57                          	pushq	%rdi
140a7c510: 53                          	pushq	%rbx
140a7c511: 48 83 ec 68                 	subq	$0x68, %rsp
140a7c515: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a7c51c: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140a7c521: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a7c526: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a7c52d: e8 de 52 d3 ff              	callq	0x1407b1810 <.text+0x7b0810>
140a7c532: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a7c537: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a7c53c: 48 83 c4 68                 	addq	$0x68, %rsp
140a7c540: 5b                          	popq	%rbx
140a7c541: 5f                          	popq	%rdi
140a7c542: 5e                          	popq	%rsi
140a7c543: 41 5c                       	popq	%r12
140a7c545: 41 5d                       	popq	%r13
140a7c547: 41 5e                       	popq	%r14
140a7c549: 41 5f                       	popq	%r15
140a7c54b: 5d                          	popq	%rbp
140a7c54c: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa9380: 55                          	pushq	%rbp
140aa9381: 41 57                       	pushq	%r15
140aa9383: 41 56                       	pushq	%r14
140aa9385: 41 55                       	pushq	%r13
140aa9387: 41 54                       	pushq	%r12
140aa9389: 56                          	pushq	%rsi
140aa938a: 57                          	pushq	%rdi
140aa938b: 53                          	pushq	%rbx
140aa938c: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140aa9393: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140aa939b: 48 c7 45 10 fe ff ff ff     	movq	$-0x2, 0x10(%rbp)
140aa93a3: 4c 89 45 f0                 	movq	%r8, -0x10(%rbp)
140aa93a7: 48 89 d6                    	movq	%rdx, %rsi
140aa93aa: 48 89 cb                    	movq	%rcx, %rbx
140aa93ad: 48 83 c1 20                 	addq	$0x20, %rcx
140aa93b1: e8 ba 79 b0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140aa93b6: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140aa93ba: 48 89 da                    	movq	%rbx, %rdx
140aa93bd: 49 89 c0                    	movq	%rax, %r8
140aa93c0: 49 89 f1                    	movq	%rsi, %r9
140aa93c3: e8 28 e4 0c 00              	callq	0x140b777f0 <.text+0xb767f0>
140aa93c8: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
140aa93cc: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140aa93d0: 48 f7 d8                    	negq	%rax
140aa93d3: 0f 80 aa 00 00 00           	jo	0x140aa9483 <.text+0xaa8483>
140aa93d9: 48 89 75 c0                 	movq	%rsi, -0x40(%rbp)
140aa93dd: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
140aa93e1: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140aa93e5: 4c 8b 7d b8                 	movq	-0x48(%rbp), %r15
140aa93e9: 4d 85 ff                    	testq	%r15, %r15
140aa93ec: 74 75                       	je	0x140aa9463 <.text+0xaa8463>
140aa93ee: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140aa93f2: 4c 8b 30                    	movq	(%rax), %r14
140aa93f5: 48 8b 48 08                 	movq	0x8(%rax), %rcx
140aa93f9: 48 89 4d 00                 	movq	%rcx, (%rbp)
140aa93fd: 48 8b 70 18                 	movq	0x18(%rax), %rsi
140aa9401: 4c 8b 60 10                 	movq	0x10(%rax), %r12
140aa9405: 49 c1 e7 03                 	shlq	$0x3, %r15
140aa9409: 31 ff                       	xorl	%edi, %edi
140aa940b: eb 0c                       	jmp	0x140aa9419 <.text+0xaa8419>
140aa940d: 0f 1f 00                    	nopl	(%rax)
140aa9410: 48 83 c7 08                 	addq	$0x8, %rdi
140aa9414: 49 39 ff                    	cmpq	%rdi, %r15
140aa9417: 74 4a                       	je	0x140aa9463 <.text+0xaa8463>
140aa9419: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140aa941d: 4c 8b 2c 38                 	movq	(%rax,%rdi), %r13
140aa9421: 4c 89 e8                    	movq	%r13, %rax
140aa9424: 48 c1 e8 20                 	shrq	$0x20, %rax
140aa9428: 74 39                       	je	0x140aa9463 <.text+0xaa8463>
140aa942a: 4c 89 f1                    	movq	%r14, %rcx
140aa942d: 4c 89 ea                    	movq	%r13, %rdx
140aa9430: e8 6b 1b 24 00              	callq	0x140ceafa0 <.text+0xce9fa0>
140aa9435: 84 c0                       	testb	%al, %al
140aa9437: 74 d7                       	je	0x140aa9410 <.text+0xaa8410>
140aa9439: 4c 89 75 c8                 	movq	%r14, -0x38(%rbp)
140aa943d: 48 8b 45 00                 	movq	(%rbp), %rax
140aa9441: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140aa9445: 4c 89 65 d8                 	movq	%r12, -0x28(%rbp)
140aa9449: 48 89 75 e0                 	movq	%rsi, -0x20(%rbp)
140aa944d: 4c 89 6d e8                 	movq	%r13, -0x18(%rbp)
140aa9451: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
140aa9455: 48 8d 15 6c 3f 03 02        	leaq	0x2033f6c(%rip), %rdx   # 0x142add3c8
140aa945c: e8 0f b3 26 00              	callq	0x140d14770 <.text+0xd13770>
140aa9461: eb ad                       	jmp	0x140aa9410 <.text+0xaa8410>
140aa9463: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140aa9467: 48 85 d2                    	testq	%rdx, %rdx
140aa946a: 74 13                       	je	0x140aa947f <.text+0xaa847f>
140aa946c: 48 c1 e2 03                 	shlq	$0x3, %rdx
140aa9470: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140aa9476: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140aa947a: e8 a1 a7 b6 ff              	callq	0x140613c20 <.text+0x612c20>
140aa947f: 48 8b 75 c0                 	movq	-0x40(%rbp), %rsi
140aa9483: 4c 8d 73 30                 	leaq	0x30(%rbx), %r14
140aa9487: 48 83 c3 50                 	addq	$0x50, %rbx
140aa948b: 48 89 d9                    	movq	%rbx, %rcx
140aa948e: 48 89 f2                    	movq	%rsi, %rdx
140aa9491: e8 da 78 b0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140aa9496: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140aa949a: 4c 89 f2                    	movq	%r14, %rdx
140aa949d: 49 89 c0                    	movq	%rax, %r8
140aa94a0: 49 89 f1                    	movq	%rsi, %r9
140aa94a3: e8 48 e3 0c 00              	callq	0x140b777f0 <.text+0xb767f0>
140aa94a8: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
140aa94ac: 48 89 45 00                 	movq	%rax, (%rbp)
140aa94b0: 48 f7 d8                    	negq	%rax
140aa94b3: 0f 80 a2 00 00 00           	jo	0x140aa955b <.text+0xaa855b>
140aa94b9: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
140aa94bd: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140aa94c1: 4c 8b 7d b8                 	movq	-0x48(%rbp), %r15
140aa94c5: 4d 85 ff                    	testq	%r15, %r15
140aa94c8: 74 75                       	je	0x140aa953f <.text+0xaa853f>
140aa94ca: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140aa94ce: 48 8b 38                    	movq	(%rax), %rdi
140aa94d1: 4c 8b 60 08                 	movq	0x8(%rax), %r12
140aa94d5: 4c 8b 68 18                 	movq	0x18(%rax), %r13
140aa94d9: 48 8b 70 10                 	movq	0x10(%rax), %rsi
140aa94dd: 49 c1 e7 03                 	shlq	$0x3, %r15
140aa94e1: 31 db                       	xorl	%ebx, %ebx
140aa94e3: eb 14                       	jmp	0x140aa94f9 <.text+0xaa84f9>
140aa94e5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140aa94f0: 48 83 c3 08                 	addq	$0x8, %rbx
140aa94f4: 49 39 df                    	cmpq	%rbx, %r15
140aa94f7: 74 46                       	je	0x140aa953f <.text+0xaa853f>
140aa94f9: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140aa94fd: 4c 8b 34 18                 	movq	(%rax,%rbx), %r14
140aa9501: 4c 89 f0                    	movq	%r14, %rax
140aa9504: 48 c1 e8 20                 	shrq	$0x20, %rax
140aa9508: 74 35                       	je	0x140aa953f <.text+0xaa853f>
140aa950a: 48 89 f9                    	movq	%rdi, %rcx
140aa950d: 4c 89 f2                    	movq	%r14, %rdx
140aa9510: e8 8b 1a 24 00              	callq	0x140ceafa0 <.text+0xce9fa0>
140aa9515: 84 c0                       	testb	%al, %al
140aa9517: 74 d7                       	je	0x140aa94f0 <.text+0xaa84f0>
140aa9519: 48 89 7d c8                 	movq	%rdi, -0x38(%rbp)
140aa951d: 4c 89 65 d0                 	movq	%r12, -0x30(%rbp)
140aa9521: 48 89 75 d8                 	movq	%rsi, -0x28(%rbp)
140aa9525: 4c 89 6d e0                 	movq	%r13, -0x20(%rbp)
140aa9529: 4c 89 75 e8                 	movq	%r14, -0x18(%rbp)
140aa952d: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
140aa9531: 48 8d 15 a8 3e 03 02        	leaq	0x2033ea8(%rip), %rdx   # 0x142add3e0
140aa9538: e8 33 b2 26 00              	callq	0x140d14770 <.text+0xd13770>
140aa953d: eb b1                       	jmp	0x140aa94f0 <.text+0xaa84f0>
140aa953f: 48 8b 55 00                 	movq	(%rbp), %rdx
140aa9543: 48 85 d2                    	testq	%rdx, %rdx
140aa9546: 74 13                       	je	0x140aa955b <.text+0xaa855b>
140aa9548: 48 c1 e2 03                 	shlq	$0x3, %rdx
140aa954c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140aa9552: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140aa9556: e8 c5 a6 b6 ff              	callq	0x140613c20 <.text+0x612c20>
140aa955b: 90                          	nop
140aa955c: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
140aa9563: 5b                          	popq	%rbx
140aa9564: 5f                          	popq	%rdi
140aa9565: 5e                          	popq	%rsi
140aa9566: 41 5c                       	popq	%r12
140aa9568: 41 5d                       	popq	%r13
140aa956a: 41 5e                       	popq	%r14
140aa956c: 41 5f                       	popq	%r15
140aa956e: 5d                          	popq	%rbp
140aa956f: c3                          	retq
140aa9570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140aa9575: 55                          	pushq	%rbp
140aa9576: 41 57                       	pushq	%r15
140aa9578: 41 56                       	pushq	%r14
140aa957a: 41 55                       	pushq	%r13
140aa957c: 41 54                       	pushq	%r12
140aa957e: 56                          	pushq	%rsi
140aa957f: 57                          	pushq	%rdi
140aa9580: 53                          	pushq	%rbx
140aa9581: 48 83 ec 28                 	subq	$0x28, %rsp
140aa9585: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140aa958c: 48 83 7d f8 00              	cmpq	$0x0, -0x8(%rbp)
140aa9591: 74 17                       	je	0x140aa95aa <.text+0xaa85aa>
140aa9593: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140aa9597: 48 c1 e2 03                 	shlq	$0x3, %rdx
140aa959b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140aa95a1: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140aa95a5: e8 76 a6 b6 ff              	callq	0x140613c20 <.text+0x612c20>
140aa95aa: 90                          	nop
140aa95ab: 48 83 c4 28                 	addq	$0x28, %rsp
140aa95af: 5b                          	popq	%rbx
140aa95b0: 5f                          	popq	%rdi
140aa95b1: 5e                          	popq	%rsi
140aa95b2: 41 5c                       	popq	%r12
140aa95b4: 41 5d                       	popq	%r13
140aa95b6: 41 5e                       	popq	%r14
140aa95b8: 41 5f                       	popq	%r15
140aa95ba: 5d                          	popq	%rbp
140aa95bb: c3                          	retq
140aa95bc: 0f 1f 40 00                 	nopl	(%rax)
140aa95c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140aa95c5: 55                          	pushq	%rbp
140aa95c6: 41 57                       	pushq	%r15
140aa95c8: 41 56                       	pushq	%r14
140aa95ca: 41 55                       	pushq	%r13
140aa95cc: 41 54                       	pushq	%r12
140aa95ce: 56                          	pushq	%rsi
140aa95cf: 57                          	pushq	%rdi
140aa95d0: 53                          	pushq	%rbx
140aa95d1: 48 83 ec 28                 	subq	$0x28, %rsp
140aa95d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140aa95dc: 48 83 7d 00 00              	cmpq	$0x0, (%rbp)
140aa95e1: 74 17                       	je	0x140aa95fa <.text+0xaa85fa>
140aa95e3: 48 8b 55 00                 	movq	(%rbp), %rdx
140aa95e7: 48 c1 e2 03                 	shlq	$0x3, %rdx
140aa95eb: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140aa95f1: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140aa95f5: e8 26 a6 b6 ff              	callq	0x140613c20 <.text+0x612c20>
140aa95fa: 90                          	nop
140aa95fb: 48 83 c4 28                 	addq	$0x28, %rsp
140aa95ff: 5b                          	popq	%rbx
140aa9600: 5f                          	popq	%rdi
140aa9601: 5e                          	popq	%rsi
140aa9602: 41 5c                       	popq	%r12
140aa9604: 41 5d                       	popq	%r13
140aa9606: 41 5e                       	popq	%r14
140aa9608: 41 5f                       	popq	%r15
140aa960a: 5d                          	popq	%rbp
140aa960b: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1411fcad0: 41 57                       	pushq	%r15
1411fcad2: 41 56                       	pushq	%r14
1411fcad4: 56                          	pushq	%rsi
1411fcad5: 57                          	pushq	%rdi
1411fcad6: 55                          	pushq	%rbp
1411fcad7: 53                          	pushq	%rbx
1411fcad8: 48 83 ec 48                 	subq	$0x48, %rsp
1411fcadc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
1411fcae1: 66 0f 6f f2                 	movdqa	%xmm2, %xmm6
1411fcae5: 48 89 d6                    	movq	%rdx, %rsi
1411fcae8: 48 89 cf                    	movq	%rcx, %rdi
1411fcaeb: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
1411fcaf0: 48 8d 59 20                 	leaq	0x20(%rcx), %rbx
1411fcaf4: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
1411fcaf9: 48 89 d9                    	movq	%rbx, %rcx
1411fcafc: e8 6f 42 3b ff              	callq	0x1405b0d70 <.text+0x5afd70>
1411fcb01: 48 83 7f 10 00              	cmpq	$0x0, 0x10(%rdi)
1411fcb06: 0f 84 2e 01 00 00           	je	0x1411fcc3a <.text+0x11fbc3a>
1411fcb0c: 48 8b 0f                    	movq	(%rdi), %rcx
1411fcb0f: 4c 8b 47 08                 	movq	0x8(%rdi), %r8
1411fcb13: 48 89 c2                    	movq	%rax, %rdx
1411fcb16: 48 c1 ea 39                 	shrq	$0x39, %rdx
1411fcb1a: 66 0f 6e c2                 	movd	%edx, %xmm0
1411fcb1e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1411fcb22: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1411fcb27: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1411fcb2c: 45 31 d2                    	xorl	%r10d, %r10d
1411fcb2f: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1411fcb33: 45 31 db                    	xorl	%r11d, %r11d
1411fcb36: 4c 21 c0                    	andq	%r8, %rax
1411fcb39: f3 0f 6f 14 01              	movdqu	(%rcx,%rax), %xmm2
1411fcb3e: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1411fcb42: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1411fcb46: 66 0f d7 db                 	pmovmskb	%xmm3, %ebx
1411fcb4a: 85 db                       	testl	%ebx, %ebx
1411fcb4c: 74 32                       	je	0x1411fcb80 <.text+0x11fbb80>
1411fcb4e: f3 44 0f bc fb              	tzcntl	%ebx, %r15d
1411fcb53: 49 01 c7                    	addq	%rax, %r15
1411fcb56: 4d 21 c7                    	andq	%r8, %r15
1411fcb59: 49 c1 e7 04                 	shlq	$0x4, %r15
1411fcb5d: 49 89 ce                    	movq	%rcx, %r14
1411fcb60: 4d 29 fe                    	subq	%r15, %r14
1411fcb63: 49 39 76 f0                 	cmpq	%rsi, -0x10(%r14)
1411fcb67: 74 6c                       	je	0x1411fcbd5 <.text+0x11fbbd5>
1411fcb69: 8d 6b ff                    	leal	-0x1(%rbx), %ebp
1411fcb6c: 66 21 dd                    	andw	%bx, %bp
1411fcb6f: 89 eb                       	movl	%ebp, %ebx
1411fcb71: 75 db                       	jne	0x1411fcb4e <.text+0x11fbb4e>
1411fcb73: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1411fcb80: 49 83 fb 01                 	cmpq	$0x1, %r11
1411fcb84: 74 39                       	je	0x1411fcbbf <.text+0x11fbbbf>
1411fcb86: 66 0f d7 da                 	pmovmskb	%xmm2, %ebx
1411fcb8a: 85 db                       	testl	%ebx, %ebx
1411fcb8c: 74 45                       	je	0x1411fcbd3 <.text+0x11fbbd3>
1411fcb8e: f3 44 0f bc cb              	tzcntl	%ebx, %r9d
1411fcb93: 49 01 c1                    	addq	%rax, %r9
1411fcb96: 4d 21 c1                    	andq	%r8, %r9
1411fcb99: 45 31 db                    	xorl	%r11d, %r11d
1411fcb9c: 66 85 db                    	testw	%bx, %bx
1411fcb9f: 41 0f 95 c3                 	setne	%r11b
1411fcba3: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1411fcba7: 66 0f d7 da                 	pmovmskb	%xmm2, %ebx
1411fcbab: 85 db                       	testl	%ebx, %ebx
1411fcbad: 75 37                       	jne	0x1411fcbe6 <.text+0x11fbbe6>
1411fcbaf: 4c 01 d0                    	addq	%r10, %rax
1411fcbb2: 48 83 c0 10                 	addq	$0x10, %rax
1411fcbb6: 49 83 c2 10                 	addq	$0x10, %r10
1411fcbba: e9 77 ff ff ff              	jmp	0x1411fcb36 <.text+0x11fbb36>
1411fcbbf: 41 bb 01 00 00 00           	movl	$0x1, %r11d
1411fcbc5: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1411fcbc9: 66 0f d7 da                 	pmovmskb	%xmm2, %ebx
1411fcbcd: 85 db                       	testl	%ebx, %ebx
1411fcbcf: 75 15                       	jne	0x1411fcbe6 <.text+0x11fbbe6>
1411fcbd1: eb dc                       	jmp	0x1411fcbaf <.text+0x11fbbaf>
1411fcbd3: eb c4                       	jmp	0x1411fcb99 <.text+0x11fbb99>
1411fcbd5: f3 41 0f 10 46 f8           	movss	-0x8(%r14), %xmm0
1411fcbdb: 49 83 c6 f8                 	addq	$-0x8, %r14
1411fcbdf: b8 01 00 00 00              	movl	$0x1, %eax
1411fcbe4: eb 3d                       	jmp	0x1411fcc23 <.text+0x11fbc23>
1411fcbe6: 42 0f b6 04 09              	movzbl	(%rcx,%r9), %eax
1411fcbeb: 84 c0                       	testb	%al, %al
1411fcbed: 79 69                       	jns	0x1411fcc58 <.text+0x11fbc58>
1411fcbef: 24 01                       	andb	$0x1, %al
1411fcbf1: 0f b6 c0                    	movzbl	%al, %eax
1411fcbf4: 48 29 47 10                 	subq	%rax, 0x10(%rdi)
1411fcbf8: 49 8d 41 f0                 	leaq	-0x10(%r9), %rax
1411fcbfc: 4c 21 c0                    	andq	%r8, %rax
1411fcbff: 42 88 14 09                 	movb	%dl, (%rcx,%r9)
1411fcc03: 88 54 01 10                 	movb	%dl, 0x10(%rcx,%rax)
1411fcc07: 48 ff 47 18                 	incq	0x18(%rdi)
1411fcc0b: 49 c1 e1 04                 	shlq	$0x4, %r9
1411fcc0f: 49 89 ce                    	movq	%rcx, %r14
1411fcc12: 4d 29 ce                    	subq	%r9, %r14
1411fcc15: 49 f7 d9                    	negq	%r9
1411fcc18: 4a 89 74 09 f0              	movq	%rsi, -0x10(%rcx,%r9)
1411fcc1d: 49 83 c6 f8                 	addq	$-0x8, %r14
1411fcc21: 31 c0                       	xorl	%eax, %eax
1411fcc23: 66 41 0f 7e 36              	movd	%xmm6, (%r14)
1411fcc28: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
1411fcc2d: 48 83 c4 48                 	addq	$0x48, %rsp
1411fcc31: 5b                          	popq	%rbx
1411fcc32: 5d                          	popq	%rbp
1411fcc33: 5f                          	popq	%rdi
1411fcc34: 5e                          	popq	%rsi
1411fcc35: 41 5e                       	popq	%r14
1411fcc37: 41 5f                       	popq	%r15
1411fcc39: c3                          	retq
1411fcc3a: ba 01 00 00 00              	movl	$0x1, %edx
1411fcc3f: 48 89 f9                    	movq	%rdi, %rcx
1411fcc42: 49 89 d8                    	movq	%rbx, %r8
1411fcc45: 41 b1 01                    	movb	$0x1, %r9b
1411fcc48: 48 89 c3                    	movq	%rax, %rbx
1411fcc4b: e8 00 d6 6a 01              	callq	0x1428aa250 <.text+0x28a9250>
1411fcc50: 48 89 d8                    	movq	%rbx, %rax
1411fcc53: e9 b4 fe ff ff              	jmp	0x1411fcb0c <.text+0x11fbb0c>
1411fcc58: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
1411fcc5c: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1411fcc60: f3 44 0f bc c8              	tzcntl	%eax, %r9d
1411fcc65: 42 0f b6 04 09              	movzbl	(%rcx,%r9), %eax
1411fcc6a: eb 83                       	jmp	0x1411fcbef <.text+0x11fbbef>

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14207b290: 55                          	pushq	%rbp
14207b291: 41 57                       	pushq	%r15
14207b293: 41 56                       	pushq	%r14
14207b295: 41 55                       	pushq	%r13
14207b297: 41 54                       	pushq	%r12
14207b299: 56                          	pushq	%rsi
14207b29a: 57                          	pushq	%rdi
14207b29b: 53                          	pushq	%rbx
14207b29c: 48 81 ec a8 01 00 00        	subq	$0x1a8, %rsp            # imm = 0x1A8
14207b2a3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14207b2ab: 44 0f 29 8d 10 01 00 00     	movaps	%xmm9, 0x110(%rbp)
14207b2b3: 44 0f 29 85 00 01 00 00     	movaps	%xmm8, 0x100(%rbp)
14207b2bb: 66 0f 7f bd f0 00 00 00     	movdqa	%xmm7, 0xf0(%rbp)
14207b2c3: 0f 29 b5 e0 00 00 00        	movaps	%xmm6, 0xe0(%rbp)
14207b2ca: 48 c7 85 d8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xd8(%rbp)
14207b2d5: 4c 89 cf                    	movq	%r9, %rdi
14207b2d8: 4c 89 c6                    	movq	%r8, %rsi
14207b2db: 49 89 d6                    	movq	%rdx, %r14
14207b2de: 48 89 cb                    	movq	%rcx, %rbx
14207b2e1: e8 1a b7 b9 fe              	callq	0x140c16a00 <.text+0xc15a00>
14207b2e6: 84 c0                       	testb	%al, %al
14207b2e8: 74 67                       	je	0x14207b351 <.text+0x207a351>
14207b2ea: 4c 89 b5 c8 00 00 00        	movq	%r14, 0xc8(%rbp)
14207b2f1: 4c 8d 25 0e 7c f5 00        	leaq	0xf57c0e(%rip), %r12    # 0x142fd2f06
14207b2f8: 41 be 38 00 00 00           	movl	$0x38, %r14d
14207b2fe: ba 38 00 00 00              	movl	$0x38, %edx
14207b303: 4c 89 e1                    	movq	%r12, %rcx
14207b306: e8 65 3b ff ff              	callq	0x14206ee70 <.text+0x206de70>
14207b30b: 48 83 f8 01                 	cmpq	$0x1, %rax
14207b30f: 0f 85 b2 00 00 00           	jne	0x14207b3c7 <.text+0x207a3c7>
14207b315: 49 89 d1                    	movq	%rdx, %r9
14207b318: 48 85 d2                    	testq	%rdx, %rdx
14207b31b: 74 3d                       	je	0x14207b35a <.text+0x207a35a>
14207b31d: 49 83 f9 38                 	cmpq	$0x38, %r9
14207b321: 73 35                       	jae	0x14207b358 <.text+0x207a358>
14207b323: 48 8d 05 dc 7b f5 00        	leaq	0xf57bdc(%rip), %rax    # 0x142fd2f06
14207b32a: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14207b32f: 7f 29                       	jg	0x14207b35a <.text+0x207a35a>
14207b331: 48 8d 05 a0 80 f5 00        	leaq	0xf580a0(%rip), %rax    # 0x142fd33d8
14207b338: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207b33d: 48 8d 0d c2 7b f5 00        	leaq	0xf57bc2(%rip), %rcx    # 0x142fd2f06
14207b344: ba 38 00 00 00              	movl	$0x38, %edx
14207b349: 45 31 c0                    	xorl	%r8d, %r8d
14207b34c: e8 ef e5 85 00              	callq	0x1428d9940 <.text+0x28d8940>
14207b351: 31 c0                       	xorl	%eax, %eax
14207b353: e9 d6 01 00 00              	jmp	0x14207b52e <.text+0x207a52e>
14207b358: 75 d7                       	jne	0x14207b331 <.text+0x207a331>
14207b35a: 4c 8d 25 a5 7b f5 00        	leaq	0xf57ba5(%rip), %r12    # 0x142fd2f06
14207b361: 4c 89 e1                    	movq	%r12, %rcx
14207b364: 4c 89 ca                    	movq	%r9, %rdx
14207b367: e8 04 3b ff ff              	callq	0x14206ee70 <.text+0x206de70>
14207b36c: 48 83 f8 01                 	cmpq	$0x1, %rax
14207b370: 75 55                       	jne	0x14207b3c7 <.text+0x207a3c7>
14207b372: 49 89 d0                    	movq	%rdx, %r8
14207b375: 49 83 c0 02                 	addq	$0x2, %r8
14207b379: 74 39                       	je	0x14207b3b4 <.text+0x207a3b4>
14207b37b: 49 83 f8 38                 	cmpq	$0x38, %r8
14207b37f: 73 31                       	jae	0x14207b3b2 <.text+0x207a3b2>
14207b381: 48 8d 05 7e 7b f5 00        	leaq	0xf57b7e(%rip), %rax    # 0x142fd2f06
14207b388: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14207b38d: 7f 25                       	jg	0x14207b3b4 <.text+0x207a3b4>
14207b38f: 48 8d 05 5a 80 f5 00        	leaq	0xf5805a(%rip), %rax    # 0x142fd33f0
14207b396: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207b39b: 48 8d 0d 64 7b f5 00        	leaq	0xf57b64(%rip), %rcx    # 0x142fd2f06
14207b3a2: ba 38 00 00 00              	movl	$0x38, %edx
14207b3a7: 41 b9 38 00 00 00           	movl	$0x38, %r9d
14207b3ad: e8 8e e5 85 00              	callq	0x1428d9940 <.text+0x28d8940>
14207b3b2: 75 db                       	jne	0x14207b38f <.text+0x207a38f>
14207b3b4: 41 be 36 00 00 00           	movl	$0x36, %r14d
14207b3ba: 49 29 d6                    	subq	%rdx, %r14
14207b3bd: 4c 8d 25 42 7b f5 00        	leaq	0xf57b42(%rip), %r12    # 0x142fd2f06
14207b3c4: 4d 01 c4                    	addq	%r8, %r12
14207b3c7: 4c 8d 05 90 80 f5 00        	leaq	0xf58090(%rip), %r8     # 0x142fd345e
14207b3ce: 4c 8d 2d 5b 80 f5 00        	leaq	0xf5805b(%rip), %r13    # 0x142fd3430
14207b3d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14207b3e0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14207b3e5: 85 c0                       	testl	%eax, %eax
14207b3e7: 78 17                       	js	0x14207b400 <.text+0x207a400>
14207b3e9: 49 ff c8                    	decq	%r8
14207b3ec: 83 f8 5c                    	cmpl	$0x5c, %eax
14207b3ef: 75 63                       	jne	0x14207b454 <.text+0x207a454>
14207b3f1: eb 75                       	jmp	0x14207b468 <.text+0x207a468>
14207b3f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14207b400: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14207b405: 80 f9 c0                    	cmpb	$-0x40, %cl
14207b408: 7d 23                       	jge	0x14207b42d <.text+0x207a42d>
14207b40a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14207b40f: 80 fa c0                    	cmpb	$-0x40, %dl
14207b412: 7d 22                       	jge	0x14207b436 <.text+0x207a436>
14207b414: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14207b419: 49 83 c0 fc                 	addq	$-0x4, %r8
14207b41d: 41 83 e1 07                 	andl	$0x7, %r9d
14207b421: 41 c1 e1 06                 	shll	$0x6, %r9d
14207b425: 83 e2 3f                    	andl	$0x3f, %edx
14207b428: 44 09 ca                    	orl	%r9d, %edx
14207b42b: eb 10                       	jmp	0x14207b43d <.text+0x207a43d>
14207b42d: 49 83 c0 fe                 	addq	$-0x2, %r8
14207b431: 83 e1 1f                    	andl	$0x1f, %ecx
14207b434: eb 0f                       	jmp	0x14207b445 <.text+0x207a445>
14207b436: 49 83 c0 fd                 	addq	$-0x3, %r8
14207b43a: 83 e2 0f                    	andl	$0xf, %edx
14207b43d: c1 e2 06                    	shll	$0x6, %edx
14207b440: 83 e1 3f                    	andl	$0x3f, %ecx
14207b443: 09 d1                       	orl	%edx, %ecx
14207b445: c1 e1 06                    	shll	$0x6, %ecx
14207b448: 24 3f                       	andb	$0x3f, %al
14207b44a: 0f b6 c0                    	movzbl	%al, %eax
14207b44d: 09 c8                       	orl	%ecx, %eax
14207b44f: 83 f8 5c                    	cmpl	$0x5c, %eax
14207b452: 74 14                       	je	0x14207b468 <.text+0x207a468>
14207b454: 83 f8 2f                    	cmpl	$0x2f, %eax
14207b457: 74 0f                       	je	0x14207b468 <.text+0x207a468>
14207b459: 4d 39 e8                    	cmpq	%r13, %r8
14207b45c: 75 82                       	jne	0x14207b3e0 <.text+0x207a3e0>
14207b45e: 49 89 ff                    	movq	%rdi, %r15
14207b461: bf 2e 00 00 00              	movl	$0x2e, %edi
14207b466: eb 4f                       	jmp	0x14207b4b7 <.text+0x207a4b7>
14207b468: 4c 8d 2d c1 7f f5 00        	leaq	0xf57fc1(%rip), %r13    # 0x142fd3430
14207b46f: 4d 29 e8                    	subq	%r13, %r8
14207b472: 49 ff c0                    	incq	%r8
14207b475: 74 32                       	je	0x14207b4a9 <.text+0x207a4a9>
14207b477: 49 83 f8 2e                 	cmpq	$0x2e, %r8
14207b47b: 73 2a                       	jae	0x14207b4a7 <.text+0x207a4a7>
14207b47d: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
14207b482: 7f 25                       	jg	0x14207b4a9 <.text+0x207a4a9>
14207b484: 48 8d 05 2d 7f f5 00        	leaq	0xf57f2d(%rip), %rax    # 0x142fd33b8
14207b48b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207b490: 48 8d 0d 99 7f f5 00        	leaq	0xf57f99(%rip), %rcx    # 0x142fd3430
14207b497: ba 2e 00 00 00              	movl	$0x2e, %edx
14207b49c: 41 b9 2e 00 00 00           	movl	$0x2e, %r9d
14207b4a2: e8 99 e4 85 00              	callq	0x1428d9940 <.text+0x28d8940>
14207b4a7: 75 db                       	jne	0x14207b484 <.text+0x207a484>
14207b4a9: 49 89 ff                    	movq	%rdi, %r15
14207b4ac: bf 2e 00 00 00              	movl	$0x2e, %edi
14207b4b1: 4c 29 c7                    	subq	%r8, %rdi
14207b4b4: 4d 01 c5                    	addq	%r8, %r13
14207b4b7: e8 74 cc b9 fe              	callq	0x140c18130 <.text+0xc17130>
14207b4bc: 48 89 c1                    	movq	%rax, %rcx
14207b4bf: 48 8b 00                    	movq	(%rax), %rax
14207b4c2: 48 83 f8 01                 	cmpq	$0x1, %rax
14207b4c6: 0f 85 b1 09 00 00           	jne	0x14207be7d <.text+0x207ae7d>
14207b4cc: 48 83 c1 08                 	addq	$0x8, %rcx
14207b4d0: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14207b4d4: 0f 85 c7 09 00 00           	jne	0x14207bea1 <.text+0x207aea1>
14207b4da: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14207b4e1: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
14207b4e8: 48 83 c1 08                 	addq	$0x8, %rcx
14207b4ec: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14207b4f1: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14207b4fa: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14207b503: 4c 89 e2                    	movq	%r12, %rdx
14207b506: 4d 89 f0                    	movq	%r14, %r8
14207b509: 4d 89 e9                    	movq	%r13, %r9
14207b50c: e8 2f b8 b9 fe              	callq	0x140c16d40 <.text+0xc15d40>
14207b511: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14207b518: 48 ff 01                    	incq	(%rcx)
14207b51b: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14207b51f: b8 01 00 00 00              	movl	$0x1, %eax
14207b524: 4c 89 ff                    	movq	%r15, %rdi
14207b527: 4c 8b b5 c8 00 00 00        	movq	0xc8(%rbp), %r14
14207b52e: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14207b532: 48 8b 0e                    	movq	(%rsi), %rcx
14207b535: b2 13                       	movb	$0x13, %dl
14207b537: e8 14 67 91 fe              	callq	0x140991c50 <.text+0x990c50>
14207b53c: 48 89 55 18                 	movq	%rdx, 0x18(%rbp)
14207b540: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14207b544: 48 8b 33                    	movq	(%rbx), %rsi
14207b547: 4c 8b 7b 08                 	movq	0x8(%rbx), %r15
14207b54b: 8b 4b 14                    	movl	0x14(%rbx), %ecx
14207b54e: 4c 8b a6 f8 00 00 00        	movq	0xf8(%rsi), %r12
14207b555: 48 8b 86 00 01 00 00        	movq	0x100(%rsi), %rax
14207b55c: 4d 8d 2c 84                 	leaq	(%r12,%rax,4), %r13
14207b560: 0f b6 96 08 01 00 00        	movzbl	0x108(%rsi), %edx
14207b567: 4d 8b 06                    	movq	(%r14), %r8
14207b56a: 48 8b 07                    	movq	(%rdi), %rax
14207b56d: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14207b571: 48 83 c0 20                 	addq	$0x20, %rax
14207b575: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14207b579: 89 8d a8 00 00 00           	movl	%ecx, 0xa8(%rbp)
14207b57f: 89 c8                       	movl	%ecx, %eax
14207b581: 2b 43 10                    	subl	0x10(%rbx), %eax
14207b584: 3d 00 b0 33 c2              	cmpl	$0xc233b000, %eax       # imm = 0xC233B000
14207b589: b9 00 b0 33 c2              	movl	$0xc233b000, %ecx       # imm = 0xC233B000
14207b58e: 0f 42 c8                    	cmovbl	%eax, %ecx
14207b591: 89 8d c0 00 00 00           	movl	%ecx, 0xc0(%rbp)
14207b597: 4c 89 45 08                 	movq	%r8, 0x8(%rbp)
14207b59b: 49 8d 40 20                 	leaq	0x20(%r8), %rax
14207b59f: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14207b5a3: b8 08 00 00 00              	movl	$0x8, %eax
14207b5a8: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14207b5ac: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
14207b5b7: 66 0f 76 ff                 	pcmpeqd	%xmm7, %xmm7
14207b5bb: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
14207b5bf: f3 0f 10 35 3d a3 8a 00     	movss	0x8aa33d(%rip), %xmm6   # 0x142925904
14207b5c7: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
14207b5d2: 31 db                       	xorl	%ebx, %ebx
14207b5d4: 31 ff                       	xorl	%edi, %edi
14207b5d6: 48 c7 85 88 00 00 00 00 00 00 00    	movq	$0x0, 0x88(%rbp)
14207b5e1: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
14207b5eb: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
14207b5f6: 45 31 c0                    	xorl	%r8d, %r8d
14207b5f9: 88 95 d6 00 00 00           	movb	%dl, 0xd6(%rbp)
14207b5ff: 84 d2                       	testb	%dl, %dl
14207b601: 0f 84 0c 02 00 00           	je	0x14207b813 <.text+0x207a813>
14207b607: e9 67 04 00 00              	jmp	0x14207ba73 <.text+0x207aa73>
14207b60c: f3 47 0f 10 8c 10 90 fd ff ff       	movss	-0x270(%r8,%r10), %xmm9
14207b616: 45 0f 2e c8                 	ucomiss	%xmm8, %xmm9
14207b61a: 0f 86 71 07 00 00           	jbe	0x14207bd91 <.text+0x207ad91>
14207b620: 4d 01 d0                    	addq	%r10, %r8
14207b623: 4c 89 85 80 00 00 00        	movq	%r8, 0x80(%rbp)
14207b62a: 41 8b 40 b8                 	movl	-0x48(%r8), %eax
14207b62e: 89 85 c4 00 00 00           	movl	%eax, 0xc4(%rbp)
14207b634: 48 8d 4d 38                 	leaq	0x38(%rbp), %rcx
14207b638: 4c 8b b5 98 00 00 00        	movq	0x98(%rbp), %r14
14207b63f: 4c 89 f2                    	movq	%r14, %rdx
14207b642: e8 49 54 d4 fe              	callq	0x140dc0a90 <.text+0xdbfa90>
14207b647: 45 31 c0                    	xorl	%r8d, %r8d
14207b64a: 83 bd c4 00 00 00 02        	cmpl	$0x2, 0xc4(%rbp)
14207b651: 0f 95 c1                    	setne	%cl
14207b654: 45 8b 4e 18                 	movl	0x18(%r14), %r9d
14207b658: 4d 8b 56 10                 	movq	0x10(%r14), %r10
14207b65c: 45 8b 5e 1c                 	movl	0x1c(%r14), %r11d
14207b660: 4c 8b 75 20                 	movq	0x20(%rbp), %r14
14207b664: 49 83 7e 18 00              	cmpq	$0x0, 0x18(%r14)
14207b669: 0f 84 d0 00 00 00           	je	0x14207b73f <.text+0x207a73f>
14207b66f: 88 8d d7 00 00 00           	movb	%cl, 0xd7(%rbp)
14207b675: 44 89 9d c4 00 00 00        	movl	%r11d, 0xc4(%rbp)
14207b67c: 4c 89 95 98 00 00 00        	movq	%r10, 0x98(%rbp)
14207b683: 44 89 8d a4 00 00 00        	movl	%r9d, 0xa4(%rbp)
14207b68a: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
14207b68e: 48 89 ea                    	movq	%rbp, %rdx
14207b691: e8 da 56 53 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14207b696: 49 8b 0e                    	movq	(%r14), %rcx
14207b699: 49 8b 56 08                 	movq	0x8(%r14), %rdx
14207b69d: 49 89 d0                    	movq	%rdx, %r8
14207b6a0: 49 21 c0                    	andq	%rax, %r8
14207b6a3: 48 c1 e8 39                 	shrq	$0x39, %rax
14207b6a7: 66 0f 6e c0                 	movd	%eax, %xmm0
14207b6ab: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14207b6af: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14207b6b4: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14207b6b9: 48 8d 41 f0                 	leaq	-0x10(%rcx), %rax
14207b6bd: 45 31 db                    	xorl	%r11d, %r11d
14207b6c0: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14207b6c6: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14207b6ca: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14207b6ce: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14207b6d3: 45 85 d2                    	testl	%r10d, %r10d
14207b6d6: 74 39                       	je	0x14207b711 <.text+0x207a711>
14207b6d8: 4c 89 5d 30                 	movq	%r11, 0x30(%rbp)
14207b6dc: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14207b6e1: 4d 01 c3                    	addq	%r8, %r11
14207b6e4: 49 21 d3                    	andq	%rdx, %r11
14207b6e7: 4d 89 d9                    	movq	%r11, %r9
14207b6ea: 49 c1 e1 04                 	shlq	$0x4, %r9
14207b6ee: 49 89 c6                    	movq	%rax, %r14
14207b6f1: 4c 29 c8                    	subq	%r9, %rax
14207b6f4: 4c 8b 4d 78                 	movq	0x78(%rbp), %r9
14207b6f8: 4c 3b 08                    	cmpq	(%rax), %r9
14207b6fb: 74 36                       	je	0x14207b733 <.text+0x207a733>
14207b6fd: 41 8d 42 ff                 	leal	-0x1(%r10), %eax
14207b701: 66 44 21 d0                 	andw	%r10w, %ax
14207b705: 41 89 c2                    	movl	%eax, %r10d
14207b708: 4c 89 f0                    	movq	%r14, %rax
14207b70b: 4c 8b 5d 30                 	movq	0x30(%rbp), %r11
14207b70f: 75 cb                       	jne	0x14207b6dc <.text+0x207a6dc>
14207b711: 49 89 c1                    	movq	%rax, %r9
14207b714: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
14207b718: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
14207b71c: 85 c0                       	testl	%eax, %eax
14207b71e: 75 2a                       	jne	0x14207b74a <.text+0x207a74a>
14207b720: 4d 01 d8                    	addq	%r11, %r8
14207b723: 49 83 c0 10                 	addq	$0x10, %r8
14207b727: 49 83 c3 10                 	addq	$0x10, %r11
14207b72b: 49 21 d0                    	andq	%rdx, %r8
14207b72e: 4c 89 c8                    	movq	%r9, %rax
14207b731: eb 8d                       	jmp	0x14207b6c0 <.text+0x207a6c0>
14207b733: 49 f7 db                    	negq	%r11
14207b736: 49 c1 e3 04                 	shlq	$0x4, %r11
14207b73a: 4c 01 d9                    	addq	%r11, %rcx
14207b73d: eb 0d                       	jmp	0x14207b74c <.text+0x207a74c>
14207b73f: 31 c0                       	xorl	%eax, %eax
14207b741: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14207b748: eb 33                       	jmp	0x14207b77d <.text+0x207a77d>
14207b74a: 31 c9                       	xorl	%ecx, %ecx
14207b74c: 48 8b 95 80 00 00 00        	movq	0x80(%rbp), %rdx
14207b753: 45 31 c0                    	xorl	%r8d, %r8d
14207b756: 44 8b 8d a4 00 00 00        	movl	0xa4(%rbp), %r9d
14207b75d: 4c 8b 95 98 00 00 00        	movq	0x98(%rbp), %r10
14207b764: 44 8b 9d c4 00 00 00        	movl	0xc4(%rbp), %r11d
14207b76b: 48 8d 41 f0                 	leaq	-0x10(%rcx), %rax
14207b76f: 48 85 c9                    	testq	%rcx, %rcx
14207b772: 48 0f 44 c1                 	cmoveq	%rcx, %rax
14207b776: 0f b6 8d d7 00 00 00        	movzbl	0xd7(%rbp), %ecx
14207b77d: 41 88 c8                    	movb	%cl, %r8b
14207b780: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
14207b784: 48 83 c2 f0                 	addq	$-0x10, %rdx
14207b788: 48 85 c0                    	testq	%rax, %rax
14207b78b: 48 0f 45 d1                 	cmovneq	%rcx, %rdx
14207b78f: 66 0f 6e 02                 	movd	(%rdx), %xmm0
14207b793: f3 0f 6f 4d 38              	movdqu	0x38(%rbp), %xmm1
14207b798: 66 0f 7f 4d c0              	movdqa	%xmm1, -0x40(%rbp)
14207b79d: 48 8b 45 78                 	movq	0x78(%rbp), %rax
14207b7a1: 89 45 d0                    	movl	%eax, -0x30(%rbp)
14207b7a4: f3 44 0f 11 4d d4           	movss	%xmm9, -0x2c(%rbp)
14207b7aa: 44 89 45 d8                 	movl	%r8d, -0x28(%rbp)
14207b7ae: 44 89 4d dc                 	movl	%r9d, -0x24(%rbp)
14207b7b2: 4c 89 55 e0                 	movq	%r10, -0x20(%rbp)
14207b7b6: 44 89 5d e8                 	movl	%r11d, -0x18(%rbp)
14207b7ba: 66 0f 7e 45 ec              	movd	%xmm0, -0x14(%rbp)
14207b7bf: c7 45 40 00 00 00 00        	movl	$0x0, 0x40(%rbp)
14207b7c6: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
14207b7ce: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
14207b7d2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14207b7d7: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
14207b7db: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
14207b7df: 4c 8d 45 38                 	leaq	0x38(%rbp), %r8
14207b7e3: 0f 28 de                    	movaps	%xmm6, %xmm3
14207b7e6: e8 45 2d 4f ff              	callq	0x14156e530 <.text+0x156d530>
14207b7eb: 0f b6 95 d6 00 00 00        	movzbl	0xd6(%rbp), %edx
14207b7f2: 44 8b 85 a0 00 00 00        	movl	0xa0(%rbp), %r8d
14207b7f9: 84 d2                       	testb	%dl, %dl
14207b7fb: 74 16                       	je	0x14207b813 <.text+0x207a813>
14207b7fd: e9 71 02 00 00              	jmp	0x14207ba73 <.text+0x207aa73>
14207b802: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14207b810: 48 ff c7                    	incq	%rdi
14207b813: 48 39 df                    	cmpq	%rbx, %rdi
14207b816: 0f 85 d4 00 00 00           	jne	0x14207b8f0 <.text+0x207a8f0>
14207b81c: 4d 39 ec                    	cmpq	%r13, %r12
14207b81f: 0f 84 89 05 00 00           	je	0x14207bdae <.text+0x207adae>
14207b825: 41 8b 0c 24                 	movl	(%r12), %ecx
14207b829: 49 83 c4 04                 	addq	$0x4, %r12
14207b82d: 49 8b 87 00 01 00 00        	movq	0x100(%r15), %rax
14207b834: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14207b838: 48 c1 e1 05                 	shlq	$0x5, %rcx
14207b83c: 48 83 7c 08 58 00           	cmpq	$0x0, 0x58(%rax,%rcx)
14207b842: 74 1c                       	je	0x14207b860 <.text+0x207a860>
14207b844: 48 8d 3c 08                 	leaq	(%rax,%rcx), %rdi
14207b848: 48 8d 1c 08                 	leaq	(%rax,%rcx), %rbx
14207b84c: 48 83 c3 58                 	addq	$0x58, %rbx
14207b850: eb 3d                       	jmp	0x14207b88f <.text+0x207a88f>
14207b852: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14207b860: 4d 39 ec                    	cmpq	%r13, %r12
14207b863: 0f 84 45 05 00 00           	je	0x14207bdae <.text+0x207adae>
14207b869: 41 8b 04 24                 	movl	(%r12), %eax
14207b86d: 49 83 c4 04                 	addq	$0x4, %r12
14207b871: 49 8b bf 00 01 00 00        	movq	0x100(%r15), %rdi
14207b878: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
14207b87c: 48 c1 e0 05                 	shlq	$0x5, %rax
14207b880: 48 83 7c 07 58 00           	cmpq	$0x0, 0x58(%rdi,%rax)
14207b886: 74 d8                       	je	0x14207b860 <.text+0x207a860>
14207b888: 48 01 c7                    	addq	%rax, %rdi
14207b88b: 48 8d 5f 58                 	leaq	0x58(%rdi), %rbx
14207b88f: 8b 87 94 00 00 00           	movl	0x94(%rdi), %eax
14207b895: 49 8b 97 a8 01 00 00        	movq	0x1a8(%r15), %rdx
14207b89c: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
14207b8a0: 4c 8d 34 c2                 	leaq	(%rdx,%rax,8), %r14
14207b8a4: 48 8b 8e 10 01 00 00        	movq	0x110(%rsi), %rcx
14207b8ab: 48 8b 44 c2 40              	movq	0x40(%rdx,%rax,8), %rax
14207b8b0: 48 39 c8                    	cmpq	%rcx, %rax
14207b8b3: 76 4b                       	jbe	0x14207b900 <.text+0x207a900>
14207b8b5: 49 8b 56 38                 	movq	0x38(%r14), %rdx
14207b8b9: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
14207b8bd: 48 85 c9                    	testq	%rcx, %rcx
14207b8c0: 74 3e                       	je	0x14207b900 <.text+0x207a900>
14207b8c2: 49 8b 56 18                 	movq	0x18(%r14), %rdx
14207b8c6: 48 f7 d1                    	notq	%rcx
14207b8c9: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14207b8cd: 48 c1 e1 04                 	shlq	$0x4, %rcx
14207b8d1: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
14207b8d6: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
14207b8dd: 48 8b 8e 18 01 00 00        	movq	0x118(%rsi), %rcx
14207b8e4: 48 39 c8                    	cmpq	%rcx, %rax
14207b8e7: 77 2e                       	ja	0x14207b917 <.text+0x207a917>
14207b8e9: eb 75                       	jmp	0x14207b960 <.text+0x207a960>
14207b8eb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14207b8f0: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14207b8f4: e9 13 01 00 00              	jmp	0x14207ba0c <.text+0x207aa0c>
14207b8f9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14207b900: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
14207b90b: 48 8b 8e 18 01 00 00        	movq	0x118(%rsi), %rcx
14207b912: 48 39 c8                    	cmpq	%rcx, %rax
14207b915: 76 49                       	jbe	0x14207b960 <.text+0x207a960>
14207b917: 49 8b 46 38                 	movq	0x38(%r14), %rax
14207b91b: 48 8b 04 c8                 	movq	(%rax,%rcx,8), %rax
14207b91f: 48 85 c0                    	testq	%rax, %rax
14207b922: 74 3c                       	je	0x14207b960 <.text+0x207a960>
14207b924: 49 8b 4e 18                 	movq	0x18(%r14), %rcx
14207b928: 48 f7 d0                    	notq	%rax
14207b92b: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14207b92f: 48 c1 e0 04                 	shlq	$0x4, %rax
14207b933: 48 8b 44 01 10              	movq	0x10(%rcx,%rax), %rax
14207b938: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
14207b93f: 48 8b 96 28 01 00 00        	movq	0x128(%rsi), %rdx
14207b946: 48 8b 87 88 00 00 00        	movq	0x88(%rdi), %rax
14207b94d: 48 39 d0                    	cmpq	%rdx, %rax
14207b950: 76 6e                       	jbe	0x14207b9c0 <.text+0x207a9c0>
14207b952: eb 2a                       	jmp	0x14207b97e <.text+0x207a97e>
14207b954: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14207b960: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
14207b96b: 48 8b 96 28 01 00 00        	movq	0x128(%rsi), %rdx
14207b972: 48 8b 87 88 00 00 00        	movq	0x88(%rdi), %rax
14207b979: 48 39 d0                    	cmpq	%rdx, %rax
14207b97c: 76 42                       	jbe	0x14207b9c0 <.text+0x207a9c0>
14207b97e: 48 8b 8f 80 00 00 00        	movq	0x80(%rdi), %rcx
14207b985: 48 83 3c d1 00              	cmpq	$0x0, (%rcx,%rdx,8)
14207b98a: 74 34                       	je	0x14207b9c0 <.text+0x207a9c0>
14207b98c: 4c 89 f1                    	movq	%r14, %rcx
14207b98f: e8 dc 4d c8 fe              	callq	0x140d00770 <.text+0xcff770>
14207b994: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14207b99b: 48 8b 87 88 00 00 00        	movq	0x88(%rdi), %rax
14207b9a2: b1 01                       	movb	$0x1, %cl
14207b9a4: 89 8d c8 00 00 00           	movl	%ecx, 0xc8(%rbp)
14207b9aa: 48 8b 96 30 01 00 00        	movq	0x130(%rsi), %rdx
14207b9b1: 48 39 d0                    	cmpq	%rdx, %rax
14207b9b4: 76 4a                       	jbe	0x14207ba00 <.text+0x207aa00>
14207b9b6: eb 1e                       	jmp	0x14207b9d6 <.text+0x207a9d6>
14207b9b8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14207b9c0: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
14207b9ca: 48 8b 96 30 01 00 00        	movq	0x130(%rsi), %rdx
14207b9d1: 48 39 d0                    	cmpq	%rdx, %rax
14207b9d4: 76 2a                       	jbe	0x14207ba00 <.text+0x207aa00>
14207b9d6: 48 8b 87 80 00 00 00        	movq	0x80(%rdi), %rax
14207b9dd: 48 83 3c d0 00              	cmpq	$0x0, (%rax,%rdx,8)
14207b9e2: 74 1c                       	je	0x14207ba00 <.text+0x207aa00>
14207b9e4: 4c 89 f1                    	movq	%r14, %rcx
14207b9e7: e8 84 4d c8 fe              	callq	0x140d00770 <.text+0xcff770>
14207b9ec: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14207b9f3: 41 b0 01                    	movb	$0x1, %r8b
14207b9f6: eb 0b                       	jmp	0x14207ba03 <.text+0x207aa03>
14207b9f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14207ba00: 45 31 c0                    	xorl	%r8d, %r8d
14207ba03: 48 8b 4f 50                 	movq	0x50(%rdi), %rcx
14207ba07: 48 8b 1b                    	movq	(%rbx), %rbx
14207ba0a: 31 ff                       	xorl	%edi, %edi
14207ba0c: 48 89 f8                    	movq	%rdi, %rax
14207ba0f: 48 c1 e0 04                 	shlq	$0x4, %rax
14207ba13: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
14207ba17: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
14207ba1b: f6 85 c8 00 00 00 01        	testb	$0x1, 0xc8(%rbp)
14207ba22: 74 1c                       	je	0x14207ba40 <.text+0x207aa40>
14207ba24: 8b 8d a8 00 00 00           	movl	0xa8(%rbp), %ecx
14207ba2a: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
14207ba31: 2b 0c 82                    	subl	(%rdx,%rax,4), %ecx
14207ba34: 3b 8d c0 00 00 00           	cmpl	0xc0(%rbp), %ecx
14207ba3a: 0f 82 43 02 00 00           	jb	0x14207bc83 <.text+0x207ac83>
14207ba40: 41 f6 c0 01                 	testb	$0x1, %r8b
14207ba44: 0f 84 c6 fd ff ff           	je	0x14207b810 <.text+0x207a810>
14207ba4a: 89 c0                       	movl	%eax, %eax
14207ba4c: 8b 8d a8 00 00 00           	movl	0xa8(%rbp), %ecx
14207ba52: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
14207ba59: 2b 0c 82                    	subl	(%rdx,%rax,4), %ecx
14207ba5c: 3b 8d c0 00 00 00           	cmpl	0xc0(%rbp), %ecx
14207ba62: 0f 83 a8 fd ff ff           	jae	0x14207b810 <.text+0x207a810>
14207ba68: e9 16 02 00 00              	jmp	0x14207bc83 <.text+0x207ac83>
14207ba6d: 0f 1f 00                    	nopl	(%rax)
14207ba70: 48 ff c7                    	incq	%rdi
14207ba73: 48 39 df                    	cmpq	%rbx, %rdi
14207ba76: 0f 85 bc 01 00 00           	jne	0x14207bc38 <.text+0x207ac38>
14207ba7c: 4d 39 ec                    	cmpq	%r13, %r12
14207ba7f: 0f 84 29 03 00 00           	je	0x14207bdae <.text+0x207adae>
14207ba85: 41 8b 0c 24                 	movl	(%r12), %ecx
14207ba89: 49 83 c4 04                 	addq	$0x4, %r12
14207ba8d: 49 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %rax
14207ba94: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14207ba98: 48 83 7c c8 10 00           	cmpq	$0x0, 0x10(%rax,%rcx,8)
14207ba9e: 74 30                       	je	0x14207bad0 <.text+0x207aad0>
14207baa0: 4c 8d 34 c8                 	leaq	(%rax,%rcx,8), %r14
14207baa4: 48 8d 3c c8                 	leaq	(%rax,%rcx,8), %rdi
14207baa8: 48 83 c7 10                 	addq	$0x10, %rdi
14207baac: 48 8b 8e 10 01 00 00        	movq	0x110(%rsi), %rcx
14207bab3: 49 8b 46 40                 	movq	0x40(%r14), %rax
14207bab7: 48 39 c8                    	cmpq	%rcx, %rax
14207baba: 0f 86 90 00 00 00           	jbe	0x14207bb50 <.text+0x207ab50>
14207bac0: eb 4a                       	jmp	0x14207bb0c <.text+0x207ab0c>
14207bac2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14207bad0: 4d 39 ec                    	cmpq	%r13, %r12
14207bad3: 0f 84 d5 02 00 00           	je	0x14207bdae <.text+0x207adae>
14207bad9: 41 8b 0c 24                 	movl	(%r12), %ecx
14207badd: 49 83 c4 04                 	addq	$0x4, %r12
14207bae1: 49 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %rax
14207bae8: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14207baec: 48 83 7c c8 10 00           	cmpq	$0x0, 0x10(%rax,%rcx,8)
14207baf2: 74 dc                       	je	0x14207bad0 <.text+0x207aad0>
14207baf4: 4c 8d 34 c8                 	leaq	(%rax,%rcx,8), %r14
14207baf8: 49 8d 7e 10                 	leaq	0x10(%r14), %rdi
14207bafc: 48 8b 8e 10 01 00 00        	movq	0x110(%rsi), %rcx
14207bb03: 49 8b 46 40                 	movq	0x40(%r14), %rax
14207bb07: 48 39 c8                    	cmpq	%rcx, %rax
14207bb0a: 76 44                       	jbe	0x14207bb50 <.text+0x207ab50>
14207bb0c: 49 8b 56 38                 	movq	0x38(%r14), %rdx
14207bb10: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
14207bb14: 48 85 c9                    	testq	%rcx, %rcx
14207bb17: 74 37                       	je	0x14207bb50 <.text+0x207ab50>
14207bb19: 49 8b 56 18                 	movq	0x18(%r14), %rdx
14207bb1d: 48 f7 d1                    	notq	%rcx
14207bb20: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14207bb24: 48 c1 e1 04                 	shlq	$0x4, %rcx
14207bb28: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
14207bb2d: 48 89 8d b0 00 00 00        	movq	%rcx, 0xb0(%rbp)
14207bb34: 48 8b 8e 18 01 00 00        	movq	0x118(%rsi), %rcx
14207bb3b: 48 39 c8                    	cmpq	%rcx, %rax
14207bb3e: 77 27                       	ja	0x14207bb67 <.text+0x207ab67>
14207bb40: eb 5e                       	jmp	0x14207bba0 <.text+0x207aba0>
14207bb42: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14207bb50: 48 c7 85 b0 00 00 00 00 00 00 00    	movq	$0x0, 0xb0(%rbp)
14207bb5b: 48 8b 8e 18 01 00 00        	movq	0x118(%rsi), %rcx
14207bb62: 48 39 c8                    	cmpq	%rcx, %rax
14207bb65: 76 39                       	jbe	0x14207bba0 <.text+0x207aba0>
14207bb67: 49 8b 56 38                 	movq	0x38(%r14), %rdx
14207bb6b: 48 8b 0c ca                 	movq	(%rdx,%rcx,8), %rcx
14207bb6f: 48 85 c9                    	testq	%rcx, %rcx
14207bb72: 74 2c                       	je	0x14207bba0 <.text+0x207aba0>
14207bb74: 49 8b 56 18                 	movq	0x18(%r14), %rdx
14207bb78: 48 f7 d1                    	notq	%rcx
14207bb7b: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14207bb7f: 48 c1 e1 04                 	shlq	$0x4, %rcx
14207bb83: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
14207bb88: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
14207bb8f: 48 8b 96 28 01 00 00        	movq	0x128(%rsi), %rdx
14207bb96: 48 39 d0                    	cmpq	%rdx, %rax
14207bb99: 76 55                       	jbe	0x14207bbf0 <.text+0x207abf0>
14207bb9b: eb 1a                       	jmp	0x14207bbb7 <.text+0x207abb7>
14207bb9d: 0f 1f 00                    	nopl	(%rax)
14207bba0: 48 c7 85 b8 00 00 00 00 00 00 00    	movq	$0x0, 0xb8(%rbp)
14207bbab: 48 8b 96 28 01 00 00        	movq	0x128(%rsi), %rdx
14207bbb2: 48 39 d0                    	cmpq	%rdx, %rax
14207bbb5: 76 39                       	jbe	0x14207bbf0 <.text+0x207abf0>
14207bbb7: 49 8b 4e 38                 	movq	0x38(%r14), %rcx
14207bbbb: 48 83 3c d1 00              	cmpq	$0x0, (%rcx,%rdx,8)
14207bbc0: 74 2e                       	je	0x14207bbf0 <.text+0x207abf0>
14207bbc2: 4c 89 f1                    	movq	%r14, %rcx
14207bbc5: e8 a6 4b c8 fe              	callq	0x140d00770 <.text+0xcff770>
14207bbca: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14207bbd1: 49 8b 46 40                 	movq	0x40(%r14), %rax
14207bbd5: b1 01                       	movb	$0x1, %cl
14207bbd7: 89 8d c8 00 00 00           	movl	%ecx, 0xc8(%rbp)
14207bbdd: 48 8b 96 30 01 00 00        	movq	0x130(%rsi), %rdx
14207bbe4: 48 39 d0                    	cmpq	%rdx, %rax
14207bbe7: 76 47                       	jbe	0x14207bc30 <.text+0x207ac30>
14207bbe9: eb 1b                       	jmp	0x14207bc06 <.text+0x207ac06>
14207bbeb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14207bbf0: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
14207bbfa: 48 8b 96 30 01 00 00        	movq	0x130(%rsi), %rdx
14207bc01: 48 39 d0                    	cmpq	%rdx, %rax
14207bc04: 76 2a                       	jbe	0x14207bc30 <.text+0x207ac30>
14207bc06: 49 8b 46 38                 	movq	0x38(%r14), %rax
14207bc0a: 48 83 3c d0 00              	cmpq	$0x0, (%rax,%rdx,8)
14207bc0f: 74 1f                       	je	0x14207bc30 <.text+0x207ac30>
14207bc11: 4c 89 f1                    	movq	%r14, %rcx
14207bc14: e8 57 4b c8 fe              	callq	0x140d00770 <.text+0xcff770>
14207bc19: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14207bc20: 41 b0 01                    	movb	$0x1, %r8b
14207bc23: eb 0e                       	jmp	0x14207bc33 <.text+0x207ac33>
14207bc25: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14207bc30: 45 31 c0                    	xorl	%r8d, %r8d
14207bc33: 48 8b 1f                    	movq	(%rdi), %rbx
14207bc36: 31 ff                       	xorl	%edi, %edi
14207bc38: f6 85 c8 00 00 00 01        	testb	$0x1, 0xc8(%rbp)
14207bc3f: 74 1a                       	je	0x14207bc5b <.text+0x207ac5b>
14207bc41: 89 f8                       	movl	%edi, %eax
14207bc43: 8b 8d a8 00 00 00           	movl	0xa8(%rbp), %ecx
14207bc49: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
14207bc50: 2b 0c 82                    	subl	(%rdx,%rax,4), %ecx
14207bc53: 3b 8d c0 00 00 00           	cmpl	0xc0(%rbp), %ecx
14207bc59: 72 28                       	jb	0x14207bc83 <.text+0x207ac83>
14207bc5b: 41 f6 c0 01                 	testb	$0x1, %r8b
14207bc5f: 0f 84 0b fe ff ff           	je	0x14207ba70 <.text+0x207aa70>
14207bc65: 89 f8                       	movl	%edi, %eax
14207bc67: 8b 8d a8 00 00 00           	movl	0xa8(%rbp), %ecx
14207bc6d: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
14207bc74: 2b 0c 82                    	subl	(%rdx,%rax,4), %ecx
14207bc77: 3b 8d c0 00 00 00           	cmpl	0xc0(%rbp), %ecx
14207bc7d: 0f 83 ed fd ff ff           	jae	0x14207ba70 <.text+0x207aa70>
14207bc83: 44 89 85 a0 00 00 00        	movl	%r8d, 0xa0(%rbp)
14207bc8a: 48 89 75 68                 	movq	%rsi, 0x68(%rbp)
14207bc8e: 48 8d 0c 40                 	leaq	(%rax,%rax,2), %rcx
14207bc92: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
14207bc99: 48 8d 0c 8a                 	leaq	(%rdx,%rcx,4), %rcx
14207bc9d: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14207bca1: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
14207bca5: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
14207bcac: 48 8d 04 c1                 	leaq	(%rcx,%rax,8), %rax
14207bcb0: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
14207bcb7: 48 8b 48 20                 	movq	0x20(%rax), %rcx
14207bcbb: 48 89 4d 00                 	movq	%rcx, (%rbp)
14207bcbf: 48 89 4d 78                 	movq	%rcx, 0x78(%rbp)
14207bcc3: 48 89 4d c0                 	movq	%rcx, -0x40(%rbp)
14207bcc7: 48 8b 75 08                 	movq	0x8(%rbp), %rsi
14207bccb: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
14207bcd0: 0f 84 7f 01 00 00           	je	0x14207be55 <.text+0x207ae55>
14207bcd6: 48 ff c7                    	incq	%rdi
14207bcd9: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
14207bcdd: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
14207bce1: e8 8a 50 53 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14207bce6: 4c 8b 16                    	movq	(%rsi), %r10
14207bce9: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
14207bced: 48 89 ca                    	movq	%rcx, %rdx
14207bcf0: 48 21 c2                    	andq	%rax, %rdx
14207bcf3: 48 c1 e8 39                 	shrq	$0x39, %rax
14207bcf7: 66 0f 6e c0                 	movd	%eax, %xmm0
14207bcfb: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14207bcff: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14207bd04: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14207bd09: 4d 8d b2 58 fd ff ff        	leaq	-0x2a8(%r10), %r14
14207bd10: 31 c0                       	xorl	%eax, %eax
14207bd12: f3 41 0f 6f 0c 12           	movdqu	(%r10,%rdx), %xmm1
14207bd18: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14207bd1c: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14207bd20: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
14207bd25: 45 85 c9                    	testl	%r9d, %r9d
14207bd28: 48 8b 75 68                 	movq	0x68(%rbp), %rsi
14207bd2c: 74 32                       	je	0x14207bd60 <.text+0x207ad60>
14207bd2e: 4d 89 d0                    	movq	%r10, %r8
14207bd31: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
14207bd36: 49 01 d2                    	addq	%rdx, %r10
14207bd39: 49 21 ca                    	andq	%rcx, %r10
14207bd3c: 49 f7 da                    	negq	%r10
14207bd3f: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
14207bd46: 4c 8b 5d 78                 	movq	0x78(%rbp), %r11
14207bd4a: 4f 39 1c 16                 	cmpq	%r11, (%r14,%r10)
14207bd4e: 74 32                       	je	0x14207bd82 <.text+0x207ad82>
14207bd50: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
14207bd54: 66 45 21 ca                 	andw	%r9w, %r10w
14207bd58: 45 89 d1                    	movl	%r10d, %r9d
14207bd5b: 4d 89 c2                    	movq	%r8, %r10
14207bd5e: 75 ce                       	jne	0x14207bd2e <.text+0x207ad2e>
14207bd60: 66 0f 74 cf                 	pcmpeqb	%xmm7, %xmm1
14207bd64: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
14207bd69: 45 85 c9                    	testl	%r9d, %r9d
14207bd6c: 0f 85 e3 00 00 00           	jne	0x14207be55 <.text+0x207ae55>
14207bd72: 48 01 c2                    	addq	%rax, %rdx
14207bd75: 48 83 c2 10                 	addq	$0x10, %rdx
14207bd79: 48 83 c0 10                 	addq	$0x10, %rax
14207bd7d: 48 21 ca                    	andq	%rcx, %rdx
14207bd80: eb 90                       	jmp	0x14207bd12 <.text+0x207ad12>
14207bd82: 4b 83 bc 10 70 fd ff ff 02  	cmpq	$0x2, -0x290(%r8,%r10)
14207bd8b: 0f 83 7b f8 ff ff           	jae	0x14207b60c <.text+0x207a60c>
14207bd91: 48 8d 0d fb e3 a5 00        	leaq	0xa5e3fb(%rip), %rcx    # 0x142ada193
14207bd98: 4c 8d 05 01 79 f5 00        	leaq	0xf57901(%rip), %r8     # 0x142fd36a0
14207bd9f: ba 32 00 00 00              	movl	$0x32, %edx
14207bda4: e8 87 d6 85 00              	callq	0x1428d9430 <.text+0x28d8430>
14207bda9: e9 b3 00 00 00              	jmp	0x14207be61 <.text+0x207ae61>
14207bdae: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
14207bdb2: 48 89 f1                    	movq	%rsi, %rcx
14207bdb5: e8 56 60 c0 fe              	callq	0x140c81e10 <.text+0xc80e10>
14207bdba: 48 89 f1                    	movq	%rsi, %rcx
14207bdbd: 48 83 c9 01                 	orq	$0x1, %rcx
14207bdc1: e8 4a 60 c0 fe              	callq	0x140c81e10 <.text+0xc80e10>
14207bdc6: 31 c9                       	xorl	%ecx, %ecx
14207bdc8: b8 08 00 00 00              	movl	$0x8, %eax
14207bdcd: f0                          	lock
14207bdce: 48 0f b1 0e                 	cmpxchgq	%rcx, (%rsi)
14207bdd2: 0f 85 d5 00 00 00           	jne	0x14207bead <.text+0x207aead>
14207bdd8: 48 83 7d 48 00              	cmpq	$0x0, 0x48(%rbp)
14207bddd: 74 44                       	je	0x14207be23 <.text+0x207ae23>
14207bddf: e8 4c c3 b9 fe              	callq	0x140c18130 <.text+0xc17130>
14207bde4: 48 89 c1                    	movq	%rax, %rcx
14207bde7: 48 8b 00                    	movq	(%rax), %rax
14207bdea: 48 83 f8 01                 	cmpq	$0x1, %rax
14207bdee: 75 73                       	jne	0x14207be63 <.text+0x207ae63>
14207bdf0: 48 83 c1 08                 	addq	$0x8, %rcx
14207bdf4: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14207bdf8: 0f 85 a3 00 00 00           	jne	0x14207bea1 <.text+0x207aea1>
14207bdfe: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14207be05: 48 89 8d c8 00 00 00        	movq	%rcx, 0xc8(%rbp)
14207be0c: 48 83 c1 08                 	addq	$0x8, %rcx
14207be10: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
14207be14: e8 d7 b2 b9 fe              	callq	0x140c170f0 <.text+0xc160f0>
14207be19: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
14207be20: 48 ff 00                    	incq	(%rax)
14207be23: 0f 28 b5 e0 00 00 00        	movaps	0xe0(%rbp), %xmm6
14207be2a: 0f 28 bd f0 00 00 00        	movaps	0xf0(%rbp), %xmm7
14207be31: 44 0f 28 85 00 01 00 00     	movaps	0x100(%rbp), %xmm8
14207be39: 44 0f 28 8d 10 01 00 00     	movaps	0x110(%rbp), %xmm9
14207be41: 48 81 c4 a8 01 00 00        	addq	$0x1a8, %rsp            # imm = 0x1A8
14207be48: 5b                          	popq	%rbx
14207be49: 5f                          	popq	%rdi
14207be4a: 5e                          	popq	%rsi
14207be4b: 41 5c                       	popq	%r12
14207be4d: 41 5d                       	popq	%r13
14207be4f: 41 5e                       	popq	%r14
14207be51: 41 5f                       	popq	%r15
14207be53: 5d                          	popq	%rbp
14207be54: c3                          	retq
14207be55: 48 8d 0d 5c 78 f5 00        	leaq	0xf5785c(%rip), %rcx    # 0x142fd36b8
14207be5c: e8 af d4 85 00              	callq	0x1428d9310 <.text+0x28d8310>
14207be61: 0f 0b                       	ud2
14207be63: 48 85 c0                    	testq	%rax, %rax
14207be66: 75 2d                       	jne	0x14207be95 <.text+0x207ae95>
14207be68: 31 d2                       	xorl	%edx, %edx
14207be6a: e8 61 c9 87 00              	callq	0x1428f87d0 <.text+0x28f77d0>
14207be6f: 48 89 c1                    	movq	%rax, %rcx
14207be72: 48 85 c0                    	testq	%rax, %rax
14207be75: 0f 85 79 ff ff ff           	jne	0x14207bdf4 <.text+0x207adf4>
14207be7b: eb 18                       	jmp	0x14207be95 <.text+0x207ae95>
14207be7d: 48 85 c0                    	testq	%rax, %rax
14207be80: 75 13                       	jne	0x14207be95 <.text+0x207ae95>
14207be82: 31 d2                       	xorl	%edx, %edx
14207be84: e8 47 c9 87 00              	callq	0x1428f87d0 <.text+0x28f77d0>
14207be89: 48 89 c1                    	movq	%rax, %rcx
14207be8c: 48 85 c0                    	testq	%rax, %rax
14207be8f: 0f 85 3b f6 ff ff           	jne	0x14207b4d0 <.text+0x207a4d0>
14207be95: 48 8d 0d 04 ec bb 00        	leaq	0xbbec04(%rip), %rcx    # 0x142c3aaa0
14207be9c: e8 2f c2 85 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14207bea1: 48 8d 0d c8 ec bb 00        	leaq	0xbbecc8(%rip), %rcx    # 0x142c3ab70
14207bea8: e8 a3 d3 85 00              	callq	0x1428d9250 <.text+0x28d8250>
14207bead: 48 89 f1                    	movq	%rsi, %rcx
14207beb0: 31 d2                       	xorl	%edx, %edx
14207beb2: e8 a9 00 85 00              	callq	0x1428cbf60 <.text+0x28caf60>
14207beb7: e9 1c ff ff ff              	jmp	0x14207bdd8 <.text+0x207add8>
14207bebc: 0f 1f 40 00                 	nopl	(%rax)
14207bec0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207bec5: 55                          	pushq	%rbp
14207bec6: 41 57                       	pushq	%r15
14207bec8: 41 56                       	pushq	%r14
14207beca: 41 55                       	pushq	%r13
14207becc: 41 54                       	pushq	%r12
14207bece: 56                          	pushq	%rsi
14207becf: 57                          	pushq	%rdi
14207bed0: 53                          	pushq	%rbx
14207bed1: 48 83 ec 78                 	subq	$0x78, %rsp
14207bed5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207bedc: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
14207bee2: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14207bee8: 66 0f 7f 7c 24 50           	movdqa	%xmm7, 0x50(%rsp)
14207beee: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14207bef3: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
14207befa: 48 ff 00                    	incq	(%rax)
14207befd: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14207bf02: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14207bf07: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14207bf0d: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
14207bf13: 48 83 c4 78                 	addq	$0x78, %rsp
14207bf17: 5b                          	popq	%rbx
14207bf18: 5f                          	popq	%rdi
14207bf19: 5e                          	popq	%rsi
14207bf1a: 41 5c                       	popq	%r12
14207bf1c: 41 5d                       	popq	%r13
14207bf1e: 41 5e                       	popq	%r14
14207bf20: 41 5f                       	popq	%r15
14207bf22: 5d                          	popq	%rbp
14207bf23: c3                          	retq
14207bf24: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14207bf30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207bf35: 55                          	pushq	%rbp
14207bf36: 41 57                       	pushq	%r15
14207bf38: 41 56                       	pushq	%r14
14207bf3a: 41 55                       	pushq	%r13
14207bf3c: 41 54                       	pushq	%r12
14207bf3e: 56                          	pushq	%rsi
14207bf3f: 57                          	pushq	%rdi
14207bf40: 53                          	pushq	%rbx
14207bf41: 48 83 ec 78                 	subq	$0x78, %rsp
14207bf45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207bf4c: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
14207bf52: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14207bf58: 66 0f 7f 7c 24 50           	movdqa	%xmm7, 0x50(%rsp)
14207bf5e: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14207bf63: 48 83 7d 48 00              	cmpq	$0x0, 0x48(%rbp)
14207bf68: 74 20                       	je	0x14207bf8a <.text+0x207af8a>
14207bf6a: 48 8d 45 50                 	leaq	0x50(%rbp), %rax
14207bf6e: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14207bf72: 48 8d 45 f8                 	leaq	-0x8(%rbp), %rax
14207bf76: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14207bf7a: 48 8d 0d d7 73 f5 00        	leaq	0xf573d7(%rip), %rcx    # 0x142fd3358
14207bf81: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
14207bf85: e8 46 33 2b ff              	callq	0x14132f2d0 <.text+0x132e2d0>
14207bf8a: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14207bf8f: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14207bf94: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14207bf9a: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
14207bfa0: 48 83 c4 78                 	addq	$0x78, %rsp
14207bfa4: 5b                          	popq	%rbx
14207bfa5: 5f                          	popq	%rdi
14207bfa6: 5e                          	popq	%rsi
14207bfa7: 41 5c                       	popq	%r12
14207bfa9: 41 5d                       	popq	%r13
14207bfab: 41 5e                       	popq	%r14
14207bfad: 41 5f                       	popq	%r15
14207bfaf: 5d                          	popq	%rbp
14207bfb0: c3                          	retq
14207bfb1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14207bfc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207bfc5: 55                          	pushq	%rbp
14207bfc6: 41 57                       	pushq	%r15
14207bfc8: 41 56                       	pushq	%r14
14207bfca: 41 55                       	pushq	%r13
14207bfcc: 41 54                       	pushq	%r12
14207bfce: 56                          	pushq	%rsi
14207bfcf: 57                          	pushq	%rdi
14207bfd0: 53                          	pushq	%rbx
14207bfd1: 48 83 ec 78                 	subq	$0x78, %rsp
14207bfd5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207bfdc: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
14207bfe2: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14207bfe8: 66 0f 7f 7c 24 50           	movdqa	%xmm7, 0x50(%rsp)
14207bfee: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14207bff3: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
14207bff7: e8 d4 52 fc fd              	callq	0x1400412d0 <.text+0x402d0>
14207bffc: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14207c001: 66 0f 6f 7c 24 50           	movdqa	0x50(%rsp), %xmm7
14207c007: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14207c00d: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
14207c013: 48 83 c4 78                 	addq	$0x78, %rsp
14207c017: 5b                          	popq	%rbx
14207c018: 5f                          	popq	%rdi
14207c019: 5e                          	popq	%rsi
14207c01a: 41 5c                       	popq	%r12
14207c01c: 41 5d                       	popq	%r13
14207c01e: 41 5e                       	popq	%r14
14207c020: 41 5f                       	popq	%r15
14207c022: 5d                          	popq	%rbp
14207c023: c3                          	retq
14207c024: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14207c030: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14207c035: 55                          	pushq	%rbp
14207c036: 41 57                       	pushq	%r15
14207c038: 41 56                       	pushq	%r14
14207c03a: 41 55                       	pushq	%r13
14207c03c: 41 54                       	pushq	%r12
14207c03e: 56                          	pushq	%rsi
14207c03f: 57                          	pushq	%rdi
14207c040: 53                          	pushq	%rbx
14207c041: 48 83 ec 78                 	subq	$0x78, %rsp
14207c045: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14207c04c: 44 0f 29 4c 24 30           	movaps	%xmm9, 0x30(%rsp)
14207c052: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
14207c058: 66 0f 7f 7c 24 50           	movdqa	%xmm7, 0x50(%rsp)
14207c05e: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14207c063: 48 8b 85 c8 00 00 00        	movq	0xc8(%rbp), %rax
14207c06a: 48 ff 00                    	incq	(%rax)
14207c06d: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14207c072: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14207c077: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
14207c07d: 44 0f 28 4c 24 30           	movaps	0x30(%rsp), %xmm9
14207c083: 48 83 c4 78                 	addq	$0x78, %rsp
14207c087: 5b                          	popq	%rbx
14207c088: 5f                          	popq	%rdi
14207c089: 5e                          	popq	%rsi
14207c08a: 41 5c                       	popq	%r12
14207c08c: 41 5d                       	popq	%r13
14207c08e: 41 5e                       	popq	%r14
14207c090: 41 5f                       	popq	%r15
14207c092: 5d                          	popq	%rbp
14207c093: c3                          	retq
