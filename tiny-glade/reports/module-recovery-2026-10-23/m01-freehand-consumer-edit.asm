
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14081bac0: 41 57                       	pushq	%r15
14081bac2: 41 56                       	pushq	%r14
14081bac4: 41 55                       	pushq	%r13
14081bac6: 41 54                       	pushq	%r12
14081bac8: 56                          	pushq	%rsi
14081bac9: 57                          	pushq	%rdi
14081baca: 55                          	pushq	%rbp
14081bacb: 53                          	pushq	%rbx
14081bacc: 48 81 ec 28 01 00 00        	subq	$0x128, %rsp            # imm = 0x128
14081bad3: 44 0f 29 94 24 10 01 00 00  	movaps	%xmm10, 0x110(%rsp)
14081badc: 66 44 0f 7f 8c 24 00 01 00 00       	movdqa	%xmm9, 0x100(%rsp)
14081bae6: 44 0f 29 84 24 f0 00 00 00  	movaps	%xmm8, 0xf0(%rsp)
14081baef: 0f 29 bc 24 e0 00 00 00     	movaps	%xmm7, 0xe0(%rsp)
14081baf7: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
14081baff: 48 8b 1a                    	movq	(%rdx), %rbx
14081bb02: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
14081bb06: 4c 8b 0b                    	movq	(%rbx), %r9
14081bb09: 45 31 f6                    	xorl	%r14d, %r14d
14081bb0c: 4d 89 ca                    	movq	%r9, %r10
14081bb0f: 4c 2b 52 18                 	subq	0x18(%rdx), %r10
14081bb13: 4d 0f 42 d6                 	cmovbq	%r14, %r10
14081bb17: 4c 2b 4a 38                 	subq	0x38(%rdx), %r9
14081bb1b: 4d 0f 42 ce                 	cmovbq	%r14, %r9
14081bb1f: 4b 8d 04 92                 	leaq	(%r10,%r10,4), %rax
14081bb23: 48 c1 e0 03                 	shlq	$0x3, %rax
14081bb27: 48 03 42 08                 	addq	0x8(%rdx), %rax
14081bb2b: 4c 8b 5a 10                 	movq	0x10(%rdx), %r11
14081bb2f: 4d 29 d3                    	subq	%r10, %r11
14081bb32: 4d 0f 42 de                 	cmovbq	%r14, %r11
14081bb36: 41 ba 08 00 00 00           	movl	$0x8, %r10d
14081bb3c: 49 0f 42 c2                 	cmovbq	%r10, %rax
14081bb40: 4b 8d 3c 89                 	leaq	(%r9,%r9,4), %rdi
14081bb44: 48 c1 e7 03                 	shlq	$0x3, %rdi
14081bb48: 48 03 7a 28                 	addq	0x28(%rdx), %rdi
14081bb4c: 48 8b 72 30                 	movq	0x30(%rdx), %rsi
14081bb50: 4c 29 ce                    	subq	%r9, %rsi
14081bb53: 49 0f 42 f6                 	cmovbq	%r14, %rsi
14081bb57: 49 0f 42 fa                 	cmovbq	%r10, %rdi
14081bb5b: 48 8b 52 40                 	movq	0x40(%rdx), %rdx
14081bb5f: 48 29 f2                    	subq	%rsi, %rdx
14081bb62: 4c 29 da                    	subq	%r11, %rdx
14081bb65: 48 89 5c 24 70              	movq	%rbx, 0x70(%rsp)
14081bb6a: 48 89 13                    	movq	%rdx, (%rbx)
14081bb6d: 4b 8d 14 9b                 	leaq	(%r11,%r11,4), %rdx
14081bb71: 48 8d 14 d0                 	leaq	(%rax,%rdx,8), %rdx
14081bb75: 48 89 54 24 58              	movq	%rdx, 0x58(%rsp)
14081bb7a: 48 8d 14 b6                 	leaq	(%rsi,%rsi,4), %rdx
14081bb7e: 48 8d 14 d7                 	leaq	(%rdi,%rdx,8), %rdx
14081bb82: 48 89 54 24 50              	movq	%rdx, 0x50(%rsp)
14081bb87: 41 8b 70 1c                 	movl	0x1c(%r8), %esi
14081bb8b: 4d 8b 30                    	movq	(%r8), %r14
14081bb8e: 4d 8b 68 10                 	movq	0x10(%r8), %r13
14081bb92: 48 8b 09                    	movq	(%rcx), %rcx
14081bb95: 48 89 4c 24 78              	movq	%rcx, 0x78(%rsp)
14081bb9a: 4c 8d bc 24 b0 00 00 00     	leaq	0xb0(%rsp), %r15
14081bba2: 66 0f 6f 35 d6 14 1b 02     	movdqa	0x21b14d6(%rip), %xmm6  # 0x1429cd080
14081bbaa: 66 0f 6f 3d de 14 1b 02     	movdqa	0x21b14de(%rip), %xmm7  # 0x1429cd090
14081bbb2: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
14081bbb6: 66 45 0f 76 c9              	pcmpeqd	%xmm9, %xmm9
14081bbbb: f3 44 0f 10 15 40 9d 10 02  	movss	0x2109d40(%rip), %xmm10 # 0x142925904
14081bbc4: 89 74 24 3c                 	movl	%esi, 0x3c(%rsp)
14081bbc8: 4c 89 74 24 48              	movq	%r14, 0x48(%rsp)
14081bbcd: 4c 89 6c 24 40              	movq	%r13, 0x40(%rsp)
14081bbd2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14081bbe0: 48 85 c0                    	testq	%rax, %rax
14081bbe3: 74 1b                       	je	0x14081bc00 <.text+0x81ac00>
14081bbe5: 48 3b 44 24 58              	cmpq	0x58(%rsp), %rax
14081bbea: 74 14                       	je	0x14081bc00 <.text+0x81ac00>
14081bbec: 48 8d 58 28                 	leaq	0x28(%rax), %rbx
14081bbf0: 48 89 fd                    	movq	%rdi, %rbp
14081bbf3: eb 2b                       	jmp	0x14081bc20 <.text+0x81ac20>
14081bbf5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14081bc00: 48 85 ff                    	testq	%rdi, %rdi
14081bc03: 0f 84 d1 03 00 00           	je	0x14081bfda <.text+0x81afda>
14081bc09: 48 3b 7c 24 50              	cmpq	0x50(%rsp), %rdi
14081bc0e: 0f 84 c6 03 00 00           	je	0x14081bfda <.text+0x81afda>
14081bc14: 48 8d 6f 28                 	leaq	0x28(%rdi), %rbp
14081bc18: 31 db                       	xorl	%ebx, %ebx
14081bc1a: 48 89 f8                    	movq	%rdi, %rax
14081bc1d: 48 89 ef                    	movq	%rbp, %rdi
14081bc20: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14081bc25: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
14081bc2a: 48 ff 00                    	incq	(%rax)
14081bc2d: 41 89 75 00                 	movl	%esi, (%r13)
14081bc31: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
14081bc36: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14081bc3a: 4c 89 f1                    	movq	%r14, %rcx
14081bc3d: e8 de 27 20 00              	callq	0x140a1e420 <.text+0xa1d420>
14081bc42: 48 8b 54 24 28              	movq	0x28(%rsp), %rdx
14081bc47: 49 89 c4                    	movq	%rax, %r12
14081bc4a: 48 89 d8                    	movq	%rbx, %rax
14081bc4d: 4d 85 e4                    	testq	%r12, %r12
14081bc50: 74 8e                       	je	0x14081bbe0 <.text+0x81abe0>
14081bc52: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
14081bc56: 48 89 ef                    	movq	%rbp, %rdi
14081bc59: 48 89 d8                    	movq	%rbx, %rax
14081bc5c: 48 85 c9                    	testq	%rcx, %rcx
14081bc5f: 0f 84 7b ff ff ff           	je	0x14081bbe0 <.text+0x81abe0>
14081bc65: 48 89 5c 24 60              	movq	%rbx, 0x60(%rsp)
14081bc6a: 48 89 6c 24 68              	movq	%rbp, 0x68(%rsp)
14081bc6f: 48 8b 7a 08                 	movq	0x8(%rdx), %rdi
14081bc73: 48 8d 04 49                 	leaq	(%rcx,%rcx,2), %rax
14081bc77: 48 8d 1c 87                 	leaq	(%rdi,%rax,4), %rbx
14081bc7b: 4d 89 e5                    	movq	%r12, %r13
14081bc7e: 49 83 c5 60                 	addq	$0x60, %r13
14081bc82: e9 79 01 00 00              	jmp	0x14081be00 <.text+0x81ae00>
14081bc87: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14081bc90: 49 21 c0                    	andq	%rax, %r8
14081bc93: f3 43 0f 6f 0c 06           	movdqu	(%r14,%r8), %xmm1
14081bc99: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14081bc9d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14081bca1: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
14081bca6: 45 85 c9                    	testl	%r9d, %r9d
14081bca9: 74 35                       	je	0x14081bce0 <.text+0x81ace0>
14081bcab: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
14081bcb0: 4d 01 c2                    	addq	%r8, %r10
14081bcb3: 49 21 c2                    	andq	%rax, %r10
14081bcb6: 49 f7 da                    	negq	%r10
14081bcb9: 4b 8d 34 92                 	leaq	(%r10,%r10,4), %rsi
14081bcbd: 48 c1 e6 04                 	shlq	$0x4, %rsi
14081bcc1: 3b 2c 31                    	cmpl	(%rcx,%rsi), %ebp
14081bcc4: 74 3a                       	je	0x14081bd00 <.text+0x81ad00>
14081bcc6: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
14081bcca: 66 45 21 ca                 	andw	%r9w, %r10w
14081bcce: 45 89 d1                    	movl	%r10d, %r9d
14081bcd1: 75 d8                       	jne	0x14081bcab <.text+0x81acab>
14081bcd3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14081bce0: 66 41 0f 74 c9              	pcmpeqb	%xmm9, %xmm1
14081bce5: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
14081bcea: 45 85 c9                    	testl	%r9d, %r9d
14081bced: 0f 85 bd 02 00 00           	jne	0x14081bfb0 <.text+0x81afb0>
14081bcf3: 49 01 d0                    	addq	%rdx, %r8
14081bcf6: 49 83 c0 10                 	addq	$0x10, %r8
14081bcfa: 48 83 c2 10                 	addq	$0x10, %rdx
14081bcfe: eb 90                       	jmp	0x14081bc90 <.text+0x81ac90>
14081bd00: 41 0f b6 44 36 da           	movzbl	-0x26(%r14,%rsi), %eax
14081bd06: 8d 48 fe                    	leal	-0x2(%rax), %ecx
14081bd09: 48 ff c8                    	decq	%rax
14081bd0c: 80 f9 03                    	cmpb	$0x3, %cl
14081bd0f: b9 00 00 00 00              	movl	$0x0, %ecx
14081bd14: 48 0f 43 c1                 	cmovaeq	%rcx, %rax
14081bd18: 48 85 c0                    	testq	%rax, %rax
14081bd1b: 74 0f                       	je	0x14081bd2c <.text+0x81ad2c>
14081bd1d: 48 83 f8 01                 	cmpq	$0x1, %rax
14081bd21: 0f 84 89 02 00 00           	je	0x14081bfb0 <.text+0x81afb0>
14081bd27: e9 ed 02 00 00              	jmp	0x14081c019 <.text+0x81b019>
14081bd2c: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
14081bd31: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14081bd35: 48 8b 4c 24 78              	movq	0x78(%rsp), %rcx
14081bd3a: 4c 8d 05 37 47 27 02        	leaq	0x2274737(%rip), %r8    # 0x142a90478
14081bd41: e8 aa 61 27 00              	callq	0x140a91ef0 <.text+0xa90ef0>
14081bd46: 48 85 c0                    	testq	%rax, %rax
14081bd49: 0f 84 fd 02 00 00           	je	0x14081c04c <.text+0x81b04c>
14081bd4f: 48 89 c5                    	movq	%rax, %rbp
14081bd52: 8b 4f 04                    	movl	0x4(%rdi), %ecx
14081bd55: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14081bd59: 48 39 ca                    	cmpq	%rcx, %rdx
14081bd5c: 0f 86 cf 02 00 00           	jbe	0x14081c031 <.text+0x81b031>
14081bd62: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
14081bd66: 48 39 d0                    	cmpq	%rdx, %rax
14081bd69: 0f 83 ce 02 00 00           	jae	0x14081c03d <.text+0x81b03d>
14081bd6f: 49 01 f6                    	addq	%rsi, %r14
14081bd72: f3 0f 10 47 08              	movss	0x8(%rdi), %xmm0
14081bd77: 48 8b 45 08                 	movq	0x8(%rbp), %rax
14081bd7b: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
14081bd7f: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14081bd83: f2 0f 10 14 c8              	movsd	(%rax,%rcx,8), %xmm2
14081bd88: f2 0f 10 5c c8 08           	movsd	0x8(%rax,%rcx,8), %xmm3
14081bd8e: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
14081bd92: 0f 59 ca                    	mulps	%xmm2, %xmm1
14081bd95: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
14081bd99: 0f 59 c3                    	mulps	%xmm3, %xmm0
14081bd9c: 0f 58 c1                    	addps	%xmm1, %xmm0
14081bd9f: 0f 13 44 24 30              	movlps	%xmm0, 0x30(%rsp)
14081bda4: f3 41 0f 10 56 bc           	movss	-0x44(%r14), %xmm2
14081bdaa: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
14081bdb2: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
14081bdb7: e8 24 10 48 00              	callq	0x140c9cde0 <.text+0xc9bde0>
14081bdbc: 8b 84 24 88 00 00 00        	movl	0x88(%rsp), %eax
14081bdc3: 41 89 46 d4                 	movl	%eax, -0x2c(%r14)
14081bdc7: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
14081bdcf: 49 89 46 cc                 	movq	%rax, -0x34(%r14)
14081bdd3: 8b 4f 04                    	movl	0x4(%rdi), %ecx
14081bdd6: 66 0f 6e 4f 08              	movd	0x8(%rdi), %xmm1
14081bddb: 66 41 0f 6e 56 bc           	movd	-0x44(%r14), %xmm2
14081bde1: 49 89 e9                    	movq	%rbp, %r9
14081bde4: e8 17 62 47 00              	callq	0x140c92000 <.text+0xc91000>
14081bde9: f3 41 0f 11 46 b8           	movss	%xmm0, -0x48(%r14)
14081bdef: 66 41 0f 7e 4e bc           	movd	%xmm1, -0x44(%r14)
14081bdf5: e9 b6 01 00 00              	jmp	0x14081bfb0 <.text+0x81afb0>
14081bdfa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
14081be00: 8b 2f                       	movl	(%rdi), %ebp
14081be02: 4c 89 e9                    	movq	%r13, %rcx
14081be05: 89 ea                       	movl	%ebp, %edx
14081be07: e8 f4 d3 aa ff              	callq	0x1402c9200 <.text+0x2c8200>
14081be0c: 49 83 7c 24 48 00           	cmpq	$0x0, 0x48(%r12)
14081be12: 0f 84 98 01 00 00           	je	0x14081bfb0 <.text+0x81afb0>
14081be18: f3 41 0f 6f 44 24 50        	movdqu	0x50(%r12), %xmm0
14081be1f: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
14081be24: 66 0f ef ce                 	pxor	%xmm6, %xmm1
14081be28: 66 0f 7f 8c 24 80 00 00 00  	movdqa	%xmm1, 0x80(%rsp)
14081be31: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
14081be36: 66 0f ef cf                 	pxor	%xmm7, %xmm1
14081be3a: 66 0f 7f 8c 24 90 00 00 00  	movdqa	%xmm1, 0x90(%rsp)
14081be43: 66 0f 7f 84 24 a0 00 00 00  	movdqa	%xmm0, 0xa0(%rsp)
14081be4c: 45 0f 29 07                 	movaps	%xmm8, (%r15)
14081be50: 49 c7 47 10 00 00 00 00     	movq	$0x0, 0x10(%r15)
14081be58: 89 6c 24 30                 	movl	%ebp, 0x30(%rsp)
14081be5c: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14081be62: 48 8d 8c 24 80 00 00 00     	leaq	0x80(%rsp), %rcx
14081be6a: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
14081be6f: e8 ec a2 8e ff              	callq	0x140106160 <.text+0x105160>
14081be74: 4c 8b 8c 24 b0 00 00 00     	movq	0xb0(%rsp), %r9
14081be7c: 49 c1 e1 38                 	shlq	$0x38, %r9
14081be80: 4c 0b 8c 24 b8 00 00 00     	orq	0xb8(%rsp), %r9
14081be88: 48 8b 8c 24 90 00 00 00     	movq	0x90(%rsp), %rcx
14081be90: 48 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %rax
14081be98: 4c 31 c8                    	xorq	%r9, %rax
14081be9b: 4c 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %r8
14081bea3: 49 01 c8                    	addq	%rcx, %r8
14081bea6: 48 8b 94 24 88 00 00 00     	movq	0x88(%rsp), %rdx
14081beae: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081beb2: 48 01 c2                    	addq	%rax, %rdx
14081beb5: 4c 31 c1                    	xorq	%r8, %rcx
14081beb8: 48 c1 c0 10                 	rolq	$0x10, %rax
14081bebc: 48 31 d0                    	xorq	%rdx, %rax
14081bebf: 49 c1 c0 20                 	rolq	$0x20, %r8
14081bec3: 48 01 ca                    	addq	%rcx, %rdx
14081bec6: 49 01 c0                    	addq	%rax, %r8
14081bec9: 48 c1 c1 11                 	rolq	$0x11, %rcx
14081becd: 48 31 d1                    	xorq	%rdx, %rcx
14081bed0: 48 c1 c0 15                 	rolq	$0x15, %rax
14081bed4: 48 c1 c2 20                 	rolq	$0x20, %rdx
14081bed8: 4c 31 c0                    	xorq	%r8, %rax
14081bedb: 4d 31 c8                    	xorq	%r9, %r8
14081bede: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
14081bee5: 49 01 c8                    	addq	%rcx, %r8
14081bee8: 48 01 c2                    	addq	%rax, %rdx
14081beeb: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081beef: 4c 31 c1                    	xorq	%r8, %rcx
14081bef2: 48 c1 c0 10                 	rolq	$0x10, %rax
14081bef6: 49 c1 c0 20                 	rolq	$0x20, %r8
14081befa: 48 31 d0                    	xorq	%rdx, %rax
14081befd: 48 01 ca                    	addq	%rcx, %rdx
14081bf00: 48 c1 c1 11                 	rolq	$0x11, %rcx
14081bf04: 49 01 c0                    	addq	%rax, %r8
14081bf07: 48 31 d1                    	xorq	%rdx, %rcx
14081bf0a: 48 c1 c0 15                 	rolq	$0x15, %rax
14081bf0e: 4c 31 c0                    	xorq	%r8, %rax
14081bf11: 48 c1 c2 20                 	rolq	$0x20, %rdx
14081bf15: 49 01 c8                    	addq	%rcx, %r8
14081bf18: 48 01 c2                    	addq	%rax, %rdx
14081bf1b: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081bf1f: 4c 31 c1                    	xorq	%r8, %rcx
14081bf22: 48 c1 c0 10                 	rolq	$0x10, %rax
14081bf26: 49 c1 c0 20                 	rolq	$0x20, %r8
14081bf2a: 48 31 d0                    	xorq	%rdx, %rax
14081bf2d: 48 01 ca                    	addq	%rcx, %rdx
14081bf30: 48 c1 c1 11                 	rolq	$0x11, %rcx
14081bf34: 49 01 c0                    	addq	%rax, %r8
14081bf37: 48 31 d1                    	xorq	%rdx, %rcx
14081bf3a: 48 c1 c0 15                 	rolq	$0x15, %rax
14081bf3e: 4c 31 c0                    	xorq	%r8, %rax
14081bf41: 48 c1 c2 20                 	rolq	$0x20, %rdx
14081bf45: 49 01 c8                    	addq	%rcx, %r8
14081bf48: 48 01 c2                    	addq	%rax, %rdx
14081bf4b: 48 c1 c1 0d                 	rolq	$0xd, %rcx
14081bf4f: 4c 31 c1                    	xorq	%r8, %rcx
14081bf52: 48 c1 c0 10                 	rolq	$0x10, %rax
14081bf56: 48 31 d0                    	xorq	%rdx, %rax
14081bf59: 48 01 ca                    	addq	%rcx, %rdx
14081bf5c: 48 c1 c1 11                 	rolq	$0x11, %rcx
14081bf60: 48 c1 c0 15                 	rolq	$0x15, %rax
14081bf64: 49 89 d0                    	movq	%rdx, %r8
14081bf67: 49 c1 c0 20                 	rolq	$0x20, %r8
14081bf6b: 49 31 c8                    	xorq	%rcx, %r8
14081bf6e: 49 31 c0                    	xorq	%rax, %r8
14081bf71: 49 31 d0                    	xorq	%rdx, %r8
14081bf74: 4c 89 c1                    	movq	%r8, %rcx
14081bf77: 48 c1 e9 39                 	shrq	$0x39, %rcx
14081bf7b: 4d 8b 74 24 30              	movq	0x30(%r12), %r14
14081bf80: 49 8b 44 24 38              	movq	0x38(%r12), %rax
14081bf85: 66 0f 6e c1                 	movd	%ecx, %xmm0
14081bf89: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14081bf8d: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14081bf92: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14081bf97: 49 8d 4e b0                 	leaq	-0x50(%r14), %rcx
14081bf9b: 31 d2                       	xorl	%edx, %edx
14081bf9d: e9 ee fc ff ff              	jmp	0x14081bc90 <.text+0x81ac90>
14081bfa2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14081bfb0: 48 83 c7 0c                 	addq	$0xc, %rdi
14081bfb4: 48 39 df                    	cmpq	%rbx, %rdi
14081bfb7: 0f 85 43 fe ff ff           	jne	0x14081be00 <.text+0x81ae00>
14081bfbd: 48 8b 7c 24 68              	movq	0x68(%rsp), %rdi
14081bfc2: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
14081bfc7: 8b 74 24 3c                 	movl	0x3c(%rsp), %esi
14081bfcb: 4c 8b 74 24 48              	movq	0x48(%rsp), %r14
14081bfd0: 4c 8b 6c 24 40              	movq	0x40(%rsp), %r13
14081bfd5: e9 06 fc ff ff              	jmp	0x14081bbe0 <.text+0x81abe0>
14081bfda: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
14081bfe2: 0f 28 bc 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm7
14081bfea: 44 0f 28 84 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm8
14081bff3: 44 0f 28 8c 24 00 01 00 00  	movaps	0x100(%rsp), %xmm9
14081bffc: 44 0f 28 94 24 10 01 00 00  	movaps	0x110(%rsp), %xmm10
14081c005: 48 81 c4 28 01 00 00        	addq	$0x128, %rsp            # imm = 0x128
14081c00c: 5b                          	popq	%rbx
14081c00d: 5d                          	popq	%rbp
14081c00e: 5f                          	popq	%rdi
14081c00f: 5e                          	popq	%rsi
14081c010: 41 5c                       	popq	%r12
14081c012: 41 5d                       	popq	%r13
14081c014: 41 5e                       	popq	%r14
14081c016: 41 5f                       	popq	%r15
14081c018: c3                          	retq
14081c019: 48 8d 0d 88 44 27 02        	leaq	0x2274488(%rip), %rcx   # 0x142a904a8
14081c020: 4c 8d 05 a9 44 27 02        	leaq	0x22744a9(%rip), %r8    # 0x142a904d0
14081c027: ba 28 00 00 00              	movl	$0x28, %edx
14081c02c: e8 ff d3 0b 02              	callq	0x1428d9430 <.text+0x28d8430>
14081c031: 4c 8d 05 b0 65 40 02        	leaq	0x24065b0(%rip), %r8    # 0x142c225e8
14081c038: e8 db d4 0b 02              	callq	0x1428d9518 <.text+0x28d8518>
14081c03d: 4c 8d 05 bc 65 40 02        	leaq	0x24065bc(%rip), %r8    # 0x142c22600
14081c044: 48 89 c1                    	movq	%rax, %rcx
14081c047: e8 cc d4 0b 02              	callq	0x1428d9518 <.text+0x28d8518>
14081c04c: 48 8d 0d 3d 44 27 02        	leaq	0x227443d(%rip), %rcx   # 0x142a90490
14081c053: e8 b8 d2 0b 02              	callq	0x1428d9310 <.text+0x28d8310>
14081c058: cc                          	int3
14081c059: cc                          	int3
14081c05a: cc                          	int3
14081c05b: cc                          	int3
14081c05c: cc                          	int3
14081c05d: cc                          	int3
14081c05e: cc                          	int3
14081c05f: cc                          	int3
