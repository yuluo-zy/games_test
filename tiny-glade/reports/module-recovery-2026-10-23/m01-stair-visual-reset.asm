
../B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140aa8a60:     	pushq	%rbp
140aa8a61:     	pushq	%r15
140aa8a63:     	pushq	%r14
140aa8a65:     	pushq	%r13
140aa8a67:     	pushq	%r12
140aa8a69:     	pushq	%rsi
140aa8a6a:     	pushq	%rdi
140aa8a6b:     	pushq	%rbx
140aa8a6c:     	subq	$0x148, %rsp            # imm = 0x148
140aa8a73:     	leaq	0x80(%rsp), %rbp
140aa8a7b:     	movq	$-0x2, 0xc0(%rbp)
140aa8a86:     	movl	%r9d, %edi
140aa8a89:     	movl	%r8d, %r12d
140aa8a8c:     	movq	%rdx, %r14
140aa8a8f:     	movq	%rcx, 0xa0(%rbp)
140aa8a96:     	movq	0x148(%rbp), %r15
140aa8a9d:     	movl	%r8d, 0x48(%rbp)
140aa8aa1:     	movl	%r9d, 0x4c(%rbp)
140aa8aa5:     	leaq	0x20(%rdx), %r13
140aa8aa9:     	leaq	0x48(%rbp), %rdx
140aa8aad:     	movq	%r13, %rcx
140aa8ab0:     	callq	0x1405b1be0 <.text+0x5b0be0>
140aa8ab5:     	movq	%rax, %rbx
140aa8ab8:     	shrq	$0x39, %rbx
140aa8abc:     	movq	(%r14), %r9
140aa8abf:     	movq	0x8(%r14), %rcx
140aa8ac3:     	movq	%rcx, %rdx
140aa8ac6:     	andq	%rax, %rdx
140aa8ac9:     	movd	%ebx, %xmm0
140aa8acd:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140aa8ad1:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140aa8ad6:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140aa8adb:     	xorl	%r8d, %r8d
140aa8ade:     	pcmpeqd	%xmm1, %xmm1
140aa8ae2:     	movq	%r9, %rsi
140aa8ae5:     	movdqu	(%r9,%rdx), %xmm2
140aa8aeb:     	movdqa	%xmm2, %xmm3
140aa8aef:     	pcmpeqb	%xmm0, %xmm3
140aa8af3:     	pmovmskb	%xmm3, %r9d
140aa8af8:     	testl	%r9d, %r9d
140aa8afb:     	je	0x140aa8b40 <.text+0xaa7b40>
140aa8afd:     	tzcntl	%r9d, %r10d
140aa8b02:     	addq	%rdx, %r10
140aa8b05:     	andq	%rcx, %r10
140aa8b08:     	negq	%r10
140aa8b0b:     	imulq	$0x110, %r10, %r10      # imm = 0x110
140aa8b12:     	cmpl	%r12d, -0x110(%rsi,%r10)
140aa8b1a:     	jne	0x140aa8b29 <.text+0xaa7b29>
140aa8b1c:     	movq	%rsi, %r11
140aa8b1f:     	cmpl	%edi, -0x10c(%rsi,%r10)
140aa8b27:     	je	0x140aa8b68 <.text+0xaa7b68>
140aa8b29:     	leal	-0x1(%r9), %r10d
140aa8b2d:     	andw	%r9w, %r10w
140aa8b31:     	movl	%r10d, %r9d
140aa8b34:     	jne	0x140aa8afd <.text+0xaa7afd>
140aa8b36:     	nopw	%cs:(%rax,%rax)
140aa8b40:     	pcmpeqb	%xmm1, %xmm2
140aa8b44:     	pmovmskb	%xmm2, %r9d
140aa8b49:     	testl	%r9d, %r9d
140aa8b4c:     	jne	0x140aa8edd <.text+0xaa7edd>
140aa8b52:     	addq	%r8, %rdx
140aa8b55:     	addq	$0x10, %rdx
140aa8b59:     	addq	$0x10, %r8
140aa8b5d:     	andq	%rcx, %rdx
140aa8b60:     	movq	%rsi, %r9
140aa8b63:     	jmp	0x140aa8ae2 <.text+0xaa7ae2>
140aa8b68:     	addq	%r10, %r11
140aa8b6b:     	movq	%r11, %r13
140aa8b6e:     	movq	0x130(%rbp), %rsi
140aa8b75:     	cmpq	$0x0, -0x30(%r13)
140aa8b7a:     	movq	0x138(%rbp), %rbx
140aa8b81:     	je	0x140aa9084 <.text+0xaa8084>
140aa8b87:     	movq	-0x28(%r13), %r14
140aa8b8b:     	testq	%r14, %r14
140aa8b8e:     	je	0x140aa910c <.text+0xaa810c>
140aa8b94:     	leaq	0x18(%r14), %rcx
140aa8b98:     	movb	$0x1, %dl
140aa8b9a:     	xorl	%eax, %eax
140aa8b9c:     	lock
140aa8b9d:     	cmpxchgb	%dl, 0x18(%r14)
140aa8ba2:     	jne	0x140aa91d4 <.text+0xaa81d4>
140aa8ba8:     	callq	0x140c81c70 <.text+0xc80c70>
140aa8bad:     	movq	0x10(%r14), %rax
140aa8bb1:     	testq	%rax, %rax
140aa8bb4:     	sete	%cl
140aa8bb7:     	shlq	$0x2, %rax
140aa8bbb:     	addq	0x8(%r14), %rax
140aa8bbf:     	addq	$-0x4, %rax
140aa8bc3:     	sete	%dl
140aa8bc6:     	orb	%cl, %dl
140aa8bc8:     	je	0x140aa8bdc <.text+0xaa7bdc>
140aa8bca:     	movq	0x28(%rbx), %rax
140aa8bce:     	testq	%rax, %rax
140aa8bd1:     	je	0x140aa9275 <.text+0xaa8275>
140aa8bd7:     	leal	-0x1(%rax), %ecx
140aa8bda:     	jmp	0x140aa8be2 <.text+0xaa7be2>
140aa8bdc:     	movl	(%rax), %ecx
140aa8bde:     	movq	0x28(%rbx), %rax
140aa8be2:     	movq	0x20(%rbx), %rdx
140aa8be6:     	movdqu	0x30(%rbx), %xmm0
140aa8beb:     	movdqu	0x40(%rbx), %xmm1
140aa8bf0:     	movdqu	0x50(%rbx), %xmm2
140aa8bf5:     	movl	%ecx, 0x40(%rbp)
140aa8bf8:     	movq	%rdx, -0x10(%rbp)
140aa8bfc:     	movq	%rax, -0x8(%rbp)
140aa8c00:     	movdqu	%xmm0, (%rbp)
140aa8c05:     	movq	%r14, 0x10(%rbp)
140aa8c09:     	movdqu	%xmm1, 0x18(%rbp)
140aa8c0e:     	movq	%rbx, 0x28(%rbp)
140aa8c12:     	movdqu	%xmm2, 0x30(%rbp)
140aa8c17:     	leaq	-0x10(%rbp), %rcx
140aa8c1b:     	callq	0x1412e4760 <.text+0x12e3760>
140aa8c20:     	leaq	-0xc0(%r13), %rcx
140aa8c27:     	movq	%rbx, %rdx
140aa8c2a:     	movq	0x140(%rbp), %r8
140aa8c31:     	callq	0x140aa8210 <.text+0xaa7210>
140aa8c36:     	cmpq	$0x0, -0x20(%r13)
140aa8c3b:     	je	0x140aa9118 <.text+0xaa8118>
140aa8c41:     	movq	-0x18(%r13), %rdi
140aa8c45:     	testq	%rdi, %rdi
140aa8c48:     	je	0x140aa9198 <.text+0xaa8198>
140aa8c4e:     	leaq	0x18(%rdi), %rcx
140aa8c52:     	movb	$0x1, %dl
140aa8c54:     	xorl	%eax, %eax
140aa8c56:     	lock
140aa8c57:     	cmpxchgb	%dl, 0x18(%rdi)
140aa8c5b:     	jne	0x140aa91ea <.text+0xaa81ea>
140aa8c61:     	callq	0x140c81c70 <.text+0xc80c70>
140aa8c66:     	movq	0x10(%rdi), %rax
140aa8c6a:     	testq	%rax, %rax
140aa8c6d:     	sete	%cl
140aa8c70:     	shlq	$0x2, %rax
140aa8c74:     	addq	0x8(%rdi), %rax
140aa8c78:     	addq	$-0x4, %rax
140aa8c7c:     	sete	%dl
140aa8c7f:     	orb	%cl, %dl
140aa8c81:     	je	0x140aa8c95 <.text+0xaa7c95>
140aa8c83:     	movq	0x28(%r15), %rax
140aa8c87:     	testq	%rax, %rax
140aa8c8a:     	je	0x140aa9281 <.text+0xaa8281>
140aa8c90:     	leal	-0x1(%rax), %ecx
140aa8c93:     	jmp	0x140aa8c9b <.text+0xaa7c9b>
140aa8c95:     	movl	(%rax), %ecx
140aa8c97:     	movq	0x28(%r15), %rax
140aa8c9b:     	movq	0x20(%r15), %rdx
140aa8c9f:     	movdqu	0x30(%r15), %xmm0
140aa8ca5:     	movdqu	0x40(%r15), %xmm1
140aa8cab:     	movdqu	0x50(%r15), %xmm2
140aa8cb1:     	movl	%ecx, 0x98(%rbp)
140aa8cb7:     	movq	%rdx, 0x48(%rbp)
140aa8cbb:     	movq	%rax, 0x50(%rbp)
140aa8cbf:     	movdqu	%xmm0, 0x58(%rbp)
140aa8cc4:     	movq	%rdi, 0x68(%rbp)
140aa8cc8:     	movdqu	%xmm1, 0x70(%rbp)
140aa8ccd:     	movq	%r15, 0x80(%rbp)
140aa8cd4:     	movdqu	%xmm2, 0x88(%rbp)
140aa8cdc:     	leaq	0x48(%rbp), %rcx
140aa8ce0:     	callq	0x1412e4760 <.text+0x12e3760>
140aa8ce5:     	leaq	-0x108(%r13), %rcx
140aa8cec:     	movq	-0x100(%r13), %rdx
140aa8cf3:     	movq	-0xf8(%r13), %r12
140aa8cfa:     	movq	%r13, 0xa8(%rbp)
140aa8d01:     	movq	$0x0, -0xf8(%r13)
140aa8d0c:     	leaq	(%rdx,%r12,8), %rax
140aa8d10:     	movq	%rax, -0x30(%rbp)
140aa8d14:     	movq	%rcx, 0xb0(%rbp)
140aa8d1b:     	movq	%rcx, -0x28(%rbp)
140aa8d1f:     	movq	%r12, -0x20(%rbp)
140aa8d23:     	movq	$0x0, -0x18(%rbp)
140aa8d2b:     	testq	%r12, %r12
140aa8d2e:     	je	0x140aa8da4 <.text+0xaa7da4>
140aa8d30:     	movq	(%rsi), %r13
140aa8d33:     	movq	0x8(%rsi), %r14
140aa8d37:     	movq	%rsi, %rax
140aa8d3a:     	movq	0x18(%rsi), %rsi
140aa8d3e:     	movq	0x10(%rax), %r15
140aa8d42:     	addq	$0x8, %rdx
140aa8d46:     	shlq	$0x3, %r12
140aa8d4a:     	leaq	0x203461f(%rip), %rdi   # 0x142add370
140aa8d51:     	leaq	-0x60(%rbp), %rbx
140aa8d55:     	nopw	%cs:(%rax,%rax)
140aa8d60:     	movq	-0x8(%rdx), %rax
140aa8d64:     	movq	%rax, %rcx
140aa8d67:     	shrq	$0x20, %rcx
140aa8d6b:     	je	0x140aa8da4 <.text+0xaa7da4>
140aa8d6d:     	movq	%rdx, 0xb8(%rbp)
140aa8d74:     	movq	%rax, -0x40(%rbp)
140aa8d78:     	movq	%r13, -0x60(%rbp)
140aa8d7c:     	movq	%r14, -0x58(%rbp)
140aa8d80:     	movq	%r15, -0x50(%rbp)
140aa8d84:     	movq	%rsi, -0x48(%rbp)
140aa8d88:     	movq	%rbx, %rcx
140aa8d8b:     	movq	%rdi, %rdx
140aa8d8e:     	callq	0x140d14770 <.text+0xd13770>
140aa8d93:     	movq	0xb8(%rbp), %rdx
140aa8d9a:     	addq	$0x8, %rdx
140aa8d9e:     	addq	$-0x8, %r12
140aa8da2:     	jne	0x140aa8d60 <.text+0xaa7d60>
140aa8da4:     	movq	0xa8(%rbp), %r14
140aa8dab:     	leaq	-0x60(%r14), %rdi
140aa8daf:     	movq	$0x0, -0xe0(%r14)
140aa8dba:     	movq	$0x0, -0xc8(%r14)
140aa8dc5:     	cmpq	$0x0, -0x48(%r14)
140aa8dca:     	movq	0xa0(%rbp), %rbx
140aa8dd1:     	je	0x140aa8e10 <.text+0xaa7e10>
140aa8dd3:     	movq	-0x58(%r14), %rsi
140aa8dd7:     	testq	%rsi, %rsi
140aa8dda:     	je	0x140aa8dea <.text+0xaa7dea>
140aa8ddc:     	movq	(%rdi), %rcx
140aa8ddf:     	leaq	0x11(%rsi), %r8
140aa8de3:     	movb	$-0x1, %dl
140aa8de5:     	callq	0x142923c20 <.text+0x2922c20>
140aa8dea:     	movq	$0x0, -0x48(%r14)
140aa8df2:     	leaq	0x1(%rsi), %rax
140aa8df6:     	movq	%rax, %rcx
140aa8df9:     	shrq	$0x3, %rcx
140aa8dfd:     	andq	$-0x8, %rax
140aa8e01:     	subq	%rcx, %rax
140aa8e04:     	cmpq	$0x8, %rsi
140aa8e08:     	cmovbq	%rsi, %rax
140aa8e0c:     	movq	%rax, -0x50(%r14)
140aa8e10:     	leaq	-0xd8(%r14), %rax
140aa8e17:     	leaq	-0xf0(%r14), %rcx
140aa8e1e:     	movq	$0x0, -0x68(%r14)
140aa8e26:     	movw	$0x0, -0x8(%r14)
140aa8e2d:     	movq	$0x0, -0x10(%r14)
140aa8e35:     	movq	0x40(%rbp), %rdx
140aa8e39:     	movq	%rdx, 0x50(%rbx)
140aa8e3d:     	movups	0x30(%rbp), %xmm0
140aa8e41:     	movups	%xmm0, 0x40(%rbx)
140aa8e45:     	movups	-0x10(%rbp), %xmm0
140aa8e49:     	movups	(%rbp), %xmm1
140aa8e4d:     	movups	0x10(%rbp), %xmm2
140aa8e51:     	movups	0x20(%rbp), %xmm3
140aa8e55:     	movups	%xmm3, 0x30(%rbx)
140aa8e59:     	movups	%xmm2, 0x20(%rbx)
140aa8e5d:     	movups	%xmm1, 0x10(%rbx)
140aa8e61:     	movups	%xmm0, (%rbx)
140aa8e64:     	movups	0x48(%rbp), %xmm0
140aa8e68:     	movups	0x58(%rbp), %xmm1
140aa8e6c:     	movups	0x68(%rbp), %xmm2
140aa8e70:     	movups	0x78(%rbp), %xmm3
140aa8e74:     	movups	%xmm0, 0x58(%rbx)
140aa8e78:     	movups	%xmm1, 0x68(%rbx)
140aa8e7c:     	movups	%xmm2, 0x78(%rbx)
140aa8e80:     	movups	%xmm3, 0x88(%rbx)
140aa8e87:     	movups	0x88(%rbp), %xmm0
140aa8e8e:     	movups	%xmm0, 0x98(%rbx)
140aa8e95:     	movq	0x98(%rbp), %rdx
140aa8e9c:     	movq	%rdx, 0xa8(%rbx)
140aa8ea3:     	movq	0xb0(%rbp), %rdx
140aa8eaa:     	movq	%rdx, 0xb0(%rbx)
140aa8eb1:     	movq	%rdi, 0xb8(%rbx)
140aa8eb8:     	movq	%rcx, 0xc0(%rbx)
140aa8ebf:     	movq	%rax, 0xc8(%rbx)
140aa8ec6:     	movq	%rbx, %rax
140aa8ec9:     	addq	$0x148, %rsp            # imm = 0x148
140aa8ed0:     	popq	%rbx
140aa8ed1:     	popq	%rdi
140aa8ed2:     	popq	%rsi
140aa8ed3:     	popq	%r12
140aa8ed5:     	popq	%r13
140aa8ed7:     	popq	%r14
140aa8ed9:     	popq	%r15
140aa8edb:     	popq	%rbp
140aa8edc:     	retq
140aa8edd:     	movq	0x130(%rbp), %r15
140aa8ee4:     	movq	%rax, %rsi
140aa8ee7:     	cmpq	$0x0, 0x10(%r14)
140aa8eec:     	je	0x140aa9228 <.text+0xaa8228>
140aa8ef2:     	xorl	%ecx, %ecx
140aa8ef4:     	callq	0x1400d9b20 <.text+0xd8b20>
140aa8ef9:     	testq	%rax, %rax
140aa8efc:     	je	0x140aa9252 <.text+0xaa8252>
140aa8f02:     	movdqu	(%rax), %xmm0
140aa8f06:     	incq	(%rax)
140aa8f09:     	movq	(%r14), %rax
140aa8f0c:     	movq	0x8(%r14), %rcx
140aa8f10:     	movq	%rsi, %rdx
140aa8f13:     	andq	%rcx, %rdx
140aa8f16:     	movdqu	(%rax,%rdx), %xmm1
140aa8f1b:     	pmovmskb	%xmm1, %r8d
140aa8f20:     	testl	%r8d, %r8d
140aa8f23:     	je	0x140aa9200 <.text+0xaa8200>
140aa8f29:     	movq	%r15, %rsi
140aa8f2c:     	tzcntl	%r8d, %r8d
140aa8f31:     	addq	%rdx, %r8
140aa8f34:     	andq	%rcx, %r8
140aa8f37:     	movzbl	(%rax,%r8), %edx
140aa8f3c:     	testb	%dl, %dl
140aa8f3e:     	movq	0x148(%rbp), %r15
140aa8f45:     	jns	0x140aa925e <.text+0xaa825e>
140aa8f4b:     	leaq	-0x10(%r8), %r9
140aa8f4f:     	andq	%rcx, %r9
140aa8f52:     	movb	%bl, (%rax,%r8)
140aa8f56:     	movb	%bl, 0x10(%rax,%r9)
140aa8f5b:     	negq	%r8
140aa8f5e:     	imulq	$0x110, %r8, %rcx       # imm = 0x110
140aa8f65:     	andb	$0x1, %dl
140aa8f68:     	movzbl	%dl, %edx
140aa8f6b:     	subq	%rdx, 0x10(%r14)
140aa8f6f:     	leaq	(%rax,%rcx), %r13
140aa8f73:     	movl	%r12d, -0x110(%rax,%rcx)
140aa8f7b:     	movl	%edi, -0x10c(%rax,%rcx)
140aa8f82:     	movq	$0x0, -0x108(%rax,%rcx)
140aa8f8e:     	movq	$0x8, -0x100(%rax,%rcx)
140aa8f9a:     	pxor	%xmm1, %xmm1
140aa8f9e:     	movdqu	%xmm1, -0xf8(%rax,%rcx)
140aa8fa7:     	movq	$0x10, -0xe8(%rax,%rcx)
140aa8fb3:     	movdqu	%xmm1, -0xe0(%rax,%rcx)
140aa8fbc:     	movq	$0x8, -0xd0(%rax,%rcx)
140aa8fc8:     	movdqu	%xmm1, -0xc8(%rax,%rcx)
140aa8fd1:     	movq	$0x10, -0xb8(%rax,%rcx)
140aa8fdd:     	movq	$0x0, -0xb0(%rax,%rcx)
140aa8fe9:     	movl	$0x0, -0xa8(%rax,%rcx)
140aa8ff4:     	movq	$0x0, -0xa0(%rax,%rcx)
140aa9000:     	movq	$0x0, -0x90(%rax,%rcx)
140aa900c:     	movb	$0x0, -0x80(%rax,%rcx)
140aa9011:     	movq	$0x0, -0x78(%rax,%rcx)
140aa901a:     	movq	$0x4, -0x70(%rax,%rcx)
140aa9023:     	movq	$0x0, -0x68(%rax,%rcx)
140aa902c:     	movups	0x21c2ecd(%rip), %xmm1  # 0x142c6bf00
140aa9033:     	movups	%xmm1, -0x60(%rax,%rcx)
140aa9038:     	movdqu	0x21c2ed0(%rip), %xmm1  # 0x142c6bf10
140aa9040:     	movdqu	%xmm1, -0x50(%rax,%rcx)
140aa9046:     	movdqu	%xmm0, -0x40(%rax,%rcx)
140aa904c:     	movq	$0x0, -0x30(%rax,%rcx)
140aa9055:     	movq	$0x0, -0x20(%rax,%rcx)
140aa905e:     	movq	$0x0, -0x10(%rax,%rcx)
140aa9067:     	movw	$0x0, -0x8(%rax,%rcx)
140aa906e:     	incq	0x18(%r14)
140aa9072:     	cmpq	$0x0, -0x30(%r13)
140aa9077:     	movq	0x138(%rbp), %rbx
140aa907e:     	jne	0x140aa8b87 <.text+0xaa7b87>
140aa9084:     	movzbl	0x2b8bb7e(%rip), %eax   # 0x143634c09
140aa908b:     	movl	$0x20, %ecx
140aa9090:     	movl	$0x8, %edx
140aa9095:     	callq	0x140613c10 <.text+0x612c10>
140aa909a:     	testq	%rax, %rax
140aa909d:     	je	0x140aa928f <.text+0xaa828f>
140aa90a3:     	movq	%rax, %r14
140aa90a6:     	movq	$0x0, (%rax)
140aa90ad:     	movq	$0x4, 0x8(%rax)
140aa90b5:     	movq	$0x0, 0x10(%rax)
140aa90bd:     	movb	$0x0, 0x18(%rax)
140aa90c1:     	movq	0x60(%rbx), %rcx
140aa90c5:     	jmp	0x140aa90d2 <.text+0xaa80d2>
140aa90c7:     	nopw	(%rax,%rax)
140aa90d0:     	pause
140aa90d2:     	movq	0x8(%rcx), %rax
140aa90d6:     	nopw	%cs:(%rax,%rax)
140aa90e0:     	cmpq	$-0x1, %rax
140aa90e4:     	je	0x140aa90d0 <.text+0xaa80d0>
140aa90e6:     	testq	%rax, %rax
140aa90e9:     	js	0x140aa91a9 <.text+0xaa81a9>
140aa90ef:     	leaq	0x1(%rax), %rdx
140aa90f3:     	lock
140aa90f4:     	cmpxchgq	%rdx, 0x8(%rcx)
140aa90f9:     	jne	0x140aa90e0 <.text+0xaa80e0>
140aa90fb:     	movq	%rcx, -0x30(%r13)
140aa90ff:     	movq	%r14, -0x28(%r13)
140aa9103:     	testq	%r14, %r14
140aa9106:     	jne	0x140aa8b94 <.text+0xaa7b94>
140aa910c:     	leaq	0x2185a65(%rip), %rcx   # 0x142c2eb78
140aa9113:     	callq	0x1428d9310 <.text+0x28d8310>
140aa9118:     	movzbl	0x2b8baea(%rip), %eax   # 0x143634c09
140aa911f:     	movl	$0x20, %ecx
140aa9124:     	movl	$0x8, %edx
140aa9129:     	callq	0x140613c10 <.text+0x612c10>
140aa912e:     	testq	%rax, %rax
140aa9131:     	je	0x140aa929e <.text+0xaa829e>
140aa9137:     	movq	%rax, %rdi
140aa913a:     	movq	$0x0, (%rax)
140aa9141:     	movq	$0x4, 0x8(%rax)
140aa9149:     	movq	$0x0, 0x10(%rax)
140aa9151:     	movb	$0x0, 0x18(%rax)
140aa9155:     	movq	0x60(%r15), %rcx
140aa9159:     	jmp	0x140aa9162 <.text+0xaa8162>
140aa915b:     	nopl	(%rax,%rax)
140aa9160:     	pause
140aa9162:     	movq	0x8(%rcx), %rax
140aa9166:     	nopw	%cs:(%rax,%rax)
140aa9170:     	cmpq	$-0x1, %rax
140aa9174:     	je	0x140aa9160 <.text+0xaa8160>
140aa9176:     	testq	%rax, %rax
140aa9179:     	js	0x140aa91bc <.text+0xaa81bc>
140aa917b:     	leaq	0x1(%rax), %rdx
140aa917f:     	lock
140aa9180:     	cmpxchgq	%rdx, 0x8(%rcx)
140aa9185:     	jne	0x140aa9170 <.text+0xaa8170>
140aa9187:     	movq	%rcx, -0x20(%r13)
140aa918b:     	movq	%rdi, -0x18(%r13)
140aa918f:     	testq	%rdi, %rdi
140aa9192:     	jne	0x140aa8c4e <.text+0xaa7c4e>
140aa9198:     	leaq	0x21859d9(%rip), %rcx   # 0x142c2eb78
140aa919f:     	callq	0x1428d9310 <.text+0x28d8310>
140aa91a4:     	jmp	0x140aa92ad <.text+0xaa82ad>
140aa91a9:     	leaq	0x2048190(%rip), %rcx   # 0x142af1340
140aa91b0:     	leaq	0x20481e9(%rip), %rdx   # 0x142af13a0
140aa91b7:     	callq	0x1428a9a00 <.text+0x28a8a00>
140aa91bc:     	leaq	0x204817d(%rip), %rcx   # 0x142af1340
140aa91c3:     	leaq	0x20481d6(%rip), %rdx   # 0x142af13a0
140aa91ca:     	callq	0x1428a9a00 <.text+0x28a8a00>
140aa91cf:     	jmp	0x140aa92ad <.text+0xaa82ad>
140aa91d4:     	movq	%rcx, %rdi
140aa91d7:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140aa91dd:     	callq	0x1428cd210 <.text+0x28cc210>
140aa91e2:     	movq	%rdi, %rcx
140aa91e5:     	jmp	0x140aa8ba8 <.text+0xaa7ba8>
140aa91ea:     	movq	%rcx, %rbx
140aa91ed:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
140aa91f3:     	callq	0x1428cd210 <.text+0x28cc210>
140aa91f8:     	movq	%rbx, %rcx
140aa91fb:     	jmp	0x140aa8c61 <.text+0xaa7c61>
140aa9200:     	movl	$0x10, %r9d
140aa9206:     	movq	%r15, %rsi
140aa9209:     	addq	%r9, %rdx
140aa920c:     	andq	%rcx, %rdx
140aa920f:     	movdqu	(%rax,%rdx), %xmm1
140aa9214:     	pmovmskb	%xmm1, %r8d
140aa9219:     	addq	$0x10, %r9
140aa921d:     	testl	%r8d, %r8d
140aa9220:     	jne	0x140aa8f2c <.text+0xaa7f2c>
140aa9226:     	jmp	0x140aa9209 <.text+0xaa8209>
140aa9228:     	movl	$0x1, %edx
140aa922d:     	movq	%r14, %rcx
140aa9230:     	movq	%r13, %r8
140aa9233:     	movb	$0x1, %r9b
140aa9236:     	callq	0x1428c67b0 <.text+0x28c57b0>
140aa923b:     	movl	0x48(%rbp), %r12d
140aa923f:     	movl	0x4c(%rbp), %edi
140aa9242:     	xorl	%ecx, %ecx
140aa9244:     	callq	0x1400d9b20 <.text+0xd8b20>
140aa9249:     	testq	%rax, %rax
140aa924c:     	jne	0x140aa8f02 <.text+0xaa7f02>
140aa9252:     	leaq	0x2191847(%rip), %rcx   # 0x142c3aaa0
140aa9259:     	callq	0x1428d80d0 <.text+0x28d70d0>
140aa925e:     	movdqa	(%rax), %xmm1
140aa9262:     	pmovmskb	%xmm1, %edx
140aa9266:     	tzcntl	%edx, %r8d
140aa926b:     	movzbl	(%rax,%r8), %edx
140aa9270:     	jmp	0x140aa8f4b <.text+0xaa7f4b>
140aa9275:     	leaq	0x2185914(%rip), %rcx   # 0x142c2eb90
140aa927c:     	callq	0x1428d9310 <.text+0x28d8310>
140aa9281:     	leaq	0x2185908(%rip), %rcx   # 0x142c2eb90
140aa9288:     	callq	0x1428d9310 <.text+0x28d8310>
140aa928d:     	jmp	0x140aa92ad <.text+0xaa82ad>
140aa928f:     	movl	$0x8, %ecx
140aa9294:     	movl	$0x20, %edx
140aa9299:     	callq	0x1428d9000 <.text+0x28d8000>
140aa929e:     	movl	$0x8, %ecx
140aa92a3:     	movl	$0x20, %edx
140aa92a8:     	callq	0x1428d9000 <.text+0x28d8000>
140aa92ad:     	ud2
140aa92af:     	nop
140aa92b0:     	movq	%rdx, 0x10(%rsp)
140aa92b5:     	pushq	%rbp
140aa92b6:     	pushq	%r15
140aa92b8:     	pushq	%r14
140aa92ba:     	pushq	%r13
140aa92bc:     	pushq	%r12
140aa92be:     	pushq	%rsi
140aa92bf:     	pushq	%rdi
140aa92c0:     	pushq	%rbx
140aa92c1:     	subq	$0x28, %rsp
140aa92c5:     	leaq	0x80(%rdx), %rbp
140aa92cc:     	leaq	-0x10(%rbp), %rcx
140aa92d0:     	callq	0x1405dec30 <.text+0x5ddc30>
140aa92d5:     	nop
140aa92d6:     	addq	$0x28, %rsp
140aa92da:     	popq	%rbx
140aa92db:     	popq	%rdi
140aa92dc:     	popq	%rsi
140aa92dd:     	popq	%r12
140aa92df:     	popq	%r13
140aa92e1:     	popq	%r14
140aa92e3:     	popq	%r15
140aa92e5:     	popq	%rbp
140aa92e6:     	retq
140aa92e7:     	nopw	(%rax,%rax)
140aa92f0:     	movq	%rdx, 0x10(%rsp)
140aa92f5:     	pushq	%rbp
140aa92f6:     	pushq	%r15
140aa92f8:     	pushq	%r14
140aa92fa:     	pushq	%r13
140aa92fc:     	pushq	%r12
140aa92fe:     	pushq	%rsi
140aa92ff:     	pushq	%rdi
140aa9300:     	pushq	%rbx
140aa9301:     	subq	$0x28, %rsp
140aa9305:     	leaq	0x80(%rdx), %rbp
140aa930c:     	leaq	0x48(%rbp), %rcx
140aa9310:     	callq	0x1405dec30 <.text+0x5ddc30>
140aa9315:     	nop
140aa9316:     	addq	$0x28, %rsp
140aa931a:     	popq	%rbx
140aa931b:     	popq	%rdi
140aa931c:     	popq	%rsi
140aa931d:     	popq	%r12
140aa931f:     	popq	%r13
140aa9321:     	popq	%r14
140aa9323:     	popq	%r15
140aa9325:     	popq	%rbp
140aa9326:     	retq
140aa9327:     	nopw	(%rax,%rax)
140aa9330:     	movq	%rdx, 0x10(%rsp)
140aa9335:     	pushq	%rbp
140aa9336:     	pushq	%r15
140aa9338:     	pushq	%r14
140aa933a:     	pushq	%r13
140aa933c:     	pushq	%r12
140aa933e:     	pushq	%rsi
140aa933f:     	pushq	%rdi
140aa9340:     	pushq	%rbx
140aa9341:     	subq	$0x28, %rsp
140aa9345:     	leaq	0x80(%rdx), %rbp
140aa934c:     	movq	0xb8(%rbp), %rax
140aa9353:     	movq	%rax, -0x38(%rbp)
140aa9357:     	leaq	-0x38(%rbp), %rcx
140aa935b:     	callq	0x140879c00 <.text+0x878c00>
140aa9360:     	nop
140aa9361:     	addq	$0x28, %rsp
140aa9365:     	popq	%rbx
140aa9366:     	popq	%rdi
140aa9367:     	popq	%rsi
140aa9368:     	popq	%r12
140aa936a:     	popq	%r13
140aa936c:     	popq	%r14
140aa936e:     	popq	%r15
140aa9370:     	popq	%rbp
140aa9371:     	retq
