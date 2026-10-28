
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a17ad2: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
140a17ad7: 83 c0 fe                    	addl	$-0x2, %eax
140a17ada: 83 f8 02                    	cmpl	$0x2, %eax
140a17add: b9 02 00 00 00              	movl	$0x2, %ecx
140a17ae2: 0f 42 c8                    	cmovbl	%eax, %ecx
140a17ae5: 83 f9 02                    	cmpl	$0x2, %ecx
140a17ae8: 89 95 40 03 00 00           	movl	%edx, 0x340(%rbp)
140a17aee: 74 11                       	je	0x140a17b01 <.text+0xa16b01>
140a17af0: 83 f9 01                    	cmpl	$0x1, %ecx
140a17af3: 75 6c                       	jne	0x140a17b61 <.text+0xa16b61>
140a17af5: b8 08 00 00 00              	movl	$0x8, %eax
140a17afa: b9 04 00 00 00              	movl	$0x4, %ecx
140a17aff: eb 0a                       	jmp	0x140a17b0b <.text+0xa16b0b>
140a17b01: b8 14 00 00 00              	movl	$0x14, %eax
140a17b06: b9 10 00 00 00              	movl	$0x10, %ecx
140a17b0b: 41 8b 14 24                 	movl	(%r12), %edx
140a17b0f: f3 48 0f 2a c2              	cvtsi2ss	%rdx, %xmm0
140a17b14: 41 8b 54 24 04              	movl	0x4(%r12), %edx
140a17b19: f3 48 0f 2a ca              	cvtsi2ss	%rdx, %xmm1
140a17b1e: f3 41 0f 10 14 0f           	movss	(%r15,%rcx), %xmm2
140a17b24: f3 0f 5e c1                 	divss	%xmm1, %xmm0
140a17b28: f3 0f 5e d0                 	divss	%xmm0, %xmm2
140a17b2c: f3 41 0f 10 04 07           	movss	(%r15,%rax), %xmm0
140a17b32: 0f 14 d0                    	unpcklps	%xmm0, %xmm2            # xmm2 = xmm2[0],xmm0[0],xmm2[1],xmm0[1]
140a17b35: 0f 59 15 a4 dd f0 01        	mulps	0x1f0dda4(%rip), %xmm2  # 0x1429258e0
140a17b3c: f3 0f 10 05 2c dd f0 01     	movss	0x1f0dd2c(%rip), %xmm0  # 0x142925870
140a17b44: 0f 58 c2                    	addps	%xmm2, %xmm0
140a17b47: 0f 28 3d 92 91 06 02        	movaps	0x2069192(%rip), %xmm7  # 0x142a80ce0
140a17b4e: 0f 5c fa                    	subps	%xmm2, %xmm7
140a17b51: 0f c6 f8 01                 	shufps	$0x1, %xmm0, %xmm7      # xmm7 = xmm7[1,0],xmm0[0,0]
140a17b55: 0f c6 f8 e2                 	shufps	$0xe2, %xmm0, %xmm7     # xmm7 = xmm7[2,0],xmm0[2,3]
140a17b59: 45 31 ed                    	xorl	%r13d, %r13d
140a17b5c: e9 9d 00 00 00              	jmp	0x140a17bfe <.text+0xa16bfe>
