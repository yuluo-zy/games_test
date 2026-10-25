
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a92670: 55                          	pushq	%rbp
140a92671: 41 57                       	pushq	%r15
140a92673: 41 56                       	pushq	%r14
140a92675: 41 55                       	pushq	%r13
140a92677: 41 54                       	pushq	%r12
140a92679: 56                          	pushq	%rsi
140a9267a: 57                          	pushq	%rdi
140a9267b: 53                          	pushq	%rbx
140a9267c: 48 81 ec 68 01 00 00        	subq	$0x168, %rsp            # imm = 0x168
140a92683: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a9268b: 0f 29 b5 d0 00 00 00        	movaps	%xmm6, 0xd0(%rbp)
140a92692: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
140a9269d: 4c 89 cb                    	movq	%r9, %rbx
140a926a0: 4c 89 c6                    	movq	%r8, %rsi
140a926a3: 48 89 d7                    	movq	%rdx, %rdi
140a926a6: 49 89 cf                    	movq	%rcx, %r15
140a926a9: 48 8d 15 b0 7b 04 02        	leaq	0x2047bb0(%rip), %rdx   # 0x142ada260
140a926b0: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a926b4: 41 b8 18 00 00 00           	movl	$0x18, %r8d
140a926ba: e8 f1 c7 07 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
140a926bf: 31 c0                       	xorl	%eax, %eax
140a926c1: 48 3b 45 50                 	cmpq	0x50(%rbp), %rax
140a926c5: 0f 80 d4 07 00 00           	jo	0x140a92e9f <.text+0xa91e9f>
140a926cb: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140a926d2: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140a926d6: 0f 10 85 b0 00 00 00        	movups	0xb0(%rbp), %xmm0
140a926dd: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
140a926e1: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
140a926e8: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140a926ec: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
140a926f3: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140a926f7: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140a926fb: 0f 10 4d 60                 	movups	0x60(%rbp), %xmm1
140a926ff: 0f 10 55 70                 	movups	0x70(%rbp), %xmm2
140a92703: 0f 10 9d 80 00 00 00        	movups	0x80(%rbp), %xmm3
140a9270a: 0f 29 5d f0                 	movaps	%xmm3, -0x10(%rbp)
140a9270e: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
140a92712: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
140a92716: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140a9271a: 48 8b 36                    	movq	(%rsi), %rsi
140a9271d: 48 8b 3f                    	movq	(%rdi), %rdi
140a92720: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
140a92725: 48 8d 15 64 7b 04 02        	leaq	0x2047b64(%rip), %rdx   # 0x142ada290
140a9272c: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a92730: 41 b8 05 00 00 00           	movl	$0x5, %r8d
140a92736: 49 89 f1                    	movq	%rsi, %r9
140a92739: e8 62 85 08 00              	callq	0x140b1aca0 <.text+0xb19ca0>
140a9273e: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140a92742: 0f b6 05 c0 24 ba 02        	movzbl	0x2ba24c0(%rip), %eax   # 0x143634c09
140a92749: b9 48 00 00 00              	movl	$0x48, %ecx
140a9274e: ba 04 00 00 00              	movl	$0x4, %edx
140a92753: e8 b8 14 b8 ff              	callq	0x140613c10 <.text+0x612c10>
140a92758: 48 85 c0                    	testq	%rax, %rax
140a9275b: 0f 84 70 07 00 00           	je	0x140a92ed1 <.text+0xa91ed1>
140a92761: 0f 28 05 d8 72 04 02        	movaps	0x20472d8(%rip), %xmm0  # 0x142ad9a40
140a92768: 0f 11 00                    	movups	%xmm0, (%rax)
140a9276b: 0f 28 05 de 72 04 02        	movaps	0x20472de(%rip), %xmm0  # 0x142ad9a50
140a92772: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
140a92776: 0f 28 05 e3 72 04 02        	movaps	0x20472e3(%rip), %xmm0  # 0x142ad9a60
140a9277d: 0f 11 40 20                 	movups	%xmm0, 0x20(%rax)
140a92781: 0f 28 05 e8 72 04 02        	movaps	0x20472e8(%rip), %xmm0  # 0x142ad9a70
140a92788: 0f 11 40 30                 	movups	%xmm0, 0x30(%rax)
140a9278c: 48 b9 03 00 00 00 01 00 00 00       	movabsq	$0x100000003, %rcx # imm = 0x100000003
140a92796: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140a9279a: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
140a9279e: 31 c9                       	xorl	%ecx, %ecx
140a927a0: e8 7b 73 64 ff              	callq	0x1400d9b20 <.text+0xd8b20>
140a927a5: 48 85 c0                    	testq	%rax, %rax
140a927a8: 0f 84 32 07 00 00           	je	0x140a92ee0 <.text+0xa91ee0>
140a927ae: 4c 8b ad 58 01 00 00        	movq	0x158(%rbp), %r13
140a927b5: 4c 8b a5 50 01 00 00        	movq	0x150(%rbp), %r12
140a927bc: 0f 10 00                    	movups	(%rax), %xmm0
140a927bf: 48 ff 00                    	incq	(%rax)
140a927c2: 0f 10 0d f7 3c 03 02        	movups	0x2033cf7(%rip), %xmm1  # 0x142ac64c0
140a927c9: 0f 11 4d d8                 	movups	%xmm1, -0x28(%rbp)
140a927cd: 0f 10 0d fc 3c 03 02        	movups	0x2033cfc(%rip), %xmm1  # 0x142ac64d0
140a927d4: 0f 11 4d e8                 	movups	%xmm1, -0x18(%rbp)
140a927d8: 48 c7 45 50 12 00 00 00     	movq	$0x12, 0x50(%rbp)
140a927e0: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140a927e4: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140a927e8: 48 c7 45 60 12 00 00 00     	movq	$0x12, 0x60(%rbp)
140a927f0: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a927fa: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140a927fe: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
140a92802: 0f 11 4d 70                 	movups	%xmm1, 0x70(%rbp)
140a92806: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
140a9280a: 0f 11 8d 80 00 00 00        	movups	%xmm1, 0x80(%rbp)
140a92811: 0f 10 4d e0                 	movups	-0x20(%rbp), %xmm1
140a92815: 0f 11 8d 90 00 00 00        	movups	%xmm1, 0x90(%rbp)
140a9281c: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a92820: 48 89 85 a0 00 00 00        	movq	%rax, 0xa0(%rbp)
140a92827: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
140a9282e: 48 8d 05 c9 7a 04 02        	leaq	0x2047ac9(%rip), %rax   # 0x142ada2fe
140a92835: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
140a9283c: 48 c7 85 c0 00 00 00 0f 00 00 00    	movq	$0xf, 0xc0(%rbp)
140a92847: 48 89 7c 24 20              	movq	%rdi, 0x20(%rsp)
140a9284c: 48 8d 15 42 7a 04 02        	leaq	0x2047a42(%rip), %rdx   # 0x142ada295
140a92853: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a92857: 41 b8 09 00 00 00           	movl	$0x9, %r8d
140a9285d: 49 89 f1                    	movq	%rsi, %r9
140a92860: e8 3b 84 08 00              	callq	0x140b1aca0 <.text+0xb19ca0>
140a92865: 48 89 c6                    	movq	%rax, %rsi
140a92868: 49 8b 3f                    	movq	(%r15), %rdi
140a9286b: 48 8b 0f                    	movq	(%rdi), %rcx
140a9286e: 48 83 c1 10                 	addq	$0x10, %rcx
140a92872: 4d 8b 34 24                 	movq	(%r12), %r14
140a92876: 48 8b 1b                    	movq	(%rbx), %rbx
140a92879: 48 89 5c 24 30              	movq	%rbx, 0x30(%rsp)
140a9287e: 4c 89 74 24 28              	movq	%r14, 0x28(%rsp)
140a92883: 48 c7 44 24 20 0f 00 00 00  	movq	$0xf, 0x20(%rsp)
140a9288c: 48 8d 15 0b 7a 04 02        	leaq	0x2047a0b(%rip), %rdx   # 0x142ada29e
140a92893: 4c 8d 0d 25 7a 04 02        	leaq	0x2047a25(%rip), %r9    # 0x142ada2bf
140a9289a: 41 b8 21 00 00 00           	movl	$0x21, %r8d
140a928a0: e8 9b d5 0a 00              	callq	0x140b3fe40 <.text+0xb3ee40>
140a928a5: 49 89 c7                    	movq	%rax, %r15
140a928a8: 48 8b 0f                    	movq	(%rdi), %rcx
140a928ab: 48 83 c1 10                 	addq	$0x10, %rcx
140a928af: 48 89 5c 24 30              	movq	%rbx, 0x30(%rsp)
140a928b4: 4c 89 74 24 28              	movq	%r14, 0x28(%rsp)
140a928b9: 48 c7 44 24 20 0f 00 00 00  	movq	$0xf, 0x20(%rsp)
140a928c2: 48 8d 15 05 7a 04 02        	leaq	0x2047a05(%rip), %rdx   # 0x142ada2ce
140a928c9: 4c 8d 0d 1f 7a 04 02        	leaq	0x2047a1f(%rip), %r9    # 0x142ada2ef
140a928d0: 41 b8 21 00 00 00           	movl	$0x21, %r8d
140a928d6: e8 65 d5 0a 00              	callq	0x140b3fe40 <.text+0xb3ee40>
140a928db: 48 89 c3                    	movq	%rax, %rbx
140a928de: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a928e2: 4c 89 ea                    	movq	%r13, %rdx
140a928e5: e8 b6 1d 28 00              	callq	0x140d146a0 <.text+0xd136a0>
140a928ea: 4c 8b 65 58                 	movq	0x58(%rbp), %r12
140a928ee: 4d 85 e4                    	testq	%r12, %r12
140a928f1: 75 04                       	jne	0x140a928f7 <.text+0xa918f7>
140a928f3: 4c 8b 65 60                 	movq	0x60(%rbp), %r12
140a928f7: 4c 8b 75 70                 	movq	0x70(%rbp), %r14
140a928fb: 49 8b 04 24                 	movq	(%r12), %rax
140a928ff: 4d 8b 6c 24 10              	movq	0x10(%r12), %r13
140a92904: 4c 29 e8                    	subq	%r13, %rax
140a92907: 48 3d 88 00 00 00           	cmpq	$0x88, %rax
140a9290d: 0f 82 89 04 00 00           	jb	0x140a92d9c <.text+0xa91d9c>
140a92913: 49 8b 44 24 08              	movq	0x8(%r12), %rax
140a92918: 48 8d 3d 21 3d 84 00        	leaq	0x843d21(%rip), %rdi    # 0x1412d6640 <.text+0x12d5640>
140a9291f: 4a 89 3c 28                 	movq	%rdi, (%rax,%r13)
140a92923: 0f 28 35 96 b7 ea 01        	movaps	0x1eab796(%rip), %xmm6  # 0x14293e0c0
140a9292a: 42 0f 11 74 28 08           	movups	%xmm6, 0x8(%rax,%r13)
140a92930: 4a c7 44 28 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r13)
140a92939: 42 c7 44 28 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r13)
140a92942: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140a9294c: 4a 89 4c 28 24              	movq	%rcx, 0x24(%rax,%r13)
140a92951: 42 c7 44 28 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r13) # imm = 0x3F800000
140a9295a: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a9295e: 4a 89 4c 28 38              	movq	%rcx, 0x38(%rax,%r13)
140a92963: 4e 89 7c 28 40              	movq	%r15, 0x40(%rax,%r13)
140a92968: 66 42 c7 44 28 48 00 00     	movw	$0x0, 0x48(%rax,%r13)
140a92970: 42 c6 44 28 58 00           	movb	$0x0, 0x58(%rax,%r13)
140a92976: 4e 89 74 28 68              	movq	%r14, 0x68(%rax,%r13)
140a9297b: 48 8d 0d ae 23 ad 00        	leaq	0xad23ae(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140a92982: 4a 89 4c 28 78              	movq	%rcx, 0x78(%rax,%r13)
140a92987: 49 81 c5 88 00 00 00        	addq	$0x88, %r13
140a9298e: 4d 89 6c 24 10              	movq	%r13, 0x10(%r12)
140a92993: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a92997: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
140a9299e: e8 fd 1c 28 00              	callq	0x140d146a0 <.text+0xd136a0>
140a929a3: 4c 8b 65 58                 	movq	0x58(%rbp), %r12
140a929a7: 4d 85 e4                    	testq	%r12, %r12
140a929aa: 75 04                       	jne	0x140a929b0 <.text+0xa919b0>
140a929ac: 4c 8b 65 60                 	movq	0x60(%rbp), %r12
140a929b0: 4c 8b 75 70                 	movq	0x70(%rbp), %r14
140a929b4: 49 8b 04 24                 	movq	(%r12), %rax
140a929b8: 4d 8b 6c 24 10              	movq	0x10(%r12), %r13
140a929bd: 4c 29 e8                    	subq	%r13, %rax
140a929c0: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
140a929c6: 0f 86 f5 03 00 00           	jbe	0x140a92dc1 <.text+0xa91dc1>
140a929cc: 49 8b 44 24 08              	movq	0x8(%r12), %rax
140a929d1: 4a 89 3c 28                 	movq	%rdi, (%rax,%r13)
140a929d5: 42 0f 11 74 28 08           	movups	%xmm6, 0x8(%rax,%r13)
140a929db: 4a c7 44 28 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r13)
140a929e4: 42 c7 44 28 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r13)
140a929ed: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140a929f7: 4a 89 4c 28 24              	movq	%rcx, 0x24(%rax,%r13)
140a929fc: 42 c7 44 28 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r13) # imm = 0x3F800000
140a92a05: 4a 89 74 28 38              	movq	%rsi, 0x38(%rax,%r13)
140a92a0a: 4a 89 5c 28 40              	movq	%rbx, 0x40(%rax,%r13)
140a92a0f: 66 42 c7 44 28 48 00 00     	movw	$0x0, 0x48(%rax,%r13)
140a92a17: 42 c6 44 28 58 00           	movb	$0x0, 0x58(%rax,%r13)
140a92a1d: 4e 89 74 28 68              	movq	%r14, 0x68(%rax,%r13)
140a92a22: 48 8d 0d 07 23 ad 00        	leaq	0xad2307(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140a92a29: 4a 89 4c 28 78              	movq	%rcx, 0x78(%rax,%r13)
140a92a2e: 49 81 c5 88 00 00 00        	addq	$0x88, %r13
140a92a35: 4d 89 6c 24 10              	movq	%r13, 0x10(%r12)
140a92a3a: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a92a3e: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
140a92a45: e8 56 1c 28 00              	callq	0x140d146a0 <.text+0xd136a0>
140a92a4a: 4c 8b 65 58                 	movq	0x58(%rbp), %r12
140a92a4e: 4d 85 e4                    	testq	%r12, %r12
140a92a51: 75 04                       	jne	0x140a92a57 <.text+0xa91a57>
140a92a53: 4c 8b 65 60                 	movq	0x60(%rbp), %r12
140a92a57: 4c 8b 75 70                 	movq	0x70(%rbp), %r14
140a92a5b: 49 8b 04 24                 	movq	(%r12), %rax
140a92a5f: 4d 8b 6c 24 10              	movq	0x10(%r12), %r13
140a92a64: 4c 29 e8                    	subq	%r13, %rax
140a92a67: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
140a92a6d: 0f 86 73 03 00 00           	jbe	0x140a92de6 <.text+0xa91de6>
140a92a73: 49 8b 44 24 08              	movq	0x8(%r12), %rax
140a92a78: 4a 89 3c 28                 	movq	%rdi, (%rax,%r13)
140a92a7c: 42 0f 11 74 28 08           	movups	%xmm6, 0x8(%rax,%r13)
140a92a82: 4a c7 44 28 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r13)
140a92a8b: 42 c7 44 28 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r13)
140a92a94: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140a92a9e: 4a 89 4c 28 24              	movq	%rcx, 0x24(%rax,%r13)
140a92aa3: 42 c7 44 28 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r13) # imm = 0x3F800000
140a92aac: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a92ab0: 4a 89 4c 28 38              	movq	%rcx, 0x38(%rax,%r13)
140a92ab5: 4e 89 7c 28 40              	movq	%r15, 0x40(%rax,%r13)
140a92aba: 66 42 c7 44 28 48 01 00     	movw	$0x1, 0x48(%rax,%r13)
140a92ac2: 42 c6 44 28 58 00           	movb	$0x0, 0x58(%rax,%r13)
140a92ac8: 4e 89 74 28 68              	movq	%r14, 0x68(%rax,%r13)
140a92acd: 48 8d 0d 5c 22 ad 00        	leaq	0xad225c(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140a92ad4: 4a 89 4c 28 78              	movq	%rcx, 0x78(%rax,%r13)
140a92ad9: 49 81 c5 88 00 00 00        	addq	$0x88, %r13
140a92ae0: 4d 89 6c 24 10              	movq	%r13, 0x10(%r12)
140a92ae5: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a92ae9: 4c 8b ad 58 01 00 00        	movq	0x158(%rbp), %r13
140a92af0: 4c 89 ea                    	movq	%r13, %rdx
140a92af3: e8 a8 1b 28 00              	callq	0x140d146a0 <.text+0xd136a0>
140a92af8: 4c 8b 75 58                 	movq	0x58(%rbp), %r14
140a92afc: 4d 85 f6                    	testq	%r14, %r14
140a92aff: 75 04                       	jne	0x140a92b05 <.text+0xa91b05>
140a92b01: 4c 8b 75 60                 	movq	0x60(%rbp), %r14
140a92b05: 4c 8b 65 70                 	movq	0x70(%rbp), %r12
140a92b09: 49 8b 06                    	movq	(%r14), %rax
140a92b0c: 4d 8b 7e 10                 	movq	0x10(%r14), %r15
140a92b10: 4c 29 f8                    	subq	%r15, %rax
140a92b13: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
140a92b19: 0f 86 ec 02 00 00           	jbe	0x140a92e0b <.text+0xa91e0b>
140a92b1f: 49 8b 46 08                 	movq	0x8(%r14), %rax
140a92b23: 4a 89 3c 38                 	movq	%rdi, (%rax,%r15)
140a92b27: 42 0f 11 74 38 08           	movups	%xmm6, 0x8(%rax,%r15)
140a92b2d: 4a c7 44 38 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r15)
140a92b36: 42 c7 44 38 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r15)
140a92b3f: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140a92b49: 4a 89 4c 38 24              	movq	%rcx, 0x24(%rax,%r15)
140a92b4e: 42 c7 44 38 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r15) # imm = 0x3F800000
140a92b57: 4a 89 74 38 38              	movq	%rsi, 0x38(%rax,%r15)
140a92b5c: 4a 89 5c 38 40              	movq	%rbx, 0x40(%rax,%r15)
140a92b61: 66 42 c7 44 38 48 01 00     	movw	$0x1, 0x48(%rax,%r15)
140a92b69: 42 c6 44 38 58 00           	movb	$0x0, 0x58(%rax,%r15)
140a92b6f: 4e 89 64 38 68              	movq	%r12, 0x68(%rax,%r15)
140a92b74: 48 8d 0d b5 21 ad 00        	leaq	0xad21b5(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140a92b7b: 4a 89 4c 38 78              	movq	%rcx, 0x78(%rax,%r15)
140a92b80: 49 81 c7 88 00 00 00        	addq	$0x88, %r15
140a92b87: 4d 89 7e 10                 	movq	%r15, 0x10(%r14)
140a92b8b: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a92b8f: 4c 89 ea                    	movq	%r13, %rdx
140a92b92: e8 09 1b 28 00              	callq	0x140d146a0 <.text+0xd136a0>
140a92b97: 4c 8b 75 58                 	movq	0x58(%rbp), %r14
140a92b9b: 4d 85 f6                    	testq	%r14, %r14
140a92b9e: 75 04                       	jne	0x140a92ba4 <.text+0xa91ba4>
140a92ba0: 4c 8b 75 60                 	movq	0x60(%rbp), %r14
140a92ba4: 48 8b 7d 70                 	movq	0x70(%rbp), %rdi
140a92ba8: 49 8b 06                    	movq	(%r14), %rax
140a92bab: 4d 8b 7e 10                 	movq	0x10(%r14), %r15
140a92baf: 4c 29 f8                    	subq	%r15, %rax
140a92bb2: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
140a92bb8: 0f 86 72 02 00 00           	jbe	0x140a92e30 <.text+0xa91e30>
140a92bbe: 49 8b 46 08                 	movq	0x8(%r14), %rax
140a92bc2: 4c 8d 25 c7 a5 84 00        	leaq	0x84a5c7(%rip), %r12    # 0x1412dd190 <.text+0x12dc190>
140a92bc9: 4e 89 24 38                 	movq	%r12, (%rax,%r15)
140a92bcd: 42 0f 11 74 38 08           	movups	%xmm6, 0x8(%rax,%r15)
140a92bd3: 4a c7 44 38 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r15)
140a92bdc: 42 c7 44 38 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r15)
140a92be5: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140a92bef: 4a 89 4c 38 24              	movq	%rcx, 0x24(%rax,%r15)
140a92bf4: 42 c7 44 38 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r15) # imm = 0x3F800000
140a92bfd: 4a 89 74 38 38              	movq	%rsi, 0x38(%rax,%r15)
140a92c02: 4a 89 5c 38 40              	movq	%rbx, 0x40(%rax,%r15)
140a92c07: 66 42 c7 44 38 48 00 01     	movw	$0x100, 0x48(%rax,%r15) # imm = 0x100
140a92c0f: 42 c6 44 38 4a 00           	movb	$0x0, 0x4a(%rax,%r15)
140a92c15: 42 c6 44 38 58 00           	movb	$0x0, 0x58(%rax,%r15)
140a92c1b: 4a 89 7c 38 68              	movq	%rdi, 0x68(%rax,%r15)
140a92c20: 48 8d 0d 09 21 ad 00        	leaq	0xad2109(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140a92c27: 4a 89 4c 38 78              	movq	%rcx, 0x78(%rax,%r15)
140a92c2c: 49 81 c7 88 00 00 00        	addq	$0x88, %r15
140a92c33: 4d 89 7e 10                 	movq	%r15, 0x10(%r14)
140a92c37: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a92c3b: 4c 89 ea                    	movq	%r13, %rdx
140a92c3e: e8 5d 1a 28 00              	callq	0x140d146a0 <.text+0xd136a0>
140a92c43: 4c 8b 75 58                 	movq	0x58(%rbp), %r14
140a92c47: 4d 85 f6                    	testq	%r14, %r14
140a92c4a: 75 04                       	jne	0x140a92c50 <.text+0xa91c50>
140a92c4c: 4c 8b 75 60                 	movq	0x60(%rbp), %r14
140a92c50: 48 8b 7d 70                 	movq	0x70(%rbp), %rdi
140a92c54: 49 8b 06                    	movq	(%r14), %rax
140a92c57: 4d 8b 7e 10                 	movq	0x10(%r14), %r15
140a92c5b: 4c 29 f8                    	subq	%r15, %rax
140a92c5e: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
140a92c64: 0f 86 eb 01 00 00           	jbe	0x140a92e55 <.text+0xa91e55>
140a92c6a: 49 8b 46 08                 	movq	0x8(%r14), %rax
140a92c6e: 4e 89 24 38                 	movq	%r12, (%rax,%r15)
140a92c72: 42 0f 11 74 38 08           	movups	%xmm6, 0x8(%rax,%r15)
140a92c78: 4a c7 44 38 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r15)
140a92c81: 42 c7 44 38 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r15)
140a92c8a: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140a92c94: 4a 89 4c 38 24              	movq	%rcx, 0x24(%rax,%r15)
140a92c99: 42 c7 44 38 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r15) # imm = 0x3F800000
140a92ca2: 4a 89 74 38 38              	movq	%rsi, 0x38(%rax,%r15)
140a92ca7: 4a 89 5c 38 40              	movq	%rbx, 0x40(%rax,%r15)
140a92cac: 66 42 c7 44 38 48 00 01     	movw	$0x100, 0x48(%rax,%r15) # imm = 0x100
140a92cb4: 42 c6 44 38 4a 01           	movb	$0x1, 0x4a(%rax,%r15)
140a92cba: 42 c6 44 38 58 00           	movb	$0x0, 0x58(%rax,%r15)
140a92cc0: 4a 89 7c 38 68              	movq	%rdi, 0x68(%rax,%r15)
140a92cc5: 48 8d 0d 64 20 ad 00        	leaq	0xad2064(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140a92ccc: 4a 89 4c 38 78              	movq	%rcx, 0x78(%rax,%r15)
140a92cd1: 49 81 c7 88 00 00 00        	addq	$0x88, %r15
140a92cd8: 4d 89 7e 10                 	movq	%r15, 0x10(%r14)
140a92cdc: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140a92ce0: 4c 89 ea                    	movq	%r13, %rdx
140a92ce3: e8 b8 19 28 00              	callq	0x140d146a0 <.text+0xd136a0>
140a92ce8: 48 8b 7d 58                 	movq	0x58(%rbp), %rdi
140a92cec: 48 85 ff                    	testq	%rdi, %rdi
140a92cef: 75 04                       	jne	0x140a92cf5 <.text+0xa91cf5>
140a92cf1: 48 8b 7d 60                 	movq	0x60(%rbp), %rdi
140a92cf5: 4c 8b 7d 70                 	movq	0x70(%rbp), %r15
140a92cf9: 48 8b 07                    	movq	(%rdi), %rax
140a92cfc: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
140a92d00: 4c 29 f0                    	subq	%r14, %rax
140a92d03: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
140a92d09: 0f 86 6b 01 00 00           	jbe	0x140a92e7a <.text+0xa91e7a>
140a92d0f: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140a92d13: 4e 89 24 30                 	movq	%r12, (%rax,%r14)
140a92d17: 42 0f 11 74 30 08           	movups	%xmm6, 0x8(%rax,%r14)
140a92d1d: 4a c7 44 30 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r14)
140a92d26: 42 c7 44 30 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r14)
140a92d2f: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
140a92d39: 4a 89 4c 30 24              	movq	%rcx, 0x24(%rax,%r14)
140a92d3e: 42 c7 44 30 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r14) # imm = 0x3F800000
140a92d47: 4a 89 74 30 38              	movq	%rsi, 0x38(%rax,%r14)
140a92d4c: 4a 89 5c 30 40              	movq	%rbx, 0x40(%rax,%r14)
140a92d51: 66 42 c7 44 30 48 00 01     	movw	$0x100, 0x48(%rax,%r14) # imm = 0x100
140a92d59: 42 c6 44 30 4a 02           	movb	$0x2, 0x4a(%rax,%r14)
140a92d5f: 42 c6 44 30 58 00           	movb	$0x0, 0x58(%rax,%r14)
140a92d65: 4e 89 7c 30 68              	movq	%r15, 0x68(%rax,%r14)
140a92d6a: 48 8d 0d bf 1f ad 00        	leaq	0xad1fbf(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
140a92d71: 4a 89 4c 30 78              	movq	%rcx, 0x78(%rax,%r14)
140a92d76: 49 81 c6 88 00 00 00        	addq	$0x88, %r14
140a92d7d: 4c 89 77 10                 	movq	%r14, 0x10(%rdi)
140a92d81: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
140a92d88: 48 81 c4 68 01 00 00        	addq	$0x168, %rsp            # imm = 0x168
140a92d8f: 5b                          	popq	%rbx
140a92d90: 5f                          	popq	%rdi
140a92d91: 5e                          	popq	%rsi
140a92d92: 41 5c                       	popq	%r12
140a92d94: 41 5d                       	popq	%r13
140a92d96: 41 5e                       	popq	%r14
140a92d98: 41 5f                       	popq	%r15
140a92d9a: 5d                          	popq	%rbp
140a92d9b: c3                          	retq
140a92d9c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a92da5: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140a92dab: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a92db1: 4c 89 e1                    	movq	%r12, %rcx
140a92db4: 4c 89 ea                    	movq	%r13, %rdx
140a92db7: e8 a4 6c e1 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a92dbc: e9 52 fb ff ff              	jmp	0x140a92913 <.text+0xa91913>
140a92dc1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a92dca: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140a92dd0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a92dd6: 4c 89 e1                    	movq	%r12, %rcx
140a92dd9: 4c 89 ea                    	movq	%r13, %rdx
140a92ddc: e8 7f 6c e1 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a92de1: e9 e6 fb ff ff              	jmp	0x140a929cc <.text+0xa919cc>
140a92de6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a92def: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140a92df5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a92dfb: 4c 89 e1                    	movq	%r12, %rcx
140a92dfe: 4c 89 ea                    	movq	%r13, %rdx
140a92e01: e8 5a 6c e1 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a92e06: e9 68 fc ff ff              	jmp	0x140a92a73 <.text+0xa91a73>
140a92e0b: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a92e14: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140a92e1a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a92e20: 4c 89 f1                    	movq	%r14, %rcx
140a92e23: 4c 89 fa                    	movq	%r15, %rdx
140a92e26: e8 35 6c e1 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a92e2b: e9 ef fc ff ff              	jmp	0x140a92b1f <.text+0xa91b1f>
140a92e30: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a92e39: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140a92e3f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a92e45: 4c 89 f1                    	movq	%r14, %rcx
140a92e48: 4c 89 fa                    	movq	%r15, %rdx
140a92e4b: e8 10 6c e1 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a92e50: e9 69 fd ff ff              	jmp	0x140a92bbe <.text+0xa91bbe>
140a92e55: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a92e5e: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140a92e64: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a92e6a: 4c 89 f1                    	movq	%r14, %rcx
140a92e6d: 4c 89 fa                    	movq	%r15, %rdx
140a92e70: e8 eb 6b e1 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a92e75: e9 f0 fd ff ff              	jmp	0x140a92c6a <.text+0xa91c6a>
140a92e7a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a92e83: 41 b8 88 00 00 00           	movl	$0x88, %r8d
140a92e89: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a92e8f: 48 89 f9                    	movq	%rdi, %rcx
140a92e92: 4c 89 f2                    	movq	%r14, %rdx
140a92e95: e8 c6 6b e1 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a92e9a: e9 70 fe ff ff              	jmp	0x140a92d0f <.text+0xa91d0f>
140a92e9f: 48 8b 45 58                 	movq	0x58(%rbp), %rax
140a92ea3: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140a92ea7: 48 8d 05 ca 73 04 02        	leaq	0x20473ca(%rip), %rax   # 0x142ada278
140a92eae: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a92eb3: 48 8d 0d 16 71 04 02        	leaq	0x2047116(%rip), %rcx   # 0x142ad9fd0
140a92eba: 4c 8d 0d ef 70 04 02        	leaq	0x20470ef(%rip), %r9    # 0x142ad9fb0
140a92ec1: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
140a92ec5: ba 2b 00 00 00              	movl	$0x2b, %edx
140a92eca: e8 91 68 e4 01              	callq	0x1428d9760 <.text+0x28d8760>
140a92ecf: eb 1b                       	jmp	0x140a92eec <.text+0xa91eec>
140a92ed1: b9 04 00 00 00              	movl	$0x4, %ecx
140a92ed6: ba 48 00 00 00              	movl	$0x48, %edx
140a92edb: e8 20 61 e4 01              	callq	0x1428d9000 <.text+0x28d8000>
140a92ee0: 48 8d 0d b9 7b 1a 02        	leaq	0x21a7bb9(%rip), %rcx   # 0x142c3aaa0
140a92ee7: e8 e4 51 e4 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a92eec: 0f 0b                       	ud2
140a92eee: 66 90                       	nop
140a92ef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a92ef5: 55                          	pushq	%rbp
140a92ef6: 41 57                       	pushq	%r15
140a92ef8: 41 56                       	pushq	%r14
140a92efa: 41 55                       	pushq	%r13
140a92efc: 41 54                       	pushq	%r12
140a92efe: 56                          	pushq	%rsi
140a92eff: 57                          	pushq	%rdi
140a92f00: 53                          	pushq	%rbx
140a92f01: 48 83 ec 48                 	subq	$0x48, %rsp
140a92f05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a92f0c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
140a92f11: ba 48 00 00 00              	movl	$0x48, %edx
140a92f16: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140a92f1c: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
140a92f20: e8 fb 0c b8 ff              	callq	0x140613c20 <.text+0x612c20>
140a92f25: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140a92f2a: 48 83 c4 48                 	addq	$0x48, %rsp
140a92f2e: 5b                          	popq	%rbx
140a92f2f: 5f                          	popq	%rdi
140a92f30: 5e                          	popq	%rsi
140a92f31: 41 5c                       	popq	%r12
140a92f33: 41 5d                       	popq	%r13
140a92f35: 41 5e                       	popq	%r14
140a92f37: 41 5f                       	popq	%r15
140a92f39: 5d                          	popq	%rbp
140a92f3a: c3                          	retq
140a92f3b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140a92f40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a92f45: 55                          	pushq	%rbp
140a92f46: 41 57                       	pushq	%r15
140a92f48: 41 56                       	pushq	%r14
140a92f4a: 41 55                       	pushq	%r13
140a92f4c: 41 54                       	pushq	%r12
140a92f4e: 56                          	pushq	%rsi
140a92f4f: 57                          	pushq	%rdi
140a92f50: 53                          	pushq	%rbx
140a92f51: 48 83 ec 48                 	subq	$0x48, %rsp
140a92f55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a92f5c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
140a92f61: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a92f65: e8 c6 9c 2b 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140a92f6a: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
140a92f6f: 48 83 c4 48                 	addq	$0x48, %rsp
140a92f73: 5b                          	popq	%rbx
140a92f74: 5f                          	popq	%rdi
140a92f75: 5e                          	popq	%rsi
140a92f76: 41 5c                       	popq	%r12
140a92f78: 41 5d                       	popq	%r13
140a92f7a: 41 5e                       	popq	%r14
140a92f7c: 41 5f                       	popq	%r15
140a92f7e: 5d                          	popq	%rbp
140a92f7f: c3                          	retq
