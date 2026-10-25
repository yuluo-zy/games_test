
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140c909a0: 48 83 ec 28                 	subq	$0x28, %rsp
140c909a4: 85 c9                       	testl	%ecx, %ecx
140c909a6: 74 7d                       	je	0x140c90a25 <.text+0xc8fa25>
140c909a8: 41 89 d0                    	movl	%edx, %r8d
140c909ab: 85 d2                       	testl	%edx, %edx
140c909ad: 74 7e                       	je	0x140c90a2d <.text+0xc8fa2d>
140c909af: 44 89 c0                    	movl	%r8d, %eax
140c909b2: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
140c909b7: f3 0f 10 0d 45 4f c9 01     	movss	0x1c94f45(%rip), %xmm1  # 0x142925904
140c909bf: f3 0f 5e c8                 	divss	%xmm0, %xmm1
140c909c3: 0f 57 d2                    	xorps	%xmm2, %xmm2
140c909c6: 45 31 c9                    	xorl	%r9d, %r9d
140c909c9: f3 0f 10 1d ff fd d0 01     	movss	0x1d0fdff(%rip), %xmm3  # 0x1429a07d0
140c909d1: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
140c909d7: 0f 57 c0                    	xorps	%xmm0, %xmm0
140c909da: 0f 28 e1                    	movaps	%xmm1, %xmm4
140c909dd: 0f 1f 00                    	nopl	(%rax)
140c909e0: 89 c8                       	movl	%ecx, %eax
140c909e2: 31 d2                       	xorl	%edx, %edx
140c909e4: 41 f7 f0                    	divl	%r8d
140c909e7: 89 c8                       	movl	%ecx, %eax
140c909e9: 0f 57 ed                    	xorps	%xmm5, %xmm5
140c909ec: f3 48 0f 2a e8              	cvtsi2ss	%rax, %xmm5
140c909f1: f3 0f 59 e9                 	mulss	%xmm1, %xmm5
140c909f5: f3 48 0f 2c cd              	cvttss2si	%xmm5, %rcx
140c909fa: 0f 2e ea                    	ucomiss	%xmm2, %xmm5
140c909fd: 41 0f 42 c9                 	cmovbl	%r9d, %ecx
140c90a01: 0f 2e eb                    	ucomiss	%xmm3, %xmm5
140c90a04: 0f 57 ed                    	xorps	%xmm5, %xmm5
140c90a07: f3 48 0f 2a ea              	cvtsi2ss	%rdx, %xmm5
140c90a0c: 41 0f 47 ca                 	cmoval	%r10d, %ecx
140c90a10: f3 0f 59 ec                 	mulss	%xmm4, %xmm5
140c90a14: f3 0f 58 c5                 	addss	%xmm5, %xmm0
140c90a18: f3 0f 59 e1                 	mulss	%xmm1, %xmm4
140c90a1c: 85 c9                       	testl	%ecx, %ecx
140c90a1e: 75 c0                       	jne	0x140c909e0 <.text+0xc8f9e0>
140c90a20: 48 83 c4 28                 	addq	$0x28, %rsp
140c90a24: c3                          	retq
140c90a25: 0f 57 c0                    	xorps	%xmm0, %xmm0
140c90a28: 48 83 c4 28                 	addq	$0x28, %rsp
140c90a2c: c3                          	retq
140c90a2d: 48 8d 0d 84 ce e9 01        	leaq	0x1e9ce84(%rip), %rcx   # 0x142b2d8b8
140c90a34: e8 e7 8f c4 01              	callq	0x1428d9a20 <.text+0x28d8a20>
140c90a39: cc                          	int3
