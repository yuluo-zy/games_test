
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f801d0: 55                          	pushq	%rbp
140f801d1: 56                          	pushq	%rsi
140f801d2: 57                          	pushq	%rdi
140f801d3: 53                          	pushq	%rbx
140f801d4: 48 81 ec f8 01 00 00        	subq	$0x1f8, %rsp            # imm = 0x1F8
140f801db: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f801e3: 48 c7 85 70 01 00 00 fe ff ff ff    	movq	$-0x2, 0x170(%rbp)
140f801ee: 44 89 c3                    	movl	%r8d, %ebx
140f801f1: 48 89 d7                    	movq	%rdx, %rdi
140f801f4: 48 89 ce                    	movq	%rcx, %rsi
140f801f7: 0f 10 42 60                 	movups	0x60(%rdx), %xmm0
140f801fb: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140f801ff: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
140f80203: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140f80207: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
140f8020b: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
140f8020f: 0f 10 02                    	movups	(%rdx), %xmm0
140f80212: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140f80216: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
140f8021a: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
140f8021e: 0f 29 5d d0                 	movaps	%xmm3, -0x30(%rbp)
140f80222: 0f 29 55 c0                 	movaps	%xmm2, -0x40(%rbp)
140f80226: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140f8022a: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
140f8022e: 0f b6 05 d4 49 6b 02        	movzbl	0x26b49d4(%rip), %eax   # 0x143634c09
140f80235: b9 50 01 00 00              	movl	$0x150, %ecx            # imm = 0x150
140f8023a: ba 08 00 00 00              	movl	$0x8, %edx
140f8023f: e8 cc 39 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f80244: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
140f8024b: 48 85 c0                    	testq	%rax, %rax
140f8024e: 0f 84 35 02 00 00           	je	0x140f80489 <.text+0xf7f489>
140f80254: 0f 10 47 60                 	movups	0x60(%rdi), %xmm0
140f80258: 0f 29 85 e0 00 00 00        	movaps	%xmm0, 0xe0(%rbp)
140f8025f: 0f 10 47 50                 	movups	0x50(%rdi), %xmm0
140f80263: 0f 29 85 d0 00 00 00        	movaps	%xmm0, 0xd0(%rbp)
140f8026a: 0f 10 47 40                 	movups	0x40(%rdi), %xmm0
140f8026e: 0f 29 85 c0 00 00 00        	movaps	%xmm0, 0xc0(%rbp)
140f80275: 0f 10 07                    	movups	(%rdi), %xmm0
140f80278: 0f 10 4f 10                 	movups	0x10(%rdi), %xmm1
140f8027c: 0f 10 57 20                 	movups	0x20(%rdi), %xmm2
140f80280: 0f 10 5f 30                 	movups	0x30(%rdi), %xmm3
140f80284: 0f 29 9d b0 00 00 00        	movaps	%xmm3, 0xb0(%rbp)
140f8028b: 0f 29 95 a0 00 00 00        	movaps	%xmm2, 0xa0(%rbp)
140f80292: 0f 29 8d 90 00 00 00        	movaps	%xmm1, 0x90(%rbp)
140f80299: 0f 29 85 80 00 00 00        	movaps	%xmm0, 0x80(%rbp)
140f802a0: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140f802a4: e8 67 2a e3 00              	callq	0x141db2d10 <.text+0x1db1d10>
140f802a9: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
140f802b0: e8 6b 56 e3 00              	callq	0x141db5920 <.text+0x1db4920>
140f802b5: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
140f802bc: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140f802c3: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140f802c7: 0f 28 85 d0 00 00 00        	movaps	0xd0(%rbp), %xmm0
140f802ce: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140f802d2: 0f 28 85 c0 00 00 00        	movaps	0xc0(%rbp), %xmm0
140f802d9: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140f802dd: 0f 28 85 80 00 00 00        	movaps	0x80(%rbp), %xmm0
140f802e4: 0f 28 8d 90 00 00 00        	movaps	0x90(%rbp), %xmm1
140f802eb: 0f 28 95 a0 00 00 00        	movaps	0xa0(%rbp), %xmm2
140f802f2: 0f 28 9d b0 00 00 00        	movaps	0xb0(%rbp), %xmm3
140f802f9: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140f802fd: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140f80301: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140f80305: 0f 11 00                    	movups	%xmm0, (%rax)
140f80308: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140f8030c: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140f80310: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140f80314: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140f80318: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140f8031c: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140f80323: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140f8032a: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140f80331: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140f80335: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140f8033c: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140f80340: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140f80347: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140f8034b: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140f80352: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
140f80359: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
140f80360: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
140f80367: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
140f8036e: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140f80375: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140f8037c: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140f80383: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140f8038a: 0f 10 85 38 01 00 00        	movups	0x138(%rbp), %xmm0
140f80391: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140f80398: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
140f8039f: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140f803a6: 0f 10 85 58 01 00 00        	movups	0x158(%rbp), %xmm0
140f803ad: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140f803b4: 48 c7 85 00 01 00 00 03 00 00 00    	movq	$0x3, 0x100(%rbp)
140f803bf: 48 89 85 08 01 00 00        	movq	%rax, 0x108(%rbp)
140f803c6: 48 c7 85 10 01 00 00 03 00 00 00    	movq	$0x3, 0x110(%rbp)
140f803d1: 48 c7 85 18 01 00 00 00 00 00 00    	movq	$0x0, 0x118(%rbp)
140f803dc: 48 c7 85 20 01 00 00 08 00 00 00    	movq	$0x8, 0x120(%rbp)
140f803e7: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f803ea: 0f 11 85 28 01 00 00        	movups	%xmm0, 0x128(%rbp)
140f803f1: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140f803fb: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140f80402: 88 5d 10                    	movb	%bl, 0x10(%rbp)
140f80405: 48 8b 0d 64 bf 6a 02        	movq	0x26abf64(%rip), %rcx   # 0x14362c370
140f8040c: 4c 8d 05 2d b5 fe 01        	leaq	0x1feb52d(%rip), %r8    # 0x142f6b940
140f80413: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140f80417: e8 d4 22 e1 00              	callq	0x141d926f0 <.text+0x1d916f0>
140f8041c: 49 89 d0                    	movq	%rdx, %r8
140f8041f: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
140f80426: 48 89 c2                    	movq	%rax, %rdx
140f80429: e8 62 01 00 00              	callq	0x140f80590 <.text+0xf7f590>
140f8042e: 0f 10 85 58 01 00 00        	movups	0x158(%rbp), %xmm0
140f80435: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140f80439: 0f 10 85 48 01 00 00        	movups	0x148(%rbp), %xmm0
140f80440: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140f80444: 0f 10 85 38 01 00 00        	movups	0x138(%rbp), %xmm0
140f8044b: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140f8044f: 0f 10 85 f8 00 00 00        	movups	0xf8(%rbp), %xmm0
140f80456: 0f 10 8d 08 01 00 00        	movups	0x108(%rbp), %xmm1
140f8045d: 0f 10 95 18 01 00 00        	movups	0x118(%rbp), %xmm2
140f80464: 0f 10 9d 28 01 00 00        	movups	0x128(%rbp), %xmm3
140f8046b: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140f8046f: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140f80473: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140f80477: 0f 11 06                    	movups	%xmm0, (%rsi)
140f8047a: 48 89 f0                    	movq	%rsi, %rax
140f8047d: 48 81 c4 f8 01 00 00        	addq	$0x1f8, %rsp            # imm = 0x1F8
140f80484: 5b                          	popq	%rbx
140f80485: 5f                          	popq	%rdi
140f80486: 5e                          	popq	%rsi
140f80487: 5d                          	popq	%rbp
140f80488: c3                          	retq
140f80489: b9 08 00 00 00              	movl	$0x8, %ecx
140f8048e: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
140f80493: e8 68 8b 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f80498: 0f 0b                       	ud2
140f8049a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f804a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f804a5: 55                          	pushq	%rbp
140f804a6: 56                          	pushq	%rsi
140f804a7: 57                          	pushq	%rdi
140f804a8: 53                          	pushq	%rbx
140f804a9: 48 83 ec 28                 	subq	$0x28, %rsp
140f804ad: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f804b4: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140f804bb: e8 c0 93 0a ff              	callq	0x140029880 <.text+0x28880>
140f804c0: ba 50 01 00 00              	movl	$0x150, %edx            # imm = 0x150
140f804c5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f804cb: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140f804d2: e8 49 37 69 ff              	callq	0x140613c20 <.text+0x612c20>
140f804d7: 90                          	nop
140f804d8: 48 83 c4 28                 	addq	$0x28, %rsp
140f804dc: 5b                          	popq	%rbx
140f804dd: 5f                          	popq	%rdi
140f804de: 5e                          	popq	%rsi
140f804df: 5d                          	popq	%rbp
140f804e0: c3                          	retq
140f804e1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140f804f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f804f5: 55                          	pushq	%rbp
140f804f6: 56                          	pushq	%rsi
140f804f7: 57                          	pushq	%rdi
140f804f8: 53                          	pushq	%rbx
140f804f9: 48 83 ec 28                 	subq	$0x28, %rsp
140f804fd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f80504: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140f80508: e8 73 93 0a ff              	callq	0x140029880 <.text+0x28880>
140f8050d: 90                          	nop
140f8050e: 48 83 c4 28                 	addq	$0x28, %rsp
140f80512: 5b                          	popq	%rbx
140f80513: 5f                          	popq	%rdi
140f80514: 5e                          	popq	%rsi
140f80515: 5d                          	popq	%rbp
140f80516: c3                          	retq
140f80517: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140f80520: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f80525: 55                          	pushq	%rbp
140f80526: 56                          	pushq	%rsi
140f80527: 57                          	pushq	%rdi
140f80528: 53                          	pushq	%rbx
140f80529: 48 83 ec 28                 	subq	$0x28, %rsp
140f8052d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f80534: 48 83 bd 68 01 00 00 00     	cmpq	$0x0, 0x168(%rbp)
140f8053c: 74 09                       	je	0x140f80547 <.text+0xf7f547>
140f8053e: 48 83 c4 28                 	addq	$0x28, %rsp
140f80542: 5b                          	popq	%rbx
140f80543: 5f                          	popq	%rdi
140f80544: 5e                          	popq	%rsi
140f80545: 5d                          	popq	%rbp
140f80546: c3                          	retq
140f80547: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f8054b: e8 30 93 0a ff              	callq	0x140029880 <.text+0x28880>
140f80550: eb ec                       	jmp	0x140f8053e <.text+0xf7f53e>
140f80552: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140f80560: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f80565: 55                          	pushq	%rbp
140f80566: 56                          	pushq	%rsi
140f80567: 57                          	pushq	%rdi
140f80568: 53                          	pushq	%rbx
140f80569: 48 83 ec 28                 	subq	$0x28, %rsp
140f8056d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f80574: 48 8d 8d f8 00 00 00        	leaq	0xf8(%rbp), %rcx
140f8057b: e8 00 93 0a ff              	callq	0x140029880 <.text+0x28880>
140f80580: 90                          	nop
140f80581: 48 83 c4 28                 	addq	$0x28, %rsp
140f80585: 5b                          	popq	%rbx
140f80586: 5f                          	popq	%rdi
140f80587: 5e                          	popq	%rsi
140f80588: 5d                          	popq	%rbp
140f80589: c3                          	retq
