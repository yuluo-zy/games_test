
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14032e0b0:     	pushq	%r15
14032e0b2:     	pushq	%r14
14032e0b4:     	pushq	%rsi
14032e0b5:     	pushq	%rdi
14032e0b6:     	pushq	%rbx
14032e0b7:     	subq	$0x60, %rsp
14032e0bb:     	movaps	%xmm6, 0x50(%rsp)
14032e0c0:     	movq	%rdx, %rbx
14032e0c3:     	movq	%rcx, %rdi
14032e0c6:     	movq	0xb8(%rsp), %rsi
14032e0ce:     	movq	%r8, 0x40(%rsp)
14032e0d3:     	movq	%r9, 0x48(%rsp)
14032e0d8:     	leaq	0x40(%rsp), %rcx
14032e0dd:     	callq	0x140683c10 <.text+0x682c10>
14032e0e2:     	movq	%rax, 0x38(%rsp)
14032e0e7:     	leaq	0x38(%rsp), %rcx
14032e0ec:     	callq	0x1400885d0 <.text+0x875d0>
14032e0f1:     	movq	%rax, 0x20(%rsp)
14032e0f6:     	movabsq	$0x12ddabfdd3347ef1, %rax # imm = 0x12DDABFDD3347EF1
14032e100:     	movq	%rax, 0x28(%rsp)
14032e105:     	movabsq	$0x1f6d60c8858b8ff, %rax # imm = 0x1F6D60C8858B8FF
14032e10f:     	movq	%rax, 0x30(%rsp)
14032e114:     	leaq	0x20(%rsp), %rcx
14032e119:     	callq	0x1405bdf20 <.text+0x5bcf20>
14032e11e:     	movq	0x8(%rbx), %rcx
14032e122:     	movq	%rcx, 0x20(%rsp)
14032e127:     	movq	%rax, 0x28(%rsp)
14032e12c:     	leaq	0x20(%rsp), %rcx
14032e131:     	callq	0x1405bd910 <.text+0x5bc910>
14032e136:     	leaq	0x20(%rsp), %r14
14032e13b:     	movq	%r14, %rcx
14032e13e:     	movq	%rbx, %rdx
14032e141:     	movq	%rax, %r8
14032e144:     	callq	0x1406861b0 <.text+0x6851b0>
14032e149:     	movq	%r14, %rcx
14032e14c:     	callq	0x14032d690 <.text+0x32c690>
14032e151:     	testq	%rsi, %rsi
14032e154:     	js	0x14032e1f5 <.text+0x32d1f5>
14032e15a:     	movq	%rax, %rbx
14032e15d:     	je	0x14032e183 <.text+0x32d183>
14032e15f:     	movzbl	0x3306aa3(%rip), %eax   # 0x143634c09
14032e166:     	movl	$0x1, %r15d
14032e16c:     	movl	$0x1, %edx
14032e171:     	movq	%rsi, %rcx
14032e174:     	callq	0x140613c10 <.text+0x612c10>
14032e179:     	testq	%rax, %rax
14032e17c:     	je	0x14032e1f8 <.text+0x32d1f8>
14032e17e:     	movq	%rax, %r14
14032e181:     	jmp	0x14032e189 <.text+0x32d189>
14032e183:     	movl	$0x1, %r14d
14032e189:     	movss	0xc0(%rsp), %xmm6
14032e192:     	movq	0xb0(%rsp), %rdx
14032e19a:     	movq	%r14, %rcx
14032e19d:     	movq	%rsi, %r8
14032e1a0:     	callq	0x142923c00 <.text+0x2922c00>
14032e1a5:     	movq	(%rbx), %rdx
14032e1a8:     	testq	%rdx, %rdx
14032e1ab:     	je	0x14032e1bc <.text+0x32d1bc>
14032e1ad:     	movq	0x8(%rbx), %rcx
14032e1b1:     	movl	$0x1, %r8d
14032e1b7:     	callq	0x140613c20 <.text+0x612c20>
14032e1bc:     	movq	%rsi, (%rbx)
14032e1bf:     	movq	%r14, 0x8(%rbx)
14032e1c3:     	movq	%rsi, 0x10(%rbx)
14032e1c7:     	movss	%xmm6, 0x18(%rbx)
14032e1cc:     	movb	$0x0, 0x1c(%rbx)
14032e1d0:     	movq	0x30(%rsp), %rax
14032e1d5:     	movq	%rax, 0x10(%rdi)
14032e1d9:     	movups	0x20(%rsp), %xmm0
14032e1de:     	movups	%xmm0, (%rdi)
14032e1e1:     	movq	%rdi, %rax
14032e1e4:     	movaps	0x50(%rsp), %xmm6
14032e1e9:     	addq	$0x60, %rsp
14032e1ed:     	popq	%rbx
14032e1ee:     	popq	%rdi
14032e1ef:     	popq	%rsi
14032e1f0:     	popq	%r14
14032e1f2:     	popq	%r15
14032e1f4:     	retq
14032e1f5:     	xorl	%r15d, %r15d
14032e1f8:     	leaq	0x2690fd1(%rip), %r8    # 0x1429bf1d0
14032e1ff:     	movq	%r15, %rcx
14032e202:     	movq	%rsi, %rdx
14032e205:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14032e20a:     	int3
