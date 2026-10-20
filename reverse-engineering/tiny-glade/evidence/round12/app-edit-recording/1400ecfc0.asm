
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400ecfc0:     	pushq	%rbp
1400ecfc1:     	pushq	%rsi
1400ecfc2:     	pushq	%rdi
1400ecfc3:     	subq	$0x190, %rsp            # imm = 0x190
1400ecfca:     	leaq	0x80(%rsp), %rbp
1400ecfd2:     	movq	$-0x2, 0x108(%rbp)
1400ecfdd:     	movq	%rcx, %rsi
1400ecfe0:     	xorps	%xmm0, %xmm0
1400ecfe3:     	movups	%xmm0, -0x18(%rbp)
1400ecfe7:     	movups	%xmm0, (%rbp)
1400ecfeb:     	movups	%xmm0, 0x18(%rbp)
1400ecfef:     	movups	%xmm0, 0x30(%rbp)
1400ecff3:     	movups	%xmm0, 0x48(%rbp)
1400ecff7:     	movl	$0x0, 0x58(%rbp)
1400ecffe:     	movups	%xmm0, 0x80(%rbp)
1400ed005:     	movups	%xmm0, 0x98(%rbp)
1400ed00c:     	movups	%xmm0, 0xb0(%rbp)
1400ed013:     	movups	%xmm0, 0xc8(%rbp)
1400ed01a:     	movups	%xmm0, 0xe0(%rbp)
1400ed021:     	movl	$0x0, 0xf0(%rbp)
1400ed02b:     	movq	$0x0, -0x60(%rbp)
1400ed033:     	movq	$0x0, -0x38(%rbp)
1400ed03b:     	movq	$0x8, -0x30(%rbp)
1400ed043:     	movq	$0x0, -0x28(%rbp)
1400ed04b:     	movq	$0x10, -0x20(%rbp)
1400ed053:     	movq	$0x10, -0x8(%rbp)
1400ed05b:     	movq	$0x10, 0x10(%rbp)
1400ed063:     	movq	$0x10, 0x28(%rbp)
1400ed06b:     	movq	$0x10, 0x40(%rbp)
1400ed073:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1400ed07d:     	movq	%rax, 0x60(%rbp)
1400ed081:     	leaq	0x28503f7(%rip), %rax   # 0x14293d47f
1400ed088:     	movq	%rax, 0x68(%rbp)
1400ed08c:     	movq	$0x54, 0x70(%rbp)
1400ed094:     	movq	$0x10, 0x78(%rbp)
1400ed09c:     	movq	$0x10, 0x90(%rbp)
1400ed0a7:     	movq	$0x10, 0xa8(%rbp)
1400ed0b2:     	movq	$0x10, 0xc0(%rbp)
1400ed0bd:     	movq	$0x10, 0xd8(%rbp)
1400ed0c8:     	movl	$0x0, 0xf8(%rbp)
1400ed0d2:     	movw	$0x1, 0xfc(%rbp)
1400ed0db:     	movl	$0x0, 0x100(%rbp)
1400ed0e5:     	movzbl	0x3547b1d(%rip), %eax   # 0x143634c09
1400ed0ec:     	movl	$0x168, %ecx            # imm = 0x168
1400ed0f1:     	movl	$0x8, %edx
1400ed0f6:     	callq	0x140613c10 <.text+0x612c10>
1400ed0fb:     	testq	%rax, %rax
1400ed0fe:     	je	0x1400ed135 <.text+0xec135>
1400ed100:     	movq	%rax, %rdi
1400ed103:     	leaq	-0x60(%rbp), %rdx
1400ed107:     	movl	$0x168, %r8d            # imm = 0x168
1400ed10d:     	movq	%rax, %rcx
1400ed110:     	callq	0x142923c00 <.text+0x2922c00>
1400ed115:     	leaq	0x2850144(%rip), %r8    # 0x14293d260
1400ed11c:     	movq	%rsi, %rcx
1400ed11f:     	movq	%rdi, %rdx
1400ed122:     	callq	0x140ce4b10 <.text+0xce3b10>
1400ed127:     	movq	%rsi, %rax
1400ed12a:     	addq	$0x190, %rsp            # imm = 0x190
1400ed131:     	popq	%rdi
1400ed132:     	popq	%rsi
1400ed133:     	popq	%rbp
1400ed134:     	retq
1400ed135:     	movl	$0x8, %ecx
1400ed13a:     	movl	$0x168, %edx            # imm = 0x168
1400ed13f:     	callq	0x1428d9000 <.text+0x28d8000>
1400ed144:     	ud2
