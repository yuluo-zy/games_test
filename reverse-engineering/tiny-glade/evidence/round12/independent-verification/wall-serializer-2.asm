
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
