
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c90ed0: 48 83 ec 18                 	subq	$0x18, %rsp
140c90ed4: 0f 29 34 24                 	movaps	%xmm6, (%rsp)
140c90ed8: 48 89 c8                    	movq	%rcx, %rax
140c90edb: 0f 28 c1                    	movaps	%xmm1, %xmm0
140c90ede: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
140c90ee2: 0f 28 da                    	movaps	%xmm2, %xmm3
140c90ee5: f3 0f 59 da                 	mulss	%xmm2, %xmm3
140c90ee9: f3 0f 58 d8                 	addss	%xmm0, %xmm3
140c90eed: f3 0f 51 db                 	sqrtss	%xmm3, %xmm3
140c90ef1: f3 0f 10 05 0b 4a c9 01     	movss	0x1c94a0b(%rip), %xmm0  # 0x142925904
140c90ef9: 0f 28 e0                    	movaps	%xmm0, %xmm4
140c90efc: f3 0f 5e e3                 	divss	%xmm3, %xmm4
140c90f00: 66 0f 7e e1                 	movd	%xmm4, %ecx
140c90f04: 85 c9                       	testl	%ecx, %ecx
140c90f06: 0f 99 c2                    	setns	%dl
140c90f09: 41 89 c8                    	movl	%ecx, %r8d
140c90f0c: 41 81 e0 ff ff ff 7f        	andl	$0x7fffffff, %r8d       # imm = 0x7FFFFFFF
140c90f13: 41 81 c0 00 00 80 ff        	addl	$0xff800000, %r8d       # imm = 0xFF800000
140c90f1a: 41 81 f8 00 00 00 7f        	cmpl	$0x7f000000, %r8d       # imm = 0x7F000000
140c90f21: 41 0f 92 c0                 	setb	%r8b
140c90f25: 41 20 d0                    	andb	%dl, %r8b
140c90f28: ff c9                       	decl	%ecx
140c90f2a: 81 f9 ff ff 7f 00           	cmpl	$0x7fffff, %ecx         # imm = 0x7FFFFF
140c90f30: 0f 92 c1                    	setb	%cl
140c90f33: 44 08 c1                    	orb	%r8b, %cl
140c90f36: 0f 57 db                    	xorps	%xmm3, %xmm3
140c90f39: 75 08                       	jne	0x140c90f43 <.text+0xc8ff43>
140c90f3b: 0f 57 d2                    	xorps	%xmm2, %xmm2
140c90f3e: 0f 28 c8                    	movaps	%xmm0, %xmm1
140c90f41: eb 08                       	jmp	0x140c90f4b <.text+0xc8ff4b>
140c90f43: f3 0f 59 cc                 	mulss	%xmm4, %xmm1
140c90f47: f3 0f 59 d4                 	mulss	%xmm4, %xmm2
140c90f4b: 0f 28 e1                    	movaps	%xmm1, %xmm4
140c90f4e: f3 0f 59 e3                 	mulss	%xmm3, %xmm4
140c90f52: 0f 28 ea                    	movaps	%xmm2, %xmm5
140c90f55: f3 0f 59 eb                 	mulss	%xmm3, %xmm5
140c90f59: f3 0f 5c e5                 	subss	%xmm5, %xmm4
140c90f5d: f3 0f 5c d9                 	subss	%xmm1, %xmm3
140c90f61: 0f 28 ea                    	movaps	%xmm2, %xmm5
140c90f64: f3 0f 59 ea                 	mulss	%xmm2, %xmm5
140c90f68: 0f 28 f2                    	movaps	%xmm2, %xmm6
140c90f6b: 0f 14 f4                    	unpcklps	%xmm4, %xmm6            # xmm6 = xmm6[0],xmm4[0],xmm6[1],xmm4[1]
140c90f6e: f3 0f 59 e4                 	mulss	%xmm4, %xmm4
140c90f72: f3 0f 58 e5                 	addss	%xmm5, %xmm4
140c90f76: 0f 28 eb                    	movaps	%xmm3, %xmm5
140c90f79: f3 0f 59 eb                 	mulss	%xmm3, %xmm5
140c90f7d: f3 0f 58 ec                 	addss	%xmm4, %xmm5
140c90f81: 0f 57 e4                    	xorps	%xmm4, %xmm4
140c90f84: f3 0f 51 e5                 	sqrtss	%xmm5, %xmm4
140c90f88: f3 0f 5e c4                 	divss	%xmm4, %xmm0
140c90f8c: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
140c90f90: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
140c90f94: 0f 59 c6                    	mulps	%xmm6, %xmm0
140c90f97: 0f 13 00                    	movlps	%xmm0, (%rax)
140c90f9a: f3 0f 11 58 08              	movss	%xmm3, 0x8(%rax)
140c90f9f: f2 0f 10 05 09 36 ca 01     	movsd	0x1ca3609(%rip), %xmm0  # 0x1429345b0
140c90fa7: f2 0f 11 40 0c              	movsd	%xmm0, 0xc(%rax)
140c90fac: c7 40 14 00 00 00 00        	movl	$0x0, 0x14(%rax)
140c90fb3: f3 0f 11 48 18              	movss	%xmm1, 0x18(%rax)
140c90fb8: c7 40 1c 00 00 00 00        	movl	$0x0, 0x1c(%rax)
140c90fbf: f3 0f 11 50 20              	movss	%xmm2, 0x20(%rax)
140c90fc4: 0f 28 34 24                 	movaps	(%rsp), %xmm6
140c90fc8: 48 83 c4 18                 	addq	$0x18, %rsp
140c90fcc: c3                          	retq
140c90fcd: cc                          	int3
140c90fce: cc                          	int3
140c90fcf: cc                          	int3
