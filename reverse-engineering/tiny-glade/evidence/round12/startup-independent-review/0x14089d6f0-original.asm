
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14089d6f0: 55                          	pushq	%rbp
14089d6f1: 56                          	pushq	%rsi
14089d6f2: 48 83 ec 78                 	subq	$0x78, %rsp
14089d6f6: 48 8d 6c 24 70              	leaq	0x70(%rsp), %rbp
14089d6fb: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14089d703: 48 89 ce                    	movq	%rcx, %rsi
14089d706: 48 8d 15 33 be 1f 02        	leaq	0x21fbe33(%rip), %rdx   # 0x142a99540
14089d70d: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14089d711: 41 b8 17 00 00 00           	movl	$0x17, %r8d
14089d717: e8 24 ba 4b 00              	callq	0x140d59140 <.text+0xd58140>
14089d71c: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
14089d720: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14089d724: 48 f7 d8                    	negq	%rax
14089d727: 0f 80 91 00 00 00           	jo	0x14089d7be <.text+0x89c7be>
14089d72d: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
14089d731: 4c 8b 45 d8                 	movq	-0x28(%rbp), %r8
14089d735: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14089d739: 48 89 55 c0                 	movq	%rdx, -0x40(%rbp)
14089d73d: e8 9e 7a b6 00              	callq	0x1414051e0 <.text+0x14041e0>
14089d742: 31 c0                       	xorl	%eax, %eax
14089d744: 48 3b 45 c8                 	cmpq	-0x38(%rbp), %rax
14089d748: 0f 80 a2 00 00 00           	jo	0x14089d7f0 <.text+0x89c7f0>
14089d74e: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14089d752: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14089d756: 0f 10 45 c8                 	movups	-0x38(%rbp), %xmm0
14089d75a: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14089d75e: 8b 46 1c                    	movl	0x1c(%rsi), %eax
14089d761: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
14089d765: 89 01                       	movl	%eax, (%rcx)
14089d767: 48 8b 36                    	movq	(%rsi), %rsi
14089d76a: 48 8b 06                    	movq	(%rsi), %rax
14089d76d: 48 85 c0                    	testq	%rax, %rax
14089d770: 74 17                       	je	0x14089d789 <.text+0x89c789>
14089d772: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
14089d776: 48 c1 e0 04                 	shlq	$0x4, %rax
14089d77a: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
14089d77e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14089d784: e8 97 64 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089d789: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14089d78d: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14089d791: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
14089d795: 0f 11 06                    	movups	%xmm0, (%rsi)
14089d798: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
14089d79c: 48 85 d2                    	testq	%rdx, %rdx
14089d79f: 74 15                       	je	0x14089d7b6 <.text+0x89c7b6>
14089d7a1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14089d7a7: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
14089d7ab: 48 83 c4 78                 	addq	$0x78, %rsp
14089d7af: 5e                          	popq	%rsi
14089d7b0: 5d                          	popq	%rbp
14089d7b1: e9 6a 64 d7 ff              	jmp	0x140613c20 <.text+0x612c20>
14089d7b6: 90                          	nop
14089d7b7: 48 83 c4 78                 	addq	$0x78, %rsp
14089d7bb: 5e                          	popq	%rsi
14089d7bc: 5d                          	popq	%rbp
14089d7bd: c3                          	retq
14089d7be: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
14089d7c2: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14089d7c6: 48 8d 05 8b bd 1f 02        	leaq	0x21fbd8b(%rip), %rax   # 0x142a99558
14089d7cd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14089d7d2: 48 8d 0d a7 b6 1f 02        	leaq	0x21fb6a7(%rip), %rcx   # 0x142a98e80
14089d7d9: 4c 8d 0d d0 b6 1f 02        	leaq	0x21fb6d0(%rip), %r9    # 0x142a98eb0
14089d7e0: 4c 8d 45 e0                 	leaq	-0x20(%rbp), %r8
14089d7e4: ba 2b 00 00 00              	movl	$0x2b, %edx
14089d7e9: e8 72 bf 03 02              	callq	0x1428d9760 <.text+0x28d8760>
14089d7ee: eb 30                       	jmp	0x14089d820 <.text+0x89c820>
14089d7f0: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
14089d7f4: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14089d7f8: 48 8d 05 29 bd 1f 02        	leaq	0x21fbd29(%rip), %rax   # 0x142a99528
14089d7ff: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14089d804: 48 8d 0d 75 b6 1f 02        	leaq	0x21fb675(%rip), %rcx   # 0x142a98e80
14089d80b: 4c 8d 0d 4e b6 1f 02        	leaq	0x21fb64e(%rip), %r9    # 0x142a98e60
14089d812: 4c 8d 45 e0                 	leaq	-0x20(%rbp), %r8
14089d816: ba 2b 00 00 00              	movl	$0x2b, %edx
14089d81b: e8 40 bf 03 02              	callq	0x1428d9760 <.text+0x28d8760>
14089d820: 0f 0b                       	ud2
14089d822: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14089d830: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089d835: 55                          	pushq	%rbp
14089d836: 56                          	pushq	%rsi
14089d837: 48 83 ec 28                 	subq	$0x28, %rsp
14089d83b: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14089d83f: 48 83 7d f8 00              	cmpq	$0x0, -0x8(%rbp)
14089d844: 74 13                       	je	0x14089d859 <.text+0x89c859>
14089d846: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14089d84c: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
14089d850: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
14089d854: e8 c7 63 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089d859: 90                          	nop
14089d85a: 48 83 c4 28                 	addq	$0x28, %rsp
14089d85e: 5e                          	popq	%rsi
14089d85f: 5d                          	popq	%rbp
14089d860: c3                          	retq
14089d861: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14089d870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089d875: 55                          	pushq	%rbp
14089d876: 56                          	pushq	%rsi
14089d877: 48 83 ec 28                 	subq	$0x28, %rsp
14089d87b: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14089d87f: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14089d883: e8 78 a1 d4 ff              	callq	0x1405e7a00 <.text+0x5e6a00>
14089d888: 90                          	nop
14089d889: 48 83 c4 28                 	addq	$0x28, %rsp
14089d88d: 5e                          	popq	%rsi
14089d88e: 5d                          	popq	%rbp
14089d88f: c3                          	retq
14089d890: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089d895: 55                          	pushq	%rbp
14089d896: 56                          	pushq	%rsi
14089d897: 48 83 ec 28                 	subq	$0x28, %rsp
14089d89b: 48 8d 6a 70                 	leaq	0x70(%rdx), %rbp
14089d89f: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14089d8a3: e8 f8 af d4 ff              	callq	0x1405e88a0 <.text+0x5e78a0>
14089d8a8: 90                          	nop
14089d8a9: 48 83 c4 28                 	addq	$0x28, %rsp
14089d8ad: 5e                          	popq	%rsi
14089d8ae: 5d                          	popq	%rbp
14089d8af: c3                          	retq
