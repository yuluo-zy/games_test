
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403dcf6a: 0f b6 85 76 03 00 00        	movzbl	0x376(%rbp), %eax
1403dcf71: 48 8b 0d 20 f1 24 03        	movq	0x324f120(%rip), %rcx   # 0x14362c098
1403dcf78: 88 01                       	movb	%al, (%rcx)

/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403dd616: 0f b6 85 72 03 00 00        	movzbl	0x372(%rbp), %eax
1403dd61d: 48 8b 0d fc e9 24 03        	movq	0x324e9fc(%rip), %rcx   # 0x14362c020
1403dd624: 88 01                       	movb	%al, (%rcx)

/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140806110: 55                          	pushq	%rbp
140806111: 41 57                       	pushq	%r15
140806113: 41 56                       	pushq	%r14
140806115: 41 55                       	pushq	%r13
140806117: 41 54                       	pushq	%r12
140806119: 56                          	pushq	%rsi
14080611a: 57                          	pushq	%rdi
14080611b: 53                          	pushq	%rbx
14080611c: 48 81 ec e8 01 00 00        	subq	$0x1e8, %rsp            # imm = 0x1E8
140806123: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14080612b: 66 0f 7f b5 50 01 00 00     	movdqa	%xmm6, 0x150(%rbp)
140806133: 48 c7 85 48 01 00 00 fe ff ff ff    	movq	$-0x2, 0x148(%rbp)
14080613e: 4c 89 85 80 00 00 00        	movq	%r8, 0x80(%rbp)
140806145: 48 89 55 68                 	movq	%rdx, 0x68(%rbp)
140806149: 48 8b 05 48 5f e2 02        	movq	0x2e25f48(%rip), %rax   # 0x14362c098
140806150: 80 38 00                    	cmpb	$0x0, (%rax)
140806153: 0f 85 61 0b 00 00           	jne	0x140806cba <.text+0x805cba>

/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14090e070: 0f b6 05 61 6c d2 02        	movzbl	0x2d26c61(%rip), %eax   # 0x143634cd8
14090e077: c3                          	retq

/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a20c9a: 80 3d ef 40 c1 02 00        	cmpb	$0x0, 0x2c140ef(%rip)   # 0x143634d90
140a20ca1: 41 0f 94 c3                 	sete	%r11b
140a20ca5: 47 8d 1c 9b                 	leal	(%r11,%r11,4), %r11d

/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14085a37c: 31 c0                       	xorl	%eax, %eax
14085a37e: 80 3d 0b aa dd 02 00        	cmpb	$0x0, 0x2ddaa0b(%rip)   # 0x143634d90
14085a385: 0f b6 8c 24 40 01 00 00     	movzbl	0x140(%rsp), %ecx
14085a38d: 0f 94 c0                    	sete	%al
14085a390: 8d 04 80                    	leal	(%rax,%rax,4), %eax

/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14090dfe0: 80 3d a9 6d d2 02 00        	cmpb	$0x0, 0x2d26da9(%rip)   # 0x143634d90
14090dfe7: 74 35                       	je	0x14090e01e <.text+0x90d01e>
14090dfe9: 0f 28 d0                    	movaps	%xmm0, %xmm2
14090dfec: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
14090dff0: 0f 28 d9                    	movaps	%xmm1, %xmm3
14090dff3: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
14090dff7: f3 0f 58 da                 	addss	%xmm2, %xmm3
14090dffb: 0f 2e 1d 86 d3 19 02        	ucomiss	0x219d386(%rip), %xmm3  # 0x142aab388
14090e002: 0f 28 d0                    	movaps	%xmm0, %xmm2
14090e005: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
14090e008: 76 4e                       	jbe	0x14090e058 <.text+0x90d058>
14090e00a: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
14090e00e: f3 0f 12 db                 	movsldup	%xmm3, %xmm3            # xmm3 = xmm3[0,0,2,2]
14090e012: 0f 5e d3                    	divps	%xmm3, %xmm2
14090e015: 0f 59 15 94 4e 18 02        	mulps	0x2184e94(%rip), %xmm2  # 0x142a92eb0
14090e01c: eb 3a                       	jmp	0x14090e058 <.text+0x90d058>
14090e01e: 0f 28 d0                    	movaps	%xmm0, %xmm2
14090e021: 0f 14 d1                    	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
14090e024: 0f 28 1d 75 4e 18 02        	movaps	0x2184e75(%rip), %xmm3  # 0x142a92ea0
14090e02b: 0f 28 e3                    	movaps	%xmm3, %xmm4
14090e02e: 0f 5f e2                    	maxps	%xmm2, %xmm4
14090e031: 0f c2 d2 03                 	cmpunordps	%xmm2, %xmm2
14090e035: 0f 54 da                    	andps	%xmm2, %xmm3
14090e038: 0f 55 d4                    	andnps	%xmm4, %xmm2
14090e03b: 0f 56 d3                    	orps	%xmm3, %xmm2
14090e03e: 0f 28 1d 6b 4e 18 02        	movaps	0x2184e6b(%rip), %xmm3  # 0x142a92eb0
14090e045: 0f 28 e3                    	movaps	%xmm3, %xmm4
14090e048: 0f 5d e2                    	minps	%xmm2, %xmm4
14090e04b: 0f c2 d2 03                 	cmpunordps	%xmm2, %xmm2
14090e04f: 0f 54 da                    	andps	%xmm2, %xmm3
14090e052: 0f 55 d4                    	andnps	%xmm4, %xmm2
14090e055: 0f 56 d3                    	orps	%xmm3, %xmm2
14090e058: f3 0f 16 da                 	movshdup	%xmm2, %xmm3            # xmm3 = xmm2[1,1,3,3]
14090e05c: 0f c2 d9 00                 	cmpeqps	%xmm1, %xmm3
14090e060: 0f c2 c2 00                 	cmpeqps	%xmm2, %xmm0
14090e064: 0f 54 c3                    	andps	%xmm3, %xmm0
14090e067: 66 0f 7e c0                 	movd	%xmm0, %eax
14090e06b: 24 01                       	andb	$0x1, %al
14090e06d: c3                          	retq

main config.dev.demo writer: 1403dcf6a..1403dcf7a

main config.dev.internal_hotkeys writer: 1403dd616..1403dd626

maintain original gate: 140806110..140806159

developer getter: 14090e070..14090e078

Place u45: 140a20c9a..140a20ca9

new_wall source69: 14085a37c..14085a393

is_within_glade_shape: 14090dfe0..14090e06e
