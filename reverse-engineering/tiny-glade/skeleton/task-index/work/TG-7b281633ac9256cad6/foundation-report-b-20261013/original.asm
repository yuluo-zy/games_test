
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1402552b0: 55                          	pushq	%rbp
1402552b1: 41 57                       	pushq	%r15
1402552b3: 41 56                       	pushq	%r14
1402552b5: 56                          	pushq	%rsi
1402552b6: 57                          	pushq	%rdi
1402552b7: 53                          	pushq	%rbx
1402552b8: 48 81 ec 38 05 00 00        	subq	$0x538, %rsp            # imm = 0x538
1402552bf: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1402552c7: 0f 29 b5 a0 04 00 00        	movaps	%xmm6, 0x4a0(%rbp)
1402552ce: 48 c7 85 98 04 00 00 fe ff ff ff    	movq	$-0x2, 0x498(%rbp)
1402552d9: e8 02 e3 b0 00              	callq	0x140d635e0 <.text+0xd625e0>
1402552de: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1402552e5: 89 95 28 01 00 00           	movl	%edx, 0x128(%rbp)
1402552eb: 48 8d b5 58 01 00 00        	leaq	0x158(%rbp), %rsi
1402552f2: 48 89 f1                    	movq	%rsi, %rcx
1402552f5: e8 d6 70 42 00              	callq	0x14067c3d0 <.text+0x67b3d0>
1402552fa: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
140255301: 48 bf 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rdi # imm = 0x8000000000000004
14025530b: 48 39 f8                    	cmpq	%rdi, %rax
14025530e: 0f 85 1f 01 00 00           	jne	0x140255433 <.text+0x254433>
140255314: 48 8b 05 8d 70 3d 03        	movq	0x33d708d(%rip), %rax   # 0x14362c3a8
14025531b: 48 8b 00                    	movq	(%rax), %rax
14025531e: 48 83 f8 03                 	cmpq	$0x3, %rax
140255322: 0f 82 b4 0a 00 00           	jb	0x140255ddc <.text+0x254ddc>
140255328: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
14025532f: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
140255336: e8 d5 e5 b0 00              	callq	0x140d63910 <.text+0xd62910>
14025533b: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140255342: 89 95 c8 01 00 00           	movl	%edx, 0x1c8(%rbp)
140255348: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
14025534f: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140255353: 48 8d 05 f6 51 b3 00        	leaq	0xb351f6(%rip), %rax    # 0x140d8a550 <.text+0xd89550>
14025535a: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
14025535e: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140255365: 48 8d 0d 4c e8 74 02        	leaq	0x274e84c(%rip), %rcx   # 0x1429a3bb8
14025536c: e8 0f 8a 42 00              	callq	0x14067dd80 <.text+0x67cd80>
140255371: 0f 10 00                    	movups	(%rax), %xmm0
140255374: 8b 40 10                    	movl	0x10(%rax), %eax
140255377: 48 c7 85 90 03 00 00 03 00 00 00    	movq	$0x3, 0x390(%rbp)
140255382: 48 8d 0d cf e5 74 02        	leaq	0x274e5cf(%rip), %rcx   # 0x1429a3958
140255389: 48 89 8d 98 03 00 00        	movq	%rcx, 0x398(%rbp)
140255390: 48 c7 85 a0 03 00 00 21 00 00 00    	movq	$0x21, 0x3a0(%rbp)
14025539b: 48 8d 15 06 e8 74 02        	leaq	0x274e806(%rip), %rdx   # 0x1429a3ba8
1402553a2: 48 89 95 b0 03 00 00        	movq	%rdx, 0x3b0(%rbp)
1402553a9: 48 c7 85 b8 03 00 00 01 00 00 00    	movq	$0x1, 0x3b8(%rbp)
1402553b4: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1402553b8: 48 89 95 c0 03 00 00        	movq	%rdx, 0x3c0(%rbp)
1402553bf: 48 c7 85 c8 03 00 00 01 00 00 00    	movq	$0x1, 0x3c8(%rbp)
1402553ca: 48 c7 85 d0 03 00 00 00 00 00 00    	movq	$0x0, 0x3d0(%rbp)
1402553d5: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
1402553e0: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
1402553e7: 48 c7 85 70 03 00 00 21 00 00 00    	movq	$0x21, 0x370(%rbp)
1402553f2: 48 c7 85 78 03 00 00 00 00 00 00    	movq	$0x0, 0x378(%rbp)
1402553fd: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140255404: c7 85 a8 03 00 00 01 00 00 00       	movl	$0x1, 0x3a8(%rbp)
14025540e: 89 85 ac 03 00 00           	movl	%eax, 0x3ac(%rbp)
140255414: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
14025541b: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140255422: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
140255429: e8 02 4f af 00              	callq	0x140d4a330 <.text+0xd49330>
14025542e: e9 a9 09 00 00              	jmp	0x140255ddc <.text+0x254ddc>
140255433: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
14025543d: 48 31 c8                    	xorq	%rcx, %rax
140255440: 48 83 f8 04                 	cmpq	$0x4, %rax
140255444: b9 04 00 00 00              	movl	$0x4, %ecx
140255449: 48 0f 42 c8                 	cmovbq	%rax, %rcx
14025544d: 48 8d 05 00 e8 74 02        	leaq	0x274e800(%rip), %rax   # 0x1429a3c54
140255454: 48 63 0c 88                 	movslq	(%rax,%rcx,4), %rcx
140255458: 48 01 c1                    	addq	%rax, %rcx
14025545b: ff e1                       	jmpq	*%rcx
14025545d: 48 8b 05 44 6f 3d 03        	movq	0x33d6f44(%rip), %rax   # 0x14362c3a8
140255464: 48 8b 00                    	movq	(%rax), %rax
140255467: 48 85 c0                    	testq	%rax, %rax
14025546a: 0f 84 6c 09 00 00           	je	0x140255ddc <.text+0x254ddc>
140255470: 48 8d 05 61 e4 74 02        	leaq	0x274e461(%rip), %rax   # 0x1429a38d8
140255477: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14025547e: 48 8d 05 0b 4a eb ff        	leaq	-0x14b5f5(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140255485: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14025548c: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140255493: 48 8d 0d a6 e4 74 02        	leaq	0x274e4a6(%rip), %rcx   # 0x1429a3940
14025549a: e8 e1 88 42 00              	callq	0x14067dd80 <.text+0x67cd80>
14025549f: 0f 10 00                    	movups	(%rax), %xmm0
1402554a2: 8b 40 10                    	movl	0x10(%rax), %eax
1402554a5: 48 c7 85 90 03 00 00 01 00 00 00    	movq	$0x1, 0x390(%rbp)
1402554b0: 48 8d 0d a1 e4 74 02        	leaq	0x274e4a1(%rip), %rcx   # 0x1429a3958
1402554b7: 48 89 8d 98 03 00 00        	movq	%rcx, 0x398(%rbp)
1402554be: 48 c7 85 a0 03 00 00 21 00 00 00    	movq	$0x21, 0x3a0(%rbp)
1402554c9: 48 8d 15 30 e4 74 02        	leaq	0x274e430(%rip), %rdx   # 0x1429a3900
1402554d0: 48 89 95 b0 03 00 00        	movq	%rdx, 0x3b0(%rbp)
1402554d7: 48 c7 85 b8 03 00 00 01 00 00 00    	movq	$0x1, 0x3b8(%rbp)
1402554e2: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1402554e9: 48 89 95 c0 03 00 00        	movq	%rdx, 0x3c0(%rbp)
1402554f0: 48 c7 85 c8 03 00 00 01 00 00 00    	movq	$0x1, 0x3c8(%rbp)
1402554fb: 48 c7 85 d0 03 00 00 00 00 00 00    	movq	$0x0, 0x3d0(%rbp)
140255506: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
140255511: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
140255518: 48 c7 85 70 03 00 00 21 00 00 00    	movq	$0x21, 0x370(%rbp)
140255523: 48 c7 85 78 03 00 00 00 00 00 00    	movq	$0x0, 0x378(%rbp)
14025552e: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140255535: c7 85 a8 03 00 00 01 00 00 00       	movl	$0x1, 0x3a8(%rbp)
14025553f: 89 85 ac 03 00 00           	movl	%eax, 0x3ac(%rbp)
140255545: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
14025554c: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140255553: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
14025555a: e8 d1 4d af 00              	callq	0x140d4a330 <.text+0xd49330>
14025555f: e9 78 08 00 00              	jmp	0x140255ddc <.text+0x254ddc>
140255564: 48 89 b5 48 03 00 00        	movq	%rsi, 0x348(%rbp)
14025556b: 48 8d 85 70 01 00 00        	leaq	0x170(%rbp), %rax
140255572: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
140255579: 48 8d 85 88 01 00 00        	leaq	0x188(%rbp), %rax
140255580: 48 89 85 38 03 00 00        	movq	%rax, 0x338(%rbp)
140255587: 48 8b 1d 1a 6e 3d 03        	movq	0x33d6e1a(%rip), %rbx   # 0x14362c3a8
14025558e: 48 8b 03                    	movq	(%rbx), %rax
140255591: 48 85 c0                    	testq	%rax, %rax
140255594: 0f 84 cb 00 00 00           	je	0x140255665 <.text+0x254665>
14025559a: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
1402555a1: 48 8d 0d 00 e5 74 02        	leaq	0x274e500(%rip), %rcx   # 0x1429a3aa8
1402555a8: e8 d3 87 42 00              	callq	0x14067dd80 <.text+0x67cd80>
1402555ad: 0f 10 00                    	movups	(%rax), %xmm0
1402555b0: 8b 40 10                    	movl	0x10(%rax), %eax
1402555b3: 48 c7 85 90 03 00 00 01 00 00 00    	movq	$0x1, 0x390(%rbp)
1402555be: 48 8d 0d 93 e3 74 02        	leaq	0x274e393(%rip), %rcx   # 0x1429a3958
1402555c5: 48 89 8d 98 03 00 00        	movq	%rcx, 0x398(%rbp)
1402555cc: 48 c7 85 a0 03 00 00 21 00 00 00    	movq	$0x21, 0x3a0(%rbp)
1402555d7: 48 8d 15 ba e4 74 02        	leaq	0x274e4ba(%rip), %rdx   # 0x1429a3a98
1402555de: 48 89 95 b0 03 00 00        	movq	%rdx, 0x3b0(%rbp)
1402555e5: 48 c7 85 b8 03 00 00 01 00 00 00    	movq	$0x1, 0x3b8(%rbp)
1402555f0: 48 c7 85 c0 03 00 00 08 00 00 00    	movq	$0x8, 0x3c0(%rbp)
1402555fb: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
140255606: 0f 57 c9                    	xorps	%xmm1, %xmm1
140255609: 0f 11 8d c8 03 00 00        	movups	%xmm1, 0x3c8(%rbp)
140255610: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
140255617: 48 c7 85 70 03 00 00 21 00 00 00    	movq	$0x21, 0x370(%rbp)
140255622: 48 c7 85 78 03 00 00 00 00 00 00    	movq	$0x0, 0x378(%rbp)
14025562d: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140255634: c7 85 a8 03 00 00 01 00 00 00       	movl	$0x1, 0x3a8(%rbp)
14025563e: 89 85 ac 03 00 00           	movl	%eax, 0x3ac(%rbp)
140255644: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
14025564b: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140255652: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
140255659: e8 d2 4c af 00              	callq	0x140d4a330 <.text+0xd49330>
14025565e: 48 8b b5 48 03 00 00        	movq	0x348(%rbp), %rsi
140255665: 48 83 7e 10 00              	cmpq	$0x0, 0x10(%rsi)
14025566a: 0f 84 39 01 00 00           	je	0x1402557a9 <.text+0x2547a9>
140255670: 48 8b 03                    	movq	(%rbx), %rax
140255673: 48 85 c0                    	testq	%rax, %rax
140255676: 0f 84 2d 01 00 00           	je	0x1402557a9 <.text+0x2547a9>
14025567c: 48 8d 85 48 03 00 00        	leaq	0x348(%rbp), %rax
140255683: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140255687: 48 8d 05 a2 fd 05 00        	leaq	0x5fda2(%rip), %rax     # 0x1402b5430 <.text+0x2b4430>
14025568e: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140255692: 48 c7 85 c0 01 00 00 02 00 00 00    	movq	$0x2, 0x1c0(%rbp)
14025569d: 48 c7 85 d0 01 00 00 02 00 00 00    	movq	$0x2, 0x1d0(%rbp)
1402556a8: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1402556b3: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
1402556bd: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
1402556c4: c6 85 f0 01 00 00 03        	movb	$0x3, 0x1f0(%rbp)
1402556cb: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
1402556d2: 48 8d 0d 17 e4 74 02        	leaq	0x274e417(%rip), %rcx   # 0x1429a3af0
1402556d9: e8 a2 86 42 00              	callq	0x14067dd80 <.text+0x67cd80>
1402556de: 0f 10 00                    	movups	(%rax), %xmm0
1402556e1: 8b 40 10                    	movl	0x10(%rax), %eax
1402556e4: 48 c7 85 90 03 00 00 01 00 00 00    	movq	$0x1, 0x390(%rbp)
1402556ef: 48 8d 0d 62 e2 74 02        	leaq	0x274e262(%rip), %rcx   # 0x1429a3958
1402556f6: 48 89 8d 98 03 00 00        	movq	%rcx, 0x398(%rbp)
1402556fd: 48 c7 85 a0 03 00 00 21 00 00 00    	movq	$0x21, 0x3a0(%rbp)
140255708: 48 8d 15 d1 e3 74 02        	leaq	0x274e3d1(%rip), %rdx   # 0x1429a3ae0
14025570f: 48 89 95 b0 03 00 00        	movq	%rdx, 0x3b0(%rbp)
140255716: 48 c7 85 b8 03 00 00 01 00 00 00    	movq	$0x1, 0x3b8(%rbp)
140255721: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140255725: 48 89 95 c0 03 00 00        	movq	%rdx, 0x3c0(%rbp)
14025572c: 48 c7 85 c8 03 00 00 01 00 00 00    	movq	$0x1, 0x3c8(%rbp)
140255737: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
14025573e: 48 89 95 d0 03 00 00        	movq	%rdx, 0x3d0(%rbp)
140255745: 48 c7 85 d8 03 00 00 01 00 00 00    	movq	$0x1, 0x3d8(%rbp)
140255750: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
14025575b: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
140255762: 48 c7 85 70 03 00 00 21 00 00 00    	movq	$0x21, 0x370(%rbp)
14025576d: 48 c7 85 78 03 00 00 00 00 00 00    	movq	$0x0, 0x378(%rbp)
140255778: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
14025577f: c7 85 a8 03 00 00 01 00 00 00       	movl	$0x1, 0x3a8(%rbp)
140255789: 89 85 ac 03 00 00           	movl	%eax, 0x3ac(%rbp)
14025578f: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140255796: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
14025579d: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
1402557a4: e8 87 4b af 00              	callq	0x140d4a330 <.text+0xd49330>
1402557a9: 48 8b 85 40 03 00 00        	movq	0x340(%rbp), %rax
1402557b0: 48 83 78 10 00              	cmpq	$0x0, 0x10(%rax)
1402557b5: 0f 84 39 01 00 00           	je	0x1402558f4 <.text+0x2548f4>
1402557bb: 48 8b 03                    	movq	(%rbx), %rax
1402557be: 48 85 c0                    	testq	%rax, %rax
1402557c1: 0f 84 2d 01 00 00           	je	0x1402558f4 <.text+0x2548f4>
1402557c7: 48 8d 85 40 03 00 00        	leaq	0x340(%rbp), %rax
1402557ce: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1402557d2: 48 8d 05 57 fc 05 00        	leaq	0x5fc57(%rip), %rax     # 0x1402b5430 <.text+0x2b4430>
1402557d9: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1402557dd: 48 c7 85 c0 01 00 00 02 00 00 00    	movq	$0x2, 0x1c0(%rbp)
1402557e8: 48 c7 85 d0 01 00 00 02 00 00 00    	movq	$0x2, 0x1d0(%rbp)
1402557f3: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1402557fe: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
140255808: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
14025580f: c6 85 f0 01 00 00 03        	movb	$0x3, 0x1f0(%rbp)
140255816: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
14025581d: 48 8d 0d 0c e3 74 02        	leaq	0x274e30c(%rip), %rcx   # 0x1429a3b30
140255824: e8 57 85 42 00              	callq	0x14067dd80 <.text+0x67cd80>
140255829: 0f 10 00                    	movups	(%rax), %xmm0
14025582c: 8b 40 10                    	movl	0x10(%rax), %eax
14025582f: 48 c7 85 90 03 00 00 01 00 00 00    	movq	$0x1, 0x390(%rbp)
14025583a: 48 8d 0d 17 e1 74 02        	leaq	0x274e117(%rip), %rcx   # 0x1429a3958
140255841: 48 89 8d 98 03 00 00        	movq	%rcx, 0x398(%rbp)
140255848: 48 c7 85 a0 03 00 00 21 00 00 00    	movq	$0x21, 0x3a0(%rbp)
140255853: 48 8d 15 c6 e2 74 02        	leaq	0x274e2c6(%rip), %rdx   # 0x1429a3b20
14025585a: 48 89 95 b0 03 00 00        	movq	%rdx, 0x3b0(%rbp)
140255861: 48 c7 85 b8 03 00 00 01 00 00 00    	movq	$0x1, 0x3b8(%rbp)
14025586c: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140255870: 48 89 95 c0 03 00 00        	movq	%rdx, 0x3c0(%rbp)
140255877: 48 c7 85 c8 03 00 00 01 00 00 00    	movq	$0x1, 0x3c8(%rbp)
140255882: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
140255889: 48 89 95 d0 03 00 00        	movq	%rdx, 0x3d0(%rbp)
140255890: 48 c7 85 d8 03 00 00 01 00 00 00    	movq	$0x1, 0x3d8(%rbp)
14025589b: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
1402558a6: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
1402558ad: 48 c7 85 70 03 00 00 21 00 00 00    	movq	$0x21, 0x370(%rbp)
1402558b8: 48 c7 85 78 03 00 00 00 00 00 00    	movq	$0x0, 0x378(%rbp)
1402558c3: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
1402558ca: c7 85 a8 03 00 00 01 00 00 00       	movl	$0x1, 0x3a8(%rbp)
1402558d4: 89 85 ac 03 00 00           	movl	%eax, 0x3ac(%rbp)
1402558da: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
1402558e1: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
1402558e8: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
1402558ef: e8 3c 4a af 00              	callq	0x140d4a330 <.text+0xd49330>
1402558f4: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
1402558fb: 48 83 78 10 00              	cmpq	$0x0, 0x10(%rax)
140255900: 0f 84 d6 04 00 00           	je	0x140255ddc <.text+0x254ddc>
140255906: 48 8b 03                    	movq	(%rbx), %rax
140255909: 48 85 c0                    	testq	%rax, %rax
14025590c: 0f 84 ca 04 00 00           	je	0x140255ddc <.text+0x254ddc>
140255912: 48 8d 85 38 03 00 00        	leaq	0x338(%rbp), %rax
140255919: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
14025591d: 48 8d 05 0c fb 05 00        	leaq	0x5fb0c(%rip), %rax     # 0x1402b5430 <.text+0x2b4430>
140255924: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140255928: 48 c7 85 c0 01 00 00 02 00 00 00    	movq	$0x2, 0x1c0(%rbp)
140255933: 48 c7 85 d0 01 00 00 02 00 00 00    	movq	$0x2, 0x1d0(%rbp)
14025593e: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
140255949: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
140255953: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
14025595a: c6 85 f0 01 00 00 03        	movb	$0x3, 0x1f0(%rbp)
140255961: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140255968: 48 8d 0d f9 e1 74 02        	leaq	0x274e1f9(%rip), %rcx   # 0x1429a3b68
14025596f: e8 0c 84 42 00              	callq	0x14067dd80 <.text+0x67cd80>
140255974: 0f 10 00                    	movups	(%rax), %xmm0
140255977: 8b 40 10                    	movl	0x10(%rax), %eax
14025597a: 48 c7 85 90 03 00 00 01 00 00 00    	movq	$0x1, 0x390(%rbp)
140255985: 48 8d 0d cc df 74 02        	leaq	0x274dfcc(%rip), %rcx   # 0x1429a3958
14025598c: 48 89 8d 98 03 00 00        	movq	%rcx, 0x398(%rbp)
140255993: 48 c7 85 a0 03 00 00 21 00 00 00    	movq	$0x21, 0x3a0(%rbp)
14025599e: 48 8d 15 b3 e1 74 02        	leaq	0x274e1b3(%rip), %rdx   # 0x1429a3b58
1402559a5: 48 89 95 b0 03 00 00        	movq	%rdx, 0x3b0(%rbp)
1402559ac: 48 c7 85 b8 03 00 00 01 00 00 00    	movq	$0x1, 0x3b8(%rbp)
1402559b7: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1402559bb: 48 89 95 c0 03 00 00        	movq	%rdx, 0x3c0(%rbp)
1402559c2: 48 c7 85 c8 03 00 00 01 00 00 00    	movq	$0x1, 0x3c8(%rbp)
1402559cd: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1402559d4: 48 89 95 d0 03 00 00        	movq	%rdx, 0x3d0(%rbp)
1402559db: 48 c7 85 d8 03 00 00 01 00 00 00    	movq	$0x1, 0x3d8(%rbp)
1402559e6: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
1402559f1: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
1402559f8: 48 c7 85 70 03 00 00 21 00 00 00    	movq	$0x21, 0x370(%rbp)
140255a03: 48 c7 85 78 03 00 00 00 00 00 00    	movq	$0x0, 0x378(%rbp)
140255a0e: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140255a15: c7 85 a8 03 00 00 01 00 00 00       	movl	$0x1, 0x3a8(%rbp)
140255a1f: 89 85 ac 03 00 00           	movl	%eax, 0x3ac(%rbp)
140255a25: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140255a2c: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140255a33: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
140255a3a: e8 f1 48 af 00              	callq	0x140d4a330 <.text+0xd49330>
140255a3f: e9 98 03 00 00              	jmp	0x140255ddc <.text+0x254ddc>
140255a44: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
140255a4b: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140255a4f: 48 8b 05 52 69 3d 03        	movq	0x33d6952(%rip), %rax   # 0x14362c3a8
140255a56: 48 8b 00                    	movq	(%rax), %rax
140255a59: 48 85 c0                    	testq	%rax, %rax
140255a5c: 0f 84 7a 03 00 00           	je	0x140255ddc <.text+0x254ddc>
140255a62: 48 8d 05 6f de 74 02        	leaq	0x274de6f(%rip), %rax   # 0x1429a38d8
140255a69: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140255a70: 48 8d 05 19 44 eb ff        	leaq	-0x14bbe7(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140255a77: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140255a7e: 48 8d 45 b0                 	leaq	-0x50(%rbp), %rax
140255a82: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140255a89: 48 8d 05 50 de df ff        	leaq	-0x2021b0(%rip), %rax   # 0x1400538e0 <.text+0x528e0>
140255a90: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
140255a97: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140255a9e: 48 8d 0d 53 df 74 02        	leaq	0x274df53(%rip), %rcx   # 0x1429a39f8
140255aa5: e8 d6 82 42 00              	callq	0x14067dd80 <.text+0x67cd80>
140255aaa: 0f 10 00                    	movups	(%rax), %xmm0
140255aad: 8b 40 10                    	movl	0x10(%rax), %eax
140255ab0: 48 c7 85 90 03 00 00 01 00 00 00    	movq	$0x1, 0x390(%rbp)
140255abb: 48 8d 0d 96 de 74 02        	leaq	0x274de96(%rip), %rcx   # 0x1429a3958
140255ac2: 48 89 8d 98 03 00 00        	movq	%rcx, 0x398(%rbp)
140255ac9: 48 c7 85 a0 03 00 00 21 00 00 00    	movq	$0x21, 0x3a0(%rbp)
140255ad4: 48 8d 15 fd de 74 02        	leaq	0x274defd(%rip), %rdx   # 0x1429a39d8
140255adb: 48 89 95 b0 03 00 00        	movq	%rdx, 0x3b0(%rbp)
140255ae2: 48 c7 85 b8 03 00 00 02 00 00 00    	movq	$0x2, 0x3b8(%rbp)
140255aed: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
140255af4: 48 89 95 c0 03 00 00        	movq	%rdx, 0x3c0(%rbp)
140255afb: 48 c7 85 c8 03 00 00 02 00 00 00    	movq	$0x2, 0x3c8(%rbp)
140255b06: 48 c7 85 d0 03 00 00 00 00 00 00    	movq	$0x0, 0x3d0(%rbp)
140255b11: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
140255b1c: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
140255b23: 48 c7 85 70 03 00 00 21 00 00 00    	movq	$0x21, 0x370(%rbp)
140255b2e: 48 c7 85 78 03 00 00 00 00 00 00    	movq	$0x0, 0x378(%rbp)
140255b39: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
140255b40: c7 85 a8 03 00 00 01 00 00 00       	movl	$0x1, 0x3a8(%rbp)
140255b4a: 89 85 ac 03 00 00           	movl	%eax, 0x3ac(%rbp)
140255b50: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140255b57: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140255b5e: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
140255b65: e8 c6 47 af 00              	callq	0x140d4a330 <.text+0xd49330>
140255b6a: e9 6d 02 00 00              	jmp	0x140255ddc <.text+0x254ddc>
140255b6f: 48 8d 85 60 01 00 00        	leaq	0x160(%rbp), %rax
140255b76: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
140255b7a: 48 8b 05 27 68 3d 03        	movq	0x33d6827(%rip), %rax   # 0x14362c3a8
140255b81: 48 8b 00                    	movq	(%rax), %rax
140255b84: 48 85 c0                    	testq	%rax, %rax
140255b87: 0f 84 4f 02 00 00           	je	0x140255ddc <.text+0x254ddc>
140255b8d: 48 8d 05 44 dd 74 02        	leaq	0x274dd44(%rip), %rax   # 0x1429a38d8
140255b94: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
140255b9b: 48 8d 05 ee 42 eb ff        	leaq	-0x14bd12(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140255ba2: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140255ba9: 48 8d 45 b0                 	leaq	-0x50(%rbp), %rax
140255bad: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
140255bb4: 48 8d 05 95 dd 2d 00        	leaq	0x2ddd95(%rip), %rax    # 0x140533950 <.text+0x532950>
140255bbb: 48 89 85 d8 01 00 00        	movq	%rax, 0x1d8(%rbp)
