
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14089dd50: 55                          	pushq	%rbp
14089dd51: 41 57                       	pushq	%r15
14089dd53: 41 56                       	pushq	%r14
14089dd55: 41 55                       	pushq	%r13
14089dd57: 41 54                       	pushq	%r12
14089dd59: 56                          	pushq	%rsi
14089dd5a: 57                          	pushq	%rdi
14089dd5b: 53                          	pushq	%rbx
14089dd5c: 48 81 ec 58 02 00 00        	subq	$0x258, %rsp            # imm = 0x258
14089dd63: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14089dd6b: 44 0f 29 8d c0 01 00 00     	movaps	%xmm9, 0x1c0(%rbp)
14089dd73: 44 0f 29 85 b0 01 00 00     	movaps	%xmm8, 0x1b0(%rbp)
14089dd7b: 0f 29 bd a0 01 00 00        	movaps	%xmm7, 0x1a0(%rbp)
14089dd82: 0f 29 b5 90 01 00 00        	movaps	%xmm6, 0x190(%rbp)
14089dd89: 48 c7 85 88 01 00 00 fe ff ff ff    	movq	$-0x2, 0x188(%rbp)
14089dd94: 4d 89 ce                    	movq	%r9, %r14
14089dd97: 4c 89 c3                    	movq	%r8, %rbx
14089dd9a: 4c 8b 4a 10                 	movq	0x10(%rdx), %r9
14089dd9e: 48 bf 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdi # imm = 0x8000000000000000
14089dda8: 4c 89 c8                    	movq	%r9, %rax
14089ddab: 48 31 f8                    	xorq	%rdi, %rax
14089ddae: 48 83 f8 05                 	cmpq	$0x5, %rax
14089ddb2: 41 b8 03 00 00 00           	movl	$0x3, %r8d
14089ddb8: 4c 0f 42 c0                 	cmovbq	%rax, %r8
14089ddbc: 48 89 8d a8 00 00 00        	movq	%rcx, 0xa8(%rbp)
14089ddc3: 48 8d 05 f6 cd 1f 02        	leaq	0x21fcdf6(%rip), %rax   # 0x142a9abc0
14089ddca: 4a 63 0c 80                 	movslq	(%rax,%r8,4), %rcx
14089ddce: 48 01 c1                    	addq	%rax, %rcx
14089ddd1: 48 89 95 58 01 00 00        	movq	%rdx, 0x158(%rbp)
14089ddd8: 48 89 9d b0 00 00 00        	movq	%rbx, 0xb0(%rbp)
14089dddf: ff e1                       	jmpq	*%rcx
14089dde1: 48 8b 42 18                 	movq	0x18(%rdx), %rax
14089dde5: 48 89 85 78 01 00 00        	movq	%rax, 0x178(%rbp)
14089ddec: 48 8b 42 20                 	movq	0x20(%rdx), %rax
14089ddf0: 4c 8b 42 28                 	movq	0x28(%rdx), %r8
14089ddf4: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14089ddf8: 4c 89 45 28                 	movq	%r8, 0x28(%rbp)
14089ddfc: 48 89 d9                    	movq	%rbx, %rcx
14089ddff: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14089de06: 48 89 c2                    	movq	%rax, %rdx
14089de09: e8 62 f7 08 00              	callq	0x14092d570 <.text+0x92c570>
14089de0e: 48 89 95 60 01 00 00        	movq	%rdx, 0x160(%rbp)
14089de15: a8 01                       	testb	$0x1, %al
14089de17: 0f 84 54 09 00 00           	je	0x14089e771 <.text+0x89d771>
14089de1d: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14089de24: 48 85 d2                    	testq	%rdx, %rdx
14089de27: 74 12                       	je	0x14089de3b <.text+0x89ce3b>
14089de29: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14089de2f: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14089de36: e8 e5 5d d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089de3b: c7 85 78 01 00 00 00 00 00 00       	movl	$0x0, 0x178(%rbp)
14089de45: e9 4c 04 00 00              	jmp	0x14089e296 <.text+0x89d296>
14089de4a: 48 8b 42 18                 	movq	0x18(%rdx), %rax
14089de4e: 4c 8b 42 20                 	movq	0x20(%rdx), %r8
14089de52: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14089de59: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
14089de60: 48 89 c2                    	movq	%rax, %rdx
14089de63: 4c 89 85 70 01 00 00        	movq	%r8, 0x170(%rbp)
14089de6a: 41 ff 50 28                 	callq	*0x28(%r8)
14089de6e: 4c 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %r9
14089de75: 48 8b 85 d0 00 00 00        	movq	0xd0(%rbp), %rax
14089de7c: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14089de80: 49 8d 04 81                 	leaq	(%r9,%rax,4), %rax
14089de84: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14089de89: 48 8d 15 58 b7 1f 02        	leaq	0x21fb758(%rip), %rdx   # 0x142a995e8
14089de90: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
14089de94: 41 b8 13 00 00 00           	movl	$0x13, %r8d
14089de9a: 4c 89 8d 60 01 00 00        	movq	%r9, 0x160(%rbp)
14089dea1: e8 7a e2 1c 00              	callq	0x140a6c120 <.text+0xa6b120>
14089dea6: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
14089dead: 48 85 c0                    	testq	%rax, %rax
14089deb0: 74 1a                       	je	0x14089decc <.text+0x89cecc>
14089deb2: 48 c1 e0 02                 	shlq	$0x2, %rax
14089deb6: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14089deba: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14089dec0: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
14089dec7: e8 54 5d d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089decc: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14089ded3: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14089deda: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
14089dee1: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
14089dee8: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
14089deef: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
14089def6: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14089defa: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
14089df01: 0f 10 45 30                 	movups	0x30(%rbp), %xmm0
14089df05: 0f 10 4d 40                 	movups	0x40(%rbp), %xmm1
14089df09: 0f 10 55 50                 	movups	0x50(%rbp), %xmm2
14089df0d: 0f 10 5d 60                 	movups	0x60(%rbp), %xmm3
14089df11: 0f 29 9d f0 00 00 00        	movaps	%xmm3, 0xf0(%rbp)
14089df18: 0f 29 95 e0 00 00 00        	movaps	%xmm2, 0xe0(%rbp)
14089df1f: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
14089df26: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14089df2d: 48 89 bd 38 01 00 00        	movq	%rdi, 0x138(%rbp)
14089df34: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
14089df3b: 48 89 d9                    	movq	%rbx, %rcx
14089df3e: e8 1d ed 08 00              	callq	0x14092cc60 <.text+0x92bc60>
14089df43: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14089df4a: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14089df51: 48 8b 00                    	movq	(%rax), %rax
14089df54: 48 85 c0                    	testq	%rax, %rax
14089df57: 74 09                       	je	0x14089df62 <.text+0x89cf62>
14089df59: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14089df60: ff d0                       	callq	*%rax
14089df62: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14089df69: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
14089df6d: 48 85 d2                    	testq	%rdx, %rdx
14089df70: 0f 84 18 03 00 00           	je	0x14089e28e <.text+0x89d28e>
14089df76: b0 01                       	movb	$0x1, %al
14089df78: 89 85 78 01 00 00           	movl	%eax, 0x178(%rbp)
14089df7e: 4c 8b 41 10                 	movq	0x10(%rcx), %r8
14089df82: e9 d8 00 00 00              	jmp	0x14089e05f <.text+0x89d05f>
14089df87: 48 8b 42 18                 	movq	0x18(%rdx), %rax
14089df8b: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
14089df92: 4c 8b 4a 20                 	movq	0x20(%rdx), %r9
14089df96: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14089df9a: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14089df9e: 49 8d 04 81                 	leaq	(%r9,%rax,4), %rax
14089dfa2: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14089dfa7: 48 8d 15 05 b6 1f 02        	leaq	0x21fb605(%rip), %rdx   # 0x142a995b3
14089dfae: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
14089dfb2: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14089dfb8: 4c 89 8d 68 01 00 00        	movq	%r9, 0x168(%rbp)
14089dfbf: e8 5c e1 1c 00              	callq	0x140a6c120 <.text+0xa6b120>
14089dfc4: 48 8b 45 18                 	movq	0x18(%rbp), %rax
14089dfc8: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14089dfcf: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
14089dfd3: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
14089dfda: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
14089dfde: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
14089dfe5: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
14089dfe9: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
14089dff0: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
14089dff4: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
14089dff8: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
14089dffc: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
14089e000: 0f 29 9d f0 00 00 00        	movaps	%xmm3, 0xf0(%rbp)
14089e007: 0f 29 95 e0 00 00 00        	movaps	%xmm2, 0xe0(%rbp)
14089e00e: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
14089e015: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14089e01c: 48 89 bd 38 01 00 00        	movq	%rdi, 0x138(%rbp)
14089e023: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
14089e02a: 48 89 d9                    	movq	%rbx, %rcx
14089e02d: e8 2e ec 08 00              	callq	0x14092cc60 <.text+0x92bc60>
14089e032: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14089e039: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14089e040: 48 85 c9                    	testq	%rcx, %rcx
14089e043: 0f 84 45 02 00 00           	je	0x14089e28e <.text+0x89d28e>
14089e049: b0 01                       	movb	$0x1, %al
14089e04b: 89 85 78 01 00 00           	movl	%eax, 0x178(%rbp)
14089e051: 48 c1 e1 02                 	shlq	$0x2, %rcx
14089e055: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
14089e059: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14089e05f: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14089e066: e8 b5 5b d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089e06b: e9 26 02 00 00              	jmp	0x14089e296 <.text+0x89d296>
14089e070: 4c 89 8d 68 01 00 00        	movq	%r9, 0x168(%rbp)
14089e077: 4c 8b 4a 18                 	movq	0x18(%rdx), %r9
14089e07b: 48 8b 42 20                 	movq	0x20(%rdx), %rax
14089e07f: 48 8b 4a 28                 	movq	0x28(%rdx), %rcx
14089e083: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
14089e08a: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
14089e08e: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
14089e095: 48 8b 72 38                 	movq	0x38(%rdx), %rsi
14089e099: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14089e09d: 49 8d 04 81                 	leaq	(%r9,%rax,4), %rax
14089e0a1: c6 85 87 01 00 00 00        	movb	$0x0, 0x187(%rbp)
14089e0a8: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14089e0ad: 48 8d 15 0f b5 1f 02        	leaq	0x21fb50f(%rip), %rdx   # 0x142a995c3
14089e0b4: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
14089e0b8: 41 b8 19 00 00 00           	movl	$0x19, %r8d
14089e0be: 4c 89 8d b8 00 00 00        	movq	%r9, 0xb8(%rbp)
14089e0c5: e8 56 e0 1c 00              	callq	0x140a6c120 <.text+0xa6b120>
14089e0ca: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14089e0d1: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14089e0d8: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
14089e0df: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14089e0e6: 48 89 b5 d0 00 00 00        	movq	%rsi, 0xd0(%rbp)
14089e0ed: c6 85 86 01 00 00 01        	movb	$0x1, 0x186(%rbp)
14089e0f4: 48 8d 15 e1 b4 1f 02        	leaq	0x21fb4e1(%rip), %rdx   # 0x142a995dc
14089e0fb: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
14089e0ff: 4c 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %r9
14089e106: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
14089e10c: e8 7f 6b 19 00              	callq	0x140a34c90 <.text+0xa33c90>
14089e111: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14089e118: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
14089e11f: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
14089e126: 0f 29 85 20 01 00 00        	movaps	%xmm0, 0x120(%rbp)
14089e12d: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
14089e134: 0f 29 85 10 01 00 00        	movaps	%xmm0, 0x110(%rbp)
14089e13b: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14089e13f: 0f 29 85 00 01 00 00        	movaps	%xmm0, 0x100(%rbp)
14089e146: 0f 10 45 30                 	movups	0x30(%rbp), %xmm0
14089e14a: 0f 10 4d 40                 	movups	0x40(%rbp), %xmm1
14089e14e: 0f 10 55 50                 	movups	0x50(%rbp), %xmm2
14089e152: 0f 10 5d 60                 	movups	0x60(%rbp), %xmm3
14089e156: 0f 29 9d f0 00 00 00        	movaps	%xmm3, 0xf0(%rbp)
14089e15d: 0f 29 95 e0 00 00 00        	movaps	%xmm2, 0xe0(%rbp)
14089e164: 0f 29 8d d0 00 00 00        	movaps	%xmm1, 0xd0(%rbp)
14089e16b: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
14089e172: 48 89 bd 38 01 00 00        	movq	%rdi, 0x138(%rbp)
14089e179: c6 85 86 01 00 00 00        	movb	$0x0, 0x186(%rbp)
14089e180: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
14089e187: 48 89 d9                    	movq	%rbx, %rcx
14089e18a: e8 d1 ea 08 00              	callq	0x14092cc60 <.text+0x92bc60>
14089e18f: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14089e196: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14089e19d: 48 85 c9                    	testq	%rcx, %rcx
14089e1a0: 0f 84 e8 00 00 00           	je	0x14089e28e <.text+0x89d28e>
14089e1a6: b0 01                       	movb	$0x1, %al
14089e1a8: 89 85 78 01 00 00           	movl	%eax, 0x178(%rbp)
14089e1ae: 48 c1 e1 02                 	shlq	$0x2, %rcx
14089e1b2: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
14089e1b6: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14089e1bc: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
14089e1c3: e8 58 5a d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089e1c8: e9 c9 00 00 00              	jmp	0x14089e296 <.text+0x89d296>
14089e1cd: 31 c9                       	xorl	%ecx, %ecx
14089e1cf: e8 4c b9 83 ff              	callq	0x1400d9b20 <.text+0xd8b20>
14089e1d4: 48 85 c0                    	testq	%rax, %rax
14089e1d7: 0f 84 f6 05 00 00           	je	0x14089e7d3 <.text+0x89d7d3>
14089e1dd: 0f 10 00                    	movups	(%rax), %xmm0
14089e1e0: 48 ff 00                    	incq	(%rax)
14089e1e3: 0f 10 0d d6 82 22 02        	movups	0x22282d6(%rip), %xmm1  # 0x142ac64c0
14089e1ea: 0f 11 4d 48                 	movups	%xmm1, 0x48(%rbp)
14089e1ee: 0f 10 0d db 82 22 02        	movups	0x22282db(%rip), %xmm1  # 0x142ac64d0
14089e1f5: 0f 11 4d 58                 	movups	%xmm1, 0x58(%rbp)
14089e1f9: 48 c7 85 c0 00 00 00 00 00 00 00    	movq	$0x0, 0xc0(%rbp)
14089e204: 48 c7 85 c8 00 00 00 04 00 00 00    	movq	$0x4, 0xc8(%rbp)
14089e20f: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
14089e21a: 48 89 bd d8 00 00 00        	movq	%rdi, 0xd8(%rbp)
14089e221: 0f 10 4d 30                 	movups	0x30(%rbp), %xmm1
14089e225: 0f 11 8d e0 00 00 00        	movups	%xmm1, 0xe0(%rbp)
14089e22c: 0f 10 4d 40                 	movups	0x40(%rbp), %xmm1
14089e230: 0f 11 8d f0 00 00 00        	movups	%xmm1, 0xf0(%rbp)
14089e237: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
14089e23b: 0f 11 8d 00 01 00 00        	movups	%xmm1, 0x100(%rbp)
14089e242: 48 8b 45 60                 	movq	0x60(%rbp), %rax
14089e246: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
14089e24d: 0f 11 85 18 01 00 00        	movups	%xmm0, 0x118(%rbp)
14089e254: 48 8d 05 15 b3 1f 02        	leaq	0x21fb315(%rip), %rax   # 0x142a99570
14089e25b: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
14089e262: 48 c7 85 30 01 00 00 0f 00 00 00    	movq	$0xf, 0x130(%rbp)
14089e26d: 48 89 bd 38 01 00 00        	movq	%rdi, 0x138(%rbp)
14089e274: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
14089e27b: 48 8b 8d b0 00 00 00        	movq	0xb0(%rbp), %rcx
14089e282: e8 d9 e9 08 00              	callq	0x14092cc60 <.text+0x92bc60>
14089e287: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14089e28e: b0 01                       	movb	$0x1, %al
14089e290: 89 85 78 01 00 00           	movl	%eax, 0x178(%rbp)
14089e296: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
14089e29d: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14089e2a4: e8 f7 63 47 00              	callq	0x140d146a0 <.text+0xd136a0>
14089e2a9: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14089e2b0: 80 78 50 00                 	cmpb	$0x0, 0x50(%rax)
14089e2b4: 75 10                       	jne	0x14089e2c6 <.text+0x89d2c6>
14089e2b6: f3 0f 10 35 46 76 08 02     	movss	0x2087646(%rip), %xmm6  # 0x142925904
14089e2be: 80 78 40 00                 	cmpb	$0x0, 0x40(%rax)
14089e2c2: 74 0d                       	je	0x14089e2d1 <.text+0x89d2d1>
14089e2c4: eb 17                       	jmp	0x14089e2dd <.text+0x89d2dd>
14089e2c6: f3 0f 10 70 54              	movss	0x54(%rax), %xmm6
14089e2cb: 80 78 40 00                 	cmpb	$0x0, 0x40(%rax)
14089e2cf: 75 0c                       	jne	0x14089e2dd <.text+0x89d2dd>
14089e2d1: c6 40 40 00                 	movb	$0x0, 0x40(%rax)
14089e2d5: f3 0f 11 70 44              	movss	%xmm6, 0x44(%rax)
14089e2da: 0f 57 f6                    	xorps	%xmm6, %xmm6
14089e2dd: 4c 8b 78 68                 	movq	0x68(%rax), %r15
14089e2e1: 4c 8b 40 70                 	movq	0x70(%rax), %r8
14089e2e5: 4c 89 f1                    	movq	%r14, %rcx
14089e2e8: 4c 89 fa                    	movq	%r15, %rdx
14089e2eb: e8 b0 97 09 00              	callq	0x140937aa0 <.text+0x936aa0>
14089e2f0: a8 01                       	testb	$0x1, %al
14089e2f2: 0f 84 6b 04 00 00           	je	0x14089e763 <.text+0x89d763>
14089e2f8: 49 89 d5                    	movq	%rdx, %r13
14089e2fb: 4c 8b b5 c8 00 00 00        	movq	0xc8(%rbp), %r14
14089e302: 4d 85 f6                    	testq	%r14, %r14
14089e305: 75 07                       	jne	0x14089e30e <.text+0x89d30e>
14089e307: 4c 8b b5 d0 00 00 00        	movq	0xd0(%rbp), %r14
14089e30e: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14089e315: 44 0f 28 08                 	movaps	(%rax), %xmm9
14089e319: f3 0f 10 78 78              	movss	0x78(%rax), %xmm7
14089e31e: 8b 70 48                    	movl	0x48(%rax), %esi
14089e321: f3 44 0f 10 40 4c           	movss	0x4c(%rax), %xmm8
14089e327: 48 8b 58 40                 	movq	0x40(%rax), %rbx
14089e32b: 48 8b bd e0 00 00 00        	movq	0xe0(%rbp), %rdi
14089e332: 49 8b 06                    	movq	(%r14), %rax
14089e335: 4d 8b 66 10                 	movq	0x10(%r14), %r12
14089e339: 4c 29 e0                    	subq	%r12, %rax
14089e33c: 48 3d 97 00 00 00           	cmpq	$0x97, %rax
14089e342: 0f 86 3d 03 00 00           	jbe	0x14089e685 <.text+0x89d685>
14089e348: 4c 8b 85 58 01 00 00        	movq	0x158(%rbp), %r8
14089e34f: 49 8d 40 7c                 	leaq	0x7c(%r8), %rax
14089e353: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
14089e357: 48 8d 15 32 d3 a3 00        	leaq	0xa3d332(%rip), %rdx    # 0x1412db690 <.text+0x12da690>
14089e35e: 4a 89 14 21                 	movq	%rdx, (%rcx,%r12)
14089e362: 46 0f 11 4c 21 08           	movups	%xmm9, 0x8(%rcx,%r12)
14089e368: 48 8b 10                    	movq	(%rax), %rdx
14089e36b: 4a 89 54 21 18              	movq	%rdx, 0x18(%rcx,%r12)
14089e370: 8b 40 08                    	movl	0x8(%rax), %eax
14089e373: 42 89 44 21 20              	movl	%eax, 0x20(%rcx,%r12)
14089e378: f3 42 0f 11 74 21 24        	movss	%xmm6, 0x24(%rcx,%r12)
14089e37f: f3 42 0f 11 74 21 28        	movss	%xmm6, 0x28(%rcx,%r12)
14089e386: f3 42 0f 11 74 21 2c        	movss	%xmm6, 0x2c(%rcx,%r12)
14089e38d: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14089e394: 4a 89 44 21 38              	movq	%rax, 0x38(%rcx,%r12)
14089e399: 4e 89 6c 21 40              	movq	%r13, 0x40(%rcx,%r12)
14089e39e: 42 89 74 21 48              	movl	%esi, 0x48(%rcx,%r12)
14089e3a3: f3 46 0f 11 44 21 4c        	movss	%xmm8, 0x4c(%rcx,%r12)
14089e3aa: 4a 89 5c 21 50              	movq	%rbx, 0x50(%rcx,%r12)
14089e3af: f3 42 0f 11 7c 21 58        	movss	%xmm7, 0x58(%rcx,%r12)
14089e3b6: 4a c7 44 21 5c 00 00 00 00  	movq	$0x0, 0x5c(%rcx,%r12)
14089e3bf: 42 c6 44 21 68 00           	movb	$0x0, 0x68(%rcx,%r12)
14089e3c5: 4a 89 7c 21 78              	movq	%rdi, 0x78(%rcx,%r12)
14089e3ca: 48 8d 1d 5f 69 cc 00        	leaq	0xcc695f(%rip), %rbx    # 0x141564d30 <.text+0x1563d30>
14089e3d1: 4a 89 9c 21 88 00 00 00     	movq	%rbx, 0x88(%rcx,%r12)
14089e3d9: 4d 8d ac 24 98 00 00 00     	leaq	0x98(%r12), %r13
14089e3e1: 4d 89 6e 10                 	movq	%r13, 0x10(%r14)
14089e3e5: 41 80 b8 89 00 00 00 00     	cmpb	$0x0, 0x89(%r8)
14089e3ed: 0f 84 82 00 00 00           	je	0x14089e475 <.text+0x89d475>
14089e3f3: 49 8b 06                    	movq	(%r14), %rax
14089e3f6: 4c 29 e8                    	subq	%r13, %rax
14089e3f9: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14089e3fd: 0f 86 a7 02 00 00           	jbe	0x14089e6aa <.text+0x89d6aa>
14089e403: 49 8b 46 08                 	movq	0x8(%r14), %rax
14089e407: 48 8d 0d 92 87 a3 00        	leaq	0xa38792(%rip), %rcx    # 0x1412d6ba0 <.text+0x12d5ba0>
14089e40e: 4a 89 8c 20 98 00 00 00     	movq	%rcx, 0x98(%rax,%r12)
14089e416: 4a 89 9c 20 a0 00 00 00     	movq	%rbx, 0xa0(%rax,%r12)
14089e41e: 4a 89 bc 20 a8 00 00 00     	movq	%rdi, 0xa8(%rax,%r12)
14089e426: 42 c6 84 20 b0 00 00 00 00  	movb	$0x0, 0xb0(%rax,%r12)
14089e42f: 4d 8d ac 24 b8 00 00 00     	leaq	0xb8(%r12), %r13
14089e437: 4d 89 6e 10                 	movq	%r13, 0x10(%r14)
14089e43b: 49 8b 06                    	movq	(%r14), %rax
14089e43e: 4c 29 e8                    	subq	%r13, %rax
14089e441: be d8 00 00 00              	movl	$0xd8, %esi
14089e446: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14089e44a: 0f 87 b9 00 00 00           	ja	0x14089e509 <.text+0x89d509>
14089e450: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e459: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14089e45f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e465: 4c 89 f1                    	movq	%r14, %rcx
14089e468: 4c 89 ea                    	movq	%r13, %rdx
14089e46b: e8 f0 b5 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e470: e9 94 00 00 00              	jmp	0x14089e509 <.text+0x89d509>
14089e475: 49 8b 06                    	movq	(%r14), %rax
14089e478: 4c 29 e8                    	subq	%r13, %rax
14089e47b: 41 80 b8 8a 00 00 00 00     	cmpb	$0x0, 0x8a(%r8)
14089e483: 74 2d                       	je	0x14089e4b2 <.text+0x89d4b2>
14089e485: be b8 00 00 00              	movl	$0xb8, %esi
14089e48a: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14089e48e: 77 79                       	ja	0x14089e509 <.text+0x89d509>
14089e490: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e499: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14089e49f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e4a5: 4c 89 f1                    	movq	%r14, %rcx
14089e4a8: 4c 89 ea                    	movq	%r13, %rdx
14089e4ab: e8 b0 b5 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e4b0: eb 57                       	jmp	0x14089e509 <.text+0x89d509>
14089e4b2: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14089e4b6: 0f 86 5d 02 00 00           	jbe	0x14089e719 <.text+0x89d719>
14089e4bc: 49 8b 46 08                 	movq	0x8(%r14), %rax
14089e4c0: 48 8d 0d d9 b1 a3 00        	leaq	0xa3b1d9(%rip), %rcx    # 0x1412d96a0 <.text+0x12d86a0>
14089e4c7: 4a 89 8c 20 98 00 00 00     	movq	%rcx, 0x98(%rax,%r12)
14089e4cf: 4a 89 9c 20 a0 00 00 00     	movq	%rbx, 0xa0(%rax,%r12)
14089e4d7: 4a 89 bc 20 a8 00 00 00     	movq	%rdi, 0xa8(%rax,%r12)
14089e4df: 42 c6 84 20 b0 00 00 00 00  	movb	$0x0, 0xb0(%rax,%r12)
14089e4e8: 4d 8d ac 24 b8 00 00 00     	leaq	0xb8(%r12), %r13
14089e4f0: 4d 89 6e 10                 	movq	%r13, 0x10(%r14)
14089e4f4: 49 8b 06                    	movq	(%r14), %rax
14089e4f7: 4c 29 e8                    	subq	%r13, %rax
14089e4fa: be d8 00 00 00              	movl	$0xd8, %esi
14089e4ff: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14089e503: 0f 86 35 02 00 00           	jbe	0x14089e73e <.text+0x89d73e>
14089e509: 49 8b 46 08                 	movq	0x8(%r14), %rax
14089e50d: 48 8d 0d 4c 9c a3 00        	leaq	0xa39c4c(%rip), %rcx    # 0x1412d8160 <.text+0x12d7160>
14089e514: 4a 89 0c 28                 	movq	%rcx, (%rax,%r13)
14089e518: 4a 89 5c 28 08              	movq	%rbx, 0x8(%rax,%r13)
14089e51d: 4a 89 7c 28 10              	movq	%rdi, 0x10(%rax,%r13)
14089e522: 42 c6 44 28 18 00           	movb	$0x0, 0x18(%rax,%r13)
14089e528: 49 01 f4                    	addq	%rsi, %r12
14089e52b: 4d 89 66 10                 	movq	%r12, 0x10(%r14)
14089e52f: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14089e536: f6 40 58 01                 	testb	$0x1, 0x58(%rax)
14089e53a: 74 53                       	je	0x14089e58f <.text+0x89d58f>
14089e53c: f3 0f 10 70 5c              	movss	0x5c(%rax), %xmm6
14089e541: 49 8b 06                    	movq	(%r14), %rax
14089e544: 4c 29 e0                    	subq	%r12, %rax
14089e547: 48 83 f8 27                 	cmpq	$0x27, %rax
14089e54b: 0f 86 7e 01 00 00           	jbe	0x14089e6cf <.text+0x89d6cf>
14089e551: 49 8b 46 08                 	movq	0x8(%r14), %rax
14089e555: 48 8d 0d 44 9d a3 00        	leaq	0xa39d44(%rip), %rcx    # 0x1412d82a0 <.text+0x12d72a0>
14089e55c: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
14089e560: 4a 89 5c 20 08              	movq	%rbx, 0x8(%rax,%r12)
14089e565: 4a 89 7c 20 10              	movq	%rdi, 0x10(%rax,%r12)
14089e56a: 42 c7 44 20 18 00 00 00 00  	movl	$0x0, 0x18(%rax,%r12)
14089e573: f3 42 0f 11 74 20 1c        	movss	%xmm6, 0x1c(%rax,%r12)
14089e57a: 42 c6 44 20 20 00           	movb	$0x0, 0x20(%rax,%r12)
14089e580: 49 83 c4 28                 	addq	$0x28, %r12
14089e584: 4d 89 66 10                 	movq	%r12, 0x10(%r14)
14089e588: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14089e58f: 80 b8 88 00 00 00 00        	cmpb	$0x0, 0x88(%rax)
14089e596: 74 37                       	je	0x14089e5cf <.text+0x89d5cf>
14089e598: 49 8b 06                    	movq	(%r14), %rax
14089e59b: 4c 29 e0                    	subq	%r12, %rax
14089e59e: 48 83 f8 1f                 	cmpq	$0x1f, %rax
14089e5a2: 0f 86 4c 01 00 00           	jbe	0x14089e6f4 <.text+0x89d6f4>
14089e5a8: 49 8b 46 08                 	movq	0x8(%r14), %rax
14089e5ac: 48 8d 0d 5d ce a3 00        	leaq	0xa3ce5d(%rip), %rcx    # 0x1412db410 <.text+0x12da410>
14089e5b3: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
14089e5b7: 4a 89 5c 20 08              	movq	%rbx, 0x8(%rax,%r12)
14089e5bc: 4a 89 7c 20 10              	movq	%rdi, 0x10(%rax,%r12)
14089e5c1: 42 c6 44 20 18 00           	movb	$0x0, 0x18(%rax,%r12)
14089e5c7: 49 83 c4 20                 	addq	$0x20, %r12
14089e5cb: 4d 89 66 10                 	movq	%r12, 0x10(%r14)
14089e5cf: 80 bd 78 01 00 00 00        	cmpb	$0x0, 0x178(%rbp)
14089e5d6: 74 2f                       	je	0x14089e607 <.text+0x89d607>
14089e5d8: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14089e5df: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14089e5e3: f0                          	lock
14089e5e4: 48 ff 02                    	incq	(%rdx)
14089e5e7: 0f 8e f2 01 00 00           	jle	0x14089e7df <.text+0x89d7df>
14089e5ed: 4c 8d 0d 24 b0 1f 02        	leaq	0x21fb024(%rip), %r9    # 0x142a99618
14089e5f4: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14089e5fb: 4c 8b 85 60 01 00 00        	movq	0x160(%rbp), %r8
14089e602: e8 49 03 a4 00              	callq	0x1412de950 <.text+0x12dd950>
14089e607: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
14089e60e: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14089e615: 48 89 41 20                 	movq	%rax, 0x20(%rcx)
14089e619: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
14089e620: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
14089e627: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
14089e62b: 0f 11 01                    	movups	%xmm0, (%rcx)
14089e62e: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14089e635: 48 8b 50 60                 	movq	0x60(%rax), %rdx
14089e639: 48 85 d2                    	testq	%rdx, %rdx
14089e63c: 74 0e                       	je	0x14089e64c <.text+0x89d64c>
14089e63e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14089e644: 4c 89 f9                    	movq	%r15, %rcx
14089e647: e8 d4 55 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089e64c: 48 8b 85 a8 00 00 00        	movq	0xa8(%rbp), %rax
14089e653: 0f 28 b5 90 01 00 00        	movaps	0x190(%rbp), %xmm6
14089e65a: 0f 28 bd a0 01 00 00        	movaps	0x1a0(%rbp), %xmm7
14089e661: 44 0f 28 85 b0 01 00 00     	movaps	0x1b0(%rbp), %xmm8
14089e669: 44 0f 28 8d c0 01 00 00     	movaps	0x1c0(%rbp), %xmm9
14089e671: 48 81 c4 58 02 00 00        	addq	$0x258, %rsp            # imm = 0x258
14089e678: 5b                          	popq	%rbx
14089e679: 5f                          	popq	%rdi
14089e67a: 5e                          	popq	%rsi
14089e67b: 41 5c                       	popq	%r12
14089e67d: 41 5d                       	popq	%r13
14089e67f: 41 5e                       	popq	%r14
14089e681: 41 5f                       	popq	%r15
14089e683: 5d                          	popq	%rbp
14089e684: c3                          	retq
14089e685: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e68e: 41 b8 98 00 00 00           	movl	$0x98, %r8d
14089e694: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e69a: 4c 89 f1                    	movq	%r14, %rcx
14089e69d: 4c 89 e2                    	movq	%r12, %rdx
14089e6a0: e8 bb b3 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e6a5: e9 9e fc ff ff              	jmp	0x14089e348 <.text+0x89d348>
14089e6aa: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e6b3: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14089e6b9: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e6bf: 4c 89 f1                    	movq	%r14, %rcx
14089e6c2: 4c 89 ea                    	movq	%r13, %rdx
14089e6c5: e8 96 b3 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e6ca: e9 34 fd ff ff              	jmp	0x14089e403 <.text+0x89d403>
14089e6cf: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e6d8: 41 b8 28 00 00 00           	movl	$0x28, %r8d
14089e6de: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e6e4: 4c 89 f1                    	movq	%r14, %rcx
14089e6e7: 4c 89 e2                    	movq	%r12, %rdx
14089e6ea: e8 71 b3 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e6ef: e9 5d fe ff ff              	jmp	0x14089e551 <.text+0x89d551>
14089e6f4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e6fd: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14089e703: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e709: 4c 89 f1                    	movq	%r14, %rcx
14089e70c: 4c 89 e2                    	movq	%r12, %rdx
14089e70f: e8 4c b3 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e714: e9 8f fe ff ff              	jmp	0x14089e5a8 <.text+0x89d5a8>
14089e719: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e722: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14089e728: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e72e: 4c 89 f1                    	movq	%r14, %rcx
14089e731: 4c 89 ea                    	movq	%r13, %rdx
14089e734: e8 27 b3 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e739: e9 7e fd ff ff              	jmp	0x14089e4bc <.text+0x89d4bc>
14089e73e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14089e747: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14089e74d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14089e753: 4c 89 f1                    	movq	%r14, %rcx
14089e756: 4c 89 ea                    	movq	%r13, %rdx
14089e759: e8 02 b3 00 02              	callq	0x1428a9a60 <.text+0x28a8a60>
14089e75e: e9 a6 fd ff ff              	jmp	0x14089e509 <.text+0x89d509>
14089e763: 48 8d 0d 96 ae 1f 02        	leaq	0x21fae96(%rip), %rcx   # 0x142a99600
14089e76a: e8 a1 ab 03 02              	callq	0x1428d9310 <.text+0x28d8310>
14089e76f: eb 6e                       	jmp	0x14089e7df <.text+0x89d7df>
14089e771: 48 8d 45 20                 	leaq	0x20(%rbp), %rax
14089e775: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14089e779: 48 8d 05 20 51 c9 ff        	leaq	-0x36aee0(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
14089e780: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14089e784: 48 8d 05 4d 15 21 02        	leaq	0x221154d(%rip), %rax   # 0x142aafcd8
14089e78b: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
14089e792: 48 c7 85 c8 00 00 00 02 00 00 00    	movq	$0x2, 0xc8(%rbp)
14089e79d: 48 c7 85 e0 00 00 00 00 00 00 00    	movq	$0x0, 0xe0(%rbp)
14089e7a8: 48 8d 45 30                 	leaq	0x30(%rbp), %rax
14089e7ac: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14089e7b3: 48 c7 85 d8 00 00 00 01 00 00 00    	movq	$0x1, 0xd8(%rbp)
14089e7be: 48 8d 15 33 15 21 02        	leaq	0x2211533(%rip), %rdx   # 0x142aafcf8
14089e7c5: 48 8d 8d c0 00 00 00        	leaq	0xc0(%rbp), %rcx
14089e7cc: e8 bf ab 03 02              	callq	0x1428d9390 <.text+0x28d8390>
14089e7d1: eb 0c                       	jmp	0x14089e7df <.text+0x89d7df>
14089e7d3: 48 8d 0d c6 c2 39 02        	leaq	0x239c2c6(%rip), %rcx   # 0x142c3aaa0
14089e7da: e8 f1 98 03 02              	callq	0x1428d80d0 <.text+0x28d70d0>
14089e7df: 0f 0b                       	ud2
14089e7e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14089e7f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089e7f5: 55                          	pushq	%rbp
14089e7f6: 41 57                       	pushq	%r15
14089e7f8: 41 56                       	pushq	%r14
14089e7fa: 41 55                       	pushq	%r13
14089e7fc: 41 54                       	pushq	%r12
14089e7fe: 56                          	pushq	%rsi
14089e7ff: 57                          	pushq	%rdi
14089e800: 53                          	pushq	%rbx
14089e801: 48 83 ec 68                 	subq	$0x68, %rsp
14089e805: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089e80c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089e812: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089e818: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089e81d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089e822: 48 83 bd 78 01 00 00 00     	cmpq	$0x0, 0x178(%rbp)
14089e82a: 74 19                       	je	0x14089e845 <.text+0x89d845>
14089e82c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14089e832: 48 8b 8d 70 01 00 00        	movq	0x170(%rbp), %rcx
14089e839: 48 8b 95 78 01 00 00        	movq	0x178(%rbp), %rdx
14089e840: e8 db 53 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089e845: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089e84a: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089e84f: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089e855: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089e85b: 48 83 c4 68                 	addq	$0x68, %rsp
14089e85f: 5b                          	popq	%rbx
14089e860: 5f                          	popq	%rdi
14089e861: 5e                          	popq	%rsi
14089e862: 41 5c                       	popq	%r12
14089e864: 41 5d                       	popq	%r13
14089e866: 41 5e                       	popq	%r14
14089e868: 41 5f                       	popq	%r15
14089e86a: 5d                          	popq	%rbp
14089e86b: c3                          	retq
14089e86c: 0f 1f 40 00                 	nopl	(%rax)
14089e870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089e875: 55                          	pushq	%rbp
14089e876: 41 57                       	pushq	%r15
14089e878: 41 56                       	pushq	%r14
14089e87a: 41 55                       	pushq	%r13
14089e87c: 41 54                       	pushq	%r12
14089e87e: 56                          	pushq	%rsi
14089e87f: 57                          	pushq	%rdi
14089e880: 53                          	pushq	%rbx
14089e881: 48 83 ec 68                 	subq	$0x68, %rsp
14089e885: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089e88c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089e892: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089e898: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089e89d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089e8a2: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14089e8a9: 48 8b 50 60                 	movq	0x60(%rax), %rdx
14089e8ad: 48 85 d2                    	testq	%rdx, %rdx
14089e8b0: 74 16                       	je	0x14089e8c8 <.text+0x89d8c8>
14089e8b2: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14089e8b9: 48 8b 48 68                 	movq	0x68(%rax), %rcx
14089e8bd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14089e8c3: e8 58 53 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089e8c8: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089e8cd: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089e8d2: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089e8d8: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089e8de: 48 83 c4 68                 	addq	$0x68, %rsp
14089e8e2: 5b                          	popq	%rbx
14089e8e3: 5f                          	popq	%rdi
14089e8e4: 5e                          	popq	%rsi
14089e8e5: 41 5c                       	popq	%r12
14089e8e7: 41 5d                       	popq	%r13
14089e8e9: 41 5e                       	popq	%r14
14089e8eb: 41 5f                       	popq	%r15
14089e8ed: 5d                          	popq	%rbp
14089e8ee: c3                          	retq
14089e8ef: 90                          	nop
14089e8f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089e8f5: 55                          	pushq	%rbp
14089e8f6: 41 57                       	pushq	%r15
14089e8f8: 41 56                       	pushq	%r14
14089e8fa: 41 55                       	pushq	%r13
14089e8fc: 41 54                       	pushq	%r12
14089e8fe: 56                          	pushq	%rsi
14089e8ff: 57                          	pushq	%rdi
14089e900: 53                          	pushq	%rbx
14089e901: 48 83 ec 68                 	subq	$0x68, %rsp
14089e905: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089e90c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089e912: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089e918: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089e91d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089e922: 48 83 bd 70 01 00 00 00     	cmpq	$0x0, 0x170(%rbp)
14089e92a: 74 21                       	je	0x14089e94d <.text+0x89d94d>
14089e92c: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14089e933: 48 c1 e0 02                 	shlq	$0x2, %rax
14089e937: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14089e93b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14089e941: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14089e948: e8 d3 52 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089e94d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089e952: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089e957: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089e95d: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089e963: 48 83 c4 68                 	addq	$0x68, %rsp
14089e967: 5b                          	popq	%rbx
14089e968: 5f                          	popq	%rdi
14089e969: 5e                          	popq	%rsi
14089e96a: 41 5c                       	popq	%r12
14089e96c: 41 5d                       	popq	%r13
14089e96e: 41 5e                       	popq	%r14
14089e970: 41 5f                       	popq	%r15
14089e972: 5d                          	popq	%rbp
14089e973: c3                          	retq
14089e974: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14089e980: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089e985: 55                          	pushq	%rbp
14089e986: 41 57                       	pushq	%r15
14089e988: 41 56                       	pushq	%r14
14089e98a: 41 55                       	pushq	%r13
14089e98c: 41 54                       	pushq	%r12
14089e98e: 56                          	pushq	%rsi
14089e98f: 57                          	pushq	%rdi
14089e990: 53                          	pushq	%rbx
14089e991: 48 83 ec 68                 	subq	$0x68, %rsp
14089e995: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089e99c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089e9a2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089e9a8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089e9ad: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089e9b2: 80 bd 86 01 00 00 00        	cmpb	$0x0, 0x186(%rbp)
14089e9b9: 74 37                       	je	0x14089e9f2 <.text+0x89d9f2>
14089e9bb: 48 8d 4d 30                 	leaq	0x30(%rbp), %rcx
14089e9bf: e8 6c a1 fe ff              	callq	0x140888b30 <.text+0x887b30>
14089e9c4: c6 85 87 01 00 00 01        	movb	$0x1, 0x187(%rbp)
14089e9cb: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089e9d0: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089e9d5: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089e9db: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089e9e1: 48 83 c4 68                 	addq	$0x68, %rsp
14089e9e5: 5b                          	popq	%rbx
14089e9e6: 5f                          	popq	%rdi
14089e9e7: 5e                          	popq	%rsi
14089e9e8: 41 5c                       	popq	%r12
14089e9ea: 41 5d                       	popq	%r13
14089e9ec: 41 5e                       	popq	%r14
14089e9ee: 41 5f                       	popq	%r15
14089e9f0: 5d                          	popq	%rbp
14089e9f1: c3                          	retq
14089e9f2: c6 85 87 01 00 00 01        	movb	$0x1, 0x187(%rbp)
14089e9f9: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089e9fe: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089ea03: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089ea09: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089ea0f: 48 83 c4 68                 	addq	$0x68, %rsp
14089ea13: 5b                          	popq	%rbx
14089ea14: 5f                          	popq	%rdi
14089ea15: 5e                          	popq	%rsi
14089ea16: 41 5c                       	popq	%r12
14089ea18: 41 5d                       	popq	%r13
14089ea1a: 41 5e                       	popq	%r14
14089ea1c: 41 5f                       	popq	%r15
14089ea1e: 5d                          	popq	%rbp
14089ea1f: c3                          	retq
14089ea20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089ea25: 55                          	pushq	%rbp
14089ea26: 41 57                       	pushq	%r15
14089ea28: 41 56                       	pushq	%r14
14089ea2a: 41 55                       	pushq	%r13
14089ea2c: 41 54                       	pushq	%r12
14089ea2e: 56                          	pushq	%rsi
14089ea2f: 57                          	pushq	%rdi
14089ea30: 53                          	pushq	%rbx
14089ea31: 48 83 ec 68                 	subq	$0x68, %rsp
14089ea35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089ea3c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089ea42: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089ea48: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089ea4d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089ea52: 48 8b 8d 78 01 00 00        	movq	0x178(%rbp), %rcx
14089ea59: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14089ea60: 48 89 c2                    	movq	%rax, %rdx
14089ea63: 48 85 c0                    	testq	%rax, %rax
14089ea66: 0f 94 c0                    	sete	%al
14089ea69: 0a 85 87 01 00 00           	orb	0x187(%rbp), %al
14089ea6f: a8 01                       	testb	$0x1, %al
14089ea71: 75 13                       	jne	0x14089ea86 <.text+0x89da86>
14089ea73: 48 c1 e2 02                 	shlq	$0x2, %rdx
14089ea77: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14089ea7b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14089ea81: e8 9a 51 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089ea86: 48 83 bd 68 01 00 00 00     	cmpq	$0x0, 0x168(%rbp)
14089ea8e: 74 21                       	je	0x14089eab1 <.text+0x89dab1>
14089ea90: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14089ea97: 48 c1 e0 02                 	shlq	$0x2, %rax
14089ea9b: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14089ea9f: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14089eaa5: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
14089eaac: e8 6f 51 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089eab1: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089eab6: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089eabb: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089eac1: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089eac7: 48 83 c4 68                 	addq	$0x68, %rsp
14089eacb: 5b                          	popq	%rbx
14089eacc: 5f                          	popq	%rdi
14089eacd: 5e                          	popq	%rsi
14089eace: 41 5c                       	popq	%r12
14089ead0: 41 5d                       	popq	%r13
14089ead2: 41 5e                       	popq	%r14
14089ead4: 41 5f                       	popq	%r15
14089ead6: 5d                          	popq	%rbp
14089ead7: c3                          	retq
14089ead8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14089eae0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089eae5: 55                          	pushq	%rbp
14089eae6: 41 57                       	pushq	%r15
14089eae8: 41 56                       	pushq	%r14
14089eaea: 41 55                       	pushq	%r13
14089eaec: 41 54                       	pushq	%r12
14089eaee: 56                          	pushq	%rsi
14089eaef: 57                          	pushq	%rdi
14089eaf0: 53                          	pushq	%rbx
14089eaf1: 48 83 ec 68                 	subq	$0x68, %rsp
14089eaf5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089eafc: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089eb02: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089eb08: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089eb0d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089eb12: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14089eb19: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
14089eb20: e8 db fd 78 ff              	callq	0x14002e900 <.text+0x2d900>
14089eb25: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089eb2a: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089eb2f: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089eb35: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089eb3b: 48 83 c4 68                 	addq	$0x68, %rsp
14089eb3f: 5b                          	popq	%rbx
14089eb40: 5f                          	popq	%rdi
14089eb41: 5e                          	popq	%rsi
14089eb42: 41 5c                       	popq	%r12
14089eb44: 41 5d                       	popq	%r13
14089eb46: 41 5e                       	popq	%r14
14089eb48: 41 5f                       	popq	%r15
14089eb4a: 5d                          	popq	%rbp
14089eb4b: c3                          	retq
14089eb4c: 0f 1f 40 00                 	nopl	(%rax)
14089eb50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089eb55: 55                          	pushq	%rbp
14089eb56: 41 57                       	pushq	%r15
14089eb58: 41 56                       	pushq	%r14
14089eb5a: 41 55                       	pushq	%r13
14089eb5c: 41 54                       	pushq	%r12
14089eb5e: 56                          	pushq	%rsi
14089eb5f: 57                          	pushq	%rdi
14089eb60: 53                          	pushq	%rbx
14089eb61: 48 83 ec 68                 	subq	$0x68, %rsp
14089eb65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089eb6c: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089eb72: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089eb78: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089eb7d: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089eb82: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
14089eb89: 48 85 c0                    	testq	%rax, %rax
14089eb8c: 74 1a                       	je	0x14089eba8 <.text+0x89dba8>
14089eb8e: 48 c1 e0 02                 	shlq	$0x2, %rax
14089eb92: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
14089eb96: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14089eb9c: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
14089eba3: e8 78 50 d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089eba8: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089ebad: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089ebb2: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089ebb8: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089ebbe: 48 83 c4 68                 	addq	$0x68, %rsp
14089ebc2: 5b                          	popq	%rbx
14089ebc3: 5f                          	popq	%rdi
14089ebc4: 5e                          	popq	%rsi
14089ebc5: 41 5c                       	popq	%r12
14089ebc7: 41 5d                       	popq	%r13
14089ebc9: 41 5e                       	popq	%r14
14089ebcb: 41 5f                       	popq	%r15
14089ebcd: 5d                          	popq	%rbp
14089ebce: c3                          	retq
14089ebcf: 90                          	nop
14089ebd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14089ebd5: 55                          	pushq	%rbp
14089ebd6: 41 57                       	pushq	%r15
14089ebd8: 41 56                       	pushq	%r14
14089ebda: 41 55                       	pushq	%r13
14089ebdc: 41 54                       	pushq	%r12
14089ebde: 56                          	pushq	%rsi
14089ebdf: 57                          	pushq	%rdi
14089ebe0: 53                          	pushq	%rbx
14089ebe1: 48 83 ec 68                 	subq	$0x68, %rsp
14089ebe5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14089ebec: 44 0f 29 4c 24 20           	movaps	%xmm9, 0x20(%rsp)
14089ebf2: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14089ebf8: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14089ebfd: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14089ec02: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14089ec09: 48 8b 50 08                 	movq	0x8(%rax), %rdx
14089ec0d: 48 85 d2                    	testq	%rdx, %rdx
14089ec10: 74 17                       	je	0x14089ec29 <.text+0x89dc29>
14089ec12: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
14089ec19: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14089ec1d: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14089ec24: e8 f7 4f d7 ff              	callq	0x140613c20 <.text+0x612c20>
14089ec29: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14089ec2e: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14089ec33: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14089ec39: 44 0f 28 4c 24 20           	movaps	0x20(%rsp), %xmm9
14089ec3f: 48 83 c4 68                 	addq	$0x68, %rsp
14089ec43: 5b                          	popq	%rbx
14089ec44: 5f                          	popq	%rdi
14089ec45: 5e                          	popq	%rsi
14089ec46: 41 5c                       	popq	%r12
14089ec48: 41 5d                       	popq	%r13
14089ec4a: 41 5e                       	popq	%r14
14089ec4c: 41 5f                       	popq	%r15
14089ec4e: 5d                          	popq	%rbp
14089ec4f: c3                          	retq
