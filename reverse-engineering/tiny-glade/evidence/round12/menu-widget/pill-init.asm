
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f9de80:     	pushq	%rbp
140f9de81:     	pushq	%r14
140f9de83:     	pushq	%rsi
140f9de84:     	pushq	%rdi
140f9de85:     	pushq	%rbx
140f9de86:     	subq	$0x80, %rsp
140f9de8d:     	leaq	0x80(%rsp), %rbp
140f9de95:     	movq	$-0x2, -0x8(%rbp)
140f9de9d:     	movq	(%rcx), %rdi
140f9dea0:     	cmpq	$0x0, 0x1b8(%rdi)
140f9dea8:     	je	0x140f9df75 <.text+0xf9cf75>
140f9deae:     	movl	%edx, %ebx
140f9deb0:     	movq	%rcx, %rsi
140f9deb3:     	leaq	0x8(%rcx), %rdx
140f9deb7:     	leaq	0x1c0(%rdi), %rcx
140f9debe:     	callq	0x1405b0d70 <.text+0x5afd70>
140f9dec3:     	movq	0x1a0(%rdi), %rcx
140f9deca:     	movq	0x1a8(%rdi), %rdx
140f9ded1:     	movq	%rdx, %r8
140f9ded4:     	andq	%rax, %r8
140f9ded7:     	shrq	$0x39, %rax
140f9dedb:     	movd	%eax, %xmm0
140f9dedf:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140f9dee3:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140f9dee8:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140f9deed:     	leaq	-0xc0(%rcx), %rax
140f9def4:     	movq	0x8(%rsi), %r9
140f9def8:     	xorl	%r10d, %r10d
140f9defb:     	pcmpeqd	%xmm1, %xmm1
140f9deff:     	movdqu	(%rcx,%r8), %xmm2
140f9df05:     	movdqa	%xmm2, %xmm3
140f9df09:     	pcmpeqb	%xmm0, %xmm3
140f9df0d:     	pmovmskb	%xmm3, %r11d
140f9df12:     	testl	%r11d, %r11d
140f9df15:     	je	0x140f9df40 <.text+0xf9cf40>
140f9df17:     	tzcntl	%r11d, %esi
140f9df1c:     	addq	%r8, %rsi
140f9df1f:     	andq	%rdx, %rsi
140f9df22:     	negq	%rsi
140f9df25:     	leaq	(%rsi,%rsi,2), %r14
140f9df29:     	shlq	$0x6, %r14
140f9df2d:     	cmpq	(%rax,%r14), %r9
140f9df31:     	je	0x140f9df5e <.text+0xf9cf5e>
140f9df33:     	leal	-0x1(%r11), %esi
140f9df37:     	andw	%r11w, %si
140f9df3b:     	movl	%esi, %r11d
140f9df3e:     	jne	0x140f9df17 <.text+0xf9cf17>
140f9df40:     	pcmpeqb	%xmm1, %xmm2
140f9df44:     	pmovmskb	%xmm2, %r11d
140f9df49:     	testl	%r11d, %r11d
140f9df4c:     	jne	0x140f9df75 <.text+0xf9cf75>
140f9df4e:     	addq	%r10, %r8
140f9df51:     	addq	$0x10, %r8
140f9df55:     	addq	$0x10, %r10
140f9df59:     	andq	%rdx, %r8
140f9df5c:     	jmp	0x140f9deff <.text+0xf9ceff>
140f9df5e:     	leaq	(%rcx,%r14), %rsi
140f9df62:     	movq	-0x38(%rcx,%r14), %rcx
140f9df67:     	testq	%rcx, %rcx
140f9df6a:     	je	0x140f9df8d <.text+0xf9cf8d>
140f9df6c:     	movq	-0x30(%rsi), %rax
140f9df70:     	jmp	0x140f9e05d <.text+0xf9d05d>
140f9df75:     	leaq	0x1c110c4(%rip), %rcx   # 0x142baf040
140f9df7c:     	leaq	0x1c110cd(%rip), %r8    # 0x142baf050
140f9df83:     	movl	$0xb, %edx
140f9df88:     	callq	0x1428d9330 <.text+0x28d8330>
140f9df8d:     	testb	%bl, %bl
140f9df8f:     	jne	0x140f9e09d <.text+0xf9d09d>
140f9df95:     	movq	0x260(%rdi), %rax
140f9df9c:     	lock
140f9df9d:     	incq	(%rax)
140f9dfa0:     	jle	0x140f9e0f8 <.text+0xf9d0f8>
140f9dfa6:     	movq	%rax, -0x58(%rbp)
140f9dfaa:     	movq	$0x0, -0x50(%rbp)
140f9dfb2:     	movq	$0x4, -0x48(%rbp)
140f9dfba:     	pxor	%xmm0, %xmm0
140f9dfbe:     	movdqu	%xmm0, -0x40(%rbp)
140f9dfc3:     	movq	$0x4, -0x30(%rbp)
140f9dfcb:     	movq	$0x0, -0x28(%rbp)
140f9dfd3:     	movq	$0x1, -0x20(%rbp)
140f9dfdb:     	movq	$0x0, -0x18(%rbp)
140f9dfe3:     	movb	$0x28, -0xc(%rbp)
140f9dfe7:     	movl	$0x3f800000, -0x10(%rbp) # imm = 0x3F800000
140f9dfee:     	lock
140f9dfef:     	decq	(%rax)
140f9dff2:     	jne	0x140f9dffd <.text+0xf9cffd>
140f9dff4:     	leaq	-0x58(%rbp), %rcx
140f9dff8:     	callq	0x140b8e120 <.text+0xb8d120>
140f9dffd:     	movzbl	0x2696c05(%rip), %eax   # 0x143634c09
140f9e004:     	movl	$0x48, %ecx
140f9e009:     	movl	$0x8, %edx
140f9e00e:     	callq	0x140613c10 <.text+0x612c10>
140f9e013:     	testq	%rax, %rax
140f9e016:     	je	0x140f9e0e9 <.text+0xf9d0e9>
140f9e01c:     	movq	%rax, %rcx
140f9e01f:     	movq	-0x10(%rbp), %rax
140f9e023:     	movq	%rax, 0x40(%rcx)
140f9e027:     	movdqu	-0x50(%rbp), %xmm0
140f9e02c:     	movdqu	-0x40(%rbp), %xmm1
140f9e031:     	movdqu	-0x30(%rbp), %xmm2
140f9e036:     	movdqu	-0x20(%rbp), %xmm3
140f9e03b:     	movdqu	%xmm3, 0x30(%rcx)
140f9e040:     	movdqu	%xmm2, 0x20(%rcx)
140f9e045:     	movdqu	%xmm1, 0x10(%rcx)
140f9e04a:     	movdqu	%xmm0, (%rcx)
140f9e04e:     	movq	%rcx, -0x38(%rsi)
140f9e052:     	leaq	0x1c11177(%rip), %rax   # 0x142baf1d0
140f9e059:     	movq	%rax, -0x30(%rsi)
140f9e05d:     	callq	*0x28(%rax)
140f9e060:     	movq	%rax, %rsi
140f9e063:     	movq	%rax, %rcx
140f9e066:     	callq	*0x18(%rdx)
140f9e069:     	testq	%rsi, %rsi
140f9e06c:     	je	0x140f9e0d1 <.text+0xf9d0d1>
140f9e06e:     	movabsq	$-0x7931e0e99eebebe0, %r8 # imm = 0x86CE1F1661141420
140f9e078:     	cmpq	%r8, %rax
140f9e07b:     	jne	0x140f9e0d1 <.text+0xf9d0d1>
140f9e07d:     	movabsq	$-0x5922082d28be21b3, %rcx # imm = 0xA6DDF7D2D741DE4D
140f9e087:     	cmpq	%rcx, %rdx
140f9e08a:     	jne	0x140f9e0d1 <.text+0xf9d0d1>
140f9e08c:     	movq	%rsi, %rax
140f9e08f:     	addq	$0x80, %rsp
140f9e096:     	popq	%rbx
140f9e097:     	popq	%rdi
140f9e098:     	popq	%rsi
140f9e099:     	popq	%r14
140f9e09b:     	popq	%rbp
140f9e09c:     	retq
140f9e09d:     	leaq	0x1c110d4(%rip), %rax   # 0x142baf178
140f9e0a4:     	movq	%rax, -0x50(%rbp)
140f9e0a8:     	movq	$0x1, -0x48(%rbp)
140f9e0b0:     	movq	$0x8, -0x40(%rbp)
140f9e0b8:     	pxor	%xmm0, %xmm0
140f9e0bc:     	movdqu	%xmm0, -0x38(%rbp)
140f9e0c1:     	leaq	0x1c110c0(%rip), %rdx   # 0x142baf188
140f9e0c8:     	leaq	-0x50(%rbp), %rcx
140f9e0cc:     	callq	0x1428d9390 <.text+0x28d8390>
140f9e0d1:     	leaq	0x1c10f90(%rip), %rcx   # 0x142baf068
140f9e0d8:     	leaq	0x1c10f91(%rip), %r8    # 0x142baf070
140f9e0df:     	movl	$0x8, %edx
140f9e0e4:     	callq	0x1428d9330 <.text+0x28d8330>
140f9e0e9:     	movl	$0x8, %ecx
140f9e0ee:     	movl	$0x48, %edx
140f9e0f3:     	callq	0x1428d9000 <.text+0x28d8000>
140f9e0f8:     	ud2
140f9e0fa:     	nopw	(%rax,%rax)
140f9e100:     	movq	%rdx, 0x10(%rsp)
140f9e105:     	pushq	%rbp
140f9e106:     	pushq	%r14
140f9e108:     	pushq	%rsi
140f9e109:     	pushq	%rdi
140f9e10a:     	pushq	%rbx
140f9e10b:     	subq	$0x20, %rsp
140f9e10f:     	leaq	0x80(%rdx), %rbp
140f9e116:     	leaq	-0x50(%rbp), %rcx
140f9e11a:     	callq	0x1405e6a70 <.text+0x5e5a70>
140f9e11f:     	nop
140f9e120:     	addq	$0x20, %rsp
140f9e124:     	popq	%rbx
140f9e125:     	popq	%rdi
140f9e126:     	popq	%rsi
140f9e127:     	popq	%r14
140f9e129:     	popq	%rbp
140f9e12a:     	retq
