
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140161cd0: 55                          	pushq	%rbp
140161cd1: 41 57                       	pushq	%r15
140161cd3: 41 56                       	pushq	%r14
140161cd5: 41 55                       	pushq	%r13
140161cd7: 41 54                       	pushq	%r12
140161cd9: 56                          	pushq	%rsi
140161cda: 57                          	pushq	%rdi
140161cdb: 53                          	pushq	%rbx
140161cdc: b8 38 1b 00 00              	movl	$0x1b38, %eax           # imm = 0x1B38
140161ce1: e8 5a 8d 73 02              	callq	0x14289aa40 <.text+0x2899a40>
140161ce6: 48 29 c4                    	subq	%rax, %rsp
140161ce9: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140161cf1: 0f 29 b5 a0 1a 00 00        	movaps	%xmm6, 0x1aa0(%rbp)
140161cf8: 48 c7 85 98 1a 00 00 fe ff ff ff    	movq	$-0x2, 0x1a98(%rbp)
140161d03: 4d 89 c4                    	movq	%r8, %r12
140161d06: 48 89 d6                    	movq	%rdx, %rsi
140161d09: 48 8b 02                    	movq	(%rdx), %rax
140161d0c: 49 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r8 # imm = 0x8000000000000000
140161d16: 4c 89 85 08 1a 00 00        	movq	%r8, 0x1a08(%rbp)
140161d1d: 4c 89 02                    	movq	%r8, (%rdx)
140161d20: 48 89 c2                    	movq	%rax, %rdx
140161d23: 48 f7 da                    	negq	%rdx
140161d26: 0f 80 31 23 00 00           	jo	0x14016405d <.text+0x16305d>
140161d2c: 4c 89 8d 98 17 00 00        	movq	%r9, 0x1798(%rbp)
140161d33: 48 89 8d 90 17 00 00        	movq	%rcx, 0x1790(%rbp)
140161d3a: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140161d3e: 48 89 85 98 11 00 00        	movq	%rax, 0x1198(%rbp)
140161d45: 48 8d 8d a0 11 00 00        	leaq	0x11a0(%rbp), %rcx
140161d4c: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140161d52: e8 a9 1e 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140161d57: 4c 8b b6 90 00 00 00        	movq	0x90(%rsi), %r14
140161d5e: 48 c7 86 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rsi)
140161d69: 4d 85 f6                    	testq	%r14, %r14
140161d6c: 4c 89 b5 a8 17 00 00        	movq	%r14, 0x17a8(%rbp)
140161d73: 0f 84 f0 22 00 00           	je	0x140164069 <.text+0x163069>
140161d79: 48 8b 9d 20 1b 00 00        	movq	0x1b20(%rbp), %rbx
140161d80: 48 8b 86 e8 00 00 00        	movq	0xe8(%rsi), %rax
140161d87: 48 89 85 30 13 00 00        	movq	%rax, 0x1330(%rbp)
140161d8e: 0f 10 86 d8 00 00 00        	movups	0xd8(%rsi), %xmm0
140161d95: 0f 11 85 20 13 00 00        	movups	%xmm0, 0x1320(%rbp)
140161d9c: 0f 10 86 98 00 00 00        	movups	0x98(%rsi), %xmm0
140161da3: 0f 10 8e a8 00 00 00        	movups	0xa8(%rsi), %xmm1
140161daa: 0f 10 96 b8 00 00 00        	movups	0xb8(%rsi), %xmm2
140161db1: 48 89 b5 b0 19 00 00        	movq	%rsi, 0x19b0(%rbp)
140161db8: 0f 10 9e c8 00 00 00        	movups	0xc8(%rsi), %xmm3
140161dbf: 0f 11 9d 10 13 00 00        	movups	%xmm3, 0x1310(%rbp)
140161dc6: 0f 11 95 00 13 00 00        	movups	%xmm2, 0x1300(%rbp)
140161dcd: 0f 11 8d f0 12 00 00        	movups	%xmm1, 0x12f0(%rbp)
140161dd4: 0f 11 85 e0 12 00 00        	movups	%xmm0, 0x12e0(%rbp)
140161ddb: 48 8d 8d 28 12 00 00        	leaq	0x1228(%rbp), %rcx
140161de2: 48 8d 95 98 11 00 00        	leaq	0x1198(%rbp), %rdx
140161de9: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140161def: e8 0c 1e 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140161df4: 4c 8b 3b                    	movq	(%rbx), %r15
140161df7: 48 8b 73 08                 	movq	0x8(%rbx), %rsi
140161dfb: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140161dff: 48 89 85 48 1a 00 00        	movq	%rax, 0x1a48(%rbp)
140161e06: 48 8b 7b 18                 	movq	0x18(%rbx), %rdi
140161e0a: 48 8b 43 20                 	movq	0x20(%rbx), %rax
140161e0e: 48 89 85 00 1a 00 00        	movq	%rax, 0x1a00(%rbp)
140161e15: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140161e19: 48 89 85 f0 19 00 00        	movq	%rax, 0x19f0(%rbp)
140161e20: 48 8b 43 30                 	movq	0x30(%rbx), %rax
140161e24: 48 89 85 80 17 00 00        	movq	%rax, 0x1780(%rbp)
140161e2b: 48 8b 43 38                 	movq	0x38(%rbx), %rax
140161e2f: 48 89 85 b8 19 00 00        	movq	%rax, 0x19b8(%rbp)
140161e36: 48 c7 85 b8 12 00 00 00 00 00 00    	movq	$0x0, 0x12b8(%rbp)
140161e41: 48 c7 85 c0 12 00 00 08 00 00 00    	movq	$0x8, 0x12c0(%rbp)
140161e4c: 48 c7 85 c8 12 00 00 00 00 00 00    	movq	$0x0, 0x12c8(%rbp)
140161e57: 4c 89 a5 88 17 00 00        	movq	%r12, 0x1788(%rbp)
140161e5e: 4c 89 a5 d0 12 00 00        	movq	%r12, 0x12d0(%rbp)
140161e65: 4c 89 b5 d8 12 00 00        	movq	%r14, 0x12d8(%rbp)
140161e6c: 4d 8d 67 38                 	leaq	0x38(%r15), %r12
140161e70: c6 85 93 1a 00 00 01        	movb	$0x1, 0x1a93(%rbp)
140161e77: 48 ba aa 4e 78 96 b3 61 85 a6       	movabsq	$-0x597a9e4c6987b156, %rdx # imm = 0xA68561B396784EAA
140161e81: 49 b8 14 ed f4 fa e6 74 fd f5       	movabsq	$-0xa028b19050b12ec, %r8 # imm = 0xF5FD74E6FAF4ED14
140161e8b: 4c 89 e1                    	movq	%r12, %rcx
140161e8e: e8 5d b8 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140161e93: 48 85 c0                    	testq	%rax, %rax
140161e96: 0f 84 72 20 00 00           	je	0x140163f0e <.text+0x162f0e>
140161e9c: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140161ea3: 0f 86 65 20 00 00           	jbe	0x140163f0e <.text+0x162f0e>
140161ea9: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140161eb0: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
140161eb4: 48 85 c0                    	testq	%rax, %rax
140161eb7: 0f 84 51 20 00 00           	je	0x140163f0e <.text+0x162f0e>
140161ebd: 4d 8b af e0 01 00 00        	movq	0x1e0(%r15), %r13
140161ec4: 48 f7 d0                    	notq	%rax
140161ec7: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140161ecb: 48 c1 e0 05                 	shlq	$0x5, %rax
140161ecf: 49 83 7c 05 20 00           	cmpq	$0x0, 0x20(%r13,%rax)
140161ed5: 0f 84 33 20 00 00           	je	0x140163f0e <.text+0x162f0e>
140161edb: 49 01 c5                    	addq	%rax, %r13
140161ede: 49 8b 5d 10                 	movq	0x10(%r13), %rbx
140161ee2: 45 8b b7 b0 05 00 00        	movl	0x5b0(%r15), %r14d
140161ee9: 45 89 75 5c                 	movl	%r14d, 0x5c(%r13)
140161eed: c6 85 93 1a 00 00 01        	movb	$0x1, 0x1a93(%rbp)
140161ef4: 48 8d 95 28 12 00 00        	leaq	0x1228(%rbp), %rdx
140161efb: 48 89 d9                    	movq	%rbx, %rcx
140161efe: e8 2d ae 7d 00              	callq	0x14093cd30 <.text+0x93bd30>
140161f03: 45 89 75 5c                 	movl	%r14d, 0x5c(%r13)
140161f07: c6 85 93 1a 00 00 01        	movb	$0x1, 0x1a93(%rbp)
140161f0e: 48 89 d9                    	movq	%rbx, %rcx
140161f11: 48 89 f2                    	movq	%rsi, %rdx
140161f14: e8 27 b1 7d 00              	callq	0x14093d040 <.text+0x93c040>
140161f19: c6 85 93 1a 00 00 01        	movb	$0x1, 0x1a93(%rbp)
140161f20: 48 8d 8d 68 15 00 00        	leaq	0x1568(%rbp), %rcx
140161f27: 48 89 c2                    	movq	%rax, %rdx
140161f2a: e8 d1 4b a7 00              	callq	0x140bd6b00 <.text+0xbd5b00>
140161f2f: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
140161f36: 48 ba ef a5 99 86 45 78 29 66       	movabsq	$0x662978458699a5ef, %rdx # imm = 0x662978458699A5EF
140161f40: 49 b8 46 9b 41 62 c1 57 c0 3e       	movabsq	$0x3ec057c162419b46, %r8 # imm = 0x3EC057C162419B46
140161f4a: 4c 89 e1                    	movq	%r12, %rcx
140161f4d: e8 9e b7 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140161f52: 48 85 c0                    	testq	%rax, %rax
140161f55: 0f 84 cb 1f 00 00           	je	0x140163f26 <.text+0x162f26>
140161f5b: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140161f62: 0f 86 be 1f 00 00           	jbe	0x140163f26 <.text+0x162f26>
140161f68: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140161f6f: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140161f73: 48 85 c9                    	testq	%rcx, %rcx
140161f76: 0f 84 aa 1f 00 00           	je	0x140163f26 <.text+0x162f26>
140161f7c: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140161f83: 48 f7 d1                    	notq	%rcx
140161f86: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140161f8a: 48 c1 e1 05                 	shlq	$0x5, %rcx
140161f8e: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140161f94: 0f 84 8c 1f 00 00           	je	0x140163f26 <.text+0x162f26>
140161f9a: 49 89 fe                    	movq	%rdi, %r14
140161f9d: 48 01 c8                    	addq	%rcx, %rax
140161fa0: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140161fa4: 41 8b 97 b0 05 00 00        	movl	0x5b0(%r15), %edx
140161fab: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140161fae: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
140161fb5: 48 8d 95 28 12 00 00        	leaq	0x1228(%rbp), %rdx
140161fbc: e8 ff 01 83 00              	callq	0x1409921c0 <.text+0x9911c0>
140161fc1: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
140161fc8: 48 ba ca 3b 97 c7 19 dc d9 8f       	movabsq	$-0x702623e63868c436, %rdx # imm = 0x8FD9DC19C7973BCA
140161fd2: 49 b8 fd 45 9b eb c5 f4 1f 36       	movabsq	$0x361ff4c5eb9b45fd, %r8 # imm = 0x361FF4C5EB9B45FD
140161fdc: 4c 89 e1                    	movq	%r12, %rcx
140161fdf: e8 0c b7 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140161fe4: 48 85 c0                    	testq	%rax, %rax
140161fe7: 0f 84 51 1f 00 00           	je	0x140163f3e <.text+0x162f3e>
140161fed: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140161ff4: 0f 86 44 1f 00 00           	jbe	0x140163f3e <.text+0x162f3e>
140161ffa: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140162001: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140162005: 48 85 c9                    	testq	%rcx, %rcx
140162008: 0f 84 30 1f 00 00           	je	0x140163f3e <.text+0x162f3e>
14016200e: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140162015: 48 f7 d1                    	notq	%rcx
140162018: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14016201c: 48 c1 e1 05                 	shlq	$0x5, %rcx
140162020: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140162026: 0f 84 12 1f 00 00           	je	0x140163f3e <.text+0x162f3e>
14016202c: 48 01 c8                    	addq	%rcx, %rax
14016202f: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140162033: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
14016203a: 41 b8 90 10 00 00           	movl	$0x1090, %r8d           # imm = 0x1090
140162040: e8 bb 1b 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140162045: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
14016204c: 48 ba 2e a7 83 82 b7 4c c9 47       	movabsq	$0x47c94cb78283a72e, %rdx # imm = 0x47C94CB78283A72E
140162056: 49 b8 9d 3e 32 97 ed 3d 17 8c       	movabsq	$-0x73e8c21268cdc163, %r8 # imm = 0x8C173DED97323E9D
140162060: 4c 89 e1                    	movq	%r12, %rcx
140162063: e8 88 b6 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140162068: 48 85 c0                    	testq	%rax, %rax
14016206b: 0f 84 e5 1e 00 00           	je	0x140163f56 <.text+0x162f56>
140162071: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140162078: 0f 86 d8 1e 00 00           	jbe	0x140163f56 <.text+0x162f56>
14016207e: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140162085: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140162089: 48 85 c9                    	testq	%rcx, %rcx
14016208c: 0f 84 c4 1e 00 00           	je	0x140163f56 <.text+0x162f56>
140162092: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140162099: 48 f7 d1                    	notq	%rcx
14016209c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1401620a0: 48 c1 e1 05                 	shlq	$0x5, %rcx
1401620a4: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1401620aa: 0f 84 a6 1e 00 00           	je	0x140163f56 <.text+0x162f56>
1401620b0: 48 01 c8                    	addq	%rcx, %rax
1401620b3: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1401620b7: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
1401620be: e8 bd bc 51 00              	callq	0x14067dd80 <.text+0x67cd80>
1401620c3: 48 89 c7                    	movq	%rax, %rdi
1401620c6: b1 01                       	movb	$0x1, %cl
1401620c8: 31 c0                       	xorl	%eax, %eax
1401620ca: f0                          	lock
1401620cb: 0f b0 0f                    	cmpxchgb	%cl, (%rdi)
1401620ce: 0f 85 bc 1d 00 00           	jne	0x140163e90 <.text+0x162e90>
1401620d4: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
1401620db: 48 89 f9                    	movq	%rdi, %rcx
1401620de: e8 8d fb b1 00              	callq	0x140c81c70 <.text+0xc80c70>
1401620e3: 48 89 bd 80 1a 00 00        	movq	%rdi, 0x1a80(%rbp)
1401620ea: 48 83 c7 08                 	addq	$0x8, %rdi
1401620ee: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
1401620f7: 48 8d 8d 10 18 00 00        	leaq	0x1810(%rbp), %rcx
1401620fe: 41 b9 90 10 00 00           	movl	$0x1090, %r9d           # imm = 0x1090
140162104: 48 89 fa                    	movq	%rdi, %rdx
140162107: 49 89 f0                    	movq	%rsi, %r8
14016210a: e8 31 06 aa 00              	callq	0x140c02740 <.text+0xc01740>
14016210f: 48 8b 8d 18 18 00 00        	movq	0x1818(%rbp), %rcx
140162116: 48 81 f9 90 10 00 00        	cmpq	$0x1090, %rcx           # imm = 0x1090
14016211d: 0f 85 87 1d 00 00           	jne	0x140163eaa <.text+0x162eaa>
140162123: 48 8b 8d 10 18 00 00        	movq	0x1810(%rbp), %rcx
14016212a: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140162131: 41 b8 90 10 00 00           	movl	$0x1090, %r8d           # imm = 0x1090
140162137: e8 c4 1a 7c 02              	callq	0x142923c00 <.text+0x2922c00>
14016213c: 0f 10 85 10 18 00 00        	movups	0x1810(%rbp), %xmm0
140162143: 0f 10 8d 20 18 00 00        	movups	0x1820(%rbp), %xmm1
14016214a: 0f 10 95 30 18 00 00        	movups	0x1830(%rbp), %xmm2
140162151: 0f 10 9d 40 18 00 00        	movups	0x1840(%rbp), %xmm3
140162158: 0f 29 85 e0 15 00 00        	movaps	%xmm0, 0x15e0(%rbp)
14016215f: 0f 29 8d f0 15 00 00        	movaps	%xmm1, 0x15f0(%rbp)
140162166: 0f 29 95 00 16 00 00        	movaps	%xmm2, 0x1600(%rbp)
14016216d: 0f 29 9d 10 16 00 00        	movaps	%xmm3, 0x1610(%rbp)
140162174: 48 8b 85 50 18 00 00        	movq	0x1850(%rbp), %rax
14016217b: 48 89 85 20 16 00 00        	movq	%rax, 0x1620(%rbp)
140162182: 48 8d 8d 38 13 00 00        	leaq	0x1338(%rbp), %rcx
140162189: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
140162190: 49 89 f0                    	movq	%rsi, %r8
140162193: e8 d8 01 aa 00              	callq	0x140c02370 <.text+0xc01370>
140162198: c6 85 8e 1a 00 00 01        	movb	$0x1, 0x1a8e(%rbp)
14016219f: 48 8b bd 80 1a 00 00        	movq	0x1a80(%rbp), %rdi
1401621a6: 48 89 f9                    	movq	%rdi, %rcx
1401621a9: e8 62 fc b1 00              	callq	0x140c81e10 <.text+0xc80e10>
1401621ae: 31 c9                       	xorl	%ecx, %ecx
1401621b0: b0 01                       	movb	$0x1, %al
1401621b2: f0                          	lock
1401621b3: 0f b0 0f                    	cmpxchgb	%cl, (%rdi)
1401621b6: 0f 85 04 1d 00 00           	jne	0x140163ec0 <.text+0x162ec0>
1401621bc: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401621c3: 48 8d 95 28 12 00 00        	leaq	0x1228(%rbp), %rdx
1401621ca: 41 b8 10 01 00 00           	movl	$0x110, %r8d            # imm = 0x110
1401621d0: e8 2b 1a 7c 02              	callq	0x142923c00 <.text+0x2922c00>
1401621d5: 0f 10 85 38 13 00 00        	movups	0x1338(%rbp), %xmm0
1401621dc: 0f 10 8d 48 13 00 00        	movups	0x1348(%rbp), %xmm1
1401621e3: 0f 10 95 58 13 00 00        	movups	0x1358(%rbp), %xmm2
1401621ea: 0f 10 9d 68 13 00 00        	movups	0x1368(%rbp), %xmm3
1401621f1: 0f 11 85 f0 16 00 00        	movups	%xmm0, 0x16f0(%rbp)
1401621f8: 0f 11 8d 00 17 00 00        	movups	%xmm1, 0x1700(%rbp)
1401621ff: 0f 11 95 10 17 00 00        	movups	%xmm2, 0x1710(%rbp)
140162206: 0f 11 9d 20 17 00 00        	movups	%xmm3, 0x1720(%rbp)
14016220d: 0f 10 85 78 13 00 00        	movups	0x1378(%rbp), %xmm0
140162214: 0f 11 85 30 17 00 00        	movups	%xmm0, 0x1730(%rbp)
14016221b: 0f 10 85 88 13 00 00        	movups	0x1388(%rbp), %xmm0
140162222: 0f 11 85 40 17 00 00        	movups	%xmm0, 0x1740(%rbp)
140162229: 0f 10 85 68 15 00 00        	movups	0x1568(%rbp), %xmm0
140162230: 0f 11 85 50 17 00 00        	movups	%xmm0, 0x1750(%rbp)
140162237: 48 8b 85 78 15 00 00        	movq	0x1578(%rbp), %rax
14016223e: 48 89 85 60 17 00 00        	movq	%rax, 0x1760(%rbp)
140162245: 48 8b 85 48 1a 00 00        	movq	0x1a48(%rbp), %rax
14016224c: 48 89 85 68 17 00 00        	movq	%rax, 0x1768(%rbp)
140162253: 48 89 b5 70 17 00 00        	movq	%rsi, 0x1770(%rbp)
14016225a: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162261: 48 ba 5a 46 f2 43 f4 45 2d 16       	movabsq	$0x162d45f443f2465a, %rdx # imm = 0x162D45F443F2465A
14016226b: 49 b8 0f fc 08 55 59 d9 b0 7d       	movabsq	$0x7db0d9595508fc0f, %r8 # imm = 0x7DB0D9595508FC0F
140162275: 4c 89 e1                    	movq	%r12, %rcx
140162278: e8 73 b4 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14016227d: 48 85 c0                    	testq	%rax, %rax
140162280: 0f 84 e8 1c 00 00           	je	0x140163f6e <.text+0x162f6e>
140162286: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
14016228d: 0f 86 db 1c 00 00           	jbe	0x140163f6e <.text+0x162f6e>
140162293: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
14016229a: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14016229e: 48 85 c9                    	testq	%rcx, %rcx
1401622a1: 0f 84 c7 1c 00 00           	je	0x140163f6e <.text+0x162f6e>
1401622a7: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
1401622ae: 48 f7 d1                    	notq	%rcx
1401622b1: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1401622b5: 48 c1 e1 05                 	shlq	$0x5, %rcx
1401622b9: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1401622bf: 0f 84 a9 1c 00 00           	je	0x140163f6e <.text+0x162f6e>
1401622c5: 48 01 c8                    	addq	%rcx, %rax
1401622c8: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1401622cc: 41 8b 97 b0 05 00 00        	movl	0x5b0(%r15), %edx
1401622d3: 89 50 5c                    	movl	%edx, 0x5c(%rax)
1401622d6: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401622dd: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
1401622e4: 4d 89 f8                    	movq	%r15, %r8
1401622e7: e8 94 bf 7a 00              	callq	0x14090e280 <.text+0x90d280>
1401622ec: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401622f3: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401622fa: 4c 89 fa                    	movq	%r15, %rdx
1401622fd: e8 6e 85 7a 00              	callq	0x14090a870 <.text+0x909870>
140162302: 49 8b 16                    	movq	(%r14), %rdx
140162305: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016230c: 48 8d 8d 10 18 00 00        	leaq	0x1810(%rbp), %rcx
140162313: 41 b8 7a 00 00 00           	movl	$0x7a, %r8d
140162319: e8 82 2f a7 00              	callq	0x140bd52a0 <.text+0xbd42a0>
14016231e: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162325: 4c 8d ad 10 1a 00 00        	leaq	0x1a10(%rbp), %r13
14016232c: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
140162333: 4c 89 e9                    	movq	%r13, %rcx
140162336: 41 b0 06                    	movb	$0x6, %r8b
140162339: e8 22 31 a7 00              	callq	0x140bd5460 <.text+0xbd4460>
14016233e: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
140162345: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
14016234c: 0f 29 8d 60 14 00 00        	movaps	%xmm1, 0x1460(%rbp)
140162353: 0f 29 85 50 14 00 00        	movaps	%xmm0, 0x1450(%rbp)
14016235a: 48 8d 8d 20 18 00 00        	leaq	0x1820(%rbp), %rcx
140162361: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162368: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
14016236f: e8 6c 49 a7 00              	callq	0x140bd6ce0 <.text+0xbd5ce0>
140162374: 48 8d 05 2d b3 7f 02        	leaq	0x27fb32d(%rip), %rax   # 0x14295d6a8
14016237b: 48 89 85 10 18 00 00        	movq	%rax, 0x1810(%rbp)
140162382: 48 c7 85 18 18 00 00 0a 00 00 00    	movq	$0xa, 0x1818(%rbp)
14016238d: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162394: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
14016239b: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
1401623a2: e8 29 71 a8 00              	callq	0x140be94d0 <.text+0xbe84d0>
1401623a7: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
1401623ae: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
1401623b5: 0f 11 85 c4 19 00 00        	movups	%xmm0, 0x19c4(%rbp)
1401623bc: 0f 11 8d d4 19 00 00        	movups	%xmm1, 0x19d4(%rbp)
1401623c3: 89 85 c0 19 00 00           	movl	%eax, 0x19c0(%rbp)
1401623c9: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401623d0: 48 ba 7a 9c b7 7f 61 ae 8d d3       	movabsq	$-0x2c72519e80486386, %rdx # imm = 0xD38DAE617FB79C7A
1401623da: 49 b8 ae 1e ad 5d 5c df aa 22       	movabsq	$0x22aadf5c5dad1eae, %r8 # imm = 0x22AADF5C5DAD1EAE
1401623e4: 4c 89 e1                    	movq	%r12, %rcx
1401623e7: e8 04 b3 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1401623ec: 48 85 c0                    	testq	%rax, %rax
1401623ef: 0f 84 91 1b 00 00           	je	0x140163f86 <.text+0x162f86>
1401623f5: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
1401623fc: 0f 86 84 1b 00 00           	jbe	0x140163f86 <.text+0x162f86>
140162402: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140162409: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14016240d: 48 85 c9                    	testq	%rcx, %rcx
140162410: 0f 84 70 1b 00 00           	je	0x140163f86 <.text+0x162f86>
140162416: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
14016241d: 48 f7 d1                    	notq	%rcx
140162420: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140162424: 48 c1 e1 05                 	shlq	$0x5, %rcx
140162428: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
14016242e: 0f 84 52 1b 00 00           	je	0x140163f86 <.text+0x162f86>
140162434: 48 01 c8                    	addq	%rcx, %rax
140162437: 48 8b 70 10                 	movq	0x10(%rax), %rsi
14016243b: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162442: 48 ba e9 78 f5 b6 45 5c 38 35       	movabsq	$0x35385c45b6f578e9, %rdx # imm = 0x35385C45B6F578E9
14016244c: 49 b8 9f 0c dd d5 81 3d 62 f1       	movabsq	$-0xe9dc27e2a22f361, %r8 # imm = 0xF1623D81D5DD0C9F
140162456: 4c 89 e1                    	movq	%r12, %rcx
140162459: e8 92 b2 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14016245e: 48 85 c0                    	testq	%rax, %rax
140162461: 0f 84 37 1b 00 00           	je	0x140163f9e <.text+0x162f9e>
140162467: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
14016246e: 0f 86 2a 1b 00 00           	jbe	0x140163f9e <.text+0x162f9e>
140162474: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
14016247b: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14016247f: 48 85 c9                    	testq	%rcx, %rcx
140162482: 0f 84 16 1b 00 00           	je	0x140163f9e <.text+0x162f9e>
140162488: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
14016248f: 48 f7 d1                    	notq	%rcx
140162492: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140162496: 48 c1 e1 05                 	shlq	$0x5, %rcx
14016249a: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1401624a0: 0f 84 f8 1a 00 00           	je	0x140163f9e <.text+0x162f9e>
1401624a6: 48 01 c8                    	addq	%rcx, %rax
1401624a9: 48 8b 40 10                 	movq	0x10(%rax), %rax
1401624ad: 80 38 01                    	cmpb	$0x1, (%rax)
1401624b0: 0f 85 fe 02 00 00           	jne	0x1401627b4 <.text+0x1617b4>
1401624b6: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401624bd: 48 89 f1                    	movq	%rsi, %rcx
1401624c0: e8 5b d7 7d 00              	callq	0x14093fc20 <.text+0x93ec20>
1401624c5: 84 c0                       	testb	%al, %al
1401624c7: 0f 84 e7 02 00 00           	je	0x1401627b4 <.text+0x1617b4>
1401624cd: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401624d4: 48 ba 7a 9c b7 7f 61 ae 8d d3       	movabsq	$-0x2c72519e80486386, %rdx # imm = 0xD38DAE617FB79C7A
1401624de: 49 b8 ae 1e ad 5d 5c df aa 22       	movabsq	$0x22aadf5c5dad1eae, %r8 # imm = 0x22AADF5C5DAD1EAE
1401624e8: 4c 89 e1                    	movq	%r12, %rcx
1401624eb: e8 00 b2 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1401624f0: 48 85 c0                    	testq	%rax, %rax
1401624f3: 0f 84 7e 1b 00 00           	je	0x140164077 <.text+0x163077>
1401624f9: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140162500: 0f 86 71 1b 00 00           	jbe	0x140164077 <.text+0x163077>
140162506: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
14016250d: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140162511: 48 85 c9                    	testq	%rcx, %rcx
140162514: 0f 84 5d 1b 00 00           	je	0x140164077 <.text+0x163077>
14016251a: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140162521: 48 f7 d1                    	notq	%rcx
140162524: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140162528: 48 c1 e1 05                 	shlq	$0x5, %rcx
14016252c: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140162532: 0f 84 3f 1b 00 00           	je	0x140164077 <.text+0x163077>
140162538: 48 01 c8                    	addq	%rcx, %rax
14016253b: 48 8b 48 10                 	movq	0x10(%rax), %rcx
14016253f: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162546: e8 75 d6 7d 00              	callq	0x14093fbc0 <.text+0x93ebc0>
14016254b: 89 85 48 1a 00 00           	movl	%eax, 0x1a48(%rbp)
140162551: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162558: 48 89 f1                    	movq	%rsi, %rcx
14016255b: e8 80 d6 7d 00              	callq	0x14093fbe0 <.text+0x93ebe0>
140162560: 84 c0                       	testb	%al, %al
140162562: 0f 85 d8 02 00 00           	jne	0x140162840 <.text+0x161840>
140162568: 80 bd 48 1a 00 00 00        	cmpb	$0x0, 0x1a48(%rbp)
14016256f: 0f 84 9f 00 00 00           	je	0x140162614 <.text+0x161614>
140162575: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016257c: 48 ba 60 6c ba bd c9 d9 18 1f       	movabsq	$0x1f18d9c9bdba6c60, %rdx # imm = 0x1F18D9C9BDBA6C60
140162586: 49 b8 67 0b e9 48 66 98 47 85       	movabsq	$-0x7ab86799b716f499, %r8 # imm = 0x8547986648E90B67
140162590: 4c 89 e1                    	movq	%r12, %rcx
140162593: e8 58 b1 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140162598: 48 85 c0                    	testq	%rax, %rax
14016259b: 0f 84 0e 1b 00 00           	je	0x1401640af <.text+0x1630af>
1401625a1: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
1401625a8: 0f 86 01 1b 00 00           	jbe	0x1401640af <.text+0x1630af>
1401625ae: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
1401625b5: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1401625b9: 48 85 c9                    	testq	%rcx, %rcx
1401625bc: 0f 84 ed 1a 00 00           	je	0x1401640af <.text+0x1630af>
1401625c2: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
1401625c9: 48 f7 d1                    	notq	%rcx
1401625cc: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1401625d0: 48 c1 e1 05                 	shlq	$0x5, %rcx
1401625d4: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1401625da: 0f 84 cf 1a 00 00           	je	0x1401640af <.text+0x1630af>
1401625e0: 48 01 c8                    	addq	%rcx, %rax
1401625e3: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1401625e7: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401625ee: e8 dd 3d 74 00              	callq	0x1408a63d0 <.text+0x8a53d0>
1401625f3: 48 8d 0d 1e b1 7f 02        	leaq	0x27fb11e(%rip), %rcx   # 0x14295d718
1401625fa: 48 89 8d 10 1a 00 00        	movq	%rcx, 0x1a10(%rbp)
140162601: 48 c7 85 18 1a 00 00 18 00 00 00    	movq	$0x18, 0x1a18(%rbp)
14016260c: 84 c0                       	testb	%al, %al
14016260e: 0f 84 7b 17 00 00           	je	0x140163d8f <.text+0x162d8f>
140162614: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016261b: 48 ba ca 3b 97 c7 19 dc d9 8f       	movabsq	$-0x702623e63868c436, %rdx # imm = 0x8FD9DC19C7973BCA
140162625: 49 b8 fd 45 9b eb c5 f4 1f 36       	movabsq	$0x361ff4c5eb9b45fd, %r8 # imm = 0x361FF4C5EB9B45FD
14016262f: 4c 89 e1                    	movq	%r12, %rcx
140162632: e8 b9 b0 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140162637: 48 85 c0                    	testq	%rax, %rax
14016263a: 0f 84 5a 1a 00 00           	je	0x14016409a <.text+0x16309a>
140162640: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140162647: 0f 86 4d 1a 00 00           	jbe	0x14016409a <.text+0x16309a>
14016264d: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140162654: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140162658: 48 85 c9                    	testq	%rcx, %rcx
14016265b: 0f 84 39 1a 00 00           	je	0x14016409a <.text+0x16309a>
140162661: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140162668: 48 f7 d1                    	notq	%rcx
14016266b: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14016266f: 48 c1 e1 05                 	shlq	$0x5, %rcx
140162673: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140162679: 0f 84 1b 1a 00 00           	je	0x14016409a <.text+0x16309a>
14016267f: 48 01 c8                    	addq	%rcx, %rax
140162682: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140162686: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016268d: e8 ee 41 81 00              	callq	0x140976880 <.text+0x975880>
140162692: 48 8d 0d af b0 7f 02        	leaq	0x27fb0af(%rip), %rcx   # 0x14295d748
140162699: 48 89 8d 10 1a 00 00        	movq	%rcx, 0x1a10(%rbp)
1401626a0: 48 c7 85 18 1a 00 00 14 00 00 00    	movq	$0x14, 0x1a18(%rbp)
1401626ab: 84 c0                       	testb	%al, %al
1401626ad: 0f 84 ab 15 00 00           	je	0x140163c5e <.text+0x162c5e>
1401626b3: 48 8b 05 ee 9c 4c 03        	movq	0x34c9cee(%rip), %rax   # 0x14362c3a8
1401626ba: 48 8b 00                    	movq	(%rax), %rax
1401626bd: 48 83 f8 05                 	cmpq	$0x5, %rax
1401626c1: 0f 82 79 01 00 00           	jb	0x140162840 <.text+0x161840>
1401626c7: 4c 89 ad 50 14 00 00        	movq	%r13, 0x1450(%rbp)
1401626ce: 48 8d 05 bb 77 fa ff        	leaq	-0x58845(%rip), %rax    # 0x140109e90 <.text+0x108e90>
1401626d5: 48 89 85 58 14 00 00        	movq	%rax, 0x1458(%rbp)
1401626dc: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401626e3: 48 8d 0d 6e 94 88 02        	leaq	0x288946e(%rip), %rcx   # 0x1429ebb58
1401626ea: e8 91 b6 51 00              	callq	0x14067dd80 <.text+0x67cd80>
1401626ef: 0f 10 00                    	movups	(%rax), %xmm0
1401626f2: 8b 40 10                    	movl	0x10(%rax), %eax
1401626f5: 48 c7 85 40 18 00 00 05 00 00 00    	movq	$0x5, 0x1840(%rbp)
140162700: 48 8d 0d 69 94 88 02        	leaq	0x2889469(%rip), %rcx   # 0x1429ebb70
140162707: 48 89 8d 48 18 00 00        	movq	%rcx, 0x1848(%rbp)
14016270e: 48 c7 85 50 18 00 00 17 00 00 00    	movq	$0x17, 0x1850(%rbp)
140162719: 48 8d 15 00 94 88 02        	leaq	0x2889400(%rip), %rdx   # 0x1429ebb20
140162720: 48 89 95 60 18 00 00        	movq	%rdx, 0x1860(%rbp)
140162727: 48 c7 85 68 18 00 00 01 00 00 00    	movq	$0x1, 0x1868(%rbp)
140162732: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140162739: 48 89 95 70 18 00 00        	movq	%rdx, 0x1870(%rbp)
140162740: 48 c7 85 78 18 00 00 01 00 00 00    	movq	$0x1, 0x1878(%rbp)
14016274b: 48 c7 85 80 18 00 00 00 00 00 00    	movq	$0x0, 0x1880(%rbp)
140162756: 48 c7 85 10 18 00 00 00 00 00 00    	movq	$0x0, 0x1810(%rbp)
140162761: 48 89 8d 18 18 00 00        	movq	%rcx, 0x1818(%rbp)
140162768: 48 c7 85 20 18 00 00 17 00 00 00    	movq	$0x17, 0x1820(%rbp)
140162773: 48 c7 85 28 18 00 00 00 00 00 00    	movq	$0x0, 0x1828(%rbp)
14016277e: 0f 11 85 30 18 00 00        	movups	%xmm0, 0x1830(%rbp)
140162785: c7 85 58 18 00 00 01 00 00 00       	movl	$0x1, 0x1858(%rbp)
14016278f: 89 85 5c 18 00 00           	movl	%eax, 0x185c(%rbp)
140162795: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016279c: 48 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %rcx
1401627a3: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
1401627aa: e8 81 7b be 00              	callq	0x140d4a330 <.text+0xd49330>
1401627af: e9 8c 00 00 00              	jmp	0x140162840 <.text+0x161840>
1401627b4: c7 85 48 1a 00 00 00 00 00 00       	movl	$0x0, 0x1a48(%rbp)
1401627be: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401627c5: 48 ba 59 51 91 8c f9 e8 31 68       	movabsq	$0x6831e8f98c915159, %rdx # imm = 0x6831E8F98C915159
1401627cf: 49 b8 25 07 00 e4 79 c4 15 d5       	movabsq	$-0x2aea3b861bfff8db, %r8 # imm = 0xD515C479E4000725
1401627d9: 4c 89 e1                    	movq	%r12, %rcx
1401627dc: e8 0f af f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1401627e1: 48 85 c0                    	testq	%rax, %rax
1401627e4: 0f 84 5e 18 00 00           	je	0x140164048 <.text+0x163048>
1401627ea: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
1401627f1: 0f 86 51 18 00 00           	jbe	0x140164048 <.text+0x163048>
1401627f7: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
1401627fe: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140162802: 48 85 c9                    	testq	%rcx, %rcx
140162805: 0f 84 3d 18 00 00           	je	0x140164048 <.text+0x163048>
14016280b: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140162812: 48 f7 d1                    	notq	%rcx
140162815: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140162819: 48 c1 e1 05                 	shlq	$0x5, %rcx
14016281d: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140162823: 0f 84 1f 18 00 00           	je	0x140164048 <.text+0x163048>
140162829: 48 01 c8                    	addq	%rcx, %rax
14016282c: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140162830: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162837: e8 b4 c8 7b 00              	callq	0x14091f0f0 <.text+0x91e0f0>
14016283c: 84 c0                       	testb	%al, %al
14016283e: 74 34                       	je	0x140162874 <.text+0x161874>
140162840: 80 3d 49 23 4d 03 01        	cmpb	$0x1, 0x34d2349(%rip)   # 0x143634b90
140162847: 0f 85 96 01 00 00           	jne	0x1401629e3 <.text+0x1619e3>
14016284d: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162854: b0 01                       	movb	$0x1, %al
140162856: 89 85 80 1a 00 00           	movl	%eax, 0x1a80(%rbp)
14016285c: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140162863: 4c 8d 85 c0 19 00 00        	leaq	0x19c0(%rbp), %r8
14016286a: 4c 89 fa                    	movq	%r15, %rdx
14016286d: e8 2e c4 85 00              	callq	0x1409beca0 <.text+0x9bdca0>
140162872: eb 0a                       	jmp	0x14016287e <.text+0x16187e>
140162874: c7 85 80 1a 00 00 00 00 00 00       	movl	$0x0, 0x1a80(%rbp)
14016287e: b3 07                       	movb	$0x7, %bl
140162880: 49 8b 16                    	movq	(%r14), %rdx
140162883: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016288a: 48 8d 8d 10 1a 00 00        	leaq	0x1a10(%rbp), %rcx
140162891: 41 b8 82 00 00 00           	movl	$0x82, %r8d
140162897: e8 04 2a a7 00              	callq	0x140bd52a0 <.text+0xbd42a0>
14016289c: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
1401628a3: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
1401628aa: 0f 29 8d 60 14 00 00        	movaps	%xmm1, 0x1460(%rbp)
1401628b1: 0f 29 85 50 14 00 00        	movaps	%xmm0, 0x1450(%rbp)
1401628b8: 48 8d 8d 20 18 00 00        	leaq	0x1820(%rbp), %rcx
1401628bf: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401628c6: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
1401628cd: e8 0e 44 a7 00              	callq	0x140bd6ce0 <.text+0xbd5ce0>
1401628d2: 48 8d 05 bf ae 7f 02        	leaq	0x27faebf(%rip), %rax   # 0x14295d798
1401628d9: 48 89 85 10 18 00 00        	movq	%rax, 0x1810(%rbp)
1401628e0: 48 c7 85 18 18 00 00 0a 00 00 00    	movq	$0xa, 0x1818(%rbp)
1401628eb: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401628f2: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401628f9: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
140162900: e8 cb 6b a8 00              	callq	0x140be94d0 <.text+0xbe84d0>
140162905: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
14016290c: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
140162913: 0f 11 85 e4 17 00 00        	movups	%xmm0, 0x17e4(%rbp)
14016291a: 0f 11 8d f4 17 00 00        	movups	%xmm1, 0x17f4(%rbp)
140162921: 89 85 e0 17 00 00           	movl	%eax, 0x17e0(%rbp)
140162927: 80 fb 07                    	cmpb	$0x7, %bl
14016292a: 0f 85 bd 02 00 00           	jne	0x140162bed <.text+0x161bed>
140162930: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162937: 48 8d 8d 10 1a 00 00        	leaq	0x1a10(%rbp), %rcx
14016293e: 48 ba 01 00 00 00 01 00 00 00       	movabsq	$0x100000001, %rdx # imm = 0x100000001
140162948: 41 b8 09 00 00 00           	movl	$0x9, %r8d
14016294e: e8 4d 29 a7 00              	callq	0x140bd52a0 <.text+0xbd42a0>
140162953: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
14016295a: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
140162961: 0f 29 8d 60 14 00 00        	movaps	%xmm1, 0x1460(%rbp)
140162968: 0f 29 85 50 14 00 00        	movaps	%xmm0, 0x1450(%rbp)
14016296f: 48 8d 8d 20 18 00 00        	leaq	0x1820(%rbp), %rcx
140162976: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016297d: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140162984: e8 57 43 a7 00              	callq	0x140bd6ce0 <.text+0xbd5ce0>
140162989: 48 8d 05 18 af 7f 02        	leaq	0x27faf18(%rip), %rax   # 0x14295d8a8
140162990: 48 89 85 10 18 00 00        	movq	%rax, 0x1810(%rbp)
140162997: 48 c7 85 18 18 00 00 03 00 00 00    	movq	$0x3, 0x1818(%rbp)
1401629a2: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401629a9: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401629b0: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
1401629b7: e8 14 6b a8 00              	callq	0x140be94d0 <.text+0xbe84d0>
1401629bc: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
1401629c3: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
1401629ca: 0f 11 85 b4 17 00 00        	movups	%xmm0, 0x17b4(%rbp)
1401629d1: 0f 11 8d c4 17 00 00        	movups	%xmm1, 0x17c4(%rbp)
1401629d8: 89 85 b0 17 00 00           	movl	%eax, 0x17b0(%rbp)
1401629de: e9 41 02 00 00              	jmp	0x140162c24 <.text+0x161c24>
1401629e3: 49 8b 16                    	movq	(%r14), %rdx
1401629e6: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401629ed: 48 8d 8d 10 1a 00 00        	leaq	0x1a10(%rbp), %rcx
1401629f4: 41 b8 82 00 00 00           	movl	$0x82, %r8d
1401629fa: e8 a1 28 a7 00              	callq	0x140bd52a0 <.text+0xbd42a0>
1401629ff: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
140162a06: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
140162a0d: 0f 29 8d 60 14 00 00        	movaps	%xmm1, 0x1460(%rbp)
140162a14: 0f 29 85 50 14 00 00        	movaps	%xmm0, 0x1450(%rbp)
140162a1b: 48 8d 8d 20 18 00 00        	leaq	0x1820(%rbp), %rcx
140162a22: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162a29: 4c 8d ad 50 14 00 00        	leaq	0x1450(%rbp), %r13
140162a30: 4c 89 ea                    	movq	%r13, %rdx
140162a33: e8 a8 42 a7 00              	callq	0x140bd6ce0 <.text+0xbd5ce0>
140162a38: 48 8d 05 59 ad 7f 02        	leaq	0x27fad59(%rip), %rax   # 0x14295d798
140162a3f: 48 89 85 10 18 00 00        	movq	%rax, 0x1810(%rbp)
140162a46: 48 c7 85 18 18 00 00 0a 00 00 00    	movq	$0xa, 0x1818(%rbp)
140162a51: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162a58: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140162a5f: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
140162a66: e8 65 6a a8 00              	callq	0x140be94d0 <.text+0xbe84d0>
140162a6b: 0f 10 85 10 1a 00 00        	movups	0x1a10(%rbp), %xmm0
140162a72: 0f 10 8d 20 1a 00 00        	movups	0x1a20(%rbp), %xmm1
140162a79: 0f 11 85 d0 13 00 00        	movups	%xmm0, 0x13d0(%rbp)
140162a80: 0f 11 8d e0 13 00 00        	movups	%xmm1, 0x13e0(%rbp)
140162a87: 89 85 cc 13 00 00           	movl	%eax, 0x13cc(%rbp)
140162a8d: 49 8b 06                    	movq	(%r14), %rax
140162a90: 8b 8d e0 19 00 00           	movl	0x19e0(%rbp), %ecx
140162a96: 89 8d 70 14 00 00           	movl	%ecx, 0x1470(%rbp)
140162a9c: 0f 10 85 c0 19 00 00        	movups	0x19c0(%rbp), %xmm0
140162aa3: 0f 10 8d d0 19 00 00        	movups	0x19d0(%rbp), %xmm1
140162aaa: 0f 29 8d 60 14 00 00        	movaps	%xmm1, 0x1460(%rbp)
140162ab1: 0f 29 85 50 14 00 00        	movaps	%xmm0, 0x1450(%rbp)
140162ab8: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162abf: 4c 89 6c 24 38              	movq	%r13, 0x38(%rsp)
140162ac4: 48 8d 8d cc 13 00 00        	leaq	0x13cc(%rbp), %rcx
140162acb: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
140162ad0: 48 8b 8d f0 19 00 00        	movq	0x19f0(%rbp), %rcx
140162ad7: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140162adc: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140162ae1: 48 8d 8d 10 18 00 00        	leaq	0x1810(%rbp), %rcx
140162ae8: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
140162aef: 4d 89 f8                    	movq	%r15, %r8
140162af2: 4c 8b 8d 00 1a 00 00        	movq	0x1a00(%rbp), %r9
140162af9: e8 12 96 38 00              	callq	0x1404ec110 <.text+0x4eb110>
140162afe: 0f 10 85 10 18 00 00        	movups	0x1810(%rbp), %xmm0
140162b05: 0f 10 8d 20 18 00 00        	movups	0x1820(%rbp), %xmm1
140162b0c: 0f 29 85 b0 15 00 00        	movaps	%xmm0, 0x15b0(%rbp)
140162b13: 0f 29 8d c0 15 00 00        	movaps	%xmm1, 0x15c0(%rbp)
140162b1a: 0f b7 85 30 18 00 00        	movzwl	0x1830(%rbp), %eax
140162b21: 66 89 85 d0 15 00 00        	movw	%ax, 0x15d0(%rbp)
140162b28: 0f b6 85 32 18 00 00        	movzbl	0x1832(%rbp), %eax
140162b2f: 0f b6 8d 33 18 00 00        	movzbl	0x1833(%rbp), %ecx
140162b36: 0f 10 85 34 18 00 00        	movups	0x1834(%rbp), %xmm0
140162b3d: 0f 29 85 80 15 00 00        	movaps	%xmm0, 0x1580(%rbp)
140162b44: 0f 10 85 44 18 00 00        	movups	0x1844(%rbp), %xmm0
140162b4b: 0f 29 85 90 15 00 00        	movaps	%xmm0, 0x1590(%rbp)
140162b52: 0f b7 95 54 18 00 00        	movzwl	0x1854(%rbp), %edx
140162b59: 66 89 95 a0 15 00 00        	movw	%dx, 0x15a0(%rbp)
140162b60: 0f b6 9d 56 18 00 00        	movzbl	0x1856(%rbp), %ebx
140162b67: 0f b6 bd 57 18 00 00        	movzbl	0x1857(%rbp), %edi
140162b6e: 8b 95 78 18 00 00           	movl	0x1878(%rbp), %edx
140162b74: 89 95 e0 19 00 00           	movl	%edx, 0x19e0(%rbp)
140162b7a: 0f 10 85 68 18 00 00        	movups	0x1868(%rbp), %xmm0
140162b81: 0f 29 85 d0 19 00 00        	movaps	%xmm0, 0x19d0(%rbp)
140162b88: 0f 10 85 58 18 00 00        	movups	0x1858(%rbp), %xmm0
140162b8f: 0f 29 85 c0 19 00 00        	movaps	%xmm0, 0x19c0(%rbp)
140162b96: b2 01                       	movb	$0x1, %dl
140162b98: 89 95 80 1a 00 00           	movl	%edx, 0x1a80(%rbp)
140162b9e: 3c 07                       	cmpb	$0x7, %al
140162ba0: 0f 84 da fc ff ff           	je	0x140162880 <.text+0x161880>
140162ba6: 0f b7 95 d0 15 00 00        	movzwl	0x15d0(%rbp), %edx
140162bad: 66 89 95 00 18 00 00        	movw	%dx, 0x1800(%rbp)
140162bb4: 0f 28 85 b0 15 00 00        	movaps	0x15b0(%rbp), %xmm0
140162bbb: 0f 28 8d c0 15 00 00        	movaps	0x15c0(%rbp), %xmm1
140162bc2: 0f 29 8d f0 17 00 00        	movaps	%xmm1, 0x17f0(%rbp)
140162bc9: 0f 29 85 e0 17 00 00        	movaps	%xmm0, 0x17e0(%rbp)
140162bd0: 88 85 02 18 00 00           	movb	%al, 0x1802(%rbp)
140162bd6: 88 8d 03 18 00 00           	movb	%cl, 0x1803(%rbp)
140162bdc: b0 01                       	movb	$0x1, %al
140162bde: 89 85 80 1a 00 00           	movl	%eax, 0x1a80(%rbp)
140162be4: 80 fb 07                    	cmpb	$0x7, %bl
140162be7: 0f 84 43 fd ff ff           	je	0x140162930 <.text+0x161930>
140162bed: 0f b7 85 a0 15 00 00        	movzwl	0x15a0(%rbp), %eax
140162bf4: 66 89 85 d0 17 00 00        	movw	%ax, 0x17d0(%rbp)
140162bfb: 0f 28 85 80 15 00 00        	movaps	0x1580(%rbp), %xmm0
140162c02: 0f 28 8d 90 15 00 00        	movaps	0x1590(%rbp), %xmm1
140162c09: 0f 29 8d c0 17 00 00        	movaps	%xmm1, 0x17c0(%rbp)
140162c10: 0f 29 85 b0 17 00 00        	movaps	%xmm0, 0x17b0(%rbp)
140162c17: 88 9d d2 17 00 00           	movb	%bl, 0x17d2(%rbp)
140162c1d: 40 88 bd d3 17 00 00        	movb	%dil, 0x17d3(%rbp)
140162c24: b1 01                       	movb	$0x1, %cl
140162c26: 31 c0                       	xorl	%eax, %eax
140162c28: f0                          	lock
140162c29: 0f b0 0d 20 84 4c 03        	cmpxchgb	%cl, 0x34c8420(%rip)    # 0x14362b050
140162c30: 0f 85 a0 12 00 00           	jne	0x140163ed6 <.text+0x162ed6>
140162c36: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162c3d: 48 8d 0d 0c 84 4c 03        	leaq	0x34c840c(%rip), %rcx   # 0x14362b050
140162c44: e8 27 f0 b1 00              	callq	0x140c81c70 <.text+0xc80c70>
140162c49: 48 8b 85 b0 19 00 00        	movq	0x19b0(%rbp), %rax
140162c50: 48 8d 98 90 00 00 00        	leaq	0x90(%rax), %rbx
140162c57: 48 8b 05 02 84 4c 03        	movq	0x34c8402(%rip), %rax   # 0x14362b060
140162c5e: 48 69 15 ff 83 4c 03 98 00 00 00    	imulq	$0x98, 0x34c83ff(%rip), %rdx # 0x14362b068
140162c69: 31 c9                       	xorl	%ecx, %ecx
140162c6b: 49 89 c0                    	movq	%rax, %r8
140162c6e: 66 90                       	nop
140162c70: 48 39 ca                    	cmpq	%rcx, %rdx
140162c73: 74 43                       	je	0x140162cb8 <.text+0x161cb8>
140162c75: 48 81 c1 98 00 00 00        	addq	$0x98, %rcx
140162c7c: 41 80 b8 96 00 00 00 06     	cmpb	$0x6, 0x96(%r8)
140162c84: 4d 8d 80 98 00 00 00        	leaq	0x98(%r8), %r8
140162c8b: 75 e3                       	jne	0x140162c70 <.text+0x161c70>
140162c8d: 0f b6 7c 08 b4              	movzbl	-0x4c(%rax,%rcx), %edi
140162c92: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162c99: 48 8d 0d b0 83 4c 03        	leaq	0x34c83b0(%rip), %rcx   # 0x14362b050
140162ca0: e8 6b f1 b1 00              	callq	0x140c81e10 <.text+0xc80e10>
140162ca5: 31 c9                       	xorl	%ecx, %ecx
140162ca7: b0 01                       	movb	$0x1, %al
140162ca9: f0                          	lock
140162caa: 0f b0 0d 9f 83 4c 03        	cmpxchgb	%cl, 0x34c839f(%rip)    # 0x14362b050
140162cb1: 74 2d                       	je	0x140162ce0 <.text+0x161ce0>
140162cb3: e9 3c 12 00 00              	jmp	0x140163ef4 <.text+0x162ef4>
140162cb8: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162cbf: 48 8d 0d 8a 83 4c 03        	leaq	0x34c838a(%rip), %rcx   # 0x14362b050
140162cc6: e8 45 f1 b1 00              	callq	0x140c81e10 <.text+0xc80e10>
140162ccb: 31 c9                       	xorl	%ecx, %ecx
140162ccd: b0 01                       	movb	$0x1, %al
140162ccf: f0                          	lock
140162cd0: 0f b0 0d 79 83 4c 03        	cmpxchgb	%cl, 0x34c8379(%rip)    # 0x14362b050
140162cd7: 40 b7 07                    	movb	$0x7, %dil
140162cda: 0f 85 14 12 00 00           	jne	0x140163ef4 <.text+0x162ef4>
140162ce0: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162ce7: 48 ba b9 75 23 64 09 2e 14 4a       	movabsq	$0x4a142e09642375b9, %rdx # imm = 0x4A142E09642375B9
140162cf1: 49 b8 7f 84 8c fa 4c 15 f6 43       	movabsq	$0x43f6154cfa8c847f, %r8 # imm = 0x43F6154CFA8C847F
140162cfb: 4c 89 e1                    	movq	%r12, %rcx
140162cfe: e8 ed a9 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140162d03: 48 85 c0                    	testq	%rax, %rax
140162d06: 0f 84 aa 12 00 00           	je	0x140163fb6 <.text+0x162fb6>
140162d0c: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140162d13: 0f 86 9d 12 00 00           	jbe	0x140163fb6 <.text+0x162fb6>
140162d19: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140162d20: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140162d24: 48 85 c9                    	testq	%rcx, %rcx
140162d27: 0f 84 89 12 00 00           	je	0x140163fb6 <.text+0x162fb6>
140162d2d: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140162d34: 48 f7 d1                    	notq	%rcx
140162d37: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140162d3b: 48 c1 e1 05                 	shlq	$0x5, %rcx
140162d3f: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140162d45: 0f 84 6b 12 00 00           	je	0x140163fb6 <.text+0x162fb6>
140162d4b: 48 01 c8                    	addq	%rcx, %rax
140162d4e: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140162d52: 40 80 ff 07                 	cmpb	$0x7, %dil
140162d56: 0f 84 34 01 00 00           	je	0x140162e90 <.text+0x161e90>
140162d5c: 8d 47 fe                    	leal	-0x2(%rdi), %eax
140162d5f: 3c 05                       	cmpb	$0x5, %al
140162d61: 0f 93 c2                    	setae	%dl
140162d64: 3c 03                       	cmpb	$0x3, %al
140162d66: 0f 94 c0                    	sete	%al
140162d69: 08 d0                       	orb	%dl, %al
140162d6b: 40 84 f8                    	testb	%dil, %al
140162d6e: 0f 84 1c 01 00 00           	je	0x140162e90 <.text+0x161e90>
140162d74: 48 8b 05 2d 96 4c 03        	movq	0x34c962d(%rip), %rax   # 0x14362c3a8
140162d7b: 48 8b 00                    	movq	(%rax), %rax
140162d7e: 48 83 f8 03                 	cmpq	$0x3, %rax
140162d82: 0f 82 fa 00 00 00           	jb	0x140162e82 <.text+0x161e82>
140162d88: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162d8f: 49 89 cd                    	movq	%rcx, %r13
140162d92: e8 e9 af 51 00              	callq	0x14067dd80 <.text+0x67cd80>
140162d97: 48 89 85 50 14 00 00        	movq	%rax, 0x1450(%rbp)
140162d9e: 48 8d 05 2b 8d ed ff        	leaq	-0x1272d5(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140162da5: 48 89 85 58 14 00 00        	movq	%rax, 0x1458(%rbp)
140162dac: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162db3: 48 8d 0d 46 aa 7f 02        	leaq	0x27faa46(%rip), %rcx   # 0x14295d800
140162dba: e8 c1 af 51 00              	callq	0x14067dd80 <.text+0x67cd80>
140162dbf: 0f 10 00                    	movups	(%rax), %xmm0
140162dc2: 8b 40 10                    	movl	0x10(%rax), %eax
140162dc5: 48 c7 85 40 18 00 00 03 00 00 00    	movq	$0x3, 0x1840(%rbp)
140162dd0: 48 8d 0d 41 aa 7f 02        	leaq	0x27faa41(%rip), %rcx   # 0x14295d818
140162dd7: 48 89 8d 48 18 00 00        	movq	%rcx, 0x1848(%rbp)
140162dde: 48 c7 85 50 18 00 00 17 00 00 00    	movq	$0x17, 0x1850(%rbp)
140162de9: 48 8d 15 00 aa 7f 02        	leaq	0x27faa00(%rip), %rdx   # 0x14295d7f0
140162df0: 48 89 95 60 18 00 00        	movq	%rdx, 0x1860(%rbp)
140162df7: 48 c7 85 68 18 00 00 01 00 00 00    	movq	$0x1, 0x1868(%rbp)
140162e02: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140162e09: 48 89 95 70 18 00 00        	movq	%rdx, 0x1870(%rbp)
140162e10: 48 c7 85 78 18 00 00 01 00 00 00    	movq	$0x1, 0x1878(%rbp)
140162e1b: 48 c7 85 80 18 00 00 00 00 00 00    	movq	$0x0, 0x1880(%rbp)
140162e26: 48 c7 85 10 18 00 00 00 00 00 00    	movq	$0x0, 0x1810(%rbp)
140162e31: 48 89 8d 18 18 00 00        	movq	%rcx, 0x1818(%rbp)
140162e38: 48 c7 85 20 18 00 00 17 00 00 00    	movq	$0x17, 0x1820(%rbp)
140162e43: 48 c7 85 28 18 00 00 00 00 00 00    	movq	$0x0, 0x1828(%rbp)
140162e4e: 0f 11 85 30 18 00 00        	movups	%xmm0, 0x1830(%rbp)
140162e55: c7 85 58 18 00 00 01 00 00 00       	movl	$0x1, 0x1858(%rbp)
140162e5f: 89 85 5c 18 00 00           	movl	%eax, 0x185c(%rbp)
140162e65: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162e6c: 48 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %rcx
140162e73: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
140162e7a: e8 b1 74 be 00              	callq	0x140d4a330 <.text+0xd49330>
140162e7f: 4c 89 e9                    	movq	%r13, %rcx
140162e82: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162e89: e8 f2 ae 51 00              	callq	0x14067dd80 <.text+0x67cd80>
140162e8e: eb 0c                       	jmp	0x140162e9c <.text+0x161e9c>
140162e90: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162e97: e8 b4 bb 74 00              	callq	0x1408aea50 <.text+0x8ada50>
140162e9c: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140162ea3: 48 8d 8d 10 1a 00 00        	leaq	0x1a10(%rbp), %rcx
140162eaa: 48 89 c2                    	movq	%rax, %rdx
140162ead: e8 6e dc 99 00              	callq	0x140b00b20 <.text+0xaffb20>
140162eb2: 40 80 ff 07                 	cmpb	$0x7, %dil
140162eb6: 48 89 b5 a0 17 00 00        	movq	%rsi, 0x17a0(%rbp)
140162ebd: 74 21                       	je	0x140162ee0 <.text+0x161ee0>
140162ebf: 8d 47 fe                    	leal	-0x2(%rdi), %eax
140162ec2: 3c 05                       	cmpb	$0x5, %al
140162ec4: 0f 93 c1                    	setae	%cl
140162ec7: 3c 03                       	cmpb	$0x3, %al
140162ec9: 0f 94 c0                    	sete	%al
140162ecc: 08 c8                       	orb	%cl, %al
140162ece: 74 10                       	je	0x140162ee0 <.text+0x161ee0>
140162ed0: 89 f8                       	movl	%edi, %eax
140162ed2: 24 01                       	andb	$0x1, %al
140162ed4: 0f b6 c0                    	movzbl	%al, %eax
140162ed7: 48 89 85 f8 19 00 00        	movq	%rax, 0x19f8(%rbp)
140162ede: eb 45                       	jmp	0x140162f25 <.text+0x161f25>
140162ee0: c6 85 8d 1a 00 00 01        	movb	$0x1, 0x1a8d(%rbp)
140162ee7: 48 8d 8d 10 18 00 00        	leaq	0x1810(%rbp), %rcx
140162eee: 48 8d 95 10 1a 00 00        	leaq	0x1a10(%rbp), %rdx
140162ef5: e8 96 8e c1 00              	callq	0x140d7bd90 <.text+0xd7ad90>
140162efa: f3 0f 10 b5 28 1a 00 00     	movss	0x1a28(%rbp), %xmm6
140162f02: 48 8b 85 10 18 00 00        	movq	0x1810(%rbp), %rax
140162f09: 48 89 85 08 1a 00 00        	movq	%rax, 0x1a08(%rbp)
140162f10: 48 8b 85 18 18 00 00        	movq	0x1818(%rbp), %rax
140162f17: 48 89 85 f8 19 00 00        	movq	%rax, 0x19f8(%rbp)
140162f1e: 48 8b b5 20 18 00 00        	movq	0x1820(%rbp), %rsi
140162f25: c6 85 96 1a 00 00 01        	movb	$0x1, 0x1a96(%rbp)
140162f2c: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140162f33: 4c 8d ad 74 11 00 00        	leaq	0x1174(%rbp), %r13
140162f3a: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
140162f41: 4c 8d 8d c0 19 00 00        	leaq	0x19c0(%rbp), %r9
140162f48: 4c 89 e9                    	movq	%r13, %rcx
140162f4b: 4d 89 f8                    	movq	%r15, %r8
140162f4e: e8 0d a9 88 00              	callq	0x1409ed860 <.text+0x9ec860>
140162f53: 48 8b 8d 80 17 00 00        	movq	0x1780(%rbp), %rcx
140162f5a: 48 8d 41 08                 	leaq	0x8(%rcx), %rax
140162f5e: f6 01 01                    	testb	$0x1, (%rcx)
140162f61: 4c 0f 45 f0                 	cmovneq	%rax, %r14
140162f65: 4d 8b 0e                    	movq	(%r14), %r9
140162f68: 8b 85 e0 19 00 00           	movl	0x19e0(%rbp), %eax
140162f6e: 89 85 30 18 00 00           	movl	%eax, 0x1830(%rbp)
140162f74: 0f 28 85 c0 19 00 00        	movaps	0x19c0(%rbp), %xmm0
140162f7b: 0f 28 8d d0 19 00 00        	movaps	0x19d0(%rbp), %xmm1
140162f82: 0f 29 8d 20 18 00 00        	movaps	%xmm1, 0x1820(%rbp)
140162f89: 0f 29 85 10 18 00 00        	movaps	%xmm0, 0x1810(%rbp)
140162f90: 48 8b 85 08 1a 00 00        	movq	0x1a08(%rbp), %rax
140162f97: 48 89 85 50 14 00 00        	movq	%rax, 0x1450(%rbp)
140162f9e: 48 8b 85 f8 19 00 00        	movq	0x19f8(%rbp), %rax
140162fa5: 48 89 85 58 14 00 00        	movq	%rax, 0x1458(%rbp)
140162fac: 48 89 b5 60 14 00 00        	movq	%rsi, 0x1460(%rbp)
140162fb3: f3 0f 11 b5 68 14 00 00     	movss	%xmm6, 0x1468(%rbp)
140162fbb: c6 85 6c 14 00 00 00        	movb	$0x0, 0x146c(%rbp)
140162fc2: 48 8b 85 f0 19 00 00        	movq	0x19f0(%rbp), %rax
140162fc9: f3 0f 10 80 9c 00 00 00     	movss	0x9c(%rax), %xmm0
140162fd1: f3 0f 10 88 a4 00 00 00     	movss	0xa4(%rax), %xmm1
140162fd9: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140162fe0: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140162fe7: f3 0f 11 4c 24 48           	movss	%xmm1, 0x48(%rsp)
140162fed: f3 0f 11 44 24 40           	movss	%xmm0, 0x40(%rsp)
140162ff3: 48 8d 85 50 14 00 00        	leaq	0x1450(%rbp), %rax
140162ffa: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140162fff: 8b 85 80 1a 00 00           	movl	0x1a80(%rbp), %eax
140163005: 88 44 24 30                 	movb	%al, 0x30(%rsp)
140163009: 4c 89 6c 24 28              	movq	%r13, 0x28(%rsp)
14016300e: 48 8d 85 10 18 00 00        	leaq	0x1810(%rbp), %rax
140163015: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14016301a: 48 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %rcx
140163021: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
140163028: 4d 89 f8                    	movq	%r15, %r8
14016302b: e8 d0 9f 7a 00              	callq	0x14090d000 <.text+0x90c000>
140163030: 48 c7 85 18 18 00 00 02 00 00 00    	movq	$0x2, 0x1818(%rbp)
14016303b: 66 c7 85 20 18 00 00 00 01  	movw	$0x100, 0x1820(%rbp)    # imm = 0x100
140163044: 31 c0                       	xorl	%eax, %eax
140163046: 4c 8d ad e0 15 00 00        	leaq	0x15e0(%rbp), %r13
14016304d: 4c 8d b5 50 1a 00 00        	leaq	0x1a50(%rbp), %r14
140163054: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140163060: 48 83 f8 02                 	cmpq	$0x2, %rax
140163064: 74 4b                       	je	0x1401630b1 <.text+0x1620b1>
140163066: 48 8d 70 01                 	leaq	0x1(%rax), %rsi
14016306a: 48 89 b5 10 18 00 00        	movq	%rsi, 0x1810(%rbp)
140163071: 44 0f b6 84 05 20 18 00 00  	movzbl	0x1820(%rbp,%rax), %r8d
14016307a: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163081: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163088: 4c 89 e9                    	movq	%r13, %rcx
14016308b: 4c 89 fa                    	movq	%r15, %rdx
14016308e: 4d 89 f1                    	movq	%r14, %r9
140163091: e8 aa a0 1b 00              	callq	0x14031d140 <.text+0x31c140>
140163096: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
14016309d: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401630a4: 48 89 c1                    	movq	%rax, %rcx
1401630a7: e8 04 8f 38 00              	callq	0x1404ebfb0 <.text+0x4eafb0>
1401630ac: 48 89 f0                    	movq	%rsi, %rax
1401630af: eb af                       	jmp	0x140163060 <.text+0x162060>
1401630b1: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401630b8: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401630bf: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401630c6: 4c 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %r9
1401630cd: 4c 89 fa                    	movq	%r15, %rdx
1401630d0: 41 b0 02                    	movb	$0x2, %r8b
1401630d3: e8 68 a0 1b 00              	callq	0x14031d140 <.text+0x31c140>
1401630d8: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401630df: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401630e6: 48 89 c1                    	movq	%rax, %rcx
1401630e9: e8 c2 8e 38 00              	callq	0x1404ebfb0 <.text+0x4eafb0>
1401630ee: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401630f5: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401630fc: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163103: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
14016310a: 4c 89 fa                    	movq	%r15, %rdx
14016310d: e8 1e 69 76 00              	callq	0x1408c9a30 <.text+0x8c8a30>
140163112: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163119: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163120: 8b 85 48 1a 00 00           	movl	0x1a48(%rbp), %eax
140163126: 88 44 24 28                 	movb	%al, 0x28(%rsp)
14016312a: 48 8d 85 b0 17 00 00        	leaq	0x17b0(%rbp), %rax
140163131: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140163136: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
14016313d: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
140163144: 4c 8d 8d e0 17 00 00        	leaq	0x17e0(%rbp), %r9
14016314b: 4c 89 fa                    	movq	%r15, %rdx
14016314e: e8 bd 32 74 00              	callq	0x1408a6410 <.text+0x8a5410>
140163153: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
14016315a: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163161: 48 ba 28 64 1a c9 41 e9 b8 c2       	movabsq	$-0x3d4716be36e59bd8, %rdx # imm = 0xC2B8E941C91A6428
14016316b: 49 b8 a0 43 da d3 d1 c6 f3 59       	movabsq	$0x59f3c6d1d3da43a0, %r8 # imm = 0x59F3C6D1D3DA43A0
140163175: 4c 89 e1                    	movq	%r12, %rcx
140163178: e8 73 a5 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
14016317d: 48 85 c0                    	testq	%rax, %rax
140163180: 0f 84 54 0e 00 00           	je	0x140163fda <.text+0x162fda>
140163186: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
14016318d: 0f 86 47 0e 00 00           	jbe	0x140163fda <.text+0x162fda>
140163193: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
14016319a: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14016319e: 48 85 c9                    	testq	%rcx, %rcx
1401631a1: 0f 84 33 0e 00 00           	je	0x140163fda <.text+0x162fda>
1401631a7: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
1401631ae: 48 f7 d1                    	notq	%rcx
1401631b1: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1401631b5: 48 c1 e1 05                 	shlq	$0x5, %rcx
1401631b9: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1401631bf: 0f 84 15 0e 00 00           	je	0x140163fda <.text+0x162fda>
1401631c5: 48 01 c8                    	addq	%rcx, %rax
1401631c8: 48 8b 40 10                 	movq	0x10(%rax), %rax
1401631cc: 44 8b 30                    	movl	(%rax), %r14d
1401631cf: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401631d6: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401631dd: 48 ba 8b 04 7b e6 61 26 c0 a4       	movabsq	$-0x5b3fd99e1984fb75, %rdx # imm = 0xA4C02661E67B048B
1401631e7: 49 b8 ec 67 2a aa e6 61 e7 ea       	movabsq	$-0x15189e1955d59814, %r8 # imm = 0xEAE761E6AA2A67EC
1401631f1: 4c 89 e1                    	movq	%r12, %rcx
1401631f4: e8 f7 a4 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1401631f9: 48 85 c0                    	testq	%rax, %rax
1401631fc: 0f 84 f7 0d 00 00           	je	0x140163ff9 <.text+0x162ff9>
140163202: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140163209: 0f 86 ea 0d 00 00           	jbe	0x140163ff9 <.text+0x162ff9>
14016320f: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140163216: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
14016321a: 48 85 c9                    	testq	%rcx, %rcx
14016321d: 0f 84 d6 0d 00 00           	je	0x140163ff9 <.text+0x162ff9>
140163223: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
14016322a: 48 f7 d1                    	notq	%rcx
14016322d: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140163231: 48 c1 e1 05                 	shlq	$0x5, %rcx
140163235: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
14016323b: 0f 84 b8 0d 00 00           	je	0x140163ff9 <.text+0x162ff9>
140163241: 48 01 c8                    	addq	%rcx, %rax
140163244: 41 83 fe 13                 	cmpl	$0x13, %r14d
140163248: 41 0f 94 c1                 	sete	%r9b
14016324c: 48 8b 40 10                 	movq	0x10(%rax), %rax
140163250: 0f b6 40 20                 	movzbl	0x20(%rax), %eax
140163254: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
14016325b: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163262: 88 44 24 20                 	movb	%al, 0x20(%rsp)
140163266: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
14016326d: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
140163274: 4c 89 fa                    	movq	%r15, %rdx
140163277: e8 74 50 74 00              	callq	0x1408a82f0 <.text+0x8a72f0>
14016327c: 89 c6                       	movl	%eax, %esi
14016327e: 41 83 fe 13                 	cmpl	$0x13, %r14d
140163282: 41 0f 95 c6                 	setne	%r14b
140163286: 48 8b 05 0b 8e 4c 03        	movq	0x34c8e0b(%rip), %rax   # 0x14362c098
14016328d: 80 38 01                    	cmpb	$0x1, (%rax)
140163290: 75 0c                       	jne	0x14016329e <.text+0x16229e>
140163292: 40 80 c7 fd                 	addb	$-0x3, %dil
140163296: 40 80 ff 02                 	cmpb	$0x2, %dil
14016329a: 72 18                       	jb	0x1401632b4 <.text+0x1622b4>
14016329c: eb 77                       	jmp	0x140163315 <.text+0x162315>
14016329e: 40 80 ff 03                 	cmpb	$0x3, %dil
1401632a2: 75 71                       	jne	0x140163315 <.text+0x162315>
1401632a4: 48 8b 85 f0 19 00 00        	movq	0x19f0(%rbp), %rax
1401632ab: 80 b8 f3 00 00 00 01        	cmpb	$0x1, 0xf3(%rax)
1401632b2: 75 24                       	jne	0x1401632d8 <.text+0x1622d8>
1401632b4: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401632bb: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401632c2: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401632c9: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
1401632d0: 4c 89 fa                    	movq	%r15, %rdx
1401632d3: e8 78 2c 74 00              	callq	0x1408a5f50 <.text+0x8a4f50>
1401632d8: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401632df: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401632e6: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401632ed: 4c 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %r9
1401632f4: 4c 89 fa                    	movq	%r15, %rdx
1401632f7: 41 b0 03                    	movb	$0x3, %r8b
1401632fa: e8 41 9e 1b 00              	callq	0x14031d140 <.text+0x31c140>
1401632ff: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163306: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
14016330d: 48 89 c1                    	movq	%rax, %rcx
140163310: e8 9b 8c 38 00              	callq	0x1404ebfb0 <.text+0x4eafb0>
140163315: 44 20 f6                    	andb	%r14b, %sil
140163318: 0f 85 91 01 00 00           	jne	0x1401634af <.text+0x1624af>
14016331e: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163325: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
14016332c: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163333: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
14016333a: 4c 89 fa                    	movq	%r15, %rdx
14016333d: e8 be 38 79 00              	callq	0x1408f6c00 <.text+0x8f5c00>
140163342: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163349: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163350: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163357: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
14016335e: 4c 89 fa                    	movq	%r15, %rdx
140163361: e8 fa d1 74 00              	callq	0x1408b0560 <.text+0x8af560>
140163366: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
14016336d: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163374: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
14016337b: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
140163382: 4c 89 fa                    	movq	%r15, %rdx
140163385: e8 e6 26 74 00              	callq	0x1408a5a70 <.text+0x8a4a70>
14016338a: 80 bd 80 1a 00 00 00        	cmpb	$0x0, 0x1a80(%rbp)
140163391: 74 56                       	je	0x1401633e9 <.text+0x1623e9>
140163393: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
14016339a: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401633a1: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401633a8: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
1401633af: 4c 89 fa                    	movq	%r15, %rdx
1401633b2: 4c 8b 8d 00 1a 00 00        	movq	0x1a00(%rbp), %r9
1401633b9: e8 62 97 86 00              	callq	0x1409ccb20 <.text+0x9cbb20>
1401633be: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401633c5: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401633cc: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401633d3: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
1401633da: 4c 89 fa                    	movq	%r15, %rdx
1401633dd: 4c 8b 8d 00 1a 00 00        	movq	0x1a00(%rbp), %r9
1401633e4: e8 57 48 6d 00              	callq	0x140837c40 <.text+0x836c40>
1401633e9: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401633f0: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401633f7: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401633fe: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
140163405: 4c 89 fa                    	movq	%r15, %rdx
140163408: e8 63 10 86 00              	callq	0x1409c4470 <.text+0x9c3470>
14016340d: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163414: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
14016341b: 48 8b 8d a0 17 00 00        	movq	0x17a0(%rbp), %rcx
140163422: e8 d9 c7 7d 00              	callq	0x14093fc00 <.text+0x93ec00>
140163427: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
14016342e: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163435: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
14016343c: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
140163443: 4c 89 fa                    	movq	%r15, %rdx
140163446: 41 89 c1                    	movl	%eax, %r9d
140163449: e8 b2 32 86 00              	callq	0x1409c6700 <.text+0x9c5700>
14016344e: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163455: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
14016345c: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163463: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
14016346a: 4c 89 fa                    	movq	%r15, %rdx
14016346d: e8 9e e2 86 00              	callq	0x1409d1710 <.text+0x9d0710>
140163472: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163479: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163480: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163487: 4c 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %r9
14016348e: 4c 89 fa                    	movq	%r15, %rdx
140163491: 41 b0 04                    	movb	$0x4, %r8b
140163494: e8 a7 9c 1b 00              	callq	0x14031d140 <.text+0x31c140>
140163499: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401634a0: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401634a7: 48 89 c1                    	movq	%rax, %rcx
1401634aa: e8 01 8b 38 00              	callq	0x1404ebfb0 <.text+0x4eafb0>
1401634af: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401634b6: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401634bd: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
1401634c4: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
1401634cb: 4c 89 fa                    	movq	%r15, %rdx
1401634ce: 4c 8b 8d 00 1a 00 00        	movq	0x1a00(%rbp), %r9
1401634d5: e8 16 4b 6d 00              	callq	0x140837ff0 <.text+0x836ff0>
1401634da: 48 83 bd b8 19 00 00 00     	cmpq	$0x0, 0x19b8(%rbp)
1401634e2: 74 2b                       	je	0x14016350f <.text+0x16250f>
1401634e4: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401634eb: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401634f2: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
1401634f9: 4c 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %r9
140163500: 48 8b 8d b8 19 00 00        	movq	0x19b8(%rbp), %rcx
140163507: 4d 89 f8                    	movq	%r15, %r8
14016350a: e8 71 a7 f7 ff              	callq	0x1400ddc80 <.text+0xdcc80>
14016350f: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163516: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
14016351d: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163524: 4c 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %r9
14016352b: 4c 89 fa                    	movq	%r15, %rdx
14016352e: 41 b0 06                    	movb	$0x6, %r8b
140163531: e8 0a 9c 1b 00              	callq	0x14031d140 <.text+0x31c140>
140163536: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
14016353d: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163544: 48 89 c1                    	movq	%rax, %rcx
140163547: e8 64 8a 38 00              	callq	0x1404ebfb0 <.text+0x4eafb0>
14016354c: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163553: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
14016355a: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163561: 4c 8d 85 50 1a 00 00        	leaq	0x1a50(%rbp), %r8
140163568: 4c 89 fa                    	movq	%r15, %rdx
14016356b: e8 20 67 42 00              	callq	0x140589c90 <.text+0x588c90>
140163570: 0f 10 85 50 1a 00 00        	movups	0x1a50(%rbp), %xmm0
140163577: 0f 10 8d 60 1a 00 00        	movups	0x1a60(%rbp), %xmm1
14016357e: 0f 29 85 10 18 00 00        	movaps	%xmm0, 0x1810(%rbp)
140163585: 8b 85 70 1a 00 00           	movl	0x1a70(%rbp), %eax
14016358b: 89 85 30 18 00 00           	movl	%eax, 0x1830(%rbp)
140163591: 0f 29 8d 20 18 00 00        	movaps	%xmm1, 0x1820(%rbp)
140163598: 44 8b b5 10 18 00 00        	movl	0x1810(%rbp), %r14d
14016359f: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401635a6: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
1401635ad: 48 8d 8d 10 18 00 00        	leaq	0x1810(%rbp), %rcx
1401635b4: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
1401635bb: 41 b0 22                    	movb	$0x22, %r8b
1401635be: e8 bd 2d a7 00              	callq	0x140bd6380 <.text+0xbd5380>
1401635c3: 48 8d b5 10 18 00 00        	leaq	0x1810(%rbp), %rsi
1401635ca: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
1401635d1: 41 b8 98 01 00 00           	movl	$0x198, %r8d            # imm = 0x198
1401635d7: 48 89 f1                    	movq	%rsi, %rcx
1401635da: e8 21 06 7c 02              	callq	0x142923c00 <.text+0x2922c00>
1401635df: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
1401635e6: c6 85 95 1a 00 00 00        	movb	$0x0, 0x1a95(%rbp)
1401635ed: 48 8d 8d 50 14 00 00        	leaq	0x1450(%rbp), %rcx
1401635f4: 48 89 f2                    	movq	%rsi, %rdx
1401635f7: e8 54 a4 87 00              	callq	0x1409dda50 <.text+0x9dca50>
1401635fc: 48 ba aa 4e 78 96 b3 61 85 a6       	movabsq	$-0x597a9e4c6987b156, %rdx # imm = 0xA68561B396784EAA
140163606: 49 b8 14 ed f4 fa e6 74 fd f5       	movabsq	$-0xa028b19050b12ec, %r8 # imm = 0xF5FD74E6FAF4ED14
140163610: 4c 89 e1                    	movq	%r12, %rcx
140163613: e8 d8 a0 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140163618: 48 8b bd b0 19 00 00        	movq	0x19b0(%rbp), %rdi
14016361f: 4c 8b ad 88 17 00 00        	movq	0x1788(%rbp), %r13
140163626: 48 85 c0                    	testq	%rax, %rax
140163629: 0f 84 e9 09 00 00           	je	0x140164018 <.text+0x163018>
14016362f: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140163636: 0f 86 dc 09 00 00           	jbe	0x140164018 <.text+0x163018>
14016363c: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140163643: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140163647: 48 85 c9                    	testq	%rcx, %rcx
14016364a: 0f 84 c8 09 00 00           	je	0x140164018 <.text+0x163018>
140163650: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140163657: 48 f7 d1                    	notq	%rcx
14016365a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14016365e: 48 c1 e1 05                 	shlq	$0x5, %rcx
140163662: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
140163668: 0f 84 aa 09 00 00           	je	0x140164018 <.text+0x163018>
14016366e: 48 01 c8                    	addq	%rcx, %rax
140163671: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140163675: 41 8b 97 b0 05 00 00        	movl	0x5b0(%r15), %edx
14016367c: 89 50 5c                    	movl	%edx, 0x5c(%rax)
14016367f: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140163686: e8 a5 97 7d 00              	callq	0x14093ce30 <.text+0x93be30>
14016368b: 48 ba 55 93 67 6e 78 cc 26 82       	movabsq	$-0x7dd9338791986cab, %rdx # imm = 0x8226CC786E679355
140163695: 49 b8 ac 41 a8 55 6a da cf 65       	movabsq	$0x65cfda6a55a841ac, %r8 # imm = 0x65CFDA6A55A841AC
14016369f: 4c 89 e1                    	movq	%r12, %rcx
1401636a2: e8 49 a0 f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
1401636a7: 48 85 c0                    	testq	%rax, %rax
1401636aa: 0f 84 79 09 00 00           	je	0x140164029 <.text+0x163029>
1401636b0: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
1401636b7: 0f 86 6c 09 00 00           	jbe	0x140164029 <.text+0x163029>
1401636bd: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
1401636c4: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
1401636c8: 48 85 c9                    	testq	%rcx, %rcx
1401636cb: 0f 84 58 09 00 00           	je	0x140164029 <.text+0x163029>
1401636d1: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
1401636d8: 48 f7 d1                    	notq	%rcx
1401636db: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1401636df: 48 c1 e1 05                 	shlq	$0x5, %rcx
1401636e3: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
1401636e9: 0f 84 3a 09 00 00           	je	0x140164029 <.text+0x163029>
1401636ef: 48 01 c8                    	addq	%rcx, %rax
1401636f2: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1401636f6: 41 8b 97 b0 05 00 00        	movl	0x5b0(%r15), %edx
1401636fd: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140163700: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140163707: e8 a4 b4 7b 00              	callq	0x14091ebb0 <.text+0x91dbb0>
14016370c: 48 ba ef a5 99 86 45 78 29 66       	movabsq	$0x662978458699a5ef, %rdx # imm = 0x662978458699A5EF
140163716: 49 b8 46 9b 41 62 c1 57 c0 3e       	movabsq	$0x3ec057c162419b46, %r8 # imm = 0x3EC057C162419B46
140163720: 4c 89 e1                    	movq	%r12, %rcx
140163723: e8 c8 9f f7 ff              	callq	0x1400dd6f0 <.text+0xdc6f0>
140163728: 48 85 c0                    	testq	%rax, %rax
14016372b: 0f 84 09 09 00 00           	je	0x14016403a <.text+0x16303a>
140163731: 49 39 97 18 02 00 00        	cmpq	%rdx, 0x218(%r15)
140163738: 0f 86 fc 08 00 00           	jbe	0x14016403a <.text+0x16303a>
14016373e: 49 8b 87 10 02 00 00        	movq	0x210(%r15), %rax
140163745: 48 8b 0c d0                 	movq	(%rax,%rdx,8), %rcx
140163749: 48 85 c9                    	testq	%rcx, %rcx
14016374c: 0f 84 e8 08 00 00           	je	0x14016403a <.text+0x16303a>
140163752: 49 8b 87 e0 01 00 00        	movq	0x1e0(%r15), %rax
140163759: 48 f7 d1                    	notq	%rcx
14016375c: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140163760: 48 c1 e1 05                 	shlq	$0x5, %rcx
140163764: 48 83 7c 08 20 00           	cmpq	$0x0, 0x20(%rax,%rcx)
14016376a: 0f 84 ca 08 00 00           	je	0x14016403a <.text+0x16303a>
140163770: 48 01 c8                    	addq	%rcx, %rax
140163773: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140163777: 41 8b 97 b0 05 00 00        	movl	0x5b0(%r15), %edx
14016377e: 89 50 5c                    	movl	%edx, 0x5c(%rax)
140163781: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140163788: e8 93 ea 82 00              	callq	0x140992220 <.text+0x991220>
14016378d: 48 8d 8d 10 18 00 00        	leaq	0x1810(%rbp), %rcx
140163794: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
14016379b: 41 b8 10 01 00 00           	movl	$0x110, %r8d            # imm = 0x110
1401637a1: e8 5a 04 7c 02              	callq	0x142923c00 <.text+0x2922c00>
1401637a6: 48 8b 95 10 1a 00 00        	movq	0x1a10(%rbp), %rdx
1401637ad: 48 85 d2                    	testq	%rdx, %rdx
1401637b0: 74 12                       	je	0x1401637c4 <.text+0x1627c4>
1401637b2: 48 8b 8d 18 1a 00 00        	movq	0x1a18(%rbp), %rcx
1401637b9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1401637bf: e8 5c 04 4b 00              	callq	0x140613c20 <.text+0x612c20>
1401637c4: 48 8d 75 d0                 	leaq	-0x30(%rbp), %rsi
1401637c8: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
1401637cf: 41 b8 10 01 00 00           	movl	$0x110, %r8d            # imm = 0x110
1401637d5: 48 89 f1                    	movq	%rsi, %rcx
1401637d8: e8 23 04 7c 02              	callq	0x142923c00 <.text+0x2922c00>
1401637dd: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1401637e4: 48 89 f2                    	movq	%rsi, %rdx
1401637e7: e8 b4 44 39 00              	callq	0x1404f7ca0 <.text+0x4f6ca0>
1401637ec: 48 8d 8d 50 14 00 00        	leaq	0x1450(%rbp), %rcx
1401637f3: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
1401637fa: 41 b8 a8 00 00 00           	movl	$0xa8, %r8d
140163800: e8 fb 03 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140163805: 0f 10 85 88 01 00 00        	movups	0x188(%rbp), %xmm0
14016380c: 0f 10 8d 98 01 00 00        	movups	0x198(%rbp), %xmm1
140163813: 0f 10 95 a8 01 00 00        	movups	0x1a8(%rbp), %xmm2
14016381a: 0f 10 9d b8 01 00 00        	movups	0x1b8(%rbp), %xmm3
140163821: 0f 29 85 f0 13 00 00        	movaps	%xmm0, 0x13f0(%rbp)
140163828: 0f 29 8d 00 14 00 00        	movaps	%xmm1, 0x1400(%rbp)
14016382f: 0f 29 95 10 14 00 00        	movaps	%xmm2, 0x1410(%rbp)
140163836: 0f 29 9d 20 14 00 00        	movaps	%xmm3, 0x1420(%rbp)
14016383d: 0f 10 85 c8 01 00 00        	movups	0x1c8(%rbp), %xmm0
140163844: 0f 29 85 30 14 00 00        	movaps	%xmm0, 0x1430(%rbp)
14016384b: 0f 10 85 d8 01 00 00        	movups	0x1d8(%rbp), %xmm0
140163852: 0f 29 85 40 14 00 00        	movaps	%xmm0, 0x1440(%rbp)
140163859: 0f 10 43 50                 	movups	0x50(%rbx), %xmm0
14016385d: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
140163864: 0f 10 43 40                 	movups	0x40(%rbx), %xmm0
140163868: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
14016386f: 0f 10 03                    	movups	(%rbx), %xmm0
140163872: 0f 10 4b 10                 	movups	0x10(%rbx), %xmm1
140163876: 0f 10 53 20                 	movups	0x20(%rbx), %xmm2
14016387a: 0f 10 5b 30                 	movups	0x30(%rbx), %xmm3
14016387e: 0f 29 9d 10 01 00 00        	movaps	%xmm3, 0x110(%rbp)
140163885: 0f 29 95 00 01 00 00        	movaps	%xmm2, 0x100(%rbp)
14016388c: 0f 29 8d f0 00 00 00        	movaps	%xmm1, 0xf0(%rbp)
140163893: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
14016389a: 0f 28 85 f0 13 00 00        	movaps	0x13f0(%rbp), %xmm0
1401638a1: 0f 28 8d 00 14 00 00        	movaps	0x1400(%rbp), %xmm1
1401638a8: 0f 28 95 10 14 00 00        	movaps	0x1410(%rbp), %xmm2
1401638af: 0f 28 9d 20 14 00 00        	movaps	0x1420(%rbp), %xmm3
1401638b6: 0f 11 03                    	movups	%xmm0, (%rbx)
1401638b9: 0f 11 4b 10                 	movups	%xmm1, 0x10(%rbx)
1401638bd: 0f 11 53 20                 	movups	%xmm2, 0x20(%rbx)
1401638c1: 0f 11 5b 30                 	movups	%xmm3, 0x30(%rbx)
1401638c5: 0f 28 85 30 14 00 00        	movaps	0x1430(%rbp), %xmm0
1401638cc: 0f 11 43 40                 	movups	%xmm0, 0x40(%rbx)
1401638d0: 0f 28 85 40 14 00 00        	movaps	0x1440(%rbp), %xmm0
1401638d7: 0f 11 43 50                 	movups	%xmm0, 0x50(%rbx)
1401638db: 48 83 bd e0 00 00 00 00     	cmpq	$0x0, 0xe0(%rbp)
1401638e3: 48 8b 9d 90 17 00 00        	movq	0x1790(%rbp), %rbx
1401638ea: 74 57                       	je	0x140163943 <.text+0x162943>
1401638ec: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
1401638f3: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1401638fa: 41 b8 68 00 00 00           	movl	$0x68, %r8d
140163900: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140163906: e8 a5 95 ee ff              	callq	0x14004ceb0 <.text+0x4beb0>
14016390b: 48 8b 95 18 01 00 00        	movq	0x118(%rbp), %rdx
140163912: 48 85 d2                    	testq	%rdx, %rdx
140163915: 74 2c                       	je	0x140163943 <.text+0x162943>
140163917: 48 89 d0                    	movq	%rdx, %rax
14016391a: 48 c1 e0 04                 	shlq	$0x4, %rax
14016391e: 48 01 c2                    	addq	%rax, %rdx
140163921: 48 83 c2 21                 	addq	$0x21, %rdx
140163925: 74 1c                       	je	0x140163943 <.text+0x162943>
140163927: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
14016392e: 48 f7 d8                    	negq	%rax
140163931: 48 01 c1                    	addq	%rax, %rcx
140163934: 48 83 c1 f0                 	addq	$-0x10, %rcx
140163938: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14016393e: e8 dd 02 4b 00              	callq	0x140613c20 <.text+0x612c20>
140163943: 48 8d 8d 38 13 00 00        	leaq	0x1338(%rbp), %rcx
14016394a: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140163951: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140163957: e8 a4 02 7c 02              	callq	0x142923c00 <.text+0x2922c00>
14016395c: 48 8b 85 f0 14 00 00        	movq	0x14f0(%rbp), %rax
140163963: 48 89 85 60 1a 00 00        	movq	%rax, 0x1a60(%rbp)
14016396a: 0f 10 85 e0 14 00 00        	movups	0x14e0(%rbp), %xmm0
140163971: 0f 29 85 50 1a 00 00        	movaps	%xmm0, 0x1a50(%rbp)
140163978: 48 c7 85 e0 00 00 00 00 00 00 00    	movq	$0x0, 0xe0(%rbp)
140163983: 48 c7 85 e8 00 00 00 08 00 00 00    	movq	$0x8, 0xe8(%rbp)
14016398e: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
140163999: 31 c9                       	xorl	%ecx, %ecx
14016399b: e8 80 61 f7 ff              	callq	0x1400d9b20 <.text+0xd8b20>
1401639a0: 48 85 c0                    	testq	%rax, %rax
1401639a3: 0f 84 e3 06 00 00           	je	0x14016408c <.text+0x16308c>
1401639a9: 0f 10 00                    	movups	(%rax), %xmm0
1401639ac: 48 ff 00                    	incq	(%rax)
1401639af: 0f 10 0d 7a c6 84 02        	movups	0x284c67a(%rip), %xmm1  # 0x1429b0030
1401639b6: 0f 11 8d c8 16 00 00        	movups	%xmm1, 0x16c8(%rbp)
1401639bd: 0f 10 0d 5c c6 84 02        	movups	0x284c65c(%rip), %xmm1  # 0x1429b0020
1401639c4: 0f 11 8d b8 16 00 00        	movups	%xmm1, 0x16b8(%rbp)
1401639cb: 0f 11 85 d8 16 00 00        	movups	%xmm0, 0x16d8(%rbp)
1401639d2: 48 c7 85 e0 15 00 00 00 00 00 00    	movq	$0x0, 0x15e0(%rbp)
1401639dd: 48 c7 85 e8 15 00 00 02 00 00 00    	movq	$0x2, 0x15e8(%rbp)
1401639e8: 0f 57 c0                    	xorps	%xmm0, %xmm0
1401639eb: 0f 11 85 f0 15 00 00        	movups	%xmm0, 0x15f0(%rbp)
1401639f2: 48 c7 85 00 16 00 00 01 00 00 00    	movq	$0x1, 0x1600(%rbp)
1401639fd: 48 c7 85 08 16 00 00 00 00 00 00    	movq	$0x0, 0x1608(%rbp)
140163a08: 0f 10 8d e0 00 00 00        	movups	0xe0(%rbp), %xmm1
140163a0f: 0f 11 8d 10 16 00 00        	movups	%xmm1, 0x1610(%rbp)
140163a16: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140163a1d: 48 89 85 20 16 00 00        	movq	%rax, 0x1620(%rbp)
140163a24: 48 8d 8d 28 16 00 00        	leaq	0x1628(%rbp), %rcx
140163a2b: 48 c7 85 28 16 00 00 00 00 00 00    	movq	$0x0, 0x1628(%rbp)
140163a36: 48 c7 85 30 16 00 00 04 00 00 00    	movq	$0x4, 0x1630(%rbp)
140163a41: 0f 11 85 38 16 00 00        	movups	%xmm0, 0x1638(%rbp)
140163a48: 48 c7 85 48 16 00 00 04 00 00 00    	movq	$0x4, 0x1648(%rbp)
140163a53: 0f 11 85 50 16 00 00        	movups	%xmm0, 0x1650(%rbp)
140163a5a: 48 c7 85 60 16 00 00 04 00 00 00    	movq	$0x4, 0x1660(%rbp)
140163a65: 0f 11 85 68 16 00 00        	movups	%xmm0, 0x1668(%rbp)
140163a6c: 48 c7 85 78 16 00 00 04 00 00 00    	movq	$0x4, 0x1678(%rbp)
140163a77: 0f 11 85 80 16 00 00        	movups	%xmm0, 0x1680(%rbp)
140163a7e: 48 c7 85 90 16 00 00 04 00 00 00    	movq	$0x4, 0x1690(%rbp)
140163a89: 0f 11 85 98 16 00 00        	movups	%xmm0, 0x1698(%rbp)
140163a90: 48 c7 85 a8 16 00 00 04 00 00 00    	movq	$0x4, 0x16a8(%rbp)
140163a9b: 48 c7 85 b0 16 00 00 00 00 00 00    	movq	$0x0, 0x16b0(%rbp)
140163aa6: 4c 8b 85 58 1a 00 00        	movq	0x1a58(%rbp), %r8
140163aad: 4c 8b 8d 60 1a 00 00        	movq	0x1a60(%rbp), %r9
140163ab4: c6 85 90 1a 00 00 01        	movb	$0x1, 0x1a90(%rbp)
140163abb: c6 85 8f 1a 00 00 01        	movb	$0x1, 0x1a8f(%rbp)
140163ac2: 48 8d 95 38 13 00 00        	leaq	0x1338(%rbp), %rdx
140163ac9: e8 62 c9 3c 00              	callq	0x140530430 <.text+0x52f430>
140163ace: c6 85 90 1a 00 00 01        	movb	$0x1, 0x1a90(%rbp)
140163ad5: c6 85 8f 1a 00 00 01        	movb	$0x1, 0x1a8f(%rbp)
140163adc: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163ae3: 48 8d 95 38 13 00 00        	leaq	0x1338(%rbp), %rdx
140163aea: 4c 8b 85 98 17 00 00        	movq	0x1798(%rbp), %r8
140163af1: 4d 89 e9                    	movq	%r13, %r9
140163af4: e8 a7 40 a8 00              	callq	0x140be7ba0 <.text+0xbe6ba0>
140163af9: 84 c0                       	testb	%al, %al
140163afb: 0f 84 a8 00 00 00           	je	0x140163ba9 <.text+0x162ba9>
140163b01: c6 85 90 1a 00 00 01        	movb	$0x1, 0x1a90(%rbp)
140163b08: c6 85 8f 1a 00 00 01        	movb	$0x1, 0x1a8f(%rbp)
140163b0f: 48 8d 8d 38 13 00 00        	leaq	0x1338(%rbp), %rcx
140163b16: e8 05 59 a8 00              	callq	0x140be9420 <.text+0xbe8420>
140163b1b: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140163b22: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140163b28: 48 89 fa                    	movq	%rdi, %rdx
140163b2b: e8 d0 00 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140163b30: 48 8d 95 38 13 00 00        	leaq	0x1338(%rbp), %rdx
140163b37: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140163b3d: 48 89 f9                    	movq	%rdi, %rcx
140163b40: e8 bb 00 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140163b45: 31 c0                       	xorl	%eax, %eax
140163b47: 48 3b 85 e0 00 00 00        	cmpq	0xe0(%rbp), %rax
140163b4e: 70 1a                       	jo	0x140163b6a <.text+0x162b6a>
140163b50: c6 85 90 1a 00 00 00        	movb	$0x0, 0x1a90(%rbp)
140163b57: c6 85 8f 1a 00 00 01        	movb	$0x1, 0x1a8f(%rbp)
140163b5e: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140163b65: e8 a6 b6 48 00              	callq	0x1405ef210 <.text+0x5ee210>
140163b6a: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140163b74: 48 89 03                    	movq	%rax, (%rbx)
140163b77: c6 85 92 1a 00 00 00        	movb	$0x0, 0x1a92(%rbp)
140163b7e: c6 85 91 1a 00 00 01        	movb	$0x1, 0x1a91(%rbp)
140163b85: 48 8d 8d e0 15 00 00        	leaq	0x15e0(%rbp), %rcx
140163b8c: e8 3f 96 49 00              	callq	0x1405fd1d0 <.text+0x5fc1d0>
140163b91: c6 85 8b 1a 00 00 00        	movb	$0x0, 0x1a8b(%rbp)
140163b98: 48 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %rcx
140163b9f: e8 7c 6c 47 00              	callq	0x1405da820 <.text+0x5d9820>
140163ba4: e9 97 00 00 00              	jmp	0x140163c40 <.text+0x162c40>
140163ba9: 48 8d b5 28 12 00 00        	leaq	0x1228(%rbp), %rsi
140163bb0: 48 8d 95 38 13 00 00        	leaq	0x1338(%rbp), %rdx
140163bb7: 41 b8 90 00 00 00           	movl	$0x90, %r8d
140163bbd: 48 89 f1                    	movq	%rsi, %rcx
140163bc0: e8 3b 00 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140163bc5: 48 8b 85 60 1a 00 00        	movq	0x1a60(%rbp), %rax
140163bcc: 48 89 85 20 1a 00 00        	movq	%rax, 0x1a20(%rbp)
140163bd3: 0f 28 85 50 1a 00 00        	movaps	0x1a50(%rbp), %xmm0
140163bda: 0f 29 85 10 1a 00 00        	movaps	%xmm0, 0x1a10(%rbp)
140163be1: 48 8d bd 10 18 00 00        	leaq	0x1810(%rbp), %rdi
140163be8: 48 8d 95 e0 15 00 00        	leaq	0x15e0(%rbp), %rdx
140163bef: 41 b8 08 01 00 00           	movl	$0x108, %r8d            # imm = 0x108
140163bf5: 48 89 f9                    	movq	%rdi, %rcx
140163bf8: e8 03 00 7c 02              	callq	0x142923c00 <.text+0x2922c00>
140163bfd: c6 85 90 1a 00 00 00        	movb	$0x0, 0x1a90(%rbp)
140163c04: c6 85 8f 1a 00 00 00        	movb	$0x0, 0x1a8f(%rbp)
140163c0b: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140163c12: 4c 8d 85 10 1a 00 00        	leaq	0x1a10(%rbp), %r8
140163c19: 48 89 f2                    	movq	%rsi, %rdx
140163c1c: 49 89 f9                    	movq	%rdi, %r9
140163c1f: e8 5c 06 16 00              	callq	0x1402c4280 <.text+0x2c3280>
140163c24: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140163c2b: 41 b8 18 02 00 00           	movl	$0x218, %r8d            # imm = 0x218
140163c31: 48 89 d9                    	movq	%rbx, %rcx
140163c34: e8 c7 ff 7b 02              	callq	0x142923c00 <.text+0x2922c00>
140163c39: 44 89 b3 18 02 00 00        	movl	%r14d, 0x218(%rbx)
140163c40: 48 89 d8                    	movq	%rbx, %rax
140163c43: 0f 28 b5 a0 1a 00 00        	movaps	0x1aa0(%rbp), %xmm6
140163c4a: 48 81 c4 38 1b 00 00        	addq	$0x1b38, %rsp           # imm = 0x1B38
140163c51: 5b                          	popq	%rbx
140163c52: 5f                          	popq	%rdi
140163c53: 5e                          	popq	%rsi
140163c54: 41 5c                       	popq	%r12
140163c56: 41 5d                       	popq	%r13
140163c58: 41 5e                       	popq	%r14
140163c5a: 41 5f                       	popq	%r15
140163c5c: 5d                          	popq	%rbp
140163c5d: c3                          	retq
140163c5e: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163c65: 48 89 f1                    	movq	%rsi, %rcx
140163c68: e8 f3 bf 7d 00              	callq	0x14093fc60 <.text+0x93ec60>
140163c6d: 48 8d 0d e8 9a 7f 02        	leaq	0x27f9ae8(%rip), %rcx   # 0x14295d75c
140163c74: 48 89 8d 10 1a 00 00        	movq	%rcx, 0x1a10(%rbp)
140163c7b: 48 c7 85 18 1a 00 00 1f 00 00 00    	movq	$0x1f, 0x1a18(%rbp)
140163c86: 84 c0                       	testb	%al, %al
140163c88: 0f 85 30 eb ff ff           	jne	0x1401627be <.text+0x1617be>
140163c8e: 48 8b 05 13 87 4c 03        	movq	0x34c8713(%rip), %rax   # 0x14362c3a8
140163c95: 48 8b 00                    	movq	(%rax), %rax
140163c98: 48 83 f8 05                 	cmpq	$0x5, %rax
140163c9c: 0f 82 9e eb ff ff           	jb	0x140162840 <.text+0x161840>
140163ca2: 4c 89 ad 50 14 00 00        	movq	%r13, 0x1450(%rbp)
140163ca9: 48 8d 05 e0 61 fa ff        	leaq	-0x59e20(%rip), %rax    # 0x140109e90 <.text+0x108e90>
140163cb0: 48 89 85 58 14 00 00        	movq	%rax, 0x1458(%rbp)
140163cb7: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163cbe: 48 8d 0d 93 7e 88 02        	leaq	0x2887e93(%rip), %rcx   # 0x1429ebb58
140163cc5: e8 b6 a0 51 00              	callq	0x14067dd80 <.text+0x67cd80>
140163cca: 0f 10 00                    	movups	(%rax), %xmm0
140163ccd: 8b 40 10                    	movl	0x10(%rax), %eax
140163cd0: 48 c7 85 40 18 00 00 05 00 00 00    	movq	$0x5, 0x1840(%rbp)
140163cdb: 48 8d 0d 8e 7e 88 02        	leaq	0x2887e8e(%rip), %rcx   # 0x1429ebb70
140163ce2: 48 89 8d 48 18 00 00        	movq	%rcx, 0x1848(%rbp)
140163ce9: 48 c7 85 50 18 00 00 17 00 00 00    	movq	$0x17, 0x1850(%rbp)
140163cf4: 48 8d 15 25 7e 88 02        	leaq	0x2887e25(%rip), %rdx   # 0x1429ebb20
140163cfb: 48 89 95 60 18 00 00        	movq	%rdx, 0x1860(%rbp)
140163d02: 48 c7 85 68 18 00 00 01 00 00 00    	movq	$0x1, 0x1868(%rbp)
140163d0d: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140163d14: 48 89 95 70 18 00 00        	movq	%rdx, 0x1870(%rbp)
140163d1b: 48 c7 85 78 18 00 00 01 00 00 00    	movq	$0x1, 0x1878(%rbp)
140163d26: 48 c7 85 80 18 00 00 00 00 00 00    	movq	$0x0, 0x1880(%rbp)
140163d31: 48 c7 85 10 18 00 00 00 00 00 00    	movq	$0x0, 0x1810(%rbp)
140163d3c: 48 89 8d 18 18 00 00        	movq	%rcx, 0x1818(%rbp)
140163d43: 48 c7 85 20 18 00 00 17 00 00 00    	movq	$0x17, 0x1820(%rbp)
140163d4e: 48 c7 85 28 18 00 00 00 00 00 00    	movq	$0x0, 0x1828(%rbp)
140163d59: 0f 11 85 30 18 00 00        	movups	%xmm0, 0x1830(%rbp)
140163d60: c7 85 58 18 00 00 01 00 00 00       	movl	$0x1, 0x1858(%rbp)
140163d6a: 89 85 5c 18 00 00           	movl	%eax, 0x185c(%rbp)
140163d70: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163d77: 48 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %rcx
140163d7e: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
140163d85: e8 a6 65 be 00              	callq	0x140d4a330 <.text+0xd49330>
140163d8a: e9 b1 ea ff ff              	jmp	0x140162840 <.text+0x161840>
140163d8f: 48 8b 05 12 86 4c 03        	movq	0x34c8612(%rip), %rax   # 0x14362c3a8
140163d96: 48 8b 00                    	movq	(%rax), %rax
140163d99: 48 83 f8 05                 	cmpq	$0x5, %rax
140163d9d: 0f 82 9d ea ff ff           	jb	0x140162840 <.text+0x161840>
140163da3: 4c 89 ad 50 14 00 00        	movq	%r13, 0x1450(%rbp)
140163daa: 48 8d 05 df 60 fa ff        	leaq	-0x59f21(%rip), %rax    # 0x140109e90 <.text+0x108e90>
140163db1: 48 89 85 58 14 00 00        	movq	%rax, 0x1458(%rbp)
140163db8: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163dbf: 48 8d 0d 92 7d 88 02        	leaq	0x2887d92(%rip), %rcx   # 0x1429ebb58
140163dc6: e8 b5 9f 51 00              	callq	0x14067dd80 <.text+0x67cd80>
140163dcb: 0f 10 00                    	movups	(%rax), %xmm0
140163dce: 8b 40 10                    	movl	0x10(%rax), %eax
140163dd1: 48 c7 85 40 18 00 00 05 00 00 00    	movq	$0x5, 0x1840(%rbp)
140163ddc: 48 8d 0d 8d 7d 88 02        	leaq	0x2887d8d(%rip), %rcx   # 0x1429ebb70
140163de3: 48 89 8d 48 18 00 00        	movq	%rcx, 0x1848(%rbp)
140163dea: 48 c7 85 50 18 00 00 17 00 00 00    	movq	$0x17, 0x1850(%rbp)
140163df5: 48 8d 15 24 7d 88 02        	leaq	0x2887d24(%rip), %rdx   # 0x1429ebb20
140163dfc: 48 89 95 60 18 00 00        	movq	%rdx, 0x1860(%rbp)
140163e03: 48 c7 85 68 18 00 00 01 00 00 00    	movq	$0x1, 0x1868(%rbp)
140163e0e: 48 8d 95 50 14 00 00        	leaq	0x1450(%rbp), %rdx
140163e15: 48 89 95 70 18 00 00        	movq	%rdx, 0x1870(%rbp)
140163e1c: 48 c7 85 78 18 00 00 01 00 00 00    	movq	$0x1, 0x1878(%rbp)
140163e27: 48 c7 85 80 18 00 00 00 00 00 00    	movq	$0x0, 0x1880(%rbp)
140163e32: 48 c7 85 10 18 00 00 00 00 00 00    	movq	$0x0, 0x1810(%rbp)
140163e3d: 48 89 8d 18 18 00 00        	movq	%rcx, 0x1818(%rbp)
140163e44: 48 c7 85 20 18 00 00 17 00 00 00    	movq	$0x17, 0x1820(%rbp)
140163e4f: 48 c7 85 28 18 00 00 00 00 00 00    	movq	$0x0, 0x1828(%rbp)
140163e5a: 0f 11 85 30 18 00 00        	movups	%xmm0, 0x1830(%rbp)
140163e61: c7 85 58 18 00 00 01 00 00 00       	movl	$0x1, 0x1858(%rbp)
140163e6b: 89 85 5c 18 00 00           	movl	%eax, 0x185c(%rbp)
140163e71: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163e78: 48 8d 8d 50 1a 00 00        	leaq	0x1a50(%rbp), %rcx
140163e7f: 48 8d 95 10 18 00 00        	leaq	0x1810(%rbp), %rdx
140163e86: e8 a5 64 be 00              	callq	0x140d4a330 <.text+0xd49330>
140163e8b: e9 b0 e9 ff ff              	jmp	0x140162840 <.text+0x161840>
140163e90: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
140163e97: 48 89 f9                    	movq	%rdi, %rcx
140163e9a: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140163ea0: e8 6b 93 76 02              	callq	0x1428cd210 <.text+0x28cc210>
140163ea5: e9 2a e2 ff ff              	jmp	0x1401620d4 <.text+0x1610d4>
140163eaa: 4c 8d 05 ff a9 87 02        	leaq	0x287a9ff(%rip), %r8    # 0x1429de8b0
140163eb1: ba 90 10 00 00              	movl	$0x1090, %edx           # imm = 0x1090
140163eb6: e8 25 5a 77 02              	callq	0x1428d98e0 <.text+0x28d88e0>
140163ebb: e9 02 02 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163ec0: c6 85 8e 1a 00 00 01        	movb	$0x1, 0x1a8e(%rbp)
140163ec7: 48 89 f9                    	movq	%rdi, %rcx
140163eca: 31 d2                       	xorl	%edx, %edx
140163ecc: e8 bf 9c 76 02              	callq	0x1428cdb90 <.text+0x28ccb90>
140163ed1: e9 e6 e2 ff ff              	jmp	0x1401621bc <.text+0x1611bc>
140163ed6: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163edd: 48 8d 0d 6c 71 4c 03        	leaq	0x34c716c(%rip), %rcx   # 0x14362b050
140163ee4: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140163eea: e8 21 93 76 02              	callq	0x1428cd210 <.text+0x28cc210>
140163eef: e9 42 ed ff ff              	jmp	0x140162c36 <.text+0x161c36>
140163ef4: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163efb: 48 8d 0d 4e 71 4c 03        	leaq	0x34c714e(%rip), %rcx   # 0x14362b050
140163f02: 31 d2                       	xorl	%edx, %edx
140163f04: e8 87 9c 76 02              	callq	0x1428cdb90 <.text+0x28ccb90>
140163f09: e9 d2 ed ff ff              	jmp	0x140162ce0 <.text+0x161ce0>
140163f0e: c6 85 93 1a 00 00 01        	movb	$0x1, 0x1a93(%rbp)
140163f15: 48 8d 0d 14 97 7f 02        	leaq	0x27f9714(%rip), %rcx   # 0x14295d630
140163f1c: e8 ef 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163f21: e9 9c 01 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163f26: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
140163f2d: 48 8d 0d 14 97 7f 02        	leaq	0x27f9714(%rip), %rcx   # 0x14295d648
140163f34: e8 d7 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163f39: e9 84 01 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163f3e: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
140163f45: 48 8d 0d 14 97 7f 02        	leaq	0x27f9714(%rip), %rcx   # 0x14295d660
140163f4c: e8 bf 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163f51: e9 6c 01 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163f56: c6 85 94 1a 00 00 01        	movb	$0x1, 0x1a94(%rbp)
140163f5d: 48 8d 0d 14 97 7f 02        	leaq	0x27f9714(%rip), %rcx   # 0x14295d678
140163f64: e8 a7 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163f69: e9 54 01 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163f6e: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163f75: 48 8d 0d 14 97 7f 02        	leaq	0x27f9714(%rip), %rcx   # 0x14295d690
140163f7c: e8 8f 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163f81: e9 3c 01 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163f86: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163f8d: 48 8d 0d 24 97 7f 02        	leaq	0x27f9724(%rip), %rcx   # 0x14295d6b8
140163f94: e8 77 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163f99: e9 24 01 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163f9e: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163fa5: 48 8d 0d 24 97 7f 02        	leaq	0x27f9724(%rip), %rcx   # 0x14295d6d0
140163fac: e8 5f 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163fb1: e9 0c 01 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163fb6: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
140163fbd: 48 8d 0d de 97 7f 02        	leaq	0x27f97de(%rip), %rcx   # 0x14295d7a2
140163fc4: 4c 8d 05 ed 97 7f 02        	leaq	0x27f97ed(%rip), %r8    # 0x14295d7b8
140163fcb: ba 15 00 00 00              	movl	$0x15, %edx
140163fd0: e8 5b 53 77 02              	callq	0x1428d9330 <.text+0x28d8330>
140163fd5: e9 e8 00 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163fda: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140163fe1: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140163fe8: 48 8d 0d 41 98 7f 02        	leaq	0x27f9841(%rip), %rcx   # 0x14295d830
140163fef: e8 1c 53 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140163ff4: e9 c9 00 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140163ff9: c6 85 96 1a 00 00 00        	movb	$0x0, 0x1a96(%rbp)
140164000: c6 85 95 1a 00 00 01        	movb	$0x1, 0x1a95(%rbp)
140164007: 48 8d 0d 3a 98 7f 02        	leaq	0x27f983a(%rip), %rcx   # 0x14295d848
14016400e: e8 fd 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140164013: e9 aa 00 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140164018: 48 8d 0d 41 98 7f 02        	leaq	0x27f9841(%rip), %rcx   # 0x14295d860
14016401f: e8 ec 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140164024: e9 99 00 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
140164029: 48 8d 0d 48 98 7f 02        	leaq	0x27f9848(%rip), %rcx   # 0x14295d878
140164030: e8 db 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140164035: e9 88 00 00 00              	jmp	0x1401640c2 <.text+0x1630c2>
14016403a: 48 8d 0d 4f 98 7f 02        	leaq	0x27f984f(%rip), %rcx   # 0x14295d890
140164041: e8 ca 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140164046: eb 7a                       	jmp	0x1401640c2 <.text+0x1630c2>
140164048: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016404f: 48 8d 0d 2a 97 7f 02        	leaq	0x27f972a(%rip), %rcx   # 0x14295d780
140164056: e8 b5 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
14016405b: eb 65                       	jmp	0x1401640c2 <.text+0x1630c2>
14016405d: 48 8d 0d 74 95 7f 02        	leaq	0x27f9574(%rip), %rcx   # 0x14295d5d8
140164064: e8 a7 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140164069: 48 8d 0d 80 95 7f 02        	leaq	0x27f9580(%rip), %rcx   # 0x14295d5f0
140164070: e8 9b 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
140164075: eb 4b                       	jmp	0x1401640c2 <.text+0x1630c2>
140164077: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
14016407e: 48 8d 0d 63 96 7f 02        	leaq	0x27f9663(%rip), %rcx   # 0x14295d6e8
140164085: e8 86 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
14016408a: eb 36                       	jmp	0x1401640c2 <.text+0x1630c2>
14016408c: 48 8d 0d 7d e9 7c 02        	leaq	0x27ce97d(%rip), %rcx   # 0x142932a10
140164093: e8 38 40 77 02              	callq	0x1428d80d0 <.text+0x28d70d0>
140164098: eb 28                       	jmp	0x1401640c2 <.text+0x1630c2>
14016409a: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401640a1: 48 8d 0d 88 96 7f 02        	leaq	0x27f9688(%rip), %rcx   # 0x14295d730
1401640a8: e8 63 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
1401640ad: eb 13                       	jmp	0x1401640c2 <.text+0x1630c2>
1401640af: c6 85 97 1a 00 00 01        	movb	$0x1, 0x1a97(%rbp)
1401640b6: 48 8d 0d 43 96 7f 02        	leaq	0x27f9643(%rip), %rcx   # 0x14295d700
1401640bd: e8 4e 52 77 02              	callq	0x1428d9310 <.text+0x28d8310>
1401640c2: 0f 0b                       	ud2
