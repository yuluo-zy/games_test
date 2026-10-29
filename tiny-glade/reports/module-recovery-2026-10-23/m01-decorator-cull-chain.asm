
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140105360: 55                          	pushq	%rbp
140105361: 56                          	pushq	%rsi
140105362: 57                          	pushq	%rdi
140105363: 53                          	pushq	%rbx
140105364: 48 81 ec f8 02 00 00        	subq	$0x2f8, %rsp            # imm = 0x2F8
14010536b: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140105373: 48 c7 85 70 02 00 00 fe ff ff ff    	movq	$-0x2, 0x270(%rbp)
14010537e: 48 89 ce                    	movq	%rcx, %rsi
140105381: 0f b6 05 81 f8 52 03        	movzbl	0x352f881(%rip), %eax   # 0x143634c09
140105388: b9 c0 01 00 00              	movl	$0x1c0, %ecx            # imm = 0x1C0
14010538d: ba 08 00 00 00              	movl	$0x8, %edx
140105392: e8 79 e8 50 00              	callq	0x140613c10 <.text+0x612c10>
140105397: 48 85 c0                    	testq	%rax, %rax
14010539a: 0f 84 a7 03 00 00           	je	0x140105747 <.text+0x104747>
1401053a0: 48 89 85 68 02 00 00        	movq	%rax, 0x268(%rbp)
1401053a7: 0f 57 c0                    	xorps	%xmm0, %xmm0
1401053aa: 0f 11 85 48 01 00 00        	movups	%xmm0, 0x148(%rbp)
1401053b1: 0f 11 85 60 01 00 00        	movups	%xmm0, 0x160(%rbp)
1401053b8: 0f 11 85 78 01 00 00        	movups	%xmm0, 0x178(%rbp)
1401053bf: 0f 11 85 90 01 00 00        	movups	%xmm0, 0x190(%rbp)
1401053c6: 0f 11 85 a8 01 00 00        	movups	%xmm0, 0x1a8(%rbp)
1401053cd: c7 85 b8 01 00 00 00 00 00 00       	movl	$0x0, 0x1b8(%rbp)
1401053d7: 0f 11 85 e0 01 00 00        	movups	%xmm0, 0x1e0(%rbp)
1401053de: 0f 11 85 f8 01 00 00        	movups	%xmm0, 0x1f8(%rbp)
1401053e5: 0f 11 85 10 02 00 00        	movups	%xmm0, 0x210(%rbp)
1401053ec: 0f 11 85 28 02 00 00        	movups	%xmm0, 0x228(%rbp)
1401053f3: 0f 11 85 40 02 00 00        	movups	%xmm0, 0x240(%rbp)
1401053fa: c7 85 50 02 00 00 00 00 00 00       	movl	$0x0, 0x250(%rbp)
140105404: 48 c7 85 f0 00 00 00 00 00 00 00    	movq	$0x0, 0xf0(%rbp)
14010540f: 48 c7 85 28 01 00 00 00 00 00 00    	movq	$0x0, 0x128(%rbp)
14010541a: 48 c7 85 30 01 00 00 08 00 00 00    	movq	$0x8, 0x130(%rbp)
140105425: 48 c7 85 38 01 00 00 00 00 00 00    	movq	$0x0, 0x138(%rbp)
140105430: 48 c7 85 40 01 00 00 10 00 00 00    	movq	$0x10, 0x140(%rbp)
14010543b: 48 c7 85 58 01 00 00 10 00 00 00    	movq	$0x10, 0x158(%rbp)
140105446: 48 c7 85 70 01 00 00 10 00 00 00    	movq	$0x10, 0x170(%rbp)
140105451: 48 c7 85 88 01 00 00 10 00 00 00    	movq	$0x10, 0x188(%rbp)
14010545c: 48 c7 85 a0 01 00 00 10 00 00 00    	movq	$0x10, 0x1a0(%rbp)
140105467: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140105471: 48 89 9d c0 01 00 00        	movq	%rbx, 0x1c0(%rbp)
140105478: 48 8d 05 fa 8b 84 02        	leaq	0x2848bfa(%rip), %rax   # 0x14294e079
14010547f: 48 89 85 c8 01 00 00        	movq	%rax, 0x1c8(%rbp)
140105486: 48 c7 85 d0 01 00 00 43 00 00 00    	movq	$0x43, 0x1d0(%rbp)
140105491: 48 c7 85 d8 01 00 00 10 00 00 00    	movq	$0x10, 0x1d8(%rbp)
14010549c: 48 c7 85 f0 01 00 00 10 00 00 00    	movq	$0x10, 0x1f0(%rbp)
1401054a7: 48 c7 85 08 02 00 00 10 00 00 00    	movq	$0x10, 0x208(%rbp)
1401054b2: 48 c7 85 20 02 00 00 10 00 00 00    	movq	$0x10, 0x220(%rbp)
1401054bd: 48 c7 85 38 02 00 00 10 00 00 00    	movq	$0x10, 0x238(%rbp)
1401054c8: c7 85 58 02 00 00 00 00 00 00       	movl	$0x0, 0x258(%rbp)
1401054d2: 66 c7 85 5c 02 00 00 01 00  	movw	$0x1, 0x25c(%rbp)
1401054db: c7 85 60 02 00 00 00 00 00 00       	movl	$0x0, 0x260(%rbp)
1401054e5: 0f b6 05 1d f7 52 03        	movzbl	0x352f71d(%rip), %eax   # 0x143634c09
1401054ec: b9 78 01 00 00              	movl	$0x178, %ecx            # imm = 0x178
1401054f1: ba 08 00 00 00              	movl	$0x8, %edx
1401054f6: e8 15 e7 50 00              	callq	0x140613c10 <.text+0x612c10>
1401054fb: 48 85 c0                    	testq	%rax, %rax
1401054fe: 0f 84 52 02 00 00           	je	0x140105756 <.text+0x104756>
140105504: 48 89 c7                    	movq	%rax, %rdi
140105507: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
14010550e: 41 b8 78 01 00 00           	movl	$0x178, %r8d            # imm = 0x178
140105514: 48 89 c1                    	movq	%rax, %rcx
140105517: e8 e4 e6 81 02              	callq	0x142923c00 <.text+0x2922c00>
14010551c: 4c 8d 05 e5 89 84 02        	leaq	0x28489e5(%rip), %r8    # 0x14294df08
140105523: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
14010552a: 48 89 fa                    	movq	%rdi, %rdx
14010552d: e8 de f5 bd 00              	callq	0x140ce4b10 <.text+0xce3b10>
140105532: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140105536: e8 85 07 fc ff              	callq	0x1400c5cc0 <.text+0xc4cc0>
14010553b: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14010553f: e8 3c 7d 27 00              	callq	0x14037d280 <.text+0x37c280>
140105544: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
14010554b: e8 b0 0c fc ff              	callq	0x1400c6200 <.text+0xc5200>
140105550: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
140105557: 48 8b 85 68 02 00 00        	movq	0x268(%rbp), %rax
14010555e: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140105562: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140105569: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
14010556d: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
140105574: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140105578: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
14010557f: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
140105586: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
14010558d: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
140105594: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140105598: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14010559c: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1401055a0: 0f 11 00                    	movups	%xmm0, (%rax)
1401055a3: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
1401055a7: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
1401055ab: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
1401055af: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
1401055b3: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
1401055b7: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
1401055be: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
1401055c5: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
1401055cc: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
1401055d0: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
1401055d7: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1401055db: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
1401055e2: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
1401055e6: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
1401055ed: 0f 10 45 00                 	movups	(%rbp), %xmm0
1401055f1: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
1401055f8: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
1401055fc: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140105603: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140105607: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
14010560e: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140105612: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140105616: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
14010561a: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
14010561e: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140105625: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
14010562c: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140105633: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
14010563a: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140105641: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140105648: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
14010564f: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140105656: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
14010565d: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140105664: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
14010566b: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140105672: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140105679: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140105680: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140105687: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
14010568e: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
140105695: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
14010569c: 48 c7 85 f8 00 00 00 04 00 00 00    	movq	$0x4, 0xf8(%rbp)
1401056a7: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1401056ae: 48 c7 85 08 01 00 00 04 00 00 00    	movq	$0x4, 0x108(%rbp)
1401056b9: 48 c7 85 10 01 00 00 00 00 00 00    	movq	$0x0, 0x110(%rbp)
1401056c4: 48 c7 85 18 01 00 00 08 00 00 00    	movq	$0x8, 0x118(%rbp)
1401056cf: 0f 57 c0                    	xorps	%xmm0, %xmm0
1401056d2: 0f 11 85 20 01 00 00        	movups	%xmm0, 0x120(%rbp)
1401056d9: 48 89 9d f0 00 00 00        	movq	%rbx, 0xf0(%rbp)
1401056e0: 48 8d 8d 28 01 00 00        	leaq	0x128(%rbp), %rcx
1401056e7: e8 34 a3 c2 00              	callq	0x140d2fa20 <.text+0xd2ea20>
1401056ec: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
1401056f3: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
1401056f7: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
1401056fe: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140105702: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140105709: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
14010570d: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140105714: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
14010571b: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140105722: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140105729: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
14010572d: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140105731: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140105735: 0f 11 06                    	movups	%xmm0, (%rsi)
140105738: 48 89 f0                    	movq	%rsi, %rax
14010573b: 48 81 c4 f8 02 00 00        	addq	$0x2f8, %rsp            # imm = 0x2F8
140105742: 5b                          	popq	%rbx
140105743: 5f                          	popq	%rdi
140105744: 5e                          	popq	%rsi
140105745: 5d                          	popq	%rbp
140105746: c3                          	retq
140105747: b9 08 00 00 00              	movl	$0x8, %ecx
14010574c: ba c0 01 00 00              	movl	$0x1c0, %edx            # imm = 0x1C0
140105751: e8 aa 38 7d 02              	callq	0x1428d9000 <.text+0x28d8000>
140105756: b9 08 00 00 00              	movl	$0x8, %ecx
14010575b: ba 78 01 00 00              	movl	$0x178, %edx            # imm = 0x178
140105760: e8 9b 38 7d 02              	callq	0x1428d9000 <.text+0x28d8000>
