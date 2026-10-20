
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140342790:     	pushq	%rbp
140342791:     	pushq	%rsi
140342792:     	pushq	%rdi
140342793:     	subq	$0x1c0, %rsp            # imm = 0x1C0
14034279a:     	leaq	0x80(%rsp), %rbp
1403427a2:     	movq	$-0x2, 0x138(%rbp)
1403427ad:     	movq	%rcx, %rsi
1403427b0:     	xorps	%xmm0, %xmm0
1403427b3:     	movups	%xmm0, 0x18(%rbp)
1403427b7:     	movups	%xmm0, 0x30(%rbp)
1403427bb:     	movups	%xmm0, 0x48(%rbp)
1403427bf:     	movups	%xmm0, 0x60(%rbp)
1403427c3:     	movups	%xmm0, 0x78(%rbp)
1403427c7:     	movl	$0x0, 0x88(%rbp)
1403427d1:     	movups	%xmm0, 0xb0(%rbp)
1403427d8:     	movups	%xmm0, 0xc8(%rbp)
1403427df:     	movups	%xmm0, 0xe0(%rbp)
1403427e6:     	movups	%xmm0, 0xf8(%rbp)
1403427ed:     	movups	%xmm0, 0x110(%rbp)
1403427f4:     	movl	$0x0, 0x120(%rbp)
1403427fe:     	movq	$0x0, -0x58(%rbp)
140342806:     	movq	$0x0, -0x8(%rbp)
14034280e:     	movq	$0x8, (%rbp)
140342816:     	movq	$0x0, 0x8(%rbp)
14034281e:     	movq	$0x10, 0x10(%rbp)
140342826:     	movq	$0x10, 0x28(%rbp)
14034282e:     	movq	$0x10, 0x40(%rbp)
140342836:     	movq	$0x10, 0x58(%rbp)
14034283e:     	movq	$0x10, 0x70(%rbp)
140342846:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140342850:     	movq	%rax, 0x90(%rbp)
140342857:     	leaq	0x2680552(%rip), %rax   # 0x1429c2db0
14034285e:     	movq	%rax, 0x98(%rbp)
140342865:     	movq	$0x4d, 0xa0(%rbp)
140342870:     	movq	$0x10, 0xa8(%rbp)
14034287b:     	movq	$0x10, 0xc0(%rbp)
140342886:     	movq	$0x10, 0xd8(%rbp)
140342891:     	movq	$0x10, 0xf0(%rbp)
14034289c:     	movq	$0x10, 0x108(%rbp)
1403428a7:     	movl	$0x0, 0x128(%rbp)
1403428b1:     	movw	$0x1, 0x12c(%rbp)
1403428ba:     	movl	$0x0, 0x130(%rbp)
1403428c4:     	movzbl	0x32f233e(%rip), %eax   # 0x143634c09
1403428cb:     	movl	$0x190, %ecx            # imm = 0x190
1403428d0:     	movl	$0x8, %edx
1403428d5:     	callq	0x140613c10 <.text+0x612c10>
1403428da:     	testq	%rax, %rax
1403428dd:     	je	0x140342914 <.text+0x341914>
1403428df:     	movq	%rax, %rdi
1403428e2:     	leaq	-0x58(%rbp), %rdx
1403428e6:     	movl	$0x190, %r8d            # imm = 0x190
1403428ec:     	movq	%rax, %rcx
1403428ef:     	callq	0x142923c00 <.text+0x2922c00>
1403428f4:     	leaq	0x2680345(%rip), %r8    # 0x1429c2c40
1403428fb:     	movq	%rsi, %rcx
1403428fe:     	movq	%rdi, %rdx
140342901:     	callq	0x140ce4b10 <.text+0xce3b10>
140342906:     	movq	%rsi, %rax
140342909:     	addq	$0x1c0, %rsp            # imm = 0x1C0
140342910:     	popq	%rdi
140342911:     	popq	%rsi
140342912:     	popq	%rbp
140342913:     	retq
140342914:     	movl	$0x8, %ecx
140342919:     	movl	$0x190, %edx            # imm = 0x190
14034291e:     	callq	0x1428d9000 <.text+0x28d8000>
140342923:     	ud2
