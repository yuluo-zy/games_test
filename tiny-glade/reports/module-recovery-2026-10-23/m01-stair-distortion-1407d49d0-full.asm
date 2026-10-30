
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407d49d0:     	pushq	%rbp
1407d49d1:     	pushq	%r15
1407d49d3:     	pushq	%r14
1407d49d5:     	pushq	%r13
1407d49d7:     	pushq	%r12
1407d49d9:     	pushq	%rsi
1407d49da:     	pushq	%rdi
1407d49db:     	pushq	%rbx
1407d49dc:     	subq	$0x328, %rsp            # imm = 0x328
1407d49e3:     	leaq	0x80(%rsp), %rbp
1407d49eb:     	movaps	%xmm14, 0x290(%rbp)
1407d49f3:     	movaps	%xmm13, 0x280(%rbp)
1407d49fb:     	movaps	%xmm12, 0x270(%rbp)
1407d4a03:     	movaps	%xmm11, 0x260(%rbp)
1407d4a0b:     	movdqa	%xmm10, 0x250(%rbp)
1407d4a14:     	movaps	%xmm9, 0x240(%rbp)
1407d4a1c:     	movaps	%xmm8, 0x230(%rbp)
1407d4a24:     	movdqa	%xmm7, 0x220(%rbp)
1407d4a2c:     	movaps	%xmm6, 0x210(%rbp)
1407d4a33:     	movq	$-0x2, 0x208(%rbp)
1407d4a3e:     	movq	%r9, %rbx
1407d4a41:     	movq	%rdx, -0x28(%rbp)
1407d4a45:     	movq	%rcx, %r12
1407d4a48:     	movq	0x318(%rbp), %rax
1407d4a4f:     	movl	0x1c(%rax), %ecx
1407d4a52:     	movq	0x10(%rax), %rdx
1407d4a56:     	movl	%ecx, (%rdx)
1407d4a58:     	movq	(%rax), %rax
1407d4a5b:     	movq	%rax, 0xb8(%rbp)
1407d4a62:     	callq	0x140c16a00 <.text+0xc15a00>
1407d4a67:     	movb	%al, 0x1ef(%rbp)
1407d4a6d:     	testb	%al, %al
1407d4a6f:     	je	0x1407d4c8c <.text+0x7d3c8c>
1407d4a75:     	leaq	0x22b30ad(%rip), %r14   # 0x142a87b29
1407d4a7c:     	movl	$0x5f, %esi
1407d4a81:     	movl	$0x5f, %edx
1407d4a86:     	movq	%r14, %rcx
1407d4a89:     	callq	0x1407ceb10 <.text+0x7cdb10>
1407d4a8e:     	cmpq	$0x1, %rax
1407d4a92:     	jne	0x1407d4b42 <.text+0x7d3b42>
1407d4a98:     	movq	%rdx, %r9
1407d4a9b:     	testq	%rdx, %rdx
1407d4a9e:     	je	0x1407d4ad6 <.text+0x7d3ad6>
1407d4aa0:     	cmpq	$0x5f, %r9
1407d4aa4:     	jae	0x1407d4ad4 <.text+0x7d3ad4>
1407d4aa6:     	leaq	0x22b307c(%rip), %rax   # 0x142a87b29
1407d4aad:     	cmpb	$-0x41, (%r9,%rax)
1407d4ab2:     	jg	0x1407d4ad6 <.text+0x7d3ad6>
1407d4ab4:     	leaq	0x22b31b5(%rip), %rax   # 0x142a87c70
1407d4abb:     	movq	%rax, 0x20(%rsp)
1407d4ac0:     	leaq	0x22b3062(%rip), %rcx   # 0x142a87b29
1407d4ac7:     	movl	$0x5f, %edx
1407d4acc:     	xorl	%r8d, %r8d
1407d4acf:     	callq	0x1428d9940 <.text+0x28d8940>
1407d4ad4:     	jne	0x1407d4ab4 <.text+0x7d3ab4>
1407d4ad6:     	leaq	0x22b304c(%rip), %r14   # 0x142a87b29
1407d4add:     	movq	%r14, %rcx
1407d4ae0:     	movq	%r9, %rdx
1407d4ae3:     	callq	0x1407ceb10 <.text+0x7cdb10>
1407d4ae8:     	cmpq	$0x1, %rax
1407d4aec:     	jne	0x1407d4b42 <.text+0x7d3b42>
1407d4aee:     	movq	%rdx, %r8
1407d4af1:     	addq	$0x2, %r8
1407d4af5:     	je	0x1407d4b30 <.text+0x7d3b30>
1407d4af7:     	cmpq	$0x5f, %r8
1407d4afb:     	jae	0x1407d4b2e <.text+0x7d3b2e>
1407d4afd:     	leaq	0x22b3025(%rip), %rax   # 0x142a87b29
1407d4b04:     	cmpb	$-0x41, (%r8,%rax)
1407d4b09:     	jg	0x1407d4b30 <.text+0x7d3b30>
1407d4b0b:     	leaq	0x22b3176(%rip), %rax   # 0x142a87c88
1407d4b12:     	movq	%rax, 0x20(%rsp)
1407d4b17:     	leaq	0x22b300b(%rip), %rcx   # 0x142a87b29
1407d4b1e:     	movl	$0x5f, %edx
1407d4b23:     	movl	$0x5f, %r9d
1407d4b29:     	callq	0x1428d9940 <.text+0x28d8940>
1407d4b2e:     	jne	0x1407d4b0b <.text+0x7d3b0b>
1407d4b30:     	movl	$0x5d, %esi
1407d4b35:     	subq	%rdx, %rsi
1407d4b38:     	leaq	0x22b2fea(%rip), %r14   # 0x142a87b29
1407d4b3f:     	addq	%r8, %r14
1407d4b42:     	leaq	0x22b33f7(%rip), %r8    # 0x142a87f40
1407d4b49:     	leaq	0x22b33b0(%rip), %r15   # 0x142a87f00
1407d4b50:     	movsbl	-0x1(%r8), %eax
1407d4b55:     	testl	%eax, %eax
1407d4b57:     	js	0x1407d4b70 <.text+0x7d3b70>
1407d4b59:     	decq	%r8
1407d4b5c:     	cmpl	$0x5c, %eax
1407d4b5f:     	jne	0x1407d4bc4 <.text+0x7d3bc4>
1407d4b61:     	jmp	0x1407d4bd5 <.text+0x7d3bd5>
1407d4b63:     	nopw	%cs:(%rax,%rax)
1407d4b70:     	movzbl	-0x2(%r8), %ecx
1407d4b75:     	cmpb	$-0x40, %cl
1407d4b78:     	jge	0x1407d4b9d <.text+0x7d3b9d>
1407d4b7a:     	movzbl	-0x3(%r8), %edx
1407d4b7f:     	cmpb	$-0x40, %dl
1407d4b82:     	jge	0x1407d4ba6 <.text+0x7d3ba6>
1407d4b84:     	movzbl	-0x4(%r8), %r9d
1407d4b89:     	addq	$-0x4, %r8
1407d4b8d:     	andl	$0x7, %r9d
1407d4b91:     	shll	$0x6, %r9d
1407d4b95:     	andl	$0x3f, %edx
1407d4b98:     	orl	%r9d, %edx
1407d4b9b:     	jmp	0x1407d4bad <.text+0x7d3bad>
1407d4b9d:     	addq	$-0x2, %r8
1407d4ba1:     	andl	$0x1f, %ecx
1407d4ba4:     	jmp	0x1407d4bb5 <.text+0x7d3bb5>
1407d4ba6:     	addq	$-0x3, %r8
1407d4baa:     	andl	$0xf, %edx
1407d4bad:     	shll	$0x6, %edx
1407d4bb0:     	andl	$0x3f, %ecx
1407d4bb3:     	orl	%edx, %ecx
1407d4bb5:     	shll	$0x6, %ecx
1407d4bb8:     	andb	$0x3f, %al
1407d4bba:     	movzbl	%al, %eax
1407d4bbd:     	orl	%ecx, %eax
1407d4bbf:     	cmpl	$0x5c, %eax
1407d4bc2:     	je	0x1407d4bd5 <.text+0x7d3bd5>
1407d4bc4:     	cmpl	$0x2f, %eax
1407d4bc7:     	je	0x1407d4bd5 <.text+0x7d3bd5>
1407d4bc9:     	cmpq	%r15, %r8
1407d4bcc:     	jne	0x1407d4b50 <.text+0x7d3b50>
1407d4bce:     	movl	$0x40, %edi
1407d4bd3:     	jmp	0x1407d4c21 <.text+0x7d3c21>
1407d4bd5:     	leaq	0x22b3324(%rip), %r15   # 0x142a87f00
1407d4bdc:     	subq	%r15, %r8
1407d4bdf:     	incq	%r8
1407d4be2:     	je	0x1407d4c16 <.text+0x7d3c16>
1407d4be4:     	cmpq	$0x40, %r8
1407d4be8:     	jae	0x1407d4c14 <.text+0x7d3c14>
1407d4bea:     	cmpb	$-0x41, (%r8,%r15)
1407d4bef:     	jg	0x1407d4c16 <.text+0x7d3c16>
1407d4bf1:     	leaq	0x22b3058(%rip), %rax   # 0x142a87c50
1407d4bf8:     	movq	%rax, 0x20(%rsp)
1407d4bfd:     	leaq	0x22b32fc(%rip), %rcx   # 0x142a87f00
1407d4c04:     	movl	$0x40, %edx
1407d4c09:     	movl	$0x40, %r9d
1407d4c0f:     	callq	0x1428d9940 <.text+0x28d8940>
1407d4c14:     	jne	0x1407d4bf1 <.text+0x7d3bf1>
1407d4c16:     	movl	$0x40, %edi
1407d4c1b:     	subq	%r8, %rdi
1407d4c1e:     	addq	%r8, %r15
1407d4c21:     	callq	0x140c18130 <.text+0xc17130>
1407d4c26:     	movq	%rax, %rcx
1407d4c29:     	movq	(%rax), %rax
1407d4c2c:     	cmpq	$0x1, %rax
1407d4c30:     	jne	0x1407d7be2 <.text+0x7d6be2>
1407d4c36:     	addq	$0x8, %rcx
1407d4c3a:     	cmpq	$0x0, (%rcx)
1407d4c3e:     	jne	0x1407d7c62 <.text+0x7d6c62>
1407d4c44:     	movq	$-0x1, (%rcx)
1407d4c4b:     	movq	%rcx, 0x1f8(%rbp)
1407d4c52:     	addq	$0x8, %rcx
1407d4c56:     	movq	%rdi, 0x20(%rsp)
1407d4c5b:     	movq	$0x0, 0x30(%rsp)
1407d4c64:     	movq	$0x1, 0x28(%rsp)
1407d4c6d:     	movq	%r14, %rdx
1407d4c70:     	movq	%rsi, %r8
1407d4c73:     	movq	%r15, %r9
1407d4c76:     	callq	0x140c16d40 <.text+0xc15d40>
1407d4c7b:     	movq	%rax, 0xc8(%rbp)
1407d4c82:     	movq	0x1f8(%rbp), %rax
1407d4c89:     	incq	(%rax)
1407d4c8c:     	movq	0x310(%rbp), %rax
1407d4c93:     	movl	0x1c(%rax), %ecx
1407d4c96:     	movq	0x10(%rax), %rdx
1407d4c9a:     	movl	%ecx, (%rdx)
1407d4c9c:     	movq	(%rax), %rax
1407d4c9f:     	movq	%rax, 0x200(%rbp)
1407d4ca6:     	movl	0x1c(%rbx), %eax
1407d4ca9:     	movq	0x10(%rbx), %rcx
1407d4cad:     	movl	%eax, (%rcx)
1407d4caf:     	movq	(%rbx), %rdx
1407d4cb2:     	leaq	-0x8(%rbp), %rcx
1407d4cb6:     	callq	0x1412929e0 <.text+0x12919e0>
1407d4cbb:     	cmpq	$0x0, -0x8(%rbp)
1407d4cc0:     	je	0x1407d52af <.text+0x7d42af>
1407d4cc6:     	movq	0x8(%rbp), %rax
1407d4cca:     	movq	%rax, 0x1c0(%rbp)
1407d4cd1:     	movdqu	-0x8(%rbp), %xmm0
1407d4cd6:     	movdqa	%xmm0, 0x1b0(%rbp)
1407d4cde:     	callq	0x140c16a00 <.text+0xc15a00>
1407d4ce3:     	testb	%al, %al
1407d4ce5:     	movb	%al, 0x1f8(%rbp)
1407d4ceb:     	movq	%r12, 0x1d8(%rbp)
1407d4cf2:     	je	0x1407d4d95 <.text+0x7d3d95>
1407d4cf8:     	leaq	0x22b3241(%rip), %r8    # 0x142a87f40
1407d4cff:     	leaq	0x22b31fa(%rip), %rbx   # 0x142a87f00
1407d4d06:     	nopw	%cs:(%rax,%rax)
1407d4d10:     	movsbl	-0x1(%r8), %eax
1407d4d15:     	testl	%eax, %eax
1407d4d17:     	js	0x1407d4d30 <.text+0x7d3d30>
1407d4d19:     	decq	%r8
1407d4d1c:     	cmpl	$0x5c, %eax
1407d4d1f:     	jne	0x1407d4d84 <.text+0x7d3d84>
1407d4d21:     	jmp	0x1407d4d9c <.text+0x7d3d9c>
1407d4d23:     	nopw	%cs:(%rax,%rax)
1407d4d30:     	movzbl	-0x2(%r8), %ecx
1407d4d35:     	cmpb	$-0x40, %cl
1407d4d38:     	jge	0x1407d4d5d <.text+0x7d3d5d>
1407d4d3a:     	movzbl	-0x3(%r8), %edx
1407d4d3f:     	cmpb	$-0x40, %dl
1407d4d42:     	jge	0x1407d4d66 <.text+0x7d3d66>
1407d4d44:     	movzbl	-0x4(%r8), %r9d
1407d4d49:     	addq	$-0x4, %r8
1407d4d4d:     	andl	$0x7, %r9d
1407d4d51:     	shll	$0x6, %r9d
1407d4d55:     	andl	$0x3f, %edx
1407d4d58:     	orl	%r9d, %edx
1407d4d5b:     	jmp	0x1407d4d6d <.text+0x7d3d6d>
1407d4d5d:     	addq	$-0x2, %r8
1407d4d61:     	andl	$0x1f, %ecx
1407d4d64:     	jmp	0x1407d4d75 <.text+0x7d3d75>
1407d4d66:     	addq	$-0x3, %r8
1407d4d6a:     	andl	$0xf, %edx
1407d4d6d:     	shll	$0x6, %edx
1407d4d70:     	andl	$0x3f, %ecx
1407d4d73:     	orl	%edx, %ecx
1407d4d75:     	shll	$0x6, %ecx
1407d4d78:     	andb	$0x3f, %al
1407d4d7a:     	movzbl	%al, %eax
1407d4d7d:     	orl	%ecx, %eax
1407d4d7f:     	cmpl	$0x5c, %eax
1407d4d82:     	je	0x1407d4d9c <.text+0x7d3d9c>
1407d4d84:     	cmpl	$0x2f, %eax
1407d4d87:     	je	0x1407d4d9c <.text+0x7d3d9c>
1407d4d89:     	cmpq	%rbx, %r8
1407d4d8c:     	jne	0x1407d4d10 <.text+0x7d3d10>
1407d4d8e:     	movl	$0x40, %edi
1407d4d93:     	jmp	0x1407d4ded <.text+0x7d3ded>
1407d4d95:     	xorl	%eax, %eax
1407d4d97:     	jmp	0x1407d4e7d <.text+0x7d3e7d>
1407d4d9c:     	leaq	0x22b315d(%rip), %rbx   # 0x142a87f00
1407d4da3:     	subq	%rbx, %r8
1407d4da6:     	incq	%r8
1407d4da9:     	je	0x1407d4de2 <.text+0x7d3de2>
1407d4dab:     	cmpq	$0x40, %r8
1407d4daf:     	jae	0x1407d4de0 <.text+0x7d3de0>
1407d4db1:     	cmpb	$-0x41, (%r8,%rbx)
1407d4db6:     	jg	0x1407d4de2 <.text+0x7d3de2>
1407d4db8:     	leaq	0x22b2e91(%rip), %rax   # 0x142a87c50
1407d4dbf:     	movq	%rax, 0x20(%rsp)
1407d4dc4:     	leaq	0x22b3135(%rip), %rcx   # 0x142a87f00
1407d4dcb:     	movl	$0x40, %edx
1407d4dd0:     	movl	$0x40, %r9d
1407d4dd6:     	callq	0x1428d9940 <.text+0x28d8940>
1407d4ddb:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d4de0:     	jne	0x1407d4db8 <.text+0x7d3db8>
1407d4de2:     	movl	$0x40, %edi
1407d4de7:     	subq	%r8, %rdi
1407d4dea:     	addq	%r8, %rbx
1407d4ded:     	callq	0x140c18130 <.text+0xc17130>
1407d4df2:     	movq	%rax, %rcx
1407d4df5:     	movq	(%rax), %rax
1407d4df8:     	testq	%rax, %rax
1407d4dfb:     	je	0x1407d4e0c <.text+0x7d3e0c>
1407d4dfd:     	cmpl	$0x1, %eax
1407d4e00:     	jne	0x1407d7dbf <.text+0x7d6dbf>
1407d4e06:     	addq	$0x8, %rcx
1407d4e0a:     	jmp	0x1407d4e1f <.text+0x7d3e1f>
1407d4e0c:     	xorl	%edx, %edx
1407d4e0e:     	callq	0x1428f2580 <.text+0x28f1580>
1407d4e13:     	movq	%rax, %rcx
1407d4e16:     	testq	%rax, %rax
1407d4e19:     	je	0x1407d7dbf <.text+0x7d6dbf>
1407d4e1f:     	cmpq	$0x0, (%rcx)
1407d4e23:     	jne	0x1407d7cc9 <.text+0x7d6cc9>
1407d4e29:     	movq	$-0x1, (%rcx)
1407d4e30:     	movq	%rcx, 0x1f0(%rbp)
1407d4e37:     	addq	$0x8, %rcx
1407d4e3b:     	movq	%rdi, 0x20(%rsp)
1407d4e40:     	movq	$0x0, 0x30(%rsp)
1407d4e49:     	movq	$0x1, 0x28(%rsp)
1407d4e52:     	leaq	0x22b32df(%rip), %rdx   # 0x142a88138
1407d4e59:     	movl	$0x9, %r8d
1407d4e5f:     	movq	%rbx, %r9
1407d4e62:     	callq	0x140c16d40 <.text+0xc15d40>
1407d4e67:     	movq	0x1f0(%rbp), %rcx
1407d4e6e:     	incq	(%rcx)
1407d4e71:     	movq	%rax, 0x198(%rbp)
1407d4e78:     	movl	$0x1, %eax
1407d4e7d:     	movq	%rax, 0x190(%rbp)
1407d4e84:     	movq	0x1b8(%rbp), %rax
1407d4e8b:     	movq	0x30(%rax), %r15
1407d4e8f:     	movq	%rax, %rsi
1407d4e92:     	movq	0x48(%rax), %r12
1407d4e96:     	movdqa	(%r15), %xmm0
1407d4e9b:     	pmovmskb	%xmm0, %ebx
1407d4e9f:     	notl	%ebx
1407d4ea1:     	leaq	0x10(%r15), %r13
1407d4ea5:     	leaq	0x19(%rbp), %r14
1407d4ea9:     	movl	$0x1a203, %edi          # imm = 0x1A203
1407d4eae:     	nop
1407d4eb0:     	testq	%r12, %r12
1407d4eb3:     	je	0x1407d4f4f <.text+0x7d3f4f>
1407d4eb9:     	testw	%bx, %bx
1407d4ebc:     	jne	0x1407d4edf <.text+0x7d3edf>
1407d4ebe:     	nop
1407d4ec0:     	movdqa	(%r13), %xmm0
1407d4ec6:     	pmovmskb	%xmm0, %ebx
1407d4eca:     	addq	$-0xc0, %r15
1407d4ed1:     	addq	$0x10, %r13
1407d4ed5:     	cmpl	$0xffff, %ebx           # imm = 0xFFFF
1407d4edb:     	je	0x1407d4ec0 <.text+0x7d3ec0>
1407d4edd:     	notl	%ebx
1407d4edf:     	movl	%ebx, %eax
1407d4ee1:     	leal	-0x1(%rax), %ebx
1407d4ee4:     	andl	%eax, %ebx
1407d4ee6:     	decq	%r12
1407d4ee9:     	tzcntl	%eax, %eax
1407d4eed:     	negq	%rax
1407d4ef0:     	leaq	(%rax,%rax,2), %rdx
1407d4ef4:     	movq	-0xc(%r15,%rdx,4), %rax
1407d4ef9:     	movl	-0x4(%r15,%rdx,4), %ecx
1407d4efe:     	movzbl	-0x1(%r15,%rdx,4), %r8d
1407d4f04:     	movb	%r8b, 0x1ae(%rbp)
1407d4f0b:     	movzwl	-0x3(%r15,%rdx,4), %edx
1407d4f11:     	movw	%dx, 0x1ac(%rbp)
1407d4f18:     	btl	%ecx, %edi
1407d4f1b:     	jae	0x1407d4eb0 <.text+0x7d3eb0>
1407d4f1d:     	movq	%rax, 0x10(%rbp)
1407d4f21:     	movb	%cl, 0x18(%rbp)
1407d4f24:     	movzbl	0x1ae(%rbp), %eax
1407d4f2b:     	movb	%al, 0x2(%r14)
1407d4f2f:     	movzwl	0x1ac(%rbp), %eax
1407d4f36:     	movw	%ax, (%r14)
1407d4f3a:     	movq	0x200(%rbp), %rcx
1407d4f41:     	leaq	0x10(%rbp), %rdx
1407d4f45:     	callq	0x14086a320 <.text+0x869320>
1407d4f4a:     	jmp	0x1407d4eb0 <.text+0x7d3eb0>
1407d4f4f:     	movq	(%rsi), %r15
1407d4f52:     	movq	0x18(%rsi), %r12
1407d4f56:     	movdqa	(%r15), %xmm0
1407d4f5b:     	pmovmskb	%xmm0, %ebx
1407d4f5f:     	notl	%ebx
1407d4f61:     	leaq	0x10(%r15), %r13
1407d4f65:     	movl	$0x1a203, %edi          # imm = 0x1A203
1407d4f6a:     	nopw	(%rax,%rax)
1407d4f70:     	testq	%r12, %r12
1407d4f73:     	je	0x1407d500f <.text+0x7d400f>
1407d4f79:     	testw	%bx, %bx
1407d4f7c:     	jne	0x1407d4f9f <.text+0x7d3f9f>
1407d4f7e:     	nop
1407d4f80:     	movdqa	(%r13), %xmm0
1407d4f86:     	pmovmskb	%xmm0, %ebx
1407d4f8a:     	addq	$-0xc0, %r15
1407d4f91:     	addq	$0x10, %r13
1407d4f95:     	cmpl	$0xffff, %ebx           # imm = 0xFFFF
1407d4f9b:     	je	0x1407d4f80 <.text+0x7d3f80>
1407d4f9d:     	notl	%ebx
1407d4f9f:     	movl	%ebx, %eax
1407d4fa1:     	leal	-0x1(%rax), %ebx
1407d4fa4:     	andl	%eax, %ebx
1407d4fa6:     	decq	%r12
1407d4fa9:     	tzcntl	%eax, %eax
1407d4fad:     	negq	%rax
1407d4fb0:     	leaq	(%rax,%rax,2), %rdx
1407d4fb4:     	movq	-0xc(%r15,%rdx,4), %rax
1407d4fb9:     	movl	-0x4(%r15,%rdx,4), %ecx
1407d4fbe:     	movzbl	-0x1(%r15,%rdx,4), %r8d
1407d4fc4:     	movb	%r8b, 0x1aa(%rbp)
1407d4fcb:     	movzwl	-0x3(%r15,%rdx,4), %edx
1407d4fd1:     	movw	%dx, 0x1a8(%rbp)
1407d4fd8:     	btl	%ecx, %edi
1407d4fdb:     	jae	0x1407d4f70 <.text+0x7d3f70>
1407d4fdd:     	movq	%rax, 0x10(%rbp)
1407d4fe1:     	movb	%cl, 0x18(%rbp)
1407d4fe4:     	movzbl	0x1aa(%rbp), %eax
1407d4feb:     	movb	%al, 0x2(%r14)
1407d4fef:     	movzwl	0x1a8(%rbp), %eax
1407d4ff6:     	movw	%ax, (%r14)
1407d4ffa:     	movq	0x200(%rbp), %rcx
1407d5001:     	leaq	0x10(%rbp), %rdx
1407d5005:     	callq	0x14086a4c0 <.text+0x8694c0>
1407d500a:     	jmp	0x1407d4f70 <.text+0x7d3f70>
1407d500f:     	movq	(%rsi), %r15
1407d5012:     	movq	0x18(%rsi), %r12
1407d5016:     	movdqa	(%r15), %xmm0
1407d501b:     	pmovmskb	%xmm0, %r13d
1407d5020:     	notl	%r13d
1407d5023:     	leaq	0x10(%r15), %r14
1407d5027:     	movq	0x1d8(%rbp), %rax
1407d502e:     	movq	(%rax), %rdi
1407d5031:     	leaq	0x128(%rbp), %rbx
1407d5038:     	jmp	0x1407d508a <.text+0x7d408a>
1407d503a:     	nopw	(%rax,%rax)
1407d5040:     	testl	$0xa203, %eax           # imm = 0xA203
1407d5045:     	je	0x1407d5115 <.text+0x7d4115>
1407d504b:     	movl	0x130(%rbp), %eax
1407d5051:     	movl	%eax, 0x18(%rbp)
1407d5054:     	movq	0x128(%rbp), %rax
1407d505b:     	movq	%rax, 0x10(%rbp)
1407d505f:     	movq	%rbx, 0x30(%rsp)
1407d5064:     	movq	0x200(%rbp), %rax
1407d506b:     	movq	%rax, 0x28(%rsp)
1407d5070:     	movq	%rsi, 0x20(%rsp)
1407d5075:     	movq	%rdi, %rcx
1407d5078:     	leaq	0x10(%rbp), %rdx
1407d507c:     	movl	$0x10000, %r8d          # imm = 0x10000
1407d5082:     	movq	%rbx, %r9
1407d5085:     	callq	0x14129b780 <.text+0x129a780>
1407d508a:     	testq	%r12, %r12
1407d508d:     	je	0x1407d51e3 <.text+0x7d41e3>
1407d5093:     	testw	%r13w, %r13w
1407d5097:     	jne	0x1407d50c1 <.text+0x7d40c1>
1407d5099:     	nopl	(%rax)
1407d50a0:     	movdqa	(%r14), %xmm0
1407d50a5:     	pmovmskb	%xmm0, %r13d
1407d50aa:     	addq	$-0xc0, %r15
1407d50b1:     	addq	$0x10, %r14
1407d50b5:     	cmpl	$0xffff, %r13d          # imm = 0xFFFF
1407d50bc:     	je	0x1407d50a0 <.text+0x7d40a0>
1407d50be:     	notl	%r13d
1407d50c1:     	movl	%r13d, %eax
1407d50c4:     	leal	-0x1(%rax), %r13d
1407d50c8:     	andl	%eax, %r13d
1407d50cb:     	decq	%r12
1407d50ce:     	tzcntl	%eax, %eax
1407d50d2:     	negq	%rax
1407d50d5:     	leaq	(%rax,%rax,2), %rax
1407d50d9:     	movl	-0x4(%r15,%rax,4), %ecx
1407d50de:     	movl	%ecx, 0x130(%rbp)
1407d50e4:     	movq	-0xc(%r15,%rax,4), %rax
1407d50e9:     	movq	%rax, 0x128(%rbp)
1407d50f0:     	movl	$0x1, %eax
1407d50f5:     	shll	%cl, %eax
1407d50f7:     	movl	$0xa203, %edx           # imm = 0xA203
1407d50fc:     	btl	%ecx, %edx
1407d50ff:     	setb	%dl
1407d5102:     	cmpb	$0x10, %cl
1407d5105:     	sete	%cl
1407d5108:     	orb	%dl, %cl
1407d510a:     	je	0x1407d508a <.text+0x7d408a>
1407d5110:     	jmp	0x1407d5040 <.text+0x7d4040>
1407d5115:     	movl	0x130(%rbp), %eax
1407d511b:     	movl	%eax, 0x18(%rbp)
1407d511e:     	movq	0x128(%rbp), %rax
1407d5125:     	movq	%rax, 0x10(%rbp)
1407d5129:     	movq	%rdi, %rcx
1407d512c:     	leaq	0x10(%rbp), %rdx
1407d5130:     	callq	0x1408ce440 <.text+0x8cd440>
1407d5135:     	testq	%rax, %rax
1407d5138:     	je	0x1407d7d2a <.text+0x7d6d2a>
1407d513e:     	movq	(%rax), %rdx
1407d5141:     	movq	%rdx, %rcx
1407d5144:     	shrq	$0x20, %rcx
1407d5148:     	orl	$0x1, %ecx
1407d514b:     	callq	0x140833250 <.text+0x832250>
1407d5150:     	movl	%edx, %r8d
1407d5153:     	movq	%rdi, %rcx
1407d5156:     	movl	%eax, %edx
1407d5158:     	callq	0x1408cdbd0 <.text+0x8ccbd0>
1407d515d:     	leaq	0xd0(%rbp), %rcx
1407d5164:     	movq	%rax, %rdx
1407d5167:     	callq	0x140681f40 <.text+0x680f40>
1407d516c:     	cmpb	$0x10, 0xd0(%rbp)
1407d5173:     	jne	0x1407d7c27 <.text+0x7d6c27>
1407d5179:     	movq	0xd8(%rbp), %rax
1407d5180:     	movq	%rax, 0x120(%rbp)
1407d5187:     	leaq	0x120(%rbp), %rax
1407d518e:     	movq	%rax, 0x10(%rbp)
1407d5192:     	movq	%rbx, 0x18(%rbp)
1407d5196:     	movq	%rsi, 0x20(%rbp)
1407d519a:     	movl	0x130(%rbp), %eax
1407d51a0:     	movl	%eax, 0x98(%rbp)
1407d51a6:     	movq	0x128(%rbp), %rax
1407d51ad:     	movq	%rax, 0x90(%rbp)
1407d51b4:     	movq	%rbx, 0x28(%rsp)
1407d51b9:     	movq	0x200(%rbp), %rax
1407d51c0:     	movq	%rax, 0x20(%rsp)
1407d51c5:     	movq	%rdi, %rcx
1407d51c8:     	leaq	0x90(%rbp), %rdx
1407d51cf:     	movl	$0x1a203, %r8d          # imm = 0x1A203
1407d51d5:     	leaq	0x10(%rbp), %r9
1407d51d9:     	callq	0x14129bad0 <.text+0x129aad0>
1407d51de:     	jmp	0x1407d508a <.text+0x7d408a>
1407d51e3:     	cmpb	$0x0, 0x1f8(%rbp)
1407d51ea:     	je	0x1407d5250 <.text+0x7d4250>
1407d51ec:     	callq	0x140c18130 <.text+0xc17130>
1407d51f1:     	movq	%rax, %rcx
1407d51f4:     	movq	(%rax), %rax
1407d51f7:     	testq	%rax, %rax
1407d51fa:     	je	0x1407d520b <.text+0x7d420b>
1407d51fc:     	cmpl	$0x1, %eax
1407d51ff:     	jne	0x1407d7dcd <.text+0x7d6dcd>
1407d5205:     	addq	$0x8, %rcx
1407d5209:     	jmp	0x1407d521e <.text+0x7d421e>
1407d520b:     	xorl	%edx, %edx
1407d520d:     	callq	0x1428f2580 <.text+0x28f1580>
1407d5212:     	movq	%rax, %rcx
1407d5215:     	testq	%rax, %rax
1407d5218:     	je	0x1407d7dcd <.text+0x7d6dcd>
1407d521e:     	cmpq	$0x0, (%rcx)
1407d5222:     	jne	0x1407d7cda <.text+0x7d6cda>
1407d5228:     	movq	$-0x1, (%rcx)
1407d522f:     	movq	%rcx, 0x1f8(%rbp)
1407d5236:     	addq	$0x8, %rcx
1407d523a:     	movq	0x198(%rbp), %rdx
1407d5241:     	callq	0x140c170f0 <.text+0xc160f0>
1407d5246:     	movq	0x1f8(%rbp), %rax
1407d524d:     	incq	(%rax)
1407d5250:     	movq	0x1b0(%rbp), %rdi
1407d5257:     	movq	%rdi, %rcx
1407d525a:     	callq	0x140c81e10 <.text+0xc80e10>
1407d525f:     	movq	%rdi, %rcx
1407d5262:     	orq	$0x1, %rcx
1407d5266:     	callq	0x140c81e10 <.text+0xc80e10>
1407d526b:     	movq	$-0x10, %rax
1407d5272:     	lock
1407d5273:     	xaddq	%rax, (%rdi)
1407d5277:     	andq	$-0xe, %rax
1407d527b:     	cmpq	$0x12, %rax
1407d527f:     	je	0x1407d7c44 <.text+0x7d6c44>
1407d5285:     	movq	0x1c0(%rbp), %rax
1407d528c:     	movq	$0x0, 0x1c0(%rbp)
1407d5297:     	leaq	-0x10(%rax), %rcx
1407d529b:     	movq	%rcx, 0x10(%rbp)
1407d529f:     	lock
1407d52a0:     	decq	-0x10(%rax)
1407d52a4:     	jne	0x1407d52af <.text+0x7d42af>
1407d52a6:     	leaq	0x10(%rbp), %rcx
1407d52aa:     	callq	0x140b5cc90 <.text+0xb5bc90>
1407d52af:     	movq	0x200(%rbp), %rax
1407d52b6:     	leaq	0x150(%rax), %rcx
1407d52bd:     	movq	%rcx, 0x138(%rbp)
1407d52c4:     	movq	0x168(%rax), %r14
1407d52cb:     	testq	%r14, %r14
1407d52ce:     	je	0x1407d5396 <.text+0x7d4396>
1407d52d4:     	movq	0x138(%rbp), %rax
1407d52db:     	movq	(%rax), %r15
1407d52de:     	leaq	0x10(%r15), %r12
1407d52e2:     	movdqa	(%r15), %xmm0
1407d52e7:     	pmovmskb	%xmm0, %r13d
1407d52ec:     	notl	%r13d
1407d52ef:     	leaq	0x10(%rbp), %rdi
1407d52f3:     	jmp	0x1407d531e <.text+0x7d431e>
1407d52f5:     	nopw	%cs:(%rax,%rax)
1407d5300:     	movq	0x138(%rbp), %rcx
1407d5307:     	movq	%rbx, %rdx
1407d530a:     	callq	0x141264d10 <.text+0x1263d10>
1407d530f:     	leal	-0x1(%r13), %eax
1407d5313:     	andl	%r13d, %eax
1407d5316:     	movl	%eax, %r13d
1407d5319:     	decq	%r14
1407d531c:     	je	0x1407d5396 <.text+0x7d4396>
1407d531e:     	testw	%r13w, %r13w
1407d5322:     	jne	0x1407d5352 <.text+0x7d4352>
1407d5324:     	nopw	%cs:(%rax,%rax)
1407d5330:     	movdqa	(%r12), %xmm0
1407d5336:     	pmovmskb	%xmm0, %r13d
1407d533b:     	addq	$-0xc0, %r15
1407d5342:     	addq	$0x10, %r12
1407d5346:     	cmpl	$0xffff, %r13d          # imm = 0xFFFF
1407d534d:     	je	0x1407d5330 <.text+0x7d4330>
1407d534f:     	notl	%r13d
1407d5352:     	tzcntl	%r13d, %eax
1407d5357:     	negq	%rax
1407d535a:     	leaq	(%rax,%rax,2), %rax
1407d535e:     	leaq	(%r15,%rax,4), %rbx
1407d5362:     	cmpb	$0x10, -0x4(%r15,%rax,4)
1407d5368:     	jne	0x1407d5300 <.text+0x7d4300>
1407d536a:     	movl	-0x4(%rbx), %eax
1407d536d:     	movl	%eax, 0x18(%rbp)
1407d5370:     	movq	-0xc(%rbx), %rax
1407d5374:     	movq	%rax, 0x10(%rbp)
1407d5378:     	movq	0x200(%rbp), %rcx
1407d537f:     	movq	%rdi, %rdx
1407d5382:     	callq	0x14125d3c0 <.text+0x125c3c0>
1407d5387:     	cmpq	$0x1, %rax
1407d538b:     	jne	0x1407d5300 <.text+0x7d4300>
1407d5391:     	jmp	0x1407d530f <.text+0x7d430f>
1407d5396:     	callq	0x140c16a00 <.text+0xc15a00>
1407d539b:     	testb	%al, %al
1407d539d:     	movb	%al, 0x1ee(%rbp)
1407d53a3:     	je	0x1407d5445 <.text+0x7d4445>
1407d53a9:     	leaq	0x22b2b90(%rip), %r8    # 0x142a87f40
1407d53b0:     	leaq	0x22b2b49(%rip), %rdi   # 0x142a87f00
1407d53b7:     	nopw	(%rax,%rax)
1407d53c0:     	movsbl	-0x1(%r8), %eax
1407d53c5:     	testl	%eax, %eax
1407d53c7:     	js	0x1407d53e0 <.text+0x7d43e0>
1407d53c9:     	decq	%r8
1407d53cc:     	cmpl	$0x5c, %eax
1407d53cf:     	jne	0x1407d5434 <.text+0x7d4434>
1407d53d1:     	jmp	0x1407d5453 <.text+0x7d4453>
1407d53d6:     	nopw	%cs:(%rax,%rax)
1407d53e0:     	movzbl	-0x2(%r8), %ecx
1407d53e5:     	cmpb	$-0x40, %cl
1407d53e8:     	jge	0x1407d540d <.text+0x7d440d>
1407d53ea:     	movzbl	-0x3(%r8), %edx
1407d53ef:     	cmpb	$-0x40, %dl
1407d53f2:     	jge	0x1407d5416 <.text+0x7d4416>
1407d53f4:     	movzbl	-0x4(%r8), %r9d
1407d53f9:     	addq	$-0x4, %r8
1407d53fd:     	andl	$0x7, %r9d
1407d5401:     	shll	$0x6, %r9d
1407d5405:     	andl	$0x3f, %edx
1407d5408:     	orl	%r9d, %edx
1407d540b:     	jmp	0x1407d541d <.text+0x7d441d>
1407d540d:     	addq	$-0x2, %r8
1407d5411:     	andl	$0x1f, %ecx
1407d5414:     	jmp	0x1407d5425 <.text+0x7d4425>
1407d5416:     	addq	$-0x3, %r8
1407d541a:     	andl	$0xf, %edx
1407d541d:     	shll	$0x6, %edx
1407d5420:     	andl	$0x3f, %ecx
1407d5423:     	orl	%edx, %ecx
1407d5425:     	shll	$0x6, %ecx
1407d5428:     	andb	$0x3f, %al
1407d542a:     	movzbl	%al, %eax
1407d542d:     	orl	%ecx, %eax
1407d542f:     	cmpl	$0x5c, %eax
1407d5432:     	je	0x1407d5453 <.text+0x7d4453>
1407d5434:     	cmpl	$0x2f, %eax
1407d5437:     	je	0x1407d5453 <.text+0x7d4453>
1407d5439:     	cmpq	%rdi, %r8
1407d543c:     	jne	0x1407d53c0 <.text+0x7d43c0>
1407d543e:     	movl	$0x40, %ebx
1407d5443:     	jmp	0x1407d54a4 <.text+0x7d44a4>
1407d5445:     	xorl	%esi, %esi
1407d5447:     	movq	0x200(%rbp), %r14
1407d544e:     	jmp	0x1407d553f <.text+0x7d453f>
1407d5453:     	leaq	0x22b2aa6(%rip), %rdi   # 0x142a87f00
1407d545a:     	subq	%rdi, %r8
1407d545d:     	incq	%r8
1407d5460:     	je	0x1407d5499 <.text+0x7d4499>
1407d5462:     	cmpq	$0x40, %r8
1407d5466:     	jae	0x1407d5497 <.text+0x7d4497>
1407d5468:     	cmpb	$-0x41, (%r8,%rdi)
1407d546d:     	jg	0x1407d5499 <.text+0x7d4499>
1407d546f:     	leaq	0x22b27da(%rip), %rax   # 0x142a87c50
1407d5476:     	movq	%rax, 0x20(%rsp)
1407d547b:     	leaq	0x22b2a7e(%rip), %rcx   # 0x142a87f00
1407d5482:     	movl	$0x40, %edx
1407d5487:     	movl	$0x40, %r9d
1407d548d:     	callq	0x1428d9940 <.text+0x28d8940>
1407d5492:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d5497:     	jne	0x1407d546f <.text+0x7d446f>
1407d5499:     	movl	$0x40, %ebx
1407d549e:     	subq	%r8, %rbx
1407d54a1:     	addq	%r8, %rdi
1407d54a4:     	callq	0x140c18130 <.text+0xc17130>
1407d54a9:     	movq	%rax, %rcx
1407d54ac:     	movq	(%rax), %rax
1407d54af:     	testq	%rax, %rax
1407d54b2:     	je	0x1407d54ca <.text+0x7d44ca>
1407d54b4:     	cmpl	$0x1, %eax
1407d54b7:     	movq	0x200(%rbp), %r14
1407d54be:     	jne	0x1407d7d87 <.text+0x7d6d87>
1407d54c4:     	addq	$0x8, %rcx
1407d54c8:     	jmp	0x1407d54e4 <.text+0x7d44e4>
1407d54ca:     	xorl	%edx, %edx
1407d54cc:     	callq	0x1428f2580 <.text+0x28f1580>
1407d54d1:     	movq	%rax, %rcx
1407d54d4:     	testq	%rax, %rax
1407d54d7:     	movq	0x200(%rbp), %r14
1407d54de:     	je	0x1407d7d87 <.text+0x7d6d87>
1407d54e4:     	cmpq	$0x0, (%rcx)
1407d54e8:     	jne	0x1407d7c6e <.text+0x7d6c6e>
1407d54ee:     	movq	$-0x1, (%rcx)
1407d54f5:     	movq	%rcx, 0x1f8(%rbp)
1407d54fc:     	addq	$0x8, %rcx
1407d5500:     	movq	%rbx, 0x20(%rsp)
1407d5505:     	movq	$0x0, 0x30(%rsp)
1407d550e:     	movq	$0x1, 0x28(%rsp)
1407d5517:     	leaq	0x22b2c82(%rip), %rdx   # 0x142a881a0
1407d551e:     	movl	$0xf, %r8d
1407d5524:     	movq	%rdi, %r9
1407d5527:     	callq	0x140c16d40 <.text+0xc15d40>
1407d552c:     	movq	0x1f8(%rbp), %rcx
1407d5533:     	incq	(%rcx)
1407d5536:     	movq	%rax, (%rbp)
1407d553a:     	movl	$0x1, %esi
1407d553f:     	movq	%rsi, -0x8(%rbp)
1407d5543:     	xorl	%ecx, %ecx
1407d5545:     	callq	0x1400d9b20 <.text+0xd8b20>
1407d554a:     	testq	%rax, %rax
1407d554d:     	je	0x1407d7d3b <.text+0x7d6d3b>
1407d5553:     	movq	(%rax), %r8
1407d5556:     	movq	0x8(%rax), %rdx
1407d555a:     	leaq	0x1(%r8), %rcx
1407d555e:     	movq	%rcx, (%rax)
1407d5561:     	movq	%r8, %rbx
1407d5564:     	movq	%r8, 0xf0(%rbp)
1407d556b:     	movq	%rdx, %rdi
1407d556e:     	movq	%rdx, 0xf8(%rbp)
1407d5575:     	movdqu	0x244cc63(%rip), %xmm7  # 0x142c221e0
1407d557d:     	movdqa	%xmm7, 0xd0(%rbp)
1407d5585:     	movdqu	0x244cc62(%rip), %xmm9  # 0x142c221f0
1407d558e:     	movdqa	%xmm9, 0xe0(%rbp)
1407d5597:     	xorl	%ecx, %ecx
1407d5599:     	callq	0x1400d9b20 <.text+0xd8b20>
1407d559e:     	movq	%rsi, 0xc0(%rbp)
1407d55a5:     	testq	%rax, %rax
1407d55a8:     	je	0x1407d7d4c <.text+0x7d6d4c>
1407d55ae:     	movdqu	(%rax), %xmm0
1407d55b2:     	incq	(%rax)
1407d55b5:     	movdqa	%xmm0, 0x30(%rbp)
1407d55ba:     	movdqa	%xmm9, 0x20(%rbp)
1407d55c0:     	movdqa	%xmm7, 0x10(%rbp)
1407d55c5:     	leaq	0xf0(%r14), %rax
1407d55cc:     	movq	%rax, 0xa0(%rbp)
1407d55d3:     	leaq	0x120(%r14), %rax
1407d55da:     	movq	%rax, -0x20(%rbp)
1407d55de:     	leaq	0x140(%r14), %rax
1407d55e5:     	movq	%rax, 0x1d8(%rbp)
1407d55ec:     	leaq	0x170(%r14), %rax
1407d55f3:     	movq	%rax, 0x1f8(%rbp)
1407d55fa:     	movq	0x150(%r14), %r12
1407d5601:     	movq	0x168(%r14), %rax
1407d5608:     	movq	%rax, 0x1f0(%rbp)
1407d560f:     	leaq	0x244cbba(%rip), %rax   # 0x142c221d0
1407d5616:     	xorl	%r8d, %r8d
1407d5619:     	movss	0x21502e3(%rip), %xmm6  # 0x142925904
1407d5621:     	pcmpeqd	%xmm10, %xmm10
1407d5626:     	movss	0x22b2169(%rip), %xmm8  # 0x142a87798
1407d562f:     	xorl	%r9d, %r9d
1407d5632:     	xorl	%edx, %edx
1407d5634:     	movq	%rax, 0x1e0(%rbp)
1407d563b:     	movq	%rax, %r10
1407d563e:     	movq	%rdi, %rax
1407d5641:     	movq	%rbx, %rcx
1407d5644:     	jmp	0x1407d5693 <.text+0x7d4693>
1407d5646:     	nopw	%cs:(%rax,%rax)
1407d5650:     	movq	$0x0, 0x28(%rbp)
1407d5658:     	leaq	0x1(%rsi), %rax
1407d565c:     	movq	%rax, %rcx
1407d565f:     	shrq	$0x3, %rcx
1407d5663:     	andq	$-0x8, %rax
1407d5667:     	subq	%rcx, %rax
1407d566a:     	cmpq	$0x8, %rsi
1407d566e:     	cmovbq	%rsi, %rax
1407d5672:     	movq	%rax, 0x20(%rbp)
1407d5676:     	movq	%r13, %rax
1407d5679:     	movq	%r15, %rcx
1407d567c:     	movq	0x1f0(%rbp), %r8
1407d5683:     	movq	%rbx, %r9
1407d5686:     	movq	%rdi, %rdx
1407d5689:     	movq	0x1e0(%rbp), %r12
1407d5690:     	movq	%r12, %r10
1407d5693:     	movq	%r10, -0x10(%rbp)
1407d5697:     	movq	%r9, 0xb0(%rbp)
1407d569e:     	movq	%r8, 0xa8(%rbp)
1407d56a5:     	movq	%rdx, 0x170(%rbp)
1407d56ac:     	movq	%rcx, 0x150(%rbp)
1407d56b3:     	movq	%rax, -0x18(%rbp)
1407d56b7:     	movdqa	(%r12), %xmm0
1407d56bd:     	pmovmskb	%xmm0, %r13d
1407d56c2:     	notl	%r13d
1407d56c5:     	leaq	0x10(%r12), %rbx
1407d56ca:     	nopw	(%rax,%rax)
1407d56d0:     	movq	0x1f0(%rbp), %rsi
1407d56d7:     	testq	%rsi, %rsi
1407d56da:     	je	0x1407d5ca0 <.text+0x7d4ca0>
1407d56e0:     	testw	%r13w, %r13w
1407d56e4:     	jne	0x1407d5710 <.text+0x7d4710>
1407d56e6:     	nopw	%cs:(%rax,%rax)
1407d56f0:     	movdqa	(%rbx), %xmm0
1407d56f4:     	pmovmskb	%xmm0, %r13d
1407d56f9:     	addq	$-0xc0, %r12
1407d5700:     	addq	$0x10, %rbx
1407d5704:     	cmpl	$0xffff, %r13d          # imm = 0xFFFF
1407d570b:     	je	0x1407d56f0 <.text+0x7d46f0>
1407d570d:     	notl	%r13d
1407d5710:     	tzcntl	%r13d, %eax
1407d5715:     	negq	%rax
1407d5718:     	leaq	(%rax,%rax,2), %rax
1407d571c:     	movl	-0x4(%r12,%rax,4), %ecx
1407d5721:     	movl	%ecx, 0x118(%rbp)
1407d5727:     	movq	-0xc(%r12,%rax,4), %rax
1407d572c:     	movq	%rax, 0x110(%rbp)
1407d5733:     	cmpb	$0x10, %cl
1407d5736:     	jne	0x1407d5768 <.text+0x7d4768>
1407d5738:     	movl	0x118(%rbp), %eax
1407d573e:     	movl	%eax, 0x1b8(%rbp)
1407d5744:     	movq	0x110(%rbp), %rax
1407d574b:     	movq	%rax, 0x1b0(%rbp)
1407d5752:     	movq	0xa0(%rbp), %rcx
1407d5759:     	leaq	0x1b0(%rbp), %rdx
1407d5760:     	movaps	%xmm6, %xmm2
1407d5763:     	callq	0x1412668d0 <.text+0x12658d0>
1407d5768:     	leal	-0x1(%r13), %eax
1407d576c:     	andl	%r13d, %eax
1407d576f:     	movl	%eax, %r13d
1407d5772:     	decq	%rsi
1407d5775:     	cmpq	$0x0, 0x138(%r14)
1407d577d:     	je	0x1407d56d7 <.text+0x7d46d7>
1407d5783:     	movq	%rsi, 0x1f0(%rbp)
1407d578a:     	movq	0x1d8(%rbp), %rcx
1407d5791:     	leaq	0x110(%rbp), %rdx
1407d5798:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d579d:     	movq	0x120(%r14), %rcx
1407d57a4:     	movq	0x128(%r14), %rdx
1407d57ab:     	movq	%rdx, %r8
1407d57ae:     	andq	%rax, %r8
1407d57b1:     	shrq	$0x39, %rax
1407d57b5:     	movd	%eax, %xmm0
1407d57b9:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d57bd:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d57c2:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d57c7:     	leaq	-0x10(%rcx), %rax
1407d57cb:     	movzbl	0x118(%rbp), %r9d
1407d57d3:     	movq	0x110(%rbp), %xmm1
1407d57db:     	xorl	%r10d, %r10d
1407d57de:     	movq	0x1f0(%rbp), %rsi
1407d57e5:     	movdqu	(%rcx,%r8), %xmm2
1407d57eb:     	movdqa	%xmm2, %xmm3
1407d57ef:     	pcmpeqb	%xmm0, %xmm3
1407d57f3:     	pmovmskb	%xmm3, %r11d
1407d57f8:     	testl	%r11d, %r11d
1407d57fb:     	je	0x1407d5850 <.text+0x7d4850>
1407d57fd:     	tzcntl	%r11d, %edi
1407d5802:     	addq	%r8, %rdi
1407d5805:     	andq	%rdx, %rdi
1407d5808:     	movq	%rdi, %r14
1407d580b:     	shlq	$0x4, %r14
1407d580f:     	movq	%rax, %rsi
1407d5812:     	subq	%r14, %rsi
1407d5815:     	cmpb	0x8(%rsi), %r9b
1407d5819:     	jne	0x1407d5831 <.text+0x7d4831>
1407d581b:     	movq	(%rsi), %xmm3
1407d581f:     	pcmpeqd	%xmm1, %xmm3
1407d5823:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d5828:     	movmskpd	%xmm3, %esi
1407d582c:     	cmpl	$0x3, %esi
1407d582f:     	je	0x1407d5880 <.text+0x7d4880>
1407d5831:     	leal	-0x1(%r11), %esi
1407d5835:     	andw	%r11w, %si
1407d5839:     	movl	%esi, %r11d
1407d583c:     	movq	0x200(%rbp), %r14
1407d5843:     	movq	0x1f0(%rbp), %rsi
1407d584a:     	jne	0x1407d57fd <.text+0x7d47fd>
1407d584c:     	nopl	(%rax)
1407d5850:     	pcmpeqb	%xmm10, %xmm2
1407d5855:     	pmovmskb	%xmm2, %r11d
1407d585a:     	testl	%r11d, %r11d
1407d585d:     	jne	0x1407d56d7 <.text+0x7d46d7>
1407d5863:     	addq	%r10, %r8
1407d5866:     	addq	$0x10, %r8
1407d586a:     	addq	$0x10, %r10
1407d586e:     	andq	%rdx, %r8
1407d5871:     	jmp	0x1407d57e5 <.text+0x7d47e5>
1407d5876:     	nopw	%cs:(%rax,%rax)
1407d5880:     	negq	%rdi
1407d5883:     	shlq	$0x4, %rdi
1407d5887:     	movss	-0x4(%rcx,%rdi), %xmm11
1407d588e:     	addss	%xmm8, %xmm11
1407d5893:     	minss	%xmm6, %xmm11
1407d5898:     	movl	0x118(%rbp), %eax
1407d589e:     	movl	%eax, 0x1b8(%rbp)
1407d58a4:     	movq	0x110(%rbp), %rax
1407d58ab:     	movq	%rax, 0x1b0(%rbp)
1407d58b2:     	movq	0x200(%rbp), %r14
1407d58b9:     	movq	%r14, %rcx
1407d58bc:     	leaq	0x1b0(%rbp), %rdx
1407d58c3:     	callq	0x14125d3c0 <.text+0x125c3c0>
1407d58c8:     	movl	$0x4, %edi
1407d58cd:     	movl	$0x4, %ecx
1407d58d2:     	cmpq	$0x1, %rax
1407d58d6:     	jne	0x1407d59a3 <.text+0x7d49a3>
1407d58dc:     	movq	0x10(%r14), %rax
1407d58e0:     	cmpq	%rax, %rdx
1407d58e3:     	jae	0x1407d7cb2 <.text+0x7d6cb2>
1407d58e9:     	movq	0x8(%r14), %rax
1407d58ed:     	leaq	(%rdx,%rdx,2), %rdx
1407d58f1:     	shlq	$0x4, %rdx
1407d58f5:     	movq	0x8(%rax,%rdx), %rcx
1407d58fa:     	movq	0x10(%rax,%rdx), %rdi
1407d58ff:     	shlq	$0x4, %rdi
1407d5903:     	addq	%rcx, %rdi
1407d5906:     	jmp	0x1407d59a3 <.text+0x7d49a3>
1407d590b:     	nopl	(%rax,%rax)
1407d5910:     	movdqu	(%rcx,%r8), %xmm2
1407d5916:     	movdqa	%xmm2, %xmm3
1407d591a:     	pcmpeqb	%xmm0, %xmm3
1407d591e:     	pmovmskb	%xmm3, %r11d
1407d5923:     	testl	%r11d, %r11d
1407d5926:     	je	0x1407d5970 <.text+0x7d4970>
1407d5928:     	tzcntl	%r11d, %esi
1407d592d:     	addq	%r8, %rsi
1407d5930:     	andq	%rdx, %rsi
1407d5933:     	negq	%rsi
1407d5936:     	leaq	(%rsi,%rsi,2), %rsi
1407d593a:     	cmpb	0x8(%rax,%rsi,4), %r9b
1407d593f:     	jne	0x1407d595b <.text+0x7d495b>
1407d5941:     	leaq	(%rax,%rsi,4), %rsi
1407d5945:     	movq	(%rsi), %xmm3
1407d5949:     	pcmpeqd	%xmm1, %xmm3
1407d594d:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d5952:     	movmskpd	%xmm3, %esi
1407d5956:     	cmpl	$0x3, %esi
1407d5959:     	je	0x1407d59a0 <.text+0x7d49a0>
1407d595b:     	leal	-0x1(%r11), %esi
1407d595f:     	andw	%r11w, %si
1407d5963:     	movl	%esi, %r11d
1407d5966:     	jne	0x1407d5928 <.text+0x7d4928>
1407d5968:     	nopl	(%rax,%rax)
1407d5970:     	pcmpeqb	%xmm10, %xmm2
1407d5975:     	pmovmskb	%xmm2, %r11d
1407d597a:     	testl	%r11d, %r11d
1407d597d:     	jne	0x1407d5a50 <.text+0x7d4a50>
1407d5983:     	addq	%r10, %r8
1407d5986:     	addq	$0x10, %r8
1407d598a:     	addq	$0x10, %r10
1407d598e:     	andq	%rdx, %r8
1407d5991:     	jmp	0x1407d5910 <.text+0x7d4910>
1407d5996:     	nopw	%cs:(%rax,%rax)
1407d59a0:     	movq	%r15, %rcx
1407d59a3:     	cmpq	%rdi, %rcx
1407d59a6:     	je	0x1407d56d0 <.text+0x7d46d0>
1407d59ac:     	leaq	0x10(%rcx), %r15
1407d59b0:     	movzbl	0x8(%rcx), %eax
1407d59b4:     	cmpb	$0x11, %al
1407d59b6:     	je	0x1407d56d0 <.text+0x7d46d0>
1407d59bc:     	movq	(%rcx), %rdx
1407d59bf:     	movq	%rdx, 0x180(%rbp)
1407d59c6:     	movb	%al, 0x188(%rbp)
1407d59cc:     	movzwl	0x9(%rcx), %eax
1407d59d0:     	movzbl	0xb(%rcx), %ecx
1407d59d4:     	leaq	0x189(%rbp), %rdx
1407d59db:     	movb	%cl, 0x2(%rdx)
1407d59de:     	movw	%ax, (%rdx)
1407d59e1:     	cmpq	$0x0, 0x168(%r14)
1407d59e9:     	je	0x1407d5a50 <.text+0x7d4a50>
1407d59eb:     	movq	0x1f8(%rbp), %rcx
1407d59f2:     	leaq	0x180(%rbp), %rdx
1407d59f9:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d59fe:     	movq	0x150(%r14), %rcx
1407d5a05:     	movq	0x158(%r14), %rdx
1407d5a0c:     	movq	%rdx, %r8
1407d5a0f:     	andq	%rax, %r8
1407d5a12:     	shrq	$0x39, %rax
1407d5a16:     	movd	%eax, %xmm0
1407d5a1a:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d5a1e:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d5a23:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d5a28:     	leaq	-0xc(%rcx), %rax
1407d5a2c:     	movzbl	0x188(%rbp), %r9d
1407d5a34:     	movq	0x180(%rbp), %xmm1
1407d5a3c:     	xorl	%r10d, %r10d
1407d5a3f:     	jmp	0x1407d5910 <.text+0x7d4910>
1407d5a44:     	nopw	%cs:(%rax,%rax)
1407d5a50:     	cmpq	$0x0, 0x28(%rbp)
1407d5a55:     	je	0x1407d5b40 <.text+0x7d4b40>
1407d5a5b:     	leaq	0x30(%rbp), %rcx
1407d5a5f:     	leaq	0x180(%rbp), %rdx
1407d5a66:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d5a6b:     	movq	0x10(%rbp), %rcx
1407d5a6f:     	movq	0x18(%rbp), %rdx
1407d5a73:     	movq	%rdx, %r8
1407d5a76:     	andq	%rax, %r8
1407d5a79:     	shrq	$0x39, %rax
1407d5a7d:     	movd	%eax, %xmm0
1407d5a81:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d5a85:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d5a8a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d5a8f:     	leaq	-0xc(%rcx), %rax
1407d5a93:     	movzbl	0x188(%rbp), %r9d
1407d5a9b:     	movq	0x180(%rbp), %xmm1
1407d5aa3:     	xorl	%r10d, %r10d
1407d5aa6:     	movdqu	(%rcx,%r8), %xmm2
1407d5aac:     	movdqa	%xmm2, %xmm3
1407d5ab0:     	pcmpeqb	%xmm0, %xmm3
1407d5ab4:     	pmovmskb	%xmm3, %r11d
1407d5ab9:     	testl	%r11d, %r11d
1407d5abc:     	je	0x1407d5b10 <.text+0x7d4b10>
1407d5abe:     	tzcntl	%r11d, %esi
1407d5ac3:     	addq	%r8, %rsi
1407d5ac6:     	andq	%rdx, %rsi
1407d5ac9:     	negq	%rsi
1407d5acc:     	leaq	(%rsi,%rsi,2), %rsi
1407d5ad0:     	cmpb	0x8(%rax,%rsi,4), %r9b
1407d5ad5:     	jne	0x1407d5af5 <.text+0x7d4af5>
1407d5ad7:     	leaq	(%rax,%rsi,4), %rsi
1407d5adb:     	movq	(%rsi), %xmm3
1407d5adf:     	pcmpeqd	%xmm1, %xmm3
1407d5ae3:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d5ae8:     	movmskpd	%xmm3, %esi
1407d5aec:     	cmpl	$0x3, %esi
1407d5aef:     	je	0x1407d5c30 <.text+0x7d4c30>
1407d5af5:     	leal	-0x1(%r11), %esi
1407d5af9:     	andw	%r11w, %si
1407d5afd:     	movl	%esi, %r11d
1407d5b00:     	jne	0x1407d5abe <.text+0x7d4abe>
1407d5b02:     	nopw	%cs:(%rax,%rax)
1407d5b10:     	pcmpeqb	%xmm10, %xmm2
1407d5b15:     	pmovmskb	%xmm2, %r11d
1407d5b1a:     	testl	%r11d, %r11d
1407d5b1d:     	jne	0x1407d5b40 <.text+0x7d4b40>
1407d5b1f:     	addq	%r10, %r8
1407d5b22:     	addq	$0x10, %r8
1407d5b26:     	addq	$0x10, %r10
1407d5b2a:     	andq	%rdx, %r8
1407d5b2d:     	jmp	0x1407d5aa6 <.text+0x7d4aa6>
1407d5b32:     	nopw	%cs:(%rax,%rax)
1407d5b40:     	cmpq	$0x0, 0x138(%r14)
1407d5b48:     	movq	%r15, %rcx
1407d5b4b:     	je	0x1407d59a3 <.text+0x7d49a3>
1407d5b51:     	movq	0x1d8(%rbp), %rcx
1407d5b58:     	leaq	0x180(%rbp), %rdx
1407d5b5f:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d5b64:     	movq	0x120(%r14), %rcx
1407d5b6b:     	movq	0x128(%r14), %rdx
1407d5b72:     	movq	%rdx, %r8
1407d5b75:     	andq	%rax, %r8
1407d5b78:     	shrq	$0x39, %rax
1407d5b7c:     	movd	%eax, %xmm0
1407d5b80:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d5b84:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d5b89:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d5b8e:     	leaq	-0x10(%rcx), %r14
1407d5b92:     	movzbl	0x188(%rbp), %r9d
1407d5b9a:     	movq	0x180(%rbp), %xmm1
1407d5ba2:     	xorl	%r10d, %r10d
1407d5ba5:     	movdqu	(%rcx,%r8), %xmm2
1407d5bab:     	movdqa	%xmm2, %xmm3
1407d5baf:     	pcmpeqb	%xmm0, %xmm3
1407d5bb3:     	pmovmskb	%xmm3, %r11d
1407d5bb8:     	testl	%r11d, %r11d
1407d5bbb:     	je	0x1407d5c10 <.text+0x7d4c10>
1407d5bbd:     	movq	%r10, 0x178(%rbp)
1407d5bc4:     	tzcntl	%r11d, %esi
1407d5bc9:     	addq	%r8, %rsi
1407d5bcc:     	andq	%rdx, %rsi
1407d5bcf:     	movq	%rsi, %r10
1407d5bd2:     	shlq	$0x4, %r10
1407d5bd6:     	movq	%r14, %rax
1407d5bd9:     	subq	%r10, %rax
1407d5bdc:     	cmpb	0x8(%rax), %r9b
1407d5be0:     	jne	0x1407d5bf8 <.text+0x7d4bf8>
1407d5be2:     	movq	(%rax), %xmm3
1407d5be6:     	pcmpeqd	%xmm1, %xmm3
1407d5bea:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d5bef:     	movmskpd	%xmm3, %eax
1407d5bf3:     	cmpl	$0x3, %eax
1407d5bf6:     	je	0x1407d5c38 <.text+0x7d4c38>
1407d5bf8:     	leal	-0x1(%r11), %eax
1407d5bfc:     	andw	%r11w, %ax
1407d5c00:     	movl	%eax, %r11d
1407d5c03:     	movq	0x178(%rbp), %r10
1407d5c0a:     	jne	0x1407d5bc4 <.text+0x7d4bc4>
1407d5c0c:     	nopl	(%rax)
1407d5c10:     	pcmpeqb	%xmm10, %xmm2
1407d5c15:     	pmovmskb	%xmm2, %eax
1407d5c19:     	testl	%eax, %eax
1407d5c1b:     	jne	0x1407d5c91 <.text+0x7d4c91>
1407d5c1d:     	addq	%r10, %r8
1407d5c20:     	addq	$0x10, %r8
1407d5c24:     	addq	$0x10, %r10
1407d5c28:     	andq	%rdx, %r8
1407d5c2b:     	jmp	0x1407d5ba5 <.text+0x7d4ba5>
1407d5c30:     	movq	%r15, %rcx
1407d5c33:     	jmp	0x1407d59a3 <.text+0x7d49a3>
1407d5c38:     	negq	%rsi
1407d5c3b:     	shlq	$0x4, %rsi
1407d5c3f:     	movss	-0x4(%rcx,%rsi), %xmm0
1407d5c45:     	ucomiss	%xmm11, %xmm0
1407d5c49:     	movq	%r15, %rcx
1407d5c4c:     	movq	0x200(%rbp), %r14
1407d5c53:     	jne	0x1407d59a3 <.text+0x7d49a3>
1407d5c59:     	jp	0x1407d59a3 <.text+0x7d49a3>
1407d5c5f:     	movl	0x188(%rbp), %eax
1407d5c65:     	movl	%eax, 0x1b8(%rbp)
1407d5c6b:     	movq	0x180(%rbp), %rax
1407d5c72:     	movq	%rax, 0x1b0(%rbp)
1407d5c79:     	leaq	0x10(%rbp), %rcx
1407d5c7d:     	leaq	0x1b0(%rbp), %rdx
1407d5c84:     	callq	0x141265a10 <.text+0x1264a10>
1407d5c89:     	movq	%r15, %rcx
1407d5c8c:     	jmp	0x1407d59a3 <.text+0x7d49a3>
1407d5c91:     	movq	%r15, %rcx
1407d5c94:     	movq	0x200(%rbp), %r14
1407d5c9b:     	jmp	0x1407d59a3 <.text+0x7d49a3>
1407d5ca0:     	movq	0x28(%rbp), %rcx
1407d5ca4:     	testq	%rcx, %rcx
1407d5ca7:     	je	0x1407d5e29 <.text+0x7d4e29>
1407d5cad:     	movq	0x10(%rbp), %rax
1407d5cb1:     	movq	0x18(%rbp), %rdi
1407d5cb5:     	movq	-0x10(%rbp), %rdx
1407d5cb9:     	movq	%rdx, 0x10(%rbp)
1407d5cbd:     	movq	%rax, 0xd0(%rbp)
1407d5cc4:     	movq	0x170(%rbp), %rdx
1407d5ccb:     	movq	%rdx, 0x18(%rbp)
1407d5ccf:     	movq	%rdi, 0xd8(%rbp)
1407d5cd6:     	movq	0x20(%rbp), %rbx
1407d5cda:     	movq	0xb0(%rbp), %rdx
1407d5ce1:     	movq	%rdx, 0x20(%rbp)
1407d5ce5:     	movq	%rbx, 0xe0(%rbp)
1407d5cec:     	movq	0xa8(%rbp), %rdx
1407d5cf3:     	movq	%rdx, 0x28(%rbp)
1407d5cf7:     	movq	%rcx, 0xe8(%rbp)
1407d5cfe:     	movq	0x30(%rbp), %r15
1407d5d02:     	movq	0x150(%rbp), %rdx
1407d5d09:     	movq	%rdx, 0x30(%rbp)
1407d5d0d:     	movq	%r15, 0xf0(%rbp)
1407d5d14:     	movq	0x38(%rbp), %r13
1407d5d18:     	movq	-0x18(%rbp), %rdx
1407d5d1c:     	movq	%rdx, 0x38(%rbp)
1407d5d20:     	movq	%r13, 0xf8(%rbp)
1407d5d27:     	movq	%rax, 0x1e0(%rbp)
1407d5d2e:     	movdqa	(%rax), %xmm11
1407d5d33:     	leaq	0x1(%rcx), %rdx
1407d5d37:     	shrq	%rdx
1407d5d3a:     	cmpq	$0x0, 0x168(%r14)
1407d5d42:     	movq	%rcx, 0x1f0(%rbp)
1407d5d49:     	cmoveq	%rcx, %rdx
1407d5d4d:     	cmpq	%rdx, 0x160(%r14)
1407d5d54:     	jb	0x1407d5e0e <.text+0x7d4e0e>
1407d5d5a:     	movq	0x1e0(%rbp), %r9
1407d5d61:     	leaq	(%r9,%rdi), %rax
1407d5d65:     	incq	%rax
1407d5d68:     	pmovmskb	%xmm11, %ecx
1407d5d6d:     	notl	%ecx
1407d5d6f:     	leaq	0x10(%r9), %rdx
1407d5d73:     	movq	0x138(%rbp), %r8
1407d5d7a:     	movq	%r8, 0x90(%rbp)
1407d5d81:     	leaq	0x90(%rbp), %r8
1407d5d88:     	movq	%r8, 0x190(%rbp)
1407d5d8f:     	movq	%r9, 0x1b0(%rbp)
1407d5d96:     	movq	%rdx, 0x1b8(%rbp)
1407d5d9d:     	movq	%rax, 0x1c0(%rbp)
1407d5da4:     	movw	%cx, 0x1c8(%rbp)
1407d5dab:     	leaq	0x1b0(%rbp), %rcx
1407d5db2:     	movq	0x1f0(%rbp), %rdx
1407d5db9:     	leaq	0x190(%rbp), %r8
1407d5dc0:     	callq	0x141264390 <.text+0x1263390>
1407d5dc5:     	cmpq	$0x0, 0x28(%rbp)
1407d5dca:     	movq	%r13, %rax
1407d5dcd:     	movq	%r15, %rcx
1407d5dd0:     	movq	0x1f0(%rbp), %r8
1407d5dd7:     	movq	%rbx, %r9
1407d5dda:     	movq	%rdi, %rdx
1407d5ddd:     	movq	0x1e0(%rbp), %r12
1407d5de4:     	movq	%r12, %r10
1407d5de7:     	je	0x1407d5693 <.text+0x7d4693>
1407d5ded:     	movq	0x18(%rbp), %rsi
1407d5df1:     	testq	%rsi, %rsi
1407d5df4:     	je	0x1407d5650 <.text+0x7d4650>
1407d5dfa:     	movq	0x10(%rbp), %rcx
1407d5dfe:     	leaq	0x11(%rsi), %r8
1407d5e02:     	movb	$-0x1, %dl
1407d5e04:     	callq	0x142923c20 <.text+0x2922c20>
1407d5e09:     	jmp	0x1407d5650 <.text+0x7d4650>
1407d5e0e:     	movq	0x138(%rbp), %rcx
1407d5e15:     	movq	0x1f8(%rbp), %r8
1407d5e1c:     	movb	$0x1, %r9b
1407d5e1f:     	callq	0x1428b7220 <.text+0x28b6220>
1407d5e24:     	jmp	0x1407d5d5a <.text+0x7d4d5a>
1407d5e29:     	movq	0x18(%rbp), %r9
1407d5e2d:     	testq	%r9, %r9
1407d5e30:     	movq	0x170(%rbp), %rsi
1407d5e37:     	je	0x1407d5e6b <.text+0x7d4e6b>
1407d5e39:     	leaq	0x1(%r9), %rax
1407d5e3d:     	movl	$0xc, %ecx
1407d5e42:     	mulq	%rcx
1407d5e45:     	addq	$0xf, %rax
1407d5e49:     	andq	$-0x10, %rax
1407d5e4d:     	addq	%rax, %r9
1407d5e50:     	addq	$0x11, %r9
1407d5e54:     	je	0x1407d5e6b <.text+0x7d4e6b>
1407d5e56:     	movq	0x10(%rbp), %rcx
1407d5e5a:     	subq	%rax, %rcx
1407d5e5d:     	movl	$0x10, %r8d
1407d5e63:     	movq	%r9, %rdx
1407d5e66:     	callq	0x140613c20 <.text+0x612c20>
1407d5e6b:     	testq	%rsi, %rsi
1407d5e6e:     	movq	0xc0(%rbp), %r8
1407d5e75:     	je	0x1407d5eb0 <.text+0x7d4eb0>
1407d5e77:     	leaq	0x1(%rsi), %rax
1407d5e7b:     	movl	$0xc, %ecx
1407d5e80:     	mulq	%rcx
1407d5e83:     	addq	$0xf, %rax
1407d5e87:     	andq	$-0x10, %rax
1407d5e8b:     	addq	%rax, %rsi
1407d5e8e:     	addq	$0x11, %rsi
1407d5e92:     	je	0x1407d5eb0 <.text+0x7d4eb0>
1407d5e94:     	movq	0x1e0(%rbp), %rcx
1407d5e9b:     	subq	%rax, %rcx
1407d5e9e:     	movl	$0x10, %r8d
1407d5ea4:     	movq	%rsi, %rdx
1407d5ea7:     	callq	0x140613c20 <.text+0x612c20>
1407d5eac:     	movq	-0x8(%rbp), %r8
1407d5eb0:     	testq	%r8, %r8
1407d5eb3:     	je	0x1407d5f16 <.text+0x7d4f16>
1407d5eb5:     	callq	0x140c18130 <.text+0xc17130>
1407d5eba:     	movq	%rax, %rcx
1407d5ebd:     	movq	(%rax), %rax
1407d5ec0:     	testq	%rax, %rax
1407d5ec3:     	je	0x1407d5ed4 <.text+0x7d4ed4>
1407d5ec5:     	cmpl	$0x1, %eax
1407d5ec8:     	jne	0x1407d7d79 <.text+0x7d6d79>
1407d5ece:     	addq	$0x8, %rcx
1407d5ed2:     	jmp	0x1407d5ee7 <.text+0x7d4ee7>
1407d5ed4:     	xorl	%edx, %edx
1407d5ed6:     	callq	0x1428f2580 <.text+0x28f1580>
1407d5edb:     	movq	%rax, %rcx
1407d5ede:     	testq	%rax, %rax
1407d5ee1:     	je	0x1407d7d79 <.text+0x7d6d79>
1407d5ee7:     	cmpq	$0x0, (%rcx)
1407d5eeb:     	jne	0x1407d7c51 <.text+0x7d6c51>
1407d5ef1:     	movq	$-0x1, (%rcx)
1407d5ef8:     	movq	%rcx, 0x1d8(%rbp)
1407d5eff:     	addq	$0x8, %rcx
1407d5f03:     	movq	(%rbp), %rdx
1407d5f07:     	callq	0x140c170f0 <.text+0xc160f0>
1407d5f0c:     	movq	0x1d8(%rbp), %rax
1407d5f13:     	incq	(%rax)
1407d5f16:     	xorl	%ecx, %ecx
1407d5f18:     	callq	0x1400d9b20 <.text+0xd8b20>
1407d5f1d:     	testq	%rax, %rax
1407d5f20:     	je	0x1407d7d5d <.text+0x7d6d5d>
1407d5f26:     	movq	(%rax), %r8
1407d5f29:     	movq	0x8(%rax), %r9
1407d5f2d:     	leaq	0x1(%r8), %rcx
1407d5f31:     	movq	%rcx, (%rax)
1407d5f34:     	leaq	0x10(%rbp), %rcx
1407d5f38:     	xorl	%edx, %edx
1407d5f3a:     	callq	0x14129ccd0 <.text+0x129bcd0>
1407d5f3f:     	movq	0x200(%rbp), %r14
1407d5f46:     	movq	0x150(%r14), %r12
1407d5f4d:     	movq	0x168(%r14), %rcx
1407d5f54:     	movdqa	(%r12), %xmm0
1407d5f5a:     	pmovmskb	%xmm0, %ebx
1407d5f5e:     	notl	%ebx
1407d5f60:     	leaq	0x10(%r12), %rdi
1407d5f65:     	leaq	0x110(%r14), %rax
1407d5f6c:     	movq	%rax, 0x1d8(%rbp)
1407d5f73:     	pcmpeqd	%xmm11, %xmm11
1407d5f78:     	movdqa	0x214f96f(%rip), %xmm10 # 0x1429258f0
1407d5f81:     	jmp	0x1407d5f97 <.text+0x7d4f97>
1407d5f83:     	nopw	%cs:(%rax,%rax)
1407d5f90:     	movq	0x1f0(%rbp), %rcx
1407d5f97:     	subq	$0x1, %rcx
1407d5f9b:     	jb	0x1407d62fc <.text+0x7d52fc>
1407d5fa1:     	movq	%rcx, 0x1f0(%rbp)
1407d5fa8:     	testw	%bx, %bx
1407d5fab:     	jne	0x1407d5fcd <.text+0x7d4fcd>
1407d5fad:     	nopl	(%rax)
1407d5fb0:     	movdqa	(%rdi), %xmm0
1407d5fb4:     	pmovmskb	%xmm0, %ebx
1407d5fb8:     	addq	$-0xc0, %r12
1407d5fbf:     	addq	$0x10, %rdi
1407d5fc3:     	cmpl	$0xffff, %ebx           # imm = 0xFFFF
1407d5fc9:     	je	0x1407d5fb0 <.text+0x7d4fb0>
1407d5fcb:     	notl	%ebx
1407d5fcd:     	tzcntl	%ebx, %eax
1407d5fd1:     	negq	%rax
1407d5fd4:     	leaq	(%rax,%rax,2), %rax
1407d5fd8:     	movl	-0x4(%r12,%rax,4), %ecx
1407d5fdd:     	movl	%ecx, -0x30(%rbp)
1407d5fe0:     	movq	-0xc(%r12,%rax,4), %rax
1407d5fe5:     	movq	%rax, -0x38(%rbp)
1407d5fe9:     	movq	%r14, %rcx
1407d5fec:     	leaq	-0x38(%rbp), %rdx
1407d5ff0:     	callq	0x14125d3c0 <.text+0x125c3c0>
1407d5ff5:     	movl	$0x4, %r13d
1407d5ffb:     	movl	$0x4, %r15d
1407d6001:     	cmpq	$0x1, %rax
1407d6005:     	jne	0x1407d6031 <.text+0x7d5031>
1407d6007:     	movq	0x10(%r14), %rax
1407d600b:     	cmpq	%rax, %rdx
1407d600e:     	jae	0x1407d7cfc <.text+0x7d6cfc>
1407d6014:     	movq	0x8(%r14), %rax
1407d6018:     	leaq	(%rdx,%rdx,2), %rcx
1407d601c:     	shlq	$0x4, %rcx
1407d6020:     	movq	0x8(%rax,%rcx), %r15
1407d6025:     	movq	0x10(%rax,%rcx), %r13
1407d602a:     	shlq	$0x4, %r13
1407d602e:     	addq	%r15, %r13
1407d6031:     	leal	-0x1(%rbx), %eax
1407d6034:     	andl	%ebx, %eax
1407d6036:     	movl	%eax, %ebx
1407d6038:     	jmp	0x1407d607f <.text+0x7d507f>
1407d603a:     	nopw	(%rax,%rax)
1407d6040:     	movq	%rax, 0xd0(%rbp)
1407d6047:     	movb	%cl, 0xd8(%rbp)
1407d604d:     	movzbl	0x1b2(%rbp), %eax
1407d6054:     	leaq	0xd9(%rbp), %rcx
1407d605b:     	movb	%al, 0x2(%rcx)
1407d605e:     	movzwl	0x1b0(%rbp), %eax
1407d6065:     	movw	%ax, (%rcx)
1407d6068:     	xorps	%xmm2, %xmm2
1407d606b:     	leaq	0x10(%rbp), %rcx
1407d606f:     	leaq	0xd0(%rbp), %rdx
1407d6076:     	callq	0x14129c810 <.text+0x129b810>
1407d607b:     	addq	$0x10, %r15
1407d607f:     	cmpq	%r13, %r15
1407d6082:     	je	0x1407d5f90 <.text+0x7d4f90>
1407d6088:     	movq	(%r15), %rax
1407d608b:     	movzbl	0x8(%r15), %ecx
1407d6090:     	movzbl	0xb(%r15), %edx
1407d6095:     	movb	%dl, 0x1b2(%rbp)
1407d609b:     	movzwl	0x9(%r15), %edx
1407d60a0:     	movw	%dx, 0x1b0(%rbp)
1407d60a7:     	cmpb	$0x11, %cl
1407d60aa:     	je	0x1407d5f90 <.text+0x7d4f90>
1407d60b0:     	leaq	0x9(%r15), %rdx
1407d60b4:     	movq	%rax, 0x158(%rbp)
1407d60bb:     	movb	%cl, 0x160(%rbp)
1407d60c1:     	movzwl	(%rdx), %r8d
1407d60c5:     	movzbl	0x2(%rdx), %edx
1407d60c9:     	leaq	0x161(%rbp), %r9
1407d60d0:     	movb	%dl, 0x2(%r9)
1407d60d4:     	movw	%r8w, (%r9)
1407d60d8:     	cmpb	$0x10, %cl
1407d60db:     	jne	0x1407d6040 <.text+0x7d5040>
1407d60e1:     	cmpq	$0x0, 0x168(%r14)
1407d60e9:     	je	0x1407d61c0 <.text+0x7d51c0>
1407d60ef:     	movq	0x1f8(%rbp), %rcx
1407d60f6:     	leaq	0x158(%rbp), %rdx
1407d60fd:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d6102:     	movq	0x150(%r14), %rcx
1407d6109:     	movq	0x158(%r14), %rdx
1407d6110:     	movq	%rdx, %r8
1407d6113:     	andq	%rax, %r8
1407d6116:     	shrq	$0x39, %rax
1407d611a:     	movd	%eax, %xmm0
1407d611e:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d6122:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d6127:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d612c:     	leaq	-0xc(%rcx), %rax
1407d6130:     	movzbl	0x160(%rbp), %r9d
1407d6138:     	movq	0x158(%rbp), %xmm1
1407d6140:     	xorl	%r10d, %r10d
1407d6143:     	movdqu	(%rcx,%r8), %xmm2
1407d6149:     	movdqa	%xmm2, %xmm3
1407d614d:     	pcmpeqb	%xmm0, %xmm3
1407d6151:     	pmovmskb	%xmm3, %r11d
1407d6156:     	testl	%r11d, %r11d
1407d6159:     	je	0x1407d61a0 <.text+0x7d51a0>
1407d615b:     	tzcntl	%r11d, %esi
1407d6160:     	addq	%r8, %rsi
1407d6163:     	andq	%rdx, %rsi
1407d6166:     	negq	%rsi
1407d6169:     	leaq	(%rsi,%rsi,2), %rsi
1407d616d:     	cmpb	0x8(%rax,%rsi,4), %r9b
1407d6172:     	jne	0x1407d6192 <.text+0x7d5192>
1407d6174:     	leaq	(%rax,%rsi,4), %rsi
1407d6178:     	movq	(%rsi), %xmm3
1407d617c:     	pcmpeqd	%xmm1, %xmm3
1407d6180:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d6185:     	movmskpd	%xmm3, %esi
1407d6189:     	cmpl	$0x3, %esi
1407d618c:     	je	0x1407d607b <.text+0x7d507b>
1407d6192:     	leal	-0x1(%r11), %esi
1407d6196:     	andw	%r11w, %si
1407d619a:     	movl	%esi, %r11d
1407d619d:     	jne	0x1407d615b <.text+0x7d515b>
1407d619f:     	nop
1407d61a0:     	pcmpeqb	%xmm11, %xmm2
1407d61a5:     	pmovmskb	%xmm2, %r11d
1407d61aa:     	testl	%r11d, %r11d
1407d61ad:     	jne	0x1407d61c0 <.text+0x7d51c0>
1407d61af:     	addq	%r10, %r8
1407d61b2:     	addq	$0x10, %r8
1407d61b6:     	addq	$0x10, %r10
1407d61ba:     	andq	%rdx, %r8
1407d61bd:     	jmp	0x1407d6143 <.text+0x7d5143>
1407d61bf:     	nop
1407d61c0:     	cmpq	$0x0, 0x108(%r14)
1407d61c8:     	je	0x1407d607b <.text+0x7d507b>
1407d61ce:     	movq	0x1d8(%rbp), %rcx
1407d61d5:     	leaq	0x158(%rbp), %rdx
1407d61dc:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d61e1:     	movq	0xf0(%r14), %rcx
1407d61e8:     	movq	0xf8(%r14), %rdx
1407d61ef:     	movq	%rdx, %r8
1407d61f2:     	andq	%rax, %r8
1407d61f5:     	shrq	$0x39, %rax
1407d61f9:     	movd	%eax, %xmm0
1407d61fd:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d6201:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d6206:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d620b:     	leaq	-0x10(%rcx), %r10
1407d620f:     	movzbl	0x160(%rbp), %r9d
1407d6217:     	movq	0x158(%rbp), %xmm1
1407d621f:     	xorl	%esi, %esi
1407d6221:     	movdqu	(%rcx,%r8), %xmm2
1407d6227:     	movdqa	%xmm2, %xmm3
1407d622b:     	pcmpeqb	%xmm0, %xmm3
1407d622f:     	pmovmskb	%xmm3, %r11d
1407d6234:     	testl	%r11d, %r11d
1407d6237:     	je	0x1407d6290 <.text+0x7d5290>
1407d6239:     	movq	%rsi, 0x178(%rbp)
1407d6240:     	tzcntl	%r11d, %esi
1407d6245:     	addq	%r8, %rsi
1407d6248:     	andq	%rdx, %rsi
1407d624b:     	movq	%rsi, %r14
1407d624e:     	shlq	$0x4, %r14
1407d6252:     	movq	%r10, %rax
1407d6255:     	subq	%r14, %rax
1407d6258:     	cmpb	0x8(%rax), %r9b
1407d625c:     	jne	0x1407d6274 <.text+0x7d5274>
1407d625e:     	movq	(%rax), %xmm3
1407d6262:     	pcmpeqd	%xmm1, %xmm3
1407d6266:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d626b:     	movmskpd	%xmm3, %eax
1407d626f:     	cmpl	$0x3, %eax
1407d6272:     	je	0x1407d62b4 <.text+0x7d52b4>
1407d6274:     	leal	-0x1(%r11), %eax
1407d6278:     	andw	%r11w, %ax
1407d627c:     	movl	%eax, %r11d
1407d627f:     	movq	0x200(%rbp), %r14
1407d6286:     	movq	0x178(%rbp), %rsi
1407d628d:     	jne	0x1407d6240 <.text+0x7d5240>
1407d628f:     	nop
1407d6290:     	pcmpeqb	%xmm11, %xmm2
1407d6295:     	pmovmskb	%xmm2, %eax
1407d6299:     	testl	%eax, %eax
1407d629b:     	jne	0x1407d607b <.text+0x7d507b>
1407d62a1:     	addq	%rsi, %r8
1407d62a4:     	addq	$0x10, %r8
1407d62a8:     	addq	$0x10, %rsi
1407d62ac:     	andq	%rdx, %r8
1407d62af:     	jmp	0x1407d6221 <.text+0x7d5221>
1407d62b4:     	negq	%rsi
1407d62b7:     	shlq	$0x4, %rsi
1407d62bb:     	movd	-0x4(%rcx,%rsi), %xmm2
1407d62c1:     	pxor	%xmm10, %xmm2
1407d62c6:     	movl	0x160(%rbp), %eax
1407d62cc:     	movl	%eax, 0xd8(%rbp)
1407d62d2:     	movq	0x158(%rbp), %rax
1407d62d9:     	movq	%rax, 0xd0(%rbp)
1407d62e0:     	leaq	0x10(%rbp), %rcx
1407d62e4:     	leaq	0xd0(%rbp), %rdx
1407d62eb:     	callq	0x14129c810 <.text+0x129b810>
1407d62f0:     	movq	0x200(%rbp), %r14
1407d62f7:     	jmp	0x1407d607b <.text+0x7d507b>
1407d62fc:     	xorl	%ecx, %ecx
1407d62fe:     	callq	0x1400d9b20 <.text+0xd8b20>
1407d6303:     	testq	%rax, %rax
1407d6306:     	je	0x1407d7d6b <.text+0x7d6d6b>
1407d630c:     	movups	(%rax), %xmm0
1407d630f:     	incq	(%rax)
1407d6312:     	movaps	%xmm0, 0xf0(%rbp)
1407d6319:     	movdqa	%xmm9, 0xe0(%rbp)
1407d6322:     	movdqa	%xmm7, 0xd0(%rbp)
1407d632a:     	movq	%r14, %rax
1407d632d:     	movq	0x150(%r14), %r14
1407d6334:     	movq	0x168(%rax), %r15
1407d633b:     	movdqa	(%r14), %xmm0
1407d6340:     	pmovmskb	%xmm0, %r13d
1407d6345:     	notl	%r13d
1407d6348:     	leaq	0x10(%r14), %r12
1407d634c:     	addq	$0xb0, %rax
1407d6352:     	movq	%rax, 0x1f8(%rbp)
1407d6359:     	pcmpeqd	%xmm7, %xmm7
1407d635d:     	jmp	0x1407d6435 <.text+0x7d5435>
1407d6362:     	nopw	%cs:(%rax,%rax)
1407d6370:     	movdqu	(%rcx,%r9), %xmm1
1407d6376:     	movdqa	%xmm1, %xmm2
1407d637a:     	pcmpeqb	%xmm0, %xmm2
1407d637e:     	pmovmskb	%xmm2, %r11d
1407d6383:     	testl	%r11d, %r11d
1407d6386:     	je	0x1407d63f0 <.text+0x7d53f0>
1407d6388:     	movzbl	-0x4(%rdx), %edi
1407d638c:     	movq	%rsi, 0x1f0(%rbp)
1407d6393:     	tzcntl	%r11d, %esi
1407d6398:     	addq	%r9, %rsi
1407d639b:     	andq	%r8, %rsi
1407d639e:     	negq	%rsi
1407d63a1:     	leaq	(%rsi,%rsi,2), %rsi
1407d63a5:     	cmpb	0x8(%rax,%rsi,8), %dil
1407d63aa:     	jne	0x1407d63cd <.text+0x7d53cd>
1407d63ac:     	leaq	(%rax,%rsi,8), %r10
1407d63b0:     	movq	(%rbx), %xmm2
1407d63b4:     	movq	(%r10), %xmm3
1407d63b9:     	pcmpeqd	%xmm2, %xmm3
1407d63bd:     	pshufd	$0x50, %xmm3, %xmm2     # xmm2 = xmm3[0,0,1,1]
1407d63c2:     	movmskpd	%xmm2, %r10d
1407d63c7:     	cmpl	$0x3, %r10d
1407d63cb:     	je	0x1407d6420 <.text+0x7d5420>
1407d63cd:     	leal	-0x1(%r11), %r10d
1407d63d1:     	andw	%r11w, %r10w
1407d63d5:     	movl	%r10d, %r11d
1407d63d8:     	movq	0x1f0(%rbp), %rsi
1407d63df:     	jne	0x1407d6393 <.text+0x7d5393>
1407d63e1:     	nopw	%cs:(%rax,%rax)
1407d63f0:     	pcmpeqb	%xmm7, %xmm1
1407d63f4:     	pmovmskb	%xmm1, %r10d
1407d63f9:     	testl	%r10d, %r10d
1407d63fc:     	jne	0x1407d6435 <.text+0x7d5435>
1407d63fe:     	addq	%rsi, %r9
1407d6401:     	addq	$0x10, %r9
1407d6405:     	addq	$0x10, %rsi
1407d6409:     	andq	%r8, %r9
1407d640c:     	jmp	0x1407d6370 <.text+0x7d5370>
1407d6411:     	nopw	%cs:(%rax,%rax)
1407d6420:     	movl	-0xc(%rcx,%rsi,8), %edx
1407d6424:     	movl	-0x8(%rcx,%rsi,8), %r8d
1407d6429:     	leaq	0xd0(%rbp), %rcx
1407d6430:     	callq	0x1402c8640 <.text+0x2c7640>
1407d6435:     	testq	%r15, %r15
1407d6438:     	je	0x1407d64f2 <.text+0x7d54f2>
1407d643e:     	testw	%r13w, %r13w
1407d6442:     	jne	0x1407d6472 <.text+0x7d5472>
1407d6444:     	nopw	%cs:(%rax,%rax)
1407d6450:     	movdqa	(%r12), %xmm0
1407d6456:     	pmovmskb	%xmm0, %r13d
1407d645b:     	addq	$-0xc0, %r14
1407d6462:     	addq	$0x10, %r12
1407d6466:     	cmpl	$0xffff, %r13d          # imm = 0xFFFF
1407d646d:     	je	0x1407d6450 <.text+0x7d5450>
1407d646f:     	notl	%r13d
1407d6472:     	movl	%r13d, %eax
1407d6475:     	leal	-0x1(%rax), %r13d
1407d6479:     	andl	%eax, %r13d
1407d647c:     	decq	%r15
1407d647f:     	movq	0x200(%rbp), %rcx
1407d6486:     	cmpq	$0x0, 0xa8(%rcx)
1407d648e:     	je	0x1407d6435 <.text+0x7d5435>
1407d6490:     	tzcntl	%eax, %eax
1407d6494:     	negq	%rax
1407d6497:     	leaq	(%rax,%rax,2), %rsi
1407d649b:     	leaq	(%r14,%rsi,4), %rbx
1407d649f:     	addq	$-0xc, %rbx
1407d64a3:     	movq	0x1f8(%rbp), %rcx
1407d64aa:     	movq	%rbx, %rdx
1407d64ad:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d64b2:     	leaq	(%r14,%rsi,4), %rdx
1407d64b6:     	movq	0x200(%rbp), %r8
1407d64bd:     	movq	0x90(%r8), %rcx
1407d64c4:     	movq	0x98(%r8), %r8
1407d64cb:     	movq	%r8, %r9
1407d64ce:     	andq	%rax, %r9
1407d64d1:     	shrq	$0x39, %rax
1407d64d5:     	movd	%eax, %xmm0
1407d64d9:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d64dd:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d64e2:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d64e7:     	leaq	-0x18(%rcx), %rax
1407d64eb:     	xorl	%esi, %esi
1407d64ed:     	jmp	0x1407d6370 <.text+0x7d5370>
1407d64f2:     	callq	0x140c16a00 <.text+0xc15a00>
1407d64f7:     	movl	%eax, %esi
1407d64f9:     	testb	%al, %al
1407d64fb:     	je	0x1407d6595 <.text+0x7d5595>
1407d6501:     	leaq	0x22b1a38(%rip), %r8    # 0x142a87f40
1407d6508:     	leaq	0x22b19f1(%rip), %rdi   # 0x142a87f00
1407d650f:     	nop
1407d6510:     	movsbl	-0x1(%r8), %eax
1407d6515:     	testl	%eax, %eax
1407d6517:     	js	0x1407d6530 <.text+0x7d5530>
1407d6519:     	decq	%r8
1407d651c:     	cmpl	$0x5c, %eax
1407d651f:     	jne	0x1407d6584 <.text+0x7d5584>
1407d6521:     	jmp	0x1407d659c <.text+0x7d559c>
1407d6523:     	nopw	%cs:(%rax,%rax)
1407d6530:     	movzbl	-0x2(%r8), %ecx
1407d6535:     	cmpb	$-0x40, %cl
1407d6538:     	jge	0x1407d655d <.text+0x7d555d>
1407d653a:     	movzbl	-0x3(%r8), %edx
1407d653f:     	cmpb	$-0x40, %dl
1407d6542:     	jge	0x1407d6566 <.text+0x7d5566>
1407d6544:     	movzbl	-0x4(%r8), %r9d
1407d6549:     	addq	$-0x4, %r8
1407d654d:     	andl	$0x7, %r9d
1407d6551:     	shll	$0x6, %r9d
1407d6555:     	andl	$0x3f, %edx
1407d6558:     	orl	%r9d, %edx
1407d655b:     	jmp	0x1407d656d <.text+0x7d556d>
1407d655d:     	addq	$-0x2, %r8
1407d6561:     	andl	$0x1f, %ecx
1407d6564:     	jmp	0x1407d6575 <.text+0x7d5575>
1407d6566:     	addq	$-0x3, %r8
1407d656a:     	andl	$0xf, %edx
1407d656d:     	shll	$0x6, %edx
1407d6570:     	andl	$0x3f, %ecx
1407d6573:     	orl	%edx, %ecx
1407d6575:     	shll	$0x6, %ecx
1407d6578:     	andb	$0x3f, %al
1407d657a:     	movzbl	%al, %eax
1407d657d:     	orl	%ecx, %eax
1407d657f:     	cmpl	$0x5c, %eax
1407d6582:     	je	0x1407d659c <.text+0x7d559c>
1407d6584:     	cmpl	$0x2f, %eax
1407d6587:     	je	0x1407d659c <.text+0x7d559c>
1407d6589:     	cmpq	%rdi, %r8
1407d658c:     	jne	0x1407d6510 <.text+0x7d5510>
1407d658e:     	movl	$0x40, %ebx
1407d6593:     	jmp	0x1407d65ed <.text+0x7d55ed>
1407d6595:     	xorl	%eax, %eax
1407d6597:     	jmp	0x1407d667a <.text+0x7d567a>
1407d659c:     	leaq	0x22b195d(%rip), %rdi   # 0x142a87f00
1407d65a3:     	subq	%rdi, %r8
1407d65a6:     	incq	%r8
1407d65a9:     	je	0x1407d65e2 <.text+0x7d55e2>
1407d65ab:     	cmpq	$0x40, %r8
1407d65af:     	jae	0x1407d65e0 <.text+0x7d55e0>
1407d65b1:     	cmpb	$-0x41, (%r8,%rdi)
1407d65b6:     	jg	0x1407d65e2 <.text+0x7d55e2>
1407d65b8:     	leaq	0x22b1691(%rip), %rax   # 0x142a87c50
1407d65bf:     	movq	%rax, 0x20(%rsp)
1407d65c4:     	leaq	0x22b1935(%rip), %rcx   # 0x142a87f00
1407d65cb:     	movl	$0x40, %edx
1407d65d0:     	movl	$0x40, %r9d
1407d65d6:     	callq	0x1428d9940 <.text+0x28d8940>
1407d65db:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d65e0:     	jne	0x1407d65b8 <.text+0x7d55b8>
1407d65e2:     	movl	$0x40, %ebx
1407d65e7:     	subq	%r8, %rbx
1407d65ea:     	addq	%r8, %rdi
1407d65ed:     	callq	0x140c18130 <.text+0xc17130>
1407d65f2:     	movq	%rax, %rcx
1407d65f5:     	movq	(%rax), %rax
1407d65f8:     	testq	%rax, %rax
1407d65fb:     	je	0x1407d660c <.text+0x7d560c>
1407d65fd:     	cmpl	$0x1, %eax
1407d6600:     	jne	0x1407d7d95 <.text+0x7d6d95>
1407d6606:     	addq	$0x8, %rcx
1407d660a:     	jmp	0x1407d661f <.text+0x7d561f>
1407d660c:     	xorl	%edx, %edx
1407d660e:     	callq	0x1428f2580 <.text+0x28f1580>
1407d6613:     	movq	%rax, %rcx
1407d6616:     	testq	%rax, %rax
1407d6619:     	je	0x1407d7d95 <.text+0x7d6d95>
1407d661f:     	cmpq	$0x0, (%rcx)
1407d6623:     	jne	0x1407d7c7f <.text+0x7d6c7f>
1407d6629:     	movq	$-0x1, (%rcx)
1407d6630:     	movq	%rcx, 0x1f0(%rbp)
1407d6637:     	addq	$0x8, %rcx
1407d663b:     	movq	%rbx, 0x20(%rsp)
1407d6640:     	movq	$0x0, 0x30(%rsp)
1407d6649:     	movq	$0x1, 0x28(%rsp)
1407d6652:     	leaq	0x22b1b56(%rip), %rdx   # 0x142a881af
1407d6659:     	movl	$0x9, %r8d
1407d665f:     	movq	%rdi, %r9
1407d6662:     	callq	0x140c16d40 <.text+0xc15d40>
1407d6667:     	movq	0x1f0(%rbp), %rcx
1407d666e:     	incq	(%rcx)
1407d6671:     	movq	%rax, (%rbp)
1407d6675:     	movl	$0x1, %eax
1407d667a:     	movb	%sil, 0x1f0(%rbp)
1407d6681:     	movq	%rax, -0x8(%rbp)
1407d6685:     	leaq	0x149(%rbp), %rsi
1407d668c:     	pcmpeqd	%xmm9, %xmm9
1407d6691:     	nopw	%cs:(%rax,%rax)
1407d66a0:     	leaq	0x1b0(%rbp), %rcx
1407d66a7:     	leaq	0x10(%rbp), %rdx
1407d66ab:     	callq	0x14129c6d0 <.text+0x129b6d0>
1407d66b0:     	cmpb	$0x11, 0x1b8(%rbp)
1407d66b7:     	je	0x1407d6bb8 <.text+0x7d5bb8>
1407d66bd:     	movl	0x1b8(%rbp), %eax
1407d66c3:     	movl	%eax, -0x40(%rbp)
1407d66c6:     	movq	0x1b0(%rbp), %rax
1407d66cd:     	movq	%rax, -0x48(%rbp)
1407d66d1:     	movss	0x1bc(%rbp), %xmm7
1407d66d9:     	movq	0x200(%rbp), %rcx
1407d66e0:     	leaq	-0x48(%rbp), %rdx
1407d66e4:     	callq	0x14125d3c0 <.text+0x125c3c0>
1407d66e9:     	movl	$0x4, %ebx
1407d66ee:     	movl	$0x4, %r15d
1407d66f4:     	cmpq	$0x1, %rax
1407d66f8:     	jne	0x1407d672b <.text+0x7d572b>
1407d66fa:     	movq	0x200(%rbp), %rcx
1407d6701:     	movq	0x10(%rcx), %rax
1407d6705:     	cmpq	%rax, %rdx
1407d6708:     	jae	0x1407d7d13 <.text+0x7d6d13>
1407d670e:     	movq	0x8(%rcx), %rax
1407d6712:     	leaq	(%rdx,%rdx,2), %rcx
1407d6716:     	shlq	$0x4, %rcx
1407d671a:     	movq	0x8(%rax,%rcx), %r15
1407d671f:     	movq	0x10(%rax,%rcx), %rbx
1407d6724:     	shlq	$0x4, %rbx
1407d6728:     	addq	%r15, %rbx
1407d672b:     	movaps	%xmm8, %xmm11
1407d672f:     	subss	%xmm7, %xmm11
1407d6734:     	minss	%xmm6, %xmm11
1407d6739:     	movaps	%xmm11, %xmm7
1407d673d:     	xorps	%xmm10, %xmm7
1407d6741:     	jmp	0x1407d67e8 <.text+0x7d57e8>
1407d6746:     	nopw	%cs:(%rax,%rax)
1407d6750:     	movdqu	(%rcx,%r8), %xmm2
1407d6756:     	movdqa	%xmm2, %xmm3
1407d675a:     	pcmpeqb	%xmm0, %xmm3
1407d675e:     	pmovmskb	%xmm3, %r11d
1407d6763:     	testl	%r11d, %r11d
1407d6766:     	je	0x1407d67b0 <.text+0x7d57b0>
1407d6768:     	movq	%rsi, %r14
1407d676b:     	tzcntl	%r11d, %esi
1407d6770:     	addq	%r8, %rsi
1407d6773:     	andq	%rdx, %rsi
1407d6776:     	negq	%rsi
1407d6779:     	leaq	(%rsi,%rsi,2), %rsi
1407d677d:     	cmpb	0x8(%rax,%rsi,8), %r9b
1407d6782:     	jne	0x1407d679e <.text+0x7d579e>
1407d6784:     	leaq	(%rax,%rsi,8), %rdi
1407d6788:     	movq	(%rdi), %xmm3
1407d678c:     	pcmpeqd	%xmm1, %xmm3
1407d6790:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d6795:     	movmskpd	%xmm3, %edi
1407d6799:     	cmpl	$0x3, %edi
1407d679c:     	je	0x1407d67d0 <.text+0x7d57d0>
1407d679e:     	leal	-0x1(%r11), %esi
1407d67a2:     	andw	%r11w, %si
1407d67a6:     	movl	%esi, %r11d
1407d67a9:     	movq	%r14, %rsi
1407d67ac:     	jne	0x1407d6768 <.text+0x7d5768>
1407d67ae:     	nop
1407d67b0:     	pcmpeqb	%xmm9, %xmm2
1407d67b5:     	pmovmskb	%xmm2, %r11d
1407d67ba:     	testl	%r11d, %r11d
1407d67bd:     	jne	0x1407d67e8 <.text+0x7d57e8>
1407d67bf:     	addq	%r10, %r8
1407d67c2:     	addq	$0x10, %r8
1407d67c6:     	addq	$0x10, %r10
1407d67ca:     	andq	%rdx, %r8
1407d67cd:     	jmp	0x1407d6750 <.text+0x7d5750>
1407d67cf:     	nop
1407d67d0:     	movl	-0xc(%rcx,%rsi,8), %edx
1407d67d4:     	movl	-0x8(%rcx,%rsi,8), %r8d
1407d67d9:     	leaq	0xd0(%rbp), %rcx
1407d67e0:     	callq	0x1402c8640 <.text+0x2c7640>
1407d67e5:     	movq	%r14, %rsi
1407d67e8:     	cmpq	%rbx, %r15
1407d67eb:     	je	0x1407d66a0 <.text+0x7d56a0>
1407d67f1:     	movq	(%r15), %rax
1407d67f4:     	movzbl	0x8(%r15), %ecx
1407d67f9:     	movzbl	0xb(%r15), %edx
1407d67fe:     	movb	%dl, 0x1a6(%rbp)
1407d6804:     	movzwl	0x9(%r15), %edx
1407d6809:     	movw	%dx, 0x1a4(%rbp)
1407d6810:     	cmpb	$0x11, %cl
1407d6813:     	je	0x1407d66a0 <.text+0x7d56a0>
1407d6819:     	leaq	0x9(%r15), %rdx
1407d681d:     	movq	%rax, 0x140(%rbp)
1407d6824:     	movb	%cl, 0x148(%rbp)
1407d682a:     	movzwl	(%rdx), %r8d
1407d682e:     	movzbl	0x2(%rdx), %edx
1407d6832:     	movb	%dl, 0x2(%rsi)
1407d6835:     	movw	%r8w, (%rsi)
1407d6839:     	movq	%rax, 0x190(%rbp)
1407d6840:     	movb	%cl, 0x198(%rbp)
1407d6846:     	movzbl	0x1a6(%rbp), %eax
1407d684d:     	leaq	0x199(%rbp), %rcx
1407d6854:     	movb	%al, 0x2(%rcx)
1407d6857:     	movzwl	0x1a4(%rbp), %eax
1407d685e:     	movw	%ax, (%rcx)
1407d6861:     	movq	0x1d8(%rbp), %rcx
1407d6868:     	leaq	0x190(%rbp), %rdx
1407d686f:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d6874:     	movq	%rax, %rdi
1407d6877:     	addq	$0x10, %r15
1407d687b:     	movq	%rax, %r13
1407d687e:     	shrq	$0x39, %r13
1407d6882:     	movq	0x200(%rbp), %r10
1407d6889:     	movq	0xf0(%r10), %rax
1407d6890:     	movq	0xf8(%r10), %rcx
1407d6897:     	movq	%rcx, %rdx
1407d689a:     	andq	%rdi, %rdx
1407d689d:     	movd	%r13d, %xmm0
1407d68a2:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d68a6:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d68ab:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d68b0:     	movzbl	0x198(%rbp), %r12d
1407d68b8:     	movq	0x190(%rbp), %xmm1
1407d68c0:     	xorl	%r8d, %r8d
1407d68c3:     	movdqu	(%rax,%rdx), %xmm2
1407d68c8:     	movdqa	%xmm2, %xmm3
1407d68cc:     	pcmpeqb	%xmm0, %xmm3
1407d68d0:     	pmovmskb	%xmm3, %r9d
1407d68d5:     	testl	%r9d, %r9d
1407d68d8:     	je	0x1407d6930 <.text+0x7d5930>
1407d68da:     	tzcntl	%r9d, %r11d
1407d68df:     	addq	%rdx, %r11
1407d68e2:     	andq	%rcx, %r11
1407d68e5:     	shlq	$0x4, %r11
1407d68e9:     	movq	%rax, %r10
1407d68ec:     	subq	%r11, %r10
1407d68ef:     	cmpb	%r12b, -0x8(%r10)
1407d68f3:     	jne	0x1407d6913 <.text+0x7d5913>
1407d68f5:     	movq	-0x10(%r10), %xmm3
1407d68fb:     	pcmpeqd	%xmm1, %xmm3
1407d68ff:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d6904:     	movmskpd	%xmm3, %r11d
1407d6909:     	cmpl	$0x3, %r11d
1407d690d:     	je	0x1407d69a1 <.text+0x7d59a1>
1407d6913:     	leal	-0x1(%r9), %r10d
1407d6917:     	andw	%r9w, %r10w
1407d691b:     	movl	%r10d, %r9d
1407d691e:     	movq	0x200(%rbp), %r10
1407d6925:     	jne	0x1407d68da <.text+0x7d58da>
1407d6927:     	nopw	(%rax,%rax)
1407d6930:     	pcmpeqb	%xmm9, %xmm2
1407d6935:     	pmovmskb	%xmm2, %r9d
1407d693a:     	testl	%r9d, %r9d
1407d693d:     	jne	0x1407d6960 <.text+0x7d5960>
1407d693f:     	addq	%r8, %rdx
1407d6942:     	addq	$0x10, %rdx
1407d6946:     	addq	$0x10, %r8
1407d694a:     	andq	%rcx, %rdx
1407d694d:     	jmp	0x1407d68c3 <.text+0x7d58c3>
1407d6952:     	nopw	%cs:(%rax,%rax)
1407d6960:     	cmpq	$0x0, 0x100(%r10)
1407d6968:     	je	0x1407d6b57 <.text+0x7d5b57>
1407d696e:     	movq	0x190(%rbp), %r14
1407d6975:     	leaq	0x199(%rbp), %rcx
1407d697c:     	movzwl	(%rcx), %eax
1407d697f:     	movw	%ax, 0x168(%rbp)
1407d6986:     	movzbl	0x2(%rcx), %eax
1407d698a:     	movb	%al, 0x16a(%rbp)
1407d6990:     	movq	0xa0(%rbp), %r10
1407d6997:     	cmpb	$0x11, %r12b
1407d699b:     	jne	0x1407d6a55 <.text+0x7d5a55>
1407d69a1:     	movss	-0x4(%r10), %xmm0
1407d69a7:     	ucomiss	%xmm11, %xmm0
1407d69ab:     	jbe	0x1407d69e0 <.text+0x7d59e0>
1407d69ad:     	movss	%xmm11, -0x4(%r10)
1407d69b3:     	movl	0x148(%rbp), %eax
1407d69b9:     	movl	%eax, 0x198(%rbp)
1407d69bf:     	movq	0x140(%rbp), %rax
1407d69c6:     	movq	%rax, 0x190(%rbp)
1407d69cd:     	leaq	0x10(%rbp), %rcx
1407d69d1:     	leaq	0x190(%rbp), %rdx
1407d69d8:     	movaps	%xmm7, %xmm2
1407d69db:     	callq	0x14129c810 <.text+0x129b810>
1407d69e0:     	movq	0x200(%rbp), %rax
1407d69e7:     	cmpq	$0x0, 0xa8(%rax)
1407d69ef:     	je	0x1407d67e8 <.text+0x7d57e8>
1407d69f5:     	movq	0x1f8(%rbp), %rcx
1407d69fc:     	leaq	0x140(%rbp), %rdx
1407d6a03:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d6a08:     	movq	0x200(%rbp), %rdx
1407d6a0f:     	movq	0x90(%rdx), %rcx
1407d6a16:     	movq	0x98(%rdx), %rdx
1407d6a1d:     	movq	%rdx, %r8
1407d6a20:     	andq	%rax, %r8
1407d6a23:     	shrq	$0x39, %rax
1407d6a27:     	movd	%eax, %xmm0
1407d6a2b:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d6a2f:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d6a34:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d6a39:     	leaq	-0x18(%rcx), %rax
1407d6a3d:     	movzbl	0x148(%rbp), %r9d
1407d6a45:     	movq	0x140(%rbp), %xmm1
1407d6a4d:     	xorl	%r10d, %r10d
1407d6a50:     	jmp	0x1407d6750 <.text+0x7d5750>
1407d6a55:     	movl	0x168(%rbp), %eax
1407d6a5b:     	movl	0x16b(%rbp), %ecx
1407d6a61:     	movl	%ecx, 0x123(%rbp)
1407d6a67:     	movl	%eax, 0x120(%rbp)
1407d6a6d:     	movl	0x148(%rbp), %eax
1407d6a73:     	movl	%eax, 0x198(%rbp)
1407d6a79:     	movq	0x140(%rbp), %rax
1407d6a80:     	movq	%rax, 0x190(%rbp)
1407d6a87:     	leaq	0x10(%rbp), %rcx
1407d6a8b:     	leaq	0x190(%rbp), %rdx
1407d6a92:     	movaps	%xmm7, %xmm2
1407d6a95:     	callq	0x14129c810 <.text+0x129b810>
1407d6a9a:     	movq	0x200(%rbp), %rcx
1407d6aa1:     	movq	0xf0(%rcx), %rax
1407d6aa8:     	movq	0xf8(%rcx), %rcx
1407d6aaf:     	andq	%rcx, %rdi
1407d6ab2:     	movdqu	(%rax,%rdi), %xmm0
1407d6ab7:     	pmovmskb	%xmm0, %edx
1407d6abb:     	testl	%edx, %edx
1407d6abd:     	je	0x1407d6b7f <.text+0x7d5b7f>
1407d6ac3:     	tzcntl	%edx, %edx
1407d6ac7:     	addq	%rdi, %rdx
1407d6aca:     	andq	%rcx, %rdx
1407d6acd:     	movzbl	(%rax,%rdx), %r8d
1407d6ad2:     	testb	%r8b, %r8b
1407d6ad5:     	jns	0x1407d6ba2 <.text+0x7d5ba2>
1407d6adb:     	leaq	-0x10(%rdx), %r9
1407d6adf:     	andq	%rcx, %r9
1407d6ae2:     	movb	%r13b, (%rax,%rdx)
1407d6ae6:     	movb	%r13b, 0x10(%rax,%r9)
1407d6aeb:     	shlq	$0x4, %rdx
1407d6aef:     	movq	%rax, %rcx
1407d6af2:     	andb	$0x1, %r8b
1407d6af6:     	movzbl	%r8b, %r8d
1407d6afa:     	movq	0x200(%rbp), %r9
1407d6b01:     	subq	%r8, 0x100(%r9)
1407d6b08:     	subq	%rdx, %rcx
1407d6b0b:     	movq	%r14, -0x10(%rcx)
1407d6b0f:     	movb	%r12b, -0x8(%rcx)
1407d6b13:     	movzwl	0x120(%rbp), %r8d
1407d6b1b:     	movw	%r8w, -0x7(%rcx)
1407d6b20:     	movzbl	0x122(%rbp), %r8d
1407d6b28:     	movb	%r8b, -0x5(%rcx)
1407d6b2c:     	negq	%rdx
1407d6b2f:     	movss	%xmm11, -0x4(%rax,%rdx)
1407d6b36:     	incq	0x108(%r9)
1407d6b3d:     	movq	0x200(%rbp), %rax
1407d6b44:     	cmpq	$0x0, 0xa8(%rax)
1407d6b4c:     	jne	0x1407d69f5 <.text+0x7d59f5>
1407d6b52:     	jmp	0x1407d67e8 <.text+0x7d57e8>
1407d6b57:     	movl	$0x1, %edx
1407d6b5c:     	movq	0xa0(%rbp), %rcx
1407d6b63:     	movq	0x1d8(%rbp), %r8
1407d6b6a:     	movb	$0x1, %r9b
1407d6b6d:     	callq	0x1428aed00 <.text+0x28add00>
1407d6b72:     	movzbl	0x198(%rbp), %r12d
1407d6b7a:     	jmp	0x1407d696e <.text+0x7d596e>
1407d6b7f:     	movl	$0x10, %r8d
1407d6b85:     	addq	%r8, %rdi
1407d6b88:     	andq	%rcx, %rdi
1407d6b8b:     	movdqu	(%rax,%rdi), %xmm0
1407d6b90:     	pmovmskb	%xmm0, %edx
1407d6b94:     	addq	$0x10, %r8
1407d6b98:     	testl	%edx, %edx
1407d6b9a:     	jne	0x1407d6ac3 <.text+0x7d5ac3>
1407d6ba0:     	jmp	0x1407d6b85 <.text+0x7d5b85>
1407d6ba2:     	movdqa	(%rax), %xmm0
1407d6ba6:     	pmovmskb	%xmm0, %edx
1407d6baa:     	tzcntl	%edx, %edx
1407d6bae:     	movzbl	(%rax,%rdx), %r8d
1407d6bb3:     	jmp	0x1407d6adb <.text+0x7d5adb>
1407d6bb8:     	cmpb	$0x0, 0x1f0(%rbp)
1407d6bbf:     	je	0x1407d6c22 <.text+0x7d5c22>
1407d6bc1:     	callq	0x140c18130 <.text+0xc17130>
1407d6bc6:     	movq	%rax, %rcx
1407d6bc9:     	movq	(%rax), %rax
1407d6bcc:     	testq	%rax, %rax
1407d6bcf:     	je	0x1407d6be0 <.text+0x7d5be0>
1407d6bd1:     	cmpl	$0x1, %eax
1407d6bd4:     	jne	0x1407d7da3 <.text+0x7d6da3>
1407d6bda:     	addq	$0x8, %rcx
1407d6bde:     	jmp	0x1407d6bf3 <.text+0x7d5bf3>
1407d6be0:     	xorl	%edx, %edx
1407d6be2:     	callq	0x1428f2580 <.text+0x28f1580>
1407d6be7:     	movq	%rax, %rcx
1407d6bea:     	testq	%rax, %rax
1407d6bed:     	je	0x1407d7da3 <.text+0x7d6da3>
1407d6bf3:     	cmpq	$0x0, (%rcx)
1407d6bf7:     	jne	0x1407d7c90 <.text+0x7d6c90>
1407d6bfd:     	movq	$-0x1, (%rcx)
1407d6c04:     	movq	%rcx, 0x1f8(%rbp)
1407d6c0b:     	addq	$0x8, %rcx
1407d6c0f:     	movq	(%rbp), %rdx
1407d6c13:     	callq	0x140c170f0 <.text+0xc160f0>
1407d6c18:     	movq	0x1f8(%rbp), %rax
1407d6c1f:     	incq	(%rax)
1407d6c22:     	callq	0x140c16a00 <.text+0xc15a00>
1407d6c27:     	movl	%eax, %r14d
1407d6c2a:     	testb	%r14b, %r14b
1407d6c2d:     	movq	0x200(%rbp), %rsi
1407d6c34:     	je	0x1407d6db8 <.text+0x7d5db8>
1407d6c3a:     	leaq	0x22b12ff(%rip), %r8    # 0x142a87f40
1407d6c41:     	leaq	0x22b12b8(%rip), %rdi   # 0x142a87f00
1407d6c48:     	nopl	(%rax,%rax)
1407d6c50:     	movsbl	-0x1(%r8), %eax
1407d6c55:     	testl	%eax, %eax
1407d6c57:     	js	0x1407d6c70 <.text+0x7d5c70>
1407d6c59:     	decq	%r8
1407d6c5c:     	cmpl	$0x5c, %eax
1407d6c5f:     	jne	0x1407d6cc4 <.text+0x7d5cc4>
1407d6c61:     	jmp	0x1407d6cd5 <.text+0x7d5cd5>
1407d6c63:     	nopw	%cs:(%rax,%rax)
1407d6c70:     	movzbl	-0x2(%r8), %ecx
1407d6c75:     	cmpb	$-0x40, %cl
1407d6c78:     	jge	0x1407d6c9d <.text+0x7d5c9d>
1407d6c7a:     	movzbl	-0x3(%r8), %edx
1407d6c7f:     	cmpb	$-0x40, %dl
1407d6c82:     	jge	0x1407d6ca6 <.text+0x7d5ca6>
1407d6c84:     	movzbl	-0x4(%r8), %r9d
1407d6c89:     	addq	$-0x4, %r8
1407d6c8d:     	andl	$0x7, %r9d
1407d6c91:     	shll	$0x6, %r9d
1407d6c95:     	andl	$0x3f, %edx
1407d6c98:     	orl	%r9d, %edx
1407d6c9b:     	jmp	0x1407d6cad <.text+0x7d5cad>
1407d6c9d:     	addq	$-0x2, %r8
1407d6ca1:     	andl	$0x1f, %ecx
1407d6ca4:     	jmp	0x1407d6cb5 <.text+0x7d5cb5>
1407d6ca6:     	addq	$-0x3, %r8
1407d6caa:     	andl	$0xf, %edx
1407d6cad:     	shll	$0x6, %edx
1407d6cb0:     	andl	$0x3f, %ecx
1407d6cb3:     	orl	%edx, %ecx
1407d6cb5:     	shll	$0x6, %ecx
1407d6cb8:     	andb	$0x3f, %al
1407d6cba:     	movzbl	%al, %eax
1407d6cbd:     	orl	%ecx, %eax
1407d6cbf:     	cmpl	$0x5c, %eax
1407d6cc2:     	je	0x1407d6cd5 <.text+0x7d5cd5>
1407d6cc4:     	cmpl	$0x2f, %eax
1407d6cc7:     	je	0x1407d6cd5 <.text+0x7d5cd5>
1407d6cc9:     	cmpq	%rdi, %r8
1407d6ccc:     	jne	0x1407d6c50 <.text+0x7d5c50>
1407d6cce:     	movl	$0x40, %ebx
1407d6cd3:     	jmp	0x1407d6d26 <.text+0x7d5d26>
1407d6cd5:     	leaq	0x22b1224(%rip), %rdi   # 0x142a87f00
1407d6cdc:     	subq	%rdi, %r8
1407d6cdf:     	incq	%r8
1407d6ce2:     	je	0x1407d6d1b <.text+0x7d5d1b>
1407d6ce4:     	cmpq	$0x40, %r8
1407d6ce8:     	jae	0x1407d6d19 <.text+0x7d5d19>
1407d6cea:     	cmpb	$-0x41, (%r8,%rdi)
1407d6cef:     	jg	0x1407d6d1b <.text+0x7d5d1b>
1407d6cf1:     	leaq	0x22b0f58(%rip), %rax   # 0x142a87c50
1407d6cf8:     	movq	%rax, 0x20(%rsp)
1407d6cfd:     	leaq	0x22b11fc(%rip), %rcx   # 0x142a87f00
1407d6d04:     	movl	$0x40, %edx
1407d6d09:     	movl	$0x40, %r9d
1407d6d0f:     	callq	0x1428d9940 <.text+0x28d8940>
1407d6d14:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d6d19:     	jne	0x1407d6cf1 <.text+0x7d5cf1>
1407d6d1b:     	movl	$0x40, %ebx
1407d6d20:     	subq	%r8, %rbx
1407d6d23:     	addq	%r8, %rdi
1407d6d26:     	callq	0x140c18130 <.text+0xc17130>
1407d6d2b:     	movq	%rax, %rcx
1407d6d2e:     	movq	(%rax), %rax
1407d6d31:     	testq	%rax, %rax
1407d6d34:     	je	0x1407d6d45 <.text+0x7d5d45>
1407d6d36:     	cmpl	$0x1, %eax
1407d6d39:     	jne	0x1407d7db1 <.text+0x7d6db1>
1407d6d3f:     	addq	$0x8, %rcx
1407d6d43:     	jmp	0x1407d6d5f <.text+0x7d5d5f>
1407d6d45:     	xorl	%edx, %edx
1407d6d47:     	callq	0x1428f2580 <.text+0x28f1580>
1407d6d4c:     	movq	%rax, %rcx
1407d6d4f:     	testq	%rax, %rax
1407d6d52:     	movq	0x200(%rbp), %rsi
1407d6d59:     	je	0x1407d7db1 <.text+0x7d6db1>
1407d6d5f:     	cmpq	$0x0, (%rcx)
1407d6d63:     	jne	0x1407d7ca1 <.text+0x7d6ca1>
1407d6d69:     	movq	$-0x1, (%rcx)
1407d6d70:     	movq	%rcx, 0x1f8(%rbp)
1407d6d77:     	addq	$0x8, %rcx
1407d6d7b:     	movq	%rbx, 0x20(%rsp)
1407d6d80:     	movq	$0x0, 0x30(%rsp)
1407d6d89:     	movq	$0x1, 0x28(%rsp)
1407d6d92:     	leaq	0x22b141f(%rip), %rdx   # 0x142a881b8
1407d6d99:     	movl	$0xe, %r8d
1407d6d9f:     	movq	%rdi, %r9
1407d6da2:     	callq	0x140c16d40 <.text+0xc15d40>
1407d6da7:     	movq	%rax, 0xc0(%rbp)
1407d6dae:     	movq	0x1f8(%rbp), %rax
1407d6db5:     	incq	(%rax)
1407d6db8:     	movq	0xb8(%rbp), %rax
1407d6dbf:     	movq	0x8(%rax), %rdx
1407d6dc3:     	imulq	$0x150, 0x10(%rax), %rcx # imm = 0x150
1407d6dcb:     	addq	%rdx, %rcx
1407d6dce:     	movq	%rcx, -0x18(%rbp)
1407d6dd2:     	movq	-0x28(%rbp), %rcx
1407d6dd6:     	movq	(%rcx), %rcx
1407d6dd9:     	movq	%rcx, 0xa8(%rbp)
1407d6de0:     	addq	$0x20, %rcx
1407d6de4:     	movq	%rcx, -0x10(%rbp)
1407d6de8:     	leaq	0xe0(%rsi), %r13
1407d6def:     	leaq	0x158(%rax), %rax
1407d6df6:     	movq	%rax, 0xb0(%rbp)
1407d6dfd:     	leaq	0x1b0(%rbp), %rsi
1407d6e04:     	pcmpeqd	%xmm7, %xmm7
1407d6e08:     	movaps	0x214eb20(%rip), %xmm8  # 0x142925930
1407d6e10:     	movss	0x22a8e8b(%rip), %xmm9  # 0x142a7fca4
1407d6e19:     	movss	0x22b097a(%rip), %xmm10 # 0x142a8779c
1407d6e22:     	movss	0x22b0975(%rip), %xmm11 # 0x142a877a0
1407d6e2b:     	movss	0x21f6244(%rip), %xmm12 # 0x1429cd078
1407d6e34:     	movss	0x214ea9b(%rip), %xmm13 # 0x1429258d8
1407d6e3d:     	movss	0x21f61ea(%rip), %xmm14 # 0x1429cd030
1407d6e46:     	movb	%r14b, 0x150(%rbp)
1407d6e4d:     	movq	%r13, 0x178(%rbp)
1407d6e54:     	cmpq	-0x18(%rbp), %rdx
1407d6e58:     	je	0x1407d7991 <.text+0x7d6991>
1407d6e5e:     	leaq	0x150(%rdx), %rax
1407d6e65:     	movq	%rax, 0x170(%rbp)
1407d6e6c:     	movq	%rdx, 0x1e0(%rbp)
1407d6e73:     	leaq	0x128(%rdx), %rax
1407d6e7a:     	movq	%rax, 0x1f8(%rbp)
1407d6e81:     	cmpq	$0x0, 0xe8(%rbp)
1407d6e89:     	je	0x1407d6f70 <.text+0x7d5f70>
1407d6e8f:     	leaq	0xf0(%rbp), %rcx
1407d6e96:     	movq	0x1f8(%rbp), %rdx
1407d6e9d:     	callq	0x1405b1be0 <.text+0x5b0be0>
1407d6ea2:     	movq	0xd0(%rbp), %rcx
1407d6ea9:     	movq	0xd8(%rbp), %rdx
1407d6eb0:     	movq	%rdx, %r8
1407d6eb3:     	andq	%rax, %r8
1407d6eb6:     	shrq	$0x39, %rax
1407d6eba:     	movd	%eax, %xmm0
1407d6ebe:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d6ec2:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d6ec7:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d6ecc:     	xorl	%eax, %eax
1407d6ece:     	movdqu	(%rcx,%r8), %xmm1
1407d6ed4:     	movdqa	%xmm1, %xmm2
1407d6ed8:     	pcmpeqb	%xmm0, %xmm2
1407d6edc:     	pmovmskb	%xmm2, %r9d
1407d6ee1:     	testl	%r9d, %r9d
1407d6ee4:     	je	0x1407d6f40 <.text+0x7d5f40>
1407d6ee6:     	movq	0x1f8(%rbp), %r10
1407d6eed:     	movq	(%r10), %xmm2
1407d6ef2:     	tzcntl	%r9d, %r10d
1407d6ef7:     	addq	%r8, %r10
1407d6efa:     	andq	%rdx, %r10
1407d6efd:     	shlq	$0x3, %r10
1407d6f01:     	movq	%rcx, %r11
1407d6f04:     	subq	%r10, %r11
1407d6f07:     	movq	-0x8(%r11), %xmm3
1407d6f0d:     	pcmpeqd	%xmm2, %xmm3
1407d6f11:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d6f16:     	movmskpd	%xmm3, %r10d
1407d6f1b:     	cmpl	$0x3, %r10d
1407d6f1f:     	je	0x1407d7050 <.text+0x7d6050>
1407d6f25:     	leal	-0x1(%r9), %r10d
1407d6f29:     	andw	%r9w, %r10w
1407d6f2d:     	movl	%r10d, %r9d
1407d6f30:     	jne	0x1407d6ef2 <.text+0x7d5ef2>
1407d6f32:     	nopw	%cs:(%rax,%rax)
1407d6f40:     	pcmpeqb	%xmm7, %xmm1
1407d6f44:     	pmovmskb	%xmm1, %r9d
1407d6f49:     	testl	%r9d, %r9d
1407d6f4c:     	jne	0x1407d6f70 <.text+0x7d5f70>
1407d6f4e:     	addq	%rax, %r8
1407d6f51:     	addq	$0x10, %r8
1407d6f55:     	addq	$0x10, %rax
1407d6f59:     	andq	%rdx, %r8
1407d6f5c:     	jmp	0x1407d6ece <.text+0x7d5ece>
1407d6f61:     	nopw	%cs:(%rax,%rax)
1407d6f70:     	movq	0xa8(%rbp), %rax
1407d6f77:     	cmpq	$0x0, 0x18(%rax)
1407d6f7c:     	je	0x1407d77f0 <.text+0x7d67f0>
1407d6f82:     	movq	-0x10(%rbp), %rcx
1407d6f86:     	movq	0x1f8(%rbp), %rdx
1407d6f8d:     	callq	0x1405b1be0 <.text+0x5b0be0>
1407d6f92:     	movq	0xa8(%rbp), %rdx
1407d6f99:     	movq	(%rdx), %rcx
1407d6f9c:     	movq	0x8(%rdx), %rdx
1407d6fa0:     	movq	%rdx, %r8
1407d6fa3:     	andq	%rax, %r8
1407d6fa6:     	shrq	$0x39, %rax
1407d6faa:     	movd	%eax, %xmm0
1407d6fae:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d6fb2:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d6fb7:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d6fbc:     	xorl	%eax, %eax
1407d6fbe:     	movdqu	(%rcx,%r8), %xmm1
1407d6fc4:     	movdqa	%xmm1, %xmm2
1407d6fc8:     	pcmpeqb	%xmm0, %xmm2
1407d6fcc:     	pmovmskb	%xmm2, %r9d
1407d6fd1:     	testl	%r9d, %r9d
1407d6fd4:     	je	0x1407d7020 <.text+0x7d6020>
1407d6fd6:     	movq	0x1f8(%rbp), %r10
1407d6fdd:     	movq	(%r10), %xmm2
1407d6fe2:     	tzcntl	%r9d, %r10d
1407d6fe7:     	addq	%r8, %r10
1407d6fea:     	andq	%rdx, %r10
1407d6fed:     	shlq	$0x3, %r10
1407d6ff1:     	movq	%rcx, %r11
1407d6ff4:     	subq	%r10, %r11
1407d6ff7:     	movq	-0x8(%r11), %xmm3
1407d6ffd:     	pcmpeqd	%xmm2, %xmm3
1407d7001:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d7006:     	movmskpd	%xmm3, %r10d
1407d700b:     	cmpl	$0x3, %r10d
1407d700f:     	je	0x1407d7050 <.text+0x7d6050>
1407d7011:     	leal	-0x1(%r9), %r10d
1407d7015:     	andw	%r9w, %r10w
1407d7019:     	movl	%r10d, %r9d
1407d701c:     	jne	0x1407d6fe2 <.text+0x7d5fe2>
1407d701e:     	nop
1407d7020:     	pcmpeqb	%xmm7, %xmm1
1407d7024:     	pmovmskb	%xmm1, %r9d
1407d7029:     	testl	%r9d, %r9d
1407d702c:     	jne	0x1407d77f0 <.text+0x7d67f0>
1407d7032:     	addq	%rax, %r8
1407d7035:     	addq	$0x10, %r8
1407d7039:     	addq	$0x10, %rax
1407d703d:     	andq	%rdx, %r8
1407d7040:     	jmp	0x1407d6fbe <.text+0x7d5fbe>
1407d7045:     	nopw	%cs:(%rax,%rax)
1407d7050:     	movq	0x1e0(%rbp), %r8
1407d7057:     	movq	0x10(%r8), %rbx
1407d705b:     	movq	0x90(%r8), %r14
1407d7062:     	movq	%rbx, %rdi
1407d7065:     	subq	%r14, %rdi
1407d7068:     	jbe	0x1407d7111 <.text+0x7d6111>
1407d706e:     	leaq	0x80(%r8), %rcx
1407d7075:     	movq	(%rcx), %rdx
1407d7078:     	subq	%r14, %rdx
1407d707b:     	movq	%r14, %rax
1407d707e:     	cmpq	%rdi, %rdx
1407d7081:     	jb	0x1407d7915 <.text+0x7d6915>
1407d7087:     	movq	0x88(%r8), %r10
1407d708e:     	leaq	(%r10,%rax,4), %rcx
1407d7092:     	cmpq	$0x2, %rdi
1407d7096:     	jb	0x1407d7105 <.text+0x7d6105>
1407d7098:     	movq	%r14, %r8
1407d709b:     	notq	%r8
1407d709e:     	addq	%rbx, %r8
1407d70a1:     	movl	$0x1, %edx
1407d70a6:     	cmpq	$0x7, %r8
1407d70aa:     	jbe	0x1407d70e9 <.text+0x7d60e9>
1407d70ac:     	movq	%r8, %r9
1407d70af:     	andq	$-0x8, %r9
1407d70b3:     	leaq	(%rcx,%r9,4), %rcx
1407d70b7:     	leaq	0x1(%r9), %rdx
1407d70bb:     	leaq	(%r10,%rax,4), %r10
1407d70bf:     	addq	$0x10, %r10
1407d70c3:     	xorl	%r11d, %r11d
1407d70c6:     	nopw	%cs:(%rax,%rax)
1407d70d0:     	movups	%xmm8, -0x10(%r10,%r11,4)
1407d70d6:     	movups	%xmm8, (%r10,%r11,4)
1407d70db:     	addq	$0x8, %r11
1407d70df:     	cmpq	%r11, %r9
1407d70e2:     	jne	0x1407d70d0 <.text+0x7d60d0>
1407d70e4:     	cmpq	%r9, %r8
1407d70e7:     	je	0x1407d70ff <.text+0x7d60ff>
1407d70e9:     	addq	%r14, %rdx
1407d70ec:     	subq	%rbx, %rdx
1407d70ef:     	nop
1407d70f0:     	movl	$0x3f800000, (%rcx)     # imm = 0x3F800000
1407d70f6:     	addq	$0x4, %rcx
1407d70fa:     	incq	%rdx
1407d70fd:     	jne	0x1407d70f0 <.text+0x7d60f0>
1407d70ff:     	addq	%rdi, %rax
1407d7102:     	decq	%rax
1407d7105:     	movl	$0x3f800000, (%rcx)     # imm = 0x3F800000
1407d710b:     	incq	%rax
1407d710e:     	movq	%rax, %rbx
1407d7111:     	movq	0x1e0(%rbp), %rax
1407d7118:     	movq	%rbx, 0x90(%rax)
1407d711f:     	movq	0x88(%rax), %r14
1407d7126:     	leaq	(%r14,%rbx,4), %rax
1407d712a:     	movq	%rax, 0x1f0(%rbp)
1407d7131:     	xorl	%r15d, %r15d
1407d7134:     	jmp	0x1407d714c <.text+0x7d614c>
1407d7136:     	nopw	%cs:(%rax,%rax)
1407d7140:     	incl	%r15d
1407d7143:     	movss	%xmm0, (%r14)
1407d7148:     	addq	$0x4, %r14
1407d714c:     	cmpq	0x1f0(%rbp), %r14
1407d7153:     	je	0x1407d7650 <.text+0x7d6650>
1407d7159:     	movq	0x1f8(%rbp), %rax
1407d7160:     	movq	(%rax), %rax
1407d7163:     	movq	%rax, 0x1b0(%rbp)
1407d716a:     	movl	%r15d, 0x1b8(%rbp)
1407d7171:     	movq	0x200(%rbp), %rax
1407d7178:     	cmpq	$0x0, 0xd8(%rax)
1407d7180:     	je	0x1407d73b0 <.text+0x7d63b0>
1407d7186:     	movq	%r13, %rcx
1407d7189:     	movq	%rsi, %rdx
1407d718c:     	callq	0x141256660 <.text+0x1255660>
1407d7191:     	movq	0x200(%rbp), %rcx
1407d7198:     	movq	0xc0(%rcx), %r12
1407d719f:     	movq	0xc8(%rcx), %rcx
1407d71a6:     	movq	%rcx, %rdx
1407d71a9:     	andq	%rax, %rdx
1407d71ac:     	shrq	$0x39, %rax
1407d71b0:     	movd	%eax, %xmm0
1407d71b4:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d71b8:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d71bd:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d71c2:     	leaq	-0x18(%r12), %rax
1407d71c7:     	movq	0x1b0(%rbp), %xmm1
1407d71cf:     	movl	0x1b8(%rbp), %r8d
1407d71d6:     	xorl	%r9d, %r9d
1407d71d9:     	movdqu	(%r12,%rdx), %xmm2
1407d71df:     	movdqa	%xmm2, %xmm3
1407d71e3:     	pcmpeqb	%xmm0, %xmm3
1407d71e7:     	pmovmskb	%xmm3, %r10d
1407d71ec:     	testl	%r10d, %r10d
1407d71ef:     	je	0x1407d7240 <.text+0x7d6240>
1407d71f1:     	tzcntl	%r10d, %r11d
1407d71f6:     	addq	%rdx, %r11
1407d71f9:     	andq	%rcx, %r11
1407d71fc:     	negq	%r11
1407d71ff:     	leaq	(%r11,%r11,2), %rbx
1407d7203:     	movq	(%rax,%rbx,8), %xmm3
1407d7208:     	pcmpeqd	%xmm1, %xmm3
1407d720c:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d7211:     	movmskpd	%xmm3, %r11d
1407d7216:     	cmpl	$0x3, %r11d
1407d721a:     	jne	0x1407d7226 <.text+0x7d6226>
1407d721c:     	leaq	(%rax,%rbx,8), %r11
1407d7220:     	cmpl	0x8(%r11), %r8d
1407d7224:     	je	0x1407d7270 <.text+0x7d6270>
1407d7226:     	leal	-0x1(%r10), %r11d
1407d722a:     	andw	%r10w, %r11w
1407d722e:     	movl	%r11d, %r10d
1407d7231:     	jne	0x1407d71f1 <.text+0x7d61f1>
1407d7233:     	nopw	%cs:(%rax,%rax)
1407d7240:     	pcmpeqb	%xmm7, %xmm2
1407d7244:     	pmovmskb	%xmm2, %r10d
1407d7249:     	testl	%r10d, %r10d
1407d724c:     	jne	0x1407d73b0 <.text+0x7d63b0>
1407d7252:     	addq	%r9, %rdx
1407d7255:     	addq	$0x10, %rdx
1407d7259:     	addq	$0x10, %r9
1407d725d:     	andq	%rcx, %rdx
1407d7260:     	jmp	0x1407d71d9 <.text+0x7d61d9>
1407d7265:     	nopw	%cs:(%rax,%rax)
1407d7270:     	movq	0x200(%rbp), %rax
1407d7277:     	cmpq	$0x0, 0x108(%rax)
1407d727f:     	je	0x1407d73b0 <.text+0x7d63b0>
1407d7285:     	leaq	(%r12,%rbx,8), %rdi
1407d7289:     	addq	$-0xc, %rdi
1407d728d:     	movq	0x1d8(%rbp), %rcx
1407d7294:     	movq	%rdi, %rdx
1407d7297:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d729c:     	leaq	(%r12,%rbx,8), %rdx
1407d72a0:     	movq	0x200(%rbp), %r8
1407d72a7:     	movq	0xf0(%r8), %rcx
1407d72ae:     	movq	0xf8(%r8), %r8
1407d72b5:     	movq	%r8, %r9
1407d72b8:     	andq	%rax, %r9
1407d72bb:     	shrq	$0x39, %rax
1407d72bf:     	movd	%eax, %xmm0
1407d72c3:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d72c7:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d72cc:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d72d1:     	leaq	-0x10(%rcx), %rax
1407d72d5:     	xorl	%r10d, %r10d
1407d72d8:     	movdqu	(%rcx,%r9), %xmm1
1407d72de:     	movdqa	%xmm1, %xmm2
1407d72e2:     	pcmpeqb	%xmm0, %xmm2
1407d72e6:     	pmovmskb	%xmm2, %r11d
1407d72eb:     	testl	%r11d, %r11d
1407d72ee:     	je	0x1407d7350 <.text+0x7d6350>
1407d72f0:     	movzbl	-0x4(%rdx), %r12d
1407d72f5:     	tzcntl	%r11d, %ebx
1407d72fa:     	addq	%r9, %rbx
1407d72fd:     	andq	%r8, %rbx
1407d7300:     	movq	%rbx, %r13
1407d7303:     	shlq	$0x4, %r13
1407d7307:     	movq	%rax, %rsi
1407d730a:     	subq	%r13, %rsi
1407d730d:     	cmpb	0x8(%rsi), %r12b
1407d7311:     	jne	0x1407d732d <.text+0x7d632d>
1407d7313:     	movq	(%rdi), %xmm2
1407d7317:     	movq	(%rsi), %xmm3
1407d731b:     	pcmpeqd	%xmm2, %xmm3
1407d731f:     	pshufd	$0x50, %xmm3, %xmm2     # xmm2 = xmm3[0,0,1,1]
1407d7324:     	movmskpd	%xmm2, %esi
1407d7328:     	cmpl	$0x3, %esi
1407d732b:     	je	0x1407d7371 <.text+0x7d6371>
1407d732d:     	leal	-0x1(%r11), %esi
1407d7331:     	andw	%r11w, %si
1407d7335:     	movl	%esi, %r11d
1407d7338:     	movq	0x178(%rbp), %r13
1407d733f:     	leaq	0x1b0(%rbp), %rsi
1407d7346:     	jne	0x1407d72f5 <.text+0x7d62f5>
1407d7348:     	nopl	(%rax,%rax)
1407d7350:     	pcmpeqb	%xmm7, %xmm1
1407d7354:     	pmovmskb	%xmm1, %r11d
1407d7359:     	testl	%r11d, %r11d
1407d735c:     	jne	0x1407d73b0 <.text+0x7d63b0>
1407d735e:     	addq	%r10, %r9
1407d7361:     	addq	$0x10, %r9
1407d7365:     	addq	$0x10, %r10
1407d7369:     	andq	%r8, %r9
1407d736c:     	jmp	0x1407d72d8 <.text+0x7d62d8>
1407d7371:     	negq	%rbx
1407d7374:     	shlq	$0x4, %rbx
1407d7378:     	movss	-0x4(%rcx,%rbx), %xmm0
1407d737e:     	movss	(%r14), %xmm1
1407d7383:     	movaps	%xmm1, %xmm2
1407d7386:     	cmpunordss	%xmm1, %xmm2
1407d738b:     	movaps	%xmm2, %xmm3
1407d738e:     	andps	%xmm0, %xmm3
1407d7391:     	minss	%xmm1, %xmm0
1407d7395:     	andnps	%xmm0, %xmm2
1407d7398:     	orps	%xmm3, %xmm2
1407d739b:     	movss	%xmm2, (%r14)
1407d73a0:     	movq	0x178(%rbp), %r13
1407d73a7:     	leaq	0x1b0(%rbp), %rsi
1407d73ae:     	nop
1407d73b0:     	leal	-0x1(%r15), %eax
1407d73b4:     	movq	0x1f8(%rbp), %rcx
1407d73bb:     	movq	(%rcx), %rcx
1407d73be:     	movq	%rcx, 0x1b0(%rbp)
1407d73c5:     	movl	%eax, 0x1b8(%rbp)
1407d73cb:     	movq	0x200(%rbp), %rax
1407d73d2:     	cmpq	$0x0, 0xd8(%rax)
1407d73da:     	je	0x1407d7600 <.text+0x7d6600>
1407d73e0:     	movq	%r13, %rcx
1407d73e3:     	movq	%rsi, %rdx
1407d73e6:     	callq	0x141256660 <.text+0x1255660>
1407d73eb:     	movq	0x200(%rbp), %rcx
1407d73f2:     	movq	0xc0(%rcx), %r12
1407d73f9:     	movq	0xc8(%rcx), %rcx
1407d7400:     	movq	%rcx, %rdx
1407d7403:     	andq	%rax, %rdx
1407d7406:     	shrq	$0x39, %rax
1407d740a:     	movd	%eax, %xmm0
1407d740e:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d7412:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d7417:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d741c:     	leaq	-0x18(%r12), %rax
1407d7421:     	movq	0x1b0(%rbp), %xmm1
1407d7429:     	movl	0x1b8(%rbp), %r8d
1407d7430:     	xorl	%r9d, %r9d
1407d7433:     	movdqu	(%r12,%rdx), %xmm2
1407d7439:     	movdqa	%xmm2, %xmm3
1407d743d:     	pcmpeqb	%xmm0, %xmm3
1407d7441:     	pmovmskb	%xmm3, %r10d
1407d7446:     	testl	%r10d, %r10d
1407d7449:     	je	0x1407d7490 <.text+0x7d6490>
1407d744b:     	tzcntl	%r10d, %r11d
1407d7450:     	addq	%rdx, %r11
1407d7453:     	andq	%rcx, %r11
1407d7456:     	negq	%r11
1407d7459:     	leaq	(%r11,%r11,2), %rbx
1407d745d:     	movq	(%rax,%rbx,8), %xmm3
1407d7462:     	pcmpeqd	%xmm1, %xmm3
1407d7466:     	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1407d746b:     	movmskpd	%xmm3, %r11d
1407d7470:     	cmpl	$0x3, %r11d
1407d7474:     	jne	0x1407d7480 <.text+0x7d6480>
1407d7476:     	leaq	(%rax,%rbx,8), %r11
1407d747a:     	cmpl	0x8(%r11), %r8d
1407d747e:     	je	0x1407d74c0 <.text+0x7d64c0>
1407d7480:     	leal	-0x1(%r10), %r11d
1407d7484:     	andw	%r10w, %r11w
1407d7488:     	movl	%r11d, %r10d
1407d748b:     	jne	0x1407d744b <.text+0x7d644b>
1407d748d:     	nopl	(%rax)
1407d7490:     	pcmpeqb	%xmm7, %xmm2
1407d7494:     	pmovmskb	%xmm2, %r10d
1407d7499:     	testl	%r10d, %r10d
1407d749c:     	jne	0x1407d7600 <.text+0x7d6600>
1407d74a2:     	addq	%r9, %rdx
1407d74a5:     	addq	$0x10, %rdx
1407d74a9:     	addq	$0x10, %r9
1407d74ad:     	andq	%rcx, %rdx
1407d74b0:     	jmp	0x1407d7433 <.text+0x7d6433>
1407d74b2:     	nopw	%cs:(%rax,%rax)
1407d74c0:     	movq	0x200(%rbp), %rax
1407d74c7:     	cmpq	$0x0, 0x108(%rax)
1407d74cf:     	je	0x1407d7600 <.text+0x7d6600>
1407d74d5:     	leaq	(%r12,%rbx,8), %rdi
1407d74d9:     	addq	$-0xc, %rdi
1407d74dd:     	movq	0x1d8(%rbp), %rcx
1407d74e4:     	movq	%rdi, %rdx
1407d74e7:     	callq	0x141256ca0 <.text+0x1255ca0>
1407d74ec:     	leaq	(%r12,%rbx,8), %rdx
1407d74f0:     	movq	0x200(%rbp), %r8
1407d74f7:     	movq	0xf0(%r8), %rcx
1407d74fe:     	movq	0xf8(%r8), %r8
1407d7505:     	movq	%r8, %r9
1407d7508:     	andq	%rax, %r9
1407d750b:     	shrq	$0x39, %rax
1407d750f:     	movd	%eax, %xmm0
1407d7513:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d7517:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d751c:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d7521:     	leaq	-0x10(%rcx), %rax
1407d7525:     	xorl	%r10d, %r10d
1407d7528:     	movdqu	(%rcx,%r9), %xmm1
1407d752e:     	movdqa	%xmm1, %xmm2
1407d7532:     	pcmpeqb	%xmm0, %xmm2
1407d7536:     	pmovmskb	%xmm2, %r11d
1407d753b:     	testl	%r11d, %r11d
1407d753e:     	je	0x1407d75a0 <.text+0x7d65a0>
1407d7540:     	movzbl	-0x4(%rdx), %r12d
1407d7545:     	tzcntl	%r11d, %ebx
1407d754a:     	addq	%r9, %rbx
1407d754d:     	andq	%r8, %rbx
1407d7550:     	movq	%rbx, %r13
1407d7553:     	shlq	$0x4, %r13
1407d7557:     	movq	%rax, %rsi
1407d755a:     	subq	%r13, %rsi
1407d755d:     	cmpb	0x8(%rsi), %r12b
1407d7561:     	jne	0x1407d757d <.text+0x7d657d>
1407d7563:     	movq	(%rdi), %xmm2
1407d7567:     	movq	(%rsi), %xmm3
1407d756b:     	pcmpeqd	%xmm2, %xmm3
1407d756f:     	pshufd	$0x50, %xmm3, %xmm2     # xmm2 = xmm3[0,0,1,1]
1407d7574:     	movmskpd	%xmm2, %esi
1407d7578:     	cmpl	$0x3, %esi
1407d757b:     	je	0x1407d75c1 <.text+0x7d65c1>
1407d757d:     	leal	-0x1(%r11), %esi
1407d7581:     	andw	%r11w, %si
1407d7585:     	movl	%esi, %r11d
1407d7588:     	movq	0x178(%rbp), %r13
1407d758f:     	leaq	0x1b0(%rbp), %rsi
1407d7596:     	jne	0x1407d7545 <.text+0x7d6545>
1407d7598:     	nopl	(%rax,%rax)
1407d75a0:     	pcmpeqb	%xmm7, %xmm1
1407d75a4:     	pmovmskb	%xmm1, %r11d
1407d75a9:     	testl	%r11d, %r11d
1407d75ac:     	jne	0x1407d7600 <.text+0x7d6600>
1407d75ae:     	addq	%r10, %r9
1407d75b1:     	addq	$0x10, %r9
1407d75b5:     	addq	$0x10, %r10
1407d75b9:     	andq	%r8, %r9
1407d75bc:     	jmp	0x1407d7528 <.text+0x7d6528>
1407d75c1:     	negq	%rbx
1407d75c4:     	shlq	$0x4, %rbx
1407d75c8:     	movss	-0x4(%rcx,%rbx), %xmm0
1407d75ce:     	movss	(%r14), %xmm1
1407d75d3:     	movaps	%xmm1, %xmm2
1407d75d6:     	cmpunordss	%xmm1, %xmm2
1407d75db:     	movaps	%xmm2, %xmm3
1407d75de:     	andps	%xmm0, %xmm3
1407d75e1:     	minss	%xmm1, %xmm0
1407d75e5:     	andnps	%xmm0, %xmm2
1407d75e8:     	orps	%xmm3, %xmm2
1407d75eb:     	movss	%xmm2, (%r14)
1407d75f0:     	movq	0x178(%rbp), %r13
1407d75f7:     	leaq	0x1b0(%rbp), %rsi
1407d75fe:     	nop
1407d7600:     	movss	(%r14), %xmm1
1407d7605:     	xorps	%xmm0, %xmm0
1407d7608:     	ucomiss	%xmm1, %xmm9
1407d760c:     	ja	0x1407d7140 <.text+0x7d6140>
1407d7612:     	addss	%xmm10, %xmm1
1407d7617:     	divss	%xmm11, %xmm1
1407d761c:     	maxss	%xmm1, %xmm0
1407d7620:     	movaps	%xmm6, %xmm1
1407d7623:     	minss	%xmm0, %xmm1
1407d7627:     	movaps	%xmm1, %xmm2
1407d762a:     	mulss	%xmm1, %xmm2
1407d762e:     	addss	%xmm1, %xmm1
1407d7632:     	movaps	%xmm12, %xmm0
1407d7636:     	subss	%xmm1, %xmm0
1407d763a:     	mulss	%xmm2, %xmm0
1407d763e:     	mulss	%xmm13, %xmm0
1407d7643:     	addss	%xmm14, %xmm0
1407d7648:     	jmp	0x1407d7140 <.text+0x7d6140>
1407d764d:     	nopl	(%rax)
1407d7650:     	movq	0x1e0(%rbp), %rax
1407d7657:     	movl	0x128(%rax), %ecx
1407d765d:     	movl	0x12c(%rax), %edx
1407d7663:     	callq	0x140959e70 <.text+0x958e70>
1407d7668:     	movq	%rax, %rdi
1407d766b:     	movq	%rax, 0x1b0(%rbp)
1407d7672:     	movq	0xb8(%rbp), %rsi
1407d7679:     	cmpq	$0x0, 0x150(%rsi)
1407d7681:     	je	0x1407d7980 <.text+0x7d6980>
1407d7687:     	movq	0xb0(%rbp), %rcx
1407d768e:     	leaq	0x1b0(%rbp), %rdx
1407d7695:     	callq	0x1405b0d70 <.text+0x5afd70>
1407d769a:     	movq	0x138(%rsi), %rcx
1407d76a1:     	movq	0x140(%rsi), %rdx
1407d76a8:     	movq	%rdx, %r8
1407d76ab:     	andq	%rax, %r8
1407d76ae:     	shrq	$0x39, %rax
1407d76b2:     	movd	%eax, %xmm0
1407d76b6:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d76ba:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d76bf:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d76c4:     	leaq	-0x78(%rcx), %rax
1407d76c8:     	xorl	%r9d, %r9d
1407d76cb:     	movdqu	(%rcx,%r8), %xmm1
1407d76d1:     	movdqa	%xmm1, %xmm2
1407d76d5:     	pcmpeqb	%xmm0, %xmm2
1407d76d9:     	pmovmskb	%xmm2, %r11d
1407d76de:     	testl	%r11d, %r11d
1407d76e1:     	movq	0x1e0(%rbp), %rsi
1407d76e8:     	je	0x1407d7710 <.text+0x7d6710>
1407d76ea:     	tzcntl	%r11d, %r10d
1407d76ef:     	addq	%r8, %r10
1407d76f2:     	andq	%rdx, %r10
1407d76f5:     	negq	%r10
1407d76f8:     	imulq	$0x78, %r10, %r10
1407d76fc:     	cmpq	(%rax,%r10), %rdi
1407d7700:     	je	0x1407d7740 <.text+0x7d6740>
1407d7702:     	leal	-0x1(%r11), %r10d
1407d7706:     	andw	%r11w, %r10w
1407d770a:     	movl	%r10d, %r11d
1407d770d:     	jne	0x1407d76ea <.text+0x7d66ea>
1407d770f:     	nop
1407d7710:     	pcmpeqb	%xmm7, %xmm1
1407d7714:     	pmovmskb	%xmm1, %r10d
1407d7719:     	testl	%r10d, %r10d
1407d771c:     	jne	0x1407d7980 <.text+0x7d6980>
1407d7722:     	addq	%r9, %r8
1407d7725:     	addq	$0x10, %r8
1407d7729:     	addq	$0x10, %r9
1407d772d:     	andq	%rdx, %r8
1407d7730:     	jmp	0x1407d76cb <.text+0x7d66cb>
1407d7732:     	nopw	%cs:(%rax,%rax)
1407d7740:     	movq	0x88(%rsi), %r14
1407d7747:     	movq	0x90(%rsi), %rdi
1407d774e:     	cmpq	-0x18(%rcx,%r10), %rdi
1407d7753:     	leaq	0x1b0(%rbp), %rsi
1407d775a:     	jne	0x1407d778b <.text+0x7d678b>
1407d775c:     	movq	-0x20(%rcx,%r10), %rax
1407d7761:     	xorl	%edx, %edx
1407d7763:     	nopw	%cs:(%rax,%rax)
1407d7770:     	cmpq	%rdx, %rdi
1407d7773:     	je	0x1407d7901 <.text+0x7d6901>
1407d7779:     	movss	(%r14,%rdx,4), %xmm0
1407d777f:     	ucomiss	(%rax,%rdx,4), %xmm0
1407d7783:     	leaq	0x1(%rdx), %rdx
1407d7787:     	jne	0x1407d778b <.text+0x7d678b>
1407d7789:     	jnp	0x1407d7770 <.text+0x7d6770>
1407d778b:     	leaq	(%rcx,%r10), %rbx
1407d778f:     	movq	0x1e0(%rbp), %rax
1407d7796:     	movb	$0x1, 0x148(%rax)
1407d779d:     	movq	$0x0, -0x18(%rbx)
1407d77a5:     	cmpq	%rdi, -0x28(%rbx)
1407d77a9:     	jb	0x1407d7957 <.text+0x7d6957>
1407d77af:     	xorl	%esi, %esi
1407d77b1:     	leaq	(,%rdi,4), %r8
1407d77b9:     	leaq	(,%rsi,4), %rcx
1407d77c1:     	addq	-0x20(%rbx), %rcx
1407d77c5:     	movq	%r14, %rdx
1407d77c8:     	callq	0x142923c00 <.text+0x2922c00>
1407d77cd:     	addq	%rdi, %rsi
1407d77d0:     	movq	%rsi, -0x18(%rbx)
1407d77d4:     	movq	0x170(%rbp), %rdx
1407d77db:     	movzbl	0x150(%rbp), %r14d
1407d77e3:     	leaq	0x1b0(%rbp), %rsi
1407d77ea:     	jmp	0x1407d6e54 <.text+0x7d5e54>
1407d77ef:     	nop
1407d77f0:     	movq	0x1e0(%rbp), %rax
1407d77f7:     	movl	0x128(%rax), %ecx
1407d77fd:     	movl	0x12c(%rax), %edx
1407d7803:     	callq	0x140959e70 <.text+0x958e70>
1407d7808:     	movq	%rax, %rdi
1407d780b:     	movq	%rax, 0x1b0(%rbp)
1407d7812:     	movq	0xb8(%rbp), %rax
1407d7819:     	cmpq	$0x0, 0x150(%rax)
1407d7821:     	movzbl	0x150(%rbp), %r14d
1407d7829:     	je	0x1407d78e0 <.text+0x7d68e0>
1407d782f:     	movq	0xb0(%rbp), %rcx
1407d7836:     	movq	%rsi, %rdx
1407d7839:     	callq	0x1405b0d70 <.text+0x5afd70>
1407d783e:     	movq	0xb8(%rbp), %rdx
1407d7845:     	movq	0x138(%rdx), %rcx
1407d784c:     	movq	0x140(%rdx), %rdx
1407d7853:     	movq	%rdx, %r8
1407d7856:     	andq	%rax, %r8
1407d7859:     	shrq	$0x39, %rax
1407d785d:     	movd	%eax, %xmm0
1407d7861:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407d7865:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407d786a:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407d786f:     	leaq	-0x78(%rcx), %rax
1407d7873:     	xorl	%r9d, %r9d
1407d7876:     	movdqu	(%rcx,%r8), %xmm1
1407d787c:     	movdqa	%xmm1, %xmm2
1407d7880:     	pcmpeqb	%xmm0, %xmm2
1407d7884:     	pmovmskb	%xmm2, %r10d
1407d7889:     	testl	%r10d, %r10d
1407d788c:     	je	0x1407d78c0 <.text+0x7d68c0>
1407d788e:     	tzcntl	%r10d, %r11d
1407d7893:     	addq	%r8, %r11
1407d7896:     	andq	%rdx, %r11
1407d7899:     	negq	%r11
1407d789c:     	imulq	$0x78, %r11, %r11
1407d78a0:     	cmpq	(%rax,%r11), %rdi
1407d78a4:     	je	0x1407d78ec <.text+0x7d68ec>
1407d78a6:     	leal	-0x1(%r10), %r11d
1407d78aa:     	andw	%r10w, %r11w
1407d78ae:     	movl	%r11d, %r10d
1407d78b1:     	jne	0x1407d788e <.text+0x7d688e>
1407d78b3:     	nopw	%cs:(%rax,%rax)
1407d78c0:     	pcmpeqb	%xmm7, %xmm1
1407d78c4:     	pmovmskb	%xmm1, %r10d
1407d78c9:     	testl	%r10d, %r10d
1407d78cc:     	jne	0x1407d78e0 <.text+0x7d68e0>
1407d78ce:     	addq	%r9, %r8
1407d78d1:     	addq	$0x10, %r8
1407d78d5:     	addq	$0x10, %r9
1407d78d9:     	andq	%rdx, %r8
1407d78dc:     	jmp	0x1407d7876 <.text+0x7d6876>
1407d78de:     	nop
1407d78e0:     	movq	0x170(%rbp), %rdx
1407d78e7:     	jmp	0x1407d6e54 <.text+0x7d5e54>
1407d78ec:     	movq	$0x0, -0x18(%rcx,%r11)
1407d78f5:     	movq	0x170(%rbp), %rdx
1407d78fc:     	jmp	0x1407d6e54 <.text+0x7d5e54>
1407d7901:     	movq	0x170(%rbp), %rdx
1407d7908:     	movzbl	0x150(%rbp), %r14d
1407d7910:     	jmp	0x1407d6e54 <.text+0x7d5e54>
1407d7915:     	movq	$0x4, 0x20(%rsp)
1407d791e:     	movl	$0x4, %r9d
1407d7924:     	movq	%r14, %rdx
1407d7927:     	movq	%rdi, %r8
1407d792a:     	callq	0x1428a7f20 <.text+0x28a6f20>
1407d792f:     	movq	0x1e0(%rbp), %r8
1407d7936:     	movq	0x90(%r8), %rax
1407d793d:     	movq	0x88(%r8), %r10
1407d7944:     	leaq	(%r10,%rax,4), %rcx
1407d7948:     	cmpq	$0x2, %rdi
1407d794c:     	jae	0x1407d7098 <.text+0x7d6098>
1407d7952:     	jmp	0x1407d7105 <.text+0x7d6105>
1407d7957:     	addq	%r10, %rcx
1407d795a:     	addq	$-0x28, %rcx
1407d795e:     	movq	$0x4, 0x20(%rsp)
1407d7967:     	movl	$0x4, %r9d
1407d796d:     	xorl	%edx, %edx
1407d796f:     	movq	%rdi, %r8
1407d7972:     	callq	0x1428a7f20 <.text+0x28a6f20>
1407d7977:     	movq	-0x18(%rbx), %rsi
1407d797b:     	jmp	0x1407d77b1 <.text+0x7d67b1>
1407d7980:     	leaq	0x22b0841(%rip), %rcx   # 0x142a881c8
1407d7987:     	callq	0x1428d9310 <.text+0x28d8310>
1407d798c:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7991:     	testb	%r14b, %r14b
1407d7994:     	je	0x1407d79fa <.text+0x7d69fa>
1407d7996:     	callq	0x140c18130 <.text+0xc17130>
1407d799b:     	movq	%rax, %rcx
1407d799e:     	movq	(%rax), %rax
1407d79a1:     	testq	%rax, %rax
1407d79a4:     	je	0x1407d79b5 <.text+0x7d69b5>
1407d79a6:     	cmpl	$0x1, %eax
1407d79a9:     	jne	0x1407d7ddb <.text+0x7d6ddb>
1407d79af:     	addq	$0x8, %rcx
1407d79b3:     	jmp	0x1407d79c8 <.text+0x7d69c8>
1407d79b5:     	xorl	%edx, %edx
1407d79b7:     	callq	0x1428f2580 <.text+0x28f1580>
1407d79bc:     	movq	%rax, %rcx
1407d79bf:     	testq	%rax, %rax
1407d79c2:     	je	0x1407d7ddb <.text+0x7d6ddb>
1407d79c8:     	cmpq	$0x0, (%rcx)
1407d79cc:     	jne	0x1407d7ceb <.text+0x7d6ceb>
1407d79d2:     	movq	$-0x1, (%rcx)
1407d79d9:     	movq	%rcx, 0x1f8(%rbp)
1407d79e0:     	addq	$0x8, %rcx
1407d79e4:     	movq	0xc0(%rbp), %rdx
1407d79eb:     	callq	0x140c170f0 <.text+0xc160f0>
1407d79f0:     	movq	0x1f8(%rbp), %rax
1407d79f7:     	incq	(%rax)
1407d79fa:     	movq	-0x20(%rbp), %rcx
1407d79fe:     	movq	0xa0(%rbp), %rdx
1407d7a05:     	callq	0x141263970 <.text+0x1262970>
1407d7a0a:     	movq	0x200(%rbp), %rdi
1407d7a11:     	movups	0x110(%rdi), %xmm0
1407d7a18:     	movups	%xmm0, 0x140(%rdi)
1407d7a1f:     	cmpq	$0x0, 0x168(%rdi)
1407d7a27:     	je	0x1407d7a76 <.text+0x7d6a76>
1407d7a29:     	movq	0x158(%rdi), %rsi
1407d7a30:     	testq	%rsi, %rsi
1407d7a33:     	je	0x1407d7a4a <.text+0x7d6a4a>
1407d7a35:     	movq	0x138(%rbp), %rax
1407d7a3c:     	movq	(%rax), %rcx
1407d7a3f:     	leaq	0x11(%rsi), %r8
1407d7a43:     	movb	$-0x1, %dl
1407d7a45:     	callq	0x142923c20 <.text+0x2922c20>
1407d7a4a:     	movq	$0x0, 0x168(%rdi)
1407d7a55:     	leaq	0x1(%rsi), %rax
1407d7a59:     	movq	%rax, %rcx
1407d7a5c:     	shrq	$0x3, %rcx
1407d7a60:     	andq	$-0x8, %rax
1407d7a64:     	subq	%rcx, %rax
1407d7a67:     	cmpq	$0x8, %rsi
1407d7a6b:     	cmovbq	%rsi, %rax
1407d7a6f:     	movq	%rax, 0x160(%rdi)
1407d7a76:     	movq	0xd8(%rbp), %rdx
1407d7a7d:     	testq	%rdx, %rdx
1407d7a80:     	je	0x1407d7aac <.text+0x7d6aac>
1407d7a82:     	leaq	0x17(,%rdx,8), %rax
1407d7a8a:     	andq	$-0x10, %rax
1407d7a8e:     	addq	%rax, %rdx
1407d7a91:     	addq	$0x11, %rdx
1407d7a95:     	je	0x1407d7aac <.text+0x7d6aac>
1407d7a97:     	movq	0xd0(%rbp), %rcx
1407d7a9e:     	subq	%rax, %rcx
1407d7aa1:     	movl	$0x10, %r8d
1407d7aa7:     	callq	0x140613c20 <.text+0x612c20>
1407d7aac:     	movq	0x30(%rbp), %rdx
1407d7ab0:     	testq	%rdx, %rdx
1407d7ab3:     	je	0x1407d7adc <.text+0x7d6adc>
1407d7ab5:     	leaq	0x17(,%rdx,8), %rax
1407d7abd:     	andq	$-0x10, %rax
1407d7ac1:     	addq	%rax, %rdx
1407d7ac4:     	addq	$0x11, %rdx
1407d7ac8:     	je	0x1407d7adc <.text+0x7d6adc>
1407d7aca:     	movq	0x28(%rbp), %rcx
1407d7ace:     	subq	%rax, %rcx
1407d7ad1:     	movl	$0x10, %r8d
1407d7ad7:     	callq	0x140613c20 <.text+0x612c20>
1407d7adc:     	movq	0x10(%rbp), %rax
1407d7ae0:     	testq	%rax, %rax
1407d7ae3:     	je	0x1407d7afc <.text+0x7d6afc>
1407d7ae5:     	movq	0x18(%rbp), %rcx
1407d7ae9:     	shlq	$0x3, %rax
1407d7aed:     	leaq	(%rax,%rax,2), %rdx
1407d7af1:     	movl	$0x8, %r8d
1407d7af7:     	callq	0x140613c20 <.text+0x612c20>
1407d7afc:     	movq	0x58(%rbp), %rdx
1407d7b00:     	testq	%rdx, %rdx
1407d7b03:     	je	0x1407d7b18 <.text+0x7d6b18>
1407d7b05:     	movq	0x60(%rbp), %rcx
1407d7b09:     	shlq	$0x3, %rdx
1407d7b0d:     	movl	$0x8, %r8d
1407d7b13:     	callq	0x140613c20 <.text+0x612c20>
1407d7b18:     	movq	0x70(%rbp), %rdx
1407d7b1c:     	testq	%rdx, %rdx
1407d7b1f:     	je	0x1407d7b34 <.text+0x7d6b34>
1407d7b21:     	movq	0x78(%rbp), %rcx
1407d7b25:     	shlq	$0x3, %rdx
1407d7b29:     	movl	$0x8, %r8d
1407d7b2f:     	callq	0x140613c20 <.text+0x612c20>
1407d7b34:     	cmpb	$0x0, 0x1ef(%rbp)
1407d7b3b:     	je	0x1407d7b88 <.text+0x7d6b88>
1407d7b3d:     	callq	0x140c18130 <.text+0xc17130>
1407d7b42:     	movq	%rax, %rcx
1407d7b45:     	movq	(%rax), %rax
1407d7b48:     	cmpq	$0x1, %rax
1407d7b4c:     	jne	0x1407d7bfc <.text+0x7d6bfc>
1407d7b52:     	addq	$0x8, %rcx
1407d7b56:     	movq	0xc8(%rbp), %rdx
1407d7b5d:     	cmpq	$0x0, (%rcx)
1407d7b61:     	jne	0x1407d7c62 <.text+0x7d6c62>
1407d7b67:     	movq	$-0x1, (%rcx)
1407d7b6e:     	movq	%rcx, 0x1f8(%rbp)
1407d7b75:     	addq	$0x8, %rcx
1407d7b79:     	callq	0x140c170f0 <.text+0xc160f0>
1407d7b7e:     	movq	0x1f8(%rbp), %rax
1407d7b85:     	incq	(%rax)
1407d7b88:     	movaps	0x210(%rbp), %xmm6
1407d7b8f:     	movaps	0x220(%rbp), %xmm7
1407d7b96:     	movaps	0x230(%rbp), %xmm8
1407d7b9e:     	movaps	0x240(%rbp), %xmm9
1407d7ba6:     	movaps	0x250(%rbp), %xmm10
1407d7bae:     	movaps	0x260(%rbp), %xmm11
1407d7bb6:     	movaps	0x270(%rbp), %xmm12
1407d7bbe:     	movaps	0x280(%rbp), %xmm13
1407d7bc6:     	movaps	0x290(%rbp), %xmm14
1407d7bce:     	addq	$0x328, %rsp            # imm = 0x328
1407d7bd5:     	popq	%rbx
1407d7bd6:     	popq	%rdi
1407d7bd7:     	popq	%rsi
1407d7bd8:     	popq	%r12
1407d7bda:     	popq	%r13
1407d7bdc:     	popq	%r14
1407d7bde:     	popq	%r15
1407d7be0:     	popq	%rbp
1407d7be1:     	retq
1407d7be2:     	testq	%rax, %rax
1407d7be5:     	jne	0x1407d7c1b <.text+0x7d6c1b>
1407d7be7:     	xorl	%edx, %edx
1407d7be9:     	callq	0x1428f2580 <.text+0x28f1580>
1407d7bee:     	movq	%rax, %rcx
1407d7bf1:     	testq	%rax, %rax
1407d7bf4:     	jne	0x1407d4c3a <.text+0x7d3c3a>
1407d7bfa:     	jmp	0x1407d7c1b <.text+0x7d6c1b>
1407d7bfc:     	testq	%rax, %rax
1407d7bff:     	jne	0x1407d7c1b <.text+0x7d6c1b>
1407d7c01:     	xorl	%edx, %edx
1407d7c03:     	callq	0x1428f2580 <.text+0x28f1580>
1407d7c08:     	movq	0xc8(%rbp), %rdx
1407d7c0f:     	movq	%rax, %rcx
1407d7c12:     	testq	%rax, %rax
1407d7c15:     	jne	0x1407d7b5d <.text+0x7d6b5d>
1407d7c1b:     	leaq	0x24497a6(%rip), %rcx   # 0x142c213c8
1407d7c22:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7c27:     	leaq	0x22b0532(%rip), %rcx   # 0x142a88160
1407d7c2e:     	leaq	0x22b0553(%rip), %r8    # 0x142a88188
1407d7c35:     	movl	$0x28, %edx
1407d7c3a:     	callq	0x1428d9430 <.text+0x28d8430>
1407d7c3f:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7c44:     	movq	%rdi, %rcx
1407d7c47:     	callq	0x1428ccf50 <.text+0x28cbf50>
1407d7c4c:     	jmp	0x1407d5285 <.text+0x7d4285>
1407d7c51:     	leaq	0x24497e0(%rip), %rcx   # 0x142c21438
1407d7c58:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7c5d:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7c62:     	leaq	0x24497cf(%rip), %rcx   # 0x142c21438
1407d7c69:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7c6e:     	leaq	0x24497c3(%rip), %rcx   # 0x142c21438
1407d7c75:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7c7a:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7c7f:     	leaq	0x24497b2(%rip), %rcx   # 0x142c21438
1407d7c86:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7c8b:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7c90:     	leaq	0x24497a1(%rip), %rcx   # 0x142c21438
1407d7c97:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7c9c:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7ca1:     	leaq	0x2449790(%rip), %rcx   # 0x142c21438
1407d7ca8:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7cad:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7cb2:     	leaq	0x244a1c7(%rip), %r8    # 0x142c21e80
1407d7cb9:     	movq	%rdx, %rcx
1407d7cbc:     	movq	%rax, %rdx
1407d7cbf:     	callq	0x1428d9518 <.text+0x28d8518>
1407d7cc4:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7cc9:     	leaq	0x2449768(%rip), %rcx   # 0x142c21438
1407d7cd0:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7cd5:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7cda:     	leaq	0x2449757(%rip), %rcx   # 0x142c21438
1407d7ce1:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7ce6:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7ceb:     	leaq	0x2449746(%rip), %rcx   # 0x142c21438
1407d7cf2:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7cf7:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7cfc:     	leaq	0x244a17d(%rip), %r8    # 0x142c21e80
1407d7d03:     	movq	%rdx, %rcx
1407d7d06:     	movq	%rax, %rdx
1407d7d09:     	callq	0x1428d9518 <.text+0x28d8518>
1407d7d0e:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d13:     	leaq	0x244a166(%rip), %r8    # 0x142c21e80
1407d7d1a:     	movq	%rdx, %rcx
1407d7d1d:     	movq	%rax, %rdx
1407d7d20:     	callq	0x1428d9518 <.text+0x28d8518>
1407d7d25:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d2a:     	leaq	0x22b0417(%rip), %rcx   # 0x142a88148
1407d7d31:     	callq	0x1428d9310 <.text+0x28d8310>
1407d7d36:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d3b:     	leaq	0x2449686(%rip), %rcx   # 0x142c213c8
1407d7d42:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7d47:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d4c:     	leaq	0x2449675(%rip), %rcx   # 0x142c213c8
1407d7d53:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7d58:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d5d:     	leaq	0x2449664(%rip), %rcx   # 0x142c213c8
1407d7d64:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7d69:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d6b:     	leaq	0x2449656(%rip), %rcx   # 0x142c213c8
1407d7d72:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7d77:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d79:     	leaq	0x2449648(%rip), %rcx   # 0x142c213c8
1407d7d80:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7d85:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d87:     	leaq	0x244963a(%rip), %rcx   # 0x142c213c8
1407d7d8e:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7d93:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7d95:     	leaq	0x244962c(%rip), %rcx   # 0x142c213c8
1407d7d9c:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7da1:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7da3:     	leaq	0x244961e(%rip), %rcx   # 0x142c213c8
1407d7daa:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7daf:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7db1:     	leaq	0x2449610(%rip), %rcx   # 0x142c213c8
1407d7db8:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7dbd:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7dbf:     	leaq	0x2449602(%rip), %rcx   # 0x142c213c8
1407d7dc6:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7dcb:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7dcd:     	leaq	0x24495f4(%rip), %rcx   # 0x142c213c8
1407d7dd4:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7dd9:     	jmp	0x1407d7de7 <.text+0x7d6de7>
1407d7ddb:     	leaq	0x24495e6(%rip), %rcx   # 0x142c213c8
1407d7de2:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7de7:     	ud2
1407d7de9:     	nopl	(%rax)
1407d7df0:     	movq	%rdx, 0x10(%rsp)
1407d7df5:     	pushq	%rbp
1407d7df6:     	pushq	%r15
1407d7df8:     	pushq	%r14
1407d7dfa:     	pushq	%r13
1407d7dfc:     	pushq	%r12
1407d7dfe:     	pushq	%rsi
1407d7dff:     	pushq	%rdi
1407d7e00:     	pushq	%rbx
1407d7e01:     	subq	$0xc8, %rsp
1407d7e08:     	leaq	0x80(%rdx), %rbp
1407d7e0f:     	movaps	%xmm14, 0x30(%rsp)
1407d7e15:     	movaps	%xmm13, 0x40(%rsp)
1407d7e1b:     	movaps	%xmm12, 0x50(%rsp)
1407d7e21:     	movaps	%xmm11, 0x60(%rsp)
1407d7e27:     	movdqa	%xmm10, 0x70(%rsp)
1407d7e2e:     	movaps	%xmm9, 0x80(%rsp)
1407d7e37:     	movaps	%xmm8, 0x90(%rsp)
1407d7e40:     	movdqa	%xmm7, 0xa0(%rsp)
1407d7e49:     	movaps	%xmm6, 0xb0(%rsp)
1407d7e51:     	movq	0x1f8(%rbp), %rax
1407d7e58:     	incq	(%rax)
1407d7e5b:     	movaps	0xb0(%rsp), %xmm6
1407d7e63:     	movaps	0xa0(%rsp), %xmm7
1407d7e6b:     	movaps	0x90(%rsp), %xmm8
1407d7e74:     	movaps	0x80(%rsp), %xmm9
1407d7e7d:     	movaps	0x70(%rsp), %xmm10
1407d7e83:     	movaps	0x60(%rsp), %xmm11
1407d7e89:     	movaps	0x50(%rsp), %xmm12
1407d7e8f:     	movaps	0x40(%rsp), %xmm13
1407d7e95:     	movaps	0x30(%rsp), %xmm14
1407d7e9b:     	addq	$0xc8, %rsp
1407d7ea2:     	popq	%rbx
1407d7ea3:     	popq	%rdi
1407d7ea4:     	popq	%rsi
1407d7ea5:     	popq	%r12
1407d7ea7:     	popq	%r13
1407d7ea9:     	popq	%r14
1407d7eab:     	popq	%r15
1407d7ead:     	popq	%rbp
1407d7eae:     	retq
1407d7eaf:     	nop
1407d7eb0:     	movq	%rdx, 0x10(%rsp)
1407d7eb5:     	pushq	%rbp
1407d7eb6:     	pushq	%r15
1407d7eb8:     	pushq	%r14
1407d7eba:     	pushq	%r13
1407d7ebc:     	pushq	%r12
1407d7ebe:     	pushq	%rsi
1407d7ebf:     	pushq	%rdi
1407d7ec0:     	pushq	%rbx
1407d7ec1:     	subq	$0xc8, %rsp
1407d7ec8:     	leaq	0x80(%rdx), %rbp
1407d7ecf:     	movaps	%xmm14, 0x30(%rsp)
1407d7ed5:     	movaps	%xmm13, 0x40(%rsp)
1407d7edb:     	movaps	%xmm12, 0x50(%rsp)
1407d7ee1:     	movaps	%xmm11, 0x60(%rsp)
1407d7ee7:     	movdqa	%xmm10, 0x70(%rsp)
1407d7eee:     	movaps	%xmm9, 0x80(%rsp)
1407d7ef7:     	movaps	%xmm8, 0x90(%rsp)
1407d7f00:     	movdqa	%xmm7, 0xa0(%rsp)
1407d7f09:     	movaps	%xmm6, 0xb0(%rsp)
1407d7f11:     	cmpb	$0x0, 0x1ef(%rbp)
1407d7f18:     	je	0x1407d7f53 <.text+0x7d6f53>
1407d7f1a:     	callq	0x140c18130 <.text+0xc17130>
1407d7f1f:     	movq	%rax, %rsi
1407d7f22:     	movq	(%rax), %rax
1407d7f25:     	cmpq	$0x1, %rax
1407d7f29:     	jne	0x1407d7fa7 <.text+0x7d6fa7>
1407d7f2b:     	addq	$0x8, %rsi
1407d7f2f:     	movq	0xc8(%rbp), %rdx
1407d7f36:     	cmpq	$0x0, (%rsi)
1407d7f3a:     	jne	0x1407d7fd5 <.text+0x7d6fd5>
1407d7f40:     	movq	$-0x1, (%rsi)
1407d7f47:     	leaq	0x8(%rsi), %rcx
1407d7f4b:     	callq	0x140c170f0 <.text+0xc160f0>
1407d7f50:     	incq	(%rsi)
1407d7f53:     	movaps	0xb0(%rsp), %xmm6
1407d7f5b:     	movaps	0xa0(%rsp), %xmm7
1407d7f63:     	movaps	0x90(%rsp), %xmm8
1407d7f6c:     	movaps	0x80(%rsp), %xmm9
1407d7f75:     	movaps	0x70(%rsp), %xmm10
1407d7f7b:     	movaps	0x60(%rsp), %xmm11
1407d7f81:     	movaps	0x50(%rsp), %xmm12
1407d7f87:     	movaps	0x40(%rsp), %xmm13
1407d7f8d:     	movaps	0x30(%rsp), %xmm14
1407d7f93:     	addq	$0xc8, %rsp
1407d7f9a:     	popq	%rbx
1407d7f9b:     	popq	%rdi
1407d7f9c:     	popq	%rsi
1407d7f9d:     	popq	%r12
1407d7f9f:     	popq	%r13
1407d7fa1:     	popq	%r14
1407d7fa3:     	popq	%r15
1407d7fa5:     	popq	%rbp
1407d7fa6:     	retq
1407d7fa7:     	testq	%rax, %rax
1407d7faa:     	jne	0x1407d7fc9 <.text+0x7d6fc9>
1407d7fac:     	movq	%rsi, %rcx
1407d7faf:     	xorl	%edx, %edx
1407d7fb1:     	callq	0x1428f2580 <.text+0x28f1580>
1407d7fb6:     	movq	0xc8(%rbp), %rdx
1407d7fbd:     	movq	%rax, %rsi
1407d7fc0:     	testq	%rax, %rax
1407d7fc3:     	jne	0x1407d7f36 <.text+0x7d6f36>
1407d7fc9:     	leaq	0x24493f8(%rip), %rcx   # 0x142c213c8
1407d7fd0:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d7fd5:     	leaq	0x244945c(%rip), %rcx   # 0x142c21438
1407d7fdc:     	callq	0x1428d9250 <.text+0x28d8250>
1407d7fe1:     	int3
1407d7fe2:     	nopw	%cs:(%rax,%rax)
1407d7ff0:     	movq	%rdx, 0x10(%rsp)
1407d7ff5:     	pushq	%rbp
1407d7ff6:     	pushq	%r15
1407d7ff8:     	pushq	%r14
1407d7ffa:     	pushq	%r13
1407d7ffc:     	pushq	%r12
1407d7ffe:     	pushq	%rsi
1407d7fff:     	pushq	%rdi
1407d8000:     	pushq	%rbx
1407d8001:     	subq	$0xc8, %rsp
1407d8008:     	leaq	0x80(%rdx), %rbp
1407d800f:     	movaps	%xmm14, 0x30(%rsp)
1407d8015:     	movaps	%xmm13, 0x40(%rsp)
1407d801b:     	movaps	%xmm12, 0x50(%rsp)
1407d8021:     	movaps	%xmm11, 0x60(%rsp)
1407d8027:     	movdqa	%xmm10, 0x70(%rsp)
1407d802e:     	movaps	%xmm9, 0x80(%rsp)
1407d8037:     	movaps	%xmm8, 0x90(%rsp)
1407d8040:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8049:     	movaps	%xmm6, 0xb0(%rsp)
1407d8051:     	leaq	0x1b0(%rbp), %rcx
1407d8058:     	callq	0x140877cc0 <.text+0x876cc0>
1407d805d:     	movaps	0xb0(%rsp), %xmm6
1407d8065:     	movdqa	0xa0(%rsp), %xmm7
1407d806e:     	movaps	0x90(%rsp), %xmm8
1407d8077:     	movaps	0x80(%rsp), %xmm9
1407d8080:     	movdqa	0x70(%rsp), %xmm10
1407d8087:     	movaps	0x60(%rsp), %xmm11
1407d808d:     	movaps	0x50(%rsp), %xmm12
1407d8093:     	movaps	0x40(%rsp), %xmm13
1407d8099:     	movaps	0x30(%rsp), %xmm14
1407d809f:     	addq	$0xc8, %rsp
1407d80a6:     	popq	%rbx
1407d80a7:     	popq	%rdi
1407d80a8:     	popq	%rsi
1407d80a9:     	popq	%r12
1407d80ab:     	popq	%r13
1407d80ad:     	popq	%r14
1407d80af:     	popq	%r15
1407d80b1:     	popq	%rbp
1407d80b2:     	retq
1407d80b3:     	nopw	%cs:(%rax,%rax)
1407d80c0:     	movq	%rdx, 0x10(%rsp)
1407d80c5:     	pushq	%rbp
1407d80c6:     	pushq	%r15
1407d80c8:     	pushq	%r14
1407d80ca:     	pushq	%r13
1407d80cc:     	pushq	%r12
1407d80ce:     	pushq	%rsi
1407d80cf:     	pushq	%rdi
1407d80d0:     	pushq	%rbx
1407d80d1:     	subq	$0xc8, %rsp
1407d80d8:     	leaq	0x80(%rdx), %rbp
1407d80df:     	movaps	%xmm14, 0x30(%rsp)
1407d80e5:     	movaps	%xmm13, 0x40(%rsp)
1407d80eb:     	movaps	%xmm12, 0x50(%rsp)
1407d80f1:     	movaps	%xmm11, 0x60(%rsp)
1407d80f7:     	movdqa	%xmm10, 0x70(%rsp)
1407d80fe:     	movaps	%xmm9, 0x80(%rsp)
1407d8107:     	movaps	%xmm8, 0x90(%rsp)
1407d8110:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8119:     	movaps	%xmm6, 0xb0(%rsp)
1407d8121:     	movq	0x1f0(%rbp), %rax
1407d8128:     	incq	(%rax)
1407d812b:     	movaps	0xb0(%rsp), %xmm6
1407d8133:     	movdqa	0xa0(%rsp), %xmm7
1407d813c:     	movaps	0x90(%rsp), %xmm8
1407d8145:     	movaps	0x80(%rsp), %xmm9
1407d814e:     	movdqa	0x70(%rsp), %xmm10
1407d8155:     	movaps	0x60(%rsp), %xmm11
1407d815b:     	movaps	0x50(%rsp), %xmm12
1407d8161:     	movaps	0x40(%rsp), %xmm13
1407d8167:     	movaps	0x30(%rsp), %xmm14
1407d816d:     	addq	$0xc8, %rsp
1407d8174:     	popq	%rbx
1407d8175:     	popq	%rdi
1407d8176:     	popq	%rsi
1407d8177:     	popq	%r12
1407d8179:     	popq	%r13
1407d817b:     	popq	%r14
1407d817d:     	popq	%r15
1407d817f:     	popq	%rbp
1407d8180:     	retq
1407d8181:     	nopw	%cs:(%rax,%rax)
1407d8190:     	movq	%rdx, 0x10(%rsp)
1407d8195:     	pushq	%rbp
1407d8196:     	pushq	%r15
1407d8198:     	pushq	%r14
1407d819a:     	pushq	%r13
1407d819c:     	pushq	%r12
1407d819e:     	pushq	%rsi
1407d819f:     	pushq	%rdi
1407d81a0:     	pushq	%rbx
1407d81a1:     	subq	$0xc8, %rsp
1407d81a8:     	leaq	0x80(%rdx), %rbp
1407d81af:     	movaps	%xmm14, 0x30(%rsp)
1407d81b5:     	movaps	%xmm13, 0x40(%rsp)
1407d81bb:     	movaps	%xmm12, 0x50(%rsp)
1407d81c1:     	movaps	%xmm11, 0x60(%rsp)
1407d81c7:     	movdqa	%xmm10, 0x70(%rsp)
1407d81ce:     	movaps	%xmm9, 0x80(%rsp)
1407d81d7:     	movaps	%xmm8, 0x90(%rsp)
1407d81e0:     	movdqa	%xmm7, 0xa0(%rsp)
1407d81e9:     	movaps	%xmm6, 0xb0(%rsp)
1407d81f1:     	cmpb	$0x0, 0x1f8(%rbp)
1407d81f8:     	je	0x1407d8229 <.text+0x7d7229>
1407d81fa:     	leaq	0x198(%rbp), %rax
1407d8201:     	movq	%rax, 0x168(%rbp)
1407d8208:     	leaq	0x168(%rbp), %rax
1407d820f:     	movq	%rax, 0x108(%rbp)
1407d8216:     	leaq	0x22af9d3(%rip), %rcx   # 0x142a87bf0
1407d821d:     	leaq	0x108(%rbp), %rdx
1407d8224:     	callq	0x141257bc0 <.text+0x1256bc0>
1407d8229:     	movaps	0xb0(%rsp), %xmm6
1407d8231:     	movdqa	0xa0(%rsp), %xmm7
1407d823a:     	movaps	0x90(%rsp), %xmm8
1407d8243:     	movaps	0x80(%rsp), %xmm9
1407d824c:     	movdqa	0x70(%rsp), %xmm10
1407d8253:     	movaps	0x60(%rsp), %xmm11
1407d8259:     	movaps	0x50(%rsp), %xmm12
1407d825f:     	movaps	0x40(%rsp), %xmm13
1407d8265:     	movaps	0x30(%rsp), %xmm14
1407d826b:     	addq	$0xc8, %rsp
1407d8272:     	popq	%rbx
1407d8273:     	popq	%rdi
1407d8274:     	popq	%rsi
1407d8275:     	popq	%r12
1407d8277:     	popq	%r13
1407d8279:     	popq	%r14
1407d827b:     	popq	%r15
1407d827d:     	popq	%rbp
1407d827e:     	retq
1407d827f:     	nop
1407d8280:     	movq	%rdx, 0x10(%rsp)
1407d8285:     	pushq	%rbp
1407d8286:     	pushq	%r15
1407d8288:     	pushq	%r14
1407d828a:     	pushq	%r13
1407d828c:     	pushq	%r12
1407d828e:     	pushq	%rsi
1407d828f:     	pushq	%rdi
1407d8290:     	pushq	%rbx
1407d8291:     	subq	$0xc8, %rsp
1407d8298:     	leaq	0x80(%rdx), %rbp
1407d829f:     	movaps	%xmm14, 0x30(%rsp)
1407d82a5:     	movaps	%xmm13, 0x40(%rsp)
1407d82ab:     	movaps	%xmm12, 0x50(%rsp)
1407d82b1:     	movaps	%xmm11, 0x60(%rsp)
1407d82b7:     	movdqa	%xmm10, 0x70(%rsp)
1407d82be:     	movaps	%xmm9, 0x80(%rsp)
1407d82c7:     	movaps	%xmm8, 0x90(%rsp)
1407d82d0:     	movdqa	%xmm7, 0xa0(%rsp)
1407d82d9:     	movaps	%xmm6, 0xb0(%rsp)
1407d82e1:     	movq	0x1f8(%rbp), %rax
1407d82e8:     	incq	(%rax)
1407d82eb:     	movaps	0xb0(%rsp), %xmm6
1407d82f3:     	movdqa	0xa0(%rsp), %xmm7
1407d82fc:     	movaps	0x90(%rsp), %xmm8
1407d8305:     	movaps	0x80(%rsp), %xmm9
1407d830e:     	movdqa	0x70(%rsp), %xmm10
1407d8315:     	movaps	0x60(%rsp), %xmm11
1407d831b:     	movaps	0x50(%rsp), %xmm12
1407d8321:     	movaps	0x40(%rsp), %xmm13
1407d8327:     	movaps	0x30(%rsp), %xmm14
1407d832d:     	addq	$0xc8, %rsp
1407d8334:     	popq	%rbx
1407d8335:     	popq	%rdi
1407d8336:     	popq	%rsi
1407d8337:     	popq	%r12
1407d8339:     	popq	%r13
1407d833b:     	popq	%r14
1407d833d:     	popq	%r15
1407d833f:     	popq	%rbp
1407d8340:     	retq
1407d8341:     	nopw	%cs:(%rax,%rax)
1407d8350:     	movq	%rdx, 0x10(%rsp)
1407d8355:     	pushq	%rbp
1407d8356:     	pushq	%r15
1407d8358:     	pushq	%r14
1407d835a:     	pushq	%r13
1407d835c:     	pushq	%r12
1407d835e:     	pushq	%rsi
1407d835f:     	pushq	%rdi
1407d8360:     	pushq	%rbx
1407d8361:     	subq	$0xc8, %rsp
1407d8368:     	leaq	0x80(%rdx), %rbp
1407d836f:     	movaps	%xmm14, 0x30(%rsp)
1407d8375:     	movaps	%xmm13, 0x40(%rsp)
1407d837b:     	movaps	%xmm12, 0x50(%rsp)
1407d8381:     	movaps	%xmm11, 0x60(%rsp)
1407d8387:     	movdqa	%xmm10, 0x70(%rsp)
1407d838e:     	movaps	%xmm9, 0x80(%rsp)
1407d8397:     	movaps	%xmm8, 0x90(%rsp)
1407d83a0:     	movdqa	%xmm7, 0xa0(%rsp)
1407d83a9:     	movaps	%xmm6, 0xb0(%rsp)
1407d83b1:     	movq	0x1f8(%rbp), %rax
1407d83b8:     	incq	(%rax)
1407d83bb:     	movaps	0xb0(%rsp), %xmm6
1407d83c3:     	movdqa	0xa0(%rsp), %xmm7
1407d83cc:     	movaps	0x90(%rsp), %xmm8
1407d83d5:     	movaps	0x80(%rsp), %xmm9
1407d83de:     	movdqa	0x70(%rsp), %xmm10
1407d83e5:     	movaps	0x60(%rsp), %xmm11
1407d83eb:     	movaps	0x50(%rsp), %xmm12
1407d83f1:     	movaps	0x40(%rsp), %xmm13
1407d83f7:     	movaps	0x30(%rsp), %xmm14
1407d83fd:     	addq	$0xc8, %rsp
1407d8404:     	popq	%rbx
1407d8405:     	popq	%rdi
1407d8406:     	popq	%rsi
1407d8407:     	popq	%r12
1407d8409:     	popq	%r13
1407d840b:     	popq	%r14
1407d840d:     	popq	%r15
1407d840f:     	popq	%rbp
1407d8410:     	retq
1407d8411:     	nopw	%cs:(%rax,%rax)
1407d8420:     	movq	%rdx, 0x10(%rsp)
1407d8425:     	pushq	%rbp
1407d8426:     	pushq	%r15
1407d8428:     	pushq	%r14
1407d842a:     	pushq	%r13
1407d842c:     	pushq	%r12
1407d842e:     	pushq	%rsi
1407d842f:     	pushq	%rdi
1407d8430:     	pushq	%rbx
1407d8431:     	subq	$0xc8, %rsp
1407d8438:     	leaq	0x80(%rdx), %rbp
1407d843f:     	movaps	%xmm14, 0x30(%rsp)
1407d8445:     	movaps	%xmm13, 0x40(%rsp)
1407d844b:     	movaps	%xmm12, 0x50(%rsp)
1407d8451:     	movaps	%xmm11, 0x60(%rsp)
1407d8457:     	movdqa	%xmm10, 0x70(%rsp)
1407d845e:     	movaps	%xmm9, 0x80(%rsp)
1407d8467:     	movaps	%xmm8, 0x90(%rsp)
1407d8470:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8479:     	movaps	%xmm6, 0xb0(%rsp)
1407d8481:     	cmpb	$0x0, 0x1ee(%rbp)
1407d8488:     	je	0x1407d84b6 <.text+0x7d74b6>
1407d848a:     	leaq	(%rbp), %rax
1407d848e:     	movq	%rax, 0x168(%rbp)
1407d8495:     	leaq	0x168(%rbp), %rax
1407d849c:     	movq	%rax, 0x108(%rbp)
1407d84a3:     	leaq	0x22af746(%rip), %rcx   # 0x142a87bf0
1407d84aa:     	leaq	0x108(%rbp), %rdx
1407d84b1:     	callq	0x141257bc0 <.text+0x1256bc0>
1407d84b6:     	movaps	0xb0(%rsp), %xmm6
1407d84be:     	movdqa	0xa0(%rsp), %xmm7
1407d84c7:     	movaps	0x90(%rsp), %xmm8
1407d84d0:     	movaps	0x80(%rsp), %xmm9
1407d84d9:     	movdqa	0x70(%rsp), %xmm10
1407d84e0:     	movaps	0x60(%rsp), %xmm11
1407d84e6:     	movaps	0x50(%rsp), %xmm12
1407d84ec:     	movaps	0x40(%rsp), %xmm13
1407d84f2:     	movaps	0x30(%rsp), %xmm14
1407d84f8:     	addq	$0xc8, %rsp
1407d84ff:     	popq	%rbx
1407d8500:     	popq	%rdi
1407d8501:     	popq	%rsi
1407d8502:     	popq	%r12
1407d8504:     	popq	%r13
1407d8506:     	popq	%r14
1407d8508:     	popq	%r15
1407d850a:     	popq	%rbp
1407d850b:     	retq
1407d850c:     	nopl	(%rax)
1407d8510:     	movq	%rdx, 0x10(%rsp)
1407d8515:     	pushq	%rbp
1407d8516:     	pushq	%r15
1407d8518:     	pushq	%r14
1407d851a:     	pushq	%r13
1407d851c:     	pushq	%r12
1407d851e:     	pushq	%rsi
1407d851f:     	pushq	%rdi
1407d8520:     	pushq	%rbx
1407d8521:     	subq	$0xc8, %rsp
1407d8528:     	leaq	0x80(%rdx), %rbp
1407d852f:     	movaps	%xmm14, 0x30(%rsp)
1407d8535:     	movaps	%xmm13, 0x40(%rsp)
1407d853b:     	movaps	%xmm12, 0x50(%rsp)
1407d8541:     	movaps	%xmm11, 0x60(%rsp)
1407d8547:     	movdqa	%xmm10, 0x70(%rsp)
1407d854e:     	movdqa	%xmm9, 0x80(%rsp)
1407d8558:     	movaps	%xmm8, 0x90(%rsp)
1407d8561:     	movdqa	%xmm7, 0xa0(%rsp)
1407d856a:     	movaps	%xmm6, 0xb0(%rsp)
1407d8572:     	leaq	0xd0(%rbp), %rcx
1407d8579:     	callq	0x14014b650 <.text+0x14a650>
1407d857e:     	movaps	0xb0(%rsp), %xmm6
1407d8586:     	movdqa	0xa0(%rsp), %xmm7
1407d858f:     	movaps	0x90(%rsp), %xmm8
1407d8598:     	movaps	0x80(%rsp), %xmm9
1407d85a1:     	movdqa	0x70(%rsp), %xmm10
1407d85a8:     	movaps	0x60(%rsp), %xmm11
1407d85ae:     	movaps	0x50(%rsp), %xmm12
1407d85b4:     	movaps	0x40(%rsp), %xmm13
1407d85ba:     	movaps	0x30(%rsp), %xmm14
1407d85c0:     	addq	$0xc8, %rsp
1407d85c7:     	popq	%rbx
1407d85c8:     	popq	%rdi
1407d85c9:     	popq	%rsi
1407d85ca:     	popq	%r12
1407d85cc:     	popq	%r13
1407d85ce:     	popq	%r14
1407d85d0:     	popq	%r15
1407d85d2:     	popq	%rbp
1407d85d3:     	retq
1407d85d4:     	nopw	%cs:(%rax,%rax)
1407d85e0:     	movq	%rdx, 0x10(%rsp)
1407d85e5:     	pushq	%rbp
1407d85e6:     	pushq	%r15
1407d85e8:     	pushq	%r14
1407d85ea:     	pushq	%r13
1407d85ec:     	pushq	%r12
1407d85ee:     	pushq	%rsi
1407d85ef:     	pushq	%rdi
1407d85f0:     	pushq	%rbx
1407d85f1:     	subq	$0xc8, %rsp
1407d85f8:     	leaq	0x80(%rdx), %rbp
1407d85ff:     	movaps	%xmm14, 0x30(%rsp)
1407d8605:     	movaps	%xmm13, 0x40(%rsp)
1407d860b:     	movaps	%xmm12, 0x50(%rsp)
1407d8611:     	movaps	%xmm11, 0x60(%rsp)
1407d8617:     	movdqa	%xmm10, 0x70(%rsp)
1407d861e:     	movdqa	%xmm9, 0x80(%rsp)
1407d8628:     	movaps	%xmm8, 0x90(%rsp)
1407d8631:     	movdqa	%xmm7, 0xa0(%rsp)
1407d863a:     	movaps	%xmm6, 0xb0(%rsp)
1407d8642:     	leaq	0x10(%rbp), %rcx
1407d8646:     	callq	0x14014b650 <.text+0x14a650>
1407d864b:     	movaps	0xb0(%rsp), %xmm6
1407d8653:     	movaps	0xa0(%rsp), %xmm7
1407d865b:     	movaps	0x90(%rsp), %xmm8
1407d8664:     	movaps	0x80(%rsp), %xmm9
1407d866d:     	movaps	0x70(%rsp), %xmm10
1407d8673:     	movaps	0x60(%rsp), %xmm11
1407d8679:     	movaps	0x50(%rsp), %xmm12
1407d867f:     	movaps	0x40(%rsp), %xmm13
1407d8685:     	movaps	0x30(%rsp), %xmm14
1407d868b:     	addq	$0xc8, %rsp
1407d8692:     	popq	%rbx
1407d8693:     	popq	%rdi
1407d8694:     	popq	%rsi
1407d8695:     	popq	%r12
1407d8697:     	popq	%r13
1407d8699:     	popq	%r14
1407d869b:     	popq	%r15
1407d869d:     	popq	%rbp
1407d869e:     	retq
1407d869f:     	nop
1407d86a0:     	movq	%rdx, 0x10(%rsp)
1407d86a5:     	pushq	%rbp
1407d86a6:     	pushq	%r15
1407d86a8:     	pushq	%r14
1407d86aa:     	pushq	%r13
1407d86ac:     	pushq	%r12
1407d86ae:     	pushq	%rsi
1407d86af:     	pushq	%rdi
1407d86b0:     	pushq	%rbx
1407d86b1:     	subq	$0xc8, %rsp
1407d86b8:     	leaq	0x80(%rdx), %rbp
1407d86bf:     	movaps	%xmm14, 0x30(%rsp)
1407d86c5:     	movaps	%xmm13, 0x40(%rsp)
1407d86cb:     	movaps	%xmm12, 0x50(%rsp)
1407d86d1:     	movaps	%xmm11, 0x60(%rsp)
1407d86d7:     	movdqa	%xmm10, 0x70(%rsp)
1407d86de:     	movdqa	%xmm9, 0x80(%rsp)
1407d86e8:     	movaps	%xmm8, 0x90(%rsp)
1407d86f1:     	movdqa	%xmm7, 0xa0(%rsp)
1407d86fa:     	movaps	%xmm6, 0xb0(%rsp)
1407d8702:     	movq	0x1d8(%rbp), %rax
1407d8709:     	incq	(%rax)
1407d870c:     	movaps	0xb0(%rsp), %xmm6
1407d8714:     	movdqa	0xa0(%rsp), %xmm7
1407d871d:     	movaps	0x90(%rsp), %xmm8
1407d8726:     	movaps	0x80(%rsp), %xmm9
1407d872f:     	movdqa	0x70(%rsp), %xmm10
1407d8736:     	movaps	0x60(%rsp), %xmm11
1407d873c:     	movaps	0x50(%rsp), %xmm12
1407d8742:     	movaps	0x40(%rsp), %xmm13
1407d8748:     	movaps	0x30(%rsp), %xmm14
1407d874e:     	addq	$0xc8, %rsp
1407d8755:     	popq	%rbx
1407d8756:     	popq	%rdi
1407d8757:     	popq	%rsi
1407d8758:     	popq	%r12
1407d875a:     	popq	%r13
1407d875c:     	popq	%r14
1407d875e:     	popq	%r15
1407d8760:     	popq	%rbp
1407d8761:     	retq
1407d8762:     	nopw	%cs:(%rax,%rax)
1407d8770:     	movq	%rdx, 0x10(%rsp)
1407d8775:     	pushq	%rbp
1407d8776:     	pushq	%r15
1407d8778:     	pushq	%r14
1407d877a:     	pushq	%r13
1407d877c:     	pushq	%r12
1407d877e:     	pushq	%rsi
1407d877f:     	pushq	%rdi
1407d8780:     	pushq	%rbx
1407d8781:     	subq	$0xc8, %rsp
1407d8788:     	leaq	0x80(%rdx), %rbp
1407d878f:     	movaps	%xmm14, 0x30(%rsp)
1407d8795:     	movaps	%xmm13, 0x40(%rsp)
1407d879b:     	movaps	%xmm12, 0x50(%rsp)
1407d87a1:     	movdqa	%xmm11, 0x60(%rsp)
1407d87a8:     	movaps	%xmm10, 0x70(%rsp)
1407d87ae:     	movdqa	%xmm9, 0x80(%rsp)
1407d87b8:     	movaps	%xmm8, 0x90(%rsp)
1407d87c1:     	movdqa	%xmm7, 0xa0(%rsp)
1407d87ca:     	movaps	%xmm6, 0xb0(%rsp)
1407d87d2:     	movq	0x30(%rbp), %rdx
1407d87d6:     	testq	%rdx, %rdx
1407d87d9:     	je	0x1407d8802 <.text+0x7d7802>
1407d87db:     	leaq	0x17(,%rdx,8), %rax
1407d87e3:     	andq	$-0x10, %rax
1407d87e7:     	addq	%rax, %rdx
1407d87ea:     	addq	$0x11, %rdx
1407d87ee:     	je	0x1407d8802 <.text+0x7d7802>
1407d87f0:     	movq	0x28(%rbp), %rcx
1407d87f4:     	subq	%rax, %rcx
1407d87f7:     	movl	$0x10, %r8d
1407d87fd:     	callq	0x140613c20 <.text+0x612c20>
1407d8802:     	movq	0x10(%rbp), %rax
1407d8806:     	testq	%rax, %rax
1407d8809:     	jne	0x1407d888a <.text+0x7d788a>
1407d880b:     	movq	0x58(%rbp), %rdx
1407d880f:     	testq	%rdx, %rdx
1407d8812:     	jne	0x1407d88ae <.text+0x7d78ae>
1407d8818:     	movq	0x70(%rbp), %rdx
1407d881c:     	testq	%rdx, %rdx
1407d881f:     	je	0x1407d8834 <.text+0x7d7834>
1407d8821:     	movq	0x78(%rbp), %rcx
1407d8825:     	shlq	$0x3, %rdx
1407d8829:     	movl	$0x8, %r8d
1407d882f:     	callq	0x140613c20 <.text+0x612c20>
1407d8834:     	movaps	0xb0(%rsp), %xmm6
1407d883c:     	movdqa	0xa0(%rsp), %xmm7
1407d8845:     	movaps	0x90(%rsp), %xmm8
1407d884e:     	movaps	0x80(%rsp), %xmm9
1407d8857:     	movdqa	0x70(%rsp), %xmm10
1407d885e:     	movaps	0x60(%rsp), %xmm11
1407d8864:     	movaps	0x50(%rsp), %xmm12
1407d886a:     	movaps	0x40(%rsp), %xmm13
1407d8870:     	movaps	0x30(%rsp), %xmm14
1407d8876:     	addq	$0xc8, %rsp
1407d887d:     	popq	%rbx
1407d887e:     	popq	%rdi
1407d887f:     	popq	%rsi
1407d8880:     	popq	%r12
1407d8882:     	popq	%r13
1407d8884:     	popq	%r14
1407d8886:     	popq	%r15
1407d8888:     	popq	%rbp
1407d8889:     	retq
1407d888a:     	movq	0x18(%rbp), %rcx
1407d888e:     	shlq	$0x3, %rax
1407d8892:     	leaq	(%rax,%rax,2), %rdx
1407d8896:     	movl	$0x8, %r8d
1407d889c:     	callq	0x140613c20 <.text+0x612c20>
1407d88a1:     	movq	0x58(%rbp), %rdx
1407d88a5:     	testq	%rdx, %rdx
1407d88a8:     	je	0x1407d8818 <.text+0x7d7818>
1407d88ae:     	movq	0x60(%rbp), %rcx
1407d88b2:     	shlq	$0x3, %rdx
1407d88b6:     	movl	$0x8, %r8d
1407d88bc:     	callq	0x140613c20 <.text+0x612c20>
1407d88c1:     	movq	0x70(%rbp), %rdx
1407d88c5:     	testq	%rdx, %rdx
1407d88c8:     	jne	0x1407d8821 <.text+0x7d7821>
1407d88ce:     	jmp	0x1407d8834 <.text+0x7d7834>
1407d88d3:     	nopw	%cs:(%rax,%rax)
1407d88e0:     	movq	%rdx, 0x10(%rsp)
1407d88e5:     	pushq	%rbp
1407d88e6:     	pushq	%r15
1407d88e8:     	pushq	%r14
1407d88ea:     	pushq	%r13
1407d88ec:     	pushq	%r12
1407d88ee:     	pushq	%rsi
1407d88ef:     	pushq	%rdi
1407d88f0:     	pushq	%rbx
1407d88f1:     	subq	$0xc8, %rsp
1407d88f8:     	leaq	0x80(%rdx), %rbp
1407d88ff:     	movaps	%xmm14, 0x30(%rsp)
1407d8905:     	movaps	%xmm13, 0x40(%rsp)
1407d890b:     	movaps	%xmm12, 0x50(%rsp)
1407d8911:     	movdqa	%xmm11, 0x60(%rsp)
1407d8918:     	movaps	%xmm10, 0x70(%rsp)
1407d891e:     	movdqa	%xmm9, 0x80(%rsp)
1407d8928:     	movaps	%xmm8, 0x90(%rsp)
1407d8931:     	movdqa	%xmm7, 0xa0(%rsp)
1407d893a:     	movaps	%xmm6, 0xb0(%rsp)
1407d8942:     	movq	0xd8(%rbp), %rdx
1407d8949:     	testq	%rdx, %rdx
1407d894c:     	je	0x1407d8978 <.text+0x7d7978>
1407d894e:     	leaq	0x17(,%rdx,8), %rax
1407d8956:     	andq	$-0x10, %rax
1407d895a:     	addq	%rax, %rdx
1407d895d:     	addq	$0x11, %rdx
1407d8961:     	je	0x1407d8978 <.text+0x7d7978>
1407d8963:     	movq	0xd0(%rbp), %rcx
1407d896a:     	subq	%rax, %rcx
1407d896d:     	movl	$0x10, %r8d
1407d8973:     	callq	0x140613c20 <.text+0x612c20>
1407d8978:     	movaps	0xb0(%rsp), %xmm6
1407d8980:     	movdqa	0xa0(%rsp), %xmm7
1407d8989:     	movaps	0x90(%rsp), %xmm8
1407d8992:     	movdqa	0x80(%rsp), %xmm9
1407d899c:     	movaps	0x70(%rsp), %xmm10
1407d89a2:     	movdqa	0x60(%rsp), %xmm11
1407d89a9:     	movaps	0x50(%rsp), %xmm12
1407d89af:     	movaps	0x40(%rsp), %xmm13
1407d89b5:     	movaps	0x30(%rsp), %xmm14
1407d89bb:     	addq	$0xc8, %rsp
1407d89c2:     	popq	%rbx
1407d89c3:     	popq	%rdi
1407d89c4:     	popq	%rsi
1407d89c5:     	popq	%r12
1407d89c7:     	popq	%r13
1407d89c9:     	popq	%r14
1407d89cb:     	popq	%r15
1407d89cd:     	popq	%rbp
1407d89ce:     	retq
1407d89cf:     	nop
1407d89d0:     	movq	%rdx, 0x10(%rsp)
1407d89d5:     	pushq	%rbp
1407d89d6:     	pushq	%r15
1407d89d8:     	pushq	%r14
1407d89da:     	pushq	%r13
1407d89dc:     	pushq	%r12
1407d89de:     	pushq	%rsi
1407d89df:     	pushq	%rdi
1407d89e0:     	pushq	%rbx
1407d89e1:     	subq	$0xc8, %rsp
1407d89e8:     	leaq	0x80(%rdx), %rbp
1407d89ef:     	movaps	%xmm14, 0x30(%rsp)
1407d89f5:     	movaps	%xmm13, 0x40(%rsp)
1407d89fb:     	movaps	%xmm12, 0x50(%rsp)
1407d8a01:     	movdqa	%xmm11, 0x60(%rsp)
1407d8a08:     	movdqa	%xmm10, 0x70(%rsp)
1407d8a0f:     	movdqa	%xmm9, 0x80(%rsp)
1407d8a19:     	movaps	%xmm8, 0x90(%rsp)
1407d8a22:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8a2b:     	movaps	%xmm6, 0xb0(%rsp)
1407d8a33:     	movq	0x1f0(%rbp), %rax
1407d8a3a:     	incq	(%rax)
1407d8a3d:     	movaps	0xb0(%rsp), %xmm6
1407d8a45:     	movdqa	0xa0(%rsp), %xmm7
1407d8a4e:     	movaps	0x90(%rsp), %xmm8
1407d8a57:     	movdqa	0x80(%rsp), %xmm9
1407d8a61:     	movaps	0x70(%rsp), %xmm10
1407d8a67:     	movdqa	0x60(%rsp), %xmm11
1407d8a6e:     	movaps	0x50(%rsp), %xmm12
1407d8a74:     	movaps	0x40(%rsp), %xmm13
1407d8a7a:     	movaps	0x30(%rsp), %xmm14
1407d8a80:     	addq	$0xc8, %rsp
1407d8a87:     	popq	%rbx
1407d8a88:     	popq	%rdi
1407d8a89:     	popq	%rsi
1407d8a8a:     	popq	%r12
1407d8a8c:     	popq	%r13
1407d8a8e:     	popq	%r14
1407d8a90:     	popq	%r15
1407d8a92:     	popq	%rbp
1407d8a93:     	retq
1407d8a94:     	nopw	%cs:(%rax,%rax)
1407d8aa0:     	movq	%rdx, 0x10(%rsp)
1407d8aa5:     	pushq	%rbp
1407d8aa6:     	pushq	%r15
1407d8aa8:     	pushq	%r14
1407d8aaa:     	pushq	%r13
1407d8aac:     	pushq	%r12
1407d8aae:     	pushq	%rsi
1407d8aaf:     	pushq	%rdi
1407d8ab0:     	pushq	%rbx
1407d8ab1:     	subq	$0xc8, %rsp
1407d8ab8:     	leaq	0x80(%rdx), %rbp
1407d8abf:     	movaps	%xmm14, 0x30(%rsp)
1407d8ac5:     	movaps	%xmm13, 0x40(%rsp)
1407d8acb:     	movaps	%xmm12, 0x50(%rsp)
1407d8ad1:     	movdqa	%xmm11, 0x60(%rsp)
1407d8ad8:     	movaps	%xmm10, 0x70(%rsp)
1407d8ade:     	movdqa	%xmm9, 0x80(%rsp)
1407d8ae8:     	movaps	%xmm8, 0x90(%rsp)
1407d8af1:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8afa:     	movaps	%xmm6, 0xb0(%rsp)
1407d8b02:     	cmpb	$0x0, 0x1f0(%rbp)
1407d8b09:     	je	0x1407d8b37 <.text+0x7d7b37>
1407d8b0b:     	leaq	(%rbp), %rax
1407d8b0f:     	movq	%rax, 0x108(%rbp)
1407d8b16:     	leaq	0x108(%rbp), %rax
1407d8b1d:     	movq	%rax, 0x90(%rbp)
1407d8b24:     	leaq	0x22af0c5(%rip), %rcx   # 0x142a87bf0
1407d8b2b:     	leaq	0x90(%rbp), %rdx
1407d8b32:     	callq	0x141257bc0 <.text+0x1256bc0>
1407d8b37:     	movaps	0xb0(%rsp), %xmm6
1407d8b3f:     	movaps	0xa0(%rsp), %xmm7
1407d8b47:     	movaps	0x90(%rsp), %xmm8
1407d8b50:     	movaps	0x80(%rsp), %xmm9
1407d8b59:     	movaps	0x70(%rsp), %xmm10
1407d8b5f:     	movaps	0x60(%rsp), %xmm11
1407d8b65:     	movaps	0x50(%rsp), %xmm12
1407d8b6b:     	movaps	0x40(%rsp), %xmm13
1407d8b71:     	movaps	0x30(%rsp), %xmm14
1407d8b77:     	addq	$0xc8, %rsp
1407d8b7e:     	popq	%rbx
1407d8b7f:     	popq	%rdi
1407d8b80:     	popq	%rsi
1407d8b81:     	popq	%r12
1407d8b83:     	popq	%r13
1407d8b85:     	popq	%r14
1407d8b87:     	popq	%r15
1407d8b89:     	popq	%rbp
1407d8b8a:     	retq
1407d8b8b:     	nopl	(%rax,%rax)
1407d8b90:     	movq	%rdx, 0x10(%rsp)
1407d8b95:     	pushq	%rbp
1407d8b96:     	pushq	%r15
1407d8b98:     	pushq	%r14
1407d8b9a:     	pushq	%r13
1407d8b9c:     	pushq	%r12
1407d8b9e:     	pushq	%rsi
1407d8b9f:     	pushq	%rdi
1407d8ba0:     	pushq	%rbx
1407d8ba1:     	subq	$0xc8, %rsp
1407d8ba8:     	leaq	0x80(%rdx), %rbp
1407d8baf:     	movaps	%xmm14, 0x30(%rsp)
1407d8bb5:     	movaps	%xmm13, 0x40(%rsp)
1407d8bbb:     	movaps	%xmm12, 0x50(%rsp)
1407d8bc1:     	movdqa	%xmm11, 0x60(%rsp)
1407d8bc8:     	movaps	%xmm10, 0x70(%rsp)
1407d8bce:     	movdqa	%xmm9, 0x80(%rsp)
1407d8bd8:     	movaps	%xmm8, 0x90(%rsp)
1407d8be1:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8bea:     	movaps	%xmm6, 0xb0(%rsp)
1407d8bf2:     	movq	0x1f8(%rbp), %rax
1407d8bf9:     	incq	(%rax)
1407d8bfc:     	movaps	0xb0(%rsp), %xmm6
1407d8c04:     	movdqa	0xa0(%rsp), %xmm7
1407d8c0d:     	movaps	0x90(%rsp), %xmm8
1407d8c16:     	movdqa	0x80(%rsp), %xmm9
1407d8c20:     	movaps	0x70(%rsp), %xmm10
1407d8c26:     	movdqa	0x60(%rsp), %xmm11
1407d8c2d:     	movaps	0x50(%rsp), %xmm12
1407d8c33:     	movaps	0x40(%rsp), %xmm13
1407d8c39:     	movaps	0x30(%rsp), %xmm14
1407d8c3f:     	addq	$0xc8, %rsp
1407d8c46:     	popq	%rbx
1407d8c47:     	popq	%rdi
1407d8c48:     	popq	%rsi
1407d8c49:     	popq	%r12
1407d8c4b:     	popq	%r13
1407d8c4d:     	popq	%r14
1407d8c4f:     	popq	%r15
1407d8c51:     	popq	%rbp
1407d8c52:     	retq
1407d8c53:     	nopw	%cs:(%rax,%rax)
1407d8c60:     	movq	%rdx, 0x10(%rsp)
1407d8c65:     	pushq	%rbp
1407d8c66:     	pushq	%r15
1407d8c68:     	pushq	%r14
1407d8c6a:     	pushq	%r13
1407d8c6c:     	pushq	%r12
1407d8c6e:     	pushq	%rsi
1407d8c6f:     	pushq	%rdi
1407d8c70:     	pushq	%rbx
1407d8c71:     	subq	$0xc8, %rsp
1407d8c78:     	leaq	0x80(%rdx), %rbp
1407d8c7f:     	movaps	%xmm14, 0x30(%rsp)
1407d8c85:     	movaps	%xmm13, 0x40(%rsp)
1407d8c8b:     	movaps	%xmm12, 0x50(%rsp)
1407d8c91:     	movdqa	%xmm11, 0x60(%rsp)
1407d8c98:     	movaps	%xmm10, 0x70(%rsp)
1407d8c9e:     	movdqa	%xmm9, 0x80(%rsp)
1407d8ca8:     	movaps	%xmm8, 0x90(%rsp)
1407d8cb1:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8cba:     	movaps	%xmm6, 0xb0(%rsp)
1407d8cc2:     	movq	0x1f8(%rbp), %rax
1407d8cc9:     	incq	(%rax)
1407d8ccc:     	movaps	0xb0(%rsp), %xmm6
1407d8cd4:     	movdqa	0xa0(%rsp), %xmm7
1407d8cdd:     	movaps	0x90(%rsp), %xmm8
1407d8ce6:     	movdqa	0x80(%rsp), %xmm9
1407d8cf0:     	movaps	0x70(%rsp), %xmm10
1407d8cf6:     	movdqa	0x60(%rsp), %xmm11
1407d8cfd:     	movaps	0x50(%rsp), %xmm12
1407d8d03:     	movaps	0x40(%rsp), %xmm13
1407d8d09:     	movaps	0x30(%rsp), %xmm14
1407d8d0f:     	addq	$0xc8, %rsp
1407d8d16:     	popq	%rbx
1407d8d17:     	popq	%rdi
1407d8d18:     	popq	%rsi
1407d8d19:     	popq	%r12
1407d8d1b:     	popq	%r13
1407d8d1d:     	popq	%r14
1407d8d1f:     	popq	%r15
1407d8d21:     	popq	%rbp
1407d8d22:     	retq
1407d8d23:     	nopw	%cs:(%rax,%rax)
1407d8d30:     	movq	%rdx, 0x10(%rsp)
1407d8d35:     	pushq	%rbp
1407d8d36:     	pushq	%r15
1407d8d38:     	pushq	%r14
1407d8d3a:     	pushq	%r13
1407d8d3c:     	pushq	%r12
1407d8d3e:     	pushq	%rsi
1407d8d3f:     	pushq	%rdi
1407d8d40:     	pushq	%rbx
1407d8d41:     	subq	$0xc8, %rsp
1407d8d48:     	leaq	0x80(%rdx), %rbp
1407d8d4f:     	movaps	%xmm14, 0x30(%rsp)
1407d8d55:     	movaps	%xmm13, 0x40(%rsp)
1407d8d5b:     	movaps	%xmm12, 0x50(%rsp)
1407d8d61:     	movaps	%xmm11, 0x60(%rsp)
1407d8d67:     	movaps	%xmm10, 0x70(%rsp)
1407d8d6d:     	movaps	%xmm9, 0x80(%rsp)
1407d8d76:     	movaps	%xmm8, 0x90(%rsp)
1407d8d7f:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8d88:     	movaps	%xmm6, 0xb0(%rsp)
1407d8d90:     	cmpb	$0x0, 0x150(%rbp)
1407d8d97:     	je	0x1407d8ded <.text+0x7d7ded>
1407d8d99:     	callq	0x140c18130 <.text+0xc17130>
1407d8d9e:     	movq	(%rax), %rcx
1407d8da1:     	testq	%rcx, %rcx
1407d8da4:     	jne	0x1407d8dc1 <.text+0x7d7dc1>
1407d8da6:     	movq	%rax, %rcx
1407d8da9:     	xorl	%edx, %edx
1407d8dab:     	callq	0x1428f2580 <.text+0x28f1580>
1407d8db0:     	testq	%rax, %rax
1407d8db3:     	jne	0x1407d8dca <.text+0x7d7dca>
1407d8db5:     	leaq	0x244860c(%rip), %rcx   # 0x142c213c8
1407d8dbc:     	callq	0x1428d80d0 <.text+0x28d70d0>
1407d8dc1:     	cmpl	$0x2, %ecx
1407d8dc4:     	je	0x1407d8db5 <.text+0x7d7db5>
1407d8dc6:     	addq	$0x8, %rax
1407d8dca:     	cmpq	$0x0, (%rax)
1407d8dce:     	jne	0x1407d8e41 <.text+0x7d7e41>
1407d8dd0:     	movq	$-0x1, (%rax)
1407d8dd7:     	leaq	0x8(%rax), %rcx
1407d8ddb:     	movq	0xc0(%rbp), %rdx
1407d8de2:     	movq	%rax, %rsi
1407d8de5:     	callq	0x140c170f0 <.text+0xc160f0>
1407d8dea:     	incq	(%rsi)
1407d8ded:     	movaps	0xb0(%rsp), %xmm6
1407d8df5:     	movaps	0xa0(%rsp), %xmm7
1407d8dfd:     	movaps	0x90(%rsp), %xmm8
1407d8e06:     	movaps	0x80(%rsp), %xmm9
1407d8e0f:     	movaps	0x70(%rsp), %xmm10
1407d8e15:     	movaps	0x60(%rsp), %xmm11
1407d8e1b:     	movaps	0x50(%rsp), %xmm12
1407d8e21:     	movaps	0x40(%rsp), %xmm13
1407d8e27:     	movaps	0x30(%rsp), %xmm14
1407d8e2d:     	addq	$0xc8, %rsp
1407d8e34:     	popq	%rbx
1407d8e35:     	popq	%rdi
1407d8e36:     	popq	%rsi
1407d8e37:     	popq	%r12
1407d8e39:     	popq	%r13
1407d8e3b:     	popq	%r14
1407d8e3d:     	popq	%r15
1407d8e3f:     	popq	%rbp
1407d8e40:     	retq
1407d8e41:     	leaq	0x24485f0(%rip), %rcx   # 0x142c21438
1407d8e48:     	callq	0x1428d9250 <.text+0x28d8250>
1407d8e4d:     	int3
1407d8e4e:     	nop
1407d8e50:     	movq	%rdx, 0x10(%rsp)
1407d8e55:     	pushq	%rbp
1407d8e56:     	pushq	%r15
1407d8e58:     	pushq	%r14
1407d8e5a:     	pushq	%r13
1407d8e5c:     	pushq	%r12
1407d8e5e:     	pushq	%rsi
1407d8e5f:     	pushq	%rdi
1407d8e60:     	pushq	%rbx
1407d8e61:     	subq	$0xc8, %rsp
1407d8e68:     	leaq	0x80(%rdx), %rbp
1407d8e6f:     	movaps	%xmm14, 0x30(%rsp)
1407d8e75:     	movaps	%xmm13, 0x40(%rsp)
1407d8e7b:     	movaps	%xmm12, 0x50(%rsp)
1407d8e81:     	movaps	%xmm11, 0x60(%rsp)
1407d8e87:     	movaps	%xmm10, 0x70(%rsp)
1407d8e8d:     	movaps	%xmm9, 0x80(%rsp)
1407d8e96:     	movaps	%xmm8, 0x90(%rsp)
1407d8e9f:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8ea8:     	movaps	%xmm6, 0xb0(%rsp)
1407d8eb0:     	movq	0x1f8(%rbp), %rax
1407d8eb7:     	incq	(%rax)
1407d8eba:     	movaps	0xb0(%rsp), %xmm6
1407d8ec2:     	movdqa	0xa0(%rsp), %xmm7
1407d8ecb:     	movaps	0x90(%rsp), %xmm8
1407d8ed4:     	movdqa	0x80(%rsp), %xmm9
1407d8ede:     	movaps	0x70(%rsp), %xmm10
1407d8ee4:     	movdqa	0x60(%rsp), %xmm11
1407d8eeb:     	movaps	0x50(%rsp), %xmm12
1407d8ef1:     	movaps	0x40(%rsp), %xmm13
1407d8ef7:     	movaps	0x30(%rsp), %xmm14
1407d8efd:     	addq	$0xc8, %rsp
1407d8f04:     	popq	%rbx
1407d8f05:     	popq	%rdi
1407d8f06:     	popq	%rsi
1407d8f07:     	popq	%r12
1407d8f09:     	popq	%r13
1407d8f0b:     	popq	%r14
1407d8f0d:     	popq	%r15
1407d8f0f:     	popq	%rbp
1407d8f10:     	retq
1407d8f11:     	nopw	%cs:(%rax,%rax)
1407d8f20:     	movq	%rdx, 0x10(%rsp)
1407d8f25:     	pushq	%rbp
1407d8f26:     	pushq	%r15
1407d8f28:     	pushq	%r14
1407d8f2a:     	pushq	%r13
1407d8f2c:     	pushq	%r12
1407d8f2e:     	pushq	%rsi
1407d8f2f:     	pushq	%rdi
1407d8f30:     	pushq	%rbx
1407d8f31:     	subq	$0xc8, %rsp
1407d8f38:     	leaq	0x80(%rdx), %rbp
1407d8f3f:     	movaps	%xmm14, 0x30(%rsp)
1407d8f45:     	movaps	%xmm13, 0x40(%rsp)
1407d8f4b:     	movaps	%xmm12, 0x50(%rsp)
1407d8f51:     	movaps	%xmm11, 0x60(%rsp)
1407d8f57:     	movaps	%xmm10, 0x70(%rsp)
1407d8f5d:     	movaps	%xmm9, 0x80(%rsp)
1407d8f66:     	movaps	%xmm8, 0x90(%rsp)
1407d8f6f:     	movdqa	%xmm7, 0xa0(%rsp)
1407d8f78:     	movaps	%xmm6, 0xb0(%rsp)
1407d8f80:     	movq	0x1f8(%rbp), %rax
1407d8f87:     	incq	(%rax)
1407d8f8a:     	movaps	0xb0(%rsp), %xmm6
1407d8f92:     	movaps	0xa0(%rsp), %xmm7
1407d8f9a:     	movaps	0x90(%rsp), %xmm8
1407d8fa3:     	movaps	0x80(%rsp), %xmm9
1407d8fac:     	movaps	0x70(%rsp), %xmm10
1407d8fb2:     	movaps	0x60(%rsp), %xmm11
1407d8fb8:     	movaps	0x50(%rsp), %xmm12
1407d8fbe:     	movaps	0x40(%rsp), %xmm13
1407d8fc4:     	movaps	0x30(%rsp), %xmm14
1407d8fca:     	addq	$0xc8, %rsp
1407d8fd1:     	popq	%rbx
1407d8fd2:     	popq	%rdi
1407d8fd3:     	popq	%rsi
1407d8fd4:     	popq	%r12
1407d8fd6:     	popq	%r13
1407d8fd8:     	popq	%r14
1407d8fda:     	popq	%r15
1407d8fdc:     	popq	%rbp
1407d8fdd:     	retq
