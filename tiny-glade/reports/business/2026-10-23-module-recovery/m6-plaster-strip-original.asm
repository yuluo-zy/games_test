
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141228550: 85 d7                       	testl	%edx, %edi
141228552: 03 00                       	addl	(%rax), %eax
141228554: 00 01                       	addb	%al, (%rcx)
141228556: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
14122855d: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
141228564: e8 97 c6 9a ff              	callq	0x140bd4c00 <.text+0xbd3c00>
141228569: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
141228570: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
141228577: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
14122857e: 0f 28 85 50 02 00 00        	movaps	0x250(%rbp), %xmm0
141228585: 0f 28 8d 60 02 00 00        	movaps	0x260(%rbp), %xmm1
14122858c: 0f 29 8d 60 03 00 00        	movaps	%xmm1, 0x360(%rbp)
141228593: 0f 29 85 50 03 00 00        	movaps	%xmm0, 0x350(%rbp)
14122859a: 48 8b b5 00 03 00 00        	movq	0x300(%rbp), %rsi
1412285a1: 48 3b b5 f0 02 00 00        	cmpq	0x2f0(%rbp), %rsi
1412285a8: 75 13                       	jne	0x1412285bd <.text+0x12275bd>
1412285aa: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
1412285b1: 48 8d 15 50 51 9f 01        	leaq	0x19f5150(%rip), %rdx   # 0x142c1d708
1412285b8: e8 f3 a4 33 ff              	callq	0x140562ab0 <.text+0x561ab0>
1412285bd: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1412285c4: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
1412285c8: 0f 28 85 50 03 00 00        	movaps	0x350(%rbp), %xmm0
1412285cf: 0f 28 8d 60 03 00 00        	movaps	0x360(%rbp), %xmm1
1412285d6: 48 8b 95 70 03 00 00        	movq	0x370(%rbp), %rdx
1412285dd: 48 89 54 c8 20              	movq	%rdx, 0x20(%rax,%rcx,8)
1412285e2: 0f 11 4c c8 10              	movups	%xmm1, 0x10(%rax,%rcx,8)
1412285e7: 0f 11 04 c8                 	movups	%xmm0, (%rax,%rcx,8)
1412285eb: 48 ff c6                    	incq	%rsi
1412285ee: 48 89 b5 00 03 00 00        	movq	%rsi, 0x300(%rbp)
1412285f5: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
1412285fc: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141228603: 45 31 e4                    	xorl	%r12d, %r12d
141228606: 4c 8b 8d b8 03 00 00        	movq	0x3b8(%rbp), %r9
14122860d: 4c 8b ad e0 02 00 00        	movq	0x2e0(%rbp), %r13
141228614: eb 11                       	jmp	0x141228627 <.text+0x1227627>
141228616: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141228620: 4c 8b a5 f0 01 00 00        	movq	0x1f0(%rbp), %r12
141228627: 4c 3b a5 c0 02 00 00        	cmpq	0x2c0(%rbp), %r12
14122862e: 0f 83 9c 01 00 00           	jae	0x1412287d0 <.text+0x12277d0>
141228634: 4d 8d 7c 24 01              	leaq	0x1(%r12), %r15
141228639: 48 8b 85 40 03 00 00        	movq	0x340(%rbp), %rax
141228640: 4c 0f af e0                 	imulq	%rax, %r12
141228644: 4c 89 bd f0 01 00 00        	movq	%r15, 0x1f0(%rbp)
14122864b: 4c 0f af f8                 	imulq	%rax, %r15
14122864f: b0 01                       	movb	$0x1, %al
141228651: 45 31 c0                    	xorl	%r8d, %r8d
141228654: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141228660: 4c 3b 85 a0 03 00 00        	cmpq	0x3a0(%rbp), %r8
141228667: 73 b7                       	jae	0x141228620 <.text+0x1227620>
141228669: 4b 8d 0c 20                 	leaq	(%r8,%r12), %rcx
14122866d: 48 3b 8d c8 03 00 00        	cmpq	0x3c8(%rbp), %rcx
141228674: 0f 83 49 09 00 00           	jae	0x141228fc3 <.text+0x1227fc3>
14122867a: 89 c2                       	movl	%eax, %edx
14122867c: 4d 8d 70 01                 	leaq	0x1(%r8), %r14
141228680: 4b 8d 3c 38                 	leaq	(%r8,%r15), %rdi
141228684: 48 89 c8                    	movq	%rcx, %rax
141228687: 48 c1 e0 05                 	shlq	$0x5, %rax
14122868b: 41 80 7c 01 1c 01           	cmpb	$0x1, 0x1c(%r9,%rax)
141228691: 75 6d                       	jne	0x141228700 <.text+0x1227700>
141228693: 4b 8d 04 20                 	leaq	(%r8,%r12), %rax
141228697: 48 ff c0                    	incq	%rax
14122869a: 48 3b 85 c8 03 00 00        	cmpq	0x3c8(%rbp), %rax
1412286a1: 0f 83 82 0a 00 00           	jae	0x141229129 <.text+0x1228129>
1412286a7: 48 c1 e0 05                 	shlq	$0x5, %rax
1412286ab: 41 80 7c 01 1c 01           	cmpb	$0x1, 0x1c(%r9,%rax)
1412286b1: 75 4d                       	jne	0x141228700 <.text+0x1227700>
1412286b3: 48 3b bd c8 03 00 00        	cmpq	0x3c8(%rbp), %rdi
1412286ba: 0f 83 8b 0a 00 00           	jae	0x14122914b <.text+0x122814b>
1412286c0: 48 89 f8                    	movq	%rdi, %rax
1412286c3: 48 c1 e0 05                 	shlq	$0x5, %rax
1412286c7: 41 80 7c 01 1c 01           	cmpb	$0x1, 0x1c(%r9,%rax)
1412286cd: 75 31                       	jne	0x141228700 <.text+0x1227700>
1412286cf: 4b 8d 04 38                 	leaq	(%r8,%r15), %rax
1412286d3: 48 ff c0                    	incq	%rax
1412286d6: 48 3b 85 c8 03 00 00        	cmpq	0x3c8(%rbp), %rax
1412286dd: 0f 83 8a 0a 00 00           	jae	0x14122916d <.text+0x122816d>
1412286e3: 48 c1 e0 05                 	shlq	$0x5, %rax
1412286e7: 41 80 7c 01 1c 00           	cmpb	$0x0, 0x1c(%r9,%rax)
1412286ed: 4d 89 f0                    	movq	%r14, %r8
1412286f0: b0 01                       	movb	$0x1, %al
1412286f2: 0f 85 68 ff ff ff           	jne	0x141228660 <.text+0x1227660>
1412286f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141228700: 4c 8b ad 38 03 00 00        	movq	0x338(%rbp), %r13
141228707: f6 c2 01                    	testb	$0x1, %dl
14122870a: 74 77                       	je	0x141228783 <.text+0x1227783>
14122870c: 89 f8                       	movl	%edi, %eax
14122870e: 29 c8                       	subl	%ecx, %eax
141228710: 89 ce                       	movl	%ecx, %esi
141228712: 29 fe                       	subl	%edi, %esi
141228714: 48 83 bd 78 03 00 00 00     	cmpq	$0x0, 0x378(%rbp)
14122871c: 48 0f 44 f9                 	cmoveq	%rcx, %rdi
141228720: 0f 44 f0                    	cmovel	%eax, %esi
141228723: 4c 3b ad 28 03 00 00        	cmpq	0x328(%rbp), %r13
14122872a: 75 1a                       	jne	0x141228746 <.text+0x1227746>
14122872c: c6 85 d7 03 00 00 00        	movb	$0x0, 0x3d7(%rbp)
141228733: 48 8d 8d 28 03 00 00        	leaq	0x328(%rbp), %rcx
14122873a: 48 8d 15 b7 50 9f 01        	leaq	0x19f50b7(%rip), %rdx   # 0x142c1d7f8
141228741: e8 ea a4 33 ff              	callq	0x140562c30 <.text+0x561c30>
141228746: 48 8d 04 7f                 	leaq	(%rdi,%rdi,2), %rax
14122874a: 48 c1 e0 04                 	shlq	$0x4, %rax
14122874e: 48 03 85 d0 02 00 00        	addq	0x2d0(%rbp), %rax
141228755: 48 8b 8d 30 03 00 00        	movq	0x330(%rbp), %rcx
14122875c: 4c 89 ea                    	movq	%r13, %rdx
14122875f: 48 c1 e2 05                 	shlq	$0x5, %rdx
141228763: 44 0f 11 2c 11              	movups	%xmm13, (%rcx,%rdx)
141228768: 48 89 44 11 10              	movq	%rax, 0x10(%rcx,%rdx)
14122876d: c7 44 11 18 00 00 00 00     	movl	$0x0, 0x18(%rcx,%rdx)
141228775: 89 74 11 1c                 	movl	%esi, 0x1c(%rcx,%rdx)
141228779: 49 ff c5                    	incq	%r13
14122877c: 4c 89 ad 38 03 00 00        	movq	%r13, 0x338(%rbp)
141228783: 4d 85 ed                    	testq	%r13, %r13
141228786: 0f 84 56 08 00 00           	je	0x141228fe2 <.text+0x1227fe2>
14122878c: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
141228793: 49 c1 e5 05                 	shlq	$0x5, %r13
141228797: 4a 8d 0c 28                 	leaq	(%rax,%r13), %rcx
14122879b: 48 83 c1 e0                 	addq	$-0x20, %rcx
14122879f: 48 85 c9                    	testq	%rcx, %rcx
1412287a2: 0f 84 3a 08 00 00           	je	0x141228fe2 <.text+0x1227fe2>
1412287a8: 4c 01 e8                    	addq	%r13, %rax
1412287ab: 8b 48 f8                    	movl	-0x8(%rax), %ecx
1412287ae: ff c1                       	incl	%ecx
1412287b0: 89 48 f8                    	movl	%ecx, -0x8(%rax)
1412287b3: 83 f9 20                    	cmpl	$0x20, %ecx
1412287b6: 0f 93 c0                    	setae	%al
1412287b9: 4d 89 f0                    	movq	%r14, %r8
1412287bc: 4c 8b 8d b8 03 00 00        	movq	0x3b8(%rbp), %r9
1412287c3: 4c 8b ad e0 02 00 00        	movq	0x2e0(%rbp), %r13
1412287ca: e9 91 fe ff ff              	jmp	0x141228660 <.text+0x1227660>
1412287cf: 90                          	nop
1412287d0: 4c 8b 8d 38 03 00 00        	movq	0x338(%rbp), %r9
1412287d7: 48 8b b5 20 01 00 00        	movq	0x120(%rbp), %rsi
1412287de: 4c 39 ce                    	cmpq	%r9, %rsi
1412287e1: 0f 87 b2 08 00 00           	ja	0x141229099 <.text+0x1228099>
1412287e7: 4c 8b bd 00 05 00 00        	movq	0x500(%rbp), %r15
1412287ee: 4c 8b a5 c8 02 00 00        	movq	0x2c8(%rbp), %r12
1412287f5: 75 1e                       	jne	0x141228815 <.text+0x1227815>
1412287f7: 83 bd 00 02 00 00 0b        	cmpl	$0xb, 0x200(%rbp)
1412287fe: 0f 84 d8 fa ff ff           	je	0x1412282dc <.text+0x12272dc>
141228804: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
14122880b: e8 f0 e0 ff ff              	callq	0x141226900 <.text+0x1225900>
141228810: e9 c7 fa ff ff              	jmp	0x1412282dc <.text+0x12272dc>
141228815: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
14122881c: 49 c1 e1 05                 	shlq	$0x5, %r9
141228820: 49 01 c1                    	addq	%rax, %r9
141228823: 48 c1 e6 05                 	shlq	$0x5, %rsi
141228827: 48 01 c6                    	addq	%rax, %rsi
14122882a: eb 20                       	jmp	0x14122884c <.text+0x122784c>
14122882c: 0f 1f 40 00                 	nopl	(%rax)
141228830: 41 0f 13 08                 	movlps	%xmm1, (%r8)
141228834: f3 41 0f 11 40 08           	movss	%xmm0, 0x8(%r8)
14122883a: 0f 57 c0                    	xorps	%xmm0, %xmm0
14122883d: f3 0f 51 c3                 	sqrtss	%xmm3, %xmm0
141228841: f3 41 0f 11 40 0c           	movss	%xmm0, 0xc(%r8)
141228847: 4c 39 ce                    	cmpq	%r9, %rsi
14122884a: 74 ab                       	je	0x1412287f7 <.text+0x12277f7>
14122884c: 49 89 f0                    	movq	%rsi, %r8
14122884f: 48 83 c6 20                 	addq	$0x20, %rsi
