
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407af340: 55                          	pushq	%rbp
1407af341: 41 57                       	pushq	%r15
1407af343: 41 56                       	pushq	%r14
1407af345: 41 55                       	pushq	%r13
1407af347: 41 54                       	pushq	%r12
1407af349: 56                          	pushq	%rsi
1407af34a: 57                          	pushq	%rdi
1407af34b: 53                          	pushq	%rbx
1407af34c: 48 81 ec 48 06 00 00        	subq	$0x648, %rsp            # imm = 0x648
1407af353: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407af35b: 44 0f 29 bd b0 05 00 00     	movaps	%xmm15, 0x5b0(%rbp)
1407af363: 44 0f 29 b5 a0 05 00 00     	movaps	%xmm14, 0x5a0(%rbp)
1407af36b: 66 44 0f 7f ad 90 05 00 00  	movdqa	%xmm13, 0x590(%rbp)
1407af374: 44 0f 29 a5 80 05 00 00     	movaps	%xmm12, 0x580(%rbp)
1407af37c: 44 0f 29 9d 70 05 00 00     	movaps	%xmm11, 0x570(%rbp)
1407af384: 44 0f 29 95 60 05 00 00     	movaps	%xmm10, 0x560(%rbp)
1407af38c: 44 0f 29 8d 50 05 00 00     	movaps	%xmm9, 0x550(%rbp)
1407af394: 44 0f 29 85 40 05 00 00     	movaps	%xmm8, 0x540(%rbp)
1407af39c: 0f 29 bd 30 05 00 00        	movaps	%xmm7, 0x530(%rbp)
1407af3a3: 0f 29 b5 20 05 00 00        	movaps	%xmm6, 0x520(%rbp)
1407af3aa: 48 c7 85 18 05 00 00 fe ff ff ff    	movq	$-0x2, 0x518(%rbp)
1407af3b5: 4c 89 8d 08 05 00 00        	movq	%r9, 0x508(%rbp)
1407af3bc: 4c 89 85 a0 04 00 00        	movq	%r8, 0x4a0(%rbp)
1407af3c3: 48 89 95 f0 04 00 00        	movq	%rdx, 0x4f0(%rbp)
1407af3ca: 49 89 ce                    	movq	%rcx, %r14
1407af3cd: 4c 8b bd 68 06 00 00        	movq	0x668(%rbp), %r15
1407af3d4: 48 8b 85 50 06 00 00        	movq	0x650(%rbp), %rax
1407af3db: 48 8b 8d 48 06 00 00        	movq	0x648(%rbp), %rcx
1407af3e2: 48 8b 31                    	movq	(%rcx), %rsi
1407af3e5: 48 8b 79 20                 	movq	0x20(%rcx), %rdi
1407af3e9: 48 8b 59 40                 	movq	0x40(%rcx), %rbx
1407af3ed: 4c 8b 61 60                 	movq	0x60(%rcx), %r12
1407af3f1: 4c 8b a9 80 00 00 00        	movq	0x80(%rcx), %r13
1407af3f8: 0f 10 81 a0 00 00 00        	movups	0xa0(%rcx), %xmm0
1407af3ff: 0f 10 89 b0 00 00 00        	movups	0xb0(%rcx), %xmm1
1407af406: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
1407af40a: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1407af40e: 49 8b 4f 28                 	movq	0x28(%r15), %rcx
1407af412: 48 89 8d c0 04 00 00        	movq	%rcx, 0x4c0(%rbp)
1407af419: 49 8b 4f 30                 	movq	0x30(%r15), %rcx
1407af41d: 48 89 8d d0 04 00 00        	movq	%rcx, 0x4d0(%rbp)
1407af424: 48 8b 08                    	movq	(%rax), %rcx
1407af427: 49 8b 47 50                 	movq	0x50(%r15), %rax
1407af42b: 48 89 85 b8 04 00 00        	movq	%rax, 0x4b8(%rbp)
1407af432: 49 8b 47 58                 	movq	0x58(%r15), %rax
1407af436: 48 89 85 e8 04 00 00        	movq	%rax, 0x4e8(%rbp)
1407af43d: 49 8b 47 78                 	movq	0x78(%r15), %rax
1407af441: 48 89 85 50 03 00 00        	movq	%rax, 0x350(%rbp)
1407af448: 49 8b 87 88 00 00 00        	movq	0x88(%r15), %rax
1407af44f: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
1407af456: 41 8b 87 94 00 00 00        	movl	0x94(%r15), %eax
1407af45d: 89 85 4c 03 00 00           	movl	%eax, 0x34c(%rbp)
1407af463: e8 58 07 19 00              	callq	0x14093fbc0 <.text+0x93ebc0>
1407af468: 84 c0                       	testb	%al, %al
1407af46a: 0f 85 d9 1e 00 00           	jne	0x1407b1349 <.text+0x7b0349>
1407af470: 49 8b 06                    	movq	(%r14), %rax
1407af473: 48 89 85 f8 04 00 00        	movq	%rax, 0x4f8(%rbp)
1407af47a: 8b 00                       	movl	(%rax), %eax
1407af47c: 8d 50 fa                    	leal	-0x6(%rax), %edx
1407af47f: 49 89 c0                    	movq	%rax, %r8
1407af482: 49 83 c0 fb                 	addq	$-0x5, %r8
1407af486: 31 c9                       	xorl	%ecx, %ecx
1407af488: 83 fa 0f                    	cmpl	$0xf, %edx
1407af48b: 49 0f 42 c8                 	cmovbq	%r8, %rcx
1407af48f: 48 85 c9                    	testq	%rcx, %rcx
1407af492: 74 33                       	je	0x1407af4c7 <.text+0x7ae4c7>
1407af494: 48 83 f9 06                 	cmpq	$0x6, %rcx
1407af498: 74 3c                       	je	0x1407af4d6 <.text+0x7ae4d6>
1407af49a: 48 83 f9 0f                 	cmpq	$0xf, %rcx
1407af49e: 74 36                       	je	0x1407af4d6 <.text+0x7ae4d6>
1407af4a0: 48 8b 95 08 05 00 00        	movq	0x508(%rbp), %rdx
1407af4a7: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1407af4aa: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407af4ae: 89 01                       	movl	%eax, (%rcx)
1407af4b0: 48 8b 02                    	movq	(%rdx), %rax
1407af4b3: 48 c7 40 08 00 00 00 00     	movq	$0x0, 0x8(%rax)
1407af4bb: 48 c7 00 06 00 00 00        	movq	$0x6, (%rax)
1407af4c2: e9 82 1e 00 00              	jmp	0x1407b1349 <.text+0x7b0349>
1407af4c7: 83 f8 04                    	cmpl	$0x4, %eax
1407af4ca: 77 d4                       	ja	0x1407af4a0 <.text+0x7ae4a0>
1407af4cc: b9 19 00 00 00              	movl	$0x19, %ecx
1407af4d1: 0f a3 c1                    	btl	%eax, %ecx
1407af4d4: 73 ca                       	jae	0x1407af4a0 <.text+0x7ae4a0>
1407af4d6: 4c 89 ad e8 02 00 00        	movq	%r13, 0x2e8(%rbp)
1407af4dd: 4c 89 a5 68 01 00 00        	movq	%r12, 0x168(%rbp)
1407af4e4: 48 89 9d f0 02 00 00        	movq	%rbx, 0x2f0(%rbp)
1407af4eb: 48 89 bd 60 01 00 00        	movq	%rdi, 0x160(%rbp)
1407af4f2: 48 89 b5 20 04 00 00        	movq	%rsi, 0x420(%rbp)
1407af4f9: 48 8b 85 30 06 00 00        	movq	0x630(%rbp), %rax
1407af500: 48 8b 30                    	movq	(%rax), %rsi
1407af503: c7 85 c4 01 00 00 ff ff 7f 7f       	movl	$0x7f7fffff, 0x1c4(%rbp) # imm = 0x7F7FFFFF
1407af50d: c6 85 c0 01 00 00 05        	movb	$0x5, 0x1c0(%rbp)
1407af514: c6 85 00 03 00 00 00        	movb	$0x0, 0x300(%rbp)
1407af51b: 0f b6 85 00 03 00 00        	movzbl	0x300(%rbp), %eax
1407af522: 88 85 30 04 00 00           	movb	%al, 0x430(%rbp)
1407af528: 48 8b 85 01 03 00 00        	movq	0x301(%rbp), %rax
1407af52f: 48 89 85 31 04 00 00        	movq	%rax, 0x431(%rbp)
1407af536: 8b 85 09 03 00 00           	movl	0x309(%rbp), %eax
1407af53c: 89 85 39 04 00 00           	movl	%eax, 0x439(%rbp)
1407af542: 0f b7 85 0d 03 00 00        	movzwl	0x30d(%rbp), %eax
1407af549: 66 89 85 3d 04 00 00        	movw	%ax, 0x43d(%rbp)
1407af550: 0f b6 85 0f 03 00 00        	movzbl	0x30f(%rbp), %eax
1407af557: 88 85 3f 04 00 00           	movb	%al, 0x43f(%rbp)
1407af55d: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407af564: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407af56b: 4c 8d 85 30 04 00 00        	leaq	0x430(%rbp), %r8
1407af572: e8 99 98 1f 00              	callq	0x1409a8e10 <.text+0x9a7e10>
1407af577: c6 85 06 05 00 00 01        	movb	$0x1, 0x506(%rbp)
1407af57e: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1407af585: e8 26 9a 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407af58a: c6 85 06 05 00 00 00        	movb	$0x0, 0x506(%rbp)
1407af591: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407af598: 48 8d 95 70 03 00 00        	leaq	0x370(%rbp), %rdx
1407af59f: 4c 8d 85 30 04 00 00        	leaq	0x430(%rbp), %r8
1407af5a6: e8 d5 98 1f 00              	callq	0x1409a8e80 <.text+0x9a7e80>
1407af5ab: 31 ff                       	xorl	%edi, %edi
1407af5ad: 48 8d 9d 70 03 00 00        	leaq	0x370(%rbp), %rbx
1407af5b4: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407af5bb: 48 89 d9                    	movq	%rbx, %rcx
1407af5be: 45 31 c0                    	xorl	%r8d, %r8d
1407af5c1: e8 1a 99 1f 00              	callq	0x1409a8ee0 <.text+0x9a7ee0>
1407af5c6: 48 8d 15 19 40 2d 02        	leaq	0x22d4019(%rip), %rdx   # 0x142a835e6
1407af5cd: 41 b8 1f 00 00 00           	movl	$0x1f, %r8d
1407af5d3: 48 89 b5 d8 04 00 00        	movq	%rsi, 0x4d8(%rbp)
1407af5da: 48 89 f1                    	movq	%rsi, %rcx
1407af5dd: 49 89 d9                    	movq	%rbx, %r9
1407af5e0: e8 eb 40 17 00              	callq	0x1409236d0 <.text+0x9226d0>
1407af5e5: 4d 8b 0f                    	movq	(%r15), %r9
1407af5e8: 4d 8b 57 08                 	movq	0x8(%r15), %r10
1407af5ec: 49 8b 01                    	movq	(%r9), %rax
1407af5ef: 48 89 c1                    	movq	%rax, %rcx
1407af5f2: 49 2b 4a 18                 	subq	0x18(%r10), %rcx
1407af5f6: 48 0f 42 cf                 	cmovbq	%rdi, %rcx
1407af5fa: 49 2b 42 38                 	subq	0x38(%r10), %rax
1407af5fe: 48 0f 42 c7                 	cmovbq	%rdi, %rax
1407af602: 4d 8b 42 10                 	movq	0x10(%r10), %r8
1407af606: 49 8b 52 30                 	movq	0x30(%r10), %rdx
1407af60a: 49 89 d3                    	movq	%rdx, %r11
1407af60d: 49 29 c3                    	subq	%rax, %r11
1407af610: 4c 0f 42 df                 	cmovbq	%rdi, %r11
1407af614: 41 0f 96 c4                 	setbe	%r12b
1407af618: 4d 89 c6                    	movq	%r8, %r14
1407af61b: 49 29 ce                    	subq	%rcx, %r14
1407af61e: 4c 0f 42 f7                 	cmovbq	%rdi, %r14
1407af622: 41 0f 96 c5                 	setbe	%r13b
1407af626: 4b 8d 34 33                 	leaq	(%r11,%r14), %rsi
1407af62a: 45 84 e5                    	testb	%r12b, %r13b
1407af62d: 48 0f 44 f7                 	cmoveq	%rdi, %rsi
1407af631: 49 8b 5a 40                 	movq	0x40(%r10), %rbx
1407af635: 48 29 f3                    	subq	%rsi, %rbx
1407af638: 49 8b 7a 08                 	movq	0x8(%r10), %rdi
1407af63c: 4d 8b 52 28                 	movq	0x28(%r10), %r10
1407af640: 49 89 19                    	movq	%rbx, (%r9)
1407af643: 45 84 e5                    	testb	%r12b, %r13b
1407af646: 74 5e                       	je	0x1407af6a6 <.text+0x7ae6a6>
1407af648: 48 8b 8d 08 05 00 00        	movq	0x508(%rbp), %rcx
1407af64f: 8b 71 1c                    	movl	0x1c(%rcx), %esi
1407af652: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1407af656: 48 89 85 58 03 00 00        	movq	%rax, 0x358(%rbp)
1407af65d: 89 30                       	movl	%esi, (%rax)
1407af65f: 4c 8b 21                    	movq	(%rcx), %r12
1407af662: 49 8b 04 24                 	movq	(%r12), %rax
1407af666: 49 8b 4c 24 08              	movq	0x8(%r12), %rcx
1407af66b: 48 83 c0 fe                 	addq	$-0x2, %rax
1407af66f: 48 83 d1 ff                 	adcq	$-0x1, %rcx
1407af673: 31 d2                       	xorl	%edx, %edx
1407af675: 48 83 f8 05                 	cmpq	$0x5, %rax
1407af679: 48 83 d9 00                 	sbbq	$0x0, %rcx
1407af67d: 48 8d 40 01                 	leaq	0x1(%rax), %rax
1407af681: 48 0f 42 d0                 	cmovbq	%rax, %rdx
1407af685: 48 8d 05 04 41 2d 02        	leaq	0x22d4104(%rip), %rax   # 0x142a83790
1407af68c: 48 63 0c 90                 	movslq	(%rax,%rdx,4), %rcx
1407af690: 48 01 c1                    	addq	%rax, %rcx
1407af693: 89 b5 cc 04 00 00           	movl	%esi, 0x4cc(%rbp)
1407af699: ff e1                       	jmpq	*%rcx
1407af69b: 41 bd 5c 00 00 00           	movl	$0x5c, %r13d
1407af6a1: e9 1d 01 00 00              	jmp	0x1407af7c3 <.text+0x7ae7c3>
1407af6a6: 4c 6b c8 70                 	imulq	$0x70, %rax, %r9
1407af6aa: 4d 01 ca                    	addq	%r9, %r10
1407af6ad: 4d 6b cb 70                 	imulq	$0x70, %r11, %r9
1407af6b1: 4c 6b d9 70                 	imulq	$0x70, %rcx, %r11
1407af6b5: 4c 01 df                    	addq	%r11, %rdi
1407af6b8: 4d 6b de 70                 	imulq	$0x70, %r14, %r11
1407af6bc: 31 f6                       	xorl	%esi, %esi
1407af6be: 49 39 c8                    	cmpq	%rcx, %r8
1407af6c1: b9 10 00 00 00              	movl	$0x10, %ecx
1407af6c6: 48 0f 42 f9                 	cmovbq	%rcx, %rdi
1407af6ca: 4e 8d 44 1f 90              	leaq	-0x70(%rdi,%r11), %r8
1407af6cf: 49 0f 47 f0                 	cmovaq	%r8, %rsi
1407af6d3: 48 39 c2                    	cmpq	%rax, %rdx
1407af6d6: 4c 0f 42 d1                 	cmovbq	%rcx, %r10
1407af6da: 4b 8d 4c 0a 90              	leaq	-0x70(%r10,%r9), %rcx
1407af6df: 48 0f 46 ce                 	cmovbeq	%rsi, %rcx
1407af6e3: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1407af6e7: 48 8b 51 30                 	movq	0x30(%rcx), %rdx
1407af6eb: 44 8b 41 38                 	movl	0x38(%rcx), %r8d
1407af6ef: 44 89 85 30 01 00 00        	movl	%r8d, 0x130(%rbp)
1407af6f6: 48 89 95 28 01 00 00        	movq	%rdx, 0x128(%rbp)
1407af6fd: 48 8b 51 3c                 	movq	0x3c(%rcx), %rdx
1407af701: 48 89 95 34 01 00 00        	movq	%rdx, 0x134(%rbp)
1407af708: 8b 51 44                    	movl	0x44(%rcx), %edx
1407af70b: 89 95 3c 01 00 00           	movl	%edx, 0x13c(%rbp)
1407af711: 8b 51 48                    	movl	0x48(%rcx), %edx
1407af714: 0f 10 41 18                 	movups	0x18(%rcx), %xmm0
1407af718: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
1407af71f: 4c 8b 41 28                 	movq	0x28(%rcx), %r8
1407af723: 4c 89 85 20 01 00 00        	movq	%r8, 0x120(%rbp)
1407af72a: 44 0f b6 41 58              	movzbl	0x58(%rcx), %r8d
1407af72f: 0f 28 01                    	movaps	(%rcx), %xmm0
1407af732: 4c 8b 9d 08 05 00 00        	movq	0x508(%rbp), %r11
1407af739: 45 8b 4b 1c                 	movl	0x1c(%r11), %r9d
1407af73d: 4d 8b 13                    	movq	(%r11), %r10
1407af740: 4d 8b 5b 10                 	movq	0x10(%r11), %r11
1407af744: f2 0f 10 49 4c              	movsd	0x4c(%rcx), %xmm1
1407af749: 45 89 0b                    	movl	%r9d, (%r11)
1407af74c: 49 c7 42 08 00 00 00 00     	movq	$0x0, 0x8(%r10)
1407af754: 49 c7 02 01 00 00 00        	movq	$0x1, (%r10)
1407af75b: 41 0f 29 42 10              	movaps	%xmm0, 0x10(%r10)
1407af760: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1407af767: 48 8b 8d 20 01 00 00        	movq	0x120(%rbp), %rcx
1407af76e: 4c 8b 8d 28 01 00 00        	movq	0x128(%rbp), %r9
1407af775: 0f 28 95 30 01 00 00        	movaps	0x130(%rbp), %xmm2
1407af77c: 41 0f 29 42 20              	movaps	%xmm0, 0x20(%r10)
1407af781: 49 89 4a 30                 	movq	%rcx, 0x30(%r10)
1407af785: 4d 89 4a 38                 	movq	%r9, 0x38(%r10)
1407af789: 41 0f 29 52 40              	movaps	%xmm2, 0x40(%r10)
1407af78e: 41 89 52 50                 	movl	%edx, 0x50(%r10)
1407af792: f2 41 0f 11 4a 54           	movsd	%xmm1, 0x54(%r10)
1407af798: 49 89 42 60                 	movq	%rax, 0x60(%r10)
1407af79c: 41 c6 42 68 01              	movb	$0x1, 0x68(%r10)
1407af7a1: 45 88 42 69                 	movb	%r8b, 0x69(%r10)
1407af7a5: e9 9f 1b 00 00              	jmp	0x1407b1349 <.text+0x7b0349>
1407af7aa: 41 bd 69 00 00 00           	movl	$0x69, %r13d
1407af7b0: 41 be 20 00 00 00           	movl	$0x20, %r14d
1407af7b6: bf 60 00 00 00              	movl	$0x60, %edi
1407af7bb: eb 11                       	jmp	0x1407af7ce <.text+0x7ae7ce>
1407af7bd: 41 bd 60 00 00 00           	movl	$0x60, %r13d
1407af7c3: 41 be 18 00 00 00           	movl	$0x18, %r14d
1407af7c9: bf 10 00 00 00              	movl	$0x10, %edi
1407af7ce: 49 8b 14 3c                 	movq	(%r12,%rdi), %rdx
1407af7d2: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407af7d9: e8 a2 e5 ec ff              	callq	0x14067dd80 <.text+0x67cd80>
1407af7de: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
1407af7e5: 48 89 95 78 03 00 00        	movq	%rdx, 0x378(%rbp)
1407af7ec: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407af7f3: e8 18 e7 17 00              	callq	0x14092df10 <.text+0x92cf10>
1407af7f8: 3c 02                       	cmpb	$0x2, %al
1407af7fa: 75 12                       	jne	0x1407af80e <.text+0x7ae80e>
1407af7fc: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407af803: 8b 00                       	movl	(%rax), %eax
1407af805: 83 f8 14                    	cmpl	$0x14, %eax
1407af808: 0f 85 e9 00 00 00           	jne	0x1407af8f7 <.text+0x7ae8f7>
1407af80e: 4d 01 e6                    	addq	%r12, %r14
1407af811: 48 8b 85 a0 04 00 00        	movq	0x4a0(%rbp), %rax
1407af818: 48 8b 18                    	movq	(%rax), %rbx
1407af81b: 43 80 3c 2c 3f              	cmpb	$0x3f, (%r12,%r13)
1407af820: b8 23 00 00 00              	movl	$0x23, %eax
1407af825: b9 ab 30 00 00              	movl	$0x30ab, %ecx           # imm = 0x30AB
1407af82a: 0f 44 c8                    	cmovel	%eax, %ecx
1407af82d: 89 4c 24 20                 	movl	%ecx, 0x20(%rsp)
1407af831: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407af838: f3 0f 10 1d 40 61 17 02     	movss	0x2176140(%rip), %xmm3  # 0x142925980
1407af840: 48 8b 95 f0 04 00 00        	movq	0x4f0(%rbp), %rdx
1407af847: 49 89 d8                    	movq	%rbx, %r8
1407af84a: e8 91 ca 11 00              	callq	0x1408cc2e0 <.text+0x8cb2e0>
1407af84f: 83 bd f4 01 00 00 02        	cmpl	$0x2, 0x1f4(%rbp)
1407af856: 74 58                       	je	0x1407af8b0 <.text+0x7ae8b0>
1407af858: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1407af85f: 0f 10 8d d0 01 00 00        	movups	0x1d0(%rbp), %xmm1
1407af866: 0f 10 95 e0 01 00 00        	movups	0x1e0(%rbp), %xmm2
1407af86d: 0f 10 9d f0 01 00 00        	movups	0x1f0(%rbp), %xmm3
1407af874: 0f 29 9d a0 03 00 00        	movaps	%xmm3, 0x3a0(%rbp)
1407af87b: 0f 29 95 90 03 00 00        	movaps	%xmm2, 0x390(%rbp)
1407af882: 0f 29 8d 80 03 00 00        	movaps	%xmm1, 0x380(%rbp)
1407af889: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
1407af890: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1407af897: 48 8d 95 70 03 00 00        	leaq	0x370(%rbp), %rdx
1407af89e: e8 8d cd 11 00              	callq	0x1408cc630 <.text+0x8cb630>
1407af8a3: 83 bd 60 04 00 00 02        	cmpl	$0x2, 0x460(%rbp)
1407af8aa: 0f 85 39 01 00 00           	jne	0x1407af9e9 <.text+0x7ae9e9>
1407af8b0: 48 8b 85 38 06 00 00        	movq	0x638(%rbp), %rax
1407af8b7: 48 8b 10                    	movq	(%rax), %rdx
1407af8ba: 48 8d 8d 88 03 00 00        	leaq	0x388(%rbp), %rcx
1407af8c1: e8 8a 2d 26 00              	callq	0x140a12650 <.text+0xa11650>
1407af8c6: c6 85 70 03 00 00 0b        	movb	$0xb, 0x370(%rbp)
1407af8cd: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
1407af8d7: 48 89 85 94 03 00 00        	movq	%rax, 0x394(%rbp)
1407af8de: 48 c7 85 9c 03 00 00 00 00 00 00    	movq	$0x0, 0x39c(%rbp)
1407af8e9: e9 33 01 00 00              	jmp	0x1407afa21 <.text+0x7aea21>
1407af8ee: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407af8f5: 8b 00                       	movl	(%rax), %eax
1407af8f7: b1 01                       	movb	$0x1, %cl
1407af8f9: 83 f8 03                    	cmpl	$0x3, %eax
1407af8fc: 0f 85 4a 02 00 00           	jne	0x1407afb4c <.text+0x7aeb4c>
1407af902: 4c 89 a5 08 05 00 00        	movq	%r12, 0x508(%rbp)
1407af909: 4c 8b 85 c0 04 00 00        	movq	0x4c0(%rbp), %r8
1407af910: 49 8b 08                    	movq	(%r8), %rcx
1407af913: 45 31 d2                    	xorl	%r10d, %r10d
1407af916: 48 89 ca                    	movq	%rcx, %rdx
1407af919: 4c 8b b5 d0 04 00 00        	movq	0x4d0(%rbp), %r14
1407af920: 49 2b 56 18                 	subq	0x18(%r14), %rdx
1407af924: 49 0f 42 d2                 	cmovbq	%r10, %rdx
1407af928: 49 2b 4e 38                 	subq	0x38(%r14), %rcx
1407af92c: 49 0f 42 ca                 	cmovbq	%r10, %rcx
1407af930: 4d 8b 4e 08                 	movq	0x8(%r14), %r9
1407af934: 49 8b 46 30                 	movq	0x30(%r14), %rax
1407af938: 48 89 85 f8 04 00 00        	movq	%rax, 0x4f8(%rbp)
1407af93f: 48 29 c8                    	subq	%rcx, %rax
1407af942: 49 0f 42 c2                 	cmovbq	%r10, %rax
1407af946: 49 8b 7e 10                 	movq	0x10(%r14), %rdi
1407af94a: 40 0f 96 c6                 	setbe	%sil
1407af94e: 48 89 fb                    	movq	%rdi, %rbx
1407af951: 48 29 d3                    	subq	%rdx, %rbx
1407af954: 49 0f 42 da                 	cmovbq	%r10, %rbx
1407af958: 4d 8b 5e 28                 	movq	0x28(%r14), %r11
1407af95c: 4d 8b 56 40                 	movq	0x40(%r14), %r10
1407af960: 41 0f 96 c6                 	setbe	%r14b
1407af964: 4d 89 d5                    	movq	%r10, %r13
1407af967: 49 29 c5                    	subq	%rax, %r13
1407af96a: 49 29 dd                    	subq	%rbx, %r13
1407af96d: 4d 89 28                    	movq	%r13, (%r8)
1407af970: 41 84 f6                    	testb	%sil, %r14b
1407af973: 0f 84 01 04 00 00           	je	0x1407afd7a <.text+0x7aed7a>
1407af979: 49 83 c7 50                 	addq	$0x50, %r15
1407af97d: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407af984: 4c 89 fa                    	movq	%r15, %rdx
1407af987: e8 d4 28 ad 00              	callq	0x141282260 <.text+0x1281260>
1407af98c: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
1407af993: 48 8b 8d 80 03 00 00        	movq	0x380(%rbp), %rcx
1407af99a: 4c 8b 85 88 03 00 00        	movq	0x388(%rbp), %r8
1407af9a1: 4c 8b 95 90 03 00 00        	movq	0x390(%rbp), %r10
1407af9a8: 48 85 c0                    	testq	%rax, %rax
1407af9ab: 41 0f 94 c1                 	sete	%r9b
1407af9af: 48 39 c8                    	cmpq	%rcx, %rax
1407af9b2: 0f 94 c2                    	sete	%dl
1407af9b5: 44 08 ca                    	orb	%r9b, %dl
1407af9b8: 4d 85 c0                    	testq	%r8, %r8
1407af9bb: 0f 94 c0                    	sete	%al
1407af9be: 4d 39 d0                    	cmpq	%r10, %r8
1407af9c1: 41 0f 94 c0                 	sete	%r8b
1407af9c5: 41 08 c0                    	orb	%al, %r8b
1407af9c8: 44 89 c0                    	movl	%r8d, %eax
1407af9cb: 20 d0                       	andb	%dl, %al
1407af9cd: 3c 01                       	cmpb	$0x1, %al
1407af9cf: 48 8b b5 08 05 00 00        	movq	0x508(%rbp), %rsi
1407af9d6: 0f 85 8a 18 00 00           	jne	0x1407b1266 <.text+0x7b0266>
1407af9dc: 41 bb 06 00 00 00           	movl	$0x6, %r11d
1407af9e2: 31 c0                       	xorl	%eax, %eax
1407af9e4: e9 08 19 00 00              	jmp	0x1407b12f1 <.text+0x7b02f1>
1407af9e9: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
1407af9f0: 0f 10 8d 40 04 00 00        	movups	0x440(%rbp), %xmm1
1407af9f7: 0f 10 95 50 04 00 00        	movups	0x450(%rbp), %xmm2
1407af9fe: 0f 10 9d 60 04 00 00        	movups	0x460(%rbp), %xmm3
1407afa05: 0f 29 9d a0 03 00 00        	movaps	%xmm3, 0x3a0(%rbp)
1407afa0c: 0f 29 95 90 03 00 00        	movaps	%xmm2, 0x390(%rbp)
1407afa13: 0f 29 8d 80 03 00 00        	movaps	%xmm1, 0x380(%rbp)
1407afa1a: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
1407afa21: 0f 28 85 70 03 00 00        	movaps	0x370(%rbp), %xmm0
1407afa28: 0f 28 8d 80 03 00 00        	movaps	0x380(%rbp), %xmm1
1407afa2f: 0f 28 95 90 03 00 00        	movaps	0x390(%rbp), %xmm2
1407afa36: 0f 28 9d a0 03 00 00        	movaps	0x3a0(%rbp), %xmm3
1407afa3d: 41 0f 11 5e 30              	movups	%xmm3, 0x30(%r14)
1407afa42: 41 0f 11 56 20              	movups	%xmm2, 0x20(%r14)
1407afa47: 41 0f 11 4e 10              	movups	%xmm1, 0x10(%r14)
1407afa4c: 41 0f 11 06                 	movups	%xmm0, (%r14)
1407afa50: 43 80 3c 2c 3f              	cmpb	$0x3f, (%r12,%r13)
1407afa55: 75 11                       	jne	0x1407afa68 <.text+0x7aea68>
1407afa57: 41 80 3e 02                 	cmpb	$0x2, (%r14)
1407afa5b: 75 0b                       	jne	0x1407afa68 <.text+0x7aea68>
1407afa5d: 41 f6 46 01 01              	testb	$0x1, 0x1(%r14)
1407afa62: 75 04                       	jne	0x1407afa68 <.text+0x7aea68>
1407afa64: 41 c6 06 0b                 	movb	$0xb, (%r14)
1407afa68: 4d 8b 3c 3c                 	movq	(%r12,%rdi), %r15
1407afa6c: f3 0f 10 03                 	movss	(%rbx), %xmm0
1407afa70: f3 0f 10 4b 04              	movss	0x4(%rbx), %xmm1
1407afa75: f3 0f 10 53 08              	movss	0x8(%rbx), %xmm2
1407afa7a: f3 41 0f 5c 46 18           	subss	0x18(%r14), %xmm0
1407afa80: f3 41 0f 5c 4e 1c           	subss	0x1c(%r14), %xmm1
1407afa86: f3 41 0f 5c 56 20           	subss	0x20(%r14), %xmm2
1407afa8c: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407afa90: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
1407afa94: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407afa98: f3 0f 59 d2                 	mulss	%xmm2, %xmm2
1407afa9c: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407afaa0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407afaa3: f3 0f 51 c2                 	sqrtss	%xmm2, %xmm0
1407afaa7: f3 0f 58 05 39 e2 22 02     	addss	0x222e239(%rip), %xmm0  # 0x1429ddce8
1407afaaf: f3 0f 11 85 c4 01 00 00     	movss	%xmm0, 0x1c4(%rbp)
1407afab7: c6 85 c0 01 00 00 05        	movb	$0x5, 0x1c0(%rbp)
1407afabe: 48 8d 9d 70 03 00 00        	leaq	0x370(%rbp), %rbx
1407afac5: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407afacc: 4c 8d 85 00 03 00 00        	leaq	0x300(%rbp), %r8
1407afad3: 48 89 d9                    	movq	%rbx, %rcx
1407afad6: e8 35 93 1f 00              	callq	0x1409a8e10 <.text+0x9a7e10>
1407afadb: 80 8d 19 04 00 00 02        	orb	$0x2, 0x419(%rbp)
1407afae2: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407afae9: 41 b8 b0 00 00 00           	movl	$0xb0, %r8d
1407afaef: 48 89 da                    	movq	%rbx, %rdx
1407afaf2: e8 09 41 17 02              	callq	0x142923c00 <.text+0x2922c00>
1407afaf7: c6 85 05 05 00 00 01        	movb	$0x1, 0x505(%rbp)
1407afafe: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1407afb05: e8 a6 94 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407afb0a: c6 85 05 05 00 00 00        	movb	$0x0, 0x505(%rbp)
1407afb11: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407afb18: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407afb1f: 4c 8d 85 30 04 00 00        	leaq	0x430(%rbp), %r8
1407afb26: e8 55 93 1f 00              	callq	0x1409a8e80 <.text+0x9a7e80>
1407afb2b: 4c 8d 85 70 03 00 00        	leaq	0x370(%rbp), %r8
1407afb32: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407afb39: 4c 89 fa                    	movq	%r15, %rdx
1407afb3c: e8 9f e8 17 00              	callq	0x14092e3e0 <.text+0x92d3e0>
1407afb41: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407afb48: 8b 00                       	movl	(%rax), %eax
1407afb4a: 31 c9                       	xorl	%ecx, %ecx
1407afb4c: 83 f8 06                    	cmpl	$0x6, %eax
1407afb4f: 0f 83 dd 01 00 00           	jae	0x1407afd32 <.text+0x7aed32>
1407afb55: 83 f8 04                    	cmpl	$0x4, %eax
1407afb58: 0f 94 c0                    	sete	%al
1407afb5b: 89 ca                       	movl	%ecx, %edx
1407afb5d: 20 c2                       	andb	%al, %dl
1407afb5f: 80 fa 01                    	cmpb	$0x1, %dl
1407afb62: 0f 85 cc 01 00 00           	jne	0x1407afd34 <.text+0x7aed34>
1407afb68: 4c 8b 9d b8 04 00 00        	movq	0x4b8(%rbp), %r11
1407afb6f: 49 8b 13                    	movq	(%r11), %rdx
1407afb72: 31 c0                       	xorl	%eax, %eax
1407afb74: 49 89 d0                    	movq	%rdx, %r8
1407afb77: 4c 8b 8d e8 04 00 00        	movq	0x4e8(%rbp), %r9
1407afb7e: 4d 2b 41 18                 	subq	0x18(%r9), %r8
1407afb82: 4c 0f 42 c0                 	cmovbq	%rax, %r8
1407afb86: 49 2b 51 38                 	subq	0x38(%r9), %rdx
1407afb8a: 48 0f 42 d0                 	cmovbq	%rax, %rdx
1407afb8e: 4d 8b 51 08                 	movq	0x8(%r9), %r10
1407afb92: 49 8b 71 10                 	movq	0x10(%r9), %rsi
1407afb96: 49 8b 59 30                 	movq	0x30(%r9), %rbx
1407afb9a: 48 89 9d 08 05 00 00        	movq	%rbx, 0x508(%rbp)
1407afba1: 48 29 d3                    	subq	%rdx, %rbx
1407afba4: 48 0f 42 d8                 	cmovbq	%rax, %rbx
1407afba8: 41 0f 96 c7                 	setbe	%r15b
1407afbac: 49 89 f6                    	movq	%rsi, %r14
1407afbaf: 4d 29 c6                    	subq	%r8, %r14
1407afbb2: 4c 0f 42 f0                 	cmovbq	%rax, %r14
1407afbb6: 49 8b 79 28                 	movq	0x28(%r9), %rdi
1407afbba: 4d 8b 69 40                 	movq	0x40(%r9), %r13
1407afbbe: 4d 89 e1                    	movq	%r12, %r9
1407afbc1: 41 0f 96 c4                 	setbe	%r12b
1407afbc5: 4c 89 ad e8 04 00 00        	movq	%r13, 0x4e8(%rbp)
1407afbcc: 49 29 dd                    	subq	%rbx, %r13
1407afbcf: 4d 29 f5                    	subq	%r14, %r13
1407afbd2: 4d 89 2b                    	movq	%r13, (%r11)
1407afbd5: b0 01                       	movb	$0x1, %al
1407afbd7: 45 84 fc                    	testb	%r15b, %r12b
1407afbda: 4d 89 cc                    	movq	%r9, %r12
1407afbdd: 0f 85 51 01 00 00           	jne	0x1407afd34 <.text+0x7aed34>
1407afbe3: 48 8d 04 52                 	leaq	(%rdx,%rdx,2), %rax
1407afbe7: 48 c1 e0 05                 	shlq	$0x5, %rax
1407afbeb: 48 01 c7                    	addq	%rax, %rdi
1407afbee: 4b 8d 04 40                 	leaq	(%r8,%r8,2), %rax
1407afbf2: 48 c1 e0 05                 	shlq	$0x5, %rax
1407afbf6: 49 01 c2                    	addq	%rax, %r10
1407afbf9: 48 8d 04 5b                 	leaq	(%rbx,%rbx,2), %rax
1407afbfd: 48 c1 e0 05                 	shlq	$0x5, %rax
1407afc01: 4b 8d 0c 76                 	leaq	(%r14,%r14,2), %rcx
1407afc05: 48 c1 e1 05                 	shlq	$0x5, %rcx
1407afc09: 31 db                       	xorl	%ebx, %ebx
1407afc0b: 4c 39 c6                    	cmpq	%r8, %rsi
1407afc0e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1407afc14: 4d 0f 42 d0                 	cmovbq	%r8, %r10
1407afc18: 49 8d 4c 0a a0              	leaq	-0x60(%r10,%rcx), %rcx
1407afc1d: 48 0f 47 d9                 	cmovaq	%rcx, %rbx
1407afc21: 48 39 95 08 05 00 00        	cmpq	%rdx, 0x508(%rbp)
1407afc28: 49 0f 42 f8                 	cmovbq	%r8, %rdi
1407afc2c: 48 8d 4c 07 a0              	leaq	-0x60(%rdi,%rax), %rcx
1407afc31: 48 0f 46 cb                 	cmovbeq	%rbx, %rcx
1407afc35: 48 8b 85 b8 04 00 00        	movq	0x4b8(%rbp), %rax
1407afc3c: 48 8b 95 e8 04 00 00        	movq	0x4e8(%rbp), %rdx
1407afc43: 48 89 10                    	movq	%rdx, (%rax)
1407afc46: 8b 41 50                    	movl	0x50(%rcx), %eax
1407afc49: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
1407afc4d: 4c 8b 41 28                 	movq	0x28(%rcx), %r8
1407afc51: 44 8b 49 30                 	movl	0x30(%rcx), %r9d
1407afc55: 44 89 8d b0 01 00 00        	movl	%r9d, 0x1b0(%rbp)
1407afc5c: 4c 89 85 a8 01 00 00        	movq	%r8, 0x1a8(%rbp)
1407afc63: 4c 8b 41 34                 	movq	0x34(%rcx), %r8
1407afc67: 4c 89 85 b4 01 00 00        	movq	%r8, 0x1b4(%rbp)
1407afc6e: 44 8b 41 3c                 	movl	0x3c(%rcx), %r8d
1407afc72: 44 89 85 bc 01 00 00        	movl	%r8d, 0x1bc(%rbp)
1407afc79: 44 8b 41 40                 	movl	0x40(%rcx), %r8d
1407afc7d: 0f 10 41 10                 	movups	0x10(%rcx), %xmm0
1407afc81: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
1407afc88: 4c 8b 49 20                 	movq	0x20(%rcx), %r9
1407afc8c: 4c 89 8d a0 01 00 00        	movq	%r9, 0x1a0(%rbp)
1407afc93: f3 0f 10 41 54              	movss	0x54(%rcx), %xmm0
1407afc98: 44 0f b6 49 58              	movzbl	0x58(%rcx), %r9d
1407afc9d: f2 0f 10 49 44              	movsd	0x44(%rcx), %xmm1
1407afca2: 8b 8d cc 04 00 00           	movl	0x4cc(%rbp), %ecx
1407afca8: 4c 8b 95 58 03 00 00        	movq	0x358(%rbp), %r10
1407afcaf: 41 89 0a                    	movl	%ecx, (%r10)
1407afcb2: 49 c7 44 24 08 00 00 00 00  	movq	$0x0, 0x8(%r12)
1407afcbb: 49 c7 04 24 03 00 00 00     	movq	$0x3, (%r12)
1407afcc3: 49 89 54 24 10              	movq	%rdx, 0x10(%r12)
1407afcc8: 0f 28 95 90 01 00 00        	movaps	0x190(%rbp), %xmm2
1407afccf: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
1407afcd6: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
1407afcdd: 44 8b 95 b0 01 00 00        	movl	0x1b0(%rbp), %r10d
1407afce4: 4c 8b 9d b4 01 00 00        	movq	0x1b4(%rbp), %r11
1407afceb: 8b b5 bc 01 00 00           	movl	0x1bc(%rbp), %esi
1407afcf1: 49 89 4c 24 28              	movq	%rcx, 0x28(%r12)
1407afcf6: 49 89 54 24 30              	movq	%rdx, 0x30(%r12)
1407afcfb: 45 89 54 24 38              	movl	%r10d, 0x38(%r12)
1407afd00: 4d 89 5c 24 3c              	movq	%r11, 0x3c(%r12)
1407afd05: 41 89 74 24 44              	movl	%esi, 0x44(%r12)
1407afd0a: 41 0f 11 54 24 18           	movups	%xmm2, 0x18(%r12)
1407afd10: 45 89 44 24 48              	movl	%r8d, 0x48(%r12)
1407afd15: f2 41 0f 11 4c 24 4c        	movsd	%xmm1, 0x4c(%r12)
1407afd1c: 41 89 44 24 58              	movl	%eax, 0x58(%r12)
1407afd21: f3 41 0f 11 44 24 5c        	movss	%xmm0, 0x5c(%r12)
1407afd28: 45 88 4c 24 60              	movb	%r9b, 0x60(%r12)
1407afd2d: e9 17 16 00 00              	jmp	0x1407b1349 <.text+0x7b0349>
1407afd32: 31 c0                       	xorl	%eax, %eax
1407afd34: 84 c9                       	testb	%cl, %cl
1407afd36: 0f 84 0d 16 00 00           	je	0x1407b1349 <.text+0x7b0349>
1407afd3c: 49 8b 0c 24                 	movq	(%r12), %rcx
1407afd40: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
1407afd45: 48 83 c1 fe                 	addq	$-0x2, %rcx
1407afd49: 48 83 d2 ff                 	adcq	$-0x1, %rdx
1407afd4d: 45 31 c0                    	xorl	%r8d, %r8d
1407afd50: 48 83 f9 05                 	cmpq	$0x5, %rcx
1407afd54: 48 83 da 00                 	sbbq	$0x0, %rdx
1407afd58: 48 8d 49 01                 	leaq	0x1(%rcx), %rcx
1407afd5c: 4c 0f 42 c1                 	cmovbq	%rcx, %r8
1407afd60: 48 8d 0d 41 3a 2d 02        	leaq	0x22d3a41(%rip), %rcx   # 0x142a837a8
1407afd67: 4a 63 14 81                 	movslq	(%rcx,%r8,4), %rdx
1407afd6b: 48 01 ca                    	addq	%rcx, %rdx
1407afd6e: ff e2                       	jmpq	*%rdx
1407afd70: b9 5c 00 00 00              	movl	$0x5c, %ecx
1407afd75: e9 bc 00 00 00              	jmp	0x1407afe36 <.text+0x7aee36>
1407afd7a: 48 6b f1 58                 	imulq	$0x58, %rcx, %rsi
1407afd7e: 49 01 f3                    	addq	%rsi, %r11
1407afd81: 48 6b f0 58                 	imulq	$0x58, %rax, %rsi
1407afd85: 48 6b c2 58                 	imulq	$0x58, %rdx, %rax
1407afd89: 49 01 c1                    	addq	%rax, %r9
1407afd8c: 48 6b db 58                 	imulq	$0x58, %rbx, %rbx
1407afd90: 31 c0                       	xorl	%eax, %eax
1407afd92: 48 39 d7                    	cmpq	%rdx, %rdi
1407afd95: ba 08 00 00 00              	movl	$0x8, %edx
1407afd9a: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
1407afd9e: 4d 8d 4c 19 a8              	leaq	-0x58(%r9,%rbx), %r9
1407afda3: 4c 0f 46 c8                 	cmovbeq	%rax, %r9
1407afda7: 48 39 8d f8 04 00 00        	cmpq	%rcx, 0x4f8(%rbp)
1407afdae: 4c 0f 42 da                 	cmovbq	%rdx, %r11
1407afdb2: 4d 8d 5c 33 a8              	leaq	-0x58(%r11,%rsi), %r11
1407afdb7: 4d 0f 46 d9                 	cmovbeq	%r9, %r11
1407afdbb: 4d 89 10                    	movq	%r10, (%r8)
1407afdbe: 41 8b 4b 50                 	movl	0x50(%r11), %ecx
1407afdc2: 41 0f b6 53 54              	movzbl	0x54(%r11), %edx
1407afdc7: 4d 8b 43 08                 	movq	0x8(%r11), %r8
1407afdcb: 4d 8b 4b 28                 	movq	0x28(%r11), %r9
1407afdcf: 45 8b 53 30                 	movl	0x30(%r11), %r10d
1407afdd3: 44 89 95 d0 02 00 00        	movl	%r10d, 0x2d0(%rbp)
1407afdda: 4c 89 8d c8 02 00 00        	movq	%r9, 0x2c8(%rbp)
1407afde1: 4d 8b 4b 34                 	movq	0x34(%r11), %r9
1407afde5: 4c 89 8d d4 02 00 00        	movq	%r9, 0x2d4(%rbp)
1407afdec: 45 8b 4b 3c                 	movl	0x3c(%r11), %r9d
1407afdf0: 44 89 8d dc 02 00 00        	movl	%r9d, 0x2dc(%rbp)
1407afdf7: 45 8b 4b 40                 	movl	0x40(%r11), %r9d
1407afdfb: f2 41 0f 10 43 44           	movsd	0x44(%r11), %xmm0
1407afe01: 4d 8b 53 20                 	movq	0x20(%r11), %r10
1407afe05: 4c 89 95 c0 02 00 00        	movq	%r10, 0x2c0(%rbp)
1407afe0c: 41 0f 10 4b 10              	movups	0x10(%r11), %xmm1
1407afe11: 0f 29 8d b0 02 00 00        	movaps	%xmm1, 0x2b0(%rbp)
1407afe18: 41 bb 02 00 00 00           	movl	$0x2, %r11d
1407afe1e: 48 8b b5 08 05 00 00        	movq	0x508(%rbp), %rsi
1407afe25: e9 c7 14 00 00              	jmp	0x1407b12f1 <.text+0x7b02f1>
1407afe2a: b9 69 00 00 00              	movl	$0x69, %ecx
1407afe2f: eb 05                       	jmp	0x1407afe36 <.text+0x7aee36>
1407afe31: b9 60 00 00 00              	movl	$0x60, %ecx
1407afe36: 4c 8b 9d f8 04 00 00        	movq	0x4f8(%rbp), %r11
1407afe3d: 41 80 3c 0c 3f              	cmpb	$0x3f, (%r12,%rcx)
1407afe42: ba 23 00 00 00              	movl	$0x23, %edx
1407afe47: b9 ab 30 00 00              	movl	$0x30ab, %ecx           # imm = 0x30AB
1407afe4c: 0f 44 ca                    	cmovel	%edx, %ecx
1407afe4f: 41 8b 13                    	movl	(%r11), %edx
1407afe52: 44 8d 4a fa                 	leal	-0x6(%rdx), %r9d
1407afe56: 49 89 d2                    	movq	%rdx, %r10
1407afe59: 49 83 c2 fb                 	addq	$-0x5, %r10
1407afe5d: 45 31 c0                    	xorl	%r8d, %r8d
1407afe60: 41 83 f9 0f                 	cmpl	$0xf, %r9d
1407afe64: 4d 0f 42 c2                 	cmovbq	%r10, %r8
1407afe68: 4d 85 c0                    	testq	%r8, %r8
1407afe6b: 74 0f                       	je	0x1407afe7c <.text+0x7aee7c>
1407afe6d: 49 83 f8 06                 	cmpq	$0x6, %r8
1407afe71: 75 14                       	jne	0x1407afe87 <.text+0x7aee87>
1407afe73: 41 80 7b 04 3f              	cmpb	$0x3f, 0x4(%r11)
1407afe78: 74 07                       	je	0x1407afe81 <.text+0x7aee81>
1407afe7a: eb 33                       	jmp	0x1407afeaf <.text+0x7aeeaf>
1407afe7c: f6 c2 1b                    	testb	$0x1b, %dl
1407afe7f: 75 1a                       	jne	0x1407afe9b <.text+0x7aee9b>
1407afe81: 81 c9 00 40 00 00           	orl	$0x4000, %ecx           # imm = 0x4000
1407afe87: 4d 85 c0                    	testq	%r8, %r8
1407afe8a: 74 0f                       	je	0x1407afe9b <.text+0x7aee9b>
1407afe8c: 49 83 f8 06                 	cmpq	$0x6, %r8
1407afe90: 75 1d                       	jne	0x1407afeaf <.text+0x7aeeaf>
1407afe92: 41 80 7b 04 3f              	cmpb	$0x3f, 0x4(%r11)
1407afe97: 74 1c                       	je	0x1407afeb5 <.text+0x7aeeb5>
1407afe99: eb 14                       	jmp	0x1407afeaf <.text+0x7aeeaf>
1407afe9b: 83 fa 04                    	cmpl	$0x4, %edx
1407afe9e: 75 0f                       	jne	0x1407afeaf <.text+0x7aeeaf>
1407afea0: 48 8b 95 a0 04 00 00        	movq	0x4a0(%rbp), %rdx
1407afea7: 4c 8b 02                    	movq	(%rdx), %r8
1407afeaa: e9 bc 00 00 00              	jmp	0x1407aff6b <.text+0x7aef6b>
1407afeaf: 81 c9 00 01 00 00           	orl	$0x100, %ecx            # imm = 0x100
1407afeb5: 4c 8b 85 a0 04 00 00        	movq	0x4a0(%rbp), %r8
1407afebc: 4d 8b 00                    	movq	(%r8), %r8
1407afebf: 83 fa 14                    	cmpl	$0x14, %edx
1407afec2: 0f 85 a3 00 00 00           	jne	0x1407aff6b <.text+0x7aef6b>
1407afec8: 48 8b bd 70 06 00 00        	movq	0x670(%rbp), %rdi
1407afecf: c7 44 24 20 02 00 00 00     	movl	$0x2, 0x20(%rsp)
1407afed7: 48 8d b5 70 03 00 00        	leaq	0x370(%rbp), %rsi
1407afede: f3 0f 10 1d 9a 5a 17 02     	movss	0x2175a9a(%rip), %xmm3  # 0x142925980
1407afee6: 48 89 f1                    	movq	%rsi, %rcx
1407afee9: 48 8b 95 f0 04 00 00        	movq	0x4f0(%rbp), %rdx
1407afef0: 4c 89 c3                    	movq	%r8, %rbx
1407afef3: e8 e8 c3 11 00              	callq	0x1408cc2e0 <.text+0x8cb2e0>
1407afef8: 8b 85 a4 03 00 00           	movl	0x3a4(%rbp), %eax
1407afefe: 83 f8 02                    	cmpl	$0x2, %eax
1407aff01: 0f 85 11 11 00 00           	jne	0x1407b1018 <.text+0x7b0018>
1407aff07: 48 8b 85 38 06 00 00        	movq	0x638(%rbp), %rax
1407aff0e: 48 8b 10                    	movq	(%rax), %rdx
1407aff11: 48 8d 8d 88 02 00 00        	leaq	0x288(%rbp), %rcx
1407aff18: e8 33 27 26 00              	callq	0x140a12650 <.text+0xa11650>
1407aff1d: c6 85 70 02 00 00 0b        	movb	$0xb, 0x270(%rbp)
1407aff24: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
1407aff2e: 48 89 85 94 02 00 00        	movq	%rax, 0x294(%rbp)
1407aff35: c7 85 9c 02 00 00 00 00 00 00       	movl	$0x0, 0x29c(%rbp)
1407aff3f: 8b 85 a8 02 00 00           	movl	0x2a8(%rbp), %eax
1407aff45: 48 89 85 d0 04 00 00        	movq	%rax, 0x4d0(%rbp)
1407aff4c: 8b 85 ac 02 00 00           	movl	0x2ac(%rbp), %eax
1407aff52: 89 85 c0 04 00 00           	movl	%eax, 0x4c0(%rbp)
1407aff58: 66 0f 6e 85 a4 02 00 00     	movd	0x2a4(%rbp), %xmm0
1407aff60: 66 44 0f 70 e0 51           	pshufd	$0x51, %xmm0, %xmm12    # xmm12 = xmm0[1,0,1,1]
1407aff66: e9 4a 11 00 00              	jmp	0x1407b10b5 <.text+0x7b00b5>
1407aff6b: 48 8b bd 78 06 00 00        	movq	0x678(%rbp), %rdi
1407aff72: 48 8b 9d 60 06 00 00        	movq	0x660(%rbp), %rbx
1407aff79: 48 8b b5 58 06 00 00        	movq	0x658(%rbp), %rsi
1407aff80: 84 c0                       	testb	%al, %al
1407aff82: b8 00 50 00 00              	movl	$0x5000, %eax           # imm = 0x5000
1407aff87: 0f 44 c1                    	cmovel	%ecx, %eax
1407aff8a: 48 8b 8d f0 04 00 00        	movq	0x4f0(%rbp), %rcx
1407aff91: 48 8b 11                    	movq	(%rcx), %rdx
1407aff94: 4c 8b 49 20                 	movq	0x20(%rcx), %r9
1407aff98: 89 44 24 28                 	movl	%eax, 0x28(%rsp)
1407aff9c: c7 44 24 20 ff ff 7f 7f     	movl	$0x7f7fffff, 0x20(%rsp) # imm = 0x7F7FFFFF
1407affa4: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407affab: 4c 89 85 b0 04 00 00        	movq	%r8, 0x4b0(%rbp)
1407affb2: e8 e9 c8 11 00              	callq	0x1408cc8a0 <.text+0x8cb8a0>
1407affb7: 4c 8b b5 70 03 00 00        	movq	0x370(%rbp), %r14
1407affbe: 48 8b 8d 78 03 00 00        	movq	0x378(%rbp), %rcx
1407affc5: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1407affcc: 48 c1 e0 06                 	shlq	$0x6, %rax
1407affd0: 48 01 c8                    	addq	%rcx, %rax
1407affd3: 48 89 85 a0 04 00 00        	movq	%rax, 0x4a0(%rbp)
1407affda: 48 8b 07                    	movq	(%rdi), %rax
1407affdd: 48 89 85 b8 04 00 00        	movq	%rax, 0x4b8(%rbp)
1407affe4: 48 8b 03                    	movq	(%rbx), %rax
1407affe7: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
1407affee: 48 8b 85 38 06 00 00        	movq	0x638(%rbp), %rax
1407afff5: 48 8b 00                    	movq	(%rax), %rax
1407afff8: 48 89 85 28 04 00 00        	movq	%rax, 0x428(%rbp)
1407affff: 48 8b 06                    	movq	(%rsi), %rax
1407b0002: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
1407b0009: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
1407b0010: 48 8d 40 20                 	leaq	0x20(%rax), %rax
1407b0014: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1407b001b: be 07 00 00 00              	movl	$0x7, %esi
1407b0020: f3 0f 10 3d 90 6c 26 02     	movss	0x2266c90(%rip), %xmm7  # 0x142a16cb8
1407b0028: f3 44 0f 10 05 53 59 17 02  	movss	0x2175953(%rip), %xmm8  # 0x142925984
1407b0031: f3 44 0f 10 15 66 1c 2d 02  	movss	0x22d1c66(%rip), %xmm10 # 0x142a81ca0
1407b003a: 44 0f 28 1d 1e 0d 18 02     	movaps	0x2180d1e(%rip), %xmm11 # 0x142930d60
1407b0042: 48 89 85 e8 04 00 00        	movq	%rax, 0x4e8(%rbp)
1407b0049: c7 85 c0 04 00 00 00 00 00 00       	movl	$0x0, 0x4c0(%rbp)
1407b0053: 48 89 8d 68 03 00 00        	movq	%rcx, 0x368(%rbp)
1407b005a: 48 89 cb                    	movq	%rcx, %rbx
1407b005d: 31 ff                       	xorl	%edi, %edi
1407b005f: 48 89 85 d0 04 00 00        	movq	%rax, 0x4d0(%rbp)
1407b0066: 4c 89 b5 f0 04 00 00        	movq	%r14, 0x4f0(%rbp)
1407b006d: 0f 1f 00                    	nopl	(%rax)
1407b0070: 48 3b 9d a0 04 00 00        	cmpq	0x4a0(%rbp), %rbx
1407b0077: 0f 84 59 0f 00 00           	je	0x1407b0fd6 <.text+0x7affd6>
1407b007d: 8b 43 34                    	movl	0x34(%rbx), %eax
1407b0080: 83 f8 02                    	cmpl	$0x2, %eax
1407b0083: 0f 84 4d 0f 00 00           	je	0x1407b0fd6 <.text+0x7affd6>
1407b0089: 48 8b 4b 38                 	movq	0x38(%rbx), %rcx
1407b008d: 8b 53 30                    	movl	0x30(%rbx), %edx
1407b0090: 89 55 00                    	movl	%edx, (%rbp)
1407b0093: 0f 10 03                    	movups	(%rbx), %xmm0
1407b0096: 0f 10 4b 10                 	movups	0x10(%rbx), %xmm1
1407b009a: 0f 10 53 20                 	movups	0x20(%rbx), %xmm2
1407b009e: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
1407b00a2: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
1407b00a6: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1407b00aa: 89 45 04                    	movl	%eax, 0x4(%rbp)
1407b00ad: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
1407b00b1: 48 89 f0                    	movq	%rsi, %rax
1407b00b4: 48 83 f0 07                 	xorq	$0x7, %rax
1407b00b8: 48 09 f8                    	orq	%rdi, %rax
1407b00bb: 0f 85 b6 10 00 00           	jne	0x1407b1177 <.text+0x7b0177>
1407b00c1: 45 89 ef                    	movl	%r13d, %r15d
1407b00c4: 48 8d 8d 30 04 00 00        	leaq	0x430(%rbp), %rcx
1407b00cb: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
1407b00cf: e8 5c c5 11 00              	callq	0x1408cc630 <.text+0x8cb630>
1407b00d4: 48 83 c3 40                 	addq	$0x40, %rbx
1407b00d8: 0f b6 85 30 04 00 00        	movzbl	0x430(%rbp), %eax
1407b00df: 3c 0d                       	cmpb	$0xd, %al
1407b00e1: 75 62                       	jne	0x1407b0145 <.text+0x7af145>
1407b00e3: 48 8b 8d f8 04 00 00        	movq	0x4f8(%rbp), %rcx
1407b00ea: 83 39 04                    	cmpl	$0x4, (%rcx)
1407b00ed: 75 56                       	jne	0x1407b0145 <.text+0x7af145>
1407b00ef: 8b b5 34 04 00 00           	movl	0x434(%rbp), %esi
1407b00f5: 48 8b 8d b8 04 00 00        	movq	0x4b8(%rbp), %rcx
1407b00fc: e8 af e7 14 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1407b0101: 48 8b 8d f8 04 00 00        	movq	0x4f8(%rbp), %rcx
1407b0108: 8b 51 04                    	movl	0x4(%rcx), %edx
1407b010b: 48 89 c1                    	movq	%rax, %rcx
1407b010e: 41 89 f0                    	movl	%esi, %r8d
1407b0111: 45 31 c9                    	xorl	%r9d, %r9d
1407b0114: e8 57 e1 aa 00              	callq	0x14125e270 <.text+0x125d270>
1407b0119: 84 c0                       	testb	%al, %al
1407b011b: 0f 84 4a 04 00 00           	je	0x1407b056b <.text+0x7af56b>
1407b0121: 0f b6 85 30 04 00 00        	movzbl	0x430(%rbp), %eax
1407b0128: 0f b6 c0                    	movzbl	%al, %eax
1407b012b: 48 8d 15 8e 36 2d 02        	leaq	0x22d368e(%rip), %rdx   # 0x142a837c0
1407b0132: 48 63 0c 82                 	movslq	(%rdx,%rax,4), %rcx
1407b0136: 48 01 d1                    	addq	%rdx, %rcx
1407b0139: 4c 8b b5 f0 04 00 00        	movq	0x4f0(%rbp), %r14
1407b0140: 45 89 fd                    	movl	%r15d, %r13d
1407b0143: ff e1                       	jmpq	*%rcx
1407b0145: 0f b6 c8                    	movzbl	%al, %ecx
1407b0148: 48 8d 15 71 36 2d 02        	leaq	0x22d3671(%rip), %rdx   # 0x142a837c0
1407b014f: 48 63 0c 8a                 	movslq	(%rdx,%rcx,4), %rcx
1407b0153: 48 01 d1                    	addq	%rdx, %rcx
1407b0156: 4c 8b b5 f0 04 00 00        	movq	0x4f0(%rbp), %r14
1407b015d: 45 89 fd                    	movl	%r15d, %r13d
1407b0160: ff e1                       	jmpq	*%rcx
1407b0162: 44 8b 8d 34 04 00 00        	movl	0x434(%rbp), %r9d
1407b0169: 44 89 8d 10 05 00 00        	movl	%r9d, 0x510(%rbp)
1407b0170: 48 8b 95 38 04 00 00        	movq	0x438(%rbp), %rdx
1407b0177: 4c 8b 85 40 04 00 00        	movq	0x440(%rbp), %r8
1407b017e: 48 89 95 90 04 00 00        	movq	%rdx, 0x490(%rbp)
1407b0185: 4c 89 85 98 04 00 00        	movq	%r8, 0x498(%rbp)
1407b018c: 48 8b 8d f8 04 00 00        	movq	0x4f8(%rbp), %rcx
1407b0193: 8b 01                       	movl	(%rcx), %eax
1407b0195: 83 f8 04                    	cmpl	$0x4, %eax
1407b0198: 0f 85 33 01 00 00           	jne	0x1407b02d1 <.text+0x7af2d1>
1407b019e: 44 8b 41 04                 	movl	0x4(%rcx), %r8d
1407b01a2: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b01a9: 48 8b 95 b8 04 00 00        	movq	0x4b8(%rbp), %rdx
1407b01b0: e8 0b e7 14 00              	callq	0x1408fe8c0 <.text+0x8fd8c0>
1407b01b5: 48 8b 85 78 03 00 00        	movq	0x378(%rbp), %rax
1407b01bc: 48 89 85 08 05 00 00        	movq	%rax, 0x508(%rbp)
1407b01c3: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1407b01ca: 8b 8d 10 05 00 00           	movl	0x510(%rbp), %ecx
1407b01d0: 48 c1 e0 02                 	shlq	$0x2, %rax
1407b01d4: 31 d2                       	xorl	%edx, %edx
1407b01d6: 4c 8b b5 f0 04 00 00        	movq	0x4f0(%rbp), %r14
1407b01dd: 45 89 fd                    	movl	%r15d, %r13d
1407b01e0: 48 39 d0                    	cmpq	%rdx, %rax
1407b01e3: 0f 84 98 03 00 00           	je	0x1407b0581 <.text+0x7af581>
1407b01e9: 4c 8d 42 04                 	leaq	0x4(%rdx), %r8
1407b01ed: 4c 8b 8d 08 05 00 00        	movq	0x508(%rbp), %r9
1407b01f4: 41 3b 0c 11                 	cmpl	(%r9,%rdx), %ecx
1407b01f8: 4c 89 c2                    	movq	%r8, %rdx
1407b01fb: 75 e3                       	jne	0x1407b01e0 <.text+0x7af1e0>
1407b01fd: 48 8b 8d b8 04 00 00        	movq	0x4b8(%rbp), %rcx
1407b0204: e8 a7 e6 14 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1407b0209: 8b 8d 10 05 00 00           	movl	0x510(%rbp), %ecx
1407b020f: 89 8d c0 01 00 00           	movl	%ecx, 0x1c0(%rbp)
1407b0215: 48 89 c1                    	movq	%rax, %rcx
1407b0218: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407b021f: e8 2c dc aa 00              	callq	0x14125de50 <.text+0x125ce50>
1407b0224: 48 85 c0                    	testq	%rax, %rax
1407b0227: 74 17                       	je	0x1407b0240 <.text+0x7af240>
1407b0229: 48 8b 40 10                 	movq	0x10(%rax), %rax
1407b022d: 48 85 c0                    	testq	%rax, %rax
1407b0230: 74 0e                       	je	0x1407b0240 <.text+0x7af240>
1407b0232: 48 8d 04 c5 f8 ff ff ff     	leaq	-0x8(,%rax,8), %rax
1407b023a: 48 83 f8 07                 	cmpq	$0x7, %rax
1407b023e: 77 51                       	ja	0x1407b0291 <.text+0x7af291>
1407b0240: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
1407b0247: 8b 8d 4c 03 00 00           	movl	0x34c(%rbp), %ecx
1407b024d: 89 08                       	movl	%ecx, (%rax)
1407b024f: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
1407b0256: 48 8b 70 30                 	movq	0x30(%rax), %rsi
1407b025a: 48 8b 78 40                 	movq	0x40(%rax), %rdi
1407b025e: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
1407b0262: 75 13                       	jne	0x1407b0277 <.text+0x7af277>
1407b0264: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
1407b026b: 48 8d 15 0e 1d 47 02        	leaq	0x2471d0e(%rip), %rdx   # 0x142c21f80
1407b0272: e8 f9 25 db ff              	callq	0x140562870 <.text+0x561870>
1407b0277: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
1407b027e: 48 8b 41 28                 	movq	0x28(%rcx), %rax
1407b0282: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
1407b0286: 48 ff c6                    	incq	%rsi
1407b0289: 48 89 71 30                 	movq	%rsi, 0x30(%rcx)
1407b028d: 48 ff 41 40                 	incq	0x40(%rcx)
1407b0291: 48 8b 95 70 03 00 00        	movq	0x370(%rbp), %rdx
1407b0298: 48 85 d2                    	testq	%rdx, %rdx
1407b029b: 74 16                       	je	0x1407b02b3 <.text+0x7af2b3>
1407b029d: 48 c1 e2 02                 	shlq	$0x2, %rdx
1407b02a1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407b02a7: 48 8b 8d 08 05 00 00        	movq	0x508(%rbp), %rcx
1407b02ae: e8 6d 39 e6 ff              	callq	0x140613c20 <.text+0x612c20>
1407b02b3: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407b02ba: 8b 00                       	movl	(%rax), %eax
1407b02bc: 48 8b 95 90 04 00 00        	movq	0x490(%rbp), %rdx
1407b02c3: 4c 8b 85 98 04 00 00        	movq	0x498(%rbp), %r8
1407b02ca: 44 8b 8d 10 05 00 00        	movl	0x510(%rbp), %r9d
1407b02d1: 83 f8 0b                    	cmpl	$0xb, %eax
1407b02d4: 75 41                       	jne	0x1407b0317 <.text+0x7af317>
1407b02d6: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407b02dd: 80 78 04 3f                 	cmpb	$0x3f, 0x4(%rax)
1407b02e1: 75 34                       	jne	0x1407b0317 <.text+0x7af317>
1407b02e3: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
1407b02ea: e8 01 c3 26 00              	callq	0x140a1c5f0 <.text+0xa1b5f0>
1407b02ef: 48 85 c0                    	testq	%rax, %rax
1407b02f2: 0f 84 db 10 00 00           	je	0x1407b13d3 <.text+0x7b03d3>
1407b02f8: 80 78 46 3f                 	cmpb	$0x3f, 0x46(%rax)
1407b02fc: 0f 85 09 07 00 00           	jne	0x1407b0a0b <.text+0x7afa0b>
1407b0302: 48 8b 95 90 04 00 00        	movq	0x490(%rbp), %rdx
1407b0309: 4c 8b 85 98 04 00 00        	movq	0x498(%rbp), %r8
1407b0310: 44 8b 8d 10 05 00 00        	movl	0x510(%rbp), %r9d
1407b0317: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
1407b031e: e8 cd c2 26 00              	callq	0x140a1c5f0 <.text+0xa1b5f0>
1407b0323: 48 85 c0                    	testq	%rax, %rax
1407b0326: 0f 84 5b 06 00 00           	je	0x1407b0987 <.text+0x7af987>
1407b032c: 8b b5 10 05 00 00           	movl	0x510(%rbp), %esi
1407b0332: 48 8b 95 90 04 00 00        	movq	0x490(%rbp), %rdx
1407b0339: 4c 8b 85 98 04 00 00        	movq	0x498(%rbp), %r8
1407b0340: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
1407b0347: 41 89 f1                    	movl	%esi, %r9d
1407b034a: e8 a1 c2 26 00              	callq	0x140a1c5f0 <.text+0xa1b5f0>
1407b034f: 49 89 c5                    	movq	%rax, %r13
1407b0352: 48 85 c0                    	testq	%rax, %rax
1407b0355: 0f 84 6a 10 00 00           	je	0x1407b13c5 <.text+0x7b03c5>
1407b035b: 41 0f b6 45 46              	movzbl	0x46(%r13), %eax
1407b0360: 8d 48 c8                    	leal	-0x38(%rax), %ecx
1407b0363: 80 f9 08                    	cmpb	$0x8, %cl
1407b0366: 0f 93 c2                    	setae	%dl
1407b0369: 80 f9 06                    	cmpb	$0x6, %cl
1407b036c: 40 0f 94 c7                 	sete	%dil
1407b0370: 40 08 d7                    	orb	%dl, %dil
1407b0373: 75 1a                       	jne	0x1407b038f <.text+0x7af38f>
1407b0375: 89 c1                       	movl	%eax, %ecx
1407b0377: 80 e1 3e                    	andb	$0x3e, %cl
1407b037a: 80 f9 3c                    	cmpb	$0x3c, %cl
1407b037d: 75 10                       	jne	0x1407b038f <.text+0x7af38f>
1407b037f: 48 8b 8d 90 04 00 00        	movq	0x490(%rbp), %rcx
1407b0386: 83 f9 01                    	cmpl	$0x1, %ecx
1407b0389: 0f 84 d7 07 00 00           	je	0x1407b0b66 <.text+0x7afb66>
1407b038f: 0f b6 c0                    	movzbl	%al, %eax
1407b0392: 83 f8 3f                    	cmpl	$0x3f, %eax
1407b0395: 0f 85 d6 04 00 00           	jne	0x1407b0871 <.text+0x7af871>
1407b039b: 48 8d 45 b0                 	leaq	-0x50(%rbp), %rax
1407b039f: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1407b03a3: 48 8d 85 10 05 00 00        	leaq	0x510(%rbp), %rax
1407b03aa: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
1407b03ae: 48 8d 85 4c 04 00 00        	leaq	0x44c(%rbp), %rax
1407b03b5: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1407b03b9: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
1407b03bd: 48 8b 95 e8 02 00 00        	movq	0x2e8(%rbp), %rdx
1407b03c4: e8 c7 e4 14 00              	callq	0x1408fe890 <.text+0x8fd890>
1407b03c9: 83 7d 18 03                 	cmpl	$0x3, 0x18(%rbp)
1407b03cd: 0f 84 7e 01 00 00           	je	0x1407b0551 <.text+0x7af551>
1407b03d3: 8b 45 28                    	movl	0x28(%rbp), %eax
1407b03d6: 3b 85 10 05 00 00           	cmpl	0x510(%rbp), %eax
1407b03dc: 0f 84 6f 01 00 00           	je	0x1407b0551 <.text+0x7af551>
1407b03e2: 48 8b 8d e8 02 00 00        	movq	0x2e8(%rbp), %rcx
1407b03e9: e8 c2 e4 14 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1407b03ee: 8b 8d 10 05 00 00           	movl	0x510(%rbp), %ecx
1407b03f4: 89 8d 70 03 00 00           	movl	%ecx, 0x370(%rbp)
1407b03fa: 48 89 c1                    	movq	%rax, %rcx
1407b03fd: 48 8d 95 70 03 00 00        	leaq	0x370(%rbp), %rdx
1407b0404: e8 47 da aa 00              	callq	0x14125de50 <.text+0x125ce50>
1407b0409: 48 85 c0                    	testq	%rax, %rax
1407b040c: 74 1b                       	je	0x1407b0429 <.text+0x7af429>
1407b040e: 48 8b 40 10                 	movq	0x10(%rax), %rax
1407b0412: 48 85 c0                    	testq	%rax, %rax
1407b0415: 74 12                       	je	0x1407b0429 <.text+0x7af429>
1407b0417: 48 8d 04 c5 f8 ff ff ff     	leaq	-0x8(,%rax,8), %rax
1407b041f: 48 83 f8 07                 	cmpq	$0x7, %rax
1407b0423: 0f 87 28 01 00 00           	ja	0x1407b0551 <.text+0x7af551>
1407b0429: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1407b0430: e8 4b 52 11 00              	callq	0x1408c5680 <.text+0x8c4680>
1407b0435: 84 c0                       	testb	%al, %al
1407b0437: 0f 85 14 01 00 00           	jne	0x1407b0551 <.text+0x7af551>
1407b043d: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
1407b0441: e8 7a 13 00 00              	callq	0x1407b17c0 <.text+0x7b07c0>
1407b0446: 84 c0                       	testb	%al, %al
1407b0448: 0f 84 03 01 00 00           	je	0x1407b0551 <.text+0x7af551>
1407b044e: 48 8b 85 b0 04 00 00        	movq	0x4b0(%rbp), %rax
1407b0455: f3 44 0f 10 28              	movss	(%rax), %xmm13
1407b045a: f3 44 0f 10 60 04           	movss	0x4(%rax), %xmm12
1407b0460: f3 44 0f 5c ad 48 04 00 00  	subss	0x448(%rbp), %xmm13
1407b0469: f3 44 0f 5c a5 4c 04 00 00  	subss	0x44c(%rbp), %xmm12
1407b0472: f3 44 0f 10 70 08           	movss	0x8(%rax), %xmm14
1407b0478: f3 44 0f 5c b5 50 04 00 00  	subss	0x450(%rbp), %xmm14
1407b0481: 41 8a 7d 46                 	movb	0x46(%r13), %dil
1407b0485: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b048c: e8 1f 8b 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407b0491: c7 45 50 07 00 00 00        	movl	$0x7, 0x50(%rbp)
1407b0498: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407b049f: 48 8d 95 70 03 00 00        	leaq	0x370(%rbp), %rdx
1407b04a6: 4c 8d 45 50                 	leaq	0x50(%rbp), %r8
1407b04aa: e8 21 8b 1f 00              	callq	0x1409a8fd0 <.text+0x9a7fd0>
1407b04af: f3 45 0f 59 ed              	mulss	%xmm13, %xmm13
1407b04b4: f3 45 0f 59 e4              	mulss	%xmm12, %xmm12
1407b04b9: f3 45 0f 58 e5              	addss	%xmm13, %xmm12
1407b04be: f3 45 0f 59 f6              	mulss	%xmm14, %xmm14
1407b04c3: f3 45 0f 58 f4              	addss	%xmm12, %xmm14
1407b04c8: f3 41 0f 51 de              	sqrtss	%xmm14, %xmm3
1407b04cd: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1407b04d4: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1407b04d9: 40 88 7c 24 20              	movb	%dil, 0x20(%rsp)
1407b04de: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407b04e5: 48 89 f2                    	movq	%rsi, %rdx
1407b04e8: 41 b0 ff                    	movb	$-0x1, %r8b
1407b04eb: e8 10 1e ad 00              	callq	0x141282300 <.text+0x1281300>
1407b04f0: 8b 8d 10 05 00 00           	movl	0x510(%rbp), %ecx
1407b04f6: 48 89 8d d0 04 00 00        	movq	%rcx, 0x4d0(%rbp)
1407b04fd: 41 0f b6 4d 46              	movzbl	0x46(%r13), %ecx
1407b0502: 88 8d 07 05 00 00           	movb	%cl, 0x507(%rbp)
1407b0508: 48 8d 8d 38 04 00 00        	leaq	0x438(%rbp), %rcx
1407b050f: 0f 10 01                    	movups	(%rcx), %xmm0
1407b0512: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
1407b0516: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
1407b051a: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
1407b0521: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1407b0528: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1407b052f: f2 44 0f 10 a5 68 04 00 00  	movsd	0x468(%rbp), %xmm12
1407b0538: 66 4c 0f 6e e8              	movq	%rax, %xmm13
1407b053d: f3 0f 7e 85 30 04 00 00     	movq	0x430(%rbp), %xmm0
1407b0545: 66 44 0f 6c e8              	punpcklqdq	%xmm0, %xmm13   # xmm13 = xmm13[0],xmm0[0]
1407b054a: be 05 00 00 00              	movl	$0x5, %esi
1407b054f: eb 1f                       	jmp	0x1407b0570 <.text+0x7af570>
1407b0551: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407b0558: 83 38 0b                    	cmpl	$0xb, (%rax)
1407b055b: 0f 85 30 04 00 00           	jne	0x1407b0991 <.text+0x7af991>
1407b0561: 80 78 04 3f                 	cmpb	$0x3f, 0x4(%rax)
1407b0565: 0f 85 26 04 00 00           	jne	0x1407b0991 <.text+0x7af991>
1407b056b: be 07 00 00 00              	movl	$0x7, %esi
1407b0570: 31 ff                       	xorl	%edi, %edi
1407b0572: 4c 8b b5 f0 04 00 00        	movq	0x4f0(%rbp), %r14
1407b0579: 45 89 fd                    	movl	%r15d, %r13d
1407b057c: e9 ef fa ff ff              	jmp	0x1407b0070 <.text+0x7af070>
1407b0581: 48 8b 95 70 03 00 00        	movq	0x370(%rbp), %rdx
1407b0588: 48 85 d2                    	testq	%rdx, %rdx
1407b058b: 74 16                       	je	0x1407b05a3 <.text+0x7af5a3>
1407b058d: 48 c1 e2 02                 	shlq	$0x2, %rdx
1407b0591: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407b0597: 48 8b 8d 08 05 00 00        	movq	0x508(%rbp), %rcx
1407b059e: e8 7d 36 e6 ff              	callq	0x140613c20 <.text+0x612c20>
1407b05a3: be 07 00 00 00              	movl	$0x7, %esi
1407b05a8: 31 ff                       	xorl	%edi, %edi
1407b05aa: e9 c1 fa ff ff              	jmp	0x1407b0070 <.text+0x7af070>
1407b05af: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407b05b6: 83 38 0b                    	cmpl	$0xb, (%rax)
1407b05b9: 0f 85 a6 02 00 00           	jne	0x1407b0865 <.text+0x7af865>
1407b05bf: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b05c6: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407b05cd: e8 7e 20 26 00              	callq	0x140a12650 <.text+0xa11650>
1407b05d2: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1407b05d9: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407b05e0: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
1407b05e7: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1407b05ee: 8b 85 78 03 00 00           	movl	0x378(%rbp), %eax
1407b05f4: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
1407b05fb: 89 41 08                    	movl	%eax, 0x8(%rcx)
1407b05fe: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
1407b0605: 48 89 01                    	movq	%rax, (%rcx)
1407b0608: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
1407b0612: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
1407b0616: c7 41 14 00 00 00 00        	movl	$0x0, 0x14(%rcx)
1407b061d: 48 8b 85 b0 04 00 00        	movq	0x4b0(%rbp), %rax
1407b0624: f3 44 0f 10 28              	movss	(%rax), %xmm13
1407b0629: f3 0f 10 70 04              	movss	0x4(%rax), %xmm6
1407b062e: f3 44 0f 10 60 08           	movss	0x8(%rax), %xmm12
1407b0634: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b063b: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407b0642: e8 09 20 26 00              	callq	0x140a12650 <.text+0xa11650>
1407b0647: f3 44 0f 10 8d 70 03 00 00  	movss	0x370(%rbp), %xmm9
1407b0650: f3 44 0f 10 bd 74 03 00 00  	movss	0x374(%rbp), %xmm15
1407b0659: f3 44 0f 10 b5 78 03 00 00  	movss	0x378(%rbp), %xmm14
1407b0662: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407b0669: 40 8a 70 04                 	movb	0x4(%rax), %sil
1407b066d: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0674: e8 37 89 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407b0679: f3 45 0f 5c e9              	subss	%xmm9, %xmm13
1407b067e: f3 45 0f 59 ed              	mulss	%xmm13, %xmm13
1407b0683: f3 41 0f 5c f7              	subss	%xmm15, %xmm6
1407b0688: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
1407b068c: f3 41 0f 58 f5              	addss	%xmm13, %xmm6
1407b0691: f3 45 0f 5c e6              	subss	%xmm14, %xmm12
1407b0696: f3 45 0f 59 e4              	mulss	%xmm12, %xmm12
1407b069b: f3 44 0f 58 e6              	addss	%xmm6, %xmm12
1407b06a0: f3 41 0f 51 dc              	sqrtss	%xmm12, %xmm3
1407b06a5: 48 8d 85 70 03 00 00        	leaq	0x370(%rbp), %rax
1407b06ac: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1407b06b1: 40 88 74 24 20              	movb	%sil, 0x20(%rsp)
1407b06b6: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
1407b06bc: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407b06c3: 48 8d 15 86 2f 2d 02        	leaq	0x22d2f86(%rip), %rdx   # 0x142a83650
1407b06ca: e8 f1 1c ad 00              	callq	0x1412823c0 <.text+0x12813c0>
1407b06cf: 48 89 85 e8 04 00 00        	movq	%rax, 0x4e8(%rbp)
1407b06d6: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407b06dd: 0f b6 40 04                 	movzbl	0x4(%rax), %eax
1407b06e1: 88 85 17 05 00 00           	movb	%al, 0x517(%rbp)
1407b06e7: 48 8b 85 e8 04 00 00        	movq	0x4e8(%rbp), %rax
1407b06ee: 48 c1 e8 08                 	shrq	$0x8, %rax
1407b06f2: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
1407b06f9: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1407b0700: 0f 28 8d d0 01 00 00        	movaps	0x1d0(%rbp), %xmm1
1407b0707: 0f 28 95 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm2
1407b070e: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
1407b0715: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
1407b071c: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
1407b0723: 0f 57 f6                    	xorps	%xmm6, %xmm6
1407b0726: e9 32 04 00 00              	jmp	0x1407b0b5d <.text+0x7afb5d>
1407b072b: 48 8b 8d f8 04 00 00        	movq	0x4f8(%rbp), %rcx
1407b0732: 83 39 0b                    	cmpl	$0xb, (%rcx)
1407b0735: 0f 85 2a 01 00 00           	jne	0x1407b0865 <.text+0x7af865>
1407b073b: 0f b6 71 04                 	movzbl	0x4(%rcx), %esi
1407b073f: 3c 02                       	cmpb	$0x2, %al
1407b0741: 48 8b 85 b0 04 00 00        	movq	0x4b0(%rbp), %rax
1407b0748: 75 16                       	jne	0x1407b0760 <.text+0x7af760>
1407b074a: 40 80 fe 3f                 	cmpb	$0x3f, %sil
1407b074e: 75 10                       	jne	0x1407b0760 <.text+0x7af760>
1407b0750: f6 85 31 04 00 00 01        	testb	$0x1, 0x431(%rbp)
1407b0757: 75 07                       	jne	0x1407b0760 <.text+0x7af760>
1407b0759: c6 85 30 04 00 00 0b        	movb	$0xb, 0x430(%rbp)
1407b0760: f3 44 0f 10 28              	movss	(%rax), %xmm13
1407b0765: f3 0f 10 70 04              	movss	0x4(%rax), %xmm6
1407b076a: f3 44 0f 10 60 08           	movss	0x8(%rax), %xmm12
1407b0770: f3 44 0f 10 8d 48 04 00 00  	movss	0x448(%rbp), %xmm9
1407b0779: f3 44 0f 10 bd 4c 04 00 00  	movss	0x44c(%rbp), %xmm15
1407b0782: f3 44 0f 10 b5 50 04 00 00  	movss	0x450(%rbp), %xmm14
1407b078b: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0792: e8 19 88 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407b0797: f3 45 0f 5c e9              	subss	%xmm9, %xmm13
1407b079c: f3 45 0f 59 ed              	mulss	%xmm13, %xmm13
1407b07a1: f3 41 0f 5c f7              	subss	%xmm15, %xmm6
1407b07a6: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
1407b07aa: f3 41 0f 58 f5              	addss	%xmm13, %xmm6
1407b07af: f3 45 0f 5c e6              	subss	%xmm14, %xmm12
1407b07b4: f3 45 0f 59 e4              	mulss	%xmm12, %xmm12
1407b07b9: f3 44 0f 58 e6              	addss	%xmm6, %xmm12
1407b07be: f3 41 0f 51 dc              	sqrtss	%xmm12, %xmm3
1407b07c3: 48 8d 85 70 03 00 00        	leaq	0x370(%rbp), %rax
1407b07ca: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1407b07cf: 40 88 74 24 20              	movb	%sil, 0x20(%rsp)
1407b07d4: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
1407b07da: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407b07e1: 48 8d 15 68 2e 2d 02        	leaq	0x22d2e68(%rip), %rdx   # 0x142a83650
1407b07e8: e8 d3 1b ad 00              	callq	0x1412823c0 <.text+0x12813c0>
1407b07ed: 48 89 85 e8 04 00 00        	movq	%rax, 0x4e8(%rbp)
1407b07f4: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
1407b07fb: 0f 10 8d 40 04 00 00        	movups	0x440(%rbp), %xmm1
1407b0802: 0f 10 95 50 04 00 00        	movups	0x450(%rbp), %xmm2
1407b0809: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
1407b0810: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
1407b0817: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
1407b081e: f2 0f 10 b5 60 04 00 00     	movsd	0x460(%rbp), %xmm6
1407b0826: 44 8b ad 68 04 00 00        	movl	0x468(%rbp), %r13d
1407b082d: 8b 85 6c 04 00 00           	movl	0x46c(%rbp), %eax
1407b0833: 89 85 e4 04 00 00           	movl	%eax, 0x4e4(%rbp)
1407b0839: 48 8b 85 f8 04 00 00        	movq	0x4f8(%rbp), %rax
1407b0840: 0f b6 40 04                 	movzbl	0x4(%rax), %eax
1407b0844: 88 85 17 05 00 00           	movb	%al, 0x517(%rbp)
1407b084a: 48 8b 85 e8 04 00 00        	movq	0x4e8(%rbp), %rax
1407b0851: 48 c1 e8 08                 	shrq	$0x8, %rax
1407b0855: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
1407b085c: 31 f6                       	xorl	%esi, %esi
1407b085e: 31 ff                       	xorl	%edi, %edi
1407b0860: e9 fc 02 00 00              	jmp	0x1407b0b61 <.text+0x7afb61>
1407b0865: be 06 00 00 00              	movl	$0x6, %esi
1407b086a: 31 ff                       	xorl	%edi, %edi
1407b086c: e9 c5 06 00 00              	jmp	0x1407b0f36 <.text+0x7aff36>
1407b0871: 83 f8 3b                    	cmpl	$0x3b, %eax
1407b0874: 0f 85 05 01 00 00           	jne	0x1407b097f <.text+0x7af97f>
1407b087a: 8b 95 10 05 00 00           	movl	0x510(%rbp), %edx
1407b0880: 48 8b 8d 88 01 00 00        	movq	0x188(%rbp), %rcx
1407b0887: e8 64 57 07 00              	callq	0x140825ff0 <.text+0x824ff0>
1407b088c: 48 85 c0                    	testq	%rax, %rax
1407b088f: 0f 84 ea 00 00 00           	je	0x1407b097f <.text+0x7af97f>
1407b0895: 4c 8b b0 18 01 00 00        	movq	0x118(%rax), %r14
1407b089c: 4d 85 f6                    	testq	%r14, %r14
1407b089f: 0f 84 da 00 00 00           	je	0x1407b097f <.text+0x7af97f>
1407b08a5: f2 0f 10 05 d3 2b 2d 02     	movsd	0x22d2bd3(%rip), %xmm0  # 0x142a83480
1407b08ad: f2 0f 11 85 48 01 00 00     	movsd	%xmm0, 0x148(%rbp)
1407b08b5: c7 85 50 01 00 00 cd cc ac 3f       	movl	$0x3faccccd, 0x150(%rbp) # imm = 0x3FACCCCD
1407b08bf: 49 8d 56 08                 	leaq	0x8(%r14), %rdx
1407b08c3: f3 41 0f 10 56 14           	movss	0x14(%r14), %xmm2
1407b08c9: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b08d0: e8 0b c5 4e 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407b08d5: f3 0f 10 85 74 03 00 00     	movss	0x374(%rbp), %xmm0
1407b08dd: f3 41 0f 58 c2              	addss	%xmm10, %xmm0
1407b08e2: f3 0f 10 8d 70 03 00 00     	movss	0x370(%rbp), %xmm1
1407b08ea: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1407b08ed: f3 0f 10 85 78 03 00 00     	movss	0x378(%rbp), %xmm0
1407b08f5: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
1407b08fa: 41 0f 58 cb                 	addps	%xmm11, %xmm1
1407b08fe: 0f 13 8d c0 01 00 00        	movlps	%xmm1, 0x1c0(%rbp)
1407b0905: f3 0f 11 85 c8 01 00 00     	movss	%xmm0, 0x1c8(%rbp)
1407b090d: f3 41 0f 10 5e 18           	movss	0x18(%r14), %xmm3
1407b0913: f3 41 0f 10 46 1c           	movss	0x1c(%r14), %xmm0
1407b0919: 48 8d 05 70 2d 2d 02        	leaq	0x22d2d70(%rip), %rax   # 0x142a83690
1407b0920: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1407b0925: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
1407b092b: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0932: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407b0939: 4c 8d 85 48 01 00 00        	leaq	0x148(%rbp), %r8
1407b0940: e8 3b 7a 4e 00              	callq	0x140c98380 <.text+0xc97380>
1407b0945: 48 8d 8d 38 04 00 00        	leaq	0x438(%rbp), %rcx
1407b094c: 8b 41 18                    	movl	0x18(%rcx), %eax
1407b094f: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
1407b0955: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1407b0959: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407b0960: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0967: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407b096e: e8 2d 7c 4e 00              	callq	0x140c985a0 <.text+0xc975a0>
1407b0973: 48 89 85 08 05 00 00        	movq	%rax, 0x508(%rbp)
1407b097a: e9 3c 03 00 00              	jmp	0x1407b0cbb <.text+0x7afcbb>
1407b097f: 45 31 ff                    	xorl	%r15d, %r15d
1407b0982: e9 c9 02 00 00              	jmp	0x1407b0c50 <.text+0x7afc50>
1407b0987: be 06 00 00 00              	movl	$0x6, %esi
1407b098c: e9 99 05 00 00              	jmp	0x1407b0f2a <.text+0x7aff2a>
1407b0991: 41 0f b6 45 22              	movzbl	0x22(%r13), %eax
1407b0996: 8d 48 fe                    	leal	-0x2(%rax), %ecx
1407b0999: 48 ff c8                    	decq	%rax
1407b099c: 31 d2                       	xorl	%edx, %edx
1407b099e: 80 f9 03                    	cmpb	$0x3, %cl
1407b09a1: 48 0f 43 c2                 	cmovaeq	%rdx, %rax
1407b09a5: 48 8d 0d 58 2e 2d 02        	leaq	0x22d2e58(%rip), %rcx   # 0x142a83804
1407b09ac: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1407b09b0: 48 01 c8                    	addq	%rcx, %rax
1407b09b3: ff e0                       	jmpq	*%rax
1407b09b5: 48 8d 8d 90 04 00 00        	leaq	0x490(%rbp), %rcx
1407b09bc: e8 7f b8 26 00              	callq	0x140a1c240 <.text+0xa1b240>
1407b09c1: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
1407b09c8: 48 89 c2                    	movq	%rax, %rdx
1407b09cb: 4c 8d 05 d6 2c 2d 02        	leaq	0x22d2cd6(%rip), %r8    # 0x142a836a8
1407b09d2: e8 19 15 2e 00              	callq	0x140a91ef0 <.text+0xa90ef0>
1407b09d7: 48 85 c0                    	testq	%rax, %rax
1407b09da: 0f 84 29 0a 00 00           	je	0x1407b1409 <.text+0x7b0409>
1407b09e0: 48 89 c1                    	movq	%rax, %rcx
1407b09e3: e8 f8 9c 37 00              	callq	0x140b2a6e0 <.text+0xb296e0>
1407b09e8: 0f 28 f0                    	movaps	%xmm0, %xmm6
1407b09eb: 4c 89 e9                    	movq	%r13, %rcx
1407b09ee: 48 83 c1 30                 	addq	$0x30, %rcx
1407b09f2: 48 8d 15 df 2c 2d 02        	leaq	0x22d2cdf(%rip), %rdx   # 0x142a836d8
1407b09f9: e8 b2 8a 08 00              	callq	0x1408394b0 <.text+0x8384b0>
1407b09fe: f3 0f 58 30                 	addss	(%rax), %xmm6
1407b0a02: f3 0f 58 f7                 	addss	%xmm7, %xmm6
1407b0a06: e9 8d 02 00 00              	jmp	0x1407b0c98 <.text+0x7afc98>
1407b0a0b: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0a12: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407b0a19: e8 32 1c 26 00              	callq	0x140a12650 <.text+0xa11650>
1407b0a1e: 48 8b 85 40 04 00 00        	movq	0x440(%rbp), %rax
1407b0a25: 48 89 85 d0 01 00 00        	movq	%rax, 0x1d0(%rbp)
1407b0a2c: 0f 10 85 30 04 00 00        	movups	0x430(%rbp), %xmm0
1407b0a33: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1407b0a3a: 8b 85 78 03 00 00           	movl	0x378(%rbp), %eax
1407b0a40: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
1407b0a47: 89 41 08                    	movl	%eax, 0x8(%rcx)
1407b0a4a: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
1407b0a51: 48 89 01                    	movq	%rax, (%rcx)
1407b0a54: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
1407b0a5e: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
1407b0a62: c7 41 14 00 00 00 00        	movl	$0x0, 0x14(%rcx)
1407b0a69: 48 8b 85 b0 04 00 00        	movq	0x4b0(%rbp), %rax
1407b0a70: f3 44 0f 10 28              	movss	(%rax), %xmm13
1407b0a75: f3 0f 10 70 04              	movss	0x4(%rax), %xmm6
1407b0a7a: f3 44 0f 10 60 08           	movss	0x8(%rax), %xmm12
1407b0a80: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0a87: 48 8b 95 28 04 00 00        	movq	0x428(%rbp), %rdx
1407b0a8e: e8 bd 1b 26 00              	callq	0x140a12650 <.text+0xa11650>
1407b0a93: f3 44 0f 10 8d 70 03 00 00  	movss	0x370(%rbp), %xmm9
1407b0a9c: f3 44 0f 10 bd 74 03 00 00  	movss	0x374(%rbp), %xmm15
1407b0aa5: f3 44 0f 10 b5 78 03 00 00  	movss	0x378(%rbp), %xmm14
1407b0aae: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0ab5: e8 f6 84 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407b0aba: f3 45 0f 5c e9              	subss	%xmm9, %xmm13
1407b0abf: f3 45 0f 59 ed              	mulss	%xmm13, %xmm13
1407b0ac4: f3 41 0f 5c f7              	subss	%xmm15, %xmm6
1407b0ac9: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
1407b0acd: f3 41 0f 58 f5              	addss	%xmm13, %xmm6
1407b0ad2: f3 45 0f 5c e6              	subss	%xmm14, %xmm12
1407b0ad7: f3 45 0f 59 e4              	mulss	%xmm12, %xmm12
1407b0adc: f3 44 0f 58 e6              	addss	%xmm6, %xmm12
1407b0ae1: f3 41 0f 51 dc              	sqrtss	%xmm12, %xmm3
1407b0ae6: 48 8d 85 70 03 00 00        	leaq	0x370(%rbp), %rax
1407b0aed: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1407b0af2: c6 44 24 20 3f              	movb	$0x3f, 0x20(%rsp)
1407b0af7: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
1407b0afd: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407b0b04: 48 8d 15 45 2b 2d 02        	leaq	0x22d2b45(%rip), %rdx   # 0x142a83650
1407b0b0b: e8 b0 18 ad 00              	callq	0x1412823c0 <.text+0x12813c0>
1407b0b10: 48 89 85 e8 04 00 00        	movq	%rax, 0x4e8(%rbp)
1407b0b17: 48 8b 85 e8 04 00 00        	movq	0x4e8(%rbp), %rax
1407b0b1e: 48 c1 e8 08                 	shrq	$0x8, %rax
1407b0b22: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
1407b0b29: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1407b0b30: 0f 28 8d d0 01 00 00        	movaps	0x1d0(%rbp), %xmm1
1407b0b37: 0f 28 95 e0 01 00 00        	movaps	0x1e0(%rbp), %xmm2
1407b0b3e: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
1407b0b45: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
1407b0b4c: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
1407b0b53: 0f 57 f6                    	xorps	%xmm6, %xmm6
1407b0b56: c6 85 17 05 00 00 3f        	movb	$0x3f, 0x517(%rbp)
1407b0b5d: 31 f6                       	xorl	%esi, %esi
1407b0b5f: 31 ff                       	xorl	%edi, %edi
1407b0b61: e9 ff 02 00 00              	jmp	0x1407b0e65 <.text+0x7afe65>
1407b0b66: 4c 89 e9                    	movq	%r13, %rcx
1407b0b69: 48 8d 15 d8 2b 2d 02        	leaq	0x22d2bd8(%rip), %rdx   # 0x142a83748
1407b0b70: e8 9b 28 08 00              	callq	0x140833410 <.text+0x832410>
1407b0b75: 48 89 c7                    	movq	%rax, %rdi
1407b0b78: f3 0f 10 08                 	movss	(%rax), %xmm1
1407b0b7c: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
1407b0b81: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1407b0b88: e8 d3 6d aa ff              	callq	0x140257960 <.text+0x256960>
1407b0b8d: 48 8d 4d 44                 	leaq	0x44(%rbp), %rcx
1407b0b91: 48 89 fa                    	movq	%rdi, %rdx
1407b0b94: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407b0b97: e8 44 c2 4e 00              	callq	0x140c9cde0 <.text+0xc9bde0>
1407b0b9c: c6 85 c0 01 00 00 1a        	movb	$0x1a, 0x1c0(%rbp)
1407b0ba3: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1407b0baa: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407b0bb1: e8 4a e5 ff ff              	callq	0x1407af100 <.text+0x7ae100>
1407b0bb6: 49 89 c7                    	movq	%rax, %r15
1407b0bb9: 48 89 c1                    	movq	%rax, %rcx
1407b0bbc: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407b0bc3: e8 a8 e6 ff ff              	callq	0x1407af270 <.text+0x7ae270>
1407b0bc8: 41 0f 10 47 20              	movups	0x20(%r15), %xmm0
1407b0bcd: 41 0f 10 4f 30              	movups	0x30(%r15), %xmm1
1407b0bd2: 0f 29 8d 80 03 00 00        	movaps	%xmm1, 0x380(%rbp)
1407b0bd9: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
1407b0be0: f3 0f 10 4f 08              	movss	0x8(%rdi), %xmm1
1407b0be5: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407b0bec: e8 2f 2f 08 00              	callq	0x140833b20 <.text+0x832b20>
1407b0bf1: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1407b0bf8: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
1407b0bfc: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0c03: 48 8d 55 30                 	leaq	0x30(%rbp), %rdx
1407b0c07: e8 e4 78 4e 00              	callq	0x140c984f0 <.text+0xc974f0>
1407b0c0c: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0c13: 48 8d 55 44                 	leaq	0x44(%rbp), %rdx
1407b0c17: e8 a4 78 4e 00              	callq	0x140c984c0 <.text+0xc974c0>
1407b0c1c: 48 8d 8d 38 04 00 00        	leaq	0x438(%rbp), %rcx
1407b0c23: 8b 41 18                    	movl	0x18(%rcx), %eax
1407b0c26: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
1407b0c2c: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1407b0c30: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1407b0c37: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0c3e: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
1407b0c45: e8 56 79 4e 00              	callq	0x140c985a0 <.text+0xc975a0>
1407b0c4a: 41 89 c7                    	movl	%eax, %r15d
1407b0c4d: 40 b7 01                    	movb	$0x1, %dil
1407b0c50: 48 c7 85 08 05 00 00 00 00 00 00    	movq	$0x0, 0x508(%rbp)
1407b0c5b: eb 61                       	jmp	0x1407b0cbe <.text+0x7afcbe>
1407b0c5d: f3 41 0f 10 4d 00           	movss	(%r13), %xmm1
1407b0c63: f3 41 0f 10 55 04           	movss	0x4(%r13), %xmm2
1407b0c69: 48 8b 8d f0 02 00 00        	movq	0x2f0(%rbp), %rcx
1407b0c70: e8 eb 6c aa ff              	callq	0x140257960 <.text+0x256960>
1407b0c75: 0f 28 f0                    	movaps	%xmm0, %xmm6
1407b0c78: 4c 89 e9                    	movq	%r13, %rcx
1407b0c7b: 48 83 c1 30                 	addq	$0x30, %rcx
1407b0c7f: 48 8d 15 6a 2a 2d 02        	leaq	0x22d2a6a(%rip), %rdx   # 0x142a836f0
1407b0c86: e8 25 88 08 00              	callq	0x1408394b0 <.text+0x8384b0>
1407b0c8b: f3 0f 58 30                 	addss	(%rax), %xmm6
1407b0c8f: f3 0f 58 f7                 	addss	%xmm7, %xmm6
1407b0c93: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
1407b0c98: f3 0f 10 85 4c 04 00 00     	movss	0x44c(%rbp), %xmm0
1407b0ca0: 0f 2e c6                    	ucomiss	%xmm6, %xmm0
1407b0ca3: 76 0b                       	jbe	0x1407b0cb0 <.text+0x7afcb0>
1407b0ca5: b0 01                       	movb	$0x1, %al
1407b0ca7: 48 89 85 08 05 00 00        	movq	%rax, 0x508(%rbp)
1407b0cae: eb 0b                       	jmp	0x1407b0cbb <.text+0x7afcbb>
1407b0cb0: 48 c7 85 08 05 00 00 00 00 00 00    	movq	$0x0, 0x508(%rbp)
1407b0cbb: 45 31 ff                    	xorl	%r15d, %r15d
1407b0cbe: 48 8b 85 b0 04 00 00        	movq	0x4b0(%rbp), %rax
1407b0cc5: f3 44 0f 10 20              	movss	(%rax), %xmm12
1407b0cca: f3 0f 10 70 04              	movss	0x4(%rax), %xmm6
1407b0ccf: f3 44 0f 5c a5 48 04 00 00  	subss	0x448(%rbp), %xmm12
1407b0cd8: f3 0f 5c b5 4c 04 00 00     	subss	0x44c(%rbp), %xmm6
1407b0ce0: f3 44 0f 10 68 08           	movss	0x8(%rax), %xmm13
1407b0ce6: f3 44 0f 5c ad 50 04 00 00  	subss	0x450(%rbp), %xmm13
1407b0cef: 45 8a 75 46                 	movb	0x46(%r13), %r14b
1407b0cf3: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1407b0cfa: e8 b1 82 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407b0cff: c7 45 50 07 00 00 00        	movl	$0x7, 0x50(%rbp)
1407b0d06: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1407b0d0d: 48 8d 95 70 03 00 00        	leaq	0x370(%rbp), %rdx
1407b0d14: 4c 8d 45 50                 	leaq	0x50(%rbp), %r8
1407b0d18: e8 b3 82 1f 00              	callq	0x1409a8fd0 <.text+0x9a7fd0>
1407b0d1d: f3 45 0f 59 e4              	mulss	%xmm12, %xmm12
1407b0d22: f3 0f 59 f6                 	mulss	%xmm6, %xmm6
1407b0d26: f3 41 0f 58 f4              	addss	%xmm12, %xmm6
1407b0d2b: f3 45 0f 59 ed              	mulss	%xmm13, %xmm13
1407b0d30: f3 44 0f 58 ee              	addss	%xmm6, %xmm13
1407b0d35: 0f 57 db                    	xorps	%xmm3, %xmm3
1407b0d38: f3 41 0f 51 dd              	sqrtss	%xmm13, %xmm3
1407b0d3d: 48 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %rax
1407b0d44: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1407b0d49: 44 88 74 24 20              	movb	%r14b, 0x20(%rsp)
1407b0d4e: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407b0d55: 48 89 f2                    	movq	%rsi, %rdx
1407b0d58: 41 b0 ff                    	movb	$-0x1, %r8b
1407b0d5b: e8 a0 15 ad 00              	callq	0x141282300 <.text+0x1281300>
1407b0d60: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
1407b0d67: 40 84 ff                    	testb	%dil, %dil
1407b0d6a: 0f 84 81 00 00 00           	je	0x1407b0df1 <.text+0x7afdf1>
1407b0d70: 48 8b 8d f8 02 00 00        	movq	0x2f8(%rbp), %rcx
1407b0d77: e8 04 49 11 00              	callq	0x1408c5680 <.text+0x8c4680>
1407b0d7c: 41 08 c7                    	orb	%al, %r15b
1407b0d7f: 75 75                       	jne	0x1407b0df6 <.text+0x7afdf6>
1407b0d81: 44 8b 8d 10 05 00 00        	movl	0x510(%rbp), %r9d
1407b0d88: 80 bd 08 05 00 00 00        	cmpb	$0x0, 0x508(%rbp)
1407b0d8f: 0f 84 dc 00 00 00           	je	0x1407b0e71 <.text+0x7afe71>
1407b0d95: 4c 8b 85 30 04 00 00        	movq	0x430(%rbp), %r8
1407b0d9c: 48 8d 85 38 04 00 00        	leaq	0x438(%rbp), %rax
1407b0da3: 0f 10 00                    	movups	(%rax), %xmm0
1407b0da6: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1407b0daa: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1407b0dae: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
1407b0db5: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
1407b0dbc: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
1407b0dc3: f2 0f 10 b5 68 04 00 00     	movsd	0x468(%rbp), %xmm6
1407b0dcb: 8b 85 e4 04 00 00           	movl	0x4e4(%rbp), %eax
1407b0dd1: 41 8a 45 46                 	movb	0x46(%r13), %al
1407b0dd5: 89 85 e4 04 00 00           	movl	%eax, 0x4e4(%rbp)
1407b0ddb: be 04 00 00 00              	movl	$0x4, %esi
1407b0de0: 31 ff                       	xorl	%edi, %edi
1407b0de2: 4c 8b b5 f0 04 00 00        	movq	0x4f0(%rbp), %r14
1407b0de9: 45 89 cd                    	movl	%r9d, %r13d
1407b0dec: e9 4c 01 00 00              	jmp	0x1407b0f3d <.text+0x7aff3d>
1407b0df1: 45 84 ff                    	testb	%r15b, %r15b
1407b0df4: 74 8b                       	je	0x1407b0d81 <.text+0x7afd81>
1407b0df6: 8b 8d 10 05 00 00           	movl	0x510(%rbp), %ecx
1407b0dfc: 4c 8b 85 30 04 00 00        	movq	0x430(%rbp), %r8
1407b0e03: 48 8d 85 38 04 00 00        	leaq	0x438(%rbp), %rax
1407b0e0a: 0f 10 00                    	movups	(%rax), %xmm0
1407b0e0d: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1407b0e11: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1407b0e15: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
1407b0e1c: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
1407b0e23: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
1407b0e2a: f2 0f 10 b5 68 04 00 00     	movsd	0x468(%rbp), %xmm6
1407b0e32: 41 0f b6 45 46              	movzbl	0x46(%r13), %eax
1407b0e37: 41 89 cd                    	movl	%ecx, %r13d
1407b0e3a: 48 89 85 e8 04 00 00        	movq	%rax, 0x4e8(%rbp)
1407b0e41: 83 c0 c8                    	addl	$-0x38, %eax
1407b0e44: 3c 06                       	cmpb	$0x6, %al
1407b0e46: ba 00 00 40 40              	movl	$0x40400000, %edx       # imm = 0x40400000
1407b0e4b: b9 00 00 00 00              	movl	$0x0, %ecx
1407b0e50: 0f 44 d1                    	cmovel	%ecx, %edx
1407b0e53: 3c 08                       	cmpb	$0x8, %al
1407b0e55: 0f 43 d1                    	cmovael	%ecx, %edx
1407b0e58: 89 95 e4 04 00 00           	movl	%edx, 0x4e4(%rbp)
1407b0e5e: be 03 00 00 00              	movl	$0x3, %esi
1407b0e63: 31 ff                       	xorl	%edi, %edi
1407b0e65: 4c 8b b5 f0 04 00 00        	movq	0x4f0(%rbp), %r14
1407b0e6c: e9 cc 00 00 00              	jmp	0x1407b0f3d <.text+0x7aff3d>
1407b0e71: 48 8b 85 30 04 00 00        	movq	0x430(%rbp), %rax
1407b0e78: 48 89 85 a8 04 00 00        	movq	%rax, 0x4a8(%rbp)
1407b0e7f: 48 8d 85 38 04 00 00        	leaq	0x438(%rbp), %rax
1407b0e86: 0f 10 00                    	movups	(%rax), %xmm0
1407b0e89: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1407b0e8d: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1407b0e91: 0f 29 85 70 03 00 00        	movaps	%xmm0, 0x370(%rbp)
1407b0e98: 0f 29 8d 80 03 00 00        	movaps	%xmm1, 0x380(%rbp)
1407b0e9f: 0f 29 95 90 03 00 00        	movaps	%xmm2, 0x390(%rbp)
1407b0ea6: f2 0f 10 b5 68 04 00 00     	movsd	0x468(%rbp), %xmm6
1407b0eae: 48 8b 95 90 04 00 00        	movq	0x490(%rbp), %rdx
1407b0eb5: 4c 8b 85 98 04 00 00        	movq	0x498(%rbp), %r8
1407b0ebc: 48 8b 8d 20 04 00 00        	movq	0x420(%rbp), %rcx
1407b0ec3: 45 89 cf                    	movl	%r9d, %r15d
1407b0ec6: e8 25 b7 26 00              	callq	0x140a1c5f0 <.text+0xa1b5f0>
1407b0ecb: 48 85 c0                    	testq	%rax, %rax
1407b0ece: 0f 84 0d 05 00 00           	je	0x1407b13e1 <.text+0x7b03e1>
1407b0ed4: 0f b6 40 46                 	movzbl	0x46(%rax), %eax
1407b0ed8: 31 c9                       	xorl	%ecx, %ecx
1407b0eda: 41 80 7d 46 3f              	cmpb	$0x3f, 0x46(%r13)
1407b0edf: 0f 95 c1                    	setne	%cl
1407b0ee2: 8b 95 e4 04 00 00           	movl	0x4e4(%rbp), %edx
1407b0ee8: 81 e2 00 00 ff ff           	andl	$0xffff0000, %edx       # imm = 0xFFFF0000
1407b0eee: 09 c2                       	orl	%eax, %edx
1407b0ef0: 0f 28 85 70 03 00 00        	movaps	0x370(%rbp), %xmm0
1407b0ef7: 0f 28 8d 80 03 00 00        	movaps	0x380(%rbp), %xmm1
1407b0efe: 0f 28 95 90 03 00 00        	movaps	0x390(%rbp), %xmm2
1407b0f05: 0f 29 95 30 03 00 00        	movaps	%xmm2, 0x330(%rbp)
1407b0f0c: 0f 29 8d 20 03 00 00        	movaps	%xmm1, 0x320(%rbp)
1407b0f13: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
1407b0f1a: c1 e1 08                    	shll	$0x8, %ecx
1407b0f1d: 09 ca                       	orl	%ecx, %edx
1407b0f1f: 89 95 e4 04 00 00           	movl	%edx, 0x4e4(%rbp)
1407b0f25: be 02 00 00 00              	movl	$0x2, %esi
1407b0f2a: 31 ff                       	xorl	%edi, %edi
1407b0f2c: 4c 8b b5 f0 04 00 00        	movq	0x4f0(%rbp), %r14
1407b0f33: 45 89 fd                    	movl	%r15d, %r13d
1407b0f36: 4c 8b 85 a8 04 00 00        	movq	0x4a8(%rbp), %r8
1407b0f3d: 0f 28 85 10 03 00 00        	movaps	0x310(%rbp), %xmm0
1407b0f44: 0f 28 8d 20 03 00 00        	movaps	0x320(%rbp), %xmm1
1407b0f4b: 0f 28 95 30 03 00 00        	movaps	0x330(%rbp), %xmm2
1407b0f52: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1407b0f59: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1407b0f60: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
1407b0f67: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
1407b0f6e: 48 c1 e0 08                 	shlq	$0x8, %rax
1407b0f72: 48 8b 8d e8 04 00 00        	movq	0x4e8(%rbp), %rcx
1407b0f79: 0f b6 c9                    	movzbl	%cl, %ecx
1407b0f7c: 48 89 8d e8 04 00 00        	movq	%rcx, 0x4e8(%rbp)
1407b0f83: 48 01 c8                    	addq	%rcx, %rax
1407b0f86: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
1407b0f8d: 4c 89 85 a8 04 00 00        	movq	%r8, 0x4a8(%rbp)
1407b0f94: 66 49 0f 6e c0              	movq	%r8, %xmm0
1407b0f99: f3 44 0f 7e ad 78 01 00 00  	movq	0x178(%rbp), %xmm13
1407b0fa2: 66 44 0f 6c e8              	punpcklqdq	%xmm0, %xmm13   # xmm13 = xmm13[0],xmm0[0]
1407b0fa7: 44 8b bd e4 04 00 00        	movl	0x4e4(%rbp), %r15d
1407b0fae: 44 88 bd 07 05 00 00        	movb	%r15b, 0x507(%rbp)
1407b0fb5: 41 81 e7 00 ff ff ff        	andl	$0xffffff00, %r15d      # imm = 0xFFFFFF00
1407b0fbc: 44 89 bd c0 04 00 00        	movl	%r15d, 0x4c0(%rbp)
1407b0fc3: 44 89 e8                    	movl	%r13d, %eax
1407b0fc6: 48 89 85 d0 04 00 00        	movq	%rax, 0x4d0(%rbp)
1407b0fcd: 44 0f 28 e6                 	movaps	%xmm6, %xmm12
1407b0fd1: e9 9a f0 ff ff              	jmp	0x1407b0070 <.text+0x7af070>
1407b0fd6: 4d 85 f6                    	testq	%r14, %r14
1407b0fd9: 74 19                       	je	0x1407b0ff4 <.text+0x7afff4>
1407b0fdb: 49 c1 e6 06                 	shlq	$0x6, %r14
1407b0fdf: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1407b0fe5: 48 8b 8d 68 03 00 00        	movq	0x368(%rbp), %rcx
1407b0fec: 4c 89 f2                    	movq	%r14, %rdx
1407b0fef: e8 2c 2c e6 ff              	callq	0x140613c20 <.text+0x612c20>
1407b0ff4: 48 89 f0                    	movq	%rsi, %rax
1407b0ff7: 48 83 f0 07                 	xorq	$0x7, %rax
1407b0ffb: 48 09 f8                    	orq	%rdi, %rax
1407b0ffe: 0f 85 91 01 00 00           	jne	0x1407b1195 <.text+0x7b0195>
1407b1004: be 06 00 00 00              	movl	$0x6, %esi
1407b1009: 31 ff                       	xorl	%edi, %edi
1407b100b: 44 0f b6 b5 17 05 00 00     	movzbl	0x517(%rbp), %r14d
1407b1013: e9 bc 01 00 00              	jmp	0x1407b11d4 <.text+0x7b01d4>
1407b1018: 8b 8d 90 03 00 00           	movl	0x390(%rbp), %ecx
1407b101e: 89 8d a0 00 00 00           	movl	%ecx, 0xa0(%rbp)
1407b1024: 8b 8d 9c 03 00 00           	movl	0x39c(%rbp), %ecx
1407b102a: 89 8d ac 00 00 00           	movl	%ecx, 0xac(%rbp)
1407b1030: 48 8b 8d 94 03 00 00        	movq	0x394(%rbp), %rcx
1407b1037: 48 89 8d a4 00 00 00        	movq	%rcx, 0xa4(%rbp)
1407b103e: 48 8b 8d a8 03 00 00        	movq	0x3a8(%rbp), %rcx
1407b1045: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
1407b104c: 0f 10 8d 80 03 00 00        	movups	0x380(%rbp), %xmm1
1407b1053: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
1407b105a: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
1407b1061: 0f 29 85 70 02 00 00        	movaps	%xmm0, 0x270(%rbp)
1407b1068: 0f 28 85 90 00 00 00        	movaps	0x90(%rbp), %xmm0
1407b106f: 0f 29 85 80 02 00 00        	movaps	%xmm0, 0x280(%rbp)
1407b1076: 8b 95 a0 00 00 00           	movl	0xa0(%rbp), %edx
1407b107c: 89 95 90 02 00 00           	movl	%edx, 0x290(%rbp)
1407b1082: 48 8b 95 a4 00 00 00        	movq	0xa4(%rbp), %rdx
1407b1089: 48 89 95 94 02 00 00        	movq	%rdx, 0x294(%rbp)
1407b1090: 8b 95 ac 00 00 00           	movl	0xac(%rbp), %edx
1407b1096: 89 95 9c 02 00 00           	movl	%edx, 0x29c(%rbp)
1407b109c: 66 0f 6e c1                 	movd	%ecx, %xmm0
1407b10a0: 48 c1 e9 20                 	shrq	$0x20, %rcx
1407b10a4: 48 89 8d d0 04 00 00        	movq	%rcx, 0x4d0(%rbp)
1407b10ab: 66 44 0f 6e e0              	movd	%eax, %xmm12
1407b10b0: 66 44 0f 62 e0              	punpckldq	%xmm0, %xmm12   # xmm12 = xmm12[0],xmm0[0],xmm12[1],xmm0[1]
1407b10b5: 48 8b 3f                    	movq	(%rdi), %rdi
1407b10b8: 44 0f b6 77 10              	movzbl	0x10(%rdi), %r14d
1407b10bd: f3 0f 10 43 04              	movss	0x4(%rbx), %xmm0
1407b10c2: f3 0f 5c 85 8c 02 00 00     	subss	0x28c(%rbp), %xmm0
1407b10ca: 0f 10 8d 88 02 00 00        	movups	0x288(%rbp), %xmm1
1407b10d1: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1407b10d5: f3 0f 10 53 08              	movss	0x8(%rbx), %xmm2
1407b10da: f3 0f 10 1b                 	movss	(%rbx), %xmm3
1407b10de: 0f 14 da                    	unpcklps	%xmm2, %xmm3            # xmm3 = xmm3[0],xmm2[0],xmm3[1],xmm2[1]
1407b10e1: f3 0f 10 95 90 02 00 00     	movss	0x290(%rbp), %xmm2
1407b10e9: 0f 16 d1                    	movlhps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0]
1407b10ec: 0f c6 d1 e2                 	shufps	$0xe2, %xmm1, %xmm2     # xmm2 = xmm2[2,0],xmm1[2,3]
1407b10f0: 0f 5c da                    	subps	%xmm2, %xmm3
1407b10f3: 0f 59 db                    	mulps	%xmm3, %xmm3
1407b10f6: f3 0f 58 c3                 	addss	%xmm3, %xmm0
1407b10fa: f3 0f 16 cb                 	movshdup	%xmm3, %xmm1            # xmm1 = xmm3[1,1,3,3]
1407b10fe: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407b1102: f3 0f 51 f1                 	sqrtss	%xmm1, %xmm6
1407b1106: f3 0f 58 35 02 13 2d 02     	addss	0x22d1302(%rip), %xmm6  # 0x142a82410
1407b110e: 48 89 f1                    	movq	%rsi, %rcx
1407b1111: e8 9a 7e 1f 00              	callq	0x1409a8fb0 <.text+0x9a7fb0>
1407b1116: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
1407b111b: 44 88 74 24 20              	movb	%r14b, 0x20(%rsp)
1407b1120: 48 8d 15 29 25 2d 02        	leaq	0x22d2529(%rip), %rdx   # 0x142a83650
1407b1127: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
1407b112d: 48 8b 8d d8 04 00 00        	movq	0x4d8(%rbp), %rcx
1407b1134: 0f 28 de                    	movaps	%xmm6, %xmm3
1407b1137: e8 84 12 ad 00              	callq	0x1412823c0 <.text+0x12813c0>
1407b113c: 66 44 0f 6f 2f              	movdqa	(%rdi), %xmm13
1407b1141: 0f 28 85 70 02 00 00        	movaps	0x270(%rbp), %xmm0
1407b1148: 0f 28 8d 80 02 00 00        	movaps	0x280(%rbp), %xmm1
1407b114f: 0f 28 95 90 02 00 00        	movaps	0x290(%rbp), %xmm2
1407b1156: 0f 29 95 00 01 00 00        	movaps	%xmm2, 0x100(%rbp)
1407b115d: 0f 29 8d f0 00 00 00        	movaps	%xmm1, 0xf0(%rbp)
1407b1164: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
1407b116b: be 01 00 00 00              	movl	$0x1, %esi
1407b1170: 41 b0 01                    	movb	$0x1, %r8b
1407b1173: 31 ff                       	xorl	%edi, %edi
1407b1175: eb 67                       	jmp	0x1407b11de <.text+0x7b01de>
1407b1177: 4d 85 f6                    	testq	%r14, %r14
1407b117a: 74 19                       	je	0x1407b1195 <.text+0x7b0195>
1407b117c: 49 c1 e6 06                 	shlq	$0x6, %r14
1407b1180: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1407b1186: 48 8b 8d 68 03 00 00        	movq	0x368(%rbp), %rcx
1407b118d: 4c 89 f2                    	movq	%r14, %rdx
1407b1190: e8 8b 2a e6 ff              	callq	0x140613c20 <.text+0x612c20>
1407b1195: 44 0f b6 b5 17 05 00 00     	movzbl	0x517(%rbp), %r14d
1407b119d: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1407b11a4: 0f 28 8d c0 00 00 00        	movaps	0xc0(%rbp), %xmm1
1407b11ab: 0f 28 95 d0 00 00 00        	movaps	0xd0(%rbp), %xmm2
1407b11b2: 0f 29 95 00 01 00 00        	movaps	%xmm2, 0x100(%rbp)
1407b11b9: 0f 29 8d f0 00 00 00        	movaps	%xmm1, 0xf0(%rbp)
1407b11c0: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
1407b11c7: 0f b6 85 07 05 00 00        	movzbl	0x507(%rbp), %eax
1407b11ce: 09 85 c0 04 00 00           	orl	%eax, 0x4c0(%rbp)
1407b11d4: 45 31 c0                    	xorl	%r8d, %r8d
1407b11d7: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
1407b11de: 8b 8d cc 04 00 00           	movl	0x4cc(%rbp), %ecx
1407b11e4: 48 8b 95 58 03 00 00        	movq	0x358(%rbp), %rdx
1407b11eb: 89 0a                       	movl	%ecx, (%rdx)
1407b11ed: 49 89 7c 24 08              	movq	%rdi, 0x8(%r12)
1407b11f2: 49 89 34 24                 	movq	%rsi, (%r12)
1407b11f6: 66 45 0f 7f 6c 24 10        	movdqa	%xmm13, 0x10(%r12)
1407b11fd: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
1407b1204: 0f 28 8d f0 00 00 00        	movaps	0xf0(%rbp), %xmm1
1407b120b: 0f 28 95 00 01 00 00        	movaps	0x100(%rbp), %xmm2
1407b1212: 41 0f 29 44 24 20           	movaps	%xmm0, 0x20(%r12)
1407b1218: 41 0f 29 4c 24 30           	movaps	%xmm1, 0x30(%r12)
1407b121e: 41 0f 29 54 24 40           	movaps	%xmm2, 0x40(%r12)
1407b1224: 45 0f 13 64 24 50           	movlps	%xmm12, 0x50(%r12)
1407b122a: 48 8b 8d d0 04 00 00        	movq	0x4d0(%rbp), %rcx
1407b1231: 41 89 4c 24 58              	movl	%ecx, 0x58(%r12)
1407b1236: 8b 8d c0 04 00 00           	movl	0x4c0(%rbp), %ecx
1407b123c: 41 89 4c 24 5c              	movl	%ecx, 0x5c(%r12)
1407b1241: 49 89 44 24 60              	movq	%rax, 0x60(%r12)
1407b1246: 45 88 44 24 68              	movb	%r8b, 0x68(%r12)
1407b124b: 45 88 74 24 69              	movb	%r14b, 0x69(%r12)
1407b1250: e9 f4 00 00 00              	jmp	0x1407b1349 <.text+0x7b0349>
1407b1255: b9 ab 30 00 00              	movl	$0x30ab, %ecx           # imm = 0x30AB
1407b125a: 4c 8b 9d f8 04 00 00        	movq	0x4f8(%rbp), %r11
1407b1261: e9 e9 eb ff ff              	jmp	0x1407afe4f <.text+0x7aee4f>
1407b1266: 4c 8b 8d 70 03 00 00        	movq	0x370(%rbp), %r9
1407b126d: 48 83 c1 a0                 	addq	$-0x60, %rcx
1407b1271: 31 c0                       	xorl	%eax, %eax
1407b1273: 84 d2                       	testb	%dl, %dl
1407b1275: 48 0f 45 c8                 	cmovneq	%rax, %rcx
1407b1279: 49 83 c2 a0                 	addq	$-0x60, %r10
1407b127d: 45 84 c0                    	testb	%r8b, %r8b
1407b1280: 4c 0f 45 d1                 	cmovneq	%rcx, %r10
1407b1284: 48 8b 8d 98 03 00 00        	movq	0x398(%rbp), %rcx
1407b128b: 49 01 09                    	addq	%rcx, (%r9)
1407b128e: 41 8b 4a 50                 	movl	0x50(%r10), %ecx
1407b1292: 4d 8b 42 08                 	movq	0x8(%r10), %r8
1407b1296: 49 8b 52 28                 	movq	0x28(%r10), %rdx
1407b129a: 45 8b 4a 30                 	movl	0x30(%r10), %r9d
1407b129e: 44 89 8d d0 02 00 00        	movl	%r9d, 0x2d0(%rbp)
1407b12a5: 48 89 95 c8 02 00 00        	movq	%rdx, 0x2c8(%rbp)
1407b12ac: 49 8b 52 34                 	movq	0x34(%r10), %rdx
1407b12b0: 48 89 95 d4 02 00 00        	movq	%rdx, 0x2d4(%rbp)
1407b12b7: 41 8b 52 3c                 	movl	0x3c(%r10), %edx
1407b12bb: 89 95 dc 02 00 00           	movl	%edx, 0x2dc(%rbp)
1407b12c1: 45 8b 4a 40                 	movl	0x40(%r10), %r9d
1407b12c5: f2 41 0f 10 42 44           	movsd	0x44(%r10), %xmm0
1407b12cb: 49 8b 52 20                 	movq	0x20(%r10), %rdx
1407b12cf: 48 89 95 c0 02 00 00        	movq	%rdx, 0x2c0(%rbp)
1407b12d6: 41 0f 10 4a 10              	movups	0x10(%r10), %xmm1
1407b12db: 0f 29 8d b0 02 00 00        	movaps	%xmm1, 0x2b0(%rbp)
1407b12e2: 41 8b 52 54                 	movl	0x54(%r10), %edx
1407b12e6: 45 0f b6 52 58              	movzbl	0x58(%r10), %r10d
1407b12eb: 41 bb 03 00 00 00           	movl	$0x3, %r11d
1407b12f1: 8b bd cc 04 00 00           	movl	0x4cc(%rbp), %edi
1407b12f7: 48 8b 9d 58 03 00 00        	movq	0x358(%rbp), %rbx
1407b12fe: 89 3b                       	movl	%edi, (%rbx)
1407b1300: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1407b1304: 4c 89 1e                    	movq	%r11, (%rsi)
1407b1307: 4c 89 46 10                 	movq	%r8, 0x10(%rsi)
1407b130b: 0f 28 8d b0 02 00 00        	movaps	0x2b0(%rbp), %xmm1
1407b1312: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
1407b1319: 4c 8b 85 c8 02 00 00        	movq	0x2c8(%rbp), %r8
1407b1320: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
1407b1327: 0f 11 4e 18                 	movups	%xmm1, 0x18(%rsi)
1407b132b: 48 89 46 28                 	movq	%rax, 0x28(%rsi)
1407b132f: 4c 89 46 30                 	movq	%r8, 0x30(%rsi)
1407b1333: 0f 11 56 38                 	movups	%xmm2, 0x38(%rsi)
1407b1337: 44 89 4e 48                 	movl	%r9d, 0x48(%rsi)
1407b133b: 0f 13 46 4c                 	movlps	%xmm0, 0x4c(%rsi)
1407b133f: 89 4e 58                    	movl	%ecx, 0x58(%rsi)
1407b1342: 89 56 5c                    	movl	%edx, 0x5c(%rsi)
1407b1345: 44 88 56 60                 	movb	%r10b, 0x60(%rsi)
1407b1349: 0f 28 b5 20 05 00 00        	movaps	0x520(%rbp), %xmm6
1407b1350: 0f 28 bd 30 05 00 00        	movaps	0x530(%rbp), %xmm7
1407b1357: 44 0f 28 85 40 05 00 00     	movaps	0x540(%rbp), %xmm8
1407b135f: 44 0f 28 8d 50 05 00 00     	movaps	0x550(%rbp), %xmm9
1407b1367: 44 0f 28 95 60 05 00 00     	movaps	0x560(%rbp), %xmm10
1407b136f: 44 0f 28 9d 70 05 00 00     	movaps	0x570(%rbp), %xmm11
1407b1377: 44 0f 28 a5 80 05 00 00     	movaps	0x580(%rbp), %xmm12
1407b137f: 44 0f 28 ad 90 05 00 00     	movaps	0x590(%rbp), %xmm13
1407b1387: 44 0f 28 b5 a0 05 00 00     	movaps	0x5a0(%rbp), %xmm14
1407b138f: 44 0f 28 bd b0 05 00 00     	movaps	0x5b0(%rbp), %xmm15
1407b1397: 48 81 c4 48 06 00 00        	addq	$0x648, %rsp            # imm = 0x648
1407b139e: 5b                          	popq	%rbx
1407b139f: 5f                          	popq	%rdi
1407b13a0: 5e                          	popq	%rsi
1407b13a1: 41 5c                       	popq	%r12
1407b13a3: 41 5d                       	popq	%r13
1407b13a5: 41 5e                       	popq	%r14
1407b13a7: 41 5f                       	popq	%r15
1407b13a9: 5d                          	popq	%rbp
1407b13aa: c3                          	retq
1407b13ab: 48 8d 0d 56 23 2d 02        	leaq	0x22d2356(%rip), %rcx   # 0x142a83708
1407b13b2: 4c 8d 05 bf 23 2d 02        	leaq	0x22d23bf(%rip), %r8    # 0x142a83778
1407b13b9: ba 28 00 00 00              	movl	$0x28, %edx
1407b13be: e8 6d 80 12 02              	callq	0x1428d9430 <.text+0x28d8430>
1407b13c3: eb 50                       	jmp	0x1407b1415 <.text+0x7b0415>
1407b13c5: 48 8d 0d ac 22 2d 02        	leaq	0x22d22ac(%rip), %rcx   # 0x142a83678
1407b13cc: e8 3f 7f 12 02              	callq	0x1428d9310 <.text+0x28d8310>
1407b13d1: eb 42                       	jmp	0x1407b1415 <.text+0x7b0415>
1407b13d3: 48 8d 0d 86 22 2d 02        	leaq	0x22d2286(%rip), %rcx   # 0x142a83660
1407b13da: e8 31 7f 12 02              	callq	0x1428d9310 <.text+0x28d8310>
1407b13df: eb 34                       	jmp	0x1407b1415 <.text+0x7b0415>
1407b13e1: 48 8d 0d 78 23 2d 02        	leaq	0x22d2378(%rip), %rcx   # 0x142a83760
1407b13e8: e8 23 7f 12 02              	callq	0x1428d9310 <.text+0x28d8310>
1407b13ed: eb 26                       	jmp	0x1407b1415 <.text+0x7b0415>
1407b13ef: 48 8d 0d 12 23 2d 02        	leaq	0x22d2312(%rip), %rcx   # 0x142a83708
1407b13f6: 4c 8d 05 33 23 2d 02        	leaq	0x22d2333(%rip), %r8    # 0x142a83730
1407b13fd: ba 28 00 00 00              	movl	$0x28, %edx
1407b1402: e8 29 80 12 02              	callq	0x1428d9430 <.text+0x28d8430>
1407b1407: eb 0c                       	jmp	0x1407b1415 <.text+0x7b0415>
1407b1409: 48 8d 0d b0 22 2d 02        	leaq	0x22d22b0(%rip), %rcx   # 0x142a836c0
1407b1410: e8 fb 7e 12 02              	callq	0x1428d9310 <.text+0x28d8310>
1407b1415: 0f 0b                       	ud2
