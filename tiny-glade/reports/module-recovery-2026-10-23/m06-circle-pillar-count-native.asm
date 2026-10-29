
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14124baf2: 4c 89 c1                    	movq	%r8, %rcx
14124baf5: e8 76 be a4 ff              	callq	0x140c97970 <.text+0xc96970>
14124bafa: 0f 28 d0                    	movaps	%xmm0, %xmm2
14124bafd: c6 85 f7 00 00 00 01        	movb	$0x1, 0xf7(%rbp)
14124bb04: f3 0f 10 05 24 3f 83 01     	movss	0x1833f24(%rip), %xmm0  # 0x142a7fa30
14124bb0c: f3 0f 10 0d 5c 15 78 01     	movss	0x178155c(%rip), %xmm1  # 0x1429cd070
14124bb14: e8 87 49 a4 ff              	callq	0x140c904a0 <.text+0xc8f4a0>
14124bb19: 48 8b b5 18 02 00 00        	movq	0x218(%rbp), %rsi
14124bb20: 0f 57 c9                    	xorps	%xmm1, %xmm1
14124bb23: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
14124bb27: f3 0f 10 35 d5 9d 6d 01     	movss	0x16d9dd5(%rip), %xmm6  # 0x142925904
14124bb2f: 0f 28 c6                    	movaps	%xmm6, %xmm0
14124bb32: f3 0f 5d c1                 	minss	%xmm1, %xmm0
14124bb36: 0f 28 ce                    	movaps	%xmm6, %xmm1
14124bb39: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14124bb3d: f3 0f 59 0d 4f 8b 6e 01     	mulss	0x16e8b4f(%rip), %xmm1  # 0x142934694
14124bb45: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
14124bb49: f3 0f 59 05 f7 11 83 01     	mulss	0x18311f7(%rip), %xmm0  # 0x142a7cd48
14124bb51: f3 0f 58 c1                 	addss	%xmm1, %xmm0
14124bb55: e8 46 84 6d 01              	callq	0x142923fa0 <.text+0x2922fa0>
14124bb5a: f3 48 0f 2c c0              	cvttss2si	%xmm0, %rax
14124bb5f: 48 89 c1                    	movq	%rax, %rcx
14124bb62: 0f 28 c8                    	movaps	%xmm0, %xmm1
14124bb65: f3 0f 5c 0d 6f e5 70 01     	subss	0x170e56f(%rip), %xmm1  # 0x14295a0dc
14124bb6d: f3 48 0f 2c d1              	cvttss2si	%xmm1, %rdx
14124bb72: 48 c1 f9 3f                 	sarq	$0x3f, %rcx
14124bb76: 48 21 ca                    	andq	%rcx, %rdx
14124bb79: 48 09 c2                    	orq	%rax, %rdx
14124bb7c: 45 31 ff                    	xorl	%r15d, %r15d
14124bb7f: 41 0f 2e c6                 	ucomiss	%xmm14, %xmm0
14124bb83: 49 0f 42 d7                 	cmovbq	%r15, %rdx
14124bb87: 0f 2e 05 52 e5 70 01        	ucomiss	0x170e552(%rip), %xmm0  # 0x14295a0e0
14124bb8e: 48 c7 c1 ff ff ff ff        	movq	$-0x1, %rcx
14124bb95: 48 0f 46 ca                 	cmovbeq	%rdx, %rcx
14124bb99: 48 85 c9                    	testq	%rcx, %rcx
