
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141228300: 00 00                       	addb	%al, (%rax)
141228302: 48 8d 50 01                 	leaq	0x1(%rax), %rdx
141228306: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
14122830d: f3 0f 10 b4 c5 a4 00 00 00  	movss	0xa4(%rbp,%rax,8), %xmm6
141228316: 49 8d 48 01                 	leaq	0x1(%r8), %rcx
14122831a: 83 bc c5 a0 00 00 00 00     	cmpl	$0x0, 0xa0(%rbp,%rax,8)
141228322: 48 89 8d 88 03 00 00        	movq	%rcx, 0x388(%rbp)
141228329: 48 89 8d c0 00 00 00        	movq	%rcx, 0xc0(%rbp)
141228330: 48 89 95 90 02 00 00        	movq	%rdx, 0x290(%rbp)
141228337: 48 89 d0                    	movq	%rdx, %rax
14122833a: 74 b5                       	je	0x1412282f1 <.text+0x12272f1>
14122833c: 4c 89 85 78 03 00 00        	movq	%r8, 0x378(%rbp)
141228343: 48 c7 85 20 03 00 00 00 00 00 00    	movq	$0x0, 0x320(%rbp)
14122834e: 48 8b bd 10 03 00 00        	movq	0x310(%rbp), %rdi
141228355: 4c 8b 85 50 01 00 00        	movq	0x150(%rbp), %r8
14122835c: 4c 39 c7                    	cmpq	%r8, %rdi
14122835f: 4c 8d a5 90 00 00 00        	leaq	0x90(%rbp), %r12
141228366: 0f 82 80 06 00 00           	jb	0x1412289ec <.text+0x12279ec>
14122836c: 48 8b 9d 18 03 00 00        	movq	0x318(%rbp), %rbx
141228373: f3 0f 12 fe                 	movsldup	%xmm6, %xmm7            # xmm7 = xmm6[0,0,2,2]
141228377: 4c 89 f6                    	movq	%r14, %rsi
14122837a: 48 3b b5 80 03 00 00        	cmpq	0x380(%rbp), %rsi
141228381: 0f 84 30 01 00 00           	je	0x1412284b7 <.text+0x12274b7>
141228387: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
141228390: 4c 89 e1                    	movq	%r12, %rcx
141228393: 48 89 f2                    	movq	%rsi, %rdx
141228396: 4d 89 f8                    	movq	%r15, %r8
141228399: e8 f2 9c a6 ff              	callq	0x140c92090 <.text+0xc91090>
14122839e: 48 8b 85 20 03 00 00        	movq	0x320(%rbp), %rax
1412283a5: 48 39 f8                    	cmpq	%rdi, %rax
1412283a8: 0f 83 84 0b 00 00           	jae	0x141228f32 <.text+0x1227f32>
1412283ae: f3 0f 10 6e 08              	movss	0x8(%rsi), %xmm5
1412283b3: f3 0f 10 66 0c              	movss	0xc(%rsi), %xmm4
1412283b8: f3 0f 10 46 18              	movss	0x18(%rsi), %xmm0
1412283bd: f2 0f 10 4e 10              	movsd	0x10(%rsi), %xmm1
1412283c2: 48 8b 0e                    	movq	(%rsi), %rcx
1412283c5: 48 83 c6 20                 	addq	$0x20, %rsi
1412283c9: f2 0f 10 95 90 00 00 00     	movsd	0x90(%rbp), %xmm2
1412283d1: 0f 57 db                    	xorps	%xmm3, %xmm3
1412283d4: f3 0f 5f dc                 	maxss	%xmm4, %xmm3
1412283d8: 45 0f 28 c4                 	movaps	%xmm12, %xmm8
1412283dc: f3 44 0f 5d c3              	minss	%xmm3, %xmm8
1412283e1: 41 0f 28 d8                 	movaps	%xmm8, %xmm3
1412283e5: f3 41 0f 59 d8              	mulss	%xmm8, %xmm3
1412283ea: f3 45 0f 58 c0              	addss	%xmm8, %xmm8
1412283ef: 45 0f 28 ce                 	movaps	%xmm14, %xmm9
1412283f3: f3 45 0f 5c c8              	subss	%xmm8, %xmm9
1412283f8: f3 44 0f 59 cb              	mulss	%xmm3, %xmm9
1412283fd: 41 0f 28 dc                 	movaps	%xmm12, %xmm3
141228401: f3 41 0f 5c d9              	subss	%xmm9, %xmm3
141228406: 45 0f 28 c4                 	movaps	%xmm12, %xmm8
14122840a: f3 44 0f 5c c3              	subss	%xmm3, %xmm8
14122840f: f3 0f 59 1d 71 d5 6f 01     	mulss	0x16fd571(%rip), %xmm3  # 0x142925988
141228417: f3 41 0f 58 d8              	addss	%xmm8, %xmm3
14122841c: 44 0f 28 c5                 	movaps	%xmm5, %xmm8
141228420: f3 44 0f c2 c5 03           	cmpunordss	%xmm5, %xmm8
141228426: 45 0f 28 c8                 	movaps	%xmm8, %xmm9
14122842a: 44 0f 54 cc                 	andps	%xmm4, %xmm9
14122842e: f3 0f 5d e5                 	minss	%xmm5, %xmm4
141228432: 44 0f 55 c4                 	andnps	%xmm4, %xmm8
141228436: 45 0f 56 c1                 	orps	%xmm9, %xmm8
14122843a: 0f 28 e6                    	movaps	%xmm6, %xmm4
14122843d: f3 0f 59 e0                 	mulss	%xmm0, %xmm4
141228441: f3 0f 59 05 e7 4f 9f 01     	mulss	0x19f4fe7(%rip), %xmm0  # 0x142c1d430
141228449: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
14122844d: f3 0f 58 85 98 00 00 00     	addss	0x98(%rbp), %xmm0
141228455: 0f 28 ef                    	movaps	%xmm7, %xmm5
141228458: 0f 59 e9                    	mulps	%xmm1, %xmm5
14122845b: 0f 59 0d de 4f 9f 01        	mulps	0x19f4fde(%rip), %xmm1  # 0x142c1d440
141228462: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141228466: 48 c1 e2 04                 	shlq	$0x4, %rdx
14122846a: 0f 59 cf                    	mulps	%xmm7, %xmm1
14122846d: 0f 58 ca                    	addps	%xmm2, %xmm1
141228470: 0f 13 0c 13                 	movlps	%xmm1, (%rbx,%rdx)
141228474: f3 0f 11 44 13 08           	movss	%xmm0, 0x8(%rbx,%rdx)
14122847a: f3 44 0f 11 44 13 0c        	movss	%xmm8, 0xc(%rbx,%rdx)
141228481: 0f 13 6c 13 10              	movlps	%xmm5, 0x10(%rbx,%rdx)
141228486: f3 0f 11 64 13 18           	movss	%xmm4, 0x18(%rbx,%rdx)
14122848c: f3 0f 11 5c 13 1c           	movss	%xmm3, 0x1c(%rbx,%rdx)
141228492: 48 89 4c 13 20              	movq	%rcx, 0x20(%rbx,%rdx)
141228497: 48 c7 44 13 28 00 00 00 00  	movq	$0x0, 0x28(%rbx,%rdx)
1412284a0: 48 ff c0                    	incq	%rax
1412284a3: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1412284aa: 48 3b b5 80 03 00 00        	cmpq	0x380(%rbp), %rsi
1412284b1: 0f 85 d9 fe ff ff           	jne	0x141228390 <.text+0x1227390>
1412284b7: 48 8b b5 20 03 00 00        	movq	0x320(%rbp), %rsi
1412284be: 48 89 f0                    	movq	%rsi, %rax
1412284c1: 48 c1 e0 04                 	shlq	$0x4, %rax
1412284c5: 48 8d 3c 40                 	leaq	(%rax,%rax,2), %rdi
1412284c9: 48 8d 8d 50 03 00 00        	leaq	0x350(%rbp), %rcx
1412284d0: 48 89 fa                    	movq	%rdi, %rdx
1412284d3: 41 b8 20 00 02 00           	movl	$0x20020, %r8d          # imm = 0x20020
1412284d9: e8 42 c9 9a ff              	callq	0x140bd4e20 <.text+0xbd3e20>
1412284de: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1412284e3: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1412284e8: 48 8d 85 50 03 00 00        	leaq	0x350(%rbp), %rax
1412284ef: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1412284f4: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1412284fa: 48 8d 8d 00 02 00 00        	leaq	0x200(%rbp), %rcx
141228501: 48 8b 95 48 01 00 00        	movq	0x148(%rbp), %rdx
141228508: 4c 8d 05 e7 51 9f 01        	leaq	0x19f51e7(%rip), %r8    # 0x142c1d6f6
14122850f: e8 4c 45 db 00              	callq	0x141fdca60 <.text+0x1fdba60>
141228514: 83 bd 00 02 00 00 0b        	cmpl	$0xb, 0x200(%rbp)
14122851b: 0f 85 af fd ff ff           	jne	0x1412282d0 <.text+0x12272d0>
141228521: 48 89 b5 c8 02 00 00        	movq	%rsi, 0x2c8(%rbp)
141228528: 48 8d 8d 08 02 00 00        	leaq	0x208(%rbp), %rcx
14122852f: 48 8b 41 20                 	movq	0x20(%rcx), %rax
141228533: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
14122853a: 0f 10 01                    	movups	(%rcx), %xmm0
14122853d: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
141228541: 0f 29 8d 60 02 00 00        	movaps	%xmm1, 0x260(%rbp)
141228548: 0f 29 85 50 02 00 00        	movaps	%xmm0, 0x250(%rbp)
14122854f: c6 85 d7 03 00 00 01        	movb	$0x1, 0x3d7(%rbp)
