
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f7e4e0: 55                          	pushq	%rbp
140f7e4e1: 56                          	pushq	%rsi
140f7e4e2: 57                          	pushq	%rdi
140f7e4e3: 53                          	pushq	%rbx
140f7e4e4: 48 81 ec a8 08 00 00        	subq	$0x8a8, %rsp            # imm = 0x8A8
140f7e4eb: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140f7e4f3: 48 c7 85 20 08 00 00 fe ff ff ff    	movq	$-0x2, 0x820(%rbp)
140f7e4fe: 48 89 ce                    	movq	%rcx, %rsi
140f7e501: 0f 10 42 60                 	movups	0x60(%rdx), %xmm0
140f7e505: 0f 29 85 90 06 00 00        	movaps	%xmm0, 0x690(%rbp)
140f7e50c: 0f 10 42 50                 	movups	0x50(%rdx), %xmm0
140f7e510: 0f 29 85 80 06 00 00        	movaps	%xmm0, 0x680(%rbp)
140f7e517: 0f 10 42 40                 	movups	0x40(%rdx), %xmm0
140f7e51b: 0f 29 85 70 06 00 00        	movaps	%xmm0, 0x670(%rbp)
140f7e522: 0f 10 02                    	movups	(%rdx), %xmm0
140f7e525: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140f7e529: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
140f7e52d: 0f 10 5a 30                 	movups	0x30(%rdx), %xmm3
140f7e531: 0f 29 9d 60 06 00 00        	movaps	%xmm3, 0x660(%rbp)
140f7e538: 0f 29 95 50 06 00 00        	movaps	%xmm2, 0x650(%rbp)
140f7e53f: 0f 29 8d 40 06 00 00        	movaps	%xmm1, 0x640(%rbp)
140f7e546: 0f 29 85 30 06 00 00        	movaps	%xmm0, 0x630(%rbp)
140f7e54d: 0f b6 05 b5 66 6b 02        	movzbl	0x26b66b5(%rip), %eax   # 0x143634c09
140f7e554: b9 00 07 00 00              	movl	$0x700, %ecx            # imm = 0x700
140f7e559: ba 08 00 00 00              	movl	$0x8, %edx
140f7e55e: e8 ad 56 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7e563: 48 85 c0                    	testq	%rax, %rax
140f7e566: 0f 84 70 09 00 00           	je	0x140f7eedc <.text+0xf7dedc>
140f7e56c: 48 89 85 10 08 00 00        	movq	%rax, 0x810(%rbp)
140f7e573: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7e576: 0f 11 85 e8 06 00 00        	movups	%xmm0, 0x6e8(%rbp)
140f7e57d: 0f 11 85 00 07 00 00        	movups	%xmm0, 0x700(%rbp)
140f7e584: 0f 11 85 18 07 00 00        	movups	%xmm0, 0x718(%rbp)
140f7e58b: 0f 11 85 30 07 00 00        	movups	%xmm0, 0x730(%rbp)
140f7e592: 0f 11 85 48 07 00 00        	movups	%xmm0, 0x748(%rbp)
140f7e599: c7 85 58 07 00 00 00 00 00 00       	movl	$0x0, 0x758(%rbp)
140f7e5a3: 0f 11 85 80 07 00 00        	movups	%xmm0, 0x780(%rbp)
140f7e5aa: 0f 11 85 98 07 00 00        	movups	%xmm0, 0x798(%rbp)
140f7e5b1: 0f 11 85 b0 07 00 00        	movups	%xmm0, 0x7b0(%rbp)
140f7e5b8: 0f 11 85 c8 07 00 00        	movups	%xmm0, 0x7c8(%rbp)
140f7e5bf: 0f 11 85 e0 07 00 00        	movups	%xmm0, 0x7e0(%rbp)
140f7e5c6: c7 85 f0 07 00 00 00 00 00 00       	movl	$0x0, 0x7f0(%rbp)
140f7e5d0: 48 c7 85 a0 06 00 00 00 00 00 00    	movq	$0x0, 0x6a0(%rbp)
140f7e5db: 48 c7 85 c8 06 00 00 00 00 00 00    	movq	$0x0, 0x6c8(%rbp)
140f7e5e6: 48 c7 85 d0 06 00 00 08 00 00 00    	movq	$0x8, 0x6d0(%rbp)
140f7e5f1: 48 c7 85 d8 06 00 00 00 00 00 00    	movq	$0x0, 0x6d8(%rbp)
140f7e5fc: 48 c7 85 e0 06 00 00 10 00 00 00    	movq	$0x10, 0x6e0(%rbp)
140f7e607: 48 c7 85 f8 06 00 00 10 00 00 00    	movq	$0x10, 0x6f8(%rbp)
140f7e612: 48 c7 85 10 07 00 00 10 00 00 00    	movq	$0x10, 0x710(%rbp)
140f7e61d: 48 c7 85 28 07 00 00 10 00 00 00    	movq	$0x10, 0x728(%rbp)
140f7e628: 48 c7 85 40 07 00 00 10 00 00 00    	movq	$0x10, 0x740(%rbp)
140f7e633: 48 bb 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140f7e63d: 48 89 9d 60 07 00 00        	movq	%rbx, 0x760(%rbp)
140f7e644: 48 8d 05 95 df c2 01        	leaq	0x1c2df95(%rip), %rax   # 0x142bac5e0
140f7e64b: 48 89 85 68 07 00 00        	movq	%rax, 0x768(%rbp)
140f7e652: 48 c7 85 70 07 00 00 20 00 00 00    	movq	$0x20, 0x770(%rbp)
140f7e65d: 48 c7 85 78 07 00 00 10 00 00 00    	movq	$0x10, 0x778(%rbp)
140f7e668: 48 c7 85 90 07 00 00 10 00 00 00    	movq	$0x10, 0x790(%rbp)
140f7e673: 48 c7 85 a8 07 00 00 10 00 00 00    	movq	$0x10, 0x7a8(%rbp)
140f7e67e: 48 c7 85 c0 07 00 00 10 00 00 00    	movq	$0x10, 0x7c0(%rbp)
140f7e689: 48 c7 85 d8 07 00 00 10 00 00 00    	movq	$0x10, 0x7d8(%rbp)
140f7e694: c7 85 f8 07 00 00 00 00 00 00       	movl	$0x0, 0x7f8(%rbp)
140f7e69e: 66 c7 85 fc 07 00 00 01 00  	movw	$0x1, 0x7fc(%rbp)
140f7e6a7: c7 85 00 08 00 00 00 00 00 00       	movl	$0x0, 0x800(%rbp)
140f7e6b1: 0f b6 05 51 65 6b 02        	movzbl	0x26b6551(%rip), %eax   # 0x143634c09
140f7e6b8: b9 68 01 00 00              	movl	$0x168, %ecx            # imm = 0x168
140f7e6bd: ba 08 00 00 00              	movl	$0x8, %edx
140f7e6c2: e8 49 55 69 ff              	callq	0x140613c10 <.text+0x612c10>
140f7e6c7: 48 85 c0                    	testq	%rax, %rax
140f7e6ca: 0f 84 24 08 00 00           	je	0x140f7eef4 <.text+0xf7def4>
140f7e6d0: 48 89 c7                    	movq	%rax, %rdi
140f7e6d3: 48 8d 95 a0 06 00 00        	leaq	0x6a0(%rbp), %rdx
140f7e6da: 41 b8 68 01 00 00           	movl	$0x168, %r8d            # imm = 0x168
140f7e6e0: 48 89 c1                    	movq	%rax, %rcx
140f7e6e3: e8 18 55 9a 01              	callq	0x142923c00 <.text+0x2922c00>
140f7e6e8: c6 85 1f 08 00 00 01        	movb	$0x1, 0x81f(%rbp)
140f7e6ef: 4c 8d 05 32 de c2 01        	leaq	0x1c2de32(%rip), %r8    # 0x142bac528
140f7e6f6: 48 8d 8d 50 05 00 00        	leaq	0x550(%rbp), %rcx
140f7e6fd: 48 89 fa                    	movq	%rdi, %rdx
140f7e700: e8 0b 64 d6 ff              	callq	0x140ce4b10 <.text+0xce3b10>
140f7e705: c6 85 1d 08 00 00 01        	movb	$0x1, 0x81d(%rbp)
140f7e70c: 48 8d 8d e0 04 00 00        	leaq	0x4e0(%rbp), %rcx
140f7e713: e8 b8 85 e2 00              	callq	0x141da6cd0 <.text+0x1da5cd0>
140f7e718: c6 85 1c 08 00 00 01        	movb	$0x1, 0x81c(%rbp)
140f7e71f: 48 8d 8d 70 04 00 00        	leaq	0x470(%rbp), %rcx
140f7e726: e8 85 8f e2 00              	callq	0x141da76b0 <.text+0x1da66b0>
140f7e72b: c6 85 1b 08 00 00 01        	movb	$0x1, 0x81b(%rbp)
140f7e732: 48 8d 8d 00 04 00 00        	leaq	0x400(%rbp), %rcx
140f7e739: e8 72 a1 e2 00              	callq	0x141da88b0 <.text+0x1da78b0>
140f7e73e: 0f 28 85 90 06 00 00        	movaps	0x690(%rbp), %xmm0
140f7e745: 0f 29 85 00 07 00 00        	movaps	%xmm0, 0x700(%rbp)
140f7e74c: 0f 28 85 80 06 00 00        	movaps	0x680(%rbp), %xmm0
140f7e753: 0f 29 85 f0 06 00 00        	movaps	%xmm0, 0x6f0(%rbp)
140f7e75a: 0f 28 85 70 06 00 00        	movaps	0x670(%rbp), %xmm0
140f7e761: 0f 29 85 e0 06 00 00        	movaps	%xmm0, 0x6e0(%rbp)
140f7e768: 0f 28 85 30 06 00 00        	movaps	0x630(%rbp), %xmm0
140f7e76f: 0f 28 8d 40 06 00 00        	movaps	0x640(%rbp), %xmm1
140f7e776: 0f 28 95 50 06 00 00        	movaps	0x650(%rbp), %xmm2
140f7e77d: 0f 28 9d 60 06 00 00        	movaps	0x660(%rbp), %xmm3
140f7e784: 0f 29 9d d0 06 00 00        	movaps	%xmm3, 0x6d0(%rbp)
140f7e78b: 0f 29 95 c0 06 00 00        	movaps	%xmm2, 0x6c0(%rbp)
140f7e792: 0f 29 8d b0 06 00 00        	movaps	%xmm1, 0x6b0(%rbp)
140f7e799: 0f 29 85 a0 06 00 00        	movaps	%xmm0, 0x6a0(%rbp)
140f7e7a0: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
140f7e7a7: 48 8d 95 a0 06 00 00        	leaq	0x6a0(%rbp), %rdx
140f7e7ae: e8 cd 14 00 00              	callq	0x140f7fc80 <.text+0xf7ec80>
140f7e7b3: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140f7e7ba: e8 51 12 e1 00              	callq	0x141d8fa10 <.text+0x1d8ea10>
140f7e7bf: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140f7e7c6: e8 f5 ad e2 00              	callq	0x141da95c0 <.text+0x1da85c0>
140f7e7cb: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
140f7e7d2: e8 b9 d5 e2 00              	callq	0x141dabd90 <.text+0x1daad90>
140f7e7d7: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140f7e7de: e8 3d e7 ff ff              	callq	0x140f7cf20 <.text+0xf7bf20>
140f7e7e3: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140f7e7ea: e8 f1 d3 e2 00              	callq	0x141dabbe0 <.text+0x1daabe0>
140f7e7ef: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140f7e7f6: e8 f5 e0 e2 00              	callq	0x141dac8f0 <.text+0x1dab8f0>
140f7e7fb: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140f7e802: e8 39 d7 e2 00              	callq	0x141dabf40 <.text+0x1daaf40>
140f7e807: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140f7e80b: e8 d0 ea e2 00              	callq	0x141dad2e0 <.text+0x1dac2e0>
140f7e810: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7e814: e8 a7 f4 e2 00              	callq	0x141dadcc0 <.text+0x1daccc0>
140f7e819: 48 8d 8d c0 05 00 00        	leaq	0x5c0(%rbp), %rcx
140f7e820: e8 2b e3 ff ff              	callq	0x140f7cb50 <.text+0xf7bb50>
140f7e825: 48 8d 8d a0 06 00 00        	leaq	0x6a0(%rbp), %rcx
140f7e82c: e8 7f fe e2 00              	callq	0x141dae6b0 <.text+0x1dad6b0>
140f7e831: 0f 10 85 b0 05 00 00        	movups	0x5b0(%rbp), %xmm0
140f7e838: 48 8b 85 10 08 00 00        	movq	0x810(%rbp), %rax
140f7e83f: 0f 11 40 60                 	movups	%xmm0, 0x60(%rax)
140f7e843: 0f 10 85 a0 05 00 00        	movups	0x5a0(%rbp), %xmm0
140f7e84a: 0f 11 40 50                 	movups	%xmm0, 0x50(%rax)
140f7e84e: 0f 10 85 90 05 00 00        	movups	0x590(%rbp), %xmm0
140f7e855: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
140f7e859: 0f 10 85 50 05 00 00        	movups	0x550(%rbp), %xmm0
140f7e860: 0f 10 8d 60 05 00 00        	movups	0x560(%rbp), %xmm1
140f7e867: 0f 10 95 70 05 00 00        	movups	0x570(%rbp), %xmm2
140f7e86e: 0f 10 9d 80 05 00 00        	movups	0x580(%rbp), %xmm3
140f7e875: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
140f7e879: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
140f7e87d: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
140f7e881: 0f 11 00                    	movups	%xmm0, (%rax)
140f7e884: 0f 10 85 e0 04 00 00        	movups	0x4e0(%rbp), %xmm0
140f7e88b: 0f 10 8d f0 04 00 00        	movups	0x4f0(%rbp), %xmm1
140f7e892: 0f 10 95 00 05 00 00        	movups	0x500(%rbp), %xmm2
140f7e899: 0f 10 9d 10 05 00 00        	movups	0x510(%rbp), %xmm3
140f7e8a0: 0f 11 40 70                 	movups	%xmm0, 0x70(%rax)
140f7e8a4: 0f 11 88 80 00 00 00        	movups	%xmm1, 0x80(%rax)
140f7e8ab: 0f 11 90 90 00 00 00        	movups	%xmm2, 0x90(%rax)
140f7e8b2: 0f 11 98 a0 00 00 00        	movups	%xmm3, 0xa0(%rax)
140f7e8b9: 0f 10 85 20 05 00 00        	movups	0x520(%rbp), %xmm0
140f7e8c0: 0f 11 80 b0 00 00 00        	movups	%xmm0, 0xb0(%rax)
140f7e8c7: 0f 10 85 30 05 00 00        	movups	0x530(%rbp), %xmm0
140f7e8ce: 0f 11 80 c0 00 00 00        	movups	%xmm0, 0xc0(%rax)
140f7e8d5: 0f 10 85 40 05 00 00        	movups	0x540(%rbp), %xmm0
140f7e8dc: 0f 11 80 d0 00 00 00        	movups	%xmm0, 0xd0(%rax)
140f7e8e3: 0f 10 85 d0 04 00 00        	movups	0x4d0(%rbp), %xmm0
140f7e8ea: 0f 11 80 40 01 00 00        	movups	%xmm0, 0x140(%rax)
140f7e8f1: 0f 10 85 c0 04 00 00        	movups	0x4c0(%rbp), %xmm0
140f7e8f8: 0f 11 80 30 01 00 00        	movups	%xmm0, 0x130(%rax)
140f7e8ff: 0f 10 85 b0 04 00 00        	movups	0x4b0(%rbp), %xmm0
140f7e906: 0f 11 80 20 01 00 00        	movups	%xmm0, 0x120(%rax)
140f7e90d: 0f 10 85 70 04 00 00        	movups	0x470(%rbp), %xmm0
140f7e914: 0f 10 8d 80 04 00 00        	movups	0x480(%rbp), %xmm1
140f7e91b: 0f 10 95 90 04 00 00        	movups	0x490(%rbp), %xmm2
140f7e922: 0f 10 9d a0 04 00 00        	movups	0x4a0(%rbp), %xmm3
140f7e929: 0f 11 98 10 01 00 00        	movups	%xmm3, 0x110(%rax)
140f7e930: 0f 11 90 00 01 00 00        	movups	%xmm2, 0x100(%rax)
140f7e937: 0f 11 88 f0 00 00 00        	movups	%xmm1, 0xf0(%rax)
140f7e93e: 0f 11 80 e0 00 00 00        	movups	%xmm0, 0xe0(%rax)
140f7e945: 0f 10 85 00 04 00 00        	movups	0x400(%rbp), %xmm0
140f7e94c: 0f 10 8d 10 04 00 00        	movups	0x410(%rbp), %xmm1
140f7e953: 0f 10 95 20 04 00 00        	movups	0x420(%rbp), %xmm2
140f7e95a: 0f 10 9d 30 04 00 00        	movups	0x430(%rbp), %xmm3
140f7e961: 0f 11 80 50 01 00 00        	movups	%xmm0, 0x150(%rax)
140f7e968: 0f 11 88 60 01 00 00        	movups	%xmm1, 0x160(%rax)
140f7e96f: 0f 11 90 70 01 00 00        	movups	%xmm2, 0x170(%rax)
140f7e976: 0f 11 98 80 01 00 00        	movups	%xmm3, 0x180(%rax)
140f7e97d: 0f 10 85 40 04 00 00        	movups	0x440(%rbp), %xmm0
140f7e984: 0f 11 80 90 01 00 00        	movups	%xmm0, 0x190(%rax)
140f7e98b: 0f 10 85 50 04 00 00        	movups	0x450(%rbp), %xmm0
140f7e992: 0f 11 80 a0 01 00 00        	movups	%xmm0, 0x1a0(%rax)
140f7e999: 0f 10 85 60 04 00 00        	movups	0x460(%rbp), %xmm0
140f7e9a0: 0f 11 80 b0 01 00 00        	movups	%xmm0, 0x1b0(%rax)
140f7e9a7: 0f 10 85 90 03 00 00        	movups	0x390(%rbp), %xmm0
140f7e9ae: 0f 10 8d a0 03 00 00        	movups	0x3a0(%rbp), %xmm1
140f7e9b5: 0f 10 95 b0 03 00 00        	movups	0x3b0(%rbp), %xmm2
140f7e9bc: 0f 10 9d c0 03 00 00        	movups	0x3c0(%rbp), %xmm3
140f7e9c3: 0f 11 80 c0 01 00 00        	movups	%xmm0, 0x1c0(%rax)
140f7e9ca: 0f 11 88 d0 01 00 00        	movups	%xmm1, 0x1d0(%rax)
140f7e9d1: 0f 11 90 e0 01 00 00        	movups	%xmm2, 0x1e0(%rax)
140f7e9d8: 0f 11 98 f0 01 00 00        	movups	%xmm3, 0x1f0(%rax)
140f7e9df: 0f 10 85 d0 03 00 00        	movups	0x3d0(%rbp), %xmm0
140f7e9e6: 0f 11 80 00 02 00 00        	movups	%xmm0, 0x200(%rax)
140f7e9ed: 0f 10 85 f0 03 00 00        	movups	0x3f0(%rbp), %xmm0
140f7e9f4: 0f 11 80 20 02 00 00        	movups	%xmm0, 0x220(%rax)
140f7e9fb: 0f 10 85 e0 03 00 00        	movups	0x3e0(%rbp), %xmm0
140f7ea02: 0f 11 80 10 02 00 00        	movups	%xmm0, 0x210(%rax)
140f7ea09: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
140f7ea10: 0f 11 80 90 02 00 00        	movups	%xmm0, 0x290(%rax)
140f7ea17: 0f 10 85 70 03 00 00        	movups	0x370(%rbp), %xmm0
140f7ea1e: 0f 11 80 80 02 00 00        	movups	%xmm0, 0x280(%rax)
140f7ea25: 0f 10 85 60 03 00 00        	movups	0x360(%rbp), %xmm0
140f7ea2c: 0f 11 80 70 02 00 00        	movups	%xmm0, 0x270(%rax)
140f7ea33: 0f 10 85 20 03 00 00        	movups	0x320(%rbp), %xmm0
140f7ea3a: 0f 10 8d 30 03 00 00        	movups	0x330(%rbp), %xmm1
140f7ea41: 0f 10 95 40 03 00 00        	movups	0x340(%rbp), %xmm2
140f7ea48: 0f 10 9d 50 03 00 00        	movups	0x350(%rbp), %xmm3
140f7ea4f: 0f 11 98 60 02 00 00        	movups	%xmm3, 0x260(%rax)
140f7ea56: 0f 11 90 50 02 00 00        	movups	%xmm2, 0x250(%rax)
140f7ea5d: 0f 11 88 40 02 00 00        	movups	%xmm1, 0x240(%rax)
140f7ea64: 0f 11 80 30 02 00 00        	movups	%xmm0, 0x230(%rax)
140f7ea6b: 0f 10 85 b0 02 00 00        	movups	0x2b0(%rbp), %xmm0
140f7ea72: 0f 10 8d c0 02 00 00        	movups	0x2c0(%rbp), %xmm1
140f7ea79: 0f 10 95 d0 02 00 00        	movups	0x2d0(%rbp), %xmm2
140f7ea80: 0f 10 9d e0 02 00 00        	movups	0x2e0(%rbp), %xmm3
140f7ea87: 0f 11 80 a0 02 00 00        	movups	%xmm0, 0x2a0(%rax)
140f7ea8e: 0f 11 88 b0 02 00 00        	movups	%xmm1, 0x2b0(%rax)
140f7ea95: 0f 11 90 c0 02 00 00        	movups	%xmm2, 0x2c0(%rax)
140f7ea9c: 0f 11 98 d0 02 00 00        	movups	%xmm3, 0x2d0(%rax)
140f7eaa3: 0f 10 85 f0 02 00 00        	movups	0x2f0(%rbp), %xmm0
140f7eaaa: 0f 11 80 e0 02 00 00        	movups	%xmm0, 0x2e0(%rax)
140f7eab1: 0f 10 85 00 03 00 00        	movups	0x300(%rbp), %xmm0
140f7eab8: 0f 11 80 f0 02 00 00        	movups	%xmm0, 0x2f0(%rax)
140f7eabf: 0f 10 85 10 03 00 00        	movups	0x310(%rbp), %xmm0
140f7eac6: 0f 11 80 00 03 00 00        	movups	%xmm0, 0x300(%rax)
140f7eacd: 0f 10 85 40 02 00 00        	movups	0x240(%rbp), %xmm0
140f7ead4: 0f 10 8d 50 02 00 00        	movups	0x250(%rbp), %xmm1
140f7eadb: 0f 10 95 60 02 00 00        	movups	0x260(%rbp), %xmm2
140f7eae2: 0f 10 9d 70 02 00 00        	movups	0x270(%rbp), %xmm3
140f7eae9: 0f 11 80 10 03 00 00        	movups	%xmm0, 0x310(%rax)
140f7eaf0: 0f 11 88 20 03 00 00        	movups	%xmm1, 0x320(%rax)
140f7eaf7: 0f 11 90 30 03 00 00        	movups	%xmm2, 0x330(%rax)
140f7eafe: 0f 11 98 40 03 00 00        	movups	%xmm3, 0x340(%rax)
140f7eb05: 0f 10 85 80 02 00 00        	movups	0x280(%rbp), %xmm0
140f7eb0c: 0f 11 80 50 03 00 00        	movups	%xmm0, 0x350(%rax)
140f7eb13: 0f 10 85 90 02 00 00        	movups	0x290(%rbp), %xmm0
140f7eb1a: 0f 11 80 60 03 00 00        	movups	%xmm0, 0x360(%rax)
140f7eb21: 0f 10 85 a0 02 00 00        	movups	0x2a0(%rbp), %xmm0
140f7eb28: 0f 11 80 70 03 00 00        	movups	%xmm0, 0x370(%rax)
140f7eb2f: 0f 10 85 d0 01 00 00        	movups	0x1d0(%rbp), %xmm0
140f7eb36: 0f 10 8d e0 01 00 00        	movups	0x1e0(%rbp), %xmm1
140f7eb3d: 0f 10 95 f0 01 00 00        	movups	0x1f0(%rbp), %xmm2
140f7eb44: 0f 10 9d 00 02 00 00        	movups	0x200(%rbp), %xmm3
140f7eb4b: 0f 11 80 80 03 00 00        	movups	%xmm0, 0x380(%rax)
140f7eb52: 0f 11 88 90 03 00 00        	movups	%xmm1, 0x390(%rax)
140f7eb59: 0f 11 90 a0 03 00 00        	movups	%xmm2, 0x3a0(%rax)
140f7eb60: 0f 11 98 b0 03 00 00        	movups	%xmm3, 0x3b0(%rax)
140f7eb67: 0f 10 85 10 02 00 00        	movups	0x210(%rbp), %xmm0
140f7eb6e: 0f 11 80 c0 03 00 00        	movups	%xmm0, 0x3c0(%rax)
140f7eb75: 0f 10 85 20 02 00 00        	movups	0x220(%rbp), %xmm0
140f7eb7c: 0f 11 80 d0 03 00 00        	movups	%xmm0, 0x3d0(%rax)
140f7eb83: 0f 10 85 30 02 00 00        	movups	0x230(%rbp), %xmm0
140f7eb8a: 0f 11 80 e0 03 00 00        	movups	%xmm0, 0x3e0(%rax)
140f7eb91: 0f 10 85 60 01 00 00        	movups	0x160(%rbp), %xmm0
140f7eb98: 0f 10 8d 70 01 00 00        	movups	0x170(%rbp), %xmm1
140f7eb9f: 0f 10 95 80 01 00 00        	movups	0x180(%rbp), %xmm2
140f7eba6: 0f 10 9d 90 01 00 00        	movups	0x190(%rbp), %xmm3
140f7ebad: 0f 11 80 f0 03 00 00        	movups	%xmm0, 0x3f0(%rax)
140f7ebb4: 0f 11 88 00 04 00 00        	movups	%xmm1, 0x400(%rax)
140f7ebbb: 0f 11 90 10 04 00 00        	movups	%xmm2, 0x410(%rax)
140f7ebc2: 0f 11 98 20 04 00 00        	movups	%xmm3, 0x420(%rax)
140f7ebc9: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
140f7ebd0: 0f 11 80 30 04 00 00        	movups	%xmm0, 0x430(%rax)
140f7ebd7: 0f 10 85 b0 01 00 00        	movups	0x1b0(%rbp), %xmm0
140f7ebde: 0f 11 80 40 04 00 00        	movups	%xmm0, 0x440(%rax)
140f7ebe5: 0f 10 85 c0 01 00 00        	movups	0x1c0(%rbp), %xmm0
140f7ebec: 0f 11 80 50 04 00 00        	movups	%xmm0, 0x450(%rax)
140f7ebf3: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
140f7ebfa: 0f 10 8d 00 01 00 00        	movups	0x100(%rbp), %xmm1
140f7ec01: 0f 10 95 10 01 00 00        	movups	0x110(%rbp), %xmm2
140f7ec08: 0f 10 9d 20 01 00 00        	movups	0x120(%rbp), %xmm3
140f7ec0f: 0f 11 80 60 04 00 00        	movups	%xmm0, 0x460(%rax)
140f7ec16: 0f 11 88 70 04 00 00        	movups	%xmm1, 0x470(%rax)
140f7ec1d: 0f 11 90 80 04 00 00        	movups	%xmm2, 0x480(%rax)
140f7ec24: 0f 11 98 90 04 00 00        	movups	%xmm3, 0x490(%rax)
140f7ec2b: 0f 10 85 30 01 00 00        	movups	0x130(%rbp), %xmm0
140f7ec32: 0f 11 80 a0 04 00 00        	movups	%xmm0, 0x4a0(%rax)
140f7ec39: 0f 10 85 40 01 00 00        	movups	0x140(%rbp), %xmm0
140f7ec40: 0f 11 80 b0 04 00 00        	movups	%xmm0, 0x4b0(%rax)
140f7ec47: 0f 10 85 50 01 00 00        	movups	0x150(%rbp), %xmm0
140f7ec4e: 0f 11 80 c0 04 00 00        	movups	%xmm0, 0x4c0(%rax)
140f7ec55: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
140f7ec5c: 0f 10 8d 90 00 00 00        	movups	0x90(%rbp), %xmm1
140f7ec63: 0f 10 95 a0 00 00 00        	movups	0xa0(%rbp), %xmm2
140f7ec6a: 0f 10 9d b0 00 00 00        	movups	0xb0(%rbp), %xmm3
140f7ec71: 0f 11 80 d0 04 00 00        	movups	%xmm0, 0x4d0(%rax)
140f7ec78: 0f 11 88 e0 04 00 00        	movups	%xmm1, 0x4e0(%rax)
140f7ec7f: 0f 11 90 f0 04 00 00        	movups	%xmm2, 0x4f0(%rax)
140f7ec86: 0f 11 98 00 05 00 00        	movups	%xmm3, 0x500(%rax)
140f7ec8d: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
140f7ec94: 0f 11 80 10 05 00 00        	movups	%xmm0, 0x510(%rax)
140f7ec9b: 0f 10 85 d0 00 00 00        	movups	0xd0(%rbp), %xmm0
140f7eca2: 0f 11 80 20 05 00 00        	movups	%xmm0, 0x520(%rax)
140f7eca9: 0f 10 85 e0 00 00 00        	movups	0xe0(%rbp), %xmm0
140f7ecb0: 0f 11 80 30 05 00 00        	movups	%xmm0, 0x530(%rax)
140f7ecb7: 0f 10 45 10                 	movups	0x10(%rbp), %xmm0
140f7ecbb: 0f 10 4d 20                 	movups	0x20(%rbp), %xmm1
140f7ecbf: 0f 10 55 30                 	movups	0x30(%rbp), %xmm2
140f7ecc3: 0f 10 5d 40                 	movups	0x40(%rbp), %xmm3
140f7ecc7: 0f 11 80 40 05 00 00        	movups	%xmm0, 0x540(%rax)
140f7ecce: 0f 11 88 50 05 00 00        	movups	%xmm1, 0x550(%rax)
140f7ecd5: 0f 11 90 60 05 00 00        	movups	%xmm2, 0x560(%rax)
140f7ecdc: 0f 11 98 70 05 00 00        	movups	%xmm3, 0x570(%rax)
140f7ece3: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
140f7ece7: 0f 11 80 80 05 00 00        	movups	%xmm0, 0x580(%rax)
140f7ecee: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140f7ecf2: 0f 11 80 90 05 00 00        	movups	%xmm0, 0x590(%rax)
140f7ecf9: 0f 10 45 70                 	movups	0x70(%rbp), %xmm0
140f7ecfd: 0f 11 80 a0 05 00 00        	movups	%xmm0, 0x5a0(%rax)
140f7ed04: 0f 10 45 a0                 	movups	-0x60(%rbp), %xmm0
140f7ed08: 0f 10 4d b0                 	movups	-0x50(%rbp), %xmm1
140f7ed0c: 0f 10 55 c0                 	movups	-0x40(%rbp), %xmm2
140f7ed10: 0f 10 5d d0                 	movups	-0x30(%rbp), %xmm3
140f7ed14: 0f 11 80 b0 05 00 00        	movups	%xmm0, 0x5b0(%rax)
140f7ed1b: 0f 11 88 c0 05 00 00        	movups	%xmm1, 0x5c0(%rax)
140f7ed22: 0f 11 90 d0 05 00 00        	movups	%xmm2, 0x5d0(%rax)
140f7ed29: 0f 11 98 e0 05 00 00        	movups	%xmm3, 0x5e0(%rax)
140f7ed30: 0f 10 45 e0                 	movups	-0x20(%rbp), %xmm0
140f7ed34: 0f 11 80 f0 05 00 00        	movups	%xmm0, 0x5f0(%rax)
140f7ed3b: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140f7ed3f: 0f 11 80 00 06 00 00        	movups	%xmm0, 0x600(%rax)
140f7ed46: 0f 10 45 00                 	movups	(%rbp), %xmm0
140f7ed4a: 0f 11 80 10 06 00 00        	movups	%xmm0, 0x610(%rax)
140f7ed51: 0f 10 85 c0 05 00 00        	movups	0x5c0(%rbp), %xmm0
140f7ed58: 0f 10 8d d0 05 00 00        	movups	0x5d0(%rbp), %xmm1
140f7ed5f: 0f 10 95 e0 05 00 00        	movups	0x5e0(%rbp), %xmm2
140f7ed66: 0f 10 9d f0 05 00 00        	movups	0x5f0(%rbp), %xmm3
140f7ed6d: 0f 11 80 20 06 00 00        	movups	%xmm0, 0x620(%rax)
140f7ed74: 0f 11 88 30 06 00 00        	movups	%xmm1, 0x630(%rax)
140f7ed7b: 0f 11 90 40 06 00 00        	movups	%xmm2, 0x640(%rax)
140f7ed82: 0f 11 98 50 06 00 00        	movups	%xmm3, 0x650(%rax)
140f7ed89: 0f 10 85 00 06 00 00        	movups	0x600(%rbp), %xmm0
140f7ed90: 0f 11 80 60 06 00 00        	movups	%xmm0, 0x660(%rax)
140f7ed97: 0f 10 85 10 06 00 00        	movups	0x610(%rbp), %xmm0
140f7ed9e: 0f 11 80 70 06 00 00        	movups	%xmm0, 0x670(%rax)
140f7eda5: 0f 10 85 20 06 00 00        	movups	0x620(%rbp), %xmm0
140f7edac: 0f 11 80 80 06 00 00        	movups	%xmm0, 0x680(%rax)
140f7edb3: 0f 10 85 a0 06 00 00        	movups	0x6a0(%rbp), %xmm0
140f7edba: 0f 10 8d b0 06 00 00        	movups	0x6b0(%rbp), %xmm1
140f7edc1: 0f 10 95 c0 06 00 00        	movups	0x6c0(%rbp), %xmm2
140f7edc8: 0f 10 9d d0 06 00 00        	movups	0x6d0(%rbp), %xmm3
140f7edcf: 0f 11 80 90 06 00 00        	movups	%xmm0, 0x690(%rax)
140f7edd6: 0f 11 88 a0 06 00 00        	movups	%xmm1, 0x6a0(%rax)
140f7eddd: 0f 11 90 b0 06 00 00        	movups	%xmm2, 0x6b0(%rax)
140f7ede4: 0f 11 98 c0 06 00 00        	movups	%xmm3, 0x6c0(%rax)
140f7edeb: 0f 10 85 e0 06 00 00        	movups	0x6e0(%rbp), %xmm0
140f7edf2: 0f 11 80 d0 06 00 00        	movups	%xmm0, 0x6d0(%rax)
140f7edf9: 0f 10 85 00 07 00 00        	movups	0x700(%rbp), %xmm0
140f7ee00: 0f 11 80 f0 06 00 00        	movups	%xmm0, 0x6f0(%rax)
140f7ee07: 0f 10 85 f0 06 00 00        	movups	0x6f0(%rbp), %xmm0
140f7ee0e: 0f 11 80 e0 06 00 00        	movups	%xmm0, 0x6e0(%rax)
140f7ee15: 48 c7 85 a8 06 00 00 10 00 00 00    	movq	$0x10, 0x6a8(%rbp)
140f7ee20: 48 89 85 b0 06 00 00        	movq	%rax, 0x6b0(%rbp)
140f7ee27: 48 c7 85 b8 06 00 00 10 00 00 00    	movq	$0x10, 0x6b8(%rbp)
140f7ee32: 48 c7 85 c0 06 00 00 00 00 00 00    	movq	$0x0, 0x6c0(%rbp)
140f7ee3d: 48 c7 85 c8 06 00 00 08 00 00 00    	movq	$0x8, 0x6c8(%rbp)
140f7ee48: 0f 57 c0                    	xorps	%xmm0, %xmm0
140f7ee4b: 0f 11 85 d0 06 00 00        	movups	%xmm0, 0x6d0(%rbp)
140f7ee52: 48 8b 85 d0 05 00 00        	movq	0x5d0(%rbp), %rax
140f7ee59: 48 89 85 f0 06 00 00        	movq	%rax, 0x6f0(%rbp)
140f7ee60: 0f 10 85 c0 05 00 00        	movups	0x5c0(%rbp), %xmm0
140f7ee67: 0f 11 85 e0 06 00 00        	movups	%xmm0, 0x6e0(%rbp)
140f7ee6e: 48 89 9d a0 06 00 00        	movq	%rbx, 0x6a0(%rbp)
140f7ee75: 48 8d 8d d8 06 00 00        	leaq	0x6d8(%rbp), %rcx
140f7ee7c: e8 cf 2e 00 00              	callq	0x140f81d50 <.text+0xf80d50>
140f7ee81: 0f 10 85 00 07 00 00        	movups	0x700(%rbp), %xmm0
140f7ee88: 0f 11 46 60                 	movups	%xmm0, 0x60(%rsi)
140f7ee8c: 0f 10 85 f0 06 00 00        	movups	0x6f0(%rbp), %xmm0
140f7ee93: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
140f7ee97: 0f 10 85 e0 06 00 00        	movups	0x6e0(%rbp), %xmm0
140f7ee9e: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
140f7eea2: 0f 10 85 a0 06 00 00        	movups	0x6a0(%rbp), %xmm0
140f7eea9: 0f 10 8d b0 06 00 00        	movups	0x6b0(%rbp), %xmm1
140f7eeb0: 0f 10 95 c0 06 00 00        	movups	0x6c0(%rbp), %xmm2
140f7eeb7: 0f 10 9d d0 06 00 00        	movups	0x6d0(%rbp), %xmm3
140f7eebe: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
140f7eec2: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
140f7eec6: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
140f7eeca: 0f 11 06                    	movups	%xmm0, (%rsi)
140f7eecd: 48 89 f0                    	movq	%rsi, %rax
140f7eed0: 48 81 c4 a8 08 00 00        	addq	$0x8a8, %rsp            # imm = 0x8A8
140f7eed7: 5b                          	popq	%rbx
140f7eed8: 5f                          	popq	%rdi
140f7eed9: 5e                          	popq	%rsi
140f7eeda: 5d                          	popq	%rbp
140f7eedb: c3                          	retq
140f7eedc: c6 85 1e 08 00 00 01        	movb	$0x1, 0x81e(%rbp)
140f7eee3: b9 08 00 00 00              	movl	$0x8, %ecx
140f7eee8: ba 00 07 00 00              	movl	$0x700, %edx            # imm = 0x700
140f7eeed: e8 0e a1 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7eef2: eb 0f                       	jmp	0x140f7ef03 <.text+0xf7df03>
140f7eef4: b9 08 00 00 00              	movl	$0x8, %ecx
140f7eef9: ba 68 01 00 00              	movl	$0x168, %edx            # imm = 0x168
140f7eefe: e8 fd a0 95 01              	callq	0x1428d9000 <.text+0x28d8000>
140f7ef03: 0f 0b                       	ud2
140f7ef05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140f7ef10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7ef15: 55                          	pushq	%rbp
140f7ef16: 56                          	pushq	%rsi
140f7ef17: 57                          	pushq	%rdi
140f7ef18: 53                          	pushq	%rbx
140f7ef19: 48 83 ec 28                 	subq	$0x28, %rsp
140f7ef1d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7ef24: c6 85 1f 08 00 00 01        	movb	$0x1, 0x81f(%rbp)
140f7ef2b: 48 8d 8d a0 06 00 00        	leaq	0x6a0(%rbp), %rcx
140f7ef32: e8 d9 7f 66 ff              	callq	0x1405e6f10 <.text+0x5e5f10>
140f7ef37: c6 85 1f 08 00 00 01        	movb	$0x1, 0x81f(%rbp)
140f7ef3e: 48 83 c4 28                 	addq	$0x28, %rsp
140f7ef42: 5b                          	popq	%rbx
140f7ef43: 5f                          	popq	%rdi
140f7ef44: 5e                          	popq	%rsi
140f7ef45: 5d                          	popq	%rbp
140f7ef46: c3                          	retq
140f7ef47: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140f7ef50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7ef55: 55                          	pushq	%rbp
140f7ef56: 56                          	pushq	%rsi
140f7ef57: 57                          	pushq	%rdi
140f7ef58: 53                          	pushq	%rbx
140f7ef59: 48 83 ec 28                 	subq	$0x28, %rsp
140f7ef5d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7ef64: 0f b6 9d 1f 08 00 00        	movzbl	0x81f(%rbp), %ebx
140f7ef6b: ba 00 07 00 00              	movl	$0x700, %edx            # imm = 0x700
140f7ef70: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140f7ef76: 48 8b 8d 10 08 00 00        	movq	0x810(%rbp), %rcx
140f7ef7d: e8 9e 4c 69 ff              	callq	0x140613c20 <.text+0x612c20>
140f7ef82: 88 9d 1e 08 00 00           	movb	%bl, 0x81e(%rbp)
140f7ef88: 48 83 c4 28                 	addq	$0x28, %rsp
140f7ef8c: 5b                          	popq	%rbx
140f7ef8d: 5f                          	popq	%rdi
140f7ef8e: 5e                          	popq	%rsi
140f7ef8f: 5d                          	popq	%rbp
140f7ef90: c3                          	retq
140f7ef91: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140f7efa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7efa5: 55                          	pushq	%rbp
140f7efa6: 56                          	pushq	%rsi
140f7efa7: 57                          	pushq	%rdi
140f7efa8: 53                          	pushq	%rbx
140f7efa9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7efad: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7efb4: 48 8d 8d c0 05 00 00        	leaq	0x5c0(%rbp), %rcx
140f7efbb: e8 c0 a8 0a ff              	callq	0x140029880 <.text+0x28880>
140f7efc0: 90                          	nop
140f7efc1: 48 83 c4 28                 	addq	$0x28, %rsp
140f7efc5: 5b                          	popq	%rbx
140f7efc6: 5f                          	popq	%rdi
140f7efc7: 5e                          	popq	%rsi
140f7efc8: 5d                          	popq	%rbp
140f7efc9: c3                          	retq
140f7efca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7efd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7efd5: 55                          	pushq	%rbp
140f7efd6: 56                          	pushq	%rsi
140f7efd7: 57                          	pushq	%rdi
140f7efd8: 53                          	pushq	%rbx
140f7efd9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7efdd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7efe4: 80 bd 1e 08 00 00 00        	cmpb	$0x0, 0x81e(%rbp)
140f7efeb: 75 09                       	jne	0x140f7eff6 <.text+0xf7dff6>
140f7efed: 48 83 c4 28                 	addq	$0x28, %rsp
140f7eff1: 5b                          	popq	%rbx
140f7eff2: 5f                          	popq	%rdi
140f7eff3: 5e                          	popq	%rsi
140f7eff4: 5d                          	popq	%rbp
140f7eff5: c3                          	retq
140f7eff6: 48 8d 8d 30 06 00 00        	leaq	0x630(%rbp), %rcx
140f7effd: e8 7e a8 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f002: eb e9                       	jmp	0x140f7efed <.text+0xf7dfed>
140f7f004: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140f7f010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f015: 55                          	pushq	%rbp
140f7f016: 56                          	pushq	%rsi
140f7f017: 57                          	pushq	%rdi
140f7f018: 53                          	pushq	%rbx
140f7f019: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f01d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f024: 48 8d 8d a0 06 00 00        	leaq	0x6a0(%rbp), %rcx
140f7f02b: e8 50 a8 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f030: 90                          	nop
140f7f031: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f035: 5b                          	popq	%rbx
140f7f036: 5f                          	popq	%rdi
140f7f037: 5e                          	popq	%rsi
140f7f038: 5d                          	popq	%rbp
140f7f039: c3                          	retq
140f7f03a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f040: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f045: 55                          	pushq	%rbp
140f7f046: 56                          	pushq	%rsi
140f7f047: 57                          	pushq	%rdi
140f7f048: 53                          	pushq	%rbx
140f7f049: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f04d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f054: 0f b6 9d 1d 08 00 00        	movzbl	0x81d(%rbp), %ebx
140f7f05b: 48 8d 8d 50 05 00 00        	leaq	0x550(%rbp), %rcx
140f7f062: e8 19 a8 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f067: 88 9d 1f 08 00 00           	movb	%bl, 0x81f(%rbp)
140f7f06d: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f071: 5b                          	popq	%rbx
140f7f072: 5f                          	popq	%rdi
140f7f073: 5e                          	popq	%rsi
140f7f074: 5d                          	popq	%rbp
140f7f075: c3                          	retq
140f7f076: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140f7f080: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f085: 55                          	pushq	%rbp
140f7f086: 56                          	pushq	%rsi
140f7f087: 57                          	pushq	%rdi
140f7f088: 53                          	pushq	%rbx
140f7f089: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f08d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f094: 0f b6 9d 1c 08 00 00        	movzbl	0x81c(%rbp), %ebx
140f7f09b: 48 8d 8d e0 04 00 00        	leaq	0x4e0(%rbp), %rcx
140f7f0a2: e8 d9 a7 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f0a7: 88 9d 1d 08 00 00           	movb	%bl, 0x81d(%rbp)
140f7f0ad: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f0b1: 5b                          	popq	%rbx
140f7f0b2: 5f                          	popq	%rdi
140f7f0b3: 5e                          	popq	%rsi
140f7f0b4: 5d                          	popq	%rbp
140f7f0b5: c3                          	retq
140f7f0b6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140f7f0c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f0c5: 55                          	pushq	%rbp
140f7f0c6: 56                          	pushq	%rsi
140f7f0c7: 57                          	pushq	%rdi
140f7f0c8: 53                          	pushq	%rbx
140f7f0c9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f0cd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f0d4: 0f b6 9d 1b 08 00 00        	movzbl	0x81b(%rbp), %ebx
140f7f0db: 48 8d 8d 70 04 00 00        	leaq	0x470(%rbp), %rcx
140f7f0e2: e8 99 a7 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f0e7: 88 9d 1c 08 00 00           	movb	%bl, 0x81c(%rbp)
140f7f0ed: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f0f1: 5b                          	popq	%rbx
140f7f0f2: 5f                          	popq	%rdi
140f7f0f3: 5e                          	popq	%rsi
140f7f0f4: 5d                          	popq	%rbp
140f7f0f5: c3                          	retq
140f7f0f6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140f7f100: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f105: 55                          	pushq	%rbp
140f7f106: 56                          	pushq	%rsi
140f7f107: 57                          	pushq	%rdi
140f7f108: 53                          	pushq	%rbx
140f7f109: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f10d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f114: 48 8d 8d 00 04 00 00        	leaq	0x400(%rbp), %rcx
140f7f11b: e8 60 a7 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f120: c6 85 1b 08 00 00 00        	movb	$0x0, 0x81b(%rbp)
140f7f127: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f12b: 5b                          	popq	%rbx
140f7f12c: 5f                          	popq	%rdi
140f7f12d: 5e                          	popq	%rsi
140f7f12e: 5d                          	popq	%rbp
140f7f12f: c3                          	retq
140f7f130: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f135: 55                          	pushq	%rbp
140f7f136: 56                          	pushq	%rsi
140f7f137: 57                          	pushq	%rdi
140f7f138: 53                          	pushq	%rbx
140f7f139: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f13d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f144: 48 8d 8d 90 03 00 00        	leaq	0x390(%rbp), %rcx
140f7f14b: e8 30 a7 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f150: 90                          	nop
140f7f151: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f155: 5b                          	popq	%rbx
140f7f156: 5f                          	popq	%rdi
140f7f157: 5e                          	popq	%rsi
140f7f158: 5d                          	popq	%rbp
140f7f159: c3                          	retq
140f7f15a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f160: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f165: 55                          	pushq	%rbp
140f7f166: 56                          	pushq	%rsi
140f7f167: 57                          	pushq	%rdi
140f7f168: 53                          	pushq	%rbx
140f7f169: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f16d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f174: 48 8d 8d 20 03 00 00        	leaq	0x320(%rbp), %rcx
140f7f17b: e8 00 a7 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f180: 90                          	nop
140f7f181: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f185: 5b                          	popq	%rbx
140f7f186: 5f                          	popq	%rdi
140f7f187: 5e                          	popq	%rsi
140f7f188: 5d                          	popq	%rbp
140f7f189: c3                          	retq
140f7f18a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f190: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f195: 55                          	pushq	%rbp
140f7f196: 56                          	pushq	%rsi
140f7f197: 57                          	pushq	%rdi
140f7f198: 53                          	pushq	%rbx
140f7f199: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f19d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f1a4: 48 8d 8d b0 02 00 00        	leaq	0x2b0(%rbp), %rcx
140f7f1ab: e8 d0 a6 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f1b0: 90                          	nop
140f7f1b1: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f1b5: 5b                          	popq	%rbx
140f7f1b6: 5f                          	popq	%rdi
140f7f1b7: 5e                          	popq	%rsi
140f7f1b8: 5d                          	popq	%rbp
140f7f1b9: c3                          	retq
140f7f1ba: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f1c0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f1c5: 55                          	pushq	%rbp
140f7f1c6: 56                          	pushq	%rsi
140f7f1c7: 57                          	pushq	%rdi
140f7f1c8: 53                          	pushq	%rbx
140f7f1c9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f1cd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f1d4: 48 8d 8d 40 02 00 00        	leaq	0x240(%rbp), %rcx
140f7f1db: e8 a0 a6 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f1e0: 90                          	nop
140f7f1e1: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f1e5: 5b                          	popq	%rbx
140f7f1e6: 5f                          	popq	%rdi
140f7f1e7: 5e                          	popq	%rsi
140f7f1e8: 5d                          	popq	%rbp
140f7f1e9: c3                          	retq
140f7f1ea: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f1f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f1f5: 55                          	pushq	%rbp
140f7f1f6: 56                          	pushq	%rsi
140f7f1f7: 57                          	pushq	%rdi
140f7f1f8: 53                          	pushq	%rbx
140f7f1f9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f1fd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f204: 48 8d 8d d0 01 00 00        	leaq	0x1d0(%rbp), %rcx
140f7f20b: e8 70 a6 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f210: 90                          	nop
140f7f211: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f215: 5b                          	popq	%rbx
140f7f216: 5f                          	popq	%rdi
140f7f217: 5e                          	popq	%rsi
140f7f218: 5d                          	popq	%rbp
140f7f219: c3                          	retq
140f7f21a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f220: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f225: 55                          	pushq	%rbp
140f7f226: 56                          	pushq	%rsi
140f7f227: 57                          	pushq	%rdi
140f7f228: 53                          	pushq	%rbx
140f7f229: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f22d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f234: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140f7f23b: e8 40 a6 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f240: 90                          	nop
140f7f241: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f245: 5b                          	popq	%rbx
140f7f246: 5f                          	popq	%rdi
140f7f247: 5e                          	popq	%rsi
140f7f248: 5d                          	popq	%rbp
140f7f249: c3                          	retq
140f7f24a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f250: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f255: 55                          	pushq	%rbp
140f7f256: 56                          	pushq	%rsi
140f7f257: 57                          	pushq	%rdi
140f7f258: 53                          	pushq	%rbx
140f7f259: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f25d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f264: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140f7f26b: e8 10 a6 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f270: 90                          	nop
140f7f271: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f275: 5b                          	popq	%rbx
140f7f276: 5f                          	popq	%rdi
140f7f277: 5e                          	popq	%rsi
140f7f278: 5d                          	popq	%rbp
140f7f279: c3                          	retq
140f7f27a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f280: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f285: 55                          	pushq	%rbp
140f7f286: 56                          	pushq	%rsi
140f7f287: 57                          	pushq	%rdi
140f7f288: 53                          	pushq	%rbx
140f7f289: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f28d: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f294: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
140f7f29b: e8 e0 a5 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f2a0: 90                          	nop
140f7f2a1: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f2a5: 5b                          	popq	%rbx
140f7f2a6: 5f                          	popq	%rdi
140f7f2a7: 5e                          	popq	%rsi
140f7f2a8: 5d                          	popq	%rbp
140f7f2a9: c3                          	retq
140f7f2aa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140f7f2b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f2b5: 55                          	pushq	%rbp
140f7f2b6: 56                          	pushq	%rsi
140f7f2b7: 57                          	pushq	%rdi
140f7f2b8: 53                          	pushq	%rbx
140f7f2b9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f2bd: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f2c4: 48 8d 4d 10                 	leaq	0x10(%rbp), %rcx
140f7f2c8: e8 b3 a5 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f2cd: 90                          	nop
140f7f2ce: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f2d2: 5b                          	popq	%rbx
140f7f2d3: 5f                          	popq	%rdi
140f7f2d4: 5e                          	popq	%rsi
140f7f2d5: 5d                          	popq	%rbp
140f7f2d6: c3                          	retq
140f7f2d7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140f7f2e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140f7f2e5: 55                          	pushq	%rbp
140f7f2e6: 56                          	pushq	%rsi
140f7f2e7: 57                          	pushq	%rdi
140f7f2e8: 53                          	pushq	%rbx
140f7f2e9: 48 83 ec 28                 	subq	$0x28, %rsp
140f7f2ed: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140f7f2f4: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
140f7f2f8: e8 83 a5 0a ff              	callq	0x140029880 <.text+0x28880>
140f7f2fd: 90                          	nop
140f7f2fe: 48 83 c4 28                 	addq	$0x28, %rsp
140f7f302: 5b                          	popq	%rbx
140f7f303: 5f                          	popq	%rdi
140f7f304: 5e                          	popq	%rsi
140f7f305: 5d                          	popq	%rbp
140f7f306: c3                          	retq
