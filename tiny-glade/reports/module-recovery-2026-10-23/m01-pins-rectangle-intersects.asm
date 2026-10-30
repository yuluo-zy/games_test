
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c9b990: 56                          	pushq	%rsi
140c9b991: 57                          	pushq	%rdi
140c9b992: 48 81 ec 78 01 00 00        	subq	$0x178, %rsp            # imm = 0x178
140c9b999: 44 0f 29 ac 24 60 01 00 00  	movaps	%xmm13, 0x160(%rsp)
140c9b9a2: 44 0f 29 a4 24 50 01 00 00  	movaps	%xmm12, 0x150(%rsp)
140c9b9ab: 44 0f 29 9c 24 40 01 00 00  	movaps	%xmm11, 0x140(%rsp)
140c9b9b4: 44 0f 29 94 24 30 01 00 00  	movaps	%xmm10, 0x130(%rsp)
140c9b9bd: 44 0f 29 8c 24 20 01 00 00  	movaps	%xmm9, 0x120(%rsp)
140c9b9c6: 44 0f 29 84 24 10 01 00 00  	movaps	%xmm8, 0x110(%rsp)
140c9b9cf: 0f 29 bc 24 00 01 00 00     	movaps	%xmm7, 0x100(%rsp)
140c9b9d7: 0f 29 b4 24 f0 00 00 00     	movaps	%xmm6, 0xf0(%rsp)
140c9b9df: f2 0f 10 71 10              	movsd	0x10(%rcx), %xmm6
140c9b9e4: 0f 28 c6                    	movaps	%xmm6, %xmm0
140c9b9e7: 0f 59 c6                    	mulps	%xmm6, %xmm0
140c9b9ea: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
140c9b9ee: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140c9b9f2: 0f 57 c0                    	xorps	%xmm0, %xmm0
140c9b9f5: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
140c9b9f9: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140c9b9fd: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
140c9ba01: 0f 82 4f 03 00 00           	jb	0x140c9bd56 <.text+0xc9ad56>
140c9ba07: f2 0f 10 02                 	movsd	(%rdx), %xmm0
140c9ba0b: f2 0f 10 4a 08              	movsd	0x8(%rdx), %xmm1
140c9ba10: 0f 28 d0                    	movaps	%xmm0, %xmm2
140c9ba13: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
140c9ba16: 0f 57 15 d3 9e c8 01        	xorps	0x1c89ed3(%rip), %xmm2  # 0x1429258f0
140c9ba1d: 0f 14 c9                    	unpcklps	%xmm1, %xmm1            # xmm1 = xmm1[0,0,1,1]
140c9ba20: f2 0f 10 21                 	movsd	(%rcx), %xmm4
140c9ba24: f2 0f 10 69 08              	movsd	0x8(%rcx), %xmm5
140c9ba29: 0f 14 ed                    	unpcklps	%xmm5, %xmm5            # xmm5 = xmm5[0,0,1,1]
140c9ba2c: f3 0f 16 dc                 	movshdup	%xmm4, %xmm3            # xmm3 = xmm4[1,1,3,3]
140c9ba30: 0f 57 1d b9 9e c8 01        	xorps	0x1c89eb9(%rip), %xmm3  # 0x1429258f0
140c9ba37: 0f 28 fc                    	movaps	%xmm4, %xmm7
140c9ba3a: 0f 16 fb                    	movlhps	%xmm3, %xmm7            # xmm7 = xmm7[0],xmm3[0]
140c9ba3d: 0f 28 dc                    	movaps	%xmm4, %xmm3
140c9ba40: 0f c6 df 24                 	shufps	$0x24, %xmm7, %xmm3     # xmm3 = xmm3[0,1],xmm7[2,0]
140c9ba44: 0f c6 ff 28                 	shufps	$0x28, %xmm7, %xmm7     # xmm7 = xmm7[0,2,2,0]
140c9ba48: 0f 59 fc                    	mulps	%xmm4, %xmm7
140c9ba4b: f3 0f 16 e7                 	movshdup	%xmm7, %xmm4            # xmm4 = xmm7[1,1,3,3]
140c9ba4f: f3 0f 5c fc                 	subss	%xmm4, %xmm7
140c9ba53: 0f c6 ff 00                 	shufps	$0x0, %xmm7, %xmm7      # xmm7 = xmm7[0,0,0,0]
140c9ba57: 0f 28 25 52 47 df 01        	movaps	0x1df4752(%rip), %xmm4  # 0x142a901b0
140c9ba5e: 0f 5e e7                    	divps	%xmm7, %xmm4
140c9ba61: 0f c6 db 27                 	shufps	$0x27, %xmm3, %xmm3     # xmm3 = xmm3[3,1,2,0]
140c9ba65: 0f 59 dc                    	mulps	%xmm4, %xmm3
140c9ba68: 0f 59 eb                    	mulps	%xmm3, %xmm5
140c9ba6b: 0f 28 e5                    	movaps	%xmm5, %xmm4
140c9ba6e: 66 0f 15 e5                 	unpckhpd	%xmm5, %xmm4            # xmm4 = xmm4[1],xmm5[1]
140c9ba72: 0f 58 e5                    	addps	%xmm5, %xmm4
140c9ba75: 0f c6 d0 0a                 	shufps	$0xa, %xmm0, %xmm2      # xmm2 = xmm2[2,2],xmm0[0,0]
140c9ba79: 0f 14 c0                    	unpcklps	%xmm0, %xmm0            # xmm0 = xmm0[0,0,1,1]
140c9ba7c: 0f 59 c3                    	mulps	%xmm3, %xmm0
140c9ba7f: 0f 59 d3                    	mulps	%xmm3, %xmm2
140c9ba82: 0f 28 e8                    	movaps	%xmm0, %xmm5
140c9ba85: 66 0f 15 ea                 	unpckhpd	%xmm2, %xmm5            # xmm5 = xmm5[1],xmm2[1]
140c9ba89: 0f 16 c2                    	movlhps	%xmm2, %xmm0            # xmm0 = xmm0[0],xmm2[0]
140c9ba8c: 0f 58 c5                    	addps	%xmm5, %xmm0
140c9ba8f: 0f 59 d9                    	mulps	%xmm1, %xmm3
140c9ba92: 0f 28 cb                    	movaps	%xmm3, %xmm1
140c9ba95: 66 0f 15 cb                 	unpckhpd	%xmm3, %xmm1            # xmm1 = xmm1[1],xmm3[1]
140c9ba99: 0f 58 cb                    	addps	%xmm3, %xmm1
140c9ba9c: 0f 5c cc                    	subps	%xmm4, %xmm1
140c9ba9f: 0f 29 84 24 d0 00 00 00     	movaps	%xmm0, 0xd0(%rsp)
140c9baa7: 0f 13 8c 24 e0 00 00 00     	movlps	%xmm1, 0xe0(%rsp)
140c9baaf: f2 0f 10 42 10              	movsd	0x10(%rdx), %xmm0
140c9bab4: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
140c9bab8: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
140c9babc: 41 0f 2e c8                 	ucomiss	%xmm8, %xmm1
140c9bac0: 0f 86 a8 02 00 00           	jbe	0x140c9bd6e <.text+0xc9ad6e>
140c9bac6: 0f c6 c0 11                 	shufps	$0x11, %xmm0, %xmm0     # xmm0 = xmm0[1,0,1,0]
140c9baca: 0f 59 05 8f a2 e0 01        	mulps	0x1e0a28f(%rip), %xmm0  # 0x142aa5d60
140c9bad1: 48 8d 84 24 d0 00 00 00     	leaq	0xd0(%rsp), %rax
140c9bad9: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
140c9bae1: f3 0f 16 c8                 	movshdup	%xmm0, %xmm1            # xmm1 = xmm0[1,1,3,3]
140c9bae5: 0f 28 d0                    	movaps	%xmm0, %xmm2
140c9bae8: 66 0f 15 d0                 	unpckhpd	%xmm0, %xmm2            # xmm2 = xmm2[1],xmm0[1]
140c9baec: 0f 28 d8                    	movaps	%xmm0, %xmm3
140c9baef: 0f c6 d8 91                 	shufps	$0x91, %xmm0, %xmm3     # xmm3 = xmm3[1,0],xmm0[1,2]
140c9baf3: 0f 11 9c 24 88 00 00 00     	movups	%xmm3, 0x88(%rsp)
140c9bafb: 0f 28 d8                    	movaps	%xmm0, %xmm3
140c9bafe: 0f c6 d8 ff                 	shufps	$0xff, %xmm0, %xmm3     # xmm3 = xmm3[3,3],xmm0[3,3]
140c9bb02: 0f 28 e0                    	movaps	%xmm0, %xmm4
140c9bb05: 0f c6 e0 3b                 	shufps	$0x3b, %xmm0, %xmm4     # xmm4 = xmm4[3,2],xmm0[3,0]
140c9bb09: 0f 11 a4 24 98 00 00 00     	movups	%xmm4, 0x98(%rsp)
140c9bb11: 48 c7 84 24 a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rsp)
140c9bb1d: 48 c7 84 24 b0 00 00 00 04 00 00 00 	movq	$0x4, 0xb0(%rsp)
140c9bb29: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
140c9bb2e: f3 0f 11 4c 24 50           	movss	%xmm1, 0x50(%rsp)
140c9bb34: 0f 11 44 24 54              	movups	%xmm0, 0x54(%rsp)
140c9bb39: f3 0f 11 54 24 64           	movss	%xmm2, 0x64(%rsp)
140c9bb3f: f3 0f 11 5c 24 68           	movss	%xmm3, 0x68(%rsp)
140c9bb45: f3 0f 11 44 24 6c           	movss	%xmm0, 0x6c(%rsp)
140c9bb4b: 48 c7 44 24 70 00 00 00 00  	movq	$0x0, 0x70(%rsp)
140c9bb54: 48 c7 44 24 78 04 00 00 00  	movq	$0x4, 0x78(%rsp)
140c9bb5d: c7 84 24 b8 00 00 00 00 00 00 00    	movl	$0x0, 0xb8(%rsp)
140c9bb68: 48 c7 44 24 40 04 00 00 00  	movq	$0x4, 0x40(%rsp)
140c9bb71: 48 8d 4c 24 2c              	leaq	0x2c(%rsp), %rcx
140c9bb76: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
140c9bb7b: e8 70 68 ca 00              	callq	0x1419423f0 <.text+0x19413f0>
140c9bb80: 83 7c 24 2c 01              	cmpl	$0x1, 0x2c(%rsp)
140c9bb85: 0f 85 79 01 00 00           	jne	0x140c9bd04 <.text+0xc9ad04>
140c9bb8b: f3 0f 10 05 dd 9c c8 01     	movss	0x1c89cdd(%rip), %xmm0  # 0x142925870
140c9bb93: f3 0f 16 fe                 	movshdup	%xmm6, %xmm7            # xmm7 = xmm6[1,1,3,3]
140c9bb97: f3 0f 59 f0                 	mulss	%xmm0, %xmm6
140c9bb9b: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
140c9bb9f: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
140c9bba3: f3 44 0f 5c ce              	subss	%xmm6, %xmm9
140c9bba8: 45 0f 57 d2                 	xorps	%xmm10, %xmm10
140c9bbac: f3 44 0f 5c d7              	subss	%xmm7, %xmm10
140c9bbb1: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
140c9bbb6: f3 41 0f 58 f8              	addss	%xmm8, %xmm7
140c9bbbb: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140c9bbbf: f3 44 0f 10 1d 3c 9d c8 01  	movss	0x1c89d3c(%rip), %xmm11 # 0x142925904
140c9bbc8: 48 8d 74 24 2c              	leaq	0x2c(%rsp), %rsi
140c9bbcd: 48 8d 7c 24 40              	leaq	0x40(%rsp), %rdi
140c9bbd2: eb 22                       	jmp	0x140c9bbf6 <.text+0xc9abf6>
140c9bbd4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140c9bbe0: 48 89 f1                    	movq	%rsi, %rcx
140c9bbe3: 48 89 fa                    	movq	%rdi, %rdx
140c9bbe6: e8 05 68 ca 00              	callq	0x1419423f0 <.text+0x19413f0>
140c9bbeb: f6 44 24 2c 01              	testb	$0x1, 0x2c(%rsp)
140c9bbf0: 0f 84 0e 01 00 00           	je	0x140c9bd04 <.text+0xc9ad04>
140c9bbf6: f3 0f 10 44 24 30           	movss	0x30(%rsp), %xmm0
140c9bbfc: f3 0f 10 4c 24 38           	movss	0x38(%rsp), %xmm1
140c9bc02: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140c9bc06: 0f 28 de                    	movaps	%xmm6, %xmm3
140c9bc09: f3 0f 5c d8                 	subss	%xmm0, %xmm3
140c9bc0d: f3 0f 5e d9                 	divss	%xmm1, %xmm3
140c9bc11: 41 0f 28 e1                 	movaps	%xmm9, %xmm4
140c9bc15: f3 0f 5c e0                 	subss	%xmm0, %xmm4
140c9bc19: f3 0f 5e e1                 	divss	%xmm1, %xmm4
140c9bc1d: 0f 2e db                    	ucomiss	%xmm3, %xmm3
140c9bc20: 0f 28 c4                    	movaps	%xmm4, %xmm0
140c9bc23: 7a 07                       	jp	0x140c9bc2c <.text+0xc9ac2c>
140c9bc25: 0f 28 c4                    	movaps	%xmm4, %xmm0
140c9bc28: f3 0f 5f c3                 	maxss	%xmm3, %xmm0
140c9bc2c: f3 0f 10 4c 24 34           	movss	0x34(%rsp), %xmm1
140c9bc32: f3 0f 10 54 24 3c           	movss	0x3c(%rsp), %xmm2
140c9bc38: f3 0f 5c d1                 	subss	%xmm1, %xmm2
140c9bc3c: 0f 28 ef                    	movaps	%xmm7, %xmm5
140c9bc3f: f3 0f 5c e9                 	subss	%xmm1, %xmm5
140c9bc43: f3 0f 5e ea                 	divss	%xmm2, %xmm5
140c9bc47: 45 0f 28 e2                 	movaps	%xmm10, %xmm12
140c9bc4b: f3 44 0f 5c e1              	subss	%xmm1, %xmm12
140c9bc50: f3 44 0f 5e e2              	divss	%xmm2, %xmm12
140c9bc55: 0f 2e ed                    	ucomiss	%xmm5, %xmm5
140c9bc58: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
140c9bc5c: 7a 08                       	jp	0x140c9bc66 <.text+0xc9ac66>
140c9bc5e: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
140c9bc62: f3 0f 5f d5                 	maxss	%xmm5, %xmm2
140c9bc66: 0f 28 cb                    	movaps	%xmm3, %xmm1
140c9bc69: f3 0f c2 cb 03              	cmpunordss	%xmm3, %xmm1
140c9bc6e: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
140c9bc72: 44 0f 54 ec                 	andps	%xmm4, %xmm13
140c9bc76: f3 0f 5d e3                 	minss	%xmm3, %xmm4
140c9bc7a: 0f 55 cc                    	andnps	%xmm4, %xmm1
140c9bc7d: 41 0f 56 cd                 	orps	%xmm13, %xmm1
140c9bc81: 0f 28 dd                    	movaps	%xmm5, %xmm3
140c9bc84: f3 0f c2 dd 03              	cmpunordss	%xmm5, %xmm3
140c9bc89: 0f 28 e3                    	movaps	%xmm3, %xmm4
140c9bc8c: 41 0f 54 e4                 	andps	%xmm12, %xmm4
140c9bc90: f3 44 0f 5d e5              	minss	%xmm5, %xmm12
140c9bc95: 41 0f 55 dc                 	andnps	%xmm12, %xmm3
140c9bc99: 0f 56 dc                    	orps	%xmm4, %xmm3
140c9bc9c: 0f 28 e3                    	movaps	%xmm3, %xmm4
140c9bc9f: f3 0f 5f e1                 	maxss	%xmm1, %xmm4
140c9bca3: f3 0f c2 c9 03              	cmpunordss	%xmm1, %xmm1
140c9bca8: 0f 28 e9                    	movaps	%xmm1, %xmm5
140c9bcab: 0f 54 eb                    	andps	%xmm3, %xmm5
140c9bcae: 0f 55 cc                    	andnps	%xmm4, %xmm1
140c9bcb1: 0f 56 cd                    	orps	%xmm5, %xmm1
140c9bcb4: 0f 28 d8                    	movaps	%xmm0, %xmm3
140c9bcb7: f3 0f c2 d8 03              	cmpunordss	%xmm0, %xmm3
140c9bcbc: 0f 28 e3                    	movaps	%xmm3, %xmm4
140c9bcbf: 0f 54 e2                    	andps	%xmm2, %xmm4
140c9bcc2: f3 0f 5d d0                 	minss	%xmm0, %xmm2
140c9bcc6: 0f 55 da                    	andnps	%xmm2, %xmm3
140c9bcc9: 0f 56 dc                    	orps	%xmm4, %xmm3
140c9bccc: 0f 2e d9                    	ucomiss	%xmm1, %xmm3
140c9bccf: 0f 82 0b ff ff ff           	jb	0x140c9bbe0 <.text+0xc9abe0>
140c9bcd5: 41 0f 2e d8                 	ucomiss	%xmm8, %xmm3
140c9bcd9: 0f 82 01 ff ff ff           	jb	0x140c9bbe0 <.text+0xc9abe0>
140c9bcdf: 44 0f 2e db                 	ucomiss	%xmm3, %xmm11
140c9bce3: 73 1b                       	jae	0x140c9bd00 <.text+0xc9ad00>
140c9bce5: 0f 57 c0                    	xorps	%xmm0, %xmm0
140c9bce8: 0f c2 c1 02                 	cmpleps	%xmm1, %xmm0
140c9bcec: 41 0f c2 cb 02              	cmpleps	%xmm11, %xmm1
140c9bcf1: 0f 54 c8                    	andps	%xmm0, %xmm1
140c9bcf4: 66 0f 7e c8                 	movd	%xmm1, %eax
140c9bcf8: a8 01                       	testb	$0x1, %al
140c9bcfa: 0f 84 e0 fe ff ff           	je	0x140c9bbe0 <.text+0xc9abe0>
140c9bd00: b0 01                       	movb	$0x1, %al
140c9bd02: eb 02                       	jmp	0x140c9bd06 <.text+0xc9ad06>
140c9bd04: 31 c0                       	xorl	%eax, %eax
140c9bd06: 0f 28 b4 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm6
140c9bd0e: 0f 28 bc 24 00 01 00 00     	movaps	0x100(%rsp), %xmm7
140c9bd16: 44 0f 28 84 24 10 01 00 00  	movaps	0x110(%rsp), %xmm8
140c9bd1f: 44 0f 28 8c 24 20 01 00 00  	movaps	0x120(%rsp), %xmm9
140c9bd28: 44 0f 28 94 24 30 01 00 00  	movaps	0x130(%rsp), %xmm10
140c9bd31: 44 0f 28 9c 24 40 01 00 00  	movaps	0x140(%rsp), %xmm11
140c9bd3a: 44 0f 28 a4 24 50 01 00 00  	movaps	0x150(%rsp), %xmm12
140c9bd43: 44 0f 28 ac 24 60 01 00 00  	movaps	0x160(%rsp), %xmm13
140c9bd4c: 48 81 c4 78 01 00 00        	addq	$0x178, %rsp            # imm = 0x178
140c9bd53: 5f                          	popq	%rdi
140c9bd54: 5e                          	popq	%rsi
140c9bd55: c3                          	retq
140c9bd56: 48 8d 0d 9b 37 f8 01        	leaq	0x1f8379b(%rip), %rcx   # 0x142c1f4f8
140c9bd5d: 4c 8d 05 dc 37 f8 01        	leaq	0x1f837dc(%rip), %r8    # 0x142c1f540
140c9bd64: ba 26 00 00 00              	movl	$0x26, %edx
140c9bd69: e8 c2 d6 c3 01              	callq	0x1428d9430 <.text+0x28d8430>
140c9bd6e: 48 8d 0d f7 28 e9 01        	leaq	0x1e928f7(%rip), %rcx   # 0x142b2e66c
140c9bd75: 4c 8d 05 3c 29 e9 01        	leaq	0x1e9293c(%rip), %r8    # 0x142b2e6b8
