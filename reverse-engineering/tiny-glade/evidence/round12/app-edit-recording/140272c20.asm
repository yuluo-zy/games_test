
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140272c20:     	pushq	%rbp
140272c21:     	pushq	%rsi
140272c22:     	pushq	%rdi
140272c23:     	pushq	%rbx
140272c24:     	subq	$0x408, %rsp            # imm = 0x408
140272c2b:     	leaq	0x80(%rsp), %rbp
140272c33:     	movq	$-0x2, 0x380(%rbp)
140272c3e:     	movq	%rcx, %rsi
140272c41:     	movzbl	0x33c1fc1(%rip), %eax   # 0x143634c09
140272c48:     	movl	$0x150, %ecx            # imm = 0x150
140272c4d:     	movl	$0x8, %edx
140272c52:     	callq	0x140613c10 <.text+0x612c10>
140272c57:     	testq	%rax, %rax
140272c5a:     	je	0x140273106 <.text+0x272106>
140272c60:     	movq	%rax, 0x378(%rbp)
140272c67:     	xorps	%xmm0, %xmm0
140272c6a:     	movups	%xmm0, 0xa8(%rbp)
140272c71:     	movups	%xmm0, 0xc0(%rbp)
140272c78:     	movups	%xmm0, 0xd8(%rbp)
140272c7f:     	movups	%xmm0, 0xf0(%rbp)
140272c86:     	movups	%xmm0, 0x108(%rbp)
140272c8d:     	movl	$0x0, 0x118(%rbp)
140272c97:     	movups	%xmm0, 0x140(%rbp)
140272c9e:     	movups	%xmm0, 0x158(%rbp)
140272ca5:     	movups	%xmm0, 0x170(%rbp)
140272cac:     	movups	%xmm0, 0x188(%rbp)
140272cb3:     	movups	%xmm0, 0x1a0(%rbp)
140272cba:     	movl	$0x0, 0x1b0(%rbp)
140272cc4:     	movq	$0x0, 0x88(%rbp)
140272ccf:     	movq	$0x8, 0x90(%rbp)
140272cda:     	movq	$0x0, 0x98(%rbp)
140272ce5:     	movq	$0x10, 0xa0(%rbp)
140272cf0:     	movq	$0x10, 0xb8(%rbp)
140272cfb:     	movq	$0x10, 0xd0(%rbp)
140272d06:     	movq	$0x10, 0xe8(%rbp)
140272d11:     	movq	$0x10, 0x100(%rbp)
140272d1c:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
140272d26:     	movq	%rbx, 0x120(%rbp)
140272d2d:     	leaq	0x27359b2(%rip), %rax   # 0x1429a86e6
140272d34:     	movq	%rax, 0x128(%rbp)
140272d3b:     	movq	$0x38, 0x130(%rbp)
140272d46:     	movq	$0x10, 0x138(%rbp)
140272d51:     	movq	$0x10, 0x150(%rbp)
140272d5c:     	movq	$0x10, 0x168(%rbp)
140272d67:     	movq	$0x10, 0x180(%rbp)
140272d72:     	movq	$0x10, 0x198(%rbp)
140272d7d:     	movl	$0x0, 0x1b8(%rbp)
140272d87:     	movw	$0x1, 0x1bc(%rbp)
140272d90:     	movq	%rbx, 0x1c0(%rbp)
140272d97:     	movl	$0x0, 0x348(%rbp)
140272da1:     	movzbl	0x33c1e61(%rip), %eax   # 0x143634c09
140272da8:     	movl	$0x2c8, %ecx            # imm = 0x2C8
140272dad:     	movl	$0x8, %edx
140272db2:     	callq	0x140613c10 <.text+0x612c10>
140272db7:     	testq	%rax, %rax
140272dba:     	je	0x140273115 <.text+0x272115>
140272dc0:     	movq	%rax, %rdi
140272dc3:     	leaq	0x88(%rbp), %rdx
140272dca:     	movl	$0x2c8, %r8d            # imm = 0x2C8
140272dd0:     	movq	%rax, %rcx
140272dd3:     	callq	0x142923c00 <.text+0x2922c00>
140272dd8:     	leaq	0x2735501(%rip), %r8    # 0x1429a82e0
140272ddf:     	leaq	0x18(%rbp), %rcx
140272de3:     	movq	%rdi, %rdx
140272de6:     	callq	0x140ce4b10 <.text+0xce3b10>
140272deb:     	xorps	%xmm0, %xmm0
140272dee:     	movups	%xmm0, 0xa8(%rbp)
140272df5:     	movups	%xmm0, 0xc0(%rbp)
140272dfc:     	movups	%xmm0, 0xd8(%rbp)
140272e03:     	movups	%xmm0, 0xf0(%rbp)
140272e0a:     	movups	%xmm0, 0x108(%rbp)
140272e11:     	movl	$0x0, 0x118(%rbp)
140272e1b:     	movups	%xmm0, 0x140(%rbp)
140272e22:     	movups	%xmm0, 0x158(%rbp)
140272e29:     	movups	%xmm0, 0x170(%rbp)
140272e30:     	movups	%xmm0, 0x188(%rbp)
140272e37:     	movups	%xmm0, 0x1a0(%rbp)
140272e3e:     	movl	$0x0, 0x1b0(%rbp)
140272e48:     	movq	$0x0, 0x88(%rbp)
140272e53:     	movq	$0x8, 0x90(%rbp)
140272e5e:     	movq	$0x0, 0x98(%rbp)
140272e69:     	movq	$0x10, 0xa0(%rbp)
140272e74:     	movq	$0x10, 0xb8(%rbp)
140272e7f:     	movq	$0x10, 0xd0(%rbp)
140272e8a:     	movq	$0x10, 0xe8(%rbp)
140272e95:     	movq	$0x10, 0x100(%rbp)
140272ea0:     	movq	%rbx, 0x120(%rbp)
140272ea7:     	leaq	0x27357fe(%rip), %rax   # 0x1429a86ac
140272eae:     	movq	%rax, 0x128(%rbp)
140272eb5:     	movq	$0x3a, 0x130(%rbp)
140272ec0:     	movq	$0x10, 0x138(%rbp)
140272ecb:     	movq	$0x10, 0x150(%rbp)
140272ed6:     	movq	$0x10, 0x168(%rbp)
140272ee1:     	movq	$0x10, 0x180(%rbp)
140272eec:     	movq	$0x10, 0x198(%rbp)
140272ef7:     	movl	$0x0, 0x1b8(%rbp)
140272f01:     	movw	$0x1, 0x1bc(%rbp)
140272f0a:     	movq	%rbx, 0x1c0(%rbp)
140272f11:     	movl	$0x0, 0x370(%rbp)
140272f1b:     	movzbl	0x33c1ce7(%rip), %eax   # 0x143634c09
140272f22:     	movl	$0x2f0, %ecx            # imm = 0x2F0
140272f27:     	movl	$0x8, %edx
140272f2c:     	callq	0x140613c10 <.text+0x612c10>
140272f31:     	testq	%rax, %rax
140272f34:     	je	0x140273126 <.text+0x272126>
140272f3a:     	movq	%rax, %rdi
140272f3d:     	leaq	0x88(%rbp), %rdx
140272f44:     	movl	$0x2f0, %r8d            # imm = 0x2F0
140272f4a:     	movq	%rax, %rcx
140272f4d:     	callq	0x142923c00 <.text+0x2922c00>
140272f52:     	leaq	0x273543f(%rip), %r8    # 0x1429a8398
140272f59:     	leaq	-0x58(%rbp), %rcx
140272f5d:     	movq	%rdi, %rdx
140272f60:     	callq	0x140ce4b10 <.text+0xce3b10>
140272f65:     	leaq	0x88(%rbp), %rcx
140272f6c:     	callq	0x14002e640 <.text+0x2d640>
140272f71:     	movups	0x78(%rbp), %xmm0
140272f75:     	movq	0x378(%rbp), %rax
140272f7c:     	movups	%xmm0, 0x60(%rax)
140272f80:     	movups	0x68(%rbp), %xmm0
140272f84:     	movups	%xmm0, 0x50(%rax)
140272f88:     	movups	0x58(%rbp), %xmm0
140272f8c:     	movups	%xmm0, 0x40(%rax)
140272f90:     	movups	0x18(%rbp), %xmm0
140272f94:     	movups	0x28(%rbp), %xmm1
140272f98:     	movups	0x38(%rbp), %xmm2
140272f9c:     	movups	0x48(%rbp), %xmm3
140272fa0:     	movups	%xmm3, 0x30(%rax)
140272fa4:     	movups	%xmm2, 0x20(%rax)
140272fa8:     	movups	%xmm1, 0x10(%rax)
140272fac:     	movups	%xmm0, (%rax)
140272faf:     	movups	-0x58(%rbp), %xmm0
140272fb3:     	movups	-0x48(%rbp), %xmm1
140272fb7:     	movups	-0x38(%rbp), %xmm2
140272fbb:     	movups	-0x28(%rbp), %xmm3
140272fbf:     	movups	%xmm0, 0x70(%rax)
140272fc3:     	movups	%xmm1, 0x80(%rax)
140272fca:     	movups	%xmm2, 0x90(%rax)
140272fd1:     	movups	%xmm3, 0xa0(%rax)
140272fd8:     	movups	-0x18(%rbp), %xmm0
140272fdc:     	movups	%xmm0, 0xb0(%rax)
140272fe3:     	movups	-0x8(%rbp), %xmm0
140272fe7:     	movups	%xmm0, 0xc0(%rax)
140272fee:     	movups	0x8(%rbp), %xmm0
140272ff2:     	movups	%xmm0, 0xd0(%rax)
140272ff9:     	movups	0x88(%rbp), %xmm0
140273000:     	movups	0x98(%rbp), %xmm1
140273007:     	movups	0xa8(%rbp), %xmm2
14027300e:     	movups	0xb8(%rbp), %xmm3
140273015:     	movups	%xmm0, 0xe0(%rax)
14027301c:     	movups	%xmm1, 0xf0(%rax)
140273023:     	movups	%xmm2, 0x100(%rax)
14027302a:     	movups	%xmm3, 0x110(%rax)
140273031:     	movups	0xc8(%rbp), %xmm0
140273038:     	movups	%xmm0, 0x120(%rax)
14027303f:     	movups	0xd8(%rbp), %xmm0
140273046:     	movups	%xmm0, 0x130(%rax)
14027304d:     	movups	0xe8(%rbp), %xmm0
140273054:     	movups	%xmm0, 0x140(%rax)
14027305b:     	movq	$0x3, 0x90(%rbp)
140273066:     	movq	%rax, 0x98(%rbp)
14027306d:     	movq	$0x3, 0xa0(%rbp)
140273078:     	movq	$0x0, 0xa8(%rbp)
140273083:     	movq	$0x8, 0xb0(%rbp)
14027308e:     	xorps	%xmm0, %xmm0
140273091:     	movups	%xmm0, 0xb8(%rbp)
140273098:     	movq	%rbx, 0x88(%rbp)
14027309f:     	leaq	0xc0(%rbp), %rcx
1402730a6:     	callq	0x1401705d0 <.text+0x16f5d0>
1402730ab:     	movups	0xe8(%rbp), %xmm0
1402730b2:     	movups	%xmm0, 0x60(%rsi)
1402730b6:     	movups	0xd8(%rbp), %xmm0
1402730bd:     	movups	%xmm0, 0x50(%rsi)
1402730c1:     	movups	0xc8(%rbp), %xmm0
1402730c8:     	movups	%xmm0, 0x40(%rsi)
1402730cc:     	movups	0x88(%rbp), %xmm0
1402730d3:     	movups	0x98(%rbp), %xmm1
1402730da:     	movups	0xa8(%rbp), %xmm2
1402730e1:     	movups	0xb8(%rbp), %xmm3
1402730e8:     	movups	%xmm3, 0x30(%rsi)
1402730ec:     	movups	%xmm2, 0x20(%rsi)
1402730f0:     	movups	%xmm1, 0x10(%rsi)
1402730f4:     	movups	%xmm0, (%rsi)
1402730f7:     	movq	%rsi, %rax
1402730fa:     	addq	$0x408, %rsp            # imm = 0x408
140273101:     	popq	%rbx
140273102:     	popq	%rdi
140273103:     	popq	%rsi
140273104:     	popq	%rbp
140273105:     	retq
140273106:     	movl	$0x8, %ecx
14027310b:     	movl	$0x150, %edx            # imm = 0x150
140273110:     	callq	0x1428d9000 <.text+0x28d8000>
140273115:     	movl	$0x8, %ecx
14027311a:     	movl	$0x2c8, %edx            # imm = 0x2C8
14027311f:     	callq	0x1428d9000 <.text+0x28d8000>
140273124:     	jmp	0x140273135 <.text+0x272135>
140273126:     	movl	$0x8, %ecx
14027312b:     	movl	$0x2f0, %edx            # imm = 0x2F0
140273130:     	callq	0x1428d9000 <.text+0x28d8000>
140273135:     	ud2
