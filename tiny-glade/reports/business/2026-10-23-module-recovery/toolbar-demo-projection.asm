
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140ac8780: f3 0f 10 19                 	movss	(%rcx), %xmm3
140ac8784: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
140ac8788: 41 0f 59 99 80 00 00 00     	mulps	0x80(%r9), %xmm3
140ac8790: f3 0f 10 61 04              	movss	0x4(%rcx), %xmm4
140ac8795: 0f c6 e4 00                 	shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
140ac8799: 41 0f 59 a1 90 00 00 00     	mulps	0x90(%r9), %xmm4
140ac87a1: f3 0f 10 41 08              	movss	0x8(%rcx), %xmm0
140ac87a6: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140ac87aa: 41 0f 59 81 a0 00 00 00     	mulps	0xa0(%r9), %xmm0
140ac87b2: 0f 58 e3                    	addps	%xmm3, %xmm4
140ac87b5: 0f 58 c4                    	addps	%xmm4, %xmm0
140ac87b8: 41 0f 58 81 b0 00 00 00     	addps	0xb0(%r9), %xmm0
140ac87c0: 0f 28 d8                    	movaps	%xmm0, %xmm3
140ac87c3: 0f c6 d8 00                 	shufps	$0x0, %xmm0, %xmm3      # xmm3 = xmm3[0,0],xmm0[0,0]
140ac87c7: 41 0f 59 19                 	mulps	(%r9), %xmm3
140ac87cb: 0f 28 e0                    	movaps	%xmm0, %xmm4
140ac87ce: 0f c6 e0 55                 	shufps	$0x55, %xmm0, %xmm4     # xmm4 = xmm4[1,1],xmm0[1,1]
140ac87d2: 41 0f 59 61 10              	mulps	0x10(%r9), %xmm4
140ac87d7: 0f c6 c0 aa                 	shufps	$0xaa, %xmm0, %xmm0     # xmm0 = xmm0[2,2,2,2]
140ac87db: 41 0f 59 41 20              	mulps	0x20(%r9), %xmm0
140ac87e0: 0f 58 e3                    	addps	%xmm3, %xmm4
140ac87e3: 0f 58 c4                    	addps	%xmm4, %xmm0
140ac87e6: 41 0f 58 41 30              	addps	0x30(%r9), %xmm0
140ac87eb: f3 0f 16 e0                 	movshdup	%xmm0, %xmm4            # xmm4 = xmm0[1,1,3,3]
140ac87ef: 0f 28 d8                    	movaps	%xmm0, %xmm3
140ac87f2: 0f c6 d8 ff                 	shufps	$0xff, %xmm0, %xmm3     # xmm3 = xmm3[3,3],xmm0[3,3]
140ac87f6: f3 0f 5e c3                 	divss	%xmm3, %xmm0
140ac87fa: f3 0f 5e e3                 	divss	%xmm3, %xmm4
140ac87fe: f3 0f 10 1d 6a d0 e5 01     	movss	0x1e5d06a(%rip), %xmm3  # 0x142925870
140ac8806: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
140ac880a: f3 0f 59 e3                 	mulss	%xmm3, %xmm4
140ac880e: f3 0f 58 c3                 	addss	%xmm3, %xmm0
140ac8812: f3 0f 58 e3                 	addss	%xmm3, %xmm4
140ac8816: f3 0f 10 1d e6 d0 e5 01     	movss	0x1e5d0e6(%rip), %xmm3  # 0x142925904
140ac881e: f3 0f 5c dc                 	subss	%xmm4, %xmm3
140ac8822: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
140ac8826: f3 0f 59 da                 	mulss	%xmm2, %xmm3
140ac882a: 0f 28 cb                    	movaps	%xmm3, %xmm1
140ac882d: c3                          	retq
