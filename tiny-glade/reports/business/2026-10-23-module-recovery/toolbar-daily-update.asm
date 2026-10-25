
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403d1750: 55                          	pushq	%rbp
1403d1751: 41 57                       	pushq	%r15
1403d1753: 41 56                       	pushq	%r14
1403d1755: 41 55                       	pushq	%r13
1403d1757: 41 54                       	pushq	%r12
1403d1759: 56                          	pushq	%rsi
1403d175a: 57                          	pushq	%rdi
1403d175b: 53                          	pushq	%rbx
1403d175c: 48 81 ec e8 02 00 00        	subq	$0x2e8, %rsp            # imm = 0x2E8
1403d1763: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403d176b: 48 c7 85 60 02 00 00 fe ff ff ff    	movq	$-0x2, 0x260(%rbp)
1403d1776: 48 89 d6                    	movq	%rdx, %rsi
1403d1779: 48 8b 39                    	movq	(%rcx), %rdi
1403d177c: b1 01                       	movb	$0x1, %cl
1403d177e: 31 c0                       	xorl	%eax, %eax
1403d1780: f0                          	lock
1403d1781: 0f b0 0f                    	cmpxchgb	%cl, (%rdi)
1403d1784: 0f 85 59 0a 00 00           	jne	0x1403d21e3 <.text+0x3d11e3>
1403d178a: 48 89 f9                    	movq	%rdi, %rcx
1403d178d: e8 de 04 8b 00              	callq	0x140c81c70 <.text+0xc80c70>
1403d1792: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403d1796: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403d179a: 48 85 c0                    	testq	%rax, %rax
1403d179d: 48 89 bd 48 02 00 00        	movq	%rdi, 0x248(%rbp)
1403d17a4: 74 13                       	je	0x1403d17b9 <.text+0x3d07b9>
1403d17a6: 83 f8 01                    	cmpl	$0x1, %eax
1403d17a9: 75 1c                       	jne	0x1403d17c7 <.text+0x3d07c7>
1403d17ab: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d17b2: e8 49 5a e9 ff              	callq	0x140267200 <.text+0x266200>
1403d17b7: eb 1a                       	jmp	0x1403d17d3 <.text+0x3d07d3>
1403d17b9: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d17c0: e8 bb 0b 12 00              	callq	0x1404f2380 <.text+0x4f1380>
1403d17c5: eb 0c                       	jmp	0x1403d17d3 <.text+0x3d07d3>
1403d17c7: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d17ce: e8 4d e2 14 00              	callq	0x14051fa20 <.text+0x51ea20>
1403d17d3: 49 bc 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r12 # imm = 0x8000000000000001
1403d17dd: 4c 39 a5 90 01 00 00        	cmpq	%r12, 0x190(%rbp)
1403d17e4: 75 2c                       	jne	0x1403d1812 <.text+0x3d0812>
1403d17e6: 48 8b b5 48 02 00 00        	movq	0x248(%rbp), %rsi
1403d17ed: 48 89 f1                    	movq	%rsi, %rcx
1403d17f0: e8 1b 06 8b 00              	callq	0x140c81e10 <.text+0xc80e10>
1403d17f5: 31 c9                       	xorl	%ecx, %ecx
1403d17f7: b0 01                       	movb	$0x1, %al
1403d17f9: f0                          	lock
1403d17fa: 0f b0 0e                    	cmpxchgb	%cl, (%rsi)
1403d17fd: 0f 84 61 08 00 00           	je	0x1403d2064 <.text+0x3d1064>
1403d1803: 48 89 f1                    	movq	%rsi, %rcx
1403d1806: 31 d2                       	xorl	%edx, %edx
1403d1808: e8 83 c3 4f 02              	callq	0x1428cdb90 <.text+0x28ccb90>
1403d180d: e9 52 08 00 00              	jmp	0x1403d2064 <.text+0x3d1064>
1403d1812: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
1403d1819: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
1403d1820: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
1403d1827: 0f 29 95 b0 00 00 00        	movaps	%xmm2, 0xb0(%rbp)
1403d182e: 0f 29 8d a0 00 00 00        	movaps	%xmm1, 0xa0(%rbp)
1403d1835: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
1403d183c: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
1403d1843: 48 8b bd 48 02 00 00        	movq	0x248(%rbp), %rdi
1403d184a: 48 89 f9                    	movq	%rdi, %rcx
1403d184d: e8 be 05 8b 00              	callq	0x140c81e10 <.text+0xc80e10>
1403d1852: 31 c9                       	xorl	%ecx, %ecx
1403d1854: b0 01                       	movb	$0x1, %al
1403d1856: f0                          	lock
1403d1857: 0f b0 0f                    	cmpxchgb	%cl, (%rdi)
1403d185a: 0f 85 96 09 00 00           	jne	0x1403d21f6 <.text+0x3d11f6>
1403d1860: 8b 5e 1c                    	movl	0x1c(%rsi), %ebx
1403d1863: 4c 8b 76 10                 	movq	0x10(%rsi), %r14
1403d1867: 41 89 1e                    	movl	%ebx, (%r14)
1403d186a: 48 8b 0e                    	movq	(%rsi), %rcx
1403d186d: 31 c0                       	xorl	%eax, %eax
1403d186f: 48 3b 01                    	cmpq	(%rcx), %rax
1403d1872: 48 89 8d 48 02 00 00        	movq	%rcx, 0x248(%rbp)
1403d1879: 71 3d                       	jno	0x1403d18b8 <.text+0x3d08b8>
1403d187b: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
1403d187f: 48 b8 03 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
1403d1889: 48 39 c2                    	cmpq	%rax, %rdx
1403d188c: 0f 9c c0                    	setl	%al
1403d188f: 4c 39 e2                    	cmpq	%r12, %rdx
1403d1892: 41 0f 95 c0                 	setne	%r8b
1403d1896: 44 84 c0                    	testb	%r8b, %al
1403d1899: 75 22                       	jne	0x1403d18bd <.text+0x3d08bd>
1403d189b: 48 85 d2                    	testq	%rdx, %rdx
1403d189e: 74 1d                       	je	0x1403d18bd <.text+0x3d08bd>
1403d18a0: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1403d18a7: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1403d18ab: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d18b1: e8 6a 23 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d18b6: eb 05                       	jmp	0x1403d18bd <.text+0x3d08bd>
1403d18b8: e8 a3 8b 22 00              	callq	0x1405fa460 <.text+0x5f9460>
1403d18bd: 48 ba 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdx # imm = 0x8000000000000000
1403d18c7: 4c 8b ad 48 02 00 00        	movq	0x248(%rbp), %r13
1403d18ce: 49 89 55 00                 	movq	%rdx, (%r13)
1403d18d2: 49 89 55 08                 	movq	%rdx, 0x8(%r13)
1403d18d6: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1403d18dd: 48 8b bd 98 00 00 00        	movq	0x98(%rbp), %rdi
1403d18e4: 4c 8b bd a0 00 00 00        	movq	0xa0(%rbp), %r15
1403d18eb: 48 8b b5 a8 00 00 00        	movq	0xa8(%rbp), %rsi
1403d18f2: 8b 8d b0 00 00 00           	movl	0xb0(%rbp), %ecx
1403d18f8: 89 8d 2c 02 00 00           	movl	%ecx, 0x22c(%rbp)
1403d18fe: 8b 8d b4 00 00 00           	movl	0xb4(%rbp), %ecx
1403d1904: 89 8d 28 02 00 00           	movl	%ecx, 0x228(%rbp)
1403d190a: 8b 8d b8 00 00 00           	movl	0xb8(%rbp), %ecx
1403d1910: 89 8d 24 02 00 00           	movl	%ecx, 0x224(%rbp)
1403d1916: 8b 8d bc 00 00 00           	movl	0xbc(%rbp), %ecx
1403d191c: 89 8d 20 02 00 00           	movl	%ecx, 0x220(%rbp)
1403d1922: 48 89 c1                    	movq	%rax, %rcx
1403d1925: 48 f7 d9                    	negq	%rcx
1403d1928: 71 0c                       	jno	0x1403d1936 <.text+0x3d0936>
1403d192a: 48 89 95 30 02 00 00        	movq	%rdx, 0x230(%rbp)
1403d1931: e9 d8 05 00 00              	jmp	0x1403d1f0e <.text+0x3d0f0e>
1403d1936: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1403d193d: 48 89 bd 78 01 00 00        	movq	%rdi, 0x178(%rbp)
1403d1944: 4c 89 bd 80 01 00 00        	movq	%r15, 0x180(%rbp)
1403d194b: 48 89 b5 88 01 00 00        	movq	%rsi, 0x188(%rbp)
1403d1952: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1403d1959: e8 72 55 9a 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403d195e: 49 89 d0                    	movq	%rdx, %r8
1403d1961: 48 c7 44 24 20 0a 00 00 00  	movq	$0xa, 0x20(%rsp)
1403d196a: 4c 8d 0d 07 04 60 02        	leaq	0x2600407(%rip), %r9    # 0x1429d1d78
1403d1971: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1978: 48 89 c2                    	movq	%rax, %rdx
1403d197b: e8 90 0e 99 00              	callq	0x140d62810 <.text+0xd61810>
1403d1980: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1987: e8 44 55 9a 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403d198c: 49 89 d0                    	movq	%rdx, %r8
1403d198f: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1403d1996: 48 89 c2                    	movq	%rax, %rdx
1403d1999: e8 12 75 98 00              	callq	0x140d58eb0 <.text+0xd57eb0>
1403d199e: 48 8b 95 90 01 00 00        	movq	0x190(%rbp), %rdx
1403d19a5: 48 85 d2                    	testq	%rdx, %rdx
1403d19a8: 74 12                       	je	0x1403d19bc <.text+0x3d09bc>
1403d19aa: 48 8b 8d 98 01 00 00        	movq	0x198(%rbp), %rcx
1403d19b1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d19b7: e8 64 22 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d19bc: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
1403d19c3: 48 8b b5 48 01 00 00        	movq	0x148(%rbp), %rsi
1403d19ca: 48 89 c8                    	movq	%rcx, %rax
1403d19cd: 48 f7 d8                    	negq	%rax
1403d19d0: 0f 81 86 00 00 00           	jno	0x1403d1a5c <.text+0x3d0a5c>
1403d19d6: 48 89 b5 20 01 00 00        	movq	%rsi, 0x120(%rbp)
1403d19dd: c6 85 5e 02 00 00 01        	movb	$0x1, 0x25e(%rbp)
1403d19e4: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d19eb: e8 10 4d 98 00              	callq	0x140d56700 <.text+0xd55700>
1403d19f0: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
1403d19f7: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
1403d19fe: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
1403d1a05: 0f 29 95 e0 00 00 00        	movaps	%xmm2, 0xe0(%rbp)
1403d1a0c: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
1403d1a13: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
1403d1a1a: 48 8d 05 61 03 60 02        	leaq	0x2600361(%rip), %rax   # 0x1429d1d82
1403d1a21: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1403d1a28: 48 c7 85 98 01 00 00 12 00 00 00    	movq	$0x12, 0x198(%rbp)
1403d1a33: 48 89 b5 a0 01 00 00        	movq	%rsi, 0x1a0(%rbp)
1403d1a3a: c6 85 5e 02 00 00 00        	movb	$0x0, 0x25e(%rbp)
1403d1a41: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1a48: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
1403d1a4f: e8 dc 1a 4d 02              	callq	0x1428a3530 <.text+0x28a2530>
1403d1a54: 48 89 c6                    	movq	%rax, %rsi
1403d1a57: e9 d4 03 00 00              	jmp	0x1403d1e30 <.text+0x3d0e30>
1403d1a5c: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
1403d1a63: 4c 8b 85 50 01 00 00        	movq	0x150(%rbp), %r8
1403d1a6a: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1403d1a71: 48 89 b5 40 02 00 00        	movq	%rsi, 0x240(%rbp)
1403d1a78: 48 89 f2                    	movq	%rsi, %rdx
1403d1a7b: e8 30 56 c8 ff              	callq	0x1400570b0 <.text+0x560b0>
1403d1a80: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1403d1a87: 48 8b bd 08 02 00 00        	movq	0x208(%rbp), %rdi
1403d1a8e: 48 89 c8                    	movq	%rcx, %rax
1403d1a91: 48 f7 d8                    	negq	%rax
1403d1a94: 0f 81 a8 00 00 00           	jno	0x1403d1b42 <.text+0x3d0b42>
1403d1a9a: 48 89 bd 40 01 00 00        	movq	%rdi, 0x140(%rbp)
1403d1aa1: c6 85 5d 02 00 00 01        	movb	$0x1, 0x25d(%rbp)
1403d1aa8: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1aaf: e8 4c 4c 98 00              	callq	0x140d56700 <.text+0xd55700>
1403d1ab4: 0f 10 85 90 01 00 00        	movups	0x190(%rbp), %xmm0
1403d1abb: 0f 10 8d a0 01 00 00        	movups	0x1a0(%rbp), %xmm1
1403d1ac2: 0f 10 95 b0 01 00 00        	movups	0x1b0(%rbp), %xmm2
1403d1ac9: 0f 29 95 e0 00 00 00        	movaps	%xmm2, 0xe0(%rbp)
1403d1ad0: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
1403d1ad7: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
1403d1ade: 48 8d 05 af 02 60 02        	leaq	0x26002af(%rip), %rax   # 0x1429d1d94
1403d1ae5: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1403d1aec: 48 c7 85 98 01 00 00 12 00 00 00    	movq	$0x12, 0x198(%rbp)
1403d1af7: 48 89 bd a0 01 00 00        	movq	%rdi, 0x1a0(%rbp)
1403d1afe: c6 85 5d 02 00 00 00        	movb	$0x0, 0x25d(%rbp)
1403d1b05: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1b0c: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
1403d1b13: e8 38 19 4d 02              	callq	0x1428a3450 <.text+0x28a2450>
1403d1b18: 48 89 c6                    	movq	%rax, %rsi
1403d1b1b: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
1403d1b22: 48 85 d2                    	testq	%rdx, %rdx
1403d1b25: 0f 84 05 03 00 00           	je	0x1403d1e30 <.text+0x3d0e30>
1403d1b2b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1b31: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1403d1b38: e8 e3 20 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1b3d: e9 ee 02 00 00              	jmp	0x1403d1e30 <.text+0x3d0e30>
1403d1b42: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
1403d1b49: 48 89 8d 30 02 00 00        	movq	%rcx, 0x230(%rbp)
1403d1b50: 48 89 4d c8                 	movq	%rcx, -0x38(%rbp)
1403d1b54: 48 89 7d d0                 	movq	%rdi, -0x30(%rbp)
1403d1b58: 49 89 c7                    	movq	%rax, %r15
1403d1b5b: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
1403d1b5f: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
1403d1b66: 48 85 d2                    	testq	%rdx, %rdx
1403d1b69: 74 12                       	je	0x1403d1b7d <.text+0x3d0b7d>
1403d1b6b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1b71: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1403d1b78: e8 a3 20 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1b7d: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1403d1b84: e8 47 53 9a 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403d1b89: 49 89 d0                    	movq	%rdx, %r8
1403d1b8c: 48 c7 44 24 20 09 00 00 00  	movq	$0x9, 0x20(%rsp)
1403d1b95: 4c 8d 0d 0a 02 60 02        	leaq	0x260020a(%rip), %r9    # 0x1429d1da6
1403d1b9c: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1403d1ba3: 48 89 c2                    	movq	%rax, %rdx
1403d1ba6: e8 65 0c 99 00              	callq	0x140d62810 <.text+0xd61810>
1403d1bab: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1403d1bb2: e8 19 53 9a 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403d1bb7: 49 89 d0                    	movq	%rdx, %r8
1403d1bba: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1bc1: 48 89 c2                    	movq	%rax, %rdx
1403d1bc4: e8 b7 48 77 00              	callq	0x140b46480 <.text+0xb45480>
1403d1bc9: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
1403d1bd0: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1403d1bd7: 48 89 ca                    	movq	%rcx, %rdx
1403d1bda: 48 f7 da                    	negq	%rdx
1403d1bdd: 71 68                       	jno	0x1403d1c47 <.text+0x3d0c47>
1403d1bdf: 48 8d 0d c9 01 60 02        	leaq	0x26001c9(%rip), %rcx   # 0x1429d1daf
1403d1be6: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
1403d1bed: 48 c7 85 28 01 00 00 14 00 00 00    	movq	$0x14, 0x128(%rbp)
1403d1bf8: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1403d1bff: 48 c7 85 40 01 00 00 03 00 00 00    	movq	$0x3, 0x140(%rbp)
1403d1c0a: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
1403d1c11: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
1403d1c18: e8 03 20 4d 02              	callq	0x1428a3c20 <.text+0x28a2c20>
1403d1c1d: 48 89 c6                    	movq	%rax, %rsi
1403d1c20: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1403d1c27: 48 85 d2                    	testq	%rdx, %rdx
1403d1c2a: 0f 84 9b 01 00 00           	je	0x1403d1dcb <.text+0x3d0dcb>
1403d1c30: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1403d1c37: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1c3d: e8 de 1f 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1c42: e9 84 01 00 00              	jmp	0x1403d1dcb <.text+0x3d0dcb>
1403d1c47: 0f 10 85 e0 01 00 00        	movups	0x1e0(%rbp), %xmm0
1403d1c4e: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
1403d1c55: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1403d1c5c: 0f 10 8d b0 01 00 00        	movups	0x1b0(%rbp), %xmm1
1403d1c63: 0f 10 95 c0 01 00 00        	movups	0x1c0(%rbp), %xmm2
1403d1c6a: 0f 10 9d d0 01 00 00        	movups	0x1d0(%rbp), %xmm3
1403d1c71: 0f 11 9d 00 01 00 00        	movups	%xmm3, 0x100(%rbp)
1403d1c78: 0f 11 95 f0 00 00 00        	movups	%xmm2, 0xf0(%rbp)
1403d1c7f: 0f 11 8d e0 00 00 00        	movups	%xmm1, 0xe0(%rbp)
1403d1c86: 0f 11 85 d0 00 00 00        	movups	%xmm0, 0xd0(%rbp)
1403d1c8d: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
1403d1c94: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
1403d1c9b: 48 8b 95 00 02 00 00        	movq	0x200(%rbp), %rdx
1403d1ca2: 48 85 d2                    	testq	%rdx, %rdx
1403d1ca5: 74 12                       	je	0x1403d1cb9 <.text+0x3d0cb9>
1403d1ca7: 48 8b 8d 08 02 00 00        	movq	0x208(%rbp), %rcx
1403d1cae: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1cb4: e8 67 1f 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1cb9: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1403d1cc0: e8 0b 52 9a 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403d1cc5: 49 89 d0                    	movq	%rdx, %r8
1403d1cc8: 48 c7 44 24 20 0c 00 00 00  	movq	$0xc, 0x20(%rsp)
1403d1cd1: 4c 8d 0d eb 00 60 02        	leaq	0x26000eb(%rip), %r9    # 0x1429d1dc3
1403d1cd8: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1403d1cdf: 48 89 c2                    	movq	%rax, %rdx
1403d1ce2: e8 29 0b 99 00              	callq	0x140d62810 <.text+0xd61810>
1403d1ce7: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1403d1cee: e8 dd 51 9a 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403d1cf3: 48 89 c1                    	movq	%rax, %rcx
1403d1cf6: e8 f5 0f 99 00              	callq	0x140d62cf0 <.text+0xd61cf0>
1403d1cfb: 84 c0                       	testb	%al, %al
1403d1cfd: 74 65                       	je	0x1403d1d64 <.text+0x3d0d64>
1403d1cff: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
1403d1d06: e8 c5 51 9a 00              	callq	0x140d76ed0 <.text+0xd75ed0>
1403d1d0b: 49 89 d0                    	movq	%rdx, %r8
1403d1d0e: 48 c7 44 24 20 0c 00 00 00  	movq	$0xc, 0x20(%rsp)
1403d1d17: 4c 8d 0d da 00 60 02        	leaq	0x26000da(%rip), %r9    # 0x1429d1df8
1403d1d1e: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1d25: 48 89 c2                    	movq	%rax, %rdx
1403d1d28: e8 e3 0a 99 00              	callq	0x140d62810 <.text+0xd61810>
1403d1d2d: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1403d1d34: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1403d1d3b: e8 00 ea f3 ff              	callq	0x140310740 <.text+0x30f740>
1403d1d40: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
1403d1d47: 48 89 c8                    	movq	%rcx, %rax
1403d1d4a: 48 f7 d8                    	negq	%rax
1403d1d4d: 0f 81 25 03 00 00           	jno	0x1403d2078 <.text+0x3d1078>
1403d1d53: 48 8d 8d 08 02 00 00        	leaq	0x208(%rbp), %rcx
1403d1d5a: e8 a1 5c 21 00              	callq	0x1405e7a00 <.text+0x5e6a00>
1403d1d5f: e9 bc 03 00 00              	jmp	0x1403d2120 <.text+0x3d1120>
1403d1d64: 48 8d 05 7d 00 60 02        	leaq	0x260007d(%rip), %rax   # 0x1429d1de8
1403d1d6b: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1403d1d72: 48 c7 85 98 01 00 00 01 00 00 00    	movq	$0x1, 0x198(%rbp)
1403d1d7d: 48 c7 85 a0 01 00 00 08 00 00 00    	movq	$0x8, 0x1a0(%rbp)
1403d1d88: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403d1d8b: 0f 11 85 a8 01 00 00        	movups	%xmm0, 0x1a8(%rbp)
1403d1d92: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d1d99: e8 82 02 4d 02              	callq	0x1428a2020 <.text+0x28a1020>
1403d1d9e: 48 89 c6                    	movq	%rax, %rsi
1403d1da1: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
1403d1da8: 48 85 d2                    	testq	%rdx, %rdx
1403d1dab: 74 12                       	je	0x1403d1dbf <.text+0x3d0dbf>
1403d1dad: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1403d1db4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1dba: e8 61 1e 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1dbf: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
1403d1dc6: e8 05 97 d7 ff              	callq	0x14014b4d0 <.text+0x14a4d0>
1403d1dcb: 4d 85 ff                    	testq	%r15, %r15
1403d1dce: 74 34                       	je	0x1403d1e04 <.text+0x3d0e04>
1403d1dd0: 4c 8d 67 08                 	leaq	0x8(%rdi), %r12
1403d1dd4: eb 13                       	jmp	0x1403d1de9 <.text+0x3d0de9>
1403d1dd6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1403d1de0: 49 83 c4 18                 	addq	$0x18, %r12
1403d1de4: 49 ff cf                    	decq	%r15
1403d1de7: 74 1b                       	je	0x1403d1e04 <.text+0x3d0e04>
1403d1de9: 49 8b 54 24 f8              	movq	-0x8(%r12), %rdx
1403d1dee: 48 85 d2                    	testq	%rdx, %rdx
1403d1df1: 74 ed                       	je	0x1403d1de0 <.text+0x3d0de0>
1403d1df3: 49 8b 0c 24                 	movq	(%r12), %rcx
1403d1df7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1dfd: e8 1e 1e 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1e02: eb dc                       	jmp	0x1403d1de0 <.text+0x3d0de0>
1403d1e04: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1403d1e0b: 48 85 c0                    	testq	%rax, %rax
1403d1e0e: 74 16                       	je	0x1403d1e26 <.text+0x3d0e26>
1403d1e10: 48 c1 e0 03                 	shlq	$0x3, %rax
1403d1e14: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1403d1e18: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403d1e1e: 48 89 f9                    	movq	%rdi, %rcx
1403d1e21: e8 fa 1d 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1e26: 49 bc 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r12 # imm = 0x8000000000000001
1403d1e30: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1403d1e37: 48 85 d2                    	testq	%rdx, %rdx
1403d1e3a: 74 12                       	je	0x1403d1e4e <.text+0x3d0e4e>
1403d1e3c: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1403d1e43: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1e49: e8 d2 1d 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1e4e: 48 89 b5 40 01 00 00        	movq	%rsi, 0x140(%rbp)
1403d1e55: 48 c7 85 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rbp)
1403d1e60: 48 c7 85 c8 00 00 00 01 00 00 00    	movq	$0x1, 0xc8(%rbp)
1403d1e6b: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
1403d1e76: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
1403d1e81: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1403d1e8c: 48 c7 85 b0 01 00 00 20 00 00 00    	movq	$0x20, 0x1b0(%rbp)
1403d1e97: c6 85 b8 01 00 00 03        	movb	$0x3, 0x1b8(%rbp)
1403d1e9e: 48 8d 85 c0 00 00 00        	leaq	0xc0(%rbp), %rax
1403d1ea5: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1403d1eac: 48 8d 05 25 fa 5f 02        	leaq	0x25ffa25(%rip), %rax   # 0x1429d18d8
1403d1eb3: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
1403d1eba: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1403d1ec1: 48 8d 95 90 01 00 00        	leaq	0x190(%rbp), %rdx
1403d1ec8: e8 43 ad 97 00              	callq	0x140d4cc10 <.text+0xd4bc10>
1403d1ecd: 84 c0                       	testb	%al, %al
1403d1ecf: 0f 85 37 03 00 00           	jne	0x1403d220c <.text+0x3d120c>
1403d1ed5: 48 8b bd c0 00 00 00        	movq	0xc0(%rbp), %rdi
1403d1edc: 4c 8b bd c8 00 00 00        	movq	0xc8(%rbp), %r15
1403d1ee3: 48 8b b5 d0 00 00 00        	movq	0xd0(%rbp), %rsi
1403d1eea: c6 85 5f 02 00 00 00        	movb	$0x0, 0x25f(%rbp)
1403d1ef1: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1403d1ef8: e8 33 ad 97 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1403d1efd: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403d1f07: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1403d1f0e: 48 89 b5 38 01 00 00        	movq	%rsi, 0x138(%rbp)
1403d1f15: 4c 89 bd 40 02 00 00        	movq	%r15, 0x240(%rbp)
1403d1f1c: 48 89 bd 50 02 00 00        	movq	%rdi, 0x250(%rbp)
1403d1f23: 41 89 1e                    	movl	%ebx, (%r14)
1403d1f26: 31 c0                       	xorl	%eax, %eax
1403d1f28: 49 3b 45 00                 	cmpq	(%r13), %rax
1403d1f2c: 71 3b                       	jno	0x1403d1f69 <.text+0x3d0f69>
1403d1f2e: 49 8b 55 08                 	movq	0x8(%r13), %rdx
1403d1f32: 48 b8 03 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffd, %rax # imm = 0x8000000000000003
1403d1f3c: 48 39 c2                    	cmpq	%rax, %rdx
1403d1f3f: 0f 9c c0                    	setl	%al
1403d1f42: 4c 39 e2                    	cmpq	%r12, %rdx
1403d1f45: 0f 95 c1                    	setne	%cl
1403d1f48: 84 c8                       	testb	%cl, %al
1403d1f4a: 75 25                       	jne	0x1403d1f71 <.text+0x3d0f71>
1403d1f4c: 48 85 d2                    	testq	%rdx, %rdx
1403d1f4f: 74 20                       	je	0x1403d1f71 <.text+0x3d0f71>
1403d1f51: 48 8b 85 48 02 00 00        	movq	0x248(%rbp), %rax
1403d1f58: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1403d1f5c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d1f62: e8 b9 1c 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d1f67: eb 08                       	jmp	0x1403d1f71 <.text+0x3d0f71>
1403d1f69: 4c 89 e9                    	movq	%r13, %rcx
1403d1f6c: e8 ef 84 22 00              	callq	0x1405fa460 <.text+0x5f9460>
1403d1f71: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1403d1f78: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1403d1f7f: 48 89 01                    	movq	%rax, (%rcx)
1403d1f82: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1403d1f89: 48 89 41 08                 	movq	%rax, 0x8(%rcx)
1403d1f8d: 48 8b 85 40 02 00 00        	movq	0x240(%rbp), %rax
1403d1f94: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
1403d1f98: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
1403d1f9f: 48 89 41 18                 	movq	%rax, 0x18(%rcx)
1403d1fa3: 0f 28 45 30                 	movaps	0x30(%rbp), %xmm0
1403d1fa7: 0f 28 4d 40                 	movaps	0x40(%rbp), %xmm1
1403d1fab: 0f 28 55 50                 	movaps	0x50(%rbp), %xmm2
1403d1faf: 0f 28 5d 60                 	movaps	0x60(%rbp), %xmm3
1403d1fb3: 0f 11 41 20                 	movups	%xmm0, 0x20(%rcx)
1403d1fb7: 0f 11 49 30                 	movups	%xmm1, 0x30(%rcx)
1403d1fbb: 0f 11 51 40                 	movups	%xmm2, 0x40(%rcx)
1403d1fbf: 0f 11 59 50                 	movups	%xmm3, 0x50(%rcx)
1403d1fc3: 0f 28 45 70                 	movaps	0x70(%rbp), %xmm0
1403d1fc7: 0f 11 41 60                 	movups	%xmm0, 0x60(%rcx)
1403d1fcb: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1403d1fd2: 48 89 41 70                 	movq	%rax, 0x70(%rcx)
1403d1fd6: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
1403d1fda: 0f 28 4d 20                 	movaps	0x20(%rbp), %xmm1
1403d1fde: 0f 11 41 78                 	movups	%xmm0, 0x78(%rcx)
1403d1fe2: 0f 11 89 88 00 00 00        	movups	%xmm1, 0x88(%rcx)
1403d1fe9: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
1403d1ff0: 48 89 81 98 00 00 00        	movq	%rax, 0x98(%rcx)
1403d1ff7: 0f 28 45 e0                 	movaps	-0x20(%rbp), %xmm0
1403d1ffb: 0f 11 81 a0 00 00 00        	movups	%xmm0, 0xa0(%rcx)
1403d2002: 48 8b 85 f8 01 00 00        	movq	0x1f8(%rbp), %rax
1403d2009: 48 89 81 b0 00 00 00        	movq	%rax, 0xb0(%rcx)
1403d2010: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
1403d2014: 0f 11 81 b8 00 00 00        	movups	%xmm0, 0xb8(%rcx)
1403d201b: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1403d2022: 48 89 81 c8 00 00 00        	movq	%rax, 0xc8(%rcx)
1403d2029: 0f 28 45 00                 	movaps	(%rbp), %xmm0
1403d202d: 0f 11 81 d0 00 00 00        	movups	%xmm0, 0xd0(%rcx)
1403d2034: 8b 85 2c 02 00 00           	movl	0x22c(%rbp), %eax
1403d203a: 89 81 e0 00 00 00           	movl	%eax, 0xe0(%rcx)
1403d2040: 8b 85 28 02 00 00           	movl	0x228(%rbp), %eax
1403d2046: 89 81 e4 00 00 00           	movl	%eax, 0xe4(%rcx)
1403d204c: 8b 85 24 02 00 00           	movl	0x224(%rbp), %eax
1403d2052: 89 81 e8 00 00 00           	movl	%eax, 0xe8(%rcx)
1403d2058: 8b 85 20 02 00 00           	movl	0x220(%rbp), %eax
1403d205e: 89 81 ec 00 00 00           	movl	%eax, 0xec(%rcx)
1403d2064: 48 81 c4 e8 02 00 00        	addq	$0x2e8, %rsp            # imm = 0x2E8
1403d206b: 5b                          	popq	%rbx
1403d206c: 5f                          	popq	%rdi
1403d206d: 5e                          	popq	%rsi
1403d206e: 41 5c                       	popq	%r12
1403d2070: 41 5d                       	popq	%r13
1403d2072: 41 5e                       	popq	%r14
1403d2074: 41 5f                       	popq	%r15
1403d2076: 5d                          	popq	%rbp
1403d2077: c3                          	retq
1403d2078: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
1403d207f: 48 8b 95 08 02 00 00        	movq	0x208(%rbp), %rdx
1403d2086: 4c 8b 85 10 02 00 00        	movq	0x210(%rbp), %r8
1403d208d: 48 8d 8d 90 01 00 00        	leaq	0x190(%rbp), %rcx
1403d2094: 48 89 95 40 02 00 00        	movq	%rdx, 0x240(%rbp)
1403d209b: e8 80 6d c8 ff              	callq	0x140058e20 <.text+0x57e20>
1403d20a0: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
1403d20a7: 48 8d 8d 98 01 00 00        	leaq	0x198(%rbp), %rcx
1403d20ae: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
1403d20b5: 4c 39 e0                    	cmpq	%r12, %rax
1403d20b8: 75 07                       	jne	0x1403d20c1 <.text+0x3d10c1>
1403d20ba: e8 e1 67 21 00              	callq	0x1405e88a0 <.text+0x5e78a0>
1403d20bf: eb 31                       	jmp	0x1403d20f2 <.text+0x3d10f2>
1403d20c1: 0f 10 01                    	movups	(%rcx), %xmm0
1403d20c4: 0f 29 85 00 02 00 00        	movaps	%xmm0, 0x200(%rbp)
1403d20cb: 48 8b b5 a8 01 00 00        	movq	0x1a8(%rbp), %rsi
1403d20d2: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1403d20d9: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
1403d20e0: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1403d20e7: 0f 10 85 c8 01 00 00        	movups	0x1c8(%rbp), %xmm0
1403d20ee: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1403d20f2: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
1403d20f9: 48 85 d2                    	testq	%rdx, %rdx
1403d20fc: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1403d2103: 74 12                       	je	0x1403d2117 <.text+0x3d1117>
1403d2105: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d210b: 48 8b 8d 40 02 00 00        	movq	0x240(%rbp), %rcx
1403d2112: e8 09 1b 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d2117: 4c 39 a5 38 02 00 00        	cmpq	%r12, 0x238(%rbp)
1403d211e: 75 21                       	jne	0x1403d2141 <.text+0x3d1141>
1403d2120: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403d212a: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
1403d2131: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
1403d2138: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1403d213f: eb 25                       	jmp	0x1403d2166 <.text+0x3d1166>
1403d2141: 48 89 b5 f8 01 00 00        	movq	%rsi, 0x1f8(%rbp)
1403d2148: 0f 28 85 00 02 00 00        	movaps	0x200(%rbp), %xmm0
1403d214f: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1403d2153: 0f 28 85 20 01 00 00        	movaps	0x120(%rbp), %xmm0
1403d215a: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1403d215e: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
1403d2162: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
1403d2166: 48 8d 85 c8 00 00 00        	leaq	0xc8(%rbp), %rax
1403d216d: 48 8b b5 c0 00 00 00        	movq	0xc0(%rbp), %rsi
1403d2174: 0f 10 00                    	movups	(%rax), %xmm0
1403d2177: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1403d217b: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1403d217f: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
1403d2183: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
1403d2187: 0f 29 4d 40                 	movaps	%xmm1, 0x40(%rbp)
1403d218b: 0f 29 55 50                 	movaps	%xmm2, 0x50(%rbp)
1403d218f: 0f 29 5d 60                 	movaps	%xmm3, 0x60(%rbp)
1403d2193: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
1403d2197: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
1403d219b: 48 8b 40 50                 	movq	0x50(%rax), %rax
1403d219f: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
1403d21a6: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1403d21ad: 0f 10 8d 50 01 00 00        	movups	0x150(%rbp), %xmm1
1403d21b4: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1403d21b8: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
1403d21bc: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1403d21c3: 48 85 d2                    	testq	%rdx, %rdx
1403d21c6: 0f 84 42 fd ff ff           	je	0x1403d1f0e <.text+0x3d0f0e>
1403d21cc: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1403d21d3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d21d9: e8 42 1a 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d21de: e9 2b fd ff ff              	jmp	0x1403d1f0e <.text+0x3d0f0e>
1403d21e3: 48 89 f9                    	movq	%rdi, %rcx
1403d21e6: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
1403d21ec: e8 1f b0 4f 02              	callq	0x1428cd210 <.text+0x28cc210>
1403d21f1: e9 94 f5 ff ff              	jmp	0x1403d178a <.text+0x3d078a>
1403d21f6: c6 85 5f 02 00 00 01        	movb	$0x1, 0x25f(%rbp)
1403d21fd: 48 89 f9                    	movq	%rdi, %rcx
1403d2200: 31 d2                       	xorl	%edx, %edx
1403d2202: e8 89 b9 4f 02              	callq	0x1428cdb90 <.text+0x28ccb90>
1403d2207: e9 54 f6 ff ff              	jmp	0x1403d1860 <.text+0x3d0860>
1403d220c: 48 8d 05 7d f7 5f 02        	leaq	0x25ff77d(%rip), %rax   # 0x1429d1990
1403d2213: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403d2218: 48 8d 0d e9 f6 5f 02        	leaq	0x25ff6e9(%rip), %rcx   # 0x1429d1908
1403d221f: 4c 8d 0d a2 f7 5f 02        	leaq	0x25ff7a2(%rip), %r9    # 0x1429d19c8
1403d2226: 4c 8d 85 00 02 00 00        	leaq	0x200(%rbp), %r8
1403d222d: ba 37 00 00 00              	movl	$0x37, %edx
1403d2232: e8 29 75 50 02              	callq	0x1428d9760 <.text+0x28d8760>
1403d2237: 0f 0b                       	ud2
1403d2239: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403d2240: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403d2245: 55                          	pushq	%rbp
1403d2246: 41 57                       	pushq	%r15
1403d2248: 41 56                       	pushq	%r14
1403d224a: 41 55                       	pushq	%r13
1403d224c: 41 54                       	pushq	%r12
1403d224e: 56                          	pushq	%rsi
1403d224f: 57                          	pushq	%rdi
1403d2250: 53                          	pushq	%rbx
1403d2251: 48 83 ec 28                 	subq	$0x28, %rsp
1403d2255: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403d225c: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
1403d2263: 48 85 d2                    	testq	%rdx, %rdx
1403d2266: 74 12                       	je	0x1403d227a <.text+0x3d127a>
1403d2268: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
1403d226f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403d2275: e8 a6 19 24 00              	callq	0x140613c20 <.text+0x612c20>
1403d227a: c6 85 5f 02 00 00 00        	movb	$0x0, 0x25f(%rbp)
1403d2281: 48 83 c4 28                 	addq	$0x28, %rsp
1403d2285: 5b                          	popq	%rbx
1403d2286: 5f                          	popq	%rdi
1403d2287: 5e                          	popq	%rsi
1403d2288: 41 5c                       	popq	%r12
1403d228a: 41 5d                       	popq	%r13
1403d228c: 41 5e                       	popq	%r14
1403d228e: 41 5f                       	popq	%r15
1403d2290: 5d                          	popq	%rbp
1403d2291: c3                          	retq
1403d2292: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1403d22a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403d22a5: 55                          	pushq	%rbp
1403d22a6: 41 57                       	pushq	%r15
1403d22a8: 41 56                       	pushq	%r14
1403d22aa: 41 55                       	pushq	%r13
1403d22ac: 41 54                       	pushq	%r12
1403d22ae: 56                          	pushq	%rsi
1403d22af: 57                          	pushq	%rdi
1403d22b0: 53                          	pushq	%rbx
1403d22b1: 48 83 ec 28                 	subq	$0x28, %rsp
1403d22b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403d22bc: 48 8b 95 90 01 00 00        	movq	0x190(%rbp), %rdx
1403d22c3: 48 85 d2                    	testq	%rdx, %rdx
1403d22c6: 74 12                       	je	0x1403d22da <.text+0x3d12da>
1403d22c8: 48 8b 8d 98 01 00 00        	movq	0x198(%rbp), %rcx
1403d22cf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
