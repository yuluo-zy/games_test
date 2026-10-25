
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14015ccb0:     	pushq	%rbp
14015ccb1:     	pushq	%rsi
14015ccb2:     	pushq	%rdi
14015ccb3:     	subq	$0x2d0, %rsp            # imm = 0x2D0
14015ccba:     	leaq	0x80(%rsp), %rbp
14015ccc2:     	movq	$-0x2, 0x248(%rbp)
14015cccd:     	movq	%rcx, %rsi
14015ccd0:     	xorps	%xmm0, %xmm0
14015ccd3:     	movups	%xmm0, -0x38(%rbp)
14015ccd7:     	movups	%xmm0, -0x20(%rbp)
14015ccdb:     	movups	%xmm0, -0x8(%rbp)
14015ccdf:     	movups	%xmm0, 0x10(%rbp)
14015cce3:     	movups	%xmm0, 0x28(%rbp)
14015cce7:     	movl	$0x0, 0x38(%rbp)
14015ccee:     	movups	%xmm0, 0x60(%rbp)
14015ccf2:     	movups	%xmm0, 0x78(%rbp)
14015ccf6:     	movups	%xmm0, 0x90(%rbp)
14015ccfd:     	movups	%xmm0, 0xa8(%rbp)
14015cd04:     	movups	%xmm0, 0xc0(%rbp)
14015cd0b:     	movl	$0x0, 0xd0(%rbp)
14015cd15:     	movq	$0x0, -0x58(%rbp)
14015cd1d:     	movq	$0x8, -0x50(%rbp)
14015cd25:     	movq	$0x0, -0x48(%rbp)
14015cd2d:     	movq	$0x10, -0x40(%rbp)
14015cd35:     	movq	$0x10, -0x28(%rbp)
14015cd3d:     	movq	$0x10, -0x10(%rbp)
14015cd45:     	movq	$0x10, 0x8(%rbp)
14015cd4d:     	movq	$0x10, 0x20(%rbp)
14015cd55:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14015cd5f:     	movq	%rax, 0x40(%rbp)
14015cd63:     	leaq	0x27ff801(%rip), %rcx   # 0x14295c56b
14015cd6a:     	movq	%rcx, 0x48(%rbp)
14015cd6e:     	movq	$0x61, 0x50(%rbp)
14015cd76:     	movq	$0x10, 0x58(%rbp)
14015cd7e:     	movq	$0x10, 0x70(%rbp)
14015cd86:     	movq	$0x10, 0x88(%rbp)
14015cd91:     	movq	$0x10, 0xa0(%rbp)
14015cd9c:     	movq	$0x10, 0xb8(%rbp)
14015cda7:     	movl	$0x0, 0xd8(%rbp)
14015cdb1:     	movw	$0x1, 0xdc(%rbp)
14015cdba:     	movq	%rax, 0x1d8(%rbp)
14015cdc1:     	movl	$0x0, 0x240(%rbp)
14015cdcb:     	movzbl	0x34d7e37(%rip), %eax   # 0x143634c09
14015cdd2:     	movl	$0x2a0, %ecx            # imm = 0x2A0
14015cdd7:     	movl	$0x8, %edx
14015cddc:     	callq	0x140613c10 <.text+0x612c10>
14015cde1:     	testq	%rax, %rax
14015cde4:     	je	0x14015ce1b <.text+0x15be1b>
14015cde6:     	movq	%rax, %rdi
14015cde9:     	leaq	-0x58(%rbp), %rdx
14015cded:     	movl	$0x2a0, %r8d            # imm = 0x2A0
14015cdf3:     	movq	%rax, %rcx
14015cdf6:     	callq	0x142923c00 <.text+0x2922c00>
14015cdfb:     	leaq	0x27ff56e(%rip), %r8    # 0x14295c370
14015ce02:     	movq	%rsi, %rcx
14015ce05:     	movq	%rdi, %rdx
14015ce08:     	callq	0x140ce4b10 <.text+0xce3b10>
14015ce0d:     	movq	%rsi, %rax
14015ce10:     	addq	$0x2d0, %rsp            # imm = 0x2D0
14015ce17:     	popq	%rdi
14015ce18:     	popq	%rsi
14015ce19:     	popq	%rbp
14015ce1a:     	retq
14015ce1b:     	movl	$0x8, %ecx
14015ce20:     	movl	$0x2a0, %edx            # imm = 0x2A0
14015ce25:     	callq	0x1428d9000 <.text+0x28d8000>
14015ce2a:     	ud2
