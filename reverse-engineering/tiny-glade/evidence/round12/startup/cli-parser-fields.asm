
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403560c0: 55                          	pushq	%rbp
1403560c1: 41 57                       	pushq	%r15
1403560c3: 41 56                       	pushq	%r14
1403560c5: 41 55                       	pushq	%r13
1403560c7: 41 54                       	pushq	%r12
1403560c9: 56                          	pushq	%rsi
1403560ca: 57                          	pushq	%rdi
1403560cb: 53                          	pushq	%rbx
1403560cc: 48 81 ec 78 01 00 00        	subq	$0x178, %rsp            # imm = 0x178
1403560d3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403560db: 48 c7 85 f0 00 00 00 fe ff ff ff    	movq	$-0x2, 0xf0(%rbp)
1403560e6: 49 89 d7                    	movq	%rdx, %r15
1403560e9: 48 89 ce                    	movq	%rcx, %rsi
1403560ec: 4c 8d 05 8f da 66 02        	leaq	0x266da8f(%rip), %r8    # 0x1429c3b82
1403560f3: 48 8d 8d d8 00 00 00        	leaq	0xd8(%rbp), %rcx
1403560fa: 41 b9 15 00 00 00           	movl	$0x15, %r9d
140356100: e8 bb 78 f0 ff              	callq	0x14025d9c0 <.text+0x25c9c0>
140356105: 48 8d 15 8b da 66 02        	leaq	0x266da8b(%rip), %rdx   # 0x1429c3b97
14035610c: 41 b8 1d 00 00 00           	movl	$0x1d, %r8d
140356112: 4c 89 f9                    	movq	%r15, %rcx
140356115: e8 16 8c f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
14035611a: 89 c3                       	movl	%eax, %ebx
14035611c: 3c 02                       	cmpb	$0x2, %al
14035611e: 75 19                       	jne	0x140356139 <.text+0x355139>
140356120: 48 8d 15 e2 db 66 02        	leaq	0x266dbe2(%rip), %rdx   # 0x1429c3d09
140356127: 41 b8 4f 00 00 00           	movl	$0x4f, %r8d
14035612d: b1 09                       	movb	$0x9, %cl
14035612f: e8 5c 32 1d 00              	callq	0x140529390 <.text+0x528390>
140356134: e9 90 01 00 00              	jmp	0x1403562c9 <.text+0x3552c9>
140356139: 48 8d 15 74 da 66 02        	leaq	0x266da74(%rip), %rdx   # 0x1429c3bb4
140356140: 41 b8 12 00 00 00           	movl	$0x12, %r8d
140356146: 4c 89 f9                    	movq	%r15, %rcx
140356149: e8 e2 8b f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
14035614e: 89 c7                       	movl	%eax, %edi
140356150: 3c 02                       	cmpb	$0x2, %al
140356152: 75 19                       	jne	0x14035616d <.text+0x35516d>
140356154: 48 8d 15 fd db 66 02        	leaq	0x266dbfd(%rip), %rdx   # 0x1429c3d58
14035615b: 41 b8 44 00 00 00           	movl	$0x44, %r8d
140356161: b1 09                       	movb	$0x9, %cl
140356163: e8 28 32 1d 00              	callq	0x140529390 <.text+0x528390>
140356168: e9 5c 01 00 00              	jmp	0x1403562c9 <.text+0x3552c9>
14035616d: 48 8d 15 52 da 66 02        	leaq	0x266da52(%rip), %rdx   # 0x1429c3bc6
140356174: 41 b8 15 00 00 00           	movl	$0x15, %r8d
14035617a: 4c 89 f9                    	movq	%r15, %rcx
14035617d: e8 ae 8b f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
140356182: 41 89 c6                    	movl	%eax, %r14d
140356185: 3c 02                       	cmpb	$0x2, %al
140356187: 75 19                       	jne	0x1403561a2 <.text+0x3551a2>
140356189: 48 8d 15 0c dc 66 02        	leaq	0x266dc0c(%rip), %rdx   # 0x1429c3d9c
140356190: 41 b8 47 00 00 00           	movl	$0x47, %r8d
140356196: b1 09                       	movb	$0x9, %cl
140356198: e8 f3 31 1d 00              	callq	0x140529390 <.text+0x528390>
14035619d: e9 27 01 00 00              	jmp	0x1403562c9 <.text+0x3552c9>
1403561a2: 48 8d 15 32 da 66 02        	leaq	0x266da32(%rip), %rdx   # 0x1429c3bdb
1403561a9: 41 b8 1b 00 00 00           	movl	$0x1b, %r8d
1403561af: 4c 89 f9                    	movq	%r15, %rcx
1403561b2: e8 79 8b f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
1403561b7: 41 89 c4                    	movl	%eax, %r12d
1403561ba: 3c 02                       	cmpb	$0x2, %al
1403561bc: 75 19                       	jne	0x1403561d7 <.text+0x3551d7>
1403561be: 48 8d 15 1e dc 66 02        	leaq	0x266dc1e(%rip), %rdx   # 0x1429c3de3
1403561c5: 41 b8 4d 00 00 00           	movl	$0x4d, %r8d
1403561cb: b1 09                       	movb	$0x9, %cl
1403561cd: e8 be 31 1d 00              	callq	0x140529390 <.text+0x528390>
1403561d2: e9 f2 00 00 00              	jmp	0x1403562c9 <.text+0x3552c9>
1403561d7: 48 8d 15 18 da 66 02        	leaq	0x266da18(%rip), %rdx   # 0x1429c3bf6
1403561de: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1403561e4: 4c 89 f9                    	movq	%r15, %rcx
1403561e7: e8 44 8b f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
1403561ec: 41 89 c5                    	movl	%eax, %r13d
1403561ef: 3c 02                       	cmpb	$0x2, %al
1403561f1: 75 19                       	jne	0x14035620c <.text+0x35520c>
1403561f3: 48 8d 15 36 dc 66 02        	leaq	0x266dc36(%rip), %rdx   # 0x1429c3e30
1403561fa: 41 b8 3c 00 00 00           	movl	$0x3c, %r8d
140356200: b1 09                       	movb	$0x9, %cl
140356202: e8 89 31 1d 00              	callq	0x140529390 <.text+0x528390>
140356207: e9 bd 00 00 00              	jmp	0x1403562c9 <.text+0x3552c9>
14035620c: 48 8d 15 ed d9 66 02        	leaq	0x266d9ed(%rip), %rdx   # 0x1429c3c00
140356213: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140356219: 4c 89 f9                    	movq	%r15, %rcx
14035621c: e8 0f 8b f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
140356221: 88 85 d2 00 00 00           	movb	%al, 0xd2(%rbp)
140356227: 3c 02                       	cmpb	$0x2, %al
140356229: 75 19                       	jne	0x140356244 <.text+0x355244>
14035622b: 48 8d 15 3a dc 66 02        	leaq	0x266dc3a(%rip), %rdx   # 0x1429c3e6c
140356232: 41 b8 42 00 00 00           	movl	$0x42, %r8d
140356238: b1 09                       	movb	$0x9, %cl
14035623a: e8 51 31 1d 00              	callq	0x140529390 <.text+0x528390>
14035623f: e9 85 00 00 00              	jmp	0x1403562c9 <.text+0x3552c9>
140356244: 48 8d 15 c5 d9 66 02        	leaq	0x266d9c5(%rip), %rdx   # 0x1429c3c10
14035624b: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140356251: 4c 89 f9                    	movq	%r15, %rcx
140356254: e8 d7 8a f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
140356259: 88 85 d3 00 00 00           	movb	%al, 0xd3(%rbp)
14035625f: 3c 02                       	cmpb	$0x2, %al
140356261: 75 16                       	jne	0x140356279 <.text+0x355279>
140356263: 48 8d 15 44 dc 66 02        	leaq	0x266dc44(%rip), %rdx   # 0x1429c3eae
14035626a: 41 b8 42 00 00 00           	movl	$0x42, %r8d
140356270: b1 09                       	movb	$0x9, %cl
140356272: e8 19 31 1d 00              	callq	0x140529390 <.text+0x528390>
140356277: eb 50                       	jmp	0x1403562c9 <.text+0x3552c9>
140356279: 48 8d 15 a0 d9 66 02        	leaq	0x266d9a0(%rip), %rdx   # 0x1429c3c20
140356280: 41 b8 10 00 00 00           	movl	$0x10, %r8d
140356286: 4c 89 f9                    	movq	%r15, %rcx
140356289: e8 c2 7d f0 ff              	callq	0x14025e050 <.text+0x25d050>
14035628e: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140356292: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
140356296: 48 8d 15 93 d9 66 02        	leaq	0x266d993(%rip), %rdx   # 0x1429c3c30
14035629d: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1403562a3: 4c 89 f9                    	movq	%r15, %rcx
1403562a6: e8 85 8a f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
1403562ab: 88 85 d4 00 00 00           	movb	%al, 0xd4(%rbp)
1403562b1: 3c 02                       	cmpb	$0x2, %al
1403562b3: 75 59                       	jne	0x14035630e <.text+0x35530e>
1403562b5: 48 8d 15 34 dc 66 02        	leaq	0x266dc34(%rip), %rdx   # 0x1429c3ef0
1403562bc: 41 b8 4a 00 00 00           	movl	$0x4a, %r8d
1403562c2: b1 09                       	movb	$0x9, %cl
1403562c4: e8 c7 30 1d 00              	callq	0x140529390 <.text+0x528390>
1403562c9: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1403562cd: 48 c7 06 02 00 00 00        	movq	$0x2, (%rsi)
1403562d4: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1403562db: 48 89 d0                    	movq	%rdx, %rax
1403562de: 48 f7 d8                    	negq	%rax
1403562e1: 70 14                       	jo	0x1403562f7 <.text+0x3552f7>
1403562e3: 73 12                       	jae	0x1403562f7 <.text+0x3552f7>
1403562e5: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1403562ec: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403562f2: e8 29 d9 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403562f7: 48 89 f0                    	movq	%rsi, %rax
1403562fa: 48 81 c4 78 01 00 00        	addq	$0x178, %rsp            # imm = 0x178
140356301: 5b                          	popq	%rbx
140356302: 5f                          	popq	%rdi
140356303: 5e                          	popq	%rsi
140356304: 41 5c                       	popq	%r12
140356306: 41 5d                       	popq	%r13
140356308: 41 5e                       	popq	%r14
14035630a: 41 5f                       	popq	%r15
14035630c: 5d                          	popq	%rbp
14035630d: c3                          	retq
14035630e: 4c 8d 05 33 d9 66 02        	leaq	0x266d933(%rip), %r8    # 0x1429c3c48
140356315: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
14035631c: 41 b9 12 00 00 00           	movl	$0x12, %r9d
140356322: 4c 89 fa                    	movq	%r15, %rdx
140356325: e8 96 83 f0 ff              	callq	0x14025e6c0 <.text+0x25d6c0>
14035632a: 48 8d 15 29 d9 66 02        	leaq	0x266d929(%rip), %rdx   # 0x1429c3c5a
140356331: 41 b8 1a 00 00 00           	movl	$0x1a, %r8d
140356337: 4c 89 f9                    	movq	%r15, %rcx
14035633a: e8 51 90 f0 ff              	callq	0x14025f390 <.text+0x25e390>
14035633f: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140356343: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
140356347: 4c 8d 05 26 d9 66 02        	leaq	0x266d926(%rip), %r8    # 0x1429c3c74
14035634e: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
140356355: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
14035635b: 4c 89 fa                    	movq	%r15, %rdx
14035635e: e8 5d 83 f0 ff              	callq	0x14025e6c0 <.text+0x25d6c0>
140356363: 4c 8d 05 16 d9 66 02        	leaq	0x266d916(%rip), %r8    # 0x1429c3c80
14035636a: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
14035636e: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
140356374: 4c 89 fa                    	movq	%r15, %rdx
140356377: e8 44 83 f0 ff              	callq	0x14025e6c0 <.text+0x25d6c0>
14035637c: 4c 8d 05 09 d9 66 02        	leaq	0x266d909(%rip), %r8    # 0x1429c3c8c
140356383: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
140356387: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
14035638d: 4c 89 fa                    	movq	%r15, %rdx
140356390: e8 2b 83 f0 ff              	callq	0x14025e6c0 <.text+0x25d6c0>
140356395: 48 8d 15 fb d8 66 02        	leaq	0x266d8fb(%rip), %rdx   # 0x1429c3c97
14035639c: 41 b8 12 00 00 00           	movl	$0x12, %r8d
1403563a2: 4c 89 f9                    	movq	%r15, %rcx
1403563a5: e8 a6 7c f0 ff              	callq	0x14025e050 <.text+0x25d050>
1403563aa: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1403563ae: 48 89 55 48                 	movq	%rdx, 0x48(%rbp)
1403563b2: 48 8d 15 f0 d8 66 02        	leaq	0x266d8f0(%rip), %rdx   # 0x1429c3ca9
1403563b9: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
1403563bf: 4c 89 f9                    	movq	%r15, %rcx
1403563c2: e8 69 89 f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
1403563c7: 88 85 d5 00 00 00           	movb	%al, 0xd5(%rbp)
1403563cd: 3c 02                       	cmpb	$0x2, %al
1403563cf: 75 16                       	jne	0x1403563e7 <.text+0x3553e7>
1403563d1: 48 8d 15 62 db 66 02        	leaq	0x266db62(%rip), %rdx   # 0x1429c3f3a
1403563d8: 41 b8 3d 00 00 00           	movl	$0x3d, %r8d
1403563de: b1 09                       	movb	$0x9, %cl
1403563e0: e8 ab 2f 1d 00              	callq	0x140529390 <.text+0x528390>
1403563e5: eb 6c                       	jmp	0x140356453 <.text+0x355453>
1403563e7: 48 8d 15 c6 d8 66 02        	leaq	0x266d8c6(%rip), %rdx   # 0x1429c3cb4
1403563ee: 41 b8 11 00 00 00           	movl	$0x11, %r8d
1403563f4: 4c 89 f9                    	movq	%r15, %rcx
1403563f7: e8 34 89 f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
1403563fc: 88 85 d6 00 00 00           	movb	%al, 0xd6(%rbp)
140356402: 3c 02                       	cmpb	$0x2, %al
140356404: 75 16                       	jne	0x14035641c <.text+0x35541c>
140356406: 48 8d 15 6a db 66 02        	leaq	0x266db6a(%rip), %rdx   # 0x1429c3f77
14035640d: 41 b8 43 00 00 00           	movl	$0x43, %r8d
140356413: b1 09                       	movb	$0x9, %cl
140356415: e8 76 2f 1d 00              	callq	0x140529390 <.text+0x528390>
14035641a: eb 37                       	jmp	0x140356453 <.text+0x355453>
14035641c: 48 8d 15 a2 d8 66 02        	leaq	0x266d8a2(%rip), %rdx   # 0x1429c3cc5
140356423: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
140356429: 4c 89 f9                    	movq	%r15, %rcx
14035642c: e8 ff 88 f0 ff              	callq	0x14025ed30 <.text+0x25dd30>
140356431: 88 85 d7 00 00 00           	movb	%al, 0xd7(%rbp)
140356437: 3c 02                       	cmpb	$0x2, %al
140356439: 0f 85 b7 00 00 00           	jne	0x1403564f6 <.text+0x3554f6>
14035643f: 48 8d 15 74 db 66 02        	leaq	0x266db74(%rip), %rdx   # 0x1429c3fba
140356446: 41 b8 3d 00 00 00           	movl	$0x3d, %r8d
14035644c: b1 09                       	movb	$0x9, %cl
14035644e: e8 3d 2f 1d 00              	callq	0x140529390 <.text+0x528390>
140356453: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
140356457: 48 c7 06 02 00 00 00        	movq	$0x2, (%rsi)
14035645e: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
140356462: 48 89 d0                    	movq	%rdx, %rax
140356465: 48 f7 d8                    	negq	%rax
140356468: 70 11                       	jo	0x14035647b <.text+0x35547b>
14035646a: 73 0f                       	jae	0x14035647b <.text+0x35547b>
14035646c: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140356470: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356476: e8 a5 d7 2b 00              	callq	0x140613c20 <.text+0x612c20>
14035647b: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
14035647f: 48 89 d0                    	movq	%rdx, %rax
140356482: 48 f7 d8                    	negq	%rax
140356485: 70 11                       	jo	0x140356498 <.text+0x355498>
140356487: 73 0f                       	jae	0x140356498 <.text+0x355498>
140356489: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14035648d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356493: e8 88 d7 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356498: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
14035649f: 48 89 d0                    	movq	%rdx, %rax
1403564a2: 48 f7 d8                    	negq	%rax
1403564a5: 70 14                       	jo	0x1403564bb <.text+0x3554bb>
1403564a7: 73 12                       	jae	0x1403564bb <.text+0x3554bb>
1403564a9: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1403564b0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403564b6: e8 65 d7 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403564bb: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1403564c2: 48 89 d0                    	movq	%rdx, %rax
1403564c5: 48 f7 d8                    	negq	%rax
1403564c8: 70 14                       	jo	0x1403564de <.text+0x3554de>
1403564ca: 73 12                       	jae	0x1403564de <.text+0x3554de>
1403564cc: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
1403564d3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403564d9: e8 42 d7 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403564de: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1403564e5: 48 89 d0                    	movq	%rdx, %rax
1403564e8: 48 f7 d8                    	negq	%rax
1403564eb: 0f 80 06 fe ff ff           	jo	0x1403562f7 <.text+0x3552f7>
1403564f1: e9 ed fd ff ff              	jmp	0x1403562e3 <.text+0x3552e3>
1403564f6: 4c 8d 05 d3 d7 66 02        	leaq	0x266d7d3(%rip), %r8    # 0x1429c3cd0
1403564fd: 48 89 e9                    	movq	%rbp, %rcx
140356500: 41 b9 02 00 00 00           	movl	$0x2, %r9d
140356506: 4c 89 fa                    	movq	%r15, %rdx
140356509: e8 b2 81 f0 ff              	callq	0x14025e6c0 <.text+0x25d6c0>
14035650e: 4c 8d 05 bd d7 66 02        	leaq	0x266d7bd(%rip), %r8    # 0x1429c3cd2
140356515: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
140356519: 41 b9 14 00 00 00           	movl	$0x14, %r9d
14035651f: 4c 89 fa                    	movq	%r15, %rdx
140356522: e8 99 81 f0 ff              	callq	0x14025e6c0 <.text+0x25d6c0>
140356527: 4c 8d 05 b8 d7 66 02        	leaq	0x266d7b8(%rip), %r8    # 0x1429c3ce6
14035652e: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140356532: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140356538: 4c 89 fa                    	movq	%r15, %rdx
14035653b: e8 80 81 f0 ff              	callq	0x14025e6c0 <.text+0x25d6c0>
140356540: 4c 8d 05 af d7 66 02        	leaq	0x266d7af(%rip), %r8    # 0x1429c3cf6
140356547: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
14035654b: 41 b9 13 00 00 00           	movl	$0x13, %r9d
140356551: 4c 89 fa                    	movq	%r15, %rdx
140356554: e8 67 74 f0 ff              	callq	0x14025d9c0 <.text+0x25c9c0>
140356559: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
140356560: 48 89 46 40                 	movq	%rax, 0x40(%rsi)
140356564: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
14035656b: 0f 11 46 30                 	movups	%xmm0, 0x30(%rsi)
14035656f: 0f 10 85 b0 00 00 00        	movups	0xb0(%rbp), %xmm0
140356576: 0f 10 8d c0 00 00 00        	movups	0xc0(%rbp), %xmm1
14035657d: 0f 11 4e 58                 	movups	%xmm1, 0x58(%rsi)
140356581: 0f 11 46 48                 	movups	%xmm0, 0x48(%rsi)
140356585: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
14035658c: 0f 10 8d a0 00 00 00        	movups	0xa0(%rbp), %xmm1
140356593: 0f 11 4e 78                 	movups	%xmm1, 0x78(%rsi)
140356597: 0f 11 46 68                 	movups	%xmm0, 0x68(%rsi)
14035659b: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14035659f: 0f 10 8d 80 00 00 00        	movups	0x80(%rbp), %xmm1
1403565a6: 0f 11 8e 98 00 00 00        	movups	%xmm1, 0x98(%rsi)
1403565ad: 0f 11 86 88 00 00 00        	movups	%xmm0, 0x88(%rsi)
1403565b4: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
1403565b8: 0f 10 4d 60                 	movups	0x60(%rbp), %xmm1
1403565bc: 0f 11 8e b8 00 00 00        	movups	%xmm1, 0xb8(%rsi)
1403565c3: 0f 11 86 a8 00 00 00        	movups	%xmm0, 0xa8(%rsi)
1403565ca: 0f 10 45 00                 	movups	(%rbp), %xmm0
1403565ce: 0f 10 4d 10                 	movups	0x10(%rbp), %xmm1
1403565d2: 0f 11 86 c8 00 00 00        	movups	%xmm0, 0xc8(%rsi)
1403565d9: 0f 11 8e d8 00 00 00        	movups	%xmm1, 0xd8(%rsi)
1403565e0: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
1403565e4: 0f 10 4d f0                 	movups	-0x10(%rbp), %xmm1
1403565e8: 0f 11 86 e8 00 00 00        	movups	%xmm0, 0xe8(%rsi)
1403565ef: 0f 11 8e f8 00 00 00        	movups	%xmm1, 0xf8(%rsi)
1403565f6: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
1403565fa: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
1403565fe: 0f 11 8e 18 01 00 00        	movups	%xmm1, 0x118(%rsi)
140356605: 0f 11 86 08 01 00 00        	movups	%xmm0, 0x108(%rsi)
14035660c: 48 8b 45 b8                 	movq	-0x48(%rbp), %rax
140356610: 48 89 86 38 01 00 00        	movq	%rax, 0x138(%rsi)
140356617: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
14035661b: 0f 11 86 28 01 00 00        	movups	%xmm0, 0x128(%rsi)
140356622: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140356626: 48 89 06                    	movq	%rax, (%rsi)
140356629: 48 8b 45 28                 	movq	0x28(%rbp), %rax
14035662d: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
140356631: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140356635: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140356639: 48 8b 45 38                 	movq	0x38(%rbp), %rax
14035663d: 48 89 46 18                 	movq	%rax, 0x18(%rsi)
140356641: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140356645: 48 89 46 20                 	movq	%rax, 0x20(%rsi)
140356649: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14035664d: 48 89 46 28                 	movq	%rax, 0x28(%rsi)
140356651: 88 9e 40 01 00 00           	movb	%bl, 0x140(%rsi)
140356657: 40 88 be 41 01 00 00        	movb	%dil, 0x141(%rsi)
14035665e: 44 88 b6 42 01 00 00        	movb	%r14b, 0x142(%rsi)
140356665: 44 88 a6 43 01 00 00        	movb	%r12b, 0x143(%rsi)
14035666c: 44 88 ae 44 01 00 00        	movb	%r13b, 0x144(%rsi)
140356673: 0f b6 85 d2 00 00 00        	movzbl	0xd2(%rbp), %eax
14035667a: 88 86 45 01 00 00           	movb	%al, 0x145(%rsi)
140356680: 0f b6 85 d3 00 00 00        	movzbl	0xd3(%rbp), %eax
140356687: 88 86 46 01 00 00           	movb	%al, 0x146(%rsi)
14035668d: 0f b6 85 d4 00 00 00        	movzbl	0xd4(%rbp), %eax
140356694: 88 86 47 01 00 00           	movb	%al, 0x147(%rsi)
14035669a: 0f b6 85 d5 00 00 00        	movzbl	0xd5(%rbp), %eax
1403566a1: 88 86 48 01 00 00           	movb	%al, 0x148(%rsi)
1403566a7: 0f b6 85 d6 00 00 00        	movzbl	0xd6(%rbp), %eax
1403566ae: 88 86 49 01 00 00           	movb	%al, 0x149(%rsi)
1403566b4: 0f b6 85 d7 00 00 00        	movzbl	0xd7(%rbp), %eax
1403566bb: 88 86 4a 01 00 00           	movb	%al, 0x14a(%rsi)
1403566c1: e9 31 fc ff ff              	jmp	0x1403562f7 <.text+0x3552f7>
1403566c6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1403566d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403566d5: 55                          	pushq	%rbp
1403566d6: 41 57                       	pushq	%r15
1403566d8: 41 56                       	pushq	%r14
1403566da: 41 55                       	pushq	%r13
1403566dc: 41 54                       	pushq	%r12
1403566de: 56                          	pushq	%rsi
1403566df: 57                          	pushq	%rdi
1403566e0: 53                          	pushq	%rbx
1403566e1: 48 83 ec 28                 	subq	$0x28, %rsp
1403566e5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403566ec: 48 8b 55 c0                 	movq	-0x40(%rbp), %rdx
1403566f0: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1403566f8: 48 85 c0                    	testq	%rax, %rax
1403566fb: 74 0f                       	je	0x14035670c <.text+0x35570c>
1403566fd: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
140356701: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356707: e8 14 d5 2b 00              	callq	0x140613c20 <.text+0x612c20>
14035670c: 90                          	nop
14035670d: 48 83 c4 28                 	addq	$0x28, %rsp
140356711: 5b                          	popq	%rbx
140356712: 5f                          	popq	%rdi
140356713: 5e                          	popq	%rsi
140356714: 41 5c                       	popq	%r12
140356716: 41 5d                       	popq	%r13
140356718: 41 5e                       	popq	%r14
14035671a: 41 5f                       	popq	%r15
14035671c: 5d                          	popq	%rbp
14035671d: c3                          	retq
14035671e: 66 90                       	nop
140356720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356725: 55                          	pushq	%rbp
140356726: 41 57                       	pushq	%r15
140356728: 41 56                       	pushq	%r14
14035672a: 41 55                       	pushq	%r13
14035672c: 41 54                       	pushq	%r12
14035672e: 56                          	pushq	%rsi
14035672f: 57                          	pushq	%rdi
140356730: 53                          	pushq	%rbx
140356731: 48 83 ec 28                 	subq	$0x28, %rsp
140356735: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14035673c: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140356740: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356748: 48 85 c0                    	testq	%rax, %rax
14035674b: 74 0f                       	je	0x14035675c <.text+0x35575c>
14035674d: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
140356751: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356757: e8 c4 d4 2b 00              	callq	0x140613c20 <.text+0x612c20>
14035675c: 90                          	nop
14035675d: 48 83 c4 28                 	addq	$0x28, %rsp
140356761: 5b                          	popq	%rbx
140356762: 5f                          	popq	%rdi
140356763: 5e                          	popq	%rsi
140356764: 41 5c                       	popq	%r12
140356766: 41 5d                       	popq	%r13
140356768: 41 5e                       	popq	%r14
14035676a: 41 5f                       	popq	%r15
14035676c: 5d                          	popq	%rbp
14035676d: c3                          	retq
14035676e: 66 90                       	nop
140356770: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356775: 55                          	pushq	%rbp
140356776: 41 57                       	pushq	%r15
140356778: 41 56                       	pushq	%r14
14035677a: 41 55                       	pushq	%r13
14035677c: 41 54                       	pushq	%r12
14035677e: 56                          	pushq	%rsi
14035677f: 57                          	pushq	%rdi
140356780: 53                          	pushq	%rbx
140356781: 48 83 ec 28                 	subq	$0x28, %rsp
140356785: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14035678c: 48 8b 55 00                 	movq	(%rbp), %rdx
140356790: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356798: 48 85 c0                    	testq	%rax, %rax
14035679b: 74 0f                       	je	0x1403567ac <.text+0x3557ac>
14035679d: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
1403567a1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403567a7: e8 74 d4 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403567ac: 90                          	nop
1403567ad: 48 83 c4 28                 	addq	$0x28, %rsp
1403567b1: 5b                          	popq	%rbx
1403567b2: 5f                          	popq	%rdi
1403567b3: 5e                          	popq	%rsi
1403567b4: 41 5c                       	popq	%r12
1403567b6: 41 5d                       	popq	%r13
1403567b8: 41 5e                       	popq	%r14
1403567ba: 41 5f                       	popq	%r15
1403567bc: 5d                          	popq	%rbp
1403567bd: c3                          	retq
1403567be: 66 90                       	nop
1403567c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403567c5: 55                          	pushq	%rbp
1403567c6: 41 57                       	pushq	%r15
1403567c8: 41 56                       	pushq	%r14
1403567ca: 41 55                       	pushq	%r13
1403567cc: 41 54                       	pushq	%r12
1403567ce: 56                          	pushq	%rsi
1403567cf: 57                          	pushq	%rdi
1403567d0: 53                          	pushq	%rbx
1403567d1: 48 83 ec 28                 	subq	$0x28, %rsp
1403567d5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403567dc: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
1403567e0: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1403567e8: 48 85 c0                    	testq	%rax, %rax
1403567eb: 75 6a                       	jne	0x140356857 <.text+0x355857>
1403567ed: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1403567f1: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1403567f9: 48 85 c0                    	testq	%rax, %rax
1403567fc: 75 79                       	jne	0x140356877 <.text+0x355877>
1403567fe: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
140356805: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
14035680d: 48 85 c0                    	testq	%rax, %rax
140356810: 0f 85 88 00 00 00           	jne	0x14035689e <.text+0x35589e>
140356816: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
14035681d: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356825: 48 85 c0                    	testq	%rax, %rax
140356828: 0f 85 9a 00 00 00           	jne	0x1403568c8 <.text+0x3558c8>
14035682e: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356835: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
14035683d: 48 85 c0                    	testq	%rax, %rax
140356840: 0f 85 ac 00 00 00           	jne	0x1403568f2 <.text+0x3558f2>
140356846: 48 83 c4 28                 	addq	$0x28, %rsp
14035684a: 5b                          	popq	%rbx
14035684b: 5f                          	popq	%rdi
14035684c: 5e                          	popq	%rsi
14035684d: 41 5c                       	popq	%r12
14035684f: 41 5d                       	popq	%r13
140356851: 41 5e                       	popq	%r14
140356853: 41 5f                       	popq	%r15
140356855: 5d                          	popq	%rbp
140356856: c3                          	retq
140356857: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
14035685b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356861: e8 ba d3 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356866: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
14035686a: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356872: 48 85 c0                    	testq	%rax, %rax
140356875: 74 87                       	je	0x1403567fe <.text+0x3557fe>
140356877: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14035687b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356881: e8 9a d3 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356886: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
14035688d: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356895: 48 85 c0                    	testq	%rax, %rax
140356898: 0f 84 78 ff ff ff           	je	0x140356816 <.text+0x355816>
14035689e: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1403568a5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403568ab: e8 70 d3 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403568b0: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1403568b7: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1403568bf: 48 85 c0                    	testq	%rax, %rax
1403568c2: 0f 84 66 ff ff ff           	je	0x14035682e <.text+0x35582e>
1403568c8: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
1403568cf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403568d5: e8 46 d3 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403568da: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
1403568e1: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1403568e9: 48 85 c0                    	testq	%rax, %rax
1403568ec: 0f 84 54 ff ff ff           	je	0x140356846 <.text+0x355846>
1403568f2: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
1403568f9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403568ff: e8 1c d3 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356904: e9 3d ff ff ff              	jmp	0x140356846 <.text+0x355846>
140356909: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140356910: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356915: 55                          	pushq	%rbp
140356916: 41 57                       	pushq	%r15
140356918: 41 56                       	pushq	%r14
14035691a: 41 55                       	pushq	%r13
14035691c: 41 54                       	pushq	%r12
14035691e: 56                          	pushq	%rsi
14035691f: 57                          	pushq	%rdi
140356920: 53                          	pushq	%rbx
140356921: 48 83 ec 28                 	subq	$0x28, %rsp
140356925: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14035692c: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
140356930: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356938: 48 85 c0                    	testq	%rax, %rax
14035693b: 75 6a                       	jne	0x1403569a7 <.text+0x3559a7>
14035693d: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140356941: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356949: 48 85 c0                    	testq	%rax, %rax
14035694c: 75 79                       	jne	0x1403569c7 <.text+0x3559c7>
14035694e: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
140356955: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
14035695d: 48 85 c0                    	testq	%rax, %rax
140356960: 0f 85 88 00 00 00           	jne	0x1403569ee <.text+0x3559ee>
140356966: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
14035696d: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356975: 48 85 c0                    	testq	%rax, %rax
140356978: 0f 85 9a 00 00 00           	jne	0x140356a18 <.text+0x355a18>
14035697e: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356985: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
14035698d: 48 85 c0                    	testq	%rax, %rax
140356990: 0f 85 ac 00 00 00           	jne	0x140356a42 <.text+0x355a42>
140356996: 48 83 c4 28                 	addq	$0x28, %rsp
14035699a: 5b                          	popq	%rbx
14035699b: 5f                          	popq	%rdi
14035699c: 5e                          	popq	%rsi
14035699d: 41 5c                       	popq	%r12
14035699f: 41 5d                       	popq	%r13
1403569a1: 41 5e                       	popq	%r14
1403569a3: 41 5f                       	popq	%r15
1403569a5: 5d                          	popq	%rbp
1403569a6: c3                          	retq
1403569a7: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
1403569ab: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403569b1: e8 6a d2 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403569b6: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
1403569ba: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1403569c2: 48 85 c0                    	testq	%rax, %rax
1403569c5: 74 87                       	je	0x14035694e <.text+0x35594e>
1403569c7: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
1403569cb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403569d1: e8 4a d2 2b 00              	callq	0x140613c20 <.text+0x612c20>
1403569d6: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
1403569dd: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
1403569e5: 48 85 c0                    	testq	%rax, %rax
1403569e8: 0f 84 78 ff ff ff           	je	0x140356966 <.text+0x355966>
1403569ee: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
1403569f5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403569fb: e8 20 d2 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356a00: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
140356a07: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356a0f: 48 85 c0                    	testq	%rax, %rax
140356a12: 0f 84 66 ff ff ff           	je	0x14035697e <.text+0x35597e>
140356a18: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
140356a1f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356a25: e8 f6 d1 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356a2a: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356a31: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356a39: 48 85 c0                    	testq	%rax, %rax
140356a3c: 0f 84 54 ff ff ff           	je	0x140356996 <.text+0x355996>
140356a42: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356a49: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356a4f: e8 cc d1 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356a54: e9 3d ff ff ff              	jmp	0x140356996 <.text+0x355996>
140356a59: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140356a60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356a65: 55                          	pushq	%rbp
140356a66: 41 57                       	pushq	%r15
140356a68: 41 56                       	pushq	%r14
140356a6a: 41 55                       	pushq	%r13
140356a6c: 41 54                       	pushq	%r12
140356a6e: 56                          	pushq	%rsi
140356a6f: 57                          	pushq	%rdi
140356a70: 53                          	pushq	%rbx
140356a71: 48 83 ec 28                 	subq	$0x28, %rsp
140356a75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356a7c: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
140356a80: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356a88: 48 85 c0                    	testq	%rax, %rax
140356a8b: 75 6a                       	jne	0x140356af7 <.text+0x355af7>
140356a8d: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140356a91: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356a99: 48 85 c0                    	testq	%rax, %rax
140356a9c: 75 79                       	jne	0x140356b17 <.text+0x355b17>
140356a9e: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
140356aa5: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356aad: 48 85 c0                    	testq	%rax, %rax
140356ab0: 0f 85 88 00 00 00           	jne	0x140356b3e <.text+0x355b3e>
140356ab6: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
140356abd: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356ac5: 48 85 c0                    	testq	%rax, %rax
140356ac8: 0f 85 9a 00 00 00           	jne	0x140356b68 <.text+0x355b68>
140356ace: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356ad5: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356add: 48 85 c0                    	testq	%rax, %rax
140356ae0: 0f 85 ac 00 00 00           	jne	0x140356b92 <.text+0x355b92>
140356ae6: 48 83 c4 28                 	addq	$0x28, %rsp
140356aea: 5b                          	popq	%rbx
140356aeb: 5f                          	popq	%rdi
140356aec: 5e                          	popq	%rsi
140356aed: 41 5c                       	popq	%r12
140356aef: 41 5d                       	popq	%r13
140356af1: 41 5e                       	popq	%r14
140356af3: 41 5f                       	popq	%r15
140356af5: 5d                          	popq	%rbp
140356af6: c3                          	retq
140356af7: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140356afb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356b01: e8 1a d1 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356b06: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140356b0a: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356b12: 48 85 c0                    	testq	%rax, %rax
140356b15: 74 87                       	je	0x140356a9e <.text+0x355a9e>
140356b17: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140356b1b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356b21: e8 fa d0 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356b26: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
140356b2d: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356b35: 48 85 c0                    	testq	%rax, %rax
140356b38: 0f 84 78 ff ff ff           	je	0x140356ab6 <.text+0x355ab6>
140356b3e: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140356b45: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356b4b: e8 d0 d0 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356b50: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
140356b57: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356b5f: 48 85 c0                    	testq	%rax, %rax
140356b62: 0f 84 66 ff ff ff           	je	0x140356ace <.text+0x355ace>
140356b68: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
140356b6f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356b75: e8 a6 d0 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356b7a: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356b81: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356b89: 48 85 c0                    	testq	%rax, %rax
140356b8c: 0f 84 54 ff ff ff           	je	0x140356ae6 <.text+0x355ae6>
140356b92: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356b99: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356b9f: e8 7c d0 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356ba4: e9 3d ff ff ff              	jmp	0x140356ae6 <.text+0x355ae6>
140356ba9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140356bb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356bb5: 55                          	pushq	%rbp
140356bb6: 41 57                       	pushq	%r15
140356bb8: 41 56                       	pushq	%r14
140356bba: 41 55                       	pushq	%r13
140356bbc: 41 54                       	pushq	%r12
140356bbe: 56                          	pushq	%rsi
140356bbf: 57                          	pushq	%rdi
140356bc0: 53                          	pushq	%rbx
140356bc1: 48 83 ec 28                 	subq	$0x28, %rsp
140356bc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356bcc: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
140356bd0: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356bd8: 48 85 c0                    	testq	%rax, %rax
140356bdb: 74 0f                       	je	0x140356bec <.text+0x355bec>
140356bdd: 48 8b 4d 58                 	movq	0x58(%rbp), %rcx
140356be1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356be7: e8 34 d0 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356bec: 90                          	nop
140356bed: 48 83 c4 28                 	addq	$0x28, %rsp
140356bf1: 5b                          	popq	%rbx
140356bf2: 5f                          	popq	%rdi
140356bf3: 5e                          	popq	%rsi
140356bf4: 41 5c                       	popq	%r12
140356bf6: 41 5d                       	popq	%r13
140356bf8: 41 5e                       	popq	%r14
140356bfa: 41 5f                       	popq	%r15
140356bfc: 5d                          	popq	%rbp
140356bfd: c3                          	retq
140356bfe: 66 90                       	nop
140356c00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356c05: 55                          	pushq	%rbp
140356c06: 41 57                       	pushq	%r15
140356c08: 41 56                       	pushq	%r14
140356c0a: 41 55                       	pushq	%r13
140356c0c: 41 54                       	pushq	%r12
140356c0e: 56                          	pushq	%rsi
140356c0f: 57                          	pushq	%rdi
140356c10: 53                          	pushq	%rbx
140356c11: 48 83 ec 28                 	subq	$0x28, %rsp
140356c15: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356c1c: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
140356c20: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356c28: 48 85 c0                    	testq	%rax, %rax
140356c2b: 74 0f                       	je	0x140356c3c <.text+0x355c3c>
140356c2d: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140356c31: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356c37: e8 e4 cf 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356c3c: 90                          	nop
140356c3d: 48 83 c4 28                 	addq	$0x28, %rsp
140356c41: 5b                          	popq	%rbx
140356c42: 5f                          	popq	%rdi
140356c43: 5e                          	popq	%rsi
140356c44: 41 5c                       	popq	%r12
140356c46: 41 5d                       	popq	%r13
140356c48: 41 5e                       	popq	%r14
140356c4a: 41 5f                       	popq	%r15
140356c4c: 5d                          	popq	%rbp
140356c4d: c3                          	retq
140356c4e: 66 90                       	nop
140356c50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356c55: 55                          	pushq	%rbp
140356c56: 41 57                       	pushq	%r15
140356c58: 41 56                       	pushq	%r14
140356c5a: 41 55                       	pushq	%r13
140356c5c: 41 54                       	pushq	%r12
140356c5e: 56                          	pushq	%rsi
140356c5f: 57                          	pushq	%rdi
140356c60: 53                          	pushq	%rbx
140356c61: 48 83 ec 28                 	subq	$0x28, %rsp
140356c65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356c6c: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
140356c73: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356c7b: 48 85 c0                    	testq	%rax, %rax
140356c7e: 74 12                       	je	0x140356c92 <.text+0x355c92>
140356c80: 48 8b 8d 98 00 00 00        	movq	0x98(%rbp), %rcx
140356c87: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356c8d: e8 8e cf 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356c92: 90                          	nop
140356c93: 48 83 c4 28                 	addq	$0x28, %rsp
140356c97: 5b                          	popq	%rbx
140356c98: 5f                          	popq	%rdi
140356c99: 5e                          	popq	%rsi
140356c9a: 41 5c                       	popq	%r12
140356c9c: 41 5d                       	popq	%r13
140356c9e: 41 5e                       	popq	%r14
140356ca0: 41 5f                       	popq	%r15
140356ca2: 5d                          	popq	%rbp
140356ca3: c3                          	retq
140356ca4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356cb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356cb5: 55                          	pushq	%rbp
140356cb6: 41 57                       	pushq	%r15
140356cb8: 41 56                       	pushq	%r14
140356cba: 41 55                       	pushq	%r13
140356cbc: 41 54                       	pushq	%r12
140356cbe: 56                          	pushq	%rsi
140356cbf: 57                          	pushq	%rdi
140356cc0: 53                          	pushq	%rbx
140356cc1: 48 83 ec 28                 	subq	$0x28, %rsp
140356cc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356ccc: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356cd3: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356cdb: 48 85 c0                    	testq	%rax, %rax
140356cde: 74 12                       	je	0x140356cf2 <.text+0x355cf2>
140356ce0: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356ce7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356ced: e8 2e cf 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356cf2: 90                          	nop
140356cf3: 48 83 c4 28                 	addq	$0x28, %rsp
140356cf7: 5b                          	popq	%rbx
140356cf8: 5f                          	popq	%rdi
140356cf9: 5e                          	popq	%rsi
140356cfa: 41 5c                       	popq	%r12
140356cfc: 41 5d                       	popq	%r13
140356cfe: 41 5e                       	popq	%r14
140356d00: 41 5f                       	popq	%r15
140356d02: 5d                          	popq	%rbp
140356d03: c3                          	retq
140356d04: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356d10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356d15: 55                          	pushq	%rbp
140356d16: 41 57                       	pushq	%r15
140356d18: 41 56                       	pushq	%r14
140356d1a: 41 55                       	pushq	%r13
140356d1c: 41 54                       	pushq	%r12
140356d1e: 56                          	pushq	%rsi
140356d1f: 57                          	pushq	%rdi
140356d20: 53                          	pushq	%rbx
140356d21: 48 83 ec 28                 	subq	$0x28, %rsp
140356d25: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356d2c: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356d33: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356d3b: 48 85 c0                    	testq	%rax, %rax
140356d3e: 74 12                       	je	0x140356d52 <.text+0x355d52>
140356d40: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356d47: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356d4d: e8 ce ce 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356d52: 90                          	nop
140356d53: 48 83 c4 28                 	addq	$0x28, %rsp
140356d57: 5b                          	popq	%rbx
140356d58: 5f                          	popq	%rdi
140356d59: 5e                          	popq	%rsi
140356d5a: 41 5c                       	popq	%r12
140356d5c: 41 5d                       	popq	%r13
140356d5e: 41 5e                       	popq	%r14
140356d60: 41 5f                       	popq	%r15
140356d62: 5d                          	popq	%rbp
140356d63: c3                          	retq
140356d64: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356d70: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356d75: 55                          	pushq	%rbp
140356d76: 41 57                       	pushq	%r15
140356d78: 41 56                       	pushq	%r14
140356d7a: 41 55                       	pushq	%r13
140356d7c: 41 54                       	pushq	%r12
140356d7e: 56                          	pushq	%rsi
140356d7f: 57                          	pushq	%rdi
140356d80: 53                          	pushq	%rbx
140356d81: 48 83 ec 28                 	subq	$0x28, %rsp
140356d85: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356d8c: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
140356d93: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356d9b: 48 85 c0                    	testq	%rax, %rax
140356d9e: 74 12                       	je	0x140356db2 <.text+0x355db2>
140356da0: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
140356da7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356dad: e8 6e ce 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356db2: 90                          	nop
140356db3: 48 83 c4 28                 	addq	$0x28, %rsp
140356db7: 5b                          	popq	%rbx
140356db8: 5f                          	popq	%rdi
140356db9: 5e                          	popq	%rsi
140356dba: 41 5c                       	popq	%r12
140356dbc: 41 5d                       	popq	%r13
140356dbe: 41 5e                       	popq	%r14
140356dc0: 41 5f                       	popq	%r15
140356dc2: 5d                          	popq	%rbp
140356dc3: c3                          	retq
140356dc4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356dd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356dd5: 55                          	pushq	%rbp
140356dd6: 41 57                       	pushq	%r15
140356dd8: 41 56                       	pushq	%r14
140356dda: 41 55                       	pushq	%r13
140356ddc: 41 54                       	pushq	%r12
140356dde: 56                          	pushq	%rsi
140356ddf: 57                          	pushq	%rdi
140356de0: 53                          	pushq	%rbx
140356de1: 48 83 ec 28                 	subq	$0x28, %rsp
140356de5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356dec: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356df3: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356dfb: 48 85 c0                    	testq	%rax, %rax
140356dfe: 74 12                       	je	0x140356e12 <.text+0x355e12>
140356e00: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356e07: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356e0d: e8 0e ce 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356e12: 90                          	nop
140356e13: 48 83 c4 28                 	addq	$0x28, %rsp
140356e17: 5b                          	popq	%rbx
140356e18: 5f                          	popq	%rdi
140356e19: 5e                          	popq	%rsi
140356e1a: 41 5c                       	popq	%r12
140356e1c: 41 5d                       	popq	%r13
140356e1e: 41 5e                       	popq	%r14
140356e20: 41 5f                       	popq	%r15
140356e22: 5d                          	popq	%rbp
140356e23: c3                          	retq
140356e24: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356e30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356e35: 55                          	pushq	%rbp
140356e36: 41 57                       	pushq	%r15
140356e38: 41 56                       	pushq	%r14
140356e3a: 41 55                       	pushq	%r13
140356e3c: 41 54                       	pushq	%r12
140356e3e: 56                          	pushq	%rsi
140356e3f: 57                          	pushq	%rdi
140356e40: 53                          	pushq	%rbx
140356e41: 48 83 ec 28                 	subq	$0x28, %rsp
140356e45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356e4c: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356e53: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356e5b: 48 85 c0                    	testq	%rax, %rax
140356e5e: 74 12                       	je	0x140356e72 <.text+0x355e72>
140356e60: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356e67: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356e6d: e8 ae cd 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356e72: 90                          	nop
140356e73: 48 83 c4 28                 	addq	$0x28, %rsp
140356e77: 5b                          	popq	%rbx
140356e78: 5f                          	popq	%rdi
140356e79: 5e                          	popq	%rsi
140356e7a: 41 5c                       	popq	%r12
140356e7c: 41 5d                       	popq	%r13
140356e7e: 41 5e                       	popq	%r14
140356e80: 41 5f                       	popq	%r15
140356e82: 5d                          	popq	%rbp
140356e83: c3                          	retq
140356e84: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356e90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356e95: 55                          	pushq	%rbp
140356e96: 41 57                       	pushq	%r15
140356e98: 41 56                       	pushq	%r14
140356e9a: 41 55                       	pushq	%r13
140356e9c: 41 54                       	pushq	%r12
140356e9e: 56                          	pushq	%rsi
140356e9f: 57                          	pushq	%rdi
140356ea0: 53                          	pushq	%rbx
140356ea1: 48 83 ec 28                 	subq	$0x28, %rsp
140356ea5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356eac: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356eb3: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356ebb: 48 85 c0                    	testq	%rax, %rax
140356ebe: 74 12                       	je	0x140356ed2 <.text+0x355ed2>
140356ec0: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356ec7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356ecd: e8 4e cd 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356ed2: 90                          	nop
140356ed3: 48 83 c4 28                 	addq	$0x28, %rsp
140356ed7: 5b                          	popq	%rbx
140356ed8: 5f                          	popq	%rdi
140356ed9: 5e                          	popq	%rsi
140356eda: 41 5c                       	popq	%r12
140356edc: 41 5d                       	popq	%r13
140356ede: 41 5e                       	popq	%r14
140356ee0: 41 5f                       	popq	%r15
140356ee2: 5d                          	popq	%rbp
140356ee3: c3                          	retq
140356ee4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356ef0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356ef5: 55                          	pushq	%rbp
140356ef6: 41 57                       	pushq	%r15
140356ef8: 41 56                       	pushq	%r14
140356efa: 41 55                       	pushq	%r13
140356efc: 41 54                       	pushq	%r12
140356efe: 56                          	pushq	%rsi
140356eff: 57                          	pushq	%rdi
140356f00: 53                          	pushq	%rbx
140356f01: 48 83 ec 28                 	subq	$0x28, %rsp
140356f05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356f0c: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356f13: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356f1b: 48 85 c0                    	testq	%rax, %rax
140356f1e: 74 12                       	je	0x140356f32 <.text+0x355f32>
140356f20: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356f27: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356f2d: e8 ee cc 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356f32: 90                          	nop
140356f33: 48 83 c4 28                 	addq	$0x28, %rsp
140356f37: 5b                          	popq	%rbx
140356f38: 5f                          	popq	%rdi
140356f39: 5e                          	popq	%rsi
140356f3a: 41 5c                       	popq	%r12
140356f3c: 41 5d                       	popq	%r13
140356f3e: 41 5e                       	popq	%r14
140356f40: 41 5f                       	popq	%r15
140356f42: 5d                          	popq	%rbp
140356f43: c3                          	retq
140356f44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356f50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356f55: 55                          	pushq	%rbp
140356f56: 41 57                       	pushq	%r15
140356f58: 41 56                       	pushq	%r14
140356f5a: 41 55                       	pushq	%r13
140356f5c: 41 54                       	pushq	%r12
140356f5e: 56                          	pushq	%rsi
140356f5f: 57                          	pushq	%rdi
140356f60: 53                          	pushq	%rbx
140356f61: 48 83 ec 28                 	subq	$0x28, %rsp
140356f65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356f6c: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356f73: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356f7b: 48 85 c0                    	testq	%rax, %rax
140356f7e: 74 12                       	je	0x140356f92 <.text+0x355f92>
140356f80: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356f87: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356f8d: e8 8e cc 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356f92: 90                          	nop
140356f93: 48 83 c4 28                 	addq	$0x28, %rsp
140356f97: 5b                          	popq	%rbx
140356f98: 5f                          	popq	%rdi
140356f99: 5e                          	popq	%rsi
140356f9a: 41 5c                       	popq	%r12
140356f9c: 41 5d                       	popq	%r13
140356f9e: 41 5e                       	popq	%r14
140356fa0: 41 5f                       	popq	%r15
140356fa2: 5d                          	popq	%rbp
140356fa3: c3                          	retq
140356fa4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140356fb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140356fb5: 55                          	pushq	%rbp
140356fb6: 41 57                       	pushq	%r15
140356fb8: 41 56                       	pushq	%r14
140356fba: 41 55                       	pushq	%r13
140356fbc: 41 54                       	pushq	%r12
140356fbe: 56                          	pushq	%rsi
140356fbf: 57                          	pushq	%rdi
140356fc0: 53                          	pushq	%rbx
140356fc1: 48 83 ec 28                 	subq	$0x28, %rsp
140356fc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140356fcc: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140356fd3: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
140356fdb: 48 85 c0                    	testq	%rax, %rax
140356fde: 74 12                       	je	0x140356ff2 <.text+0x355ff2>
140356fe0: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140356fe7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140356fed: e8 2e cc 2b 00              	callq	0x140613c20 <.text+0x612c20>
140356ff2: 90                          	nop
140356ff3: 48 83 c4 28                 	addq	$0x28, %rsp
140356ff7: 5b                          	popq	%rbx
140356ff8: 5f                          	popq	%rdi
140356ff9: 5e                          	popq	%rsi
140356ffa: 41 5c                       	popq	%r12
140356ffc: 41 5d                       	popq	%r13
140356ffe: 41 5e                       	popq	%r14
140357000: 41 5f                       	popq	%r15
140357002: 5d                          	popq	%rbp
140357003: c3                          	retq
140357004: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140357010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140357015: 55                          	pushq	%rbp
140357016: 41 57                       	pushq	%r15
140357018: 41 56                       	pushq	%r14
14035701a: 41 55                       	pushq	%r13
14035701c: 41 54                       	pushq	%r12
14035701e: 56                          	pushq	%rsi
14035701f: 57                          	pushq	%rdi
140357020: 53                          	pushq	%rbx
140357021: 48 83 ec 28                 	subq	$0x28, %rsp
140357025: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14035702c: 48 8b 95 d8 00 00 00        	movq	0xd8(%rbp), %rdx
140357033: 48 8d 04 55 00 00 00 00     	leaq	(,%rdx,2), %rax
14035703b: 48 85 c0                    	testq	%rax, %rax
14035703e: 74 12                       	je	0x140357052 <.text+0x356052>
140357040: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140357047: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14035704d: e8 ce cb 2b 00              	callq	0x140613c20 <.text+0x612c20>
140357052: 90                          	nop
140357053: 48 83 c4 28                 	addq	$0x28, %rsp
140357057: 5b                          	popq	%rbx
140357058: 5f                          	popq	%rdi
140357059: 5e                          	popq	%rsi
14035705a: 41 5c                       	popq	%r12
14035705c: 41 5d                       	popq	%r13
14035705e: 41 5e                       	popq	%r14
140357060: 41 5f                       	popq	%r15
140357062: 5d                          	popq	%rbp
140357063: c3                          	retq
