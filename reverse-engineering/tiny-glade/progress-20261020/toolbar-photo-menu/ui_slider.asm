
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1423486d0:     	pushq	%r15
1423486d2:     	pushq	%r14
1423486d4:     	pushq	%r13
1423486d6:     	pushq	%r12
1423486d8:     	pushq	%rsi
1423486d9:     	pushq	%rdi
1423486da:     	pushq	%rbp
1423486db:     	pushq	%rbx
1423486dc:     	subq	$0x208, %rsp            # imm = 0x208
1423486e3:     	movaps	%xmm15, 0x1f0(%rsp)
1423486ec:     	movaps	%xmm14, 0x1e0(%rsp)
1423486f5:     	movaps	%xmm13, 0x1d0(%rsp)
1423486fe:     	movaps	%xmm12, 0x1c0(%rsp)
142348707:     	movaps	%xmm11, 0x1b0(%rsp)
142348710:     	movaps	%xmm10, 0x1a0(%rsp)
142348719:     	movaps	%xmm9, 0x190(%rsp)
142348722:     	movaps	%xmm8, 0x180(%rsp)
14234872b:     	movaps	%xmm7, 0x170(%rsp)
142348733:     	movaps	%xmm6, 0x160(%rsp)
14234873b:     	movq	%r9, 0xe0(%rsp)
142348743:     	movq	%rcx, 0x118(%rsp)
14234874b:     	movq	0x2c0(%rsp), %r10
142348753:     	movq	0x2b8(%rsp), %r11
14234875b:     	movq	0x288(%rsp), %rax
142348763:     	movq	0x280(%rsp), %rcx
14234876b:     	movq	0x278(%rsp), %r9
142348773:     	movq	(%r9), %r9
142348776:     	movss	0x28(%r9), %xmm13
14234877c:     	movq	(%r8), %r9
14234877f:     	movq	0x8(%r8), %rbp
142348783:     	movl	0x14(%r8), %r8d
142348787:     	movl	%r8d, 0x3c(%rsp)
14234878c:     	movq	0xf8(%r9), %rbx
142348793:     	movq	0x100(%r9), %r8
14234879a:     	leaq	(%rbx,%r8,4), %rdi
14234879e:     	movq	(%rdx), %rdx
1423487a1:     	movq	%rdx, 0xd0(%rsp)
1423487a9:     	movaps	%xmm13, %xmm10
1423487ad:     	divss	0x6848b2(%rip), %xmm10  # 0x1429cd068
1423487b6:     	movq	(%rcx), %rdx
1423487b9:     	movq	(%rax), %r12
1423487bc:     	movaps	0x5dd12c(%rip), %xmm14  # 0x1429258f0
1423487c4:     	xorps	%xmm10, %xmm14
1423487c8:     	xorps	%xmm15, %xmm15
1423487cc:     	movss	0x5dd12f(%rip), %xmm11  # 0x142925904
1423487d5:     	movzbl	0x108(%r9), %eax
1423487dd:     	movb	%al, 0x4c(%rsp)
1423487e1:     	movq	0x2a8(%rsp), %rcx
1423487e9:     	movq	0x2a0(%rsp), %rax
1423487f1:     	movq	0x290(%rsp), %r8
1423487f9:     	movq	(%rax), %rax
1423487fc:     	movq	%rax, 0x50(%rsp)
142348801:     	movq	(%rcx), %rax
142348804:     	movq	%rax, 0xb8(%rsp)
14234880c:     	movq	(%r11), %rax
14234880f:     	movq	%rax, 0xf8(%rsp)
142348817:     	movq	(%r10), %rax
14234881a:     	movq	%rax, 0xb0(%rsp)
142348822:     	movq	%rdx, 0xe8(%rsp)
14234882a:     	leaq	0x10(%rdx), %rax
14234882e:     	movq	%rax, 0x100(%rsp)
142348836:     	movzbl	(%r8), %eax
14234883a:     	movl	%eax, 0x5c(%rsp)
14234883e:     	movq	$0x0, 0x68(%rsp)
142348847:     	movl	$0x8, %eax
14234884c:     	movq	%rax, 0xd8(%rsp)
142348854:     	movq	$0x0, 0x40(%rsp)
14234885d:     	movl	$0x8, %eax
142348862:     	movq	%rax, 0x108(%rsp)
14234886a:     	xorl	%esi, %esi
14234886c:     	xorl	%r14d, %r14d
14234886f:     	movq	$0x0, 0xa8(%rsp)
14234887b:     	xorl	%r15d, %r15d
14234887e:     	movq	%r9, 0x30(%rsp)
142348883:     	movq	%r12, 0x110(%rsp)
14234888b:     	movq	%r15, 0xf0(%rsp)
142348893:     	cmpb	$0x0, 0x4c(%rsp)
142348898:     	je	0x1423488e0 <.text+0x23478e0>
14234889a:     	cmpq	%rsi, %r14
14234889d:     	movq	0x30(%rsp), %r11
1423488a2:     	jne	0x1423489fb <.text+0x23479fb>
1423488a8:     	cmpq	%rdi, %rbx
1423488ab:     	je	0x142348ef9 <.text+0x2347ef9>
1423488b1:     	movq	0x1a8(%rbp), %rax
1423488b8:     	nopl	(%rax,%rax)
1423488c0:     	movl	(%rbx), %ecx
1423488c2:     	leaq	(%rcx,%rcx,8), %rcx
1423488c6:     	movq	0x10(%rax,%rcx,8), %rsi
1423488cb:     	testq	%rsi, %rsi
1423488ce:     	jne	0x142348922 <.text+0x2347922>
1423488d0:     	addq	$0x4, %rbx
1423488d4:     	cmpq	%rdi, %rbx
1423488d7:     	jne	0x1423488c0 <.text+0x23478c0>
1423488d9:     	jmp	0x142348ef9 <.text+0x2347ef9>
1423488de:     	nop
1423488e0:     	cmpq	%rsi, %r14
1423488e3:     	movq	0x30(%rsp), %r11
1423488e8:     	jne	0x142348a59 <.text+0x2347a59>
1423488ee:     	cmpq	%rdi, %rbx
1423488f1:     	je	0x142348ef9 <.text+0x2347ef9>
1423488f7:     	movq	0x100(%rbp), %rax
1423488fe:     	nop
142348900:     	movl	(%rbx), %ecx
142348902:     	leaq	(%rcx,%rcx,4), %rcx
142348906:     	shlq	$0x5, %rcx
14234890a:     	movq	0x58(%rax,%rcx), %rsi
14234890f:     	testq	%rsi, %rsi
142348912:     	jne	0x14234895b <.text+0x234795b>
142348914:     	addq	$0x4, %rbx
142348918:     	cmpq	%rdi, %rbx
14234891b:     	jne	0x142348900 <.text+0x2347900>
14234891d:     	jmp	0x142348ef9 <.text+0x2347ef9>
142348922:     	leaq	(%rax,%rcx,8), %rax
142348926:     	movq	0x110(%r11), %rdx
14234892d:     	movq	0x38(%rax), %rcx
142348931:     	cmpq	%rdx, 0x40(%rax)
142348935:     	jbe	0x1423489ae <.text+0x23479ae>
142348937:     	movq	(%rcx,%rdx,8), %r8
14234893b:     	movq	0x18(%rax), %rdx
14234893f:     	testq	%r8, %r8
142348942:     	je	0x1423489b2 <.text+0x23479b2>
142348944:     	notq	%r8
142348947:     	leaq	(%r8,%r8,2), %r8
14234894b:     	shlq	$0x4, %r8
14234894f:     	movq	0x10(%rdx,%r8), %r8
142348954:     	movq	%r8, 0x40(%rsp)
142348959:     	jmp	0x1423489bb <.text+0x23479bb>
14234895b:     	addq	%rcx, %rax
14234895e:     	movl	0x94(%rax), %ecx
142348964:     	movq	0x1a8(%rbp), %r9
14234896b:     	leaq	(%rcx,%rcx,8), %r10
14234896f:     	leaq	(%r9,%r10,8), %rdx
142348973:     	movq	0x110(%r11), %r8
14234897a:     	movq	0x38(%r9,%r10,8), %rcx
14234897f:     	cmpq	%r8, 0x40(%r9,%r10,8)
142348984:     	jbe	0x142348a0c <.text+0x2347a0c>
14234898a:     	movq	(%rcx,%r8,8), %r8
14234898e:     	movq	0x18(%rdx), %rdx
142348992:     	testq	%r8, %r8
142348995:     	je	0x142348a10 <.text+0x2347a10>
142348997:     	notq	%r8
14234899a:     	leaq	(%r8,%r8,2), %r8
14234899e:     	shlq	$0x4, %r8
1423489a2:     	movq	0x10(%rdx,%r8), %r8
1423489a7:     	movq	%r8, 0x40(%rsp)
1423489ac:     	jmp	0x142348a19 <.text+0x2347a19>
1423489ae:     	movq	0x18(%rax), %rdx
1423489b2:     	movq	$0x0, 0x40(%rsp)
1423489bb:     	movq	0x118(%r11), %r8
1423489c2:     	addq	$0x4, %rbx
1423489c6:     	movq	(%rcx,%r8,8), %rcx
1423489ca:     	notq	%rcx
1423489cd:     	leaq	(%rcx,%rcx,2), %rcx
1423489d1:     	shlq	$0x4, %rcx
1423489d5:     	movq	0x10(%rdx,%rcx), %r8
1423489da:     	movq	%r8, 0x68(%rsp)
1423489df:     	movq	0x28(%rdx,%rcx), %rcx
1423489e4:     	movq	%rcx, 0xc0(%rsp)
1423489ec:     	movq	0x8(%rax), %rax
1423489f0:     	movq	%rax, 0xd8(%rsp)
1423489f8:     	xorl	%r14d, %r14d
1423489fb:     	movq	0xd8(%rsp), %rax
142348a03:     	leaq	(%rax,%r14,8), %rax
142348a07:     	movl	%r14d, %r15d
142348a0a:     	jmp	0x142348a71 <.text+0x2347a71>
142348a0c:     	movq	0x18(%rdx), %rdx
142348a10:     	movq	$0x0, 0x40(%rsp)
142348a19:     	movq	0x118(%r11), %r8
142348a20:     	addq	$0x4, %rbx
142348a24:     	movq	(%rcx,%r8,8), %rcx
142348a28:     	notq	%rcx
142348a2b:     	leaq	(%rcx,%rcx,2), %rcx
142348a2f:     	shlq	$0x4, %rcx
142348a33:     	movq	0x10(%rdx,%rcx), %r8
142348a38:     	movq	%r8, 0x68(%rsp)
142348a3d:     	movq	0x28(%rdx,%rcx), %rcx
142348a42:     	movq	%rcx, 0xc0(%rsp)
142348a4a:     	movq	0x50(%rax), %rax
142348a4e:     	movq	%rax, 0x108(%rsp)
142348a56:     	xorl	%r14d, %r14d
142348a59:     	movq	%r14, %rcx
142348a5c:     	shlq	$0x4, %rcx
142348a60:     	movq	0x108(%rsp), %rdx
142348a68:     	leaq	(%rdx,%rcx), %rax
142348a6c:     	movl	0x8(%rdx,%rcx), %r15d
142348a71:     	leaq	(%r15,%r15,2), %rcx
142348a75:     	movq	0xc0(%rsp), %rdx
142348a7d:     	leaq	(%rdx,%r15,4), %r13
142348a81:     	shlq	$0x4, %r15
142348a85:     	addq	0x40(%rsp), %r15
142348a8a:     	movq	0x68(%rsp), %rdx
142348a8f:     	leaq	(%rdx,%rcx,8), %r12
142348a93:     	movq	(%rax), %rax
142348a96:     	movq	%rax, 0x60(%rsp)
142348a9b:     	cmpl	$0x0, 0x4(%r12)
142348aa1:     	je	0x142348adc <.text+0x2347adc>
142348aa3:     	movq	(%r12), %rax
142348aa7:     	movq	0xe0(%rsp), %rcx
142348aaf:     	movups	(%rcx), %xmm0
142348ab2:     	movups	0x10(%rcx), %xmm1
142348ab6:     	movq	%rax, 0x90(%rsp)
142348abe:     	movups	%xmm0, 0x70(%rsp)
142348ac3:     	movups	%xmm1, 0x80(%rsp)
142348acb:     	leaq	0x70(%rsp), %rcx
142348ad0:     	leaq	0xd06231(%rip), %rdx    # 0x14304ed08
142348ad7:     	callq	0x140d14770 <.text+0xd13770>
142348adc:     	incq	%r14
142348adf:     	movl	0x3c(%rsp), %eax
142348ae3:     	movl	%eax, (%r13)
142348ae7:     	movl	$0x0, 0x4(%r12)
142348af0:     	movsd	0x8(%r15), %xmm0
142348af6:     	movaps	%xmm0, %xmm1
142348af9:     	mulps	%xmm0, %xmm1
142348afc:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
142348b00:     	addss	%xmm1, %xmm2
142348b04:     	xorps	%xmm1, %xmm1
142348b07:     	sqrtss	%xmm2, %xmm1
142348b0b:     	ucomiss	%xmm15, %xmm1
142348b0f:     	jbe	0x142348893 <.text+0x2347893>
142348b15:     	movshdup	%xmm0, %xmm12           # xmm12 = xmm0[1,1,3,3]
142348b1a:     	mulss	0x5dcdb5(%rip), %xmm12  # 0x1429258d8
142348b23:     	movss	(%r15), %xmm1
142348b28:     	movaps	%xmm12, %xmm7
142348b2c:     	mulss	0x69668c(%rip), %xmm7   # 0x1429df1c0
142348b34:     	addss	%xmm0, %xmm7
142348b38:     	mulss	0x5dcd30(%rip), %xmm7   # 0x142925870
142348b40:     	movaps	%xmm1, %xmm8
142348b44:     	subss	%xmm7, %xmm8
142348b49:     	addss	%xmm1, %xmm7
142348b4d:     	movq	0xf8(%rsp), %rax
142348b55:     	movss	(%rax), %xmm3
142348b59:     	movzbl	0x10(%r12), %ecx
142348b5f:     	movq	0xb0(%rsp), %rax
142348b67:     	movq	%rax, 0x20(%rsp)
142348b6c:     	movq	0x50(%rsp), %rdx
142348b71:     	movq	0xb8(%rsp), %r8
142348b79:     	callq	0x142349de0 <.text+0x2348de0>
142348b7e:     	movaps	%xmm0, %xmm6
142348b81:     	movaps	%xmm11, %xmm0
142348b85:     	subss	%xmm6, %xmm0
142348b89:     	mulss	%xmm8, %xmm0
142348b8e:     	mulss	%xmm6, %xmm7
142348b92:     	addss	%xmm0, %xmm7
142348b96:     	movss	0x4(%r15), %xmm8
142348b9c:     	movaps	%xmm12, %xmm3
142348ba0:     	mulss	0x5dce08(%rip), %xmm3   # 0x1429259b0
142348ba8:     	movss	%xmm3, 0x20(%rsp)
142348bae:     	leaq	0x70(%rsp), %r15
142348bb3:     	movq	%r15, %rcx
142348bb6:     	movaps	%xmm7, %xmm1
142348bb9:     	movaps	%xmm8, %xmm2
142348bbd:     	callq	0x14123fff0 <.text+0x123eff0>
142348bc2:     	movq	0xd0(%rsp), %rax
142348bca:     	movss	(%rax), %xmm1
142348bce:     	movss	0x4(%rax), %xmm2
142348bd3:     	movq	%r15, %rcx
142348bd6:     	callq	0x141240140 <.text+0x123f140>
142348bdb:     	movl	0x3c(%rsp), %ecx
142348bdf:     	movl	%ecx, (%r13)
142348be3:     	movaps	%xmm10, %xmm0
142348be7:     	testb	%al, %al
142348be9:     	jne	0x142348bef <.text+0x2347bef>
142348beb:     	movaps	%xmm14, %xmm0
142348bef:     	addss	0x8(%r12), %xmm0
142348bf6:     	xorps	%xmm2, %xmm2
142348bf9:     	maxss	%xmm0, %xmm2
142348bfd:     	movaps	%xmm11, %xmm1
142348c01:     	minss	%xmm2, %xmm1
142348c05:     	movss	%xmm1, 0x8(%r12)
142348c0c:     	leaq	0xd0610d(%rip), %rcx    # 0x14304ed20
142348c13:     	callq	0x140c95ef0 <.text+0xc94ef0>
142348c18:     	movaps	%xmm0, %xmm9
142348c1c:     	movzbl	0x12ebfe6(%rip), %eax   # 0x143634c09
142348c23:     	movl	$0x15, %ecx
142348c28:     	movl	$0x1, %edx
142348c2d:     	callq	0x140613c10 <.text+0x612c10>
142348c32:     	testq	%rax, %rax
142348c35:     	je	0x142349d48 <.text+0x2348d48>
142348c3b:     	movaps	%xmm11, %xmm0
142348c3f:     	subss	%xmm9, %xmm0
142348c44:     	mulss	0x5dcd63(%rip), %xmm9   # 0x1429259b0
142348c4d:     	addss	%xmm0, %xmm9
142348c52:     	mulss	%xmm9, %xmm12
142348c57:     	movups	0xd060ca(%rip), %xmm0   # 0x14304ed28
142348c5e:     	movups	%xmm0, (%rax)
142348c61:     	movabsq	$0x7465676469775f72, %rcx # imm = 0x7465676469775F72
142348c6b:     	movq	%rcx, 0xd(%rax)
142348c6f:     	movq	$0x15, 0x70(%rsp)
142348c78:     	movq	%rax, 0x78(%rsp)
142348c7d:     	movq	$0x15, 0x80(%rsp)
142348c89:     	movl	$0x0, 0x88(%rsp)
142348c94:     	movss	%xmm7, 0x90(%rsp)
142348c9d:     	movss	%xmm8, 0x94(%rsp)
142348ca7:     	movss	%xmm12, 0x98(%rsp)
142348cb1:     	movss	%xmm12, 0x9c(%rsp)
142348cbb:     	movq	$0x1, 0xa0(%rsp)
142348cc7:     	leaq	0x138(%rsp), %rcx
142348ccf:     	movq	0xe0(%rsp), %rdx
142348cd7:     	leaq	0x70(%rsp), %r8
142348cdc:     	leaq	0xd0605d(%rip), %r9     # 0x14304ed40
142348ce3:     	callq	0x140fe06b0 <.text+0xfdf6b0>
142348ce8:     	movq	0x158(%rsp), %rax
142348cf0:     	movl	0x3c(%rsp), %ecx
142348cf4:     	movl	%ecx, (%r13)
142348cf8:     	movq	%rax, (%r12)
142348cfc:     	movq	0x100(%rsp), %rax
142348d04:     	cmpl	$0x1, (%rax)
142348d07:     	jne	0x142348893 <.text+0x2347893>
142348d0d:     	movq	0xe8(%rsp), %rax
142348d15:     	movq	0x60(%rsp), %rcx
142348d1a:     	cmpq	%rcx, 0x18(%rax)
142348d1e:     	jne	0x142348893 <.text+0x2347893>
142348d24:     	movq	0x110(%rsp), %r12
142348d2c:     	movl	0xa0(%r12), %ecx
142348d34:     	movss	0xa4(%r12), %xmm1
142348d3e:     	callq	0x14093fa90 <.text+0x93ea90>
142348d43:     	testb	%al, %al
142348d45:     	jne	0x142348d62 <.text+0x2347d62>
142348d47:     	movl	0xa8(%r12), %ecx
142348d4f:     	movss	0xac(%r12), %xmm1
142348d59:     	callq	0x14093fa90 <.text+0x93ea90>
142348d5e:     	testb	%al, %al
142348d60:     	je	0x142348dad <.text+0x2347dad>
142348d62:     	movq	0x298(%rsp), %rax
142348d6a:     	movq	(%rax), %rax
142348d6d:     	movq	0x100(%rsp), %rcx
142348d75:     	cmpq	(%rcx), %rax
142348d78:     	jne	0x142348dad <.text+0x2347dad>
142348d7a:     	testq	%rax, %rax
142348d7d:     	je	0x142348d9c <.text+0x2347d9c>
142348d7f:     	cmpl	$0x1, %eax
142348d82:     	movq	0x298(%rsp), %rax
142348d8a:     	movq	0x8(%rax), %rax
142348d8e:     	movq	0xe8(%rsp), %rcx
142348d96:     	cmpq	0x18(%rcx), %rax
142348d9a:     	jne	0x142348dad <.text+0x2347dad>
142348d9c:     	movq	0x290(%rsp), %rax
142348da4:     	movb	$0x1, (%rax)
142348da7:     	movb	$0x1, %al
142348da9:     	movl	%eax, 0x5c(%rsp)
142348dad:     	movl	0xa0(%r12), %ecx
142348db5:     	movss	0xa4(%r12), %xmm1
142348dbf:     	callq	0x14093fa80 <.text+0x93ea80>
142348dc4:     	testb	%al, %al
142348dc6:     	je	0x142348dfb <.text+0x2347dfb>
142348dc8:     	movl	0xa8(%r12), %ecx
142348dd0:     	movss	0xac(%r12), %xmm1
142348dda:     	callq	0x14093fa80 <.text+0x93ea80>
142348ddf:     	testb	%al, %al
142348de1:     	je	0x142348dfb <.text+0x2347dfb>
142348de3:     	movq	0x290(%rsp), %rax
142348deb:     	movb	$0x0, (%rax)
142348dee:     	movl	$0x0, 0x5c(%rsp)
142348df6:     	jmp	0x142348893 <.text+0x2347893>
142348dfb:     	testb	$0x1, 0x5c(%rsp)
142348e00:     	je	0x142348893 <.text+0x2347893>
142348e06:     	movl	0xa0(%r12), %ecx
142348e0e:     	movss	0xa4(%r12), %xmm1
142348e18:     	callq	0x14093fa70 <.text+0x93ea70>
142348e1d:     	testb	%al, %al
142348e1f:     	je	0x142348e58 <.text+0x2347e58>
142348e21:     	movaps	%xmm6, %xmm0
142348e24:     	subss	%xmm13, %xmm0
142348e29:     	xorps	%xmm1, %xmm1
142348e2c:     	maxss	%xmm0, %xmm1
142348e30:     	movaps	%xmm11, %xmm0
142348e34:     	minss	%xmm1, %xmm0
142348e38:     	movaps	%xmm0, 0x120(%rsp)
142348e40:     	movq	0x60(%rsp), %rcx
142348e45:     	movq	%rcx, %rax
142348e48:     	shrq	$0x20, %rax
142348e4c:     	movq	%rax, 0xa8(%rsp)
142348e54:     	movl	%ecx, 0x48(%rsp)
142348e58:     	movl	0xa8(%r12), %ecx
142348e60:     	movss	0xac(%r12), %xmm1
142348e6a:     	callq	0x14093fa70 <.text+0x93ea70>
142348e6f:     	testb	%al, %al
142348e71:     	je	0x142348ea7 <.text+0x2347ea7>
142348e73:     	addss	%xmm13, %xmm6
142348e78:     	xorps	%xmm0, %xmm0
142348e7b:     	maxss	%xmm6, %xmm0
142348e7f:     	movaps	%xmm11, %xmm1
142348e83:     	minss	%xmm0, %xmm1
142348e87:     	movaps	%xmm1, 0x120(%rsp)
142348e8f:     	movq	0x60(%rsp), %rcx
142348e94:     	movq	%rcx, %rax
142348e97:     	shrq	$0x20, %rax
142348e9b:     	movq	%rax, 0xa8(%rsp)
142348ea3:     	movl	%ecx, 0x48(%rsp)
142348ea7:     	movl	0xa0(%r12), %ecx
142348eaf:     	movss	0xa4(%r12), %xmm1
142348eb9:     	callq	0x14093fa90 <.text+0x93ea90>
142348ebe:     	testb	%al, %al
142348ec0:     	jne	0x142348ee1 <.text+0x2347ee1>
142348ec2:     	movl	0xa8(%r12), %ecx
142348eca:     	movss	0xac(%r12), %xmm1
142348ed4:     	callq	0x14093fa90 <.text+0x93ea90>
142348ed9:     	testb	%al, %al
142348edb:     	je	0x142348893 <.text+0x2347893>
142348ee1:     	movq	0x60(%rsp), %rax
142348ee6:     	movq	%rax, %r15
142348ee9:     	shrq	$0x20, %r15
142348eed:     	movl	%eax, 0xcc(%rsp)
142348ef4:     	jmp	0x14234888b <.text+0x234788b>
142348ef9:     	movq	0x118(%rsp), %rax
142348f01:     	movq	(%rax), %rsi
142348f04:     	movq	0x8(%rax), %rax
142348f08:     	movq	(%rsi), %rdx
142348f0b:     	xorl	%ecx, %ecx
142348f0d:     	movq	%rdx, %r8
142348f10:     	subq	0x18(%rax), %r8
142348f14:     	cmovbq	%rcx, %r8
142348f18:     	subq	0x38(%rax), %rdx
142348f1c:     	cmovbq	%rcx, %rdx
142348f20:     	leaq	(%r8,%r8,2), %rdi
142348f24:     	shlq	$0x3, %rdi
142348f28:     	addq	0x8(%rax), %rdi
142348f2c:     	movq	0x10(%rax), %r9
142348f30:     	movq	0x30(%rax), %r10
142348f34:     	subq	%r8, %r9
142348f37:     	cmovbq	%rcx, %r9
142348f3b:     	movl	$0x8, %r8d
142348f41:     	cmovbq	%r8, %rdi
142348f45:     	leaq	(%rdx,%rdx,2), %r12
142348f49:     	shlq	$0x3, %r12
142348f4d:     	addq	0x28(%rax), %r12
142348f51:     	subq	%rdx, %r10
142348f54:     	cmovbq	%rcx, %r10
142348f58:     	cmovbq	%r8, %r12
142348f5c:     	movq	0x40(%rax), %rax
142348f60:     	subq	%r10, %rax
142348f63:     	subq	%r9, %rax
142348f66:     	movq	%rax, (%rsi)
142348f69:     	leaq	(%r9,%r9,2), %rax
142348f6d:     	leaq	(%rdi,%rax,8), %rbx
142348f71:     	leaq	(%r10,%r10,2), %rax
142348f75:     	leaq	(%r12,%rax,8), %r15
142348f79:     	movq	0x270(%rsp), %rax
142348f81:     	movl	(%rax), %ecx
142348f83:     	movl	%ecx, 0x4c(%rsp)
142348f87:     	movss	0x4(%rax), %xmm10
142348f8d:     	movl	$0xffffffff, %r14d      # imm = 0xFFFFFFFF
142348f93:     	movaps	0x120(%rsp), %xmm12
142348f9c:     	movl	0x48(%rsp), %r10d
142348fa1:     	movq	%rdi, %rdx
142348fa4:     	movq	%r12, %rcx
142348fa7:     	nopw	(%rax,%rax)
142348fb0:     	testq	%rdx, %rdx
142348fb3:     	je	0x142348fd0 <.text+0x2347fd0>
142348fb5:     	cmpq	%rbx, %rdx
142348fb8:     	je	0x142348fd0 <.text+0x2347fd0>
142348fba:     	leaq	0x18(%rdx), %rdi
142348fbe:     	movq	%rcx, %r12
142348fc1:     	movq	%rdx, %r13
142348fc4:     	jmp	0x142348fee <.text+0x2347fee>
142348fc6:     	nopw	%cs:(%rax,%rax)
142348fd0:     	testq	%rcx, %rcx
142348fd3:     	je	0x1423492e4 <.text+0x23482e4>
142348fd9:     	cmpq	%r15, %rcx
142348fdc:     	je	0x1423492e4 <.text+0x23482e4>
142348fe2:     	leaq	0x18(%rcx), %r12
142348fe6:     	xorl	%edi, %edi
142348fe8:     	movq	%rcx, %r13
142348feb:     	movq	%r12, %rcx
142348fee:     	incq	(%rsi)
142348ff1:     	movq	0x8(%r13), %rax
142348ff5:     	movl	%eax, %r8d
142348ff8:     	movq	%rdi, %rdx
142348ffb:     	cmpq	0x10(%rbp), %r8
142348fff:     	jae	0x142348fb0 <.text+0x2347fb0>
142349001:     	movq	0x8(%rbp), %r9
142349005:     	shrq	$0x20, %rax
142349009:     	leaq	(%r8,%r8,4), %r8
14234900d:     	movq	%rdi, %rdx
142349010:     	movq	%r12, %rcx
142349013:     	cmpl	%eax, (%r9,%r8,4)
142349017:     	jne	0x142348fb0 <.text+0x2347fb0>
142349019:     	leaq	(%r9,%r8,4), %rax
14234901d:     	movl	0x4(%rax), %r8d
142349021:     	movq	%rdi, %rdx
142349024:     	movq	%r12, %rcx
142349027:     	cmpq	%r14, %r8
14234902a:     	je	0x142348fb0 <.text+0x2347fb0>
14234902c:     	movq	%rdi, %rdx
14234902f:     	movq	%r12, %rcx
142349032:     	cmpq	%r8, 0x38(%r11)
142349036:     	jbe	0x142348fb0 <.text+0x2347fb0>
14234903c:     	movq	0x28(%r11), %rcx
142349040:     	movl	%r8d, %edx
142349043:     	shrl	$0x6, %edx
142349046:     	movq	(%rcx,%rdx,8), %rcx
14234904a:     	btq	%r8, %rcx
14234904e:     	movq	%rdi, %rdx
142349051:     	movq	%r12, %rcx
142349054:     	jae	0x142348fb0 <.text+0x2347fb0>
14234905a:     	movl	%r10d, 0x48(%rsp)
14234905f:     	movl	0xc(%rax), %ecx
142349062:     	movq	0x1a8(%rbp), %r9
142349069:     	leaq	(%rcx,%rcx,8), %r10
14234906d:     	leaq	(%r9,%r10,8), %rdx
142349071:     	movq	0x110(%r11), %r8
142349078:     	movq	0x38(%r9,%r10,8), %rcx
14234907d:     	cmpq	%r8, 0x40(%r9,%r10,8)
142349082:     	jbe	0x1423490a3 <.text+0x23480a3>
142349084:     	movq	(%rcx,%r8,8), %r8
142349088:     	movq	0x18(%rdx), %rdx
14234908c:     	testq	%r8, %r8
14234908f:     	je	0x1423490a7 <.text+0x23480a7>
142349091:     	notq	%r8
142349094:     	leaq	(%r8,%r8,2), %r8
142349098:     	shlq	$0x4, %r8
14234909c:     	movq	0x10(%rdx,%r8), %r8
1423490a1:     	jmp	0x1423490aa <.text+0x23480aa>
1423490a3:     	movq	0x18(%rdx), %rdx
1423490a7:     	xorl	%r8d, %r8d
1423490aa:     	movq	%r8, 0xd8(%rsp)
1423490b2:     	movl	0x10(%rax), %r9d
1423490b6:     	movq	0x118(%r11), %rax
1423490bd:     	movq	(%rcx,%rax,8), %rax
1423490c1:     	notq	%rax
1423490c4:     	leaq	(%rax,%rax,2), %rax
1423490c8:     	shlq	$0x4, %rax
1423490cc:     	movq	0x10(%rdx,%rax), %r10
1423490d1:     	movq	%r10, 0x60(%rsp)
1423490d6:     	movq	0x28(%rdx,%rax), %rax
1423490db:     	movq	%rax, 0x40(%rsp)
1423490e0:     	movq	%r9, 0x68(%rsp)
1423490e5:     	leaq	(%r9,%r9,2), %rcx
1423490e9:     	movq	%rcx, 0xc0(%rsp)
1423490f1:     	shlq	$0x4, %r9
1423490f5:     	movq	%r9, 0xe0(%rsp)
1423490fd:     	movss	(%r8,%r9), %xmm0
142349103:     	movss	0xc(%r8,%r9), %xmm8
14234910a:     	mulss	0x5dc7c5(%rip), %xmm8   # 0x1429258d8
142349113:     	movaps	%xmm8, %xmm6
142349117:     	mulss	0x6960a1(%rip), %xmm6   # 0x1429df1c0
14234911f:     	addss	0x8(%r8,%r9), %xmm6
142349126:     	mulss	0x5dc742(%rip), %xmm6   # 0x142925870
14234912e:     	movaps	%xmm0, %xmm7
142349131:     	subss	%xmm6, %xmm7
142349135:     	addss	%xmm0, %xmm6
142349139:     	movq	0xf8(%rsp), %rax
142349141:     	movss	(%rax), %xmm3
142349145:     	movzbl	0x10(%r10,%rcx,8), %ecx
14234914b:     	movq	0xb0(%rsp), %rax
142349153:     	movq	%rax, 0x20(%rsp)
142349158:     	movq	0x50(%rsp), %rdx
14234915d:     	movq	0xb8(%rsp), %r8
142349165:     	callq	0x142349de0 <.text+0x2348de0>
14234916a:     	movq	0xe0(%rsp), %rax
142349172:     	movq	0xd8(%rsp), %rcx
14234917a:     	movss	0x4(%rcx,%rax), %xmm2
142349180:     	movl	0x3c(%rsp), %eax
142349184:     	movq	0x40(%rsp), %rcx
142349189:     	movq	0x68(%rsp), %rdx
14234918e:     	movl	%eax, (%rcx,%rdx,4)
142349191:     	movq	0xc0(%rsp), %rax
142349199:     	movq	0x60(%rsp), %rcx
14234919e:     	movl	$0x3f800000, 0x8(%rcx,%rax,8) # imm = 0x3F800000
1423491a6:     	movzbl	0x10(%r13), %eax
1423491ab:     	cmpb	$0x3, %al
1423491ad:     	jne	0x14234926e <.text+0x234826e>
1423491b3:     	movaps	%xmm0, %xmm9
1423491b7:     	movaps	%xmm11, %xmm0
1423491bb:     	subss	%xmm9, %xmm0
1423491c0:     	mulss	%xmm7, %xmm0
1423491c4:     	mulss	%xmm6, %xmm9
1423491c9:     	addss	%xmm0, %xmm9
1423491ce:     	movq	0x270(%rsp), %rax
1423491d6:     	movl	$0x0, (%rax)
1423491dc:     	mulss	0x5dc7cb(%rip), %xmm8   # 0x1429259b0
1423491e5:     	movss	%xmm8, 0x20(%rsp)
1423491ec:     	leaq	0x70(%rsp), %rcx
1423491f1:     	movaps	%xmm9, %xmm1
1423491f5:     	movaps	%xmm8, %xmm3
1423491f9:     	callq	0x14123fff0 <.text+0x123eff0>
1423491fe:     	movq	0xd0(%rsp), %rax
142349206:     	movss	(%rax), %xmm1
14234920a:     	movss	0x4(%rax), %xmm2
14234920f:     	leaq	0x70(%rsp), %rcx
142349214:     	callq	0x141240140 <.text+0x123f140>
142349219:     	xorps	%xmm10, %xmm10
14234921d:     	testb	%al, %al
14234921f:     	je	0x142349232 <.text+0x2348232>
142349221:     	movq	0xd0(%rsp), %rax
142349229:     	subss	(%rax), %xmm9
14234922e:     	movaps	%xmm9, %xmm10
142349232:     	movq	0x270(%rsp), %rax
14234923a:     	movl	$0x1, (%rax)
142349240:     	movss	%xmm10, 0x4(%rax)
142349246:     	movzbl	0x10(%r13), %eax
14234924b:     	movl	$0x1, 0x4c(%rsp)
142349253:     	cmpb	$0x2, %al
142349255:     	jb	0x142349272 <.text+0x2348272>
142349257:     	testb	$0x1, 0x4c(%rsp)
14234925c:     	movq	0x30(%rsp), %r11
142349261:     	movl	0x48(%rsp), %r10d
142349266:     	je	0x142348fa1 <.text+0x2347fa1>
14234926c:     	jmp	0x14234929e <.text+0x234829e>
14234926e:     	cmpb	$0x2, %al
142349270:     	jae	0x142349257 <.text+0x2348257>
142349272:     	movl	0x8(%r13), %eax
142349276:     	movl	%eax, 0xcc(%rsp)
14234927d:     	movl	0xc(%r13), %eax
142349281:     	movq	%rax, 0xf0(%rsp)
142349289:     	testb	$0x1, 0x4c(%rsp)
14234928e:     	movq	0x30(%rsp), %r11
142349293:     	movl	0x48(%rsp), %r10d
142349298:     	je	0x142348fa1 <.text+0x2347fa1>
14234929e:     	movq	0xd0(%rsp), %rax
1423492a6:     	movss	(%rax), %xmm2
1423492aa:     	addss	%xmm10, %xmm2
1423492af:     	movaps	%xmm7, %xmm0
1423492b2:     	movaps	%xmm6, %xmm1
1423492b5:     	callq	0x140c904a0 <.text+0xc8f4a0>
1423492ba:     	movq	0x30(%rsp), %r11
1423492bf:     	xorps	%xmm1, %xmm1
1423492c2:     	maxss	%xmm0, %xmm1
1423492c6:     	movaps	%xmm11, %xmm12
1423492ca:     	minss	%xmm1, %xmm12
1423492cf:     	movl	0x8(%r13), %r10d
1423492d3:     	movl	0xc(%r13), %eax
1423492d7:     	movq	%rax, 0xa8(%rsp)
1423492df:     	jmp	0x142348fa1 <.text+0x2347fa1>
1423492e4:     	movq	0xa8(%rsp), %r9
1423492ec:     	testl	%r9d, %r9d
1423492ef:     	movq	0xf0(%rsp), %r13
1423492f7:     	je	0x1423499d7 <.text+0x23489d7>
1423492fd:     	movl	%r10d, %eax
142349300:     	movl	$0x1, %ecx
142349305:     	cmpq	%rax, 0x10(%rbp)
142349309:     	jbe	0x142349d44 <.text+0x2348d44>
14234930f:     	movq	0x8(%rbp), %rdx
142349313:     	leaq	(%rax,%rax,4), %r8
142349317:     	cmpl	%r9d, (%rdx,%r8,4)
14234931b:     	jne	0x142349d44 <.text+0x2348d44>
142349321:     	leaq	(%rdx,%r8,4), %r8
142349325:     	movl	0x4(%r8), %edx
142349329:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
14234932f:     	cmpq	%r9, %rdx
142349332:     	je	0x142349d44 <.text+0x2348d44>
142349338:     	cmpq	%rdx, 0x38(%r11)
14234933c:     	jbe	0x142349d5e <.text+0x2348d5e>
142349342:     	movq	0x28(%r11), %rcx
142349346:     	movl	%edx, %r9d
142349349:     	shrl	$0x6, %r9d
14234934d:     	movq	(%rcx,%r9,8), %rcx
142349351:     	btq	%rdx, %rcx
142349355:     	jae	0x142349d5e <.text+0x2348d5e>
14234935b:     	movd	%xmm12, %eax
142349360:     	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
142349365:     	cmpl	$0x7f7fffff, %eax       # imm = 0x7F7FFFFF
14234936a:     	jg	0x1423499d7 <.text+0x23489d7>
142349370:     	movl	0xc(%r8), %eax
142349374:     	movl	0x10(%r8), %esi
142349378:     	movq	0x1a8(%rbp), %rcx
14234937f:     	leaq	(%rax,%rax,8), %rax
142349383:     	movq	0x118(%r11), %rdx
14234938a:     	movq	0x18(%rcx,%rax,8), %r8
14234938f:     	movq	0x38(%rcx,%rax,8), %rax
142349394:     	movq	(%rax,%rdx,8), %rax
142349398:     	notq	%rax
14234939b:     	leaq	(%rax,%rax,2), %rax
14234939f:     	shlq	$0x4, %rax
1423493a3:     	leaq	(%r8,%rax), %rcx
1423493a7:     	movq	0x28(%r8,%rax), %rdi
1423493ac:     	movq	0x10(%rcx), %rcx
1423493b0:     	leaq	(%rsi,%rsi,2), %r8
1423493b4:     	leaq	(%rcx,%r8,8), %rbx
1423493b8:     	movzbl	0x10(%rcx,%r8,8), %eax
1423493be:     	leal	(,%rax,4), %edx
1423493c5:     	movss	0xc(%rcx,%r8,8), %xmm0
1423493cc:     	unpcklps	%xmm12, %xmm0           # xmm0 = xmm0[0],xmm12[0],xmm0[1],xmm12[1]
1423493d0:     	mulps	0xd05739(%rip), %xmm0   # 0x14304eb10
1423493d7:     	cvttss2si	%xmm0, %r8
1423493dc:     	leaq	0xd059a9(%rip), %r9     # 0x14304ed8c
1423493e3:     	movq	%r8, %r10
1423493e6:     	sarq	$0x3f, %r10
1423493ea:     	movss	0x610cea(%rip), %xmm1   # 0x14295a0dc
1423493f2:     	movaps	%xmm0, %xmm2
1423493f5:     	subss	%xmm1, %xmm2
1423493f9:     	cvttss2si	%xmm2, %rcx
1423493fe:     	andq	%r10, %rcx
142349401:     	orq	%r8, %rcx
142349404:     	xorl	%r8d, %r8d
142349407:     	xorps	%xmm2, %xmm2
14234940a:     	ucomiss	%xmm2, %xmm0
14234940d:     	cmovbq	%r8, %rcx
142349411:     	movss	(%rdx,%r9), %xmm6
142349417:     	movss	0x610cc1(%rip), %xmm3   # 0x14295a0e0
14234941f:     	ucomiss	%xmm3, %xmm0
142349422:     	movq	$-0x1, %r9
142349429:     	cmovaq	%r9, %rcx
14234942d:     	leaq	0xd0598c(%rip), %r10    # 0x14304edc0
142349434:     	movss	(%rdx,%r10), %xmm7
14234943a:     	movq	%rcx, %xmm4
14234943f:     	movshdup	%xmm0, %xmm0            # xmm0 = xmm0[1,1,3,3]
142349443:     	cvttss2si	%xmm0, %rdx
142349448:     	movq	%rdx, %r10
14234944b:     	sarq	$0x3f, %r10
14234944f:     	movaps	%xmm0, %xmm5
142349452:     	subss	%xmm1, %xmm5
142349456:     	cvttss2si	%xmm5, %r11
14234945b:     	andq	%r10, %r11
14234945e:     	orq	%rdx, %r11
142349461:     	ucomiss	%xmm2, %xmm0
142349464:     	cmovbq	%r8, %r11
142349468:     	ucomiss	%xmm3, %xmm0
14234946b:     	cmovaq	%r9, %r11
14234946f:     	movq	%r11, %xmm0
142349474:     	punpcklqdq	%xmm0, %xmm4    # xmm4 = xmm4[0],xmm0[0]
142349478:     	pshufd	$0xee, %xmm4, %xmm0     # xmm0 = xmm4[2,3,2,3]
14234947d:     	movq	%xmm0, %rdx
142349482:     	cmpq	%rdx, %rcx
142349485:     	jne	0x1423494b5 <.text+0x23484b5>
142349487:     	movaps	%xmm11, %xmm0
14234948b:     	subss	%xmm12, %xmm0
142349490:     	mulss	%xmm7, %xmm0
142349494:     	mulss	%xmm12, %xmm6
142349499:     	addss	%xmm0, %xmm6
14234949d:     	movzbl	%al, %eax
1423494a0:     	leaq	0xd05981(%rip), %rcx    # 0x14304ee28
1423494a7:     	movslq	(%rcx,%rax,4), %rax
1423494ab:     	addq	%rcx, %rax
1423494ae:     	movq	0x30(%rsp), %r11
1423494b3:     	jmpq	*%rax
1423494b5:     	movq	0x2b0(%rsp), %rdx
1423494bd:     	movl	0x1c(%rdx), %eax
1423494c0:     	movq	0x10(%rdx), %rcx
1423494c4:     	movl	%eax, (%rcx)
1423494c6:     	movq	(%rdx), %r14
1423494c9:     	movq	0x30(%r14), %r15
1423494cd:     	movq	0x40(%r14), %r12
1423494d1:     	cmpq	0x20(%r14), %r15
1423494d5:     	jne	0x1423494e7 <.text+0x23484e7>
1423494d7:     	leaq	0x20(%r14), %rcx
1423494db:     	leaq	0x86bd8e(%rip), %rdx    # 0x142bb5270
1423494e2:     	callq	0x140562c30 <.text+0x561c30>
1423494e7:     	movq	0x28(%r14), %rax
1423494eb:     	movq	%r15, %rcx
1423494ee:     	shlq	$0x5, %rcx
1423494f2:     	movl	$0x1c, (%rax,%rcx)
1423494f9:     	movq	%r12, 0x18(%rax,%rcx)
1423494fe:     	incq	%r15
142349501:     	movq	%r15, 0x30(%r14)
142349505:     	incq	0x40(%r14)
142349509:     	movaps	%xmm11, %xmm0
14234950d:     	subss	%xmm12, %xmm0
142349512:     	mulss	%xmm7, %xmm0
142349516:     	mulss	%xmm12, %xmm6
14234951b:     	addss	%xmm0, %xmm6
14234951f:     	movzbl	0x10(%rbx), %eax
142349523:     	leaq	0xd058fe(%rip), %rcx    # 0x14304ee28
14234952a:     	movslq	(%rcx,%rax,4), %rax
14234952e:     	addq	%rcx, %rax
142349531:     	movq	0x30(%rsp), %r11
142349536:     	jmpq	*%rax
142349538:     	movq	0x2c8(%rsp), %rax
142349540:     	movq	(%rax), %rax
142349543:     	cmpl	$0x13, (%rax)
142349546:     	jne	0x1423499ca <.text+0x23489ca>
14234954c:     	movq	0x2a0(%rsp), %rcx
142349554:     	movl	0x1c(%rcx), %eax
142349557:     	movq	0x10(%rcx), %rcx
14234955b:     	movl	%eax, (%rcx)
14234955d:     	leaq	0x70(%rsp), %rcx
142349562:     	movq	0x50(%rsp), %rdx
142349567:     	callq	0x1408aea60 <.text+0x8ada60>
14234956c:     	movq	0x70(%rsp), %rax
142349571:     	movq	0x78(%rsp), %rcx
142349576:     	cmpq	%rcx, %rax
142349579:     	je	0x1423499c5 <.text+0x23489c5>
14234957f:     	movq	0x80(%rsp), %rdx
142349587:     	incq	%rdx
14234958a:     	jmp	0x1423495a0 <.text+0x23485a0>
14234958c:     	nopl	(%rax)
142349590:     	addq	$0x40, %rax
142349594:     	incq	%rdx
142349597:     	cmpq	%rcx, %rax
14234959a:     	je	0x1423499c5 <.text+0x23489c5>
1423495a0:     	testq	%rax, %rax
1423495a3:     	je	0x142349590 <.text+0x2348590>
1423495a5:     	cmpq	$0x1, %rdx
1423495a9:     	je	0x142349590 <.text+0x2348590>
1423495ab:     	movss	%xmm6, 0x1c(%rax)
1423495b0:     	addq	$0x40, %rax
1423495b4:     	cmpq	%rcx, %rax
1423495b7:     	jne	0x142349587 <.text+0x2348587>
1423495b9:     	jmp	0x1423499c5 <.text+0x23489c5>
1423495be:     	movq	0x2c8(%rsp), %rax
1423495c6:     	movq	(%rax), %rax
1423495c9:     	cmpl	$0x13, (%rax)
1423495cc:     	je	0x1423499ca <.text+0x23489ca>
1423495d2:     	movq	0x2c0(%rsp), %rcx
1423495da:     	movl	0x1c(%rcx), %eax
1423495dd:     	movq	0x10(%rcx), %rcx
1423495e1:     	movl	%eax, (%rcx)
1423495e3:     	movq	0xb0(%rsp), %rcx
1423495eb:     	movaps	%xmm6, %xmm1
1423495ee:     	callq	0x1409f18a0 <.text+0x9f08a0>
1423495f3:     	movq	0x30(%rsp), %r11
1423495f8:     	jmp	0x1423499ca <.text+0x23489ca>
1423495fd:     	movq	0x2c8(%rsp), %rax
142349605:     	movq	(%rax), %rax
142349608:     	cmpl	$0x13, (%rax)
14234960b:     	jne	0x1423499ca <.text+0x23489ca>
142349611:     	movq	0x2a0(%rsp), %rcx
142349619:     	movl	0x1c(%rcx), %eax
14234961c:     	movq	0x10(%rcx), %rcx
142349620:     	movl	%eax, (%rcx)
142349622:     	leaq	0x70(%rsp), %rcx
142349627:     	movq	0x50(%rsp), %rdx
14234962c:     	callq	0x1408aea60 <.text+0x8ada60>
142349631:     	movq	0x70(%rsp), %rcx
142349636:     	movq	0x78(%rsp), %rax
14234963b:     	cmpq	%rax, %rcx
14234963e:     	je	0x1423499c5 <.text+0x23489c5>
142349644:     	movq	0x80(%rsp), %rdx
14234964c:     	addq	$0x40, %rcx
142349650:     	incq	%rdx
142349653:     	jmp	0x142349673 <.text+0x2348673>
142349655:     	nopw	%cs:(%rax,%rax)
142349660:     	leaq	0x40(%rcx), %r8
142349664:     	incq	%rdx
142349667:     	cmpq	%rax, %rcx
14234966a:     	movq	%r8, %rcx
14234966d:     	je	0x1423499c5 <.text+0x23489c5>
142349673:     	cmpq	$0x40, %rcx
142349677:     	je	0x142349660 <.text+0x2348660>
142349679:     	cmpq	$0x1, %rdx
14234967d:     	je	0x142349660 <.text+0x2348660>
14234967f:     	movss	%xmm6, -0x8(%rcx)
142349684:     	cmpq	%rax, %rcx
142349687:     	jne	0x14234964c <.text+0x234864c>
142349689:     	jmp	0x1423499c5 <.text+0x23489c5>
14234968e:     	movq	0x2c8(%rsp), %rax
142349696:     	movq	(%rax), %rax
142349699:     	cmpl	$0x13, (%rax)
14234969c:     	je	0x1423499ca <.text+0x23489ca>
1423496a2:     	movq	0x2c0(%rsp), %rcx
1423496aa:     	movl	0x1c(%rcx), %eax
1423496ad:     	movq	0x10(%rcx), %rcx
1423496b1:     	movl	%eax, (%rcx)
1423496b3:     	movq	0xb0(%rsp), %rcx
1423496bb:     	movaps	%xmm6, %xmm1
1423496be:     	callq	0x1409f18d0 <.text+0x9f08d0>
1423496c3:     	movq	0x30(%rsp), %r11
1423496c8:     	jmp	0x1423499ca <.text+0x23489ca>
1423496cd:     	movq	0x2c8(%rsp), %rax
1423496d5:     	movq	(%rax), %rax
1423496d8:     	cmpl	$0x13, (%rax)
1423496db:     	je	0x1423499ca <.text+0x23489ca>
1423496e1:     	movss	0x5dc1ef(%rip), %xmm1   # 0x1429258d8
1423496e9:     	maxss	%xmm6, %xmm1
1423496ed:     	movss	0x5e4ffb(%rip), %xmm0   # 0x14292e6f0
1423496f5:     	minss	%xmm1, %xmm0
1423496f9:     	movq	0x2b8(%rsp), %rcx
142349701:     	movl	0x1c(%rcx), %eax
142349704:     	movq	0x10(%rcx), %rcx
142349708:     	movl	%eax, (%rcx)
14234970a:     	movq	0xf8(%rsp), %rax
142349712:     	movss	%xmm0, (%rax)
142349716:     	jmp	0x1423499ca <.text+0x23489ca>
14234971b:     	movq	0x2c8(%rsp), %rax
142349723:     	movq	(%rax), %rax
142349726:     	cmpl	$0x13, (%rax)
142349729:     	jne	0x1423499ca <.text+0x23489ca>
14234972f:     	movq	0x2a0(%rsp), %rcx
142349737:     	movl	0x1c(%rcx), %eax
14234973a:     	movq	0x10(%rcx), %rcx
14234973e:     	movl	%eax, (%rcx)
142349740:     	leaq	0x70(%rsp), %rcx
142349745:     	movq	0x50(%rsp), %rdx
14234974a:     	callq	0x1408aea60 <.text+0x8ada60>
14234974f:     	movq	0x70(%rsp), %rcx
142349754:     	movq	0x78(%rsp), %rax
142349759:     	cmpq	%rax, %rcx
14234975c:     	je	0x1423499c5 <.text+0x23489c5>
142349762:     	movq	0x80(%rsp), %rdx
14234976a:     	addq	$0x40, %rcx
14234976e:     	incq	%rdx
142349771:     	jmp	0x142349793 <.text+0x2348793>
142349773:     	nopw	%cs:(%rax,%rax)
142349780:     	leaq	0x40(%rcx), %r8
142349784:     	incq	%rdx
142349787:     	cmpq	%rax, %rcx
14234978a:     	movq	%r8, %rcx
14234978d:     	je	0x1423499c5 <.text+0x23489c5>
142349793:     	cmpq	$0x40, %rcx
142349797:     	je	0x142349780 <.text+0x2348780>
142349799:     	cmpq	$0x1, %rdx
14234979d:     	je	0x142349780 <.text+0x2348780>
14234979f:     	movss	%xmm6, -0x1c(%rcx)
1423497a4:     	cmpq	%rax, %rcx
1423497a7:     	jne	0x14234976a <.text+0x234876a>
1423497a9:     	jmp	0x1423499c5 <.text+0x23489c5>
1423497ae:     	movq	0x2c8(%rsp), %rax
1423497b6:     	movq	(%rax), %rax
1423497b9:     	cmpl	$0x13, (%rax)
1423497bc:     	jne	0x1423499ca <.text+0x23489ca>
1423497c2:     	movq	0x2a0(%rsp), %rcx
1423497ca:     	movl	0x1c(%rcx), %eax
1423497cd:     	movq	0x10(%rcx), %rcx
1423497d1:     	movl	%eax, (%rcx)
1423497d3:     	leaq	0x70(%rsp), %rcx
1423497d8:     	movq	0x50(%rsp), %rdx
1423497dd:     	callq	0x1408aea60 <.text+0x8ada60>
1423497e2:     	movq	0x70(%rsp), %rcx
1423497e7:     	movq	0x78(%rsp), %rax
1423497ec:     	cmpq	%rax, %rcx
1423497ef:     	je	0x1423499c5 <.text+0x23489c5>
1423497f5:     	movq	0x80(%rsp), %rdx
1423497fd:     	addq	$0x40, %rcx
142349801:     	incq	%rdx
142349804:     	jmp	0x142349823 <.text+0x2348823>
142349806:     	nopw	%cs:(%rax,%rax)
142349810:     	leaq	0x40(%rcx), %r8
142349814:     	incq	%rdx
142349817:     	cmpq	%rax, %rcx
14234981a:     	movq	%r8, %rcx
14234981d:     	je	0x1423499c5 <.text+0x23489c5>
142349823:     	cmpq	$0x40, %rcx
142349827:     	je	0x142349810 <.text+0x2348810>
142349829:     	cmpq	$0x1, %rdx
14234982d:     	je	0x142349810 <.text+0x2348810>
14234982f:     	movss	%xmm6, -0x10(%rcx)
142349834:     	cmpq	%rax, %rcx
142349837:     	jne	0x1423497fd <.text+0x23487fd>
142349839:     	jmp	0x1423499c5 <.text+0x23489c5>
14234983e:     	movq	0x2c8(%rsp), %rax
142349846:     	movq	(%rax), %rax
142349849:     	cmpl	$0x13, (%rax)
14234984c:     	jne	0x1423499ca <.text+0x23489ca>
142349852:     	leaq	0xd0546b(%rip), %rcx    # 0x14304ecc4
142349859:     	movaps	%xmm6, %xmm1
14234985c:     	callq	0x142383730 <.text+0x2382730>
142349861:     	movaps	%xmm0, %xmm6
142349864:     	movq	0x2a8(%rsp), %rcx
14234986c:     	movl	0x1c(%rcx), %eax
14234986f:     	movq	0x10(%rcx), %rcx
142349873:     	movl	%eax, (%rcx)
142349875:     	movq	0xb8(%rsp), %rcx
14234987d:     	callq	0x140911240 <.text+0x910240>
142349882:     	movq	0x30(%rsp), %r11
142349887:     	movss	0x18(%rax), %xmm0
14234988c:     	ucomiss	%xmm6, %xmm0
14234988f:     	jne	0x142349897 <.text+0x2348897>
142349891:     	jnp	0x1423499ca <.text+0x23489ca>
142349897:     	movss	%xmm6, 0x18(%rax)
14234989c:     	cmpl	$0x1, (%rax)
14234989f:     	ja	0x1423499ca <.text+0x23489ca>
1423498a5:     	movl	$0x3, (%rax)
1423498ab:     	movsd	0x10(%rax), %xmm0
1423498b0:     	movsd	%xmm0, 0x4(%rax)
1423498b5:     	jmp	0x1423499ca <.text+0x23489ca>
1423498ba:     	movq	0x2c8(%rsp), %rax
1423498c2:     	movq	(%rax), %rax
1423498c5:     	cmpl	$0x13, (%rax)
1423498c8:     	jne	0x1423499ca <.text+0x23489ca>
1423498ce:     	movq	0x2a0(%rsp), %rcx
1423498d6:     	movl	0x1c(%rcx), %eax
1423498d9:     	movq	0x10(%rcx), %rcx
1423498dd:     	movl	%eax, (%rcx)
1423498df:     	leaq	0x70(%rsp), %rcx
1423498e4:     	movq	0x50(%rsp), %rdx
1423498e9:     	callq	0x1408aea60 <.text+0x8ada60>
1423498ee:     	movq	0x70(%rsp), %rcx
1423498f3:     	movq	0x78(%rsp), %rax
1423498f8:     	cmpq	%rax, %rcx
1423498fb:     	je	0x1423499c5 <.text+0x23489c5>
142349901:     	movq	0x80(%rsp), %rdx
142349909:     	addq	$0x40, %rcx
14234990d:     	incq	%rdx
142349910:     	jmp	0x142349933 <.text+0x2348933>
142349912:     	nopw	%cs:(%rax,%rax)
142349920:     	leaq	0x40(%rcx), %r8
142349924:     	incq	%rdx
142349927:     	cmpq	%rax, %rcx
14234992a:     	movq	%r8, %rcx
14234992d:     	je	0x1423499c5 <.text+0x23489c5>
142349933:     	cmpq	$0x40, %rcx
142349937:     	je	0x142349920 <.text+0x2348920>
142349939:     	cmpq	$0x1, %rdx
14234993d:     	je	0x142349920 <.text+0x2348920>
14234993f:     	movss	%xmm6, -0x20(%rcx)
142349944:     	cmpq	%rax, %rcx
142349947:     	jne	0x142349909 <.text+0x2348909>
142349949:     	jmp	0x1423499c5 <.text+0x23489c5>
14234994b:     	movq	0x2c8(%rsp), %rax
142349953:     	movq	(%rax), %rax
142349956:     	cmpl	$0x13, (%rax)
142349959:     	jne	0x1423499ca <.text+0x23489ca>
14234995b:     	movq	0x2a0(%rsp), %rcx
142349963:     	movl	0x1c(%rcx), %eax
142349966:     	movq	0x10(%rcx), %rcx
14234996a:     	movl	%eax, (%rcx)
14234996c:     	leaq	0x70(%rsp), %rcx
142349971:     	movq	0x50(%rsp), %rdx
142349976:     	callq	0x1408aea60 <.text+0x8ada60>
14234997b:     	movq	0x70(%rsp), %rcx
142349980:     	movq	0x78(%rsp), %rax
142349985:     	cmpq	%rax, %rcx
142349988:     	je	0x1423499c5 <.text+0x23489c5>
14234998a:     	movq	0x80(%rsp), %rdx
142349992:     	addq	$0x40, %rcx
142349996:     	incq	%rdx
142349999:     	jmp	0x1423499af <.text+0x23489af>
14234999b:     	nopl	(%rax,%rax)
1423499a0:     	leaq	0x40(%rcx), %r8
1423499a4:     	incq	%rdx
1423499a7:     	cmpq	%rax, %rcx
1423499aa:     	movq	%r8, %rcx
1423499ad:     	je	0x1423499c5 <.text+0x23489c5>
1423499af:     	cmpq	$0x40, %rcx
1423499b3:     	je	0x1423499a0 <.text+0x23489a0>
1423499b5:     	cmpq	$0x1, %rdx
1423499b9:     	je	0x1423499a0 <.text+0x23489a0>
1423499bb:     	movss	%xmm6, -0xc(%rcx)
1423499c0:     	cmpq	%rax, %rcx
1423499c3:     	jne	0x142349992 <.text+0x2348992>
1423499c5:     	movq	0x30(%rsp), %r11
1423499ca:     	movl	0x3c(%rsp), %eax
1423499ce:     	movl	%eax, (%rdi,%rsi,4)
1423499d1:     	movss	%xmm12, 0xc(%rbx)
1423499d7:     	testl	%r13d, %r13d
1423499da:     	movl	0xcc(%rsp), %eax
1423499e1:     	je	0x142349b40 <.text+0x2348b40>
1423499e7:     	movl	%eax, %eax
1423499e9:     	movl	$0x1, %ecx
1423499ee:     	cmpq	%rax, 0x10(%rbp)
1423499f2:     	jbe	0x142349d46 <.text+0x2348d46>
1423499f8:     	movq	0x8(%rbp), %rdx
1423499fc:     	leaq	(%rax,%rax,4), %r8
142349a00:     	cmpl	%r13d, (%rdx,%r8,4)
142349a04:     	jne	0x142349d46 <.text+0x2348d46>
142349a0a:     	leaq	(%rdx,%r8,4), %r8
142349a0e:     	movl	0x4(%r8), %edx
142349a12:     	movl	$0xffffffff, %r9d       # imm = 0xFFFFFFFF
142349a18:     	cmpq	%r9, %rdx
142349a1b:     	je	0x142349d46 <.text+0x2348d46>
142349a21:     	cmpq	%rdx, 0x38(%r11)
142349a25:     	jbe	0x142349d89 <.text+0x2348d89>
142349a2b:     	movq	0x28(%r11), %rcx
142349a2f:     	movl	%edx, %r9d
142349a32:     	shrl	$0x6, %r9d
142349a36:     	movq	(%rcx,%r9,8), %rcx
142349a3a:     	btq	%rdx, %rcx
142349a3e:     	jae	0x142349d89 <.text+0x2348d89>
142349a44:     	movl	0xc(%r8), %eax
142349a48:     	movl	0x10(%r8), %ecx
142349a4c:     	movq	0x1a8(%rbp), %rdx
142349a53:     	leaq	(%rax,%rax,8), %rax
142349a57:     	movq	0x118(%r11), %r8
142349a5e:     	movq	0x18(%rdx,%rax,8), %r9
142349a63:     	movq	0x38(%rdx,%rax,8), %rax
142349a68:     	movq	(%rax,%r8,8), %rax
142349a6c:     	notq	%rax
142349a6f:     	leaq	(%rax,%rax,2), %rax
142349a73:     	shlq	$0x4, %rax
142349a77:     	movq	0x10(%r9,%rax), %rax
142349a7c:     	leaq	(%rcx,%rcx,2), %rcx
142349a80:     	movzbl	0x10(%rax,%rcx,8), %eax
142349a85:     	addl	$-0x9, %eax
142349a88:     	cmpl	$0x3, %eax
142349a8b:     	ja	0x142349b40 <.text+0x2348b40>
142349a91:     	leaq	0xd053c4(%rip), %rcx    # 0x14304ee5c
142349a98:     	movslq	(%rcx,%rax,4), %rax
142349a9c:     	addq	%rcx, %rax
142349a9f:     	jmpq	*%rax
142349aa1:     	movq	0x2c8(%rsp), %rax
142349aa9:     	movq	(%rax), %rax
142349aac:     	cmpl	$0x9, (%rax)
142349aaf:     	je	0x142349b40 <.text+0x2348b40>
142349ab5:     	movq	0x2d0(%rsp), %rdx
142349abd:     	movl	0x1c(%rdx), %eax
142349ac0:     	movq	0x10(%rdx), %rcx
142349ac4:     	movl	%eax, (%rcx)
142349ac6:     	movq	(%rdx), %rsi
142349ac9:     	movq	0x30(%rsi), %rdi
142349acd:     	movq	0x40(%rsi), %rbx
142349ad1:     	movb	$0x4, %bpl
142349ad4:     	cmpq	0x20(%rsi), %rdi
142349ad8:     	je	0x142349b11 <.text+0x2348b11>
142349ada:     	jmp	0x142349b21 <.text+0x2348b21>
142349adc:     	movq	0x2c8(%rsp), %rax
142349ae4:     	movq	(%rax), %rax
142349ae7:     	cmpl	$0xf, (%rax)
142349aea:     	je	0x142349b40 <.text+0x2348b40>
142349aec:     	movq	0x2d0(%rsp), %rdx
142349af4:     	movl	0x1c(%rdx), %eax
142349af7:     	movq	0x10(%rdx), %rcx
142349afb:     	movl	%eax, (%rcx)
142349afd:     	movq	(%rdx), %rsi
142349b00:     	movq	0x30(%rsi), %rdi
142349b04:     	movq	0x40(%rsi), %rbx
142349b08:     	movb	$0x8, %bpl
142349b0b:     	cmpq	0x20(%rsi), %rdi
142349b0f:     	jne	0x142349b21 <.text+0x2348b21>
142349b11:     	leaq	0x20(%rsi), %rcx
142349b15:     	leaq	0x86b754(%rip), %rdx    # 0x142bb5270
142349b1c:     	callq	0x140562630 <.text+0x561630>
142349b21:     	movq	0x28(%rsi), %rax
142349b25:     	movq	%rdi, %rcx
142349b28:     	shlq	$0x4, %rcx
142349b2c:     	movq	%rbx, (%rax,%rcx)
142349b30:     	movb	%bpl, 0x8(%rax,%rcx)
142349b35:     	incq	%rdi
142349b38:     	movq	%rdi, 0x30(%rsi)
142349b3c:     	incq	0x40(%rsi)
142349b40:     	movq	0x100(%rsp), %rax
142349b48:     	movups	(%rax), %xmm0
142349b4b:     	movq	0x298(%rsp), %rax
142349b53:     	movups	%xmm0, (%rax)
142349b56:     	movaps	0x160(%rsp), %xmm6
142349b5e:     	movaps	0x170(%rsp), %xmm7
142349b66:     	movaps	0x180(%rsp), %xmm8
142349b6f:     	movaps	0x190(%rsp), %xmm9
142349b78:     	movaps	0x1a0(%rsp), %xmm10
142349b81:     	movaps	0x1b0(%rsp), %xmm11
142349b8a:     	movaps	0x1c0(%rsp), %xmm12
142349b93:     	movaps	0x1d0(%rsp), %xmm13
142349b9c:     	movaps	0x1e0(%rsp), %xmm14
142349ba5:     	movaps	0x1f0(%rsp), %xmm15
142349bae:     	addq	$0x208, %rsp            # imm = 0x208
142349bb5:     	popq	%rbx
142349bb6:     	popq	%rbp
142349bb7:     	popq	%rdi
142349bb8:     	popq	%rsi
142349bb9:     	popq	%r12
142349bbb:     	popq	%r13
142349bbd:     	popq	%r14
142349bbf:     	popq	%r15
142349bc1:     	retq
142349bc2:     	movq	0x2c8(%rsp), %rax
142349bca:     	movq	(%rax), %rax
142349bcd:     	cmpl	$0x11, (%rax)
142349bd0:     	je	0x142349b40 <.text+0x2348b40>
142349bd6:     	movq	0x2d0(%rsp), %rdx
142349bde:     	movl	0x1c(%rdx), %eax
142349be1:     	movq	0x10(%rdx), %rcx
142349be5:     	movl	%eax, (%rcx)
142349be7:     	movq	(%rdx), %rsi
142349bea:     	movq	0x30(%rsi), %rdi
142349bee:     	movq	0x40(%rsi), %rbx
142349bf2:     	movb	$0x9, %bpl
142349bf5:     	cmpq	0x20(%rsi), %rdi
142349bf9:     	je	0x142349b11 <.text+0x2348b11>
142349bff:     	jmp	0x142349b21 <.text+0x2348b21>
142349c04:     	movq	0x2c8(%rsp), %rax
142349c0c:     	movq	(%rax), %rax
142349c0f:     	movl	(%rax), %eax
142349c11:     	addl	$-0x7, %eax
142349c14:     	cmpl	$0x2, %eax
142349c17:     	jb	0x142349b40 <.text+0x2348b40>
142349c1d:     	movq	0x2d0(%rsp), %rdx
142349c25:     	movl	0x1c(%rdx), %eax
142349c28:     	movq	0x10(%rdx), %rcx
142349c2c:     	movl	%eax, (%rcx)
142349c2e:     	movq	(%rdx), %rsi
142349c31:     	movq	0x30(%rsi), %rdi
142349c35:     	movq	0x40(%rsi), %rbx
142349c39:     	movb	$0x1, %bpl
142349c3c:     	cmpq	0x20(%rsi), %rdi
142349c40:     	jne	0x142349b21 <.text+0x2348b21>
142349c46:     	jmp	0x142349b11 <.text+0x2348b11>
142349c4b:     	movq	0x2c8(%rsp), %rax
142349c53:     	movq	(%rax), %rax
142349c56:     	cmpl	$0x13, (%rax)
142349c59:     	jne	0x1423499ca <.text+0x23489ca>
142349c5f:     	movq	0x2a8(%rsp), %rcx
142349c67:     	movl	0x1c(%rcx), %eax
142349c6a:     	movq	0x10(%rcx), %rcx
142349c6e:     	movl	%eax, (%rcx)
142349c70:     	movq	0xb8(%rsp), %rcx
142349c78:     	callq	0x140911260 <.text+0x910260>
142349c7d:     	movq	0x30(%rsp), %r11
142349c82:     	subss	%xmm6, %xmm11
142349c87:     	movsd	0xc(%rax), %xmm0
142349c8c:     	unpcklps	%xmm6, %xmm11           # xmm11 = xmm11[0],xmm6[0],xmm11[1],xmm6[1]
142349c90:     	mulps	%xmm0, %xmm11
142349c94:     	movshdup	%xmm11, %xmm0           # xmm0 = xmm11[1,1,3,3]
142349c99:     	addss	%xmm11, %xmm0
142349c9e:     	movss	%xmm0, 0x8(%rax)
142349ca3:     	jmp	0x1423499ca <.text+0x23489ca>
142349ca8:     	movq	0x2c8(%rsp), %rax
142349cb0:     	movq	(%rax), %rax
142349cb3:     	cmpl	$0x13, (%rax)
142349cb6:     	je	0x1423499ca <.text+0x23489ca>
142349cbc:     	movq	0x2c0(%rsp), %rcx
142349cc4:     	movl	0x1c(%rcx), %eax
142349cc7:     	movq	0x10(%rcx), %rcx
142349ccb:     	movl	%eax, (%rcx)
142349ccd:     	movq	0xb0(%rsp), %rcx
142349cd5:     	movaps	%xmm6, %xmm1
142349cd8:     	callq	0x1409f1870 <.text+0x9f0870>
142349cdd:     	movq	0x30(%rsp), %r11
142349ce2:     	jmp	0x1423499ca <.text+0x23489ca>
142349ce7:     	movq	0x2c8(%rsp), %rax
142349cef:     	movq	(%rax), %rax
142349cf2:     	cmpl	$0x13, (%rax)
142349cf5:     	jne	0x1423499ca <.text+0x23489ca>
142349cfb:     	movq	0x2a8(%rsp), %rcx
142349d03:     	movl	0x1c(%rcx), %eax
142349d06:     	movq	0x10(%rcx), %rcx
142349d0a:     	movl	%eax, (%rcx)
142349d0c:     	movq	0xb8(%rsp), %rcx
142349d14:     	callq	0x140911260 <.text+0x910260>
142349d19:     	movq	0x30(%rsp), %r11
142349d1e:     	subss	%xmm6, %xmm11
142349d23:     	movsd	0x1c(%rax), %xmm0
142349d28:     	unpcklps	%xmm6, %xmm11           # xmm11 = xmm11[0],xmm6[0],xmm11[1],xmm6[1]
142349d2c:     	mulps	%xmm0, %xmm11
142349d30:     	movshdup	%xmm11, %xmm0           # xmm0 = xmm11[1,1,3,3]
142349d35:     	addss	%xmm11, %xmm0
142349d3a:     	movss	%xmm0, 0x18(%rax)
142349d3f:     	jmp	0x1423499ca <.text+0x23489ca>
142349d44:     	jmp	0x142349d60 <.text+0x2348d60>
142349d46:     	jmp	0x142349d8b <.text+0x2348d8b>
142349d48:     	leaq	0xd04e69(%rip), %r8     # 0x14304ebb8
142349d4f:     	movl	$0x1, %ecx
142349d54:     	movl	$0x15, %edx
142349d59:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
142349d5e:     	xorl	%ecx, %ecx
142349d60:     	movq	0xa8(%rsp), %r8
142349d68:     	shlq	$0x20, %r8
142349d6c:     	orq	%rax, %r8
142349d6f:     	shlq	$0x20, %rdx
142349d73:     	orq	%rcx, %rdx
142349d76:     	movq	%rdx, 0x70(%rsp)
142349d7b:     	movq	%r8, 0x78(%rsp)
142349d80:     	leaq	0xd04f51(%rip), %rax    # 0x14304ecd8
142349d87:     	jmp	0x142349db2 <.text+0x2348db2>
142349d89:     	xorl	%ecx, %ecx
142349d8b:     	movq	0xf0(%rsp), %r8
142349d93:     	shlq	$0x20, %r8
142349d97:     	orq	%rax, %r8
142349d9a:     	shlq	$0x20, %rdx
142349d9e:     	orq	%rcx, %rdx
142349da1:     	movq	%rdx, 0x70(%rsp)
142349da6:     	movq	%r8, 0x78(%rsp)
142349dab:     	leaq	0xd04f3e(%rip), %rax    # 0x14304ecf0
142349db2:     	movq	%rax, 0x20(%rsp)
142349db7:     	leaq	0xd04d82(%rip), %rcx    # 0x14304eb40
142349dbe:     	leaq	0xd04d5b(%rip), %r9     # 0x14304eb20
142349dc5:     	leaq	0x70(%rsp), %r8
142349dca:     	movl	$0x2b, %edx
142349dcf:     	callq	0x1428d9760 <.text+0x28d8760>
142349dd4:     	int3
142349dd5:     	int3
142349dd6:     	int3
142349dd7:     	int3
142349dd8:     	int3
142349dd9:     	int3
142349dda:     	int3
142349ddb:     	int3
142349ddc:     	int3
142349ddd:     	int3
142349dde:     	int3
142349ddf:     	int3
