
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1405c6fb0:     	pushq	%rbp
1405c6fb1:     	pushq	%rsi
1405c6fb2:     	pushq	%rdi
1405c6fb3:     	subq	$0x1a0, %rsp            # imm = 0x1A0
1405c6fba:     	leaq	0x80(%rsp), %rbp
1405c6fc2:     	movq	$-0x2, 0x118(%rbp)
1405c6fcd:     	movq	%rcx, %rsi
1405c6fd0:     	xorps	%xmm0, %xmm0
1405c6fd3:     	movups	%xmm0, -0x8(%rbp)
1405c6fd7:     	movups	%xmm0, 0x10(%rbp)
1405c6fdb:     	movups	%xmm0, 0x28(%rbp)
1405c6fdf:     	movups	%xmm0, 0x40(%rbp)
1405c6fe3:     	movups	%xmm0, 0x58(%rbp)
1405c6fe7:     	movl	$0x0, 0x68(%rbp)
1405c6fee:     	movups	%xmm0, 0x90(%rbp)
1405c6ff5:     	movups	%xmm0, 0xa8(%rbp)
1405c6ffc:     	movups	%xmm0, 0xc0(%rbp)
1405c7003:     	movups	%xmm0, 0xd8(%rbp)
1405c700a:     	movups	%xmm0, 0xf0(%rbp)
1405c7011:     	movl	$0x0, 0x100(%rbp)
1405c701b:     	movq	$0x0, -0x58(%rbp)
1405c7023:     	movq	$0x0, -0x28(%rbp)
1405c702b:     	movq	$0x8, -0x20(%rbp)
1405c7033:     	movq	$0x0, -0x18(%rbp)
1405c703b:     	movq	$0x10, -0x10(%rbp)
1405c7043:     	movq	$0x10, 0x8(%rbp)
1405c704b:     	movq	$0x10, 0x20(%rbp)
1405c7053:     	movq	$0x10, 0x38(%rbp)
1405c705b:     	movq	$0x10, 0x50(%rbp)
1405c7063:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405c706d:     	movq	%rax, 0x70(%rbp)
1405c7071:     	leaq	0x243f16a(%rip), %rax   # 0x142a061e2
1405c7078:     	movq	%rax, 0x78(%rbp)
1405c707c:     	movq	$0x3b, 0x80(%rbp)
1405c7087:     	movq	$0x10, 0x88(%rbp)
1405c7092:     	movq	$0x10, 0xa0(%rbp)
1405c709d:     	movq	$0x10, 0xb8(%rbp)
1405c70a8:     	movq	$0x10, 0xd0(%rbp)
1405c70b3:     	movq	$0x10, 0xe8(%rbp)
1405c70be:     	movl	$0x0, 0x108(%rbp)
1405c70c8:     	movw	$0x1, 0x10c(%rbp)
1405c70d1:     	movl	$0x0, 0x110(%rbp)
1405c70db:     	movzbl	0x306db27(%rip), %eax   # 0x143634c09
1405c70e2:     	movl	$0x170, %ecx            # imm = 0x170
1405c70e7:     	movl	$0x8, %edx
1405c70ec:     	callq	0x140613c10 <.text+0x612c10>
1405c70f1:     	testq	%rax, %rax
1405c70f4:     	je	0x1405c712b <.text+0x5c612b>
1405c70f6:     	movq	%rax, %rdi
1405c70f9:     	leaq	-0x58(%rbp), %rdx
1405c70fd:     	movl	$0x170, %r8d            # imm = 0x170
1405c7103:     	movq	%rax, %rcx
1405c7106:     	callq	0x142923c00 <.text+0x2922c00>
1405c710b:     	leaq	0x243ee2e(%rip), %r8    # 0x142a05f40
1405c7112:     	movq	%rsi, %rcx
1405c7115:     	movq	%rdi, %rdx
1405c7118:     	callq	0x140ce4b10 <.text+0xce3b10>
1405c711d:     	movq	%rsi, %rax
1405c7120:     	addq	$0x1a0, %rsp            # imm = 0x1A0
1405c7127:     	popq	%rdi
1405c7128:     	popq	%rsi
1405c7129:     	popq	%rbp
1405c712a:     	retq
1405c712b:     	movl	$0x8, %ecx
1405c7130:     	movl	$0x170, %edx            # imm = 0x170
1405c7135:     	callq	0x1428d9000 <.text+0x28d8000>
1405c713a:     	ud2
