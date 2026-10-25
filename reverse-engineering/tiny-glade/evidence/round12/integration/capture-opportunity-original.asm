
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14031d140: 55                          	pushq	%rbp
14031d141: 41 57                       	pushq	%r15
14031d143: 41 56                       	pushq	%r14
14031d145: 41 55                       	pushq	%r13
14031d147: 41 54                       	pushq	%r12
14031d149: 56                          	pushq	%rsi
14031d14a: 57                          	pushq	%rdi
14031d14b: 53                          	pushq	%rbx
14031d14c: 48 81 ec e8 06 00 00        	subq	$0x6e8, %rsp            # imm = 0x6E8
14031d153: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14031d15b: 44 0f 29 85 50 06 00 00     	movaps	%xmm8, 0x650(%rbp)
14031d163: 0f 29 bd 40 06 00 00        	movaps	%xmm7, 0x640(%rbp)
14031d16a: 0f 29 b5 30 06 00 00        	movaps	%xmm6, 0x630(%rbp)
14031d171: 48 c7 85 28 06 00 00 fe ff ff ff    	movq	$-0x2, 0x628(%rbp)
14031d17c: 4c 89 8d f0 05 00 00        	movq	%r9, 0x5f0(%rbp)
14031d183: 4c 89 85 98 05 00 00        	movq	%r8, 0x598(%rbp)
14031d18a: 48 89 95 f8 05 00 00        	movq	%rdx, 0x5f8(%rbp)
14031d191: 49 89 ce                    	movq	%rcx, %r14
14031d194: b1 01                       	movb	$0x1, %cl
14031d196: 31 c0                       	xorl	%eax, %eax
14031d198: f0                          	lock
14031d199: 0f b0 0d b0 de 30 03        	cmpxchgb	%cl, 0x330deb0(%rip)    # 0x14362b050
14031d1a0: 0f 85 96 0e 00 00           	jne	0x14031e03c <.text+0x31d03c>
14031d1a6: 48 8d 0d a3 de 30 03        	leaq	0x330dea3(%rip), %rcx   # 0x14362b050
14031d1ad: e8 be 4a 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031d1b2: 48 8b 05 af de 30 03        	movq	0x330deaf(%rip), %rax   # 0x14362b068
14031d1b9: 48 85 c0                    	testq	%rax, %rax
14031d1bc: 0f 84 91 0d 00 00           	je	0x14031df53 <.text+0x31cf53>
14031d1c2: 48 8b 0d 97 de 30 03        	movq	0x330de97(%rip), %rcx   # 0x14362b060
14031d1c9: 4c 69 e0 98 00 00 00        	imulq	$0x98, %rax, %r12
14031d1d0: 48 89 8d 90 05 00 00        	movq	%rcx, 0x590(%rbp)
14031d1d7: 49 01 cc                    	addq	%rcx, %r12
14031d1da: 48 8b 85 98 05 00 00        	movq	0x598(%rbp), %rax
14031d1e1: 83 c0 fe                    	addl	$-0x2, %eax
14031d1e4: 3c 05                       	cmpb	$0x5, %al
14031d1e6: 0f b6 c0                    	movzbl	%al, %eax
14031d1e9: b9 03 00 00 00              	movl	$0x3, %ecx
14031d1ee: 0f 43 c1                    	cmovael	%ecx, %eax
14031d1f1: 89 85 ec 05 00 00           	movl	%eax, 0x5ec(%rbp)
14031d1f7: 48 8b 85 f0 05 00 00        	movq	0x5f0(%rbp), %rax
14031d1fe: 48 83 c0 04                 	addq	$0x4, %rax
14031d202: 48 89 85 88 05 00 00        	movq	%rax, 0x588(%rbp)
14031d209: f3 0f 10 35 cb ce 63 02     	movss	0x263cecb(%rip), %xmm6  # 0x14295a0dc
14031d211: 0f 57 ff                    	xorps	%xmm7, %xmm7
14031d214: f3 44 0f 10 05 c3 ce 63 02  	movss	0x263cec3(%rip), %xmm8  # 0x14295a0e0
14031d21d: 48 8d bd 20 03 00 00        	leaq	0x320(%rbp), %rdi
14031d224: 48 8d 5d c0                 	leaq	-0x40(%rbp), %rbx
14031d228: 4c 89 b5 18 05 00 00        	movq	%r14, 0x518(%rbp)
14031d22f: 4c 89 a5 20 05 00 00        	movq	%r12, 0x520(%rbp)
14031d236: eb 58                       	jmp	0x14031d290 <.text+0x31c290>
14031d238: 48 8b 85 18 06 00 00        	movq	0x618(%rbp), %rax
14031d23f: 48 8b 8d 08 06 00 00        	movq	0x608(%rbp), %rcx
14031d246: 48 89 41 58                 	movq	%rax, 0x58(%rcx)
14031d24a: 48 8b 85 10 06 00 00        	movq	0x610(%rbp), %rax
14031d251: 48 89 41 60                 	movq	%rax, 0x60(%rcx)
14031d255: 48 8b 85 00 06 00 00        	movq	0x600(%rbp), %rax
14031d25c: 48 89 41 68                 	movq	%rax, 0x68(%rcx)
14031d260: 0f 28 85 20 03 00 00        	movaps	0x320(%rbp), %xmm0
14031d267: 0f 28 8d 30 03 00 00        	movaps	0x330(%rbp), %xmm1
14031d26e: 0f 11 41 70                 	movups	%xmm0, 0x70(%rcx)
14031d272: 0f 11 89 80 00 00 00        	movups	%xmm1, 0x80(%rcx)
14031d279: 48 8b 85 40 03 00 00        	movq	0x340(%rbp), %rax
14031d280: 48 89 81 90 00 00 00        	movq	%rax, 0x90(%rcx)
14031d287: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031d290: 4c 8b 8d 90 05 00 00        	movq	0x590(%rbp), %r9
14031d297: 4d 39 e1                    	cmpq	%r12, %r9
14031d29a: 0f 84 67 0c 00 00           	je	0x14031df07 <.text+0x31cf07>
14031d2a0: 80 bd ec 05 00 00 03        	cmpb	$0x3, 0x5ec(%rbp)
14031d2a7: 0f 94 c1                    	sete	%cl
14031d2aa: 41 0f b6 51 4c              	movzbl	0x4c(%r9), %edx
14031d2af: 8d 42 fe                    	leal	-0x2(%rdx), %eax
14031d2b2: 3c 05                       	cmpb	$0x5, %al
14031d2b4: 0f b6 c0                    	movzbl	%al, %eax
14031d2b7: 41 b8 03 00 00 00           	movl	$0x3, %r8d
14031d2bd: 41 0f 43 c0                 	cmovael	%r8d, %eax
14031d2c1: 4d 8d 81 98 00 00 00        	leaq	0x98(%r9), %r8
14031d2c8: 4c 89 85 90 05 00 00        	movq	%r8, 0x590(%rbp)
14031d2cf: 3a 95 98 05 00 00           	cmpb	0x598(%rbp), %dl
14031d2d5: 0f 95 c2                    	setne	%dl
14031d2d8: 84 d1                       	testb	%dl, %cl
14031d2da: 75 b4                       	jne	0x14031d290 <.text+0x31c290>
14031d2dc: 3a 85 ec 05 00 00           	cmpb	0x5ec(%rbp), %al
14031d2e2: 75 ac                       	jne	0x14031d290 <.text+0x31c290>
14031d2e4: 41 80 b9 96 00 00 00 06     	cmpb	$0x6, 0x96(%r9)
14031d2ec: 75 a2                       	jne	0x14031d290 <.text+0x31c290>
14031d2ee: 4c 89 8d 08 06 00 00        	movq	%r9, 0x608(%rbp)
14031d2f5: 48 8b 8d 88 05 00 00        	movq	0x588(%rbp), %rcx
14031d2fc: e8 8f 84 8b 00              	callq	0x140bd5790 <.text+0xbd4790>
14031d301: 48 89 c2                    	movq	%rax, %rdx
14031d304: 49 89 c7                    	movq	%rax, %r15
14031d307: 49 c1 ef 20                 	shrq	$0x20, %r15
14031d30b: 48 8b 8d 08 06 00 00        	movq	0x608(%rbp), %rcx
14031d312: f6 41 40 01                 	testb	$0x1, 0x40(%rcx)
14031d316: 74 08                       	je	0x14031d320 <.text+0x31c320>
14031d318: 8b 51 44                    	movl	0x44(%rcx), %edx
14031d31b: 8b 41 48                    	movl	0x48(%rcx), %eax
14031d31e: eb 03                       	jmp	0x14031d323 <.text+0x31c323>
14031d320: 44 89 f8                    	movl	%r15d, %eax
14031d323: 48 89 85 10 06 00 00        	movq	%rax, 0x610(%rbp)
14031d32a: 89 d6                       	movl	%edx, %esi
14031d32c: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
14031d336: 48 39 41 18                 	cmpq	%rax, 0x18(%rcx)
14031d33a: 0f 85 ab 05 00 00           	jne	0x14031d8eb <.text+0x31c8eb>
14031d340: 48 89 95 00 06 00 00        	movq	%rdx, 0x600(%rbp)
14031d347: 4d 8b ae 90 01 00 00        	movq	0x190(%r14), %r13
14031d34e: 48 8b 85 10 06 00 00        	movq	0x610(%rbp), %rax
14031d355: 48 c1 e0 20                 	shlq	$0x20, %rax
14031d359: 48 09 c6                    	orq	%rax, %rsi
14031d35c: 48 89 f9                    	movq	%rdi, %rcx
14031d35f: 48 89 b5 18 06 00 00        	movq	%rsi, 0x618(%rbp)
14031d366: 48 89 f2                    	movq	%rsi, %rdx
14031d369: 41 b8 25 00 00 00           	movl	$0x25, %r8d
14031d36f: e8 2c 7f 8b 00              	callq	0x140bd52a0 <.text+0xbd42a0>
14031d374: 48 8d 8d a0 05 00 00        	leaq	0x5a0(%rbp), %rcx
14031d37b: 48 89 fa                    	movq	%rdi, %rdx
14031d37e: e8 0d 80 8b 00              	callq	0x140bd5390 <.text+0xbd4390>
14031d383: 48 89 f9                    	movq	%rdi, %rcx
14031d386: 48 8d 95 a0 05 00 00        	leaq	0x5a0(%rbp), %rdx
14031d38d: e8 de 7f 8b 00              	callq	0x140bd5370 <.text+0xbd4370>
14031d392: 48 c7 85 a0 05 00 00 00 00 00 00    	movq	$0x0, 0x5a0(%rbp)
14031d39d: 48 8d 85 a0 05 00 00        	leaq	0x5a0(%rbp), %rax
14031d3a4: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14031d3a9: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14031d3ae: 41 b9 11 00 00 00           	movl	$0x11, %r9d
14031d3b4: 48 89 d9                    	movq	%rbx, %rcx
14031d3b7: 4c 89 ea                    	movq	%r13, %rdx
14031d3ba: 4c 8d 05 3f f5 69 02        	leaq	0x269f53f(%rip), %r8    # 0x1429bc900
14031d3c1: e8 2a d2 29 00              	callq	0x1405ba5f0 <.text+0x5b95f0>
14031d3c6: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
14031d3ca: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14031d3ce: 0f 10 01                    	movups	(%rcx), %xmm0
14031d3d1: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
14031d3d5: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
14031d3d9: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
14031d3e0: 0f 29 8d 80 01 00 00        	movaps	%xmm1, 0x180(%rbp)
14031d3e7: 0f 29 95 90 01 00 00        	movaps	%xmm2, 0x190(%rbp)
14031d3ee: 48 8b 49 30                 	movq	0x30(%rcx), %rcx
14031d3f2: 48 89 8d a0 01 00 00        	movq	%rcx, 0x1a0(%rbp)
14031d3f9: 48 83 f8 0b                 	cmpq	$0xb, %rax
14031d3fd: 0f 85 af 0b 00 00           	jne	0x14031dfb2 <.text+0x31cfb2>
14031d403: 0f 28 85 70 01 00 00        	movaps	0x170(%rbp), %xmm0
14031d40a: 0f 28 8d 80 01 00 00        	movaps	0x180(%rbp), %xmm1
14031d411: 0f 28 95 90 01 00 00        	movaps	0x190(%rbp), %xmm2
14031d418: 0f 29 85 e0 04 00 00        	movaps	%xmm0, 0x4e0(%rbp)
14031d41f: 0f 29 8d f0 04 00 00        	movaps	%xmm1, 0x4f0(%rbp)
14031d426: 0f 29 95 00 05 00 00        	movaps	%xmm2, 0x500(%rbp)
14031d42d: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
14031d434: 48 89 85 10 05 00 00        	movq	%rax, 0x510(%rbp)
14031d43b: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d442: 48 8d 8d bc 04 00 00        	leaq	0x4bc(%rbp), %rcx
14031d449: 48 8d 95 e0 04 00 00        	leaq	0x4e0(%rbp), %rdx
14031d450: 4d 89 f0                    	movq	%r14, %r8
14031d453: 45 31 c9                    	xorl	%r9d, %r9d
14031d456: e8 45 8c 8b 00              	callq	0x140bd60a0 <.text+0xbd50a0>
14031d45b: 44 89 f8                    	movl	%r15d, %eax
14031d45e: 0f 57 c0                    	xorps	%xmm0, %xmm0
14031d461: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
14031d466: 8b 85 10 06 00 00           	movl	0x610(%rbp), %eax
14031d46c: 0f 57 c9                    	xorps	%xmm1, %xmm1
14031d46f: f3 48 0f 2a c8              	cvtsi2ss	%rax, %xmm1
14031d474: f3 0f 5e c1                 	divss	%xmm1, %xmm0
14031d478: e8 d3 6a 60 02              	callq	0x142923f50 <.text+0x2922f50>
14031d47d: e8 1e 6b 60 02              	callq	0x142923fa0 <.text+0x2922fa0>
14031d482: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
14031d487: 48 89 c1                    	movq	%rax, %rcx
14031d48a: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
14031d48e: 0f 28 c8                    	movaps	%xmm0, %xmm1
14031d491: f3 0f 5c ce                 	subss	%xmm6, %xmm1
14031d495: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
14031d49a: 48 21 ca                    	andq	%rcx, %rdx
14031d49d: 48 09 c2                    	orq	%rax, %rdx
14031d4a0: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
14031d4a3: 41 bd 00 00 00 00           	movl	$0x0, %r13d
14031d4a9: 4c 0f 43 ea                 	cmovaeq	%rdx, %r13
14031d4ad: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
14031d4b1: 48 c7 c0 ff ff ff ff        	movq	$-0x1, %rax
14031d4b8: 4c 0f 47 e8                 	cmovaq	%rax, %r13
14031d4bc: 4d 85 ed                    	testq	%r13, %r13
14031d4bf: 0f 84 83 06 00 00           	je	0x14031db48 <.text+0x31cb48>
14031d4c5: c6 85 72 05 00 00 07        	movb	$0x7, 0x572(%rbp)
14031d4cc: 45 31 e4                    	xorl	%r12d, %r12d
14031d4cf: 90                          	nop
14031d4d0: 40 8a b5 72 05 00 00        	movb	0x572(%rbp), %sil
14031d4d7: 40 80 fe 07                 	cmpb	$0x7, %sil
14031d4db: 4c 8d bd 50 05 00 00        	leaq	0x550(%rbp), %r15
14031d4e2: 4c 0f 44 bd f0 05 00 00     	cmoveq	0x5f0(%rbp), %r15
14031d4ea: 48 8d 85 54 05 00 00        	leaq	0x554(%rbp), %rax
14031d4f1: 48 0f 44 85 88 05 00 00     	cmoveq	0x588(%rbp), %rax
14031d4f9: 0f 10 00                    	movups	(%rax), %xmm0
14031d4fc: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
14031d500: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
14031d507: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
14031d50e: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d515: 48 89 d9                    	movq	%rbx, %rcx
14031d518: 48 89 fa                    	movq	%rdi, %rdx
14031d51b: e8 10 82 8b 00              	callq	0x140bd5730 <.text+0xbd4730>
14031d520: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d527: 48 89 d9                    	movq	%rbx, %rcx
14031d52a: e8 61 82 8b 00              	callq	0x140bd5790 <.text+0xbd4790>
14031d52f: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d536: 48 8d 8d 00 03 00 00        	leaq	0x300(%rbp), %rcx
14031d53d: 48 89 c2                    	movq	%rax, %rdx
14031d540: 41 b8 25 00 00 00           	movl	$0x25, %r8d
14031d546: e8 55 7d 8b 00              	callq	0x140bd52a0 <.text+0xbd42a0>
14031d54b: 40 80 fe 07                 	cmpb	$0x7, %sil
14031d54f: 48 8d 85 50 05 00 00        	leaq	0x550(%rbp), %rax
14031d556: 48 0f 44 85 f0 05 00 00     	cmoveq	0x5f0(%rbp), %rax
14031d55e: 40 8a 70 21                 	movb	0x21(%rax), %sil
14031d562: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
14031d569: 0f 10 8d 10 03 00 00        	movups	0x310(%rbp), %xmm1
14031d570: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
14031d574: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
14031d578: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d57f: 48 8d 8d 30 03 00 00        	leaq	0x330(%rbp), %rcx
14031d586: 48 89 da                    	movq	%rbx, %rdx
14031d589: e8 52 97 8b 00              	callq	0x140bd6ce0 <.text+0xbd5ce0>
14031d58e: 48 8d 05 33 f6 69 02        	leaq	0x269f633(%rip), %rax   # 0x1429bcbc8
14031d595: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
14031d59c: 48 c7 85 28 03 00 00 13 00 00 00    	movq	$0x13, 0x328(%rbp)
14031d5a7: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d5ae: 4c 89 f1                    	movq	%r14, %rcx
14031d5b1: 48 89 fa                    	movq	%rdi, %rdx
14031d5b4: e8 17 bf 8c 00              	callq	0x140be94d0 <.text+0xbe84d0>
14031d5b9: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
14031d5c0: 0f 10 8d 10 03 00 00        	movups	0x310(%rbp), %xmm1
14031d5c7: 48 8d 8d 2c 05 00 00        	leaq	0x52c(%rbp), %rcx
14031d5ce: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
14031d5d2: 0f 11 01                    	movups	%xmm0, (%rcx)
14031d5d5: 89 85 28 05 00 00           	movl	%eax, 0x528(%rbp)
14031d5db: 40 80 fe 01                 	cmpb	$0x1, %sil
14031d5df: 75 4f                       	jne	0x14031d630 <.text+0x31c630>
14031d5e1: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d5e8: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
14031d5ed: 48 8b 85 f8 05 00 00        	movq	0x5f8(%rbp), %rax
14031d5f4: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14031d5f9: 48 c7 44 24 20 18 00 00 00  	movq	$0x18, 0x20(%rsp)
14031d602: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
14031d608: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14031d60f: 48 8d 15 c5 f5 69 02        	leaq	0x269f5c5(%rip), %rdx   # 0x1429bcbdb
14031d616: 4c 8d 0d 93 f5 69 02        	leaq	0x269f593(%rip), %r9    # 0x1429bcbb0
14031d61d: e8 8e a8 26 00              	callq	0x140587eb0 <.text+0x586eb0>
14031d622: 49 b8 dd b3 40 cc 97 56 ba 11       	movabsq	$0x11ba5697cc40b3dd, %r8 # imm = 0x11BA5697CC40B3DD
14031d62c: eb 4d                       	jmp	0x14031d67b <.text+0x31c67b>
14031d62e: 66 90                       	nop
14031d630: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d637: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
14031d63c: 48 8b 85 f8 05 00 00        	movq	0x5f8(%rbp), %rax
14031d643: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14031d648: 48 c7 44 24 20 2d 00 00 00  	movq	$0x2d, 0x20(%rsp)
14031d651: 41 b8 0f 00 00 00           	movl	$0xf, %r8d
14031d657: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14031d65e: 48 8d 15 76 f5 69 02        	leaq	0x269f576(%rip), %rdx   # 0x1429bcbdb
14031d665: 4c 8d 0d 7e f5 69 02        	leaq	0x269f57e(%rip), %r9    # 0x1429bcbea
14031d66c: e8 3f a8 26 00              	callq	0x140587eb0 <.text+0x586eb0>
14031d671: 49 b8 46 ab aa a9 b4 c8 3d d0       	movabsq	$-0x2fc2374b565554ba, %r8 # imm = 0xD03DC8B4A9AAAB46
14031d67b: 4c 89 bd 80 05 00 00        	movq	%r15, 0x580(%rbp)
14031d682: c7 85 a0 05 00 00 02 00 00 00       	movl	$0x2, 0x5a0(%rbp)
14031d68c: 48 8d b5 a0 05 00 00        	leaq	0x5a0(%rbp), %rsi
14031d693: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
14031d698: 4c 8d b5 80 05 00 00        	leaq	0x580(%rbp), %r14
14031d69f: 4c 89 f1                    	movq	%r14, %rcx
14031d6a2: 4c 8d bd 70 01 00 00        	leaq	0x170(%rbp), %r15
14031d6a9: 4c 89 fa                    	movq	%r15, %rdx
14031d6ac: 41 b1 19                    	movb	$0x19, %r9b
14031d6af: e8 4c 7d e3 ff              	callq	0x140155400 <.text+0x154400>
14031d6b4: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031d6ba: 48 89 f9                    	movq	%rdi, %rcx
14031d6bd: 4c 89 fa                    	movq	%r15, %rdx
14031d6c0: e8 3b 65 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031d6c5: 48 8d 85 28 05 00 00        	leaq	0x528(%rbp), %rax
14031d6cc: 48 89 85 80 05 00 00        	movq	%rax, 0x580(%rbp)
14031d6d3: c7 85 a0 05 00 00 02 00 00 00       	movl	$0x2, 0x5a0(%rbp)
14031d6dd: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
14031d6e2: 4c 89 f1                    	movq	%r14, %rcx
14031d6e5: 48 89 fa                    	movq	%rdi, %rdx
14031d6e8: 49 b8 b2 92 98 e6 3a 9a 21 88       	movabsq	$-0x77de65c519676d4e, %r8 # imm = 0x88219A3AE69892B2
14031d6f2: 41 b1 32                    	movb	$0x32, %r9b
14031d6f5: e8 86 86 e3 ff              	callq	0x140155d80 <.text+0x154d80>
14031d6fa: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031d700: 48 89 d9                    	movq	%rbx, %rcx
14031d703: 48 89 fa                    	movq	%rdi, %rdx
14031d706: e8 f5 64 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031d70b: c6 85 24 06 00 00 01        	movb	$0x1, 0x624(%rbp)
14031d712: 48 8d 8d 2c 05 00 00        	leaq	0x52c(%rbp), %rcx
14031d719: e8 72 80 8b 00              	callq	0x140bd5790 <.text+0xbd4790>
14031d71e: 4c 8b b5 18 05 00 00        	movq	0x518(%rbp), %r14
14031d725: c6 85 24 06 00 00 00        	movb	$0x0, 0x624(%rbp)
14031d72c: 48 89 d9                    	movq	%rbx, %rcx
14031d72f: 48 89 c2                    	movq	%rax, %rdx
14031d732: e8 49 15 65 00              	callq	0x14096ec80 <.text+0x96dc80>
14031d737: 49 ff c4                    	incq	%r12
14031d73a: 0f 10 85 28 05 00 00        	movups	0x528(%rbp), %xmm0
14031d741: 0f 10 8d 38 05 00 00        	movups	0x538(%rbp), %xmm1
14031d748: 0f 29 85 50 05 00 00        	movaps	%xmm0, 0x550(%rbp)
14031d74f: 0f 29 8d 60 05 00 00        	movaps	%xmm1, 0x560(%rbp)
14031d756: 8b 85 48 05 00 00           	movl	0x548(%rbp), %eax
14031d75c: 89 85 70 05 00 00           	movl	%eax, 0x570(%rbp)
14031d762: 4d 39 ec                    	cmpq	%r13, %r12
14031d765: 0f 85 65 fd ff ff           	jne	0x14031d4d0 <.text+0x31c4d0>
14031d76b: 0f b6 85 72 05 00 00        	movzbl	0x572(%rbp), %eax
14031d772: 3c 07                       	cmpb	$0x7, %al
14031d774: 0f 84 d9 08 00 00           	je	0x14031e053 <.text+0x31d053>
14031d77a: 0f b6 8d 73 05 00 00        	movzbl	0x573(%rbp), %ecx
14031d781: 0f 28 85 50 05 00 00        	movaps	0x550(%rbp), %xmm0
14031d788: 0f 28 8d 60 05 00 00        	movaps	0x560(%rbp), %xmm1
14031d78f: 0f 29 85 a0 05 00 00        	movaps	%xmm0, 0x5a0(%rbp)
14031d796: 0f 29 8d b0 05 00 00        	movaps	%xmm1, 0x5b0(%rbp)
14031d79d: 0f b7 95 70 05 00 00        	movzwl	0x570(%rbp), %edx
14031d7a4: 66 89 95 c0 05 00 00        	movw	%dx, 0x5c0(%rbp)
14031d7ab: 88 85 c2 05 00 00           	movb	%al, 0x5c2(%rbp)
14031d7b1: 88 8d c3 05 00 00           	movb	%cl, 0x5c3(%rbp)
14031d7b7: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d7be: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
14031d7c3: 48 8b 85 f8 05 00 00        	movq	0x5f8(%rbp), %rax
14031d7ca: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14031d7cf: 48 c7 44 24 20 36 00 00 00  	movq	$0x36, 0x20(%rsp)
14031d7d8: 41 b8 1a 00 00 00           	movl	$0x1a, %r8d
14031d7de: 48 89 f9                    	movq	%rdi, %rcx
14031d7e1: 48 8d 15 77 f1 69 02        	leaq	0x269f177(%rip), %rdx   # 0x1429bc95f
14031d7e8: 4c 8d 0d c0 f1 69 02        	leaq	0x269f1c0(%rip), %r9    # 0x1429bc9af
14031d7ef: e8 bc a6 26 00              	callq	0x140587eb0 <.text+0x586eb0>
14031d7f4: 48 8d 85 a0 05 00 00        	leaq	0x5a0(%rbp), %rax
14031d7fb: 48 89 85 28 05 00 00        	movq	%rax, 0x528(%rbp)
14031d802: c7 45 c0 02 00 00 00        	movl	$0x2, -0x40(%rbp)
14031d809: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14031d80e: 48 8d b5 28 05 00 00        	leaq	0x528(%rbp), %rsi
14031d815: 48 89 f1                    	movq	%rsi, %rcx
14031d818: 48 89 fa                    	movq	%rdi, %rdx
14031d81b: 49 b8 47 59 0f f9 9a e6 5b d1       	movabsq	$-0x2ea4196506f0a6b9, %r8 # imm = 0xD15BE69AF90F5947
14031d825: 41 b1 19                    	movb	$0x19, %r9b
14031d828: e8 d3 7b e3 ff              	callq	0x140155400 <.text+0x154400>
14031d82d: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031d833: 48 89 d9                    	movq	%rbx, %rcx
14031d836: 48 89 fa                    	movq	%rdi, %rdx
14031d839: e8 c2 63 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031d83e: 48 8d 85 bc 04 00 00        	leaq	0x4bc(%rbp), %rax
14031d845: 48 89 85 28 05 00 00        	movq	%rax, 0x528(%rbp)
14031d84c: c7 85 20 03 00 00 02 00 00 00       	movl	$0x2, 0x320(%rbp)
14031d856: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14031d85b: 48 89 f1                    	movq	%rsi, %rcx
14031d85e: 48 89 da                    	movq	%rbx, %rdx
14031d861: 49 b8 ea 8e 3f d7 d9 b4 f8 94       	movabsq	$-0x6b074b2628c07116, %r8 # imm = 0x94F8B4D9D73F8EEA
14031d86b: 41 b1 32                    	movb	$0x32, %r9b
14031d86e: e8 0d 85 e3 ff              	callq	0x140155d80 <.text+0x154d80>
14031d873: 48 8d b5 70 01 00 00        	leaq	0x170(%rbp), %rsi
14031d87a: 4c 8b bd 18 06 00 00        	movq	0x618(%rbp), %r15
14031d881: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031d887: 48 89 f1                    	movq	%rsi, %rcx
14031d88a: 48 89 da                    	movq	%rbx, %rdx
14031d88d: e8 6e 63 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031d892: 80 bd a6 02 00 00 08        	cmpb	$0x8, 0x2a6(%rbp)
14031d899: 74 28                       	je	0x14031d8c3 <.text+0x31c8c3>
14031d89b: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
14031d8a2: b8 e8 00 00 00              	movl	$0xe8, %eax
14031d8a7: 48 01 c2                    	addq	%rax, %rdx
14031d8aa: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
14031d8b1: 49 b8 1d c4 23 f4 18 28 50 ca       	movabsq	$-0x35afd7e70bdc3be3, %r8 # imm = 0xCA502818F423C41D
14031d8bb: 4d 89 f9                    	movq	%r15, %r9
14031d8be: e8 9d 1d 17 00              	callq	0x14048f660 <.text+0x48e660>
14031d8c3: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031d8c9: 48 89 f9                    	movq	%rdi, %rcx
14031d8cc: 48 89 f2                    	movq	%rsi, %rdx
14031d8cf: e8 2c 63 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031d8d4: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031d8db: 48 89 f9                    	movq	%rdi, %rcx
14031d8de: 4c 89 fa                    	movq	%r15, %rdx
14031d8e1: e8 9a 13 65 00              	callq	0x14096ec80 <.text+0x96dc80>
14031d8e6: e9 8c 03 00 00              	jmp	0x14031dc77 <.text+0x31cc77>
14031d8eb: 0f af 95 10 06 00 00        	imull	0x610(%rbp), %edx
14031d8f2: 48 c1 e2 02                 	shlq	$0x2, %rdx
14031d8f6: 4d 8b ae 90 01 00 00        	movq	0x190(%r14), %r13
14031d8fd: 48 89 f9                    	movq	%rdi, %rcx
14031d900: 41 b8 20 00 00 00           	movl	$0x20, %r8d
14031d906: e8 d5 75 8b 00              	callq	0x140bd4ee0 <.text+0xbd3ee0>
14031d90b: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14031d910: 41 b9 11 00 00 00           	movl	$0x11, %r9d
14031d916: 48 89 d9                    	movq	%rbx, %rcx
14031d919: 4c 89 ea                    	movq	%r13, %rdx
14031d91c: 4c 8d 05 dd ef 69 02        	leaq	0x269efdd(%rip), %r8    # 0x1429bc900
14031d923: e8 58 11 8d 00              	callq	0x140beea80 <.text+0xbeda80>
14031d928: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
14031d92c: 4c 8b 6d f0                 	movq	-0x10(%rbp), %r13
14031d930: 48 8d 4d c8                 	leaq	-0x38(%rbp), %rcx
14031d934: 0f 10 01                    	movups	(%rcx), %xmm0
14031d937: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
14031d93b: 0f 29 85 70 01 00 00        	movaps	%xmm0, 0x170(%rbp)
14031d942: 0f 29 8d 80 01 00 00        	movaps	%xmm1, 0x180(%rbp)
14031d949: 48 8b 49 20                 	movq	0x20(%rcx), %rcx
14031d94d: 48 89 8d 90 01 00 00        	movq	%rcx, 0x190(%rbp)
14031d954: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
14031d958: 48 83 f8 0b                 	cmpq	$0xb, %rax
14031d95c: 0f 85 f8 05 00 00           	jne	0x14031df5a <.text+0x31cf5a>
14031d962: 48 89 95 00 06 00 00        	movq	%rdx, 0x600(%rbp)
14031d969: 48 8b 85 90 01 00 00        	movq	0x190(%rbp), %rax
14031d970: 48 89 85 c0 05 00 00        	movq	%rax, 0x5c0(%rbp)
14031d977: 0f 28 85 70 01 00 00        	movaps	0x170(%rbp), %xmm0
14031d97e: 0f 28 8d 80 01 00 00        	movaps	0x180(%rbp), %xmm1
14031d985: 0f 29 8d b0 05 00 00        	movaps	%xmm1, 0x5b0(%rbp)
14031d98c: 0f 29 85 a0 05 00 00        	movaps	%xmm0, 0x5a0(%rbp)
14031d993: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031d99a: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14031d9a1: 4c 8d bd a0 05 00 00        	leaq	0x5a0(%rbp), %r15
14031d9a8: 4c 89 fa                    	movq	%r15, %rdx
14031d9ab: 4d 89 f0                    	movq	%r14, %r8
14031d9ae: 45 31 c9                    	xorl	%r9d, %r9d
14031d9b1: e8 0a 76 8b 00              	callq	0x140bd4fc0 <.text+0xbd3fc0>
14031d9b6: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031d9bd: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
14031d9c2: 48 8b 85 f8 05 00 00        	movq	0x5f8(%rbp), %rax
14031d9c9: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14031d9ce: 48 c7 44 24 20 35 00 00 00  	movq	$0x35, 0x20(%rsp)
14031d9d7: 41 b8 19 00 00 00           	movl	$0x19, %r8d
14031d9dd: 48 89 f9                    	movq	%rdi, %rcx
14031d9e0: 48 8d 15 2a ef 69 02        	leaq	0x269ef2a(%rip), %rdx   # 0x1429bc911
14031d9e7: 4c 8d 0d 3c ef 69 02        	leaq	0x269ef3c(%rip), %r9    # 0x1429bc92a
14031d9ee: e8 bd a4 26 00              	callq	0x140587eb0 <.text+0x586eb0>
14031d9f3: 48 8b 85 f0 05 00 00        	movq	0x5f0(%rbp), %rax
14031d9fa: 48 89 85 e0 04 00 00        	movq	%rax, 0x4e0(%rbp)
14031da01: c7 45 c0 02 00 00 00        	movl	$0x2, -0x40(%rbp)
14031da08: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14031da0d: 48 8d 8d e0 04 00 00        	leaq	0x4e0(%rbp), %rcx
14031da14: 48 89 fa                    	movq	%rdi, %rdx
14031da17: 49 b8 9c dc 57 35 c9 a9 1c 68       	movabsq	$0x681ca9c93557dc9c, %r8 # imm = 0x681CA9C93557DC9C
14031da21: 41 b1 19                    	movb	$0x19, %r9b
14031da24: e8 d7 79 e3 ff              	callq	0x140155400 <.text+0x154400>
14031da29: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031da2f: 48 89 d9                    	movq	%rbx, %rcx
14031da32: 48 89 fa                    	movq	%rdi, %rdx
14031da35: e8 c6 61 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031da3a: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031da41: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14031da48: 48 89 da                    	movq	%rbx, %rdx
14031da4b: 49 b8 0e c3 ec 1c da 04 68 0f       	movabsq	$0xf6804da1cecc30e, %r8 # imm = 0xF6804DA1CECC30E
14031da55: 4d 89 f9                    	movq	%r15, %r9
14031da58: e8 43 08 65 00              	callq	0x14096e2a0 <.text+0x96d2a0>
14031da5d: 48 8b 85 10 06 00 00        	movq	0x610(%rbp), %rax
14031da64: 48 c1 e0 20                 	shlq	$0x20, %rax
14031da68: 48 09 c6                    	orq	%rax, %rsi
14031da6b: 80 bd a6 02 00 00 08        	cmpb	$0x8, 0x2a6(%rbp)
14031da72: 74 28                       	je	0x14031da9c <.text+0x31ca9c>
14031da74: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
14031da7b: b8 e8 00 00 00              	movl	$0xe8, %eax
14031da80: 48 01 c2                    	addq	%rax, %rdx
14031da83: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
14031da8a: 49 b8 1d c4 23 f4 18 28 50 ca       	movabsq	$-0x35afd7e70bdc3be3, %r8 # imm = 0xCA502818F423C41D
14031da94: 49 89 f1                    	movq	%rsi, %r9
14031da97: e8 c4 1b 17 00              	callq	0x14048f660 <.text+0x48e660>
14031da9c: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031daa2: 48 89 f9                    	movq	%rdi, %rcx
14031daa5: 48 8d 95 70 01 00 00        	leaq	0x170(%rbp), %rdx
14031daac: e8 4f 61 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031dab1: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031dab8: 48 89 f9                    	movq	%rdi, %rcx
14031dabb: 48 89 b5 18 06 00 00        	movq	%rsi, 0x618(%rbp)
14031dac2: 48 89 f2                    	movq	%rsi, %rdx
14031dac5: e8 b6 11 65 00              	callq	0x14096ec80 <.text+0x96dc80>
14031daca: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031dad1: 48 8d 8d 58 01 00 00        	leaq	0x158(%rbp), %rcx
14031dad8: 4c 89 f2                    	movq	%r14, %rdx
14031dadb: 41 b0 37                    	movb	$0x37, %r8b
14031dade: e8 3d 76 8b 00              	callq	0x140bd5120 <.text+0xbd4120>
14031dae3: 4c 89 ad 10 06 00 00        	movq	%r13, 0x610(%rbp)
14031daea: 48 8b 85 c0 05 00 00        	movq	0x5c0(%rbp), %rax
14031daf1: 48 89 85 40 03 00 00        	movq	%rax, 0x340(%rbp)
14031daf8: 0f 28 85 a0 05 00 00        	movaps	0x5a0(%rbp), %xmm0
14031daff: 0f 28 8d b0 05 00 00        	movaps	0x5b0(%rbp), %xmm1
14031db06: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
14031db0d: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
14031db14: 48 8b 85 08 06 00 00        	movq	0x608(%rbp), %rax
14031db1b: 0f b6 80 96 00 00 00        	movzbl	0x96(%rax), %eax
14031db22: 04 fb                       	addb	$-0x5, %al
14031db24: 3c 02                       	cmpb	$0x2, %al
14031db26: 0f 82 0c f7 ff ff           	jb	0x14031d238 <.text+0x31c238>
14031db2c: 48 8b 85 08 06 00 00        	movq	0x608(%rbp), %rax
14031db33: 48 8d 48 70                 	leaq	0x70(%rax), %rcx
14031db37: 48 89 8d e0 05 00 00        	movq	%rcx, 0x5e0(%rbp)
14031db3e: e8 6d 49 8c 00              	callq	0x140be24b0 <.text+0xbe14b0>
14031db43: e9 f0 f6 ff ff              	jmp	0x14031d238 <.text+0x31c238>
14031db48: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031db4f: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
14031db54: 48 8b 85 f8 05 00 00        	movq	0x5f8(%rbp), %rax
14031db5b: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
14031db60: 48 c7 44 24 20 36 00 00 00  	movq	$0x36, 0x20(%rsp)
14031db69: 41 b8 1a 00 00 00           	movl	$0x1a, %r8d
14031db6f: 48 89 f9                    	movq	%rdi, %rcx
14031db72: 48 8d 15 e6 ed 69 02        	leaq	0x269ede6(%rip), %rdx   # 0x1429bc95f
14031db79: 4c 8d 0d f9 ed 69 02        	leaq	0x269edf9(%rip), %r9    # 0x1429bc979
14031db80: e8 2b a3 26 00              	callq	0x140587eb0 <.text+0x586eb0>
14031db85: 48 8b 85 f0 05 00 00        	movq	0x5f0(%rbp), %rax
14031db8c: 48 89 85 a0 05 00 00        	movq	%rax, 0x5a0(%rbp)
14031db93: c7 45 c0 02 00 00 00        	movl	$0x2, -0x40(%rbp)
14031db9a: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
14031db9f: 48 8d b5 a0 05 00 00        	leaq	0x5a0(%rbp), %rsi
14031dba6: 48 89 f1                    	movq	%rsi, %rcx
14031dba9: 48 89 fa                    	movq	%rdi, %rdx
14031dbac: 49 b8 9c dc 57 35 c9 a9 1c 68       	movabsq	$0x681ca9c93557dc9c, %r8 # imm = 0x681CA9C93557DC9C
14031dbb6: 41 b1 19                    	movb	$0x19, %r9b
14031dbb9: e8 42 78 e3 ff              	callq	0x140155400 <.text+0x154400>
14031dbbe: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031dbc4: 48 89 d9                    	movq	%rbx, %rcx
14031dbc7: 48 89 fa                    	movq	%rdi, %rdx
14031dbca: e8 31 60 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031dbcf: 48 8d 85 bc 04 00 00        	leaq	0x4bc(%rbp), %rax
14031dbd6: 48 89 85 a0 05 00 00        	movq	%rax, 0x5a0(%rbp)
14031dbdd: c7 85 20 03 00 00 02 00 00 00       	movl	$0x2, 0x320(%rbp)
14031dbe7: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
14031dbec: 48 89 f1                    	movq	%rsi, %rcx
14031dbef: 48 89 da                    	movq	%rbx, %rdx
14031dbf2: 49 b8 ea 8e 3f d7 d9 b4 f8 94       	movabsq	$-0x6b074b2628c07116, %r8 # imm = 0x94F8B4D9D73F8EEA
14031dbfc: 41 b1 32                    	movb	$0x32, %r9b
14031dbff: e8 7c 81 e3 ff              	callq	0x140155d80 <.text+0x154d80>
14031dc04: 4c 8b bd 18 06 00 00        	movq	0x618(%rbp), %r15
14031dc0b: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031dc11: 48 8d b5 70 01 00 00        	leaq	0x170(%rbp), %rsi
14031dc18: 48 89 f1                    	movq	%rsi, %rcx
14031dc1b: 48 89 da                    	movq	%rbx, %rdx
14031dc1e: e8 dd 5f 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031dc23: 80 bd a6 02 00 00 08        	cmpb	$0x8, 0x2a6(%rbp)
14031dc2a: 74 28                       	je	0x14031dc54 <.text+0x31cc54>
14031dc2c: 48 8b 95 a8 02 00 00        	movq	0x2a8(%rbp), %rdx
14031dc33: b8 e8 00 00 00              	movl	$0xe8, %eax
14031dc38: 48 01 c2                    	addq	%rax, %rdx
14031dc3b: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
14031dc42: 49 b8 1d c4 23 f4 18 28 50 ca       	movabsq	$-0x35afd7e70bdc3be3, %r8 # imm = 0xCA502818F423C41D
14031dc4c: 4d 89 f9                    	movq	%r15, %r9
14031dc4f: e8 0c 1a 17 00              	callq	0x14048f660 <.text+0x48e660>
14031dc54: 41 b8 90 01 00 00           	movl	$0x190, %r8d            # imm = 0x190
14031dc5a: 48 89 f9                    	movq	%rdi, %rcx
14031dc5d: 48 89 f2                    	movq	%rsi, %rdx
14031dc60: e8 9b 5f 60 02              	callq	0x142923c00 <.text+0x2922c00>
14031dc65: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031dc6c: 48 89 f9                    	movq	%rdi, %rcx
14031dc6f: 4c 89 fa                    	movq	%r15, %rdx
14031dc72: e8 09 10 65 00              	callq	0x14096ec80 <.text+0x96dc80>
14031dc77: 49 8b 8e a0 00 00 00        	movq	0xa0(%r14), %rcx
14031dc7e: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031dc85: e8 b6 6e 8b 00              	callq	0x140bd4b40 <.text+0xbd3b40>
14031dc8a: 4c 89 75 00                 	movq	%r14, (%rbp)
14031dc8e: 89 45 08                    	movl	%eax, 0x8(%rbp)
14031dc91: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
14031dc99: 48 c7 45 c8 04 00 00 00     	movq	$0x4, -0x38(%rbp)
14031dca1: 48 c7 45 d0 00 00 00 00     	movq	$0x0, -0x30(%rbp)
14031dca9: 48 8d 0d 35 ed 69 02        	leaq	0x269ed35(%rip), %rcx   # 0x1429bc9e5
14031dcb0: 48 89 4d d8                 	movq	%rcx, -0x28(%rbp)
14031dcb4: 48 c7 45 e0 0d 00 00 00     	movq	$0xd, -0x20(%rbp)
14031dcbc: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
14031dcc4: 89 45 f8                    	movl	%eax, -0x8(%rbp)
14031dcc7: c6 85 25 06 00 00 01        	movb	$0x1, 0x625(%rbp)
14031dcce: b1 22                       	movb	$0x22, %cl
14031dcd0: e8 ab de d1 ff              	callq	0x14003bb80 <.text+0x3ab80>
14031dcd5: 8b 95 bc 04 00 00           	movl	0x4bc(%rbp), %edx
14031dcdb: c6 85 25 06 00 00 01        	movb	$0x1, 0x625(%rbp)
14031dce2: 48 89 d9                    	movq	%rbx, %rcx
14031dce5: 41 b0 22                    	movb	$0x22, %r8b
14031dce8: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031dcee: e8 1d 5e 21 00              	callq	0x140533b10 <.text+0x532b10>
14031dcf3: 0f 10 45 00                 	movups	(%rbp), %xmm0
14031dcf7: 0f 29 85 60 03 00 00        	movaps	%xmm0, 0x360(%rbp)
14031dcfe: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
14031dd02: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
14031dd06: 0f 10 55 e0                 	movups	-0x20(%rbp), %xmm2
14031dd0a: 0f 10 5d f0                 	movups	-0x10(%rbp), %xmm3
14031dd0e: 0f 29 9d 50 03 00 00        	movaps	%xmm3, 0x350(%rbp)
14031dd15: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
14031dd1c: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
14031dd23: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
14031dd2a: c6 85 25 06 00 00 00        	movb	$0x0, 0x625(%rbp)
14031dd31: 48 89 f9                    	movq	%rdi, %rcx
14031dd34: e8 57 60 21 00              	callq	0x140533d90 <.text+0x532d90>
14031dd39: 8b 85 dc 04 00 00           	movl	0x4dc(%rbp), %eax
14031dd3f: 89 85 40 03 00 00           	movl	%eax, 0x340(%rbp)
14031dd45: 0f 10 85 bc 04 00 00        	movups	0x4bc(%rbp), %xmm0
14031dd4c: 0f 10 8d cc 04 00 00        	movups	0x4cc(%rbp), %xmm1
14031dd53: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
14031dd5a: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
14031dd61: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031dd68: 48 89 f9                    	movq	%rdi, %rcx
14031dd6b: 4c 89 f2                    	movq	%r14, %rdx
14031dd6e: 41 b0 22                    	movb	$0x22, %r8b
14031dd71: e8 0a 86 8b 00              	callq	0x140bd6380 <.text+0xbd5380>
14031dd76: 48 8b 85 10 05 00 00        	movq	0x510(%rbp), %rax
14031dd7d: 48 89 85 50 03 00 00        	movq	%rax, 0x350(%rbp)
14031dd84: 0f 28 85 e0 04 00 00        	movaps	0x4e0(%rbp), %xmm0
14031dd8b: 0f 28 8d f0 04 00 00        	movaps	0x4f0(%rbp), %xmm1
14031dd92: 0f 28 95 00 05 00 00        	movaps	0x500(%rbp), %xmm2
14031dd99: 0f 29 95 40 03 00 00        	movaps	%xmm2, 0x340(%rbp)
14031dda0: 0f 29 8d 30 03 00 00        	movaps	%xmm1, 0x330(%rbp)
14031dda7: 0f 29 85 20 03 00 00        	movaps	%xmm0, 0x320(%rbp)
14031ddae: 48 8b 85 08 06 00 00        	movq	0x608(%rbp), %rax
14031ddb5: 48 8b 70 20                 	movq	0x20(%rax), %rsi
14031ddb9: 4c 8d 7e 10                 	leaq	0x10(%rsi), %r15
14031ddbd: 31 c0                       	xorl	%eax, %eax
14031ddbf: b1 01                       	movb	$0x1, %cl
14031ddc1: f0                          	lock
14031ddc2: 0f b0 4e 10                 	cmpxchgb	%cl, 0x10(%rsi)
14031ddc6: 0f 85 12 01 00 00           	jne	0x14031dede <.text+0x31cede>
14031ddcc: 4c 89 f9                    	movq	%r15, %rcx
14031ddcf: e8 9c 3e 96 00              	callq	0x140c81c70 <.text+0xc80c70>
14031ddd4: 4c 89 bd e0 05 00 00        	movq	%r15, 0x5e0(%rbp)
14031dddb: 48 8d 46 18                 	leaq	0x18(%rsi), %rax
14031dddf: 48 89 85 18 06 00 00        	movq	%rax, 0x618(%rbp)
14031dde6: 80 7e 4e 07                 	cmpb	$0x7, 0x4e(%rsi)
14031ddea: 74 18                       	je	0x14031de04 <.text+0x31ce04>
14031ddec: 48 8b 8d 18 06 00 00        	movq	0x618(%rbp), %rcx
14031ddf3: e8 88 dd d1 ff              	callq	0x14003bb80 <.text+0x3ab80>
14031ddf8: 48 8b 8d 18 06 00 00        	movq	0x618(%rbp), %rcx
14031ddff: e8 ac 46 8c 00              	callq	0x140be24b0 <.text+0xbe14b0>
14031de04: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14031de0b: 48 8b 8d 18 06 00 00        	movq	0x618(%rbp), %rcx
14031de12: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
14031de16: 0f 28 85 20 03 00 00        	movaps	0x320(%rbp), %xmm0
14031de1d: 0f 28 8d 30 03 00 00        	movaps	0x330(%rbp), %xmm1
14031de24: 0f 28 95 40 03 00 00        	movaps	0x340(%rbp), %xmm2
14031de2b: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
14031de2f: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
14031de33: 0f 11 01                    	movups	%xmm0, (%rcx)
14031de36: c6 85 27 06 00 00 00        	movb	$0x0, 0x627(%rbp)
14031de3d: 48 8b b5 e0 05 00 00        	movq	0x5e0(%rbp), %rsi
14031de44: 48 89 f1                    	movq	%rsi, %rcx
14031de47: e8 c4 3f 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031de4c: b0 01                       	movb	$0x1, %al
14031de4e: 31 c9                       	xorl	%ecx, %ecx
14031de50: f0                          	lock
14031de51: 0f b0 0e                    	cmpxchgb	%cl, (%rsi)
14031de54: 0f 85 97 00 00 00           	jne	0x14031def1 <.text+0x31cef1>
14031de5a: 48 8b 85 08 06 00 00        	movq	0x608(%rbp), %rax
14031de61: 0f b6 80 96 00 00 00        	movzbl	0x96(%rax), %eax
14031de68: 04 fb                       	addb	$-0x5, %al
14031de6a: 3c 02                       	cmpb	$0x2, %al
14031de6c: 72 10                       	jb	0x14031de7e <.text+0x31ce7e>
14031de6e: 48 8b 85 08 06 00 00        	movq	0x608(%rbp), %rax
14031de75: 48 8d 48 70                 	leaq	0x70(%rax), %rcx
14031de79: e8 32 46 8c 00              	callq	0x140be24b0 <.text+0xbe14b0>
14031de7e: 48 8b 85 00 06 00 00        	movq	0x600(%rbp), %rax
14031de85: 48 8b 8d 08 06 00 00        	movq	0x608(%rbp), %rcx
14031de8c: 89 41 58                    	movl	%eax, 0x58(%rcx)
14031de8f: 48 8b 85 10 06 00 00        	movq	0x610(%rbp), %rax
14031de96: 89 41 5c                    	movl	%eax, 0x5c(%rcx)
14031de99: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
14031dea0: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
14031dea7: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
14031deae: 0f 11 41 60                 	movups	%xmm0, 0x60(%rcx)
14031deb2: 0f 11 49 70                 	movups	%xmm1, 0x70(%rcx)
14031deb6: 0f 11 91 80 00 00 00        	movups	%xmm2, 0x80(%rcx)
14031debd: 48 8b 85 4e 03 00 00        	movq	0x34e(%rbp), %rax
14031dec4: 48 89 81 8e 00 00 00        	movq	%rax, 0x8e(%rcx)
14031decb: c6 81 96 00 00 00 05        	movb	$0x5, 0x96(%rcx)
14031ded2: 4c 8b a5 20 05 00 00        	movq	0x520(%rbp), %r12
14031ded9: e9 b2 f3 ff ff              	jmp	0x14031d290 <.text+0x31c290>
14031dede: 4c 89 f9                    	movq	%r15, %rcx
14031dee1: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14031dee7: e8 24 f3 5a 02              	callq	0x1428cd210 <.text+0x28cc210>
14031deec: e9 db fe ff ff              	jmp	0x14031ddcc <.text+0x31cdcc>
14031def1: c6 85 27 06 00 00 00        	movb	$0x0, 0x627(%rbp)
14031def8: 48 89 f1                    	movq	%rsi, %rcx
14031defb: 31 d2                       	xorl	%edx, %edx
14031defd: e8 8e fc 5a 02              	callq	0x1428cdb90 <.text+0x28ccb90>
14031df02: e9 53 ff ff ff              	jmp	0x14031de5a <.text+0x31ce5a>
14031df07: 48 8d 0d 42 d1 30 03        	leaq	0x330d142(%rip), %rcx   # 0x14362b050
14031df0e: e8 fd 3e 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031df13: 31 f6                       	xorl	%esi, %esi
14031df15: b0 01                       	movb	$0x1, %al
14031df17: f0                          	lock
14031df18: 40 0f b0 35 30 d1 30 03     	cmpxchgb	%sil, 0x330d130(%rip)   # 0x14362b050
14031df20: 0f 85 03 01 00 00           	jne	0x14031e029 <.text+0x31d029>
14031df26: 48 89 f0                    	movq	%rsi, %rax
14031df29: 0f 28 b5 30 06 00 00        	movaps	0x630(%rbp), %xmm6
14031df30: 0f 28 bd 40 06 00 00        	movaps	0x640(%rbp), %xmm7
14031df37: 44 0f 28 85 50 06 00 00     	movaps	0x650(%rbp), %xmm8
14031df3f: 48 81 c4 e8 06 00 00        	addq	$0x6e8, %rsp            # imm = 0x6E8
14031df46: 5b                          	popq	%rbx
14031df47: 5f                          	popq	%rdi
14031df48: 5e                          	popq	%rsi
14031df49: 41 5c                       	popq	%r12
14031df4b: 41 5d                       	popq	%r13
14031df4d: 41 5e                       	popq	%r14
14031df4f: 41 5f                       	popq	%r15
14031df51: 5d                          	popq	%rbp
14031df52: c3                          	retq
14031df53: 31 f6                       	xorl	%esi, %esi
14031df55: e9 b1 00 00 00              	jmp	0x14031e00b <.text+0x31d00b>
14031df5a: 0f 10 45 00                 	movups	(%rbp), %xmm0
14031df5e: 0f 11 85 60 03 00 00        	movups	%xmm0, 0x360(%rbp)
14031df65: 0f 28 85 70 01 00 00        	movaps	0x170(%rbp), %xmm0
14031df6c: 0f 28 8d 80 01 00 00        	movaps	0x180(%rbp), %xmm1
14031df73: 0f 11 85 28 03 00 00        	movups	%xmm0, 0x328(%rbp)
14031df7a: 0f 11 8d 38 03 00 00        	movups	%xmm1, 0x338(%rbp)
14031df81: 48 8b 8d 90 01 00 00        	movq	0x190(%rbp), %rcx
14031df88: 48 89 8d 48 03 00 00        	movq	%rcx, 0x348(%rbp)
14031df8f: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
14031df96: 4c 89 ad 50 03 00 00        	movq	%r13, 0x350(%rbp)
14031df9d: 48 89 95 58 03 00 00        	movq	%rdx, 0x358(%rbp)
14031dfa4: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14031dfab: e8 10 61 58 02              	callq	0x1428a40c0 <.text+0x28a30c0>
14031dfb0: eb 56                       	jmp	0x14031e008 <.text+0x31d008>
14031dfb2: 0f 10 45 00                 	movups	(%rbp), %xmm0
14031dfb6: 0f 11 85 60 03 00 00        	movups	%xmm0, 0x360(%rbp)
14031dfbd: 0f 28 85 70 01 00 00        	movaps	0x170(%rbp), %xmm0
14031dfc4: 0f 28 8d 80 01 00 00        	movaps	0x180(%rbp), %xmm1
14031dfcb: 0f 28 95 90 01 00 00        	movaps	0x190(%rbp), %xmm2
14031dfd2: 0f 11 85 28 03 00 00        	movups	%xmm0, 0x328(%rbp)
14031dfd9: 0f 11 8d 38 03 00 00        	movups	%xmm1, 0x338(%rbp)
14031dfe0: 0f 11 95 48 03 00 00        	movups	%xmm2, 0x348(%rbp)
14031dfe7: 48 8b 8d a0 01 00 00        	movq	0x1a0(%rbp), %rcx
14031dfee: 48 89 8d 58 03 00 00        	movq	%rcx, 0x358(%rbp)
14031dff5: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
14031dffc: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14031e003: e8 b8 60 58 02              	callq	0x1428a40c0 <.text+0x28a30c0>
14031e008: 48 89 c6                    	movq	%rax, %rsi
14031e00b: 48 8d 0d 3e d0 30 03        	leaq	0x330d03e(%rip), %rcx   # 0x14362b050
14031e012: e8 f9 3d 96 00              	callq	0x140c81e10 <.text+0xc80e10>
14031e017: 31 c9                       	xorl	%ecx, %ecx
14031e019: b0 01                       	movb	$0x1, %al
14031e01b: f0                          	lock
14031e01c: 0f b0 0d 2d d0 30 03        	cmpxchgb	%cl, 0x330d02d(%rip)    # 0x14362b050
14031e023: 0f 84 fd fe ff ff           	je	0x14031df26 <.text+0x31cf26>
14031e029: 48 8d 0d 20 d0 30 03        	leaq	0x330d020(%rip), %rcx   # 0x14362b050
14031e030: 31 d2                       	xorl	%edx, %edx
14031e032: e8 59 fb 5a 02              	callq	0x1428cdb90 <.text+0x28ccb90>
14031e037: e9 ea fe ff ff              	jmp	0x14031df26 <.text+0x31cf26>
14031e03c: 48 8d 0d 0d d0 30 03        	leaq	0x330d00d(%rip), %rcx   # 0x14362b050
14031e043: 41 b8 00 ca 9a 3b           	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14031e049: e8 c2 f1 5a 02              	callq	0x1428cd210 <.text+0x28cc210>
14031e04e: e9 53 f1 ff ff              	jmp	0x14031d1a6 <.text+0x31c1a6>
14031e053: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e05a: 48 8d 0d 37 eb 69 02        	leaq	0x269eb37(%rip), %rcx   # 0x1429bcb98
14031e061: e8 aa b2 5b 02              	callq	0x1428d9310 <.text+0x28d8310>
14031e066: 0f 0b                       	ud2
14031e068: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14031e070: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e075: 55                          	pushq	%rbp
14031e076: 41 57                       	pushq	%r15
14031e078: 41 56                       	pushq	%r14
14031e07a: 41 55                       	pushq	%r13
14031e07c: 41 54                       	pushq	%r12
14031e07e: 56                          	pushq	%rsi
14031e07f: 57                          	pushq	%rdi
14031e080: 53                          	pushq	%rbx
14031e081: 48 83 ec 68                 	subq	$0x68, %rsp
14031e085: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e08c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e092: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e097: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e09c: 80 bd 26 06 00 00 00        	cmpb	$0x0, 0x626(%rbp)
14031e0a3: 74 2d                       	je	0x14031e0d2 <.text+0x31d0d2>
14031e0a5: 48 8d 8d a0 05 00 00        	leaq	0x5a0(%rbp), %rcx
14031e0ac: e8 ff 43 8c 00              	callq	0x140be24b0 <.text+0xbe14b0>
14031e0b1: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e0b6: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e0bb: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e0c1: 48 83 c4 68                 	addq	$0x68, %rsp
14031e0c5: 5b                          	popq	%rbx
14031e0c6: 5f                          	popq	%rdi
14031e0c7: 5e                          	popq	%rsi
14031e0c8: 41 5c                       	popq	%r12
14031e0ca: 41 5d                       	popq	%r13
14031e0cc: 41 5e                       	popq	%r14
14031e0ce: 41 5f                       	popq	%r15
14031e0d0: 5d                          	popq	%rbp
14031e0d1: c3                          	retq
14031e0d2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e0d7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e0dc: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e0e2: 48 83 c4 68                 	addq	$0x68, %rsp
14031e0e6: 5b                          	popq	%rbx
14031e0e7: 5f                          	popq	%rdi
14031e0e8: 5e                          	popq	%rsi
14031e0e9: 41 5c                       	popq	%r12
14031e0eb: 41 5d                       	popq	%r13
14031e0ed: 41 5e                       	popq	%r14
14031e0ef: 41 5f                       	popq	%r15
14031e0f1: 5d                          	popq	%rbp
14031e0f2: c3                          	retq
14031e0f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14031e100: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e105: 55                          	pushq	%rbp
14031e106: 41 57                       	pushq	%r15
14031e108: 41 56                       	pushq	%r14
14031e10a: 41 55                       	pushq	%r13
14031e10c: 41 54                       	pushq	%r12
14031e10e: 56                          	pushq	%rsi
14031e10f: 57                          	pushq	%rdi
14031e110: 53                          	pushq	%rbx
14031e111: 48 83 ec 68                 	subq	$0x68, %rsp
14031e115: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e11c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e122: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e127: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e12c: 80 bd 24 06 00 00 00        	cmpb	$0x0, 0x624(%rbp)
14031e133: 74 10                       	je	0x14031e145 <.text+0x31d145>
14031e135: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e13c: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14031e140: e8 db b3 2e 00              	callq	0x140609520 <.text+0x608520>
14031e145: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e14c: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e151: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e156: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e15c: 48 83 c4 68                 	addq	$0x68, %rsp
14031e160: 5b                          	popq	%rbx
14031e161: 5f                          	popq	%rdi
14031e162: 5e                          	popq	%rsi
14031e163: 41 5c                       	popq	%r12
14031e165: 41 5d                       	popq	%r13
14031e167: 41 5e                       	popq	%r14
14031e169: 41 5f                       	popq	%r15
14031e16b: 5d                          	popq	%rbp
14031e16c: c3                          	retq
14031e16d: 0f 1f 00                    	nopl	(%rax)
14031e170: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e175: 55                          	pushq	%rbp
14031e176: 41 57                       	pushq	%r15
14031e178: 41 56                       	pushq	%r14
14031e17a: 41 55                       	pushq	%r13
14031e17c: 41 54                       	pushq	%r12
14031e17e: 56                          	pushq	%rsi
14031e17f: 57                          	pushq	%rdi
14031e180: 53                          	pushq	%rbx
14031e181: 48 83 ec 68                 	subq	$0x68, %rsp
14031e185: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e18c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e192: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e197: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e19c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e1a3: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14031e1aa: e8 71 b3 2e 00              	callq	0x140609520 <.text+0x608520>
14031e1af: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e1b6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e1bb: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e1c0: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e1c6: 48 83 c4 68                 	addq	$0x68, %rsp
14031e1ca: 5b                          	popq	%rbx
14031e1cb: 5f                          	popq	%rdi
14031e1cc: 5e                          	popq	%rsi
14031e1cd: 41 5c                       	popq	%r12
14031e1cf: 41 5d                       	popq	%r13
14031e1d1: 41 5e                       	popq	%r14
14031e1d3: 41 5f                       	popq	%r15
14031e1d5: 5d                          	popq	%rbp
14031e1d6: c3                          	retq
14031e1d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031e1e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e1e5: 55                          	pushq	%rbp
14031e1e6: 41 57                       	pushq	%r15
14031e1e8: 41 56                       	pushq	%r14
14031e1ea: 41 55                       	pushq	%r13
14031e1ec: 41 54                       	pushq	%r12
14031e1ee: 56                          	pushq	%rsi
14031e1ef: 57                          	pushq	%rdi
14031e1f0: 53                          	pushq	%rbx
14031e1f1: 48 83 ec 68                 	subq	$0x68, %rsp
14031e1f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e1fc: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e202: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e207: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e20c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e213: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14031e217: e8 04 b3 2e 00              	callq	0x140609520 <.text+0x608520>
14031e21c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e223: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e228: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e22d: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e233: 48 83 c4 68                 	addq	$0x68, %rsp
14031e237: 5b                          	popq	%rbx
14031e238: 5f                          	popq	%rdi
14031e239: 5e                          	popq	%rsi
14031e23a: 41 5c                       	popq	%r12
14031e23c: 41 5d                       	popq	%r13
14031e23e: 41 5e                       	popq	%r14
14031e240: 41 5f                       	popq	%r15
14031e242: 5d                          	popq	%rbp
14031e243: c3                          	retq
14031e244: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031e250: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e255: 55                          	pushq	%rbp
14031e256: 41 57                       	pushq	%r15
14031e258: 41 56                       	pushq	%r14
14031e25a: 41 55                       	pushq	%r13
14031e25c: 41 54                       	pushq	%r12
14031e25e: 56                          	pushq	%rsi
14031e25f: 57                          	pushq	%rdi
14031e260: 53                          	pushq	%rbx
14031e261: 48 83 ec 68                 	subq	$0x68, %rsp
14031e265: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e26c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e272: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e277: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e27c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e283: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14031e28a: e8 91 b2 2e 00              	callq	0x140609520 <.text+0x608520>
14031e28f: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e296: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e29b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e2a0: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e2a6: 48 83 c4 68                 	addq	$0x68, %rsp
14031e2aa: 5b                          	popq	%rbx
14031e2ab: 5f                          	popq	%rdi
14031e2ac: 5e                          	popq	%rsi
14031e2ad: 41 5c                       	popq	%r12
14031e2af: 41 5d                       	popq	%r13
14031e2b1: 41 5e                       	popq	%r14
14031e2b3: 41 5f                       	popq	%r15
14031e2b5: 5d                          	popq	%rbp
14031e2b6: c3                          	retq
14031e2b7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031e2c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e2c5: 55                          	pushq	%rbp
14031e2c6: 41 57                       	pushq	%r15
14031e2c8: 41 56                       	pushq	%r14
14031e2ca: 41 55                       	pushq	%r13
14031e2cc: 41 54                       	pushq	%r12
14031e2ce: 56                          	pushq	%rsi
14031e2cf: 57                          	pushq	%rdi
14031e2d0: 53                          	pushq	%rbx
14031e2d1: 48 83 ec 68                 	subq	$0x68, %rsp
14031e2d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e2dc: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e2e2: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e2e7: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e2ec: 80 bd 56 03 00 00 07        	cmpb	$0x7, 0x356(%rbp)
14031e2f3: 74 0c                       	je	0x14031e301 <.text+0x31d301>
14031e2f5: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14031e2fc: e8 df 02 2d 00              	callq	0x1405ee5e0 <.text+0x5ed5e0>
14031e301: c6 85 27 06 00 00 00        	movb	$0x0, 0x627(%rbp)
14031e308: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e30d: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e312: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e318: 48 83 c4 68                 	addq	$0x68, %rsp
14031e31c: 5b                          	popq	%rbx
14031e31d: 5f                          	popq	%rdi
14031e31e: 5e                          	popq	%rsi
14031e31f: 41 5c                       	popq	%r12
14031e321: 41 5d                       	popq	%r13
14031e323: 41 5e                       	popq	%r14
14031e325: 41 5f                       	popq	%r15
14031e327: 5d                          	popq	%rbp
14031e328: c3                          	retq
14031e329: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14031e330: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e335: 55                          	pushq	%rbp
14031e336: 41 57                       	pushq	%r15
14031e338: 41 56                       	pushq	%r14
14031e33a: 41 55                       	pushq	%r13
14031e33c: 41 54                       	pushq	%r12
14031e33e: 56                          	pushq	%rsi
14031e33f: 57                          	pushq	%rdi
14031e340: 53                          	pushq	%rbx
14031e341: 48 83 ec 68                 	subq	$0x68, %rsp
14031e345: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e34c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e352: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e357: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e35c: 48 8b 8d 18 06 00 00        	movq	0x618(%rbp), %rcx
14031e363: e8 48 41 8c 00              	callq	0x140be24b0 <.text+0xbe14b0>
14031e368: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e36d: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e372: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e378: 48 83 c4 68                 	addq	$0x68, %rsp
14031e37c: 5b                          	popq	%rbx
14031e37d: 5f                          	popq	%rdi
14031e37e: 5e                          	popq	%rsi
14031e37f: 41 5c                       	popq	%r12
14031e381: 41 5d                       	popq	%r13
14031e383: 41 5e                       	popq	%r14
14031e385: 41 5f                       	popq	%r15
14031e387: 5d                          	popq	%rbp
14031e388: c3                          	retq
14031e389: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14031e390: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e395: 55                          	pushq	%rbp
14031e396: 41 57                       	pushq	%r15
14031e398: 41 56                       	pushq	%r14
14031e39a: 41 55                       	pushq	%r13
14031e39c: 41 54                       	pushq	%r12
14031e39e: 56                          	pushq	%rsi
14031e39f: 57                          	pushq	%rdi
14031e3a0: 53                          	pushq	%rbx
14031e3a1: 48 83 ec 68                 	subq	$0x68, %rsp
14031e3a5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e3ac: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e3b2: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e3b7: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e3bc: 48 8b 85 50 03 00 00        	movq	0x350(%rbp), %rax
14031e3c3: 48 8b 8d 18 06 00 00        	movq	0x618(%rbp), %rcx
14031e3ca: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
14031e3ce: 0f 28 85 20 03 00 00        	movaps	0x320(%rbp), %xmm0
14031e3d5: 0f 28 8d 30 03 00 00        	movaps	0x330(%rbp), %xmm1
14031e3dc: 0f 28 95 40 03 00 00        	movaps	0x340(%rbp), %xmm2
14031e3e3: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
14031e3e7: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
14031e3eb: 0f 11 01                    	movups	%xmm0, (%rcx)
14031e3ee: 48 8b 8d e0 05 00 00        	movq	0x5e0(%rbp), %rcx
14031e3f5: e8 36 38 e4 ff              	callq	0x140161c30 <.text+0x160c30>
14031e3fa: c6 85 27 06 00 00 00        	movb	$0x0, 0x627(%rbp)
14031e401: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e406: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e40b: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e411: 48 83 c4 68                 	addq	$0x68, %rsp
14031e415: 5b                          	popq	%rbx
14031e416: 5f                          	popq	%rdi
14031e417: 5e                          	popq	%rsi
14031e418: 41 5c                       	popq	%r12
14031e41a: 41 5d                       	popq	%r13
14031e41c: 41 5e                       	popq	%r14
14031e41e: 41 5f                       	popq	%r15
14031e420: 5d                          	popq	%rbp
14031e421: c3                          	retq
14031e422: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14031e430: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e435: 55                          	pushq	%rbp
14031e436: 41 57                       	pushq	%r15
14031e438: 41 56                       	pushq	%r14
14031e43a: 41 55                       	pushq	%r13
14031e43c: 41 54                       	pushq	%r12
14031e43e: 56                          	pushq	%rsi
14031e43f: 57                          	pushq	%rdi
14031e440: 53                          	pushq	%rbx
14031e441: 48 83 ec 68                 	subq	$0x68, %rsp
14031e445: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e44c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e452: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e457: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e45c: 48 8b 85 00 06 00 00        	movq	0x600(%rbp), %rax
14031e463: 48 8b 8d 08 06 00 00        	movq	0x608(%rbp), %rcx
14031e46a: 89 41 58                    	movl	%eax, 0x58(%rcx)
14031e46d: 48 8b 85 10 06 00 00        	movq	0x610(%rbp), %rax
14031e474: 89 41 5c                    	movl	%eax, 0x5c(%rcx)
14031e477: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
14031e47e: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
14031e485: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
14031e48c: 0f 11 41 60                 	movups	%xmm0, 0x60(%rcx)
14031e490: 0f 11 49 70                 	movups	%xmm1, 0x70(%rcx)
14031e494: 0f 11 91 80 00 00 00        	movups	%xmm2, 0x80(%rcx)
14031e49b: 48 8b 85 4e 03 00 00        	movq	0x34e(%rbp), %rax
14031e4a2: 48 89 81 8e 00 00 00        	movq	%rax, 0x8e(%rcx)
14031e4a9: c6 81 96 00 00 00 05        	movb	$0x5, 0x96(%rcx)
14031e4b0: c6 85 27 06 00 00 00        	movb	$0x0, 0x627(%rbp)
14031e4b7: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e4bc: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e4c1: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e4c7: 48 83 c4 68                 	addq	$0x68, %rsp
14031e4cb: 5b                          	popq	%rbx
14031e4cc: 5f                          	popq	%rdi
14031e4cd: 5e                          	popq	%rsi
14031e4ce: 41 5c                       	popq	%r12
14031e4d0: 41 5d                       	popq	%r13
14031e4d2: 41 5e                       	popq	%r14
14031e4d4: 41 5f                       	popq	%r15
14031e4d6: 5d                          	popq	%rbp
14031e4d7: c3                          	retq
14031e4d8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14031e4e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e4e5: 55                          	pushq	%rbp
14031e4e6: 41 57                       	pushq	%r15
14031e4e8: 41 56                       	pushq	%r14
14031e4ea: 41 55                       	pushq	%r13
14031e4ec: 41 54                       	pushq	%r12
14031e4ee: 56                          	pushq	%rsi
14031e4ef: 57                          	pushq	%rdi
14031e4f0: 53                          	pushq	%rbx
14031e4f1: 48 83 ec 68                 	subq	$0x68, %rsp
14031e4f5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e4fc: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e502: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e507: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e50c: 80 bd 25 06 00 00 00        	cmpb	$0x0, 0x625(%rbp)
14031e513: 74 31                       	je	0x14031e546 <.text+0x31d546>
14031e515: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14031e519: e8 e2 b7 2b 00              	callq	0x1405d9d00 <.text+0x5d8d00>
14031e51e: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e525: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e52a: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e52f: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e535: 48 83 c4 68                 	addq	$0x68, %rsp
14031e539: 5b                          	popq	%rbx
14031e53a: 5f                          	popq	%rdi
14031e53b: 5e                          	popq	%rsi
14031e53c: 41 5c                       	popq	%r12
14031e53e: 41 5d                       	popq	%r13
14031e540: 41 5e                       	popq	%r14
14031e542: 41 5f                       	popq	%r15
14031e544: 5d                          	popq	%rbp
14031e545: c3                          	retq
14031e546: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e54d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e552: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e557: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e55d: 48 83 c4 68                 	addq	$0x68, %rsp
14031e561: 5b                          	popq	%rbx
14031e562: 5f                          	popq	%rdi
14031e563: 5e                          	popq	%rsi
14031e564: 41 5c                       	popq	%r12
14031e566: 41 5d                       	popq	%r13
14031e568: 41 5e                       	popq	%r14
14031e56a: 41 5f                       	popq	%r15
14031e56c: 5d                          	popq	%rbp
14031e56d: c3                          	retq
14031e56e: 66 90                       	nop
14031e570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e575: 55                          	pushq	%rbp
14031e576: 41 57                       	pushq	%r15
14031e578: 41 56                       	pushq	%r14
14031e57a: 41 55                       	pushq	%r13
14031e57c: 41 54                       	pushq	%r12
14031e57e: 56                          	pushq	%rsi
14031e57f: 57                          	pushq	%rdi
14031e580: 53                          	pushq	%rbx
14031e581: 48 83 ec 68                 	subq	$0x68, %rsp
14031e585: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e58c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e592: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e597: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e59c: 80 bd 27 06 00 00 00        	cmpb	$0x0, 0x627(%rbp)
14031e5a3: 74 2d                       	je	0x14031e5d2 <.text+0x31d5d2>
14031e5a5: 48 8d 8d e0 04 00 00        	leaq	0x4e0(%rbp), %rcx
14031e5ac: e8 2f 00 2d 00              	callq	0x1405ee5e0 <.text+0x5ed5e0>
14031e5b1: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e5b6: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e5bb: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e5c1: 48 83 c4 68                 	addq	$0x68, %rsp
14031e5c5: 5b                          	popq	%rbx
14031e5c6: 5f                          	popq	%rdi
14031e5c7: 5e                          	popq	%rsi
14031e5c8: 41 5c                       	popq	%r12
14031e5ca: 41 5d                       	popq	%r13
14031e5cc: 41 5e                       	popq	%r14
14031e5ce: 41 5f                       	popq	%r15
14031e5d0: 5d                          	popq	%rbp
14031e5d1: c3                          	retq
14031e5d2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e5d7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e5dc: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e5e2: 48 83 c4 68                 	addq	$0x68, %rsp
14031e5e6: 5b                          	popq	%rbx
14031e5e7: 5f                          	popq	%rdi
14031e5e8: 5e                          	popq	%rsi
14031e5e9: 41 5c                       	popq	%r12
14031e5eb: 41 5d                       	popq	%r13
14031e5ed: 41 5e                       	popq	%r14
14031e5ef: 41 5f                       	popq	%r15
14031e5f1: 5d                          	popq	%rbp
14031e5f2: c3                          	retq
14031e5f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14031e600: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e605: 55                          	pushq	%rbp
14031e606: 41 57                       	pushq	%r15
14031e608: 41 56                       	pushq	%r14
14031e60a: 41 55                       	pushq	%r13
14031e60c: 41 54                       	pushq	%r12
14031e60e: 56                          	pushq	%rsi
14031e60f: 57                          	pushq	%rdi
14031e610: 53                          	pushq	%rbx
14031e611: 48 83 ec 68                 	subq	$0x68, %rsp
14031e615: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e61c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e622: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e627: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e62c: 48 8d 0d 1d ca 30 03        	leaq	0x330ca1d(%rip), %rcx   # 0x14362b050
14031e633: e8 f8 35 e4 ff              	callq	0x140161c30 <.text+0x160c30>
14031e638: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e63d: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e642: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e648: 48 83 c4 68                 	addq	$0x68, %rsp
14031e64c: 5b                          	popq	%rbx
14031e64d: 5f                          	popq	%rdi
14031e64e: 5e                          	popq	%rsi
14031e64f: 41 5c                       	popq	%r12
14031e651: 41 5d                       	popq	%r13
14031e653: 41 5e                       	popq	%r14
14031e655: 41 5f                       	popq	%r15
14031e657: 5d                          	popq	%rbp
14031e658: c3                          	retq
14031e659: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14031e660: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e665: 55                          	pushq	%rbp
14031e666: 41 57                       	pushq	%r15
14031e668: 41 56                       	pushq	%r14
14031e66a: 41 55                       	pushq	%r13
14031e66c: 41 54                       	pushq	%r12
14031e66e: 56                          	pushq	%rsi
14031e66f: 57                          	pushq	%rdi
14031e670: 53                          	pushq	%rbx
14031e671: 48 83 ec 68                 	subq	$0x68, %rsp
14031e675: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e67c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e682: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e687: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e68c: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031e693: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14031e69a: e8 81 ae 2e 00              	callq	0x140609520 <.text+0x608520>
14031e69f: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031e6a6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e6ab: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e6b0: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e6b6: 48 83 c4 68                 	addq	$0x68, %rsp
14031e6ba: 5b                          	popq	%rbx
14031e6bb: 5f                          	popq	%rdi
14031e6bc: 5e                          	popq	%rsi
14031e6bd: 41 5c                       	popq	%r12
14031e6bf: 41 5d                       	popq	%r13
14031e6c1: 41 5e                       	popq	%r14
14031e6c3: 41 5f                       	popq	%r15
14031e6c5: 5d                          	popq	%rbp
14031e6c6: c3                          	retq
14031e6c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031e6d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e6d5: 55                          	pushq	%rbp
14031e6d6: 41 57                       	pushq	%r15
14031e6d8: 41 56                       	pushq	%r14
14031e6da: 41 55                       	pushq	%r13
14031e6dc: 41 54                       	pushq	%r12
14031e6de: 56                          	pushq	%rsi
14031e6df: 57                          	pushq	%rdi
14031e6e0: 53                          	pushq	%rbx
14031e6e1: 48 83 ec 68                 	subq	$0x68, %rsp
14031e6e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e6ec: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e6f2: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e6f7: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e6fc: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031e703: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14031e70a: e8 11 ae 2e 00              	callq	0x140609520 <.text+0x608520>
14031e70f: c6 85 26 06 00 00 01        	movb	$0x1, 0x626(%rbp)
14031e716: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e71b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e720: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e726: 48 83 c4 68                 	addq	$0x68, %rsp
14031e72a: 5b                          	popq	%rbx
14031e72b: 5f                          	popq	%rdi
14031e72c: 5e                          	popq	%rsi
14031e72d: 41 5c                       	popq	%r12
14031e72f: 41 5d                       	popq	%r13
14031e731: 41 5e                       	popq	%r14
14031e733: 41 5f                       	popq	%r15
14031e735: 5d                          	popq	%rbp
14031e736: c3                          	retq
14031e737: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031e740: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e745: 55                          	pushq	%rbp
14031e746: 41 57                       	pushq	%r15
14031e748: 41 56                       	pushq	%r14
14031e74a: 41 55                       	pushq	%r13
14031e74c: 41 54                       	pushq	%r12
14031e74e: 56                          	pushq	%rsi
14031e74f: 57                          	pushq	%rdi
14031e750: 53                          	pushq	%rbx
14031e751: 48 83 ec 68                 	subq	$0x68, %rsp
14031e755: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e75c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e762: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e767: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e76c: 48 8b 85 18 06 00 00        	movq	0x618(%rbp), %rax
14031e773: 48 8b 8d 08 06 00 00        	movq	0x608(%rbp), %rcx
14031e77a: 48 89 41 58                 	movq	%rax, 0x58(%rcx)
14031e77e: 48 8b 85 10 06 00 00        	movq	0x610(%rbp), %rax
14031e785: 48 89 41 60                 	movq	%rax, 0x60(%rcx)
14031e789: 48 8b 85 00 06 00 00        	movq	0x600(%rbp), %rax
14031e790: 48 89 41 68                 	movq	%rax, 0x68(%rcx)
14031e794: 48 8b 85 40 03 00 00        	movq	0x340(%rbp), %rax
14031e79b: 48 8b 8d e0 05 00 00        	movq	0x5e0(%rbp), %rcx
14031e7a2: 48 89 41 20                 	movq	%rax, 0x20(%rcx)
14031e7a6: 0f 28 85 20 03 00 00        	movaps	0x320(%rbp), %xmm0
14031e7ad: 0f 28 8d 30 03 00 00        	movaps	0x330(%rbp), %xmm1
14031e7b4: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
14031e7b8: 0f 11 01                    	movups	%xmm0, (%rcx)
14031e7bb: c6 85 26 06 00 00 00        	movb	$0x0, 0x626(%rbp)
14031e7c2: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e7c7: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e7cc: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e7d2: 48 83 c4 68                 	addq	$0x68, %rsp
14031e7d6: 5b                          	popq	%rbx
14031e7d7: 5f                          	popq	%rdi
14031e7d8: 5e                          	popq	%rsi
14031e7d9: 41 5c                       	popq	%r12
14031e7db: 41 5d                       	popq	%r13
14031e7dd: 41 5e                       	popq	%r14
14031e7df: 41 5f                       	popq	%r15
14031e7e1: 5d                          	popq	%rbp
14031e7e2: c3                          	retq
14031e7e3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14031e7f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e7f5: 55                          	pushq	%rbp
14031e7f6: 41 57                       	pushq	%r15
14031e7f8: 41 56                       	pushq	%r14
14031e7fa: 41 55                       	pushq	%r13
14031e7fc: 41 54                       	pushq	%r12
14031e7fe: 56                          	pushq	%rsi
14031e7ff: 57                          	pushq	%rdi
14031e800: 53                          	pushq	%rbx
14031e801: 48 83 ec 68                 	subq	$0x68, %rsp
14031e805: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e80c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e812: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e817: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e81c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e823: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14031e82a: e8 f1 ac 2e 00              	callq	0x140609520 <.text+0x608520>
14031e82f: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e836: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e83b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e840: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e846: 48 83 c4 68                 	addq	$0x68, %rsp
14031e84a: 5b                          	popq	%rbx
14031e84b: 5f                          	popq	%rdi
14031e84c: 5e                          	popq	%rsi
14031e84d: 41 5c                       	popq	%r12
14031e84f: 41 5d                       	popq	%r13
14031e851: 41 5e                       	popq	%r14
14031e853: 41 5f                       	popq	%r15
14031e855: 5d                          	popq	%rbp
14031e856: c3                          	retq
14031e857: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031e860: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e865: 55                          	pushq	%rbp
14031e866: 41 57                       	pushq	%r15
14031e868: 41 56                       	pushq	%r14
14031e86a: 41 55                       	pushq	%r13
14031e86c: 41 54                       	pushq	%r12
14031e86e: 56                          	pushq	%rsi
14031e86f: 57                          	pushq	%rdi
14031e870: 53                          	pushq	%rbx
14031e871: 48 83 ec 68                 	subq	$0x68, %rsp
14031e875: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e87c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e882: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e887: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e88c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e893: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14031e89a: e8 81 ac 2e 00              	callq	0x140609520 <.text+0x608520>
14031e89f: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e8a6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e8ab: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e8b0: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e8b6: 48 83 c4 68                 	addq	$0x68, %rsp
14031e8ba: 5b                          	popq	%rbx
14031e8bb: 5f                          	popq	%rdi
14031e8bc: 5e                          	popq	%rsi
14031e8bd: 41 5c                       	popq	%r12
14031e8bf: 41 5d                       	popq	%r13
14031e8c1: 41 5e                       	popq	%r14
14031e8c3: 41 5f                       	popq	%r15
14031e8c5: 5d                          	popq	%rbp
14031e8c6: c3                          	retq
14031e8c7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031e8d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e8d5: 55                          	pushq	%rbp
14031e8d6: 41 57                       	pushq	%r15
14031e8d8: 41 56                       	pushq	%r14
14031e8da: 41 55                       	pushq	%r13
14031e8dc: 41 54                       	pushq	%r12
14031e8de: 56                          	pushq	%rsi
14031e8df: 57                          	pushq	%rdi
14031e8e0: 53                          	pushq	%rbx
14031e8e1: 48 83 ec 68                 	subq	$0x68, %rsp
14031e8e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e8ec: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e8f2: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e8f7: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e8fc: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e903: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14031e90a: e8 11 ac 2e 00              	callq	0x140609520 <.text+0x608520>
14031e90f: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e916: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e91b: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e920: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e926: 48 83 c4 68                 	addq	$0x68, %rsp
14031e92a: 5b                          	popq	%rbx
14031e92b: 5f                          	popq	%rdi
14031e92c: 5e                          	popq	%rsi
14031e92d: 41 5c                       	popq	%r12
14031e92f: 41 5d                       	popq	%r13
14031e931: 41 5e                       	popq	%r14
14031e933: 41 5f                       	popq	%r15
14031e935: 5d                          	popq	%rbp
14031e936: c3                          	retq
14031e937: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
14031e940: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e945: 55                          	pushq	%rbp
14031e946: 41 57                       	pushq	%r15
14031e948: 41 56                       	pushq	%r14
14031e94a: 41 55                       	pushq	%r13
14031e94c: 41 54                       	pushq	%r12
14031e94e: 56                          	pushq	%rsi
14031e94f: 57                          	pushq	%rdi
14031e950: 53                          	pushq	%rbx
14031e951: 48 83 ec 68                 	subq	$0x68, %rsp
14031e955: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e95c: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e962: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e967: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e96c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e973: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
14031e977: e8 a4 ab 2e 00              	callq	0x140609520 <.text+0x608520>
14031e97c: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e983: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e988: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031e98d: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031e993: 48 83 c4 68                 	addq	$0x68, %rsp
14031e997: 5b                          	popq	%rbx
14031e998: 5f                          	popq	%rdi
14031e999: 5e                          	popq	%rsi
14031e99a: 41 5c                       	popq	%r12
14031e99c: 41 5d                       	popq	%r13
14031e99e: 41 5e                       	popq	%r14
14031e9a0: 41 5f                       	popq	%r15
14031e9a2: 5d                          	popq	%rbp
14031e9a3: c3                          	retq
14031e9a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14031e9b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14031e9b5: 55                          	pushq	%rbp
14031e9b6: 41 57                       	pushq	%r15
14031e9b8: 41 56                       	pushq	%r14
14031e9ba: 41 55                       	pushq	%r13
14031e9bc: 41 54                       	pushq	%r12
14031e9be: 56                          	pushq	%rsi
14031e9bf: 57                          	pushq	%rdi
14031e9c0: 53                          	pushq	%rbx
14031e9c1: 48 83 ec 68                 	subq	$0x68, %rsp
14031e9c5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14031e9cc: 44 0f 29 44 24 30           	movaps	%xmm8, 0x30(%rsp)
14031e9d2: 0f 29 7c 24 40              	movaps	%xmm7, 0x40(%rsp)
14031e9d7: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
14031e9dc: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e9e3: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
14031e9ea: e8 31 ab 2e 00              	callq	0x140609520 <.text+0x608520>
14031e9ef: c6 85 27 06 00 00 01        	movb	$0x1, 0x627(%rbp)
14031e9f6: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
14031e9fb: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
14031ea00: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
14031ea06: 48 83 c4 68                 	addq	$0x68, %rsp
14031ea0a: 5b                          	popq	%rbx
14031ea0b: 5f                          	popq	%rdi
14031ea0c: 5e                          	popq	%rsi
14031ea0d: 41 5c                       	popq	%r12
14031ea0f: 41 5d                       	popq	%r13
14031ea11: 41 5e                       	popq	%r14
14031ea13: 41 5f                       	popq	%r15
14031ea15: 5d                          	popq	%rbp
14031ea16: c3                          	retq
