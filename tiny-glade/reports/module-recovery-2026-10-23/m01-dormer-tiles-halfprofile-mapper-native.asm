
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1412ac9b0: 56                          	pushq	%rsi
1412ac9b1: 57                          	pushq	%rdi
1412ac9b2: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1412ac9b9: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1412ac9c2: 44 0f 29 4c 24 70           	movaps	%xmm9, 0x70(%rsp)
1412ac9c8: 44 0f 29 44 24 60           	movaps	%xmm8, 0x60(%rsp)
1412ac9ce: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
1412ac9d3: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
1412ac9d8: 48 89 d7                    	movq	%rdx, %rdi
1412ac9db: 48 89 ce                    	movq	%rcx, %rsi
1412ac9de: f3 41 0f 10 30              	movss	(%r8), %xmm6
1412ac9e3: f3 45 0f 10 40 04           	movss	0x4(%r8), %xmm8
1412ac9e9: 48 8b 12                    	movq	(%rdx), %rdx
1412ac9ec: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1412ac9f1: e8 da dd 87 ff              	callq	0x140b2a7d0 <.text+0xb297d0>
1412ac9f6: 83 7c 24 28 00              	cmpl	$0x0, 0x28(%rsp)
1412ac9fb: f3 0f 10 3d c5 e6 97 01     	movss	0x197e6c5(%rip), %xmm7  # 0x142c2b0c8
1412aca03: 74 29                       	je	0x1412aca2e <.text+0x12aba2e>
1412aca05: 48 8b 47 08                 	movq	0x8(%rdi), %rax
1412aca09: 80 38 07                    	cmpb	$0x7, (%rax)
1412aca0c: 75 20                       	jne	0x1412aca2e <.text+0x12aba2e>
1412aca0e: 48 8b 47 10                 	movq	0x10(%rdi), %rax
1412aca12: 0f b6 00                    	movzbl	(%rax), %eax
1412aca15: fe c8                       	decb	%al
1412aca17: 3c 03                       	cmpb	$0x3, %al
1412aca19: 0f 83 e9 01 00 00           	jae	0x1412acc08 <.text+0x12abc08>
1412aca1f: 0f b6 c0                    	movzbl	%al, %eax
1412aca22: 48 8d 0d 57 e7 97 01        	leaq	0x197e757(%rip), %rcx   # 0x142c2b180
1412aca29: f3 0f 10 3c 81              	movss	(%rcx,%rax,4), %xmm7
1412aca2e: 48 8b 57 18                 	movq	0x18(%rdi), %rdx
1412aca32: 48 8b 4f 20                 	movq	0x20(%rdi), %rcx
1412aca36: 48 8b 41 10                 	movq	0x10(%rcx), %rax
1412aca3a: 48 85 c0                    	testq	%rax, %rax
1412aca3d: 41 0f 94 c0                 	sete	%r8b
1412aca41: 48 c1 e0 03                 	shlq	$0x3, %rax
1412aca45: 48 03 41 08                 	addq	0x8(%rcx), %rax
1412aca49: 48 83 c0 f8                 	addq	$-0x8, %rax
1412aca4d: 0f 94 c1                    	sete	%cl
1412aca50: 44 08 c1                    	orb	%r8b, %cl
1412aca53: 83 3a 00                    	cmpl	$0x0, (%rdx)
1412aca56: 74 7f                       	je	0x1412acad7 <.text+0x12abad7>
1412aca58: 84 c9                       	testb	%cl, %cl
1412aca5a: 0f 85 90 01 00 00           	jne	0x1412acbf0 <.text+0x12abbf0>
1412aca60: f3 0f 10 08                 	movss	(%rax), %xmm1
1412aca64: 0f 57 c0                    	xorps	%xmm0, %xmm0
1412aca67: 0f 28 d6                    	movaps	%xmm6, %xmm2
1412aca6a: e8 31 3a 9e ff              	callq	0x140c904a0 <.text+0xc8f4a0>
1412aca6f: 0f 28 f0                    	movaps	%xmm0, %xmm6
1412aca72: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1412aca76: f3 0f 10 05 86 8e 67 01     	movss	0x1678e86(%rip), %xmm0  # 0x142925904
1412aca7e: 0f 28 c8                    	movaps	%xmm0, %xmm1
1412aca81: f3 0f 5c ce                 	subss	%xmm6, %xmm1
1412aca85: f3 44 0f 10 08              	movss	(%rax), %xmm9
1412aca8a: f3 44 0f 59 c9              	mulss	%xmm1, %xmm9
1412aca8f: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
1412aca94: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
1412aca98: f3 0f 12 d9                 	movsldup	%xmm1, %xmm3            # xmm3 = xmm1[0,0,2,2]
1412aca9c: f3 0f 59 48 08              	mulss	0x8(%rax), %xmm1
1412acaa1: 48 8b 47 48                 	movq	0x48(%rdi), %rax
1412acaa5: f3 44 0f 10 10              	movss	(%rax), %xmm10
1412acaaa: f3 44 0f 59 d6              	mulss	%xmm6, %xmm10
1412acaaf: f3 0f 10 60 04              	movss	0x4(%rax), %xmm4
1412acab4: f3 0f 59 e6                 	mulss	%xmm6, %xmm4
1412acab8: f3 0f 12 ee                 	movsldup	%xmm6, %xmm5            # xmm5 = xmm6[0,0,2,2]
1412acabc: f3 0f 59 70 08              	mulss	0x8(%rax), %xmm6
1412acac1: f3 0f 58 f1                 	addss	%xmm1, %xmm6
1412acac5: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1412acac9: 44 0f 14 cc                 	unpcklps	%xmm4, %xmm9            # xmm9 = xmm9[0],xmm4[0],xmm9[1],xmm4[1]
1412acacd: 48 8b 47 38                 	movq	0x38(%rdi), %rax
1412acad1: 48 8b 4f 50                 	movq	0x50(%rdi), %rcx
1412acad5: eb 7d                       	jmp	0x1412acb54 <.text+0x12abb54>
1412acad7: 84 c9                       	testb	%cl, %cl
1412acad9: 0f 85 1d 01 00 00           	jne	0x1412acbfc <.text+0x12abbfc>
1412acadf: f3 0f 10 08                 	movss	(%rax), %xmm1
1412acae3: 0f 57 c0                    	xorps	%xmm0, %xmm0
1412acae6: 0f 28 d6                    	movaps	%xmm6, %xmm2
1412acae9: e8 b2 39 9e ff              	callq	0x140c904a0 <.text+0xc8f4a0>
1412acaee: 0f 28 f0                    	movaps	%xmm0, %xmm6
1412acaf1: 48 8b 47 28                 	movq	0x28(%rdi), %rax
1412acaf5: f3 0f 10 05 07 8e 67 01     	movss	0x1678e07(%rip), %xmm0  # 0x142925904
1412acafd: 0f 28 c8                    	movaps	%xmm0, %xmm1
1412acb00: f3 0f 5c ce                 	subss	%xmm6, %xmm1
1412acb04: f3 44 0f 10 08              	movss	(%rax), %xmm9
1412acb09: f3 44 0f 59 c9              	mulss	%xmm1, %xmm9
1412acb0e: f3 0f 10 50 04              	movss	0x4(%rax), %xmm2
1412acb13: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
1412acb17: f3 0f 12 d9                 	movsldup	%xmm1, %xmm3            # xmm3 = xmm1[0,0,2,2]
1412acb1b: f3 0f 59 48 08              	mulss	0x8(%rax), %xmm1
1412acb20: 48 8b 47 30                 	movq	0x30(%rdi), %rax
1412acb24: f3 44 0f 10 10              	movss	(%rax), %xmm10
1412acb29: f3 44 0f 59 d6              	mulss	%xmm6, %xmm10
1412acb2e: f3 0f 10 60 04              	movss	0x4(%rax), %xmm4
1412acb33: f3 0f 59 e6                 	mulss	%xmm6, %xmm4
1412acb37: f3 0f 12 ee                 	movsldup	%xmm6, %xmm5            # xmm5 = xmm6[0,0,2,2]
1412acb3b: f3 0f 59 70 08              	mulss	0x8(%rax), %xmm6
1412acb40: f3 0f 58 f1                 	addss	%xmm1, %xmm6
1412acb44: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1412acb48: 44 0f 14 cc                 	unpcklps	%xmm4, %xmm9            # xmm9 = xmm9[0],xmm4[0],xmm9[1],xmm4[1]
1412acb4c: 48 8b 47 38                 	movq	0x38(%rdi), %rax
1412acb50: 48 8b 4f 40                 	movq	0x40(%rdi), %rcx
1412acb54: f2 0f 10 08                 	movsd	(%rax), %xmm1
1412acb58: f2 0f 10 11                 	movsd	(%rcx), %xmm2
1412acb5c: 0f 59 cb                    	mulps	%xmm3, %xmm1
1412acb5f: 0f 59 d5                    	mulps	%xmm5, %xmm2
1412acb62: 0f 58 d1                    	addps	%xmm1, %xmm2
1412acb65: 0f 28 ca                    	movaps	%xmm2, %xmm1
1412acb68: 0f 59 ca                    	mulps	%xmm2, %xmm1
1412acb6b: f3 0f 16 d9                 	movshdup	%xmm1, %xmm3            # xmm3 = xmm1[1,1,3,3]
1412acb6f: f3 0f 58 d9                 	addss	%xmm1, %xmm3
1412acb73: 0f 57 c9                    	xorps	%xmm1, %xmm1
1412acb76: f3 0f 51 cb                 	sqrtss	%xmm3, %xmm1
1412acb7a: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1412acb7e: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1412acb82: 0f 59 c2                    	mulps	%xmm2, %xmm0
1412acb85: 0f 13 44 24 20              	movlps	%xmm0, 0x20(%rsp)
1412acb8a: 48 8d 4c 24 28              	leaq	0x28(%rsp), %rcx
1412acb8f: 48 8d 54 24 20              	leaq	0x20(%rsp), %rdx
1412acb94: e8 27 02 9f ff              	callq	0x140c9cdc0 <.text+0xc9bdc0>
1412acb99: 45 0f 14 d0                 	unpcklps	%xmm8, %xmm10           # xmm10 = xmm10[0],xmm8[0],xmm10[1],xmm8[1]
1412acb9d: 45 0f 58 d1                 	addps	%xmm9, %xmm10
1412acba1: f2 0f 10 4c 24 28           	movsd	0x28(%rsp), %xmm1
1412acba7: f3 0f 12 c7                 	movsldup	%xmm7, %xmm0            # xmm0 = xmm7[0,0,2,2]
1412acbab: 0f 59 c1                    	mulps	%xmm1, %xmm0
1412acbae: 41 0f 58 c2                 	addps	%xmm10, %xmm0
1412acbb2: f3 0f 59 7c 24 30           	mulss	0x30(%rsp), %xmm7
1412acbb8: f3 0f 58 fe                 	addss	%xmm6, %xmm7
1412acbbc: 0f 13 06                    	movlps	%xmm0, (%rsi)
1412acbbf: f3 0f 11 7e 08              	movss	%xmm7, 0x8(%rsi)
1412acbc4: 48 89 f0                    	movq	%rsi, %rax
1412acbc7: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
1412acbcc: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
1412acbd1: 44 0f 28 44 24 60           	movaps	0x60(%rsp), %xmm8
1412acbd7: 44 0f 28 4c 24 70           	movaps	0x70(%rsp), %xmm9
1412acbdd: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1412acbe6: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1412acbed: 5f                          	popq	%rdi
1412acbee: 5e                          	popq	%rsi
1412acbef: c3                          	retq
1412acbf0: 48 8d 0d 71 e5 97 01        	leaq	0x197e571(%rip), %rcx   # 0x142c2b168
1412acbf7: e8 14 c7 62 01              	callq	0x1428d9310 <.text+0x28d8310>
1412acbfc: 48 8d 0d 4d e5 97 01        	leaq	0x197e54d(%rip), %rcx   # 0x142c2b150
1412acc03: e8 08 c7 62 01              	callq	0x1428d9310 <.text+0x28d8310>
1412acc08: 48 8d 0d bd e4 97 01        	leaq	0x197e4bd(%rip), %rcx   # 0x142c2b0cc
1412acc0f: 4c 8d 05 22 e5 97 01        	leaq	0x197e522(%rip), %r8    # 0x142c2b138
1412acc16: ba 28 00 00 00              	movl	$0x28, %edx
1412acc1b: e8 10 c8 62 01              	callq	0x1428d9430 <.text+0x28d8430>
1412acc20: cc                          	int3
