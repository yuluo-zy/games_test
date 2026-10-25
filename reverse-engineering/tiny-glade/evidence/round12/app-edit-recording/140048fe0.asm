
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140048fe0:     	pushq	%rbp
140048fe1:     	pushq	%rsi
140048fe2:     	pushq	%rdi
140048fe3:     	subq	$0x190, %rsp            # imm = 0x190
140048fea:     	leaq	0x80(%rsp), %rbp
140048ff2:     	movq	$-0x2, 0x108(%rbp)
140048ffd:     	movq	%rcx, %rsi
140049000:     	xorps	%xmm0, %xmm0
140049003:     	movups	%xmm0, -0x18(%rbp)
140049007:     	movups	%xmm0, (%rbp)
14004900b:     	movups	%xmm0, 0x18(%rbp)
14004900f:     	movups	%xmm0, 0x30(%rbp)
140049013:     	movups	%xmm0, 0x48(%rbp)
140049017:     	movl	$0x0, 0x58(%rbp)
14004901e:     	movups	%xmm0, 0x80(%rbp)
140049025:     	movups	%xmm0, 0x98(%rbp)
14004902c:     	movups	%xmm0, 0xb0(%rbp)
140049033:     	movups	%xmm0, 0xc8(%rbp)
14004903a:     	movups	%xmm0, 0xe0(%rbp)
140049041:     	movl	$0x0, 0xf0(%rbp)
14004904b:     	movq	$0x0, -0x60(%rbp)
140049053:     	movq	$0x0, -0x38(%rbp)
14004905b:     	movq	$0x8, -0x30(%rbp)
140049063:     	movq	$0x0, -0x28(%rbp)
14004906b:     	movq	$0x10, -0x20(%rbp)
140049073:     	movq	$0x10, -0x8(%rbp)
14004907b:     	movq	$0x10, 0x10(%rbp)
140049083:     	movq	$0x10, 0x28(%rbp)
14004908b:     	movq	$0x10, 0x40(%rbp)
140049093:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14004909d:     	movq	%rax, 0x60(%rbp)
1400490a1:     	leaq	0x28e5354(%rip), %rax   # 0x14292e3fc
1400490a8:     	movq	%rax, 0x68(%rbp)
1400490ac:     	movq	$0x4e, 0x70(%rbp)
1400490b4:     	movq	$0x10, 0x78(%rbp)
1400490bc:     	movq	$0x10, 0x90(%rbp)
1400490c7:     	movq	$0x10, 0xa8(%rbp)
1400490d2:     	movq	$0x10, 0xc0(%rbp)
1400490dd:     	movq	$0x10, 0xd8(%rbp)
1400490e8:     	movl	$0x0, 0xf8(%rbp)
1400490f2:     	movw	$0x1, 0xfc(%rbp)
1400490fb:     	movl	$0x0, 0x100(%rbp)
140049105:     	movzbl	0x35ebafd(%rip), %eax   # 0x143634c09
14004910c:     	movl	$0x168, %ecx            # imm = 0x168
140049111:     	movl	$0x8, %edx
140049116:     	callq	0x140613c10 <.text+0x612c10>
14004911b:     	testq	%rax, %rax
14004911e:     	je	0x140049155 <.text+0x48155>
140049120:     	movq	%rax, %rdi
140049123:     	leaq	-0x60(%rbp), %rdx
140049127:     	movl	$0x168, %r8d            # imm = 0x168
14004912d:     	movq	%rax, %rcx
140049130:     	callq	0x142923c00 <.text+0x2922c00>
140049135:     	leaq	0x28e519c(%rip), %r8    # 0x14292e2d8
14004913c:     	movq	%rsi, %rcx
14004913f:     	movq	%rdi, %rdx
140049142:     	callq	0x140ce4b10 <.text+0xce3b10>
140049147:     	movq	%rsi, %rax
14004914a:     	addq	$0x190, %rsp            # imm = 0x190
140049151:     	popq	%rdi
140049152:     	popq	%rsi
140049153:     	popq	%rbp
140049154:     	retq
140049155:     	movl	$0x8, %ecx
14004915a:     	movl	$0x168, %edx            # imm = 0x168
14004915f:     	callq	0x1428d9000 <.text+0x28d8000>
140049164:     	ud2
