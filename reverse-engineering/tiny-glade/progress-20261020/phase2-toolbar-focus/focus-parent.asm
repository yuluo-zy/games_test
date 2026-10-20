
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14233dcb0: 55                          	pushq	%rbp
14233dcb1: 41 57                       	pushq	%r15
14233dcb3: 41 56                       	pushq	%r14
14233dcb5: 41 55                       	pushq	%r13
14233dcb7: 41 54                       	pushq	%r12
14233dcb9: 56                          	pushq	%rsi
14233dcba: 57                          	pushq	%rdi
14233dcbb: 53                          	pushq	%rbx
14233dcbc: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
14233dcc3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14233dccb: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
14233dcd6: 49 89 d5                    	movq	%rdx, %r13
14233dcd9: 48 89 cb                    	movq	%rcx, %rbx
14233dcdc: e8 1f 8d 8d fe              	callq	0x140c16a00 <.text+0xc15a00>
14233dce1: 84 c0                       	testb	%al, %al
14233dce3: 88 45 7f                    	movb	%al, 0x7f(%rbp)
14233dce6: 74 5f                       	je	0x14233dd47 <.text+0x233cd47>
14233dce8: 4c 8d 35 c3 fd d0 00        	leaq	0xd0fdc3(%rip), %r14    # 0x14304dab2
14233dcef: be 39 00 00 00              	movl	$0x39, %esi
14233dcf4: ba 39 00 00 00              	movl	$0x39, %edx
14233dcf9: 4c 89 f1                    	movq	%r14, %rcx
14233dcfc: e8 2f a4 ff ff              	callq	0x142338130 <.text+0x2337130>
14233dd01: 48 83 f8 01                 	cmpq	$0x1, %rax
14233dd05: 0f 85 b1 00 00 00           	jne	0x14233ddbc <.text+0x233cdbc>
14233dd0b: 49 89 d1                    	movq	%rdx, %r9
14233dd0e: 48 85 d2                    	testq	%rdx, %rdx
14233dd11: 74 3d                       	je	0x14233dd50 <.text+0x233cd50>
14233dd13: 49 83 f9 39                 	cmpq	$0x39, %r9
14233dd17: 73 35                       	jae	0x14233dd4e <.text+0x233cd4e>
14233dd19: 48 8d 05 92 fd d0 00        	leaq	0xd0fd92(%rip), %rax    # 0x14304dab2
14233dd20: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
14233dd25: 7f 29                       	jg	0x14233dd50 <.text+0x233cd50>
14233dd27: 48 8d 05 9a fe d0 00        	leaq	0xd0fe9a(%rip), %rax    # 0x14304dbc8
14233dd2e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233dd33: 48 8d 0d 78 fd d0 00        	leaq	0xd0fd78(%rip), %rcx    # 0x14304dab2
14233dd3a: ba 39 00 00 00              	movl	$0x39, %edx
14233dd3f: 45 31 c0                    	xorl	%r8d, %r8d
14233dd42: e8 f9 bb 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233dd47: 31 c0                       	xorl	%eax, %eax
14233dd49: e9 be 01 00 00              	jmp	0x14233df0c <.text+0x233cf0c>
14233dd4e: 75 d7                       	jne	0x14233dd27 <.text+0x233cd27>
14233dd50: 4c 8d 35 5b fd d0 00        	leaq	0xd0fd5b(%rip), %r14    # 0x14304dab2
14233dd57: 4c 89 f1                    	movq	%r14, %rcx
14233dd5a: 4c 89 ca                    	movq	%r9, %rdx
14233dd5d: e8 ce a3 ff ff              	callq	0x142338130 <.text+0x2337130>
14233dd62: 48 83 f8 01                 	cmpq	$0x1, %rax
14233dd66: 75 54                       	jne	0x14233ddbc <.text+0x233cdbc>
14233dd68: 49 89 d0                    	movq	%rdx, %r8
14233dd6b: 49 83 c0 02                 	addq	$0x2, %r8
14233dd6f: 74 39                       	je	0x14233ddaa <.text+0x233cdaa>
14233dd71: 49 83 f8 39                 	cmpq	$0x39, %r8
14233dd75: 73 31                       	jae	0x14233dda8 <.text+0x233cda8>
14233dd77: 48 8d 05 34 fd d0 00        	leaq	0xd0fd34(%rip), %rax    # 0x14304dab2
14233dd7e: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
14233dd83: 7f 25                       	jg	0x14233ddaa <.text+0x233cdaa>
14233dd85: 48 8d 05 54 fe d0 00        	leaq	0xd0fe54(%rip), %rax    # 0x14304dbe0
14233dd8c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233dd91: 48 8d 0d 1a fd d0 00        	leaq	0xd0fd1a(%rip), %rcx    # 0x14304dab2
14233dd98: ba 39 00 00 00              	movl	$0x39, %edx
14233dd9d: 41 b9 39 00 00 00           	movl	$0x39, %r9d
14233dda3: e8 98 bb 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233dda8: 75 db                       	jne	0x14233dd85 <.text+0x233cd85>
14233ddaa: be 37 00 00 00              	movl	$0x37, %esi
14233ddaf: 48 29 d6                    	subq	%rdx, %rsi
14233ddb2: 4c 8d 35 f9 fc d0 00        	leaq	0xd0fcf9(%rip), %r14    # 0x14304dab2
14233ddb9: 4d 01 c6                    	addq	%r8, %r14
14233ddbc: 4c 8d 05 41 ff d0 00        	leaq	0xd0ff41(%rip), %r8     # 0x14304dd04
14233ddc3: 4c 8d 3d 0e ff d0 00        	leaq	0xd0ff0e(%rip), %r15    # 0x14304dcd8
14233ddca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14233ddd0: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
14233ddd5: 85 c0                       	testl	%eax, %eax
14233ddd7: 78 17                       	js	0x14233ddf0 <.text+0x233cdf0>
14233ddd9: 49 ff c8                    	decq	%r8
14233dddc: 83 f8 5c                    	cmpl	$0x5c, %eax
14233dddf: 75 63                       	jne	0x14233de44 <.text+0x233ce44>
14233dde1: eb 72                       	jmp	0x14233de55 <.text+0x233ce55>
14233dde3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233ddf0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
14233ddf5: 80 f9 c0                    	cmpb	$-0x40, %cl
14233ddf8: 7d 23                       	jge	0x14233de1d <.text+0x233ce1d>
14233ddfa: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
14233ddff: 80 fa c0                    	cmpb	$-0x40, %dl
14233de02: 7d 22                       	jge	0x14233de26 <.text+0x233ce26>
14233de04: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
14233de09: 49 83 c0 fc                 	addq	$-0x4, %r8
14233de0d: 41 83 e1 07                 	andl	$0x7, %r9d
14233de11: 41 c1 e1 06                 	shll	$0x6, %r9d
14233de15: 83 e2 3f                    	andl	$0x3f, %edx
14233de18: 44 09 ca                    	orl	%r9d, %edx
14233de1b: eb 10                       	jmp	0x14233de2d <.text+0x233ce2d>
14233de1d: 49 83 c0 fe                 	addq	$-0x2, %r8
14233de21: 83 e1 1f                    	andl	$0x1f, %ecx
14233de24: eb 0f                       	jmp	0x14233de35 <.text+0x233ce35>
14233de26: 49 83 c0 fd                 	addq	$-0x3, %r8
14233de2a: 83 e2 0f                    	andl	$0xf, %edx
14233de2d: c1 e2 06                    	shll	$0x6, %edx
14233de30: 83 e1 3f                    	andl	$0x3f, %ecx
14233de33: 09 d1                       	orl	%edx, %ecx
14233de35: c1 e1 06                    	shll	$0x6, %ecx
14233de38: 24 3f                       	andb	$0x3f, %al
14233de3a: 0f b6 c0                    	movzbl	%al, %eax
14233de3d: 09 c8                       	orl	%ecx, %eax
14233de3f: 83 f8 5c                    	cmpl	$0x5c, %eax
14233de42: 74 11                       	je	0x14233de55 <.text+0x233ce55>
14233de44: 83 f8 2f                    	cmpl	$0x2f, %eax
14233de47: 74 0c                       	je	0x14233de55 <.text+0x233ce55>
14233de49: 4d 39 f8                    	cmpq	%r15, %r8
14233de4c: 75 82                       	jne	0x14233ddd0 <.text+0x233cdd0>
14233de4e: bf 2c 00 00 00              	movl	$0x2c, %edi
14233de53: eb 4c                       	jmp	0x14233dea1 <.text+0x233cea1>
14233de55: 4c 8d 3d 7c fe d0 00        	leaq	0xd0fe7c(%rip), %r15    # 0x14304dcd8
14233de5c: 4d 29 f8                    	subq	%r15, %r8
14233de5f: 49 ff c0                    	incq	%r8
14233de62: 74 32                       	je	0x14233de96 <.text+0x233ce96>
14233de64: 49 83 f8 2c                 	cmpq	$0x2c, %r8
14233de68: 73 2a                       	jae	0x14233de94 <.text+0x233ce94>
14233de6a: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
14233de6f: 7f 25                       	jg	0x14233de96 <.text+0x233ce96>
14233de71: 48 8d 05 30 fd d0 00        	leaq	0xd0fd30(%rip), %rax    # 0x14304dba8
14233de78: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14233de7d: 48 8d 0d 54 fe d0 00        	leaq	0xd0fe54(%rip), %rcx    # 0x14304dcd8
14233de84: ba 2c 00 00 00              	movl	$0x2c, %edx
14233de89: 41 b9 2c 00 00 00           	movl	$0x2c, %r9d
14233de8f: e8 ac ba 59 00              	callq	0x1428d9940 <.text+0x28d8940>
14233de94: 75 db                       	jne	0x14233de71 <.text+0x233ce71>
14233de96: bf 2c 00 00 00              	movl	$0x2c, %edi
14233de9b: 4c 29 c7                    	subq	%r8, %rdi
14233de9e: 4d 01 c7                    	addq	%r8, %r15
14233dea1: e8 8a a2 8d fe              	callq	0x140c18130 <.text+0xc17130>
14233dea6: 48 89 c1                    	movq	%rax, %rcx
14233dea9: 48 8b 00                    	movq	(%rax), %rax
14233deac: 48 83 f8 01                 	cmpq	$0x1, %rax
14233deb0: 0f 85 c0 06 00 00           	jne	0x14233e576 <.text+0x233d576>
14233deb6: 48 83 c1 08                 	addq	$0x8, %rcx
14233deba: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14233debe: 0f 85 ec 06 00 00           	jne	0x14233e5b0 <.text+0x233d5b0>
14233dec4: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14233decb: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14233decf: 48 83 c1 08                 	addq	$0x8, %rcx
14233ded3: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14233ded8: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14233dee1: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
14233deea: 4c 89 f2                    	movq	%r14, %rdx
14233deed: 49 89 f0                    	movq	%rsi, %r8
14233def0: 4d 89 f9                    	movq	%r15, %r9
14233def3: e8 48 8e 8d fe              	callq	0x140c16d40 <.text+0xc15d40>
14233def8: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14233defc: 48 ff 01                    	incq	(%rcx)
14233deff: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14233df03: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
14233df07: b8 01 00 00 00              	movl	$0x1, %eax
14233df0c: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14233df10: 31 c9                       	xorl	%ecx, %ecx
14233df12: e8 09 bc d9 fd              	callq	0x1400d9b20 <.text+0xd8b20>
14233df17: 4c 89 6d 30                 	movq	%r13, 0x30(%rbp)
14233df1b: 48 85 c0                    	testq	%rax, %rax
14233df1e: 0f 84 98 06 00 00           	je	0x14233e5bc <.text+0x233d5bc>
14233df24: 0f 10 00                    	movups	(%rax), %xmm0
14233df27: 48 ff 00                    	incq	(%rax)
14233df2a: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
14233df2e: 0f 10 05 fb 47 87 00        	movups	0x8747fb(%rip), %xmm0   # 0x142bb2730
14233df35: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
14233df39: 0f 10 05 00 48 87 00        	movups	0x874800(%rip), %xmm0   # 0x142bb2740
14233df40: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
14233df44: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
14233df4c: 48 c7 45 48 08 00 00 00     	movq	$0x8, 0x48(%rbp)
14233df54: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
14233df5c: 4c 8b 23                    	movq	(%rbx), %r12
14233df5f: 4c 8b 6b 08                 	movq	0x8(%rbx), %r13
14233df63: 8b 43 14                    	movl	0x14(%rbx), %eax
14233df66: 89 45 2c                    	movl	%eax, 0x2c(%rbp)
14233df69: 49 8b 9c 24 f8 00 00 00     	movq	0xf8(%r12), %rbx
14233df71: 49 8b 84 24 00 01 00 00     	movq	0x100(%r12), %rax
14233df79: 48 8d 34 83                 	leaq	(%rbx,%rax,4), %rsi
14233df7d: 45 0f b6 94 24 08 01 00 00  	movzbl	0x108(%r12), %r10d
14233df86: b8 08 00 00 00              	movl	$0x8, %eax
14233df8b: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14233df8f: 45 31 ff                    	xorl	%r15d, %r15d
14233df92: 4c 8d 5d d0                 	leaq	-0x30(%rbp), %r11
14233df96: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14233df9a: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14233dfa2: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
14233dfaa: 31 ff                       	xorl	%edi, %edi
14233dfac: 45 31 f6                    	xorl	%r14d, %r14d
14233dfaf: 45 84 d2                    	testb	%r10b, %r10b
14233dfb2: 75 49                       	jne	0x14233dffd <.text+0x233cffd>
14233dfb4: e9 67 01 00 00              	jmp	0x14233e120 <.text+0x233d120>
14233dfb9: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
14233dfc1: 48 8b 40 08                 	movq	0x8(%rax), %rax
14233dfc5: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14233dfc9: 45 31 f6                    	xorl	%r14d, %r14d
14233dfcc: 48 8b 45 38                 	movq	0x38(%rbp), %rax
14233dfd0: 4a 8d 04 f0                 	leaq	(%rax,%r14,8), %rax
14233dfd4: 44 89 f1                    	movl	%r14d, %ecx
14233dfd7: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
14233dfdb: 48 8d 14 ca                 	leaq	(%rdx,%rcx,8), %rdx
14233dfdf: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233dfe3: 48 03 4d 60                 	addq	0x60(%rbp), %rcx
14233dfe7: 49 ff c6                    	incq	%r14
14233dfea: 80 79 01 00                 	cmpb	$0x0, 0x1(%rcx)
14233dfee: 0f 85 d4 00 00 00           	jne	0x14233e0c8 <.text+0x233d0c8>
14233dff4: 45 84 d2                    	testb	%r10b, %r10b
14233dff7: 0f 84 23 01 00 00           	je	0x14233e120 <.text+0x233d120>
14233dffd: 49 39 fe                    	cmpq	%rdi, %r14
14233e000: 75 ca                       	jne	0x14233dfcc <.text+0x233cfcc>
14233e002: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14233e010: 48 39 f3                    	cmpq	%rsi, %rbx
14233e013: 0f 84 04 02 00 00           	je	0x14233e21d <.text+0x233d21d>
14233e019: 8b 0b                       	movl	(%rbx), %ecx
14233e01b: 48 83 c3 04                 	addq	$0x4, %rbx
14233e01f: 49 8b 85 a8 01 00 00        	movq	0x1a8(%r13), %rax
14233e026: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14233e02a: 48 8b 7c c8 10              	movq	0x10(%rax,%rcx,8), %rdi
14233e02f: 48 85 ff                    	testq	%rdi, %rdi
14233e032: 74 dc                       	je	0x14233e010 <.text+0x233d010>
14233e034: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
14233e038: 49 8b 94 24 10 01 00 00     	movq	0x110(%r12), %rdx
14233e040: 48 8b 48 40                 	movq	0x40(%rax), %rcx
14233e044: 48 39 d1                    	cmpq	%rdx, %rcx
14233e047: 76 38                       	jbe	0x14233e081 <.text+0x233d081>
14233e049: 4c 8b 40 38                 	movq	0x38(%rax), %r8
14233e04d: 49 8b 14 d0                 	movq	(%r8,%rdx,8), %rdx
14233e051: 48 85 d2                    	testq	%rdx, %rdx
14233e054: 74 2b                       	je	0x14233e081 <.text+0x233d081>
14233e056: 4c 8b 40 18                 	movq	0x18(%rax), %r8
14233e05a: 48 f7 d2                    	notq	%rdx
14233e05d: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14233e061: 48 c1 e2 04                 	shlq	$0x4, %rdx
14233e065: 49 8b 54 10 10              	movq	0x10(%r8,%rdx), %rdx
14233e06a: 48 89 55 70                 	movq	%rdx, 0x70(%rbp)
14233e06e: 49 8b 94 24 18 01 00 00     	movq	0x118(%r12), %rdx
14233e076: 48 39 d1                    	cmpq	%rdx, %rcx
14233e079: 0f 86 3a ff ff ff           	jbe	0x14233dfb9 <.text+0x233cfb9>
14233e07f: eb 19                       	jmp	0x14233e09a <.text+0x233d09a>
14233e081: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14233e089: 49 8b 94 24 18 01 00 00     	movq	0x118(%r12), %rdx
14233e091: 48 39 d1                    	cmpq	%rdx, %rcx
14233e094: 0f 86 1f ff ff ff           	jbe	0x14233dfb9 <.text+0x233cfb9>
14233e09a: 48 8b 48 38                 	movq	0x38(%rax), %rcx
14233e09e: 48 8b 0c d1                 	movq	(%rcx,%rdx,8), %rcx
14233e0a2: 48 85 c9                    	testq	%rcx, %rcx
14233e0a5: 0f 84 0e ff ff ff           	je	0x14233dfb9 <.text+0x233cfb9>
14233e0ab: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14233e0af: 48 f7 d1                    	notq	%rcx
14233e0b2: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233e0b6: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233e0ba: 48 8b 4c 0a 10              	movq	0x10(%rdx,%rcx), %rcx
14233e0bf: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
14233e0c3: e9 f9 fe ff ff              	jmp	0x14233dfc1 <.text+0x233cfc1>
14233e0c8: 49 89 d7                    	movq	%rdx, %r15
14233e0cb: 44 88 55 6f                 	movb	%r10b, 0x6f(%rbp)
14233e0cf: 48 8b 10                    	movq	(%rax), %rdx
14233e0d2: 4c 89 d9                    	movq	%r11, %rcx
14233e0d5: e8 16 c2 f8 fd              	callq	0x1402ca2f0 <.text+0x2c92f0>
14233e0da: 49 8b 07                    	movq	(%r15), %rax
14233e0dd: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14233e0e1: 4c 8b 7d 50                 	movq	0x50(%rbp), %r15
14233e0e5: 4c 3b 7d 40                 	cmpq	0x40(%rbp), %r15
14233e0e9: 75 10                       	jne	0x14233e0fb <.text+0x233d0fb>
14233e0eb: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14233e0ef: 48 8d 15 e2 fc d0 00        	leaq	0xd0fce2(%rip), %rdx    # 0x14304ddd8
14233e0f6: e8 f5 3c 84 fe              	callq	0x140b81df0 <.text+0xb80df0>
14233e0fb: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14233e0ff: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
14233e103: 4a 89 0c f8                 	movq	%rcx, (%rax,%r15,8)
14233e107: 49 ff c7                    	incq	%r15
14233e10a: 4c 89 7d 50                 	movq	%r15, 0x50(%rbp)
14233e10e: 44 0f b6 55 6f              	movzbl	0x6f(%rbp), %r10d
14233e113: 4c 8d 5d d0                 	leaq	-0x30(%rbp), %r11
14233e117: 45 84 d2                    	testb	%r10b, %r10b
14233e11a: 0f 85 dd fe ff ff           	jne	0x14233dffd <.text+0x233cffd>
14233e120: 49 39 fe                    	cmpq	%rdi, %r14
14233e123: 0f 85 dc 00 00 00           	jne	0x14233e205 <.text+0x233d205>
14233e129: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14233e130: 48 39 f3                    	cmpq	%rsi, %rbx
14233e133: 0f 84 e4 00 00 00           	je	0x14233e21d <.text+0x233d21d>
14233e139: 8b 0b                       	movl	(%rbx), %ecx
14233e13b: 48 83 c3 04                 	addq	$0x4, %rbx
14233e13f: 49 8b 85 00 01 00 00        	movq	0x100(%r13), %rax
14233e146: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233e14a: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233e14e: 48 8b 7c 08 58              	movq	0x58(%rax,%rcx), %rdi
14233e153: 48 85 ff                    	testq	%rdi, %rdi
14233e156: 74 d8                       	je	0x14233e130 <.text+0x233d130>
14233e158: 48 01 c8                    	addq	%rcx, %rax
14233e15b: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14233e161: 49 8b 95 a8 01 00 00        	movq	0x1a8(%r13), %rdx
14233e168: 4c 8d 0c c9                 	leaq	(%rcx,%rcx,8), %r9
14233e16c: 4a 8d 0c ca                 	leaq	(%rdx,%r9,8), %rcx
14233e170: 4d 8b 84 24 10 01 00 00     	movq	0x110(%r12), %r8
14233e178: 4a 8b 54 ca 40              	movq	0x40(%rdx,%r9,8), %rdx
14233e17d: 4c 39 c2                    	cmpq	%r8, %rdx
14233e180: 76 34                       	jbe	0x14233e1b6 <.text+0x233d1b6>
14233e182: 4c 8b 49 38                 	movq	0x38(%rcx), %r9
14233e186: 4f 8b 04 c1                 	movq	(%r9,%r8,8), %r8
14233e18a: 4d 85 c0                    	testq	%r8, %r8
14233e18d: 74 27                       	je	0x14233e1b6 <.text+0x233d1b6>
14233e18f: 4c 8b 49 18                 	movq	0x18(%rcx), %r9
14233e193: 49 f7 d0                    	notq	%r8
14233e196: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14233e19a: 49 c1 e0 04                 	shlq	$0x4, %r8
14233e19e: 4f 8b 44 01 10              	movq	0x10(%r9,%r8), %r8
14233e1a3: 4c 89 45 70                 	movq	%r8, 0x70(%rbp)
14233e1a7: 4d 8b 84 24 18 01 00 00     	movq	0x118(%r12), %r8
14233e1af: 4c 39 c2                    	cmpq	%r8, %rdx
14233e1b2: 77 17                       	ja	0x14233e1cb <.text+0x233d1cb>
14233e1b4: eb 3c                       	jmp	0x14233e1f2 <.text+0x233d1f2>
14233e1b6: 48 c7 45 70 00 00 00 00     	movq	$0x0, 0x70(%rbp)
14233e1be: 4d 8b 84 24 18 01 00 00     	movq	0x118(%r12), %r8
14233e1c6: 4c 39 c2                    	cmpq	%r8, %rdx
14233e1c9: 76 27                       	jbe	0x14233e1f2 <.text+0x233d1f2>
14233e1cb: 48 8b 51 38                 	movq	0x38(%rcx), %rdx
14233e1cf: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
14233e1d3: 48 85 d2                    	testq	%rdx, %rdx
14233e1d6: 74 1a                       	je	0x14233e1f2 <.text+0x233d1f2>
14233e1d8: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14233e1dc: 48 f7 d2                    	notq	%rdx
14233e1df: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14233e1e3: 48 c1 e2 04                 	shlq	$0x4, %rdx
14233e1e7: 48 8b 4c 11 10              	movq	0x10(%rcx,%rdx), %rcx
14233e1ec: 48 89 4d 60                 	movq	%rcx, 0x60(%rbp)
14233e1f0: eb 08                       	jmp	0x14233e1fa <.text+0x233d1fa>
14233e1f2: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
14233e1fa: 48 8b 40 50                 	movq	0x50(%rax), %rax
14233e1fe: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14233e202: 45 31 f6                    	xorl	%r14d, %r14d
14233e205: 4c 89 f1                    	movq	%r14, %rcx
14233e208: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233e20c: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
14233e210: 48 8d 04 0a                 	leaq	(%rdx,%rcx), %rax
14233e214: 8b 4c 0a 08                 	movl	0x8(%rdx,%rcx), %ecx
14233e218: e9 ba fd ff ff              	jmp	0x14233dfd7 <.text+0x233cfd7>
14233e21d: 4d 85 ff                    	testq	%r15, %r15
14233e220: 0f 84 ae 02 00 00           	je	0x14233e4d4 <.text+0x233d4d4>
14233e226: 48 8b 45 30                 	movq	0x30(%rbp), %rax
14233e22a: 48 8b 30                    	movq	(%rax), %rsi
14233e22d: 4c 8b 70 08                 	movq	0x8(%rax), %r14
14233e231: 8b 40 14                    	movl	0x14(%rax), %eax
14233e234: 89 45 60                    	movl	%eax, 0x60(%rbp)
14233e237: bf ff ff ff ff              	movl	$0xffffffff, %edi       # imm = 0xFFFFFFFF
14233e23c: eb 0f                       	jmp	0x14233e24d <.text+0x233d24d>
14233e23e: 66 90                       	nop
14233e240: 4c 8b 7d 50                 	movq	0x50(%rbp), %r15
14233e244: 4d 85 ff                    	testq	%r15, %r15
14233e247: 0f 84 87 02 00 00           	je	0x14233e4d4 <.text+0x233d4d4>
14233e24d: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14233e251: 4a 8d 0c f8                 	leaq	(%rax,%r15,8), %rcx
14233e255: 49 8b 56 10                 	movq	0x10(%r14), %rdx
14233e259: eb 12                       	jmp	0x14233e26d <.text+0x233d26d>
14233e25b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14233e260: 48 83 c0 08                 	addq	$0x8, %rax
14233e264: 48 39 c8                    	cmpq	%rcx, %rax
14233e267: 0f 84 a3 00 00 00           	je	0x14233e310 <.text+0x233d310>
14233e26d: 4c 8b 00                    	movq	(%rax), %r8
14233e270: 45 89 c1                    	movl	%r8d, %r9d
14233e273: 49 39 d1                    	cmpq	%rdx, %r9
14233e276: 73 e8                       	jae	0x14233e260 <.text+0x233d260>
14233e278: 4d 8b 56 08                 	movq	0x8(%r14), %r10
14233e27c: 49 c1 e8 20                 	shrq	$0x20, %r8
14233e280: 4f 8d 0c 89                 	leaq	(%r9,%r9,4), %r9
14233e284: 47 39 04 8a                 	cmpl	%r8d, (%r10,%r9,4)
14233e288: 75 d6                       	jne	0x14233e260 <.text+0x233d260>
14233e28a: 4f 8d 04 8a                 	leaq	(%r10,%r9,4), %r8
14233e28e: 45 8b 48 04                 	movl	0x4(%r8), %r9d
14233e292: 49 39 f9                    	cmpq	%rdi, %r9
14233e295: 74 c9                       	je	0x14233e260 <.text+0x233d260>
14233e297: 4c 39 4e 38                 	cmpq	%r9, 0x38(%rsi)
14233e29b: 76 c3                       	jbe	0x14233e260 <.text+0x233d260>
14233e29d: 4c 8b 56 28                 	movq	0x28(%rsi), %r10
14233e2a1: 45 89 cb                    	movl	%r9d, %r11d
14233e2a4: 41 c1 eb 06                 	shrl	$0x6, %r11d
14233e2a8: 4f 8b 14 da                 	movq	(%r10,%r11,8), %r10
14233e2ac: 4d 0f a3 ca                 	btq	%r9, %r10
14233e2b0: 73 ae                       	jae	0x14233e260 <.text+0x233d260>
14233e2b2: 41 8b 50 0c                 	movl	0xc(%r8), %edx
14233e2b6: 45 8b 40 10                 	movl	0x10(%r8), %r8d
14233e2ba: 4d 8b 8e a8 01 00 00        	movq	0x1a8(%r14), %r9
14233e2c1: 48 8d 14 d2                 	leaq	(%rdx,%rdx,8), %rdx
14233e2c5: 4c 8b 96 10 01 00 00        	movq	0x110(%rsi), %r10
14233e2cc: 4d 8b 5c d1 18              	movq	0x18(%r9,%rdx,8), %r11
14233e2d1: 49 8b 54 d1 38              	movq	0x38(%r9,%rdx,8), %rdx
14233e2d6: 4a 8b 14 d2                 	movq	(%rdx,%r10,8), %rdx
14233e2da: 48 f7 d2                    	notq	%rdx
14233e2dd: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14233e2e1: 48 c1 e2 04                 	shlq	$0x4, %rdx
14233e2e5: 4d 8b 4c 13 10              	movq	0x10(%r11,%rdx), %r9
14233e2ea: 49 8b 54 13 28              	movq	0x28(%r11,%rdx), %rdx
14233e2ef: 44 8b 55 60                 	movl	0x60(%rbp), %r10d
14233e2f3: 46 89 14 82                 	movl	%r10d, (%rdx,%r8,4)
14233e2f7: 4b 8d 14 40                 	leaq	(%r8,%r8,2), %rdx
14233e2fb: 41 c6 44 11 01 01           	movb	$0x1, 0x1(%r9,%rdx)
14233e301: 48 83 c0 08                 	addq	$0x8, %rax
14233e305: 48 39 c8                    	cmpq	%rcx, %rax
14233e308: 0f 85 47 ff ff ff           	jne	0x14233e255 <.text+0x233d255>
14233e30e: 66 90                       	nop
14233e310: 4c 8b 55 40                 	movq	0x40(%rbp), %r10
14233e314: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14233e318: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14233e31c: 48 8b 45 50                 	movq	0x50(%rbp), %rax
14233e320: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
14233e328: 48 c7 45 48 08 00 00 00     	movq	$0x8, 0x48(%rbp)
14233e330: 48 c7 45 50 00 00 00 00     	movq	$0x0, 0x50(%rbp)
14233e338: 48 85 c0                    	testq	%rax, %rax
14233e33b: 0f 84 6f 01 00 00           	je	0x14233e4b0 <.text+0x233d4b0>
14233e341: 48 8b 5d 70                 	movq	0x70(%rbp), %rbx
14233e345: 4c 8d 3c c3                 	leaq	(%rbx,%rax,8), %r15
14233e349: eb 12                       	jmp	0x14233e35d <.text+0x233d35d>
14233e34b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14233e350: 48 83 c3 08                 	addq	$0x8, %rbx
14233e354: 4c 39 fb                    	cmpq	%r15, %rbx
14233e357: 0f 84 53 01 00 00           	je	0x14233e4b0 <.text+0x233d4b0>
14233e35d: 48 8b 13                    	movq	(%rbx), %rdx
14233e360: 48 89 d0                    	movq	%rdx, %rax
14233e363: 48 c1 e8 20                 	shrq	$0x20, %rax
14233e367: 0f 84 43 01 00 00           	je	0x14233e4b0 <.text+0x233d4b0>
14233e36d: 89 d1                       	movl	%edx, %ecx
14233e36f: 49 3b 4d 10                 	cmpq	0x10(%r13), %rcx
14233e373: 73 db                       	jae	0x14233e350 <.text+0x233d350>
14233e375: 4d 8b 45 08                 	movq	0x8(%r13), %r8
14233e379: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233e37d: 41 39 04 88                 	cmpl	%eax, (%r8,%rcx,4)
14233e381: 75 cd                       	jne	0x14233e350 <.text+0x233d350>
14233e383: 49 8d 04 88                 	leaq	(%r8,%rcx,4), %rax
14233e387: 8b 48 04                    	movl	0x4(%rax), %ecx
14233e38a: 48 39 f9                    	cmpq	%rdi, %rcx
14233e38d: 74 c1                       	je	0x14233e350 <.text+0x233d350>
14233e38f: 49 39 4c 24 38              	cmpq	%rcx, 0x38(%r12)
14233e394: 76 ba                       	jbe	0x14233e350 <.text+0x233d350>
14233e396: 4d 8b 44 24 28              	movq	0x28(%r12), %r8
14233e39b: 41 89 c9                    	movl	%ecx, %r9d
14233e39e: 41 c1 e9 06                 	shrl	$0x6, %r9d
14233e3a2: 4f 8b 04 c8                 	movq	(%r8,%r9,8), %r8
14233e3a6: 49 0f a3 c8                 	btq	%rcx, %r8
14233e3aa: 73 a4                       	jae	0x14233e350 <.text+0x233d350>
14233e3ac: 4c 89 55 38                 	movq	%r10, 0x38(%rbp)
14233e3b0: 8b 48 0c                    	movl	0xc(%rax), %ecx
14233e3b3: 4d 8b 95 a8 01 00 00        	movq	0x1a8(%r13), %r10
14233e3ba: 4c 8d 1c c9                 	leaq	(%rcx,%rcx,8), %r11
14233e3be: 4f 8d 04 da                 	leaq	(%r10,%r11,8), %r8
14233e3c2: 4d 8b 8c 24 10 01 00 00     	movq	0x110(%r12), %r9
14233e3ca: 4b 8b 4c da 38              	movq	0x38(%r10,%r11,8), %rcx
14233e3cf: 4f 39 4c da 40              	cmpq	%r9, 0x40(%r10,%r11,8)
14233e3d4: 76 23                       	jbe	0x14233e3f9 <.text+0x233d3f9>
14233e3d6: 4e 8b 0c c9                 	movq	(%rcx,%r9,8), %r9
14233e3da: 4d 8b 40 18                 	movq	0x18(%r8), %r8
14233e3de: 4d 85 c9                    	testq	%r9, %r9
14233e3e1: 74 1a                       	je	0x14233e3fd <.text+0x233d3fd>
14233e3e3: 49 f7 d1                    	notq	%r9
14233e3e6: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
14233e3ea: 49 c1 e1 04                 	shlq	$0x4, %r9
14233e3ee: 4f 8b 4c 08 10              	movq	0x10(%r8,%r9), %r9
14233e3f3: 4c 89 4d 58                 	movq	%r9, 0x58(%rbp)
14233e3f7: eb 0c                       	jmp	0x14233e405 <.text+0x233d405>
14233e3f9: 4d 8b 40 18                 	movq	0x18(%r8), %r8
14233e3fd: 48 c7 45 58 00 00 00 00     	movq	$0x0, 0x58(%rbp)
14233e405: 44 8b 48 10                 	movl	0x10(%rax), %r9d
14233e409: 49 8b 84 24 18 01 00 00     	movq	0x118(%r12), %rax
14233e411: 48 8b 04 c1                 	movq	(%rcx,%rax,8), %rax
14233e415: 48 f7 d0                    	notq	%rax
14233e418: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14233e41c: 48 c1 e0 04                 	shlq	$0x4, %rax
14233e420: 49 8b 4c 00 10              	movq	0x10(%r8,%rax), %rcx
14233e425: 49 8b 44 00 28              	movq	0x28(%r8,%rax), %rax
14233e42a: 44 8b 45 2c                 	movl	0x2c(%rbp), %r8d
14233e42e: 46 89 04 88                 	movl	%r8d, (%rax,%r9,4)
14233e432: 4c 89 4d 30                 	movq	%r9, 0x30(%rbp)
14233e436: 4b 8d 04 49                 	leaq	(%r9,%r9,2), %rax
14233e43a: c6 44 01 01 01              	movb	$0x1, 0x1(%rcx,%rax)
14233e43f: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14233e443: e8 a8 be f8 fd              	callq	0x1402ca2f0 <.text+0x2c92f0>
14233e448: 84 c0                       	testb	%al, %al
14233e44a: 75 45                       	jne	0x14233e491 <.text+0x233d491>
14233e44c: 48 8b 45 58                 	movq	0x58(%rbp), %rax
14233e450: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
14233e454: 48 8b 04 c8                 	movq	(%rax,%rcx,8), %rax
14233e458: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14233e45c: 48 8b 45 50                 	movq	0x50(%rbp), %rax
14233e460: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14233e464: 48 3b 45 40                 	cmpq	0x40(%rbp), %rax
14233e468: 75 10                       	jne	0x14233e47a <.text+0x233d47a>
14233e46a: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14233e46e: 48 8d 15 4b f9 d0 00        	leaq	0xd0f94b(%rip), %rdx    # 0x14304ddc0
14233e475: e8 76 39 84 fe              	callq	0x140b81df0 <.text+0xb80df0>
14233e47a: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14233e47e: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14233e482: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
14233e486: 48 89 14 c8                 	movq	%rdx, (%rax,%rcx,8)
14233e48a: 48 ff c1                    	incq	%rcx
14233e48d: 48 89 4d 50                 	movq	%rcx, 0x50(%rbp)
14233e491: 48 83 c3 08                 	addq	$0x8, %rbx
14233e495: 4c 39 fb                    	cmpq	%r15, %rbx
14233e498: 4c 8b 55 38                 	movq	0x38(%rbp), %r10
14233e49c: 0f 85 bb fe ff ff           	jne	0x14233e35d <.text+0x233d35d>
14233e4a2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14233e4b0: 4d 85 d2                    	testq	%r10, %r10
14233e4b3: 0f 84 87 fd ff ff           	je	0x14233e240 <.text+0x233d240>
14233e4b9: 49 c1 e2 03                 	shlq	$0x3, %r10
14233e4bd: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14233e4c3: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14233e4c7: 4c 89 d2                    	movq	%r10, %rdx
14233e4ca: e8 51 57 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233e4cf: e9 6c fd ff ff              	jmp	0x14233e240 <.text+0x233d240>
14233e4d4: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14233e4d8: 48 85 d2                    	testq	%rdx, %rdx
14233e4db: 74 13                       	je	0x14233e4f0 <.text+0x233d4f0>
14233e4dd: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233e4e1: 48 c1 e2 03                 	shlq	$0x3, %rdx
14233e4e5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14233e4eb: e8 30 57 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233e4f0: 48 8b 55 d8                 	movq	-0x28(%rbp), %rdx
14233e4f4: 48 85 d2                    	testq	%rdx, %rdx
14233e4f7: 0f b6 5d 7f                 	movzbl	0x7f(%rbp), %ebx
14233e4fb: 74 27                       	je	0x14233e524 <.text+0x233d524>
14233e4fd: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
14233e505: 48 83 e0 f0                 	andq	$-0x10, %rax
14233e509: 48 01 c2                    	addq	%rax, %rdx
14233e50c: 48 83 c2 11                 	addq	$0x11, %rdx
14233e510: 74 12                       	je	0x14233e524 <.text+0x233d524>
14233e512: 48 8b 4d d0                 	movq	-0x30(%rbp), %rcx
14233e516: 48 29 c1                    	subq	%rax, %rcx
14233e519: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14233e51f: e8 fc 56 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233e524: 84 db                       	testb	%bl, %bl
14233e526: 74 3a                       	je	0x14233e562 <.text+0x233d562>
14233e528: e8 03 9c 8d fe              	callq	0x140c18130 <.text+0xc17130>
14233e52d: 48 89 c1                    	movq	%rax, %rcx
14233e530: 48 8b 00                    	movq	(%rax), %rax
14233e533: 48 83 f8 01                 	cmpq	$0x1, %rax
14233e537: 75 57                       	jne	0x14233e590 <.text+0x233d590>
14233e539: 48 83 c1 08                 	addq	$0x8, %rcx
14233e53d: 48 83 39 00                 	cmpq	$0x0, (%rcx)
14233e541: 75 6d                       	jne	0x14233e5b0 <.text+0x233d5b0>
14233e543: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14233e54a: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14233e54e: 48 83 c1 08                 	addq	$0x8, %rcx
14233e552: 48 8b 55 18                 	movq	0x18(%rbp), %rdx
14233e556: e8 95 8b 8d fe              	callq	0x140c170f0 <.text+0xc160f0>
14233e55b: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14233e55f: 48 ff 00                    	incq	(%rax)
14233e562: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
14233e569: 5b                          	popq	%rbx
14233e56a: 5f                          	popq	%rdi
14233e56b: 5e                          	popq	%rsi
14233e56c: 41 5c                       	popq	%r12
14233e56e: 41 5d                       	popq	%r13
14233e570: 41 5e                       	popq	%r14
14233e572: 41 5f                       	popq	%r15
14233e574: 5d                          	popq	%rbp
14233e575: c3                          	retq
14233e576: 48 85 c0                    	testq	%rax, %rax
14233e579: 75 29                       	jne	0x14233e5a4 <.text+0x233d5a4>
14233e57b: 31 d2                       	xorl	%edx, %edx
14233e57d: e8 ce 7c 5a 00              	callq	0x1428e6250 <.text+0x28e5250>
14233e582: 48 89 c1                    	movq	%rax, %rcx
14233e585: 48 85 c0                    	testq	%rax, %rax
14233e588: 0f 85 2c f9 ff ff           	jne	0x14233deba <.text+0x233ceba>
14233e58e: eb 14                       	jmp	0x14233e5a4 <.text+0x233d5a4>
14233e590: 48 85 c0                    	testq	%rax, %rax
14233e593: 75 0f                       	jne	0x14233e5a4 <.text+0x233d5a4>
14233e595: 31 d2                       	xorl	%edx, %edx
14233e597: e8 b4 7c 5a 00              	callq	0x1428e6250 <.text+0x28e5250>
14233e59c: 48 89 c1                    	movq	%rax, %rcx
14233e59f: 48 85 c0                    	testq	%rax, %rax
14233e5a2: 75 99                       	jne	0x14233e53d <.text+0x233d53d>
14233e5a4: 48 8d 0d 2d 6d 87 00        	leaq	0x876d2d(%rip), %rcx    # 0x142bb52d8
14233e5ab: e8 20 9b 59 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14233e5b0: 48 8d 0d 91 6d 87 00        	leaq	0x876d91(%rip), %rcx    # 0x142bb5348
14233e5b7: e8 94 ac 59 00              	callq	0x1428d9250 <.text+0x28d8250>
14233e5bc: 48 8d 0d 15 6d 87 00        	leaq	0x876d15(%rip), %rcx    # 0x142bb52d8
14233e5c3: e8 08 9b 59 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14233e5c8: 0f 0b                       	ud2
14233e5ca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14233e5d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233e5d5: 55                          	pushq	%rbp
14233e5d6: 41 57                       	pushq	%r15
14233e5d8: 41 56                       	pushq	%r14
14233e5da: 41 55                       	pushq	%r13
14233e5dc: 41 54                       	pushq	%r12
14233e5de: 56                          	pushq	%rsi
14233e5df: 57                          	pushq	%rdi
14233e5e0: 53                          	pushq	%rbx
14233e5e1: 48 83 ec 38                 	subq	$0x38, %rsp
14233e5e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233e5ec: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14233e5f0: 48 ff 00                    	incq	(%rax)
14233e5f3: 48 83 c4 38                 	addq	$0x38, %rsp
14233e5f7: 5b                          	popq	%rbx
14233e5f8: 5f                          	popq	%rdi
14233e5f9: 5e                          	popq	%rsi
14233e5fa: 41 5c                       	popq	%r12
14233e5fc: 41 5d                       	popq	%r13
14233e5fe: 41 5e                       	popq	%r14
14233e600: 41 5f                       	popq	%r15
14233e602: 5d                          	popq	%rbp
14233e603: c3                          	retq
14233e604: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233e610: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233e615: 55                          	pushq	%rbp
14233e616: 41 57                       	pushq	%r15
14233e618: 41 56                       	pushq	%r14
14233e61a: 41 55                       	pushq	%r13
14233e61c: 41 54                       	pushq	%r12
14233e61e: 56                          	pushq	%rsi
14233e61f: 57                          	pushq	%rdi
14233e620: 53                          	pushq	%rbx
14233e621: 48 83 ec 38                 	subq	$0x38, %rsp
14233e625: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233e62c: 48 8b 45 70                 	movq	0x70(%rbp), %rax
14233e630: 48 ff 00                    	incq	(%rax)
14233e633: 48 83 c4 38                 	addq	$0x38, %rsp
14233e637: 5b                          	popq	%rbx
14233e638: 5f                          	popq	%rdi
14233e639: 5e                          	popq	%rsi
14233e63a: 41 5c                       	popq	%r12
14233e63c: 41 5d                       	popq	%r13
14233e63e: 41 5e                       	popq	%r14
14233e640: 41 5f                       	popq	%r15
14233e642: 5d                          	popq	%rbp
14233e643: c3                          	retq
14233e644: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233e650: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233e655: 55                          	pushq	%rbp
14233e656: 41 57                       	pushq	%r15
14233e658: 41 56                       	pushq	%r14
14233e65a: 41 55                       	pushq	%r13
14233e65c: 41 54                       	pushq	%r12
14233e65e: 56                          	pushq	%rsi
14233e65f: 57                          	pushq	%rdi
14233e660: 53                          	pushq	%rbx
14233e661: 48 83 ec 38                 	subq	$0x38, %rsp
14233e665: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233e66c: 48 83 7d 38 00              	cmpq	$0x0, 0x38(%rbp)
14233e671: 74 17                       	je	0x14233e68a <.text+0x233d68a>
14233e673: 48 8b 55 38                 	movq	0x38(%rbp), %rdx
14233e677: 48 c1 e2 03                 	shlq	$0x3, %rdx
14233e67b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14233e681: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
14233e685: e8 96 55 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233e68a: 90                          	nop
14233e68b: 48 83 c4 38                 	addq	$0x38, %rsp
14233e68f: 5b                          	popq	%rbx
14233e690: 5f                          	popq	%rdi
14233e691: 5e                          	popq	%rsi
14233e692: 41 5c                       	popq	%r12
14233e694: 41 5d                       	popq	%r13
14233e696: 41 5e                       	popq	%r14
14233e698: 41 5f                       	popq	%r15
14233e69a: 5d                          	popq	%rbp
14233e69b: c3                          	retq
14233e69c: 0f 1f 40 00                 	nopl	(%rax)
14233e6a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233e6a5: 55                          	pushq	%rbp
14233e6a6: 41 57                       	pushq	%r15
14233e6a8: 41 56                       	pushq	%r14
14233e6aa: 41 55                       	pushq	%r13
14233e6ac: 41 54                       	pushq	%r12
14233e6ae: 56                          	pushq	%rsi
14233e6af: 57                          	pushq	%rdi
14233e6b0: 53                          	pushq	%rbx
14233e6b1: 48 83 ec 38                 	subq	$0x38, %rsp
14233e6b5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233e6bc: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14233e6c0: 48 85 d2                    	testq	%rdx, %rdx
14233e6c3: 74 13                       	je	0x14233e6d8 <.text+0x233d6d8>
14233e6c5: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14233e6c9: 48 c1 e2 03                 	shlq	$0x3, %rdx
14233e6cd: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14233e6d3: e8 48 55 2d fe              	callq	0x140613c20 <.text+0x612c20>
14233e6d8: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14233e6dc: e8 6f b8 e0 fd              	callq	0x140149f50 <.text+0x148f50>
14233e6e1: 90                          	nop
14233e6e2: 48 83 c4 38                 	addq	$0x38, %rsp
14233e6e6: 5b                          	popq	%rbx
14233e6e7: 5f                          	popq	%rdi
14233e6e8: 5e                          	popq	%rsi
14233e6e9: 41 5c                       	popq	%r12
14233e6eb: 41 5d                       	popq	%r13
14233e6ed: 41 5e                       	popq	%r14
14233e6ef: 41 5f                       	popq	%r15
14233e6f1: 5d                          	popq	%rbp
14233e6f2: c3                          	retq
14233e6f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233e700: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14233e705: 55                          	pushq	%rbp
14233e706: 41 57                       	pushq	%r15
14233e708: 41 56                       	pushq	%r14
14233e70a: 41 55                       	pushq	%r13
14233e70c: 41 54                       	pushq	%r12
14233e70e: 56                          	pushq	%rsi
14233e70f: 57                          	pushq	%rdi
14233e710: 53                          	pushq	%rbx
14233e711: 48 83 ec 38                 	subq	$0x38, %rsp
14233e715: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14233e71c: 80 7d 7f 00                 	cmpb	$0x0, 0x7f(%rbp)
14233e720: 74 20                       	je	0x14233e742 <.text+0x233d742>
14233e722: 48 8d 45 c8                 	leaq	-0x38(%rbp), %rax
14233e726: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
14233e72a: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
14233e72e: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14233e732: 48 8d 0d 0f f4 d0 00        	leaq	0xd0f40f(%rip), %rcx    # 0x14304db48
14233e739: 48 8d 55 08                 	leaq	0x8(%rbp), %rdx
14233e73d: e8 de 14 c9 fe              	callq	0x140fcfc20 <.text+0xfcec20>
14233e742: 90                          	nop
14233e743: 48 83 c4 38                 	addq	$0x38, %rsp
14233e747: 5b                          	popq	%rbx
14233e748: 5f                          	popq	%rdi
14233e749: 5e                          	popq	%rsi
14233e74a: 41 5c                       	popq	%r12
14233e74c: 41 5d                       	popq	%r13
14233e74e: 41 5e                       	popq	%r14
14233e750: 41 5f                       	popq	%r15
14233e752: 5d                          	popq	%rbp
14233e753: c3                          	retq
