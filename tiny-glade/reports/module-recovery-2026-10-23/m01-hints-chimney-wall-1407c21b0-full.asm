
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407c21b0: 55                          	pushq	%rbp
1407c21b1: 41 57                       	pushq	%r15
1407c21b3: 41 56                       	pushq	%r14
1407c21b5: 41 55                       	pushq	%r13
1407c21b7: 41 54                       	pushq	%r12
1407c21b9: 56                          	pushq	%rsi
1407c21ba: 57                          	pushq	%rdi
1407c21bb: 53                          	pushq	%rbx
1407c21bc: 48 81 ec 88 04 00 00        	subq	$0x488, %rsp            # imm = 0x488
1407c21c3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407c21cb: 44 0f 29 bd f0 03 00 00     	movaps	%xmm15, 0x3f0(%rbp)
1407c21d3: 44 0f 29 b5 e0 03 00 00     	movaps	%xmm14, 0x3e0(%rbp)
1407c21db: 44 0f 29 ad d0 03 00 00     	movaps	%xmm13, 0x3d0(%rbp)
1407c21e3: 44 0f 29 a5 c0 03 00 00     	movaps	%xmm12, 0x3c0(%rbp)
1407c21eb: 44 0f 29 9d b0 03 00 00     	movaps	%xmm11, 0x3b0(%rbp)
1407c21f3: 44 0f 29 95 a0 03 00 00     	movaps	%xmm10, 0x3a0(%rbp)
1407c21fb: 44 0f 29 8d 90 03 00 00     	movaps	%xmm9, 0x390(%rbp)
1407c2203: 44 0f 29 85 80 03 00 00     	movaps	%xmm8, 0x380(%rbp)
1407c220b: 0f 29 bd 70 03 00 00        	movaps	%xmm7, 0x370(%rbp)
1407c2212: 66 0f 7f b5 60 03 00 00     	movdqa	%xmm6, 0x360(%rbp)
1407c221a: 48 c7 85 58 03 00 00 fe ff ff ff    	movq	$-0x2, 0x358(%rbp)
1407c2225: 4d 89 cd                    	movq	%r9, %r13
1407c2228: 4c 89 c3                    	movq	%r8, %rbx
1407c222b: 48 89 d6                    	movq	%rdx, %rsi
1407c222e: 49 89 ce                    	movq	%rcx, %r14
1407c2231: e8 ca 47 45 00              	callq	0x140c16a00 <.text+0xc15a00>
1407c2236: 84 c0                       	testb	%al, %al
1407c2238: 74 5d                       	je	0x1407c2297 <.text+0x7c1297>
1407c223a: 41 bf 48 00 00 00           	movl	$0x48, %r15d
1407c2240: b9 48 00 00 00              	movl	$0x48, %ecx
1407c2245: e8 36 fb ff ff              	callq	0x1407c1d80 <.text+0x7c0d80>
1407c224a: 4c 8d 25 07 42 2c 02        	leaq	0x22c4207(%rip), %r12   # 0x142a86458
1407c2251: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c2255: 0f 85 c1 00 00 00           	jne	0x1407c231c <.text+0x7c131c>
1407c225b: 49 89 d1                    	movq	%rdx, %r9
1407c225e: 48 85 d2                    	testq	%rdx, %rdx
1407c2261: 74 56                       	je	0x1407c22b9 <.text+0x7c12b9>
1407c2263: 49 83 f9 48                 	cmpq	$0x48, %r9
1407c2267: 73 4e                       	jae	0x1407c22b7 <.text+0x7c12b7>
1407c2269: 48 8d 05 e8 41 2c 02        	leaq	0x22c41e8(%rip), %rax   # 0x142a86458
1407c2270: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407c2275: 7f 42                       	jg	0x1407c22b9 <.text+0x7c12b9>
1407c2277: 48 8d 05 aa 42 2c 02        	leaq	0x22c42aa(%rip), %rax   # 0x142a86528
1407c227e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c2283: 48 8d 0d ce 41 2c 02        	leaq	0x22c41ce(%rip), %rcx   # 0x142a86458
1407c228a: ba 48 00 00 00              	movl	$0x48, %edx
1407c228f: 45 31 c0                    	xorl	%r8d, %r8d
1407c2292: e8 a9 76 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c2297: 48 c7 85 70 01 00 00 00 00 00 00    	movq	$0x0, 0x170(%rbp)
1407c22a2: 48 8b 05 ef 9d e6 02        	movq	0x2e69def(%rip), %rax   # 0x14362c098
1407c22a9: 80 38 00                    	cmpb	$0x0, (%rax)
1407c22ac: 0f 85 39 02 00 00           	jne	0x1407c24eb <.text+0x7c14eb>
1407c22b2: e9 96 02 00 00              	jmp	0x1407c254d <.text+0x7c154d>
1407c22b7: 75 be                       	jne	0x1407c2277 <.text+0x7c1277>
1407c22b9: 4c 89 c9                    	movq	%r9, %rcx
1407c22bc: e8 bf fa ff ff              	callq	0x1407c1d80 <.text+0x7c0d80>
1407c22c1: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c22c5: 75 55                       	jne	0x1407c231c <.text+0x7c131c>
1407c22c7: 49 89 d0                    	movq	%rdx, %r8
1407c22ca: 49 83 c0 02                 	addq	$0x2, %r8
1407c22ce: 74 39                       	je	0x1407c2309 <.text+0x7c1309>
1407c22d0: 49 83 f8 48                 	cmpq	$0x48, %r8
1407c22d4: 73 31                       	jae	0x1407c2307 <.text+0x7c1307>
1407c22d6: 48 8d 05 7b 41 2c 02        	leaq	0x22c417b(%rip), %rax   # 0x142a86458
1407c22dd: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407c22e2: 7f 25                       	jg	0x1407c2309 <.text+0x7c1309>
1407c22e4: 48 8d 05 55 42 2c 02        	leaq	0x22c4255(%rip), %rax   # 0x142a86540
1407c22eb: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c22f0: 48 8d 0d 61 41 2c 02        	leaq	0x22c4161(%rip), %rcx   # 0x142a86458
1407c22f7: ba 48 00 00 00              	movl	$0x48, %edx
1407c22fc: 41 b9 48 00 00 00           	movl	$0x48, %r9d
1407c2302: e8 39 76 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c2307: 75 db                       	jne	0x1407c22e4 <.text+0x7c12e4>
1407c2309: 41 bf 46 00 00 00           	movl	$0x46, %r15d
1407c230f: 49 29 d7                    	subq	%rdx, %r15
1407c2312: 4c 8d 25 3f 41 2c 02        	leaq	0x22c413f(%rip), %r12   # 0x142a86458
1407c2319: 4d 01 c4                    	addq	%r8, %r12
1407c231c: 48 89 9d 30 03 00 00        	movq	%rbx, 0x330(%rbp)
1407c2323: 4c 89 eb                    	movq	%r13, %rbx
1407c2326: 4c 8d 05 80 42 2c 02        	leaq	0x22c4280(%rip), %r8    # 0x142a865ad
1407c232d: 4c 8d 2d 3c 42 2c 02        	leaq	0x22c423c(%rip), %r13   # 0x142a86570
1407c2334: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407c2340: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407c2345: 85 c0                       	testl	%eax, %eax
1407c2347: 78 17                       	js	0x1407c2360 <.text+0x7c1360>
1407c2349: 49 ff c8                    	decq	%r8
1407c234c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407c234f: 75 63                       	jne	0x1407c23b4 <.text+0x7c13b4>
1407c2351: eb 72                       	jmp	0x1407c23c5 <.text+0x7c13c5>
1407c2353: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407c2360: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407c2365: 80 f9 c0                    	cmpb	$-0x40, %cl
1407c2368: 7d 23                       	jge	0x1407c238d <.text+0x7c138d>
1407c236a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407c236f: 80 fa c0                    	cmpb	$-0x40, %dl
1407c2372: 7d 22                       	jge	0x1407c2396 <.text+0x7c1396>
1407c2374: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407c2379: 49 83 c0 fc                 	addq	$-0x4, %r8
1407c237d: 41 83 e1 07                 	andl	$0x7, %r9d
1407c2381: 41 c1 e1 06                 	shll	$0x6, %r9d
1407c2385: 83 e2 3f                    	andl	$0x3f, %edx
1407c2388: 44 09 ca                    	orl	%r9d, %edx
1407c238b: eb 10                       	jmp	0x1407c239d <.text+0x7c139d>
1407c238d: 49 83 c0 fe                 	addq	$-0x2, %r8
1407c2391: 83 e1 1f                    	andl	$0x1f, %ecx
1407c2394: eb 0f                       	jmp	0x1407c23a5 <.text+0x7c13a5>
1407c2396: 49 83 c0 fd                 	addq	$-0x3, %r8
1407c239a: 83 e2 0f                    	andl	$0xf, %edx
1407c239d: c1 e2 06                    	shll	$0x6, %edx
1407c23a0: 83 e1 3f                    	andl	$0x3f, %ecx
1407c23a3: 09 d1                       	orl	%edx, %ecx
1407c23a5: c1 e1 06                    	shll	$0x6, %ecx
1407c23a8: 24 3f                       	andb	$0x3f, %al
1407c23aa: 0f b6 c0                    	movzbl	%al, %eax
1407c23ad: 09 c8                       	orl	%ecx, %eax
1407c23af: 83 f8 5c                    	cmpl	$0x5c, %eax
1407c23b2: 74 11                       	je	0x1407c23c5 <.text+0x7c13c5>
1407c23b4: 83 f8 2f                    	cmpl	$0x2f, %eax
1407c23b7: 74 0c                       	je	0x1407c23c5 <.text+0x7c13c5>
1407c23b9: 4d 39 e8                    	cmpq	%r13, %r8
1407c23bc: 75 82                       	jne	0x1407c2340 <.text+0x7c1340>
1407c23be: bf 3d 00 00 00              	movl	$0x3d, %edi
1407c23c3: eb 4c                       	jmp	0x1407c2411 <.text+0x7c1411>
1407c23c5: 4c 8d 2d a4 41 2c 02        	leaq	0x22c41a4(%rip), %r13   # 0x142a86570
1407c23cc: 4d 29 e8                    	subq	%r13, %r8
1407c23cf: 49 ff c0                    	incq	%r8
1407c23d2: 74 32                       	je	0x1407c2406 <.text+0x7c1406>
1407c23d4: 49 83 f8 3d                 	cmpq	$0x3d, %r8
1407c23d8: 73 2a                       	jae	0x1407c2404 <.text+0x7c1404>
1407c23da: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1407c23df: 7f 25                       	jg	0x1407c2406 <.text+0x7c1406>
1407c23e1: 48 8d 05 20 41 2c 02        	leaq	0x22c4120(%rip), %rax   # 0x142a86508
1407c23e8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c23ed: 48 8d 0d 7c 41 2c 02        	leaq	0x22c417c(%rip), %rcx   # 0x142a86570
1407c23f4: ba 3d 00 00 00              	movl	$0x3d, %edx
1407c23f9: 41 b9 3d 00 00 00           	movl	$0x3d, %r9d
1407c23ff: e8 3c 75 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c2404: 75 db                       	jne	0x1407c23e1 <.text+0x7c13e1>
1407c2406: bf 3d 00 00 00              	movl	$0x3d, %edi
1407c240b: 4c 29 c7                    	subq	%r8, %rdi
1407c240e: 4d 01 c5                    	addq	%r8, %r13
1407c2411: e8 1a 5d 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c2416: 48 89 c1                    	movq	%rax, %rcx
1407c2419: 48 8b 00                    	movq	(%rax), %rax
1407c241c: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c2420: 0f 85 16 20 00 00           	jne	0x1407c443c <.text+0x7c343c>
1407c2426: 48 83 c1 08                 	addq	$0x8, %rcx
1407c242a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c242e: 0f 85 c7 20 00 00           	jne	0x1407c44fb <.text+0x7c34fb>
1407c2434: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c243b: 48 89 8d e0 02 00 00        	movq	%rcx, 0x2e0(%rbp)
1407c2442: 48 83 c1 08                 	addq	$0x8, %rcx
1407c2446: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
1407c244b: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407c2454: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407c245d: 4c 89 e2                    	movq	%r12, %rdx
1407c2460: 4d 89 f8                    	movq	%r15, %r8
1407c2463: 4d 89 e9                    	movq	%r13, %r9
1407c2466: e8 d5 48 45 00              	callq	0x140c16d40 <.text+0xc15d40>
1407c246b: 49 89 c7                    	movq	%rax, %r15
1407c246e: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
1407c2475: 48 ff 00                    	incq	(%rax)
1407c2478: 4c 89 bd 78 01 00 00        	movq	%r15, 0x178(%rbp)
1407c247f: 48 c7 85 70 01 00 00 01 00 00 00    	movq	$0x1, 0x170(%rbp)
1407c248a: 48 8b 05 07 9c e6 02        	movq	0x2e69c07(%rip), %rax   # 0x14362c098
1407c2491: 80 38 00                    	cmpb	$0x0, (%rax)
1407c2494: 49 89 dd                    	movq	%rbx, %r13
1407c2497: 48 8b 9d 30 03 00 00        	movq	0x330(%rbp), %rbx
1407c249e: 0f 84 a9 00 00 00           	je	0x1407c254d <.text+0x7c154d>
1407c24a4: e8 87 5c 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c24a9: 48 89 c1                    	movq	%rax, %rcx
1407c24ac: 48 8b 00                    	movq	(%rax), %rax
1407c24af: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c24b3: 0f 85 bc 1f 00 00           	jne	0x1407c4475 <.text+0x7c3475>
1407c24b9: 48 83 c1 08                 	addq	$0x8, %rcx
1407c24bd: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c24c1: 0f 85 34 20 00 00           	jne	0x1407c44fb <.text+0x7c34fb>
1407c24c7: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c24ce: 48 89 8d 30 03 00 00        	movq	%rcx, 0x330(%rbp)
1407c24d5: 48 83 c1 08                 	addq	$0x8, %rcx
1407c24d9: 4c 89 fa                    	movq	%r15, %rdx
1407c24dc: e8 0f 4c 45 00              	callq	0x140c170f0 <.text+0xc160f0>
1407c24e1: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
1407c24e8: 48 ff 00                    	incq	(%rax)
1407c24eb: 0f 28 b5 60 03 00 00        	movaps	0x360(%rbp), %xmm6
1407c24f2: 0f 28 bd 70 03 00 00        	movaps	0x370(%rbp), %xmm7
1407c24f9: 44 0f 28 85 80 03 00 00     	movaps	0x380(%rbp), %xmm8
1407c2501: 44 0f 28 8d 90 03 00 00     	movaps	0x390(%rbp), %xmm9
1407c2509: 44 0f 28 95 a0 03 00 00     	movaps	0x3a0(%rbp), %xmm10
1407c2511: 44 0f 28 9d b0 03 00 00     	movaps	0x3b0(%rbp), %xmm11
1407c2519: 44 0f 28 a5 c0 03 00 00     	movaps	0x3c0(%rbp), %xmm12
1407c2521: 44 0f 28 ad d0 03 00 00     	movaps	0x3d0(%rbp), %xmm13
1407c2529: 44 0f 28 b5 e0 03 00 00     	movaps	0x3e0(%rbp), %xmm14
1407c2531: 44 0f 28 bd f0 03 00 00     	movaps	0x3f0(%rbp), %xmm15
1407c2539: 48 81 c4 88 04 00 00        	addq	$0x488, %rsp            # imm = 0x488
1407c2540: 5b                          	popq	%rbx
1407c2541: 5f                          	popq	%rdi
1407c2542: 5e                          	popq	%rsi
1407c2543: 41 5c                       	popq	%r12
1407c2545: 41 5d                       	popq	%r13
1407c2547: 41 5e                       	popq	%r14
1407c2549: 41 5f                       	popq	%r15
1407c254b: 5d                          	popq	%rbp
1407c254c: c3                          	retq
1407c254d: 4c 89 f1                    	movq	%r14, %rcx
1407c2550: e8 1b 45 36 00              	callq	0x140b26a70 <.text+0xb25a70>
1407c2555: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1407c2559: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
1407c255d: 48 8b 16                    	movq	(%rsi), %rdx
1407c2560: 48 8d bd 30 02 00 00        	leaq	0x230(%rbp), %rdi
1407c2567: 48 89 f9                    	movq	%rdi, %rcx
1407c256a: e8 41 c0 25 00              	callq	0x140a1e5b0 <.text+0xa1d5b0>
1407c256f: 48 8b 85 78 04 00 00        	movq	0x478(%rbp), %rax
1407c2576: 48 8b 95 70 04 00 00        	movq	0x470(%rbp), %rdx
1407c257d: 44 0f b6 9d 30 02 00 00     	movzbl	0x230(%rbp), %r11d
1407c2585: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1407c258c: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
1407c2593: 4c 8b 95 40 02 00 00        	movq	0x240(%rbp), %r10
1407c259a: 48 8b 8d 48 02 00 00        	movq	0x248(%rbp), %rcx
1407c25a1: 48 89 8d 08 03 00 00        	movq	%rcx, 0x308(%rbp)
1407c25a8: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1407c25af: 48 89 8d 10 03 00 00        	movq	%rcx, 0x310(%rbp)
1407c25b6: 4c 8b 85 58 02 00 00        	movq	0x258(%rbp), %r8
1407c25bd: 0f b7 8d 68 02 00 00        	movzwl	0x268(%rbp), %ecx
1407c25c4: 4c 8b 0b                    	movq	(%rbx), %r9
1407c25c7: 4c 89 8d 28 01 00 00        	movq	%r9, 0x128(%rbp)
1407c25ce: 44 8b 4a 1c                 	movl	0x1c(%rdx), %r9d
1407c25d2: 44 89 8d 84 01 00 00        	movl	%r9d, 0x184(%rbp)
1407c25d9: 4c 8b 0a                    	movq	(%rdx), %r9
1407c25dc: 4c 89 4d 28                 	movq	%r9, 0x28(%rbp)
1407c25e0: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
1407c25e4: 48 89 55 30                 	movq	%rdx, 0x30(%rbp)
1407c25e8: 48 8b 00                    	movq	(%rax), %rax
1407c25eb: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
1407c25ef: 49 8b 45 00                 	movq	(%r13), %rax
1407c25f3: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1407c25f7: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1407c25fb: 48 8b 85 70 02 00 00        	movq	0x270(%rbp), %rax
1407c2602: 44 88 9d 56 03 00 00        	movb	%r11b, 0x356(%rbp)
1407c2609: 4c 89 85 20 03 00 00        	movq	%r8, 0x320(%rbp)
1407c2610: 49 83 fa 02                 	cmpq	$0x2, %r10
1407c2614: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407c261b: 0f 84 9f 01 00 00           	je	0x1407c27c0 <.text+0x7c17c0>
1407c2621: 4c 89 95 e8 01 00 00        	movq	%r10, 0x1e8(%rbp)
1407c2628: 48 8b 95 10 03 00 00        	movq	0x310(%rbp), %rdx
1407c262f: 48 85 d2                    	testq	%rdx, %rdx
1407c2632: 4c 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %r8
1407c2639: 0f 84 db 00 00 00           	je	0x1407c271a <.text+0x7c171a>
1407c263f: 48 85 c0                    	testq	%rax, %rax
1407c2642: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
1407c2649: 0f 84 2c 01 00 00           	je	0x1407c277b <.text+0x7c177b>
1407c264f: 66 85 c9                    	testw	%cx, %cx
1407c2652: 74 1c                       	je	0x1407c2670 <.text+0x7c1670>
1407c2654: 8d 41 ff                    	leal	-0x1(%rcx), %eax
1407c2657: 21 c8                       	andl	%ecx, %eax
1407c2659: 89 85 50 03 00 00           	movl	%eax, 0x350(%rbp)
1407c265f: eb 47                       	jmp	0x1407c26a8 <.text+0x7c16a8>
1407c2661: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407c2670: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1407c2674: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1407c2678: 48 81 c2 80 f6 ff ff        	addq	$-0x980, %rdx           # imm = 0xF680
1407c267f: 48 83 c0 10                 	addq	$0x10, %rax
1407c2683: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
1407c2689: 74 e5                       	je	0x1407c2670 <.text+0x7c1670>
1407c268b: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1407c2692: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407c2697: 29 c8                       	subl	%ecx, %eax
1407c2699: f7 d1                       	notl	%ecx
1407c269b: 41 89 c8                    	movl	%ecx, %r8d
1407c269e: 41 21 c0                    	andl	%eax, %r8d
1407c26a1: 44 89 85 50 03 00 00        	movl	%r8d, 0x350(%rbp)
1407c26a8: f3 0f bc c1                 	tzcntl	%ecx, %eax
1407c26ac: 48 f7 d8                    	negq	%rax
1407c26af: 48 69 c8 98 00 00 00        	imulq	$0x98, %rax, %rcx
1407c26b6: 48 89 95 10 03 00 00        	movq	%rdx, 0x310(%rbp)
1407c26bd: 48 89 ce                    	movq	%rcx, %rsi
1407c26c0: 48 01 d1                    	addq	%rdx, %rcx
1407c26c3: 48 81 c1 68 ff ff ff        	addq	$-0x98, %rcx
1407c26ca: e8 d1 9b 25 00              	callq	0x140a1c2a0 <.text+0xa1b2a0>
1407c26cf: 48 ff 8d f0 01 00 00        	decq	0x1f0(%rbp)
1407c26d6: 48 83 f8 03                 	cmpq	$0x3, %rax
1407c26da: 0f 85 cd 00 00 00           	jne	0x1407c27ad <.text+0x7c17ad>
1407c26e0: 8b 85 50 03 00 00           	movl	0x350(%rbp), %eax
1407c26e6: 89 c1                       	movl	%eax, %ecx
1407c26e8: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
1407c26ef: 4c 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %r8
1407c26f6: 4c 8b 8d 08 03 00 00        	movq	0x308(%rbp), %r9
1407c26fd: 44 0f b6 9d 56 03 00 00     	movzbl	0x356(%rbp), %r11d
1407c2705: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1407c270c: f6 85 e8 01 00 00 01        	testb	$0x1, 0x1e8(%rbp)
1407c2713: 75 23                       	jne	0x1407c2738 <.text+0x7c1738>
1407c2715: e9 83 00 00 00              	jmp	0x1407c279d <.text+0x7c179d>
1407c271a: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
1407c2721: 4c 8b 8d 08 03 00 00        	movq	0x308(%rbp), %r9
1407c2728: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1407c272f: f6 85 e8 01 00 00 01        	testb	$0x1, 0x1e8(%rbp)
1407c2736: 74 65                       	je	0x1407c279d <.text+0x7c179d>
1407c2738: b8 01 00 00 00              	movl	$0x1, %eax
1407c273d: ba 00 00 00 00              	movl	$0x0, %edx
1407c2742: 48 89 95 10 03 00 00        	movq	%rdx, 0x310(%rbp)
1407c2749: ba 00 00 00 00              	movl	$0x0, %edx
1407c274e: 4d 85 c9                    	testq	%r9, %r9
1407c2751: 0f 84 77 00 00 00           	je	0x1407c27ce <.text+0x7c17ce>
1407c2757: 4d 89 c8                    	movq	%r9, %r8
1407c275a: 89 8d 50 03 00 00           	movl	%ecx, 0x350(%rbp)
1407c2760: 48 c7 85 10 03 00 00 00 00 00 00    	movq	$0x0, 0x310(%rbp)
1407c276b: 48 c7 85 08 03 00 00 00 00 00 00    	movq	$0x0, 0x308(%rbp)
1407c2776: e9 8d 00 00 00              	jmp	0x1407c2808 <.text+0x7c1808>
1407c277b: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
1407c2786: 4c 8b 8d 08 03 00 00        	movq	0x308(%rbp), %r9
1407c278d: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1407c2794: f6 85 e8 01 00 00 01        	testb	$0x1, 0x1e8(%rbp)
1407c279b: 75 9b                       	jne	0x1407c2738 <.text+0x7c1738>
1407c279d: 48 c7 85 10 03 00 00 00 00 00 00    	movq	$0x0, 0x310(%rbp)
1407c27a8: 4c 89 ca                    	movq	%r9, %rdx
1407c27ab: eb 21                       	jmp	0x1407c27ce <.text+0x7c17ce>
1407c27ad: 49 89 f0                    	movq	%rsi, %r8
1407c27b0: 4c 03 85 10 03 00 00        	addq	0x310(%rbp), %r8
1407c27b7: 49 81 c0 70 ff ff ff        	addq	$-0x90, %r8
1407c27be: eb 4f                       	jmp	0x1407c280f <.text+0x7c180f>
1407c27c0: 48 8b 95 08 03 00 00        	movq	0x308(%rbp), %rdx
1407c27c7: 4c 8b 85 e0 01 00 00        	movq	0x1e0(%rbp), %r8
1407c27ce: 4d 85 c0                    	testq	%r8, %r8
1407c27d1: 0f 95 c0                    	setne	%al
1407c27d4: 41 84 c3                    	testb	%al, %r11b
1407c27d7: 0f 84 b2 1c 00 00           	je	0x1407c448f <.text+0x7c348f>
1407c27dd: b8 02 00 00 00              	movl	$0x2, %eax
1407c27e2: 48 89 85 e8 01 00 00        	movq	%rax, 0x1e8(%rbp)
1407c27e9: 89 c8                       	movl	%ecx, %eax
1407c27eb: 89 85 50 03 00 00           	movl	%eax, 0x350(%rbp)
1407c27f1: 48 89 95 08 03 00 00        	movq	%rdx, 0x308(%rbp)
1407c27f8: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
1407c2803: b8 02 00 00 00              	movl	$0x2, %eax
1407c2808: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407c280f: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1407c2813: 48 89 95 20 01 00 00        	movq	%rdx, 0x120(%rbp)
1407c281a: 48 89 55 60                 	movq	%rdx, 0x60(%rbp)
1407c281e: 48 89 f9                    	movq	%rdi, %rcx
1407c2821: 4c 89 45 48                 	movq	%r8, 0x48(%rbp)
1407c2825: 4c 89 c2                    	movq	%r8, %rdx
1407c2828: e8 43 88 25 00              	callq	0x140a1b070 <.text+0xa1a070>
1407c282d: 4c 8b b5 30 02 00 00        	movq	0x230(%rbp), %r14
1407c2834: 4c 8b bd 38 02 00 00        	movq	0x238(%rbp), %r15
1407c283b: 0f b7 85 48 02 00 00        	movzwl	0x248(%rbp), %eax
1407c2842: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
1407c2849: 48 89 8d e0 02 00 00        	movq	%rcx, 0x2e0(%rbp)
1407c2850: 48 8b 8d 58 02 00 00        	movq	0x258(%rbp), %rcx
1407c2857: 48 89 8d 18 01 00 00        	movq	%rcx, 0x118(%rbp)
1407c285e: 48 83 c1 50                 	addq	$0x50, %rcx
1407c2862: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
1407c2866: 48 83 bd e0 02 00 00 00     	cmpq	$0x0, 0x2e0(%rbp)
1407c286e: 0f 84 5d 1b 00 00           	je	0x1407c43d1 <.text+0x7c33d1>
1407c2874: 66 85 c0                    	testw	%ax, %ax
1407c2877: 74 17                       	je	0x1407c2890 <.text+0x7c1890>
1407c2879: 8d 48 ff                    	leal	-0x1(%rax), %ecx
1407c287c: 21 c1                       	andl	%eax, %ecx
1407c287e: 89 8d 18 03 00 00           	movl	%ecx, 0x318(%rbp)
1407c2884: 4d 85 f6                    	testq	%r14, %r14
1407c2887: 75 32                       	jne	0x1407c28bb <.text+0x7c18bb>
1407c2889: e9 43 1b 00 00              	jmp	0x1407c43d1 <.text+0x7c33d1>
1407c288e: 66 90                       	nop
1407c2890: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
1407c2895: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1407c2899: 49 83 c6 c0                 	addq	$-0x40, %r14
1407c289d: 49 83 c7 10                 	addq	$0x10, %r15
1407c28a1: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1407c28a6: 74 e8                       	je	0x1407c2890 <.text+0x7c1890>
1407c28a8: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1407c28ad: 29 c1                       	subl	%eax, %ecx
1407c28af: f7 d0                       	notl	%eax
1407c28b1: 89 c2                       	movl	%eax, %edx
1407c28b3: 21 ca                       	andl	%ecx, %edx
1407c28b5: 89 95 18 03 00 00           	movl	%edx, 0x318(%rbp)
1407c28bb: f3 0f bc c0                 	tzcntl	%eax, %eax
1407c28bf: c1 e0 02                    	shll	$0x2, %eax
1407c28c2: 4c 89 f1                    	movq	%r14, %rcx
1407c28c5: 48 29 c1                    	subq	%rax, %rcx
1407c28c8: 8b 49 fc                    	movl	-0x4(%rcx), %ecx
1407c28cb: 89 8d 30 02 00 00           	movl	%ecx, 0x230(%rbp)
1407c28d1: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
1407c28d8: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407c28dd: 0f 84 30 1b 00 00           	je	0x1407c4413 <.text+0x7c3413>
1407c28e3: 89 8d 30 03 00 00           	movl	%ecx, 0x330(%rbp)
1407c28e9: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
1407c28ed: 48 89 fa                    	movq	%rdi, %rdx
1407c28f0: e8 db e5 de ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1407c28f5: 48 ff 8d e0 02 00 00        	decq	0x2e0(%rbp)
1407c28fc: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
1407c2903: 48 8b 59 30                 	movq	0x30(%rcx), %rbx
1407c2907: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1407c290b: 49 89 cb                    	movq	%rcx, %r11
1407c290e: 49 21 c3                    	andq	%rax, %r11
1407c2911: 48 c1 e8 39                 	shrq	$0x39, %rax
1407c2915: 66 0f 6e c0                 	movd	%eax, %xmm0
1407c2919: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407c291d: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407c2922: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c2927: 48 8d 43 b0                 	leaq	-0x50(%rbx), %rax
1407c292b: 45 31 c0                    	xorl	%r8d, %r8d
1407c292e: 8b 95 30 03 00 00           	movl	0x330(%rbp), %edx
1407c2934: f3 42 0f 6f 0c 1b           	movdqu	(%rbx,%r11), %xmm1
1407c293a: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c293e: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407c2942: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407c2947: 45 85 c9                    	testl	%r9d, %r9d
1407c294a: 74 34                       	je	0x1407c2980 <.text+0x7c1980>
1407c294c: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407c2951: 4d 01 da                    	addq	%r11, %r10
1407c2954: 49 21 ca                    	andq	%rcx, %r10
1407c2957: 49 f7 da                    	negq	%r10
1407c295a: 4f 8d 24 92                 	leaq	(%r10,%r10,4), %r12
1407c295e: 49 c1 e4 04                 	shlq	$0x4, %r12
1407c2962: 42 3b 14 20                 	cmpl	(%rax,%r12), %edx
1407c2966: 74 48                       	je	0x1407c29b0 <.text+0x7c19b0>
1407c2968: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407c296c: 66 45 21 ca                 	andw	%r9w, %r10w
1407c2970: 45 89 d1                    	movl	%r10d, %r9d
1407c2973: 75 d7                       	jne	0x1407c294c <.text+0x7c194c>
1407c2975: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407c2980: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1407c2984: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407c2989: 45 85 c9                    	testl	%r9d, %r9d
1407c298c: 0f 85 81 1a 00 00           	jne	0x1407c4413 <.text+0x7c3413>
1407c2992: 4d 01 c3                    	addq	%r8, %r11
1407c2995: 49 83 c3 10                 	addq	$0x10, %r11
1407c2999: 49 83 c0 10                 	addq	$0x10, %r8
1407c299d: 49 21 cb                    	andq	%rcx, %r11
1407c29a0: eb 92                       	jmp	0x1407c2934 <.text+0x7c1934>
1407c29a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1407c29b0: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1407c29b4: e8 b7 3f 36 00              	callq	0x140b26970 <.text+0xb25970>
1407c29b9: 89 c6                       	movl	%eax, %esi
1407c29bb: 8d 4e ed                    	leal	-0x13(%rsi), %ecx
1407c29be: 8b 85 18 03 00 00           	movl	0x318(%rbp), %eax
1407c29c4: 80 f9 01                    	cmpb	$0x1, %cl
1407c29c7: 0f 87 99 fe ff ff           	ja	0x1407c2866 <.text+0x7c1866>
1407c29cd: 4a 8d 3c 23                 	leaq	(%rbx,%r12), %rdi
1407c29d1: 44 8a 6f ff                 	movb	-0x1(%rdi), %r13b
1407c29d5: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
1407c29dc: 48 8b 95 28 01 00 00        	movq	0x128(%rbp), %rdx
1407c29e3: 44 8b 85 30 03 00 00        	movl	0x330(%rbp), %r8d
1407c29ea: 4c 8d 0d bf 3b 2c 02        	leaq	0x22c3bbf(%rip), %r9    # 0x142a865b0
1407c29f1: e8 2a 80 25 00              	callq	0x140a1aa20 <.text+0xa19a20>
1407c29f6: 8b 85 84 01 00 00           	movl	0x184(%rbp), %eax
1407c29fc: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1407c2a00: 89 01                       	movl	%eax, (%rcx)
1407c2a02: 4c 8b 45 58                 	movq	0x58(%rbp), %r8
1407c2a06: 4c 8b 4d 60                 	movq	0x60(%rbp), %r9
1407c2a0a: 44 88 6c 24 20              	movb	%r13b, 0x20(%rsp)
1407c2a0f: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
1407c2a13: 8b 95 30 03 00 00           	movl	0x330(%rbp), %edx
1407c2a19: e8 92 2e 06 00              	callq	0x1408258b0 <.text+0x8248b0>
1407c2a1e: 48 89 85 28 03 00 00        	movq	%rax, 0x328(%rbp)
1407c2a25: 44 88 ad 55 03 00 00        	movb	%r13b, 0x355(%rbp)
1407c2a2c: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
1407c2a37: 48 c7 85 a8 01 00 00 04 00 00 00    	movq	$0x4, 0x1a8(%rbp)
1407c2a42: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
1407c2a4d: 8b 47 e0                    	movl	-0x20(%rdi), %eax
1407c2a50: 89 85 f8 01 00 00           	movl	%eax, 0x1f8(%rbp)
1407c2a56: 48 83 c7 e8                 	addq	$-0x18, %rdi
1407c2a5a: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2a61: 48 89 f9                    	movq	%rdi, %rcx
1407c2a64: 48 8d 15 5d 3b 2c 02        	leaq	0x22c3b5d(%rip), %rdx   # 0x142a865c8
1407c2a6b: e8 60 68 07 00              	callq	0x1408392d0 <.text+0x8382d0>
1407c2a70: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
1407c2a77: 48 8b 95 28 03 00 00        	movq	0x328(%rbp), %rdx
1407c2a7e: 48 8b 02                    	movq	(%rdx), %rax
1407c2a81: 48 0b 42 08                 	orq	0x8(%rdx), %rax
1407c2a85: 0f 85 7c 1a 00 00           	jne	0x1407c4507 <.text+0x7c3507>
1407c2a8b: 48 c7 42 08 00 00 00 00     	movq	$0x0, 0x8(%rdx)
1407c2a93: 48 c7 02 01 00 00 00        	movq	$0x1, (%rdx)
1407c2a9a: 0f 28 85 00 02 00 00        	movaps	0x200(%rbp), %xmm0
1407c2aa1: 66 0f 6f 8d 10 02 00 00     	movdqa	0x210(%rbp), %xmm1
1407c2aa9: 66 0f 6f 95 20 02 00 00     	movdqa	0x220(%rbp), %xmm2
1407c2ab1: 0f 29 42 10                 	movaps	%xmm0, 0x10(%rdx)
1407c2ab5: 66 0f 7f 4a 20              	movdqa	%xmm1, 0x20(%rdx)
1407c2aba: 66 0f 7f 52 30              	movdqa	%xmm2, 0x30(%rdx)
1407c2abf: 40 80 fe 13                 	cmpb	$0x13, %sil
1407c2ac3: 0f 85 34 05 00 00           	jne	0x1407c2ffd <.text+0x7c1ffd>
1407c2ac9: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2ad0: 48 8d 4d 58                 	leaq	0x58(%rbp), %rcx
1407c2ad4: e8 67 97 25 00              	callq	0x140a1c240 <.text+0xa1b240>
1407c2ad9: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2ae0: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
1407c2ae4: 48 89 c2                    	movq	%rax, %rdx
1407c2ae7: 4c 8d 05 f2 3a 2c 02        	leaq	0x22c3af2(%rip), %r8    # 0x142a865e0
1407c2aee: e8 fd f3 2c 00              	callq	0x140a91ef0 <.text+0xa90ef0>
1407c2af3: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
1407c2afa: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c2b01: 48 85 c9                    	testq	%rcx, %rcx
1407c2b04: 0f 84 e4 1a 00 00           	je	0x1407c45ee <.text+0x7c35ee>
1407c2b0a: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2b11: e8 0a 7b 36 00              	callq	0x140b2a620 <.text+0xb29620>
1407c2b16: 0f 28 f8                    	movaps	%xmm0, %xmm7
1407c2b19: 4a 8d 0c 23                 	leaq	(%rbx,%r12), %rcx
1407c2b1d: 48 83 c1 b8                 	addq	$-0x48, %rcx
1407c2b21: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2b28: 48 8d 15 e1 3a 2c 02        	leaq	0x22c3ae1(%rip), %rdx   # 0x142a86610
1407c2b2f: e8 2c 07 07 00              	callq	0x140833260 <.text+0x832260>
1407c2b34: 48 8b 95 40 03 00 00        	movq	0x340(%rbp), %rdx
1407c2b3b: 48 8b 4a 48                 	movq	0x48(%rdx), %rcx
1407c2b3f: 48 83 f9 01                 	cmpq	$0x1, %rcx
1407c2b43: 0f 84 db 18 00 00           	je	0x1407c4424 <.text+0x7c3424>
1407c2b49: 48 85 c9                    	testq	%rcx, %rcx
1407c2b4c: 0f 84 04 1a 00 00           	je	0x1407c4556 <.text+0x7c3556>
1407c2b52: f3 44 0f 10 00              	movss	(%rax), %xmm8
1407c2b57: f3 44 0f 10 70 04           	movss	0x4(%rax), %xmm14
1407c2b5d: 48 8b 42 40                 	movq	0x40(%rdx), %rax
1407c2b61: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1407c2b68: 48 89 8d 38 02 00 00        	movq	%rcx, 0x238(%rbp)
1407c2b6f: 48 8d 05 ca 40 a4 00        	leaq	0xa440ca(%rip), %rax    # 0x141206c40 <.text+0x1205c40>
1407c2b76: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1407c2b7d: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1407c2b81: f3 0f 5f 0d 07 db 19 02     	maxss	0x219db07(%rip), %xmm1  # 0x142960690
1407c2b89: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407c2b8c: 48 8d 85 40 02 00 00        	leaq	0x240(%rbp), %rax
1407c2b93: 0f 11 40 08                 	movups	%xmm0, 0x8(%rax)
1407c2b97: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2b9e: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407c2ba5: e8 36 6d ac 00              	callq	0x1412898e0 <.text+0x12888e0>
1407c2baa: 8b 8d 50 02 00 00           	movl	0x250(%rbp), %ecx
1407c2bb0: 4c 8b 85 40 03 00 00        	movq	0x340(%rbp), %r8
1407c2bb7: 49 8b 50 48                 	movq	0x48(%r8), %rdx
1407c2bbb: 48 39 ca                    	cmpq	%rcx, %rdx
1407c2bbe: 0f 86 da 1a 00 00           	jbe	0x1407c469e <.text+0x7c369e>
1407c2bc4: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1407c2bc8: 48 39 d0                    	cmpq	%rdx, %rax
1407c2bcb: 0f 83 e5 1a 00 00           	jae	0x1407c46b6 <.text+0x7c36b6>
1407c2bd1: f3 0f 10 b5 54 02 00 00     	movss	0x254(%rbp), %xmm6
1407c2bd9: 49 8b 50 40                 	movq	0x40(%r8), %rdx
1407c2bdd: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1407c2be1: f3 44 0f 10 0d 1a 2d 16 02  	movss	0x2162d1a(%rip), %xmm9  # 0x142925904
1407c2bea: f3 44 0f 5c ce              	subss	%xmm6, %xmm9
1407c2bef: f3 44 0f 59 4c 8a 04        	mulss	0x4(%rdx,%rcx,4), %xmm9
1407c2bf6: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407c2bfa: f3 0f 59 74 82 04           	mulss	0x4(%rdx,%rax,4), %xmm6
1407c2c00: f3 41 0f 5c fe              	subss	%xmm14, %xmm7
1407c2c05: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
1407c2c09: f3 41 0f 5f fc              	maxss	%xmm12, %xmm7
1407c2c0e: f3 0f 58 3d 62 a4 20 02     	addss	0x220a462(%rip), %xmm7  # 0x1429cd078
1407c2c16: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407c2c1d: f3 44 0f 10 38              	movss	(%rax), %xmm15
1407c2c22: f3 44 0f 58 ff              	addss	%xmm7, %xmm15
1407c2c27: c7 85 1c 02 00 00 9a 99 59 3f       	movl	$0x3f59999a, 0x21c(%rbp) # imm = 0x3F59999A
1407c2c31: f3 44 0f 11 bd 20 02 00 00  	movss	%xmm15, 0x220(%rbp)
1407c2c3a: c7 85 24 02 00 00 9a 99 59 3f       	movl	$0x3f59999a, 0x224(%rbp) # imm = 0x3F59999A
1407c2c44: 0f 28 95 00 02 00 00        	movaps	0x200(%rbp), %xmm2
1407c2c4b: 0f 28 c2                    	movaps	%xmm2, %xmm0
1407c2c4e: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407c2c51: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407c2c55: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407c2c59: 0f 28 d8                    	movaps	%xmm0, %xmm3
1407c2c5c: 66 0f 15 d8                 	unpckhpd	%xmm0, %xmm3            # xmm3 = xmm3[1],xmm0[1]
1407c2c60: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1407c2c64: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
1407c2c68: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407c2c6c: 0f 5c c3                    	subps	%xmm3, %xmm0
1407c2c6f: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407c2c72: 44 0f 28 15 76 36 2c 02     	movaps	0x22c3676(%rip), %xmm10 # 0x142a862f0
1407c2c7a: 41 0f 59 ca                 	mulps	%xmm10, %xmm1
1407c2c7e: 0f 28 ea                    	movaps	%xmm2, %xmm5
1407c2c81: 41 0f 59 ea                 	mulps	%xmm10, %xmm5
1407c2c85: f3 0f 16 dd                 	movshdup	%xmm5, %xmm3            # xmm3 = xmm5[1,1,3,3]
1407c2c89: f3 0f 58 dd                 	addss	%xmm5, %xmm3
1407c2c8d: 0f 12 ed                    	movhlps	%xmm5, %xmm5            # xmm5 = xmm5[1,1]
1407c2c90: f3 0f 58 eb                 	addss	%xmm3, %xmm5
1407c2c94: f3 0f 58 ed                 	addss	%xmm5, %xmm5
1407c2c98: 0f c6 ed 00                 	shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
1407c2c9c: 0f 59 ea                    	mulps	%xmm2, %xmm5
1407c2c9f: 0f 58 e9                    	addps	%xmm1, %xmm5
1407c2ca2: 0f 28 ca                    	movaps	%xmm2, %xmm1
1407c2ca5: 0f c6 ca d2                 	shufps	$0xd2, %xmm2, %xmm1     # xmm1 = xmm1[2,0],xmm2[1,3]
1407c2ca9: 0f 28 e1                    	movaps	%xmm1, %xmm4
1407c2cac: 41 0f 59 e2                 	mulps	%xmm10, %xmm4
1407c2cb0: 0f 28 da                    	movaps	%xmm2, %xmm3
1407c2cb3: 0f 59 1d 46 36 2c 02        	mulps	0x22c3646(%rip), %xmm3  # 0x142a86300
1407c2cba: 0f 58 dc                    	addps	%xmm4, %xmm3
1407c2cbd: 0f c6 db d2                 	shufps	$0xd2, %xmm3, %xmm3     # xmm3 = xmm3[2,0,1,3]
1407c2cc1: 0f 28 e2                    	movaps	%xmm2, %xmm4
1407c2cc4: 0f 58 e2                    	addps	%xmm2, %xmm4
1407c2cc7: 0f c6 e4 ff                 	shufps	$0xff, %xmm4, %xmm4     # xmm4 = xmm4[3,3,3,3]
1407c2ccb: 0f 59 dc                    	mulps	%xmm4, %xmm3
1407c2cce: 0f 58 dd                    	addps	%xmm5, %xmm3
1407c2cd1: 0f 28 ea                    	movaps	%xmm2, %xmm5
1407c2cd4: 44 0f 28 1d 34 36 2c 02     	movaps	0x22c3634(%rip), %xmm11 # 0x142a86310
1407c2cdc: 41 0f 59 eb                 	mulps	%xmm11, %xmm5
1407c2ce0: f3 44 0f 16 d5              	movshdup	%xmm5, %xmm10           # xmm10 = xmm5[1,1,3,3]
1407c2ce5: f3 44 0f 58 d5              	addss	%xmm5, %xmm10
1407c2cea: 0f 12 ed                    	movhlps	%xmm5, %xmm5            # xmm5 = xmm5[1,1]
1407c2ced: f3 41 0f 58 ea              	addss	%xmm10, %xmm5
1407c2cf2: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
1407c2cf6: f3 0f 58 ed                 	addss	%xmm5, %xmm5
1407c2cfa: 0f c6 ed 00                 	shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
1407c2cfe: 0f 59 ea                    	mulps	%xmm2, %xmm5
1407c2d01: 0f 58 e8                    	addps	%xmm0, %xmm5
1407c2d04: 41 0f 59 cb                 	mulps	%xmm11, %xmm1
1407c2d08: 0f 59 15 11 36 2c 02        	mulps	0x22c3611(%rip), %xmm2  # 0x142a86320
1407c2d0f: 0f 58 d1                    	addps	%xmm1, %xmm2
1407c2d12: 45 0f 28 df                 	movaps	%xmm15, %xmm11
1407c2d16: f3 44 0f 59 1d 51 2b 16 02  	mulss	0x2162b51(%rip), %xmm11 # 0x142925870
1407c2d1f: 0f c6 d2 d2                 	shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
1407c2d23: 0f 59 d4                    	mulps	%xmm4, %xmm2
1407c2d26: f3 0f 10 8d 14 02 00 00     	movss	0x214(%rbp), %xmm1
1407c2d2e: f3 41 0f 58 cb              	addss	%xmm11, %xmm1
1407c2d33: 0f 58 d5                    	addps	%xmm5, %xmm2
1407c2d36: 0f 28 85 10 02 00 00        	movaps	0x210(%rbp), %xmm0
1407c2d3d: 0f 16 c8                    	movlhps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0]
1407c2d40: 0f c6 c8 e2                 	shufps	$0xe2, %xmm0, %xmm1     # xmm1 = xmm1[2,0],xmm0[2,3]
1407c2d44: 0f 58 cb                    	addps	%xmm3, %xmm1
1407c2d47: 0f 12 db                    	movhlps	%xmm3, %xmm3            # xmm3 = xmm3[1,1]
1407c2d4a: f3 0f 58 9d 18 02 00 00     	addss	0x218(%rbp), %xmm3
1407c2d52: 0f 58 ca                    	addps	%xmm2, %xmm1
1407c2d55: 0f 13 8d 10 02 00 00        	movlps	%xmm1, 0x210(%rbp)
1407c2d5c: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407c2d5f: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407c2d63: f3 0f 11 95 18 02 00 00     	movss	%xmm2, 0x218(%rbp)
1407c2d6b: f3 41 0f 10 80 80 02 00 00  	movss	0x280(%r8), %xmm0
1407c2d74: 41 0f 2e c4                 	ucomiss	%xmm12, %xmm0
1407c2d78: 76 5e                       	jbe	0x1407c2dd8 <.text+0x7c1dd8>
1407c2d7a: 49 8d 48 38                 	leaq	0x38(%r8), %rcx
1407c2d7e: 49 8b 78 78                 	movq	0x78(%r8), %rdi
1407c2d82: 49 8b b0 80 00 00 00        	movq	0x80(%r8), %rsi
1407c2d89: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2d90: e8 bb 76 aa 00              	callq	0x14126a450 <.text+0x1269450>
1407c2d95: 89 c1                       	movl	%eax, %ecx
1407c2d97: 48 39 ce                    	cmpq	%rcx, %rsi
1407c2d9a: 0f 86 4c 19 00 00           	jbe	0x1407c46ec <.text+0x7c36ec>
1407c2da0: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1407c2da4: 48 39 f0                    	cmpq	%rsi, %rax
1407c2da7: 0f 83 57 19 00 00           	jae	0x1407c4704 <.text+0x7c3704>
1407c2dad: f3 0f 10 0c 8f              	movss	(%rdi,%rcx,4), %xmm1
1407c2db2: f3 0f 10 54 8f 04           	movss	0x4(%rdi,%rcx,4), %xmm2
1407c2db8: f3 0f 5c d1                 	subss	%xmm1, %xmm2
1407c2dbc: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1407c2dc0: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407c2dc4: 0f 2e 15 a5 2a 16 02        	ucomiss	0x2162aa5(%rip), %xmm2  # 0x142925870
1407c2dcb: 4c 8b 85 40 03 00 00        	movq	0x340(%rbp), %r8
1407c2dd2: 0f 87 93 0b 00 00           	ja	0x1407c396b <.text+0x7c296b>
1407c2dd8: f3 44 0f 58 ce              	addss	%xmm6, %xmm9
1407c2ddd: 45 0f 28 e6                 	movaps	%xmm14, %xmm12
1407c2de1: f3 45 0f 5c e1              	subss	%xmm9, %xmm12
1407c2de6: f3 0f 10 05 9a 2b 16 02     	movss	0x2162b9a(%rip), %xmm0  # 0x142925988
1407c2dee: 41 0f 2e c4                 	ucomiss	%xmm12, %xmm0
1407c2df2: 0f 86 6f 0b 00 00           	jbe	0x1407c3967 <.text+0x7c2967>
1407c2df8: f3 44 0f 5f 25 cf 2a 16 02  	maxss	0x2162acf(%rip), %xmm12 # 0x1429258d0
1407c2e01: 0f 28 85 00 02 00 00        	movaps	0x200(%rbp), %xmm0
1407c2e08: 66 0f 6f 8d 10 02 00 00     	movdqa	0x210(%rbp), %xmm1
1407c2e10: 0f 28 95 20 02 00 00        	movaps	0x220(%rbp), %xmm2
1407c2e17: 66 0f 7f 8d a0 00 00 00     	movdqa	%xmm1, 0xa0(%rbp)
1407c2e1f: 0f 29 95 b0 00 00 00        	movaps	%xmm2, 0xb0(%rbp)
1407c2e26: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
1407c2e2d: c7 85 ac 00 00 00 00 00 e0 3f       	movl	$0x3fe00000, 0xac(%rbp) # imm = 0x3FE00000
1407c2e37: c7 85 b4 00 00 00 9a 99 99 3f       	movl	$0x3f99999a, 0xb4(%rbp) # imm = 0x3F99999A
1407c2e41: f3 44 0f 11 a5 b0 00 00 00  	movss	%xmm12, 0xb0(%rbp)
1407c2e4a: f3 0f 10 85 a4 00 00 00     	movss	0xa4(%rbp), %xmm0
1407c2e52: f3 41 0f 5c c3              	subss	%xmm11, %xmm0
1407c2e57: 45 0f 28 d4                 	movaps	%xmm12, %xmm10
1407c2e5b: f3 44 0f 59 15 0c 2a 16 02  	mulss	0x2162a0c(%rip), %xmm10 # 0x142925870
1407c2e64: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
1407c2e69: f3 0f 11 85 a4 00 00 00     	movss	%xmm0, 0xa4(%rbp)
1407c2e71: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2e78: 4c 89 c1                    	movq	%r8, %rcx
1407c2e7b: e8 c0 78 36 00              	callq	0x140b2a740 <.text+0xb29740>
1407c2e80: 89 85 30 02 00 00           	movl	%eax, 0x230(%rbp)
1407c2e86: f3 0f 11 85 34 02 00 00     	movss	%xmm0, 0x234(%rbp)
1407c2e8e: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2e95: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407c2e9c: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1407c2ea0: e8 7b f0 4c 00              	callq	0x140c91f20 <.text+0xc90f20>
1407c2ea5: f3 0f 11 85 dc 02 00 00     	movss	%xmm0, 0x2dc(%rbp)
1407c2ead: f3 0f 10 85 ac 00 00 00     	movss	0xac(%rbp), %xmm0
1407c2eb5: f3 0f 11 85 1c 03 00 00     	movss	%xmm0, 0x31c(%rbp)
1407c2ebd: f3 44 0f 10 ad b0 00 00 00  	movss	0xb0(%rbp), %xmm13
1407c2ec6: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2ecd: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c2ed4: e8 67 78 36 00              	callq	0x140b2a740 <.text+0xb29740>
1407c2ed9: 89 85 88 01 00 00           	movl	%eax, 0x188(%rbp)
1407c2edf: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c2ee6: 48 8b 81 98 00 00 00        	movq	0x98(%rcx), %rax
1407c2eed: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c2ef1: 0f 84 66 15 00 00           	je	0x1407c445d <.text+0x7c345d>
1407c2ef7: 48 85 c0                    	testq	%rax, %rax
1407c2efa: 0f 84 7a 16 00 00           	je	0x1407c457a <.text+0x7c357a>
1407c2f00: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1407c2f04: 48 8b 89 90 00 00 00        	movq	0x90(%rcx), %rcx
1407c2f0b: 48 89 8d 30 02 00 00        	movq	%rcx, 0x230(%rbp)
1407c2f12: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
1407c2f19: 48 8d 05 20 3d a4 00        	leaq	0xa43d20(%rip), %rax    # 0x141206c40 <.text+0x1205c40>
1407c2f20: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1407c2f27: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407c2f2a: 48 8d 85 40 02 00 00        	leaq	0x240(%rbp), %rax
1407c2f31: 0f 11 40 08                 	movups	%xmm0, 0x8(%rax)
1407c2f35: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c2f3c: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407c2f43: 66 0f 6e 8d dc 02 00 00     	movd	0x2dc(%rbp), %xmm1
1407c2f4b: e8 90 69 ac 00              	callq	0x1412898e0 <.text+0x12888e0>
1407c2f50: 8b 8d 50 02 00 00           	movl	0x250(%rbp), %ecx
1407c2f56: 4c 8b 95 40 03 00 00        	movq	0x340(%rbp), %r10
1407c2f5d: 49 8b 92 e8 00 00 00        	movq	0xe8(%r10), %rdx
1407c2f64: 48 39 ca                    	cmpq	%rcx, %rdx
1407c2f67: 0f 86 b2 17 00 00           	jbe	0x1407c471f <.text+0x7c371f>
1407c2f6d: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1407c2f71: 48 39 d0                    	cmpq	%rdx, %rax
1407c2f74: 4c 8b 45 48                 	movq	0x48(%rbp), %r8
1407c2f78: 44 8b 8d 30 03 00 00        	movl	0x330(%rbp), %r9d
1407c2f7f: 0f 83 af 17 00 00           	jae	0x1407c4734 <.text+0x7c3734>
1407c2f85: f3 45 0f 5c f2              	subss	%xmm10, %xmm14
1407c2f8a: 4c 89 d0                    	movq	%r10, %rax
1407c2f8d: 48 05 88 00 00 00           	addq	$0x88, %rax
1407c2f93: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
1407c2f97: 49 8b 82 e0 00 00 00        	movq	0xe0(%r10), %rax
1407c2f9e: f3 0f 10 04 88              	movss	(%rax,%rcx,4), %xmm0
1407c2fa3: f3 0f 10 4c 88 04           	movss	0x4(%rax,%rcx,4), %xmm1
1407c2fa9: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1407c2fad: f3 0f 59 8d 54 02 00 00     	mulss	0x254(%rbp), %xmm1
1407c2fb5: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407c2fb9: f3 0f 11 8d 8c 01 00 00     	movss	%xmm1, 0x18c(%rbp)
1407c2fc1: 49 8b 78 30                 	movq	0x30(%r8), %rdi
1407c2fc5: 49 8b 70 48                 	movq	0x48(%r8), %rsi
1407c2fc9: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
1407c2fcd: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1407c2fd1: f7 d0                       	notl	%eax
1407c2fd3: 48 8d 5f 10                 	leaq	0x10(%rdi), %rbx
1407c2fd7: f3 0f 10 05 91 28 16 02     	movss	0x2162891(%rip), %xmm0  # 0x142925870
1407c2fdf: f3 44 0f 59 e8              	mulss	%xmm0, %xmm13
1407c2fe4: f3 0f 10 8d 1c 03 00 00     	movss	0x31c(%rbp), %xmm1
1407c2fec: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407c2ff0: f3 0f 11 8d 1c 03 00 00     	movss	%xmm1, 0x31c(%rbp)
1407c2ff8: e9 c5 07 00 00              	jmp	0x1407c37c2 <.text+0x7c27c2>
1407c2ffd: 0f 28 bd 00 02 00 00        	movaps	0x200(%rbp), %xmm7
1407c3004: f2 0f 10 b5 10 02 00 00     	movsd	0x210(%rbp), %xmm6
1407c300c: 48 8d 8d 1c 02 00 00        	leaq	0x21c(%rbp), %rcx
1407c3013: 0f 10 01                    	movups	(%rcx), %xmm0
1407c3016: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1407c301d: 8b 41 10                    	movl	0x10(%rcx), %eax
1407c3020: 44 0f 28 d7                 	movaps	%xmm7, %xmm10
1407c3024: 44 0f 59 d7                 	mulps	%xmm7, %xmm10
1407c3028: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
1407c302d: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
1407c3032: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407c3036: 66 41 0f 15 ca              	unpckhpd	%xmm10, %xmm1           # xmm1 = xmm1[1],xmm10[1]
1407c303b: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407c303f: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407c3043: 45 0f c6 d2 ff              	shufps	$0xff, %xmm10, %xmm10   # xmm10 = xmm10[3,3,3,3]
1407c3048: 44 0f 5c d1                 	subps	%xmm1, %xmm10
1407c304c: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407c3050: 0f 28 1d 79 dc 2b 02        	movaps	0x22bdc79(%rip), %xmm3  # 0x142a80cd0
1407c3057: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407c305a: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407c305d: 0f 59 c3                    	mulps	%xmm3, %xmm0
1407c3060: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407c3064: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407c3068: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407c306b: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407c306f: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407c3073: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c3077: 0f 59 c7                    	mulps	%xmm7, %xmm0
1407c307a: 0f 58 c1                    	addps	%xmm1, %xmm0
1407c307d: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407c3080: 0f c6 cf d2                 	shufps	$0xd2, %xmm7, %xmm1     # xmm1 = xmm1[2,0],xmm7[1,3]
1407c3084: 0f 29 8d 40 03 00 00        	movaps	%xmm1, 0x340(%rbp)
1407c308b: 0f 59 cb                    	mulps	%xmm3, %xmm1
1407c308e: 44 0f 28 ef                 	movaps	%xmm7, %xmm13
1407c3092: 44 0f 59 2d 26 32 2c 02     	mulps	0x22c3226(%rip), %xmm13 # 0x142a862c0
1407c309a: 44 0f 58 e9                 	addps	%xmm1, %xmm13
1407c309e: 45 0f c6 ed d2              	shufps	$0xd2, %xmm13, %xmm13   # xmm13 = xmm13[2,0,1,3]
1407c30a3: 44 0f 28 e7                 	movaps	%xmm7, %xmm12
1407c30a7: 44 0f 58 e7                 	addps	%xmm7, %xmm12
1407c30ab: 45 0f 28 cc                 	movaps	%xmm12, %xmm9
1407c30af: 45 0f c6 cc ff              	shufps	$0xff, %xmm12, %xmm9    # xmm9 = xmm9[3,3],xmm12[3,3]
1407c30b4: 45 0f 59 e9                 	mulps	%xmm9, %xmm13
1407c30b8: 44 0f 58 e8                 	addps	%xmm0, %xmm13
1407c30bc: 45 0f 28 dd                 	movaps	%xmm13, %xmm11
1407c30c0: 66 45 0f 15 dd              	unpckhpd	%xmm13, %xmm11          # xmm11 = xmm11[1],xmm13[1]
1407c30c5: f3 44 0f 58 9d 18 02 00 00  	addss	0x218(%rbp), %xmm11
1407c30ce: 89 85 60 01 00 00           	movl	%eax, 0x160(%rbp)
1407c30d4: 48 89 d7                    	movq	%rdx, %rdi
1407c30d7: 48 81 c7 b0 00 00 00        	addq	$0xb0, %rdi
1407c30de: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
1407c30e2: 66 0f 7f 85 30 02 00 00     	movdqa	%xmm0, 0x230(%rbp)
1407c30ea: 8b 41 10                    	movl	0x10(%rcx), %eax
1407c30ed: 89 85 40 02 00 00           	movl	%eax, 0x240(%rbp)
1407c30f3: 48 8d 85 93 00 00 00        	leaq	0x93(%rbp), %rax
1407c30fa: c7 40 08 00 00 00 00        	movl	$0x0, 0x8(%rax)
1407c3101: 48 c7 00 00 00 00 00        	movq	$0x0, (%rax)
1407c3108: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1407c310f: 48 8b 8d 97 00 00 00        	movq	0x97(%rbp), %rcx
1407c3116: 48 89 8d ef 02 00 00        	movq	%rcx, 0x2ef(%rbp)
1407c311d: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
1407c3124: 48 8b b2 c0 00 00 00        	movq	0xc0(%rdx), %rsi
1407c312b: 48 3b b2 b0 00 00 00        	cmpq	0xb0(%rdx), %rsi
1407c3132: 75 16                       	jne	0x1407c314a <.text+0x7c214a>
1407c3134: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c313b: 48 89 f9                    	movq	%rdi, %rcx
1407c313e: 48 8d 15 a3 35 2c 02        	leaq	0x22c35a3(%rip), %rdx   # 0x142a866e8
1407c3145: e8 66 82 06 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407c314a: 41 0f 58 f5                 	addps	%xmm13, %xmm6
1407c314e: 4c 8b 8d 28 03 00 00        	movq	0x328(%rbp), %r9
1407c3155: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407c315c: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407c3163: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407c316b: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407c3174: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407c317d: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407c3185: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407c318d: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407c3198: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407c31a3: 48 8d 15 26 35 2c 02        	leaq	0x22c3526(%rip), %rdx   # 0x142a866d0
1407c31aa: 48 89 94 08 a0 00 00 00     	movq	%rdx, 0xa0(%rax,%rcx)
1407c31b2: 48 c7 84 08 a8 00 00 00 14 00 00 00 	movq	$0x14, 0xa8(%rax,%rcx)
1407c31be: 48 8d 15 3f e0 45 02        	leaq	0x245e03f(%rip), %rdx   # 0x142c21204
1407c31c5: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407c31cd: 48 c7 84 08 b8 00 00 00 17 00 00 00 	movq	$0x17, 0xb8(%rax,%rcx)
1407c31d9: 0f 29 bc 08 c0 00 00 00     	movaps	%xmm7, 0xc0(%rax,%rcx)
1407c31e1: 0f 13 b4 08 d0 00 00 00     	movlps	%xmm6, 0xd0(%rax,%rcx)
1407c31e9: f3 44 0f 11 9c 08 d8 00 00 00       	movss	%xmm11, 0xd8(%rax,%rcx)
1407c31f3: 66 0f 6f 85 30 02 00 00     	movdqa	0x230(%rbp), %xmm0
1407c31fb: f3 0f 7f 84 08 dc 00 00 00  	movdqu	%xmm0, 0xdc(%rax,%rcx)
1407c3204: 8b 95 40 02 00 00           	movl	0x240(%rbp), %edx
1407c320a: 89 94 08 ec 00 00 00        	movl	%edx, 0xec(%rax,%rcx)
1407c3211: c7 84 08 f0 00 00 00 00 00 00 00    	movl	$0x0, 0xf0(%rax,%rcx)
1407c321c: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407c3227: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407c322f: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407c323a: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407c3242: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
1407c3249: 4c 8b 85 ef 02 00 00        	movq	0x2ef(%rbp), %r8
1407c3250: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407c3258: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407c3260: c6 84 08 4c 01 00 00 02     	movb	$0x2, 0x14c(%rax,%rcx)
1407c3268: 0f b6 95 55 03 00 00        	movzbl	0x355(%rbp), %edx
1407c326f: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407c3276: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407c3280: 48 ff c6                    	incq	%rsi
1407c3283: 49 89 b1 c0 00 00 00        	movq	%rsi, 0xc0(%r9)
1407c328a: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c3291: 41 b8 1e 00 00 00           	movl	$0x1e, %r8d
1407c3297: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
1407c329b: 48 8d 15 5e 34 2c 02        	leaq	0x22c345e(%rip), %rdx   # 0x142a86700
1407c32a2: e8 a9 a1 16 00              	callq	0x14092d450 <.text+0x92c450>
1407c32a7: 0f 29 b5 30 03 00 00        	movaps	%xmm6, 0x330(%rbp)
1407c32ae: 48 85 c0                    	testq	%rax, %rax
1407c32b1: 0f 84 1f 13 00 00           	je	0x1407c45d6 <.text+0x7c35d6>
1407c32b7: f3 0f 16 d7                 	movshdup	%xmm7, %xmm2            # xmm2 = xmm7[1,1,3,3]
1407c32bb: 44 0f 28 ef                 	movaps	%xmm7, %xmm13
1407c32bf: 66 44 0f 15 ef              	unpckhpd	%xmm7, %xmm13           # xmm13 = xmm13[1],xmm7[1]
1407c32c4: 0f 28 ef                    	movaps	%xmm7, %xmm5
1407c32c7: 0f c6 ef ff                 	shufps	$0xff, %xmm7, %xmm5     # xmm5 = xmm5[3,3],xmm7[3,3]
1407c32cb: 45 0f 28 fd                 	movaps	%xmm13, %xmm15
1407c32cf: f3 45 0f 58 fd              	addss	%xmm13, %xmm15
1407c32d4: 0f 28 e7                    	movaps	%xmm7, %xmm4
1407c32d7: f3 41 0f 59 e7              	mulss	%xmm15, %xmm4
1407c32dc: f3 41 0f 16 c4              	movshdup	%xmm12, %xmm0           # xmm0 = xmm12[1,1,3,3]
1407c32e1: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407c32e4: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407c32e8: 0f 28 df                    	movaps	%xmm7, %xmm3
1407c32eb: 41 0f 59 dc                 	mulps	%xmm12, %xmm3
1407c32ef: f3 41 0f 59 d7              	mulss	%xmm15, %xmm2
1407c32f4: f3 45 0f 59 ef              	mulss	%xmm15, %xmm13
1407c32f9: f3 44 0f 59 e5              	mulss	%xmm5, %xmm12
1407c32fe: f3 0f 59 c5                 	mulss	%xmm5, %xmm0
1407c3302: f3 44 0f 59 fd              	mulss	%xmm5, %xmm15
1407c3307: f3 0f 16 f3                 	movshdup	%xmm3, %xmm6            # xmm6 = xmm3[1,1,3,3]
1407c330b: 44 0f 28 f6                 	movaps	%xmm6, %xmm14
1407c330f: f3 45 0f 58 f5              	addss	%xmm13, %xmm14
1407c3314: f3 0f 10 2d e8 25 16 02     	movss	0x21625e8(%rip), %xmm5  # 0x142925904
1407c331c: f3 41 0f 5c ee              	subss	%xmm14, %xmm5
1407c3321: 44 0f 28 f1                 	movaps	%xmm1, %xmm14
1407c3325: f3 45 0f 58 f7              	addss	%xmm15, %xmm14
1407c332a: 41 0f 14 ee                 	unpcklps	%xmm14, %xmm5           # xmm5 = xmm5[0],xmm14[0],xmm5[1],xmm14[1]
1407c332e: 44 0f 28 f4                 	movaps	%xmm4, %xmm14
1407c3332: f3 44 0f 5c f0              	subss	%xmm0, %xmm14
1407c3337: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
1407c333b: f3 45 0f 10 c6              	movss	%xmm14, %xmm8           # xmm8 = xmm14[0],xmm8[1,2,3]
1407c3340: 41 0f 16 e8                 	movlhps	%xmm8, %xmm5            # xmm5 = xmm5[0],xmm8[0]
1407c3344: f3 41 0f 5c cf              	subss	%xmm15, %xmm1
1407c3349: f3 44 0f 58 eb              	addss	%xmm3, %xmm13
1407c334e: f3 44 0f 10 35 ad 25 16 02  	movss	0x21625ad(%rip), %xmm14 # 0x142925904
1407c3357: 45 0f 28 c6                 	movaps	%xmm14, %xmm8
1407c335b: f3 45 0f 5c c5              	subss	%xmm13, %xmm8
1407c3360: 41 0f 14 c8                 	unpcklps	%xmm8, %xmm1            # xmm1 = xmm1[0],xmm8[0],xmm1[1],xmm8[1]
1407c3364: 44 0f 28 c2                 	movaps	%xmm2, %xmm8
1407c3368: f3 45 0f 58 c4              	addss	%xmm12, %xmm8
1407c336d: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
1407c3371: f3 45 0f 10 e8              	movss	%xmm8, %xmm13           # xmm13 = xmm8[0],xmm13[1,2,3]
1407c3376: 41 0f 16 cd                 	movlhps	%xmm13, %xmm1           # xmm1 = xmm1[0],xmm13[0]
1407c337a: f3 0f 58 c4                 	addss	%xmm4, %xmm0
1407c337e: f3 41 0f 5c d4              	subss	%xmm12, %xmm2
1407c3383: 0f 14 c2                    	unpcklps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0],xmm0[1],xmm2[1]
1407c3386: f3 0f 58 de                 	addss	%xmm6, %xmm3
1407c338a: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
1407c338e: f3 0f 5c d3                 	subss	%xmm3, %xmm2
1407c3392: 0f 57 db                    	xorps	%xmm3, %xmm3
1407c3395: f3 0f 10 da                 	movss	%xmm2, %xmm3            # xmm3 = xmm2[0],xmm3[1,2,3]
1407c3399: 0f 28 95 50 01 00 00        	movaps	0x150(%rbp), %xmm2
1407c33a0: 0f 16 c3                    	movlhps	%xmm3, %xmm0            # xmm0 = xmm0[0],xmm3[0]
1407c33a3: 0f 10 9d 54 01 00 00        	movups	0x154(%rbp), %xmm3
1407c33aa: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407c33ae: 0f 59 d5                    	mulps	%xmm5, %xmm2
1407c33b1: 0f 28 e3                    	movaps	%xmm3, %xmm4
1407c33b4: 0f c6 e3 00                 	shufps	$0x0, %xmm3, %xmm4      # xmm4 = xmm4[0,0],xmm3[0,0]
1407c33b8: 0f 59 e1                    	mulps	%xmm1, %xmm4
1407c33bb: 0f c6 db 55                 	shufps	$0x55, %xmm3, %xmm3     # xmm3 = xmm3[1,1,1,1]
1407c33bf: 0f 59 d8                    	mulps	%xmm0, %xmm3
1407c33c2: 41 0f 28 c6                 	movaps	%xmm14, %xmm0
1407c33c6: 41 0f 16 c3                 	movlhps	%xmm11, %xmm0           # xmm0 = xmm0[0],xmm11[0]
1407c33ca: 0f 28 b5 30 03 00 00        	movaps	0x330(%rbp), %xmm6
1407c33d1: 0f 28 ce                    	movaps	%xmm6, %xmm1
1407c33d4: 0f c6 c8 24                 	shufps	$0x24, %xmm0, %xmm1     # xmm1 = xmm1[0,1],xmm0[2,0]
1407c33d8: 0f 29 95 90 00 00 00        	movaps	%xmm2, 0x90(%rbp)
1407c33df: 0f 29 a5 a0 00 00 00        	movaps	%xmm4, 0xa0(%rbp)
1407c33e6: 0f 29 9d b0 00 00 00        	movaps	%xmm3, 0xb0(%rbp)
1407c33ed: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1407c33f4: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c33fb: 48 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %rcx
1407c3402: 48 89 c2                    	movq	%rax, %rdx
1407c3405: e8 76 4e 24 00              	callq	0x140a08280 <.text+0xa07280>
1407c340a: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1407c3411: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1407c3418: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
1407c341c: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1407c3420: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407c342b: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
1407c3436: 48 89 8d f8 01 00 00        	movq	%rcx, 0x1f8(%rbp)
1407c343d: 48 89 8d b0 02 00 00        	movq	%rcx, 0x2b0(%rbp)
1407c3444: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
1407c344b: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
1407c3452: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
1407c3459: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c3460: 48 8d 95 30 02 00 00        	leaq	0x230(%rbp), %rdx
1407c3467: 4c 8d 05 ca 32 2c 02        	leaq	0x22c32ca(%rip), %r8    # 0x142a86738
1407c346e: e8 fd c9 a9 00              	callq	0x14125fe70 <.text+0x125ee70>
1407c3473: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
1407c347a: 48 85 c0                    	testq	%rax, %rax
1407c347d: 4c 8b a5 28 03 00 00        	movq	0x328(%rbp), %r12
1407c3484: 74 1a                       	je	0x1407c34a0 <.text+0x7c24a0>
1407c3486: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c348a: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c348e: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c3494: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
1407c349b: e8 80 07 e5 ff              	callq	0x140613c20 <.text+0x612c20>
1407c34a0: 44 0f 28 e7                 	movaps	%xmm7, %xmm12
1407c34a4: 44 0f 59 25 34 2e 2c 02     	mulps	0x22c2e34(%rip), %xmm12 # 0x142a862e0
1407c34ac: f3 0f 7e 05 2c 24 16 02     	movq	0x216242c(%rip), %xmm0  # 0x1429258e0
1407c34b4: 66 0f d6 85 50 01 00 00     	movq	%xmm0, 0x150(%rbp)
1407c34bc: c7 85 58 01 00 00 00 00 00 3f       	movl	$0x3f000000, 0x158(%rbp) # imm = 0x3F000000
1407c34c6: 48 8d 85 40 02 00 00        	leaq	0x240(%rbp), %rax
1407c34cd: c7 40 fb 00 00 00 00        	movl	$0x0, -0x5(%rax)
1407c34d4: 48 c7 40 f3 00 00 00 00     	movq	$0x0, -0xd(%rax)
1407c34dc: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407c34e3: 0f b6 58 04                 	movzbl	0x4(%rax), %ebx
1407c34e7: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1407c34ee: 48 8b 8d 37 02 00 00        	movq	0x237(%rbp), %rcx
1407c34f5: 48 89 8d 37 01 00 00        	movq	%rcx, 0x137(%rbp)
1407c34fc: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1407c3503: 49 8b b4 24 c0 00 00 00     	movq	0xc0(%r12), %rsi
1407c350b: 49 3b b4 24 b0 00 00 00     	cmpq	0xb0(%r12), %rsi
1407c3513: 75 16                       	jne	0x1407c352b <.text+0x7c252b>
1407c3515: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c351c: 48 89 f9                    	movq	%rdi, %rcx
1407c351f: 48 8d 15 2a 32 2c 02        	leaq	0x22c322a(%rip), %rdx   # 0x142a86750
1407c3526: e8 85 7e 06 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407c352b: 0f 28 15 9e 2d 2c 02        	movaps	0x22c2d9e(%rip), %xmm2  # 0x142a862d0
1407c3532: 44 0f 59 d2                 	mulps	%xmm2, %xmm10
1407c3536: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407c3539: 0f 59 c2                    	mulps	%xmm2, %xmm0
1407c353c: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
1407c3540: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407c3544: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407c3547: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407c354b: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407c354f: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c3553: 0f 59 c7                    	mulps	%xmm7, %xmm0
1407c3556: 41 0f 58 c2                 	addps	%xmm10, %xmm0
1407c355a: 0f 28 8d 40 03 00 00        	movaps	0x340(%rbp), %xmm1
1407c3561: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407c3564: 41 0f 58 cc                 	addps	%xmm12, %xmm1
1407c3568: 0f c6 c9 d2                 	shufps	$0xd2, %xmm1, %xmm1     # xmm1 = xmm1[2,0,1,3]
1407c356c: 44 0f 59 c9                 	mulps	%xmm1, %xmm9
1407c3570: 44 0f 58 c8                 	addps	%xmm0, %xmm9
1407c3574: 41 0f 58 f1                 	addps	%xmm9, %xmm6
1407c3578: 45 0f 12 c9                 	movhlps	%xmm9, %xmm9            # xmm9 = xmm9[1,1]
1407c357c: f3 45 0f 58 cb              	addss	%xmm11, %xmm9
1407c3581: 4c 8b 8d 28 03 00 00        	movq	0x328(%rbp), %r9
1407c3588: 49 8b 81 b8 00 00 00        	movq	0xb8(%r9), %rax
1407c358f: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407c3596: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407c359e: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407c35a7: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407c35b0: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407c35b8: 48 8b 95 90 01 00 00        	movq	0x190(%rbp), %rdx
1407c35bf: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407c35c4: 8b 95 98 01 00 00           	movl	0x198(%rbp), %edx
1407c35ca: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407c35ce: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407c35d6: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1407c35dd: 0f 10 8d 40 02 00 00        	movups	0x240(%rbp), %xmm1
1407c35e4: 0f 10 95 50 02 00 00        	movups	0x250(%rbp), %xmm2
1407c35eb: 0f 10 9d 5c 02 00 00        	movups	0x25c(%rbp), %xmm3
1407c35f2: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407c35f7: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407c35fc: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407c3601: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407c3606: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407c3611: 48 8b 55 78                 	movq	0x78(%rbp), %rdx
1407c3615: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407c361d: 8b 95 80 00 00 00           	movl	0x80(%rbp), %edx
1407c3623: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407c362a: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407c3635: 48 8b 95 84 00 00 00        	movq	0x84(%rbp), %rdx
1407c363c: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407c3644: 8b 95 8c 00 00 00           	movl	0x8c(%rbp), %edx
1407c364a: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407c3651: 48 c7 84 08 a0 00 00 00 01 00 00 00 	movq	$0x1, 0xa0(%rax,%rcx)
1407c365d: 48 c7 84 08 a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rax,%rcx)
1407c3669: 48 8d 15 b4 db 45 02        	leaq	0x245dbb4(%rip), %rdx   # 0x142c21224
1407c3670: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407c3678: 48 c7 84 08 b8 00 00 00 0a 00 00 00 	movq	$0xa, 0xb8(%rax,%rcx)
1407c3684: 0f 29 bc 08 c0 00 00 00     	movaps	%xmm7, 0xc0(%rax,%rcx)
1407c368c: 0f 13 b4 08 d0 00 00 00     	movlps	%xmm6, 0xd0(%rax,%rcx)
1407c3694: f3 44 0f 11 8c 08 d8 00 00 00       	movss	%xmm9, 0xd8(%rax,%rcx)
1407c369e: 8b 95 60 01 00 00           	movl	0x160(%rbp), %edx
1407c36a4: 89 94 08 ec 00 00 00        	movl	%edx, 0xec(%rax,%rcx)
1407c36ab: 0f 28 85 50 01 00 00        	movaps	0x150(%rbp), %xmm0
1407c36b2: 0f 11 84 08 dc 00 00 00     	movups	%xmm0, 0xdc(%rax,%rcx)
1407c36ba: c7 84 08 f0 00 00 00 00 00 00 00    	movl	$0x0, 0xf0(%rax,%rcx)
1407c36c5: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407c36d0: 48 8b 95 d0 01 00 00        	movq	0x1d0(%rbp), %rdx
1407c36d7: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407c36df: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1407c36e6: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407c36ee: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407c36f6: 0f b6 95 d3 02 00 00        	movzbl	0x2d3(%rbp), %edx
1407c36fd: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407c3704: 0f b7 95 d1 02 00 00        	movzwl	0x2d1(%rbp), %edx
1407c370b: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407c3713: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407c371e: 0f 10 85 e8 02 00 00        	movups	0x2e8(%rbp), %xmm0
1407c3725: 0f 10 8d f8 02 00 00        	movups	0x2f8(%rbp), %xmm1
1407c372c: 0f 11 8c 08 2c 01 00 00     	movups	%xmm1, 0x12c(%rax,%rcx)
1407c3734: 0f 11 84 08 1c 01 00 00     	movups	%xmm0, 0x11c(%rax,%rcx)
1407c373c: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407c3744: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1407c374b: 4c 8b 85 37 01 00 00        	movq	0x137(%rbp), %r8
1407c3752: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407c375a: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407c3762: c6 84 08 4c 01 00 00 01     	movb	$0x1, 0x14c(%rax,%rcx)
1407c376a: 88 9c 08 4d 01 00 00        	movb	%bl, 0x14d(%rax,%rcx)
1407c3771: 0f b6 95 55 03 00 00        	movzbl	0x355(%rbp), %edx
1407c3778: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407c377f: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407c3789: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
1407c3790: 4c 8b 85 47 01 00 00        	movq	0x147(%rbp), %r8
1407c3797: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407c379f: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407c37a7: 48 ff c6                    	incq	%rsi
1407c37aa: 49 89 b1 c0 00 00 00        	movq	%rsi, 0xc0(%r9)
1407c37b1: e9 61 08 00 00              	jmp	0x1407c4017 <.text+0x7c3017>
1407c37b6: 44 89 d0                    	movl	%r10d, %eax
1407c37b9: 45 84 db                    	testb	%r11b, %r11b
1407c37bc: 0f 85 a9 01 00 00           	jne	0x1407c396b <.text+0x7c296b>
1407c37c2: 48 83 ee 01                 	subq	$0x1, %rsi
1407c37c6: 0f 82 b8 0a 00 00           	jb	0x1407c4284 <.text+0x7c3284>
1407c37cc: 66 85 c0                    	testw	%ax, %ax
1407c37cf: 74 0f                       	je	0x1407c37e0 <.text+0x7c27e0>
1407c37d1: 44 8d 50 ff                 	leal	-0x1(%rax), %r10d
1407c37d5: 41 21 c2                    	andl	%eax, %r10d
1407c37d8: eb 2f                       	jmp	0x1407c3809 <.text+0x7c2809>
1407c37da: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407c37e0: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
1407c37e4: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
1407c37e8: 48 81 c7 00 fb ff ff        	addq	$-0x500, %rdi           # imm = 0xFB00
1407c37ef: 48 83 c3 10                 	addq	$0x10, %rbx
1407c37f3: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
1407c37f8: 74 e6                       	je	0x1407c37e0 <.text+0x7c27e0>
1407c37fa: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
1407c37ff: 29 c1                       	subl	%eax, %ecx
1407c3801: f7 d0                       	notl	%eax
1407c3803: 41 89 c2                    	movl	%eax, %r10d
1407c3806: 41 21 ca                    	andl	%ecx, %r10d
1407c3809: f3 0f bc c0                 	tzcntl	%eax, %eax
1407c380d: 48 f7 d8                    	negq	%rax
1407c3810: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1407c3814: 48 c1 e0 04                 	shlq	$0x4, %rax
1407c3818: 8b 4c 07 b0                 	movl	-0x50(%rdi,%rax), %ecx
1407c381c: 0f b6 54 07 da              	movzbl	-0x26(%rdi,%rax), %edx
1407c3821: 44 8d 42 fe                 	leal	-0x2(%rdx), %r8d
1407c3825: 48 ff ca                    	decq	%rdx
1407c3828: 45 31 db                    	xorl	%r11d, %r11d
1407c382b: 41 80 f8 03                 	cmpb	$0x3, %r8b
1407c382f: 49 0f 43 d3                 	cmovaeq	%r11, %rdx
1407c3833: 48 85 d2                    	testq	%rdx, %rdx
1407c3836: 74 1b                       	je	0x1407c3853 <.text+0x7c2853>
1407c3838: 48 83 fa 01                 	cmpq	$0x1, %rdx
1407c383c: 0f 85 5c 0d 00 00           	jne	0x1407c459e <.text+0x7c359e>
1407c3842: 44 89 d0                    	movl	%r10d, %eax
1407c3845: 44 39 c9                    	cmpl	%r9d, %ecx
1407c3848: 0f 85 68 ff ff ff           	jne	0x1407c37b6 <.text+0x7c27b6>
1407c384e: e9 6f ff ff ff              	jmp	0x1407c37c2 <.text+0x7c27c2>
1407c3853: 44 89 95 d4 02 00 00        	movl	%r10d, 0x2d4(%rbp)
1407c385a: 48 01 f8                    	addq	%rdi, %rax
1407c385d: f3 44 0f 10 50 b8           	movss	-0x48(%rax), %xmm10
1407c3863: f3 44 0f 10 40 bc           	movss	-0x44(%rax), %xmm8
1407c3869: 44 8a 68 fc                 	movb	-0x4(%rax), %r13b
1407c386d: 48 8b 55 68                 	movq	0x68(%rbp), %rdx
1407c3871: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c3878: 89 8d d8 02 00 00           	movl	%ecx, 0x2d8(%rbp)
1407c387e: e8 1d 03 07 00              	callq	0x140833ba0 <.text+0x832ba0>
1407c3883: 4c 8b 65 70                 	movq	0x70(%rbp), %r12
1407c3887: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c388e: 44 89 e9                    	movl	%r13d, %ecx
1407c3891: 89 c2                       	movl	%eax, %edx
1407c3893: e8 f8 23 36 00              	callq	0x140b25c90 <.text+0xb24c90>
1407c3898: 88 85 c0 01 00 00           	movb	%al, 0x1c0(%rbp)
1407c389e: 88 95 c1 01 00 00           	movb	%dl, 0x1c1(%rbp)
1407c38a4: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c38ab: 4c 89 e1                    	movq	%r12, %rcx
1407c38ae: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407c38b5: e8 d6 96 a9 00              	callq	0x14125cf90 <.text+0x125bf90>
1407c38ba: 48 85 c0                    	testq	%rax, %rax
1407c38bd: 0f 84 43 0d 00 00           	je	0x1407c4606 <.text+0x7c3606>
1407c38c3: f3 0f 10 70 30              	movss	0x30(%rax), %xmm6
1407c38c8: f3 0f 10 40 3c              	movss	0x3c(%rax), %xmm0
1407c38cd: f3 0f 5c 40 38              	subss	0x38(%rax), %xmm0
1407c38d2: 8b 85 88 01 00 00           	movl	0x188(%rbp), %eax
1407c38d8: 89 85 30 02 00 00           	movl	%eax, 0x230(%rbp)
1407c38de: f3 44 0f 11 8d 34 02 00 00  	movss	%xmm9, 0x234(%rbp)
1407c38e7: f3 0f 59 05 81 1f 16 02     	mulss	0x2161f81(%rip), %xmm0  # 0x142925870
1407c38ef: f3 45 0f 5c c6              	subss	%xmm14, %xmm8
1407c38f4: 44 0f 54 05 84 b0 16 02     	andps	0x216b084(%rip), %xmm8  # 0x14292e980
1407c38fc: f3 41 0f 58 c5              	addss	%xmm13, %xmm0
1407c3901: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
1407c3905: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407c390c: 76 31                       	jbe	0x1407c393f <.text+0x7c293f>
1407c390e: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c3915: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407c3919: f3 0f 10 95 dc 02 00 00     	movss	0x2dc(%rbp), %xmm2
1407c3921: e8 ca e4 4c 00              	callq	0x140c91df0 <.text+0xc90df0>
1407c3926: f3 0f 59 35 42 1f 16 02     	mulss	0x2161f42(%rip), %xmm6  # 0x142925870
1407c392e: f3 0f 58 b5 1c 03 00 00     	addss	0x31c(%rbp), %xmm6
1407c3936: 0f 2e f0                    	ucomiss	%xmm0, %xmm6
1407c3939: 41 0f 97 c3                 	seta	%r11b
1407c393d: eb 03                       	jmp	0x1407c3942 <.text+0x7c2942>
1407c393f: 45 31 db                    	xorl	%r11d, %r11d
1407c3942: 44 8b 8d 30 03 00 00        	movl	0x330(%rbp), %r9d
1407c3949: 44 8b 95 d4 02 00 00        	movl	0x2d4(%rbp), %r10d
1407c3950: 8b 8d d8 02 00 00           	movl	0x2d8(%rbp), %ecx
1407c3956: 44 89 d0                    	movl	%r10d, %eax
1407c3959: 44 39 c9                    	cmpl	%r9d, %ecx
1407c395c: 0f 85 54 fe ff ff           	jne	0x1407c37b6 <.text+0x7c27b6>
1407c3962: e9 5b fe ff ff              	jmp	0x1407c37c2 <.text+0x7c27c2>
1407c3967: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
1407c396b: 31 ff                       	xorl	%edi, %edi
1407c396d: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c3974: 48 8d 85 00 02 00 00        	leaq	0x200(%rbp), %rax
1407c397b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c3980: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1407c3987: 66 0f 6e 0d 75 1f 16 02     	movd	0x2161f75(%rip), %xmm1  # 0x142925904
1407c398f: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c3993: 66 0f 6f d9                 	movdqa	%xmm1, %xmm3
1407c3997: e8 44 67 31 00              	callq	0x140ada0e0 <.text+0xad90e0>
1407c399c: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1407c39a3: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
1407c39aa: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
1407c39ae: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1407c39b2: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407c39bd: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
1407c39c8: 48 89 8d 40 03 00 00        	movq	%rcx, 0x340(%rbp)
1407c39cf: 48 89 8d b0 02 00 00        	movq	%rcx, 0x2b0(%rbp)
1407c39d6: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
1407c39dd: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c39e4: 48 8d 95 30 02 00 00        	leaq	0x230(%rbp), %rdx
1407c39eb: 4c 8d 05 7e 2c 2c 02        	leaq	0x22c2c7e(%rip), %r8    # 0x142a86670
1407c39f2: e8 29 c1 a9 00              	callq	0x14125fb20 <.text+0x125eb20>
1407c39f7: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1407c39fe: 48 85 c0                    	testq	%rax, %rax
1407c3a01: 8b b5 30 03 00 00           	movl	0x330(%rbp), %esi
1407c3a07: 74 1a                       	je	0x1407c3a23 <.text+0x7c2a23>
1407c3a09: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c3a0d: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c3a11: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c3a17: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c3a1e: e8 fd 01 e5 ff              	callq	0x140613c20 <.text+0x612c20>
1407c3a23: 40 84 ff                    	testb	%dil, %dil
1407c3a26: 0f 85 33 01 00 00           	jne	0x1407c3b5f <.text+0x7c2b5f>
1407c3a2c: 0f 28 85 00 02 00 00        	movaps	0x200(%rbp), %xmm0
1407c3a33: 0f 28 8d 10 02 00 00        	movaps	0x210(%rbp), %xmm1
1407c3a3a: 0f 28 95 20 02 00 00        	movaps	0x220(%rbp), %xmm2
1407c3a41: 0f 29 8d a0 00 00 00        	movaps	%xmm1, 0xa0(%rbp)
1407c3a48: 0f 29 95 b0 00 00 00        	movaps	%xmm2, 0xb0(%rbp)
1407c3a4f: 0f 29 85 90 00 00 00        	movaps	%xmm0, 0x90(%rbp)
1407c3a56: f3 0f 10 85 ac 00 00 00     	movss	0xac(%rbp), %xmm0
1407c3a5e: f3 0f 10 0d 06 b7 21 02     	movss	0x221b706(%rip), %xmm1  # 0x1429df16c
1407c3a66: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407c3a6a: f3 0f 11 85 ac 00 00 00     	movss	%xmm0, 0xac(%rbp)
1407c3a72: f3 0f 10 85 b4 00 00 00     	movss	0xb4(%rbp), %xmm0
1407c3a7a: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407c3a7e: f3 0f 11 85 b4 00 00 00     	movss	%xmm0, 0xb4(%rbp)
1407c3a86: c7 85 b0 00 00 00 cd cc cc 3e       	movl	$0x3ecccccd, 0xb0(%rbp) # imm = 0x3ECCCCCD
1407c3a90: f3 0f 10 85 a4 00 00 00     	movss	0xa4(%rbp), %xmm0
1407c3a98: f3 41 0f 5c c3              	subss	%xmm11, %xmm0
1407c3a9d: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407c3aa1: f3 0f 11 85 a4 00 00 00     	movss	%xmm0, 0xa4(%rbp)
1407c3aa9: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c3ab0: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
1407c3ab7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c3abc: 48 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %rcx
1407c3ac3: 66 0f 6e 0d 39 1e 16 02     	movd	0x2161e39(%rip), %xmm1  # 0x142925904
1407c3acb: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c3acf: 66 0f 6f d9                 	movdqa	%xmm1, %xmm3
1407c3ad3: e8 08 66 31 00              	callq	0x140ada0e0 <.text+0xad90e0>
1407c3ad8: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1407c3adf: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1407c3ae6: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
1407c3aea: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1407c3aee: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407c3af9: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
1407c3b04: 48 89 8d 40 03 00 00        	movq	%rcx, 0x340(%rbp)
1407c3b0b: 48 89 8d b0 02 00 00        	movq	%rcx, 0x2b0(%rbp)
1407c3b12: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
1407c3b19: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c3b20: 48 8d 95 30 02 00 00        	leaq	0x230(%rbp), %rdx
1407c3b27: 4c 8d 05 5a 2b 2c 02        	leaq	0x22c2b5a(%rip), %r8    # 0x142a86688
1407c3b2e: e8 ed bf a9 00              	callq	0x14125fb20 <.text+0x125eb20>
1407c3b33: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
1407c3b3a: 48 85 c0                    	testq	%rax, %rax
1407c3b3d: 8b b5 30 03 00 00           	movl	0x330(%rbp), %esi
1407c3b43: 74 1a                       	je	0x1407c3b5f <.text+0x7c2b5f>
1407c3b45: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c3b49: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c3b4d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c3b53: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c3b5a: e8 c1 00 e5 ff              	callq	0x140613c20 <.text+0x612c20>
1407c3b5f: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c3b66: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
1407c3b6a: 48 8b 95 28 01 00 00        	movq	0x128(%rbp), %rdx
1407c3b71: 41 89 f0                    	movl	%esi, %r8d
1407c3b74: 4c 8d 0d 25 2b 2c 02        	leaq	0x22c2b25(%rip), %r9    # 0x142a866a0
1407c3b7b: e8 a0 6e 25 00              	callq	0x140a1aa20 <.text+0xa19a20>
1407c3b80: 48 8d 45 0c                 	leaq	0xc(%rbp), %rax
1407c3b84: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1407c3b8e: 48 89 08                    	movq	%rcx, (%rax)
1407c3b91: c7 40 08 00 00 80 3f        	movl	$0x3f800000, 0x8(%rax)  # imm = 0x3F800000
1407c3b98: f3 44 0f 58 7d 04           	addss	0x4(%rbp), %xmm15
1407c3b9e: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
1407c3ba2: 0f 28 4d 00                 	movaps	(%rbp), %xmm1
1407c3ba6: 41 0f 14 cf                 	unpcklps	%xmm15, %xmm1           # xmm1 = xmm1[0],xmm15[0],xmm1[1],xmm15[1]
1407c3baa: 0f 28 d8                    	movaps	%xmm0, %xmm3
1407c3bad: 0f 59 d8                    	mulps	%xmm0, %xmm3
1407c3bb0: f3 0f 16 d3                 	movshdup	%xmm3, %xmm2            # xmm2 = xmm3[1,1,3,3]
1407c3bb4: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407c3bb8: 0f 28 e3                    	movaps	%xmm3, %xmm4
1407c3bbb: 66 0f 15 e3                 	unpckhpd	%xmm3, %xmm4            # xmm4 = xmm4[1],xmm3[1]
1407c3bbf: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1407c3bc3: 0f c6 e4 00                 	shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
1407c3bc7: 0f c6 db ff                 	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
1407c3bcb: 0f 5c dc                    	subps	%xmm4, %xmm3
1407c3bce: 0f 28 25 1b 27 2c 02        	movaps	0x22c271b(%rip), %xmm4  # 0x142a862f0
1407c3bd5: 0f 59 dc                    	mulps	%xmm4, %xmm3
1407c3bd8: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407c3bdb: 0f 59 d4                    	mulps	%xmm4, %xmm2
1407c3bde: f3 0f 16 e2                 	movshdup	%xmm2, %xmm4            # xmm4 = xmm2[1,1,3,3]
1407c3be2: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1407c3be6: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407c3be9: f3 0f 58 d4                 	addss	%xmm4, %xmm2
1407c3bed: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1407c3bf1: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407c3bf5: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407c3bf8: 0f 58 d3                    	addps	%xmm3, %xmm2
1407c3bfb: 0f 28 d8                    	movaps	%xmm0, %xmm3
1407c3bfe: f3 0f 10 2d 2a 27 2c 02     	movss	0x22c272a(%rip), %xmm5  # 0x142a86330
1407c3c06: 0f 59 dd                    	mulps	%xmm5, %xmm3
1407c3c09: 0f 28 e0                    	movaps	%xmm0, %xmm4
1407c3c0c: 0f c6 e0 c9                 	shufps	$0xc9, %xmm0, %xmm4     # xmm4 = xmm4[1,2],xmm0[0,3]
1407c3c10: 0f 59 e5                    	mulps	%xmm5, %xmm4
1407c3c13: 0f c6 db d2                 	shufps	$0xd2, %xmm3, %xmm3     # xmm3 = xmm3[2,0,1,3]
1407c3c17: 0f 5c e3                    	subps	%xmm3, %xmm4
1407c3c1a: 0f 58 c0                    	addps	%xmm0, %xmm0
1407c3c1d: 0f c6 c0 ff                 	shufps	$0xff, %xmm0, %xmm0     # xmm0 = xmm0[3,3,3,3]
1407c3c21: 0f 59 c4                    	mulps	%xmm4, %xmm0
1407c3c24: 0f 58 c2                    	addps	%xmm2, %xmm0
1407c3c27: 0f 58 c8                    	addps	%xmm0, %xmm1
1407c3c2a: 0f 13 4d 00                 	movlps	%xmm1, (%rbp)
1407c3c2e: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407c3c31: f3 0f 58 45 08              	addss	0x8(%rbp), %xmm0
1407c3c36: f3 0f 11 45 08              	movss	%xmm0, 0x8(%rbp)
1407c3c3b: 48 8d 85 40 02 00 00        	leaq	0x240(%rbp), %rax
1407c3c42: c7 40 fb 00 00 00 00        	movl	$0x0, -0x5(%rax)
1407c3c49: 48 c7 40 f3 00 00 00 00     	movq	$0x0, -0xd(%rax)
1407c3c51: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1407c3c58: 0f b6 58 04                 	movzbl	0x4(%rax), %ebx
1407c3c5c: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
1407c3c63: 48 8b 8d 37 02 00 00        	movq	0x237(%rbp), %rcx
1407c3c6a: 48 89 8d c7 01 00 00        	movq	%rcx, 0x1c7(%rbp)
1407c3c71: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407c3c78: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
1407c3c7f: 48 8b b0 c0 00 00 00        	movq	0xc0(%rax), %rsi
1407c3c86: 48 3b b0 b0 00 00 00        	cmpq	0xb0(%rax), %rsi
1407c3c8d: 75 21                       	jne	0x1407c3cb0 <.text+0x7c2cb0>
1407c3c8f: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
1407c3c96: 48 81 c1 b0 00 00 00        	addq	$0xb0, %rcx
1407c3c9d: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c3ca4: 48 8d 15 0d 2a 2c 02        	leaq	0x22c2a0d(%rip), %rdx   # 0x142a866b8
1407c3cab: e8 00 77 06 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407c3cb0: 4c 8b a5 28 03 00 00        	movq	0x328(%rbp), %r12
1407c3cb7: 49 8b 84 24 b8 00 00 00     	movq	0xb8(%r12), %rax
1407c3cbf: 48 69 ce 60 01 00 00        	imulq	$0x160, %rsi, %rcx      # imm = 0x160
1407c3cc6: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407c3cce: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407c3cd7: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407c3ce0: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407c3ce8: 48 8b 95 40 01 00 00        	movq	0x140(%rbp), %rdx
1407c3cef: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407c3cf4: 8b 95 48 01 00 00           	movl	0x148(%rbp), %edx
1407c3cfa: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407c3cfe: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407c3d06: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1407c3d0d: 0f 10 8d 40 02 00 00        	movups	0x240(%rbp), %xmm1
1407c3d14: 0f 10 95 50 02 00 00        	movups	0x250(%rbp), %xmm2
1407c3d1b: 0f 10 9d 5c 02 00 00        	movups	0x25c(%rbp), %xmm3
1407c3d22: 0f 11 44 08 44              	movups	%xmm0, 0x44(%rax,%rcx)
1407c3d27: 0f 11 4c 08 54              	movups	%xmm1, 0x54(%rax,%rcx)
1407c3d2c: 0f 11 54 08 64              	movups	%xmm2, 0x64(%rax,%rcx)
1407c3d31: 0f 11 5c 08 70              	movups	%xmm3, 0x70(%rax,%rcx)
1407c3d36: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407c3d41: 48 8b 95 90 01 00 00        	movq	0x190(%rbp), %rdx
1407c3d48: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407c3d50: 8b 95 98 01 00 00           	movl	0x198(%rbp), %edx
1407c3d56: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407c3d5d: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407c3d68: 48 8b 55 78                 	movq	0x78(%rbp), %rdx
1407c3d6c: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407c3d74: 8b 95 80 00 00 00           	movl	0x80(%rbp), %edx
1407c3d7a: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407c3d81: 48 c7 84 08 a0 00 00 00 01 00 00 00 	movq	$0x1, 0xa0(%rax,%rcx)
1407c3d8d: 48 c7 84 08 a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rax,%rcx)
1407c3d99: 48 8d 15 84 d4 45 02        	leaq	0x245d484(%rip), %rdx   # 0x142c21224
1407c3da0: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407c3da8: 48 c7 84 08 b8 00 00 00 0a 00 00 00 	movq	$0xa, 0xb8(%rax,%rcx)
1407c3db4: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
1407c3db8: 0f 28 4d 00                 	movaps	(%rbp), %xmm1
1407c3dbc: 66 0f 6f 55 10              	movdqa	0x10(%rbp), %xmm2
1407c3dc1: 66 0f 7f 94 08 e0 00 00 00  	movdqa	%xmm2, 0xe0(%rax,%rcx)
1407c3dca: 0f 29 8c 08 d0 00 00 00     	movaps	%xmm1, 0xd0(%rax,%rcx)
1407c3dd2: 0f 29 84 08 c0 00 00 00     	movaps	%xmm0, 0xc0(%rax,%rcx)
1407c3dda: c7 84 08 f0 00 00 00 00 00 00 00    	movl	$0x0, 0xf0(%rax,%rcx)
1407c3de5: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407c3df0: 48 8b 95 f8 02 00 00        	movq	0x2f8(%rbp), %rdx
1407c3df7: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407c3dff: 0f 10 85 e8 02 00 00        	movups	0x2e8(%rbp), %xmm0
1407c3e06: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407c3e0e: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407c3e16: 0f b6 95 86 00 00 00        	movzbl	0x86(%rbp), %edx
1407c3e1d: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407c3e24: 0f b7 95 84 00 00 00        	movzwl	0x84(%rbp), %edx
1407c3e2b: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407c3e33: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407c3e3e: f3 0f 6f 85 90 00 00 00     	movdqu	0x90(%rbp), %xmm0
1407c3e46: f3 0f 6f 8d a0 00 00 00     	movdqu	0xa0(%rbp), %xmm1
1407c3e4e: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407c3e57: f3 0f 7f 84 08 1c 01 00 00  	movdqu	%xmm0, 0x11c(%rax,%rcx)
1407c3e60: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407c3e68: 48 8b 95 c0 01 00 00        	movq	0x1c0(%rbp), %rdx
1407c3e6f: 4c 8b 85 c7 01 00 00        	movq	0x1c7(%rbp), %r8
1407c3e76: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407c3e7e: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407c3e86: c6 84 08 4c 01 00 00 00     	movb	$0x0, 0x14c(%rax,%rcx)
1407c3e8e: 88 9c 08 4d 01 00 00        	movb	%bl, 0x14d(%rax,%rcx)
1407c3e95: 0f b6 95 55 03 00 00        	movzbl	0x355(%rbp), %edx
1407c3e9c: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407c3ea3: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407c3ead: 48 8b 95 30 01 00 00        	movq	0x130(%rbp), %rdx
1407c3eb4: 4c 8b 85 37 01 00 00        	movq	0x137(%rbp), %r8
1407c3ebb: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407c3ec3: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407c3ecb: 48 ff c6                    	incq	%rsi
1407c3ece: 49 89 b4 24 c0 00 00 00     	movq	%rsi, 0xc0(%r12)
1407c3ed6: f3 0f 10 b5 10 02 00 00     	movss	0x210(%rbp), %xmm6
1407c3ede: f3 44 0f 10 85 18 02 00 00  	movss	0x218(%rbp), %xmm8
1407c3ee7: f3 44 0f 10 8d 14 02 00 00  	movss	0x214(%rbp), %xmm9
1407c3ef0: 44 0f 28 95 00 02 00 00     	movaps	0x200(%rbp), %xmm10
1407c3ef8: 0f b6 05 0a 0d e7 02        	movzbl	0x2e70d0a(%rip), %eax   # 0x143634c09
1407c3eff: b9 28 00 00 00              	movl	$0x28, %ecx
1407c3f04: ba 04 00 00 00              	movl	$0x4, %edx
1407c3f09: e8 02 fd e4 ff              	callq	0x140613c10 <.text+0x612c10>
1407c3f0e: 48 85 c0                    	testq	%rax, %rax
1407c3f11: 0f 84 ba 07 00 00           	je	0x1407c46d1 <.text+0x7c36d1>
1407c3f17: 48 89 c6                    	movq	%rax, %rsi
1407c3f1a: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1407c3f1e: f3 41 0f 5c c3              	subss	%xmm11, %xmm0
1407c3f23: f3 0f 58 f8                 	addss	%xmm0, %xmm7
1407c3f27: 40 80 f7 01                 	xorb	$0x1, %dil
1407c3f2b: 40 0f b6 c7                 	movzbl	%dil, %eax
1407c3f2f: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
1407c3f33: 41 0f 58 d2                 	addps	%xmm10, %xmm2
1407c3f37: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
1407c3f3b: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407c3f3f: 41 0f c6 ca c5              	shufps	$0xc5, %xmm10, %xmm1    # xmm1 = xmm1[1,1],xmm10[0,3]
1407c3f44: 0f 59 0d 55 1a 16 02        	mulps	0x2161a55(%rip), %xmm1  # 0x1429259a0
1407c3f4b: 41 0f 28 da                 	movaps	%xmm10, %xmm3
1407c3f4f: 41 0f c6 da d6              	shufps	$0xd6, %xmm10, %xmm3    # xmm3 = xmm3[2,1],xmm10[1,3]
1407c3f54: 0f 59 1d a5 e4 2b 02        	mulps	0x22be4a5(%rip), %xmm3  # 0x142a82400
1407c3f5b: 0f 5c cb                    	subps	%xmm3, %xmm1
1407c3f5e: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407c3f61: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
1407c3f65: 41 0f 59 d2                 	mulps	%xmm10, %xmm2
1407c3f69: 0f 28 da                    	movaps	%xmm2, %xmm3
1407c3f6c: 0f c6 da ff                 	shufps	$0xff, %xmm2, %xmm3     # xmm3 = xmm3[3,3],xmm2[3,3]
1407c3f70: f3 0f 16 e2                 	movshdup	%xmm2, %xmm4            # xmm4 = xmm2[1,1,3,3]
1407c3f74: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1407c3f78: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407c3f7b: f3 0f 58 d4                 	addss	%xmm4, %xmm2
1407c3f7f: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407c3f83: 0f 5c da                    	subps	%xmm2, %xmm3
1407c3f86: f3 0f 10 25 76 19 16 02     	movss	0x2161976(%rip), %xmm4  # 0x142925904
1407c3f8e: 0f 59 dc                    	mulps	%xmm4, %xmm3
1407c3f91: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
1407c3f95: 0f 59 d4                    	mulps	%xmm4, %xmm2
1407c3f98: f3 0f 16 e2                 	movshdup	%xmm2, %xmm4            # xmm4 = xmm2[1,1,3,3]
1407c3f9c: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1407c3fa0: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407c3fa3: f3 0f 58 d4                 	addss	%xmm4, %xmm2
1407c3fa7: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1407c3fab: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407c3faf: 41 0f 59 d2                 	mulps	%xmm10, %xmm2
1407c3fb3: 0f 58 d3                    	addps	%xmm3, %xmm2
1407c3fb6: 0f 58 d1                    	addps	%xmm1, %xmm2
1407c3fb9: f3 45 0f 58 d9              	addss	%xmm9, %xmm11
1407c3fbe: 89 06                       	movl	%eax, (%rsi)
1407c3fc0: f3 44 0f 11 66 04           	movss	%xmm12, 0x4(%rsi)
1407c3fc6: f3 0f 11 76 08              	movss	%xmm6, 0x8(%rsi)
1407c3fcb: f3 44 0f 11 46 0c           	movss	%xmm8, 0xc(%rsi)
1407c3fd1: f3 0f 11 46 10              	movss	%xmm0, 0x10(%rsi)
1407c3fd6: f3 44 0f 11 5e 14           	movss	%xmm11, 0x14(%rsi)
1407c3fdc: 0f c6 d2 e8                 	shufps	$0xe8, %xmm2, %xmm2     # xmm2 = xmm2[0,2,2,3]
1407c3fe0: 0f 13 56 18                 	movlps	%xmm2, 0x18(%rsi)
1407c3fe4: 8b 85 f8 01 00 00           	movl	0x1f8(%rbp), %eax
1407c3fea: 89 46 20                    	movl	%eax, 0x20(%rsi)
1407c3fed: f3 0f 11 7e 24              	movss	%xmm7, 0x24(%rsi)
1407c3ff2: 49 8b 8c 24 18 01 00 00     	movq	0x118(%r12), %rcx
1407c3ffa: 48 85 c9                    	testq	%rcx, %rcx
1407c3ffd: 74 10                       	je	0x1407c400f <.text+0x7c300f>
1407c3fff: ba 28 00 00 00              	movl	$0x28, %edx
1407c4004: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c400a: e8 11 fc e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407c400f: 49 89 b4 24 18 01 00 00     	movq	%rsi, 0x118(%r12)
1407c4017: 48 8b bd b0 01 00 00        	movq	0x1b0(%rbp), %rdi
1407c401e: 48 89 f8                    	movq	%rdi, %rax
1407c4021: 48 b9 ab aa aa aa aa aa aa aa       	movabsq	$-0x5555555555555555, %rcx # imm = 0xAAAAAAAAAAAAAAAB
1407c402b: 48 f7 e1                    	mulq	%rcx
1407c402e: 48 83 ff 03                 	cmpq	$0x3, %rdi
1407c4032: 73 13                       	jae	0x1407c4047 <.text+0x7c3047>
1407c4034: b8 04 00 00 00              	movl	$0x4, %eax
1407c4039: 31 f6                       	xorl	%esi, %esi
1407c403b: 4c 8d a5 30 02 00 00        	leaq	0x230(%rbp), %r12
1407c4042: e9 bd 00 00 00              	jmp	0x1407c4104 <.text+0x7c3104>
1407c4047: 48 89 d6                    	movq	%rdx, %rsi
1407c404a: 48 d1 ee                    	shrq	%rsi
1407c404d: 48 8d 04 b5 00 00 00 00     	leaq	(,%rsi,4), %rax
1407c4055: 48 8d 1c 40                 	leaq	(%rax,%rax,2), %rbx
1407c4059: 0f b6 05 a9 0b e7 02        	movzbl	0x2e70ba9(%rip), %eax   # 0x143634c09
1407c4060: ba 04 00 00 00              	movl	$0x4, %edx
1407c4065: 48 89 d9                    	movq	%rbx, %rcx
1407c4068: e8 a3 fb e4 ff              	callq	0x140613c10 <.text+0x612c10>
1407c406d: 48 85 c0                    	testq	%rax, %rax
1407c4070: 4c 8d a5 30 02 00 00        	leaq	0x230(%rbp), %r12
1407c4077: 0f 84 01 06 00 00           	je	0x1407c467e <.text+0x7c367e>
1407c407d: 48 83 c7 fd                 	addq	$-0x3, %rdi
1407c4081: 48 83 ff 03                 	cmpq	$0x3, %rdi
1407c4085: 73 04                       	jae	0x1407c408b <.text+0x7c308b>
1407c4087: 31 c9                       	xorl	%ecx, %ecx
1407c4089: eb 53                       	jmp	0x1407c40de <.text+0x7c30de>
1407c408b: 48 89 f2                    	movq	%rsi, %rdx
1407c408e: 48 b9 fe ff ff ff ff ff ff 07       	movabsq	$0x7fffffffffffffe, %rcx # imm = 0x7FFFFFFFFFFFFFE
1407c4098: 48 21 ca                    	andq	%rcx, %rdx
1407c409b: 45 31 c0                    	xorl	%r8d, %r8d
1407c409e: 31 c9                       	xorl	%ecx, %ecx
1407c40a0: 45 8d 48 01                 	leal	0x1(%r8), %r9d
1407c40a4: 45 8d 50 02                 	leal	0x2(%r8), %r10d
1407c40a8: 46 89 04 80                 	movl	%r8d, (%rax,%r8,4)
1407c40ac: 46 89 4c 80 04              	movl	%r9d, 0x4(%rax,%r8,4)
1407c40b1: 46 89 54 80 08              	movl	%r10d, 0x8(%rax,%r8,4)
1407c40b6: 48 83 c1 02                 	addq	$0x2, %rcx
1407c40ba: 45 8d 48 03                 	leal	0x3(%r8), %r9d
1407c40be: 45 8d 50 04                 	leal	0x4(%r8), %r10d
1407c40c2: 45 8d 58 05                 	leal	0x5(%r8), %r11d
1407c40c6: 46 89 4c 80 0c              	movl	%r9d, 0xc(%rax,%r8,4)
1407c40cb: 46 89 54 80 10              	movl	%r10d, 0x10(%rax,%r8,4)
1407c40d0: 46 89 5c 80 14              	movl	%r11d, 0x14(%rax,%r8,4)
1407c40d5: 49 83 c0 06                 	addq	$0x6, %r8
1407c40d9: 48 39 ca                    	cmpq	%rcx, %rdx
1407c40dc: 75 c2                       	jne	0x1407c40a0 <.text+0x7c30a0>
1407c40de: 40 f6 c6 01                 	testb	$0x1, %sil
1407c40e2: 74 20                       	je	0x1407c4104 <.text+0x7c3104>
1407c40e4: 8d 14 49                    	leal	(%rcx,%rcx,2), %edx
1407c40e7: 44 8d 04 49                 	leal	(%rcx,%rcx,2), %r8d
1407c40eb: 41 ff c0                    	incl	%r8d
1407c40ee: 44 8d 4c 49 02              	leal	0x2(%rcx,%rcx,2), %r9d
1407c40f3: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1407c40f7: 89 14 88                    	movl	%edx, (%rax,%rcx,4)
1407c40fa: 44 89 44 88 04              	movl	%r8d, 0x4(%rax,%rcx,4)
1407c40ff: 44 89 4c 88 08              	movl	%r9d, 0x8(%rax,%rcx,4)
1407c4104: 48 89 75 d8                 	movq	%rsi, -0x28(%rbp)
1407c4108: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1407c410c: 48 89 75 e8                 	movq	%rsi, -0x18(%rbp)
1407c4110: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
1407c4117: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407c411e: f3 0f 6f 85 a0 01 00 00     	movdqu	0x1a0(%rbp), %xmm0
1407c4126: 66 0f 7f 85 c0 01 00 00     	movdqa	%xmm0, 0x1c0(%rbp)
1407c412e: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1407c4138: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
1407c413f: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1407c4146: c6 85 57 03 00 00 00        	movb	$0x0, 0x357(%rbp)
1407c414d: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
1407c4152: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1407c4159: 48 8d 55 d8                 	leaq	-0x28(%rbp), %rdx
1407c415d: 4c 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %r8
1407c4164: 4c 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %r9
1407c416b: e8 d0 ac 10 00              	callq	0x1408cee40 <.text+0x8cde40>
1407c4170: 48 c7 85 30 02 00 00 01 00 00 00    	movq	$0x1, 0x230(%rbp)
1407c417b: 48 c7 85 38 02 00 00 01 00 00 00    	movq	$0x1, 0x238(%rbp)
1407c4186: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1407c418d: 48 8d 85 40 02 00 00        	leaq	0x240(%rbp), %rax
1407c4194: 0f 29 40 70                 	movaps	%xmm0, 0x70(%rax)
1407c4198: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1407c419f: 0f 29 40 60                 	movaps	%xmm0, 0x60(%rax)
1407c41a3: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
1407c41aa: 0f 29 40 50                 	movaps	%xmm0, 0x50(%rax)
1407c41ae: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
1407c41b5: 0f 29 40 40                 	movaps	%xmm0, 0x40(%rax)
1407c41b9: 66 0f 6f 85 90 00 00 00     	movdqa	0x90(%rbp), %xmm0
1407c41c1: 66 0f 6f 8d a0 00 00 00     	movdqa	0xa0(%rbp), %xmm1
1407c41c9: 66 0f 6f 95 b0 00 00 00     	movdqa	0xb0(%rbp), %xmm2
1407c41d1: 0f 28 9d c0 00 00 00        	movaps	0xc0(%rbp), %xmm3
1407c41d8: 0f 29 58 30                 	movaps	%xmm3, 0x30(%rax)
1407c41dc: 66 0f 7f 50 20              	movdqa	%xmm2, 0x20(%rax)
1407c41e1: 66 0f 7f 48 10              	movdqa	%xmm1, 0x10(%rax)
1407c41e6: 66 0f 7f 00                 	movdqa	%xmm0, (%rax)
1407c41ea: 0f b6 05 18 0a e7 02        	movzbl	0x2e70a18(%rip), %eax   # 0x143634c09
1407c41f1: b9 90 00 00 00              	movl	$0x90, %ecx
1407c41f6: ba 10 00 00 00              	movl	$0x10, %edx
1407c41fb: e8 10 fa e4 ff              	callq	0x140613c10 <.text+0x612c10>
1407c4200: 48 85 c0                    	testq	%rax, %rax
1407c4203: 0f 84 b9 03 00 00           	je	0x1407c45c2 <.text+0x7c35c2>
1407c4209: 48 89 c6                    	movq	%rax, %rsi
1407c420c: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1407c4212: 48 89 c1                    	movq	%rax, %rcx
1407c4215: 48 8d bd 30 02 00 00        	leaq	0x230(%rbp), %rdi
1407c421c: 48 89 fa                    	movq	%rdi, %rdx
1407c421f: e8 dc f9 15 02              	callq	0x142923c00 <.text+0x2922c00>
1407c4224: f0                          	lock
1407c4225: 48 ff 06                    	incq	(%rsi)
1407c4228: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
1407c422f: 0f 8e 42 05 00 00           	jle	0x1407c4777 <.text+0x7c3777>
1407c4235: c7 85 38 02 00 00 00 00 00 00       	movl	$0x0, 0x238(%rbp)
1407c423f: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407c424a: 66 0f 6f 05 6e 9e 17 02     	movdqa	0x2179e6e(%rip), %xmm0  # 0x14293e0c0
1407c4252: 66 0f 7f 45 c0              	movdqa	%xmm0, -0x40(%rbp)
1407c4257: c6 85 57 03 00 00 00        	movb	$0x0, 0x357(%rbp)
1407c425e: 48 8d 45 c0                 	leaq	-0x40(%rbp), %rax
1407c4262: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c4267: 48 89 f2                    	movq	%rsi, %rdx
1407c426a: 49 89 f0                    	movq	%rsi, %r8
1407c426d: 49 89 f9                    	movq	%rdi, %r9
1407c4270: e8 3b 10 06 00              	callq	0x1408252b0 <.text+0x8242b0>
1407c4275: 8b 85 18 03 00 00           	movl	0x318(%rbp), %eax
1407c427b: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1407c427f: e9 e2 e5 ff ff              	jmp	0x1407c2866 <.text+0x7c1866>
1407c4284: f3 45 0f 58 f5              	addss	%xmm13, %xmm14
1407c4289: f3 44 0f 58 35 46 16 16 02  	addss	0x2161646(%rip), %xmm14 # 0x1429258d8
1407c4292: f3 0f 10 85 8c 01 00 00     	movss	0x18c(%rbp), %xmm0
1407c429a: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
1407c429e: 0f 86 c7 f6 ff ff           	jbe	0x1407c396b <.text+0x7c296b>
1407c42a4: 66 0f 6e 8d 10 02 00 00     	movd	0x210(%rbp), %xmm1
1407c42ac: 66 0f 6e 95 18 02 00 00     	movd	0x218(%rbp), %xmm2
1407c42b4: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c42bb: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
1407c42bf: e8 8c 61 aa 00              	callq	0x14126a450 <.text+0x1269450>
1407c42c4: 4c 8b 85 40 03 00 00        	movq	0x340(%rbp), %r8
1407c42cb: 49 8b 90 d0 00 00 00        	movq	0xd0(%r8), %rdx
1407c42d2: 89 c1                       	movl	%eax, %ecx
1407c42d4: 48 39 ca                    	cmpq	%rcx, %rdx
1407c42d7: 0f 86 6f 04 00 00           	jbe	0x1407c474c <.text+0x7c374c>
1407c42dd: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
1407c42e1: 48 39 d0                    	cmpq	%rdx, %rax
1407c42e4: 0f 83 77 04 00 00           	jae	0x1407c4761 <.text+0x7c3761>
1407c42ea: 49 8b 80 c8 00 00 00        	movq	0xc8(%r8), %rax
1407c42f1: f3 0f 10 0c 88              	movss	(%rax,%rcx,4), %xmm1
1407c42f6: f3 0f 10 54 88 04           	movss	0x4(%rax,%rcx,4), %xmm2
1407c42fc: f3 0f 5c d1                 	subss	%xmm1, %xmm2
1407c4300: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
1407c4304: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407c4308: 0f 2e 15 81 c3 19 02        	ucomiss	0x219c381(%rip), %xmm2  # 0x142960690
1407c430f: 0f 85 56 f6 ff ff           	jne	0x1407c396b <.text+0x7c296b>
1407c4315: 0f 8a 50 f6 ff ff           	jp	0x1407c396b <.text+0x7c296b>
1407c431b: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4322: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
1407c4329: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c432e: 48 8d 8d e8 02 00 00        	leaq	0x2e8(%rbp), %rcx
1407c4335: f3 0f 10 0d c7 15 16 02     	movss	0x21615c7(%rip), %xmm1  # 0x142925904
1407c433d: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407c4340: 0f 28 d9                    	movaps	%xmm1, %xmm3
1407c4343: e8 98 5d 31 00              	callq	0x140ada0e0 <.text+0xad90e0>
1407c4348: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1407c434f: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1407c4356: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
1407c435a: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1407c435e: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
1407c4369: 48 c7 85 70 02 00 00 00 00 00 00    	movq	$0x0, 0x270(%rbp)
1407c4374: 48 89 8d 40 03 00 00        	movq	%rcx, 0x340(%rbp)
1407c437b: 48 89 8d b0 02 00 00        	movq	%rcx, 0x2b0(%rbp)
1407c4382: 48 89 85 b8 02 00 00        	movq	%rax, 0x2b8(%rbp)
1407c4389: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
1407c4390: 48 8d 95 30 02 00 00        	leaq	0x230(%rbp), %rdx
1407c4397: 4c 8d 05 ba 22 2c 02        	leaq	0x22c22ba(%rip), %r8    # 0x142a86658
1407c439e: e8 7d b7 a9 00              	callq	0x14125fb20 <.text+0x125eb20>
1407c43a3: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
1407c43aa: 48 85 c0                    	testq	%rax, %rax
1407c43ad: 74 1a                       	je	0x1407c43c9 <.text+0x7c33c9>
1407c43af: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c43b3: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c43b7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c43bd: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c43c4: e8 57 f8 e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407c43c9: 40 b7 01                    	movb	$0x1, %dil
1407c43cc: e9 9c f5 ff ff              	jmp	0x1407c396d <.text+0x7c296d>
1407c43d1: 8b 85 50 03 00 00           	movl	0x350(%rbp), %eax
1407c43d7: 89 c1                       	movl	%eax, %ecx
1407c43d9: 4c 8b 85 20 03 00 00        	movq	0x320(%rbp), %r8
1407c43e0: 4c 8b 95 e8 01 00 00        	movq	0x1e8(%rbp), %r10
1407c43e7: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1407c43ee: 44 0f b6 9d 56 03 00 00     	movzbl	0x356(%rbp), %r11d
1407c43f6: 4c 89 85 20 03 00 00        	movq	%r8, 0x320(%rbp)
1407c43fd: 49 83 fa 02                 	cmpq	$0x2, %r10
1407c4401: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1407c4408: 0f 85 13 e2 ff ff           	jne	0x1407c2621 <.text+0x7c1621>
1407c440e: e9 ad e3 ff ff              	jmp	0x1407c27c0 <.text+0x7c17c0>
1407c4413: 48 8d 0d ce ce 45 02        	leaq	0x245cece(%rip), %rcx   # 0x142c212e8
1407c441a: e8 f1 4e 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c441f: e9 53 03 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c4424: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c442b: 48 8d 0d a6 48 46 02        	leaq	0x24648a6(%rip), %rcx   # 0x142c28cd8
1407c4432: e8 d9 4e 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c4437: e9 3b 03 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c443c: 48 85 c0                    	testq	%rax, %rax
1407c443f: 0f 85 aa 00 00 00           	jne	0x1407c44ef <.text+0x7c34ef>
1407c4445: 31 d2                       	xorl	%edx, %edx
1407c4447: e8 34 e1 12 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c444c: 48 89 c1                    	movq	%rax, %rcx
1407c444f: 48 85 c0                    	testq	%rax, %rax
1407c4452: 0f 85 d2 df ff ff           	jne	0x1407c242a <.text+0x7c142a>
1407c4458: e9 92 00 00 00              	jmp	0x1407c44ef <.text+0x7c34ef>
1407c445d: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4464: 48 8d 0d 6d 48 46 02        	leaq	0x246486d(%rip), %rcx   # 0x142c28cd8
1407c446b: e8 a0 4e 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c4470: e9 02 03 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c4475: 48 85 c0                    	testq	%rax, %rax
1407c4478: 75 75                       	jne	0x1407c44ef <.text+0x7c34ef>
1407c447a: 31 d2                       	xorl	%edx, %edx
1407c447c: e8 ff e0 12 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c4481: 48 89 c1                    	movq	%rax, %rcx
1407c4484: 48 85 c0                    	testq	%rax, %rax
1407c4487: 0f 85 30 e0 ff ff           	jne	0x1407c24bd <.text+0x7c14bd>
1407c448d: eb 60                       	jmp	0x1407c44ef <.text+0x7c34ef>
1407c448f: 48 83 bd 70 01 00 00 00     	cmpq	$0x0, 0x170(%rbp)
1407c4497: 0f 84 4e e0 ff ff           	je	0x1407c24eb <.text+0x7c14eb>
1407c449d: e8 8e 3c 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c44a2: 48 89 c1                    	movq	%rax, %rcx
1407c44a5: 48 8b 00                    	movq	(%rax), %rax
1407c44a8: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c44ac: 75 2d                       	jne	0x1407c44db <.text+0x7c34db>
1407c44ae: 48 83 c1 08                 	addq	$0x8, %rcx
1407c44b2: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c44b6: 75 43                       	jne	0x1407c44fb <.text+0x7c34fb>
1407c44b8: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c44bf: 48 89 8d 30 03 00 00        	movq	%rcx, 0x330(%rbp)
1407c44c6: 48 83 c1 08                 	addq	$0x8, %rcx
1407c44ca: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
1407c44d1: e8 1a 2c 45 00              	callq	0x140c170f0 <.text+0xc160f0>
1407c44d6: e9 06 e0 ff ff              	jmp	0x1407c24e1 <.text+0x7c14e1>
1407c44db: 48 85 c0                    	testq	%rax, %rax
1407c44de: 75 0f                       	jne	0x1407c44ef <.text+0x7c34ef>
1407c44e0: 31 d2                       	xorl	%edx, %edx
1407c44e2: e8 99 e0 12 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c44e7: 48 89 c1                    	movq	%rax, %rcx
1407c44ea: 48 85 c0                    	testq	%rax, %rax
1407c44ed: 75 c3                       	jne	0x1407c44b2 <.text+0x7c34b2>
1407c44ef: 48 8d 0d d2 ce 45 02        	leaq	0x245ced2(%rip), %rcx   # 0x142c213c8
1407c44f6: e8 d5 3b 11 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407c44fb: 48 8d 0d 36 cf 45 02        	leaq	0x245cf36(%rip), %rcx   # 0x142c21438
1407c4502: e8 49 4d 11 02              	callq	0x1428d9250 <.text+0x28d8250>
1407c4507: 48 8d 05 1a d2 2c 02        	leaq	0x22cd21a(%rip), %rax   # 0x142a91728
1407c450e: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1407c4515: 48 c7 85 38 02 00 00 01 00 00 00    	movq	$0x1, 0x238(%rbp)
1407c4520: 48 c7 85 40 02 00 00 08 00 00 00    	movq	$0x8, 0x240(%rbp)
1407c452b: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1407c452f: f3 0f 7f 85 48 02 00 00     	movdqu	%xmm0, 0x248(%rbp)
1407c4537: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c453e: 48 8d 15 f3 d1 2c 02        	leaq	0x22cd1f3(%rip), %rdx   # 0x142a91738
1407c4545: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407c454c: e8 3f 4e 11 02              	callq	0x1428d9390 <.text+0x28d8390>
1407c4551: e9 21 02 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c4556: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c455d: 48 8d 0d 2c 47 46 02        	leaq	0x246472c(%rip), %rcx   # 0x142c28c90
1407c4564: 4c 8d 05 55 47 46 02        	leaq	0x2464755(%rip), %r8    # 0x142c28cc0
1407c456b: ba 2d 00 00 00              	movl	$0x2d, %edx
1407c4570: e8 bb 4e 11 02              	callq	0x1428d9430 <.text+0x28d8430>
1407c4575: e9 fd 01 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c457a: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4581: 48 8d 0d 08 47 46 02        	leaq	0x2464708(%rip), %rcx   # 0x142c28c90
1407c4588: 4c 8d 05 31 47 46 02        	leaq	0x2464731(%rip), %r8    # 0x142c28cc0
1407c458f: ba 2d 00 00 00              	movl	$0x2d, %edx
1407c4594: e8 97 4e 11 02              	callq	0x1428d9430 <.text+0x28d8430>
1407c4599: e9 d9 01 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c459e: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c45a5: 48 8d 0d d4 21 2c 02        	leaq	0x22c21d4(%rip), %rcx   # 0x142a86780
1407c45ac: 4c 8d 05 dd 21 2c 02        	leaq	0x22c21dd(%rip), %r8    # 0x142a86790
1407c45b3: ba 0f 00 00 00              	movl	$0xf, %edx
1407c45b8: e8 73 4e 11 02              	callq	0x1428d9430 <.text+0x28d8430>
1407c45bd: e9 b5 01 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c45c2: b9 10 00 00 00              	movl	$0x10, %ecx
1407c45c7: ba 90 00 00 00              	movl	$0x90, %edx
1407c45cc: e8 2f 4a 11 02              	callq	0x1428d9000 <.text+0x28d8000>
1407c45d1: e9 a1 01 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c45d6: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c45dd: 48 8d 0d 3c 21 2c 02        	leaq	0x22c213c(%rip), %rcx   # 0x142a86720
1407c45e4: e8 27 4d 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c45e9: e9 89 01 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c45ee: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c45f5: 48 8d 0d fc 1f 2c 02        	leaq	0x22c1ffc(%rip), %rcx   # 0x142a865f8
1407c45fc: e8 0f 4d 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c4601: e9 71 01 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c4606: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1407c460d: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
1407c4614: 48 8d 05 d5 88 a9 00        	leaq	0xa988d5(%rip), %rax    # 0x14125cef0 <.text+0x125bef0>
1407c461b: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
1407c4622: 48 8d 05 57 d6 45 02        	leaq	0x245d657(%rip), %rax   # 0x142c21c80
1407c4629: 48 89 85 30 02 00 00        	movq	%rax, 0x230(%rbp)
1407c4630: 48 c7 85 38 02 00 00 01 00 00 00    	movq	$0x1, 0x238(%rbp)
1407c463b: 48 c7 85 50 02 00 00 00 00 00 00    	movq	$0x0, 0x250(%rbp)
1407c4646: 48 8d 85 e8 02 00 00        	leaq	0x2e8(%rbp), %rax
1407c464d: 48 89 85 40 02 00 00        	movq	%rax, 0x240(%rbp)
1407c4654: 48 c7 85 48 02 00 00 01 00 00 00    	movq	$0x1, 0x248(%rbp)
1407c465f: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4666: 48 8d 15 23 d6 45 02        	leaq	0x245d623(%rip), %rdx   # 0x142c21c90
1407c466d: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1407c4674: e8 17 4d 11 02              	callq	0x1428d9390 <.text+0x28d8390>
1407c4679: e9 f9 00 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c467e: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4685: 4c 8d 05 b4 1d 2c 02        	leaq	0x22c1db4(%rip), %r8    # 0x142a86440
1407c468c: b9 04 00 00 00              	movl	$0x4, %ecx
1407c4691: 48 89 da                    	movq	%rbx, %rdx
1407c4694: e8 4a 49 11 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1407c4699: e9 d9 00 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c469e: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c46a5: 4c 8d 05 3c df 45 02        	leaq	0x245df3c(%rip), %r8    # 0x142c225e8
1407c46ac: e8 67 4e 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c46b1: e9 c1 00 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c46b6: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c46bd: 4c 8d 05 3c df 45 02        	leaq	0x245df3c(%rip), %r8    # 0x142c22600
1407c46c4: 48 89 c1                    	movq	%rax, %rcx
1407c46c7: e8 4c 4e 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c46cc: e9 a6 00 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c46d1: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c46d8: b9 04 00 00 00              	movl	$0x4, %ecx
1407c46dd: ba 28 00 00 00              	movl	$0x28, %edx
1407c46e2: e8 19 49 11 02              	callq	0x1428d9000 <.text+0x28d8000>
1407c46e7: e9 8b 00 00 00              	jmp	0x1407c4777 <.text+0x7c3777>
1407c46ec: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c46f3: 4c 8d 05 2e 1f 2c 02        	leaq	0x22c1f2e(%rip), %r8    # 0x142a86628
1407c46fa: 48 89 f2                    	movq	%rsi, %rdx
1407c46fd: e8 16 4e 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c4702: eb 73                       	jmp	0x1407c4777 <.text+0x7c3777>
1407c4704: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c470b: 4c 8d 05 16 1f 2c 02        	leaq	0x22c1f16(%rip), %r8    # 0x142a86628
1407c4712: 48 89 c1                    	movq	%rax, %rcx
1407c4715: 48 89 f2                    	movq	%rsi, %rdx
1407c4718: e8 fb 4d 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c471d: eb 58                       	jmp	0x1407c4777 <.text+0x7c3777>
1407c471f: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4726: 4c 8d 05 3b 20 2c 02        	leaq	0x22c203b(%rip), %r8    # 0x142a86768
1407c472d: e8 e6 4d 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c4732: eb 43                       	jmp	0x1407c4777 <.text+0x7c3777>
1407c4734: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c473b: 4c 8d 05 26 20 2c 02        	leaq	0x22c2026(%rip), %r8    # 0x142a86768
1407c4742: 48 89 c1                    	movq	%rax, %rcx
1407c4745: e8 ce 4d 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c474a: eb 2b                       	jmp	0x1407c4777 <.text+0x7c3777>
1407c474c: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4753: 4c 8d 05 e6 1e 2c 02        	leaq	0x22c1ee6(%rip), %r8    # 0x142a86640
1407c475a: e8 b9 4d 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c475f: eb 16                       	jmp	0x1407c4777 <.text+0x7c3777>
1407c4761: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4768: 4c 8d 05 d1 1e 2c 02        	leaq	0x22c1ed1(%rip), %r8    # 0x142a86640
1407c476f: 48 89 c1                    	movq	%rax, %rcx
1407c4772: e8 a1 4d 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c4777: 0f 0b                       	ud2
1407c4779: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407c4780: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4785: 55                          	pushq	%rbp
1407c4786: 41 57                       	pushq	%r15
1407c4788: 41 56                       	pushq	%r14
1407c478a: 41 55                       	pushq	%r13
1407c478c: 41 54                       	pushq	%r12
1407c478e: 56                          	pushq	%rsi
1407c478f: 57                          	pushq	%rdi
1407c4790: 53                          	pushq	%rbx
1407c4791: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4798: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c479f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c47a5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c47ab: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c47b1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c47b7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c47bd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c47c6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c47cf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c47d8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c47e0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c47e8: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
1407c47ef: 48 85 c0                    	testq	%rax, %rax
1407c47f2: 74 1a                       	je	0x1407c480e <.text+0x7c380e>
1407c47f4: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c47f8: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c47fc: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c4802: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c4809: e8 12 f4 e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407c480e: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4815: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
1407c481e: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4826: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c482f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4838: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4841: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4847: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c484d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4853: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4859: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c485f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4866: 5b                          	popq	%rbx
1407c4867: 5f                          	popq	%rdi
1407c4868: 5e                          	popq	%rsi
1407c4869: 41 5c                       	popq	%r12
1407c486b: 41 5d                       	popq	%r13
1407c486d: 41 5e                       	popq	%r14
1407c486f: 41 5f                       	popq	%r15
1407c4871: 5d                          	popq	%rbp
1407c4872: c3                          	retq
1407c4873: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407c4880: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4885: 55                          	pushq	%rbp
1407c4886: 41 57                       	pushq	%r15
1407c4888: 41 56                       	pushq	%r14
1407c488a: 41 55                       	pushq	%r13
1407c488c: 41 54                       	pushq	%r12
1407c488e: 56                          	pushq	%rsi
1407c488f: 57                          	pushq	%rdi
1407c4890: 53                          	pushq	%rbx
1407c4891: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4898: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c489f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c48a5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c48ab: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c48b1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c48b7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c48bd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c48c6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c48cf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c48d8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c48e0: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
1407c48e9: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
1407c48f0: 48 ff 00                    	incq	(%rax)
1407c48f3: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c48fb: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4903: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c490c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4915: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c491e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4924: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c492a: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4930: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4936: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c493c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4943: 5b                          	popq	%rbx
1407c4944: 5f                          	popq	%rdi
1407c4945: 5e                          	popq	%rsi
1407c4946: 41 5c                       	popq	%r12
1407c4948: 41 5d                       	popq	%r13
1407c494a: 41 5e                       	popq	%r14
1407c494c: 41 5f                       	popq	%r15
1407c494e: 5d                          	popq	%rbp
1407c494f: c3                          	retq
1407c4950: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4955: 55                          	pushq	%rbp
1407c4956: 41 57                       	pushq	%r15
1407c4958: 41 56                       	pushq	%r14
1407c495a: 41 55                       	pushq	%r13
1407c495c: 41 54                       	pushq	%r12
1407c495e: 56                          	pushq	%rsi
1407c495f: 57                          	pushq	%rdi
1407c4960: 53                          	pushq	%rbx
1407c4961: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4968: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c496f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c4975: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c497b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c4981: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c4987: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c498d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c4996: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c499f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c49a8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c49b0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c49b8: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
1407c49bf: 48 85 c0                    	testq	%rax, %rax
1407c49c2: 74 1a                       	je	0x1407c49de <.text+0x7c39de>
1407c49c4: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c49c8: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c49cc: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c49d2: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c49d9: e8 42 f2 e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407c49de: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c49e5: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
1407c49ee: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c49f6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c49ff: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4a08: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4a11: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4a17: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c4a1d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4a23: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4a29: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c4a2f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4a36: 5b                          	popq	%rbx
1407c4a37: 5f                          	popq	%rdi
1407c4a38: 5e                          	popq	%rsi
1407c4a39: 41 5c                       	popq	%r12
1407c4a3b: 41 5d                       	popq	%r13
1407c4a3d: 41 5e                       	popq	%r14
1407c4a3f: 41 5f                       	popq	%r15
1407c4a41: 5d                          	popq	%rbp
1407c4a42: c3                          	retq
1407c4a43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407c4a50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4a55: 55                          	pushq	%rbp
1407c4a56: 41 57                       	pushq	%r15
1407c4a58: 41 56                       	pushq	%r14
1407c4a5a: 41 55                       	pushq	%r13
1407c4a5c: 41 54                       	pushq	%r12
1407c4a5e: 56                          	pushq	%rsi
1407c4a5f: 57                          	pushq	%rdi
1407c4a60: 53                          	pushq	%rbx
1407c4a61: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4a68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c4a6f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c4a75: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c4a7b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c4a81: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c4a87: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c4a8d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c4a96: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c4a9f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c4aa8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c4ab0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c4ab8: 48 8b 85 90 00 00 00        	movq	0x90(%rbp), %rax
1407c4abf: 48 85 c0                    	testq	%rax, %rax
1407c4ac2: 74 1a                       	je	0x1407c4ade <.text+0x7c3ade>
1407c4ac4: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c4ac8: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c4acc: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c4ad2: 48 8b 8d 40 03 00 00        	movq	0x340(%rbp), %rcx
1407c4ad9: e8 42 f1 e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407c4ade: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4ae5: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
1407c4aee: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4af6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c4aff: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4b08: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4b11: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4b17: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c4b1d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4b23: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4b29: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c4b2f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4b36: 5b                          	popq	%rbx
1407c4b37: 5f                          	popq	%rdi
1407c4b38: 5e                          	popq	%rsi
1407c4b39: 41 5c                       	popq	%r12
1407c4b3b: 41 5d                       	popq	%r13
1407c4b3d: 41 5e                       	popq	%r14
1407c4b3f: 41 5f                       	popq	%r15
1407c4b41: 5d                          	popq	%rbp
1407c4b42: c3                          	retq
1407c4b43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407c4b50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4b55: 55                          	pushq	%rbp
1407c4b56: 41 57                       	pushq	%r15
1407c4b58: 41 56                       	pushq	%r14
1407c4b5a: 41 55                       	pushq	%r13
1407c4b5c: 41 54                       	pushq	%r12
1407c4b5e: 56                          	pushq	%rsi
1407c4b5f: 57                          	pushq	%rdi
1407c4b60: 53                          	pushq	%rbx
1407c4b61: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4b68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c4b6f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c4b75: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c4b7b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c4b81: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c4b87: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c4b8d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c4b96: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c4b9f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c4ba8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c4bb0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c4bb8: 48 8b 85 e8 02 00 00        	movq	0x2e8(%rbp), %rax
1407c4bbf: 48 85 c0                    	testq	%rax, %rax
1407c4bc2: 74 1a                       	je	0x1407c4bde <.text+0x7c3bde>
1407c4bc4: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c4bc8: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1407c4bcc: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c4bd2: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
1407c4bd9: e8 42 f0 e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407c4bde: c6 85 57 03 00 00 01        	movb	$0x1, 0x357(%rbp)
1407c4be5: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
1407c4bee: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4bf6: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c4bff: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4c08: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4c11: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4c17: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c4c1d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4c23: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4c29: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c4c2f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4c36: 5b                          	popq	%rbx
1407c4c37: 5f                          	popq	%rdi
1407c4c38: 5e                          	popq	%rsi
1407c4c39: 41 5c                       	popq	%r12
1407c4c3b: 41 5d                       	popq	%r13
1407c4c3d: 41 5e                       	popq	%r14
1407c4c3f: 41 5f                       	popq	%r15
1407c4c41: 5d                          	popq	%rbp
1407c4c42: c3                          	retq
1407c4c43: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407c4c50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4c55: 55                          	pushq	%rbp
1407c4c56: 41 57                       	pushq	%r15
1407c4c58: 41 56                       	pushq	%r14
1407c4c5a: 41 55                       	pushq	%r13
1407c4c5c: 41 54                       	pushq	%r12
1407c4c5e: 56                          	pushq	%rsi
1407c4c5f: 57                          	pushq	%rdi
1407c4c60: 53                          	pushq	%rbx
1407c4c61: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4c68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c4c6f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c4c75: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c4c7b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c4c81: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c4c87: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c4c8d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c4c96: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c4c9f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c4ca8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c4cb0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c4cb8: c6 85 57 03 00 00 00        	movb	$0x0, 0x357(%rbp)
1407c4cbf: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1407c4cc6: e8 15 cf e3 ff              	callq	0x140601be0 <.text+0x600be0>
1407c4ccb: c6 85 57 03 00 00 00        	movb	$0x0, 0x357(%rbp)
1407c4cd2: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
1407c4cdb: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4ce3: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c4cec: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4cf5: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4cfe: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4d04: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c4d0a: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4d10: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4d16: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c4d1c: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4d23: 5b                          	popq	%rbx
1407c4d24: 5f                          	popq	%rdi
1407c4d25: 5e                          	popq	%rsi
1407c4d26: 41 5c                       	popq	%r12
1407c4d28: 41 5d                       	popq	%r13
1407c4d2a: 41 5e                       	popq	%r14
1407c4d2c: 41 5f                       	popq	%r15
1407c4d2e: 5d                          	popq	%rbp
1407c4d2f: c3                          	retq
1407c4d30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4d35: 55                          	pushq	%rbp
1407c4d36: 41 57                       	pushq	%r15
1407c4d38: 41 56                       	pushq	%r14
1407c4d3a: 41 55                       	pushq	%r13
1407c4d3c: 41 54                       	pushq	%r12
1407c4d3e: 56                          	pushq	%rsi
1407c4d3f: 57                          	pushq	%rdi
1407c4d40: 53                          	pushq	%rbx
1407c4d41: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4d48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c4d4f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c4d55: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c4d5b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c4d61: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c4d67: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c4d6d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c4d76: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c4d7f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c4d88: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c4d90: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
1407c4d99: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
1407c4da0: 48 ff 00                    	incq	(%rax)
1407c4da3: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c4dab: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4db3: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c4dbc: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4dc5: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4dce: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4dd4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c4dda: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4de0: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4de6: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c4dec: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4df3: 5b                          	popq	%rbx
1407c4df4: 5f                          	popq	%rdi
1407c4df5: 5e                          	popq	%rsi
1407c4df6: 41 5c                       	popq	%r12
1407c4df8: 41 5d                       	popq	%r13
1407c4dfa: 41 5e                       	popq	%r14
1407c4dfc: 41 5f                       	popq	%r15
1407c4dfe: 5d                          	popq	%rbp
1407c4dff: c3                          	retq
1407c4e00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4e05: 55                          	pushq	%rbp
1407c4e06: 41 57                       	pushq	%r15
1407c4e08: 41 56                       	pushq	%r14
1407c4e0a: 41 55                       	pushq	%r13
1407c4e0c: 41 54                       	pushq	%r12
1407c4e0e: 56                          	pushq	%rsi
1407c4e0f: 57                          	pushq	%rdi
1407c4e10: 53                          	pushq	%rbx
1407c4e11: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4e18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c4e1f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c4e25: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c4e2b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c4e31: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c4e37: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c4e3d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c4e46: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c4e4f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c4e58: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c4e60: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
1407c4e69: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
1407c4e70: 48 ff 00                    	incq	(%rax)
1407c4e73: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c4e7b: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4e83: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c4e8c: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4e95: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4e9e: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4ea4: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c4eaa: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4eb0: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4eb6: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c4ebc: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4ec3: 5b                          	popq	%rbx
1407c4ec4: 5f                          	popq	%rdi
1407c4ec5: 5e                          	popq	%rsi
1407c4ec6: 41 5c                       	popq	%r12
1407c4ec8: 41 5d                       	popq	%r13
1407c4eca: 41 5e                       	popq	%r14
1407c4ecc: 41 5f                       	popq	%r15
1407c4ece: 5d                          	popq	%rbp
1407c4ecf: c3                          	retq
1407c4ed0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c4ed5: 55                          	pushq	%rbp
1407c4ed6: 41 57                       	pushq	%r15
1407c4ed8: 41 56                       	pushq	%r14
1407c4eda: 41 55                       	pushq	%r13
1407c4edc: 41 54                       	pushq	%r12
1407c4ede: 56                          	pushq	%rsi
1407c4edf: 57                          	pushq	%rdi
1407c4ee0: 53                          	pushq	%rbx
1407c4ee1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c4ee8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c4eef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c4ef5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c4efb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c4f01: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c4f07: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c4f0d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c4f16: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c4f1f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c4f28: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c4f30: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
1407c4f39: 80 bd 57 03 00 00 00        	cmpb	$0x0, 0x357(%rbp)
1407c4f40: 0f 84 84 00 00 00           	je	0x1407c4fca <.text+0x7c3fca>
1407c4f46: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1407c4f4d: 48 85 c0                    	testq	%rax, %rax
1407c4f50: 74 1a                       	je	0x1407c4f6c <.text+0x7c3f6c>
1407c4f52: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
1407c4f59: 48 c1 e0 02                 	shlq	$0x2, %rax
1407c4f5d: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1407c4f61: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c4f67: e8 b4 ec e4 ff              	callq	0x140613c20 <.text+0x612c20>
1407c4f6c: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
1407c4f75: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4f7d: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c4f86: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4f8f: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4f98: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4f9e: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c4fa4: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c4faa: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c4fb0: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c4fb6: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c4fbd: 5b                          	popq	%rbx
1407c4fbe: 5f                          	popq	%rdi
1407c4fbf: 5e                          	popq	%rsi
1407c4fc0: 41 5c                       	popq	%r12
1407c4fc2: 41 5d                       	popq	%r13
1407c4fc4: 41 5e                       	popq	%r14
1407c4fc6: 41 5f                       	popq	%r15
1407c4fc8: 5d                          	popq	%rbp
1407c4fc9: c3                          	retq
1407c4fca: 66 0f 6f b4 24 c0 00 00 00  	movdqa	0xc0(%rsp), %xmm6
1407c4fd3: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c4fdb: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c4fe4: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c4fed: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c4ff6: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c4ffc: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c5002: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c5008: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c500e: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c5014: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c501b: 5b                          	popq	%rbx
1407c501c: 5f                          	popq	%rdi
1407c501d: 5e                          	popq	%rsi
1407c501e: 41 5c                       	popq	%r12
1407c5020: 41 5d                       	popq	%r13
1407c5022: 41 5e                       	popq	%r14
1407c5024: 41 5f                       	popq	%r15
1407c5026: 5d                          	popq	%rbp
1407c5027: c3                          	retq
1407c5028: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407c5030: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c5035: 55                          	pushq	%rbp
1407c5036: 41 57                       	pushq	%r15
1407c5038: 41 56                       	pushq	%r14
1407c503a: 41 55                       	pushq	%r13
1407c503c: 41 54                       	pushq	%r12
1407c503e: 56                          	pushq	%rsi
1407c503f: 57                          	pushq	%rdi
1407c5040: 53                          	pushq	%rbx
1407c5041: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c5048: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c504f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c5055: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c505b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c5061: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c5067: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c506d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c5076: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c507f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c5088: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c5090: 66 0f 7f b4 24 c0 00 00 00  	movdqa	%xmm6, 0xc0(%rsp)
1407c5099: 48 83 bd 70 01 00 00 00     	cmpq	$0x0, 0x170(%rbp)
1407c50a1: 74 2f                       	je	0x1407c50d2 <.text+0x7c40d2>
1407c50a3: 48 8d 85 78 01 00 00        	leaq	0x178(%rbp), %rax
1407c50aa: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1407c50b1: 48 8d 85 90 01 00 00        	leaq	0x190(%rbp), %rax
1407c50b8: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1407c50bf: 48 8d 0d e2 13 2c 02        	leaq	0x22c13e2(%rip), %rcx   # 0x142a864a8
1407c50c6: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
1407c50cd: e8 ee 2a a9 00              	callq	0x141257bc0 <.text+0x1256bc0>
1407c50d2: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c50da: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c50e2: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c50eb: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c50f4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c50fd: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c5103: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c5109: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c510f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c5115: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c511b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c5122: 5b                          	popq	%rbx
1407c5123: 5f                          	popq	%rdi
1407c5124: 5e                          	popq	%rsi
1407c5125: 41 5c                       	popq	%r12
1407c5127: 41 5d                       	popq	%r13
1407c5129: 41 5e                       	popq	%r14
1407c512b: 41 5f                       	popq	%r15
1407c512d: 5d                          	popq	%rbp
1407c512e: c3                          	retq
