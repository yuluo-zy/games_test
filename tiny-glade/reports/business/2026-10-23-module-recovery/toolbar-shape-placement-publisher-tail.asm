
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a8bb00:     	movl	0x10(%rcx), %edx
140a8bb03:     	movq	0x30(%rcx), %r11
140a8bb07:     	subq	%rax, %r10
140a8bb0a:     	cmovbq	%rdx, %r10
140a8bb0e:     	movl	$0x8, %r9d
140a8bb14:     	cmovbq	%r9, %rbx
140a8bb18:     	movq	%r8, %rax
140a8bb1b:     	shlq	$0x4, %rax
140a8bb1f:     	addq	0x28(%rcx), %rax
140a8bb23:     	subq	%r8, %r11
140a8bb26:     	cmovbq	%rdx, %r11
140a8bb2a:     	cmovbq	%r9, %rax
140a8bb2e:     	movq	0x40(%rcx), %rcx
140a8bb32:     	subq	%r11, %rcx
140a8bb35:     	subq	%r10, %rcx
140a8bb38:     	movq	%rcx, (%r13)
140a8bb3c:     	shlq	$0x4, %r10
140a8bb40:     	addq	%rbx, %r10
140a8bb43:     	movq	%r10, 0xd30(%rbp)
140a8bb4a:     	shlq	$0x4, %r11
140a8bb4e:     	addq	%rax, %r11
140a8bb51:     	movq	%r11, 0xcf0(%rbp)
140a8bb58:     	pcmpeqd	%xmm6, %xmm6
140a8bb5c:     	jmp	0x140a8bb85 <.text+0xa8ab85>
140a8bb5e:     	nop
140a8bb60:     	movq	0x28(%r14), %rax
140a8bb64:     	movq	%rbx, %rcx
140a8bb67:     	shlq	$0x4, %rcx
140a8bb6b:     	movq	%rsi, (%rax,%rcx)
140a8bb6f:     	movq	%r15, 0x8(%rax,%rcx)
140a8bb74:     	incq	%rbx
140a8bb77:     	movq	%rbx, 0x30(%r14)
140a8bb7b:     	incq	0x40(%r14)
140a8bb7f:     	movq	%r12, %rbx
140a8bb82:     	movq	%rdi, %rax
140a8bb85:     	testq	%rbx, %rbx
140a8bb88:     	je	0x140a8bba0 <.text+0xa8aba0>
140a8bb8a:     	cmpq	0xd30(%rbp), %rbx
140a8bb91:     	je	0x140a8bba0 <.text+0xa8aba0>
140a8bb93:     	leaq	0x10(%rbx), %r12
140a8bb97:     	movq	%rax, %rdi
140a8bb9a:     	jmp	0x140a8bbc0 <.text+0xa8abc0>
140a8bb9c:     	nopl	(%rax)
140a8bba0:     	testq	%rax, %rax
140a8bba3:     	je	0x140a8bccd <.text+0xa8accd>
140a8bba9:     	cmpq	0xcf0(%rbp), %rax
140a8bbb0:     	je	0x140a8bccd <.text+0xa8accd>
140a8bbb6:     	leaq	0x10(%rax), %rdi
140a8bbba:     	xorl	%r12d, %r12d
140a8bbbd:     	movq	%rax, %rbx
140a8bbc0:     	incq	(%r13)
140a8bbc4:     	movq	0x8(%rbx), %r15
140a8bbc8:     	movq	%r15, 0x840(%rbp)
140a8bbcf:     	movq	0xd48(%rbp), %rax
140a8bbd6:     	cmpq	$0x0, 0x18(%rax)
140a8bbdb:     	je	0x140a8bb7f <.text+0xa8ab7f>
140a8bbdd:     	movq	0xcc0(%rbp), %rcx
140a8bbe4:     	leaq	0x840(%rbp), %rdx
140a8bbeb:     	callq	0x1405b0d70 <.text+0x5afd70>
140a8bbf0:     	movq	0xd48(%rbp), %rdx
140a8bbf7:     	movq	(%rdx), %rcx
140a8bbfa:     	movq	0x8(%rdx), %rdx
140a8bbfe:     	movq	%rdx, %r8
140a8bc01:     	andq	%rax, %r8
140a8bc04:     	shrq	$0x39, %rax
140a8bc08:     	movd	%eax, %xmm0
140a8bc0c:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8bc10:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8bc15:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8bc1a:     	leaq	-0x78(%rcx), %rax
140a8bc1e:     	xorl	%r9d, %r9d
140a8bc21:     	movdqu	(%rcx,%r8), %xmm1
140a8bc27:     	movdqa	%xmm1, %xmm2
140a8bc2b:     	pcmpeqb	%xmm0, %xmm2
140a8bc2f:     	pmovmskb	%xmm2, %r10d
140a8bc34:     	testl	%r10d, %r10d
140a8bc37:     	je	0x140a8bc60 <.text+0xa8ac60>
140a8bc39:     	tzcntl	%r10d, %r11d
140a8bc3e:     	addq	%r8, %r11
140a8bc41:     	andq	%rdx, %r11
140a8bc44:     	negq	%r11
140a8bc47:     	imulq	$0x78, %r11, %r11
140a8bc4b:     	cmpq	%r15, (%rax,%r11)
140a8bc4f:     	je	0x140a8bc90 <.text+0xa8ac90>
140a8bc51:     	leal	-0x1(%r10), %r11d
140a8bc55:     	andw	%r10w, %r11w
140a8bc59:     	movl	%r11d, %r10d
140a8bc5c:     	jne	0x140a8bc39 <.text+0xa8ac39>
140a8bc5e:     	nop
140a8bc60:     	pcmpeqb	%xmm6, %xmm1
140a8bc64:     	pmovmskb	%xmm1, %r10d
140a8bc69:     	testl	%r10d, %r10d
140a8bc6c:     	jne	0x140a8bb7f <.text+0xa8ab7f>
140a8bc72:     	addq	%r9, %r8
140a8bc75:     	addq	$0x10, %r8
140a8bc79:     	addq	$0x10, %r9
140a8bc7d:     	andq	%rdx, %r8
140a8bc80:     	jmp	0x140a8bc21 <.text+0xa8ac21>
140a8bc82:     	nopw	%cs:(%rax,%rax)
140a8bc90:     	movq	0x8(%rbx), %r15
140a8bc94:     	movl	0xce0(%rbp), %eax
140a8bc9a:     	movq	0xce8(%rbp), %rcx
140a8bca1:     	movl	%eax, (%rcx)
140a8bca3:     	movq	0x30(%r14), %rbx
140a8bca7:     	movq	0x40(%r14), %rsi
140a8bcab:     	cmpq	0x20(%r14), %rbx
140a8bcaf:     	jne	0x140a8bb60 <.text+0xa8ab60>
140a8bcb5:     	movq	0xd20(%rbp), %rcx
140a8bcbc:     	leaq	0x21ae485(%rip), %rdx   # 0x142c3a148
140a8bcc3:     	callq	0x140562630 <.text+0x561630>
140a8bcc8:     	jmp	0x140a8bb60 <.text+0xa8ab60>
140a8bccd:     	movq	0xd10(%rbp), %rax
140a8bcd4:     	movq	(%rax), %rsi
140a8bcd7:     	movq	0xd00(%rbp), %rax
140a8bcde:     	movq	(%rax), %rax
140a8bce1:     	movq	%rax, 0xce0(%rbp)
140a8bce8:     	movq	0xe98(%rbp), %rdx
140a8bcef:     	movl	0x1c(%rdx), %eax
140a8bcf2:     	movq	0x10(%rdx), %rcx
140a8bcf6:     	movl	%eax, (%rcx)
140a8bcf8:     	movq	(%rdx), %r14
140a8bcfb:     	movq	%rsi, 0x840(%rbp)
140a8bd02:     	movq	0xd48(%rbp), %rax
140a8bd09:     	movq	%rax, 0x848(%rbp)
140a8bd10:     	movq	0xcf8(%rbp), %rax
140a8bd17:     	movq	%rax, 0x850(%rbp)
140a8bd1e:     	leaq	0x840(%rbp), %r15
140a8bd25:     	movq	%r14, %rcx
140a8bd28:     	movq	%r15, %rdx
140a8bd2b:     	callq	0x141458b20 <.text+0x1457b20>
140a8bd30:     	xorl	%ecx, %ecx
140a8bd32:     	callq	0x1400d9b20 <.text+0xd8b20>
140a8bd37:     	testq	%rax, %rax
140a8bd3a:     	je	0x140a8e0cd <.text+0xa8d0cd>
140a8bd40:     	movups	(%rax), %xmm0
140a8bd43:     	incq	(%rax)
140a8bd46:     	movaps	%xmm0, 0x860(%rbp)
140a8bd4d:     	movups	0x21e01ac(%rip), %xmm0  # 0x142c6bf00
140a8bd54:     	movaps	%xmm0, 0x840(%rbp)
140a8bd5b:     	movdqu	0x21e01ad(%rip), %xmm0  # 0x142c6bf10
140a8bd63:     	movdqa	%xmm0, 0x850(%rbp)
140a8bd6b:     	addq	$0x8, %r14
140a8bd6f:     	movq	%rsi, 0x260(%rbp)
140a8bd76:     	movq	0xd48(%rbp), %rsi
140a8bd7d:     	movq	%rsi, 0x268(%rbp)
140a8bd84:     	movq	0xcf8(%rbp), %rbx
140a8bd8b:     	movq	%rbx, 0x270(%rbp)
140a8bd92:     	movq	%r15, 0x278(%rbp)
140a8bd99:     	movb	$0x1, 0xd55(%rbp)
140a8bda0:     	leaq	0x260(%rbp), %rdx
140a8bda7:     	movq	%r14, %rcx
140a8bdaa:     	callq	0x141458a50 <.text+0x1457a50>
140a8bdaf:     	movq	0x840(%rbp), %rdi
140a8bdb6:     	movq	0x848(%rbp), %rcx
140a8bdbd:     	movdqa	(%rdi), %xmm0
140a8bdc1:     	testq	%rcx, %rcx
140a8bdc4:     	movq	%rcx, 0xd30(%rbp)
140a8bdcb:     	je	0x140a8bf1e <.text+0xa8af1e>
140a8bdd1:     	leaq	0x17(,%rcx,8), %rax
140a8bdd9:     	andq	$-0x10, %rax
140a8bddd:     	addq	%rax, %rcx
140a8bde0:     	addq	$0x11, %rcx
140a8bde4:     	movq	%rcx, 0xd20(%rbp)
140a8bdeb:     	movq	%rdi, %rcx
140a8bdee:     	subq	%rax, %rcx
140a8bdf1:     	movq	%rcx, 0xd00(%rbp)
140a8bdf8:     	movl	$0x10, %eax
140a8bdfd:     	movq	%rax, 0xd10(%rbp)
140a8be04:     	jmp	0x140a8bf29 <.text+0xa8af29>
140a8be09:     	movq	%rcx, (%rax)
140a8be0c:     	movq	(%rbx), %rsi
140a8be0f:     	movq	0x18(%rbx), %rdx
140a8be13:     	movdqa	(%rsi), %xmm0
140a8be17:     	pmovmskb	%xmm0, %eax
140a8be1b:     	notl	%eax
140a8be1d:     	leaq	0x10(%rsi), %rbx
140a8be21:     	movl	0x1c(%r15), %ecx
140a8be25:     	movl	%ecx, 0xce0(%rbp)
140a8be2b:     	movq	(%r15), %r14
140a8be2e:     	movq	0x10(%r15), %rcx
140a8be32:     	movq	%rcx, 0xce8(%rbp)
140a8be39:     	leaq	0x20(%r14), %rcx
140a8be3d:     	movq	%rcx, 0xd30(%rbp)
140a8be44:     	jmp	0x140a8be79 <.text+0xa8ae79>
140a8be46:     	nopw	%cs:(%rax,%rax)
140a8be50:     	movq	0x28(%r14), %rax
140a8be54:     	movq	%r15, %rcx
140a8be57:     	shlq	$0x4, %rcx
140a8be5b:     	movq	%rdi, (%rax,%rcx)
140a8be5f:     	movq	%r12, 0x8(%rax,%rcx)
140a8be64:     	incq	%r15
140a8be67:     	movq	%r15, 0x30(%r14)
140a8be6b:     	incq	0x40(%r14)
140a8be6f:     	movl	%r13d, %eax
140a8be72:     	movq	0xcc0(%rbp), %rdx
140a8be79:     	testq	%rdx, %rdx
140a8be7c:     	je	0x140a8b8ad <.text+0xa8a8ad>
140a8be82:     	testw	%ax, %ax
140a8be85:     	je	0x140a8be90 <.text+0xa8ae90>
140a8be87:     	leal	-0x1(%rax), %r13d
140a8be8b:     	andl	%eax, %r13d
140a8be8e:     	jmp	0x140a8beb9 <.text+0xa8aeb9>
140a8be90:     	movdqa	(%rbx), %xmm0
140a8be94:     	pmovmskb	%xmm0, %eax
140a8be98:     	addq	$-0x2a80, %rsi          # imm = 0xD580
140a8be9f:     	addq	$0x10, %rbx
140a8bea3:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a8bea8:     	je	0x140a8be90 <.text+0xa8ae90>
140a8beaa:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140a8beaf:     	subl	%eax, %ecx
140a8beb1:     	notl	%eax
140a8beb3:     	movl	%eax, %r13d
140a8beb6:     	andl	%ecx, %r13d
140a8beb9:     	decq	%rdx
140a8bebc:     	tzcntl	%eax, %eax
140a8bec0:     	negq	%rax
140a8bec3:     	imulq	$0x2a8, %rax, %rcx      # imm = 0x2A8
140a8beca:     	cmpb	$0x1, -0x7(%rsi,%rcx)
140a8becf:     	movl	%r13d, %eax
140a8bed2:     	jbe	0x140a8be79 <.text+0xa8ae79>
140a8bed4:     	movq	%rdx, 0xcc0(%rbp)
140a8bedb:     	addq	%rsi, %rcx
140a8bede:     	movq	-0x2a8(%rcx), %r12
140a8bee5:     	movl	0xce0(%rbp), %eax
140a8beeb:     	movq	0xce8(%rbp), %rcx
140a8bef2:     	movl	%eax, (%rcx)
140a8bef4:     	movq	0x30(%r14), %r15
140a8bef8:     	movq	0x40(%r14), %rdi
140a8befc:     	cmpq	0x20(%r14), %r15
140a8bf00:     	jne	0x140a8be50 <.text+0xa8ae50>
140a8bf06:     	movq	0xd30(%rbp), %rcx
140a8bf0d:     	leaq	0x21ae234(%rip), %rdx   # 0x142c3a148
140a8bf14:     	callq	0x140562630 <.text+0x561630>
140a8bf19:     	jmp	0x140a8be50 <.text+0xa8ae50>
140a8bf1e:     	movq	$0x0, 0xd10(%rbp)
140a8bf29:     	movq	0x858(%rbp), %r14
140a8bf30:     	leaq	0x10(%rdi), %r15
140a8bf34:     	pmovmskb	%xmm0, %eax
140a8bf38:     	notl	%eax
140a8bf3a:     	leaq	0x60(%rsi), %rcx
140a8bf3e:     	movq	%rcx, 0xcf0(%rbp)
140a8bf45:     	leaq	0x20(%rbx), %rcx
140a8bf49:     	movq	%rcx, 0xce8(%rbp)
140a8bf50:     	pcmpeqd	%xmm6, %xmm6
140a8bf54:     	xorps	%xmm7, %xmm7
140a8bf57:     	jmp	0x140a8c150 <.text+0xa8b150>
140a8bf5c:     	nopl	(%rax)
140a8bf60:     	movdqu	(%rsi,%rdx), %xmm1
140a8bf65:     	movdqa	%xmm1, %xmm2
140a8bf69:     	pcmpeqb	%xmm0, %xmm2
140a8bf6d:     	pmovmskb	%xmm2, %r9d
140a8bf72:     	testl	%r9d, %r9d
140a8bf75:     	je	0x140a8bfa0 <.text+0xa8afa0>
140a8bf77:     	tzcntl	%r9d, %r10d
140a8bf7c:     	addq	%rdx, %r10
140a8bf7f:     	andq	%rcx, %r10
140a8bf82:     	negq	%r10
140a8bf85:     	imulq	$0x2a8, %r10, %r12      # imm = 0x2A8
140a8bf8c:     	cmpq	%r13, (%rax,%r12)
140a8bf90:     	je	0x140a8bfd0 <.text+0xa8afd0>
140a8bf92:     	leal	-0x1(%r9), %r10d
140a8bf96:     	andw	%r9w, %r10w
140a8bf9a:     	movl	%r10d, %r9d
140a8bf9d:     	jne	0x140a8bf77 <.text+0xa8af77>
140a8bf9f:     	nop
140a8bfa0:     	pcmpeqb	%xmm6, %xmm1
140a8bfa4:     	pmovmskb	%xmm1, %r9d
140a8bfa9:     	testl	%r9d, %r9d
140a8bfac:     	jne	0x140a8c210 <.text+0xa8b210>
140a8bfb2:     	addq	%r8, %rdx
140a8bfb5:     	addq	$0x10, %rdx
140a8bfb9:     	addq	$0x10, %r8
140a8bfbd:     	andq	%rcx, %rdx
140a8bfc0:     	jmp	0x140a8bf60 <.text+0xa8af60>
140a8bfc2:     	nopw	%cs:(%rax,%rax)
140a8bfd0:     	cmpq	$0x2, -0x290(%rsi,%r12)
140a8bfd9:     	jb	0x140a8df59 <.text+0xa8cf59>
140a8bfdf:     	movss	-0x270(%rsi,%r12), %xmm0
140a8bfe9:     	ucomiss	%xmm7, %xmm0
140a8bfec:     	jbe	0x140a8df59 <.text+0xa8cf59>
140a8bff2:     	movq	%r13, 0x260(%rbp)
140a8bff9:     	movq	0xd48(%rbp), %rax
140a8c000:     	cmpq	$0x0, 0x18(%rax)
140a8c005:     	je	0x140a8c217 <.text+0xa8b217>
140a8c00b:     	movq	0xcc0(%rbp), %rcx
140a8c012:     	leaq	0x260(%rbp), %rdx
140a8c019:     	callq	0x1405b0d70 <.text+0x5afd70>
140a8c01e:     	addq	%r12, %rsi
140a8c021:     	movq	0xd48(%rbp), %rdx
140a8c028:     	movq	(%rdx), %rcx
140a8c02b:     	movq	0x8(%rdx), %rdx
140a8c02f:     	movq	%rdx, %r8
140a8c032:     	andq	%rax, %r8
140a8c035:     	shrq	$0x39, %rax
140a8c039:     	movd	%eax, %xmm0
140a8c03d:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8c041:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8c046:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8c04b:     	leaq	-0x78(%rcx), %rax
140a8c04f:     	xorl	%r9d, %r9d
140a8c052:     	movdqu	(%rcx,%r8), %xmm1
140a8c058:     	movdqa	%xmm1, %xmm2
140a8c05c:     	pcmpeqb	%xmm0, %xmm2
140a8c060:     	pmovmskb	%xmm2, %r10d
140a8c065:     	testl	%r10d, %r10d
140a8c068:     	je	0x140a8c090 <.text+0xa8b090>
140a8c06a:     	tzcntl	%r10d, %r11d
140a8c06f:     	addq	%r8, %r11
140a8c072:     	andq	%rdx, %r11
140a8c075:     	negq	%r11
140a8c078:     	imulq	$0x78, %r11, %r11
140a8c07c:     	cmpq	%r13, (%rax,%r11)
140a8c080:     	je	0x140a8c0c0 <.text+0xa8b0c0>
140a8c082:     	leal	-0x1(%r10), %r11d
140a8c086:     	andw	%r10w, %r11w
140a8c08a:     	movl	%r11d, %r10d
140a8c08d:     	jne	0x140a8c06a <.text+0xa8b06a>
140a8c08f:     	nop
140a8c090:     	pcmpeqb	%xmm6, %xmm1
140a8c094:     	pmovmskb	%xmm1, %r10d
140a8c099:     	testl	%r10d, %r10d
140a8c09c:     	jne	0x140a8c217 <.text+0xa8b217>
140a8c0a2:     	addq	%r9, %r8
140a8c0a5:     	addq	$0x10, %r8
140a8c0a9:     	addq	$0x10, %r9
140a8c0ad:     	andq	%rdx, %r8
140a8c0b0:     	jmp	0x140a8c052 <.text+0xa8b052>
140a8c0b2:     	nopw	%cs:(%rax,%rax)
140a8c0c0:     	movss	-0x18(%rcx,%r11), %xmm8
140a8c0c7:     	movq	-0x260(%rsi), %r12
140a8c0ce:     	movq	-0x258(%rsi), %rax
140a8c0d5:     	leaq	(%rax,%rax,2), %rax
140a8c0d9:     	leaq	(%r12,%rax,4), %rsi
140a8c0dd:     	jmp	0x140a8c0e5 <.text+0xa8b0e5>
140a8c0df:     	nop
140a8c0e0:     	ucomiss	%xmm1, %xmm0
140a8c0e3:     	ja	0x140a8c132 <.text+0xa8b132>
140a8c0e5:     	cmpq	%rsi, %r12
140a8c0e8:     	je	0x140a8c12e <.text+0xa8b12e>
140a8c0ea:     	movd	(%r12), %xmm1
140a8c0f0:     	movd	0x8(%r12), %xmm2
140a8c0f7:     	movq	0xce0(%rbp), %rcx
140a8c0fe:     	callq	0x1409e9640 <.text+0x9e8640>
140a8c103:     	movss	0x4(%r12), %xmm1
140a8c10a:     	addq	$0xc, %r12
140a8c10e:     	ucomiss	%xmm8, %xmm1
140a8c112:     	jbe	0x140a8c0e0 <.text+0xa8b0e0>
140a8c114:     	ucomiss	%xmm1, %xmm0
140a8c117:     	jne	0x140a8c11b <.text+0xa8b11b>
140a8c119:     	jnp	0x140a8c0e5 <.text+0xa8b0e5>
140a8c11b:     	movq	0xcf0(%rbp), %rcx
140a8c122:     	movq	%r13, %rdx
140a8c125:     	callq	0x1402ca2f0 <.text+0x2c92f0>
140a8c12a:     	movl	%ebx, %eax
140a8c12c:     	jmp	0x140a8c150 <.text+0xa8b150>
140a8c12e:     	movl	%ebx, %eax
140a8c130:     	jmp	0x140a8c150 <.text+0xa8b150>
140a8c132:     	movq	0xcf0(%rbp), %rcx
140a8c139:     	movq	%r13, %rdx
140a8c13c:     	callq	0x1402ca2f0 <.text+0x2c92f0>
140a8c141:     	movl	%ebx, %eax
140a8c143:     	nopw	%cs:(%rax,%rax)
140a8c150:     	testq	%r14, %r14
140a8c153:     	je	0x140a8c228 <.text+0xa8b228>
140a8c159:     	testw	%ax, %ax
140a8c15c:     	je	0x140a8c170 <.text+0xa8b170>
140a8c15e:     	leal	-0x1(%rax), %ebx
140a8c161:     	andl	%eax, %ebx
140a8c163:     	jmp	0x140a8c195 <.text+0xa8b195>
140a8c165:     	nopw	%cs:(%rax,%rax)
140a8c170:     	movdqa	(%r15), %xmm0
140a8c175:     	pmovmskb	%xmm0, %eax
140a8c179:     	addq	$-0x80, %rdi
140a8c17d:     	addq	$0x10, %r15
140a8c181:     	cmpl	$0xffff, %eax           # imm = 0xFFFF
140a8c186:     	je	0x140a8c170 <.text+0xa8b170>
140a8c188:     	movl	$0xfffffffe, %ecx       # imm = 0xFFFFFFFE
140a8c18d:     	subl	%eax, %ecx
140a8c18f:     	notl	%eax
140a8c191:     	movl	%eax, %ebx
140a8c193:     	andl	%ecx, %ebx
140a8c195:     	decq	%r14
140a8c198:     	tzcntl	%eax, %eax
140a8c19c:     	shll	$0x3, %eax
140a8c19f:     	movq	%rdi, %rcx
140a8c1a2:     	subq	%rax, %rcx
140a8c1a5:     	movq	-0x8(%rcx), %r13
140a8c1a9:     	movq	%r13, 0x260(%rbp)
140a8c1b0:     	movq	0xcf8(%rbp), %rax
140a8c1b7:     	cmpq	$0x0, 0x18(%rax)
140a8c1bc:     	je	0x140a8c210 <.text+0xa8b210>
140a8c1be:     	movq	0xce8(%rbp), %rcx
140a8c1c5:     	leaq	0x260(%rbp), %rdx
140a8c1cc:     	callq	0x1405b0d70 <.text+0x5afd70>
140a8c1d1:     	movq	0xcf8(%rbp), %rcx
140a8c1d8:     	movq	(%rcx), %rsi
140a8c1db:     	movq	0x8(%rcx), %rcx
140a8c1df:     	movq	%rcx, %rdx
140a8c1e2:     	andq	%rax, %rdx
140a8c1e5:     	shrq	$0x39, %rax
140a8c1e9:     	movd	%eax, %xmm0
140a8c1ed:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8c1f1:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8c1f6:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8c1fb:     	leaq	-0x2a8(%rsi), %rax
140a8c202:     	xorl	%r8d, %r8d
140a8c205:     	jmp	0x140a8bf60 <.text+0xa8af60>
140a8c20a:     	nopw	(%rax,%rax)
140a8c210:     	movl	%ebx, %eax
140a8c212:     	jmp	0x140a8c150 <.text+0xa8b150>
140a8c217:     	leaq	0x204d102(%rip), %rcx   # 0x142ad9320
140a8c21e:     	callq	0x1428d9310 <.text+0x28d8310>
140a8c223:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8c228:     	cmpq	$0x0, 0xd30(%rbp)
140a8c230:     	movq	0xd48(%rbp), %rsi
140a8c237:     	movq	0xd20(%rbp), %rdx
140a8c23e:     	movq	0xd00(%rbp), %rcx
140a8c245:     	je	0x140a8c258 <.text+0xa8b258>
140a8c247:     	testq	%rdx, %rdx
140a8c24a:     	je	0x140a8c258 <.text+0xa8b258>
140a8c24c:     	movq	0xd10(%rbp), %r8
140a8c253:     	callq	0x140613c20 <.text+0x612c20>
140a8c258:     	leaq	0x90(%rsi), %rax
140a8c25f:     	movq	%rax, 0xb80(%rbp)
140a8c266:     	movq	%rax, 0x840(%rbp)
140a8c26d:     	leaq	0x840(%rbp), %rdx
140a8c274:     	movq	0xcf0(%rbp), %rcx
140a8c27b:     	callq	0x14152d580 <.text+0x152c580>
140a8c280:     	movq	0xd48(%rbp), %rax
140a8c287:     	movq	0x90(%rax), %rsi
140a8c28e:     	movq	0xa8(%rax), %rdi
140a8c295:     	movdqa	(%rsi), %xmm0
140a8c299:     	pmovmskb	%xmm0, %r12d
140a8c29e:     	notl	%r12d
140a8c2a1:     	leaq	0x10(%rsi), %rbx
140a8c2a5:     	movq	0xe70(%rbp), %rax
140a8c2ac:     	movl	0x1c(%rax), %ecx
140a8c2af:     	movl	%ecx, 0xd20(%rbp)
140a8c2b5:     	movq	(%rax), %rcx
140a8c2b8:     	movq	0x10(%rax), %rax
140a8c2bc:     	movq	%rax, 0xd10(%rbp)
140a8c2c3:     	movq	%rcx, 0xd30(%rbp)
140a8c2ca:     	leaq	0x20(%rcx), %rax
140a8c2ce:     	movq	%rax, 0xd00(%rbp)
140a8c2d5:     	jmp	0x140a8c306 <.text+0xa8b306>
140a8c2d7:     	nopw	(%rax,%rax)
140a8c2e0:     	movq	0xd30(%rbp), %rdx
140a8c2e7:     	movq	0x28(%rdx), %rax
140a8c2eb:     	movq	%r15, %rcx
140a8c2ee:     	shlq	$0x4, %rcx
140a8c2f2:     	movq	%r13, (%rax,%rcx)
140a8c2f6:     	movq	%r14, 0x8(%rax,%rcx)
140a8c2fb:     	incq	%r15
140a8c2fe:     	movq	%r15, 0x30(%rdx)
140a8c302:     	incq	0x40(%rdx)
140a8c306:     	testq	%rdi, %rdi
140a8c309:     	je	0x140a8c41d <.text+0xa8b41d>
140a8c30f:     	testw	%r12w, %r12w
140a8c313:     	jne	0x140a8c33d <.text+0xa8b33d>
140a8c315:     	nopw	%cs:(%rax,%rax)
140a8c320:     	movdqa	(%rbx), %xmm0
140a8c324:     	pmovmskb	%xmm0, %r12d
140a8c329:     	addq	$-0x80, %rsi
140a8c32d:     	addq	$0x10, %rbx
140a8c331:     	cmpl	$0xffff, %r12d          # imm = 0xFFFF
140a8c338:     	je	0x140a8c320 <.text+0xa8b320>
140a8c33a:     	notl	%r12d
140a8c33d:     	tzcntl	%r12d, %eax
140a8c342:     	shll	$0x3, %eax
140a8c345:     	movq	%rsi, %rcx
140a8c348:     	subq	%rax, %rcx
140a8c34b:     	movq	-0x8(%rcx), %rax
140a8c34f:     	movq	%rax, 0xc70(%rbp)
140a8c356:     	movq	0xce8(%rbp), %rcx
140a8c35d:     	leaq	0xc70(%rbp), %rdx
140a8c364:     	callq	0x1405b0d70 <.text+0x5afd70>
140a8c369:     	leaq	0x840(%rbp), %rcx
140a8c370:     	movq	0xcf8(%rbp), %rdx
140a8c377:     	movq	%rax, %r8
140a8c37a:     	leaq	0xc70(%rbp), %r9
140a8c381:     	callq	0x140b772b0 <.text+0xb762b0>
140a8c386:     	leal	-0x1(%r12), %eax
140a8c38b:     	andl	%r12d, %eax
140a8c38e:     	movl	%eax, %r12d
140a8c391:     	decq	%rdi
140a8c394:     	xorl	%r14d, %r14d
140a8c397:     	cmpq	0x848(%rbp), %r14
140a8c39e:     	jo	0x140a8c306 <.text+0xa8b306>
140a8c3a4:     	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8c3aa:     	leaq	0x260(%rbp), %rcx
140a8c3b1:     	leaq	0x848(%rbp), %rdx
140a8c3b8:     	callq	0x142923c00 <.text+0x2922c00>
140a8c3bd:     	cmpq	0x260(%rbp), %r14
140a8c3c4:     	jo	0x140a8c306 <.text+0xa8b306>
140a8c3ca:     	leaq	0x260(%rbp), %rcx
140a8c3d1:     	callq	0x140895600 <.text+0x894600>
140a8c3d6:     	movq	0xc70(%rbp), %r14
140a8c3dd:     	movl	0xd20(%rbp), %eax
140a8c3e3:     	movq	0xd10(%rbp), %rcx
140a8c3ea:     	movl	%eax, (%rcx)
140a8c3ec:     	movq	0xd30(%rbp), %rax
140a8c3f3:     	movq	0x30(%rax), %r15
140a8c3f7:     	movq	0x40(%rax), %r13
140a8c3fb:     	cmpq	0x20(%rax), %r15
140a8c3ff:     	jne	0x140a8c2e0 <.text+0xa8b2e0>
140a8c405:     	movq	0xd00(%rbp), %rcx
140a8c40c:     	leaq	0x21add35(%rip), %rdx   # 0x142c3a148
140a8c413:     	callq	0x140562630 <.text+0x561630>
140a8c418:     	jmp	0x140a8c2e0 <.text+0xa8b2e0>
140a8c41d:     	movq	0xd48(%rbp), %rax
140a8c424:     	movq	0x30(%rax), %r8
140a8c428:     	movq	0x48(%rax), %rcx
140a8c42c:     	movq	%rcx, 0xd30(%rbp)
140a8c433:     	movdqa	(%r8), %xmm0
140a8c438:     	pmovmskb	%xmm0, %ecx
140a8c43c:     	notl	%ecx
140a8c43e:     	movq	%rcx, 0xd00(%rbp)
140a8c445:     	leaq	0x10(%r8), %r13
140a8c449:     	movq	0x60(%rax), %r12
140a8c44d:     	movdqa	(%r12), %xmm0
140a8c453:     	pmovmskb	%xmm0, %r15d
140a8c458:     	notl	%r15d
140a8c45b:     	movq	0xea0(%rbp), %rcx
140a8c462:     	movq	(%rcx), %rcx
140a8c465:     	movq	%rcx, 0xc18(%rbp)
140a8c46c:     	movq	0xeb8(%rbp), %rcx
140a8c473:     	movq	(%rcx), %rcx
140a8c476:     	movq	%rcx, 0xb60(%rbp)
140a8c47d:     	movq	0xeb0(%rbp), %rcx
140a8c484:     	movq	(%rcx), %rcx
140a8c487:     	movq	%rcx, 0xb68(%rbp)
140a8c48e:     	movq	0xea8(%rbp), %rcx
140a8c495:     	movq	(%rcx), %rcx
140a8c498:     	movq	%rcx, 0xb78(%rbp)
140a8c49f:     	movq	0xe78(%rbp), %rdx
140a8c4a6:     	movl	0x1c(%rdx), %ecx
140a8c4a9:     	movl	%ecx, 0xc60(%rbp)
140a8c4af:     	movq	(%rdx), %rcx
140a8c4b2:     	movq	0x10(%rdx), %rdx
140a8c4b6:     	movq	%rdx, 0xb58(%rbp)
140a8c4bd:     	pcmpeqd	%xmm7, %xmm7
140a8c4c1:     	xorps	%xmm8, %xmm8
140a8c4c5:     	movss	0x1e993a3(%rip), %xmm6  # 0x142925870
140a8c4cd:     	movss	0x1ea812a(%rip), %xmm10 # 0x142934600
140a8c4d6:     	movss	0x1ea028d(%rip), %xmm11 # 0x14292c76c
140a8c4df:     	movss	0x1e993f0(%rip), %xmm15 # 0x1429258d8
140a8c4e8:     	movss	0x1f40b73(%rip), %xmm14 # 0x1429cd064
140a8c4f1:     	leaq	0x10(%r12), %r14
140a8c4f6:     	movq	0x78(%rax), %rax
140a8c4fa:     	movq	%rax, 0xd10(%rbp)
140a8c501:     	movq	%rcx, 0xc10(%rbp)
140a8c508:     	leaq	0x20(%rcx), %rax
140a8c50c:     	movq	%rax, 0xb70(%rbp)
140a8c513:     	testq	%r8, %r8
140a8c516:     	je	0x140a8c598 <.text+0xa8b598>
140a8c51c:     	cmpq	$0x0, 0xd30(%rbp)
140a8c524:     	je	0x140a8c58d <.text+0xa8b58d>
140a8c526:     	movq	0xd00(%rbp), %rcx
140a8c52d:     	testw	%cx, %cx
140a8c530:     	je	0x140a8c540 <.text+0xa8b540>
140a8c532:     	leal	-0x1(%rcx), %edx
140a8c535:     	andl	%ecx, %edx
140a8c537:     	jmp	0x140a8c567 <.text+0xa8b567>
140a8c539:     	nopl	(%rax)
140a8c540:     	movdqa	(%r13), %xmm0
140a8c546:     	pmovmskb	%xmm0, %ecx
140a8c54a:     	addq	$-0x80, %r8
140a8c54e:     	addq	$0x10, %r13
140a8c552:     	cmpl	$0xffff, %ecx           # imm = 0xFFFF
140a8c558:     	je	0x140a8c540 <.text+0xa8b540>
140a8c55a:     	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140a8c55f:     	subl	%ecx, %eax
140a8c561:     	notl	%ecx
140a8c563:     	movl	%ecx, %edx
140a8c565:     	andl	%eax, %edx
140a8c567:     	decq	0xd30(%rbp)
140a8c56e:     	movb	$0x1, %al
140a8c570:     	movq	%rax, 0xd38(%rbp)
140a8c577:     	movq	%r8, %rax
140a8c57a:     	movq	%r8, 0xd20(%rbp)
140a8c581:     	movq	%rdx, 0xd00(%rbp)
140a8c588:     	jmp	0x140a8c621 <.text+0xa8b621>
140a8c58d:     	movq	$0x0, 0xd30(%rbp)
140a8c598:     	movq	0xd48(%rbp), %rdi
140a8c59f:     	testq	%r12, %r12
140a8c5a2:     	je	0x140a8ddbb <.text+0xa8cdbb>
140a8c5a8:     	cmpq	$0x0, 0xd10(%rbp)
140a8c5b0:     	je	0x140a8ddbb <.text+0xa8cdbb>
140a8c5b6:     	testw	%r15w, %r15w
140a8c5ba:     	je	0x140a8c5d0 <.text+0xa8b5d0>
140a8c5bc:     	leal	-0x1(%r15), %edx
140a8c5c0:     	andl	%r15d, %edx
140a8c5c3:     	jmp	0x140a8c5fb <.text+0xa8b5fb>
140a8c5c5:     	nopw	%cs:(%rax,%rax)
140a8c5d0:     	movdqa	(%r14), %xmm0
140a8c5d5:     	pmovmskb	%xmm0, %r15d
140a8c5da:     	addq	$-0x80, %r12
140a8c5de:     	addq	$0x10, %r14
140a8c5e2:     	cmpl	$0xffff, %r15d          # imm = 0xFFFF
140a8c5e9:     	je	0x140a8c5d0 <.text+0xa8b5d0>
140a8c5eb:     	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
140a8c5f0:     	subl	%r15d, %eax
140a8c5f3:     	notl	%r15d
140a8c5f6:     	movl	%r15d, %edx
140a8c5f9:     	andl	%eax, %edx
140a8c5fb:     	decq	0xd10(%rbp)
140a8c602:     	movq	$0x0, 0xd20(%rbp)
140a8c60d:     	movl	%r15d, %ecx
140a8c610:     	movq	%r12, %rax
140a8c613:     	movl	%edx, %r15d
140a8c616:     	movq	$0x0, 0xd38(%rbp)
140a8c621:     	tzcntl	%ecx, %ecx
140a8c625:     	shll	$0x3, %ecx
140a8c628:     	subq	%rcx, %rax
140a8c62b:     	movq	-0x8(%rax), %rsi
140a8c62f:     	movq	%rsi, 0xcb8(%rbp)
140a8c636:     	movq	%rsi, 0x840(%rbp)
140a8c63d:     	movq	0xd48(%rbp), %rax
140a8c644:     	cmpq	$0x0, 0x18(%rax)
140a8c649:     	je	0x140a8dd86 <.text+0xa8cd86>
140a8c64f:     	movq	0xcc0(%rbp), %rcx
140a8c656:     	leaq	0x840(%rbp), %rdx
140a8c65d:     	callq	0x1405b0d70 <.text+0x5afd70>
140a8c662:     	movq	0xd48(%rbp), %rcx
140a8c669:     	movq	(%rcx), %rbx
140a8c66c:     	movq	0x8(%rcx), %rcx
140a8c670:     	movq	%rcx, %rdx
140a8c673:     	andq	%rax, %rdx
140a8c676:     	shrq	$0x39, %rax
140a8c67a:     	movd	%eax, %xmm0
140a8c67e:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a8c682:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a8c687:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140a8c68c:     	leaq	-0x78(%rbx), %rax
140a8c690:     	xorl	%r8d, %r8d
140a8c693:     	movdqu	(%rbx,%rdx), %xmm1
140a8c698:     	movdqa	%xmm1, %xmm2
140a8c69c:     	pcmpeqb	%xmm0, %xmm2
140a8c6a0:     	pmovmskb	%xmm2, %r9d
140a8c6a5:     	testl	%r9d, %r9d
140a8c6a8:     	je	0x140a8c6cf <.text+0xa8b6cf>
140a8c6aa:     	tzcntl	%r9d, %r10d
140a8c6af:     	addq	%rdx, %r10
140a8c6b2:     	andq	%rcx, %r10
140a8c6b5:     	negq	%r10
140a8c6b8:     	imulq	$0x78, %r10, %rdi
140a8c6bc:     	cmpq	%rsi, (%rax,%rdi)
140a8c6c0:     	je	0x140a8c700 <.text+0xa8b700>
140a8c6c2:     	leal	-0x1(%r9), %r10d
140a8c6c6:     	andw	%r9w, %r10w
140a8c6ca:     	movl	%r10d, %r9d
140a8c6cd:     	jne	0x140a8c6aa <.text+0xa8b6aa>
140a8c6cf:     	pcmpeqb	%xmm7, %xmm1
140a8c6d3:     	pmovmskb	%xmm1, %r9d
140a8c6d8:     	testl	%r9d, %r9d
140a8c6db:     	jne	0x140a8dd86 <.text+0xa8cd86>
140a8c6e1:     	addq	%r8, %rdx
140a8c6e4:     	addq	$0x10, %rdx
140a8c6e8:     	addq	$0x10, %r8
140a8c6ec:     	andq	%rcx, %rdx
140a8c6ef:     	jmp	0x140a8c693 <.text+0xa8b693>
140a8c6f1:     	nopw	%cs:(%rax,%rax)
140a8c700:     	cmpq	$0x2, -0x60(%rbx,%rdi)
140a8c706:     	movq	0xd20(%rbp), %r8
140a8c70d:     	jb	0x140a8c513 <.text+0xa8b513>
140a8c713:     	movss	-0x40(%rbx,%rdi), %xmm0
140a8c719:     	ucomiss	%xmm8, %xmm0
140a8c71d:     	jbe	0x140a8c513 <.text+0xa8b513>
140a8c723:     	movq	0xcb8(%rbp), %rax
140a8c72a:     	movq	%rax, 0xd40(%rbp)
140a8c731:     	movl	0xc60(%rbp), %eax
140a8c737:     	movq	0xb58(%rbp), %rcx
140a8c73e:     	movl	%eax, (%rcx)
140a8c740:     	movq	0xc10(%rbp), %rax
140a8c747:     	movq	0x30(%rax), %rsi
140a8c74b:     	movq	0x40(%rax), %rcx
140a8c74f:     	movq	%rcx, 0xd18(%rbp)
140a8c756:     	cmpq	0x20(%rax), %rsi
140a8c75a:     	jne	0x140a8c76f <.text+0xa8b76f>
140a8c75c:     	movq	0xb70(%rbp), %rcx
140a8c763:     	leaq	0x21ad9de(%rip), %rdx   # 0x142c3a148
140a8c76a:     	callq	0x140562630 <.text+0x561630>
140a8c76f:     	movq	0xc10(%rbp), %rdx
140a8c776:     	movq	0x28(%rdx), %rax
140a8c77a:     	movq	%rsi, %rcx
140a8c77d:     	shlq	$0x4, %rcx
140a8c781:     	movq	0xd18(%rbp), %r8
140a8c788:     	movq	%r8, (%rax,%rcx)
140a8c78c:     	movq	0xd40(%rbp), %r8
140a8c793:     	movq	%r8, 0x8(%rax,%rcx)
140a8c798:     	incq	%rsi
140a8c79b:     	movq	%rsi, 0x30(%rdx)
140a8c79f:     	incq	0x40(%rdx)
140a8c7a3:     	movq	0xce8(%rbp), %rcx
140a8c7aa:     	leaq	0xcb8(%rbp), %rdx
140a8c7b1:     	callq	0x1405b0d70 <.text+0x5afd70>
140a8c7b6:     	leaq	0x840(%rbp), %rcx
140a8c7bd:     	movq	0xcf8(%rbp), %rdx
140a8c7c4:     	movq	%rax, %r8
140a8c7c7:     	leaq	0xcb8(%rbp), %r9
140a8c7ce:     	callq	0x140b772b0 <.text+0xb762b0>
140a8c7d3:     	xorl	%eax, %eax
140a8c7d5:     	cmpq	0x848(%rbp), %rax
140a8c7dc:     	jno	0x140a8c7fc <.text+0xa8b7fc>
140a8c7de:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a8c7e8:     	movq	%rax, -0x40(%rbp)
140a8c7ec:     	movb	$0x1, %r10b
140a8c7ef:     	movq	$0x0, 0xcc8(%rbp)
140a8c7fa:     	jmp	0x140a8c82b <.text+0xa8b82b>
140a8c7fc:     	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8c802:     	leaq	-0x40(%rbp), %rsi
140a8c806:     	movq	%rsi, %rcx
140a8c809:     	leaq	0x848(%rbp), %rdx
140a8c810:     	callq	0x142923c00 <.text+0x2922c00>
140a8c815:     	xorl	%eax, %eax
140a8c817:     	cmpq	-0x40(%rbp), %rax
140a8c81b:     	jno	0x140a8c838 <.text+0xa8b838>
140a8c81d:     	movq	$0x0, 0xcc8(%rbp)
140a8c828:     	movb	$0x1, %r10b
140a8c82b:     	movb	$0x1, %al
140a8c82d:     	movl	%eax, 0xd28(%rbp)
140a8c833:     	jmp	0x140a8cad6 <.text+0xa8bad6>
140a8c838:     	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8c83e:     	leaq	0x840(%rbp), %rcx
140a8c845:     	movq	%rsi, %rdx
140a8c848:     	callq	0x142923c00 <.text+0x2922c00>
140a8c84d:     	cmpb	$0x0, 0xd38(%rbp)
140a8c854:     	jne	0x140a8e0b0 <.text+0xa8d0b0>
140a8c85a:     	movzbl	0x1b0(%rbp), %eax
140a8c861:     	movb	%al, 0xd2f(%rbp)
140a8c867:     	leaq	0x1b1(%rbp), %rdx
140a8c86e:     	movl	(%rdx), %eax
140a8c870:     	movl	0x3(%rdx), %ecx
140a8c873:     	movl	%ecx, 0xc93(%rbp)
140a8c879:     	movl	%eax, 0xc90(%rbp)
140a8c87f:     	movq	0x1b8(%rbp), %rax
140a8c886:     	movq	%rax, 0xc30(%rbp)
140a8c88d:     	movq	0x1c0(%rbp), %rax
140a8c894:     	movq	%rax, 0xc38(%rbp)
140a8c89b:     	movups	0x17(%rdx), %xmm0
140a8c89f:     	movaps	%xmm0, 0x7b0(%rbp)
140a8c8a6:     	movq	0x57(%rdx), %rax
140a8c8aa:     	movq	%rax, 0x830(%rbp)
140a8c8b1:     	movups	0x27(%rdx), %xmm0
140a8c8b5:     	movdqu	0x37(%rdx), %xmm1
140a8c8ba:     	movdqu	0x47(%rdx), %xmm2
140a8c8bf:     	movdqa	%xmm2, 0x820(%rbp)
140a8c8c7:     	movdqa	%xmm1, 0x810(%rbp)
140a8c8cf:     	movaps	%xmm0, 0x800(%rbp)
140a8c8d6:     	movl	0x210(%rbp), %eax
140a8c8dc:     	movl	%eax, 0xc6c(%rbp)
140a8c8e2:     	movl	0x214(%rbp), %eax
140a8c8e8:     	movl	%eax, 0xc64(%rbp)
140a8c8ee:     	movq	0x840(%rbp), %rdx
140a8c8f5:     	testq	%rdx, %rdx
140a8c8f8:     	je	0x140a8c910 <.text+0xa8b910>
140a8c8fa:     	movq	0x848(%rbp), %rcx
140a8c901:     	shlq	$0x3, %rdx
140a8c905:     	movl	$0x4, %r8d
140a8c90b:     	callq	0x140613c20 <.text+0x612c20>
140a8c910:     	movq	0x858(%rbp), %rdx
140a8c917:     	testq	%rdx, %rdx
140a8c91a:     	je	0x140a8c932 <.text+0xa8b932>
140a8c91c:     	movq	0x860(%rbp), %rcx
140a8c923:     	shlq	$0x2, %rdx
140a8c927:     	movl	$0x4, %r8d
140a8c92d:     	callq	0x140613c20 <.text+0x612c20>
140a8c932:     	movq	0x878(%rbp), %rax
140a8c939:     	testq	%rax, %rax
140a8c93c:     	je	0x140a8c958 <.text+0xa8b958>
140a8c93e:     	movq	0x880(%rbp), %rcx
140a8c945:     	shlq	$0x2, %rax
140a8c949:     	leaq	(%rax,%rax,2), %rdx
140a8c94d:     	movl	$0x4, %r8d
140a8c953:     	callq	0x140613c20 <.text+0x612c20>
140a8c958:     	movq	0x890(%rbp), %rdx
140a8c95f:     	testq	%rdx, %rdx
140a8c962:     	je	0x140a8c97a <.text+0xa8b97a>
140a8c964:     	movq	0x898(%rbp), %rcx
140a8c96b:     	shlq	$0x2, %rdx
140a8c96f:     	movl	$0x4, %r8d
140a8c975:     	callq	0x140613c20 <.text+0x612c20>
140a8c97a:     	movq	0x8b0(%rbp), %rdx
140a8c981:     	testq	%rdx, %rdx
140a8c984:     	je	0x140a8c99c <.text+0xa8b99c>
140a8c986:     	movq	0x8b8(%rbp), %rcx
140a8c98d:     	shlq	$0x2, %rdx
140a8c991:     	movl	$0x4, %r8d
140a8c997:     	callq	0x140613c20 <.text+0x612c20>
140a8c99c:     	movq	0x8c8(%rbp), %rax
140a8c9a3:     	testq	%rax, %rax
140a8c9a6:     	je	0x140a8c9c2 <.text+0xa8b9c2>
140a8c9a8:     	movq	0x8d0(%rbp), %rcx
140a8c9af:     	shlq	$0x2, %rax
140a8c9b3:     	leaq	(%rax,%rax,2), %rdx
140a8c9b7:     	movl	$0x4, %r8d
140a8c9bd:     	callq	0x140613c20 <.text+0x612c20>
140a8c9c2:     	movq	0x8e0(%rbp), %rdx
140a8c9c9:     	testq	%rdx, %rdx
140a8c9cc:     	je	0x140a8c9e4 <.text+0xa8b9e4>
140a8c9ce:     	movq	0x8e8(%rbp), %rcx
140a8c9d5:     	shlq	$0x2, %rdx
140a8c9d9:     	movl	$0x4, %r8d
140a8c9df:     	callq	0x140613c20 <.text+0x612c20>
140a8c9e4:     	movq	0x900(%rbp), %rdx
140a8c9eb:     	testq	%rdx, %rdx
140a8c9ee:     	je	0x140a8ca06 <.text+0xa8ba06>
140a8c9f0:     	movq	0x908(%rbp), %rcx
140a8c9f7:     	shlq	$0x2, %rdx
140a8c9fb:     	movl	$0x4, %r8d
140a8ca01:     	callq	0x140613c20 <.text+0x612c20>
140a8ca06:     	leaq	0x958(%rbp), %rcx
140a8ca0d:     	callq	0x14087f390 <.text+0x87e390>
140a8ca12:     	leaq	0x968(%rbp), %rcx
140a8ca19:     	callq	0x14087f390 <.text+0x87e390>
140a8ca1e:     	leaq	0x978(%rbp), %rcx
140a8ca25:     	callq	0x14087f390 <.text+0x87e390>
140a8ca2a:     	leaq	0x988(%rbp), %rcx
140a8ca31:     	callq	0x14087f390 <.text+0x87e390>
140a8ca36:     	leaq	0x998(%rbp), %rcx
140a8ca3d:     	callq	0x14087f390 <.text+0x87e390>
140a8ca42:     	leaq	0x9a8(%rbp), %rcx
140a8ca49:     	callq	0x14087f390 <.text+0x87e390>
140a8ca4e:     	leaq	0x9b8(%rbp), %rcx
140a8ca55:     	callq	0x14087f390 <.text+0x87e390>
140a8ca5a:     	movq	0x918(%rbp), %rdx
140a8ca61:     	testq	%rdx, %rdx
140a8ca64:     	je	0x140a8ca7c <.text+0xa8ba7c>
140a8ca66:     	movq	0x920(%rbp), %rcx
140a8ca6d:     	shlq	$0x2, %rdx
140a8ca71:     	movl	$0x4, %r8d
140a8ca77:     	callq	0x140613c20 <.text+0x612c20>
140a8ca7c:     	movq	0x930(%rbp), %rdx
140a8ca83:     	testq	%rdx, %rdx
140a8ca86:     	je	0x140a8ca9e <.text+0xa8ba9e>
140a8ca88:     	movq	0x938(%rbp), %rcx
140a8ca8f:     	shlq	$0x2, %rdx
140a8ca93:     	movl	$0x4, %r8d
140a8ca99:     	callq	0x140613c20 <.text+0x612c20>
140a8ca9e:     	movq	0x9d0(%rbp), %rdx
140a8caa5:     	testq	%rdx, %rdx
140a8caa8:     	je	0x140a8cac0 <.text+0xa8bac0>
140a8caaa:     	movq	0x9d8(%rbp), %rcx
140a8cab1:     	shlq	$0x3, %rdx
140a8cab5:     	movl	$0x4, %r8d
140a8cabb:     	callq	0x140613c20 <.text+0x612c20>
140a8cac0:     	movb	$0x1, %al
140a8cac2:     	movq	%rax, 0xcc8(%rbp)
140a8cac9:     	xorl	%r10d, %r10d
140a8cacc:     	movl	$0x0, 0xd28(%rbp)
140a8cad6:     	leaq	(%rbx,%rdi), %rax
140a8cada:     	movq	%rax, 0xd40(%rbp)
140a8cae1:     	leaq	(%rbx,%rdi), %r11
140a8cae5:     	addq	$-0x70, %r11
140a8cae9:     	movl	0xc90(%rbp), %eax
140a8caef:     	movl	0xc93(%rbp), %ecx
140a8caf5:     	movl	%eax, 0xc88(%rbp)
140a8cafb:     	movl	%ecx, 0xc8b(%rbp)
140a8cb01:     	movaps	0x7b0(%rbp), %xmm0
140a8cb08:     	movaps	%xmm0, 0x7a0(%rbp)
140a8cb0f:     	movq	0x830(%rbp), %rax
140a8cb16:     	movq	%rax, 0x7f0(%rbp)
140a8cb1d:     	movaps	0x800(%rbp), %xmm0
140a8cb24:     	movaps	0x810(%rbp), %xmm1
140a8cb2b:     	movaps	0x820(%rbp), %xmm2
140a8cb32:     	movaps	%xmm2, 0x7e0(%rbp)
140a8cb39:     	movaps	%xmm1, 0x7d0(%rbp)
140a8cb40:     	movaps	%xmm0, 0x7c0(%rbp)
140a8cb47:     	movq	0xcb8(%rbp), %rax
140a8cb4e:     	movq	0xb60(%rbp), %rdx
140a8cb55:     	movq	0x8(%rdx), %rcx
140a8cb59:     	imulq	$0x98, 0x10(%rdx), %rdx
140a8cb61:     	movq	%rdx, %r9
140a8cb64:     	testq	%rdx, %rdx
140a8cb67:     	je	0x140a8cb80 <.text+0xa8bb80>
140a8cb69:     	leaq	0x98(%rcx), %r8
140a8cb70:     	leaq	-0x98(%r9), %rdx
140a8cb77:     	cmpq	%rax, 0x70(%rcx)
140a8cb7b:     	movq	%r8, %rcx
140a8cb7e:     	jne	0x140a8cb61 <.text+0xa8bb61>
140a8cb80:     	movq	%r9, 0xb88(%rbp)
140a8cb87:     	movq	0xd40(%rbp), %rax
140a8cb8e:     	movq	-0x60(%rax), %rdi
140a8cb92:     	leaq	(,%rdi,8), %r9
140a8cb9a:     	movq	%rdi, %rax
140a8cb9d:     	shrq	$0x3d, %rax
140a8cba1:     	movl	%r10d, 0xca8(%rbp)
140a8cba8:     	jne	0x140a8e1a3 <.text+0xa8d1a3>
140a8cbae:     	movl	$0x0, %ecx
140a8cbb3:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8cbbd:     	cmpq	%rax, %r9
140a8cbc0:     	ja	0x140a8e1a5 <.text+0xa8d1a5>
140a8cbc6:     	movq	0xd40(%rbp), %rax
140a8cbcd:     	movq	-0x68(%rax), %rsi
140a8cbd1:     	testq	%r9, %r9
140a8cbd4:     	movq	%r11, 0xc98(%rbp)
140a8cbdb:     	je	0x140a8cc16 <.text+0xa8bc16>
140a8cbdd:     	movzbl	0x2ba8025(%rip), %eax   # 0x143634c09
140a8cbe4:     	movl	$0x4, %edx
140a8cbe9:     	movq	%r9, %rcx
140a8cbec:     	movq	%r9, 0xd18(%rbp)
140a8cbf3:     	callq	0x140613c10 <.text+0x612c10>
140a8cbf8:     	movl	$0x4, %ecx
140a8cbfd:     	movq	0xd18(%rbp), %r9
140a8cc04:     	testq	%rax, %rax
140a8cc07:     	je	0x140a8e1a5 <.text+0xa8d1a5>
140a8cc0d:     	movq	%rdi, 0xd38(%rbp)
140a8cc14:     	jmp	0x140a8cc26 <.text+0xa8bc26>
140a8cc16:     	movl	$0x4, %eax
140a8cc1b:     	movq	$0x0, 0xd38(%rbp)
140a8cc26:     	movq	%rax, 0xd18(%rbp)
140a8cc2d:     	movq	%rax, %rcx
140a8cc30:     	movq	%rsi, %rdx
140a8cc33:     	movq	%r9, %r8
140a8cc36:     	callq	0x142923c00 <.text+0x2922c00>
140a8cc3b:     	movq	0xd40(%rbp), %rax
140a8cc42:     	movq	-0x48(%rax), %rbx
140a8cc46:     	leaq	(,%rbx,4), %r9
140a8cc4e:     	movq	%rbx, %rax
140a8cc51:     	shrq	$0x3e, %rax
140a8cc55:     	jne	0x140a8e1b6 <.text+0xa8d1b6>
140a8cc5b:     	movl	$0x0, %ecx
140a8cc60:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8cc6a:     	cmpq	%rax, %r9
140a8cc6d:     	ja	0x140a8e1b8 <.text+0xa8d1b8>
140a8cc73:     	movq	0xd40(%rbp), %rax
140a8cc7a:     	movq	-0x50(%rax), %rsi
140a8cc7e:     	testq	%r9, %r9
140a8cc81:     	je	0x140a8ccbc <.text+0xa8bcbc>
140a8cc83:     	movzbl	0x2ba7f7f(%rip), %eax   # 0x143634c09
140a8cc8a:     	movl	$0x4, %edx
140a8cc8f:     	movq	%r9, %rcx
140a8cc92:     	movq	%r9, 0xcd0(%rbp)
140a8cc99:     	callq	0x140613c10 <.text+0x612c10>
140a8cc9e:     	movl	$0x4, %ecx
140a8cca3:     	movq	0xcd0(%rbp), %r9
140a8ccaa:     	testq	%rax, %rax
140a8ccad:     	je	0x140a8e1b8 <.text+0xa8d1b8>
140a8ccb3:     	movq	%rbx, 0xd08(%rbp)
140a8ccba:     	jmp	0x140a8cccc <.text+0xa8bccc>
140a8ccbc:     	movl	$0x4, %eax
140a8ccc1:     	movq	$0x0, 0xd08(%rbp)
140a8cccc:     	movq	%rax, %rcx
140a8cccf:     	movq	%rsi, %rdx
140a8ccd2:     	movq	%r9, %r8
140a8ccd5:     	movq	%rax, %rsi
140a8ccd8:     	callq	0x142923c00 <.text+0x2922c00>
140a8ccdd:     	movq	0xd40(%rbp), %rax
140a8cce4:     	movd	-0x40(%rax), %xmm7
140a8cce9:     	movq	0xd38(%rbp), %rcx
140a8ccf0:     	movq	%rcx, 0x840(%rbp)
140a8ccf7:     	movq	0xd18(%rbp), %rcx
140a8ccfe:     	movq	%rcx, 0x848(%rbp)
140a8cd05:     	movq	%rdi, 0x850(%rbp)
140a8cd0c:     	movq	0xd08(%rbp), %rcx
140a8cd13:     	movq	%rcx, 0x858(%rbp)
140a8cd1a:     	movq	%rsi, 0x860(%rbp)
140a8cd21:     	movq	%rbx, 0x868(%rbp)
140a8cd28:     	movd	%xmm7, 0x870(%rbp)
140a8cd30:     	movl	-0x38(%rax), %eax
140a8cd33:     	movq	%rax, 0xcd0(%rbp)
140a8cd3a:     	cmpl	$0x1, %eax
140a8cd3d:     	sete	%r8b
140a8cd41:     	movq	0xc98(%rbp), %rax
140a8cd48:     	movq	%rax, 0x28(%rsp)
140a8cd4d:     	movq	0xce0(%rbp), %rax
140a8cd54:     	movq	%rax, 0x20(%rsp)
140a8cd59:     	leaq	0x260(%rbp), %rcx
140a8cd60:     	leaq	0x840(%rbp), %rdx
140a8cd67:     	movaps	%xmm6, %xmm3
140a8cd6a:     	callq	0x14146bb70 <.text+0x146ab70>
140a8cd6f:     	movq	0x840(%rbp), %rdx
140a8cd76:     	testq	%rdx, %rdx
140a8cd79:     	je	0x140a8cd91 <.text+0xa8bd91>
140a8cd7b:     	movq	0x848(%rbp), %rcx
140a8cd82:     	shlq	$0x3, %rdx
140a8cd86:     	movl	$0x4, %r8d
140a8cd8c:     	callq	0x140613c20 <.text+0x612c20>
140a8cd91:     	movq	0x858(%rbp), %rdx
140a8cd98:     	testq	%rdx, %rdx
140a8cd9b:     	je	0x140a8cdb3 <.text+0xa8bdb3>
140a8cd9d:     	movq	0x860(%rbp), %rcx
140a8cda4:     	shlq	$0x2, %rdx
140a8cda8:     	movl	$0x4, %r8d
140a8cdae:     	callq	0x140613c20 <.text+0x612c20>
140a8cdb3:     	movq	0x268(%rbp), %rax
140a8cdba:     	movq	%rax, 0xd08(%rbp)
140a8cdc1:     	movq	0x270(%rbp), %rbx
140a8cdc8:     	leaq	(,%rbx,4), %rcx
140a8cdd0:     	testq	%rbx, %rbx
140a8cdd3:     	movq	%rbx, 0xd18(%rbp)
140a8cdda:     	movq	%rcx, 0xcd8(%rbp)
140a8cde1:     	je	0x140a8ce4c <.text+0xa8be4c>
140a8cde3:     	movzbl	0x2ba7e1f(%rip), %eax   # 0x143634c09
140a8cdea:     	movl	$0x4, %edx
140a8cdef:     	callq	0x140613c10 <.text+0x612c10>
140a8cdf4:     	movq	%rax, 0xd38(%rbp)
140a8cdfb:     	testq	%rax, %rax
140a8cdfe:     	je	0x140a8e189 <.text+0xa8d189>
140a8ce04:     	movq	0xd08(%rbp), %rax
140a8ce0b:     	leaq	0x8(%rax), %rsi
140a8ce0f:     	xorl	%edi, %edi
140a8ce11:     	movss	-0x8(%rsi), %xmm1
140a8ce16:     	movss	-0x4(%rsi), %xmm9
140a8ce1c:     	movss	(%rsi), %xmm2
140a8ce20:     	movq	0xce0(%rbp), %rcx
140a8ce27:     	callq	0x1409e9640 <.text+0x9e8640>
140a8ce2c:     	subss	%xmm0, %xmm9
140a8ce31:     	movq	0xd38(%rbp), %rax
140a8ce38:     	movss	%xmm9, (%rax,%rdi,4)
140a8ce3e:     	incq	%rdi
140a8ce41:     	addq	$0xc, %rsi
140a8ce45:     	cmpq	%rdi, %rbx
140a8ce48:     	jne	0x140a8ce11 <.text+0xa8be11>
140a8ce4a:     	jmp	0x140a8ce58 <.text+0xa8be58>
140a8ce4c:     	movl	$0x4, %eax
140a8ce51:     	movq	%rax, 0xd38(%rbp)
140a8ce58:     	movq	0xd40(%rbp), %rax
140a8ce5f:     	movss	-0x1c(%rax), %xmm12
140a8ce65:     	movzbl	-0xb(%rax), %eax
140a8ce69:     	movb	%al, 0xc98(%rbp)
140a8ce6f:     	movq	0xc18(%rbp), %rcx
140a8ce76:     	leaq	0x21a143b(%rip), %rdx   # 0x142c2e2b8
140a8ce7d:     	callq	0x140969720 <.text+0x968720>
140a8ce82:     	movq	%rax, %rsi
140a8ce85:     	testq	%rax, %rax
140a8ce88:     	je	0x140a8e0de <.text+0xa8d0de>
140a8ce8e:     	leaq	0x201096a(%rip), %rcx   # 0x142a9d7ff
140a8ce95:     	movq	%rcx, 0xc40(%rbp)
140a8ce9c:     	movq	$0x3a, 0xc48(%rbp)
140a8cea7:     	movq	%rsi, %rdi
140a8ceaa:     	addq	$0x10, %rdi
140a8ceae:     	cmpq	$0x3a, 0x18(%rsi)
140a8ceb3:     	jne	0x140a8e04b <.text+0xa8d04b>
140a8ceb9:     	movq	(%rdi), %rdx
140a8cebc:     	movl	$0x3a, %r8d
140a8cec2:     	callq	0x142923bf0 <.text+0x2922bf0>
140a8cec7:     	testl	%eax, %eax
140a8cec9:     	jne	0x140a8e04b <.text+0xa8d04b>
140a8cecf:     	movq	(%rsi), %rcx
140a8ced2:     	movq	0x8(%rsi), %rax
140a8ced6:     	callq	*0x18(%rax)
140a8ced9:     	movq	%rax, %rdi
140a8cedc:     	testq	%rax, %rax
140a8cedf:     	je	0x140a8e0de <.text+0xa8d0de>
140a8cee5:     	movzbl	0x2ba7d1d(%rip), %eax   # 0x143634c09
140a8ceec:     	movl	$0x20, %ecx
140a8cef1:     	movl	$0x8, %edx
140a8cef6:     	callq	0x140613c10 <.text+0x612c10>
140a8cefb:     	testq	%rax, %rax
140a8cefe:     	je	0x140a8e100 <.text+0xa8d100>
140a8cf04:     	movq	%rax, %rdx
140a8cf07:     	movq	$0x0, (%rax)
140a8cf0e:     	movq	$0x4, 0x8(%rax)
140a8cf16:     	movq	$0x0, 0x10(%rax)
140a8cf1e:     	movb	$0x0, 0x18(%rax)
140a8cf22:     	movq	0x60(%rdi), %rsi
140a8cf26:     	jmp	0x140a8cf2a <.text+0xa8bf2a>
140a8cf28:     	pause
140a8cf2a:     	movq	0x8(%rsi), %rax
140a8cf2e:     	nop
140a8cf30:     	cmpq	$-0x1, %rax
140a8cf34:     	je	0x140a8cf28 <.text+0xa8bf28>
140a8cf36:     	testq	%rax, %rax
140a8cf39:     	js	0x140a8dfa3 <.text+0xa8cfa3>
140a8cf3f:     	leaq	0x1(%rax), %rcx
140a8cf43:     	lock
140a8cf44:     	cmpxchgq	%rcx, 0x8(%rsi)
140a8cf49:     	jne	0x140a8cf30 <.text+0xa8bf30>
140a8cf4b:     	movq	%rsi, 0xb48(%rbp)
140a8cf52:     	movq	%rdx, 0xb90(%rbp)
140a8cf59:     	movq	%rdx, 0xb50(%rbp)
140a8cf60:     	movzbl	0x2ba7ca2(%rip), %eax   # 0x143634c09
140a8cf67:     	movl	$0x20, %ecx
140a8cf6c:     	movl	$0x8, %edx
140a8cf71:     	callq	0x140613c10 <.text+0x612c10>
140a8cf76:     	testq	%rax, %rax
140a8cf79:     	je	0x140a8e114 <.text+0xa8d114>
140a8cf7f:     	movq	%rax, %rbx
140a8cf82:     	movq	$0x0, (%rax)
140a8cf89:     	movq	$0x4, 0x8(%rax)
140a8cf91:     	movq	$0x0, 0x10(%rax)
140a8cf99:     	movb	$0x0, 0x18(%rax)
140a8cf9d:     	movq	0x60(%rdi), %rdx
140a8cfa1:     	jmp	0x140a8cfa5 <.text+0xa8bfa5>
140a8cfa3:     	pause
140a8cfa5:     	movq	0x8(%rdx), %rax
140a8cfa9:     	nopl	(%rax)
140a8cfb0:     	cmpq	$-0x1, %rax
140a8cfb4:     	je	0x140a8cfa3 <.text+0xa8bfa3>
140a8cfb6:     	testq	%rax, %rax
140a8cfb9:     	js	0x140a8dfbb <.text+0xa8cfbb>
140a8cfbf:     	leaq	0x1(%rax), %rcx
140a8cfc3:     	lock
140a8cfc4:     	cmpxchgq	%rcx, 0x8(%rdx)
140a8cfc9:     	jne	0x140a8cfb0 <.text+0xa8bfb0>
140a8cfcb:     	movq	%rdx, 0xb98(%rbp)
140a8cfd2:     	movq	%rdx, 0xb38(%rbp)
140a8cfd9:     	movq	%rbx, 0xb40(%rbp)
140a8cfe0:     	movzbl	0x2ba7c22(%rip), %eax   # 0x143634c09
140a8cfe7:     	movl	$0x20, %ecx
140a8cfec:     	movl	$0x8, %edx
140a8cff1:     	callq	0x140613c10 <.text+0x612c10>
140a8cff6:     	testq	%rax, %rax
140a8cff9:     	je	0x140a8e128 <.text+0xa8d128>
140a8cfff:     	movq	%rax, %rdx
140a8d002:     	movq	$0x0, (%rax)
140a8d009:     	movq	$0x4, 0x8(%rax)
140a8d011:     	movq	$0x0, 0x10(%rax)
140a8d019:     	movb	$0x0, 0x18(%rax)
140a8d01d:     	movq	0x60(%rdi), %r8
140a8d021:     	jmp	0x140a8d025 <.text+0xa8c025>
140a8d023:     	pause
140a8d025:     	movq	0x8(%r8), %rax
140a8d029:     	nopl	(%rax)
140a8d030:     	cmpq	$-0x1, %rax
140a8d034:     	je	0x140a8d023 <.text+0xa8c023>
140a8d036:     	testq	%rax, %rax
140a8d039:     	js	0x140a8dfd3 <.text+0xa8cfd3>
140a8d03f:     	leaq	0x1(%rax), %rcx
140a8d043:     	lock
140a8d044:     	cmpxchgq	%rcx, 0x8(%r8)
140a8d049:     	jne	0x140a8d030 <.text+0xa8c030>
140a8d04b:     	movq	%r8, 0xba8(%rbp)
140a8d052:     	movq	%r8, 0xb28(%rbp)
140a8d059:     	movq	%rdx, 0xba0(%rbp)
140a8d060:     	movq	%rdx, 0xb30(%rbp)
140a8d067:     	movzbl	0x2ba7b9b(%rip), %eax   # 0x143634c09
140a8d06e:     	movl	$0x20, %ecx
140a8d073:     	movl	$0x8, %edx
140a8d078:     	callq	0x140613c10 <.text+0x612c10>
140a8d07d:     	testq	%rax, %rax
140a8d080:     	je	0x140a8e13c <.text+0xa8d13c>
140a8d086:     	movq	%rax, %rdx
140a8d089:     	movq	$0x0, (%rax)
140a8d090:     	movq	$0x4, 0x8(%rax)
140a8d098:     	movq	$0x0, 0x10(%rax)
140a8d0a0:     	movb	$0x0, 0x18(%rax)
140a8d0a4:     	movq	0x60(%rdi), %r8
140a8d0a8:     	jmp	0x140a8d0ac <.text+0xa8c0ac>
140a8d0aa:     	pause
140a8d0ac:     	movq	0x8(%r8), %rax
140a8d0b0:     	cmpq	$-0x1, %rax
140a8d0b4:     	je	0x140a8d0aa <.text+0xa8c0aa>
140a8d0b6:     	testq	%rax, %rax
140a8d0b9:     	js	0x140a8dfeb <.text+0xa8cfeb>
140a8d0bf:     	leaq	0x1(%rax), %rcx
140a8d0c3:     	lock
140a8d0c4:     	cmpxchgq	%rcx, 0x8(%r8)
140a8d0c9:     	jne	0x140a8d0b0 <.text+0xa8c0b0>
140a8d0cb:     	movq	%r8, 0xbb8(%rbp)
140a8d0d2:     	movq	%r8, 0xb18(%rbp)
140a8d0d9:     	movq	%rdx, 0xbb0(%rbp)
140a8d0e0:     	movq	%rdx, 0xb20(%rbp)
140a8d0e7:     	movq	0xc18(%rbp), %rcx
140a8d0ee:     	leaq	0x21a126b(%rip), %rdx   # 0x142c2e360
140a8d0f5:     	callq	0x140969720 <.text+0x968720>
140a8d0fa:     	movq	%rax, %rdi
140a8d0fd:     	testq	%rax, %rax
140a8d100:     	je	0x140a8e0ef <.text+0xa8d0ef>
140a8d106:     	leaq	0x2010d2c(%rip), %rcx   # 0x142a9de39
140a8d10d:     	movq	%rcx, 0xc40(%rbp)
140a8d114:     	movq	$0x3d, 0xc48(%rbp)
140a8d11f:     	movq	%rdi, %r8
140a8d122:     	addq	$0x10, %r8
140a8d126:     	cmpq	$0x3d, 0x18(%rdi)
140a8d12b:     	jne	0x140a8e07f <.text+0xa8d07f>
140a8d131:     	movq	(%r8), %rdx
140a8d134:     	movq	%r8, 0xc78(%rbp)
140a8d13b:     	movl	$0x3d, %r8d
140a8d141:     	callq	0x142923bf0 <.text+0x2922bf0>
140a8d146:     	movq	0xc78(%rbp), %r8
140a8d14d:     	testl	%eax, %eax
140a8d14f:     	jne	0x140a8e07f <.text+0xa8d07f>
140a8d155:     	movq	(%rdi), %rcx
140a8d158:     	movq	0x8(%rdi), %rax
140a8d15c:     	callq	*0x18(%rax)
140a8d15f:     	movq	%rax, %rdi
140a8d162:     	testq	%rax, %rax
140a8d165:     	je	0x140a8e0ef <.text+0xa8d0ef>
140a8d16b:     	movq	%rbx, 0xbc0(%rbp)
140a8d172:     	movq	%rsi, 0xc78(%rbp)
140a8d179:     	movzbl	0x2ba7a89(%rip), %eax   # 0x143634c09
140a8d180:     	movl	$0x20, %ecx
140a8d185:     	movl	$0x8, %edx
140a8d18a:     	callq	0x140613c10 <.text+0x612c10>
140a8d18f:     	testq	%rax, %rax
140a8d192:     	movq	0xd18(%rbp), %rsi
140a8d199:     	movq	0xd38(%rbp), %rbx
140a8d1a0:     	je	0x140a8e150 <.text+0xa8d150>
140a8d1a6:     	movq	%rax, %rdx
140a8d1a9:     	movq	$0x0, (%rax)
140a8d1b0:     	movq	$0x4, 0x8(%rax)
140a8d1b8:     	movq	$0x0, 0x10(%rax)
140a8d1c0:     	movb	$0x0, 0x18(%rax)
140a8d1c4:     	movq	0x60(%rdi), %r8
140a8d1c8:     	jmp	0x140a8d1cc <.text+0xa8c1cc>
140a8d1ca:     	pause
140a8d1cc:     	movq	0x8(%r8), %rax
140a8d1d0:     	cmpq	$-0x1, %rax
140a8d1d4:     	je	0x140a8d1ca <.text+0xa8c1ca>
140a8d1d6:     	testq	%rax, %rax
140a8d1d9:     	js	0x140a8e003 <.text+0xa8d003>
140a8d1df:     	leaq	0x1(%rax), %rcx
140a8d1e3:     	lock
140a8d1e4:     	cmpxchgq	%rcx, 0x8(%r8)
140a8d1e9:     	jne	0x140a8d1d0 <.text+0xa8c1d0>
140a8d1eb:     	movq	%r8, 0xbd0(%rbp)
140a8d1f2:     	movq	%r8, 0xb08(%rbp)
140a8d1f9:     	movq	%rdx, 0xbc8(%rbp)
140a8d200:     	movq	%rdx, 0xb10(%rbp)
140a8d207:     	movzbl	0x2ba79fb(%rip), %eax   # 0x143634c09
140a8d20e:     	movl	$0x20, %ecx
140a8d213:     	movl	$0x8, %edx
140a8d218:     	callq	0x140613c10 <.text+0x612c10>
140a8d21d:     	testq	%rax, %rax
140a8d220:     	je	0x140a8e164 <.text+0xa8d164>
140a8d226:     	movq	%rax, %rdx
140a8d229:     	movq	$0x0, (%rax)
140a8d230:     	movq	$0x4, 0x8(%rax)
140a8d238:     	movq	$0x0, 0x10(%rax)
140a8d240:     	movb	$0x0, 0x18(%rax)
140a8d244:     	movq	0x60(%rdi), %r8
140a8d248:     	jmp	0x140a8d24c <.text+0xa8c24c>
140a8d24a:     	pause
140a8d24c:     	movq	0x8(%r8), %rax
140a8d250:     	cmpq	$-0x1, %rax
140a8d254:     	je	0x140a8d24a <.text+0xa8c24a>
140a8d256:     	testq	%rax, %rax
140a8d259:     	js	0x140a8e01b <.text+0xa8d01b>
140a8d25f:     	leaq	0x1(%rax), %rcx
140a8d263:     	lock
140a8d264:     	cmpxchgq	%rcx, 0x8(%r8)
140a8d269:     	jne	0x140a8d250 <.text+0xa8c250>
140a8d26b:     	movq	%r8, 0xbe0(%rbp)
140a8d272:     	movq	%r8, 0xaf8(%rbp)
140a8d279:     	movq	%rdx, 0xbd8(%rbp)
140a8d280:     	movq	%rdx, 0xb00(%rbp)
140a8d287:     	movzbl	0x2ba797b(%rip), %eax   # 0x143634c09
140a8d28e:     	movl	$0x20, %ecx
140a8d293:     	movl	$0x8, %edx
140a8d298:     	callq	0x140613c10 <.text+0x612c10>
140a8d29d:     	testq	%rax, %rax
140a8d2a0:     	je	0x140a8e178 <.text+0xa8d178>
140a8d2a6:     	movq	%rax, %r10
140a8d2a9:     	movq	$0x0, (%rax)
140a8d2b0:     	movq	$0x4, 0x8(%rax)
140a8d2b8:     	movq	$0x0, 0x10(%rax)
140a8d2c0:     	movb	$0x0, 0x18(%rax)
140a8d2c4:     	movq	0x60(%rdi), %rdi
140a8d2c8:     	jmp	0x140a8d2cc <.text+0xa8c2cc>
140a8d2ca:     	pause
140a8d2cc:     	movq	0x8(%rdi), %rax
140a8d2d0:     	cmpq	$-0x1, %rax
140a8d2d4:     	je	0x140a8d2ca <.text+0xa8c2ca>
140a8d2d6:     	testq	%rax, %rax
140a8d2d9:     	js	0x140a8e033 <.text+0xa8d033>
140a8d2df:     	leaq	0x1(%rax), %rcx
140a8d2e3:     	lock
140a8d2e4:     	cmpxchgq	%rcx, 0x8(%rdi)
140a8d2e9:     	jne	0x140a8d2d0 <.text+0xa8c2d0>
140a8d2eb:     	movq	%rdi, 0xae8(%rbp)
140a8d2f2:     	movq	%r10, 0xaf0(%rbp)
140a8d2f9:     	movaps	%xmm10, %xmm9
140a8d2fd:     	testq	%rsi, %rsi
140a8d300:     	je	0x140a8d39d <.text+0xa8c39d>
140a8d306:     	cmpq	$0x1, %rsi
140a8d30a:     	jne	0x140a8d314 <.text+0xa8c314>
140a8d30c:     	movaps	%xmm10, %xmm9
140a8d310:     	xorl	%eax, %eax
140a8d312:     	jmp	0x140a8d373 <.text+0xa8c373>
140a8d314:     	movq	%rsi, %rcx
140a8d317:     	andq	$-0x2, %rcx
140a8d31b:     	movaps	%xmm10, %xmm0
140a8d31f:     	xorl	%eax, %eax
140a8d321:     	movaps	%xmm10, %xmm9
140a8d325:     	movss	(%rbx,%rax,4), %xmm1
140a8d32a:     	movss	0x4(%rbx,%rax,4), %xmm2
140a8d330:     	cmpunordss	%xmm9, %xmm9
140a8d336:     	movaps	%xmm9, %xmm3
140a8d33a:     	andps	%xmm1, %xmm3
140a8d33d:     	minss	%xmm0, %xmm1
140a8d341:     	andnps	%xmm1, %xmm9
140a8d345:     	orps	%xmm3, %xmm9
140a8d349:     	movaps	%xmm2, %xmm0
140a8d34c:     	minss	%xmm9, %xmm0
140a8d351:     	cmpunordss	%xmm9, %xmm9
140a8d357:     	movaps	%xmm9, %xmm1
140a8d35b:     	andnps	%xmm0, %xmm1
140a8d35e:     	andps	%xmm2, %xmm9
140a8d362:     	orps	%xmm1, %xmm9
140a8d366:     	addq	$0x2, %rax
140a8d36a:     	movaps	%xmm9, %xmm0
140a8d36e:     	cmpq	%rax, %rcx
140a8d371:     	jne	0x140a8d325 <.text+0xa8c325>
140a8d373:     	testb	$0x1, %sil
140a8d377:     	je	0x140a8d39d <.text+0xa8c39d>
140a8d379:     	movss	(%rbx,%rax,4), %xmm0
140a8d37e:     	movaps	%xmm9, %xmm1
140a8d382:     	cmpunordss	%xmm9, %xmm1
140a8d388:     	movaps	%xmm1, %xmm2
140a8d38b:     	andps	%xmm0, %xmm2
140a8d38e:     	minss	%xmm9, %xmm0
140a8d393:     	andnps	%xmm0, %xmm1
140a8d396:     	orps	%xmm2, %xmm1
140a8d399:     	movaps	%xmm1, %xmm9
140a8d39d:     	testq	%rsi, %rsi
140a8d3a0:     	je	0x140a8d41a <.text+0xa8c41a>
140a8d3a2:     	movq	%rsi, %rax
140a8d3a5:     	decq	%rax
140a8d3a8:     	je	0x140a8d427 <.text+0xa8c427>
140a8d3aa:     	movq	%rsi, %rdx
140a8d3ad:     	andq	$-0x2, %rdx
140a8d3b1:     	movq	0xd08(%rbp), %r9
140a8d3b8:     	leaq	0x10(%r9), %r8
140a8d3bc:     	movaps	%xmm11, %xmm0
140a8d3c0:     	xorl	%ecx, %ecx
140a8d3c2:     	movaps	%xmm11, %xmm10
140a8d3c6:     	movss	-0xc(%r8), %xmm1
140a8d3cc:     	movss	(%r8), %xmm2
140a8d3d1:     	cmpunordss	%xmm10, %xmm10
140a8d3d7:     	movaps	%xmm10, %xmm3
140a8d3db:     	andps	%xmm1, %xmm3
140a8d3de:     	minss	%xmm0, %xmm1
140a8d3e2:     	andnps	%xmm1, %xmm10
140a8d3e6:     	orps	%xmm3, %xmm10
140a8d3ea:     	movaps	%xmm2, %xmm0
140a8d3ed:     	minss	%xmm10, %xmm0
140a8d3f2:     	cmpunordss	%xmm10, %xmm10
140a8d3f8:     	movaps	%xmm10, %xmm1
140a8d3fc:     	andnps	%xmm0, %xmm1
140a8d3ff:     	andps	%xmm2, %xmm10
140a8d403:     	orps	%xmm1, %xmm10
140a8d407:     	addq	$0x2, %rcx
140a8d40b:     	addq	$0x18, %r8
140a8d40f:     	movaps	%xmm10, %xmm0
140a8d413:     	cmpq	%rcx, %rdx
140a8d416:     	jne	0x140a8d3c6 <.text+0xa8c3c6>
140a8d418:     	jmp	0x140a8d434 <.text+0xa8c434>
140a8d41a:     	xorps	%xmm13, %xmm13
140a8d41e:     	movaps	%xmm11, %xmm10
140a8d422:     	jmp	0x140a8d505 <.text+0xa8c505>
140a8d427:     	movaps	%xmm11, %xmm10
140a8d42b:     	xorl	%ecx, %ecx
140a8d42d:     	movq	0xd08(%rbp), %r9
140a8d434:     	testb	$0x1, %sil
140a8d438:     	je	0x140a8d464 <.text+0xa8c464>
140a8d43a:     	leaq	(%rcx,%rcx,2), %rcx
140a8d43e:     	movss	0x4(%r9,%rcx,4), %xmm0
140a8d445:     	movaps	%xmm10, %xmm1
140a8d449:     	cmpunordss	%xmm10, %xmm1
140a8d44f:     	movaps	%xmm1, %xmm2
140a8d452:     	andps	%xmm0, %xmm2
140a8d455:     	minss	%xmm10, %xmm0
140a8d45a:     	andnps	%xmm0, %xmm1
140a8d45d:     	orps	%xmm2, %xmm1
140a8d460:     	movaps	%xmm1, %xmm10
140a8d464:     	testq	%rax, %rax
140a8d467:     	je	0x140a8d4cf <.text+0xa8c4cf>
140a8d469:     	movq	%rsi, %rcx
140a8d46c:     	andq	$-0x2, %rcx
140a8d470:     	leaq	0x10(%r9), %rdx
140a8d474:     	xorps	%xmm0, %xmm0
140a8d477:     	xorl	%eax, %eax
140a8d479:     	movaps	%xmm0, %xmm13
140a8d47d:     	movss	-0xc(%rdx), %xmm1
140a8d482:     	movss	(%rdx), %xmm2
140a8d486:     	cmpunordss	%xmm13, %xmm13
140a8d48c:     	movaps	%xmm13, %xmm3
140a8d490:     	andps	%xmm1, %xmm3
140a8d493:     	maxss	%xmm0, %xmm1
140a8d497:     	andnps	%xmm1, %xmm13
140a8d49b:     	orps	%xmm3, %xmm13
140a8d49f:     	movaps	%xmm2, %xmm0
140a8d4a2:     	maxss	%xmm13, %xmm0
140a8d4a7:     	cmpunordss	%xmm13, %xmm13
140a8d4ad:     	movaps	%xmm13, %xmm1
140a8d4b1:     	andnps	%xmm0, %xmm1
140a8d4b4:     	andps	%xmm2, %xmm13
140a8d4b8:     	orps	%xmm1, %xmm13
140a8d4bc:     	addq	$0x2, %rax
140a8d4c0:     	addq	$0x18, %rdx
140a8d4c4:     	movaps	%xmm13, %xmm0
140a8d4c8:     	cmpq	%rax, %rcx
140a8d4cb:     	jne	0x140a8d47d <.text+0xa8c47d>
140a8d4cd:     	jmp	0x140a8d4d5 <.text+0xa8c4d5>
140a8d4cf:     	xorps	%xmm13, %xmm13
140a8d4d3:     	xorl	%eax, %eax
140a8d4d5:     	testb	$0x1, %sil
140a8d4d9:     	je	0x140a8d505 <.text+0xa8c505>
140a8d4db:     	leaq	(%rax,%rax,2), %rax
140a8d4df:     	movss	0x4(%r9,%rax,4), %xmm0
140a8d4e6:     	movaps	%xmm13, %xmm1
140a8d4ea:     	cmpunordss	%xmm13, %xmm1
140a8d4f0:     	movaps	%xmm1, %xmm2
140a8d4f3:     	andps	%xmm0, %xmm2
140a8d4f6:     	maxss	%xmm13, %xmm0
140a8d4fb:     	andnps	%xmm0, %xmm1
140a8d4fe:     	orps	%xmm2, %xmm1
140a8d501:     	movaps	%xmm1, %xmm13
140a8d505:     	movq	0xd40(%rbp), %rax
140a8d50c:     	movq	-0x60(%rax), %rax
140a8d510:     	leaq	(,%rax,8), %rsi
140a8d518:     	movq	%rax, 0xd08(%rbp)
140a8d51f:     	shrq	$0x3d, %rax
140a8d523:     	jne	0x140a8e1c9 <.text+0xa8d1c9>
140a8d529:     	movl	$0x0, %ecx
140a8d52e:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8d538:     	cmpq	%rax, %rsi
140a8d53b:     	ja	0x140a8e1cb <.text+0xa8d1cb>
140a8d541:     	movq	0xd40(%rbp), %rax
140a8d548:     	movq	-0x68(%rax), %rbx
140a8d54c:     	testq	%rsi, %rsi
140a8d54f:     	movq	%r10, 0xbe8(%rbp)
140a8d556:     	je	0x140a8d58a <.text+0xa8c58a>
140a8d558:     	movzbl	0x2ba76aa(%rip), %eax   # 0x143634c09
140a8d55f:     	movl	$0x4, %edx
140a8d564:     	movq	%rsi, %rcx
140a8d567:     	callq	0x140613c10 <.text+0x612c10>
140a8d56c:     	movl	$0x4, %ecx
140a8d571:     	testq	%rax, %rax
140a8d574:     	je	0x140a8e1cb <.text+0xa8d1cb>
140a8d57a:     	movq	0xd08(%rbp), %rcx
140a8d581:     	movq	%rcx, 0xca0(%rbp)
140a8d588:     	jmp	0x140a8d59a <.text+0xa8c59a>
140a8d58a:     	movl	$0x4, %eax
140a8d58f:     	movq	$0x0, 0xca0(%rbp)
140a8d59a:     	movq	%rax, 0xc20(%rbp)
140a8d5a1:     	movq	%rax, %rcx
140a8d5a4:     	movq	%rbx, %rdx
140a8d5a7:     	movq	%rsi, %r8
140a8d5aa:     	callq	0x142923c00 <.text+0x2922c00>
140a8d5af:     	movq	0xd40(%rbp), %rax
140a8d5b6:     	movq	-0x48(%rax), %rsi
140a8d5ba:     	leaq	(,%rsi,4), %r9
140a8d5c2:     	movq	%rsi, %rax
140a8d5c5:     	shrq	$0x3e, %rax
140a8d5c9:     	jne	0x140a8e1dc <.text+0xa8d1dc>
140a8d5cf:     	movl	$0x0, %ecx
140a8d5d4:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8d5de:     	cmpq	%rax, %r9
140a8d5e1:     	ja	0x140a8e1de <.text+0xa8d1de>
140a8d5e7:     	movq	0xd40(%rbp), %rax
140a8d5ee:     	movq	-0x50(%rax), %rbx
140a8d5f2:     	testq	%r9, %r9
140a8d5f5:     	je	0x140a8d636 <.text+0xa8c636>
140a8d5f7:     	movzbl	0x2ba760b(%rip), %eax   # 0x143634c09
140a8d5fe:     	movl	$0x4, %edx
140a8d603:     	movq	%r9, %rcx
140a8d606:     	movq	%rsi, 0xc80(%rbp)
140a8d60d:     	movq	%r9, %rsi
140a8d610:     	callq	0x140613c10 <.text+0x612c10>
140a8d615:     	movl	$0x4, %ecx
140a8d61a:     	movq	%rsi, %r9
140a8d61d:     	movq	0xc80(%rbp), %rsi
140a8d624:     	testq	%rax, %rax
140a8d627:     	je	0x140a8e1de <.text+0xa8d1de>
140a8d62d:     	movq	%rsi, 0xca8(%rbp)
140a8d634:     	jmp	0x140a8d646 <.text+0xa8c646>
140a8d636:     	movl	$0x4, %eax
140a8d63b:     	movq	$0x0, 0xca8(%rbp)
140a8d646:     	movq	%rax, 0xc80(%rbp)
140a8d64d:     	movq	%rax, %rcx
140a8d650:     	movq	%rbx, %rdx
140a8d653:     	movq	%r9, %r8
140a8d656:     	callq	0x142923c00 <.text+0x2922c00>
140a8d65b:     	movq	0xd40(%rbp), %rax
140a8d662:     	movss	-0x40(%rax), %xmm0
140a8d667:     	movq	0xcd0(%rbp), %rdx
140a8d66e:     	leal	-0x2(%rdx), %ecx
140a8d671:     	cmpl	$0x2, %ecx
140a8d674:     	movl	$0x2, %eax
140a8d679:     	cmovael	%eax, %ecx
140a8d67c:     	testl	%ecx, %ecx
140a8d67e:     	je	0x140a8d6ab <.text+0xa8c6ab>
140a8d680:     	cmpl	$0x2, %ecx
140a8d683:     	jne	0x140a8d6a6 <.text+0xa8c6a6>
140a8d685:     	movq	0xd40(%rbp), %rcx
140a8d68c:     	movq	-0x24(%rcx), %rax
140a8d690:     	movq	%rax, 0xc50(%rbp)
140a8d697:     	movups	-0x34(%rcx), %xmm1
140a8d69b:     	movaps	%xmm1, 0xc40(%rbp)
140a8d6a2:     	movl	%edx, %eax
140a8d6a4:     	jmp	0x140a8d6ab <.text+0xa8c6ab>
140a8d6a6:     	movl	$0x3, %eax
140a8d6ab:     	cmpb	$0x0, 0xcc8(%rbp)
140a8d6b2:     	je	0x140a8d74e <.text+0xa8c74e>
140a8d6b8:     	movl	0xc88(%rbp), %ecx
140a8d6be:     	movl	0xc8b(%rbp), %edx
140a8d6c4:     	movl	%edx, 0xcb3(%rbp)
140a8d6ca:     	movl	%ecx, 0xcb0(%rbp)
140a8d6d0:     	movaps	0x7a0(%rbp), %xmm1
140a8d6d7:     	movaps	%xmm1, 0xbf0(%rbp)
140a8d6de:     	movaps	0x7c0(%rbp), %xmm1
140a8d6e5:     	movaps	0x7d0(%rbp), %xmm2
140a8d6ec:     	movaps	0x7e0(%rbp), %xmm3
140a8d6f3:     	movaps	%xmm1, 0x840(%rbp)
140a8d6fa:     	movaps	%xmm2, 0x850(%rbp)
140a8d701:     	movaps	%xmm3, 0x860(%rbp)
140a8d708:     	movq	0x7f0(%rbp), %rcx
140a8d70f:     	movq	%rcx, 0x870(%rbp)
140a8d716:     	movzbl	0xd2f(%rbp), %ecx
140a8d71d:     	movq	%rcx, 0xcd8(%rbp)
140a8d724:     	movq	0xc30(%rbp), %rcx
140a8d72b:     	movq	%rcx, 0xc28(%rbp)
140a8d732:     	movq	0xc38(%rbp), %rcx
140a8d739:     	movq	%rcx, 0xcd0(%rbp)
140a8d740:     	movl	0xc6c(%rbp), %ecx
140a8d746:     	movl	%ecx, 0xcc8(%rbp)
140a8d74c:     	jmp	0x140a8d7a4 <.text+0xa8c7a4>
140a8d74e:     	xorps	%xmm1, %xmm1
140a8d751:     	movaps	%xmm1, 0x860(%rbp)
140a8d758:     	movaps	%xmm1, 0x850(%rbp)
140a8d75f:     	movaps	%xmm1, 0x840(%rbp)
140a8d766:     	movq	$0x0, 0x870(%rbp)
140a8d771:     	movaps	%xmm1, 0xbf0(%rbp)
140a8d778:     	movq	$0x0, 0xcd8(%rbp)
140a8d783:     	movl	$0x4, %ecx
140a8d788:     	movq	%rcx, 0xcd0(%rbp)
140a8d78f:     	movq	$0x0, 0xc28(%rbp)
140a8d79a:     	movl	$0x0, 0xcc8(%rbp)
140a8d7a4:     	movaps	%xmm15, %xmm8
140a8d7a8:     	movaps	%xmm15, %xmm3
140a8d7ac:     	cmpb	$0x0, 0xc98(%rbp)
140a8d7b3:     	jne	0x140a8d7b8 <.text+0xa8c7b8>
140a8d7b5:     	movaps	%xmm6, %xmm3
140a8d7b8:     	movaps	%xmm6, %xmm15
140a8d7bc:     	movaps	%xmm11, %xmm6
140a8d7c0:     	movaps	%xmm14, %xmm2
140a8d7c4:     	jne	0x140a8d7ce <.text+0xa8c7ce>
140a8d7c6:     	movss	0x1ff0632(%rip), %xmm2  # 0x142a7de00
140a8d7ce:     	cmpq	$0x0, 0xb88(%rbp)
140a8d7d6:     	xorps	%xmm1, %xmm1
140a8d7d9:     	leaq	0x708(%rbp), %r8
140a8d7e0:     	movups	%xmm1, 0x30(%r8)
140a8d7e5:     	movups	%xmm1, 0x20(%r8)
140a8d7ea:     	movups	%xmm1, 0x10(%r8)
140a8d7ef:     	movups	%xmm1, (%r8)
140a8d7f3:     	movq	$0x0, 0x40(%r8)
140a8d7fb:     	movq	0xd18(%rbp), %rcx
140a8d802:     	movq	%rcx, 0x570(%rbp)
140a8d809:     	movq	0xd38(%rbp), %rdx
140a8d810:     	movq	%rdx, 0x578(%rbp)
140a8d817:     	movq	%rcx, 0x580(%rbp)
140a8d81e:     	movq	0xd40(%rbp), %r11
140a8d825:     	movss	-0x18(%r11), %xmm4
140a8d82b:     	movss	-0x14(%r11), %xmm5
140a8d831:     	movd	-0x10(%r11), %xmm1
140a8d837:     	movzbl	-0xc(%r11), %ecx
140a8d83c:     	movzbl	-0xa(%r11), %edx
140a8d841:     	movzbl	-0x9(%r11), %r10d
140a8d846:     	movzwl	-0x8(%r11), %r11d
140a8d84b:     	movq	0xca0(%rbp), %r9
140a8d852:     	movq	%r9, 0x500(%rbp)
140a8d859:     	movq	0xc20(%rbp), %r9
140a8d860:     	movq	%r9, 0x508(%rbp)
140a8d867:     	movq	0xd08(%rbp), %r9
140a8d86e:     	movq	%r9, 0x510(%rbp)
140a8d875:     	movq	0xca8(%rbp), %r9
140a8d87c:     	movq	%r9, 0x518(%rbp)
140a8d883:     	movq	0xc80(%rbp), %r9
140a8d88a:     	movq	%r9, 0x520(%rbp)
140a8d891:     	movq	%rsi, 0x528(%rbp)
140a8d898:     	movd	%xmm7, 0x530(%rbp)
140a8d8a0:     	movq	0x290(%rbp), %rsi
140a8d8a7:     	movq	%rsi, -0x1a0(%r8)
140a8d8ae:     	maxss	%xmm12, %xmm3
140a8d8b3:     	minss	%xmm3, %xmm2
140a8d8b7:     	movups	0x260(%rbp), %xmm3
140a8d8be:     	movups	0x270(%rbp), %xmm11
140a8d8c6:     	movups	0x280(%rbp), %xmm12
140a8d8ce:     	movups	%xmm12, -0x1b0(%r8)
140a8d8d6:     	movups	%xmm11, -0x1c0(%r8)
140a8d8de:     	movups	%xmm3, -0x1d0(%r8)
140a8d8e6:     	movss	%xmm10, 0x774(%rbp)
140a8d8ef:     	xorps	%xmm3, %xmm3
140a8d8f2:     	movss	%xmm13, 0x778(%rbp)
140a8d8fb:     	movss	%xmm9, 0x77c(%rbp)
140a8d904:     	movss	%xmm4, 0x780(%rbp)
140a8d90c:     	movss	%xmm2, 0x784(%rbp)
140a8d914:     	movss	%xmm5, 0x788(%rbp)
140a8d91c:     	movb	%cl, 0x794(%rbp)
140a8d922:     	movb	%dl, 0x795(%rbp)
140a8d928:     	movq	$0x0, 0x588(%rbp)
140a8d933:     	movq	$0x4, 0x590(%rbp)
140a8d93e:     	movups	%xmm3, -0x170(%r8)
140a8d946:     	movq	$0x4, 0x5a8(%rbp)
140a8d951:     	movq	$0x0, 0x5b0(%rbp)
140a8d95c:     	movl	$0x0, 0x5b8(%rbp)
140a8d966:     	movq	$0x0, 0x5c0(%rbp)
140a8d971:     	movq	$0x4, 0x5c8(%rbp)
140a8d97c:     	movq	$0x0, 0x5d0(%rbp)
140a8d987:     	movss	%xmm0, 0x78c(%rbp)
140a8d98f:     	movl	%eax, 0x758(%rbp)
140a8d995:     	movq	0xc50(%rbp), %rax
140a8d99c:     	movq	%rax, 0x64(%r8)
140a8d9a0:     	movaps	0xc40(%rbp), %xmm0
140a8d9a7:     	movups	%xmm0, 0x54(%r8)
140a8d9ac:     	movq	0xc78(%rbp), %rax
140a8d9b3:     	movq	%rax, 0x618(%rbp)
140a8d9ba:     	movq	0xb90(%rbp), %rax
140a8d9c1:     	movq	%rax, 0x620(%rbp)
140a8d9c8:     	movq	0xbd0(%rbp), %rax
140a8d9cf:     	movq	%rax, 0x628(%rbp)
140a8d9d6:     	movq	0xbc8(%rbp), %rax
140a8d9dd:     	movq	%rax, 0x630(%rbp)
140a8d9e4:     	movq	0xbe0(%rbp), %rax
140a8d9eb:     	movq	%rax, 0x638(%rbp)
140a8d9f2:     	movq	0xbd8(%rbp), %rax
140a8d9f9:     	movq	%rax, 0x640(%rbp)
140a8da00:     	movq	0xb98(%rbp), %rax
140a8da07:     	movq	%rax, 0x648(%rbp)
140a8da0e:     	movq	0xbc0(%rbp), %rax
140a8da15:     	movq	%rax, 0x650(%rbp)
140a8da1c:     	movq	0xba8(%rbp), %rax
140a8da23:     	movq	%rax, 0x658(%rbp)
140a8da2a:     	movq	0xba0(%rbp), %rax
140a8da31:     	movq	%rax, 0x660(%rbp)
140a8da38:     	movq	0xbb8(%rbp), %rax
140a8da3f:     	movq	%rax, 0x668(%rbp)
140a8da46:     	movq	0xbb0(%rbp), %rax
140a8da4d:     	movq	%rax, 0x670(%rbp)
140a8da54:     	movq	%rdi, 0x678(%rbp)
140a8da5b:     	movq	0xbe8(%rbp), %rax
140a8da62:     	movq	%rax, 0x680(%rbp)
140a8da69:     	movq	$0x0, 0x5d8(%rbp)
140a8da74:     	movq	$0x4, 0x5e0(%rbp)
140a8da7f:     	movups	%xmm3, -0x120(%r8)
140a8da87:     	movq	$0x4, 0x5f8(%rbp)
140a8da92:     	movq	$0x0, 0x600(%rbp)
140a8da9d:     	movq	0xcd8(%rbp), %rax
140a8daa4:     	movb	%al, 0x688(%rbp)
140a8daaa:     	movl	0xcb3(%rbp), %eax
140a8dab0:     	movl	%eax, -0x7c(%r8)
140a8dab4:     	movl	0xcb0(%rbp), %eax
140a8daba:     	movl	%eax, -0x7f(%r8)
140a8dabe:     	movq	0xc28(%rbp), %rax
140a8dac5:     	movq	%rax, 0x690(%rbp)
140a8dacc:     	movq	0xcd0(%rbp), %rax
140a8dad3:     	movq	%rax, 0x698(%rbp)
140a8dada:     	movaps	0xbf0(%rbp), %xmm0
140a8dae1:     	movups	%xmm0, -0x68(%r8)
140a8dae6:     	movq	0x870(%rbp), %rax
140a8daed:     	movq	%rax, -0x28(%r8)
140a8daf1:     	movdqa	0x840(%rbp), %xmm0
140a8daf9:     	movdqa	0x850(%rbp), %xmm2
140a8db01:     	movaps	0x860(%rbp), %xmm3
140a8db08:     	movups	%xmm3, -0x38(%r8)
140a8db0d:     	movdqu	%xmm2, -0x48(%r8)
140a8db13:     	movdqu	%xmm0, -0x58(%r8)
140a8db19:     	movl	0xcc8(%rbp), %eax
140a8db1f:     	movl	%eax, 0x6e8(%rbp)
140a8db25:     	movl	0xc64(%rbp), %eax
140a8db2b:     	movl	%eax, 0x6ec(%rbp)
140a8db31:     	movb	$0x0, 0x6f0(%rbp)
140a8db38:     	movq	$0x0, 0x6f8(%rbp)
140a8db43:     	movq	$0x4, 0x700(%rbp)
140a8db4e:     	movl	$0x0, 0x750(%rbp)
140a8db58:     	movl	$0x0, 0x608(%rbp)
140a8db62:     	movl	$0x0, 0x610(%rbp)
140a8db6c:     	movw	%r11w, 0x799(%rbp)
140a8db74:     	movzbl	0xc98(%rbp), %eax
140a8db7b:     	movb	%al, 0x796(%rbp)
140a8db81:     	setne	0x797(%rbp)
140a8db88:     	movd	%xmm1, 0x790(%rbp)
140a8db90:     	movb	%r10b, 0x798(%rbp)
140a8db97:     	movq	0xb68(%rbp), %rax
140a8db9e:     	cmpb	$0x1, 0xf4(%rax)
140a8dba5:     	jne	0x140a8dc47 <.text+0xa8cc47>
140a8dbab:     	movq	0x548(%rbp), %rbx
140a8dbb2:     	movq	%rbx, %rax
140a8dbb5:     	movl	$0xc, %ecx
140a8dbba:     	mulq	%rcx
140a8dbbd:     	movq	%rax, %r9
140a8dbc0:     	movq	0xcf8(%rbp), %rdi
140a8dbc7:     	jo	0x140a8e1ef <.text+0xa8d1ef>
140a8dbcd:     	movaps	%xmm6, %xmm11
140a8dbd1:     	movl	$0x0, %ecx
140a8dbd6:     	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
140a8dbe0:     	cmpq	%rax, %r9
140a8dbe3:     	movaps	%xmm15, %xmm6
140a8dbe7:     	movss	0x1ea6a10(%rip), %xmm10 # 0x142934600
140a8dbf0:     	movaps	%xmm8, %xmm15
140a8dbf4:     	xorps	%xmm8, %xmm8
140a8dbf8:     	pcmpeqd	%xmm7, %xmm7
140a8dbfc:     	ja	0x140a8e1f1 <.text+0xa8d1f1>
140a8dc02:     	movq	0x540(%rbp), %rsi
140a8dc09:     	testq	%r9, %r9
140a8dc0c:     	je	0x140a8dc70 <.text+0xa8cc70>
140a8dc0e:     	movzbl	0x2ba6ff4(%rip), %eax   # 0x143634c09
140a8dc15:     	movl	$0x4, %edx
140a8dc1a:     	movq	%r9, %rcx
140a8dc1d:     	movq	%r9, 0xd40(%rbp)
140a8dc24:     	callq	0x140613c10 <.text+0x612c10>
140a8dc29:     	movl	$0x4, %ecx
140a8dc2e:     	movq	0xd40(%rbp), %r9
140a8dc35:     	testq	%rax, %rax
140a8dc38:     	je	0x140a8e1f1 <.text+0xa8d1f1>
140a8dc3e:     	movq	%rbx, 0xd38(%rbp)
140a8dc45:     	jmp	0x140a8dc80 <.text+0xa8cc80>
140a8dc47:     	movq	0xcf8(%rbp), %rdi
140a8dc4e:     	movaps	%xmm6, %xmm11
140a8dc52:     	movaps	%xmm15, %xmm6
140a8dc56:     	movss	0x1ea69a1(%rip), %xmm10 # 0x142934600
140a8dc5f:     	movaps	%xmm8, %xmm15
140a8dc63:     	xorps	%xmm8, %xmm8
140a8dc67:     	pcmpeqd	%xmm7, %xmm7
140a8dc6b:     	jmp	0x140a8dd06 <.text+0xa8cd06>
140a8dc70:     	movl	$0x4, %eax
140a8dc75:     	movq	$0x0, 0xd38(%rbp)
140a8dc80:     	movq	%rax, %rcx
140a8dc83:     	movq	%rsi, %rdx
140a8dc86:     	movq	%r9, %r8
140a8dc89:     	movq	%rax, %rsi
140a8dc8c:     	callq	0x142923c00 <.text+0x2922c00>
140a8dc91:     	movl	0xc68(%rbp), %ecx
140a8dc97:     	andl	$0xffff0000, %ecx       # imm = 0xFFFF0000
140a8dc9d:     	incl	%ecx
140a8dc9f:     	movq	0xd38(%rbp), %rax
140a8dca6:     	movq	%rax, 0x840(%rbp)
140a8dcad:     	movq	%rsi, 0x848(%rbp)
140a8dcb4:     	movq	%rbx, 0x850(%rbp)
140a8dcbb:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a8dcc5:     	movq	%rax, 0x858(%rbp)
140a8dccc:     	movl	$0x1, 0x870(%rbp)
140a8dcd6:     	movl	$0x40400000, 0x880(%rbp) # imm = 0x40400000
140a8dce0:     	movl	%ecx, 0xc68(%rbp)
140a8dce6:     	movl	%ecx, 0x884(%rbp)
140a8dcec:     	movb	$0x1, 0xd56(%rbp)
140a8dcf3:     	leaq	0x840(%rbp), %rcx
140a8dcfa:     	movq	0xb78(%rbp), %rdx
140a8dd01:     	callq	0x140b04f70 <.text+0xb03f70>
140a8dd06:     	movl	$0x2a0, %r8d            # imm = 0x2A0
140a8dd0c:     	leaq	0x840(%rbp), %rsi
140a8dd13:     	movq	%rsi, %rcx
140a8dd16:     	leaq	0x500(%rbp), %rdx
140a8dd1d:     	callq	0x142923c00 <.text+0x2922c00>
140a8dd22:     	movq	0xcb8(%rbp), %r8
140a8dd29:     	movb	$0x0, 0xd56(%rbp)
140a8dd30:     	leaq	0x260(%rbp), %rcx
140a8dd37:     	movq	%rdi, %rdx
140a8dd3a:     	movq	%rsi, %r9
140a8dd3d:     	callq	0x14152a360 <.text+0x1529360>
140a8dd42:     	xorl	%eax, %eax
140a8dd44:     	cmpq	0x260(%rbp), %rax
140a8dd4b:     	jo	0x140a8dd60 <.text+0xa8cd60>
140a8dd4d:     	movb	$0x0, 0xd56(%rbp)
140a8dd54:     	leaq	0x260(%rbp), %rcx
140a8dd5b:     	callq	0x140895600 <.text+0x894600>
140a8dd60:     	xorl	%eax, %eax
140a8dd62:     	cmpq	-0x40(%rbp), %rax
140a8dd66:     	jo	0x140a8dd7a <.text+0xa8cd7a>
140a8dd68:     	cmpb	$0x0, 0xd28(%rbp)
140a8dd6f:     	je	0x140a8dd7a <.text+0xa8cd7a>
140a8dd71:     	leaq	-0x40(%rbp), %rcx
140a8dd75:     	callq	0x140895600 <.text+0x894600>
140a8dd7a:     	movq	0xd20(%rbp), %r8
140a8dd81:     	jmp	0x140a8c513 <.text+0xa8b513>
140a8dd86:     	leaq	0x204b52b(%rip), %rcx   # 0x142ad92b8
140a8dd8d:     	callq	0x1428d9310 <.text+0x28d8310>
140a8dd92:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8dd97:     	testq	%rax, %rax
140a8dd9a:     	jne	0x140a8ddaf <.text+0xa8cdaf>
140a8dd9c:     	xorl	%edx, %edx
140a8dd9e:     	callq	0x1428f87d0 <.text+0x28f77d0>
140a8dda3:     	movq	%rax, %rcx
140a8dda6:     	testq	%rax, %rax
140a8dda9:     	jne	0x140a8b7a6 <.text+0xa8a7a6>
140a8ddaf:     	leaq	0x21accea(%rip), %rcx   # 0x142c3aaa0
140a8ddb6:     	callq	0x1428d80d0 <.text+0x28d70d0>
140a8ddbb:     	cmpq	$0x0, 0x48(%rdi)
140a8ddc0:     	je	0x140a8de00 <.text+0xa8ce00>
140a8ddc2:     	movq	0x38(%rdi), %rsi
140a8ddc6:     	testq	%rsi, %rsi
140a8ddc9:     	je	0x140a8ddda <.text+0xa8cdda>
140a8ddcb:     	movq	0x30(%rdi), %rcx
140a8ddcf:     	leaq	0x11(%rsi), %r8
140a8ddd3:     	movb	$-0x1, %dl
140a8ddd5:     	callq	0x142923c20 <.text+0x2922c20>
140a8ddda:     	movq	$0x0, 0x48(%rdi)
140a8dde2:     	leaq	0x1(%rsi), %rax
140a8dde6:     	movq	%rax, %rcx
140a8dde9:     	shrq	$0x3, %rcx
140a8dded:     	andq	$-0x8, %rax
140a8ddf1:     	subq	%rcx, %rax
140a8ddf4:     	cmpq	$0x8, %rsi
140a8ddf8:     	cmovbq	%rsi, %rax
140a8ddfc:     	movq	%rax, 0x40(%rdi)
140a8de00:     	cmpq	$0x0, 0x78(%rdi)
140a8de05:     	je	0x140a8de4b <.text+0xa8ce4b>
140a8de07:     	movq	0x68(%rdi), %rsi
140a8de0b:     	testq	%rsi, %rsi
140a8de0e:     	je	0x140a8de25 <.text+0xa8ce25>
140a8de10:     	movq	0xcf0(%rbp), %rax
140a8de17:     	movq	(%rax), %rcx
140a8de1a:     	leaq	0x11(%rsi), %r8
140a8de1e:     	movb	$-0x1, %dl
140a8de20:     	callq	0x142923c20 <.text+0x2922c20>
140a8de25:     	movq	$0x0, 0x78(%rdi)
140a8de2d:     	leaq	0x1(%rsi), %rax
140a8de31:     	movq	%rax, %rcx
140a8de34:     	shrq	$0x3, %rcx
140a8de38:     	andq	$-0x8, %rax
140a8de3c:     	subq	%rcx, %rax
140a8de3f:     	cmpq	$0x8, %rsi
140a8de43:     	cmovbq	%rsi, %rax
140a8de47:     	movq	%rax, 0x70(%rdi)
140a8de4b:     	cmpq	$0x0, 0xa8(%rdi)
140a8de53:     	je	0x140a8dea2 <.text+0xa8cea2>
140a8de55:     	movq	0x98(%rdi), %rsi
140a8de5c:     	testq	%rsi, %rsi
140a8de5f:     	je	0x140a8de76 <.text+0xa8ce76>
140a8de61:     	movq	0xb80(%rbp), %rax
140a8de68:     	movq	(%rax), %rcx
140a8de6b:     	leaq	0x11(%rsi), %r8
140a8de6f:     	movb	$-0x1, %dl
140a8de71:     	callq	0x142923c20 <.text+0x2922c20>
140a8de76:     	movq	$0x0, 0xa8(%rdi)
140a8de81:     	leaq	0x1(%rsi), %rax
140a8de85:     	movq	%rax, %rcx
140a8de88:     	shrq	$0x3, %rcx
140a8de8c:     	andq	$-0x8, %rax
140a8de90:     	subq	%rcx, %rax
140a8de93:     	cmpq	$0x8, %rsi
140a8de97:     	cmovbq	%rsi, %rax
140a8de9b:     	movq	%rax, 0xa0(%rdi)
140a8dea2:     	cmpq	$0x0, 0xc00(%rbp)
140a8deaa:     	je	0x140a8def7 <.text+0xa8cef7>
140a8deac:     	callq	0x140c18130 <.text+0xc17130>
140a8deb1:     	movq	%rax, %rcx
140a8deb4:     	movq	(%rax), %rax
140a8deb7:     	cmpq	$0x1, %rax
140a8debb:     	jne	0x140a8df76 <.text+0xa8cf76>
140a8dec1:     	addq	$0x8, %rcx
140a8dec5:     	cmpq	$0x0, (%rcx)
140a8dec9:     	jne	0x140a8df97 <.text+0xa8cf97>
140a8decf:     	movq	$-0x1, (%rcx)
140a8ded6:     	movq	%rcx, 0xd48(%rbp)
140a8dedd:     	addq	$0x8, %rcx
140a8dee1:     	movq	0xc08(%rbp), %rdx
140a8dee8:     	callq	0x140c170f0 <.text+0xc160f0>
140a8deed:     	movq	0xd48(%rbp), %rax
140a8def4:     	incq	(%rax)
140a8def7:     	movaps	0xd60(%rbp), %xmm6
140a8defe:     	movaps	0xd70(%rbp), %xmm7
140a8df05:     	movaps	0xd80(%rbp), %xmm8
140a8df0d:     	movaps	0xd90(%rbp), %xmm9
140a8df15:     	movaps	0xda0(%rbp), %xmm10
140a8df1d:     	movaps	0xdb0(%rbp), %xmm11
140a8df25:     	movaps	0xdc0(%rbp), %xmm12
140a8df2d:     	movaps	0xdd0(%rbp), %xmm13
140a8df35:     	movaps	0xde0(%rbp), %xmm14
140a8df3d:     	movaps	0xdf0(%rbp), %xmm15
140a8df45:     	addq	$0xe88, %rsp            # imm = 0xE88
140a8df4c:     	popq	%rbx
140a8df4d:     	popq	%rdi
140a8df4e:     	popq	%rsi
140a8df4f:     	popq	%r12
140a8df51:     	popq	%r13
140a8df53:     	popq	%r14
140a8df55:     	popq	%r15
140a8df57:     	popq	%rbp
140a8df58:     	retq
140a8df59:     	leaq	0x204c233(%rip), %rcx   # 0x142ada193
140a8df60:     	leaq	0x204b3a1(%rip), %r8    # 0x142ad9308
140a8df67:     	movl	$0x32, %edx
140a8df6c:     	callq	0x1428d9430 <.text+0x28d8430>
140a8df71:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8df76:     	testq	%rax, %rax
140a8df79:     	jne	0x140a8ddaf <.text+0xa8cdaf>
140a8df7f:     	xorl	%edx, %edx
140a8df81:     	callq	0x1428f87d0 <.text+0x28f77d0>
140a8df86:     	movq	%rax, %rcx
140a8df89:     	testq	%rax, %rax
140a8df8c:     	jne	0x140a8dec5 <.text+0xa8cec5>
140a8df92:     	jmp	0x140a8ddaf <.text+0xa8cdaf>
140a8df97:     	leaq	0x21acbd2(%rip), %rcx   # 0x142c3ab70
140a8df9e:     	callq	0x1428d9250 <.text+0x28d8250>
140a8dfa3:     	leaq	0x2063396(%rip), %rcx   # 0x142af1340
140a8dfaa:     	leaq	0x20633ef(%rip), %rdx   # 0x142af13a0
140a8dfb1:     	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dfb6:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8dfbb:     	leaq	0x206337e(%rip), %rcx   # 0x142af1340
140a8dfc2:     	leaq	0x20633d7(%rip), %rdx   # 0x142af13a0
140a8dfc9:     	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dfce:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8dfd3:     	leaq	0x2063366(%rip), %rcx   # 0x142af1340
140a8dfda:     	leaq	0x20633bf(%rip), %rdx   # 0x142af13a0
140a8dfe1:     	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dfe6:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8dfeb:     	leaq	0x206334e(%rip), %rcx   # 0x142af1340
140a8dff2:     	leaq	0x20633a7(%rip), %rdx   # 0x142af13a0
140a8dff9:     	callq	0x1428a9a00 <.text+0x28a8a00>
140a8dffe:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e003:     	leaq	0x2063336(%rip), %rcx   # 0x142af1340
140a8e00a:     	leaq	0x206338f(%rip), %rdx   # 0x142af13a0
140a8e011:     	callq	0x1428a9a00 <.text+0x28a8a00>
140a8e016:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e01b:     	leaq	0x206331e(%rip), %rcx   # 0x142af1340
140a8e022:     	leaq	0x2063377(%rip), %rdx   # 0x142af13a0
140a8e029:     	callq	0x1428a9a00 <.text+0x28a8a00>
140a8e02e:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e033:     	leaq	0x2063306(%rip), %rcx   # 0x142af1340
140a8e03a:     	leaq	0x206335f(%rip), %rdx   # 0x142af13a0
140a8e041:     	callq	0x1428a9a00 <.text+0x28a8a00>
140a8e046:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e04b:     	movq	$0x0, 0x840(%rbp)
140a8e056:     	leaq	0x21a0123(%rip), %rax   # 0x142c2e180
140a8e05d:     	movq	%rax, 0x20(%rsp)
140a8e062:     	leaq	0xc40(%rbp), %rdx
140a8e069:     	leaq	0x840(%rbp), %r9
140a8e070:     	xorl	%ecx, %ecx
140a8e072:     	movq	%rdi, %r8
140a8e075:     	callq	0x1428cab41 <.text+0x28c9b41>
140a8e07a:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e07f:     	movq	$0x0, 0x840(%rbp)
140a8e08a:     	leaq	0x21a00ef(%rip), %rax   # 0x142c2e180
140a8e091:     	movq	%rax, 0x20(%rsp)
140a8e096:     	leaq	0xc40(%rbp), %rdx
140a8e09d:     	leaq	0x840(%rbp), %r9
140a8e0a4:     	xorl	%ecx, %ecx
140a8e0a6:     	callq	0x1428cab41 <.text+0x28c9b41>
140a8e0ab:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0b0:     	leaq	0x204b219(%rip), %rcx   # 0x142ad92d0
140a8e0b7:     	leaq	0x204b232(%rip), %r8    # 0x142ad92f0
140a8e0be:     	movl	$0x1c, %edx
140a8e0c3:     	callq	0x1428d9430 <.text+0x28d8430>
140a8e0c8:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0cd:     	leaq	0x21ac9cc(%rip), %rcx   # 0x142c3aaa0
140a8e0d4:     	callq	0x1428d80d0 <.text+0x28d70d0>
140a8e0d9:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0de:     	leaq	0x204b253(%rip), %rcx   # 0x142ad9338
140a8e0e5:     	callq	0x1428d9310 <.text+0x28d8310>
140a8e0ea:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e0ef:     	leaq	0x204b25a(%rip), %rcx   # 0x142ad9350
140a8e0f6:     	callq	0x1428d9310 <.text+0x28d8310>
140a8e0fb:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e100:     	movl	$0x8, %ecx
140a8e105:     	movl	$0x20, %edx
140a8e10a:     	callq	0x1428d9000 <.text+0x28d8000>
140a8e10f:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e114:     	movl	$0x8, %ecx
140a8e119:     	movl	$0x20, %edx
140a8e11e:     	callq	0x1428d9000 <.text+0x28d8000>
140a8e123:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e128:     	movl	$0x8, %ecx
140a8e12d:     	movl	$0x20, %edx
140a8e132:     	callq	0x1428d9000 <.text+0x28d8000>
140a8e137:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e13c:     	movl	$0x8, %ecx
140a8e141:     	movl	$0x20, %edx
140a8e146:     	callq	0x1428d9000 <.text+0x28d8000>
140a8e14b:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e150:     	movl	$0x8, %ecx
140a8e155:     	movl	$0x20, %edx
140a8e15a:     	callq	0x1428d9000 <.text+0x28d8000>
140a8e15f:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e164:     	movl	$0x8, %ecx
140a8e169:     	movl	$0x20, %edx
140a8e16e:     	callq	0x1428d9000 <.text+0x28d8000>
140a8e173:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e178:     	movl	$0x8, %ecx
140a8e17d:     	movl	$0x20, %edx
140a8e182:     	callq	0x1428d9000 <.text+0x28d8000>
140a8e187:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e189:     	leaq	0x204a9a0(%rip), %r8    # 0x142ad8b30
140a8e190:     	movl	$0x4, %ecx
140a8e195:     	movq	0xcd8(%rbp), %rdx
140a8e19c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1a1:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1a3:     	xorl	%ecx, %ecx
140a8e1a5:     	leaq	0x21aa9cc(%rip), %r8    # 0x142c38b78
140a8e1ac:     	movq	%r9, %rdx
140a8e1af:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1b4:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1b6:     	xorl	%ecx, %ecx
140a8e1b8:     	leaq	0x21aa9b9(%rip), %r8    # 0x142c38b78
140a8e1bf:     	movq	%r9, %rdx
140a8e1c2:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1c7:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1c9:     	xorl	%ecx, %ecx
140a8e1cb:     	leaq	0x21aa9a6(%rip), %r8    # 0x142c38b78
140a8e1d2:     	movq	%rsi, %rdx
140a8e1d5:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1da:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1dc:     	xorl	%ecx, %ecx
140a8e1de:     	leaq	0x21aa993(%rip), %r8    # 0x142c38b78
140a8e1e5:     	movq	%r9, %rdx
140a8e1e8:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e1ed:     	jmp	0x140a8e207 <.text+0xa8d207>
140a8e1ef:     	xorl	%ecx, %ecx
140a8e1f1:     	movb	$0x1, 0xd56(%rbp)
140a8e1f8:     	leaq	0x21aa979(%rip), %r8    # 0x142c38b78
140a8e1ff:     	movq	%r9, %rdx
140a8e202:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a8e207:     	ud2
140a8e209:     	nopl	(%rax)
140a8e210:     	movq	%rdx, 0x10(%rsp)
140a8e215:     	pushq	%rbp
140a8e216:     	pushq	%r15
140a8e218:     	pushq	%r14
140a8e21a:     	pushq	%r13
140a8e21c:     	pushq	%r12
140a8e21e:     	pushq	%rsi
140a8e21f:     	pushq	%rdi
140a8e220:     	pushq	%rbx
140a8e221:     	subq	$0xd8, %rsp
140a8e228:     	leaq	0x80(%rdx), %rbp
140a8e22f:     	movaps	%xmm15, 0x30(%rsp)
140a8e235:     	movaps	%xmm14, 0x40(%rsp)
140a8e23b:     	movaps	%xmm13, 0x50(%rsp)
140a8e241:     	movaps	%xmm12, 0x60(%rsp)
140a8e247:     	movaps	%xmm11, 0x70(%rsp)
140a8e24d:     	movaps	%xmm10, 0x80(%rsp)
140a8e256:     	movaps	%xmm9, 0x90(%rsp)
140a8e25f:     	movaps	%xmm8, 0xa0(%rsp)
140a8e268:     	movaps	%xmm7, 0xb0(%rsp)
140a8e270:     	movdqa	%xmm6, 0xc0(%rsp)
140a8e279:     	movq	0xd48(%rbp), %rax
140a8e280:     	incq	(%rax)
140a8e283:     	movaps	0xc0(%rsp), %xmm6
140a8e28b:     	movaps	0xb0(%rsp), %xmm7
140a8e293:     	movaps	0xa0(%rsp), %xmm8
140a8e29c:     	movaps	0x90(%rsp), %xmm9
140a8e2a5:     	movaps	0x80(%rsp), %xmm10
140a8e2ae:     	movaps	0x70(%rsp), %xmm11
140a8e2b4:     	movaps	0x60(%rsp), %xmm12
140a8e2ba:     	movaps	0x50(%rsp), %xmm13
140a8e2c0:     	movaps	0x40(%rsp), %xmm14
140a8e2c6:     	movaps	0x30(%rsp), %xmm15
140a8e2cc:     	addq	$0xd8, %rsp
140a8e2d3:     	popq	%rbx
140a8e2d4:     	popq	%rdi
140a8e2d5:     	popq	%rsi
140a8e2d6:     	popq	%r12
140a8e2d8:     	popq	%r13
140a8e2da:     	popq	%r14
140a8e2dc:     	popq	%r15
140a8e2de:     	popq	%rbp
140a8e2df:     	retq
140a8e2e0:     	movq	%rdx, 0x10(%rsp)
140a8e2e5:     	pushq	%rbp
140a8e2e6:     	pushq	%r15
140a8e2e8:     	pushq	%r14
140a8e2ea:     	pushq	%r13
140a8e2ec:     	pushq	%r12
140a8e2ee:     	pushq	%rsi
140a8e2ef:     	pushq	%rdi
140a8e2f0:     	pushq	%rbx
140a8e2f1:     	subq	$0xd8, %rsp
140a8e2f8:     	leaq	0x80(%rdx), %rbp
140a8e2ff:     	movaps	%xmm15, 0x30(%rsp)
140a8e305:     	movaps	%xmm14, 0x40(%rsp)
140a8e30b:     	movaps	%xmm13, 0x50(%rsp)
140a8e311:     	movaps	%xmm12, 0x60(%rsp)
140a8e317:     	movaps	%xmm11, 0x70(%rsp)
140a8e31d:     	movaps	%xmm10, 0x80(%rsp)
140a8e326:     	movaps	%xmm9, 0x90(%rsp)
140a8e32f:     	movaps	%xmm8, 0xa0(%rsp)
140a8e338:     	movaps	%xmm7, 0xb0(%rsp)
140a8e340:     	movdqa	%xmm6, 0xc0(%rsp)
140a8e349:     	cmpq	$0x0, 0xc00(%rbp)
140a8e351:     	je	0x140a8e382 <.text+0xa8d382>
140a8e353:     	leaq	0xc08(%rbp), %rax
140a8e35a:     	movq	%rax, 0xcb0(%rbp)
140a8e361:     	leaq	0xcb0(%rbp), %rax
140a8e368:     	movq	%rax, 0xbf0(%rbp)
140a8e36f:     	leaq	0x204a952(%rip), %rcx   # 0x142ad8cc8
140a8e376:     	leaq	0xbf0(%rbp), %rdx
140a8e37d:     	callq	0x14132f2d0 <.text+0x132e2d0>
140a8e382:     	movaps	0xc0(%rsp), %xmm6
140a8e38a:     	movaps	0xb0(%rsp), %xmm7
140a8e392:     	movaps	0xa0(%rsp), %xmm8
140a8e39b:     	movaps	0x90(%rsp), %xmm9
140a8e3a4:     	movaps	0x80(%rsp), %xmm10
140a8e3ad:     	movaps	0x70(%rsp), %xmm11
140a8e3b3:     	movaps	0x60(%rsp), %xmm12
140a8e3b9:     	movaps	0x50(%rsp), %xmm13
140a8e3bf:     	movaps	0x40(%rsp), %xmm14
140a8e3c5:     	movaps	0x30(%rsp), %xmm15
140a8e3cb:     	addq	$0xd8, %rsp
140a8e3d2:     	popq	%rbx
140a8e3d3:     	popq	%rdi
140a8e3d4:     	popq	%rsi
140a8e3d5:     	popq	%r12
140a8e3d7:     	popq	%r13
140a8e3d9:     	popq	%r14
140a8e3db:     	popq	%r15
140a8e3dd:     	popq	%rbp
140a8e3de:     	retq
140a8e3df:     	nop
140a8e3e0:     	movq	%rdx, 0x10(%rsp)
140a8e3e5:     	pushq	%rbp
140a8e3e6:     	pushq	%r15
140a8e3e8:     	pushq	%r14
140a8e3ea:     	pushq	%r13
140a8e3ec:     	pushq	%r12
140a8e3ee:     	pushq	%rsi
140a8e3ef:     	pushq	%rdi
140a8e3f0:     	pushq	%rbx
140a8e3f1:     	subq	$0xd8, %rsp
140a8e3f8:     	leaq	0x80(%rdx), %rbp
140a8e3ff:     	movaps	%xmm15, 0x30(%rsp)
140a8e405:     	movaps	%xmm14, 0x40(%rsp)
140a8e40b:     	movaps	%xmm13, 0x50(%rsp)
140a8e411:     	movaps	%xmm12, 0x60(%rsp)
140a8e417:     	movaps	%xmm11, 0x70(%rsp)
140a8e41d:     	movaps	%xmm10, 0x80(%rsp)
140a8e426:     	movaps	%xmm9, 0x90(%rsp)
140a8e42f:     	movaps	%xmm8, 0xa0(%rsp)
140a8e438:     	movaps	%xmm7, 0xb0(%rsp)
140a8e440:     	movdqa	%xmm6, 0xc0(%rsp)
140a8e449:     	cmpq	$0x0, 0xd30(%rbp)
140a8e451:     	movq	0xd20(%rbp), %rdx
140a8e458:     	je	0x140a8e472 <.text+0xa8d472>
140a8e45a:     	testq	%rdx, %rdx
140a8e45d:     	je	0x140a8e472 <.text+0xa8d472>
140a8e45f:     	movq	0xd00(%rbp), %rcx
140a8e466:     	movq	0xd10(%rbp), %r8
140a8e46d:     	callq	0x140613c20 <.text+0x612c20>
140a8e472:     	movb	$0x0, 0xd55(%rbp)
140a8e479:     	movdqa	0xc0(%rsp), %xmm6
140a8e482:     	movaps	0xb0(%rsp), %xmm7
140a8e48a:     	movaps	0xa0(%rsp), %xmm8
140a8e493:     	movaps	0x90(%rsp), %xmm9
140a8e49c:     	movaps	0x80(%rsp), %xmm10
140a8e4a5:     	movaps	0x70(%rsp), %xmm11
140a8e4ab:     	movaps	0x60(%rsp), %xmm12
140a8e4b1:     	movaps	0x50(%rsp), %xmm13
140a8e4b7:     	movaps	0x40(%rsp), %xmm14
140a8e4bd:     	movaps	0x30(%rsp), %xmm15
140a8e4c3:     	addq	$0xd8, %rsp
140a8e4ca:     	popq	%rbx
140a8e4cb:     	popq	%rdi
140a8e4cc:     	popq	%rsi
140a8e4cd:     	popq	%r12
140a8e4cf:     	popq	%r13
140a8e4d1:     	popq	%r14
140a8e4d3:     	popq	%r15
140a8e4d5:     	popq	%rbp
140a8e4d6:     	retq
140a8e4d7:     	nopw	(%rax,%rax)
140a8e4e0:     	movq	%rdx, 0x10(%rsp)
140a8e4e5:     	pushq	%rbp
140a8e4e6:     	pushq	%r15
140a8e4e8:     	pushq	%r14
140a8e4ea:     	pushq	%r13
140a8e4ec:     	pushq	%r12
140a8e4ee:     	pushq	%rsi
140a8e4ef:     	pushq	%rdi
140a8e4f0:     	pushq	%rbx
140a8e4f1:     	subq	$0xd8, %rsp
140a8e4f8:     	leaq	0x80(%rdx), %rbp
140a8e4ff:     	movaps	%xmm15, 0x30(%rsp)
140a8e505:     	movaps	%xmm14, 0x40(%rsp)
140a8e50b:     	movaps	%xmm13, 0x50(%rsp)
140a8e511:     	movaps	%xmm12, 0x60(%rsp)
140a8e517:     	movaps	%xmm11, 0x70(%rsp)
140a8e51d:     	movaps	%xmm10, 0x80(%rsp)
140a8e526:     	movaps	%xmm9, 0x90(%rsp)
140a8e52f:     	movaps	%xmm8, 0xa0(%rsp)
140a8e538:     	movaps	%xmm7, 0xb0(%rsp)
140a8e540:     	movdqa	%xmm6, 0xc0(%rsp)
140a8e549:     	cmpb	$0x0, 0xd55(%rbp)
140a8e550:     	je	0x140a8e55e <.text+0xa8d55e>
140a8e552:     	leaq	0x840(%rbp), %rcx
140a8e559:     	callq	0x140149f50 <.text+0x148f50>
140a8e55e:     	movaps	0xc0(%rsp), %xmm6
140a8e566:     	movaps	0xb0(%rsp), %xmm7
140a8e56e:     	movaps	0xa0(%rsp), %xmm8
140a8e577:     	movaps	0x90(%rsp), %xmm9
140a8e580:     	movaps	0x80(%rsp), %xmm10
140a8e589:     	movaps	0x70(%rsp), %xmm11
140a8e58f:     	movaps	0x60(%rsp), %xmm12
140a8e595:     	movaps	0x50(%rsp), %xmm13
140a8e59b:     	movaps	0x40(%rsp), %xmm14
140a8e5a1:     	movaps	0x30(%rsp), %xmm15
140a8e5a7:     	addq	$0xd8, %rsp
140a8e5ae:     	popq	%rbx
140a8e5af:     	popq	%rdi
140a8e5b0:     	popq	%rsi
140a8e5b1:     	popq	%r12
140a8e5b3:     	popq	%r13
140a8e5b5:     	popq	%r14
140a8e5b7:     	popq	%r15
140a8e5b9:     	popq	%rbp
140a8e5ba:     	retq
140a8e5bb:     	nopl	(%rax,%rax)
140a8e5c0:     	movq	%rdx, 0x10(%rsp)
140a8e5c5:     	pushq	%rbp
140a8e5c6:     	pushq	%r15
140a8e5c8:     	pushq	%r14
140a8e5ca:     	pushq	%r13
140a8e5cc:     	pushq	%r12
140a8e5ce:     	pushq	%rsi
140a8e5cf:     	pushq	%rdi
140a8e5d0:     	pushq	%rbx
140a8e5d1:     	subq	$0xd8, %rsp
140a8e5d8:     	leaq	0x80(%rdx), %rbp
140a8e5df:     	movaps	%xmm15, 0x30(%rsp)
140a8e5e5:     	movaps	%xmm14, 0x40(%rsp)
140a8e5eb:     	movaps	%xmm13, 0x50(%rsp)
140a8e5f1:     	movaps	%xmm12, 0x60(%rsp)
140a8e5f7:     	movaps	%xmm11, 0x70(%rsp)
140a8e5fd:     	movaps	%xmm10, 0x80(%rsp)
140a8e606:     	movaps	%xmm9, 0x90(%rsp)
140a8e60f:     	movaps	%xmm8, 0xa0(%rsp)
140a8e618:     	movdqa	%xmm7, 0xb0(%rsp)
140a8e621:     	movaps	%xmm6, 0xc0(%rsp)
140a8e629:     	movq	0xd48(%rbp), %rax
140a8e630:     	incq	(%rax)
140a8e633:     	movaps	0xc0(%rsp), %xmm6
140a8e63b:     	movaps	0xb0(%rsp), %xmm7
140a8e643:     	movaps	0xa0(%rsp), %xmm8
140a8e64c:     	movaps	0x90(%rsp), %xmm9
140a8e655:     	movaps	0x80(%rsp), %xmm10
140a8e65e:     	movaps	0x70(%rsp), %xmm11
140a8e664:     	movaps	0x60(%rsp), %xmm12
140a8e66a:     	movaps	0x50(%rsp), %xmm13
140a8e670:     	movaps	0x40(%rsp), %xmm14
140a8e676:     	movaps	0x30(%rsp), %xmm15
140a8e67c:     	addq	$0xd8, %rsp
140a8e683:     	popq	%rbx
140a8e684:     	popq	%rdi
140a8e685:     	popq	%rsi
140a8e686:     	popq	%r12
140a8e688:     	popq	%r13
140a8e68a:     	popq	%r14
140a8e68c:     	popq	%r15
140a8e68e:     	popq	%rbp
140a8e68f:     	retq
140a8e690:     	movq	%rdx, 0x10(%rsp)
140a8e695:     	pushq	%rbp
140a8e696:     	pushq	%r15
140a8e698:     	pushq	%r14
140a8e69a:     	pushq	%r13
140a8e69c:     	pushq	%r12
140a8e69e:     	pushq	%rsi
140a8e69f:     	pushq	%rdi
140a8e6a0:     	pushq	%rbx
140a8e6a1:     	subq	$0xd8, %rsp
140a8e6a8:     	leaq	0x80(%rdx), %rbp
140a8e6af:     	movaps	%xmm15, 0x30(%rsp)
140a8e6b5:     	movaps	%xmm14, 0x40(%rsp)
140a8e6bb:     	movaps	%xmm13, 0x50(%rsp)
140a8e6c1:     	movaps	%xmm12, 0x60(%rsp)
140a8e6c7:     	movaps	%xmm11, 0x70(%rsp)
140a8e6cd:     	movaps	%xmm10, 0x80(%rsp)
140a8e6d6:     	movaps	%xmm9, 0x90(%rsp)
140a8e6df:     	movaps	%xmm8, 0xa0(%rsp)
140a8e6e8:     	movdqa	%xmm7, 0xb0(%rsp)
140a8e6f1:     	movaps	%xmm6, 0xc0(%rsp)
140a8e6f9:     	movq	0x918(%rbp), %rdx
140a8e700:     	testq	%rdx, %rdx
140a8e703:     	jne	0x140a8e79f <.text+0xa8d79f>
140a8e709:     	movq	0x930(%rbp), %rdx
140a8e710:     	testq	%rdx, %rdx
140a8e713:     	jne	0x140a8e7c5 <.text+0xa8d7c5>
140a8e719:     	movq	0x9d0(%rbp), %rdx
140a8e720:     	testq	%rdx, %rdx
140a8e723:     	je	0x140a8e73b <.text+0xa8d73b>
140a8e725:     	movq	0x9d8(%rbp), %rcx
140a8e72c:     	shlq	$0x3, %rdx
140a8e730:     	movl	$0x4, %r8d
140a8e736:     	callq	0x140613c20 <.text+0x612c20>
140a8e73b:     	movb	$0x0, 0xd57(%rbp)
140a8e742:     	movaps	0xc0(%rsp), %xmm6
140a8e74a:     	movaps	0xb0(%rsp), %xmm7
140a8e752:     	movaps	0xa0(%rsp), %xmm8
140a8e75b:     	movaps	0x90(%rsp), %xmm9
140a8e764:     	movaps	0x80(%rsp), %xmm10
140a8e76d:     	movaps	0x70(%rsp), %xmm11
140a8e773:     	movaps	0x60(%rsp), %xmm12
140a8e779:     	movaps	0x50(%rsp), %xmm13
140a8e77f:     	movaps	0x40(%rsp), %xmm14
140a8e785:     	movaps	0x30(%rsp), %xmm15
140a8e78b:     	addq	$0xd8, %rsp
140a8e792:     	popq	%rbx
140a8e793:     	popq	%rdi
140a8e794:     	popq	%rsi
140a8e795:     	popq	%r12
140a8e797:     	popq	%r13
140a8e799:     	popq	%r14
140a8e79b:     	popq	%r15
140a8e79d:     	popq	%rbp
140a8e79e:     	retq
140a8e79f:     	movq	0x920(%rbp), %rcx
140a8e7a6:     	shlq	$0x2, %rdx
140a8e7aa:     	movl	$0x4, %r8d
140a8e7b0:     	callq	0x140613c20 <.text+0x612c20>
140a8e7b5:     	movq	0x930(%rbp), %rdx
140a8e7bc:     	testq	%rdx, %rdx
140a8e7bf:     	je	0x140a8e719 <.text+0xa8d719>
140a8e7c5:     	movq	0x938(%rbp), %rcx
140a8e7cc:     	shlq	$0x2, %rdx
140a8e7d0:     	movl	$0x4, %r8d
140a8e7d6:     	callq	0x140613c20 <.text+0x612c20>
140a8e7db:     	movq	0x9d0(%rbp), %rdx
140a8e7e2:     	testq	%rdx, %rdx
140a8e7e5:     	jne	0x140a8e725 <.text+0xa8d725>
140a8e7eb:     	jmp	0x140a8e73b <.text+0xa8d73b>
140a8e7f0:     	movq	%rdx, 0x10(%rsp)
140a8e7f5:     	pushq	%rbp
140a8e7f6:     	pushq	%r15
140a8e7f8:     	pushq	%r14
140a8e7fa:     	pushq	%r13
140a8e7fc:     	pushq	%r12
140a8e7fe:     	pushq	%rsi
140a8e7ff:     	pushq	%rdi
140a8e800:     	pushq	%rbx
140a8e801:     	subq	$0xd8, %rsp
140a8e808:     	leaq	0x80(%rdx), %rbp
140a8e80f:     	movaps	%xmm15, 0x30(%rsp)
140a8e815:     	movaps	%xmm14, 0x40(%rsp)
140a8e81b:     	movaps	%xmm13, 0x50(%rsp)
140a8e821:     	movaps	%xmm12, 0x60(%rsp)
140a8e827:     	movaps	%xmm11, 0x70(%rsp)
140a8e82d:     	movaps	%xmm10, 0x80(%rsp)
140a8e836:     	movaps	%xmm9, 0x90(%rsp)
140a8e83f:     	movaps	%xmm8, 0xa0(%rsp)
140a8e848:     	movdqa	%xmm7, 0xb0(%rsp)
140a8e851:     	movaps	%xmm6, 0xc0(%rsp)
140a8e859:     	leaq	0x840(%rbp), %rcx
140a8e860:     	callq	0x140895600 <.text+0x894600>
140a8e865:     	movb	$0x0, 0xd57(%rbp)
140a8e86c:     	movaps	0xc0(%rsp), %xmm6
140a8e874:     	movaps	0xb0(%rsp), %xmm7
140a8e87c:     	movaps	0xa0(%rsp), %xmm8
140a8e885:     	movaps	0x90(%rsp), %xmm9
140a8e88e:     	movaps	0x80(%rsp), %xmm10
140a8e897:     	movaps	0x70(%rsp), %xmm11
140a8e89d:     	movaps	0x60(%rsp), %xmm12
140a8e8a3:     	movaps	0x50(%rsp), %xmm13
140a8e8a9:     	movaps	0x40(%rsp), %xmm14
140a8e8af:     	movaps	0x30(%rsp), %xmm15
140a8e8b5:     	addq	$0xd8, %rsp
140a8e8bc:     	popq	%rbx
140a8e8bd:     	popq	%rdi
140a8e8be:     	popq	%rsi
140a8e8bf:     	popq	%r12
140a8e8c1:     	popq	%r13
140a8e8c3:     	popq	%r14
140a8e8c5:     	popq	%r15
140a8e8c7:     	popq	%rbp
140a8e8c8:     	retq
140a8e8c9:     	nopl	(%rax)
140a8e8d0:     	movq	%rdx, 0x10(%rsp)
140a8e8d5:     	pushq	%rbp
140a8e8d6:     	pushq	%r15
140a8e8d8:     	pushq	%r14
140a8e8da:     	pushq	%r13
140a8e8dc:     	pushq	%r12
140a8e8de:     	pushq	%rsi
140a8e8df:     	pushq	%rdi
140a8e8e0:     	pushq	%rbx
140a8e8e1:     	subq	$0xd8, %rsp
140a8e8e8:     	leaq	0x80(%rdx), %rbp
140a8e8ef:     	movaps	%xmm15, 0x30(%rsp)
140a8e8f5:     	movaps	%xmm14, 0x40(%rsp)
140a8e8fb:     	movaps	%xmm13, 0x50(%rsp)
140a8e901:     	movaps	%xmm12, 0x60(%rsp)
140a8e907:     	movaps	%xmm11, 0x70(%rsp)
140a8e90d:     	movaps	%xmm10, 0x80(%rsp)
140a8e916:     	movaps	%xmm9, 0x90(%rsp)
140a8e91f:     	movaps	%xmm8, 0xa0(%rsp)
140a8e928:     	movdqa	%xmm7, 0xb0(%rsp)
140a8e931:     	movaps	%xmm6, 0xc0(%rsp)
140a8e939:     	cmpq	$0x0, 0xd38(%rbp)
140a8e941:     	je	0x140a8e960 <.text+0xa8d960>
140a8e943:     	movq	0xd38(%rbp), %rdx
140a8e94a:     	shlq	$0x3, %rdx
140a8e94e:     	movl	$0x4, %r8d
140a8e954:     	movq	0xd18(%rbp), %rcx
140a8e95b:     	callq	0x140613c20 <.text+0x612c20>
140a8e960:     	movaps	0xc0(%rsp), %xmm6
140a8e968:     	movdqa	0xb0(%rsp), %xmm7
140a8e971:     	movaps	0xa0(%rsp), %xmm8
140a8e97a:     	movaps	0x90(%rsp), %xmm9
140a8e983:     	movaps	0x80(%rsp), %xmm10
140a8e98c:     	movaps	0x70(%rsp), %xmm11
140a8e992:     	movaps	0x60(%rsp), %xmm12
140a8e998:     	movaps	0x50(%rsp), %xmm13
140a8e99e:     	movaps	0x40(%rsp), %xmm14
140a8e9a4:     	movaps	0x30(%rsp), %xmm15
140a8e9aa:     	addq	$0xd8, %rsp
140a8e9b1:     	popq	%rbx
140a8e9b2:     	popq	%rdi
140a8e9b3:     	popq	%rsi
140a8e9b4:     	popq	%r12
140a8e9b6:     	popq	%r13
140a8e9b8:     	popq	%r14
140a8e9ba:     	popq	%r15
140a8e9bc:     	popq	%rbp
140a8e9bd:     	retq
140a8e9be:     	nop
140a8e9c0:     	movq	%rdx, 0x10(%rsp)
140a8e9c5:     	pushq	%rbp
140a8e9c6:     	pushq	%r15
140a8e9c8:     	pushq	%r14
140a8e9ca:     	pushq	%r13
140a8e9cc:     	pushq	%r12
140a8e9ce:     	pushq	%rsi
140a8e9cf:     	pushq	%rdi
140a8e9d0:     	pushq	%rbx
140a8e9d1:     	subq	$0xd8, %rsp
140a8e9d8:     	leaq	0x80(%rdx), %rbp
140a8e9df:     	movaps	%xmm15, 0x30(%rsp)
140a8e9e5:     	movaps	%xmm14, 0x40(%rsp)
140a8e9eb:     	movaps	%xmm13, 0x50(%rsp)
140a8e9f1:     	movaps	%xmm12, 0x60(%rsp)
140a8e9f7:     	movaps	%xmm11, 0x70(%rsp)
140a8e9fd:     	movaps	%xmm10, 0x80(%rsp)
140a8ea06:     	movaps	%xmm9, 0x90(%rsp)
140a8ea0f:     	movaps	%xmm8, 0xa0(%rsp)
140a8ea18:     	movdqa	%xmm7, 0xb0(%rsp)
140a8ea21:     	movaps	%xmm6, 0xc0(%rsp)
140a8ea29:     	leaq	0x840(%rbp), %rcx
140a8ea30:     	callq	0x1405d24f0 <.text+0x5d14f0>
140a8ea35:     	movaps	0xc0(%rsp), %xmm6
140a8ea3d:     	movdqa	0xb0(%rsp), %xmm7
140a8ea46:     	movaps	0xa0(%rsp), %xmm8
140a8ea4f:     	movaps	0x90(%rsp), %xmm9
140a8ea58:     	movaps	0x80(%rsp), %xmm10
140a8ea61:     	movaps	0x70(%rsp), %xmm11
140a8ea67:     	movaps	0x60(%rsp), %xmm12
140a8ea6d:     	movaps	0x50(%rsp), %xmm13
140a8ea73:     	movaps	0x40(%rsp), %xmm14
140a8ea79:     	movaps	0x30(%rsp), %xmm15
140a8ea7f:     	addq	$0xd8, %rsp
140a8ea86:     	popq	%rbx
140a8ea87:     	popq	%rdi
140a8ea88:     	popq	%rsi
140a8ea89:     	popq	%r12
140a8ea8b:     	popq	%r13
140a8ea8d:     	popq	%r14
140a8ea8f:     	popq	%r15
140a8ea91:     	popq	%rbp
140a8ea92:     	retq
140a8ea93:     	nopw	%cs:(%rax,%rax)
140a8eaa0:     	movq	%rdx, 0x10(%rsp)
140a8eaa5:     	pushq	%rbp
140a8eaa6:     	pushq	%r15
140a8eaa8:     	pushq	%r14
140a8eaaa:     	pushq	%r13
140a8eaac:     	pushq	%r12
140a8eaae:     	pushq	%rsi
140a8eaaf:     	pushq	%rdi
140a8eab0:     	pushq	%rbx
140a8eab1:     	subq	$0xd8, %rsp
140a8eab8:     	leaq	0x80(%rdx), %rbp
140a8eabf:     	movaps	%xmm15, 0x30(%rsp)
140a8eac5:     	movaps	%xmm14, 0x40(%rsp)
140a8eacb:     	movaps	%xmm13, 0x50(%rsp)
140a8ead1:     	movaps	%xmm12, 0x60(%rsp)
140a8ead7:     	movaps	%xmm11, 0x70(%rsp)
140a8eadd:     	movaps	%xmm10, 0x80(%rsp)
140a8eae6:     	movaps	%xmm9, 0x90(%rsp)
140a8eaef:     	movaps	%xmm8, 0xa0(%rsp)
140a8eaf8:     	movdqa	%xmm7, 0xb0(%rsp)
140a8eb01:     	movaps	%xmm6, 0xc0(%rsp)
140a8eb09:     	leaq	0x260(%rbp), %rcx
140a8eb10:     	callq	0x14014ba20 <.text+0x14aa20>
140a8eb15:     	movaps	0xc0(%rsp), %xmm6
140a8eb1d:     	movdqa	0xb0(%rsp), %xmm7
140a8eb26:     	movaps	0xa0(%rsp), %xmm8
140a8eb2f:     	movaps	0x90(%rsp), %xmm9
140a8eb38:     	movaps	0x80(%rsp), %xmm10
140a8eb41:     	movaps	0x70(%rsp), %xmm11
140a8eb47:     	movaps	0x60(%rsp), %xmm12
140a8eb4d:     	movaps	0x50(%rsp), %xmm13
140a8eb53:     	movaps	0x40(%rsp), %xmm14
140a8eb59:     	movaps	0x30(%rsp), %xmm15
140a8eb5f:     	addq	$0xd8, %rsp
140a8eb66:     	popq	%rbx
140a8eb67:     	popq	%rdi
140a8eb68:     	popq	%rsi
140a8eb69:     	popq	%r12
140a8eb6b:     	popq	%r13
140a8eb6d:     	popq	%r14
140a8eb6f:     	popq	%r15
140a8eb71:     	popq	%rbp
140a8eb72:     	retq
140a8eb73:     	nopw	%cs:(%rax,%rax)
140a8eb80:     	movq	%rdx, 0x10(%rsp)
140a8eb85:     	pushq	%rbp
140a8eb86:     	pushq	%r15
140a8eb88:     	pushq	%r14
140a8eb8a:     	pushq	%r13
140a8eb8c:     	pushq	%r12
140a8eb8e:     	pushq	%rsi
140a8eb8f:     	pushq	%rdi
140a8eb90:     	pushq	%rbx
140a8eb91:     	subq	$0xd8, %rsp
140a8eb98:     	leaq	0x80(%rdx), %rbp
140a8eb9f:     	movaps	%xmm15, 0x30(%rsp)
140a8eba5:     	movaps	%xmm14, 0x40(%rsp)
140a8ebab:     	movaps	%xmm13, 0x50(%rsp)
140a8ebb1:     	movaps	%xmm12, 0x60(%rsp)
140a8ebb7:     	movaps	%xmm11, 0x70(%rsp)
140a8ebbd:     	movaps	%xmm10, 0x80(%rsp)
140a8ebc6:     	movaps	%xmm9, 0x90(%rsp)
140a8ebcf:     	movaps	%xmm8, 0xa0(%rsp)
140a8ebd8:     	movdqa	%xmm7, 0xb0(%rsp)
140a8ebe1:     	movaps	%xmm6, 0xc0(%rsp)
140a8ebe9:     	movl	$0x4, %r8d
140a8ebef:     	movq	0xd38(%rbp), %rcx
140a8ebf6:     	movq	0xcd8(%rbp), %rdx
140a8ebfd:     	callq	0x140613c20 <.text+0x612c20>
140a8ec02:     	movaps	0xc0(%rsp), %xmm6
140a8ec0a:     	movdqa	0xb0(%rsp), %xmm7
140a8ec13:     	movaps	0xa0(%rsp), %xmm8
140a8ec1c:     	movaps	0x90(%rsp), %xmm9
140a8ec25:     	movaps	0x80(%rsp), %xmm10
140a8ec2e:     	movaps	0x70(%rsp), %xmm11
140a8ec34:     	movaps	0x60(%rsp), %xmm12
140a8ec3a:     	movaps	0x50(%rsp), %xmm13
140a8ec40:     	movaps	0x40(%rsp), %xmm14
140a8ec46:     	movaps	0x30(%rsp), %xmm15
140a8ec4c:     	addq	$0xd8, %rsp
140a8ec53:     	popq	%rbx
140a8ec54:     	popq	%rdi
140a8ec55:     	popq	%rsi
140a8ec56:     	popq	%r12
140a8ec58:     	popq	%r13
140a8ec5a:     	popq	%r14
140a8ec5c:     	popq	%r15
140a8ec5e:     	popq	%rbp
140a8ec5f:     	retq
140a8ec60:     	movq	%rdx, 0x10(%rsp)
140a8ec65:     	pushq	%rbp
140a8ec66:     	pushq	%r15
140a8ec68:     	pushq	%r14
140a8ec6a:     	pushq	%r13
140a8ec6c:     	pushq	%r12
140a8ec6e:     	pushq	%rsi
140a8ec6f:     	pushq	%rdi
140a8ec70:     	pushq	%rbx
140a8ec71:     	subq	$0xd8, %rsp
140a8ec78:     	leaq	0x80(%rdx), %rbp
140a8ec7f:     	movaps	%xmm15, 0x30(%rsp)
140a8ec85:     	movaps	%xmm14, 0x40(%rsp)
140a8ec8b:     	movaps	%xmm13, 0x50(%rsp)
140a8ec91:     	movaps	%xmm12, 0x60(%rsp)
140a8ec97:     	movaps	%xmm11, 0x70(%rsp)
140a8ec9d:     	movaps	%xmm10, 0x80(%rsp)
140a8eca6:     	movaps	%xmm9, 0x90(%rsp)
140a8ecaf:     	movaps	%xmm8, 0xa0(%rsp)
140a8ecb8:     	movdqa	%xmm7, 0xb0(%rsp)
140a8ecc1:     	movaps	%xmm6, 0xc0(%rsp)
140a8ecc9:     	cmpq	$0x0, 0xd18(%rbp)
140a8ecd1:     	je	0x140a8ecec <.text+0xa8dcec>
140a8ecd3:     	movl	$0x4, %r8d
140a8ecd9:     	movq	0xd38(%rbp), %rcx
140a8ece0:     	movq	0xcd8(%rbp), %rdx
140a8ece7:     	callq	0x140613c20 <.text+0x612c20>
140a8ecec:     	movaps	0xc0(%rsp), %xmm6
140a8ecf4:     	movdqa	0xb0(%rsp), %xmm7
140a8ecfd:     	movaps	0xa0(%rsp), %xmm8
140a8ed06:     	movaps	0x90(%rsp), %xmm9
140a8ed0f:     	movaps	0x80(%rsp), %xmm10
140a8ed18:     	movaps	0x70(%rsp), %xmm11
140a8ed1e:     	movaps	0x60(%rsp), %xmm12
140a8ed24:     	movaps	0x50(%rsp), %xmm13
140a8ed2a:     	movaps	0x40(%rsp), %xmm14
140a8ed30:     	movaps	0x30(%rsp), %xmm15
140a8ed36:     	addq	$0xd8, %rsp
140a8ed3d:     	popq	%rbx
140a8ed3e:     	popq	%rdi
140a8ed3f:     	popq	%rsi
140a8ed40:     	popq	%r12
140a8ed42:     	popq	%r13
140a8ed44:     	popq	%r14
140a8ed46:     	popq	%r15
140a8ed48:     	popq	%rbp
140a8ed49:     	retq
140a8ed4a:     	nopw	(%rax,%rax)
140a8ed50:     	movq	%rdx, 0x10(%rsp)
140a8ed55:     	pushq	%rbp
140a8ed56:     	pushq	%r15
140a8ed58:     	pushq	%r14
140a8ed5a:     	pushq	%r13
140a8ed5c:     	pushq	%r12
140a8ed5e:     	pushq	%rsi
140a8ed5f:     	pushq	%rdi
140a8ed60:     	pushq	%rbx
140a8ed61:     	subq	$0xd8, %rsp
140a8ed68:     	leaq	0x80(%rdx), %rbp
140a8ed6f:     	movaps	%xmm15, 0x30(%rsp)
140a8ed75:     	movaps	%xmm14, 0x40(%rsp)
140a8ed7b:     	movaps	%xmm13, 0x50(%rsp)
140a8ed81:     	movaps	%xmm12, 0x60(%rsp)
