
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14127c7f0: 41 56                       	pushq	%r14
14127c7f2: 56                          	pushq	%rsi
14127c7f3: 57                          	pushq	%rdi
14127c7f4: 53                          	pushq	%rbx
14127c7f5: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14127c7fc: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
14127c805: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14127c80a: 0f 29 74 24 60              	movaps	%xmm6, 0x60(%rsp)
14127c80f: 48 89 ce                    	movq	%rcx, %rsi
14127c812: 41 f6 c0 01                 	testb	$0x1, %r8b
14127c816: 74 7f                       	je	0x14127c897 <.text+0x127b897>
14127c818: 0f 28 f3                    	movaps	%xmm3, %xmm6
14127c81b: 48 89 d7                    	movq	%rdx, %rdi
14127c81e: 48 8b 12                    	movq	(%rdx), %rdx
14127c821: f6 42 08 01                 	testb	$0x1, 0x8(%rdx)
14127c825: 74 7c                       	je	0x14127c8a3 <.text+0x127b8a3>
14127c827: 4c 8b 77 08                 	movq	0x8(%rdi), %r14
14127c82b: 41 8b 46 08                 	movl	0x8(%r14), %eax
14127c82f: 89 44 24 28                 	movl	%eax, 0x28(%rsp)
14127c833: 49 8b 06                    	movq	(%r14), %rax
14127c836: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14127c83b: 48 8d 5c 24 44              	leaq	0x44(%rsp), %rbx
14127c840: 4c 8d 44 24 20              	leaq	0x20(%rsp), %r8
14127c845: 48 89 d9                    	movq	%rbx, %rcx
14127c848: e8 03 7f 66 ff              	callq	0x1408e4750 <.text+0x8e3750>
14127c84d: 48 89 d9                    	movq	%rbx, %rcx
14127c850: e8 eb 2e f8 ff              	callq	0x1411ff740 <.text+0x11fe740>
14127c855: 0f 10 00                    	movups	(%rax), %xmm0
14127c858: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
14127c85d: 48 8b 40 10                 	movq	0x10(%rax), %rax
14127c861: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14127c866: f3 41 0f 10 0e              	movss	(%r14), %xmm1
14127c86b: f3 41 0f 10 56 08           	movss	0x8(%r14), %xmm2
14127c871: 48 8d 5c 24 20              	leaq	0x20(%rsp), %rbx
14127c876: 48 89 d9                    	movq	%rbx, %rcx
14127c879: e8 32 e4 a1 ff              	callq	0x140c9acb0 <.text+0xc99cb0>
14127c87e: 48 89 d9                    	movq	%rbx, %rcx
14127c881: 48 89 c2                    	movq	%rax, %rdx
14127c884: e8 c7 01 a2 ff              	callq	0x140c9ca50 <.text+0xc9ba50>
14127c889: f3 0f 5e f0                 	divss	%xmm0, %xmm6
14127c88d: f3 0f 59 35 bb 90 6a 01     	mulss	0x16a90bb(%rip), %xmm6  # 0x142925950
14127c895: eb 3d                       	jmp	0x14127c8d4 <.text+0x127b8d4>
14127c897: 48 c7 06 03 00 00 00        	movq	$0x3, (%rsi)
14127c89e: e9 8a 00 00 00              	jmp	0x14127c92d <.text+0x127b92d>
14127c8a3: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14127c8a7: 8b 48 08                    	movl	0x8(%rax), %ecx
14127c8aa: 89 4c 24 28                 	movl	%ecx, 0x28(%rsp)
14127c8ae: 48 8b 00                    	movq	(%rax), %rax
14127c8b1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14127c8b6: 48 8d 5c 24 44              	leaq	0x44(%rsp), %rbx
14127c8bb: 4c 8d 44 24 20              	leaq	0x20(%rsp), %r8
14127c8c0: 48 89 d9                    	movq	%rbx, %rcx
14127c8c3: e8 88 7e 66 ff              	callq	0x1408e4750 <.text+0x8e3750>
14127c8c8: 48 89 d9                    	movq	%rbx, %rcx
14127c8cb: e8 70 2d f8 ff              	callq	0x1411ff640 <.text+0x11fe640>
14127c8d0: f3 0f 5e f0                 	divss	%xmm0, %xmm6
14127c8d4: 48 8b 47 10                 	movq	0x10(%rdi), %rax
14127c8d8: f3 0f 58 30                 	addss	(%rax), %xmm6
14127c8dc: f3 0f 10 78 04              	movss	0x4(%rax), %xmm7
14127c8e1: 0f 2e 35 1c 90 6a 01        	ucomiss	0x16a901c(%rip), %xmm6  # 0x142925904
14127c8e8: 76 0a                       	jbe	0x14127c8f4 <.text+0x127b8f4>
14127c8ea: f3 0f 58 35 f2 d7 6d 01     	addss	0x16dd7f2(%rip), %xmm6  # 0x14295a0e4
14127c8f2: eb 10                       	jmp	0x14127c904 <.text+0x127b904>
14127c8f4: 0f 57 c0                    	xorps	%xmm0, %xmm0
14127c8f7: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
14127c8fa: 76 08                       	jbe	0x14127c904 <.text+0x127b904>
14127c8fc: f3 0f 58 35 00 90 6a 01     	addss	0x16a9000(%rip), %xmm6  # 0x142925904
14127c904: 48 8b 47 18                 	movq	0x18(%rdi), %rax
14127c908: 44 0f 10 00                 	movups	(%rax), %xmm8
14127c90c: e8 5f f2 db fe              	callq	0x14003bb70 <.text+0x3ab70>
14127c911: 44 0f 11 06                 	movups	%xmm8, (%rsi)
14127c915: f3 0f 11 76 10              	movss	%xmm6, 0x10(%rsi)
14127c91a: f3 0f 11 7e 14              	movss	%xmm7, 0x14(%rsi)
14127c91f: c7 46 18 00 00 00 00        	movl	$0x0, 0x18(%rsi)
14127c926: c6 46 32 02                 	movb	$0x2, 0x32(%rsi)
14127c92a: 88 46 38                    	movb	%al, 0x38(%rsi)
14127c92d: 0f 28 74 24 60              	movaps	0x60(%rsp), %xmm6
14127c932: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14127c937: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
14127c940: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14127c947: 5b                          	popq	%rbx
14127c948: 5f                          	popq	%rdi
14127c949: 5e                          	popq	%rsi
14127c94a: 41 5e                       	popq	%r14
14127c94c: c3                          	retq
