
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141400220: 41 56                       	pushq	%r14
141400222: 56                          	pushq	%rsi
141400223: 57                          	pushq	%rdi
141400224: 53                          	pushq	%rbx
141400225: 48 83 ec 48                 	subq	$0x48, %rsp
141400229: 48 89 ce                    	movq	%rcx, %rsi
14140022c: 48 8b 01                    	movq	(%rcx), %rax
14140022f: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
141400239: 48 01 c1                    	addq	%rax, %rcx
14140023c: 45 31 c0                    	xorl	%r8d, %r8d
14140023f: 49 b9 10 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff0, %r9 # imm = 0x8000000000000010
141400249: 4c 39 c8                    	cmpq	%r9, %rax
14140024c: 4c 0f 4c c1                 	cmovlq	%rcx, %r8
141400250: 48 89 d7                    	movq	%rdx, %rdi
141400253: 48 8d 05 82 b8 85 01        	leaq	0x185b882(%rip), %rax   # 0x142c5badc
14140025a: 4a 63 0c 80                 	movslq	(%rax,%r8,4), %rcx
14140025e: 48 01 c1                    	addq	%rax, %rcx
141400261: ff e1                       	jmpq	*%rcx
141400263: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400267: 48 8b 1f                    	movq	(%rdi), %rbx
14140026a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140026e: 48 39 03                    	cmpq	%rax, (%rbx)
141400271: 0f 84 70 11 00 00           	je	0x1414013e7 <.text+0x14003e7>
141400277: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14140027b: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14140027f: 48 ff c0                    	incq	%rax
141400282: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400286: 4c 8d 05 0b b3 85 01        	leaq	0x185b30b(%rip), %r8    # 0x142c5b598
14140028d: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
141400293: 48 89 f9                    	movq	%rdi, %rcx
141400296: e8 85 0a 18 00              	callq	0x141580d20 <.text+0x157fd20>
14140029b: 48 85 c0                    	testq	%rax, %rax
14140029e: 0f 85 0e 0f 00 00           	jne	0x1414011b2 <.text+0x14001b2>
1414002a4: 48 8b 1f                    	movq	(%rdi), %rbx
1414002a7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414002ab: 48 39 13                    	cmpq	%rdx, (%rbx)
1414002ae: 0f 84 dc 14 00 00           	je	0x141401790 <.text+0x1400790>
1414002b4: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414002b8: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1414002bc: 48 ff c2                    	incq	%rdx
1414002bf: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414002c3: 48 8b 1f                    	movq	(%rdi), %rbx
1414002c6: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414002ca: 48 39 13                    	cmpq	%rdx, (%rbx)
1414002cd: 0f 84 e3 14 00 00           	je	0x1414017b6 <.text+0x14007b6>
1414002d3: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414002d7: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414002db: 48 ff c2                    	incq	%rdx
1414002de: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414002e2: 4c 8d 8e e0 00 00 00        	leaq	0xe0(%rsi), %r9
1414002e9: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1414002ee: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1414002f3: 48 8d 15 a9 b2 85 01        	leaq	0x185b2a9(%rip), %rdx   # 0x142c5b5a3
1414002fa: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1414002ff: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400305: e8 a6 47 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
14140030a: 48 85 c0                    	testq	%rax, %rax
14140030d: 0f 85 a7 0e 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400313: 4c 8d 8e e8 00 00 00        	leaq	0xe8(%rsi), %r9
14140031a: 48 8d 15 8c b2 85 01        	leaq	0x185b28c(%rip), %rdx   # 0x142c5b5ad
141400321: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400326: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
14140032c: e8 2f 8c 18 00              	callq	0x141588f60 <.text+0x1587f60>
141400331: 48 85 c0                    	testq	%rax, %rax
141400334: 0f 85 80 0e 00 00           	jne	0x1414011ba <.text+0x14001ba>
14140033a: 48 8d 15 78 b2 85 01        	leaq	0x185b278(%rip), %rdx   # 0x142c5b5b9
141400341: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400346: 41 b8 06 00 00 00           	movl	$0x6, %r8d
14140034c: 49 89 f1                    	movq	%rsi, %r9
14140034f: e8 dc 3d 18 00              	callq	0x141584130 <.text+0x1583130>
141400354: 48 85 c0                    	testq	%rax, %rax
141400357: 0f 85 5d 0e 00 00           	jne	0x1414011ba <.text+0x14001ba>
14140035d: 48 83 c6 70                 	addq	$0x70, %rsi
141400361: 48 8d 15 57 b2 85 01        	leaq	0x185b257(%rip), %rdx   # 0x142c5b5bf
141400368: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140036d: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141400373: 49 89 f1                    	movq	%rsi, %r9
141400376: e8 b5 3d 18 00              	callq	0x141584130 <.text+0x1583130>
14140037b: e9 17 0e 00 00              	jmp	0x141401197 <.text+0x1400197>
141400380: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400384: 48 8b 1f                    	movq	(%rdi), %rbx
141400387: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140038b: 48 39 03                    	cmpq	%rax, (%rbx)
14140038e: 0f 84 7d 0e 00 00           	je	0x141401211 <.text+0x1400211>
141400394: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400398: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14140039c: 48 ff c0                    	incq	%rax
14140039f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1414003a3: 4c 8d 05 71 b3 85 01        	leaq	0x185b371(%rip), %r8    # 0x142c5b71b
1414003aa: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
1414003b0: 48 89 f9                    	movq	%rdi, %rcx
1414003b3: e8 68 09 18 00              	callq	0x141580d20 <.text+0x157fd20>
1414003b8: 48 85 c0                    	testq	%rax, %rax
1414003bb: 0f 85 f1 0d 00 00           	jne	0x1414011b2 <.text+0x14001b2>
1414003c1: 48 8b 1f                    	movq	(%rdi), %rbx
1414003c4: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414003c8: 48 39 13                    	cmpq	%rdx, (%rbx)
1414003cb: 0f 84 5f 11 00 00           	je	0x141401530 <.text+0x1400530>
1414003d1: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414003d5: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1414003d9: 48 ff c2                    	incq	%rdx
1414003dc: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414003e0: 48 8b 1f                    	movq	(%rdi), %rbx
1414003e3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414003e7: 48 39 13                    	cmpq	%rdx, (%rbx)
1414003ea: 0f 84 66 11 00 00           	je	0x141401556 <.text+0x1400556>
1414003f0: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414003f4: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414003f8: 48 ff c2                    	incq	%rdx
1414003fb: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414003ff: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141400403: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141400408: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14140040d: 48 8d 15 8f b1 85 01        	leaq	0x185b18f(%rip), %rdx   # 0x142c5b5a3
141400414: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400419: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14140041f: e8 8c 46 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400424: 48 85 c0                    	testq	%rax, %rax
141400427: 0f 85 8d 0d 00 00           	jne	0x1414011ba <.text+0x14001ba>
14140042d: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
141400431: 48 8d 15 81 b1 85 01        	leaq	0x185b181(%rip), %rdx   # 0x142c5b5b9
141400438: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140043d: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400443: e8 08 7d 18 00              	callq	0x141588150 <.text+0x1587150>
141400448: 48 85 c0                    	testq	%rax, %rax
14140044b: 0f 85 69 0d 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400451: 48 83 c6 11                 	addq	$0x11, %rsi
141400455: 48 8d 15 63 b1 85 01        	leaq	0x185b163(%rip), %rdx   # 0x142c5b5bf
14140045c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400461: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141400467: e9 b8 06 00 00              	jmp	0x141400b24 <.text+0x13ffb24>
14140046c: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400470: 48 8b 1f                    	movq	(%rdi), %rbx
141400473: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400477: 48 39 03                    	cmpq	%rax, (%rbx)
14140047a: 0f 84 c0 0d 00 00           	je	0x141401240 <.text+0x1400240>
141400480: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400484: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400488: 48 ff c0                    	incq	%rax
14140048b: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14140048f: 4c 8d 05 43 b2 85 01        	leaq	0x185b243(%rip), %r8    # 0x142c5b6d9
141400496: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
14140049c: 48 89 f9                    	movq	%rdi, %rcx
14140049f: e8 7c 08 18 00              	callq	0x141580d20 <.text+0x157fd20>
1414004a4: 48 85 c0                    	testq	%rax, %rax
1414004a7: 0f 85 05 0d 00 00           	jne	0x1414011b2 <.text+0x14001b2>
1414004ad: 48 8b 1f                    	movq	(%rdi), %rbx
1414004b0: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414004b4: 48 39 13                    	cmpq	%rdx, (%rbx)
1414004b7: 0f 84 bf 10 00 00           	je	0x14140157c <.text+0x140057c>
1414004bd: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414004c1: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1414004c5: 48 ff c2                    	incq	%rdx
1414004c8: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414004cc: 48 8b 1f                    	movq	(%rdi), %rbx
1414004cf: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414004d3: 48 39 13                    	cmpq	%rdx, (%rbx)
1414004d6: 0f 84 c6 10 00 00           	je	0x1414015a2 <.text+0x14005a2>
1414004dc: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414004e0: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414004e4: 48 ff c2                    	incq	%rdx
1414004e7: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414004eb: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
1414004ef: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1414004f4: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1414004f9: 48 8d 15 a3 b0 85 01        	leaq	0x185b0a3(%rip), %rdx   # 0x142c5b5a3
141400500: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400505: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14140050b: e8 a0 45 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400510: 48 85 c0                    	testq	%rax, %rax
141400513: 0f 85 a1 0c 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400519: 48 8d 46 08                 	leaq	0x8(%rsi), %rax
14140051d: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
141400522: 48 8d 15 bf b1 85 01        	leaq	0x185b1bf(%rip), %rdx   # 0x142c5b6e8
141400529: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140052e: 4c 8d 4c 24 40              	leaq	0x40(%rsp), %r9
141400533: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141400539: e8 b2 74 18 00              	callq	0x1415879f0 <.text+0x15869f0>
14140053e: 48 85 c0                    	testq	%rax, %rax
141400541: 0f 85 73 0c 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400547: 48 83 c6 40                 	addq	$0x40, %rsi
14140054b: 48 89 74 24 40              	movq	%rsi, 0x40(%rsp)
141400550: 48 8d 15 99 b1 85 01        	leaq	0x185b199(%rip), %rdx   # 0x142c5b6f0
141400557: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140055c: 4c 8d 4c 24 40              	leaq	0x40(%rsp), %r9
141400561: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400567: e8 84 74 18 00              	callq	0x1415879f0 <.text+0x15869f0>
14140056c: e9 26 0c 00 00              	jmp	0x141401197 <.text+0x1400197>
141400571: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400575: 48 8b 1f                    	movq	(%rdi), %rbx
141400578: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140057c: 48 39 03                    	cmpq	%rax, (%rbx)
14140057f: 0f 84 ea 0c 00 00           	je	0x14140126f <.text+0x140026f>
141400585: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400589: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14140058d: 48 ff c0                    	incq	%rax
141400590: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400594: 4c 8d 05 da b0 85 01        	leaq	0x185b0da(%rip), %r8    # 0x142c5b675
14140059b: 41 b9 18 00 00 00           	movl	$0x18, %r9d
1414005a1: 48 89 f9                    	movq	%rdi, %rcx
1414005a4: e8 77 07 18 00              	callq	0x141580d20 <.text+0x157fd20>
1414005a9: 48 85 c0                    	testq	%rax, %rax
1414005ac: 0f 85 00 0c 00 00           	jne	0x1414011b2 <.text+0x14001b2>
1414005b2: 48 8b 1f                    	movq	(%rdi), %rbx
1414005b5: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414005b9: 48 39 13                    	cmpq	%rdx, (%rbx)
1414005bc: 0f 84 06 10 00 00           	je	0x1414015c8 <.text+0x14005c8>
1414005c2: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414005c6: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1414005ca: 48 ff c2                    	incq	%rdx
1414005cd: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414005d1: 48 8b 1f                    	movq	(%rdi), %rbx
1414005d4: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414005d8: 48 39 13                    	cmpq	%rdx, (%rbx)
1414005db: 0f 84 0d 10 00 00           	je	0x1414015ee <.text+0x14005ee>
1414005e1: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414005e5: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414005e9: 48 ff c2                    	incq	%rdx
1414005ec: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414005f0: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
1414005f4: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1414005f9: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1414005fe: 48 8d 15 9e af 85 01        	leaq	0x185af9e(%rip), %rdx   # 0x142c5b5a3
141400605: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140060a: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400610: e8 9b 44 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400615: 48 85 c0                    	testq	%rax, %rax
141400618: 0f 85 9c 0b 00 00           	jne	0x1414011ba <.text+0x14001ba>
14140061e: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
141400622: 48 8d 15 90 af 85 01        	leaq	0x185af90(%rip), %rdx   # 0x142c5b5b9
141400629: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140062e: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400634: e8 57 45 18 00              	callq	0x141584b90 <.text+0x1583b90>
141400639: 48 85 c0                    	testq	%rax, %rax
14140063c: 0f 85 78 0b 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400642: 48 83 c6 11                 	addq	$0x11, %rsi
141400646: 48 8d 15 72 af 85 01        	leaq	0x185af72(%rip), %rdx   # 0x142c5b5bf
14140064d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400652: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141400658: 49 89 f1                    	movq	%rsi, %r9
14140065b: e8 30 45 18 00              	callq	0x141584b90 <.text+0x1583b90>
141400660: e9 32 0b 00 00              	jmp	0x141401197 <.text+0x1400197>
141400665: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400669: 48 8b 1f                    	movq	(%rdi), %rbx
14140066c: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400670: 48 39 03                    	cmpq	%rax, (%rbx)
141400673: 0f 84 25 0c 00 00           	je	0x14140129e <.text+0x140029e>
141400679: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14140067d: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400681: 48 ff c0                    	incq	%rax
141400684: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400688: 4c 8d 05 71 af 85 01        	leaq	0x185af71(%rip), %r8    # 0x142c5b600
14140068f: 41 b9 15 00 00 00           	movl	$0x15, %r9d
141400695: 48 89 f9                    	movq	%rdi, %rcx
141400698: e8 83 06 18 00              	callq	0x141580d20 <.text+0x157fd20>
14140069d: 48 85 c0                    	testq	%rax, %rax
1414006a0: 0f 85 0c 0b 00 00           	jne	0x1414011b2 <.text+0x14001b2>
1414006a6: 48 8b 1f                    	movq	(%rdi), %rbx
1414006a9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414006ad: 48 39 13                    	cmpq	%rdx, (%rbx)
1414006b0: 0f 84 5e 0f 00 00           	je	0x141401614 <.text+0x1400614>
1414006b6: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414006ba: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1414006be: 48 ff c2                    	incq	%rdx
1414006c1: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414006c5: 48 8b 1f                    	movq	(%rdi), %rbx
1414006c8: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414006cc: 48 39 13                    	cmpq	%rdx, (%rbx)
1414006cf: 0f 84 65 0f 00 00           	je	0x14140163a <.text+0x140063a>
1414006d5: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414006d9: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414006dd: 48 ff c2                    	incq	%rdx
1414006e0: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414006e4: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
1414006e8: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1414006ed: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1414006f2: 48 8d 15 aa ae 85 01        	leaq	0x185aeaa(%rip), %rdx   # 0x142c5b5a3
1414006f9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1414006fe: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400704: e8 a7 43 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400709: 48 85 c0                    	testq	%rax, %rax
14140070c: 0f 85 a8 0a 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400712: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
141400716: 48 8d 15 9c ae 85 01        	leaq	0x185ae9c(%rip), %rdx   # 0x142c5b5b9
14140071d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400722: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400728: e8 63 2b 18 00              	callq	0x141583290 <.text+0x1582290>
14140072d: 48 85 c0                    	testq	%rax, %rax
141400730: 0f 85 84 0a 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400736: 48 83 c6 2c                 	addq	$0x2c, %rsi
14140073a: 48 8d 15 7e ae 85 01        	leaq	0x185ae7e(%rip), %rdx   # 0x142c5b5bf
141400741: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400746: 41 b8 05 00 00 00           	movl	$0x5, %r8d
14140074c: 49 89 f1                    	movq	%rsi, %r9
14140074f: e8 3c 2b 18 00              	callq	0x141583290 <.text+0x1582290>
141400754: e9 3e 0a 00 00              	jmp	0x141401197 <.text+0x1400197>
141400759: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
14140075d: 48 8b 1f                    	movq	(%rdi), %rbx
141400760: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400764: 48 39 03                    	cmpq	%rax, (%rbx)
141400767: 0f 84 60 0b 00 00           	je	0x1414012cd <.text+0x14002cd>
14140076d: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400771: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400775: 48 ff c0                    	incq	%rax
141400778: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14140077c: 4c 8d 05 0a af 85 01        	leaq	0x185af0a(%rip), %r8    # 0x142c5b68d
141400783: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
141400789: 48 89 f9                    	movq	%rdi, %rcx
14140078c: e8 8f 05 18 00              	callq	0x141580d20 <.text+0x157fd20>
141400791: 48 85 c0                    	testq	%rax, %rax
141400794: 0f 85 18 0a 00 00           	jne	0x1414011b2 <.text+0x14001b2>
14140079a: 48 8b 1f                    	movq	(%rdi), %rbx
14140079d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414007a1: 48 39 13                    	cmpq	%rdx, (%rbx)
1414007a4: 0f 84 b6 0e 00 00           	je	0x141401660 <.text+0x1400660>
1414007aa: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414007ae: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1414007b2: 48 ff c2                    	incq	%rdx
1414007b5: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414007b9: 48 8b 1f                    	movq	(%rdi), %rbx
1414007bc: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414007c0: 48 39 13                    	cmpq	%rdx, (%rbx)
1414007c3: 0f 84 bd 0e 00 00           	je	0x141401686 <.text+0x1400686>
1414007c9: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414007cd: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414007d1: 48 ff c2                    	incq	%rdx
1414007d4: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414007d8: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
1414007dc: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1414007e1: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1414007e6: 48 8d 15 b6 ad 85 01        	leaq	0x185adb6(%rip), %rdx   # 0x142c5b5a3
1414007ed: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1414007f2: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1414007f8: e8 b3 42 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
1414007fd: 48 85 c0                    	testq	%rax, %rax
141400800: 0f 85 b4 09 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400806: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
14140080a: 48 8d 15 8a ae 85 01        	leaq	0x185ae8a(%rip), %rdx   # 0x142c5b69b
141400811: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400816: 41 b8 17 00 00 00           	movl	$0x17, %r8d
14140081c: e8 2f 79 18 00              	callq	0x141588150 <.text+0x1587150>
141400821: 48 85 c0                    	testq	%rax, %rax
141400824: 0f 85 90 09 00 00           	jne	0x1414011ba <.text+0x14001ba>
14140082a: 48 83 c6 11                 	addq	$0x11, %rsi
14140082e: 48 8d 15 7d ae 85 01        	leaq	0x185ae7d(%rip), %rdx   # 0x142c5b6b2
141400835: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140083a: 41 b8 16 00 00 00           	movl	$0x16, %r8d
141400840: e9 df 02 00 00              	jmp	0x141400b24 <.text+0x13ffb24>
141400845: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400849: 48 8b 1f                    	movq	(%rdi), %rbx
14140084c: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400850: 48 39 03                    	cmpq	%rax, (%rbx)
141400853: 0f 84 a3 0a 00 00           	je	0x1414012fc <.text+0x14002fc>
141400859: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14140085d: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400861: 48 ff c0                    	incq	%rax
141400864: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400868: 4c 8d 05 e4 ad 85 01        	leaq	0x185ade4(%rip), %r8    # 0x142c5b653
14140086f: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
141400875: 48 89 f9                    	movq	%rdi, %rcx
141400878: e8 a3 04 18 00              	callq	0x141580d20 <.text+0x157fd20>
14140087d: 48 85 c0                    	testq	%rax, %rax
141400880: 0f 85 2c 09 00 00           	jne	0x1414011b2 <.text+0x14001b2>
141400886: 48 8b 1f                    	movq	(%rdi), %rbx
141400889: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140088d: 48 39 13                    	cmpq	%rdx, (%rbx)
141400890: 0f 84 16 0e 00 00           	je	0x1414016ac <.text+0x14006ac>
141400896: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14140089a: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14140089e: 48 ff c2                    	incq	%rdx
1414008a1: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414008a5: 48 8b 1f                    	movq	(%rdi), %rbx
1414008a8: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414008ac: 48 39 13                    	cmpq	%rdx, (%rbx)
1414008af: 0f 84 1d 0e 00 00           	je	0x1414016d2 <.text+0x14006d2>
1414008b5: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414008b9: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414008bd: 48 ff c2                    	incq	%rdx
1414008c0: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414008c4: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
1414008c8: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1414008cd: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1414008d2: 48 8d 15 ca ac 85 01        	leaq	0x185acca(%rip), %rdx   # 0x142c5b5a3
1414008d9: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1414008de: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1414008e4: e8 c7 41 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
1414008e9: 48 85 c0                    	testq	%rax, %rax
1414008ec: 0f 85 c8 08 00 00           	jne	0x1414011ba <.text+0x14001ba>
1414008f2: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
1414008f6: 48 8d 15 bc ac 85 01        	leaq	0x185acbc(%rip), %rdx   # 0x142c5b5b9
1414008fd: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400902: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400908: e8 13 30 18 00              	callq	0x141583920 <.text+0x1582920>
14140090d: 48 85 c0                    	testq	%rax, %rax
141400910: 0f 85 a4 08 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400916: 48 83 c6 11                 	addq	$0x11, %rsi
14140091a: 48 8d 15 9e ac 85 01        	leaq	0x185ac9e(%rip), %rdx   # 0x142c5b5bf
141400921: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400926: 41 b8 05 00 00 00           	movl	$0x5, %r8d
14140092c: 49 89 f1                    	movq	%rsi, %r9
14140092f: e8 ec 2f 18 00              	callq	0x141583920 <.text+0x1582920>
141400934: e9 5e 08 00 00              	jmp	0x141401197 <.text+0x1400197>
141400939: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
14140093d: 48 8b 1f                    	movq	(%rdi), %rbx
141400940: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400944: 48 39 03                    	cmpq	%rax, (%rbx)
141400947: 0f 84 de 09 00 00           	je	0x14140132b <.text+0x140032b>
14140094d: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400951: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400955: 48 ff c0                    	incq	%rax
141400958: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14140095c: 4c 8d 05 87 ac 85 01        	leaq	0x185ac87(%rip), %r8    # 0x142c5b5ea
141400963: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
141400969: 48 89 f9                    	movq	%rdi, %rcx
14140096c: e8 af 03 18 00              	callq	0x141580d20 <.text+0x157fd20>
141400971: 48 85 c0                    	testq	%rax, %rax
141400974: 0f 85 38 08 00 00           	jne	0x1414011b2 <.text+0x14001b2>
14140097a: 48 8b 1f                    	movq	(%rdi), %rbx
14140097d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400981: 48 39 13                    	cmpq	%rdx, (%rbx)
141400984: 0f 84 6e 0d 00 00           	je	0x1414016f8 <.text+0x14006f8>
14140098a: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14140098e: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141400992: 48 ff c2                    	incq	%rdx
141400995: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400999: 48 8b 1f                    	movq	(%rdi), %rbx
14140099c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414009a0: 48 39 13                    	cmpq	%rdx, (%rbx)
1414009a3: 0f 84 75 0d 00 00           	je	0x14140171e <.text+0x140071e>
1414009a9: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1414009ad: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1414009b1: 48 ff c2                    	incq	%rdx
1414009b4: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1414009b8: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
1414009bc: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1414009c1: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
1414009c6: 48 8d 15 d6 ab 85 01        	leaq	0x185abd6(%rip), %rdx   # 0x142c5b5a3
1414009cd: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1414009d2: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1414009d8: e8 d3 40 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
1414009dd: 48 85 c0                    	testq	%rax, %rax
1414009e0: 0f 85 d4 07 00 00           	jne	0x1414011ba <.text+0x14001ba>
1414009e6: 4c 8d 8e 80 00 00 00        	leaq	0x80(%rsi), %r9
1414009ed: 48 8d 15 da ab 85 01        	leaq	0x185abda(%rip), %rdx   # 0x142c5b5ce
1414009f4: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
1414009f9: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
1414009ff: e8 5c 47 18 00              	callq	0x141585160 <.text+0x1584160>
141400a04: 48 85 c0                    	testq	%rax, %rax
141400a07: 0f 85 ad 07 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400a0d: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141400a11: 48 8d 15 a1 ab 85 01        	leaq	0x185aba1(%rip), %rdx   # 0x142c5b5b9
141400a18: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400a1d: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400a23: e8 e8 2b 18 00              	callq	0x141583610 <.text+0x1582610>
141400a28: 48 85 c0                    	testq	%rax, %rax
141400a2b: 0f 85 89 07 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400a31: 48 81 c6 84 00 00 00        	addq	$0x84, %rsi
141400a38: e9 84 02 00 00              	jmp	0x141400cc1 <.text+0x13ffcc1>
141400a3d: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400a41: 48 8b 1f                    	movq	(%rdi), %rbx
141400a44: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400a48: 48 39 03                    	cmpq	%rax, (%rbx)
141400a4b: 0f 84 09 09 00 00           	je	0x14140135a <.text+0x140035a>
141400a51: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400a55: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400a59: 48 ff c0                    	incq	%rax
141400a5c: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400a60: 4c 8d 05 8f ac 85 01        	leaq	0x185ac8f(%rip), %r8    # 0x142c5b6f6
141400a67: 41 b9 0d 00 00 00           	movl	$0xd, %r9d
141400a6d: 48 89 f9                    	movq	%rdi, %rcx
141400a70: e8 ab 02 18 00              	callq	0x141580d20 <.text+0x157fd20>
141400a75: 48 85 c0                    	testq	%rax, %rax
141400a78: 0f 85 34 07 00 00           	jne	0x1414011b2 <.text+0x14001b2>
141400a7e: 48 8b 1f                    	movq	(%rdi), %rbx
141400a81: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400a85: 48 39 13                    	cmpq	%rdx, (%rbx)
141400a88: 0f 84 b6 0c 00 00           	je	0x141401744 <.text+0x1400744>
141400a8e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400a92: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141400a96: 48 ff c2                    	incq	%rdx
141400a99: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400a9d: 48 8b 1f                    	movq	(%rdi), %rbx
141400aa0: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400aa4: 48 39 13                    	cmpq	%rdx, (%rbx)
141400aa7: 0f 84 bd 0c 00 00           	je	0x14140176a <.text+0x140076a>
141400aad: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400ab1: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
141400ab5: 48 ff c2                    	incq	%rdx
141400ab8: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400abc: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141400ac0: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141400ac5: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
141400aca: 48 8d 15 d2 aa 85 01        	leaq	0x185aad2(%rip), %rdx   # 0x142c5b5a3
141400ad1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400ad6: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400adc: e8 cf 3f 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400ae1: 48 85 c0                    	testq	%rax, %rax
141400ae4: 0f 85 d0 06 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400aea: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
141400aee: 48 8d 15 0e ac 85 01        	leaq	0x185ac0e(%rip), %rdx   # 0x142c5b703
141400af5: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400afa: 41 b8 10 00 00 00           	movl	$0x10, %r8d
141400b00: e8 5b 5c 18 00              	callq	0x141586760 <.text+0x1585760>
141400b05: 48 85 c0                    	testq	%rax, %rax
141400b08: 0f 85 ac 06 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400b0e: 48 83 c6 2c                 	addq	$0x2c, %rsi
141400b12: 48 8d 15 fa ab 85 01        	leaq	0x185abfa(%rip), %rdx   # 0x142c5b713
141400b19: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400b1e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141400b24: 49 89 f1                    	movq	%rsi, %r9
141400b27: e8 24 76 18 00              	callq	0x141588150 <.text+0x1587150>
141400b2c: e9 66 06 00 00              	jmp	0x141401197 <.text+0x1400197>
141400b31: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400b35: 48 8b 1f                    	movq	(%rdi), %rbx
141400b38: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400b3c: 48 39 03                    	cmpq	%rax, (%rbx)
141400b3f: 0f 84 44 08 00 00           	je	0x141401389 <.text+0x1400389>
141400b45: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400b49: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400b4d: 48 ff c0                    	incq	%rax
141400b50: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400b54: 4c 8d 05 99 aa 85 01        	leaq	0x185aa99(%rip), %r8    # 0x142c5b5f4
141400b5b: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
141400b61: e9 92 03 00 00              	jmp	0x141400ef8 <.text+0x13ffef8>
141400b66: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400b6a: 48 8b 1f                    	movq	(%rdi), %rbx
141400b6d: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400b71: 48 39 03                    	cmpq	%rax, (%rbx)
141400b74: 0f 84 3e 08 00 00           	je	0x1414013b8 <.text+0x14003b8>
141400b7a: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400b7e: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400b82: 48 ff c0                    	incq	%rax
141400b85: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400b89: 4c 8d 05 a5 aa 85 01        	leaq	0x185aaa5(%rip), %r8    # 0x142c5b635
141400b90: 41 b9 10 00 00 00           	movl	$0x10, %r9d
141400b96: e9 5d 03 00 00              	jmp	0x141400ef8 <.text+0x13ffef8>
141400b9b: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400b9f: 48 8b 1f                    	movq	(%rdi), %rbx
141400ba2: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400ba6: 48 39 03                    	cmpq	%rax, (%rbx)
141400ba9: 0f 84 67 08 00 00           	je	0x141401416 <.text+0x1400416>
141400baf: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400bb3: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400bb7: 48 ff c0                    	incq	%rax
141400bba: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400bbe: 4c 8d 05 ff a9 85 01        	leaq	0x185a9ff(%rip), %r8    # 0x142c5b5c4
141400bc5: 41 b9 0a 00 00 00           	movl	$0xa, %r9d
141400bcb: 48 89 f9                    	movq	%rdi, %rcx
141400bce: e8 4d 01 18 00              	callq	0x141580d20 <.text+0x157fd20>
141400bd3: 48 85 c0                    	testq	%rax, %rax
141400bd6: 0f 85 d6 05 00 00           	jne	0x1414011b2 <.text+0x14001b2>
141400bdc: 48 8b 1f                    	movq	(%rdi), %rbx
141400bdf: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400be3: 48 39 13                    	cmpq	%rdx, (%rbx)
141400be6: 0f 84 f0 0b 00 00           	je	0x1414017dc <.text+0x14007dc>
141400bec: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400bf0: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141400bf4: 48 ff c2                    	incq	%rdx
141400bf7: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400bfb: 48 8b 1f                    	movq	(%rdi), %rbx
141400bfe: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400c02: 48 39 13                    	cmpq	%rdx, (%rbx)
141400c05: 0f 84 f7 0b 00 00           	je	0x141401802 <.text+0x1400802>
141400c0b: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400c0f: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
141400c13: 48 ff c2                    	incq	%rdx
141400c16: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400c1a: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
141400c1e: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141400c23: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
141400c28: 48 8d 15 74 a9 85 01        	leaq	0x185a974(%rip), %rdx   # 0x142c5b5a3
141400c2f: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400c34: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400c3a: e8 71 3e 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400c3f: 48 85 c0                    	testq	%rax, %rax
141400c42: 0f 85 72 05 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400c48: 4c 8d 8e 80 00 00 00        	leaq	0x80(%rsi), %r9
141400c4f: 48 8d 15 78 a9 85 01        	leaq	0x185a978(%rip), %rdx   # 0x142c5b5ce
141400c56: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400c5b: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
141400c61: e8 fa 44 18 00              	callq	0x141585160 <.text+0x1584160>
141400c66: 48 85 c0                    	testq	%rax, %rax
141400c69: 0f 85 4b 05 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400c6f: 4c 8d 8e 84 00 00 00        	leaq	0x84(%rsi), %r9
141400c76: 48 8d 15 5c a9 85 01        	leaq	0x185a95c(%rip), %rdx   # 0x142c5b5d9
141400c7d: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400c82: 41 b8 09 00 00 00           	movl	$0x9, %r8d
141400c88: e8 03 54 18 00              	callq	0x141586090 <.text+0x1585090>
141400c8d: 48 85 c0                    	testq	%rax, %rax
141400c90: 0f 85 24 05 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400c96: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141400c9a: 48 8d 15 1e a9 85 01        	leaq	0x185a91e(%rip), %rdx   # 0x142c5b5bf
141400ca1: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400ca6: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141400cac: e8 5f 29 18 00              	callq	0x141583610 <.text+0x1582610>
141400cb1: 48 85 c0                    	testq	%rax, %rax
141400cb4: 0f 85 00 05 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400cba: 48 81 c6 88 00 00 00        	addq	$0x88, %rsi
141400cc1: 48 8d 15 1a a9 85 01        	leaq	0x185a91a(%rip), %rdx   # 0x142c5b5e2
141400cc8: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400ccd: 41 b8 08 00 00 00           	movl	$0x8, %r8d
141400cd3: 49 89 f1                    	movq	%rsi, %r9
141400cd6: e8 65 2d 18 00              	callq	0x141583a40 <.text+0x1582a40>
141400cdb: e9 b7 04 00 00              	jmp	0x141401197 <.text+0x1400197>
141400ce0: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400ce4: 48 8b 1f                    	movq	(%rdi), %rbx
141400ce7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400ceb: 48 39 03                    	cmpq	%rax, (%rbx)
141400cee: 0f 84 51 07 00 00           	je	0x141401445 <.text+0x1400445>
141400cf4: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400cf8: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400cfc: 48 ff c0                    	incq	%rax
141400cff: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400d03: 4c 8d 05 0b a9 85 01        	leaq	0x185a90b(%rip), %r8    # 0x142c5b615
141400d0a: 41 b9 11 00 00 00           	movl	$0x11, %r9d
141400d10: 48 89 f9                    	movq	%rdi, %rcx
141400d13: e8 08 00 18 00              	callq	0x141580d20 <.text+0x157fd20>
141400d18: 48 85 c0                    	testq	%rax, %rax
141400d1b: 0f 85 91 04 00 00           	jne	0x1414011b2 <.text+0x14001b2>
141400d21: 48 8b 1f                    	movq	(%rdi), %rbx
141400d24: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400d28: 48 39 13                    	cmpq	%rdx, (%rbx)
141400d2b: 0f 84 f7 0a 00 00           	je	0x141401828 <.text+0x1400828>
141400d31: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400d35: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141400d39: 48 ff c2                    	incq	%rdx
141400d3c: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400d40: 48 8b 1f                    	movq	(%rdi), %rbx
141400d43: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400d47: 48 39 13                    	cmpq	%rdx, (%rbx)
141400d4a: 0f 84 fe 0a 00 00           	je	0x14140184e <.text+0x140084e>
141400d50: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400d54: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
141400d58: 48 ff c2                    	incq	%rdx
141400d5b: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400d5f: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141400d63: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141400d68: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
141400d6d: 48 8d 15 2f a8 85 01        	leaq	0x185a82f(%rip), %rdx   # 0x142c5b5a3
141400d74: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400d79: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400d7f: e8 2c 3d 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400d84: 48 85 c0                    	testq	%rax, %rax
141400d87: 0f 85 2d 04 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400d8d: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
141400d91: 48 8d 15 21 a8 85 01        	leaq	0x185a821(%rip), %rdx   # 0x142c5b5b9
141400d98: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400d9d: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400da3: e8 b8 59 18 00              	callq	0x141586760 <.text+0x1585760>
141400da8: 48 85 c0                    	testq	%rax, %rax
141400dab: 0f 85 09 04 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400db1: 48 83 c6 2c                 	addq	$0x2c, %rsi
141400db5: 48 8d 15 03 a8 85 01        	leaq	0x185a803(%rip), %rdx   # 0x142c5b5bf
141400dbc: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400dc1: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141400dc7: 49 89 f1                    	movq	%rsi, %r9
141400dca: e8 91 59 18 00              	callq	0x141586760 <.text+0x1585760>
141400dcf: e9 c3 03 00 00              	jmp	0x141401197 <.text+0x1400197>
141400dd4: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400dd8: 48 8b 1f                    	movq	(%rdi), %rbx
141400ddb: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400ddf: 48 39 03                    	cmpq	%rax, (%rbx)
141400de2: 0f 84 8c 06 00 00           	je	0x141401474 <.text+0x1400474>
141400de8: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400dec: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400df0: 48 ff c0                    	incq	%rax
141400df3: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400df7: 4c 8d 05 ca a8 85 01        	leaq	0x185a8ca(%rip), %r8    # 0x142c5b6c8
141400dfe: 41 b9 11 00 00 00           	movl	$0x11, %r9d
141400e04: 48 89 f9                    	movq	%rdi, %rcx
141400e07: e8 14 ff 17 00              	callq	0x141580d20 <.text+0x157fd20>
141400e0c: 48 85 c0                    	testq	%rax, %rax
141400e0f: 0f 85 9d 03 00 00           	jne	0x1414011b2 <.text+0x14001b2>
141400e15: 48 8b 1f                    	movq	(%rdi), %rbx
141400e18: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400e1c: 48 39 13                    	cmpq	%rdx, (%rbx)
141400e1f: 0f 84 4f 0a 00 00           	je	0x141401874 <.text+0x1400874>
141400e25: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400e29: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141400e2d: 48 ff c2                    	incq	%rdx
141400e30: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400e34: 48 8b 1f                    	movq	(%rdi), %rbx
141400e37: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400e3b: 48 39 13                    	cmpq	%rdx, (%rbx)
141400e3e: 0f 84 56 0a 00 00           	je	0x14140189a <.text+0x140089a>
141400e44: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400e48: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
141400e4c: 48 ff c2                    	incq	%rdx
141400e4f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400e53: 4c 8d 4e 78                 	leaq	0x78(%rsi), %r9
141400e57: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141400e5c: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
141400e61: 48 8d 15 3b a7 85 01        	leaq	0x185a73b(%rip), %rdx   # 0x142c5b5a3
141400e68: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400e6d: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400e73: e8 38 3c 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400e78: 48 85 c0                    	testq	%rax, %rax
141400e7b: 0f 85 39 03 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400e81: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141400e85: 48 8d 15 2d a7 85 01        	leaq	0x185a72d(%rip), %rdx   # 0x142c5b5b9
141400e8c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400e91: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400e97: e8 c4 4d 18 00              	callq	0x141585c60 <.text+0x1584c60>
141400e9c: 48 85 c0                    	testq	%rax, %rax
141400e9f: 0f 85 15 03 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400ea5: 48 83 c6 40                 	addq	$0x40, %rsi
141400ea9: 48 8d 15 0f a7 85 01        	leaq	0x185a70f(%rip), %rdx   # 0x142c5b5bf
141400eb0: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400eb5: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141400ebb: 49 89 f1                    	movq	%rsi, %r9
141400ebe: e8 9d 4d 18 00              	callq	0x141585c60 <.text+0x1584c60>
141400ec3: e9 cf 02 00 00              	jmp	0x141401197 <.text+0x1400197>
141400ec8: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400ecc: 48 8b 1f                    	movq	(%rdi), %rbx
141400ecf: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400ed3: 48 39 03                    	cmpq	%rax, (%rbx)
141400ed6: 0f 84 c7 05 00 00           	je	0x1414014a3 <.text+0x14004a3>
141400edc: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400ee0: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400ee4: 48 ff c0                    	incq	%rax
141400ee7: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400eeb: 4c 8d 05 34 a7 85 01        	leaq	0x185a734(%rip), %r8    # 0x142c5b626
141400ef2: 41 b9 0f 00 00 00           	movl	$0xf, %r9d
141400ef8: 48 89 f9                    	movq	%rdi, %rcx
141400efb: e8 20 fe 17 00              	callq	0x141580d20 <.text+0x157fd20>
141400f00: 48 85 c0                    	testq	%rax, %rax
141400f03: 0f 85 a9 02 00 00           	jne	0x1414011b2 <.text+0x14001b2>
141400f09: 48 8b 1f                    	movq	(%rdi), %rbx
141400f0c: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400f10: 48 39 13                    	cmpq	%rdx, (%rbx)
141400f13: 0f 84 ac 02 00 00           	je	0x1414011c5 <.text+0x14001c5>
141400f19: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400f1d: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141400f21: 48 ff c2                    	incq	%rdx
141400f24: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400f28: 48 8b 1f                    	movq	(%rdi), %rbx
141400f2b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141400f2f: 48 39 13                    	cmpq	%rdx, (%rbx)
141400f32: 0f 84 b3 02 00 00           	je	0x1414011eb <.text+0x14001eb>
141400f38: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141400f3c: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
141400f40: 48 ff c2                    	incq	%rdx
141400f43: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141400f47: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141400f4b: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141400f50: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
141400f55: 48 8d 15 47 a6 85 01        	leaq	0x185a647(%rip), %rdx   # 0x142c5b5a3
141400f5c: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400f61: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
141400f67: e8 44 3b 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141400f6c: 48 85 c0                    	testq	%rax, %rax
141400f6f: 0f 85 45 02 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400f75: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
141400f79: 48 8d 15 39 a6 85 01        	leaq	0x185a639(%rip), %rdx   # 0x142c5b5b9
141400f80: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400f85: 41 b8 06 00 00 00           	movl	$0x6, %r8d
141400f8b: e8 00 51 18 00              	callq	0x141586090 <.text+0x1585090>
141400f90: 48 85 c0                    	testq	%rax, %rax
141400f93: 0f 85 21 02 00 00           	jne	0x1414011ba <.text+0x14001ba>
141400f99: 48 83 c6 14                 	addq	$0x14, %rsi
141400f9d: 48 8d 15 1b a6 85 01        	leaq	0x185a61b(%rip), %rdx   # 0x142c5b5bf
141400fa4: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141400fa9: 41 b8 05 00 00 00           	movl	$0x5, %r8d
141400faf: 49 89 f1                    	movq	%rsi, %r9
141400fb2: e8 d9 50 18 00              	callq	0x141586090 <.text+0x1585090>
141400fb7: e9 db 01 00 00              	jmp	0x141401197 <.text+0x1400197>
141400fbc: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
141400fc0: 48 8b 1f                    	movq	(%rdi), %rbx
141400fc3: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141400fc7: 48 39 03                    	cmpq	%rax, (%rbx)
141400fca: 0f 84 02 05 00 00           	je	0x1414014d2 <.text+0x14004d2>
141400fd0: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
141400fd4: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
141400fd8: 48 ff c0                    	incq	%rax
141400fdb: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
141400fdf: 4c 8d 05 78 a6 85 01        	leaq	0x185a678(%rip), %r8    # 0x142c5b65e
141400fe6: 41 b9 17 00 00 00           	movl	$0x17, %r9d
141400fec: 48 89 f9                    	movq	%rdi, %rcx
141400fef: e8 2c fd 17 00              	callq	0x141580d20 <.text+0x157fd20>
141400ff4: 48 85 c0                    	testq	%rax, %rax
141400ff7: 0f 85 b5 01 00 00           	jne	0x1414011b2 <.text+0x14001b2>
141400ffd: 48 8b 1f                    	movq	(%rdi), %rbx
141401000: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401004: 48 39 13                    	cmpq	%rdx, (%rbx)
141401007: 0f 84 b3 08 00 00           	je	0x1414018c0 <.text+0x14008c0>
14140100d: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141401011: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141401015: 48 ff c2                    	incq	%rdx
141401018: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14140101c: 48 8b 1f                    	movq	(%rdi), %rbx
14140101f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401023: 48 39 13                    	cmpq	%rdx, (%rbx)
141401026: 0f 84 ba 08 00 00           	je	0x1414018e6 <.text+0x14008e6>
14140102c: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141401030: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
141401034: 48 ff c2                    	incq	%rdx
141401037: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14140103b: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
14140103f: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141401044: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
141401049: 48 8d 15 53 a5 85 01        	leaq	0x185a553(%rip), %rdx   # 0x142c5b5a3
141401050: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141401055: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14140105b: e8 50 3a 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141401060: 48 85 c0                    	testq	%rax, %rax
141401063: 0f 85 51 01 00 00           	jne	0x1414011ba <.text+0x14001ba>
141401069: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
14140106d: 48 8d 15 45 a5 85 01        	leaq	0x185a545(%rip), %rdx   # 0x142c5b5b9
141401074: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141401079: 41 b8 06 00 00 00           	movl	$0x6, %r8d
14140107f: e8 dc 40 18 00              	callq	0x141585160 <.text+0x1584160>
141401084: 48 85 c0                    	testq	%rax, %rax
141401087: 0f 85 2d 01 00 00           	jne	0x1414011ba <.text+0x14001ba>
14140108d: 48 83 c6 14                 	addq	$0x14, %rsi
141401091: 48 8d 15 27 a5 85 01        	leaq	0x185a527(%rip), %rdx   # 0x142c5b5bf
141401098: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
14140109d: 41 b8 05 00 00 00           	movl	$0x5, %r8d
1414010a3: 49 89 f1                    	movq	%rsi, %r9
1414010a6: e8 b5 40 18 00              	callq	0x141585160 <.text+0x1584160>
1414010ab: e9 e7 00 00 00              	jmp	0x141401197 <.text+0x1400197>
1414010b0: 48 8d 57 08                 	leaq	0x8(%rdi), %rdx
1414010b4: 48 8b 1f                    	movq	(%rdi), %rbx
1414010b7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1414010bb: 48 39 03                    	cmpq	%rax, (%rbx)
1414010be: 0f 84 3d 04 00 00           	je	0x141401501 <.text+0x1400501>
1414010c4: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1414010c8: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1414010cc: 48 ff c0                    	incq	%rax
1414010cf: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1414010d3: 4c 8d 05 6b a5 85 01        	leaq	0x185a56b(%rip), %r8    # 0x142c5b645
1414010da: 41 b9 0e 00 00 00           	movl	$0xe, %r9d
1414010e0: 48 89 f9                    	movq	%rdi, %rcx
1414010e3: e8 38 fc 17 00              	callq	0x141580d20 <.text+0x157fd20>
1414010e8: 48 85 c0                    	testq	%rax, %rax
1414010eb: 0f 85 c1 00 00 00           	jne	0x1414011b2 <.text+0x14001b2>
1414010f1: 48 8b 1f                    	movq	(%rdi), %rbx
1414010f4: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414010f8: 48 39 13                    	cmpq	%rdx, (%rbx)
1414010fb: 0f 84 0b 08 00 00           	je	0x14140190c <.text+0x140090c>
141401101: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141401105: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
141401109: 48 ff c2                    	incq	%rdx
14140110c: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
141401110: 48 8b 1f                    	movq	(%rdi), %rbx
141401113: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401117: 48 39 13                    	cmpq	%rdx, (%rbx)
14140111a: 0f 84 12 08 00 00           	je	0x141401932 <.text+0x1400932>
141401120: 48 8b 43 08                 	movq	0x8(%rbx), %rax
141401124: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
141401128: 48 ff c2                    	incq	%rdx
14140112b: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14140112f: 4c 8d 4e 08                 	leaq	0x8(%rsi), %r9
141401133: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
141401138: c6 44 24 38 01              	movb	$0x1, 0x38(%rsp)
14140113d: 48 8d 15 5f a4 85 01        	leaq	0x185a45f(%rip), %rdx   # 0x142c5b5a3
141401144: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141401149: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
14140114f: e8 5c 39 18 00              	callq	0x141584ab0 <.text+0x1583ab0>
141401154: 48 85 c0                    	testq	%rax, %rax
141401157: 75 61                       	jne	0x1414011ba <.text+0x14001ba>
141401159: 4c 8d 4e 10                 	leaq	0x10(%rsi), %r9
14140115d: 48 8d 15 55 a4 85 01        	leaq	0x185a455(%rip), %rdx   # 0x142c5b5b9
141401164: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141401169: 41 b8 06 00 00 00           	movl	$0x6, %r8d
14140116f: e8 1c 3a 18 00              	callq	0x141584b90 <.text+0x1583b90>
141401174: 48 85 c0                    	testq	%rax, %rax
141401177: 75 41                       	jne	0x1414011ba <.text+0x14001ba>
141401179: 48 83 c6 11                 	addq	$0x11, %rsi
14140117d: 48 8d 15 3b a4 85 01        	leaq	0x185a43b(%rip), %rdx   # 0x142c5b5bf
141401184: 48 8d 4c 24 30              	leaq	0x30(%rsp), %rcx
141401189: 41 b8 05 00 00 00           	movl	$0x5, %r8d
14140118f: 49 89 f1                    	movq	%rsi, %r9
141401192: e8 f9 39 18 00              	callq	0x141584b90 <.text+0x1583b90>
141401197: 48 85 c0                    	testq	%rax, %rax
14140119a: 75 1e                       	jne	0x1414011ba <.text+0x14001ba>
14140119c: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
1414011a1: 0f b6 54 24 38              	movzbl	0x38(%rsp), %edx
1414011a6: 48 8b 08                    	movq	(%rax), %rcx
1414011a9: e8 a2 51 73 ff              	callq	0x140b36350 <.text+0xb35350>
1414011ae: 31 c0                       	xorl	%eax, %eax
1414011b0: eb 08                       	jmp	0x1414011ba <.text+0x14001ba>
1414011b2: 48 89 c1                    	movq	%rax, %rcx
1414011b5: e8 e6 86 4c 01              	callq	0x1428c98a0 <.text+0x28c88a0>
1414011ba: 90                          	nop
1414011bb: 48 83 c4 48                 	addq	$0x48, %rsp
1414011bf: 5b                          	popq	%rbx
1414011c0: 5f                          	popq	%rdi
1414011c1: 5e                          	popq	%rsi
1414011c2: 41 5e                       	popq	%r14
1414011c4: c3                          	retq
1414011c5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414011ce: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414011d4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414011da: 48 89 d9                    	movq	%rbx, %rcx
1414011dd: e8 7e 88 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414011e2: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414011e6: e9 2e fd ff ff              	jmp	0x141400f19 <.text+0x13fff19>
1414011eb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414011f4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414011fa: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401200: 48 89 d9                    	movq	%rbx, %rcx
141401203: e8 58 88 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401208: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140120c: e9 27 fd ff ff              	jmp	0x141400f38 <.text+0x13fff38>
141401211: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140121a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401220: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401226: 48 89 d9                    	movq	%rbx, %rcx
141401229: 49 89 d6                    	movq	%rdx, %r14
14140122c: 48 89 c2                    	movq	%rax, %rdx
14140122f: e8 2c 88 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401234: 4c 89 f2                    	movq	%r14, %rdx
141401237: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140123b: e9 54 f1 ff ff              	jmp	0x141400394 <.text+0x13ff394>
141401240: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401249: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140124f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401255: 48 89 d9                    	movq	%rbx, %rcx
141401258: 49 89 d6                    	movq	%rdx, %r14
14140125b: 48 89 c2                    	movq	%rax, %rdx
14140125e: e8 fd 87 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401263: 4c 89 f2                    	movq	%r14, %rdx
141401266: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140126a: e9 11 f2 ff ff              	jmp	0x141400480 <.text+0x13ff480>
14140126f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401278: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140127e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401284: 48 89 d9                    	movq	%rbx, %rcx
141401287: 49 89 d6                    	movq	%rdx, %r14
14140128a: 48 89 c2                    	movq	%rax, %rdx
14140128d: e8 ce 87 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401292: 4c 89 f2                    	movq	%r14, %rdx
141401295: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141401299: e9 e7 f2 ff ff              	jmp	0x141400585 <.text+0x13ff585>
14140129e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414012a7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414012ad: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414012b3: 48 89 d9                    	movq	%rbx, %rcx
1414012b6: 49 89 d6                    	movq	%rdx, %r14
1414012b9: 48 89 c2                    	movq	%rax, %rdx
1414012bc: e8 9f 87 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414012c1: 4c 89 f2                    	movq	%r14, %rdx
1414012c4: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1414012c8: e9 ac f3 ff ff              	jmp	0x141400679 <.text+0x13ff679>
1414012cd: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414012d6: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414012dc: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414012e2: 48 89 d9                    	movq	%rbx, %rcx
1414012e5: 49 89 d6                    	movq	%rdx, %r14
1414012e8: 48 89 c2                    	movq	%rax, %rdx
1414012eb: e8 70 87 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414012f0: 4c 89 f2                    	movq	%r14, %rdx
1414012f3: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1414012f7: e9 71 f4 ff ff              	jmp	0x14140076d <.text+0x13ff76d>
1414012fc: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401305: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140130b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401311: 48 89 d9                    	movq	%rbx, %rcx
141401314: 49 89 d6                    	movq	%rdx, %r14
141401317: 48 89 c2                    	movq	%rax, %rdx
14140131a: e8 41 87 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140131f: 4c 89 f2                    	movq	%r14, %rdx
141401322: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141401326: e9 2e f5 ff ff              	jmp	0x141400859 <.text+0x13ff859>
14140132b: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401334: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140133a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401340: 48 89 d9                    	movq	%rbx, %rcx
141401343: 49 89 d6                    	movq	%rdx, %r14
141401346: 48 89 c2                    	movq	%rax, %rdx
141401349: e8 12 87 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140134e: 4c 89 f2                    	movq	%r14, %rdx
141401351: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141401355: e9 f3 f5 ff ff              	jmp	0x14140094d <.text+0x13ff94d>
14140135a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401363: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401369: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140136f: 48 89 d9                    	movq	%rbx, %rcx
141401372: 49 89 d6                    	movq	%rdx, %r14
141401375: 48 89 c2                    	movq	%rax, %rdx
141401378: e8 e3 86 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140137d: 4c 89 f2                    	movq	%r14, %rdx
141401380: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141401384: e9 c8 f6 ff ff              	jmp	0x141400a51 <.text+0x13ffa51>
141401389: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401392: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401398: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140139e: 48 89 d9                    	movq	%rbx, %rcx
1414013a1: 49 89 d6                    	movq	%rdx, %r14
1414013a4: 48 89 c2                    	movq	%rax, %rdx
1414013a7: e8 b4 86 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414013ac: 4c 89 f2                    	movq	%r14, %rdx
1414013af: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1414013b3: e9 8d f7 ff ff              	jmp	0x141400b45 <.text+0x13ffb45>
1414013b8: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414013c1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414013c7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414013cd: 48 89 d9                    	movq	%rbx, %rcx
1414013d0: 49 89 d6                    	movq	%rdx, %r14
1414013d3: 48 89 c2                    	movq	%rax, %rdx
1414013d6: e8 85 86 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414013db: 4c 89 f2                    	movq	%r14, %rdx
1414013de: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1414013e2: e9 93 f7 ff ff              	jmp	0x141400b7a <.text+0x13ffb7a>
1414013e7: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414013f0: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414013f6: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414013fc: 48 89 d9                    	movq	%rbx, %rcx
1414013ff: 49 89 d6                    	movq	%rdx, %r14
141401402: 48 89 c2                    	movq	%rax, %rdx
141401405: e8 56 86 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140140a: 4c 89 f2                    	movq	%r14, %rdx
14140140d: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141401411: e9 61 ee ff ff              	jmp	0x141400277 <.text+0x13ff277>
141401416: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140141f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401425: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140142b: 48 89 d9                    	movq	%rbx, %rcx
14140142e: 49 89 d6                    	movq	%rdx, %r14
141401431: 48 89 c2                    	movq	%rax, %rdx
141401434: e8 27 86 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401439: 4c 89 f2                    	movq	%r14, %rdx
14140143c: 48 8b 43 10                 	movq	0x10(%rbx), %rax
141401440: e9 6a f7 ff ff              	jmp	0x141400baf <.text+0x13ffbaf>
141401445: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140144e: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401454: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140145a: 48 89 d9                    	movq	%rbx, %rcx
14140145d: 49 89 d6                    	movq	%rdx, %r14
141401460: 48 89 c2                    	movq	%rax, %rdx
141401463: e8 f8 85 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401468: 4c 89 f2                    	movq	%r14, %rdx
14140146b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140146f: e9 80 f8 ff ff              	jmp	0x141400cf4 <.text+0x13ffcf4>
141401474: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140147d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401483: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401489: 48 89 d9                    	movq	%rbx, %rcx
14140148c: 49 89 d6                    	movq	%rdx, %r14
14140148f: 48 89 c2                    	movq	%rax, %rdx
141401492: e8 c9 85 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401497: 4c 89 f2                    	movq	%r14, %rdx
14140149a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140149e: e9 45 f9 ff ff              	jmp	0x141400de8 <.text+0x13ffde8>
1414014a3: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414014ac: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414014b2: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414014b8: 48 89 d9                    	movq	%rbx, %rcx
1414014bb: 49 89 d6                    	movq	%rdx, %r14
1414014be: 48 89 c2                    	movq	%rax, %rdx
1414014c1: e8 9a 85 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414014c6: 4c 89 f2                    	movq	%r14, %rdx
1414014c9: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1414014cd: e9 0a fa ff ff              	jmp	0x141400edc <.text+0x13ffedc>
1414014d2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414014db: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414014e1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414014e7: 48 89 d9                    	movq	%rbx, %rcx
1414014ea: 49 89 d6                    	movq	%rdx, %r14
1414014ed: 48 89 c2                    	movq	%rax, %rdx
1414014f0: e8 6b 85 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414014f5: 4c 89 f2                    	movq	%r14, %rdx
1414014f8: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1414014fc: e9 cf fa ff ff              	jmp	0x141400fd0 <.text+0x13fffd0>
141401501: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140150a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401510: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401516: 48 89 d9                    	movq	%rbx, %rcx
141401519: 49 89 d6                    	movq	%rdx, %r14
14140151c: 48 89 c2                    	movq	%rax, %rdx
14140151f: e8 3c 85 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401524: 4c 89 f2                    	movq	%r14, %rdx
141401527: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14140152b: e9 94 fb ff ff              	jmp	0x1414010c4 <.text+0x14000c4>
141401530: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401539: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140153f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401545: 48 89 d9                    	movq	%rbx, %rcx
141401548: e8 13 85 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140154d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401551: e9 7b ee ff ff              	jmp	0x1414003d1 <.text+0x13ff3d1>
141401556: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140155f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401565: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140156b: 48 89 d9                    	movq	%rbx, %rcx
14140156e: e8 ed 84 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401573: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401577: e9 74 ee ff ff              	jmp	0x1414003f0 <.text+0x13ff3f0>
14140157c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401585: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140158b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401591: 48 89 d9                    	movq	%rbx, %rcx
141401594: e8 c7 84 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401599: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140159d: e9 1b ef ff ff              	jmp	0x1414004bd <.text+0x13ff4bd>
1414015a2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414015ab: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414015b1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414015b7: 48 89 d9                    	movq	%rbx, %rcx
1414015ba: e8 a1 84 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414015bf: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414015c3: e9 14 ef ff ff              	jmp	0x1414004dc <.text+0x13ff4dc>
1414015c8: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414015d1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414015d7: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414015dd: 48 89 d9                    	movq	%rbx, %rcx
1414015e0: e8 7b 84 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414015e5: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414015e9: e9 d4 ef ff ff              	jmp	0x1414005c2 <.text+0x13ff5c2>
1414015ee: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414015f7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414015fd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401603: 48 89 d9                    	movq	%rbx, %rcx
141401606: e8 55 84 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140160b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140160f: e9 cd ef ff ff              	jmp	0x1414005e1 <.text+0x13ff5e1>
141401614: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140161d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401623: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401629: 48 89 d9                    	movq	%rbx, %rcx
14140162c: e8 2f 84 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401631: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401635: e9 7c f0 ff ff              	jmp	0x1414006b6 <.text+0x13ff6b6>
14140163a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401643: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401649: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140164f: 48 89 d9                    	movq	%rbx, %rcx
141401652: e8 09 84 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401657: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140165b: e9 75 f0 ff ff              	jmp	0x1414006d5 <.text+0x13ff6d5>
141401660: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401669: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140166f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401675: 48 89 d9                    	movq	%rbx, %rcx
141401678: e8 e3 83 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140167d: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401681: e9 24 f1 ff ff              	jmp	0x1414007aa <.text+0x13ff7aa>
141401686: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140168f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401695: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140169b: 48 89 d9                    	movq	%rbx, %rcx
14140169e: e8 bd 83 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414016a3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414016a7: e9 1d f1 ff ff              	jmp	0x1414007c9 <.text+0x13ff7c9>
1414016ac: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414016b5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414016bb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414016c1: 48 89 d9                    	movq	%rbx, %rcx
1414016c4: e8 97 83 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414016c9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414016cd: e9 c4 f1 ff ff              	jmp	0x141400896 <.text+0x13ff896>
1414016d2: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414016db: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414016e1: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414016e7: 48 89 d9                    	movq	%rbx, %rcx
1414016ea: e8 71 83 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414016ef: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414016f3: e9 bd f1 ff ff              	jmp	0x1414008b5 <.text+0x13ff8b5>
1414016f8: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401701: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401707: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140170d: 48 89 d9                    	movq	%rbx, %rcx
141401710: e8 4b 83 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401715: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401719: e9 6c f2 ff ff              	jmp	0x14140098a <.text+0x13ff98a>
14140171e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401727: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140172d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401733: 48 89 d9                    	movq	%rbx, %rcx
141401736: e8 25 83 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140173b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140173f: e9 65 f2 ff ff              	jmp	0x1414009a9 <.text+0x13ff9a9>
141401744: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140174d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401753: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401759: 48 89 d9                    	movq	%rbx, %rcx
14140175c: e8 ff 82 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401761: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401765: e9 24 f3 ff ff              	jmp	0x141400a8e <.text+0x13ffa8e>
14140176a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401773: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401779: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140177f: 48 89 d9                    	movq	%rbx, %rcx
141401782: e8 d9 82 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401787: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140178b: e9 1d f3 ff ff              	jmp	0x141400aad <.text+0x13ffaad>
141401790: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401799: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140179f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414017a5: 48 89 d9                    	movq	%rbx, %rcx
1414017a8: e8 b3 82 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414017ad: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414017b1: e9 fe ea ff ff              	jmp	0x1414002b4 <.text+0x13ff2b4>
1414017b6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414017bf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414017c5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414017cb: 48 89 d9                    	movq	%rbx, %rcx
1414017ce: e8 8d 82 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414017d3: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414017d7: e9 f7 ea ff ff              	jmp	0x1414002d3 <.text+0x13ff2d3>
1414017dc: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414017e5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414017eb: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414017f1: 48 89 d9                    	movq	%rbx, %rcx
1414017f4: e8 67 82 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414017f9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414017fd: e9 ea f3 ff ff              	jmp	0x141400bec <.text+0x13ffbec>
141401802: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140180b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401811: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401817: 48 89 d9                    	movq	%rbx, %rcx
14140181a: e8 41 82 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140181f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401823: e9 e3 f3 ff ff              	jmp	0x141400c0b <.text+0x13ffc0b>
141401828: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401831: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401837: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14140183d: 48 89 d9                    	movq	%rbx, %rcx
141401840: e8 1b 82 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401845: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401849: e9 e3 f4 ff ff              	jmp	0x141400d31 <.text+0x13ffd31>
14140184e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401857: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140185d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401863: 48 89 d9                    	movq	%rbx, %rcx
141401866: e8 f5 81 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140186b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140186f: e9 dc f4 ff ff              	jmp	0x141400d50 <.text+0x13ffd50>
141401874: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140187d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401883: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401889: 48 89 d9                    	movq	%rbx, %rcx
14140188c: e8 cf 81 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401891: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401895: e9 8b f5 ff ff              	jmp	0x141400e25 <.text+0x13ffe25>
14140189a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414018a3: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414018a9: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414018af: 48 89 d9                    	movq	%rbx, %rcx
1414018b2: e8 a9 81 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414018b7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414018bb: e9 84 f5 ff ff              	jmp	0x141400e44 <.text+0x13ffe44>
1414018c0: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414018c9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414018cf: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414018d5: 48 89 d9                    	movq	%rbx, %rcx
1414018d8: e8 83 81 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1414018dd: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1414018e1: e9 27 f7 ff ff              	jmp	0x14140100d <.text+0x140000d>
1414018e6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1414018ef: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1414018f5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1414018fb: 48 89 d9                    	movq	%rbx, %rcx
1414018fe: e8 5d 81 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401903: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401907: e9 20 f7 ff ff              	jmp	0x14140102c <.text+0x140002c>
14140190c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141401915: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14140191b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401921: 48 89 d9                    	movq	%rbx, %rcx
141401924: e8 37 81 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141401929: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14140192d: e9 cf f7 ff ff              	jmp	0x141401101 <.text+0x1400101>
141401932: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14140193b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
141401941: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141401947: 48 89 d9                    	movq	%rbx, %rcx
14140194a: e8 11 81 4a 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14140194f: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
141401953: e9 c8 f7 ff ff              	jmp	0x141401120 <.text+0x1400120>

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140316660: 41 57                       	pushq	%r15
140316662: 41 56                       	pushq	%r14
140316664: 56                          	pushq	%rsi
140316665: 57                          	pushq	%rdi
140316666: 53                          	pushq	%rbx
140316667: 48 83 ec 30                 	subq	$0x30, %rsp
14031666b: 48 89 d6                    	movq	%rdx, %rsi
14031666e: 48 8b 01                    	movq	(%rcx), %rax
140316671: 48 8d 15 c8 4c 6a 02        	leaq	0x26a4cc8(%rip), %rdx   # 0x1429bb340
140316678: 48 63 04 82                 	movslq	(%rdx,%rax,4), %rax
14031667c: 48 01 d0                    	addq	%rdx, %rax
14031667f: ff e0                       	jmpq	*%rax
140316681: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316685: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316689: 48 8b 1e                    	movq	(%rsi), %rbx
14031668c: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316690: 48 39 03                    	cmpq	%rax, (%rbx)
140316693: 0f 84 d9 07 00 00           	je	0x140316e72 <.text+0x315e72>
140316699: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031669d: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403166a1: 48 ff c0                    	incq	%rax
1403166a4: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403166a8: 4c 8d 05 c4 4b 6a 02        	leaq	0x26a4bc4(%rip), %r8    # 0x1429bb273
1403166af: 41 b9 07 00 00 00           	movl	$0x7, %r9d
1403166b5: 48 89 f1                    	movq	%rsi, %rcx
1403166b8: e8 43 fa 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403166bd: 48 85 c0                    	testq	%rax, %rax
1403166c0: 0f 85 d5 05 00 00           	jne	0x140316c9b <.text+0x315c9b>
1403166c6: 48 8b 1e                    	movq	(%rsi), %rbx
1403166c9: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403166cd: 48 39 13                    	cmpq	%rdx, (%rbx)
1403166d0: 0f 84 ef 09 00 00           	je	0x1403170c5 <.text+0x3160c5>
1403166d6: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403166da: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403166de: 48 ff c2                    	incq	%rdx
1403166e1: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403166e5: 48 83 c7 10                 	addq	$0x10, %rdi
1403166e9: 48 89 f9                    	movq	%rdi, %rcx
1403166ec: 48 89 f2                    	movq	%rsi, %rdx
1403166ef: e8 bc da 1d 00              	callq	0x1404f41b0 <.text+0x4f31b0>
1403166f4: e9 84 04 00 00              	jmp	0x140316b7d <.text+0x315b7d>
1403166f9: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
1403166fd: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316701: 48 8b 1e                    	movq	(%rsi), %rbx
140316704: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316708: 48 39 03                    	cmpq	%rax, (%rbx)
14031670b: 0f 84 e9 05 00 00           	je	0x140316cfa <.text+0x315cfa>
140316711: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316715: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316719: 48 ff c0                    	incq	%rax
14031671c: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316720: 4c 8d 05 b9 4b 6a 02        	leaq	0x26a4bb9(%rip), %r8    # 0x1429bb2e0
140316727: 41 b9 06 00 00 00           	movl	$0x6, %r9d
14031672d: 48 89 f1                    	movq	%rsi, %rcx
140316730: e8 cb f9 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316735: 48 85 c0                    	testq	%rax, %rax
140316738: 0f 85 5d 05 00 00           	jne	0x140316c9b <.text+0x315c9b>
14031673e: 48 8b 1e                    	movq	(%rsi), %rbx
140316741: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316745: 48 39 13                    	cmpq	%rdx, (%rbx)
140316748: 0f 84 6d 08 00 00           	je	0x140316fbb <.text+0x315fbb>
14031674e: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316752: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316756: 48 ff c2                    	incq	%rdx
140316759: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
14031675d: 48 83 c7 10                 	addq	$0x10, %rdi
140316761: 48 89 f9                    	movq	%rdi, %rcx
140316764: 48 89 f2                    	movq	%rsi, %rdx
140316767: e8 84 33 f4 ff              	callq	0x140259af0 <.text+0x258af0>
14031676c: e9 0c 04 00 00              	jmp	0x140316b7d <.text+0x315b7d>
140316771: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316775: 48 8b 3e                    	movq	(%rsi), %rdi
140316778: 48 8b 47 10                 	movq	0x10(%rdi), %rax
14031677c: 48 39 07                    	cmpq	%rax, (%rdi)
14031677f: 0f 84 a4 05 00 00           	je	0x140316d29 <.text+0x315d29>
140316785: 48 8b 4f 08                 	movq	0x8(%rdi), %rcx
140316789: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031678d: 48 ff c0                    	incq	%rax
140316790: 48 89 47 10                 	movq	%rax, 0x10(%rdi)
140316794: 4c 8d 05 2a 4b 6a 02        	leaq	0x26a4b2a(%rip), %r8    # 0x1429bb2c5
14031679b: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1403167a1: 48 89 f1                    	movq	%rsi, %rcx
1403167a4: e8 57 f9 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403167a9: 48 85 c0                    	testq	%rax, %rax
1403167ac: 0f 85 e9 04 00 00           	jne	0x140316c9b <.text+0x315c9b>
1403167b2: 48 8b 3e                    	movq	(%rsi), %rdi
1403167b5: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403167b9: 48 39 17                    	cmpq	%rdx, (%rdi)
1403167bc: 0f 84 1f 08 00 00           	je	0x140316fe1 <.text+0x315fe1>
1403167c2: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403167c6: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403167ca: 48 ff c2                    	incq	%rdx
1403167cd: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403167d1: 48 8b 3e                    	movq	(%rsi), %rdi
1403167d4: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403167d8: 48 39 17                    	cmpq	%rdx, (%rdi)
1403167db: 0f 84 26 08 00 00           	je	0x140317007 <.text+0x316007>
1403167e1: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403167e5: c6 04 10 7b                 	movb	$0x7b, (%rax,%rdx)
1403167e9: 48 ff c2                    	incq	%rdx
1403167ec: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
1403167f0: 48 39 17                    	cmpq	%rdx, (%rdi)
1403167f3: 0f 84 34 08 00 00           	je	0x14031702d <.text+0x31602d>
1403167f9: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1403167fd: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
140316801: 48 ff c2                    	incq	%rdx
140316804: 48 89 57 10                 	movq	%rdx, 0x10(%rdi)
140316808: e9 75 03 00 00              	jmp	0x140316b82 <.text+0x315b82>
14031680d: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316811: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316815: 48 8b 1e                    	movq	(%rsi), %rbx
140316818: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031681c: 48 39 03                    	cmpq	%rax, (%rbx)
14031681f: 0f 84 33 05 00 00           	je	0x140316d58 <.text+0x315d58>
140316825: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316829: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031682d: 48 ff c0                    	incq	%rax
140316830: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316834: 4c 8d 05 5b 4a 6a 02        	leaq	0x26a4a5b(%rip), %r8    # 0x1429bb296
14031683b: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
140316841: e9 f8 02 00 00              	jmp	0x140316b3e <.text+0x315b3e>
140316846: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
14031684a: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
14031684e: 48 8b 1e                    	movq	(%rsi), %rbx
140316851: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316855: 48 39 03                    	cmpq	%rax, (%rbx)
140316858: 0f 84 29 05 00 00           	je	0x140316d87 <.text+0x315d87>
14031685e: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316862: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316866: 48 ff c0                    	incq	%rax
140316869: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031686d: 4c 8d 05 5d 4a 6a 02        	leaq	0x26a4a5d(%rip), %r8    # 0x1429bb2d1
140316874: 41 b9 04 00 00 00           	movl	$0x4, %r9d
14031687a: 48 89 f1                    	movq	%rsi, %rcx
14031687d: e8 7e f8 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316882: 48 85 c0                    	testq	%rax, %rax
140316885: 0f 85 10 04 00 00           	jne	0x140316c9b <.text+0x315c9b>
14031688b: 48 8b 1e                    	movq	(%rsi), %rbx
14031688e: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316892: 48 39 13                    	cmpq	%rdx, (%rbx)
140316895: 0f 84 b8 07 00 00           	je	0x140317053 <.text+0x316053>
14031689b: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14031689f: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
1403168a3: 48 ff c2                    	incq	%rdx
1403168a6: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403168aa: 48 83 c7 10                 	addq	$0x10, %rdi
1403168ae: 48 89 f9                    	movq	%rdi, %rcx
1403168b1: 48 89 f2                    	movq	%rsi, %rdx
1403168b4: e8 97 6d d2 ff              	callq	0x14003d650 <.text+0x3c650>
1403168b9: e9 bf 02 00 00              	jmp	0x140316b7d <.text+0x315b7d>
1403168be: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
1403168c2: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1403168c6: 48 8b 1e                    	movq	(%rsi), %rbx
1403168c9: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403168cd: 48 39 03                    	cmpq	%rax, (%rbx)
1403168d0: 0f 84 e0 04 00 00           	je	0x140316db6 <.text+0x315db6>
1403168d6: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403168da: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403168de: 48 ff c0                    	incq	%rax
1403168e1: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
1403168e5: 4c 8d 05 c9 49 6a 02        	leaq	0x26a49c9(%rip), %r8    # 0x1429bb2b5
1403168ec: 41 b9 05 00 00 00           	movl	$0x5, %r9d
1403168f2: 48 89 f1                    	movq	%rsi, %rcx
1403168f5: e8 06 f8 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403168fa: 48 85 c0                    	testq	%rax, %rax
1403168fd: 0f 85 98 03 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316903: 48 8b 1e                    	movq	(%rsi), %rbx
140316906: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031690a: 48 39 13                    	cmpq	%rdx, (%rbx)
14031690d: 0f 84 66 07 00 00           	je	0x140317079 <.text+0x316079>
140316913: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316917: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
14031691b: 48 ff c2                    	incq	%rdx
14031691e: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316922: 48 83 c7 10                 	addq	$0x10, %rdi
140316926: 48 89 f9                    	movq	%rdi, %rcx
140316929: 48 89 f2                    	movq	%rsi, %rdx
14031692c: e8 9f 5c d2 ff              	callq	0x14003c5d0 <.text+0x3b5d0>
140316931: e9 47 02 00 00              	jmp	0x140316b7d <.text+0x315b7d>
140316936: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
14031693a: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
14031693e: 48 8b 1e                    	movq	(%rsi), %rbx
140316941: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316945: 48 39 03                    	cmpq	%rax, (%rbx)
140316948: 0f 84 97 04 00 00           	je	0x140316de5 <.text+0x315de5>
14031694e: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316952: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316956: 48 ff c0                    	incq	%rax
140316959: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
14031695d: 4c 8d 05 1a 49 6a 02        	leaq	0x26a491a(%rip), %r8    # 0x1429bb27e
140316964: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
14031696a: e9 cf 01 00 00              	jmp	0x140316b3e <.text+0x315b3e>
14031696f: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316973: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316977: 48 8b 1e                    	movq	(%rsi), %rbx
14031697a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
14031697e: 48 39 03                    	cmpq	%rax, (%rbx)
140316981: 0f 84 8d 04 00 00           	je	0x140316e14 <.text+0x315e14>
140316987: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
14031698b: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
14031698f: 48 ff c0                    	incq	%rax
140316992: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316996: 4c 8d 05 ed 48 6a 02        	leaq	0x26a48ed(%rip), %r8    # 0x1429bb28a
14031699d: 41 b9 0c 00 00 00           	movl	$0xc, %r9d
1403169a3: e9 96 01 00 00              	jmp	0x140316b3e <.text+0x315b3e>
1403169a8: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
1403169ac: 48 8b 59 10                 	movq	0x10(%rcx), %rbx
1403169b0: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
1403169b4: 4c 8b 36                    	movq	(%rsi), %r14
1403169b7: 49 8b 46 10                 	movq	0x10(%r14), %rax
1403169bb: 49 39 06                    	cmpq	%rax, (%r14)
1403169be: 0f 84 7f 04 00 00           	je	0x140316e43 <.text+0x315e43>
1403169c4: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
1403169c8: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
1403169cc: 48 ff c0                    	incq	%rax
1403169cf: 49 89 46 10                 	movq	%rax, 0x10(%r14)
1403169d3: 4c 8d 05 d3 48 6a 02        	leaq	0x26a48d3(%rip), %r8    # 0x1429bb2ad
1403169da: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1403169e0: 48 89 f1                    	movq	%rsi, %rcx
1403169e3: e8 18 f7 19 00              	callq	0x1404b6100 <.text+0x4b5100>
1403169e8: 48 85 c0                    	testq	%rax, %rax
1403169eb: 0f 85 aa 02 00 00           	jne	0x140316c9b <.text+0x315c9b>
1403169f1: 4c 8b 36                    	movq	(%rsi), %r14
1403169f4: 49 8b 56 10                 	movq	0x10(%r14), %rdx
1403169f8: 49 39 16                    	cmpq	%rdx, (%r14)
1403169fb: 0f 84 9e 06 00 00           	je	0x14031709f <.text+0x31609f>
140316a01: 49 8b 46 08                 	movq	0x8(%r14), %rax
140316a05: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316a09: 48 ff c2                    	incq	%rdx
140316a0c: 49 89 56 10                 	movq	%rdx, 0x10(%r14)
140316a10: 48 89 f1                    	movq	%rsi, %rcx
140316a13: 48 89 fa                    	movq	%rdi, %rdx
140316a16: 49 89 d8                    	movq	%rbx, %r8
140316a19: e8 12 0a 1a 00              	callq	0x1404b7430 <.text+0x4b6430>
140316a1e: e9 5a 01 00 00              	jmp	0x140316b7d <.text+0x315b7d>
140316a23: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316a27: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316a2b: 48 8b 1e                    	movq	(%rsi), %rbx
140316a2e: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316a32: 48 39 03                    	cmpq	%rax, (%rbx)
140316a35: 0f 84 66 04 00 00           	je	0x140316ea1 <.text+0x315ea1>
140316a3b: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316a3f: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316a43: 48 ff c0                    	incq	%rax
140316a46: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316a4a: 4c 8d 05 29 48 6a 02        	leaq	0x26a4829(%rip), %r8    # 0x1429bb27a
140316a51: 41 b9 04 00 00 00           	movl	$0x4, %r9d
140316a57: e9 e2 00 00 00              	jmp	0x140316b3e <.text+0x315b3e>
140316a5c: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316a60: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316a64: 48 8b 1e                    	movq	(%rsi), %rbx
140316a67: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316a6b: 48 39 03                    	cmpq	%rax, (%rbx)
140316a6e: 0f 84 5c 04 00 00           	je	0x140316ed0 <.text+0x315ed0>
140316a74: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316a78: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316a7c: 48 ff c0                    	incq	%rax
140316a7f: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316a83: 4c 8d 05 18 48 6a 02        	leaq	0x26a4818(%rip), %r8    # 0x1429bb2a2
140316a8a: 41 b9 05 00 00 00           	movl	$0x5, %r9d
140316a90: e9 a9 00 00 00              	jmp	0x140316b3e <.text+0x315b3e>
140316a95: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316a99: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316a9d: 48 8b 1e                    	movq	(%rsi), %rbx
140316aa0: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316aa4: 48 39 03                    	cmpq	%rax, (%rbx)
140316aa7: 0f 84 52 04 00 00           	je	0x140316eff <.text+0x315eff>
140316aad: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316ab1: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316ab5: 48 ff c0                    	incq	%rax
140316ab8: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316abc: 4c 8d 05 12 48 6a 02        	leaq	0x26a4812(%rip), %r8    # 0x1429bb2d5
140316ac3: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140316ac9: 48 89 f1                    	movq	%rsi, %rcx
140316acc: e8 2f f6 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316ad1: 48 85 c0                    	testq	%rax, %rax
140316ad4: 0f 85 c1 01 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316ada: 48 8b 1e                    	movq	(%rsi), %rbx
140316add: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316ae1: 48 39 13                    	cmpq	%rdx, (%rbx)
140316ae4: 0f 84 01 06 00 00           	je	0x1403170eb <.text+0x3160eb>
140316aea: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316aee: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316af2: 48 ff c2                    	incq	%rdx
140316af5: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316af9: 48 83 c7 10                 	addq	$0x10, %rdi
140316afd: 48 89 f9                    	movq	%rdi, %rcx
140316b00: 48 89 f2                    	movq	%rsi, %rdx
140316b03: e8 f8 67 d2 ff              	callq	0x14003d300 <.text+0x3c300>
140316b08: eb 73                       	jmp	0x140316b7d <.text+0x315b7d>
140316b0a: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316b0e: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316b12: 48 8b 1e                    	movq	(%rsi), %rbx
140316b15: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316b19: 48 39 03                    	cmpq	%rax, (%rbx)
140316b1c: 0f 84 0c 04 00 00           	je	0x140316f2e <.text+0x315f2e>
140316b22: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316b26: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316b2a: 48 ff c0                    	incq	%rax
140316b2d: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316b31: 4c 8d 05 6f 47 6a 02        	leaq	0x26a476f(%rip), %r8    # 0x1429bb2a7
140316b38: 41 b9 06 00 00 00           	movl	$0x6, %r9d
140316b3e: 48 89 f1                    	movq	%rsi, %rcx
140316b41: e8 ba f5 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316b46: 48 85 c0                    	testq	%rax, %rax
140316b49: 0f 85 4c 01 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316b4f: 48 8b 1e                    	movq	(%rsi), %rbx
140316b52: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316b56: 48 39 13                    	cmpq	%rdx, (%rbx)
140316b59: 0f 84 75 01 00 00           	je	0x140316cd4 <.text+0x315cd4>
140316b5f: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316b63: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316b67: 48 ff c2                    	incq	%rdx
140316b6a: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316b6e: 48 83 c7 10                 	addq	$0x10, %rdi
140316b72: 48 89 f9                    	movq	%rdi, %rcx
140316b75: 48 89 f2                    	movq	%rsi, %rdx
140316b78: e8 73 2b 17 00              	callq	0x1404896f0 <.text+0x4886f0>
140316b7d: 48 85 c0                    	testq	%rax, %rax
140316b80: 75 21                       	jne	0x140316ba3 <.text+0x315ba3>
140316b82: 48 8b 36                    	movq	(%rsi), %rsi
140316b85: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140316b89: 48 39 16                    	cmpq	%rdx, (%rsi)
140316b8c: 0f 84 1c 01 00 00           	je	0x140316cae <.text+0x315cae>
140316b92: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140316b96: c6 04 10 7d                 	movb	$0x7d, (%rax,%rdx)
140316b9a: 48 ff c2                    	incq	%rdx
140316b9d: 48 89 56 10                 	movq	%rdx, 0x10(%rsi)
140316ba1: 31 c0                       	xorl	%eax, %eax
140316ba3: 48 83 c4 30                 	addq	$0x30, %rsp
140316ba7: 5b                          	popq	%rbx
140316ba8: 5f                          	popq	%rdi
140316ba9: 5e                          	popq	%rsi
140316baa: 41 5e                       	popq	%r14
140316bac: 41 5f                       	popq	%r15
140316bae: c3                          	retq
140316baf: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316bb3: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316bb7: 48 8b 1e                    	movq	(%rsi), %rbx
140316bba: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316bbe: 48 39 03                    	cmpq	%rax, (%rbx)
140316bc1: 0f 84 96 03 00 00           	je	0x140316f5d <.text+0x315f5d>
140316bc7: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316bcb: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316bcf: 48 ff c0                    	incq	%rax
140316bd2: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316bd6: 4c 8d 05 dd 46 6a 02        	leaq	0x26a46dd(%rip), %r8    # 0x1429bb2ba
140316bdd: 41 b9 0b 00 00 00           	movl	$0xb, %r9d
140316be3: 48 89 f1                    	movq	%rsi, %rcx
140316be6: e8 15 f5 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316beb: 48 85 c0                    	testq	%rax, %rax
140316bee: 0f 85 a7 00 00 00           	jne	0x140316c9b <.text+0x315c9b>
140316bf4: 48 8b 1e                    	movq	(%rsi), %rbx
140316bf7: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316bfb: 48 39 13                    	cmpq	%rdx, (%rbx)
140316bfe: 0f 84 0d 05 00 00           	je	0x140317111 <.text+0x316111>
140316c04: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316c08: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316c0c: 48 ff c2                    	incq	%rdx
140316c0f: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316c13: 48 83 c7 10                 	addq	$0x10, %rdi
140316c17: 48 89 f9                    	movq	%rdi, %rcx
140316c1a: 48 89 f2                    	movq	%rsi, %rdx
140316c1d: e8 9e db 1d 00              	callq	0x1404f47c0 <.text+0x4f37c0>
140316c22: e9 56 ff ff ff              	jmp	0x140316b7d <.text+0x315b7d>
140316c27: 48 8b 79 08                 	movq	0x8(%rcx), %rdi
140316c2b: 48 8d 56 08                 	leaq	0x8(%rsi), %rdx
140316c2f: 48 8b 1e                    	movq	(%rsi), %rbx
140316c32: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316c36: 48 39 03                    	cmpq	%rax, (%rbx)
140316c39: 0f 84 4d 03 00 00           	je	0x140316f8c <.text+0x315f8c>
140316c3f: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140316c43: c6 04 01 7b                 	movb	$0x7b, (%rcx,%rax)
140316c47: 48 ff c0                    	incq	%rax
140316c4a: 48 89 43 10                 	movq	%rax, 0x10(%rbx)
140316c4e: 4c 8d 05 5c 46 6a 02        	leaq	0x26a465c(%rip), %r8    # 0x1429bb2b1
140316c55: 41 b9 04 00 00 00           	movl	$0x4, %r9d
140316c5b: 48 89 f1                    	movq	%rsi, %rcx
140316c5e: e8 9d f4 19 00              	callq	0x1404b6100 <.text+0x4b5100>
140316c63: 48 85 c0                    	testq	%rax, %rax
140316c66: 75 33                       	jne	0x140316c9b <.text+0x315c9b>
140316c68: 48 8b 1e                    	movq	(%rsi), %rbx
140316c6b: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316c6f: 48 39 13                    	cmpq	%rdx, (%rbx)
140316c72: 0f 84 bf 04 00 00           	je	0x140317137 <.text+0x316137>
140316c78: 48 8b 43 08                 	movq	0x8(%rbx), %rax
140316c7c: c6 04 10 3a                 	movb	$0x3a, (%rax,%rdx)
140316c80: 48 ff c2                    	incq	%rdx
140316c83: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
140316c87: 48 83 c7 10                 	addq	$0x10, %rdi
140316c8b: 48 89 f9                    	movq	%rdi, %rcx
140316c8e: 48 89 f2                    	movq	%rsi, %rdx
140316c91: e8 9a 31 f4 ff              	callq	0x140259e30 <.text+0x258e30>
140316c96: e9 e2 fe ff ff              	jmp	0x140316b7d <.text+0x315b7d>
140316c9b: 48 89 c1                    	movq	%rax, %rcx
140316c9e: 48 83 c4 30                 	addq	$0x30, %rsp
140316ca2: 5b                          	popq	%rbx
140316ca3: 5f                          	popq	%rdi
140316ca4: 5e                          	popq	%rsi
140316ca5: 41 5e                       	popq	%r14
140316ca7: 41 5f                       	popq	%r15
140316ca9: e9 f2 2b 5b 02              	jmp	0x1428c98a0 <.text+0x28c88a0>
140316cae: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316cb7: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316cbd: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316cc3: 48 89 f1                    	movq	%rsi, %rcx
140316cc6: e8 15 bd 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316ccb: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140316ccf: e9 be fe ff ff              	jmp	0x140316b92 <.text+0x315b92>
140316cd4: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316cdd: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316ce3: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316ce9: 48 89 d9                    	movq	%rbx, %rcx
140316cec: e8 ef bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316cf1: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316cf5: e9 65 fe ff ff              	jmp	0x140316b5f <.text+0x315b5f>
140316cfa: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d03: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d09: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d0f: 48 89 d9                    	movq	%rbx, %rcx
140316d12: 49 89 d6                    	movq	%rdx, %r14
140316d15: 48 89 c2                    	movq	%rax, %rdx
140316d18: e8 c3 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316d1d: 4c 89 f2                    	movq	%r14, %rdx
140316d20: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316d24: e9 e8 f9 ff ff              	jmp	0x140316711 <.text+0x315711>
140316d29: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d32: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d38: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d3e: 48 89 f9                    	movq	%rdi, %rcx
140316d41: 48 89 d3                    	movq	%rdx, %rbx
140316d44: 48 89 c2                    	movq	%rax, %rdx
140316d47: e8 94 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316d4c: 48 89 da                    	movq	%rbx, %rdx
140316d4f: 48 8b 47 10                 	movq	0x10(%rdi), %rax
140316d53: e9 2d fa ff ff              	jmp	0x140316785 <.text+0x315785>
140316d58: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d61: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d67: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d6d: 48 89 d9                    	movq	%rbx, %rcx
140316d70: 49 89 d6                    	movq	%rdx, %r14
140316d73: 48 89 c2                    	movq	%rax, %rdx
140316d76: e8 65 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316d7b: 4c 89 f2                    	movq	%r14, %rdx
140316d7e: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316d82: e9 9e fa ff ff              	jmp	0x140316825 <.text+0x315825>
140316d87: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316d90: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316d96: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316d9c: 48 89 d9                    	movq	%rbx, %rcx
140316d9f: 49 89 d6                    	movq	%rdx, %r14
140316da2: 48 89 c2                    	movq	%rax, %rdx
140316da5: e8 36 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316daa: 4c 89 f2                    	movq	%r14, %rdx
140316dad: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316db1: e9 a8 fa ff ff              	jmp	0x14031685e <.text+0x31585e>
140316db6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316dbf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316dc5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316dcb: 48 89 d9                    	movq	%rbx, %rcx
140316dce: 49 89 d6                    	movq	%rdx, %r14
140316dd1: 48 89 c2                    	movq	%rax, %rdx
140316dd4: e8 07 bc 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316dd9: 4c 89 f2                    	movq	%r14, %rdx
140316ddc: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316de0: e9 f1 fa ff ff              	jmp	0x1403168d6 <.text+0x3158d6>
140316de5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316dee: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316df4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316dfa: 48 89 d9                    	movq	%rbx, %rcx
140316dfd: 49 89 d6                    	movq	%rdx, %r14
140316e00: 48 89 c2                    	movq	%rax, %rdx
140316e03: e8 d8 bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e08: 4c 89 f2                    	movq	%r14, %rdx
140316e0b: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316e0f: e9 3a fb ff ff              	jmp	0x14031694e <.text+0x31594e>
140316e14: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316e1d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316e23: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316e29: 48 89 d9                    	movq	%rbx, %rcx
140316e2c: 49 89 d6                    	movq	%rdx, %r14
140316e2f: 48 89 c2                    	movq	%rax, %rdx
140316e32: e8 a9 bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e37: 4c 89 f2                    	movq	%r14, %rdx
140316e3a: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316e3e: e9 44 fb ff ff              	jmp	0x140316987 <.text+0x315987>
140316e43: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316e4c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316e52: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316e58: 4c 89 f1                    	movq	%r14, %rcx
140316e5b: 49 89 d7                    	movq	%rdx, %r15
140316e5e: 48 89 c2                    	movq	%rax, %rdx
140316e61: e8 7a bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e66: 4c 89 fa                    	movq	%r15, %rdx
140316e69: 49 8b 46 10                 	movq	0x10(%r14), %rax
140316e6d: e9 52 fb ff ff              	jmp	0x1403169c4 <.text+0x3159c4>
140316e72: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316e7b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316e81: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316e87: 48 89 d9                    	movq	%rbx, %rcx
140316e8a: 49 89 d6                    	movq	%rdx, %r14
140316e8d: 48 89 c2                    	movq	%rax, %rdx
140316e90: e8 4b bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316e95: 4c 89 f2                    	movq	%r14, %rdx
140316e98: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316e9c: e9 f8 f7 ff ff              	jmp	0x140316699 <.text+0x315699>
140316ea1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316eaa: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316eb0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316eb6: 48 89 d9                    	movq	%rbx, %rcx
140316eb9: 49 89 d6                    	movq	%rdx, %r14
140316ebc: 48 89 c2                    	movq	%rax, %rdx
140316ebf: e8 1c bb 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316ec4: 4c 89 f2                    	movq	%r14, %rdx
140316ec7: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316ecb: e9 6b fb ff ff              	jmp	0x140316a3b <.text+0x315a3b>
140316ed0: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316ed9: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316edf: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316ee5: 48 89 d9                    	movq	%rbx, %rcx
140316ee8: 49 89 d6                    	movq	%rdx, %r14
140316eeb: 48 89 c2                    	movq	%rax, %rdx
140316eee: e8 ed ba 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316ef3: 4c 89 f2                    	movq	%r14, %rdx
140316ef6: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316efa: e9 75 fb ff ff              	jmp	0x140316a74 <.text+0x315a74>
140316eff: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316f08: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316f0e: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316f14: 48 89 d9                    	movq	%rbx, %rcx
140316f17: 49 89 d6                    	movq	%rdx, %r14
140316f1a: 48 89 c2                    	movq	%rax, %rdx
140316f1d: e8 be ba 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316f22: 4c 89 f2                    	movq	%r14, %rdx
140316f25: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316f29: e9 7f fb ff ff              	jmp	0x140316aad <.text+0x315aad>
140316f2e: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316f37: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316f3d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316f43: 48 89 d9                    	movq	%rbx, %rcx
140316f46: 49 89 d6                    	movq	%rdx, %r14
140316f49: 48 89 c2                    	movq	%rax, %rdx
140316f4c: e8 8f ba 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316f51: 4c 89 f2                    	movq	%r14, %rdx
140316f54: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316f58: e9 c5 fb ff ff              	jmp	0x140316b22 <.text+0x315b22>
140316f5d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316f66: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316f6c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316f72: 48 89 d9                    	movq	%rbx, %rcx
140316f75: 49 89 d6                    	movq	%rdx, %r14
140316f78: 48 89 c2                    	movq	%rax, %rdx
140316f7b: e8 60 ba 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316f80: 4c 89 f2                    	movq	%r14, %rdx
140316f83: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316f87: e9 3b fc ff ff              	jmp	0x140316bc7 <.text+0x315bc7>
140316f8c: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316f95: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316f9b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316fa1: 48 89 d9                    	movq	%rbx, %rcx
140316fa4: 49 89 d6                    	movq	%rdx, %r14
140316fa7: 48 89 c2                    	movq	%rax, %rdx
140316faa: e8 31 ba 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316faf: 4c 89 f2                    	movq	%r14, %rdx
140316fb2: 48 8b 43 10                 	movq	0x10(%rbx), %rax
140316fb6: e9 84 fc ff ff              	jmp	0x140316c3f <.text+0x315c3f>
140316fbb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316fc4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316fca: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316fd0: 48 89 d9                    	movq	%rbx, %rcx
140316fd3: e8 08 ba 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316fd8: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140316fdc: e9 6d f7 ff ff              	jmp	0x14031674e <.text+0x31574e>
140316fe1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140316fea: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140316ff0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140316ff6: 48 89 f9                    	movq	%rdi, %rcx
140316ff9: e8 e2 b9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140316ffe: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140317002: e9 bb f7 ff ff              	jmp	0x1403167c2 <.text+0x3157c2>
140317007: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140317010: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140317016: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031701c: 48 89 f9                    	movq	%rdi, %rcx
14031701f: e8 bc b9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140317024: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
140317028: e9 b4 f7 ff ff              	jmp	0x1403167e1 <.text+0x3157e1>
14031702d: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140317036: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14031703c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140317042: 48 89 f9                    	movq	%rdi, %rcx
140317045: e8 96 b9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031704a: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
14031704e: e9 a6 f7 ff ff              	jmp	0x1403167f9 <.text+0x3157f9>
140317053: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031705c: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140317062: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140317068: 48 89 d9                    	movq	%rbx, %rcx
14031706b: e8 70 b9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140317070: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140317074: e9 22 f8 ff ff              	jmp	0x14031689b <.text+0x31589b>
140317079: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140317082: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140317088: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031708e: 48 89 d9                    	movq	%rbx, %rcx
140317091: e8 4a b9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140317096: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031709a: e9 74 f8 ff ff              	jmp	0x140316913 <.text+0x315913>
14031709f: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403170a8: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403170ae: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403170b4: 4c 89 f1                    	movq	%r14, %rcx
1403170b7: e8 24 b9 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403170bc: 49 8b 56 10                 	movq	0x10(%r14), %rdx
1403170c0: e9 3c f9 ff ff              	jmp	0x140316a01 <.text+0x315a01>
1403170c5: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403170ce: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403170d4: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403170da: 48 89 d9                    	movq	%rbx, %rcx
1403170dd: e8 fe b8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403170e2: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403170e6: e9 eb f5 ff ff              	jmp	0x1403166d6 <.text+0x3156d6>
1403170eb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403170f4: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403170fa: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140317100: 48 89 d9                    	movq	%rbx, %rcx
140317103: e8 d8 b8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140317108: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
14031710c: e9 d9 f9 ff ff              	jmp	0x140316aea <.text+0x315aea>
140317111: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14031711a: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140317120: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140317126: 48 89 d9                    	movq	%rbx, %rcx
140317129: e8 b2 b8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
14031712e: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140317132: e9 cd fa ff ff              	jmp	0x140316c04 <.text+0x315c04>
140317137: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140317140: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140317146: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14031714c: 48 89 d9                    	movq	%rbx, %rcx
14031714f: e8 8c b8 58 02              	callq	0x1428a29e0 <.text+0x28a19e0>
140317154: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
140317158: e9 1b fb ff ff              	jmp	0x140316c78 <.text+0x315c78>

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aaa350: 41 57                       	pushq	%r15
140aaa352: 41 56                       	pushq	%r14
140aaa354: 56                          	pushq	%rsi
140aaa355: 57                          	pushq	%rdi
140aaa356: 53                          	pushq	%rbx
140aaa357: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
140aaa35e: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
140aaa363: 48 83 79 18 00              	cmpq	$0x0, 0x18(%rcx)
140aaa368: 0f 84 b0 00 00 00           	je	0x140aaa41e <.text+0xaa941e>
140aaa36e: 4c 89 c6                    	movq	%r8, %rsi
140aaa371: 48 89 d7                    	movq	%rdx, %rdi
140aaa374: 48 89 cb                    	movq	%rcx, %rbx
140aaa377: 48 83 c1 20                 	addq	$0x20, %rcx
140aaa37b: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
140aaa380: e8 eb 69 b0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140aaa385: 4c 8b 33                    	movq	(%rbx), %r14
140aaa388: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140aaa38c: 48 89 ca                    	movq	%rcx, %rdx
140aaa38f: 48 21 c2                    	andq	%rax, %rdx
140aaa392: 48 c1 e8 39                 	shrq	$0x39, %rax
140aaa396: 66 0f 6e c0                 	movd	%eax, %xmm0
140aaa39a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140aaa39e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140aaa3a3: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140aaa3a8: 49 8d 46 88                 	leaq	-0x78(%r14), %rax
140aaa3ac: 45 31 c0                    	xorl	%r8d, %r8d
140aaa3af: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140aaa3b3: f3 41 0f 6f 14 16           	movdqu	(%r14,%rdx), %xmm2
140aaa3b9: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140aaa3bd: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140aaa3c1: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140aaa3c6: 45 85 c9                    	testl	%r9d, %r9d
140aaa3c9: 74 35                       	je	0x140aaa400 <.text+0xaa9400>
140aaa3cb: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140aaa3d0: 49 01 d2                    	addq	%rdx, %r10
140aaa3d3: 49 21 ca                    	andq	%rcx, %r10
140aaa3d6: 49 f7 da                    	negq	%r10
140aaa3d9: 4d 6b fa 78                 	imulq	$0x78, %r10, %r15
140aaa3dd: 4a 39 3c 38                 	cmpq	%rdi, (%rax,%r15)
140aaa3e1: 0f 84 1e 01 00 00           	je	0x140aaa505 <.text+0xaa9505>
140aaa3e7: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140aaa3eb: 66 45 21 ca                 	andw	%r9w, %r10w
140aaa3ef: 45 89 d1                    	movl	%r10d, %r9d
140aaa3f2: 75 d7                       	jne	0x140aaa3cb <.text+0xaa93cb>
140aaa3f4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140aaa400: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140aaa404: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140aaa409: 45 85 c9                    	testl	%r9d, %r9d
140aaa40c: 75 10                       	jne	0x140aaa41e <.text+0xaa941e>
140aaa40e: 4c 01 c2                    	addq	%r8, %rdx
140aaa411: 48 83 c2 10                 	addq	$0x10, %rdx
140aaa415: 49 83 c0 10                 	addq	$0x10, %r8
140aaa419: 48 21 ca                    	andq	%rcx, %rdx
140aaa41c: eb 95                       	jmp	0x140aaa3b3 <.text+0xaa93b3>
140aaa41e: 48 8b 05 83 1f b8 02        	movq	0x2b81f83(%rip), %rax   # 0x14362c3a8
140aaa425: 48 8b 00                    	movq	(%rax), %rax
140aaa428: 48 83 f8 02                 	cmpq	$0x2, %rax
140aaa42c: 0f 82 ea 00 00 00           	jb	0x140aaa51c <.text+0xaa951c>
140aaa432: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140aaa437: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140aaa43c: 48 8d 05 7d aa 08 00        	leaq	0x8aa7d(%rip), %rax     # 0x140b34ec0 <.text+0xb33ec0>
140aaa443: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140aaa448: 48 8d 0d 31 30 03 02        	leaq	0x2033031(%rip), %rcx   # 0x142add480
140aaa44f: e8 2c 39 bd ff              	callq	0x14067dd80 <.text+0x67cd80>
140aaa454: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140aaa458: 8b 40 10                    	movl	0x10(%rax), %eax
140aaa45b: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
140aaa464: 48 8d 0d 2d 30 03 02        	leaq	0x203302d(%rip), %rcx   # 0x142add498
140aaa46b: 48 89 4c 24 78              	movq	%rcx, 0x78(%rsp)
140aaa470: 48 c7 84 24 80 00 00 00 2b 00 00 00 	movq	$0x2b, 0x80(%rsp)
140aaa47c: 48 8d 15 a5 2f 03 02        	leaq	0x2032fa5(%rip), %rdx   # 0x142add428
140aaa483: 48 89 94 24 90 00 00 00     	movq	%rdx, 0x90(%rsp)
140aaa48b: 48 c7 84 24 98 00 00 00 02 00 00 00 	movq	$0x2, 0x98(%rsp)
140aaa497: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140aaa49c: 48 89 94 24 a0 00 00 00     	movq	%rdx, 0xa0(%rsp)
140aaa4a4: 48 c7 84 24 a8 00 00 00 01 00 00 00 	movq	$0x1, 0xa8(%rsp)
140aaa4b0: 48 c7 84 24 b0 00 00 00 00 00 00 00 	movq	$0x0, 0xb0(%rsp)
140aaa4bc: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
140aaa4c5: 48 89 4c 24 48              	movq	%rcx, 0x48(%rsp)
140aaa4ca: 48 c7 44 24 50 2b 00 00 00  	movq	$0x2b, 0x50(%rsp)
140aaa4d3: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
140aaa4dc: f3 0f 7f 44 24 60           	movdqu	%xmm0, 0x60(%rsp)
140aaa4e2: c7 84 24 88 00 00 00 01 00 00 00    	movl	$0x1, 0x88(%rsp)
140aaa4ed: 89 84 24 8c 00 00 00        	movl	%eax, 0x8c(%rsp)
140aaa4f4: 48 8d 4c 24 27              	leaq	0x27(%rsp), %rcx
140aaa4f9: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
140aaa4fe: e8 2d fe 29 00              	callq	0x140d4a330 <.text+0xd49330>
140aaa503: eb 17                       	jmp	0x140aaa51c <.text+0xaa951c>
140aaa505: 48 83 c3 60                 	addq	$0x60, %rbx
140aaa509: 48 89 d9                    	movq	%rbx, %rcx
140aaa50c: 48 89 fa                    	movq	%rdi, %rdx
140aaa50f: e8 dc fd 81 ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
140aaa514: 0f b6 06                    	movzbl	(%rsi), %eax
140aaa517: 43 88 44 3e f8              	movb	%al, -0x8(%r14,%r15)
140aaa51c: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
140aaa523: 5b                          	popq	%rbx
140aaa524: 5f                          	popq	%rdi
140aaa525: 5e                          	popq	%rsi
140aaa526: 41 5e                       	popq	%r14
140aaa528: 41 5f                       	popq	%r15
140aaa52a: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aaa530: 41 57                       	pushq	%r15
140aaa532: 41 56                       	pushq	%r14
140aaa534: 56                          	pushq	%rsi
140aaa535: 57                          	pushq	%rdi
140aaa536: 53                          	pushq	%rbx
140aaa537: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
140aaa53e: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
140aaa543: 48 83 79 18 00              	cmpq	$0x0, 0x18(%rcx)
140aaa548: 0f 84 b0 00 00 00           	je	0x140aaa5fe <.text+0xaa95fe>
140aaa54e: 4c 89 c6                    	movq	%r8, %rsi
140aaa551: 48 89 d7                    	movq	%rdx, %rdi
140aaa554: 48 89 cb                    	movq	%rcx, %rbx
140aaa557: 48 83 c1 20                 	addq	$0x20, %rcx
140aaa55b: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
140aaa560: e8 0b 68 b0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140aaa565: 4c 8b 33                    	movq	(%rbx), %r14
140aaa568: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140aaa56c: 48 89 ca                    	movq	%rcx, %rdx
140aaa56f: 48 21 c2                    	andq	%rax, %rdx
140aaa572: 48 c1 e8 39                 	shrq	$0x39, %rax
140aaa576: 66 0f 6e c0                 	movd	%eax, %xmm0
140aaa57a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140aaa57e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140aaa583: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140aaa588: 49 8d 46 88                 	leaq	-0x78(%r14), %rax
140aaa58c: 45 31 c0                    	xorl	%r8d, %r8d
140aaa58f: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140aaa593: f3 41 0f 6f 14 16           	movdqu	(%r14,%rdx), %xmm2
140aaa599: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140aaa59d: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140aaa5a1: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140aaa5a6: 45 85 c9                    	testl	%r9d, %r9d
140aaa5a9: 74 35                       	je	0x140aaa5e0 <.text+0xaa95e0>
140aaa5ab: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140aaa5b0: 49 01 d2                    	addq	%rdx, %r10
140aaa5b3: 49 21 ca                    	andq	%rcx, %r10
140aaa5b6: 49 f7 da                    	negq	%r10
140aaa5b9: 4d 6b fa 78                 	imulq	$0x78, %r10, %r15
140aaa5bd: 4a 39 3c 38                 	cmpq	%rdi, (%rax,%r15)
140aaa5c1: 0f 84 1e 01 00 00           	je	0x140aaa6e5 <.text+0xaa96e5>
140aaa5c7: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140aaa5cb: 66 45 21 ca                 	andw	%r9w, %r10w
140aaa5cf: 45 89 d1                    	movl	%r10d, %r9d
140aaa5d2: 75 d7                       	jne	0x140aaa5ab <.text+0xaa95ab>
140aaa5d4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140aaa5e0: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140aaa5e4: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140aaa5e9: 45 85 c9                    	testl	%r9d, %r9d
140aaa5ec: 75 10                       	jne	0x140aaa5fe <.text+0xaa95fe>
140aaa5ee: 4c 01 c2                    	addq	%r8, %rdx
140aaa5f1: 48 83 c2 10                 	addq	$0x10, %rdx
140aaa5f5: 49 83 c0 10                 	addq	$0x10, %r8
140aaa5f9: 48 21 ca                    	andq	%rcx, %rdx
140aaa5fc: eb 95                       	jmp	0x140aaa593 <.text+0xaa9593>
140aaa5fe: 48 8b 05 a3 1d b8 02        	movq	0x2b81da3(%rip), %rax   # 0x14362c3a8
140aaa605: 48 8b 00                    	movq	(%rax), %rax
140aaa608: 48 83 f8 02                 	cmpq	$0x2, %rax
140aaa60c: 0f 82 ed 00 00 00           	jb	0x140aaa6ff <.text+0xaa96ff>
140aaa612: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140aaa617: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140aaa61c: 48 8d 05 9d a8 08 00        	leaq	0x8a89d(%rip), %rax     # 0x140b34ec0 <.text+0xb33ec0>
140aaa623: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140aaa628: 48 8d 0d 51 2e 03 02        	leaq	0x2032e51(%rip), %rcx   # 0x142add480
140aaa62f: e8 4c 37 bd ff              	callq	0x14067dd80 <.text+0x67cd80>
140aaa634: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140aaa638: 8b 40 10                    	movl	0x10(%rax), %eax
140aaa63b: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
140aaa644: 48 8d 0d 4d 2e 03 02        	leaq	0x2032e4d(%rip), %rcx   # 0x142add498
140aaa64b: 48 89 4c 24 78              	movq	%rcx, 0x78(%rsp)
140aaa650: 48 c7 84 24 80 00 00 00 2b 00 00 00 	movq	$0x2b, 0x80(%rsp)
140aaa65c: 48 8d 15 c5 2d 03 02        	leaq	0x2032dc5(%rip), %rdx   # 0x142add428
140aaa663: 48 89 94 24 90 00 00 00     	movq	%rdx, 0x90(%rsp)
140aaa66b: 48 c7 84 24 98 00 00 00 02 00 00 00 	movq	$0x2, 0x98(%rsp)
140aaa677: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140aaa67c: 48 89 94 24 a0 00 00 00     	movq	%rdx, 0xa0(%rsp)
140aaa684: 48 c7 84 24 a8 00 00 00 01 00 00 00 	movq	$0x1, 0xa8(%rsp)
140aaa690: 48 c7 84 24 b0 00 00 00 00 00 00 00 	movq	$0x0, 0xb0(%rsp)
140aaa69c: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
140aaa6a5: 48 89 4c 24 48              	movq	%rcx, 0x48(%rsp)
140aaa6aa: 48 c7 44 24 50 2b 00 00 00  	movq	$0x2b, 0x50(%rsp)
140aaa6b3: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
140aaa6bc: f3 0f 7f 44 24 60           	movdqu	%xmm0, 0x60(%rsp)
140aaa6c2: c7 84 24 88 00 00 00 01 00 00 00    	movl	$0x1, 0x88(%rsp)
140aaa6cd: 89 84 24 8c 00 00 00        	movl	%eax, 0x8c(%rsp)
140aaa6d4: 48 8d 4c 24 27              	leaq	0x27(%rsp), %rcx
140aaa6d9: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
140aaa6de: e8 4d fc 29 00              	callq	0x140d4a330 <.text+0xd49330>
140aaa6e3: eb 1a                       	jmp	0x140aaa6ff <.text+0xaa96ff>
140aaa6e5: 48 83 c3 60                 	addq	$0x60, %rbx
140aaa6e9: 48 89 d9                    	movq	%rbx, %rcx
140aaa6ec: 48 89 fa                    	movq	%rdi, %rdx
140aaa6ef: e8 fc fb 81 ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
140aaa6f4: f3 0f 10 06                 	movss	(%rsi), %xmm0
140aaa6f8: f3 43 0f 11 44 3e f0        	movss	%xmm0, -0x10(%r14,%r15)
140aaa6ff: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
140aaa706: 5b                          	popq	%rbx
140aaa707: 5f                          	popq	%rdi
140aaa708: 5e                          	popq	%rsi
140aaa709: 41 5e                       	popq	%r14
140aaa70b: 41 5f                       	popq	%r15
140aaa70d: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa9800: 41 57                       	pushq	%r15
140aa9802: 41 56                       	pushq	%r14
140aa9804: 56                          	pushq	%rsi
140aa9805: 57                          	pushq	%rdi
140aa9806: 53                          	pushq	%rbx
140aa9807: 48 81 ec c0 00 00 00        	subq	$0xc0, %rsp
140aa980e: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
140aa9813: 48 83 79 18 00              	cmpq	$0x0, 0x18(%rcx)
140aa9818: 0f 84 b0 00 00 00           	je	0x140aa98ce <.text+0xaa88ce>
140aa981e: 4c 89 c6                    	movq	%r8, %rsi
140aa9821: 48 89 d7                    	movq	%rdx, %rdi
140aa9824: 48 89 cb                    	movq	%rcx, %rbx
140aa9827: 48 83 c1 20                 	addq	$0x20, %rcx
140aa982b: 48 8d 54 24 28              	leaq	0x28(%rsp), %rdx
140aa9830: e8 3b 75 b0 ff              	callq	0x1405b0d70 <.text+0x5afd70>
140aa9835: 4c 8b 33                    	movq	(%rbx), %r14
140aa9838: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
140aa983c: 48 89 ca                    	movq	%rcx, %rdx
140aa983f: 48 21 c2                    	andq	%rax, %rdx
140aa9842: 48 c1 e8 39                 	shrq	$0x39, %rax
140aa9846: 66 0f 6e c0                 	movd	%eax, %xmm0
140aa984a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140aa984e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140aa9853: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140aa9858: 49 8d 46 88                 	leaq	-0x78(%r14), %rax
140aa985c: 45 31 c0                    	xorl	%r8d, %r8d
140aa985f: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
140aa9863: f3 41 0f 6f 14 16           	movdqu	(%r14,%rdx), %xmm2
140aa9869: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
140aa986d: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
140aa9871: 66 44 0f d7 cb              	pmovmskb	%xmm3, %r9d
140aa9876: 45 85 c9                    	testl	%r9d, %r9d
140aa9879: 74 35                       	je	0x140aa98b0 <.text+0xaa88b0>
140aa987b: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
140aa9880: 49 01 d2                    	addq	%rdx, %r10
140aa9883: 49 21 ca                    	andq	%rcx, %r10
140aa9886: 49 f7 da                    	negq	%r10
140aa9889: 4d 6b fa 78                 	imulq	$0x78, %r10, %r15
140aa988d: 4a 39 3c 38                 	cmpq	%rdi, (%rax,%r15)
140aa9891: 0f 84 1e 01 00 00           	je	0x140aa99b5 <.text+0xaa89b5>
140aa9897: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
140aa989b: 66 45 21 ca                 	andw	%r9w, %r10w
140aa989f: 45 89 d1                    	movl	%r10d, %r9d
140aa98a2: 75 d7                       	jne	0x140aa987b <.text+0xaa887b>
140aa98a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140aa98b0: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
140aa98b4: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
140aa98b9: 45 85 c9                    	testl	%r9d, %r9d
140aa98bc: 75 10                       	jne	0x140aa98ce <.text+0xaa88ce>
140aa98be: 4c 01 c2                    	addq	%r8, %rdx
140aa98c1: 48 83 c2 10                 	addq	$0x10, %rdx
140aa98c5: 49 83 c0 10                 	addq	$0x10, %r8
140aa98c9: 48 21 ca                    	andq	%rcx, %rdx
140aa98cc: eb 95                       	jmp	0x140aa9863 <.text+0xaa8863>
140aa98ce: 48 8b 05 d3 2a b8 02        	movq	0x2b82ad3(%rip), %rax   # 0x14362c3a8
140aa98d5: 48 8b 00                    	movq	(%rax), %rax
140aa98d8: 48 83 f8 02                 	cmpq	$0x2, %rax
140aa98dc: 0f 82 ea 00 00 00           	jb	0x140aa99cc <.text+0xaa89cc>
140aa98e2: 48 8d 44 24 28              	leaq	0x28(%rsp), %rax
140aa98e7: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
140aa98ec: 48 8d 05 cd b5 08 00        	leaq	0x8b5cd(%rip), %rax     # 0x140b34ec0 <.text+0xb33ec0>
140aa98f3: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140aa98f8: 48 8d 0d 81 3b 03 02        	leaq	0x2033b81(%rip), %rcx   # 0x142add480
140aa98ff: e8 7c 44 bd ff              	callq	0x14067dd80 <.text+0x67cd80>
140aa9904: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
140aa9908: 8b 40 10                    	movl	0x10(%rax), %eax
140aa990b: 48 c7 44 24 70 02 00 00 00  	movq	$0x2, 0x70(%rsp)
140aa9914: 48 8d 0d 7d 3b 03 02        	leaq	0x2033b7d(%rip), %rcx   # 0x142add498
140aa991b: 48 89 4c 24 78              	movq	%rcx, 0x78(%rsp)
140aa9920: 48 c7 84 24 80 00 00 00 2b 00 00 00 	movq	$0x2b, 0x80(%rsp)
140aa992c: 48 8d 15 f5 3a 03 02        	leaq	0x2033af5(%rip), %rdx   # 0x142add428
140aa9933: 48 89 94 24 90 00 00 00     	movq	%rdx, 0x90(%rsp)
140aa993b: 48 c7 84 24 98 00 00 00 02 00 00 00 	movq	$0x2, 0x98(%rsp)
140aa9947: 48 8d 54 24 30              	leaq	0x30(%rsp), %rdx
140aa994c: 48 89 94 24 a0 00 00 00     	movq	%rdx, 0xa0(%rsp)
140aa9954: 48 c7 84 24 a8 00 00 00 01 00 00 00 	movq	$0x1, 0xa8(%rsp)
140aa9960: 48 c7 84 24 b0 00 00 00 00 00 00 00 	movq	$0x0, 0xb0(%rsp)
140aa996c: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
140aa9975: 48 89 4c 24 48              	movq	%rcx, 0x48(%rsp)
140aa997a: 48 c7 44 24 50 2b 00 00 00  	movq	$0x2b, 0x50(%rsp)
140aa9983: 48 c7 44 24 58 00 00 00 00  	movq	$0x0, 0x58(%rsp)
140aa998c: f3 0f 7f 44 24 60           	movdqu	%xmm0, 0x60(%rsp)
140aa9992: c7 84 24 88 00 00 00 01 00 00 00    	movl	$0x1, 0x88(%rsp)
140aa999d: 89 84 24 8c 00 00 00        	movl	%eax, 0x8c(%rsp)
140aa99a4: 48 8d 4c 24 27              	leaq	0x27(%rsp), %rcx
140aa99a9: 48 8d 54 24 40              	leaq	0x40(%rsp), %rdx
140aa99ae: e8 7d 09 2a 00              	callq	0x140d4a330 <.text+0xd49330>
140aa99b3: eb 17                       	jmp	0x140aa99cc <.text+0xaa89cc>
140aa99b5: 48 83 c3 60                 	addq	$0x60, %rbx
140aa99b9: 48 89 d9                    	movq	%rbx, %rcx
140aa99bc: 48 89 fa                    	movq	%rdi, %rdx
140aa99bf: e8 2c 09 82 ff              	callq	0x1402ca2f0 <.text+0x2c92f0>
140aa99c4: 0f b6 06                    	movzbl	(%rsi), %eax
140aa99c7: 43 88 44 3e f7              	movb	%al, -0x9(%r14,%r15)
140aa99cc: 48 81 c4 c0 00 00 00        	addq	$0xc0, %rsp
140aa99d3: 5b                          	popq	%rbx
140aa99d4: 5f                          	popq	%rdi
140aa99d5: 5e                          	popq	%rsi
140aa99d6: 41 5e                       	popq	%r14
140aa99d8: 41 5f                       	popq	%r15
140aa99da: c3                          	retq

