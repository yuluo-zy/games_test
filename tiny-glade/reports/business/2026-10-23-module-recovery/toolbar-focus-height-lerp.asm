
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

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
