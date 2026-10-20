
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142349de0:     	subq	$0x28, %rsp
142349de4:     	movq	0x50(%rsp), %rax
142349de9:     	movzbl	%cl, %ecx
142349dec:     	leaq	0xd05079(%rip), %r9     # 0x14304ee6c
142349df3:     	movslq	(%r9,%rcx,4), %rcx
142349df7:     	addq	%r9, %rcx
142349dfa:     	jmpq	*%rcx
142349dfc:     	movq	%rdx, %rcx
142349dff:     	callq	0x1408aea50 <.text+0x8ada50>
142349e04:     	movss	0x1c(%rax), %xmm2
142349e09:     	movss	0x6953af(%rip), %xmm0   # 0x1429df1c0
142349e11:     	movss	0x68324b(%rip), %xmm1   # 0x1429cd064
142349e19:     	addq	$0x28, %rsp
142349e1d:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349e22:     	movss	0x4(%rax), %xmm2
142349e27:     	movss	0x5dbad5(%rip), %xmm0   # 0x142925904
142349e2f:     	movss	0x5dbb51(%rip), %xmm1   # 0x142925988
142349e37:     	addq	$0x28, %rsp
142349e3b:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349e40:     	movq	%rdx, %rcx
142349e43:     	callq	0x1408aea50 <.text+0x8ada50>
142349e48:     	movss	0x38(%rax), %xmm2
142349e4d:     	xorps	%xmm0, %xmm0
142349e50:     	movss	0x7d4a6c(%rip), %xmm1   # 0x142b1e8c4
142349e58:     	addq	$0x28, %rsp
142349e5c:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349e61:     	movss	0x8(%rax), %xmm2
142349e66:     	movss	0x5e4882(%rip), %xmm0   # 0x14292e6f0
142349e6e:     	movss	0x5ea81e(%rip), %xmm1   # 0x142934694
142349e76:     	addq	$0x28, %rsp
142349e7a:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349e7f:     	movaps	%xmm3, %xmm2
142349e82:     	movss	0x5dba4e(%rip), %xmm0   # 0x1429258d8
142349e8a:     	movss	0x5e485e(%rip), %xmm1   # 0x14292e6f0
142349e92:     	addq	$0x28, %rsp
142349e96:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349e9b:     	movq	%rdx, %rcx
142349e9e:     	callq	0x1408aea50 <.text+0x8ada50>
142349ea3:     	movss	0x24(%rax), %xmm2
142349ea8:     	xorps	%xmm0, %xmm0
142349eab:     	movss	0x683275(%rip), %xmm1   # 0x1429cd128
142349eb3:     	addq	$0x28, %rsp
142349eb7:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349ebc:     	movq	%rdx, %rcx
142349ebf:     	callq	0x1408aea50 <.text+0x8ada50>
142349ec4:     	movss	0x30(%rax), %xmm2
142349ec9:     	xorps	%xmm0, %xmm0
142349ecc:     	movss	0x5dba30(%rip), %xmm1   # 0x142925904
142349ed4:     	addq	$0x28, %rsp
142349ed8:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349edd:     	movq	%r8, %rcx
142349ee0:     	callq	0x140911240 <.text+0x910240>
142349ee5:     	movss	0x18(%rax), %xmm0
142349eea:     	callq	0x142923f50 <.text+0x2922f50>
142349eef:     	movaps	%xmm0, %xmm1
142349ef2:     	addss	0x68317e(%rip), %xmm1   # 0x1429cd078
142349efa:     	movss	0x5ea79a(%rip), %xmm0   # 0x14293469c
142349f02:     	movaps	%xmm0, %xmm3
142349f05:     	maxss	%xmm1, %xmm3
142349f09:     	movss	0x5e47df(%rip), %xmm1   # 0x14292e6f0
142349f11:     	jmp	0x142349fc4 <.text+0x2348fc4>
142349f16:     	movq	%rdx, %rcx
142349f19:     	callq	0x1408aea50 <.text+0x8ada50>
142349f1e:     	movss	0x20(%rax), %xmm2
142349f23:     	movss	0x5db9dd(%rip), %xmm0   # 0x142925908
142349f2b:     	movss	0x7a2c11(%rip), %xmm1   # 0x142aecb44
142349f33:     	addq	$0x28, %rsp
142349f37:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349f3c:     	movq	%rdx, %rcx
142349f3f:     	callq	0x1408aea50 <.text+0x8ada50>
142349f44:     	movss	0x34(%rax), %xmm2
142349f49:     	xorps	%xmm0, %xmm0
142349f4c:     	movss	0x6830d8(%rip), %xmm1   # 0x1429cd02c
142349f54:     	addq	$0x28, %rsp
142349f58:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349f5d:     	movq	%r8, %rcx
142349f60:     	callq	0x140911260 <.text+0x910260>
142349f65:     	movss	0x8(%rax), %xmm2
142349f6a:     	movss	0xc(%rax), %xmm0
142349f6f:     	movss	0x10(%rax), %xmm1
142349f74:     	jmp	0x142349faa <.text+0x2348faa>
142349f76:     	movss	(%rax), %xmm2
142349f7a:     	movss	0x77b73e(%rip), %xmm0   # 0x142ac56c0
142349f82:     	movss	0x68319e(%rip), %xmm1   # 0x1429cd128
142349f8a:     	addq	$0x28, %rsp
142349f8e:     	jmp	0x140c904a0 <.text+0xc8f4a0>
142349f93:     	movq	%r8, %rcx
142349f96:     	callq	0x140911260 <.text+0x910260>
142349f9b:     	movss	0x18(%rax), %xmm2
142349fa0:     	movss	0x1c(%rax), %xmm0
142349fa5:     	movss	0x20(%rax), %xmm1
142349faa:     	callq	0x140c904a0 <.text+0xc8f4a0>
142349faf:     	movaps	%xmm0, %xmm1
142349fb2:     	xorps	%xmm0, %xmm0
142349fb5:     	xorps	%xmm3, %xmm3
142349fb8:     	maxss	%xmm1, %xmm3
142349fbc:     	movss	0x5db940(%rip), %xmm1   # 0x142925904
142349fc4:     	movaps	%xmm1, %xmm2
142349fc7:     	minss	%xmm3, %xmm2
142349fcb:     	addq	$0x28, %rsp
142349fcf:     	jmp	0x140c904a0 <.text+0xc8f4a0>
