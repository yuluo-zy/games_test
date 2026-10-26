
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c90600: 56                          	pushq	%rsi
140c90601: 57                          	pushq	%rdi
140c90602: 53                          	pushq	%rbx
140c90603: 48 83 ec 20                 	subq	$0x20, %rsp
140c90607: 0f 28 c3                    	movaps	%xmm3, %xmm0
140c9060a: 4c 89 c7                    	movq	%r8, %rdi
140c9060d: 48 89 d3                    	movq	%rdx, %rbx
140c90610: 48 89 ce                    	movq	%rcx, %rsi
140c90613: 0f 57 05 d6 52 c9 01        	xorps	0x1c952d6(%rip), %xmm0  # 0x1429258f0
140c9061a: f3 0f 59 44 24 60           	mulss	0x60(%rsp), %xmm0
140c90620: e8 9b 38 c9 01              	callq	0x142923ec0 <.text+0x2922ec0>
140c90625: f3 0f 10 0d d7 52 c9 01     	movss	0x1c952d7(%rip), %xmm1  # 0x142925904
140c9062d: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140c90631: f3 0f 10 43 08              	movss	0x8(%rbx), %xmm0
140c90636: f3 0f 10 57 08              	movss	0x8(%rdi), %xmm2
140c9063b: f3 0f 5c d0                 	subss	%xmm0, %xmm2
140c9063f: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
140c90643: f3 0f 58 d0                 	addss	%xmm0, %xmm2
140c90647: f2 0f 10 03                 	movsd	(%rbx), %xmm0
140c9064b: f2 0f 10 1f                 	movsd	(%rdi), %xmm3
140c9064f: 0f 5c d8                    	subps	%xmm0, %xmm3
140c90652: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
140c90656: 0f 59 cb                    	mulps	%xmm3, %xmm1
140c90659: 0f 58 c8                    	addps	%xmm0, %xmm1
140c9065c: 0f 13 0e                    	movlps	%xmm1, (%rsi)
140c9065f: f3 0f 11 56 08              	movss	%xmm2, 0x8(%rsi)
140c90664: 48 89 f0                    	movq	%rsi, %rax
140c90667: 48 83 c4 20                 	addq	$0x20, %rsp
140c9066b: 5b                          	popq	%rbx
140c9066c: 5f                          	popq	%rdi
140c9066d: 5e                          	popq	%rsi
140c9066e: c3                          	retq
140c9066f: cc                          	int3
140c90670: 56                          	pushq	%rsi
140c90671: 48 83 ec 40                 	subq	$0x40, %rsp
140c90675: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140c9067a: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
140c9067f: 0f 28 c2                    	movaps	%xmm2, %xmm0
140c90682: 0f 28 f1                    	movaps	%xmm1, %xmm6
140c90685: 48 89 ce                    	movq	%rcx, %rsi
140c90688: f3 0f 10 39                 	movss	(%rcx), %xmm7
140c9068c: 0f 57 05 5d 52 c9 01        	xorps	0x1c9525d(%rip), %xmm0  # 0x1429258f0
140c90693: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
140c90697: e8 24 38 c9 01              	callq	0x142923ec0 <.text+0x2922ec0>
140c9069c: f3 0f 10 0d 60 52 c9 01     	movss	0x1c95260(%rip), %xmm1  # 0x142925904
140c906a4: f3 0f 5c c8                 	subss	%xmm0, %xmm1
140c906a8: f3 0f 5c f7                 	subss	%xmm7, %xmm6
140c906ac: f3 0f 59 f1                 	mulss	%xmm1, %xmm6
140c906b0: f3 0f 58 f7                 	addss	%xmm7, %xmm6
140c906b4: f3 0f 11 36                 	movss	%xmm6, (%rsi)
140c906b8: 0f 28 c6                    	movaps	%xmm6, %xmm0
140c906bb: 0f 28 74 24 20              	movaps	0x20(%rsp), %xmm6
140c906c0: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140c906c5: 48 83 c4 40                 	addq	$0x40, %rsp
140c906c9: 5e                          	popq	%rsi
140c906ca: c3                          	retq
140c906cb: cc                          	int3
140c906cc: cc                          	int3
140c906cd: cc                          	int3
140c906ce: cc                          	int3
140c906cf: cc                          	int3
140c906d0: f3 0f 10 2d 2c 52 c9 01     	movss	0x1c9522c(%rip), %xmm5  # 0x142925904
140c906d8: 0f 28 e5                    	movaps	%xmm5, %xmm4
140c906db: f3 0f 5c e1                 	subss	%xmm1, %xmm4
140c906df: f3 0f 5e c2                 	divss	%xmm2, %xmm0
140c906e3: f3 0f 5e e3                 	divss	%xmm3, %xmm4
140c906e7: f3 0f 58 c0                 	addss	%xmm0, %xmm0
140c906eb: f3 0f 58 05 f1 99 cc 01     	addss	0x1cc99f1(%rip), %xmm0  # 0x14295a0e4
140c906f3: f3 0f 58 e4                 	addss	%xmm4, %xmm4
140c906f7: f3 0f 58 e5                 	addss	%xmm5, %xmm4
140c906fb: 0f 28 cc                    	movaps	%xmm4, %xmm1
140c906fe: c3                          	retq
140c906ff: cc                          	int3
140c90700: 48 83 ec 58                 	subq	$0x58, %rsp
140c90704: 44 0f 29 44 24 40           	movaps	%xmm8, 0x40(%rsp)
140c9070a: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140c9070f: 0f 29 74 24 20              	movaps	%xmm6, 0x20(%rsp)
