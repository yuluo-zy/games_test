
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140b47840:     	pushq	%rbp
140b47841:     	pushq	%rsi
140b47842:     	pushq	%rdi
140b47843:     	subq	$0x940, %rsp            # imm = 0x940
140b4784a:     	leaq	0x80(%rsp), %rbp
140b47852:     	movq	$-0x2, 0x8b8(%rbp)
140b4785d:     	movq	%rdx, %rcx
140b47860:     	movq	$0x0, 0x3f0(%rbp)
140b4786b:     	movq	$0x4, 0x3f8(%rbp)
140b47876:     	movq	$0x0, 0x400(%rbp)
140b47881:     	leaq	0x3f0(%rbp), %rdx
140b47888:     	callq	0x14154ad00 <.text+0x1549d00>
140b4788d:     	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
140b47897:     	movq	%rcx, 0x408(%rbp)
140b4789e:     	movq	$0x0, 0x3f0(%rbp)
140b478a9:     	movq	$0x8, 0x3f8(%rbp)
140b478b4:     	movq	$0x0, 0x400(%rbp)
140b478bf:     	movb	$0x0, 0x488(%rbp)
140b478c6:     	leaq	0x3f0(%rbp), %rdx
140b478cd:     	movq	%rax, %rcx
140b478d0:     	callq	0x141543b60 <.text+0x1542b60>
140b478d5:     	movq	%rax, %rsi
140b478d8:     	movq	%rax, %rcx
140b478db:     	callq	0x140b66eb0 <.text+0xb65eb0>
140b478e0:     	leaq	0x840(%rbp), %rcx
140b478e7:     	callq	0x1413288d0 <.text+0x13278d0>
140b478ec:     	leaq	-0x58(%rbp), %rcx
140b478f0:     	callq	0x1415650e0 <.text+0x15640e0>
140b478f5:     	movb	$0x1, 0x8b7(%rbp)
140b478fc:     	leaq	0x380(%rbp), %rcx
140b47903:     	callq	0x1413280a0 <.text+0x13270a0>
140b47908:     	leaq	0x3f0(%rbp), %rdi
140b4790f:     	leaq	-0x58(%rbp), %rdx
140b47913:     	movl	$0x3d8, %r8d            # imm = 0x3D8
140b47919:     	movq	%rdi, %rcx
140b4791c:     	callq	0x142923c00 <.text+0x2922c00>
140b47921:     	movb	$0x0, 0x8b7(%rbp)
140b47928:     	leaq	0x7d0(%rbp), %rcx
140b4792f:     	leaq	0x380(%rbp), %rdx
140b47936:     	movq	%rdi, %r8
140b47939:     	callq	0x14144e100 <.text+0x144d100>
140b4793e:     	movups	0x840(%rbp), %xmm0
140b47945:     	movups	0x850(%rbp), %xmm1
140b4794c:     	movups	0x860(%rbp), %xmm2
140b47953:     	movups	0x870(%rbp), %xmm3
140b4795a:     	movaps	%xmm0, 0x3f0(%rbp)
140b47961:     	movaps	%xmm1, 0x400(%rbp)
140b47968:     	movaps	%xmm2, 0x410(%rbp)
140b4796f:     	movaps	%xmm3, 0x420(%rbp)
140b47976:     	movups	0x880(%rbp), %xmm0
140b4797d:     	movaps	%xmm0, 0x430(%rbp)
140b47984:     	movups	0x890(%rbp), %xmm0
140b4798b:     	movaps	%xmm0, 0x440(%rbp)
140b47992:     	movups	0x8a0(%rbp), %xmm0
140b47999:     	movaps	%xmm0, 0x450(%rbp)
140b479a0:     	movups	0x7d0(%rbp), %xmm0
140b479a7:     	movups	0x7e0(%rbp), %xmm1
140b479ae:     	movups	0x7f0(%rbp), %xmm2
140b479b5:     	movups	0x800(%rbp), %xmm3
140b479bc:     	movaps	%xmm0, 0x460(%rbp)
140b479c3:     	movaps	%xmm1, 0x470(%rbp)
140b479ca:     	movaps	%xmm2, 0x480(%rbp)
140b479d1:     	movaps	%xmm3, 0x490(%rbp)
140b479d8:     	movups	0x810(%rbp), %xmm0
140b479df:     	movaps	%xmm0, 0x4a0(%rbp)
140b479e6:     	movups	0x820(%rbp), %xmm0
140b479ed:     	movaps	%xmm0, 0x4b0(%rbp)
140b479f4:     	movups	0x830(%rbp), %xmm0
140b479fb:     	movaps	%xmm0, 0x4c0(%rbp)
140b47a02:     	leaq	0x3f0(%rbp), %rdx
140b47a09:     	movq	%rsi, %rcx
140b47a0c:     	callq	0x140b679c0 <.text+0xb669c0>
140b47a11:     	movq	%rsi, %rdi
140b47a14:     	addq	$0x38, %rdi
140b47a18:     	movabsq	$0x2e360f7300b870d3, %rdx # imm = 0x2E360F7300B870D3
140b47a22:     	movabsq	$-0x740e4978abd7e9e1, %r8 # imm = 0x8BF1B6875428161F
140b47a2c:     	movq	%rdi, %rcx
140b47a2f:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140b47a34:     	testq	%rax, %rax
140b47a37:     	je	0x140b47a6c <.text+0xb46a6c>
140b47a39:     	cmpq	%rdx, 0x218(%rsi)
140b47a40:     	jbe	0x140b47a6c <.text+0xb46a6c>
140b47a42:     	movq	0x210(%rsi), %rax
140b47a49:     	movq	(%rax,%rdx,8), %rax
140b47a4d:     	testq	%rax, %rax
140b47a50:     	je	0x140b47a6c <.text+0xb46a6c>
140b47a52:     	movq	0x1e0(%rsi), %rcx
140b47a59:     	notq	%rax
140b47a5c:     	leaq	(%rax,%rax,2), %rax
140b47a60:     	shlq	$0x5, %rax
140b47a64:     	cmpq	$0x0, 0x20(%rcx,%rax)
140b47a6a:     	jne	0x140b47a74 <.text+0xb46a74>
140b47a6c:     	movq	%rsi, %rcx
140b47a6f:     	callq	0x141516b30 <.text+0x1515b30>
140b47a74:     	movabsq	$-0x70ad28fb09d00bb5, %rdx # imm = 0x8F52D704F62FF44B
140b47a7e:     	movabsq	$-0x26d3e6e50131216a, %r8 # imm = 0xD92C191AFECEDE96
140b47a88:     	movq	%rdi, %rcx
140b47a8b:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140b47a90:     	testq	%rax, %rax
140b47a93:     	je	0x140b47ac8 <.text+0xb46ac8>
140b47a95:     	cmpq	%rdx, 0x218(%rsi)
140b47a9c:     	jbe	0x140b47ac8 <.text+0xb46ac8>
140b47a9e:     	movq	0x210(%rsi), %rax
140b47aa5:     	movq	(%rax,%rdx,8), %rax
140b47aa9:     	testq	%rax, %rax
140b47aac:     	je	0x140b47ac8 <.text+0xb46ac8>
140b47aae:     	movq	0x1e0(%rsi), %rcx
140b47ab5:     	notq	%rax
140b47ab8:     	leaq	(%rax,%rax,2), %rax
140b47abc:     	shlq	$0x5, %rax
140b47ac0:     	cmpq	$0x0, 0x20(%rcx,%rax)
140b47ac6:     	jne	0x140b47ad0 <.text+0xb46ad0>
140b47ac8:     	movq	%rsi, %rcx
140b47acb:     	callq	0x141510e60 <.text+0x150fe60>
140b47ad0:     	movabsq	$-0x36e6aa175b4c1127, %rdx # imm = 0xC91955E8A4B3EED9
140b47ada:     	movabsq	$-0x6748af215ec04740, %r8 # imm = 0x98B750DEA13FB8C0
140b47ae4:     	movq	%rdi, %rcx
140b47ae7:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140b47aec:     	testq	%rax, %rax
140b47aef:     	je	0x140b47b2f <.text+0xb46b2f>
140b47af1:     	cmpq	%rdx, 0x218(%rsi)
140b47af8:     	jbe	0x140b47b2f <.text+0xb46b2f>
140b47afa:     	movq	0x210(%rsi), %rax
140b47b01:     	movq	(%rax,%rdx,8), %rax
140b47b05:     	testq	%rax, %rax
140b47b08:     	je	0x140b47b2f <.text+0xb46b2f>
140b47b0a:     	movq	0x1e0(%rsi), %rcx
140b47b11:     	notq	%rax
140b47b14:     	leaq	(%rax,%rax,2), %rax
140b47b18:     	shlq	$0x5, %rax
140b47b1c:     	cmpq	$0x0, 0x20(%rcx,%rax)
140b47b22:     	je	0x140b47b2f <.text+0xb46b2f>
140b47b24:     	addq	$0x940, %rsp            # imm = 0x940
140b47b2b:     	popq	%rdi
140b47b2c:     	popq	%rsi
140b47b2d:     	popq	%rbp
140b47b2e:     	retq
140b47b2f:     	movq	%rsi, %rcx
140b47b32:     	addq	$0x940, %rsp            # imm = 0x940
140b47b39:     	popq	%rdi
140b47b3a:     	popq	%rsi
140b47b3b:     	popq	%rbp
140b47b3c:     	jmp	0x14151ce30 <.text+0x151be30>
