
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b2a890: 56                          	pushq	%rsi
140b2a891: 48 83 ec 60                 	subq	$0x60, %rsp
140b2a895: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140b2a89a: f6 81 08 01 00 00 01        	testb	$0x1, 0x108(%rcx)
140b2a8a1: 0f 84 2c 01 00 00           	je	0x140b2a9d3 <.text+0xb299d3>
140b2a8a7: 48 89 ce                    	movq	%rcx, %rsi
140b2a8aa: 83 b9 58 02 00 00 02        	cmpl	$0x2, 0x258(%rcx)
140b2a8b1: 73 0d                       	jae	0x140b2a8c0 <.text+0xb298c0>
140b2a8b3: f3 0f 10 86 0c 01 00 00     	movss	0x10c(%rsi), %xmm0
140b2a8bb: e9 d8 00 00 00              	jmp	0x140b2a998 <.text+0xb29998>
140b2a8c0: f3 0f 59 4e 30              	mulss	0x30(%rsi), %xmm1
140b2a8c5: 0f 2e c9                    	ucomiss	%xmm1, %xmm1
140b2a8c8: 0f 8a d5 00 00 00           	jp	0x140b2a9a3 <.text+0xb299a3>
140b2a8ce: 66 0f 7e c8                 	movd	%xmm1, %eax
140b2a8d2: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
140b2a8d7: 3d 00 00 80 7f              	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
140b2a8dc: 0f 84 d9 00 00 00           	je	0x140b2a9bb <.text+0xb299bb>
140b2a8e2: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b2a8e5: f3 0f 5f c1                 	maxss	%xmm1, %xmm0
140b2a8e9: f3 0f 10 35 13 b0 df 01     	movss	0x1dfb013(%rip), %xmm6  # 0x142925904
140b2a8f1: 0f 28 ce                    	movaps	%xmm6, %xmm1
140b2a8f4: f3 0f 5d c8                 	minss	%xmm0, %xmm1
140b2a8f8: 48 89 f1                    	movq	%rsi, %rcx
140b2a8fb: e8 e0 d3 93 00              	callq	0x141467ce0 <.text+0x1466ce0>
140b2a900: 89 c1                       	movl	%eax, %ecx
140b2a902: 48 8b 56 10                 	movq	0x10(%rsi), %rdx
140b2a906: 48 39 ca                    	cmpq	%rcx, %rdx
140b2a909: 0f 86 fb 00 00 00           	jbe	0x140b2aa0a <.text+0xb29a0a>
140b2a90f: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
140b2a913: 48 39 d0                    	cmpq	%rdx, %rax
140b2a916: 0f 83 fa 00 00 00           	jae	0x140b2aa16 <.text+0xb29a16>
140b2a91c: 48 8b 46 08                 	movq	0x8(%rsi), %rax
140b2a920: f3 0f 5c f0                 	subss	%xmm0, %xmm6
140b2a924: f3 0f 10 14 c8              	movss	(%rax,%rcx,8), %xmm2
140b2a929: f3 0f 59 d6                 	mulss	%xmm6, %xmm2
140b2a92d: f3 0f 59 74 c8 04           	mulss	0x4(%rax,%rcx,8), %xmm6
140b2a933: f3 0f 10 4c c8 08           	movss	0x8(%rax,%rcx,8), %xmm1
140b2a939: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
140b2a93d: f3 0f 58 ca                 	addss	%xmm2, %xmm1
140b2a941: f3 0f 10 54 c8 0c           	movss	0xc(%rax,%rcx,8), %xmm2
140b2a947: f3 0f 59 d0                 	mulss	%xmm0, %xmm2
140b2a94b: f3 0f 58 d6                 	addss	%xmm6, %xmm2
140b2a94f: 48 8d 8e 88 00 00 00        	leaq	0x88(%rsi), %rcx
140b2a956: e8 a5 f4 93 00              	callq	0x141469e00 <.text+0x1468e00>
140b2a95b: 48 8b 96 e8 00 00 00        	movq	0xe8(%rsi), %rdx
140b2a962: 89 c1                       	movl	%eax, %ecx
140b2a964: 48 39 ca                    	cmpq	%rcx, %rdx
140b2a967: 0f 86 b8 00 00 00           	jbe	0x140b2aa25 <.text+0xb29a25>
140b2a96d: 48 8d 41 01                 	leaq	0x1(%rcx), %rax
140b2a971: 48 39 d0                    	cmpq	%rdx, %rax
140b2a974: 0f 83 b7 00 00 00           	jae	0x140b2aa31 <.text+0xb29a31>
140b2a97a: 48 8b 86 e0 00 00 00        	movq	0xe0(%rsi), %rax
140b2a981: f3 0f 10 0c 88              	movss	(%rax,%rcx,4), %xmm1
140b2a986: f3 0f 10 54 88 04           	movss	0x4(%rax,%rcx,4), %xmm2
140b2a98c: f3 0f 5c d1                 	subss	%xmm1, %xmm2
140b2a990: f3 0f 59 c2                 	mulss	%xmm2, %xmm0
140b2a994: f3 0f 58 c1                 	addss	%xmm1, %xmm0
140b2a998: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140b2a99d: 48 83 c4 60                 	addq	$0x60, %rsp
140b2a9a1: 5e                          	popq	%rsi
140b2a9a2: c3                          	retq
140b2a9a3: 48 8d 0d 26 aa f7 01        	leaq	0x1f7aa26(%rip), %rcx   # 0x142aa53d0
140b2a9aa: 4c 8d 05 e7 05 fc 01        	leaq	0x1fc05e7(%rip), %r8    # 0x142aeaf98
140b2a9b1: ba 1d 00 00 00              	movl	$0x1d, %edx
140b2a9b6: e8 75 ea da 01              	callq	0x1428d9430 <.text+0x28d8430>
140b2a9bb: 48 8d 0d d6 aa f7 01        	leaq	0x1f7aad6(%rip), %rcx   # 0x142aa5498
140b2a9c2: 4c 8d 05 cf 05 fc 01        	leaq	0x1fc05cf(%rip), %r8    # 0x142aeaf98
140b2a9c9: ba 22 00 00 00              	movl	$0x22, %edx
140b2a9ce: e8 5d ea da 01              	callq	0x1428d9430 <.text+0x28d8430>
140b2a9d3: 48 8d 05 8e 06 fc 01        	leaq	0x1fc068e(%rip), %rax   # 0x142aeb068
140b2a9da: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140b2a9df: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140b2a9e8: 48 c7 44 24 30 08 00 00 00  	movq	$0x8, 0x30(%rsp)
140b2a9f1: 0f 57 c0                    	xorps	%xmm0, %xmm0
140b2a9f4: 0f 11 44 24 38              	movups	%xmm0, 0x38(%rsp)
140b2a9f9: 48 8d 15 78 06 fc 01        	leaq	0x1fc0678(%rip), %rdx   # 0x142aeb078
140b2aa00: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140b2aa05: e8 86 e9 da 01              	callq	0x1428d9390 <.text+0x28d8390>
140b2aa0a: 4c 8d 05 47 8c 13 02        	leaq	0x2138c47(%rip), %r8    # 0x142c63658
140b2aa11: e8 02 eb da 01              	callq	0x1428d9518 <.text+0x28d8518>
140b2aa16: 4c 8d 05 53 8c 13 02        	leaq	0x2138c53(%rip), %r8    # 0x142c63670
140b2aa1d: 48 89 c1                    	movq	%rax, %rcx
140b2aa20: e8 f3 ea da 01              	callq	0x1428d9518 <.text+0x28d8518>
140b2aa25: 4c 8d 05 84 05 fc 01        	leaq	0x1fc0584(%rip), %r8    # 0x142aeafb0
140b2aa2c: e8 e7 ea da 01              	callq	0x1428d9518 <.text+0x28d8518>
140b2aa31: 4c 8d 05 78 05 fc 01        	leaq	0x1fc0578(%rip), %r8    # 0x142aeafb0
140b2aa38: 48 89 c1                    	movq	%rax, %rcx
140b2aa3b: e8 d8 ea da 01              	callq	0x1428d9518 <.text+0x28d8518>
