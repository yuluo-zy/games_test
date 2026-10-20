
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fed8a0: 55                          	pushq	%rbp
140fed8a1: 56                          	pushq	%rsi
140fed8a2: 57                          	pushq	%rdi
140fed8a3: 53                          	pushq	%rbx
140fed8a4: 48 81 ec 48 05 00 00        	subq	$0x548, %rsp            # imm = 0x548
140fed8ab: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140fed8b3: 48 c7 85 c0 04 00 00 fe ff ff ff    	movq	$-0x2, 0x4c0(%rbp)
140fed8be: 48 89 ce                    	movq	%rcx, %rsi
140fed8c1: 0f b6 05 41 73 64 02        	movzbl	0x2647341(%rip), %eax   # 0x143634c09
140fed8c8: b9 a0 02 00 00              	movl	$0x2a0, %ecx            # imm = 0x2A0
140fed8cd: ba 08 00 00 00              	movl	$0x8, %edx
140fed8d2: e8 39 63 62 ff              	callq	0x140613c10 <.text+0x612c10>
140fed8d7: 48 85 c0                    	testq	%rax, %rax
140fed8da: 0f 84 0c 06 00 00           	je	0x140fedeec <.text+0xfeceec>
140fed8e0: 48 89 85 b8 04 00 00        	movq	%rax, 0x4b8(%rbp)
140fed8e7: 0f 57 c0                    	xorps	%xmm0, %xmm0
140fed8ea: 0f 11 85 68 02 00 00        	movups	%xmm0, 0x268(%rbp)
140fed8f1: 0f 11 85 80 02 00 00        	movups	%xmm0, 0x280(%rbp)
140fed8f8: 0f 11 85 98 02 00 00        	movups	%xmm0, 0x298(%rbp)
140fed8ff: 0f 11 85 b0 02 00 00        	movups	%xmm0, 0x2b0(%rbp)
140fed906: 0f 11 85 c8 02 00 00        	movups	%xmm0, 0x2c8(%rbp)
140fed90d: c7 85 d8 02 00 00 00 00 00 00       	movl	$0x0, 0x2d8(%rbp)
140fed917: 0f 11 85 00 03 00 00        	movups	%xmm0, 0x300(%rbp)
140fed91e: 0f 11 85 18 03 00 00        	movups	%xmm0, 0x318(%rbp)
140fed925: 0f 11 85 30 03 00 00        	movups	%xmm0, 0x330(%rbp)
140fed92c: 0f 11 85 48 03 00 00        	movups	%xmm0, 0x348(%rbp)
140fed933: 0f 11 85 60 03 00 00        	movups	%xmm0, 0x360(%rbp)
140fed93a: c7 85 70 03 00 00 00 00 00 00       	movl	$0x0, 0x370(%rbp)
140fed944: 48 c7 85 48 02 00 00 00 00 00 00    	movq	$0x0, 0x248(%rbp)
140fed94f: 48 c7 85 50 02 00 00 08 00 00 00    	movq	$0x8, 0x250(%rbp)
140fed95a: 48 c7 85 58 02 00 00 00 00 00 00    	movq	$0x0, 0x258(%rbp)
140fed965: 48 c7 85 60 02 00 00 10 00 00 00    	movq	$0x10, 0x260(%rbp)
140fed970: 48 c7 85 78 02 00 00 10 00 00 00    	movq	$0x10, 0x278(%rbp)
140fed97b: 48 c7 85 90 02 00 00 10 00 00 00    	movq	$0x10, 0x290(%rbp)
140fed986: 48 c7 85 a8 02 00 00 10 00 00 00    	movq	$0x10, 0x2a8(%rbp)
140fed991: 48 c7 85 c0 02 00 00 10 00 00 00    	movq	$0x10, 0x2c0(%rbp)
140fed99c: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140fed9a6: 48 89 9d e0 02 00 00        	movq	%rbx, 0x2e0(%rbp)
140fed9ad: 48 8d 05 db 59 bd 01        	leaq	0x1bd59db(%rip), %rax   # 0x142bc338f
140fed9b4: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140fed9bb: 48 c7 85 f0 02 00 00 25 00 00 00    	movq	$0x25, 0x2f0(%rbp)
140fed9c6: 48 c7 85 f8 02 00 00 10 00 00 00    	movq	$0x10, 0x2f8(%rbp)
140fed9d1: 48 c7 85 10 03 00 00 10 00 00 00    	movq	$0x10, 0x310(%rbp)
140fed9dc: 48 c7 85 28 03 00 00 10 00 00 00    	movq	$0x10, 0x328(%rbp)
140fed9e7: 48 c7 85 40 03 00 00 10 00 00 00    	movq	$0x10, 0x340(%rbp)
140fed9f2: 48 c7 85 58 03 00 00 10 00 00 00    	movq	$0x10, 0x358(%rbp)
140fed9fd: c7 85 78 03 00 00 00 00 00 00       	movl	$0x0, 0x378(%rbp)
140feda07: 66 c7 85 7c 03 00 00 01 00  	movw	$0x1, 0x37c(%rbp)
140feda10: 48 89 9d 80 04 00 00        	movq	%rbx, 0x480(%rbp)
140feda17: c7 85 a8 04 00 00 00 00 00 00       	movl	$0x0, 0x4a8(%rbp)
140feda21: 0f b6 05 e1 71 64 02        	movzbl	0x26471e1(%rip), %eax   # 0x143634c09
140feda28: b9 68 02 00 00              	movl	$0x268, %ecx            # imm = 0x268
140feda2d: ba 08 00 00 00              	movl	$0x8, %edx
140feda32: e8 d9 61 62 ff              	callq	0x140613c10 <.text+0x612c10>
140feda37: 48 85 c0                    	testq	%rax, %rax
140feda3a: 0f 84 bb 04 00 00           	je	0x140fedefb <.text+0xfecefb>
140feda40: 48 89 c7                    	movq	%rax, %rdi
140feda43: 48 8d 95 48 02 00 00        	leaq	0x248(%rbp), %rdx
140feda4a: 41 b8 68 02 00 00           	movl	$0x268, %r8d            # imm = 0x268
140feda50: 48 89 c1                    	movq	%rax, %rcx
140feda53: e8 a8 61 93 01              	callq	0x142923c00 <.text+0x2922c00>
140feda58: 4c 8d 05 09 58 bd 01        	leaq	0x1bd5809(%rip), %r8    # 0x142bc3268
140feda5f: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
140feda66: 48 89 fa                    	movq	%rdi, %rdx
140feda69: e8 a2 70 cf ff              	callq	0x140ce4b10 <.text+0xce3b10>
140feda6e: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
140feda75: e8 16 1e eb 00              	callq	0x141e9f890 <.text+0x1e9e890>
140feda7a: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140feda81: e8 ba 1f eb 00              	callq	0x141e9fa40 <.text+0x1e9ea40>
140feda86: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140feda8a: e8 51 1c eb 00              	callq	0x141e9f6e0 <.text+0x1e9e6e0>
140feda8f: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140feda93: e8 38 a9 eb 00              	callq	0x141ea83d0 <.text+0x1ea73d0>
140feda98: 0f 57 c0                    	xorps	%xmm0, %xmm0
140feda9b: 0f 11 85 68 02 00 00        	movups	%xmm0, 0x268(%rbp)
140fedaa2: 0f 11 85 80 02 00 00        	movups	%xmm0, 0x280(%rbp)
140fedaa9: 0f 11 85 98 02 00 00        	movups	%xmm0, 0x298(%rbp)
140fedab0: 0f 11 85 b0 02 00 00        	movups	%xmm0, 0x2b0(%rbp)
140fedab7: 0f 11 85 c8 02 00 00        	movups	%xmm0, 0x2c8(%rbp)
140fedabe: c7 85 d8 02 00 00 00 00 00 00       	movl	$0x0, 0x2d8(%rbp)
140fedac8: 0f 11 85 00 03 00 00        	movups	%xmm0, 0x300(%rbp)
140fedacf: 0f 11 85 18 03 00 00        	movups	%xmm0, 0x318(%rbp)
140fedad6: 0f 11 85 30 03 00 00        	movups	%xmm0, 0x330(%rbp)
140fedadd: 0f 11 85 48 03 00 00        	movups	%xmm0, 0x348(%rbp)
140fedae4: 0f 11 85 60 03 00 00        	movups	%xmm0, 0x360(%rbp)
140fedaeb: c7 85 70 03 00 00 00 00 00 00       	movl	$0x0, 0x370(%rbp)
140fedaf5: 48 c7 85 48 02 00 00 00 00 00 00    	movq	$0x0, 0x248(%rbp)
140fedb00: 48 c7 85 50 02 00 00 08 00 00 00    	movq	$0x8, 0x250(%rbp)
140fedb0b: 48 c7 85 58 02 00 00 00 00 00 00    	movq	$0x0, 0x258(%rbp)
140fedb16: 48 c7 85 60 02 00 00 10 00 00 00    	movq	$0x10, 0x260(%rbp)
140fedb21: 48 c7 85 78 02 00 00 10 00 00 00    	movq	$0x10, 0x278(%rbp)
140fedb2c: 48 c7 85 90 02 00 00 10 00 00 00    	movq	$0x10, 0x290(%rbp)
140fedb37: 48 c7 85 a8 02 00 00 10 00 00 00    	movq	$0x10, 0x2a8(%rbp)
140fedb42: 48 c7 85 c0 02 00 00 10 00 00 00    	movq	$0x10, 0x2c0(%rbp)
140fedb4d: 48 89 9d e0 02 00 00        	movq	%rbx, 0x2e0(%rbp)
140fedb54: 48 8d 05 e7 57 bd 01        	leaq	0x1bd57e7(%rip), %rax   # 0x142bc3342
140fedb5b: 48 89 85 e8 02 00 00        	movq	%rax, 0x2e8(%rbp)
140fedb62: 48 c7 85 f0 02 00 00 24 00 00 00    	movq	$0x24, 0x2f0(%rbp)
140fedb6d: 48 c7 85 f8 02 00 00 10 00 00 00    	movq	$0x10, 0x2f8(%rbp)
140fedb78: 48 c7 85 10 03 00 00 10 00 00 00    	movq	$0x10, 0x310(%rbp)
140fedb83: 48 c7 85 28 03 00 00 10 00 00 00    	movq	$0x10, 0x328(%rbp)
140fedb8e: 48 c7 85 40 03 00 00 10 00 00 00    	movq	$0x10, 0x340(%rbp)
140fedb99: 48 c7 85 58 03 00 00 10 00 00 00    	movq	$0x10, 0x358(%rbp)
140fedba4: c7 85 78 03 00 00 00 00 00 00       	movl	$0x0, 0x378(%rbp)
140fedbae: 66 c7 85 7c 03 00 00 01 00  	movw	$0x1, 0x37c(%rbp)
140fedbb7: 48 89 9d 80 04 00 00        	movq	%rbx, 0x480(%rbp)
140fedbbe: c7 85 b0 04 00 00 00 00 00 00       	movl	$0x0, 0x4b0(%rbp)
140fedbc8: 0f b6 05 3a 70 64 02        	movzbl	0x264703a(%rip), %eax   # 0x143634c09
140fedbcf: b9 70 02 00 00              	movl	$0x270, %ecx            # imm = 0x270
140fedbd4: ba 08 00 00 00              	movl	$0x8, %edx
140fedbd9: e8 32 60 62 ff              	callq	0x140613c10 <.text+0x612c10>
140fedbde: 48 85 c0                    	testq	%rax, %rax
140fedbe1: 0f 84 25 03 00 00           	je	0x140fedf0c <.text+0xfecf0c>
140fedbe7: 48 89 c7                    	movq	%rax, %rdi
140fedbea: 48 8d 95 48 02 00 00        	leaq	0x248(%rbp), %rdx
140fedbf1: 41 b8 70 02 00 00           	movl	$0x270, %r8d            # imm = 0x270
140fedbf7: 48 89 c1                    	movq	%rax, %rcx
140fedbfa: e8 01 60 93 01              	callq	0x142923c00 <.text+0x2922c00>
140fedbff: 4c 8d 05 aa 55 bd 01        	leaq	0x1bd55aa(%rip), %r8    # 0x142bc31b0
140fedc06: 48 8d 8d d8 01 00 00        	leaq	0x1d8(%rbp), %rcx
140fedc0d: 48 89 fa                    	movq	%rdi, %rdx
140fedc10: e8 fb 6e cf ff              	callq	0x140ce4b10 <.text+0xce3b10>
140fedc15: 0f 10 85 c8 01 00 00        	movups	0x1c8(%rbp), %xmm0
140fedc1c: 48 8b 85 b8 04 00 00        	movq	0x4b8(%rbp), %rax
140fedc23: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140fedc27: 0f 10 85 b8 01 00 00        	movups	0x1b8(%rbp), %xmm0
140fedc2e: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140fedc32: 0f 10 85 a8 01 00 00        	movups	0x1a8(%rbp), %xmm0
140fedc39: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140fedc3d: 0f 10 85 68 01 00 00        	movups	0x168(%rbp), %xmm0
140fedc44: 0f 10 8d 78 01 00 00        	movups	0x178(%rbp), %xmm1
140fedc4b: 0f 10 95 88 01 00 00        	movups	0x188(%rbp), %xmm2
140fedc52: 0f 10 9d 98 01 00 00        	movups	0x198(%rbp), %xmm3
140fedc59: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140fedc5d: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140fedc61: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140fedc65: 0f 11 00                    	movups	%xmm0, (%rax)
140fedc68: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
140fedc6f: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
140fedc76: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
140fedc7d: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
140fedc84: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140fedc88: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140fedc8f: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140fedc96: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140fedc9d: 0f 10 85 38 01 00 00        	movups	0x138(%rbp), %xmm0
140fedca4: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140fedcab: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
140fedcb2: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140fedcb9: 0f 10 85 58 01 00 00        	movups	0x158(%rbp), %xmm0
140fedcc0: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140fedcc7: 0f 10 85 e8 00 00 00        	movups	0xe8(%rbp), %xmm0
140fedcce: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140fedcd5: 0f 10 85 d8 00 00 00        	movups	0xd8(%rbp), %xmm0
140fedcdc: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140fedce3: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
140fedcea: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140fedcf1: 0f 10 85 88 00 00 00        	movups	0x88(%rbp), %xmm0
140fedcf8: 0f 10 8d 98 00 00 00        	movups	0x98(%rbp), %xmm1
140fedcff: 0f 10 95 a8 00 00 00        	movups	0xa8(%rbp), %xmm2
140fedd06: 0f 10 9d b8 00 00 00        	movups	0xb8(%rbp), %xmm3
140fedd0d: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140fedd14: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140fedd1b: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140fedd22: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140fedd29: 0f 10 45 18                 	movups	0x18(%rbp), %xmm0
140fedd2d: 0f 10 4d 28                 	movups	0x28(%rbp), %xmm1
140fedd31: 0f 10 55 38                 	movups	0x38(%rbp), %xmm2
140fedd35: 0f 10 5d 48                 	movups	0x48(%rbp), %xmm3
140fedd39: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140fedd40: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140fedd47: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140fedd4e: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140fedd55: 0f 10 45 58                 	movups	0x58(%rbp), %xmm0
140fedd59: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140fedd60: 0f 10 45 68                 	movups	0x68(%rbp), %xmm0
140fedd64: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140fedd6b: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
140fedd6f: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140fedd76: 0f 10 45 a8                 	movups	-0x58(%rbp), %xmm0
140fedd7a: 0f 10 4d b8                 	movups	-0x48(%rbp), %xmm1
140fedd7e: 0f 10 55 c8                 	movups	-0x38(%rbp), %xmm2
140fedd82: 0f 10 5d d8                 	movups	-0x28(%rbp), %xmm3
140fedd86: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
140fedd8d: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
140fedd94: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
140fedd9b: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
140fedda2: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140fedda6: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
140feddad: 0f 10 45 08                 	movups	0x8(%rbp), %xmm0
140feddb1: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
140feddb8: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
140feddbc: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
140feddc3: 0f 10 85 38 02 00 00        	movups	0x238(%rbp), %xmm0
140feddca: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
140feddd1: 0f 10 85 28 02 00 00        	movups	0x228(%rbp), %xmm0
140feddd8: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
140fedddf: 0f 10 85 18 02 00 00        	movups	0x218(%rbp), %xmm0
140fedde6: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
140fedded: 0f 10 85 d8 01 00 00        	movups	0x1d8(%rbp), %xmm0
140feddf4: 0f 10 8d e8 01 00 00        	movups	0x1e8(%rbp), %xmm1
140feddfb: 0f 10 95 f8 01 00 00        	movups	0x1f8(%rbp), %xmm2
140fede02: 0f 10 9d 08 02 00 00        	movups	0x208(%rbp), %xmm3
140fede09: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
140fede10: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
140fede17: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
140fede1e: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
140fede25: 48 c7 85 50 02 00 00 06 00 00 00    	movq	$0x6, 0x250(%rbp)
140fede30: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
140fede37: 48 c7 85 60 02 00 00 06 00 00 00    	movq	$0x6, 0x260(%rbp)
140fede42: 48 c7 85 68 02 00 00 00 00 00 00    	movq	$0x0, 0x268(%rbp)
140fede4d: 48 c7 85 70 02 00 00 08 00 00 00    	movq	$0x8, 0x270(%rbp)
140fede58: 0f 57 c0                    	xorps	%xmm0, %xmm0
140fede5b: 0f 11 85 78 02 00 00        	movups	%xmm0, 0x278(%rbp)
140fede62: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
140fede69: 48 89 85 98 02 00 00        	movq	%rax, 0x298(%rbp)
140fede70: 0f 10 85 d8 01 00 00        	movups	0x1d8(%rbp), %xmm0
140fede77: 0f 11 85 88 02 00 00        	movups	%xmm0, 0x288(%rbp)
140fede7e: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
140fede85: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
140fede8c: e8 8f 68 fe ff              	callq	0x140fd4720 <.text+0xfd3720>
140fede91: 0f 10 85 a8 02 00 00        	movups	0x2a8(%rbp), %xmm0
140fede98: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140fede9c: 0f 10 85 98 02 00 00        	movups	0x298(%rbp), %xmm0
140fedea3: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140fedea7: 0f 10 85 88 02 00 00        	movups	0x288(%rbp), %xmm0
140fedeae: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140fedeb2: 0f 10 85 48 02 00 00        	movups	0x248(%rbp), %xmm0
140fedeb9: 0f 10 8d 58 02 00 00        	movups	0x258(%rbp), %xmm1
140fedec0: 0f 10 95 68 02 00 00        	movups	0x268(%rbp), %xmm2
140fedec7: 0f 10 9d 78 02 00 00        	movups	0x278(%rbp), %xmm3
140fedece: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140feded2: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140feded6: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140fededa: 0f 11 06                    	movups	%xmm0, (%rsi)
140fededd: 48 89 f0                    	movq	%rsi, %rax
140fedee0: 48 81 c4 48 05 00 00        	addq	$0x548, %rsp            # imm = 0x548
140fedee7: 5b                          	popq	%rbx
140fedee8: 5f                          	popq	%rdi
140fedee9: 5e                          	popq	%rsi
140fedeea: 5d                          	popq	%rbp
140fedeeb: c3                          	retq
140fedeec: b9 08 00 00 00              	movl	$0x8, %ecx
140fedef1: ba a0 02 00 00              	movl	$0x2a0, %edx            # imm = 0x2A0
140fedef6: e8 05 b1 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140fedefb: b9 08 00 00 00              	movl	$0x8, %ecx
140fedf00: ba 68 02 00 00              	movl	$0x268, %edx            # imm = 0x268
140fedf05: e8 f6 b0 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140fedf0a: eb 0f                       	jmp	0x140fedf1b <.text+0xfecf1b>
140fedf0c: b9 08 00 00 00              	movl	$0x8, %ecx
140fedf11: ba 70 02 00 00              	movl	$0x270, %edx            # imm = 0x270
140fedf16: e8 e5 b0 8e 01              	callq	0x1428d9000 <.text+0x28d8000>
140fedf1b: 0f 0b                       	ud2
140fedf1d: 0f 1f 00                    	nopl	(%rax)
140fedf20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fedf25: 55                          	pushq	%rbp
140fedf26: 56                          	pushq	%rsi
140fedf27: 57                          	pushq	%rdi
140fedf28: 53                          	pushq	%rbx
140fedf29: 48 83 ec 28                 	subq	$0x28, %rsp
140fedf2d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fedf34: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140fedf3b: e8 90 8a 89 ff              	callq	0x1408869d0 <.text+0x8859d0>
140fedf40: 90                          	nop
140fedf41: 48 83 c4 28                 	addq	$0x28, %rsp
140fedf45: 5b                          	popq	%rbx
140fedf46: 5f                          	popq	%rdi
140fedf47: 5e                          	popq	%rsi
140fedf48: 5d                          	popq	%rbp
140fedf49: c3                          	retq
140fedf4a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fedf50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fedf55: 55                          	pushq	%rbp
140fedf56: 56                          	pushq	%rsi
140fedf57: 57                          	pushq	%rdi
140fedf58: 53                          	pushq	%rbx
140fedf59: 48 83 ec 28                 	subq	$0x28, %rsp
140fedf5d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fedf64: ba a0 02 00 00              	movl	$0x2a0, %edx            # imm = 0x2A0
140fedf69: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140fedf6f: 48 8b 8d b8 04 00 00        	movq	0x4b8(%rbp), %rcx
140fedf76: e8 a5 5c 62 ff              	callq	0x140613c20 <.text+0x612c20>
140fedf7b: 90                          	nop
140fedf7c: 48 83 c4 28                 	addq	$0x28, %rsp
140fedf80: 5b                          	popq	%rbx
140fedf81: 5f                          	popq	%rdi
140fedf82: 5e                          	popq	%rsi
140fedf83: 5d                          	popq	%rbp
140fedf84: c3                          	retq
140fedf85: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140fedf90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fedf95: 55                          	pushq	%rbp
140fedf96: 56                          	pushq	%rsi
140fedf97: 57                          	pushq	%rdi
140fedf98: 53                          	pushq	%rbx
140fedf99: 48 83 ec 28                 	subq	$0x28, %rsp
140fedf9d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fedfa4: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140fedfab: e8 20 8a 89 ff              	callq	0x1408869d0 <.text+0x8859d0>
140fedfb0: 90                          	nop
140fedfb1: 48 83 c4 28                 	addq	$0x28, %rsp
140fedfb5: 5b                          	popq	%rbx
140fedfb6: 5f                          	popq	%rdi
140fedfb7: 5e                          	popq	%rsi
140fedfb8: 5d                          	popq	%rbp
140fedfb9: c3                          	retq
140fedfba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fedfc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fedfc5: 55                          	pushq	%rbp
140fedfc6: 56                          	pushq	%rsi
140fedfc7: 57                          	pushq	%rdi
140fedfc8: 53                          	pushq	%rbx
140fedfc9: 48 83 ec 28                 	subq	$0x28, %rsp
140fedfcd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fedfd4: 48 8d 4d a8                 	leaq	-0x58(%rbp), %rcx
140fedfd8: e8 a3 b8 03 ff              	callq	0x140029880 <.text+0x28880>
140fedfdd: 90                          	nop
140fedfde: 48 83 c4 28                 	addq	$0x28, %rsp
140fedfe2: 5b                          	popq	%rbx
140fedfe3: 5f                          	popq	%rdi
140fedfe4: 5e                          	popq	%rsi
140fedfe5: 5d                          	popq	%rbp
140fedfe6: c3                          	retq
140fedfe7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140fedff0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fedff5: 55                          	pushq	%rbp
140fedff6: 56                          	pushq	%rsi
140fedff7: 57                          	pushq	%rdi
140fedff8: 53                          	pushq	%rbx
140fedff9: 48 83 ec 28                 	subq	$0x28, %rsp
140fedffd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee004: 48 8d 8d 68 01 00 00        	leaq	0x168(%rbp), %rcx
140fee00b: e8 70 b8 03 ff              	callq	0x140029880 <.text+0x28880>
140fee010: 90                          	nop
140fee011: 48 83 c4 28                 	addq	$0x28, %rsp
140fee015: 5b                          	popq	%rbx
140fee016: 5f                          	popq	%rdi
140fee017: 5e                          	popq	%rsi
140fee018: 5d                          	popq	%rbp
140fee019: c3                          	retq
140fee01a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee020: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee025: 55                          	pushq	%rbp
140fee026: 56                          	pushq	%rsi
140fee027: 57                          	pushq	%rdi
140fee028: 53                          	pushq	%rbx
140fee029: 48 83 ec 28                 	subq	$0x28, %rsp
140fee02d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee034: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
140fee03b: e8 40 b8 03 ff              	callq	0x140029880 <.text+0x28880>
140fee040: 90                          	nop
140fee041: 48 83 c4 28                 	addq	$0x28, %rsp
140fee045: 5b                          	popq	%rbx
140fee046: 5f                          	popq	%rdi
140fee047: 5e                          	popq	%rsi
140fee048: 5d                          	popq	%rbp
140fee049: c3                          	retq
140fee04a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee050: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee055: 55                          	pushq	%rbp
140fee056: 56                          	pushq	%rsi
140fee057: 57                          	pushq	%rdi
140fee058: 53                          	pushq	%rbx
140fee059: 48 83 ec 28                 	subq	$0x28, %rsp
140fee05d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee064: 48 8d 8d 88 00 00 00        	leaq	0x88(%rbp), %rcx
140fee06b: e8 10 b8 03 ff              	callq	0x140029880 <.text+0x28880>
140fee070: 90                          	nop
140fee071: 48 83 c4 28                 	addq	$0x28, %rsp
140fee075: 5b                          	popq	%rbx
140fee076: 5f                          	popq	%rdi
140fee077: 5e                          	popq	%rsi
140fee078: 5d                          	popq	%rbp
140fee079: c3                          	retq
140fee07a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140fee080: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee085: 55                          	pushq	%rbp
140fee086: 56                          	pushq	%rsi
140fee087: 57                          	pushq	%rdi
140fee088: 53                          	pushq	%rbx
140fee089: 48 83 ec 28                 	subq	$0x28, %rsp
140fee08d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee094: 48 8d 4d 18                 	leaq	0x18(%rbp), %rcx
140fee098: e8 e3 b7 03 ff              	callq	0x140029880 <.text+0x28880>
140fee09d: 90                          	nop
140fee09e: 48 83 c4 28                 	addq	$0x28, %rsp
140fee0a2: 5b                          	popq	%rbx
140fee0a3: 5f                          	popq	%rdi
140fee0a4: 5e                          	popq	%rsi
140fee0a5: 5d                          	popq	%rbp
140fee0a6: c3                          	retq
140fee0a7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140fee0b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140fee0b5: 55                          	pushq	%rbp
140fee0b6: 56                          	pushq	%rsi
140fee0b7: 57                          	pushq	%rdi
140fee0b8: 53                          	pushq	%rbx
140fee0b9: 48 83 ec 28                 	subq	$0x28, %rsp
140fee0bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140fee0c4: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140fee0cb: e8 b0 b7 03 ff              	callq	0x140029880 <.text+0x28880>
140fee0d0: 90                          	nop
140fee0d1: 48 83 c4 28                 	addq	$0x28, %rsp
140fee0d5: 5b                          	popq	%rbx
140fee0d6: 5f                          	popq	%rdi
140fee0d7: 5e                          	popq	%rsi
140fee0d8: 5d                          	popq	%rbp
140fee0d9: c3                          	retq
140fee0da: cc                          	int3
140fee0db: cc                          	int3
140fee0dc: cc                          	int3
140fee0dd: cc                          	int3
140fee0de: cc                          	int3
140fee0df: cc                          	int3
