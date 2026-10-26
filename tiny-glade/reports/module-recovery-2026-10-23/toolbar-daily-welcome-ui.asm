
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141ebedd0: 55                          	pushq	%rbp
141ebedd1: 41 57                       	pushq	%r15
141ebedd3: 41 56                       	pushq	%r14
141ebedd5: 41 55                       	pushq	%r13
141ebedd7: 41 54                       	pushq	%r12
141ebedd9: 56                          	pushq	%rsi
141ebedda: 57                          	pushq	%rdi
141ebeddb: 53                          	pushq	%rbx
141ebeddc: 48 81 ec c8 01 00 00        	subq	$0x1c8, %rsp            # imm = 0x1C8
141ebede3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141ebedeb: 0f 29 b5 30 01 00 00        	movaps	%xmm6, 0x130(%rbp)
141ebedf2: 48 c7 85 28 01 00 00 fe ff ff ff    	movq	$-0x2, 0x128(%rbp)
141ebedfd: 4d 89 ce                    	movq	%r9, %r14
141ebee00: 4d 89 c5                    	movq	%r8, %r13
141ebee03: 48 89 d6                    	movq	%rdx, %rsi
141ebee06: 48 89 cf                    	movq	%rcx, %rdi
141ebee09: e8 f2 7b d5 fe              	callq	0x140c16a00 <.text+0xc15a00>
141ebee0e: 84 c0                       	testb	%al, %al
141ebee10: 74 5d                       	je	0x141ebee6f <.text+0x1ebde6f>
141ebee12: 41 bf 20 00 00 00           	movl	$0x20, %r15d
141ebee18: b9 20 00 00 00              	movl	$0x20, %ecx
141ebee1d: e8 7e fb ff ff              	callq	0x141ebe9a0 <.text+0x1ebd9a0>
141ebee22: 4c 8d 25 0f c3 0d 01        	leaq	0x10dc30f(%rip), %r12   # 0x142f9b138
141ebee29: 48 83 f8 01                 	cmpq	$0x1, %rax
141ebee2d: 0f 85 a8 00 00 00           	jne	0x141ebeedb <.text+0x1ebdedb>
141ebee33: 49 89 d1                    	movq	%rdx, %r9
141ebee36: 48 85 d2                    	testq	%rdx, %rdx
141ebee39: 74 3d                       	je	0x141ebee78 <.text+0x1ebde78>
141ebee3b: 49 83 f9 20                 	cmpq	$0x20, %r9
141ebee3f: 73 35                       	jae	0x141ebee76 <.text+0x1ebde76>
141ebee41: 48 8d 05 f0 c2 0d 01        	leaq	0x10dc2f0(%rip), %rax   # 0x142f9b138
141ebee48: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
141ebee4d: 7f 29                       	jg	0x141ebee78 <.text+0x1ebde78>
141ebee4f: 48 8d 05 8a c3 0d 01        	leaq	0x10dc38a(%rip), %rax   # 0x142f9b1e0
141ebee56: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141ebee5b: 48 8d 0d d6 c2 0d 01        	leaq	0x10dc2d6(%rip), %rcx   # 0x142f9b138
141ebee62: ba 20 00 00 00              	movl	$0x20, %edx
141ebee67: 45 31 c0                    	xorl	%r8d, %r8d
141ebee6a: e8 d1 aa a1 00              	callq	0x1428d9940 <.text+0x28d8940>
141ebee6f: 31 c0                       	xorl	%eax, %eax
141ebee71: e9 d8 01 00 00              	jmp	0x141ebf04e <.text+0x1ebe04e>
141ebee76: 75 d7                       	jne	0x141ebee4f <.text+0x1ebde4f>
141ebee78: 4c 89 c9                    	movq	%r9, %rcx
141ebee7b: e8 20 fb ff ff              	callq	0x141ebe9a0 <.text+0x1ebd9a0>
141ebee80: 48 83 f8 01                 	cmpq	$0x1, %rax
141ebee84: 75 55                       	jne	0x141ebeedb <.text+0x1ebdedb>
141ebee86: 49 89 d0                    	movq	%rdx, %r8
141ebee89: 49 83 c0 02                 	addq	$0x2, %r8
141ebee8d: 74 39                       	je	0x141ebeec8 <.text+0x1ebdec8>
141ebee8f: 49 83 f8 20                 	cmpq	$0x20, %r8
141ebee93: 73 31                       	jae	0x141ebeec6 <.text+0x1ebdec6>
141ebee95: 48 8d 05 9c c2 0d 01        	leaq	0x10dc29c(%rip), %rax   # 0x142f9b138
141ebee9c: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
141ebeea1: 7f 25                       	jg	0x141ebeec8 <.text+0x1ebdec8>
141ebeea3: 48 8d 05 4e c3 0d 01        	leaq	0x10dc34e(%rip), %rax   # 0x142f9b1f8
141ebeeaa: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141ebeeaf: 48 8d 0d 82 c2 0d 01        	leaq	0x10dc282(%rip), %rcx   # 0x142f9b138
141ebeeb6: ba 20 00 00 00              	movl	$0x20, %edx
141ebeebb: 41 b9 20 00 00 00           	movl	$0x20, %r9d
141ebeec1: e8 7a aa a1 00              	callq	0x1428d9940 <.text+0x28d8940>
141ebeec6: 75 db                       	jne	0x141ebeea3 <.text+0x1ebdea3>
141ebeec8: 41 bf 1e 00 00 00           	movl	$0x1e, %r15d
141ebeece: 49 29 d7                    	subq	%rdx, %r15
141ebeed1: 4c 8d 25 60 c2 0d 01        	leaq	0x10dc260(%rip), %r12   # 0x142f9b138
141ebeed8: 4d 01 c4                    	addq	%r8, %r12
141ebeedb: 48 89 bd 20 01 00 00        	movq	%rdi, 0x120(%rbp)
141ebeee2: 4c 89 f7                    	movq	%r14, %rdi
141ebeee5: 4d 89 ee                    	movq	%r13, %r14
141ebeee8: 4c 8d 05 69 c3 0d 01        	leaq	0x10dc369(%rip), %r8    # 0x142f9b258
141ebeeef: 4c 8d 2d 32 c3 0d 01        	leaq	0x10dc332(%rip), %r13   # 0x142f9b228
141ebeef6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141ebef00: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
141ebef05: 85 c0                       	testl	%eax, %eax
141ebef07: 78 17                       	js	0x141ebef20 <.text+0x1ebdf20>
141ebef09: 49 ff c8                    	decq	%r8
141ebef0c: 83 f8 5c                    	cmpl	$0x5c, %eax
141ebef0f: 75 63                       	jne	0x141ebef74 <.text+0x1ebdf74>
141ebef11: eb 72                       	jmp	0x141ebef85 <.text+0x1ebdf85>
141ebef13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
141ebef20: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
141ebef25: 80 f9 c0                    	cmpb	$-0x40, %cl
141ebef28: 7d 23                       	jge	0x141ebef4d <.text+0x1ebdf4d>
141ebef2a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
141ebef2f: 80 fa c0                    	cmpb	$-0x40, %dl
141ebef32: 7d 22                       	jge	0x141ebef56 <.text+0x1ebdf56>
141ebef34: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
141ebef39: 49 83 c0 fc                 	addq	$-0x4, %r8
141ebef3d: 41 83 e1 07                 	andl	$0x7, %r9d
141ebef41: 41 c1 e1 06                 	shll	$0x6, %r9d
141ebef45: 83 e2 3f                    	andl	$0x3f, %edx
141ebef48: 44 09 ca                    	orl	%r9d, %edx
141ebef4b: eb 10                       	jmp	0x141ebef5d <.text+0x1ebdf5d>
141ebef4d: 49 83 c0 fe                 	addq	$-0x2, %r8
141ebef51: 83 e1 1f                    	andl	$0x1f, %ecx
141ebef54: eb 0f                       	jmp	0x141ebef65 <.text+0x1ebdf65>
141ebef56: 49 83 c0 fd                 	addq	$-0x3, %r8
141ebef5a: 83 e2 0f                    	andl	$0xf, %edx
141ebef5d: c1 e2 06                    	shll	$0x6, %edx
141ebef60: 83 e1 3f                    	andl	$0x3f, %ecx
141ebef63: 09 d1                       	orl	%edx, %ecx
141ebef65: c1 e1 06                    	shll	$0x6, %ecx
141ebef68: 24 3f                       	andb	$0x3f, %al
141ebef6a: 0f b6 c0                    	movzbl	%al, %eax
141ebef6d: 09 c8                       	orl	%ecx, %eax
141ebef6f: 83 f8 5c                    	cmpl	$0x5c, %eax
141ebef72: 74 11                       	je	0x141ebef85 <.text+0x1ebdf85>
141ebef74: 83 f8 2f                    	cmpl	$0x2f, %eax
141ebef77: 74 0c                       	je	0x141ebef85 <.text+0x1ebdf85>
141ebef79: 4d 39 e8                    	cmpq	%r13, %r8
141ebef7c: 75 82                       	jne	0x141ebef00 <.text+0x1ebdf00>
141ebef7e: bb 30 00 00 00              	movl	$0x30, %ebx
141ebef83: eb 4c                       	jmp	0x141ebefd1 <.text+0x1ebdfd1>
141ebef85: 4c 8d 2d 9c c2 0d 01        	leaq	0x10dc29c(%rip), %r13   # 0x142f9b228
141ebef8c: 4d 29 e8                    	subq	%r13, %r8
141ebef8f: 49 ff c0                    	incq	%r8
141ebef92: 74 32                       	je	0x141ebefc6 <.text+0x1ebdfc6>
141ebef94: 49 83 f8 30                 	cmpq	$0x30, %r8
141ebef98: 73 2a                       	jae	0x141ebefc4 <.text+0x1ebdfc4>
141ebef9a: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
141ebef9f: 7f 25                       	jg	0x141ebefc6 <.text+0x1ebdfc6>
141ebefa1: 48 8d 05 18 c2 0d 01        	leaq	0x10dc218(%rip), %rax   # 0x142f9b1c0
141ebefa8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141ebefad: 48 8d 0d 74 c2 0d 01        	leaq	0x10dc274(%rip), %rcx   # 0x142f9b228
141ebefb4: ba 30 00 00 00              	movl	$0x30, %edx
141ebefb9: 41 b9 30 00 00 00           	movl	$0x30, %r9d
141ebefbf: e8 7c a9 a1 00              	callq	0x1428d9940 <.text+0x28d8940>
141ebefc4: 75 db                       	jne	0x141ebefa1 <.text+0x1ebdfa1>
141ebefc6: bb 30 00 00 00              	movl	$0x30, %ebx
141ebefcb: 4c 29 c3                    	subq	%r8, %rbx
141ebefce: 4d 01 c5                    	addq	%r8, %r13
141ebefd1: e8 5a 91 d5 fe              	callq	0x140c18130 <.text+0xc17130>
141ebefd6: 48 89 c1                    	movq	%rax, %rcx
141ebefd9: 48 8b 00                    	movq	(%rax), %rax
141ebefdc: 48 83 f8 01                 	cmpq	$0x1, %rax
141ebefe0: 0f 85 4d 04 00 00           	jne	0x141ebf433 <.text+0x1ebe433>
141ebefe6: 48 83 c1 08                 	addq	$0x8, %rcx
141ebefea: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141ebefee: 0f 85 31 09 00 00           	jne	0x141ebf925 <.text+0x1ebe925>
141ebeff4: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141ebeffb: 48 89 8d 18 01 00 00        	movq	%rcx, 0x118(%rbp)
141ebf002: 48 83 c1 08                 	addq	$0x8, %rcx
141ebf006: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
141ebf00b: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
141ebf014: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
141ebf01d: 4c 89 e2                    	movq	%r12, %rdx
141ebf020: 4d 89 f8                    	movq	%r15, %r8
141ebf023: 4d 89 e9                    	movq	%r13, %r9
141ebf026: e8 15 7d d5 fe              	callq	0x140c16d40 <.text+0xc15d40>
141ebf02b: 48 8b 8d 18 01 00 00        	movq	0x118(%rbp), %rcx
141ebf032: 48 ff 01                    	incq	(%rcx)
141ebf035: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
141ebf03c: b8 01 00 00 00              	movl	$0x1, %eax
141ebf041: 4d 89 f5                    	movq	%r14, %r13
141ebf044: 49 89 fe                    	movq	%rdi, %r14
141ebf047: 48 8b bd 20 01 00 00        	movq	0x120(%rbp), %rdi
141ebf04e: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
141ebf055: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
141ebf05c: 48 8b 09                    	movq	(%rcx), %rcx
141ebf05f: 48 85 c9                    	testq	%rcx, %rcx
141ebf062: 74 62                       	je	0x141ebf0c6 <.text+0x1ebe0c6>
141ebf064: e8 17 ed 7b fe              	callq	0x14067dd80 <.text+0x67cd80>
141ebf069: 49 89 c7                    	movq	%rax, %r15
141ebf06c: e8 ff 44 a6 00              	callq	0x142923570 <.text+0x2922570>
141ebf071: 49 8b 1f                    	movq	(%r15), %rbx
141ebf074: f0                          	lock
141ebf075: 48 ff 03                    	incq	(%rbx)
141ebf078: 0f 8e b3 03 00 00           	jle	0x141ebf431 <.text+0x1ebe431>
141ebf07e: 48 89 5d d0                 	movq	%rbx, -0x30(%rbp)
141ebf082: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
141ebf086: 48 89 c1                    	movq	%rax, %rcx
141ebf089: e8 62 44 a6 00              	callq	0x1429234f0 <.text+0x29224f0>
141ebf08e: f0                          	lock
141ebf08f: 48 ff 0b                    	decq	(%rbx)
141ebf092: 0f 94 c1                    	sete	%cl
141ebf095: 84 c0                       	testb	%al, %al
141ebf097: 74 20                       	je	0x141ebf0b9 <.text+0x1ebe0b9>
141ebf099: 84 c9                       	testb	%cl, %cl
141ebf09b: 74 09                       	je	0x141ebf0a6 <.text+0x1ebe0a6>
141ebf09d: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141ebf0a1: e8 aa 62 cc fe              	callq	0x140b85350 <.text+0xb84350>
141ebf0a6: 48 83 bd c8 00 00 00 00     	cmpq	$0x0, 0xc8(%rbp)
141ebf0ae: 0f 85 6d 02 00 00           	jne	0x141ebf321 <.text+0x1ebe321>
141ebf0b4: e9 6e 07 00 00              	jmp	0x141ebf827 <.text+0x1ebe827>
141ebf0b9: 84 c9                       	testb	%cl, %cl
141ebf0bb: 74 09                       	je	0x141ebf0c6 <.text+0x1ebe0c6>
141ebf0bd: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141ebf0c1: e8 8a 62 cc fe              	callq	0x140b85350 <.text+0xb84350>
141ebf0c6: 8b 46 1c                    	movl	0x1c(%rsi), %eax
141ebf0c9: 48 8b 4e 10                 	movq	0x10(%rsi), %rcx
141ebf0cd: 89 01                       	movl	%eax, (%rcx)
141ebf0cf: 4c 8b 3e                    	movq	(%rsi), %r15
141ebf0d2: 41 f6 07 01                 	testb	$0x1, (%r15)
141ebf0d6: 74 6f                       	je	0x141ebf147 <.text+0x1ebe147>
141ebf0d8: 4c 8b 85 b0 01 00 00        	movq	0x1b0(%rbp), %r8
141ebf0df: 49 8b 10                    	movq	(%r8), %rdx
141ebf0e2: 4d 8b 40 08                 	movq	0x8(%r8), %r8
141ebf0e6: 4c 8b 0a                    	movq	(%rdx), %r9
141ebf0e9: 45 31 d2                    	xorl	%r10d, %r10d
141ebf0ec: 4d 89 cb                    	movq	%r9, %r11
141ebf0ef: 4d 2b 58 18                 	subq	0x18(%r8), %r11
141ebf0f3: 4d 0f 42 da                 	cmovbq	%r10, %r11
141ebf0f7: 4d 2b 48 38                 	subq	0x38(%r8), %r9
141ebf0fb: 4d 0f 42 ca                 	cmovbq	%r10, %r9
141ebf0ff: 49 8b 70 30                 	movq	0x30(%r8), %rsi
141ebf103: 4c 29 ce                    	subq	%r9, %rsi
141ebf106: 49 0f 42 f2                 	cmovbq	%r10, %rsi
141ebf10a: 41 0f 96 c1                 	setbe	%r9b
141ebf10e: 49 8b 78 10                 	movq	0x10(%r8), %rdi
141ebf112: 4c 29 df                    	subq	%r11, %rdi
141ebf115: 49 0f 42 fa                 	cmovbq	%r10, %rdi
141ebf119: 4d 8b 40 40                 	movq	0x40(%r8), %r8
141ebf11d: 41 0f 96 c2                 	setbe	%r10b
141ebf121: 48 01 f7                    	addq	%rsi, %rdi
141ebf124: 4d 89 c3                    	movq	%r8, %r11
141ebf127: 49 29 fb                    	subq	%rdi, %r11
141ebf12a: 4c 89 1a                    	movq	%r11, (%rdx)
141ebf12d: 45 84 ca                    	testb	%r9b, %r10b
141ebf130: 0f 85 9c 06 00 00           	jne	0x141ebf7d2 <.text+0x1ebe7d2>
141ebf136: 4c 89 02                    	movq	%r8, (%rdx)
141ebf139: 89 01                       	movl	%eax, (%rcx)
141ebf13b: 49 c7 07 00 00 00 00        	movq	$0x0, (%r15)
141ebf142: e9 8b 06 00 00              	jmp	0x141ebf7d2 <.text+0x1ebe7d2>
141ebf147: 49 8b 06                    	movq	(%r14), %rax
141ebf14a: 83 38 02                    	cmpl	$0x2, (%rax)
141ebf14d: 0f 85 b9 01 00 00           	jne	0x141ebf30c <.text+0x1ebe30c>
141ebf153: c7 85 c4 00 00 00 00 00 90 41       	movl	$0x41900000, 0xc4(%rbp) # imm = 0x41900000
141ebf15d: 8b 47 1c                    	movl	0x1c(%rdi), %eax
141ebf160: 48 8b 4f 10                 	movq	0x10(%rdi), %rcx
141ebf164: 89 01                       	movl	%eax, (%rcx)
141ebf166: 48 8b 3f                    	movq	(%rdi), %rdi
141ebf169: 48 8d 05 e8 c0 0d 01        	leaq	0x10dc0e8(%rip), %rax   # 0x142f9b258
141ebf170: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
141ebf177: 48 c7 85 90 00 00 00 07 00 00 00    	movq	$0x7, 0x90(%rbp)
141ebf182: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf189: e8 12 ec 6f fe              	callq	0x1405bdda0 <.text+0x5bcda0>
141ebf18e: 49 89 c6                    	movq	%rax, %r14
141ebf191: 48 b8 43 b4 4f 00 20 53 69 b1       	movabsq	$-0x4e96acdfffb04bbd, %rax # imm = 0xB1695320004FB443
141ebf19b: 49 31 c6                    	xorq	%rax, %r14
141ebf19e: 48 89 f9                    	movq	%rdi, %rcx
141ebf1a1: 4c 89 f2                    	movq	%r14, %rdx
141ebf1a4: e8 17 4d 7c fe              	callq	0x140683ec0 <.text+0x682ec0>
141ebf1a9: c6 40 5d 00                 	movb	$0x0, 0x5d(%rax)
141ebf1ad: c6 80 b7 00 00 00 01        	movb	$0x1, 0xb7(%rax)
141ebf1b4: f6 c2 01                    	testb	$0x1, %dl
141ebf1b7: 75 33                       	jne	0x141ebf1ec <.text+0x1ebe1ec>
141ebf1b9: 48 8b 5f 10                 	movq	0x10(%rdi), %rbx
141ebf1bd: 48 3b 1f                    	cmpq	(%rdi), %rbx
141ebf1c0: 75 0f                       	jne	0x141ebf1d1 <.text+0x1ebe1d1>
141ebf1c2: 48 8d 15 ff fd ce 00        	leaq	0xcefdff(%rip), %rdx    # 0x142baefc8
141ebf1c9: 48 89 f9                    	movq	%rdi, %rcx
141ebf1cc: e8 ff 27 77 fe              	callq	0x1406319d0 <.text+0x6309d0>
141ebf1d1: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141ebf1d5: 48 6b cb 38                 	imulq	$0x38, %rbx, %rcx
141ebf1d9: c7 04 08 01 00 00 00        	movl	$0x1, (%rax,%rcx)
141ebf1e0: 4c 89 74 08 30              	movq	%r14, 0x30(%rax,%rcx)
141ebf1e5: 48 ff c3                    	incq	%rbx
141ebf1e8: 48 89 5f 10                 	movq	%rbx, 0x10(%rdi)
141ebf1ec: 48 89 7d d0                 	movq	%rdi, -0x30(%rbp)
141ebf1f0: 4c 89 75 d8                 	movq	%r14, -0x28(%rbp)
141ebf1f4: 48 c7 45 e0 00 00 00 00     	movq	$0x0, -0x20(%rbp)
141ebf1fc: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
141ebf205: 4c 8d 05 53 c0 0d 01        	leaq	0x10dc053(%rip), %r8    # 0x142f9b25f
141ebf20c: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf213: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141ebf217: 41 b9 12 00 00 00           	movl	$0x12, %r9d
141ebf21d: e8 4e f7 0d ff              	callq	0x140f9e970 <.text+0xf9d970>
141ebf222: 49 8b 45 00                 	movq	(%r13), %rax
141ebf226: f3 0f 10 70 28              	movss	0x28(%rax), %xmm6
141ebf22b: f3 41 0f 58 77 04           	addss	0x4(%r15), %xmm6
141ebf231: f3 41 0f 11 77 04           	movss	%xmm6, 0x4(%r15)
141ebf237: 48 8b bd 98 00 00 00        	movq	0x98(%rbp), %rdi
141ebf23e: 48 83 bf b8 01 00 00 00     	cmpq	$0x0, 0x1b8(%rdi)
141ebf246: 0f 84 d9 01 00 00           	je	0x141ebf425 <.text+0x1ebe425>
141ebf24c: 48 8d 95 a0 00 00 00        	leaq	0xa0(%rbp), %rdx
141ebf253: 48 8d 8f c0 01 00 00        	leaq	0x1c0(%rdi), %rcx
141ebf25a: e8 11 1b 6f fe              	callq	0x1405b0d70 <.text+0x5afd70>
141ebf25f: 48 8b 9d b8 01 00 00        	movq	0x1b8(%rbp), %rbx
141ebf266: 48 8b 8f a0 01 00 00        	movq	0x1a0(%rdi), %rcx
141ebf26d: 48 8b 97 a8 01 00 00        	movq	0x1a8(%rdi), %rdx
141ebf274: 49 89 d0                    	movq	%rdx, %r8
141ebf277: 49 21 c0                    	andq	%rax, %r8
141ebf27a: 48 c1 e8 39                 	shrq	$0x39, %rax
141ebf27e: 66 0f 6e c0                 	movd	%eax, %xmm0
141ebf282: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
141ebf286: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
141ebf28b: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
141ebf290: 48 8d 81 40 ff ff ff        	leaq	-0xc0(%rcx), %rax
141ebf297: 4c 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %r9
141ebf29e: 45 31 d2                    	xorl	%r10d, %r10d
141ebf2a1: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
141ebf2a5: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
141ebf2ab: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
141ebf2af: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
141ebf2b3: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
141ebf2b8: 45 85 db                    	testl	%r11d, %r11d
141ebf2bb: 74 2d                       	je	0x141ebf2ea <.text+0x1ebe2ea>
141ebf2bd: f3 41 0f bc fb              	tzcntl	%r11d, %edi
141ebf2c2: 4c 01 c7                    	addq	%r8, %rdi
141ebf2c5: 48 21 d7                    	andq	%rdx, %rdi
141ebf2c8: 48 f7 df                    	negq	%rdi
141ebf2cb: 48 8d 3c 7f                 	leaq	(%rdi,%rdi,2), %rdi
141ebf2cf: 48 c1 e7 06                 	shlq	$0x6, %rdi
141ebf2d3: 4c 3b 0c 38                 	cmpq	(%rax,%rdi), %r9
141ebf2d7: 0f 84 8a 00 00 00           	je	0x141ebf367 <.text+0x1ebe367>
141ebf2dd: 41 8d 7b ff                 	leal	-0x1(%r11), %edi
141ebf2e1: 66 44 21 df                 	andw	%r11w, %di
141ebf2e5: 41 89 fb                    	movl	%edi, %r11d
141ebf2e8: 75 d3                       	jne	0x141ebf2bd <.text+0x1ebe2bd>
141ebf2ea: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
141ebf2ee: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
141ebf2f3: 45 85 db                    	testl	%r11d, %r11d
141ebf2f6: 0f 85 29 01 00 00           	jne	0x141ebf425 <.text+0x1ebe425>
141ebf2fc: 4d 01 d0                    	addq	%r10, %r8
141ebf2ff: 49 83 c0 10                 	addq	$0x10, %r8
141ebf303: 49 83 c2 10                 	addq	$0x10, %r10
141ebf307: 49 21 d0                    	andq	%rdx, %r8
141ebf30a: eb 99                       	jmp	0x141ebf2a5 <.text+0x1ebe2a5>
141ebf30c: 41 c7 07 01 00 00 00        	movl	$0x1, (%r15)
141ebf313: 48 83 bd c8 00 00 00 00     	cmpq	$0x0, 0xc8(%rbp)
141ebf31b: 0f 84 06 05 00 00           	je	0x141ebf827 <.text+0x1ebe827>
141ebf321: e8 0a 8e d5 fe              	callq	0x140c18130 <.text+0xc17130>
141ebf326: 48 89 c1                    	movq	%rax, %rcx
141ebf329: 48 8b 00                    	movq	(%rax), %rax
141ebf32c: 48 83 f8 01                 	cmpq	$0x1, %rax
141ebf330: 0f 85 e4 03 00 00           	jne	0x141ebf71a <.text+0x1ebe71a>
141ebf336: 48 83 c1 08                 	addq	$0x8, %rcx
141ebf33a: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141ebf33e: 0f 85 e1 05 00 00           	jne	0x141ebf925 <.text+0x1ebe925>
141ebf344: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141ebf34b: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
141ebf352: 48 83 c1 08                 	addq	$0x8, %rcx
141ebf356: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
141ebf35d: e8 8e 7d d5 fe              	callq	0x140c170f0 <.text+0xc160f0>
141ebf362: e9 b6 04 00 00              	jmp	0x141ebf81d <.text+0x1ebe81d>
141ebf367: f3 0f 58 35 11 bd 0d 01     	addss	0x10dbd11(%rip), %xmm6  # 0x142f9b080
141ebf36f: f3 0f 5e 35 61 65 a6 00     	divss	0xa66561(%rip), %xmm6   # 0x1429258d8
141ebf377: 66 0f ef c0                 	pxor	%xmm0, %xmm0
141ebf37b: f3 0f 5f c6                 	maxss	%xmm6, %xmm0
141ebf37f: f3 0f 10 0d 7d 65 a6 00     	movss	0xa6657d(%rip), %xmm1   # 0x142925904
141ebf387: f3 0f 5d c8                 	minss	%xmm0, %xmm1
141ebf38b: 0f 28 c1                    	movaps	%xmm1, %xmm0
141ebf38e: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
141ebf392: f3 0f 58 c9                 	addss	%xmm1, %xmm1
141ebf396: f3 0f 10 15 da dc b0 00     	movss	0xb0dcda(%rip), %xmm2   # 0x1429cd078
141ebf39e: f3 0f 5c d1                 	subss	%xmm1, %xmm2
141ebf3a2: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
141ebf3a6: f3 0f 11 54 39 f8           	movss	%xmm2, -0x8(%rcx,%rdi)
141ebf3ac: 48 8d 15 be be 0d 01        	leaq	0x10dbebe(%rip), %rdx   # 0x142f9b271
141ebf3b3: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf3ba: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141ebf3c0: 49 89 d9                    	movq	%rbx, %r9
141ebf3c3: e8 68 f4 0f ff              	callq	0x140fbe830 <.text+0xfbd830>
141ebf3c8: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
141ebf3cd: 48 8d 15 a2 be 0d 01        	leaq	0x10dbea2(%rip), %rdx   # 0x142f9b276
141ebf3d4: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf3db: 4c 8d 8d c4 00 00 00        	leaq	0xc4(%rbp), %r9
141ebf3e2: 41 b8 07 00 00 00           	movl	$0x7, %r8d
141ebf3e8: e8 e3 0d 10 ff              	callq	0x140fc01d0 <.text+0xfbf1d0>
141ebf3ed: 48 8b 03                    	movq	(%rbx), %rax
141ebf3f0: 31 d2                       	xorl	%edx, %edx
141ebf3f2: 31 c9                       	xorl	%ecx, %ecx
141ebf3f4: 48 3b 10                    	cmpq	(%rax), %rdx
141ebf3f7: 0f 90 c2                    	seto	%dl
141ebf3fa: 0f 80 d2 03 00 00           	jo	0x141ebf7d2 <.text+0x1ebe7d2>
141ebf400: 88 d1                       	movb	%dl, %cl
141ebf402: 48 8d 34 c8                 	leaq	(%rax,%rcx,8), %rsi
141ebf406: 48 89 f1                    	movq	%rsi, %rcx
141ebf409: e8 32 d0 97 fe              	callq	0x14083c440 <.text+0x83b440>
141ebf40e: 84 c0                       	testb	%al, %al
141ebf410: 74 42                       	je	0x141ebf454 <.text+0x1ebe454>
141ebf412: 48 8d 0d 78 be 0d 01        	leaq	0x10dbe78(%rip), %rcx   # 0x142f9b291
141ebf419: ba 1b 00 00 00              	movl	$0x1b, %edx
141ebf41e: e8 5d e0 cd fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebf423: eb 40                       	jmp	0x141ebf465 <.text+0x1ebe465>
141ebf425: 48 8d 0d c4 8b b5 00        	leaq	0xb58bc4(%rip), %rcx    # 0x142a17ff0
141ebf42c: e8 df 9e a1 00              	callq	0x1428d9310 <.text+0x28d8310>
141ebf431: 0f 0b                       	ud2
141ebf433: 48 85 c0                    	testq	%rax, %rax
141ebf436: 0f 85 57 04 00 00           	jne	0x141ebf893 <.text+0x1ebe893>
141ebf43c: 31 d2                       	xorl	%edx, %edx
141ebf43e: e8 0d 6e a2 00              	callq	0x1428e6250 <.text+0x28e5250>
141ebf443: 48 89 c1                    	movq	%rax, %rcx
141ebf446: 48 85 c0                    	testq	%rax, %rax
141ebf449: 0f 85 9b fb ff ff           	jne	0x141ebefea <.text+0x1ebdfea>
141ebf44f: e9 3f 04 00 00              	jmp	0x141ebf893 <.text+0x1ebe893>
141ebf454: 48 8d 0d 22 be 0d 01        	leaq	0x10dbe22(%rip), %rcx   # 0x142f9b27d
141ebf45b: ba 14 00 00 00              	movl	$0x14, %edx
141ebf460: e8 1b e0 cd fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebf465: 48 89 54 24 20              	movq	%rdx, 0x20(%rsp)
141ebf46a: 48 8d 15 3b be 0d 01        	leaq	0x10dbe3b(%rip), %rdx   # 0x142f9b2ac
141ebf471: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf478: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141ebf47e: 49 89 c1                    	movq	%rax, %r9
141ebf481: e8 7a c9 0f ff              	callq	0x140fbbe00 <.text+0xfbae00>
141ebf486: 48 8d 0d 25 be 0d 01        	leaq	0x10dbe25(%rip), %rcx   # 0x142f9b2b2
141ebf48d: ba 12 00 00 00              	movl	$0x12, %edx
141ebf492: e8 e9 df cd fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebf497: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
141ebf49e: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
141ebf4a5: 31 c0                       	xorl	%eax, %eax
141ebf4a7: 48 3b 86 c8 00 00 00        	cmpq	0xc8(%rsi), %rax
141ebf4ae: 71 0c                       	jno	0x141ebf4bc <.text+0x1ebe4bc>
141ebf4b0: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141ebf4ba: eb 14                       	jmp	0x141ebf4d0 <.text+0x1ebe4d0>
141ebf4bc: 48 8d 96 c8 00 00 00        	leaq	0xc8(%rsi), %rdx
141ebf4c3: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141ebf4c7: e8 c4 c8 eb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
141ebf4cc: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141ebf4d0: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
141ebf4d7: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
141ebf4db: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
141ebf4e2: 48 8b 5d e0                 	movq	-0x20(%rbp), %rbx
141ebf4e6: 48 8d 0d db bd 0d 01        	leaq	0x10dbddb(%rip), %rcx   # 0x142f9b2c8
141ebf4ed: ba 14 00 00 00              	movl	$0x14, %edx
141ebf4f2: e8 89 df cd fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebf4f7: 49 89 c7                    	movq	%rax, %r15
141ebf4fa: 49 89 d4                    	movq	%rdx, %r12
141ebf4fd: 48 8d 96 98 00 00 00        	leaq	0x98(%rsi), %rdx
141ebf504: 31 c0                       	xorl	%eax, %eax
141ebf506: 48 3b 02                    	cmpq	(%rdx), %rax
141ebf509: 71 0c                       	jno	0x141ebf517 <.text+0x1ebe517>
141ebf50b: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141ebf515: eb 0d                       	jmp	0x141ebf524 <.text+0x1ebe524>
141ebf517: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141ebf51b: e8 70 c8 eb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
141ebf520: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
141ebf524: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
141ebf52b: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
141ebf52f: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
141ebf536: 4c 8b 75 e0                 	movq	-0x20(%rbp), %r14
141ebf53a: 48 8d 0d a1 bd 0d 01        	leaq	0x10dbda1(%rip), %rcx   # 0x142f9b2e2
141ebf541: ba 19 00 00 00              	movl	$0x19, %edx
141ebf546: e8 35 df cd fe              	callq	0x140b9d480 <.text+0xb9c480>
141ebf54b: 49 89 c5                    	movq	%rax, %r13
141ebf54e: 48 89 d7                    	movq	%rdx, %rdi
141ebf551: 31 c0                       	xorl	%eax, %eax
141ebf553: 48 3b 86 b0 00 00 00        	cmpq	0xb0(%rsi), %rax
141ebf55a: 71 13                       	jno	0x141ebf56f <.text+0x1ebe56f>
141ebf55c: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141ebf566: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
141ebf56d: eb 16                       	jmp	0x141ebf585 <.text+0x1ebe585>
141ebf56f: 48 81 c6 b0 00 00 00        	addq	$0xb0, %rsi
141ebf576: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
141ebf57d: 48 89 f2                    	movq	%rsi, %rdx
141ebf580: e8 0b c8 eb fe              	callq	0x140d7bd90 <.text+0xd7ad90>
141ebf585: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
141ebf58c: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
141ebf593: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
141ebf59a: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
141ebf59e: 48 8b 85 08 01 00 00        	movq	0x108(%rbp), %rax
141ebf5a5: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
141ebf5a9: 48 c7 45 d0 00 00 00 00     	movq	$0x0, -0x30(%rbp)
141ebf5b1: 48 c7 45 d8 03 00 00 00     	movq	$0x3, -0x28(%rbp)
141ebf5b9: 48 8d 15 04 bd 0d 01        	leaq	0x10dbd04(%rip), %rdx   # 0x142f9b2c4
141ebf5c0: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
141ebf5c4: 48 c7 45 e8 04 00 00 00     	movq	$0x4, -0x18(%rbp)
141ebf5cc: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
141ebf5d3: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
141ebf5d7: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
141ebf5de: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
141ebf5e2: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
141ebf5e9: 48 89 4d 00                 	movq	%rcx, (%rbp)
141ebf5ed: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
141ebf5f4: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
141ebf5f8: 48 89 5d 10                 	movq	%rbx, 0x10(%rbp)
141ebf5fc: 48 8d 05 d9 bc 0d 01        	leaq	0x10dbcd9(%rip), %rax   # 0x142f9b2dc
141ebf603: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
141ebf607: 48 c7 45 20 06 00 00 00     	movq	$0x6, 0x20(%rbp)
141ebf60f: 4c 89 7d 28                 	movq	%r15, 0x28(%rbp)
141ebf613: 4c 89 65 30                 	movq	%r12, 0x30(%rbp)
141ebf617: 4c 8b 85 f8 00 00 00        	movq	0xf8(%rbp), %r8
141ebf61e: 4c 89 45 38                 	movq	%r8, 0x38(%rbp)
141ebf622: 4c 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %r8
141ebf629: 4c 89 45 40                 	movq	%r8, 0x40(%rbp)
141ebf62d: 4c 89 75 48                 	movq	%r14, 0x48(%rbp)
141ebf631: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
141ebf635: 48 c7 45 58 06 00 00 00     	movq	$0x6, 0x58(%rbp)
141ebf63d: 4c 89 6d 60                 	movq	%r13, 0x60(%rbp)
141ebf641: 48 89 7d 68                 	movq	%rdi, 0x68(%rbp)
141ebf645: 48 c7 45 d0 01 00 00 00     	movq	$0x1, -0x30(%rbp)
141ebf64d: 48 bf 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rdi # imm = 0x8000000000000001
141ebf657: 48 39 f9                    	cmpq	%rdi, %rcx
141ebf65a: 75 15                       	jne	0x141ebf671 <.text+0x1ebe671>
141ebf65c: b8 01 00 00 00              	movl	$0x1, %eax
141ebf661: be 02 00 00 00              	movl	$0x2, %esi
141ebf666: 31 c9                       	xorl	%ecx, %ecx
141ebf668: 48 8d 5d d0                 	leaq	-0x30(%rbp), %rbx
141ebf66c: e9 5d 01 00 00              	jmp	0x141ebf7ce <.text+0x1ebe7ce>
141ebf671: 48 89 c8                    	movq	%rcx, %rax
141ebf674: 48 8d b5 e0 00 00 00        	leaq	0xe0(%rbp), %rsi
141ebf67b: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
141ebf682: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
141ebf686: 0f 11 06                    	movups	%xmm0, (%rsi)
141ebf689: 48 f7 d8                    	negq	%rax
141ebf68c: 48 8d 5d d0                 	leaq	-0x30(%rbp), %rbx
141ebf690: 70 66                       	jo	0x141ebf6f8 <.text+0x1ebe6f8>
141ebf692: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
141ebf699: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
141ebf6a0: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
141ebf6a7: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
141ebf6ae: 48 8d 85 d8 00 00 00        	leaq	0xd8(%rbp), %rax
141ebf6b5: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebf6bc: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf6c3: 4c 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %r9
141ebf6ca: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141ebf6d0: e8 5b 99 0f ff              	callq	0x140fb9030 <.text+0xfb8030>
141ebf6d5: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141ebf6dc: 48 89 d0                    	movq	%rdx, %rax
141ebf6df: 48 f7 d8                    	negq	%rax
141ebf6e2: 70 14                       	jo	0x141ebf6f8 <.text+0x1ebe6f8>
141ebf6e4: 73 12                       	jae	0x141ebf6f8 <.text+0x1ebe6f8>
141ebf6e6: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141ebf6ed: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebf6f3: e8 28 45 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebf6f8: 48 c7 45 d0 02 00 00 00     	movq	$0x2, -0x30(%rbp)
141ebf700: 48 8b 45 38                 	movq	0x38(%rbp), %rax
141ebf704: 48 39 f8                    	cmpq	%rdi, %rax
141ebf707: 75 32                       	jne	0x141ebf73b <.text+0x1ebe73b>
141ebf709: b8 02 00 00 00              	movl	$0x2, %eax
141ebf70e: be 01 00 00 00              	movl	$0x1, %esi
141ebf713: 31 c9                       	xorl	%ecx, %ecx
141ebf715: e9 b4 00 00 00              	jmp	0x141ebf7ce <.text+0x1ebe7ce>
141ebf71a: 48 85 c0                    	testq	%rax, %rax
141ebf71d: 0f 85 70 01 00 00           	jne	0x141ebf893 <.text+0x1ebe893>
141ebf723: 31 d2                       	xorl	%edx, %edx
141ebf725: e8 26 6b a2 00              	callq	0x1428e6250 <.text+0x28e5250>
141ebf72a: 48 89 c1                    	movq	%rax, %rcx
141ebf72d: 48 85 c0                    	testq	%rax, %rax
141ebf730: 0f 85 04 fc ff ff           	jne	0x141ebf33a <.text+0x1ebe33a>
141ebf736: e9 58 01 00 00              	jmp	0x141ebf893 <.text+0x1ebe893>
141ebf73b: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
141ebf73f: 4c 8b 45 20                 	movq	0x20(%rbp), %r8
141ebf743: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
141ebf747: 4c 8b 4d 30                 	movq	0x30(%rbp), %r9
141ebf74b: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
141ebf752: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
141ebf756: 0f 11 06                    	movups	%xmm0, (%rsi)
141ebf759: 48 f7 d8                    	negq	%rax
141ebf75c: 70 52                       	jo	0x141ebf7b0 <.text+0x1ebe7b0>
141ebf75e: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
141ebf765: 4c 89 8d 08 01 00 00        	movq	%r9, 0x108(%rbp)
141ebf76c: 48 8d 85 d8 00 00 00        	leaq	0xd8(%rbp), %rax
141ebf773: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebf77a: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf781: 4c 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %r9
141ebf788: e8 a3 98 0f ff              	callq	0x140fb9030 <.text+0xfb8030>
141ebf78d: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141ebf794: 48 89 d0                    	movq	%rdx, %rax
141ebf797: 48 f7 d8                    	negq	%rax
141ebf79a: 70 14                       	jo	0x141ebf7b0 <.text+0x1ebe7b0>
141ebf79c: 73 12                       	jae	0x141ebf7b0 <.text+0x1ebe7b0>
141ebf79e: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141ebf7a5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebf7ab: e8 70 44 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebf7b0: 48 c7 45 d0 03 00 00 00     	movq	$0x3, -0x30(%rbp)
141ebf7b8: 48 8b 45 70                 	movq	0x70(%rbp), %rax
141ebf7bc: 48 39 f8                    	cmpq	%rdi, %rax
141ebf7bf: 0f 85 da 00 00 00           	jne	0x141ebf89f <.text+0x1ebe89f>
141ebf7c5: b8 03 00 00 00              	movl	$0x3, %eax
141ebf7ca: b1 01                       	movb	$0x1, %cl
141ebf7cc: 31 f6                       	xorl	%esi, %esi
141ebf7ce: 84 c9                       	testb	%cl, %cl
141ebf7d0: 74 70                       	je	0x141ebf842 <.text+0x1ebe842>
141ebf7d2: 48 83 bd c8 00 00 00 00     	cmpq	$0x0, 0xc8(%rbp)
141ebf7da: 74 4b                       	je	0x141ebf827 <.text+0x1ebe827>
141ebf7dc: e8 4f 89 d5 fe              	callq	0x140c18130 <.text+0xc17130>
141ebf7e1: 48 89 c1                    	movq	%rax, %rcx
141ebf7e4: 48 8b 00                    	movq	(%rax), %rax
141ebf7e7: 48 83 f8 01                 	cmpq	$0x1, %rax
141ebf7eb: 0f 85 8a 00 00 00           	jne	0x141ebf87b <.text+0x1ebe87b>
141ebf7f1: 48 83 c1 08                 	addq	$0x8, %rcx
141ebf7f5: 48 83 39 00                 	cmpq	$0x0, (%rcx)
141ebf7f9: 0f 85 26 01 00 00           	jne	0x141ebf925 <.text+0x1ebe925>
141ebf7ff: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
141ebf806: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
141ebf80d: 48 83 c1 08                 	addq	$0x8, %rcx
141ebf811: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
141ebf818: e8 d3 78 d5 fe              	callq	0x140c170f0 <.text+0xc160f0>
141ebf81d: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
141ebf824: 48 ff 00                    	incq	(%rax)
141ebf827: 0f 28 b5 30 01 00 00        	movaps	0x130(%rbp), %xmm6
141ebf82e: 48 81 c4 c8 01 00 00        	addq	$0x1c8, %rsp            # imm = 0x1C8
141ebf835: 5b                          	popq	%rbx
141ebf836: 5f                          	popq	%rdi
141ebf837: 5e                          	popq	%rsi
141ebf838: 41 5c                       	popq	%r12
141ebf83a: 41 5d                       	popq	%r13
141ebf83c: 41 5e                       	popq	%r14
141ebf83e: 41 5f                       	popq	%r15
141ebf840: 5d                          	popq	%rbp
141ebf841: c3                          	retq
141ebf842: 48 6b c0 38                 	imulq	$0x38, %rax, %rax
141ebf846: 48 8d 3c 18                 	leaq	(%rax,%rbx), %rdi
141ebf84a: 48 83 c7 38                 	addq	$0x38, %rdi
141ebf84e: eb 0d                       	jmp	0x141ebf85d <.text+0x1ebe85d>
141ebf850: 48 83 c7 38                 	addq	$0x38, %rdi
141ebf854: 48 ff ce                    	decq	%rsi
141ebf857: 0f 84 75 ff ff ff           	je	0x141ebf7d2 <.text+0x1ebe7d2>
141ebf85d: 48 8b 57 f8                 	movq	-0x8(%rdi), %rdx
141ebf861: 48 89 d0                    	movq	%rdx, %rax
141ebf864: 48 f7 d8                    	negq	%rax
141ebf867: 70 e7                       	jo	0x141ebf850 <.text+0x1ebe850>
141ebf869: 73 e5                       	jae	0x141ebf850 <.text+0x1ebe850>
141ebf86b: 48 8b 0f                    	movq	(%rdi), %rcx
141ebf86e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebf874: e8 a7 43 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebf879: eb d5                       	jmp	0x141ebf850 <.text+0x1ebe850>
141ebf87b: 48 85 c0                    	testq	%rax, %rax
141ebf87e: 75 13                       	jne	0x141ebf893 <.text+0x1ebe893>
141ebf880: 31 d2                       	xorl	%edx, %edx
141ebf882: e8 c9 69 a2 00              	callq	0x1428e6250 <.text+0x28e5250>
141ebf887: 48 89 c1                    	movq	%rax, %rcx
141ebf88a: 48 85 c0                    	testq	%rax, %rax
141ebf88d: 0f 85 62 ff ff ff           	jne	0x141ebf7f5 <.text+0x1ebe7f5>
141ebf893: 48 8d 0d 3e 5a cf 00        	leaq	0xcf5a3e(%rip), %rcx    # 0x142bb52d8
141ebf89a: e8 31 88 a1 00              	callq	0x1428d80d0 <.text+0x28d70d0>
141ebf89f: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
141ebf8a3: 4c 8b 45 58                 	movq	0x58(%rbp), %r8
141ebf8a7: 48 8b 4d 60                 	movq	0x60(%rbp), %rcx
141ebf8ab: 4c 8b 4d 68                 	movq	0x68(%rbp), %r9
141ebf8af: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
141ebf8b6: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
141ebf8ba: 0f 11 06                    	movups	%xmm0, (%rsi)
141ebf8bd: 48 f7 d8                    	negq	%rax
141ebf8c0: 0f 80 0c ff ff ff           	jo	0x141ebf7d2 <.text+0x1ebe7d2>
141ebf8c6: 48 89 8d 00 01 00 00        	movq	%rcx, 0x100(%rbp)
141ebf8cd: 4c 89 8d 08 01 00 00        	movq	%r9, 0x108(%rbp)
141ebf8d4: 48 8d 85 d8 00 00 00        	leaq	0xd8(%rbp), %rax
141ebf8db: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
141ebf8e2: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
141ebf8e9: 4c 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %r9
141ebf8f0: e8 3b 97 0f ff              	callq	0x140fb9030 <.text+0xfb8030>
141ebf8f5: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
141ebf8fc: 48 89 d0                    	movq	%rdx, %rax
141ebf8ff: 48 f7 d8                    	negq	%rax
141ebf902: 0f 80 ca fe ff ff           	jo	0x141ebf7d2 <.text+0x1ebe7d2>
141ebf908: 0f 83 c4 fe ff ff           	jae	0x141ebf7d2 <.text+0x1ebe7d2>
141ebf90e: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
141ebf915: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141ebf91b: e8 00 43 75 fe              	callq	0x140613c20 <.text+0x612c20>
141ebf920: e9 ad fe ff ff              	jmp	0x141ebf7d2 <.text+0x1ebe7d2>
