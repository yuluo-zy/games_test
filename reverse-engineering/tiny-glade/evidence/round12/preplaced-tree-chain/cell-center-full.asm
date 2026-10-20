
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa5c70:     	pushq	%rsi
140aa5c71:     	pushq	%rdi
140aa5c72:     	subq	$0x78, %rsp
140aa5c76:     	movaps	%xmm10, 0x60(%rsp)
140aa5c7c:     	movaps	%xmm9, 0x50(%rsp)
140aa5c82:     	movaps	%xmm8, 0x40(%rsp)
140aa5c88:     	movaps	%xmm7, 0x30(%rsp)
140aa5c8d:     	movaps	%xmm6, 0x20(%rsp)
140aa5c92:     	movq	%rcx, %rsi
140aa5c95:     	movl	0x48(%rcx), %ecx
140aa5c98:     	testl	%ecx, %ecx
140aa5c9a:     	je	0x140aa5db0 <.text+0xaa4db0>
140aa5ca0:     	movl	%ecx, %eax
140aa5ca2:     	notl	%eax
140aa5ca4:     	leal	-0x80000000(%rdx), %r8d
140aa5cab:     	orl	%eax, %r8d
140aa5cae:     	je	0x140aa5dbc <.text+0xaa4dbc>
140aa5cb4:     	cvtsi2ss	%edx, %xmm0
140aa5cb8:     	cvtsi2ss	%ecx, %xmm1
140aa5cbc:     	movl	%edx, %eax
140aa5cbe:     	cltd
140aa5cbf:     	idivl	%ecx
140aa5cc1:     	movl	%edx, %edi
140aa5cc3:     	divss	%xmm1, %xmm0
140aa5cc7:     	callq	0x142923ee0 <.text+0x2922ee0>
140aa5ccc:     	cvttss2si	%xmm0, %eax
140aa5cd0:     	ucomiss	0x1efece5(%rip), %xmm0  # 0x1429a49bc
140aa5cd7:     	movl	$0x7fffffff, %ecx       # imm = 0x7FFFFFFF
140aa5cdc:     	cmovbel	%eax, %ecx
140aa5cdf:     	xorl	%eax, %eax
140aa5ce1:     	ucomiss	%xmm0, %xmm0
140aa5ce4:     	cmovnpl	%ecx, %eax
140aa5ce7:     	xorps	%xmm0, %xmm0
140aa5cea:     	cvtsi2ss	%edi, %xmm0
140aa5cee:     	movq	0x48(%rsi), %rcx
140aa5cf2:     	cvtsi2ss	%ecx, %xmm3
140aa5cf6:     	cvtsi2ss	%eax, %xmm4
140aa5cfa:     	shrq	$0x20, %rcx
140aa5cfe:     	cvtsi2ss	%ecx, %xmm5
140aa5d02:     	movss	0x50(%rsi), %xmm6
140aa5d07:     	movss	0x54(%rsi), %xmm1
140aa5d0c:     	movss	0x1e7fb5c(%rip), %xmm2  # 0x142925870
140aa5d14:     	movaps	%xmm6, %xmm7
140aa5d17:     	mulss	%xmm2, %xmm7
140aa5d1b:     	movaps	%xmm1, %xmm8
140aa5d1f:     	mulss	%xmm2, %xmm8
140aa5d24:     	unpcklps	%xmm0, %xmm6            # xmm6 = xmm6[0],xmm0[0],xmm6[1],xmm0[1]
140aa5d27:     	movsldup	%xmm3, %xmm0            # xmm0 = xmm3[0,0,2,2]
140aa5d2b:     	divps	%xmm0, %xmm6
140aa5d2e:     	movshdup	%xmm6, %xmm0            # xmm0 = xmm6[1,1,3,3]
140aa5d32:     	movss	0x1e7fbca(%rip), %xmm3  # 0x142925904
140aa5d3a:     	movaps	%xmm3, %xmm9
140aa5d3e:     	subss	%xmm0, %xmm9
140aa5d43:     	mulss	%xmm7, %xmm9
140aa5d48:     	movaps	%xmm2, %xmm10
140aa5d4c:     	unpcklps	%xmm7, %xmm10           # xmm10 = xmm10[0],xmm7[0],xmm10[1],xmm7[1]
140aa5d50:     	mulps	%xmm6, %xmm10
140aa5d54:     	movshdup	%xmm10, %xmm0           # xmm0 = xmm10[1,1,3,3]
140aa5d59:     	subss	%xmm9, %xmm0
140aa5d5e:     	unpcklps	%xmm4, %xmm1            # xmm1 = xmm1[0],xmm4[0],xmm1[1],xmm4[1]
140aa5d61:     	movsldup	%xmm5, %xmm4            # xmm4 = xmm5[0,0,2,2]
140aa5d65:     	divps	%xmm4, %xmm1
140aa5d68:     	movshdup	%xmm1, %xmm4            # xmm4 = xmm1[1,1,3,3]
140aa5d6c:     	subss	%xmm4, %xmm3
140aa5d70:     	mulss	%xmm8, %xmm3
140aa5d75:     	unpcklps	%xmm8, %xmm2            # xmm2 = xmm2[0],xmm8[0],xmm2[1],xmm8[1]
140aa5d79:     	mulps	%xmm1, %xmm2
140aa5d7c:     	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
140aa5d80:     	subss	%xmm3, %xmm1
140aa5d84:     	addss	%xmm10, %xmm0
140aa5d89:     	addss	%xmm2, %xmm1
140aa5d8d:     	movaps	0x20(%rsp), %xmm6
140aa5d92:     	movaps	0x30(%rsp), %xmm7
140aa5d97:     	movaps	0x40(%rsp), %xmm8
140aa5d9d:     	movaps	0x50(%rsp), %xmm9
140aa5da3:     	movaps	0x60(%rsp), %xmm10
140aa5da9:     	addq	$0x78, %rsp
140aa5dad:     	popq	%rdi
140aa5dae:     	popq	%rsi
140aa5daf:     	retq
140aa5db0:     	leaq	0x20370e1(%rip), %rcx   # 0x142adce98
140aa5db7:     	callq	0x1428d9a20 <.text+0x28d8a20>
140aa5dbc:     	leaq	0x20370d5(%rip), %rcx   # 0x142adce98
140aa5dc3:     	callq	0x1428d99a0 <.text+0x28d89a0>
140aa5dc8:     	int3
