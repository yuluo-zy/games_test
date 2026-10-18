141698b83: 48 8b 85 d8 01 00 00        	movq	0x1d8(%rbp), %rax
141698b8a: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
141698b91: 48 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %rax
141698b98: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
141698b9f: 80 bd 78 01 00 00 00        	cmpb	$0x0, 0x178(%rbp)
141698ba6: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698bad: 0f 84 3e 0c 00 00           	je	0x1416997f1 <.text+0x16987f1>
141698bb3: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
141698bba: 48 85 c0                    	testq	%rax, %rax
141698bbd: 0f 84 0f 05 00 00           	je	0x1416990d2 <.text+0x16980d2>
141698bc3: 48 8b bd f8 02 00 00        	movq	0x2f8(%rbp), %rdi
141698bca: 48 8d 34 c7                 	leaq	(%rdi,%rax,8), %rsi
141698bce: eb 3c                       	jmp	0x141698c0c <.text+0x1697c0c>
141698bd0: f0                          	lock
141698bd1: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698bd8: 49 8b 0f                    	movq	(%r15), %rcx
141698bdb: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698bdf: 44 89 74 24 28              	movl	%r14d, 0x28(%rsp)
141698be4: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698bec: 48 8b 95 d8 02 00 00        	movq	0x2d8(%rbp), %rdx
141698bf3: 41 89 d8                    	movl	%ebx, %r8d
141698bf6: 45 31 c9                    	xorl	%r9d, %r9d
141698bf9: ff 90 48 03 00 00           	callq	*0x348(%rax)
141698bff: 48 83 c7 08                 	addq	$0x8, %rdi
141698c03: 48 39 f7                    	cmpq	%rsi, %rdi
141698c06: 0f 84 c6 04 00 00           	je	0x1416990d2 <.text+0x16980d2>
141698c0c: 44 8b 37                    	movl	(%rdi), %r14d
141698c0f: 8b 5f 04                    	movl	0x4(%rdi), %ebx
141698c12: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c16: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698c1d: 75 b1                       	jne	0x141698bd0 <.text+0x1697bd0>
141698c1f: 49 8b 0f                    	movq	(%r15), %rcx
141698c22: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698c26: 48 89 c1                    	movq	%rax, %rcx
141698c29: e8 62 54 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698c2e: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c32: eb 9c                       	jmp	0x141698bd0 <.text+0x1697bd0>
141698c34: 8b 9d d4 01 00 00           	movl	0x1d4(%rbp), %ebx
141698c3a: 44 8b b5 d8 01 00 00        	movl	0x1d8(%rbp), %r14d
141698c41: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698c48: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c4c: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698c53: 75 2f                       	jne	0x141698c84 <.text+0x1697c84>
141698c55: 49 8b 0f                    	movq	(%r15), %rcx
141698c58: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698c5c: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698c63: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698c6a: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698c71: 48 89 c1                    	movq	%rax, %rcx
141698c74: e8 17 54 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698c79: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698c80: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698c84: f0                          	lock
141698c85: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698c8c: 49 8b 0f                    	movq	(%r15), %rcx
141698c8f: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698c93: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698c9b: 89 da                       	movl	%ebx, %edx
141698c9d: 45 89 f0                    	movl	%r14d, %r8d
141698ca0: 45 31 c9                    	xorl	%r9d, %r9d
141698ca3: ff 90 40 03 00 00           	callq	*0x340(%rax)
141698ca9: e9 88 00 00 00              	jmp	0x141698d36 <.text+0x1697d36>
141698cae: 80 bd 78 01 00 00 00        	cmpb	$0x0, 0x178(%rbp)
141698cb5: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698cbc: 0f 84 4c 0b 00 00           	je	0x14169980e <.text+0x169880e>
141698cc2: 8b 9d d4 01 00 00           	movl	0x1d4(%rbp), %ebx
141698cc8: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698ccc: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698cd3: 75 2f                       	jne	0x141698d04 <.text+0x1697d04>
141698cd5: 49 8b 0f                    	movq	(%r15), %rcx
141698cd8: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698cdc: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698ce3: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698cea: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698cf1: 48 89 c1                    	movq	%rax, %rcx
141698cf4: e8 97 53 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698cf9: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
141698d00: 49 8b 47 08                 	movq	0x8(%r15), %rax
141698d04: f0                          	lock
141698d05: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698d0c: 49 8b 0f                    	movq	(%r15), %rcx
141698d0f: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698d13: c7 44 24 28 00 00 00 00     	movl	$0x0, 0x28(%rsp)
141698d1b: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698d23: 48 8b 95 d8 02 00 00        	movq	0x2d8(%rbp), %rdx
141698d2a: 41 89 d8                    	movl	%ebx, %r8d
141698d2d: 45 31 c9                    	xorl	%r9d, %r9d
141698d30: ff 90 48 03 00 00           	callq	*0x348(%rax)
141698d36: 40 b6 01                    	movb	$0x1, %sil
141698d39: b0 01                       	movb	$0x1, %al
141698d3b: e9 a5 05 00 00              	jmp	0x1416992e5 <.text+0x16982e5>
141698d40: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141698d47: 48 8b 9d d8 01 00 00        	movq	0x1d8(%rbp), %rbx
141698d4e: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
141698d55: 48 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %rsi
141698d5c: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698d63: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698d67: 41 80 bf b9 0a 00 00 00     	cmpb	$0x0, 0xab9(%r15)
141698d6f: 74 2f                       	je	0x141698da0 <.text+0x1697da0>
141698d71: 48 8b 01                    	movq	(%rcx), %rax
141698d74: 48 8b 50 48                 	movq	0x48(%rax), %rdx
141698d78: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698d7f: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698d86: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698d8d: 4c 89 f9                    	movq	%r15, %rcx
141698d90: e8 fb 52 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698d95: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698d9c: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698da0: f0                          	lock
141698da1: 49 ff 87 a8 0a 00 00        	incq	0xaa8(%r15)
141698da8: 48 8b 01                    	movq	(%rcx), %rax
141698dab: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141698daf: 4c 8b 60 48                 	movq	0x48(%rax), %r12
141698db3: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698dba: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698dc1: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698dc8: 44 89 ea                    	movl	%r13d, %edx
141698dcb: e8 00 ec 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141698dd0: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698dd7: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698dde: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698de5: 4c 89 f9                    	movq	%r15, %rcx
141698de8: 48 89 c2                    	movq	%rax, %rdx
141698deb: 45 31 c0                    	xorl	%r8d, %r8d
141698dee: e8 7d 66 55 ff              	callq	0x140bef470 <.text+0xbee470>
141698df3: a8 01                       	testb	$0x1, %al
141698df5: 0f 84 88 0a 00 00           	je	0x141699883 <.text+0x1698883>
141698dfb: 89 74 24 20                 	movl	%esi, 0x20(%rsp)
141698dff: 4c 89 e1                    	movq	%r12, %rcx
141698e02: 49 89 d8                    	movq	%rbx, %r8
141698e05: 45 89 f1                    	movl	%r14d, %r9d
141698e08: 41 ff 97 58 03 00 00        	callq	*0x358(%r15)
141698e0f: e9 c5 04 00 00              	jmp	0x1416992d9 <.text+0x16982d9>
141698e14: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
141698e1b: 48 8b 41 10                 	movq	0x10(%rcx), %rax
141698e1f: 48 89 85 60 02 00 00        	movq	%rax, 0x260(%rbp)
141698e26: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
141698e2a: 66 0f 7f 85 50 02 00 00     	movdqa	%xmm0, 0x250(%rbp)
141698e32: 8b 9d d4 01 00 00           	movl	0x1d4(%rbp), %ebx
141698e38: 48 8b b5 08 03 00 00        	movq	0x308(%rbp), %rsi
141698e3f: 48 89 f1                    	movq	%rsi, %rcx
141698e42: 48 8d 95 50 02 00 00        	leaq	0x250(%rbp), %rdx
141698e49: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141698e4f: e8 9c eb 05 00              	callq	0x1416f79f0 <.text+0x16f69f0>
141698e54: 48 8b 46 08                 	movq	0x8(%rsi), %rax
141698e58: 80 b8 b9 0a 00 00 01        	cmpb	$0x1, 0xab9(%rax)
141698e5f: 75 13                       	jne	0x141698e74 <.text+0x1697e74>
141698e61: 48 8b 0e                    	movq	(%rsi), %rcx
141698e64: 48 8b 51 48                 	movq	0x48(%rcx), %rdx
141698e68: 48 89 c1                    	movq	%rax, %rcx
141698e6b: e8 20 52 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698e70: 48 8b 46 08                 	movq	0x8(%rsi), %rax
141698e74: f0                          	lock
141698e75: 48 ff 80 a8 0a 00 00        	incq	0xaa8(%rax)
141698e7c: 48 8b 0e                    	movq	(%rsi), %rcx
141698e7f: 48 8b 49 48                 	movq	0x48(%rcx), %rcx
141698e83: c7 44 24 28 00 00 00 00     	movl	$0x0, 0x28(%rsp)
141698e8b: c7 44 24 20 00 00 00 00     	movl	$0x0, 0x20(%rsp)
141698e93: 89 da                       	movl	%ebx, %edx
141698e95: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141698e9b: 45 31 c9                    	xorl	%r9d, %r9d
141698e9e: ff 90 48 03 00 00           	callq	*0x348(%rax)
141698ea4: c6 85 17 03 00 00 00        	movb	$0x0, 0x317(%rbp)
141698eab: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698eb2: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698eb9: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
141698ec0: e8 eb 95 54 ff              	callq	0x140be24b0 <.text+0xbe14b0>
141698ec5: b0 01                       	movb	$0x1, %al
141698ec7: 31 f6                       	xorl	%esi, %esi
141698ec9: e9 10 04 00 00              	jmp	0x1416992de <.text+0x16982de>
141698ece: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141698ed5: 48 8b 9d d8 01 00 00        	movq	0x1d8(%rbp), %rbx
141698edc: 4c 8b b5 e0 01 00 00        	movq	0x1e0(%rbp), %r14
141698ee3: 48 8b b5 e8 01 00 00        	movq	0x1e8(%rbp), %rsi
141698eea: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698ef1: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698ef5: 41 80 bf b9 0a 00 00 00     	cmpb	$0x0, 0xab9(%r15)
141698efd: 74 2f                       	je	0x141698f2e <.text+0x1697f2e>
141698eff: 48 8b 01                    	movq	(%rcx), %rax
141698f02: 48 8b 50 48                 	movq	0x48(%rax), %rdx
141698f06: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698f0d: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698f14: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698f1b: 4c 89 f9                    	movq	%r15, %rcx
141698f1e: e8 6d 51 55 ff              	callq	0x140bee090 <.text+0xbed090>
141698f23: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141698f2a: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
141698f2e: f0                          	lock
141698f2f: 49 ff 87 a8 0a 00 00        	incq	0xaa8(%r15)
141698f36: 48 8b 01                    	movq	(%rcx), %rax
141698f39: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141698f3d: 4c 8b 60 48                 	movq	0x48(%rax), %r12
141698f41: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698f48: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698f4f: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698f56: 44 89 ea                    	movl	%r13d, %edx
141698f59: e8 72 ea 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141698f5e: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141698f65: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141698f6c: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141698f73: 4c 89 f9                    	movq	%r15, %rcx
141698f76: 48 89 c2                    	movq	%rax, %rdx
141698f79: 45 31 c0                    	xorl	%r8d, %r8d
141698f7c: e8 ef 64 55 ff              	callq	0x140bef470 <.text+0xbee470>
141698f81: a8 01                       	testb	$0x1, %al
141698f83: 0f 84 42 08 00 00           	je	0x1416997cb <.text+0x16987cb>
141698f89: 89 74 24 20                 	movl	%esi, 0x20(%rsp)
141698f8d: 4c 89 e1                    	movq	%r12, %rcx
141698f90: 49 89 d8                    	movq	%rbx, %r8
141698f93: 45 89 f1                    	movl	%r14d, %r9d
141698f96: 41 ff 97 50 03 00 00        	callq	*0x350(%r15)
141698f9d: e9 37 03 00 00              	jmp	0x1416992d9 <.text+0x16982d9>
141698fa2: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141698fa9: 48 8b 58 08                 	movq	0x8(%rax), %rbx
141698fad: 80 bb d8 0a 00 00 00        	cmpb	$0x0, 0xad8(%rbx)
141698fb4: 0f 84 5c 06 00 00           	je	0x141699616 <.text+0x1698616>
141698fba: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141698fc1: 44 8b a5 d8 01 00 00        	movl	0x1d8(%rbp), %r12d
141698fc8: 48 8b bd e0 01 00 00        	movq	0x1e0(%rbp), %rdi
141698fcf: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
141698fd6: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
141698fdd: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
141698fe4: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
141698feb: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
141698ff2: 80 bb b9 0a 00 00 00        	cmpb	$0x0, 0xab9(%rbx)
141698ff9: 0f 84 03 01 00 00           	je	0x141699102 <.text+0x1698102>
141698fff: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141699006: 48 8b 00                    	movq	(%rax), %rax
141699009: 48 8b 50 48                 	movq	0x48(%rax), %rdx
14169900d: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699014: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169901b: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699022: 48 89 d9                    	movq	%rbx, %rcx
141699025: e8 66 50 55 ff              	callq	0x140bee090 <.text+0xbed090>
14169902a: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141699031: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
141699035: e9 cf 00 00 00              	jmp	0x141699109 <.text+0x1698109>
14169903a: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
141699041: 48 8b 58 08                 	movq	0x8(%rax), %rbx
141699045: 80 bb d8 0a 00 00 00        	cmpb	$0x0, 0xad8(%rbx)
14169904c: 0f 84 f6 05 00 00           	je	0x141699648 <.text+0x1698648>
141699052: 44 8b ad d4 01 00 00        	movl	0x1d4(%rbp), %r13d
141699059: 44 8b a5 d8 01 00 00        	movl	0x1d8(%rbp), %r12d
141699060: 48 8b bd e0 01 00 00        	movq	0x1e0(%rbp), %rdi
141699067: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
14169906e: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
141699075: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
14169907c: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
141699083: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
14169908a: 80 bb b9 0a 00 00 00        	cmpb	$0x0, 0xab9(%rbx)
141699091: 0f 84 59 01 00 00           	je	0x1416991f0 <.text+0x16981f0>
141699097: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14169909e: 48 8b 00                    	movq	(%rax), %rax
1416990a1: 48 8b 50 48                 	movq	0x48(%rax), %rdx
1416990a5: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
1416990ac: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416990b3: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416990ba: 48 89 d9                    	movq	%rbx, %rcx
1416990bd: e8 ce 4f 55 ff              	callq	0x140bee090 <.text+0xbed090>
1416990c2: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416990c9: 48 8b 59 08                 	movq	0x8(%rcx), %rbx
1416990cd: e9 25 01 00 00              	jmp	0x1416991f7 <.text+0x16981f7>
1416990d2: 40 b6 01                    	movb	$0x1, %sil
1416990d5: 48 8b 95 e0 02 00 00        	movq	0x2e0(%rbp), %rdx
1416990dc: 48 85 d2                    	testq	%rdx, %rdx
1416990df: 0f 84 6d 02 00 00           	je	0x141699352 <.text+0x1698352>
1416990e5: 48 c1 e2 03                 	shlq	$0x3, %rdx
1416990e9: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416990ef: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1416990f6: e8 25 ab f7 fe              	callq	0x140613c20 <.text+0x612c20>
1416990fb: 31 c0                       	xorl	%eax, %eax
1416990fd: e9 e3 01 00 00              	jmp	0x1416992e5 <.text+0x16982e5>
141699102: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
141699109: f0                          	lock
14169910a: 48 ff 83 a8 0a 00 00        	incq	0xaa8(%rbx)
141699111: 48 8b 01                    	movq	(%rcx), %rax
141699114: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141699118: 4c 8b 78 48                 	movq	0x48(%rax), %r15
14169911c: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699123: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169912a: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699131: 44 89 ea                    	movl	%r13d, %edx
141699134: e8 97 e8 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699139: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699140: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699147: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169914e: 48 89 d9                    	movq	%rbx, %rcx
141699151: 48 89 c2                    	movq	%rax, %rdx
141699154: 45 31 c0                    	xorl	%r8d, %r8d
141699157: e8 14 63 55 ff              	callq	0x140bef470 <.text+0xbee470>
14169915c: a8 01                       	testb	$0x1, %al
14169915e: 0f 84 1b 06 00 00           	je	0x14169977f <.text+0x169877f>
141699164: 49 89 d5                    	movq	%rdx, %r13
141699167: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14169916e: 4c 8b 70 08                 	movq	0x8(%rax), %r14
141699172: 48 8b 48 10                 	movq	0x10(%rax), %rcx
141699176: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169917d: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699184: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169918b: 44 89 e2                    	movl	%r12d, %edx
14169918e: e8 3d e8 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699193: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169919a: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
1416991a1: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
1416991a8: 4c 89 f1                    	movq	%r14, %rcx
1416991ab: 48 89 c2                    	movq	%rax, %rdx
1416991ae: 45 31 c0                    	xorl	%r8d, %r8d
1416991b1: e8 ba 62 55 ff              	callq	0x140bef470 <.text+0xbee470>
1416991b6: a8 01                       	testb	$0x1, %al
1416991b8: 0f 84 e7 05 00 00           	je	0x1416997a5 <.text+0x16987a5>
1416991be: 49 89 d1                    	movq	%rdx, %r9
1416991c1: 48 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %rax
1416991c8: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
1416991cc: 89 74 24 28                 	movl	%esi, 0x28(%rsp)
1416991d0: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1416991d7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416991dc: 4c 89 f9                    	movq	%r15, %rcx
1416991df: 4c 89 ea                    	movq	%r13, %rdx
1416991e2: 49 89 f8                    	movq	%rdi, %r8
1416991e5: ff 93 b8 04 00 00           	callq	*0x4b8(%rbx)
1416991eb: e9 e9 00 00 00              	jmp	0x1416992d9 <.text+0x16982d9>
1416991f0: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416991f7: f0                          	lock
1416991f8: 48 ff 83 a8 0a 00 00        	incq	0xaa8(%rbx)
1416991ff: 48 8b 01                    	movq	(%rcx), %rax
141699202: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
141699206: 4c 8b 78 48                 	movq	0x48(%rax), %r15
14169920a: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699211: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699218: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169921f: 44 89 ea                    	movl	%r13d, %edx
141699222: e8 a9 e7 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699227: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169922e: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699235: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
14169923c: 48 89 d9                    	movq	%rbx, %rcx
14169923f: 48 89 c2                    	movq	%rax, %rdx
141699242: 45 31 c0                    	xorl	%r8d, %r8d
141699245: e8 26 62 55 ff              	callq	0x140bef470 <.text+0xbee470>
14169924a: a8 01                       	testb	$0x1, %al
14169924c: 0f 84 eb 05 00 00           	je	0x14169983d <.text+0x169883d>
141699252: 49 89 d5                    	movq	%rdx, %r13
141699255: 48 8b 85 08 03 00 00        	movq	0x308(%rbp), %rax
14169925c: 4c 8b 70 08                 	movq	0x8(%rax), %r14
141699260: 48 8b 48 10                 	movq	0x10(%rax), %rcx
141699264: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
14169926b: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
141699272: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699279: 44 89 e2                    	movl	%r12d, %edx
14169927c: e8 4f e7 54 ff              	callq	0x140be79d0 <.text+0xbe69d0>
141699281: c6 85 17 03 00 00 01        	movb	$0x1, 0x317(%rbp)
141699288: c6 85 16 03 00 00 01        	movb	$0x1, 0x316(%rbp)
14169928f: c6 85 15 03 00 00 00        	movb	$0x0, 0x315(%rbp)
141699296: 4c 89 f1                    	movq	%r14, %rcx
141699299: 48 89 c2                    	movq	%rax, %rdx
14169929c: 45 31 c0                    	xorl	%r8d, %r8d
14169929f: e8 cc 61 55 ff              	callq	0x140bef470 <.text+0xbee470>
1416992a4: a8 01                       	testb	$0x1, %al
1416992a6: 0f 84 b4 05 00 00           	je	0x141699860 <.text+0x1698860>
1416992ac: 49 89 d1                    	movq	%rdx, %r9
1416992af: 48 8b 85 d8 02 00 00        	movq	0x2d8(%rbp), %rax
1416992b6: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
1416992ba: 89 74 24 28                 	movl	%esi, 0x28(%rsp)
1416992be: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1416992c5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416992ca: 4c 89 f9                    	movq	%r15, %rcx
1416992cd: 4c 89 ea                    	movq	%r13, %rdx
1416992d0: 49 89 f8                    	movq	%rdi, %r8
1416992d3: ff 93 c0 04 00 00           	callq	*0x4c0(%rbx)
1416992d9: 40 b6 01                    	movb	$0x1, %sil
1416992dc: b0 01                       	movb	$0x1, %al
1416992de: 4c 8b bd 08 03 00 00        	movq	0x308(%rbp), %r15
