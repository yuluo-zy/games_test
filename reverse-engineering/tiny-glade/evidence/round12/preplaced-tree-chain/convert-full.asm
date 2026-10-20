
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14226be70: 55                          	pushq	%rbp
14226be71: 41 57                       	pushq	%r15
14226be73: 41 56                       	pushq	%r14
14226be75: 41 55                       	pushq	%r13
14226be77: 41 54                       	pushq	%r12
14226be79: 56                          	pushq	%rsi
14226be7a: 57                          	pushq	%rdi
14226be7b: 53                          	pushq	%rbx
14226be7c: 48 81 ec a8 02 00 00        	subq	$0x2a8, %rsp            # imm = 0x2A8
14226be83: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14226be8b: 44 0f 29 bd 10 02 00 00     	movaps	%xmm15, 0x210(%rbp)
14226be93: 44 0f 29 b5 00 02 00 00     	movaps	%xmm14, 0x200(%rbp)
14226be9b: 44 0f 29 ad f0 01 00 00     	movaps	%xmm13, 0x1f0(%rbp)
14226bea3: 44 0f 29 a5 e0 01 00 00     	movaps	%xmm12, 0x1e0(%rbp)
14226beab: 44 0f 29 9d d0 01 00 00     	movaps	%xmm11, 0x1d0(%rbp)
14226beb3: 44 0f 29 95 c0 01 00 00     	movaps	%xmm10, 0x1c0(%rbp)
14226bebb: 44 0f 29 8d b0 01 00 00     	movaps	%xmm9, 0x1b0(%rbp)
14226bec3: 44 0f 29 85 a0 01 00 00     	movaps	%xmm8, 0x1a0(%rbp)
14226becb: 0f 29 bd 90 01 00 00        	movaps	%xmm7, 0x190(%rbp)
14226bed2: 0f 29 b5 80 01 00 00        	movaps	%xmm6, 0x180(%rbp)
14226bed9: 48 c7 85 78 01 00 00 fe ff ff ff    	movq	$-0x2, 0x178(%rbp)
14226bee4: 48 8b 01                    	movq	(%rcx), %rax
14226bee7: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
14226beeb: 4c 8b 10                    	movq	(%rax), %r10
14226beee: 45 31 db                    	xorl	%r11d, %r11d
14226bef1: 4c 89 d6                    	movq	%r10, %rsi
14226bef4: 48 2b 71 18                 	subq	0x18(%rcx), %rsi
14226bef8: 49 0f 42 f3                 	cmovbq	%r11, %rsi
14226befc: 4c 2b 51 38                 	subq	0x38(%rcx), %r10
14226bf00: 4d 0f 42 d3                 	cmovbq	%r11, %r10
14226bf04: 48 8b 79 30                 	movq	0x30(%rcx), %rdi
14226bf08: 4c 29 d7                    	subq	%r10, %rdi
14226bf0b: 49 0f 42 fb                 	cmovbq	%r11, %rdi
14226bf0f: 41 0f 96 c2                 	setbe	%r10b
14226bf13: 48 8b 59 10                 	movq	0x10(%rcx), %rbx
14226bf17: 48 29 f3                    	subq	%rsi, %rbx
14226bf1a: 49 0f 42 db                 	cmovbq	%r11, %rbx
14226bf1e: 48 8b 49 40                 	movq	0x40(%rcx), %rcx
14226bf22: 41 0f 96 c3                 	setbe	%r11b
14226bf26: 48 01 fb                    	addq	%rdi, %rbx
14226bf29: 48 89 ce                    	movq	%rcx, %rsi
14226bf2c: 48 29 de                    	subq	%rbx, %rsi
14226bf2f: 48 89 30                    	movq	%rsi, (%rax)
14226bf32: 45 84 d3                    	testb	%r10b, %r11b
14226bf35: 0f 84 5f 07 00 00           	je	0x14226c69a <.text+0x226b69a>
14226bf3b: 41 f6 40 08 01              	testb	$0x1, 0x8(%r8)
14226bf40: 0f 84 63 07 00 00           	je	0x14226c6a9 <.text+0x226b6a9>
14226bf46: 49 8b 00                    	movq	(%r8), %rax
14226bf49: 48 ff c0                    	incq	%rax
14226bf4c: 49 89 00                    	movq	%rax, (%r8)
14226bf4f: 48 83 f8 02                 	cmpq	$0x2, %rax
14226bf53: 0f 86 50 07 00 00           	jbe	0x14226c6a9 <.text+0x226b6a9>
14226bf59: 4c 89 ce                    	movq	%r9, %rsi
14226bf5c: 49 89 d6                    	movq	%rdx, %r14
14226bf5f: 48 8b bd 90 02 00 00        	movq	0x290(%rbp), %rdi
14226bf66: 41 c6 40 08 00              	movb	$0x0, 0x8(%r8)
14226bf6b: 48 8b 05 36 04 3c 01        	movq	0x13c0436(%rip), %rax   # 0x14362c3a8
14226bf72: 48 8b 00                    	movq	(%rax), %rax
14226bf75: 48 83 f8 03                 	cmpq	$0x3, %rax
14226bf79: 0f 82 b8 00 00 00           	jb	0x14226c037 <.text+0x226b037>
14226bf7f: 48 8d 0d 12 97 dc 00        	leaq	0xdc9712(%rip), %rcx    # 0x143035698
14226bf86: e8 f5 1d 41 fe              	callq	0x14067dd80 <.text+0x67cd80>
14226bf8b: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14226bf8f: 8b 40 10                    	movl	0x10(%rax), %eax
14226bf92: 48 c7 85 b0 00 00 00 03 00 00 00    	movq	$0x3, 0xb0(%rbp)
14226bf9d: 48 8d 0d 0c 97 dc 00        	leaq	0xdc970c(%rip), %rcx    # 0x1430356b0
14226bfa4: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
14226bfab: 48 c7 85 c0 00 00 00 24 00 00 00    	movq	$0x24, 0xc0(%rbp)
14226bfb6: 48 8d 15 93 96 dc 00        	leaq	0xdc9693(%rip), %rdx    # 0x143035650
14226bfbd: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
14226bfc4: 48 c7 85 d8 00 00 00 01 00 00 00    	movq	$0x1, 0xd8(%rbp)
14226bfcf: 48 c7 85 e0 00 00 00 08 00 00 00    	movq	$0x8, 0xe0(%rbp)
14226bfda: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
14226bfe5: 0f 57 c9                    	xorps	%xmm1, %xmm1
14226bfe8: 0f 11 8d e8 00 00 00        	movups	%xmm1, 0xe8(%rbp)
14226bfef: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
14226bff6: 48 c7 85 90 00 00 00 24 00 00 00    	movq	$0x24, 0x90(%rbp)
14226c001: 48 c7 85 98 00 00 00 00 00 00 00    	movq	$0x0, 0x98(%rbp)
14226c00c: f3 0f 7f 85 a0 00 00 00     	movdqu	%xmm0, 0xa0(%rbp)
14226c014: c7 85 c8 00 00 00 01 00 00 00       	movl	$0x1, 0xc8(%rbp)
14226c01e: 89 85 cc 00 00 00           	movl	%eax, 0xcc(%rbp)
14226c024: 48 8d 8d 76 01 00 00        	leaq	0x176(%rbp), %rcx
14226c02b: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14226c032: e8 f9 e2 ad fe              	callq	0x140d4a330 <.text+0xd49330>
14226c037: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
14226c042: 48 c7 85 48 01 00 00 08 00 00 00    	movq	$0x8, 0x148(%rbp)
14226c04d: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
14226c058: 48 8b 0f                    	movq	(%rdi), %rcx
14226c05b: e8 20 1d 41 fe              	callq	0x14067dd80 <.text+0x67cd80>
14226c060: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
14226c064: 48 8d 50 20                 	leaq	0x20(%rax), %rdx
14226c068: 4c 8d 40 10                 	leaq	0x10(%rax), %r8
14226c06c: 49 89 c1                    	movq	%rax, %r9
14226c06f: 49 83 c1 28                 	addq	$0x28, %r9
14226c073: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
14226c07a: 48 0f 45 ca                 	cmovneq	%rdx, %rcx
14226c07e: 4d 0f 44 c8                 	cmoveq	%r8, %r9
14226c082: 49 8b 01                    	movq	(%r9), %rax
14226c085: 48 8b 09                    	movq	(%rcx), %rcx
14226c088: 48 89 4d 10                 	movq	%rcx, 0x10(%rbp)
14226c08c: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
14226c090: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14226c094: 48 89 f2                    	movq	%rsi, %rdx
14226c097: e8 84 8b 87 fe              	callq	0x140ae4c20 <.text+0xae3c20>
14226c09c: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
14226c0a3: 48 8b 08                    	movq	(%rax), %rcx
14226c0a6: e8 d5 1c 41 fe              	callq	0x14067dd80 <.text+0x67cd80>
14226c0ab: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
14226c0b2: 48 8d 50 08                 	leaq	0x8(%rax), %rdx
14226c0b6: 4c 8d 40 20                 	leaq	0x20(%rax), %r8
14226c0ba: 49 89 c1                    	movq	%rax, %r9
14226c0bd: 49 83 c1 28                 	addq	$0x28, %r9
14226c0c1: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
14226c0c8: 49 0f 45 d0                 	cmovneq	%r8, %rdx
14226c0cc: 48 8d 40 10                 	leaq	0x10(%rax), %rax
14226c0d0: 4c 0f 44 c8                 	cmoveq	%rax, %r9
14226c0d4: 49 8b 01                    	movq	(%r9), %rax
14226c0d7: 48 8b 12                    	movq	(%rdx), %rdx
14226c0da: 48 89 55 00                 	movq	%rdx, (%rbp)
14226c0de: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14226c0e2: 48 8b 09                    	movq	(%rcx), %rcx
14226c0e5: 48 8d 15 94 8f ec 00        	leaq	0xec8f94(%rip), %rdx    # 0x143135080
14226c0ec: e8 cf 38 37 00              	callq	0x1425df9c0 <.text+0x25de9c0>
14226c0f1: 48 89 c6                    	movq	%rax, %rsi
14226c0f4: 48 85 c0                    	testq	%rax, %rax
14226c0f7: 0f 84 97 09 00 00           	je	0x14226ca94 <.text+0x226ba94>
14226c0fd: 48 8d 05 b4 90 ec 00        	leaq	0xec90b4(%rip), %rax    # 0x1431351b8
14226c104: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14226c10b: 48 c7 85 08 01 00 00 3f 00 00 00    	movq	$0x3f, 0x108(%rbp)
14226c116: 48 89 f7                    	movq	%rsi, %rdi
14226c119: 48 83 c7 10                 	addq	$0x10, %rdi
14226c11d: 48 83 7e 18 3f              	cmpq	$0x3f, 0x18(%rsi)
14226c122: 0f 85 3b 09 00 00           	jne	0x14226ca63 <.text+0x226ba63>
14226c128: 48 8b 17                    	movq	(%rdi), %rdx
14226c12b: 48 8d 0d 86 90 ec 00        	leaq	0xec9086(%rip), %rcx    # 0x1431351b8
14226c132: 41 b8 3f 00 00 00           	movl	$0x3f, %r8d
14226c138: e8 b3 7a 6b 00              	callq	0x142923bf0 <.text+0x2922bf0>
14226c13d: 85 c0                       	testl	%eax, %eax
14226c13f: 0f 85 1e 09 00 00           	jne	0x14226ca63 <.text+0x226ba63>
14226c145: 48 8b 0e                    	movq	(%rsi), %rcx
14226c148: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14226c14c: ff 50 18                    	callq	*0x18(%rax)
14226c14f: 48 85 c0                    	testq	%rax, %rax
14226c152: 0f 84 3c 09 00 00           	je	0x14226ca94 <.text+0x226ba94>
14226c158: 48 8b 8d b0 02 00 00        	movq	0x2b0(%rbp), %rcx
14226c15f: 48 8b 10                    	movq	(%rax), %rdx
14226c162: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
14226c166: 48 8b 40 08                 	movq	0x8(%rax), %rax
14226c16a: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14226c16e: 48 8b 11                    	movq	(%rcx), %rdx
14226c171: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14226c178: e8 83 ef 7a fe              	callq	0x140a1b100 <.text+0xa1a100>
14226c17d: 48 8b 85 a0 02 00 00        	movq	0x2a0(%rbp), %rax
14226c184: 48 8b 8d b8 02 00 00        	movq	0x2b8(%rbp), %rcx
14226c18b: 48 8b b5 80 00 00 00        	movq	0x80(%rbp), %rsi
14226c192: 48 8b bd 88 00 00 00        	movq	0x88(%rbp), %rdi
14226c199: 44 0f b7 ad 98 00 00 00     	movzwl	0x98(%rbp), %r13d
14226c1a1: 4c 8b a5 a0 00 00 00        	movq	0xa0(%rbp), %r12
14226c1a8: 48 8b 19                    	movq	(%rcx), %rbx
14226c1ab: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
14226c1af: 48 8b 00                    	movq	(%rax), %rax
14226c1b2: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14226c1b6: 41 8b 46 1c                 	movl	0x1c(%r14), %eax
14226c1ba: 89 85 2c 01 00 00           	movl	%eax, 0x12c(%rbp)
14226c1c0: 49 8b 06                    	movq	(%r14), %rax
14226c1c3: 4d 8b 76 10                 	movq	0x10(%r14), %r14
14226c1c7: f3 0f 10 35 89 f4 80 00     	movss	0x80f489(%rip), %xmm6   # 0x142a7b658
14226c1cf: 45 0f 57 ff                 	xorps	%xmm15, %xmm15
14226c1d3: f3 0f 10 3d 6d 0b 81 00     	movss	0x810b6d(%rip), %xmm7   # 0x142a7cd48
14226c1db: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
14226c1e2: 48 83 c0 20                 	addq	$0x20, %rax
14226c1e6: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14226c1ea: 44 89 ea                    	movl	%r13d, %edx
14226c1ed: 0f 1f 00                    	nopl	(%rax)
14226c1f0: 4d 85 e4                    	testq	%r12, %r12
14226c1f3: 0f 84 12 05 00 00           	je	0x14226c70b <.text+0x226b70b>
14226c1f9: 66 85 d2                    	testw	%dx, %dx
14226c1fc: 74 12                       	je	0x14226c210 <.text+0x226b210>
14226c1fe: 48 85 f6                    	testq	%rsi, %rsi
14226c201: 0f 84 04 05 00 00           	je	0x14226c70b <.text+0x226b70b>
14226c207: 44 8d 6a ff                 	leal	-0x1(%rdx), %r13d
14226c20b: 41 21 d5                    	andl	%edx, %r13d
14226c20e: eb 2a                       	jmp	0x14226c23a <.text+0x226b23a>
14226c210: 66 0f 6f 07                 	movdqa	(%rdi), %xmm0
14226c214: 66 0f d7 d0                 	pmovmskb	%xmm0, %edx
14226c218: 48 81 c6 00 ff ff ff        	addq	$-0x100, %rsi
14226c21f: 48 83 c7 10                 	addq	$0x10, %rdi
14226c223: 81 fa ff ff 00 00           	cmpl	$0xffff, %edx           # imm = 0xFFFF
14226c229: 74 e5                       	je	0x14226c210 <.text+0x226b210>
14226c22b: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
14226c230: 29 d0                       	subl	%edx, %eax
14226c232: f7 d2                       	notl	%edx
14226c234: 41 89 d5                    	movl	%edx, %r13d
14226c237: 41 21 c5                    	andl	%eax, %r13d
14226c23a: 49 ff cc                    	decq	%r12
14226c23d: f3 0f bc ca                 	tzcntl	%edx, %ecx
14226c241: c1 e1 04                    	shll	$0x4, %ecx
14226c244: 48 89 f0                    	movq	%rsi, %rax
14226c247: 48 29 c8                    	subq	%rcx, %rax
14226c24a: 48 8b 48 f8                 	movq	-0x8(%rax), %rcx
14226c24e: 41 89 c8                    	movl	%ecx, %r8d
14226c251: 44 89 ea                    	movl	%r13d, %edx
14226c254: 4d 3b 47 10                 	cmpq	0x10(%r15), %r8
14226c258: 73 96                       	jae	0x14226c1f0 <.text+0x226b1f0>
14226c25a: 4d 8b 4f 08                 	movq	0x8(%r15), %r9
14226c25e: 48 c1 e9 20                 	shrq	$0x20, %rcx
14226c262: 4f 8d 04 80                 	leaq	(%r8,%r8,4), %r8
14226c266: 44 89 ea                    	movl	%r13d, %edx
14226c269: 43 39 0c 81                 	cmpl	%ecx, (%r9,%r8,4)
14226c26d: 75 81                       	jne	0x14226c1f0 <.text+0x226b1f0>
14226c26f: 4b 8d 0c 81                 	leaq	(%r9,%r8,4), %rcx
14226c273: 44 8b 41 04                 	movl	0x4(%rcx), %r8d
14226c277: 44 89 ea                    	movl	%r13d, %edx
14226c27a: 41 b9 ff ff ff ff           	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
14226c280: 4d 39 c8                    	cmpq	%r9, %r8
14226c283: 0f 84 67 ff ff ff           	je	0x14226c1f0 <.text+0x226b1f0>
14226c289: 44 89 ea                    	movl	%r13d, %edx
14226c28c: 4c 39 43 38                 	cmpq	%r8, 0x38(%rbx)
14226c290: 0f 86 5a ff ff ff           	jbe	0x14226c1f0 <.text+0x226b1f0>
14226c296: 48 8b 53 28                 	movq	0x28(%rbx), %rdx
14226c29a: 45 89 c1                    	movl	%r8d, %r9d
14226c29d: 41 c1 e9 06                 	shrl	$0x6, %r9d
14226c2a1: 4a 8b 14 ca                 	movq	(%rdx,%r9,8), %rdx
14226c2a5: 4c 0f a3 c2                 	btq	%r8, %rdx
14226c2a9: 44 89 ea                    	movl	%r13d, %edx
14226c2ac: 0f 83 3e ff ff ff           	jae	0x14226c1f0 <.text+0x226b1f0>
14226c2b2: 8b 51 0c                    	movl	0xc(%rcx), %edx
14226c2b5: 4d 8b 87 a8 01 00 00        	movq	0x1a8(%r15), %r8
14226c2bc: 4c 8d 14 d2                 	leaq	(%rdx,%rdx,8), %r10
14226c2c0: 4b 8d 14 d0                 	leaq	(%r8,%r10,8), %rdx
14226c2c4: 4c 8b 8b 10 01 00 00        	movq	0x110(%rbx), %r9
14226c2cb: 4f 8b 44 d0 40              	movq	0x40(%r8,%r10,8), %r8
14226c2d0: 4d 39 c8                    	cmpq	%r9, %r8
14226c2d3: 76 23                       	jbe	0x14226c2f8 <.text+0x226b2f8>
14226c2d5: 4c 8b 52 38                 	movq	0x38(%rdx), %r10
14226c2d9: 4f 8b 0c ca                 	movq	(%r10,%r9,8), %r9
14226c2dd: 4d 85 c9                    	testq	%r9, %r9
14226c2e0: 74 16                       	je	0x14226c2f8 <.text+0x226b2f8>
14226c2e2: 4c 8b 52 18                 	movq	0x18(%rdx), %r10
14226c2e6: 49 f7 d1                    	notq	%r9
14226c2e9: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
14226c2ed: 49 c1 e1 04                 	shlq	$0x4, %r9
14226c2f1: 4f 8b 54 0a 10              	movq	0x10(%r10,%r9), %r10
14226c2f6: eb 03                       	jmp	0x14226c2fb <.text+0x226b2fb>
14226c2f8: 45 31 d2                    	xorl	%r10d, %r10d
14226c2fb: 4c 8b 8b 18 01 00 00        	movq	0x118(%rbx), %r9
14226c302: 4d 39 c8                    	cmpq	%r9, %r8
14226c305: 4c 89 b5 30 01 00 00        	movq	%r14, 0x130(%rbp)
14226c30c: 76 2a                       	jbe	0x14226c338 <.text+0x226b338>
14226c30e: 4c 8b 42 38                 	movq	0x38(%rdx), %r8
14226c312: 4f 8b 04 c8                 	movq	(%r8,%r9,8), %r8
14226c316: 4d 85 c0                    	testq	%r8, %r8
14226c319: 74 1d                       	je	0x14226c338 <.text+0x226b338>
14226c31b: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14226c31f: 49 f7 d0                    	notq	%r8
14226c322: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14226c326: 49 c1 e0 04                 	shlq	$0x4, %r8
14226c32a: 4a 8b 54 02 10              	movq	0x10(%rdx,%r8), %rdx
14226c32f: 48 89 95 68 01 00 00        	movq	%rdx, 0x168(%rbp)
14226c336: eb 0b                       	jmp	0x14226c343 <.text+0x226b343>
14226c338: 48 c7 85 68 01 00 00 00 00 00 00    	movq	$0x0, 0x168(%rbp)
14226c343: 48 8b 40 f0                 	movq	-0x10(%rax), %rax
14226c347: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14226c34b: 8b 41 10                    	movl	0x10(%rcx), %eax
14226c34e: 48 89 85 38 01 00 00        	movq	%rax, 0x138(%rbp)
14226c355: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14226c359: 48 c1 e0 04                 	shlq	$0x4, %rax
14226c35d: 4d 89 d6                    	movq	%r10, %r14
14226c360: 49 01 c6                    	addq	%rax, %r14
14226c363: 48 8d 4d 24                 	leaq	0x24(%rbp), %rcx
14226c367: 4c 89 f2                    	movq	%r14, %rdx
14226c36a: e8 31 8e 86 fe              	callq	0x140ad51a0 <.text+0xad41a0>
14226c36f: f3 44 0f 10 45 24           	movss	0x24(%rbp), %xmm8
14226c375: f3 44 0f 10 4d 2c           	movss	0x2c(%rbp), %xmm9
14226c37b: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c383: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14226c387: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14226c38b: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14226c38f: 0f 28 de                    	movaps	%xmm6, %xmm3
14226c392: e8 59 8a 7a fe              	callq	0x140a14df0 <.text+0xa13df0>
14226c397: 66 44 0f 6f d0              	movdqa	%xmm0, %xmm10
14226c39c: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c3a4: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14226c3a8: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14226c3ac: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14226c3b0: 0f 28 de                    	movaps	%xmm6, %xmm3
14226c3b3: e8 38 8a 7a fe              	callq	0x140a14df0 <.text+0xa13df0>
14226c3b8: 66 44 0f 6f d8              	movdqa	%xmm0, %xmm11
14226c3bd: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c3c5: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
14226c3c9: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14226c3cd: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14226c3d1: 0f 28 de                    	movaps	%xmm6, %xmm3
14226c3d4: e8 17 8a 7a fe              	callq	0x140a14df0 <.text+0xa13df0>
14226c3d9: 66 44 0f 6f e0              	movdqa	%xmm0, %xmm12
14226c3de: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c3e6: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
14226c3ea: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14226c3ee: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14226c3f2: 0f 28 de                    	movaps	%xmm6, %xmm3
14226c3f5: e8 f6 89 7a fe              	callq	0x140a14df0 <.text+0xa13df0>
14226c3fa: 66 44 0f 6f e8              	movdqa	%xmm0, %xmm13
14226c3ff: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c407: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14226c40b: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14226c40f: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14226c413: 0f 28 de                    	movaps	%xmm6, %xmm3
14226c416: e8 d5 89 7a fe              	callq	0x140a14df0 <.text+0xa13df0>
14226c41b: 66 44 0f 6f f0              	movdqa	%xmm0, %xmm14
14226c420: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
14226c428: 48 89 e9                    	movq	%rbp, %rcx
14226c42b: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14226c42f: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14226c433: 0f 28 de                    	movaps	%xmm6, %xmm3
14226c436: e8 b5 89 7a fe              	callq	0x140a14df0 <.text+0xa13df0>
14226c43b: 45 0f 2e f7                 	ucomiss	%xmm15, %xmm14
14226c43f: 77 1f                       	ja	0x14226c460 <.text+0x226b460>
14226c441: 41 0f 2e c7                 	ucomiss	%xmm15, %xmm0
14226c445: 77 19                       	ja	0x14226c460 <.text+0x226b460>
14226c447: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
14226c44b: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14226c44f: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
14226c453: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
14226c457: e8 a4 e3 77 fe              	callq	0x1409ea800 <.text+0x9e9800>
14226c45c: 84 c0                       	testb	%al, %al
14226c45e: 74 7c                       	je	0x14226c4dc <.text+0x226b4dc>
14226c460: c6 45 68 00                 	movb	$0x0, 0x68(%rbp)
14226c464: 8b 85 2c 01 00 00           	movl	0x12c(%rbp), %eax
14226c46a: 48 8b 8d 30 01 00 00        	movq	0x130(%rbp), %rcx
14226c471: 89 01                       	movl	%eax, (%rcx)
14226c473: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14226c47a: 4c 8b 70 30                 	movq	0x30(%rax), %r14
14226c47e: 48 8b 48 40                 	movq	0x40(%rax), %rcx
14226c482: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
14226c489: 4c 3b 70 20                 	cmpq	0x20(%rax), %r14
14226c48d: 75 10                       	jne	0x14226c49f <.text+0x226b49f>
14226c48f: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14226c493: 48 8d 15 76 88 dc 00        	leaq	0xdc8876(%rip), %rdx    # 0x143034d10
14226c49a: e8 91 61 2f fe              	callq	0x140562630 <.text+0x561630>
14226c49f: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
14226c4a6: 48 8b 42 28                 	movq	0x28(%rdx), %rax
14226c4aa: 4c 89 f1                    	movq	%r14, %rcx
14226c4ad: 48 c1 e1 04                 	shlq	$0x4, %rcx
14226c4b1: 4c 8b 85 68 01 00 00        	movq	0x168(%rbp), %r8
14226c4b8: 4c 89 04 08                 	movq	%r8, (%rax,%rcx)
14226c4bc: 4c 8b 45 60                 	movq	0x60(%rbp), %r8
14226c4c0: 4c 89 44 08 08              	movq	%r8, 0x8(%rax,%rcx)
14226c4c5: 49 ff c6                    	incq	%r14
14226c4c8: 4c 89 72 30                 	movq	%r14, 0x30(%rdx)
14226c4cc: 48 ff 42 40                 	incq	0x40(%rdx)
14226c4d0: 4c 8b b5 30 01 00 00        	movq	0x130(%rbp), %r14
14226c4d7: e9 0e fd ff ff              	jmp	0x14226c1ea <.text+0x226b1ea>
14226c4dc: f3 45 0f 58 d3              	addss	%xmm11, %xmm10
14226c4e1: f3 45 0f 58 d4              	addss	%xmm12, %xmm10
14226c4e6: f3 45 0f 58 d5              	addss	%xmm13, %xmm10
14226c4eb: 0f 57 c0                    	xorps	%xmm0, %xmm0
14226c4ee: f3 41 0f 5f c2              	maxss	%xmm10, %xmm0
14226c4f3: f3 0f 10 0d 09 94 6b 00     	movss	0x6b9409(%rip), %xmm1   # 0x142925904
14226c4fb: f3 0f 5d c8                 	minss	%xmm0, %xmm1
14226c4ff: 41 0f 2e cf                 	ucomiss	%xmm15, %xmm1
14226c503: 0f 96 45 68                 	setbe	0x68(%rbp)
14226c507: 0f 87 57 ff ff ff           	ja	0x14226c464 <.text+0x226b464>
14226c50d: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
14226c511: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14226c515: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
14226c519: 0f 28 df                    	movaps	%xmm7, %xmm3
14226c51c: e8 1f b0 a2 fe              	callq	0x140c97540 <.text+0xc96540>
14226c521: 48 8b 4d 50                 	movq	0x50(%rbp), %rcx
14226c525: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
14226c529: 4c 8d 45 68                 	leaq	0x68(%rbp), %r8
14226c52d: e8 9e 2a 37 00              	callq	0x1425defd0 <.text+0x25ddfd0>
14226c532: f6 45 68 01                 	testb	$0x1, 0x68(%rbp)
14226c536: 0f 84 28 ff ff ff           	je	0x14226c464 <.text+0x226b464>
14226c53c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14226c543: 4c 89 f2                    	movq	%r14, %rdx
14226c546: e8 55 8c 86 fe              	callq	0x140ad51a0 <.text+0xad41a0>
14226c54b: 48 8b 85 38 01 00 00        	movq	0x138(%rbp), %rax
14226c552: 48 c1 e0 04                 	shlq	$0x4, %rax
14226c556: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14226c55d: 48 01 c1                    	addq	%rax, %rcx
14226c560: f2 45 0f 10 46 0c           	movsd	0xc(%r14), %xmm8
14226c566: 45 8b 76 14                 	movl	0x14(%r14), %r14d
14226c56a: e8 c1 9b 2e ff              	callq	0x141556130 <.text+0x1555130>
14226c56f: 49 89 d0                    	movq	%rdx, %r8
14226c572: 44 89 b5 38 01 00 00        	movl	%r14d, 0x138(%rbp)
14226c579: 48 85 d2                    	testq	%rdx, %rdx
14226c57c: 0f 88 50 05 00 00           	js	0x14226cad2 <.text+0x226bad2>
14226c582: 49 89 c1                    	movq	%rax, %r9
14226c585: 4d 85 c0                    	testq	%r8, %r8
14226c588: 4c 8b b5 30 01 00 00        	movq	0x130(%rbp), %r14
14226c58f: 4c 89 85 68 01 00 00        	movq	%r8, 0x168(%rbp)
14226c596: 74 34                       	je	0x14226c5cc <.text+0x226b5cc>
14226c598: 4c 89 8d 60 01 00 00        	movq	%r9, 0x160(%rbp)
14226c59f: 0f b6 05 63 86 3c 01        	movzbl	0x13c8663(%rip), %eax   # 0x143634c09
14226c5a6: ba 01 00 00 00              	movl	$0x1, %edx
14226c5ab: 4c 89 c1                    	movq	%r8, %rcx
14226c5ae: e8 5d 76 3a fe              	callq	0x140613c10 <.text+0x612c10>
14226c5b3: 48 85 c0                    	testq	%rax, %rax
14226c5b6: 0f 84 1a 05 00 00           	je	0x14226cad6 <.text+0x226bad6>
14226c5bc: 4c 8b 85 68 01 00 00        	movq	0x168(%rbp), %r8
14226c5c3: 4c 8b 8d 60 01 00 00        	movq	0x160(%rbp), %r9
14226c5ca: eb 05                       	jmp	0x14226c5d1 <.text+0x226b5d1>
14226c5cc: b8 01 00 00 00              	movl	$0x1, %eax
14226c5d1: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14226c5d8: 48 89 c1                    	movq	%rax, %rcx
14226c5db: 4c 89 ca                    	movq	%r9, %rdx
14226c5de: e8 1d 76 6b 00              	callq	0x142923c00 <.text+0x2922c00>
14226c5e3: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
14226c5ea: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
14226c5f1: 8b 85 88 00 00 00           	movl	0x88(%rbp), %eax
14226c5f7: 89 85 08 01 00 00           	movl	%eax, 0x108(%rbp)
14226c5fd: 48 8b 85 50 01 00 00        	movq	0x150(%rbp), %rax
14226c604: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14226c608: 48 3b 85 40 01 00 00        	cmpq	0x140(%rbp), %rax
14226c60f: 75 13                       	jne	0x14226c624 <.text+0x226b624>
14226c611: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
14226c618: 48 8d 15 39 91 dc 00        	leaq	0xdc9139(%rip), %rdx    # 0x143035758
14226c61f: e8 cc 33 8f fe              	callq	0x140b5f9f0 <.text+0xb5e9f0>
14226c624: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
14226c62b: 4c 8b 4d 58                 	movq	0x58(%rbp), %r9
14226c62f: 4b 8d 0c c9                 	leaq	(%r9,%r9,8), %rcx
14226c633: c7 04 c8 00 00 00 00        	movl	$0x0, (%rax,%rcx,8)
14226c63a: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
14226c63e: 48 89 54 c8 08              	movq	%rdx, 0x8(%rax,%rcx,8)
14226c643: 4c 8b 85 68 01 00 00        	movq	0x168(%rbp), %r8
14226c64a: 4c 89 44 c8 10              	movq	%r8, 0x10(%rax,%rcx,8)
14226c64f: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
14226c656: 48 89 54 c8 18              	movq	%rdx, 0x18(%rax,%rcx,8)
14226c65b: 4c 89 44 c8 20              	movq	%r8, 0x20(%rax,%rcx,8)
14226c660: 48 8b 95 00 01 00 00        	movq	0x100(%rbp), %rdx
14226c667: 48 89 54 c8 28              	movq	%rdx, 0x28(%rax,%rcx,8)
14226c66c: 8b 95 08 01 00 00           	movl	0x108(%rbp), %edx
14226c672: 89 54 c8 30                 	movl	%edx, 0x30(%rax,%rcx,8)
14226c676: 44 0f 13 44 c8 34           	movlps	%xmm8, 0x34(%rax,%rcx,8)
14226c67c: 8b 95 38 01 00 00           	movl	0x138(%rbp), %edx
14226c682: 89 54 c8 3c                 	movl	%edx, 0x3c(%rax,%rcx,8)
14226c686: c6 44 c8 40 01              	movb	$0x1, 0x40(%rax,%rcx,8)
14226c68b: 49 ff c1                    	incq	%r9
14226c68e: 4c 89 8d 50 01 00 00        	movq	%r9, 0x150(%rbp)
14226c695: e9 50 fb ff ff              	jmp	0x14226c1ea <.text+0x226b1ea>
14226c69a: 48 89 08                    	movq	%rcx, (%rax)
14226c69d: 41 c6 40 08 01              	movb	$0x1, 0x8(%r8)
14226c6a2: 49 c7 00 01 00 00 00        	movq	$0x1, (%r8)
14226c6a9: 0f 28 b5 80 01 00 00        	movaps	0x180(%rbp), %xmm6
14226c6b0: 0f 28 bd 90 01 00 00        	movaps	0x190(%rbp), %xmm7
14226c6b7: 44 0f 28 85 a0 01 00 00     	movaps	0x1a0(%rbp), %xmm8
14226c6bf: 44 0f 28 8d b0 01 00 00     	movaps	0x1b0(%rbp), %xmm9
14226c6c7: 44 0f 28 95 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm10
14226c6cf: 44 0f 28 9d d0 01 00 00     	movaps	0x1d0(%rbp), %xmm11
14226c6d7: 44 0f 28 a5 e0 01 00 00     	movaps	0x1e0(%rbp), %xmm12
14226c6df: 44 0f 28 ad f0 01 00 00     	movaps	0x1f0(%rbp), %xmm13
14226c6e7: 44 0f 28 b5 00 02 00 00     	movaps	0x200(%rbp), %xmm14
14226c6ef: 44 0f 28 bd 10 02 00 00     	movaps	0x210(%rbp), %xmm15
14226c6f7: 48 81 c4 a8 02 00 00        	addq	$0x2a8, %rsp            # imm = 0x2A8
14226c6fe: 5b                          	popq	%rbx
14226c6ff: 5f                          	popq	%rdi
14226c700: 5e                          	popq	%rsi
14226c701: 41 5c                       	popq	%r12
14226c703: 41 5d                       	popq	%r13
14226c705: 41 5e                       	popq	%r14
14226c707: 41 5f                       	popq	%r15
14226c709: 5d                          	popq	%rbp
14226c70a: c3                          	retq
14226c70b: 48 8b 85 48 01 00 00        	movq	0x148(%rbp), %rax
14226c712: 48 8b 8d 50 01 00 00        	movq	0x150(%rbp), %rcx
14226c719: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
14226c724: 48 8d 14 c9                 	leaq	(%rcx,%rcx,8), %rdx
14226c728: 48 8d 14 d0                 	leaq	(%rax,%rdx,8), %rdx
14226c72c: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
14226c733: 48 89 95 88 00 00 00        	movq	%rdx, 0x88(%rbp)
14226c73a: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
14226c741: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14226c748: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
14226c74f: 48 c7 85 a0 00 00 00 00 00 00 00    	movq	$0x0, 0xa0(%rbp)
14226c75a: 4c 8d 05 3f 8e dc 00        	leaq	0xdc8e3f(%rip), %r8     # 0x1430355a0
14226c761: 48 8d 75 68                 	leaq	0x68(%rbp), %rsi
14226c765: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14226c76c: 48 89 f1                    	movq	%rsi, %rcx
14226c76f: e8 6c f0 96 ff              	callq	0x141bdb7e0 <.text+0x1bda7e0>
14226c774: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
14226c77f: 48 c7 85 88 00 00 00 08 00 00 00    	movq	$0x8, 0x88(%rbp)
14226c78a: 48 c7 85 90 00 00 00 00 00 00 00    	movq	$0x0, 0x90(%rbp)
14226c795: 0f b6 05 6d 84 3c 01        	movzbl	0x13c846d(%rip), %eax   # 0x143634c09
14226c79c: b9 40 00 00 00              	movl	$0x40, %ecx
14226c7a1: ba 08 00 00 00              	movl	$0x8, %edx
14226c7a6: e8 65 74 3a fe              	callq	0x140613c10 <.text+0x612c10>
14226c7ab: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
14226c7b2: 48 85 c0                    	testq	%rax, %rax
14226c7b5: 0f 84 e7 02 00 00           	je	0x14226caa2 <.text+0x226baa2>
14226c7bb: 31 c9                       	xorl	%ecx, %ecx
14226c7bd: e8 5e d3 e6 fd              	callq	0x1400d9b20 <.text+0xd8b20>
14226c7c2: 48 85 c0                    	testq	%rax, %rax
14226c7c5: 0f 84 e8 02 00 00           	je	0x14226cab3 <.text+0x226bab3>
14226c7cb: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14226c7cf: 48 ff 00                    	incq	(%rax)
14226c7d2: 48 8b bd 58 01 00 00        	movq	0x158(%rbp), %rdi
14226c7d9: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
14226c7e0: 48 c7 47 08 01 00 00 00     	movq	$0x1, 0x8(%rdi)
14226c7e8: 0f 10 0d 21 8a ec 00        	movups	0xec8a21(%rip), %xmm1   # 0x143135210
14226c7ef: 0f 11 4f 10                 	movups	%xmm1, 0x10(%rdi)
14226c7f3: 0f 10 0d 26 8a ec 00        	movups	0xec8a26(%rip), %xmm1   # 0x143135220
14226c7fa: 0f 11 4f 20                 	movups	%xmm1, 0x20(%rdi)
14226c7fe: f3 0f 7f 47 30              	movdqu	%xmm0, 0x30(%rdi)
14226c803: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
14226c80a: 0f b6 05 f8 83 3c 01        	movzbl	0x13c83f8(%rip), %eax   # 0x143634c09
14226c811: b9 60 00 00 00              	movl	$0x60, %ecx
14226c816: ba 08 00 00 00              	movl	$0x8, %edx
14226c81b: e8 f0 73 3a fe              	callq	0x140613c10 <.text+0x612c10>
14226c820: 48 85 c0                    	testq	%rax, %rax
14226c823: 0f 84 98 02 00 00           	je	0x14226cac1 <.text+0x226bac1>
14226c829: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
14226c830: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
14226c838: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
14226c840: 48 c7 40 20 00 00 00 00     	movq	$0x0, 0x20(%rax)
14226c848: 48 c7 40 30 00 00 00 00     	movq	$0x0, 0x30(%rax)
14226c850: 48 c7 40 38 04 00 00 00     	movq	$0x4, 0x38(%rax)
14226c858: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14226c85c: f3 0f 7f 40 40              	movdqu	%xmm0, 0x40(%rax)
14226c861: 48 c7 40 50 04 00 00 00     	movq	$0x4, 0x50(%rax)
14226c869: 48 c7 40 58 00 00 00 00     	movq	$0x0, 0x58(%rax)
14226c871: f3 0f 6f 45 68              	movdqu	0x68(%rbp), %xmm0
14226c876: 66 0f 7f 85 00 01 00 00     	movdqa	%xmm0, 0x100(%rbp)
14226c87e: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14226c882: 48 89 8d 10 01 00 00        	movq	%rcx, 0x110(%rbp)
14226c889: 48 89 bd 18 01 00 00        	movq	%rdi, 0x118(%rbp)
14226c890: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
14226c897: 48 8b 05 0a fb 3b 01        	movq	0x13bfb0a(%rip), %rax   # 0x14362c3a8
14226c89e: 48 8b 00                    	movq	(%rax), %rax
14226c8a1: 48 83 f8 03                 	cmpq	$0x3, %rax
14226c8a5: 0f 82 ec 00 00 00           	jb	0x14226c997 <.text+0x226b997>
14226c8ab: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
14226c8b2: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
14226c8b6: 48 8d 45 30                 	leaq	0x30(%rbp), %rax
14226c8ba: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14226c8be: 48 8d 05 fb 0d b2 fe        	leaq	-0x14df205(%rip), %rax  # 0x140d8d6c0 <.text+0xd8c6c0>
14226c8c5: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14226c8c9: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
14226c8d0: 48 8d 0d 69 8e dc 00        	leaq	0xdc8e69(%rip), %rcx    # 0x143035740
14226c8d7: e8 a4 14 41 fe              	callq	0x14067dd80 <.text+0x67cd80>
14226c8dc: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14226c8e0: 8b 40 10                    	movl	0x10(%rax), %eax
14226c8e3: 48 c7 85 b0 00 00 00 03 00 00 00    	movq	$0x3, 0xb0(%rbp)
14226c8ee: 48 8d 0d bb 8d dc 00        	leaq	0xdc8dbb(%rip), %rcx    # 0x1430356b0
14226c8f5: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
14226c8fc: 48 c7 85 c0 00 00 00 24 00 00 00    	movq	$0x24, 0xc0(%rbp)
14226c907: 48 8d 15 12 8e dc 00        	leaq	0xdc8e12(%rip), %rdx    # 0x143035720
14226c90e: 48 89 95 d0 00 00 00        	movq	%rdx, 0xd0(%rbp)
14226c915: 48 c7 85 d8 00 00 00 02 00 00 00    	movq	$0x2, 0xd8(%rbp)
14226c920: 48 89 b5 e0 00 00 00        	movq	%rsi, 0xe0(%rbp)
14226c927: 48 c7 85 e8 00 00 00 01 00 00 00    	movq	$0x1, 0xe8(%rbp)
14226c932: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
14226c93d: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
14226c948: 48 89 8d 88 00 00 00        	movq	%rcx, 0x88(%rbp)
14226c94f: 48 c7 85 90 00 00 00 24 00 00 00    	movq	$0x24, 0x90(%rbp)
14226c95a: 48 c7 85 98 00 00 00 00 00 00 00    	movq	$0x0, 0x98(%rbp)
14226c965: f3 0f 7f 85 a0 00 00 00     	movdqu	%xmm0, 0xa0(%rbp)
14226c96d: c7 85 c8 00 00 00 01 00 00 00       	movl	$0x1, 0xc8(%rbp)
14226c977: 89 85 cc 00 00 00           	movl	%eax, 0xcc(%rbp)
14226c97d: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
14226c984: 48 8d 8d 76 01 00 00        	leaq	0x176(%rbp), %rcx
14226c98b: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14226c992: e8 99 d9 ad fe              	callq	0x140d4a330 <.text+0xd49330>
14226c997: 48 8b 95 c0 02 00 00        	movq	0x2c0(%rbp), %rdx
14226c99e: 8b 42 1c                    	movl	0x1c(%rdx), %eax
14226c9a1: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14226c9a5: 89 01                       	movl	%eax, (%rcx)
14226c9a7: 48 8b 0a                    	movq	(%rdx), %rcx
14226c9aa: 48 8b 85 20 01 00 00        	movq	0x120(%rbp), %rax
14226c9b1: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
14226c9b8: 66 0f 6f 85 00 01 00 00     	movdqa	0x100(%rbp), %xmm0
14226c9c0: 48 8b 85 10 01 00 00        	movq	0x110(%rbp), %rax
14226c9c7: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14226c9ce: 48 8b 85 18 01 00 00        	movq	0x118(%rbp), %rax
14226c9d5: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
14226c9dc: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
14226c9e4: c6 85 77 01 00 00 00        	movb	$0x0, 0x177(%rbp)
14226c9eb: 48 8d 95 80 00 00 00        	leaq	0x80(%rbp), %rdx
14226c9f2: e8 f9 7a 81 fe              	callq	0x140a844f0 <.text+0xa834f0>
14226c9f7: 48 8b b5 48 01 00 00        	movq	0x148(%rbp), %rsi
14226c9fe: 48 8b bd 50 01 00 00        	movq	0x150(%rbp), %rdi
14226ca05: 48 85 ff                    	testq	%rdi, %rdi
14226ca08: 74 2e                       	je	0x14226ca38 <.text+0x226ba38>
14226ca0a: 48 8d 5e 18                 	leaq	0x18(%rsi), %rbx
14226ca0e: eb 09                       	jmp	0x14226ca19 <.text+0x226ba19>
14226ca10: 48 83 c3 48                 	addq	$0x48, %rbx
14226ca14: 48 ff cf                    	decq	%rdi
14226ca17: 74 1f                       	je	0x14226ca38 <.text+0x226ba38>
14226ca19: 83 7b e8 01                 	cmpl	$0x1, -0x18(%rbx)
14226ca1d: 74 f1                       	je	0x14226ca10 <.text+0x226ba10>
14226ca1f: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
14226ca23: 48 85 d2                    	testq	%rdx, %rdx
14226ca26: 74 e8                       	je	0x14226ca10 <.text+0x226ba10>
14226ca28: 48 8b 0b                    	movq	(%rbx), %rcx
14226ca2b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14226ca31: e8 ea 71 3a fe              	callq	0x140613c20 <.text+0x612c20>
14226ca36: eb d8                       	jmp	0x14226ca10 <.text+0x226ba10>
14226ca38: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
14226ca3f: 48 85 c0                    	testq	%rax, %rax
14226ca42: 0f 84 61 fc ff ff           	je	0x14226c6a9 <.text+0x226b6a9>
14226ca48: 48 c1 e0 03                 	shlq	$0x3, %rax
14226ca4c: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
14226ca50: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14226ca56: 48 89 f1                    	movq	%rsi, %rcx
14226ca59: e8 c2 71 3a fe              	callq	0x140613c20 <.text+0x612c20>
14226ca5e: e9 46 fc ff ff              	jmp	0x14226c6a9 <.text+0x226b6a9>
14226ca63: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
14226ca6e: 48 8d 05 7b 86 ec 00        	leaq	0xec867b(%rip), %rax    # 0x1431350f0
14226ca75: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14226ca7a: 48 8d 95 00 01 00 00        	leaq	0x100(%rbp), %rdx
14226ca81: 4c 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %r9
14226ca88: 31 c9                       	xorl	%ecx, %ecx
14226ca8a: 49 89 f8                    	movq	%rdi, %r8
14226ca8d: e8 af e0 65 00              	callq	0x1428cab41 <.text+0x28c9b41>
14226ca92: eb 68                       	jmp	0x14226cafc <.text+0x226bafc>
14226ca94: 48 8d 0d 3d 8c dc 00        	leaq	0xdc8c3d(%rip), %rcx    # 0x1430356d8
14226ca9b: e8 70 c8 66 00              	callq	0x1428d9310 <.text+0x28d8310>
14226caa0: eb 5a                       	jmp	0x14226cafc <.text+0x226bafc>
14226caa2: b9 08 00 00 00              	movl	$0x8, %ecx
14226caa7: ba 40 00 00 00              	movl	$0x40, %edx
14226caac: e8 4f c5 66 00              	callq	0x1428d9000 <.text+0x28d8000>
14226cab1: eb 49                       	jmp	0x14226cafc <.text+0x226bafc>
14226cab3: 48 8d 0d c6 7f dc 00        	leaq	0xdc7fc6(%rip), %rcx    # 0x143034a80
14226caba: e8 11 b6 66 00              	callq	0x1428d80d0 <.text+0x28d70d0>
14226cabf: eb 3b                       	jmp	0x14226cafc <.text+0x226bafc>
14226cac1: b9 08 00 00 00              	movl	$0x8, %ecx
14226cac6: ba 60 00 00 00              	movl	$0x60, %edx
14226cacb: e8 30 c5 66 00              	callq	0x1428d9000 <.text+0x28d8000>
14226cad0: eb 2a                       	jmp	0x14226cafc <.text+0x226bafc>
14226cad2: 31 c9                       	xorl	%ecx, %ecx
14226cad4: eb 13                       	jmp	0x14226cae9 <.text+0x226bae9>
14226cad6: b9 01 00 00 00              	movl	$0x1, %ecx
14226cadb: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14226cae2: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14226cae9: 4c 8d 05 18 8b dc 00        	leaq	0xdc8b18(%rip), %r8     # 0x143035608
14226caf0: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
14226caf7: e8 e7 c4 66 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14226cafc: 0f 0b                       	ud2
14226cafe: 66 90                       	nop
14226cb00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14226cb05: 55                          	pushq	%rbp
14226cb06: 41 57                       	pushq	%r15
14226cb08: 41 56                       	pushq	%r14
14226cb0a: 41 55                       	pushq	%r13
14226cb0c: 41 54                       	pushq	%r12
14226cb0e: 56                          	pushq	%rsi
14226cb0f: 57                          	pushq	%rdi
14226cb10: 53                          	pushq	%rbx
14226cb11: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14226cb18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14226cb1f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14226cb25: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14226cb2b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14226cb31: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14226cb37: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14226cb3d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14226cb43: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14226cb4c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14226cb55: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14226cb5d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14226cb65: 48 83 bd 68 01 00 00 00     	cmpq	$0x0, 0x168(%rbp)
14226cb6d: 74 19                       	je	0x14226cb88 <.text+0x226bb88>
14226cb6f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14226cb75: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
14226cb7c: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
14226cb83: e8 98 70 3a fe              	callq	0x140613c20 <.text+0x612c20>
14226cb88: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226cb90: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226cb98: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226cba1: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226cbaa: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226cbb0: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226cbb6: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226cbbc: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226cbc2: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226cbc8: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226cbce: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226cbd5: 5b                          	popq	%rbx
14226cbd6: 5f                          	popq	%rdi
14226cbd7: 5e                          	popq	%rsi
14226cbd8: 41 5c                       	popq	%r12
14226cbda: 41 5d                       	popq	%r13
14226cbdc: 41 5e                       	popq	%r14
14226cbde: 41 5f                       	popq	%r15
14226cbe0: 5d                          	popq	%rbp
14226cbe1: c3                          	retq
14226cbe2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14226cbf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14226cbf5: 55                          	pushq	%rbp
14226cbf6: 41 57                       	pushq	%r15
14226cbf8: 41 56                       	pushq	%r14
14226cbfa: 41 55                       	pushq	%r13
14226cbfc: 41 54                       	pushq	%r12
14226cbfe: 56                          	pushq	%rsi
14226cbff: 57                          	pushq	%rdi
14226cc00: 53                          	pushq	%rbx
14226cc01: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14226cc08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14226cc0f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14226cc15: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14226cc1b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14226cc21: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14226cc27: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14226cc2d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14226cc33: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14226cc3c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14226cc45: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14226cc4d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14226cc55: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
14226cc5c: f0                          	lock
14226cc5d: 48 ff 08                    	decq	(%rax)
14226cc60: 75 0c                       	jne	0x14226cc6e <.text+0x226bc6e>
14226cc62: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14226cc69: e8 f2 fe 8e fe              	callq	0x140b5cb60 <.text+0xb5bb60>
14226cc6e: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226cc76: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226cc7e: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226cc87: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226cc90: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226cc96: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226cc9c: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226cca2: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226cca8: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226ccae: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226ccb4: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226ccbb: 5b                          	popq	%rbx
14226ccbc: 5f                          	popq	%rdi
14226ccbd: 5e                          	popq	%rsi
14226ccbe: 41 5c                       	popq	%r12
14226ccc0: 41 5d                       	popq	%r13
14226ccc2: 41 5e                       	popq	%r14
14226ccc4: 41 5f                       	popq	%r15
14226ccc6: 5d                          	popq	%rbp
14226ccc7: c3                          	retq
14226ccc8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14226ccd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14226ccd5: 55                          	pushq	%rbp
14226ccd6: 41 57                       	pushq	%r15
14226ccd8: 41 56                       	pushq	%r14
14226ccda: 41 55                       	pushq	%r13
14226ccdc: 41 54                       	pushq	%r12
14226ccde: 56                          	pushq	%rsi
14226ccdf: 57                          	pushq	%rdi
14226cce0: 53                          	pushq	%rbx
14226cce1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14226cce8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14226ccef: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14226ccf5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14226ccfb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14226cd01: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14226cd07: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14226cd0d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14226cd13: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14226cd1c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14226cd25: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14226cd2d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14226cd35: 80 bd 77 01 00 00 00        	cmpb	$0x0, 0x177(%rbp)
14226cd3c: 74 66                       	je	0x14226cda4 <.text+0x226bda4>
14226cd3e: 48 8d 8d 00 01 00 00        	leaq	0x100(%rbp), %rcx
14226cd45: e8 a6 b8 c2 fe              	callq	0x140e985f0 <.text+0xe975f0>
14226cd4a: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226cd52: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226cd5a: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226cd63: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226cd6c: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226cd72: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226cd78: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226cd7e: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226cd84: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226cd8a: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226cd90: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226cd97: 5b                          	popq	%rbx
14226cd98: 5f                          	popq	%rdi
14226cd99: 5e                          	popq	%rsi
14226cd9a: 41 5c                       	popq	%r12
14226cd9c: 41 5d                       	popq	%r13
14226cd9e: 41 5e                       	popq	%r14
14226cda0: 41 5f                       	popq	%r15
14226cda2: 5d                          	popq	%rbp
14226cda3: c3                          	retq
14226cda4: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226cdac: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226cdb4: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226cdbd: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226cdc6: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226cdcc: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226cdd2: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226cdd8: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226cdde: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226cde4: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226cdea: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226cdf1: 5b                          	popq	%rbx
14226cdf2: 5f                          	popq	%rdi
14226cdf3: 5e                          	popq	%rsi
14226cdf4: 41 5c                       	popq	%r12
14226cdf6: 41 5d                       	popq	%r13
14226cdf8: 41 5e                       	popq	%r14
14226cdfa: 41 5f                       	popq	%r15
14226cdfc: 5d                          	popq	%rbp
14226cdfd: c3                          	retq
14226cdfe: 66 90                       	nop
14226ce00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14226ce05: 55                          	pushq	%rbp
14226ce06: 41 57                       	pushq	%r15
14226ce08: 41 56                       	pushq	%r14
14226ce0a: 41 55                       	pushq	%r13
14226ce0c: 41 54                       	pushq	%r12
14226ce0e: 56                          	pushq	%rsi
14226ce0f: 57                          	pushq	%rdi
14226ce10: 53                          	pushq	%rbx
14226ce11: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14226ce18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14226ce1f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14226ce25: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14226ce2b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14226ce31: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14226ce37: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14226ce3d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14226ce43: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14226ce4c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14226ce55: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14226ce5d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14226ce65: ba 40 00 00 00              	movl	$0x40, %edx
14226ce6a: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14226ce70: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
14226ce77: e8 a4 6d 3a fe              	callq	0x140613c20 <.text+0x612c20>
14226ce7c: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226ce84: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226ce8c: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226ce95: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226ce9e: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226cea4: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226ceaa: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226ceb0: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226ceb6: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226cebc: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226cec2: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226cec9: 5b                          	popq	%rbx
14226ceca: 5f                          	popq	%rdi
14226cecb: 5e                          	popq	%rsi
14226cecc: 41 5c                       	popq	%r12
14226cece: 41 5d                       	popq	%r13
14226ced0: 41 5e                       	popq	%r14
14226ced2: 41 5f                       	popq	%r15
14226ced4: 5d                          	popq	%rbp
14226ced5: c3                          	retq
14226ced6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14226cee0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14226cee5: 55                          	pushq	%rbp
14226cee6: 41 57                       	pushq	%r15
14226cee8: 41 56                       	pushq	%r14
14226ceea: 41 55                       	pushq	%r13
14226ceec: 41 54                       	pushq	%r12
14226ceee: 56                          	pushq	%rsi
14226ceef: 57                          	pushq	%rdi
14226cef0: 53                          	pushq	%rbx
14226cef1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14226cef8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14226ceff: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14226cf05: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14226cf0b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14226cf11: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14226cf17: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14226cf1d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14226cf23: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14226cf2c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14226cf35: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14226cf3d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14226cf45: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14226cf4c: e8 2f b9 c2 fe              	callq	0x140e98880 <.text+0xe97880>
14226cf51: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226cf59: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226cf61: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226cf6a: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226cf73: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226cf79: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226cf7f: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226cf85: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226cf8b: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226cf91: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226cf97: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226cf9e: 5b                          	popq	%rbx
14226cf9f: 5f                          	popq	%rdi
14226cfa0: 5e                          	popq	%rsi
14226cfa1: 41 5c                       	popq	%r12
14226cfa3: 41 5d                       	popq	%r13
14226cfa5: 41 5e                       	popq	%r14
14226cfa7: 41 5f                       	popq	%r15
14226cfa9: 5d                          	popq	%rbp
14226cfaa: c3                          	retq
14226cfab: 0f 1f 44 00 00              	nopl	(%rax,%rax)
14226cfb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14226cfb5: 55                          	pushq	%rbp
14226cfb6: 41 57                       	pushq	%r15
14226cfb8: 41 56                       	pushq	%r14
14226cfba: 41 55                       	pushq	%r13
14226cfbc: 41 54                       	pushq	%r12
14226cfbe: 56                          	pushq	%rsi
14226cfbf: 57                          	pushq	%rdi
14226cfc0: 53                          	pushq	%rbx
14226cfc1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14226cfc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14226cfcf: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14226cfd5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14226cfdb: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14226cfe1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14226cfe7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14226cfed: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14226cff3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14226cffc: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14226d005: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14226d00d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14226d015: 48 8d 4d 68                 	leaq	0x68(%rbp), %rcx
14226d019: e8 62 b8 c2 fe              	callq	0x140e98880 <.text+0xe97880>
14226d01e: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226d026: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226d02e: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226d037: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226d040: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226d046: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226d04c: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226d052: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226d058: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226d05e: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226d064: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226d06b: 5b                          	popq	%rbx
14226d06c: 5f                          	popq	%rdi
14226d06d: 5e                          	popq	%rsi
14226d06e: 41 5c                       	popq	%r12
14226d070: 41 5d                       	popq	%r13
14226d072: 41 5e                       	popq	%r14
14226d074: 41 5f                       	popq	%r15
14226d076: 5d                          	popq	%rbp
14226d077: c3                          	retq
14226d078: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14226d080: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14226d085: 55                          	pushq	%rbp
14226d086: 41 57                       	pushq	%r15
14226d088: 41 56                       	pushq	%r14
14226d08a: 41 55                       	pushq	%r13
14226d08c: 41 54                       	pushq	%r12
14226d08e: 56                          	pushq	%rsi
14226d08f: 57                          	pushq	%rdi
14226d090: 53                          	pushq	%rbx
14226d091: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
14226d098: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14226d09f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
14226d0a5: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
14226d0ab: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
14226d0b1: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
14226d0b7: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
14226d0bd: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
14226d0c3: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
14226d0cc: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
14226d0d5: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
14226d0dd: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
14226d0e5: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
14226d0ec: e8 ef bd c2 fe              	callq	0x140e98ee0 <.text+0xe97ee0>
14226d0f1: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
14226d0f9: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
14226d101: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
14226d10a: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
14226d113: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
14226d119: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
14226d11f: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
14226d125: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
14226d12b: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
14226d131: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
14226d137: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
14226d13e: 5b                          	popq	%rbx
14226d13f: 5f                          	popq	%rdi
14226d140: 5e                          	popq	%rsi
14226d141: 41 5c                       	popq	%r12
14226d143: 41 5d                       	popq	%r13
14226d145: 41 5e                       	popq	%r14
14226d147: 41 5f                       	popq	%r15
14226d149: 5d                          	popq	%rbp
14226d14a: c3                          	retq
