
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140501640:     	pushq	%r15
140501642:     	pushq	%r14
140501644:     	pushq	%r13
140501646:     	pushq	%r12
140501648:     	pushq	%rsi
140501649:     	pushq	%rdi
14050164a:     	pushq	%rbp
14050164b:     	pushq	%rbx
14050164c:     	subq	$0xd8, %rsp
140501653:     	movq	%r9, %rsi
140501656:     	movq	%rcx, %rbx
140501659:     	movq	%rdx, 0x28(%rsp)
14050165e:     	movq	%r8, 0x30(%rsp)
140501663:     	leaq	0x10(%rcx), %rdi
140501667:     	movq	%rdi, %rcx
14050166a:     	callq	0x1406868b0 <.text+0x6858b0>
14050166f:     	movq	%rax, %r12
140501672:     	leaq	0x28(%rsp), %r15
140501677:     	movq	%r15, %rcx
14050167a:     	callq	0x140683c10 <.text+0x682c10>
14050167f:     	movq	%rax, 0x38(%rsp)
140501684:     	leaq	0x38(%rsp), %r14
140501689:     	movq	%r14, %rcx
14050168c:     	callq	0x1400885d0 <.text+0x875d0>
140501691:     	movq	%rax, 0x58(%rsp)
140501696:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
1405016a0:     	movq	%r13, 0x60(%rsp)
1405016a5:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
1405016af:     	movq	%rbp, 0x68(%rsp)
1405016b4:     	leaq	0x58(%rsp), %rcx
1405016b9:     	callq	0x1405bdf20 <.text+0x5bcf20>
1405016be:     	movq	%r12, 0x58(%rsp)
1405016c3:     	movq	%rax, 0x60(%rsp)
1405016c8:     	leaq	0x58(%rsp), %rcx
1405016cd:     	callq	0x1405bd910 <.text+0x5bc910>
1405016d2:     	movq	%rax, %r12
1405016d5:     	movq	%rdi, %rcx
1405016d8:     	movq	%rax, %rdx
1405016db:     	callq	0x140686450 <.text+0x685450>
1405016e0:     	testb	%al, %al
1405016e2:     	je	0x1405016fc <.text+0x5006fc>
1405016e4:     	leaq	0x58(%rsp), %rbx
1405016e9:     	movq	%rbx, %rcx
1405016ec:     	movq	%rdi, %rdx
1405016ef:     	movq	%r12, %r8
1405016f2:     	callq	0x140686530 <.text+0x685530>
1405016f7:     	jmp	0x14050183c <.text+0x50083c>
1405016fc:     	movq	0x312aca5(%rip), %rax   # 0x14362c3a8
140501703:     	movq	(%rax), %rax
140501706:     	testq	%rax, %rax
140501709:     	je	0x1405017e4 <.text+0x5007e4>
14050170f:     	movq	%rbx, 0x38(%rsp)
140501714:     	leaq	0x32185(%rip), %rax     # 0x1405338a0 <.text+0x5328a0>
14050171b:     	movq	%rax, 0x40(%rsp)
140501720:     	movq	%r15, 0x48(%rsp)
140501725:     	movq	%rax, 0x50(%rsp)
14050172a:     	leaq	0x24ec627(%rip), %rcx   # 0x1429edd58
140501731:     	callq	0x14067dd80 <.text+0x67cd80>
140501736:     	movups	(%rax), %xmm0
140501739:     	movl	0x10(%rax), %eax
14050173c:     	movq	$0x1, 0x88(%rsp)
140501748:     	leaq	0x24ec5d9(%rip), %rcx   # 0x1429edd28
14050174f:     	movq	%rcx, 0x90(%rsp)
140501757:     	movq	$0x14, 0x98(%rsp)
140501763:     	leaq	0x24ec52e(%rip), %rdx   # 0x1429edc98
14050176a:     	movq	%rdx, 0xa8(%rsp)
140501772:     	movq	$0x2, 0xb0(%rsp)
14050177e:     	movq	%r14, 0xb8(%rsp)
140501786:     	movq	$0x2, 0xc0(%rsp)
140501792:     	movq	$0x0, 0xc8(%rsp)
14050179e:     	movq	$0x0, 0x58(%rsp)
1405017a7:     	movq	%rcx, 0x60(%rsp)
1405017ac:     	movq	$0x14, 0x68(%rsp)
1405017b5:     	movq	$0x0, 0x70(%rsp)
1405017be:     	movups	%xmm0, 0x78(%rsp)
1405017c3:     	movl	$0x1, 0xa0(%rsp)
1405017ce:     	movl	%eax, 0xa4(%rsp)
1405017d5:     	leaq	0x27(%rsp), %rcx
1405017da:     	leaq	0x58(%rsp), %rdx
1405017df:     	callq	0x140d4a330 <.text+0xd49330>
1405017e4:     	movups	0x28(%rsp), %xmm0
1405017e9:     	movq	0x18(%rbx), %rbx
1405017ed:     	movups	%xmm0, 0x38(%rsp)
1405017f2:     	leaq	0x38(%rsp), %rcx
1405017f7:     	callq	0x140683c10 <.text+0x682c10>
1405017fc:     	movq	%rax, 0x58(%rsp)
140501801:     	movq	%r13, 0x60(%rsp)
140501806:     	movq	%rbp, 0x68(%rsp)
14050180b:     	leaq	0x58(%rsp), %rcx
140501810:     	callq	0x1405bdf20 <.text+0x5bcf20>
140501815:     	movq	%rbx, 0x58(%rsp)
14050181a:     	movq	%rax, 0x60(%rsp)
14050181f:     	leaq	0x58(%rsp), %rcx
140501824:     	callq	0x1405bd910 <.text+0x5bc910>
140501829:     	leaq	0x58(%rsp), %rbx
14050182e:     	movq	%rbx, %rcx
140501831:     	movq	%rdi, %rdx
140501834:     	movq	%rax, %r8
140501837:     	callq	0x1406861b0 <.text+0x6851b0>
14050183c:     	movss	0x24240c0(%rip), %xmm1  # 0x142925904
140501844:     	movq	%rbx, %rcx
140501847:     	movq	%rsi, %r8
14050184a:     	callq	0x1403d07f0 <.text+0x3cf7f0>
14050184f:     	nop
140501850:     	addq	$0xd8, %rsp
140501857:     	popq	%rbx
140501858:     	popq	%rbp
140501859:     	popq	%rdi
14050185a:     	popq	%rsi
14050185b:     	popq	%r12
14050185d:     	popq	%r13
14050185f:     	popq	%r14
140501861:     	popq	%r15
140501863:     	retq
