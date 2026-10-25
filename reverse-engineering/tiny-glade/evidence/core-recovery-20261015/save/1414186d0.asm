
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414186d0: 55                          	pushq	%rbp
1414186d1: 41 57                       	pushq	%r15
1414186d3: 41 56                       	pushq	%r14
1414186d5: 41 55                       	pushq	%r13
1414186d7: 41 54                       	pushq	%r12
1414186d9: 56                          	pushq	%rsi
1414186da: 57                          	pushq	%rdi
1414186db: 53                          	pushq	%rbx
1414186dc: 48 83 ec 58                 	subq	$0x58, %rsp
1414186e0: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
1414186e5: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
1414186ed: 48 89 ce                    	movq	%rcx, %rsi
1414186f0: 4c 8b 75 78                 	movq	0x78(%rbp), %r14
1414186f4: 48 8d 79 18                 	leaq	0x18(%rcx), %rdi
1414186f8: f6 41 60 01                 	testb	$0x1, 0x60(%rcx)
1414186fc: 74 06                       	je	0x141418704 <.text+0x1417704>
1414186fe: 0f b6 46 61                 	movzbl	0x61(%rsi), %eax
141418702: eb 6a                       	jmp	0x14141876e <.text+0x141776e>
141418704: 48 8b 4e 28                 	movq	0x28(%rsi), %rcx
141418708: 48 39 4e 30                 	cmpq	%rcx, 0x30(%rsi)
14141870c: 75 31                       	jne	0x14141873f <.text+0x141773f>
14141870e: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141418712: 48 89 fa                    	movq	%rdi, %rdx
141418715: e8 26 85 68 ff              	callq	0x140aa0c40 <.text+0xa9fc40>
14141871a: 0f b6 45 e8                 	movzbl	-0x18(%rbp), %eax
14141871e: 3c 02                       	cmpb	$0x2, %al
141418720: 0f 84 c5 03 00 00           	je	0x141418aeb <.text+0x1417aeb>
141418726: a8 01                       	testb	$0x1, %al
141418728: 0f 85 d8 03 00 00           	jne	0x141418b06 <.text+0x1417b06>
14141872e: 0f b6 45 e9                 	movzbl	-0x17(%rbp), %eax
141418732: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
141418736: 48 ff c1                    	incq	%rcx
141418739: 3c 0a                       	cmpb	$0xa, %al
14141873b: 74 1c                       	je	0x141418759 <.text+0x1417759>
14141873d: eb 24                       	jmp	0x141418763 <.text+0x1417763>
14141873f: 48 8b 46 18                 	movq	0x18(%rsi), %rax
141418743: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
141418747: 48 ff c1                    	incq	%rcx
14141874a: 48 89 4e 28                 	movq	%rcx, 0x28(%rsi)
14141874e: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
141418752: 48 ff c1                    	incq	%rcx
141418755: 3c 0a                       	cmpb	$0xa, %al
141418757: 75 0a                       	jne	0x141418763 <.text+0x1417763>
141418759: 48 01 4e 58                 	addq	%rcx, 0x58(%rsi)
14141875d: 48 ff 46 48                 	incq	0x48(%rsi)
141418761: 31 c9                       	xorl	%ecx, %ecx
141418763: 48 89 4e 50                 	movq	%rcx, 0x50(%rsi)
141418767: c6 46 60 01                 	movb	$0x1, 0x60(%rsi)
14141876b: 88 46 61                    	movb	%al, 0x61(%rsi)
14141876e: 0f b6 c0                    	movzbl	%al, %eax
141418771: 3c 20                       	cmpb	$0x20, %al
141418773: 77 19                       	ja	0x14141878e <.text+0x141778e>
141418775: 48 b9 00 26 00 00 01 00 00 00       	movabsq	$0x100002600, %rcx # imm = 0x100002600
14141877f: 48 0f a3 c1                 	btq	%rax, %rcx
141418783: 73 09                       	jae	0x14141878e <.text+0x141778e>
141418785: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
141418789: e9 76 ff ff ff              	jmp	0x141418704 <.text+0x1417704>
14141878e: 83 f8 7b                    	cmpl	$0x7b, %eax
141418791: 0f 85 88 03 00 00           	jne	0x141418b1f <.text+0x1417b1f>
141418797: fe 4e 68                    	decb	0x68(%rsi)
14141879a: 0f 84 b0 04 00 00           	je	0x141418c50 <.text+0x1417c50>
1414187a0: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
1414187a4: 41 b4 01                    	movb	$0x1, %r12b
1414187a7: 48 8d 5d e8                 	leaq	-0x18(%rbp), %rbx
1414187ab: 49 bf 00 26 00 00 01 00 00 00       	movabsq	$0x100002600, %r15 # imm = 0x100002600
1414187b5: 49 bd 00 22 00 00 01 00 00 00       	movabsq	$0x100002200, %r13 # imm = 0x100002200
1414187bf: eb 20                       	jmp	0x1414187e1 <.text+0x14177e1>
1414187c1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1414187d0: 48 89 f1                    	movq	%rsi, %rcx
1414187d3: e8 f8 0e ff ff              	callq	0x1414096d0 <.text+0x14086d0>
1414187d8: 48 85 c0                    	testq	%rax, %rax
1414187db: 0f 85 66 02 00 00           	jne	0x141418a47 <.text+0x1417a47>
1414187e1: f6 46 60 01                 	testb	$0x1, 0x60(%rsi)
1414187e5: 74 23                       	je	0x14141880a <.text+0x141780a>
1414187e7: 0f b6 46 61                 	movzbl	0x61(%rsi), %eax
1414187eb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1414187f0: 0f b6 c8                    	movzbl	%al, %ecx
1414187f3: 80 f9 2c                    	cmpb	$0x2c, %cl
1414187f6: 0f 87 54 01 00 00           	ja	0x141418950 <.text+0x1417950>
1414187fc: 49 0f a3 cf                 	btq	%rcx, %r15
141418800: 0f 83 8a 00 00 00           	jae	0x141418890 <.text+0x1417890>
141418806: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
14141880a: 48 8b 4e 28                 	movq	0x28(%rsi), %rcx
14141880e: 48 39 4e 30                 	cmpq	%rcx, 0x30(%rsi)
141418812: 75 3c                       	jne	0x141418850 <.text+0x1417850>
141418814: 48 89 d9                    	movq	%rbx, %rcx
141418817: 48 89 fa                    	movq	%rdi, %rdx
14141881a: e8 21 84 68 ff              	callq	0x140aa0c40 <.text+0xa9fc40>
14141881f: 0f b6 45 e8                 	movzbl	-0x18(%rbp), %eax
141418823: 3c 02                       	cmpb	$0x2, %al
141418825: 0f 84 b6 03 00 00           	je	0x141418be1 <.text+0x1417be1>
14141882b: a8 01                       	testb	$0x1, %al
14141882d: 0f 85 f0 01 00 00           	jne	0x141418a23 <.text+0x1417a23>
141418833: 0f b6 45 e9                 	movzbl	-0x17(%rbp), %eax
141418837: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
14141883b: 48 ff c1                    	incq	%rcx
14141883e: 3c 0a                       	cmpb	$0xa, %al
141418840: 74 28                       	je	0x14141886a <.text+0x141786a>
141418842: eb 30                       	jmp	0x141418874 <.text+0x1417874>
141418844: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141418850: 48 8b 46 18                 	movq	0x18(%rsi), %rax
141418854: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
141418858: 48 ff c1                    	incq	%rcx
14141885b: 48 89 4e 28                 	movq	%rcx, 0x28(%rsi)
14141885f: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
141418863: 48 ff c1                    	incq	%rcx
141418866: 3c 0a                       	cmpb	$0xa, %al
141418868: 75 0a                       	jne	0x141418874 <.text+0x1417874>
14141886a: 48 01 4e 58                 	addq	%rcx, 0x58(%rsi)
14141886e: 48 ff 46 48                 	incq	0x48(%rsi)
141418872: 31 c9                       	xorl	%ecx, %ecx
141418874: 48 89 4e 50                 	movq	%rcx, 0x50(%rsi)
141418878: c6 46 60 01                 	movb	$0x1, 0x60(%rsi)
14141887c: 88 46 61                    	movb	%al, 0x61(%rsi)
14141887f: e9 6c ff ff ff              	jmp	0x1414187f0 <.text+0x14177f0>
141418884: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141418890: 48 83 f9 2c                 	cmpq	$0x2c, %rcx
141418894: 0f 85 b6 00 00 00           	jne	0x141418950 <.text+0x1417950>
14141889a: 41 f6 c4 01                 	testb	$0x1, %r12b
14141889e: 74 26                       	je	0x1414188c6 <.text+0x14178c6>
1414188a0: e9 51 03 00 00              	jmp	0x141418bf6 <.text+0x1417bf6>
1414188a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1414188b0: 48 01 4e 58                 	addq	%rcx, 0x58(%rsi)
1414188b4: 48 ff 46 48                 	incq	0x48(%rsi)
1414188b8: 48 c7 46 50 00 00 00 00     	movq	$0x0, 0x50(%rsi)
1414188c0: 66 c7 46 60 01 0a           	movw	$0xa01, 0x60(%rsi)      # imm = 0xA01
1414188c6: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
1414188ca: 48 8b 4e 28                 	movq	0x28(%rsi), %rcx
1414188ce: 48 39 4e 30                 	cmpq	%rcx, 0x30(%rsi)
1414188d2: 75 3c                       	jne	0x141418910 <.text+0x1417910>
1414188d4: 48 89 d9                    	movq	%rbx, %rcx
1414188d7: 48 89 fa                    	movq	%rdi, %rdx
1414188da: e8 61 83 68 ff              	callq	0x140aa0c40 <.text+0xa9fc40>
1414188df: 0f b6 45 e8                 	movzbl	-0x18(%rbp), %eax
1414188e3: 3c 02                       	cmpb	$0x2, %al
1414188e5: 0f 84 43 01 00 00           	je	0x141418a2e <.text+0x1417a2e>
1414188eb: a8 01                       	testb	$0x1, %al
1414188ed: 0f 85 30 01 00 00           	jne	0x141418a23 <.text+0x1417a23>
1414188f3: 0f b6 45 e9                 	movzbl	-0x17(%rbp), %eax
1414188f7: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
1414188fb: 48 ff c1                    	incq	%rcx
1414188fe: 3c 0a                       	cmpb	$0xa, %al
141418900: 74 ae                       	je	0x1414188b0 <.text+0x14178b0>
141418902: eb 26                       	jmp	0x14141892a <.text+0x141792a>
141418904: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141418910: 48 8b 46 18                 	movq	0x18(%rsi), %rax
141418914: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
141418918: 48 ff c1                    	incq	%rcx
14141891b: 48 89 4e 28                 	movq	%rcx, 0x28(%rsi)
14141891f: 48 8b 4e 50                 	movq	0x50(%rsi), %rcx
141418923: 48 ff c1                    	incq	%rcx
141418926: 3c 0a                       	cmpb	$0xa, %al
141418928: 74 86                       	je	0x1414188b0 <.text+0x14178b0>
14141892a: 48 89 4e 50                 	movq	%rcx, 0x50(%rsi)
14141892e: c6 46 60 01                 	movb	$0x1, 0x60(%rsi)
141418932: 88 46 61                    	movb	%al, 0x61(%rsi)
141418935: 3c 20                       	cmpb	$0x20, %al
141418937: 77 2d                       	ja	0x141418966 <.text+0x1417966>
141418939: 0f b6 c8                    	movzbl	%al, %ecx
14141893c: 49 0f a3 cd                 	btq	%rcx, %r13
141418940: 72 84                       	jb	0x1414188c6 <.text+0x14178c6>
141418942: eb 22                       	jmp	0x141418966 <.text+0x1417966>
141418944: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
141418950: 83 f9 7d                    	cmpl	$0x7d, %ecx
141418953: 0f 84 c6 00 00 00           	je	0x141418a1f <.text+0x1417a1f>
141418959: 41 f6 c4 01                 	testb	$0x1, %r12b
14141895d: 0f 84 d8 02 00 00           	je	0x141418c3b <.text+0x1417c3b>
141418963: 45 31 e4                    	xorl	%r12d, %r12d
141418966: 3c 22                       	cmpb	$0x22, %al
141418968: 0f 85 56 02 00 00           	jne	0x141418bc4 <.text+0x1417bc4>
14141896e: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
141418972: 48 c7 46 10 00 00 00 00     	movq	$0x0, 0x10(%rsi)
14141897a: 48 89 d9                    	movq	%rbx, %rcx
14141897d: 48 89 fa                    	movq	%rdi, %rdx
141418980: 49 89 f0                    	movq	%rsi, %r8
141418983: e8 08 1a 73 ff              	callq	0x140b4a390 <.text+0xb49390>
141418988: 83 7d e8 02                 	cmpl	$0x2, -0x18(%rbp)
14141898c: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
141418990: 0f 84 b1 00 00 00           	je	0x141418a47 <.text+0x1417a47>
141418996: 48 83 7d f8 09              	cmpq	$0x9, -0x8(%rbp)
14141899b: 0f 85 2f fe ff ff           	jne	0x1414187d0 <.text+0x14177d0>
1414189a1: 48 8b 15 20 87 82 01        	movq	0x1828720(%rip), %rdx   # 0x142c410c8
1414189a8: 48 33 10                    	xorq	(%rax), %rdx
1414189ab: 0f b6 0d 1e 87 82 01        	movzbl	0x182871e(%rip), %ecx   # 0x142c410d0
1414189b2: 32 48 08                    	xorb	0x8(%rax), %cl
1414189b5: 0f b6 c9                    	movzbl	%cl, %ecx
1414189b8: 48 09 d1                    	orq	%rdx, %rcx
1414189bb: 0f 85 0f fe ff ff           	jne	0x1414187d0 <.text+0x14177d0>
1414189c1: 48 89 f1                    	movq	%rsi, %rcx
1414189c4: e8 27 bf fe ff              	callq	0x1414048f0 <.text+0x14038f0>
1414189c9: 48 85 c0                    	testq	%rax, %rax
1414189cc: 75 79                       	jne	0x141418a47 <.text+0x1417a47>
1414189ce: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
1414189d2: 48 89 f2                    	movq	%rsi, %rdx
1414189d5: e8 76 7a ff ff              	callq	0x141410450 <.text+0x140f450>
1414189da: 48 8b 7d e8                 	movq	-0x18(%rbp), %rdi
1414189de: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
1414189e2: 48 89 f9                    	movq	%rdi, %rcx
1414189e5: 48 f7 d9                    	negq	%rcx
1414189e8: 70 5d                       	jo	0x141418a47 <.text+0x1417a47>
1414189ea: 48 8b 5d f8                 	movq	-0x8(%rbp), %rbx
1414189ee: 49 8b 16                    	movq	(%r14), %rdx
1414189f1: 48 89 d1                    	movq	%rdx, %rcx
1414189f4: 48 f7 d9                    	negq	%rcx
1414189f7: 70 1b                       	jo	0x141418a14 <.text+0x1417a14>
1414189f9: 73 19                       	jae	0x141418a14 <.text+0x1417a14>
1414189fb: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
1414189ff: 48 c1 e2 04                 	shlq	$0x4, %rdx
141418a03: 49 89 c7                    	movq	%rax, %r15
141418a06: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141418a0c: e8 0f b2 1f ff              	callq	0x140613c20 <.text+0x612c20>
141418a11: 4c 89 f8                    	movq	%r15, %rax
141418a14: 49 89 3e                    	movq	%rdi, (%r14)
141418a17: 49 89 46 08                 	movq	%rax, 0x8(%r14)
141418a1b: 49 89 5e 10                 	movq	%rbx, 0x10(%r14)
141418a1f: 31 c0                       	xorl	%eax, %eax
141418a21: eb 24                       	jmp	0x141418a47 <.text+0x1417a47>
141418a23: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
141418a27: e8 74 0e 4b 01              	callq	0x1428c98a0 <.text+0x28c88a0>
141418a2c: eb 19                       	jmp	0x141418a47 <.text+0x1417a47>
141418a2e: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
141418a32: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
141418a36: 48 c7 45 e8 05 00 00 00     	movq	$0x5, -0x18(%rbp)
141418a3e: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141418a42: e8 b9 0d 4b 01              	callq	0x1428c9800 <.text+0x28c8800>
141418a47: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
141418a4b: fe 46 68                    	incb	0x68(%rsi)
141418a4e: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
141418a52: 48 89 f1                    	movq	%rsi, %rcx
141418a55: e8 96 bf fe ff              	callq	0x1414049f0 <.text+0x14039f0>
141418a5a: 4c 8b 45 e0                 	movq	-0x20(%rbp), %r8
141418a5e: 4d 85 c0                    	testq	%r8, %r8
141418a61: 4c 0f 44 c0                 	cmoveq	%rax, %r8
141418a65: 0f 94 c2                    	sete	%dl
141418a68: 48 85 c0                    	testq	%rax, %rax
141418a6b: 0f 94 c1                    	sete	%cl
141418a6e: 08 d1                       	orb	%dl, %cl
141418a70: 4c 89 c1                    	movq	%r8, %rcx
141418a73: 75 5a                       	jne	0x141418acf <.text+0x1417acf>
141418a75: 48 89 cf                    	movq	%rcx, %rdi
141418a78: 48 8b 08                    	movq	(%rax), %rcx
141418a7b: 48 83 f9 01                 	cmpq	$0x1, %rcx
141418a7f: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
141418a83: 74 27                       	je	0x141418aac <.text+0x1417aac>
141418a85: 48 85 c9                    	testq	%rcx, %rcx
141418a88: 75 2e                       	jne	0x141418ab8 <.text+0x1417ab8>
141418a8a: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
141418a8e: 48 8b 50 10                 	movq	0x10(%rax), %rdx
141418a92: 48 85 d2                    	testq	%rdx, %rdx
141418a95: 74 21                       	je	0x141418ab8 <.text+0x1417ab8>
141418a97: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
141418a9b: 48 8b 48 08                 	movq	0x8(%rax), %rcx
141418a9f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141418aa5: e8 76 b1 1f ff              	callq	0x140613c20 <.text+0x612c20>
141418aaa: eb 0c                       	jmp	0x141418ab8 <.text+0x1417ab8>
141418aac: 48 89 c1                    	movq	%rax, %rcx
141418aaf: 48 83 c1 08                 	addq	$0x8, %rcx
141418ab3: e8 48 ef 1c ff              	callq	0x1405e7a00 <.text+0x5e6a00>
141418ab8: ba 28 00 00 00              	movl	$0x28, %edx
141418abd: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141418ac3: 48 8b 4d d8                 	movq	-0x28(%rbp), %rcx
141418ac7: e8 54 b1 1f ff              	callq	0x140613c20 <.text+0x612c20>
141418acc: 48 89 f9                    	movq	%rdi, %rcx
141418acf: 48 85 c9                    	testq	%rcx, %rcx
141418ad2: 0f 85 56 01 00 00           	jne	0x141418c2e <.text+0x1417c2e>
141418ad8: 31 c0                       	xorl	%eax, %eax
141418ada: 48 83 c4 58                 	addq	$0x58, %rsp
141418ade: 5b                          	popq	%rbx
141418adf: 5f                          	popq	%rdi
141418ae0: 5e                          	popq	%rsi
141418ae1: 41 5c                       	popq	%r12
141418ae3: 41 5d                       	popq	%r13
141418ae5: 41 5e                       	popq	%r14
141418ae7: 41 5f                       	popq	%r15
141418ae9: 5d                          	popq	%rbp
141418aea: c3                          	retq
141418aeb: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
141418aef: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
141418af3: 48 c7 45 e8 05 00 00 00     	movq	$0x5, -0x18(%rbp)
141418afb: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141418aff: e8 fc 0c 4b 01              	callq	0x1428c9800 <.text+0x28c8800>
141418b04: eb d4                       	jmp	0x141418ada <.text+0x1417ada>
141418b06: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
141418b0a: 48 83 c4 58                 	addq	$0x58, %rsp
141418b0e: 5b                          	popq	%rbx
141418b0f: 5f                          	popq	%rdi
141418b10: 5e                          	popq	%rsi
141418b11: 41 5c                       	popq	%r12
141418b13: 41 5d                       	popq	%r13
141418b15: 41 5e                       	popq	%r14
141418b17: 41 5f                       	popq	%r15
141418b19: 5d                          	popq	%rbp
141418b1a: e9 81 0d 4b 01              	jmp	0x1428c98a0 <.text+0x28c88a0>
141418b1f: 83 f8 5b                    	cmpl	$0x5b, %eax
141418b22: 0f 85 84 00 00 00           	jne	0x141418bac <.text+0x1417bac>
141418b28: fe 4e 68                    	decb	0x68(%rsi)
141418b2b: 0f 84 1f 01 00 00           	je	0x141418c50 <.text+0x1417c50>
141418b31: c6 46 60 00                 	movb	$0x0, 0x60(%rsi)
141418b35: 4c 89 75 d0                 	movq	%r14, -0x30(%rbp)
141418b39: c6 45 e8 0a                 	movb	$0xa, -0x18(%rbp)
141418b3d: 4c 8d 05 2c 86 82 01        	leaq	0x182862c(%rip), %r8    # 0x142c41170
141418b44: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141418b48: 48 8d 55 d0                 	leaq	-0x30(%rbp), %rdx
141418b4c: e8 af 0e 4b 01              	callq	0x1428c9a00 <.text+0x28c8a00>
141418b51: fe 46 68                    	incb	0x68(%rsi)
141418b54: 48 89 c7                    	movq	%rax, %rdi
141418b57: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
141418b5b: 48 89 f1                    	movq	%rsi, %rcx
141418b5e: e8 9d bf fe ff              	callq	0x141404b00 <.text+0x1403b00>
141418b63: 48 85 c0                    	testq	%rax, %rax
141418b66: 0f 84 bf 00 00 00           	je	0x141418c2b <.text+0x1417c2b>
141418b6c: 48 8b 08                    	movq	(%rax), %rcx
141418b6f: 48 83 f9 01                 	cmpq	$0x1, %rcx
141418b73: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
141418b77: 0f 84 8e 00 00 00           	je	0x141418c0b <.text+0x1417c0b>
141418b7d: 48 85 c9                    	testq	%rcx, %rcx
141418b80: 0f 85 91 00 00 00           	jne	0x141418c17 <.text+0x1417c17>
141418b86: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
141418b8a: 48 8b 50 10                 	movq	0x10(%rax), %rdx
141418b8e: 48 85 d2                    	testq	%rdx, %rdx
141418b91: 0f 84 80 00 00 00           	je	0x141418c17 <.text+0x1417c17>
141418b97: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
141418b9b: 48 8b 48 08                 	movq	0x8(%rax), %rcx
141418b9f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141418ba5: e8 76 b0 1f ff              	callq	0x140613c20 <.text+0x612c20>
141418baa: eb 6b                       	jmp	0x141418c17 <.text+0x1417c17>
141418bac: 4c 8d 05 5d 38 84 01        	leaq	0x184385d(%rip), %r8    # 0x142c5c410
141418bb3: 48 8d 55 78                 	leaq	0x78(%rbp), %rdx
141418bb7: 48 89 f1                    	movq	%rsi, %rcx
141418bba: e8 41 03 4e 01              	callq	0x1428f8f00 <.text+0x28f7f00>
141418bbf: 48 89 c1                    	movq	%rax, %rcx
141418bc2: eb 6a                       	jmp	0x141418c2e <.text+0x1417c2e>
141418bc4: 0f b6 c0                    	movzbl	%al, %eax
141418bc7: 83 f8 7d                    	cmpl	$0x7d, %eax
141418bca: 75 2a                       	jne	0x141418bf6 <.text+0x1417bf6>
141418bcc: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
141418bd0: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
141418bd4: 48 c7 45 e8 12 00 00 00     	movq	$0x12, -0x18(%rbp)
141418bdc: e9 5d fe ff ff              	jmp	0x141418a3e <.text+0x1417a3e>
141418be1: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
141418be5: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
141418be9: 48 c7 45 e8 03 00 00 00     	movq	$0x3, -0x18(%rbp)
141418bf1: e9 48 fe ff ff              	jmp	0x141418a3e <.text+0x1417a3e>
141418bf6: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
141418bfa: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
141418bfe: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
141418c06: e9 33 fe ff ff              	jmp	0x141418a3e <.text+0x1417a3e>
141418c0b: 48 89 c1                    	movq	%rax, %rcx
141418c0e: 48 83 c1 08                 	addq	$0x8, %rcx
141418c12: e8 e9 ed 1c ff              	callq	0x1405e7a00 <.text+0x5e6a00>
141418c17: ba 28 00 00 00              	movl	$0x28, %edx
141418c1c: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141418c22: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
141418c26: e8 f5 af 1f ff              	callq	0x140613c20 <.text+0x612c20>
141418c2b: 48 89 f9                    	movq	%rdi, %rcx
141418c2e: 48 89 f2                    	movq	%rsi, %rdx
141418c31: e8 6a 0e 4e 01              	callq	0x1428f9aa0 <.text+0x28f8aa0>
141418c36: e9 9f fe ff ff              	jmp	0x141418ada <.text+0x1417ada>
141418c3b: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
141418c3f: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
141418c43: 48 c7 45 e8 08 00 00 00     	movq	$0x8, -0x18(%rbp)
141418c4b: e9 ee fd ff ff              	jmp	0x141418a3e <.text+0x1417a3e>
141418c50: 48 8b 56 48                 	movq	0x48(%rsi), %rdx
141418c54: 4c 8b 46 50                 	movq	0x50(%rsi), %r8
141418c58: 48 c7 45 e8 15 00 00 00     	movq	$0x15, -0x18(%rbp)
141418c60: e9 96 fe ff ff              	jmp	0x141418afb <.text+0x1417afb>
141418c65: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141418c70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141418c75: 55                          	pushq	%rbp
141418c76: 41 57                       	pushq	%r15
141418c78: 41 56                       	pushq	%r14
141418c7a: 41 55                       	pushq	%r13
141418c7c: 41 54                       	pushq	%r12
141418c7e: 56                          	pushq	%rsi
141418c7f: 57                          	pushq	%rdi
141418c80: 53                          	pushq	%rbx
141418c81: 48 83 ec 28                 	subq	$0x28, %rsp
141418c85: 48 8d 6a 50                 	leaq	0x50(%rdx), %rbp
141418c89: ba 28 00 00 00              	movl	$0x28, %edx
141418c8e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141418c94: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
141418c98: e8 83 af 1f ff              	callq	0x140613c20 <.text+0x612c20>
141418c9d: 90                          	nop
141418c9e: 48 83 c4 28                 	addq	$0x28, %rsp
141418ca2: 5b                          	popq	%rbx
141418ca3: 5f                          	popq	%rdi
141418ca4: 5e                          	popq	%rsi
141418ca5: 41 5c                       	popq	%r12
141418ca7: 41 5d                       	popq	%r13
141418ca9: 41 5e                       	popq	%r14
141418cab: 41 5f                       	popq	%r15
141418cad: 5d                          	popq	%rbp
141418cae: c3                          	retq
141418caf: 90                          	nop
141418cb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141418cb5: 55                          	pushq	%rbp
141418cb6: 41 57                       	pushq	%r15
141418cb8: 41 56                       	pushq	%r14
141418cba: 41 55                       	pushq	%r13
141418cbc: 41 54                       	pushq	%r12
141418cbe: 56                          	pushq	%rsi
141418cbf: 57                          	pushq	%rdi
141418cc0: 53                          	pushq	%rbx
141418cc1: 48 83 ec 28                 	subq	$0x28, %rsp
141418cc5: 48 8d 6a 50                 	leaq	0x50(%rdx), %rbp
141418cc9: 48 83 7d e0 00              	cmpq	$0x0, -0x20(%rbp)
141418cce: 74 09                       	je	0x141418cd9 <.text+0x1417cd9>
141418cd0: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
141418cd4: e8 c7 fb 1c ff              	callq	0x1405e88a0 <.text+0x5e78a0>
141418cd9: 90                          	nop
141418cda: 48 83 c4 28                 	addq	$0x28, %rsp
141418cde: 5b                          	popq	%rbx
141418cdf: 5f                          	popq	%rdi
141418ce0: 5e                          	popq	%rsi
141418ce1: 41 5c                       	popq	%r12
141418ce3: 41 5d                       	popq	%r13
141418ce5: 41 5e                       	popq	%r14
141418ce7: 41 5f                       	popq	%r15
141418ce9: 5d                          	popq	%rbp
141418cea: c3                          	retq
141418ceb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
