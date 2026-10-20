
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140504f10:     	pushq	%r14
140504f12:     	pushq	%rsi
140504f13:     	pushq	%rdi
140504f14:     	pushq	%rbx
140504f15:     	subq	$0xa8, %rsp
140504f1c:     	movq	%rdx, %rdi
140504f1f:     	movq	%rcx, %rsi
140504f22:     	leaq	0x24e8eef(%rip), %rax   # 0x1429ede18
140504f29:     	movq	%rax, 0x20(%rsp)
140504f2e:     	movq	$0xf, 0x28(%rsp)
140504f37:     	leaq	0x24e8fb4(%rip), %r8    # 0x1429edef2
140504f3e:     	leaq	0x30(%rsp), %rbx
140504f43:     	movl	$0x3, %r9d
140504f49:     	movq	%rbx, %rcx
140504f4c:     	callq	0x14032e5d0 <.text+0x32d5d0>
140504f51:     	movq	%rbx, %rcx
140504f54:     	callq	0x140681d50 <.text+0x680d50>
140504f59:     	movq	%rax, %rcx
140504f5c:     	callq	0x140683c80 <.text+0x682c80>
140504f61:     	testb	%al, %al
140504f63:     	je	0x140504f89 <.text+0x503f89>
140504f65:     	movl	0x1c(%rsi), %eax
140504f68:     	movq	0x10(%rsi), %rcx
140504f6c:     	movl	%eax, (%rcx)
140504f6e:     	movq	(%rsi), %rcx
140504f71:     	movb	$0x0, 0x38(%rsp)
140504f76:     	movq	$0x8, 0x30(%rsp)
140504f7f:     	leaq	0x30(%rsp), %rdx
140504f84:     	callq	0x14093fad0 <.text+0x93ead0>
140504f89:     	leaq	0x24e8e97(%rip), %rax   # 0x1429ede27
140504f90:     	movq	%rax, 0x20(%rsp)
140504f95:     	movq	$0x12, 0x28(%rsp)
140504f9e:     	leaq	0x24e8f50(%rip), %r8    # 0x1429edef5
140504fa5:     	leaq	0x30(%rsp), %rbx
140504faa:     	movl	$0x8, %r9d
140504fb0:     	movq	%rbx, %rcx
140504fb3:     	movq	%rdi, %rdx
140504fb6:     	callq	0x14032e5d0 <.text+0x32d5d0>
140504fbb:     	leaq	0x90(%rsp), %r14
140504fc3:     	movq	%r14, %rcx
140504fc6:     	movq	%rbx, %rdx
140504fc9:     	movb	$0x3, %r8b
140504fcc:     	callq	0x14067e5b0 <.text+0x67d5b0>
140504fd1:     	movq	%r14, %rcx
140504fd4:     	callq	0x140681d50 <.text+0x680d50>
140504fd9:     	movq	%rax, %rcx
140504fdc:     	callq	0x140683c80 <.text+0x682c80>
140504fe1:     	testb	%al, %al
140504fe3:     	je	0x140505009 <.text+0x504009>
140504fe5:     	movl	0x1c(%rsi), %eax
140504fe8:     	movq	0x10(%rsi), %rcx
140504fec:     	movl	%eax, (%rcx)
140504fee:     	movq	(%rsi), %rcx
140504ff1:     	movb	$0x3, 0x38(%rsp)
140504ff6:     	movq	$0x7, 0x30(%rsp)
140504fff:     	leaq	0x30(%rsp), %rdx
140505004:     	callq	0x14093fad0 <.text+0x93ead0>
140505009:     	leaq	0x24e8ef1(%rip), %rax   # 0x1429edf01
140505010:     	movq	%rax, 0x20(%rsp)
140505015:     	movq	$0xe, 0x28(%rsp)
14050501e:     	leaq	0x24e8ed8(%rip), %r8    # 0x1429edefd
140505025:     	leaq	0x30(%rsp), %rbx
14050502a:     	movl	$0x4, %r9d
140505030:     	movq	%rbx, %rcx
140505033:     	movq	%rdi, %rdx
140505036:     	callq	0x14032e5d0 <.text+0x32d5d0>
14050503b:     	movq	%rbx, %rcx
14050503e:     	callq	0x140681d50 <.text+0x680d50>
140505043:     	movq	%rax, %rcx
140505046:     	callq	0x140683c80 <.text+0x682c80>
14050504b:     	testb	%al, %al
14050504d:     	je	0x14050506e <.text+0x50406e>
14050504f:     	movl	0x1c(%rsi), %eax
140505052:     	movq	0x10(%rsi), %rcx
140505056:     	movl	%eax, (%rcx)
140505058:     	movq	(%rsi), %rcx
14050505b:     	movq	$0x4, 0x30(%rsp)
140505064:     	leaq	0x30(%rsp), %rdx
140505069:     	callq	0x14093fad0 <.text+0x93ead0>
14050506e:     	nop
14050506f:     	addq	$0xa8, %rsp
140505076:     	popq	%rbx
140505077:     	popq	%rdi
140505078:     	popq	%rsi
140505079:     	popq	%r14
14050507b:     	retq
