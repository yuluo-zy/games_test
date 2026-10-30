
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14075dc60: 55                          	pushq	%rbp
14075dc61: 41 57                       	pushq	%r15
14075dc63: 41 56                       	pushq	%r14
14075dc65: 41 55                       	pushq	%r13
14075dc67: 41 54                       	pushq	%r12
14075dc69: 56                          	pushq	%rsi
14075dc6a: 57                          	pushq	%rdi
14075dc6b: 53                          	pushq	%rbx
14075dc6c: 48 81 ec a8 01 00 00        	subq	$0x1a8, %rsp            # imm = 0x1A8
14075dc73: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14075dc7b: 44 0f 29 bd 10 01 00 00     	movaps	%xmm15, 0x110(%rbp)
14075dc83: 44 0f 29 b5 00 01 00 00     	movaps	%xmm14, 0x100(%rbp)
14075dc8b: 66 44 0f 7f ad f0 00 00 00  	movdqa	%xmm13, 0xf0(%rbp)
14075dc94: 44 0f 29 a5 e0 00 00 00     	movaps	%xmm12, 0xe0(%rbp)
14075dc9c: 44 0f 29 9d d0 00 00 00     	movaps	%xmm11, 0xd0(%rbp)
14075dca4: 44 0f 29 95 c0 00 00 00     	movaps	%xmm10, 0xc0(%rbp)
14075dcac: 44 0f 29 8d b0 00 00 00     	movaps	%xmm9, 0xb0(%rbp)
14075dcb4: 44 0f 29 85 a0 00 00 00     	movaps	%xmm8, 0xa0(%rbp)
14075dcbc: 0f 29 bd 90 00 00 00        	movaps	%xmm7, 0x90(%rbp)
14075dcc3: 0f 29 b5 80 00 00 00        	movaps	%xmm6, 0x80(%rbp)
14075dcca: 48 c7 45 78 fe ff ff ff     	movq	$-0x2, 0x78(%rbp)
14075dcd2: 48 89 d7                    	movq	%rdx, %rdi
14075dcd5: 48 89 ce                    	movq	%rcx, %rsi
14075dcd8: 8b 82 58 02 00 00           	movl	0x258(%rdx), %eax
14075dcde: 83 f8 02                    	cmpl	$0x2, %eax
14075dce1: b9 02 00 00 00              	movl	$0x2, %ecx
14075dce6: 0f 42 c8                    	cmovbl	%eax, %ecx
14075dce9: f6 c1 01                    	testb	$0x1, %cl
14075dcec: 0f 84 99 00 00 00           	je	0x14075dd8b <.text+0x75cd8b>
14075dcf2: 4c 89 c3                    	movq	%r8, %rbx
14075dcf5: 48 8b 87 6c 02 00 00        	movq	0x26c(%rdi), %rax
14075dcfc: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14075dd00: 0f 10 87 5c 02 00 00        	movups	0x25c(%rdi), %xmm0
14075dd07: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
14075dd0b: 4d 85 c0                    	testq	%r8, %r8
14075dd0e: 0f 84 1c 01 00 00           	je	0x14075de30 <.text+0x75ce30>
14075dd14: 48 89 d9                    	movq	%rbx, %rcx
14075dd17: e8 94 56 18 00              	callq	0x1408e33b0 <.text+0x8e23b0>
14075dd1c: 84 c0                       	testb	%al, %al
14075dd1e: 0f 84 0c 01 00 00           	je	0x14075de30 <.text+0x75ce30>
14075dd24: 44 0f b6 73 3c              	movzbl	0x3c(%rbx), %r14d
14075dd29: 48 89 f9                    	movq	%rdi, %rcx
14075dd2c: e8 ef c8 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075dd31: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
14075dd35: 45 84 f6                    	testb	%r14b, %r14b
14075dd38: 0f 84 7b 07 00 00           	je	0x14075e4b9 <.text+0x75d4b9>
14075dd3e: 48 89 f9                    	movq	%rdi, %rcx
14075dd41: e8 da c8 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075dd46: 0f 28 f0                    	movaps	%xmm0, %xmm6
14075dd49: 48 89 d9                    	movq	%rbx, %rcx
14075dd4c: e8 0f 55 18 00              	callq	0x1408e3260 <.text+0x8e2260>
14075dd51: 0f 28 f8                    	movaps	%xmm0, %xmm7
14075dd54: f3 0f 58 fe                 	addss	%xmm6, %xmm7
14075dd58: 48 89 f9                    	movq	%rdi, %rcx
14075dd5b: e8 c0 c8 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075dd60: f3 0f 11 45 5c              	movss	%xmm0, 0x5c(%rbp)
14075dd65: 48 89 f9                    	movq	%rdi, %rcx
14075dd68: e8 b3 c8 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075dd6d: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
14075dd71: 48 89 d9                    	movq	%rbx, %rcx
14075dd74: e8 e7 54 18 00              	callq	0x1408e3260 <.text+0x8e2260>
14075dd79: 0f 28 f0                    	movaps	%xmm0, %xmm6
14075dd7c: f3 41 0f 58 f2              	addss	%xmm10, %xmm6
14075dd81: b3 01                       	movb	$0x1, %bl
14075dd83: 45 31 ff                    	xorl	%r15d, %r15d
14075dd86: e9 c1 00 00 00              	jmp	0x14075de4c <.text+0x75ce4c>
14075dd8b: 48 89 f9                    	movq	%rdi, %rcx
14075dd8e: e8 8d c8 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075dd93: 0f 28 c8                    	movaps	%xmm0, %xmm1
14075dd96: f3 0f 5c 8f 80 02 00 00     	subss	0x280(%rdi), %xmm1
14075dd9e: f3 0f 10 77 30              	movss	0x30(%rdi), %xmm6
14075dda3: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
14075ddad: 48 89 06                    	movq	%rax, (%rsi)
14075ddb0: f3 0f 11 46 08              	movss	%xmm0, 0x8(%rsi)
14075ddb5: f3 0f 11 4e 0c              	movss	%xmm1, 0xc(%rsi)
14075ddba: 0f 14 f1                    	unpcklps	%xmm1, %xmm6            # xmm6 = xmm6[0],xmm1[0],xmm6[1],xmm1[1]
14075ddbd: 0f 5e 35 fc f4 26 02        	divps	0x226f4fc(%rip), %xmm6  # 0x1429cd2c0
14075ddc4: 0f 28 c6                    	movaps	%xmm6, %xmm0
14075ddc7: e8 d4 61 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075ddcc: 0f 28 f8                    	movaps	%xmm0, %xmm7
14075ddcf: f3 0f 16 c6                 	movshdup	%xmm6, %xmm0            # xmm0 = xmm6[1,1,3,3]
14075ddd3: e8 c8 61 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075ddd8: 0f 57 c9                    	xorps	%xmm1, %xmm1
14075dddb: 0f 57 d2                    	xorps	%xmm2, %xmm2
14075ddde: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
14075dde2: f3 0f 10 05 62 ef 31 02     	movss	0x231ef62(%rip), %xmm0  # 0x142a7cd4c
14075ddea: 0f 28 d8                    	movaps	%xmm0, %xmm3
14075dded: f3 0f 5d da                 	minss	%xmm2, %xmm3
14075ddf1: f3 0f 2c c3                 	cvttss2si	%xmm3, %eax
14075ddf5: f3 0f 5f cf                 	maxss	%xmm7, %xmm1
14075ddf9: f3 0f 5d c1                 	minss	%xmm1, %xmm0
14075ddfd: f3 0f 2c c8                 	cvttss2si	%xmm0, %ecx
14075de01: 66 0f 6e c1                 	movd	%ecx, %xmm0
14075de05: 66 0f c4 c0 01              	pinsrw	$0x1, %eax, %xmm0
14075de0a: 66 0f 6f 0d 3e ef 31 02     	movdqa	0x231ef3e(%rip), %xmm1  # 0x142a7cd50
14075de12: 66 0f d9 c8                 	psubusw	%xmm0, %xmm1
14075de16: 66 0f fd c8                 	paddw	%xmm0, %xmm1
14075de1a: 66 0f 7e 4e 10              	movd	%xmm1, 0x10(%rsi)
14075de1f: c6 46 14 00                 	movb	$0x0, 0x14(%rsi)
14075de23: 48 c7 46 60 01 00 00 00     	movq	$0x1, 0x60(%rsi)
14075de2b: e9 24 06 00 00              	jmp	0x14075e454 <.text+0x75d454>
14075de30: 48 89 f9                    	movq	%rdi, %rcx
14075de33: e8 e8 c7 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075de38: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
14075de3c: 45 31 ff                    	xorl	%r15d, %r15d
14075de3f: 0f 28 f8                    	movaps	%xmm0, %xmm7
14075de42: f3 0f 11 45 5c              	movss	%xmm0, 0x5c(%rbp)
14075de47: 0f 28 f0                    	movaps	%xmm0, %xmm6
14075de4a: 31 db                       	xorl	%ebx, %ebx
14075de4c: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
14075de54: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075de58: 31 d2                       	xorl	%edx, %edx
14075de5a: e8 71 b8 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075de5f: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
14075de63: f3 44 0f 58 15 f4 ee 31 02  	addss	0x231eef4(%rip), %xmm10 # 0x142a7cd60
14075de6c: f3 44 0f 11 55 74           	movss	%xmm10, 0x74(%rbp)
14075de72: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075de75: 44 0f 2e d0                 	ucomiss	%xmm0, %xmm10
14075de79: 0f 82 81 06 00 00           	jb	0x14075e500 <.text+0x75d500>
14075de7f: f3 44 0f 10 35 7c 7a 1c 02  	movss	0x21c7a7c(%rip), %xmm14 # 0x142925904
14075de88: 45 0f 2e f2                 	ucomiss	%xmm10, %xmm14
14075de8c: 0f 82 6e 06 00 00           	jb	0x14075e500 <.text+0x75d500>
14075de92: 45 0f 2e d2                 	ucomiss	%xmm10, %xmm10
14075de96: 0f 8a b7 06 00 00           	jp	0x14075e553 <.text+0x75d553>
14075de9c: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075dea0: ba 01 00 00 00              	movl	$0x1, %edx
14075dea5: e8 26 b8 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075deaa: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
14075deae: f3 44 0f 58 1d ad ee 31 02  	addss	0x231eead(%rip), %xmm11 # 0x142a7cd64
14075deb7: f3 44 0f 11 5d 74           	movss	%xmm11, 0x74(%rbp)
14075debd: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075dec0: 44 0f 2e d8                 	ucomiss	%xmm0, %xmm11
14075dec4: 0f 82 a6 06 00 00           	jb	0x14075e570 <.text+0x75d570>
14075deca: 45 0f 2e f3                 	ucomiss	%xmm11, %xmm14
14075dece: 0f 82 9c 06 00 00           	jb	0x14075e570 <.text+0x75d570>
14075ded4: 45 0f 2e db                 	ucomiss	%xmm11, %xmm11
14075ded8: 0f 8a e5 06 00 00           	jp	0x14075e5c3 <.text+0x75d5c3>
14075dede: 45 0f 28 c8                 	movaps	%xmm8, %xmm9
14075dee2: f3 44 0f 5c 8f 80 02 00 00  	subss	0x280(%rdi), %xmm9
14075deeb: 45 0f 28 eb                 	movaps	%xmm11, %xmm13
14075deef: f3 45 0f 5c ea              	subss	%xmm10, %xmm13
14075def4: f3 44 0f 59 6f 30           	mulss	0x30(%rdi), %xmm13
14075defa: 4c 8d 65 08                 	leaq	0x8(%rbp), %r12
14075defe: 45 0f 14 e9                 	unpcklps	%xmm9, %xmm13           # xmm13 = xmm13[0],xmm9[0],xmm13[1],xmm9[1]
14075df02: 44 0f 5e 2d b6 f3 26 02     	divps	0x226f3b6(%rip), %xmm13 # 0x1429cd2c0
14075df0a: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
14075df0e: e8 8d 60 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075df13: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
14075df17: f3 41 0f 16 c5              	movshdup	%xmm13, %xmm0           # xmm0 = xmm13[1,1,3,3]
14075df1c: e8 7f 60 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075df21: 44 0f 28 e8                 	movaps	%xmm0, %xmm13
14075df25: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14075df29: 48 83 fa 05                 	cmpq	$0x5, %rdx
14075df2d: 4c 8d 75 a8                 	leaq	-0x58(%rbp), %r14
14075df31: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
14075df35: 49 0f 42 c6                 	cmovbq	%r14, %rax
14075df39: 4c 8d 6d b0                 	leaq	-0x50(%rbp), %r13
14075df3d: 4c 89 e1                    	movq	%r12, %rcx
14075df40: 49 0f 43 cd                 	cmovaeq	%r13, %rcx
14075df44: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075df4a: 4c 0f 43 c2                 	cmovaeq	%rdx, %r8
14075df4e: 4c 89 e2                    	movq	%r12, %rdx
14075df51: 49 0f 43 d5                 	cmovaeq	%r13, %rdx
14075df55: 48 8b 12                    	movq	(%rdx), %rdx
14075df58: 4c 39 c2                    	cmpq	%r8, %rdx
14075df5b: 0f 84 7f 06 00 00           	je	0x14075e5e0 <.text+0x75d5e0>
14075df61: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075df64: 0f 57 c9                    	xorps	%xmm1, %xmm1
14075df67: f3 41 0f 5f cd              	maxss	%xmm13, %xmm1
14075df6c: f3 44 0f 10 3d d7 ed 31 02  	movss	0x231edd7(%rip), %xmm15 # 0x142a7cd4c
14075df75: 41 0f 28 d7                 	movaps	%xmm15, %xmm2
14075df79: f3 0f 5d d1                 	minss	%xmm1, %xmm2
14075df7d: f3 44 0f 2c c2              	cvttss2si	%xmm2, %r8d
14075df82: f3 41 0f 5f c4              	maxss	%xmm12, %xmm0
14075df87: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
14075df8b: f3 0f 5d c8                 	minss	%xmm0, %xmm1
14075df8f: f3 44 0f 2c c9              	cvttss2si	%xmm1, %r9d
14075df94: 66 41 0f 6e c1              	movd	%r9d, %xmm0
14075df99: 66 41 0f c4 c0 01           	pinsrw	$0x1, %r8d, %xmm0
14075df9f: 66 44 0f 6f 2d a8 ed 31 02  	movdqa	0x231eda8(%rip), %xmm13 # 0x142a7cd50
14075dfa8: 66 41 0f 6f cd              	movdqa	%xmm13, %xmm1
14075dfad: 66 0f d9 c8                 	psubusw	%xmm0, %xmm1
14075dfb1: 66 0f fd c8                 	paddw	%xmm0, %xmm1
14075dfb5: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14075dfb9: f3 44 0f 11 14 d0           	movss	%xmm10, (%rax,%rdx,8)
14075dfbf: f3 44 0f 11 5c d0 04        	movss	%xmm11, 0x4(%rax,%rdx,8)
14075dfc6: f3 44 0f 11 44 d0 08        	movss	%xmm8, 0x8(%rax,%rdx,8)
14075dfcd: f3 44 0f 11 4c d0 0c        	movss	%xmm9, 0xc(%rax,%rdx,8)
14075dfd4: 66 0f 7e 4c d0 10           	movd	%xmm1, 0x10(%rax,%rdx,8)
14075dfda: 44 88 7c d0 14              	movb	%r15b, 0x14(%rax,%rdx,8)
14075dfdf: 48 ff 01                    	incq	(%rcx)
14075dfe2: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075dfe6: ba 01 00 00 00              	movl	$0x1, %edx
14075dfeb: e8 e0 b6 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075dff0: 66 44 0f 6f c8              	movdqa	%xmm0, %xmm9
14075dff5: f3 44 0f 58 0d 62 ed 31 02  	addss	0x231ed62(%rip), %xmm9  # 0x142a7cd60
14075dffe: f3 44 0f 11 4d 74           	movss	%xmm9, 0x74(%rbp)
14075e004: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14075e008: 44 0f 2e c8                 	ucomiss	%xmm0, %xmm9
14075e00c: 0f 82 e7 05 00 00           	jb	0x14075e5f9 <.text+0x75d5f9>
14075e012: 45 0f 2e f1                 	ucomiss	%xmm9, %xmm14
14075e016: 0f 82 dd 05 00 00           	jb	0x14075e5f9 <.text+0x75d5f9>
14075e01c: 45 0f 2e c9                 	ucomiss	%xmm9, %xmm9
14075e020: 0f 8a 26 06 00 00           	jp	0x14075e64c <.text+0x75d64c>
14075e026: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075e02a: ba 02 00 00 00              	movl	$0x2, %edx
14075e02f: e8 9c b6 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075e034: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
14075e038: f3 44 0f 58 15 23 ed 31 02  	addss	0x231ed23(%rip), %xmm10 # 0x142a7cd64
14075e041: f3 44 0f 11 55 74           	movss	%xmm10, 0x74(%rbp)
14075e047: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075e04a: 44 0f 2e d0                 	ucomiss	%xmm0, %xmm10
14075e04e: 0f 82 15 06 00 00           	jb	0x14075e669 <.text+0x75d669>
14075e054: 45 0f 2e f2                 	ucomiss	%xmm10, %xmm14
14075e058: 0f 82 0b 06 00 00           	jb	0x14075e669 <.text+0x75d669>
14075e05e: 45 0f 2e d2                 	ucomiss	%xmm10, %xmm10
14075e062: 0f 8a 54 06 00 00           	jp	0x14075e6bc <.text+0x75d6bc>
14075e068: 44 0f 28 c7                 	movaps	%xmm7, %xmm8
14075e06c: f3 44 0f 5c 87 80 02 00 00  	subss	0x280(%rdi), %xmm8
14075e075: 45 0f 28 e2                 	movaps	%xmm10, %xmm12
14075e079: f3 45 0f 5c e1              	subss	%xmm9, %xmm12
14075e07e: f3 44 0f 59 67 30           	mulss	0x30(%rdi), %xmm12
14075e084: 45 0f 14 e0                 	unpcklps	%xmm8, %xmm12           # xmm12 = xmm12[0],xmm8[0],xmm12[1],xmm8[1]
14075e088: 44 0f 5e 25 30 f2 26 02     	divps	0x226f230(%rip), %xmm12 # 0x1429cd2c0
14075e090: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
14075e094: e8 07 5f 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075e099: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
14075e09d: f3 41 0f 16 c4              	movshdup	%xmm12, %xmm0           # xmm0 = xmm12[1,1,3,3]
14075e0a2: e8 f9 5e 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075e0a7: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14075e0ab: 48 83 fa 05                 	cmpq	$0x5, %rdx
14075e0af: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
14075e0b3: 49 0f 42 c6                 	cmovbq	%r14, %rax
14075e0b7: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
14075e0bb: 4c 89 e1                    	movq	%r12, %rcx
14075e0be: 49 0f 43 cd                 	cmovaeq	%r13, %rcx
14075e0c2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075e0c8: 4c 0f 43 c2                 	cmovaeq	%rdx, %r8
14075e0cc: 4c 89 e2                    	movq	%r12, %rdx
14075e0cf: 49 0f 43 d5                 	cmovaeq	%r13, %rdx
14075e0d3: 48 8b 12                    	movq	(%rdx), %rdx
14075e0d6: 4c 39 c2                    	cmpq	%r8, %rdx
14075e0d9: 0f 84 fa 05 00 00           	je	0x14075e6d9 <.text+0x75d6d9>
14075e0df: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075e0e2: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14075e0e6: f3 41 0f 5f cc              	maxss	%xmm12, %xmm1
14075e0eb: 41 0f 28 d7                 	movaps	%xmm15, %xmm2
14075e0ef: f3 0f 5d d1                 	minss	%xmm1, %xmm2
14075e0f3: f3 44 0f 2c c2              	cvttss2si	%xmm2, %r8d
14075e0f8: f3 41 0f 5f c3              	maxss	%xmm11, %xmm0
14075e0fd: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
14075e101: f3 0f 5d c8                 	minss	%xmm0, %xmm1
14075e105: f3 44 0f 2c c9              	cvttss2si	%xmm1, %r9d
14075e10a: 66 41 0f 6e c1              	movd	%r9d, %xmm0
14075e10f: 66 41 0f c4 c0 01           	pinsrw	$0x1, %r8d, %xmm0
14075e115: 66 41 0f 6f cd              	movdqa	%xmm13, %xmm1
14075e11a: 66 0f d9 c8                 	psubusw	%xmm0, %xmm1
14075e11e: 66 0f fd c8                 	paddw	%xmm0, %xmm1
14075e122: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14075e126: f3 44 0f 11 0c d0           	movss	%xmm9, (%rax,%rdx,8)
14075e12c: f3 44 0f 11 54 d0 04        	movss	%xmm10, 0x4(%rax,%rdx,8)
14075e133: f3 0f 11 7c d0 08           	movss	%xmm7, 0x8(%rax,%rdx,8)
14075e139: f3 44 0f 11 44 d0 0c        	movss	%xmm8, 0xc(%rax,%rdx,8)
14075e140: 66 0f 7e 4c d0 10           	movd	%xmm1, 0x10(%rax,%rdx,8)
14075e146: 88 5c d0 14                 	movb	%bl, 0x14(%rax,%rdx,8)
14075e14a: 48 ff 01                    	incq	(%rcx)
14075e14d: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075e151: ba 02 00 00 00              	movl	$0x2, %edx
14075e156: e8 75 b5 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075e15b: 66 44 0f 6f c0              	movdqa	%xmm0, %xmm8
14075e160: f3 44 0f 58 05 f7 eb 31 02  	addss	0x231ebf7(%rip), %xmm8  # 0x142a7cd60
14075e169: f3 44 0f 11 45 74           	movss	%xmm8, 0x74(%rbp)
14075e16f: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14075e173: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
14075e177: 0f 82 75 05 00 00           	jb	0x14075e6f2 <.text+0x75d6f2>
14075e17d: 45 0f 2e f0                 	ucomiss	%xmm8, %xmm14
14075e181: 0f 82 6b 05 00 00           	jb	0x14075e6f2 <.text+0x75d6f2>
14075e187: 45 0f 2e c0                 	ucomiss	%xmm8, %xmm8
14075e18b: 0f 8a b4 05 00 00           	jp	0x14075e745 <.text+0x75d745>
14075e191: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075e195: ba 03 00 00 00              	movl	$0x3, %edx
14075e19a: e8 31 b5 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075e19f: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
14075e1a3: f3 44 0f 58 0d b8 eb 31 02  	addss	0x231ebb8(%rip), %xmm9  # 0x142a7cd64
14075e1ac: f3 44 0f 11 4d 74           	movss	%xmm9, 0x74(%rbp)
14075e1b2: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075e1b5: 44 0f 2e c8                 	ucomiss	%xmm0, %xmm9
14075e1b9: 0f 82 a3 05 00 00           	jb	0x14075e762 <.text+0x75d762>
14075e1bf: 45 0f 2e f1                 	ucomiss	%xmm9, %xmm14
14075e1c3: 0f 82 99 05 00 00           	jb	0x14075e762 <.text+0x75d762>
14075e1c9: 45 0f 2e c9                 	ucomiss	%xmm9, %xmm9
14075e1cd: 0f 8a e2 05 00 00           	jp	0x14075e7b5 <.text+0x75d7b5>
14075e1d3: f3 44 0f 10 65 5c           	movss	0x5c(%rbp), %xmm12
14075e1d9: 41 0f 28 fc                 	movaps	%xmm12, %xmm7
14075e1dd: f3 0f 5c bf 80 02 00 00     	subss	0x280(%rdi), %xmm7
14075e1e5: 45 0f 28 d9                 	movaps	%xmm9, %xmm11
14075e1e9: f3 45 0f 5c d8              	subss	%xmm8, %xmm11
14075e1ee: f3 44 0f 59 5f 30           	mulss	0x30(%rdi), %xmm11
14075e1f4: 44 0f 14 df                 	unpcklps	%xmm7, %xmm11           # xmm11 = xmm11[0],xmm7[0],xmm11[1],xmm7[1]
14075e1f8: 44 0f 5e 1d c0 f0 26 02     	divps	0x226f0c0(%rip), %xmm11 # 0x1429cd2c0
14075e200: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
14075e204: e8 97 5d 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075e209: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
14075e20d: f3 41 0f 16 c3              	movshdup	%xmm11, %xmm0           # xmm0 = xmm11[1,1,3,3]
14075e212: e8 89 5d 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075e217: 48 8b 55 08                 	movq	0x8(%rbp), %rdx
14075e21b: 48 83 fa 05                 	cmpq	$0x5, %rdx
14075e21f: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
14075e223: 49 0f 42 c6                 	cmovbq	%r14, %rax
14075e227: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
14075e22b: 4c 89 e1                    	movq	%r12, %rcx
14075e22e: 49 0f 43 cd                 	cmovaeq	%r13, %rcx
14075e232: 41 b8 04 00 00 00           	movl	$0x4, %r8d
14075e238: 4c 0f 43 c2                 	cmovaeq	%rdx, %r8
14075e23c: 4c 89 e2                    	movq	%r12, %rdx
14075e23f: 49 0f 43 d5                 	cmovaeq	%r13, %rdx
14075e243: 48 8b 12                    	movq	(%rdx), %rdx
14075e246: 4c 39 c2                    	cmpq	%r8, %rdx
14075e249: 0f 84 83 05 00 00           	je	0x14075e7d2 <.text+0x75d7d2>
14075e24f: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075e252: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14075e256: f3 41 0f 5f cb              	maxss	%xmm11, %xmm1
14075e25b: 41 0f 28 d7                 	movaps	%xmm15, %xmm2
14075e25f: f3 0f 5d d1                 	minss	%xmm1, %xmm2
14075e263: f3 44 0f 2c c2              	cvttss2si	%xmm2, %r8d
14075e268: f3 41 0f 5f c2              	maxss	%xmm10, %xmm0
14075e26d: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
14075e271: f3 0f 5d c8                 	minss	%xmm0, %xmm1
14075e275: f3 44 0f 2c c9              	cvttss2si	%xmm1, %r9d
14075e27a: 66 41 0f 6e c1              	movd	%r9d, %xmm0
14075e27f: 66 41 0f c4 c0 01           	pinsrw	$0x1, %r8d, %xmm0
14075e285: 66 41 0f 6f cd              	movdqa	%xmm13, %xmm1
14075e28a: 66 0f d9 c8                 	psubusw	%xmm0, %xmm1
14075e28e: 66 0f fd c8                 	paddw	%xmm0, %xmm1
14075e292: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14075e296: f3 44 0f 11 04 d0           	movss	%xmm8, (%rax,%rdx,8)
14075e29c: f3 44 0f 11 4c d0 04        	movss	%xmm9, 0x4(%rax,%rdx,8)
14075e2a3: f3 44 0f 11 64 d0 08        	movss	%xmm12, 0x8(%rax,%rdx,8)
14075e2aa: f3 0f 11 7c d0 0c           	movss	%xmm7, 0xc(%rax,%rdx,8)
14075e2b0: 66 0f 7e 4c d0 10           	movd	%xmm1, 0x10(%rax,%rdx,8)
14075e2b6: 44 88 7c d0 14              	movb	%r15b, 0x14(%rax,%rdx,8)
14075e2bb: 48 ff 01                    	incq	(%rcx)
14075e2be: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075e2c2: ba 03 00 00 00              	movl	$0x3, %edx
14075e2c7: e8 04 b4 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075e2cc: 66 0f 6f f8                 	movdqa	%xmm0, %xmm7
14075e2d0: f3 0f 58 3d 88 ea 31 02     	addss	0x231ea88(%rip), %xmm7  # 0x142a7cd60
14075e2d8: f3 0f 11 7d 74              	movss	%xmm7, 0x74(%rbp)
14075e2dd: 66 0f ef c0                 	pxor	%xmm0, %xmm0
14075e2e1: 0f 2e f8                    	ucomiss	%xmm0, %xmm7
14075e2e4: 0f 82 07 05 00 00           	jb	0x14075e7f1 <.text+0x75d7f1>
14075e2ea: 44 0f 2e f7                 	ucomiss	%xmm7, %xmm14
14075e2ee: 0f 82 fd 04 00 00           	jb	0x14075e7f1 <.text+0x75d7f1>
14075e2f4: 0f 2e ff                    	ucomiss	%xmm7, %xmm7
14075e2f7: 0f 8a 47 05 00 00           	jp	0x14075e844 <.text+0x75d844>
14075e2fd: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
14075e301: ba 04 00 00 00              	movl	$0x4, %edx
14075e306: e8 c5 b3 53 00              	callq	0x140c996d0 <.text+0xc986d0>
14075e30b: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
14075e30f: f3 44 0f 58 05 4c ea 31 02  	addss	0x231ea4c(%rip), %xmm8  # 0x142a7cd64
14075e318: f3 44 0f 11 45 74           	movss	%xmm8, 0x74(%rbp)
14075e31e: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075e321: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
14075e325: 0f 82 33 05 00 00           	jb	0x14075e85e <.text+0x75d85e>
14075e32b: 45 0f 2e f0                 	ucomiss	%xmm8, %xmm14
14075e32f: 0f 82 29 05 00 00           	jb	0x14075e85e <.text+0x75d85e>
14075e335: 45 0f 2e c0                 	ucomiss	%xmm8, %xmm8
14075e339: 0f 8a 6f 05 00 00           	jp	0x14075e8ae <.text+0x75d8ae>
14075e33f: 44 0f 28 de                 	movaps	%xmm6, %xmm11
14075e343: f3 44 0f 5c 9f 80 02 00 00  	subss	0x280(%rdi), %xmm11
14075e34c: 45 0f 28 d0                 	movaps	%xmm8, %xmm10
14075e350: f3 44 0f 5c d7              	subss	%xmm7, %xmm10
14075e355: f3 44 0f 59 57 30           	mulss	0x30(%rdi), %xmm10
14075e35b: 45 0f 14 d3                 	unpcklps	%xmm11, %xmm10          # xmm10 = xmm10[0],xmm11[0],xmm10[1],xmm11[1]
14075e35f: 44 0f 5e 15 59 ef 26 02     	divps	0x226ef59(%rip), %xmm10 # 0x1429cd2c0
14075e367: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
14075e36b: e8 30 5c 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075e370: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
14075e374: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
14075e379: e8 22 5c 1c 02              	callq	0x142923fa0 <.text+0x2922fa0>
14075e37e: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
14075e382: 48 83 f9 05                 	cmpq	$0x5, %rcx
14075e386: 4c 0f 43 75 a8              	cmovaeq	-0x58(%rbp), %r14
14075e38b: 4c 89 e0                    	movq	%r12, %rax
14075e38e: 49 0f 43 c5                 	cmovaeq	%r13, %rax
14075e392: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
14075e396: ba 04 00 00 00              	movl	$0x4, %edx
14075e39b: 48 0f 43 d1                 	cmovaeq	%rcx, %rdx
14075e39f: 4d 0f 43 e5                 	cmovaeq	%r13, %r12
14075e3a3: 49 8b 0c 24                 	movq	(%r12), %rcx
14075e3a7: 48 39 d1                    	cmpq	%rdx, %rcx
14075e3aa: 0f 84 18 05 00 00           	je	0x14075e8c8 <.text+0x75d8c8>
14075e3b0: 0f 57 c0                    	xorps	%xmm0, %xmm0
14075e3b3: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14075e3b7: f3 41 0f 5f ca              	maxss	%xmm10, %xmm1
14075e3bc: 41 0f 28 d7                 	movaps	%xmm15, %xmm2
14075e3c0: f3 0f 5d d1                 	minss	%xmm1, %xmm2
14075e3c4: f3 0f 2c d2                 	cvttss2si	%xmm2, %edx
14075e3c8: f3 41 0f 5f c1              	maxss	%xmm9, %xmm0
14075e3cd: f3 44 0f 5d f8              	minss	%xmm0, %xmm15
14075e3d2: f3 45 0f 2c c7              	cvttss2si	%xmm15, %r8d
14075e3d7: 66 41 0f 6e c0              	movd	%r8d, %xmm0
14075e3dc: 66 0f c4 c2 01              	pinsrw	$0x1, %edx, %xmm0
14075e3e1: 66 44 0f d9 e8              	psubusw	%xmm0, %xmm13
14075e3e6: 66 44 0f fd e8              	paddw	%xmm0, %xmm13
14075e3eb: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14075e3ef: f3 41 0f 11 3c ce           	movss	%xmm7, (%r14,%rcx,8)
14075e3f5: f3 45 0f 11 44 ce 04        	movss	%xmm8, 0x4(%r14,%rcx,8)
14075e3fc: f3 41 0f 11 74 ce 08        	movss	%xmm6, 0x8(%r14,%rcx,8)
14075e403: f3 45 0f 11 5c ce 0c        	movss	%xmm11, 0xc(%r14,%rcx,8)
14075e40a: 66 45 0f 7e 6c ce 10        	movd	%xmm13, 0x10(%r14,%rcx,8)
14075e411: 41 88 5c ce 14              	movb	%bl, 0x14(%r14,%rcx,8)
14075e416: 48 ff 00                    	incq	(%rax)
14075e419: 48 8b 45 08                 	movq	0x8(%rbp), %rax
14075e41d: 48 89 46 60                 	movq	%rax, 0x60(%rsi)
14075e421: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
14075e425: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
14075e429: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
14075e42d: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
14075e431: f3 0f 6f 45 a8              	movdqu	-0x58(%rbp), %xmm0
14075e436: f3 0f 6f 4d b8              	movdqu	-0x48(%rbp), %xmm1
14075e43b: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
14075e43f: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
14075e443: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
14075e447: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
14075e44b: f3 0f 7f 4e 10              	movdqu	%xmm1, 0x10(%rsi)
14075e450: f3 0f 7f 06                 	movdqu	%xmm0, (%rsi)
14075e454: 48 89 f0                    	movq	%rsi, %rax
14075e457: 0f 28 b5 80 00 00 00        	movaps	0x80(%rbp), %xmm6
14075e45e: 0f 28 bd 90 00 00 00        	movaps	0x90(%rbp), %xmm7
14075e465: 44 0f 28 85 a0 00 00 00     	movaps	0xa0(%rbp), %xmm8
14075e46d: 44 0f 28 8d b0 00 00 00     	movaps	0xb0(%rbp), %xmm9
14075e475: 44 0f 28 95 c0 00 00 00     	movaps	0xc0(%rbp), %xmm10
14075e47d: 44 0f 28 9d d0 00 00 00     	movaps	0xd0(%rbp), %xmm11
14075e485: 44 0f 28 a5 e0 00 00 00     	movaps	0xe0(%rbp), %xmm12
14075e48d: 44 0f 28 ad f0 00 00 00     	movaps	0xf0(%rbp), %xmm13
14075e495: 44 0f 28 b5 00 01 00 00     	movaps	0x100(%rbp), %xmm14
14075e49d: 44 0f 28 bd 10 01 00 00     	movaps	0x110(%rbp), %xmm15
14075e4a5: 48 81 c4 a8 01 00 00        	addq	$0x1a8, %rsp            # imm = 0x1A8
14075e4ac: 5b                          	popq	%rbx
14075e4ad: 5f                          	popq	%rdi
14075e4ae: 5e                          	popq	%rsi
14075e4af: 41 5c                       	popq	%r12
14075e4b1: 41 5d                       	popq	%r13
14075e4b3: 41 5e                       	popq	%r14
14075e4b5: 41 5f                       	popq	%r15
14075e4b7: 5d                          	popq	%rbp
14075e4b8: c3                          	retq
14075e4b9: 48 89 d9                    	movq	%rbx, %rcx
14075e4bc: e8 9f 4d 18 00              	callq	0x1408e3260 <.text+0x8e2260>
14075e4c1: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
14075e4c6: 48 89 f9                    	movq	%rdi, %rcx
14075e4c9: e8 52 c1 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075e4ce: 0f 28 f8                    	movaps	%xmm0, %xmm7
14075e4d1: 48 89 f9                    	movq	%rdi, %rcx
14075e4d4: e8 47 c1 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075e4d9: 0f 28 f0                    	movaps	%xmm0, %xmm6
14075e4dc: 48 89 d9                    	movq	%rbx, %rcx
14075e4df: e8 7c 4d 18 00              	callq	0x1408e3260 <.text+0x8e2260>
14075e4e4: f3 0f 58 c6                 	addss	%xmm6, %xmm0
14075e4e8: f3 0f 11 45 5c              	movss	%xmm0, 0x5c(%rbp)
14075e4ed: 48 89 f9                    	movq	%rdi, %rcx
14075e4f0: e8 2b c1 3c 00              	callq	0x140b2a620 <.text+0xb29620>
14075e4f5: 0f 28 f0                    	movaps	%xmm0, %xmm6
14075e4f8: 41 b7 01                    	movb	$0x1, %r15b
14075e4fb: e9 4a f9 ff ff              	jmp	0x14075de4a <.text+0x75ce4a>
14075e500: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e504: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e508: 48 8d 05 01 e4 62 00        	leaq	0x62e401(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e50f: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e513: 48 8d 05 be c0 4b 02        	leaq	0x24bc0be(%rip), %rax   # 0x142c1a5d8
14075e51a: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e51e: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e526: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e52e: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e532: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e536: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
14075e53e: 48 8d 15 3b ec 31 02        	leaq	0x231ec3b(%rip), %rdx   # 0x142a7d180
14075e545: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14075e549: e8 42 ae 17 02              	callq	0x1428d9390 <.text+0x28d8390>
14075e54e: e9 73 03 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e553: 48 8d 0d 8e c0 4b 02        	leaq	0x24bc08e(%rip), %rcx   # 0x142c1a5e8
14075e55a: 4c 8d 05 1f ec 31 02        	leaq	0x231ec1f(%rip), %r8    # 0x142a7d180
14075e561: ba 1d 00 00 00              	movl	$0x1d, %edx
14075e566: e8 c5 ae 17 02              	callq	0x1428d9430 <.text+0x28d8430>
14075e56b: e9 56 03 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e570: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e574: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e578: 48 8d 05 91 e3 62 00        	leaq	0x62e391(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e57f: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e583: 48 8d 05 4e c0 4b 02        	leaq	0x24bc04e(%rip), %rax   # 0x142c1a5d8
14075e58a: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e58e: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e596: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e59e: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e5a2: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e5a6: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
14075e5ae: 48 8d 15 e3 eb 31 02        	leaq	0x231ebe3(%rip), %rdx   # 0x142a7d198
14075e5b5: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14075e5b9: e8 d2 ad 17 02              	callq	0x1428d9390 <.text+0x28d8390>
14075e5be: e9 03 03 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e5c3: 48 8d 0d 1e c0 4b 02        	leaq	0x24bc01e(%rip), %rcx   # 0x142c1a5e8
14075e5ca: 4c 8d 05 c7 eb 31 02        	leaq	0x231ebc7(%rip), %r8    # 0x142a7d198
14075e5d1: ba 1d 00 00 00              	movl	$0x1d, %edx
14075e5d6: e8 55 ae 17 02              	callq	0x1428d9430 <.text+0x28d8430>
14075e5db: e9 e6 02 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e5e0: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
14075e5e4: e8 b7 3c 19 02              	callq	0x1428f22a0 <.text+0x28f12a0>
14075e5e9: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
14075e5ed: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
14075e5f1: 4c 89 e9                    	movq	%r13, %rcx
14075e5f4: e9 68 f9 ff ff              	jmp	0x14075df61 <.text+0x75cf61>
14075e5f9: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e5fd: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e601: 48 8d 05 08 e3 62 00        	leaq	0x62e308(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e608: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e60c: 48 8d 05 c5 bf 4b 02        	leaq	0x24bbfc5(%rip), %rax   # 0x142c1a5d8
14075e613: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e617: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e61f: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e627: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e62b: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e62f: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
14075e637: 48 8d 15 72 eb 31 02        	leaq	0x231eb72(%rip), %rdx   # 0x142a7d1b0
14075e63e: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14075e642: e8 49 ad 17 02              	callq	0x1428d9390 <.text+0x28d8390>
14075e647: e9 7a 02 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e64c: 48 8d 0d 95 bf 4b 02        	leaq	0x24bbf95(%rip), %rcx   # 0x142c1a5e8
14075e653: 4c 8d 05 56 eb 31 02        	leaq	0x231eb56(%rip), %r8    # 0x142a7d1b0
14075e65a: ba 1d 00 00 00              	movl	$0x1d, %edx
14075e65f: e8 cc ad 17 02              	callq	0x1428d9430 <.text+0x28d8430>
14075e664: e9 5d 02 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e669: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e66d: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e671: 48 8d 05 98 e2 62 00        	leaq	0x62e298(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e678: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e67c: 48 8d 05 55 bf 4b 02        	leaq	0x24bbf55(%rip), %rax   # 0x142c1a5d8
14075e683: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e687: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e68f: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e697: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e69b: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e69f: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
14075e6a7: 48 8d 15 1a eb 31 02        	leaq	0x231eb1a(%rip), %rdx   # 0x142a7d1c8
14075e6ae: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14075e6b2: e8 d9 ac 17 02              	callq	0x1428d9390 <.text+0x28d8390>
14075e6b7: e9 0a 02 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e6bc: 48 8d 0d 25 bf 4b 02        	leaq	0x24bbf25(%rip), %rcx   # 0x142c1a5e8
14075e6c3: 4c 8d 05 fe ea 31 02        	leaq	0x231eafe(%rip), %r8    # 0x142a7d1c8
14075e6ca: ba 1d 00 00 00              	movl	$0x1d, %edx
14075e6cf: e8 5c ad 17 02              	callq	0x1428d9430 <.text+0x28d8430>
14075e6d4: e9 ed 01 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e6d9: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
14075e6dd: e8 be 3b 19 02              	callq	0x1428f22a0 <.text+0x28f12a0>
14075e6e2: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
14075e6e6: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
14075e6ea: 4c 89 e9                    	movq	%r13, %rcx
14075e6ed: e9 ed f9 ff ff              	jmp	0x14075e0df <.text+0x75d0df>
14075e6f2: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e6f6: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e6fa: 48 8d 05 0f e2 62 00        	leaq	0x62e20f(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e701: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e705: 48 8d 05 cc be 4b 02        	leaq	0x24bbecc(%rip), %rax   # 0x142c1a5d8
14075e70c: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e710: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e718: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e720: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e724: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e728: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
14075e730: 48 8d 15 a9 ea 31 02        	leaq	0x231eaa9(%rip), %rdx   # 0x142a7d1e0
14075e737: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14075e73b: e8 50 ac 17 02              	callq	0x1428d9390 <.text+0x28d8390>
14075e740: e9 81 01 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e745: 48 8d 0d 9c be 4b 02        	leaq	0x24bbe9c(%rip), %rcx   # 0x142c1a5e8
14075e74c: 4c 8d 05 8d ea 31 02        	leaq	0x231ea8d(%rip), %r8    # 0x142a7d1e0
14075e753: ba 1d 00 00 00              	movl	$0x1d, %edx
14075e758: e8 d3 ac 17 02              	callq	0x1428d9430 <.text+0x28d8430>
14075e75d: e9 64 01 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e762: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e766: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e76a: 48 8d 05 9f e1 62 00        	leaq	0x62e19f(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e771: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e775: 48 8d 05 5c be 4b 02        	leaq	0x24bbe5c(%rip), %rax   # 0x142c1a5d8
14075e77c: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e780: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e788: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e790: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e794: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e798: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
14075e7a0: 48 8d 15 51 ea 31 02        	leaq	0x231ea51(%rip), %rdx   # 0x142a7d1f8
14075e7a7: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14075e7ab: e8 e0 ab 17 02              	callq	0x1428d9390 <.text+0x28d8390>
14075e7b0: e9 11 01 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e7b5: 48 8d 0d 2c be 4b 02        	leaq	0x24bbe2c(%rip), %rcx   # 0x142c1a5e8
14075e7bc: 4c 8d 05 35 ea 31 02        	leaq	0x231ea35(%rip), %r8    # 0x142a7d1f8
14075e7c3: ba 1d 00 00 00              	movl	$0x1d, %edx
14075e7c8: e8 63 ac 17 02              	callq	0x1428d9430 <.text+0x28d8430>
14075e7cd: e9 f4 00 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e7d2: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
14075e7d6: e8 c5 3a 19 02              	callq	0x1428f22a0 <.text+0x28f12a0>
14075e7db: 48 8b 45 a8                 	movq	-0x58(%rbp), %rax
14075e7df: 48 8b 55 b0                 	movq	-0x50(%rbp), %rdx
14075e7e3: 4c 89 e9                    	movq	%r13, %rcx
14075e7e6: f3 44 0f 10 65 5c           	movss	0x5c(%rbp), %xmm12
14075e7ec: e9 5e fa ff ff              	jmp	0x14075e24f <.text+0x75d24f>
14075e7f1: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e7f5: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e7f9: 48 8d 05 10 e1 62 00        	leaq	0x62e110(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e800: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e804: 48 8d 05 cd bd 4b 02        	leaq	0x24bbdcd(%rip), %rax   # 0x142c1a5d8
14075e80b: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e80f: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e817: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e81f: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e823: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e827: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
14075e82f: 48 8d 15 da e9 31 02        	leaq	0x231e9da(%rip), %rdx   # 0x142a7d210
14075e836: 48 8d 4d 28                 	leaq	0x28(%rbp), %rcx
14075e83a: e8 51 ab 17 02              	callq	0x1428d9390 <.text+0x28d8390>
14075e83f: e9 82 00 00 00              	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e844: 48 8d 0d 9d bd 4b 02        	leaq	0x24bbd9d(%rip), %rcx   # 0x142c1a5e8
14075e84b: 4c 8d 05 be e9 31 02        	leaq	0x231e9be(%rip), %r8    # 0x142a7d210
14075e852: ba 1d 00 00 00              	movl	$0x1d, %edx
14075e857: e8 d4 ab 17 02              	callq	0x1428d9430 <.text+0x28d8430>
14075e85c: eb 68                       	jmp	0x14075e8c6 <.text+0x75d8c6>
14075e85e: 48 8d 45 74                 	leaq	0x74(%rbp), %rax
14075e862: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
14075e866: 48 8d 05 a3 e0 62 00        	leaq	0x62e0a3(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
14075e86d: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
14075e871: 48 8d 05 60 bd 4b 02        	leaq	0x24bbd60(%rip), %rax   # 0x142c1a5d8
14075e878: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
14075e87c: 48 c7 45 30 01 00 00 00     	movq	$0x1, 0x30(%rbp)
14075e884: 48 c7 45 48 00 00 00 00     	movq	$0x0, 0x48(%rbp)
14075e88c: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
14075e890: 48 89 45 38                 	movq	%rax, 0x38(%rbp)
14075e894: 48 c7 45 40 01 00 00 00     	movq	$0x1, 0x40(%rbp)
