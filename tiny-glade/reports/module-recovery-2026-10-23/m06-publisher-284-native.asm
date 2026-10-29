
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a8d7a4: 45 0f 28 c7                 	movaps	%xmm15, %xmm8
140a8d7a8: 41 0f 28 df                 	movaps	%xmm15, %xmm3
140a8d7ac: 80 bd 98 0c 00 00 00        	cmpb	$0x0, 0xc98(%rbp)
140a8d7b3: 75 03                       	jne	0x140a8d7b8 <.text+0xa8c7b8>
140a8d7b5: 0f 28 de                    	movaps	%xmm6, %xmm3
140a8d7b8: 44 0f 28 fe                 	movaps	%xmm6, %xmm15
140a8d7bc: 41 0f 28 f3                 	movaps	%xmm11, %xmm6
140a8d7c0: 41 0f 28 d6                 	movaps	%xmm14, %xmm2
140a8d7c4: 75 08                       	jne	0x140a8d7ce <.text+0xa8c7ce>
140a8d7c6: f3 0f 10 15 32 06 ff 01     	movss	0x1ff0632(%rip), %xmm2  # 0x142a7de00
140a8d7ce: 48 83 bd 88 0b 00 00 00     	cmpq	$0x0, 0xb88(%rbp)
140a8d7d6: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a8d7d9: 4c 8d 85 08 07 00 00        	leaq	0x708(%rbp), %r8
140a8d7e0: 41 0f 11 48 30              	movups	%xmm1, 0x30(%r8)
140a8d7e5: 41 0f 11 48 20              	movups	%xmm1, 0x20(%r8)
140a8d7ea: 41 0f 11 48 10              	movups	%xmm1, 0x10(%r8)
140a8d7ef: 41 0f 11 08                 	movups	%xmm1, (%r8)
140a8d7f3: 49 c7 40 40 00 00 00 00     	movq	$0x0, 0x40(%r8)
140a8d7fb: 48 8b 8d 18 0d 00 00        	movq	0xd18(%rbp), %rcx
140a8d802: 48 89 8d 70 05 00 00        	movq	%rcx, 0x570(%rbp)
140a8d809: 48 8b 95 38 0d 00 00        	movq	0xd38(%rbp), %rdx
140a8d810: 48 89 95 78 05 00 00        	movq	%rdx, 0x578(%rbp)
140a8d817: 48 89 8d 80 05 00 00        	movq	%rcx, 0x580(%rbp)
140a8d81e: 4c 8b 9d 40 0d 00 00        	movq	0xd40(%rbp), %r11
140a8d825: f3 41 0f 10 63 e8           	movss	-0x18(%r11), %xmm4
140a8d82b: f3 41 0f 10 6b ec           	movss	-0x14(%r11), %xmm5
140a8d831: 66 41 0f 6e 4b f0           	movd	-0x10(%r11), %xmm1
140a8d837: 41 0f b6 4b f4              	movzbl	-0xc(%r11), %ecx
140a8d83c: 41 0f b6 53 f6              	movzbl	-0xa(%r11), %edx
140a8d841: 45 0f b6 53 f7              	movzbl	-0x9(%r11), %r10d
140a8d846: 45 0f b7 5b f8              	movzwl	-0x8(%r11), %r11d
140a8d84b: 4c 8b 8d a0 0c 00 00        	movq	0xca0(%rbp), %r9
140a8d852: 4c 89 8d 00 05 00 00        	movq	%r9, 0x500(%rbp)
140a8d859: 4c 8b 8d 20 0c 00 00        	movq	0xc20(%rbp), %r9
140a8d860: 4c 89 8d 08 05 00 00        	movq	%r9, 0x508(%rbp)
140a8d867: 4c 8b 8d 08 0d 00 00        	movq	0xd08(%rbp), %r9
140a8d86e: 4c 89 8d 10 05 00 00        	movq	%r9, 0x510(%rbp)
140a8d875: 4c 8b 8d a8 0c 00 00        	movq	0xca8(%rbp), %r9
140a8d87c: 4c 89 8d 18 05 00 00        	movq	%r9, 0x518(%rbp)
140a8d883: 4c 8b 8d 80 0c 00 00        	movq	0xc80(%rbp), %r9
140a8d88a: 4c 89 8d 20 05 00 00        	movq	%r9, 0x520(%rbp)
140a8d891: 48 89 b5 28 05 00 00        	movq	%rsi, 0x528(%rbp)
140a8d898: 66 0f 7e bd 30 05 00 00     	movd	%xmm7, 0x530(%rbp)
140a8d8a0: 48 8b b5 90 02 00 00        	movq	0x290(%rbp), %rsi
140a8d8a7: 49 89 b0 60 fe ff ff        	movq	%rsi, -0x1a0(%r8)
140a8d8ae: f3 41 0f 5f dc              	maxss	%xmm12, %xmm3
140a8d8b3: f3 0f 5d d3                 	minss	%xmm3, %xmm2
140a8d8b7: 0f 10 9d 60 02 00 00        	movups	0x260(%rbp), %xmm3
140a8d8be: 44 0f 10 9d 70 02 00 00     	movups	0x270(%rbp), %xmm11
140a8d8c6: 44 0f 10 a5 80 02 00 00     	movups	0x280(%rbp), %xmm12
140a8d8ce: 45 0f 11 a0 50 fe ff ff     	movups	%xmm12, -0x1b0(%r8)
140a8d8d6: 45 0f 11 98 40 fe ff ff     	movups	%xmm11, -0x1c0(%r8)
140a8d8de: 41 0f 11 98 30 fe ff ff     	movups	%xmm3, -0x1d0(%r8)
140a8d8e6: f3 44 0f 11 95 74 07 00 00  	movss	%xmm10, 0x774(%rbp)
140a8d8ef: 0f 57 db                    	xorps	%xmm3, %xmm3
140a8d8f2: f3 44 0f 11 ad 78 07 00 00  	movss	%xmm13, 0x778(%rbp)
140a8d8fb: f3 44 0f 11 8d 7c 07 00 00  	movss	%xmm9, 0x77c(%rbp)
140a8d904: f3 0f 11 a5 80 07 00 00     	movss	%xmm4, 0x780(%rbp)
140a8d90c: f3 0f 11 95 84 07 00 00     	movss	%xmm2, 0x784(%rbp)
140a8d914: f3 0f 11 ad 88 07 00 00     	movss	%xmm5, 0x788(%rbp)
