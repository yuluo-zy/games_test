
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140325050:     	pushq	%rbp
140325051:     	pushq	%rsi
140325052:     	pushq	%rdi
140325053:     	subq	$0x1b0, %rsp            # imm = 0x1B0
14032505a:     	leaq	0x80(%rsp), %rbp
140325062:     	movq	$-0x2, 0x128(%rbp)
14032506d:     	movq	%rcx, %rsi
140325070:     	xorps	%xmm0, %xmm0
140325073:     	movups	%xmm0, 0x8(%rbp)
140325077:     	movups	%xmm0, 0x20(%rbp)
14032507b:     	movups	%xmm0, 0x38(%rbp)
14032507f:     	movups	%xmm0, 0x50(%rbp)
140325083:     	movups	%xmm0, 0x68(%rbp)
140325087:     	movl	$0x0, 0x78(%rbp)
14032508e:     	movups	%xmm0, 0xa0(%rbp)
140325095:     	movups	%xmm0, 0xb8(%rbp)
14032509c:     	movups	%xmm0, 0xd0(%rbp)
1403250a3:     	movups	%xmm0, 0xe8(%rbp)
1403250aa:     	movups	%xmm0, 0x100(%rbp)
1403250b1:     	movl	$0x0, 0x110(%rbp)
1403250bb:     	movq	$0x0, -0x58(%rbp)
1403250c3:     	movq	$0x0, -0x18(%rbp)
1403250cb:     	movq	$0x8, -0x10(%rbp)
1403250d3:     	movq	$0x0, -0x8(%rbp)
1403250db:     	movq	$0x10, (%rbp)
1403250e3:     	movq	$0x10, 0x18(%rbp)
1403250eb:     	movq	$0x10, 0x30(%rbp)
1403250f3:     	movq	$0x10, 0x48(%rbp)
1403250fb:     	movq	$0x10, 0x60(%rbp)
140325103:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14032510d:     	movq	%rax, 0x80(%rbp)
140325114:     	leaq	0x2698e0a(%rip), %rax   # 0x1429bdf25
14032511b:     	movq	%rax, 0x88(%rbp)
140325122:     	movq	$0x51, 0x90(%rbp)
14032512d:     	movq	$0x10, 0x98(%rbp)
140325138:     	movq	$0x10, 0xb0(%rbp)
140325143:     	movq	$0x10, 0xc8(%rbp)
14032514e:     	movq	$0x10, 0xe0(%rbp)
140325159:     	movq	$0x10, 0xf8(%rbp)
140325164:     	movl	$0x0, 0x118(%rbp)
14032516e:     	movw	$0x1, 0x11c(%rbp)
140325177:     	movl	$0x0, 0x120(%rbp)
140325181:     	movzbl	0x330fa81(%rip), %eax   # 0x143634c09
140325188:     	movl	$0x180, %ecx            # imm = 0x180
14032518d:     	movl	$0x8, %edx
140325192:     	callq	0x140613c10 <.text+0x612c10>
140325197:     	testq	%rax, %rax
14032519a:     	je	0x1403251d1 <.text+0x3241d1>
14032519c:     	movq	%rax, %rdi
14032519f:     	leaq	-0x58(%rbp), %rdx
1403251a3:     	movl	$0x180, %r8d            # imm = 0x180
1403251a9:     	movq	%rax, %rcx
1403251ac:     	callq	0x142923c00 <.text+0x2922c00>
1403251b1:     	leaq	0x2698b28(%rip), %r8    # 0x1429bdce0
1403251b8:     	movq	%rsi, %rcx
1403251bb:     	movq	%rdi, %rdx
1403251be:     	callq	0x140ce4b10 <.text+0xce3b10>
1403251c3:     	movq	%rsi, %rax
1403251c6:     	addq	$0x1b0, %rsp            # imm = 0x1B0
1403251cd:     	popq	%rdi
1403251ce:     	popq	%rsi
1403251cf:     	popq	%rbp
1403251d0:     	retq
1403251d1:     	movl	$0x8, %ecx
1403251d6:     	movl	$0x180, %edx            # imm = 0x180
1403251db:     	callq	0x1428d9000 <.text+0x28d8000>
1403251e0:     	ud2
