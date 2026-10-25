
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c90700: 48 83 ec 58                 	subq	$0x58, %rsp
140c90704: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
140c9070a: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140c9070f: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
140c90714: 66 0f 6e c1                 	movd	%ecx, %xmm0
140c90718: 48 c1 e9 20                 	shrq	$0x20, %rcx
140c9071c: 66 0f 6e f1                 	movd	%ecx, %xmm6
140c90720: 0f 57 ff                    	xorps	%xmm7, %xmm7
140c90723: f3 0f 51 f8                 	sqrtss	%xmm0, %xmm7
140c90727: f3 0f 59 35 45 3f ca 01     	mulss	0x1ca3f45(%rip), %xmm6  # 0x142934674
140c9072f: 0f 28 c6                    	movaps	%xmm6, %xmm0
140c90732: e8 49 37 c9 01              	callq	0x142923e80 <.text+0x2922e80>
140c90737: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
140c9073b: f3 44 0f 59 c7              	mulss	%xmm7, %xmm8
140c90740: 0f 28 c6                    	movaps	%xmm6, %xmm0
140c90743: e8 88 38 c9 01              	callq	0x142923fd0 <.text+0x2922fd0>
140c90748: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
140c9074c: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
140c90750: 0f 28 cf                    	movaps	%xmm7, %xmm1
140c90753: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140c90758: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140c9075d: 44 0f 28 44 24 40           	movaps	0x40(%rsp), %xmm8
140c90763: 48 83 c4 58                 	addq	$0x58, %rsp
140c90767: c3                          	retq
