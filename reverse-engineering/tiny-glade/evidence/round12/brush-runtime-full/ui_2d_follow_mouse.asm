
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14092a100: 55                          	pushq	%rbp
14092a101: 41 57                       	pushq	%r15
14092a103: 41 56                       	pushq	%r14
14092a105: 41 55                       	pushq	%r13
14092a107: 41 54                       	pushq	%r12
14092a109: 56                          	pushq	%rsi
14092a10a: 57                          	pushq	%rdi
14092a10b: 53                          	pushq	%rbx
14092a10c: 48 83 ec 48                 	subq	$0x48, %rsp
14092a110: 48 8d 6c 24 40              	leaq	0x40(%rsp), %rbp
14092a115: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14092a11d: 48 89 d6                    	movq	%rdx, %rsi
14092a120: 48 89 cf                    	movq	%rcx, %rdi
14092a123: e8 d8 c8 2e 00              	callq	0x140c16a00 <.text+0xc15a00>
14092a128: 89 c3                       	movl	%eax, %ebx
14092a12a: 84 c0                       	testb	%al, %al
14092a12c: 0f 84 1e 02 00 00           	je	0x14092a350 <.text+0x929350>
14092a132: 4c 8d 3d 8f 44 18 02        	leaq	0x218448f(%rip), %r15   # 0x142aae5c8
14092a139: 41 be 3d 00 00 00           	movl	$0x3d, %r14d
14092a13f: ba 3d 00 00 00              	movl	$0x3d, %edx
14092a144: 4c 89 f9                    	movq	%r15, %rcx
14092a147: e8 d4 97 ff ff              	callq	0x140923920 <.text+0x922920>
14092a14c: 48 83 f8 01                 	cmpq	$0x1, %rax
14092a150: 0f 85 ab 00 00 00           	jne	0x14092a201 <.text+0x929201>
14092a156: 49 89 d1                    	movq	%rdx, %r9
14092a159: 48 85 d2                    	testq	%rdx, %rdx
14092a15c: 74 36                       	je	0x14092a194 <.text+0x929194>
14092a15e: 49 83 f9 3d                 	cmpq	$0x3d, %r9
14092a162: 73 2e                       	jae	0x14092a192 <.text+0x929192>
14092a164: 48 8d 05 5d 44 18 02        	leaq	0x218445d(%rip), %rax   # 0x142aae5c8
14092a16b: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14092a170: 7f 22                       	jg	0x14092a194 <.text+0x929194>
14092a172: 48 8d 05 bf 45 18 02        	leaq	0x21845bf(%rip), %rax   # 0x142aae738
14092a179: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14092a17e: 48 8d 0d 43 44 18 02        	leaq	0x2184443(%rip), %rcx   # 0x142aae5c8
14092a185: ba 3d 00 00 00              	movl	$0x3d, %edx
14092a18a: 45 31 c0                    	xorl	%r8d, %r8d
14092a18d: e8 ae f7 fa 01              	callq	0x1428d9940 <.text+0x28d8940>
14092a192: 75 de                       	jne	0x14092a172 <.text+0x929172>
14092a194: 4c 8d 3d 2d 44 18 02        	leaq	0x218442d(%rip), %r15   # 0x142aae5c8
14092a19b: 4c 89 f9                    	movq	%r15, %rcx
14092a19e: 4c 89 ca                    	movq	%r9, %rdx
14092a1a1: e8 7a 97 ff ff              	callq	0x140923920 <.text+0x922920>
14092a1a6: 48 83 f8 01                 	cmpq	$0x1, %rax
14092a1aa: 75 55                       	jne	0x14092a201 <.text+0x929201>
14092a1ac: 49 89 d0                    	movq	%rdx, %r8
14092a1af: 49 83 c0 02                 	addq	$0x2, %r8
14092a1b3: 74 39                       	je	0x14092a1ee <.text+0x9291ee>
14092a1b5: 49 83 f8 3d                 	cmpq	$0x3d, %r8
14092a1b9: 73 31                       	jae	0x14092a1ec <.text+0x9291ec>
14092a1bb: 48 8d 05 06 44 18 02        	leaq	0x2184406(%rip), %rax   # 0x142aae5c8
14092a1c2: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14092a1c7: 7f 25                       	jg	0x14092a1ee <.text+0x9291ee>
14092a1c9: 48 8d 05 80 45 18 02        	leaq	0x2184580(%rip), %rax   # 0x142aae750
14092a1d0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14092a1d5: 48 8d 0d ec 43 18 02        	leaq	0x21843ec(%rip), %rcx   # 0x142aae5c8
14092a1dc: ba 3d 00 00 00              	movl	$0x3d, %edx
14092a1e1: 41 b9 3d 00 00 00           	movl	$0x3d, %r9d
14092a1e7: e8 54 f7 fa 01              	callq	0x1428d9940 <.text+0x28d8940>
14092a1ec: 75 db                       	jne	0x14092a1c9 <.text+0x9291c9>
14092a1ee: 41 be 3b 00 00 00           	movl	$0x3b, %r14d
14092a1f4: 49 29 d6                    	subq	%rdx, %r14
14092a1f7: 4c 8d 3d ca 43 18 02        	leaq	0x21843ca(%rip), %r15   # 0x142aae5c8
14092a1fe: 4d 01 c7                    	addq	%r8, %r15
14092a201: 48 8d 05 cd 53 18 02        	leaq	0x21853cd(%rip), %rax   # 0x142aaf5d5
14092a208: 4c 8d 25 91 53 18 02        	leaq	0x2185391(%rip), %r12   # 0x142aaf5a0
14092a20f: 90                          	nop
14092a210: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
14092a214: 85 c9                       	testl	%ecx, %ecx
14092a216: 78 18                       	js	0x14092a230 <.text+0x929230>
14092a218: 48 ff c8                    	decq	%rax
14092a21b: 83 f9 5c                    	cmpl	$0x5c, %ecx
14092a21e: 75 69                       	jne	0x14092a289 <.text+0x929289>
14092a220: eb 7d                       	jmp	0x14092a29f <.text+0x92929f>
14092a222: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14092a230: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
14092a234: 80 fa c0                    	cmpb	$-0x40, %dl
14092a237: 7d 25                       	jge	0x14092a25e <.text+0x92925e>
14092a239: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
14092a23e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
14092a242: 7d 23                       	jge	0x14092a267 <.text+0x929267>
14092a244: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
14092a249: 48 83 c0 fc                 	addq	$-0x4, %rax
14092a24d: 41 83 e1 07                 	andl	$0x7, %r9d
14092a251: 41 c1 e1 06                 	shll	$0x6, %r9d
14092a255: 41 83 e0 3f                 	andl	$0x3f, %r8d
14092a259: 45 09 c8                    	orl	%r9d, %r8d
14092a25c: eb 11                       	jmp	0x14092a26f <.text+0x92926f>
14092a25e: 48 83 c0 fe                 	addq	$-0x2, %rax
14092a262: 83 e2 1f                    	andl	$0x1f, %edx
14092a265: eb 12                       	jmp	0x14092a279 <.text+0x929279>
14092a267: 48 83 c0 fd                 	addq	$-0x3, %rax
14092a26b: 41 83 e0 0f                 	andl	$0xf, %r8d
14092a26f: 41 c1 e0 06                 	shll	$0x6, %r8d
14092a273: 83 e2 3f                    	andl	$0x3f, %edx
14092a276: 44 09 c2                    	orl	%r8d, %edx
14092a279: c1 e2 06                    	shll	$0x6, %edx
14092a27c: 80 e1 3f                    	andb	$0x3f, %cl
14092a27f: 0f b6 c9                    	movzbl	%cl, %ecx
14092a282: 09 d1                       	orl	%edx, %ecx
14092a284: 83 f9 5c                    	cmpl	$0x5c, %ecx
14092a287: 74 16                       	je	0x14092a29f <.text+0x92929f>
14092a289: 83 f9 2f                    	cmpl	$0x2f, %ecx
14092a28c: 74 11                       	je	0x14092a29f <.text+0x92929f>
14092a28e: 4c 39 e0                    	cmpq	%r12, %rax
14092a291: 0f 85 79 ff ff ff           	jne	0x14092a210 <.text+0x929210>
14092a297: 41 bd 35 00 00 00           	movl	$0x35, %r13d
14092a29d: eb 50                       	jmp	0x14092a2ef <.text+0x9292ef>
14092a29f: 4c 8d 25 fa 52 18 02        	leaq	0x21852fa(%rip), %r12   # 0x142aaf5a0
14092a2a6: 4c 29 e0                    	subq	%r12, %rax
14092a2a9: 49 89 c0                    	movq	%rax, %r8
14092a2ac: 49 ff c0                    	incq	%r8
14092a2af: 74 32                       	je	0x14092a2e3 <.text+0x9292e3>
14092a2b1: 49 83 f8 35                 	cmpq	$0x35, %r8
14092a2b5: 73 2a                       	jae	0x14092a2e1 <.text+0x9292e1>
14092a2b7: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
14092a2bc: 7f 25                       	jg	0x14092a2e3 <.text+0x9292e3>
14092a2be: 48 8d 05 53 44 18 02        	leaq	0x2184453(%rip), %rax   # 0x142aae718
14092a2c5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14092a2ca: 48 8d 0d cf 52 18 02        	leaq	0x21852cf(%rip), %rcx   # 0x142aaf5a0
14092a2d1: ba 35 00 00 00              	movl	$0x35, %edx
14092a2d6: 41 b9 35 00 00 00           	movl	$0x35, %r9d
14092a2dc: e8 5f f6 fa 01              	callq	0x1428d9940 <.text+0x28d8940>
14092a2e1: 75 db                       	jne	0x14092a2be <.text+0x9292be>
14092a2e3: 41 bd 34 00 00 00           	movl	$0x34, %r13d
14092a2e9: 49 29 c5                    	subq	%rax, %r13
14092a2ec: 4d 01 c4                    	addq	%r8, %r12
14092a2ef: e8 3c de 2e 00              	callq	0x140c18130 <.text+0xc17130>
14092a2f4: 48 89 c1                    	movq	%rax, %rcx
14092a2f7: 48 8b 00                    	movq	(%rax), %rax
14092a2fa: 48 83 f8 01                 	cmpq	$0x1, %rax
14092a2fe: 0f 85 2c 02 00 00           	jne	0x14092a530 <.text+0x929530>
14092a304: 48 83 c1 08                 	addq	$0x8, %rcx
14092a308: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14092a30c: 0f 85 58 02 00 00           	jne	0x14092a56a <.text+0x92956a>
14092a312: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14092a319: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
14092a31d: 48 83 c1 08                 	addq	$0x8, %rcx
14092a321: 4c 89 6c 24 20              	movq	%r13, 0x20(%rsp)
14092a326: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14092a32f: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14092a338: 4c 89 fa                    	movq	%r15, %rdx
14092a33b: 4d 89 f0                    	movq	%r14, %r8
14092a33e: 4d 89 e1                    	movq	%r12, %r9
14092a341: e8 fa c9 2e 00              	callq	0x140c16d40 <.text+0xc15d40>
14092a346: 49 89 c6                    	movq	%rax, %r14
14092a349: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14092a34d: 48 ff 00                    	incq	(%rax)
14092a350: 48 8b 07                    	movq	(%rdi), %rax
14092a353: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
14092a357: 8b 57 14                    	movl	0x14(%rdi), %edx
14092a35a: 4c 8b 80 f8 00 00 00        	movq	0xf8(%rax), %r8
14092a361: 4c 8b 88 00 01 00 00        	movq	0x100(%rax), %r9
14092a368: 4f 8d 0c 88                 	leaq	(%r8,%r9,4), %r9
14092a36c: 4c 8b 16                    	movq	(%rsi), %r10
14092a36f: 80 b8 08 01 00 00 00        	cmpb	$0x0, 0x108(%rax)
14092a376: 74 0a                       	je	0x14092a382 <.text+0x929382>
14092a378: 31 f6                       	xorl	%esi, %esi
14092a37a: 45 31 db                    	xorl	%r11d, %r11d
14092a37d: 45 31 ff                    	xorl	%r15d, %r15d
14092a380: eb 6a                       	jmp	0x14092a3ec <.text+0x9293ec>
14092a382: bf 08 00 00 00              	movl	$0x8, %edi
14092a387: 31 f6                       	xorl	%esi, %esi
14092a389: 45 31 db                    	xorl	%r11d, %r11d
14092a38c: 45 31 e4                    	xorl	%r12d, %r12d
14092a38f: e9 17 01 00 00              	jmp	0x14092a4ab <.text+0x9294ab>
14092a394: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14092a3a0: 48 8d 34 fe                 	leaq	(%rsi,%rdi,8), %rsi
14092a3a4: 48 8b b8 10 01 00 00        	movq	0x110(%rax), %rdi
14092a3ab: 4c 8b 7e 18                 	movq	0x18(%rsi), %r15
14092a3af: 48 8b 76 38                 	movq	0x38(%rsi), %rsi
14092a3b3: 48 8b 34 fe                 	movq	(%rsi,%rdi,8), %rsi
14092a3b7: 48 f7 d6                    	notq	%rsi
14092a3ba: 48 8d 3c 76                 	leaq	(%rsi,%rsi,2), %rdi
14092a3be: 48 c1 e7 04                 	shlq	$0x4, %rdi
14092a3c2: 49 8b 74 3f 10              	movq	0x10(%r15,%rdi), %rsi
14092a3c7: 49 8b 7c 3f 28              	movq	0x28(%r15,%rdi), %rdi
14092a3cc: 49 83 c0 04                 	addq	$0x4, %r8
14092a3d0: 45 31 ff                    	xorl	%r15d, %r15d
14092a3d3: 45 89 fc                    	movl	%r15d, %r12d
14092a3d6: 49 ff c7                    	incq	%r15
14092a3d9: f2 41 0f 10 02              	movsd	(%r10), %xmm0
14092a3de: 42 89 14 a7                 	movl	%edx, (%rdi,%r12,4)
14092a3e2: 49 c1 e4 04                 	shlq	$0x4, %r12
14092a3e6: f2 42 0f 11 04 26           	movsd	%xmm0, (%rsi,%r12)
14092a3ec: 4d 39 df                    	cmpq	%r11, %r15
14092a3ef: 75 e2                       	jne	0x14092a3d3 <.text+0x9293d3>
14092a3f1: 4d 39 c8                    	cmpq	%r9, %r8
14092a3f4: 0f 84 e8 00 00 00           	je	0x14092a4e2 <.text+0x9294e2>
14092a3fa: 48 8b b1 a8 01 00 00        	movq	0x1a8(%rcx), %rsi
14092a401: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14092a410: 45 8b 18                    	movl	(%r8), %r11d
14092a413: 4b 8d 3c db                 	leaq	(%r11,%r11,8), %rdi
14092a417: 4c 8b 5c fe 10              	movq	0x10(%rsi,%rdi,8), %r11
14092a41c: 4d 85 db                    	testq	%r11, %r11
14092a41f: 0f 85 7b ff ff ff           	jne	0x14092a3a0 <.text+0x9293a0>
14092a425: 49 83 c0 04                 	addq	$0x4, %r8
14092a429: 4d 39 c8                    	cmpq	%r9, %r8
14092a42c: 75 e2                       	jne	0x14092a410 <.text+0x929410>
14092a42e: e9 af 00 00 00              	jmp	0x14092a4e2 <.text+0x9294e2>
14092a433: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14092a440: 48 01 f7                    	addq	%rsi, %rdi
14092a443: 8b b7 94 00 00 00           	movl	0x94(%rdi), %esi
14092a449: 4c 8b b9 a8 01 00 00        	movq	0x1a8(%rcx), %r15
14092a450: 48 8d 34 f6                 	leaq	(%rsi,%rsi,8), %rsi
14092a454: 4c 8b a0 10 01 00 00        	movq	0x110(%rax), %r12
14092a45b: 4d 8b 6c f7 18              	movq	0x18(%r15,%rsi,8), %r13
14092a460: 49 8b 74 f7 38              	movq	0x38(%r15,%rsi,8), %rsi
14092a465: 4a 8b 34 e6                 	movq	(%rsi,%r12,8), %rsi
14092a469: 48 f7 d6                    	notq	%rsi
14092a46c: 4c 8d 3c 76                 	leaq	(%rsi,%rsi,2), %r15
14092a470: 49 c1 e7 04                 	shlq	$0x4, %r15
14092a474: 4b 8b 74 3d 10              	movq	0x10(%r13,%r15), %rsi
14092a479: 4f 8b 7c 3d 28              	movq	0x28(%r13,%r15), %r15
14092a47e: 48 8b 7f 50                 	movq	0x50(%rdi), %rdi
14092a482: 49 83 c0 04                 	addq	$0x4, %r8
14092a486: 45 31 e4                    	xorl	%r12d, %r12d
14092a489: 4d 89 e5                    	movq	%r12, %r13
14092a48c: 49 c1 e5 04                 	shlq	$0x4, %r13
14092a490: 46 8b 6c 2f 08              	movl	0x8(%rdi,%r13), %r13d
14092a495: 49 ff c4                    	incq	%r12
14092a498: f2 41 0f 10 02              	movsd	(%r10), %xmm0
14092a49d: 43 89 14 af                 	movl	%edx, (%r15,%r13,4)
14092a4a1: 49 c1 e5 04                 	shlq	$0x4, %r13
14092a4a5: f2 42 0f 11 04 2e           	movsd	%xmm0, (%rsi,%r13)
14092a4ab: 4d 39 dc                    	cmpq	%r11, %r12
14092a4ae: 75 d9                       	jne	0x14092a489 <.text+0x929489>
14092a4b0: 4d 39 c8                    	cmpq	%r9, %r8
14092a4b3: 74 2d                       	je	0x14092a4e2 <.text+0x9294e2>
14092a4b5: 48 8b b9 00 01 00 00        	movq	0x100(%rcx), %rdi
14092a4bc: 0f 1f 40 00                 	nopl	(%rax)
14092a4c0: 45 8b 18                    	movl	(%r8), %r11d
14092a4c3: 4b 8d 34 9b                 	leaq	(%r11,%r11,4), %rsi
14092a4c7: 48 c1 e6 05                 	shlq	$0x5, %rsi
14092a4cb: 4c 8b 5c 37 58              	movq	0x58(%rdi,%rsi), %r11
14092a4d0: 4d 85 db                    	testq	%r11, %r11
14092a4d3: 0f 85 67 ff ff ff           	jne	0x14092a440 <.text+0x929440>
14092a4d9: 49 83 c0 04                 	addq	$0x4, %r8
14092a4dd: 4d 39 c8                    	cmpq	%r9, %r8
14092a4e0: 75 de                       	jne	0x14092a4c0 <.text+0x9294c0>
14092a4e2: 84 db                       	testb	%bl, %bl
14092a4e4: 74 39                       	je	0x14092a51f <.text+0x92951f>
14092a4e6: e8 45 dc 2e 00              	callq	0x140c18130 <.text+0xc17130>
14092a4eb: 48 89 c1                    	movq	%rax, %rcx
14092a4ee: 48 8b 00                    	movq	(%rax), %rax
14092a4f1: 48 83 f8 01                 	cmpq	$0x1, %rax
14092a4f5: 75 53                       	jne	0x14092a54a <.text+0x92954a>
14092a4f7: 48 83 c1 08                 	addq	$0x8, %rcx
14092a4fb: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14092a4ff: 75 69                       	jne	0x14092a56a <.text+0x92956a>
14092a501: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14092a508: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
14092a50c: 48 83 c1 08                 	addq	$0x8, %rcx
14092a510: 4c 89 f2                    	movq	%r14, %rdx
14092a513: e8 d8 cb 2e 00              	callq	0x140c170f0 <.text+0xc160f0>
14092a518: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14092a51c: 48 ff 00                    	incq	(%rax)
14092a51f: 48 83 c4 48                 	addq	$0x48, %rsp
14092a523: 5b                          	popq	%rbx
14092a524: 5f                          	popq	%rdi
14092a525: 5e                          	popq	%rsi
14092a526: 41 5c                       	popq	%r12
14092a528: 41 5d                       	popq	%r13
14092a52a: 41 5e                       	popq	%r14
14092a52c: 41 5f                       	popq	%r15
14092a52e: 5d                          	popq	%rbp
14092a52f: c3                          	retq
14092a530: 48 85 c0                    	testq	%rax, %rax
14092a533: 75 29                       	jne	0x14092a55e <.text+0x92955e>
14092a535: 31 d2                       	xorl	%edx, %edx
14092a537: e8 94 e2 fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
14092a53c: 48 89 c1                    	movq	%rax, %rcx
14092a53f: 48 85 c0                    	testq	%rax, %rax
14092a542: 0f 85 c0 fd ff ff           	jne	0x14092a308 <.text+0x929308>
14092a548: eb 14                       	jmp	0x14092a55e <.text+0x92955e>
14092a54a: 48 85 c0                    	testq	%rax, %rax
14092a54d: 75 0f                       	jne	0x14092a55e <.text+0x92955e>
14092a54f: 31 d2                       	xorl	%edx, %edx
14092a551: e8 7a e2 fc 01              	callq	0x1428f87d0 <.text+0x28f77d0>
14092a556: 48 89 c1                    	movq	%rax, %rcx
14092a559: 48 85 c0                    	testq	%rax, %rax
14092a55c: 75 9d                       	jne	0x14092a4fb <.text+0x9294fb>
14092a55e: 48 8d 0d 3b 05 31 02        	leaq	0x231053b(%rip), %rcx   # 0x142c3aaa0
14092a565: e8 66 db fa 01              	callq	0x1428d80d0 <.text+0x28d70d0>
14092a56a: 48 8d 0d ff 05 31 02        	leaq	0x23105ff(%rip), %rcx   # 0x142c3ab70
14092a571: e8 da ec fa 01              	callq	0x1428d9250 <.text+0x28d8250>
14092a576: cc                          	int3
14092a577: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14092a580: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a585: 55                          	pushq	%rbp
14092a586: 41 57                       	pushq	%r15
14092a588: 41 56                       	pushq	%r14
14092a58a: 41 55                       	pushq	%r13
14092a58c: 41 54                       	pushq	%r12
14092a58e: 56                          	pushq	%rsi
14092a58f: 57                          	pushq	%rdi
14092a590: 53                          	pushq	%rbx
14092a591: 48 83 ec 38                 	subq	$0x38, %rsp
14092a595: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
14092a599: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14092a59d: 48 ff 00                    	incq	(%rax)
14092a5a0: 48 83 c4 38                 	addq	$0x38, %rsp
14092a5a4: 5b                          	popq	%rbx
14092a5a5: 5f                          	popq	%rdi
14092a5a6: 5e                          	popq	%rsi
14092a5a7: 41 5c                       	popq	%r12
14092a5a9: 41 5d                       	popq	%r13
14092a5ab: 41 5e                       	popq	%r14
14092a5ad: 41 5f                       	popq	%r15
14092a5af: 5d                          	popq	%rbp
14092a5b0: c3                          	retq
14092a5b1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14092a5c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14092a5c5: 55                          	pushq	%rbp
14092a5c6: 41 57                       	pushq	%r15
14092a5c8: 41 56                       	pushq	%r14
14092a5ca: 41 55                       	pushq	%r13
14092a5cc: 41 54                       	pushq	%r12
14092a5ce: 56                          	pushq	%rsi
14092a5cf: 57                          	pushq	%rdi
14092a5d0: 53                          	pushq	%rbx
14092a5d1: 48 83 ec 38                 	subq	$0x38, %rsp
14092a5d5: 48 8d 6a 40                 	leaq	0x40(%rdx), %rbp
14092a5d9: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14092a5dd: 48 ff 00                    	incq	(%rax)
14092a5e0: 48 83 c4 38                 	addq	$0x38, %rsp
14092a5e4: 5b                          	popq	%rbx
14092a5e5: 5f                          	popq	%rdi
14092a5e6: 5e                          	popq	%rsi
14092a5e7: 41 5c                       	popq	%r12
14092a5e9: 41 5d                       	popq	%r13
14092a5eb: 41 5e                       	popq	%r14
14092a5ed: 41 5f                       	popq	%r15
14092a5ef: 5d                          	popq	%rbp
14092a5f0: c3                          	retq
