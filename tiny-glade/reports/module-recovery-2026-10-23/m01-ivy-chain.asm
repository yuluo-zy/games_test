
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141450fa0: 55                          	pushq	%rbp
141450fa1: 56                          	pushq	%rsi
141450fa2: 57                          	pushq	%rdi
141450fa3: 48 81 ec 20 04 00 00        	subq	$0x420, %rsp            # imm = 0x420
141450faa: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141450fb2: 48 c7 85 98 03 00 00 fe ff ff ff    	movq	$-0x2, 0x398(%rbp)
141450fbd: 48 89 d7                    	movq	%rdx, %rdi
141450fc0: 48 89 ce                    	movq	%rcx, %rsi
141450fc3: 0f 10 42 60                 	movups	0x60(%rdx), %xmm0
141450fc7: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
141450fcb: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
141450fcf: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
141450fd3: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
141450fd7: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
141450fdb: 0f 10 02                    	movups	(%rdx), %xmm0
141450fde: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
141450fe2: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
141450fe6: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
141450fea: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
141450fee: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
141450ff2: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
141450ff6: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
141450ffa: 0f b6 05 08 3c 1e 02        	movzbl	0x21e3c08(%rip), %eax   # 0x143634c09
141451001: b9 80 03 00 00              	movl	$0x380, %ecx            # imm = 0x380
141451006: ba 08 00 00 00              	movl	$0x8, %edx
14145100b: e8 00 2c 1c ff              	callq	0x140613c10 <.text+0x612c10>
141451010: 48 89 85 90 03 00 00        	movq	%rax, 0x390(%rbp)
141451017: 48 85 c0                    	testq	%rax, %rax
14145101a: 0f 84 3a 04 00 00           	je	0x14145145a <.text+0x145045a>
141451020: 0f 10 47 60                 	movups	0x60(%rdi), %xmm0
141451024: 0f 29 85 10 03 00 00        	movaps	%xmm0, 0x310(%rbp)
14145102b: 0f 10 47 50                 	movups	0x50(%rdi), %xmm0
14145102f: 0f 29 85 00 03 00 00        	movaps	%xmm0, 0x300(%rbp)
141451036: 0f 10 47 40                 	movups	0x40(%rdi), %xmm0
14145103a: 0f 29 85 f0 02 00 00        	movaps	%xmm0, 0x2f0(%rbp)
141451041: 0f 10 07                    	movups	(%rdi), %xmm0
141451044: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
141451048: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
14145104c: 0f 10 5f 30                 	movups	0x30(%rdi), %xmm3
141451050: 0f 29 9d e0 02 00 00        	movaps	%xmm3, 0x2e0(%rbp)
141451057: 0f 29 95 d0 02 00 00        	movaps	%xmm2, 0x2d0(%rbp)
14145105e: 0f 29 8d c0 02 00 00        	movaps	%xmm1, 0x2c0(%rbp)
141451065: 0f 29 85 b0 02 00 00        	movaps	%xmm0, 0x2b0(%rbp)
14145106c: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
141451073: e8 98 e3 64 ff              	callq	0x140a9f410 <.text+0xa9e410>
141451078: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14145107f: e8 ac 38 21 00              	callq	0x141664930 <.text+0x1663930>
141451084: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
14145108b: e8 50 33 21 00              	callq	0x1416643e0 <.text+0x16633e0>
141451090: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
141451097: e8 14 35 21 00              	callq	0x1416645b0 <.text+0x16635b0>
14145109c: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1414510a3: e8 58 3a 21 00              	callq	0x141664b00 <.text+0x1663b00>
1414510a8: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1414510ac: e8 3f 53 5d ff              	callq	0x140a263f0 <.text+0xa253f0>
1414510b1: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
1414510b8: e8 83 f8 37 00              	callq	0x1417d0940 <.text+0x17cf940>
1414510bd: 0f 28 85 10 03 00 00        	movaps	0x310(%rbp), %xmm0
1414510c4: 48 8b 85 90 03 00 00        	movq	0x390(%rbp), %rax
1414510cb: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1414510cf: 0f 28 85 00 03 00 00        	movaps	0x300(%rbp), %xmm0
1414510d6: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1414510da: 0f 28 85 f0 02 00 00        	movaps	0x2f0(%rbp), %xmm0
1414510e1: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1414510e5: 0f 28 85 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm0
1414510ec: 0f 28 8d c0 02 00 00        	movaps	0x2c0(%rbp), %xmm1
1414510f3: 0f 28 95 d0 02 00 00        	movaps	0x2d0(%rbp), %xmm2
1414510fa: 0f 28 9d e0 02 00 00        	movaps	0x2e0(%rbp), %xmm3
141451101: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
141451105: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
141451109: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14145110d: 0f 11 00                    	movups	%xmm0, (%rax)
141451110: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
141451117: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
14145111e: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
141451125: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
14145112c: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
141451130: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
141451137: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
14145113e: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
141451145: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
14145114c: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
141451153: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
14145115a: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
141451161: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
141451168: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
14145116f: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
141451176: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
14145117d: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
141451184: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
14145118b: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
141451192: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
141451199: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
1414511a0: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
1414511a7: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
1414511ae: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
1414511b5: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
1414511bc: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
1414511c3: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
1414511ca: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
1414511d1: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
1414511d8: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
1414511df: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
1414511e6: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
1414511ed: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
1414511f4: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
1414511fb: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
141451202: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
141451209: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
141451210: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
141451217: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
14145121e: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
141451225: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
14145122c: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
141451233: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
14145123a: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
141451241: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
141451248: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
14145124f: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
141451256: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
14145125d: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
141451264: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
14145126b: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
141451272: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
141451279: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
141451280: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
141451287: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
14145128e: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
141451295: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
14145129c: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
1414512a3: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
1414512aa: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
1414512b1: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1414512b8: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
1414512bf: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1414512c6: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
1414512cd: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
1414512d4: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
1414512db: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
1414512e2: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
1414512e9: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
1414512f0: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
1414512f7: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1414512fb: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1414512ff: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
141451303: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
141451307: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
14145130e: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
141451315: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
14145131c: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
141451323: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
141451327: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
14145132e: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
141451332: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
141451339: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
14145133d: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
141451344: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
14145134b: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
141451352: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
141451359: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
141451360: 0f 11 80 10 03 00 00        	movups	%xmm0, 0x310(%rax)
141451367: 0f 11 88 20 03 00 00        	movups	%xmm1, 0x320(%rax)
14145136e: 0f 11 90 30 03 00 00        	movups	%xmm2, 0x330(%rax)
141451375: 0f 11 98 40 03 00 00        	movups	%xmm3, 0x340(%rax)
14145137c: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
141451383: 0f 11 80 50 03 00 00        	movups	%xmm0, 0x350(%rax)
14145138a: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
141451391: 0f 11 80 70 03 00 00        	movups	%xmm0, 0x370(%rax)
141451398: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
14145139f: 0f 11 80 60 03 00 00        	movups	%xmm0, 0x360(%rax)
1414513a6: 48 c7 85 28 03 00 00 08 00 00 00    	movq	$0x8, 0x328(%rbp)
1414513b1: 48 89 85 30 03 00 00        	movq	%rax, 0x330(%rbp)
1414513b8: 48 c7 85 38 03 00 00 08 00 00 00    	movq	$0x8, 0x338(%rbp)
1414513c3: 48 c7 85 40 03 00 00 00 00 00 00    	movq	$0x0, 0x340(%rbp)
1414513ce: 48 c7 85 48 03 00 00 08 00 00 00    	movq	$0x8, 0x348(%rbp)
1414513d9: 48 8d 8d 58 03 00 00        	leaq	0x358(%rbp), %rcx
1414513e0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1414513e3: 0f 11 85 50 03 00 00        	movups	%xmm0, 0x350(%rbp)
1414513ea: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1414513f4: 48 89 85 20 03 00 00        	movq	%rax, 0x320(%rbp)
1414513fb: e8 a0 ec 24 00              	callq	0x1416a00a0 <.text+0x169f0a0>
141451400: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
141451407: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
14145140b: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
141451412: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
141451416: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
14145141d: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
141451421: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
141451428: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
14145142f: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
141451436: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
14145143d: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
141451441: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
141451445: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
141451449: 0f 11 06                    	movups	%xmm0, (%rsi)
14145144c: 48 89 f0                    	movq	%rsi, %rax
14145144f: 48 81 c4 20 04 00 00        	addq	$0x420, %rsp            # imm = 0x420
141451456: 5f                          	popq	%rdi
141451457: 5e                          	popq	%rsi
141451458: 5d                          	popq	%rbp
141451459: c3                          	retq
14145145a: b9 08 00 00 00              	movl	$0x8, %ecx
14145145f: ba 80 03 00 00              	movl	$0x380, %edx            # imm = 0x380
141451464: e8 97 7b 48 01              	callq	0x1428d9000 <.text+0x28d8000>
141451469: 0f 0b                       	ud2
14145146b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141451470: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141451475: 55                          	pushq	%rbp
141451476: 56                          	pushq	%rsi
141451477: 57                          	pushq	%rdi
141451478: 48 83 ec 20                 	subq	$0x20, %rsp
14145147c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141451483: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
14145148a: e8 f1 83 bd fe              	callq	0x140029880 <.text+0x28880>
14145148f: ba 80 03 00 00              	movl	$0x380, %edx            # imm = 0x380
141451494: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14145149a: 48 8b 8d 90 03 00 00        	movq	0x390(%rbp), %rcx
1414514a1: e8 7a 27 1c ff              	callq	0x140613c20 <.text+0x612c20>
1414514a6: 90                          	nop
1414514a7: 48 83 c4 20                 	addq	$0x20, %rsp
1414514ab: 5f                          	popq	%rdi
1414514ac: 5e                          	popq	%rsi
1414514ad: 5d                          	popq	%rbp
1414514ae: c3                          	retq
1414514af: 90                          	nop
1414514b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414514b5: 55                          	pushq	%rbp
1414514b6: 56                          	pushq	%rsi
1414514b7: 57                          	pushq	%rdi
1414514b8: 48 83 ec 20                 	subq	$0x20, %rsp
1414514bc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414514c3: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
1414514c7: e8 b4 83 bd fe              	callq	0x140029880 <.text+0x28880>
1414514cc: 90                          	nop
1414514cd: 48 83 c4 20                 	addq	$0x20, %rsp
1414514d1: 5f                          	popq	%rdi
1414514d2: 5e                          	popq	%rsi
1414514d3: 5d                          	popq	%rbp
1414514d4: c3                          	retq
1414514d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1414514e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414514e5: 55                          	pushq	%rbp
1414514e6: 56                          	pushq	%rsi
1414514e7: 57                          	pushq	%rdi
1414514e8: 48 83 ec 20                 	subq	$0x20, %rsp
1414514ec: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414514f3: 48 83 bd 90 03 00 00 00     	cmpq	$0x0, 0x390(%rbp)
1414514fb: 74 08                       	je	0x141451505 <.text+0x1450505>
1414514fd: 48 83 c4 20                 	addq	$0x20, %rsp
141451501: 5f                          	popq	%rdi
141451502: 5e                          	popq	%rsi
141451503: 5d                          	popq	%rbp
141451504: c3                          	retq
141451505: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141451509: e8 72 83 bd fe              	callq	0x140029880 <.text+0x28880>
14145150e: eb ed                       	jmp	0x1414514fd <.text+0x14504fd>
141451510: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141451515: 55                          	pushq	%rbp
141451516: 56                          	pushq	%rsi
141451517: 57                          	pushq	%rdi
141451518: 48 83 ec 20                 	subq	$0x20, %rsp
14145151c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141451523: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
14145152a: e8 51 83 bd fe              	callq	0x140029880 <.text+0x28880>
14145152f: 90                          	nop
141451530: 48 83 c4 20                 	addq	$0x20, %rsp
141451534: 5f                          	popq	%rdi
141451535: 5e                          	popq	%rsi
141451536: 5d                          	popq	%rbp
141451537: c3                          	retq
141451538: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141451540: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141451545: 55                          	pushq	%rbp
141451546: 56                          	pushq	%rsi
141451547: 57                          	pushq	%rdi
141451548: 48 83 ec 20                 	subq	$0x20, %rsp
14145154c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141451553: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
14145155a: e8 21 83 bd fe              	callq	0x140029880 <.text+0x28880>
14145155f: 90                          	nop
141451560: 48 83 c4 20                 	addq	$0x20, %rsp
141451564: 5f                          	popq	%rdi
141451565: 5e                          	popq	%rsi
141451566: 5d                          	popq	%rbp
141451567: c3                          	retq
141451568: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141451570: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141451575: 55                          	pushq	%rbp
141451576: 56                          	pushq	%rsi
141451577: 57                          	pushq	%rdi
141451578: 48 83 ec 20                 	subq	$0x20, %rsp
14145157c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141451583: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
14145158a: e8 f1 82 bd fe              	callq	0x140029880 <.text+0x28880>
14145158f: 90                          	nop
141451590: 48 83 c4 20                 	addq	$0x20, %rsp
141451594: 5f                          	popq	%rdi
141451595: 5e                          	popq	%rsi
141451596: 5d                          	popq	%rbp
141451597: c3                          	retq
141451598: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1414515a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414515a5: 55                          	pushq	%rbp
1414515a6: 56                          	pushq	%rsi
1414515a7: 57                          	pushq	%rdi
1414515a8: 48 83 ec 20                 	subq	$0x20, %rsp
1414515ac: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414515b3: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
1414515ba: e8 c1 82 bd fe              	callq	0x140029880 <.text+0x28880>
1414515bf: 90                          	nop
1414515c0: 48 83 c4 20                 	addq	$0x20, %rsp
1414515c4: 5f                          	popq	%rdi
1414515c5: 5e                          	popq	%rsi
1414515c6: 5d                          	popq	%rbp
1414515c7: c3                          	retq
1414515c8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1414515d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1414515d5: 55                          	pushq	%rbp
1414515d6: 56                          	pushq	%rsi
1414515d7: 57                          	pushq	%rdi
1414515d8: 48 83 ec 20                 	subq	$0x20, %rsp
1414515dc: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1414515e3: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1414515ea: e8 91 82 bd fe              	callq	0x140029880 <.text+0x28880>
1414515ef: 90                          	nop
1414515f0: 48 83 c4 20                 	addq	$0x20, %rsp
1414515f4: 5f                          	popq	%rdi
1414515f5: 5e                          	popq	%rsi
1414515f6: 5d                          	popq	%rbp
1414515f7: c3                          	retq
1414515f8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
141451600: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141451605: 55                          	pushq	%rbp
141451606: 56                          	pushq	%rsi
141451607: 57                          	pushq	%rdi
141451608: 48 83 ec 20                 	subq	$0x20, %rsp
14145160c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141451613: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14145161a: e8 61 82 bd fe              	callq	0x140029880 <.text+0x28880>
14145161f: 90                          	nop
141451620: 48 83 c4 20                 	addq	$0x20, %rsp
141451624: 5f                          	popq	%rdi
141451625: 5e                          	popq	%rsi
141451626: 5d                          	popq	%rbp
141451627: c3                          	retq
