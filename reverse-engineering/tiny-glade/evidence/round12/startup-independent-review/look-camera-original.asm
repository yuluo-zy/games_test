
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140873de0: 55                          	pushq	%rbp
140873de1: 41 57                       	pushq	%r15
140873de3: 41 56                       	pushq	%r14
140873de5: 41 55                       	pushq	%r13
140873de7: 41 54                       	pushq	%r12
140873de9: 56                          	pushq	%rsi
140873dea: 57                          	pushq	%rdi
140873deb: 53                          	pushq	%rbx
140873dec: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140873df3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140873dfb: 48 c7 45 30 fe ff ff ff     	movq	$-0x2, 0x30(%rbp)
140873e03: 48 89 d6                    	movq	%rdx, %rsi
140873e06: 48 89 cf                    	movq	%rcx, %rdi
140873e09: e8 f2 2b 3a 00              	callq	0x140c16a00 <.text+0xc15a00>
140873e0e: 84 c0                       	testb	%al, %al
140873e10: 74 5f                       	je	0x140873e71 <.text+0x872e71>
140873e12: 4c 8d 35 df 40 22 02        	leaq	0x22240df(%rip), %r14   # 0x142a97ef8
140873e19: bb 35 00 00 00              	movl	$0x35, %ebx
140873e1e: ba 35 00 00 00              	movl	$0x35, %edx
140873e23: 4c 89 f1                    	movq	%r14, %rcx
140873e26: e8 25 e3 ff ff              	callq	0x140872150 <.text+0x871150>
140873e2b: 48 83 f8 01                 	cmpq	$0x1, %rax
140873e2f: 0f 85 b1 00 00 00           	jne	0x140873ee6 <.text+0x872ee6>
140873e35: 49 89 d1                    	movq	%rdx, %r9
140873e38: 48 85 d2                    	testq	%rdx, %rdx
140873e3b: 74 3d                       	je	0x140873e7a <.text+0x872e7a>
140873e3d: 49 83 f9 35                 	cmpq	$0x35, %r9
140873e41: 73 35                       	jae	0x140873e78 <.text+0x872e78>
140873e43: 48 8d 05 ae 40 22 02        	leaq	0x22240ae(%rip), %rax   # 0x142a97ef8
140873e4a: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140873e4f: 7f 29                       	jg	0x140873e7a <.text+0x872e7a>
140873e51: 48 8d 05 98 41 22 02        	leaq	0x2224198(%rip), %rax   # 0x142a97ff0
140873e58: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140873e5d: 48 8d 0d 94 40 22 02        	leaq	0x2224094(%rip), %rcx   # 0x142a97ef8
140873e64: ba 35 00 00 00              	movl	$0x35, %edx
140873e69: 45 31 c0                    	xorl	%r8d, %r8d
140873e6c: e8 cf 5a 06 02              	callq	0x1428d9940 <.text+0x28d8940>
140873e71: 31 c0                       	xorl	%eax, %eax
140873e73: e9 ce 01 00 00              	jmp	0x140874046 <.text+0x873046>
140873e78: 75 d7                       	jne	0x140873e51 <.text+0x872e51>
140873e7a: 4c 8d 35 77 40 22 02        	leaq	0x2224077(%rip), %r14   # 0x142a97ef8
140873e81: 4c 89 f1                    	movq	%r14, %rcx
140873e84: 4c 89 ca                    	movq	%r9, %rdx
140873e87: e8 c4 e2 ff ff              	callq	0x140872150 <.text+0x871150>
140873e8c: 48 83 f8 01                 	cmpq	$0x1, %rax
140873e90: 75 54                       	jne	0x140873ee6 <.text+0x872ee6>
140873e92: 49 89 d0                    	movq	%rdx, %r8
140873e95: 49 83 c0 02                 	addq	$0x2, %r8
140873e99: 74 39                       	je	0x140873ed4 <.text+0x872ed4>
140873e9b: 49 83 f8 35                 	cmpq	$0x35, %r8
140873e9f: 73 31                       	jae	0x140873ed2 <.text+0x872ed2>
140873ea1: 48 8d 05 50 40 22 02        	leaq	0x2224050(%rip), %rax   # 0x142a97ef8
140873ea8: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140873ead: 7f 25                       	jg	0x140873ed4 <.text+0x872ed4>
140873eaf: 48 8d 05 52 41 22 02        	leaq	0x2224152(%rip), %rax   # 0x142a98008
140873eb6: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140873ebb: 48 8d 0d 36 40 22 02        	leaq	0x2224036(%rip), %rcx   # 0x142a97ef8
140873ec2: ba 35 00 00 00              	movl	$0x35, %edx
140873ec7: 41 b9 35 00 00 00           	movl	$0x35, %r9d
140873ecd: e8 6e 5a 06 02              	callq	0x1428d9940 <.text+0x28d8940>
140873ed2: 75 db                       	jne	0x140873eaf <.text+0x872eaf>
140873ed4: bb 33 00 00 00              	movl	$0x33, %ebx
140873ed9: 48 29 d3                    	subq	%rdx, %rbx
140873edc: 4c 8d 35 15 40 22 02        	leaq	0x2224015(%rip), %r14   # 0x142a97ef8
140873ee3: 4d 01 c6                    	addq	%r8, %r14
140873ee6: 48 8d 05 4c 43 22 02        	leaq	0x222434c(%rip), %rax   # 0x142a98239
140873eed: 4c 8d 3d 14 43 22 02        	leaq	0x2224314(%rip), %r15   # 0x142a98208
140873ef4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140873f00: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140873f04: 85 c9                       	testl	%ecx, %ecx
140873f06: 78 18                       	js	0x140873f20 <.text+0x872f20>
140873f08: 48 ff c8                    	decq	%rax
140873f0b: 83 f9 5c                    	cmpl	$0x5c, %ecx
140873f0e: 75 69                       	jne	0x140873f79 <.text+0x872f79>
140873f10: eb 7d                       	jmp	0x140873f8f <.text+0x872f8f>
140873f12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140873f20: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140873f24: 80 fa c0                    	cmpb	$-0x40, %dl
140873f27: 7d 25                       	jge	0x140873f4e <.text+0x872f4e>
140873f29: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140873f2e: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140873f32: 7d 23                       	jge	0x140873f57 <.text+0x872f57>
140873f34: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140873f39: 48 83 c0 fc                 	addq	$-0x4, %rax
140873f3d: 41 83 e1 07                 	andl	$0x7, %r9d
140873f41: 41 c1 e1 06                 	shll	$0x6, %r9d
140873f45: 41 83 e0 3f                 	andl	$0x3f, %r8d
140873f49: 45 09 c8                    	orl	%r9d, %r8d
140873f4c: eb 11                       	jmp	0x140873f5f <.text+0x872f5f>
140873f4e: 48 83 c0 fe                 	addq	$-0x2, %rax
140873f52: 83 e2 1f                    	andl	$0x1f, %edx
140873f55: eb 12                       	jmp	0x140873f69 <.text+0x872f69>
140873f57: 48 83 c0 fd                 	addq	$-0x3, %rax
140873f5b: 41 83 e0 0f                 	andl	$0xf, %r8d
140873f5f: 41 c1 e0 06                 	shll	$0x6, %r8d
140873f63: 83 e2 3f                    	andl	$0x3f, %edx
140873f66: 44 09 c2                    	orl	%r8d, %edx
140873f69: c1 e2 06                    	shll	$0x6, %edx
140873f6c: 80 e1 3f                    	andb	$0x3f, %cl
140873f6f: 0f b6 c9                    	movzbl	%cl, %ecx
140873f72: 09 d1                       	orl	%edx, %ecx
140873f74: 83 f9 5c                    	cmpl	$0x5c, %ecx
140873f77: 74 16                       	je	0x140873f8f <.text+0x872f8f>
140873f79: 83 f9 2f                    	cmpl	$0x2f, %ecx
140873f7c: 74 11                       	je	0x140873f8f <.text+0x872f8f>
140873f7e: 4c 39 f8                    	cmpq	%r15, %rax
140873f81: 0f 85 79 ff ff ff           	jne	0x140873f00 <.text+0x872f00>
140873f87: 41 bc 31 00 00 00           	movl	$0x31, %r12d
140873f8d: eb 50                       	jmp	0x140873fdf <.text+0x872fdf>
140873f8f: 4c 8d 3d 72 42 22 02        	leaq	0x2224272(%rip), %r15   # 0x142a98208
140873f96: 4c 29 f8                    	subq	%r15, %rax
140873f99: 49 89 c0                    	movq	%rax, %r8
140873f9c: 49 ff c0                    	incq	%r8
140873f9f: 74 32                       	je	0x140873fd3 <.text+0x872fd3>
140873fa1: 49 83 f8 31                 	cmpq	$0x31, %r8
140873fa5: 73 2a                       	jae	0x140873fd1 <.text+0x872fd1>
140873fa7: 43 80 3c 38 bf              	cmpb	$-0x41, (%r8,%r15)
140873fac: 7f 25                       	jg	0x140873fd3 <.text+0x872fd3>
140873fae: 48 8d 05 1b 40 22 02        	leaq	0x222401b(%rip), %rax   # 0x142a97fd0
140873fb5: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140873fba: 48 8d 0d 47 42 22 02        	leaq	0x2224247(%rip), %rcx   # 0x142a98208
140873fc1: ba 31 00 00 00              	movl	$0x31, %edx
140873fc6: 41 b9 31 00 00 00           	movl	$0x31, %r9d
140873fcc: e8 6f 59 06 02              	callq	0x1428d9940 <.text+0x28d8940>
140873fd1: 75 db                       	jne	0x140873fae <.text+0x872fae>
140873fd3: 41 bc 30 00 00 00           	movl	$0x30, %r12d
140873fd9: 49 29 c4                    	subq	%rax, %r12
140873fdc: 4d 01 c7                    	addq	%r8, %r15
140873fdf: e8 4c 41 3a 00              	callq	0x140c18130 <.text+0xc17130>
140873fe4: 48 89 c1                    	movq	%rax, %rcx
140873fe7: 48 8b 00                    	movq	(%rax), %rax
140873fea: 48 83 f8 01                 	cmpq	$0x1, %rax
140873fee: 0f 85 a8 02 00 00           	jne	0x14087429c <.text+0x87329c>
140873ff4: 48 83 c1 08                 	addq	$0x8, %rcx
140873ff8: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140873ffc: 0f 85 be 02 00 00           	jne	0x1408742c0 <.text+0x8732c0>
140874002: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140874009: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
14087400d: 48 83 c1 08                 	addq	$0x8, %rcx
140874011: 4c 89 64 24 20              	movq	%r12, 0x20(%rsp)
140874016: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
14087401f: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140874028: 4c 89 f2                    	movq	%r14, %rdx
14087402b: 49 89 d8                    	movq	%rbx, %r8
14087402e: 4d 89 f9                    	movq	%r15, %r9
140874031: e8 0a 2d 3a 00              	callq	0x140c16d40 <.text+0xc15d40>
140874036: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
14087403a: 48 ff 01                    	incq	(%rcx)
14087403d: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140874041: b8 01 00 00 00              	movl	$0x1, %eax
140874046: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14087404a: 48 8b 0f                    	movq	(%rdi), %rcx
14087404d: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140874051: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140874055: 8b 47 14                    	movl	0x14(%rdi), %eax
140874058: 89 45 14                    	movl	%eax, 0x14(%rbp)
14087405b: 48 8b 99 f8 00 00 00        	movq	0xf8(%rcx), %rbx
140874062: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140874069: 4c 8d 24 83                 	leaq	(%rbx,%rax,4), %r12
14087406d: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
140874071: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
140874078: 88 45 28                    	movb	%al, 0x28(%rbp)
14087407b: 48 8b 06                    	movq	(%rsi), %rax
14087407e: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140874082: b8 08 00 00 00              	movl	$0x8, %eax
140874087: 48 89 45 00                 	movq	%rax, (%rbp)
14087408b: 31 f6                       	xorl	%esi, %esi
14087408d: 4c 8d 7d bc                 	leaq	-0x44(%rbp), %r15
140874091: 45 31 ed                    	xorl	%r13d, %r13d
140874094: 31 ff                       	xorl	%edi, %edi
140874096: 80 7d 28 00                 	cmpb	$0x0, 0x28(%rbp)
14087409a: 74 44                       	je	0x1408740e0 <.text+0x8730e0>
14087409c: 4c 39 ef                    	cmpq	%r13, %rdi
14087409f: 0f 85 c5 00 00 00           	jne	0x14087416a <.text+0x87316a>
1408740a5: 4c 39 e3                    	cmpq	%r12, %rbx
1408740a8: 0f 84 83 01 00 00           	je	0x140874231 <.text+0x873231>
1408740ae: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1408740b2: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
1408740b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1408740c0: 8b 0b                       	movl	(%rbx), %ecx
1408740c2: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1408740c6: 4c 8b 6c c8 10              	movq	0x10(%rax,%rcx,8), %r13
1408740cb: 4d 85 ed                    	testq	%r13, %r13
1408740ce: 75 60                       	jne	0x140874130 <.text+0x873130>
1408740d0: 48 83 c3 04                 	addq	$0x4, %rbx
1408740d4: 4c 39 e3                    	cmpq	%r12, %rbx
1408740d7: 75 e7                       	jne	0x1408740c0 <.text+0x8730c0>
1408740d9: e9 53 01 00 00              	jmp	0x140874231 <.text+0x873231>
1408740de: 66 90                       	nop
1408740e0: 4c 39 ef                    	cmpq	%r13, %rdi
1408740e3: 0f 85 df 00 00 00           	jne	0x1408741c8 <.text+0x8731c8>
1408740e9: 4c 39 e3                    	cmpq	%r12, %rbx
1408740ec: 0f 84 3f 01 00 00           	je	0x140874231 <.text+0x873231>
1408740f2: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1408740f6: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
1408740fd: 0f 1f 00                    	nopl	(%rax)
140874100: 8b 0b                       	movl	(%rbx), %ecx
140874102: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140874106: 48 c1 e1 05                 	shlq	$0x5, %rcx
14087410a: 4c 8b 6c 08 58              	movq	0x58(%rax,%rcx), %r13
14087410f: 4d 85 ed                    	testq	%r13, %r13
140874112: 75 5c                       	jne	0x140874170 <.text+0x873170>
140874114: 48 83 c3 04                 	addq	$0x4, %rbx
140874118: 4c 39 e3                    	cmpq	%r12, %rbx
14087411b: 75 e3                       	jne	0x140874100 <.text+0x873100>
14087411d: e9 0f 01 00 00              	jmp	0x140874231 <.text+0x873231>
140874122: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140874130: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140874134: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140874138: 48 8b 89 10 01 00 00        	movq	0x110(%rcx), %rcx
14087413f: 48 8b 50 18                 	movq	0x18(%rax), %rdx
140874143: 48 8b 40 38                 	movq	0x38(%rax), %rax
140874147: 48 8b 04 c8                 	movq	(%rax,%rcx,8), %rax
14087414b: 48 f7 d0                    	notq	%rax
14087414e: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140874152: 48 c1 e0 04                 	shlq	$0x4, %rax
140874156: 48 8b 74 02 10              	movq	0x10(%rdx,%rax), %rsi
14087415b: 48 8b 44 02 28              	movq	0x28(%rdx,%rax), %rax
140874160: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140874164: 48 83 c3 04                 	addq	$0x4, %rbx
140874168: 31 ff                       	xorl	%edi, %edi
14087416a: 89 f8                       	movl	%edi, %eax
14087416c: eb 69                       	jmp	0x1408741d7 <.text+0x8731d7>
14087416e: 66 90                       	nop
140874170: 48 01 c8                    	addq	%rcx, %rax
140874173: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140874179: 48 8b 55 20                 	movq	0x20(%rbp), %rdx
14087417d: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
140874184: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140874188: 4c 8b 45 08                 	movq	0x8(%rbp), %r8
14087418c: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
140874193: 4c 8b 4c ca 18              	movq	0x18(%rdx,%rcx,8), %r9
140874198: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
14087419d: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
1408741a1: 48 f7 d1                    	notq	%rcx
1408741a4: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1408741a8: 48 c1 e1 04                 	shlq	$0x4, %rcx
1408741ac: 49 8b 74 09 10              	movq	0x10(%r9,%rcx), %rsi
1408741b1: 49 8b 4c 09 28              	movq	0x28(%r9,%rcx), %rcx
1408741b6: 48 89 4d 18                 	movq	%rcx, 0x18(%rbp)
1408741ba: 48 8b 40 50                 	movq	0x50(%rax), %rax
1408741be: 48 89 45 00                 	movq	%rax, (%rbp)
1408741c2: 48 83 c3 04                 	addq	$0x4, %rbx
1408741c6: 31 ff                       	xorl	%edi, %edi
1408741c8: 48 89 f8                    	movq	%rdi, %rax
1408741cb: 48 c1 e0 04                 	shlq	$0x4, %rax
1408741cf: 48 8b 4d 00                 	movq	(%rbp), %rcx
1408741d3: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
1408741d7: 4c 8d 34 40                 	leaq	(%rax,%rax,2), %r14
1408741db: 49 c1 e6 04                 	shlq	$0x4, %r14
1408741df: 49 01 f6                    	addq	%rsi, %r14
1408741e2: 48 8b 4d 18                 	movq	0x18(%rbp), %rcx
1408741e6: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1408741ea: 8b 4d 14                    	movl	0x14(%rbp), %ecx
1408741ed: 89 08                       	movl	%ecx, (%rax)
1408741ef: 4c 89 f9                    	movq	%r15, %rcx
1408741f2: 48 8b 55 e8                 	movq	-0x18(%rbp), %rdx
1408741f6: e8 45 2b 42 00              	callq	0x140c96d40 <.text+0xc95d40>
1408741fb: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
140874205: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
140874209: c7 45 e0 00 00 00 00        	movl	$0x0, -0x20(%rbp)
140874210: 4c 89 f1                    	movq	%r14, %rcx
140874213: 4c 89 fa                    	movq	%r15, %rdx
140874216: 4c 8d 45 d8                 	leaq	-0x28(%rbp), %r8
14087421a: e8 e1 8b 42 00              	callq	0x140c9ce00 <.text+0xc9be00>
14087421f: 48 ff c7                    	incq	%rdi
140874222: 80 7d 28 00                 	cmpb	$0x0, 0x28(%rbp)
140874226: 0f 84 b4 fe ff ff           	je	0x1408740e0 <.text+0x8730e0>
14087422c: e9 6b fe ff ff              	jmp	0x14087409c <.text+0x87309c>
140874231: 48 83 7d f0 00              	cmpq	$0x0, -0x10(%rbp)
140874236: 74 3a                       	je	0x140874272 <.text+0x873272>
140874238: e8 f3 3e 3a 00              	callq	0x140c18130 <.text+0xc17130>
14087423d: 48 89 c1                    	movq	%rax, %rcx
140874240: 48 8b 00                    	movq	(%rax), %rax
140874243: 48 83 f8 01                 	cmpq	$0x1, %rax
140874247: 75 3d                       	jne	0x140874286 <.text+0x873286>
140874249: 48 83 c1 08                 	addq	$0x8, %rcx
14087424d: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140874251: 75 6d                       	jne	0x1408742c0 <.text+0x8732c0>
140874253: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
14087425a: 48 89 4d 28                 	movq	%rcx, 0x28(%rbp)
14087425e: 48 83 c1 08                 	addq	$0x8, %rcx
140874262: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140874266: e8 85 2e 3a 00              	callq	0x140c170f0 <.text+0xc160f0>
14087426b: 48 8b 45 28                 	movq	0x28(%rbp), %rax
14087426f: 48 ff 00                    	incq	(%rax)
140874272: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140874279: 5b                          	popq	%rbx
14087427a: 5f                          	popq	%rdi
14087427b: 5e                          	popq	%rsi
14087427c: 41 5c                       	popq	%r12
14087427e: 41 5d                       	popq	%r13
140874280: 41 5e                       	popq	%r14
140874282: 41 5f                       	popq	%r15
140874284: 5d                          	popq	%rbp
140874285: c3                          	retq
140874286: 48 85 c0                    	testq	%rax, %rax
140874289: 75 29                       	jne	0x1408742b4 <.text+0x8732b4>
14087428b: 31 d2                       	xorl	%edx, %edx
14087428d: e8 3e 45 08 02              	callq	0x1428f87d0 <.text+0x28f77d0>
140874292: 48 89 c1                    	movq	%rax, %rcx
140874295: 48 85 c0                    	testq	%rax, %rax
140874298: 75 b3                       	jne	0x14087424d <.text+0x87324d>
14087429a: eb 18                       	jmp	0x1408742b4 <.text+0x8732b4>
14087429c: 48 85 c0                    	testq	%rax, %rax
14087429f: 75 13                       	jne	0x1408742b4 <.text+0x8732b4>
1408742a1: 31 d2                       	xorl	%edx, %edx
1408742a3: e8 28 45 08 02              	callq	0x1428f87d0 <.text+0x28f77d0>
1408742a8: 48 89 c1                    	movq	%rax, %rcx
1408742ab: 48 85 c0                    	testq	%rax, %rax
1408742ae: 0f 85 44 fd ff ff           	jne	0x140873ff8 <.text+0x872ff8>
1408742b4: 48 8d 0d e5 67 3c 02        	leaq	0x23c67e5(%rip), %rcx   # 0x142c3aaa0
1408742bb: e8 10 3e 06 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1408742c0: 48 8d 0d a9 68 3c 02        	leaq	0x23c68a9(%rip), %rcx   # 0x142c3ab70
1408742c7: e8 84 4f 06 02              	callq	0x1428d9250 <.text+0x28d8250>
1408742cc: cc                          	int3
1408742cd: 0f 1f 00                    	nopl	(%rax)
1408742d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408742d5: 55                          	pushq	%rbp
1408742d6: 41 57                       	pushq	%r15
1408742d8: 41 56                       	pushq	%r14
1408742da: 41 55                       	pushq	%r13
1408742dc: 41 54                       	pushq	%r12
1408742de: 56                          	pushq	%rsi
1408742df: 57                          	pushq	%rdi
1408742e0: 53                          	pushq	%rbx
1408742e1: 48 83 ec 38                 	subq	$0x38, %rsp
1408742e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408742ec: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1408742f0: 48 ff 00                    	incq	(%rax)
1408742f3: 48 83 c4 38                 	addq	$0x38, %rsp
1408742f7: 5b                          	popq	%rbx
1408742f8: 5f                          	popq	%rdi
1408742f9: 5e                          	popq	%rsi
1408742fa: 41 5c                       	popq	%r12
1408742fc: 41 5d                       	popq	%r13
1408742fe: 41 5e                       	popq	%r14
140874300: 41 5f                       	popq	%r15
140874302: 5d                          	popq	%rbp
140874303: c3                          	retq
140874304: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140874310: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140874315: 55                          	pushq	%rbp
140874316: 41 57                       	pushq	%r15
140874318: 41 56                       	pushq	%r14
14087431a: 41 55                       	pushq	%r13
14087431c: 41 54                       	pushq	%r12
14087431e: 56                          	pushq	%rsi
14087431f: 57                          	pushq	%rdi
140874320: 53                          	pushq	%rbx
140874321: 48 83 ec 38                 	subq	$0x38, %rsp
140874325: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14087432c: 48 83 7d f0 00              	cmpq	$0x0, -0x10(%rbp)
140874331: 74 20                       	je	0x140874353 <.text+0x873353>
140874333: 48 8d 45 f8                 	leaq	-0x8(%rbp), %rax
140874337: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
14087433b: 48 8d 45 d0                 	leaq	-0x30(%rbp), %rax
14087433f: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
140874343: 48 8d 0d 26 3c 22 02        	leaq	0x2223c26(%rip), %rcx   # 0x142a97f70
14087434a: 48 8d 55 c8                 	leaq	-0x38(%rbp), %rdx
14087434e: e8 7d af ab 00              	callq	0x14132f2d0 <.text+0x132e2d0>
140874353: 90                          	nop
140874354: 48 83 c4 38                 	addq	$0x38, %rsp
140874358: 5b                          	popq	%rbx
140874359: 5f                          	popq	%rdi
14087435a: 5e                          	popq	%rsi
14087435b: 41 5c                       	popq	%r12
14087435d: 41 5d                       	popq	%r13
14087435f: 41 5e                       	popq	%r14
140874361: 41 5f                       	popq	%r15
140874363: 5d                          	popq	%rbp
140874364: c3                          	retq
140874365: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140874370: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140874375: 55                          	pushq	%rbp
140874376: 41 57                       	pushq	%r15
140874378: 41 56                       	pushq	%r14
14087437a: 41 55                       	pushq	%r13
14087437c: 41 54                       	pushq	%r12
14087437e: 56                          	pushq	%rsi
14087437f: 57                          	pushq	%rdi
140874380: 53                          	pushq	%rbx
140874381: 48 83 ec 38                 	subq	$0x38, %rsp
140874385: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14087438c: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140874390: 48 ff 00                    	incq	(%rax)
140874393: 48 83 c4 38                 	addq	$0x38, %rsp
140874397: 5b                          	popq	%rbx
140874398: 5f                          	popq	%rdi
140874399: 5e                          	popq	%rsi
14087439a: 41 5c                       	popq	%r12
14087439c: 41 5d                       	popq	%r13
14087439e: 41 5e                       	popq	%r14
1408743a0: 41 5f                       	popq	%r15
1408743a2: 5d                          	popq	%rbp
1408743a3: c3                          	retq
