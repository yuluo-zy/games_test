
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141416520:     	pushq	%rbp
141416521:     	pushq	%r15
141416523:     	pushq	%r14
141416525:     	pushq	%r13
141416527:     	pushq	%r12
141416529:     	pushq	%rsi
14141652a:     	pushq	%rdi
14141652b:     	pushq	%rbx
14141652c:     	subq	$0x138, %rsp            # imm = 0x138
141416533:     	leaq	0x80(%rsp), %rbp
14141653b:     	movaps	%xmm6, 0xa0(%rbp)
141416542:     	movq	$-0x2, 0x98(%rbp)
14141654d:     	movq	%rdx, %rdi
141416550:     	movq	%rcx, 0x20(%rbp)
141416554:     	movq	0x20(%rdx), %r10
141416558:     	movq	0x28(%rdx), %rax
14141655c:     	cmpq	%r10, %rax
14141655f:     	jae	0x1414165a6 <.text+0x14155a6>
141416561:     	leaq	0x18(%rdi), %rbx
141416565:     	movq	0x18(%rdi), %rdx
141416569:     	incq	%rax
14141656c:     	movq	%r10, %r8
14141656f:     	negq	%r8
141416572:     	movabsq	$0x100002600, %r15      # imm = 0x100002600
14141657c:     	nopl	(%rax)
141416580:     	movzbl	-0x1(%rdx,%rax), %r9d
141416586:     	cmpq	$0x20, %r9
14141658a:     	ja	0x1414165bf <.text+0x14155bf>
14141658c:     	btq	%r9, %r15
141416590:     	jae	0x1414165bf <.text+0x14155bf>
141416592:     	movq	%rax, 0x28(%rdi)
141416596:     	leaq	(%r8,%rax), %r9
14141659a:     	incq	%r9
14141659d:     	incq	%rax
1414165a0:     	cmpq	$0x1, %r9
1414165a4:     	jne	0x141416580 <.text+0x1415580>
1414165a6:     	movq	$0x5, -0x30(%rbp)
1414165ae:     	leaq	-0x30(%rbp), %rdx
1414165b2:     	movq	%rdi, %rcx
1414165b5:     	callq	0x1428a0920 <.text+0x289f920>
1414165ba:     	jmp	0x1414173fd <.text+0x14163fd>
1414165bf:     	cmpl	$0x5b, %r9d
1414165c3:     	je	0x141416b1c <.text+0x1415b1c>
1414165c9:     	cmpl	$0x7b, %r9d
1414165cd:     	jne	0x141416eba <.text+0x1415eba>
1414165d3:     	decb	0x30(%rdi)
1414165d6:     	je	0x141417533 <.text+0x1416533>
1414165dc:     	movq	%rax, 0x28(%rdi)
1414165e0:     	movb	$0x1, %r13b
1414165e3:     	xorl	%esi, %esi
1414165e5:     	movabsq	$-0x7fffffffffffffff, %r8 # imm = 0x8000000000000001
1414165ef:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
1414165f9:     	leaq	-0x30(%rbp), %rcx
1414165fd:     	movq	%r14, 0x88(%rbp)
141416604:     	movq	%r14, 0x80(%rbp)
14141660b:     	movq	%r8, 0x70(%rbp)
14141660f:     	cmpq	%r10, %rax
141416612:     	jae	0x141416a87 <.text+0x1415a87>
141416618:     	movq	(%rbx), %rdx
14141661b:     	nopl	(%rax,%rax)
141416620:     	movzbl	(%rdx,%rax), %r8d
141416625:     	cmpq	$0x2c, %r8
141416629:     	ja	0x141416692 <.text+0x1415692>
14141662b:     	btq	%r8, %r15
14141662f:     	jae	0x141416642 <.text+0x1415642>
141416631:     	incq	%rax
141416634:     	movq	%rax, 0x28(%rdi)
141416638:     	cmpq	%rax, %r10
14141663b:     	jne	0x141416620 <.text+0x1415620>
14141663d:     	jmp	0x141416a87 <.text+0x1415a87>
141416642:     	cmpq	$0x2c, %r8
141416646:     	jne	0x141416692 <.text+0x1415692>
141416648:     	testb	$0x1, %r13b
14141664c:     	jne	0x14141705b <.text+0x141605b>
141416652:     	incq	%rax
141416655:     	movq	%rax, 0x28(%rdi)
141416659:     	cmpq	%r10, %rax
14141665c:     	jae	0x141416c36 <.text+0x1415c36>
141416662:     	nopw	%cs:(%rax,%rax)
141416670:     	movzbl	(%rdx,%rax), %r8d
141416675:     	cmpq	$0x20, %r8
141416679:     	ja	0x1414166a9 <.text+0x14156a9>
14141667b:     	btq	%r8, %r15
14141667f:     	jae	0x1414166a9 <.text+0x14156a9>
141416681:     	incq	%rax
141416684:     	movq	%rax, 0x28(%rdi)
141416688:     	cmpq	%rax, %r10
14141668b:     	jne	0x141416670 <.text+0x1415670>
14141668d:     	jmp	0x141416c36 <.text+0x1415c36>
141416692:     	cmpl	$0x7d, %r8d
141416696:     	je	0x141416bdd <.text+0x1415bdd>
14141669c:     	testb	$0x1, %r13b
1414166a0:     	je	0x141417540 <.text+0x1416540>
1414166a6:     	xorl	%r13d, %r13d
1414166a9:     	cmpb	$0x22, %r8b
1414166ad:     	jne	0x141417014 <.text+0x1416014>
1414166b3:     	incq	%rax
1414166b6:     	movq	%rax, 0x28(%rdi)
1414166ba:     	movq	$0x0, 0x10(%rdi)
1414166c2:     	movq	0x88(%rbp), %rax
1414166c9:     	movq	%rax, 0x68(%rbp)
1414166cd:     	movq	0x80(%rbp), %rax
1414166d4:     	movq	%rax, 0x60(%rbp)
1414166d8:     	movq	0x70(%rbp), %rax
1414166dc:     	movq	%rax, 0x58(%rbp)
1414166e0:     	movb	$0x1, 0x97(%rbp)
1414166e7:     	movq	%rbx, %rdx
1414166ea:     	movq	%rdi, %r8
1414166ed:     	callq	0x140b8a910 <.text+0xb89910>
1414166f2:     	cmpl	$0x2, -0x30(%rbp)
1414166f6:     	movq	-0x28(%rbp), %r12
1414166fa:     	je	0x141416ac3 <.text+0x1415ac3>
141416700:     	movq	-0x20(%rbp), %r8
141416704:     	leaq	0x30(%rbp), %rcx
141416708:     	movq	%r12, %rdx
14141670b:     	callq	0x140a14620 <.text+0xa13620>
141416710:     	testb	$0x1, 0x30(%rbp)
141416714:     	jne	0x141417199 <.text+0x1416199>
14141671a:     	movzbl	0x31(%rbp), %eax
14141671e:     	leaq	0x184734b(%rip), %rcx   # 0x142c5da70
141416725:     	movslq	(%rcx,%rax,4), %rax
141416729:     	addq	%rcx, %rax
14141672c:     	jmpq	*%rax
14141672e:     	movq	0x88(%rbp), %rcx
141416735:     	movq	%rcx, %rax
141416738:     	negq	%rax
14141673b:     	jno	0x1414175ec <.text+0x14165ec>
141416741:     	movq	0x20(%rdi), %r9
141416745:     	movq	0x28(%rdi), %rax
141416749:     	cmpq	%r9, %rax
14141674c:     	leaq	0x30(%rbp), %rcx
141416750:     	jae	0x141417430 <.text+0x1416430>
141416756:     	movq	(%rbx), %rdx
141416759:     	incq	%rax
14141675c:     	negq	%r9
14141675f:     	movzbl	-0x1(%rdx,%rax), %r8d
141416765:     	cmpq	$0x3a, %r8
141416769:     	ja	0x14141765b <.text+0x141665b>
14141676f:     	btq	%r8, %r15
141416773:     	jae	0x1414169bc <.text+0x14159bc>
141416779:     	movq	%rax, 0x28(%rdi)
14141677d:     	leaq	(%r9,%rax), %r8
141416781:     	incq	%r8
141416784:     	incq	%rax
141416787:     	cmpq	$0x1, %r8
14141678b:     	jne	0x14141675f <.text+0x141575f>
14141678d:     	jmp	0x141417430 <.text+0x1416430>
141416792:     	movq	0x88(%rbp), %rax
141416799:     	movq	%rax, 0x68(%rbp)
14141679d:     	movq	0x80(%rbp), %rax
1414167a4:     	movq	%rax, 0x60(%rbp)
1414167a8:     	movq	0x70(%rbp), %rax
1414167ac:     	movq	%rax, 0x58(%rbp)
1414167b0:     	movb	$0x1, 0x97(%rbp)
1414167b7:     	movq	%rdi, %rcx
1414167ba:     	callq	0x141409100 <.text+0x1408100>
1414167bf:     	movq	%rax, %r12
1414167c2:     	testq	%rax, %rax
1414167c5:     	jne	0x141416ac3 <.text+0x1415ac3>
1414167cb:     	movq	0x50(%rbp), %r12
1414167cf:     	jmp	0x141416a6e <.text+0x1415a6e>
1414167d4:     	movq	0x70(%rbp), %rax
1414167d8:     	movabsq	$-0x7fffffffffffffff, %r9 # imm = 0x8000000000000001
1414167e2:     	cmpq	%r9, %rax
1414167e5:     	jne	0x14141757e <.text+0x141657e>
1414167eb:     	movq	0x20(%rdi), %rcx
1414167ef:     	movq	0x28(%rdi), %rax
1414167f3:     	cmpq	%rcx, %rax
1414167f6:     	jae	0x1414171a2 <.text+0x14161a2>
1414167fc:     	movq	(%rbx), %rdx
1414167ff:     	incq	%rax
141416802:     	negq	%rcx
141416805:     	movzbl	-0x1(%rdx,%rax), %r8d
14141680b:     	cmpq	$0x3a, %r8
14141680f:     	ja	0x14141771b <.text+0x141671b>
141416815:     	btq	%r8, %r15
141416819:     	jae	0x1414168f5 <.text+0x14158f5>
14141681f:     	movq	%rax, 0x28(%rdi)
141416823:     	leaq	(%rcx,%rax), %r8
141416827:     	incq	%r8
14141682a:     	incq	%rax
14141682d:     	cmpq	$0x1, %r8
141416831:     	jne	0x141416805 <.text+0x1415805>
141416833:     	jmp	0x1414171a2 <.text+0x14161a2>
141416838:     	testl	%esi, %esi
14141683a:     	jne	0x141417620 <.text+0x1416620>
141416840:     	movq	0x20(%rdi), %r9
141416844:     	movq	0x28(%rdi), %rax
141416848:     	cmpq	%r9, %rax
14141684b:     	leaq	0x30(%rbp), %rcx
14141684f:     	jae	0x1414174d5 <.text+0x14164d5>
141416855:     	movq	(%rbx), %rdx
141416858:     	incq	%rax
14141685b:     	negq	%r9
14141685e:     	movzbl	-0x1(%rdx,%rax), %r8d
141416864:     	cmpq	$0x3a, %r8
141416868:     	ja	0x1414176dd <.text+0x14166dd>
14141686e:     	btq	%r8, %r15
141416872:     	jae	0x141416964 <.text+0x1415964>
141416878:     	movq	%rax, 0x28(%rdi)
14141687c:     	leaq	(%r9,%rax), %r8
141416880:     	incq	%r8
141416883:     	incq	%rax
141416886:     	cmpq	$0x1, %r8
14141688a:     	jne	0x14141685e <.text+0x141585e>
14141688c:     	jmp	0x1414174d5 <.text+0x14164d5>
141416891:     	movq	0x80(%rbp), %rcx
141416898:     	movq	%rcx, %rax
14141689b:     	negq	%rax
14141689e:     	jno	0x1414175b8 <.text+0x14165b8>
1414168a4:     	movq	0x20(%rdi), %r9
1414168a8:     	movq	0x28(%rdi), %rax
1414168ac:     	cmpq	%r9, %rax
1414168af:     	leaq	0x30(%rbp), %rcx
1414168b3:     	jae	0x14141747e <.text+0x141647e>
1414168b9:     	movq	(%rbx), %rdx
1414168bc:     	incq	%rax
1414168bf:     	negq	%r9
1414168c2:     	movzbl	-0x1(%rdx,%rax), %r8d
1414168c8:     	cmpq	$0x3a, %r8
1414168cc:     	ja	0x14141769c <.text+0x141669c>
1414168d2:     	btq	%r8, %r15
1414168d6:     	jae	0x141416a12 <.text+0x1415a12>
1414168dc:     	movq	%rax, 0x28(%rdi)
1414168e0:     	leaq	(%r9,%rax), %r8
1414168e4:     	incq	%r8
1414168e7:     	incq	%rax
1414168ea:     	cmpq	$0x1, %r8
1414168ee:     	jne	0x1414168c2 <.text+0x14158c2>
1414168f0:     	jmp	0x14141747e <.text+0x141647e>
1414168f5:     	cmpq	$0x3a, %r8
1414168f9:     	jne	0x14141771b <.text+0x141671b>
1414168ff:     	movq	%rax, 0x28(%rdi)
141416903:     	movq	0x88(%rbp), %rax
14141690a:     	movq	%rax, 0x68(%rbp)
14141690e:     	movq	0x80(%rbp), %rax
141416915:     	movq	%rax, 0x60(%rbp)
141416919:     	movq	%r9, 0x58(%rbp)
14141691d:     	movb	$0x1, 0x97(%rbp)
141416924:     	leaq	0x30(%rbp), %rcx
141416928:     	movq	%rdi, %rdx
14141692b:     	callq	0x140a4ced0 <.text+0xa4bed0>
141416930:     	movq	0x30(%rbp), %rcx
141416934:     	movq	0x38(%rbp), %rax
141416938:     	movq	%rcx, 0x70(%rbp)
14141693c:     	movabsq	$-0x7fffffffffffffff, %rdx # imm = 0x8000000000000001
141416946:     	cmpq	%rdx, %rcx
141416949:     	je	0x1414171e1 <.text+0x14161e1>
14141694f:     	movq	0x40(%rbp), %rcx
141416953:     	movq	%rcx, -0x48(%rbp)
141416957:     	movq	0x50(%rbp), %r12
14141695b:     	movq	%rax, 0x48(%rbp)
14141695f:     	jmp	0x141416a6e <.text+0x1415a6e>
141416964:     	cmpq	$0x3a, %r8
141416968:     	jne	0x1414176dd <.text+0x14166dd>
14141696e:     	movq	%rax, 0x28(%rdi)
141416972:     	movq	0x88(%rbp), %rax
141416979:     	movq	%rax, 0x68(%rbp)
14141697d:     	movq	0x80(%rbp), %rax
141416984:     	movq	%rax, 0x60(%rbp)
141416988:     	movq	0x70(%rbp), %rax
14141698c:     	movq	%rax, 0x58(%rbp)
141416990:     	movb	$0x1, 0x97(%rbp)
141416997:     	movq	%rdi, %rdx
14141699a:     	callq	0x14140cfd0 <.text+0x140bfd0>
14141699f:     	testb	$0x1, 0x30(%rbp)
1414169a3:     	jne	0x141417199 <.text+0x1416199>
1414169a9:     	movss	0x34(%rbp), %xmm6
1414169ae:     	movl	$0x1, %esi
1414169b3:     	movq	0x50(%rbp), %r12
1414169b7:     	jmp	0x141416a6e <.text+0x1415a6e>
1414169bc:     	cmpq	$0x3a, %r8
1414169c0:     	jne	0x14141765b <.text+0x141665b>
1414169c6:     	movq	%rax, 0x28(%rdi)
1414169ca:     	movq	%r14, 0x68(%rbp)
1414169ce:     	movq	0x80(%rbp), %rax
1414169d5:     	movq	%rax, 0x60(%rbp)
1414169d9:     	movq	0x70(%rbp), %rax
1414169dd:     	movq	%rax, 0x58(%rbp)
1414169e1:     	movb	$0x1, 0x97(%rbp)
1414169e8:     	movq	%rdi, %rdx
1414169eb:     	callq	0x141413b20 <.text+0x1412b20>
1414169f0:     	movq	0x30(%rbp), %rax
1414169f4:     	movq	0x38(%rbp), %r12
1414169f8:     	movq	%rax, 0x88(%rbp)
1414169ff:     	negq	%rax
141416a02:     	jo	0x14141746f <.text+0x141646f>
141416a08:     	movq	0x40(%rbp), %rax
141416a0c:     	movq	%rax, -0x40(%rbp)
141416a10:     	jmp	0x141416a6e <.text+0x1415a6e>
141416a12:     	cmpq	$0x3a, %r8
141416a16:     	jne	0x14141769c <.text+0x141669c>
141416a1c:     	movq	%rax, 0x28(%rdi)
141416a20:     	movq	0x88(%rbp), %rax
141416a27:     	movq	%rax, 0x68(%rbp)
141416a2b:     	movq	%r14, 0x60(%rbp)
141416a2f:     	movq	0x70(%rbp), %rax
141416a33:     	movq	%rax, 0x58(%rbp)
141416a37:     	movb	$0x1, 0x97(%rbp)
141416a3e:     	movq	%rdi, %rdx
141416a41:     	callq	0x141412a90 <.text+0x1411a90>
141416a46:     	movq	0x30(%rbp), %rcx
141416a4a:     	movq	0x38(%rbp), %rax
141416a4e:     	movq	%rcx, 0x80(%rbp)
141416a55:     	negq	%rcx
141416a58:     	jo	0x1414174ba <.text+0x14164ba>
141416a5e:     	movq	0x40(%rbp), %rcx
141416a62:     	movq	%rcx, 0x28(%rbp)
141416a66:     	movq	0x50(%rbp), %r12
141416a6a:     	movq	%rax, 0x78(%rbp)
141416a6e:     	movq	0x20(%rdi), %r10
141416a72:     	movq	0x28(%rdi), %rax
141416a76:     	movq	%r12, 0x50(%rbp)
141416a7a:     	leaq	-0x30(%rbp), %rcx
141416a7e:     	cmpq	%r10, %rax
141416a81:     	jb	0x141416618 <.text+0x1415618>
141416a87:     	movq	$0x3, -0x30(%rbp)
141416a8f:     	movq	0x88(%rbp), %rax
141416a96:     	movq	%rax, 0x68(%rbp)
141416a9a:     	movq	0x80(%rbp), %rax
141416aa1:     	movq	%rax, 0x60(%rbp)
141416aa5:     	movq	0x70(%rbp), %rax
141416aa9:     	movq	%rax, 0x58(%rbp)
141416aad:     	movb	$0x1, 0x97(%rbp)
141416ab4:     	leaq	-0x30(%rbp), %rdx
141416ab8:     	movq	%rdi, %rcx
141416abb:     	callq	0x1428a0920 <.text+0x289f920>
141416ac0:     	movq	%rax, %r12
141416ac3:     	xorl	%ebx, %ebx
141416ac5:     	movq	0x78(%rbp), %r13
141416ac9:     	movq	0x88(%rbp), %r15
141416ad0:     	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
141416ada:     	movq	0x70(%rbp), %rdx
141416ade:     	cmpq	%rax, %rdx
141416ae1:     	jl	0x141416f4d <.text+0x1415f4d>
141416ae7:     	testq	%rdx, %rdx
141416aea:     	je	0x141416f4d <.text+0x1415f4d>
141416af0:     	movl	$0x1, %r8d
141416af6:     	movq	0x48(%rbp), %rcx
141416afa:     	callq	0x140613c20 <.text+0x612c20>
141416aff:     	movq	0x80(%rbp), %rax
141416b06:     	leaq	(,%rax,2), %rax
141416b0e:     	testq	%rax, %rax
141416b11:     	je	0x141416f65 <.text+0x1415f65>
141416b17:     	jmp	0x141417209 <.text+0x1416209>
141416b1c:     	decb	0x30(%rdi)
141416b1f:     	je	0x141417533 <.text+0x1416533>
141416b25:     	movq	%rax, 0x28(%rdi)
141416b29:     	movq	%rdi, 0x30(%rbp)
141416b2d:     	movb	$0x1, 0x38(%rbp)
141416b31:     	leaq	-0x30(%rbp), %rcx
141416b35:     	leaq	0x30(%rbp), %rdx
141416b39:     	callq	0x14140bee0 <.text+0x140aee0>
141416b3e:     	movq	-0x30(%rbp), %rax
141416b42:     	movq	-0x28(%rbp), %r12
141416b46:     	movabsq	$-0x7fffffffffffffff, %rbx # imm = 0x8000000000000001
141416b50:     	cmpq	%rbx, %rax
141416b53:     	jne	0x141416b64 <.text+0x1415b64>
141416b55:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416b5f:     	jmp	0x141416d2e <.text+0x1415d2e>
141416b64:     	movq	%rax, 0x88(%rbp)
141416b6b:     	negq	%rax
141416b6e:     	jo	0x141416f12 <.text+0x1415f12>
141416b74:     	movq	%r12, 0x50(%rbp)
141416b78:     	movq	-0x20(%rbp), %rax
141416b7c:     	movq	%rax, -0x40(%rbp)
141416b80:     	leaq	-0x30(%rbp), %rcx
141416b84:     	leaq	0x30(%rbp), %rdx
141416b88:     	callq	0x14140ad50 <.text+0x1409d50>
141416b8d:     	movq	-0x30(%rbp), %rax
141416b91:     	movq	-0x28(%rbp), %r13
141416b95:     	cmpq	%rbx, %rax
141416b98:     	jne	0x141416ba6 <.text+0x1415ba6>
141416b9a:     	movq	0x88(%rbp), %rdx
141416ba1:     	jmp	0x141416d0d <.text+0x1415d0d>
141416ba6:     	movq	%rax, 0x80(%rbp)
141416bad:     	negq	%rax
141416bb0:     	jno	0x141416c74 <.text+0x1415c74>
141416bb6:     	leaq	0x184680b(%rip), %rdx   # 0x142c5d3c8
141416bbd:     	leaq	0x184532c(%rip), %r8    # 0x142c5bef0
141416bc4:     	movl	$0x1, %ecx
141416bc9:     	callq	0x1428f9d20 <.text+0x28f8d20>
141416bce:     	movq	%rax, %r13
141416bd1:     	movq	0x88(%rbp), %rdx
141416bd8:     	jmp	0x141416d0d <.text+0x1415d0d>
141416bdd:     	movq	0x88(%rbp), %r14
141416be4:     	movq	%r14, %rax
141416be7:     	negq	%rax
141416bea:     	jno	0x141416e5c <.text+0x1415e5c>
141416bf0:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141416bfa:     	movq	%rax, %r15
141416bfd:     	movq	%rax, 0x68(%rbp)
141416c01:     	movq	0x80(%rbp), %rax
141416c08:     	movq	%rax, 0x60(%rbp)
141416c0c:     	movq	0x70(%rbp), %rax
141416c10:     	movq	%rax, 0x58(%rbp)
141416c14:     	movb	$0x1, 0x97(%rbp)
141416c1b:     	leaq	0x18467b6(%rip), %rcx   # 0x142c5d3d8
141416c22:     	movl	$0x6, %edx
141416c27:     	callq	0x1428f9bc0 <.text+0x28f8bc0>
141416c2c:     	movq	%rax, %r12
141416c2f:     	xorl	%ebx, %ebx
141416c31:     	jmp	0x141416f32 <.text+0x1415f32>
141416c36:     	movq	$0x5, -0x30(%rbp)
141416c3e:     	movq	0x88(%rbp), %rax
141416c45:     	movq	%rax, 0x68(%rbp)
141416c49:     	movq	0x80(%rbp), %rax
141416c50:     	movq	%rax, 0x60(%rbp)
141416c54:     	movq	0x70(%rbp), %rax
141416c58:     	movq	%rax, 0x58(%rbp)
141416c5c:     	movb	$0x1, 0x97(%rbp)
141416c63:     	leaq	-0x30(%rbp), %rdx
141416c67:     	movq	%rdi, %rcx
141416c6a:     	callq	0x1428a0920 <.text+0x289f920>
141416c6f:     	jmp	0x141416ac0 <.text+0x1415ac0>
141416c74:     	movq	0x30(%rbp), %rcx
141416c78:     	movq	0x20(%rcx), %r8
141416c7c:     	movq	0x28(%rcx), %rax
141416c80:     	cmpq	%r8, %rax
141416c83:     	movq	%r13, 0x78(%rbp)
141416c87:     	jae	0x141416cce <.text+0x1415cce>
141416c89:     	movq	-0x20(%rbp), %r12
141416c8d:     	movq	0x18(%rcx), %rdx
141416c91:     	incq	%rax
141416c94:     	movq	%r8, %r10
141416c97:     	negq	%r10
141416c9a:     	nopw	(%rax,%rax)
141416ca0:     	movzbl	-0x1(%rdx,%rax), %r9d
141416ca6:     	cmpq	$0x2c, %r9
141416caa:     	ja	0x141416ff4 <.text+0x1415ff4>
141416cb0:     	btq	%r9, %r15
141416cb4:     	jae	0x141416f87 <.text+0x1415f87>
141416cba:     	movq	%rax, 0x28(%rcx)
141416cbe:     	leaq	(%r10,%rax), %r9
141416cc2:     	incq	%r9
141416cc5:     	incq	%rax
141416cc8:     	cmpq	$0x1, %r9
141416ccc:     	jne	0x141416ca0 <.text+0x1415ca0>
141416cce:     	movq	$0x2, -0x30(%rbp)
141416cd6:     	leaq	-0x30(%rbp), %rdx
141416cda:     	callq	0x1428a0920 <.text+0x289f920>
141416cdf:     	movq	%rax, %r13
141416ce2:     	movq	0x78(%rbp), %rcx
141416ce6:     	movq	0x88(%rbp), %rdx
141416ced:     	movq	0x80(%rbp), %rax
141416cf4:     	testq	%rax, %rax
141416cf7:     	je	0x141416d0d <.text+0x1415d0d>
141416cf9:     	movl	$0x1, %r8d
141416cff:     	movq	%rdx, %rbx
141416d02:     	movq	%rax, %rdx
141416d05:     	callq	0x140613c20 <.text+0x612c20>
141416d0a:     	movq	%rbx, %rdx
141416d0d:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416d17:     	testq	%rdx, %rdx
141416d1a:     	je	0x141416d2b <.text+0x1415d2b>
141416d1c:     	movl	$0x1, %r8d
141416d22:     	movq	0x50(%rbp), %rcx
141416d26:     	callq	0x140613c20 <.text+0x612c20>
141416d2b:     	movq	%r13, %r12
141416d2e:     	incb	0x30(%rdi)
141416d31:     	movq	%r14, -0x30(%rbp)
141416d35:     	movq	%r12, -0x28(%rbp)
141416d39:     	movq	%rax, %rbx
141416d3c:     	movq	%rax, -0x20(%rbp)
141416d40:     	movq	%r15, -0x18(%rbp)
141416d44:     	movq	%r13, 0x78(%rbp)
141416d48:     	movq	%r13, -0x10(%rbp)
141416d4c:     	movq	%rcx, 0x28(%rbp)
141416d50:     	movq	%rcx, -0x8(%rbp)
141416d54:     	movq	%r8, %r13
141416d57:     	movq	%r8, (%rbp)
141416d5b:     	movq	%rdx, 0x48(%rbp)
141416d5f:     	movq	%rdx, 0x8(%rbp)
141416d63:     	movq	%r9, 0x10(%rbp)
141416d67:     	movss	%xmm0, 0x18(%rbp)
141416d6c:     	movq	%rdi, %rcx
141416d6f:     	callq	0x1400584f0 <.text+0x574f0>
141416d74:     	leaq	0x10(%rbp), %rcx
141416d78:     	movups	(%rcx), %xmm0
141416d7b:     	movaps	%xmm0, 0x30(%rbp)
141416d7f:     	movq	%r14, %rcx
141416d82:     	negq	%rcx
141416d85:     	jno	0x141416db8 <.text+0x1415db8>
141416d87:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416d91:     	testq	%rax, %rax
141416d94:     	je	0x141417386 <.text+0x1416386>
141416d9a:     	movq	(%rax), %rcx
141416d9d:     	cmpq	$0x1, %rcx
141416da1:     	movq	%rax, 0x80(%rbp)
141416da8:     	je	0x141416e25 <.text+0x1415e25>
141416daa:     	testq	%rcx, %rcx
141416dad:     	je	0x1414172e7 <.text+0x14162e7>
141416db3:     	jmp	0x1414173d3 <.text+0x14163d3>
141416db8:     	testq	%rax, %rax
141416dbb:     	je	0x14141737e <.text+0x141637e>
141416dc1:     	testq	%r14, %r14
141416dc4:     	je	0x141416ddd <.text+0x1415ddd>
141416dc6:     	movl	$0x1, %r8d
141416dcc:     	movq	%rax, %rsi
141416dcf:     	movq	%r12, %rcx
141416dd2:     	movq	%r14, %rdx
141416dd5:     	callq	0x140613c20 <.text+0x612c20>
141416dda:     	movq	%rsi, %rax
141416ddd:     	testq	%r15, %r15
141416de0:     	je	0x141416dfa <.text+0x1415dfa>
141416de2:     	movl	$0x1, %r8d
141416de8:     	movq	%rax, %rsi
141416deb:     	movq	0x78(%rbp), %rcx
141416def:     	movq	%r15, %rdx
141416df2:     	callq	0x140613c20 <.text+0x612c20>
141416df7:     	movq	%rsi, %rax
141416dfa:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416e04:     	leaq	(,%r13,2), %rcx
141416e0c:     	testq	%rcx, %rcx
141416e0f:     	jne	0x141416e36 <.text+0x1415e36>
141416e11:     	movq	%rax, %r12
141416e14:     	movq	%r14, %rax
141416e17:     	negq	%rax
141416e1a:     	jno	0x14141738e <.text+0x141638e>
141416e20:     	jmp	0x1414173f2 <.text+0x14163f2>
141416e25:     	movq	%rax, %rcx
141416e28:     	addq	$0x8, %rcx
141416e2c:     	callq	0x1405e7a00 <.text+0x5e6a00>
141416e31:     	jmp	0x1414173d3 <.text+0x14163d3>
141416e36:     	movl	$0x1, %r8d
141416e3c:     	movq	%rax, %r12
141416e3f:     	movq	0x48(%rbp), %rcx
141416e43:     	movq	%r13, %rdx
141416e46:     	callq	0x140613c20 <.text+0x612c20>
141416e4b:     	movq	%r14, %rax
141416e4e:     	negq	%rax
141416e51:     	jno	0x14141738e <.text+0x141638e>
141416e57:     	jmp	0x1414173f2 <.text+0x14163f2>
141416e5c:     	movq	0x80(%rbp), %r15
141416e63:     	movq	%r15, %rax
141416e66:     	negq	%rax
141416e69:     	movq	-0x40(%rbp), %rbx
141416e6d:     	jno	0x141416ed5 <.text+0x1415ed5>
141416e6f:     	leaq	0x1846568(%rip), %rcx   # 0x142c5d3de
141416e76:     	movl	$0xa, %edx
141416e7b:     	callq	0x1428f9bc0 <.text+0x28f8bc0>
141416e80:     	movq	%rax, %r12
141416e83:     	movb	$0x1, %bl
141416e85:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141416e8f:     	movq	%rax, 0x80(%rbp)
141416e96:     	movq	0x88(%rbp), %r15
141416e9d:     	testq	%r15, %r15
141416ea0:     	je	0x141416f2f <.text+0x1415f2f>
141416ea6:     	movl	$0x1, %r8d
141416eac:     	movq	0x50(%rbp), %rcx
141416eb0:     	movq	%r15, %rdx
141416eb3:     	callq	0x140613c20 <.text+0x612c20>
141416eb8:     	jmp	0x141416f32 <.text+0x1415f32>
141416eba:     	leaq	0x184550f(%rip), %r8    # 0x142c5c3d0
141416ec1:     	leaq	-0x31(%rbp), %rdx
141416ec5:     	movq	%rdi, %rcx
141416ec8:     	callq	0x1428f8c00 <.text+0x28f7c00>
141416ecd:     	movq	%rax, %r12
141416ed0:     	jmp	0x1414173f2 <.text+0x14163f2>
141416ed5:     	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
141416edf:     	movq	0x70(%rbp), %rcx
141416ee3:     	cmpq	%rax, %rcx
141416ee6:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141416ef0:     	cmovneq	%rcx, %rax
141416ef4:     	movq	%rax, %rcx
141416ef7:     	testb	$0x1, %sil
141416efb:     	movq	0x50(%rbp), %r12
141416eff:     	jne	0x141416f09 <.text+0x1415f09>
141416f01:     	movss	0x150e9fb(%rip), %xmm6  # 0x142925904
141416f09:     	movq	0x78(%rbp), %r13
141416f0d:     	jmp	0x14141725a <.text+0x141625a>
141416f12:     	leaq	0x18464af(%rip), %rdx   # 0x142c5d3c8
141416f19:     	leaq	0x1844fd0(%rip), %r8    # 0x142c5bef0
141416f20:     	xorl	%ecx, %ecx
141416f22:     	callq	0x1428f9d20 <.text+0x28f8d20>
141416f27:     	movq	%rax, %r12
141416f2a:     	jmp	0x141416b55 <.text+0x1415b55>
141416f2f:     	xorl	%r15d, %r15d
141416f32:     	movq	0x78(%rbp), %r13
141416f36:     	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
141416f40:     	movq	0x70(%rbp), %rdx
141416f44:     	cmpq	%rax, %rdx
141416f47:     	jge	0x141416ae7 <.text+0x1415ae7>
141416f4d:     	movq	0x80(%rbp), %rax
141416f54:     	leaq	(,%rax,2), %rax
141416f5c:     	testq	%rax, %rax
141416f5f:     	jne	0x141417209 <.text+0x1416209>
141416f65:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141416f6f:     	testb	%bl, %bl
141416f71:     	je	0x141417230 <.text+0x1416230>
141416f77:     	movq	0x80(%rbp), %r15
141416f7e:     	movq	-0x40(%rbp), %rbx
141416f82:     	jmp	0x14141725a <.text+0x141625a>
141416f87:     	cmpq	$0x2c, %r9
141416f8b:     	jne	0x141416ff4 <.text+0x1415ff4>
141416f8d:     	movzbl	0x38(%rbp), %r10d
141416f92:     	testb	%r10b, %r10b
141416f95:     	jne	0x14141709e <.text+0x141609e>
141416f9b:     	movq	%rax, 0x28(%rcx)
141416f9f:     	cmpq	%r8, %rax
141416fa2:     	jae	0x141416fde <.text+0x1415fde>
141416fa4:     	incq	%rax
141416fa7:     	movl	$0x1, %r10d
141416fad:     	subq	%r8, %r10
141416fb0:     	movzbl	-0x1(%rdx,%rax), %r9d
141416fb6:     	cmpq	$0x20, %r9
141416fba:     	ja	0x1414170ac <.text+0x14160ac>
141416fc0:     	btq	%r9, %r15
141416fc4:     	jae	0x1414170ac <.text+0x14160ac>
141416fca:     	movq	%rax, 0x28(%rcx)
141416fce:     	leaq	(%r10,%rax), %r8
141416fd2:     	incq	%r8
141416fd5:     	incq	%rax
141416fd8:     	cmpq	$0x2, %r8
141416fdc:     	jne	0x141416fb0 <.text+0x1415fb0>
141416fde:     	movq	$0x5, -0x30(%rbp)
141416fe6:     	leaq	-0x30(%rbp), %rdx
141416fea:     	callq	0x1428a0920 <.text+0x289f920>
141416fef:     	jmp	0x141416cdf <.text+0x1415cdf>
141416ff4:     	cmpl	$0x5d, %r9d
141416ff8:     	jne	0x141417099 <.text+0x1416099>
141416ffe:     	movb	$0x1, %r14b
141417001:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141700b:     	movq	%rax, 0x28(%rbp)
14141700f:     	jmp	0x141417109 <.text+0x1416109>
141417014:     	movzbl	%r8b, %eax
141417018:     	cmpl	$0x7d, %eax
14141701b:     	jne	0x14141705b <.text+0x141605b>
14141701d:     	movq	$0x12, -0x30(%rbp)
141417025:     	movq	0x88(%rbp), %rax
14141702c:     	movq	%rax, 0x68(%rbp)
141417030:     	movq	0x80(%rbp), %rax
141417037:     	movq	%rax, 0x60(%rbp)
14141703b:     	movq	0x70(%rbp), %rax
14141703f:     	movq	%rax, 0x58(%rbp)
141417043:     	movb	$0x1, 0x97(%rbp)
14141704a:     	leaq	-0x30(%rbp), %rdx
14141704e:     	movq	%rdi, %rcx
141417051:     	callq	0x1428a0920 <.text+0x289f920>
141417056:     	jmp	0x141416ac0 <.text+0x1415ac0>
14141705b:     	movq	$0x10, -0x30(%rbp)
141417063:     	movq	0x88(%rbp), %rax
14141706a:     	movq	%rax, 0x68(%rbp)
14141706e:     	movq	0x80(%rbp), %rax
141417075:     	movq	%rax, 0x60(%rbp)
141417079:     	movq	0x70(%rbp), %rax
14141707d:     	movq	%rax, 0x58(%rbp)
141417081:     	movb	$0x1, 0x97(%rbp)
141417088:     	leaq	-0x30(%rbp), %rdx
14141708c:     	movq	%rdi, %rcx
14141708f:     	callq	0x1428a0920 <.text+0x289f920>
141417094:     	jmp	0x141416ac0 <.text+0x1415ac0>
141417099:     	movzbl	0x38(%rbp), %r10d
14141709e:     	testb	$0x1, %r10b
1414170a2:     	je	0x14141777b <.text+0x141677b>
1414170a8:     	movb	$0x0, 0x38(%rbp)
1414170ac:     	movq	%rcx, %rdx
1414170af:     	cmpb	$0x5d, %r9b
1414170b3:     	je	0x14141775f <.text+0x141675f>
1414170b9:     	leaq	-0x30(%rbp), %rcx
1414170bd:     	callq	0x140a4ced0 <.text+0xa4bed0>
1414170c2:     	movq	-0x30(%rbp), %rcx
1414170c6:     	movq	-0x28(%rbp), %rax
1414170ca:     	movq	%rax, 0x48(%rbp)
1414170ce:     	cmpq	%rbx, %rcx
1414170d1:     	jne	0x1414170f7 <.text+0x14160f7>
1414170d3:     	movq	0x78(%rbp), %rcx
1414170d7:     	movq	0x88(%rbp), %rdx
1414170de:     	movq	0x80(%rbp), %rax
1414170e5:     	movq	0x48(%rbp), %r13
1414170e9:     	testq	%rax, %rax
1414170ec:     	jne	0x141416cf9 <.text+0x1415cf9>
1414170f2:     	jmp	0x141416d0d <.text+0x1415d0d>
1414170f7:     	movq	%rcx, %rax
1414170fa:     	movq	-0x20(%rbp), %rsi
1414170fe:     	movq	%rax, 0x28(%rbp)
141417102:     	negq	%rax
141417105:     	seto	%r14b
141417109:     	leaq	-0x30(%rbp), %rcx
14141710d:     	leaq	0x30(%rbp), %rdx
141417111:     	callq	0x14140c050 <.text+0x140b050>
141417116:     	testb	$0x1, -0x30(%rbp)
14141711a:     	je	0x141417170 <.text+0x1416170>
14141711c:     	movq	-0x28(%rbp), %rbx
141417120:     	movq	0x28(%rbp), %r9
141417124:     	testq	%r9, %r9
141417127:     	sete	%al
14141712a:     	orb	%al, %r14b
14141712d:     	movq	0x78(%rbp), %rcx
141417131:     	movq	0x88(%rbp), %rdx
141417138:     	jne	0x141417158 <.text+0x1416158>
14141713a:     	movl	$0x1, %r8d
141417140:     	movq	%rcx, %r14
141417143:     	movq	0x48(%rbp), %rcx
141417147:     	movq	%rdx, %r15
14141714a:     	movq	%r9, %rdx
14141714d:     	callq	0x140613c20 <.text+0x612c20>
141417152:     	movq	%r15, %rdx
141417155:     	movq	%r14, %rcx
141417158:     	movq	%rbx, %r13
14141715b:     	movq	0x80(%rbp), %rax
141417162:     	testq	%rax, %rax
141417165:     	jne	0x141416cf9 <.text+0x1415cf9>
14141716b:     	jmp	0x141416d0d <.text+0x1415d0d>
141417170:     	movq	%r12, %rcx
141417173:     	testb	$0x1, -0x2c(%rbp)
141417177:     	movq	0x78(%rbp), %r13
14141717b:     	movq	0x88(%rbp), %r14
141417182:     	movq	0x80(%rbp), %r15
141417189:     	je	0x141417513 <.text+0x1416513>
14141718f:     	movss	-0x28(%rbp), %xmm0
141417194:     	jmp	0x14141751b <.text+0x141651b>
141417199:     	movq	0x38(%rbp), %r12
14141719d:     	jmp	0x141416ac3 <.text+0x1415ac3>
1414171a2:     	movq	$0x3, -0x30(%rbp)
1414171aa:     	movq	0x88(%rbp), %rax
1414171b1:     	movq	%rax, 0x68(%rbp)
1414171b5:     	movq	0x80(%rbp), %rax
1414171bc:     	movq	%rax, 0x60(%rbp)
1414171c0:     	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1414171ca:     	movq	%rax, 0x58(%rbp)
1414171ce:     	movb	$0x1, 0x97(%rbp)
1414171d5:     	leaq	-0x30(%rbp), %rdx
1414171d9:     	movq	%rdi, %rcx
1414171dc:     	callq	0x1428a0920 <.text+0x289f920>
1414171e1:     	xorl	%ebx, %ebx
1414171e3:     	movq	%rax, %r12
1414171e6:     	movq	0x78(%rbp), %r13
1414171ea:     	movq	0x88(%rbp), %r15
1414171f1:     	movq	0x80(%rbp), %rax
1414171f8:     	leaq	(,%rax,2), %rax
141417200:     	testq	%rax, %rax
141417203:     	je	0x141416f65 <.text+0x1415f65>
141417209:     	movl	$0x1, %r8d
14141720f:     	movq	%r13, %rcx
141417212:     	movq	0x80(%rbp), %rdx
141417219:     	callq	0x140613c20 <.text+0x612c20>
14141721e:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141417228:     	testb	%bl, %bl
14141722a:     	jne	0x141416f77 <.text+0x1415f77>
141417230:     	leaq	(,%r15,2), %rax
141417238:     	testq	%rax, %rax
14141723b:     	movq	-0x40(%rbp), %rbx
14141723f:     	je	0x141417253 <.text+0x1416253>
141417241:     	movl	$0x1, %r8d
141417247:     	movq	0x50(%rbp), %rcx
14141724b:     	movq	%r15, %rdx
14141724e:     	callq	0x140613c20 <.text+0x612c20>
141417253:     	movq	0x80(%rbp), %r15
14141725a:     	incb	0x30(%rdi)
14141725d:     	movq	%r14, -0x30(%rbp)
141417261:     	movq	%r12, -0x28(%rbp)
141417265:     	movq	%rbx, -0x20(%rbp)
141417269:     	movq	%r15, -0x18(%rbp)
14141726d:     	movq	%r13, 0x78(%rbp)
141417271:     	movq	%r13, -0x10(%rbp)
141417275:     	movq	0x28(%rbp), %rax
141417279:     	movq	%rax, -0x8(%rbp)
14141727d:     	movq	%rcx, %r13
141417280:     	movq	%rcx, (%rbp)
141417284:     	movq	0x48(%rbp), %rax
141417288:     	movq	%rax, 0x8(%rbp)
14141728c:     	movq	-0x48(%rbp), %rax
141417290:     	movq	%rax, 0x10(%rbp)
141417294:     	movss	%xmm6, 0x18(%rbp)
141417299:     	movq	%rdi, %rcx
14141729c:     	callq	0x140058450 <.text+0x57450>
1414172a1:     	movq	%rax, %rsi
1414172a4:     	leaq	0x10(%rbp), %rax
1414172a8:     	movups	(%rax), %xmm0
1414172ab:     	movaps	%xmm0, 0x30(%rbp)
1414172af:     	movq	%r14, %rax
1414172b2:     	negq	%rax
1414172b5:     	jno	0x141417316 <.text+0x1416316>
1414172b7:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
1414172c1:     	testq	%rsi, %rsi
1414172c4:     	je	0x141417386 <.text+0x1416386>
1414172ca:     	movq	(%rsi), %rax
1414172cd:     	cmpq	$0x1, %rax
1414172d1:     	movq	%rsi, 0x80(%rbp)
1414172d8:     	je	0x1414173c7 <.text+0x14163c7>
1414172de:     	testq	%rax, %rax
1414172e1:     	jne	0x1414173d3 <.text+0x14163d3>
1414172e7:     	movq	0x80(%rbp), %rax
1414172ee:     	movq	0x10(%rax), %rdx
1414172f2:     	testq	%rdx, %rdx
1414172f5:     	je	0x1414173d3 <.text+0x14163d3>
1414172fb:     	movq	0x80(%rbp), %rax
141417302:     	movq	0x8(%rax), %rcx
141417306:     	movl	$0x1, %r8d
14141730c:     	callq	0x140613c20 <.text+0x612c20>
141417311:     	jmp	0x1414173d3 <.text+0x14163d3>
141417316:     	testq	%rsi, %rsi
141417319:     	je	0x14141737e <.text+0x141637e>
14141731b:     	testq	%r14, %r14
14141731e:     	je	0x141417331 <.text+0x1416331>
141417320:     	movl	$0x1, %r8d
141417326:     	movq	%r12, %rcx
141417329:     	movq	%r14, %rdx
14141732c:     	callq	0x140613c20 <.text+0x612c20>
141417331:     	testq	%r15, %r15
141417334:     	je	0x141417348 <.text+0x1416348>
141417336:     	movl	$0x1, %r8d
14141733c:     	movq	0x78(%rbp), %rcx
141417340:     	movq	%r15, %rdx
141417343:     	callq	0x140613c20 <.text+0x612c20>
141417348:     	movabsq	$-0x8000000000000000, %r14 # imm = 0x8000000000000000
141417352:     	leaq	(,%r13,2), %rax
14141735a:     	testq	%rax, %rax
14141735d:     	je	0x141417371 <.text+0x1416371>
14141735f:     	movl	$0x1, %r8d
141417365:     	movq	0x48(%rbp), %rcx
141417369:     	movq	%r13, %rdx
14141736c:     	callq	0x140613c20 <.text+0x612c20>
141417371:     	movq	%rsi, %r12
141417374:     	movq	%r14, %rax
141417377:     	negq	%rax
14141737a:     	jno	0x14141738e <.text+0x141638e>
14141737c:     	jmp	0x1414173f2 <.text+0x14163f2>
14141737e:     	movaps	0x30(%rbp), %xmm0
141417382:     	movaps	%xmm0, -0x60(%rbp)
141417386:     	movq	%r14, %rax
141417389:     	negq	%rax
14141738c:     	jo	0x1414173f2 <.text+0x14163f2>
14141738e:     	movq	0x20(%rbp), %rcx
141417392:     	movq	%rbx, 0x10(%rcx)
141417396:     	movq	%r15, 0x18(%rcx)
14141739a:     	movq	0x78(%rbp), %rax
14141739e:     	movq	%rax, 0x20(%rcx)
1414173a2:     	movq	0x28(%rbp), %rax
1414173a6:     	movq	%rax, 0x28(%rcx)
1414173aa:     	movq	%r13, 0x30(%rcx)
1414173ae:     	movq	0x48(%rbp), %rax
1414173b2:     	movq	%rax, 0x38(%rcx)
1414173b6:     	movaps	-0x60(%rbp), %xmm0
1414173ba:     	movups	%xmm0, 0x40(%rcx)
1414173be:     	movq	%r14, (%rcx)
1414173c1:     	movq	%r12, 0x8(%rcx)
1414173c5:     	jmp	0x141417412 <.text+0x1416412>
1414173c7:     	movq	%rsi, %rcx
1414173ca:     	addq	$0x8, %rcx
1414173ce:     	callq	0x1405e7a00 <.text+0x5e6a00>
1414173d3:     	movl	$0x28, %edx
1414173d8:     	movl	$0x8, %r8d
1414173de:     	movq	0x80(%rbp), %rcx
1414173e5:     	callq	0x140613c20 <.text+0x612c20>
1414173ea:     	movq	%r14, %rax
1414173ed:     	negq	%rax
1414173f0:     	jno	0x14141738e <.text+0x141638e>
1414173f2:     	movq	%r12, %rcx
1414173f5:     	movq	%rdi, %rdx
1414173f8:     	callq	0x1428a2200 <.text+0x28a1200>
1414173fd:     	movq	0x20(%rbp), %rcx
141417401:     	movq	%rax, 0x8(%rcx)
141417405:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141740f:     	movq	%rax, (%rcx)
141417412:     	movq	%rcx, %rax
141417415:     	movaps	0xa0(%rbp), %xmm6
14141741c:     	addq	$0x138, %rsp            # imm = 0x138
141417423:     	popq	%rbx
141417424:     	popq	%rdi
141417425:     	popq	%rsi
141417426:     	popq	%r12
141417428:     	popq	%r13
14141742a:     	popq	%r14
14141742c:     	popq	%r15
14141742e:     	popq	%rbp
14141742f:     	retq
141417430:     	movq	$0x3, -0x30(%rbp)
141417438:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141417442:     	movq	%rax, 0x68(%rbp)
141417446:     	movq	0x80(%rbp), %rax
14141744d:     	movq	%rax, 0x60(%rbp)
141417451:     	movq	0x70(%rbp), %rax
141417455:     	movq	%rax, 0x58(%rbp)
141417459:     	movb	$0x1, 0x97(%rbp)
141417460:     	leaq	-0x30(%rbp), %rdx
141417464:     	movq	%rdi, %rcx
141417467:     	callq	0x1428a0920 <.text+0x289f920>
14141746c:     	movq	%rax, %r12
14141746f:     	movabsq	$-0x8000000000000000, %r15 # imm = 0x8000000000000000
141417479:     	jmp	0x141416c2f <.text+0x1415c2f>
14141747e:     	movq	$0x3, -0x30(%rbp)
141417486:     	movq	0x88(%rbp), %rax
14141748d:     	movq	%rax, 0x68(%rbp)
141417491:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141749b:     	movq	%rax, 0x60(%rbp)
14141749f:     	movq	0x70(%rbp), %rax
1414174a3:     	movq	%rax, 0x58(%rbp)
1414174a7:     	movb	$0x1, 0x97(%rbp)
1414174ae:     	leaq	-0x30(%rbp), %rdx
1414174b2:     	movq	%rdi, %rcx
1414174b5:     	callq	0x1428a0920 <.text+0x289f920>
1414174ba:     	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
1414174c4:     	movq	%rcx, 0x80(%rbp)
1414174cb:     	xorl	%ebx, %ebx
1414174cd:     	movq	%rax, %r12
1414174d0:     	jmp	0x141416ac5 <.text+0x1415ac5>
1414174d5:     	movq	$0x3, -0x30(%rbp)
1414174dd:     	movq	0x88(%rbp), %rax
1414174e4:     	movq	%rax, 0x68(%rbp)
1414174e8:     	movq	0x80(%rbp), %rax
1414174ef:     	movq	%rax, 0x60(%rbp)
1414174f3:     	movq	0x70(%rbp), %rax
1414174f7:     	movq	%rax, 0x58(%rbp)
1414174fb:     	movb	$0x1, 0x97(%rbp)
141417502:     	leaq	-0x30(%rbp), %rdx
141417506:     	movq	%rdi, %rcx
141417509:     	callq	0x1428a0920 <.text+0x289f920>
14141750e:     	jmp	0x141416ac0 <.text+0x1415ac0>
141417513:     	movss	0x150e3e9(%rip), %xmm0  # 0x142925904
14141751b:     	movq	0x50(%rbp), %r12
14141751f:     	movq	-0x40(%rbp), %rax
141417523:     	movq	0x48(%rbp), %rdx
141417527:     	movq	0x28(%rbp), %r8
14141752b:     	movq	%rsi, %r9
14141752e:     	jmp	0x141416d2e <.text+0x1415d2e>
141417533:     	movq	$0x15, -0x30(%rbp)
14141753b:     	jmp	0x1414165ae <.text+0x14155ae>
141417540:     	movq	$0x8, -0x30(%rbp)
141417548:     	movq	0x88(%rbp), %rax
14141754f:     	movq	%rax, 0x68(%rbp)
141417553:     	movq	0x80(%rbp), %rax
14141755a:     	movq	%rax, 0x60(%rbp)
14141755e:     	movq	0x70(%rbp), %rax
141417562:     	movq	%rax, 0x58(%rbp)
141417566:     	movb	$0x1, 0x97(%rbp)
14141756d:     	leaq	-0x30(%rbp), %rdx
141417571:     	movq	%rdi, %rcx
141417574:     	callq	0x1428a0920 <.text+0x289f920>
141417579:     	jmp	0x141416ac0 <.text+0x1415ac0>
14141757e:     	movq	%rax, %rcx
141417581:     	movq	0x88(%rbp), %rax
141417588:     	movq	%rax, 0x68(%rbp)
14141758c:     	movq	0x80(%rbp), %rax
141417593:     	movq	%rax, 0x60(%rbp)
141417597:     	movq	%rcx, 0x58(%rbp)
14141759b:     	movb	$0x1, 0x97(%rbp)
1414175a2:     	leaq	0x1845e3f(%rip), %rcx   # 0x142c5d3e8
1414175a9:     	movl	$0x10, %edx
1414175ae:     	callq	0x1428f9db0 <.text+0x28f8db0>
1414175b3:     	jmp	0x141416ac0 <.text+0x1415ac0>
1414175b8:     	movq	0x88(%rbp), %rax
1414175bf:     	movq	%rax, 0x68(%rbp)
1414175c3:     	movq	%rcx, 0x60(%rbp)
1414175c7:     	movq	0x70(%rbp), %rax
1414175cb:     	movq	%rax, 0x58(%rbp)
1414175cf:     	movb	$0x1, 0x97(%rbp)
1414175d6:     	leaq	0x1845e01(%rip), %rcx   # 0x142c5d3de
1414175dd:     	movl	$0xa, %edx
1414175e2:     	callq	0x1428f9db0 <.text+0x28f8db0>
1414175e7:     	jmp	0x141416ac0 <.text+0x1415ac0>
1414175ec:     	movq	%rcx, 0x68(%rbp)
1414175f0:     	movq	0x80(%rbp), %rax
1414175f7:     	movq	%rax, 0x60(%rbp)
1414175fb:     	movq	0x70(%rbp), %rax
1414175ff:     	movq	%rax, 0x58(%rbp)
141417603:     	movb	$0x1, 0x97(%rbp)
14141760a:     	leaq	0x1845dc7(%rip), %rcx   # 0x142c5d3d8
141417611:     	movl	$0x6, %edx
141417616:     	callq	0x1428f9db0 <.text+0x28f8db0>
14141761b:     	jmp	0x141416ac0 <.text+0x1415ac0>
141417620:     	movq	0x88(%rbp), %rax
141417627:     	movq	%rax, 0x68(%rbp)
14141762b:     	movq	0x80(%rbp), %rax
141417632:     	movq	%rax, 0x60(%rbp)
141417636:     	movq	0x70(%rbp), %rax
14141763a:     	movq	%rax, 0x58(%rbp)
14141763e:     	movb	$0x1, 0x97(%rbp)
141417645:     	leaq	0x1845dac(%rip), %rcx   # 0x142c5d3f8
14141764c:     	movl	$0x19, %edx
141417651:     	callq	0x1428f9db0 <.text+0x28f8db0>
141417656:     	jmp	0x141416ac0 <.text+0x1415ac0>
14141765b:     	movq	$0x6, -0x30(%rbp)
141417663:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
14141766d:     	movq	%rax, 0x68(%rbp)
141417671:     	movq	0x80(%rbp), %rax
141417678:     	movq	%rax, 0x60(%rbp)
14141767c:     	movq	0x70(%rbp), %rax
141417680:     	movq	%rax, 0x58(%rbp)
141417684:     	movb	$0x1, 0x97(%rbp)
14141768b:     	leaq	-0x30(%rbp), %rdx
14141768f:     	movq	%rdi, %rcx
141417692:     	callq	0x1428a0920 <.text+0x289f920>
141417697:     	jmp	0x14141746c <.text+0x141646c>
14141769c:     	movq	$0x6, -0x30(%rbp)
1414176a4:     	movq	0x88(%rbp), %rax
1414176ab:     	movq	%rax, 0x68(%rbp)
1414176af:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1414176b9:     	movq	%rax, 0x60(%rbp)
1414176bd:     	movq	0x70(%rbp), %rax
1414176c1:     	movq	%rax, 0x58(%rbp)
1414176c5:     	movb	$0x1, 0x97(%rbp)
1414176cc:     	leaq	-0x30(%rbp), %rdx
1414176d0:     	movq	%rdi, %rcx
1414176d3:     	callq	0x1428a0920 <.text+0x289f920>
1414176d8:     	jmp	0x1414174ba <.text+0x14164ba>
1414176dd:     	movq	$0x6, -0x30(%rbp)
1414176e5:     	movq	0x88(%rbp), %rax
1414176ec:     	movq	%rax, 0x68(%rbp)
1414176f0:     	movq	0x80(%rbp), %rax
1414176f7:     	movq	%rax, 0x60(%rbp)
1414176fb:     	movq	0x70(%rbp), %rax
1414176ff:     	movq	%rax, 0x58(%rbp)
141417703:     	movb	$0x1, 0x97(%rbp)
14141770a:     	leaq	-0x30(%rbp), %rdx
14141770e:     	movq	%rdi, %rcx
141417711:     	callq	0x1428a0920 <.text+0x289f920>
141417716:     	jmp	0x141416ac0 <.text+0x1415ac0>
14141771b:     	movq	$0x6, -0x30(%rbp)
141417723:     	movq	0x88(%rbp), %rax
14141772a:     	movq	%rax, 0x68(%rbp)
14141772e:     	movq	0x80(%rbp), %rax
141417735:     	movq	%rax, 0x60(%rbp)
141417739:     	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
141417743:     	movq	%rax, 0x58(%rbp)
141417747:     	movb	$0x1, 0x97(%rbp)
14141774e:     	leaq	-0x30(%rbp), %rdx
141417752:     	movq	%rdi, %rcx
141417755:     	callq	0x1428a0920 <.text+0x289f920>
14141775a:     	jmp	0x1414171e1 <.text+0x14161e1>
14141775f:     	movq	$0x12, -0x30(%rbp)
141417767:     	leaq	-0x30(%rbp), %rax
14141776b:     	movq	%rdx, %rcx
14141776e:     	movq	%rax, %rdx
141417771:     	callq	0x1428a0920 <.text+0x289f920>
141417776:     	jmp	0x141416cdf <.text+0x1415cdf>
14141777b:     	movq	$0x7, -0x30(%rbp)
141417783:     	leaq	-0x30(%rbp), %rdx
141417787:     	callq	0x1428a0920 <.text+0x289f920>
14141778c:     	jmp	0x141416cdf <.text+0x1415cdf>
141417791:     	nopw	%cs:(%rax,%rax)
1414177a0:     	movq	%rdx, 0x10(%rsp)
1414177a5:     	pushq	%rbp
1414177a6:     	pushq	%r15
1414177a8:     	pushq	%r14
1414177aa:     	pushq	%r13
1414177ac:     	pushq	%r12
1414177ae:     	pushq	%rsi
1414177af:     	pushq	%rdi
1414177b0:     	pushq	%rbx
1414177b1:     	subq	$0x38, %rsp
1414177b5:     	leaq	0x80(%rdx), %rbp
1414177bc:     	movaps	%xmm6, 0x20(%rsp)
1414177c1:     	movq	0x28(%rbp), %rax
1414177c5:     	leaq	(,%rax,2), %rax
1414177cd:     	testq	%rax, %rax
1414177d0:     	je	0x1414177e5 <.text+0x14167e5>
1414177d2:     	movl	$0x1, %r8d
1414177d8:     	movq	0x48(%rbp), %rcx
1414177dc:     	movq	0x28(%rbp), %rdx
1414177e0:     	callq	0x140613c20 <.text+0x612c20>
1414177e5:     	movaps	0x20(%rsp), %xmm6
1414177ea:     	addq	$0x38, %rsp
1414177ee:     	popq	%rbx
1414177ef:     	popq	%rdi
1414177f0:     	popq	%rsi
1414177f1:     	popq	%r12
1414177f3:     	popq	%r13
1414177f5:     	popq	%r14
1414177f7:     	popq	%r15
1414177f9:     	popq	%rbp
1414177fa:     	retq
1414177fb:     	nopl	(%rax,%rax)
141417800:     	movq	%rdx, 0x10(%rsp)
141417805:     	pushq	%rbp
141417806:     	pushq	%r15
141417808:     	pushq	%r14
14141780a:     	pushq	%r13
14141780c:     	pushq	%r12
14141780e:     	pushq	%rsi
14141780f:     	pushq	%rdi
141417810:     	pushq	%rbx
141417811:     	subq	$0x38, %rsp
141417815:     	leaq	0x80(%rdx), %rbp
14141781c:     	movaps	%xmm6, 0x20(%rsp)
141417821:     	cmpq	$0x0, 0x88(%rbp)
141417829:     	je	0x141417841 <.text+0x1416841>
14141782b:     	movl	$0x1, %r8d
141417831:     	movq	0x50(%rbp), %rcx
141417835:     	movq	0x88(%rbp), %rdx
14141783c:     	callq	0x140613c20 <.text+0x612c20>
141417841:     	movq	0x88(%rbp), %rax
141417848:     	movq	%rax, 0x68(%rbp)
14141784c:     	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141417856:     	movq	%rax, 0x60(%rbp)
14141785a:     	movq	0x70(%rbp), %rax
14141785e:     	movq	%rax, 0x58(%rbp)
141417862:     	movb	$0x0, 0x97(%rbp)
141417869:     	movaps	0x20(%rsp), %xmm6
14141786e:     	addq	$0x38, %rsp
141417872:     	popq	%rbx
141417873:     	popq	%rdi
141417874:     	popq	%rsi
141417875:     	popq	%r12
141417877:     	popq	%r13
141417879:     	popq	%r14
14141787b:     	popq	%r15
14141787d:     	popq	%rbp
14141787e:     	retq
14141787f:     	nop
141417880:     	movq	%rdx, 0x10(%rsp)
141417885:     	pushq	%rbp
141417886:     	pushq	%r15
141417888:     	pushq	%r14
14141788a:     	pushq	%r13
14141788c:     	pushq	%r12
14141788e:     	pushq	%rsi
14141788f:     	pushq	%rdi
141417890:     	pushq	%rbx
141417891:     	subq	$0x38, %rsp
141417895:     	leaq	0x80(%rdx), %rbp
14141789c:     	movaps	%xmm6, 0x20(%rsp)
1414178a1:     	movl	$0x28, %edx
1414178a6:     	movl	$0x8, %r8d
1414178ac:     	movq	0x80(%rbp), %rcx
1414178b3:     	callq	0x140613c20 <.text+0x612c20>
1414178b8:     	movaps	0x20(%rsp), %xmm6
1414178bd:     	addq	$0x38, %rsp
1414178c1:     	popq	%rbx
1414178c2:     	popq	%rdi
1414178c3:     	popq	%rsi
1414178c4:     	popq	%r12
1414178c6:     	popq	%r13
1414178c8:     	popq	%r14
1414178ca:     	popq	%r15
1414178cc:     	popq	%rbp
1414178cd:     	retq
1414178ce:     	nop
1414178d0:     	movq	%rdx, 0x10(%rsp)
1414178d5:     	pushq	%rbp
1414178d6:     	pushq	%r15
1414178d8:     	pushq	%r14
1414178da:     	pushq	%r13
1414178dc:     	pushq	%r12
1414178de:     	pushq	%rsi
1414178df:     	pushq	%rdi
1414178e0:     	pushq	%rbx
1414178e1:     	subq	$0x38, %rsp
1414178e5:     	leaq	0x80(%rdx), %rbp
1414178ec:     	movaps	%xmm6, 0x20(%rsp)
1414178f1:     	movl	$0x28, %edx
1414178f6:     	movl	$0x8, %r8d
1414178fc:     	movq	0x80(%rbp), %rcx
141417903:     	callq	0x140613c20 <.text+0x612c20>
141417908:     	movaps	0x20(%rsp), %xmm6
14141790d:     	addq	$0x38, %rsp
141417911:     	popq	%rbx
141417912:     	popq	%rdi
141417913:     	popq	%rsi
141417914:     	popq	%r12
141417916:     	popq	%r13
141417918:     	popq	%r14
14141791a:     	popq	%r15
14141791c:     	popq	%rbp
14141791d:     	retq
14141791e:     	nop
141417920:     	movq	%rdx, 0x10(%rsp)
141417925:     	pushq	%rbp
141417926:     	pushq	%r15
141417928:     	pushq	%r14
14141792a:     	pushq	%r13
14141792c:     	pushq	%r12
14141792e:     	pushq	%rsi
14141792f:     	pushq	%rdi
141417930:     	pushq	%rbx
141417931:     	subq	$0x38, %rsp
141417935:     	leaq	0x80(%rdx), %rbp
14141793c:     	movaps	%xmm6, 0x20(%rsp)
141417941:     	cmpq	$0x0, 0x80(%rbp)
141417949:     	je	0x141417961 <.text+0x1416961>
14141794b:     	movl	$0x1, %r8d
141417951:     	movq	0x78(%rbp), %rcx
141417955:     	movq	0x80(%rbp), %rdx
14141795c:     	callq	0x140613c20 <.text+0x612c20>
141417961:     	movaps	0x20(%rsp), %xmm6
141417966:     	addq	$0x38, %rsp
14141796a:     	popq	%rbx
14141796b:     	popq	%rdi
14141796c:     	popq	%rsi
14141796d:     	popq	%r12
14141796f:     	popq	%r13
141417971:     	popq	%r14
141417973:     	popq	%r15
141417975:     	popq	%rbp
141417976:     	retq
141417977:     	nopw	(%rax,%rax)
141417980:     	movq	%rdx, 0x10(%rsp)
141417985:     	pushq	%rbp
141417986:     	pushq	%r15
141417988:     	pushq	%r14
14141798a:     	pushq	%r13
14141798c:     	pushq	%r12
14141798e:     	pushq	%rsi
14141798f:     	pushq	%rdi
141417990:     	pushq	%rbx
141417991:     	subq	$0x38, %rsp
141417995:     	leaq	0x80(%rdx), %rbp
14141799c:     	movaps	%xmm6, 0x20(%rsp)
1414179a1:     	cmpq	$0x0, 0x88(%rbp)
1414179a9:     	je	0x1414179c1 <.text+0x14169c1>
1414179ab:     	movl	$0x1, %r8d
1414179b1:     	movq	0x50(%rbp), %rcx
1414179b5:     	movq	0x88(%rbp), %rdx
1414179bc:     	callq	0x140613c20 <.text+0x612c20>
1414179c1:     	movaps	0x20(%rsp), %xmm6
1414179c6:     	addq	$0x38, %rsp
1414179ca:     	popq	%rbx
1414179cb:     	popq	%rdi
1414179cc:     	popq	%rsi
1414179cd:     	popq	%r12
1414179cf:     	popq	%r13
1414179d1:     	popq	%r14
1414179d3:     	popq	%r15
1414179d5:     	popq	%rbp
1414179d6:     	retq
1414179d7:     	nopw	(%rax,%rax)
1414179e0:     	movq	%rdx, 0x10(%rsp)
1414179e5:     	pushq	%rbp
1414179e6:     	pushq	%r15
1414179e8:     	pushq	%r14
1414179ea:     	pushq	%r13
1414179ec:     	pushq	%r12
1414179ee:     	pushq	%rsi
1414179ef:     	pushq	%rdi
1414179f0:     	pushq	%rbx
1414179f1:     	subq	$0x38, %rsp
1414179f5:     	leaq	0x80(%rdx), %rbp
1414179fc:     	movaps	%xmm6, 0x20(%rsp)
141417a01:     	leaq	-0x30(%rbp), %rcx
141417a05:     	callq	0x141408540 <.text+0x1407540>
141417a0a:     	movaps	0x20(%rsp), %xmm6
141417a0f:     	addq	$0x38, %rsp
141417a13:     	popq	%rbx
141417a14:     	popq	%rdi
141417a15:     	popq	%rsi
141417a16:     	popq	%r12
141417a18:     	popq	%r13
141417a1a:     	popq	%r14
141417a1c:     	popq	%r15
141417a1e:     	popq	%rbp
141417a1f:     	retq
141417a20:     	movq	%rdx, 0x10(%rsp)
141417a25:     	pushq	%rbp
141417a26:     	pushq	%r15
141417a28:     	pushq	%r14
141417a2a:     	pushq	%r13
141417a2c:     	pushq	%r12
141417a2e:     	pushq	%rsi
141417a2f:     	pushq	%rdi
141417a30:     	pushq	%rbx
141417a31:     	subq	$0x38, %rsp
141417a35:     	leaq	0x80(%rdx), %rbp
141417a3c:     	movaps	%xmm6, 0x20(%rsp)
141417a41:     	leaq	-0x30(%rbp), %rcx
141417a45:     	callq	0x141408540 <.text+0x1407540>
141417a4a:     	movaps	0x20(%rsp), %xmm6
141417a4f:     	addq	$0x38, %rsp
141417a53:     	popq	%rbx
141417a54:     	popq	%rdi
141417a55:     	popq	%rsi
141417a56:     	popq	%r12
141417a58:     	popq	%r13
141417a5a:     	popq	%r14
141417a5c:     	popq	%r15
141417a5e:     	popq	%rbp
141417a5f:     	retq
141417a60:     	movq	%rdx, 0x10(%rsp)
141417a65:     	pushq	%rbp
141417a66:     	pushq	%r15
141417a68:     	pushq	%r14
141417a6a:     	pushq	%r13
141417a6c:     	pushq	%r12
141417a6e:     	pushq	%rsi
141417a6f:     	pushq	%rdi
141417a70:     	pushq	%rbx
141417a71:     	subq	$0x38, %rsp
141417a75:     	leaq	0x80(%rdx), %rbp
141417a7c:     	movaps	%xmm6, 0x20(%rsp)
141417a81:     	movzbl	0x97(%rbp), %eax
141417a88:     	movb	%al, 0x88(%rbp)
141417a8e:     	movq	0x58(%rbp), %rdx
141417a92:     	movq	0x60(%rbp), %rax
141417a96:     	movq	%rax, 0x80(%rbp)
141417a9d:     	movq	0x68(%rbp), %rax
141417aa1:     	movq	%rax, 0x20(%rbp)
141417aa5:     	movq	%rdx, %rax
141417aa8:     	negq	%rax
141417aab:     	jo	0x141417ad0 <.text+0x1416ad0>
141417aad:     	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
141417ab7:     	cmpq	%rax, %rdx
141417aba:     	je	0x141417ae6 <.text+0x1416ae6>
141417abc:     	testq	%rdx, %rdx
141417abf:     	je	0x141417ad0 <.text+0x1416ad0>
141417ac1:     	movl	$0x1, %r8d
141417ac7:     	movq	0x48(%rbp), %rcx
141417acb:     	callq	0x140613c20 <.text+0x612c20>
141417ad0:     	movaps	0x20(%rsp), %xmm6
141417ad5:     	addq	$0x38, %rsp
141417ad9:     	popq	%rbx
141417ada:     	popq	%rdi
141417adb:     	popq	%rsi
141417adc:     	popq	%r12
141417ade:     	popq	%r13
141417ae0:     	popq	%r14
141417ae2:     	popq	%r15
141417ae4:     	popq	%rbp
141417ae5:     	retq
141417ae6:     	movaps	0x20(%rsp), %xmm6
141417aeb:     	addq	$0x38, %rsp
141417aef:     	popq	%rbx
141417af0:     	popq	%rdi
141417af1:     	popq	%rsi
141417af2:     	popq	%r12
141417af4:     	popq	%r13
141417af6:     	popq	%r14
141417af8:     	popq	%r15
141417afa:     	popq	%rbp
141417afb:     	retq
141417afc:     	nopl	(%rax)
141417b00:     	movq	%rdx, 0x10(%rsp)
141417b05:     	pushq	%rbp
141417b06:     	pushq	%r15
141417b08:     	pushq	%r14
141417b0a:     	pushq	%r13
141417b0c:     	pushq	%r12
141417b0e:     	pushq	%rsi
141417b0f:     	pushq	%rdi
141417b10:     	pushq	%rbx
141417b11:     	subq	$0x38, %rsp
141417b15:     	leaq	0x80(%rdx), %rbp
141417b1c:     	movaps	%xmm6, 0x20(%rsp)
141417b21:     	movq	0x80(%rbp), %rax
141417b28:     	negq	%rax
141417b2b:     	jo	0x141417b5b <.text+0x1416b5b>
141417b2d:     	jae	0x141417b45 <.text+0x1416b45>
141417b2f:     	movl	$0x1, %r8d
141417b35:     	movq	0x78(%rbp), %rcx
141417b39:     	movq	0x80(%rbp), %rdx
141417b40:     	callq	0x140613c20 <.text+0x612c20>
141417b45:     	movaps	0x20(%rsp), %xmm6
141417b4a:     	addq	$0x38, %rsp
141417b4e:     	popq	%rbx
141417b4f:     	popq	%rdi
141417b50:     	popq	%rsi
141417b51:     	popq	%r12
141417b53:     	popq	%r13
141417b55:     	popq	%r14
141417b57:     	popq	%r15
141417b59:     	popq	%rbp
141417b5a:     	retq
141417b5b:     	movaps	0x20(%rsp), %xmm6
141417b60:     	addq	$0x38, %rsp
141417b64:     	popq	%rbx
141417b65:     	popq	%rdi
141417b66:     	popq	%rsi
141417b67:     	popq	%r12
141417b69:     	popq	%r13
141417b6b:     	popq	%r14
141417b6d:     	popq	%r15
141417b6f:     	popq	%rbp
141417b70:     	retq
141417b71:     	nopw	%cs:(%rax,%rax)
141417b80:     	movq	%rdx, 0x10(%rsp)
141417b85:     	pushq	%rbp
141417b86:     	pushq	%r15
141417b88:     	pushq	%r14
141417b8a:     	pushq	%r13
141417b8c:     	pushq	%r12
141417b8e:     	pushq	%rsi
141417b8f:     	pushq	%rdi
141417b90:     	pushq	%rbx
141417b91:     	subq	$0x38, %rsp
141417b95:     	leaq	0x80(%rdx), %rbp
141417b9c:     	movaps	%xmm6, 0x20(%rsp)
141417ba1:     	movq	0x20(%rbp), %rax
141417ba5:     	negq	%rax
141417ba8:     	jo	0x141417bcd <.text+0x1416bcd>
141417baa:     	testb	$0x1, 0x88(%rbp)
141417bb1:     	je	0x141417bcd <.text+0x1416bcd>
141417bb3:     	cmpq	$0x0, 0x20(%rbp)
141417bb8:     	je	0x141417bcd <.text+0x1416bcd>
141417bba:     	movl	$0x1, %r8d
141417bc0:     	movq	0x50(%rbp), %rcx
141417bc4:     	movq	0x20(%rbp), %rdx
141417bc8:     	callq	0x140613c20 <.text+0x612c20>
141417bcd:     	movaps	0x20(%rsp), %xmm6
141417bd2:     	addq	$0x38, %rsp
141417bd6:     	popq	%rbx
141417bd7:     	popq	%rdi
141417bd8:     	popq	%rsi
141417bd9:     	popq	%r12
141417bdb:     	popq	%r13
141417bdd:     	popq	%r14
141417bdf:     	popq	%r15
141417be1:     	popq	%rbp
141417be2:     	retq
