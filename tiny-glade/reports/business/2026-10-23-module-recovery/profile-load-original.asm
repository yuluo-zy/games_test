
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140916480:     	pushq	%rbp
140916481:     	pushq	%rsi
140916482:     	pushq	%rdi
140916483:     	pushq	%rbx
140916484:     	subq	$0x518, %rsp            # imm = 0x518
14091648b:     	leaq	0x80(%rsp), %rbp
140916493:     	movq	$-0x2, 0x490(%rbp)
14091649e:     	movq	%rcx, %rsi
1409164a1:     	callq	0x140b9db30 <.text+0xb9cb30>
1409164a6:     	movq	%rdx, %r8
1409164a9:     	movq	$0xb, 0x20(%rsp)
1409164b2:     	leaq	0x2196563(%rip), %r9    # 0x142aaca1c
1409164b9:     	leaq	0x230(%rbp), %rdi
1409164c0:     	movq	%rdi, %rcx
1409164c3:     	movq	%rax, %rdx
1409164c6:     	callq	0x140d62810 <.text+0xd61810>
1409164cb:     	movb	$0x1, 0x48f(%rbp)
1409164d2:     	movq	%rdi, %rcx
1409164d5:     	callq	0x140d76ed0 <.text+0xd75ed0>
1409164da:     	movb	$0x1, 0x48f(%rbp)
1409164e1:     	movq	%rax, %rcx
1409164e4:     	callq	0x140d62cf0 <.text+0xd61cf0>
1409164e9:     	testb	%al, %al
1409164eb:     	je	0x14091685d <.text+0x91585d>
1409164f1:     	movups	0x230(%rbp), %xmm0
1409164f8:     	movups	0x240(%rbp), %xmm1
1409164ff:     	movaps	%xmm1, -0x40(%rbp)
140916503:     	movaps	%xmm0, -0x50(%rbp)
140916507:     	leaq	-0x50(%rbp), %rcx
14091650b:     	callq	0x140d76ed0 <.text+0xd75ed0>
140916510:     	movq	%rdx, %r8
140916513:     	leaq	0x458(%rbp), %rcx
14091651a:     	movq	%rax, %rdx
14091651d:     	callq	0x140d59140 <.text+0xd58140>
140916522:     	movq	-0x50(%rbp), %rdx
140916526:     	testq	%rdx, %rdx
140916529:     	je	0x14091653a <.text+0x91553a>
14091652b:     	movq	-0x48(%rbp), %rcx
14091652f:     	movl	$0x1, %r8d
140916535:     	callq	0x140613c20 <.text+0x612c20>
14091653a:     	movq	0x458(%rbp), %rcx
140916541:     	movq	%rcx, %rax
140916544:     	negq	%rax
140916547:     	jno	0x140916ad6 <.text+0x915ad6>
14091654d:     	movq	0x460(%rbp), %rax
140916554:     	movq	%rax, 0x1c0(%rbp)
14091655b:     	movq	0x2d15e46(%rip), %rax   # 0x14362c3a8
140916562:     	movq	(%rax), %rax
140916565:     	testq	%rax, %rax
140916568:     	je	0x14091661f <.text+0x91561f>
14091656e:     	leaq	0x1c0(%rbp), %rax
140916575:     	movq	%rax, 0x250(%rbp)
14091657c:     	leaq	0x44475d(%rip), %rax    # 0x140d5ace0 <.text+0xd59ce0>
140916583:     	movq	%rax, 0x258(%rbp)
14091658a:     	leaq	0x21965ff(%rip), %rcx   # 0x142aacb90
140916591:     	callq	0x14067dd80 <.text+0x67cd80>
140916596:     	movups	(%rax), %xmm0
140916599:     	movl	0x10(%rax), %eax
14091659c:     	movq	$0x1, -0x20(%rbp)
1409165a4:     	leaq	0x21964fd(%rip), %rcx   # 0x142aacaa8
1409165ab:     	movq	%rcx, -0x18(%rbp)
1409165af:     	movq	$0x1a, -0x10(%rbp)
1409165b7:     	leaq	0x21965c2(%rip), %rdx   # 0x142aacb80
1409165be:     	movq	%rdx, (%rbp)
1409165c2:     	movq	$0x1, 0x8(%rbp)
1409165ca:     	leaq	0x250(%rbp), %rdx
1409165d1:     	movq	%rdx, 0x10(%rbp)
1409165d5:     	movq	$0x1, 0x18(%rbp)
1409165dd:     	movq	$0x0, 0x20(%rbp)
1409165e5:     	movq	$0x0, -0x50(%rbp)
1409165ed:     	movq	%rcx, -0x48(%rbp)
1409165f1:     	movq	$0x1a, -0x40(%rbp)
1409165f9:     	movq	$0x0, -0x38(%rbp)
140916601:     	movups	%xmm0, -0x30(%rbp)
140916605:     	movl	$0x1, -0x8(%rbp)
14091660c:     	movl	%eax, -0x4(%rbp)
14091660f:     	leaq	0x48e(%rbp), %rcx
140916616:     	leaq	-0x50(%rbp), %rdx
14091661a:     	callq	0x140d4a330 <.text+0xd49330>
14091661f:     	leaq	0x418(%rbp), %rcx
140916626:     	xorps	%xmm0, %xmm0
140916629:     	movups	%xmm0, 0x440(%rbp)
140916630:     	movups	%xmm0, 0x430(%rbp)
140916637:     	movq	$0x0, 0x44f(%rbp)
140916642:     	callq	0x140c102e0 <.text+0xc0f2e0>
140916647:     	movl	$0x2a, 0x250(%rbp)
140916651:     	movl	$0x26, 0x25c(%rbp)
14091665b:     	movl	$0x14, 0x268(%rbp)
140916665:     	movl	$0x17, 0x274(%rbp)
14091666f:     	movl	$0x18, 0x280(%rbp)
140916679:     	movl	$0x24, 0x28c(%rbp)
140916683:     	movl	$0xc3, 0x298(%rbp)
14091668d:     	movl	$0xc3, 0x2a4(%rbp)
140916697:     	movl	$0xc3, 0x2b0(%rbp)
1409166a1:     	movl	$0xc3, 0x2bc(%rbp)
1409166ab:     	movl	$0x7, 0x2c8(%rbp)
1409166b5:     	movl	$0x8, 0x2d4(%rbp)
1409166bf:     	movl	$0x9, 0x2e0(%rbp)
1409166c9:     	movl	$0xa, 0x2ec(%rbp)
1409166d3:     	movl	$0xb, 0x2f8(%rbp)
1409166dd:     	movl	$0xc, 0x304(%rbp)
1409166e7:     	movl	$0xd, 0x310(%rbp)
1409166f1:     	movl	$0xe, 0x31c(%rbp)
1409166fb:     	movl	$0xf, 0x328(%rbp)
140916705:     	movl	$0x23, 0x334(%rbp)
14091670f:     	movl	$0x27, 0x340(%rbp)
140916719:     	movl	$0x2d, 0x34c(%rbp)
140916723:     	movl	$0x2b, 0x358(%rbp)
14091672d:     	movl	$0xc3, 0x364(%rbp)
140916737:     	movl	$0x38, 0x370(%rbp)
140916741:     	movl	$0xab, 0x37c(%rbp)
14091674b:     	movl	$0xa5, 0x388(%rbp)
140916755:     	movl	$0x1b, 0x394(%rbp)
14091675f:     	movl	$0x49, 0x3a0(%rbp)
140916769:     	movl	$0x3a, 0x3ac(%rbp)
140916773:     	movl	$0x51, 0x3b8(%rbp)
14091677d:     	movl	$0x52, 0x3c4(%rbp)
140916787:     	movl	$0x50, 0x3d0(%rbp)
140916791:     	movl	$0x53, 0x3dc(%rbp)
14091679b:     	movl	$0x4f, 0x3e8(%rbp)
1409167a5:     	movl	$0x4e, 0x3f4(%rbp)
1409167af:     	movw	$0x1, 0x40c(%rbp)
1409167b8:     	movw	$0x2, 0x410(%rbp)
1409167c1:     	movw	$0x1, 0x414(%rbp)
1409167ca:     	movl	$0xc3, 0x400(%rbp)
1409167d4:     	movq	$0x0, 0x428(%rbp)
1409167df:     	movq	0x1c0(%rbp), %rax
1409167e6:     	movl	%eax, %ecx
1409167e8:     	andl	$0x3, %ecx
1409167eb:     	cmpl	$0x1, %ecx
1409167ee:     	jne	0x140916e74 <.text+0x915e74>
1409167f4:     	leaq	-0x1(%rax), %rcx
1409167f8:     	movq	%rcx, 0x470(%rbp)
1409167ff:     	movq	-0x1(%rax), %rcx
140916803:     	movq	%rcx, 0x480(%rbp)
14091680a:     	movq	0x7(%rax), %rax
14091680e:     	movq	%rax, 0x478(%rbp)
140916815:     	movq	(%rax), %rax
140916818:     	testq	%rax, %rax
14091681b:     	je	0x140916826 <.text+0x915826>
14091681d:     	movq	0x480(%rbp), %rcx
140916824:     	callq	*%rax
140916826:     	movq	0x480(%rbp), %rcx
14091682d:     	movq	0x478(%rbp), %rax
140916834:     	movq	0x8(%rax), %rdx
140916838:     	testq	%rdx, %rdx
14091683b:     	je	0x140916846 <.text+0x915846>
14091683d:     	movq	0x10(%rax), %r8
140916841:     	callq	0x140613c20 <.text+0x612c20>
140916846:     	movl	$0x18, %edx
14091684b:     	movl	$0x8, %r8d
140916851:     	movq	0x470(%rbp), %rcx
140916858:     	jmp	0x140916e6f <.text+0x915e6f>
14091685d:     	movq	0x2d15b44(%rip), %rax   # 0x14362c3a8
140916864:     	movq	(%rax), %rax
140916867:     	cmpq	$0x3, %rax
14091686b:     	jb	0x140916908 <.text+0x915908>
140916871:     	movb	$0x1, 0x48f(%rbp)
140916878:     	leaq	0x2196211(%rip), %rcx   # 0x142aaca90
14091687f:     	callq	0x14067dd80 <.text+0x67cd80>
140916884:     	movups	(%rax), %xmm0
140916887:     	movl	0x10(%rax), %eax
14091688a:     	movq	$0x3, -0x20(%rbp)
140916892:     	leaq	0x219620f(%rip), %rcx   # 0x142aacaa8
140916899:     	movq	%rcx, -0x18(%rbp)
14091689d:     	movq	$0x1a, -0x10(%rbp)
1409168a5:     	leaq	0x21961ac(%rip), %rdx   # 0x142aaca58
1409168ac:     	movq	%rdx, (%rbp)
1409168b0:     	movq	$0x1, 0x8(%rbp)
1409168b8:     	movq	$0x8, 0x10(%rbp)
1409168c0:     	movq	$0x0, -0x50(%rbp)
1409168c8:     	xorps	%xmm1, %xmm1
1409168cb:     	movups	%xmm1, 0x18(%rbp)
1409168cf:     	movq	%rcx, -0x48(%rbp)
1409168d3:     	movq	$0x1a, -0x40(%rbp)
1409168db:     	movq	$0x0, -0x38(%rbp)
1409168e3:     	movups	%xmm0, -0x30(%rbp)
1409168e7:     	movl	$0x1, -0x8(%rbp)
1409168ee:     	movl	%eax, -0x4(%rbp)
1409168f1:     	movb	$0x1, 0x48f(%rbp)
1409168f8:     	leaq	0x48e(%rbp), %rcx
1409168ff:     	leaq	-0x50(%rbp), %rdx
140916903:     	callq	0x140d4a330 <.text+0xd49330>
140916908:     	leaq	0x418(%rbp), %rcx
14091690f:     	xorps	%xmm0, %xmm0
140916912:     	movups	%xmm0, 0x440(%rbp)
140916919:     	movups	%xmm0, 0x430(%rbp)
140916920:     	movq	$0x0, 0x44f(%rbp)
14091692b:     	movb	$0x1, 0x48f(%rbp)
140916932:     	callq	0x140c102e0 <.text+0xc0f2e0>
140916937:     	movl	$0x2a, 0x250(%rbp)
140916941:     	movl	$0x26, 0x25c(%rbp)
14091694b:     	movl	$0x14, 0x268(%rbp)
140916955:     	movl	$0x17, 0x274(%rbp)
14091695f:     	movl	$0x18, 0x280(%rbp)
140916969:     	movl	$0x24, 0x28c(%rbp)
140916973:     	movl	$0xc3, 0x298(%rbp)
14091697d:     	movl	$0xc3, 0x2a4(%rbp)
140916987:     	movl	$0xc3, 0x2b0(%rbp)
140916991:     	movl	$0xc3, 0x2bc(%rbp)
14091699b:     	movl	$0x7, 0x2c8(%rbp)
1409169a5:     	movl	$0x8, 0x2d4(%rbp)
1409169af:     	movl	$0x9, 0x2e0(%rbp)
1409169b9:     	movl	$0xa, 0x2ec(%rbp)
1409169c3:     	movl	$0xb, 0x2f8(%rbp)
1409169cd:     	movl	$0xc, 0x304(%rbp)
1409169d7:     	movl	$0xd, 0x310(%rbp)
1409169e1:     	movl	$0xe, 0x31c(%rbp)
1409169eb:     	movl	$0xf, 0x328(%rbp)
1409169f5:     	movl	$0x23, 0x334(%rbp)
1409169ff:     	movl	$0x27, 0x340(%rbp)
140916a09:     	movl	$0x2d, 0x34c(%rbp)
140916a13:     	movl	$0x2b, 0x358(%rbp)
140916a1d:     	movl	$0xc3, 0x364(%rbp)
140916a27:     	movl	$0x38, 0x370(%rbp)
140916a31:     	movl	$0xab, 0x37c(%rbp)
140916a3b:     	movl	$0xa5, 0x388(%rbp)
140916a45:     	movl	$0x1b, 0x394(%rbp)
140916a4f:     	movl	$0x49, 0x3a0(%rbp)
140916a59:     	movl	$0x3a, 0x3ac(%rbp)
140916a63:     	movl	$0x51, 0x3b8(%rbp)
140916a6d:     	movl	$0x52, 0x3c4(%rbp)
140916a77:     	movl	$0x50, 0x3d0(%rbp)
140916a81:     	movl	$0x53, 0x3dc(%rbp)
140916a8b:     	movl	$0x4f, 0x3e8(%rbp)
140916a95:     	movl	$0x4e, 0x3f4(%rbp)
140916a9f:     	movw	$0x1, 0x40c(%rbp)
140916aa8:     	movw	$0x2, 0x410(%rbp)
140916ab1:     	movw	$0x1, 0x414(%rbp)
140916aba:     	movl	$0xc3, 0x400(%rbp)
140916ac4:     	movq	$0x0, 0x428(%rbp)
140916acf:     	movb	$0x1, %bl
140916ad1:     	jmp	0x140916e76 <.text+0x915e76>
140916ad6:     	movq	%rcx, 0x480(%rbp)
140916add:     	movq	0x460(%rbp), %r8
140916ae4:     	movq	0x468(%rbp), %r9
140916aeb:     	movq	$0x0, 0x260(%rbp)
140916af6:     	movq	$0x1, 0x250(%rbp)
140916b01:     	movq	$0x80, 0x258(%rbp)
140916b0c:     	leaq	-0x50(%rbp), %rcx
140916b10:     	leaq	0x250(%rbp), %rdx
140916b17:     	movq	%r8, 0x478(%rbp)
140916b1e:     	callq	0x14169c0d0 <.text+0x169b0d0>
140916b23:     	cmpl	$0xc4, -0x50(%rbp)
140916b2a:     	jne	0x140916e40 <.text+0x915e40>
140916b30:     	movq	0x8(%rbp), %rax
140916b34:     	movq	%rax, 0x210(%rbp)
140916b3b:     	movups	-0x8(%rbp), %xmm0
140916b3f:     	movaps	%xmm0, 0x200(%rbp)
140916b46:     	movups	-0x48(%rbp), %xmm0
140916b4a:     	movups	-0x38(%rbp), %xmm1
140916b4e:     	movups	-0x28(%rbp), %xmm2
140916b52:     	movups	-0x18(%rbp), %xmm3
140916b56:     	movaps	%xmm3, 0x1f0(%rbp)
140916b5d:     	movaps	%xmm2, 0x1e0(%rbp)
140916b64:     	movaps	%xmm1, 0x1d0(%rbp)
140916b6b:     	movaps	%xmm0, 0x1c0(%rbp)
140916b72:     	movq	0x2d1582f(%rip), %rax   # 0x14362c3a8
140916b79:     	movq	(%rax), %rax
140916b7c:     	testq	%rax, %rax
140916b7f:     	je	0x140916c66 <.text+0x915c66>
140916b85:     	leaq	0x1c0(%rbp), %rax
140916b8c:     	movq	%rax, 0x220(%rbp)
140916b93:     	leaq	-0xffa(%rip), %rax      # 0x140915ba0 <.text+0x914ba0>
140916b9a:     	movq	%rax, 0x228(%rbp)
140916ba1:     	leaq	0x2195f78(%rip), %rcx   # 0x142aacb20
140916ba8:     	callq	0x14067dd80 <.text+0x67cd80>
140916bad:     	movups	(%rax), %xmm0
140916bb0:     	movl	0x10(%rax), %eax
140916bb3:     	movq	$0x1, 0x280(%rbp)
140916bbe:     	leaq	0x2195ee3(%rip), %rcx   # 0x142aacaa8
140916bc5:     	movq	%rcx, 0x288(%rbp)
140916bcc:     	movq	$0x1a, 0x290(%rbp)
140916bd7:     	leaq	0x2195f32(%rip), %rdx   # 0x142aacb10
140916bde:     	movq	%rdx, 0x2a0(%rbp)
140916be5:     	movq	$0x1, 0x2a8(%rbp)
140916bf0:     	leaq	0x220(%rbp), %rdx
140916bf7:     	movq	%rdx, 0x2b0(%rbp)
140916bfe:     	movq	$0x1, 0x2b8(%rbp)
140916c09:     	movq	$0x0, 0x2c0(%rbp)
140916c14:     	movq	$0x0, 0x250(%rbp)
140916c1f:     	movq	%rcx, 0x258(%rbp)
140916c26:     	movq	$0x1a, 0x260(%rbp)
140916c31:     	movq	$0x0, 0x268(%rbp)
140916c3c:     	movups	%xmm0, 0x270(%rbp)
140916c43:     	movl	$0x1, 0x298(%rbp)
140916c4d:     	movl	%eax, 0x29c(%rbp)
140916c53:     	leaq	0x48e(%rbp), %rcx
140916c5a:     	leaq	0x250(%rbp), %rdx
140916c61:     	callq	0x140d4a330 <.text+0xd49330>
140916c66:     	leaq	0x418(%rbp), %rcx
140916c6d:     	xorps	%xmm0, %xmm0
140916c70:     	movups	%xmm0, 0x440(%rbp)
140916c77:     	movups	%xmm0, 0x430(%rbp)
140916c7e:     	movq	$0x0, 0x44f(%rbp)
140916c89:     	callq	0x140c102e0 <.text+0xc0f2e0>
140916c8e:     	movl	$0x2a, 0x250(%rbp)
140916c98:     	movl	$0x26, 0x25c(%rbp)
140916ca2:     	movl	$0x14, 0x268(%rbp)
140916cac:     	movl	$0x17, 0x274(%rbp)
140916cb6:     	movl	$0x18, 0x280(%rbp)
140916cc0:     	movl	$0x24, 0x28c(%rbp)
140916cca:     	movl	$0xc3, 0x298(%rbp)
140916cd4:     	movl	$0xc3, 0x2a4(%rbp)
140916cde:     	movl	$0xc3, 0x2b0(%rbp)
140916ce8:     	movl	$0xc3, 0x2bc(%rbp)
140916cf2:     	movl	$0x7, 0x2c8(%rbp)
140916cfc:     	movl	$0x8, 0x2d4(%rbp)
140916d06:     	movl	$0x9, 0x2e0(%rbp)
140916d10:     	movl	$0xa, 0x2ec(%rbp)
140916d1a:     	movl	$0xb, 0x2f8(%rbp)
140916d24:     	movl	$0xc, 0x304(%rbp)
140916d2e:     	movl	$0xd, 0x310(%rbp)
140916d38:     	movl	$0xe, 0x31c(%rbp)
140916d42:     	movl	$0xf, 0x328(%rbp)
140916d4c:     	movl	$0x23, 0x334(%rbp)
140916d56:     	movl	$0x27, 0x340(%rbp)
140916d60:     	movl	$0x2d, 0x34c(%rbp)
140916d6a:     	movl	$0x2b, 0x358(%rbp)
140916d74:     	movl	$0xc3, 0x364(%rbp)
140916d7e:     	movl	$0x38, 0x370(%rbp)
140916d88:     	movl	$0xab, 0x37c(%rbp)
140916d92:     	movl	$0xa5, 0x388(%rbp)
140916d9c:     	movl	$0x1b, 0x394(%rbp)
140916da6:     	movl	$0x49, 0x3a0(%rbp)
140916db0:     	movl	$0x3a, 0x3ac(%rbp)
140916dba:     	movl	$0x51, 0x3b8(%rbp)
140916dc4:     	movl	$0x52, 0x3c4(%rbp)
140916dce:     	movl	$0x50, 0x3d0(%rbp)
140916dd8:     	movl	$0x53, 0x3dc(%rbp)
140916de2:     	movl	$0x4f, 0x3e8(%rbp)
140916dec:     	movl	$0x4e, 0x3f4(%rbp)
140916df6:     	movw	$0x1, 0x40c(%rbp)
140916dff:     	movw	$0x2, 0x410(%rbp)
140916e08:     	movw	$0x1, 0x414(%rbp)
140916e11:     	movl	$0xc3, 0x400(%rbp)
140916e1b:     	movq	$0x0, 0x428(%rbp)
140916e26:     	leaq	0x1c0(%rbp), %rcx
140916e2d:     	callq	0x140887a50 <.text+0x886a50>
140916e32:     	movq	0x480(%rbp), %rdx
140916e39:     	testq	%rdx, %rdx
140916e3c:     	jne	0x140916e62 <.text+0x915e62>
140916e3e:     	jmp	0x140916e74 <.text+0x915e74>
140916e40:     	leaq	0x250(%rbp), %rcx
140916e47:     	leaq	-0x50(%rbp), %rdx
140916e4b:     	movl	$0x208, %r8d            # imm = 0x208
140916e51:     	callq	0x142923c00 <.text+0x2922c00>
140916e56:     	movq	0x480(%rbp), %rdx
140916e5d:     	testq	%rdx, %rdx
140916e60:     	je	0x140916e74 <.text+0x915e74>
140916e62:     	movl	$0x1, %r8d
140916e68:     	movq	0x478(%rbp), %rcx
140916e6f:     	callq	0x140613c20 <.text+0x612c20>
140916e74:     	xorl	%ebx, %ebx
140916e76:     	movb	%bl, 0x48f(%rbp)
140916e7c:     	leaq	0x250(%rbp), %rcx
140916e83:     	callq	0x140917360 <.text+0x916360>
140916e88:     	testq	%rax, %rax
140916e8b:     	je	0x140916f7a <.text+0x915f7a>
140916e91:     	movq	%rax, 0x458(%rbp)
140916e98:     	movq	0x2d15509(%rip), %rax   # 0x14362c3a8
140916e9f:     	movq	(%rax), %rax
140916ea2:     	testq	%rax, %rax
140916ea5:     	je	0x140916f68 <.text+0x915f68>
140916eab:     	movl	%ebx, 0x480(%rbp)
140916eb1:     	leaq	0x458(%rbp), %rax
140916eb8:     	movq	%rax, 0x1c0(%rbp)
140916ebf:     	leaq	0x435d5a(%rip), %rax    # 0x140d4cc20 <.text+0xd4bc20>
140916ec6:     	movq	%rax, 0x1c8(%rbp)
140916ecd:     	leaq	0x2195d04(%rip), %rcx   # 0x142aacbd8
140916ed4:     	callq	0x14067dd80 <.text+0x67cd80>
140916ed9:     	movups	(%rax), %xmm0
140916edc:     	movl	0x10(%rax), %eax
140916edf:     	movq	$0x1, -0x20(%rbp)
140916ee7:     	leaq	0x2195bba(%rip), %rcx   # 0x142aacaa8
140916eee:     	movq	%rcx, -0x18(%rbp)
140916ef2:     	movq	$0x1a, -0x10(%rbp)
140916efa:     	leaq	0x2195cc7(%rip), %rdx   # 0x142aacbc8
140916f01:     	movq	%rdx, (%rbp)
140916f05:     	movq	$0x1, 0x8(%rbp)
140916f0d:     	leaq	0x1c0(%rbp), %rdx
140916f14:     	movq	%rdx, 0x10(%rbp)
140916f18:     	movq	$0x1, 0x18(%rbp)
140916f20:     	movq	$0x0, 0x20(%rbp)
140916f28:     	movq	$0x0, -0x50(%rbp)
140916f30:     	movq	%rcx, -0x48(%rbp)
140916f34:     	movq	$0x1a, -0x40(%rbp)
140916f3c:     	movq	$0x0, -0x38(%rbp)
140916f44:     	movups	%xmm0, -0x30(%rbp)
140916f48:     	movl	$0x1, -0x8(%rbp)
140916f4f:     	movl	%eax, -0x4(%rbp)
140916f52:     	leaq	0x48e(%rbp), %rcx
140916f59:     	leaq	-0x50(%rbp), %rdx
140916f5d:     	callq	0x140d4a330 <.text+0xd49330>
140916f62:     	movl	0x480(%rbp), %ebx
140916f68:     	movb	%bl, 0x48f(%rbp)
140916f6e:     	leaq	0x458(%rbp), %rcx
140916f75:     	callq	0x140d4cc30 <.text+0xd4bc30>
140916f7a:     	leaq	-0x50(%rbp), %rcx
140916f7e:     	leaq	0x250(%rbp), %rdx
140916f85:     	movl	$0x208, %r8d            # imm = 0x208
140916f8b:     	callq	0x142923c00 <.text+0x2922c00>
140916f90:     	movq	0x8(%rsi), %rdi
140916f94:     	testq	%rdi, %rdi
140916f97:     	jne	0x140916f9d <.text+0x915f9d>
140916f99:     	movq	0x10(%rsi), %rdi
140916f9d:     	movq	(%rdi), %rax
140916fa0:     	movq	0x10(%rdi), %rsi
140916fa4:     	subq	%rsi, %rax
140916fa7:     	cmpq	$0x20f, %rax            # imm = 0x20F
140916fad:     	jbe	0x14091700f <.text+0x91600f>
140916faf:     	movq	0x8(%rdi), %rax
140916fb3:     	leaq	0x9c14d6(%rip), %rcx    # 0x1412d8490 <.text+0x12d7490>
140916fba:     	movq	%rcx, (%rax,%rsi)
140916fbe:     	leaq	(%rax,%rsi), %rcx
140916fc2:     	addq	$0x8, %rcx
140916fc6:     	leaq	-0x50(%rbp), %rdx
140916fca:     	movl	$0x208, %r8d            # imm = 0x208
140916fd0:     	callq	0x142923c00 <.text+0x2922c00>
140916fd5:     	addq	$0x210, %rsi            # imm = 0x210
140916fdc:     	movq	%rsi, 0x10(%rdi)
140916fe0:     	testb	%bl, %bl
140916fe2:     	je	0x140917002 <.text+0x916002>
140916fe4:     	movq	0x230(%rbp), %rdx
140916feb:     	testq	%rdx, %rdx
140916fee:     	je	0x140917002 <.text+0x916002>
140916ff0:     	movq	0x238(%rbp), %rcx
140916ff7:     	movl	$0x1, %r8d
140916ffd:     	callq	0x140613c20 <.text+0x612c20>
140917002:     	nop
140917003:     	addq	$0x518, %rsp            # imm = 0x518
14091700a:     	popq	%rbx
14091700b:     	popq	%rdi
14091700c:     	popq	%rsi
14091700d:     	popq	%rbp
14091700e:     	retq
14091700f:     	movb	%bl, 0x48f(%rbp)
140917015:     	movq	$0x1, 0x20(%rsp)
14091701e:     	movl	$0x210, %r8d            # imm = 0x210
140917024:     	movl	$0x1, %r9d
14091702a:     	movq	%rdi, %rcx
14091702d:     	movq	%rsi, %rdx
140917030:     	callq	0x1428a9a60 <.text+0x28a8a60>
140917035:     	jmp	0x140916faf <.text+0x915faf>
14091703a:     	nopw	(%rax,%rax)
140917040:     	movq	%rdx, 0x10(%rsp)
140917045:     	pushq	%rbp
140917046:     	pushq	%rsi
140917047:     	pushq	%rdi
140917048:     	pushq	%rbx
140917049:     	subq	$0x28, %rsp
14091704d:     	leaq	0x80(%rdx), %rbp
140917054:     	movq	-0x50(%rbp), %rdx
140917058:     	testq	%rdx, %rdx
14091705b:     	je	0x14091706c <.text+0x91606c>
14091705d:     	movq	-0x48(%rbp), %rcx
140917061:     	movl	$0x1, %r8d
140917067:     	callq	0x140613c20 <.text+0x612c20>
14091706c:     	movb	$0x0, 0x48f(%rbp)
140917073:     	addq	$0x28, %rsp
140917077:     	popq	%rbx
140917078:     	popq	%rdi
140917079:     	popq	%rsi
14091707a:     	popq	%rbp
14091707b:     	retq
14091707c:     	nopl	(%rax)
140917080:     	movq	%rdx, 0x10(%rsp)
140917085:     	pushq	%rbp
140917086:     	pushq	%rsi
140917087:     	pushq	%rdi
140917088:     	pushq	%rbx
140917089:     	subq	$0x28, %rsp
14091708d:     	leaq	0x80(%rdx), %rbp
140917094:     	cmpq	$0x0, 0x480(%rbp)
14091709c:     	je	0x1409170b7 <.text+0x9160b7>
14091709e:     	movl	$0x1, %r8d
1409170a4:     	movq	0x478(%rbp), %rcx
1409170ab:     	movq	0x480(%rbp), %rdx
1409170b2:     	callq	0x140613c20 <.text+0x612c20>
1409170b7:     	movb	$0x0, 0x48f(%rbp)
1409170be:     	addq	$0x28, %rsp
1409170c2:     	popq	%rbx
1409170c3:     	popq	%rdi
1409170c4:     	popq	%rsi
1409170c5:     	popq	%rbp
1409170c6:     	retq
1409170c7:     	nopw	(%rax,%rax)
1409170d0:     	movq	%rdx, 0x10(%rsp)
1409170d5:     	pushq	%rbp
1409170d6:     	pushq	%rsi
1409170d7:     	pushq	%rdi
1409170d8:     	pushq	%rbx
1409170d9:     	subq	$0x28, %rsp
1409170dd:     	leaq	0x80(%rdx), %rbp
1409170e4:     	leaq	0x1c0(%rbp), %rcx
1409170eb:     	callq	0x140887a50 <.text+0x886a50>
1409170f0:     	nop
1409170f1:     	addq	$0x28, %rsp
1409170f5:     	popq	%rbx
1409170f6:     	popq	%rdi
1409170f7:     	popq	%rsi
1409170f8:     	popq	%rbp
1409170f9:     	retq
1409170fa:     	nopw	(%rax,%rax)
140917100:     	movq	%rdx, 0x10(%rsp)
140917105:     	pushq	%rbp
140917106:     	pushq	%rsi
140917107:     	pushq	%rdi
140917108:     	pushq	%rbx
140917109:     	subq	$0x28, %rsp
14091710d:     	leaq	0x80(%rdx), %rbp
140917114:     	leaq	0x1c0(%rbp), %rcx
14091711b:     	callq	0x1405e7a00 <.text+0x5e6a00>
140917120:     	movb	$0x0, 0x48f(%rbp)
140917127:     	addq	$0x28, %rsp
14091712b:     	popq	%rbx
14091712c:     	popq	%rdi
14091712d:     	popq	%rsi
14091712e:     	popq	%rbp
14091712f:     	retq
140917130:     	movq	%rdx, 0x10(%rsp)
140917135:     	pushq	%rbp
140917136:     	pushq	%rsi
140917137:     	pushq	%rdi
140917138:     	pushq	%rbx
140917139:     	subq	$0x28, %rsp
14091713d:     	leaq	0x80(%rdx), %rbp
140917144:     	movq	0x480(%rbp), %rcx
14091714b:     	movq	0x478(%rbp), %rax
140917152:     	movq	0x8(%rax), %rdx
140917156:     	testq	%rdx, %rdx
140917159:     	je	0x14091716b <.text+0x91616b>
14091715b:     	movq	0x478(%rbp), %rax
140917162:     	movq	0x10(%rax), %r8
140917166:     	callq	0x140613c20 <.text+0x612c20>
14091716b:     	movl	$0x18, %edx
140917170:     	movl	$0x8, %r8d
140917176:     	movq	0x470(%rbp), %rcx
14091717d:     	callq	0x140613c20 <.text+0x612c20>
140917182:     	movb	$0x0, 0x48f(%rbp)
140917189:     	addq	$0x28, %rsp
14091718d:     	popq	%rbx
14091718e:     	popq	%rdi
14091718f:     	popq	%rsi
140917190:     	popq	%rbp
140917191:     	retq
140917192:     	nopw	%cs:(%rax,%rax)
1409171a0:     	movq	%rdx, 0x10(%rsp)
1409171a5:     	pushq	%rbp
1409171a6:     	pushq	%rsi
1409171a7:     	pushq	%rdi
1409171a8:     	pushq	%rbx
1409171a9:     	subq	$0x28, %rsp
1409171ad:     	leaq	0x80(%rdx), %rbp
1409171b4:     	leaq	0x458(%rbp), %rcx
1409171bb:     	callq	0x140d4cc30 <.text+0xd4bc30>
1409171c0:     	movl	0x480(%rbp), %eax
1409171c6:     	movb	%al, 0x48f(%rbp)
1409171cc:     	addq	$0x28, %rsp
1409171d0:     	popq	%rbx
1409171d1:     	popq	%rdi
1409171d2:     	popq	%rsi
1409171d3:     	popq	%rbp
1409171d4:     	retq
1409171d5:     	nopw	%cs:(%rax,%rax)
1409171e0:     	movq	%rdx, 0x10(%rsp)
1409171e5:     	pushq	%rbp
1409171e6:     	pushq	%rsi
1409171e7:     	pushq	%rdi
1409171e8:     	pushq	%rbx
1409171e9:     	subq	$0x28, %rsp
1409171ed:     	leaq	0x80(%rdx), %rbp
1409171f4:     	testb	$0x1, 0x48f(%rbp)
1409171fb:     	je	0x14091721b <.text+0x91621b>
1409171fd:     	movq	0x230(%rbp), %rdx
140917204:     	testq	%rdx, %rdx
140917207:     	je	0x14091721b <.text+0x91621b>
140917209:     	movq	0x238(%rbp), %rcx
140917210:     	movl	$0x1, %r8d
140917216:     	callq	0x140613c20 <.text+0x612c20>
14091721b:     	nop
14091721c:     	addq	$0x28, %rsp
140917220:     	popq	%rbx
140917221:     	popq	%rdi
140917222:     	popq	%rsi
140917223:     	popq	%rbp
140917224:     	retq
140917225:     	int3
140917226:     	int3
140917227:     	int3
140917228:     	int3
140917229:     	int3
14091722a:     	int3
14091722b:     	int3
14091722c:     	int3
14091722d:     	int3
14091722e:     	int3
14091722f:     	int3
