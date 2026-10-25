
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140538010:     	pushq	%rbp
140538011:     	pushq	%rsi
140538012:     	pushq	%rdi
140538013:     	subq	$0x2a0, %rsp            # imm = 0x2A0
14053801a:     	leaq	0x80(%rsp), %rbp
140538022:     	movq	$-0x2, 0x218(%rbp)
14053802d:     	movq	%rcx, %rsi
140538030:     	xorps	%xmm0, %xmm0
140538033:     	movups	%xmm0, -0x40(%rbp)
140538037:     	movups	%xmm0, -0x28(%rbp)
14053803b:     	movups	%xmm0, -0x10(%rbp)
14053803f:     	movups	%xmm0, 0x8(%rbp)
140538043:     	movups	%xmm0, 0x20(%rbp)
140538047:     	movl	$0x0, 0x30(%rbp)
14053804e:     	movups	%xmm0, 0x58(%rbp)
140538052:     	movups	%xmm0, 0x70(%rbp)
140538056:     	movups	%xmm0, 0x88(%rbp)
14053805d:     	movups	%xmm0, 0xa0(%rbp)
140538064:     	movups	%xmm0, 0xb8(%rbp)
14053806b:     	movl	$0x0, 0xc8(%rbp)
140538075:     	movq	$0x0, -0x60(%rbp)
14053807d:     	movq	$0x8, -0x58(%rbp)
140538085:     	movq	$0x0, -0x50(%rbp)
14053808d:     	movq	$0x10, -0x48(%rbp)
140538095:     	movq	$0x10, -0x30(%rbp)
14053809d:     	movq	$0x10, -0x18(%rbp)
1405380a5:     	movq	$0x10, (%rbp)
1405380ad:     	movq	$0x10, 0x18(%rbp)
1405380b5:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1405380bf:     	movq	%rax, 0x38(%rbp)
1405380c3:     	leaq	0x24be5a3(%rip), %rcx   # 0x1429f666d
1405380ca:     	movq	%rcx, 0x40(%rbp)
1405380ce:     	movq	$0x6b, 0x48(%rbp)
1405380d6:     	movq	$0x10, 0x50(%rbp)
1405380de:     	movq	$0x10, 0x68(%rbp)
1405380e6:     	movq	$0x10, 0x80(%rbp)
1405380f1:     	movq	$0x10, 0x98(%rbp)
1405380fc:     	movq	$0x10, 0xb0(%rbp)
140538107:     	movl	$0x0, 0xd0(%rbp)
140538111:     	movw	$0x1, 0xd4(%rbp)
14053811a:     	movq	%rax, 0x1e8(%rbp)
140538121:     	movl	$0x0, 0x210(%rbp)
14053812b:     	movzbl	0x30fcad7(%rip), %eax   # 0x143634c09
140538132:     	movl	$0x278, %ecx            # imm = 0x278
140538137:     	movl	$0x8, %edx
14053813c:     	callq	0x140613c10 <.text+0x612c10>
140538141:     	testq	%rax, %rax
140538144:     	je	0x14053817b <.text+0x53717b>
140538146:     	movq	%rax, %rdi
140538149:     	leaq	-0x60(%rbp), %rdx
14053814d:     	movl	$0x278, %r8d            # imm = 0x278
140538153:     	movq	%rax, %rcx
140538156:     	callq	0x142923c00 <.text+0x2922c00>
14053815b:     	leaq	0x24be26e(%rip), %r8    # 0x1429f63d0
140538162:     	movq	%rsi, %rcx
140538165:     	movq	%rdi, %rdx
140538168:     	callq	0x140ce4b10 <.text+0xce3b10>
14053816d:     	movq	%rsi, %rax
140538170:     	addq	$0x2a0, %rsp            # imm = 0x2A0
140538177:     	popq	%rdi
140538178:     	popq	%rsi
140538179:     	popq	%rbp
14053817a:     	retq
14053817b:     	movl	$0x8, %ecx
140538180:     	movl	$0x278, %edx            # imm = 0x278
140538185:     	callq	0x1428d9000 <.text+0x28d8000>
14053818a:     	ud2