D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140abfb30: 55                          	pushq	%rbp
140abfb31: 41 57                       	pushq	%r15
140abfb33: 41 56                       	pushq	%r14
140abfb35: 41 55                       	pushq	%r13
140abfb37: 41 54                       	pushq	%r12
140abfb39: 56                          	pushq	%rsi
140abfb3a: 57                          	pushq	%rdi
140abfb3b: 53                          	pushq	%rbx
140abfb3c: 48 81 ec e8 02 00 00        	subq	$0x2e8, %rsp            # imm = 0x2E8
140abfb43: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140abfb4b: 44 0f 29 b5 50 02 00 00     	movaps	%xmm14, 0x250(%rbp)
140abfb53: 44 0f 29 ad 40 02 00 00     	movaps	%xmm13, 0x240(%rbp)
140abfb5b: 44 0f 29 a5 30 02 00 00     	movaps	%xmm12, 0x230(%rbp)
140abfb63: 44 0f 29 9d 20 02 00 00     	movaps	%xmm11, 0x220(%rbp)
140abfb6b: 44 0f 29 95 10 02 00 00     	movaps	%xmm10, 0x210(%rbp)
140abfb73: 44 0f 29 8d 00 02 00 00     	movaps	%xmm9, 0x200(%rbp)
140abfb7b: 44 0f 29 85 f0 01 00 00     	movaps	%xmm8, 0x1f0(%rbp)
140abfb83: 0f 29 bd e0 01 00 00        	movaps	%xmm7, 0x1e0(%rbp)
140abfb8a: 0f 29 b5 d0 01 00 00        	movaps	%xmm6, 0x1d0(%rbp)
140abfb91: 48 c7 85 c8 01 00 00 fe ff ff ff    	movq	$-0x2, 0x1c8(%rbp)
140abfb9c: 80 79 15 00                 	cmpb	$0x0, 0x15(%rcx)
140abfba0: 0f 85 37 0e 00 00           	jne	0x140ac09dd <.text+0xabf9dd>
140abfba6: 48 89 ce                    	movq	%rcx, %rsi
140abfba9: 48 8b 9d e8 02 00 00        	movq	0x2e8(%rbp), %rbx
140abfbb0: 48 8b bd e0 02 00 00        	movq	0x2e0(%rbp), %rdi
140abfbb7: f6 01 01                    	testb	$0x1, (%rcx)
140abfbba: 49 be 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r14 # imm = 0x8000000000000001
140abfbc4: 49 bf 10 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffff0, %r15 # imm = 0x8000000000000010
140abfbce: 74 79                       	je	0x140abfc49 <.text+0xabec49>
140abfbd0: 4d 89 cc                    	movq	%r9, %r12
140abfbd3: 44 0f 28 e1                 	movaps	%xmm1, %xmm12
140abfbd7: 4c 89 85 b8 01 00 00        	movq	%r8, 0x1b8(%rbp)
140abfbde: f3 44 0f 10 5e 10           	movss	0x10(%rsi), %xmm11
140abfbe4: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
140abfbe8: f3 0f 5e 46 04              	divss	0x4(%rsi), %xmm0
140abfbed: 0f 57 c9                    	xorps	%xmm1, %xmm1
140abfbf0: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
140abfbf4: f3 0f 10 3d 08 5d e6 01     	movss	0x1e65d08(%rip), %xmm7  # 0x142925904
140abfbfc: 0f 28 c7                    	movaps	%xmm7, %xmm0
140abfbff: f3 0f 5d c1                 	minss	%xmm1, %xmm0
140abfc03: e8 d8 65 1d 00              	callq	0x140c961e0 <.text+0xc951e0>
140abfc08: 4c 8b 6e 08                 	movq	0x8(%rsi), %r13
140abfc0c: 49 8b 45 10                 	movq	0x10(%r13), %rax
140abfc10: 49 01 c6                    	addq	%rax, %r14
140abfc13: 31 c9                       	xorl	%ecx, %ecx
140abfc15: 4c 39 f8                    	cmpq	%r15, %rax
140abfc18: 49 0f 4c ce                 	cmovlq	%r14, %rcx
140abfc1c: 0f 28 f0                    	movaps	%xmm0, %xmm6
140abfc1f: 48 8d 05 26 f8 01 02        	leaq	0x201f826(%rip), %rax   # 0x142adf44c
140abfc26: 48 63 0c 88                 	movslq	(%rax,%rcx,4), %rcx
140abfc2a: 48 01 c1                    	addq	%rax, %rcx
140abfc2d: ff e1                       	jmpq	*%rcx
140abfc2f: 4d 8d 45 18                 	leaq	0x18(%r13), %r8
140abfc33: 4d 8d 4d 50                 	leaq	0x50(%r13), %r9
140abfc37: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140abfc3b: 0f 85 1f 02 00 00           	jne	0x140abfe60 <.text+0xabee60>
140abfc41: 0f 28 c6                    	movaps	%xmm6, %xmm0
140abfc44: e9 1e 02 00 00              	jmp	0x140abfe67 <.text+0xabee67>
140abfc49: 4c 8b ad d8 02 00 00        	movq	0x2d8(%rbp), %r13
140abfc50: c6 46 15 01                 	movb	$0x1, 0x15(%rsi)
140abfc54: 4c 8b 66 08                 	movq	0x8(%rsi), %r12
140abfc58: 49 8b 44 24 10              	movq	0x10(%r12), %rax
140abfc5d: 49 01 c6                    	addq	%rax, %r14
140abfc60: 31 c9                       	xorl	%ecx, %ecx
140abfc62: 4c 39 f8                    	cmpq	%r15, %rax
140abfc65: 49 0f 4c ce                 	cmovlq	%r14, %rcx
140abfc69: 4c 8b b5 d0 02 00 00        	movq	0x2d0(%rbp), %r14
140abfc70: 48 8d 05 91 f7 01 02        	leaq	0x201f791(%rip), %rax   # 0x142adf408
140abfc77: 48 63 0c 88                 	movslq	(%rax,%rcx,4), %rcx
140abfc7b: 48 01 c1                    	addq	%rax, %rcx
140abfc7e: ff e1                       	jmpq	*%rcx
140abfc80: 49 8d 44 24 18              	leaq	0x18(%r12), %rax
140abfc85: 4d 8d 4c 24 50              	leaq	0x50(%r12), %r9
140abfc8a: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140abfc8e: 0f 57 c0                    	xorps	%xmm0, %xmm0
140abfc91: 75 08                       	jne	0x140abfc9b <.text+0xabec9b>
140abfc93: f3 0f 10 05 69 5c e6 01     	movss	0x1e65c69(%rip), %xmm0  # 0x142925904
140abfc9b: 49 8b 8c 24 88 00 00 00     	movq	0x88(%r12), %rcx
140abfca3: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140abfca9: 4c 89 c2                    	movq	%r8, %rdx
140abfcac: 49 89 c0                    	movq	%rax, %r8
140abfcaf: e8 bc 28 e9 ff              	callq	0x140952570 <.text+0x951570>
140abfcb4: e9 24 0d 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140abfcb9: 4d 8d 7d 20                 	leaq	0x20(%r13), %r15
140abfcbd: 4d 8d 65 3c                 	leaq	0x3c(%r13), %r12
140abfcc1: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140abfcc5: 0f 85 58 02 00 00           	jne	0x140abff23 <.text+0xabef23>
140abfccb: 0f 28 fe                    	movaps	%xmm6, %xmm7
140abfcce: e9 54 02 00 00              	jmp	0x140abff27 <.text+0xabef27>
140abfcd3: 4d 8d 75 10                 	leaq	0x10(%r13), %r14
140abfcd7: 4d 8d bd 80 00 00 00        	leaq	0x80(%r13), %r15
140abfcde: 0f 2e fe                    	ucomiss	%xmm6, %xmm7
140abfce1: 0f 96 c1                    	setbe	%cl
140abfce4: 41 8b 85 f8 00 00 00        	movl	0xf8(%r13), %eax
140abfceb: 83 f8 02                    	cmpl	$0x2, %eax
140abfcee: 0f 94 c2                    	sete	%dl
140abfcf1: 08 ca                       	orb	%cl, %dl
140abfcf3: 0f 84 07 04 00 00           	je	0x140ac0100 <.text+0xabf100>
140abfcf9: 49 8b 9d f0 00 00 00        	movq	0xf0(%r13), %rbx
140abfd00: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140abfd04: 0f 84 ed 09 00 00           	je	0x140ac06f7 <.text+0xabf6f7>
140abfd0a: 4c 89 f2                    	movq	%r14, %rdx
140abfd0d: 4c 8d 35 e4 f0 01 02        	leaq	0x201f0e4(%rip), %r14   # 0x142adedf8
140abfd14: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
140abfd1b: 4d 89 f0                    	movq	%r14, %r8
140abfd1e: e8 4d 7c 83 00              	callq	0x1412f7970 <.text+0x12f6970>
140abfd23: 49 8d 55 28                 	leaq	0x28(%r13), %rdx
140abfd27: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140abfd2e: 4d 89 f0                    	movq	%r14, %r8
140abfd31: e8 8a 7b 83 00              	callq	0x1412f78c0 <.text+0x12f68c0>
140abfd36: f3 41 0f 10 45 40           	movss	0x40(%r13), %xmm0
140abfd3c: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140abfd43: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
140abfd4a: 0f 10 8d a0 00 00 00        	movups	0xa0(%rbp), %xmm1
140abfd51: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140abfd58: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
140abfd5f: 0f 11 8d 58 01 00 00        	movups	%xmm1, 0x158(%rbp)
140abfd66: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140abfd6d: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140abfd74: 41 8b 55 48                 	movl	0x48(%r13), %edx
140abfd78: 44 8d 42 fe                 	leal	-0x2(%rdx), %r8d
140abfd7c: 41 83 f8 02                 	cmpl	$0x2, %r8d
140abfd80: b8 02 00 00 00              	movl	$0x2, %eax
140abfd85: 44 0f 43 c0                 	cmovael	%eax, %r8d
140abfd89: 45 85 c0                    	testl	%r8d, %r8d
140abfd8c: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140abfd93: 0f 84 74 0e 00 00           	je	0x140ac0c0d <.text+0xabfc0d>
140abfd99: 41 83 f8 02                 	cmpl	$0x2, %r8d
140abfd9d: 0f 85 65 0e 00 00           	jne	0x140ac0c08 <.text+0xabfc08>
140abfda3: 49 8b 45 5c                 	movq	0x5c(%r13), %rax
140abfda7: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140abfdab: 41 0f 10 4d 4c              	movups	0x4c(%r13), %xmm1
140abfdb0: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
140abfdb4: 89 d0                       	movl	%edx, %eax
140abfdb6: e9 52 0e 00 00              	jmp	0x140ac0c0d <.text+0xabfc0d>
140abfdbb: 49 8b 7d 18                 	movq	0x18(%r13), %rdi
140abfdbf: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140abfdc3: f3 0f 5c fe                 	subss	%xmm6, %xmm7
140abfdc7: f3 41 0f 59 7c 85 20        	mulss	0x20(%r13,%rax,4), %xmm7
140abfdce: 34 01                       	xorb	$0x1, %al
140abfdd0: 0f b6 c0                    	movzbl	%al, %eax
140abfdd3: f3 45 0f 10 44 85 20        	movss	0x20(%r13,%rax,4), %xmm8
140abfdda: f3 44 0f 59 c6              	mulss	%xmm6, %xmm8
140abfddf: f3 44 0f 58 c7              	addss	%xmm7, %xmm8
140abfde4: 41 8b 44 24 1c              	movl	0x1c(%r12), %eax
140abfde9: 49 8b 4c 24 10              	movq	0x10(%r12), %rcx
140abfdee: 89 01                       	movl	%eax, (%rcx)
140abfdf0: 49 8b 1c 24                 	movq	(%r12), %rbx
140abfdf4: 4c 8b 73 30                 	movq	0x30(%rbx), %r14
140abfdf8: 4c 8b 7b 40                 	movq	0x40(%rbx), %r15
140abfdfc: 4c 3b 73 20                 	cmpq	0x20(%rbx), %r14
140abfe00: 75 10                       	jne	0x140abfe12 <.text+0xabee12>
140abfe02: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140abfe06: 48 8d 15 3b a3 17 02        	leaq	0x217a33b(%rip), %rdx   # 0x142c3a148
140abfe0d: e8 de 2b aa ff              	callq	0x1405629f0 <.text+0x5619f0>
140abfe12: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140abfe16: 4b 8d 0c 76                 	leaq	(%r14,%r14,2), %rcx
140abfe1a: 48 c1 e1 04                 	shlq	$0x4, %rcx
140abfe1e: c7 04 08 00 00 00 00        	movl	$0x0, (%rax,%rcx)
140abfe25: c7 44 08 10 01 00 00 00     	movl	$0x1, 0x10(%rax,%rcx)
140abfe2d: f3 44 0f 11 44 08 14        	movss	%xmm8, 0x14(%rax,%rcx)
140abfe34: 48 89 7c 08 18              	movq	%rdi, 0x18(%rax,%rcx)
140abfe39: c6 44 08 20 01              	movb	$0x1, 0x20(%rax,%rcx)
140abfe3e: 4c 89 7c 08 28              	movq	%r15, 0x28(%rax,%rcx)
140abfe43: 49 ff c6                    	incq	%r14
140abfe46: 4c 89 73 30                 	movq	%r14, 0x30(%rbx)
140abfe4a: 48 ff 43 40                 	incq	0x40(%rbx)
140abfe4e: 0f 2e 35 af 5a e6 01        	ucomiss	0x1e65aaf(%rip), %xmm6  # 0x142925904
140abfe55: 0f 83 bf 00 00 00           	jae	0x140abff1a <.text+0xabef1a>
140abfe5b: e9 72 0b 00 00              	jmp	0x140ac09d2 <.text+0xabf9d2>
140abfe60: 0f 28 c7                    	movaps	%xmm7, %xmm0
140abfe63: f3 0f 5c c6                 	subss	%xmm6, %xmm0
140abfe67: 48 8b 95 b8 01 00 00        	movq	0x1b8(%rbp), %rdx
140abfe6e: 49 8b 8d 88 00 00 00        	movq	0x88(%r13), %rcx
140abfe75: f3 0f 11 44 24 20           	movss	%xmm0, 0x20(%rsp)
140abfe7b: e8 f0 26 e9 ff              	callq	0x140952570 <.text+0x951570>
140abfe80: e9 8c 00 00 00              	jmp	0x140abff11 <.text+0xabef11>
140abfe85: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140abfe89: 0f 28 c7                    	movaps	%xmm7, %xmm0
140abfe8c: f3 0f 5c c6                 	subss	%xmm6, %xmm0
140abfe90: f3 41 0f 59 44 85 20        	mulss	0x20(%r13,%rax,4), %xmm0
140abfe97: 34 01                       	xorb	$0x1, %al
140abfe99: 0f b6 c0                    	movzbl	%al, %eax
140abfe9c: f3 41 0f 10 4c 85 20        	movss	0x20(%r13,%rax,4), %xmm1
140abfea3: f3 0f 59 ce                 	mulss	%xmm6, %xmm1
140abfea7: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140abfeab: 49 8b 55 18                 	movq	0x18(%r13), %rdx
140abfeaf: f3 0f 11 8d 40 01 00 00     	movss	%xmm1, 0x140(%rbp)
140abfeb7: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140abfebe: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140abfec5: e8 f6 ab fe ff              	callq	0x140aaaac0 <.text+0xaa9ac0>
140abfeca: eb 45                       	jmp	0x140abff11 <.text+0xabef11>
140abfecc: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140abfed0: 0f 28 c7                    	movaps	%xmm7, %xmm0
140abfed3: f3 0f 5c c6                 	subss	%xmm6, %xmm0
140abfed7: f3 41 0f 59 44 85 20        	mulss	0x20(%r13,%rax,4), %xmm0
140abfede: 34 01                       	xorb	$0x1, %al
140abfee0: 0f b6 c0                    	movzbl	%al, %eax
140abfee3: f3 41 0f 10 4c 85 20        	movss	0x20(%r13,%rax,4), %xmm1
140abfeea: f3 0f 59 ce                 	mulss	%xmm6, %xmm1
140abfeee: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140abfef2: 49 8b 55 18                 	movq	0x18(%r13), %rdx
140abfef6: f3 0f 11 8d 40 01 00 00     	movss	%xmm1, 0x140(%rbp)
140abfefe: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140abff05: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140abff0c: e8 ff a7 fe ff              	callq	0x140aaa710 <.text+0xaa9710>
140abff11: 0f 2e f7                    	ucomiss	%xmm7, %xmm6
140abff14: 0f 82 b8 0a 00 00           	jb	0x140ac09d2 <.text+0xabf9d2>
140abff1a: c6 46 15 01                 	movb	$0x1, 0x15(%rsi)
140abff1e: e9 af 0a 00 00              	jmp	0x140ac09d2 <.text+0xabf9d2>
140abff23: f3 0f 5c fe                 	subss	%xmm6, %xmm7
140abff27: 4c 89 e0                    	movq	%r12, %rax
140abff2a: 49 0f 45 c7                 	cmovneq	%r15, %rax
140abff2e: 8b 08                       	movl	(%rax), %ecx
140abff30: 48 8b 44 c8 04              	movq	0x4(%rax,%rcx,8), %rax
140abff35: 48 89 85 88 00 00 00        	movq	%rax, 0x88(%rbp)
140abff3c: 49 8b 45 18                 	movq	0x18(%r13), %rax
140abff40: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140abff44: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
140abff4b: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
140abff52: e8 69 ce 1d 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
140abff57: f3 44 0f 10 85 40 01 00 00  	movss	0x140(%rbp), %xmm8
140abff60: f3 44 0f 10 ad 44 01 00 00  	movss	0x144(%rbp), %xmm13
140abff69: f3 44 0f 10 8d 48 01 00 00  	movss	0x148(%rbp), %xmm9
140abff72: 48 8b 0f                    	movq	(%rdi), %rcx
140abff75: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
140abff79: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
140abff7d: e8 be 96 f2 ff              	callq	0x1409e9640 <.text+0x9e8640>
140abff82: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
140abff86: f3 45 0f 58 d5              	addss	%xmm13, %xmm10
140abff8b: 8b 43 1c                    	movl	0x1c(%rbx), %eax
140abff8e: 48 8b 4b 10                 	movq	0x10(%rbx), %rcx
140abff92: 89 01                       	movl	%eax, (%rcx)
140abff94: 48 8b 1b                    	movq	(%rbx), %rbx
140abff97: 48 8b 7b 30                 	movq	0x30(%rbx), %rdi
140abff9b: 4c 8b 73 40                 	movq	0x40(%rbx), %r14
140abff9f: 48 3b 7b 20                 	cmpq	0x20(%rbx), %rdi
140abffa3: 75 10                       	jne	0x140abffb5 <.text+0xabefb5>
140abffa5: 48 8d 4b 20                 	leaq	0x20(%rbx), %rcx
140abffa9: 48 8d 15 98 a1 17 02        	leaq	0x217a198(%rip), %rdx   # 0x142c3a148
140abffb0: e8 7b 2c aa ff              	callq	0x140562c30 <.text+0x561c30>
140abffb5: 48 8b 43 28                 	movq	0x28(%rbx), %rax
140abffb9: 48 89 f9                    	movq	%rdi, %rcx
140abffbc: 48 c1 e1 05                 	shlq	$0x5, %rcx
140abffc0: 4c 89 34 08                 	movq	%r14, (%rax,%rcx)
140abffc4: 48 8b 55 58                 	movq	0x58(%rbp), %rdx
140abffc8: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
140abffcd: f3 44 0f 11 44 08 10        	movss	%xmm8, 0x10(%rax,%rcx)
140abffd4: f3 44 0f 11 54 08 14        	movss	%xmm10, 0x14(%rax,%rcx)
140abffdb: f3 44 0f 11 4c 08 18        	movss	%xmm9, 0x18(%rax,%rcx)
140abffe2: 66 c7 44 08 1c 00 01        	movw	$0x100, 0x1c(%rax,%rcx) # imm = 0x100
140abffe9: 48 ff c7                    	incq	%rdi
140abffec: 48 89 7b 30                 	movq	%rdi, 0x30(%rbx)
140abfff0: 48 ff 43 40                 	incq	0x40(%rbx)
140abfff4: 41 0f b6 04 24              	movzbl	(%r12), %eax
140abfff9: 41 f6 07 01                 	testb	$0x1, (%r15)
140abfffd: 74 27                       	je	0x140ac0026 <.text+0xabf026>
140abffff: a8 01                       	testb	$0x1, %al
140ac0001: 0f 84 ce 15 00 00           	je	0x140ac15d5 <.text+0xac05d5>
140ac0007: 49 8d 55 24                 	leaq	0x24(%r13), %rdx
140ac000b: 4d 8d 45 40                 	leaq	0x40(%r13), %r8
140ac000f: 48 8d 8d d4 00 00 00        	leaq	0xd4(%rbp), %rcx
140ac0016: 0f 28 df                    	movaps	%xmm7, %xmm3
140ac0019: e8 12 c7 1d 00              	callq	0x140c9c730 <.text+0xc9b730>
140ac001e: 41 be 01 00 00 00           	movl	$0x1, %r14d
140ac0024: eb 57                       	jmp	0x140ac007d <.text+0xabf07d>
140ac0026: a8 01                       	testb	$0x1, %al
140ac0028: 0f 85 a7 15 00 00           	jne	0x140ac15d5 <.text+0xac05d5>
140ac002e: 49 8d 55 24                 	leaq	0x24(%r13), %rdx
140ac0032: 4d 8d 45 40                 	leaq	0x40(%r13), %r8
140ac0036: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
140ac003d: 0f 28 df                    	movaps	%xmm7, %xmm3
140ac0040: e8 2b 81 1d 00              	callq	0x140c98170 <.text+0xc97170>
140ac0045: f3 41 0f 10 45 30           	movss	0x30(%r13), %xmm0
140ac004b: f3 41 0f 10 4d 4c           	movss	0x4c(%r13), %xmm1
140ac0051: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140ac0054: 75 02                       	jne	0x140ac0058 <.text+0xabf058>
140ac0056: 7b 14                       	jnp	0x140ac006c <.text+0xabf06c>
140ac0058: 49 8d 4d 30                 	leaq	0x30(%r13), %rcx
140ac005c: 0f 28 d7                    	movaps	%xmm7, %xmm2
140ac005f: e8 cc 32 c3 00              	callq	0x1416f3330 <.text+0x16f2330>
140ac0064: f3 0f 11 85 ac 00 00 00     	movss	%xmm0, 0xac(%rbp)
140ac006c: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
140ac0073: 0f 11 85 d4 00 00 00        	movups	%xmm0, 0xd4(%rbp)
140ac007a: 45 31 f6                    	xorl	%r14d, %r14d
140ac007d: 44 89 b5 d0 00 00 00        	movl	%r14d, 0xd0(%rbp)
140ac0084: 0f 2e 35 79 58 e6 01        	ucomiss	0x1e65879(%rip), %xmm6  # 0x142925904
140ac008b: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140ac0092: 72 04                       	jb	0x140ac0098 <.text+0xabf098>
140ac0094: c6 46 15 01                 	movb	$0x1, 0x15(%rsi)
140ac0098: 49 8b 55 18                 	movq	0x18(%r13), %rdx
140ac009c: 8b 85 d0 00 00 00           	movl	0xd0(%rbp), %eax
140ac00a2: 4c 8b 85 d4 00 00 00        	movq	0xd4(%rbp), %r8
140ac00a9: 44 8b 8d dc 00 00 00        	movl	0xdc(%rbp), %r9d
140ac00b0: 0f 10 85 dc 00 00 00        	movups	0xdc(%rbp), %xmm0
140ac00b7: 0f 11 85 4c 01 00 00        	movups	%xmm0, 0x14c(%rbp)
140ac00be: 89 85 40 01 00 00           	movl	%eax, 0x140(%rbp)
140ac00c4: 4c 89 85 44 01 00 00        	movq	%r8, 0x144(%rbp)
140ac00cb: 44 89 8d 4c 01 00 00        	movl	%r9d, 0x14c(%rbp)
140ac00d2: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac00d9: e8 32 95 fe ff              	callq	0x140aa9610 <.text+0xaa8610>
140ac00de: 49 8b 5d 18                 	movq	0x18(%r13), %rbx
140ac00e2: 48 8d 95 d4 00 00 00        	leaq	0xd4(%rbp), %rdx
140ac00e9: 48 8d 4d b8                 	leaq	-0x48(%rbp), %rcx
140ac00ed: 45 85 f6                    	testl	%r14d, %r14d
140ac00f0: 0f 84 09 01 00 00           	je	0x140ac01ff <.text+0xabf1ff>
140ac00f6: e8 f5 8b 1d 00              	callq	0x140c98cf0 <.text+0xc97cf0>
140ac00fb: e9 0c 01 00 00              	jmp	0x140ac020c <.text+0xabf20c>
140ac0100: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140ac0104: 74 0a                       	je	0x140ac0110 <.text+0xabf110>
140ac0106: 0f 28 c7                    	movaps	%xmm7, %xmm0
140ac0109: f3 0f 5c c6                 	subss	%xmm6, %xmm0
140ac010d: 0f 28 f0                    	movaps	%xmm0, %xmm6
140ac0110: f3 45 0f 10 55 64           	movss	0x64(%r13), %xmm10
140ac0116: f3 45 0f 10 ad d4 00 00 00  	movss	0xd4(%r13), %xmm13
140ac011f: 45 0f 2e d5                 	ucomiss	%xmm13, %xmm10
140ac0123: 75 02                       	jne	0x140ac0127 <.text+0xabf127>
140ac0125: 7b 66                       	jnp	0x140ac018d <.text+0xabf18d>
140ac0127: 49 8b 85 f0 00 00 00        	movq	0xf0(%r13), %rax
140ac012e: 0f 28 c7                    	movaps	%xmm7, %xmm0
140ac0131: f3 0f 5c c6                 	subss	%xmm6, %xmm0
140ac0135: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
140ac013a: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
140ac013e: f3 0f 59 ce                 	mulss	%xmm6, %xmm1
140ac0142: f3 0f 58 c8                 	addss	%xmm0, %xmm1
140ac0146: c7 85 40 01 00 00 00 00 00 00       	movl	$0x0, 0x140(%rbp)
140ac0150: c6 85 60 01 00 00 01        	movb	$0x1, 0x160(%rbp)
140ac0157: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
140ac015e: c7 85 50 01 00 00 01 00 00 00       	movl	$0x1, 0x150(%rbp)
140ac0168: f3 0f 11 8d 54 01 00 00     	movss	%xmm1, 0x154(%rbp)
140ac0170: 4c 8d 05 e1 ee 01 02        	leaq	0x201eee1(%rip), %r8    # 0x142adf058
140ac0177: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac017e: 4c 89 e1                    	movq	%r12, %rcx
140ac0181: e8 8a 70 86 00              	callq	0x141327210 <.text+0x1326210>
140ac0186: 41 8b 85 f8 00 00 00        	movl	0xf8(%r13), %eax
140ac018d: 85 c0                       	testl	%eax, %eax
140ac018f: 0f 84 22 06 00 00           	je	0x140ac07b7 <.text+0xabf7b7>
140ac0195: 83 f8 01                    	cmpl	$0x1, %eax
140ac0198: 0f 85 78 14 00 00           	jne	0x140ac1616 <.text+0xac0616>
140ac019e: f3 45 0f 10 85 fc 00 00 00  	movss	0xfc(%r13), %xmm8
140ac01a7: f3 41 0f 10 85 00 01 00 00  	movss	0x100(%r13), %xmm0
140ac01b0: 41 0f 2e c0                 	ucomiss	%xmm8, %xmm0
140ac01b4: 0f 86 8c 13 00 00           	jbe	0x140ac1546 <.text+0xac0546>
140ac01ba: 4d 8d a5 00 01 00 00        	leaq	0x100(%r13), %r12
140ac01c1: 44 0f 28 f7                 	movaps	%xmm7, %xmm14
140ac01c5: f3 44 0f 5c f6              	subss	%xmm6, %xmm14
140ac01ca: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ac01cd: f3 41 0f 59 c6              	mulss	%xmm14, %xmm0
140ac01d2: f3 44 0f 59 c6              	mulss	%xmm6, %xmm8
140ac01d7: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
140ac01dc: 48 8d 15 1d ef 01 02        	leaq	0x201ef1d(%rip), %rdx   # 0x142adf100
140ac01e3: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
140ac01e7: e8 44 14 e2 ff              	callq	0x1408e1630 <.text+0x8e0630>
140ac01ec: 48 8d 15 25 ef 01 02        	leaq	0x201ef25(%rip), %rdx   # 0x142adf118
140ac01f3: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
140ac01f7: 4d 89 f7                    	movq	%r14, %r15
140ac01fa: e9 11 06 00 00              	jmp	0x140ac0810 <.text+0xabf810>
140ac01ff: f3 0f 10 15 61 ce f0 01     	movss	0x1f0ce61(%rip), %xmm2  # 0x1429cd068
140ac0207: e8 e4 7a 1d 00              	callq	0x140c97cf0 <.text+0xc96cf0>
140ac020c: 4c 8d 45 b8                 	leaq	-0x48(%rbp), %r8
140ac0210: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140ac0217: 48 89 da                    	movq	%rbx, %rdx
140ac021a: e8 71 9e fe ff              	callq	0x140aaa090 <.text+0xaa9090>
140ac021f: e9 ae 07 00 00              	jmp	0x140ac09d2 <.text+0xabf9d2>
140ac0224: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac0229: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac022d: 48 83 f0 21                 	xorq	$0x21, %rax
140ac0231: 41 0f b6 04 04              	movzbl	(%r12,%rax), %eax
140ac0236: 88 85 40 01 00 00           	movb	%al, 0x140(%rbp)
140ac023c: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac0243: 4c 89 c1                    	movq	%r8, %rcx
140ac0246: 49 89 c0                    	movq	%rax, %r8
140ac0249: e8 b2 95 fe ff              	callq	0x140aa9800 <.text+0xaa8800>
140ac024e: e9 8a 07 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0253: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac0258: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140ac025c: 0f 84 d5 07 00 00           	je	0x140ac0a37 <.text+0xabfa37>
140ac0262: 41 8b 44 24 20              	movl	0x20(%r12), %eax
140ac0267: 44 8d 48 fe                 	leal	-0x2(%rax), %r9d
140ac026b: 41 83 f9 02                 	cmpl	$0x2, %r9d
140ac026f: b9 02 00 00 00              	movl	$0x2, %ecx
140ac0274: 44 0f 43 c9                 	cmovael	%ecx, %r9d
140ac0278: 45 85 c9                    	testl	%r9d, %r9d
140ac027b: 0f 84 4e 09 00 00           	je	0x140ac0bcf <.text+0xabfbcf>
140ac0281: 41 83 f9 02                 	cmpl	$0x2, %r9d
140ac0285: 0f 85 3f 09 00 00           	jne	0x140ac0bca <.text+0xabfbca>
140ac028b: 49 8b 4c 24 34              	movq	0x34(%r12), %rcx
140ac0290: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140ac0297: 41 0f 10 44 24 24           	movups	0x24(%r12), %xmm0
140ac029d: e9 d0 07 00 00              	jmp	0x140ac0a72 <.text+0xabfa72>
140ac02a2: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140ac02a6: 0f 85 cb 02 00 00           	jne	0x140ac0577 <.text+0xabf577>
140ac02ac: 4c 89 c6                    	movq	%r8, %rsi
140ac02af: 31 c0                       	xorl	%eax, %eax
140ac02b1: 49 3b 44 24 18              	cmpq	0x18(%r12), %rax
140ac02b6: 0f 80 a1 0d 00 00           	jo	0x140ac105d <.text+0xac005d>
140ac02bc: 49 8d 54 24 18              	leaq	0x18(%r12), %rdx
140ac02c1: 48 8d 1d 30 eb 01 02        	leaq	0x201eb30(%rip), %rbx   # 0x142adedf8
140ac02c8: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140ac02cf: 49 89 d8                    	movq	%rbx, %r8
140ac02d2: e8 99 76 83 00              	callq	0x1412f7970 <.text+0x12f6970>
140ac02d7: 49 8d 54 24 30              	leaq	0x30(%r12), %rdx
140ac02dc: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac02e3: 49 89 d8                    	movq	%rbx, %r8
140ac02e6: e8 d5 75 83 00              	callq	0x1412f78c0 <.text+0x12f68c0>
140ac02eb: f3 41 0f 10 44 24 48        	movss	0x48(%r12), %xmm0
140ac02f2: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140ac02f9: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
140ac0300: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140ac0307: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
140ac030e: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
140ac0315: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140ac031c: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
140ac0323: 41 8b 54 24 50              	movl	0x50(%r12), %edx
140ac0328: 44 8d 42 fe                 	leal	-0x2(%rdx), %r8d
140ac032c: 41 83 f8 02                 	cmpl	$0x2, %r8d
140ac0330: b9 02 00 00 00              	movl	$0x2, %ecx
140ac0335: 44 0f 43 c1                 	cmovael	%ecx, %r8d
140ac0339: 45 85 c0                    	testl	%r8d, %r8d
140ac033c: 0f 84 be 0c 00 00           	je	0x140ac1000 <.text+0xac0000>
140ac0342: 41 83 f8 02                 	cmpl	$0x2, %r8d
140ac0346: 0f 85 af 0c 00 00           	jne	0x140ac0ffb <.text+0xabfffb>
140ac034c: 49 8b 4c 24 64              	movq	0x64(%r12), %rcx
140ac0351: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
140ac0355: 41 0f 10 4c 24 54           	movups	0x54(%r12), %xmm1
140ac035b: 0f 29 4d 10                 	movaps	%xmm1, 0x10(%rbp)
140ac035f: 89 d1                       	movl	%edx, %ecx
140ac0361: e9 9a 0c 00 00              	jmp	0x140ac1000 <.text+0xac0000>
140ac0366: 4d 89 c6                    	movq	%r8, %r14
140ac0369: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140ac036d: 0f 84 0d 07 00 00           	je	0x140ac0a80 <.text+0xabfa80>
140ac0373: 49 8d 54 24 10              	leaq	0x10(%r12), %rdx
140ac0378: 48 8d 35 79 ea 01 02        	leaq	0x201ea79(%rip), %rsi   # 0x142adedf8
140ac037f: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
140ac0386: 49 89 f0                    	movq	%rsi, %r8
140ac0389: e8 e2 75 83 00              	callq	0x1412f7970 <.text+0x12f6970>
140ac038e: 49 8d 54 24 28              	leaq	0x28(%r12), %rdx
140ac0393: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac039a: 49 89 f0                    	movq	%rsi, %r8
140ac039d: e8 1e 75 83 00              	callq	0x1412f78c0 <.text+0x12f68c0>
140ac03a2: f3 41 0f 10 44 24 40        	movss	0x40(%r12), %xmm0
140ac03a9: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140ac03b0: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
140ac03b7: 0f 10 8d a0 00 00 00        	movups	0xa0(%rbp), %xmm1
140ac03be: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140ac03c5: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
140ac03cc: 0f 11 8d 58 01 00 00        	movups	%xmm1, 0x158(%rbp)
140ac03d3: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140ac03da: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140ac03e1: f3 41 0f 10 4c 24 64        	movss	0x64(%r12), %xmm1
140ac03e8: 41 8b 4c 24 48              	movl	0x48(%r12), %ecx
140ac03ed: 8d 51 fe                    	leal	-0x2(%rcx), %edx
140ac03f0: 83 fa 02                    	cmpl	$0x2, %edx
140ac03f3: b8 02 00 00 00              	movl	$0x2, %eax
140ac03f8: 0f 43 d0                    	cmovael	%eax, %edx
140ac03fb: 85 d2                       	testl	%edx, %edx
140ac03fd: 0f 84 40 09 00 00           	je	0x140ac0d43 <.text+0xabfd43>
140ac0403: 83 fa 02                    	cmpl	$0x2, %edx
140ac0406: 0f 85 32 09 00 00           	jne	0x140ac0d3e <.text+0xabfd3e>
140ac040c: 49 8b 44 24 5c              	movq	0x5c(%r12), %rax
140ac0411: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140ac0418: 41 0f 10 54 24 4c           	movups	0x4c(%r12), %xmm2
140ac041e: 0f 29 95 a0 00 00 00        	movaps	%xmm2, 0xa0(%rbp)
140ac0425: 89 c8                       	movl	%ecx, %eax
140ac0427: e9 17 09 00 00              	jmp	0x140ac0d43 <.text+0xabfd43>
140ac042c: 49 8b 44 24 18              	movq	0x18(%r12), %rax
140ac0431: 0f b6 4e 14                 	movzbl	0x14(%rsi), %ecx
140ac0435: 83 f1 01                    	xorl	$0x1, %ecx
140ac0438: f3 41 0f 10 44 8c 20        	movss	0x20(%r12,%rcx,4), %xmm0
140ac043f: c7 85 40 01 00 00 00 00 00 00       	movl	$0x0, 0x140(%rbp)
140ac0449: c6 85 60 01 00 00 01        	movb	$0x1, 0x160(%rbp)
140ac0450: 48 89 85 58 01 00 00        	movq	%rax, 0x158(%rbp)
140ac0457: c7 85 50 01 00 00 01 00 00 00       	movl	$0x1, 0x150(%rbp)
140ac0461: f3 0f 11 85 54 01 00 00     	movss	%xmm0, 0x154(%rbp)
140ac0469: 4c 8d 05 98 eb 01 02        	leaq	0x201eb98(%rip), %r8    # 0x142adf008
140ac0470: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac0477: 4c 89 c9                    	movq	%r9, %rcx
140ac047a: e8 91 6d 86 00              	callq	0x141327210 <.text+0x1326210>
140ac047f: e9 59 05 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0484: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac0488: 48 83 f0 21                 	xorq	$0x21, %rax
140ac048c: 41 0f b6 04 04              	movzbl	(%r12,%rax), %eax
140ac0491: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac0496: 88 85 40 01 00 00           	movb	%al, 0x140(%rbp)
140ac049c: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac04a3: 4c 89 c1                    	movq	%r8, %rcx
140ac04a6: 49 89 c0                    	movq	%rax, %r8
140ac04a9: e8 02 9a fe ff              	callq	0x140aa9eb0 <.text+0xaa8eb0>
140ac04ae: e9 2a 05 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac04b3: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140ac04b7: 0f 84 ba 00 00 00           	je	0x140ac0577 <.text+0xabf577>
140ac04bd: 4c 89 c3                    	movq	%r8, %rbx
140ac04c0: 31 c0                       	xorl	%eax, %eax
140ac04c2: 49 3b 44 24 18              	cmpq	0x18(%r12), %rax
140ac04c7: 0f 80 55 0e 00 00           	jo	0x140ac1322 <.text+0xac0322>
140ac04cd: 49 8d 54 24 18              	leaq	0x18(%r12), %rdx
140ac04d2: 48 8d 35 1f e9 01 02        	leaq	0x201e91f(%rip), %rsi   # 0x142adedf8
140ac04d9: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140ac04e0: 49 89 f0                    	movq	%rsi, %r8
140ac04e3: e8 88 74 83 00              	callq	0x1412f7970 <.text+0x12f6970>
140ac04e8: 49 8d 54 24 30              	leaq	0x30(%r12), %rdx
140ac04ed: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac04f4: 49 89 f0                    	movq	%rsi, %r8
140ac04f7: e8 c4 73 83 00              	callq	0x1412f78c0 <.text+0x12f68c0>
140ac04fc: f3 41 0f 10 44 24 48        	movss	0x48(%r12), %xmm0
140ac0503: 48 8b 85 88 00 00 00        	movq	0x88(%rbp), %rax
140ac050a: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
140ac0511: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140ac0518: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
140ac051f: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
140ac0526: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140ac052d: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
140ac0534: 41 8b 54 24 50              	movl	0x50(%r12), %edx
140ac0539: 44 8d 42 fe                 	leal	-0x2(%rdx), %r8d
140ac053d: 41 83 f8 02                 	cmpl	$0x2, %r8d
140ac0541: b9 02 00 00 00              	movl	$0x2, %ecx
140ac0546: 44 0f 43 c1                 	cmovael	%ecx, %r8d
140ac054a: 45 85 c0                    	testl	%r8d, %r8d
140ac054d: 0f 84 6b 0d 00 00           	je	0x140ac12be <.text+0xac02be>
140ac0553: 41 83 f8 02                 	cmpl	$0x2, %r8d
140ac0557: 0f 85 5c 0d 00 00           	jne	0x140ac12b9 <.text+0xac02b9>
140ac055d: 49 8b 4c 24 64              	movq	0x64(%r12), %rcx
140ac0562: 48 89 4d 00                 	movq	%rcx, (%rbp)
140ac0566: 41 0f 10 4c 24 54           	movups	0x54(%r12), %xmm1
140ac056c: 0f 29 4d f0                 	movaps	%xmm1, -0x10(%rbp)
140ac0570: 89 d1                       	movl	%edx, %ecx
140ac0572: e9 47 0d 00 00              	jmp	0x140ac12be <.text+0xac02be>
140ac0577: 49 8b 84 24 88 00 00 00     	movq	0x88(%r12), %rax
140ac057f: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
140ac0586: 4c 89 c2                    	movq	%r8, %rdx
140ac0589: 49 89 c0                    	movq	%rax, %r8
140ac058c: e8 cf a4 d9 ff              	callq	0x14085aa60 <.text+0x859a60>
140ac0591: 31 c0                       	xorl	%eax, %eax
140ac0593: 48 3b 85 40 01 00 00        	cmpq	0x140(%rbp), %rax
140ac059a: 0f 80 3d 04 00 00           	jo	0x140ac09dd <.text+0xabf9dd>
140ac05a0: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
140ac05a7: e8 44 1f b1 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140ac05ac: e9 2c 04 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac05b1: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac05b5: 83 f0 01                    	xorl	$0x1, %eax
140ac05b8: f3 41 0f 10 44 84 20        	movss	0x20(%r12,%rax,4), %xmm0
140ac05bf: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac05c4: f3 0f 11 85 40 01 00 00     	movss	%xmm0, 0x140(%rbp)
140ac05cc: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac05d3: 4c 89 c1                    	movq	%r8, %rcx
140ac05d6: 49 89 c0                    	movq	%rax, %r8
140ac05d9: e8 32 a1 fe ff              	callq	0x140aaa710 <.text+0xaa9710>
140ac05de: e9 fa 03 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac05e3: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac05e8: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac05ec: 48 83 f0 21                 	xorq	$0x21, %rax
140ac05f0: 41 0f b6 04 04              	movzbl	(%r12,%rax), %eax
140ac05f5: 88 85 40 01 00 00           	movb	%al, 0x140(%rbp)
140ac05fb: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac0602: 4c 89 c1                    	movq	%r8, %rcx
140ac0605: 49 89 c0                    	movq	%rax, %r8
140ac0608: e8 43 9d fe ff              	callq	0x140aaa350 <.text+0xaa9350>
140ac060d: e9 cb 03 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0612: 80 7e 14 00                 	cmpb	$0x0, 0x14(%rsi)
140ac0616: 0f 84 16 05 00 00           	je	0x140ac0b32 <.text+0xabfb32>
140ac061c: 4c 89 c6                    	movq	%r8, %rsi
140ac061f: 49 8d 54 24 20              	leaq	0x20(%r12), %rdx
140ac0624: 41 0f 10 44 24 20           	movups	0x20(%r12), %xmm0
140ac062a: 41 0f 10 4c 24 2c           	movups	0x2c(%r12), %xmm1
140ac0630: e9 11 05 00 00              	jmp	0x140ac0b46 <.text+0xabfb46>
140ac0635: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac063a: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac063e: 83 f0 01                    	xorl	$0x1, %eax
140ac0641: f3 41 0f 10 44 84 20        	movss	0x20(%r12,%rax,4), %xmm0
140ac0648: f3 0f 11 85 40 01 00 00     	movss	%xmm0, 0x140(%rbp)
140ac0650: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac0657: 4c 89 c1                    	movq	%r8, %rcx
140ac065a: 49 89 c0                    	movq	%rax, %r8
140ac065d: e8 ce 9e fe ff              	callq	0x140aaa530 <.text+0xaa9530>
140ac0662: e9 76 03 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0667: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac066b: 83 f0 01                    	xorl	$0x1, %eax
140ac066e: f3 41 0f 10 44 84 20        	movss	0x20(%r12,%rax,4), %xmm0
140ac0675: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac067a: f3 0f 11 85 40 01 00 00     	movss	%xmm0, 0x140(%rbp)
140ac0682: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac0689: 4c 89 c1                    	movq	%r8, %rcx
140ac068c: 49 89 c0                    	movq	%rax, %r8
140ac068f: e8 2c a4 fe ff              	callq	0x140aaaac0 <.text+0xaa9ac0>
140ac0694: e9 44 03 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0699: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac069e: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac06a2: 48 83 f0 21                 	xorq	$0x21, %rax
140ac06a6: 41 0f b6 04 04              	movzbl	(%r12,%rax), %eax
140ac06ab: 88 85 40 01 00 00           	movb	%al, 0x140(%rbp)
140ac06b1: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac06b8: 4c 89 c1                    	movq	%r8, %rcx
140ac06bb: 49 89 c0                    	movq	%rax, %r8
140ac06be: e8 8d 9c fe ff              	callq	0x140aaa350 <.text+0xaa9350>
140ac06c3: e9 15 03 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac06c8: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac06cd: 0f b6 46 14                 	movzbl	0x14(%rsi), %eax
140ac06d1: 48 83 f0 21                 	xorq	$0x21, %rax
140ac06d5: 41 0f b6 04 04              	movzbl	(%r12,%rax), %eax
140ac06da: 88 85 40 01 00 00           	movb	%al, 0x140(%rbp)
140ac06e0: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac06e7: 4c 89 c1                    	movq	%r8, %rcx
140ac06ea: 49 89 c0                    	movq	%rax, %r8
140ac06ed: e8 ee 92 fe ff              	callq	0x140aa99e0 <.text+0xaa89e0>
140ac06f2: e9 e6 02 00 00              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac06f7: 4c 8d 35 fa e6 01 02        	leaq	0x201e6fa(%rip), %r14   # 0x142adedf8
140ac06fe: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
140ac0705: 4c 89 fa                    	movq	%r15, %rdx
140ac0708: 4d 89 f0                    	movq	%r14, %r8
140ac070b: e8 60 72 83 00              	callq	0x1412f7970 <.text+0x12f6970>
140ac0710: 49 8d 95 98 00 00 00        	leaq	0x98(%r13), %rdx
140ac0717: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac071e: 4d 89 f0                    	movq	%r14, %r8
140ac0721: e8 9a 71 83 00              	callq	0x1412f78c0 <.text+0x12f68c0>
140ac0726: f3 41 0f 10 85 b0 00 00 00  	movss	0xb0(%r13), %xmm0
140ac072f: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140ac0736: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
140ac073d: 0f 10 8d a0 00 00 00        	movups	0xa0(%rbp), %xmm1
140ac0744: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140ac074b: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
140ac0752: 0f 11 8d 58 01 00 00        	movups	%xmm1, 0x158(%rbp)
140ac0759: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140ac0760: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140ac0767: 41 8b 95 b8 00 00 00        	movl	0xb8(%r13), %edx
140ac076e: 44 8d 42 fe                 	leal	-0x2(%rdx), %r8d
140ac0772: 41 83 f8 02                 	cmpl	$0x2, %r8d
140ac0776: b8 02 00 00 00              	movl	$0x2, %eax
140ac077b: 44 0f 43 c0                 	cmovael	%eax, %r8d
140ac077f: 45 85 c0                    	testl	%r8d, %r8d
140ac0782: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140ac0789: 0f 84 a7 04 00 00           	je	0x140ac0c36 <.text+0xabfc36>
140ac078f: 41 83 f8 02                 	cmpl	$0x2, %r8d
140ac0793: 0f 85 98 04 00 00           	jne	0x140ac0c31 <.text+0xabfc31>
140ac0799: 49 8b 85 cc 00 00 00        	movq	0xcc(%r13), %rax
140ac07a0: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
140ac07a4: 41 0f 10 8d bc 00 00 00     	movups	0xbc(%r13), %xmm1
140ac07ac: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
140ac07b0: 89 d0                       	movl	%edx, %eax
140ac07b2: e9 7f 04 00 00              	jmp	0x140ac0c36 <.text+0xabfc36>
140ac07b7: f3 41 0f 10 85 fc 00 00 00  	movss	0xfc(%r13), %xmm0
140ac07c0: f3 41 0f 10 8d 00 01 00 00  	movss	0x100(%r13), %xmm1
140ac07c9: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
140ac07cc: 0f 86 8c 0d 00 00           	jbe	0x140ac155e <.text+0xac055e>
140ac07d2: 4d 8d a5 00 01 00 00        	leaq	0x100(%r13), %r12
140ac07d9: 44 0f 28 cf                 	movaps	%xmm7, %xmm9
140ac07dd: f3 44 0f 5c ce              	subss	%xmm6, %xmm9
140ac07e2: f3 41 0f 59 c1              	mulss	%xmm9, %xmm0
140ac07e7: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
140ac07eb: f3 44 0f 59 c6              	mulss	%xmm6, %xmm8
140ac07f0: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
140ac07f5: 48 8d 15 bc e8 01 02        	leaq	0x201e8bc(%rip), %rdx   # 0x142adf0b8
140ac07fc: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
140ac0800: e8 2b 0e e2 ff              	callq	0x1408e1630 <.text+0x8e0630>
140ac0805: 48 8d 15 c4 e8 01 02        	leaq	0x201e8c4(%rip), %rdx   # 0x142adf0d0
140ac080c: 44 0f 28 f6                 	movaps	%xmm6, %xmm14
140ac0810: f3 45 0f 59 0c 24           	mulss	(%r12), %xmm9
140ac0816: f3 45 0f 58 ce              	addss	%xmm14, %xmm9
140ac081b: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
140ac081f: e8 0c 0e e2 ff              	callq	0x1408e1630 <.text+0x8e0630>
140ac0824: 48 8d 05 05 e9 01 02        	leaq	0x201e905(%rip), %rax   # 0x142adf130
140ac082b: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140ac0830: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac0837: 4c 89 fa                    	movq	%r15, %rdx
140ac083a: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
140ac083e: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
140ac0842: e8 69 63 9a 00              	callq	0x141466bb0 <.text+0x1465bb0>
140ac0847: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140ac084e: 48 85 c0                    	testq	%rax, %rax
140ac0851: 0f 94 c1                    	sete	%cl
140ac0854: 48 8d 14 c5 00 00 00 00     	leaq	(,%rax,8), %rdx
140ac085c: 48 03 95 d8 00 00 00        	addq	0xd8(%rbp), %rdx
140ac0863: 48 83 c2 f8                 	addq	$-0x8, %rdx
140ac0867: 41 0f 94 c0                 	sete	%r8b
140ac086b: 41 08 c8                    	orb	%cl, %r8b
140ac086e: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140ac0875: 0f 85 a0 00 00 00           	jne	0x140ac091b <.text+0xabf91b>
140ac087b: 45 0f 2e d5                 	ucomiss	%xmm13, %xmm10
140ac087f: f2 0f 10 02                 	movsd	(%rdx), %xmm0
140ac0883: f2 0f 11 85 88 00 00 00     	movsd	%xmm0, 0x88(%rbp)
140ac088b: 0f 85 8a 00 00 00           	jne	0x140ac091b <.text+0xabf91b>
140ac0891: 0f 8a 84 00 00 00           	jp	0x140ac091b <.text+0xabf91b>
140ac0897: 48 83 f8 02                 	cmpq	$0x2, %rax
140ac089b: 72 7e                       	jb	0x140ac091b <.text+0xabf91b>
140ac089d: f3 0f 10 85 00 01 00 00     	movss	0x100(%rbp), %xmm0
140ac08a5: 0f 57 c9                    	xorps	%xmm1, %xmm1
140ac08a8: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140ac08ab: 76 6e                       	jbe	0x140ac091b <.text+0xabf91b>
140ac08ad: 49 89 fe                    	movq	%rdi, %r14
140ac08b0: 49 8b bd f0 00 00 00        	movq	0xf0(%r13), %rdi
140ac08b7: c6 85 c6 01 00 00 01        	movb	$0x1, 0x1c6(%rbp)
140ac08be: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
140ac08c5: 48 8d 95 88 00 00 00        	leaq	0x88(%rbp), %rdx
140ac08cc: e8 ef c4 1d 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
140ac08d1: 8b 85 a8 00 00 00           	movl	0xa8(%rbp), %eax
140ac08d7: 89 85 50 01 00 00           	movl	%eax, 0x150(%rbp)
140ac08dd: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
140ac08e4: 48 89 85 48 01 00 00        	movq	%rax, 0x148(%rbp)
140ac08eb: 48 89 bd 40 01 00 00        	movq	%rdi, 0x140(%rbp)
140ac08f2: 66 c7 85 54 01 00 00 02 01  	movw	$0x102, 0x154(%rbp)     # imm = 0x102
140ac08fb: c6 85 c6 01 00 00 01        	movb	$0x1, 0x1c6(%rbp)
140ac0902: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac0909: 4c 89 f1                    	movq	%r14, %rcx
140ac090c: 48 89 da                    	movq	%rbx, %rdx
140ac090f: e8 8c f1 ff ff              	callq	0x140abfaa0 <.text+0xabeaa0>
140ac0914: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140ac091b: 49 8b 95 f0 00 00 00        	movq	0xf0(%r13), %rdx
140ac0922: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140ac0929: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140ac0930: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140ac0937: 0f 10 8d e0 00 00 00        	movups	0xe0(%rbp), %xmm1
140ac093e: 0f 10 95 f0 00 00 00        	movups	0xf0(%rbp), %xmm2
140ac0945: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
140ac094c: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
140ac0953: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140ac095a: c6 85 c6 01 00 00 00        	movb	$0x0, 0x1c6(%rbp)
140ac0961: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac0968: e8 23 97 fe ff              	callq	0x140aaa090 <.text+0xaa9090>
140ac096d: 49 8b 9d f0 00 00 00        	movq	0xf0(%r13), %rbx
140ac0974: f3 45 0f 10 45 68           	movss	0x68(%r13), %xmm8
140ac097a: f3 45 0f 10 8d d8 00 00 00  	movss	0xd8(%r13), %xmm9
140ac0983: c6 85 c6 01 00 00 00        	movb	$0x0, 0x1c6(%rbp)
140ac098a: 48 8d 0d b7 e7 01 02        	leaq	0x201e7b7(%rip), %rcx   # 0x142adf148
140ac0991: 0f 28 ce                    	movaps	%xmm6, %xmm1
140ac0994: e8 57 55 1d 00              	callq	0x140c95ef0 <.text+0xc94ef0>
140ac0999: f3 0f 5c f8                 	subss	%xmm0, %xmm7
140ac099d: f3 44 0f 59 c7              	mulss	%xmm7, %xmm8
140ac09a2: f3 44 0f 59 c8              	mulss	%xmm0, %xmm9
140ac09a7: f3 45 0f 58 c8              	addss	%xmm8, %xmm9
140ac09ac: f3 44 0f 11 8d 40 01 00 00  	movss	%xmm9, 0x140(%rbp)
140ac09b5: c6 85 c6 01 00 00 00        	movb	$0x0, 0x1c6(%rbp)
140ac09bc: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac09c3: 48 8b 8d b8 01 00 00        	movq	0x1b8(%rbp), %rcx
140ac09ca: 48 89 da                    	movq	%rbx, %rdx
140ac09cd: e8 ee a0 fe ff              	callq	0x140aaaac0 <.text+0xaa9ac0>
140ac09d2: f3 45 0f 58 dc              	addss	%xmm12, %xmm11
140ac09d7: f3 44 0f 11 5e 10           	movss	%xmm11, 0x10(%rsi)
140ac09dd: 0f 28 b5 d0 01 00 00        	movaps	0x1d0(%rbp), %xmm6
140ac09e4: 0f 28 bd e0 01 00 00        	movaps	0x1e0(%rbp), %xmm7
140ac09eb: 44 0f 28 85 f0 01 00 00     	movaps	0x1f0(%rbp), %xmm8
140ac09f3: 44 0f 28 8d 00 02 00 00     	movaps	0x200(%rbp), %xmm9
140ac09fb: 44 0f 28 95 10 02 00 00     	movaps	0x210(%rbp), %xmm10
140ac0a03: 44 0f 28 9d 20 02 00 00     	movaps	0x220(%rbp), %xmm11
140ac0a0b: 44 0f 28 a5 30 02 00 00     	movaps	0x230(%rbp), %xmm12
140ac0a13: 44 0f 28 ad 40 02 00 00     	movaps	0x240(%rbp), %xmm13
140ac0a1b: 44 0f 28 b5 50 02 00 00     	movaps	0x250(%rbp), %xmm14
140ac0a23: 48 81 c4 e8 02 00 00        	addq	$0x2e8, %rsp            # imm = 0x2E8
140ac0a2a: 5b                          	popq	%rbx
140ac0a2b: 5f                          	popq	%rdi
140ac0a2c: 5e                          	popq	%rsi
140ac0a2d: 41 5c                       	popq	%r12
140ac0a2f: 41 5d                       	popq	%r13
140ac0a31: 41 5e                       	popq	%r14
140ac0a33: 41 5f                       	popq	%r15
140ac0a35: 5d                          	popq	%rbp
140ac0a36: c3                          	retq
140ac0a37: 41 8b 44 24 3c              	movl	0x3c(%r12), %eax
140ac0a3c: 44 8d 48 fe                 	leal	-0x2(%rax), %r9d
140ac0a40: 41 83 f9 02                 	cmpl	$0x2, %r9d
140ac0a44: b9 02 00 00 00              	movl	$0x2, %ecx
140ac0a49: 44 0f 43 c9                 	cmovael	%ecx, %r9d
140ac0a4d: 45 85 c9                    	testl	%r9d, %r9d
140ac0a50: 0f 84 79 01 00 00           	je	0x140ac0bcf <.text+0xabfbcf>
140ac0a56: 41 83 f9 02                 	cmpl	$0x2, %r9d
140ac0a5a: 0f 85 6a 01 00 00           	jne	0x140ac0bca <.text+0xabfbca>
140ac0a60: 49 8b 4c 24 50              	movq	0x50(%r12), %rcx
140ac0a65: 48 89 8d e0 00 00 00        	movq	%rcx, 0xe0(%rbp)
140ac0a6c: 41 0f 10 44 24 40           	movups	0x40(%r12), %xmm0
140ac0a72: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
140ac0a79: 89 c1                       	movl	%eax, %ecx
140ac0a7b: e9 4f 01 00 00              	jmp	0x140ac0bcf <.text+0xabfbcf>
140ac0a80: 49 8d 94 24 80 00 00 00     	leaq	0x80(%r12), %rdx
140ac0a88: 48 8d 35 69 e3 01 02        	leaq	0x201e369(%rip), %rsi   # 0x142adedf8
140ac0a8f: 48 8d 8d a0 00 00 00        	leaq	0xa0(%rbp), %rcx
140ac0a96: 49 89 f0                    	movq	%rsi, %r8
140ac0a99: e8 d2 6e 83 00              	callq	0x1412f7970 <.text+0x12f6970>
140ac0a9e: 49 8d 94 24 98 00 00 00     	leaq	0x98(%r12), %rdx
140ac0aa6: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac0aad: 49 89 f0                    	movq	%rsi, %r8
140ac0ab0: e8 0b 6e 83 00              	callq	0x1412f78c0 <.text+0x12f68c0>
140ac0ab5: f3 41 0f 10 84 24 b0 00 00 00       	movss	0xb0(%r12), %xmm0
140ac0abf: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140ac0ac6: 48 89 85 50 01 00 00        	movq	%rax, 0x150(%rbp)
140ac0acd: 0f 10 8d a0 00 00 00        	movups	0xa0(%rbp), %xmm1
140ac0ad4: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140ac0adb: 0f 10 8d d0 00 00 00        	movups	0xd0(%rbp), %xmm1
140ac0ae2: 0f 11 8d 58 01 00 00        	movups	%xmm1, 0x158(%rbp)
140ac0ae9: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140ac0af0: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140ac0af7: f3 41 0f 10 8c 24 d4 00 00 00       	movss	0xd4(%r12), %xmm1
140ac0b01: 41 8b 8c 24 b8 00 00 00     	movl	0xb8(%r12), %ecx
140ac0b09: 8d 51 fe                    	leal	-0x2(%rcx), %edx
140ac0b0c: 83 fa 02                    	cmpl	$0x2, %edx
140ac0b0f: b8 02 00 00 00              	movl	$0x2, %eax
140ac0b14: 0f 43 d0                    	cmovael	%eax, %edx
140ac0b17: 85 d2                       	testl	%edx, %edx
140ac0b19: 0f 84 e4 02 00 00           	je	0x140ac0e03 <.text+0xabfe03>
140ac0b1f: 83 fa 01                    	cmpl	$0x1, %edx
140ac0b22: 0f 85 ba 02 00 00           	jne	0x140ac0de2 <.text+0xabfde2>
140ac0b28: b8 03 00 00 00              	movl	$0x3, %eax
140ac0b2d: e9 d1 02 00 00              	jmp	0x140ac0e03 <.text+0xabfe03>
140ac0b32: 4c 89 c6                    	movq	%r8, %rsi
140ac0b35: 49 8d 54 24 3c              	leaq	0x3c(%r12), %rdx
140ac0b3a: 41 0f 10 44 24 3c           	movups	0x3c(%r12), %xmm0
140ac0b40: 41 0f 10 4c 24 48           	movups	0x48(%r12), %xmm1
140ac0b46: 0f 11 4d 3c                 	movups	%xmm1, 0x3c(%rbp)
140ac0b4a: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
140ac0b4e: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac0b55: e8 56 e7 73 00              	callq	0x1411ff2b0 <.text+0x11fe2b0>
140ac0b5a: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac0b5f: c6 85 c3 01 00 00 01        	movb	$0x1, 0x1c3(%rbp)
140ac0b66: 4c 8d 45 30                 	leaq	0x30(%rbp), %r8
140ac0b6a: 48 89 f1                    	movq	%rsi, %rcx
140ac0b6d: e8 9e 8a fe ff              	callq	0x140aa9610 <.text+0xaa8610>
140ac0b72: 49 8b 54 24 18              	movq	0x18(%r12), %rdx
140ac0b77: 48 8b 85 00 01 00 00        	movq	0x100(%rbp), %rax
140ac0b7e: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140ac0b85: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140ac0b8c: 0f 10 8d e0 00 00 00        	movups	0xe0(%rbp), %xmm1
140ac0b93: 0f 10 95 f0 00 00 00        	movups	0xf0(%rbp), %xmm2
140ac0b9a: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
140ac0ba1: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
140ac0ba8: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140ac0baf: c6 85 c3 01 00 00 00        	movb	$0x0, 0x1c3(%rbp)
140ac0bb6: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac0bbd: 48 89 f1                    	movq	%rsi, %rcx
140ac0bc0: e8 cb 94 fe ff              	callq	0x140aaa090 <.text+0xaa9090>
140ac0bc5: e9 13 fe ff ff              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0bca: b9 03 00 00 00              	movl	$0x3, %ecx
140ac0bcf: 89 8d 40 01 00 00           	movl	%ecx, 0x140(%rbp)
140ac0bd5: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
140ac0bdc: 0f 11 85 44 01 00 00        	movups	%xmm0, 0x144(%rbp)
140ac0be3: 48 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %rax
140ac0bea: 48 89 85 54 01 00 00        	movq	%rax, 0x154(%rbp)
140ac0bf1: 48 8d 85 40 01 00 00        	leaq	0x140(%rbp), %rax
140ac0bf8: 4c 89 c1                    	movq	%r8, %rcx
140ac0bfb: 49 89 c0                    	movq	%rax, %r8
140ac0bfe: e8 0d 8a fe ff              	callq	0x140aa9610 <.text+0xaa8610>
140ac0c03: e9 d5 fd ff ff              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0c08: b8 03 00 00 00              	movl	$0x3, %eax
140ac0c0d: f3 41 0f 10 4d 64           	movss	0x64(%r13), %xmm1
140ac0c13: f2 41 0f 10 55 68           	movsd	0x68(%r13), %xmm2
140ac0c19: 41 0f b6 55 78              	movzbl	0x78(%r13), %edx
140ac0c1e: 45 0f b6 45 79              	movzbl	0x79(%r13), %r8d
140ac0c23: f3 41 0f 10 5d 70           	movss	0x70(%r13), %xmm3
140ac0c29: f3 41 0f 10 65 74           	movss	0x74(%r13), %xmm4
140ac0c2f: eb 39                       	jmp	0x140ac0c6a <.text+0xabfc6a>
140ac0c31: b8 03 00 00 00              	movl	$0x3, %eax
140ac0c36: f3 41 0f 10 8d d4 00 00 00  	movss	0xd4(%r13), %xmm1
140ac0c3f: f2 41 0f 10 95 d8 00 00 00  	movsd	0xd8(%r13), %xmm2
140ac0c48: 41 0f b6 95 e8 00 00 00     	movzbl	0xe8(%r13), %edx
140ac0c50: 45 0f b6 85 e9 00 00 00     	movzbl	0xe9(%r13), %r8d
140ac0c58: f3 41 0f 10 9d e0 00 00 00  	movss	0xe0(%r13), %xmm3
140ac0c61: f3 41 0f 10 a5 e4 00 00 00  	movss	0xe4(%r13), %xmm4
140ac0c6a: 0f 28 ad 40 01 00 00        	movaps	0x140(%rbp), %xmm5
140ac0c71: 0f 28 b5 50 01 00 00        	movaps	0x150(%rbp), %xmm6
140ac0c78: 4c 8b 8d 60 01 00 00        	movq	0x160(%rbp), %r9
140ac0c7f: 4c 89 8d f0 00 00 00        	movq	%r9, 0xf0(%rbp)
140ac0c86: 4c 8b 8d 68 01 00 00        	movq	0x168(%rbp), %r9
140ac0c8d: 4c 89 8d f8 00 00 00        	movq	%r9, 0xf8(%rbp)
140ac0c94: 0f 29 b5 e0 00 00 00        	movaps	%xmm6, 0xe0(%rbp)
140ac0c9b: 0f 29 ad d0 00 00 00        	movaps	%xmm5, 0xd0(%rbp)
140ac0ca2: 0f 28 ad d0 00 00 00        	movaps	0xd0(%rbp), %xmm5
140ac0ca9: 0f 28 b5 e0 00 00 00        	movaps	0xe0(%rbp), %xmm6
140ac0cb0: 4c 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %r9
140ac0cb7: 4c 89 8d 60 01 00 00        	movq	%r9, 0x160(%rbp)
140ac0cbe: 4c 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %r9
140ac0cc5: 4c 89 8d 68 01 00 00        	movq	%r9, 0x168(%rbp)
140ac0ccc: 0f 29 b5 50 01 00 00        	movaps	%xmm6, 0x150(%rbp)
140ac0cd3: 0f 29 ad 40 01 00 00        	movaps	%xmm5, 0x140(%rbp)
140ac0cda: f3 0f 11 85 70 01 00 00     	movss	%xmm0, 0x170(%rbp)
140ac0ce2: 89 85 78 01 00 00           	movl	%eax, 0x178(%rbp)
140ac0ce8: 0f 28 45 60                 	movaps	0x60(%rbp), %xmm0
140ac0cec: 0f 11 85 7c 01 00 00        	movups	%xmm0, 0x17c(%rbp)
140ac0cf3: 48 8b 45 70                 	movq	0x70(%rbp), %rax
140ac0cf7: 48 89 85 8c 01 00 00        	movq	%rax, 0x18c(%rbp)
140ac0cfe: f3 0f 11 8d 94 01 00 00     	movss	%xmm1, 0x194(%rbp)
140ac0d06: 0f 13 95 98 01 00 00        	movlps	%xmm2, 0x198(%rbp)
140ac0d0d: f3 0f 11 9d a0 01 00 00     	movss	%xmm3, 0x1a0(%rbp)
140ac0d15: f3 0f 11 a5 a4 01 00 00     	movss	%xmm4, 0x1a4(%rbp)
140ac0d1d: 88 95 a8 01 00 00           	movb	%dl, 0x1a8(%rbp)
140ac0d23: 44 88 85 a9 01 00 00        	movb	%r8b, 0x1a9(%rbp)
140ac0d2a: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac0d31: 48 89 da                    	movq	%rbx, %rdx
140ac0d34: e8 87 8e fe ff              	callq	0x140aa9bc0 <.text+0xaa8bc0>
140ac0d39: e9 dc f1 ff ff              	jmp	0x140abff1a <.text+0xabef1a>
140ac0d3e: b8 03 00 00 00              	movl	$0x3, %eax
140ac0d43: 41 0f b7 4c 24 78           	movzwl	0x78(%r12), %ecx
140ac0d49: f3 41 0f 10 54 24 70        	movss	0x70(%r12), %xmm2
140ac0d50: 0f 28 9d 40 01 00 00        	movaps	0x140(%rbp), %xmm3
140ac0d57: 0f 28 a5 50 01 00 00        	movaps	0x150(%rbp), %xmm4
140ac0d5e: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
140ac0d65: 4c 8b 85 68 01 00 00        	movq	0x168(%rbp), %r8
140ac0d6c: 0f 29 9d d0 00 00 00        	movaps	%xmm3, 0xd0(%rbp)
140ac0d73: 0f 29 a5 e0 00 00 00        	movaps	%xmm4, 0xe0(%rbp)
140ac0d7a: 48 89 95 f0 00 00 00        	movq	%rdx, 0xf0(%rbp)
140ac0d81: 4c 89 85 f8 00 00 00        	movq	%r8, 0xf8(%rbp)
140ac0d88: f3 0f 11 85 00 01 00 00     	movss	%xmm0, 0x100(%rbp)
140ac0d90: f3 0f 11 8d 24 01 00 00     	movss	%xmm1, 0x124(%rbp)
140ac0d98: 89 85 08 01 00 00           	movl	%eax, 0x108(%rbp)
140ac0d9e: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
140ac0da5: 0f 11 85 0c 01 00 00        	movups	%xmm0, 0x10c(%rbp)
140ac0dac: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140ac0db3: 48 89 85 1c 01 00 00        	movq	%rax, 0x11c(%rbp)
140ac0dba: f2 41 0f 10 44 24 68        	movsd	0x68(%r12), %xmm0
140ac0dc1: f2 0f 11 85 28 01 00 00     	movsd	%xmm0, 0x128(%rbp)
140ac0dc9: 66 89 8d 38 01 00 00        	movw	%cx, 0x138(%rbp)
140ac0dd0: f3 0f 11 95 30 01 00 00     	movss	%xmm2, 0x130(%rbp)
140ac0dd8: 41 8b 44 24 74              	movl	0x74(%r12), %eax
140ac0ddd: e9 c7 00 00 00              	jmp	0x140ac0ea9 <.text+0xabfea9>
140ac0de2: 49 8b 84 24 cc 00 00 00     	movq	0xcc(%r12), %rax
140ac0dea: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140ac0df1: 41 0f 10 94 24 bc 00 00 00  	movups	0xbc(%r12), %xmm2
140ac0dfa: 0f 29 95 a0 00 00 00        	movaps	%xmm2, 0xa0(%rbp)
140ac0e01: 89 c8                       	movl	%ecx, %eax
140ac0e03: 41 0f b7 8c 24 e8 00 00 00  	movzwl	0xe8(%r12), %ecx
140ac0e0c: f3 41 0f 10 94 24 e0 00 00 00       	movss	0xe0(%r12), %xmm2
140ac0e16: 0f 28 9d 40 01 00 00        	movaps	0x140(%rbp), %xmm3
140ac0e1d: 0f 28 a5 50 01 00 00        	movaps	0x150(%rbp), %xmm4
140ac0e24: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
140ac0e2b: 4c 8b 85 68 01 00 00        	movq	0x168(%rbp), %r8
140ac0e32: 0f 29 9d d0 00 00 00        	movaps	%xmm3, 0xd0(%rbp)
140ac0e39: 0f 29 a5 e0 00 00 00        	movaps	%xmm4, 0xe0(%rbp)
140ac0e40: 48 89 95 f0 00 00 00        	movq	%rdx, 0xf0(%rbp)
140ac0e47: 4c 89 85 f8 00 00 00        	movq	%r8, 0xf8(%rbp)
140ac0e4e: f3 0f 11 85 00 01 00 00     	movss	%xmm0, 0x100(%rbp)
140ac0e56: f3 0f 11 8d 24 01 00 00     	movss	%xmm1, 0x124(%rbp)
140ac0e5e: 89 85 08 01 00 00           	movl	%eax, 0x108(%rbp)
140ac0e64: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
140ac0e6b: 0f 11 85 0c 01 00 00        	movups	%xmm0, 0x10c(%rbp)
140ac0e72: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
140ac0e79: 48 89 85 1c 01 00 00        	movq	%rax, 0x11c(%rbp)
140ac0e80: f2 41 0f 10 84 24 d8 00 00 00       	movsd	0xd8(%r12), %xmm0
140ac0e8a: f2 0f 11 85 28 01 00 00     	movsd	%xmm0, 0x128(%rbp)
140ac0e92: 66 89 8d 38 01 00 00        	movw	%cx, 0x138(%rbp)
140ac0e99: f3 0f 11 95 30 01 00 00     	movss	%xmm2, 0x130(%rbp)
140ac0ea1: 41 8b 84 24 e4 00 00 00     	movl	0xe4(%r12), %eax
140ac0ea9: 89 85 34 01 00 00           	movl	%eax, 0x134(%rbp)
140ac0eaf: c6 85 c7 01 00 00 01        	movb	$0x1, 0x1c7(%rbp)
140ac0eb6: 4c 8d 05 d3 e0 01 02        	leaq	0x201e0d3(%rip), %r8    # 0x142adef90
140ac0ebd: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac0ec4: f3 0f 10 0d a4 49 e6 01     	movss	0x1e649a4(%rip), %xmm1  # 0x142925870
140ac0ecc: e8 9f 63 9a 00              	callq	0x141467270 <.text+0x1466270>
140ac0ed1: f3 0f 11 85 40 01 00 00     	movss	%xmm0, 0x140(%rbp)
140ac0ed9: f3 0f 11 8d 44 01 00 00     	movss	%xmm1, 0x144(%rbp)
140ac0ee1: c6 85 c7 01 00 00 01        	movb	$0x1, 0x1c7(%rbp)
140ac0ee8: 48 8d 4d 7c                 	leaq	0x7c(%rbp), %rcx
140ac0eec: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac0ef3: e8 c8 be 1d 00              	callq	0x140c9cdc0 <.text+0xc9bdc0>
140ac0ef8: 49 8b 94 24 f0 00 00 00     	movq	0xf0(%r12), %rdx
140ac0f00: 0f 28 85 30 01 00 00        	movaps	0x130(%rbp), %xmm0
140ac0f07: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
140ac0f0e: 0f 28 85 20 01 00 00        	movaps	0x120(%rbp), %xmm0
140ac0f15: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
140ac0f1c: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
140ac0f23: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
140ac0f2a: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
140ac0f31: 0f 28 8d e0 00 00 00        	movaps	0xe0(%rbp), %xmm1
140ac0f38: 0f 28 95 f0 00 00 00        	movaps	0xf0(%rbp), %xmm2
140ac0f3f: 0f 28 9d 00 01 00 00        	movaps	0x100(%rbp), %xmm3
140ac0f46: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
140ac0f4d: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
140ac0f54: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
140ac0f5b: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140ac0f62: c6 85 c7 01 00 00 00        	movb	$0x0, 0x1c7(%rbp)
140ac0f69: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac0f70: 4c 89 f1                    	movq	%r14, %rcx
140ac0f73: e8 48 8c fe ff              	callq	0x140aa9bc0 <.text+0xaa8bc0>
140ac0f78: 49 8b b4 24 f0 00 00 00     	movq	0xf0(%r12), %rsi
140ac0f80: f3 0f 10 75 7c              	movss	0x7c(%rbp), %xmm6
140ac0f85: f3 44 0f 10 85 80 00 00 00  	movss	0x80(%rbp), %xmm8
140ac0f8e: f3 0f 10 bd 84 00 00 00     	movss	0x84(%rbp), %xmm7
140ac0f96: 48 8b 0f                    	movq	(%rdi), %rcx
140ac0f99: c6 85 c7 01 00 00 00        	movb	$0x0, 0x1c7(%rbp)
140ac0fa0: 0f 28 ce                    	movaps	%xmm6, %xmm1
140ac0fa3: 0f 28 d7                    	movaps	%xmm7, %xmm2
140ac0fa6: e8 95 86 f2 ff              	callq	0x1409e9640 <.text+0x9e8640>
140ac0fab: f3 44 0f 58 c0              	addss	%xmm0, %xmm8
140ac0fb0: 48 89 b5 40 01 00 00        	movq	%rsi, 0x140(%rbp)
140ac0fb7: f3 0f 11 b5 48 01 00 00     	movss	%xmm6, 0x148(%rbp)
140ac0fbf: f3 44 0f 11 85 4c 01 00 00  	movss	%xmm8, 0x14c(%rbp)
140ac0fc8: f3 0f 11 bd 50 01 00 00     	movss	%xmm7, 0x150(%rbp)
140ac0fd0: 66 c7 85 54 01 00 00 02 01  	movw	$0x102, 0x154(%rbp)     # imm = 0x102
140ac0fd9: c6 85 c7 01 00 00 00        	movb	$0x0, 0x1c7(%rbp)
140ac0fe0: 4c 8d 05 91 df 01 02        	leaq	0x201df91(%rip), %r8    # 0x142adef78
140ac0fe7: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac0fee: 48 89 d9                    	movq	%rbx, %rcx
140ac0ff1: e8 5a 67 86 00              	callq	0x141327750 <.text+0x1326750>
140ac0ff6: e9 e2 f9 ff ff              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac0ffb: b9 03 00 00 00              	movl	$0x3, %ecx
140ac1000: f2 41 0f 10 4c 24 70        	movsd	0x70(%r12), %xmm1
140ac1007: 41 0f b6 94 24 80 00 00 00  	movzbl	0x80(%r12), %edx
140ac1010: 45 0f b6 84 24 81 00 00 00  	movzbl	0x81(%r12), %r8d
140ac1019: f3 41 0f 10 5c 24 78        	movss	0x78(%r12), %xmm3
140ac1020: 66 41 0f 6e 54 24 7c        	movd	0x7c(%r12), %xmm2
140ac1027: 4c 8b 8d 60 01 00 00        	movq	0x160(%rbp), %r9
140ac102e: 4c 89 8d c0 00 00 00        	movq	%r9, 0xc0(%rbp)
140ac1035: 0f 28 a5 40 01 00 00        	movaps	0x140(%rbp), %xmm4
140ac103c: 0f 28 ad 50 01 00 00        	movaps	0x150(%rbp), %xmm5
140ac1043: 0f 29 ad b0 00 00 00        	movaps	%xmm5, 0xb0(%rbp)
140ac104a: 0f 29 a5 a0 00 00 00        	movaps	%xmm4, 0xa0(%rbp)
140ac1051: 49 89 c1                    	movq	%rax, %r9
140ac1054: 49 f7 d9                    	negq	%r9
140ac1057: 0f 81 78 01 00 00           	jno	0x140ac11d5 <.text+0xac01d5>
140ac105d: f3 41 0f 10 84 24 90 00 00 00       	movss	0x90(%r12), %xmm0
140ac1067: 0f 57 c9                    	xorps	%xmm1, %xmm1
140ac106a: 0f 11 8d e0 00 00 00        	movups	%xmm1, 0xe0(%rbp)
140ac1071: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
140ac107c: 48 c7 85 d8 00 00 00 04 00 00 00    	movq	$0x4, 0xd8(%rbp)
140ac1087: 48 c7 85 f0 00 00 00 04 00 00 00    	movq	$0x4, 0xf0(%rbp)
140ac1092: 48 c7 85 f8 00 00 00 00 00 00 00    	movq	$0x0, 0xf8(%rbp)
140ac109d: c7 85 00 01 00 00 00 00 00 00       	movl	$0x0, 0x100(%rbp)
140ac10a7: c7 85 08 01 00 00 02 00 00 00       	movl	$0x2, 0x108(%rbp)
140ac10b1: f2 0f 10 0d 57 38 fd 01     	movsd	0x1fd3857(%rip), %xmm1  # 0x142a94910
140ac10b9: f2 0f 11 8d 28 01 00 00     	movsd	%xmm1, 0x128(%rbp)
140ac10c1: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
140ac10cc: 66 c7 85 38 01 00 00 00 00  	movw	$0x0, 0x138(%rbp)
140ac10d5: f3 0f 11 85 24 01 00 00     	movss	%xmm0, 0x124(%rbp)
140ac10dd: 49 8d 9c 24 88 00 00 00     	leaq	0x88(%r12), %rbx
140ac10e5: 49 8d 84 24 98 00 00 00     	leaq	0x98(%r12), %rax
140ac10ed: 48 8b 0b                    	movq	(%rbx), %rcx
140ac10f0: 8b 00                       	movl	(%rax), %eax
140ac10f2: 48 89 8d 40 01 00 00        	movq	%rcx, 0x140(%rbp)
140ac10f9: 89 85 48 01 00 00           	movl	%eax, 0x148(%rbp)
140ac10ff: c6 85 4c 01 00 00 00        	movb	$0x0, 0x14c(%rbp)
140ac1106: c6 85 c5 01 00 00 01        	movb	$0x1, 0x1c5(%rbp)
140ac110d: 4c 8d 05 94 de 01 02        	leaq	0x201de94(%rip), %r8    # 0x142adefa8
140ac1114: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac111b: 4c 89 f1                    	movq	%r14, %rcx
140ac111e: e8 4d 62 86 00              	callq	0x141327370 <.text+0x1326370>
140ac1123: 48 89 f1                    	movq	%rsi, %rcx
140ac1126: 48 8b 13                    	movq	(%rbx), %rdx
140ac1129: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140ac1130: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
140ac1137: 0f 10 85 20 01 00 00        	movups	0x120(%rbp), %xmm0
140ac113e: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
140ac1145: 0f 10 85 10 01 00 00        	movups	0x110(%rbp), %xmm0
140ac114c: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
140ac1153: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140ac115a: 0f 10 8d e0 00 00 00        	movups	0xe0(%rbp), %xmm1
140ac1161: f3 0f 6f 95 f0 00 00 00     	movdqu	0xf0(%rbp), %xmm2
140ac1169: 0f 10 9d 00 01 00 00        	movups	0x100(%rbp), %xmm3
140ac1170: 0f 29 9d 70 01 00 00        	movaps	%xmm3, 0x170(%rbp)
140ac1177: 66 0f 7f 95 60 01 00 00     	movdqa	%xmm2, 0x160(%rbp)
140ac117f: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
140ac1186: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140ac118d: c6 85 c5 01 00 00 00        	movb	$0x0, 0x1c5(%rbp)
140ac1194: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac119b: e8 00 93 d9 ff              	callq	0x14085a4a0 <.text+0x8594a0>
140ac11a0: 49 8b 94 24 88 00 00 00     	movq	0x88(%r12), %rdx
140ac11a8: f3 41 0f 10 84 24 94 00 00 00       	movss	0x94(%r12), %xmm0
140ac11b2: f3 0f 11 85 40 01 00 00     	movss	%xmm0, 0x140(%rbp)
140ac11ba: c6 85 c5 01 00 00 00        	movb	$0x0, 0x1c5(%rbp)
140ac11c1: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac11c8: 48 89 f1                    	movq	%rsi, %rcx
140ac11cb: e8 f0 98 fe ff              	callq	0x140aaaac0 <.text+0xaa9ac0>
140ac11d0: e9 08 f8 ff ff              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac11d5: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140ac11dc: 0f 28 a5 a0 00 00 00        	movaps	0xa0(%rbp), %xmm4
140ac11e3: 0f 28 ad b0 00 00 00        	movaps	0xb0(%rbp), %xmm5
140ac11ea: 0f 11 a5 d8 00 00 00        	movups	%xmm4, 0xd8(%rbp)
140ac11f1: 0f 11 ad e8 00 00 00        	movups	%xmm5, 0xe8(%rbp)
140ac11f8: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140ac11ff: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140ac1206: f3 0f 11 85 00 01 00 00     	movss	%xmm0, 0x100(%rbp)
140ac120e: 89 8d 08 01 00 00           	movl	%ecx, 0x108(%rbp)
140ac1214: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
140ac1218: 0f 11 85 0c 01 00 00        	movups	%xmm0, 0x10c(%rbp)
140ac121f: 48 8b 45 20                 	movq	0x20(%rbp), %rax
140ac1223: 48 89 85 1c 01 00 00        	movq	%rax, 0x11c(%rbp)
140ac122a: 0f 13 8d 28 01 00 00        	movlps	%xmm1, 0x128(%rbp)
140ac1231: f3 0f 11 9d 30 01 00 00     	movss	%xmm3, 0x130(%rbp)
140ac1239: 66 0f 7e 95 34 01 00 00     	movd	%xmm2, 0x134(%rbp)
140ac1241: 66 0f 7e d1                 	movd	%xmm2, %ecx
140ac1245: 88 95 38 01 00 00           	movb	%dl, 0x138(%rbp)
140ac124b: 44 88 85 39 01 00 00        	movb	%r8b, 0x139(%rbp)
140ac1252: f3 41 0f 10 84 24 90 00 00 00       	movss	0x90(%r12), %xmm0
140ac125c: f3 0f 11 85 24 01 00 00     	movss	%xmm0, 0x124(%rbp)
140ac1264: 49 8d 9c 24 88 00 00 00     	leaq	0x88(%r12), %rbx
140ac126c: 49 8d 84 24 98 00 00 00     	leaq	0x98(%r12), %rax
140ac1274: 0f ba e1 08                 	btl	$0x8, %ecx
140ac1278: 0f 83 6f fe ff ff           	jae	0x140ac10ed <.text+0xac00ed>
140ac127e: 48 8b 0b                    	movq	(%rbx), %rcx
140ac1281: 8b 00                       	movl	(%rax), %eax
140ac1283: 48 89 8d 40 01 00 00        	movq	%rcx, 0x140(%rbp)
140ac128a: 89 85 48 01 00 00           	movl	%eax, 0x148(%rbp)
140ac1290: c6 85 4c 01 00 00 00        	movb	$0x0, 0x14c(%rbp)
140ac1297: c6 85 c5 01 00 00 01        	movb	$0x1, 0x1c5(%rbp)
140ac129e: 4c 8d 05 1b dd 01 02        	leaq	0x201dd1b(%rip), %r8    # 0x142adefc0
140ac12a5: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac12ac: 4c 89 e9                    	movq	%r13, %rcx
140ac12af: e8 bc 60 86 00              	callq	0x141327370 <.text+0x1326370>
140ac12b4: e9 6a fe ff ff              	jmp	0x140ac1123 <.text+0xac0123>
140ac12b9: b9 03 00 00 00              	movl	$0x3, %ecx
140ac12be: f3 41 0f 10 4c 24 6c        	movss	0x6c(%r12), %xmm1
140ac12c5: f2 41 0f 10 54 24 70        	movsd	0x70(%r12), %xmm2
140ac12cc: 41 0f b6 94 24 80 00 00 00  	movzbl	0x80(%r12), %edx
140ac12d5: 45 0f b6 84 24 81 00 00 00  	movzbl	0x81(%r12), %r8d
140ac12de: f3 41 0f 10 64 24 78        	movss	0x78(%r12), %xmm4
140ac12e5: 66 41 0f 6e 5c 24 7c        	movd	0x7c(%r12), %xmm3
140ac12ec: 4c 8b 8d 60 01 00 00        	movq	0x160(%rbp), %r9
140ac12f3: 4c 89 8d c0 00 00 00        	movq	%r9, 0xc0(%rbp)
140ac12fa: 0f 28 ad 40 01 00 00        	movaps	0x140(%rbp), %xmm5
140ac1301: 0f 28 b5 50 01 00 00        	movaps	0x150(%rbp), %xmm6
140ac1308: 0f 29 b5 b0 00 00 00        	movaps	%xmm6, 0xb0(%rbp)
140ac130f: 0f 29 ad a0 00 00 00        	movaps	%xmm5, 0xa0(%rbp)
140ac1316: 49 89 c1                    	movq	%rax, %r9
140ac1319: 49 f7 d9                    	negq	%r9
140ac131c: 0f 81 49 01 00 00           	jno	0x140ac146b <.text+0xac046b>
140ac1322: f3 41 0f 10 84 24 90 00 00 00       	movss	0x90(%r12), %xmm0
140ac132c: 0f 57 c9                    	xorps	%xmm1, %xmm1
140ac132f: 0f 11 8d e0 00 00 00        	movups	%xmm1, 0xe0(%rbp)
140ac1336: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
140ac1341: 48 c7 85 d8 00 00 00 04 00 00 00    	movq	$0x4, 0xd8(%rbp)
140ac134c: 48 c7 85 f0 00 00 00 04 00 00 00    	movq	$0x4, 0xf0(%rbp)
140ac1357: 48 c7 85 f8 00 00 00 00 00 00 00    	movq	$0x0, 0xf8(%rbp)
140ac1362: c7 85 00 01 00 00 00 00 00 00       	movl	$0x0, 0x100(%rbp)
140ac136c: f3 0f 11 85 24 01 00 00     	movss	%xmm0, 0x124(%rbp)
140ac1374: c7 85 08 01 00 00 02 00 00 00       	movl	$0x2, 0x108(%rbp)
140ac137e: f2 0f 10 05 8a 35 fd 01     	movsd	0x1fd358a(%rip), %xmm0  # 0x142a94910
140ac1386: f2 0f 11 85 28 01 00 00     	movsd	%xmm0, 0x128(%rbp)
140ac138e: 48 c7 85 30 01 00 00 00 00 00 00    	movq	$0x0, 0x130(%rbp)
140ac1399: 66 c7 85 38 01 00 00 00 00  	movw	$0x0, 0x138(%rbp)
140ac13a2: 49 8d b4 24 88 00 00 00     	leaq	0x88(%r12), %rsi
140ac13aa: 49 81 c4 94 00 00 00        	addq	$0x94, %r12
140ac13b1: 48 8b 06                    	movq	(%rsi), %rax
140ac13b4: 41 8b 0c 24                 	movl	(%r12), %ecx
140ac13b8: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140ac13bf: 89 8d 48 01 00 00           	movl	%ecx, 0x148(%rbp)
140ac13c5: c6 85 4c 01 00 00 00        	movb	$0x0, 0x14c(%rbp)
140ac13cc: c6 85 c4 01 00 00 01        	movb	$0x1, 0x1c4(%rbp)
140ac13d3: 4c 8d 05 fe db 01 02        	leaq	0x201dbfe(%rip), %r8    # 0x142adefd8
140ac13da: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac13e1: 4c 89 f1                    	movq	%r14, %rcx
140ac13e4: e8 87 5f 86 00              	callq	0x141327370 <.text+0x1326370>
140ac13e9: 48 89 d9                    	movq	%rbx, %rcx
140ac13ec: 48 8b 16                    	movq	(%rsi), %rdx
140ac13ef: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140ac13f6: 0f 29 85 a0 01 00 00        	movaps	%xmm0, 0x1a0(%rbp)
140ac13fd: 0f 10 85 20 01 00 00        	movups	0x120(%rbp), %xmm0
140ac1404: 0f 29 85 90 01 00 00        	movaps	%xmm0, 0x190(%rbp)
140ac140b: 0f 10 85 10 01 00 00        	movups	0x110(%rbp), %xmm0
140ac1412: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
140ac1419: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140ac1420: 0f 10 8d e0 00 00 00        	movups	0xe0(%rbp), %xmm1
140ac1427: 0f 10 95 f0 00 00 00        	movups	0xf0(%rbp), %xmm2
140ac142e: f3 0f 6f 9d 00 01 00 00     	movdqu	0x100(%rbp), %xmm3
140ac1436: 66 0f 7f 9d 70 01 00 00     	movdqa	%xmm3, 0x170(%rbp)
140ac143e: 0f 29 95 60 01 00 00        	movaps	%xmm2, 0x160(%rbp)
140ac1445: 0f 29 8d 50 01 00 00        	movaps	%xmm1, 0x150(%rbp)
140ac144c: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
140ac1453: c6 85 c4 01 00 00 00        	movb	$0x0, 0x1c4(%rbp)
140ac145a: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
140ac1461: e8 3a 90 d9 ff              	callq	0x14085a4a0 <.text+0x8594a0>
140ac1466: e9 72 f5 ff ff              	jmp	0x140ac09dd <.text+0xabf9dd>
140ac146b: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
140ac1472: 0f 28 ad a0 00 00 00        	movaps	0xa0(%rbp), %xmm5
140ac1479: 0f 28 b5 b0 00 00 00        	movaps	0xb0(%rbp), %xmm6
140ac1480: 0f 11 ad d8 00 00 00        	movups	%xmm5, 0xd8(%rbp)
140ac1487: 0f 11 b5 e8 00 00 00        	movups	%xmm6, 0xe8(%rbp)
140ac148e: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140ac1495: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140ac149c: f3 0f 11 85 00 01 00 00     	movss	%xmm0, 0x100(%rbp)
140ac14a4: 89 8d 08 01 00 00           	movl	%ecx, 0x108(%rbp)
140ac14aa: 0f 28 45 f0                 	movaps	-0x10(%rbp), %xmm0
140ac14ae: 0f 11 85 0c 01 00 00        	movups	%xmm0, 0x10c(%rbp)
140ac14b5: 48 8b 45 00                 	movq	(%rbp), %rax
140ac14b9: 48 89 85 1c 01 00 00        	movq	%rax, 0x11c(%rbp)
140ac14c0: f3 0f 11 8d 24 01 00 00     	movss	%xmm1, 0x124(%rbp)
140ac14c8: 0f 13 95 28 01 00 00        	movlps	%xmm2, 0x128(%rbp)
140ac14cf: f3 0f 11 a5 30 01 00 00     	movss	%xmm4, 0x130(%rbp)
140ac14d7: 66 0f 7e 9d 34 01 00 00     	movd	%xmm3, 0x134(%rbp)
140ac14df: 66 0f 7e d8                 	movd	%xmm3, %eax
140ac14e3: 88 95 38 01 00 00           	movb	%dl, 0x138(%rbp)
140ac14e9: 44 88 85 39 01 00 00        	movb	%r8b, 0x139(%rbp)
140ac14f0: 49 8d b4 24 88 00 00 00     	leaq	0x88(%r12), %rsi
140ac14f8: 49 81 c4 94 00 00 00        	addq	$0x94, %r12
140ac14ff: 0f ba e0 08                 	btl	$0x8, %eax
140ac1503: 0f 83 a8 fe ff ff           	jae	0x140ac13b1 <.text+0xac03b1>
140ac1509: 48 8b 06                    	movq	(%rsi), %rax
140ac150c: 41 8b 0c 24                 	movl	(%r12), %ecx
140ac1510: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140ac1517: 89 8d 48 01 00 00           	movl	%ecx, 0x148(%rbp)
140ac151d: c6 85 4c 01 00 00 00        	movb	$0x0, 0x14c(%rbp)
140ac1524: c6 85 c4 01 00 00 01        	movb	$0x1, 0x1c4(%rbp)
140ac152b: 4c 8d 05 be da 01 02        	leaq	0x201dabe(%rip), %r8    # 0x142adeff0
140ac1532: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
140ac1539: 4c 89 e9                    	movq	%r13, %rcx
140ac153c: e8 2f 5e 86 00              	callq	0x141327370 <.text+0x1326370>
140ac1541: e9 a3 fe ff ff              	jmp	0x140ac13e9 <.text+0xac03e9>
140ac1546: 48 8d 0d 23 db 01 02        	leaq	0x201db23(%rip), %rcx   # 0x142adf070
140ac154d: 4c 8d 05 94 db 01 02        	leaq	0x201db94(%rip), %r8    # 0x142adf0e8
140ac1554: ba 29 00 00 00              	movl	$0x29, %edx
140ac1559: e8 d2 7e e1 01              	callq	0x1428d9430 <.text+0x28d8430>
140ac155e: 48 8d 0d 0b db 01 02        	leaq	0x201db0b(%rip), %rcx   # 0x142adf070
140ac1565: 4c 8d 05 34 db 01 02        	leaq	0x201db34(%rip), %r8    # 0x142adf0a0
140ac156c: ba 29 00 00 00              	movl	$0x29, %edx
140ac1571: e8 ba 7e e1 01              	callq	0x1428d9430 <.text+0x28d8430>
140ac1576: 48 8d 05 eb dc 01 02        	leaq	0x201dceb(%rip), %rax   # 0x142adf268
140ac157d: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140ac1584: 48 c7 85 48 01 00 00 01 00 00 00    	movq	$0x1, 0x148(%rbp)
140ac158f: 48 c7 85 50 01 00 00 08 00 00 00    	movq	$0x8, 0x150(%rbp)
140ac159a: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ac159d: 0f 11 85 58 01 00 00        	movups	%xmm0, 0x158(%rbp)
140ac15a4: 48 8d 15 cd dc 01 02        	leaq	0x201dccd(%rip), %rdx   # 0x142adf278
140ac15ab: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
140ac15b2: e8 d9 7d e1 01              	callq	0x1428d9390 <.text+0x28d8390>
140ac15b7: 48 8d 0d 62 da 01 02        	leaq	0x201da62(%rip), %rcx   # 0x142adf020
140ac15be: 4c 8d 05 79 da 01 02        	leaq	0x201da79(%rip), %r8    # 0x142adf03e
140ac15c5: ba 1e 00 00 00              	movl	$0x1e, %edx
140ac15ca: 41 b9 1a 00 00 00           	movl	$0x1a, %r9d
140ac15d0: e8 4b 18 fc ff              	callq	0x140a82e20 <.text+0xa81e20>
140ac15d5: 48 8d 05 dc db 01 02        	leaq	0x201dbdc(%rip), %rax   # 0x142adf1b8
140ac15dc: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
140ac15e3: 48 c7 85 48 01 00 00 01 00 00 00    	movq	$0x1, 0x148(%rbp)
140ac15ee: 48 c7 85 50 01 00 00 08 00 00 00    	movq	$0x8, 0x150(%rbp)
140ac15f9: 0f 57 c0                    	xorps	%xmm0, %xmm0
140ac15fc: 0f 11 85 58 01 00 00        	movups	%xmm0, 0x158(%rbp)
140ac1603: 48 8d 15 be db 01 02        	leaq	0x201dbbe(%rip), %rdx   # 0x142adf1c8
140ac160a: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
140ac1611: e8 7a 7d e1 01              	callq	0x1428d9390 <.text+0x28d8390>
140ac1616: 48 8d 0d 33 db 01 02        	leaq	0x201db33(%rip), %rcx   # 0x142adf150
140ac161d: 4c 8d 05 54 db 01 02        	leaq	0x201db54(%rip), %r8    # 0x142adf178
140ac1624: ba 28 00 00 00              	movl	$0x28, %edx
140ac1629: e8 02 7e e1 01              	callq	0x1428d9430 <.text+0x28d8430>
140ac162e: cc                          	int3
140ac162f: 90                          	nop
140ac1630: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1635: 55                          	pushq	%rbp
140ac1636: 41 57                       	pushq	%r15
140ac1638: 41 56                       	pushq	%r14
140ac163a: 41 55                       	pushq	%r13
140ac163c: 41 54                       	pushq	%r12
140ac163e: 56                          	pushq	%rsi
140ac163f: 57                          	pushq	%rdi
140ac1640: 53                          	pushq	%rbx
140ac1641: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1648: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac164f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1655: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac165b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1661: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1667: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac166d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1673: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac167c: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1684: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac168c: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
140ac1693: 48 85 d2                    	testq	%rdx, %rdx
140ac1696: 74 16                       	je	0x140ac16ae <.text+0xac06ae>
140ac1698: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140ac169f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140ac16a3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac16a9: e8 72 25 b5 ff              	callq	0x140613c20 <.text+0x612c20>
140ac16ae: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac16b6: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac16be: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac16c7: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac16cd: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac16d3: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac16d9: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac16df: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac16e5: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac16eb: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac16f2: 5b                          	popq	%rbx
140ac16f3: 5f                          	popq	%rdi
140ac16f4: 5e                          	popq	%rsi
140ac16f5: 41 5c                       	popq	%r12
140ac16f7: 41 5d                       	popq	%r13
140ac16f9: 41 5e                       	popq	%r14
140ac16fb: 41 5f                       	popq	%r15
140ac16fd: 5d                          	popq	%rbp
140ac16fe: c3                          	retq
140ac16ff: 90                          	nop
140ac1700: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1705: 55                          	pushq	%rbp
140ac1706: 41 57                       	pushq	%r15
140ac1708: 41 56                       	pushq	%r14
140ac170a: 41 55                       	pushq	%r13
140ac170c: 41 54                       	pushq	%r12
140ac170e: 56                          	pushq	%rsi
140ac170f: 57                          	pushq	%rdi
140ac1710: 53                          	pushq	%rbx
140ac1711: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1718: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac171f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1725: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac172b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1731: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1737: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac173d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1743: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac174c: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1754: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac175c: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
140ac1763: 48 85 d2                    	testq	%rdx, %rdx
140ac1766: 74 16                       	je	0x140ac177e <.text+0xac077e>
140ac1768: 48 8b 8d 90 00 00 00        	movq	0x90(%rbp), %rcx
140ac176f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140ac1773: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac1779: e8 a2 24 b5 ff              	callq	0x140613c20 <.text+0x612c20>
140ac177e: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1786: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac178e: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1797: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac179d: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac17a3: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac17a9: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac17af: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac17b5: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac17bb: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac17c2: 5b                          	popq	%rbx
140ac17c3: 5f                          	popq	%rdi
140ac17c4: 5e                          	popq	%rsi
140ac17c5: 41 5c                       	popq	%r12
140ac17c7: 41 5d                       	popq	%r13
140ac17c9: 41 5e                       	popq	%r14
140ac17cb: 41 5f                       	popq	%r15
140ac17cd: 5d                          	popq	%rbp
140ac17ce: c3                          	retq
140ac17cf: 90                          	nop
140ac17d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac17d5: 55                          	pushq	%rbp
140ac17d6: 41 57                       	pushq	%r15
140ac17d8: 41 56                       	pushq	%r14
140ac17da: 41 55                       	pushq	%r13
140ac17dc: 41 54                       	pushq	%r12
140ac17de: 56                          	pushq	%rsi
140ac17df: 57                          	pushq	%rdi
140ac17e0: 53                          	pushq	%rbx
140ac17e1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac17e8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac17ef: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac17f5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac17fb: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1801: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1807: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac180d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1813: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac181c: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1824: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac182c: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140ac1833: 48 85 d2                    	testq	%rdx, %rdx
140ac1836: 74 16                       	je	0x140ac184e <.text+0xac084e>
140ac1838: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140ac183f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140ac1843: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac1849: e8 d2 23 b5 ff              	callq	0x140613c20 <.text+0x612c20>
140ac184e: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1856: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac185e: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1867: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac186d: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1873: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1879: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac187f: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1885: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac188b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1892: 5b                          	popq	%rbx
140ac1893: 5f                          	popq	%rdi
140ac1894: 5e                          	popq	%rsi
140ac1895: 41 5c                       	popq	%r12
140ac1897: 41 5d                       	popq	%r13
140ac1899: 41 5e                       	popq	%r14
140ac189b: 41 5f                       	popq	%r15
140ac189d: 5d                          	popq	%rbp
140ac189e: c3                          	retq
140ac189f: 90                          	nop
140ac18a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac18a5: 55                          	pushq	%rbp
140ac18a6: 41 57                       	pushq	%r15
140ac18a8: 41 56                       	pushq	%r14
140ac18aa: 41 55                       	pushq	%r13
140ac18ac: 41 54                       	pushq	%r12
140ac18ae: 56                          	pushq	%rsi
140ac18af: 57                          	pushq	%rdi
140ac18b0: 53                          	pushq	%rbx
140ac18b1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac18b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac18bf: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac18c5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac18cb: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac18d1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac18d7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac18dd: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac18e3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac18ec: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac18f4: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac18fc: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140ac1903: 48 85 d2                    	testq	%rdx, %rdx
140ac1906: 74 16                       	je	0x140ac191e <.text+0xac091e>
140ac1908: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140ac190f: 48 c1 e2 03                 	shlq	$0x3, %rdx
140ac1913: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac1919: e8 02 23 b5 ff              	callq	0x140613c20 <.text+0x612c20>
140ac191e: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1926: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac192e: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1937: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac193d: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1943: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1949: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac194f: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1955: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac195b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1962: 5b                          	popq	%rbx
140ac1963: 5f                          	popq	%rdi
140ac1964: 5e                          	popq	%rsi
140ac1965: 41 5c                       	popq	%r12
140ac1967: 41 5d                       	popq	%r13
140ac1969: 41 5e                       	popq	%r14
140ac196b: 41 5f                       	popq	%r15
140ac196d: 5d                          	popq	%rbp
140ac196e: c3                          	retq
140ac196f: 90                          	nop
140ac1970: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1975: 55                          	pushq	%rbp
140ac1976: 41 57                       	pushq	%r15
140ac1978: 41 56                       	pushq	%r14
140ac197a: 41 55                       	pushq	%r13
140ac197c: 41 54                       	pushq	%r12
140ac197e: 56                          	pushq	%rsi
140ac197f: 57                          	pushq	%rdi
140ac1980: 53                          	pushq	%rbx
140ac1981: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1988: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac198f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1995: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac199b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac19a1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac19a7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac19ad: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac19b3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac19bc: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac19c4: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac19cc: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140ac19d3: 48 85 d2                    	testq	%rdx, %rdx
140ac19d6: 74 16                       	je	0x140ac19ee <.text+0xac09ee>
140ac19d8: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140ac19df: 48 c1 e2 03                 	shlq	$0x3, %rdx
140ac19e3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac19e9: e8 32 22 b5 ff              	callq	0x140613c20 <.text+0x612c20>
140ac19ee: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac19f6: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac19fe: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1a07: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac1a0d: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1a13: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1a19: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac1a1f: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1a25: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac1a2b: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1a32: 5b                          	popq	%rbx
140ac1a33: 5f                          	popq	%rdi
140ac1a34: 5e                          	popq	%rsi
140ac1a35: 41 5c                       	popq	%r12
140ac1a37: 41 5d                       	popq	%r13
140ac1a39: 41 5e                       	popq	%r14
140ac1a3b: 41 5f                       	popq	%r15
140ac1a3d: 5d                          	popq	%rbp
140ac1a3e: c3                          	retq
140ac1a3f: 90                          	nop
140ac1a40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1a45: 55                          	pushq	%rbp
140ac1a46: 41 57                       	pushq	%r15
140ac1a48: 41 56                       	pushq	%r14
140ac1a4a: 41 55                       	pushq	%r13
140ac1a4c: 41 54                       	pushq	%r12
140ac1a4e: 56                          	pushq	%rsi
140ac1a4f: 57                          	pushq	%rdi
140ac1a50: 53                          	pushq	%rbx
140ac1a51: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1a58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac1a5f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1a65: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac1a6b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1a71: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1a77: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac1a7d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1a83: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac1a8c: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1a94: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac1a9c: 48 8b 95 a0 00 00 00        	movq	0xa0(%rbp), %rdx
140ac1aa3: 48 85 d2                    	testq	%rdx, %rdx
140ac1aa6: 74 16                       	je	0x140ac1abe <.text+0xac0abe>
140ac1aa8: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
140ac1aaf: 48 c1 e2 03                 	shlq	$0x3, %rdx
140ac1ab3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
140ac1ab9: e8 62 21 b5 ff              	callq	0x140613c20 <.text+0x612c20>
140ac1abe: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1ac6: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac1ace: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1ad7: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac1add: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1ae3: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1ae9: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac1aef: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1af5: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac1afb: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1b02: 5b                          	popq	%rbx
140ac1b03: 5f                          	popq	%rdi
140ac1b04: 5e                          	popq	%rsi
140ac1b05: 41 5c                       	popq	%r12
140ac1b07: 41 5d                       	popq	%r13
140ac1b09: 41 5e                       	popq	%r14
140ac1b0b: 41 5f                       	popq	%r15
140ac1b0d: 5d                          	popq	%rbp
140ac1b0e: c3                          	retq
140ac1b0f: 90                          	nop
140ac1b10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1b15: 55                          	pushq	%rbp
140ac1b16: 41 57                       	pushq	%r15
140ac1b18: 41 56                       	pushq	%r14
140ac1b1a: 41 55                       	pushq	%r13
140ac1b1c: 41 54                       	pushq	%r12
140ac1b1e: 56                          	pushq	%rsi
140ac1b1f: 57                          	pushq	%rdi
140ac1b20: 53                          	pushq	%rbx
140ac1b21: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1b28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac1b2f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1b35: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac1b3b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1b41: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1b47: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac1b4d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1b53: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac1b5c: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1b64: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac1b6c: 80 bd c4 01 00 00 00        	cmpb	$0x0, 0x1c4(%rbp)
140ac1b73: 75 51                       	jne	0x140ac1bc6 <.text+0xac0bc6>
140ac1b75: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1b7d: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac1b85: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1b8e: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac1b94: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1b9a: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1ba0: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac1ba6: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1bac: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac1bb2: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1bb9: 5b                          	popq	%rbx
140ac1bba: 5f                          	popq	%rdi
140ac1bbb: 5e                          	popq	%rsi
140ac1bbc: 41 5c                       	popq	%r12
140ac1bbe: 41 5d                       	popq	%r13
140ac1bc0: 41 5e                       	popq	%r14
140ac1bc2: 41 5f                       	popq	%r15
140ac1bc4: 5d                          	popq	%rbp
140ac1bc5: c3                          	retq
140ac1bc6: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac1bcd: e8 1e 09 b1 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140ac1bd2: eb a1                       	jmp	0x140ac1b75 <.text+0xac0b75>
140ac1bd4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140ac1be0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1be5: 55                          	pushq	%rbp
140ac1be6: 41 57                       	pushq	%r15
140ac1be8: 41 56                       	pushq	%r14
140ac1bea: 41 55                       	pushq	%r13
140ac1bec: 41 54                       	pushq	%r12
140ac1bee: 56                          	pushq	%rsi
140ac1bef: 57                          	pushq	%rdi
140ac1bf0: 53                          	pushq	%rbx
140ac1bf1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1bf8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac1bff: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1c05: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac1c0b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1c11: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1c17: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac1c1d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1c23: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac1c2c: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1c34: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac1c3c: 80 bd c5 01 00 00 00        	cmpb	$0x0, 0x1c5(%rbp)
140ac1c43: 75 51                       	jne	0x140ac1c96 <.text+0xac0c96>
140ac1c45: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1c4d: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac1c55: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1c5e: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac1c64: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1c6a: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1c70: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac1c76: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1c7c: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac1c82: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1c89: 5b                          	popq	%rbx
140ac1c8a: 5f                          	popq	%rdi
140ac1c8b: 5e                          	popq	%rsi
140ac1c8c: 41 5c                       	popq	%r12
140ac1c8e: 41 5d                       	popq	%r13
140ac1c90: 41 5e                       	popq	%r14
140ac1c92: 41 5f                       	popq	%r15
140ac1c94: 5d                          	popq	%rbp
140ac1c95: c3                          	retq
140ac1c96: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac1c9d: e8 4e 08 b1 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140ac1ca2: eb a1                       	jmp	0x140ac1c45 <.text+0xac0c45>
140ac1ca4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140ac1cb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1cb5: 55                          	pushq	%rbp
140ac1cb6: 41 57                       	pushq	%r15
140ac1cb8: 41 56                       	pushq	%r14
140ac1cba: 41 55                       	pushq	%r13
140ac1cbc: 41 54                       	pushq	%r12
140ac1cbe: 56                          	pushq	%rsi
140ac1cbf: 57                          	pushq	%rdi
140ac1cc0: 53                          	pushq	%rbx
140ac1cc1: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1cc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac1ccf: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1cd5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac1cdb: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1ce1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1ce7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac1ced: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1cf3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac1cfc: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1d04: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac1d0c: 80 bd c3 01 00 00 00        	cmpb	$0x0, 0x1c3(%rbp)
140ac1d13: 75 51                       	jne	0x140ac1d66 <.text+0xac0d66>
140ac1d15: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1d1d: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac1d25: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1d2e: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac1d34: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1d3a: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1d40: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac1d46: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1d4c: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac1d52: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1d59: 5b                          	popq	%rbx
140ac1d5a: 5f                          	popq	%rdi
140ac1d5b: 5e                          	popq	%rsi
140ac1d5c: 41 5c                       	popq	%r12
140ac1d5e: 41 5d                       	popq	%r13
140ac1d60: 41 5e                       	popq	%r14
140ac1d62: 41 5f                       	popq	%r15
140ac1d64: 5d                          	popq	%rbp
140ac1d65: c3                          	retq
140ac1d66: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac1d6d: e8 7e 07 b1 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140ac1d72: eb a1                       	jmp	0x140ac1d15 <.text+0xac0d15>
140ac1d74: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140ac1d80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1d85: 55                          	pushq	%rbp
140ac1d86: 41 57                       	pushq	%r15
140ac1d88: 41 56                       	pushq	%r14
140ac1d8a: 41 55                       	pushq	%r13
140ac1d8c: 41 54                       	pushq	%r12
140ac1d8e: 56                          	pushq	%rsi
140ac1d8f: 57                          	pushq	%rdi
140ac1d90: 53                          	pushq	%rbx
140ac1d91: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1d98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac1d9f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1da5: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac1dab: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1db1: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1db7: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac1dbd: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1dc3: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac1dcc: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1dd4: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac1ddc: 80 bd c6 01 00 00 00        	cmpb	$0x0, 0x1c6(%rbp)
140ac1de3: 75 51                       	jne	0x140ac1e36 <.text+0xac0e36>
140ac1de5: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1ded: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac1df5: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1dfe: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac1e04: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1e0a: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1e10: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac1e16: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1e1c: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac1e22: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1e29: 5b                          	popq	%rbx
140ac1e2a: 5f                          	popq	%rdi
140ac1e2b: 5e                          	popq	%rsi
140ac1e2c: 41 5c                       	popq	%r12
140ac1e2e: 41 5d                       	popq	%r13
140ac1e30: 41 5e                       	popq	%r14
140ac1e32: 41 5f                       	popq	%r15
140ac1e34: 5d                          	popq	%rbp
140ac1e35: c3                          	retq
140ac1e36: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac1e3d: e8 ae 06 b1 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140ac1e42: eb a1                       	jmp	0x140ac1de5 <.text+0xac0de5>
140ac1e44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140ac1e50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140ac1e55: 55                          	pushq	%rbp
140ac1e56: 41 57                       	pushq	%r15
140ac1e58: 41 56                       	pushq	%r14
140ac1e5a: 41 55                       	pushq	%r13
140ac1e5c: 41 54                       	pushq	%r12
140ac1e5e: 56                          	pushq	%rsi
140ac1e5f: 57                          	pushq	%rdi
140ac1e60: 53                          	pushq	%rbx
140ac1e61: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
140ac1e68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140ac1e6f: 44 0f 29 74 24 20           	movaps	%xmm14, 0x20(%rsp)
140ac1e75: 44 0f 29 6c 24 30           	movaps	%xmm13, 0x30(%rsp)
140ac1e7b: 44 0f 29 64 24 40           	movaps	%xmm12, 0x40(%rsp)
140ac1e81: 44 0f 29 5c 24 50           	movaps	%xmm11, 0x50(%rsp)
140ac1e87: 44 0f 29 54 24 60           	movaps	%xmm10, 0x60(%rsp)
140ac1e8d: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
140ac1e93: 44 0f 29 84 24 80 00 00 00  	movaps	%xmm8, 0x80(%rsp)
140ac1e9c: 0f 29 bc 24 90 00 00 00     	movaps	%xmm7, 0x90(%rsp)
140ac1ea4: 0f 29 b4 24 a0 00 00 00     	movaps	%xmm6, 0xa0(%rsp)
140ac1eac: 80 bd c7 01 00 00 00        	cmpb	$0x0, 0x1c7(%rbp)
140ac1eb3: 75 51                       	jne	0x140ac1f06 <.text+0xac0f06>
140ac1eb5: 0f 28 b4 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm6
140ac1ebd: 0f 28 bc 24 90 00 00 00     	movaps	0x90(%rsp), %xmm7
140ac1ec5: 44 0f 28 84 24 80 00 00 00  	movaps	0x80(%rsp), %xmm8
140ac1ece: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
140ac1ed4: 44 0f 28 54 24 60           	movaps	0x60(%rsp), %xmm10
140ac1eda: 44 0f 28 5c 24 50           	movaps	0x50(%rsp), %xmm11
140ac1ee0: 44 0f 28 64 24 40           	movaps	0x40(%rsp), %xmm12
140ac1ee6: 44 0f 28 6c 24 30           	movaps	0x30(%rsp), %xmm13
140ac1eec: 44 0f 28 74 24 20           	movaps	0x20(%rsp), %xmm14
140ac1ef2: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
140ac1ef9: 5b                          	popq	%rbx
140ac1efa: 5f                          	popq	%rdi
140ac1efb: 5e                          	popq	%rsi
140ac1efc: 41 5c                       	popq	%r12
140ac1efe: 41 5d                       	popq	%r13
140ac1f00: 41 5e                       	popq	%r14
140ac1f02: 41 5f                       	popq	%r15
140ac1f04: 5d                          	popq	%rbp
140ac1f05: c3                          	retq
140ac1f06: 48 8d 8d d0 00 00 00        	leaq	0xd0(%rbp), %rcx
140ac1f0d: e8 de 05 b1 ff              	callq	0x1405d24f0 <.text+0x5d14f0>
140ac1f12: eb a1                       	jmp	0x140ac1eb5 <.text+0xac0eb5>
