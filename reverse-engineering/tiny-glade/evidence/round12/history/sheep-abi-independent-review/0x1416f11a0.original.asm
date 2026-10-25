
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416f11a0: 55                          	pushq	%rbp
1416f11a1: 41 57                       	pushq	%r15
1416f11a3: 41 56                       	pushq	%r14
1416f11a5: 41 55                       	pushq	%r13
1416f11a7: 41 54                       	pushq	%r12
1416f11a9: 56                          	pushq	%rsi
1416f11aa: 57                          	pushq	%rdi
1416f11ab: 53                          	pushq	%rbx
1416f11ac: 48 81 ec 38 04 00 00        	subq	$0x438, %rsp            # imm = 0x438
1416f11b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416f11bb: 44 0f 29 8d a0 03 00 00     	movaps	%xmm9, 0x3a0(%rbp)
1416f11c3: 44 0f 29 85 90 03 00 00     	movaps	%xmm8, 0x390(%rbp)
1416f11cb: 0f 29 bd 80 03 00 00        	movaps	%xmm7, 0x380(%rbp)
1416f11d2: 0f 29 b5 70 03 00 00        	movaps	%xmm6, 0x370(%rbp)
1416f11d9: 48 c7 85 68 03 00 00 fe ff ff ff    	movq	$-0x2, 0x368(%rbp)
1416f11e4: 4c 89 8d 50 03 00 00        	movq	%r9, 0x350(%rbp)
1416f11eb: 48 89 95 38 03 00 00        	movq	%rdx, 0x338(%rbp)
1416f11f2: 4d 8b 38                    	movq	(%r8), %r15
1416f11f5: 49 8b 40 08                 	movq	0x8(%r8), %rax
1416f11f9: f2 41 0f 10 40 10           	movsd	0x10(%r8), %xmm0
1416f11ff: 49 8b 97 f8 00 00 00        	movq	0xf8(%r15), %rdx
1416f1206: 4d 8b 87 00 01 00 00        	movq	0x100(%r15), %r8
1416f120d: 45 0f b6 8f 08 01 00 00     	movzbl	0x108(%r15), %r9d
1416f1215: 45 84 c9                    	testb	%r9b, %r9b
1416f1218: 0f 84 88 00 00 00           	je	0x1416f12a6 <.text+0x16f02a6>
1416f121e: 4c 8d 55 b8                 	leaq	-0x48(%rbp), %r10
1416f1222: 4c 8d 5d c0                 	leaq	-0x40(%rbp), %r11
1416f1226: 48 8d 75 c8                 	leaq	-0x38(%rbp), %rsi
1416f122a: 4e 8d 24 85 00 00 00 00     	leaq	(,%r8,4), %r12
1416f1232: 31 db                       	xorl	%ebx, %ebx
1416f1234: 48 89 d1                    	movq	%rdx, %rcx
1416f1237: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416f1240: 4d 85 e4                    	testq	%r12, %r12
1416f1243: 0f 84 70 01 00 00           	je	0x1416f13b9 <.text+0x16f03b9>
1416f1249: 8b 39                       	movl	(%rcx), %edi
1416f124b: 48 83 c1 04                 	addq	$0x4, %rcx
1416f124f: 4c 8b b0 a8 01 00 00        	movq	0x1a8(%rax), %r14
1416f1256: 4c 8d 2c ff                 	leaq	(%rdi,%rdi,8), %r13
1416f125a: 4b 8b 7c ee 10              	movq	0x10(%r14,%r13,8), %rdi
1416f125f: 49 83 c4 fc                 	addq	$-0x4, %r12
1416f1263: 48 85 ff                    	testq	%rdi, %rdi
1416f1266: 74 d8                       	je	0x1416f1240 <.text+0x16f0240>
1416f1268: 4b 8d 1c ee                 	leaq	(%r14,%r13,8), %rbx
1416f126c: 4d 8b b7 10 01 00 00        	movq	0x110(%r15), %r14
1416f1273: 4c 8b 7b 18                 	movq	0x18(%rbx), %r15
1416f1277: 48 8b 5b 38                 	movq	0x38(%rbx), %rbx
1416f127b: 4a 8b 1c f3                 	movq	(%rbx,%r14,8), %rbx
1416f127f: 48 f7 d3                    	notq	%rbx
1416f1282: 4c 8d 34 5b                 	leaq	(%rbx,%rbx,2), %r14
1416f1286: 49 c1 e6 04                 	shlq	$0x4, %r14
1416f128a: 4b 8b 5c 37 28              	movq	0x28(%r15,%r14), %rbx
1416f128f: 4f 8b 6c 37 10              	movq	0x10(%r15,%r14), %r13
1416f1294: 4f 8b 74 37 20              	movq	0x20(%r15,%r14), %r14
1416f1299: 4c 89 6d b0                 	movq	%r13, -0x50(%rbp)
1416f129d: 4c 89 75 b8                 	movq	%r14, -0x48(%rbp)
1416f12a1: e9 bf 00 00 00              	jmp	0x1416f1365 <.text+0x16f0365>
1416f12a6: 4c 8d 95 60 01 00 00        	leaq	0x160(%rbp), %r10
1416f12ad: 4c 8d 9d 68 01 00 00        	leaq	0x168(%rbp), %r11
1416f12b4: 48 8d b5 70 01 00 00        	leaq	0x170(%rbp), %rsi
1416f12bb: 4e 8d 24 85 00 00 00 00     	leaq	(,%r8,4), %r12
1416f12c3: 31 db                       	xorl	%ebx, %ebx
1416f12c5: 48 89 d1                    	movq	%rdx, %rcx
1416f12c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1416f12d0: 4d 85 e4                    	testq	%r12, %r12
1416f12d3: 0f 84 e0 00 00 00           	je	0x1416f13b9 <.text+0x16f03b9>
1416f12d9: 8b 39                       	movl	(%rcx), %edi
1416f12db: 48 83 c1 04                 	addq	$0x4, %rcx
1416f12df: 4c 8b a8 00 01 00 00        	movq	0x100(%rax), %r13
1416f12e6: 4c 8d 34 bf                 	leaq	(%rdi,%rdi,4), %r14
1416f12ea: 49 c1 e6 05                 	shlq	$0x5, %r14
1416f12ee: 4b 8b 7c 35 58              	movq	0x58(%r13,%r14), %rdi
1416f12f3: 49 83 c4 fc                 	addq	$-0x4, %r12
1416f12f7: 48 85 ff                    	testq	%rdi, %rdi
1416f12fa: 74 d4                       	je	0x1416f12d0 <.text+0x16f02d0>
1416f12fc: 4d 01 f5                    	addq	%r14, %r13
1416f12ff: 41 8b 9d 94 00 00 00        	movl	0x94(%r13), %ebx
1416f1306: 4c 8b b0 a8 01 00 00        	movq	0x1a8(%rax), %r14
1416f130d: 48 8d 1c db                 	leaq	(%rbx,%rbx,8), %rbx
1416f1311: 4d 8b bf 10 01 00 00        	movq	0x110(%r15), %r15
1416f1318: 4d 8b 64 de 18              	movq	0x18(%r14,%rbx,8), %r12
1416f131d: 49 8b 5c de 38              	movq	0x38(%r14,%rbx,8), %rbx
1416f1322: 4a 8b 1c fb                 	movq	(%rbx,%r15,8), %rbx
1416f1326: 48 f7 d3                    	notq	%rbx
1416f1329: 4d 8b 75 50                 	movq	0x50(%r13), %r14
1416f132d: 4c 8d 3c 5b                 	leaq	(%rbx,%rbx,2), %r15
1416f1331: 49 c1 e7 04                 	shlq	$0x4, %r15
1416f1335: 41 8b 5e 08                 	movl	0x8(%r14), %ebx
1416f1339: 4c 8d 2c 9b                 	leaq	(%rbx,%rbx,4), %r13
1416f133d: 49 c1 e5 04                 	shlq	$0x4, %r13
1416f1341: 4f 03 6c 3c 10              	addq	0x10(%r12,%r15), %r13
1416f1346: 48 c1 e3 02                 	shlq	$0x2, %rbx
1416f134a: 4f 8b 74 3c 20              	movq	0x20(%r12,%r15), %r14
1416f134f: 49 01 de                    	addq	%rbx, %r14
1416f1352: 4b 03 5c 3c 28              	addq	0x28(%r12,%r15), %rbx
1416f1357: 4c 89 ad 58 01 00 00        	movq	%r13, 0x158(%rbp)
1416f135e: 4c 89 b5 60 01 00 00        	movq	%r14, 0x160(%rbp)
1416f1365: 49 89 1b                    	movq	%rbx, (%r11)
1416f1368: 0f 13 06                    	movlps	%xmm0, (%rsi)
1416f136b: 4d 8b 5a 08                 	movq	0x8(%r10), %r11
1416f136f: 4d 8b 52 10                 	movq	0x10(%r10), %r10
1416f1373: bb 01 00 00 00              	movl	$0x1, %ebx
1416f1378: 4a 8d 14 82                 	leaq	(%rdx,%r8,4), %rdx
1416f137c: 45 84 c9                    	testb	%r9b, %r9b
1416f137f: 75 46                       	jne	0x1416f13c7 <.text+0x16f03c7>
1416f1381: 48 39 fb                    	cmpq	%rdi, %rbx
1416f1384: 75 69                       	jne	0x1416f13ef <.text+0x16f03ef>
1416f1386: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1416f1390: 48 39 d1                    	cmpq	%rdx, %rcx
1416f1393: 0f 84 ac 00 00 00           	je	0x1416f1445 <.text+0x16f0445>
1416f1399: 44 8b 01                    	movl	(%rcx), %r8d
1416f139c: 48 83 c1 04                 	addq	$0x4, %rcx
1416f13a0: 4c 8b 88 00 01 00 00        	movq	0x100(%rax), %r9
1416f13a7: 4f 8d 04 80                 	leaq	(%r8,%r8,4), %r8
1416f13ab: 49 c1 e0 05                 	shlq	$0x5, %r8
1416f13af: 4b 83 7c 01 58 00           	cmpq	$0x0, 0x58(%r9,%r8)
1416f13b5: 74 d9                       	je	0x1416f1390 <.text+0x16f0390>
1416f13b7: eb 36                       	jmp	0x1416f13ef <.text+0x16f03ef>
1416f13b9: 31 ff                       	xorl	%edi, %edi
1416f13bb: 45 31 ed                    	xorl	%r13d, %r13d
1416f13be: 4a 8d 14 82                 	leaq	(%rdx,%r8,4), %rdx
1416f13c2: 45 84 c9                    	testb	%r9b, %r9b
1416f13c5: 74 ba                       	je	0x1416f1381 <.text+0x16f0381>
1416f13c7: 48 39 fb                    	cmpq	%rdi, %rbx
1416f13ca: 75 23                       	jne	0x1416f13ef <.text+0x16f03ef>
1416f13cc: 0f 1f 40 00                 	nopl	(%rax)
1416f13d0: 48 39 d1                    	cmpq	%rdx, %rcx
1416f13d3: 74 70                       	je	0x1416f1445 <.text+0x16f0445>
1416f13d5: 44 8b 01                    	movl	(%rcx), %r8d
1416f13d8: 48 83 c1 04                 	addq	$0x4, %rcx
1416f13dc: 4c 8b 88 a8 01 00 00        	movq	0x1a8(%rax), %r9
1416f13e3: 4f 8d 04 c0                 	leaq	(%r8,%r8,8), %r8
1416f13e7: 4b 83 7c c1 10 00           	cmpq	$0x0, 0x10(%r9,%r8,8)
1416f13ed: 74 e1                       	je	0x1416f13d0 <.text+0x16f03d0>
1416f13ef: 4d 85 ed                    	testq	%r13, %r13
1416f13f2: 0f 94 c0                    	sete	%al
1416f13f5: 34 01                       	xorb	$0x1, %al
1416f13f7: 0f b6 c0                    	movzbl	%al, %eax
1416f13fa: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1416f1401: 48 8d 05 60 ee 59 01        	leaq	0x159ee60(%rip), %rax   # 0x142c90268
1416f1408: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1416f140f: 48 c7 85 68 01 00 00 5f 00 00 00    	movq	$0x5f, 0x168(%rbp)
1416f141a: 48 8d 05 af ef 59 01        	leaq	0x159efaf(%rip), %rax   # 0x142c903d0
1416f1421: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f1426: 48 8d 0d 83 eb 59 01        	leaq	0x159eb83(%rip), %rcx   # 0x142c8ffb0
1416f142d: 4c 8d 0d 5c eb 59 01        	leaq	0x159eb5c(%rip), %r9    # 0x142c8ff90
1416f1434: 4c 8d 85 58 01 00 00        	leaq	0x158(%rbp), %r8
1416f143b: ba 2b 00 00 00              	movl	$0x2b, %edx
1416f1440: e8 1b 83 1e 01              	callq	0x1428d9760 <.text+0x28d8760>
1416f1445: 4d 85 ed                    	testq	%r13, %r13
1416f1448: 0f 94 c0                    	sete	%al
1416f144b: 74 a8                       	je	0x1416f13f5 <.text+0x16f03f5>
1416f144d: 49 c1 ea 20                 	shrq	$0x20, %r10
1416f1451: 45 89 13                    	movl	%r10d, (%r11)
1416f1454: 31 c0                       	xorl	%eax, %eax
1416f1456: 49 3b 45 00                 	cmpq	(%r13), %rax
1416f145a: 71 0c                       	jno	0x1416f1468 <.text+0x16f0468>
1416f145c: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416f1466: eb 13                       	jmp	0x1416f147b <.text+0x16f047b>
1416f1468: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1416f146f: 4c 89 ea                    	movq	%r13, %rdx
1416f1472: e8 79 10 00 00              	callq	0x1416f24f0 <.text+0x16f14f0>
1416f1477: 49 8b 45 00                 	movq	(%r13), %rax
1416f147b: 48 f7 d8                    	negq	%rax
1416f147e: 0f 91 85 67 03 00 00        	setno	0x367(%rbp)
1416f1485: 48 8b bd 30 04 00 00        	movq	0x430(%rbp), %rdi
1416f148c: 0f 91 c3                    	setno	%bl
1416f148f: 80 3d 42 38 f4 01 00        	cmpb	$0x0, 0x1f43842(%rip)   # 0x143634cd8
1416f1496: 0f 84 22 01 00 00           	je	0x1416f15be <.text+0x16f05be>
1416f149c: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1416f14a3: 48 8b 08                    	movq	(%rax), %rcx
1416f14a6: c7 85 58 01 00 00 28 00 00 00       	movl	$0x28, 0x158(%rbp)
1416f14b0: 48 83 c1 20                 	addq	$0x20, %rcx
1416f14b4: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1416f14bb: e8 30 9a 1a ff              	callq	0x14089aef0 <.text+0x899ef0>
1416f14c0: 48 85 c0                    	testq	%rax, %rax
1416f14c3: 0f 84 f5 00 00 00           	je	0x1416f15be <.text+0x16f05be>
1416f14c9: 80 f3 01                    	xorb	$0x1, %bl
1416f14cc: 88 9d 67 03 00 00           	movb	%bl, 0x367(%rbp)
1416f14d2: 48 8b 05 cf ae f3 01        	movq	0x1f3aecf(%rip), %rax   # 0x14362c3a8
1416f14d9: 48 8b 00                    	movq	(%rax), %rax
1416f14dc: 48 83 f8 03                 	cmpq	$0x3, %rax
1416f14e0: 0f 82 d8 00 00 00           	jb	0x1416f15be <.text+0x16f05be>
1416f14e6: 48 8d 85 67 03 00 00        	leaq	0x367(%rbp), %rax
1416f14ed: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1416f14f1: 48 8d 05 28 6e 69 ff        	leaq	-0x9691d8(%rip), %rax   # 0x140d88320 <.text+0xd87320>
1416f14f8: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1416f14fc: 48 8d 0d 05 ef 59 01        	leaq	0x159ef05(%rip), %rcx   # 0x142c90408
1416f1503: e8 78 c8 f8 fe              	callq	0x14067dd80 <.text+0x67cd80>
1416f1508: 0f 10 00                    	movups	(%rax), %xmm0
1416f150b: 8b 40 10                    	movl	0x10(%rax), %eax
1416f150e: 48 c7 85 88 01 00 00 03 00 00 00    	movq	$0x3, 0x188(%rbp)
1416f1519: 48 8d 0d 00 ef 59 01        	leaq	0x159ef00(%rip), %rcx   # 0x142c90420
1416f1520: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
1416f1527: 48 c7 85 98 01 00 00 2c 00 00 00    	movq	$0x2c, 0x198(%rbp)
1416f1532: 48 8d 15 bf ee 59 01        	leaq	0x159eebf(%rip), %rdx   # 0x142c903f8
1416f1539: 48 89 95 a8 01 00 00        	movq	%rdx, 0x1a8(%rbp)
1416f1540: 48 c7 85 b0 01 00 00 01 00 00 00    	movq	$0x1, 0x1b0(%rbp)
1416f154b: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1416f154f: 48 89 95 b8 01 00 00        	movq	%rdx, 0x1b8(%rbp)
1416f1556: 48 c7 85 c0 01 00 00 01 00 00 00    	movq	$0x1, 0x1c0(%rbp)
1416f1561: 48 c7 85 c8 01 00 00 00 00 00 00    	movq	$0x0, 0x1c8(%rbp)
1416f156c: 48 c7 85 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%rbp)
1416f1577: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
1416f157e: 48 c7 85 68 01 00 00 2c 00 00 00    	movq	$0x2c, 0x168(%rbp)
1416f1589: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1416f1594: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
1416f159b: c7 85 a0 01 00 00 01 00 00 00       	movl	$0x1, 0x1a0(%rbp)
1416f15a5: 89 85 a4 01 00 00           	movl	%eax, 0x1a4(%rbp)
1416f15ab: 48 8d 8d 38 01 00 00        	leaq	0x138(%rbp), %rcx
1416f15b2: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1416f15b9: e8 72 8d 65 ff              	callq	0x140d4a330 <.text+0xd49330>
1416f15be: 48 8b b5 38 04 00 00        	movq	0x438(%rbp), %rsi
1416f15c5: 4c 8b 27                    	movq	(%rdi), %r12
1416f15c8: 41 80 bc 24 f5 00 00 00 00  	cmpb	$0x0, 0xf5(%r12)
1416f15d1: 0f 84 0a 02 00 00           	je	0x1416f17e1 <.text+0x16f07e1>
1416f15d7: 41 80 bc 24 f0 00 00 00 00  	cmpb	$0x0, 0xf0(%r12)
1416f15e0: 0f 84 fb 01 00 00           	je	0x1416f17e1 <.text+0x16f07e1>
1416f15e6: 48 8b 85 28 04 00 00        	movq	0x428(%rbp), %rax
1416f15ed: 48 8b 18                    	movq	(%rax), %rbx
1416f15f0: 48 85 db                    	testq	%rbx, %rbx
1416f15f3: 0f 84 74 02 00 00           	je	0x1416f186d <.text+0x16f086d>
1416f15f9: 49 81 c4 f5 00 00 00        	addq	$0xf5, %r12
1416f1600: 48 8d 15 45 ee 59 01        	leaq	0x159ee45(%rip), %rdx   # 0x142c9044c
1416f1607: 4c 8d 7d b0                 	leaq	-0x50(%rbp), %r15
1416f160b: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1416f1611: 4c 89 f9                    	movq	%r15, %rcx
1416f1614: e8 d7 9c 3e ff              	callq	0x140adb2f0 <.text+0xada2f0>
1416f1619: 8b 47 1c                    	movl	0x1c(%rdi), %eax
1416f161c: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
1416f1620: 89 01                       	movl	%eax, (%rcx)
1416f1622: 4c 89 a5 d8 00 00 00        	movq	%r12, 0xd8(%rbp)
1416f1629: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
1416f1630: 41 b8 88 01 00 00           	movl	$0x188, %r8d            # imm = 0x188
1416f1636: 4c 89 fa                    	movq	%r15, %rdx
1416f1639: e8 c2 25 23 01              	callq	0x142923c00 <.text+0x2922c00>
1416f163e: 0f b6 05 c4 35 f4 01        	movzbl	0x1f435c4(%rip), %eax   # 0x143634c09
1416f1645: b9 48 00 00 00              	movl	$0x48, %ecx
1416f164a: ba 08 00 00 00              	movl	$0x8, %edx
1416f164f: e8 bc 25 f2 fe              	callq	0x140613c10 <.text+0x612c10>
1416f1654: 48 85 c0                    	testq	%rax, %rax
1416f1657: 48 89 85 58 03 00 00        	movq	%rax, 0x358(%rbp)
1416f165e: 0f 84 fa 0d 00 00           	je	0x1416f245e <.text+0x16f145e>
1416f1664: 49 89 c3                    	movq	%rax, %r11
1416f1667: 4d 8d 55 40                 	leaq	0x40(%r13), %r10
1416f166b: 4d 8d 4d 48                 	leaq	0x48(%r13), %r9
1416f166f: 49 8d 45 18                 	leaq	0x18(%r13), %rax
1416f1673: 49 8d 4d 38                 	leaq	0x38(%r13), %rcx
1416f1677: 49 8d 55 30                 	leaq	0x30(%r13), %rdx
1416f167b: 4c 8d 85 67 03 00 00        	leaq	0x367(%rbp), %r8
1416f1682: 4d 89 03                    	movq	%r8, (%r11)
1416f1685: 4c 89 95 18 03 00 00        	movq	%r10, 0x318(%rbp)
1416f168c: 4d 89 53 08                 	movq	%r10, 0x8(%r11)
1416f1690: 4c 89 8d f0 02 00 00        	movq	%r9, 0x2f0(%rbp)
1416f1697: 4d 89 4b 10                 	movq	%r9, 0x10(%r11)
1416f169b: 49 89 53 18                 	movq	%rdx, 0x18(%r11)
1416f169f: 49 89 4b 20                 	movq	%rcx, 0x20(%r11)
1416f16a3: 4d 89 6b 28                 	movq	%r13, 0x28(%r11)
1416f16a7: 49 89 43 30                 	movq	%rax, 0x30(%r11)
1416f16ab: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
1416f16b2: 49 89 43 38                 	movq	%rax, 0x38(%r11)
1416f16b6: 49 89 73 40                 	movq	%rsi, 0x40(%r11)
1416f16ba: 48 8d 05 5f e9 59 01        	leaq	0x159e95f(%rip), %rax   # 0x142c90020
1416f16c1: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1416f16c6: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416f16ca: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1416f16d1: 49 89 d8                    	movq	%rbx, %r8
1416f16d4: 4d 89 d9                    	movq	%r11, %r9
1416f16d7: e8 34 9f 3e ff              	callq	0x140adb610 <.text+0xada610>
1416f16dc: 4c 89 ad 28 03 00 00        	movq	%r13, 0x328(%rbp)
1416f16e3: 48 8b b5 20 04 00 00        	movq	0x420(%rbp), %rsi
1416f16ea: 83 7d f0 02                 	cmpl	$0x2, -0x10(%rbp)
1416f16ee: 74 13                       	je	0x1416f1703 <.text+0x16f0703>
1416f16f0: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1416f16f4: f0                          	lock
1416f16f5: 48 ff 08                    	decq	(%rax)
1416f16f8: 75 09                       	jne	0x1416f1703 <.text+0x16f0703>
1416f16fa: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1416f16fe: e8 6d fa 54 ff              	callq	0x140c41170 <.text+0xc40170>
1416f1703: 48 8b 06                    	movq	(%rsi), %rax
1416f1706: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1416f170d: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
1416f1714: 48 8b 08                    	movq	(%rax), %rcx
1416f1717: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
1416f171e: 48 8b 10                    	movq	(%rax), %rdx
1416f1721: 4c 8b 40 08                 	movq	0x8(%rax), %r8
1416f1725: 8b 40 14                    	movl	0x14(%rax), %eax
1416f1728: 89 85 44 03 00 00           	movl	%eax, 0x344(%rbp)
1416f172e: 4c 8b b2 f8 00 00 00        	movq	0xf8(%rdx), %r14
1416f1735: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
1416f173c: 4d 8d 14 86                 	leaq	(%r14,%rax,4), %r10
1416f1740: 48 89 95 00 03 00 00        	movq	%rdx, 0x300(%rbp)
1416f1747: 0f b6 82 08 01 00 00        	movzbl	0x108(%rdx), %eax
1416f174e: 88 85 50 03 00 00           	movb	%al, 0x350(%rbp)
1416f1754: f3 0f 10 41 10              	movss	0x10(%rcx), %xmm0
1416f1759: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1416f175d: f2 0f 10 49 14              	movsd	0x14(%rcx), %xmm1
1416f1762: 0f 59 c9                    	mulps	%xmm1, %xmm1
1416f1765: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1416f1769: f3 0f 16 c9                 	movshdup	%xmm1, %xmm1            # xmm1 = xmm1[1,1,3,3]
1416f176d: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1416f1771: f3 0f 51 f1                 	sqrtss	%xmm1, %xmm6
1416f1775: 48 89 8d 30 03 00 00        	movq	%rcx, 0x330(%rbp)
1416f177c: 48 8d 41 04                 	leaq	0x4(%rcx), %rax
1416f1780: 48 89 85 08 03 00 00        	movq	%rax, 0x308(%rbp)
1416f1787: b8 08 00 00 00              	movl	$0x8, %eax
1416f178c: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
1416f1793: 48 c7 85 58 03 00 00 00 00 00 00    	movq	$0x0, 0x358(%rbp)
1416f179e: 0f 57 ff                    	xorps	%xmm7, %xmm7
1416f17a1: f2 44 0f 10 05 a6 fc 38 01  	movsd	0x138fca6(%rip), %xmm8  # 0x142a81450
1416f17aa: f3 44 0f 10 0d c1 ee 26 01  	movss	0x126eec1(%rip), %xmm9  # 0x142960674
1416f17b3: 48 c7 85 48 03 00 00 00 00 00 00    	movq	$0x0, 0x348(%rbp)
1416f17be: 31 db                       	xorl	%ebx, %ebx
1416f17c0: 45 31 e4                    	xorl	%r12d, %r12d
1416f17c3: 48 c7 85 f8 02 00 00 00 00 00 00    	movq	$0x0, 0x2f8(%rbp)
1416f17ce: 4c 89 85 48 01 00 00        	movq	%r8, 0x148(%rbp)
1416f17d5: 4c 89 95 50 01 00 00        	movq	%r10, 0x150(%rbp)
1416f17dc: e9 87 04 00 00              	jmp	0x1416f1c68 <.text+0x16f0c68>
1416f17e1: 48 8b 5e 08                 	movq	0x8(%rsi), %rbx
1416f17e5: 48 85 db                    	testq	%rbx, %rbx
1416f17e8: 75 04                       	jne	0x1416f17ee <.text+0x16f07ee>
1416f17ea: 48 8b 5e 10                 	movq	0x10(%rsi), %rbx
1416f17ee: 4d 8b 65 30                 	movq	0x30(%r13), %r12
1416f17f2: 48 8b 03                    	movq	(%rbx), %rax
1416f17f5: 4c 8b 73 10                 	movq	0x10(%rbx), %r14
1416f17f9: 4c 29 f0                    	subq	%r14, %rax
1416f17fc: 48 83 f8 17                 	cmpq	$0x17, %rax
1416f1800: 0f 86 bb 0a 00 00           	jbe	0x1416f22c1 <.text+0x16f12c1>
1416f1806: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1416f180a: 48 8d 3d 5f b6 be ff        	leaq	-0x4149a1(%rip), %rdi   # 0x1412dce70 <.text+0x12dbe70>
1416f1811: 4a 89 3c 30                 	movq	%rdi, (%rax,%r14)
1416f1815: 4c 8d 3d b4 c3 be ff        	leaq	-0x413c4c(%rip), %r15   # 0x1412ddbd0 <.text+0x12dcbd0>
1416f181c: 4e 89 7c 30 08              	movq	%r15, 0x8(%rax,%r14)
1416f1821: 4e 89 64 30 10              	movq	%r12, 0x10(%rax,%r14)
1416f1826: 49 83 c6 18                 	addq	$0x18, %r14
1416f182a: 4c 89 73 10                 	movq	%r14, 0x10(%rbx)
1416f182e: 48 8b 5e 08                 	movq	0x8(%rsi), %rbx
1416f1832: 48 85 db                    	testq	%rbx, %rbx
1416f1835: 75 04                       	jne	0x1416f183b <.text+0x16f083b>
1416f1837: 48 8b 5e 10                 	movq	0x10(%rsi), %rbx
1416f183b: 49 8b 75 38                 	movq	0x38(%r13), %rsi
1416f183f: 48 8b 03                    	movq	(%rbx), %rax
1416f1842: 4c 8b 73 10                 	movq	0x10(%rbx), %r14
1416f1846: 4c 29 f0                    	subq	%r14, %rax
1416f1849: 48 83 f8 17                 	cmpq	$0x17, %rax
1416f184d: 0f 86 93 0a 00 00           	jbe	0x1416f22e6 <.text+0x16f12e6>
1416f1853: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1416f1857: 4a 89 3c 30                 	movq	%rdi, (%rax,%r14)
1416f185b: 4e 89 7c 30 08              	movq	%r15, 0x8(%rax,%r14)
1416f1860: 4a 89 74 30 10              	movq	%rsi, 0x10(%rax,%r14)
1416f1865: 49 83 c6 18                 	addq	$0x18, %r14
1416f1869: 4c 89 73 10                 	movq	%r14, 0x10(%rbx)
1416f186d: 48 8b b5 48 04 00 00        	movq	0x448(%rbp), %rsi
1416f1874: 49 8b 45 00                 	movq	(%r13), %rax
1416f1878: 80 bd 67 03 00 00 00        	cmpb	$0x0, 0x367(%rbp)
1416f187f: 0f 84 34 01 00 00           	je	0x1416f19b9 <.text+0x16f09b9>
1416f1885: 48 f7 d8                    	negq	%rax
1416f1888: 0f 81 5a 01 00 00           	jno	0x1416f19e8 <.text+0x16f09e8>
1416f188e: 4c 8b 85 38 03 00 00        	movq	0x338(%rbp), %r8
1416f1895: 49 8b 00                    	movq	(%r8), %rax
1416f1898: 49 8b 48 08                 	movq	0x8(%r8), %rcx
1416f189c: 41 8b 50 10                 	movl	0x10(%r8), %edx
1416f18a0: 45 8b 40 14                 	movl	0x14(%r8), %r8d
1416f18a4: 4c 8d 89 a0 01 00 00        	leaq	0x1a0(%rcx), %r9
1416f18ab: 4c 8d 91 f8 00 00 00        	leaq	0xf8(%rcx), %r10
1416f18b2: 4c 8b 98 f8 00 00 00        	movq	0xf8(%rax), %r11
1416f18b9: 48 8b b8 00 01 00 00        	movq	0x100(%rax), %rdi
1416f18c0: 49 8d 3c bb                 	leaq	(%r11,%rdi,4), %rdi
1416f18c4: 0f b6 98 08 01 00 00        	movzbl	0x108(%rax), %ebx
1416f18cb: 88 9d 18 02 00 00           	movb	%bl, 0x218(%rbp)
1416f18d1: 4c 89 9d 78 01 00 00        	movq	%r11, 0x178(%rbp)
1416f18d8: 48 89 bd 80 01 00 00        	movq	%rdi, 0x180(%rbp)
1416f18df: 48 c7 85 88 01 00 00 08 00 00 00    	movq	$0x8, 0x188(%rbp)
1416f18ea: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
1416f18f5: 48 c7 85 98 01 00 00 08 00 00 00    	movq	$0x8, 0x198(%rbp)
1416f1900: 0f 57 c0                    	xorps	%xmm0, %xmm0
1416f1903: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
1416f190a: 89 95 c0 01 00 00           	movl	%edx, 0x1c0(%rbp)
1416f1910: 44 89 85 c4 01 00 00        	movl	%r8d, 0x1c4(%rbp)
1416f1917: 48 c7 85 c8 01 00 00 00 00 00 00    	movq	$0x0, 0x1c8(%rbp)
1416f1922: 89 95 e0 01 00 00           	movl	%edx, 0x1e0(%rbp)
1416f1928: 44 89 85 e4 01 00 00        	movl	%r8d, 0x1e4(%rbp)
1416f192f: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
1416f193a: 89 95 00 02 00 00           	movl	%edx, 0x200(%rbp)
1416f1940: 44 89 85 04 02 00 00        	movl	%r8d, 0x204(%rbp)
1416f1947: 0f 11 85 08 02 00 00        	movups	%xmm0, 0x208(%rbp)
1416f194e: 48 89 8d 58 01 00 00        	movq	%rcx, 0x158(%rbp)
1416f1955: 4c 89 8d 60 01 00 00        	movq	%r9, 0x160(%rbp)
1416f195c: 4c 89 95 68 01 00 00        	movq	%r10, 0x168(%rbp)
1416f1963: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1416f196a: 4c 8d 05 b7 c4 57 01        	leaq	0x157c4b7(%rip), %r8    # 0x142c6de28
1416f1971: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416f1975: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1416f197c: e8 4f 32 c1 ff              	callq	0x141304bd0 <.text+0x1303bd0>
1416f1981: 49 8b 45 00                 	movq	(%r13), %rax
1416f1985: 48 89 c1                    	movq	%rax, %rcx
1416f1988: 48 f7 d9                    	negq	%rcx
1416f198b: 70 19                       	jo	0x1416f19a6 <.text+0x16f09a6>
1416f198d: 73 17                       	jae	0x1416f19a6 <.text+0x16f09a6>
1416f198f: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1416f1993: 48 c1 e0 03                 	shlq	$0x3, %rax
1416f1997: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1416f199b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416f19a1: e8 7a 22 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416f19a6: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1416f19aa: 49 89 45 10                 	movq	%rax, 0x10(%r13)
1416f19ae: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
1416f19b2: 41 0f 11 45 00              	movups	%xmm0, (%r13)
1416f19b7: eb 2f                       	jmp	0x1416f19e8 <.text+0x16f09e8>
1416f19b9: 48 89 c1                    	movq	%rax, %rcx
1416f19bc: 48 f7 d9                    	negq	%rcx
1416f19bf: 70 19                       	jo	0x1416f19da <.text+0x16f09da>
1416f19c1: 73 17                       	jae	0x1416f19da <.text+0x16f09da>
1416f19c3: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1416f19c7: 48 c1 e0 03                 	shlq	$0x3, %rax
1416f19cb: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1416f19cf: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416f19d5: e8 46 22 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416f19da: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416f19e4: 49 89 45 00                 	movq	%rax, (%r13)
1416f19e8: 48 8b 06                    	movq	(%rsi), %rax
1416f19eb: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
1416f19ef: 48 8b 10                    	movq	(%rax), %rdx
1416f19f2: 45 31 c0                    	xorl	%r8d, %r8d
1416f19f5: 49 89 d1                    	movq	%rdx, %r9
1416f19f8: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
1416f19fc: 4d 0f 42 c8                 	cmovbq	%r8, %r9
1416f1a00: 48 2b 51 38                 	subq	0x38(%rcx), %rdx
1416f1a04: 49 0f 42 d0                 	cmovbq	%r8, %rdx
1416f1a08: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
1416f1a0c: 49 29 d2                    	subq	%rdx, %r10
1416f1a0f: 4d 0f 42 d0                 	cmovbq	%r8, %r10
1416f1a13: 0f 96 c2                    	setbe	%dl
1416f1a16: 4c 8b 59 10                 	movq	0x10(%rcx), %r11
1416f1a1a: 4d 29 cb                    	subq	%r9, %r11
1416f1a1d: 4d 0f 42 d8                 	cmovbq	%r8, %r11
1416f1a21: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
1416f1a25: 41 0f 96 c0                 	setbe	%r8b
1416f1a29: 4d 01 d3                    	addq	%r10, %r11
1416f1a2c: 49 89 c9                    	movq	%rcx, %r9
1416f1a2f: 4d 29 d9                    	subq	%r11, %r9
1416f1a32: 4c 89 08                    	movq	%r9, (%rax)
1416f1a35: 41 84 d0                    	testb	%dl, %r8b
1416f1a38: 75 36                       	jne	0x1416f1a70 <.text+0x16f0a70>
1416f1a3a: 48 89 08                    	movq	%rcx, (%rax)
1416f1a3d: 49 8b 45 00                 	movq	(%r13), %rax
1416f1a41: 48 89 c1                    	movq	%rax, %rcx
1416f1a44: 48 f7 d9                    	negq	%rcx
1416f1a47: 70 19                       	jo	0x1416f1a62 <.text+0x16f0a62>
1416f1a49: 73 17                       	jae	0x1416f1a62 <.text+0x16f0a62>
1416f1a4b: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1416f1a4f: 48 c1 e0 03                 	shlq	$0x3, %rax
1416f1a53: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1416f1a57: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416f1a5d: e8 be 21 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416f1a62: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416f1a6c: 49 89 45 00                 	movq	%rax, (%r13)
1416f1a70: 0f 28 b5 70 03 00 00        	movaps	0x370(%rbp), %xmm6
1416f1a77: 0f 28 bd 80 03 00 00        	movaps	0x380(%rbp), %xmm7
1416f1a7e: 44 0f 28 85 90 03 00 00     	movaps	0x390(%rbp), %xmm8
1416f1a86: 44 0f 28 8d a0 03 00 00     	movaps	0x3a0(%rbp), %xmm9
1416f1a8e: 48 81 c4 38 04 00 00        	addq	$0x438, %rsp            # imm = 0x438
1416f1a95: 5b                          	popq	%rbx
1416f1a96: 5f                          	popq	%rdi
1416f1a97: 5e                          	popq	%rsi
1416f1a98: 41 5c                       	popq	%r12
1416f1a9a: 41 5d                       	popq	%r13
1416f1a9c: 41 5e                       	popq	%r14
1416f1a9e: 41 5f                       	popq	%r15
1416f1aa0: 5d                          	popq	%rbp
1416f1aa1: c3                          	retq
1416f1aa2: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1416f1aa6: 48 8b 48 18                 	movq	0x18(%rax), %rcx
1416f1aaa: 48 8b 40 38                 	movq	0x38(%rax), %rax
1416f1aae: 4c 8b 85 00 03 00 00        	movq	0x300(%rbp), %r8
1416f1ab5: 49 8b 90 10 01 00 00        	movq	0x110(%r8), %rdx
1416f1abc: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
1416f1ac3: 48 8b 14 d0                 	movq	(%rax,%rdx,8), %rdx
1416f1ac7: 48 f7 d2                    	notq	%rdx
1416f1aca: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1416f1ace: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416f1ad2: 4c 8b 4c 11 10              	movq	0x10(%rcx,%rdx), %r9
1416f1ad7: 4c 89 8d 48 03 00 00        	movq	%r9, 0x348(%rbp)
1416f1ade: 48 8b 54 11 28              	movq	0x28(%rcx,%rdx), %rdx
1416f1ae3: 48 89 95 10 03 00 00        	movq	%rdx, 0x310(%rbp)
1416f1aea: 4a 8b 04 c0                 	movq	(%rax,%r8,8), %rax
1416f1aee: 48 f7 d0                    	notq	%rax
1416f1af1: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1416f1af5: 48 c1 e0 04                 	shlq	$0x4, %rax
1416f1af9: 48 8b 44 01 10              	movq	0x10(%rcx,%rax), %rax
1416f1afe: 48 89 85 58 03 00 00        	movq	%rax, 0x358(%rbp)
1416f1b05: 45 31 e4                    	xorl	%r12d, %r12d
1416f1b08: 44 89 e0                    	movl	%r12d, %eax
1416f1b0b: 48 8d 34 40                 	leaq	(%rax,%rax,2), %rsi
1416f1b0f: 48 c1 e6 04                 	shlq	$0x4, %rsi
1416f1b13: 48 03 b5 48 03 00 00        	addq	0x348(%rbp), %rsi
1416f1b1a: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
1416f1b21: 48 8d 0c 81                 	leaq	(%rcx,%rax,4), %rcx
1416f1b25: 48 89 8d e0 02 00 00        	movq	%rcx, 0x2e0(%rbp)
1416f1b2c: 4c 6b f8 68                 	imulq	$0x68, %rax, %r15
1416f1b30: 4c 03 bd 58 03 00 00        	addq	0x358(%rbp), %r15
1416f1b37: 49 ff c4                    	incq	%r12
1416f1b3a: 49 8d 47 20                 	leaq	0x20(%r15), %rax
1416f1b3e: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
1416f1b45: 48 8d 05 74 bb 69 ff        	leaq	-0x96448c(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
1416f1b4c: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1416f1b53: 48 8d 05 4e ea 59 01        	leaq	0x159ea4e(%rip), %rax   # 0x142c905a8
1416f1b5a: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1416f1b61: 48 c7 85 60 01 00 00 01 00 00 00    	movq	$0x1, 0x160(%rbp)
1416f1b6c: 48 8d 85 38 01 00 00        	leaq	0x138(%rbp), %rax
1416f1b73: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
1416f1b7a: 48 c7 85 70 01 00 00 01 00 00 00    	movq	$0x1, 0x170(%rbp)
1416f1b85: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
1416f1b90: 48 8d 7d b0                 	leaq	-0x50(%rbp), %rdi
1416f1b94: 48 89 f9                    	movq	%rdi, %rcx
1416f1b97: 4c 8d ad 58 01 00 00        	leaq	0x158(%rbp), %r13
1416f1b9e: 4c 89 ea                    	movq	%r13, %rdx
1416f1ba1: e8 ea 83 68 ff              	callq	0x140d79f90 <.text+0xd78f90>
1416f1ba6: f3 0f 10 46 18              	movss	0x18(%rsi), %xmm0
1416f1bab: f3 0f 58 c7                 	addss	%xmm7, %xmm0
1416f1baf: f2 0f 10 4e 10              	movsd	0x10(%rsi), %xmm1
1416f1bb4: 41 0f 58 c8                 	addps	%xmm8, %xmm1
1416f1bb8: c6 85 e0 01 00 00 00        	movb	$0x0, 0x1e0(%rbp)
1416f1bbf: c7 85 e4 01 00 00 00 00 a0 3f       	movl	$0x3fa00000, 0x1e4(%rbp) # imm = 0x3FA00000
1416f1bc9: 0f 13 8d e8 01 00 00        	movlps	%xmm1, 0x1e8(%rbp)
1416f1bd0: f3 0f 11 85 f0 01 00 00     	movss	%xmm0, 0x1f0(%rbp)
1416f1bd8: 66 c7 85 d0 01 00 00 01 0e  	movw	$0xe01, 0x1d0(%rbp)     # imm = 0xE01
1416f1be1: c7 85 70 01 00 00 01 00 00 00       	movl	$0x1, 0x170(%rbp)
1416f1beb: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
1416f1bf6: c7 85 88 01 00 00 02 00 00 00       	movl	$0x2, 0x188(%rbp)
1416f1c00: c7 85 a0 01 00 00 03 00 00 00       	movl	$0x3, 0x1a0(%rbp)
1416f1c0a: c7 85 b8 01 00 00 03 00 00 00       	movl	$0x3, 0x1b8(%rbp)
1416f1c14: 48 c7 85 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%rbp)
1416f1c1f: 48 c7 85 60 01 00 00 08 00 00 00    	movq	$0x8, 0x160(%rbp)
1416f1c2a: 48 c7 85 68 01 00 00 00 00 00 00    	movq	$0x0, 0x168(%rbp)
1416f1c35: 66 c7 85 00 02 00 00 01 01  	movw	$0x101, 0x200(%rbp)     # imm = 0x101
1416f1c3e: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
1416f1c45: 48 89 fa                    	movq	%rdi, %rdx
1416f1c48: 4d 89 e8                    	movq	%r13, %r8
1416f1c4b: e8 a0 15 23 ff              	callq	0x1409231f0 <.text+0x9221f0>
1416f1c50: 80 78 18 02                 	cmpb	$0x2, 0x18(%rax)
1416f1c54: 4c 8b 85 48 01 00 00        	movq	0x148(%rbp), %r8
1416f1c5b: 4c 8b 95 50 01 00 00        	movq	0x150(%rbp), %r10
1416f1c62: 0f 83 1b 01 00 00           	jae	0x1416f1d83 <.text+0x16f0d83>
1416f1c68: 80 bd 50 03 00 00 00        	cmpb	$0x0, 0x350(%rbp)
1416f1c6f: 74 3f                       	je	0x1416f1cb0 <.text+0x16f0cb0>
1416f1c71: 49 39 dc                    	cmpq	%rbx, %r12
1416f1c74: 0f 85 8e fe ff ff           	jne	0x1416f1b08 <.text+0x16f0b08>
1416f1c7a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1416f1c80: 4d 39 d6                    	cmpq	%r10, %r14
1416f1c83: 0f 84 8f 01 00 00           	je	0x1416f1e18 <.text+0x16f0e18>
1416f1c89: 41 8b 0e                    	movl	(%r14), %ecx
1416f1c8c: 49 83 c6 04                 	addq	$0x4, %r14
1416f1c90: 49 8b 80 a8 01 00 00        	movq	0x1a8(%r8), %rax
1416f1c97: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1416f1c9b: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
1416f1ca0: 48 85 db                    	testq	%rbx, %rbx
1416f1ca3: 74 db                       	je	0x1416f1c80 <.text+0x16f0c80>
1416f1ca5: e9 f8 fd ff ff              	jmp	0x1416f1aa2 <.text+0x16f0aa2>
1416f1caa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1416f1cb0: 49 39 dc                    	cmpq	%rbx, %r12
1416f1cb3: 0f 85 b3 00 00 00           	jne	0x1416f1d6c <.text+0x16f0d6c>
1416f1cb9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1416f1cc0: 4d 39 d6                    	cmpq	%r10, %r14
1416f1cc3: 0f 84 4f 01 00 00           	je	0x1416f1e18 <.text+0x16f0e18>
1416f1cc9: 41 8b 0e                    	movl	(%r14), %ecx
1416f1ccc: 49 83 c6 04                 	addq	$0x4, %r14
1416f1cd0: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
1416f1cd7: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1416f1cdb: 48 c1 e1 05                 	shlq	$0x5, %rcx
1416f1cdf: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
1416f1ce4: 48 85 db                    	testq	%rbx, %rbx
1416f1ce7: 74 d7                       	je	0x1416f1cc0 <.text+0x16f0cc0>
1416f1ce9: 48 01 c8                    	addq	%rcx, %rax
1416f1cec: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
1416f1cf2: 49 8b 90 a8 01 00 00        	movq	0x1a8(%r8), %rdx
1416f1cf9: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1416f1cfd: 4c 8b 44 ca 18              	movq	0x18(%rdx,%rcx,8), %r8
1416f1d02: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
1416f1d07: 4c 8b 8d 00 03 00 00        	movq	0x300(%rbp), %r9
1416f1d0e: 49 8b 91 10 01 00 00        	movq	0x110(%r9), %rdx
1416f1d15: 4d 8b 89 18 01 00 00        	movq	0x118(%r9), %r9
1416f1d1c: 48 8b 14 d1                 	movq	(%rcx,%rdx,8), %rdx
1416f1d20: 48 f7 d2                    	notq	%rdx
1416f1d23: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1416f1d27: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416f1d2b: 4d 8b 54 10 10              	movq	0x10(%r8,%rdx), %r10
1416f1d30: 4c 89 95 48 03 00 00        	movq	%r10, 0x348(%rbp)
1416f1d37: 49 8b 54 10 28              	movq	0x28(%r8,%rdx), %rdx
1416f1d3c: 48 89 95 10 03 00 00        	movq	%rdx, 0x310(%rbp)
1416f1d43: 4a 8b 0c c9                 	movq	(%rcx,%r9,8), %rcx
1416f1d47: 48 f7 d1                    	notq	%rcx
1416f1d4a: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1416f1d4e: 48 c1 e1 04                 	shlq	$0x4, %rcx
1416f1d52: 49 8b 4c 08 10              	movq	0x10(%r8,%rcx), %rcx
1416f1d57: 48 89 8d 58 03 00 00        	movq	%rcx, 0x358(%rbp)
1416f1d5e: 48 8b 40 50                 	movq	0x50(%rax), %rax
1416f1d62: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
1416f1d69: 45 31 e4                    	xorl	%r12d, %r12d
1416f1d6c: 4c 89 e0                    	movq	%r12, %rax
1416f1d6f: 48 c1 e0 04                 	shlq	$0x4, %rax
1416f1d73: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
1416f1d7a: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
1416f1d7e: e9 88 fd ff ff              	jmp	0x1416f1b0b <.text+0x16f0b0b>
1416f1d83: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
1416f1d87: 75 5c                       	jne	0x1416f1de5 <.text+0x16f0de5>
1416f1d89: 80 78 19 04                 	cmpb	$0x4, 0x19(%rax)
1416f1d8d: 75 56                       	jne	0x1416f1de5 <.text+0x16f0de5>
1416f1d8f: 41 0f 2e f1                 	ucomiss	%xmm9, %xmm6
1416f1d93: 76 50                       	jbe	0x1416f1de5 <.text+0x16f0de5>
1416f1d95: 0f 57 c0                    	xorps	%xmm0, %xmm0
1416f1d98: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
1416f1d9f: f6 01 01                    	testb	$0x1, (%rcx)
1416f1da2: 0f 57 c9                    	xorps	%xmm1, %xmm1
1416f1da5: 74 11                       	je	0x1416f1db8 <.text+0x16f0db8>
1416f1da7: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
1416f1dae: f3 0f 10 49 04              	movss	0x4(%rcx), %xmm1
1416f1db3: f3 0f 10 41 0c              	movss	0xc(%rcx), %xmm0
1416f1db8: f3 0f 11 4e 10              	movss	%xmm1, 0x10(%rsi)
1416f1dbd: 8b 8d 44 03 00 00           	movl	0x344(%rbp), %ecx
1416f1dc3: 4c 8b 9d e0 02 00 00        	movq	0x2e0(%rbp), %r11
1416f1dca: 41 89 0b                    	movl	%ecx, (%r11)
1416f1dcd: f3 0f 11 46 18              	movss	%xmm0, 0x18(%rsi)
1416f1dd2: b1 01                       	movb	$0x1, %cl
1416f1dd4: 48 89 8d f8 02 00 00        	movq	%rcx, 0x2f8(%rbp)
1416f1ddb: 80 78 18 02                 	cmpb	$0x2, 0x18(%rax)
1416f1ddf: 0f 82 83 fe ff ff           	jb	0x1416f1c68 <.text+0x16f0c68>
1416f1de5: 0f b6 48 19                 	movzbl	0x19(%rax), %ecx
1416f1de9: 80 f9 01                    	cmpb	$0x1, %cl
1416f1dec: 0f 87 76 fe ff ff           	ja	0x1416f1c68 <.text+0x16f0c68>
1416f1df2: f6 c1 01                    	testb	$0x1, %cl
1416f1df5: 0f 84 6d fe ff ff           	je	0x1416f1c68 <.text+0x16f0c68>
1416f1dfb: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
1416f1dff: 0f 85 63 fe ff ff           	jne	0x1416f1c68 <.text+0x16f0c68>
1416f1e05: 49 8b 47 20                 	movq	0x20(%r15), %rax
1416f1e09: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
1416f1e10: 48 89 01                    	movq	%rax, (%rcx)
1416f1e13: e9 50 fe ff ff              	jmp	0x1416f1c68 <.text+0x16f0c68>
1416f1e18: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
1416f1e1f: 0f b6 18                    	movzbl	(%rax), %ebx
1416f1e22: 84 db                       	testb	%bl, %bl
1416f1e24: 4c 8b ad 28 03 00 00        	movq	0x328(%rbp), %r13
1416f1e2b: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
1416f1e32: 0f 84 4b 03 00 00           	je	0x1416f2183 <.text+0x16f1183>
1416f1e38: f6 00 01                    	testb	$0x1, (%rax)
1416f1e3b: 74 16                       	je	0x1416f1e53 <.text+0x16f0e53>
1416f1e3d: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416f1e44: 8b 41 08                    	movl	0x8(%rcx), %eax
1416f1e47: 89 45 b8                    	movl	%eax, -0x48(%rbp)
1416f1e4a: 48 8b 01                    	movq	(%rcx), %rax
1416f1e4d: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1416f1e51: eb 0f                       	jmp	0x1416f1e62 <.text+0x16f0e62>
1416f1e53: c7 45 b8 00 00 00 00        	movl	$0x0, -0x48(%rbp)
1416f1e5a: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
1416f1e62: 8b 45 b8                    	movl	-0x48(%rbp), %eax
1416f1e65: 89 85 f0 01 00 00           	movl	%eax, 0x1f0(%rbp)
1416f1e6b: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
1416f1e6f: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
1416f1e76: 48 c7 85 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%rbp)
1416f1e81: 48 c7 85 60 01 00 00 08 00 00 00    	movq	$0x8, 0x160(%rbp)
1416f1e8c: 48 c7 85 68 01 00 00 00 00 00 00    	movq	$0x0, 0x168(%rbp)
1416f1e97: c7 85 70 01 00 00 01 00 00 00       	movl	$0x1, 0x170(%rbp)
1416f1ea1: 48 c7 85 78 01 00 00 00 00 00 00    	movq	$0x0, 0x178(%rbp)
1416f1eac: c7 85 88 01 00 00 02 00 00 00       	movl	$0x2, 0x188(%rbp)
1416f1eb6: c7 85 a0 01 00 00 03 00 00 00       	movl	$0x3, 0x1a0(%rbp)
1416f1ec0: c7 85 b8 01 00 00 03 00 00 00       	movl	$0x3, 0x1b8(%rbp)
1416f1eca: 66 c7 85 d0 01 00 00 01 01  	movw	$0x101, 0x1d0(%rbp)     # imm = 0x101
1416f1ed3: c6 85 e0 01 00 00 00        	movb	$0x0, 0x1e0(%rbp)
1416f1eda: c7 85 e4 01 00 00 00 00 80 3e       	movl	$0x3e800000, 0x1e4(%rbp) # imm = 0x3E800000
1416f1ee4: 66 c7 85 00 02 00 00 00 01  	movw	$0x100, 0x200(%rbp)     # imm = 0x100
1416f1eed: 48 8d 15 94 e6 59 01        	leaq	0x159e694(%rip), %rdx   # 0x142c90588
1416f1ef4: 4c 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %r9
1416f1efb: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1416f1f01: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
1416f1f08: e8 c3 17 23 ff              	callq	0x1409236d0 <.text+0x9226d0>
1416f1f0d: 80 78 18 02                 	cmpb	$0x2, 0x18(%rax)
1416f1f11: 0f 82 6c 02 00 00           	jb	0x1416f2183 <.text+0x16f1183>
1416f1f17: 0f b6 48 19                 	movzbl	0x19(%rax), %ecx
1416f1f1b: 80 f9 01                    	cmpb	$0x1, %cl
1416f1f1e: 0f 87 5f 02 00 00           	ja	0x1416f2183 <.text+0x16f1183>
1416f1f24: f6 c1 01                    	testb	$0x1, %cl
1416f1f27: 0f 84 56 02 00 00           	je	0x1416f2183 <.text+0x16f1183>
1416f1f2d: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
1416f1f31: 0f 85 4c 02 00 00           	jne	0x1416f2183 <.text+0x16f1183>
1416f1f37: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
1416f1f3e: 48 8b 01                    	movq	(%rcx), %rax
1416f1f41: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1416f1f45: 48 8b 90 f8 00 00 00        	movq	0xf8(%rax), %rdx
1416f1f4c: 4c 8b 80 00 01 00 00        	movq	0x100(%rax), %r8
1416f1f53: 4e 8d 04 82                 	leaq	(%rdx,%r8,4), %r8
1416f1f57: 80 b8 08 01 00 00 00        	cmpb	$0x0, 0x108(%rax)
1416f1f5e: 74 1c                       	je	0x1416f1f7c <.text+0x16f0f7c>
1416f1f60: 45 31 e4                    	xorl	%r12d, %r12d
1416f1f63: 45 31 ff                    	xorl	%r15d, %r15d
1416f1f66: 45 31 f6                    	xorl	%r14d, %r14d
1416f1f69: 48 c7 85 50 03 00 00 00 00 00 00    	movq	$0x0, 0x350(%rbp)
1416f1f74: 45 31 d2                    	xorl	%r10d, %r10d
1416f1f77: e9 c5 00 00 00              	jmp	0x1416f2041 <.text+0x16f1041>
1416f1f7c: 41 bc 08 00 00 00           	movl	$0x8, %r12d
1416f1f82: 45 31 ed                    	xorl	%r13d, %r13d
1416f1f85: 45 31 ff                    	xorl	%r15d, %r15d
1416f1f88: 45 31 f6                    	xorl	%r14d, %r14d
1416f1f8b: 48 c7 85 50 03 00 00 00 00 00 00    	movq	$0x0, 0x350(%rbp)
1416f1f96: 45 31 d2                    	xorl	%r10d, %r10d
1416f1f99: e9 a5 01 00 00              	jmp	0x1416f2143 <.text+0x16f1143>
1416f1f9e: 4f 8d 0c d9                 	leaq	(%r9,%r11,8), %r9
1416f1fa2: 4d 8b 59 18                 	movq	0x18(%r9), %r11
1416f1fa6: 49 8b 79 38                 	movq	0x38(%r9), %rdi
1416f1faa: 4c 8b 88 10 01 00 00        	movq	0x110(%rax), %r9
1416f1fb1: 48 8b b0 18 01 00 00        	movq	0x118(%rax), %rsi
1416f1fb8: 4e 8b 0c cf                 	movq	(%rdi,%r9,8), %r9
1416f1fbc: 49 f7 d1                    	notq	%r9
1416f1fbf: 4f 8d 34 49                 	leaq	(%r9,%r9,2), %r14
1416f1fc3: 49 c1 e6 04                 	shlq	$0x4, %r14
1416f1fc7: 4f 8b 4c 33 10              	movq	0x10(%r11,%r14), %r9
1416f1fcc: 4c 89 8d 50 03 00 00        	movq	%r9, 0x350(%rbp)
1416f1fd3: 4f 8b 4c 33 28              	movq	0x28(%r11,%r14), %r9
1416f1fd8: 4c 89 8d 58 03 00 00        	movq	%r9, 0x358(%rbp)
1416f1fdf: 48 8b 34 f7                 	movq	(%rdi,%rsi,8), %rsi
1416f1fe3: 48 f7 d6                    	notq	%rsi
1416f1fe6: 48 8d 34 76                 	leaq	(%rsi,%rsi,2), %rsi
1416f1fea: 48 c1 e6 04                 	shlq	$0x4, %rsi
1416f1fee: 4d 8b 74 33 10              	movq	0x10(%r11,%rsi), %r14
1416f1ff3: 4d 8b 4c 33 28              	movq	0x28(%r11,%rsi), %r9
1416f1ff8: 4c 89 8d 48 03 00 00        	movq	%r9, 0x348(%rbp)
1416f1fff: 4c 8b b8 20 01 00 00        	movq	0x120(%rax), %r15
1416f2006: 4a 8b 3c ff                 	movq	(%rdi,%r15,8), %rdi
1416f200a: 48 f7 d7                    	notq	%rdi
1416f200d: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
1416f2011: 48 c1 e7 04                 	shlq	$0x4, %rdi
1416f2015: 4d 8b 7c 3b 10              	movq	0x10(%r11,%rdi), %r15
1416f201a: 49 8b 7c 3b 28              	movq	0x28(%r11,%rdi), %rdi
1416f201f: 45 31 e4                    	xorl	%r12d, %r12d
1416f2022: 45 89 e5                    	movl	%r12d, %r13d
1416f2025: 4d 6b dd 68                 	imulq	$0x68, %r13, %r11
1416f2029: 49 ff c4                    	incq	%r12
1416f202c: 4b 8b 74 1e 20              	movq	0x20(%r14,%r11), %rsi
1416f2031: 4c 8b 8d 18 03 00 00        	movq	0x318(%rbp), %r9
1416f2038: 49 3b 31                    	cmpq	(%r9), %rsi
1416f203b: 0f 84 ca 02 00 00           	je	0x1416f230b <.text+0x16f130b>
1416f2041: 4d 39 d4                    	cmpq	%r10, %r12
1416f2044: 75 dc                       	jne	0x1416f2022 <.text+0x16f1022>
1416f2046: 4c 8b ad 28 03 00 00        	movq	0x328(%rbp), %r13
1416f204d: 4c 39 c2                    	cmpq	%r8, %rdx
1416f2050: 0f 84 23 01 00 00           	je	0x1416f2179 <.text+0x16f1179>
1416f2056: 44 8b 12                    	movl	(%rdx), %r10d
1416f2059: 48 83 c2 04                 	addq	$0x4, %rdx
1416f205d: 4c 8b 89 a8 01 00 00        	movq	0x1a8(%rcx), %r9
1416f2064: 4f 8d 1c d2                 	leaq	(%r10,%r10,8), %r11
1416f2068: 4f 8b 54 d9 10              	movq	0x10(%r9,%r11,8), %r10
1416f206d: 4d 85 d2                    	testq	%r10, %r10
1416f2070: 74 db                       	je	0x1416f204d <.text+0x16f104d>
1416f2072: e9 27 ff ff ff              	jmp	0x1416f1f9e <.text+0x16f0f9e>
1416f2077: 4d 01 cc                    	addq	%r9, %r12
1416f207a: 45 8b 8c 24 94 00 00 00     	movl	0x94(%r12), %r9d
1416f2082: 4c 8b 99 a8 01 00 00        	movq	0x1a8(%rcx), %r11
1416f2089: 4f 8d 0c c9                 	leaq	(%r9,%r9,8), %r9
1416f208d: 4b 8b 74 cb 18              	movq	0x18(%r11,%r9,8), %rsi
1416f2092: 4f 8b 5c cb 38              	movq	0x38(%r11,%r9,8), %r11
1416f2097: 4c 8b 88 10 01 00 00        	movq	0x110(%rax), %r9
1416f209e: 48 8b b8 18 01 00 00        	movq	0x118(%rax), %rdi
1416f20a5: 4f 8b 0c cb                 	movq	(%r11,%r9,8), %r9
1416f20a9: 49 f7 d1                    	notq	%r9
1416f20ac: 4f 8d 34 49                 	leaq	(%r9,%r9,2), %r14
1416f20b0: 49 c1 e6 04                 	shlq	$0x4, %r14
1416f20b4: 4e 8b 4c 36 10              	movq	0x10(%rsi,%r14), %r9
1416f20b9: 4c 89 8d 50 03 00 00        	movq	%r9, 0x350(%rbp)
1416f20c0: 4e 8b 4c 36 28              	movq	0x28(%rsi,%r14), %r9
1416f20c5: 4c 89 8d 58 03 00 00        	movq	%r9, 0x358(%rbp)
1416f20cc: 49 8b 3c fb                 	movq	(%r11,%rdi,8), %rdi
1416f20d0: 48 f7 d7                    	notq	%rdi
1416f20d3: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
1416f20d7: 48 c1 e7 04                 	shlq	$0x4, %rdi
1416f20db: 4c 8b 74 3e 10              	movq	0x10(%rsi,%rdi), %r14
1416f20e0: 4c 8b 4c 3e 28              	movq	0x28(%rsi,%rdi), %r9
1416f20e5: 4c 89 8d 48 03 00 00        	movq	%r9, 0x348(%rbp)
1416f20ec: 48 8b b8 20 01 00 00        	movq	0x120(%rax), %rdi
1416f20f3: 4d 8b 1c fb                 	movq	(%r11,%rdi,8), %r11
1416f20f7: 49 f7 d3                    	notq	%r11
1416f20fa: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1416f20fe: 49 c1 e3 04                 	shlq	$0x4, %r11
1416f2102: 4e 8b 7c 1e 10              	movq	0x10(%rsi,%r11), %r15
1416f2107: 4e 8b 4c 1e 28              	movq	0x28(%rsi,%r11), %r9
1416f210c: 4c 89 8d 20 03 00 00        	movq	%r9, 0x320(%rbp)
1416f2113: 4d 8b 64 24 50              	movq	0x50(%r12), %r12
1416f2118: 45 31 ed                    	xorl	%r13d, %r13d
1416f211b: 4d 89 eb                    	movq	%r13, %r11
1416f211e: 49 c1 e3 04                 	shlq	$0x4, %r11
1416f2122: 47 8b 5c 1c 08              	movl	0x8(%r12,%r11), %r11d
1416f2127: 49 6b f3 68                 	imulq	$0x68, %r11, %rsi
1416f212b: 49 ff c5                    	incq	%r13
1416f212e: 49 8b 7c 36 20              	movq	0x20(%r14,%rsi), %rdi
1416f2133: 4c 8b 8d 18 03 00 00        	movq	0x318(%rbp), %r9
1416f213a: 49 3b 39                    	cmpq	(%r9), %rdi
1416f213d: 0f 84 07 02 00 00           	je	0x1416f234a <.text+0x16f134a>
1416f2143: 4d 39 d5                    	cmpq	%r10, %r13
1416f2146: 75 d3                       	jne	0x1416f211b <.text+0x16f111b>
1416f2148: 4c 8b ad 28 03 00 00        	movq	0x328(%rbp), %r13
1416f214f: 4c 39 c2                    	cmpq	%r8, %rdx
1416f2152: 74 25                       	je	0x1416f2179 <.text+0x16f1179>
1416f2154: 44 8b 0a                    	movl	(%rdx), %r9d
1416f2157: 48 83 c2 04                 	addq	$0x4, %rdx
1416f215b: 4c 8b a1 00 01 00 00        	movq	0x100(%rcx), %r12
1416f2162: 4f 8d 0c 89                 	leaq	(%r9,%r9,4), %r9
1416f2166: 49 c1 e1 05                 	shlq	$0x5, %r9
1416f216a: 4f 8b 54 0c 58              	movq	0x58(%r12,%r9), %r10
1416f216f: 4d 85 d2                    	testq	%r10, %r10
1416f2172: 74 db                       	je	0x1416f214f <.text+0x16f114f>
1416f2174: e9 fe fe ff ff              	jmp	0x1416f2077 <.text+0x16f1077>
1416f2179: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
1416f2180: c6 00 00                    	movb	$0x0, (%rax)
1416f2183: 31 c0                       	xorl	%eax, %eax
1416f2185: 49 3b 45 00                 	cmpq	(%r13), %rax
1416f2189: 0f 91 c0                    	setno	%al
1416f218c: 84 85 f8 02 00 00           	testb	%al, 0x2f8(%rbp)
1416f2192: 0f 84 d5 f6 ff ff           	je	0x1416f186d <.text+0x16f086d>
1416f2198: 4c 8b 85 38 03 00 00        	movq	0x338(%rbp), %r8
1416f219f: 49 8b 00                    	movq	(%r8), %rax
1416f21a2: 49 8b 48 08                 	movq	0x8(%r8), %rcx
1416f21a6: 41 8b 50 10                 	movl	0x10(%r8), %edx
1416f21aa: 45 8b 40 14                 	movl	0x14(%r8), %r8d
1416f21ae: 4c 8d 89 a0 01 00 00        	leaq	0x1a0(%rcx), %r9
1416f21b5: 4c 8d 91 f8 00 00 00        	leaq	0xf8(%rcx), %r10
1416f21bc: 4c 8b 98 f8 00 00 00        	movq	0xf8(%rax), %r11
1416f21c3: 48 8b b0 00 01 00 00        	movq	0x100(%rax), %rsi
1416f21ca: 49 8d 34 b3                 	leaq	(%r11,%rsi,4), %rsi
1416f21ce: 0f b6 98 08 01 00 00        	movzbl	0x108(%rax), %ebx
1416f21d5: 88 9d 18 02 00 00           	movb	%bl, 0x218(%rbp)
1416f21db: 4c 89 9d 78 01 00 00        	movq	%r11, 0x178(%rbp)
1416f21e2: 48 89 b5 80 01 00 00        	movq	%rsi, 0x180(%rbp)
1416f21e9: 48 c7 85 88 01 00 00 08 00 00 00    	movq	$0x8, 0x188(%rbp)
1416f21f4: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
1416f21ff: 48 c7 85 98 01 00 00 08 00 00 00    	movq	$0x8, 0x198(%rbp)
1416f220a: 0f 57 c0                    	xorps	%xmm0, %xmm0
1416f220d: 0f 11 85 a0 01 00 00        	movups	%xmm0, 0x1a0(%rbp)
1416f2214: 89 95 c0 01 00 00           	movl	%edx, 0x1c0(%rbp)
1416f221a: 44 89 85 c4 01 00 00        	movl	%r8d, 0x1c4(%rbp)
1416f2221: 48 c7 85 c8 01 00 00 00 00 00 00    	movq	$0x0, 0x1c8(%rbp)
1416f222c: 89 95 e0 01 00 00           	movl	%edx, 0x1e0(%rbp)
1416f2232: 44 89 85 e4 01 00 00        	movl	%r8d, 0x1e4(%rbp)
1416f2239: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
1416f2244: 89 95 00 02 00 00           	movl	%edx, 0x200(%rbp)
1416f224a: 44 89 85 04 02 00 00        	movl	%r8d, 0x204(%rbp)
1416f2251: 0f 11 85 08 02 00 00        	movups	%xmm0, 0x208(%rbp)
1416f2258: 48 89 8d 58 01 00 00        	movq	%rcx, 0x158(%rbp)
1416f225f: 4c 89 8d 60 01 00 00        	movq	%r9, 0x160(%rbp)
1416f2266: 4c 89 95 68 01 00 00        	movq	%r10, 0x168(%rbp)
1416f226d: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1416f2274: 4c 8d 05 ad bb 57 01        	leaq	0x157bbad(%rip), %r8    # 0x142c6de28
1416f227b: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416f227f: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1416f2286: e8 45 29 c1 ff              	callq	0x141304bd0 <.text+0x1303bd0>
1416f228b: 49 8b 45 00                 	movq	(%r13), %rax
1416f228f: 48 85 c0                    	testq	%rax, %rax
1416f2292: 74 17                       	je	0x1416f22ab <.text+0x16f12ab>
1416f2294: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
1416f2298: 48 c1 e0 03                 	shlq	$0x3, %rax
1416f229c: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
1416f22a0: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1416f22a6: e8 75 19 f2 fe              	callq	0x140613c20 <.text+0x612c20>
1416f22ab: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
1416f22af: 49 89 45 10                 	movq	%rax, 0x10(%r13)
1416f22b3: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
1416f22b7: 41 0f 11 45 00              	movups	%xmm0, (%r13)
1416f22bc: e9 ac f5 ff ff              	jmp	0x1416f186d <.text+0x16f086d>
1416f22c1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416f22ca: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1416f22d0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416f22d6: 48 89 d9                    	movq	%rbx, %rcx
1416f22d9: 4c 89 f2                    	movq	%r14, %rdx
1416f22dc: e8 7f 77 1b 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1416f22e1: e9 20 f5 ff ff              	jmp	0x1416f1806 <.text+0x16f0806>
1416f22e6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1416f22ef: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1416f22f5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1416f22fb: 48 89 d9                    	movq	%rbx, %rcx
1416f22fe: 4c 89 f2                    	movq	%r14, %rdx
1416f2301: e8 5a 77 1b 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1416f2306: e9 48 f5 ff ff              	jmp	0x1416f1853 <.text+0x16f0853>
1416f230b: 4d 01 de                    	addq	%r11, %r14
1416f230e: 4a 8d 04 6d 00 00 00 00     	leaq	(,%r13,2), %rax
1416f2316: 4c 01 e8                    	addq	%r13, %rax
1416f2319: 48 c1 e0 04                 	shlq	$0x4, %rax
1416f231d: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
1416f2324: 48 01 c2                    	addq	%rax, %rdx
1416f2327: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
1416f232e: 4a 8d 04 a8                 	leaq	(%rax,%r13,4), %rax
1416f2332: 48 8b 8d 48 03 00 00        	movq	0x348(%rbp), %rcx
1416f2339: 4a 8d 34 a9                 	leaq	(%rcx,%r13,4), %rsi
1416f233d: 4a 8d 3c af                 	leaq	(%rdi,%r13,4), %rdi
1416f2341: 49 c1 e5 04                 	shlq	$0x4, %r13
1416f2345: 4d 01 ef                    	addq	%r13, %r15
1416f2348: eb 3d                       	jmp	0x1416f2387 <.text+0x16f1387>
1416f234a: 49 01 f6                    	addq	%rsi, %r14
1416f234d: 4b 8d 04 5b                 	leaq	(%r11,%r11,2), %rax
1416f2351: 48 c1 e0 04                 	shlq	$0x4, %rax
1416f2355: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
1416f235c: 48 01 c2                    	addq	%rax, %rdx
1416f235f: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
1416f2366: 4a 8d 04 98                 	leaq	(%rax,%r11,4), %rax
1416f236a: 48 8b 8d 48 03 00 00        	movq	0x348(%rbp), %rcx
1416f2371: 4a 8d 34 99                 	leaq	(%rcx,%r11,4), %rsi
1416f2375: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
1416f237c: 4a 8d 3c 99                 	leaq	(%rcx,%r11,4), %rdi
1416f2380: 49 c1 e3 04                 	shlq	$0x4, %r11
1416f2384: 4d 01 df                    	addq	%r11, %r15
1416f2387: 80 fb 02                    	cmpb	$0x2, %bl
1416f238a: 4c 8b ad 28 03 00 00        	movq	0x328(%rbp), %r13
1416f2391: 75 33                       	jne	0x1416f23c6 <.text+0x16f13c6>
1416f2393: 0f 57 c0                    	xorps	%xmm0, %xmm0
1416f2396: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
1416f239d: f6 01 01                    	testb	$0x1, (%rcx)
1416f23a0: 0f 57 c9                    	xorps	%xmm1, %xmm1
1416f23a3: 74 0a                       	je	0x1416f23af <.text+0x16f13af>
1416f23a5: f3 0f 10 49 04              	movss	0x4(%rcx), %xmm1
1416f23aa: f3 0f 10 41 0c              	movss	0xc(%rcx), %xmm0
1416f23af: f3 0f 11 4a 10              	movss	%xmm1, 0x10(%rdx)
1416f23b4: 8b 8d 44 03 00 00           	movl	0x344(%rbp), %ecx
1416f23ba: 89 08                       	movl	%ecx, (%rax)
1416f23bc: f3 0f 11 42 18              	movss	%xmm0, 0x18(%rdx)
1416f23c1: e9 82 00 00 00              	jmp	0x1416f2448 <.text+0x16f1448>
1416f23c6: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
1416f23cd: f6 00 01                    	testb	$0x1, (%rax)
1416f23d0: 74 1c                       	je	0x1416f23ee <.text+0x16f13ee>
1416f23d2: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
1416f23d9: 8b 41 08                    	movl	0x8(%rcx), %eax
1416f23dc: 89 85 60 01 00 00           	movl	%eax, 0x160(%rbp)
1416f23e2: 48 8b 01                    	movq	(%rcx), %rax
1416f23e5: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1416f23ec: eb 15                       	jmp	0x1416f2403 <.text+0x16f1403>
1416f23ee: c7 85 60 01 00 00 00 00 00 00       	movl	$0x0, 0x160(%rbp)
1416f23f8: 48 c7 85 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%rbp)
1416f2403: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1416f2407: 48 8d 95 58 01 00 00        	leaq	0x158(%rbp), %rdx
1416f240e: e8 1d dc 59 ff              	callq	0x140c90030 <.text+0xc8f030>
1416f2413: 8b 8d 44 03 00 00           	movl	0x344(%rbp), %ecx
1416f2419: 89 0f                       	movl	%ecx, (%rdi)
1416f241b: 41 c7 07 01 00 00 00        	movl	$0x1, (%r15)
1416f2422: 48 8b 45 b0                 	movq	-0x50(%rbp), %rax
1416f2426: 49 89 47 04                 	movq	%rax, 0x4(%r15)
1416f242a: 8b 45 b8                    	movl	-0x48(%rbp), %eax
1416f242d: 41 89 47 0c                 	movl	%eax, 0xc(%r15)
1416f2431: 89 0e                       	movl	%ecx, (%rsi)
1416f2433: 66 41 c7 46 30 01 01        	movw	$0x101, 0x30(%r14)      # imm = 0x101
1416f243a: f2 0f 10 05 de a6 3c 01     	movsd	0x13ca6de(%rip), %xmm0  # 0x142abcb20
1416f2442: f2 41 0f 11 46 34           	movsd	%xmm0, 0x34(%r14)
1416f2448: 41 c6 45 48 00              	movb	$0x0, 0x48(%r13)
1416f244d: 31 c0                       	xorl	%eax, %eax
1416f244f: 49 3b 45 00                 	cmpq	(%r13), %rax
1416f2453: 0f 80 14 f4 ff ff           	jo	0x1416f186d <.text+0x16f086d>
1416f2459: e9 3a fd ff ff              	jmp	0x1416f2198 <.text+0x16f1198>
1416f245e: b9 08 00 00 00              	movl	$0x8, %ecx
1416f2463: ba 48 00 00 00              	movl	$0x48, %edx
1416f2468: e8 93 6b 1e 01              	callq	0x1428d9000 <.text+0x28d8000>
1416f246d: 0f 0b                       	ud2
1416f246f: 90                          	nop
1416f2470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416f2475: 55                          	pushq	%rbp
1416f2476: 41 57                       	pushq	%r15
1416f2478: 41 56                       	pushq	%r14
1416f247a: 41 55                       	pushq	%r13
1416f247c: 41 54                       	pushq	%r12
1416f247e: 56                          	pushq	%rsi
1416f247f: 57                          	pushq	%rdi
1416f2480: 53                          	pushq	%rbx
1416f2481: 48 83 ec 68                 	subq	$0x68, %rsp
1416f2485: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416f248c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
1416f2492: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
1416f2498: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
1416f249d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
1416f24a2: 48 83 bd 58 03 00 00 00     	cmpq	$0x0, 0x358(%rbp)
1416f24aa: 74 27                       	je	0x1416f24d3 <.text+0x16f14d3>
1416f24ac: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
1416f24b1: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
1416f24b6: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
1416f24bc: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
1416f24c2: 48 83 c4 68                 	addq	$0x68, %rsp
1416f24c6: 5b                          	popq	%rbx
1416f24c7: 5f                          	popq	%rdi
1416f24c8: 5e                          	popq	%rsi
1416f24c9: 41 5c                       	popq	%r12
1416f24cb: 41 5d                       	popq	%r13
1416f24cd: 41 5e                       	popq	%r14
1416f24cf: 41 5f                       	popq	%r15
1416f24d1: 5d                          	popq	%rbp
1416f24d2: c3                          	retq
1416f24d3: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
1416f24da: e8 a1 15 96 fe              	callq	0x140053a80 <.text+0x52a80>
1416f24df: eb cb                       	jmp	0x1416f24ac <.text+0x16f14ac>
