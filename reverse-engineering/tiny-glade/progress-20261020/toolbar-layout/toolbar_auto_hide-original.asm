
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142343a10: 41 57                       	pushq	%r15
142343a12: 41 56                       	pushq	%r14
142343a14: 41 55                       	pushq	%r13
142343a16: 41 54                       	pushq	%r12
142343a18: 56                          	pushq	%rsi
142343a19: 57                          	pushq	%rdi
142343a1a: 55                          	pushq	%rbp
142343a1b: 53                          	pushq	%rbx
142343a1c: 48 81 ec a8 01 00 00        	subq	$0x1a8, %rsp            # imm = 0x1A8
142343a23: 44 0f 29 bc 24 90 01 00 00  	movaps	%xmm15, 0x190(%rsp)
142343a2c: 44 0f 29 b4 24 80 01 00 00  	movaps	%xmm14, 0x180(%rsp)
142343a35: 44 0f 29 ac 24 70 01 00 00  	movaps	%xmm13, 0x170(%rsp)
142343a3e: 44 0f 29 a4 24 60 01 00 00  	movaps	%xmm12, 0x160(%rsp)
142343a47: 44 0f 29 9c 24 50 01 00 00  	movaps	%xmm11, 0x150(%rsp)
142343a50: 44 0f 29 94 24 40 01 00 00  	movaps	%xmm10, 0x140(%rsp)
142343a59: 44 0f 29 8c 24 30 01 00 00  	movaps	%xmm9, 0x130(%rsp)
142343a62: 44 0f 29 84 24 20 01 00 00  	movaps	%xmm8, 0x120(%rsp)
142343a6b: 0f 29 bc 24 10 01 00 00     	movaps	%xmm7, 0x110(%rsp)
142343a73: 0f 29 b4 24 00 01 00 00     	movaps	%xmm6, 0x100(%rsp)
142343a7b: 48 8b 84 24 40 02 00 00     	movq	0x240(%rsp), %rax
142343a83: 4c 8b 94 24 28 02 00 00     	movq	0x228(%rsp), %r10
142343a8b: 4c 8b 9c 24 10 02 00 00     	movq	0x210(%rsp), %r11
142343a93: 48 8b b4 24 48 02 00 00     	movq	0x248(%rsp), %rsi
142343a9b: 48 8b bc 24 18 02 00 00     	movq	0x218(%rsp), %rdi
142343aa3: 48 8b 9c 24 20 02 00 00     	movq	0x220(%rsp), %rbx
142343aab: 4c 8b b4 24 58 02 00 00     	movq	0x258(%rsp), %r14
142343ab3: 4c 8b bc 24 38 02 00 00     	movq	0x238(%rsp), %r15
142343abb: 4c 8b a4 24 30 02 00 00     	movq	0x230(%rsp), %r12
142343ac3: 48 8b 09                    	movq	(%rcx), %rcx
142343ac6: 8b 29                       	movl	(%rcx), %ebp
142343ac8: 41 8b 4c 24 1c              	movl	0x1c(%r12), %ecx
142343acd: 4d 8b 6c 24 10              	movq	0x10(%r12), %r13
142343ad2: 41 89 4d 00                 	movl	%ecx, (%r13)
142343ad6: 49 8b 0c 24                 	movq	(%r12), %rcx
142343ada: 45 8b 67 1c                 	movl	0x1c(%r15), %r12d
142343ade: 4d 8b 6f 10                 	movq	0x10(%r15), %r13
142343ae2: 45 89 65 00                 	movl	%r12d, (%r13)
142343ae6: 4d 8b 3f                    	movq	(%r15), %r15
142343ae9: 48 89 8c 24 c0 00 00 00     	movq	%rcx, 0xc0(%rsp)
142343af1: c6 84 24 c8 00 00 00 00     	movb	$0x0, 0xc8(%rsp)
142343af9: 4c 89 bc 24 d0 00 00 00     	movq	%r15, 0xd0(%rsp)
142343b01: c6 84 24 d8 00 00 00 01     	movb	$0x1, 0xd8(%rsp)
142343b09: 48 c7 84 24 e8 00 00 00 02 00 00 00 	movq	$0x2, 0xe8(%rsp)
142343b15: 4d 8b 29                    	movq	(%r9), %r13
142343b18: 49 8b 0e                    	movq	(%r14), %rcx
142343b1b: 48 89 8c 24 b8 00 00 00     	movq	%rcx, 0xb8(%rsp)
142343b23: 48 8b 0b                    	movq	(%rbx), %rcx
142343b26: 48 89 8c 24 a8 00 00 00     	movq	%rcx, 0xa8(%rsp)
142343b2e: 48 8b 0f                    	movq	(%rdi), %rcx
142343b31: 48 89 4c 24 40              	movq	%rcx, 0x40(%rsp)
142343b36: 4c 8b 67 08                 	movq	0x8(%rdi), %r12
142343b3a: 48 8b 0e                    	movq	(%rsi), %rcx
142343b3d: 48 89 4c 24 58              	movq	%rcx, 0x58(%rsp)
142343b42: 48 8b 4e 08                 	movq	0x8(%rsi), %rcx
142343b46: 48 89 4c 24 60              	movq	%rcx, 0x60(%rsp)
142343b4b: 8b 4e 14                    	movl	0x14(%rsi), %ecx
142343b4e: 89 4c 24 4c                 	movl	%ecx, 0x4c(%rsp)
142343b52: 48 8b 0a                    	movq	(%rdx), %rcx
142343b55: 48 89 8c 24 98 00 00 00     	movq	%rcx, 0x98(%rsp)
142343b5d: 49 8b 08                    	movq	(%r8), %rcx
142343b60: 48 89 8c 24 a0 00 00 00     	movq	%rcx, 0xa0(%rsp)
142343b68: 49 8b 0b                    	movq	(%r11), %rcx
142343b6b: 48 89 8c 24 80 00 00 00     	movq	%rcx, 0x80(%rsp)
142343b73: 49 8b 0a                    	movq	(%r10), %rcx
142343b76: 48 89 8c 24 90 00 00 00     	movq	%rcx, 0x90(%rsp)
142343b7e: 44 8b 78 1c                 	movl	0x1c(%rax), %r15d
142343b82: 48 8b 38                    	movq	(%rax), %rdi
142343b85: 48 8b 40 10                 	movq	0x10(%rax), %rax
142343b89: 48 89 84 24 b0 00 00 00     	movq	%rax, 0xb0(%rsp)
142343b91: 48 8b 84 24 50 02 00 00     	movq	0x250(%rsp), %rax
142343b99: 8b 48 1c                    	movl	0x1c(%rax), %ecx
142343b9c: 89 4c 24 48                 	movl	%ecx, 0x48(%rsp)
142343ba0: 48 8b 08                    	movq	(%rax), %rcx
142343ba3: 48 8b 40 10                 	movq	0x10(%rax), %rax
142343ba7: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
142343bac: f3 0f 10 35 48 a7 d0 00     	movss	0xd0a748(%rip), %xmm6   # 0x14304e2fc
142343bb4: f3 0f 10 3d 28 65 61 00     	movss	0x616528(%rip), %xmm7   # 0x14295a0e4
142343bbc: f3 44 0f 10 05 e3 09 5f 00  	movss	0x5f09e3(%rip), %xmm8   # 0x1429345a8
142343bc5: f3 44 0f 10 0d 0a 1d 5e 00  	movss	0x5e1d0a(%rip), %xmm9   # 0x1429258d8
142343bce: f3 44 0f 10 25 11 65 61 00  	movss	0x616511(%rip), %xmm12  # 0x14295a0e8
142343bd7: f3 44 0f 10 1d 90 1c 5e 00  	movss	0x5e1c90(%rip), %xmm11  # 0x142925870
142343be0: f2 44 0f 10 35 27 a7 d0 00  	movsd	0xd0a727(%rip), %xmm14  # 0x14304e310
142343be9: f2 44 0f 10 3d 0e a7 d0 00  	movsd	0xd0a70e(%rip), %xmm15  # 0x14304e300
142343bf2: f3 44 0f 10 15 4d fc 6c 00  	movss	0x6cfc4d(%rip), %xmm10  # 0x142a13848
142343bfb: 4c 8d 84 24 c8 00 00 00     	leaq	0xc8(%rsp), %r8
142343c03: 48 89 4c 24 50              	movq	%rcx, 0x50(%rsp)
142343c08: 48 8d 41 20                 	leaq	0x20(%rcx), %rax
142343c0c: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
142343c11: b9 02 00 00 00              	movl	$0x2, %ecx
142343c16: 31 c0                       	xorl	%eax, %eax
142343c18: 48 89 c2                    	movq	%rax, %rdx
142343c1b: 48 c1 e2 04                 	shlq	$0x4, %rdx
142343c1f: 4c 01 c2                    	addq	%r8, %rdx
142343c22: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
142343c30: 48 39 c1                    	cmpq	%rax, %rcx
142343c33: 0f 84 ae 08 00 00           	je	0x1423444e7 <.text+0x23434e7>
142343c39: 83 fd 13                    	cmpl	$0x13, %ebp
142343c3c: 41 0f 94 c0                 	sete	%r8b
142343c40: 48 ff c0                    	incq	%rax
142343c43: 0f b6 1a                    	movzbl	(%rdx), %ebx
142343c46: 41 30 d8                    	xorb	%bl, %r8b
142343c49: 48 83 c2 10                 	addq	$0x10, %rdx
142343c4d: 45 84 c0                    	testb	%r8b, %r8b
142343c50: 75 de                       	jne	0x142343c30 <.text+0x2342c30>
142343c52: 48 89 84 24 e0 00 00 00     	movq	%rax, 0xe0(%rsp)
142343c5a: 4c 8b 42 e8                 	movq	-0x18(%rdx), %r8
142343c5e: f3 45 0f 10 ad e8 00 00 00  	movss	0xe8(%r13), %xmm13
142343c67: 48 8b 84 24 b8 00 00 00     	movq	0xb8(%rsp), %rax
142343c6f: f3 44 0f 59 28              	mulss	(%rax), %xmm13
142343c74: 41 80 bd f1 00 00 00 00     	cmpb	$0x0, 0xf1(%r13)
142343c7c: 0f 84 4e 01 00 00           	je	0x142343dd0 <.text+0x2342dd0>
142343c82: 4c 89 44 24 38              	movq	%r8, 0x38(%rsp)
142343c87: 48 8b 8c 24 a8 00 00 00     	movq	0xa8(%rsp), %rcx
142343c8f: e8 7c 5f 60 fe              	callq	0x140949c10 <.text+0x948c10>
142343c94: 0f 28 d1                    	movaps	%xmm1, %xmm2
142343c97: 4c 8b 44 24 40              	movq	0x40(%rsp), %r8
142343c9c: 49 8b 88 f8 00 00 00        	movq	0xf8(%r8), %rcx
142343ca3: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
142343caa: 48 8d 14 81                 	leaq	(%rcx,%rax,4), %rdx
142343cae: 41 80 b8 08 01 00 00 00     	cmpb	$0x0, 0x108(%r8)
142343cb6: 0f 84 21 01 00 00           	je	0x142343ddd <.text+0x2342ddd>
142343cbc: 45 31 c9                    	xorl	%r9d, %r9d
142343cbf: 45 31 c0                    	xorl	%r8d, %r8d
142343cc2: 45 31 d2                    	xorl	%r10d, %r10d
142343cc5: 31 c0                       	xorl	%eax, %eax
142343cc7: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
142343cd0: 31 f6                       	xorl	%esi, %esi
142343cd2: 4d 39 c8                    	cmpq	%r9, %r8
142343cd5: 0f 85 95 00 00 00           	jne	0x142343d70 <.text+0x2342d70>
142343cdb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
142343ce0: 48 39 d1                    	cmpq	%rdx, %rcx
142343ce3: 0f 84 46 02 00 00           	je	0x142343f2f <.text+0x2342f2f>
142343ce9: 44 8b 01                    	movl	(%rcx), %r8d
142343cec: 48 83 c1 04                 	addq	$0x4, %rcx
142343cf0: 49 8b 84 24 a8 01 00 00     	movq	0x1a8(%r12), %rax
142343cf8: 4f 8d 04 c0                 	leaq	(%r8,%r8,8), %r8
142343cfc: 4e 8b 4c c0 10              	movq	0x10(%rax,%r8,8), %r9
142343d01: 4d 85 c9                    	testq	%r9, %r9
142343d04: 74 da                       	je	0x142343ce0 <.text+0x2342ce0>
142343d06: 4e 8d 14 c0                 	leaq	(%rax,%r8,8), %r10
142343d0a: 49 8b 42 38                 	movq	0x38(%r10), %rax
142343d0e: 4d 8b 42 40                 	movq	0x40(%r10), %r8
142343d12: 4c 8b 5c 24 40              	movq	0x40(%rsp), %r11
142343d17: 4d 8b 9b 18 01 00 00        	movq	0x118(%r11), %r11
142343d1e: 4d 39 d8                    	cmpq	%r11, %r8
142343d21: 76 79                       	jbe	0x142343d9c <.text+0x2342d9c>
142343d23: 4e 8b 1c d8                 	movq	(%rax,%r11,8), %r11
142343d27: 4d 85 db                    	testq	%r11, %r11
142343d2a: 74 70                       	je	0x142343d9c <.text+0x2342d9c>
142343d2c: 4d 8b 52 18                 	movq	0x18(%r10), %r10
142343d30: 49 f7 d3                    	notq	%r11
142343d33: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
142343d37: 49 c1 e3 04                 	shlq	$0x4, %r11
142343d3b: 4f 8b 54 1a 10              	movq	0x10(%r10,%r11), %r10
142343d40: 4c 8b 5c 24 40              	movq	0x40(%rsp), %r11
142343d45: 4d 8b 9b 20 01 00 00        	movq	0x120(%r11), %r11
142343d4c: 4d 39 d8                    	cmpq	%r11, %r8
142343d4f: 77 5f                       	ja	0x142343db0 <.text+0x2342db0>
142343d51: 45 31 db                    	xorl	%r11d, %r11d
142343d54: 31 c0                       	xorl	%eax, %eax
142343d56: 4d 8d 43 01                 	leaq	0x1(%r11), %r8
142343d5a: 38 d8                       	cmpb	%bl, %al
142343d5c: 0f 85 70 ff ff ff           	jne	0x142343cd2 <.text+0x2342cd2>
142343d62: eb 1b                       	jmp	0x142343d7f <.text+0x2342d7f>
142343d64: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
142343d70: 4d 89 c3                    	movq	%r8, %r11
142343d73: 4d 8d 43 01                 	leaq	0x1(%r11), %r8
142343d77: 38 d8                       	cmpb	%bl, %al
142343d79: 0f 85 53 ff ff ff           	jne	0x142343cd2 <.text+0x2342cd2>
142343d7f: 45 89 db                    	movl	%r11d, %r11d
142343d82: 4f 8d 1c 5b                 	leaq	(%r11,%r11,2), %r11
142343d86: 4c 8b 74 24 30              	movq	0x30(%rsp), %r14
142343d8b: 47 0a 74 1a 01              	orb	0x1(%r10,%r11), %r14b
142343d90: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
142343d95: ff c6                       	incl	%esi
142343d97: e9 36 ff ff ff              	jmp	0x142343cd2 <.text+0x2342cd2>
142343d9c: 45 31 d2                    	xorl	%r10d, %r10d
142343d9f: 4c 8b 5c 24 40              	movq	0x40(%rsp), %r11
142343da4: 4d 8b 9b 20 01 00 00        	movq	0x120(%r11), %r11
142343dab: 4d 39 d8                    	cmpq	%r11, %r8
142343dae: 76 a1                       	jbe	0x142343d51 <.text+0x2342d51>
142343db0: 4a 83 3c d8 00              	cmpq	$0x0, (%rax,%r11,8)
142343db5: 0f 95 c0                    	setne	%al
142343db8: 45 31 db                    	xorl	%r11d, %r11d
142343dbb: 4d 8d 43 01                 	leaq	0x1(%r11), %r8
142343dbf: 38 d8                       	cmpb	%bl, %al
142343dc1: 0f 85 0b ff ff ff           	jne	0x142343cd2 <.text+0x2342cd2>
142343dc7: eb b6                       	jmp	0x142343d7f <.text+0x2342d7f>
142343dc9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
142343dd0: 41 c7 40 2c 00 00 00 00     	movl	$0x0, 0x2c(%r8)
142343dd8: e9 33 06 00 00              	jmp	0x142344410 <.text+0x2343410>
142343ddd: 44 89 7c 24 6c              	movl	%r15d, 0x6c(%rsp)
142343de2: 31 f6                       	xorl	%esi, %esi
142343de4: 41 bb 08 00 00 00           	movl	$0x8, %r11d
142343dea: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
142343df3: 45 31 c9                    	xorl	%r9d, %r9d
142343df6: 45 31 f6                    	xorl	%r14d, %r14d
142343df9: 45 31 c0                    	xorl	%r8d, %r8d
142343dfc: 45 31 d2                    	xorl	%r10d, %r10d
142343dff: 31 c0                       	xorl	%eax, %eax
142343e01: 4d 39 f0                    	cmpq	%r14, %r8
142343e04: 74 1a                       	je	0x142343e20 <.text+0x2342e20>
142343e06: 4d 89 c7                    	movq	%r8, %r15
142343e09: 4d 8d 47 01                 	leaq	0x1(%r15), %r8
142343e0d: 38 d8                       	cmpb	%bl, %al
142343e0f: 0f 84 d9 00 00 00           	je	0x142343eee <.text+0x2342eee>
142343e15: 4d 39 f0                    	cmpq	%r14, %r8
142343e18: 75 ec                       	jne	0x142343e06 <.text+0x2342e06>
142343e1a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
142343e20: 48 39 d1                    	cmpq	%rdx, %rcx
142343e23: 0f 84 01 01 00 00           	je	0x142343f2a <.text+0x2342f2a>
142343e29: 8b 01                       	movl	(%rcx), %eax
142343e2b: 48 83 c1 04                 	addq	$0x4, %rcx
142343e2f: 4d 8b 84 24 00 01 00 00     	movq	0x100(%r12), %r8
142343e37: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
142343e3b: 48 c1 e0 05                 	shlq	$0x5, %rax
142343e3f: 4d 8b 4c 00 58              	movq	0x58(%r8,%rax), %r9
142343e44: 4d 85 c9                    	testq	%r9, %r9
142343e47: 74 d7                       	je	0x142343e20 <.text+0x2342e20>
142343e49: 49 01 c0                    	addq	%rax, %r8
142343e4c: 45 8b 90 94 00 00 00        	movl	0x94(%r8), %r10d
142343e53: 49 8b 84 24 a8 01 00 00     	movq	0x1a8(%r12), %rax
142343e5b: 4f 8d 14 d2                 	leaq	(%r10,%r10,8), %r10
142343e5f: 4c 8b 5c 24 40              	movq	0x40(%rsp), %r11
142343e64: 4d 8b 9b 18 01 00 00        	movq	0x118(%r11), %r11
142343e6b: 4e 39 5c d0 40              	cmpq	%r11, 0x40(%rax,%r10,8)
142343e70: 76 3f                       	jbe	0x142343eb1 <.text+0x2342eb1>
142343e72: 4e 8b 74 d0 38              	movq	0x38(%rax,%r10,8), %r14
142343e77: 4f 8b 1c de                 	movq	(%r14,%r11,8), %r11
142343e7b: 4d 85 db                    	testq	%r11, %r11
142343e7e: 74 31                       	je	0x142343eb1 <.text+0x2342eb1>
142343e80: 4a 8d 04 d0                 	leaq	(%rax,%r10,8), %rax
142343e84: 48 8b 40 18                 	movq	0x18(%rax), %rax
142343e88: 49 f7 d3                    	notq	%r11
142343e8b: 4f 8d 14 5b                 	leaq	(%r11,%r11,2), %r10
142343e8f: 49 c1 e2 04                 	shlq	$0x4, %r10
142343e93: 4e 8b 54 10 10              	movq	0x10(%rax,%r10), %r10
142343e98: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
142343e9d: 48 8b 80 20 01 00 00        	movq	0x120(%rax), %rax
142343ea4: 49 39 80 88 00 00 00        	cmpq	%rax, 0x88(%r8)
142343eab: 77 1c                       	ja	0x142343ec9 <.text+0x2342ec9>
142343ead: 31 c0                       	xorl	%eax, %eax
142343eaf: eb 27                       	jmp	0x142343ed8 <.text+0x2342ed8>
142343eb1: 45 31 d2                    	xorl	%r10d, %r10d
142343eb4: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
142343eb9: 48 8b 80 20 01 00 00        	movq	0x120(%rax), %rax
142343ec0: 49 39 80 88 00 00 00        	cmpq	%rax, 0x88(%r8)
142343ec7: 76 e4                       	jbe	0x142343ead <.text+0x2342ead>
142343ec9: 4d 8b 98 80 00 00 00        	movq	0x80(%r8), %r11
142343ed0: 49 83 3c c3 00              	cmpq	$0x0, (%r11,%rax,8)
142343ed5: 0f 95 c0                    	setne	%al
142343ed8: 4d 8b 58 50                 	movq	0x50(%r8), %r11
142343edc: 4d 89 ce                    	movq	%r9, %r14
142343edf: 45 31 ff                    	xorl	%r15d, %r15d
142343ee2: 4d 8d 47 01                 	leaq	0x1(%r15), %r8
142343ee6: 38 d8                       	cmpb	%bl, %al
142343ee8: 0f 85 27 ff ff ff           	jne	0x142343e15 <.text+0x2342e15>
142343eee: 49 c1 e7 04                 	shlq	$0x4, %r15
142343ef2: 47 8b 7c 3b 08              	movl	0x8(%r11,%r15), %r15d
142343ef7: 4f 8d 3c 7f                 	leaq	(%r15,%r15,2), %r15
142343efb: 48 89 b4 24 88 00 00 00     	movq	%rsi, 0x88(%rsp)
142343f03: 48 8b 74 24 30              	movq	0x30(%rsp), %rsi
142343f08: 43 0a 74 3a 01              	orb	0x1(%r10,%r15), %sil
142343f0d: 48 89 74 24 30              	movq	%rsi, 0x30(%rsp)
142343f12: 48 8b b4 24 88 00 00 00     	movq	0x88(%rsp), %rsi
142343f1a: ff c6                       	incl	%esi
142343f1c: 4d 39 f0                    	cmpq	%r14, %r8
142343f1f: 0f 85 e1 fe ff ff           	jne	0x142343e06 <.text+0x2342e06>
142343f25: e9 f6 fe ff ff              	jmp	0x142343e20 <.text+0x2342e20>
142343f2a: 44 8b 7c 24 6c              	movl	0x6c(%rsp), %r15d
142343f2f: f3 0f 2a e6                 	cvtsi2ss	%esi, %xmm4
142343f33: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
142343f37: f3 0f 59 ce                 	mulss	%xmm6, %xmm1
142343f3b: 0f 28 d9                    	movaps	%xmm1, %xmm3
142343f3e: f3 0f 59 dc                 	mulss	%xmm4, %xmm3
142343f42: f3 0f 58 e7                 	addss	%xmm7, %xmm4
142343f46: f3 41 0f 59 e0              	mulss	%xmm8, %xmm4
142343f4b: f3 41 0f 59 e5              	mulss	%xmm13, %xmm4
142343f50: f3 0f 58 e3                 	addss	%xmm3, %xmm4
142343f54: f3 41 0f 59 c9              	mulss	%xmm9, %xmm1
142343f59: 41 0f b6 85 f0 00 00 00     	movzbl	0xf0(%r13), %eax
142343f61: 8d 48 ff                    	leal	-0x1(%rax), %ecx
142343f64: 80 f9 02                    	cmpb	$0x2, %cl
142343f67: 72 13                       	jb	0x142343f7c <.text+0x2342f7c>
142343f69: 0f 28 dc                    	movaps	%xmm4, %xmm3
142343f6c: 48 8d 0d 45 a7 d0 00        	leaq	0xd0a745(%rip), %rcx    # 0x14304e6b8
142343f73: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
142343f77: 48 01 c8                    	addq	%rcx, %rax
142343f7a: ff e0                       	jmpq	*%rax
142343f7c: 0f 28 d9                    	movaps	%xmm1, %xmm3
142343f7f: 0f 28 cc                    	movaps	%xmm4, %xmm1
142343f82: 48 8d 0d 2f a7 d0 00        	leaq	0xd0a72f(%rip), %rcx    # 0x14304e6b8
142343f89: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
142343f8d: 48 01 c8                    	addq	%rcx, %rax
142343f90: ff e0                       	jmpq	*%rax
142343f92: 0f 28 e1                    	movaps	%xmm1, %xmm4
142343f95: f3 41 0f 59 e4              	mulss	%xmm12, %xmm4
142343f9a: f3 0f 58 d4                 	addss	%xmm4, %xmm2
142343f9e: eb 16                       	jmp	0x142343fb6 <.text+0x2342fb6>
142343fa0: 0f 28 e3                    	movaps	%xmm3, %xmm4
142343fa3: f3 41 0f 59 e4              	mulss	%xmm12, %xmm4
142343fa8: f3 0f 58 c4                 	addss	%xmm4, %xmm0
142343fac: eb 20                       	jmp	0x142343fce <.text+0x2342fce>
142343fae: 0f 28 d1                    	movaps	%xmm1, %xmm2
142343fb1: f3 41 0f 59 d3              	mulss	%xmm11, %xmm2
142343fb6: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
142343fbb: f3 41 0f 12 e5              	movsldup	%xmm13, %xmm4           # xmm4 = xmm13[0,0,2,2]
142343fc0: 41 0f 59 e6                 	mulps	%xmm14, %xmm4
142343fc4: eb 16                       	jmp	0x142343fdc <.text+0x2342fdc>
142343fc6: 0f 28 c3                    	movaps	%xmm3, %xmm0
142343fc9: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
142343fce: f3 41 0f 59 d3              	mulss	%xmm11, %xmm2
142343fd3: f3 41 0f 12 e5              	movsldup	%xmm13, %xmm4           # xmm4 = xmm13[0,0,2,2]
142343fd8: 41 0f 59 e7                 	mulps	%xmm15, %xmm4
142343fdc: 4c 8b 44 24 58              	movq	0x58(%rsp), %r8
142343fe1: 49 8b 88 f8 00 00 00        	movq	0xf8(%r8), %rcx
142343fe8: 49 8b 80 00 01 00 00        	movq	0x100(%r8), %rax
142343fef: 48 8d 14 81                 	leaq	(%rcx,%rax,4), %rdx
142343ff3: 0f 58 e4                    	addps	%xmm4, %xmm4
142343ff6: 0f 14 d9                    	unpcklps	%xmm1, %xmm3            # xmm3 = xmm3[0],xmm1[0],xmm3[1],xmm1[1]
142343ff9: 0f 58 dc                    	addps	%xmm4, %xmm3
142343ffc: 41 80 b8 08 01 00 00 00     	cmpb	$0x0, 0x108(%r8)
142344004: 74 0d                       	je	0x142344013 <.text+0x2343013>
142344006: 45 31 c9                    	xorl	%r9d, %r9d
142344009: 31 c0                       	xorl	%eax, %eax
14234400b: 45 31 d2                    	xorl	%r10d, %r10d
14234400e: e9 8e 01 00 00              	jmp	0x1423441a1 <.text+0x23431a1>
142344013: 41 b8 08 00 00 00           	movl	$0x8, %r8d
142344019: 45 31 d2                    	xorl	%r10d, %r10d
14234401c: 31 c0                       	xorl	%eax, %eax
14234401e: 45 31 db                    	xorl	%r11d, %r11d
142344021: 49 39 c3                    	cmpq	%rax, %r11
142344024: 74 1a                       	je	0x142344040 <.text+0x2343040>
142344026: 48 89 fe                    	movq	%rdi, %rsi
142344029: 4c 89 ef                    	movq	%r13, %rdi
14234402c: 45 89 fd                    	movl	%r15d, %r13d
14234402f: e9 b9 00 00 00              	jmp	0x1423440ed <.text+0x23430ed>
142344034: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
142344040: 48 39 d1                    	cmpq	%rdx, %rcx
142344043: 0f 84 94 01 00 00           	je	0x1423441dd <.text+0x23431dd>
142344049: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
14234404e: 4c 8b 80 00 01 00 00        	movq	0x100(%rax), %r8
142344055: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
142344060: 8b 01                       	movl	(%rcx), %eax
142344062: 4c 8d 0c 80                 	leaq	(%rax,%rax,4), %r9
142344066: 49 c1 e1 05                 	shlq	$0x5, %r9
14234406a: 4b 8b 44 08 58              	movq	0x58(%r8,%r9), %rax
14234406f: 48 85 c0                    	testq	%rax, %rax
142344072: 75 1c                       	jne	0x142344090 <.text+0x2343090>
142344074: 48 83 c1 04                 	addq	$0x4, %rcx
142344078: 48 39 d1                    	cmpq	%rdx, %rcx
14234407b: 75 e3                       	jne	0x142344060 <.text+0x2343060>
14234407d: e9 5b 01 00 00              	jmp	0x1423441dd <.text+0x23431dd>
142344082: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
142344090: 48 89 fe                    	movq	%rdi, %rsi
142344093: 4c 89 ef                    	movq	%r13, %rdi
142344096: 45 89 fd                    	movl	%r15d, %r13d
142344099: 4d 01 c8                    	addq	%r9, %r8
14234409c: 45 8b 88 94 00 00 00        	movl	0x94(%r8), %r9d
1423440a3: 4c 8b 54 24 60              	movq	0x60(%rsp), %r10
1423440a8: 4d 8b 92 a8 01 00 00        	movq	0x1a8(%r10), %r10
1423440af: 4f 8d 0c c9                 	leaq	(%r9,%r9,8), %r9
1423440b3: 4c 8b 5c 24 58              	movq	0x58(%rsp), %r11
1423440b8: 4d 8b 9b 10 01 00 00        	movq	0x110(%r11), %r11
1423440bf: 4b 8b 5c ca 18              	movq	0x18(%r10,%r9,8), %rbx
1423440c4: 4f 8b 4c ca 38              	movq	0x38(%r10,%r9,8), %r9
1423440c9: 4f 8b 0c d9                 	movq	(%r9,%r11,8), %r9
1423440cd: 49 f7 d1                    	notq	%r9
1423440d0: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1423440d4: 49 c1 e1 04                 	shlq	$0x4, %r9
1423440d8: 4e 8b 54 0b 10              	movq	0x10(%rbx,%r9), %r10
1423440dd: 4e 8b 4c 0b 28              	movq	0x28(%rbx,%r9), %r9
1423440e2: 4d 8b 40 50                 	movq	0x50(%r8), %r8
1423440e6: 48 83 c1 04                 	addq	$0x4, %rcx
1423440ea: 45 31 db                    	xorl	%r11d, %r11d
1423440ed: 4c 89 db                    	movq	%r11, %rbx
1423440f0: 48 c1 e3 04                 	shlq	$0x4, %rbx
1423440f4: 41 8b 5c 18 08              	movl	0x8(%r8,%rbx), %ebx
1423440f9: 49 89 de                    	movq	%rbx, %r14
1423440fc: 49 c1 e6 04                 	shlq	$0x4, %r14
142344100: 49 ff c3                    	incq	%r11
142344103: f3 43 0f 11 04 32           	movss	%xmm0, (%r10,%r14)
142344109: f3 43 0f 11 54 32 04        	movss	%xmm2, 0x4(%r10,%r14)
142344110: 44 8b 7c 24 4c              	movl	0x4c(%rsp), %r15d
142344115: 45 89 3c 99                 	movl	%r15d, (%r9,%rbx,4)
142344119: 43 0f 13 5c 32 08           	movlps	%xmm3, 0x8(%r10,%r14)
14234411f: 45 89 ef                    	movl	%r13d, %r15d
142344122: 49 89 fd                    	movq	%rdi, %r13
142344125: 48 89 f7                    	movq	%rsi, %rdi
142344128: 49 39 c3                    	cmpq	%rax, %r11
14234412b: 0f 84 0f ff ff ff           	je	0x142344040 <.text+0x2343040>
142344131: e9 f0 fe ff ff              	jmp	0x142344026 <.text+0x2343026>
142344136: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
142344140: 4f 8d 04 c8                 	leaq	(%r8,%r9,8), %r8
142344144: 4c 8b 4c 24 58              	movq	0x58(%rsp), %r9
142344149: 4d 8b 89 10 01 00 00        	movq	0x110(%r9), %r9
142344150: 4d 8b 50 18                 	movq	0x18(%r8), %r10
142344154: 4d 8b 40 38                 	movq	0x38(%r8), %r8
142344158: 4f 8b 04 c8                 	movq	(%r8,%r9,8), %r8
14234415c: 49 f7 d0                    	notq	%r8
14234415f: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
142344163: 49 c1 e0 04                 	shlq	$0x4, %r8
142344167: 4f 8b 4c 02 10              	movq	0x10(%r10,%r8), %r9
14234416c: 4f 8b 44 02 28              	movq	0x28(%r10,%r8), %r8
142344171: 48 83 c1 04                 	addq	$0x4, %rcx
142344175: 45 31 d2                    	xorl	%r10d, %r10d
142344178: 45 89 d3                    	movl	%r10d, %r11d
14234417b: 4c 89 db                    	movq	%r11, %rbx
14234417e: 48 c1 e3 04                 	shlq	$0x4, %rbx
142344182: 49 ff c2                    	incq	%r10
142344185: f3 41 0f 11 04 19           	movss	%xmm0, (%r9,%rbx)
14234418b: f3 41 0f 11 54 19 04        	movss	%xmm2, 0x4(%r9,%rbx)
142344192: 44 8b 74 24 4c              	movl	0x4c(%rsp), %r14d
142344197: 47 89 34 98                 	movl	%r14d, (%r8,%r11,4)
14234419b: 41 0f 13 5c 19 08           	movlps	%xmm3, 0x8(%r9,%rbx)
1423441a1: 49 39 c2                    	cmpq	%rax, %r10
1423441a4: 75 d2                       	jne	0x142344178 <.text+0x2343178>
1423441a6: 48 39 d1                    	cmpq	%rdx, %rcx
1423441a9: 74 32                       	je	0x1423441dd <.text+0x23431dd>
1423441ab: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
1423441b0: 4c 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %r8
1423441b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1423441c0: 8b 01                       	movl	(%rcx), %eax
1423441c2: 4c 8d 0c c0                 	leaq	(%rax,%rax,8), %r9
1423441c6: 4b 8b 44 c8 10              	movq	0x10(%r8,%r9,8), %rax
1423441cb: 48 85 c0                    	testq	%rax, %rax
1423441ce: 0f 85 6c ff ff ff           	jne	0x142344140 <.text+0x2343140>
1423441d4: 48 83 c1 04                 	addq	$0x4, %rcx
1423441d8: 48 39 d1                    	cmpq	%rdx, %rcx
1423441db: 75 e3                       	jne	0x1423441c0 <.text+0x23431c0>
1423441dd: 45 0f 28 d7                 	movaps	%xmm15, %xmm10
1423441e1: 45 0f 28 fe                 	movaps	%xmm14, %xmm15
1423441e5: 45 0f 28 f3                 	movaps	%xmm11, %xmm14
1423441e9: 45 0f 28 d8                 	movaps	%xmm8, %xmm11
1423441ed: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
1423441f1: 0f 28 fe                    	movaps	%xmm6, %xmm7
1423441f4: 41 0f 28 f4                 	movaps	%xmm12, %xmm6
1423441f8: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
1423441fd: f3 44 0f 10 60 2c           	movss	0x2c(%rax), %xmm12
142344203: f6 44 24 30 01              	testb	$0x1, 0x30(%rsp)
142344208: 75 3a                       	jne	0x142344244 <.text+0x2343244>
14234420a: f3 0f 16 cb                 	movshdup	%xmm3, %xmm1            # xmm1 = xmm3[1,1,3,3]
14234420e: f3 0f 11 4c 24 20           	movss	%xmm1, 0x20(%rsp)
142344214: 48 8d 9c 24 f0 00 00 00     	leaq	0xf0(%rsp), %rbx
14234421c: 48 89 d9                    	movq	%rbx, %rcx
14234421f: 0f 28 c8                    	movaps	%xmm0, %xmm1
142344222: e8 c9 bd ef fe              	callq	0x14123fff0 <.text+0x123eff0>
142344227: 48 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %rax
14234422f: f3 0f 10 08                 	movss	(%rax), %xmm1
142344233: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
142344238: 48 89 d9                    	movq	%rbx, %rcx
14234423b: e8 00 bf ef fe              	callq	0x141240140 <.text+0x123f140>
142344240: 84 c0                       	testb	%al, %al
142344242: 74 4f                       	je	0x142344293 <.text+0x2343293>
142344244: 48 8b 8c 24 a0 00 00 00     	movq	0xa0(%rsp), %rcx
14234424c: e8 8f b9 5f fe              	callq	0x14093fbe0 <.text+0x93ebe0>
142344251: 84 c0                       	testb	%al, %al
142344253: 74 3e                       	je	0x142344293 <.text+0x2343293>
142344255: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
14234425d: 8b 08                       	movl	(%rax), %ecx
14234425f: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
142344264: e8 07 b8 5f fe              	callq	0x14093fa70 <.text+0x93ea70>
142344269: 4c 8b 44 24 38              	movq	0x38(%rsp), %r8
14234426e: f3 41 0f 10 40 2c           	movss	0x2c(%r8), %xmm0
142344274: 84 c0                       	testb	%al, %al
142344276: 75 38                       	jne	0x1423442b0 <.text+0x23432b0>
142344278: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
142344280: f3 0f 10 48 58              	movss	0x58(%rax), %xmm1
142344285: f3 0f 5e 0d 73 16 5e 00     	divss	0x5e1673(%rip), %xmm1   # 0x142925900
14234428d: f3 0f 58 c1                 	addss	%xmm1, %xmm0
142344291: eb 1d                       	jmp	0x1423442b0 <.text+0x23432b0>
142344293: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
14234429b: f3 0f 10 40 58              	movss	0x58(%rax), %xmm0
1423442a0: f3 41 0f 5e c1              	divss	%xmm9, %xmm0
1423442a5: 4c 8b 44 24 38              	movq	0x38(%rsp), %r8
1423442aa: f3 41 0f 58 40 2c           	addss	0x2c(%r8), %xmm0
1423442b0: 0f 57 c9                    	xorps	%xmm1, %xmm1
1423442b3: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1423442b7: f3 0f 10 05 45 16 5e 00     	movss	0x5e1645(%rip), %xmm0   # 0x142925904
1423442bf: f3 0f 5d c1                 	minss	%xmm1, %xmm0
1423442c3: f3 41 0f 11 40 2c           	movss	%xmm0, 0x2c(%r8)
1423442c9: 0f 57 c9                    	xorps	%xmm1, %xmm1
1423442cc: 44 0f 2e e1                 	ucomiss	%xmm1, %xmm12
1423442d0: 0f 85 93 00 00 00           	jne	0x142344369 <.text+0x2343369>
1423442d6: 0f 8a 8d 00 00 00           	jp	0x142344369 <.text+0x2343369>
1423442dc: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
1423442df: 0f 86 84 00 00 00           	jbe	0x142344369 <.text+0x2343369>
1423442e5: 8b 44 24 48                 	movl	0x48(%rsp), %eax
1423442e9: 48 8b 4c 24 78              	movq	0x78(%rsp), %rcx
1423442ee: 89 01                       	movl	%eax, (%rcx)
1423442f0: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
1423442f5: 48 8b 58 30                 	movq	0x30(%rax), %rbx
1423442f9: 4c 8b 70 40                 	movq	0x40(%rax), %r14
1423442fd: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
142344301: 75 16                       	jne	0x142344319 <.text+0x2343319>
142344303: 48 8b 4c 24 70              	movq	0x70(%rsp), %rcx
142344308: 48 8d 15 61 0f 87 00        	leaq	0x870f61(%rip), %rdx    # 0x142bb5270
14234430f: e8 1c e9 21 fe              	callq	0x140562c30 <.text+0x561c30>
142344314: 4c 8b 44 24 38              	movq	0x38(%rsp), %r8
142344319: 48 8b 54 24 50              	movq	0x50(%rsp), %rdx
14234431e: 48 8b 42 28                 	movq	0x28(%rdx), %rax
142344322: 48 89 d9                    	movq	%rbx, %rcx
142344325: 48 c1 e1 05                 	shlq	$0x5, %rcx
142344329: c7 04 08 10 00 00 00        	movl	$0x10, (%rax,%rcx)
142344330: 4c 89 74 08 18              	movq	%r14, 0x18(%rax,%rcx)
142344335: 48 ff c3                    	incq	%rbx
142344338: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
14234433c: 48 ff 42 40                 	incq	0x40(%rdx)
142344340: 44 0f 28 e6                 	movaps	%xmm6, %xmm12
142344344: 0f 28 f7                    	movaps	%xmm7, %xmm6
142344347: 41 0f 28 f8                 	movaps	%xmm8, %xmm7
14234434b: 45 0f 28 c3                 	movaps	%xmm11, %xmm8
14234434f: 45 0f 28 de                 	movaps	%xmm14, %xmm11
142344353: 45 0f 28 f7                 	movaps	%xmm15, %xmm14
142344357: 45 0f 28 fa                 	movaps	%xmm10, %xmm15
14234435b: f3 44 0f 10 15 e4 f4 6c 00  	movss	0x6cf4e4(%rip), %xmm10  # 0x142a13848
142344364: e9 a7 00 00 00              	jmp	0x142344410 <.text+0x2343410>
142344369: 44 0f 2e 25 93 15 5e 00     	ucomiss	0x5e1593(%rip), %xmm12  # 0x142925904
142344371: 44 0f 28 e6                 	movaps	%xmm6, %xmm12
142344375: 0f 28 f7                    	movaps	%xmm7, %xmm6
142344378: 41 0f 28 f8                 	movaps	%xmm8, %xmm7
14234437c: 45 0f 28 c3                 	movaps	%xmm11, %xmm8
142344380: 45 0f 28 de                 	movaps	%xmm14, %xmm11
142344384: 45 0f 28 f7                 	movaps	%xmm15, %xmm14
142344388: 45 0f 28 fa                 	movaps	%xmm10, %xmm15
14234438c: f3 44 0f 10 15 b3 f4 6c 00  	movss	0x6cf4b3(%rip), %xmm10  # 0x142a13848
142344395: 75 79                       	jne	0x142344410 <.text+0x2343410>
142344397: 7a 77                       	jp	0x142344410 <.text+0x2343410>
142344399: f3 0f 10 0d 63 15 5e 00     	movss	0x5e1563(%rip), %xmm1   # 0x142925904
1423443a1: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1423443a4: 76 6a                       	jbe	0x142344410 <.text+0x2343410>
1423443a6: 8b 44 24 48                 	movl	0x48(%rsp), %eax
1423443aa: 48 8b 4c 24 78              	movq	0x78(%rsp), %rcx
1423443af: 89 01                       	movl	%eax, (%rcx)
1423443b1: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
1423443b6: 48 8b 58 30                 	movq	0x30(%rax), %rbx
1423443ba: 4c 8b 70 40                 	movq	0x40(%rax), %r14
1423443be: 48 3b 58 20                 	cmpq	0x20(%rax), %rbx
1423443c2: 75 16                       	jne	0x1423443da <.text+0x23433da>
1423443c4: 48 8b 4c 24 70              	movq	0x70(%rsp), %rcx
1423443c9: 48 8d 15 a0 0e 87 00        	leaq	0x870ea0(%rip), %rdx    # 0x142bb5270
1423443d0: e8 5b e8 21 fe              	callq	0x140562c30 <.text+0x561c30>
1423443d5: 4c 8b 44 24 38              	movq	0x38(%rsp), %r8
1423443da: 48 8b 54 24 50              	movq	0x50(%rsp), %rdx
1423443df: 48 8b 42 28                 	movq	0x28(%rdx), %rax
1423443e3: 48 89 d9                    	movq	%rbx, %rcx
1423443e6: 48 c1 e1 05                 	shlq	$0x5, %rcx
1423443ea: c7 04 08 11 00 00 00        	movl	$0x11, (%rax,%rcx)
1423443f1: 4c 89 74 08 18              	movq	%r14, 0x18(%rax,%rcx)
1423443f6: 48 ff c3                    	incq	%rbx
1423443f9: 48 89 5a 30                 	movq	%rbx, 0x30(%rdx)
1423443fd: 48 ff 42 40                 	incq	0x40(%rdx)
142344401: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
142344410: f3 41 0f 10 48 2c           	movss	0x2c(%r8), %xmm1
142344416: 48 8d 0d 4b a2 d0 00        	leaq	0xd0a24b(%rip), %rcx    # 0x14304e668
14234441d: 4c 89 c3                    	movq	%r8, %rbx
142344420: e8 cb 1a 95 fe              	callq	0x140c95ef0 <.text+0xc94ef0>
142344425: f3 45 0f 59 ea              	mulss	%xmm10, %xmm13
14234442a: f3 44 0f 59 e8              	mulss	%xmm0, %xmm13
14234442f: 48 8b 84 24 b0 00 00 00     	movq	0xb0(%rsp), %rax
142344437: 44 89 38                    	movl	%r15d, (%rax)
14234443a: f3 0f 7e 43 18              	movq	0x18(%rbx), %xmm0
14234443f: 48 8b 87 a0 00 00 00        	movq	0xa0(%rdi), %rax
142344446: 48 8b 8f a8 00 00 00        	movq	0xa8(%rdi), %rcx
14234444d: 48 c1 e1 03                 	shlq	$0x3, %rcx
142344451: 48 8d 14 89                 	leaq	(%rcx,%rcx,4), %rdx
142344455: 31 c9                       	xorl	%ecx, %ecx
142344457: 49 89 c0                    	movq	%rax, %r8
14234445a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
142344460: 48 39 ca                    	cmpq	%rcx, %rdx
142344463: 74 65                       	je	0x1423444ca <.text+0x23434ca>
142344465: f3 41 0f 7e 48 10           	movq	0x10(%r8), %xmm1
14234446b: 49 83 c0 28                 	addq	$0x28, %r8
14234446f: 66 0f 76 c8                 	pcmpeqd	%xmm0, %xmm1
142344473: 66 0f 70 c9 50              	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
142344478: 66 44 0f 50 c9              	movmskpd	%xmm1, %r9d
14234447d: 48 83 c1 28                 	addq	$0x28, %rcx
142344481: 41 83 f9 03                 	cmpl	$0x3, %r9d
142344485: 75 d9                       	jne	0x142344460 <.text+0x2343460>
142344487: 41 0f b6 95 f0 00 00 00     	movzbl	0xf0(%r13), %edx
14234448f: 66 0f ef c0                 	pxor	%xmm0, %xmm0
142344493: 4c 8d 05 2e a2 d0 00        	leaq	0xd0a22e(%rip), %r8     # 0x14304e6c8
14234449a: 49 63 14 90                 	movslq	(%r8,%rdx,4), %rdx
14234449e: 4c 01 c2                    	addq	%r8, %rdx
1423444a1: ff e2                       	jmpq	*%rdx
1423444a3: 44 0f 57 2d 45 14 5e 00     	xorps	0x5e1445(%rip), %xmm13  # 0x1429258f0
1423444ab: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1423444af: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
1423444b3: eb 08                       	jmp	0x1423444bd <.text+0x23434bd>
1423444b5: 44 0f 57 2d 33 14 5e 00     	xorps	0x5e1433(%rip), %xmm13  # 0x1429258f0
1423444bd: 66 0f 7e 44 08 e0           	movd	%xmm0, -0x20(%rax,%rcx)
1423444c3: f3 44 0f 11 6c 08 e4        	movss	%xmm13, -0x1c(%rax,%rcx)
1423444ca: 48 8b 84 24 e0 00 00 00     	movq	0xe0(%rsp), %rax
1423444d2: 48 8b 8c 24 e8 00 00 00     	movq	0xe8(%rsp), %rcx
1423444da: 4c 8d 84 24 c8 00 00 00     	leaq	0xc8(%rsp), %r8
1423444e2: e9 31 f7 ff ff              	jmp	0x142343c18 <.text+0x2342c18>
1423444e7: 0f 28 b4 24 00 01 00 00     	movaps	0x100(%rsp), %xmm6
1423444ef: 0f 28 bc 24 10 01 00 00     	movaps	0x110(%rsp), %xmm7
1423444f7: 44 0f 28 84 24 20 01 00 00  	movaps	0x120(%rsp), %xmm8
142344500: 44 0f 28 8c 24 30 01 00 00  	movaps	0x130(%rsp), %xmm9
142344509: 44 0f 28 94 24 40 01 00 00  	movaps	0x140(%rsp), %xmm10
142344512: 44 0f 28 9c 24 50 01 00 00  	movaps	0x150(%rsp), %xmm11
14234451b: 44 0f 28 a4 24 60 01 00 00  	movaps	0x160(%rsp), %xmm12
142344524: 44 0f 28 ac 24 70 01 00 00  	movaps	0x170(%rsp), %xmm13
14234452d: 44 0f 28 b4 24 80 01 00 00  	movaps	0x180(%rsp), %xmm14
142344536: 44 0f 28 bc 24 90 01 00 00  	movaps	0x190(%rsp), %xmm15
14234453f: 48 81 c4 a8 01 00 00        	addq	$0x1a8, %rsp            # imm = 0x1A8
142344546: 5b                          	popq	%rbx
142344547: 5d                          	popq	%rbp
142344548: 5f                          	popq	%rdi
142344549: 5e                          	popq	%rsi
14234454a: 41 5c                       	popq	%r12
14234454c: 41 5d                       	popq	%r13
14234454e: 41 5e                       	popq	%r14
142344550: 41 5f                       	popq	%r15
142344552: c3                          	retq
142344553: cc                          	int3
142344554: cc                          	int3
142344555: cc                          	int3
142344556: cc                          	int3
142344557: cc                          	int3
142344558: cc                          	int3
142344559: cc                          	int3
14234455a: cc                          	int3
14234455b: cc                          	int3
14234455c: cc                          	int3
14234455d: cc                          	int3
14234455e: cc                          	int3
14234455f: cc                          	int3
