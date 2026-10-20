
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407f3340: 41 57                       	pushq	%r15
1407f3342: 41 56                       	pushq	%r14
1407f3344: 56                          	pushq	%rsi
1407f3345: 57                          	pushq	%rdi
1407f3346: 53                          	pushq	%rbx
1407f3347: 48 81 ec 20 01 00 00        	subq	$0x120, %rsp            # imm = 0x120
1407f334e: 44 0f 29 bc 24 10 01 00 00  	movaps	%xmm15, 0x110(%rsp)
1407f3357: 44 0f 29 b4 24 00 01 00 00  	movaps	%xmm14, 0x100(%rsp)
1407f3360: 44 0f 29 ac 24 f0 00 00 00  	movaps	%xmm13, 0xf0(%rsp)
1407f3369: 44 0f 29 a4 24 e0 00 00 00  	movaps	%xmm12, 0xe0(%rsp)
1407f3372: 44 0f 29 9c 24 d0 00 00 00  	movaps	%xmm11, 0xd0(%rsp)
1407f337b: 44 0f 29 94 24 c0 00 00 00  	movaps	%xmm10, 0xc0(%rsp)
1407f3384: 44 0f 29 8c 24 b0 00 00 00  	movaps	%xmm9, 0xb0(%rsp)
1407f338d: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407f3396: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
1407f339e: 0f 29 b4 24 80 00 00 00     	movaps	%xmm6, 0x80(%rsp)
1407f33a6: 4c 89 cf                    	movq	%r9, %rdi
1407f33a9: 4c 89 c3                    	movq	%r8, %rbx
1407f33ac: 48 89 d6                    	movq	%rdx, %rsi
1407f33af: f3 45 0f 10 60 10           	movss	0x10(%r8), %xmm12
1407f33b5: f3 41 0f 10 70 14           	movss	0x14(%r8), %xmm6
1407f33bb: f3 45 0f 10 68 18           	movss	0x18(%r8), %xmm13
1407f33c1: 4c 8b 71 08                 	movq	0x8(%rcx), %r14
1407f33c5: 4c 8b 79 10                 	movq	0x10(%rcx), %r15
1407f33c9: 4d 01 ff                    	addq	%r15, %r15
1407f33cc: f2 44 0f 10 71 50           	movsd	0x50(%rcx), %xmm14
1407f33d2: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1407f33d7: 4c 89 7c 24 38              	movq	%r15, 0x38(%rsp)
1407f33dc: f2 44 0f 11 74 24 40        	movsd	%xmm14, 0x40(%rsp)
1407f33e3: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1407f33e8: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407f33ec: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
1407f33f0: e8 ab 47 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f33f5: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1407f33f9: 0f 57 ff                    	xorps	%xmm7, %xmm7
1407f33fc: f3 44 0f 5f c7              	maxss	%xmm7, %xmm8
1407f3401: 48 89 f1                    	movq	%rsi, %rcx
1407f3404: e8 77 a9 e8 ff              	callq	0x14067dd80 <.text+0x67cd80>
1407f3409: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
1407f340d: 48 8d 50 20                 	leaq	0x20(%rax), %rdx
1407f3411: 4c 8d 40 10                 	leaq	0x10(%rax), %r8
1407f3415: 4c 8d 48 28                 	leaq	0x28(%rax), %r9
1407f3419: 80 b8 98 02 00 00 00        	cmpb	$0x0, 0x298(%rax)
1407f3420: 48 0f 45 ca                 	cmovneq	%rdx, %rcx
1407f3424: 4d 0f 44 c8                 	cmoveq	%r8, %r9
1407f3428: 49 8b 01                    	movq	(%r9), %rax
1407f342b: 48 8b 09                    	movq	(%rcx), %rcx
1407f342e: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
1407f3433: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1407f3438: c7 44 24 20 00 00 02 43     	movl	$0x43020000, 0x20(%rsp) # imm = 0x43020000
1407f3440: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1407f3445: f3 0f 10 1d 0b 82 28 02     	movss	0x228820b(%rip), %xmm3  # 0x142a7b658
1407f344d: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407f3451: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
1407f3455: e8 96 19 22 00              	callq	0x140a14df0 <.text+0xa13df0>
1407f345a: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
1407f345d: f3 0f 10 47 1c              	movss	0x1c(%rdi), %xmm0
1407f3462: f3 0f 5c 47 18              	subss	0x18(%rdi), %xmm0
1407f3467: f3 0f 59 05 79 6c 16 02     	mulss	0x2166c79(%rip), %xmm0  # 0x14295a0e8
1407f346f: f3 0f 58 c6                 	addss	%xmm6, %xmm0
1407f3473: 76 25                       	jbe	0x1407f349a <.text+0x7f249a>
1407f3475: f3 0f 10 0d 07 25 13 02     	movss	0x2132507(%rip), %xmm1  # 0x142925984
1407f347d: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1407f3480: 40 0f 93 c6                 	setae	%sil
1407f3484: 73 28                       	jae	0x1407f34ae <.text+0x7f24ae>
1407f3486: f3 0f 58 05 de bc 1e 02     	addss	0x21ebcde(%rip), %xmm0  # 0x1429df16c
1407f348e: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
1407f3492: 0f 86 60 03 00 00           	jbe	0x1407f37f8 <.text+0x7f27f8>
1407f3498: eb 14                       	jmp	0x1407f34ae <.text+0x7f24ae>
1407f349a: f3 0f 58 05 ca bc 1e 02     	addss	0x21ebcca(%rip), %xmm0  # 0x1429df16c
1407f34a2: 31 f6                       	xorl	%esi, %esi
1407f34a4: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
1407f34a8: 0f 86 4c 03 00 00           	jbe	0x1407f37fa <.text+0x7f27fa>
1407f34ae: 0f 28 33                    	movaps	(%rbx), %xmm6
1407f34b1: 44 0f 28 fe                 	movaps	%xmm6, %xmm15
1407f34b5: 44 0f 59 fe                 	mulps	%xmm6, %xmm15
1407f34b9: f3 41 0f 16 c7              	movshdup	%xmm15, %xmm0           # xmm0 = xmm15[1,1,3,3]
1407f34be: f3 41 0f 58 c7              	addss	%xmm15, %xmm0
1407f34c3: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
1407f34c7: 66 41 0f 15 cf              	unpckhpd	%xmm15, %xmm1           # xmm1 = xmm1[1],xmm15[1]
1407f34cc: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1407f34d0: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407f34d4: 45 0f c6 ff ff              	shufps	$0xff, %xmm15, %xmm15   # xmm15 = xmm15[3,3,3,3]
1407f34d9: 44 0f 5c f9                 	subps	%xmm1, %xmm15
1407f34dd: 0f 28 0d 4c 7a 29 02        	movaps	0x2297a4c(%rip), %xmm1  # 0x142a8af30
1407f34e4: 41 0f 59 cf                 	mulps	%xmm15, %xmm1
1407f34e8: 0f 28 05 41 7a 29 02        	movaps	0x2297a41(%rip), %xmm0  # 0x142a8af30
1407f34ef: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f34f2: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1407f34f6: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1407f34fa: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1407f34fd: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1407f3501: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1407f3505: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407f3509: 0f 59 c6                    	mulps	%xmm6, %xmm0
1407f350c: 0f 58 c1                    	addps	%xmm1, %xmm0
1407f350f: 44 0f 28 d6                 	movaps	%xmm6, %xmm10
1407f3513: 44 0f c6 d6 d2              	shufps	$0xd2, %xmm6, %xmm10    # xmm10 = xmm10[2,0],xmm6[1,3]
1407f3518: 44 0f 28 0d 10 7a 29 02     	movaps	0x2297a10(%rip), %xmm9  # 0x142a8af30
1407f3520: 45 0f 59 ca                 	mulps	%xmm10, %xmm9
1407f3524: 0f 57 c9                    	xorps	%xmm1, %xmm1
1407f3527: 0f 59 ce                    	mulps	%xmm6, %xmm1
1407f352a: 0f 29 4c 24 70              	movaps	%xmm1, 0x70(%rsp)
1407f352f: 44 0f 5c c9                 	subps	%xmm1, %xmm9
1407f3533: 45 0f c6 c9 d6              	shufps	$0xd6, %xmm9, %xmm9     # xmm9 = xmm9[2,1,1,3]
1407f3538: 0f 28 fe                    	movaps	%xmm6, %xmm7
1407f353b: 0f 58 fe                    	addps	%xmm6, %xmm7
1407f353e: 0f c6 ff ff                 	shufps	$0xff, %xmm7, %xmm7     # xmm7 = xmm7[3,3,3,3]
1407f3542: 44 0f 59 cf                 	mulps	%xmm7, %xmm9
1407f3546: 44 0f 58 c8                 	addps	%xmm0, %xmm9
1407f354a: 45 0f 28 dc                 	movaps	%xmm12, %xmm11
1407f354e: f3 45 0f 58 d9              	addss	%xmm9, %xmm11
1407f3553: 45 0f 12 c9                 	movhlps	%xmm9, %xmm9            # xmm9 = xmm9[1,1]
1407f3557: f3 45 0f 58 cd              	addss	%xmm13, %xmm9
1407f355c: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1407f3561: 4c 89 7c 24 38              	movq	%r15, 0x38(%rsp)
1407f3566: 44 0f 13 74 24 40           	movlps	%xmm14, 0x40(%rsp)
1407f356c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1407f3571: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
1407f3575: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
1407f3579: e8 22 46 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f357e: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1407f3582: f3 0f 5c c8                 	subss	%xmm0, %xmm1
1407f3586: 0f 2e 0d f7 23 13 02        	ucomiss	0x21323f7(%rip), %xmm1  # 0x142925984
1407f358d: 0f 86 65 02 00 00           	jbe	0x1407f37f8 <.text+0x7f27f8>
1407f3593: f3 44 0f 11 6c 24 48        	movss	%xmm13, 0x48(%rsp)
1407f359a: f3 44 0f 11 64 24 4c        	movss	%xmm12, 0x4c(%rsp)
1407f35a1: 0f 29 44 24 60              	movaps	%xmm0, 0x60(%rsp)
1407f35a6: f3 0f 10 4f 10              	movss	0x10(%rdi), %xmm1
1407f35ab: 0f 57 c0                    	xorps	%xmm0, %xmm0
1407f35ae: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
1407f35b2: 0f 29 4c 24 50              	movaps	%xmm1, 0x50(%rsp)
1407f35b7: 0f 14 c8                    	unpcklps	%xmm0, %xmm1            # xmm1 = xmm1[0],xmm0[0],xmm1[1],xmm0[1]
1407f35ba: 0f 59 0d 1f 23 13 02        	mulps	0x213231f(%rip), %xmm1  # 0x1429258e0
1407f35c1: 44 0f 28 e1                 	movaps	%xmm1, %xmm12
1407f35c5: 44 0f c6 e1 54              	shufps	$0x54, %xmm1, %xmm12    # xmm12 = xmm12[0,1],xmm1[1,1]
1407f35ca: 41 0f 28 c7                 	movaps	%xmm15, %xmm0
1407f35ce: 41 0f 59 c4                 	mulps	%xmm12, %xmm0
1407f35d2: 0f 28 d6                    	movaps	%xmm6, %xmm2
1407f35d5: 41 0f 59 d4                 	mulps	%xmm12, %xmm2
1407f35d9: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
1407f35dd: f3 0f 58 da                 	addss	%xmm2, %xmm3
1407f35e1: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1407f35e4: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407f35e8: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1407f35ec: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1407f35f0: 0f 59 d6                    	mulps	%xmm6, %xmm2
1407f35f3: 0f 58 d0                    	addps	%xmm0, %xmm2
1407f35f6: 0f c6 c9 d0                 	shufps	$0xd0, %xmm1, %xmm1     # xmm1 = xmm1[0,0,1,3]
1407f35fa: 45 0f 59 e2                 	mulps	%xmm10, %xmm12
1407f35fe: 0f 59 ce                    	mulps	%xmm6, %xmm1
1407f3601: 44 0f 5c e1                 	subps	%xmm1, %xmm12
1407f3605: 45 0f c6 e4 d6              	shufps	$0xd6, %xmm12, %xmm12   # xmm12 = xmm12[2,1,1,3]
1407f360a: 44 0f 59 e7                 	mulps	%xmm7, %xmm12
1407f360e: 44 0f 58 e2                 	addps	%xmm2, %xmm12
1407f3612: 45 0f 28 ec                 	movaps	%xmm12, %xmm13
1407f3616: 66 45 0f 15 ec              	unpckhpd	%xmm12, %xmm13          # xmm13 = xmm13[1],xmm12[1]
1407f361b: 41 0f 28 cb                 	movaps	%xmm11, %xmm1
1407f361f: f3 41 0f 5c cc              	subss	%xmm12, %xmm1
1407f3624: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
1407f3628: f3 41 0f 5c d5              	subss	%xmm13, %xmm2
1407f362d: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1407f3632: 4c 89 7c 24 38              	movq	%r15, 0x38(%rsp)
1407f3637: 44 0f 13 74 24 40           	movlps	%xmm14, 0x40(%rsp)
1407f363d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1407f3642: e8 59 45 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f3647: 41 0f 28 ce                 	movaps	%xmm14, %xmm1
1407f364b: 44 0f 28 f0                 	movaps	%xmm0, %xmm14
1407f364f: f3 45 0f 58 e3              	addss	%xmm11, %xmm12
1407f3654: f3 45 0f 58 e9              	addss	%xmm9, %xmm13
1407f3659: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1407f365e: 4c 89 7c 24 38              	movq	%r15, 0x38(%rsp)
1407f3663: 0f 13 4c 24 40              	movlps	%xmm1, 0x40(%rsp)
1407f3668: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1407f366d: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1407f3671: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
1407f3675: e8 26 45 a6 ff              	callq	0x140257ba0 <.text+0x256ba0>
1407f367a: 45 0f 28 ce                 	movaps	%xmm14, %xmm9
1407f367e: f3 45 0f c2 ce 03           	cmpunordss	%xmm14, %xmm9
1407f3684: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1407f3688: 0f 54 c8                    	andps	%xmm0, %xmm1
1407f368b: f3 41 0f 5d c6              	minss	%xmm14, %xmm0
1407f3690: 44 0f 55 c8                 	andnps	%xmm0, %xmm9
1407f3694: 44 0f 56 c9                 	orps	%xmm1, %xmm9
1407f3698: 0f 28 54 24 60              	movaps	0x60(%rsp), %xmm2
1407f369d: 0f 28 c2                    	movaps	%xmm2, %xmm0
1407f36a0: f3 41 0f 5d c1              	minss	%xmm9, %xmm0
1407f36a5: f3 45 0f c2 c9 03           	cmpunordss	%xmm9, %xmm9
1407f36ab: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1407f36af: 0f 55 c8                    	andnps	%xmm0, %xmm1
1407f36b2: 44 0f 54 ca                 	andps	%xmm2, %xmm9
1407f36b6: 44 0f 56 c9                 	orps	%xmm1, %xmm9
1407f36ba: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1407f36be: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
1407f36c3: f3 0f 10 0d 05 22 13 02     	movss	0x2132205(%rip), %xmm1  # 0x1429258d0
1407f36cb: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1407f36ce: 0f 96 c0                    	setbe	%al
1407f36d1: 89 f1                       	movl	%esi, %ecx
1407f36d3: f6 d1                       	notb	%cl
1407f36d5: 84 c1                       	testb	%al, %cl
1407f36d7: 0f 85 1b 01 00 00           	jne	0x1407f37f8 <.text+0x7f27f8>
1407f36dd: 0f b6 05 25 15 e4 02        	movzbl	0x2e41525(%rip), %eax   # 0x143634c09
1407f36e4: b9 20 00 00 00              	movl	$0x20, %ecx
1407f36e9: ba 04 00 00 00              	movl	$0x4, %edx
1407f36ee: e8 1d 05 e2 ff              	callq	0x140613c10 <.text+0x612c10>
1407f36f3: 48 85 c0                    	testq	%rax, %rax
1407f36f6: 0f 84 67 01 00 00           	je	0x1407f3863 <.text+0x7f2863>
1407f36fc: 48 89 c7                    	movq	%rax, %rdi
1407f36ff: 48 8b 9c 24 70 01 00 00     	movq	0x170(%rsp), %rbx
1407f3707: f3 0f 10 0d 41 c3 28 02     	movss	0x228c341(%rip), %xmm1  # 0x142a7fa50
1407f370f: 0f 28 c1                    	movaps	%xmm1, %xmm0
1407f3712: f3 41 0f 5c c1              	subss	%xmm9, %xmm0
1407f3717: 0f 57 d2                    	xorps	%xmm2, %xmm2
1407f371a: f3 0f 5f c2                 	maxss	%xmm2, %xmm0
1407f371e: 44 0f 59 15 9a 99 1d 02     	mulps	0x21d999a(%rip), %xmm10 # 0x1429cd0c0
1407f3726: 44 0f 5c 54 24 70           	subps	0x70(%rsp), %xmm10
1407f372c: 45 0f c6 d2 d6              	shufps	$0xd6, %xmm10, %xmm10   # xmm10 = xmm10[2,1,1,3]
1407f3731: 41 0f 59 fa                 	mulps	%xmm10, %xmm7
1407f3735: 44 0f 59 3d 83 99 1d 02     	mulps	0x21d9983(%rip), %xmm15 # 0x1429cd0c0
1407f373d: 0f 28 1d 7c 99 1d 02        	movaps	0x21d997c(%rip), %xmm3  # 0x1429cd0c0
1407f3744: 0f 59 de                    	mulps	%xmm6, %xmm3
1407f3747: f3 0f 16 d3                 	movshdup	%xmm3, %xmm2            # xmm2 = xmm3[1,1,3,3]
1407f374b: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1407f374f: 0f 12 db                    	movhlps	%xmm3, %xmm3            # xmm3 = xmm3[1,1]
1407f3752: f3 0f 58 da                 	addss	%xmm2, %xmm3
1407f3756: f3 0f 58 db                 	addss	%xmm3, %xmm3
1407f375a: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
1407f375e: 0f 59 de                    	mulps	%xmm6, %xmm3
1407f3761: 41 0f 58 df                 	addps	%xmm15, %xmm3
1407f3765: 0f 58 df                    	addps	%xmm7, %xmm3
1407f3768: 0f 28 d3                    	movaps	%xmm3, %xmm2
1407f376b: 0f c6 d3 e8                 	shufps	$0xe8, %xmm3, %xmm2     # xmm2 = xmm2[0,2],xmm3[2,3]
1407f376f: 0f 59 db                    	mulps	%xmm3, %xmm3
1407f3772: 0f 28 e3                    	movaps	%xmm3, %xmm4
1407f3775: 66 0f 15 e3                 	unpckhpd	%xmm3, %xmm4            # xmm4 = xmm4[1],xmm3[1]
1407f3779: f3 0f 58 e3                 	addss	%xmm3, %xmm4
1407f377d: f3 0f 51 e4                 	sqrtss	%xmm4, %xmm4
1407f3781: f3 0f 10 1d 7b 21 13 02     	movss	0x213217b(%rip), %xmm3  # 0x142925904
1407f3789: f3 0f 5e dc                 	divss	%xmm4, %xmm3
1407f378d: 40 84 f6                    	testb	%sil, %sil
1407f3790: f3 0f 10 64 24 4c           	movss	0x4c(%rsp), %xmm4
1407f3796: f3 0f 10 6c 24 48           	movss	0x48(%rsp), %xmm5
1407f379c: 75 04                       	jne	0x1407f37a2 <.text+0x7f27a2>
1407f379e: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1407f37a2: f3 0f 11 27                 	movss	%xmm4, (%rdi)
1407f37a6: f3 0f 11 6f 04              	movss	%xmm5, 0x4(%rdi)
1407f37ab: f3 0f 12 db                 	movsldup	%xmm3, %xmm3            # xmm3 = xmm3[0,0,2,2]
1407f37af: 0f 59 d3                    	mulps	%xmm3, %xmm2
1407f37b2: 0f 13 57 08                 	movlps	%xmm2, 0x8(%rdi)
1407f37b6: f3 44 0f 11 47 10           	movss	%xmm8, 0x10(%rdi)
1407f37bc: f3 0f 11 4f 14              	movss	%xmm1, 0x14(%rdi)
1407f37c1: f3 0f 11 47 18              	movss	%xmm0, 0x18(%rdi)
1407f37c6: 0f 28 44 24 50              	movaps	0x50(%rsp), %xmm0
1407f37cb: f3 0f 11 47 1c              	movss	%xmm0, 0x1c(%rdi)
1407f37d0: 48 8b 8b 20 01 00 00        	movq	0x120(%rbx), %rcx
1407f37d7: 48 85 c9                    	testq	%rcx, %rcx
1407f37da: 74 10                       	je	0x1407f37ec <.text+0x7f27ec>
1407f37dc: ba 20 00 00 00              	movl	$0x20, %edx
1407f37e1: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407f37e7: e8 34 04 e2 ff              	callq	0x140613c20 <.text+0x612c20>
1407f37ec: 48 89 bb 20 01 00 00        	movq	%rdi, 0x120(%rbx)
1407f37f3: 40 b6 01                    	movb	$0x1, %sil
1407f37f6: eb 02                       	jmp	0x1407f37fa <.text+0x7f27fa>
1407f37f8: 31 f6                       	xorl	%esi, %esi
1407f37fa: 89 f0                       	movl	%esi, %eax
1407f37fc: 0f 28 b4 24 80 00 00 00     	movaps	0x80(%rsp), %xmm6
1407f3804: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
1407f380c: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407f3815: 44 0f 28 8c 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm9
1407f381e: 44 0f 28 94 24 c0 00 00 00  	movaps	0xc0(%rsp), %xmm10
1407f3827: 44 0f 28 9c 24 d0 00 00 00  	movaps	0xd0(%rsp), %xmm11
1407f3830: 44 0f 28 a4 24 e0 00 00 00  	movaps	0xe0(%rsp), %xmm12
1407f3839: 44 0f 28 ac 24 f0 00 00 00  	movaps	0xf0(%rsp), %xmm13
1407f3842: 44 0f 28 b4 24 00 01 00 00  	movaps	0x100(%rsp), %xmm14
1407f384b: 44 0f 28 bc 24 10 01 00 00  	movaps	0x110(%rsp), %xmm15
1407f3854: 48 81 c4 20 01 00 00        	addq	$0x120, %rsp            # imm = 0x120
1407f385b: 5b                          	popq	%rbx
1407f385c: 5f                          	popq	%rdi
1407f385d: 5e                          	popq	%rsi
1407f385e: 41 5e                       	popq	%r14
1407f3860: 41 5f                       	popq	%r15
1407f3862: c3                          	retq
1407f3863: b9 04 00 00 00              	movl	$0x4, %ecx
1407f3868: ba 20 00 00 00              	movl	$0x20, %edx
1407f386d: e8 8e 57 0e 02              	callq	0x1428d9000 <.text+0x28d8000>
1407f3872: cc                          	int3
1407f3873: cc                          	int3
1407f3874: cc                          	int3
1407f3875: cc                          	int3
1407f3876: cc                          	int3
1407f3877: cc                          	int3
1407f3878: cc                          	int3
1407f3879: cc                          	int3
1407f387a: cc                          	int3
1407f387b: cc                          	int3
1407f387c: cc                          	int3
1407f387d: cc                          	int3
1407f387e: cc                          	int3
1407f387f: cc                          	int3
1407f3880: 48 83 ec 68                 	subq	$0x68, %rsp
1407f3884: 48 89 d0                    	movq	%rdx, %rax
1407f3887: 80 39 1a                    	cmpb	$0x1a, (%rcx)
1407f388a: 75 19                       	jne	0x1407f38a5 <.text+0x7f28a5>
1407f388c: 48 8d 15 1f 7e 29 02        	leaq	0x2297e1f(%rip), %rdx   # 0x142a8b6b2
1407f3893: 41 b8 16 00 00 00           	movl	$0x16, %r8d
1407f3899: 48 89 c1                    	movq	%rax, %rcx
1407f389c: 48 83 c4 68                 	addq	$0x68, %rsp
1407f38a0: e9 eb 32 59 00              	jmp	0x140d86b90 <.text+0xd85b90>
1407f38a5: 48 89 4c 24 60              	movq	%rcx, 0x60(%rsp)
1407f38aa: 48 ff c1                    	incq	%rcx
1407f38ad: 48 8d 15 cc 7d 29 02        	leaq	0x2297dcc(%rip), %rdx   # 0x142a8b680
1407f38b4: 48 89 54 24 50              	movq	%rdx, 0x50(%rsp)
1407f38b9: 48 8d 54 24 60              	leaq	0x60(%rsp), %rdx
1407f38be: 48 89 54 24 48              	movq	%rdx, 0x48(%rsp)
