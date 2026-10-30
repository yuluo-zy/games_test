
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b37180: 55                          	pushq	%rbp
140b37181: 41 57                       	pushq	%r15
140b37183: 41 56                       	pushq	%r14
140b37185: 41 55                       	pushq	%r13
140b37187: 41 54                       	pushq	%r12
140b37189: 56                          	pushq	%rsi
140b3718a: 57                          	pushq	%rdi
140b3718b: 53                          	pushq	%rbx
140b3718c: 48 81 ec 38 02 00 00        	subq	$0x238, %rsp            # imm = 0x238
140b37193: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b3719b: 44 0f 29 85 a0 01 00 00     	movaps	%xmm8, 0x1a0(%rbp)
140b371a3: 0f 29 bd 90 01 00 00        	movaps	%xmm7, 0x190(%rbp)
140b371aa: 0f 29 b5 80 01 00 00        	movaps	%xmm6, 0x180(%rbp)
140b371b1: 48 c7 85 78 01 00 00 fe ff ff ff    	movq	$-0x2, 0x178(%rbp)
140b371bc: 4c 89 cf                    	movq	%r9, %rdi
140b371bf: 48 89 95 48 01 00 00        	movq	%rdx, 0x148(%rbp)
140b371c6: 48 89 8d 50 01 00 00        	movq	%rcx, 0x150(%rbp)
140b371cd: 41 8b 40 1c                 	movl	0x1c(%r8), %eax
140b371d1: 49 8b 48 10                 	movq	0x10(%r8), %rcx
140b371d5: 89 01                       	movl	%eax, (%rcx)
140b371d7: 49 8b 00                    	movq	(%r8), %rax
140b371da: 4c 8b b0 48 01 00 00        	movq	0x148(%rax), %r14
140b371e1: 4c 8b b8 50 01 00 00        	movq	0x150(%rax), %r15
140b371e8: 66 41 0f 6f 36              	movdqa	(%r14), %xmm6
140b371ed: 48 8b 98 60 01 00 00        	movq	0x160(%rax), %rbx
140b371f4: 44 0f 10 80 50 01 00 00     	movups	0x150(%rax), %xmm8
140b371fc: 0f 10 05 ad d9 fb 01        	movups	0x1fbd9ad(%rip), %xmm0  # 0x142af4bb0
140b37203: 0f 11 80 48 01 00 00        	movups	%xmm0, 0x148(%rax)
140b3720a: 0f 10 05 af d9 fb 01        	movups	0x1fbd9af(%rip), %xmm0  # 0x142af4bc0
140b37211: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
140b37218: 0f 11 80 58 01 00 00        	movups	%xmm0, 0x158(%rax)
140b3721f: e8 1c 65 21 00              	callq	0x140d4d740 <.text+0xd4c740>
140b37224: 48 89 c6                    	movq	%rax, %rsi
140b37227: f6 00 01                    	testb	$0x1, (%rax)
140b3722a: 4c 89 b5 58 01 00 00        	movq	%r14, 0x158(%rbp)
140b37231: 4c 89 bd 68 01 00 00        	movq	%r15, 0x168(%rbp)
140b37238: 74 11                       	je	0x140b3724b <.text+0xb3624b>
140b3723a: 48 89 f1                    	movq	%rsi, %rcx
140b3723d: 48 83 c1 08                 	addq	$0x8, %rcx
140b37241: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140b37245: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140b37249: eb 30                       	jmp	0x140b3727b <.text+0xb3627b>
140b3724b: c6 85 75 01 00 00 01        	movb	$0x1, 0x175(%rbp)
140b37252: e8 a9 db 22 00              	callq	0x140d64e00 <.text+0xd63e00>
140b37257: 48 89 f1                    	movq	%rsi, %rcx
140b3725a: 48 83 c1 08                 	addq	$0x8, %rcx
140b3725e: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140b37265: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
140b37269: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
140b3726d: 4c 8b b5 58 01 00 00        	movq	0x158(%rbp), %r14
140b37274: 4c 8b bd 68 01 00 00        	movq	0x168(%rbp), %r15
140b3727b: 4c 8b 85 60 01 00 00        	movq	0x160(%rbp), %r8
140b37282: 49 81 c0 48 01 00 00        	addq	$0x148, %r8             # imm = 0x148
140b37289: 66 44 0f d7 ce              	pmovmskb	%xmm6, %r9d
140b3728e: 41 f7 d1                    	notl	%r9d
140b37291: 4d 8d 56 10                 	leaq	0x10(%r14), %r10
140b37295: 4f 8d 5c 3e 01              	leaq	0x1(%r14,%r15), %r11
140b3729a: 48 8d 70 01                 	leaq	0x1(%rax), %rsi
140b3729e: 48 89 31                    	movq	%rsi, (%rcx)
140b372a1: f3 0f 6f 35 87 68 fb 01     	movdqu	0x1fb6887(%rip), %xmm6  # 0x142aedb30
140b372a9: 66 0f 7f b5 a0 00 00 00     	movdqa	%xmm6, 0xa0(%rbp)
140b372b1: 0f 10 3d 68 68 fb 01        	movups	0x1fb6868(%rip), %xmm7  # 0x142aedb20
140b372b8: 0f 29 bd 90 00 00 00        	movaps	%xmm7, 0x90(%rbp)
140b372bf: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140b372c6: 48 89 95 b8 00 00 00        	movq	%rdx, 0xb8(%rbp)
140b372cd: 4c 89 b5 e0 00 00 00        	movq	%r14, 0xe0(%rbp)
140b372d4: 4c 89 95 e8 00 00 00        	movq	%r10, 0xe8(%rbp)
140b372db: 4c 89 9d f0 00 00 00        	movq	%r11, 0xf0(%rbp)
140b372e2: 66 44 89 8d f8 00 00 00     	movw	%r9w, 0xf8(%rbp)
140b372ea: 48 89 9d 00 01 00 00        	movq	%rbx, 0x100(%rbp)
140b372f1: 4c 89 b5 08 01 00 00        	movq	%r14, 0x108(%rbp)
140b372f8: 44 0f 11 85 10 01 00 00     	movups	%xmm8, 0x110(%rbp)
140b37300: 48 89 9d 20 01 00 00        	movq	%rbx, 0x120(%rbp)
140b37307: 4c 89 85 28 01 00 00        	movq	%r8, 0x128(%rbp)
140b3730e: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
140b37315: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140b3731c: e8 0f c1 9e 00              	callq	0x141523430 <.text+0x1522430>
140b37321: 48 8b 85 30 02 00 00        	movq	0x230(%rbp), %rax
140b37328: 48 8b 8d 20 02 00 00        	movq	0x220(%rbp), %rcx
140b3732f: 66 0f 6f 85 90 00 00 00     	movdqa	0x90(%rbp), %xmm0
140b37337: 66 0f 6f 8d a0 00 00 00     	movdqa	0xa0(%rbp), %xmm1
140b3733f: 66 0f 6f 95 b0 00 00 00     	movdqa	0xb0(%rbp), %xmm2
140b37347: 66 0f 7f 45 b0              	movdqa	%xmm0, -0x50(%rbp)
140b3734c: 66 0f 7f 4d c0              	movdqa	%xmm1, -0x40(%rbp)
140b37351: 66 0f 7f 55 d0              	movdqa	%xmm2, -0x30(%rbp)
140b37356: 8b 51 1c                    	movl	0x1c(%rcx), %edx
140b37359: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
140b3735d: 89 95 58 01 00 00           	movl	%edx, 0x158(%rbp)
140b37363: 4c 89 85 c8 00 00 00        	movq	%r8, 0xc8(%rbp)
140b3736a: 41 89 10                    	movl	%edx, (%r8)
140b3736d: 48 8b 09                    	movq	(%rcx), %rcx
140b37370: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
140b37377: c6 41 30 00                 	movb	$0x0, 0x30(%rcx)
140b3737b: 8b 48 1c                    	movl	0x1c(%rax), %ecx
140b3737e: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140b37382: 89 8d 30 01 00 00           	movl	%ecx, 0x130(%rbp)
140b37388: 48 89 55 50                 	movq	%rdx, 0x50(%rbp)
140b3738c: 89 0a                       	movl	%ecx, (%rdx)
140b3738e: 48 8b 00                    	movq	(%rax), %rax
140b37391: 48 8b 70 08                 	movq	0x8(%rax), %rsi
140b37395: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140b3739c: 48 8b 58 10                 	movq	0x10(%rax), %rbx
140b373a0: e8 9b 63 21 00              	callq	0x140d4d740 <.text+0xd4c740>
140b373a5: 49 89 c7                    	movq	%rax, %r15
140b373a8: f6 00 01                    	testb	$0x1, (%rax)
140b373ab: 74 11                       	je	0x140b373be <.text+0xb363be>
140b373ad: 4c 89 f9                    	movq	%r15, %rcx
140b373b0: 48 83 c1 08                 	addq	$0x8, %rcx
140b373b4: 49 8b 47 08                 	movq	0x8(%r15), %rax
140b373b8: 49 8b 57 10                 	movq	0x10(%r15), %rdx
140b373bc: eb 22                       	jmp	0x140b373e0 <.text+0xb363e0>
140b373be: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
140b373c5: e8 36 da 22 00              	callq	0x140d64e00 <.text+0xd63e00>
140b373ca: 4c 89 f9                    	movq	%r15, %rcx
140b373cd: 48 83 c1 08                 	addq	$0x8, %rcx
140b373d1: 49 c7 07 01 00 00 00        	movq	$0x1, (%r15)
140b373d8: 49 89 47 08                 	movq	%rax, 0x8(%r15)
140b373dc: 49 89 57 10                 	movq	%rdx, 0x10(%r15)
140b373e0: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140b373e4: 4c 89 01                    	movq	%r8, (%rcx)
140b373e7: 48 69 cb 50 01 00 00        	imulq	$0x150, %rbx, %rcx      # imm = 0x150
140b373ee: 48 01 f1                    	addq	%rsi, %rcx
140b373f1: 66 0f 7f b5 f0 00 00 00     	movdqa	%xmm6, 0xf0(%rbp)
140b373f9: 0f 29 bd e0 00 00 00        	movaps	%xmm7, 0xe0(%rbp)
140b37400: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
140b37407: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
140b3740e: 48 89 b5 90 00 00 00        	movq	%rsi, 0x90(%rbp)
140b37415: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
140b3741c: 48 8d 45 b0                 	leaq	-0x50(%rbp), %rax
140b37420: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
140b37427: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
140b3742e: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140b37435: e8 b6 0b 94 00              	callq	0x141477ff0 <.text+0x1476ff0>
140b3743a: 66 0f 6f 85 e0 00 00 00     	movdqa	0xe0(%rbp), %xmm0
140b37442: 66 0f 6f 8d f0 00 00 00     	movdqa	0xf0(%rbp), %xmm1
140b3744a: 66 0f 6f 95 00 01 00 00     	movdqa	0x100(%rbp), %xmm2
140b37452: 66 0f 7f 8d a0 00 00 00     	movdqa	%xmm1, 0xa0(%rbp)
140b3745a: 66 0f 7f 85 90 00 00 00     	movdqa	%xmm0, 0x90(%rbp)
140b37462: 66 0f 7f 95 b0 00 00 00     	movdqa	%xmm2, 0xb0(%rbp)
140b3746a: 4c 8b b5 a8 00 00 00        	movq	0xa8(%rbp), %r14
140b37471: 4d 85 f6                    	testq	%r14, %r14
140b37474: 0f 84 2b 01 00 00           	je	0x140b375a5 <.text+0xb365a5>
140b3747a: 4c 8b ad 90 00 00 00        	movq	0x90(%rbp), %r13
140b37481: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
140b37487: 66 0f d7 f0                 	pmovmskb	%xmm0, %esi
140b3748b: f7 d6                       	notl	%esi
140b3748d: 49 8d 5d 10                 	leaq	0x10(%r13), %rbx
140b37491: 8b 47 1c                    	movl	0x1c(%rdi), %eax
140b37494: 89 85 38 01 00 00           	movl	%eax, 0x138(%rbp)
140b3749a: 4c 8b 27                    	movq	(%rdi), %r12
140b3749d: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140b374a1: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140b374a8: 49 8d bc 24 80 01 00 00     	leaq	0x180(%r12), %rdi
140b374b0: 49 81 c4 a0 01 00 00        	addq	$0x1a0, %r12            # imm = 0x1A0
140b374b7: 49 83 ee 01                 	subq	$0x1, %r14
140b374bb: 0f 82 98 00 00 00           	jb	0x140b37559 <.text+0xb36559>
140b374c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140b374d0: 66 85 f6                    	testw	%si, %si
140b374d3: 75 25                       	jne	0x140b374fa <.text+0xb364fa>
140b374d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140b374e0: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
140b374e4: 66 0f d7 f0                 	pmovmskb	%xmm0, %esi
140b374e8: 49 83 c5 80                 	addq	$-0x80, %r13
140b374ec: 48 83 c3 10                 	addq	$0x10, %rbx
140b374f0: 81 fe ff ff 00 00           	cmpl	$0xffff, %esi           # imm = 0xFFFF
140b374f6: 74 e8                       	je	0x140b374e0 <.text+0xb364e0>
140b374f8: f7 d6                       	notl	%esi
140b374fa: f3 0f bc c6                 	tzcntl	%esi, %eax
140b374fe: c1 e0 03                    	shll	$0x3, %eax
140b37501: 4d 89 ef                    	movq	%r13, %r15
140b37504: 49 29 c7                    	subq	%rax, %r15
140b37507: 49 83 c7 f8                 	addq	$-0x8, %r15
140b3750b: 8b 85 38 01 00 00           	movl	0x138(%rbp), %eax
140b37511: 48 8b 8d 40 01 00 00        	movq	0x140(%rbp), %rcx
140b37518: 89 01                       	movl	%eax, (%rcx)
140b3751a: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
140b37521: 4c 89 e1                    	movq	%r12, %rcx
140b37524: 4c 89 fa                    	movq	%r15, %rdx
140b37527: e8 b4 a6 a7 ff              	callq	0x1405b1be0 <.text+0x5b0be0>
140b3752c: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
140b37533: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140b3753a: 48 89 fa                    	movq	%rdi, %rdx
140b3753d: 49 89 c0                    	movq	%rax, %r8
140b37540: 4d 89 f9                    	movq	%r15, %r9
140b37543: e8 38 07 04 00              	callq	0x140b77c80 <.text+0xb76c80>
140b37548: 8d 46 ff                    	leal	-0x1(%rsi), %eax
140b3754b: 21 f0                       	andl	%esi, %eax
140b3754d: 89 c6                       	movl	%eax, %esi
140b3754f: 49 83 ee 01                 	subq	$0x1, %r14
140b37553: 0f 83 77 ff ff ff           	jae	0x140b374d0 <.text+0xb364d0>
140b37559: 48 8b 95 50 01 00 00        	movq	0x150(%rbp), %rdx
140b37560: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140b37563: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140b37567: 89 01                       	movl	%eax, (%rcx)
140b37569: 4c 8b 02                    	movq	(%rdx), %r8
140b3756c: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
140b37573: 8b 42 1c                    	movl	0x1c(%rdx), %eax
140b37576: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
140b3757a: 89 01                       	movl	%eax, (%rcx)
140b3757c: b9 90 00 00 00              	movl	$0x90, %ecx
140b37581: 48 03 0a                    	addq	(%rdx), %rcx
140b37584: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
140b3758b: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
140b37592: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
140b37599: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140b375a0: e8 7b 0f 73 00              	callq	0x141268520 <.text+0x1267520>
140b375a5: 48 8b 8d 28 02 00 00        	movq	0x228(%rbp), %rcx
140b375ac: 4c 8b 45 b0                 	movq	-0x50(%rbp), %r8
140b375b0: 48 8b 55 b8                 	movq	-0x48(%rbp), %rdx
140b375b4: 66 41 0f 6f 00              	movdqa	(%r8), %xmm0
140b375b9: 48 85 d2                    	testq	%rdx, %rdx
140b375bc: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
140b375c0: 74 2f                       	je	0x140b375f1 <.text+0xb365f1>
140b375c2: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
140b375ca: 48 83 e0 f0                 	andq	$-0x10, %rax
140b375ce: 48 01 c2                    	addq	%rax, %rdx
140b375d1: 48 83 c2 11                 	addq	$0x11, %rdx
140b375d5: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
140b375d9: 4c 89 c2                    	movq	%r8, %rdx
140b375dc: 48 29 c2                    	subq	%rax, %rdx
140b375df: 48 89 55 78                 	movq	%rdx, 0x78(%rbp)
140b375e3: b8 10 00 00 00              	movl	$0x10, %eax
140b375e8: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
140b375ef: eb 0b                       	jmp	0x140b375fc <.text+0xb365fc>
140b375f1: 48 c7 85 d8 00 00 00 00 00 00 00    	movq	$0x0, 0xd8(%rbp)
140b375fc: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140b37600: f7 d0                       	notl	%eax
140b37602: 4c 8b 8d 48 01 00 00        	movq	0x148(%rbp), %r9
140b37609: 41 8b 51 1c                 	movl	0x1c(%r9), %edx
140b3760d: 89 95 c0 00 00 00           	movl	%edx, 0xc0(%rbp)
140b37613: 49 8b 11                    	movq	(%r9), %rdx
140b37616: 4d 8b 49 10                 	movq	0x10(%r9), %r9
140b3761a: 4c 89 4d 10                 	movq	%r9, 0x10(%rbp)
140b3761e: 4c 8b 95 50 01 00 00        	movq	0x150(%rbp), %r10
140b37625: 45 8b 4a 1c                 	movl	0x1c(%r10), %r9d
140b37629: 44 89 8d 40 01 00 00        	movl	%r9d, 0x140(%rbp)
140b37630: 4d 8b 0a                    	movq	(%r10), %r9
140b37633: 4c 89 8d 48 01 00 00        	movq	%r9, 0x148(%rbp)
140b3763a: 4d 8b 4a 10                 	movq	0x10(%r10), %r9
140b3763e: 4c 89 8d 50 01 00 00        	movq	%r9, 0x150(%rbp)
140b37645: 44 8b 49 1c                 	movl	0x1c(%rcx), %r9d
140b37649: 44 89 8d c4 00 00 00        	movl	%r9d, 0xc4(%rbp)
140b37650: 4c 8b 09                    	movq	(%rcx), %r9
140b37653: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
140b37657: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
140b3765b: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140b37662: 4c 8d 91 98 01 00 00        	leaq	0x198(%rcx), %r10
140b37669: 4c 89 55 40                 	movq	%r10, 0x40(%rbp)
140b3766d: 4c 8d 91 68 01 00 00        	leaq	0x168(%rcx), %r10
140b37674: 4c 89 55 48                 	movq	%r10, 0x48(%rbp)
140b37678: 4c 8d 91 b8 01 00 00        	leaq	0x1b8(%rcx), %r10
140b3767f: 4c 89 55 38                 	movq	%r10, 0x38(%rbp)
140b37683: 48 81 c1 88 01 00 00        	addq	$0x188, %rcx            # imm = 0x188
140b3768a: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
140b37691: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140b37695: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
140b37699: 48 89 4d 00                 	movq	%rcx, (%rbp)
140b3769d: 49 8d 78 10                 	leaq	0x10(%r8), %rdi
140b376a1: 48 8d 4a 30                 	leaq	0x30(%rdx), %rcx
140b376a5: 48 89 4d 18                 	movq	%rcx, 0x18(%rbp)
140b376a9: 48 89 55 08                 	movq	%rdx, 0x8(%rbp)
140b376ad: 48 8d 4a 60                 	leaq	0x60(%rdx), %rcx
140b376b1: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
140b376b5: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
140b376bc: 48 83 c1 28                 	addq	$0x28, %rcx
140b376c0: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
140b376c4: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140b376cb: 48 83 c1 20                 	addq	$0x20, %rcx
140b376cf: 48 89 8d 38 01 00 00        	movq	%rcx, 0x138(%rbp)
140b376d6: 4c 89 4d 58                 	movq	%r9, 0x58(%rbp)
140b376da: 49 8d 49 20                 	leaq	0x20(%r9), %rcx
140b376de: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
140b376e2: 48 83 6d 00 01              	subq	$0x1, (%rbp)
140b376e7: 0f 82 9e 07 00 00           	jb	0x140b37e8b <.text+0xb36e8b>
140b376ed: 0f 1f 00                    	nopl	(%rax)
140b376f0: 66 85 c0                    	testw	%ax, %ax
140b376f3: 74 1b                       	je	0x140b37710 <.text+0xb36710>
140b376f5: 8d 48 ff                    	leal	-0x1(%rax), %ecx
140b376f8: 21 c1                       	andl	%eax, %ecx
140b376fa: 89 8d 34 01 00 00           	movl	%ecx, 0x134(%rbp)
140b37700: eb 38                       	jmp	0x140b3773a <.text+0xb3673a>
140b37702: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b37710: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
140b37714: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
140b37718: 49 83 c0 80                 	addq	$-0x80, %r8
140b3771c: 48 83 c7 10                 	addq	$0x10, %rdi
140b37720: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
140b37725: 74 e9                       	je	0x140b37710 <.text+0xb36710>
140b37727: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140b3772c: 29 c1                       	subl	%eax, %ecx
140b3772e: f7 d0                       	notl	%eax
140b37730: 89 c2                       	movl	%eax, %edx
140b37732: 21 ca                       	andl	%ecx, %edx
140b37734: 89 95 34 01 00 00           	movl	%edx, 0x134(%rbp)
140b3773a: f3 0f bc c0                 	tzcntl	%eax, %eax
140b3773e: c1 e0 03                    	shll	$0x3, %eax
140b37741: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
140b37745: 4c 89 c1                    	movq	%r8, %rcx
140b37748: 48 29 c1                    	subq	%rax, %rcx
140b3774b: 8b 59 f8                    	movl	-0x8(%rcx), %ebx
140b3774e: 44 8b 79 fc                 	movl	-0x4(%rcx), %r15d
140b37752: 8b 85 c0 00 00 00           	movl	0xc0(%rbp), %eax
140b37758: 48 8b 4d 10                 	movq	0x10(%rbp), %rcx
140b3775c: 89 01                       	movl	%eax, (%rcx)
140b3775e: 8b 85 40 01 00 00           	movl	0x140(%rbp), %eax
140b37764: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
140b3776b: 89 01                       	movl	%eax, (%rcx)
140b3776d: 44 89 7c 24 20              	movl	%r15d, 0x20(%rsp)
140b37772: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140b37779: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
140b37780: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
140b37784: 41 89 d9                    	movl	%ebx, %r9d
140b37787: e8 44 b3 71 ff              	callq	0x140252ad0 <.text+0x251ad0>
140b3778c: 80 bd 00 01 00 00 11        	cmpb	$0x11, 0x100(%rbp)
140b37793: 74 19                       	je	0x140b377ae <.text+0xb367ae>
140b37795: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
140b3779c: f0                          	lock
140b3779d: 48 ff 08                    	decq	(%rax)
140b377a0: 75 0c                       	jne	0x140b377ae <.text+0xb367ae>
140b377a2: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
140b377a9: e8 72 4e 02 00              	callq	0x140b5c620 <.text+0xb5b620>
140b377ae: 8b 85 40 01 00 00           	movl	0x140(%rbp), %eax
140b377b4: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
140b377bb: 89 01                       	movl	%eax, (%rcx)
140b377bd: 44 89 7c 24 20              	movl	%r15d, 0x20(%rsp)
140b377c2: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140b377c9: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
140b377d0: 4c 8b 45 18                 	movq	0x18(%rbp), %r8
140b377d4: 41 89 d9                    	movl	%ebx, %r9d
140b377d7: e8 f4 b2 71 ff              	callq	0x140252ad0 <.text+0x251ad0>
140b377dc: 80 bd 00 01 00 00 11        	cmpb	$0x11, 0x100(%rbp)
140b377e3: 74 19                       	je	0x140b377fe <.text+0xb367fe>
140b377e5: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
140b377ec: f0                          	lock
140b377ed: 48 ff 08                    	decq	(%rax)
140b377f0: 75 0c                       	jne	0x140b377fe <.text+0xb367fe>
140b377f2: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
140b377f9: e8 22 4e 02 00              	callq	0x140b5c620 <.text+0xb5b620>
140b377fe: 8b 85 40 01 00 00           	movl	0x140(%rbp), %eax
140b37804: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
140b3780b: 89 01                       	movl	%eax, (%rcx)
140b3780d: 44 89 7c 24 20              	movl	%r15d, 0x20(%rsp)
140b37812: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140b37819: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
140b37820: 4c 8b 45 20                 	movq	0x20(%rbp), %r8
140b37824: 41 89 d9                    	movl	%ebx, %r9d
140b37827: e8 a4 b2 71 ff              	callq	0x140252ad0 <.text+0x251ad0>
140b3782c: 80 bd 00 01 00 00 11        	cmpb	$0x11, 0x100(%rbp)
140b37833: 74 19                       	je	0x140b3784e <.text+0xb3684e>
140b37835: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
140b3783c: f0                          	lock
140b3783d: 48 ff 08                    	decq	(%rax)
140b37840: 75 0c                       	jne	0x140b3784e <.text+0xb3684e>
140b37842: 48 8d 8d 18 01 00 00        	leaq	0x118(%rbp), %rcx
140b37849: e8 d2 4d 02 00              	callq	0x140b5c620 <.text+0xb5b620>
140b3784e: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
140b37855: 89 da                       	movl	%ebx, %edx
140b37857: 45 89 f8                    	movl	%r15d, %r8d
140b3785a: e8 21 96 dc ff              	callq	0x140900e80 <.text+0x8ffe80>
140b3785f: 8b 8d 58 01 00 00           	movl	0x158(%rbp), %ecx
140b37865: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
140b3786c: 89 0a                       	movl	%ecx, (%rdx)
140b3786e: 84 c0                       	testb	%al, %al
140b37870: 0f 84 ea 00 00 00           	je	0x140b37960 <.text+0xb36960>
140b37876: 89 9d e0 00 00 00           	movl	%ebx, 0xe0(%rbp)
140b3787c: 44 89 bd e4 00 00 00        	movl	%r15d, 0xe4(%rbp)
140b37883: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140b3788a: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140b3788f: 0f 84 3b 01 00 00           	je	0x140b379d0 <.text+0xb369d0>
140b37895: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140b3789c: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140b378a3: e8 38 a3 a7 ff              	callq	0x1405b1be0 <.text+0x5b0be0>
140b378a8: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
140b378af: 48 8b 0a                    	movq	(%rdx), %rcx
140b378b2: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140b378b6: 49 89 d0                    	movq	%rdx, %r8
140b378b9: 49 21 c0                    	andq	%rax, %r8
140b378bc: 48 c1 e8 39                 	shrq	$0x39, %rax
140b378c0: 66 0f 6e c0                 	movd	%eax, %xmm0
140b378c4: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b378c8: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b378cd: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b378d2: 48 8d 81 f0 fe ff ff        	leaq	-0x110(%rcx), %rax
140b378d9: 45 31 c9                    	xorl	%r9d, %r9d
140b378dc: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140b378e2: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140b378e6: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140b378ea: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140b378ef: 45 85 d2                    	testl	%r10d, %r10d
140b378f2: 74 3c                       	je	0x140b37930 <.text+0xb36930>
140b378f4: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140b378f9: 4d 01 c3                    	addq	%r8, %r11
140b378fc: 49 21 d3                    	andq	%rdx, %r11
140b378ff: 49 f7 db                    	negq	%r11
140b37902: 4d 69 db 10 01 00 00        	imulq	$0x110, %r11, %r11      # imm = 0x110
140b37909: 42 39 1c 18                 	cmpl	%ebx, (%rax,%r11)
140b3790d: 75 0b                       	jne	0x140b3791a <.text+0xb3691a>
140b3790f: 46 39 7c 18 04              	cmpl	%r15d, 0x4(%rax,%r11)
140b37914: 0f 84 a9 00 00 00           	je	0x140b379c3 <.text+0xb369c3>
140b3791a: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140b3791e: 66 45 21 d3                 	andw	%r10w, %r11w
140b37922: 45 89 da                    	movl	%r11d, %r10d
140b37925: 75 cd                       	jne	0x140b378f4 <.text+0xb368f4>
140b37927: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b37930: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140b37934: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140b37939: 45 85 d2                    	testl	%r10d, %r10d
140b3793c: 0f 85 8e 00 00 00           	jne	0x140b379d0 <.text+0xb369d0>
140b37942: 4d 01 c8                    	addq	%r9, %r8
140b37945: 49 83 c0 10                 	addq	$0x10, %r8
140b37949: 49 83 c1 10                 	addq	$0x10, %r9
140b3794d: 49 21 d0                    	andq	%rdx, %r8
140b37950: eb 8a                       	jmp	0x140b378dc <.text+0xb368dc>
140b37952: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b37960: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140b37967: 89 da                       	movl	%ebx, %edx
140b37969: 45 89 f8                    	movl	%r15d, %r8d
140b3796c: 4c 8b 8d 38 02 00 00        	movq	0x238(%rbp), %r9
140b37973: e8 a8 0c f7 ff              	callq	0x140aa8620 <.text+0xaa7620>
140b37978: 89 9d e0 00 00 00           	movl	%ebx, 0xe0(%rbp)
140b3797e: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b37985: 48 8b 70 38                 	movq	0x38(%rax), %rsi
140b37989: 41 be 04 00 00 00           	movl	$0x4, %r14d
140b3798f: 48 85 f6                    	testq	%rsi, %rsi
140b37992: 0f 84 7f 01 00 00           	je	0x140b37b17 <.text+0xb36b17>
140b37998: 48 83 fe 01                 	cmpq	$0x1, %rsi
140b3799c: 0f 85 80 01 00 00           	jne	0x140b37b22 <.text+0xb36b22>
140b379a2: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b379a9: 48 8b 40 30                 	movq	0x30(%rax), %rax
140b379ad: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b379b3: 39 58 20                    	cmpl	%ebx, 0x20(%rax)
140b379b6: 0f 85 5e 02 00 00           	jne	0x140b37c1a <.text+0xb36c1a>
140b379bc: 31 d2                       	xorl	%edx, %edx
140b379be: e9 b2 01 00 00              	jmp	0x140b37b75 <.text+0xb36b75>
140b379c3: 42 c6 44 19 f8 01           	movb	$0x1, -0x8(%rcx,%r11)
140b379c9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b379d0: 8b 85 c4 00 00 00           	movl	0xc4(%rbp), %eax
140b379d6: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
140b379da: 89 01                       	movl	%eax, (%rcx)
140b379dc: 48 c1 e3 20                 	shlq	$0x20, %rbx
140b379e0: 4c 09 fb                    	orq	%r15, %rbx
140b379e3: 48 c7 85 80 00 00 00 01 00 00 00    	movq	$0x1, 0x80(%rbp)
140b379ee: 48 89 9d 88 00 00 00        	movq	%rbx, 0x88(%rbp)
140b379f5: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140b379f9: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
140b37a00: e8 ab 9a a7 ff              	callq	0x1405b14b0 <.text+0x5b04b0>
140b37a05: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140b37a0c: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
140b37a10: 49 89 c0                    	movq	%rax, %r8
140b37a13: 4c 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %r9
140b37a1a: e8 31 0b 04 00              	callq	0x140b78550 <.text+0xb77550>
140b37a1f: 48 83 bd e0 00 00 00 03     	cmpq	$0x3, 0xe0(%rbp)
140b37a27: b8 02 00 00 00              	movl	$0x2, %eax
140b37a2c: 74 06                       	je	0x140b37a34 <.text+0xb36a34>
140b37a2e: 8b 85 f8 00 00 00           	movl	0xf8(%rbp), %eax
140b37a34: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
140b37a3b: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140b37a3f: 88 45 f0                    	movb	%al, -0x10(%rbp)
140b37a42: 74 17                       	je	0x140b37a5b <.text+0xb36a5b>
140b37a44: f0                          	lock
140b37a45: 48 ff 09                    	decq	(%rcx)
140b37a48: 75 09                       	jne	0x140b37a53 <.text+0xb36a53>
140b37a4a: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140b37a4e: e8 7d 50 02 00              	callq	0x140b5cad0 <.text+0xb5bad0>
140b37a53: 48 8b 45 58                 	movq	0x58(%rbp), %rax
140b37a57: c6 40 38 01                 	movb	$0x1, 0x38(%rax)
140b37a5b: 8b 85 30 01 00 00           	movl	0x130(%rbp), %eax
140b37a61: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140b37a65: 89 01                       	movl	%eax, (%rcx)
140b37a67: 48 89 9d 80 00 00 00        	movq	%rbx, 0x80(%rbp)
140b37a6e: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140b37a72: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
140b37a79: e8 f2 92 a7 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b37a7e: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140b37a85: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
140b37a89: 49 89 c0                    	movq	%rax, %r8
140b37a8c: 4c 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %r9
140b37a93: e8 e8 0f 04 00              	callq	0x140b78a80 <.text+0xb77a80>
140b37a98: f6 85 e0 00 00 00 01        	testb	$0x1, 0xe0(%rbp)
140b37a9f: 74 23                       	je	0x140b37ac4 <.text+0xb36ac4>
140b37aa1: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
140b37aa8: 48 83 fa 02                 	cmpq	$0x2, %rdx
140b37aac: 72 16                       	jb	0x140b37ac4 <.text+0xb36ac4>
140b37aae: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
140b37ab5: 48 c1 e2 02                 	shlq	$0x2, %rdx
140b37ab9: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140b37abf: e8 5c c1 ad ff              	callq	0x140613c20 <.text+0x612c20>
140b37ac4: 8b 85 30 01 00 00           	movl	0x130(%rbp), %eax
140b37aca: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
140b37ace: 89 01                       	movl	%eax, (%rcx)
140b37ad0: 48 89 9d e0 00 00 00        	movq	%rbx, 0xe0(%rbp)
140b37ad7: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
140b37ade: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140b37ae5: e8 86 92 a7 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b37aea: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
140b37aee: 48 89 c2                    	movq	%rax, %rdx
140b37af1: 4c 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %r8
140b37af8: e8 b3 ea 03 00              	callq	0x140b765b0 <.text+0xb755b0>
140b37afd: 8b 85 34 01 00 00           	movl	0x134(%rbp), %eax
140b37b03: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
140b37b07: 48 83 6d 00 01              	subq	$0x1, (%rbp)
140b37b0c: 0f 83 de fb ff ff           	jae	0x140b376f0 <.text+0xb366f0>
140b37b12: e9 74 03 00 00              	jmp	0x140b37e8b <.text+0xb36e8b>
140b37b17: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b37b1d: e9 f8 00 00 00              	jmp	0x140b37c1a <.text+0xb36c1a>
140b37b22: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b37b29: 48 8b 48 60                 	movq	0x60(%rax), %rcx
140b37b2d: 48 8b 50 68                 	movq	0x68(%rax), %rdx
140b37b31: 41 89 d8                    	movl	%ebx, %r8d
140b37b34: e8 37 f0 5c ff              	callq	0x140106b70 <.text+0x105b70>
140b37b39: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140b37b3d: 48 89 c2                    	movq	%rax, %rdx
140b37b40: 4c 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %r8
140b37b47: e8 14 78 01 00              	callq	0x140b4f360 <.text+0xb4e360>
140b37b4c: 48 83 f8 01                 	cmpq	$0x1, %rax
140b37b50: 75 41                       	jne	0x140b37b93 <.text+0xb36b93>
140b37b52: 48 39 f2                    	cmpq	%rsi, %rdx
140b37b55: 0f 83 b0 03 00 00           	jae	0x140b37f0b <.text+0xb36f0b>
140b37b5b: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b37b62: 48 8b 40 30                 	movq	0x30(%rax), %rax
140b37b66: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b37b6c: 48 85 c0                    	testq	%rax, %rax
140b37b6f: 0f 84 a5 00 00 00           	je	0x140b37c1a <.text+0xb36c1a>
140b37b75: 48 8d 0c 92                 	leaq	(%rdx,%rdx,4), %rcx
140b37b79: 4c 8b 6c c8 08              	movq	0x8(%rax,%rcx,8), %r13
140b37b7e: 48 8b 44 c8 10              	movq	0x10(%rax,%rcx,8), %rax
140b37b83: 4c 8d 34 c5 00 00 00 00     	leaq	(,%rax,8), %r14
140b37b8b: 4d 01 ee                    	addq	%r13, %r14
140b37b8e: e9 87 00 00 00              	jmp	0x140b37c1a <.text+0xb36c1a>
140b37b93: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b37b99: e9 7c 00 00 00              	jmp	0x140b37c1a <.text+0xb36c1a>
140b37b9e: 66 90                       	nop
140b37ba0: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140b37ba6: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140b37baa: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140b37bae: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140b37bb3: 45 85 d2                    	testl	%r10d, %r10d
140b37bb6: 74 38                       	je	0x140b37bf0 <.text+0xb36bf0>
140b37bb8: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140b37bbd: 4d 01 c3                    	addq	%r8, %r11
140b37bc0: 49 21 d3                    	andq	%rdx, %r11
140b37bc3: 49 f7 db                    	negq	%r11
140b37bc6: 4d 69 db 10 01 00 00        	imulq	$0x110, %r11, %r11      # imm = 0x110
140b37bcd: 46 39 24 18                 	cmpl	%r12d, (%rax,%r11)
140b37bd1: 75 07                       	jne	0x140b37bda <.text+0xb36bda>
140b37bd3: 42 39 74 18 04              	cmpl	%esi, 0x4(%rax,%r11)
140b37bd8: 74 36                       	je	0x140b37c10 <.text+0xb36c10>
140b37bda: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140b37bde: 66 45 21 d3                 	andw	%r10w, %r11w
140b37be2: 45 89 da                    	movl	%r11d, %r10d
140b37be5: 75 d1                       	jne	0x140b37bb8 <.text+0xb36bb8>
140b37be7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b37bf0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140b37bf4: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140b37bf9: 45 85 d2                    	testl	%r10d, %r10d
140b37bfc: 75 18                       	jne	0x140b37c16 <.text+0xb36c16>
140b37bfe: 4d 01 c8                    	addq	%r9, %r8
140b37c01: 49 83 c0 10                 	addq	$0x10, %r8
140b37c05: 49 83 c1 10                 	addq	$0x10, %r9
140b37c09: 49 21 d0                    	andq	%rdx, %r8
140b37c0c: eb 92                       	jmp	0x140b37ba0 <.text+0xb36ba0>
140b37c0e: 66 90                       	nop
140b37c10: 42 c6 44 19 f8 01           	movb	$0x1, -0x8(%rcx,%r11)
140b37c16: 49 83 c5 08                 	addq	$0x8, %r13
140b37c1a: 4d 39 f5                    	cmpq	%r14, %r13
140b37c1d: 0f 84 8d 00 00 00           	je	0x140b37cb0 <.text+0xb36cb0>
140b37c23: 41 8b 75 00                 	movl	(%r13), %esi
140b37c27: 39 f3                       	cmpl	%esi, %ebx
140b37c29: 41 89 f4                    	movl	%esi, %r12d
140b37c2c: 44 0f 42 e3                 	cmovbl	%ebx, %r12d
140b37c30: 8b 85 58 01 00 00           	movl	0x158(%rbp), %eax
140b37c36: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
140b37c3d: 89 01                       	movl	%eax, (%rcx)
140b37c3f: 0f 47 f3                    	cmoval	%ebx, %esi
140b37c42: 44 89 a5 e0 00 00 00        	movl	%r12d, 0xe0(%rbp)
140b37c49: 89 b5 e4 00 00 00           	movl	%esi, 0xe4(%rbp)
140b37c4f: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140b37c56: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140b37c5b: 74 b9                       	je	0x140b37c16 <.text+0xb36c16>
140b37c5d: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140b37c64: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140b37c6b: e8 70 9f a7 ff              	callq	0x1405b1be0 <.text+0x5b0be0>
140b37c70: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
140b37c77: 48 8b 0a                    	movq	(%rdx), %rcx
140b37c7a: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140b37c7e: 49 89 d0                    	movq	%rdx, %r8
140b37c81: 49 21 c0                    	andq	%rax, %r8
140b37c84: 48 c1 e8 39                 	shrq	$0x39, %rax
140b37c88: 66 0f 6e c0                 	movd	%eax, %xmm0
140b37c8c: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b37c90: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b37c95: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b37c9a: 48 8d 81 f0 fe ff ff        	leaq	-0x110(%rcx), %rax
140b37ca1: 45 31 c9                    	xorl	%r9d, %r9d
140b37ca4: e9 f7 fe ff ff              	jmp	0x140b37ba0 <.text+0xb36ba0>
140b37ca9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b37cb0: 44 89 bd e0 00 00 00        	movl	%r15d, 0xe0(%rbp)
140b37cb7: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b37cbe: 48 8b 70 38                 	movq	0x38(%rax), %rsi
140b37cc2: 41 be 04 00 00 00           	movl	$0x4, %r14d
140b37cc8: 48 85 f6                    	testq	%rsi, %rsi
140b37ccb: 74 25                       	je	0x140b37cf2 <.text+0xb36cf2>
140b37ccd: 48 83 fe 01                 	cmpq	$0x1, %rsi
140b37cd1: 75 2a                       	jne	0x140b37cfd <.text+0xb36cfd>
140b37cd3: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b37cda: 48 8b 40 30                 	movq	0x30(%rax), %rax
140b37cde: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b37ce4: 44 39 78 20                 	cmpl	%r15d, 0x20(%rax)
140b37ce8: 0f 85 0c 01 00 00           	jne	0x140b37dfa <.text+0xb36dfa>
140b37cee: 31 d2                       	xorl	%edx, %edx
140b37cf0: eb 5e                       	jmp	0x140b37d50 <.text+0xb36d50>
140b37cf2: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b37cf8: e9 fd 00 00 00              	jmp	0x140b37dfa <.text+0xb36dfa>
140b37cfd: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b37d04: 48 8b 48 60                 	movq	0x60(%rax), %rcx
140b37d08: 48 8b 50 68                 	movq	0x68(%rax), %rdx
140b37d0c: 45 89 f8                    	movl	%r15d, %r8d
140b37d0f: e8 5c ee 5c ff              	callq	0x140106b70 <.text+0x105b70>
140b37d14: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
140b37d18: 48 89 c2                    	movq	%rax, %rdx
140b37d1b: 4c 8d 85 e0 00 00 00        	leaq	0xe0(%rbp), %r8
140b37d22: e8 39 76 01 00              	callq	0x140b4f360 <.text+0xb4e360>
140b37d27: 48 83 f8 01                 	cmpq	$0x1, %rax
140b37d2b: 75 41                       	jne	0x140b37d6e <.text+0xb36d6e>
140b37d2d: 48 39 f2                    	cmpq	%rsi, %rdx
140b37d30: 0f 83 e9 01 00 00           	jae	0x140b37f1f <.text+0xb36f1f>
140b37d36: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140b37d3d: 48 8b 40 30                 	movq	0x30(%rax), %rax
140b37d41: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b37d47: 48 85 c0                    	testq	%rax, %rax
140b37d4a: 0f 84 aa 00 00 00           	je	0x140b37dfa <.text+0xb36dfa>
140b37d50: 48 8d 0c 92                 	leaq	(%rdx,%rdx,4), %rcx
140b37d54: 4c 8b 6c c8 08              	movq	0x8(%rax,%rcx,8), %r13
140b37d59: 48 8b 44 c8 10              	movq	0x10(%rax,%rcx,8), %rax
140b37d5e: 4c 8d 34 c5 00 00 00 00     	leaq	(,%rax,8), %r14
140b37d66: 4d 01 ee                    	addq	%r13, %r14
140b37d69: e9 8c 00 00 00              	jmp	0x140b37dfa <.text+0xb36dfa>
140b37d6e: 41 bd 04 00 00 00           	movl	$0x4, %r13d
140b37d74: e9 81 00 00 00              	jmp	0x140b37dfa <.text+0xb36dfa>
140b37d79: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b37d80: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
140b37d86: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
140b37d8a: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
140b37d8e: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
140b37d93: 45 85 d2                    	testl	%r10d, %r10d
140b37d96: 74 38                       	je	0x140b37dd0 <.text+0xb36dd0>
140b37d98: f3 45 0f bc da              	tzcntl	%r10d, %r11d
140b37d9d: 4d 01 c3                    	addq	%r8, %r11
140b37da0: 49 21 d3                    	andq	%rdx, %r11
140b37da3: 49 f7 db                    	negq	%r11
140b37da6: 4d 69 db 10 01 00 00        	imulq	$0x110, %r11, %r11      # imm = 0x110
140b37dad: 46 39 24 18                 	cmpl	%r12d, (%rax,%r11)
140b37db1: 75 07                       	jne	0x140b37dba <.text+0xb36dba>
140b37db3: 42 39 74 18 04              	cmpl	%esi, 0x4(%rax,%r11)
140b37db8: 74 36                       	je	0x140b37df0 <.text+0xb36df0>
140b37dba: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
140b37dbe: 66 45 21 d3                 	andw	%r10w, %r11w
140b37dc2: 45 89 da                    	movl	%r11d, %r10d
140b37dc5: 75 d1                       	jne	0x140b37d98 <.text+0xb36d98>
140b37dc7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140b37dd0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
140b37dd4: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
140b37dd9: 45 85 d2                    	testl	%r10d, %r10d
140b37ddc: 75 18                       	jne	0x140b37df6 <.text+0xb36df6>
140b37dde: 4d 01 c8                    	addq	%r9, %r8
140b37de1: 49 83 c0 10                 	addq	$0x10, %r8
140b37de5: 49 83 c1 10                 	addq	$0x10, %r9
140b37de9: 49 21 d0                    	andq	%rdx, %r8
140b37dec: eb 92                       	jmp	0x140b37d80 <.text+0xb36d80>
140b37dee: 66 90                       	nop
140b37df0: 42 c6 44 19 f8 01           	movb	$0x1, -0x8(%rcx,%r11)
140b37df6: 49 83 c5 08                 	addq	$0x8, %r13
140b37dfa: 4d 39 f5                    	cmpq	%r14, %r13
140b37dfd: 0f 84 cd fb ff ff           	je	0x140b379d0 <.text+0xb369d0>
140b37e03: 41 8b 75 00                 	movl	(%r13), %esi
140b37e07: 41 39 f7                    	cmpl	%esi, %r15d
140b37e0a: 41 89 f4                    	movl	%esi, %r12d
140b37e0d: 45 0f 42 e7                 	cmovbl	%r15d, %r12d
140b37e11: 8b 85 58 01 00 00           	movl	0x158(%rbp), %eax
140b37e17: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
140b37e1e: 89 01                       	movl	%eax, (%rcx)
140b37e20: 41 0f 47 f7                 	cmoval	%r15d, %esi
140b37e24: 44 89 a5 e0 00 00 00        	movl	%r12d, 0xe0(%rbp)
140b37e2b: 89 b5 e4 00 00 00           	movl	%esi, 0xe4(%rbp)
140b37e31: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140b37e38: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
140b37e3d: 74 b7                       	je	0x140b37df6 <.text+0xb36df6>
140b37e3f: 48 8b 8d 38 01 00 00        	movq	0x138(%rbp), %rcx
140b37e46: 48 8d 95 e0 00 00 00        	leaq	0xe0(%rbp), %rdx
140b37e4d: e8 8e 9d a7 ff              	callq	0x1405b1be0 <.text+0x5b0be0>
140b37e52: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
140b37e59: 48 8b 0a                    	movq	(%rdx), %rcx
140b37e5c: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140b37e60: 49 89 d0                    	movq	%rdx, %r8
140b37e63: 49 21 c0                    	andq	%rax, %r8
140b37e66: 48 c1 e8 39                 	shrq	$0x39, %rax
140b37e6a: 66 0f 6e c0                 	movd	%eax, %xmm0
140b37e6e: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b37e72: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b37e77: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b37e7c: 48 8d 81 f0 fe ff ff        	leaq	-0x110(%rcx), %rax
140b37e83: 45 31 c9                    	xorl	%r9d, %r9d
140b37e86: e9 f5 fe ff ff              	jmp	0x140b37d80 <.text+0xb36d80>
140b37e8b: 48 83 7d 68 00              	cmpq	$0x0, 0x68(%rbp)
140b37e90: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140b37e94: 74 15                       	je	0x140b37eab <.text+0xb36eab>
140b37e96: 48 85 d2                    	testq	%rdx, %rdx
140b37e99: 74 10                       	je	0x140b37eab <.text+0xb36eab>
140b37e9b: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140b37e9f: 4c 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %r8
140b37ea6: e8 75 bd ad ff              	callq	0x140613c20 <.text+0x612c20>
140b37eab: 48 8b 95 98 00 00 00        	movq	0x98(%rbp), %rdx
140b37eb2: 48 85 d2                    	testq	%rdx, %rdx
140b37eb5: 74 2a                       	je	0x140b37ee1 <.text+0xb36ee1>
140b37eb7: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
140b37ebf: 48 83 e0 f0                 	andq	$-0x10, %rax
140b37ec3: 48 01 c2                    	addq	%rax, %rdx
140b37ec6: 48 83 c2 11                 	addq	$0x11, %rdx
140b37eca: 74 15                       	je	0x140b37ee1 <.text+0xb36ee1>
140b37ecc: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140b37ed3: 48 29 c1                    	subq	%rax, %rcx
140b37ed6: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b37edc: e8 3f bd ad ff              	callq	0x140613c20 <.text+0x612c20>
140b37ee1: 0f 28 b5 80 01 00 00        	movaps	0x180(%rbp), %xmm6
140b37ee8: 0f 28 bd 90 01 00 00        	movaps	0x190(%rbp), %xmm7
140b37eef: 44 0f 28 85 a0 01 00 00     	movaps	0x1a0(%rbp), %xmm8
140b37ef7: 48 81 c4 38 02 00 00        	addq	$0x238, %rsp            # imm = 0x238
140b37efe: 5b                          	popq	%rbx
140b37eff: 5f                          	popq	%rdi
140b37f00: 5e                          	popq	%rsi
140b37f01: 41 5c                       	popq	%r12
140b37f03: 41 5d                       	popq	%r13
140b37f05: 41 5e                       	popq	%r14
140b37f07: 41 5f                       	popq	%r15
140b37f09: 5d                          	popq	%rbp
140b37f0a: c3                          	retq
140b37f0b: 4c 8d 05 9e 93 14 02        	leaq	0x214939e(%rip), %r8    # 0x142c812b0
140b37f12: 48 89 d1                    	movq	%rdx, %rcx
140b37f15: 48 89 f2                    	movq	%rsi, %rdx
140b37f18: e8 fb 15 da 01              	callq	0x1428d9518 <.text+0x28d8518>
140b37f1d: eb 12                       	jmp	0x140b37f31 <.text+0xb36f31>
140b37f1f: 4c 8d 05 8a 93 14 02        	leaq	0x214938a(%rip), %r8    # 0x142c812b0
140b37f26: 48 89 d1                    	movq	%rdx, %rcx
140b37f29: 48 89 f2                    	movq	%rsi, %rdx
140b37f2c: e8 e7 15 da 01              	callq	0x1428d9518 <.text+0x28d8518>
140b37f31: 0f 0b                       	ud2
140b37f33: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140b37f40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b37f45: 55                          	pushq	%rbp
140b37f46: 41 57                       	pushq	%r15
140b37f48: 41 56                       	pushq	%r14
140b37f4a: 41 55                       	pushq	%r13
140b37f4c: 41 54                       	pushq	%r12
140b37f4e: 56                          	pushq	%rsi
140b37f4f: 57                          	pushq	%rdi
140b37f50: 53                          	pushq	%rbx
140b37f51: 48 83 ec 58                 	subq	$0x58, %rsp
140b37f55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b37f5c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
140b37f62: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140b37f67: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140b37f6d: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
140b37f74: e8 d7 1f 61 ff              	callq	0x140149f50 <.text+0x148f50>
140b37f79: c6 85 75 01 00 00 00        	movb	$0x0, 0x175(%rbp)
140b37f80: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
140b37f86: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140b37f8b: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
140b37f91: 48 83 c4 58                 	addq	$0x58, %rsp
140b37f95: 5b                          	popq	%rbx
140b37f96: 5f                          	popq	%rdi
140b37f97: 5e                          	popq	%rsi
140b37f98: 41 5c                       	popq	%r12
140b37f9a: 41 5d                       	popq	%r13
140b37f9c: 41 5e                       	popq	%r14
140b37f9e: 41 5f                       	popq	%r15
140b37fa0: 5d                          	popq	%rbp
140b37fa1: c3                          	retq
140b37fa2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b37fb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b37fb5: 55                          	pushq	%rbp
140b37fb6: 41 57                       	pushq	%r15
140b37fb8: 41 56                       	pushq	%r14
140b37fba: 41 55                       	pushq	%r13
140b37fbc: 41 54                       	pushq	%r12
140b37fbe: 56                          	pushq	%rsi
140b37fbf: 57                          	pushq	%rdi
140b37fc0: 53                          	pushq	%rbx
140b37fc1: 48 83 ec 58                 	subq	$0x58, %rsp
140b37fc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b37fcc: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
140b37fd2: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140b37fd7: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140b37fdd: 80 bd 75 01 00 00 00        	cmpb	$0x0, 0x175(%rbp)
140b37fe4: 74 76                       	je	0x140b3805c <.text+0xb3705c>
140b37fe6: 48 83 bd 68 01 00 00 00     	cmpq	$0x0, 0x168(%rbp)
140b37fee: 74 35                       	je	0x140b38025 <.text+0xb37025>
140b37ff0: 48 8b b5 68 01 00 00        	movq	0x168(%rbp), %rsi
140b37ff7: 4c 8d 46 11                 	leaq	0x11(%rsi), %r8
140b37ffb: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
140b38002: b2 ff                       	movb	$-0x1, %dl
140b38004: e8 17 bc de 01              	callq	0x142923c20 <.text+0x2922c20>
140b38009: 48 8d 46 01                 	leaq	0x1(%rsi), %rax
140b3800d: 48 89 c1                    	movq	%rax, %rcx
140b38010: 48 c1 e9 03                 	shrq	$0x3, %rcx
140b38014: 48 83 e0 f8                 	andq	$-0x8, %rax
140b38018: 48 29 c8                    	subq	%rcx, %rax
140b3801b: 48 83 fe 08                 	cmpq	$0x8, %rsi
140b3801f: 48 0f 42 c6                 	cmovbq	%rsi, %rax
140b38023: eb 02                       	jmp	0x140b38027 <.text+0xb37027>
140b38025: 31 c0                       	xorl	%eax, %eax
140b38027: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
140b3802e: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
140b38035: 48 89 91 48 01 00 00        	movq	%rdx, 0x148(%rcx)
140b3803c: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
140b38043: 48 89 91 50 01 00 00        	movq	%rdx, 0x150(%rcx)
140b3804a: 48 89 81 58 01 00 00        	movq	%rax, 0x158(%rcx)
140b38051: 48 c7 81 60 01 00 00 00 00 00 00    	movq	$0x0, 0x160(%rcx)
140b3805c: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140b38061: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140b38066: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
140b3806c: 48 83 c4 58                 	addq	$0x58, %rsp
140b38070: 5b                          	popq	%rbx
140b38071: 5f                          	popq	%rdi
140b38072: 5e                          	popq	%rsi
140b38073: 41 5c                       	popq	%r12
140b38075: 41 5d                       	popq	%r13
140b38077: 41 5e                       	popq	%r14
140b38079: 41 5f                       	popq	%r15
140b3807b: 5d                          	popq	%rbp
140b3807c: c3                          	retq
140b3807d: 0f 1f 00                    	nopl	(%rax)
140b38080: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b38085: 55                          	pushq	%rbp
140b38086: 41 57                       	pushq	%r15
140b38088: 41 56                       	pushq	%r14
140b3808a: 41 55                       	pushq	%r13
140b3808c: 41 54                       	pushq	%r12
140b3808e: 56                          	pushq	%rsi
140b3808f: 57                          	pushq	%rdi
140b38090: 53                          	pushq	%rbx
140b38091: 48 83 ec 58                 	subq	$0x58, %rsp
140b38095: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3809c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
140b380a2: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140b380a7: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140b380ad: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
140b380b4: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
140b380bb: e8 90 1e 61 ff              	callq	0x140149f50 <.text+0x148f50>
140b380c0: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
140b380c7: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
140b380cd: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140b380d2: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
140b380d8: 48 83 c4 58                 	addq	$0x58, %rsp
140b380dc: 5b                          	popq	%rbx
140b380dd: 5f                          	popq	%rdi
140b380de: 5e                          	popq	%rsi
140b380df: 41 5c                       	popq	%r12
140b380e1: 41 5d                       	popq	%r13
140b380e3: 41 5e                       	popq	%r14
140b380e5: 41 5f                       	popq	%r15
140b380e7: 5d                          	popq	%rbp
140b380e8: c3                          	retq
140b380e9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140b380f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b380f5: 55                          	pushq	%rbp
140b380f6: 41 57                       	pushq	%r15
140b380f8: 41 56                       	pushq	%r14
140b380fa: 41 55                       	pushq	%r13
140b380fc: 41 54                       	pushq	%r12
140b380fe: 56                          	pushq	%rsi
140b380ff: 57                          	pushq	%rdi
140b38100: 53                          	pushq	%rbx
140b38101: 48 83 ec 58                 	subq	$0x58, %rsp
140b38105: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3810c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
140b38112: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140b38117: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140b3811d: 0f b6 9d 76 01 00 00        	movzbl	0x176(%rbp), %ebx
140b38124: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
140b3812b: e8 20 1e 61 ff              	callq	0x140149f50 <.text+0x148f50>
140b38130: 88 9d 77 01 00 00           	movb	%bl, 0x177(%rbp)
140b38136: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
140b3813c: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140b38141: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
140b38147: 48 83 c4 58                 	addq	$0x58, %rsp
140b3814b: 5b                          	popq	%rbx
140b3814c: 5f                          	popq	%rdi
140b3814d: 5e                          	popq	%rsi
140b3814e: 41 5c                       	popq	%r12
140b38150: 41 5d                       	popq	%r13
140b38152: 41 5e                       	popq	%r14
140b38154: 41 5f                       	popq	%r15
140b38156: 5d                          	popq	%rbp
140b38157: c3                          	retq
140b38158: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140b38160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b38165: 55                          	pushq	%rbp
140b38166: 41 57                       	pushq	%r15
140b38168: 41 56                       	pushq	%r14
140b3816a: 41 55                       	pushq	%r13
140b3816c: 41 54                       	pushq	%r12
140b3816e: 56                          	pushq	%rsi
140b3816f: 57                          	pushq	%rdi
140b38170: 53                          	pushq	%rbx
140b38171: 48 83 ec 58                 	subq	$0x58, %rsp
140b38175: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b3817c: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
140b38182: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140b38187: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140b3818d: 48 83 7d 68 00              	cmpq	$0x0, 0x68(%rbp)
140b38192: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140b38196: 74 15                       	je	0x140b381ad <.text+0xb371ad>
140b38198: 48 85 d2                    	testq	%rdx, %rdx
140b3819b: 74 10                       	je	0x140b381ad <.text+0xb371ad>
140b3819d: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140b381a1: 4c 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %r8
140b381a8: e8 73 ba ad ff              	callq	0x140613c20 <.text+0x612c20>
140b381ad: c6 85 76 01 00 00 00        	movb	$0x0, 0x176(%rbp)
140b381b4: 66 0f 6f 74 24 40           	movdqa	0x40(%rsp), %xmm6
140b381ba: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140b381bf: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
140b381c5: 48 83 c4 58                 	addq	$0x58, %rsp
140b381c9: 5b                          	popq	%rbx
140b381ca: 5f                          	popq	%rdi
140b381cb: 5e                          	popq	%rsi
140b381cc: 41 5c                       	popq	%r12
140b381ce: 41 5d                       	popq	%r13
140b381d0: 41 5e                       	popq	%r14
140b381d2: 41 5f                       	popq	%r15
140b381d4: 5d                          	popq	%rbp
140b381d5: c3                          	retq
140b381d6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140b381e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b381e5: 55                          	pushq	%rbp
140b381e6: 41 57                       	pushq	%r15
140b381e8: 41 56                       	pushq	%r14
140b381ea: 41 55                       	pushq	%r13
140b381ec: 41 54                       	pushq	%r12
140b381ee: 56                          	pushq	%rsi
140b381ef: 57                          	pushq	%rdi
140b381f0: 53                          	pushq	%rbx
140b381f1: 48 83 ec 58                 	subq	$0x58, %rsp
140b381f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b381fc: 44 0f 29 44 24 20           	movaps	%xmm8, 0x20(%rsp)
140b38202: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140b38207: 66 0f 7f 74 24 40           	movdqa	%xmm6, 0x40(%rsp)
140b3820d: 80 bd 77 01 00 00 00        	cmpb	$0x0, 0x177(%rbp)
140b38214: 75 21                       	jne	0x140b38237 <.text+0xb37237>
140b38216: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140b3821b: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140b38220: 44 0f 28 44 24 20           	movaps	0x20(%rsp), %xmm8
140b38226: 48 83 c4 58                 	addq	$0x58, %rsp
140b3822a: 5b                          	popq	%rbx
140b3822b: 5f                          	popq	%rdi
140b3822c: 5e                          	popq	%rsi
140b3822d: 41 5c                       	popq	%r12
140b3822f: 41 5d                       	popq	%r13
140b38231: 41 5e                       	popq	%r14
140b38233: 41 5f                       	popq	%r15
140b38235: 5d                          	popq	%rbp
140b38236: c3                          	retq
140b38237: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140b3823b: e8 10 1d 61 ff              	callq	0x140149f50 <.text+0x148f50>
140b38240: eb d4                       	jmp	0x140b38216 <.text+0xb37216>
