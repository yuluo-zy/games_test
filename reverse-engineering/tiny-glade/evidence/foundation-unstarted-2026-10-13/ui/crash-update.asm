
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140620470: 55                          	pushq	%rbp
140620471: 41 57                       	pushq	%r15
140620473: 41 56                       	pushq	%r14
140620475: 56                          	pushq	%rsi
140620476: 57                          	pushq	%rdi
140620477: 53                          	pushq	%rbx
140620478: 48 81 ec 78 01 00 00        	subq	$0x178, %rsp            # imm = 0x178
14062047f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140620487: 48 c7 85 f0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xf0(%rbp)
140620492: 49 89 d7                    	movq	%rdx, %r15
140620495: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14062049f: 49 33 00                    	xorq	(%r8), %rax
1406204a2: 48 83 f8 0a                 	cmpq	$0xa, %rax
1406204a6: ba 01 00 00 00              	movl	$0x1, %edx
1406204ab: 48 0f 42 d0                 	cmovbq	%rax, %rdx
1406204af: 48 89 ce                    	movq	%rcx, %rsi
1406204b2: 48 8d 05 ef be 3e 02        	leaq	0x23ebeef(%rip), %rax   # 0x142a0c3a8
1406204b9: 48 63 0c 90                 	movslq	(%rax,%rdx,4), %rcx
1406204bd: 48 01 c1                    	addq	%rax, %rcx
1406204c0: ff e1                       	jmpq	*%rcx
1406204c2: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1406204cc: 48 89 06                    	movq	%rax, (%rsi)
1406204cf: 48 c7 46 08 00 00 00 00     	movq	$0x0, 0x8(%rsi)
1406204d7: e9 53 09 00 00              	jmp	0x140620e2f <.text+0x61fe2f>
1406204dc: 49 8b 47 40                 	movq	0x40(%r15), %rax
1406204e0: 48 b9 f0 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff0, %rcx # imm = 0x7FFFFFFFFFFFFFF0
1406204ea: 48 01 c1                    	addq	%rax, %rcx
1406204ed: 48 83 f9 04                 	cmpq	$0x4, %rcx
1406204f1: 0f 93 c2                    	setae	%dl
1406204f4: 48 83 f9 01                 	cmpq	$0x1, %rcx
1406204f8: 0f 94 c1                    	sete	%cl
1406204fb: 08 d1                       	orb	%dl, %cl
1406204fd: 0f 84 1f 09 00 00           	je	0x140620e22 <.text+0x61fe22>
140620503: 49 83 c7 40                 	addq	$0x40, %r15
140620507: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
14062050e: 48 ba f3 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff3, %rdx # imm = 0x7FFFFFFFFFFFFFF3
140620518: 48 01 c2                    	addq	%rax, %rdx
14062051b: 48 83 fa 03                 	cmpq	$0x3, %rdx
14062051f: 0f 93 c1                    	setae	%cl
140620522: 48 83 fa 01                 	cmpq	$0x1, %rdx
140620526: 0f 94 c2                    	sete	%dl
140620529: 49 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %r8 # imm = 0x8000000000000004
140620533: 4c 39 c0                    	cmpq	%r8, %rax
140620536: 77 10                       	ja	0x140620548 <.text+0x61f548>
140620538: 08 d1                       	orb	%dl, %cl
14062053a: 74 0c                       	je	0x140620548 <.text+0x61f548>
14062053c: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620543: e8 48 9d 79 00              	callq	0x140dba290 <.text+0xdb9290>
140620548: 48 b8 0f 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff1, %rax # imm = 0x800000000000000F
140620552: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620559: 48 89 01                    	movq	%rax, (%rcx)
14062055c: e9 c1 08 00 00              	jmp	0x140620e22 <.text+0x61fe22>
140620561: 49 8b 40 08                 	movq	0x8(%r8), %rax
140620565: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14062056c: 49 8b 40 10                 	movq	0x10(%r8), %rax
140620570: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140620577: 49 8b 47 40                 	movq	0x40(%r15), %rax
14062057b: 48 b9 f0 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff0, %rcx # imm = 0x7FFFFFFFFFFFFFF0
140620585: 48 01 c1                    	addq	%rax, %rcx
140620588: 48 83 f9 04                 	cmpq	$0x4, %rcx
14062058c: 0f 93 c2                    	setae	%dl
14062058f: 48 83 f9 01                 	cmpq	$0x1, %rcx
140620593: 0f 94 c1                    	sete	%cl
140620596: 08 d1                       	orb	%dl, %cl
140620598: 74 3c                       	je	0x1406205d6 <.text+0x61f5d6>
14062059a: 48 b9 f3 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff3, %rcx # imm = 0x7FFFFFFFFFFFFFF3
1406205a4: 48 01 c8                    	addq	%rcx, %rax
1406205a7: 48 83 f8 03                 	cmpq	$0x3, %rax
1406205ab: 0f 93 c1                    	setae	%cl
1406205ae: 48 83 f8 01                 	cmpq	$0x1, %rax
1406205b2: 0f 94 c2                    	sete	%dl
1406205b5: 48 83 f8 f7                 	cmpq	$-0x9, %rax
1406205b9: 77 1b                       	ja	0x1406205d6 <.text+0x61f5d6>
1406205bb: 08 d1                       	orb	%dl, %cl
1406205bd: 74 17                       	je	0x1406205d6 <.text+0x61f5d6>
1406205bf: 4d 8b 40 18                 	movq	0x18(%r8), %r8
1406205c3: 49 83 c7 40                 	addq	$0x40, %r15
1406205c7: 4c 89 f9                    	movq	%r15, %rcx
1406205ca: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
1406205d1: e8 fa a8 79 00              	callq	0x140dbaed0 <.text+0xdb9ed0>
1406205d6: 48 8b 95 e0 00 00 00        	movq	0xe0(%rbp), %rdx
1406205dd: 48 85 d2                    	testq	%rdx, %rdx
1406205e0: 0f 84 3c 08 00 00           	je	0x140620e22 <.text+0x61fe22>
1406205e6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1406205ec: e9 55 05 00 00              	jmp	0x140620b46 <.text+0x61fb46>
1406205f1: 48 8d 0d 0f ae 3e 02        	leaq	0x23eae0f(%rip), %rcx   # 0x142a0b407
1406205f8: ba 22 00 00 00              	movl	$0x22, %edx
1406205fd: e8 ce ec 55 00              	callq	0x140b7f2d0 <.text+0xb7e2d0>
140620602: 89 c1                       	movl	%eax, %ecx
140620604: 83 e1 03                    	andl	$0x3, %ecx
140620607: 83 f9 01                    	cmpl	$0x1, %ecx
14062060a: 0f 85 12 08 00 00           	jne	0x140620e22 <.text+0x61fe22>
140620610: 48 89 c1                    	movq	%rax, %rcx
140620613: 48 ff c8                    	decq	%rax
140620616: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14062061d: 48 8b 41 ff                 	movq	-0x1(%rcx), %rax
140620621: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140620628: 48 8b 41 07                 	movq	0x7(%rcx), %rax
14062062c: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
140620633: 48 8b 00                    	movq	(%rax), %rax
140620636: 48 85 c0                    	testq	%rax, %rax
140620639: 0f 84 dc 04 00 00           	je	0x140620b1b <.text+0x61fb1b>
14062063f: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620646: ff d0                       	callq	*%rax
140620648: e9 ce 04 00 00              	jmp	0x140620b1b <.text+0x61fb1b>
14062064d: 49 8b 87 a0 00 00 00        	movq	0xa0(%r15), %rax
140620654: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14062065b: 41 0f 10 87 90 00 00 00     	movups	0x90(%r15), %xmm0
140620663: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
14062066a: 41 0f 10 87 80 00 00 00     	movups	0x80(%r15), %xmm0
140620672: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140620676: 41 0f 10 47 40              	movups	0x40(%r15), %xmm0
14062067b: 41 0f 10 4f 50              	movups	0x50(%r15), %xmm1
140620680: 41 0f 10 57 60              	movups	0x60(%r15), %xmm2
140620685: 41 0f 10 5f 70              	movups	0x70(%r15), %xmm3
14062068a: 0f 29 5d 60                 	movaps	%xmm3, 0x60(%rbp)
14062068e: 0f 29 55 50                 	movaps	%xmm2, 0x50(%rbp)
140620692: 0f 29 4d 40                 	movaps	%xmm1, 0x40(%rbp)
140620696: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
14062069a: 48 b8 10 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff0, %rax # imm = 0x8000000000000010
1406206a4: 49 89 47 40                 	movq	%rax, 0x40(%r15)
1406206a8: 48 b9 f0 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff0, %rcx # imm = 0x7FFFFFFFFFFFFFF0
1406206b2: 48 8b 45 30                 	movq	0x30(%rbp), %rax
1406206b6: 48 01 c8                    	addq	%rcx, %rax
1406206b9: 48 83 f8 04                 	cmpq	$0x4, %rax
1406206bd: 0f 93 c1                    	setae	%cl
1406206c0: 48 83 f8 01                 	cmpq	$0x1, %rax
1406206c4: 0f 94 c0                    	sete	%al
1406206c7: 08 c8                       	orb	%cl, %al
1406206c9: 0f 84 73 07 00 00           	je	0x140620e42 <.text+0x61fe42>
1406206cf: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1406206d6: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1406206dd: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1406206e1: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
1406206e8: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
1406206ef: ff 50 18                    	callq	*0x18(%rax)
1406206f2: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
1406206f9: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1406206fd: 48 85 d2                    	testq	%rdx, %rdx
140620700: 74 10                       	je	0x140620712 <.text+0x61f712>
140620702: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140620706: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
14062070d: e8 0e 35 ff ff              	callq	0x140613c20 <.text+0x612c20>
140620712: 49 8d 7f 40                 	leaq	0x40(%r15), %rdi
140620716: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14062071a: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
140620721: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140620725: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
14062072c: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140620730: 48 ba f0 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff0, %rdx # imm = 0x7FFFFFFFFFFFFFF0
14062073a: 48 01 c2                    	addq	%rax, %rdx
14062073d: 48 83 fa 04                 	cmpq	$0x4, %rdx
140620741: 0f 92 c1                    	setb	%cl
140620744: 48 83 fa 01                 	cmpq	$0x1, %rdx
140620748: 0f 95 c2                    	setne	%dl
14062074b: 84 d1                       	testb	%dl, %cl
14062074d: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
140620754: 0f 85 30 06 00 00           	jne	0x140620d8a <.text+0x61fd8a>
14062075a: 48 ba f3 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff3, %rdx # imm = 0x7FFFFFFFFFFFFFF3
140620764: 48 01 c2                    	addq	%rax, %rdx
140620767: 48 83 fa 03                 	cmpq	$0x3, %rdx
14062076b: 0f 93 c1                    	setae	%cl
14062076e: 48 83 fa 01                 	cmpq	$0x1, %rdx
140620772: 0f 94 c2                    	sete	%dl
140620775: 49 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %r8 # imm = 0x8000000000000004
14062077f: 4c 39 c0                    	cmpq	%r8, %rax
140620782: 0f 87 0b 06 00 00           	ja	0x140620d93 <.text+0x61fd93>
140620788: 08 d1                       	orb	%dl, %cl
14062078a: 0f 84 03 06 00 00           	je	0x140620d93 <.text+0x61fd93>
140620790: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140620794: e8 f7 9a 79 00              	callq	0x140dba290 <.text+0xdb9290>
140620799: e9 f5 05 00 00              	jmp	0x140620d93 <.text+0x61fd93>
14062079e: 49 8b 47 40                 	movq	0x40(%r15), %rax
1406207a2: 48 b9 f0 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff0, %rcx # imm = 0x7FFFFFFFFFFFFFF0
1406207ac: 48 01 c1                    	addq	%rax, %rcx
1406207af: 48 83 f9 04                 	cmpq	$0x4, %rcx
1406207b3: 0f 93 c2                    	setae	%dl
1406207b6: 48 83 f9 01                 	cmpq	$0x1, %rcx
1406207ba: 0f 94 c1                    	sete	%cl
1406207bd: 08 d1                       	orb	%dl, %cl
1406207bf: 0f 84 5d 06 00 00           	je	0x140620e22 <.text+0x61fe22>
1406207c5: 48 b9 f3 ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffff3, %rcx # imm = 0x7FFFFFFFFFFFFFF3
1406207cf: 48 01 c8                    	addq	%rcx, %rax
1406207d2: 48 83 f8 03                 	cmpq	$0x3, %rax
1406207d6: 0f 93 c1                    	setae	%cl
1406207d9: 48 83 f8 01                 	cmpq	$0x1, %rax
1406207dd: 0f 94 c2                    	sete	%dl
1406207e0: 48 83 f8 f7                 	cmpq	$-0x9, %rax
1406207e4: 0f 87 38 06 00 00           	ja	0x140620e22 <.text+0x61fe22>
1406207ea: 08 d1                       	orb	%dl, %cl
1406207ec: 0f 84 30 06 00 00           	je	0x140620e22 <.text+0x61fe22>
1406207f2: 41 80 b7 90 00 00 00 01     	xorb	$0x1, 0x90(%r15)
1406207fa: e9 23 06 00 00              	jmp	0x140620e22 <.text+0x61fe22>
1406207ff: 41 0f 10 00                 	movups	(%r8), %xmm0
140620803: 41 0f 10 48 10              	movups	0x10(%r8), %xmm1
140620808: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
14062080c: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140620810: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140620814: e8 67 ee 79 00              	callq	0x140dbf680 <.text+0xdbe680>
140620819: 89 c1                       	movl	%eax, %ecx
14062081b: 83 e1 03                    	andl	$0x3, %ecx
14062081e: 83 f9 01                    	cmpl	$0x1, %ecx
140620821: 0f 85 fb 05 00 00           	jne	0x140620e22 <.text+0x61fe22>
140620827: 48 89 c1                    	movq	%rax, %rcx
14062082a: 48 ff c8                    	decq	%rax
14062082d: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140620834: 48 8b 41 ff                 	movq	-0x1(%rcx), %rax
140620838: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
14062083f: 48 8b 41 07                 	movq	0x7(%rcx), %rax
140620843: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14062084a: 48 8b 00                    	movq	(%rax), %rax
14062084d: 48 85 c0                    	testq	%rax, %rax
140620850: 0f 84 c5 02 00 00           	je	0x140620b1b <.text+0x61fb1b>
140620856: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
14062085d: ff d0                       	callq	*%rax
14062085f: e9 b7 02 00 00              	jmp	0x140620b1b <.text+0x61fb1b>
140620864: 49 8b 40 08                 	movq	0x8(%r8), %rax
140620868: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14062086f: 49 8b 40 10                 	movq	0x10(%r8), %rax
140620873: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14062087a: 49 8b 58 18                 	movq	0x18(%r8), %rbx
14062087e: c6 85 ed 00 00 00 00        	movb	$0x0, 0xed(%rbp)
140620885: 48 8d 7d 10                 	leaq	0x10(%rbp), %rdi
140620889: 48 8d 95 ee 00 00 00        	leaq	0xee(%rbp), %rdx
140620890: 48 89 f9                    	movq	%rdi, %rcx
140620893: e8 c8 e2 00 00              	callq	0x14062eb60 <.text+0x62db60>
140620898: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14062089f: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1406208a6: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
1406208ad: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
1406208b4: 48 89 9d c0 00 00 00        	movq	%rbx, 0xc0(%rbp)
1406208bb: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1406208bf: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1406208c3: 0f 29 4d 40                 	movaps	%xmm1, 0x40(%rbp)
1406208c7: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
1406208cb: c6 85 ed 00 00 00 01        	movb	$0x1, 0xed(%rbp)
1406208d2: 4c 8d 75 f8                 	leaq	-0x8(%rbp), %r14
1406208d6: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
1406208da: 48 8d 9d b0 00 00 00        	leaq	0xb0(%rbp), %rbx
1406208e1: 4c 89 f1                    	movq	%r14, %rcx
1406208e4: 49 89 d8                    	movq	%rbx, %r8
1406208e7: e8 74 f8 79 00              	callq	0x140dc0160 <.text+0xdbf160>
1406208ec: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
1406208f0: 48 b9 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rcx # imm = 0x8000000000000004
1406208fa: 48 39 c8                    	cmpq	%rcx, %rax
1406208fd: 0f 85 80 03 00 00           	jne	0x140620c83 <.text+0x61fc83>
140620903: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
14062090b: c6 85 ef 00 00 00 00        	movb	$0x0, 0xef(%rbp)
140620912: e8 c9 2c 74 00              	callq	0x140d635e0 <.text+0xd625e0>
140620917: e9 b1 04 00 00              	jmp	0x140620dcd <.text+0x61fdcd>
14062091c: 49 8b 48 08                 	movq	0x8(%r8), %rcx
140620920: 49 8b 50 10                 	movq	0x10(%r8), %rdx
140620924: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140620928: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
14062092c: 48 8b 3d 75 ba 00 03        	movq	0x300ba75(%rip), %rdi   # 0x14362c3a8
140620933: 48 8b 07                    	movq	(%rdi), %rax
140620936: 48 83 f8 02                 	cmpq	$0x2, %rax
14062093a: 0f 86 bc 00 00 00           	jbe	0x1406209fc <.text+0x61f9fc>
140620940: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
140620944: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140620948: 48 8d 05 51 2f f1 ff        	leaq	-0xed0af(%rip), %rax    # 0x1405338a0 <.text+0x5328a0>
14062094f: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140620953: 48 8d 0d a6 ab 3e 02        	leaq	0x23eaba6(%rip), %rcx   # 0x142a0b500
14062095a: e8 21 d4 05 00              	callq	0x14067dd80 <.text+0x67cd80>
14062095f: 0f 10 00                    	movups	(%rax), %xmm0
140620962: 8b 40 10                    	movl	0x10(%rax), %eax
140620965: 48 c7 45 60 03 00 00 00     	movq	$0x3, 0x60(%rbp)
14062096d: 48 8d 0d 5c ab 3e 02        	leaq	0x23eab5c(%rip), %rcx   # 0x142a0b4d0
140620974: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
140620978: 48 c7 45 70 12 00 00 00     	movq	$0x12, 0x70(%rbp)
140620980: 48 8d 15 69 ab 3e 02        	leaq	0x23eab69(%rip), %rdx   # 0x142a0b4f0
140620987: 48 89 95 80 00 00 00        	movq	%rdx, 0x80(%rbp)
14062098e: 48 c7 85 88 00 00 00 01 00 00 00    	movq	$0x1, 0x88(%rbp)
140620999: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
14062099d: 48 89 95 90 00 00 00        	movq	%rdx, 0x90(%rbp)
1406209a4: 48 c7 85 98 00 00 00 01 00 00 00    	movq	$0x1, 0x98(%rbp)
1406209af: 48 c7 85 a0 00 00 00 00 00 00 00    	movq	$0x0, 0xa0(%rbp)
1406209ba: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
1406209c2: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
1406209c6: 48 c7 45 40 12 00 00 00     	movq	$0x12, 0x40(%rbp)
1406209ce: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
1406209d6: 0f 11 45 50                 	movups	%xmm0, 0x50(%rbp)
1406209da: c7 45 78 01 00 00 00        	movl	$0x1, 0x78(%rbp)
1406209e1: 89 45 7c                    	movl	%eax, 0x7c(%rbp)
1406209e4: 48 8d 8d ee 00 00 00        	leaq	0xee(%rbp), %rcx
1406209eb: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
1406209ef: e8 3c 99 72 00              	callq	0x140d4a330 <.text+0xd49330>
1406209f4: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
1406209f8: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
1406209fc: e8 cf e8 55 00              	callq	0x140b7f2d0 <.text+0xb7e2d0>
140620a01: 48 85 c0                    	testq	%rax, %rax
140620a04: 0f 84 18 04 00 00           	je	0x140620e22 <.text+0x61fe22>
140620a0a: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140620a11: 48 8b 0f                    	movq	(%rdi), %rcx
140620a14: 48 85 c9                    	testq	%rcx, %rcx
140620a17: 0f 84 be 00 00 00           	je	0x140620adb <.text+0x61fadb>
140620a1d: 48 8d 85 b0 00 00 00        	leaq	0xb0(%rbp), %rax
140620a24: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140620a28: 48 8d 05 b1 a2 73 00        	leaq	0x73a2b1(%rip), %rax    # 0x140d5ace0 <.text+0xd59ce0>
140620a2f: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140620a33: 48 8d 0d 0e ab 3e 02        	leaq	0x23eab0e(%rip), %rcx   # 0x142a0b548
140620a3a: e8 41 d3 05 00              	callq	0x14067dd80 <.text+0x67cd80>
140620a3f: 0f 10 00                    	movups	(%rax), %xmm0
140620a42: 8b 40 10                    	movl	0x10(%rax), %eax
140620a45: 48 c7 45 60 01 00 00 00     	movq	$0x1, 0x60(%rbp)
140620a4d: 48 8d 0d 7c aa 3e 02        	leaq	0x23eaa7c(%rip), %rcx   # 0x142a0b4d0
140620a54: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
140620a58: 48 c7 45 70 12 00 00 00     	movq	$0x12, 0x70(%rbp)
140620a60: 48 8d 15 d1 aa 3e 02        	leaq	0x23eaad1(%rip), %rdx   # 0x142a0b538
140620a67: 48 89 95 80 00 00 00        	movq	%rdx, 0x80(%rbp)
140620a6e: 48 c7 85 88 00 00 00 01 00 00 00    	movq	$0x1, 0x88(%rbp)
140620a79: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140620a7d: 48 89 95 90 00 00 00        	movq	%rdx, 0x90(%rbp)
140620a84: 48 c7 85 98 00 00 00 01 00 00 00    	movq	$0x1, 0x98(%rbp)
140620a8f: 48 c7 85 a0 00 00 00 00 00 00 00    	movq	$0x0, 0xa0(%rbp)
140620a9a: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140620aa2: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140620aa6: 48 c7 45 40 12 00 00 00     	movq	$0x12, 0x40(%rbp)
140620aae: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
140620ab6: 0f 11 45 50                 	movups	%xmm0, 0x50(%rbp)
140620aba: c7 45 78 01 00 00 00        	movl	$0x1, 0x78(%rbp)
140620ac1: 89 45 7c                    	movl	%eax, 0x7c(%rbp)
140620ac4: 48 8d 8d ee 00 00 00        	leaq	0xee(%rbp), %rcx
140620acb: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
140620acf: e8 5c 98 72 00              	callq	0x140d4a330 <.text+0xd49330>
140620ad4: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140620adb: 89 c1                       	movl	%eax, %ecx
140620add: 83 e1 03                    	andl	$0x3, %ecx
140620ae0: 83 f9 01                    	cmpl	$0x1, %ecx
140620ae3: 0f 85 39 03 00 00           	jne	0x140620e22 <.text+0x61fe22>
140620ae9: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
140620aed: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
140620af4: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
140620af8: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
140620aff: 48 8b 40 07                 	movq	0x7(%rax), %rax
140620b03: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
140620b0a: 48 8b 00                    	movq	(%rax), %rax
140620b0d: 48 85 c0                    	testq	%rax, %rax
140620b10: 74 09                       	je	0x140620b1b <.text+0x61fb1b>
140620b12: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620b19: ff d0                       	callq	*%rax
140620b1b: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620b22: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140620b29: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140620b2d: 48 85 d2                    	testq	%rdx, %rdx
140620b30: 74 09                       	je	0x140620b3b <.text+0x61fb3b>
140620b32: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140620b36: e8 e5 30 ff ff              	callq	0x140613c20 <.text+0x612c20>
140620b3b: ba 18 00 00 00              	movl	$0x18, %edx
140620b40: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140620b46: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140620b4d: e8 ce 30 ff ff              	callq	0x140613c20 <.text+0x612c20>
140620b52: e9 cb 02 00 00              	jmp	0x140620e22 <.text+0x61fe22>
140620b57: 48 b9 12 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffee, %rcx # imm = 0x8000000000000012
140620b61: 49 39 4f 40                 	cmpq	%rcx, 0x40(%r15)
140620b65: 0f 85 b7 02 00 00           	jne	0x140620e22 <.text+0x61fe22>
140620b6b: 49 8b 47 50                 	movq	0x50(%r15), %rax
140620b6f: 48 8b 00                    	movq	(%rax), %rax
140620b72: 48 83 f8 01                 	cmpq	$0x1, %rax
140620b76: 0f 85 a6 02 00 00           	jne	0x140620e22 <.text+0x61fe22>
140620b7c: 49 8d 7f 40                 	leaq	0x40(%r15), %rdi
140620b80: 48 8b 47 60                 	movq	0x60(%rdi), %rax
140620b84: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
140620b8b: 0f 10 47 50                 	movups	0x50(%rdi), %xmm0
140620b8f: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
140620b96: 0f 10 47 40                 	movups	0x40(%rdi), %xmm0
140620b9a: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140620b9e: 0f 10 07                    	movups	(%rdi), %xmm0
140620ba1: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
140620ba5: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
140620ba9: 0f 10 5f 30                 	movups	0x30(%rdi), %xmm3
140620bad: 0f 29 5d 60                 	movaps	%xmm3, 0x60(%rbp)
140620bb1: 0f 29 55 50                 	movaps	%xmm2, 0x50(%rbp)
140620bb5: 0f 29 4d 40                 	movaps	%xmm1, 0x40(%rbp)
140620bb9: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
140620bbd: 48 b8 10 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff0, %rax # imm = 0x8000000000000010
140620bc7: 48 89 07                    	movq	%rax, (%rdi)
140620bca: 48 39 4d 30                 	cmpq	%rcx, 0x30(%rbp)
140620bce: 0f 85 8f 02 00 00           	jne	0x140620e63 <.text+0x61fe63>
140620bd4: 49 83 c7 48                 	addq	$0x48, %r15
140620bd8: 49 8b 47 10                 	movq	0x10(%r15), %rax
140620bdc: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140620be0: 41 0f 10 07                 	movups	(%r15), %xmm0
140620be4: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
140620be8: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140620bec: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
140620bf0: e8 7b f9 79 00              	callq	0x140dc0570 <.text+0xdbf570>
140620bf5: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
140620bff: 48 39 45 10                 	cmpq	%rax, 0x10(%rbp)
140620c03: 0f 84 7b 02 00 00           	je	0x140620e84 <.text+0x61fe84>
140620c09: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
140620c10: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140620c14: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
140620c1b: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140620c1f: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
140620c26: 48 b8 12 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffee, %rax # imm = 0x8000000000000012
140620c30: 48 39 45 30                 	cmpq	%rax, 0x30(%rbp)
140620c34: 74 09                       	je	0x140620c3f <.text+0x61fc3f>
140620c36: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140620c3a: e8 41 de ff ff              	callq	0x14061ea80 <.text+0x61da80>
140620c3f: 48 89 f9                    	movq	%rdi, %rcx
140620c42: 48 89 bd e0 00 00 00        	movq	%rdi, 0xe0(%rbp)
140620c49: e8 32 de ff ff              	callq	0x14061ea80 <.text+0x61da80>
140620c4e: 48 b8 13 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffed, %rax # imm = 0x8000000000000013
140620c58: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140620c5f: 48 89 01                    	movq	%rax, (%rcx)
140620c62: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140620c69: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620c70: 48 89 41 10                 	movq	%rax, 0x10(%rcx)
140620c74: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
140620c7b: 0f 11 01                    	movups	%xmm0, (%rcx)
140620c7e: e9 9f 01 00 00              	jmp	0x140620e22 <.text+0x61fe22>
140620c83: 0f 10 45 00                 	movups	(%rbp), %xmm0
140620c87: 0f 11 45 38                 	movups	%xmm0, 0x38(%rbp)
140620c8b: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140620c8f: c6 85 ed 00 00 00 01        	movb	$0x1, 0xed(%rbp)
140620c96: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140620c9a: e8 31 9f 2b 02              	callq	0x1428dabd0 <.text+0x28d9bd0>
140620c9f: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140620ca3: 4c 89 b5 b0 00 00 00        	movq	%r14, 0xb0(%rbp)
140620caa: 48 8b 0d f7 b6 00 03        	movq	0x300b6f7(%rip), %rcx   # 0x14362c3a8
140620cb1: 48 8b 09                    	movq	(%rcx), %rcx
140620cb4: 48 85 c9                    	testq	%rcx, %rcx
140620cb7: 0f 84 0b 01 00 00           	je	0x140620dc8 <.text+0x61fdc8>
140620cbd: 48 89 5d 10                 	movq	%rbx, 0x10(%rbp)
140620cc1: 48 8d 05 88 6d f9 ff        	leaq	-0x69278(%rip), %rax    # 0x1405b7a50 <.text+0x5b6a50>
140620cc8: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140620ccc: c6 85 ef 00 00 00 01        	movb	$0x1, 0xef(%rbp)
140620cd3: 48 8d 0d de a7 3e 02        	leaq	0x23ea7de(%rip), %rcx   # 0x142a0b4b8
140620cda: e8 a1 d0 05 00              	callq	0x14067dd80 <.text+0x67cd80>
140620cdf: 0f 10 00                    	movups	(%rax), %xmm0
140620ce2: 8b 40 10                    	movl	0x10(%rax), %eax
140620ce5: 48 c7 45 60 01 00 00 00     	movq	$0x1, 0x60(%rbp)
140620ced: 48 8d 0d dc a7 3e 02        	leaq	0x23ea7dc(%rip), %rcx   # 0x142a0b4d0
140620cf4: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
140620cf8: 48 c7 45 70 12 00 00 00     	movq	$0x12, 0x70(%rbp)
140620d00: 48 8d 15 a1 a7 3e 02        	leaq	0x23ea7a1(%rip), %rdx   # 0x142a0b4a8
140620d07: 48 89 95 80 00 00 00        	movq	%rdx, 0x80(%rbp)
140620d0e: 48 c7 85 88 00 00 00 01 00 00 00    	movq	$0x1, 0x88(%rbp)
140620d19: 48 89 bd 90 00 00 00        	movq	%rdi, 0x90(%rbp)
140620d20: 48 c7 85 98 00 00 00 01 00 00 00    	movq	$0x1, 0x98(%rbp)
140620d2b: 48 c7 85 a0 00 00 00 00 00 00 00    	movq	$0x0, 0xa0(%rbp)
140620d36: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140620d3e: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140620d42: 48 c7 45 40 12 00 00 00     	movq	$0x12, 0x40(%rbp)
140620d4a: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
140620d52: 0f 11 45 50                 	movups	%xmm0, 0x50(%rbp)
140620d56: c7 45 78 01 00 00 00        	movl	$0x1, 0x78(%rbp)
140620d5d: 89 45 7c                    	movl	%eax, 0x7c(%rbp)
140620d60: c6 85 ef 00 00 00 01        	movb	$0x1, 0xef(%rbp)
140620d67: 48 8d 8d ee 00 00 00        	leaq	0xee(%rbp), %rcx
140620d6e: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
140620d72: e8 b9 95 72 00              	callq	0x140d4a330 <.text+0xd49330>
140620d77: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140620d7b: ba 00 ca 9a 3b              	movl	$0x3b9aca00, %edx       # imm = 0x3B9ACA00
140620d80: 48 85 c0                    	testq	%rax, %rax
140620d83: 75 48                       	jne	0x140620dcd <.text+0x61fdcd>
140620d85: e9 81 fb ff ff              	jmp	0x14062090b <.text+0x61f90b>
140620d8a: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140620d8e: e8 ed dc ff ff              	callq	0x14061ea80 <.text+0x61da80>
140620d93: 48 89 f9                    	movq	%rdi, %rcx
140620d96: e8 e5 dc ff ff              	callq	0x14061ea80 <.text+0x61da80>
140620d9b: 48 b8 12 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffee, %rax # imm = 0x8000000000000012
140620da5: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620dac: 48 89 41 40                 	movq	%rax, 0x40(%rcx)
140620db0: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
140620db7: 0f 11 41 48                 	movups	%xmm0, 0x48(%rcx)
140620dbb: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140620dc2: 48 89 41 58                 	movq	%rax, 0x58(%rcx)
140620dc6: eb 5a                       	jmp	0x140620e22 <.text+0x61fe22>
140620dc8: ba 00 ca 9a 3b              	movl	$0x3b9aca00, %edx       # imm = 0x3B9ACA00
140620dcd: 89 95 d0 00 00 00           	movl	%edx, 0xd0(%rbp)
140620dd3: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
140620dda: 41 81 bf b0 00 00 00 00 ca 9a 3b    	cmpl	$0x3b9aca00, 0xb0(%r15) # imm = 0x3B9ACA00
140620de5: 4c 89 bd d8 00 00 00        	movq	%r15, 0xd8(%rbp)
140620dec: 75 13                       	jne	0x140620e01 <.text+0x61fe01>
140620dee: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
140620df5: 48 8d 88 a8 00 00 00        	leaq	0xa8(%rax), %rcx
140620dfc: e8 2f be 72 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140620e01: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140620e08: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
140620e0f: 48 89 81 a8 00 00 00        	movq	%rax, 0xa8(%rcx)
140620e16: 8b 85 d0 00 00 00           	movl	0xd0(%rbp), %eax
140620e1c: 89 81 b0 00 00 00           	movl	%eax, 0xb0(%rcx)
140620e22: 48 b8 1e 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffe2, %rax # imm = 0x800000000000001E
140620e2c: 48 89 06                    	movq	%rax, (%rsi)
140620e2f: 48 89 f0                    	movq	%rsi, %rax
140620e32: 48 81 c4 78 01 00 00        	addq	$0x178, %rsp            # imm = 0x178
140620e39: 5b                          	popq	%rbx
140620e3a: 5f                          	popq	%rdi
140620e3b: 5e                          	popq	%rsi
140620e3c: 41 5e                       	popq	%r14
140620e3e: 41 5f                       	popq	%r15
140620e40: 5d                          	popq	%rbp
140620e41: c3                          	retq
140620e42: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140620e46: e8 35 dc ff ff              	callq	0x14061ea80 <.text+0x61da80>
140620e4b: 48 8d 0d d7 a5 3e 02        	leaq	0x23ea5d7(%rip), %rcx   # 0x142a0b429
140620e52: 4c 8d 05 1f a6 3e 02        	leaq	0x23ea61f(%rip), %r8    # 0x142a0b478
140620e59: ba 28 00 00 00              	movl	$0x28, %edx
140620e5e: e8 cd 85 2b 02              	callq	0x1428d9430 <.text+0x28d8430>
140620e63: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
140620e67: e8 14 dc ff ff              	callq	0x14061ea80 <.text+0x61da80>
140620e6c: 48 8d 0d b6 a5 3e 02        	leaq	0x23ea5b6(%rip), %rcx   # 0x142a0b429
140620e73: 4c 8d 05 fe a6 3e 02        	leaq	0x23ea6fe(%rip), %r8    # 0x142a0b578
140620e7a: ba 28 00 00 00              	movl	$0x28, %edx
140620e7f: e8 ac 85 2b 02              	callq	0x1428d9430 <.text+0x28d8430>
140620e84: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140620e88: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
140620e8f: 48 8d 05 ca a6 3e 02        	leaq	0x23ea6ca(%rip), %rax   # 0x142a0b560
140620e96: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140620e9b: 48 8d 0d ae a3 3e 02        	leaq	0x23ea3ae(%rip), %rcx   # 0x142a0b250
140620ea2: 4c 8d 0d 87 a3 3e 02        	leaq	0x23ea387(%rip), %r9    # 0x142a0b230
140620ea9: 4c 8d 85 b0 00 00 00        	leaq	0xb0(%rbp), %r8
140620eb0: ba 2b 00 00 00              	movl	$0x2b, %edx
140620eb5: e8 a6 88 2b 02              	callq	0x1428d9760 <.text+0x28d8760>
140620eba: 0f 0b                       	ud2
