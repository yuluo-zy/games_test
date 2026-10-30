
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407fdf90: 41 57                       	pushq	%r15
1407fdf92: 41 56                       	pushq	%r14
1407fdf94: 41 55                       	pushq	%r13
1407fdf96: 41 54                       	pushq	%r12
1407fdf98: 56                          	pushq	%rsi
1407fdf99: 57                          	pushq	%rdi
1407fdf9a: 55                          	pushq	%rbp
1407fdf9b: 53                          	pushq	%rbx
1407fdf9c: 48 81 ec 68 02 00 00        	subq	$0x268, %rsp            # imm = 0x268
1407fdfa3: 44 0f 29 bc 24 50 02 00 00  	movaps	%xmm15, 0x250(%rsp)
1407fdfac: 44 0f 29 b4 24 40 02 00 00  	movaps	%xmm14, 0x240(%rsp)
1407fdfb5: 44 0f 29 ac 24 30 02 00 00  	movaps	%xmm13, 0x230(%rsp)
1407fdfbe: 44 0f 29 a4 24 20 02 00 00  	movaps	%xmm12, 0x220(%rsp)
1407fdfc7: 44 0f 29 9c 24 10 02 00 00  	movaps	%xmm11, 0x210(%rsp)
1407fdfd0: 44 0f 29 94 24 00 02 00 00  	movaps	%xmm10, 0x200(%rsp)
1407fdfd9: 44 0f 29 8c 24 f0 01 00 00  	movaps	%xmm9, 0x1f0(%rsp)
1407fdfe2: 44 0f 29 84 24 e0 01 00 00  	movaps	%xmm8, 0x1e0(%rsp)
1407fdfeb: 0f 29 bc 24 d0 01 00 00     	movaps	%xmm7, 0x1d0(%rsp)
1407fdff3: 0f 29 b4 24 c0 01 00 00     	movaps	%xmm6, 0x1c0(%rsp)
1407fdffb: f3 0f 11 9c 24 b4 00 00 00  	movss	%xmm3, 0xb4(%rsp)
1407fe004: 4c 89 44 24 58              	movq	%r8, 0x58(%rsp)
1407fe009: 89 54 24 34                 	movl	%edx, 0x34(%rsp)
1407fe00d: 48 8b 9c 24 d8 02 00 00     	movq	0x2d8(%rsp), %rbx
1407fe015: 48 85 db                    	testq	%rbx, %rbx
1407fe018: 0f 84 e9 0a 00 00           	je	0x1407feb07 <.text+0x7fdb07>
1407fe01e: 49 89 ce                    	movq	%rcx, %r14
1407fe021: 48 8b bc 24 d0 02 00 00     	movq	0x2d0(%rsp), %rdi
1407fe029: 48 8d 04 5b                 	leaq	(%rbx,%rbx,2), %rax
1407fe02d: 48 8d 04 87                 	leaq	(%rdi,%rax,4), %rax
1407fe031: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
1407fe039: 48 83 44 24 58 58           	addq	$0x58, 0x58(%rsp)
1407fe03f: 45 31 ed                    	xorl	%r13d, %r13d
1407fe042: 48 be 2f 64 bd 78 64 1d 76 a0       	movabsq	$-0x5f89e29b87429bd1, %rsi # imm = 0xA0761D6478BD642F
1407fe04c: 49 bc db 28 b4 a0 d1 7e 03 e7       	movabsq	$-0x18fc812e5f4bd725, %r12 # imm = 0xE7037ED1A0B428DB
1407fe056: 48 89 fd                    	movq	%rdi, %rbp
1407fe059: e9 85 00 00 00              	jmp	0x1407fe0e3 <.text+0x7fd0e3>
1407fe05e: 66 90                       	nop
1407fe060: 48 83 c5 0c                 	addq	$0xc, %rbp
1407fe064: 89 d0                       	movl	%edx, %eax
1407fe066: 48 69 c0 e8 03 00 00        	imulq	$0x3e8, %rax, %rax      # imm = 0x3E8
1407fe06d: 48 c1 e8 20                 	shrq	$0x20, %rax
1407fe071: 0f 10 84 24 30 01 00 00     	movups	0x130(%rsp), %xmm0
1407fe079: 0f 10 8c 24 40 01 00 00     	movups	0x140(%rsp), %xmm1
1407fe081: 0f 10 94 24 50 01 00 00     	movups	0x150(%rsp), %xmm2
1407fe089: 0f 29 94 24 80 00 00 00     	movaps	%xmm2, 0x80(%rsp)
1407fe091: 0f 29 4c 24 70              	movaps	%xmm1, 0x70(%rsp)
1407fe096: 0f 29 44 24 60              	movaps	%xmm0, 0x60(%rsp)
1407fe09b: c7 84 24 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rsp)
1407fe0a6: 89 84 24 94 00 00 00        	movl	%eax, 0x94(%rsp)
1407fe0ad: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1407fe0b1: 89 84 24 98 00 00 00        	movl	%eax, 0x98(%rsp)
1407fe0b8: c7 84 24 9c 00 00 00 18 00 00 00    	movl	$0x18, 0x9c(%rsp)
1407fe0c3: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1407fe0c8: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
1407fe0cd: e8 0e f0 a8 00              	callq	0x14128d0e0 <.text+0x128c0e0>
1407fe0d2: 4d 89 fd                    	movq	%r15, %r13
1407fe0d5: 48 3b ac 24 a0 00 00 00     	cmpq	0xa0(%rsp), %rbp
1407fe0dd: 0f 84 e4 02 00 00           	je	0x1407fe3c7 <.text+0x7fd3c7>
1407fe0e3: f3 44 0f 10 75 04           	movss	0x4(%rbp), %xmm14
1407fe0e9: f3 44 0f 58 35 62 02 28 02  	addss	0x2280262(%rip), %xmm14 # 0x142a7e354
1407fe0f2: f3 0f 10 75 08              	movss	0x8(%rbp), %xmm6
1407fe0f7: f3 44 0f 10 7d 00           	movss	(%rbp), %xmm15
1407fe0fd: 4c 89 f1                    	movq	%r14, %rcx
1407fe100: e8 7b 12 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe105: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1407fe109: 4c 89 f1                    	movq	%r14, %rcx
1407fe10c: e8 6f 12 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe111: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1407fe115: 4c 89 f1                    	movq	%r14, %rcx
1407fe118: e8 63 12 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe11d: 4c 3b ac 24 e8 02 00 00     	cmpq	0x2e8(%rsp), %r13
1407fe125: 0f 84 48 0a 00 00           	je	0x1407feb73 <.text+0x7fdb73>
1407fe12b: 0f 28 f8                    	movaps	%xmm0, %xmm7
1407fe12e: 4d 8d 7d 01                 	leaq	0x1(%r13), %r15
1407fe132: f3 0f 58 3d ae bf 15 02     	addss	0x215bfae(%rip), %xmm7  # 0x14295a0e8
1407fe13a: 45 0f 14 c1                 	unpcklps	%xmm9, %xmm8            # xmm8 = xmm8[0],xmm9[0],xmm8[1],xmm9[1]
1407fe13e: 44 0f 58 05 aa 90 21 02     	addps	0x22190aa(%rip), %xmm8  # 0x142a171f0
1407fe146: 48 8b 84 24 e0 02 00 00     	movq	0x2e0(%rsp), %rax
1407fe14e: f3 42 0f 10 04 a8           	movss	(%rax,%r13,4), %xmm0
1407fe154: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1407fe158: f3 44 0f 12 c8              	movsldup	%xmm0, %xmm9            # xmm9 = xmm0[0,0,2,2]
1407fe15d: 45 0f 59 c8                 	mulps	%xmm8, %xmm9
1407fe161: 44 0f 59 0d f7 18 28 02     	mulps	0x22818f7(%rip), %xmm9  # 0x142a7fa60
1407fe169: f3 0f 59 3d 67 77 12 02     	mulss	0x2127767(%rip), %xmm7  # 0x1429258d8
1407fe171: 44 0f 58 0d 37 64 13 02     	addps	0x2136437(%rip), %xmm9  # 0x1429345b0
1407fe179: 49 39 df                    	cmpq	%rbx, %r15
1407fe17c: 73 14                       	jae	0x1407fe192 <.text+0x7fd192>
1407fe17e: 4b 8d 04 7f                 	leaq	(%r15,%r15,2), %rax
1407fe182: f3 44 0f 10 2c 87           	movss	(%rdi,%rax,4), %xmm13
1407fe188: f3 0f 10 44 87 08           	movss	0x8(%rdi,%rax,4), %xmm0
1407fe18e: 44 0f 14 e8                 	unpcklps	%xmm0, %xmm13           # xmm13 = xmm13[0],xmm0[0],xmm13[1],xmm0[1]
1407fe192: 44 0f 14 fe                 	unpcklps	%xmm6, %xmm15           # xmm15 = xmm15[0],xmm6[0],xmm15[1],xmm6[1]
1407fe196: 44 0f 58 3d 02 78 12 02     	addps	0x2127802(%rip), %xmm15 # 0x1429259a0
1407fe19e: f3 0f 58 3d ea 24 16 02     	addss	0x21624ea(%rip), %xmm7  # 0x142960690
1407fe1a6: 49 83 ed 01                 	subq	$0x1, %r13
1407fe1aa: b8 00 00 00 00              	movl	$0x0, %eax
1407fe1af: 49 0f 43 c5                 	cmovaeq	%r13, %rax
1407fe1b3: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407fe1b6: 48 39 d8                    	cmpq	%rbx, %rax
1407fe1b9: 73 12                       	jae	0x1407fe1cd <.text+0x7fd1cd>
1407fe1bb: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1407fe1bf: f3 0f 10 04 87              	movss	(%rdi,%rax,4), %xmm0
1407fe1c4: f3 0f 10 4c 87 08           	movss	0x8(%rdi,%rax,4), %xmm1
1407fe1ca: 0f 14 c1                    	unpcklps	%xmm1, %xmm0            # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
1407fe1cd: 31 c0                       	xorl	%eax, %eax
1407fe1cf: 49 39 df                    	cmpq	%rbx, %r15
1407fe1d2: 0f 92 c0                    	setb	%al
1407fe1d5: 66 0f 6e c8                 	movd	%eax, %xmm1
1407fe1d9: 66 0f 70 c9 50              	pshufd	$0x50, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,1]
1407fe1de: 66 0f 72 f1 1f              	pslld	$0x1f, %xmm1
1407fe1e3: 66 0f 72 e1 1f              	psrad	$0x1f, %xmm1
1407fe1e8: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
1407fe1ec: 0f 54 d1                    	andps	%xmm1, %xmm2
1407fe1ef: 0f 55 c8                    	andnps	%xmm0, %xmm1
1407fe1f2: 0f 56 ca                    	orps	%xmm2, %xmm1
1407fe1f5: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
1407fe1f9: 0f 5c c1                    	subps	%xmm1, %xmm0
1407fe1fc: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407fe1ff: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407fe202: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1407fe206: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407fe20a: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407fe20d: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1407fe211: f3 44 0f 10 15 ea 76 12 02  	movss	0x21276ea(%rip), %xmm10 # 0x142925904
1407fe21a: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
1407fe21e: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1407fe222: f3 0f 12 ca                 	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
1407fe226: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407fe229: 0f 13 4c 24 60              	movlps	%xmm1, 0x60(%rsp)
1407fe22e: 48 8d 8c 24 b8 00 00 00     	leaq	0xb8(%rsp), %rcx
1407fe236: 4c 8d 6c 24 60              	leaq	0x60(%rsp), %r13
1407fe23b: 4c 89 ea                    	movq	%r13, %rdx
1407fe23e: e8 7d eb 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fe243: f3 0f 10 b4 24 b8 00 00 00  	movss	0xb8(%rsp), %xmm6
1407fe24c: f3 0f 10 84 24 c0 00 00 00  	movss	0xc0(%rsp), %xmm0
1407fe255: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407fe258: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407fe25c: f3 44 0f 58 35 4b e4 28 02  	addss	0x228e44b(%rip), %xmm14 # 0x142a8c6b0
1407fe265: f3 45 0f 16 c7              	movshdup	%xmm15, %xmm8           # xmm8 = xmm15[1,1,3,3]
1407fe26a: 44 0f 28 e0                 	movaps	%xmm0, %xmm12
1407fe26e: f3 44 0f 10 1d 3d e4 28 02  	movss	0x228e43d(%rip), %xmm11 # 0x142a8c6b4
1407fe277: f3 45 0f 59 e3              	mulss	%xmm11, %xmm12
1407fe27c: 0f 28 d6                    	movaps	%xmm6, %xmm2
1407fe27f: f3 0f 59 d6                 	mulss	%xmm6, %xmm2
1407fe283: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407fe287: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407fe28a: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1407fe28e: f3 44 0f 5e d1              	divss	%xmm1, %xmm10
1407fe293: 41 0f 28 ca                 	movaps	%xmm10, %xmm1
1407fe297: 0f 57 0d 52 76 12 02        	xorps	0x2127652(%rip), %xmm1  # 0x1429258f0
1407fe29e: 0f 14 c6                    	unpcklps	%xmm6, %xmm0            # xmm0 = xmm0[0],xmm6[0],xmm0[1],xmm6[1]
1407fe2a1: 41 0f 14 ca                 	unpcklps	%xmm10, %xmm1           # xmm1 = xmm1[0],xmm10[0],xmm1[1],xmm10[1]
1407fe2a5: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407fe2a8: 0f 13 4c 24 60              	movlps	%xmm1, 0x60(%rsp)
1407fe2ad: 48 8d 8c 24 d0 00 00 00     	leaq	0xd0(%rsp), %rcx
1407fe2b5: 4c 89 ea                    	movq	%r13, %rdx
1407fe2b8: e8 03 eb 49 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1407fe2bd: f3 44 0f 11 bc 24 c4 00 00 00       	movss	%xmm15, 0xc4(%rsp)
1407fe2c7: f3 44 0f 11 b4 24 c8 00 00 00       	movss	%xmm14, 0xc8(%rsp)
1407fe2d1: f3 44 0f 11 84 24 cc 00 00 00       	movss	%xmm8, 0xcc(%rsp)
1407fe2db: f3 0f 10 84 24 bc 00 00 00  	movss	0xbc(%rsp), %xmm0
1407fe2e4: 0f 14 f0                    	unpcklps	%xmm0, %xmm6            # xmm6 = xmm6[0],xmm0[0],xmm6[1],xmm0[1]
1407fe2e7: 0f 28 15 d2 e3 28 02        	movaps	0x228e3d2(%rip), %xmm2  # 0x142a8c6c0
1407fe2ee: 0f 59 f2                    	mulps	%xmm2, %xmm6
1407fe2f1: 0f 13 74 24 38              	movlps	%xmm6, 0x38(%rsp)
1407fe2f6: f3 44 0f 11 64 24 40        	movss	%xmm12, 0x40(%rsp)
1407fe2fd: 44 0f 59 0d cb e3 28 02     	mulps	0x228e3cb(%rip), %xmm9  # 0x142a8c6d0
1407fe305: f3 0f 59 3d d3 e3 28 02     	mulss	0x228e3d3(%rip), %xmm7  # 0x142a8c6e0
1407fe30d: 44 0f 13 4c 24 48           	movlps	%xmm9, 0x48(%rsp)
1407fe313: f3 0f 11 7c 24 50           	movss	%xmm7, 0x50(%rsp)
1407fe319: f3 0f 10 84 24 d8 00 00 00  	movss	0xd8(%rsp), %xmm0
1407fe322: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
1407fe327: f2 0f 10 8c 24 d0 00 00 00  	movsd	0xd0(%rsp), %xmm1
1407fe330: 0f 59 ca                    	mulps	%xmm2, %xmm1
1407fe333: 0f 13 4c 24 60              	movlps	%xmm1, 0x60(%rsp)
1407fe338: f3 0f 11 44 24 68           	movss	%xmm0, 0x68(%rsp)
1407fe33e: 48 8d 84 24 c4 00 00 00     	leaq	0xc4(%rsp), %rax
1407fe346: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407fe34b: 48 8d 8c 24 30 01 00 00     	leaq	0x130(%rsp), %rcx
1407fe353: 48 8d 54 24 38              	leaq	0x38(%rsp), %rdx
1407fe358: 4c 8d 44 24 48              	leaq	0x48(%rsp), %r8
1407fe35d: 4d 89 e9                    	movq	%r13, %r9
1407fe360: e8 9b f1 09 00              	callq	0x14089d500 <.text+0x89c500>
1407fe365: 4d 8b 06                    	movq	(%r14), %r8
1407fe368: 49 8d 0c 30                 	leaq	(%r8,%rsi), %rcx
1407fe36c: 49 89 0e                    	movq	%rcx, (%r14)
1407fe36f: 48 89 c8                    	movq	%rcx, %rax
1407fe372: 4c 31 e0                    	xorq	%r12, %rax
1407fe375: 48 f7 e1                    	mulq	%rcx
1407fe378: 48 31 c2                    	xorq	%rax, %rdx
1407fe37b: 69 c2 e8 03 00 00           	imull	$0x3e8, %edx, %eax      # imm = 0x3E8
1407fe381: 3d 27 01 00 00              	cmpl	$0x127, %eax            # imm = 0x127
1407fe386: 0f 87 d4 fc ff ff           	ja	0x1407fe060 <.text+0x7fd060>
1407fe38c: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407fe396: 49 01 c0                    	addq	%rax, %r8
1407fe399: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407fe3a0: 48 01 f1                    	addq	%rsi, %rcx
1407fe3a3: 4c 89 c0                    	movq	%r8, %rax
1407fe3a6: 4c 31 e0                    	xorq	%r12, %rax
1407fe3a9: 49 f7 e0                    	mulq	%r8
1407fe3ac: 48 31 c2                    	xorq	%rax, %rdx
1407fe3af: 69 c2 e8 03 00 00           	imull	$0x3e8, %edx, %eax      # imm = 0x3E8
1407fe3b5: 49 01 f0                    	addq	%rsi, %r8
1407fe3b8: 3d 28 01 00 00              	cmpl	$0x128, %eax            # imm = 0x128
1407fe3bd: 72 e1                       	jb	0x1407fe3a0 <.text+0x7fd3a0>
1407fe3bf: 49 89 0e                    	movq	%rcx, (%r14)
1407fe3c2: e9 99 fc ff ff              	jmp	0x1407fe060 <.text+0x7fd060>
1407fe3c7: 48 85 db                    	testq	%rbx, %rbx
1407fe3ca: 4c 8d 7c 24 60              	leaq	0x60(%rsp), %r15
1407fe3cf: 0f 84 32 07 00 00           	je	0x1407feb07 <.text+0x7fdb07>
1407fe3d5: b0 01                       	movb	$0x1, %al
1407fe3d7: 45 31 ed                    	xorl	%r13d, %r13d
1407fe3da: f3 44 0f 10 25 1d f8 1e 02  	movss	0x21ef81d(%rip), %xmm12 # 0x1429edc00
1407fe3e3: 44 0f 28 2d 05 e3 28 02     	movaps	0x228e305(%rip), %xmm13 # 0x142a8c6f0
1407fe3eb: 48 8d ac 24 60 01 00 00     	leaq	0x160(%rsp), %rbp
1407fe3f3: e9 93 00 00 00              	jmp	0x1407fe48b <.text+0x7fd48b>
1407fe3f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407fe400: 49 ff c5                    	incq	%r13
1407fe403: 89 d0                       	movl	%edx, %eax
1407fe405: 48 69 c0 e8 03 00 00        	imulq	$0x3e8, %rax, %rax      # imm = 0x3E8
1407fe40c: 48 c1 e8 20                 	shrq	$0x20, %rax
1407fe410: 0f 10 84 24 90 01 00 00     	movups	0x190(%rsp), %xmm0
1407fe418: 0f 10 8c 24 a0 01 00 00     	movups	0x1a0(%rsp), %xmm1
1407fe420: 0f 10 94 24 b0 01 00 00     	movups	0x1b0(%rsp), %xmm2
1407fe428: 0f 29 94 24 80 00 00 00     	movaps	%xmm2, 0x80(%rsp)
1407fe430: 0f 29 4c 24 70              	movaps	%xmm1, 0x70(%rsp)
1407fe435: 0f 29 44 24 60              	movaps	%xmm0, 0x60(%rsp)
1407fe43a: c7 84 24 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rsp)
1407fe445: 89 84 24 94 00 00 00        	movl	%eax, 0x94(%rsp)
1407fe44c: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1407fe450: 89 84 24 98 00 00 00        	movl	%eax, 0x98(%rsp)
1407fe457: c7 84 24 9c 00 00 00 18 00 00 00    	movl	$0x18, 0x9c(%rsp)
1407fe462: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1407fe467: 4c 89 fa                    	movq	%r15, %rdx
1407fe46a: e8 71 ec a8 00              	callq	0x14128d0e0 <.text+0x128c0e0>
1407fe46f: 31 c0                       	xorl	%eax, %eax
1407fe471: 48 89 d9                    	movq	%rbx, %rcx
1407fe474: 48 3b bc 24 a0 00 00 00     	cmpq	0xa0(%rsp), %rdi
1407fe47c: f3 44 0f 10 25 7b f7 1e 02  	movss	0x21ef77b(%rip), %xmm12 # 0x1429edc00
1407fe485: 0f 84 7c 06 00 00           	je	0x1407feb07 <.text+0x7fdb07>
1407fe48b: 48 8d 57 0c                 	leaq	0xc(%rdi), %rdx
1407fe48f: a8 01                       	testb	$0x1, %al
1407fe491: 74 1d                       	je	0x1407fe4b0 <.text+0x7fd4b0>
1407fe493: 48 3b 94 24 a0 00 00 00     	cmpq	0xa0(%rsp), %rdx
1407fe49b: 0f 84 66 06 00 00           	je	0x1407feb07 <.text+0x7fdb07>
1407fe4a1: 48 89 f9                    	movq	%rdi, %rcx
1407fe4a4: 48 83 c7 18                 	addq	$0x18, %rdi
1407fe4a8: 48 89 d3                    	movq	%rdx, %rbx
1407fe4ab: eb 09                       	jmp	0x1407fe4b6 <.text+0x7fd4b6>
1407fe4ad: 0f 1f 00                    	nopl	(%rax)
1407fe4b0: 48 89 fb                    	movq	%rdi, %rbx
1407fe4b3: 48 89 d7                    	movq	%rdx, %rdi
1407fe4b6: f3 44 0f 10 39              	movss	(%rcx), %xmm15
1407fe4bb: f3 44 0f 10 71 08           	movss	0x8(%rcx), %xmm14
1407fe4c1: f3 0f 10 71 04              	movss	0x4(%rcx), %xmm6
1407fe4c6: f3 41 0f 58 f4              	addss	%xmm12, %xmm6
1407fe4cb: f3 44 0f 10 1b              	movss	(%rbx), %xmm11
1407fe4d0: f3 44 0f 10 4b 04           	movss	0x4(%rbx), %xmm9
1407fe4d6: f3 44 0f 10 53 08           	movss	0x8(%rbx), %xmm10
1407fe4dc: 4c 89 f1                    	movq	%r14, %rcx
1407fe4df: e8 9c 0e 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe4e4: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1407fe4e8: 4c 89 f1                    	movq	%r14, %rcx
1407fe4eb: e8 90 0e 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe4f0: 0f 28 f8                    	movaps	%xmm0, %xmm7
1407fe4f3: 4c 89 f1                    	movq	%r14, %rcx
1407fe4f6: e8 85 0e 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe4fb: 4c 3b ac 24 e8 02 00 00     	cmpq	0x2e8(%rsp), %r13
1407fe503: 0f 84 81 06 00 00           	je	0x1407feb8a <.text+0x7fdb8a>
1407fe509: 0f 14 f8                    	unpcklps	%xmm0, %xmm7            # xmm7 = xmm7[0],xmm0[0],xmm7[1],xmm0[1]
1407fe50c: 0f 58 3d dd 8c 21 02        	addps	0x2218cdd(%rip), %xmm7  # 0x142a171f0
1407fe513: f3 44 0f 58 05 cc bb 15 02  	addss	0x215bbcc(%rip), %xmm8  # 0x14295a0e8
1407fe51c: f3 45 0f 58 cc              	addss	%xmm12, %xmm9
1407fe521: 45 0f 14 ca                 	unpcklps	%xmm10, %xmm9           # xmm9 = xmm9[0],xmm10[0],xmm9[1],xmm10[1]
1407fe525: 48 8b 84 24 e0 02 00 00     	movq	0x2e0(%rsp), %rax
1407fe52d: f3 46 0f 10 24 a8           	movss	(%rax,%r13,4), %xmm12
1407fe533: f3 45 0f 59 c4              	mulss	%xmm12, %xmm8
1407fe538: f3 44 0f 59 05 27 eb 1c 02  	mulss	0x21ceb27(%rip), %xmm8  # 0x1429cd068
1407fe541: f3 45 0f 58 d8              	addss	%xmm8, %xmm11
1407fe546: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1407fe54a: f3 41 0f 5c c7              	subss	%xmm15, %xmm0
1407fe54f: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407fe552: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1407fe556: 41 0f 28 df                 	movaps	%xmm15, %xmm3
1407fe55a: f3 41 0f 5c db              	subss	%xmm11, %xmm3
1407fe55f: f3 0f 59 db                 	mulss	%xmm3, %xmm3
1407fe563: f3 41 0f 12 d4              	movsldup	%xmm12, %xmm2           # xmm2 = xmm12[0,0,2,2]
1407fe568: 0f 29 94 24 f0 00 00 00     	movaps	%xmm2, 0xf0(%rsp)
1407fe570: 0f 59 fa                    	mulps	%xmm2, %xmm7
1407fe573: 41 0f 59 fd                 	mulps	%xmm13, %xmm7
1407fe577: 41 0f 58 f9                 	addps	%xmm9, %xmm7
1407fe57b: f3 0f 16 e7                 	movshdup	%xmm7, %xmm4            # xmm4 = xmm7[1,1,3,3]
1407fe57f: 0f 28 d6                    	movaps	%xmm6, %xmm2
1407fe582: 41 0f 14 d6                 	unpcklps	%xmm14, %xmm2           # xmm2 = xmm2[0],xmm14[0],xmm2[1],xmm14[1]
1407fe586: 0f 28 ef                    	movaps	%xmm7, %xmm5
1407fe589: 0f 5c ea                    	subps	%xmm2, %xmm5
1407fe58c: 0f 28 d5                    	movaps	%xmm5, %xmm2
1407fe58f: 0f 59 d5                    	mulps	%xmm5, %xmm2
1407fe592: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1407fe596: f3 0f 16 d2                 	movshdup	%xmm2, %xmm2            # xmm2 = xmm2[1,1,3,3]
1407fe59a: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1407fe59e: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407fe5a1: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1407fe5a5: f3 44 0f 10 2d 56 73 12 02  	movss	0x2127356(%rip), %xmm13 # 0x142925904
1407fe5ae: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
1407fe5b2: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1407fe5b6: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
1407fe5ba: f3 0f 12 d2                 	movsldup	%xmm2, %xmm2            # xmm2 = xmm2[0,0,2,2]
1407fe5be: 0f 59 d5                    	mulps	%xmm5, %xmm2
1407fe5c1: 44 0f 28 c2                 	movaps	%xmm2, %xmm8
1407fe5c5: 44 0f 59 05 d3 73 12 02     	mulps	0x21273d3(%rip), %xmm8  # 0x1429259a0
1407fe5cd: 0f 28 c8                    	movaps	%xmm0, %xmm1
1407fe5d0: 0f 57 ed                    	xorps	%xmm5, %xmm5
1407fe5d3: f3 0f 59 cd                 	mulss	%xmm5, %xmm1
1407fe5d7: 0f c6 ca d4                 	shufps	$0xd4, %xmm2, %xmm1     # xmm1 = xmm1[0,1],xmm2[1,3]
1407fe5db: 0f c6 ca f2                 	shufps	$0xf2, %xmm2, %xmm1     # xmm1 = xmm1[2,0],xmm2[3,3]
1407fe5df: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1407fe5e3: f3 45 0f 5c c8              	subss	%xmm8, %xmm9
1407fe5e8: 44 0f 5c c1                 	subps	%xmm1, %xmm8
1407fe5ec: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1407fe5f0: 41 0f 59 c8                 	mulps	%xmm8, %xmm1
1407fe5f4: f3 0f 16 e9                 	movshdup	%xmm1, %xmm5            # xmm5 = xmm1[1,1,3,3]
1407fe5f8: f3 0f 58 e9                 	addss	%xmm1, %xmm5
1407fe5fc: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1407fe600: f3 41 0f 59 c9              	mulss	%xmm9, %xmm1
1407fe605: f3 0f 58 cd                 	addss	%xmm5, %xmm1
1407fe609: 0f 57 ed                    	xorps	%xmm5, %xmm5
1407fe60c: f3 0f 51 e9                 	sqrtss	%xmm1, %xmm5
1407fe610: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1407fe614: f3 0f 5e cd                 	divss	%xmm5, %xmm1
1407fe618: f3 0f 12 e9                 	movsldup	%xmm1, %xmm5            # xmm5 = xmm1[0,0,2,2]
1407fe61c: 41 0f 59 e8                 	mulps	%xmm8, %xmm5
1407fe620: f3 41 0f 59 c9              	mulss	%xmm9, %xmm1
1407fe625: f3 44 0f 16 c5              	movshdup	%xmm5, %xmm8            # xmm8 = xmm5[1,1,3,3]
1407fe62a: f3 44 0f 59 c0              	mulss	%xmm0, %xmm8
1407fe62f: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
1407fe633: f3 44 0f 5c cf              	subss	%xmm7, %xmm9
1407fe638: 45 0f 28 d6                 	movaps	%xmm14, %xmm10
1407fe63c: f3 44 0f 5c d4              	subss	%xmm4, %xmm10
1407fe641: f3 45 0f 59 c9              	mulss	%xmm9, %xmm9
1407fe646: f3 44 0f 58 cb              	addss	%xmm3, %xmm9
1407fe64b: f3 45 0f 59 d2              	mulss	%xmm10, %xmm10
1407fe650: f3 45 0f 58 d1              	addss	%xmm9, %xmm10
1407fe655: 0f 57 e4                    	xorps	%xmm4, %xmm4
1407fe658: f3 41 0f 51 e2              	sqrtss	%xmm10, %xmm4
1407fe65d: 0f 28 dc                    	movaps	%xmm4, %xmm3
1407fe660: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
1407fe664: 0f 29 a4 24 10 01 00 00     	movaps	%xmm4, 0x110(%rsp)
1407fe66c: f3 0f 12 e4                 	movsldup	%xmm4, %xmm4            # xmm4 = xmm4[0,0,2,2]
1407fe670: 0f 29 a4 24 00 01 00 00     	movaps	%xmm4, 0x100(%rsp)
1407fe678: 0f 59 e2                    	mulps	%xmm2, %xmm4
1407fe67b: 0f c6 c2 d4                 	shufps	$0xd4, %xmm2, %xmm0     # xmm0 = xmm0[0,1],xmm2[1,3]
1407fe67f: 0f c6 c2 f2                 	shufps	$0xf2, %xmm2, %xmm0     # xmm0 = xmm0[2,0],xmm2[3,3]
1407fe683: 44 0f 28 cd                 	movaps	%xmm5, %xmm9
1407fe687: 44 0f c6 cd e1              	shufps	$0xe1, %xmm5, %xmm9     # xmm9 = xmm9[1,0],xmm5[2,3]
1407fe68c: f3 44 0f 10 c9              	movss	%xmm1, %xmm9            # xmm9 = xmm1[0],xmm9[1,2,3]
1407fe691: 44 0f 59 ca                 	mulps	%xmm2, %xmm9
1407fe695: f3 0f 59 d5                 	mulss	%xmm5, %xmm2
1407fe699: f3 41 0f 5c d0              	subss	%xmm8, %xmm2
1407fe69e: 44 0f 28 c1                 	movaps	%xmm1, %xmm8
1407fe6a2: 0f c6 cd d4                 	shufps	$0xd4, %xmm5, %xmm1     # xmm1 = xmm1[0,1],xmm5[1,3]
1407fe6a6: 0f c6 cd e2                 	shufps	$0xe2, %xmm5, %xmm1     # xmm1 = xmm1[2,0],xmm5[2,3]
1407fe6aa: 44 0f 28 15 0e e0 28 02     	movaps	0x228e00e(%rip), %xmm10 # 0x142a8c6c0
1407fe6b2: 41 0f 59 ea                 	mulps	%xmm10, %xmm5
1407fe6b6: 0f 13 ac 24 a8 00 00 00     	movlps	%xmm5, 0xa8(%rsp)
1407fe6be: f3 0f 10 2d ee df 28 02     	movss	0x228dfee(%rip), %xmm5  # 0x142a8c6b4
1407fe6c6: f3 44 0f 59 c5              	mulss	%xmm5, %xmm8
1407fe6cb: f3 44 0f 11 84 24 b0 00 00 00       	movss	%xmm8, 0xb0(%rsp)
1407fe6d5: f3 0f 11 5c 24 38           	movss	%xmm3, 0x38(%rsp)
1407fe6db: 0f 13 64 24 3c              	movlps	%xmm4, 0x3c(%rsp)
1407fe6e0: 0f 59 c8                    	mulps	%xmm0, %xmm1
1407fe6e3: 41 0f 5c c9                 	subps	%xmm9, %xmm1
1407fe6e7: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407fe6ea: 0f 59 c1                    	mulps	%xmm1, %xmm0
1407fe6ed: f3 0f 16 d8                 	movshdup	%xmm0, %xmm3            # xmm3 = xmm0[1,1,3,3]
1407fe6f1: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1407fe6f5: 0f 28 c2                    	movaps	%xmm2, %xmm0
1407fe6f8: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
1407fe6fc: f3 0f 58 d8                 	addss	%xmm0, %xmm3
1407fe700: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407fe703: f3 0f 51 c3                 	sqrtss	%xmm3, %xmm0
1407fe707: 41 0f 28 dd                 	movaps	%xmm13, %xmm3
1407fe70b: f3 0f 5e d8                 	divss	%xmm0, %xmm3
1407fe70f: f3 0f 12 c3                 	movsldup	%xmm3, %xmm0            # xmm0 = xmm3[0,0,2,2]
1407fe713: 0f 59 c1                    	mulps	%xmm1, %xmm0
1407fe716: f3 0f 59 da                 	mulss	%xmm2, %xmm3
1407fe71a: 41 0f 59 c2                 	mulps	%xmm10, %xmm0
1407fe71e: f3 0f 59 dd                 	mulss	%xmm5, %xmm3
1407fe722: 0f 13 44 24 48              	movlps	%xmm0, 0x48(%rsp)
1407fe727: f3 0f 11 5c 24 50           	movss	%xmm3, 0x50(%rsp)
1407fe72d: 41 0f 28 c6                 	movaps	%xmm14, %xmm0
1407fe731: 45 0f 28 f7                 	movaps	%xmm15, %xmm14
1407fe735: 44 0f 14 fe                 	unpcklps	%xmm6, %xmm15           # xmm15 = xmm15[0],xmm6[0],xmm15[1],xmm6[1]
1407fe739: 44 0f 58 3d bf df 28 02     	addps	0x228dfbf(%rip), %xmm15 # 0x142a8c700
1407fe741: 0f 29 84 24 20 01 00 00     	movaps	%xmm0, 0x120(%rsp)
1407fe749: f3 0f 58 05 3f 1f 16 02     	addss	0x2161f3f(%rip), %xmm0  # 0x142960690
1407fe751: 44 0f 13 7c 24 60           	movlps	%xmm15, 0x60(%rsp)
1407fe757: 0f 29 84 24 e0 00 00 00     	movaps	%xmm0, 0xe0(%rsp)
1407fe75f: f3 0f 11 44 24 68           	movss	%xmm0, 0x68(%rsp)
1407fe765: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1407fe76a: 48 89 e9                    	movq	%rbp, %rcx
1407fe76d: 48 8d 94 24 a8 00 00 00     	leaq	0xa8(%rsp), %rdx
1407fe775: 4c 8d 44 24 38              	leaq	0x38(%rsp), %r8
1407fe77a: 4c 8d 4c 24 48              	leaq	0x48(%rsp), %r9
1407fe77f: e8 7c ed 09 00              	callq	0x14089d500 <.text+0x89c500>
1407fe784: f3 0f 10 05 20 15 28 02     	movss	0x2281520(%rip), %xmm0  # 0x142a7fcac
1407fe78c: 0f 2e 84 24 b4 00 00 00     	ucomiss	0xb4(%rsp), %xmm0
1407fe794: 0f 86 c4 00 00 00           	jbe	0x1407fe85e <.text+0x7fd85e>
1407fe79a: 4d 8b 06                    	movq	(%r14), %r8
1407fe79d: 49 8d 0c 30                 	leaq	(%r8,%rsi), %rcx
1407fe7a1: 49 89 0e                    	movq	%rcx, (%r14)
1407fe7a4: 48 89 c8                    	movq	%rcx, %rax
1407fe7a7: 4c 31 e0                    	xorq	%r12, %rax
1407fe7aa: 48 f7 e1                    	mulq	%rcx
1407fe7ad: 48 31 c2                    	xorq	%rax, %rdx
1407fe7b0: 69 c2 e8 03 00 00           	imull	$0x3e8, %edx, %eax      # imm = 0x3E8
1407fe7b6: 3d 27 01 00 00              	cmpl	$0x127, %eax            # imm = 0x127
1407fe7bb: 77 35                       	ja	0x1407fe7f2 <.text+0x7fd7f2>
1407fe7bd: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407fe7c7: 49 01 c0                    	addq	%rax, %r8
1407fe7ca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407fe7d0: 48 01 f1                    	addq	%rsi, %rcx
1407fe7d3: 4c 89 c0                    	movq	%r8, %rax
1407fe7d6: 4c 31 e0                    	xorq	%r12, %rax
1407fe7d9: 49 f7 e0                    	mulq	%r8
1407fe7dc: 48 31 c2                    	xorq	%rax, %rdx
1407fe7df: 69 c2 e8 03 00 00           	imull	$0x3e8, %edx, %eax      # imm = 0x3E8
1407fe7e5: 49 01 f0                    	addq	%rsi, %r8
1407fe7e8: 3d 28 01 00 00              	cmpl	$0x128, %eax            # imm = 0x128
1407fe7ed: 72 e1                       	jb	0x1407fe7d0 <.text+0x7fd7d0>
1407fe7ef: 49 89 0e                    	movq	%rcx, (%r14)
1407fe7f2: 89 d0                       	movl	%edx, %eax
1407fe7f4: 48 69 c0 e8 03 00 00        	imulq	$0x3e8, %rax, %rax      # imm = 0x3E8
1407fe7fb: 48 c1 e8 20                 	shrq	$0x20, %rax
1407fe7ff: 0f 10 84 24 60 01 00 00     	movups	0x160(%rsp), %xmm0
1407fe807: 0f 10 8c 24 70 01 00 00     	movups	0x170(%rsp), %xmm1
1407fe80f: 0f 10 94 24 80 01 00 00     	movups	0x180(%rsp), %xmm2
1407fe817: 0f 29 94 24 80 00 00 00     	movaps	%xmm2, 0x80(%rsp)
1407fe81f: 0f 29 4c 24 70              	movaps	%xmm1, 0x70(%rsp)
1407fe824: 0f 29 44 24 60              	movaps	%xmm0, 0x60(%rsp)
1407fe829: c7 84 24 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rsp)
1407fe834: 89 84 24 94 00 00 00        	movl	%eax, 0x94(%rsp)
1407fe83b: 8b 44 24 34                 	movl	0x34(%rsp), %eax
1407fe83f: 89 84 24 98 00 00 00        	movl	%eax, 0x98(%rsp)
1407fe846: c7 84 24 9c 00 00 00 18 00 00 00    	movl	$0x18, 0x9c(%rsp)
1407fe851: 48 8b 4c 24 58              	movq	0x58(%rsp), %rcx
1407fe856: 4c 89 fa                    	movq	%r15, %rdx
1407fe859: e8 82 e8 a8 00              	callq	0x14128d0e0 <.text+0x128c0e0>
1407fe85e: 44 0f 28 d6                 	movaps	%xmm6, %xmm10
1407fe862: f3 0f 10 05 96 f3 1e 02     	movss	0x21ef396(%rip), %xmm0  # 0x1429edc00
1407fe86a: f3 44 0f 58 d0              	addss	%xmm0, %xmm10
1407fe86f: 44 0f 28 ef                 	movaps	%xmm7, %xmm13
1407fe873: f3 44 0f 58 e8              	addss	%xmm0, %xmm13
1407fe878: 4c 89 f1                    	movq	%r14, %rcx
1407fe87b: e8 00 0b 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe880: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1407fe884: 4c 89 f1                    	movq	%r14, %rcx
1407fe887: e8 f4 0a 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe88c: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1407fe890: 4c 89 f1                    	movq	%r14, %rcx
1407fe893: e8 e8 0a 4b 00              	callq	0x140caf380 <.text+0xcae380>
1407fe898: f3 44 0f 58 05 47 b8 15 02  	addss	0x215b847(%rip), %xmm8  # 0x14295a0e8
1407fe8a1: f3 45 0f 59 e0              	mulss	%xmm8, %xmm12
1407fe8a6: f3 44 0f 59 25 b9 e7 1c 02  	mulss	0x21ce7b9(%rip), %xmm12 # 0x1429cd068
1407fe8af: f3 45 0f 58 dc              	addss	%xmm12, %xmm11
1407fe8b4: f3 45 0f 5c de              	subss	%xmm14, %xmm11
1407fe8b9: 44 0f 14 c8                 	unpcklps	%xmm0, %xmm9            # xmm9 = xmm9[0],xmm0[0],xmm9[1],xmm0[1]
1407fe8bd: 44 0f 58 0d 2b 89 21 02     	addps	0x221892b(%rip), %xmm9  # 0x142a171f0
1407fe8c5: 0f 28 8c 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm1
1407fe8cd: 41 0f 59 c9                 	mulps	%xmm9, %xmm1
1407fe8d1: f3 41 0f 10 fd              	movss	%xmm13, %xmm7           # xmm7 = xmm13[0],xmm7[1,2,3]
1407fe8d6: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1407fe8da: f3 41 0f 59 c3              	mulss	%xmm11, %xmm0
1407fe8df: 44 0f 28 2d 09 de 28 02     	movaps	0x228de09(%rip), %xmm13 # 0x142a8c6f0
1407fe8e7: 41 0f 59 cd                 	mulps	%xmm13, %xmm1
1407fe8eb: 0f 58 f9                    	addps	%xmm1, %xmm7
1407fe8ee: 44 0f 14 94 24 20 01 00 00  	unpcklps	0x120(%rsp), %xmm10     # xmm10 = xmm10[0],mem[0],xmm10[1],mem[1]
1407fe8f7: 41 0f 5c fa                 	subps	%xmm10, %xmm7
1407fe8fb: 0f 28 cf                    	movaps	%xmm7, %xmm1
1407fe8fe: 0f 59 cf                    	mulps	%xmm7, %xmm1
1407fe901: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1407fe905: f3 0f 16 c9                 	movshdup	%xmm1, %xmm1            # xmm1 = xmm1[1,1,3,3]
1407fe909: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407fe90d: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407fe910: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1407fe914: f3 0f 10 2d e8 6f 12 02     	movss	0x2126fe8(%rip), %xmm5  # 0x142925904
1407fe91c: 0f 28 cd                    	movaps	%xmm5, %xmm1
1407fe91f: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1407fe923: f3 44 0f 59 d9              	mulss	%xmm1, %xmm11
1407fe928: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
1407fe92c: 0f 59 cf                    	mulps	%xmm7, %xmm1
1407fe92f: 0f 28 d9                    	movaps	%xmm1, %xmm3
1407fe932: 0f 59 1d 67 70 12 02        	mulps	0x2127067(%rip), %xmm3  # 0x1429259a0
1407fe939: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1407fe93d: f3 0f 59 05 4b 1d 16 02     	mulss	0x2161d4b(%rip), %xmm0  # 0x142960690
1407fe945: 0f c6 c1 d4                 	shufps	$0xd4, %xmm1, %xmm0     # xmm0 = xmm0[0,1],xmm1[1,3]
1407fe949: 0f c6 c1 f2                 	shufps	$0xf2, %xmm1, %xmm0     # xmm0 = xmm0[2,0],xmm1[3,3]
1407fe94d: 41 0f 28 e3                 	movaps	%xmm11, %xmm4
1407fe951: f3 0f 5c e3                 	subss	%xmm3, %xmm4
1407fe955: 0f 5c d8                    	subps	%xmm0, %xmm3
1407fe958: 0f 28 c3                    	movaps	%xmm3, %xmm0
1407fe95b: 0f 59 c3                    	mulps	%xmm3, %xmm0
1407fe95e: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407fe962: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407fe966: 0f 28 c4                    	movaps	%xmm4, %xmm0
1407fe969: f3 0f 59 c4                 	mulss	%xmm4, %xmm0
1407fe96d: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407fe971: 0f 57 d2                    	xorps	%xmm2, %xmm2
1407fe974: f3 0f 51 d0                 	sqrtss	%xmm0, %xmm2
1407fe978: 0f 28 c5                    	movaps	%xmm5, %xmm0
1407fe97b: f3 0f 5e c2                 	divss	%xmm2, %xmm0
1407fe97f: f3 0f 12 d0                 	movsldup	%xmm0, %xmm2            # xmm2 = xmm0[0,0,2,2]
1407fe983: 0f 59 d3                    	mulps	%xmm3, %xmm2
1407fe986: f3 0f 59 c4                 	mulss	%xmm4, %xmm0
1407fe98a: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
1407fe98e: f3 41 0f 59 db              	mulss	%xmm11, %xmm3
1407fe993: 44 0f 28 8c 24 10 01 00 00  	movaps	0x110(%rsp), %xmm9
1407fe99c: f3 45 0f 59 cb              	mulss	%xmm11, %xmm9
1407fe9a1: 44 0f 28 94 24 00 01 00 00  	movaps	0x100(%rsp), %xmm10
1407fe9aa: 44 0f 59 d1                 	mulps	%xmm1, %xmm10
1407fe9ae: 44 0f c6 d9 d4              	shufps	$0xd4, %xmm1, %xmm11    # xmm11 = xmm11[0,1],xmm1[1,3]
1407fe9b3: 44 0f c6 d9 f2              	shufps	$0xf2, %xmm1, %xmm11    # xmm11 = xmm11[2,0],xmm1[3,3]
1407fe9b8: 0f 28 e2                    	movaps	%xmm2, %xmm4
1407fe9bb: 0f c6 e2 e1                 	shufps	$0xe1, %xmm2, %xmm4     # xmm4 = xmm4[1,0],xmm2[2,3]
1407fe9bf: f3 0f 10 e0                 	movss	%xmm0, %xmm4            # xmm4 = xmm0[0],xmm4[1,2,3]
1407fe9c3: 0f 59 e1                    	mulps	%xmm1, %xmm4
1407fe9c6: f3 0f 59 ca                 	mulss	%xmm2, %xmm1
1407fe9ca: f3 0f 5c cb                 	subss	%xmm3, %xmm1
1407fe9ce: 0f 28 d8                    	movaps	%xmm0, %xmm3
1407fe9d1: 0f c6 c2 d4                 	shufps	$0xd4, %xmm2, %xmm0     # xmm0 = xmm0[0,1],xmm2[1,3]
1407fe9d5: 0f c6 c2 e2                 	shufps	$0xe2, %xmm2, %xmm0     # xmm0 = xmm0[2,0],xmm2[2,3]
1407fe9d9: 44 0f 28 05 df dc 28 02     	movaps	0x228dcdf(%rip), %xmm8  # 0x142a8c6c0
1407fe9e1: 41 0f 59 d0                 	mulps	%xmm8, %xmm2
1407fe9e5: 0f 13 94 24 a8 00 00 00     	movlps	%xmm2, 0xa8(%rsp)
1407fe9ed: f3 0f 10 3d bf dc 28 02     	movss	0x228dcbf(%rip), %xmm7  # 0x142a8c6b4
1407fe9f5: f3 0f 59 df                 	mulss	%xmm7, %xmm3
1407fe9f9: f3 0f 11 9c 24 b0 00 00 00  	movss	%xmm3, 0xb0(%rsp)
1407fea02: f3 44 0f 11 4c 24 38        	movss	%xmm9, 0x38(%rsp)
1407fea09: 44 0f 13 54 24 3c           	movlps	%xmm10, 0x3c(%rsp)
1407fea0f: 41 0f 59 c3                 	mulps	%xmm11, %xmm0
1407fea13: 0f 5c c4                    	subps	%xmm4, %xmm0
1407fea16: 0f 28 d0                    	movaps	%xmm0, %xmm2
1407fea19: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407fea1c: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
1407fea20: f3 0f 58 da                 	addss	%xmm2, %xmm3
1407fea24: 0f 28 d1                    	movaps	%xmm1, %xmm2
1407fea27: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
1407fea2b: f3 0f 58 da                 	addss	%xmm2, %xmm3
1407fea2f: 0f 57 d2                    	xorps	%xmm2, %xmm2
1407fea32: f3 0f 51 d3                 	sqrtss	%xmm3, %xmm2
1407fea36: 0f 28 dd                    	movaps	%xmm5, %xmm3
1407fea39: f3 0f 5e da                 	divss	%xmm2, %xmm3
1407fea3d: f3 0f 12 d3                 	movsldup	%xmm3, %xmm2            # xmm2 = xmm3[0,0,2,2]
1407fea41: 0f 59 d0                    	mulps	%xmm0, %xmm2
1407fea44: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1407fea48: 41 0f 59 d0                 	mulps	%xmm8, %xmm2
1407fea4c: 0f 13 54 24 48              	movlps	%xmm2, 0x48(%rsp)
1407fea51: f3 0f 59 df                 	mulss	%xmm7, %xmm3
1407fea55: f3 0f 11 5c 24 50           	movss	%xmm3, 0x50(%rsp)
1407fea5b: f3 44 0f 11 7c 24 60        	movss	%xmm15, 0x60(%rsp)
1407fea62: f3 0f 58 35 a6 dc 28 02     	addss	0x228dca6(%rip), %xmm6  # 0x142a8c710
1407fea6a: f3 0f 11 74 24 64           	movss	%xmm6, 0x64(%rsp)
1407fea70: 0f 28 84 24 e0 00 00 00     	movaps	0xe0(%rsp), %xmm0
1407fea78: f3 0f 11 44 24 68           	movss	%xmm0, 0x68(%rsp)
1407fea7e: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1407fea83: 48 8d 8c 24 90 01 00 00     	leaq	0x190(%rsp), %rcx
1407fea8b: 48 8d 94 24 a8 00 00 00     	leaq	0xa8(%rsp), %rdx
1407fea93: 4c 8d 44 24 38              	leaq	0x38(%rsp), %r8
1407fea98: 4c 8d 4c 24 48              	leaq	0x48(%rsp), %r9
1407fea9d: e8 5e ea 09 00              	callq	0x14089d500 <.text+0x89c500>
1407feaa2: 4d 8b 06                    	movq	(%r14), %r8
1407feaa5: 49 8d 0c 30                 	leaq	(%r8,%rsi), %rcx
1407feaa9: 49 89 0e                    	movq	%rcx, (%r14)
1407feaac: 48 89 c8                    	movq	%rcx, %rax
1407feaaf: 4c 31 e0                    	xorq	%r12, %rax
1407feab2: 48 f7 e1                    	mulq	%rcx
1407feab5: 48 31 c2                    	xorq	%rax, %rdx
1407feab8: 69 c2 e8 03 00 00           	imull	$0x3e8, %edx, %eax      # imm = 0x3E8
1407feabe: 3d 27 01 00 00              	cmpl	$0x127, %eax            # imm = 0x127
1407feac3: 0f 87 37 f9 ff ff           	ja	0x1407fe400 <.text+0x7fd400>
1407feac9: 48 b8 5e c8 7a f1 c8 3a ec 40       	movabsq	$0x40ec3ac8f17ac85e, %rax # imm = 0x40EC3AC8F17AC85E
1407fead3: 49 01 c0                    	addq	%rax, %r8
1407fead6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407feae0: 48 01 f1                    	addq	%rsi, %rcx
1407feae3: 4c 89 c0                    	movq	%r8, %rax
1407feae6: 4c 31 e0                    	xorq	%r12, %rax
1407feae9: 49 f7 e0                    	mulq	%r8
1407feaec: 48 31 c2                    	xorq	%rax, %rdx
1407feaef: 69 c2 e8 03 00 00           	imull	$0x3e8, %edx, %eax      # imm = 0x3E8
1407feaf5: 49 01 f0                    	addq	%rsi, %r8
1407feaf8: 3d 28 01 00 00              	cmpl	$0x128, %eax            # imm = 0x128
1407feafd: 72 e1                       	jb	0x1407feae0 <.text+0x7fdae0>
1407feaff: 49 89 0e                    	movq	%rcx, (%r14)
1407feb02: e9 f9 f8 ff ff              	jmp	0x1407fe400 <.text+0x7fd400>
1407feb07: 0f 28 b4 24 c0 01 00 00     	movaps	0x1c0(%rsp), %xmm6
1407feb0f: 0f 28 bc 24 d0 01 00 00     	movaps	0x1d0(%rsp), %xmm7
1407feb17: 44 0f 28 84 24 e0 01 00 00  	movaps	0x1e0(%rsp), %xmm8
1407feb20: 44 0f 28 8c 24 f0 01 00 00  	movaps	0x1f0(%rsp), %xmm9
1407feb29: 44 0f 28 94 24 00 02 00 00  	movaps	0x200(%rsp), %xmm10
1407feb32: 44 0f 28 9c 24 10 02 00 00  	movaps	0x210(%rsp), %xmm11
1407feb3b: 44 0f 28 a4 24 20 02 00 00  	movaps	0x220(%rsp), %xmm12
1407feb44: 44 0f 28 ac 24 30 02 00 00  	movaps	0x230(%rsp), %xmm13
1407feb4d: 44 0f 28 b4 24 40 02 00 00  	movaps	0x240(%rsp), %xmm14
1407feb56: 44 0f 28 bc 24 50 02 00 00  	movaps	0x250(%rsp), %xmm15
1407feb5f: 48 81 c4 68 02 00 00        	addq	$0x268, %rsp            # imm = 0x268
1407feb66: 5b                          	popq	%rbx
1407feb67: 5d                          	popq	%rbp
1407feb68: 5f                          	popq	%rdi
1407feb69: 5e                          	popq	%rsi
1407feb6a: 41 5c                       	popq	%r12
1407feb6c: 41 5d                       	popq	%r13
1407feb6e: 41 5e                       	popq	%r14
1407feb70: 41 5f                       	popq	%r15
1407feb72: c3                          	retq
1407feb73: 4c 8d 05 3e e0 28 02        	leaq	0x228e03e(%rip), %r8    # 0x142a8cbb8
1407feb7a: 48 8b 8c 24 e8 02 00 00     	movq	0x2e8(%rsp), %rcx
1407feb82: 48 89 ca                    	movq	%rcx, %rdx
1407feb85: e8 8e a9 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407feb8a: 4c 8d 05 0f e0 28 02        	leaq	0x228e00f(%rip), %r8    # 0x142a8cba0
1407feb91: 48 8b 8c 24 e8 02 00 00     	movq	0x2e8(%rsp), %rcx
1407feb99: 48 89 ca                    	movq	%rcx, %rdx
1407feb9c: e8 77 a9 0d 02              	callq	0x1428d9518 <.text+0x28d8518>
1407feba1: cc                          	int3
1407feba2: cc                          	int3
1407feba3: cc                          	int3
1407feba4: cc                          	int3
1407feba5: cc                          	int3
1407feba6: cc                          	int3
1407feba7: cc                          	int3
1407feba8: cc                          	int3
1407feba9: cc                          	int3
1407febaa: cc                          	int3
1407febab: cc                          	int3
1407febac: cc                          	int3
1407febad: cc                          	int3
1407febae: cc                          	int3
1407febaf: cc                          	int3
