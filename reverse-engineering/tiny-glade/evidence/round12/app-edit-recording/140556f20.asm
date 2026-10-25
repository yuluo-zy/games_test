
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140556f20:     	pushq	%rbp
140556f21:     	pushq	%rsi
140556f22:     	pushq	%rdi
140556f23:     	subq	$0x310, %rsp            # imm = 0x310
140556f2a:     	leaq	0x80(%rsp), %rbp
140556f32:     	movq	$-0x2, 0x288(%rbp)
140556f3d:     	movq	%rcx, %rsi
140556f40:     	xorps	%xmm0, %xmm0
140556f43:     	movups	%xmm0, -0x38(%rbp)
140556f47:     	movups	%xmm0, -0x20(%rbp)
140556f4b:     	movups	%xmm0, -0x8(%rbp)
140556f4f:     	movups	%xmm0, 0x10(%rbp)
140556f53:     	movups	%xmm0, 0x28(%rbp)
140556f57:     	movl	$0x0, 0x38(%rbp)
140556f5e:     	movups	%xmm0, 0x60(%rbp)
140556f62:     	movups	%xmm0, 0x78(%rbp)
140556f66:     	movups	%xmm0, 0x90(%rbp)
140556f6d:     	movups	%xmm0, 0xa8(%rbp)
140556f74:     	movups	%xmm0, 0xc0(%rbp)
140556f7b:     	movl	$0x0, 0xd0(%rbp)
140556f85:     	movq	$0x0, -0x58(%rbp)
140556f8d:     	movq	$0x8, -0x50(%rbp)
140556f95:     	movq	$0x0, -0x48(%rbp)
140556f9d:     	movq	$0x10, -0x40(%rbp)
140556fa5:     	movq	$0x10, -0x28(%rbp)
140556fad:     	movq	$0x10, -0x10(%rbp)
140556fb5:     	movq	$0x10, 0x8(%rbp)
140556fbd:     	movq	$0x10, 0x20(%rbp)
140556fc5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140556fcf:     	movq	%rax, 0x40(%rbp)
140556fd3:     	leaq	0x24a40d6(%rip), %rcx   # 0x1429fb0b0
140556fda:     	movq	%rcx, 0x48(%rbp)
140556fde:     	movq	$0x5d, 0x50(%rbp)
140556fe6:     	movq	$0x10, 0x58(%rbp)
140556fee:     	movq	$0x10, 0x70(%rbp)
140556ff6:     	movq	$0x10, 0x88(%rbp)
140557001:     	movq	$0x10, 0xa0(%rbp)
14055700c:     	movq	$0x10, 0xb8(%rbp)
140557017:     	movl	$0x0, 0xd8(%rbp)
140557021:     	movw	$0x1, 0xdc(%rbp)
14055702a:     	movq	%rax, 0xe0(%rbp)
140557031:     	movl	$0x0, 0x280(%rbp)
14055703b:     	movzbl	0x30ddbc7(%rip), %eax   # 0x143634c09
140557042:     	movl	$0x2e0, %ecx            # imm = 0x2E0
140557047:     	movl	$0x8, %edx
14055704c:     	callq	0x140613c10 <.text+0x612c10>
140557051:     	testq	%rax, %rax
140557054:     	je	0x14055708b <.text+0x55608b>
140557056:     	movq	%rax, %rdi
140557059:     	leaq	-0x58(%rbp), %rdx
14055705d:     	movl	$0x2e0, %r8d            # imm = 0x2E0
140557063:     	movq	%rax, %rcx
140557066:     	callq	0x142923c00 <.text+0x2922c00>
14055706b:     	leaq	0x24a3dde(%rip), %r8    # 0x1429fae50
140557072:     	movq	%rsi, %rcx
140557075:     	movq	%rdi, %rdx
140557078:     	callq	0x140ce4b10 <.text+0xce3b10>
14055707d:     	movq	%rsi, %rax
140557080:     	addq	$0x310, %rsp            # imm = 0x310
140557087:     	popq	%rdi
140557088:     	popq	%rsi
140557089:     	popq	%rbp
14055708a:     	retq
14055708b:     	movl	$0x8, %ecx
140557090:     	movl	$0x2e0, %edx            # imm = 0x2E0
140557095:     	callq	0x1428d9000 <.text+0x28d8000>
14055709a:     	ud2
