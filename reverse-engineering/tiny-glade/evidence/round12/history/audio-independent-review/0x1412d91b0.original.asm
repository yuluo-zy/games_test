
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412d91b0: 56                          	pushq	%rsi
1412d91b1: 48 83 ec 40                 	subq	$0x40, %rsp
1412d91b5: 48 85 d2                    	testq	%rdx, %rdx
1412d91b8: 0f 84 a2 00 00 00           	je	0x1412d9260 <.text+0x12d8260>
1412d91be: 48 89 d6                    	movq	%rdx, %rsi
1412d91c1: 48 8d 4a 38                 	leaq	0x38(%rdx), %rcx
1412d91c5: 48 ba e1 46 a3 14 d0 6c 55 3a       	movabsq	$0x3a556cd014a346e1, %rdx # imm = 0x3A556CD014A346E1
1412d91cf: 49 b8 a1 d6 90 86 3c 1a 6f 79       	movabsq	$0x796f1a3c8690d6a1, %r8 # imm = 0x796F1A3C8690D6A1
1412d91d9: e8 12 45 e0 fe              	callq	0x1400dd6f0 <.text+0xdc6f0>
1412d91de: 48 85 c0                    	testq	%rax, %rax
1412d91e1: 74 60                       	je	0x1412d9243 <.text+0x12d8243>
1412d91e3: 48 39 96 18 02 00 00        	cmpq	%rdx, 0x218(%rsi)
1412d91ea: 76 57                       	jbe	0x1412d9243 <.text+0x12d8243>
1412d91ec: 48 8b 86 10 02 00 00        	movq	0x210(%rsi), %rax
1412d91f3: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
1412d91f7: 48 85 c0                    	testq	%rax, %rax
1412d91fa: 74 47                       	je	0x1412d9243 <.text+0x12d8243>
1412d91fc: 48 8b 8e e0 01 00 00        	movq	0x1e0(%rsi), %rcx
1412d9203: 48 f7 d0                    	notq	%rax
1412d9206: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1412d920a: 48 c1 e0 05                 	shlq	$0x5, %rax
1412d920e: 48 83 7c 01 20 00           	cmpq	$0x0, 0x20(%rcx,%rax)
1412d9214: 74 2d                       	je	0x1412d9243 <.text+0x12d8243>
1412d9216: 48 01 c1                    	addq	%rax, %rcx
1412d9219: 31 d2                       	xorl	%edx, %edx
1412d921b: e8 f0 64 a2 ff              	callq	0x140cff710 <.text+0xcfe710>
1412d9220: 0f 10 00                    	movups	(%rax), %xmm0
1412d9223: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
1412d9227: 0f 29 44 24 20              	movaps	%xmm0, 0x20(%rsp)
1412d922c: 0f 29 4c 24 30              	movaps	%xmm1, 0x30(%rsp)
1412d9231: 48 83 7c 24 20 02           	cmpq	$0x2, 0x20(%rsp)
1412d9237: 74 0a                       	je	0x1412d9243 <.text+0x12d8243>
1412d9239: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
1412d923e: e8 3d da 5b ff              	callq	0x140896c80 <.text+0x895c80>
1412d9243: 48 89 f1                    	movq	%rsi, %rcx
1412d9246: e8 65 ef a1 ff              	callq	0x140cf81b0 <.text+0xcf71b0>
1412d924b: 48 89 f1                    	movq	%rsi, %rcx
1412d924e: e8 fd ef a1 ff              	callq	0x140cf8250 <.text+0xcf7250>
1412d9253: 48 89 f1                    	movq	%rsi, %rcx
1412d9256: 48 83 c4 40                 	addq	$0x40, %rsp
1412d925a: 5e                          	popq	%rsi
1412d925b: e9 a0 ef a1 ff              	jmp	0x140cf8200 <.text+0xcf7200>
1412d9260: 90                          	nop
1412d9261: 48 83 c4 40                 	addq	$0x40, %rsp
1412d9265: 5e                          	popq	%rsi
1412d9266: c3                          	retq
