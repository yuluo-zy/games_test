
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b15f80: 41 57                       	pushq	%r15
140b15f82: 41 56                       	pushq	%r14
140b15f84: 41 55                       	pushq	%r13
140b15f86: 41 54                       	pushq	%r12
140b15f88: 56                          	pushq	%rsi
140b15f89: 57                          	pushq	%rdi
140b15f8a: 55                          	pushq	%rbp
140b15f8b: 53                          	pushq	%rbx
140b15f8c: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
140b15f93: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
140b15f99: 44 0f 29 4c 24 60           	movaps	%xmm9, 0x60(%rsp)
140b15f9f: 44 0f 29 44 24 50           	movaps	%xmm8, 0x50(%rsp)
140b15fa5: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
140b15faa: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
140b15faf: 48 8b 02                    	movq	(%rdx), %rax
140b15fb2: 48 8b 28                    	movq	(%rax), %rbp
140b15fb5: 4c 8b 60 08                 	movq	0x8(%rax), %r12
140b15fb9: 4d 8b 38                    	movq	(%r8), %r15
140b15fbc: 49 8b 5f 28                 	movq	0x28(%r15), %rbx
140b15fc0: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140b15fc5: 48 83 7d 18 00              	cmpq	$0x0, 0x18(%rbp)
140b15fca: 0f 84 1d 01 00 00           	je	0x140b160ed <.text+0xb150ed>
140b15fd0: 48 89 ce                    	movq	%rcx, %rsi
140b15fd3: 48 8d 7d 20                 	leaq	0x20(%rbp), %rdi
140b15fd7: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
140b15fdc: 48 89 f9                    	movq	%rdi, %rcx
140b15fdf: e8 8c ad a9 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b15fe4: 48 8b 4d 00                 	movq	(%rbp), %rcx
140b15fe8: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
140b15fec: 49 89 d0                    	movq	%rdx, %r8
140b15fef: 49 21 c0                    	andq	%rax, %r8
140b15ff2: 48 c1 e8 39                 	shrq	$0x39, %rax
140b15ff6: 66 0f 6e c0                 	movd	%eax, %xmm0
140b15ffa: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b15ffe: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b16003: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b16008: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
140b1600f: 45 31 c9                    	xorl	%r9d, %r9d
140b16012: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b16016: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140b1601c: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140b16020: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140b16024: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140b16029: 45 85 d2                    	testl	%r10d, %r10d
140b1602c: 74 32                       	je	0x140b16060 <.text+0xb15060>
140b1602e: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140b16033: 4d 01 c3                    	addq	%r8, %r11
140b16036: 49 21 d3                    	andq	%rdx, %r11
140b16039: 49 f7 db                    	negq	%r11
140b1603c: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
140b16043: 4a 39 1c 18                 	cmpq	%rbx, (%rax,%r11)
140b16047: 74 35                       	je	0x140b1607e <.text+0xb1507e>
140b16049: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140b1604d: 66 45 21 d3                 	andw	%r10w, %r11w
140b16051: 45 89 da                    	movl	%r11d, %r10d
140b16054: 75 d8                       	jne	0x140b1602e <.text+0xb1502e>
140b16056: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b16060: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140b16064: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140b16069: 45 85 d2                    	testl	%r10d, %r10d
140b1606c: 75 7f                       	jne	0x140b160ed <.text+0xb150ed>
140b1606e: 4d 01 c8                    	addq	%r9, %r8
140b16071: 49 83 c0 10                 	addq	$0x10, %r8
140b16075: 49 83 c1 10                 	addq	$0x10, %r9
140b16079: 49 21 d0                    	andq	%rdx, %r8
140b1607c: eb 98                       	jmp	0x140b16016 <.text+0xb15016>
140b1607e: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
140b16087: 0f 82 d0 03 00 00           	jb	0x140b1645d <.text+0xb1545d>
140b1608d: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
140b16097: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140b1609b: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140b1609e: 0f 86 b9 03 00 00           	jbe	0x140b1645d <.text+0xb1545d>
140b160a4: 4c 01 d9                    	addq	%r11, %rcx
140b160a7: f3 0f 10 41 e0              	movss	-0x20(%rcx), %xmm0
140b160ac: 0f 2e 05 1d f8 e0 01        	ucomiss	0x1e0f81d(%rip), %xmm0  # 0x1429258d0
140b160b3: 76 44                       	jbe	0x140b160f9 <.text+0xb150f9>
140b160b5: 31 c0                       	xorl	%eax, %eax
140b160b7: 48 89 06                    	movq	%rax, (%rsi)
140b160ba: 48 89 f0                    	movq	%rsi, %rax
140b160bd: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140b160c2: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140b160c7: 44 0f 28 44 24 50           	movaps	0x50(%rsp), %xmm8
140b160cd: 44 0f 28 4c 24 60           	movaps	0x60(%rsp), %xmm9
140b160d3: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
140b160d9: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
140b160e0: 5b                          	popq	%rbx
140b160e1: 5d                          	popq	%rbp
140b160e2: 5f                          	popq	%rdi
140b160e3: 5e                          	popq	%rsi
140b160e4: 41 5c                       	popq	%r12
140b160e6: 41 5d                       	popq	%r13
140b160e8: 41 5e                       	popq	%r14
140b160ea: 41 5f                       	popq	%r15
140b160ec: c3                          	retq
140b160ed: 48 8d 0d 5c 2d fd 01        	leaq	0x1fd2d5c(%rip), %rcx   # 0x142ae8e50
140b160f4: e8 17 32 dc 01              	callq	0x1428d9310 <.text+0x28d8310>
140b160f9: 49 8b 5f 28                 	movq	0x28(%r15), %rbx
140b160fd: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140b16102: 48 83 7d 18 00              	cmpq	$0x0, 0x18(%rbp)
140b16107: 0f 84 5d 02 00 00           	je	0x140b1636a <.text+0xb1536a>
140b1610d: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
140b16112: 48 89 f9                    	movq	%rdi, %rcx
140b16115: e8 56 ac a9 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b1611a: 4c 8b 6d 00                 	movq	(%rbp), %r13
140b1611e: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140b16122: 48 89 ca                    	movq	%rcx, %rdx
140b16125: 48 21 c2                    	andq	%rax, %rdx
140b16128: 48 c1 e8 39                 	shrq	$0x39, %rax
140b1612c: 66 0f 6e c0                 	movd	%eax, %xmm0
140b16130: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b16134: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b16139: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b1613e: 49 8d 85 58 fd ff ff        	leaq	-0x2a8(%r13), %rax
140b16145: 45 31 c0                    	xorl	%r8d, %r8d
140b16148: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b1614c: f3 41 0f 6f 54 15 00        	movdqu	(%r13,%rdx), %xmm2
140b16153: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140b16157: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140b1615b: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140b16160: 45 85 c9                    	testl	%r9d, %r9d
140b16163: 74 28                       	je	0x140b1618d <.text+0xb1518d>
140b16165: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140b1616a: 49 01 d2                    	addq	%rdx, %r10
140b1616d: 49 21 ca                    	andq	%rcx, %r10
140b16170: 49 f7 da                    	negq	%r10
140b16173: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
140b1617a: 4a 39 1c 10                 	cmpq	%rbx, (%rax,%r10)
140b1617e: 74 2f                       	je	0x140b161af <.text+0xb151af>
140b16180: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140b16184: 66 45 21 ca                 	andw	%r9w, %r10w
140b16188: 45 89 d1                    	movl	%r10d, %r9d
140b1618b: 75 d8                       	jne	0x140b16165 <.text+0xb15165>
140b1618d: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140b16191: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140b16196: 45 85 c9                    	testl	%r9d, %r9d
140b16199: 0f 85 cb 01 00 00           	jne	0x140b1636a <.text+0xb1536a>
140b1619f: 4c 01 c2                    	addq	%r8, %rdx
140b161a2: 48 83 c2 10                 	addq	$0x10, %rdx
140b161a6: 49 83 c0 10                 	addq	$0x10, %r8
140b161aa: 48 21 ca                    	andq	%rcx, %rdx
140b161ad: eb 9d                       	jmp	0x140b1614c <.text+0xb1514c>
140b161af: 4b 83 bc 15 70 fd ff ff 02  	cmpq	$0x2, -0x290(%r13,%r10)
140b161b8: 0f 82 b7 02 00 00           	jb	0x140b16475 <.text+0xb15475>
140b161be: f3 43 0f 10 84 15 90 fd ff ff       	movss	-0x270(%r13,%r10), %xmm0
140b161c8: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140b161cc: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140b161cf: 0f 86 a0 02 00 00           	jbe	0x140b16475 <.text+0xb15475>
140b161d5: 4d 01 d5                    	addq	%r10, %r13
140b161d8: 49 8d 8d 98 fd ff ff        	leaq	-0x268(%r13), %rcx
140b161df: f3 41 0f 10 0c 24           	movss	(%r12), %xmm1
140b161e5: f3 41 0f 10 54 24 08        	movss	0x8(%r12), %xmm2
140b161ec: e8 0f 3c 95 00              	callq	0x141469e00 <.text+0x1468e00>
140b161f1: 89 c3                       	movl	%eax, %ebx
140b161f3: 49 8b 95 a8 fd ff ff        	movq	-0x258(%r13), %rdx
140b161fa: 48 39 da                    	cmpq	%rbx, %rdx
140b161fd: 0f 86 ba 02 00 00           	jbe	0x140b164bd <.text+0xb154bd>
140b16203: 48 8d 4b 01                 	leaq	0x1(%rbx), %rcx
140b16207: 48 39 d1                    	cmpq	%rdx, %rcx
140b1620a: 0f 83 bc 02 00 00           	jae	0x140b164cc <.text+0xb154cc>
140b16210: 0f 28 f0                    	movaps	%xmm0, %xmm6
140b16213: 49 8b 85 a0 fd ff ff        	movq	-0x260(%r13), %rax
140b1621a: 4c 8d 04 5b                 	leaq	(%rbx,%rbx,2), %r8
140b1621e: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140b16223: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
140b16227: f3 44 0f 10 05 d4 f6 e0 01  	movss	0x1e0f6d4(%rip), %xmm8  # 0x142925904
140b16230: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
140b16234: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140b16238: f3 42 0f 10 7c 80 08        	movss	0x8(%rax,%r8,4), %xmm7
140b1623f: f3 0f 59 f9                 	mulss	%xmm1, %xmm7
140b16243: f2 42 0f 10 1c 80           	movsd	(%rax,%r8,4), %xmm3
140b16249: f2 0f 10 14 90              	movsd	(%rax,%rdx,4), %xmm2
140b1624e: f3 0f 10 44 90 08           	movss	0x8(%rax,%rdx,4), %xmm0
140b16254: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
140b16258: 4d 8b 77 28                 	movq	0x28(%r15), %r14
140b1625c: 4c 89 74 24 20              	movq	%r14, 0x20(%rsp)
140b16261: 48 83 7d 18 00              	cmpq	$0x0, 0x18(%rbp)
140b16266: 0f 84 0a 01 00 00           	je	0x140b16376 <.text+0xb15376>
140b1626c: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
140b16270: 0f 59 cb                    	mulps	%xmm3, %xmm1
140b16273: f3 44 0f 12 ce              	movsldup	%xmm6, %xmm9            # xmm9 = xmm6[0,0,2,2]
140b16278: 44 0f 59 ca                 	mulps	%xmm2, %xmm9
140b1627c: 44 0f 58 c9                 	addps	%xmm1, %xmm9
140b16280: f3 0f 58 f8                 	addss	%xmm0, %xmm7
140b16284: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
140b16289: 48 89 f9                    	movq	%rdi, %rcx
140b1628c: e8 df aa a9 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b16291: 48 8b 7d 00                 	movq	(%rbp), %rdi
140b16295: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
140b16299: 49 89 d0                    	movq	%rdx, %r8
140b1629c: 49 21 c0                    	andq	%rax, %r8
140b1629f: 48 c1 e8 39                 	shrq	$0x39, %rax
140b162a3: 66 0f 6e c0                 	movd	%eax, %xmm0
140b162a7: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b162ab: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b162b0: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b162b5: 48 8d 87 58 fd ff ff        	leaq	-0x2a8(%rdi), %rax
140b162bc: 45 31 c9                    	xorl	%r9d, %r9d
140b162bf: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b162c3: f3 42 0f 6f 14 07           	movdqu	(%rdi,%r8), %xmm2
140b162c9: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140b162cd: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140b162d1: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
140b162d6: 45 85 d2                    	testl	%r10d, %r10d
140b162d9: 74 28                       	je	0x140b16303 <.text+0xb15303>
140b162db: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140b162e0: 4d 01 c3                    	addq	%r8, %r11
140b162e3: 49 21 d3                    	andq	%rdx, %r11
140b162e6: 49 f7 db                    	negq	%r11
140b162e9: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
140b162f0: 4e 39 34 18                 	cmpq	%r14, (%rax,%r11)
140b162f4: 74 2b                       	je	0x140b16321 <.text+0xb15321>
140b162f6: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140b162fa: 66 45 21 d3                 	andw	%r10w, %r11w
140b162fe: 45 89 da                    	movl	%r11d, %r10d
140b16301: 75 d8                       	jne	0x140b162db <.text+0xb152db>
140b16303: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140b16307: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140b1630c: 45 85 d2                    	testl	%r10d, %r10d
140b1630f: 75 65                       	jne	0x140b16376 <.text+0xb15376>
140b16311: 4d 01 c8                    	addq	%r9, %r8
140b16314: 49 83 c0 10                 	addq	$0x10, %r8
140b16318: 49 83 c1 10                 	addq	$0x10, %r9
140b1631c: 49 21 d0                    	andq	%rdx, %r8
140b1631f: eb a2                       	jmp	0x140b162c3 <.text+0xb152c3>
140b16321: 4a 83 bc 1f 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rdi,%r11)
140b1632a: 0f 82 5d 01 00 00           	jb	0x140b1648d <.text+0xb1548d>
140b16330: f3 42 0f 10 84 1f 90 fd ff ff       	movss	-0x270(%rdi,%r11), %xmm0
140b1633a: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140b1633e: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140b16341: 0f 86 46 01 00 00           	jbe	0x140b1648d <.text+0xb1548d>
140b16347: 4c 01 df                    	addq	%r11, %rdi
140b1634a: 8b 47 b8                    	movl	-0x48(%rdi), %eax
140b1634d: 83 f8 02                    	cmpl	$0x2, %eax
140b16350: ba 02 00 00 00              	movl	$0x2, %edx
140b16355: 0f 42 d0                    	cmovbl	%eax, %edx
140b16358: f6 c2 01                    	testb	$0x1, %dl
140b1635b: 48 8b 4c 24 28              	movq	0x28(%rsp), %rcx
140b16360: 74 20                       	je	0x140b16382 <.text+0xb15382>
140b16362: f2 44 0f 10 57 bc           	movsd	-0x44(%rdi), %xmm10
140b16368: eb 4c                       	jmp	0x140b163b6 <.text+0xb153b6>
140b1636a: 48 8d 0d 0f 2b fd 01        	leaq	0x1fd2b0f(%rip), %rcx   # 0x142ae8e80
140b16371: e8 9a 2f dc 01              	callq	0x1428d9310 <.text+0x28d8310>
140b16376: 48 8d 0d 33 2b fd 01        	leaq	0x1fd2b33(%rip), %rcx   # 0x142ae8eb0
140b1637d: e8 8e 2f dc 01              	callq	0x1428d9310 <.text+0x28d8310>
140b16382: f2 41 0f 10 07              	movsd	(%r15), %xmm0
140b16387: f2 41 0f 10 4f 08           	movsd	0x8(%r15), %xmm1
140b1638d: 0f 5c c8                    	subps	%xmm0, %xmm1
140b16390: 0f 28 c1                    	movaps	%xmm1, %xmm0
140b16393: 0f 59 c1                    	mulps	%xmm1, %xmm0
140b16396: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
140b1639a: f3 0f 58 d0                 	addss	%xmm0, %xmm2
140b1639e: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b163a1: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
140b163a5: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
140b163a9: f3 0f 5e d0                 	divss	%xmm0, %xmm2
140b163ad: f3 44 0f 12 d2              	movsldup	%xmm2, %xmm10           # xmm10 = xmm2[0,0,2,2]
140b163b2: 44 0f 59 d1                 	mulps	%xmm1, %xmm10
140b163b6: f3 41 0f 10 04 24           	movss	(%r12), %xmm0
140b163bc: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
140b163c0: 0f 28 d7                    	movaps	%xmm7, %xmm2
140b163c3: f3 41 0f 5c 54 24 08        	subss	0x8(%r12), %xmm2
140b163ca: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140b163ce: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
140b163d2: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
140b163d6: f3 0f 58 d1                 	addss	%xmm1, %xmm2
140b163da: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b163dd: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
140b163e1: 0f 2e 05 e8 f4 e0 01        	ucomiss	0x1e0f4e8(%rip), %xmm0  # 0x1429258d0
140b163e8: 0f 87 c7 fc ff ff           	ja	0x140b160b5 <.text+0xb150b5>
140b163ee: 49 8b 95 c0 fd ff ff        	movq	-0x240(%r13), %rdx
140b163f5: 48 39 da                    	cmpq	%rbx, %rdx
140b163f8: 0f 86 da 00 00 00           	jbe	0x140b164d8 <.text+0xb154d8>
140b163fe: 48 39 d1                    	cmpq	%rdx, %rcx
140b16401: 0f 83 e0 00 00 00           	jae	0x140b164e7 <.text+0xb154e7>
140b16407: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b1640a: 0f 2e f0                    	ucomiss	%xmm0, %xmm6
140b1640d: 0f 82 92 00 00 00           	jb	0x140b164a5 <.text+0xb154a5>
140b16413: 44 0f 2e c6                 	ucomiss	%xmm6, %xmm8
140b16417: 0f 82 88 00 00 00           	jb	0x140b164a5 <.text+0xb154a5>
140b1641d: 49 8b 85 b8 fd ff ff        	movq	-0x248(%r13), %rax
140b16424: f3 0f 10 04 98              	movss	(%rax,%rbx,4), %xmm0
140b16429: f3 0f 10 4c 98 04           	movss	0x4(%rax,%rbx,4), %xmm1
140b1642f: 0f 28 d6                    	movaps	%xmm6, %xmm2
140b16432: e8 99 a0 17 00              	callq	0x140c904d0 <.text+0xc8f4d0>
140b16437: 49 8b 47 28                 	movq	0x28(%r15), %rax
140b1643b: 44 0f 13 4e 1c              	movlps	%xmm9, 0x1c(%rsi)
140b16440: f3 0f 11 7e 24              	movss	%xmm7, 0x24(%rsi)
140b16445: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
140b16449: 44 0f 13 56 10              	movlps	%xmm10, 0x10(%rsi)
140b1644e: f3 0f 11 46 18              	movss	%xmm0, 0x18(%rsi)
140b16453: b8 01 00 00 00              	movl	$0x1, %eax
140b16458: e9 5a fc ff ff              	jmp	0x140b160b7 <.text+0xb150b7>
140b1645d: 48 8d 0d 2f 3d fc 01        	leaq	0x1fc3d2f(%rip), %rcx   # 0x142ada193
140b16464: 4c 8d 05 cd 29 fd 01        	leaq	0x1fd29cd(%rip), %r8    # 0x142ae8e38
140b1646b: ba 32 00 00 00              	movl	$0x32, %edx
140b16470: e8 bb 2f dc 01              	callq	0x1428d9430 <.text+0x28d8430>
140b16475: 48 8d 0d 17 3d fc 01        	leaq	0x1fc3d17(%rip), %rcx   # 0x142ada193
140b1647c: 4c 8d 05 e5 29 fd 01        	leaq	0x1fd29e5(%rip), %r8    # 0x142ae8e68
140b16483: ba 32 00 00 00              	movl	$0x32, %edx
140b16488: e8 a3 2f dc 01              	callq	0x1428d9430 <.text+0x28d8430>
140b1648d: 48 8d 0d ff 3c fc 01        	leaq	0x1fc3cff(%rip), %rcx   # 0x142ada193
140b16494: 4c 8d 05 fd 29 fd 01        	leaq	0x1fd29fd(%rip), %r8    # 0x142ae8e98
140b1649b: ba 32 00 00 00              	movl	$0x32, %edx
140b164a0: e8 8b 2f dc 01              	callq	0x1428d9430 <.text+0x28d8430>
140b164a5: 48 8d 0d 41 ef f8 01        	leaq	0x1f8ef41(%rip), %rcx   # 0x142aa53ed
140b164ac: 4c 8d 05 b5 ef f8 01        	leaq	0x1f8efb5(%rip), %r8    # 0x142aa5468
140b164b3: ba 2a 00 00 00              	movl	$0x2a, %edx
140b164b8: e8 73 2f dc 01              	callq	0x1428d9430 <.text+0x28d8430>
140b164bd: 4c 8d 05 94 d1 14 02        	leaq	0x214d194(%rip), %r8    # 0x142c63658
140b164c4: 48 89 d9                    	movq	%rbx, %rcx
140b164c7: e8 4c 30 dc 01              	callq	0x1428d9518 <.text+0x28d8518>
140b164cc: 4c 8d 05 9d d1 14 02        	leaq	0x214d19d(%rip), %r8    # 0x142c63670
140b164d3: e8 40 30 dc 01              	callq	0x1428d9518 <.text+0x28d8518>
140b164d8: 4c 8d 05 f9 d0 14 02        	leaq	0x214d0f9(%rip), %r8    # 0x142c635d8
140b164df: 48 89 d9                    	movq	%rbx, %rcx
140b164e2: e8 31 30 dc 01              	callq	0x1428d9518 <.text+0x28d8518>
140b164e7: 4c 8d 05 02 d1 14 02        	leaq	0x214d102(%rip), %r8    # 0x142c635f0
140b164ee: e8 25 30 dc 01              	callq	0x1428d9518 <.text+0x28d8518>
140b164f3: cc                          	int3
140b164f4: cc                          	int3
140b164f5: cc                          	int3
140b164f6: cc                          	int3
140b164f7: cc                          	int3
140b164f8: cc                          	int3
140b164f9: cc                          	int3
140b164fa: cc                          	int3
140b164fb: cc                          	int3
140b164fc: cc                          	int3
140b164fd: cc                          	int3
140b164fe: cc                          	int3
140b164ff: cc                          	int3
