
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400d52f0: 55                          	pushq	%rbp
1400d52f1: 56                          	pushq	%rsi
1400d52f2: 57                          	pushq	%rdi
1400d52f3: 53                          	pushq	%rbx
1400d52f4: 48 81 ec c8 0a 00 00        	subq	$0xac8, %rsp            # imm = 0xAC8
1400d52fb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400d5303: 48 c7 85 40 0a 00 00 fe ff ff ff    	movq	$-0x2, 0xa40(%rbp)
1400d530e: 48 89 d7                    	movq	%rdx, %rdi
1400d5311: 48 89 ce                    	movq	%rcx, %rsi
1400d5314: 0f 10 42 60                 	movups	0x60(%rdx), %xmm0
1400d5318: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
1400d531c: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
1400d5320: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
1400d5324: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
1400d5328: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
1400d532c: 0f 10 02                    	movups	(%rdx), %xmm0
1400d532f: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
1400d5333: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
1400d5337: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
1400d533b: 0f 29 5d 40                 	movaps	%xmm3, 0x40(%rbp)
1400d533f: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
1400d5343: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
1400d5347: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1400d534b: 0f 10 82 d0 00 00 00        	movups	0xd0(%rdx), %xmm0
1400d5352: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
1400d5356: 0f 10 82 c0 00 00 00        	movups	0xc0(%rdx), %xmm0
1400d535d: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1400d5361: 0f 10 82 b0 00 00 00        	movups	0xb0(%rdx), %xmm0
1400d5368: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1400d536c: 0f 10 42 70                 	movups	0x70(%rdx), %xmm0
1400d5370: 0f 10 8a 80 00 00 00        	movups	0x80(%rdx), %xmm1
1400d5377: 0f 10 92 90 00 00 00        	movups	0x90(%rdx), %xmm2
1400d537e: 0f 10 9a a0 00 00 00        	movups	0xa0(%rdx), %xmm3
1400d5385: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
1400d5389: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
1400d538d: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
1400d5391: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
1400d5395: 0f 10 82 40 01 00 00        	movups	0x140(%rdx), %xmm0
1400d539c: 0f 29 85 30 09 00 00        	movaps	%xmm0, 0x930(%rbp)
1400d53a3: 0f 10 82 30 01 00 00        	movups	0x130(%rdx), %xmm0
1400d53aa: 0f 29 85 20 09 00 00        	movaps	%xmm0, 0x920(%rbp)
1400d53b1: 0f 10 82 20 01 00 00        	movups	0x120(%rdx), %xmm0
1400d53b8: 0f 29 85 10 09 00 00        	movaps	%xmm0, 0x910(%rbp)
1400d53bf: 0f 10 82 e0 00 00 00        	movups	0xe0(%rdx), %xmm0
1400d53c6: 0f 10 8a f0 00 00 00        	movups	0xf0(%rdx), %xmm1
1400d53cd: 0f 10 92 00 01 00 00        	movups	0x100(%rdx), %xmm2
1400d53d4: 0f 10 9a 10 01 00 00        	movups	0x110(%rdx), %xmm3
1400d53db: 0f 29 9d 00 09 00 00        	movaps	%xmm3, 0x900(%rbp)
1400d53e2: 0f 29 95 f0 08 00 00        	movaps	%xmm2, 0x8f0(%rbp)
1400d53e9: 0f 29 8d e0 08 00 00        	movaps	%xmm1, 0x8e0(%rbp)
1400d53f0: 0f 29 85 d0 08 00 00        	movaps	%xmm0, 0x8d0(%rbp)
1400d53f7: 48 81 c2 50 01 00 00        	addq	$0x150, %rdx            # imm = 0x150
1400d53fe: 48 8d 8d 30 06 00 00        	leaq	0x630(%rbp), %rcx
1400d5405: 41 b8 e0 00 00 00           	movl	$0xe0, %r8d
1400d540b: e8 f0 e7 84 02              	callq	0x142923c00 <.text+0x2922c00>
1400d5410: 0f b6 05 f2 f7 55 03        	movzbl	0x355f7f2(%rip), %eax   # 0x143634c09
1400d5417: b9 50 08 00 00              	movl	$0x850, %ecx            # imm = 0x850
1400d541c: ba 08 00 00 00              	movl	$0x8, %edx
1400d5421: e8 ea e7 53 00              	callq	0x140613c10 <.text+0x612c10>
1400d5426: 48 89 85 28 0a 00 00        	movq	%rax, 0xa28(%rbp)
1400d542d: 48 85 c0                    	testq	%rax, %rax
1400d5430: 0f 84 ce 0b 00 00           	je	0x1400d6004 <.text+0xd5004>
1400d5436: 48 8d 47 70                 	leaq	0x70(%rdi), %rax
1400d543a: 0f 10 47 60                 	movups	0x60(%rdi), %xmm0
1400d543e: 0f 29 85 c0 08 00 00        	movaps	%xmm0, 0x8c0(%rbp)
1400d5445: 0f 10 47 50                 	movups	0x50(%rdi), %xmm0
1400d5449: 0f 29 85 b0 08 00 00        	movaps	%xmm0, 0x8b0(%rbp)
1400d5450: 0f 10 47 40                 	movups	0x40(%rdi), %xmm0
1400d5454: 0f 29 85 a0 08 00 00        	movaps	%xmm0, 0x8a0(%rbp)
1400d545b: 0f 10 07                    	movups	(%rdi), %xmm0
1400d545e: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
1400d5462: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
1400d5466: 0f 10 5f 30                 	movups	0x30(%rdi), %xmm3
1400d546a: 0f 29 9d 90 08 00 00        	movaps	%xmm3, 0x890(%rbp)
1400d5471: 0f 29 95 80 08 00 00        	movaps	%xmm2, 0x880(%rbp)
1400d5478: 0f 29 8d 70 08 00 00        	movaps	%xmm1, 0x870(%rbp)
1400d547f: 0f 29 85 60 08 00 00        	movaps	%xmm0, 0x860(%rbp)
1400d5486: 0f 10 40 60                 	movups	0x60(%rax), %xmm0
1400d548a: 0f 29 85 50 08 00 00        	movaps	%xmm0, 0x850(%rbp)
1400d5491: 0f 10 40 50                 	movups	0x50(%rax), %xmm0
1400d5495: 0f 29 85 40 08 00 00        	movaps	%xmm0, 0x840(%rbp)
1400d549c: 0f 10 40 40                 	movups	0x40(%rax), %xmm0
1400d54a0: 0f 29 85 30 08 00 00        	movaps	%xmm0, 0x830(%rbp)
1400d54a7: 0f 10 00                    	movups	(%rax), %xmm0
1400d54aa: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1400d54ae: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1400d54b2: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
1400d54b6: 0f 29 9d 20 08 00 00        	movaps	%xmm3, 0x820(%rbp)
1400d54bd: 0f 29 95 10 08 00 00        	movaps	%xmm2, 0x810(%rbp)
1400d54c4: 0f 29 8d 00 08 00 00        	movaps	%xmm1, 0x800(%rbp)
1400d54cb: 0f 29 85 f0 07 00 00        	movaps	%xmm0, 0x7f0(%rbp)
1400d54d2: c6 85 3d 0a 00 00 01        	movb	$0x1, 0xa3d(%rbp)
1400d54d9: c6 85 3c 0a 00 00 01        	movb	$0x1, 0xa3c(%rbp)
1400d54e0: 48 8d 8d c0 05 00 00        	leaq	0x5c0(%rbp), %rcx
1400d54e7: e8 14 0d ff ff              	callq	0x1400c6200 <.text+0xc5200>
1400d54ec: 0f 28 85 30 09 00 00        	movaps	0x930(%rbp), %xmm0
1400d54f3: 0f 29 85 e0 07 00 00        	movaps	%xmm0, 0x7e0(%rbp)
1400d54fa: 0f 28 85 20 09 00 00        	movaps	0x920(%rbp), %xmm0
1400d5501: 0f 29 85 d0 07 00 00        	movaps	%xmm0, 0x7d0(%rbp)
1400d5508: 0f 28 85 10 09 00 00        	movaps	0x910(%rbp), %xmm0
1400d550f: 0f 29 85 c0 07 00 00        	movaps	%xmm0, 0x7c0(%rbp)
1400d5516: 0f 28 85 d0 08 00 00        	movaps	0x8d0(%rbp), %xmm0
1400d551d: 0f 28 8d e0 08 00 00        	movaps	0x8e0(%rbp), %xmm1
1400d5524: 0f 28 95 f0 08 00 00        	movaps	0x8f0(%rbp), %xmm2
1400d552b: 0f 28 9d 00 09 00 00        	movaps	0x900(%rbp), %xmm3
1400d5532: 0f 29 9d b0 07 00 00        	movaps	%xmm3, 0x7b0(%rbp)
1400d5539: 0f 29 95 a0 07 00 00        	movaps	%xmm2, 0x7a0(%rbp)
1400d5540: 0f 29 8d 90 07 00 00        	movaps	%xmm1, 0x790(%rbp)
1400d5547: 0f 29 85 80 07 00 00        	movaps	%xmm0, 0x780(%rbp)
1400d554e: c6 85 3b 0a 00 00 01        	movb	$0x1, 0xa3b(%rbp)
1400d5555: 48 8d 8d 50 05 00 00        	leaq	0x550(%rbp), %rcx
1400d555c: e8 ef 49 31 00              	callq	0x1403e9f50 <.text+0x3e8f50>
1400d5561: c6 85 3a 0a 00 00 01        	movb	$0x1, 0xa3a(%rbp)
1400d5568: 48 8d 8d e0 04 00 00        	leaq	0x4e0(%rbp), %rcx
1400d556f: e8 0c 10 30 00              	callq	0x1403d6580 <.text+0x3d5580>
1400d5574: c6 85 39 0a 00 00 01        	movb	$0x1, 0xa39(%rbp)
1400d557b: 48 8d 8d 70 04 00 00        	leaq	0x470(%rbp), %rcx
1400d5582: e8 99 46 45 00              	callq	0x140529c20 <.text+0x528c20>
1400d5587: c6 85 38 0a 00 00 01        	movb	$0x1, 0xa38(%rbp)
1400d558e: 48 8d 8d 00 04 00 00        	leaq	0x400(%rbp), %rcx
1400d5595: e8 e6 14 30 00              	callq	0x1403d6a80 <.text+0x3d5a80>
1400d559a: c6 85 37 0a 00 00 01        	movb	$0x1, 0xa37(%rbp)
1400d55a1: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
1400d55a8: e8 e3 f7 17 00              	callq	0x140254d90 <.text+0x253d90>
1400d55ad: c6 85 36 0a 00 00 01        	movb	$0x1, 0xa36(%rbp)
1400d55b4: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
1400d55bb: e8 10 13 30 00              	callq	0x1403d68d0 <.text+0x3d58d0>
1400d55c0: c6 85 35 0a 00 00 01        	movb	$0x1, 0xa35(%rbp)
1400d55c7: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
1400d55ce: e8 8d f9 02 00              	callq	0x140104f60 <.text+0x103f60>
1400d55d3: c6 85 34 0a 00 00 01        	movb	$0x1, 0xa34(%rbp)
1400d55da: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
1400d55e1: e8 fa 08 1b 00              	callq	0x140285ee0 <.text+0x284ee0>
1400d55e6: c6 85 33 0a 00 00 01        	movb	$0x1, 0xa33(%rbp)
1400d55ed: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
1400d55f4: e8 17 fb 02 00              	callq	0x140105110 <.text+0x104110>
1400d55f9: c6 85 32 0a 00 00 01        	movb	$0x1, 0xa32(%rbp)
1400d5600: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1400d5607: e8 74 44 3b 00              	callq	0x140489a80 <.text+0x488a80>
1400d560c: 0f 10 85 90 06 00 00        	movups	0x690(%rbp), %xmm0
1400d5613: 0f 29 85 a0 09 00 00        	movaps	%xmm0, 0x9a0(%rbp)
1400d561a: 0f 10 85 80 06 00 00        	movups	0x680(%rbp), %xmm0
1400d5621: 0f 29 85 90 09 00 00        	movaps	%xmm0, 0x990(%rbp)
1400d5628: 0f 10 85 70 06 00 00        	movups	0x670(%rbp), %xmm0
1400d562f: 0f 29 85 80 09 00 00        	movaps	%xmm0, 0x980(%rbp)
1400d5636: 0f 10 85 30 06 00 00        	movups	0x630(%rbp), %xmm0
1400d563d: 0f 10 8d 40 06 00 00        	movups	0x640(%rbp), %xmm1
1400d5644: 0f 10 95 50 06 00 00        	movups	0x650(%rbp), %xmm2
1400d564b: 0f 10 9d 60 06 00 00        	movups	0x660(%rbp), %xmm3
1400d5652: 0f 29 9d 70 09 00 00        	movaps	%xmm3, 0x970(%rbp)
1400d5659: 0f 29 95 60 09 00 00        	movaps	%xmm2, 0x960(%rbp)
1400d5660: 0f 29 8d 50 09 00 00        	movaps	%xmm1, 0x950(%rbp)
1400d5667: 0f 29 85 40 09 00 00        	movaps	%xmm0, 0x940(%rbp)
1400d566e: 0f 10 85 00 07 00 00        	movups	0x700(%rbp), %xmm0
1400d5675: 0f 29 85 10 0a 00 00        	movaps	%xmm0, 0xa10(%rbp)
1400d567c: 0f 10 85 f0 06 00 00        	movups	0x6f0(%rbp), %xmm0
1400d5683: 0f 29 85 00 0a 00 00        	movaps	%xmm0, 0xa00(%rbp)
1400d568a: 0f 10 85 e0 06 00 00        	movups	0x6e0(%rbp), %xmm0
1400d5691: 0f 29 85 f0 09 00 00        	movaps	%xmm0, 0x9f0(%rbp)
1400d5698: 0f 10 85 d0 06 00 00        	movups	0x6d0(%rbp), %xmm0
1400d569f: 0f 29 85 e0 09 00 00        	movaps	%xmm0, 0x9e0(%rbp)
1400d56a6: 0f 10 85 c0 06 00 00        	movups	0x6c0(%rbp), %xmm0
1400d56ad: 0f 29 85 d0 09 00 00        	movaps	%xmm0, 0x9d0(%rbp)
1400d56b4: 0f 10 85 b0 06 00 00        	movups	0x6b0(%rbp), %xmm0
1400d56bb: 0f 29 85 c0 09 00 00        	movaps	%xmm0, 0x9c0(%rbp)
1400d56c2: 0f 10 85 a0 06 00 00        	movups	0x6a0(%rbp), %xmm0
1400d56c9: 0f 29 85 b0 09 00 00        	movaps	%xmm0, 0x9b0(%rbp)
1400d56d0: 0f b6 05 32 f5 55 03        	movzbl	0x355f532(%rip), %eax   # 0x143634c09
1400d56d7: b9 e0 00 00 00              	movl	$0xe0, %ecx
1400d56dc: ba 08 00 00 00              	movl	$0x8, %edx
1400d56e1: e8 2a e5 53 00              	callq	0x140613c10 <.text+0x612c10>
1400d56e6: 48 85 c0                    	testq	%rax, %rax
1400d56e9: 0f 84 34 09 00 00           	je	0x1400d6023 <.text+0xd5023>
1400d56ef: 48 8d 8d a0 06 00 00        	leaq	0x6a0(%rbp), %rcx
1400d56f6: 0f 10 85 90 06 00 00        	movups	0x690(%rbp), %xmm0
1400d56fd: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1400d5701: 0f 10 85 80 06 00 00        	movups	0x680(%rbp), %xmm0
1400d5708: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1400d570c: 0f 10 85 70 06 00 00        	movups	0x670(%rbp), %xmm0
1400d5713: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1400d5717: 0f 10 85 30 06 00 00        	movups	0x630(%rbp), %xmm0
1400d571e: 0f 10 8d 40 06 00 00        	movups	0x640(%rbp), %xmm1
1400d5725: 0f 10 95 50 06 00 00        	movups	0x650(%rbp), %xmm2
1400d572c: 0f 10 9d 60 06 00 00        	movups	0x660(%rbp), %xmm3
1400d5733: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1400d5737: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1400d573b: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1400d573f: 0f 11 00                    	movups	%xmm0, (%rax)
1400d5742: 0f 10 01                    	movups	(%rcx), %xmm0
1400d5745: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
1400d5749: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
1400d574d: 0f 10 59 30                 	movups	0x30(%rcx), %xmm3
1400d5751: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
1400d5755: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
1400d575c: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
1400d5763: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
1400d576a: 0f 10 41 40                 	movups	0x40(%rcx), %xmm0
1400d576e: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
1400d5775: 0f 10 41 50                 	movups	0x50(%rcx), %xmm0
1400d5779: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
1400d5780: 0f 10 41 60                 	movups	0x60(%rcx), %xmm0
1400d5784: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
1400d578b: 48 c7 85 18 07 00 00 02 00 00 00    	movq	$0x2, 0x718(%rbp)
1400d5796: 48 89 85 20 07 00 00        	movq	%rax, 0x720(%rbp)
1400d579d: 48 c7 85 28 07 00 00 02 00 00 00    	movq	$0x2, 0x728(%rbp)
1400d57a8: 48 c7 85 30 07 00 00 00 00 00 00    	movq	$0x0, 0x730(%rbp)
1400d57b3: 48 c7 85 38 07 00 00 08 00 00 00    	movq	$0x8, 0x738(%rbp)
1400d57be: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400d57c1: 0f 11 85 40 07 00 00        	movups	%xmm0, 0x740(%rbp)
1400d57c8: 48 bf 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rdi # imm = 0x8000000000000000
1400d57d2: 48 89 bd 10 07 00 00        	movq	%rdi, 0x710(%rbp)
1400d57d9: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1400d57e0: e8 eb 91 20 00              	callq	0x1402de9d0 <.text+0x2dd9d0>
1400d57e5: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1400d57ec: e8 9f f5 17 00              	callq	0x140254d90 <.text+0x253d90>
1400d57f1: 48 8d 8d 40 09 00 00        	leaq	0x940(%rbp), %rcx
1400d57f8: e8 13 f8 00 00              	callq	0x1400e5010 <.text+0xe4010>
1400d57fd: 48 8d 8d b0 09 00 00        	leaq	0x9b0(%rbp), %rcx
1400d5804: e8 57 e0 2e 00              	callq	0x1403c3860 <.text+0x3c2860>
1400d5809: 0f 28 85 c0 08 00 00        	movaps	0x8c0(%rbp), %xmm0
1400d5810: 48 8b 85 28 0a 00 00        	movq	0xa28(%rbp), %rax
1400d5817: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1400d581b: 0f 28 85 b0 08 00 00        	movaps	0x8b0(%rbp), %xmm0
1400d5822: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1400d5826: 0f 28 85 a0 08 00 00        	movaps	0x8a0(%rbp), %xmm0
1400d582d: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1400d5831: 0f 28 85 60 08 00 00        	movaps	0x860(%rbp), %xmm0
1400d5838: 0f 28 8d 70 08 00 00        	movaps	0x870(%rbp), %xmm1
1400d583f: 0f 28 95 80 08 00 00        	movaps	0x880(%rbp), %xmm2
1400d5846: 0f 28 9d 90 08 00 00        	movaps	0x890(%rbp), %xmm3
1400d584d: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1400d5851: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1400d5855: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1400d5859: 0f 11 00                    	movups	%xmm0, (%rax)
1400d585c: 0f 28 85 f0 07 00 00        	movaps	0x7f0(%rbp), %xmm0
1400d5863: 0f 28 8d 00 08 00 00        	movaps	0x800(%rbp), %xmm1
1400d586a: 0f 28 95 10 08 00 00        	movaps	0x810(%rbp), %xmm2
1400d5871: 0f 28 9d 20 08 00 00        	movaps	0x820(%rbp), %xmm3
1400d5878: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
1400d587c: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
1400d5883: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
1400d588a: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
1400d5891: 0f 28 85 30 08 00 00        	movaps	0x830(%rbp), %xmm0
1400d5898: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
1400d589f: 0f 28 85 40 08 00 00        	movaps	0x840(%rbp), %xmm0
1400d58a6: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
1400d58ad: 0f 28 85 50 08 00 00        	movaps	0x850(%rbp), %xmm0
1400d58b4: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
1400d58bb: 0f 10 85 20 06 00 00        	movups	0x620(%rbp), %xmm0
1400d58c2: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
1400d58c9: 0f 10 85 10 06 00 00        	movups	0x610(%rbp), %xmm0
1400d58d0: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
1400d58d7: 0f 10 85 00 06 00 00        	movups	0x600(%rbp), %xmm0
1400d58de: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
1400d58e5: 0f 10 85 c0 05 00 00        	movups	0x5c0(%rbp), %xmm0
1400d58ec: 0f 10 8d d0 05 00 00        	movups	0x5d0(%rbp), %xmm1
1400d58f3: 0f 10 95 e0 05 00 00        	movups	0x5e0(%rbp), %xmm2
1400d58fa: 0f 10 9d f0 05 00 00        	movups	0x5f0(%rbp), %xmm3
1400d5901: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
1400d5908: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
1400d590f: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
1400d5916: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
1400d591d: 0f 28 85 80 07 00 00        	movaps	0x780(%rbp), %xmm0
1400d5924: 0f 28 8d 90 07 00 00        	movaps	0x790(%rbp), %xmm1
1400d592b: 0f 28 95 a0 07 00 00        	movaps	0x7a0(%rbp), %xmm2
1400d5932: 0f 28 9d b0 07 00 00        	movaps	0x7b0(%rbp), %xmm3
1400d5939: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
1400d5940: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
1400d5947: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
1400d594e: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
1400d5955: 0f 28 85 c0 07 00 00        	movaps	0x7c0(%rbp), %xmm0
1400d595c: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
1400d5963: 0f 28 85 d0 07 00 00        	movaps	0x7d0(%rbp), %xmm0
1400d596a: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
1400d5971: 0f 28 85 e0 07 00 00        	movaps	0x7e0(%rbp), %xmm0
1400d5978: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
1400d597f: 0f 10 85 50 05 00 00        	movups	0x550(%rbp), %xmm0
1400d5986: 0f 10 8d 60 05 00 00        	movups	0x560(%rbp), %xmm1
1400d598d: 0f 10 95 70 05 00 00        	movups	0x570(%rbp), %xmm2
1400d5994: 0f 10 9d 80 05 00 00        	movups	0x580(%rbp), %xmm3
1400d599b: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
1400d59a2: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
1400d59a9: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
1400d59b0: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
1400d59b7: 0f 10 85 90 05 00 00        	movups	0x590(%rbp), %xmm0
1400d59be: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
1400d59c5: 0f 10 85 b0 05 00 00        	movups	0x5b0(%rbp), %xmm0
1400d59cc: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
1400d59d3: 0f 10 85 a0 05 00 00        	movups	0x5a0(%rbp), %xmm0
1400d59da: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
1400d59e1: 0f 10 85 40 05 00 00        	movups	0x540(%rbp), %xmm0
1400d59e8: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
1400d59ef: 0f 10 85 30 05 00 00        	movups	0x530(%rbp), %xmm0
1400d59f6: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
1400d59fd: 0f 10 85 20 05 00 00        	movups	0x520(%rbp), %xmm0
1400d5a04: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
1400d5a0b: 0f 10 85 e0 04 00 00        	movups	0x4e0(%rbp), %xmm0
1400d5a12: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
1400d5a19: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
1400d5a20: 0f 10 9d 10 05 00 00        	movups	0x510(%rbp), %xmm3
1400d5a27: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
1400d5a2e: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
1400d5a35: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
1400d5a3c: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
1400d5a43: 0f 10 85 70 04 00 00        	movups	0x470(%rbp), %xmm0
1400d5a4a: 0f 10 8d 80 04 00 00        	movups	0x480(%rbp), %xmm1
1400d5a51: 0f 10 95 90 04 00 00        	movups	0x490(%rbp), %xmm2
1400d5a58: 0f 10 9d a0 04 00 00        	movups	0x4a0(%rbp), %xmm3
1400d5a5f: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
1400d5a66: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
1400d5a6d: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
1400d5a74: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
1400d5a7b: 0f 10 85 b0 04 00 00        	movups	0x4b0(%rbp), %xmm0
1400d5a82: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
1400d5a89: 0f 10 85 c0 04 00 00        	movups	0x4c0(%rbp), %xmm0
1400d5a90: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
1400d5a97: 0f 10 85 d0 04 00 00        	movups	0x4d0(%rbp), %xmm0
1400d5a9e: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
1400d5aa5: 0f 10 85 00 04 00 00        	movups	0x400(%rbp), %xmm0
1400d5aac: 0f 10 8d 10 04 00 00        	movups	0x410(%rbp), %xmm1
1400d5ab3: 0f 10 95 20 04 00 00        	movups	0x420(%rbp), %xmm2
1400d5aba: 0f 10 9d 30 04 00 00        	movups	0x430(%rbp), %xmm3
1400d5ac1: 0f 11 80 10 03 00 00        	movups	%xmm0, 0x310(%rax)
1400d5ac8: 0f 11 88 20 03 00 00        	movups	%xmm1, 0x320(%rax)
1400d5acf: 0f 11 90 30 03 00 00        	movups	%xmm2, 0x330(%rax)
1400d5ad6: 0f 11 98 40 03 00 00        	movups	%xmm3, 0x340(%rax)
1400d5add: 0f 10 85 40 04 00 00        	movups	0x440(%rbp), %xmm0
1400d5ae4: 0f 11 80 50 03 00 00        	movups	%xmm0, 0x350(%rax)
1400d5aeb: 0f 10 85 50 04 00 00        	movups	0x450(%rbp), %xmm0
1400d5af2: 0f 11 80 60 03 00 00        	movups	%xmm0, 0x360(%rax)
1400d5af9: 0f 10 85 60 04 00 00        	movups	0x460(%rbp), %xmm0
1400d5b00: 0f 11 80 70 03 00 00        	movups	%xmm0, 0x370(%rax)
1400d5b07: 0f 10 85 90 03 00 00        	movups	0x390(%rbp), %xmm0
1400d5b0e: 0f 10 8d a0 03 00 00        	movups	0x3a0(%rbp), %xmm1
1400d5b15: 0f 10 95 b0 03 00 00        	movups	0x3b0(%rbp), %xmm2
1400d5b1c: 0f 10 9d c0 03 00 00        	movups	0x3c0(%rbp), %xmm3
1400d5b23: 0f 11 80 80 03 00 00        	movups	%xmm0, 0x380(%rax)
1400d5b2a: 0f 11 88 90 03 00 00        	movups	%xmm1, 0x390(%rax)
1400d5b31: 0f 11 90 a0 03 00 00        	movups	%xmm2, 0x3a0(%rax)
1400d5b38: 0f 11 98 b0 03 00 00        	movups	%xmm3, 0x3b0(%rax)
1400d5b3f: 0f 10 85 d0 03 00 00        	movups	0x3d0(%rbp), %xmm0
1400d5b46: 0f 11 80 c0 03 00 00        	movups	%xmm0, 0x3c0(%rax)
1400d5b4d: 0f 10 85 e0 03 00 00        	movups	0x3e0(%rbp), %xmm0
1400d5b54: 0f 11 80 d0 03 00 00        	movups	%xmm0, 0x3d0(%rax)
1400d5b5b: 0f 10 85 f0 03 00 00        	movups	0x3f0(%rbp), %xmm0
1400d5b62: 0f 11 80 e0 03 00 00        	movups	%xmm0, 0x3e0(%rax)
1400d5b69: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
1400d5b70: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
1400d5b77: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
1400d5b7e: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
1400d5b85: 0f 11 80 f0 03 00 00        	movups	%xmm0, 0x3f0(%rax)
1400d5b8c: 0f 11 88 00 04 00 00        	movups	%xmm1, 0x400(%rax)
1400d5b93: 0f 11 90 10 04 00 00        	movups	%xmm2, 0x410(%rax)
1400d5b9a: 0f 11 98 20 04 00 00        	movups	%xmm3, 0x420(%rax)
1400d5ba1: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
1400d5ba8: 0f 11 80 30 04 00 00        	movups	%xmm0, 0x430(%rax)
1400d5baf: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
1400d5bb6: 0f 11 80 40 04 00 00        	movups	%xmm0, 0x440(%rax)
1400d5bbd: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
1400d5bc4: 0f 11 80 50 04 00 00        	movups	%xmm0, 0x450(%rax)
1400d5bcb: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
1400d5bd2: 0f 10 8d c0 02 00 00        	movups	0x2c0(%rbp), %xmm1
1400d5bd9: 0f 10 95 d0 02 00 00        	movups	0x2d0(%rbp), %xmm2
1400d5be0: 0f 10 9d e0 02 00 00        	movups	0x2e0(%rbp), %xmm3
1400d5be7: 0f 11 80 60 04 00 00        	movups	%xmm0, 0x460(%rax)
1400d5bee: 0f 11 88 70 04 00 00        	movups	%xmm1, 0x470(%rax)
1400d5bf5: 0f 11 90 80 04 00 00        	movups	%xmm2, 0x480(%rax)
1400d5bfc: 0f 11 98 90 04 00 00        	movups	%xmm3, 0x490(%rax)
1400d5c03: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
1400d5c0a: 0f 11 80 a0 04 00 00        	movups	%xmm0, 0x4a0(%rax)
1400d5c11: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
1400d5c18: 0f 11 80 b0 04 00 00        	movups	%xmm0, 0x4b0(%rax)
1400d5c1f: 0f 10 85 10 03 00 00        	movups	0x310(%rbp), %xmm0
1400d5c26: 0f 11 80 c0 04 00 00        	movups	%xmm0, 0x4c0(%rax)
1400d5c2d: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
1400d5c34: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
1400d5c3b: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
1400d5c42: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
1400d5c49: 0f 11 80 d0 04 00 00        	movups	%xmm0, 0x4d0(%rax)
1400d5c50: 0f 11 88 e0 04 00 00        	movups	%xmm1, 0x4e0(%rax)
1400d5c57: 0f 11 90 f0 04 00 00        	movups	%xmm2, 0x4f0(%rax)
1400d5c5e: 0f 11 98 00 05 00 00        	movups	%xmm3, 0x500(%rax)
1400d5c65: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
1400d5c6c: 0f 11 80 10 05 00 00        	movups	%xmm0, 0x510(%rax)
1400d5c73: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
1400d5c7a: 0f 11 80 20 05 00 00        	movups	%xmm0, 0x520(%rax)
1400d5c81: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
1400d5c88: 0f 11 80 30 05 00 00        	movups	%xmm0, 0x530(%rax)
1400d5c8f: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
1400d5c96: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
1400d5c9d: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
1400d5ca4: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
1400d5cab: 0f 11 80 40 05 00 00        	movups	%xmm0, 0x540(%rax)
1400d5cb2: 0f 11 88 50 05 00 00        	movups	%xmm1, 0x550(%rax)
1400d5cb9: 0f 11 90 60 05 00 00        	movups	%xmm2, 0x560(%rax)
1400d5cc0: 0f 11 98 70 05 00 00        	movups	%xmm3, 0x570(%rax)
1400d5cc7: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
1400d5cce: 0f 11 80 80 05 00 00        	movups	%xmm0, 0x580(%rax)
1400d5cd5: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
1400d5cdc: 0f 11 80 90 05 00 00        	movups	%xmm0, 0x590(%rax)
1400d5ce3: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
1400d5cea: 0f 11 80 a0 05 00 00        	movups	%xmm0, 0x5a0(%rax)
1400d5cf1: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
1400d5cf8: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
1400d5cff: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
1400d5d06: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
1400d5d0d: 0f 11 80 b0 05 00 00        	movups	%xmm0, 0x5b0(%rax)
1400d5d14: 0f 11 88 c0 05 00 00        	movups	%xmm1, 0x5c0(%rax)
1400d5d1b: 0f 11 90 d0 05 00 00        	movups	%xmm2, 0x5d0(%rax)
1400d5d22: 0f 11 98 e0 05 00 00        	movups	%xmm3, 0x5e0(%rax)
1400d5d29: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1400d5d30: 0f 11 80 f0 05 00 00        	movups	%xmm0, 0x5f0(%rax)
1400d5d37: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
1400d5d3e: 0f 11 80 00 06 00 00        	movups	%xmm0, 0x600(%rax)
1400d5d45: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
1400d5d4c: 0f 11 80 10 06 00 00        	movups	%xmm0, 0x610(%rax)
1400d5d53: 0f 10 85 10 07 00 00        	movups	0x710(%rbp), %xmm0
1400d5d5a: 0f 10 8d 20 07 00 00        	movups	0x720(%rbp), %xmm1
1400d5d61: 0f 10 95 30 07 00 00        	movups	0x730(%rbp), %xmm2
1400d5d68: 0f 10 9d 40 07 00 00        	movups	0x740(%rbp), %xmm3
1400d5d6f: 0f 11 80 20 06 00 00        	movups	%xmm0, 0x620(%rax)
1400d5d76: 0f 11 88 30 06 00 00        	movups	%xmm1, 0x630(%rax)
1400d5d7d: 0f 11 90 40 06 00 00        	movups	%xmm2, 0x640(%rax)
1400d5d84: 0f 11 98 50 06 00 00        	movups	%xmm3, 0x650(%rax)
1400d5d8b: 0f 10 85 50 07 00 00        	movups	0x750(%rbp), %xmm0
1400d5d92: 0f 11 80 60 06 00 00        	movups	%xmm0, 0x660(%rax)
1400d5d99: 0f 10 85 60 07 00 00        	movups	0x760(%rbp), %xmm0
1400d5da0: 0f 11 80 70 06 00 00        	movups	%xmm0, 0x670(%rax)
1400d5da7: 0f 10 85 70 07 00 00        	movups	0x770(%rbp), %xmm0
1400d5dae: 0f 11 80 80 06 00 00        	movups	%xmm0, 0x680(%rax)
1400d5db5: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1400d5dbc: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
1400d5dc3: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
1400d5dca: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
1400d5dd1: 0f 11 80 90 06 00 00        	movups	%xmm0, 0x690(%rax)
1400d5dd8: 0f 11 88 a0 06 00 00        	movups	%xmm1, 0x6a0(%rax)
1400d5ddf: 0f 11 90 b0 06 00 00        	movups	%xmm2, 0x6b0(%rax)
1400d5de6: 0f 11 98 c0 06 00 00        	movups	%xmm3, 0x6c0(%rax)
1400d5ded: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
1400d5df4: 0f 11 80 d0 06 00 00        	movups	%xmm0, 0x6d0(%rax)
1400d5dfb: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1400d5e02: 0f 11 80 e0 06 00 00        	movups	%xmm0, 0x6e0(%rax)
1400d5e09: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1400d5e10: 0f 11 80 f0 06 00 00        	movups	%xmm0, 0x6f0(%rax)
1400d5e17: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1400d5e1e: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
1400d5e25: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
1400d5e2c: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
1400d5e33: 0f 11 80 00 07 00 00        	movups	%xmm0, 0x700(%rax)
1400d5e3a: 0f 11 88 10 07 00 00        	movups	%xmm1, 0x710(%rax)
1400d5e41: 0f 11 90 20 07 00 00        	movups	%xmm2, 0x720(%rax)
1400d5e48: 0f 11 98 30 07 00 00        	movups	%xmm3, 0x730(%rax)
1400d5e4f: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1400d5e56: 0f 11 80 40 07 00 00        	movups	%xmm0, 0x740(%rax)
1400d5e5d: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
1400d5e64: 0f 11 80 50 07 00 00        	movups	%xmm0, 0x750(%rax)
1400d5e6b: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
1400d5e72: 0f 11 80 60 07 00 00        	movups	%xmm0, 0x760(%rax)
1400d5e79: 0f 10 85 40 09 00 00        	movups	0x940(%rbp), %xmm0
1400d5e80: 0f 10 8d 50 09 00 00        	movups	0x950(%rbp), %xmm1
1400d5e87: 0f 10 95 60 09 00 00        	movups	0x960(%rbp), %xmm2
1400d5e8e: 0f 10 9d 70 09 00 00        	movups	0x970(%rbp), %xmm3
1400d5e95: 0f 11 80 70 07 00 00        	movups	%xmm0, 0x770(%rax)
1400d5e9c: 0f 11 88 80 07 00 00        	movups	%xmm1, 0x780(%rax)
1400d5ea3: 0f 11 90 90 07 00 00        	movups	%xmm2, 0x790(%rax)
1400d5eaa: 0f 11 98 a0 07 00 00        	movups	%xmm3, 0x7a0(%rax)
1400d5eb1: 0f 10 85 80 09 00 00        	movups	0x980(%rbp), %xmm0
1400d5eb8: 0f 11 80 b0 07 00 00        	movups	%xmm0, 0x7b0(%rax)
1400d5ebf: 0f 10 85 90 09 00 00        	movups	0x990(%rbp), %xmm0
1400d5ec6: 0f 11 80 c0 07 00 00        	movups	%xmm0, 0x7c0(%rax)
1400d5ecd: 0f 10 85 a0 09 00 00        	movups	0x9a0(%rbp), %xmm0
1400d5ed4: 0f 11 80 d0 07 00 00        	movups	%xmm0, 0x7d0(%rax)
1400d5edb: 0f 10 85 b0 09 00 00        	movups	0x9b0(%rbp), %xmm0
1400d5ee2: 0f 10 8d c0 09 00 00        	movups	0x9c0(%rbp), %xmm1
1400d5ee9: 0f 10 95 d0 09 00 00        	movups	0x9d0(%rbp), %xmm2
1400d5ef0: 0f 10 9d e0 09 00 00        	movups	0x9e0(%rbp), %xmm3
1400d5ef7: 0f 11 80 e0 07 00 00        	movups	%xmm0, 0x7e0(%rax)
1400d5efe: 0f 11 88 f0 07 00 00        	movups	%xmm1, 0x7f0(%rax)
1400d5f05: 0f 11 90 00 08 00 00        	movups	%xmm2, 0x800(%rax)
1400d5f0c: 0f 11 98 10 08 00 00        	movups	%xmm3, 0x810(%rax)
1400d5f13: 0f 10 85 f0 09 00 00        	movups	0x9f0(%rbp), %xmm0
1400d5f1a: 0f 11 80 20 08 00 00        	movups	%xmm0, 0x820(%rax)
1400d5f21: 0f 10 85 00 0a 00 00        	movups	0xa00(%rbp), %xmm0
1400d5f28: 0f 11 80 30 08 00 00        	movups	%xmm0, 0x830(%rax)
1400d5f2f: 0f 10 85 10 0a 00 00        	movups	0xa10(%rbp), %xmm0
1400d5f36: 0f 11 80 40 08 00 00        	movups	%xmm0, 0x840(%rax)
1400d5f3d: 48 c7 85 b8 09 00 00 13 00 00 00    	movq	$0x13, 0x9b8(%rbp)
1400d5f48: 48 89 85 c0 09 00 00        	movq	%rax, 0x9c0(%rbp)
1400d5f4f: 48 c7 85 c8 09 00 00 13 00 00 00    	movq	$0x13, 0x9c8(%rbp)
1400d5f5a: 48 c7 85 d0 09 00 00 00 00 00 00    	movq	$0x0, 0x9d0(%rbp)
1400d5f65: 48 c7 85 d8 09 00 00 08 00 00 00    	movq	$0x8, 0x9d8(%rbp)
1400d5f70: 48 8d 8d e8 09 00 00        	leaq	0x9e8(%rbp), %rcx
1400d5f77: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400d5f7a: 0f 11 85 e0 09 00 00        	movups	%xmm0, 0x9e0(%rbp)
1400d5f81: 48 8b 85 50 09 00 00        	movq	0x950(%rbp), %rax
1400d5f88: 48 89 85 00 0a 00 00        	movq	%rax, 0xa00(%rbp)
1400d5f8f: 0f 10 85 40 09 00 00        	movups	0x940(%rbp), %xmm0
1400d5f96: 0f 11 85 f0 09 00 00        	movups	%xmm0, 0x9f0(%rbp)
1400d5f9d: 48 89 bd b0 09 00 00        	movq	%rdi, 0x9b0(%rbp)
1400d5fa4: e8 77 9a c5 00              	callq	0x140d2fa20 <.text+0xd2ea20>
1400d5fa9: 0f 10 85 10 0a 00 00        	movups	0xa10(%rbp), %xmm0
1400d5fb0: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
1400d5fb4: 0f 10 85 00 0a 00 00        	movups	0xa00(%rbp), %xmm0
1400d5fbb: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
1400d5fbf: 0f 10 85 f0 09 00 00        	movups	0x9f0(%rbp), %xmm0
1400d5fc6: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
1400d5fca: 0f 10 85 b0 09 00 00        	movups	0x9b0(%rbp), %xmm0
1400d5fd1: 0f 10 8d c0 09 00 00        	movups	0x9c0(%rbp), %xmm1
1400d5fd8: 0f 10 95 d0 09 00 00        	movups	0x9d0(%rbp), %xmm2
1400d5fdf: 0f 10 9d e0 09 00 00        	movups	0x9e0(%rbp), %xmm3
1400d5fe6: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
1400d5fea: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
1400d5fee: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
1400d5ff2: 0f 11 06                    	movups	%xmm0, (%rsi)
1400d5ff5: 48 89 f0                    	movq	%rsi, %rax
1400d5ff8: 48 81 c4 c8 0a 00 00        	addq	$0xac8, %rsp            # imm = 0xAC8
1400d5fff: 5b                          	popq	%rbx
1400d6000: 5f                          	popq	%rdi
1400d6001: 5e                          	popq	%rsi
1400d6002: 5d                          	popq	%rbp
1400d6003: c3                          	retq
1400d6004: c6 85 3f 0a 00 00 01        	movb	$0x1, 0xa3f(%rbp)
1400d600b: c6 85 3e 0a 00 00 01        	movb	$0x1, 0xa3e(%rbp)
1400d6012: b9 08 00 00 00              	movl	$0x8, %ecx
1400d6017: ba 50 08 00 00              	movl	$0x850, %edx            # imm = 0x850
1400d601c: e8 df 2f 80 02              	callq	0x1428d9000 <.text+0x28d8000>
1400d6021: eb 0f                       	jmp	0x1400d6032 <.text+0xd5032>
1400d6023: b9 08 00 00 00              	movl	$0x8, %ecx
1400d6028: ba e0 00 00 00              	movl	$0xe0, %edx
1400d602d: e8 ce 2f 80 02              	callq	0x1428d9000 <.text+0x28d8000>
