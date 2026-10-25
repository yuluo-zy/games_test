
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409438a0:     	pushq	%rbp
1409438a1:     	pushq	%r14
1409438a3:     	pushq	%rsi
1409438a4:     	pushq	%rdi
1409438a5:     	pushq	%rbx
1409438a6:     	subq	$0x420, %rsp            # imm = 0x420
1409438ad:     	leaq	0x80(%rsp), %rbp
1409438b5:     	movq	$-0x2, 0x398(%rbp)
1409438c0:     	movq	%rcx, %rsi
1409438c3:     	leaq	0x216f076(%rip), %r8    # 0x142ab2940
1409438ca:     	leaq	0x10(%rbp), %rcx
1409438ce:     	movb	$0xe, %dl
1409438d0:     	callq	0x141738630 <.text+0x1737630>
1409438d5:     	leaq	0xf0(%rbp), %rcx
1409438dc:     	callq	0x141737fb0 <.text+0x1736fb0>
1409438e1:     	movb	$0x23, 0x2b0(%rbp)
1409438e8:     	movq	0x2ce8a81(%rip), %rcx   # 0x14362c370
1409438ef:     	leaq	0x231c46a(%rip), %r8    # 0x142c5fd60
1409438f6:     	leaq	0x2b0(%rbp), %rdx
1409438fd:     	callq	0x140b218a0 <.text+0xb208a0>
140943902:     	movq	%rdx, %r8
140943905:     	leaq	0xf0(%rbp), %rcx
14094390c:     	movq	%rax, %rdx
14094390f:     	callq	0x141453000 <.text+0x1452000>
140943914:     	movups	0x150(%rbp), %xmm0
14094391b:     	movaps	%xmm0, 0x380(%rbp)
140943922:     	movups	0x140(%rbp), %xmm0
140943929:     	movaps	%xmm0, 0x370(%rbp)
140943930:     	movups	0x130(%rbp), %xmm0
140943937:     	movaps	%xmm0, 0x360(%rbp)
14094393e:     	movups	0xf0(%rbp), %xmm0
140943945:     	movups	0x100(%rbp), %xmm1
14094394c:     	movups	0x110(%rbp), %xmm2
140943953:     	movups	0x120(%rbp), %xmm3
14094395a:     	movaps	%xmm3, 0x350(%rbp)
140943961:     	movaps	%xmm2, 0x340(%rbp)
140943968:     	movaps	%xmm1, 0x330(%rbp)
14094396f:     	movaps	%xmm0, 0x320(%rbp)
140943976:     	leaq	0x216efdb(%rip), %r8    # 0x142ab2958
14094397d:     	leaq	-0x60(%rbp), %rcx
140943981:     	movb	$0x28, %dl
140943983:     	callq	0x140ae3740 <.text+0xae2740>
140943988:     	leaq	0xf0(%rbp), %rcx
14094398f:     	callq	0x14098ee10 <.text+0x98de10>
140943994:     	movups	0x150(%rbp), %xmm0
14094399b:     	movaps	%xmm0, 0x310(%rbp)
1409439a2:     	movups	0x140(%rbp), %xmm0
1409439a9:     	movaps	%xmm0, 0x300(%rbp)
1409439b0:     	movups	0x130(%rbp), %xmm0
1409439b7:     	movaps	%xmm0, 0x2f0(%rbp)
1409439be:     	movups	0xf0(%rbp), %xmm0
1409439c5:     	movups	0x100(%rbp), %xmm1
1409439cc:     	movups	0x110(%rbp), %xmm2
1409439d3:     	movups	0x120(%rbp), %xmm3
1409439da:     	movaps	%xmm3, 0x2e0(%rbp)
1409439e1:     	movaps	%xmm2, 0x2d0(%rbp)
1409439e8:     	movaps	%xmm1, 0x2c0(%rbp)
1409439ef:     	movaps	%xmm0, 0x2b0(%rbp)
1409439f6:     	leaq	0x80(%rbp), %rcx
1409439fd:     	leaq	0x2b0(%rbp), %rdx
140943a04:     	callq	0x141737ae0 <.text+0x1736ae0>
140943a09:     	movb	$0x29, 0x2b0(%rbp)
140943a10:     	movq	0x2ce8959(%rip), %rcx   # 0x14362c370
140943a17:     	leaq	0x231c342(%rip), %r8    # 0x142c5fd60
140943a1e:     	leaq	0x2b0(%rbp), %rdi
140943a25:     	movq	%rdi, %rdx
140943a28:     	callq	0x140b218a0 <.text+0xb208a0>
140943a2d:     	movq	%rdx, %r8
140943a30:     	leaq	0x80(%rbp), %rbx
140943a37:     	movq	%rbx, %rcx
140943a3a:     	movq	%rax, %rdx
140943a3d:     	callq	0x141453000 <.text+0x1452000>
140943a42:     	movups	0xe0(%rbp), %xmm0
140943a49:     	movaps	%xmm0, 0x2a0(%rbp)
140943a50:     	movups	0xd0(%rbp), %xmm0
140943a57:     	movaps	%xmm0, 0x290(%rbp)
140943a5e:     	movups	0xc0(%rbp), %xmm0
140943a65:     	movaps	%xmm0, 0x280(%rbp)
140943a6c:     	movups	0x80(%rbp), %xmm0
140943a73:     	movups	0x90(%rbp), %xmm1
140943a7a:     	movups	0xa0(%rbp), %xmm2
140943a81:     	movups	0xb0(%rbp), %xmm3
140943a88:     	movaps	%xmm3, 0x270(%rbp)
140943a8f:     	movaps	%xmm2, 0x260(%rbp)
140943a96:     	movaps	%xmm1, 0x250(%rbp)
140943a9d:     	movaps	%xmm0, 0x240(%rbp)
140943aa4:     	movups	0x10(%rbp), %xmm0
140943aa8:     	movups	0x20(%rbp), %xmm1
140943aac:     	movups	0x30(%rbp), %xmm2
140943ab0:     	movups	0x40(%rbp), %xmm3
140943ab4:     	movaps	%xmm0, 0xf0(%rbp)
140943abb:     	movaps	%xmm1, 0x100(%rbp)
140943ac2:     	movaps	%xmm2, 0x110(%rbp)
140943ac9:     	movaps	%xmm3, 0x120(%rbp)
140943ad0:     	movups	0x50(%rbp), %xmm0
140943ad4:     	movaps	%xmm0, 0x130(%rbp)
140943adb:     	movups	0x60(%rbp), %xmm0
140943adf:     	movaps	%xmm0, 0x140(%rbp)
140943ae6:     	movups	0x70(%rbp), %xmm0
140943aea:     	movaps	%xmm0, 0x150(%rbp)
140943af1:     	movaps	0x380(%rbp), %xmm0
140943af8:     	movaps	%xmm0, 0x1c0(%rbp)
140943aff:     	movaps	0x370(%rbp), %xmm0
140943b06:     	movaps	%xmm0, 0x1b0(%rbp)
140943b0d:     	movaps	0x360(%rbp), %xmm0
140943b14:     	movaps	%xmm0, 0x1a0(%rbp)
140943b1b:     	movaps	0x320(%rbp), %xmm0
140943b22:     	movaps	0x330(%rbp), %xmm1
140943b29:     	movaps	0x340(%rbp), %xmm2
140943b30:     	movaps	0x350(%rbp), %xmm3
140943b37:     	movaps	%xmm3, 0x190(%rbp)
140943b3e:     	movaps	%xmm2, 0x180(%rbp)
140943b45:     	movaps	%xmm1, 0x170(%rbp)
140943b4c:     	movaps	%xmm0, 0x160(%rbp)
140943b53:     	movups	(%rbp), %xmm0
140943b57:     	movaps	%xmm0, 0x230(%rbp)
140943b5e:     	movups	-0x10(%rbp), %xmm0
140943b62:     	movaps	%xmm0, 0x220(%rbp)
140943b69:     	movups	-0x20(%rbp), %xmm0
140943b6d:     	movaps	%xmm0, 0x210(%rbp)
140943b74:     	movups	-0x60(%rbp), %xmm0
140943b78:     	movups	-0x50(%rbp), %xmm1
140943b7c:     	movups	-0x40(%rbp), %xmm2
140943b80:     	movups	-0x30(%rbp), %xmm3
140943b84:     	movaps	%xmm3, 0x200(%rbp)
140943b8b:     	movaps	%xmm2, 0x1f0(%rbp)
140943b92:     	movaps	%xmm1, 0x1e0(%rbp)
140943b99:     	movaps	%xmm0, 0x1d0(%rbp)
140943ba0:     	leaq	0xf0(%rbp), %rdx
140943ba7:     	movq	%rsi, %rcx
140943baa:     	callq	0x140b669c0 <.text+0xb659c0>
140943baf:     	leaq	0x38(%rsi), %r14
140943bb3:     	movabsq	$-0x71e42dae69d4587c, %rdx # imm = 0x8E1BD251962BA784
140943bbd:     	movabsq	$0x6a782374c0e6529b, %r8 # imm = 0x6A782374C0E6529B
140943bc7:     	movq	%r14, %rcx
140943bca:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140943bcf:     	testq	%rax, %rax
140943bd2:     	je	0x140943c99 <.text+0x942c99>
140943bd8:     	cmpq	%rdx, 0x218(%rsi)
140943bdf:     	jbe	0x140943c99 <.text+0x942c99>
140943be5:     	movq	0x210(%rsi), %rax
140943bec:     	movq	(%rax,%rdx,8), %rcx
140943bf0:     	testq	%rcx, %rcx
140943bf3:     	je	0x140943c99 <.text+0x942c99>
140943bf9:     	movq	0x1e0(%rsi), %rax
140943c00:     	notq	%rcx
140943c03:     	leaq	(%rcx,%rcx,2), %rcx
140943c07:     	shlq	$0x5, %rcx
140943c0b:     	cmpq	$0x0, 0x20(%rax,%rcx)
140943c11:     	je	0x140943c99 <.text+0x942c99>
140943c17:     	addq	%rcx, %rax
140943c1a:     	movq	0x10(%rax), %rcx
140943c1e:     	movl	0x5b0(%rsi), %edx
140943c24:     	movl	%edx, 0x5c(%rax)
140943c27:     	callq	0x1416a0b70 <.text+0x169fb70>
140943c2c:     	movabsq	$0x2557ffe8a7986afe, %rdx # imm = 0x2557FFE8A7986AFE
140943c36:     	movabsq	$0x4932233c9b0e0737, %r8 # imm = 0x4932233C9B0E0737
140943c40:     	movq	%r14, %rcx
140943c43:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140943c48:     	testq	%rax, %rax
140943c4b:     	je	0x140943c80 <.text+0x942c80>
140943c4d:     	cmpq	%rdx, 0x218(%rsi)
140943c54:     	jbe	0x140943c80 <.text+0x942c80>
140943c56:     	movq	0x210(%rsi), %rax
140943c5d:     	movq	(%rax,%rdx,8), %rax
140943c61:     	testq	%rax, %rax
140943c64:     	je	0x140943c80 <.text+0x942c80>
140943c66:     	movq	0x1e0(%rsi), %rcx
140943c6d:     	notq	%rax
140943c70:     	leaq	(%rax,%rax,2), %rax
140943c74:     	shlq	$0x5, %rax
140943c78:     	cmpq	$0x0, 0x20(%rcx,%rax)
140943c7e:     	jne	0x140943c88 <.text+0x942c88>
140943c80:     	movq	%rsi, %rcx
140943c83:     	callq	0x141513380 <.text+0x1512380>
140943c88:     	movq	%rsi, %rax
140943c8b:     	addq	$0x420, %rsp            # imm = 0x420
140943c92:     	popq	%rbx
140943c93:     	popq	%rdi
140943c94:     	popq	%rsi
140943c95:     	popq	%r14
140943c97:     	popq	%rbp
140943c98:     	retq
140943c99:     	leaq	0x21af020(%rip), %rax   # 0x142af2cc0
140943ca0:     	movq	%rax, 0x2b0(%rbp)
140943ca7:     	movq	$0x27, 0x2b8(%rbp)
140943cb2:     	movq	%rdi, 0x80(%rbp)
140943cb9:     	leaq	-0x839e30(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140943cc0:     	movq	%rax, 0x88(%rbp)
140943cc7:     	leaq	0x21af122(%rip), %rax   # 0x142af2df0
140943cce:     	movq	%rax, 0xf0(%rbp)
140943cd5:     	movq	$0x2, 0xf8(%rbp)
140943ce0:     	movq	$0x0, 0x110(%rbp)
140943ceb:     	movq	%rbx, 0x100(%rbp)
140943cf2:     	movq	$0x1, 0x108(%rbp)
140943cfd:     	leaq	0x21aefa4(%rip), %rdx   # 0x142af2ca8
140943d04:     	leaq	0xf0(%rbp), %rcx
140943d0b:     	callq	0x1428d9390 <.text+0x28d8390>
140943d10:     	int3
