
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140957390: 55                          	pushq	%rbp
140957391: 41 57                       	pushq	%r15
140957393: 41 56                       	pushq	%r14
140957395: 41 55                       	pushq	%r13
140957397: 41 54                       	pushq	%r12
140957399: 56                          	pushq	%rsi
14095739a: 57                          	pushq	%rdi
14095739b: 53                          	pushq	%rbx
14095739c: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1409573a3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409573ab: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
1409573b6: 4c 89 cb                    	movq	%r9, %rbx
1409573b9: 4c 89 c7                    	movq	%r8, %rdi
1409573bc: 49 89 d4                    	movq	%rdx, %r12
1409573bf: 48 89 ce                    	movq	%rcx, %rsi
1409573c2: 0f b6 05 40 d8 cd 02        	movzbl	0x2cdd840(%rip), %eax   # 0x143634c09
1409573c9: b9 08 00 00 00              	movl	$0x8, %ecx
1409573ce: ba 08 00 00 00              	movl	$0x8, %edx
1409573d3: e8 38 c8 cb ff              	callq	0x140613c10 <.text+0x612c10>
1409573d8: 48 85 c0                    	testq	%rax, %rax
1409573db: 0f 84 2b 02 00 00           	je	0x14095760c <.text+0x95660c>
1409573e1: 48 89 7d 70                 	movq	%rdi, 0x70(%rbp)
1409573e5: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1409573e9: 4d 8b 74 24 10              	movq	0x10(%r12), %r14
1409573ee: 45 31 ed                    	xorl	%r13d, %r13d
1409573f1: b9 0c 00 00 00              	movl	$0xc, %ecx
1409573f6: 4c 89 f0                    	movq	%r14, %rax
1409573f9: 48 f7 e1                    	mulq	%rcx
1409573fc: 49 89 c7                    	movq	%rax, %r15
1409573ff: 0f 80 f3 01 00 00           	jo	0x1409575f8 <.text+0x9565f8>
140957405: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14095740f: 49 39 c7                    	cmpq	%rax, %r15
140957412: 0f 87 e0 01 00 00           	ja	0x1409575f8 <.text+0x9565f8>
140957418: 4d 8b 64 24 08              	movq	0x8(%r12), %r12
14095741d: 4d 85 ff                    	testq	%r15, %r15
140957420: 74 2b                       	je	0x14095744d <.text+0x95644d>
140957422: 0f b6 05 e0 d7 cd 02        	movzbl	0x2cdd7e0(%rip), %eax   # 0x143634c09
140957429: 41 bd 04 00 00 00           	movl	$0x4, %r13d
14095742f: ba 04 00 00 00              	movl	$0x4, %edx
140957434: 4c 89 f9                    	movq	%r15, %rcx
140957437: e8 d4 c7 cb ff              	callq	0x140613c10 <.text+0x612c10>
14095743c: 48 85 c0                    	testq	%rax, %rax
14095743f: 0f 84 b3 01 00 00           	je	0x1409575f8 <.text+0x9565f8>
140957445: 48 89 c7                    	movq	%rax, %rdi
140957448: 4d 89 f5                    	movq	%r14, %r13
14095744b: eb 08                       	jmp	0x140957455 <.text+0x956455>
14095744d: bf 04 00 00 00              	movl	$0x4, %edi
140957452: 45 31 ed                    	xorl	%r13d, %r13d
140957455: 48 89 f9                    	movq	%rdi, %rcx
140957458: 4c 89 e2                    	movq	%r12, %rdx
14095745b: 4d 89 f8                    	movq	%r15, %r8
14095745e: e8 9d c7 fc 01              	callq	0x142923c00 <.text+0x2922c00>
140957463: 4c 89 6d 48                 	movq	%r13, 0x48(%rbp)
140957467: 48 89 7d 50                 	movq	%rdi, 0x50(%rbp)
14095746b: 4c 89 75 58                 	movq	%r14, 0x58(%rbp)
14095746f: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
140957479: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14095747d: 0f b6 05 85 d7 cd 02        	movzbl	0x2cdd785(%rip), %eax   # 0x143634c09
140957484: b9 13 00 00 00              	movl	$0x13, %ecx
140957489: ba 01 00 00 00              	movl	$0x1, %edx
14095748e: e8 7d c7 cb ff              	callq	0x140613c10 <.text+0x612c10>
140957493: 48 85 c0                    	testq	%rax, %rax
140957496: 0f 84 7f 01 00 00           	je	0x14095761b <.text+0x95661b>
14095749c: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1409574a3: 8b 15 f2 1f 14 02           	movl	0x2141ff2(%rip), %edx   # 0x142a9949b
1409574a9: 89 50 0f                    	movl	%edx, 0xf(%rax)
1409574ac: 0f 10 05 d9 1f 14 02        	movups	0x2141fd9(%rip), %xmm0  # 0x142a9948c
1409574b3: 0f 11 00                    	movups	%xmm0, (%rax)
1409574b6: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409574ba: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409574be: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409574c2: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1409574c6: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1409574ca: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1409574ce: 48 c7 45 2c 00 00 00 00     	movq	$0x0, 0x2c(%rbp)
1409574d6: 48 c7 45 31 00 00 00 00     	movq	$0x0, 0x31(%rbp)
1409574de: 0f 28 05 db 6b fe 01        	movaps	0x1fe6bdb(%rip), %xmm0  # 0x14293e0c0
1409574e5: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1409574e9: 66 c7 45 f0 01 01           	movw	$0x101, -0x10(%rbp)     # imm = 0x101
1409574ef: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
1409574f9: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
1409574fd: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
140957504: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
14095750b: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
140957513: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140957517: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
14095751f: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
140957526: 66 c7 45 39 00 00           	movw	$0x0, 0x39(%rbp)
14095752c: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140957531: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
140957535: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140957539: 4c 8b 45 70                 	movq	0x70(%rbp), %r8
14095753d: 49 89 d9                    	movq	%rbx, %r9
140957540: e8 0b 68 f4 ff              	callq	0x14089dd50 <.text+0x89cd50>
140957545: 48 8b 7d 48                 	movq	0x48(%rbp), %rdi
140957549: 48 85 ff                    	testq	%rdi, %rdi
14095754c: 75 04                       	jne	0x140957552 <.text+0x956552>
14095754e: 48 8b 7d 50                 	movq	0x50(%rbp), %rdi
140957552: 4c 8b 75 60                 	movq	0x60(%rbp), %r14
140957556: 48 8b 07                    	movq	(%rdi), %rax
140957559: 48 8b 5f 10                 	movq	0x10(%rdi), %rbx
14095755d: 48 29 d8                    	subq	%rbx, %rax
140957560: 48 83 f8 27                 	cmpq	$0x27, %rax
140957564: 76 6d                       	jbe	0x1409575d3 <.text+0x9565d3>
140957566: 48 8b 47 08                 	movq	0x8(%rdi), %rax
14095756a: 48 8d 0d 7f 53 98 00        	leaq	0x98537f(%rip), %rcx    # 0x1412dc8f0 <.text+0x12db8f0>
140957571: 48 89 0c 18                 	movq	%rcx, (%rax,%rbx)
140957575: 4c 89 74 18 08              	movq	%r14, 0x8(%rax,%rbx)
14095757a: 48 b9 01 00 00 00 00 00 40 40       	movabsq	$0x4040000000000001, %rcx # imm = 0x4040000000000001
140957584: 48 89 4c 18 10              	movq	%rcx, 0x10(%rax,%rbx)
140957589: c6 44 18 18 00              	movb	$0x0, 0x18(%rax,%rbx)
14095758e: 48 8d 0d 9b d7 c0 00        	leaq	0xc0d79b(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140957595: 48 89 4c 18 20              	movq	%rcx, 0x20(%rax,%rbx)
14095759a: 48 83 c3 28                 	addq	$0x28, %rbx
14095759e: 48 89 5f 10                 	movq	%rbx, 0x10(%rdi)
1409575a2: 48 8b 45 78                 	movq	0x78(%rbp), %rax
1409575a6: 4c 89 30                    	movq	%r14, (%rax)
1409575a9: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1409575b0: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1409575b4: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1409575bc: 48 89 f0                    	movq	%rsi, %rax
1409575bf: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1409575c6: 5b                          	popq	%rbx
1409575c7: 5f                          	popq	%rdi
1409575c8: 5e                          	popq	%rsi
1409575c9: 41 5c                       	popq	%r12
1409575cb: 41 5d                       	popq	%r13
1409575cd: 41 5e                       	popq	%r14
1409575cf: 41 5f                       	popq	%r15
1409575d1: 5d                          	popq	%rbp
1409575d2: c3                          	retq
1409575d3: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409575dc: 41 b8 28 00 00 00           	movl	$0x28, %r8d
1409575e2: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409575e8: 48 89 f9                    	movq	%rdi, %rcx
1409575eb: 48 89 da                    	movq	%rbx, %rdx
1409575ee: e8 6d 24 f5 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1409575f3: e9 6e ff ff ff              	jmp	0x140957566 <.text+0x956566>
1409575f8: 4c 8d 05 79 15 2e 02        	leaq	0x22e1579(%rip), %r8    # 0x142c38b78
1409575ff: 4c 89 e9                    	movq	%r13, %rcx
140957602: 4c 89 fa                    	movq	%r15, %rdx
140957605: e8 d9 19 f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14095760a: eb 25                       	jmp	0x140957631 <.text+0x956631>
14095760c: b9 08 00 00 00              	movl	$0x8, %ecx
140957611: ba 08 00 00 00              	movl	$0x8, %edx
140957616: e8 e5 19 f8 01              	callq	0x1428d9000 <.text+0x28d8000>
14095761b: 4c 8d 05 fe 18 14 02        	leaq	0x21418fe(%rip), %r8    # 0x142a98f20
140957622: b9 01 00 00 00              	movl	$0x1, %ecx
140957627: ba 13 00 00 00              	movl	$0x13, %edx
14095762c: e8 b2 19 f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140957631: 0f 0b                       	ud2
140957633: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140957640: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140957645: 55                          	pushq	%rbp
140957646: 41 57                       	pushq	%r15
140957648: 41 56                       	pushq	%r14
14095764a: 41 55                       	pushq	%r13
14095764c: 41 54                       	pushq	%r12
14095764e: 56                          	pushq	%rsi
14095764f: 57                          	pushq	%rdi
140957650: 53                          	pushq	%rbx
140957651: 48 83 ec 28                 	subq	$0x28, %rsp
140957655: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14095765c: ba 08 00 00 00              	movl	$0x8, %edx
140957661: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140957667: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14095766b: e8 b0 c5 cb ff              	callq	0x140613c20 <.text+0x612c20>
140957670: 90                          	nop
140957671: 48 83 c4 28                 	addq	$0x28, %rsp
140957675: 5b                          	popq	%rbx
140957676: 5f                          	popq	%rdi
140957677: 5e                          	popq	%rsi
140957678: 41 5c                       	popq	%r12
14095767a: 41 5d                       	popq	%r13
14095767c: 41 5e                       	popq	%r14
14095767e: 41 5f                       	popq	%r15
140957680: 5d                          	popq	%rbp
140957681: c3                          	retq
140957682: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140957690: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140957695: 55                          	pushq	%rbp
140957696: 41 57                       	pushq	%r15
140957698: 41 56                       	pushq	%r14
14095769a: 41 55                       	pushq	%r13
14095769c: 41 54                       	pushq	%r12
14095769e: 56                          	pushq	%rsi
14095769f: 57                          	pushq	%rdi
1409576a0: 53                          	pushq	%rbx
1409576a1: 48 83 ec 28                 	subq	$0x28, %rsp
1409576a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409576ac: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409576b0: e8 eb 36 f4 ff              	callq	0x14089ada0 <.text+0x899da0>
1409576b5: 90                          	nop
1409576b6: 48 83 c4 28                 	addq	$0x28, %rsp
1409576ba: 5b                          	popq	%rbx
1409576bb: 5f                          	popq	%rdi
1409576bc: 5e                          	popq	%rsi
1409576bd: 41 5c                       	popq	%r12
1409576bf: 41 5d                       	popq	%r13
1409576c1: 41 5e                       	popq	%r14
1409576c3: 41 5f                       	popq	%r15
1409576c5: 5d                          	popq	%rbp
1409576c6: c3                          	retq
1409576c7: cc                          	int3
1409576c8: cc                          	int3
1409576c9: cc                          	int3
1409576ca: cc                          	int3
1409576cb: cc                          	int3
1409576cc: cc                          	int3
1409576cd: cc                          	int3
1409576ce: cc                          	int3
1409576cf: cc                          	int3
1409576d0: 55                          	pushq	%rbp
1409576d1: 41 57                       	pushq	%r15
1409576d3: 41 56                       	pushq	%r14
1409576d5: 56                          	pushq	%rsi
1409576d6: 57                          	pushq	%rdi
1409576d7: 53                          	pushq	%rbx
1409576d8: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
1409576df: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409576e7: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
1409576f2: 4c 89 cb                    	movq	%r9, %rbx
1409576f5: 4d 89 c6                    	movq	%r8, %r14
1409576f8: 48 89 d7                    	movq	%rdx, %rdi
1409576fb: 48 89 ce                    	movq	%rcx, %rsi
1409576fe: 0f b6 05 04 d5 cd 02        	movzbl	0x2cdd504(%rip), %eax   # 0x143634c09
140957705: b9 08 00 00 00              	movl	$0x8, %ecx
14095770a: ba 08 00 00 00              	movl	$0x8, %edx
14095770f: e8 fc c4 cb ff              	callq	0x140613c10 <.text+0x612c10>
140957714: 48 85 c0                    	testq	%rax, %rax
140957717: 0f 84 ce 01 00 00           	je	0x1409578eb <.text+0x9568eb>
14095771d: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140957721: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
14095772b: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14095772f: 0f b6 05 d3 d4 cd 02        	movzbl	0x2cdd4d3(%rip), %eax   # 0x143634c09
140957736: b9 13 00 00 00              	movl	$0x13, %ecx
14095773b: ba 01 00 00 00              	movl	$0x1, %edx
140957740: e8 cb c4 cb ff              	callq	0x140613c10 <.text+0x612c10>
140957745: 48 85 c0                    	testq	%rax, %rax
140957748: 0f 84 ac 01 00 00           	je	0x1409578fa <.text+0x9568fa>
14095774e: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140957755: 8b 15 40 1d 14 02           	movl	0x2141d40(%rip), %edx   # 0x142a9949b
14095775b: 89 50 0f                    	movl	%edx, 0xf(%rax)
14095775e: 0f 10 05 27 1d 14 02        	movups	0x2141d27(%rip), %xmm0  # 0x142a9948c
140957765: 0f 11 00                    	movups	%xmm0, (%rax)
140957768: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
14095776c: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
140957770: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
140957774: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140957778: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
14095777c: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
140957780: 48 c7 45 2c 00 00 00 00     	movq	$0x0, 0x2c(%rbp)
140957788: 48 c7 45 31 00 00 00 00     	movq	$0x0, 0x31(%rbp)
140957790: f3 0f 10 47 08              	movss	0x8(%rdi), %xmm0
140957795: 0f 28 0d 24 69 fe 01        	movaps	0x1fe6924(%rip), %xmm1  # 0x14293e0c0
14095779c: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
1409577a0: 66 c7 45 f0 01 01           	movw	$0x101, -0x10(%rbp)     # imm = 0x101
1409577a6: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
1409577b0: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
1409577b4: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
1409577bb: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
1409577c2: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
1409577ca: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
1409577ce: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
1409577d6: f3 0f 11 45 28              	movss	%xmm0, 0x28(%rbp)
1409577db: 66 c7 45 39 00 00           	movw	$0x0, 0x39(%rbp)
1409577e1: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
1409577e6: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
1409577ea: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1409577ee: 4d 89 f0                    	movq	%r14, %r8
1409577f1: 49 89 d9                    	movq	%rbx, %r9
1409577f4: e8 57 65 f4 ff              	callq	0x14089dd50 <.text+0x89cd50>
1409577f9: 48 8b 5d 50                 	movq	0x50(%rbp), %rbx
1409577fd: 48 85 db                    	testq	%rbx, %rbx
140957800: 75 04                       	jne	0x140957806 <.text+0x956806>
140957802: 48 8b 5d 58                 	movq	0x58(%rbp), %rbx
140957806: 4c 8b 7d 68                 	movq	0x68(%rbp), %r15
14095780a: 48 8b 03                    	movq	(%rbx), %rax
14095780d: 4c 8b 73 10                 	movq	0x10(%rbx), %r14
140957811: 4c 29 f0                    	subq	%r14, %rax
140957814: 48 83 f8 27                 	cmpq	$0x27, %rax
140957818: 0f 86 a8 00 00 00           	jbe	0x1409578c6 <.text+0x9568c6>
14095781e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140957822: 48 8d 0d c7 50 98 00        	leaq	0x9850c7(%rip), %rcx    # 0x1412dc8f0 <.text+0x12db8f0>
140957829: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
14095782d: 4e 89 7c 30 08              	movq	%r15, 0x8(%rax,%r14)
140957832: 48 b9 01 00 00 00 00 00 40 40       	movabsq	$0x4040000000000001, %rcx # imm = 0x4040000000000001
14095783c: 4a 89 4c 30 10              	movq	%rcx, 0x10(%rax,%r14)
140957841: 42 c6 44 30 18 00           	movb	$0x0, 0x18(%rax,%r14)
140957847: 48 8d 0d e2 d4 c0 00        	leaq	0xc0d4e2(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14095784e: 4a 89 4c 30 20              	movq	%rcx, 0x20(%rax,%r14)
140957853: 49 83 c6 28                 	addq	$0x28, %r14
140957857: 4c 89 73 10                 	movq	%r14, 0x10(%rbx)
14095785b: 48 8b 07                    	movq	(%rdi), %rax
14095785e: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
140957862: c7 45 e8 00 00 00 00        	movl	$0x0, -0x18(%rbp)
140957869: c7 45 f0 00 00 00 00        	movl	$0x0, -0x10(%rbp)
140957870: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14095787a: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14095787e: c7 45 f8 00 00 00 00        	movl	$0x0, -0x8(%rbp)
140957885: 4c 8d 05 24 d0 15 02        	leaq	0x215d024(%rip), %r8    # 0x142ab48b0
14095788c: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140957890: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140957894: e8 f7 66 98 00              	callq	0x1412ddf90 <.text+0x12dcf90>
140957899: 48 8b 45 78                 	movq	0x78(%rbp), %rax
14095789d: 4c 89 38                    	movq	%r15, (%rax)
1409578a0: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1409578a7: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1409578ab: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1409578b3: 48 89 f0                    	movq	%rsi, %rax
1409578b6: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
1409578bd: 5b                          	popq	%rbx
1409578be: 5f                          	popq	%rdi
1409578bf: 5e                          	popq	%rsi
1409578c0: 41 5e                       	popq	%r14
1409578c2: 41 5f                       	popq	%r15
1409578c4: 5d                          	popq	%rbp
1409578c5: c3                          	retq
1409578c6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409578cf: 41 b8 28 00 00 00           	movl	$0x28, %r8d
1409578d5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409578db: 48 89 d9                    	movq	%rbx, %rcx
1409578de: 4c 89 f2                    	movq	%r14, %rdx
1409578e1: e8 7a 21 f5 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1409578e6: e9 33 ff ff ff              	jmp	0x14095781e <.text+0x95681e>
1409578eb: b9 08 00 00 00              	movl	$0x8, %ecx
1409578f0: ba 08 00 00 00              	movl	$0x8, %edx
1409578f5: e8 06 17 f8 01              	callq	0x1428d9000 <.text+0x28d8000>
1409578fa: 4c 8d 05 1f 16 14 02        	leaq	0x214161f(%rip), %r8    # 0x142a98f20
140957901: b9 01 00 00 00              	movl	$0x1, %ecx
140957906: ba 13 00 00 00              	movl	$0x13, %edx
14095790b: e8 d3 16 f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140957910: 0f 0b                       	ud2
140957912: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140957920: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140957925: 55                          	pushq	%rbp
140957926: 41 57                       	pushq	%r15
140957928: 41 56                       	pushq	%r14
14095792a: 56                          	pushq	%rsi
14095792b: 57                          	pushq	%rdi
14095792c: 53                          	pushq	%rbx
14095792d: 48 83 ec 28                 	subq	$0x28, %rsp
140957931: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140957938: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14095793c: e8 5f 34 f4 ff              	callq	0x14089ada0 <.text+0x899da0>
140957941: 90                          	nop
140957942: 48 83 c4 28                 	addq	$0x28, %rsp
140957946: 5b                          	popq	%rbx
140957947: 5f                          	popq	%rdi
140957948: 5e                          	popq	%rsi
140957949: 41 5e                       	popq	%r14
14095794b: 41 5f                       	popq	%r15
14095794d: 5d                          	popq	%rbp
14095794e: c3                          	retq
14095794f: 90                          	nop
140957950: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140957955: 55                          	pushq	%rbp
140957956: 41 57                       	pushq	%r15
140957958: 41 56                       	pushq	%r14
14095795a: 56                          	pushq	%rsi
14095795b: 57                          	pushq	%rdi
14095795c: 53                          	pushq	%rbx
14095795d: 48 83 ec 28                 	subq	$0x28, %rsp
140957961: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140957968: ba 08 00 00 00              	movl	$0x8, %edx
14095796d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140957973: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140957977: e8 a4 c2 cb ff              	callq	0x140613c20 <.text+0x612c20>
14095797c: 90                          	nop
14095797d: 48 83 c4 28                 	addq	$0x28, %rsp
140957981: 5b                          	popq	%rbx
140957982: 5f                          	popq	%rdi
140957983: 5e                          	popq	%rsi
140957984: 41 5e                       	popq	%r14
140957986: 41 5f                       	popq	%r15
140957988: 5d                          	popq	%rbp
140957989: c3                          	retq
14095798a: cc                          	int3
14095798b: cc                          	int3
14095798c: cc                          	int3
14095798d: cc                          	int3
14095798e: cc                          	int3
14095798f: cc                          	int3
140957990: 55                          	pushq	%rbp
140957991: 41 57                       	pushq	%r15
140957993: 41 56                       	pushq	%r14
140957995: 41 55                       	pushq	%r13
140957997: 41 54                       	pushq	%r12
140957999: 56                          	pushq	%rsi
14095799a: 57                          	pushq	%rdi
14095799b: 53                          	pushq	%rbx
14095799c: 48 81 ec a8 00 00 00        	subq	$0xa8, %rsp
1409579a3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409579ab: 0f 29 7d 10                 	movaps	%xmm7, 0x10(%rbp)
1409579af: 0f 29 75 00                 	movaps	%xmm6, (%rbp)
1409579b3: 48 c7 45 f8 fe ff ff ff     	movq	$-0x2, -0x8(%rbp)
1409579bb: 4c 89 45 a8                 	movq	%r8, -0x58(%rbp)
1409579bf: 0f b6 c2                    	movzbl	%dl, %eax
1409579c2: 48 69 c0 70 04 00 00        	imulq	$0x470, %rax, %rax      # imm = 0x470
1409579c9: 48 8b 11                    	movq	(%rcx), %rdx
1409579cc: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
1409579d0: 44 8b 4c 02 68              	movl	0x68(%rdx,%rax), %r9d
1409579d5: 44 89 4d f4                 	movl	%r9d, -0xc(%rbp)
1409579d9: f3 0f 10 74 02 6c           	movss	0x6c(%rdx,%rax), %xmm6
1409579df: 44 8b 49 1c                 	movl	0x1c(%rcx), %r9d
1409579e3: 45 89 08                    	movl	%r9d, (%r8)
1409579e6: 4c 8b 6c 02 48              	movq	0x48(%rdx,%rax), %r13
1409579eb: 48 8b 5c 02 50              	movq	0x50(%rdx,%rax), %rbx
1409579f0: 48 8b 74 02 58              	movq	0x58(%rdx,%rax), %rsi
1409579f5: f3 0f 10 7c 02 60           	movss	0x60(%rdx,%rax), %xmm7
1409579fb: 0f b6 7c 02 64              	movzbl	0x64(%rdx,%rax), %edi
140957a00: 48 c7 44 02 48 00 00 00 00  	movq	$0x0, 0x48(%rdx,%rax)
140957a09: 48 c7 44 02 50 04 00 00 00  	movq	$0x4, 0x50(%rdx,%rax)
140957a12: 48 c7 44 02 58 00 00 00 00  	movq	$0x0, 0x58(%rdx,%rax)
140957a1b: c7 44 02 60 00 00 00 00     	movl	$0x0, 0x60(%rdx,%rax)
140957a23: c6 44 02 64 00              	movb	$0x0, 0x64(%rdx,%rax)
140957a28: 8b 41 3c                    	movl	0x3c(%rcx), %eax
140957a2b: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
140957a2f: 89 02                       	movl	%eax, (%rdx)
140957a31: 48 8b 41 20                 	movq	0x20(%rcx), %rax
140957a35: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140957a39: 48 8d 04 b5 00 00 00 00     	leaq	(,%rsi,4), %rax
140957a41: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
140957a45: b9 04 00 00 00              	movl	$0x4, %ecx
140957a4a: 4c 89 f2                    	movq	%r14, %rdx
140957a4d: e8 2e 47 42 00              	callq	0x140d7c180 <.text+0xd7b180>
140957a52: 49 89 c4                    	movq	%rax, %r12
140957a55: 49 89 d7                    	movq	%rdx, %r15
140957a58: 48 85 d2                    	testq	%rdx, %rdx
140957a5b: 0f 84 fd 00 00 00           	je	0x140957b5e <.text+0x956b5e>
140957a61: 0f b6 05 a1 d1 cd 02        	movzbl	0x2cdd1a1(%rip), %eax   # 0x143634c09
140957a68: 4c 89 f9                    	movq	%r15, %rcx
140957a6b: 4c 89 e2                    	movq	%r12, %rdx
140957a6e: e8 9d c1 cb ff              	callq	0x140613c10 <.text+0x612c10>
140957a73: 48 85 c0                    	testq	%rax, %rax
140957a76: 0f 84 ee 00 00 00           	je	0x140957b6a <.text+0x956b6a>
140957a7c: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140957a83: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140957a8b: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140957a8f: 48 8d 48 10                 	leaq	0x10(%rax), %rcx
140957a93: 48 89 da                    	movq	%rbx, %rdx
140957a96: 4d 89 f0                    	movq	%r14, %r8
140957a99: e8 62 c1 fc 01              	callq	0x142923c00 <.text+0x2922c00>
140957a9e: 4d 85 ed                    	testq	%r13, %r13
140957aa1: 74 1d                       	je	0x140957ac0 <.text+0x956ac0>
140957aa3: 49 c1 e5 02                 	shlq	$0x2, %r13
140957aa7: 4a 8d 14 6d 00 00 00 00     	leaq	(,%r13,2), %rdx
140957aaf: 4c 01 ea                    	addq	%r13, %rdx
140957ab2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140957ab8: 48 89 d9                    	movq	%rbx, %rcx
140957abb: e8 60 c1 cb ff              	callq	0x140613c20 <.text+0x612c20>
140957ac0: 48 8b 45 e8                 	movq	-0x18(%rbp), %rax
140957ac4: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140957ac8: 48 89 75 d8                 	movq	%rsi, -0x28(%rbp)
140957acc: 40 88 7d e4                 	movb	%dil, -0x1c(%rbp)
140957ad0: f3 0f 11 7d e0              	movss	%xmm7, -0x20(%rbp)
140957ad5: 48 c7 45 b8 01 00 00 00     	movq	$0x1, -0x48(%rbp)
140957add: 48 c7 45 c0 01 00 00 00     	movq	$0x1, -0x40(%rbp)
140957ae5: 8b 45 f4                    	movl	-0xc(%rbp), %eax
140957ae8: 89 45 c8                    	movl	%eax, -0x38(%rbp)
140957aeb: f3 0f 11 75 cc              	movss	%xmm6, -0x34(%rbp)
140957af0: 0f b6 05 12 d1 cd 02        	movzbl	0x2cdd112(%rip), %eax   # 0x143634c09
140957af7: b9 30 00 00 00              	movl	$0x30, %ecx
140957afc: ba 08 00 00 00              	movl	$0x8, %edx
