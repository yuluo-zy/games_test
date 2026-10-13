
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140255ddc: 31 db                       	xorl	%ebx, %ebx
140255dde: 48 39 bd 58 01 00 00        	cmpq	%rdi, 0x158(%rbp)
140255de5: 48 c7 85 f0 02 00 00 00 00 00 00    	movq	$0x0, 0x2f0(%rbp)
140255df0: 66 c7 85 00 03 00 00 00 01  	movw	$0x100, 0x300(%rbp)     # imm = 0x100
140255df9: 48 8b 05 80 55 3d 03        	movq	0x33d5580(%rip), %rax   # 0x14362b380
140255e00: b1 01                       	movb	$0x1, %cl
140255e02: 86 08                       	xchgb	%cl, (%rax)
140255e04: 40 0f 94 c7                 	sete	%dil
140255e08: be 03 00 00 00              	movl	$0x3, %esi
140255e0d: 84 c9                       	testb	%cl, %cl
140255e0f: 74 08                       	je	0x140255e19 <.text+0x254e19>
140255e11: 41 be 03 00 00 00           	movl	$0x3, %r14d
140255e17: eb 72                       	jmp	0x140255e8b <.text+0x254e8b>
140255e19: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
140255e20: 48 8d 95 f0 02 00 00        	leaq	0x2f0(%rbp), %rdx
140255e27: e8 b4 6d 2e 00              	callq	0x14053cbe0 <.text+0x53bbe0>
140255e2c: 48 8b 85 60 03 00 00        	movq	0x360(%rbp), %rax
140255e33: 44 8b b5 68 03 00 00        	movl	0x368(%rbp), %r14d
140255e3a: 0f 10 85 6c 03 00 00        	movups	0x36c(%rbp), %xmm0
140255e41: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
140255e48: 0f 10 85 7c 03 00 00        	movups	0x37c(%rbp), %xmm0
140255e4f: 0f 29 85 d0 01 00 00        	movaps	%xmm0, 0x1d0(%rbp)
140255e56: 8b 8d 8c 03 00 00           	movl	0x38c(%rbp), %ecx
140255e5c: 89 8d e0 01 00 00           	movl	%ecx, 0x1e0(%rbp)
140255e62: 48 83 f8 03                 	cmpq	$0x3, %rax
140255e66: 0f 85 2d 05 00 00           	jne	0x140256399 <.text+0x255399>
140255e6c: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
140255e73: 0f 28 8d d0 01 00 00        	movaps	0x1d0(%rbp), %xmm1
140255e7a: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140255e7e: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
140255e82: 8b 85 e0 01 00 00           	movl	0x1e0(%rbp), %eax
140255e88: 89 45 d0                    	movl	%eax, -0x30(%rbp)
140255e8b: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
140255e92: 48 89 85 50 03 00 00        	movq	%rax, 0x350(%rbp)
140255e99: 48 85 c0                    	testq	%rax, %rax
140255e9c: 74 3f                       	je	0x140255edd <.text+0x254edd>
140255e9e: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
140255ea5: 48 89 85 58 03 00 00        	movq	%rax, 0x358(%rbp)
140255eac: 48 8b 00                    	movq	(%rax), %rax
140255eaf: 48 85 c0                    	testq	%rax, %rax
140255eb2: 74 09                       	je	0x140255ebd <.text+0x254ebd>
140255eb4: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
140255ebb: ff d0                       	callq	*%rax
140255ebd: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
140255ec4: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140255ec8: 48 85 d2                    	testq	%rdx, %rdx
140255ecb: 74 10                       	je	0x140255edd <.text+0x254edd>
140255ecd: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140255ed1: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
140255ed8: e8 43 dd 3b 00              	callq	0x140613c20 <.text+0x612c20>
140255edd: 83 fe 03                    	cmpl	$0x3, %esi
140255ee0: 0f 84 78 05 00 00           	je	0x14025645e <.text+0x25545e>
140255ee6: 48 89 b5 e8 00 00 00        	movq	%rsi, 0xe8(%rbp)
140255eed: 44 89 b5 f0 00 00 00        	movl	%r14d, 0xf0(%rbp)
140255ef4: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
140255ef8: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
140255efc: 0f 11 85 f4 00 00 00        	movups	%xmm0, 0xf4(%rbp)
140255f03: 0f 11 8d 04 01 00 00        	movups	%xmm1, 0x104(%rbp)
140255f0a: 8b 45 d0                    	movl	-0x30(%rbp), %eax
140255f0d: 89 85 14 01 00 00           	movl	%eax, 0x114(%rbp)
140255f13: 4c 89 bd 18 01 00 00        	movq	%r15, 0x118(%rbp)
140255f1a: 0f b6 05 e8 ec 3d 03        	movzbl	0x33dece8(%rip), %eax   # 0x143634c09
140255f21: b9 0c 00 00 00              	movl	$0xc, %ecx
140255f26: ba 01 00 00 00              	movl	$0x1, %edx
140255f2b: e8 e0 dc 3b 00              	callq	0x140613c10 <.text+0x612c10>
140255f30: 48 85 c0                    	testq	%rax, %rax
140255f33: 0f 84 7b 05 00 00           	je	0x1402564b4 <.text+0x2554b4>
140255f39: 48 b9 77 69 6e 69 74 20 77 69       	movabsq	$0x69772074696e6977, %rcx # imm = 0x69772074696E6977
140255f43: 48 89 08                    	movq	%rcx, (%rax)
140255f46: c7 40 08 6e 64 6f 77        	movl	$0x776f646e, 0x8(%rax)  # imm = 0x776F646E
140255f4d: c7 85 00 02 00 00 02 00 00 00       	movl	$0x2, 0x200(%rbp)
140255f57: c7 85 18 02 00 00 02 00 00 00       	movl	$0x2, 0x218(%rbp)
140255f61: c7 85 30 02 00 00 02 00 00 00       	movl	$0x2, 0x230(%rbp)
140255f6b: c7 85 48 02 00 00 02 00 00 00       	movl	$0x2, 0x248(%rbp)
140255f75: c6 85 9c 02 00 00 01        	movb	$0x1, 0x29c(%rbp)
140255f7c: c7 85 98 02 00 00 07 00 00 00       	movl	$0x7, 0x298(%rbp)
140255f86: 48 c7 85 e8 01 00 00 0c 00 00 00    	movq	$0xc, 0x1e8(%rbp)
140255f91: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140255f98: 48 c7 85 f8 01 00 00 0c 00 00 00    	movq	$0xc, 0x1f8(%rbp)
140255fa3: c7 85 9d 02 00 00 00 01 00 00       	movl	$0x100, 0x29d(%rbp) # imm = 0x100
140255fad: c6 85 a1 02 00 00 01        	movb	$0x1, 0x2a1(%rbp)
140255fb4: 48 c7 85 90 02 00 00 00 00 00 00    	movq	$0x0, 0x290(%rbp)
140255fbf: c7 85 60 02 00 00 02 00 00 00       	movl	$0x2, 0x260(%rbp)
140255fc9: c7 85 a2 02 00 00 00 01 02 01       	movl	$0x1020100, 0x2a2(%rbp) # imm = 0x1020100
140255fd3: c7 85 78 02 00 00 0f 00 00 00       	movl	$0xf, 0x278(%rbp)
140255fdd: 48 c7 85 c0 01 00 00 00 00 00 00    	movq	$0x0, 0x1c0(%rbp)
140255fe8: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
140255fef: e8 dc 15 44 00              	callq	0x1406975d0 <.text+0x6965d0>
140255ff4: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
140255ffb: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
140256002: 41 b8 e8 00 00 00           	movl	$0xe8, %r8d
140256008: e8 f3 db 6c 02              	callq	0x142923c00 <.text+0x2922c00>
14025600d: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
140256014: 0f 10 8d 00 03 00 00        	movups	0x300(%rbp), %xmm1
14025601b: 0f 10 95 10 03 00 00        	movups	0x310(%rbp), %xmm2
140256022: 0f 10 9d 20 03 00 00        	movups	0x320(%rbp), %xmm3
140256029: 0f 11 85 48 04 00 00        	movups	%xmm0, 0x448(%rbp)
140256030: 0f 11 8d 58 04 00 00        	movups	%xmm1, 0x458(%rbp)
140256037: 0f 11 95 68 04 00 00        	movups	%xmm2, 0x468(%rbp)
14025603e: 0f 11 9d 78 04 00 00        	movups	%xmm3, 0x478(%rbp)
140256045: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
14025604c: 48 89 85 88 04 00 00        	movq	%rax, 0x488(%rbp)
140256053: 0f b6 05 af eb 3d 03        	movzbl	0x33debaf(%rip), %eax   # 0x143634c09
14025605a: b9 23 00 00 00              	movl	$0x23, %ecx
14025605f: ba 01 00 00 00              	movl	$0x1, %edx
140256064: e8 a7 db 3b 00              	callq	0x140613c10 <.text+0x612c10>
140256069: 48 85 c0                    	testq	%rax, %rax
14025606c: 0f 84 61 04 00 00           	je	0x1402564d3 <.text+0x2554d3>
140256072: 48 89 c6                    	movq	%rax, %rsi
140256075: 40 88 fb                    	movb	%dil, %bl
140256078: c1 e3 03                    	shll	$0x3, %ebx
14025607b: 48 8d 05 4e cf 74 02        	leaq	0x274cf4e(%rip), %rax   # 0x1429a2fd0
140256082: f2 0f 10 34 03              	movsd	(%rbx,%rax), %xmm6
140256087: 0f 10 05 6a db 74 02        	movups	0x274db6a(%rip), %xmm0  # 0x1429a3bf8
14025608e: 0f 11 46 10                 	movups	%xmm0, 0x10(%rsi)
140256092: 0f 10 05 4f db 74 02        	movups	0x274db4f(%rip), %xmm0  # 0x1429a3be8
140256099: 0f 11 06                    	movups	%xmm0, (%rsi)
14025609c: c7 46 1f 70 6f 72 74        	movl	$0x74726f70, 0x1f(%rsi) # imm = 0x74726F70
1402560a3: 48 8b 95 88 03 00 00        	movq	0x388(%rbp), %rdx
1402560aa: 48 85 d2                    	testq	%rdx, %rdx
1402560ad: 74 12                       	je	0x1402560c1 <.text+0x2550c1>
1402560af: 48 8b 8d 90 03 00 00        	movq	0x390(%rbp), %rcx
1402560b6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1402560bc: e8 5f db 3b 00              	callq	0x140613c20 <.text+0x612c20>
1402560c1: 48 c7 85 88 03 00 00 23 00 00 00    	movq	$0x23, 0x388(%rbp)
1402560cc: 48 89 b5 90 03 00 00        	movq	%rsi, 0x390(%rbp)
1402560d3: 48 c7 85 98 03 00 00 23 00 00 00    	movq	$0x23, 0x398(%rbp)
1402560de: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
1402560e5: 48 8d 95 60 03 00 00        	leaq	0x360(%rbp), %rdx
1402560ec: 41 b8 30 01 00 00           	movl	$0x130, %r8d            # imm = 0x130
1402560f2: e8 09 db 6c 02              	callq	0x142923c00 <.text+0x2922c00>
1402560f7: 0f 28 c6                    	movaps	%xmm6, %xmm0
1402560fa: e8 f1 16 44 00              	callq	0x1406977f0 <.text+0x6967f0>
1402560ff: 89 c6                       	movl	%eax, %esi
140256101: 48 8d 05 d8 ce 74 02        	leaq	0x274ced8(%rip), %rax   # 0x1429a2fe0
140256108: f2 0f 10 04 03              	movsd	(%rbx,%rax), %xmm0
14025610d: e8 de 16 44 00              	callq	0x1406977f0 <.text+0x6967f0>
140256112: c7 85 00 02 00 00 00 00 00 00       	movl	$0x0, 0x200(%rbp)
14025611c: 89 b5 04 02 00 00           	movl	%esi, 0x204(%rbp)
140256122: 89 85 08 02 00 00           	movl	%eax, 0x208(%rbp)
140256128: 48 8d 75 b0                 	leaq	-0x50(%rbp), %rsi
14025612c: 48 8d 95 c0 01 00 00        	leaq	0x1c0(%rbp), %rdx
140256133: 41 b8 30 01 00 00           	movl	$0x130, %r8d            # imm = 0x130
140256139: 48 89 f1                    	movq	%rsi, %rcx
14025613c: e8 bf da 6c 02              	callq	0x142923c00 <.text+0x2922c00>
140256141: 48 8d bd f8 00 00 00        	leaq	0xf8(%rbp), %rdi
140256148: 48 8d 9d 60 03 00 00        	leaq	0x360(%rbp), %rbx
14025614f: 41 b8 e8 00 00 00           	movl	$0xe8, %r8d
140256155: 48 89 d9                    	movq	%rbx, %rcx
140256158: 48 89 f2                    	movq	%rsi, %rdx
14025615b: e8 a0 da 6c 02              	callq	0x142923c00 <.text+0x2922c00>
140256160: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
140256167: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
14025616e: 0f 10 85 98 00 00 00        	movups	0x98(%rbp), %xmm0
140256175: 0f 10 8d a8 00 00 00        	movups	0xa8(%rbp), %xmm1
14025617c: 0f 10 95 b8 00 00 00        	movups	0xb8(%rbp), %xmm2
140256183: 0f 10 9d c8 00 00 00        	movups	0xc8(%rbp), %xmm3
14025618a: 0f 29 9d f0 01 00 00        	movaps	%xmm3, 0x1f0(%rbp)
140256191: 0f 29 95 e0 01 00 00        	movaps	%xmm2, 0x1e0(%rbp)
140256198: 0f 29 8d d0 01 00 00        	movaps	%xmm1, 0x1d0(%rbp)
14025619f: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1402561a6: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
1402561ad: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
1402561b4: 4c 8d 85 c0 01 00 00        	leaq	0x1c0(%rbp), %r8
1402561bb: 48 89 da                    	movq	%rbx, %rdx
1402561be: 49 89 f9                    	movq	%rdi, %r9
1402561c1: e8 1a 82 e2 ff              	callq	0x14007e3e0 <.text+0x7d3e0>
1402561c6: 0f 10 85 f8 02 00 00        	movups	0x2f8(%rbp), %xmm0
1402561cd: 0f 10 8d 08 03 00 00        	movups	0x308(%rbp), %xmm1
1402561d4: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1402561db: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
1402561e2: f6 85 f0 02 00 00 01        	testb	$0x1, 0x2f0(%rbp)
1402561e9: 0f 85 fc 02 00 00           	jne	0x1402564eb <.text+0x2554eb>
1402561ef: 0f 28 85 30 01 00 00        	movaps	0x130(%rbp), %xmm0
1402561f6: 0f 28 8d 40 01 00 00        	movaps	0x140(%rbp), %xmm1
1402561fd: 0f 29 8d b0 01 00 00        	movaps	%xmm1, 0x1b0(%rbp)
140256204: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
14025620b: 48 8b bd a0 01 00 00        	movq	0x1a0(%rbp), %rdi
140256212: 48 8d 77 10                 	leaq	0x10(%rdi), %rsi
140256216: b1 01                       	movb	$0x1, %cl
140256218: 31 c0                       	xorl	%eax, %eax
14025621a: f0                          	lock
14025621b: 0f b0 4f 10                 	cmpxchgb	%cl, 0x10(%rdi)
14025621f: 0f 85 b9 01 00 00           	jne	0x1402563de <.text+0x2553de>
140256225: 48 8b 1d f4 61 3d 03        	movq	0x33d61f4(%rip), %rbx   # 0x14362c420
14025622c: 48 8b 03                    	movq	(%rbx), %rax
14025622f: 48 d1 e0                    	shlq	%rax
140256232: 48 85 c0                    	testq	%rax, %rax
140256235: 0f 85 b0 01 00 00           	jne	0x1402563eb <.text+0x2553eb>
14025623b: 31 c0                       	xorl	%eax, %eax
14025623d: 0f b6 4f 11                 	movzbl	0x11(%rdi), %ecx
140256241: 84 c9                       	testb	%cl, %cl
140256243: 0f 85 b5 01 00 00           	jne	0x1402563fe <.text+0x2553fe>
140256249: c6 87 e4 00 00 00 01        	movb	$0x1, 0xe4(%rdi)
140256250: 84 c0                       	testb	%al, %al
140256252: 75 0f                       	jne	0x140256263 <.text+0x255263>
140256254: 48 8b 03                    	movq	(%rbx), %rax
140256257: 48 d1 e0                    	shlq	%rax
14025625a: 48 85 c0                    	testq	%rax, %rax
14025625d: 0f 85 e5 01 00 00           	jne	0x140256448 <.text+0x255448>
140256263: 31 c0                       	xorl	%eax, %eax
140256265: 86 06                       	xchgb	%al, (%rsi)
140256267: 3c 02                       	cmpb	$0x2, %al
140256269: 0f 84 cc 01 00 00           	je	0x14025643b <.text+0x25543b>
14025626f: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
140256276: 31 d2                       	xorl	%edx, %edx
140256278: 45 31 c0                    	xorl	%r8d, %r8d
14025627b: 41 b9 02 00 00 00           	movl	$0x2, %r9d
140256281: ff 15 11 c7 f3 02           	callq	*0x2f3c711(%rip)        # 0x143192998
140256287: 0f 28 85 a0 01 00 00        	movaps	0x1a0(%rbp), %xmm0
14025628e: 0f 28 8d b0 01 00 00        	movaps	0x1b0(%rbp), %xmm1
140256295: 0f 29 8d d0 01 00 00        	movaps	%xmm1, 0x1d0(%rbp)
14025629c: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
1402562a3: 0f 11 8d 80 03 00 00        	movups	%xmm1, 0x380(%rbp)
1402562aa: 0f 11 85 70 03 00 00        	movups	%xmm0, 0x370(%rbp)
1402562b1: 48 c7 85 60 03 00 00 01 00 00 00    	movq	$0x1, 0x360(%rbp)
1402562bc: 48 c7 85 68 03 00 00 01 00 00 00    	movq	$0x1, 0x368(%rbp)
1402562c7: 0f b6 05 3b e9 3d 03        	movzbl	0x33de93b(%rip), %eax   # 0x143634c09
1402562ce: b9 30 00 00 00              	movl	$0x30, %ecx
1402562d3: ba 08 00 00 00              	movl	$0x8, %edx
1402562d8: e8 33 d9 3b 00              	callq	0x140613c10 <.text+0x612c10>
1402562dd: 48 85 c0                    	testq	%rax, %rax
1402562e0: 0f 84 5c 02 00 00           	je	0x140256542 <.text+0x255542>
1402562e6: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
1402562ed: 0f 10 8d 70 03 00 00        	movups	0x370(%rbp), %xmm1
1402562f4: 0f 10 95 80 03 00 00        	movups	0x380(%rbp), %xmm2
1402562fb: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1402562ff: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140256303: 0f 11 00                    	movups	%xmm0, (%rax)
140256306: 48 8b 8d 98 01 00 00        	movq	0x198(%rbp), %rcx
14025630d: 48 89 8d a0 03 00 00        	movq	%rcx, 0x3a0(%rbp)
140256314: 0f 10 85 58 01 00 00        	movups	0x158(%rbp), %xmm0
14025631b: 0f 10 8d 68 01 00 00        	movups	0x168(%rbp), %xmm1
140256322: 0f 10 95 78 01 00 00        	movups	0x178(%rbp), %xmm2
140256329: 0f 10 9d 88 01 00 00        	movups	0x188(%rbp), %xmm3
140256330: 0f 29 9d 90 03 00 00        	movaps	%xmm3, 0x390(%rbp)
140256337: 0f 29 95 80 03 00 00        	movaps	%xmm2, 0x380(%rbp)
14025633e: 0f 29 8d 70 03 00 00        	movaps	%xmm1, 0x370(%rbp)
140256345: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
14025634c: c6 85 96 04 00 00 00        	movb	$0x0, 0x496(%rbp)
140256353: 48 8d 8d e8 00 00 00        	leaq	0xe8(%rbp), %rcx
14025635a: 4c 8d 85 60 03 00 00        	leaq	0x360(%rbp), %r8
140256361: 48 89 c2                    	movq	%rax, %rdx
140256364: e8 67 85 3d 00              	callq	0x14062e8d0 <.text+0x62d8d0>
140256369: 48 89 c6                    	movq	%rax, %rsi
14025636c: c6 85 97 04 00 00 00        	movb	$0x0, 0x497(%rbp)
140256373: 48 8d 8d e8 00 00 00        	leaq	0xe8(%rbp), %rcx
14025637a: e8 51 34 3a 00              	callq	0x1405f97d0 <.text+0x5f87d0>
14025637f: 48 89 f0                    	movq	%rsi, %rax
140256382: 0f 28 b5 a0 04 00 00        	movaps	0x4a0(%rbp), %xmm6
140256389: 48 81 c4 38 05 00 00        	addq	$0x538, %rsp            # imm = 0x538
140256390: 5b                          	popq	%rbx
140256391: 5f                          	popq	%rdi
140256392: 5e                          	popq	%rsi
140256393: 41 5e                       	popq	%r14
140256395: 41 5f                       	popq	%r15
140256397: 5d                          	popq	%rbp
140256398: c3                          	retq
140256399: 4c 8b bd 90 03 00 00        	movq	0x390(%rbp), %r15
1402563a0: 0f 28 85 c0 01 00 00        	movaps	0x1c0(%rbp), %xmm0
1402563a7: 0f 28 8d d0 01 00 00        	movaps	0x1d0(%rbp), %xmm1
1402563ae: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1402563b2: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
1402563b6: 8b 8d e0 01 00 00           	movl	0x1e0(%rbp), %ecx
1402563bc: 89 4d d0                    	movl	%ecx, -0x30(%rbp)
1402563bf: 48 89 c6                    	movq	%rax, %rsi
1402563c2: 48 8b 85 f0 02 00 00        	movq	0x2f0(%rbp), %rax
1402563c9: 48 89 85 50 03 00 00        	movq	%rax, 0x350(%rbp)
1402563d0: 48 85 c0                    	testq	%rax, %rax
1402563d3: 0f 85 c5 fa ff ff           	jne	0x140255e9e <.text+0x254e9e>
1402563d9: e9 ff fa ff ff              	jmp	0x140255edd <.text+0x254edd>
1402563de: 48 89 f1                    	movq	%rsi, %rcx
1402563e1: e8 6a 23 68 02              	callq	0x1428d8750 <.text+0x28d7750>
1402563e6: e9 3a fe ff ff              	jmp	0x140256225 <.text+0x255225>
1402563eb: e8 c0 22 68 02              	callq	0x1428d86b0 <.text+0x28d76b0>
1402563f0: 34 01                       	xorb	$0x1, %al
1402563f2: 0f b6 4f 11                 	movzbl	0x11(%rdi), %ecx
1402563f6: 84 c9                       	testb	%cl, %cl
1402563f8: 0f 84 4b fe ff ff           	je	0x140256249 <.text+0x255249>
1402563fe: 48 89 b5 60 03 00 00        	movq	%rsi, 0x360(%rbp)
140256405: 88 85 68 03 00 00           	movb	%al, 0x368(%rbp)
14025640b: 48 8d 05 1e d3 74 02        	leaq	0x274d31e(%rip), %rax   # 0x1429a3730
140256412: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140256417: 48 8d 0d 82 d1 74 02        	leaq	0x274d182(%rip), %rcx   # 0x1429a35a0
14025641e: 4c 8d 0d 5b d1 74 02        	leaq	0x274d15b(%rip), %r9    # 0x1429a3580
140256425: 4c 8d 85 60 03 00 00        	leaq	0x360(%rbp), %r8
14025642c: ba 2b 00 00 00              	movl	$0x2b, %edx
140256431: e8 2a 33 68 02              	callq	0x1428d9760 <.text+0x28d8760>
140256436: e9 16 01 00 00              	jmp	0x140256551 <.text+0x255551>
14025643b: 48 89 f1                    	movq	%rsi, %rcx
14025643e: e8 cd 23 68 02              	callq	0x1428d8810 <.text+0x28d7810>
140256443: e9 27 fe ff ff              	jmp	0x14025626f <.text+0x25526f>
140256448: e8 63 22 68 02              	callq	0x1428d86b0 <.text+0x28d76b0>
14025644d: 84 c0                       	testb	%al, %al
14025644f: 0f 85 0e fe ff ff           	jne	0x140256263 <.text+0x255263>
140256455: c6 47 11 01                 	movb	$0x1, 0x11(%rdi)
140256459: e9 05 fe ff ff              	jmp	0x140256263 <.text+0x255263>
14025645e: 44 89 b5 60 03 00 00        	movl	%r14d, 0x360(%rbp)
140256465: 0f 28 45 b0                 	movaps	-0x50(%rbp), %xmm0
140256469: 0f 28 4d c0                 	movaps	-0x40(%rbp), %xmm1
14025646d: 0f 11 85 64 03 00 00        	movups	%xmm0, 0x364(%rbp)
140256474: 0f 11 8d 74 03 00 00        	movups	%xmm1, 0x374(%rbp)
14025647b: 8b 45 d0                    	movl	-0x30(%rbp), %eax
14025647e: 89 85 84 03 00 00           	movl	%eax, 0x384(%rbp)
140256484: 48 8d 05 45 d7 74 02        	leaq	0x274d745(%rip), %rax   # 0x1429a3bd0
14025648b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140256490: 48 8d 0d 09 d1 74 02        	leaq	0x274d109(%rip), %rcx   # 0x1429a35a0
140256497: 4c 8d 0d 32 d1 74 02        	leaq	0x274d132(%rip), %r9    # 0x1429a35d0
14025649e: 4c 8d 85 60 03 00 00        	leaq	0x360(%rbp), %r8
1402564a5: ba 2b 00 00 00              	movl	$0x2b, %edx
1402564aa: e8 b1 32 68 02              	callq	0x1428d9760 <.text+0x28d8760>
1402564af: e9 9d 00 00 00              	jmp	0x140256551 <.text+0x255551>
1402564b4: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
1402564bb: 4c 8d 05 7e d1 74 02        	leaq	0x274d17e(%rip), %r8    # 0x1429a3640
1402564c2: b9 01 00 00 00              	movl	$0x1, %ecx
1402564c7: ba 0c 00 00 00              	movl	$0xc, %edx
1402564cc: e8 12 2b 68 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1402564d1: eb 7e                       	jmp	0x140256551 <.text+0x255551>
1402564d3: 4c 8d 05 66 d1 74 02        	leaq	0x274d166(%rip), %r8    # 0x1429a3640
1402564da: b9 01 00 00 00              	movl	$0x1, %ecx
1402564df: ba 23 00 00 00              	movl	$0x23, %edx
1402564e4: e8 fa 2a 68 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1402564e9: eb 66                       	jmp	0x140256551 <.text+0x255551>
1402564eb: 0f 28 85 30 01 00 00        	movaps	0x130(%rbp), %xmm0
1402564f2: 0f 28 8d 40 01 00 00        	movaps	0x140(%rbp), %xmm1
1402564f9: 0f 29 8d d0 01 00 00        	movaps	%xmm1, 0x1d0(%rbp)
140256500: 0f 29 85 c0 01 00 00        	movaps	%xmm0, 0x1c0(%rbp)
140256507: 0f 29 8d 70 03 00 00        	movaps	%xmm1, 0x370(%rbp)
14025650e: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
140256515: 48 8d 05 fc d6 74 02        	leaq	0x274d6fc(%rip), %rax   # 0x1429a3c18
14025651c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140256521: 48 8d 0d e3 d6 74 02        	leaq	0x274d6e3(%rip), %rcx   # 0x1429a3c0b
140256528: 4c 8d 0d 31 d0 74 02        	leaq	0x274d031(%rip), %r9    # 0x1429a3560
14025652f: 4c 8d 85 60 03 00 00        	leaq	0x360(%rbp), %r8
140256536: ba 06 00 00 00              	movl	$0x6, %edx
14025653b: e8 20 32 68 02              	callq	0x1428d9760 <.text+0x28d8760>
140256540: eb 0f                       	jmp	0x140256551 <.text+0x255551>
140256542: b9 08 00 00 00              	movl	$0x8, %ecx
140256547: ba 30 00 00 00              	movl	$0x30, %edx
14025654c: e8 af 2a 68 02              	callq	0x1428d9000 <.text+0x28d8000>
140256551: 0f 0b                       	ud2
140256553: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140256560: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140256565: 55                          	pushq	%rbp
140256566: 41 57                       	pushq	%r15
140256568: 41 56                       	pushq	%r14
14025656a: 56                          	pushq	%rsi
14025656b: 57                          	pushq	%rdi
14025656c: 53                          	pushq	%rbx
14025656d: 48 83 ec 38                 	subq	$0x38, %rsp
140256571: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140256578: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14025657d: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140256584: 48 8d 8d f0 02 00 00        	leaq	0x2f0(%rbp), %rcx
14025658b: e8 f0 62 38 00              	callq	0x1405dc880 <.text+0x5db880>
140256590: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140256597: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14025659c: 48 83 c4 38                 	addq	$0x38, %rsp
1402565a0: 5b                          	popq	%rbx
1402565a1: 5f                          	popq	%rdi
1402565a2: 5e                          	popq	%rsi
1402565a3: 41 5e                       	popq	%r14
1402565a5: 41 5f                       	popq	%r15
1402565a7: 5d                          	popq	%rbp
1402565a8: c3                          	retq
1402565a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1402565b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1402565b5: 55                          	pushq	%rbp
1402565b6: 41 57                       	pushq	%r15
1402565b8: 41 56                       	pushq	%r14
1402565ba: 56                          	pushq	%rsi
1402565bb: 57                          	pushq	%rdi
1402565bc: 53                          	pushq	%rbx
1402565bd: 48 83 ec 38                 	subq	$0x38, %rsp
1402565c1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1402565c8: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1402565cd: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
1402565d4: 48 8b 50 08                 	movq	0x8(%rax), %rdx
1402565d8: 48 85 d2                    	testq	%rdx, %rdx
1402565db: 74 17                       	je	0x1402565f4 <.text+0x2555f4>
1402565dd: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
1402565e4: 4c 8b 40 10                 	movq	0x10(%rax), %r8
1402565e8: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
1402565ef: e8 2c d6 3b 00              	callq	0x140613c20 <.text+0x612c20>
1402565f4: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
1402565fb: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140256600: 48 83 c4 38                 	addq	$0x38, %rsp
140256604: 5b                          	popq	%rbx
140256605: 5f                          	popq	%rdi
140256606: 5e                          	popq	%rsi
140256607: 41 5e                       	popq	%r14
140256609: 41 5f                       	popq	%r15
14025660b: 5d                          	popq	%rbp
14025660c: c3                          	retq
14025660d: 0f 1f 00                    	nopl	(%rax)
140256610: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140256615: 55                          	pushq	%rbp
140256616: 41 57                       	pushq	%r15
140256618: 41 56                       	pushq	%r14
14025661a: 56                          	pushq	%rsi
14025661b: 57                          	pushq	%rdi
14025661c: 53                          	pushq	%rbx
14025661d: 48 83 ec 38                 	subq	$0x38, %rsp
140256621: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140256628: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14025662d: 83 bd 60 03 00 00 01        	cmpl	$0x1, 0x360(%rbp)
140256634: 75 13                       	jne	0x140256649 <.text+0x255649>
140256636: 48 8d 8d 78 03 00 00        	leaq	0x378(%rbp), %rcx
14025663d: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140256644: e8 b7 13 39 00              	callq	0x1405e7a00 <.text+0x5e6a00>
140256649: c6 85 97 04 00 00 01        	movb	$0x1, 0x497(%rbp)
140256650: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140256655: 48 83 c4 38                 	addq	$0x38, %rsp
140256659: 5b                          	popq	%rbx
14025665a: 5f                          	popq	%rdi
14025665b: 5e                          	popq	%rsi
14025665c: 41 5e                       	popq	%r14
14025665e: 41 5f                       	popq	%r15
140256660: 5d                          	popq	%rbp
140256661: c3                          	retq
140256662: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140256670: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140256675: 55                          	pushq	%rbp
140256676: 41 57                       	pushq	%r15
140256678: 41 56                       	pushq	%r14
14025667a: 56                          	pushq	%rsi
14025667b: 57                          	pushq	%rdi
14025667c: 53                          	pushq	%rbx
14025667d: 48 83 ec 38                 	subq	$0x38, %rsp
140256681: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140256688: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14025668d: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
140256694: 48 8d 8d c0 01 00 00        	leaq	0x1c0(%rbp), %rcx
14025669b: e8 20 84 39 00              	callq	0x1405eeac0 <.text+0x5edac0>
1402566a0: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
1402566a7: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1402566ac: 48 83 c4 38                 	addq	$0x38, %rsp
1402566b0: 5b                          	popq	%rbx
1402566b1: 5f                          	popq	%rdi
1402566b2: 5e                          	popq	%rsi
1402566b3: 41 5e                       	popq	%r14
1402566b5: 41 5f                       	popq	%r15
1402566b7: 5d                          	popq	%rbp
1402566b8: c3                          	retq
1402566b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1402566c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1402566c5: 55                          	pushq	%rbp
1402566c6: 41 57                       	pushq	%r15
1402566c8: 41 56                       	pushq	%r14
1402566ca: 56                          	pushq	%rsi
1402566cb: 57                          	pushq	%rdi
1402566cc: 53                          	pushq	%rbx
1402566cd: 48 83 ec 38                 	subq	$0x38, %rsp
1402566d1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1402566d8: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1402566dd: 0f b6 9d 96 04 00 00        	movzbl	0x496(%rbp), %ebx
1402566e4: 48 8d 8d e8 00 00 00        	leaq	0xe8(%rbp), %rcx
1402566eb: e8 e0 30 3a 00              	callq	0x1405f97d0 <.text+0x5f87d0>
1402566f0: 88 9d 97 04 00 00           	movb	%bl, 0x497(%rbp)
1402566f6: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1402566fb: 48 83 c4 38                 	addq	$0x38, %rsp
1402566ff: 5b                          	popq	%rbx
140256700: 5f                          	popq	%rdi
140256701: 5e                          	popq	%rsi
140256702: 41 5e                       	popq	%r14
140256704: 41 5f                       	popq	%r15
140256706: 5d                          	popq	%rbp
140256707: c3                          	retq
140256708: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140256710: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140256715: 55                          	pushq	%rbp
140256716: 41 57                       	pushq	%r15
140256718: 41 56                       	pushq	%r14
14025671a: 56                          	pushq	%rsi
14025671b: 57                          	pushq	%rdi
14025671c: 53                          	pushq	%rbx
14025671d: 48 83 ec 38                 	subq	$0x38, %rsp
140256721: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140256728: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14025672d: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
140256734: 48 8d 8d 60 03 00 00        	leaq	0x360(%rbp), %rcx
14025673b: e8 d0 54 39 00              	callq	0x1405ebc10 <.text+0x5eac10>
140256740: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
140256747: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14025674c: 48 83 c4 38                 	addq	$0x38, %rsp
140256750: 5b                          	popq	%rbx
140256751: 5f                          	popq	%rdi
140256752: 5e                          	popq	%rsi
140256753: 41 5e                       	popq	%r14
140256755: 41 5f                       	popq	%r15
140256757: 5d                          	popq	%rbp
140256758: c3                          	retq
140256759: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140256760: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140256765: 55                          	pushq	%rbp
140256766: 41 57                       	pushq	%r15
140256768: 41 56                       	pushq	%r14
14025676a: 56                          	pushq	%rsi
14025676b: 57                          	pushq	%rdi
14025676c: 53                          	pushq	%rbx
14025676d: 48 83 ec 38                 	subq	$0x38, %rsp
140256771: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140256778: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
14025677d: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
140256784: 48 8d 8d a0 01 00 00        	leaq	0x1a0(%rbp), %rcx
14025678b: e8 30 c9 de ff              	callq	0x1400430c0 <.text+0x420c0>
140256790: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
140256797: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
14025679c: 48 83 c4 38                 	addq	$0x38, %rsp
1402567a0: 5b                          	popq	%rbx
1402567a1: 5f                          	popq	%rdi
1402567a2: 5e                          	popq	%rsi
1402567a3: 41 5e                       	popq	%r14
1402567a5: 41 5f                       	popq	%r15
1402567a7: 5d                          	popq	%rbp
1402567a8: c3                          	retq
1402567a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1402567b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1402567b5: 55                          	pushq	%rbp
1402567b6: 41 57                       	pushq	%r15
1402567b8: 41 56                       	pushq	%r14
1402567ba: 56                          	pushq	%rsi
1402567bb: 57                          	pushq	%rdi
1402567bc: 53                          	pushq	%rbx
1402567bd: 48 83 ec 38                 	subq	$0x38, %rsp
1402567c1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1402567c8: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
1402567cd: 48 8d 8d 70 03 00 00        	leaq	0x370(%rbp), %rcx
1402567d4: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
1402567db: e8 20 12 39 00              	callq	0x1405e7a00 <.text+0x5e6a00>
1402567e0: c6 85 96 04 00 00 01        	movb	$0x1, 0x496(%rbp)
1402567e7: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
1402567ec: 48 83 c4 38                 	addq	$0x38, %rsp
1402567f0: 5b                          	popq	%rbx
1402567f1: 5f                          	popq	%rdi
1402567f2: 5e                          	popq	%rsi
1402567f3: 41 5e                       	popq	%r14
1402567f5: 41 5f                       	popq	%r15
1402567f7: 5d                          	popq	%rbp
1402567f8: c3                          	retq
1402567f9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
