
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14142ce00: 55                          	pushq	%rbp
14142ce01: 41 57                       	pushq	%r15
14142ce03: 41 56                       	pushq	%r14
14142ce05: 41 55                       	pushq	%r13
14142ce07: 41 54                       	pushq	%r12
14142ce09: 56                          	pushq	%rsi
14142ce0a: 57                          	pushq	%rdi
14142ce0b: 53                          	pushq	%rbx
14142ce0c: 48 83 ec 58                 	subq	$0x58, %rsp
14142ce10: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
14142ce15: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14142ce1d: 48 89 ce                    	movq	%rcx, %rsi
14142ce20: 4c 8b 75 78                 	movq	0x78(%rbp), %r14
14142ce24: 48 8d 79 18                 	leaq	0x18(%rcx), %rdi
14142ce28: f6 41 60 01                 	testb	$0x1, 0x60(%rcx)
14142ce2c: 74 06                       	je	0x14142ce34 <.text+0x142be34>
14142ce2e: 0f b6 46 61                 	movzbl	0x61(%rsi), %eax
14142ce32: eb 6a                       	jmp	0x14142ce9e <.text+0x142be9e>
14142ce34: 48 8b 4e 28                 	movq	0x28(%rsi), %rcx
14142ce38: 48 39 4e 30                 	cmpq	%rcx, 0x30(%rsi)
14142ce3c: 75 31                       	jne	0x14142ce6f <.text+0x142be6f>
14142ce3e: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
14142ce42: 48 89 fa                    	movq	%rdi, %rdx
14142ce45: e8 f6 3d 67 ff              	callq	0x140aa0c40 <.text+0xa9fc40>
14142ce4a: 0f b6 45 e8                 	movzbl	-0x18(%rbp), %eax
14142ce4e: 3c 02                       	cmpb	$0x2, %al
14142ce50: 0f 84 be 03 00 00           	je	0x14142d214 <.text+0x142c214>
14142ce56: a8 01                       	testb	$0x1, %al
14142ce58: 0f 85 d1 03 00 00           	jne	0x14142d22f <.text+0x142c22f>
14142ce5e: 0f b6 45 e9                 	movzbl	-0x17(%rbp), %eax
14142ce62: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
14142ce66: 48 ff c1                    	incq	%rcx
14142ce69: 3c 0a                       	cmpb	$0xa, %al
14142ce6b: 74 1c                       	je	0x14142ce89 <.text+0x142be89>
14142ce6d: eb 24                       	jmp	0x14142ce93 <.text+0x142be93>
14142ce6f: 48 8b 46 18                 	movq	0x18(%rsi), %rax
14142ce73: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
14142ce77: 48 ff c1                    	incq	%rcx
14142ce7a: 48 89 4e 28                 	movq	%rcx, 0x28(%rsi)
14142ce7e: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
14142ce82: 48 ff c1                    	incq	%rcx
14142ce85: 3c 0a                       	cmpb	$0xa, %al
14142ce87: 75 0a                       	jne	0x14142ce93 <.text+0x142be93>
14142ce89: 48 01 4e 58                 	addq	%rcx, 0x58(%rsi)
14142ce8d: 48 ff 46 48                 	incq	0x48(%rsi)
14142ce91: 31 c9                       	xorl	%ecx, %ecx
14142ce93: 48 89 4e 50                 	movq	%rcx, 0x50(%rsi)
14142ce97: c6 46 60 01                 	movb	$0x1, 0x60(%rsi)
14142ce9b: 88 46 61                    	movb	%al, 0x61(%rsi)
14142ce9e: 0f b6 c0                    	movzbl	%al, %eax
14142cea1: 3c 20                       	cmpb	$0x20, %al
14142cea3: 77 19                       	ja	0x14142cebe <.text+0x142bebe>
14142cea5: 48 b9 00 26 00 00 01 00 00 00       	movabsq	$0x100002600, %rcx # imm = 0x100002600
14142ceaf: 48 0f a3 c1                 	btq	%rax, %rcx
14142ceb3: 73 09                       	jae	0x14142cebe <.text+0x142bebe>
14142ceb5: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
14142ceb9: e9 76 ff ff ff              	jmp	0x14142ce34 <.text+0x142be34>
14142cebe: 83 f8 7b                    	cmpl	$0x7b, %eax
14142cec1: 0f 85 81 03 00 00           	jne	0x14142d248 <.text+0x142c248>
14142cec7: fe 4e 68                    	decb	0x68(%rsi)
14142ceca: 0f 84 a9 04 00 00           	je	0x14142d379 <.text+0x142c379>
14142ced0: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
14142ced4: 41 b4 01                    	movb	$0x1, %r12b
14142ced7: 48 8d 5d e8                 	leaq	-0x18(%rbp), %rbx
14142cedb: 49 bf 00 26 00 00 01 00 00 00       	movabsq	$0x100002600, %r15 # imm = 0x100002600
14142cee5: 49 bd 00 22 00 00 01 00 00 00       	movabsq	$0x100002200, %r13 # imm = 0x100002200
14142ceef: eb 20                       	jmp	0x14142cf11 <.text+0x142bf11>
14142cef1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14142cf00: 48 89 f1                    	movq	%rsi, %rcx
14142cf03: e8 c8 c7 fd ff              	callq	0x1414096d0 <.text+0x14086d0>
14142cf08: 48 85 c0                    	testq	%rax, %rax
14142cf0b: 0f 85 5f 02 00 00           	jne	0x14142d170 <.text+0x142c170>
14142cf11: f6 46 60 01                 	testb	$0x1, 0x60(%rsi)
14142cf15: 74 23                       	je	0x14142cf3a <.text+0x142bf3a>
14142cf17: 0f b6 46 61                 	movzbl	0x61(%rsi), %eax
14142cf1b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14142cf20: 0f b6 c8                    	movzbl	%al, %ecx
14142cf23: 80 f9 2c                    	cmpb	$0x2c, %cl
14142cf26: 0f 87 54 01 00 00           	ja	0x14142d080 <.text+0x142c080>
14142cf2c: 49 0f a3 cf                 	btq	%rcx, %r15
14142cf30: 0f 83 8a 00 00 00           	jae	0x14142cfc0 <.text+0x142bfc0>
14142cf36: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
14142cf3a: 48 8b 4e 28                 	movq	0x28(%rsi), %rcx
14142cf3e: 48 39 4e 30                 	cmpq	%rcx, 0x30(%rsi)
14142cf42: 75 3c                       	jne	0x14142cf80 <.text+0x142bf80>
14142cf44: 48 89 d9                    	movq	%rbx, %rcx
14142cf47: 48 89 fa                    	movq	%rdi, %rdx
14142cf4a: e8 f1 3c 67 ff              	callq	0x140aa0c40 <.text+0xa9fc40>
14142cf4f: 0f b6 45 e8                 	movzbl	-0x18(%rbp), %eax
14142cf53: 3c 02                       	cmpb	$0x2, %al
14142cf55: 0f 84 af 03 00 00           	je	0x14142d30a <.text+0x142c30a>
14142cf5b: a8 01                       	testb	$0x1, %al
14142cf5d: 0f 85 e9 01 00 00           	jne	0x14142d14c <.text+0x142c14c>
14142cf63: 0f b6 45 e9                 	movzbl	-0x17(%rbp), %eax
14142cf67: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
14142cf6b: 48 ff c1                    	incq	%rcx
14142cf6e: 3c 0a                       	cmpb	$0xa, %al
14142cf70: 74 28                       	je	0x14142cf9a <.text+0x142bf9a>
14142cf72: eb 30                       	jmp	0x14142cfa4 <.text+0x142bfa4>
14142cf74: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14142cf80: 48 8b 46 18                 	movq	0x18(%rsi), %rax
14142cf84: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
14142cf88: 48 ff c1                    	incq	%rcx
14142cf8b: 48 89 4e 28                 	movq	%rcx, 0x28(%rsi)
14142cf8f: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
14142cf93: 48 ff c1                    	incq	%rcx
14142cf96: 3c 0a                       	cmpb	$0xa, %al
14142cf98: 75 0a                       	jne	0x14142cfa4 <.text+0x142bfa4>
14142cf9a: 48 01 4e 58                 	addq	%rcx, 0x58(%rsi)
14142cf9e: 48 ff 46 48                 	incq	0x48(%rsi)
14142cfa2: 31 c9                       	xorl	%ecx, %ecx
14142cfa4: 48 89 4e 50                 	movq	%rcx, 0x50(%rsi)
14142cfa8: c6 46 60 01                 	movb	$0x1, 0x60(%rsi)
14142cfac: 88 46 61                    	movb	%al, 0x61(%rsi)
14142cfaf: e9 6c ff ff ff              	jmp	0x14142cf20 <.text+0x142bf20>
14142cfb4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14142cfc0: 48 83 f9 2c                 	cmpq	$0x2c, %rcx
14142cfc4: 0f 85 b6 00 00 00           	jne	0x14142d080 <.text+0x142c080>
14142cfca: 41 f6 c4 01                 	testb	$0x1, %r12b
14142cfce: 74 26                       	je	0x14142cff6 <.text+0x142bff6>
14142cfd0: e9 4a 03 00 00              	jmp	0x14142d31f <.text+0x142c31f>
14142cfd5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14142cfe0: 48 01 4e 58                 	addq	%rcx, 0x58(%rsi)
14142cfe4: 48 ff 46 48                 	incq	0x48(%rsi)
14142cfe8: 48 c7 46 50 00 00 00 00     	movq	$0x0, 0x50(%rsi)
14142cff0: 66 c7 46 60 01 0a           	movw	$0xa01, 0x60(%rsi)      # imm = 0xA01
14142cff6: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
14142cffa: 48 8b 4e 28                 	movq	0x28(%rsi), %rcx
14142cffe: 48 39 4e 30                 	cmpq	%rcx, 0x30(%rsi)
14142d002: 75 3c                       	jne	0x14142d040 <.text+0x142c040>
14142d004: 48 89 d9                    	movq	%rbx, %rcx
14142d007: 48 89 fa                    	movq	%rdi, %rdx
14142d00a: e8 31 3c 67 ff              	callq	0x140aa0c40 <.text+0xa9fc40>
14142d00f: 0f b6 45 e8                 	movzbl	-0x18(%rbp), %eax
14142d013: 3c 02                       	cmpb	$0x2, %al
14142d015: 0f 84 3c 01 00 00           	je	0x14142d157 <.text+0x142c157>
14142d01b: a8 01                       	testb	$0x1, %al
14142d01d: 0f 85 29 01 00 00           	jne	0x14142d14c <.text+0x142c14c>
14142d023: 0f b6 45 e9                 	movzbl	-0x17(%rbp), %eax
14142d027: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
14142d02b: 48 ff c1                    	incq	%rcx
14142d02e: 3c 0a                       	cmpb	$0xa, %al
14142d030: 74 ae                       	je	0x14142cfe0 <.text+0x142bfe0>
14142d032: eb 26                       	jmp	0x14142d05a <.text+0x142c05a>
14142d034: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14142d040: 48 8b 46 18                 	movq	0x18(%rsi), %rax
14142d044: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
14142d048: 48 ff c1                    	incq	%rcx
14142d04b: 48 89 4e 28                 	movq	%rcx, 0x28(%rsi)
14142d04f: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
14142d053: 48 ff c1                    	incq	%rcx
14142d056: 3c 0a                       	cmpb	$0xa, %al
14142d058: 74 86                       	je	0x14142cfe0 <.text+0x142bfe0>
14142d05a: 48 89 4e 50                 	movq	%rcx, 0x50(%rsi)
14142d05e: c6 46 60 01                 	movb	$0x1, 0x60(%rsi)
14142d062: 88 46 61                    	movb	%al, 0x61(%rsi)
14142d065: 3c 20                       	cmpb	$0x20, %al
14142d067: 77 2d                       	ja	0x14142d096 <.text+0x142c096>
14142d069: 0f b6 c8                    	movzbl	%al, %ecx
14142d06c: 49 0f a3 cd                 	btq	%rcx, %r13
14142d070: 72 84                       	jb	0x14142cff6 <.text+0x142bff6>
14142d072: eb 22                       	jmp	0x14142d096 <.text+0x142c096>
14142d074: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14142d080: 83 f9 7d                    	cmpl	$0x7d, %ecx
14142d083: 0f 84 bf 00 00 00           	je	0x14142d148 <.text+0x142c148>
14142d089: 41 f6 c4 01                 	testb	$0x1, %r12b
14142d08d: 0f 84 d1 02 00 00           	je	0x14142d364 <.text+0x142c364>
14142d093: 45 31 e4                    	xorl	%r12d, %r12d
14142d096: 3c 22                       	cmpb	$0x22, %al
14142d098: 0f 85 4f 02 00 00           	jne	0x14142d2ed <.text+0x142c2ed>
14142d09e: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
14142d0a2: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
14142d0aa: 48 89 d9                    	movq	%rbx, %rcx
14142d0ad: 48 89 fa                    	movq	%rdi, %rdx
14142d0b0: 49 89 f0                    	movq	%rsi, %r8
14142d0b3: e8 d8 d2 71 ff              	callq	0x140b4a390 <.text+0xb49390>
14142d0b8: 83 7d e8 02                 	cmpl	$0x2, -0x18(%rbp)
14142d0bc: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14142d0c0: 0f 84 aa 00 00 00           	je	0x14142d170 <.text+0x142c170>
14142d0c6: 48 83 7d f8 07              	cmpq	$0x7, -0x8(%rbp)
14142d0cb: 0f 85 2f fe ff ff           	jne	0x14142cf00 <.text+0x142bf00>
14142d0d1: 8b 15 d8 3f 81 01           	movl	0x1813fd8(%rip), %edx   # 0x142c410af
14142d0d7: 33 10                       	xorl	(%rax), %edx
14142d0d9: 8b 0d d3 3f 81 01           	movl	0x1813fd3(%rip), %ecx   # 0x142c410b2
14142d0df: 33 48 03                    	xorl	0x3(%rax), %ecx
14142d0e2: 09 d1                       	orl	%edx, %ecx
14142d0e4: 0f 85 16 fe ff ff           	jne	0x14142cf00 <.text+0x142bf00>
14142d0ea: 48 89 f1                    	movq	%rsi, %rcx
14142d0ed: e8 fe 77 fd ff              	callq	0x1414048f0 <.text+0x14038f0>
14142d0f2: 48 85 c0                    	testq	%rax, %rax
14142d0f5: 75 79                       	jne	0x14142d170 <.text+0x142c170>
14142d0f7: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
14142d0fb: 48 89 f2                    	movq	%rsi, %rdx
14142d0fe: e8 8d 78 fb ff              	callq	0x1413e4990 <.text+0x13e3990>
14142d103: 48 8b 7d e8                 	movq	-0x18(%rbp), %rdi
14142d107: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14142d10b: 48 89 f9                    	movq	%rdi, %rcx
14142d10e: 48 f7 d9                    	negq	%rcx
14142d111: 70 5d                       	jo	0x14142d170 <.text+0x142c170>
14142d113: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
14142d117: 49 8b 16                    	movq	(%r14), %rdx
14142d11a: 48 89 d1                    	movq	%rdx, %rcx
14142d11d: 48 f7 d9                    	negq	%rcx
14142d120: 70 1b                       	jo	0x14142d13d <.text+0x142c13d>
14142d122: 73 19                       	jae	0x14142d13d <.text+0x142c13d>
14142d124: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
14142d128: 48 c1 e2 04                 	shlq	$0x4, %rdx
14142d12c: 49 89 c7                    	movq	%rax, %r15
14142d12f: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14142d135: e8 e6 6a 1e ff              	callq	0x140613c20 <.text+0x612c20>
14142d13a: 4c 89 f8                    	movq	%r15, %rax
14142d13d: 49 89 3e                    	movq	%rdi, (%r14)
14142d140: 49 89 46 08                 	movq	%rax, 0x8(%r14)
14142d144: 49 89 5e 10                 	movq	%rbx, 0x10(%r14)
14142d148: 31 c0                       	xorl	%eax, %eax
14142d14a: eb 24                       	jmp	0x14142d170 <.text+0x142c170>
14142d14c: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14142d150: e8 4b c7 49 01              	callq	0x1428c98a0 <.text+0x28c88a0>
14142d155: eb 19                       	jmp	0x14142d170 <.text+0x142c170>
14142d157: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
14142d15b: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
14142d15f: 48 c7 45 e8 05 00 00 00     	movq	$0x5, -0x18(%rbp)
14142d167: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
14142d16b: e8 90 c6 49 01              	callq	0x1428c9800 <.text+0x28c8800>
14142d170: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14142d174: fe 46 68                    	incb	0x68(%rsi)
14142d177: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14142d17b: 48 89 f1                    	movq	%rsi, %rcx
14142d17e: e8 6d 78 fd ff              	callq	0x1414049f0 <.text+0x14039f0>
14142d183: 4c 8b 45 e0                 	movq	-0x20(%rbp), %r8
14142d187: 4d 85 c0                    	testq	%r8, %r8
14142d18a: 4c 0f 44 c0                 	cmoveq	%rax, %r8
14142d18e: 0f 94 c2                    	sete	%dl
14142d191: 48 85 c0                    	testq	%rax, %rax
14142d194: 0f 94 c1                    	sete	%cl
14142d197: 08 d1                       	orb	%dl, %cl
14142d199: 4c 89 c1                    	movq	%r8, %rcx
14142d19c: 75 5a                       	jne	0x14142d1f8 <.text+0x142c1f8>
14142d19e: 48 89 cf                    	movq	%rcx, %rdi
14142d1a1: 48 8b 08                    	movq	(%rax), %rcx
14142d1a4: 48 83 f9 01                 	cmpq	$0x1, %rcx
14142d1a8: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14142d1ac: 74 27                       	je	0x14142d1d5 <.text+0x142c1d5>
14142d1ae: 48 85 c9                    	testq	%rcx, %rcx
14142d1b1: 75 2e                       	jne	0x14142d1e1 <.text+0x142c1e1>
14142d1b3: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14142d1b7: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14142d1bb: 48 85 d2                    	testq	%rdx, %rdx
14142d1be: 74 21                       	je	0x14142d1e1 <.text+0x142c1e1>
14142d1c0: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14142d1c4: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14142d1c8: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14142d1ce: e8 4d 6a 1e ff              	callq	0x140613c20 <.text+0x612c20>
14142d1d3: eb 0c                       	jmp	0x14142d1e1 <.text+0x142c1e1>
14142d1d5: 48 89 c1                    	movq	%rax, %rcx
14142d1d8: 48 83 c1 08                 	addq	$0x8, %rcx
14142d1dc: e8 1f a8 1b ff              	callq	0x1405e7a00 <.text+0x5e6a00>
14142d1e1: ba 28 00 00 00              	movl	$0x28, %edx
14142d1e6: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14142d1ec: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
14142d1f0: e8 2b 6a 1e ff              	callq	0x140613c20 <.text+0x612c20>
14142d1f5: 48 89 f9                    	movq	%rdi, %rcx
14142d1f8: 48 85 c9                    	testq	%rcx, %rcx
14142d1fb: 0f 85 56 01 00 00           	jne	0x14142d357 <.text+0x142c357>
14142d201: 31 c0                       	xorl	%eax, %eax
14142d203: 48 83 c4 58                 	addq	$0x58, %rsp
14142d207: 5b                          	popq	%rbx
14142d208: 5f                          	popq	%rdi
14142d209: 5e                          	popq	%rsi
14142d20a: 41 5c                       	popq	%r12
14142d20c: 41 5d                       	popq	%r13
14142d20e: 41 5e                       	popq	%r14
14142d210: 41 5f                       	popq	%r15
14142d212: 5d                          	popq	%rbp
14142d213: c3                          	retq
14142d214: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
14142d218: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
14142d21c: 48 c7 45 e8 05 00 00 00     	movq	$0x5, -0x18(%rbp)
14142d224: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
14142d228: e8 d3 c5 49 01              	callq	0x1428c9800 <.text+0x28c8800>
14142d22d: eb d4                       	jmp	0x14142d203 <.text+0x142c203>
14142d22f: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
14142d233: 48 83 c4 58                 	addq	$0x58, %rsp
14142d237: 5b                          	popq	%rbx
14142d238: 5f                          	popq	%rdi
14142d239: 5e                          	popq	%rsi
14142d23a: 41 5c                       	popq	%r12
14142d23c: 41 5d                       	popq	%r13
14142d23e: 41 5e                       	popq	%r14
14142d240: 41 5f                       	popq	%r15
14142d242: 5d                          	popq	%rbp
14142d243: e9 58 c6 49 01              	jmp	0x1428c98a0 <.text+0x28c88a0>
14142d248: 83 f8 5b                    	cmpl	$0x5b, %eax
14142d24b: 0f 85 84 00 00 00           	jne	0x14142d2d5 <.text+0x142c2d5>
14142d251: fe 4e 68                    	decb	0x68(%rsi)
14142d254: 0f 84 1f 01 00 00           	je	0x14142d379 <.text+0x142c379>
14142d25a: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
14142d25e: 4c 89 75 d0                 	movq	%r14, -0x30(%rbp)
14142d262: c6 45 e8 0a                 	movb	$0xa, -0x18(%rbp)
14142d266: 4c 8d 05 e3 3e 81 01        	leaq	0x1813ee3(%rip), %r8    # 0x142c41150
14142d26d: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
14142d271: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
14142d275: e8 86 c7 49 01              	callq	0x1428c9a00 <.text+0x28c8a00>
14142d27a: fe 46 68                    	incb	0x68(%rsi)
14142d27d: 48 89 c7                    	movq	%rax, %rdi
14142d280: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
14142d284: 48 89 f1                    	movq	%rsi, %rcx
14142d287: e8 74 78 fd ff              	callq	0x141404b00 <.text+0x1403b00>
14142d28c: 48 85 c0                    	testq	%rax, %rax
14142d28f: 0f 84 bf 00 00 00           	je	0x14142d354 <.text+0x142c354>
14142d295: 48 8b 08                    	movq	(%rax), %rcx
14142d298: 48 83 f9 01                 	cmpq	$0x1, %rcx
14142d29c: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
14142d2a0: 0f 84 8e 00 00 00           	je	0x14142d334 <.text+0x142c334>
14142d2a6: 48 85 c9                    	testq	%rcx, %rcx
14142d2a9: 0f 85 91 00 00 00           	jne	0x14142d340 <.text+0x142c340>
14142d2af: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
14142d2b3: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14142d2b7: 48 85 d2                    	testq	%rdx, %rdx
14142d2ba: 0f 84 80 00 00 00           	je	0x14142d340 <.text+0x142c340>
14142d2c0: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
14142d2c4: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14142d2c8: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14142d2ce: e8 4d 69 1e ff              	callq	0x140613c20 <.text+0x612c20>
14142d2d3: eb 6b                       	jmp	0x14142d340 <.text+0x142c340>
14142d2d5: 4c 8d 05 54 f3 82 01        	leaq	0x182f354(%rip), %r8    # 0x142c5c630
14142d2dc: 48 8d 55 78                 	leaq	0x78(%rbp), %rdx
14142d2e0: 48 89 f1                    	movq	%rsi, %rcx
14142d2e3: e8 18 bc 4c 01              	callq	0x1428f8f00 <.text+0x28f7f00>
14142d2e8: 48 89 c1                    	movq	%rax, %rcx
14142d2eb: eb 6a                       	jmp	0x14142d357 <.text+0x142c357>
14142d2ed: 0f b6 c0                    	movzbl	%al, %eax
14142d2f0: 83 f8 7d                    	cmpl	$0x7d, %eax
14142d2f3: 75 2a                       	jne	0x14142d31f <.text+0x142c31f>
14142d2f5: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
14142d2f9: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
14142d2fd: 48 c7 45 e8 12 00 00 00     	movq	$0x12, -0x18(%rbp)
14142d305: e9 5d fe ff ff              	jmp	0x14142d167 <.text+0x142c167>
14142d30a: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
14142d30e: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
14142d312: 48 c7 45 e8 03 00 00 00     	movq	$0x3, -0x18(%rbp)
14142d31a: e9 48 fe ff ff              	jmp	0x14142d167 <.text+0x142c167>
14142d31f: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
14142d323: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
14142d327: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
14142d32f: e9 33 fe ff ff              	jmp	0x14142d167 <.text+0x142c167>
14142d334: 48 89 c1                    	movq	%rax, %rcx
14142d337: 48 83 c1 08                 	addq	$0x8, %rcx
14142d33b: e8 c0 a6 1b ff              	callq	0x1405e7a00 <.text+0x5e6a00>
14142d340: ba 28 00 00 00              	movl	$0x28, %edx
14142d345: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14142d34b: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
14142d34f: e8 cc 68 1e ff              	callq	0x140613c20 <.text+0x612c20>
14142d354: 48 89 f9                    	movq	%rdi, %rcx
14142d357: 48 89 f2                    	movq	%rsi, %rdx
14142d35a: e8 41 c7 4c 01              	callq	0x1428f9aa0 <.text+0x28f8aa0>
14142d35f: e9 9f fe ff ff              	jmp	0x14142d203 <.text+0x142c203>
14142d364: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
14142d368: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
14142d36c: 48 c7 45 e8 08 00 00 00     	movq	$0x8, -0x18(%rbp)
14142d374: e9 ee fd ff ff              	jmp	0x14142d167 <.text+0x142c167>
14142d379: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
14142d37d: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
14142d381: 48 c7 45 e8 15 00 00 00     	movq	$0x15, -0x18(%rbp)
14142d389: e9 96 fe ff ff              	jmp	0x14142d224 <.text+0x142c224>
