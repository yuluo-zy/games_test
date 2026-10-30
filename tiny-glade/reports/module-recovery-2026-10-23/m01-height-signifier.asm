
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140956cd0: 55                          	pushq	%rbp
140956cd1: 41 57                       	pushq	%r15
140956cd3: 41 56                       	pushq	%r14
140956cd5: 41 55                       	pushq	%r13
140956cd7: 41 54                       	pushq	%r12
140956cd9: 56                          	pushq	%rsi
140956cda: 57                          	pushq	%rdi
140956cdb: 53                          	pushq	%rbx
140956cdc: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
140956ce3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140956ceb: 48 c7 45 60 fe ff ff ff     	movq	$-0x2, 0x60(%rbp)
140956cf3: 48 8b b5 d0 00 00 00        	movq	0xd0(%rbp), %rsi
140956cfa: 44 89 4d 04                 	movl	%r9d, 0x4(%rbp)
140956cfe: 80 79 09 01                 	cmpb	$0x1, 0x9(%rcx)
140956d02: 0f 85 f5 00 00 00           	jne	0x140956dfd <.text+0x955dfd>
140956d08: 44 0f b6 71 08              	movzbl	0x8(%rcx), %r14d
140956d0d: 45 84 f6                    	testb	%r14b, %r14b
140956d10: 0f 84 73 01 00 00           	je	0x140956e89 <.text+0x955e89>
140956d16: 4c 8b 39                    	movq	(%rcx), %r15
140956d19: 48 89 4d c0                 	movq	%rcx, -0x40(%rbp)
140956d1d: 48 8d 05 9c e1 1d 00        	leaq	0x1de19c(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
140956d24: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140956d28: 48 8d 05 f1 da 15 02        	leaq	0x215daf1(%rip), %rax   # 0x142ab4820
140956d2f: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140956d33: 48 c7 45 28 01 00 00 00     	movq	$0x1, 0x28(%rbp)
140956d3b: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
140956d3f: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140956d43: 48 c7 45 38 01 00 00 00     	movq	$0x1, 0x38(%rbp)
140956d4b: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140956d53: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
140956d57: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140956d5b: e8 30 32 42 00              	callq	0x140d79f90 <.text+0xd78f90>
140956d60: 48 8b 7d d8                 	movq	-0x28(%rbp), %rdi
140956d64: 48 8b 5d e0                 	movq	-0x20(%rbp), %rbx
140956d68: 0f b6 05 9a de cd 02        	movzbl	0x2cdde9a(%rip), %eax   # 0x143634c09
140956d6f: b9 20 00 00 00              	movl	$0x20, %ecx
140956d74: ba 08 00 00 00              	movl	$0x8, %edx
140956d79: e8 92 ce cb ff              	callq	0x140613c10 <.text+0x612c10>
140956d7e: 48 85 c0                    	testq	%rax, %rax
140956d81: 0f 84 f7 03 00 00           	je	0x14095717e <.text+0x95617e>
140956d87: 49 89 c5                    	movq	%rax, %r13
140956d8a: 0f 57 c0                    	xorps	%xmm0, %xmm0
140956d8d: 45 84 f6                    	testb	%r14b, %r14b
140956d90: 75 08                       	jne	0x140956d9a <.text+0x955d9a>
140956d92: f3 0f 10 05 ce 62 07 02     	movss	0x20762ce(%rip), %xmm0  # 0x1429cd068
140956d9a: 49 c7 45 00 01 00 00 00     	movq	$0x1, (%r13)
140956da2: 49 c7 45 08 01 00 00 00     	movq	$0x1, 0x8(%r13)
140956daa: 4d 89 7d 10                 	movq	%r15, 0x10(%r13)
140956dae: f3 41 0f 11 45 18           	movss	%xmm0, 0x18(%r13)
140956db4: 4c 89 6d 20                 	movq	%r13, 0x20(%rbp)
140956db8: 4c 8d 25 71 da 15 02        	leaq	0x215da71(%rip), %r12   # 0x142ab4830
140956dbf: 4c 89 65 28                 	movq	%r12, 0x28(%rbp)
140956dc3: 48 85 db                    	testq	%rbx, %rbx
140956dc6: 0f 88 c3 03 00 00           	js	0x14095718f <.text+0x95618f>
140956dcc: 0f 84 0c 03 00 00           	je	0x1409570de <.text+0x9560de>
140956dd2: 0f b6 05 30 de cd 02        	movzbl	0x2cdde30(%rip), %eax   # 0x143634c09
140956dd9: 41 bf 01 00 00 00           	movl	$0x1, %r15d
140956ddf: ba 01 00 00 00              	movl	$0x1, %edx
140956de4: 48 89 d9                    	movq	%rbx, %rcx
140956de7: e8 24 ce cb ff              	callq	0x140613c10 <.text+0x612c10>
140956dec: 48 85 c0                    	testq	%rax, %rax
140956def: 0f 84 9d 03 00 00           	je	0x140957192 <.text+0x956192>
140956df5: 49 89 c6                    	movq	%rax, %r14
140956df8: e9 e7 02 00 00              	jmp	0x1409570e4 <.text+0x9560e4>
140956dfd: 4c 8b aa 98 00 00 00        	movq	0x98(%rdx), %r13
140956e04: 48 8b 82 e8 00 00 00        	movq	0xe8(%rdx), %rax
140956e0b: 4c 39 e8                    	cmpq	%r13, %rax
140956e0e: 4c 0f 42 e8                 	cmovbq	%rax, %r13
140956e12: 4a 8d 04 ad 00 00 00 00     	leaq	(,%r13,4), %rax
140956e1a: 4c 8d 24 40                 	leaq	(%rax,%rax,2), %r12
140956e1e: 48 b8 aa aa aa aa aa aa aa 0a       	movabsq	$0xaaaaaaaaaaaaaaa, %rax # imm = 0xAAAAAAAAAAAAAAA
140956e28: 49 39 c5                    	cmpq	%rax, %r13
140956e2b: 0f 87 0d 03 00 00           	ja	0x14095713e <.text+0x95613e>
140956e31: 4d 85 ed                    	testq	%r13, %r13
140956e34: 4c 89 6d 08                 	movq	%r13, 0x8(%rbp)
140956e38: 4c 89 65 f8                 	movq	%r12, -0x8(%rbp)
140956e3c: 74 75                       	je	0x140956eb3 <.text+0x955eb3>
140956e3e: 49 89 cf                    	movq	%rcx, %r15
140956e41: 48 8b 9a 90 00 00 00        	movq	0x90(%rdx), %rbx
140956e48: 4c 8b b2 e0 00 00 00        	movq	0xe0(%rdx), %r14
140956e4f: 0f b6 05 b3 dd cd 02        	movzbl	0x2cdddb3(%rip), %eax   # 0x143634c09
140956e56: bf 04 00 00 00              	movl	$0x4, %edi
140956e5b: ba 04 00 00 00              	movl	$0x4, %edx
140956e60: 4c 89 e1                    	movq	%r12, %rcx
140956e63: e8 a8 cd cb ff              	callq	0x140613c10 <.text+0x612c10>
140956e68: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140956e6c: 48 85 c0                    	testq	%rax, %rax
140956e6f: 0f 84 cb 02 00 00           	je	0x140957140 <.text+0x956140>
140956e75: 49 83 fd 01                 	cmpq	$0x1, %r13
140956e79: 75 46                       	jne	0x140956ec1 <.text+0x955ec1>
140956e7b: 31 c0                       	xorl	%eax, %eax
140956e7d: 4c 89 f9                    	movq	%r15, %rcx
140956e80: 4c 8b 4d 50                 	movq	0x50(%rbp), %r9
140956e84: e9 cd 00 00 00              	jmp	0x140956f56 <.text+0x955f56>
140956e89: 49 8b 00                    	movq	(%r8), %rax
140956e8c: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
140956e90: 0f 84 93 02 00 00           	je	0x140957129 <.text+0x956129>
140956e96: 48 8b 50 38                 	movq	0x38(%rax), %rdx
140956e9a: 49 3b 50 08                 	cmpq	0x8(%r8), %rdx
140956e9e: 0f 85 85 02 00 00           	jne	0x140957129 <.text+0x956129>
140956ea4: 83 78 28 02                 	cmpl	$0x2, 0x28(%rax)
140956ea8: 0f 85 68 fe ff ff           	jne	0x140956d16 <.text+0x955d16>
140956eae: e9 76 02 00 00              	jmp	0x140957129 <.text+0x956129>
140956eb3: b8 04 00 00 00              	movl	$0x4, %eax
140956eb8: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140956ebc: e9 d4 00 00 00              	jmp	0x140956f95 <.text+0x955f95>
140956ec1: 48 ba fe ff ff ff ff ff ff 0f       	movabsq	$0xffffffffffffffe, %rdx # imm = 0xFFFFFFFFFFFFFFE
140956ecb: 4c 21 ea                    	andq	%r13, %rdx
140956ece: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140956ed4: 31 c0                       	xorl	%eax, %eax
140956ed6: f3 0f 10 05 2a ea fc 01     	movss	0x1fcea2a(%rip), %xmm0  # 0x142925908
140956ede: 4c 89 f9                    	movq	%r15, %rcx
140956ee1: 4c 8b 4d 50                 	movq	0x50(%rbp), %r9
140956ee5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140956ef0: f3 42 0f 10 4c 83 ec        	movss	-0x14(%rbx,%r8,4), %xmm1
140956ef7: f3 42 0f 10 54 83 f4        	movss	-0xc(%rbx,%r8,4), %xmm2
140956efe: f3 41 0f 10 1c 86           	movss	(%r14,%rax,4), %xmm3
140956f04: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140956f08: f3 43 0f 11 4c 81 ec        	movss	%xmm1, -0x14(%r9,%r8,4)
140956f0f: f3 43 0f 11 5c 81 f0        	movss	%xmm3, -0x10(%r9,%r8,4)
140956f16: f3 43 0f 11 54 81 f4        	movss	%xmm2, -0xc(%r9,%r8,4)
140956f1d: f3 42 0f 10 4c 83 f8        	movss	-0x8(%rbx,%r8,4), %xmm1
140956f24: f3 42 0f 10 14 83           	movss	(%rbx,%r8,4), %xmm2
140956f2a: f3 41 0f 10 5c 86 04        	movss	0x4(%r14,%rax,4), %xmm3
140956f31: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140956f35: 48 83 c0 02                 	addq	$0x2, %rax
140956f39: f3 43 0f 11 4c 81 f8        	movss	%xmm1, -0x8(%r9,%r8,4)
140956f40: f3 43 0f 11 5c 81 fc        	movss	%xmm3, -0x4(%r9,%r8,4)
140956f47: f3 43 0f 11 14 81           	movss	%xmm2, (%r9,%r8,4)
140956f4d: 49 83 c0 06                 	addq	$0x6, %r8
140956f51: 48 39 c2                    	cmpq	%rax, %rdx
140956f54: 75 9a                       	jne	0x140956ef0 <.text+0x955ef0>
140956f56: 41 f6 c5 01                 	testb	$0x1, %r13b
140956f5a: 74 39                       	je	0x140956f95 <.text+0x955f95>
140956f5c: 48 8d 14 85 00 00 00 00     	leaq	(,%rax,4), %rdx
140956f64: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140956f68: f3 0f 10 04 13              	movss	(%rbx,%rdx), %xmm0
140956f6d: f3 41 0f 10 0c 86           	movss	(%r14,%rax,4), %xmm1
140956f73: f3 0f 58 0d 8d e9 fc 01     	addss	0x1fce98d(%rip), %xmm1  # 0x142925908
140956f7b: f3 0f 10 54 13 08           	movss	0x8(%rbx,%rdx), %xmm2
140956f81: f3 41 0f 11 04 11           	movss	%xmm0, (%r9,%rdx)
140956f87: f3 41 0f 11 4c 11 04        	movss	%xmm1, 0x4(%r9,%rdx)
140956f8e: f3 41 0f 11 54 11 08        	movss	%xmm2, 0x8(%r9,%rdx)
140956f95: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140956f99: 48 8d 05 20 df 1d 00        	leaq	0x1ddf20(%rip), %rax    # 0x140b34ec0 <.text+0xb33ec0>
140956fa0: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140956fa4: 48 8d 45 04                 	leaq	0x4(%rbp), %rax
140956fa8: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140956fac: 48 8d 05 ad 65 43 00        	leaq	0x4365ad(%rip), %rax    # 0x140d8d560 <.text+0xd8c560>
140956fb3: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140956fb7: 48 8d 05 12 d8 15 02        	leaq	0x215d812(%rip), %rax   # 0x142ab47d0
140956fbe: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140956fc2: 48 c7 45 28 02 00 00 00     	movq	$0x2, 0x28(%rbp)
140956fca: 48 8d 45 d0                 	leaq	-0x30(%rbp), %rax
140956fce: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140956fd2: 48 c7 45 38 02 00 00 00     	movq	$0x2, 0x38(%rbp)
140956fda: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
140956fe2: c6 45 5f 01                 	movb	$0x1, 0x5f(%rbp)
140956fe6: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140956fea: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
140956fee: e8 9d 2f 42 00              	callq	0x140d79f90 <.text+0xd78f90>
140956ff3: 4c 8b 65 a8                 	movq	-0x58(%rbp), %r12
140956ff7: 4c 8b 6d b0                 	movq	-0x50(%rbp), %r13
140956ffb: 48 8b 7d b8                 	movq	-0x48(%rbp), %rdi
140956fff: 0f b6 05 03 dc cd 02        	movzbl	0x2cddc03(%rip), %eax   # 0x143634c09
140957006: b9 28 00 00 00              	movl	$0x28, %ecx
14095700b: ba 08 00 00 00              	movl	$0x8, %edx
140957010: e8 fb cb cb ff              	callq	0x140613c10 <.text+0x612c10>
140957015: 48 85 c0                    	testq	%rax, %rax
140957018: 4c 89 65 10                 	movq	%r12, 0x10(%rbp)
14095701c: 4c 89 6d f0                 	movq	%r13, -0x10(%rbp)
140957020: 0f 84 2c 01 00 00           	je	0x140957152 <.text+0x956152>
140957026: 49 89 c6                    	movq	%rax, %r14
140957029: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140957030: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140957038: 48 8b 45 08                 	movq	0x8(%rbp), %rax
14095703c: 49 89 46 10                 	movq	%rax, 0x10(%r14)
140957040: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140957044: 49 89 4e 18                 	movq	%rcx, 0x18(%r14)
140957048: 49 89 46 20                 	movq	%rax, 0x20(%r14)
14095704c: 4c 89 75 20                 	movq	%r14, 0x20(%rbp)
140957050: 4c 8d 3d 99 d7 15 02        	leaq	0x215d799(%rip), %r15   # 0x142ab47f0
140957057: 4c 89 7d 28                 	movq	%r15, 0x28(%rbp)
14095705b: 48 85 ff                    	testq	%rdi, %rdi
14095705e: 0f 88 ff 00 00 00           	js	0x140957163 <.text+0x956163>
140957064: 74 30                       	je	0x140957096 <.text+0x956096>
140957066: 4c 89 75 18                 	movq	%r14, 0x18(%rbp)
14095706a: 0f b6 05 98 db cd 02        	movzbl	0x2cddb98(%rip), %eax   # 0x143634c09
140957071: 41 be 01 00 00 00           	movl	$0x1, %r14d
140957077: ba 01 00 00 00              	movl	$0x1, %edx
14095707c: 48 89 f9                    	movq	%rdi, %rcx
14095707f: e8 8c cb cb ff              	callq	0x140613c10 <.text+0x612c10>
140957084: 48 85 c0                    	testq	%rax, %rax
140957087: 0f 84 dd 00 00 00           	je	0x14095716a <.text+0x95616a>
14095708d: 48 89 c3                    	movq	%rax, %rbx
140957090: 4c 8b 75 18                 	movq	0x18(%rbp), %r14
140957094: eb 05                       	jmp	0x14095709b <.text+0x95609b>
140957096: bb 01 00 00 00              	movl	$0x1, %ebx
14095709b: 48 89 d9                    	movq	%rbx, %rcx
14095709e: 4c 89 ea                    	movq	%r13, %rdx
1409570a1: 49 89 f8                    	movq	%rdi, %r8
1409570a4: e8 57 cb fc 01              	callq	0x142923c00 <.text+0x2922c00>
1409570a9: 48 89 7d 20                 	movq	%rdi, 0x20(%rbp)
1409570ad: 48 89 5d 28                 	movq	%rbx, 0x28(%rbp)
1409570b1: 48 89 7d 30                 	movq	%rdi, 0x30(%rbp)
1409570b5: 4c 89 75 38                 	movq	%r14, 0x38(%rbp)
1409570b9: 4c 89 7d 40                 	movq	%r15, 0x40(%rbp)
1409570bd: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
1409570c1: 48 89 f1                    	movq	%rsi, %rcx
1409570c4: e8 b7 57 b1 00              	callq	0x14146c880 <.text+0x146b880>
1409570c9: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
1409570cd: 48 85 d2                    	testq	%rdx, %rdx
1409570d0: 74 57                       	je	0x140957129 <.text+0x956129>
1409570d2: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409570d8: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1409570dc: eb 46                       	jmp	0x140957124 <.text+0x956124>
1409570de: 41 be 01 00 00 00           	movl	$0x1, %r14d
1409570e4: 4c 89 f1                    	movq	%r14, %rcx
1409570e7: 48 89 fa                    	movq	%rdi, %rdx
1409570ea: 49 89 d8                    	movq	%rbx, %r8
1409570ed: e8 0e cb fc 01              	callq	0x142923c00 <.text+0x2922c00>
1409570f2: 48 89 5d 20                 	movq	%rbx, 0x20(%rbp)
1409570f6: 4c 89 75 28                 	movq	%r14, 0x28(%rbp)
1409570fa: 48 89 5d 30                 	movq	%rbx, 0x30(%rbp)
1409570fe: 4c 89 6d 38                 	movq	%r13, 0x38(%rbp)
140957102: 4c 89 65 40                 	movq	%r12, 0x40(%rbp)
140957106: 48 8d 55 20                 	leaq	0x20(%rbp), %rdx
14095710a: 48 89 f1                    	movq	%rsi, %rcx
14095710d: e8 6e 57 b1 00              	callq	0x14146c880 <.text+0x146b880>
140957112: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
140957116: 48 85 d2                    	testq	%rdx, %rdx
140957119: 74 0e                       	je	0x140957129 <.text+0x956129>
14095711b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140957121: 48 89 f9                    	movq	%rdi, %rcx
140957124: e8 f7 ca cb ff              	callq	0x140613c20 <.text+0x612c20>
140957129: 90                          	nop
14095712a: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
140957131: 5b                          	popq	%rbx
140957132: 5f                          	popq	%rdi
140957133: 5e                          	popq	%rsi
140957134: 41 5c                       	popq	%r12
140957136: 41 5d                       	popq	%r13
140957138: 41 5e                       	popq	%r14
14095713a: 41 5f                       	popq	%r15
14095713c: 5d                          	popq	%rbp
14095713d: c3                          	retq
14095713e: 31 ff                       	xorl	%edi, %edi
140957140: 4c 8d 05 d1 cb 15 02        	leaq	0x215cbd1(%rip), %r8    # 0x142ab3d18
140957147: 48 89 f9                    	movq	%rdi, %rcx
14095714a: 4c 89 e2                    	movq	%r12, %rdx
14095714d: e8 91 1e f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140957152: b9 08 00 00 00              	movl	$0x8, %ecx
140957157: ba 28 00 00 00              	movl	$0x28, %edx
14095715c: e8 9f 1e f8 01              	callq	0x1428d9000 <.text+0x28d8000>
140957161: eb 45                       	jmp	0x1409571a8 <.text+0x9561a8>
140957163: 4c 89 75 18                 	movq	%r14, 0x18(%rbp)
140957167: 45 31 f6                    	xorl	%r14d, %r14d
14095716a: 4c 8d 05 af 1d 14 02        	leaq	0x2141daf(%rip), %r8    # 0x142a98f20
140957171: 4c 89 f1                    	movq	%r14, %rcx
140957174: 48 89 fa                    	movq	%rdi, %rdx
140957177: e8 67 1e f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14095717c: eb 2a                       	jmp	0x1409571a8 <.text+0x9561a8>
14095717e: b9 08 00 00 00              	movl	$0x8, %ecx
140957183: ba 20 00 00 00              	movl	$0x20, %edx
140957188: e8 73 1e f8 01              	callq	0x1428d9000 <.text+0x28d8000>
14095718d: eb 19                       	jmp	0x1409571a8 <.text+0x9561a8>
14095718f: 45 31 ff                    	xorl	%r15d, %r15d
140957192: 4c 89 6d 50                 	movq	%r13, 0x50(%rbp)
140957196: 4c 8d 05 83 1d 14 02        	leaq	0x2141d83(%rip), %r8    # 0x142a98f20
14095719d: 4c 89 f9                    	movq	%r15, %rcx
1409571a0: 48 89 da                    	movq	%rbx, %rdx
1409571a3: e8 3b 1e f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409571a8: 0f 0b                       	ud2
1409571aa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1409571b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409571b5: 55                          	pushq	%rbp
1409571b6: 41 57                       	pushq	%r15
1409571b8: 41 56                       	pushq	%r14
1409571ba: 41 55                       	pushq	%r13
1409571bc: 41 54                       	pushq	%r12
1409571be: 56                          	pushq	%rsi
1409571bf: 57                          	pushq	%rdi
1409571c0: 53                          	pushq	%rbx
1409571c1: 48 83 ec 28                 	subq	$0x28, %rsp
1409571c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409571cc: 48 83 7d 10 00              	cmpq	$0x0, 0x10(%rbp)
1409571d1: 74 13                       	je	0x1409571e6 <.text+0x9561e6>
1409571d3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409571d9: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
1409571dd: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
1409571e1: e8 3a ca cb ff              	callq	0x140613c20 <.text+0x612c20>
1409571e6: c6 45 5f 00                 	movb	$0x0, 0x5f(%rbp)
1409571ea: 48 83 c4 28                 	addq	$0x28, %rsp
1409571ee: 5b                          	popq	%rbx
1409571ef: 5f                          	popq	%rdi
1409571f0: 5e                          	popq	%rsi
1409571f1: 41 5c                       	popq	%r12
1409571f3: 41 5d                       	popq	%r13
1409571f5: 41 5e                       	popq	%r14
1409571f7: 41 5f                       	popq	%r15
1409571f9: 5d                          	popq	%rbp
1409571fa: c3                          	retq
1409571fb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140957200: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140957205: 55                          	pushq	%rbp
140957206: 41 57                       	pushq	%r15
140957208: 41 56                       	pushq	%r14
14095720a: 41 55                       	pushq	%r13
14095720c: 41 54                       	pushq	%r12
14095720e: 56                          	pushq	%rsi
14095720f: 57                          	pushq	%rdi
140957210: 53                          	pushq	%rbx
140957211: 48 83 ec 28                 	subq	$0x28, %rsp
140957215: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095721c: 48 83 7d 08 00              	cmpq	$0x0, 0x8(%rbp)
140957221: 74 13                       	je	0x140957236 <.text+0x956236>
140957223: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140957229: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14095722d: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140957231: e8 ea c9 cb ff              	callq	0x140613c20 <.text+0x612c20>
140957236: 90                          	nop
140957237: 48 83 c4 28                 	addq	$0x28, %rsp
14095723b: 5b                          	popq	%rbx
14095723c: 5f                          	popq	%rdi
14095723d: 5e                          	popq	%rsi
14095723e: 41 5c                       	popq	%r12
140957240: 41 5d                       	popq	%r13
140957242: 41 5e                       	popq	%r14
140957244: 41 5f                       	popq	%r15
140957246: 5d                          	popq	%rbp
140957247: c3                          	retq
140957248: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140957250: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140957255: 55                          	pushq	%rbp
140957256: 41 57                       	pushq	%r15
140957258: 41 56                       	pushq	%r14
14095725a: 41 55                       	pushq	%r13
14095725c: 41 54                       	pushq	%r12
14095725e: 56                          	pushq	%rsi
14095725f: 57                          	pushq	%rdi
140957260: 53                          	pushq	%rbx
140957261: 48 83 ec 28                 	subq	$0x28, %rsp
140957265: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095726c: 48 8b 45 18                 	movq	0x18(%rbp), %rax
140957270: f0                          	lock
140957271: 48 ff 08                    	decq	(%rax)
140957274: 75 09                       	jne	0x14095727f <.text+0x95627f>
140957276: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14095727a: e8 51 87 cf ff              	callq	0x14064f9d0 <.text+0x64e9d0>
14095727f: 90                          	nop
140957280: 48 83 c4 28                 	addq	$0x28, %rsp
140957284: 5b                          	popq	%rbx
140957285: 5f                          	popq	%rdi
140957286: 5e                          	popq	%rsi
140957287: 41 5c                       	popq	%r12
140957289: 41 5d                       	popq	%r13
14095728b: 41 5e                       	popq	%r14
14095728d: 41 5f                       	popq	%r15
14095728f: 5d                          	popq	%rbp
140957290: c3                          	retq
140957291: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409572a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409572a5: 55                          	pushq	%rbp
1409572a6: 41 57                       	pushq	%r15
1409572a8: 41 56                       	pushq	%r14
1409572aa: 41 55                       	pushq	%r13
1409572ac: 41 54                       	pushq	%r12
1409572ae: 56                          	pushq	%rsi
1409572af: 57                          	pushq	%rdi
1409572b0: 53                          	pushq	%rbx
1409572b1: 48 83 ec 28                 	subq	$0x28, %rsp
1409572b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409572bc: 80 7d 5f 00                 	cmpb	$0x0, 0x5f(%rbp)
1409572c0: 74 1a                       	je	0x1409572dc <.text+0x9562dc>
1409572c2: 48 83 7d 08 00              	cmpq	$0x0, 0x8(%rbp)
1409572c7: 74 13                       	je	0x1409572dc <.text+0x9562dc>
1409572c9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409572cf: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1409572d3: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
1409572d7: e8 44 c9 cb ff              	callq	0x140613c20 <.text+0x612c20>
1409572dc: 90                          	nop
1409572dd: 48 83 c4 28                 	addq	$0x28, %rsp
1409572e1: 5b                          	popq	%rbx
1409572e2: 5f                          	popq	%rdi
1409572e3: 5e                          	popq	%rsi
1409572e4: 41 5c                       	popq	%r12
1409572e6: 41 5d                       	popq	%r13
1409572e8: 41 5e                       	popq	%r14
1409572ea: 41 5f                       	popq	%r15
1409572ec: 5d                          	popq	%rbp
1409572ed: c3                          	retq
1409572ee: 66 90                       	nop
1409572f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409572f5: 55                          	pushq	%rbp
1409572f6: 41 57                       	pushq	%r15
1409572f8: 41 56                       	pushq	%r14
1409572fa: 41 55                       	pushq	%r13
1409572fc: 41 54                       	pushq	%r12
1409572fe: 56                          	pushq	%rsi
1409572ff: 57                          	pushq	%rdi
140957300: 53                          	pushq	%rbx
140957301: 48 83 ec 28                 	subq	$0x28, %rsp
140957305: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095730c: 48 8b 55 d0                 	movq	-0x30(%rbp), %rdx
140957310: 48 85 d2                    	testq	%rdx, %rdx
140957313: 74 0f                       	je	0x140957324 <.text+0x956324>
140957315: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
140957319: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14095731f: e8 fc c8 cb ff              	callq	0x140613c20 <.text+0x612c20>
140957324: 90                          	nop
140957325: 48 83 c4 28                 	addq	$0x28, %rsp
140957329: 5b                          	popq	%rbx
14095732a: 5f                          	popq	%rdi
14095732b: 5e                          	popq	%rsi
14095732c: 41 5c                       	popq	%r12
14095732e: 41 5d                       	popq	%r13
140957330: 41 5e                       	popq	%r14
140957332: 41 5f                       	popq	%r15
140957334: 5d                          	popq	%rbp
140957335: c3                          	retq
140957336: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140957340: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140957345: 55                          	pushq	%rbp
140957346: 41 57                       	pushq	%r15
140957348: 41 56                       	pushq	%r14
14095734a: 41 55                       	pushq	%r13
14095734c: 41 54                       	pushq	%r12
14095734e: 56                          	pushq	%rsi
14095734f: 57                          	pushq	%rdi
140957350: 53                          	pushq	%rbx
140957351: 48 83 ec 28                 	subq	$0x28, %rsp
140957355: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095735c: 48 8b 45 50                 	movq	0x50(%rbp), %rax
140957360: f0                          	lock
140957361: 48 ff 08                    	decq	(%rax)
140957364: 75 09                       	jne	0x14095736f <.text+0x95636f>
140957366: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14095736a: e8 61 86 cf ff              	callq	0x14064f9d0 <.text+0x64e9d0>
14095736f: 90                          	nop
140957370: 48 83 c4 28                 	addq	$0x28, %rsp
140957374: 5b                          	popq	%rbx
140957375: 5f                          	popq	%rdi
140957376: 5e                          	popq	%rsi
140957377: 41 5c                       	popq	%r12
140957379: 41 5d                       	popq	%r13
14095737b: 41 5e                       	popq	%r14
14095737d: 41 5f                       	popq	%r15
14095737f: 5d                          	popq	%rbp
140957380: c3                          	retq
140957381: cc                          	int3
140957382: cc                          	int3
140957383: cc                          	int3
140957384: cc                          	int3
140957385: cc                          	int3
140957386: cc                          	int3
140957387: cc                          	int3
140957388: cc                          	int3
140957389: cc                          	int3
14095738a: cc                          	int3
14095738b: cc                          	int3
14095738c: cc                          	int3
14095738d: cc                          	int3
14095738e: cc                          	int3
14095738f: cc                          	int3
