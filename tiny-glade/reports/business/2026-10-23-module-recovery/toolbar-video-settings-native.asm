
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140247870: 55                          	pushq	%rbp
140247871: 41 57                       	pushq	%r15
140247873: 41 56                       	pushq	%r14
140247875: 41 55                       	pushq	%r13
140247877: 41 54                       	pushq	%r12
140247879: 56                          	pushq	%rsi
14024787a: 57                          	pushq	%rdi
14024787b: 53                          	pushq	%rbx
14024787c: 48 81 ec 28 03 00 00        	subq	$0x328, %rsp            # imm = 0x328
140247883: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14024788b: 0f 29 bd 90 02 00 00        	movaps	%xmm7, 0x290(%rbp)
140247892: 0f 29 b5 80 02 00 00        	movaps	%xmm6, 0x280(%rbp)
140247899: 48 c7 85 78 02 00 00 fe ff ff ff    	movq	$-0x2, 0x278(%rbp)
1402478a4: 4c 89 8d 68 02 00 00        	movq	%r9, 0x268(%rbp)
1402478ab: 49 89 ce                    	movq	%rcx, %r14
1402478ae: 0f b6 0a                    	movzbl	(%rdx), %ecx
1402478b1: 85 c9                       	testl	%ecx, %ecx
1402478b3: 74 4c                       	je	0x140247901 <.text+0x246901>
1402478b5: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
1402478bc: 83 f9 01                    	cmpl	$0x1, %ecx
1402478bf: 0f 85 48 02 00 00           	jne	0x140247b0d <.text+0x246b0d>
1402478c5: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
1402478cc: 4c 8b 38                    	movq	(%rax), %r15
1402478cf: 48 8b 01                    	movq	(%rcx), %rax
1402478d2: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1402478d9: 48 8b 18                    	movq	(%rax), %rbx
1402478dc: 4c 89 f9                    	movq	%r15, %rcx
1402478df: e8 bc 03 74 00              	callq	0x140987ca0 <.text+0x986ca0>
1402478e4: 49 8b 3e                    	movq	(%r14), %rdi
1402478e7: 80 bf 88 00 00 00 00        	cmpb	$0x0, 0x88(%rdi)
1402478ee: 0f 85 4c 03 00 00           	jne	0x140247c40 <.text+0x246c40>
1402478f4: 66 0f 6e 1d 08 e0 6d 02     	movd	0x26de008(%rip), %xmm3  # 0x142925904
1402478fc: e9 47 03 00 00              	jmp	0x140247c48 <.text+0x246c48>
140247901: 48 8d 0d dc 93 75 02        	leaq	0x27593dc(%rip), %rcx   # 0x1429a0ce4
140247908: ba 19 00 00 00              	movl	$0x19, %edx
14024790d: e8 6e 5b 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140247912: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140247919: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
140247920: 48 8d b5 d0 01 00 00        	leaq	0x1d0(%rbp), %rsi
140247927: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
14024792e: 4c 8d 35 5b 25 ec ff        	leaq	-0x13daa5(%rip), %r14   # 0x140109e90 <.text+0x108e90>
140247935: 4c 89 b5 b8 01 00 00        	movq	%r14, 0x1b8(%rbp)
14024793c: 4c 8d 3d bd 93 75 02        	leaq	0x27593bd(%rip), %r15   # 0x1429a0d00
140247943: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
14024794a: 48 c7 85 08 01 00 00 01 00 00 00    	movq	$0x1, 0x108(%rbp)
140247955: 4c 8d a5 b0 01 00 00        	leaq	0x1b0(%rbp), %r12
14024795c: 4c 89 a5 10 01 00 00        	movq	%r12, 0x110(%rbp)
140247963: 48 c7 85 18 01 00 00 01 00 00 00    	movq	$0x1, 0x118(%rbp)
14024796e: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140247979: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140247980: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140247987: e8 04 26 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
14024798c: 4c 8b 85 28 02 00 00        	movq	0x228(%rbp), %r8
140247993: 4c 8b 8d 30 02 00 00        	movq	0x230(%rbp), %r9
14024799a: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1402479a1: 48 8b 95 68 02 00 00        	movq	0x268(%rbp), %rdx
1402479a8: 4c 89 85 70 02 00 00        	movq	%r8, 0x270(%rbp)
1402479af: e8 dc 4d 44 00              	callq	0x14068c790 <.text+0x68b790>
1402479b4: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1402479bb: e8 90 a3 43 00              	callq	0x140681d50 <.text+0x680d50>
1402479c0: 48 89 c1                    	movq	%rax, %rcx
1402479c3: e8 b8 c2 43 00              	callq	0x140683c80 <.text+0x682c80>
1402479c8: 89 c3                       	movl	%eax, %ebx
1402479ca: 48 8b 95 20 02 00 00        	movq	0x220(%rbp), %rdx
1402479d1: 48 85 d2                    	testq	%rdx, %rdx
1402479d4: 74 12                       	je	0x1402479e8 <.text+0x2469e8>
1402479d6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402479dc: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1402479e3: e8 38 c2 3c 00              	callq	0x140613c20 <.text+0x612c20>
1402479e8: 48 8b bd 40 03 00 00        	movq	0x340(%rbp), %rdi
1402479ef: 84 db                       	testb	%bl, %bl
1402479f1: 74 21                       	je	0x140247a14 <.text+0x246a14>
1402479f3: c6 85 08 01 00 00 01        	movb	$0x1, 0x108(%rbp)
1402479fa: 48 c7 85 00 01 00 00 07 00 00 00    	movq	$0x7, 0x100(%rbp)
140247a05: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140247a0c: 48 89 f9                    	movq	%rdi, %rcx
140247a0f: e8 bc 80 6f 00              	callq	0x14093fad0 <.text+0x93ead0>
140247a14: 48 8d 0d f5 92 75 02        	leaq	0x27592f5(%rip), %rcx   # 0x1429a0d10
140247a1b: ba 1b 00 00 00              	movl	$0x1b, %edx
140247a20: e8 5b 5a 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140247a25: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140247a2c: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
140247a33: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
140247a3a: 4c 89 b5 b8 01 00 00        	movq	%r14, 0x1b8(%rbp)
140247a41: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
140247a48: 48 c7 85 08 01 00 00 01 00 00 00    	movq	$0x1, 0x108(%rbp)
140247a53: 4c 89 a5 10 01 00 00        	movq	%r12, 0x110(%rbp)
140247a5a: 48 c7 85 18 01 00 00 01 00 00 00    	movq	$0x1, 0x118(%rbp)
140247a65: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140247a70: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140247a77: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140247a7e: e8 0d 25 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
140247a83: 4c 8b 85 28 02 00 00        	movq	0x228(%rbp), %r8
140247a8a: 4c 8b 8d 30 02 00 00        	movq	0x230(%rbp), %r9
140247a91: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140247a98: 48 8b 95 68 02 00 00        	movq	0x268(%rbp), %rdx
140247a9f: 4c 89 85 68 02 00 00        	movq	%r8, 0x268(%rbp)
140247aa6: e8 e5 4c 44 00              	callq	0x14068c790 <.text+0x68b790>
140247aab: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140247ab2: e8 99 a2 43 00              	callq	0x140681d50 <.text+0x680d50>
140247ab7: 48 89 c1                    	movq	%rax, %rcx
140247aba: e8 c1 c1 43 00              	callq	0x140683c80 <.text+0x682c80>
140247abf: 89 c3                       	movl	%eax, %ebx
140247ac1: 48 8b 95 20 02 00 00        	movq	0x220(%rbp), %rdx
140247ac8: 48 85 d2                    	testq	%rdx, %rdx
140247acb: 74 12                       	je	0x140247adf <.text+0x246adf>
140247acd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140247ad3: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140247ada: e8 41 c1 3c 00              	callq	0x140613c20 <.text+0x612c20>
140247adf: 84 db                       	testb	%bl, %bl
140247ae1: 0f 84 bb 24 00 00           	je	0x140249fa2 <.text+0x248fa2>
140247ae7: c6 85 08 01 00 00 02        	movb	$0x2, 0x108(%rbp)
140247aee: 48 c7 85 00 01 00 00 07 00 00 00    	movq	$0x7, 0x100(%rbp)
140247af9: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140247b00: 48 89 f9                    	movq	%rdi, %rcx
140247b03: e8 c8 7f 6f 00              	callq	0x14093fad0 <.text+0x93ead0>
140247b08: e9 95 24 00 00              	jmp	0x140249fa2 <.text+0x248fa2>
140247b0d: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
140247b14: 4d 8b 20                    	movq	(%r8), %r12
140247b17: 48 8b 00                    	movq	(%rax), %rax
140247b1a: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
140247b21: 48 8b 01                    	movq	(%rcx), %rax
140247b24: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
140247b2b: 48 8d 0d de 91 75 02        	leaq	0x27591de(%rip), %rcx   # 0x1429a0d10
140247b32: ba 1b 00 00 00              	movl	$0x1b, %edx
140247b37: e8 44 59 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140247b3c: 49 89 d1                    	movq	%rdx, %r9
140247b3f: 48 8d 9d 00 01 00 00        	leaq	0x100(%rbp), %rbx
140247b46: 48 89 d9                    	movq	%rbx, %rcx
140247b49: 48 8b 95 68 02 00 00        	movq	0x268(%rbp), %rdx
140247b50: 49 89 c0                    	movq	%rax, %r8
140247b53: e8 a8 43 44 00              	callq	0x14068bf00 <.text+0x68af00>
140247b58: 48 89 d9                    	movq	%rbx, %rcx
140247b5b: e8 d0 a1 43 00              	callq	0x140681d30 <.text+0x680d30>
140247b60: 66 0f 6e 0d 20 8c 75 02     	movd	0x2758c20(%rip), %xmm1  # 0x1429a0788
140247b68: 48 89 c1                    	movq	%rax, %rcx
140247b6b: e8 e0 e7 eb ff              	callq	0x140106350 <.text+0x105350>
140247b70: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140247b77: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140247b7e: f3 0f 6f 85 00 01 00 00     	movdqu	0x100(%rbp), %xmm0
140247b86: 66 0f 7f 85 20 02 00 00     	movdqa	%xmm0, 0x220(%rbp)
140247b8e: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140247b95: 48 8d 95 20 02 00 00        	leaq	0x220(%rbp), %rdx
140247b9c: 66 41 b8 bc 02              	movw	$0x2bc, %r8w            # imm = 0x2BC
140247ba1: e8 ba 68 43 00              	callq	0x14067e460 <.text+0x67d460>
140247ba6: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
140247baa: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
140247bae: 89 85 60 02 00 00           	movl	%eax, 0x260(%rbp)
140247bb4: 48 89 8d 08 02 00 00        	movq	%rcx, 0x208(%rbp)
140247bbb: 89 01                       	movl	%eax, (%rcx)
140247bbd: 49 8b 3e                    	movq	(%r14), %rdi
140247bc0: 80 bf ac 00 00 00 01        	cmpb	$0x1, 0xac(%rdi)
140247bc7: 0f 85 37 17 00 00           	jne	0x140249304 <.text+0x248304>
140247bcd: 4d 8b 7c 24 08              	movq	0x8(%r12), %r15
140247bd2: 4d 85 ff                    	testq	%r15, %r15
140247bd5: 0f 84 29 17 00 00           	je	0x140249304 <.text+0x248304>
140247bdb: 48 8d b7 ad 00 00 00        	leaq	0xad(%rdi), %rsi
140247be2: 48 8b 06                    	movq	(%rsi), %rax
140247be5: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
140247be9: 4d 8b 34 24                 	movq	(%r12), %r14
140247bed: 4f 8d 2c bf                 	leaq	(%r15,%r15,4), %r13
140247bf1: 49 c1 e5 05                 	shlq	$0x5, %r13
140247bf5: 4d 01 f5                    	addq	%r14, %r13
140247bf8: 4c 89 fa                    	movq	%r15, %rdx
140247bfb: 48 c1 e2 05                 	shlq	$0x5, %rdx
140247bff: 48 8d 14 92                 	leaq	(%rdx,%rdx,4), %rdx
140247c03: 45 31 c0                    	xorl	%r8d, %r8d
140247c06: 45 31 c9                    	xorl	%r9d, %r9d
140247c09: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140247c10: 4f 8b 54 06 78              	movq	0x78(%r14,%r8), %r10
140247c15: 49 31 c2                    	xorq	%rax, %r10
140247c18: 4f 8b 9c 06 80 00 00 00     	movq	0x80(%r14,%r8), %r11
140247c20: 49 31 cb                    	xorq	%rcx, %r11
140247c23: 4d 09 d3                    	orq	%r10, %r11
140247c26: 0f 84 aa 04 00 00           	je	0x1402480d6 <.text+0x2470d6>
140247c2c: 49 ff c1                    	incq	%r9
140247c2f: 49 81 c0 a0 00 00 00        	addq	$0xa0, %r8
140247c36: 4c 39 c2                    	cmpq	%r8, %rdx
140247c39: 75 d5                       	jne	0x140247c10 <.text+0x246c10>
140247c3b: e9 c4 16 00 00              	jmp	0x140249304 <.text+0x248304>
140247c40: 66 0f 6e 9f 8c 00 00 00     	movd	0x8c(%rdi), %xmm3
140247c48: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140247c4c: 48 89 c2                    	movq	%rax, %rdx
140247c4f: 49 89 d8                    	movq	%rbx, %r8
140247c52: e8 e9 05 2b 00              	callq	0x1404f8240 <.text+0x4f7240>
140247c57: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
140247c5e: 48 8b 00                    	movq	(%rax), %rax
140247c61: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
140247c68: 80 78 28 00                 	cmpb	$0x0, 0x28(%rax)
140247c6c: 74 23                       	je	0x140247c91 <.text+0x246c91>
140247c6e: 4c 89 f9                    	movq	%r15, %rcx
140247c71: e8 2a 00 74 00              	callq	0x140987ca0 <.text+0x986ca0>
140247c76: 48 89 c1                    	movq	%rax, %rcx
140247c79: 48 89 da                    	movq	%rbx, %rdx
140247c7c: e8 9f 02 2b 00              	callq	0x1404f7f20 <.text+0x4f6f20>
140247c81: 0f 28 f0                    	movaps	%xmm0, %xmm6
140247c84: 0f 28 f8                    	movaps	%xmm0, %xmm7
140247c87: f3 0f 5d 3d 41 dc 6d 02     	minss	0x26ddc41(%rip), %xmm7  # 0x1429258d0
140247c8f: eb 0b                       	jmp	0x140247c9c <.text+0x246c9c>
140247c91: f3 0f 10 35 3b 8b 75 02     	movss	0x2758b3b(%rip), %xmm6  # 0x1429a07d4
140247c99: 0f 28 fe                    	movaps	%xmm6, %xmm7
140247c9c: 48 8d 0d bd 92 75 02        	leaq	0x27592bd(%rip), %rcx   # 0x1429a0f60
140247ca3: ba 21 00 00 00              	movl	$0x21, %edx
140247ca8: e8 d3 57 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140247cad: 49 89 d1                    	movq	%rdx, %r9
140247cb0: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140247cb7: 48 8b 95 68 02 00 00        	movq	0x268(%rbp), %rdx
140247cbe: 49 89 c0                    	movq	%rax, %r8
140247cc1: e8 3a 42 44 00              	callq	0x14068bf00 <.text+0x68af00>
140247cc6: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140247ccd: e8 5e a0 43 00              	callq	0x140681d30 <.text+0x680d30>
140247cd2: f3 0f 10 0d ae 8a 75 02     	movss	0x2758aae(%rip), %xmm1  # 0x1429a0788
140247cda: 48 89 c1                    	movq	%rax, %rcx
140247cdd: e8 6e e6 eb ff              	callq	0x140106350 <.text+0x105350>
140247ce2: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140247ce9: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140247cf0: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
140247cf7: 0f 29 85 20 02 00 00        	movaps	%xmm0, 0x220(%rbp)
140247cfe: 4c 8d a5 00 01 00 00        	leaq	0x100(%rbp), %r12
140247d05: 48 8d 95 20 02 00 00        	leaq	0x220(%rbp), %rdx
140247d0c: 4c 89 e1                    	movq	%r12, %rcx
140247d0f: 66 41 b8 bc 02              	movw	$0x2bc, %r8w            # imm = 0x2BC
140247d14: e8 47 67 43 00              	callq	0x14067e460 <.text+0x67d460>
140247d19: 0f b6 05 e9 ce 3e 03        	movzbl	0x33ecee9(%rip), %eax   # 0x143634c09
140247d20: b9 30 00 00 00              	movl	$0x30, %ecx
140247d25: ba 04 00 00 00              	movl	$0x4, %edx
140247d2a: e8 e1 be 3c 00              	callq	0x140613c10 <.text+0x612c10>
140247d2f: 48 85 c0                    	testq	%rax, %rax
140247d32: 0f 84 9d 23 00 00           	je	0x14024a0d5 <.text+0x2490d5>
140247d38: 48 89 c6                    	movq	%rax, %rsi
140247d3b: c6 00 00                    	movb	$0x0, (%rax)
140247d3e: c7 40 04 00 00 80 3f        	movl	$0x3f800000, 0x4(%rax)  # imm = 0x3F800000
140247d45: 66 c7 40 08 01 01           	movw	$0x101, 0x8(%rax)       # imm = 0x101
140247d4b: c6 40 0a 00                 	movb	$0x0, 0xa(%rax)
140247d4f: c6 40 0c 01                 	movb	$0x1, 0xc(%rax)
140247d53: c7 40 10 00 00 80 3f        	movl	$0x3f800000, 0x10(%rax) # imm = 0x3F800000
140247d5a: 66 c7 40 14 03 01           	movw	$0x103, 0x14(%rax)      # imm = 0x103
140247d60: c6 40 16 01                 	movb	$0x1, 0x16(%rax)
140247d64: c6 40 18 02                 	movb	$0x2, 0x18(%rax)
140247d68: c7 40 1c 00 00 80 3f        	movl	$0x3f800000, 0x1c(%rax) # imm = 0x3F800000
140247d6f: 66 c7 40 20 03 00           	movw	$0x3, 0x20(%rax)
140247d75: c6 40 22 01                 	movb	$0x1, 0x22(%rax)
140247d79: c6 40 24 03                 	movb	$0x3, 0x24(%rax)
140247d7d: c7 40 28 00 00 c0 3f        	movl	$0x3fc00000, 0x28(%rax) # imm = 0x3FC00000
140247d84: 66 c7 40 2c 04 01           	movw	$0x104, 0x2c(%rax)      # imm = 0x104
140247d8a: c6 40 2e 01                 	movb	$0x1, 0x2e(%rax)
140247d8e: 0f b6 05 74 ce 3e 03        	movzbl	0x33ece74(%rip), %eax   # 0x143634c09
140247d95: b9 04 00 00 00              	movl	$0x4, %ecx
140247d9a: ba 01 00 00 00              	movl	$0x1, %edx
140247d9f: e8 6c be 3c 00              	callq	0x140613c10 <.text+0x612c10>
140247da4: 48 85 c0                    	testq	%rax, %rax
140247da7: 48 89 b5 08 02 00 00        	movq	%rsi, 0x208(%rbp)
140247dae: 0f 84 35 23 00 00           	je	0x14024a0e9 <.text+0x2490e9>
140247db4: 0f b6 0e                    	movzbl	(%rsi), %ecx
140247db7: 88 08                       	movb	%cl, (%rax)
140247db9: 0f b6 4e 0c                 	movzbl	0xc(%rsi), %ecx
140247dbd: 88 48 01                    	movb	%cl, 0x1(%rax)
140247dc0: 0f b6 4e 18                 	movzbl	0x18(%rsi), %ecx
140247dc4: 88 48 02                    	movb	%cl, 0x2(%rax)
140247dc7: 0f b6 4e 24                 	movzbl	0x24(%rsi), %ecx
140247dcb: 88 48 03                    	movb	%cl, 0x3(%rax)
140247dce: 44 0f b6 8f ab 00 00 00     	movzbl	0xab(%rdi), %r9d
140247dd6: 80 bf 88 00 00 00 00        	cmpb	$0x0, 0x88(%rdi)
140247ddd: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
140247de4: 75 1e                       	jne	0x140247e04 <.text+0x246e04>
140247de6: f3 0f 10 05 16 db 6d 02     	movss	0x26ddb16(%rip), %xmm0  # 0x142925904
140247dee: 0f b6 97 c3 00 00 00        	movzbl	0xc3(%rdi), %edx
140247df5: 0f b6 8f a9 00 00 00        	movzbl	0xa9(%rdi), %ecx
140247dfc: 44 3a 4e 09                 	cmpb	0x9(%rsi), %r9b
140247e00: 74 1e                       	je	0x140247e20 <.text+0x246e20>
140247e02: eb 33                       	jmp	0x140247e37 <.text+0x246e37>
140247e04: f3 0f 10 87 8c 00 00 00     	movss	0x8c(%rdi), %xmm0
140247e0c: 0f b6 97 c3 00 00 00        	movzbl	0xc3(%rdi), %edx
140247e13: 0f b6 8f a9 00 00 00        	movzbl	0xa9(%rdi), %ecx
140247e1a: 44 3a 4e 09                 	cmpb	0x9(%rsi), %r9b
140247e1e: 75 17                       	jne	0x140247e37 <.text+0x246e37>
140247e20: 3a 56 08                    	cmpb	0x8(%rsi), %dl
140247e23: 75 12                       	jne	0x140247e37 <.text+0x246e37>
140247e25: 3a 4e 0a                    	cmpb	0xa(%rsi), %cl
140247e28: 75 0d                       	jne	0x140247e37 <.text+0x246e37>
140247e2a: 0f 2e 46 04                 	ucomiss	0x4(%rsi), %xmm0
140247e2e: 75 07                       	jne	0x140247e37 <.text+0x246e37>
140247e30: 7a 05                       	jp	0x140247e37 <.text+0x246e37>
140247e32: 45 31 c0                    	xorl	%r8d, %r8d
140247e35: eb 6e                       	jmp	0x140247ea5 <.text+0x246ea5>
140247e37: 44 3a 4e 15                 	cmpb	0x15(%rsi), %r9b
140247e3b: 75 18                       	jne	0x140247e55 <.text+0x246e55>
140247e3d: 3a 56 14                    	cmpb	0x14(%rsi), %dl
140247e40: 75 13                       	jne	0x140247e55 <.text+0x246e55>
140247e42: 3a 4e 16                    	cmpb	0x16(%rsi), %cl
140247e45: 75 0e                       	jne	0x140247e55 <.text+0x246e55>
140247e47: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140247e4d: 0f 2e 46 10                 	ucomiss	0x10(%rsi), %xmm0
140247e51: 75 02                       	jne	0x140247e55 <.text+0x246e55>
140247e53: 7b 50                       	jnp	0x140247ea5 <.text+0x246ea5>
140247e55: 44 3a 4e 21                 	cmpb	0x21(%rsi), %r9b
140247e59: 75 18                       	jne	0x140247e73 <.text+0x246e73>
140247e5b: 3a 56 20                    	cmpb	0x20(%rsi), %dl
140247e5e: 75 13                       	jne	0x140247e73 <.text+0x246e73>
140247e60: 3a 4e 22                    	cmpb	0x22(%rsi), %cl
140247e63: 75 0e                       	jne	0x140247e73 <.text+0x246e73>
140247e65: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140247e6b: 0f 2e 46 1c                 	ucomiss	0x1c(%rsi), %xmm0
140247e6f: 75 02                       	jne	0x140247e73 <.text+0x246e73>
140247e71: 7b 32                       	jnp	0x140247ea5 <.text+0x246ea5>
140247e73: 44 3a 4e 2d                 	cmpb	0x2d(%rsi), %r9b
140247e77: 0f 85 e0 00 00 00           	jne	0x140247f5d <.text+0x246f5d>
140247e7d: 3a 56 2c                    	cmpb	0x2c(%rsi), %dl
140247e80: 0f 85 d7 00 00 00           	jne	0x140247f5d <.text+0x246f5d>
140247e86: 3a 4e 2e                    	cmpb	0x2e(%rsi), %cl
140247e89: 0f 85 ce 00 00 00           	jne	0x140247f5d <.text+0x246f5d>
140247e8f: 41 b8 03 00 00 00           	movl	$0x3, %r8d
140247e95: 0f 2e 46 28                 	ucomiss	0x28(%rsi), %xmm0
140247e99: 0f 85 be 00 00 00           	jne	0x140247f5d <.text+0x246f5d>
140247e9f: 0f 8a b8 00 00 00           	jp	0x140247f5d <.text+0x246f5d>
140247ea5: 4c 89 85 d0 01 00 00        	movq	%r8, 0x1d0(%rbp)
140247eac: 48 8d 0d ce 90 75 02        	leaq	0x27590ce(%rip), %rcx   # 0x1429a0f81
140247eb3: ba 21 00 00 00              	movl	$0x21, %edx
140247eb8: e8 c3 55 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140247ebd: 49 89 c7                    	movq	%rax, %r15
140247ec0: 49 89 d5                    	movq	%rdx, %r13
140247ec3: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140247eca: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140247ed0: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140247ed7: 45 31 c9                    	xorl	%r9d, %r9d
140247eda: e8 11 5e 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140247edf: e8 8c 5e 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140247ee4: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
140247eeb: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140247ef2: 48 c7 85 08 01 00 00 04 00 00 00    	movq	$0x4, 0x108(%rbp)
140247efd: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
140247f04: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140247f0b: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140247f10: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140247f16: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
140247f1d: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140247f24: 4c 89 fa                    	movq	%r15, %rdx
140247f27: 4d 89 e8                    	movq	%r13, %r8
140247f2a: e8 71 7e 0e 00              	callq	0x14032fda0 <.text+0x32eda0>
140247f2f: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
140247f36: 48 83 f9 04                 	cmpq	$0x4, %rcx
140247f3a: 0f 83 d5 21 00 00           	jae	0x14024a115 <.text+0x249115>
140247f40: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140247f44: 48 8b 9d 08 02 00 00        	movq	0x208(%rbp), %rbx
140247f4b: 0f b6 0c 8b                 	movzbl	(%rbx,%rcx,4), %ecx
140247f4f: 80 f9 05                    	cmpb	$0x5, %cl
140247f52: 0f 85 b6 00 00 00           	jne	0x14024800e <.text+0x24700e>
140247f58: e9 67 04 00 00              	jmp	0x1402483c4 <.text+0x2473c4>
140247f5d: 48 c7 85 d0 01 00 00 01 00 00 00    	movq	$0x1, 0x1d0(%rbp)
140247f68: 48 8d 0d 12 90 75 02        	leaq	0x2759012(%rip), %rcx   # 0x1429a0f81
140247f6f: ba 21 00 00 00              	movl	$0x21, %edx
140247f74: e8 07 55 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140247f79: 49 89 c5                    	movq	%rax, %r13
140247f7c: 49 89 d7                    	movq	%rdx, %r15
140247f7f: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140247f86: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140247f8c: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140247f93: 45 31 c9                    	xorl	%r9d, %r9d
140247f96: e8 55 5d 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140247f9b: e8 d0 5d 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140247fa0: 48 8d 05 19 90 75 02        	leaq	0x2759019(%rip), %rax   # 0x1429a0fc0
140247fa7: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140247fae: 48 c7 85 08 01 00 00 03 00 00 00    	movq	$0x3, 0x108(%rbp)
140247fb9: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
140247fc0: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140247fc7: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140247fcc: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140247fd2: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
140247fd9: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140247fe0: 4c 89 ea                    	movq	%r13, %rdx
140247fe3: 4d 89 f8                    	movq	%r15, %r8
140247fe6: e8 b5 7d 0e 00              	callq	0x14032fda0 <.text+0x32eda0>
140247feb: 31 c9                       	xorl	%ecx, %ecx
140247fed: 48 83 bd d0 01 00 00 01     	cmpq	$0x1, 0x1d0(%rbp)
140247ff5: 0f 94 c1                    	sete	%cl
140247ff8: 8d 0c 49                    	leal	(%rcx,%rcx,2), %ecx
140247ffb: 83 c1 02                    	addl	$0x2, %ecx
140247ffe: 48 8b 9d 08 02 00 00        	movq	0x208(%rbp), %rbx
140248005: 80 f9 05                    	cmpb	$0x5, %cl
140248008: 0f 84 b6 03 00 00           	je	0x1402483c4 <.text+0x2473c4>
14024800e: 84 c0                       	testb	%al, %al
140248010: 0f 84 ae 03 00 00           	je	0x1402483c4 <.text+0x2473c4>
140248016: 48 89 da                    	movq	%rbx, %rdx
140248019: 38 0b                       	cmpb	%cl, (%rbx)
14024801b: 74 1f                       	je	0x14024803c <.text+0x24703c>
14024801d: 48 8d 53 0c                 	leaq	0xc(%rbx), %rdx
140248021: 38 0a                       	cmpb	%cl, (%rdx)
140248023: 74 17                       	je	0x14024803c <.text+0x24703c>
140248025: 48 8d 53 18                 	leaq	0x18(%rbx), %rdx
140248029: 38 0a                       	cmpb	%cl, (%rdx)
14024802b: 74 0f                       	je	0x14024803c <.text+0x24703c>
14024802d: 48 89 da                    	movq	%rbx, %rdx
140248030: 48 83 c2 24                 	addq	$0x24, %rdx
140248034: 38 0a                       	cmpb	%cl, (%rdx)
140248036: 0f 85 53 21 00 00           	jne	0x14024a18f <.text+0x24918f>
14024803c: 0f 2e 72 04                 	ucomiss	0x4(%rdx), %xmm6
140248040: 0f 82 7e 03 00 00           	jb	0x1402483c4 <.text+0x2473c4>
140248046: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14024804d: c6 00 01                    	movb	$0x1, (%rax)
140248050: 44 0f b6 42 09              	movzbl	0x9(%rdx), %r8d
140248055: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
140248059: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
14024805d: 89 01                       	movl	%eax, (%rcx)
14024805f: 44 88 87 ab 00 00 00        	movb	%r8b, 0xab(%rdi)
140248066: 44 0f b6 42 08              	movzbl	0x8(%rdx), %r8d
14024806b: 89 01                       	movl	%eax, (%rcx)
14024806d: 44 88 87 c3 00 00 00        	movb	%r8b, 0xc3(%rdi)
140248074: 44 0f b6 42 0a              	movzbl	0xa(%rdx), %r8d
140248079: 89 01                       	movl	%eax, (%rcx)
14024807b: 44 88 87 a9 00 00 00        	movb	%r8b, 0xa9(%rdi)
140248082: f3 0f 10 42 04              	movss	0x4(%rdx), %xmm0
140248087: 4c 8b 8d 30 03 00 00        	movq	0x330(%rbp), %r9
14024808e: 45 8b 41 1c                 	movl	0x1c(%r9), %r8d
140248092: 4d 8b 49 10                 	movq	0x10(%r9), %r9
140248096: 45 89 01                    	movl	%r8d, (%r9)
140248099: 4c 8b 85 50 02 00 00        	movq	0x250(%rbp), %r8
1402480a0: f3 41 0f 11 40 18           	movss	%xmm0, 0x18(%r8)
1402480a6: f3 0f 10 42 04              	movss	0x4(%rdx), %xmm0
1402480ab: 0f 2e 05 52 d8 6d 02        	ucomiss	0x26dd852(%rip), %xmm0  # 0x142925904
1402480b2: 44 8b 87 88 00 00 00        	movl	0x88(%rdi), %r8d
1402480b9: 0f 85 c1 02 00 00           	jne	0x140248380 <.text+0x247380>
1402480bf: 0f 8a bb 02 00 00           	jp	0x140248380 <.text+0x247380>
1402480c5: 31 d2                       	xorl	%edx, %edx
1402480c7: 41 f6 c0 01                 	testb	$0x1, %r8b
1402480cb: 0f 85 c5 02 00 00           	jne	0x140248396 <.text+0x247396>
1402480d1: e9 ee 02 00 00              	jmp	0x1402483c4 <.text+0x2473c4>
1402480d6: 4c 89 8d b0 01 00 00        	movq	%r9, 0x1b0(%rbp)
1402480dd: 4a 8d 04 fd 00 00 00 00     	leaq	(,%r15,8), %rax
1402480e5: 48 8d 1c 40                 	leaq	(%rax,%rax,2), %rbx
1402480e9: 0f b6 05 19 cb 3e 03        	movzbl	0x33ecb19(%rip), %eax   # 0x143634c09
1402480f0: ba 08 00 00 00              	movl	$0x8, %edx
1402480f5: 48 89 d9                    	movq	%rbx, %rcx
1402480f8: e8 13 bb 3c 00              	callq	0x140613c10 <.text+0x612c10>
1402480fd: 48 85 c0                    	testq	%rax, %rax
140248100: 0f 84 59 20 00 00           	je	0x14024a15f <.text+0x24915f>
140248106: 4c 89 bd 20 02 00 00        	movq	%r15, 0x220(%rbp)
14024810d: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
140248114: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
14024811b: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
140248126: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
14024812d: 48 c7 85 08 01 00 00 00 00 00 00    	movq	$0x0, 0x108(%rbp)
140248138: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14024813f: 4c 8d bd 00 01 00 00        	leaq	0x100(%rbp), %r15
140248146: 4c 89 f1                    	movq	%r14, %rcx
140248149: 4c 89 ea                    	movq	%r13, %rdx
14024814c: 4d 89 f8                    	movq	%r15, %r8
14024814f: e8 6c 54 24 00              	callq	0x14048d5c0 <.text+0x48c5c0>
140248154: f3 0f 6f 85 20 02 00 00     	movdqu	0x220(%rbp), %xmm0
14024815c: 66 0f 7f 85 d0 01 00 00     	movdqa	%xmm0, 0x1d0(%rbp)
140248164: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
14024816b: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
140248172: 48 8d 0d b2 8b 75 02        	leaq	0x2758bb2(%rip), %rcx   # 0x1429a0d2b
140248179: ba 1a 00 00 00              	movl	$0x1a, %edx
14024817e: e8 fd 52 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248183: 49 89 c5                    	movq	%rax, %r13
140248186: 48 89 d3                    	movq	%rdx, %rbx
140248189: 48 8d 0d b5 8b 75 02        	leaq	0x2758bb5(%rip), %rcx   # 0x1429a0d45
140248190: ba 1e 00 00 00              	movl	$0x1e, %edx
140248195: e8 e6 52 95 00              	callq	0x140b9d480 <.text+0xb9c480>
14024819a: 49 89 d1                    	movq	%rdx, %r9
14024819d: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
1402481a4: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
1402481ab: 49 89 c0                    	movq	%rax, %r8
1402481ae: e8 3d 5b 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
1402481b3: e8 b8 5b 43 00              	callq	0x14067dd70 <.text+0x67cd70>
1402481b8: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
1402481bf: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
1402481c6: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
1402481cd: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1402481d4: 4c 89 b5 08 01 00 00        	movq	%r14, 0x108(%rbp)
1402481db: 48 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %rax
1402481e2: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1402481e9: 4c 89 7c 24 28              	movq	%r15, 0x28(%rsp)
1402481ee: 66 0f 7e 44 24 20           	movd	%xmm0, 0x20(%rsp)
1402481f4: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
1402481fb: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140248202: 4c 89 ea                    	movq	%r13, %rdx
140248205: 49 89 d8                    	movq	%rbx, %r8
140248208: e8 e3 97 0e 00              	callq	0x1403319f0 <.text+0x3309f0>
14024820d: 84 c0                       	testb	%al, %al
14024820f: 74 59                       	je	0x14024826a <.text+0x24726a>
140248211: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140248218: c6 00 01                    	movb	$0x1, (%rax)
14024821b: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
140248222: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
140248227: 48 39 d1                    	cmpq	%rdx, %rcx
14024822a: 0f 83 43 1f 00 00           	jae	0x14024a173 <.text+0x249173>
140248230: 49 8b 04 24                 	movq	(%r12), %rax
140248234: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140248238: 48 c1 e1 05                 	shlq	$0x5, %rcx
14024823c: 0f 10 44 08 78              	movups	0x78(%rax,%rcx), %xmm0
140248241: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
140248248: 8b 85 60 02 00 00           	movl	0x260(%rbp), %eax
14024824e: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
140248255: 89 01                       	movl	%eax, (%rcx)
140248257: c6 87 ac 00 00 00 01        	movb	$0x1, 0xac(%rdi)
14024825e: 66 0f 6f 85 00 01 00 00     	movdqa	0x100(%rbp), %xmm0
140248266: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
14024826a: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
140248271: e8 2a fa 73 00              	callq	0x140987ca0 <.text+0x986ca0>
140248276: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14024827d: 48 89 c2                    	movq	%rax, %rdx
140248280: e8 3b 02 9b 00              	callq	0x140bf84c0 <.text+0xbf74c0>
140248285: 80 bf ac 00 00 00 01        	cmpb	$0x1, 0xac(%rdi)
14024828c: 75 25                       	jne	0x1402482b3 <.text+0x2472b3>
14024828e: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140248295: 48 33 87 ad 00 00 00        	xorq	0xad(%rdi), %rax
14024829c: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
1402482a3: 48 33 8f b5 00 00 00        	xorq	0xb5(%rdi), %rcx
1402482aa: 48 09 c1                    	orq	%rax, %rcx
1402482ad: 0f 84 f7 0f 00 00           	je	0x1402492aa <.text+0x2482aa>
1402482b3: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
1402482ba: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
1402482bf: 48 39 d1                    	cmpq	%rdx, %rcx
1402482c2: 0f 83 b9 1e 00 00           	jae	0x14024a181 <.text+0x249181>
1402482c8: 49 8b 04 24                 	movq	(%r12), %rax
1402482cc: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1402482d0: 48 c1 e1 05                 	shlq	$0x5, %rcx
1402482d4: 48 01 c1                    	addq	%rax, %rcx
1402482d7: 48 81 c1 98 00 00 00        	addq	$0x98, %rcx
1402482de: e8 8d 77 85 00              	callq	0x140a9fa70 <.text+0xa9ea70>
1402482e3: 84 c0                       	testb	%al, %al
1402482e5: 0f 84 2f 0f 00 00           	je	0x14024921a <.text+0x24821a>
1402482eb: 48 8d 0d cf 8a 75 02        	leaq	0x2758acf(%rip), %rcx   # 0x1429a0dc1
1402482f2: ba 1f 00 00 00              	movl	$0x1f, %edx
1402482f7: e8 84 51 95 00              	callq	0x140b9d480 <.text+0xb9c480>
1402482fc: 49 89 d1                    	movq	%rdx, %r9
1402482ff: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140248306: 48 8b 95 68 02 00 00        	movq	0x268(%rbp), %rdx
14024830d: 49 89 c0                    	movq	%rax, %r8
140248310: e8 eb 3b 44 00              	callq	0x14068bf00 <.text+0x68af00>
140248315: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14024831c: e8 0f 9a 43 00              	callq	0x140681d30 <.text+0x680d30>
140248321: 66 0f 6e 0d 53 d5 6d 02     	movd	0x26dd553(%rip), %xmm1  # 0x14292587c
140248329: 48 89 c1                    	movq	%rax, %rcx
14024832c: e8 1f e0 eb ff              	callq	0x140106350 <.text+0x105350>
140248331: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140248338: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
14024833f: f3 0f 6f 85 00 01 00 00     	movdqu	0x100(%rbp), %xmm0
140248347: 66 0f 7f 85 20 02 00 00     	movdqa	%xmm0, 0x220(%rbp)
14024834f: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140248356: 48 8d 95 20 02 00 00        	leaq	0x220(%rbp), %rdx
14024835d: 41 b8 ff 80 a0 ff           	movl	$0xffa080ff, %r8d       # imm = 0xFFA080FF
140248363: e8 38 61 43 00              	callq	0x14067e4a0 <.text+0x67d4a0>
140248368: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
14024836f: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140248376: e8 c5 9b 43 00              	callq	0x140681f40 <.text+0x680f40>
14024837b: e9 2a 0f 00 00              	jmp	0x1402492aa <.text+0x2482aa>
140248380: ba 01 00 00 00              	movl	$0x1, %edx
140248385: 41 f6 c0 01                 	testb	$0x1, %r8b
140248389: 74 0b                       	je	0x140248396 <.text+0x247396>
14024838b: 0f 2e 87 8c 00 00 00        	ucomiss	0x8c(%rdi), %xmm0
140248392: 75 02                       	jne	0x140248396 <.text+0x247396>
140248394: 7b 2e                       	jnp	0x1402483c4 <.text+0x2473c4>
140248396: 89 01                       	movl	%eax, (%rcx)
140248398: 89 97 88 00 00 00           	movl	%edx, 0x88(%rdi)
14024839e: f3 0f 11 87 8c 00 00 00     	movss	%xmm0, 0x8c(%rdi)
1402483a6: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1402483ad: 8b 41 1c                    	movl	0x1c(%rcx), %eax
1402483b0: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1402483b4: 89 01                       	movl	%eax, (%rcx)
1402483b6: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
1402483bd: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1402483c4: be 01 00 00 00              	movl	$0x1, %esi
1402483c9: ba 04 00 00 00              	movl	$0x4, %edx
1402483ce: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402483d4: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
1402483db: e8 40 b8 3c 00              	callq	0x140613c20 <.text+0x612c20>
1402483e0: ba 30 00 00 00              	movl	$0x30, %edx
1402483e5: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1402483eb: 48 89 d9                    	movq	%rbx, %rcx
1402483ee: e8 2d b8 3c 00              	callq	0x140613c20 <.text+0x612c20>
1402483f3: 48 8d 0d e6 8b 75 02        	leaq	0x2758be6(%rip), %rcx   # 0x1429a0fe0
1402483fa: ba 21 00 00 00              	movl	$0x21, %edx
1402483ff: e8 7c 50 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248404: 48 8d 9f ab 00 00 00        	leaq	0xab(%rdi), %rbx
14024840b: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14024840f: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140248413: 48 8d 05 e7 8b 75 02        	leaq	0x2758be7(%rip), %rax   # 0x1429a1001
14024841a: 31 c9                       	xorl	%ecx, %ecx
14024841c: 80 3b 00                    	cmpb	$0x0, (%rbx)
14024841f: 48 0f 45 f0                 	cmovneq	%rax, %rsi
140248423: b8 1d 00 00 00              	movl	$0x1d, %eax
140248428: 48 0f 44 c1                 	cmoveq	%rcx, %rax
14024842c: 48 89 75 10                 	movq	%rsi, 0x10(%rbp)
140248430: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140248434: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
140248438: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
14024843f: 4c 8d 2d 4a 1a ec ff        	leaq	-0x13e5b6(%rip), %r13   # 0x140109e90 <.text+0x108e90>
140248446: 4c 89 ad 28 02 00 00        	movq	%r13, 0x228(%rbp)
14024844d: 48 8d 45 20                 	leaq	0x20(%rbp), %rax
140248451: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140248458: 4c 89 ad 38 02 00 00        	movq	%r13, 0x238(%rbp)
14024845f: 48 8d 05 ba 8b 75 02        	leaq	0x2758bba(%rip), %rax   # 0x1429a1020
140248466: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14024846d: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140248478: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
14024847f: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140248486: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
140248491: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
14024849c: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
1402484a0: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1402484a7: e8 e4 1a b3 00              	callq	0x140d79f90 <.text+0xd78f90>
1402484ac: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1402484b0: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
1402484b7: 48 8b 45 00                 	movq	(%rbp), %rax
1402484bb: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
1402484c2: 4c 8b 65 08                 	movq	0x8(%rbp), %r12
1402484c6: 48 8d 05 73 8b 75 02        	leaq	0x2758b73(%rip), %rax   # 0x1429a1040
1402484cd: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1402484d4: 48 c7 85 98 00 00 00 04 00 00 00    	movq	$0x4, 0x98(%rbp)
1402484df: 48 8d 0d 5e 8b 75 02        	leaq	0x2758b5e(%rip), %rcx   # 0x1429a1044
1402484e6: ba 1d 00 00 00              	movl	$0x1d, %edx
1402484eb: e8 90 4f 95 00              	callq	0x140b9d480 <.text+0xb9c480>
1402484f0: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1402484f7: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
1402484fe: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
140248505: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
14024850c: 4c 89 ad 28 02 00 00        	movq	%r13, 0x228(%rbp)
140248513: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14024851a: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140248521: 4c 89 ad 38 02 00 00        	movq	%r13, 0x238(%rbp)
140248528: 48 8d 05 f1 8a 75 02        	leaq	0x2758af1(%rip), %rax   # 0x1429a1020
14024852f: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140248536: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140248541: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
140248548: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14024854f: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
14024855a: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140248565: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140248569: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140248570: e8 1b 1a b3 00              	callq	0x140d79f90 <.text+0xd78f90>
140248575: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140248579: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
140248580: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
140248584: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
14024858b: 4c 8b 7d f0                 	movq	-0x10(%rbp), %r15
14024858f: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140248596: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
14024859d: 4c 8b 85 08 02 00 00        	movq	0x208(%rbp), %r8
1402485a4: 4d 89 e1                    	movq	%r12, %r9
1402485a7: e8 44 57 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
1402485ac: e8 bf 57 43 00              	callq	0x14067dd70 <.text+0x67cd70>
1402485b1: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
1402485b5: 49 8b 76 10                 	movq	0x10(%r14), %rsi
1402485b9: 89 85 70 02 00 00           	movl	%eax, 0x270(%rbp)
1402485bf: 89 06                       	movl	%eax, (%rsi)
1402485c1: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1402485c6: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1402485cc: 4c 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %r9
1402485d3: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1402485da: 48 8b 95 60 02 00 00        	movq	0x260(%rbp), %rdx
1402485e1: 4d 89 f8                    	movq	%r15, %r8
1402485e4: e8 07 72 0e 00              	callq	0x14032f7f0 <.text+0x32e7f0>
1402485e9: 41 89 c6                    	movl	%eax, %r14d
1402485ec: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1402485f3: 44 0a 30                    	orb	(%rax), %r14b
1402485f6: 44 88 30                    	movb	%r14b, (%rax)
1402485f9: 48 8b 95 18 02 00 00        	movq	0x218(%rbp), %rdx
140248600: 48 85 d2                    	testq	%rdx, %rdx
140248603: 74 12                       	je	0x140248617 <.text+0x247617>
140248605: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14024860b: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140248612: e8 09 b6 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248617: 80 3b 01                    	cmpb	$0x1, (%rbx)
14024861a: 0f 85 33 01 00 00           	jne	0x140248753 <.text+0x247753>
140248620: 48 8d 0d 3a 8a 75 02        	leaq	0x2758a3a(%rip), %rcx   # 0x1429a1061
140248627: ba 23 00 00 00              	movl	$0x23, %edx
14024862c: e8 4f 4e 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248631: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140248638: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
14024863f: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
140248646: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
14024864d: 4c 89 ad 28 02 00 00        	movq	%r13, 0x228(%rbp)
140248654: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14024865b: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140248662: 4c 89 ad 38 02 00 00        	movq	%r13, 0x238(%rbp)
140248669: 48 8d 05 b0 89 75 02        	leaq	0x27589b0(%rip), %rax   # 0x1429a1020
140248670: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140248677: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140248682: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
140248689: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140248690: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
14024869b: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
1402486a6: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
1402486aa: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1402486b1: e8 da 18 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
1402486b6: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
1402486ba: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1402486c1: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1402486c5: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1402486cc: 4c 8b 7d d8                 	movq	-0x28(%rbp), %r15
1402486d0: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1402486d7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402486dd: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
1402486e4: 45 31 c9                    	xorl	%r9d, %r9d
1402486e7: e8 04 56 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
1402486ec: e8 7f 56 43 00              	callq	0x14067dd70 <.text+0x67cd70>
1402486f1: 48 8d 87 c3 00 00 00        	leaq	0xc3(%rdi), %rax
1402486f8: 8b 8d 70 02 00 00           	movl	0x270(%rbp), %ecx
1402486fe: 89 0e                       	movl	%ecx, (%rsi)
140248700: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140248705: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
14024870b: 4c 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %r9
140248712: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140248719: 48 8b 95 60 02 00 00        	movq	0x260(%rbp), %rdx
140248720: 4d 89 f8                    	movq	%r15, %r8
140248723: e8 58 b4 0e 00              	callq	0x140333b80 <.text+0x332b80>
140248728: 41 08 c6                    	orb	%al, %r14b
14024872b: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140248732: 44 88 30                    	movb	%r14b, (%rax)
140248735: 48 8b 95 18 02 00 00        	movq	0x218(%rbp), %rdx
14024873c: 48 85 d2                    	testq	%rdx, %rdx
14024873f: 74 12                       	je	0x140248753 <.text+0x247753>
140248741: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248747: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
14024874e: e8 cd b4 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248753: 48 8d 0d 2a 89 75 02        	leaq	0x275892a(%rip), %rcx   # 0x1429a1084
14024875a: ba 17 00 00 00              	movl	$0x17, %edx
14024875f: e8 1c 4d 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248764: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
14024876b: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
140248772: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
140248779: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140248780: 4c 89 ad 28 02 00 00        	movq	%r13, 0x228(%rbp)
140248787: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
14024878e: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140248795: 4c 89 ad 38 02 00 00        	movq	%r13, 0x238(%rbp)
14024879c: 48 8d 05 7d 88 75 02        	leaq	0x275887d(%rip), %rax   # 0x1429a1020
1402487a3: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1402487aa: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
1402487b5: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
1402487bc: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1402487c3: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
1402487ce: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
1402487d9: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1402487dd: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1402487e4: e8 a7 17 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
1402487e9: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
1402487ed: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
1402487f4: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
1402487f8: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
1402487ff: 4c 8b 7d c0                 	movq	-0x40(%rbp), %r15
140248803: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14024880a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248810: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140248817: 45 31 c9                    	xorl	%r9d, %r9d
14024881a: e8 d1 54 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
14024881f: e8 4c 55 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140248824: 48 8d 87 a9 00 00 00        	leaq	0xa9(%rdi), %rax
14024882b: 8b 8d 70 02 00 00           	movl	0x270(%rbp), %ecx
140248831: 89 0e                       	movl	%ecx, (%rsi)
140248833: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140248838: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
14024883e: 4c 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %r9
140248845: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
14024884c: 48 8b 95 60 02 00 00        	movq	0x260(%rbp), %rdx
140248853: 4d 89 f8                    	movq	%r15, %r8
140248856: e8 e5 80 0e 00              	callq	0x140330940 <.text+0x32f940>
14024885b: 41 08 c6                    	orb	%al, %r14b
14024885e: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140248865: 44 88 30                    	movb	%r14b, (%rax)
140248868: 48 8b 95 18 02 00 00        	movq	0x218(%rbp), %rdx
14024886f: 48 85 d2                    	testq	%rdx, %rdx
140248872: 74 12                       	je	0x140248886 <.text+0x247886>
140248874: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14024887a: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140248881: e8 9a b3 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248886: 80 bf 88 00 00 00 00        	cmpb	$0x0, 0x88(%rdi)
14024888d: 75 0a                       	jne	0x140248899 <.text+0x247899>
14024888f: f3 0f 10 05 6d d0 6d 02     	movss	0x26dd06d(%rip), %xmm0  # 0x142925904
140248897: eb 08                       	jmp	0x1402488a1 <.text+0x2478a1>
140248899: f3 0f 10 87 8c 00 00 00     	movss	0x8c(%rdi), %xmm0
1402488a1: f3 0f 11 85 fc 00 00 00     	movss	%xmm0, 0xfc(%rbp)
1402488a9: 48 8d 0d eb 87 75 02        	leaq	0x27587eb(%rip), %rcx   # 0x1429a109b
1402488b0: ba 23 00 00 00              	movl	$0x23, %edx
1402488b5: e8 c6 4b 95 00              	callq	0x140b9d480 <.text+0xb9c480>
1402488ba: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1402488c1: 48 89 95 d8 01 00 00        	movq	%rdx, 0x1d8(%rbp)
1402488c8: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
1402488cf: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
1402488d6: 4c 89 ad 28 02 00 00        	movq	%r13, 0x228(%rbp)
1402488dd: 48 8d 45 48                 	leaq	0x48(%rbp), %rax
1402488e1: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1402488e8: 48 8d 1d e1 31 df ff        	leaq	-0x20ce1f(%rip), %rbx   # 0x14003bad0 <.text+0x3aad0>
1402488ef: 48 89 9d 38 02 00 00        	movq	%rbx, 0x238(%rbp)
1402488f6: 48 8d 05 c3 87 75 02        	leaq	0x27587c3(%rip), %rax   # 0x1429a10c0
1402488fd: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140248904: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
14024890f: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
140248916: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14024891d: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
140248928: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140248933: 4c 8d a5 a8 00 00 00        	leaq	0xa8(%rbp), %r12
14024893a: 4c 8d bd 00 01 00 00        	leaq	0x100(%rbp), %r15
140248941: 4c 89 e1                    	movq	%r12, %rcx
140248944: 4c 89 fa                    	movq	%r15, %rdx
140248947: e8 44 16 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
14024894c: 0f 10 85 a8 00 00 00        	movups	0xa8(%rbp), %xmm0
140248953: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14024895a: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140248961: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140248968: 80 bf 88 00 00 00 00        	cmpb	$0x0, 0x88(%rdi)
14024896f: 75 0a                       	jne	0x14024897b <.text+0x24797b>
140248971: f3 0f 10 05 8b cf 6d 02     	movss	0x26dcf8b(%rip), %xmm0  # 0x142925904
140248979: eb 08                       	jmp	0x140248983 <.text+0x247983>
14024897b: f3 0f 10 87 8c 00 00 00     	movss	0x8c(%rdi), %xmm0
140248983: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
14024898a: 48 8d 48 18                 	leaq	0x18(%rax), %rcx
14024898e: 48 89 8d a0 00 00 00        	movq	%rcx, 0xa0(%rbp)
140248995: 0f 2e 40 18                 	ucomiss	0x18(%rax), %xmm0
140248999: 76 29                       	jbe	0x1402489c4 <.text+0x2479c4>
14024899b: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
1402489a2: 48 8b 00                    	movq	(%rax), %rax
1402489a5: 48 8d 0d 98 8d 75 02        	leaq	0x2758d98(%rip), %rcx   # 0x1429a1744
1402489ac: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1402489b0: 48 01 c8                    	addq	%rcx, %rax
1402489b3: ff e0                       	jmpq	*%rax
1402489b5: c7 85 cc 01 00 00 00 00 00 00       	movl	$0x0, 0x1cc(%rbp)
1402489bf: e9 ae 01 00 00              	jmp	0x140248b72 <.text+0x247b72>
1402489c4: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
1402489cb: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
1402489d2: 0f 10 85 a8 00 00 00        	movups	0xa8(%rbp), %xmm0
1402489d9: 0f 29 85 d0 01 00 00        	movaps	%xmm0, 0x1d0(%rbp)
1402489e0: c7 85 cc 01 00 00 00 00 00 00       	movl	$0x0, 0x1cc(%rbp)
1402489ea: e9 9b 01 00 00              	jmp	0x140248b8a <.text+0x247b8a>
1402489ef: b8 16 00 00 00              	movl	$0x16, %eax
1402489f4: 48 8d 0d e5 86 75 02        	leaq	0x27586e5(%rip), %rcx   # 0x1429a10e0
1402489fb: eb 0c                       	jmp	0x140248a09 <.text+0x247a09>
1402489fd: b8 15 00 00 00              	movl	$0x15, %eax
140248a02: 48 8d 0d ed 86 75 02        	leaq	0x27586ed(%rip), %rcx   # 0x1429a10f6
140248a09: 48 89 8d b0 01 00 00        	movq	%rcx, 0x1b0(%rbp)
140248a10: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
140248a17: 48 8d 85 c0 00 00 00        	leaq	0xc0(%rbp), %rax
140248a1e: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140248a25: 48 89 9d d8 01 00 00        	movq	%rbx, 0x1d8(%rbp)
140248a2c: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
140248a33: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
140248a3a: 48 8d 05 cf 3e b4 00        	leaq	0xb43ecf(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
140248a41: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
140248a48: 48 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %rax
140248a4f: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140248a56: 4c 89 ad f8 01 00 00        	movq	%r13, 0x1f8(%rbp)
140248a5d: 48 c7 85 00 01 00 00 02 00 00 00    	movq	$0x2, 0x100(%rbp)
140248a68: 48 c7 85 10 01 00 00 02 00 00 00    	movq	$0x2, 0x110(%rbp)
140248a73: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140248a7e: 48 c7 85 28 01 00 00 20 00 00 00    	movq	$0x20, 0x128(%rbp)
140248a89: c6 85 30 01 00 00 03        	movb	$0x3, 0x130(%rbp)
140248a90: 48 c7 85 38 01 00 00 00 00 00 00    	movq	$0x0, 0x138(%rbp)
140248a9b: 48 c7 85 40 01 00 00 01 00 00 00    	movq	$0x1, 0x140(%rbp)
140248aa6: 48 c7 85 48 01 00 00 00 00 00 00    	movq	$0x0, 0x148(%rbp)
140248ab1: 48 c7 85 50 01 00 00 01 00 00 00    	movq	$0x1, 0x150(%rbp)
140248abc: 48 c7 85 58 01 00 00 01 00 00 00    	movq	$0x1, 0x158(%rbp)
140248ac7: 48 c7 85 60 01 00 00 20 00 00 00    	movq	$0x20, 0x160(%rbp)
140248ad2: c6 85 68 01 00 00 03        	movb	$0x3, 0x168(%rbp)
140248ad9: 48 c7 85 70 01 00 00 02 00 00 00    	movq	$0x2, 0x170(%rbp)
140248ae4: 48 c7 85 80 01 00 00 02 00 00 00    	movq	$0x2, 0x180(%rbp)
140248aef: 48 c7 85 90 01 00 00 02 00 00 00    	movq	$0x2, 0x190(%rbp)
140248afa: 48 c7 85 98 01 00 00 20 00 00 00    	movq	$0x20, 0x198(%rbp)
140248b05: c6 85 a0 01 00 00 03        	movb	$0x3, 0x1a0(%rbp)
140248b0c: 48 8d 05 0d 86 75 02        	leaq	0x275860d(%rip), %rax   # 0x1429a1120
140248b13: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140248b1a: 48 c7 85 28 02 00 00 03 00 00 00    	movq	$0x3, 0x228(%rbp)
140248b25: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
140248b2c: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140248b33: 48 c7 85 38 02 00 00 03 00 00 00    	movq	$0x3, 0x238(%rbp)
140248b3e: 4c 89 bd 40 02 00 00        	movq	%r15, 0x240(%rbp)
140248b45: 48 c7 85 48 02 00 00 03 00 00 00    	movq	$0x3, 0x248(%rbp)
140248b50: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
140248b57: 4c 8d 65 30                 	leaq	0x30(%rbp), %r12
140248b5b: 48 8d 95 20 02 00 00        	leaq	0x220(%rbp), %rdx
140248b62: 4c 89 e1                    	movq	%r12, %rcx
140248b65: e8 26 14 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
140248b6a: b0 01                       	movb	$0x1, %al
140248b6c: 89 85 cc 01 00 00           	movl	%eax, 0x1cc(%rbp)
140248b72: 49 8b 44 24 10              	movq	0x10(%r12), %rax
140248b77: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
140248b7e: 41 0f 10 04 24              	movups	(%r12), %xmm0
140248b83: 0f 29 85 d0 01 00 00        	movaps	%xmm0, 0x1d0(%rbp)
140248b8a: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
140248b91: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140248b98: 48 89 9d 28 02 00 00        	movq	%rbx, 0x228(%rbp)
140248b9f: 48 8d 05 fa 85 75 02        	leaq	0x27585fa(%rip), %rax   # 0x1429a11a0
140248ba6: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140248bad: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140248bb8: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
140248bbf: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140248bc6: 48 c7 85 18 01 00 00 01 00 00 00    	movq	$0x1, 0x118(%rbp)
140248bd1: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140248bdc: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
140248be0: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140248be7: e8 a4 13 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
140248bec: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140248bf0: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
140248bf7: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140248bfb: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
140248c02: 4c 8b 7d 70                 	movq	0x70(%rbp), %r15
140248c06: 48 8d 0d b3 85 75 02        	leaq	0x27585b3(%rip), %rcx   # 0x1429a11c0
140248c0d: ba 1f 00 00 00              	movl	$0x1f, %edx
140248c12: e8 69 48 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248c17: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140248c1e: 48 89 95 88 00 00 00        	movq	%rdx, 0x88(%rbp)
140248c25: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
140248c2c: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140248c33: 4c 89 ad 28 02 00 00        	movq	%r13, 0x228(%rbp)
140248c3a: 48 8d 85 80 00 00 00        	leaq	0x80(%rbp), %rax
140248c41: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140248c48: 4c 89 ad 38 02 00 00        	movq	%r13, 0x238(%rbp)
140248c4f: 48 8d 05 ca 83 75 02        	leaq	0x27583ca(%rip), %rax   # 0x1429a1020
140248c56: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140248c5d: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140248c68: 48 8d 85 20 02 00 00        	leaq	0x220(%rbp), %rax
140248c6f: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140248c76: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
140248c81: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140248c8c: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
140248c93: 48 8d 9d 00 01 00 00        	leaq	0x100(%rbp), %rbx
140248c9a: 48 89 da                    	movq	%rbx, %rdx
140248c9d: e8 ee 12 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
140248ca2: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
140248ca9: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
140248cb0: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
140248cb7: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
140248cbe: 4c 8b a5 c0 01 00 00        	movq	0x1c0(%rbp), %r12
140248cc5: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140248ccc: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140248cd3: 4c 8b 85 60 02 00 00        	movq	0x260(%rbp), %r8
140248cda: 4d 89 f9                    	movq	%r15, %r9
140248cdd: e8 0e 50 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140248ce2: e8 89 50 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140248ce7: 0f 28 ce                    	movaps	%xmm6, %xmm1
140248cea: f3 0f 5d 0d 92 cc 6d 02     	minss	0x26dcc92(%rip), %xmm1  # 0x142925984
140248cf2: 48 8d 85 fc 00 00 00        	leaq	0xfc(%rbp), %rax
140248cf9: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140248d00: f3 0f 11 8d 0c 01 00 00     	movss	%xmm1, 0x10c(%rbp)
140248d08: f3 0f 11 bd 10 01 00 00     	movss	%xmm7, 0x110(%rbp)
140248d10: c6 85 14 01 00 00 00        	movb	$0x0, 0x114(%rbp)
140248d17: c7 85 08 01 00 00 cd cc cc 3d       	movl	$0x3dcccccd, 0x108(%rbp) # imm = 0x3DCCCCCD
140248d21: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
140248d26: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140248d2c: 48 8d 9d 20 02 00 00        	leaq	0x220(%rbp), %rbx
140248d33: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140248d3a: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
140248d41: 4d 89 e0                    	movq	%r12, %r8
140248d44: 49 89 d9                    	movq	%rbx, %r9
140248d47: e8 c4 92 0e 00              	callq	0x140332010 <.text+0x331010>
140248d4c: 84 c0                       	testb	%al, %al
140248d4e: 0f 84 a8 00 00 00           	je	0x140248dfc <.text+0x247dfc>
140248d54: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
140248d5b: 48 85 d2                    	testq	%rdx, %rdx
140248d5e: 74 12                       	je	0x140248d72 <.text+0x247d72>
140248d60: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248d66: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
140248d6d: e8 ae ae 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248d72: f3 0f 10 85 fc 00 00 00     	movss	0xfc(%rbp), %xmm0
140248d7a: 8b 85 70 02 00 00           	movl	0x270(%rbp), %eax
140248d80: 89 06                       	movl	%eax, (%rsi)
140248d82: c7 87 88 00 00 00 01 00 00 00       	movl	$0x1, 0x88(%rdi)
140248d8c: f3 0f 11 87 8c 00 00 00     	movss	%xmm0, 0x8c(%rdi)
140248d94: f3 0f 10 85 fc 00 00 00     	movss	0xfc(%rbp), %xmm0
140248d9c: 0f 28 c8                    	movaps	%xmm0, %xmm1
140248d9f: f3 0f c2 c8 03              	cmpunordss	%xmm0, %xmm1
140248da4: 0f 28 d1                    	movaps	%xmm1, %xmm2
140248da7: 0f 54 d6                    	andps	%xmm6, %xmm2
140248daa: f3 0f 5d f0                 	minss	%xmm0, %xmm6
140248dae: 0f 55 ce                    	andnps	%xmm6, %xmm1
140248db1: 0f 56 ca                    	orps	%xmm2, %xmm1
140248db4: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
140248dbb: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140248dbe: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140248dc2: 89 01                       	movl	%eax, (%rcx)
140248dc4: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
140248dcb: f3 0f 11 08                 	movss	%xmm1, (%rax)
140248dcf: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
140248dd6: 8b 41 1c                    	movl	0x1c(%rcx), %eax
140248dd9: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140248ddd: 89 01                       	movl	%eax, (%rcx)
140248ddf: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
140248de6: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
140248ded: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140248df4: c6 00 01                    	movb	$0x1, (%rax)
140248df7: 41 b6 01                    	movb	$0x1, %r14b
140248dfa: eb 1e                       	jmp	0x140248e1a <.text+0x247e1a>
140248dfc: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
140248e03: 48 85 d2                    	testq	%rdx, %rdx
140248e06: 74 12                       	je	0x140248e1a <.text+0x247e1a>
140248e08: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248e0e: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
140248e15: e8 06 ae 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248e1a: 48 8b 95 18 02 00 00        	movq	0x218(%rbp), %rdx
140248e21: 48 85 d2                    	testq	%rdx, %rdx
140248e24: 74 12                       	je	0x140248e38 <.text+0x247e38>
140248e26: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248e2c: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140248e33: e8 e8 ad 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248e38: 48 8b 95 d0 01 00 00        	movq	0x1d0(%rbp), %rdx
140248e3f: 48 85 d2                    	testq	%rdx, %rdx
140248e42: 74 12                       	je	0x140248e56 <.text+0x247e56>
140248e44: 48 8b 8d d8 01 00 00        	movq	0x1d8(%rbp), %rcx
140248e4b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248e51: e8 ca ad 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248e56: 80 bd cc 01 00 00 00        	cmpb	$0x0, 0x1cc(%rbp)
140248e5d: 74 1e                       	je	0x140248e7d <.text+0x247e7d>
140248e5f: 48 8b 95 c0 00 00 00        	movq	0xc0(%rbp), %rdx
140248e66: 48 85 d2                    	testq	%rdx, %rdx
140248e69: 74 12                       	je	0x140248e7d <.text+0x247e7d>
140248e6b: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
140248e72: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248e78: e8 a3 ad 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248e7d: 66 0f 6e 15 f7 c9 6d 02     	movd	0x26dc9f7(%rip), %xmm2  # 0x14292587c
140248e85: 0f 57 c9                    	xorps	%xmm1, %xmm1
140248e88: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140248e8f: e8 fc 3b 44 00              	callq	0x14068ca90 <.text+0x68ba90>
140248e94: 48 8d 0d e9 81 75 02        	leaq	0x27581e9(%rip), %rcx   # 0x1429a1084
140248e9b: ba 17 00 00 00              	movl	$0x17, %edx
140248ea0: e8 db 45 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248ea5: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140248eac: 48 89 95 88 00 00 00        	movq	%rdx, 0x88(%rbp)
140248eb3: 48 8d 0d 25 83 75 02        	leaq	0x2758325(%rip), %rcx   # 0x1429a11df
140248eba: ba 09 00 00 00              	movl	$0x9, %edx
140248ebf: e8 bc 45 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248ec4: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
140248ecb: 48 89 95 c8 00 00 00        	movq	%rdx, 0xc8(%rbp)
140248ed2: 48 8d 85 80 00 00 00        	leaq	0x80(%rbp), %rax
140248ed9: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140248ee0: 4c 89 ad 28 02 00 00        	movq	%r13, 0x228(%rbp)
140248ee7: 48 8d 85 c0 00 00 00        	leaq	0xc0(%rbp), %rax
140248eee: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140248ef5: 4c 89 ad 38 02 00 00        	movq	%r13, 0x238(%rbp)
140248efc: 48 8d 05 ed 82 75 02        	leaq	0x27582ed(%rip), %rax   # 0x1429a11f0
140248f03: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140248f0a: 48 c7 85 08 01 00 00 03 00 00 00    	movq	$0x3, 0x108(%rbp)
140248f15: 48 89 9d 10 01 00 00        	movq	%rbx, 0x110(%rbp)
140248f1c: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
140248f27: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
140248f32: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140248f39: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
140248f40: e8 4b 10 b3 00              	callq	0x140d79f90 <.text+0xd78f90>
140248f45: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
140248f4c: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
140248f53: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
140248f5a: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
140248f61: 4c 8b bd e0 01 00 00        	movq	0x1e0(%rbp), %r15
140248f68: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140248f6f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248f75: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140248f7c: 45 31 c9                    	xorl	%r9d, %r9d
140248f7f: e8 6c 4d 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140248f84: e8 e7 4d 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140248f89: 8b 85 70 02 00 00           	movl	0x270(%rbp), %eax
140248f8f: 89 06                       	movl	%eax, (%rsi)
140248f91: 48 8d 87 aa 00 00 00        	leaq	0xaa(%rdi), %rax
140248f98: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140248f9d: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140248fa3: 48 8d 9d 00 01 00 00        	leaq	0x100(%rbp), %rbx
140248faa: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140248fb1: 48 8b 95 60 02 00 00        	movq	0x260(%rbp), %rdx
140248fb8: 4d 89 f8                    	movq	%r15, %r8
140248fbb: 49 89 d9                    	movq	%rbx, %r9
140248fbe: e8 7d 79 0e 00              	callq	0x140330940 <.text+0x32f940>
140248fc3: 41 08 c6                    	orb	%al, %r14b
140248fc6: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140248fcd: 44 88 30                    	movb	%r14b, (%rax)
140248fd0: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
140248fd7: 48 85 d2                    	testq	%rdx, %rdx
140248fda: 74 12                       	je	0x140248fee <.text+0x247fee>
140248fdc: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140248fe2: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140248fe9: e8 32 ac 3c 00              	callq	0x140613c20 <.text+0x612c20>
140248fee: 48 8d 0d 2b 82 75 02        	leaq	0x275822b(%rip), %rcx   # 0x1429a1220
140248ff5: ba 1e 00 00 00              	movl	$0x1e, %edx
140248ffa: e8 81 44 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140248fff: 49 89 c7                    	movq	%rax, %r15
140249002: 49 89 d4                    	movq	%rdx, %r12
140249005: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
14024900c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140249012: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140249019: 45 31 c9                    	xorl	%r9d, %r9d
14024901c: e8 cf 4c 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140249021: e8 4a 4d 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140249026: 8b 85 70 02 00 00           	movl	0x270(%rbp), %eax
14024902c: 89 06                       	movl	%eax, (%rsi)
14024902e: 48 8d 87 a0 00 00 00        	leaq	0xa0(%rdi), %rax
140249035: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14024903c: c7 85 10 01 00 00 00 00 00 40       	movl	$0x40000000, 0x110(%rbp) # imm = 0x40000000
140249046: c6 85 14 01 00 00 00        	movb	$0x0, 0x114(%rbp)
14024904d: f2 0f 10 0d 8b 77 75 02     	movsd	0x275778b(%rip), %xmm1  # 0x1429a07e0
140249055: f2 0f 11 8d 08 01 00 00     	movsd	%xmm1, 0x108(%rbp)
14024905d: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
140249062: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140249068: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
14024906f: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140249076: 4c 89 fa                    	movq	%r15, %rdx
140249079: 4d 89 e0                    	movq	%r12, %r8
14024907c: e8 8f 8f 0e 00              	callq	0x140332010 <.text+0x331010>
140249081: 41 08 c6                    	orb	%al, %r14b
140249084: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14024908b: 44 88 30                    	movb	%r14b, (%rax)
14024908e: 48 8d 0d a9 81 75 02        	leaq	0x27581a9(%rip), %rcx   # 0x1429a123e
140249095: ba 19 00 00 00              	movl	$0x19, %edx
14024909a: e8 e1 43 95 00              	callq	0x140b9d480 <.text+0xb9c480>
14024909f: 49 89 c7                    	movq	%rax, %r15
1402490a2: 49 89 d4                    	movq	%rdx, %r12
1402490a5: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
1402490ac: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402490b2: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
1402490b9: 45 31 c9                    	xorl	%r9d, %r9d
1402490bc: e8 2f 4c 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
1402490c1: e8 aa 4c 43 00              	callq	0x14067dd70 <.text+0x67cd70>
1402490c6: 8b 85 70 02 00 00           	movl	0x270(%rbp), %eax
1402490cc: 89 06                       	movl	%eax, (%rsi)
1402490ce: 48 8d 87 a4 00 00 00        	leaq	0xa4(%rdi), %rax
1402490d5: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1402490dc: c7 85 10 01 00 00 00 00 00 3f       	movl	$0x3f000000, 0x110(%rbp) # imm = 0x3F000000
1402490e6: c6 85 14 01 00 00 00        	movb	$0x0, 0x114(%rbp)
1402490ed: f2 0f 10 0d fb 76 75 02     	movsd	0x27576fb(%rip), %xmm1  # 0x1429a07f0
1402490f5: f2 0f 11 8d 08 01 00 00     	movsd	%xmm1, 0x108(%rbp)
1402490fd: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
140249102: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140249108: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
14024910f: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140249116: 4c 89 fa                    	movq	%r15, %rdx
140249119: 4d 89 e0                    	movq	%r12, %r8
14024911c: e8 ef 8e 0e 00              	callq	0x140332010 <.text+0x331010>
140249121: 41 08 c6                    	orb	%al, %r14b
140249124: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14024912b: 44 88 30                    	movb	%r14b, (%rax)
14024912e: 48 8d 0d 22 81 75 02        	leaq	0x2758122(%rip), %rcx   # 0x1429a1257
140249135: ba 1b 00 00 00              	movl	$0x1b, %edx
14024913a: e8 41 43 95 00              	callq	0x140b9d480 <.text+0xb9c480>
14024913f: 49 89 c7                    	movq	%rax, %r15
140249142: 49 89 d4                    	movq	%rdx, %r12
140249145: 48 8d 0d 26 81 75 02        	leaq	0x2758126(%rip), %rcx   # 0x1429a1272
14024914c: ba 1f 00 00 00              	movl	$0x1f, %edx
140249151: e8 2a 43 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249156: 49 89 d1                    	movq	%rdx, %r9
140249159: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140249160: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140249167: 49 89 c0                    	movq	%rax, %r8
14024916a: e8 81 4b 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
14024916f: e8 fc 4b 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140249174: 8b 85 70 02 00 00           	movl	0x270(%rbp), %eax
14024917a: 89 06                       	movl	%eax, (%rsi)
14024917c: 48 81 c7 9c 00 00 00        	addq	$0x9c, %rdi
140249183: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
14024918a: c7 85 10 01 00 00 00 00 80 3f       	movl	$0x3f800000, 0x110(%rbp) # imm = 0x3F800000
140249194: c6 85 14 01 00 00 00        	movb	$0x0, 0x114(%rbp)
14024919b: f3 0f 10 0d e1 c7 6d 02     	movss	0x26dc7e1(%rip), %xmm1  # 0x142925984
1402491a3: 0f 13 8d 08 01 00 00        	movlps	%xmm1, 0x108(%rbp)
1402491aa: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1402491af: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1402491b5: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
1402491bc: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
1402491c3: 4c 89 fa                    	movq	%r15, %rdx
1402491c6: 4d 89 e0                    	movq	%r12, %r8
1402491c9: e8 42 8e 0e 00              	callq	0x140332010 <.text+0x331010>
1402491ce: 41 08 c6                    	orb	%al, %r14b
1402491d1: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1402491d8: 44 88 30                    	movb	%r14b, (%rax)
1402491db: 48 8b 95 e8 00 00 00        	movq	0xe8(%rbp), %rdx
1402491e2: 48 85 d2                    	testq	%rdx, %rdx
1402491e5: 74 12                       	je	0x1402491f9 <.text+0x2481f9>
1402491e7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402491ed: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1402491f4: e8 27 aa 3c 00              	callq	0x140613c20 <.text+0x612c20>
1402491f9: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
1402491fd: 48 85 d2                    	testq	%rdx, %rdx
140249200: 0f 84 9c 0d 00 00           	je	0x140249fa2 <.text+0x248fa2>
140249206: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14024920a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140249210: e8 0b aa 3c 00              	callq	0x140613c20 <.text+0x612c20>
140249215: e9 88 0d 00 00              	jmp	0x140249fa2 <.text+0x248fa2>
14024921a: 48 8d 0d 77 7b 75 02        	leaq	0x2757b77(%rip), %rcx   # 0x1429a0d98
140249221: ba 29 00 00 00              	movl	$0x29, %edx
140249226: e8 55 42 95 00              	callq	0x140b9d480 <.text+0xb9c480>
14024922b: 49 89 d1                    	movq	%rdx, %r9
14024922e: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140249235: 48 8b 95 68 02 00 00        	movq	0x268(%rbp), %rdx
14024923c: 49 89 c0                    	movq	%rax, %r8
14024923f: e8 bc 2c 44 00              	callq	0x14068bf00 <.text+0x68af00>
140249244: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14024924b: e8 e0 8a 43 00              	callq	0x140681d30 <.text+0x680d30>
140249250: 66 0f 6e 0d 24 c6 6d 02     	movd	0x26dc624(%rip), %xmm1  # 0x14292587c
140249258: 48 89 c1                    	movq	%rax, %rcx
14024925b: e8 f0 d0 eb ff              	callq	0x140106350 <.text+0x105350>
140249260: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140249267: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
14024926e: f3 0f 6f 85 00 01 00 00     	movdqu	0x100(%rbp), %xmm0
140249276: 66 0f 7f 85 20 02 00 00     	movdqa	%xmm0, 0x220(%rbp)
14024927e: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
140249285: 48 8d 95 20 02 00 00        	leaq	0x220(%rbp), %rdx
14024928c: 41 b8 ff 80 a0 ff           	movl	$0xffa080ff, %r8d       # imm = 0xFFA080FF
140249292: e8 09 52 43 00              	callq	0x14067e4a0 <.text+0x67d4a0>
140249297: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
14024929e: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1402492a5: e8 96 8c 43 00              	callq	0x140681f40 <.text+0x680f40>
1402492aa: 4d 85 f6                    	testq	%r14, %r14
1402492ad: 48 8b 9d 10 02 00 00        	movq	0x210(%rbp), %rbx
1402492b4: 74 2c                       	je	0x1402492e2 <.text+0x2482e2>
1402492b6: 48 8d 73 08                 	leaq	0x8(%rbx), %rsi
1402492ba: eb 0d                       	jmp	0x1402492c9 <.text+0x2482c9>
1402492bc: 0f 1f 40 00                 	nopl	(%rax)
1402492c0: 48 83 c6 18                 	addq	$0x18, %rsi
1402492c4: 49 ff ce                    	decq	%r14
1402492c7: 74 19                       	je	0x1402492e2 <.text+0x2482e2>
1402492c9: 48 8b 56 f8                 	movq	-0x8(%rsi), %rdx
1402492cd: 48 85 d2                    	testq	%rdx, %rdx
1402492d0: 74 ee                       	je	0x1402492c0 <.text+0x2482c0>
1402492d2: 48 8b 0e                    	movq	(%rsi), %rcx
1402492d5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402492db: e8 40 a9 3c 00              	callq	0x140613c20 <.text+0x612c20>
1402492e0: eb de                       	jmp	0x1402492c0 <.text+0x2482c0>
1402492e2: 48 8b 85 d0 01 00 00        	movq	0x1d0(%rbp), %rax
1402492e9: 48 85 c0                    	testq	%rax, %rax
1402492ec: 74 16                       	je	0x140249304 <.text+0x248304>
1402492ee: 48 c1 e0 03                 	shlq	$0x3, %rax
1402492f2: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1402492f6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1402492fc: 48 89 d9                    	movq	%rbx, %rcx
1402492ff: e8 1c a9 3c 00              	callq	0x140613c20 <.text+0x612c20>
140249304: 8b b5 60 02 00 00           	movl	0x260(%rbp), %esi
14024930a: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140249311: 89 30                       	movl	%esi, (%rax)
140249313: 0f b6 47 7d                 	movzbl	0x7d(%rdi), %eax
140249317: 24 01                       	andb	$0x1, %al
140249319: fe c0                       	incb	%al
14024931b: 31 c9                       	xorl	%ecx, %ecx
14024931d: 80 7f 7c 00                 	cmpb	$0x0, 0x7c(%rdi)
140249321: 0f b6 c0                    	movzbl	%al, %eax
140249324: 0f 45 c8                    	cmovnel	%eax, %ecx
140249327: 88 8d a8 00 00 00           	movb	%cl, 0xa8(%rbp)
14024932d: 48 8d 0d ac 7a 75 02        	leaq	0x2757aac(%rip), %rcx   # 0x1429a0de0
140249334: ba 1f 00 00 00              	movl	$0x1f, %edx
140249339: e8 42 41 95 00              	callq	0x140b9d480 <.text+0xb9c480>
14024933e: 48 89 c3                    	movq	%rax, %rbx
140249341: 49 89 d6                    	movq	%rdx, %r14
140249344: 48 8d 0d b4 7a 75 02        	leaq	0x2757ab4(%rip), %rcx   # 0x1429a0dff
14024934b: ba 23 00 00 00              	movl	$0x23, %edx
140249350: e8 2b 41 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249355: 49 89 d1                    	movq	%rdx, %r9
140249358: 4c 8d bd 00 01 00 00        	leaq	0x100(%rbp), %r15
14024935f: 4c 89 f9                    	movq	%r15, %rcx
140249362: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140249369: 49 89 c0                    	movq	%rax, %r8
14024936c: e8 7f 49 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140249371: e8 fa 49 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140249376: 48 8d 85 a8 00 00 00        	leaq	0xa8(%rbp), %rax
14024937d: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140249382: 66 0f 7e 44 24 20           	movd	%xmm0, 0x20(%rsp)
140249388: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
14024938f: 48 89 da                    	movq	%rbx, %rdx
140249392: 4d 89 f0                    	movq	%r14, %r8
140249395: 4d 89 f9                    	movq	%r15, %r9
140249398: e8 33 97 0e 00              	callq	0x140332ad0 <.text+0x331ad0>
14024939d: 84 c0                       	testb	%al, %al
14024939f: 74 1e                       	je	0x1402493bf <.text+0x2483bf>
1402493a1: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1402493a8: c6 00 01                    	movb	$0x1, (%rax)
1402493ab: 0f b6 85 a8 00 00 00        	movzbl	0xa8(%rbp), %eax
1402493b2: 85 c0                       	testl	%eax, %eax
1402493b4: 74 1e                       	je	0x1402493d4 <.text+0x2483d4>
1402493b6: 83 f8 01                    	cmpl	$0x1, %eax
1402493b9: 75 22                       	jne	0x1402493dd <.text+0x2483dd>
1402493bb: 31 c0                       	xorl	%eax, %eax
1402493bd: eb 20                       	jmp	0x1402493df <.text+0x2483df>
1402493bf: 80 bd a8 00 00 00 00        	cmpb	$0x0, 0xa8(%rbp)
1402493c6: 4c 8b ad 50 02 00 00        	movq	0x250(%rbp), %r13
1402493cd: 75 1e                       	jne	0x1402493ed <.text+0x2483ed>
1402493cf: e9 91 09 00 00              	jmp	0x140249d65 <.text+0x248d65>
1402493d4: c6 47 7c 00                 	movb	$0x0, 0x7c(%rdi)
1402493d8: e9 88 09 00 00              	jmp	0x140249d65 <.text+0x248d65>
1402493dd: b0 01                       	movb	$0x1, %al
1402493df: 4c 8b ad 50 02 00 00        	movq	0x250(%rbp), %r13
1402493e6: c6 47 7c 01                 	movb	$0x1, 0x7c(%rdi)
1402493ea: 88 47 7d                    	movb	%al, 0x7d(%rdi)
1402493ed: 49 8b 55 08                 	movq	0x8(%r13), %rdx
1402493f1: 4d 8b 45 10                 	movq	0x10(%r13), %r8
1402493f5: 49 c1 e0 06                 	shlq	$0x6, %r8
1402493f9: 49 01 d0                    	addq	%rdx, %r8
1402493fc: 4c 8d 0d 75 74 75 02        	leaq	0x2757475(%rip), %r9    # 0x1429a0878
140249403: 4c 89 f9                    	movq	%r15, %rcx
140249406: e8 55 2e 07 00              	callq	0x1402bc260 <.text+0x2bb260>
14024940b: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
140249412: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
140249419: 4c 8b b5 10 01 00 00        	movq	0x110(%rbp), %r14
140249420: 49 83 fe 02                 	cmpq	$0x2, %r14
140249424: 0f 83 9a 0b 00 00           	jae	0x140249fc4 <.text+0x248fc4>
14024942a: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
140249431: 4d 85 f6                    	testq	%r14, %r14
140249434: 0f 84 13 02 00 00           	je	0x14024964d <.text+0x24864d>
14024943a: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
140249444: 48 39 47 38                 	cmpq	%rax, 0x38(%rdi)
140249448: 0f 8c ff 01 00 00           	jl	0x14024964d <.text+0x24864d>
14024944e: 48 89 95 10 02 00 00        	movq	%rdx, 0x210(%rbp)
140249455: 48 8d 47 38                 	leaq	0x38(%rdi), %rax
140249459: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
140249460: 48 8b 5f 40                 	movq	0x40(%rdi), %rbx
140249464: 4c 8b 67 48                 	movq	0x48(%rdi), %r12
140249468: 4e 8d 2c f5 00 00 00 00     	leaq	(,%r14,8), %r13
140249470: 31 f6                       	xorl	%esi, %esi
140249472: eb 19                       	jmp	0x14024948d <.text+0x24848d>
140249474: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140249480: 48 ff c6                    	incq	%rsi
140249483: 49 83 c5 f8                 	addq	$-0x8, %r13
140249487: 0f 84 ac 01 00 00           	je	0x140249639 <.text+0x248639>
14024948d: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
140249494: 48 8b 04 f0                 	movq	(%rax,%rsi,8), %rax
140249498: 4c 39 60 10                 	cmpq	%r12, 0x10(%rax)
14024949c: 75 e2                       	jne	0x140249480 <.text+0x248480>
14024949e: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1402494a2: 48 89 da                    	movq	%rbx, %rdx
1402494a5: 4d 89 e0                    	movq	%r12, %r8
1402494a8: e8 43 a7 6d 02              	callq	0x142923bf0 <.text+0x2922bf0>
1402494ad: 85 c0                       	testl	%eax, %eax
1402494af: 75 cf                       	jne	0x140249480 <.text+0x248480>
1402494b1: 48 89 b5 d0 01 00 00        	movq	%rsi, 0x1d0(%rbp)
1402494b8: 48 8d 0d 63 79 75 02        	leaq	0x2757963(%rip), %rcx   # 0x1429a0e22
1402494bf: ba 1b 00 00 00              	movl	$0x1b, %edx
1402494c4: e8 b7 3f 95 00              	callq	0x140b9d480 <.text+0xb9c480>
1402494c9: 4c 8b ad 50 02 00 00        	movq	0x250(%rbp), %r13
1402494d0: 48 89 c3                    	movq	%rax, %rbx
1402494d3: 49 89 d4                    	movq	%rdx, %r12
1402494d6: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
1402494dd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402494e3: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
1402494ea: 45 31 c9                    	xorl	%r9d, %r9d
1402494ed: e8 fe 47 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
1402494f2: e8 79 48 43 00              	callq	0x14067dd70 <.text+0x67cd70>
1402494f7: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1402494fe: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140249505: 4c 89 b5 08 01 00 00        	movq	%r14, 0x108(%rbp)
14024950c: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
140249513: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14024951a: 4c 89 7c 24 28              	movq	%r15, 0x28(%rsp)
14024951f: 66 0f 7e 44 24 20           	movd	%xmm0, 0x20(%rsp)
140249525: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
14024952c: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140249533: 48 89 da                    	movq	%rbx, %rdx
140249536: 4d 89 e0                    	movq	%r12, %r8
140249539: e8 a2 8f 0e 00              	callq	0x1403324e0 <.text+0x3314e0>
14024953e: 84 c0                       	testb	%al, %al
140249540: 0f 84 eb 00 00 00           	je	0x140249631 <.text+0x248631>
140249546: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14024954d: c6 00 01                    	movb	$0x1, (%rax)
140249550: 49 8b 75 10                 	movq	0x10(%r13), %rsi
140249554: 48 85 f6                    	testq	%rsi, %rsi
140249557: 0f 84 d4 00 00 00           	je	0x140249631 <.text+0x248631>
14024955d: 4c 8b bd d0 01 00 00        	movq	0x1d0(%rbp), %r15
140249564: 4d 39 f7                    	cmpq	%r14, %r15
140249567: 0f 83 4e 0c 00 00           	jae	0x14024a1bb <.text+0x2491bb>
14024956d: 49 8b 5d 08                 	movq	0x8(%r13), %rbx
140249571: 48 83 c3 18                 	addq	$0x18, %rbx
140249575: 48 c1 e6 06                 	shlq	$0x6, %rsi
140249579: 45 31 f6                    	xorl	%r14d, %r14d
14024957c: eb 0e                       	jmp	0x14024958c <.text+0x24858c>
14024957e: 48 83 c3 40                 	addq	$0x40, %rbx
140249582: 48 83 c6 c0                 	addq	$-0x40, %rsi
140249586: 0f 84 a5 00 00 00           	je	0x140249631 <.text+0x248631>
14024958c: 4c 3b 33                    	cmpq	(%rbx), %r14
14024958f: 70 ed                       	jo	0x14024957e <.text+0x24857e>
140249591: 4c 8b 43 10                 	movq	0x10(%rbx), %r8
140249595: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
14024959c: 4a 8b 04 f8                 	movq	(%rax,%r15,8), %rax
1402495a0: 4c 3b 40 10                 	cmpq	0x10(%rax), %r8
1402495a4: 75 d8                       	jne	0x14024957e <.text+0x24857e>
1402495a6: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1402495aa: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1402495ae: e8 3d a6 6d 02              	callq	0x142923bf0 <.text+0x2922bf0>
1402495b3: 85 c0                       	testl	%eax, %eax
1402495b5: 75 c7                       	jne	0x14024957e <.text+0x24857e>
1402495b7: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
1402495be: 48 89 da                    	movq	%rbx, %rdx
1402495c1: e8 ca 27 b3 00              	callq	0x140d7bd90 <.text+0xd7ad90>
1402495c6: 0f 10 73 18                 	movups	0x18(%rbx), %xmm6
1402495ca: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
1402495d1: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1402495d8: 0f 10 85 00 01 00 00        	movups	0x100(%rbp), %xmm0
1402495df: 0f 29 85 20 02 00 00        	movaps	%xmm0, 0x220(%rbp)
1402495e6: 48 8b b5 e8 00 00 00        	movq	0xe8(%rbp), %rsi
1402495ed: 48 8b 16                    	movq	(%rsi), %rdx
1402495f0: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1402495fa: 48 39 c2                    	cmpq	%rax, %rdx
1402495fd: 74 19                       	je	0x140249618 <.text+0x248618>
1402495ff: 48 89 d0                    	movq	%rdx, %rax
140249602: 48 f7 d8                    	negq	%rax
140249605: 70 11                       	jo	0x140249618 <.text+0x248618>
140249607: 73 0f                       	jae	0x140249618 <.text+0x248618>
140249609: 48 8b 4f 40                 	movq	0x40(%rdi), %rcx
14024960d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140249613: e8 08 a6 3c 00              	callq	0x140613c20 <.text+0x612c20>
140249618: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
14024961f: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140249623: 0f 28 85 20 02 00 00        	movaps	0x220(%rbp), %xmm0
14024962a: 0f 11 06                    	movups	%xmm0, (%rsi)
14024962d: 0f 11 77 50                 	movups	%xmm6, 0x50(%rdi)
140249631: 8b b5 60 02 00 00           	movl	0x260(%rbp), %esi
140249637: eb 0d                       	jmp	0x140249646 <.text+0x248646>
140249639: 8b b5 60 02 00 00           	movl	0x260(%rbp), %esi
14024963f: 4c 8b ad 50 02 00 00        	movq	0x250(%rbp), %r13
140249646: 48 8b 95 10 02 00 00        	movq	0x210(%rbp), %rdx
14024964d: 48 85 d2                    	testq	%rdx, %rdx
140249650: 74 16                       	je	0x140249668 <.text+0x248668>
140249652: 48 c1 e2 03                 	shlq	$0x3, %rdx
140249656: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14024965c: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
140249663: e8 b8 a5 3c 00              	callq	0x140613c20 <.text+0x612c20>
140249668: 80 bd a8 00 00 00 02        	cmpb	$0x2, 0xa8(%rbp)
14024966f: 0f 85 f0 06 00 00           	jne	0x140249d65 <.text+0x248d65>
140249675: 4d 8b 7d 10                 	movq	0x10(%r13), %r15
140249679: 4d 85 ff                    	testq	%r15, %r15
14024967c: 74 62                       	je	0x1402496e0 <.text+0x2486e0>
14024967e: 49 8b 75 08                 	movq	0x8(%r13), %rsi
140249682: 48 8b 47 38                 	movq	0x38(%rdi), %rax
140249686: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
140249690: 48 39 c8                    	cmpq	%rcx, %rax
140249693: 74 2d                       	je	0x1402496c2 <.text+0x2486c2>
140249695: 48 f7 d8                    	negq	%rax
140249698: 0f 81 9a 01 00 00           	jno	0x140249838 <.text+0x248838>
14024969e: 49 c1 e7 06                 	shlq	$0x6, %r15
1402496a2: 31 c0                       	xorl	%eax, %eax
1402496a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1402496b0: 48 3b 46 18                 	cmpq	0x18(%rsi), %rax
1402496b4: 70 2c                       	jo	0x1402496e2 <.text+0x2486e2>
1402496b6: 48 83 c6 40                 	addq	$0x40, %rsi
1402496ba: 49 83 c7 c0                 	addq	$-0x40, %r15
1402496be: 75 f0                       	jne	0x1402496b0 <.text+0x2486b0>
1402496c0: eb 1e                       	jmp	0x1402496e0 <.text+0x2486e0>
1402496c2: 49 c1 e7 06                 	shlq	$0x6, %r15
1402496c6: 31 c0                       	xorl	%eax, %eax
1402496c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1402496d0: 48 3b 46 18                 	cmpq	0x18(%rsi), %rax
1402496d4: 70 0c                       	jo	0x1402496e2 <.text+0x2486e2>
1402496d6: 48 83 c6 40                 	addq	$0x40, %rsi
1402496da: 49 83 c7 c0                 	addq	$-0x40, %r15
1402496de: 75 f0                       	jne	0x1402496d0 <.text+0x2486d0>
1402496e0: 31 f6                       	xorl	%esi, %esi
1402496e2: 48 c7 85 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rbp)
1402496ed: 48 89 b5 08 01 00 00        	movq	%rsi, 0x108(%rbp)
1402496f4: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
1402496ff: 48 c7 85 20 01 00 00 00 00 00 00    	movq	$0x0, 0x120(%rbp)
14024970a: 4c 8d 05 67 71 75 02        	leaq	0x2757167(%rip), %r8    # 0x1429a0878
140249711: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
140249718: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
14024971f: e8 3c 67 07 00              	callq	0x1402bfe60 <.text+0x2bee60>
140249724: 48 8b b5 c0 01 00 00        	movq	0x1c0(%rbp), %rsi
14024972b: 48 85 f6                    	testq	%rsi, %rsi
14024972e: 0f 84 09 06 00 00           	je	0x140249d3d <.text+0x248d3d>
140249734: 4c 8b b5 b8 01 00 00        	movq	0x1b8(%rbp), %r14
14024973b: 48 8d 1c f5 00 00 00 00     	leaq	(,%rsi,8), %rbx
140249743: 0f b6 05 bf b4 3e 03        	movzbl	0x33eb4bf(%rip), %eax   # 0x143634c09
14024974a: ba 04 00 00 00              	movl	$0x4, %edx
14024974f: 48 89 d9                    	movq	%rbx, %rcx
140249752: e8 b9 a4 3c 00              	callq	0x140613c10 <.text+0x612c10>
140249757: 48 85 c0                    	testq	%rax, %rax
14024975a: 0f 84 cb 09 00 00           	je	0x14024a12b <.text+0x24912b>
140249760: 49 89 c5                    	movq	%rax, %r13
140249763: 89 f0                       	movl	%esi, %eax
140249765: 83 e0 03                    	andl	$0x3, %eax
140249768: 48 83 fe 04                 	cmpq	$0x4, %rsi
14024976c: 73 04                       	jae	0x140249772 <.text+0x248772>
14024976e: 31 c9                       	xorl	%ecx, %ecx
140249770: eb 3e                       	jmp	0x1402497b0 <.text+0x2487b0>
140249772: 48 89 f2                    	movq	%rsi, %rdx
140249775: 48 83 e2 fc                 	andq	$-0x4, %rdx
140249779: 4d 8d 46 30                 	leaq	0x30(%r14), %r8
14024977d: 31 c9                       	xorl	%ecx, %ecx
14024977f: 90                          	nop
140249780: 4d 8b 48 d0                 	movq	-0x30(%r8), %r9
140249784: 4d 89 4c cd 00              	movq	%r9, (%r13,%rcx,8)
140249789: 4d 8b 48 e0                 	movq	-0x20(%r8), %r9
14024978d: 4d 89 4c cd 08              	movq	%r9, 0x8(%r13,%rcx,8)
140249792: 4d 8b 48 f0                 	movq	-0x10(%r8), %r9
140249796: 4d 89 4c cd 10              	movq	%r9, 0x10(%r13,%rcx,8)
14024979b: 4d 8b 08                    	movq	(%r8), %r9
14024979e: 4d 89 4c cd 18              	movq	%r9, 0x18(%r13,%rcx,8)
1402497a3: 48 83 c1 04                 	addq	$0x4, %rcx
1402497a7: 49 83 c0 40                 	addq	$0x40, %r8
1402497ab: 48 39 ca                    	cmpq	%rcx, %rdx
1402497ae: 75 d0                       	jne	0x140249780 <.text+0x248780>
1402497b0: 48 85 c0                    	testq	%rax, %rax
1402497b3: 74 2c                       	je	0x1402497e1 <.text+0x2487e1>
1402497b5: 48 8d 14 cd 00 00 00 00     	leaq	(,%rcx,8), %rdx
1402497bd: 4c 01 ea                    	addq	%r13, %rdx
1402497c0: 48 c1 e1 04                 	shlq	$0x4, %rcx
1402497c4: 4c 01 f1                    	addq	%r14, %rcx
1402497c7: c1 e0 03                    	shll	$0x3, %eax
1402497ca: 45 31 c0                    	xorl	%r8d, %r8d
1402497cd: 0f 1f 00                    	nopl	(%rax)
1402497d0: 4e 8b 0c 41                 	movq	(%rcx,%r8,2), %r9
1402497d4: 4e 89 0c 02                 	movq	%r9, (%rdx,%r8)
1402497d8: 49 83 c0 08                 	addq	$0x8, %r8
1402497dc: 4c 39 c0                    	cmpq	%r8, %rax
1402497df: 75 ef                       	jne	0x1402497d0 <.text+0x2487d0>
1402497e1: b8 01 00 00 00              	movl	$0x1, %eax
1402497e6: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1402497ec: 48 83 fe 01                 	cmpq	$0x1, %rsi
1402497f0: 0f 84 70 01 00 00           	je	0x140249966 <.text+0x248966>
1402497f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140249800: 4b 8b 44 fd 00              	movq	(%r13,%r15,8), %rax
140249805: 4b 3b 44 fd f8              	cmpq	-0x8(%r13,%r15,8), %rax
14024980a: 74 76                       	je	0x140249882 <.text+0x248882>
14024980c: 49 ff c7                    	incq	%r15
14024980f: 4c 39 fe                    	cmpq	%r15, %rsi
140249812: 75 ec                       	jne	0x140249800 <.text+0x248800>
140249814: 49 89 f7                    	movq	%rsi, %r15
140249817: 49 83 ff 02                 	cmpq	$0x2, %r15
14024981b: 0f 82 39 01 00 00           	jb	0x14024995a <.text+0x24895a>
140249821: 4c 89 f8                    	movq	%r15, %rax
140249824: 48 d1 e8                    	shrq	%rax
140249827: 49 83 ff 08                 	cmpq	$0x8, %r15
14024982b: 0f 83 81 00 00 00           	jae	0x1402498b2 <.text+0x2488b2>
140249831: 31 c9                       	xorl	%ecx, %ecx
140249833: e9 ec 00 00 00              	jmp	0x140249924 <.text+0x248924>
140249838: 48 8b 5f 40                 	movq	0x40(%rdi), %rbx
14024983c: 4c 8b 77 48                 	movq	0x48(%rdi), %r14
140249840: 49 c1 e7 06                 	shlq	$0x6, %r15
140249844: 45 31 e4                    	xorl	%r12d, %r12d
140249847: eb 15                       	jmp	0x14024985e <.text+0x24885e>
140249849: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140249850: 48 83 c6 40                 	addq	$0x40, %rsi
140249854: 49 83 c7 c0                 	addq	$-0x40, %r15
140249858: 0f 84 82 fe ff ff           	je	0x1402496e0 <.text+0x2486e0>
14024985e: 4c 3b 66 18                 	cmpq	0x18(%rsi), %r12
140249862: 70 ec                       	jo	0x140249850 <.text+0x248850>
140249864: 4c 39 76 28                 	cmpq	%r14, 0x28(%rsi)
140249868: 75 e6                       	jne	0x140249850 <.text+0x248850>
14024986a: 48 8b 4e 20                 	movq	0x20(%rsi), %rcx
14024986e: 48 89 da                    	movq	%rbx, %rdx
140249871: 4d 89 f0                    	movq	%r14, %r8
140249874: e8 77 a3 6d 02              	callq	0x142923bf0 <.text+0x2922bf0>
140249879: 85 c0                       	testl	%eax, %eax
14024987b: 75 d3                       	jne	0x140249850 <.text+0x248850>
14024987d: e9 60 fe ff ff              	jmp	0x1402496e2 <.text+0x2486e2>
140249882: 49 8d 47 01                 	leaq	0x1(%r15), %rax
140249886: eb 0b                       	jmp	0x140249893 <.text+0x248893>
140249888: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140249890: 48 ff c0                    	incq	%rax
140249893: 48 39 f0                    	cmpq	%rsi, %rax
140249896: 0f 83 7b ff ff ff           	jae	0x140249817 <.text+0x248817>
14024989c: 49 8b 4c c5 00              	movq	(%r13,%rax,8), %rcx
1402498a1: 4b 3b 4c fd f8              	cmpq	-0x8(%r13,%r15,8), %rcx
1402498a6: 74 e8                       	je	0x140249890 <.text+0x248890>
1402498a8: 4b 89 4c fd 00              	movq	%rcx, (%r13,%r15,8)
1402498ad: 49 ff c7                    	incq	%r15
1402498b0: eb de                       	jmp	0x140249890 <.text+0x248890>
1402498b2: 48 89 c1                    	movq	%rax, %rcx
1402498b5: 48 83 e1 fc                 	andq	$-0x4, %rcx
1402498b9: 4a 8d 14 fd f0 ff ff ff     	leaq	-0x10(,%r15,8), %rdx
1402498c1: 4c 01 ea                    	addq	%r13, %rdx
1402498c4: 45 31 c0                    	xorl	%r8d, %r8d
1402498c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1402498d0: f3 43 0f 6f 44 c5 00        	movdqu	(%r13,%r8,8), %xmm0
1402498d7: f3 43 0f 6f 4c c5 10        	movdqu	0x10(%r13,%r8,8), %xmm1
1402498de: f3 0f 6f 52 f0              	movdqu	-0x10(%rdx), %xmm2
1402498e3: f3 0f 6f 1a                 	movdqu	(%rdx), %xmm3
1402498e7: 66 0f 70 db 4e              	pshufd	$0x4e, %xmm3, %xmm3     # xmm3 = xmm3[2,3,0,1]
1402498ec: 66 0f 70 d2 4e              	pshufd	$0x4e, %xmm2, %xmm2     # xmm2 = xmm2[2,3,0,1]
1402498f1: f3 43 0f 7f 5c c5 00        	movdqu	%xmm3, (%r13,%r8,8)
1402498f8: f3 43 0f 7f 54 c5 10        	movdqu	%xmm2, 0x10(%r13,%r8,8)
1402498ff: 66 0f 70 c0 4e              	pshufd	$0x4e, %xmm0, %xmm0     # xmm0 = xmm0[2,3,0,1]
140249904: f3 0f 7f 02                 	movdqu	%xmm0, (%rdx)
140249908: 66 0f 70 c1 4e              	pshufd	$0x4e, %xmm1, %xmm0     # xmm0 = xmm1[2,3,0,1]
14024990d: f3 0f 7f 42 f0              	movdqu	%xmm0, -0x10(%rdx)
140249912: 49 83 c0 04                 	addq	$0x4, %r8
140249916: 48 83 c2 e0                 	addq	$-0x20, %rdx
14024991a: 4c 39 c1                    	cmpq	%r8, %rcx
14024991d: 75 b1                       	jne	0x1402498d0 <.text+0x2488d0>
14024991f: 48 39 c8                    	cmpq	%rcx, %rax
140249922: 74 36                       	je	0x14024995a <.text+0x24895a>
140249924: 48 f7 d8                    	negq	%rax
140249927: 48 8d 14 cd 00 00 00 00     	leaq	(,%rcx,8), %rdx
14024992f: 4c 01 ea                    	addq	%r13, %rdx
140249932: 48 f7 d9                    	negq	%rcx
140249935: 4e 8d 04 fd f8 ff ff ff     	leaq	-0x8(,%r15,8), %r8
14024993d: 4d 01 e8                    	addq	%r13, %r8
140249940: 4c 8b 0a                    	movq	(%rdx), %r9
140249943: 4d 8b 14 c8                 	movq	(%r8,%rcx,8), %r10
140249947: 4c 89 12                    	movq	%r10, (%rdx)
14024994a: 4d 89 0c c8                 	movq	%r9, (%r8,%rcx,8)
14024994e: 48 ff c9                    	decq	%rcx
140249951: 48 83 c2 08                 	addq	$0x8, %rdx
140249955: 48 39 c8                    	cmpq	%rcx, %rax
140249958: 75 e6                       	jne	0x140249940 <.text+0x248940>
14024995a: 48 89 f0                    	movq	%rsi, %rax
14024995d: 4d 85 ff                    	testq	%r15, %r15
140249960: 0f 84 31 02 00 00           	je	0x140249b97 <.text+0x248b97>
140249966: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
14024996d: 4c 89 ad 70 02 00 00        	movq	%r13, 0x270(%rbp)
140249974: 48 8d 57 68                 	leaq	0x68(%rdi), %rdx
140249978: 4c 8d a5 00 01 00 00        	leaq	0x100(%rbp), %r12
14024997f: 4c 89 e1                    	movq	%r12, %rcx
140249982: 48 89 95 50 02 00 00        	movq	%rdx, 0x250(%rbp)
140249989: e8 42 f1 71 00              	callq	0x140968ad0 <.text+0x967ad0>
14024998e: 48 c1 e6 04                 	shlq	$0x4, %rsi
140249992: f6 85 00 01 00 00 01        	testb	$0x1, 0x100(%rbp)
140249999: 74 37                       	je	0x1402499d2 <.text+0x2489d2>
14024999b: 48 8b 85 04 01 00 00        	movq	0x104(%rbp), %rax
1402499a2: 4a 8d 14 fd 00 00 00 00     	leaq	(,%r15,8), %rdx
1402499aa: 31 c9                       	xorl	%ecx, %ecx
1402499ac: 4c 8b ad 70 02 00 00        	movq	0x270(%rbp), %r13
1402499b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1402499c0: 49 39 44 cd 00              	cmpq	%rax, (%r13,%rcx,8)
1402499c5: 74 7f                       	je	0x140249a46 <.text+0x248a46>
1402499c7: 48 ff c1                    	incq	%rcx
1402499ca: 48 83 c2 f8                 	addq	$-0x8, %rdx
1402499ce: 75 f0                       	jne	0x1402499c0 <.text+0x2489c0>
1402499d0: eb 07                       	jmp	0x1402499d9 <.text+0x2489d9>
1402499d2: 4c 8b ad 70 02 00 00        	movq	0x270(%rbp), %r13
1402499d9: 49 8d 4f ff                 	leaq	-0x1(%r15), %rcx
1402499dd: 48 89 8d d0 01 00 00        	movq	%rcx, 0x1d0(%rbp)
1402499e4: 4c 39 f9                    	cmpq	%r15, %rcx
1402499e7: 0f 83 fa 00 00 00           	jae	0x140249ae7 <.text+0x248ae7>
1402499ed: 49 8b 44 cd 00              	movq	(%r13,%rcx,8), %rax
1402499f2: 31 c9                       	xorl	%ecx, %ecx
1402499f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140249a00: 48 39 ce                    	cmpq	%rcx, %rsi
140249a03: 0f 84 f2 00 00 00           	je	0x140249afb <.text+0x248afb>
140249a09: 48 8d 51 10                 	leaq	0x10(%rcx), %rdx
140249a0d: 49 39 04 0e                 	cmpq	%rax, (%r14,%rcx)
140249a11: 48 89 d1                    	movq	%rdx, %rcx
140249a14: 75 ea                       	jne	0x140249a00 <.text+0x248a00>
140249a16: 41 0f 10 44 16 f0           	movups	-0x10(%r14,%rdx), %xmm0
140249a1c: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
140249a23: c7 47 68 01 00 00 00        	movl	$0x1, 0x68(%rdi)
140249a2a: 66 0f 6f 85 00 01 00 00     	movdqa	0x100(%rbp), %xmm0
140249a32: f3 0f 7f 47 6c              	movdqu	%xmm0, 0x6c(%rdi)
140249a37: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140249a3e: c6 00 01                    	movb	$0x1, (%rax)
140249a41: e9 b5 00 00 00              	jmp	0x140249afb <.text+0x248afb>
140249a46: 48 89 8d d0 01 00 00        	movq	%rcx, 0x1d0(%rbp)
140249a4d: 48 8d 0d e9 73 75 02        	leaq	0x27573e9(%rip), %rcx   # 0x1429a0e3d
140249a54: ba 1e 00 00 00              	movl	$0x1e, %edx
140249a59: e8 22 3a 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249a5e: 48 89 c3                    	movq	%rax, %rbx
140249a61: 49 89 d5                    	movq	%rdx, %r13
140249a64: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140249a6b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140249a71: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140249a78: 45 31 c9                    	xorl	%r9d, %r9d
140249a7b: e8 70 42 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140249a80: e8 eb 42 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140249a85: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
140249a8c: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140249a93: 4c 89 bd 08 01 00 00        	movq	%r15, 0x108(%rbp)
140249a9a: 48 8d 85 d0 01 00 00        	leaq	0x1d0(%rbp), %rax
140249aa1: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140249aa8: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140249aad: 66 0f 7e 44 24 20           	movd	%xmm0, 0x20(%rsp)
140249ab3: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
140249aba: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140249ac1: 48 89 da                    	movq	%rbx, %rdx
140249ac4: 4d 89 e8                    	movq	%r13, %r8
140249ac7: e8 34 79 0e 00              	callq	0x140331400 <.text+0x330400>
140249acc: 84 c0                       	testb	%al, %al
140249ace: 4c 8b ad 70 02 00 00        	movq	0x270(%rbp), %r13
140249ad5: 74 24                       	je	0x140249afb <.text+0x248afb>
140249ad7: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
140249ade: 4c 39 f9                    	cmpq	%r15, %rcx
140249ae1: 0f 82 06 ff ff ff           	jb	0x1402499ed <.text+0x2489ed>
140249ae7: 4c 8d 05 72 73 75 02        	leaq	0x2757372(%rip), %r8    # 0x1429a0e60
140249aee: 4c 89 fa                    	movq	%r15, %rdx
140249af1: e8 22 fa 68 02              	callq	0x1428d9518 <.text+0x28d8518>
140249af6: e9 d2 06 00 00              	jmp	0x14024a1cd <.text+0x2491cd>
140249afb: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
140249b02: f6 00 01                    	testb	$0x1, (%rax)
140249b05: 0f 84 19 02 00 00           	je	0x140249d24 <.text+0x248d24>
140249b0b: 4c 01 f6                    	addq	%r14, %rsi
140249b0e: f3 0f 6f 47 6c              	movdqu	0x6c(%rdi), %xmm0
140249b13: 66 0f 7f 45 60              	movdqa	%xmm0, 0x60(%rbp)
140249b18: 4c 89 b5 00 01 00 00        	movq	%r14, 0x100(%rbp)
140249b1f: 48 89 b5 08 01 00 00        	movq	%rsi, 0x108(%rbp)
140249b26: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
140249b2a: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140249b31: 4c 8d 05 40 6d 75 02        	leaq	0x2756d40(%rip), %r8    # 0x1429a0878
140249b38: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140249b3f: 48 8d b5 00 01 00 00        	leaq	0x100(%rbp), %rsi
140249b46: 48 89 f2                    	movq	%rsi, %rdx
140249b49: e8 c2 6d 07 00              	callq	0x1402c0910 <.text+0x2bf910>
140249b4e: 4c 8b bd d8 01 00 00        	movq	0x1d8(%rbp), %r15
140249b55: 4c 8b ad e0 01 00 00        	movq	0x1e0(%rbp), %r13
140249b5c: 49 83 fd 01                 	cmpq	$0x1, %r13
140249b60: 0f 87 b1 04 00 00           	ja	0x14024a017 <.text+0x249017>
140249b66: 4d 85 ed                    	testq	%r13, %r13
140249b69: 74 34                       	je	0x140249b9f <.text+0x248b9f>
140249b6b: 4b 8d 04 af                 	leaq	(%r15,%r13,4), %rax
140249b6f: 8b 55 68                    	movl	0x68(%rbp), %edx
140249b72: 31 c9                       	xorl	%ecx, %ecx
140249b74: 4d 89 f8                    	movq	%r15, %r8
140249b77: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140249b80: 41 3b 10                    	cmpl	(%r8), %edx
140249b83: 0f 84 b5 00 00 00           	je	0x140249c3e <.text+0x248c3e>
140249b89: 49 83 c0 04                 	addq	$0x4, %r8
140249b8d: 48 ff c1                    	incq	%rcx
140249b90: 49 39 c0                    	cmpq	%rax, %r8
140249b93: 75 eb                       	jne	0x140249b80 <.text+0x248b80>
140249b95: eb 0b                       	jmp	0x140249ba2 <.text+0x248ba2>
140249b97: 48 89 f2                    	movq	%rsi, %rdx
140249b9a: e9 8c 01 00 00              	jmp	0x140249d2b <.text+0x248d2b>
140249b9f: 45 31 ed                    	xorl	%r13d, %r13d
140249ba2: 49 8d 4d ff                 	leaq	-0x1(%r13), %rcx
140249ba6: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
140249baa: 4c 39 e9                    	cmpq	%r13, %rcx
140249bad: 0f 83 31 01 00 00           	jae	0x140249ce4 <.text+0x248ce4>
140249bb3: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
140249bba: 48 85 d2                    	testq	%rdx, %rdx
140249bbd: 0f 84 3c 01 00 00           	je	0x140249cff <.text+0x248cff>
140249bc3: 48 8d 47 6c                 	leaq	0x6c(%rdi), %rax
140249bc7: 45 8b 04 8f                 	movl	(%r15,%rcx,4), %r8d
140249bcb: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140249bd2: 4c 8b 4d 60                 	movq	0x60(%rbp), %r9
140249bd6: 48 c1 e2 04                 	shlq	$0x4, %rdx
140249bda: 45 31 d2                    	xorl	%r10d, %r10d
140249bdd: 4c 8b ad 70 02 00 00        	movq	0x270(%rbp), %r13
140249be4: eb 17                       	jmp	0x140249bfd <.text+0x248bfd>
140249be6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140249bf0: 49 83 c2 10                 	addq	$0x10, %r10
140249bf4: 4c 39 d2                    	cmpq	%r10, %rdx
140249bf7: 0f 84 09 01 00 00           	je	0x140249d06 <.text+0x248d06>
140249bfd: 4e 39 0c 11                 	cmpq	%r9, (%rcx,%r10)
140249c01: 75 ed                       	jne	0x140249bf0 <.text+0x248bf0>
140249c03: 46 39 44 11 08              	cmpl	%r8d, 0x8(%rcx,%r10)
140249c08: 75 e6                       	jne	0x140249bf0 <.text+0x248bf0>
140249c0a: 42 0f 10 04 11              	movups	(%rcx,%r10), %xmm0
140249c0f: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
140249c16: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
140249c1d: c7 01 01 00 00 00           	movl	$0x1, (%rcx)
140249c23: 66 0f 6f 85 00 01 00 00     	movdqa	0x100(%rbp), %xmm0
140249c2b: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
140249c2f: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140249c36: c6 00 01                    	movb	$0x1, (%rax)
140249c39: e9 c8 00 00 00              	jmp	0x140249d06 <.text+0x248d06>
140249c3e: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
140249c42: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
140249c49: 48 8d 0d 28 72 75 02        	leaq	0x2757228(%rip), %rcx   # 0x1429a0e78
140249c50: ba 1f 00 00 00              	movl	$0x1f, %edx
140249c55: e8 26 38 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249c5a: 48 89 c3                    	movq	%rax, %rbx
140249c5d: 49 89 d4                    	movq	%rdx, %r12
140249c60: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
140249c67: 48 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %rcx
140249c6e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140249c74: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140249c7b: 45 31 c9                    	xorl	%r9d, %r9d
140249c7e: e8 6d 40 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140249c83: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
140249c8a: e8 e1 40 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140249c8f: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
140249c96: 4c 89 ad 08 01 00 00        	movq	%r13, 0x108(%rbp)
140249c9d: 48 8d 45 30                 	leaq	0x30(%rbp), %rax
140249ca1: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140249ca8: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
140249caf: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
140249cb4: 66 0f 7e 44 24 20           	movd	%xmm0, 0x20(%rsp)
140249cba: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
140249cc1: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140249cc8: 48 89 da                    	movq	%rbx, %rdx
140249ccb: 4d 89 e0                    	movq	%r12, %r8
140249cce: e8 3d 71 0e 00              	callq	0x140330e10 <.text+0x32fe10>
140249cd3: 84 c0                       	testb	%al, %al
140249cd5: 74 28                       	je	0x140249cff <.text+0x248cff>
140249cd7: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140249cdb: 4c 39 e9                    	cmpq	%r13, %rcx
140249cde: 0f 82 cf fe ff ff           	jb	0x140249bb3 <.text+0x248bb3>
140249ce4: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
140249ceb: 4c 8d 05 a6 71 75 02        	leaq	0x27571a6(%rip), %r8    # 0x1429a0e98
140249cf2: 4c 89 ea                    	movq	%r13, %rdx
140249cf5: e8 1e f8 68 02              	callq	0x1428d9518 <.text+0x28d8518>
140249cfa: e9 ce 04 00 00              	jmp	0x14024a1cd <.text+0x2491cd>
140249cff: 4c 8b ad 70 02 00 00        	movq	0x270(%rbp), %r13
140249d06: 48 8b 95 d0 01 00 00        	movq	0x1d0(%rbp), %rdx
140249d0d: 48 85 d2                    	testq	%rdx, %rdx
140249d10: 74 12                       	je	0x140249d24 <.text+0x248d24>
140249d12: 48 c1 e2 02                 	shlq	$0x2, %rdx
140249d16: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140249d1c: 4c 89 f9                    	movq	%r15, %rcx
140249d1f: e8 fc 9e 3c 00              	callq	0x140613c20 <.text+0x612c20>
140249d24: 48 8b 95 10 02 00 00        	movq	0x210(%rbp), %rdx
140249d2b: 48 c1 e2 03                 	shlq	$0x3, %rdx
140249d2f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140249d35: 4c 89 e9                    	movq	%r13, %rcx
140249d38: e8 e3 9e 3c 00              	callq	0x140613c20 <.text+0x612c20>
140249d3d: 48 8b 95 b0 01 00 00        	movq	0x1b0(%rbp), %rdx
140249d44: 48 85 d2                    	testq	%rdx, %rdx
140249d47: 74 16                       	je	0x140249d5f <.text+0x248d5f>
140249d49: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140249d50: 48 c1 e2 04                 	shlq	$0x4, %rdx
140249d54: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140249d5a: e8 c1 9e 3c 00              	callq	0x140613c20 <.text+0x612c20>
140249d5f: 8b b5 60 02 00 00           	movl	0x260(%rbp), %esi
140249d65: 48 8d 0d 44 71 75 02        	leaq	0x2757144(%rip), %rcx   # 0x1429a0eb0
140249d6c: ba 19 00 00 00              	movl	$0x19, %edx
140249d71: e8 0a 37 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249d76: 48 89 c3                    	movq	%rax, %rbx
140249d79: 49 89 d7                    	movq	%rdx, %r15
140249d7c: 48 8d 0d 46 71 75 02        	leaq	0x2757146(%rip), %rcx   # 0x1429a0ec9
140249d83: ba 1d 00 00 00              	movl	$0x1d, %edx
140249d88: e8 f3 36 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249d8d: 49 89 d1                    	movq	%rdx, %r9
140249d90: 4c 8d b5 00 01 00 00        	leaq	0x100(%rbp), %r14
140249d97: 4c 89 f1                    	movq	%r14, %rcx
140249d9a: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140249da1: 49 89 c0                    	movq	%rax, %r8
140249da4: e8 47 3f 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140249da9: e8 c2 3f 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140249dae: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140249db5: 89 30                       	movl	%esi, (%rax)
140249db7: 48 8d 87 a8 00 00 00        	leaq	0xa8(%rdi), %rax
140249dbe: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140249dc3: 66 0f 7e 44 24 20           	movd	%xmm0, 0x20(%rsp)
140249dc9: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140249dd0: 48 89 da                    	movq	%rbx, %rdx
140249dd3: 4d 89 f8                    	movq	%r15, %r8
140249dd6: 4d 89 f1                    	movq	%r14, %r9
140249dd9: e8 62 6b 0e 00              	callq	0x140330940 <.text+0x32f940>
140249dde: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
140249de5: 08 01                       	orb	%al, (%rcx)
140249de7: 0f 28 05 a2 69 75 02        	movaps	0x27569a2(%rip), %xmm0  # 0x1429a0790
140249dee: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
140249df5: 0f 28 05 a4 69 75 02        	movaps	0x27569a4(%rip), %xmm0  # 0x1429a07a0
140249dfc: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
140249e03: 0f 28 05 a6 69 75 02        	movaps	0x27569a6(%rip), %xmm0  # 0x1429a07b0
140249e0a: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
140249e11: 0f 28 05 a8 69 75 02        	movaps	0x27569a8(%rip), %xmm0  # 0x1429a07c0
140249e18: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
140249e1f: c7 85 40 01 00 00 01 00 00 00       	movl	$0x1, 0x140(%rbp)
140249e29: f3 0f 10 87 98 00 00 00     	movss	0x98(%rdi), %xmm0
140249e31: f3 48 0f 2c d0              	cvttss2si	%xmm0, %rdx
140249e36: 31 c0                       	xorl	%eax, %eax
140249e38: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140249e3c: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140249e3f: 0f 42 d0                    	cmovbl	%eax, %edx
140249e42: 0f 2e 05 87 69 75 02        	ucomiss	0x2756987(%rip), %xmm0  # 0x1429a07d0
140249e49: b9 ff ff ff ff              	movl	$0xffffffff, %ecx       # imm = 0xFFFFFFFF
140249e4e: 0f 46 ca                    	cmovbel	%edx, %ecx
140249e51: 83 f9 3b                    	cmpl	$0x3b, %ecx
140249e54: 7e 26                       	jle	0x140249e7c <.text+0x248e7c>
140249e56: 8d 41 c4                    	leal	-0x3c(%rcx), %eax
140249e59: 83 f8 1e                    	cmpl	$0x1e, %eax
140249e5c: 4c 8d ad 20 02 00 00        	leaq	0x220(%rbp), %r13
140249e63: 77 34                       	ja	0x140249e99 <.text+0x248e99>
140249e65: 48 8d 0d 5c 78 75 02        	leaq	0x275785c(%rip), %rcx   # 0x1429a16c8
140249e6c: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140249e70: 48 01 c8                    	addq	%rcx, %rax
140249e73: ff e0                       	jmpq	*%rax
140249e75: b8 03 00 00 00              	movl	$0x3, %eax
140249e7a: eb 4a                       	jmp	0x140249ec6 <.text+0x248ec6>
140249e7c: 83 f9 0f                    	cmpl	$0xf, %ecx
140249e7f: 4c 8d ad 20 02 00 00        	leaq	0x220(%rbp), %r13
140249e86: 74 3e                       	je	0x140249ec6 <.text+0x248ec6>
140249e88: 83 f9 14                    	cmpl	$0x14, %ecx
140249e8b: 74 34                       	je	0x140249ec1 <.text+0x248ec1>
140249e8d: 83 f9 1e                    	cmpl	$0x1e, %ecx
140249e90: 75 13                       	jne	0x140249ea5 <.text+0x248ea5>
140249e92: b8 02 00 00 00              	movl	$0x2, %eax
140249e97: eb 2d                       	jmp	0x140249ec6 <.text+0x248ec6>
140249e99: 83 f9 78                    	cmpl	$0x78, %ecx
140249e9c: 75 07                       	jne	0x140249ea5 <.text+0x248ea5>
140249e9e: b8 07 00 00 00              	movl	$0x7, %eax
140249ea3: eb 21                       	jmp	0x140249ec6 <.text+0x248ec6>
140249ea5: b8 08 00 00 00              	movl	$0x8, %eax
140249eaa: eb 1a                       	jmp	0x140249ec6 <.text+0x248ec6>
140249eac: b8 06 00 00 00              	movl	$0x6, %eax
140249eb1: eb 13                       	jmp	0x140249ec6 <.text+0x248ec6>
140249eb3: b8 04 00 00 00              	movl	$0x4, %eax
140249eb8: eb 0c                       	jmp	0x140249ec6 <.text+0x248ec6>
140249eba: b8 05 00 00 00              	movl	$0x5, %eax
140249ebf: eb 05                       	jmp	0x140249ec6 <.text+0x248ec6>
140249ec1: b8 01 00 00 00              	movl	$0x1, %eax
140249ec6: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
140249ecd: 48 8d 0d 12 70 75 02        	leaq	0x2757012(%rip), %rcx   # 0x1429a0ee6
140249ed4: ba 22 00 00 00              	movl	$0x22, %edx
140249ed9: e8 a2 35 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249ede: 48 89 c3                    	movq	%rax, %rbx
140249ee1: 49 89 d7                    	movq	%rdx, %r15
140249ee4: 48 8d 0d 1d 70 75 02        	leaq	0x275701d(%rip), %rcx   # 0x1429a0f08
140249eeb: ba 26 00 00 00              	movl	$0x26, %edx
140249ef0: e8 8b 35 95 00              	callq	0x140b9d480 <.text+0xb9c480>
140249ef5: 49 89 d1                    	movq	%rdx, %r9
140249ef8: 4c 8d a5 d0 01 00 00        	leaq	0x1d0(%rbp), %r12
140249eff: 4c 89 e1                    	movq	%r12, %rcx
140249f02: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
140249f09: 49 89 c0                    	movq	%rax, %r8
140249f0c: e8 df 3d 43 00              	callq	0x14067dcf0 <.text+0x67ccf0>
140249f11: e8 5a 3e 43 00              	callq	0x14067dd70 <.text+0x67cd70>
140249f16: 4c 89 b5 20 02 00 00        	movq	%r14, 0x220(%rbp)
140249f1d: 48 c7 85 28 02 00 00 09 00 00 00    	movq	$0x9, 0x228(%rbp)
140249f28: 48 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %rax
140249f2f: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
140249f36: 4c 89 6c 24 28              	movq	%r13, 0x28(%rsp)
140249f3b: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140249f41: 48 8b 8d 68 02 00 00        	movq	0x268(%rbp), %rcx
140249f48: 48 89 da                    	movq	%rbx, %rdx
140249f4b: 4d 89 f8                    	movq	%r15, %r8
140249f4e: 4d 89 e1                    	movq	%r12, %r9
140249f51: e8 3a 96 0e 00              	callq	0x140333590 <.text+0x332590>
140249f56: 84 c0                       	testb	%al, %al
140249f58: 74 48                       	je	0x140249fa2 <.text+0x248fa2>
140249f5a: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
140249f61: c6 00 01                    	movb	$0x1, (%rax)
140249f64: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
140249f6b: 48 83 f9 08                 	cmpq	$0x8, %rcx
140249f6f: 0f 87 8f 01 00 00           	ja	0x14024a104 <.text+0x249104>
140249f75: 0f 57 c0                    	xorps	%xmm0, %xmm0
140249f78: f6 84 cd 00 01 00 00 01     	testb	$0x1, 0x100(%rbp,%rcx,8)
140249f80: 75 0f                       	jne	0x140249f91 <.text+0x248f91>
140249f82: 8b 84 cd 04 01 00 00        	movl	0x104(%rbp,%rcx,8), %eax
140249f89: 0f 57 c0                    	xorps	%xmm0, %xmm0
140249f8c: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
140249f91: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140249f98: 89 30                       	movl	%esi, (%rax)
140249f9a: f3 0f 11 87 98 00 00 00     	movss	%xmm0, 0x98(%rdi)
140249fa2: 0f 28 b5 80 02 00 00        	movaps	0x280(%rbp), %xmm6
140249fa9: 0f 28 bd 90 02 00 00        	movaps	0x290(%rbp), %xmm7
140249fb0: 48 81 c4 28 03 00 00        	addq	$0x328, %rsp            # imm = 0x328
140249fb7: 5b                          	popq	%rbx
140249fb8: 5f                          	popq	%rdi
140249fb9: 5e                          	popq	%rsi
140249fba: 41 5c                       	popq	%r12
140249fbc: 41 5d                       	popq	%r13
140249fbe: 41 5e                       	popq	%r14
140249fc0: 41 5f                       	popq	%r15
140249fc2: 5d                          	popq	%rbp
140249fc3: c3                          	retq
140249fc4: 49 83 fe 15                 	cmpq	$0x15, %r14
140249fc8: 0f 83 76 01 00 00           	jae	0x14024a144 <.text+0x249144>
140249fce: 4c 8d 8d 20 02 00 00        	leaq	0x220(%rbp), %r9
140249fd5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140249fdb: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
140249fe2: 4c 89 f2                    	movq	%r14, %rdx
140249fe5: e8 a6 94 37 00              	callq	0x1405c3490 <.text+0x5c2490>
140249fea: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
140249ff1: 8b b5 60 02 00 00           	movl	0x260(%rbp), %esi
140249ff7: 4c 8b ad 50 02 00 00        	movq	0x250(%rbp), %r13
140249ffe: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
14024a008: 48 39 47 38                 	cmpq	%rax, 0x38(%rdi)
14024a00c: 0f 8d 3c f4 ff ff           	jge	0x14024944e <.text+0x24844e>
14024a012: e9 36 f6 ff ff              	jmp	0x14024964d <.text+0x24864d>
14024a017: 49 83 fd 15                 	cmpq	$0x15, %r13
14024a01b: 0f 83 7c 01 00 00           	jae	0x14024a19d <.text+0x24919d>
14024a021: 4b 8d 04 af                 	leaq	(%r15,%r13,4), %rax
14024a025: 49 8d 4f 04                 	leaq	0x4(%r15), %rcx
14024a029: ba 04 00 00 00              	movl	$0x4, %edx
14024a02e: eb 13                       	jmp	0x14024a043 <.text+0x249043>
14024a030: 4d 89 f9                    	movq	%r15, %r9
14024a033: 45 89 01                    	movl	%r8d, (%r9)
14024a036: 48 83 c1 04                 	addq	$0x4, %rcx
14024a03a: 48 83 c2 04                 	addq	$0x4, %rdx
14024a03e: 48 39 c1                    	cmpq	%rax, %rcx
14024a041: 74 2c                       	je	0x14024a06f <.text+0x24906f>
14024a043: 44 8b 51 fc                 	movl	-0x4(%rcx), %r10d
14024a047: 44 8b 01                    	movl	(%rcx), %r8d
14024a04a: 45 39 d0                    	cmpl	%r10d, %r8d
14024a04d: 73 e7                       	jae	0x14024a036 <.text+0x249036>
14024a04f: 49 89 d1                    	movq	%rdx, %r9
14024a052: 47 89 14 0f                 	movl	%r10d, (%r15,%r9)
14024a056: 49 83 f9 04                 	cmpq	$0x4, %r9
14024a05a: 74 d4                       	je	0x14024a030 <.text+0x249030>
14024a05c: 47 8b 54 0f f8              	movl	-0x8(%r15,%r9), %r10d
14024a061: 49 83 c1 fc                 	addq	$-0x4, %r9
14024a065: 45 39 d0                    	cmpl	%r10d, %r8d
14024a068: 72 e8                       	jb	0x14024a052 <.text+0x249052>
14024a06a: 4d 01 f9                    	addq	%r15, %r9
14024a06d: eb c4                       	jmp	0x14024a033 <.text+0x249033>
14024a06f: 4c 8b bd d8 01 00 00        	movq	0x1d8(%rbp), %r15
14024a076: 4c 8b ad e0 01 00 00        	movq	0x1e0(%rbp), %r13
14024a07d: 49 83 fd 02                 	cmpq	$0x2, %r13
14024a081: 0f 82 df fa ff ff           	jb	0x140249b66 <.text+0x248b66>
14024a087: b8 01 00 00 00              	movl	$0x1, %eax
14024a08c: 41 8b 0c 87                 	movl	(%r15,%rax,4), %ecx
14024a090: 41 3b 4c 87 fc              	cmpl	-0x4(%r15,%rax,4), %ecx
14024a095: 74 0d                       	je	0x14024a0a4 <.text+0x2490a4>
14024a097: 48 ff c0                    	incq	%rax
14024a09a: 49 39 c5                    	cmpq	%rax, %r13
14024a09d: 75 ed                       	jne	0x14024a08c <.text+0x24908c>
14024a09f: e9 c7 fa ff ff              	jmp	0x140249b6b <.text+0x248b6b>
14024a0a4: 48 8d 48 01                 	leaq	0x1(%rax), %rcx
14024a0a8: eb 03                       	jmp	0x14024a0ad <.text+0x2490ad>
14024a0aa: 48 ff c1                    	incq	%rcx
14024a0ad: 4c 39 e9                    	cmpq	%r13, %rcx
14024a0b0: 73 14                       	jae	0x14024a0c6 <.text+0x2490c6>
14024a0b2: 41 8b 14 8f                 	movl	(%r15,%rcx,4), %edx
14024a0b6: 41 3b 54 87 fc              	cmpl	-0x4(%r15,%rax,4), %edx
14024a0bb: 74 ed                       	je	0x14024a0aa <.text+0x2490aa>
14024a0bd: 41 89 14 87                 	movl	%edx, (%r15,%rax,4)
14024a0c1: 48 ff c0                    	incq	%rax
14024a0c4: eb e4                       	jmp	0x14024a0aa <.text+0x2490aa>
14024a0c6: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
14024a0cd: 49 89 c5                    	movq	%rax, %r13
14024a0d0: e9 91 fa ff ff              	jmp	0x140249b66 <.text+0x248b66>
14024a0d5: b9 04 00 00 00              	movl	$0x4, %ecx
14024a0da: ba 30 00 00 00              	movl	$0x30, %edx
14024a0df: e8 1c ef 68 02              	callq	0x1428d9000 <.text+0x28d8000>
14024a0e4: e9 e4 00 00 00              	jmp	0x14024a1cd <.text+0x2491cd>
14024a0e9: 4c 8d 05 88 67 75 02        	leaq	0x2756788(%rip), %r8    # 0x1429a0878
14024a0f0: b9 01 00 00 00              	movl	$0x1, %ecx
14024a0f5: ba 04 00 00 00              	movl	$0x4, %edx
14024a0fa: e8 e4 ee 68 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14024a0ff: e9 c9 00 00 00              	jmp	0x14024a1cd <.text+0x2491cd>
14024a104: 4c 8d 05 25 6e 75 02        	leaq	0x2756e25(%rip), %r8    # 0x1429a0f30
14024a10b: ba 09 00 00 00              	movl	$0x9, %edx
14024a110: e8 03 f4 68 02              	callq	0x1428d9518 <.text+0x28d8518>
14024a115: 4c 8d 05 8c 6e 75 02        	leaq	0x2756e8c(%rip), %r8    # 0x1429a0fa8
14024a11c: ba 04 00 00 00              	movl	$0x4, %edx
14024a121: e8 f2 f3 68 02              	callq	0x1428d9518 <.text+0x28d8518>
14024a126: e9 a2 00 00 00              	jmp	0x14024a1cd <.text+0x2491cd>
14024a12b: 4c 8d 05 46 67 75 02        	leaq	0x2756746(%rip), %r8    # 0x1429a0878
14024a132: b9 04 00 00 00              	movl	$0x4, %ecx
14024a137: 48 89 da                    	movq	%rbx, %rdx
14024a13a: e8 a4 ee 68 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14024a13f: e9 89 00 00 00              	jmp	0x14024a1cd <.text+0x2491cd>
14024a144: 4c 8d 85 20 02 00 00        	leaq	0x220(%rbp), %r8
14024a14b: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
14024a152: 4c 89 f2                    	movq	%r14, %rdx
14024a155: e8 26 27 90 00              	callq	0x140b4c880 <.text+0xb4b880>
14024a15a: e9 8b fe ff ff              	jmp	0x140249fea <.text+0x248fea>
14024a15f: 4c 8d 05 12 67 75 02        	leaq	0x2756712(%rip), %r8    # 0x1429a0878
14024a166: b9 08 00 00 00              	movl	$0x8, %ecx
14024a16b: 48 89 da                    	movq	%rbx, %rdx
14024a16e: e8 70 ee 68 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14024a173: 4c 8d 05 ee 6b 75 02        	leaq	0x2756bee(%rip), %r8    # 0x1429a0d68
14024a17a: e8 99 f3 68 02              	callq	0x1428d9518 <.text+0x28d8518>
14024a17f: eb 4c                       	jmp	0x14024a1cd <.text+0x2491cd>
14024a181: 4c 8d 05 f8 6b 75 02        	leaq	0x2756bf8(%rip), %r8    # 0x1429a0d80
14024a188: e8 8b f3 68 02              	callq	0x1428d9518 <.text+0x28d8518>
14024a18d: eb 3e                       	jmp	0x14024a1cd <.text+0x2491cd>
14024a18f: 48 8d 0d 32 6e 75 02        	leaq	0x2756e32(%rip), %rcx   # 0x1429a0fc8
14024a196: e8 75 f1 68 02              	callq	0x1428d9310 <.text+0x28d8310>
14024a19b: eb 30                       	jmp	0x14024a1cd <.text+0x2491cd>
14024a19d: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
14024a1a4: 4c 8d 85 20 02 00 00        	leaq	0x220(%rbp), %r8
14024a1ab: 4c 89 f9                    	movq	%r15, %rcx
14024a1ae: 4c 89 ea                    	movq	%r13, %rdx
14024a1b1: e8 fa 91 2f 00              	callq	0x1405433b0 <.text+0x5423b0>
14024a1b6: e9 cc fe ff ff              	jmp	0x14024a087 <.text+0x249087>
