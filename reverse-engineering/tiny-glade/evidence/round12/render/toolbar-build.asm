
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14068d610:     	pushq	%rbp
14068d611:     	pushq	%rsi
14068d612:     	pushq	%rdi
14068d613:     	subq	$0x1f0, %rsp            # imm = 0x1F0
14068d61a:     	leaq	0x80(%rsp), %rbp
14068d622:     	movq	$-0x2, 0x168(%rbp)
14068d62d:     	movq	%rdx, %rsi
14068d630:     	leaq	0x10(%rbp), %rdi
14068d634:     	movq	%rdi, %rcx
14068d637:     	callq	0x140fe7410 <.text+0xfe6410>
14068d63c:     	movq	$0x0, 0xa8(%rbp)
14068d647:     	movq	$0x8, 0xb0(%rbp)
14068d652:     	movq	$0x0, 0xb8(%rbp)
14068d65d:     	movq	%rsi, %rcx
14068d660:     	movq	%rdi, %rdx
14068d663:     	callq	0x140fd02c0 <.text+0xfcf2c0>
14068d668:     	xorps	%xmm1, %xmm1
14068d66b:     	movq	%rax, %rcx
14068d66e:     	callq	0x140fd25b0 <.text+0xfd15b0>
14068d673:     	leaq	0x38(%rsi), %rdi
14068d677:     	movabsq	$-0x4f18b7061c5534a9, %rdx # imm = 0xB0E748F9E3AACB57
14068d681:     	movabsq	$0x757117fa8299e96e, %r8 # imm = 0x757117FA8299E96E
14068d68b:     	movq	%rdi, %rcx
14068d68e:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d693:     	testq	%rax, %rax
14068d696:     	je	0x14068d6cb <.text+0x68c6cb>
14068d698:     	cmpq	%rdx, 0x218(%rsi)
14068d69f:     	jbe	0x14068d6cb <.text+0x68c6cb>
14068d6a1:     	movq	0x210(%rsi), %rax
14068d6a8:     	movq	(%rax,%rdx,8), %rax
14068d6ac:     	testq	%rax, %rax
14068d6af:     	je	0x14068d6cb <.text+0x68c6cb>
14068d6b1:     	movq	0x1e0(%rsi), %rcx
14068d6b8:     	notq	%rax
14068d6bb:     	leaq	(%rax,%rax,2), %rax
14068d6bf:     	shlq	$0x5, %rax
14068d6c3:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d6c9:     	jne	0x14068d6d3 <.text+0x68c6d3>
14068d6cb:     	movq	%rsi, %rcx
14068d6ce:     	callq	0x140fea370 <.text+0xfe9370>
14068d6d3:     	movabsq	$0x4d2642e55f9027d1, %rdx # imm = 0x4D2642E55F9027D1
14068d6dd:     	movabsq	$0x4d297e300d4880fc, %r8 # imm = 0x4D297E300D4880FC
14068d6e7:     	movq	%rdi, %rcx
14068d6ea:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d6ef:     	testq	%rax, %rax
14068d6f2:     	je	0x14068d727 <.text+0x68c727>
14068d6f4:     	cmpq	%rdx, 0x218(%rsi)
14068d6fb:     	jbe	0x14068d727 <.text+0x68c727>
14068d6fd:     	movq	0x210(%rsi), %rax
14068d704:     	movq	(%rax,%rdx,8), %rax
14068d708:     	testq	%rax, %rax
14068d70b:     	je	0x14068d727 <.text+0x68c727>
14068d70d:     	movq	0x1e0(%rsi), %rcx
14068d714:     	notq	%rax
14068d717:     	leaq	(%rax,%rax,2), %rax
14068d71b:     	shlq	$0x5, %rax
14068d71f:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d725:     	jne	0x14068d72f <.text+0x68c72f>
14068d727:     	movq	%rsi, %rcx
14068d72a:     	callq	0x140feb600 <.text+0xfea600>
14068d72f:     	movabsq	$0x189e38b5bedf98fb, %rdx # imm = 0x189E38B5BEDF98FB
14068d739:     	movabsq	$-0x2cc096ed7da3d66d, %r8 # imm = 0xD33F6912825C2993
14068d743:     	movq	%rdi, %rcx
14068d746:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d74b:     	testq	%rax, %rax
14068d74e:     	je	0x14068d783 <.text+0x68c783>
14068d750:     	cmpq	%rdx, 0x218(%rsi)
14068d757:     	jbe	0x14068d783 <.text+0x68c783>
14068d759:     	movq	0x210(%rsi), %rax
14068d760:     	movq	(%rax,%rdx,8), %rax
14068d764:     	testq	%rax, %rax
14068d767:     	je	0x14068d783 <.text+0x68c783>
14068d769:     	movq	0x1e0(%rsi), %rcx
14068d770:     	notq	%rax
14068d773:     	leaq	(%rax,%rax,2), %rax
14068d777:     	shlq	$0x5, %rax
14068d77b:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d781:     	jne	0x14068d78b <.text+0x68c78b>
14068d783:     	movq	%rsi, %rcx
14068d786:     	callq	0x140fe8490 <.text+0xfe7490>
14068d78b:     	movabsq	$0x1ce85a7892ad8401, %rdx # imm = 0x1CE85A7892AD8401
14068d795:     	movabsq	$-0x2a95b1feb9958c5e, %r8 # imm = 0xD56A4E01466A73A2
14068d79f:     	movq	%rdi, %rcx
14068d7a2:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d7a7:     	testq	%rax, %rax
14068d7aa:     	je	0x14068d7df <.text+0x68c7df>
14068d7ac:     	cmpq	%rdx, 0x218(%rsi)
14068d7b3:     	jbe	0x14068d7df <.text+0x68c7df>
14068d7b5:     	movq	0x210(%rsi), %rax
14068d7bc:     	movq	(%rax,%rdx,8), %rax
14068d7c0:     	testq	%rax, %rax
14068d7c3:     	je	0x14068d7df <.text+0x68c7df>
14068d7c5:     	movq	0x1e0(%rsi), %rcx
14068d7cc:     	notq	%rax
14068d7cf:     	leaq	(%rax,%rax,2), %rax
14068d7d3:     	shlq	$0x5, %rax
14068d7d7:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d7dd:     	jne	0x14068d7e7 <.text+0x68c7e7>
14068d7df:     	movq	%rsi, %rcx
14068d7e2:     	callq	0x140fea9a0 <.text+0xfe99a0>
14068d7e7:     	movabsq	$0x62bb6cad4efc598, %rdx # imm = 0x62BB6CAD4EFC598
14068d7f1:     	movabsq	$0x22acd43c16b1856e, %r8 # imm = 0x22ACD43C16B1856E
14068d7fb:     	movq	%rdi, %rcx
14068d7fe:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d803:     	testq	%rax, %rax
14068d806:     	je	0x14068d83b <.text+0x68c83b>
14068d808:     	cmpq	%rdx, 0x218(%rsi)
14068d80f:     	jbe	0x14068d83b <.text+0x68c83b>
14068d811:     	movq	0x210(%rsi), %rax
14068d818:     	movq	(%rax,%rdx,8), %rax
14068d81c:     	testq	%rax, %rax
14068d81f:     	je	0x14068d83b <.text+0x68c83b>
14068d821:     	movq	0x1e0(%rsi), %rcx
14068d828:     	notq	%rax
14068d82b:     	leaq	(%rax,%rax,2), %rax
14068d82f:     	shlq	$0x5, %rax
14068d833:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d839:     	jne	0x14068d843 <.text+0x68c843>
14068d83b:     	movq	%rsi, %rcx
14068d83e:     	callq	0x140fe8ac0 <.text+0xfe7ac0>
14068d843:     	movabsq	$0xd150ed151a658bf, %rdx # imm = 0xD150ED151A658BF
14068d84d:     	movabsq	$0x1dd614aa5ea5002, %r8 # imm = 0x1DD614AA5EA5002
14068d857:     	movq	%rdi, %rcx
14068d85a:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d85f:     	testq	%rax, %rax
14068d862:     	je	0x14068d897 <.text+0x68c897>
14068d864:     	cmpq	%rdx, 0x218(%rsi)
14068d86b:     	jbe	0x14068d897 <.text+0x68c897>
14068d86d:     	movq	0x210(%rsi), %rax
14068d874:     	movq	(%rax,%rdx,8), %rax
14068d878:     	testq	%rax, %rax
14068d87b:     	je	0x14068d897 <.text+0x68c897>
14068d87d:     	movq	0x1e0(%rsi), %rcx
14068d884:     	notq	%rax
14068d887:     	leaq	(%rax,%rax,2), %rax
14068d88b:     	shlq	$0x5, %rax
14068d88f:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d895:     	jne	0x14068d89f <.text+0x68c89f>
14068d897:     	movq	%rsi, %rcx
14068d89a:     	callq	0x140fe90f0 <.text+0xfe80f0>
14068d89f:     	movabsq	$0x5eaaca67868e0174, %rdx # imm = 0x5EAACA67868E0174
14068d8a9:     	movabsq	$0x67675b60a5763276, %r8 # imm = 0x67675B60A5763276
14068d8b3:     	movq	%rdi, %rcx
14068d8b6:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d8bb:     	testq	%rax, %rax
14068d8be:     	je	0x14068d8f3 <.text+0x68c8f3>
14068d8c0:     	cmpq	%rdx, 0x218(%rsi)
14068d8c7:     	jbe	0x14068d8f3 <.text+0x68c8f3>
14068d8c9:     	movq	0x210(%rsi), %rax
14068d8d0:     	movq	(%rax,%rdx,8), %rax
14068d8d4:     	testq	%rax, %rax
14068d8d7:     	je	0x14068d8f3 <.text+0x68c8f3>
14068d8d9:     	movq	0x1e0(%rsi), %rcx
14068d8e0:     	notq	%rax
14068d8e3:     	leaq	(%rax,%rax,2), %rax
14068d8e7:     	shlq	$0x5, %rax
14068d8eb:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d8f1:     	jne	0x14068d8fb <.text+0x68c8fb>
14068d8f3:     	movq	%rsi, %rcx
14068d8f6:     	callq	0x140feafd0 <.text+0xfe9fd0>
14068d8fb:     	movabsq	$0x7e239f0526778219, %rdx # imm = 0x7E239F0526778219
14068d905:     	movabsq	$0x52d214c46f437022, %r8 # imm = 0x52D214C46F437022
14068d90f:     	movq	%rdi, %rcx
14068d912:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d917:     	testq	%rax, %rax
14068d91a:     	je	0x14068d94f <.text+0x68c94f>
14068d91c:     	cmpq	%rdx, 0x218(%rsi)
14068d923:     	jbe	0x14068d94f <.text+0x68c94f>
14068d925:     	movq	0x210(%rsi), %rax
14068d92c:     	movq	(%rax,%rdx,8), %rax
14068d930:     	testq	%rax, %rax
14068d933:     	je	0x14068d94f <.text+0x68c94f>
14068d935:     	movq	0x1e0(%rsi), %rcx
14068d93c:     	notq	%rax
14068d93f:     	leaq	(%rax,%rax,2), %rax
14068d943:     	shlq	$0x5, %rax
14068d947:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d94d:     	jne	0x14068d957 <.text+0x68c957>
14068d94f:     	movq	%rsi, %rcx
14068d952:     	callq	0x140fe9710 <.text+0xfe8710>
14068d957:     	movabsq	$0x73f303a4e7b78adc, %rdx # imm = 0x73F303A4E7B78ADC
14068d961:     	movabsq	$-0x12b0c8d44b15ec92, %r8 # imm = 0xED4F372BB4EA136E
14068d96b:     	movq	%rdi, %rcx
14068d96e:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14068d973:     	testq	%rax, %rax
14068d976:     	je	0x14068d9ab <.text+0x68c9ab>
14068d978:     	cmpq	%rdx, 0x218(%rsi)
14068d97f:     	jbe	0x14068d9ab <.text+0x68c9ab>
14068d981:     	movq	0x210(%rsi), %rax
14068d988:     	movq	(%rax,%rdx,8), %rax
14068d98c:     	testq	%rax, %rax
14068d98f:     	je	0x14068d9ab <.text+0x68c9ab>
14068d991:     	movq	0x1e0(%rsi), %rcx
14068d998:     	notq	%rax
14068d99b:     	leaq	(%rax,%rax,2), %rax
14068d99f:     	shlq	$0x5, %rax
14068d9a3:     	cmpq	$0x0, 0x20(%rcx,%rax)
14068d9a9:     	jne	0x14068d9b3 <.text+0x68c9b3>
14068d9ab:     	movq	%rsi, %rcx
14068d9ae:     	callq	0x140fe7e60 <.text+0xfe6e60>
14068d9b3:     	movq	%rsi, %rcx
14068d9b6:     	callq	0x140fd2b00 <.text+0xfd1b00>
14068d9bb:     	leaq	0x10(%rbp), %rcx
14068d9bf:     	callq	0x140fee0e0 <.text+0xfed0e0>
14068d9c4:     	movb	$0x7, 0xf0(%rbp)
14068d9cb:     	movq	0x2f9e99e(%rip), %rcx   # 0x14362c370
14068d9d2:     	leaq	0x253419f(%rip), %r8    # 0x142bc1b78
14068d9d9:     	leaq	0xf0(%rbp), %rdx
14068d9e0:     	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068d9e5:     	movq	%rdx, %r8
14068d9e8:     	leaq	0x10(%rbp), %rcx
14068d9ec:     	movq	%rax, %rdx
14068d9ef:     	callq	0x140fddf80 <.text+0xfdcf80>
14068d9f4:     	movups	0x70(%rbp), %xmm0
14068d9f8:     	movaps	%xmm0, (%rbp)
14068d9fc:     	movups	0x60(%rbp), %xmm0
14068da00:     	movaps	%xmm0, -0x10(%rbp)
14068da04:     	movups	0x50(%rbp), %xmm0
14068da08:     	movaps	%xmm0, -0x20(%rbp)
14068da0c:     	movups	0x10(%rbp), %xmm0
14068da10:     	movups	0x20(%rbp), %xmm1
14068da14:     	movups	0x30(%rbp), %xmm2
14068da18:     	movups	0x40(%rbp), %xmm3
14068da1c:     	movaps	%xmm3, -0x30(%rbp)
14068da20:     	movaps	%xmm2, -0x40(%rbp)
14068da24:     	movaps	%xmm1, -0x50(%rbp)
14068da28:     	movaps	%xmm0, -0x60(%rbp)
14068da2c:     	leaq	0xf0(%rbp), %rcx
14068da33:     	callq	0x140fef7c0 <.text+0xfee7c0>
14068da38:     	movb	$0x24, 0x167(%rbp)
14068da3f:     	movq	0x2f9e92a(%rip), %rcx   # 0x14362c370
14068da46:     	leaq	0x253412b(%rip), %r8    # 0x142bc1b78
14068da4d:     	leaq	0x167(%rbp), %rdx
14068da54:     	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068da59:     	movq	%rdx, %r8
14068da5c:     	leaq	0xf0(%rbp), %rcx
14068da63:     	movq	%rax, %rdx
14068da66:     	callq	0x140fddf80 <.text+0xfdcf80>
14068da6b:     	movups	0x150(%rbp), %xmm0
14068da72:     	movaps	%xmm0, 0xe0(%rbp)
14068da79:     	movups	0x140(%rbp), %xmm0
14068da80:     	movaps	%xmm0, 0xd0(%rbp)
14068da87:     	movups	0x130(%rbp), %xmm0
14068da8e:     	movaps	%xmm0, 0xc0(%rbp)
14068da95:     	movups	0xf0(%rbp), %xmm0
14068da9c:     	movups	0x100(%rbp), %xmm1
14068daa3:     	movups	0x110(%rbp), %xmm2
14068daaa:     	movups	0x120(%rbp), %xmm3
14068dab1:     	movaps	%xmm3, 0xb0(%rbp)
14068dab8:     	movaps	%xmm2, 0xa0(%rbp)
14068dabf:     	movaps	%xmm1, 0x90(%rbp)
14068dac6:     	movaps	%xmm0, 0x80(%rbp)
14068dacd:     	movaps	-0x60(%rbp), %xmm0
14068dad1:     	movaps	-0x50(%rbp), %xmm1
14068dad5:     	movaps	-0x40(%rbp), %xmm2
14068dad9:     	movaps	-0x30(%rbp), %xmm3
14068dadd:     	movaps	%xmm0, 0x10(%rbp)
14068dae1:     	movaps	%xmm1, 0x20(%rbp)
14068dae5:     	movaps	%xmm2, 0x30(%rbp)
14068dae9:     	movaps	%xmm3, 0x40(%rbp)
14068daed:     	movaps	-0x20(%rbp), %xmm0
14068daf1:     	movaps	%xmm0, 0x50(%rbp)
14068daf5:     	movaps	-0x10(%rbp), %xmm0
14068daf9:     	movaps	%xmm0, 0x60(%rbp)
14068dafd:     	movaps	(%rbp), %xmm0
14068db01:     	movaps	%xmm0, 0x70(%rbp)
14068db05:     	leaq	0x10(%rbp), %rdx
14068db09:     	movq	%rsi, %rcx
14068db0c:     	callq	0x140fd2c40 <.text+0xfd1c40>
14068db11:     	leaq	0x10(%rbp), %rcx
14068db15:     	callq	0x140fed8a0 <.text+0xfec8a0>
14068db1a:     	movb	$0x2c, -0x60(%rbp)
14068db1e:     	movq	0x2f9e84b(%rip), %rcx   # 0x14362c370
14068db25:     	leaq	0x253404c(%rip), %r8    # 0x142bc1b78
14068db2c:     	leaq	-0x60(%rbp), %rdx
14068db30:     	callq	0x140fdc2a0 <.text+0xfdb2a0>
14068db35:     	movq	%rdx, %r8
14068db38:     	leaq	0x10(%rbp), %rcx
14068db3c:     	movq	%rax, %rdx
14068db3f:     	callq	0x140fddf80 <.text+0xfdcf80>
14068db44:     	movups	0x70(%rbp), %xmm0
14068db48:     	movaps	%xmm0, 0x150(%rbp)
14068db4f:     	movups	0x60(%rbp), %xmm0
14068db53:     	movaps	%xmm0, 0x140(%rbp)
14068db5a:     	movups	0x50(%rbp), %xmm0
14068db5e:     	movaps	%xmm0, 0x130(%rbp)
14068db65:     	movups	0x10(%rbp), %xmm0
14068db69:     	movups	0x20(%rbp), %xmm1
14068db6d:     	movups	0x30(%rbp), %xmm2
14068db71:     	movups	0x40(%rbp), %xmm3
14068db75:     	movaps	%xmm3, 0x120(%rbp)
14068db7c:     	movaps	%xmm2, 0x110(%rbp)
14068db83:     	movaps	%xmm1, 0x100(%rbp)
14068db8a:     	movaps	%xmm0, 0xf0(%rbp)
14068db91:     	leaq	0xf0(%rbp), %rdx
14068db98:     	movq	%rsi, %rcx
14068db9b:     	callq	0x140fd2dd0 <.text+0xfd1dd0>
14068dba0:     	nop
14068dba1:     	addq	$0x1f0, %rsp            # imm = 0x1F0
14068dba8:     	popq	%rdi
14068dba9:     	popq	%rsi
14068dbaa:     	popq	%rbp
14068dbab:     	retq
14068dbac:     	nopl	(%rax)
14068dbb0:     	movq	%rdx, 0x10(%rsp)
14068dbb5:     	pushq	%rbp
14068dbb6:     	pushq	%rsi
14068dbb7:     	pushq	%rdi
14068dbb8:     	subq	$0x20, %rsp
14068dbbc:     	leaq	0x80(%rdx), %rbp
14068dbc3:     	leaq	0x10(%rbp), %rcx
14068dbc7:     	callq	0x140029880 <.text+0x28880>
14068dbcc:     	nop
14068dbcd:     	addq	$0x20, %rsp
14068dbd1:     	popq	%rdi
14068dbd2:     	popq	%rsi
14068dbd3:     	popq	%rbp
14068dbd4:     	retq
14068dbd5:     	nopw	%cs:(%rax,%rax)
14068dbe0:     	movq	%rdx, 0x10(%rsp)
14068dbe5:     	pushq	%rbp
14068dbe6:     	pushq	%rsi
14068dbe7:     	pushq	%rdi
14068dbe8:     	subq	$0x20, %rsp
14068dbec:     	leaq	0x80(%rdx), %rbp
14068dbf3:     	leaq	-0x60(%rbp), %rcx
14068dbf7:     	callq	0x140029880 <.text+0x28880>
14068dbfc:     	nop
14068dbfd:     	addq	$0x20, %rsp
14068dc01:     	popq	%rdi
14068dc02:     	popq	%rsi
14068dc03:     	popq	%rbp
14068dc04:     	retq
14068dc05:     	nopw	%cs:(%rax,%rax)
14068dc10:     	movq	%rdx, 0x10(%rsp)
14068dc15:     	pushq	%rbp
14068dc16:     	pushq	%rsi
14068dc17:     	pushq	%rdi
14068dc18:     	subq	$0x20, %rsp
14068dc1c:     	leaq	0x80(%rdx), %rbp
14068dc23:     	leaq	0xf0(%rbp), %rcx
14068dc2a:     	callq	0x140029880 <.text+0x28880>
14068dc2f:     	nop
14068dc30:     	addq	$0x20, %rsp
14068dc34:     	popq	%rdi
14068dc35:     	popq	%rsi
14068dc36:     	popq	%rbp
14068dc37:     	retq
14068dc38:     	nopl	(%rax,%rax)
14068dc40:     	movq	%rdx, 0x10(%rsp)
14068dc45:     	pushq	%rbp
14068dc46:     	pushq	%rsi
14068dc47:     	pushq	%rdi
14068dc48:     	subq	$0x20, %rsp
14068dc4c:     	leaq	0x80(%rdx), %rbp
14068dc53:     	leaq	0x10(%rbp), %rcx
14068dc57:     	callq	0x140029880 <.text+0x28880>
14068dc5c:     	nop
14068dc5d:     	addq	$0x20, %rsp
14068dc61:     	popq	%rdi
14068dc62:     	popq	%rsi
14068dc63:     	popq	%rbp
14068dc64:     	retq
