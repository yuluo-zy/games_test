
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141762b90: f3 0f 10 01                 	movss	(%rcx), %xmm0
141762b94: f3 0f 10 4a 04              	movss	0x4(%rdx), %xmm1
141762b99: f3 0f 10 51 04              	movss	0x4(%rcx), %xmm2
141762b9e: f3 0f 10 1a                 	movss	(%rdx), %xmm3
141762ba2: 0f c2 da 02                 	cmpleps	%xmm2, %xmm3
141762ba6: 0f c2 c1 02                 	cmpleps	%xmm1, %xmm0
141762baa: 0f 54 c3                    	andps	%xmm3, %xmm0
141762bad: 66 0f 7e c0                 	movd	%xmm0, %eax
141762bb1: 24 01                       	andb	$0x1, %al
141762bb3: c3                          	retq
