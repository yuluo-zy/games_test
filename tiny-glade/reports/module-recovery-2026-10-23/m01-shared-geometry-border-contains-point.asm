
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c980f0: f2 0f 10 01                 	movsd	(%rcx), %xmm0
140c980f4: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
140c980f7: 0f 5c c8                    	subps	%xmm0, %xmm1
140c980fa: 0f 59 c9                    	mulps	%xmm1, %xmm1
140c980fd: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
140c98101: f3 0f 58 c1                 	addss	%xmm1, %xmm0
140c98105: f3 0f 10 49 08              	movss	0x8(%rcx), %xmm1
140c9810a: f3 0f 59 c9                 	mulss	%xmm1, %xmm1
140c9810e: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
140c98111: 0f 97 c0                    	seta	%al
140c98114: c3                          	retq
