
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400f50e0: 55                          	pushq	%rbp
1400f50e1: 41 56                       	pushq	%r14
1400f50e3: 56                          	pushq	%rsi
1400f50e4: 57                          	pushq	%rdi
1400f50e5: 53                          	pushq	%rbx
1400f50e6: 48 81 ec 10 03 00 00        	subq	$0x310, %rsp            # imm = 0x310
1400f50ed: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1400f50f5: 48 c7 85 88 02 00 00 fe ff ff ff    	movq	$-0x2, 0x288(%rbp)
1400f5100: 89 d3                       	movl	%edx, %ebx
1400f5102: 48 89 ce                    	movq	%rcx, %rsi
1400f5105: 0f b6 05 fd fa 53 03        	movzbl	0x353fafd(%rip), %eax   # 0x143634c09
1400f510c: b9 e0 00 00 00              	movl	$0xe0, %ecx
1400f5111: ba 08 00 00 00              	movl	$0x8, %edx
1400f5116: e8 f5 ea 51 00              	callq	0x140613c10 <.text+0x612c10>
1400f511b: 48 85 c0                    	testq	%rax, %rax
1400f511e: 0f 84 98 02 00 00           	je	0x1400f53bc <.text+0xf43bc>
1400f5124: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
1400f512b: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400f512e: 0f 11 45 c0                 	movups	%xmm0, -0x40(%rbp)
1400f5132: 0f 11 45 d8                 	movups	%xmm0, -0x28(%rbp)
1400f5136: 0f 11 45 f0                 	movups	%xmm0, -0x10(%rbp)
1400f513a: 0f 11 45 08                 	movups	%xmm0, 0x8(%rbp)
1400f513e: 0f 11 45 20                 	movups	%xmm0, 0x20(%rbp)
1400f5142: c7 45 30 00 00 00 00        	movl	$0x0, 0x30(%rbp)
1400f5149: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
1400f514d: 0f 11 45 70                 	movups	%xmm0, 0x70(%rbp)
1400f5151: 0f 11 85 88 00 00 00        	movups	%xmm0, 0x88(%rbp)
1400f5158: 0f 11 85 a0 00 00 00        	movups	%xmm0, 0xa0(%rbp)
1400f515f: 0f 11 85 b8 00 00 00        	movups	%xmm0, 0xb8(%rbp)
1400f5166: c7 85 c8 00 00 00 00 00 00 00       	movl	$0x0, 0xc8(%rbp)
1400f5170: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
1400f5178: 48 c7 45 a8 08 00 00 00     	movq	$0x8, -0x58(%rbp)
1400f5180: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
1400f5188: 48 c7 45 b8 10 00 00 00     	movq	$0x10, -0x48(%rbp)
1400f5190: 48 c7 45 d0 10 00 00 00     	movq	$0x10, -0x30(%rbp)
1400f5198: 48 c7 45 e8 10 00 00 00     	movq	$0x10, -0x18(%rbp)
1400f51a0: 48 c7 45 00 10 00 00 00     	movq	$0x10, (%rbp)
1400f51a8: 48 c7 45 18 10 00 00 00     	movq	$0x10, 0x18(%rbp)
1400f51b0: 49 be 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
1400f51ba: 4c 89 75 38                 	movq	%r14, 0x38(%rbp)
1400f51be: 48 8d 05 b5 63 85 02        	leaq	0x28563b5(%rip), %rax   # 0x14294b57a
1400f51c5: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1400f51c9: 48 c7 45 48 50 00 00 00     	movq	$0x50, 0x48(%rbp)
1400f51d1: 48 c7 45 50 10 00 00 00     	movq	$0x10, 0x50(%rbp)
1400f51d9: 48 c7 45 68 10 00 00 00     	movq	$0x10, 0x68(%rbp)
1400f51e1: 48 c7 85 80 00 00 00 10 00 00 00    	movq	$0x10, 0x80(%rbp)
1400f51ec: 48 c7 85 98 00 00 00 10 00 00 00    	movq	$0x10, 0x98(%rbp)
1400f51f7: 48 c7 85 b0 00 00 00 10 00 00 00    	movq	$0x10, 0xb0(%rbp)
1400f5202: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
1400f520c: 66 c7 85 d4 00 00 00 01 00  	movw	$0x1, 0xd4(%rbp)
1400f5215: 4c 89 b5 e0 01 00 00        	movq	%r14, 0x1e0(%rbp)
1400f521c: c7 85 08 02 00 00 00 00 00 00       	movl	$0x0, 0x208(%rbp)
1400f5226: 0f b6 05 dc f9 53 03        	movzbl	0x353f9dc(%rip), %eax   # 0x143634c09
1400f522d: b9 70 02 00 00              	movl	$0x270, %ecx            # imm = 0x270
1400f5232: ba 08 00 00 00              	movl	$0x8, %edx
1400f5237: e8 d4 e9 51 00              	callq	0x140613c10 <.text+0x612c10>
1400f523c: 48 85 c0                    	testq	%rax, %rax
1400f523f: 0f 84 86 01 00 00           	je	0x1400f53cb <.text+0xf43cb>
1400f5245: 48 89 c7                    	movq	%rax, %rdi
1400f5248: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
1400f524c: 41 b8 70 02 00 00           	movl	$0x270, %r8d            # imm = 0x270
1400f5252: 48 89 c1                    	movq	%rax, %rcx
1400f5255: e8 a6 e9 82 02              	callq	0x142923c00 <.text+0x2922c00>
1400f525a: 4c 8d 05 ef 5d 85 02        	leaq	0x2855def(%rip), %r8    # 0x14294b050
1400f5261: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1400f5268: 48 89 fa                    	movq	%rdi, %rdx
1400f526b: e8 a0 f8 be 00              	callq	0x140ce4b10 <.text+0xce3b10>
1400f5270: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1400f5274: e8 97 3f 27 00              	callq	0x140369210 <.text+0x368210>
1400f5279: 0f 10 85 70 02 00 00        	movups	0x270(%rbp), %xmm0
1400f5280: 48 8b 85 80 02 00 00        	movq	0x280(%rbp), %rax
1400f5287: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
1400f528b: 0f 10 85 60 02 00 00        	movups	0x260(%rbp), %xmm0
1400f5292: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
1400f5296: 0f 10 85 50 02 00 00        	movups	0x250(%rbp), %xmm0
1400f529d: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1400f52a1: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
1400f52a8: 0f 10 8d 20 02 00 00        	movups	0x220(%rbp), %xmm1
1400f52af: 0f 10 95 30 02 00 00        	movups	0x230(%rbp), %xmm2
1400f52b6: 0f 10 9d 40 02 00 00        	movups	0x240(%rbp), %xmm3
1400f52bd: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1400f52c1: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1400f52c5: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1400f52c9: 0f 11 00                    	movups	%xmm0, (%rax)
1400f52cc: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
1400f52d0: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
1400f52d4: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
1400f52d8: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
1400f52dc: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
1400f52e0: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
1400f52e7: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
1400f52ee: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
1400f52f5: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
1400f52f9: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
1400f5300: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
1400f5304: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
1400f530b: 0f 10 45 00                 	movups	(%rbp), %xmm0
1400f530f: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
1400f5316: 48 c7 45 a8 02 00 00 00     	movq	$0x2, -0x58(%rbp)
1400f531e: 48 89 45 b0                 	movq	%rax, -0x50(%rbp)
1400f5322: 48 c7 45 b8 02 00 00 00     	movq	$0x2, -0x48(%rbp)
1400f532a: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
1400f5332: 48 c7 45 c8 08 00 00 00     	movq	$0x8, -0x38(%rbp)
1400f533a: 0f 57 c0                    	xorps	%xmm0, %xmm0
1400f533d: 0f 11 45 d0                 	movups	%xmm0, -0x30(%rbp)
1400f5341: 4c 89 75 a0                 	movq	%r14, -0x60(%rbp)
1400f5345: 88 9d 10 02 00 00           	movb	%bl, 0x210(%rbp)
1400f534b: 48 8b 0d 1e 70 53 03        	movq	0x353701e(%rip), %rcx   # 0x14362c370
1400f5352: 4c 8d 05 3f 5c 83 02        	leaq	0x2835c3f(%rip), %r8    # 0x14292af98
1400f5359: 48 8d 95 10 02 00 00        	leaq	0x210(%rbp), %rdx
1400f5360: e8 bb f9 26 00              	callq	0x140364d20 <.text+0x363d20>
1400f5365: 49 89 d0                    	movq	%rdx, %r8
1400f5368: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1400f536c: 48 89 c2                    	movq	%rax, %rdx
1400f536f: e8 4c 15 fe ff              	callq	0x1400d68c0 <.text+0xd58c0>
1400f5374: 0f 10 45 00                 	movups	(%rbp), %xmm0
1400f5378: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
1400f537c: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
1400f5380: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
1400f5384: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
1400f5388: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
1400f538c: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
1400f5390: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
1400f5394: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
1400f5398: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
1400f539c: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
1400f53a0: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
1400f53a4: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
1400f53a8: 0f 11 06                    	movups	%xmm0, (%rsi)
1400f53ab: 48 89 f0                    	movq	%rsi, %rax
1400f53ae: 48 81 c4 10 03 00 00        	addq	$0x310, %rsp            # imm = 0x310
1400f53b5: 5b                          	popq	%rbx
1400f53b6: 5f                          	popq	%rdi
1400f53b7: 5e                          	popq	%rsi
1400f53b8: 41 5e                       	popq	%r14
1400f53ba: 5d                          	popq	%rbp
1400f53bb: c3                          	retq
1400f53bc: b9 08 00 00 00              	movl	$0x8, %ecx
1400f53c1: ba e0 00 00 00              	movl	$0xe0, %edx
1400f53c6: e8 35 3c 7e 02              	callq	0x1428d9000 <.text+0x28d8000>
1400f53cb: b9 08 00 00 00              	movl	$0x8, %ecx
1400f53d0: ba 70 02 00 00              	movl	$0x270, %edx            # imm = 0x270
1400f53d5: e8 26 3c 7e 02              	callq	0x1428d9000 <.text+0x28d8000>
1400f53da: 0f 0b                       	ud2
1400f53dc: 0f 1f 40 00                 	nopl	(%rax)
1400f53e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400f53e5: 55                          	pushq	%rbp
1400f53e6: 41 56                       	pushq	%r14
1400f53e8: 56                          	pushq	%rsi
1400f53e9: 57                          	pushq	%rdi
1400f53ea: 53                          	pushq	%rbx
1400f53eb: 48 83 ec 20                 	subq	$0x20, %rsp
1400f53ef: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400f53f6: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1400f53fa: e8 91 00 4f 00              	callq	0x1405e5490 <.text+0x5e4490>
1400f53ff: 90                          	nop
1400f5400: 48 83 c4 20                 	addq	$0x20, %rsp
1400f5404: 5b                          	popq	%rbx
1400f5405: 5f                          	popq	%rdi
1400f5406: 5e                          	popq	%rsi
1400f5407: 41 5e                       	popq	%r14
1400f5409: 5d                          	popq	%rbp
1400f540a: c3                          	retq
1400f540b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1400f5410: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400f5415: 55                          	pushq	%rbp
1400f5416: 41 56                       	pushq	%r14
1400f5418: 56                          	pushq	%rsi
1400f5419: 57                          	pushq	%rdi
1400f541a: 53                          	pushq	%rbx
1400f541b: 48 83 ec 20                 	subq	$0x20, %rsp
1400f541f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400f5426: ba e0 00 00 00              	movl	$0xe0, %edx
1400f542b: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1400f5431: 48 8b 8d 80 02 00 00        	movq	0x280(%rbp), %rcx
1400f5438: e8 e3 e7 51 00              	callq	0x140613c20 <.text+0x612c20>
1400f543d: 90                          	nop
1400f543e: 48 83 c4 20                 	addq	$0x20, %rsp
1400f5442: 5b                          	popq	%rbx
1400f5443: 5f                          	popq	%rdi
1400f5444: 5e                          	popq	%rsi
1400f5445: 41 5e                       	popq	%r14
1400f5447: 5d                          	popq	%rbp
1400f5448: c3                          	retq
1400f5449: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1400f5450: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400f5455: 55                          	pushq	%rbp
1400f5456: 41 56                       	pushq	%r14
1400f5458: 56                          	pushq	%rsi
1400f5459: 57                          	pushq	%rdi
1400f545a: 53                          	pushq	%rbx
1400f545b: 48 83 ec 20                 	subq	$0x20, %rsp
1400f545f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400f5466: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
1400f546d: e8 0e 44 f3 ff              	callq	0x140029880 <.text+0x28880>
1400f5472: 90                          	nop
1400f5473: 48 83 c4 20                 	addq	$0x20, %rsp
1400f5477: 5b                          	popq	%rbx
1400f5478: 5f                          	popq	%rdi
1400f5479: 5e                          	popq	%rsi
1400f547a: 41 5e                       	popq	%r14
1400f547c: 5d                          	popq	%rbp
1400f547d: c3                          	retq
1400f547e: 66 90                       	nop
1400f5480: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1400f5485: 55                          	pushq	%rbp
1400f5486: 41 56                       	pushq	%r14
1400f5488: 56                          	pushq	%rsi
1400f5489: 57                          	pushq	%rdi
1400f548a: 53                          	pushq	%rbx
1400f548b: 48 83 ec 20                 	subq	$0x20, %rsp
1400f548f: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1400f5496: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
1400f549a: e8 e1 43 f3 ff              	callq	0x140029880 <.text+0x28880>
1400f549f: 90                          	nop
1400f54a0: 48 83 c4 20                 	addq	$0x20, %rsp
1400f54a4: 5b                          	popq	%rbx
1400f54a5: 5f                          	popq	%rdi
1400f54a6: 5e                          	popq	%rsi
1400f54a7: 41 5e                       	popq	%r14
1400f54a9: 5d                          	popq	%rbp
1400f54aa: c3                          	retq
