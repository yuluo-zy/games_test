
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2be70: 55                          	pushq	%rbp
140b2be71: 41 57                       	pushq	%r15
140b2be73: 41 56                       	pushq	%r14
140b2be75: 56                          	pushq	%rsi
140b2be76: 57                          	pushq	%rdi
140b2be77: 53                          	pushq	%rbx
140b2be78: 48 81 ec 38 01 00 00        	subq	$0x138, %rsp            # imm = 0x138
140b2be7f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140b2be87: 48 c7 85 b0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb0(%rbp)
140b2be92: 48 89 ce                    	movq	%rcx, %rsi
140b2be95: 0f b6 9d 20 01 00 00        	movzbl	0x120(%rbp), %ebx
140b2be9c: 4c 89 45 d8                 	movq	%r8, -0x28(%rbp)
140b2bea0: 4c 89 4d e0                 	movq	%r9, -0x20(%rbp)
140b2bea4: 4d 85 c0                    	testq	%r8, %r8
140b2bea7: 74 1d                       	je	0x140b2bec6 <.text+0xb2aec6>
140b2bea9: 49 83 f8 01                 	cmpq	$0x1, %r8
140b2bead: 0f 85 cc 00 00 00           	jne	0x140b2bf7f <.text+0xb2af7f>
140b2beb3: 48 8b 82 90 00 00 00        	movq	0x90(%rdx), %rax
140b2beba: 48 81 c2 90 00 00 00        	addq	$0x90, %rdx
140b2bec1: e9 c7 00 00 00              	jmp	0x140b2bf8d <.text+0xb2af8d>
140b2bec6: 4c 89 4d 10                 	movq	%r9, 0x10(%rbp)
140b2beca: 48 83 7a 18 00              	cmpq	$0x0, 0x18(%rdx)
140b2becf: 0f 84 d7 00 00 00           	je	0x140b2bfac <.text+0xb2afac>
140b2bed5: 4c 89 cf                    	movq	%r9, %rdi
140b2bed8: 48 8d 4a 20                 	leaq	0x20(%rdx), %rcx
140b2bedc: 48 8d 45 10                 	leaq	0x10(%rbp), %rax
140b2bee0: 49 89 d6                    	movq	%rdx, %r14
140b2bee3: 48 89 c2                    	movq	%rax, %rdx
140b2bee6: e8 85 4e a8 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140b2beeb: 49 8b 0e                    	movq	(%r14), %rcx
140b2beee: 49 8b 56 08                 	movq	0x8(%r14), %rdx
140b2bef2: 49 89 d0                    	movq	%rdx, %r8
140b2bef5: 49 21 c0                    	andq	%rax, %r8
140b2bef8: 48 c1 e8 39                 	shrq	$0x39, %rax
140b2befc: 66 0f 6e c0                 	movd	%eax, %xmm0
140b2bf00: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140b2bf04: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140b2bf09: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140b2bf0e: 4c 8d 89 68 ff ff ff        	leaq	-0x98(%rcx), %r9
140b2bf15: 31 c0                       	xorl	%eax, %eax
140b2bf17: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140b2bf1b: 45 31 d2                    	xorl	%r10d, %r10d
140b2bf1e: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
140b2bf24: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140b2bf28: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140b2bf2c: 66 44 0f d7 db              	pmovmskb	%xmm3, %r11d
140b2bf31: 45 85 db                    	testl	%r11d, %r11d
140b2bf34: 74 2b                       	je	0x140b2bf61 <.text+0xb2af61>
140b2bf36: 49 89 fe                    	movq	%rdi, %r14
140b2bf39: f3 45 0f bc fb              	tzcntl	%r11d, %r15d
140b2bf3e: 4d 01 c7                    	addq	%r8, %r15
140b2bf41: 49 21 d7                    	andq	%rdx, %r15
140b2bf44: 49 f7 df                    	negq	%r15
140b2bf47: 4d 69 ff 98 00 00 00        	imulq	$0x98, %r15, %r15
140b2bf4e: 4f 39 34 39                 	cmpq	%r14, (%r9,%r15)
140b2bf52: 74 42                       	je	0x140b2bf96 <.text+0xb2af96>
140b2bf54: 45 8d 7b ff                 	leal	-0x1(%r11), %r15d
140b2bf58: 66 45 21 df                 	andw	%r11w, %r15w
140b2bf5c: 45 89 fb                    	movl	%r15d, %r11d
140b2bf5f: 75 d8                       	jne	0x140b2bf39 <.text+0xb2af39>
140b2bf61: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140b2bf65: 66 44 0f d7 da              	pmovmskb	%xmm2, %r11d
140b2bf6a: 45 85 db                    	testl	%r11d, %r11d
140b2bf6d: 75 2d                       	jne	0x140b2bf9c <.text+0xb2af9c>
140b2bf6f: 4d 01 d0                    	addq	%r10, %r8
140b2bf72: 49 83 c0 10                 	addq	$0x10, %r8
140b2bf76: 49 83 c2 10                 	addq	$0x10, %r10
140b2bf7a: 49 21 d0                    	andq	%rdx, %r8
140b2bf7d: eb 9f                       	jmp	0x140b2bf1e <.text+0xb2af1e>
140b2bf7f: 48 8b 82 20 01 00 00        	movq	0x120(%rdx), %rax
140b2bf86: 48 81 c2 20 01 00 00        	addq	$0x120, %rdx            # imm = 0x120
140b2bf8d: 48 85 c0                    	testq	%rax, %rax
140b2bf90: 48 0f 44 d0                 	cmoveq	%rax, %rdx
140b2bf94: eb 23                       	jmp	0x140b2bfb9 <.text+0xb2afb9>
140b2bf96: 4c 01 f9                    	addq	%r15, %rcx
140b2bf99: 48 89 c8                    	movq	%rcx, %rax
140b2bf9c: 48 8d 88 68 ff ff ff        	leaq	-0x98(%rax), %rcx
140b2bfa3: 48 85 c0                    	testq	%rax, %rax
140b2bfa6: 48 0f 44 c8                 	cmoveq	%rax, %rcx
140b2bfaa: eb 02                       	jmp	0x140b2bfae <.text+0xb2afae>
140b2bfac: 31 c9                       	xorl	%ecx, %ecx
140b2bfae: 48 8d 51 08                 	leaq	0x8(%rcx), %rdx
140b2bfb2: 48 85 c9                    	testq	%rcx, %rcx
140b2bfb5: 48 0f 44 d1                 	cmoveq	%rcx, %rdx
140b2bfb9: 4c 8b b5 10 01 00 00        	movq	0x110(%rbp), %r14
140b2bfc0: 48 85 d2                    	testq	%rdx, %rdx
140b2bfc3: 0f 95 c0                    	setne	%al
140b2bfc6: 80 f3 01                    	xorb	$0x1, %bl
140b2bfc9: 84 d8                       	testb	%bl, %al
140b2bfcb: 0f 84 ab 00 00 00           	je	0x140b2c07c <.text+0xb2b07c>
140b2bfd1: 48 8b 42 48                 	movq	0x48(%rdx), %rax
140b2bfd5: 48 85 c0                    	testq	%rax, %rax
140b2bfd8: 0f 84 a7 00 00 00           	je	0x140b2c085 <.text+0xb2b085>
140b2bfde: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
140b2bfe2: 4c 8d 41 10                 	leaq	0x10(%rcx), %r8
140b2bfe6: 66 0f 6f 01                 	movdqa	(%rcx), %xmm0
140b2bfea: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
140b2bfef: 41 f7 d1                    	notl	%r9d
140b2bff2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b2c000: 66 45 85 c9                 	testw	%r9w, %r9w
140b2c004: 74 0a                       	je	0x140b2c010 <.text+0xb2b010>
140b2c006: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140b2c00a: 45 21 ca                    	andl	%r9d, %r10d
140b2c00d: eb 31                       	jmp	0x140b2c040 <.text+0xb2b040>
140b2c00f: 90                          	nop
140b2c010: 66 41 0f 6f 00              	movdqa	(%r8), %xmm0
140b2c015: 66 44 0f d7 c8              	pmovmskb	%xmm0, %r9d
140b2c01a: 48 81 c1 00 fb ff ff        	addq	$-0x500, %rcx           # imm = 0xFB00
140b2c021: 49 83 c0 10                 	addq	$0x10, %r8
140b2c025: 41 81 f9 ff ff 00 00        	cmpl	$0xffff, %r9d           # imm = 0xFFFF
140b2c02c: 74 e2                       	je	0x140b2c010 <.text+0xb2b010>
140b2c02e: 41 bb fe ff ff ff           	movl	$0xfffffffe, %r11d      # imm = 0xFFFFFFFE
140b2c034: 45 29 cb                    	subl	%r9d, %r11d
140b2c037: 41 f7 d1                    	notl	%r9d
140b2c03a: 45 89 ca                    	movl	%r9d, %r10d
140b2c03d: 45 21 da                    	andl	%r11d, %r10d
140b2c040: f3 45 0f bc c9              	tzcntl	%r9d, %r9d
140b2c045: 49 f7 d9                    	negq	%r9
140b2c048: 4f 8d 0c 89                 	leaq	(%r9,%r9,4), %r9
140b2c04c: 49 c1 e1 04                 	shlq	$0x4, %r9
140b2c050: 4e 8d 1c 09                 	leaq	(%rcx,%r9), %r11
140b2c054: 49 83 c3 b0                 	addq	$-0x50, %r11
140b2c058: 4a 8d 3c 09                 	leaq	(%rcx,%r9), %rdi
140b2c05c: 48 83 c7 b8                 	addq	$-0x48, %rdi
140b2c060: 4c 89 5d 08                 	movq	%r11, 0x8(%rbp)
140b2c064: 48 89 7d e8                 	movq	%rdi, -0x18(%rbp)
140b2c068: 42 80 7c 09 ff 00           	cmpb	$0x0, -0x1(%rcx,%r9)
140b2c06e: 0f 85 8b 02 00 00           	jne	0x140b2c2ff <.text+0xb2b2ff>
140b2c074: 45 89 d1                    	movl	%r10d, %r9d
140b2c077: 48 ff c8                    	decq	%rax
140b2c07a: 75 84                       	jne	0x140b2c000 <.text+0xb2b000>
140b2c07c: 48 85 d2                    	testq	%rdx, %rdx
140b2c07f: 0f 84 c1 00 00 00           	je	0x140b2c146 <.text+0xb2b146>
140b2c085: 48 8b 42 30                 	movq	0x30(%rdx), %rax
140b2c089: 48 8b 4a 38                 	movq	0x38(%rdx), %rcx
140b2c08d: 48 01 c1                    	addq	%rax, %rcx
140b2c090: 48 ff c1                    	incq	%rcx
140b2c093: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
140b2c097: 66 44 0f d7 c0              	pmovmskb	%xmm0, %r8d
140b2c09c: 41 f7 d0                    	notl	%r8d
140b2c09f: 66 44 89 45 28              	movw	%r8w, 0x28(%rbp)
140b2c0a4: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140b2c0a8: 48 83 c0 10                 	addq	$0x10, %rax
140b2c0ac: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140b2c0b0: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
140b2c0b4: 48 8b 42 48                 	movq	0x48(%rdx), %rax
140b2c0b8: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140b2c0bc: c6 85 af 00 00 00 00        	movb	$0x0, 0xaf(%rbp)
140b2c0c3: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140b2c0c7: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140b2c0cb: e8 a0 d5 91 00              	callq	0x141449670 <.text+0x1448670>
140b2c0d0: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
140b2c0d4: 4c 8b b5 10 01 00 00        	movq	0x110(%rbp), %r14
140b2c0db: 49 8b 0e                    	movq	(%r14), %rcx
140b2c0de: 48 85 c9                    	testq	%rcx, %rcx
140b2c0e1: 0f 84 c1 00 00 00           	je	0x140b2c1a8 <.text+0xb2b1a8>
140b2c0e7: 48 85 c0                    	testq	%rax, %rax
140b2c0ea: 74 66                       	je	0x140b2c152 <.text+0xb2b152>
140b2c0ec: f3 41 0f 6f 06              	movdqu	(%r14), %xmm0
140b2c0f1: f3 41 0f 6f 4e 10           	movdqu	0x10(%r14), %xmm1
140b2c0f7: f3 41 0f 6f 56 20           	movdqu	0x20(%r14), %xmm2
140b2c0fd: 66 0f 7f 55 30              	movdqa	%xmm2, 0x30(%rbp)
140b2c102: 66 0f 7f 4d 20              	movdqa	%xmm1, 0x20(%rbp)
140b2c107: 66 0f 7f 45 10              	movdqa	%xmm0, 0x10(%rbp)
140b2c10c: f3 0f 6f 5d a8              	movdqu	-0x58(%rbp), %xmm3
140b2c111: 0f 10 65 b8                 	movups	-0x48(%rbp), %xmm4
140b2c115: 0f 10 6d c8                 	movups	-0x38(%rbp), %xmm5
140b2c119: 66 0f 7f 5d 40              	movdqa	%xmm3, 0x40(%rbp)
140b2c11e: 0f 29 65 50                 	movaps	%xmm4, 0x50(%rbp)
140b2c122: 0f 29 6d 60                 	movaps	%xmm5, 0x60(%rbp)
140b2c126: 0f 11 6e 50                 	movups	%xmm5, 0x50(%rsi)
140b2c12a: 0f 11 66 40                 	movups	%xmm4, 0x40(%rsi)
140b2c12e: f3 0f 7f 5e 30              	movdqu	%xmm3, 0x30(%rsi)
140b2c133: f3 0f 7f 56 20              	movdqu	%xmm2, 0x20(%rsi)
140b2c138: f3 0f 7f 4e 10              	movdqu	%xmm1, 0x10(%rsi)
140b2c13d: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
140b2c141: e9 a6 01 00 00              	jmp	0x140b2c2ec <.text+0xb2b2ec>
140b2c146: 49 8b 0e                    	movq	(%r14), %rcx
140b2c149: 48 85 c9                    	testq	%rcx, %rcx
140b2c14c: 0f 84 c9 00 00 00           	je	0x140b2c21b <.text+0xb2b21b>
140b2c152: 49 83 7e 18 00              	cmpq	$0x0, 0x18(%r14)
140b2c157: 0f 84 ca 00 00 00           	je	0x140b2c227 <.text+0xb2b227>
140b2c15d: f3 41 0f 6f 06              	movdqu	(%r14), %xmm0
140b2c162: f3 41 0f 6f 4e 10           	movdqu	0x10(%r14), %xmm1
140b2c168: f3 41 0f 6f 56 20           	movdqu	0x20(%r14), %xmm2
140b2c16e: 66 0f 7f 95 90 00 00 00     	movdqa	%xmm2, 0x90(%rbp)
140b2c176: 66 0f 7f 8d 80 00 00 00     	movdqa	%xmm1, 0x80(%rbp)
140b2c17e: 66 0f 7f 45 70              	movdqa	%xmm0, 0x70(%rbp)
140b2c183: e8 b8 15 22 00              	callq	0x140d4d740 <.text+0xd4c740>
140b2c188: 48 89 c7                    	movq	%rax, %rdi
140b2c18b: f6 00 01                    	testb	$0x1, (%rax)
140b2c18e: 0f 84 db 00 00 00           	je	0x140b2c26f <.text+0xb2b26f>
140b2c194: 48 89 f9                    	movq	%rdi, %rcx
140b2c197: 48 83 c1 08                 	addq	$0x8, %rcx
140b2c19b: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140b2c19f: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140b2c1a3: e9 e2 00 00 00              	jmp	0x140b2c28a <.text+0xb2b28a>
140b2c1a8: 48 85 c0                    	testq	%rax, %rax
140b2c1ab: 74 6e                       	je	0x140b2c21b <.text+0xb2b21b>
140b2c1ad: 0f b6 85 18 01 00 00        	movzbl	0x118(%rbp), %eax
140b2c1b4: f3 0f 6f 45 a8              	movdqu	-0x58(%rbp), %xmm0
140b2c1b9: f3 0f 6f 4d b8              	movdqu	-0x48(%rbp), %xmm1
140b2c1be: f3 0f 6f 55 c8              	movdqu	-0x38(%rbp), %xmm2
140b2c1c3: 66 0f 7f 95 90 00 00 00     	movdqa	%xmm2, 0x90(%rbp)
140b2c1cb: 66 0f 7f 8d 80 00 00 00     	movdqa	%xmm1, 0x80(%rbp)
140b2c1d3: 66 0f 7f 45 70              	movdqa	%xmm0, 0x70(%rbp)
140b2c1d8: 84 c0                       	testb	%al, %al
140b2c1da: 0f 85 8f 01 00 00           	jne	0x140b2c36f <.text+0xb2b36f>
140b2c1e0: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
140b2c1e7: 4c 8b 4d 78                 	movq	0x78(%rbp), %r9
140b2c1eb: 4d 85 c9                    	testq	%r9, %r9
140b2c1ee: 0f 84 f8 00 00 00           	je	0x140b2c2ec <.text+0xb2b2ec>
140b2c1f4: 49 8d 41 01                 	leaq	0x1(%r9), %rax
140b2c1f8: b9 58 00 00 00              	movl	$0x58, %ecx
140b2c1fd: 48 f7 e1                    	mulq	%rcx
140b2c200: 48 83 c0 0f                 	addq	$0xf, %rax
140b2c204: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2c208: 49 01 c1                    	addq	%rax, %r9
140b2c20b: 49 83 c1 11                 	addq	$0x11, %r9
140b2c20f: 0f 84 d7 00 00 00           	je	0x140b2c2ec <.text+0xb2b2ec>
140b2c215: 48 8b 4d 70                 	movq	0x70(%rbp), %rcx
140b2c219: eb 41                       	jmp	0x140b2c25c <.text+0xb2b25c>
140b2c21b: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
140b2c222: e9 c5 00 00 00              	jmp	0x140b2c2ec <.text+0xb2b2ec>
140b2c227: 4d 8b 4e 08                 	movq	0x8(%r14), %r9
140b2c22b: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
140b2c232: 4d 85 c9                    	testq	%r9, %r9
140b2c235: 0f 84 b1 00 00 00           	je	0x140b2c2ec <.text+0xb2b2ec>
140b2c23b: 49 8d 41 01                 	leaq	0x1(%r9), %rax
140b2c23f: ba 58 00 00 00              	movl	$0x58, %edx
140b2c244: 48 f7 e2                    	mulq	%rdx
140b2c247: 48 83 c0 0f                 	addq	$0xf, %rax
140b2c24b: 48 83 e0 f0                 	andq	$-0x10, %rax
140b2c24f: 49 01 c1                    	addq	%rax, %r9
140b2c252: 49 83 c1 11                 	addq	$0x11, %r9
140b2c256: 0f 84 90 00 00 00           	je	0x140b2c2ec <.text+0xb2b2ec>
140b2c25c: 48 29 c1                    	subq	%rax, %rcx
140b2c25f: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140b2c265: 4c 89 ca                    	movq	%r9, %rdx
140b2c268: e8 b3 79 ae ff              	callq	0x140613c20 <.text+0x612c20>
140b2c26d: eb 7d                       	jmp	0x140b2c2ec <.text+0xb2b2ec>
140b2c26f: e8 8c 8b 23 00              	callq	0x140d64e00 <.text+0xd63e00>
140b2c274: 48 89 f9                    	movq	%rdi, %rcx
140b2c277: 48 83 c1 08                 	addq	$0x8, %rcx
140b2c27b: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
140b2c282: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
140b2c286: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140b2c28a: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
140b2c28e: 4c 89 01                    	movq	%r8, (%rcx)
140b2c291: 66 0f 6f 45 70              	movdqa	0x70(%rbp), %xmm0
140b2c296: 66 0f 6f 8d 80 00 00 00     	movdqa	0x80(%rbp), %xmm1
140b2c29e: 66 0f 6f 95 90 00 00 00     	movdqa	0x90(%rbp), %xmm2
140b2c2a6: 66 0f 7f 55 30              	movdqa	%xmm2, 0x30(%rbp)
140b2c2ab: 66 0f 7f 4d 20              	movdqa	%xmm1, 0x20(%rbp)
140b2c2b0: 66 0f 7f 45 10              	movdqa	%xmm0, 0x10(%rbp)
140b2c2b5: f3 0f 6f 1d 33 ee fb 01     	movdqu	0x1fbee33(%rip), %xmm3  # 0x142aeb0f0
140b2c2bd: 66 0f 7f 5d 40              	movdqa	%xmm3, 0x40(%rbp)
140b2c2c2: 0f 10 25 37 ee fb 01        	movups	0x1fbee37(%rip), %xmm4  # 0x142aeb100
140b2c2c9: 0f 29 65 50                 	movaps	%xmm4, 0x50(%rbp)
140b2c2cd: 0f 11 66 40                 	movups	%xmm4, 0x40(%rsi)
140b2c2d1: f3 0f 7f 5e 30              	movdqu	%xmm3, 0x30(%rsi)
140b2c2d6: f3 0f 7f 56 20              	movdqu	%xmm2, 0x20(%rsi)
140b2c2db: f3 0f 7f 4e 10              	movdqu	%xmm1, 0x10(%rsi)
140b2c2e0: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
140b2c2e4: 48 89 46 50                 	movq	%rax, 0x50(%rsi)
140b2c2e8: 48 89 56 58                 	movq	%rdx, 0x58(%rsi)
140b2c2ec: 48 89 f0                    	movq	%rsi, %rax
140b2c2ef: 48 81 c4 38 01 00 00        	addq	$0x138, %rsp            # imm = 0x138
140b2c2f6: 5b                          	popq	%rbx
140b2c2f7: 5f                          	popq	%rdi
140b2c2f8: 5e                          	popq	%rsi
140b2c2f9: 41 5e                       	popq	%r14
140b2c2fb: 41 5f                       	popq	%r15
140b2c2fd: 5d                          	popq	%rbp
140b2c2fe: c3                          	retq
140b2c2ff: 48 8d 45 08                 	leaq	0x8(%rbp), %rax
140b2c303: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140b2c307: 48 8d 05 22 85 f1 ff        	leaq	-0xe7ade(%rip), %rax    # 0x140a44830 <.text+0xa43830>
140b2c30e: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
140b2c312: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
140b2c316: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140b2c31d: 48 8d 05 dc 7b f1 ff        	leaq	-0xe8424(%rip), %rax    # 0x140a43f00 <.text+0xa42f00>
140b2c324: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140b2c32b: 48 8d 05 0e f2 fb 01        	leaq	0x1fbf20e(%rip), %rax   # 0x142aeb540
140b2c332: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140b2c336: 48 c7 45 18 02 00 00 00     	movq	$0x2, 0x18(%rbp)
140b2c33e: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140b2c346: 48 8d 45 70                 	leaq	0x70(%rbp), %rax
140b2c34a: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140b2c34e: 48 c7 45 28 02 00 00 00     	movq	$0x2, 0x28(%rbp)
140b2c356: c6 85 af 00 00 00 00        	movb	$0x0, 0xaf(%rbp)
140b2c35d: 48 8d 15 3c f2 fb 01        	leaq	0x1fbf23c(%rip), %rdx   # 0x142aeb5a0
140b2c364: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b2c368: e8 23 d0 da 01              	callq	0x1428d9390 <.text+0x28d8390>
140b2c36d: eb 61                       	jmp	0x140b2c3d0 <.text+0xb2b3d0>
140b2c36f: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
140b2c373: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140b2c377: 48 8d 05 12 47 00 00        	leaq	0x4712(%rip), %rax      # 0x140b30a90 <.text+0xb2fa90>
140b2c37e: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
140b2c382: 48 8d 45 70                 	leaq	0x70(%rbp), %rax
140b2c386: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140b2c38a: 48 8d 05 7f 46 00 00        	leaq	0x467f(%rip), %rax      # 0x140b30a10 <.text+0xb2fa10>
140b2c391: 48 89 45 00                 	movq	%rax, (%rbp)
140b2c395: 48 8d 05 84 f2 fb 01        	leaq	0x1fbf284(%rip), %rax   # 0x142aeb620
140b2c39c: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140b2c3a0: 48 c7 45 18 03 00 00 00     	movq	$0x3, 0x18(%rbp)
140b2c3a8: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140b2c3b0: 48 8d 45 e8                 	leaq	-0x18(%rbp), %rax
140b2c3b4: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140b2c3b8: 48 c7 45 28 02 00 00 00     	movq	$0x2, 0x28(%rbp)
140b2c3c0: 48 8d 15 89 f2 fb 01        	leaq	0x1fbf289(%rip), %rdx   # 0x142aeb650
140b2c3c7: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140b2c3cb: e8 c0 cf da 01              	callq	0x1428d9390 <.text+0x28d8390>
140b2c3d0: 0f 0b                       	ud2
140b2c3d2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140b2c3e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2c3e5: 55                          	pushq	%rbp
140b2c3e6: 41 57                       	pushq	%r15
140b2c3e8: 41 56                       	pushq	%r14
140b2c3ea: 56                          	pushq	%rsi
140b2c3eb: 57                          	pushq	%rdi
140b2c3ec: 53                          	pushq	%rbx
140b2c3ed: 48 83 ec 28                 	subq	$0x28, %rsp
140b2c3f1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2c3f8: 80 bd af 00 00 00 00        	cmpb	$0x0, 0xaf(%rbp)
140b2c3ff: 75 19                       	jne	0x140b2c41a <.text+0xb2b41a>
140b2c401: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
140b2c408: 48 83 38 00                 	cmpq	$0x0, (%rax)
140b2c40c: 74 0c                       	je	0x140b2c41a <.text+0xb2b41a>
140b2c40e: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
140b2c415: e8 e6 aa d4 ff              	callq	0x140876f00 <.text+0x875f00>
140b2c41a: 90                          	nop
140b2c41b: 48 83 c4 28                 	addq	$0x28, %rsp
140b2c41f: 5b                          	popq	%rbx
140b2c420: 5f                          	popq	%rdi
140b2c421: 5e                          	popq	%rsi
140b2c422: 41 5e                       	popq	%r14
140b2c424: 41 5f                       	popq	%r15
140b2c426: 5d                          	popq	%rbp
140b2c427: c3                          	retq
140b2c428: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140b2c430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2c435: 55                          	pushq	%rbp
140b2c436: 41 57                       	pushq	%r15
140b2c438: 41 56                       	pushq	%r14
140b2c43a: 56                          	pushq	%rsi
140b2c43b: 57                          	pushq	%rdi
140b2c43c: 53                          	pushq	%rbx
140b2c43d: 48 83 ec 28                 	subq	$0x28, %rsp
140b2c441: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2c448: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140b2c44c: e8 af aa d4 ff              	callq	0x140876f00 <.text+0x875f00>
140b2c451: c6 85 af 00 00 00 01        	movb	$0x1, 0xaf(%rbp)
140b2c458: 48 83 c4 28                 	addq	$0x28, %rsp
140b2c45c: 5b                          	popq	%rbx
140b2c45d: 5f                          	popq	%rdi
140b2c45e: 5e                          	popq	%rsi
140b2c45f: 41 5e                       	popq	%r14
140b2c461: 41 5f                       	popq	%r15
140b2c463: 5d                          	popq	%rbp
140b2c464: c3                          	retq
140b2c465: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140b2c470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140b2c475: 55                          	pushq	%rbp
140b2c476: 41 57                       	pushq	%r15
140b2c478: 41 56                       	pushq	%r14
140b2c47a: 56                          	pushq	%rsi
140b2c47b: 57                          	pushq	%rdi
140b2c47c: 53                          	pushq	%rbx
140b2c47d: 48 83 ec 28                 	subq	$0x28, %rsp
140b2c481: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140b2c488: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140b2c48c: e8 6f aa d4 ff              	callq	0x140876f00 <.text+0x875f00>
140b2c491: c6 85 af 00 00 00 01        	movb	$0x1, 0xaf(%rbp)
140b2c498: 48 83 c4 28                 	addq	$0x28, %rsp
140b2c49c: 5b                          	popq	%rbx
140b2c49d: 5f                          	popq	%rdi
140b2c49e: 5e                          	popq	%rsi
140b2c49f: 41 5e                       	popq	%r14
140b2c4a1: 41 5f                       	popq	%r15
140b2c4a3: 5d                          	popq	%rbp
140b2c4a4: c3                          	retq
