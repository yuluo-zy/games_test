
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14165d460: 55                          	pushq	%rbp
14165d461: 41 57                       	pushq	%r15
14165d463: 41 56                       	pushq	%r14
14165d465: 41 55                       	pushq	%r13
14165d467: 41 54                       	pushq	%r12
14165d469: 56                          	pushq	%rsi
14165d46a: 57                          	pushq	%rdi
14165d46b: 53                          	pushq	%rbx
14165d46c: 48 81 ec a8 04 00 00        	subq	$0x4a8, %rsp            # imm = 0x4A8
14165d473: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14165d47b: 44 0f 29 9d 10 04 00 00     	movaps	%xmm11, 0x410(%rbp)
14165d483: 44 0f 29 95 00 04 00 00     	movaps	%xmm10, 0x400(%rbp)
14165d48b: 44 0f 29 8d f0 03 00 00     	movaps	%xmm9, 0x3f0(%rbp)
14165d493: 44 0f 29 85 e0 03 00 00     	movaps	%xmm8, 0x3e0(%rbp)
14165d49b: 0f 29 bd d0 03 00 00        	movaps	%xmm7, 0x3d0(%rbp)
14165d4a2: 66 0f 7f b5 c0 03 00 00     	movdqa	%xmm6, 0x3c0(%rbp)
14165d4aa: 48 c7 85 b8 03 00 00 fe ff ff ff    	movq	$-0x2, 0x3b8(%rbp)
14165d4b5: 4c 89 4d 58                 	movq	%r9, 0x58(%rbp)
14165d4b9: 48 89 95 f8 00 00 00        	movq	%rdx, 0xf8(%rbp)
14165d4c0: 48 89 ce                    	movq	%rcx, %rsi
14165d4c3: 48 8b 85 a0 04 00 00        	movq	0x4a0(%rbp), %rax
14165d4ca: 48 8b 8d 98 04 00 00        	movq	0x498(%rbp), %rcx
14165d4d1: 4d 8b 28                    	movq	(%r8), %r13
14165d4d4: 49 8b 58 20                 	movq	0x20(%r8), %rbx
14165d4d8: 49 8b 50 40                 	movq	0x40(%r8), %rdx
14165d4dc: 48 89 55 50                 	movq	%rdx, 0x50(%rbp)
14165d4e0: 49 8b 50 48                 	movq	0x48(%r8), %rdx
14165d4e4: 48 89 95 90 03 00 00        	movq	%rdx, 0x390(%rbp)
14165d4eb: 48 8b 11                    	movq	(%rcx), %rdx
14165d4ee: 48 89 95 a0 01 00 00        	movq	%rdx, 0x1a0(%rbp)
14165d4f5: 48 8b 49 20                 	movq	0x20(%rcx), %rcx
14165d4f9: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
14165d500: 48 8b 08                    	movq	(%rax), %rcx
14165d503: 48 89 8d 38 03 00 00        	movq	%rcx, 0x338(%rbp)
14165d50a: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14165d50e: 48 89 8d a0 03 00 00        	movq	%rcx, 0x3a0(%rbp)
14165d515: 48 8b 78 10                 	movq	0x10(%rax), %rdi
14165d519: 4c 8b 60 20                 	movq	0x20(%rax), %r12
14165d51d: 44 8b 78 2c                 	movl	0x2c(%rax), %r15d
14165d521: 48 8b 85 a8 04 00 00        	movq	0x4a8(%rbp), %rax
14165d528: 48 8b 08                    	movq	(%rax), %rcx
14165d52b: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
14165d532: 48 8b 48 20                 	movq	0x20(%rax), %rcx
14165d536: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14165d53d: 48 8b 40 40                 	movq	0x40(%rax), %rax
14165d541: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14165d548: 48 8b 85 b0 04 00 00        	movq	0x4b0(%rbp), %rax
14165d54f: 48 8b 08                    	movq	(%rax), %rcx
14165d552: 48 89 8d 20 03 00 00        	movq	%rcx, 0x320(%rbp)
14165d559: 48 8b 48 10                 	movq	0x10(%rax), %rcx
14165d55d: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
14165d564: 8b 48 1c                    	movl	0x1c(%rax), %ecx
14165d567: 89 8d 44 03 00 00           	movl	%ecx, 0x344(%rbp)
14165d56d: 48 8b 48 20                 	movq	0x20(%rax), %rcx
14165d571: 48 89 8d a8 01 00 00        	movq	%rcx, 0x1a8(%rbp)
14165d578: 48 8b 48 30                 	movq	0x30(%rax), %rcx
14165d57c: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
14165d583: 8b 40 3c                    	movl	0x3c(%rax), %eax
14165d586: 89 85 94 01 00 00           	movl	%eax, 0x194(%rbp)
14165d58c: 48 8b 85 b8 04 00 00        	movq	0x4b8(%rbp), %rax
14165d593: 48 8b 08                    	movq	(%rax), %rcx
14165d596: 48 89 8d f0 00 00 00        	movq	%rcx, 0xf0(%rbp)
14165d59d: 48 8b 40 20                 	movq	0x20(%rax), %rax
14165d5a1: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14165d5a5: 4c 8b b5 c8 04 00 00        	movq	0x4c8(%rbp), %r14
14165d5ac: f3 41 0f 6f 46 40           	movdqu	0x40(%r14), %xmm0
14165d5b2: f3 41 0f 6f 4e 50           	movdqu	0x50(%r14), %xmm1
14165d5b8: 66 0f 7f 4d d0              	movdqa	%xmm1, -0x30(%rbp)
14165d5bd: 66 0f 7f 45 c0              	movdqa	%xmm0, -0x40(%rbp)
14165d5c2: 49 8b 46 20                 	movq	0x20(%r14), %rax
14165d5c6: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
14165d5cd: 49 8b 46 30                 	movq	0x30(%r14), %rax
14165d5d1: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
14165d5d8: 41 8b 46 3c                 	movl	0x3c(%r14), %eax
14165d5dc: 89 85 44 01 00 00           	movl	%eax, 0x144(%rbp)
14165d5e2: 49 8b 46 60                 	movq	0x60(%r14), %rax
14165d5e6: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14165d5ed: 49 8b 46 70                 	movq	0x70(%r14), %rax
14165d5f1: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14165d5f8: 41 8b 46 7c                 	movl	0x7c(%r14), %eax
14165d5fc: 89 85 4c 01 00 00           	movl	%eax, 0x14c(%rbp)
14165d602: 49 8b 86 80 00 00 00        	movq	0x80(%r14), %rax
14165d609: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
14165d610: 49 8b 86 90 00 00 00        	movq	0x90(%r14), %rax
14165d617: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
14165d61e: 41 8b 86 9c 00 00 00        	movl	0x9c(%r14), %eax
14165d625: 89 85 48 01 00 00           	movl	%eax, 0x148(%rbp)
14165d62b: 49 8b 86 a0 00 00 00        	movq	0xa0(%r14), %rax
14165d632: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14165d639: 49 8b 86 b0 00 00 00        	movq	0xb0(%r14), %rax
14165d640: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14165d644: 41 8b 86 bc 00 00 00        	movl	0xbc(%r14), %eax
14165d64b: 89 85 3c 01 00 00           	movl	%eax, 0x13c(%rbp)
14165d651: 49 8b 86 c0 00 00 00        	movq	0xc0(%r14), %rax
14165d658: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
14165d65f: 49 8b 86 d0 00 00 00        	movq	0xd0(%r14), %rax
14165d666: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14165d66a: 41 8b 86 dc 00 00 00        	movl	0xdc(%r14), %eax
14165d671: 89 85 40 01 00 00           	movl	%eax, 0x140(%rbp)
14165d677: 49 8b 86 e0 00 00 00        	movq	0xe0(%r14), %rax
14165d67e: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
14165d685: 49 8b 86 f0 00 00 00        	movq	0xf0(%r14), %rax
14165d68c: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
14165d693: 41 8b 86 fc 00 00 00        	movl	0xfc(%r14), %eax
14165d69a: 89 85 90 01 00 00           	movl	%eax, 0x190(%rbp)
14165d6a0: 49 8b 86 00 01 00 00        	movq	0x100(%r14), %rax
14165d6a7: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
14165d6ae: 49 8b 86 10 01 00 00        	movq	0x110(%r14), %rax
14165d6b5: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14165d6b9: 41 8b 86 1c 01 00 00        	movl	0x11c(%r14), %eax
14165d6c0: 89 85 38 01 00 00           	movl	%eax, 0x138(%rbp)
14165d6c6: e8 35 93 5b ff              	callq	0x140c16a00 <.text+0xc15a00>
14165d6cb: 88 85 98 01 00 00           	movb	%al, 0x198(%rbp)
14165d6d1: 84 c0                       	testb	%al, %al
14165d6d3: 48 89 bd 30 03 00 00        	movq	%rdi, 0x330(%rbp)
14165d6da: 4c 89 a5 08 03 00 00        	movq	%r12, 0x308(%rbp)
14165d6e1: 44 89 bd 10 03 00 00        	movl	%r15d, 0x310(%rbp)
14165d6e8: 0f 84 3a 02 00 00           	je	0x14165d928 <.text+0x165c928>
14165d6ee: 48 89 9d a8 03 00 00        	movq	%rbx, 0x3a8(%rbp)
14165d6f5: bb 4b 00 00 00              	movl	$0x4b, %ebx
14165d6fa: b9 4b 00 00 00              	movl	$0x4b, %ecx
14165d6ff: e8 8c dc ff ff              	callq	0x14165b390 <.text+0x165a390>
14165d704: 4c 8d 3d 65 48 62 01        	leaq	0x1624865(%rip), %r15   # 0x142c81f70
14165d70b: 48 83 f8 01                 	cmpq	$0x1, %rax
14165d70f: 0f 85 a0 00 00 00           	jne	0x14165d7b5 <.text+0x165c7b5>
14165d715: 49 89 d1                    	movq	%rdx, %r9
14165d718: 48 85 d2                    	testq	%rdx, %rdx
14165d71b: 74 36                       	je	0x14165d753 <.text+0x165c753>
14165d71d: 49 83 f9 4b                 	cmpq	$0x4b, %r9
14165d721: 73 2e                       	jae	0x14165d751 <.text+0x165c751>
14165d723: 48 8d 05 46 48 62 01        	leaq	0x1624846(%rip), %rax   # 0x142c81f70
14165d72a: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14165d72f: 7f 22                       	jg	0x14165d753 <.text+0x165c753>
14165d731: 48 8d 05 00 49 62 01        	leaq	0x1624900(%rip), %rax   # 0x142c82038
14165d738: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14165d73d: 48 8d 0d 2c 48 62 01        	leaq	0x162482c(%rip), %rcx   # 0x142c81f70
14165d744: ba 4b 00 00 00              	movl	$0x4b, %edx
14165d749: 45 31 c0                    	xorl	%r8d, %r8d
14165d74c: e8 ef c1 27 01              	callq	0x1428d9940 <.text+0x28d8940>
14165d751: 75 de                       	jne	0x14165d731 <.text+0x165c731>
14165d753: 4c 89 c9                    	movq	%r9, %rcx
14165d756: e8 35 dc ff ff              	callq	0x14165b390 <.text+0x165a390>
14165d75b: 48 83 f8 01                 	cmpq	$0x1, %rax
14165d75f: 75 54                       	jne	0x14165d7b5 <.text+0x165c7b5>
14165d761: 49 89 d0                    	movq	%rdx, %r8
14165d764: 49 83 c0 02                 	addq	$0x2, %r8
14165d768: 74 39                       	je	0x14165d7a3 <.text+0x165c7a3>
14165d76a: 49 83 f8 4b                 	cmpq	$0x4b, %r8
14165d76e: 73 31                       	jae	0x14165d7a1 <.text+0x165c7a1>
14165d770: 48 8d 05 f9 47 62 01        	leaq	0x16247f9(%rip), %rax   # 0x142c81f70
14165d777: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14165d77c: 7f 25                       	jg	0x14165d7a3 <.text+0x165c7a3>
14165d77e: 48 8d 05 cb 48 62 01        	leaq	0x16248cb(%rip), %rax   # 0x142c82050
14165d785: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14165d78a: 48 8d 0d df 47 62 01        	leaq	0x16247df(%rip), %rcx   # 0x142c81f70
14165d791: ba 4b 00 00 00              	movl	$0x4b, %edx
14165d796: 41 b9 4b 00 00 00           	movl	$0x4b, %r9d
14165d79c: e8 9f c1 27 01              	callq	0x1428d9940 <.text+0x28d8940>
14165d7a1: 75 db                       	jne	0x14165d77e <.text+0x165c77e>
14165d7a3: bb 49 00 00 00              	movl	$0x49, %ebx
14165d7a8: 48 29 d3                    	subq	%rdx, %rbx
14165d7ab: 4c 8d 3d be 47 62 01        	leaq	0x16247be(%rip), %r15   # 0x142c81f70
14165d7b2: 4d 01 c7                    	addq	%r8, %r15
14165d7b5: 4c 8d 05 46 4b 62 01        	leaq	0x1624b46(%rip), %r8    # 0x142c82302
14165d7bc: 4c 8d 25 fb 4a 62 01        	leaq	0x1624afb(%rip), %r12   # 0x142c822be
14165d7c3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14165d7d0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14165d7d5: 85 c0                       	testl	%eax, %eax
14165d7d7: 78 17                       	js	0x14165d7f0 <.text+0x165c7f0>
14165d7d9: 49 ff c8                    	decq	%r8
14165d7dc: 83 f8 5c                    	cmpl	$0x5c, %eax
14165d7df: 75 63                       	jne	0x14165d844 <.text+0x165c844>
14165d7e1: eb 72                       	jmp	0x14165d855 <.text+0x165c855>
14165d7e3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14165d7f0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14165d7f5: 80 f9 c0                    	cmpb	$-0x40, %cl
14165d7f8: 7d 23                       	jge	0x14165d81d <.text+0x165c81d>
14165d7fa: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14165d7ff: 80 fa c0                    	cmpb	$-0x40, %dl
14165d802: 7d 22                       	jge	0x14165d826 <.text+0x165c826>
14165d804: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14165d809: 49 83 c0 fc                 	addq	$-0x4, %r8
14165d80d: 41 83 e1 07                 	andl	$0x7, %r9d
14165d811: 41 c1 e1 06                 	shll	$0x6, %r9d
14165d815: 83 e2 3f                    	andl	$0x3f, %edx
14165d818: 44 09 ca                    	orl	%r9d, %edx
14165d81b: eb 10                       	jmp	0x14165d82d <.text+0x165c82d>
14165d81d: 49 83 c0 fe                 	addq	$-0x2, %r8
14165d821: 83 e1 1f                    	andl	$0x1f, %ecx
14165d824: eb 0f                       	jmp	0x14165d835 <.text+0x165c835>
14165d826: 49 83 c0 fd                 	addq	$-0x3, %r8
14165d82a: 83 e2 0f                    	andl	$0xf, %edx
14165d82d: c1 e2 06                    	shll	$0x6, %edx
14165d830: 83 e1 3f                    	andl	$0x3f, %ecx
14165d833: 09 d1                       	orl	%edx, %ecx
14165d835: c1 e1 06                    	shll	$0x6, %ecx
14165d838: 24 3f                       	andb	$0x3f, %al
14165d83a: 0f b6 c0                    	movzbl	%al, %eax
14165d83d: 09 c8                       	orl	%ecx, %eax
14165d83f: 83 f8 5c                    	cmpl	$0x5c, %eax
14165d842: 74 11                       	je	0x14165d855 <.text+0x165c855>
14165d844: 83 f8 2f                    	cmpl	$0x2f, %eax
14165d847: 74 0c                       	je	0x14165d855 <.text+0x165c855>
14165d849: 4d 39 e0                    	cmpq	%r12, %r8
14165d84c: 75 82                       	jne	0x14165d7d0 <.text+0x165c7d0>
14165d84e: bf 44 00 00 00              	movl	$0x44, %edi
14165d853: eb 4c                       	jmp	0x14165d8a1 <.text+0x165c8a1>
14165d855: 4c 8d 25 62 4a 62 01        	leaq	0x1624a62(%rip), %r12   # 0x142c822be
14165d85c: 4d 29 e0                    	subq	%r12, %r8
14165d85f: 49 ff c0                    	incq	%r8
14165d862: 74 32                       	je	0x14165d896 <.text+0x165c896>
14165d864: 49 83 f8 44                 	cmpq	$0x44, %r8
14165d868: 73 2a                       	jae	0x14165d894 <.text+0x165c894>
14165d86a: 43 80 3c 20 bf              	cmpb	$-0x41, (%r8,%r12)
14165d86f: 7f 25                       	jg	0x14165d896 <.text+0x165c896>
14165d871: 48 8d 05 a0 47 62 01        	leaq	0x16247a0(%rip), %rax   # 0x142c82018
14165d878: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14165d87d: 48 8d 0d 3a 4a 62 01        	leaq	0x1624a3a(%rip), %rcx   # 0x142c822be
14165d884: ba 44 00 00 00              	movl	$0x44, %edx
14165d889: 41 b9 44 00 00 00           	movl	$0x44, %r9d
14165d88f: e8 ac c0 27 01              	callq	0x1428d9940 <.text+0x28d8940>
14165d894: 75 db                       	jne	0x14165d871 <.text+0x165c871>
14165d896: bf 44 00 00 00              	movl	$0x44, %edi
14165d89b: 4c 29 c7                    	subq	%r8, %rdi
14165d89e: 4d 01 c4                    	addq	%r8, %r12
14165d8a1: e8 8a a8 5b ff              	callq	0x140c18130 <.text+0xc17130>
14165d8a6: 48 89 c1                    	movq	%rax, %rcx
14165d8a9: 48 8b 00                    	movq	(%rax), %rax
14165d8ac: 48 83 f8 01                 	cmpq	$0x1, %rax
14165d8b0: 0f 85 c2 12 00 00           	jne	0x14165eb78 <.text+0x165db78>
14165d8b6: 48 83 c1 08                 	addq	$0x8, %rcx
14165d8ba: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14165d8be: 0f 85 e3 22 00 00           	jne	0x14165fba7 <.text+0x165eba7>
14165d8c4: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14165d8cb: 48 89 8d 88 03 00 00        	movq	%rcx, 0x388(%rbp)
14165d8d2: 48 83 c1 08                 	addq	$0x8, %rcx
14165d8d6: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14165d8db: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14165d8e4: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14165d8ed: 4c 89 fa                    	movq	%r15, %rdx
14165d8f0: 49 89 d8                    	movq	%rbx, %r8
14165d8f3: 4d 89 e1                    	movq	%r12, %r9
14165d8f6: e8 45 94 5b ff              	callq	0x140c16d40 <.text+0xc15d40>
14165d8fb: 48 89 85 88 01 00 00        	movq	%rax, 0x188(%rbp)
14165d902: 48 8b 85 88 03 00 00        	movq	0x388(%rbp), %rax
14165d909: 48 ff 00                    	incq	(%rax)
14165d90c: 48 8b bd 30 03 00 00        	movq	0x330(%rbp), %rdi
14165d913: 4c 8b a5 08 03 00 00        	movq	0x308(%rbp), %r12
14165d91a: 44 8b bd 10 03 00 00        	movl	0x310(%rbp), %r15d
14165d921: 48 8b 9d a8 03 00 00        	movq	0x3a8(%rbp), %rbx
14165d928: 48 8b 8d c0 04 00 00        	movq	0x4c0(%rbp), %rcx
14165d92f: 48 8b 01                    	movq	(%rcx), %rax
14165d932: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
14165d936: 4c 8b 00                    	movq	(%rax), %r8
14165d939: 31 d2                       	xorl	%edx, %edx
14165d93b: 4d 89 c1                    	movq	%r8, %r9
14165d93e: 4c 2b 49 18                 	subq	0x18(%rcx), %r9
14165d942: 4c 0f 42 ca                 	cmovbq	%rdx, %r9
14165d946: 4c 2b 41 38                 	subq	0x38(%rcx), %r8
14165d94a: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
14165d94e: 4c 8b 51 30                 	movq	0x30(%rcx), %r10
14165d952: 4d 29 c2                    	subq	%r8, %r10
14165d955: 4c 0f 42 d2                 	cmovbq	%rdx, %r10
14165d959: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
14165d95d: 41 0f 96 c3                 	setbe	%r11b
14165d961: 4d 29 c8                    	subq	%r9, %r8
14165d964: 4c 0f 42 c2                 	cmovbq	%rdx, %r8
14165d968: 41 0f 96 c1                 	setbe	%r9b
14165d96c: 4d 01 d0                    	addq	%r10, %r8
14165d96f: 45 20 d9                    	andb	%r11b, %r9b
14165d972: 44 88 8d 9f 03 00 00        	movb	%r9b, 0x39f(%rbp)
14165d979: 4c 0f 44 c2                 	cmoveq	%rdx, %r8
14165d97d: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14165d981: 4c 29 c1                    	subq	%r8, %rcx
14165d984: 48 89 08                    	movq	%rcx, (%rax)
14165d987: 45 89 3c 24                 	movl	%r15d, (%r12)
14165d98b: c6 47 38 02                 	movb	$0x2, 0x38(%rdi)
14165d98f: 48 8b 36                    	movq	(%rsi), %rsi
14165d992: 8b 06                       	movl	(%rsi), %eax
14165d994: 85 c0                       	testl	%eax, %eax
14165d996: 0f 84 e5 00 00 00           	je	0x14165da81 <.text+0x165ca81>
14165d99c: 41 b0 02                    	movb	$0x2, %r8b
14165d99f: 83 f8 05                    	cmpl	$0x5, %eax
14165d9a2: 0f 87 b0 08 00 00           	ja	0x14165e258 <.text+0x165d258>
14165d9a8: 83 f8 02                    	cmpl	$0x2, %eax
14165d9ab: 74 09                       	je	0x14165d9b6 <.text+0x165c9b6>
14165d9ad: 83 f8 01                    	cmpl	$0x1, %eax
14165d9b0: 0f 85 a2 08 00 00           	jne	0x14165e258 <.text+0x165d258>
14165d9b6: 44 89 85 88 03 00 00        	movl	%r8d, 0x388(%rbp)
14165d9bd: 48 8b 76 08                 	movq	0x8(%rsi), %rsi
14165d9c1: 48 89 b5 b0 02 00 00        	movq	%rsi, 0x2b0(%rbp)
14165d9c8: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
14165d9cf: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
14165d9d4: 0f 84 49 0a 00 00           	je	0x14165e423 <.text+0x165d423>
14165d9da: 49 8d 4d 20                 	leaq	0x20(%r13), %rcx
14165d9de: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165d9e5: e8 86 33 f5 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14165d9ea: 49 8b 5d 00                 	movq	(%r13), %rbx
14165d9ee: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
14165d9f2: 48 89 ca                    	movq	%rcx, %rdx
14165d9f5: 48 21 c2                    	andq	%rax, %rdx
14165d9f8: 48 c1 e8 39                 	shrq	$0x39, %rax
14165d9fc: 66 0f 6e c0                 	movd	%eax, %xmm0
14165da00: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14165da04: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14165da09: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14165da0e: 48 8d 83 58 fd ff ff        	leaq	-0x2a8(%rbx), %rax
14165da15: 45 31 c0                    	xorl	%r8d, %r8d
14165da18: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14165da1c: f3 0f 6f 14 13              	movdqu	(%rbx,%rdx), %xmm2
14165da21: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14165da25: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14165da29: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
14165da2e: 45 85 c9                    	testl	%r9d, %r9d
14165da31: 74 2c                       	je	0x14165da5f <.text+0x165ca5f>
14165da33: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
14165da38: 49 01 d2                    	addq	%rdx, %r10
14165da3b: 49 21 ca                    	andq	%rcx, %r10
14165da3e: 49 f7 da                    	negq	%r10
14165da41: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
14165da48: 4a 39 34 10                 	cmpq	%rsi, (%rax,%r10)
14165da4c: 0f 84 59 05 00 00           	je	0x14165dfab <.text+0x165cfab>
14165da52: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
14165da56: 66 45 21 ca                 	andw	%r9w, %r10w
14165da5a: 45 89 d1                    	movl	%r10d, %r9d
14165da5d: 75 d4                       	jne	0x14165da33 <.text+0x165ca33>
14165da5f: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14165da63: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
14165da68: 45 85 c9                    	testl	%r9d, %r9d
14165da6b: 0f 85 b2 09 00 00           	jne	0x14165e423 <.text+0x165d423>
14165da71: 4c 01 c2                    	addq	%r8, %rdx
14165da74: 48 83 c2 10                 	addq	$0x10, %rdx
14165da78: 49 83 c0 10                 	addq	$0x10, %r8
14165da7c: 48 21 ca                    	andq	%rcx, %rdx
14165da7f: eb 9b                       	jmp	0x14165da1c <.text+0x165ca1c>
14165da81: 48 89 b5 70 01 00 00        	movq	%rsi, 0x170(%rbp)
14165da88: 48 89 d9                    	movq	%rbx, %rcx
14165da8b: 49 8b 5d 00                 	movq	(%r13), %rbx
14165da8f: 49 8b 7d 18                 	movq	0x18(%r13), %rdi
14165da93: 66 0f 6f 03                 	movdqa	(%rbx), %xmm0
14165da97: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14165da9b: f7 d0                       	notl	%eax
14165da9d: 4c 8d 63 10                 	leaq	0x10(%rbx), %r12
14165daa1: 48 89 8d a8 03 00 00        	movq	%rcx, 0x3a8(%rbp)
14165daa8: 48 83 c1 20                 	addq	$0x20, %rcx
14165daac: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
14165dab3: 49 8d 4d 20                 	leaq	0x20(%r13), %rcx
14165dab7: 48 89 8d 88 03 00 00        	movq	%rcx, 0x388(%rbp)
14165dabe: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165dac5: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
14165dac9: 0f 57 ff                    	xorps	%xmm7, %xmm7
14165dacc: f3 44 0f 10 05 6b 82 44 01  	movss	0x144826b(%rip), %xmm8  # 0x142aa5d40
14165dad5: 48 85 ff                    	testq	%rdi, %rdi
14165dad8: 0f 84 bb 10 00 00           	je	0x14165eb99 <.text+0x165db99>
14165dade: 66 85 c0                    	testw	%ax, %ax
14165dae1: 74 0d                       	je	0x14165daf0 <.text+0x165caf0>
14165dae3: 44 8d 78 ff                 	leal	-0x1(%rax), %r15d
14165dae7: 41 21 c7                    	andl	%eax, %r15d
14165daea: eb 2f                       	jmp	0x14165db1b <.text+0x165cb1b>
14165daec: 0f 1f 40 00                 	nopl	(%rax)
14165daf0: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
14165daf6: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14165dafa: 48 81 c3 80 d5 ff ff        	addq	$-0x2a80, %rbx          # imm = 0xD580
14165db01: 49 83 c4 10                 	addq	$0x10, %r12
14165db05: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14165db0a: 74 e4                       	je	0x14165daf0 <.text+0x165caf0>
14165db0c: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14165db11: 29 c1                       	subl	%eax, %ecx
14165db13: f7 d0                       	notl	%eax
14165db15: 41 89 c7                    	movl	%eax, %r15d
14165db18: 41 21 cf                    	andl	%ecx, %r15d
14165db1b: f3 0f bc c0                 	tzcntl	%eax, %eax
14165db1f: 48 f7 d8                    	negq	%rax
14165db22: 48 69 c0 a8 02 00 00        	imulq	$0x2a8, %rax, %rax      # imm = 0x2A8
14165db29: 48 8b b4 03 58 fd ff ff     	movq	-0x2a8(%rbx,%rax), %rsi
14165db31: 48 89 b5 c8 00 00 00        	movq	%rsi, 0xc8(%rbp)
14165db38: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
14165db3f: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
14165db44: 0f 84 50 04 00 00           	je	0x14165df9a <.text+0x165cf9a>
14165db4a: 48 ff cf                    	decq	%rdi
14165db4d: 48 8b 8d 88 03 00 00        	movq	0x388(%rbp), %rcx
14165db54: e8 17 32 f5 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14165db59: 49 8b 4d 00                 	movq	(%r13), %rcx
14165db5d: 49 8b 55 08                 	movq	0x8(%r13), %rdx
14165db61: 49 89 d0                    	movq	%rdx, %r8
14165db64: 49 21 c0                    	andq	%rax, %r8
14165db67: 48 c1 e8 39                 	shrq	$0x39, %rax
14165db6b: 66 0f 6e c0                 	movd	%eax, %xmm0
14165db6f: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14165db73: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14165db78: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14165db7d: 48 8d 81 58 fd ff ff        	leaq	-0x2a8(%rcx), %rax
14165db84: 45 31 c9                    	xorl	%r9d, %r9d
14165db87: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14165db8d: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14165db91: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14165db95: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14165db9a: 45 85 d2                    	testl	%r10d, %r10d
14165db9d: 74 31                       	je	0x14165dbd0 <.text+0x165cbd0>
14165db9f: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14165dba4: 4d 01 c3                    	addq	%r8, %r11
14165dba7: 49 21 d3                    	andq	%rdx, %r11
14165dbaa: 49 f7 db                    	negq	%r11
14165dbad: 4d 69 db a8 02 00 00        	imulq	$0x2a8, %r11, %r11      # imm = 0x2A8
14165dbb4: 4a 39 34 18                 	cmpq	%rsi, (%rax,%r11)
14165dbb8: 74 46                       	je	0x14165dc00 <.text+0x165cc00>
14165dbba: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14165dbbe: 66 45 21 d3                 	andw	%r10w, %r11w
14165dbc2: 45 89 da                    	movl	%r11d, %r10d
14165dbc5: 75 d8                       	jne	0x14165db9f <.text+0x165cb9f>
14165dbc7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14165dbd0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
14165dbd4: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
14165dbd9: 45 85 d2                    	testl	%r10d, %r10d
14165dbdc: 0f 85 b8 03 00 00           	jne	0x14165df9a <.text+0x165cf9a>
14165dbe2: 4d 01 c8                    	addq	%r9, %r8
14165dbe5: 49 83 c0 10                 	addq	$0x10, %r8
14165dbe9: 49 83 c1 10                 	addq	$0x10, %r9
14165dbed: 49 21 d0                    	andq	%rdx, %r8
14165dbf0: eb 95                       	jmp	0x14165db87 <.text+0x165cb87>
14165dbf2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14165dc00: 4a 83 bc 19 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rcx,%r11)
14165dc09: 0f 82 a4 1f 00 00           	jb	0x14165fbb3 <.text+0x165ebb3>
14165dc0f: f3 42 0f 10 84 19 90 fd ff ff       	movss	-0x270(%rcx,%r11), %xmm0
14165dc19: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
14165dc1c: 0f 86 91 1f 00 00           	jbe	0x14165fbb3 <.text+0x165ebb3>
14165dc22: 4c 01 d9                    	addq	%r11, %rcx
14165dc25: 83 79 b8 00                 	cmpl	$0x0, -0x48(%rcx)
14165dc29: 44 89 f8                    	movl	%r15d, %eax
14165dc2c: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165dc33: 4c 8d 85 60 03 00 00        	leaq	0x360(%rbp), %r8
14165dc3a: 0f 85 95 fe ff ff           	jne	0x14165dad5 <.text+0x165cad5>
14165dc40: 48 89 bd b0 00 00 00        	movq	%rdi, 0xb0(%rbp)
14165dc47: 4c 89 ad 08 01 00 00        	movq	%r13, 0x108(%rbp)
14165dc4e: f2 44 0f 10 51 bc           	movsd	-0x44(%rcx), %xmm10
14165dc54: 48 89 b5 60 03 00 00        	movq	%rsi, 0x360(%rbp)
14165dc5b: 4c 8b ad a8 03 00 00        	movq	0x3a8(%rbp), %r13
14165dc62: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
14165dc67: 0f 84 c7 07 00 00           	je	0x14165e434 <.text+0x165d434>
14165dc6d: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14165dc74: 4c 89 c2                    	movq	%r8, %rdx
14165dc77: e8 f4 30 f5 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14165dc7c: 49 8b 7d 00                 	movq	(%r13), %rdi
14165dc80: 4d 8b 6d 08                 	movq	0x8(%r13), %r13
14165dc84: 4d 89 e8                    	movq	%r13, %r8
14165dc87: 49 21 c0                    	andq	%rax, %r8
14165dc8a: 48 c1 e8 39                 	shrq	$0x39, %rax
14165dc8e: 66 0f 6e c0                 	movd	%eax, %xmm0
14165dc92: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14165dc96: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14165dc9b: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14165dca0: 48 8d 47 f0                 	leaq	-0x10(%rdi), %rax
14165dca4: 31 d2                       	xorl	%edx, %edx
14165dca6: f3 42 0f 6f 0c 07           	movdqu	(%rdi,%r8), %xmm1
14165dcac: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14165dcb0: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14165dcb4: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14165dcb9: 45 85 d2                    	testl	%r10d, %r10d
14165dcbc: 74 33                       	je	0x14165dcf1 <.text+0x165ccf1>
14165dcbe: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14165dcc3: 4d 01 c3                    	addq	%r8, %r11
14165dcc6: 4d 21 eb                    	andq	%r13, %r11
14165dcc9: 4d 89 d9                    	movq	%r11, %r9
14165dccc: 49 c1 e1 04                 	shlq	$0x4, %r9
14165dcd0: 4c 89 e9                    	movq	%r13, %rcx
14165dcd3: 49 89 c5                    	movq	%rax, %r13
14165dcd6: 4c 29 c8                    	subq	%r9, %rax
14165dcd9: 48 3b 30                    	cmpq	(%rax), %rsi
14165dcdc: 74 39                       	je	0x14165dd17 <.text+0x165cd17>
14165dcde: 41 8d 42 ff                 	leal	-0x1(%r10), %eax
14165dce2: 66 44 21 d0                 	andw	%r10w, %ax
14165dce6: 41 89 c2                    	movl	%eax, %r10d
14165dce9: 4c 89 e8                    	movq	%r13, %rax
14165dcec: 49 89 cd                    	movq	%rcx, %r13
14165dcef: 75 cd                       	jne	0x14165dcbe <.text+0x165ccbe>
14165dcf1: 49 89 c1                    	movq	%rax, %r9
14165dcf4: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
14165dcf8: 66 0f d7 c1                 	pmovmskb	%xmm1, %eax
14165dcfc: 85 c0                       	testl	%eax, %eax
14165dcfe: 0f 85 30 07 00 00           	jne	0x14165e434 <.text+0x165d434>
14165dd04: 49 01 d0                    	addq	%rdx, %r8
14165dd07: 49 83 c0 10                 	addq	$0x10, %r8
14165dd0b: 48 83 c2 10                 	addq	$0x10, %rdx
14165dd0f: 4d 21 e8                    	andq	%r13, %r8
14165dd12: 4c 89 c8                    	movq	%r9, %rax
14165dd15: eb 8f                       	jmp	0x14165dca6 <.text+0x165cca6>
14165dd17: 49 f7 db                    	negq	%r11
14165dd1a: 49 c1 e3 04                 	shlq	$0x4, %r11
14165dd1e: 4a 8b 44 1f f8              	movq	-0x8(%rdi,%r11), %rax
14165dd23: 89 c2                       	movl	%eax, %edx
14165dd25: 48 8b b5 90 03 00 00        	movq	0x390(%rbp), %rsi
14165dd2c: 48 3b 56 10                 	cmpq	0x10(%rsi), %rdx
14165dd30: 0f 83 ad 1f 00 00           	jae	0x14165fce3 <.text+0x165ece3>
14165dd36: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
14165dd3a: 49 89 c1                    	movq	%rax, %r9
14165dd3d: 49 c1 e9 20                 	shrq	$0x20, %r9
14165dd41: 4c 8d 04 92                 	leaq	(%rdx,%rdx,4), %r8
14165dd45: 46 39 0c 81                 	cmpl	%r9d, (%rcx,%r8,4)
14165dd49: 4c 8b 5d 50                 	movq	0x50(%rbp), %r11
14165dd4d: 48 8b bd b0 00 00 00        	movq	0xb0(%rbp), %rdi
14165dd54: 0f 85 90 1f 00 00           	jne	0x14165fcea <.text+0x165ecea>
14165dd5a: 4e 8d 04 81                 	leaq	(%rcx,%r8,4), %r8
14165dd5e: 41 8b 48 04                 	movl	0x4(%r8), %ecx
14165dd62: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
14165dd68: 4c 39 d1                    	cmpq	%r10, %rcx
14165dd6b: 0f 84 79 1f 00 00           	je	0x14165fcea <.text+0x165ecea>
14165dd71: 31 d2                       	xorl	%edx, %edx
14165dd73: 49 39 4b 38                 	cmpq	%rcx, 0x38(%r11)
14165dd77: 0f 86 7c 1f 00 00           	jbe	0x14165fcf9 <.text+0x165ecf9>
14165dd7d: 4d 8b 4b 28                 	movq	0x28(%r11), %r9
14165dd81: 41 89 ca                    	movl	%ecx, %r10d
14165dd84: 41 c1 ea 06                 	shrl	$0x6, %r10d
14165dd88: 4f 8b 0c d1                 	movq	(%r9,%r10,8), %r9
14165dd8c: 49 0f a3 c9                 	btq	%rcx, %r9
14165dd90: 0f 83 63 1f 00 00           	jae	0x14165fcf9 <.text+0x165ecf9>
14165dd96: 41 8b 40 0c                 	movl	0xc(%r8), %eax
14165dd9a: 41 8b 48 10                 	movl	0x10(%r8), %ecx
14165dd9e: 48 8b 96 a8 01 00 00        	movq	0x1a8(%rsi), %rdx
14165dda5: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
14165dda9: 4d 8b 83 10 01 00 00        	movq	0x110(%r11), %r8
14165ddb0: 4c 8b 4c c2 18              	movq	0x18(%rdx,%rax,8), %r9
14165ddb5: 48 8b 44 c2 38              	movq	0x38(%rdx,%rax,8), %rax
14165ddba: 4a 8b 04 c0                 	movq	(%rax,%r8,8), %rax
14165ddbe: 48 f7 d0                    	notq	%rax
14165ddc1: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14165ddc5: 48 c1 e0 04                 	shlq	$0x4, %rax
14165ddc9: 49 8b 54 01 10              	movq	0x10(%r9,%rax), %rdx
14165ddce: 48 6b c1 68                 	imulq	$0x68, %rcx, %rax
14165ddd2: 48 83 7c 02 10 00           	cmpq	$0x0, 0x10(%rdx,%rax)
14165ddd8: 74 4e                       	je	0x14165de28 <.text+0x165ce28>
14165ddda: 48 01 c2                    	addq	%rax, %rdx
14165dddd: c7 85 c8 01 00 00 00 00 00 bf       	movl	$0xbf000000, 0x1c8(%rbp) # imm = 0xBF000000
14165dde7: c7 85 b0 01 00 00 00 00 00 00       	movl	$0x0, 0x1b0(%rbp)
14165ddf1: c7 85 bc 01 00 00 01 00 00 00       	movl	$0x1, 0x1bc(%rbp)
14165ddfb: 44 0f 13 85 c0 01 00 00     	movlps	%xmm8, 0x1c0(%rbp)
14165de03: c6 85 cc 01 00 00 01        	movb	$0x1, 0x1cc(%rbp)
14165de0a: 48 8d 8d d8 02 00 00        	leaq	0x2d8(%rbp), %rcx
14165de11: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
14165de18: e8 d3 a2 28 ff              	callq	0x1408e80f0 <.text+0x8e70f0>
14165de1d: 48 8b b5 70 01 00 00        	movq	0x170(%rbp), %rsi
14165de24: b0 02                       	movb	$0x2, %al
14165de26: eb 09                       	jmp	0x14165de31 <.text+0x165ce31>
14165de28: b0 07                       	movb	$0x7, %al
14165de2a: 48 8b b5 70 01 00 00        	movq	0x170(%rbp), %rsi
14165de31: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165de38: 88 85 d0 02 00 00           	movb	%al, 0x2d0(%rbp)
14165de3e: 48 8d 85 c8 00 00 00        	leaq	0xc8(%rbp), %rax
14165de45: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
14165de4c: 48 8d 05 6d 70 4d ff        	leaq	-0xb28f93(%rip), %rax   # 0x140b34ec0 <.text+0xb33ec0>
14165de53: 48 89 85 68 03 00 00        	movq	%rax, 0x368(%rbp)
14165de5a: 48 8d 05 ef 44 62 01        	leaq	0x16244ef(%rip), %rax   # 0x142c82350
14165de61: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165de68: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165de73: 48 8d 85 60 03 00 00        	leaq	0x360(%rbp), %rax
14165de7a: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14165de81: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
14165de8c: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14165de97: c6 85 b5 03 00 00 01        	movb	$0x1, 0x3b5(%rbp)
14165de9e: 48 89 e9                    	movq	%rbp, %rcx
14165dea1: 49 89 d5                    	movq	%rdx, %r13
14165dea4: e8 e7 c0 71 ff              	callq	0x140d79f90 <.text+0xd78f90>
14165dea9: 0f 10 45 00                 	movups	(%rbp), %xmm0
14165dead: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
14165deb4: 48 8b 45 10                 	movq	0x10(%rbp), %rax
14165deb8: 48 89 85 70 03 00 00        	movq	%rax, 0x370(%rbp)
14165debf: f3 0f 6f 85 d0 02 00 00     	movdqu	0x2d0(%rbp), %xmm0
14165dec7: f3 0f 6f 8d e0 02 00 00     	movdqu	0x2e0(%rbp), %xmm1
14165decf: 48 8d 85 38 02 00 00        	leaq	0x238(%rbp), %rax
14165ded6: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
14165dedb: f3 0f 7f 00                 	movdqu	%xmm0, (%rax)
14165dedf: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
14165deea: 48 c7 85 b8 01 00 00 08 00 00 00    	movq	$0x8, 0x1b8(%rbp)
14165def5: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
14165df00: c7 85 c8 01 00 00 01 00 00 00       	movl	$0x1, 0x1c8(%rbp)
14165df0a: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14165df15: c7 85 e0 01 00 00 02 00 00 00       	movl	$0x2, 0x1e0(%rbp)
14165df1f: c7 85 f8 01 00 00 03 00 00 00       	movl	$0x3, 0x1f8(%rbp)
14165df29: c7 85 10 02 00 00 03 00 00 00       	movl	$0x3, 0x210(%rbp)
14165df33: c6 85 28 02 00 00 00        	movb	$0x0, 0x228(%rbp)
14165df3a: 66 c7 85 58 02 00 00 01 01  	movw	$0x101, 0x258(%rbp)     # imm = 0x101
14165df43: c6 85 b5 03 00 00 00        	movb	$0x0, 0x3b5(%rbp)
14165df4a: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14165df51: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
14165df58: 4d 89 e8                    	movq	%r13, %r8
14165df5b: e8 90 52 2c ff              	callq	0x1409231f0 <.text+0x9221f0>
14165df60: 48 89 95 28 03 00 00        	movq	%rdx, 0x328(%rbp)
14165df67: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
14165df6b: 74 17                       	je	0x14165df84 <.text+0x165cf84>
14165df6d: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
14165df74: 48 39 48 38                 	cmpq	%rcx, 0x38(%rax)
14165df78: 75 0a                       	jne	0x14165df84 <.text+0x165cf84>
14165df7a: 83 78 28 02                 	cmpl	$0x2, 0x28(%rax)
14165df7e: 0f 85 9f 1b 00 00           	jne	0x14165fb23 <.text+0x165eb23>
14165df84: 44 89 f8                    	movl	%r15d, %eax
14165df87: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165df8e: 4c 8b ad 08 01 00 00        	movq	0x108(%rbp), %r13
14165df95: e9 3b fb ff ff              	jmp	0x14165dad5 <.text+0x165cad5>
14165df9a: 48 8d 0d 7f 43 62 01        	leaq	0x162437f(%rip), %rcx   # 0x142c82320
14165dfa1: e8 6a b3 27 01              	callq	0x1428d9310 <.text+0x28d8310>
14165dfa6: e9 6c 1f 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165dfab: 4a 83 bc 13 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rbx,%r10)
14165dfb4: 0f 82 33 1c 00 00           	jb	0x14165fbed <.text+0x165ebed>
14165dfba: f3 42 0f 10 84 13 90 fd ff ff       	movss	-0x270(%rbx,%r10), %xmm0
14165dfc4: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14165dfc8: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14165dfcb: 0f 86 1c 1c 00 00           	jbe	0x14165fbed <.text+0x165ebed>
14165dfd1: 4c 01 d3                    	addq	%r10, %rbx
14165dfd4: 83 7b b8 00                 	cmpl	$0x0, -0x48(%rbx)
14165dfd8: 0f 85 73 02 00 00           	jne	0x14165e251 <.text+0x165d251>
14165dfde: f2 0f 10 73 bc              	movsd	-0x44(%rbx), %xmm6
14165dfe3: f3 0f 16 fe                 	movshdup	%xmm6, %xmm7            # xmm7 = xmm6[1,1,3,3]
14165dfe7: f3 44 0f 10 43 c4           	movss	-0x3c(%rbx), %xmm8
14165dfed: f3 44 0f 10 5b c8           	movss	-0x38(%rbx), %xmm11
14165dff3: f3 44 0f 58 05 74 78 2c 01  	addss	0x12c7874(%rip), %xmm8  # 0x142925870
14165dffc: f3 44 0f 11 5c 24 20        	movss	%xmm11, 0x20(%rsp)
14165e003: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
14165e00a: 0f 28 ce                    	movaps	%xmm6, %xmm1
14165e00d: 0f 28 d7                    	movaps	%xmm7, %xmm2
14165e010: 41 0f 28 d8                 	movaps	%xmm8, %xmm3
14165e014: e8 47 95 63 ff              	callq	0x140c97560 <.text+0xc96560>
14165e019: f6 83 68 fe ff ff 01        	testb	$0x1, -0x198(%rbx)
14165e020: 0f 84 5f 1d 00 00           	je	0x14165fd85 <.text+0x165ed85>
14165e026: f3 0f 10 9b 6c fe ff ff     	movss	-0x194(%rbx), %xmm3
14165e02e: f3 0f 10 15 3a 78 2c 01     	movss	0x12c783a(%rip), %xmm2  # 0x142925870
14165e036: f3 0f 10 43 e4              	movss	-0x1c(%rbx), %xmm0
14165e03b: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
14165e03f: f3 0f 5c d8                 	subss	%xmm0, %xmm3
14165e043: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165e04a: 48 8d 95 48 03 00 00        	leaq	0x348(%rbp), %rdx
14165e051: e8 ba 9d 63 ff              	callq	0x140c97e10 <.text+0xc96e10>
14165e056: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
14165e05d: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
14165e064: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
14165e06b: 0f 11 85 d8 02 00 00        	movups	%xmm0, 0x2d8(%rbp)
14165e072: c7 85 d4 02 00 00 00 00 00 3f       	movl	$0x3f000000, 0x2d4(%rbp) # imm = 0x3F000000
14165e07c: c6 85 d0 02 00 00 01        	movb	$0x1, 0x2d0(%rbp)
14165e083: 48 8b 95 c8 01 00 00        	movq	0x1c8(%rbp), %rdx
14165e08a: 48 85 d2                    	testq	%rdx, %rdx
14165e08d: 74 16                       	je	0x14165e0a5 <.text+0x165d0a5>
14165e08f: 48 8b 8d d0 01 00 00        	movq	0x1d0(%rbp), %rcx
14165e096: 48 c1 e2 02                 	shlq	$0x2, %rdx
14165e09a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14165e0a0: e8 7b 5b fb fe              	callq	0x140613c20 <.text+0x612c20>
14165e0a5: 48 8d 85 b0 02 00 00        	leaq	0x2b0(%rbp), %rax
14165e0ac: 48 89 85 60 03 00 00        	movq	%rax, 0x360(%rbp)
14165e0b3: 48 8d 05 06 6e 4d ff        	leaq	-0xb291fa(%rip), %rax   # 0x140b34ec0 <.text+0xb33ec0>
14165e0ba: 48 89 85 68 03 00 00        	movq	%rax, 0x368(%rbp)
14165e0c1: 48 8d 05 00 43 62 01        	leaq	0x1624300(%rip), %rax   # 0x142c823c8
14165e0c8: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165e0cf: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165e0da: 48 8d 85 60 03 00 00        	leaq	0x360(%rbp), %rax
14165e0e1: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
14165e0e8: 48 c7 85 c8 01 00 00 01 00 00 00    	movq	$0x1, 0x1c8(%rbp)
14165e0f3: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14165e0fe: c6 85 b7 03 00 00 01        	movb	$0x1, 0x3b7(%rbp)
14165e105: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
14165e109: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165e110: e8 7b be 71 ff              	callq	0x140d79f90 <.text+0xd78f90>
14165e115: 0f b6 85 9f 03 00 00        	movzbl	0x39f(%rbp), %eax
14165e11c: 8d 0c 40                    	leal	(%rax,%rax,2), %ecx
14165e11f: 8d 14 00                    	leal	(%rax,%rax), %edx
14165e122: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
14165e126: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
14165e12d: 4c 8b 45 f8                 	movq	-0x8(%rbp), %r8
14165e131: 4c 89 85 70 03 00 00        	movq	%r8, 0x370(%rbp)
14165e138: 0f 10 85 d0 02 00 00        	movups	0x2d0(%rbp), %xmm0
14165e13f: 0f 10 8d e0 02 00 00        	movups	0x2e0(%rbp), %xmm1
14165e146: 0f 11 8d 48 02 00 00        	movups	%xmm1, 0x248(%rbp)
14165e14d: 0f 11 85 38 02 00 00        	movups	%xmm0, 0x238(%rbp)
14165e154: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
14165e15f: 48 c7 85 b8 01 00 00 08 00 00 00    	movq	$0x8, 0x1b8(%rbp)
14165e16a: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
14165e175: 89 85 c8 01 00 00           	movl	%eax, 0x1c8(%rbp)
14165e17b: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14165e186: 89 95 e0 01 00 00           	movl	%edx, 0x1e0(%rbp)
14165e18c: 89 8d f8 01 00 00           	movl	%ecx, 0x1f8(%rbp)
14165e192: 89 8d 10 02 00 00           	movl	%ecx, 0x210(%rbp)
14165e198: c6 85 28 02 00 00 00        	movb	$0x0, 0x228(%rbp)
14165e19f: 66 c7 85 58 02 00 00 00 01  	movw	$0x100, 0x258(%rbp)     # imm = 0x100
14165e1a8: c6 85 b7 03 00 00 00        	movb	$0x0, 0x3b7(%rbp)
14165e1af: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
14165e1b6: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
14165e1bd: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14165e1c4: e8 27 50 2c ff              	callq	0x1409231f0 <.text+0x9221f0>
14165e1c9: 80 78 18 00                 	cmpb	$0x0, 0x18(%rax)
14165e1cd: 74 10                       	je	0x14165e1df <.text+0x165d1df>
14165e1cf: 48 39 50 38                 	cmpq	%rdx, 0x38(%rax)
14165e1d3: 75 0a                       	jne	0x14165e1df <.text+0x165d1df>
14165e1d5: 83 78 28 02                 	cmpl	$0x2, 0x28(%rax)
14165e1d9: 0f 85 d4 12 00 00           	jne	0x14165f4b3 <.text+0x165e4b3>
14165e1df: 48 8b b5 b0 02 00 00        	movq	0x2b0(%rbp), %rsi
14165e1e6: c6 85 b7 03 00 00 00        	movb	$0x0, 0x3b7(%rbp)
14165e1ed: f3 44 0f 11 5c 24 20        	movss	%xmm11, 0x20(%rsp)
14165e1f4: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
14165e1fb: 0f 28 ce                    	movaps	%xmm6, %xmm1
14165e1fe: 0f 28 d7                    	movaps	%xmm7, %xmm2
14165e201: 41 0f 28 d8                 	movaps	%xmm8, %xmm3
14165e205: e8 56 93 63 ff              	callq	0x140c97560 <.text+0xc96560>
14165e20a: f6 83 68 fe ff ff 01        	testb	$0x1, -0x198(%rbx)
14165e211: 0f 84 0b 1c 00 00           	je	0x14165fe22 <.text+0x165ee22>
14165e217: f3 0f 10 5b e4              	movss	-0x1c(%rbx), %xmm3
14165e21c: f3 0f 59 1d c4 be 2f 01     	mulss	0x12fbec4(%rip), %xmm3  # 0x14295a0e8
14165e224: f3 0f 58 9b 6c fe ff ff     	addss	-0x194(%rbx), %xmm3
14165e22c: c6 85 b7 03 00 00 00        	movb	$0x0, 0x3b7(%rbp)
14165e233: c7 44 24 20 00 00 40 40     	movl	$0x40400000, 0x20(%rsp) # imm = 0x40400000
14165e23b: 4c 8d 85 60 03 00 00        	leaq	0x360(%rbp), %r8
14165e242: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
14165e249: 48 89 f2                    	movq	%rsi, %rdx
14165e24c: e8 2f 26 00 00              	callq	0x141660880 <.text+0x165f880>
14165e251: 44 8b 85 88 03 00 00        	movl	0x388(%rbp), %r8d
14165e258: 41 80 f8 02                 	cmpb	$0x2, %r8b
14165e25c: 0f 84 04 18 00 00           	je	0x14165fa66 <.text+0x165ea66>
14165e262: 44 89 85 88 03 00 00        	movl	%r8d, 0x388(%rbp)
14165e269: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
14165e270: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165e277: 49 83 7d 18 00              	cmpq	$0x0, 0x18(%r13)
14165e27c: 0f 84 90 01 00 00           	je	0x14165e412 <.text+0x165d412>
14165e282: 4c 8b bd 90 04 00 00        	movq	0x490(%rbp), %r15
14165e289: 49 8d 4d 20                 	leaq	0x20(%r13), %rcx
14165e28d: 48 8d b5 b0 01 00 00        	leaq	0x1b0(%rbp), %rsi
14165e294: 48 89 f2                    	movq	%rsi, %rdx
14165e297: e8 d4 2a f5 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14165e29c: 49 8b 5d 00                 	movq	(%r13), %rbx
14165e2a0: 49 8b 4d 08                 	movq	0x8(%r13), %rcx
14165e2a4: 48 89 ca                    	movq	%rcx, %rdx
14165e2a7: 48 21 c2                    	andq	%rax, %rdx
14165e2aa: 48 c1 e8 39                 	shrq	$0x39, %rax
14165e2ae: 66 0f 6e c0                 	movd	%eax, %xmm0
14165e2b2: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14165e2b6: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14165e2bb: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14165e2c0: 48 8d 83 58 fd ff ff        	leaq	-0x2a8(%rbx), %rax
14165e2c7: 45 31 c0                    	xorl	%r8d, %r8d
14165e2ca: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14165e2ce: 4c 8b 9d a0 01 00 00        	movq	0x1a0(%rbp), %r11
14165e2d5: 4c 8b ad a8 03 00 00        	movq	0x3a8(%rbp), %r13
14165e2dc: f3 0f 6f 14 13              	movdqu	(%rbx,%rdx), %xmm2
14165e2e1: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14165e2e5: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14165e2e9: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
14165e2ee: 45 85 c9                    	testl	%r9d, %r9d
14165e2f1: 74 28                       	je	0x14165e31b <.text+0x165d31b>
14165e2f3: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
14165e2f8: 49 01 d2                    	addq	%rdx, %r10
14165e2fb: 49 21 ca                    	andq	%rcx, %r10
14165e2fe: 49 f7 da                    	negq	%r10
14165e301: 4d 69 d2 a8 02 00 00        	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
14165e308: 4e 39 2c 10                 	cmpq	%r13, (%rax,%r10)
14165e30c: 74 2f                       	je	0x14165e33d <.text+0x165d33d>
14165e30e: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
14165e312: 66 45 21 ca                 	andw	%r9w, %r10w
14165e316: 45 89 d1                    	movl	%r10d, %r9d
14165e319: 75 d8                       	jne	0x14165e2f3 <.text+0x165d2f3>
14165e31b: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14165e31f: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
14165e324: 45 85 c9                    	testl	%r9d, %r9d
14165e327: 0f 85 e5 00 00 00           	jne	0x14165e412 <.text+0x165d412>
14165e32d: 4c 01 c2                    	addq	%r8, %rdx
14165e330: 48 83 c2 10                 	addq	$0x10, %rdx
14165e334: 49 83 c0 10                 	addq	$0x10, %r8
14165e338: 48 21 ca                    	andq	%rcx, %rdx
14165e33b: eb 9f                       	jmp	0x14165e2dc <.text+0x165d2dc>
14165e33d: 4a 83 bc 13 70 fd ff ff 02  	cmpq	$0x2, -0x290(%rbx,%r10)
14165e346: 0f 82 84 18 00 00           	jb	0x14165fbd0 <.text+0x165ebd0>
14165e34c: f3 42 0f 10 84 13 90 fd ff ff       	movss	-0x270(%rbx,%r10), %xmm0
14165e356: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14165e35a: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14165e35d: 0f 86 6d 18 00 00           	jbe	0x14165fbd0 <.text+0x165ebd0>
14165e363: 4c 01 d3                    	addq	%r10, %rbx
14165e366: 8b 43 b8                    	movl	-0x48(%rbx), %eax
14165e369: 83 f8 02                    	cmpl	$0x2, %eax
14165e36c: 0f 83 1a 19 00 00           	jae	0x14165fc8c <.text+0x165ec8c>
14165e372: a8 01                       	testb	$0x1, %al
14165e374: 0f 85 23 19 00 00           	jne	0x14165fc9d <.text+0x165ec9d>
14165e37a: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
14165e381: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14165e385: b0 02                       	movb	$0x2, %al
14165e387: 41 80 7b 18 02              	cmpb	$0x2, 0x18(%r11)
14165e38c: 72 12                       	jb	0x14165e3a0 <.text+0x165d3a0>
14165e38e: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
14165e395: 49 39 4b 38                 	cmpq	%rcx, 0x38(%r11)
14165e399: 75 05                       	jne	0x14165e3a0 <.text+0x165d3a0>
14165e39b: 41 0f b6 43 19              	movzbl	0x19(%r11), %eax
14165e3a0: 48 8d 53 bc                 	leaq	-0x44(%rbx), %rdx
14165e3a4: 04 fe                       	addb	$-0x2, %al
14165e3a6: 0f b6 c0                    	movzbl	%al, %eax
14165e3a9: 3c 03                       	cmpb	$0x3, %al
14165e3ab: b9 03 00 00 00              	movl	$0x3, %ecx
14165e3b0: 0f 42 c8                    	cmovbl	%eax, %ecx
14165e3b3: 0f b6 c1                    	movzbl	%cl, %eax
14165e3b6: 48 8d 0d 0f 43 62 01        	leaq	0x162430f(%rip), %rcx   # 0x142c826cc
14165e3bd: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
14165e3c1: 48 01 c8                    	addq	%rcx, %rax
14165e3c4: 48 89 95 90 03 00 00        	movq	%rdx, 0x390(%rbp)
14165e3cb: ff e0                       	jmpq	*%rax
14165e3cd: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165e3d4: 44 0f 13 10                 	movlps	%xmm10, (%rax)
14165e3d8: f3 44 0f 11 48 08           	movss	%xmm9, 0x8(%rax)
14165e3de: 0f 10 02                    	movups	(%rdx), %xmm0
14165e3e1: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
14165e3e8: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14165e3ef: c7 00 01 00 00 00           	movl	$0x1, (%rax)
14165e3f5: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
14165e3fc: 0f 11 40 04                 	movups	%xmm0, 0x4(%rax)
14165e400: f6 85 88 03 00 00 01        	testb	$0x1, 0x388(%rbp)
14165e407: 0f 85 37 0d 00 00           	jne	0x14165f144 <.text+0x165e144>
14165e40d: e9 9f 13 00 00              	jmp	0x14165f7b1 <.text+0x165e7b1>
14165e412: 48 8d 0d d7 3f 62 01        	leaq	0x1623fd7(%rip), %rcx   # 0x142c823f0
14165e419: e8 f2 ae 27 01              	callq	0x1428d9310 <.text+0x28d8310>
14165e41e: e9 f4 1a 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165e423: 48 8d 0d 66 3f 62 01        	leaq	0x1623f66(%rip), %rcx   # 0x142c82390
14165e42a: e8 e1 ae 27 01              	callq	0x1428d9310 <.text+0x28d8310>
14165e42f: e9 e3 1a 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165e434: 48 8d 0d 43 37 62 01        	leaq	0x1623743(%rip), %rcx   # 0x142c81b7e
14165e43b: 4c 8d 05 16 42 62 01        	leaq	0x1624216(%rip), %r8    # 0x142c82658
14165e442: ba 16 00 00 00              	movl	$0x16, %edx
14165e447: e8 e4 ae 27 01              	callq	0x1428d9330 <.text+0x28d8330>
14165e44c: e9 c6 1a 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165e451: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165e458: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
14165e45d: 48 8d 8d 20 01 00 00        	leaq	0x120(%rbp), %rcx
14165e464: e8 77 e9 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165e469: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
14165e470: 48 8b 95 a0 03 00 00        	movq	0x3a0(%rbp), %rdx
14165e477: e8 44 e9 63 ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
14165e47c: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
14165e483: 8b 01                       	movl	(%rcx), %eax
14165e485: 0f 57 d2                    	xorps	%xmm2, %xmm2
14165e488: f3 48 0f 2a d0              	cvtsi2ss	%rax, %xmm2
14165e48d: 8b 41 04                    	movl	0x4(%rcx), %eax
14165e490: 0f 57 db                    	xorps	%xmm3, %xmm3
14165e493: f3 48 0f 2a d8              	cvtsi2ss	%rax, %xmm3
14165e498: 0f 28 f2                    	movaps	%xmm2, %xmm6
14165e49b: f3 0f 5e f3                 	divss	%xmm3, %xmm6
14165e49f: f3 0f 11 95 d0 02 00 00     	movss	%xmm2, 0x2d0(%rbp)
14165e4a7: f3 0f 11 9d d4 02 00 00     	movss	%xmm3, 0x2d4(%rbp)
14165e4af: f3 0f 11 b5 d8 02 00 00     	movss	%xmm6, 0x2d8(%rbp)
14165e4b7: c7 85 dc 02 00 00 00 00 80 3f       	movl	$0x3f800000, 0x2dc(%rbp) # imm = 0x3F800000
14165e4c1: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14165e4c8: f3 0f 10 00                 	movss	(%rax), %xmm0
14165e4cc: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
14165e4d1: e8 fa 21 63 ff              	callq	0x140c906d0 <.text+0xc8f6d0>
14165e4d6: 0f 28 d1                    	movaps	%xmm1, %xmm2
14165e4d9: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
14165e4dd: 8b 85 28 01 00 00           	movl	0x128(%rbp), %eax
14165e4e3: 89 85 b8 01 00 00           	movl	%eax, 0x1b8(%rbp)
14165e4e9: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
14165e4f0: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165e4f7: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
14165e4fe: 48 89 85 bc 01 00 00        	movq	%rax, 0x1bc(%rbp)
14165e505: 8b 85 50 03 00 00           	movl	0x350(%rbp), %eax
14165e50b: 89 85 c4 01 00 00           	movl	%eax, 0x1c4(%rbp)
14165e511: f2 0f 10 05 07 e6 45 01     	movsd	0x145e607(%rip), %xmm0  # 0x142abcb20
14165e519: f2 0f 11 85 60 03 00 00     	movsd	%xmm0, 0x360(%rbp)
14165e521: c6 85 68 03 00 00 00        	movb	$0x0, 0x368(%rbp)
14165e528: 48 8d 85 60 03 00 00        	leaq	0x360(%rbp), %rax
14165e52f: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14165e534: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
14165e539: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14165e540: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14165e545: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
14165e54c: 4c 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %r9
14165e553: 0f 28 ce                    	movaps	%xmm6, %xmm1
14165e556: e8 45 9f 46 ff              	callq	0x140ac84a0 <.text+0xac74a0>
14165e55b: f3 0f 10 85 b4 02 00 00     	movss	0x2b4(%rbp), %xmm0
14165e563: f3 0f 5c 85 24 01 00 00     	subss	0x124(%rbp), %xmm0
14165e56b: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14165e56f: f3 0f 10 8d b8 02 00 00     	movss	0x2b8(%rbp), %xmm1
14165e577: f3 0f 10 95 b0 02 00 00     	movss	0x2b0(%rbp), %xmm2
14165e57f: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
14165e582: f3 0f 10 8d 20 01 00 00     	movss	0x120(%rbp), %xmm1
14165e58a: f3 0f 10 9d 28 01 00 00     	movss	0x128(%rbp), %xmm3
14165e592: 0f 14 cb                    	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
14165e595: 0f 5c d1                    	subps	%xmm1, %xmm2
14165e598: 0f 59 d2                    	mulps	%xmm2, %xmm2
14165e59b: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14165e59f: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
14165e5a3: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14165e5a7: 0f 57 f6                    	xorps	%xmm6, %xmm6
14165e5aa: f3 0f 51 f1                 	sqrtss	%xmm1, %xmm6
14165e5ae: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14165e5b5: f3 0f 5c 70 14              	subss	0x14(%rax), %xmm6
14165e5ba: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
14165e5c1: 0f 10 00                    	movups	(%rax), %xmm0
14165e5c4: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
14165e5cb: f3 0f 10 7b e0              	movss	-0x20(%rbx), %xmm7
14165e5d0: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
14165e5d7: 8b 8d 90 01 00 00           	movl	0x190(%rbp), %ecx
14165e5dd: 89 08                       	movl	%ecx, (%rax)
14165e5df: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14165e5e6: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14165e5ea: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14165e5ee: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14165e5f2: 75 17                       	jne	0x14165e60b <.text+0x165d60b>
14165e5f4: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14165e5fb: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165e5ff: 48 8d 15 42 bb 5d 01        	leaq	0x15dbb42(%rip), %rdx   # 0x142c3a148
14165e606: e8 e5 43 f0 fe              	callq	0x1405629f0 <.text+0x5619f0>
14165e60b: 48 8b 95 18 03 00 00        	movq	0x318(%rbp), %rdx
14165e612: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14165e616: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
14165e61a: 48 c1 e1 04                 	shlq	$0x4, %rcx
14165e61e: c7 04 08 00 00 00 00        	movl	$0x0, (%rax,%rcx)
14165e625: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
14165e62c: 0f 11 44 08 04              	movups	%xmm0, 0x4(%rax,%rcx)
14165e631: f3 0f 11 7c 08 1c           	movss	%xmm7, 0x1c(%rax,%rcx)
14165e637: c7 44 08 20 00 00 c0 3f     	movl	$0x3fc00000, 0x20(%rax,%rcx) # imm = 0x3FC00000
14165e63f: c6 44 08 24 00              	movb	$0x0, 0x24(%rax,%rcx)
14165e644: 48 89 7c 08 28              	movq	%rdi, 0x28(%rax,%rcx)
14165e649: 48 ff c6                    	incq	%rsi
14165e64c: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
14165e650: 48 ff 42 40                 	incq	0x40(%rdx)
14165e654: f6 83 68 fe ff ff 01        	testb	$0x1, -0x198(%rbx)
14165e65b: 0f 84 7b 17 00 00           	je	0x14165fddc <.text+0x165eddc>
14165e661: f3 0f 10 7b e0              	movss	-0x20(%rbx), %xmm7
14165e666: f3 44 0f 10 83 6c fe ff ff  	movss	-0x194(%rbx), %xmm8
14165e66f: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
14165e676: 0f 10 00                    	movups	(%rax), %xmm0
14165e679: 0f 11 85 b2 01 00 00        	movups	%xmm0, 0x1b2(%rbp)
14165e680: 48 8b 45 60                 	movq	0x60(%rbp), %rax
14165e684: 8b 8d 38 01 00 00           	movl	0x138(%rbp), %ecx
14165e68a: 89 08                       	movl	%ecx, (%rax)
14165e68c: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
14165e693: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14165e697: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14165e69b: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14165e69f: 75 17                       	jne	0x14165e6b8 <.text+0x165d6b8>
14165e6a1: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
14165e6a8: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165e6ac: 48 8d 15 95 ba 5d 01        	leaq	0x15dba95(%rip), %rdx   # 0x142c3a148
14165e6b3: e8 f8 40 f0 fe              	callq	0x1405627b0 <.text+0x5617b0>
14165e6b8: f3 0f 10 05 c0 e1 41 01     	movss	0x141e1c0(%rip), %xmm0  # 0x142a7c880
14165e6c0: f3 0f 5f c6                 	maxss	%xmm6, %xmm0
14165e6c4: 4c 8b 85 50 01 00 00        	movq	0x150(%rbp), %r8
14165e6cb: 49 8b 40 28                 	movq	0x28(%r8), %rax
14165e6cf: 48 89 f1                    	movq	%rsi, %rcx
14165e6d2: 48 c1 e1 06                 	shlq	$0x6, %rcx
14165e6d6: 66 c7 04 08 00 00           	movw	$0x0, (%rax,%rcx)
14165e6dc: 0f 10 8d b0 01 00 00        	movups	0x1b0(%rbp), %xmm1
14165e6e3: 0f 11 4c 08 02              	movups	%xmm1, 0x2(%rax,%rcx)
14165e6e8: f3 0f 10 0d 14 5f 2d 01     	movss	0x12d5f14(%rip), %xmm1  # 0x142934604
14165e6f0: 0f b7 95 c0 01 00 00        	movzwl	0x1c0(%rbp), %edx
14165e6f7: 66 89 54 08 12              	movw	%dx, 0x12(%rax,%rcx)
14165e6fc: f3 0f 11 7c 08 14           	movss	%xmm7, 0x14(%rax,%rcx)
14165e702: f3 44 0f 11 44 08 18        	movss	%xmm8, 0x18(%rax,%rcx)
14165e709: 48 c7 44 08 20 01 00 00 00  	movq	$0x1, 0x20(%rax,%rcx)
14165e712: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165e719: 48 89 54 08 28              	movq	%rdx, 0x28(%rax,%rcx)
14165e71e: 48 89 7c 08 38              	movq	%rdi, 0x38(%rax,%rcx)
14165e723: 48 ff c6                    	incq	%rsi
14165e726: 49 89 70 30                 	movq	%rsi, 0x30(%r8)
14165e72a: 49 ff 40 40                 	incq	0x40(%r8)
14165e72e: f3 0f 5d c8                 	minss	%xmm0, %xmm1
14165e732: f3 0f 10 73 c4              	movss	-0x3c(%rbx), %xmm6
14165e737: 0f 2e ce                    	ucomiss	%xmm6, %xmm1
14165e73a: 48 8b 95 90 03 00 00        	movq	0x390(%rbp), %rdx
14165e741: 75 06                       	jne	0x14165e749 <.text+0x165d749>
14165e743: 0f 8b ee 09 00 00           	jnp	0x14165f137 <.text+0x165e137>
14165e749: 49 8b 37                    	movq	(%r15), %rsi
14165e74c: f3 0f 10 7b bc              	movss	-0x44(%rbx), %xmm7
14165e751: f3 44 0f 10 43 c0           	movss	-0x40(%rbx), %xmm8
14165e757: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14165e75e: f3 0f 10 50 28              	movss	0x28(%rax), %xmm2
14165e763: f3 0f 10 1d 3d 5e 2d 01     	movss	0x12d5e3d(%rip), %xmm3  # 0x1429345a8
14165e76b: 0f 28 c6                    	movaps	%xmm6, %xmm0
14165e76e: e8 6d 1d 63 ff              	callq	0x140c904e0 <.text+0xc8f4e0>
14165e773: f3 44 0f 10 4b c8           	movss	-0x38(%rbx), %xmm9
14165e779: f3 44 0f 11 4c 24 20        	movss	%xmm9, 0x20(%rsp)
14165e780: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
14165e787: 0f 28 cf                    	movaps	%xmm7, %xmm1
14165e78a: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14165e78e: 0f 28 d8                    	movaps	%xmm0, %xmm3
14165e791: e8 ca 8d 63 ff              	callq	0x140c97560 <.text+0xc96560>
14165e796: f3 44 0f 11 8d 54 03 00 00  	movss	%xmm9, 0x354(%rbp)
14165e79f: 0f 10 85 48 03 00 00        	movups	0x348(%rbp), %xmm0
14165e7a6: 0f 11 85 b4 01 00 00        	movups	%xmm0, 0x1b4(%rbp)
14165e7ad: c7 85 b0 01 00 00 00 00 00 00       	movl	$0x0, 0x1b0(%rbp)
14165e7b7: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165e7be: 48 89 f1                    	movq	%rsi, %rcx
14165e7c1: e8 aa 41 47 ff              	callq	0x140ad2970 <.text+0xad1970>
14165e7c6: 84 c0                       	testb	%al, %al
14165e7c8: 0f 84 b9 05 00 00           	je	0x14165ed87 <.text+0x165dd87>
14165e7ce: f3 0f 10 85 50 03 00 00     	movss	0x350(%rbp), %xmm0
14165e7d6: e9 87 0d 00 00              	jmp	0x14165f562 <.text+0x165e562>
14165e7db: 48 8b bd 38 03 00 00        	movq	0x338(%rbp), %rdi
14165e7e2: f6 07 01                    	testb	$0x1, (%rdi)
14165e7e5: 0f 84 e0 15 00 00           	je	0x14165fdcb <.text+0x165edcb>
14165e7eb: f3 0f 10 47 0c              	movss	0xc(%rdi), %xmm0
14165e7f0: f3 0f 5c 43 c4              	subss	-0x3c(%rbx), %xmm0
14165e7f5: 0f 54 05 84 01 2d 01        	andps	0x12d0184(%rip), %xmm0  # 0x14292e980
14165e7fc: 0f 2e 05 71 1e 30 01        	ucomiss	0x1301e71(%rip), %xmm0  # 0x142960674
14165e803: 0f 86 aa 03 00 00           	jbe	0x14165ebb3 <.text+0x165dbb3>
14165e809: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
14165e810: 48 8b 30                    	movq	(%rax), %rsi
14165e813: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
14165e81a: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165e821: 48 83 7e 18 00              	cmpq	$0x0, 0x18(%rsi)
14165e826: 0f 84 76 0c 00 00           	je	0x14165f4a2 <.text+0x165e4a2>
14165e82c: 48 83 c7 04                 	addq	$0x4, %rdi
14165e830: 48 8d 4e 20                 	leaq	0x20(%rsi), %rcx
14165e834: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165e83b: e8 30 25 f5 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14165e840: 48 8b 0e                    	movq	(%rsi), %rcx
14165e843: 48 8b 56 08                 	movq	0x8(%rsi), %rdx
14165e847: 49 89 d0                    	movq	%rdx, %r8
14165e84a: 49 21 c0                    	andq	%rax, %r8
14165e84d: 48 c1 e8 39                 	shrq	$0x39, %rax
14165e851: 66 0f 6e c0                 	movd	%eax, %xmm0
14165e855: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14165e859: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14165e85e: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14165e863: 48 8d 41 88                 	leaq	-0x78(%rcx), %rax
14165e867: 45 31 c9                    	xorl	%r9d, %r9d
14165e86a: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
14165e86e: 48 8b b5 a8 03 00 00        	movq	0x3a8(%rbp), %rsi
14165e875: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
14165e87b: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
14165e87f: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
14165e883: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
14165e888: 45 85 d2                    	testl	%r10d, %r10d
14165e88b: 74 29                       	je	0x14165e8b6 <.text+0x165d8b6>
14165e88d: f3 45 0f bc da              	tzcntl	%r10d, %r11d
14165e892: 4d 01 c3                    	addq	%r8, %r11
14165e895: 49 21 d3                    	andq	%rdx, %r11
14165e898: 49 f7 db                    	negq	%r11
14165e89b: 4d 6b db 78                 	imulq	$0x78, %r11, %r11
14165e89f: 4a 39 34 18                 	cmpq	%rsi, (%rax,%r11)
14165e8a3: 0f 84 40 03 00 00           	je	0x14165ebe9 <.text+0x165dbe9>
14165e8a9: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
14165e8ad: 66 45 21 d3                 	andw	%r10w, %r11w
14165e8b1: 45 89 da                    	movl	%r11d, %r10d
14165e8b4: 75 d7                       	jne	0x14165e88d <.text+0x165d88d>
14165e8b6: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
14165e8ba: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
14165e8bf: 45 85 d2                    	testl	%r10d, %r10d
14165e8c2: 0f 85 da 0b 00 00           	jne	0x14165f4a2 <.text+0x165e4a2>
14165e8c8: 4d 01 c8                    	addq	%r9, %r8
14165e8cb: 49 83 c0 10                 	addq	$0x10, %r8
14165e8cf: 49 83 c1 10                 	addq	$0x10, %r9
14165e8d3: 49 21 d0                    	andq	%rdx, %r8
14165e8d6: eb 9d                       	jmp	0x14165e875 <.text+0x165d875>
14165e8d8: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165e8df: 44 0f 13 10                 	movlps	%xmm10, (%rax)
14165e8e3: f3 44 0f 11 48 08           	movss	%xmm9, 0x8(%rax)
14165e8e9: 0f 10 02                    	movups	(%rdx), %xmm0
14165e8ec: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
14165e8f3: 48 8b 8d 38 03 00 00        	movq	0x338(%rbp), %rcx
14165e8fa: c7 01 01 00 00 00           	movl	$0x1, (%rcx)
14165e900: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
14165e907: 0f 11 41 04                 	movups	%xmm0, 0x4(%rcx)
14165e90b: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
14165e910: 48 8d 8d 2c 01 00 00        	leaq	0x12c(%rbp), %rcx
14165e917: e8 c4 e4 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165e91c: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
14165e923: 48 8b 95 a0 03 00 00        	movq	0x3a0(%rbp), %rdx
14165e92a: e8 91 e4 63 ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
14165e92f: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
14165e936: 8b 01                       	movl	(%rcx), %eax
14165e938: 0f 57 d2                    	xorps	%xmm2, %xmm2
14165e93b: f3 48 0f 2a d0              	cvtsi2ss	%rax, %xmm2
14165e940: 8b 41 04                    	movl	0x4(%rcx), %eax
14165e943: f3 48 0f 2a d8              	cvtsi2ss	%rax, %xmm3
14165e948: 0f 28 f2                    	movaps	%xmm2, %xmm6
14165e94b: f3 0f 5e f3                 	divss	%xmm3, %xmm6
14165e94f: f3 0f 11 95 d0 02 00 00     	movss	%xmm2, 0x2d0(%rbp)
14165e957: f3 0f 11 9d d4 02 00 00     	movss	%xmm3, 0x2d4(%rbp)
14165e95f: f3 0f 11 b5 d8 02 00 00     	movss	%xmm6, 0x2d8(%rbp)
14165e967: c7 85 dc 02 00 00 00 00 80 3f       	movl	$0x3f800000, 0x2dc(%rbp) # imm = 0x3F800000
14165e971: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14165e978: f3 0f 10 00                 	movss	(%rax), %xmm0
14165e97c: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
14165e981: e8 4a 1d 63 ff              	callq	0x140c906d0 <.text+0xc8f6d0>
14165e986: 0f 28 d1                    	movaps	%xmm1, %xmm2
14165e989: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
14165e98d: 8b 85 34 01 00 00           	movl	0x134(%rbp), %eax
14165e993: 89 85 b8 01 00 00           	movl	%eax, 0x1b8(%rbp)
14165e999: 48 8b 85 2c 01 00 00        	movq	0x12c(%rbp), %rax
14165e9a0: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165e9a7: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
14165e9ae: 48 89 85 bc 01 00 00        	movq	%rax, 0x1bc(%rbp)
14165e9b5: 8b 85 50 03 00 00           	movl	0x350(%rbp), %eax
14165e9bb: 89 85 c4 01 00 00           	movl	%eax, 0x1c4(%rbp)
14165e9c1: f2 0f 10 05 57 e1 45 01     	movsd	0x145e157(%rip), %xmm0  # 0x142abcb20
14165e9c9: f2 0f 11 85 60 03 00 00     	movsd	%xmm0, 0x360(%rbp)
14165e9d1: c6 85 68 03 00 00 00        	movb	$0x0, 0x368(%rbp)
14165e9d8: 48 8d 85 60 03 00 00        	leaq	0x360(%rbp), %rax
14165e9df: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14165e9e4: 48 89 74 24 28              	movq	%rsi, 0x28(%rsp)
14165e9e9: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14165e9f0: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14165e9f5: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
14165e9fc: 4c 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %r9
14165ea03: 0f 28 ce                    	movaps	%xmm6, %xmm1
14165ea06: e8 95 9a 46 ff              	callq	0x140ac84a0 <.text+0xac74a0>
14165ea0b: f3 0f 10 85 b4 02 00 00     	movss	0x2b4(%rbp), %xmm0
14165ea13: f3 0f 5c 85 30 01 00 00     	subss	0x130(%rbp), %xmm0
14165ea1b: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
14165ea1f: f3 0f 10 8d b8 02 00 00     	movss	0x2b8(%rbp), %xmm1
14165ea27: f3 0f 10 95 b0 02 00 00     	movss	0x2b0(%rbp), %xmm2
14165ea2f: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
14165ea32: f3 0f 10 8d 2c 01 00 00     	movss	0x12c(%rbp), %xmm1
14165ea3a: f3 0f 10 9d 34 01 00 00     	movss	0x134(%rbp), %xmm3
14165ea42: 0f 14 cb                    	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
14165ea45: 0f 5c d1                    	subps	%xmm1, %xmm2
14165ea48: 0f 59 d2                    	mulps	%xmm2, %xmm2
14165ea4b: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14165ea4f: f3 0f 16 ca                 	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
14165ea53: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14165ea57: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165ea5a: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
14165ea5e: f3 0f 5c 43 c4              	subss	-0x3c(%rbx), %xmm0
14165ea63: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14165ea6a: f3 0f 11 40 14              	movss	%xmm0, 0x14(%rax)
14165ea6f: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
14165ea76: 8b 8d 44 03 00 00           	movl	0x344(%rbp), %ecx
14165ea7c: 89 08                       	movl	%ecx, (%rax)
14165ea7e: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
14165ea89: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
14165ea90: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14165ea97: 48 c7 85 c0 01 00 00 01 00 00 00    	movq	$0x1, 0x1c0(%rbp)
14165eaa2: 48 8d 05 5f 39 62 01        	leaq	0x162395f(%rip), %rax   # 0x142c82408
14165eaa9: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14165eaae: 48 8b 45 58                 	movq	0x58(%rbp), %rax
14165eab2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14165eab7: 48 c7 44 24 30 0b 00 00 00  	movq	$0xb, 0x30(%rsp)
14165eac0: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165eac7: 41 b8 02 00 00 00           	movl	$0x2, %r8d
14165eacd: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
14165ead4: 4c 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %r9
14165eadb: e8 10 8d d8 ff              	callq	0x1413e77f0 <.text+0x13e67f0>
14165eae0: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
14165eae7: 8b 8d 44 03 00 00           	movl	0x344(%rbp), %ecx
14165eaed: 89 08                       	movl	%ecx, (%rax)
14165eaef: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
14165eaf6: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165eafd: 4c 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %r8
14165eb04: e8 f7 d0 4c ff              	callq	0x140b2bc00 <.text+0xb2ac00>
14165eb09: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
14165eb10: 8b 8d 44 03 00 00           	movl	0x344(%rbp), %ecx
14165eb16: 89 08                       	movl	%ecx, (%rax)
14165eb18: 48 8b 95 20 03 00 00        	movq	0x320(%rbp), %rdx
14165eb1f: 48 83 c2 30                 	addq	$0x30, %rdx
14165eb23: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
14165eb2a: 48 8b 81 90 00 00 00        	movq	0x90(%rcx), %rax
14165eb31: 48 81 c1 90 00 00 00        	addq	$0x90, %rcx
14165eb38: 48 85 c0                    	testq	%rax, %rax
14165eb3b: 48 0f 44 c8                 	cmoveq	%rax, %rcx
14165eb3f: 48 c7 85 b0 01 00 00 01 00 00 00    	movq	$0x1, 0x1b0(%rbp)
14165eb4a: 48 89 8d b8 01 00 00        	movq	%rcx, 0x1b8(%rbp)
14165eb51: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
14165eb5c: 48 c7 85 e8 01 00 00 00 00 00 00    	movq	$0x0, 0x1e8(%rbp)
14165eb67: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165eb6e: e8 0d 47 c3 ff              	callq	0x141293280 <.text+0x1292280>
14165eb73: e9 25 0c 00 00              	jmp	0x14165f79d <.text+0x165e79d>
14165eb78: 48 85 c0                    	testq	%rax, %rax
14165eb7b: 0f 85 96 0f 00 00           	jne	0x14165fb17 <.text+0x165eb17>
14165eb81: 31 d2                       	xorl	%edx, %edx
14165eb83: e8 48 9c 29 01              	callq	0x1428f87d0 <.text+0x28f77d0>
14165eb88: 48 89 c1                    	movq	%rax, %rcx
14165eb8b: 48 85 c0                    	testq	%rax, %rax
14165eb8e: 0f 85 26 ed ff ff           	jne	0x14165d8ba <.text+0x165c8ba>
14165eb94: e9 7e 0f 00 00              	jmp	0x14165fb17 <.text+0x165eb17>
14165eb99: 41 b0 02                    	movb	$0x2, %r8b
14165eb9c: 48 8b b5 70 01 00 00        	movq	0x170(%rbp), %rsi
14165eba3: 8b 06                       	movl	(%rsi), %eax
14165eba5: 83 f8 05                    	cmpl	$0x5, %eax
14165eba8: 0f 86 fa ed ff ff           	jbe	0x14165d9a8 <.text+0x165c9a8>
14165ebae: e9 a5 f6 ff ff              	jmp	0x14165e258 <.text+0x165d258>
14165ebb3: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
14165ebba: 8b 8d 44 03 00 00           	movl	0x344(%rbp), %ecx
14165ebc0: 89 08                       	movl	%ecx, (%rax)
14165ebc2: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14165ebc9: 8b 8d 94 01 00 00           	movl	0x194(%rbp), %ecx
14165ebcf: 89 08                       	movl	%ecx, (%rax)
14165ebd1: 48 8b 8d 20 03 00 00        	movq	0x320(%rbp), %rcx
14165ebd8: 48 8b 95 a8 01 00 00        	movq	0x1a8(%rbp), %rdx
14165ebdf: e8 cc d8 4c ff              	callq	0x140b2c4b0 <.text+0xb2b4b0>
14165ebe4: e9 ad 04 00 00              	jmp	0x14165f096 <.text+0x165e096>
14165ebe9: 42 8b 54 19 c8              	movl	-0x38(%rcx,%r11), %edx
14165ebee: 83 fa 01                    	cmpl	$0x1, %edx
14165ebf1: 0f 87 30 10 00 00           	ja	0x14165fc27 <.text+0x165ec27>
14165ebf7: 48 89 bd 38 03 00 00        	movq	%rdi, 0x338(%rbp)
14165ebfe: 4c 01 d9                    	addq	%r11, %rcx
14165ec01: 48 8b 41 dc                 	movq	-0x24(%rcx), %rax
14165ec05: 48 89 85 c0 02 00 00        	movq	%rax, 0x2c0(%rbp)
14165ec0c: 0f 10 41 cc                 	movups	-0x34(%rcx), %xmm0
14165ec10: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
14165ec17: 89 95 d0 02 00 00           	movl	%edx, 0x2d0(%rbp)
14165ec1d: f3 0f 6f 41 cc              	movdqu	-0x34(%rcx), %xmm0
14165ec22: f3 0f 7f 85 d4 02 00 00     	movdqu	%xmm0, 0x2d4(%rbp)
14165ec2a: 48 8b 41 dc                 	movq	-0x24(%rcx), %rax
14165ec2e: 48 89 85 e4 02 00 00        	movq	%rax, 0x2e4(%rbp)
14165ec35: 89 95 20 03 00 00           	movl	%edx, 0x320(%rbp)
14165ec3b: f6 c2 01                    	testb	$0x1, %dl
14165ec3e: 0f 85 2b 12 00 00           	jne	0x14165fe6f <.text+0x165ee6f>
14165ec44: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
14165ec4b: 8b 8d 94 01 00 00           	movl	0x194(%rbp), %ecx
14165ec51: 89 08                       	movl	%ecx, (%rax)
14165ec53: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
14165ec5a: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165ec61: e8 ba f7 3b ff              	callq	0x140a1e420 <.text+0xa1d420>
14165ec66: 48 89 c6                    	movq	%rax, %rsi
14165ec69: 48 85 c0                    	testq	%rax, %rax
14165ec6c: 0f 84 1b 02 00 00           	je	0x14165ee8d <.text+0x165de8d>
14165ec72: 4c 8b 7e 30                 	movq	0x30(%rsi), %r15
14165ec76: 4c 8b 76 48                 	movq	0x48(%rsi), %r14
14165ec7a: 66 41 0f 6f 07              	movdqa	(%r15), %xmm0
14165ec7f: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14165ec83: f7 d0                       	notl	%eax
14165ec85: 4d 8d 6f 10                 	leaq	0x10(%r15), %r13
14165ec89: 48 83 c6 60                 	addq	$0x60, %rsi
14165ec8d: 31 d2                       	xorl	%edx, %edx
14165ec8f: 4d 85 f6                    	testq	%r14, %r14
14165ec92: 0f 84 f5 01 00 00           	je	0x14165ee8d <.text+0x165de8d>
14165ec98: 66 85 c0                    	testw	%ax, %ax
14165ec9b: 74 13                       	je	0x14165ecb0 <.text+0x165dcb0>
14165ec9d: 44 8d 60 ff                 	leal	-0x1(%rax), %r12d
14165eca1: 41 21 c4                    	andl	%eax, %r12d
14165eca4: eb 35                       	jmp	0x14165ecdb <.text+0x165dcdb>
14165eca6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14165ecb0: 66 41 0f 6f 45 00           	movdqa	(%r13), %xmm0
14165ecb6: 66 0f d7 c0                 	pmovmskb	%xmm0, %eax
14165ecba: 49 81 c7 00 fb ff ff        	addq	$-0x500, %r15           # imm = 0xFB00
14165ecc1: 49 83 c5 10                 	addq	$0x10, %r13
14165ecc5: 3d ff ff 00 00              	cmpl	$0xffff, %eax           # imm = 0xFFFF
14165ecca: 74 e4                       	je	0x14165ecb0 <.text+0x165dcb0>
14165eccc: b9 fe ff ff ff              	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
14165ecd1: 29 c1                       	subl	%eax, %ecx
14165ecd3: f7 d0                       	notl	%eax
14165ecd5: 41 89 c4                    	movl	%eax, %r12d
14165ecd8: 41 21 cc                    	andl	%ecx, %r12d
14165ecdb: 49 ff ce                    	decq	%r14
14165ecde: f3 0f bc c0                 	tzcntl	%eax, %eax
14165ece2: 48 f7 d8                    	negq	%rax
14165ece5: 48 8d 3c 80                 	leaq	(%rax,%rax,4), %rdi
14165ece9: 48 c1 e7 04                 	shlq	$0x4, %rdi
14165eced: 41 0f b6 4c 3f da           	movzbl	-0x26(%r15,%rdi), %ecx
14165ecf3: 8d 41 fe                    	leal	-0x2(%rcx), %eax
14165ecf6: 48 ff c9                    	decq	%rcx
14165ecf9: 3c 03                       	cmpb	$0x3, %al
14165ecfb: 48 0f 43 ca                 	cmovaeq	%rdx, %rcx
14165ecff: 48 85 c9                    	testq	%rcx, %rcx
14165ed02: 74 0e                       	je	0x14165ed12 <.text+0x165dd12>
14165ed04: 44 89 e0                    	movl	%r12d, %eax
14165ed07: 48 83 f9 01                 	cmpq	$0x1, %rcx
14165ed0b: 74 82                       	je	0x14165ec8f <.text+0x165dc8f>
14165ed0d: e9 5d 0f 00 00              	jmp	0x14165fc6f <.text+0x165ec6f>
14165ed12: 4c 01 ff                    	addq	%r15, %rdi
14165ed15: 8b 57 b0                    	movl	-0x50(%rdi), %edx
14165ed18: 48 89 f1                    	movq	%rsi, %rcx
14165ed1b: e8 e0 a4 c6 fe              	callq	0x1402c9200 <.text+0x2c8200>
14165ed20: 80 7f da 02                 	cmpb	$0x2, -0x26(%rdi)
14165ed24: 0f 83 e0 0e 00 00           	jae	0x14165fc0a <.text+0x165ec0a>
14165ed2a: 66 0f 6e 4f cc              	movd	-0x34(%rdi), %xmm1
14165ed2f: f3 0f 10 57 d4              	movss	-0x2c(%rdi), %xmm2
14165ed34: 48 8d 8d d4 02 00 00        	leaq	0x2d4(%rbp), %rcx
14165ed3b: e8 00 8e 63 ff              	callq	0x140c97b40 <.text+0xc96b40>
14165ed40: 66 0f 7e 85 60 03 00 00     	movd	%xmm0, 0x360(%rbp)
14165ed48: 66 0f 7e 8d 64 03 00 00     	movd	%xmm1, 0x364(%rbp)
14165ed50: 66 0f 6e 57 d0              	movd	-0x30(%rdi), %xmm2
14165ed55: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165ed5c: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
14165ed63: e8 78 e0 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165ed68: 48 83 c7 cc                 	addq	$-0x34, %rdi
14165ed6c: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
14165ed72: 89 47 08                    	movl	%eax, 0x8(%rdi)
14165ed75: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
14165ed7c: 48 89 07                    	movq	%rax, (%rdi)
14165ed7f: 44 89 e0                    	movl	%r12d, %eax
14165ed82: e9 06 ff ff ff              	jmp	0x14165ec8d <.text+0x165dc8d>
14165ed87: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
14165ed8b: 49 8b 4e 10                 	movq	0x10(%r14), %rcx
14165ed8f: 89 01                       	movl	%eax, (%rcx)
14165ed91: 49 8b 3e                    	movq	(%r14), %rdi
14165ed94: 4c 8b 77 30                 	movq	0x30(%rdi), %r14
14165ed98: 4c 8b 7f 40                 	movq	0x40(%rdi), %r15
14165ed9c: 4c 3b 77 20                 	cmpq	0x20(%rdi), %r14
14165eda0: 75 10                       	jne	0x14165edb2 <.text+0x165ddb2>
14165eda2: 48 8d 4f 20                 	leaq	0x20(%rdi), %rcx
14165eda6: 48 8d 15 9b b3 5d 01        	leaq	0x15db39b(%rip), %rdx   # 0x142c3a148
14165edad: e8 be 3a f0 fe              	callq	0x140562870 <.text+0x561870>
14165edb2: 48 8b 47 28                 	movq	0x28(%rdi), %rax
14165edb6: 4e 89 3c f0                 	movq	%r15, (%rax,%r14,8)
14165edba: 49 ff c6                    	incq	%r14
14165edbd: 4c 89 77 30                 	movq	%r14, 0x30(%rdi)
14165edc1: 48 ff 47 40                 	incq	0x40(%rdi)
14165edc5: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14165edcc: 8b 8d 44 01 00 00           	movl	0x144(%rbp), %ecx
14165edd2: 89 08                       	movl	%ecx, (%rax)
14165edd4: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14165eddb: 48 8b 78 30                 	movq	0x30(%rax), %rdi
14165eddf: 4c 8b 70 40                 	movq	0x40(%rax), %r14
14165ede3: 48 3b 78 20                 	cmpq	0x20(%rax), %rdi
14165ede7: 75 17                       	jne	0x14165ee00 <.text+0x165de00>
14165ede9: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14165edf0: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165edf4: 48 8d 15 4d b3 5d 01        	leaq	0x15db34d(%rip), %rdx   # 0x142c3a148
14165edfb: e8 70 3a f0 fe              	callq	0x140562870 <.text+0x561870>
14165ee00: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
14165ee07: 48 8b 41 28                 	movq	0x28(%rcx), %rax
14165ee0b: 4c 89 34 f8                 	movq	%r14, (%rax,%rdi,8)
14165ee0f: 48 ff c7                    	incq	%rdi
14165ee12: 48 89 79 30                 	movq	%rdi, 0x30(%rcx)
14165ee16: 48 ff 41 40                 	incq	0x40(%rcx)
14165ee1a: f6 06 01                    	testb	$0x1, (%rsi)
14165ee1d: 0f 84 0c 07 00 00           	je	0x14165f52f <.text+0x165e52f>
14165ee23: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
14165ee2a: 0f 10 00                    	movups	(%rax), %xmm0
14165ee2d: 0f 11 85 d4 02 00 00        	movups	%xmm0, 0x2d4(%rbp)
14165ee34: c7 85 d0 02 00 00 00 00 00 00       	movl	$0x0, 0x2d0(%rbp)
14165ee3e: 0f 10 85 48 03 00 00        	movups	0x348(%rbp), %xmm0
14165ee45: 0f 11 85 b4 01 00 00        	movups	%xmm0, 0x1b4(%rbp)
14165ee4c: c7 85 b0 01 00 00 00 00 00 00       	movl	$0x0, 0x1b0(%rbp)
14165ee56: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
14165ee5d: 4c 8d 85 d0 02 00 00        	leaq	0x2d0(%rbp), %r8
14165ee64: 4c 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %r9
14165ee6b: 48 89 f2                    	movq	%rsi, %rdx
14165ee6e: e8 6d 3e 47 ff              	callq	0x140ad2ce0 <.text+0xad1ce0>
14165ee73: 83 bd 60 03 00 00 01        	cmpl	$0x1, 0x360(%rbp)
14165ee7a: 0f 84 56 10 00 00           	je	0x14165fed6 <.text+0x165eed6>
14165ee80: f3 0f 10 85 6c 03 00 00     	movss	0x36c(%rbp), %xmm0
14165ee88: e9 cd 06 00 00              	jmp	0x14165f55a <.text+0x165e55a>
14165ee8d: 48 c7 85 48 03 00 00 00 00 00 00    	movq	$0x0, 0x348(%rbp)
14165ee98: 48 c7 85 50 03 00 00 08 00 00 00    	movq	$0x8, 0x350(%rbp)
14165eea3: 48 c7 85 58 03 00 00 00 00 00 00    	movq	$0x0, 0x358(%rbp)
14165eeae: 48 8b 85 38 03 00 00        	movq	0x338(%rbp), %rax
14165eeb5: 0f 10 00                    	movups	(%rax), %xmm0
14165eeb8: 0f 11 85 c4 01 00 00        	movups	%xmm0, 0x1c4(%rbp)
14165eebf: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
14165eec6: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14165eecd: c7 85 c0 01 00 00 00 00 00 00       	movl	$0x0, 0x1c0(%rbp)
14165eed7: 8b 85 20 03 00 00           	movl	0x320(%rbp), %eax
14165eedd: 89 85 dc 01 00 00           	movl	%eax, 0x1dc(%rbp)
14165eee3: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
14165eeea: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
14165eef1: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
14165eef8: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
14165eeff: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
14165ef09: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165ef10: c6 85 b6 03 00 00 01        	movb	$0x1, 0x3b6(%rbp)
14165ef17: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
14165ef1e: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165ef25: e8 16 3c 42 ff              	callq	0x140a82b40 <.text+0xa81b40>
14165ef2a: 4c 8b b5 88 00 00 00        	movq	0x88(%rbp), %r14
14165ef31: 4c 8b bd 90 00 00 00        	movq	0x90(%rbp), %r15
14165ef38: 48 8b 95 d0 04 00 00        	movq	0x4d0(%rbp), %rdx
14165ef3f: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14165ef42: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14165ef46: 89 01                       	movl	%eax, (%rcx)
14165ef48: 48 8b 32                    	movq	(%rdx), %rsi
14165ef4b: 0f 10 85 48 03 00 00        	movups	0x348(%rbp), %xmm0
14165ef52: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
14165ef59: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
14165ef60: 48 8b 8d 60 03 00 00        	movq	0x360(%rbp), %rcx
14165ef67: 48 8b 95 68 03 00 00        	movq	0x368(%rbp), %rdx
14165ef6e: 48 c7 85 60 03 00 00 00 00 00 00    	movq	$0x0, 0x360(%rbp)
14165ef79: 48 c7 85 68 03 00 00 08 00 00 00    	movq	$0x8, 0x368(%rbp)
14165ef84: 48 c7 85 70 03 00 00 00 00 00 00    	movq	$0x0, 0x370(%rbp)
14165ef8f: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14165ef93: 48 8d 04 c2                 	leaq	(%rdx,%rax,8), %rax
14165ef97: 48 89 95 b0 01 00 00        	movq	%rdx, 0x1b0(%rbp)
14165ef9e: 48 89 95 b8 01 00 00        	movq	%rdx, 0x1b8(%rbp)
14165efa5: 48 89 8d c0 01 00 00        	movq	%rcx, 0x1c0(%rbp)
14165efac: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14165efb3: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165efba: e8 91 7f 03 00              	callq	0x141696f50 <.text+0x1695f50>
14165efbf: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14165efc6: 48 89 95 c0 01 00 00        	movq	%rdx, 0x1c0(%rbp)
14165efcd: 48 c7 85 b0 01 00 00 07 00 00 00    	movq	$0x7, 0x1b0(%rbp)
14165efd8: c6 85 b6 03 00 00 00        	movb	$0x0, 0x3b6(%rbp)
14165efdf: 4c 8d 05 e2 35 62 01        	leaq	0x16235e2(%rip), %r8    # 0x142c825c8
14165efe6: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165efed: 48 89 f1                    	movq	%rsi, %rcx
14165eff0: e8 cb 5e 42 ff              	callq	0x140a84ec0 <.text+0xa83ec0>
14165eff5: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14165effc: 8b 8d 4c 01 00 00           	movl	0x14c(%rbp), %ecx
14165f002: 89 08                       	movl	%ecx, (%rax)
14165f004: 49 8b 77 30                 	movq	0x30(%r15), %rsi
14165f008: 49 8b 7f 40                 	movq	0x40(%r15), %rdi
14165f00c: 49 3b 77 20                 	cmpq	0x20(%r15), %rsi
14165f010: 75 17                       	jne	0x14165f029 <.text+0x165e029>
14165f012: 49 8d 4f 20                 	leaq	0x20(%r15), %rcx
14165f016: c6 85 b6 03 00 00 00        	movb	$0x0, 0x3b6(%rbp)
14165f01d: 48 8d 15 24 b1 5d 01        	leaq	0x15db124(%rip), %rdx   # 0x142c3a148
14165f024: e8 47 38 f0 fe              	callq	0x140562870 <.text+0x561870>
14165f029: 49 8b 47 28                 	movq	0x28(%r15), %rax
14165f02d: 48 89 3c f0                 	movq	%rdi, (%rax,%rsi,8)
14165f031: 48 ff c6                    	incq	%rsi
14165f034: 49 89 77 30                 	movq	%rsi, 0x30(%r15)
14165f038: 49 ff 47 40                 	incq	0x40(%r15)
14165f03c: 48 8b 85 98 00 00 00        	movq	0x98(%rbp), %rax
14165f043: 8b 8d 48 01 00 00           	movl	0x148(%rbp), %ecx
14165f049: 89 08                       	movl	%ecx, (%rax)
14165f04b: 49 8b 76 30                 	movq	0x30(%r14), %rsi
14165f04f: 49 8b 7e 40                 	movq	0x40(%r14), %rdi
14165f053: 49 3b 76 20                 	cmpq	0x20(%r14), %rsi
14165f057: 75 17                       	jne	0x14165f070 <.text+0x165e070>
14165f059: 49 8d 4e 20                 	leaq	0x20(%r14), %rcx
14165f05d: c6 85 b6 03 00 00 00        	movb	$0x0, 0x3b6(%rbp)
14165f064: 48 8d 15 dd b0 5d 01        	leaq	0x15db0dd(%rip), %rdx   # 0x142c3a148
14165f06b: e8 c0 35 f0 fe              	callq	0x140562630 <.text+0x561630>
14165f070: 49 8b 46 28                 	movq	0x28(%r14), %rax
14165f074: 48 89 f1                    	movq	%rsi, %rcx
14165f077: 48 c1 e1 04                 	shlq	$0x4, %rcx
14165f07b: 48 89 3c 08                 	movq	%rdi, (%rax,%rcx)
14165f07f: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165f086: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14165f08b: 48 ff c6                    	incq	%rsi
14165f08e: 49 89 76 30                 	movq	%rsi, 0x30(%r14)
14165f092: 49 ff 46 40                 	incq	0x40(%r14)
14165f096: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
14165f09d: 0f 10 00                    	movups	(%rax), %xmm0
14165f0a0: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
14165f0a7: f3 0f 10 73 e0              	movss	-0x20(%rbx), %xmm6
14165f0ac: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
14165f0b3: 8b 8d 90 01 00 00           	movl	0x190(%rbp), %ecx
14165f0b9: 89 08                       	movl	%ecx, (%rax)
14165f0bb: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14165f0c2: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14165f0c6: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14165f0ca: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14165f0ce: 75 17                       	jne	0x14165f0e7 <.text+0x165e0e7>
14165f0d0: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
14165f0d7: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165f0db: 48 8d 15 66 b0 5d 01        	leaq	0x15db066(%rip), %rdx   # 0x142c3a148
14165f0e2: e8 09 39 f0 fe              	callq	0x1405629f0 <.text+0x5619f0>
14165f0e7: 48 8b 95 18 03 00 00        	movq	0x318(%rbp), %rdx
14165f0ee: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14165f0f2: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
14165f0f6: 48 c1 e1 04                 	shlq	$0x4, %rcx
14165f0fa: c7 04 08 00 00 00 00        	movl	$0x0, (%rax,%rcx)
14165f101: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
14165f108: 0f 11 44 08 04              	movups	%xmm0, 0x4(%rax,%rcx)
14165f10d: f3 0f 11 74 08 1c           	movss	%xmm6, 0x1c(%rax,%rcx)
14165f113: c7 44 08 20 00 00 c0 3f     	movl	$0x3fc00000, 0x20(%rax,%rcx) # imm = 0x3FC00000
14165f11b: c6 44 08 24 01              	movb	$0x1, 0x24(%rax,%rcx)
14165f120: 48 89 7c 08 28              	movq	%rdi, 0x28(%rax,%rcx)
14165f125: 48 ff c6                    	incq	%rsi
14165f128: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
14165f12c: 48 ff 42 40                 	incq	0x40(%rdx)
14165f130: 48 8b 95 90 03 00 00        	movq	0x390(%rbp), %rdx
14165f137: f6 85 88 03 00 00 01        	testb	$0x1, 0x388(%rbp)
14165f13e: 0f 84 6d 06 00 00           	je	0x14165f7b1 <.text+0x165e7b1>
14165f144: f3 0f 10 4b bc              	movss	-0x44(%rbx), %xmm1
14165f149: f3 0f 10 53 c0              	movss	-0x40(%rbx), %xmm2
14165f14e: f3 0f 10 5b c4              	movss	-0x3c(%rbx), %xmm3
14165f153: f3 0f 58 1d 15 67 2c 01     	addss	0x12c6715(%rip), %xmm3  # 0x142925870
14165f15b: f3 0f 10 43 c8              	movss	-0x38(%rbx), %xmm0
14165f160: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
14165f166: 48 8d 75 30                 	leaq	0x30(%rbp), %rsi
14165f16a: 48 89 f1                    	movq	%rsi, %rcx
14165f16d: e8 ee 83 63 ff              	callq	0x140c97560 <.text+0xc96560>
14165f172: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165f179: f3 0f 10 70 08              	movss	0x8(%rax), %xmm6
14165f17e: 0f 28 45 30                 	movaps	0x30(%rbp), %xmm0
14165f182: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
14165f189: c7 44 24 20 00 00 a0 40     	movl	$0x40a00000, 0x20(%rsp) # imm = 0x40A00000
14165f191: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
14165f198: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
14165f19f: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165f1a6: 0f 28 de                    	movaps	%xmm6, %xmm3
14165f1a9: e8 d2 16 00 00              	callq	0x141660880 <.text+0x165f880>
14165f1ae: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165f1b5: f3 0f 10 08                 	movss	(%rax), %xmm1
14165f1b9: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
14165f1be: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165f1c5: e8 06 1e 63 ff              	callq	0x140c90fd0 <.text+0xc8ffd0>
14165f1ca: 0f 28 bd b0 01 00 00        	movaps	0x1b0(%rbp), %xmm7
14165f1d1: 0f 28 45 30                 	movaps	0x30(%rbp), %xmm0
14165f1d5: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
14165f1dc: f2 0f 10 08                 	movsd	(%rax), %xmm1
14165f1e0: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165f1e7: f2 0f 10 10                 	movsd	(%rax), %xmm2
14165f1eb: 0f 15 c0                    	unpckhps	%xmm0, %xmm0            # xmm0 = xmm0[2,2,3,3]
14165f1ee: 0f 59 c2                    	mulps	%xmm2, %xmm0
14165f1f1: 0f 58 c1                    	addps	%xmm1, %xmm0
14165f1f4: 0f 13 85 d0 02 00 00        	movlps	%xmm0, 0x2d0(%rbp)
14165f1fb: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165f202: 48 8d 9d d0 02 00 00        	leaq	0x2d0(%rbp), %rbx
14165f209: 48 89 da                    	movq	%rbx, %rdx
14165f20c: 0f 28 d6                    	movaps	%xmm6, %xmm2
14165f20f: e8 cc db 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165f214: 0f 28 cf                    	movaps	%xmm7, %xmm1
14165f217: 0f 59 cf                    	mulps	%xmm7, %xmm1
14165f21a: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
14165f21e: f3 0f 58 c1                 	addss	%xmm1, %xmm0
14165f222: 0f 28 d1                    	movaps	%xmm1, %xmm2
14165f225: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
14165f229: f3 0f 58 d0                 	addss	%xmm0, %xmm2
14165f22d: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
14165f231: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
14165f235: 0f 5c ca                    	subps	%xmm2, %xmm1
14165f238: 0f 28 05 a1 9f 42 01        	movaps	0x1429fa1(%rip), %xmm0  # 0x142a891e0
14165f23f: 0f 59 c8                    	mulps	%xmm0, %xmm1
14165f242: 0f 59 c7                    	mulps	%xmm7, %xmm0
14165f245: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
14165f249: f3 0f 58 d0                 	addss	%xmm0, %xmm2
14165f24d: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
14165f250: f3 0f 58 c2                 	addss	%xmm2, %xmm0
14165f254: f3 0f 58 c0                 	addss	%xmm0, %xmm0
14165f258: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14165f25c: 0f 59 c7                    	mulps	%xmm7, %xmm0
14165f25f: 0f 58 c1                    	addps	%xmm1, %xmm0
14165f262: f3 0f 10 0d fe dd 36 01     	movss	0x136ddfe(%rip), %xmm1  # 0x1429cd068
14165f26a: 0f 28 d7                    	movaps	%xmm7, %xmm2
14165f26d: 0f 59 d1                    	mulps	%xmm1, %xmm2
14165f270: 0f 28 df                    	movaps	%xmm7, %xmm3
14165f273: 0f c6 df c9                 	shufps	$0xc9, %xmm7, %xmm3     # xmm3 = xmm3[1,2],xmm7[0,3]
14165f277: 0f 59 d9                    	mulps	%xmm1, %xmm3
14165f27a: 0f c6 d2 d2                 	shufps	$0xd2, %xmm2, %xmm2     # xmm2 = xmm2[2,0,1,3]
14165f27e: 0f 5c da                    	subps	%xmm2, %xmm3
14165f281: 0f 28 cf                    	movaps	%xmm7, %xmm1
14165f284: 0f 58 cf                    	addps	%xmm7, %xmm1
14165f287: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
14165f28b: 0f 59 cb                    	mulps	%xmm3, %xmm1
14165f28e: 0f 58 c8                    	addps	%xmm0, %xmm1
14165f291: 0f 28 c1                    	movaps	%xmm1, %xmm0
14165f294: 66 0f 15 c1                 	unpckhpd	%xmm1, %xmm0            # xmm0 = xmm0[1],xmm1[1]
14165f298: f3 0f 58 85 b8 01 00 00     	addss	0x1b8(%rbp), %xmm0
14165f2a0: f2 0f 10 95 b0 01 00 00     	movsd	0x1b0(%rbp), %xmm2
14165f2a8: 0f 58 d1                    	addps	%xmm1, %xmm2
14165f2ab: 0f 13 95 60 03 00 00        	movlps	%xmm2, 0x360(%rbp)
14165f2b2: f3 0f 11 85 68 03 00 00     	movss	%xmm0, 0x368(%rbp)
14165f2ba: 48 8d 45 48                 	leaq	0x48(%rbp), %rax
14165f2be: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
14165f2c5: 48 8d 05 54 1b 00 00        	leaq	0x1b54(%rip), %rax      # 0x141660e20 <.text+0x165fe20>
14165f2cc: 48 89 85 d8 02 00 00        	movq	%rax, 0x2d8(%rbp)
14165f2d3: 48 89 b5 e0 02 00 00        	movq	%rsi, 0x2e0(%rbp)
14165f2da: 48 8d 05 5f ca ff ff        	leaq	-0x35a1(%rip), %rax     # 0x14165bd40 <.text+0x165ad40>
14165f2e1: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
14165f2e8: 48 8d 85 60 03 00 00        	leaq	0x360(%rbp), %rax
14165f2ef: 48 89 85 f0 02 00 00        	movq	%rax, 0x2f0(%rbp)
14165f2f6: 48 8d 05 03 c7 6e ff        	leaq	-0x9138fd(%rip), %rax   # 0x140d4ba00 <.text+0xd4aa00>
14165f2fd: 48 89 85 f8 02 00 00        	movq	%rax, 0x2f8(%rbp)
14165f304: 48 8d 05 1d 31 62 01        	leaq	0x162311d(%rip), %rax   # 0x142c82428
14165f30b: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165f312: 48 c7 85 b8 01 00 00 04 00 00 00    	movq	$0x4, 0x1b8(%rbp)
14165f31d: 48 89 9d c0 01 00 00        	movq	%rbx, 0x1c0(%rbp)
14165f324: 48 c7 85 c8 01 00 00 03 00 00 00    	movq	$0x3, 0x1c8(%rbp)
14165f32f: 48 c7 85 d0 01 00 00 00 00 00 00    	movq	$0x0, 0x1d0(%rbp)
14165f33a: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
14165f33e: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165f345: e8 46 ac 71 ff              	callq	0x140d79f90 <.text+0xd78f90>
14165f34a: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14165f34e: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
14165f355: 4c 8b 6d 20                 	movq	0x20(%rbp), %r13
14165f359: 48 8b 75 28                 	movq	0x28(%rbp), %rsi
14165f35d: 8b 85 68 03 00 00           	movl	0x368(%rbp), %eax
14165f363: 89 85 b8 01 00 00           	movl	%eax, 0x1b8(%rbp)
14165f369: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
14165f370: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165f377: 0f b6 05 8b 58 fd 01        	movzbl	0x1fd588b(%rip), %eax   # 0x143634c09
14165f37e: b9 30 00 00 00              	movl	$0x30, %ecx
14165f383: ba 10 00 00 00              	movl	$0x10, %edx
14165f388: e8 83 48 fb fe              	callq	0x140613c10 <.text+0x612c10>
14165f38d: 48 85 c0                    	testq	%rax, %rax
14165f390: 4c 89 ad 08 03 00 00        	movq	%r13, 0x308(%rbp)
14165f397: 0f 84 9f 09 00 00           	je	0x14165fd3c <.text+0x165ed3c>
14165f39d: 49 89 c6                    	movq	%rax, %r14
14165f3a0: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14165f3a7: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14165f3af: 0f 29 78 10                 	movaps	%xmm7, 0x10(%rax)
14165f3b3: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
14165f3ba: 49 89 46 20                 	movq	%rax, 0x20(%r14)
14165f3be: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
14165f3c4: 41 89 46 28                 	movl	%eax, 0x28(%r14)
14165f3c8: 4c 89 b5 b0 01 00 00        	movq	%r14, 0x1b0(%rbp)
14165f3cf: 48 8d 3d 92 30 62 01        	leaq	0x1623092(%rip), %rdi   # 0x142c82468
14165f3d6: 48 89 bd b8 01 00 00        	movq	%rdi, 0x1b8(%rbp)
14165f3dd: 48 85 f6                    	testq	%rsi, %rsi
14165f3e0: 0f 88 7e 09 00 00           	js	0x14165fd64 <.text+0x165ed64>
14165f3e6: 4c 8b bd a0 01 00 00        	movq	0x1a0(%rbp), %r15
14165f3ed: 4c 8b a5 90 03 00 00        	movq	0x390(%rbp), %r12
14165f3f4: 74 36                       	je	0x14165f42c <.text+0x165e42c>
14165f3f6: 4c 89 b5 10 03 00 00        	movq	%r14, 0x310(%rbp)
14165f3fd: 0f b6 05 05 58 fd 01        	movzbl	0x1fd5805(%rip), %eax   # 0x143634c09
14165f404: 41 be 01 00 00 00           	movl	$0x1, %r14d
14165f40a: ba 01 00 00 00              	movl	$0x1, %edx
14165f40f: 48 89 f1                    	movq	%rsi, %rcx
14165f412: e8 f9 47 fb fe              	callq	0x140613c10 <.text+0x612c10>
14165f417: 48 85 c0                    	testq	%rax, %rax
14165f41a: 0f 84 4e 09 00 00           	je	0x14165fd6e <.text+0x165ed6e>
14165f420: 48 89 c3                    	movq	%rax, %rbx
14165f423: 4c 8b b5 10 03 00 00        	movq	0x310(%rbp), %r14
14165f42a: eb 05                       	jmp	0x14165f431 <.text+0x165e431>
14165f42c: bb 01 00 00 00              	movl	$0x1, %ebx
14165f431: 48 89 d9                    	movq	%rbx, %rcx
14165f434: 4c 89 ea                    	movq	%r13, %rdx
14165f437: 49 89 f0                    	movq	%rsi, %r8
14165f43a: e8 c1 47 2c 01              	callq	0x142923c00 <.text+0x2922c00>
14165f43f: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
14165f446: 48 89 9d b8 01 00 00        	movq	%rbx, 0x1b8(%rbp)
14165f44d: 48 89 b5 c0 01 00 00        	movq	%rsi, 0x1c0(%rbp)
14165f454: 4c 89 b5 c8 01 00 00        	movq	%r14, 0x1c8(%rbp)
14165f45b: 48 89 bd d0 01 00 00        	movq	%rdi, 0x1d0(%rbp)
14165f462: 48 8d 95 b0 01 00 00        	leaq	0x1b0(%rbp), %rdx
14165f469: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
14165f470: e8 0b d4 e0 ff              	callq	0x14146c880 <.text+0x146b880>
14165f475: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
14165f47c: 48 85 d2                    	testq	%rdx, %rdx
14165f47f: 8b b5 88 03 00 00           	movl	0x388(%rbp), %esi
14165f485: 0f 84 67 04 00 00           	je	0x14165f8f2 <.text+0x165e8f2>
14165f48b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14165f491: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
14165f498: e8 83 47 fb fe              	callq	0x140613c20 <.text+0x612c20>
14165f49d: e9 50 04 00 00              	jmp	0x14165f8f2 <.text+0x165e8f2>
14165f4a2: 48 8d 0d c7 30 62 01        	leaq	0x16230c7(%rip), %rcx   # 0x142c82570
14165f4a9: e8 62 9e 27 01              	callq	0x1428d9310 <.text+0x28d8310>
14165f4ae: e9 64 0a 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165f4b3: f6 83 68 fe ff ff 01        	testb	$0x1, -0x198(%rbx)
14165f4ba: 0f 84 cc 09 00 00           	je	0x14165fe8c <.text+0x165ee8c>
14165f4c0: f3 0f 10 40 4c              	movss	0x4c(%rax), %xmm0
14165f4c5: f3 0f 10 48 54              	movss	0x54(%rax), %xmm1
14165f4ca: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
14165f4cd: 0f 5c c6                    	subps	%xmm6, %xmm0
14165f4d0: 0f 28 c8                    	movaps	%xmm0, %xmm1
14165f4d3: 0f 59 c8                    	mulps	%xmm0, %xmm1
14165f4d6: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14165f4da: f3 0f 58 d1                 	addss	%xmm1, %xmm2
14165f4de: 0f 57 c9                    	xorps	%xmm1, %xmm1
14165f4e1: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
14165f4e5: f3 0f 10 15 17 64 2c 01     	movss	0x12c6417(%rip), %xmm2  # 0x142925904
14165f4ed: f3 0f 5e d1                 	divss	%xmm1, %xmm2
14165f4f1: f3 44 0f 12 d2              	movsldup	%xmm2, %xmm10           # xmm10 = xmm2[0,0,2,2]
14165f4f6: f3 44 0f 10 4b e4           	movss	-0x1c(%rbx), %xmm9
14165f4fc: f3 44 0f 59 0d e3 ab 2f 01  	mulss	0x12fabe3(%rip), %xmm9  # 0x14295a0e8
14165f505: f3 44 0f 58 8b 6c fe ff ff  	addss	-0x194(%rbx), %xmm9
14165f50e: 44 0f 59 d0                 	mulps	%xmm0, %xmm10
14165f512: 41 b0 01                    	movb	$0x1, %r8b
14165f515: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
14165f51c: 48 89 85 a8 03 00 00        	movq	%rax, 0x3a8(%rbp)
14165f523: 48 89 95 28 03 00 00        	movq	%rdx, 0x328(%rbp)
14165f52a: e9 29 ed ff ff              	jmp	0x14165e258 <.text+0x165d258>
14165f52f: f3 0f 10 46 0c              	movss	0xc(%rsi), %xmm0
14165f534: f2 0f 10 4e 04              	movsd	0x4(%rsi), %xmm1
14165f539: f2 0f 10 95 48 03 00 00     	movsd	0x348(%rbp), %xmm2
14165f541: 0f 5c ca                    	subps	%xmm2, %xmm1
14165f544: 0f 59 c9                    	mulps	%xmm1, %xmm1
14165f547: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
14165f54b: f3 0f 58 d1                 	addss	%xmm1, %xmm2
14165f54f: 0f 57 c9                    	xorps	%xmm1, %xmm1
14165f552: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
14165f556: f3 0f 5c c1                 	subss	%xmm1, %xmm0
14165f55a: f3 0f 11 85 50 03 00 00     	movss	%xmm0, 0x350(%rbp)
14165f562: f3 0f 5c c6                 	subss	%xmm6, %xmm0
14165f566: 0f 54 05 13 f4 2c 01        	andps	0x12cf413(%rip), %xmm0  # 0x14292e980
14165f56d: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
14165f574: f3 0f 10 48 28              	movss	0x28(%rax), %xmm1
14165f579: f3 0f 59 0d ef 62 2c 01     	mulss	0x12c62ef(%rip), %xmm1  # 0x142925870
14165f581: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
14165f584: 0f 86 13 02 00 00           	jbe	0x14165f79d <.text+0x165e79d>
14165f58a: 48 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %rax
14165f591: 8b 78 1c                    	movl	0x1c(%rax), %edi
14165f594: 4c 8b 70 10                 	movq	0x10(%rax), %r14
14165f598: 41 89 3e                    	movl	%edi, (%r14)
14165f59b: 48 8b 30                    	movq	(%rax), %rsi
14165f59e: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
14165f5a5: 48 8d 95 48 03 00 00        	leaq	0x348(%rbp), %rdx
14165f5ac: f3 0f 10 15 bc 62 2c 01     	movss	0x12c62bc(%rip), %xmm2  # 0x142925870
14165f5b4: e8 37 87 63 ff              	callq	0x140c97cf0 <.text+0xc96cf0>
14165f5b9: 48 8b 85 00 03 00 00        	movq	0x300(%rbp), %rax
14165f5c0: 48 89 85 e0 01 00 00        	movq	%rax, 0x1e0(%rbp)
14165f5c7: 0f 10 85 d0 02 00 00        	movups	0x2d0(%rbp), %xmm0
14165f5ce: 0f 10 8d e0 02 00 00        	movups	0x2e0(%rbp), %xmm1
14165f5d5: 0f 10 95 f0 02 00 00        	movups	0x2f0(%rbp), %xmm2
14165f5dc: 0f 29 95 d0 01 00 00        	movaps	%xmm2, 0x1d0(%rbp)
14165f5e3: 0f 29 8d c0 01 00 00        	movaps	%xmm1, 0x1c0(%rbp)
14165f5ea: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
14165f5f1: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
14165f5f8: 48 89 f1                    	movq	%rsi, %rcx
14165f5fb: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165f602: e8 89 aa 44 ff              	callq	0x140aaa090 <.text+0xaa9090>
14165f607: 41 89 3e                    	movl	%edi, (%r14)
14165f60a: 0f 10 85 48 03 00 00        	movups	0x348(%rbp), %xmm0
14165f611: 0f 11 85 b4 01 00 00        	movups	%xmm0, 0x1b4(%rbp)
14165f618: c7 85 b0 01 00 00 00 00 00 00       	movl	$0x0, 0x1b0(%rbp)
14165f622: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
14165f629: 48 89 f1                    	movq	%rsi, %rcx
14165f62c: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165f633: e8 d8 9f 44 ff              	callq	0x140aa9610 <.text+0xaa8610>
14165f638: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165f63f: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
14165f644: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
14165f64b: 48 8b 95 90 03 00 00        	movq	0x390(%rbp), %rdx
14165f652: e8 89 d7 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165f657: 48 8b 45 78                 	movq	0x78(%rbp), %rax
14165f65b: 8b 8d 40 01 00 00           	movl	0x140(%rbp), %ecx
14165f661: 89 08                       	movl	%ecx, (%rax)
14165f663: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14165f66a: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14165f66e: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14165f672: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14165f676: 75 17                       	jne	0x14165f68f <.text+0x165e68f>
14165f678: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14165f67f: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165f683: 48 8d 15 be aa 5d 01        	leaq	0x15daabe(%rip), %rdx   # 0x142c3a148
14165f68a: e8 a1 35 f0 fe              	callq	0x140562c30 <.text+0x561c30>
14165f68f: 4c 8b 85 68 01 00 00        	movq	0x168(%rbp), %r8
14165f696: 49 8b 40 28                 	movq	0x28(%r8), %rax
14165f69a: 48 89 f1                    	movq	%rsi, %rcx
14165f69d: 48 c1 e1 05                 	shlq	$0x5, %rcx
14165f6a1: 48 89 3c 08                 	movq	%rdi, (%rax,%rcx)
14165f6a5: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165f6ac: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
14165f6b1: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
14165f6b8: 48 89 54 08 10              	movq	%rdx, 0x10(%rax,%rcx)
14165f6bd: 8b 95 18 01 00 00           	movl	0x118(%rbp), %edx
14165f6c3: 89 54 08 18                 	movl	%edx, 0x18(%rax,%rcx)
14165f6c7: 66 c7 44 08 1c 04 00        	movw	$0x4, 0x1c(%rax,%rcx)
14165f6ce: 48 ff c6                    	incq	%rsi
14165f6d1: 49 89 70 30                 	movq	%rsi, 0x30(%r8)
14165f6d5: 49 ff 40 40                 	incq	0x40(%r8)
14165f6d9: f3 0f 5c b5 50 03 00 00     	subss	0x350(%rbp), %xmm6
14165f6e1: 0f 54 35 98 f2 2c 01        	andps	0x12cf298(%rip), %xmm6  # 0x14292e980
14165f6e8: 48 8b 45 68                 	movq	0x68(%rbp), %rax
14165f6ec: f3 0f 5e 70 58              	divss	0x58(%rax), %xmm6
14165f6f1: 0f 57 ff                    	xorps	%xmm7, %xmm7
14165f6f4: f3 0f 10 0d 34 03 42 01     	movss	0x1420334(%rip), %xmm1  # 0x142a7fa30
14165f6fc: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165f6ff: 0f 28 d6                    	movaps	%xmm6, %xmm2
14165f702: e8 99 0d 63 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14165f707: 0f 28 f0                    	movaps	%xmm0, %xmm6
14165f70a: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14165f70e: 8b 8d 3c 01 00 00           	movl	0x13c(%rbp), %ecx
14165f714: 89 08                       	movl	%ecx, (%rax)
14165f716: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14165f71d: 48 8b 70 30                 	movq	0x30(%rax), %rsi
14165f721: 48 8b 78 40                 	movq	0x40(%rax), %rdi
14165f725: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
14165f729: 75 17                       	jne	0x14165f742 <.text+0x165e742>
14165f72b: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14165f732: 48 8d 48 20                 	leaq	0x20(%rax), %rcx
14165f736: 48 8d 15 0b aa 5d 01        	leaq	0x15daa0b(%rip), %rdx   # 0x142c3a148
14165f73d: e8 6e 33 f0 fe              	callq	0x140562ab0 <.text+0x561ab0>
14165f742: f3 0f 5f fe                 	maxss	%xmm6, %xmm7
14165f746: f3 0f 10 05 b6 61 2c 01     	movss	0x12c61b6(%rip), %xmm0  # 0x142925904
14165f74e: f3 0f 5d c7                 	minss	%xmm7, %xmm0
14165f752: 4c 8b 85 60 01 00 00        	movq	0x160(%rbp), %r8
14165f759: 49 8b 40 28                 	movq	0x28(%r8), %rax
14165f75d: 48 8d 0c b6                 	leaq	(%rsi,%rsi,4), %rcx
14165f761: 48 89 3c c8                 	movq	%rdi, (%rax,%rcx,8)
14165f765: 48 8b 95 a8 03 00 00        	movq	0x3a8(%rbp), %rdx
14165f76c: 48 89 54 c8 08              	movq	%rdx, 0x8(%rax,%rcx,8)
14165f771: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
14165f778: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
14165f77d: 8b 95 18 01 00 00           	movl	0x118(%rbp), %edx
14165f783: 89 54 c8 18                 	movl	%edx, 0x18(%rax,%rcx,8)
14165f787: f3 0f 11 44 c8 1c           	movss	%xmm0, 0x1c(%rax,%rcx,8)
14165f78d: c6 44 c8 20 00              	movb	$0x0, 0x20(%rax,%rcx,8)
14165f792: 48 ff c6                    	incq	%rsi
14165f795: 49 89 70 30                 	movq	%rsi, 0x30(%r8)
14165f799: 49 ff 40 40                 	incq	0x40(%r8)
14165f79d: 48 8b 95 90 03 00 00        	movq	0x390(%rbp), %rdx
14165f7a4: f6 85 88 03 00 00 01        	testb	$0x1, 0x388(%rbp)
14165f7ab: 0f 85 93 f9 ff ff           	jne	0x14165f144 <.text+0x165e144>
14165f7b1: 0f 10 02                    	movups	(%rdx), %xmm0
14165f7b4: 0f 29 85 b0 01 00 00        	movaps	%xmm0, 0x1b0(%rbp)
14165f7bb: f2 0f 10 43 bc              	movsd	-0x44(%rbx), %xmm0
14165f7c0: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165f7c7: f2 0f 10 08                 	movsd	(%rax), %xmm1
14165f7cb: f3 0f 10 53 c4              	movss	-0x3c(%rbx), %xmm2
14165f7d0: f3 0f 12 d2                 	movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
14165f7d4: 0f 59 d1                    	mulps	%xmm1, %xmm2
14165f7d7: 0f 58 d0                    	addps	%xmm0, %xmm2
14165f7da: 0f 13 55 40                 	movlps	%xmm2, 0x40(%rbp)
14165f7de: f3 0f 10 50 08              	movss	0x8(%rax), %xmm2
14165f7e3: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
14165f7ea: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
14165f7ee: e8 ed d5 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165f7f3: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
14165f7fa: 48 8b 95 a0 03 00 00        	movq	0x3a0(%rbp), %rdx
14165f801: e8 ba d5 63 ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
14165f806: b0 02                       	movb	$0x2, %al
14165f808: 4c 8b bd a0 01 00 00        	movq	0x1a0(%rbp), %r15
14165f80f: 41 80 7f 18 02              	cmpb	$0x2, 0x18(%r15)
14165f814: 73 16                       	jae	0x14165f82c <.text+0x165e82c>
14165f816: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
14165f81d: 4c 8b 85 08 03 00 00        	movq	0x308(%rbp), %r8
14165f824: 8b b5 88 03 00 00           	movl	0x388(%rbp), %esi
14165f82a: eb 26                       	jmp	0x14165f852 <.text+0x165e852>
14165f82c: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
14165f833: 49 39 4f 38                 	cmpq	%rcx, 0x38(%r15)
14165f837: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
14165f83e: 4c 8b 85 08 03 00 00        	movq	0x308(%rbp), %r8
14165f845: 8b b5 88 03 00 00           	movl	0x388(%rbp), %esi
14165f84b: 75 05                       	jne	0x14165f852 <.text+0x165e852>
14165f84d: 41 0f b6 47 19              	movzbl	0x19(%r15), %eax
14165f852: 0f 28 85 b0 01 00 00        	movaps	0x1b0(%rbp), %xmm0
14165f859: 0f 29 85 d0 02 00 00        	movaps	%xmm0, 0x2d0(%rbp)
14165f860: 48 8b 8d 48 03 00 00        	movq	0x348(%rbp), %rcx
14165f867: 48 89 8d b0 02 00 00        	movq	%rcx, 0x2b0(%rbp)
14165f86e: 8b 8d 50 03 00 00           	movl	0x350(%rbp), %ecx
14165f874: 89 8d b8 02 00 00           	movl	%ecx, 0x2b8(%rbp)
14165f87a: 48 8b 8d 60 03 00 00        	movq	0x360(%rbp), %rcx
14165f881: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
14165f888: 8b 8d 68 03 00 00           	movl	0x368(%rbp), %ecx
14165f88e: 89 8d c0 00 00 00           	movl	%ecx, 0xc0(%rbp)
14165f894: 8b 8d 10 03 00 00           	movl	0x310(%rbp), %ecx
14165f89a: 41 89 08                    	movl	%ecx, (%r8)
14165f89d: 3c 02                       	cmpb	$0x2, %al
14165f89f: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
14165f8a6: 48 89 02                    	movq	%rax, (%rdx)
14165f8a9: 48 8b 85 a8 03 00 00        	movq	0x3a8(%rbp), %rax
14165f8b0: 48 89 42 08                 	movq	%rax, 0x8(%rdx)
14165f8b4: 0f 28 85 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm0
14165f8bb: 0f 11 42 10                 	movups	%xmm0, 0x10(%rdx)
14165f8bf: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
14165f8c6: 48 89 42 20                 	movq	%rax, 0x20(%rdx)
14165f8ca: 8b 85 b8 02 00 00           	movl	0x2b8(%rbp), %eax
14165f8d0: 89 42 28                    	movl	%eax, 0x28(%rdx)
14165f8d3: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
14165f8da: 48 89 42 2c                 	movq	%rax, 0x2c(%rdx)
14165f8de: 8b 85 c0 00 00 00           	movl	0xc0(%rbp), %eax
14165f8e4: 89 42 34                    	movl	%eax, 0x34(%rdx)
14165f8e7: 0f 95 42 38                 	setne	0x38(%rdx)
14165f8eb: 4c 8b a5 90 03 00 00        	movq	0x390(%rbp), %r12
14165f8f2: b0 02                       	movb	$0x2, %al
14165f8f4: 41 80 7f 18 02              	cmpb	$0x2, 0x18(%r15)
14165f8f9: 72 12                       	jb	0x14165f90d <.text+0x165e90d>
14165f8fb: 48 8b 8d 28 03 00 00        	movq	0x328(%rbp), %rcx
14165f902: 49 39 4f 38                 	cmpq	%rcx, 0x38(%r15)
14165f906: 75 05                       	jne	0x14165f90d <.text+0x165e90d>
14165f908: 41 0f b6 47 19              	movzbl	0x19(%r15), %eax
14165f90d: 04 fe                       	addb	$-0x2, %al
14165f90f: 0f b6 c0                    	movzbl	%al, %eax
14165f912: 3c 03                       	cmpb	$0x3, %al
14165f914: b9 03 00 00 00              	movl	$0x3, %ecx
14165f919: 0f 42 c8                    	cmovbl	%eax, %ecx
14165f91c: fe c9                       	decb	%cl
14165f91e: 80 f9 02                    	cmpb	$0x2, %cl
14165f921: 0f 83 3f 01 00 00           	jae	0x14165fa66 <.text+0x165ea66>
14165f927: 40 f6 c6 01                 	testb	$0x1, %sil
14165f92b: 75 05                       	jne	0x14165f932 <.text+0x165e932>
14165f92d: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165f930: eb 08                       	jmp	0x14165f93a <.text+0x165e93a>
14165f932: f3 0f 10 05 36 5f 2c 01     	movss	0x12c5f36(%rip), %xmm0  # 0x142925870
14165f93a: 48 8b 85 a0 03 00 00        	movq	0x3a0(%rbp), %rax
14165f941: 66 0f 6e 70 08              	movd	0x8(%rax), %xmm6
14165f946: f3 0f 10 0d 32 cf 41 01     	movss	0x141cf32(%rip), %xmm1  # 0x142a7c880
14165f94e: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14165f952: f2 0f 10 10                 	movsd	(%rax), %xmm2
14165f956: f2 41 0f 10 1c 24           	movsd	(%r12), %xmm3
14165f95c: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14165f960: 0f 59 ca                    	mulps	%xmm2, %xmm1
14165f963: 0f 58 cb                    	addps	%xmm3, %xmm1
14165f966: 0f 13 8d 48 03 00 00        	movlps	%xmm1, 0x348(%rbp)
14165f96d: f3 0f 58 05 8f 4c 2d 01     	addss	0x12d4c8f(%rip), %xmm0  # 0x142934604
14165f975: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14165f979: 0f 59 c2                    	mulps	%xmm2, %xmm0
14165f97c: 0f 58 c3                    	addps	%xmm3, %xmm0
14165f97f: 0f 13 85 60 03 00 00        	movlps	%xmm0, 0x360(%rbp)
14165f986: 0f b6 05 7c 52 fd 01        	movzbl	0x1fd527c(%rip), %eax   # 0x143634c09
14165f98d: b9 18 00 00 00              	movl	$0x18, %ecx
14165f992: ba 04 00 00 00              	movl	$0x4, %edx
14165f997: e8 74 42 fb fe              	callq	0x140613c10 <.text+0x612c10>
14165f99c: 48 89 85 a0 03 00 00        	movq	%rax, 0x3a0(%rbp)
14165f9a3: 48 85 c0                    	testq	%rax, %rax
14165f9a6: 0f 84 a4 03 00 00           	je	0x14165fd50 <.text+0x165ed50>
14165f9ac: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
14165f9b3: 48 8d 95 48 03 00 00        	leaq	0x348(%rbp), %rdx
14165f9ba: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
14165f9be: e8 1d d4 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165f9c3: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165f9ca: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
14165f9d1: 66 0f 6f d6                 	movdqa	%xmm6, %xmm2
14165f9d5: e8 06 d4 63 ff              	callq	0x140c9cde0 <.text+0xc9bde0>
14165f9da: 8b 85 d8 02 00 00           	movl	0x2d8(%rbp), %eax
14165f9e0: 48 8b 8d a0 03 00 00        	movq	0x3a0(%rbp), %rcx
14165f9e7: 89 41 08                    	movl	%eax, 0x8(%rcx)
14165f9ea: 48 8b 85 d0 02 00 00        	movq	0x2d0(%rbp), %rax
14165f9f1: 48 89 01                    	movq	%rax, (%rcx)
14165f9f4: 8b 85 b8 01 00 00           	movl	0x1b8(%rbp), %eax
14165f9fa: 89 41 14                    	movl	%eax, 0x14(%rcx)
14165f9fd: 48 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %rax
14165fa04: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
14165fa08: 48 c7 85 b0 01 00 00 02 00 00 00    	movq	$0x2, 0x1b0(%rbp)
14165fa13: 48 89 8d b8 01 00 00        	movq	%rcx, 0x1b8(%rbp)
14165fa1a: 48 c7 85 c0 01 00 00 02 00 00 00    	movq	$0x2, 0x1c0(%rbp)
14165fa25: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14165fa2f: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
14165fa36: c7 85 e0 01 00 00 00 00 00 00       	movl	$0x0, 0x1e0(%rbp)
14165fa40: c7 85 f0 01 00 00 00 00 00 40       	movl	$0x40000000, 0x1f0(%rbp) # imm = 0x40000000
14165fa4a: 66 c7 85 f4 01 00 00 01 00  	movw	$0x1, 0x1f4(%rbp)
14165fa53: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165fa5a: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14165fa61: e8 0a 55 4a ff              	callq	0x140b04f70 <.text+0xb03f70>
14165fa66: 80 bd 98 01 00 00 00        	cmpb	$0x0, 0x198(%rbp)
14165fa6d: 74 47                       	je	0x14165fab6 <.text+0x165eab6>
14165fa6f: e8 bc 86 5b ff              	callq	0x140c18130 <.text+0xc17130>
14165fa74: 48 89 c1                    	movq	%rax, %rcx
14165fa77: 48 8b 00                    	movq	(%rax), %rax
14165fa7a: 48 83 f8 01                 	cmpq	$0x1, %rax
14165fa7e: 75 78                       	jne	0x14165faf8 <.text+0x165eaf8>
14165fa80: 48 83 c1 08                 	addq	$0x8, %rcx
14165fa84: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
14165fa8b: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14165fa8f: 0f 85 12 01 00 00           	jne	0x14165fba7 <.text+0x165eba7>
14165fa95: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14165fa9c: 48 89 8d 98 01 00 00        	movq	%rcx, 0x198(%rbp)
14165faa3: 48 83 c1 08                 	addq	$0x8, %rcx
14165faa7: e8 44 76 5b ff              	callq	0x140c170f0 <.text+0xc160f0>
14165faac: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
14165fab3: 48 ff 00                    	incq	(%rax)
14165fab6: 0f 28 b5 c0 03 00 00        	movaps	0x3c0(%rbp), %xmm6
14165fabd: 0f 28 bd d0 03 00 00        	movaps	0x3d0(%rbp), %xmm7
14165fac4: 44 0f 28 85 e0 03 00 00     	movaps	0x3e0(%rbp), %xmm8
14165facc: 44 0f 28 8d f0 03 00 00     	movaps	0x3f0(%rbp), %xmm9
14165fad4: 44 0f 28 95 00 04 00 00     	movaps	0x400(%rbp), %xmm10
14165fadc: 44 0f 28 9d 10 04 00 00     	movaps	0x410(%rbp), %xmm11
14165fae4: 48 81 c4 a8 04 00 00        	addq	$0x4a8, %rsp            # imm = 0x4A8
14165faeb: 5b                          	popq	%rbx
14165faec: 5f                          	popq	%rdi
14165faed: 5e                          	popq	%rsi
14165faee: 41 5c                       	popq	%r12
14165faf0: 41 5d                       	popq	%r13
14165faf2: 41 5e                       	popq	%r14
14165faf4: 41 5f                       	popq	%r15
14165faf6: 5d                          	popq	%rbp
14165faf7: c3                          	retq
14165faf8: 48 85 c0                    	testq	%rax, %rax
14165fafb: 75 1a                       	jne	0x14165fb17 <.text+0x165eb17>
14165fafd: 31 d2                       	xorl	%edx, %edx
14165faff: e8 cc 8c 29 01              	callq	0x1428f87d0 <.text+0x28f77d0>
14165fb04: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
14165fb0b: 48 89 c1                    	movq	%rax, %rcx
14165fb0e: 48 85 c0                    	testq	%rax, %rax
14165fb11: 0f 85 74 ff ff ff           	jne	0x14165fa8b <.text+0x165ea8b>
14165fb17: 48 8d 0d 82 af 5d 01        	leaq	0x15daf82(%rip), %rcx   # 0x142c3aaa0
14165fb1e: e8 ad 85 27 01              	callq	0x1428d80d0 <.text+0x28d70d0>
14165fb23: 66 0f 6e 70 4c              	movd	0x4c(%rax), %xmm6
14165fb28: f3 44 0f 10 48 50           	movss	0x50(%rax), %xmm9
14165fb2e: f3 0f 10 78 54              	movss	0x54(%rax), %xmm7
14165fb33: 48 8b 95 c8 00 00 00        	movq	0xc8(%rbp), %rdx
14165fb3a: c6 85 b5 03 00 00 00        	movb	$0x0, 0x3b5(%rbp)
14165fb41: 4c 8d 0d 18 28 62 01        	leaq	0x1622818(%rip), %r9    # 0x142c82360
14165fb48: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14165fb4c: 48 89 95 a8 03 00 00        	movq	%rdx, 0x3a8(%rbp)
14165fb53: 41 b0 02                    	movb	$0x2, %r8b
14165fb56: e8 85 7b cc ff              	callq	0x1413276e0 <.text+0x13266e0>
14165fb5b: 66 0f 62 f7                 	punpckldq	%xmm7, %xmm6    # xmm6 = xmm6[0],xmm7[0],xmm6[1],xmm7[1]
14165fb5f: 41 0f 5c f2                 	subps	%xmm10, %xmm6
14165fb63: 0f 28 c6                    	movaps	%xmm6, %xmm0
14165fb66: 0f 59 c6                    	mulps	%xmm6, %xmm0
14165fb69: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
14165fb6d: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14165fb71: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165fb74: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
14165fb78: f3 0f 10 0d 84 5d 2c 01     	movss	0x12c5d84(%rip), %xmm1  # 0x142925904
14165fb80: f3 0f 5e c8                 	divss	%xmm0, %xmm1
14165fb84: f3 44 0f 12 d1              	movsldup	%xmm1, %xmm10           # xmm10 = xmm1[0,0,2,2]
14165fb89: 44 0f 59 d6                 	mulps	%xmm6, %xmm10
14165fb8d: 45 31 c0                    	xorl	%r8d, %r8d
14165fb90: 4c 8b ad 08 01 00 00        	movq	0x108(%rbp), %r13
14165fb97: 8b 06                       	movl	(%rsi), %eax
14165fb99: 83 f8 05                    	cmpl	$0x5, %eax
14165fb9c: 0f 86 06 de ff ff           	jbe	0x14165d9a8 <.text+0x165c9a8>
14165fba2: e9 b1 e6 ff ff              	jmp	0x14165e258 <.text+0x165d258>
14165fba7: 48 8d 0d c2 af 5d 01        	leaq	0x15dafc2(%rip), %rcx   # 0x142c3ab70
14165fbae: e8 9d 96 27 01              	callq	0x1428d9250 <.text+0x28d8250>
14165fbb3: 48 8d 0d d9 a5 47 01        	leaq	0x147a5d9(%rip), %rcx   # 0x142ada193
14165fbba: 4c 8d 05 47 27 62 01        	leaq	0x1622747(%rip), %r8    # 0x142c82308
14165fbc1: ba 32 00 00 00              	movl	$0x32, %edx
14165fbc6: e8 65 98 27 01              	callq	0x1428d9430 <.text+0x28d8430>
14165fbcb: e9 47 03 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fbd0: 48 8d 0d bc a5 47 01        	leaq	0x147a5bc(%rip), %rcx   # 0x142ada193
14165fbd7: 4c 8d 05 fa 27 62 01        	leaq	0x16227fa(%rip), %r8    # 0x142c823d8
14165fbde: ba 32 00 00 00              	movl	$0x32, %edx
14165fbe3: e8 48 98 27 01              	callq	0x1428d9430 <.text+0x28d8430>
14165fbe8: e9 2a 03 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fbed: 48 8d 0d 9f a5 47 01        	leaq	0x147a59f(%rip), %rcx   # 0x142ada193
14165fbf4: 4c 8d 05 7d 27 62 01        	leaq	0x162277d(%rip), %r8    # 0x142c82378
14165fbfb: ba 32 00 00 00              	movl	$0x32, %edx
14165fc00: e8 2b 98 27 01              	callq	0x1428d9430 <.text+0x28d8430>
14165fc05: e9 0d 03 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fc0a: 48 8d 0d ab 2c 43 01        	leaq	0x1432cab(%rip), %rcx   # 0x142a928bc
14165fc11: 4c 8d 05 00 2d 43 01        	leaq	0x1432d00(%rip), %r8    # 0x142a92918
14165fc18: ba 28 00 00 00              	movl	$0x28, %edx
14165fc1d: e8 0e 98 27 01              	callq	0x1428d9430 <.text+0x28d8430>
14165fc22: e9 f0 02 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fc27: 48 8d 05 02 2a 62 01        	leaq	0x1622a02(%rip), %rax   # 0x142c82630
14165fc2e: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165fc35: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165fc40: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
14165fc4b: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14165fc4f: f3 0f 7f 85 c8 01 00 00     	movdqu	%xmm0, 0x1c8(%rbp)
14165fc57: 48 8d 15 e2 29 62 01        	leaq	0x16229e2(%rip), %rdx   # 0x142c82640
14165fc5e: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165fc65: e8 26 97 27 01              	callq	0x1428d9390 <.text+0x28d8390>
14165fc6a: e9 a8 02 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fc6f: 48 8d 0d 12 29 62 01        	leaq	0x1622912(%rip), %rcx   # 0x142c82588
14165fc76: 4c 8d 05 33 29 62 01        	leaq	0x1622933(%rip), %r8    # 0x142c825b0
14165fc7d: ba 28 00 00 00              	movl	$0x28, %edx
14165fc82: e8 a9 97 27 01              	callq	0x1428d9430 <.text+0x28d8430>
14165fc87: e9 8b 02 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fc8c: 48 8d 0d 5d b1 48 01        	leaq	0x148b15d(%rip), %rcx   # 0x142aeadf0
14165fc93: e8 78 96 27 01              	callq	0x1428d9310 <.text+0x28d8310>
14165fc98: e9 7a 02 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fc9d: 48 8d 05 6c 9c 5b 01        	leaq	0x15b9c6c(%rip), %rax   # 0x142c19910
14165fca4: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165fcab: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165fcb6: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
14165fcc1: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165fcc4: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14165fccb: 48 8d 15 4e 9c 5b 01        	leaq	0x15b9c4e(%rip), %rdx   # 0x142c19920
14165fcd2: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165fcd9: e8 b2 96 27 01              	callq	0x1428d9390 <.text+0x28d8390>
14165fcde: e9 34 02 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fce3: 48 c1 e8 20                 	shrq	$0x20, %rax
14165fce7: 49 89 c1                    	movq	%rax, %r9
14165fcea: 49 c1 e1 20                 	shlq	$0x20, %r9
14165fcee: 49 09 d1                    	orq	%rdx, %r9
14165fcf1: ba 01 00 00 00              	movl	$0x1, %edx
14165fcf6: 4c 89 c8                    	movq	%r9, %rax
14165fcf9: 89 95 b0 01 00 00           	movl	%edx, 0x1b0(%rbp)
14165fcff: 89 8d b4 01 00 00           	movl	%ecx, 0x1b4(%rbp)
14165fd05: 48 89 85 b8 01 00 00        	movq	%rax, 0x1b8(%rbp)
14165fd0c: 48 8d 05 5d 29 62 01        	leaq	0x162295d(%rip), %rax   # 0x142c82670
14165fd13: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14165fd18: 48 8d 0d 99 21 62 01        	leaq	0x1622199(%rip), %rcx   # 0x142c81eb8
14165fd1f: 4c 8d 0d 72 21 62 01        	leaq	0x1622172(%rip), %r9    # 0x142c81e98
14165fd26: 4c 8d 85 b0 01 00 00        	leaq	0x1b0(%rbp), %r8
14165fd2d: ba 2b 00 00 00              	movl	$0x2b, %edx
14165fd32: e8 29 9a 27 01              	callq	0x1428d9760 <.text+0x28d8760>
14165fd37: e9 db 01 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fd3c: b9 10 00 00 00              	movl	$0x10, %ecx
14165fd41: ba 30 00 00 00              	movl	$0x30, %edx
14165fd46: e8 b5 92 27 01              	callq	0x1428d9000 <.text+0x28d8000>
14165fd4b: e9 c7 01 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fd50: b9 04 00 00 00              	movl	$0x4, %ecx
14165fd55: ba 18 00 00 00              	movl	$0x18, %edx
14165fd5a: e8 a1 92 27 01              	callq	0x1428d9000 <.text+0x28d8000>
14165fd5f: e9 b3 01 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fd64: 4c 89 b5 10 03 00 00        	movq	%r14, 0x310(%rbp)
14165fd6b: 45 31 f6                    	xorl	%r14d, %r14d
14165fd6e: 4c 8d 05 ab 91 43 01        	leaq	0x14391ab(%rip), %r8    # 0x142a98f20
14165fd75: 4c 89 f1                    	movq	%r14, %rcx
14165fd78: 48 89 f2                    	movq	%rsi, %rdx
14165fd7b: e8 63 92 27 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14165fd80: e9 92 01 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fd85: 48 8d 05 14 b1 48 01        	leaq	0x148b114(%rip), %rax   # 0x142aeaea0
14165fd8c: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165fd93: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165fd9e: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
14165fda9: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165fdac: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14165fdb3: 48 8d 15 f6 b0 48 01        	leaq	0x148b0f6(%rip), %rdx   # 0x142aeaeb0
14165fdba: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165fdc1: e8 ca 95 27 01              	callq	0x1428d9390 <.text+0x28d8390>
14165fdc6: e9 4c 01 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fdcb: 48 8d 0d 86 27 62 01        	leaq	0x1622786(%rip), %rcx   # 0x142c82558
14165fdd2: e8 39 95 27 01              	callq	0x1428d9310 <.text+0x28d8310>
14165fdd7: e9 3b 01 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fddc: 48 8d 05 bd b0 48 01        	leaq	0x148b0bd(%rip), %rax   # 0x142aeaea0
14165fde3: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165fdea: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165fdf5: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
14165fe00: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165fe03: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14165fe0a: 48 8d 15 9f b0 48 01        	leaq	0x148b09f(%rip), %rdx   # 0x142aeaeb0
14165fe11: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165fe18: e8 73 95 27 01              	callq	0x1428d9390 <.text+0x28d8390>
14165fe1d: e9 f5 00 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fe22: 48 8d 05 77 b0 48 01        	leaq	0x148b077(%rip), %rax   # 0x142aeaea0
14165fe29: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165fe30: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165fe3b: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
14165fe46: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165fe49: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14165fe50: c6 85 b7 03 00 00 00        	movb	$0x0, 0x3b7(%rbp)
14165fe57: 48 8d 15 52 b0 48 01        	leaq	0x148b052(%rip), %rdx   # 0x142aeaeb0
14165fe5e: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165fe65: e8 26 95 27 01              	callq	0x1428d9390 <.text+0x28d8390>
14165fe6a: e9 a8 00 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fe6f: 48 8d 0d 12 27 62 01        	leaq	0x1622712(%rip), %rcx   # 0x142c82588
14165fe76: 4c 8d 05 63 27 62 01        	leaq	0x1622763(%rip), %r8    # 0x142c825e0
14165fe7d: ba 28 00 00 00              	movl	$0x28, %edx
14165fe82: e8 a9 95 27 01              	callq	0x1428d9430 <.text+0x28d8430>
14165fe87: e9 8b 00 00 00              	jmp	0x14165ff17 <.text+0x165ef17>
14165fe8c: 48 8d 05 0d b0 48 01        	leaq	0x148b00d(%rip), %rax   # 0x142aeaea0
14165fe93: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165fe9a: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165fea5: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
14165feb0: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165feb3: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14165feba: c6 85 b7 03 00 00 00        	movb	$0x0, 0x3b7(%rbp)
14165fec1: 48 8d 15 e8 af 48 01        	leaq	0x148afe8(%rip), %rdx   # 0x142aeaeb0
14165fec8: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165fecf: e8 bc 94 27 01              	callq	0x1428d9390 <.text+0x28d8390>
14165fed4: eb 41                       	jmp	0x14165ff17 <.text+0x165ef17>
14165fed6: 48 8d 05 33 9a 5b 01        	leaq	0x15b9a33(%rip), %rax   # 0x142c19910
14165fedd: 48 89 85 b0 01 00 00        	movq	%rax, 0x1b0(%rbp)
14165fee4: 48 c7 85 b8 01 00 00 01 00 00 00    	movq	$0x1, 0x1b8(%rbp)
14165feef: 48 c7 85 c0 01 00 00 08 00 00 00    	movq	$0x8, 0x1c0(%rbp)
14165fefa: 0f 57 c0                    	xorps	%xmm0, %xmm0
14165fefd: 0f 11 85 c8 01 00 00        	movups	%xmm0, 0x1c8(%rbp)
14165ff04: 48 8d 15 15 9a 5b 01        	leaq	0x15b9a15(%rip), %rdx   # 0x142c19920
14165ff0b: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
14165ff12: e8 79 94 27 01              	callq	0x1428d9390 <.text+0x28d8390>
14165ff17: 0f 0b                       	ud2
14165ff19: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14165ff20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14165ff25: 55                          	pushq	%rbp
14165ff26: 41 57                       	pushq	%r15
14165ff28: 41 56                       	pushq	%r14
14165ff2a: 41 55                       	pushq	%r13
14165ff2c: 41 54                       	pushq	%r12
14165ff2e: 56                          	pushq	%rsi
14165ff2f: 57                          	pushq	%rdi
14165ff30: 53                          	pushq	%rbx
14165ff31: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14165ff38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14165ff3f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
14165ff45: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14165ff4b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14165ff51: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
14165ff57: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14165ff5c: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
14165ff65: c6 85 b6 03 00 00 00        	movb	$0x0, 0x3b6(%rbp)
14165ff6c: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
14165ff73: e8 f8 84 23 ff              	callq	0x140898470 <.text+0x897470>
14165ff78: c6 85 b6 03 00 00 00        	movb	$0x0, 0x3b6(%rbp)
14165ff7f: 66 0f 6f b4 24 80 00 00 00  	movdqa	0x80(%rsp), %xmm6
14165ff88: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14165ff8d: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14165ff93: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14165ff99: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14165ff9f: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14165ffa5: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14165ffac: 5b                          	popq	%rbx
14165ffad: 5f                          	popq	%rdi
14165ffae: 5e                          	popq	%rsi
14165ffaf: 41 5c                       	popq	%r12
14165ffb1: 41 5d                       	popq	%r13
14165ffb3: 41 5e                       	popq	%r14
14165ffb5: 41 5f                       	popq	%r15
14165ffb7: 5d                          	popq	%rbp
14165ffb8: c3                          	retq
14165ffb9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14165ffc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14165ffc5: 55                          	pushq	%rbp
14165ffc6: 41 57                       	pushq	%r15
14165ffc8: 41 56                       	pushq	%r14
14165ffca: 41 55                       	pushq	%r13
14165ffcc: 41 54                       	pushq	%r12
14165ffce: 56                          	pushq	%rsi
14165ffcf: 57                          	pushq	%rdi
14165ffd0: 53                          	pushq	%rbx
14165ffd1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
14165ffd8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14165ffdf: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
14165ffe5: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14165ffeb: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
14165fff1: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
14165fff7: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14165fffc: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141660005: 80 bd b6 03 00 00 00        	cmpb	$0x0, 0x3b6(%rbp)
14166000c: 74 0c                       	je	0x14166001a <.text+0x165f01a>
14166000e: 48 8d 8d 48 03 00 00        	leaq	0x348(%rbp), %rcx
141660015: e8 56 84 23 ff              	callq	0x140898470 <.text+0x897470>
14166001a: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141660022: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141660027: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14166002d: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141660033: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141660039: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14166003f: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141660046: 5b                          	popq	%rbx
141660047: 5f                          	popq	%rdi
141660048: 5e                          	popq	%rsi
141660049: 41 5c                       	popq	%r12
14166004b: 41 5d                       	popq	%r13
14166004d: 41 5e                       	popq	%r14
14166004f: 41 5f                       	popq	%r15
141660051: 5d                          	popq	%rbp
141660052: c3                          	retq
141660053: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141660060: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660065: 55                          	pushq	%rbp
141660066: 41 57                       	pushq	%r15
141660068: 41 56                       	pushq	%r14
14166006a: 41 55                       	pushq	%r13
14166006c: 41 54                       	pushq	%r12
14166006e: 56                          	pushq	%rsi
14166006f: 57                          	pushq	%rdi
141660070: 53                          	pushq	%rbx
141660071: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141660078: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14166007f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
141660085: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14166008b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141660091: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
141660097: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14166009c: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1416600a4: 48 8b 85 10 03 00 00        	movq	0x310(%rbp), %rax
1416600ab: f0                          	lock
1416600ac: 48 ff 08                    	decq	(%rax)
1416600af: 75 0c                       	jne	0x1416600bd <.text+0x165f0bd>
1416600b1: 48 8d 8d b0 01 00 00        	leaq	0x1b0(%rbp), %rcx
1416600b8: e8 13 f9 fe fe              	callq	0x14064f9d0 <.text+0x64e9d0>
1416600bd: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1416600c5: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
1416600ca: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1416600d0: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
1416600d6: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1416600dc: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1416600e2: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1416600e9: 5b                          	popq	%rbx
1416600ea: 5f                          	popq	%rdi
1416600eb: 5e                          	popq	%rsi
1416600ec: 41 5c                       	popq	%r12
1416600ee: 41 5d                       	popq	%r13
1416600f0: 41 5e                       	popq	%r14
1416600f2: 41 5f                       	popq	%r15
1416600f4: 5d                          	popq	%rbp
1416600f5: c3                          	retq
1416600f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141660100: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660105: 55                          	pushq	%rbp
141660106: 41 57                       	pushq	%r15
141660108: 41 56                       	pushq	%r14
14166010a: 41 55                       	pushq	%r13
14166010c: 41 54                       	pushq	%r12
14166010e: 56                          	pushq	%rsi
14166010f: 57                          	pushq	%rdi
141660110: 53                          	pushq	%rbx
141660111: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141660118: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14166011f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
141660125: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14166012b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141660131: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
141660137: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14166013c: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141660145: ba 18 00 00 00              	movl	$0x18, %edx
14166014a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141660150: 48 8b 8d a0 03 00 00        	movq	0x3a0(%rbp), %rcx
141660157: e8 c4 3a fb fe              	callq	0x140613c20 <.text+0x612c20>
14166015c: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141660164: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141660169: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14166016f: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141660175: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
14166017b: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141660181: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141660188: 5b                          	popq	%rbx
141660189: 5f                          	popq	%rdi
14166018a: 5e                          	popq	%rsi
14166018b: 41 5c                       	popq	%r12
14166018d: 41 5d                       	popq	%r13
14166018f: 41 5e                       	popq	%r14
141660191: 41 5f                       	popq	%r15
141660193: 5d                          	popq	%rbp
141660194: c3                          	retq
141660195: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1416601a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416601a5: 55                          	pushq	%rbp
1416601a6: 41 57                       	pushq	%r15
1416601a8: 41 56                       	pushq	%r14
1416601aa: 41 55                       	pushq	%r13
1416601ac: 41 54                       	pushq	%r12
1416601ae: 56                          	pushq	%rsi
1416601af: 57                          	pushq	%rdi
1416601b0: 53                          	pushq	%rbx
1416601b1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1416601b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416601bf: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
1416601c5: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
1416601cb: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1416601d1: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1416601d7: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
1416601dc: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
1416601e5: 80 bd b5 03 00 00 00        	cmpb	$0x0, 0x3b5(%rbp)
1416601ec: 74 45                       	je	0x141660233 <.text+0x165f233>
1416601ee: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
1416601f5: e8 d6 f3 2e ff              	callq	0x14094f5d0 <.text+0x94e5d0>
1416601fa: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141660202: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141660207: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14166020d: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141660213: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141660219: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14166021f: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141660226: 5b                          	popq	%rbx
141660227: 5f                          	popq	%rdi
141660228: 5e                          	popq	%rsi
141660229: 41 5c                       	popq	%r12
14166022b: 41 5d                       	popq	%r13
14166022d: 41 5e                       	popq	%r14
14166022f: 41 5f                       	popq	%r15
141660231: 5d                          	popq	%rbp
141660232: c3                          	retq
141660233: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14166023b: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141660240: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141660246: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14166024c: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141660252: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
141660258: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14166025f: 5b                          	popq	%rbx
141660260: 5f                          	popq	%rdi
141660261: 5e                          	popq	%rsi
141660262: 41 5c                       	popq	%r12
141660264: 41 5d                       	popq	%r13
141660266: 41 5e                       	popq	%r14
141660268: 41 5f                       	popq	%r15
14166026a: 5d                          	popq	%rbp
14166026b: c3                          	retq
14166026c: 0f 1f 40 00                 	nopl	(%rax)
141660270: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660275: 55                          	pushq	%rbp
141660276: 41 57                       	pushq	%r15
141660278: 41 56                       	pushq	%r14
14166027a: 41 55                       	pushq	%r13
14166027c: 41 54                       	pushq	%r12
14166027e: 56                          	pushq	%rsi
14166027f: 57                          	pushq	%rdi
141660280: 53                          	pushq	%rbx
141660281: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141660288: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14166028f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
141660295: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14166029b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1416602a1: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1416602a7: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
1416602ac: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
1416602b5: 48 8b 85 98 01 00 00        	movq	0x198(%rbp), %rax
1416602bc: 48 ff 00                    	incq	(%rax)
1416602bf: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1416602c7: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
1416602cc: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1416602d2: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
1416602d8: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1416602de: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1416602e4: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1416602eb: 5b                          	popq	%rbx
1416602ec: 5f                          	popq	%rdi
1416602ed: 5e                          	popq	%rsi
1416602ee: 41 5c                       	popq	%r12
1416602f0: 41 5d                       	popq	%r13
1416602f2: 41 5e                       	popq	%r14
1416602f4: 41 5f                       	popq	%r15
1416602f6: 5d                          	popq	%rbp
1416602f7: c3                          	retq
1416602f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141660300: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660305: 55                          	pushq	%rbp
141660306: 41 57                       	pushq	%r15
141660308: 41 56                       	pushq	%r14
14166030a: 41 55                       	pushq	%r13
14166030c: 41 54                       	pushq	%r12
14166030e: 56                          	pushq	%rsi
14166030f: 57                          	pushq	%rdi
141660310: 53                          	pushq	%rbx
141660311: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141660318: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14166031f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
141660325: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14166032b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141660331: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
141660337: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14166033c: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
141660344: 80 bd b7 03 00 00 00        	cmpb	$0x0, 0x3b7(%rbp)
14166034b: 74 45                       	je	0x141660392 <.text+0x165f392>
14166034d: 48 8d 8d d0 02 00 00        	leaq	0x2d0(%rbp), %rcx
141660354: e8 77 f2 2e ff              	callq	0x14094f5d0 <.text+0x94e5d0>
141660359: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
141660361: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141660366: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14166036c: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141660372: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141660378: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14166037e: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141660385: 5b                          	popq	%rbx
141660386: 5f                          	popq	%rdi
141660387: 5e                          	popq	%rsi
141660388: 41 5c                       	popq	%r12
14166038a: 41 5d                       	popq	%r13
14166038c: 41 5e                       	popq	%r14
14166038e: 41 5f                       	popq	%r15
141660390: 5d                          	popq	%rbp
141660391: c3                          	retq
141660392: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14166039a: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
14166039f: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1416603a5: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
1416603ab: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1416603b1: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1416603b7: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1416603be: 5b                          	popq	%rbx
1416603bf: 5f                          	popq	%rdi
1416603c0: 5e                          	popq	%rsi
1416603c1: 41 5c                       	popq	%r12
1416603c3: 41 5d                       	popq	%r13
1416603c5: 41 5e                       	popq	%r14
1416603c7: 41 5f                       	popq	%r15
1416603c9: 5d                          	popq	%rbp
1416603ca: c3                          	retq
1416603cb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1416603d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1416603d5: 55                          	pushq	%rbp
1416603d6: 41 57                       	pushq	%r15
1416603d8: 41 56                       	pushq	%r14
1416603da: 41 55                       	pushq	%r13
1416603dc: 41 54                       	pushq	%r12
1416603de: 56                          	pushq	%rsi
1416603df: 57                          	pushq	%rdi
1416603e0: 53                          	pushq	%rbx
1416603e1: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1416603e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1416603ef: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
1416603f5: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
1416603fb: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141660401: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
141660407: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14166040c: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
141660414: 48 83 bd 30 03 00 00 00     	cmpq	$0x0, 0x330(%rbp)
14166041c: 74 19                       	je	0x141660437 <.text+0x165f437>
14166041e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141660424: 48 8b 8d 08 03 00 00        	movq	0x308(%rbp), %rcx
14166042b: 48 8b 95 30 03 00 00        	movq	0x330(%rbp), %rdx
141660432: e8 e9 37 fb fe              	callq	0x140613c20 <.text+0x612c20>
141660437: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14166043f: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141660444: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
14166044a: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
141660450: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
141660456: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
14166045c: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
141660463: 5b                          	popq	%rbx
141660464: 5f                          	popq	%rdi
141660465: 5e                          	popq	%rsi
141660466: 41 5c                       	popq	%r12
141660468: 41 5d                       	popq	%r13
14166046a: 41 5e                       	popq	%r14
14166046c: 41 5f                       	popq	%r15
14166046e: 5d                          	popq	%rbp
14166046f: c3                          	retq
141660470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660475: 55                          	pushq	%rbp
141660476: 41 57                       	pushq	%r15
141660478: 41 56                       	pushq	%r14
14166047a: 41 55                       	pushq	%r13
14166047c: 41 54                       	pushq	%r12
14166047e: 56                          	pushq	%rsi
14166047f: 57                          	pushq	%rdi
141660480: 53                          	pushq	%rbx
141660481: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141660488: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14166048f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
141660495: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14166049b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
1416604a1: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1416604a7: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
1416604ac: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
1416604b5: 48 8b 85 88 03 00 00        	movq	0x388(%rbp), %rax
1416604bc: 48 ff 00                    	incq	(%rax)
1416604bf: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1416604c7: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
1416604cc: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1416604d2: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
1416604d8: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1416604de: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1416604e4: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1416604eb: 5b                          	popq	%rbx
1416604ec: 5f                          	popq	%rdi
1416604ed: 5e                          	popq	%rsi
1416604ee: 41 5c                       	popq	%r12
1416604f0: 41 5d                       	popq	%r13
1416604f2: 41 5e                       	popq	%r14
1416604f4: 41 5f                       	popq	%r15
1416604f6: 5d                          	popq	%rbp
1416604f7: c3                          	retq
1416604f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141660500: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660505: 55                          	pushq	%rbp
141660506: 41 57                       	pushq	%r15
141660508: 41 56                       	pushq	%r14
14166050a: 41 55                       	pushq	%r13
14166050c: 41 54                       	pushq	%r12
14166050e: 56                          	pushq	%rsi
14166050f: 57                          	pushq	%rdi
141660510: 53                          	pushq	%rbx
141660511: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
141660518: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14166051f: 44 0f 29 5c 24 30           	movaps	%xmm11, 0x30(%rsp)
141660525: 44 0f 29 54 24 40           	movaps	%xmm10, 0x40(%rsp)
14166052b: 44 0f 29 4c 24 50           	movaps	%xmm9, 0x50(%rsp)
141660531: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
141660537: 0f 29 7c 24 70              	movaps	%xmm7, 0x70(%rsp)
14166053c: 66 0f 7f b4 24 80 00 00 00  	movdqa	%xmm6, 0x80(%rsp)
141660545: 80 bd 98 01 00 00 00        	cmpb	$0x0, 0x198(%rbp)
14166054c: 74 35                       	je	0x141660583 <.text+0x165f583>
14166054e: e8 dd 7b 5b ff              	callq	0x140c18130 <.text+0xc17130>
141660553: 48 89 c6                    	movq	%rax, %rsi
141660556: 48 8b 00                    	movq	(%rax), %rax
141660559: 48 83 f8 01                 	cmpq	$0x1, %rax
14166055d: 75 5d                       	jne	0x1416605bc <.text+0x165f5bc>
14166055f: 48 83 c6 08                 	addq	$0x8, %rsi
141660563: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
14166056a: 48 83 3e 00                 	cmpq	$0x0, (%rsi)
14166056e: 75 76                       	jne	0x1416605e6 <.text+0x165f5e6>
141660570: 48 c7 06 ff ff ff ff        	movq	$-0x1, (%rsi)
141660577: 48 8d 4e 08                 	leaq	0x8(%rsi), %rcx
14166057b: e8 70 6b 5b ff              	callq	0x140c170f0 <.text+0xc160f0>
141660580: 48 ff 06                    	incq	(%rsi)
141660583: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
14166058b: 0f 28 7c 24 70              	movaps	0x70(%rsp), %xmm7
141660590: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
141660596: 44 0f 28 4c 24 50           	movaps	0x50(%rsp), %xmm9
14166059c: 44 0f 28 54 24 40           	movaps	0x40(%rsp), %xmm10
1416605a2: 44 0f 28 5c 24 30           	movaps	0x30(%rsp), %xmm11
1416605a8: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1416605af: 5b                          	popq	%rbx
1416605b0: 5f                          	popq	%rdi
1416605b1: 5e                          	popq	%rsi
1416605b2: 41 5c                       	popq	%r12
1416605b4: 41 5d                       	popq	%r13
1416605b6: 41 5e                       	popq	%r14
1416605b8: 41 5f                       	popq	%r15
1416605ba: 5d                          	popq	%rbp
1416605bb: c3                          	retq
1416605bc: 48 85 c0                    	testq	%rax, %rax
1416605bf: 75 19                       	jne	0x1416605da <.text+0x165f5da>
1416605c1: 48 89 f1                    	movq	%rsi, %rcx
1416605c4: 31 d2                       	xorl	%edx, %edx
1416605c6: e8 05 82 29 01              	callq	0x1428f87d0 <.text+0x28f77d0>
1416605cb: 48 8b 95 88 01 00 00        	movq	0x188(%rbp), %rdx
1416605d2: 48 89 c6                    	movq	%rax, %rsi
1416605d5: 48 85 c0                    	testq	%rax, %rax
1416605d8: 75 90                       	jne	0x14166056a <.text+0x165f56a>
1416605da: 48 8d 0d bf a4 5d 01        	leaq	0x15da4bf(%rip), %rcx   # 0x142c3aaa0
1416605e1: e8 ea 7a 27 01              	callq	0x1428d80d0 <.text+0x28d70d0>
1416605e6: 48 8d 0d 83 a5 5d 01        	leaq	0x15da583(%rip), %rcx   # 0x142c3ab70
1416605ed: e8 5e 8c 27 01              	callq	0x1428d9250 <.text+0x28d8250>
1416605f2: cc                          	int3
1416605f3: cc                          	int3
1416605f4: cc                          	int3
1416605f5: cc                          	int3
1416605f6: cc                          	int3
1416605f7: cc                          	int3
1416605f8: cc                          	int3
1416605f9: cc                          	int3
1416605fa: cc                          	int3
1416605fb: cc                          	int3
1416605fc: cc                          	int3
1416605fd: cc                          	int3
1416605fe: cc                          	int3
1416605ff: cc                          	int3
141660600: 55                          	pushq	%rbp
141660601: 41 57                       	pushq	%r15
141660603: 41 56                       	pushq	%r14
141660605: 56                          	pushq	%rsi
141660606: 57                          	pushq	%rdi
141660607: 53                          	pushq	%rbx
141660608: 48 81 ec 18 01 00 00        	subq	$0x118, %rsp            # imm = 0x118
14166060f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141660617: 48 c7 85 90 00 00 00 fe ff ff ff    	movq	$-0x2, 0x90(%rbp)
141660622: 4c 89 cf                    	movq	%r9, %rdi
141660625: 4c 89 c3                    	movq	%r8, %rbx
141660628: 49 89 d6                    	movq	%rdx, %r14
14166062b: 48 89 ce                    	movq	%rcx, %rsi
14166062e: 0f b6 05 d4 45 fd 01        	movzbl	0x1fd45d4(%rip), %eax   # 0x143634c09
141660635: b9 08 00 00 00              	movl	$0x8, %ecx
14166063a: ba 08 00 00 00              	movl	$0x8, %edx
14166063f: e8 cc 35 fb fe              	callq	0x140613c10 <.text+0x612c10>
141660644: 48 85 c0                    	testq	%rax, %rax
141660647: 0f 84 76 01 00 00           	je	0x1416607c3 <.text+0x165f7c3>
14166064d: 49 89 c7                    	movq	%rax, %r15
141660650: 0f b6 05 b2 45 fd 01        	movzbl	0x1fd45b2(%rip), %eax   # 0x143634c09
141660657: b9 1e 00 00 00              	movl	$0x1e, %ecx
14166065c: ba 01 00 00 00              	movl	$0x1, %edx
141660661: e8 aa 35 fb fe              	callq	0x140613c10 <.text+0x612c10>
141660666: 48 85 c0                    	testq	%rax, %rax
141660669: 4c 89 bd 88 00 00 00        	movq	%r15, 0x88(%rbp)
141660670: 0f 84 5c 01 00 00           	je	0x1416607d2 <.text+0x165f7d2>
141660676: 0f 10 05 29 1e 62 01        	movups	0x1621e29(%rip), %xmm0  # 0x142c824a6
14166067d: 0f 11 40 0e                 	movups	%xmm0, 0xe(%rax)
141660681: 0f 10 05 10 1e 62 01        	movups	0x1621e10(%rip), %xmm0  # 0x142c82498
141660688: 0f 11 00                    	movups	%xmm0, (%rax)
14166068b: 48 c7 45 50 1e 00 00 00     	movq	$0x1e, 0x50(%rbp)
141660693: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
141660697: 48 c7 45 60 1e 00 00 00     	movq	$0x1e, 0x60(%rbp)
14166069f: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416606a9: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
1416606ad: 0f b6 05 55 45 fd 01        	movzbl	0x1fd4555(%rip), %eax   # 0x143634c09
1416606b4: b9 13 00 00 00              	movl	$0x13, %ecx
1416606b9: ba 01 00 00 00              	movl	$0x1, %edx
1416606be: e8 4d 35 fb fe              	callq	0x140613c10 <.text+0x612c10>
1416606c3: 48 85 c0                    	testq	%rax, %rax
1416606c6: 0f 84 1e 01 00 00           	je	0x1416607ea <.text+0x165f7ea>
1416606cc: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1416606d3: 8b 15 c2 8d 43 01           	movl	0x1438dc2(%rip), %edx   # 0x142a9949b
1416606d9: 89 50 0f                    	movl	%edx, 0xf(%rax)
1416606dc: 0f 10 05 a9 8d 43 01        	movups	0x1438da9(%rip), %xmm0  # 0x142a9948c
1416606e3: 0f 11 00                    	movups	%xmm0, (%rax)
1416606e6: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
1416606ea: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
1416606ee: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
1416606f2: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1416606f6: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1416606fa: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1416606fe: 49 8b 56 10                 	movq	0x10(%r14), %rdx
141660702: 48 89 55 78                 	movq	%rdx, 0x78(%rbp)
141660706: 41 8b 56 18                 	movl	0x18(%r14), %edx
14166070a: 89 95 80 00 00 00           	movl	%edx, 0x80(%rbp)
141660710: 41 0f 28 06                 	movaps	(%r14), %xmm0
141660714: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
141660718: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
14166071e: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
141660728: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
14166072c: 48 ba 01 00 00 00 00 00 c0 3f       	movabsq	$0x3fc0000000000001, %rdx # imm = 0x3FC0000000000001
141660736: 48 89 55 00                 	movq	%rdx, (%rbp)
14166073a: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
141660741: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
141660749: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14166074d: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
141660755: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
14166075c: 8b 85 80 00 00 00           	movl	0x80(%rbp), %eax
141660762: 89 45 34                    	movl	%eax, 0x34(%rbp)
141660765: 48 8b 45 78                 	movq	0x78(%rbp), %rax
141660769: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
14166076d: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
141660773: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
141660777: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14166077c: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
141660780: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
141660784: 49 89 d8                    	movq	%rbx, %r8
141660787: 49 89 f9                    	movq	%rdi, %r9
14166078a: e8 c1 d5 23 ff              	callq	0x14089dd50 <.text+0x89cd50>
14166078f: 48 8b 45 68                 	movq	0x68(%rbp), %rax
141660793: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14166079a: 48 89 01                    	movq	%rax, (%rcx)
14166079d: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1416607a4: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
1416607a8: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1416607b0: 48 89 f0                    	movq	%rsi, %rax
1416607b3: 48 81 c4 18 01 00 00        	addq	$0x118, %rsp            # imm = 0x118
1416607ba: 5b                          	popq	%rbx
1416607bb: 5f                          	popq	%rdi
1416607bc: 5e                          	popq	%rsi
1416607bd: 41 5e                       	popq	%r14
1416607bf: 41 5f                       	popq	%r15
1416607c1: 5d                          	popq	%rbp
1416607c2: c3                          	retq
1416607c3: b9 08 00 00 00              	movl	$0x8, %ecx
1416607c8: ba 08 00 00 00              	movl	$0x8, %edx
1416607cd: e8 2e 88 27 01              	callq	0x1428d9000 <.text+0x28d8000>
1416607d2: 4c 8d 05 7f 17 62 01        	leaq	0x162177f(%rip), %r8    # 0x142c81f58
1416607d9: b9 01 00 00 00              	movl	$0x1, %ecx
1416607de: ba 1e 00 00 00              	movl	$0x1e, %edx
1416607e3: e8 fb 87 27 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1416607e8: eb 16                       	jmp	0x141660800 <.text+0x165f800>
1416607ea: 4c 8d 05 2f 87 43 01        	leaq	0x143872f(%rip), %r8    # 0x142a98f20
1416607f1: b9 01 00 00 00              	movl	$0x1, %ecx
1416607f6: ba 13 00 00 00              	movl	$0x13, %edx
1416607fb: e8 e3 87 27 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
141660800: 0f 0b                       	ud2
141660802: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
141660810: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660815: 55                          	pushq	%rbp
141660816: 41 57                       	pushq	%r15
141660818: 41 56                       	pushq	%r14
14166081a: 56                          	pushq	%rsi
14166081b: 57                          	pushq	%rdi
14166081c: 53                          	pushq	%rbx
14166081d: 48 83 ec 28                 	subq	$0x28, %rsp
141660821: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141660828: ba 08 00 00 00              	movl	$0x8, %edx
14166082d: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141660833: 48 8b 8d 88 00 00 00        	movq	0x88(%rbp), %rcx
14166083a: e8 e1 33 fb fe              	callq	0x140613c20 <.text+0x612c20>
14166083f: 90                          	nop
141660840: 48 83 c4 28                 	addq	$0x28, %rsp
141660844: 5b                          	popq	%rbx
141660845: 5f                          	popq	%rdi
141660846: 5e                          	popq	%rsi
141660847: 41 5e                       	popq	%r14
141660849: 41 5f                       	popq	%r15
14166084b: 5d                          	popq	%rbp
14166084c: c3                          	retq
14166084d: 0f 1f 00                    	nopl	(%rax)
141660850: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141660855: 55                          	pushq	%rbp
141660856: 41 57                       	pushq	%r15
141660858: 41 56                       	pushq	%r14
14166085a: 56                          	pushq	%rsi
14166085b: 57                          	pushq	%rdi
14166085c: 53                          	pushq	%rbx
14166085d: 48 83 ec 28                 	subq	$0x28, %rsp
141660861: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141660868: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
14166086c: e8 2f a5 23 ff              	callq	0x14089ada0 <.text+0x899da0>
141660871: 90                          	nop
141660872: 48 83 c4 28                 	addq	$0x28, %rsp
141660876: 5b                          	popq	%rbx
141660877: 5f                          	popq	%rdi
141660878: 5e                          	popq	%rsi
141660879: 41 5e                       	popq	%r14
14166087b: 41 5f                       	popq	%r15
14166087d: 5d                          	popq	%rbp
14166087e: c3                          	retq
14166087f: cc                          	int3
