
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140fe1101: 80 3f 00                    	cmpb	$0x0, (%rdi)
140fe1104: 66 0f ef d2                 	pxor	%xmm2, %xmm2
140fe1108: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140fe110c: 0f 84 09 01 00 00           	je	0x140fe121b <.text+0xfe021b>
140fe1112: 80 7f 08 00                 	cmpb	$0x0, 0x8(%rdi)
140fe1116: 66 0f ef c9                 	pxor	%xmm1, %xmm1
140fe111a: 0f 84 0e 01 00 00           	je	0x140fe122e <.text+0xfe022e>
140fe1120: 80 7e 28 00                 	cmpb	$0x0, 0x28(%rsi)
140fe1124: 0f 57 e4                    	xorps	%xmm4, %xmm4
140fe1127: 0f 84 13 01 00 00           	je	0x140fe1240 <.text+0xfe0240>
140fe112d: 80 7e 30 00                 	cmpb	$0x0, 0x30(%rsi)
140fe1131: 0f 57 db                    	xorps	%xmm3, %xmm3
140fe1134: 0f 84 18 01 00 00           	je	0x140fe1252 <.text+0xfe0252>
140fe113a: 80 3e 00                    	cmpb	$0x0, (%rsi)
140fe113d: 0f 57 ed                    	xorps	%xmm5, %xmm5
140fe1140: 0f 84 1d 01 00 00           	je	0x140fe1263 <.text+0xfe0263>
140fe1146: 80 7e 08 00                 	cmpb	$0x0, 0x8(%rsi)
140fe114a: 66 45 0f ef c0              	pxor	%xmm8, %xmm8
140fe114f: 0f 84 22 01 00 00           	je	0x140fe1277 <.text+0xfe0277>
140fe1155: 80 7e 10 00                 	cmpb	$0x0, 0x10(%rsi)
140fe1159: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
140fe115d: 0f 84 28 01 00 00           	je	0x140fe128b <.text+0xfe028b>
140fe1163: 80 7e 18 00                 	cmpb	$0x0, 0x18(%rsi)
140fe1167: 75 05                       	jne	0x140fe116e <.text+0xfe016e>
140fe1169: 66 0f 6e 56 1c              	movd	0x1c(%rsi), %xmm2
140fe116e: f3 44 0f 58 c2              	addss	%xmm2, %xmm8
140fe1173: f3 41 0f 58 e9              	addss	%xmm9, %xmm5
140fe1178: f3 0f 58 f5                 	addss	%xmm5, %xmm6
140fe117c: f3 41 0f 58 f8              	addss	%xmm8, %xmm7
