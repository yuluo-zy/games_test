
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1416f24f0: 41 57                       	pushq	%r15
1416f24f2: 41 56                       	pushq	%r14
1416f24f4: 41 55                       	pushq	%r13
1416f24f6: 41 54                       	pushq	%r12
1416f24f8: 56                          	pushq	%rsi
1416f24f9: 57                          	pushq	%rdi
1416f24fa: 55                          	pushq	%rbp
1416f24fb: 53                          	pushq	%rbx
1416f24fc: 48 83 ec 68                 	subq	$0x68, %rsp
1416f2500: 48 8b 01                    	movq	(%rcx), %rax
1416f2503: 4c 8b 41 08                 	movq	0x8(%rcx), %r8
1416f2507: 8b 49 14                    	movl	0x14(%rcx), %ecx
1416f250a: 4c 8b 88 f8 00 00 00        	movq	0xf8(%rax), %r9
1416f2511: 4c 8b 90 00 01 00 00        	movq	0x100(%rax), %r10
1416f2518: 4f 8d 14 91                 	leaq	(%r9,%r10,4), %r10
1416f251c: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
1416f2521: 0f b6 80 08 01 00 00        	movzbl	0x108(%rax), %eax
1416f2528: 88 44 24 0f                 	movb	%al, 0xf(%rsp)
1416f252c: 48 8b 72 08                 	movq	0x8(%rdx), %rsi
1416f2530: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
1416f2534: 48 8d 14 92                 	leaq	(%rdx,%rdx,4), %rdx
1416f2538: 48 8d 04 d6                 	leaq	(%rsi,%rdx,8), %rax
1416f253c: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
1416f2541: b8 08 00 00 00              	movl	$0x8, %eax
1416f2546: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
1416f254b: 48 c7 44 24 28 00 00 00 00  	movq	$0x0, 0x28(%rsp)
1416f2554: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1416f255d: 31 ed                       	xorl	%ebp, %ebp
1416f255f: 31 ff                       	xorl	%edi, %edi
1416f2561: 31 d2                       	xorl	%edx, %edx
1416f2563: 80 7c 24 0f 00              	cmpb	$0x0, 0xf(%rsp)
1416f2568: 0f 84 d2 00 00 00           	je	0x1416f2640 <.text+0x16f1640>
1416f256e: 48 39 fa                    	cmpq	%rdi, %rdx
1416f2571: 0f 85 ba 00 00 00           	jne	0x1416f2631 <.text+0x16f1631>
1416f2577: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1416f2580: 4d 39 d1                    	cmpq	%r10, %r9
1416f2583: 0f 84 5b 02 00 00           	je	0x1416f27e4 <.text+0x16f17e4>
1416f2589: 45 8b 19                    	movl	(%r9), %r11d
1416f258c: 49 83 c1 04                 	addq	$0x4, %r9
1416f2590: 49 8b 90 a8 01 00 00        	movq	0x1a8(%r8), %rdx
1416f2597: 4f 8d 1c db                 	leaq	(%r11,%r11,8), %r11
1416f259b: 4a 8b 7c da 10              	movq	0x10(%rdx,%r11,8), %rdi
1416f25a0: 48 85 ff                    	testq	%rdi, %rdi
1416f25a3: 74 db                       	je	0x1416f2580 <.text+0x16f1580>
1416f25a5: 4a 8d 14 da                 	leaq	(%rdx,%r11,8), %rdx
1416f25a9: 4c 8b 5a 18                 	movq	0x18(%rdx), %r11
1416f25ad: 48 8b 52 38                 	movq	0x38(%rdx), %rdx
1416f25b1: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
1416f25b6: 48 8b 98 10 01 00 00        	movq	0x110(%rax), %rbx
1416f25bd: 4c 8b b0 18 01 00 00        	movq	0x118(%rax), %r14
1416f25c4: 48 8b 1c da                 	movq	(%rdx,%rbx,8), %rbx
1416f25c8: 48 f7 d3                    	notq	%rbx
1416f25cb: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
1416f25cf: 48 c1 e3 04                 	shlq	$0x4, %rbx
1416f25d3: 4d 8b 7c 1b 10              	movq	0x10(%r11,%rbx), %r15
1416f25d8: 4c 89 7c 24 30              	movq	%r15, 0x30(%rsp)
1416f25dd: 49 8b 5c 1b 28              	movq	0x28(%r11,%rbx), %rbx
1416f25e2: 48 89 5c 24 48              	movq	%rbx, 0x48(%rsp)
1416f25e7: 4a 8b 1c f2                 	movq	(%rdx,%r14,8), %rbx
1416f25eb: 48 f7 d3                    	notq	%rbx
1416f25ee: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
1416f25f2: 48 c1 e3 04                 	shlq	$0x4, %rbx
1416f25f6: 4d 8b 74 1b 10              	movq	0x10(%r11,%rbx), %r14
1416f25fb: 4c 89 74 24 28              	movq	%r14, 0x28(%rsp)
1416f2600: 49 8b 5c 1b 28              	movq	0x28(%r11,%rbx), %rbx
1416f2605: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1416f260a: 48 8b 98 20 01 00 00        	movq	0x120(%rax), %rbx
1416f2611: 48 8b 14 da                 	movq	(%rdx,%rbx,8), %rdx
1416f2615: 48 f7 d2                    	notq	%rdx
1416f2618: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1416f261c: 48 c1 e2 04                 	shlq	$0x4, %rdx
1416f2620: 49 8b 6c 13 10              	movq	0x10(%r11,%rdx), %rbp
1416f2625: 49 8b 44 13 28              	movq	0x28(%r11,%rdx), %rax
1416f262a: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1416f262f: 31 d2                       	xorl	%edx, %edx
1416f2631: 41 89 d7                    	movl	%edx, %r15d
1416f2634: e9 f9 00 00 00              	jmp	0x1416f2732 <.text+0x16f1732>
1416f2639: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1416f2640: 48 39 fa                    	cmpq	%rdi, %rdx
1416f2643: 0f 85 d8 00 00 00           	jne	0x1416f2721 <.text+0x16f1721>
1416f2649: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1416f2650: 4d 39 d1                    	cmpq	%r10, %r9
1416f2653: 0f 84 8b 01 00 00           	je	0x1416f27e4 <.text+0x16f17e4>
1416f2659: 45 8b 19                    	movl	(%r9), %r11d
1416f265c: 49 83 c1 04                 	addq	$0x4, %r9
1416f2660: 49 8b 90 00 01 00 00        	movq	0x100(%r8), %rdx
1416f2667: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
1416f266b: 49 c1 e3 05                 	shlq	$0x5, %r11
1416f266f: 4a 8b 7c 1a 58              	movq	0x58(%rdx,%r11), %rdi
1416f2674: 48 85 ff                    	testq	%rdi, %rdi
1416f2677: 74 d7                       	je	0x1416f2650 <.text+0x16f1650>
1416f2679: 4c 01 da                    	addq	%r11, %rdx
1416f267c: 44 8b 9a 94 00 00 00        	movl	0x94(%rdx), %r11d
1416f2683: 49 8b 98 a8 01 00 00        	movq	0x1a8(%r8), %rbx
1416f268a: 4f 8d 1c db                 	leaq	(%r11,%r11,8), %r11
1416f268e: 4e 8b 74 db 18              	movq	0x18(%rbx,%r11,8), %r14
1416f2693: 4e 8b 5c db 38              	movq	0x38(%rbx,%r11,8), %r11
1416f2698: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
1416f269d: 48 8b 98 10 01 00 00        	movq	0x110(%rax), %rbx
1416f26a4: 4c 8b b8 18 01 00 00        	movq	0x118(%rax), %r15
1416f26ab: 49 8b 1c db                 	movq	(%r11,%rbx,8), %rbx
1416f26af: 48 f7 d3                    	notq	%rbx
1416f26b2: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
1416f26b6: 48 c1 e3 04                 	shlq	$0x4, %rbx
1416f26ba: 4d 8b 64 1e 10              	movq	0x10(%r14,%rbx), %r12
1416f26bf: 4c 89 64 24 30              	movq	%r12, 0x30(%rsp)
1416f26c4: 49 8b 5c 1e 28              	movq	0x28(%r14,%rbx), %rbx
1416f26c9: 48 89 5c 24 48              	movq	%rbx, 0x48(%rsp)
1416f26ce: 4b 8b 1c fb                 	movq	(%r11,%r15,8), %rbx
1416f26d2: 48 f7 d3                    	notq	%rbx
1416f26d5: 48 8d 1c 5b                 	leaq	(%rbx,%rbx,2), %rbx
1416f26d9: 48 c1 e3 04                 	shlq	$0x4, %rbx
1416f26dd: 4d 8b 7c 1e 10              	movq	0x10(%r14,%rbx), %r15
1416f26e2: 4c 89 7c 24 28              	movq	%r15, 0x28(%rsp)
1416f26e7: 49 8b 5c 1e 28              	movq	0x28(%r14,%rbx), %rbx
1416f26ec: 48 89 5c 24 40              	movq	%rbx, 0x40(%rsp)
1416f26f1: 48 8b 98 20 01 00 00        	movq	0x120(%rax), %rbx
1416f26f8: 4d 8b 1c db                 	movq	(%r11,%rbx,8), %r11
1416f26fc: 49 f7 d3                    	notq	%r11
1416f26ff: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
1416f2703: 49 c1 e3 04                 	shlq	$0x4, %r11
1416f2707: 4b 8b 6c 1e 10              	movq	0x10(%r14,%r11), %rbp
1416f270c: 4b 8b 44 1e 28              	movq	0x28(%r14,%r11), %rax
1416f2711: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1416f2716: 48 8b 42 50                 	movq	0x50(%rdx), %rax
1416f271a: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
1416f271f: 31 d2                       	xorl	%edx, %edx
1416f2721: 49 89 d3                    	movq	%rdx, %r11
1416f2724: 49 c1 e3 04                 	shlq	$0x4, %r11
1416f2728: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
1416f272d: 46 8b 7c 18 08              	movl	0x8(%rax,%r11), %r15d
1416f2732: 4f 8d 2c 7f                 	leaq	(%r15,%r15,2), %r13
1416f2736: 49 c1 e5 04                 	shlq	$0x4, %r13
1416f273a: 4c 03 6c 24 30              	addq	0x30(%rsp), %r13
1416f273f: 48 8b 44 24 48              	movq	0x48(%rsp), %rax
1416f2744: 4e 8d 34 b8                 	leaq	(%rax,%r15,4), %r14
1416f2748: 4d 6b df 68                 	imulq	$0x68, %r15, %r11
1416f274c: 4c 03 5c 24 28              	addq	0x28(%rsp), %r11
1416f2751: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
1416f2756: 4a 8d 1c b8                 	leaq	(%rax,%r15,4), %rbx
1416f275a: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
1416f275f: 4e 8d 24 b8                 	leaq	(%rax,%r15,4), %r12
1416f2763: 49 c1 e7 04                 	shlq	$0x4, %r15
1416f2767: 49 01 ef                    	addq	%rbp, %r15
1416f276a: 48 3b 74 24 60              	cmpq	0x60(%rsp), %rsi
1416f276f: 74 73                       	je	0x1416f27e4 <.text+0x16f17e4>
1416f2771: 48 ff c2                    	incq	%rdx
1416f2774: 8b 46 24                    	movl	0x24(%rsi), %eax
1416f2777: 89 44 24 18                 	movl	%eax, 0x18(%rsp)
1416f277b: 48 8b 46 1c                 	movq	0x1c(%rsi), %rax
1416f277f: 48 89 44 24 10              	movq	%rax, 0x10(%rsp)
1416f2784: 41 89 0e                    	movl	%ecx, (%r14)
1416f2787: 8b 44 24 18                 	movl	0x18(%rsp), %eax
1416f278b: 41 89 45 18                 	movl	%eax, 0x18(%r13)
1416f278f: 48 8b 44 24 10              	movq	0x10(%rsp), %rax
1416f2794: 49 89 45 10                 	movq	%rax, 0x10(%r13)
1416f2798: 0f 10 06                    	movups	(%rsi), %xmm0
1416f279b: 0f 29 44 24 10              	movaps	%xmm0, 0x10(%rsp)
1416f27a0: 41 89 0c 24                 	movl	%ecx, (%r12)
1416f27a4: 0f 28 44 24 10              	movaps	0x10(%rsp), %xmm0
1416f27a9: 41 0f 11 07                 	movups	%xmm0, (%r15)
1416f27ad: 8b 46 18                    	movl	0x18(%rsi), %eax
1416f27b0: 89 44 24 18                 	movl	%eax, 0x18(%rsp)
1416f27b4: 48 8b 46 10                 	movq	0x10(%rsi), %rax
1416f27b8: 48 83 c6 28                 	addq	$0x28, %rsi
1416f27bc: 48 89 44 24 10              	movq	%rax, 0x10(%rsp)
1416f27c1: 89 0b                       	movl	%ecx, (%rbx)
1416f27c3: 8b 44 24 18                 	movl	0x18(%rsp), %eax
1416f27c7: 41 89 43 38                 	movl	%eax, 0x38(%r11)
1416f27cb: 48 8b 44 24 10              	movq	0x10(%rsp), %rax
1416f27d0: 49 89 43 30                 	movq	%rax, 0x30(%r11)
1416f27d4: 80 7c 24 0f 00              	cmpb	$0x0, 0xf(%rsp)
1416f27d9: 0f 84 61 fe ff ff           	je	0x1416f2640 <.text+0x16f1640>
1416f27df: e9 8a fd ff ff              	jmp	0x1416f256e <.text+0x16f156e>
1416f27e4: 48 83 c4 68                 	addq	$0x68, %rsp
1416f27e8: 5b                          	popq	%rbx
1416f27e9: 5d                          	popq	%rbp
1416f27ea: 5f                          	popq	%rdi
1416f27eb: 5e                          	popq	%rsi
1416f27ec: 41 5c                       	popq	%r12
1416f27ee: 41 5d                       	popq	%r13
1416f27f0: 41 5e                       	popq	%r14
1416f27f2: 41 5f                       	popq	%r15
1416f27f4: c3                          	retq
