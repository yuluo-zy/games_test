
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405073a0: 41 57                       	pushq	%r15
1405073a2: 41 56                       	pushq	%r14
1405073a4: 41 55                       	pushq	%r13
1405073a6: 41 54                       	pushq	%r12
1405073a8: 56                          	pushq	%rsi
1405073a9: 57                          	pushq	%rdi
1405073aa: 55                          	pushq	%rbp
1405073ab: 53                          	pushq	%rbx
1405073ac: 48 81 ec 28 01 00 00        	subq	$0x128, %rsp            # imm = 0x128
1405073b3: 48 89 d7                    	movq	%rdx, %rdi
1405073b6: 48 89 ce                    	movq	%rcx, %rsi
1405073b9: 48 89 d1                    	movq	%rdx, %rcx
1405073bc: e8 8f aa 17 00              	callq	0x140681e50 <.text+0x680e50>
1405073c1: 48 89 c1                    	movq	%rax, %rcx
1405073c4: e8 87 c8 17 00              	callq	0x140683c50 <.text+0x682c50>
1405073c9: c6 44 24 5f 00              	movb	$0x0, 0x5f(%rsp)
1405073ce: 48 8b 06                    	movq	(%rsi), %rax
1405073d1: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
1405073d5: 48 8b 10                    	movq	(%rax), %rdx
1405073d8: 80 3a 05                    	cmpb	$0x5, (%rdx)
1405073db: 75 3b                       	jne	0x140507418 <.text+0x506418>
1405073dd: 4c 8b 4e 10                 	movq	0x10(%rsi), %r9
1405073e1: 48 8b 5e 18                 	movq	0x18(%rsi), %rbx
1405073e5: 0f 10 46 20                 	movups	0x20(%rsi), %xmm0
1405073e9: 0f 10 4e 30                 	movups	0x30(%rsi), %xmm1
1405073ed: 0f 29 4c 24 70              	movaps	%xmm1, 0x70(%rsp)
1405073f2: 0f 29 44 24 60              	movaps	%xmm0, 0x60(%rsp)
1405073f7: 48 8d 44 24 60              	leaq	0x60(%rsp), %rax
1405073fc: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140507401: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140507406: 48 8d 54 24 5f              	leaq	0x5f(%rsp), %rdx
14050740b: 49 89 f8                    	movq	%rdi, %r8
14050740e: e8 4d fc d3 ff              	callq	0x140247060 <.text+0x246060>
140507413: e9 31 01 00 00              	jmp	0x140507549 <.text+0x506549>
140507418: c6 01 02                    	movb	$0x2, (%rcx)
14050741b: 48 8b 10                    	movq	(%rax), %rdx
14050741e: 0f b6 02                    	movzbl	(%rdx), %eax
140507421: 44 8d 40 fd                 	leal	-0x3(%rax), %r8d
140507425: 41 80 f8 07                 	cmpb	$0x7, %r8b
140507429: 77 33                       	ja	0x14050745e <.text+0x50645e>
14050742b: 0f b6 c0                    	movzbl	%al, %eax
14050742e: 83 c0 fd                    	addl	$-0x3, %eax
140507431: 4c 8d 05 20 70 4e 02        	leaq	0x24e7020(%rip), %r8    # 0x1429ee458
140507438: 49 63 04 80                 	movslq	(%r8,%rax,4), %rax
14050743c: 4c 01 c0                    	addq	%r8, %rax
14050743f: ff e0                       	jmpq	*%rax
140507441: 4c 8b 46 10                 	movq	0x10(%rsi), %r8
140507445: 48 8b 5e 18                 	movq	0x18(%rsi), %rbx
140507449: 48 8d 4c 24 5f              	leaq	0x5f(%rsp), %rcx
14050744e: 48 89 fa                    	movq	%rdi, %rdx
140507451: 49 89 d9                    	movq	%rbx, %r9
140507454: e8 77 01 d4 ff              	callq	0x1402475d0 <.text+0x2465d0>
140507459: e9 eb 00 00 00              	jmp	0x140507549 <.text+0x506549>
14050745e: 48 8b 5e 18                 	movq	0x18(%rsi), %rbx
140507462: 0f 10 46 40                 	movups	0x40(%rsi), %xmm0
140507466: 0f 10 4e 50                 	movups	0x50(%rsi), %xmm1
14050746a: 0f 29 8c 24 d0 00 00 00     	movaps	%xmm1, 0xd0(%rsp)
140507472: 0f 29 84 24 c0 00 00 00     	movaps	%xmm0, 0xc0(%rsp)
14050747a: 48 8b 46 10                 	movq	0x10(%rsi), %rax
14050747e: 0f 10 46 60                 	movups	0x60(%rsi), %xmm0
140507482: 0f 10 4e 70                 	movups	0x70(%rsi), %xmm1
140507486: 0f 29 8c 24 f0 00 00 00     	movaps	%xmm1, 0xf0(%rsp)
14050748e: 0f 29 84 24 e0 00 00 00     	movaps	%xmm0, 0xe0(%rsp)
140507496: 0f 10 86 80 00 00 00        	movups	0x80(%rsi), %xmm0
14050749d: 0f 10 8e 90 00 00 00        	movups	0x90(%rsi), %xmm1
1405074a4: 0f 29 8c 24 10 01 00 00     	movaps	%xmm1, 0x110(%rsp)
1405074ac: 0f 29 84 24 00 01 00 00     	movaps	%xmm0, 0x100(%rsp)
1405074b4: 0f 10 86 a0 00 00 00        	movups	0xa0(%rsi), %xmm0
1405074bb: 0f 10 8e b0 00 00 00        	movups	0xb0(%rsi), %xmm1
1405074c2: 0f 29 8c 24 a0 00 00 00     	movaps	%xmm1, 0xa0(%rsp)
1405074ca: 0f 29 84 24 90 00 00 00     	movaps	%xmm0, 0x90(%rsp)
1405074d2: 0f 10 86 c0 00 00 00        	movups	0xc0(%rsi), %xmm0
1405074d9: 0f 10 8e d0 00 00 00        	movups	0xd0(%rsi), %xmm1
1405074e0: 0f 29 4c 24 70              	movaps	%xmm1, 0x70(%rsp)
1405074e5: 0f 29 44 24 60              	movaps	%xmm0, 0x60(%rsp)
1405074ea: 48 8b 8e e0 00 00 00        	movq	0xe0(%rsi), %rcx
1405074f1: 48 89 4c 24 50              	movq	%rcx, 0x50(%rsp)
1405074f6: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
1405074fb: 48 89 4c 24 48              	movq	%rcx, 0x48(%rsp)
140507500: 48 8d 8c 24 90 00 00 00     	leaq	0x90(%rsp), %rcx
140507508: 48 89 4c 24 40              	movq	%rcx, 0x40(%rsp)
14050750d: 48 8d 8c 24 00 01 00 00     	leaq	0x100(%rsp), %rcx
140507515: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
14050751a: 48 8d 8c 24 e0 00 00 00     	leaq	0xe0(%rsp), %rcx
140507522: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
140507527: 48 8d 4c 24 5f              	leaq	0x5f(%rsp), %rcx
14050752c: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
140507531: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140507536: 4c 8d 84 24 c0 00 00 00     	leaq	0xc0(%rsp), %r8
14050753e: 48 89 d9                    	movq	%rbx, %rcx
140507541: 49 89 f9                    	movq	%rdi, %r9
140507544: e8 27 03 d4 ff              	callq	0x140247870 <.text+0x246870>
140507549: 48 8b 86 f8 00 00 00        	movq	0xf8(%rsi), %rax
140507550: 48 8b 08                    	movq	(%rax), %rcx
140507553: 48 8b 13                    	movq	(%rbx), %rdx
140507556: e8 d5 e0 d3 ff              	callq	0x140245630 <.text+0x244630>
14050755b: 84 c0                       	testb	%al, %al
14050755d: 0f 84 8c 00 00 00           	je	0x1405075ef <.text+0x5065ef>
140507563: 89 c3                       	movl	%eax, %ebx
140507565: 48 8d 0d d5 6c 4e 02        	leaq	0x24e6cd5(%rip), %rcx   # 0x1429ee241
14050756c: ba 19 00 00 00              	movl	$0x19, %edx
140507571: e8 0a 5f 69 00              	callq	0x140b9d480 <.text+0xb9c480>
140507576: 49 89 d1                    	movq	%rdx, %r9
140507579: 4c 8d 74 24 60              	leaq	0x60(%rsp), %r14
14050757e: 4c 89 f1                    	movq	%r14, %rcx
140507581: 48 89 fa                    	movq	%rdi, %rdx
140507584: 49 89 c0                    	movq	%rax, %r8
140507587: e8 34 4a 18 00              	callq	0x14068bfc0 <.text+0x68afc0>
14050758c: 4c 89 f1                    	movq	%r14, %rcx
14050758f: e8 bc a7 17 00              	callq	0x140681d50 <.text+0x680d50>
140507594: 48 89 c1                    	movq	%rax, %rcx
140507597: e8 e4 c6 17 00              	callq	0x140683c80 <.text+0x682c80>
14050759c: 84 c0                       	testb	%al, %al
14050759e: 74 4f                       	je	0x1405075ef <.text+0x5065ef>
1405075a0: 48 8b 86 00 01 00 00        	movq	0x100(%rsi), %rax
1405075a7: 8b 48 1c                    	movl	0x1c(%rax), %ecx
1405075aa: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1405075ae: 89 0a                       	movl	%ecx, (%rdx)
1405075b0: 48 8b 38                    	movq	(%rax), %rdi
1405075b3: 4c 8b 77 30                 	movq	0x30(%rdi), %r14
1405075b7: 4c 8b 7f 40                 	movq	0x40(%rdi), %r15
1405075bb: 4c 3b 77 20                 	cmpq	0x20(%rdi), %r14
1405075bf: 75 10                       	jne	0x1405075d1 <.text+0x5065d1>
1405075c1: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
1405075c5: 48 8d 15 04 5f 45 02        	leaq	0x2455f04(%rip), %rdx   # 0x14295d4d0
1405075cc: e8 5f b0 05 00              	callq	0x140562630 <.text+0x561630>
1405075d1: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1405075d5: 4c 89 f1                    	movq	%r14, %rcx
1405075d8: 48 c1 e1 04                 	shlq	$0x4, %rcx
1405075dc: 4c 89 3c 08                 	movq	%r15, (%rax,%rcx)
1405075e0: 88 5c 08 08                 	movb	%bl, 0x8(%rax,%rcx)
1405075e4: 49 ff c6                    	incq	%r14
1405075e7: 4c 89 77 30                 	movq	%r14, 0x30(%rdi)
1405075eb: 48 ff 47 40                 	incq	0x40(%rdi)
1405075ef: 80 7c 24 5f 00              	cmpb	$0x0, 0x5f(%rsp)
1405075f4: 74 44                       	je	0x14050763a <.text+0x50663a>
1405075f6: 48 8b 86 08 01 00 00        	movq	0x108(%rsi), %rax
1405075fd: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140507600: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140507604: 89 0a                       	movl	%ecx, (%rdx)
140507606: 48 8b 30                    	movq	(%rax), %rsi
140507609: 48 8b 7e 30                 	movq	0x30(%rsi), %rdi
14050760d: 48 8b 5e 40                 	movq	0x40(%rsi), %rbx
140507611: 48 3b 7e 20                 	cmpq	0x20(%rsi), %rdi
140507615: 75 10                       	jne	0x140507627 <.text+0x506627>
140507617: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14050761b: 48 8d 15 06 bb 4e 02        	leaq	0x24ebb06(%rip), %rdx   # 0x1429f3128
140507622: e8 49 b2 05 00              	callq	0x140562870 <.text+0x561870>
140507627: 48 8b 46 28                 	movq	0x28(%rsi), %rax
14050762b: 48 89 1c f8                 	movq	%rbx, (%rax,%rdi,8)
14050762f: 48 ff c7                    	incq	%rdi
140507632: 48 89 7e 30                 	movq	%rdi, 0x30(%rsi)
140507636: 48 ff 46 40                 	incq	0x40(%rsi)
14050763a: 48 81 c4 28 01 00 00        	addq	$0x128, %rsp            # imm = 0x128
140507641: 5b                          	popq	%rbx
140507642: 5d                          	popq	%rbp
140507643: 5f                          	popq	%rdi
140507644: 5e                          	popq	%rsi
140507645: 41 5c                       	popq	%r12
140507647: 41 5d                       	popq	%r13
140507649: 41 5e                       	popq	%r14
14050764b: 41 5f                       	popq	%r15
14050764d: c3                          	retq
14050764e: 48 8b 86 f0 00 00 00        	movq	0xf0(%rsi), %rax
140507655: c6 00 00                    	movb	$0x0, (%rax)
140507658: 48 8b 5f 08                 	movq	0x8(%rdi), %rbx
14050765c: 48 8d 05 bd 6b 4e 02        	leaq	0x24e6bbd(%rip), %rax   # 0x1429ee220
140507663: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
14050766b: 48 c7 84 24 98 00 00 00 10 00 00 00 	movq	$0x10, 0x98(%rsp)
140507677: 48 8d 8c 24 90 00 00 00     	leaq	0x90(%rsp), %rcx
14050767f: e8 8c c5 17 00              	callq	0x140683c10 <.text+0x682c10>
140507684: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140507689: 48 bd 3b 81 28 ff 69 31 22 41       	movabsq	$0x41223169ff28813b, %rbp # imm = 0x41223169FF28813B
140507693: 48 89 6c 24 68              	movq	%rbp, 0x68(%rsp)
140507698: 49 bf d9 68 a9 a2 68 72 9b a7       	movabsq	$-0x58648d975d569727, %r15 # imm = 0xA79B7268A2A968D9
1405076a2: 4c 89 7c 24 70              	movq	%r15, 0x70(%rsp)
1405076a7: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
1405076ac: e8 6f 68 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
1405076b1: 48 89 5c 24 60              	movq	%rbx, 0x60(%rsp)
1405076b6: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
1405076bb: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
1405076c0: e8 4b 62 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
1405076c5: 48 8d 5c 24 60              	leaq	0x60(%rsp), %rbx
1405076ca: 48 89 d9                    	movq	%rbx, %rcx
1405076cd: 48 89 fa                    	movq	%rdi, %rdx
1405076d0: 49 89 c0                    	movq	%rax, %r8
1405076d3: e8 d8 ea 17 00              	callq	0x1406861b0 <.text+0x6851b0>
1405076d8: 4c 8d 05 51 6b 4e 02        	leaq	0x24e6b51(%rip), %r8    # 0x1429ee230
1405076df: 48 8d 8c 24 00 01 00 00     	leaq	0x100(%rsp), %rcx
1405076e7: 41 b9 0d 00 00 00           	movl	$0xd, %r9d
1405076ed: 48 89 da                    	movq	%rbx, %rdx
1405076f0: e8 eb 47 18 00              	callq	0x14068bee0 <.text+0x68aee0>
1405076f5: 48 8b 5e 18                 	movq	0x18(%rsi), %rbx
1405076f9: 48 8d 05 3d 6b 4e 02        	leaq	0x24e6b3d(%rip), %rax   # 0x1429ee23d
140507700: 48 89 84 24 c0 00 00 00     	movq	%rax, 0xc0(%rsp)
140507708: 48 c7 84 24 c8 00 00 00 04 00 00 00 	movq	$0x4, 0xc8(%rsp)
140507714: 4c 8d b4 24 10 01 00 00     	leaq	0x110(%rsp), %r14
14050771c: 4c 89 f1                    	movq	%r14, %rcx
14050771f: e8 8c f1 17 00              	callq	0x1406868b0 <.text+0x6858b0>
140507724: 49 89 c5                    	movq	%rax, %r13
140507727: 4c 8d a4 24 c0 00 00 00     	leaq	0xc0(%rsp), %r12
14050772f: 4c 89 e1                    	movq	%r12, %rcx
140507732: e8 d9 c4 17 00              	callq	0x140683c10 <.text+0x682c10>
140507737: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
14050773f: 48 8d 8c 24 90 00 00 00     	leaq	0x90(%rsp), %rcx
140507747: e8 84 0e b8 ff              	callq	0x1400885d0 <.text+0x875d0>
14050774c: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140507751: 48 89 6c 24 68              	movq	%rbp, 0x68(%rsp)
140507756: 4c 89 7c 24 70              	movq	%r15, 0x70(%rsp)
14050775b: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
140507760: e8 bb 67 0b 00              	callq	0x1405bdf20 <.text+0x5bcf20>
140507765: 4c 89 6c 24 60              	movq	%r13, 0x60(%rsp)
14050776a: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
14050776f: 48 8d 4c 24 60              	leaq	0x60(%rsp), %rcx
140507774: e8 97 61 0b 00              	callq	0x1405bd910 <.text+0x5bc910>
140507779: 49 89 c5                    	movq	%rax, %r13
14050777c: 4c 89 f1                    	movq	%r14, %rcx
14050777f: 48 89 c2                    	movq	%rax, %rdx
140507782: e8 c9 ec 17 00              	callq	0x140686450 <.text+0x685450>
140507787: 84 c0                       	testb	%al, %al
140507789: 0f 84 9c 00 00 00           	je	0x14050782b <.text+0x50682b>
14050778f: 4c 8d 7c 24 60              	leaq	0x60(%rsp), %r15
140507794: 4c 89 f9                    	movq	%r15, %rcx
140507797: 4c 89 f2                    	movq	%r14, %rdx
14050779a: 4d 89 e8                    	movq	%r13, %r8
14050779d: e8 8e ed 17 00              	callq	0x140686530 <.text+0x685530>
1405077a2: e9 77 01 00 00              	jmp	0x14050791e <.text+0x50691e>
1405077a7: 4c 8b 46 10                 	movq	0x10(%rsi), %r8
1405077ab: 48 8b 5e 18                 	movq	0x18(%rsi), %rbx
1405077af: 48 8b 86 e8 00 00 00        	movq	0xe8(%rsi), %rax
1405077b6: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1405077bb: 48 8d 4c 24 5f              	leaq	0x5f(%rsp), %rcx
1405077c0: 48 89 fa                    	movq	%rdi, %rdx
1405077c3: 49 89 d9                    	movq	%rbx, %r9
1405077c6: e8 b5 32 d4 ff              	callq	0x14024aa80 <.text+0x249a80>
1405077cb: e9 79 fd ff ff              	jmp	0x140507549 <.text+0x506549>
1405077d0: 4c 8b 46 10                 	movq	0x10(%rsi), %r8
1405077d4: 48 8b 5e 18                 	movq	0x18(%rsi), %rbx
1405077d8: 48 8d 4c 24 5f              	leaq	0x5f(%rsp), %rcx
1405077dd: 48 89 fa                    	movq	%rdi, %rdx
1405077e0: 49 89 d9                    	movq	%rbx, %r9
1405077e3: e8 78 36 d4 ff              	callq	0x14024ae60 <.text+0x249e60>
1405077e8: e9 5c fd ff ff              	jmp	0x140507549 <.text+0x506549>
1405077ed: 48 8b 8e e0 00 00 00        	movq	0xe0(%rsi), %rcx
1405077f4: 4c 8b 4e 10                 	movq	0x10(%rsi), %r9
1405077f8: 48 8b 5e 18                 	movq	0x18(%rsi), %rbx
1405077fc: 48 8d 86 10 01 00 00        	leaq	0x110(%rsi), %rax
140507803: 48 8d 96 30 01 00 00        	leaq	0x130(%rsi), %rdx
14050780a: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
14050780f: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140507814: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140507819: 4c 8d 44 24 5f              	leaq	0x5f(%rsp), %r8
14050781e: 48 89 fa                    	movq	%rdi, %rdx
140507821: e8 fa 38 d4 ff              	callq	0x14024b120 <.text+0x24a120>
140507826: e9 1e fd ff ff              	jmp	0x140507549 <.text+0x506549>
14050782b: 48 8b 05 76 4b 12 03        	movq	0x3124b76(%rip), %rax   # 0x14362c3a8
140507832: 48 8b 00                    	movq	(%rax), %rax
140507835: 48 85 c0                    	testq	%rax, %rax
140507838: 0f 84 c0 00 00 00           	je	0x1405078fe <.text+0x5068fe>
14050783e: 48 8d 84 24 00 01 00 00     	leaq	0x100(%rsp), %rax
140507846: 48 89 84 24 e0 00 00 00     	movq	%rax, 0xe0(%rsp)
14050784e: 48 8d 05 4b c0 02 00        	leaq	0x2c04b(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
140507855: 48 89 84 24 e8 00 00 00     	movq	%rax, 0xe8(%rsp)
14050785d: 4c 89 a4 24 f0 00 00 00     	movq	%r12, 0xf0(%rsp)
140507865: 48 89 84 24 f8 00 00 00     	movq	%rax, 0xf8(%rsp)
14050786d: 48 8d 05 24 64 4e 02        	leaq	0x24e6424(%rip), %rax   # 0x1429edc98
140507874: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
140507879: 48 c7 44 24 68 02 00 00 00  	movq	$0x2, 0x68(%rsp)
140507882: 48 c7 84 24 80 00 00 00 00 00 00 00 	movq	$0x0, 0x80(%rsp)
14050788e: 48 8d 84 24 e0 00 00 00     	leaq	0xe0(%rsp), %rax
140507896: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
14050789b: 48 c7 44 24 78 02 00 00 00  	movq	$0x2, 0x78(%rsp)
1405078a4: 48 8d 0d ad 64 4e 02        	leaq	0x24e64ad(%rip), %rcx   # 0x1429edd58
1405078ab: e8 d0 64 17 00              	callq	0x14067dd80 <.text+0x67cd80>
1405078b0: 48 8d 0d 71 64 4e 02        	leaq	0x24e6471(%rip), %rcx   # 0x1429edd28
1405078b7: 48 89 8c 24 90 00 00 00     	movq	%rcx, 0x90(%rsp)
1405078bf: 48 c7 84 24 98 00 00 00 14 00 00 00 	movq	$0x14, 0x98(%rsp)
