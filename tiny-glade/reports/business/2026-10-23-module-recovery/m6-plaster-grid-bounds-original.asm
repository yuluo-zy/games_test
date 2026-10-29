141227aef: c1 e7 05                    	shll	$0x5, %edi
141227af2: 48 c7 85 58 01 00 00 00 00 00 00    	movq	$0x0, 0x158(%rbp)
141227afd: 4c 8b bd 48 03 00 00        	movq	0x348(%rbp), %r15
141227b04: f3 0f 11 bd b0 02 00 00     	movss	%xmm7, 0x2b0(%rbp)
141227b0c: eb 1a                       	jmp	0x141227b28 <.text+0x1226b28>
141227b0e: 66 90                       	nop
141227b10: 49 83 c7 10                 	addq	$0x10, %r15
141227b14: 48 ff 85 58 01 00 00        	incq	0x158(%rbp)
141227b1b: 4c 3b bd d0 00 00 00        	cmpq	0xd0(%rbp), %r15
141227b22: 0f 84 3a 05 00 00           	je	0x141228062 <.text+0x1227062>
141227b28: 48 8b 85 00 05 00 00        	movq	0x500(%rbp), %rax
141227b2f: f3 44 0f 10 80 80 02 00 00  	movss	0x280(%rax), %xmm8
141227b38: f2 41 0f 10 07              	movsd	(%r15), %xmm0
141227b3d: f2 45 0f 10 4f 08           	movsd	0x8(%r15), %xmm9
141227b43: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
141227b47: 45 0f 5c d1                 	subps	%xmm9, %xmm10
141227b4b: 44 0f 58 15 7d 6b 70 01     	addps	0x1706b7d(%rip), %xmm10 # 0x14292e6d0
141227b53: 0f 29 85 a0 03 00 00        	movaps	%xmm0, 0x3a0(%rbp)
141227b5a: 44 0f 58 c8                 	addps	%xmm0, %xmm9
141227b5e: 44 0f 58 0d 4a 6b 70 01     	addps	0x1706b4a(%rip), %xmm9  # 0x14292e6b0
141227b66: 44 0f 28 65 f0              	movaps	-0x10(%rbp), %xmm12
141227b6b: 45 0f 5c d4                 	subps	%xmm12, %xmm10
141227b6f: 44 0f 28 75 00              	movaps	(%rbp), %xmm14
141227b74: 45 0f 59 d6                 	mulps	%xmm14, %xmm10
141227b78: f3 41 0f 16 c2              	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
141227b7d: e8 de c2 6f 01              	callq	0x142923e60 <.text+0x2922e60>
141227b82: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
141227b86: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
141227b8a: e8 d1 c2 6f 01              	callq	0x142923e60 <.text+0x2922e60>
141227b8f: f3 0f 2c c0                 	cvttss2si	%xmm0, %eax
141227b93: f3 0f 10 3d 21 ce 77 01     	movss	0x177ce21(%rip), %xmm7  # 0x1429a49bc
141227b9b: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
141227b9e: bb ff ff ff 7f              	movl	$0x7fffffff, %ebx       # imm = 0x7FFFFFFF
141227ba3: 0f 47 c3                    	cmoval	%ebx, %eax
141227ba6: 0f 2e c0                    	ucomiss	%xmm0, %xmm0
141227ba9: be 00 00 00 00              	movl	$0x0, %esi
141227bae: 0f 4a c6                    	cmovpl	%esi, %eax
141227bb1: f3 41 0f 2c cb              	cvttss2si	%xmm11, %ecx
141227bb6: 44 0f 2e df                 	ucomiss	%xmm7, %xmm11
141227bba: 0f 47 cb                    	cmoval	%ebx, %ecx
141227bbd: 66 0f 6e c0                 	movd	%eax, %xmm0
141227bc1: 45 0f 2e db                 	ucomiss	%xmm11, %xmm11
141227bc5: 0f 4a ce                    	cmovpl	%esi, %ecx
141227bc8: 66 0f 6e c9                 	movd	%ecx, %xmm1
141227bcc: 66 0f 62 c1                 	punpckldq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
141227bd0: f3 0f 7e f0                 	movq	%xmm0, %xmm6            # xmm6 = xmm0[0],zero
141227bd4: 45 0f 5c cc                 	subps	%xmm12, %xmm9
141227bd8: 45 0f 59 ce                 	mulps	%xmm14, %xmm9
141227bdc: f3 41 0f 16 c1              	movshdup	%xmm9, %xmm0            # xmm0 = xmm9[1,1,3,3]
141227be1: e8 7a c2 6f 01              	callq	0x142923e60 <.text+0x2922e60>
141227be6: 44 0f 28 d0                 	movaps	%xmm0, %xmm10
141227bea: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
141227bee: e8 6d c2 6f 01              	callq	0x142923e60 <.text+0x2922e60>
141227bf3: f3 0f 2c c0                 	cvttss2si	%xmm0, %eax
141227bf7: 0f 2e c7                    	ucomiss	%xmm7, %xmm0
141227bfa: 0f 47 c3                    	cmoval	%ebx, %eax
141227bfd: 0f 2e c0                    	ucomiss	%xmm0, %xmm0
141227c00: 0f 4a c6                    	cmovpl	%esi, %eax
141227c03: f3 41 0f 2c ca              	cvttss2si	%xmm10, %ecx
141227c08: 44 0f 2e d7                 	ucomiss	%xmm7, %xmm10
141227c0c: f3 0f 10 bd b0 02 00 00     	movss	0x2b0(%rbp), %xmm7
141227c14: f3 44 0f 10 35 5b 54 7a 01  	movss	0x17a545b(%rip), %xmm14 # 0x1429cd078
141227c1d: 0f 47 cb                    	cmoval	%ebx, %ecx
141227c20: 45 0f 2e d2                 	ucomiss	%xmm10, %xmm10
141227c24: 66 0f 6e c0                 	movd	%eax, %xmm0
141227c28: 0f 4a ce                    	cmovpl	%esi, %ecx
141227c2b: 66 0f 6e c9                 	movd	%ecx, %xmm1
141227c2f: 66 0f 62 c1                 	punpckldq	%xmm1, %xmm0    # xmm0 = xmm0[0],xmm1[0],xmm0[1],xmm1[1]
141227c33: f3 0f 7e d0                 	movq	%xmm0, %xmm2            # xmm2 = xmm0[0],zero
141227c37: 66 0f 6f c6                 	movdqa	%xmm6, %xmm0
141227c3b: 66 41 0f 66 c5              	pcmpgtd	%xmm13, %xmm0
141227c40: 66 0f db c6                 	pand	%xmm6, %xmm0
141227c44: 66 0f 6f ca                 	movdqa	%xmm2, %xmm1
141227c48: 66 0f 6f 5d e0              	movdqa	-0x20(%rbp), %xmm3
141227c4d: 66 0f 66 cb                 	pcmpgtd	%xmm3, %xmm1
141227c51: 66 0f db d9                 	pand	%xmm1, %xmm3
141227c55: 66 0f df ca                 	pandn	%xmm2, %xmm1
141227c59: 66 0f eb cb                 	por	%xmm3, %xmm1
141227c5d: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
141227c61: 66 0f 66 d0                 	pcmpgtd	%xmm0, %xmm2
141227c65: 66 0f 70 d2 50              	pshufd	$0x50, %xmm2, %xmm2     # xmm2 = xmm2[0,0,1,1]
141227c6a: 66 0f 50 c2                 	movmskpd	%xmm2, %eax
141227c6e: 83 f8 03                    	cmpl	$0x3, %eax
141227c71: 44 0f 28 0d 07 6d 70 01     	movaps	0x1706d07(%rip), %xmm9  # 0x14292e980
141227c79: 0f 85 91 fe ff ff           	jne	0x141227b10 <.text+0x1226b10>
141227c7f: 48 8b 85 58 01 00 00        	movq	0x158(%rbp), %rax
