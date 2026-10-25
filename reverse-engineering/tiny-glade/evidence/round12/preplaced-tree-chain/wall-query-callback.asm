
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1425defd0: 41 57                       	pushq	%r15
1425defd2: 41 56                       	pushq	%r14
1425defd4: 41 55                       	pushq	%r13
1425defd6: 41 54                       	pushq	%r12
1425defd8: 56                          	pushq	%rsi
1425defd9: 57                          	pushq	%rdi
1425defda: 55                          	pushq	%rbp
1425defdb: 53                          	pushq	%rbx
1425defdc: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1425defe3: 44 0f 29 94 24 c0 00 00 00  	movaps	%xmm10, 0xc0(%rsp)
1425defec: 44 0f 29 8c 24 b0 00 00 00  	movaps	%xmm9, 0xb0(%rsp)
1425deff5: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1425deffe: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
1425df006: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1425df00e: 4c 89 c6                    	movq	%r8, %rsi
1425df011: 48 89 d7                    	movq	%rdx, %rdi
1425df014: 48 89 cb                    	movq	%rcx, %rbx
1425df017: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1425df01c: 48 89 da                    	movq	%rbx, %rdx
1425df01f: e8 dc 75 4c fe              	callq	0x140aa6600 <.text+0xaa5600>
1425df024: f2 44 0f 10 54 24 38        	movsd	0x38(%rsp), %xmm10
1425df02b: f2 44 0f 10 4c 24 40        	movsd	0x40(%rsp), %xmm9
1425df032: 48 8d 4c 24 70              	leaq	0x70(%rsp), %rcx
1425df037: 48 89 7c 24 68              	movq	%rdi, 0x68(%rsp)
1425df03c: 48 89 fa                    	movq	%rdi, %rdx
1425df03f: e8 fc 90 6b fe              	callq	0x140c98140 <.text+0xc97140>
1425df044: 48 89 d9                    	movq	%rbx, %rcx
1425df047: e8 94 75 4c fe              	callq	0x140aa65e0 <.text+0xaa55e0>
1425df04c: 0f 28 f0                    	movaps	%xmm0, %xmm6
1425df04f: f2 44 0f 10 44 24 70        	movsd	0x70(%rsp), %xmm8
1425df056: f2 0f 10 7c 24 78           	movsd	0x78(%rsp), %xmm7
1425df05c: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1425df060: 41 0f 5f c0                 	maxps	%xmm8, %xmm0
1425df064: 45 0f c2 c0 03              	cmpunordps	%xmm8, %xmm8
1425df069: 45 0f 54 d0                 	andps	%xmm8, %xmm10
1425df06d: 44 0f 55 c0                 	andnps	%xmm0, %xmm8
1425df071: 45 0f 56 c2                 	orps	%xmm10, %xmm8
1425df075: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1425df07a: 48 89 da                    	movq	%rbx, %rdx
1425df07d: e8 7e 75 4c fe              	callq	0x140aa6600 <.text+0xaa5600>
1425df082: f2 0f 10 44 24 38           	movsd	0x38(%rsp), %xmm0
1425df088: 41 0f c2 c0 02              	cmpleps	%xmm8, %xmm0
1425df08d: 0f 14 c0                    	unpcklps	%xmm0, %xmm0            # xmm0 = xmm0[0,0,1,1]
1425df090: 66 0f 50 c0                 	movmskpd	%xmm0, %eax
1425df094: 83 f8 03                    	cmpl	$0x3, %eax
1425df097: 0f 85 1b 02 00 00           	jne	0x1425df2b8 <.text+0x25de2b8>
1425df09d: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1425df0a1: 0f 5d c7                    	minps	%xmm7, %xmm0
1425df0a4: 0f c2 ff 03                 	cmpunordps	%xmm7, %xmm7
1425df0a8: 44 0f 54 cf                 	andps	%xmm7, %xmm9
1425df0ac: 0f 55 f8                    	andnps	%xmm0, %xmm7
1425df0af: 41 0f 56 f9                 	orps	%xmm9, %xmm7
1425df0b3: f2 0f 10 44 24 40           	movsd	0x40(%rsp), %xmm0
1425df0b9: 0f 28 cf                    	movaps	%xmm7, %xmm1
1425df0bc: 0f c2 c8 02                 	cmpleps	%xmm0, %xmm1
1425df0c0: 0f 14 c9                    	unpcklps	%xmm1, %xmm1            # xmm1 = xmm1[0,0,1,1]
1425df0c3: 66 0f 50 c1                 	movmskpd	%xmm1, %eax
1425df0c7: 83 f8 03                    	cmpl	$0x3, %eax
1425df0ca: 0f 85 00 02 00 00           	jne	0x1425df2d0 <.text+0x25de2d0>
1425df0d0: 48 89 d9                    	movq	%rbx, %rcx
1425df0d3: e8 08 75 4c fe              	callq	0x140aa65e0 <.text+0xaa55e0>
1425df0d8: 0f 2e 05 91 67 34 00        	ucomiss	0x346791(%rip), %xmm0   # 0x142925870
1425df0df: 0f 86 03 02 00 00           	jbe	0x1425df2e8 <.text+0x25de2e8>
1425df0e5: 48 89 74 24 50              	movq	%rsi, 0x50(%rsp)
1425df0ea: f3 41 0f 16 d0              	movshdup	%xmm8, %xmm2            # xmm2 = xmm8[1,1,3,3]
1425df0ef: 48 89 d9                    	movq	%rbx, %rcx
1425df0f2: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1425df0f6: e8 25 74 4c fe              	callq	0x140aa6520 <.text+0xaa5520>
1425df0fb: 41 89 c5                    	movl	%eax, %r13d
1425df0fe: 45 31 e4                    	xorl	%r12d, %r12d
1425df101: 85 c0                       	testl	%eax, %eax
1425df103: 45 0f 4e ec                 	cmovlel	%r12d, %r13d
1425df107: 85 d2                       	testl	%edx, %edx
1425df109: 44 0f 4f e2                 	cmovgl	%edx, %r12d
1425df10d: 0f 57 c0                    	xorps	%xmm0, %xmm0
1425df110: f3 0f 16 d7                 	movshdup	%xmm7, %xmm2            # xmm2 = xmm7[1,1,3,3]
1425df114: f3 0f 58 f8                 	addss	%xmm0, %xmm7
1425df118: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1425df11c: 48 89 d9                    	movq	%rbx, %rcx
1425df11f: 0f 28 cf                    	movaps	%xmm7, %xmm1
1425df122: e8 f9 73 4c fe              	callq	0x140aa6520 <.text+0xaa5520>
1425df127: 41 89 c6                    	movl	%eax, %r14d
1425df12a: 4c 8b 7b 48                 	movq	0x48(%rbx), %r15
1425df12e: 41 8d 47 ff                 	leal	-0x1(%r15), %eax
1425df132: 4d 89 f8                    	movq	%r15, %r8
1425df135: 49 c1 e8 20                 	shrq	$0x20, %r8
1425df139: 41 ff c8                    	decl	%r8d
1425df13c: 44 39 f0                    	cmpl	%r14d, %eax
1425df13f: 44 0f 4c f0                 	cmovll	%eax, %r14d
1425df143: 41 39 d0                    	cmpl	%edx, %r8d
1425df146: 44 0f 4d c2                 	cmovgel	%edx, %r8d
1425df14a: 45 39 c4                    	cmpl	%r8d, %r12d
1425df14d: 0f 8f 26 01 00 00           	jg	0x1425df279 <.text+0x25de279>
1425df153: 45 39 f5                    	cmpl	%r14d, %r13d
1425df156: 7e 25                       	jle	0x1425df17d <.text+0x25de17d>
1425df158: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1425df160: 31 c0                       	xorl	%eax, %eax
1425df162: 45 39 c4                    	cmpl	%r8d, %r12d
1425df165: 0f 9c c1                    	setl	%cl
1425df168: 0f 8d 0b 01 00 00           	jge	0x1425df279 <.text+0x25de279>
1425df16e: 88 c8                       	movb	%cl, %al
1425df170: 41 01 c4                    	addl	%eax, %r12d
1425df173: 45 39 c4                    	cmpl	%r8d, %r12d
1425df176: 7e e8                       	jle	0x1425df160 <.text+0x25de160>
1425df178: e9 fc 00 00 00              	jmp	0x1425df279 <.text+0x25de279>
1425df17d: f3 0f 59 35 eb 66 34 00     	mulss	0x3466eb(%rip), %xmm6   # 0x142925870
1425df185: f3 0f 59 35 67 f5 34 00     	mulss	0x34f567(%rip), %xmm6   # 0x14292e6f4
1425df18d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1425df191: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
1425df196: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1425df19a: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1425df19f: 48 89 5c 24 60              	movq	%rbx, 0x60(%rsp)
1425df1a4: 44 89 6c 24 2c              	movl	%r13d, 0x2c(%rsp)
1425df1a9: 4c 89 44 24 48              	movq	%r8, 0x48(%rsp)
1425df1ae: 48 8b 6c 24 60              	movq	0x60(%rsp), %rbp
1425df1b3: 45 89 e7                    	movl	%r12d, %r15d
1425df1b6: 45 31 e4                    	xorl	%r12d, %r12d
1425df1b9: 45 39 c7                    	cmpl	%r8d, %r15d
1425df1bc: 41 0f 9c c4                 	setl	%r12b
1425df1c0: 45 01 fc                    	addl	%r15d, %r12d
1425df1c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1425df1d0: 31 ff                       	xorl	%edi, %edi
1425df1d2: 45 39 f5                    	cmpl	%r14d, %r13d
1425df1d5: 0f 9c c3                    	setl	%bl
1425df1d8: 48 89 e9                    	movq	%rbp, %rcx
1425df1db: 44 89 ea                    	movl	%r13d, %edx
1425df1de: 45 89 f8                    	movl	%r15d, %r8d
1425df1e1: e8 ea 73 4c fe              	callq	0x140aa65d0 <.text+0xaa55d0>
1425df1e6: 48 89 c6                    	movq	%rax, %rsi
1425df1e9: 48 89 e9                    	movq	%rbp, %rcx
1425df1ec: 48 89 c2                    	movq	%rax, %rdx
1425df1ef: e8 7c 6a 4c fe              	callq	0x140aa5c70 <.text+0xaa4c70>
1425df1f4: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
1425df1f9: f2 0f 10 10                 	movsd	(%rax), %xmm2
1425df1fd: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1425df200: 0f 5c c2                    	subps	%xmm2, %xmm0
1425df203: 0f 59 c0                    	mulps	%xmm0, %xmm0
1425df206: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1425df20a: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1425df20e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1425df211: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1425df215: f3 0f 10 48 08              	movss	0x8(%rax), %xmm1
1425df21a: f3 0f 58 ce                 	addss	%xmm6, %xmm1
1425df21e: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1425df221: 76 20                       	jbe	0x1425df243 <.text+0x25de243>
1425df223: 48 39 74 24 30              	cmpq	%rsi, 0x30(%rsp)
1425df228: 0f 86 d2 00 00 00           	jbe	0x1425df300 <.text+0x25de300>
1425df22e: 48 8d 04 76                 	leaq	(%rsi,%rsi,2), %rax
1425df232: 48 c1 e0 04                 	shlq	$0x4, %rax
1425df236: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1425df23b: 48 83 7c 01 10 00           	cmpq	$0x0, 0x10(%rcx,%rax)
1425df241: 75 2e                       	jne	0x1425df271 <.text+0x25de271>
1425df243: 45 39 f5                    	cmpl	%r14d, %r13d
1425df246: 7d 0f                       	jge	0x1425df257 <.text+0x25de257>
1425df248: 40 88 df                    	movb	%bl, %dil
1425df24b: 41 01 fd                    	addl	%edi, %r13d
1425df24e: 45 39 f5                    	cmpl	%r14d, %r13d
1425df251: 0f 8e 79 ff ff ff           	jle	0x1425df1d0 <.text+0x25de1d0>
1425df257: 4c 8b 44 24 48              	movq	0x48(%rsp), %r8
1425df25c: 45 39 c7                    	cmpl	%r8d, %r15d
1425df25f: 44 8b 6c 24 2c              	movl	0x2c(%rsp), %r13d
1425df264: 7d 13                       	jge	0x1425df279 <.text+0x25de279>
1425df266: 45 39 c4                    	cmpl	%r8d, %r12d
1425df269: 0f 8e 44 ff ff ff           	jle	0x1425df1b3 <.text+0x25de1b3>
1425df26f: eb 08                       	jmp	0x1425df279 <.text+0x25de279>
1425df271: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
1425df276: c6 00 00                    	movb	$0x0, (%rax)
1425df279: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1425df281: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1425df289: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1425df292: 44 0f 28 8c 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm9
1425df29b: 44 0f 28 94 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm10
1425df2a4: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1425df2ab: 5b                          	popq	%rbx
1425df2ac: 5d                          	popq	%rbp
1425df2ad: 5f                          	popq	%rdi
1425df2ae: 5e                          	popq	%rsi
1425df2af: 41 5c                       	popq	%r12
1425df2b1: 41 5d                       	popq	%r13
1425df2b3: 41 5e                       	popq	%r14
1425df2b5: 41 5f                       	popq	%r15
1425df2b7: c3                          	retq
1425df2b8: 48 8d 0d 29 5c b5 00        	leaq	0xb55c29(%rip), %rcx    # 0x143134ee8
1425df2bf: 4c 8d 05 da 5c b5 00        	leaq	0xb55cda(%rip), %r8     # 0x143134fa0
1425df2c6: ba 44 00 00 00              	movl	$0x44, %edx
1425df2cb: e8 60 a1 2f 00              	callq	0x1428d9430 <.text+0x28d8430>
1425df2d0: 48 8d 0d e1 5c b5 00        	leaq	0xb55ce1(%rip), %rcx    # 0x143134fb8
1425df2d7: 4c 8d 05 22 5d b5 00        	leaq	0xb55d22(%rip), %r8     # 0x143135000
1425df2de: ba 44 00 00 00              	movl	$0x44, %edx
1425df2e3: e8 48 a1 2f 00              	callq	0x1428d9430 <.text+0x28d8430>
1425df2e8: 48 8d 0d 29 5d b5 00        	leaq	0xb55d29(%rip), %rcx    # 0x143135018
1425df2ef: 4c 8d 05 5a 5d b5 00        	leaq	0xb55d5a(%rip), %r8     # 0x143135050
1425df2f6: ba 31 00 00 00              	movl	$0x31, %edx
1425df2fb: e8 30 a1 2f 00              	callq	0x1428d9430 <.text+0x28d8430>
1425df300: 4c 8d 05 61 5d b5 00        	leaq	0xb55d61(%rip), %r8     # 0x143135068
1425df307: 48 89 f1                    	movq	%rsi, %rcx
1425df30a: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
1425df30f: e8 04 a2 2f 00              	callq	0x1428d9518 <.text+0x28d8518>
1425df314: cc                          	int3
