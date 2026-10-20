
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14089d9b0:     	pushq	%rbp
14089d9b1:     	pushq	%rsi
14089d9b2:     	pushq	%rdi
14089d9b3:     	subq	$0x30, %rsp
14089d9b7:     	leaq	0x30(%rsp), %rbp
14089d9bc:     	movq	$-0x2, -0x8(%rbp)
14089d9c4:     	movq	%rdx, %rdi
14089d9c7:     	movq	%rcx, %rsi
14089d9ca:     	movzbl	0x2d97238(%rip), %eax   # 0x143634c09
14089d9d1:     	movl	$0x13, %ecx
14089d9d6:     	movl	$0x1, %edx
14089d9db:     	callq	0x140613c10 <.text+0x612c10>
14089d9e0:     	testq	%rax, %rax
14089d9e3:     	je	0x14089da7f <.text+0x89ca7f>
14089d9e9:     	movups	0x21fba9c(%rip), %xmm0  # 0x142a9948c
14089d9f0:     	movups	%xmm0, (%rax)
14089d9f3:     	movl	$0x72656461, 0xf(%rax)  # imm = 0x72656461
14089d9fa:     	movups	(%rdi), %xmm0
14089d9fd:     	movups	0x10(%rdi), %xmm1
14089da01:     	movups	0x20(%rdi), %xmm2
14089da05:     	movups	%xmm0, 0x10(%rsi)
14089da09:     	movups	%xmm1, 0x20(%rsi)
14089da0d:     	movups	%xmm2, 0x30(%rsi)
14089da11:     	movl	$0x3e4ccccd, 0x78(%rsi) # imm = 0x3E4CCCCD
14089da18:     	movw	$0x100, 0x40(%rsi)      # imm = 0x100
14089da1e:     	movabsq	$0x13f800000, %rcx      # imm = 0x13F800000
14089da28:     	movq	%rcx, 0x44(%rsi)
14089da2c:     	movq	$0x13, 0x60(%rsi)
14089da34:     	movq	%rax, 0x68(%rsi)
14089da38:     	movq	$0x13, 0x70(%rsi)
14089da40:     	movq	$0x0, 0x81(%rsi)
14089da4b:     	movq	$0x0, 0x7c(%rsi)
14089da53:     	movaps	0x20a0666(%rip), %xmm0  # 0x14293e0c0
14089da5a:     	movaps	%xmm0, (%rsi)
14089da5d:     	movl	$0x0, 0x50(%rsi)
14089da64:     	movw	$0x0, 0x89(%rsi)
14089da6d:     	movl	$0x0, 0x58(%rsi)
14089da74:     	movq	%rsi, %rax
14089da77:     	addq	$0x30, %rsp
14089da7b:     	popq	%rdi
14089da7c:     	popq	%rsi
14089da7d:     	popq	%rbp
14089da7e:     	retq
14089da7f:     	movq	%rdi, -0x10(%rbp)
14089da83:     	leaq	0x21fb496(%rip), %r8    # 0x142a98f20
14089da8a:     	movl	$0x1, %ecx
14089da8f:     	movl	$0x13, %edx
14089da94:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089da99:     	ud2
14089da9b:     	nopl	(%rax,%rax)
14089daa0:     	movq	%rdx, 0x10(%rsp)
14089daa5:     	pushq	%rbp
14089daa6:     	pushq	%rsi
14089daa7:     	pushq	%rdi
14089daa8:     	subq	$0x20, %rsp
14089daac:     	leaq	0x30(%rdx), %rbp
14089dab0:     	movq	-0x10(%rbp), %rcx
14089dab4:     	callq	0x14089ada0 <.text+0x899da0>
14089dab9:     	nop
14089daba:     	addq	$0x20, %rsp
14089dabe:     	popq	%rdi
14089dabf:     	popq	%rsi
14089dac0:     	popq	%rbp
14089dac1:     	retq
14089dac2:     	int3
14089dac3:     	int3
14089dac4:     	int3
14089dac5:     	int3
14089dac6:     	int3
14089dac7:     	int3
14089dac8:     	int3
14089dac9:     	int3
14089daca:     	int3
14089dacb:     	int3
14089dacc:     	int3
14089dacd:     	int3
14089dace:     	int3
14089dacf:     	int3
14089dad0:     	pushq	%rsi
14089dad1:     	subq	$0x20, %rsp
14089dad5:     	movq	%rcx, %rsi
14089dad8:     	xorb	$0x1, %r8b
14089dadc:     	movb	%r8b, 0x8a(%rdx)
14089dae3:     	movl	$0x90, %r8d
14089dae9:     	callq	0x142923c00 <.text+0x2922c00>
14089daee:     	movq	%rsi, %rax
14089daf1:     	addq	$0x20, %rsp
14089daf5:     	popq	%rsi
14089daf6:     	retq
14089daf7:     	int3
14089daf8:     	int3
14089daf9:     	int3
14089dafa:     	int3
14089dafb:     	int3
14089dafc:     	int3
14089dafd:     	int3
14089dafe:     	int3
14089daff:     	int3
14089db00:     	pushq	%rbp
14089db01:     	pushq	%r15
14089db03:     	pushq	%r14
14089db05:     	pushq	%r12
14089db07:     	pushq	%rsi
14089db08:     	pushq	%rdi
14089db09:     	pushq	%rbx
14089db0a:     	subq	$0x30, %rsp
14089db0e:     	leaq	0x30(%rsp), %rbp
14089db13:     	movq	$-0x2, -0x8(%rbp)
14089db1b:     	movq	%r9, %rsi
14089db1e:     	movq	%rdx, %r12
14089db21:     	testq	%r9, %r9
14089db24:     	js	0x14089dbb5 <.text+0x89cbb5>
14089db2a:     	movq	%r8, %r14
14089db2d:     	movq	%rcx, %rdi
14089db30:     	je	0x14089db56 <.text+0x89cb56>
14089db32:     	movzbl	0x2d970d0(%rip), %eax   # 0x143634c09
14089db39:     	movl	$0x1, %r15d
14089db3f:     	movl	$0x1, %edx
14089db44:     	movq	%rsi, %rcx
14089db47:     	callq	0x140613c10 <.text+0x612c10>
14089db4c:     	testq	%rax, %rax
14089db4f:     	je	0x14089dbb8 <.text+0x89cbb8>
14089db51:     	movq	%rax, %rbx
14089db54:     	jmp	0x14089db5b <.text+0x89cb5b>
14089db56:     	movl	$0x1, %ebx
14089db5b:     	movq	%rbx, %rcx
14089db5e:     	movq	%r14, %rdx
14089db61:     	movq	%rsi, %r8
14089db64:     	callq	0x142923c00 <.text+0x2922c00>
14089db69:     	movq	0x60(%r12), %rdx
14089db6e:     	testq	%rdx, %rdx
14089db71:     	je	0x14089db83 <.text+0x89cb83>
14089db73:     	movq	0x68(%r12), %rcx
14089db78:     	movl	$0x1, %r8d
14089db7e:     	callq	0x140613c20 <.text+0x612c20>
14089db83:     	movq	%rsi, 0x60(%r12)
14089db88:     	movq	%rbx, 0x68(%r12)
14089db8d:     	movq	%rsi, 0x70(%r12)
14089db92:     	movl	$0x90, %r8d
14089db98:     	movq	%rdi, %rcx
14089db9b:     	movq	%r12, %rdx
14089db9e:     	callq	0x142923c00 <.text+0x2922c00>
14089dba3:     	movq	%rdi, %rax
14089dba6:     	addq	$0x30, %rsp
14089dbaa:     	popq	%rbx
14089dbab:     	popq	%rdi
14089dbac:     	popq	%rsi
14089dbad:     	popq	%r12
14089dbaf:     	popq	%r14
14089dbb1:     	popq	%r15
14089dbb3:     	popq	%rbp
14089dbb4:     	retq
14089dbb5:     	xorl	%r15d, %r15d
14089dbb8:     	movq	%r12, -0x10(%rbp)
14089dbbc:     	leaq	0x21fb35d(%rip), %r8    # 0x142a98f20
14089dbc3:     	movq	%r15, %rcx
14089dbc6:     	movq	%rsi, %rdx
14089dbc9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14089dbce:     	ud2
14089dbd0:     	movq	%rdx, 0x10(%rsp)
14089dbd5:     	pushq	%rbp
14089dbd6:     	pushq	%r15
14089dbd8:     	pushq	%r14
14089dbda:     	pushq	%r12
14089dbdc:     	pushq	%rsi
14089dbdd:     	pushq	%rdi
14089dbde:     	pushq	%rbx
14089dbdf:     	subq	$0x20, %rsp
14089dbe3:     	leaq	0x30(%rdx), %rbp
14089dbe7:     	movq	-0x10(%rbp), %rcx
14089dbeb:     	callq	0x140893950 <.text+0x892950>
14089dbf0:     	nop
14089dbf1:     	addq	$0x20, %rsp
14089dbf5:     	popq	%rbx
14089dbf6:     	popq	%rdi
14089dbf7:     	popq	%rsi
14089dbf8:     	popq	%r12
14089dbfa:     	popq	%r14
14089dbfc:     	popq	%r15
14089dbfe:     	popq	%rbp
14089dbff:     	retq
14089dc00:     	pushq	%rsi
14089dc01:     	subq	$0x20, %rsp
14089dc05:     	movq	%rcx, %rsi
14089dc08:     	movl	0x8(%r8), %eax
14089dc0c:     	movl	%eax, 0x84(%rdx)
14089dc12:     	movq	(%r8), %rax
14089dc15:     	movq	%rax, 0x7c(%rdx)
14089dc19:     	movl	$0x90, %r8d
14089dc1f:     	callq	0x142923c00 <.text+0x2922c00>
14089dc24:     	movq	%rsi, %rax
14089dc27:     	addq	$0x20, %rsp
14089dc2b:     	popq	%rsi
14089dc2c:     	retq
14089dc2d:     	int3
14089dc2e:     	int3
14089dc2f:     	int3
14089dc30:     	pushq	%rsi
14089dc31:     	subq	$0x20, %rsp
14089dc35:     	movq	%rcx, %rsi
14089dc38:     	movaps	(%r8), %xmm0
14089dc3c:     	movaps	%xmm0, (%rdx)
14089dc3f:     	movl	$0x90, %r8d
14089dc45:     	callq	0x142923c00 <.text+0x2922c00>
14089dc4a:     	movq	%rsi, %rax
14089dc4d:     	addq	$0x20, %rsp
14089dc51:     	popq	%rsi
14089dc52:     	retq
14089dc53:     	int3
14089dc54:     	int3
14089dc55:     	int3
14089dc56:     	int3
14089dc57:     	int3
14089dc58:     	int3
14089dc59:     	int3
14089dc5a:     	int3
14089dc5b:     	int3
14089dc5c:     	int3
14089dc5d:     	int3
14089dc5e:     	int3
14089dc5f:     	int3
14089dc60:     	pushq	%rsi
14089dc61:     	subq	$0x20, %rsp
14089dc65:     	movq	%rcx, %rsi
14089dc68:     	movl	$0x1, 0x50(%rdx)
14089dc6f:     	movss	%xmm2, 0x54(%rdx)
14089dc74:     	movl	$0x90, %r8d
14089dc7a:     	callq	0x142923c00 <.text+0x2922c00>
14089dc7f:     	movq	%rsi, %rax
14089dc82:     	addq	$0x20, %rsp
14089dc86:     	popq	%rsi
14089dc87:     	retq
14089dc88:     	int3
14089dc89:     	int3
14089dc8a:     	int3
14089dc8b:     	int3
14089dc8c:     	int3
14089dc8d:     	int3
14089dc8e:     	int3
14089dc8f:     	int3
14089dc90:     	pushq	%rsi
14089dc91:     	subq	$0x20, %rsp
14089dc95:     	movq	%rcx, %rsi
14089dc98:     	movss	%xmm2, 0x78(%rdx)
14089dc9d:     	movl	$0x90, %r8d
14089dca3:     	callq	0x142923c00 <.text+0x2922c00>
14089dca8:     	movq	%rsi, %rax
14089dcab:     	addq	$0x20, %rsp
14089dcaf:     	popq	%rsi
14089dcb0:     	retq
14089dcb1:     	int3
14089dcb2:     	int3
14089dcb3:     	int3
14089dcb4:     	int3
14089dcb5:     	int3
14089dcb6:     	int3
14089dcb7:     	int3
14089dcb8:     	int3
14089dcb9:     	int3
14089dcba:     	int3
14089dcbb:     	int3
14089dcbc:     	int3
14089dcbd:     	int3
14089dcbe:     	int3
14089dcbf:     	int3
14089dcc0:     	pushq	%rsi
14089dcc1:     	subq	$0x20, %rsp
14089dcc5:     	movq	%rcx, %rsi
14089dcc8:     	movl	$0x1, 0x58(%rdx)
14089dccf:     	movss	%xmm2, 0x5c(%rdx)
14089dcd4:     	movl	$0x90, %r8d
14089dcda:     	callq	0x142923c00 <.text+0x2922c00>
14089dcdf:     	movq	%rsi, %rax
14089dce2:     	addq	$0x20, %rsp
14089dce6:     	popq	%rsi
14089dce7:     	retq
14089dce8:     	int3
14089dce9:     	int3
14089dcea:     	int3
14089dceb:     	int3
14089dcec:     	int3
14089dced:     	int3
14089dcee:     	int3
14089dcef:     	int3
14089dcf0:     	pushq	%rsi
14089dcf1:     	subq	$0x20, %rsp
14089dcf5:     	movq	%rcx, %rsi
14089dcf8:     	movl	%r8d, 0x48(%rdx)
14089dcfc:     	movss	%xmm3, 0x4c(%rdx)
14089dd01:     	movl	$0x90, %r8d
14089dd07:     	callq	0x142923c00 <.text+0x2922c00>
14089dd0c:     	movq	%rsi, %rax
14089dd0f:     	addq	$0x20, %rsp
14089dd13:     	popq	%rsi
14089dd14:     	retq
14089dd15:     	int3
14089dd16:     	int3
14089dd17:     	int3
14089dd18:     	int3
14089dd19:     	int3
14089dd1a:     	int3
14089dd1b:     	int3
14089dd1c:     	int3
14089dd1d:     	int3
14089dd1e:     	int3
14089dd1f:     	int3
14089dd20:     	pushq	%rsi
14089dd21:     	subq	$0x20, %rsp
14089dd25:     	movq	%rcx, %rsi
14089dd28:     	movb	$0x1, 0x88(%rdx)
14089dd2f:     	movl	$0x90, %r8d
14089dd35:     	callq	0x142923c00 <.text+0x2922c00>
14089dd3a:     	movq	%rsi, %rax
14089dd3d:     	addq	$0x20, %rsp
14089dd41:     	popq	%rsi
14089dd42:     	retq
14089dd43:     	int3
14089dd44:     	int3
14089dd45:     	int3
14089dd46:     	int3
14089dd47:     	int3
14089dd48:     	int3
14089dd49:     	int3
14089dd4a:     	int3
14089dd4b:     	int3
14089dd4c:     	int3
14089dd4d:     	int3
14089dd4e:     	int3
14089dd4f:     	int3
14089dd50:     	pushq	%rbp
14089dd51:     	pushq	%r15
14089dd53:     	pushq	%r14
14089dd55:     	pushq	%r13
14089dd57:     	pushq	%r12
14089dd59:     	pushq	%rsi
14089dd5a:     	pushq	%rdi
14089dd5b:     	pushq	%rbx
14089dd5c:     	subq	$0x258, %rsp            # imm = 0x258
14089dd63:     	leaq	0x80(%rsp), %rbp
14089dd6b:     	movaps	%xmm9, 0x1c0(%rbp)
14089dd73:     	movaps	%xmm8, 0x1b0(%rbp)
14089dd7b:     	movaps	%xmm7, 0x1a0(%rbp)
14089dd82:     	movaps	%xmm6, 0x190(%rbp)
14089dd89:     	movq	$-0x2, 0x188(%rbp)
14089dd94:     	movq	%r9, %r14
14089dd97:     	movq	%r8, %rbx
14089dd9a:     	movq	0x10(%rdx), %r9
14089dd9e:     	movabsq	$-0x8000000000000000, %rdi # imm = 0x8000000000000000
14089dda8:     	movq	%r9, %rax
14089ddab:     	xorq	%rdi, %rax
14089ddae:     	cmpq	$0x5, %rax
14089ddb2:     	movl	$0x3, %r8d
14089ddb8:     	cmovbq	%rax, %r8
14089ddbc:     	movq	%rcx, 0xa8(%rbp)
14089ddc3:     	leaq	0x21fcdf6(%rip), %rax   # 0x142a9abc0
14089ddca:     	movslq	(%rax,%r8,4), %rcx
14089ddce:     	addq	%rax, %rcx
14089ddd1:     	movq	%rdx, 0x158(%rbp)
14089ddd8:     	movq	%rbx, 0xb0(%rbp)
14089dddf:     	jmpq	*%rcx
14089dde1:     	movq	0x18(%rdx), %rax
14089dde5:     	movq	%rax, 0x178(%rbp)
14089ddec:     	movq	0x20(%rdx), %rax
14089ddf0:     	movq	0x28(%rdx), %r8
14089ddf4:     	movq	%rax, 0x20(%rbp)
14089ddf8:     	movq	%r8, 0x28(%rbp)
14089ddfc:     	movq	%rbx, %rcx
14089ddff:     	movq	%rax, 0x170(%rbp)
14089de06:     	movq	%rax, %rdx
14089de09:     	callq	0x14092d570 <.text+0x92c570>
14089de0e:     	movq	%rdx, 0x160(%rbp)
14089de15:     	testb	$0x1, %al
14089de17:     	je	0x14089e771 <.text+0x89d771>
14089de1d:     	movq	0x178(%rbp), %rdx
14089de24:     	testq	%rdx, %rdx
14089de27:     	je	0x14089de3b <.text+0x89ce3b>
14089de29:     	movl	$0x1, %r8d
14089de2f:     	movq	0x170(%rbp), %rcx
14089de36:     	callq	0x140613c20 <.text+0x612c20>
14089de3b:     	movl	$0x0, 0x178(%rbp)
14089de45:     	jmp	0x14089e296 <.text+0x89d296>
14089de4a:     	movq	0x18(%rdx), %rax
14089de4e:     	movq	0x20(%rdx), %r8
14089de52:     	leaq	0xc0(%rbp), %rcx
14089de59:     	movq	%rax, 0x168(%rbp)
14089de60:     	movq	%rax, %rdx
14089de63:     	movq	%r8, 0x170(%rbp)
14089de6a:     	callq	*0x28(%r8)
14089de6e:     	movq	0xc8(%rbp), %r9
14089de75:     	movq	0xd0(%rbp), %rax
14089de7c:     	leaq	(%rax,%rax,2), %rax
14089de80:     	leaq	(%r9,%rax,4), %rax
14089de84:     	movq	%rax, 0x20(%rsp)
14089de89:     	leaq	0x21fb758(%rip), %rdx   # 0x142a995e8
14089de90:     	leaq	0x30(%rbp), %rcx
14089de94:     	movl	$0x13, %r8d
14089de9a:     	movq	%r9, 0x160(%rbp)
14089dea1:     	callq	0x140a6c120 <.text+0xa6b120>
14089dea6:     	movq	0xc0(%rbp), %rax
14089dead:     	testq	%rax, %rax
14089deb0:     	je	0x14089decc <.text+0x89cecc>
14089deb2:     	shlq	$0x2, %rax
14089deb6:     	leaq	(%rax,%rax,2), %rdx
14089deba:     	movl	$0x4, %r8d
14089dec0:     	movq	0x160(%rbp), %rcx
14089dec7:     	callq	0x140613c20 <.text+0x612c20>
14089decc:     	movq	0xa0(%rbp), %rax
14089ded3:     	movq	%rax, 0x130(%rbp)
14089deda:     	movups	0x90(%rbp), %xmm0
14089dee1:     	movaps	%xmm0, 0x120(%rbp)
14089dee8:     	movups	0x80(%rbp), %xmm0
14089deef:     	movaps	%xmm0, 0x110(%rbp)
14089def6:     	movups	0x70(%rbp), %xmm0
14089defa:     	movaps	%xmm0, 0x100(%rbp)
14089df01:     	movups	0x30(%rbp), %xmm0
14089df05:     	movups	0x40(%rbp), %xmm1
14089df09:     	movups	0x50(%rbp), %xmm2
14089df0d:     	movups	0x60(%rbp), %xmm3
14089df11:     	movaps	%xmm3, 0xf0(%rbp)
14089df18:     	movaps	%xmm2, 0xe0(%rbp)
14089df1f:     	movaps	%xmm1, 0xd0(%rbp)
14089df26:     	movaps	%xmm0, 0xc0(%rbp)
14089df2d:     	movq	%rdi, 0x138(%rbp)
14089df34:     	leaq	0xc0(%rbp), %rdx
14089df3b:     	movq	%rbx, %rcx
14089df3e:     	callq	0x14092cc60 <.text+0x92bc60>
14089df43:     	movq	%rax, 0x160(%rbp)
14089df4a:     	movq	0x170(%rbp), %rax
14089df51:     	movq	(%rax), %rax
14089df54:     	testq	%rax, %rax
14089df57:     	je	0x14089df62 <.text+0x89cf62>
14089df59:     	movq	0x168(%rbp), %rcx
14089df60:     	callq	*%rax
14089df62:     	movq	0x170(%rbp), %rcx
14089df69:     	movq	0x8(%rcx), %rdx
14089df6d:     	testq	%rdx, %rdx
14089df70:     	je	0x14089e28e <.text+0x89d28e>
14089df76:     	movb	$0x1, %al
14089df78:     	movl	%eax, 0x178(%rbp)
14089df7e:     	movq	0x10(%rcx), %r8
14089df82:     	jmp	0x14089e05f <.text+0x89d05f>
14089df87:     	movq	0x18(%rdx), %rax
14089df8b:     	movq	%rax, 0x170(%rbp)
14089df92:     	movq	0x20(%rdx), %r9
14089df96:     	movq	0x28(%rdx), %rax
14089df9a:     	leaq	(%rax,%rax,2), %rax
14089df9e:     	leaq	(%r9,%rax,4), %rax
14089dfa2:     	movq	%rax, 0x20(%rsp)
14089dfa7:     	leaq	0x21fb605(%rip), %rdx   # 0x142a995b3
14089dfae:     	leaq	-0x58(%rbp), %rcx
14089dfb2:     	movl	$0x10, %r8d
14089dfb8:     	movq	%r9, 0x168(%rbp)
14089dfbf:     	callq	0x140a6c120 <.text+0xa6b120>
14089dfc4:     	movq	0x18(%rbp), %rax
14089dfc8:     	movq	%rax, 0x130(%rbp)
14089dfcf:     	movups	0x8(%rbp), %xmm0
14089dfd3:     	movaps	%xmm0, 0x120(%rbp)
14089dfda:     	movups	-0x8(%rbp), %xmm0
14089dfde:     	movaps	%xmm0, 0x110(%rbp)
14089dfe5:     	movups	-0x18(%rbp), %xmm0
14089dfe9:     	movaps	%xmm0, 0x100(%rbp)
14089dff0:     	movups	-0x58(%rbp), %xmm0
14089dff4:     	movups	-0x48(%rbp), %xmm1
14089dff8:     	movups	-0x38(%rbp), %xmm2
14089dffc:     	movups	-0x28(%rbp), %xmm3
14089e000:     	movaps	%xmm3, 0xf0(%rbp)
14089e007:     	movaps	%xmm2, 0xe0(%rbp)
14089e00e:     	movaps	%xmm1, 0xd0(%rbp)
14089e015:     	movaps	%xmm0, 0xc0(%rbp)
14089e01c:     	movq	%rdi, 0x138(%rbp)
14089e023:     	leaq	0xc0(%rbp), %rdx
14089e02a:     	movq	%rbx, %rcx
14089e02d:     	callq	0x14092cc60 <.text+0x92bc60>
14089e032:     	movq	%rax, 0x160(%rbp)
14089e039:     	movq	0x170(%rbp), %rcx
14089e040:     	testq	%rcx, %rcx
14089e043:     	je	0x14089e28e <.text+0x89d28e>
14089e049:     	movb	$0x1, %al
14089e04b:     	movl	%eax, 0x178(%rbp)
14089e051:     	shlq	$0x2, %rcx
14089e055:     	leaq	(%rcx,%rcx,2), %rdx
14089e059:     	movl	$0x4, %r8d
14089e05f:     	movq	0x168(%rbp), %rcx
14089e066:     	callq	0x140613c20 <.text+0x612c20>
14089e06b:     	jmp	0x14089e296 <.text+0x89d296>
14089e070:     	movq	%r9, 0x168(%rbp)
14089e077:     	movq	0x18(%rdx), %r9
14089e07b:     	movq	0x20(%rdx), %rax
14089e07f:     	movq	0x28(%rdx), %rcx
14089e083:     	movq	%rcx, 0x170(%rbp)
14089e08a:     	movq	0x30(%rdx), %rcx
14089e08e:     	movq	%rcx, 0x178(%rbp)
14089e095:     	movq	0x38(%rdx), %rsi
14089e099:     	leaq	(%rax,%rax,2), %rax
14089e09d:     	leaq	(%r9,%rax,4), %rax
14089e0a1:     	movb	$0x0, 0x187(%rbp)
14089e0a8:     	movq	%rax, 0x20(%rsp)
14089e0ad:     	leaq	0x21fb50f(%rip), %rdx   # 0x142a995c3
14089e0b4:     	leaq	0x30(%rbp), %rcx
14089e0b8:     	movl	$0x19, %r8d
14089e0be:     	movq	%r9, 0xb8(%rbp)
14089e0c5:     	callq	0x140a6c120 <.text+0xa6b120>
14089e0ca:     	movq	0x170(%rbp), %rax
14089e0d1:     	movq	%rax, 0xc0(%rbp)
14089e0d8:     	movq	0x178(%rbp), %rax
14089e0df:     	movq	%rax, 0xc8(%rbp)
14089e0e6:     	movq	%rsi, 0xd0(%rbp)
14089e0ed:     	movb	$0x1, 0x186(%rbp)
14089e0f4:     	leaq	0x21fb4e1(%rip), %rdx   # 0x142a995dc
14089e0fb:     	leaq	0x30(%rbp), %rcx
14089e0ff:     	leaq	0xc0(%rbp), %r9
14089e106:     	movl	$0xc, %r8d
14089e10c:     	callq	0x140a34c90 <.text+0xa33c90>
14089e111:     	movq	0xa0(%rbp), %rax
14089e118:     	movq	%rax, 0x130(%rbp)
14089e11f:     	movups	0x90(%rbp), %xmm0
14089e126:     	movaps	%xmm0, 0x120(%rbp)
14089e12d:     	movups	0x80(%rbp), %xmm0
14089e134:     	movaps	%xmm0, 0x110(%rbp)
14089e13b:     	movups	0x70(%rbp), %xmm0
14089e13f:     	movaps	%xmm0, 0x100(%rbp)
14089e146:     	movups	0x30(%rbp), %xmm0
14089e14a:     	movups	0x40(%rbp), %xmm1
14089e14e:     	movups	0x50(%rbp), %xmm2
14089e152:     	movups	0x60(%rbp), %xmm3
14089e156:     	movaps	%xmm3, 0xf0(%rbp)
14089e15d:     	movaps	%xmm2, 0xe0(%rbp)
14089e164:     	movaps	%xmm1, 0xd0(%rbp)
14089e16b:     	movaps	%xmm0, 0xc0(%rbp)
14089e172:     	movq	%rdi, 0x138(%rbp)
14089e179:     	movb	$0x0, 0x186(%rbp)
14089e180:     	leaq	0xc0(%rbp), %rdx
14089e187:     	movq	%rbx, %rcx
14089e18a:     	callq	0x14092cc60 <.text+0x92bc60>
14089e18f:     	movq	%rax, 0x160(%rbp)
14089e196:     	movq	0x168(%rbp), %rcx
14089e19d:     	testq	%rcx, %rcx
14089e1a0:     	je	0x14089e28e <.text+0x89d28e>
14089e1a6:     	movb	$0x1, %al
14089e1a8:     	movl	%eax, 0x178(%rbp)
14089e1ae:     	shlq	$0x2, %rcx
14089e1b2:     	leaq	(%rcx,%rcx,2), %rdx
14089e1b6:     	movl	$0x4, %r8d
14089e1bc:     	movq	0xb8(%rbp), %rcx
14089e1c3:     	callq	0x140613c20 <.text+0x612c20>
14089e1c8:     	jmp	0x14089e296 <.text+0x89d296>
14089e1cd:     	xorl	%ecx, %ecx
14089e1cf:     	callq	0x1400d9b20 <.text+0xd8b20>
14089e1d4:     	testq	%rax, %rax
14089e1d7:     	je	0x14089e7d3 <.text+0x89d7d3>
14089e1dd:     	movups	(%rax), %xmm0
14089e1e0:     	incq	(%rax)
14089e1e3:     	movups	0x22282d6(%rip), %xmm1  # 0x142ac64c0
14089e1ea:     	movups	%xmm1, 0x48(%rbp)
14089e1ee:     	movups	0x22282db(%rip), %xmm1  # 0x142ac64d0
14089e1f5:     	movups	%xmm1, 0x58(%rbp)
14089e1f9:     	movq	$0x0, 0xc0(%rbp)
14089e204:     	movq	$0x4, 0xc8(%rbp)
14089e20f:     	movq	$0x0, 0xd0(%rbp)
14089e21a:     	movq	%rdi, 0xd8(%rbp)
14089e221:     	movups	0x30(%rbp), %xmm1
14089e225:     	movups	%xmm1, 0xe0(%rbp)
14089e22c:     	movups	0x40(%rbp), %xmm1
14089e230:     	movups	%xmm1, 0xf0(%rbp)
14089e237:     	movups	0x50(%rbp), %xmm1
14089e23b:     	movups	%xmm1, 0x100(%rbp)
14089e242:     	movq	0x60(%rbp), %rax
14089e246:     	movq	%rax, 0x110(%rbp)
14089e24d:     	movups	%xmm0, 0x118(%rbp)
14089e254:     	leaq	0x21fb315(%rip), %rax   # 0x142a99570
14089e25b:     	movq	%rax, 0x128(%rbp)
14089e262:     	movq	$0xf, 0x130(%rbp)
14089e26d:     	movq	%rdi, 0x138(%rbp)
14089e274:     	leaq	0xc0(%rbp), %rdx
14089e27b:     	movq	0xb0(%rbp), %rcx
14089e282:     	callq	0x14092cc60 <.text+0x92bc60>
14089e287:     	movq	%rax, 0x160(%rbp)
14089e28e:     	movb	$0x1, %al
14089e290:     	movl	%eax, 0x178(%rbp)
14089e296:     	movq	0x240(%rbp), %rdx
14089e29d:     	leaq	0xc0(%rbp), %rcx
14089e2a4:     	callq	0x140d146a0 <.text+0xd136a0>
14089e2a9:     	movq	0x158(%rbp), %rax
14089e2b0:     	cmpb	$0x0, 0x50(%rax)
14089e2b4:     	jne	0x14089e2c6 <.text+0x89d2c6>
14089e2b6:     	movss	0x2087646(%rip), %xmm6  # 0x142925904
14089e2be:     	cmpb	$0x0, 0x40(%rax)
14089e2c2:     	je	0x14089e2d1 <.text+0x89d2d1>
14089e2c4:     	jmp	0x14089e2dd <.text+0x89d2dd>
14089e2c6:     	movss	0x54(%rax), %xmm6
14089e2cb:     	cmpb	$0x0, 0x40(%rax)
14089e2cf:     	jne	0x14089e2dd <.text+0x89d2dd>
14089e2d1:     	movb	$0x0, 0x40(%rax)
14089e2d5:     	movss	%xmm6, 0x44(%rax)
14089e2da:     	xorps	%xmm6, %xmm6
14089e2dd:     	movq	0x68(%rax), %r15
14089e2e1:     	movq	0x70(%rax), %r8
14089e2e5:     	movq	%r14, %rcx
14089e2e8:     	movq	%r15, %rdx
14089e2eb:     	callq	0x140937aa0 <.text+0x936aa0>
14089e2f0:     	testb	$0x1, %al
14089e2f2:     	je	0x14089e763 <.text+0x89d763>
14089e2f8:     	movq	%rdx, %r13
14089e2fb:     	movq	0xc8(%rbp), %r14
14089e302:     	testq	%r14, %r14
14089e305:     	jne	0x14089e30e <.text+0x89d30e>
14089e307:     	movq	0xd0(%rbp), %r14
14089e30e:     	movq	0x158(%rbp), %rax
14089e315:     	movaps	(%rax), %xmm9
14089e319:     	movss	0x78(%rax), %xmm7
14089e31e:     	movl	0x48(%rax), %esi
14089e321:     	movss	0x4c(%rax), %xmm8
14089e327:     	movq	0x40(%rax), %rbx
14089e32b:     	movq	0xe0(%rbp), %rdi
14089e332:     	movq	(%r14), %rax
14089e335:     	movq	0x10(%r14), %r12
14089e339:     	subq	%r12, %rax
14089e33c:     	cmpq	$0x97, %rax
14089e342:     	jbe	0x14089e685 <.text+0x89d685>
14089e348:     	movq	0x158(%rbp), %r8
14089e34f:     	leaq	0x7c(%r8), %rax
14089e353:     	movq	0x8(%r14), %rcx
14089e357:     	leaq	0xa3d332(%rip), %rdx    # 0x1412db690 <.text+0x12da690>
14089e35e:     	movq	%rdx, (%rcx,%r12)
14089e362:     	movups	%xmm9, 0x8(%rcx,%r12)
14089e368:     	movq	(%rax), %rdx
14089e36b:     	movq	%rdx, 0x18(%rcx,%r12)
14089e370:     	movl	0x8(%rax), %eax
14089e373:     	movl	%eax, 0x20(%rcx,%r12)
14089e378:     	movss	%xmm6, 0x24(%rcx,%r12)
14089e37f:     	movss	%xmm6, 0x28(%rcx,%r12)
14089e386:     	movss	%xmm6, 0x2c(%rcx,%r12)
14089e38d:     	movq	0x160(%rbp), %rax
14089e394:     	movq	%rax, 0x38(%rcx,%r12)
14089e399:     	movq	%r13, 0x40(%rcx,%r12)
14089e39e:     	movl	%esi, 0x48(%rcx,%r12)
14089e3a3:     	movss	%xmm8, 0x4c(%rcx,%r12)
14089e3aa:     	movq	%rbx, 0x50(%rcx,%r12)
14089e3af:     	movss	%xmm7, 0x58(%rcx,%r12)
14089e3b6:     	movq	$0x0, 0x5c(%rcx,%r12)
14089e3bf:     	movb	$0x0, 0x68(%rcx,%r12)
14089e3c5:     	movq	%rdi, 0x78(%rcx,%r12)
14089e3ca:     	leaq	0xcc695f(%rip), %rbx    # 0x141564d30 <.text+0x1563d30>
14089e3d1:     	movq	%rbx, 0x88(%rcx,%r12)
14089e3d9:     	leaq	0x98(%r12), %r13
14089e3e1:     	movq	%r13, 0x10(%r14)
14089e3e5:     	cmpb	$0x0, 0x89(%r8)
14089e3ed:     	je	0x14089e475 <.text+0x89d475>
14089e3f3:     	movq	(%r14), %rax
14089e3f6:     	subq	%r13, %rax
14089e3f9:     	cmpq	$0x1f, %rax
14089e3fd:     	jbe	0x14089e6aa <.text+0x89d6aa>
14089e403:     	movq	0x8(%r14), %rax
14089e407:     	leaq	0xa38792(%rip), %rcx    # 0x1412d6ba0 <.text+0x12d5ba0>
14089e40e:     	movq	%rcx, 0x98(%rax,%r12)
14089e416:     	movq	%rbx, 0xa0(%rax,%r12)
14089e41e:     	movq	%rdi, 0xa8(%rax,%r12)
14089e426:     	movb	$0x0, 0xb0(%rax,%r12)
14089e42f:     	leaq	0xb8(%r12), %r13
14089e437:     	movq	%r13, 0x10(%r14)
14089e43b:     	movq	(%r14), %rax
14089e43e:     	subq	%r13, %rax
14089e441:     	movl	$0xd8, %esi
14089e446:     	cmpq	$0x1f, %rax
14089e44a:     	ja	0x14089e509 <.text+0x89d509>
14089e450:     	movq	$0x1, 0x20(%rsp)
14089e459:     	movl	$0x20, %r8d
14089e45f:     	movl	$0x1, %r9d
14089e465:     	movq	%r14, %rcx
14089e468:     	movq	%r13, %rdx
14089e46b:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e470:     	jmp	0x14089e509 <.text+0x89d509>
14089e475:     	movq	(%r14), %rax
14089e478:     	subq	%r13, %rax
14089e47b:     	cmpb	$0x0, 0x8a(%r8)
14089e483:     	je	0x14089e4b2 <.text+0x89d4b2>
14089e485:     	movl	$0xb8, %esi
14089e48a:     	cmpq	$0x1f, %rax
14089e48e:     	ja	0x14089e509 <.text+0x89d509>
14089e490:     	movq	$0x1, 0x20(%rsp)
14089e499:     	movl	$0x20, %r8d
14089e49f:     	movl	$0x1, %r9d
14089e4a5:     	movq	%r14, %rcx
14089e4a8:     	movq	%r13, %rdx
14089e4ab:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e4b0:     	jmp	0x14089e509 <.text+0x89d509>
14089e4b2:     	cmpq	$0x1f, %rax
14089e4b6:     	jbe	0x14089e719 <.text+0x89d719>
14089e4bc:     	movq	0x8(%r14), %rax
14089e4c0:     	leaq	0xa3b1d9(%rip), %rcx    # 0x1412d96a0 <.text+0x12d86a0>
14089e4c7:     	movq	%rcx, 0x98(%rax,%r12)
14089e4cf:     	movq	%rbx, 0xa0(%rax,%r12)
14089e4d7:     	movq	%rdi, 0xa8(%rax,%r12)
14089e4df:     	movb	$0x0, 0xb0(%rax,%r12)
14089e4e8:     	leaq	0xb8(%r12), %r13
14089e4f0:     	movq	%r13, 0x10(%r14)
14089e4f4:     	movq	(%r14), %rax
14089e4f7:     	subq	%r13, %rax
14089e4fa:     	movl	$0xd8, %esi
14089e4ff:     	cmpq	$0x1f, %rax
14089e503:     	jbe	0x14089e73e <.text+0x89d73e>
14089e509:     	movq	0x8(%r14), %rax
14089e50d:     	leaq	0xa39c4c(%rip), %rcx    # 0x1412d8160 <.text+0x12d7160>
14089e514:     	movq	%rcx, (%rax,%r13)
14089e518:     	movq	%rbx, 0x8(%rax,%r13)
14089e51d:     	movq	%rdi, 0x10(%rax,%r13)
14089e522:     	movb	$0x0, 0x18(%rax,%r13)
14089e528:     	addq	%rsi, %r12
14089e52b:     	movq	%r12, 0x10(%r14)
14089e52f:     	movq	0x158(%rbp), %rax
14089e536:     	testb	$0x1, 0x58(%rax)
14089e53a:     	je	0x14089e58f <.text+0x89d58f>
14089e53c:     	movss	0x5c(%rax), %xmm6
14089e541:     	movq	(%r14), %rax
14089e544:     	subq	%r12, %rax
14089e547:     	cmpq	$0x27, %rax
14089e54b:     	jbe	0x14089e6cf <.text+0x89d6cf>
14089e551:     	movq	0x8(%r14), %rax
14089e555:     	leaq	0xa39d44(%rip), %rcx    # 0x1412d82a0 <.text+0x12d72a0>
14089e55c:     	movq	%rcx, (%rax,%r12)
14089e560:     	movq	%rbx, 0x8(%rax,%r12)
14089e565:     	movq	%rdi, 0x10(%rax,%r12)
14089e56a:     	movl	$0x0, 0x18(%rax,%r12)
14089e573:     	movss	%xmm6, 0x1c(%rax,%r12)
14089e57a:     	movb	$0x0, 0x20(%rax,%r12)
14089e580:     	addq	$0x28, %r12
14089e584:     	movq	%r12, 0x10(%r14)
14089e588:     	movq	0x158(%rbp), %rax
14089e58f:     	cmpb	$0x0, 0x88(%rax)
14089e596:     	je	0x14089e5cf <.text+0x89d5cf>
14089e598:     	movq	(%r14), %rax
14089e59b:     	subq	%r12, %rax
14089e59e:     	cmpq	$0x1f, %rax
14089e5a2:     	jbe	0x14089e6f4 <.text+0x89d6f4>
14089e5a8:     	movq	0x8(%r14), %rax
14089e5ac:     	leaq	0xa3ce5d(%rip), %rcx    # 0x1412db410 <.text+0x12da410>
14089e5b3:     	movq	%rcx, (%rax,%r12)
14089e5b7:     	movq	%rbx, 0x8(%rax,%r12)
14089e5bc:     	movq	%rdi, 0x10(%rax,%r12)
14089e5c1:     	movb	$0x0, 0x18(%rax,%r12)
14089e5c7:     	addq	$0x20, %r12
14089e5cb:     	movq	%r12, 0x10(%r14)
14089e5cf:     	cmpb	$0x0, 0x178(%rbp)
14089e5d6:     	je	0x14089e607 <.text+0x89d607>
14089e5d8:     	movq	0xb0(%rbp), %rax
14089e5df:     	movq	0x18(%rax), %rdx
14089e5e3:     	lock
14089e5e4:     	incq	(%rdx)
14089e5e7:     	jle	0x14089e7df <.text+0x89d7df>
14089e5ed:     	leaq	0x21fb024(%rip), %r9    # 0x142a99618
14089e5f4:     	leaq	0xc0(%rbp), %rcx
14089e5fb:     	movq	0x160(%rbp), %r8
14089e602:     	callq	0x1412de950 <.text+0x12dd950>
14089e607:     	movq	0xe0(%rbp), %rax
14089e60e:     	movq	0xa8(%rbp), %rcx
14089e615:     	movq	%rax, 0x20(%rcx)
14089e619:     	movups	0xc0(%rbp), %xmm0
14089e620:     	movups	0xd0(%rbp), %xmm1
14089e627:     	movups	%xmm1, 0x10(%rcx)
14089e62b:     	movups	%xmm0, (%rcx)
14089e62e:     	movq	0x158(%rbp), %rax
14089e635:     	movq	0x60(%rax), %rdx
14089e639:     	testq	%rdx, %rdx
14089e63c:     	je	0x14089e64c <.text+0x89d64c>
14089e63e:     	movl	$0x1, %r8d
14089e644:     	movq	%r15, %rcx
14089e647:     	callq	0x140613c20 <.text+0x612c20>
14089e64c:     	movq	0xa8(%rbp), %rax
14089e653:     	movaps	0x190(%rbp), %xmm6
14089e65a:     	movaps	0x1a0(%rbp), %xmm7
14089e661:     	movaps	0x1b0(%rbp), %xmm8
14089e669:     	movaps	0x1c0(%rbp), %xmm9
14089e671:     	addq	$0x258, %rsp            # imm = 0x258
14089e678:     	popq	%rbx
14089e679:     	popq	%rdi
14089e67a:     	popq	%rsi
14089e67b:     	popq	%r12
14089e67d:     	popq	%r13
14089e67f:     	popq	%r14
14089e681:     	popq	%r15
14089e683:     	popq	%rbp
14089e684:     	retq
14089e685:     	movq	$0x1, 0x20(%rsp)
14089e68e:     	movl	$0x98, %r8d
14089e694:     	movl	$0x1, %r9d
14089e69a:     	movq	%r14, %rcx
14089e69d:     	movq	%r12, %rdx
14089e6a0:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e6a5:     	jmp	0x14089e348 <.text+0x89d348>
14089e6aa:     	movq	$0x1, 0x20(%rsp)
14089e6b3:     	movl	$0x20, %r8d
14089e6b9:     	movl	$0x1, %r9d
14089e6bf:     	movq	%r14, %rcx
14089e6c2:     	movq	%r13, %rdx
14089e6c5:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e6ca:     	jmp	0x14089e403 <.text+0x89d403>
14089e6cf:     	movq	$0x1, 0x20(%rsp)
14089e6d8:     	movl	$0x28, %r8d
14089e6de:     	movl	$0x1, %r9d
14089e6e4:     	movq	%r14, %rcx
14089e6e7:     	movq	%r12, %rdx
14089e6ea:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e6ef:     	jmp	0x14089e551 <.text+0x89d551>
14089e6f4:     	movq	$0x1, 0x20(%rsp)
14089e6fd:     	movl	$0x20, %r8d
14089e703:     	movl	$0x1, %r9d
14089e709:     	movq	%r14, %rcx
14089e70c:     	movq	%r12, %rdx
14089e70f:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e714:     	jmp	0x14089e5a8 <.text+0x89d5a8>
14089e719:     	movq	$0x1, 0x20(%rsp)
14089e722:     	movl	$0x20, %r8d
14089e728:     	movl	$0x1, %r9d
14089e72e:     	movq	%r14, %rcx
14089e731:     	movq	%r13, %rdx
14089e734:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e739:     	jmp	0x14089e4bc <.text+0x89d4bc>
14089e73e:     	movq	$0x1, 0x20(%rsp)
14089e747:     	movl	$0x20, %r8d
14089e74d:     	movl	$0x1, %r9d
14089e753:     	movq	%r14, %rcx
14089e756:     	movq	%r13, %rdx
14089e759:     	callq	0x1428a9a60 <.text+0x28a8a60>
14089e75e:     	jmp	0x14089e509 <.text+0x89d509>
14089e763:     	leaq	0x21fae96(%rip), %rcx   # 0x142a99600
14089e76a:     	callq	0x1428d9310 <.text+0x28d8310>
14089e76f:     	jmp	0x14089e7df <.text+0x89d7df>
14089e771:     	leaq	0x20(%rbp), %rax
14089e775:     	movq	%rax, 0x30(%rbp)
14089e779:     	leaq	-0x36aee0(%rip), %rax   # 0x1405338a0 <.text+0x5328a0>
14089e780:     	movq	%rax, 0x38(%rbp)
14089e784:     	leaq	0x221154d(%rip), %rax   # 0x142aafcd8
14089e78b:     	movq	%rax, 0xc0(%rbp)
14089e792:     	movq	$0x2, 0xc8(%rbp)
14089e79d:     	movq	$0x0, 0xe0(%rbp)
14089e7a8:     	leaq	0x30(%rbp), %rax
14089e7ac:     	movq	%rax, 0xd0(%rbp)
14089e7b3:     	movq	$0x1, 0xd8(%rbp)
14089e7be:     	leaq	0x2211533(%rip), %rdx   # 0x142aafcf8
14089e7c5:     	leaq	0xc0(%rbp), %rcx
14089e7cc:     	callq	0x1428d9390 <.text+0x28d8390>
14089e7d1:     	jmp	0x14089e7df <.text+0x89d7df>
14089e7d3:     	leaq	0x239c2c6(%rip), %rcx   # 0x142c3aaa0
14089e7da:     	callq	0x1428d80d0 <.text+0x28d70d0>
14089e7df:     	ud2
14089e7e1:     	nopw	%cs:(%rax,%rax)
14089e7f0:     	movq	%rdx, 0x10(%rsp)
14089e7f5:     	pushq	%rbp
14089e7f6:     	pushq	%r15
14089e7f8:     	pushq	%r14
14089e7fa:     	pushq	%r13
14089e7fc:     	pushq	%r12
14089e7fe:     	pushq	%rsi
14089e7ff:     	pushq	%rdi
14089e800:     	pushq	%rbx
14089e801:     	subq	$0x68, %rsp
14089e805:     	leaq	0x80(%rdx), %rbp
14089e80c:     	movaps	%xmm9, 0x20(%rsp)
14089e812:     	movaps	%xmm8, 0x30(%rsp)
14089e818:     	movaps	%xmm7, 0x40(%rsp)
14089e81d:     	movaps	%xmm6, 0x50(%rsp)
14089e822:     	cmpq	$0x0, 0x178(%rbp)
14089e82a:     	je	0x14089e845 <.text+0x89d845>
14089e82c:     	movl	$0x1, %r8d
14089e832:     	movq	0x170(%rbp), %rcx
14089e839:     	movq	0x178(%rbp), %rdx
14089e840:     	callq	0x140613c20 <.text+0x612c20>
14089e845:     	movaps	0x50(%rsp), %xmm6
14089e84a:     	movaps	0x40(%rsp), %xmm7
14089e84f:     	movaps	0x30(%rsp), %xmm8
14089e855:     	movaps	0x20(%rsp), %xmm9
14089e85b:     	addq	$0x68, %rsp
14089e85f:     	popq	%rbx
14089e860:     	popq	%rdi
14089e861:     	popq	%rsi
14089e862:     	popq	%r12
14089e864:     	popq	%r13
14089e866:     	popq	%r14
14089e868:     	popq	%r15
14089e86a:     	popq	%rbp
14089e86b:     	retq
14089e86c:     	nopl	(%rax)
14089e870:     	movq	%rdx, 0x10(%rsp)
14089e875:     	pushq	%rbp
14089e876:     	pushq	%r15
14089e878:     	pushq	%r14
14089e87a:     	pushq	%r13
14089e87c:     	pushq	%r12
14089e87e:     	pushq	%rsi
14089e87f:     	pushq	%rdi
14089e880:     	pushq	%rbx
14089e881:     	subq	$0x68, %rsp
14089e885:     	leaq	0x80(%rdx), %rbp
14089e88c:     	movaps	%xmm9, 0x20(%rsp)
14089e892:     	movaps	%xmm8, 0x30(%rsp)
14089e898:     	movaps	%xmm7, 0x40(%rsp)
14089e89d:     	movaps	%xmm6, 0x50(%rsp)
14089e8a2:     	movq	0x158(%rbp), %rax
14089e8a9:     	movq	0x60(%rax), %rdx
14089e8ad:     	testq	%rdx, %rdx
14089e8b0:     	je	0x14089e8c8 <.text+0x89d8c8>
14089e8b2:     	movq	0x158(%rbp), %rax
14089e8b9:     	movq	0x68(%rax), %rcx
14089e8bd:     	movl	$0x1, %r8d
14089e8c3:     	callq	0x140613c20 <.text+0x612c20>
14089e8c8:     	movaps	0x50(%rsp), %xmm6
14089e8cd:     	movaps	0x40(%rsp), %xmm7
14089e8d2:     	movaps	0x30(%rsp), %xmm8
14089e8d8:     	movaps	0x20(%rsp), %xmm9
14089e8de:     	addq	$0x68, %rsp
14089e8e2:     	popq	%rbx
14089e8e3:     	popq	%rdi
14089e8e4:     	popq	%rsi
14089e8e5:     	popq	%r12
14089e8e7:     	popq	%r13
14089e8e9:     	popq	%r14
14089e8eb:     	popq	%r15
14089e8ed:     	popq	%rbp
14089e8ee:     	retq
14089e8ef:     	nop
14089e8f0:     	movq	%rdx, 0x10(%rsp)
14089e8f5:     	pushq	%rbp
14089e8f6:     	pushq	%r15
14089e8f8:     	pushq	%r14
14089e8fa:     	pushq	%r13
14089e8fc:     	pushq	%r12
14089e8fe:     	pushq	%rsi
14089e8ff:     	pushq	%rdi
14089e900:     	pushq	%rbx
14089e901:     	subq	$0x68, %rsp
14089e905:     	leaq	0x80(%rdx), %rbp
14089e90c:     	movaps	%xmm9, 0x20(%rsp)
14089e912:     	movaps	%xmm8, 0x30(%rsp)
14089e918:     	movaps	%xmm7, 0x40(%rsp)
14089e91d:     	movaps	%xmm6, 0x50(%rsp)
14089e922:     	cmpq	$0x0, 0x170(%rbp)
14089e92a:     	je	0x14089e94d <.text+0x89d94d>
14089e92c:     	movq	0x170(%rbp), %rax
14089e933:     	shlq	$0x2, %rax
14089e937:     	leaq	(%rax,%rax,2), %rdx
14089e93b:     	movl	$0x4, %r8d
14089e941:     	movq	0x168(%rbp), %rcx
14089e948:     	callq	0x140613c20 <.text+0x612c20>
14089e94d:     	movaps	0x50(%rsp), %xmm6
14089e952:     	movaps	0x40(%rsp), %xmm7
14089e957:     	movaps	0x30(%rsp), %xmm8
14089e95d:     	movaps	0x20(%rsp), %xmm9
14089e963:     	addq	$0x68, %rsp
14089e967:     	popq	%rbx
14089e968:     	popq	%rdi
14089e969:     	popq	%rsi
14089e96a:     	popq	%r12
14089e96c:     	popq	%r13
14089e96e:     	popq	%r14
14089e970:     	popq	%r15
14089e972:     	popq	%rbp
14089e973:     	retq
14089e974:     	nopw	%cs:(%rax,%rax)
14089e980:     	movq	%rdx, 0x10(%rsp)
14089e985:     	pushq	%rbp
14089e986:     	pushq	%r15
14089e988:     	pushq	%r14
14089e98a:     	pushq	%r13
14089e98c:     	pushq	%r12
14089e98e:     	pushq	%rsi
14089e98f:     	pushq	%rdi
14089e990:     	pushq	%rbx
14089e991:     	subq	$0x68, %rsp
14089e995:     	leaq	0x80(%rdx), %rbp
14089e99c:     	movaps	%xmm9, 0x20(%rsp)
14089e9a2:     	movaps	%xmm8, 0x30(%rsp)
14089e9a8:     	movaps	%xmm7, 0x40(%rsp)
14089e9ad:     	movaps	%xmm6, 0x50(%rsp)
14089e9b2:     	cmpb	$0x0, 0x186(%rbp)
14089e9b9:     	je	0x14089e9f2 <.text+0x89d9f2>
14089e9bb:     	leaq	0x30(%rbp), %rcx
14089e9bf:     	callq	0x140888b30 <.text+0x887b30>
14089e9c4:     	movb	$0x1, 0x187(%rbp)
14089e9cb:     	movaps	0x50(%rsp), %xmm6
14089e9d0:     	movaps	0x40(%rsp), %xmm7
14089e9d5:     	movaps	0x30(%rsp), %xmm8
14089e9db:     	movaps	0x20(%rsp), %xmm9
14089e9e1:     	addq	$0x68, %rsp
14089e9e5:     	popq	%rbx
14089e9e6:     	popq	%rdi
14089e9e7:     	popq	%rsi
14089e9e8:     	popq	%r12
14089e9ea:     	popq	%r13
14089e9ec:     	popq	%r14
14089e9ee:     	popq	%r15
14089e9f0:     	popq	%rbp
14089e9f1:     	retq
14089e9f2:     	movb	$0x1, 0x187(%rbp)
14089e9f9:     	movaps	0x50(%rsp), %xmm6
14089e9fe:     	movaps	0x40(%rsp), %xmm7
14089ea03:     	movaps	0x30(%rsp), %xmm8
14089ea09:     	movaps	0x20(%rsp), %xmm9
14089ea0f:     	addq	$0x68, %rsp
14089ea13:     	popq	%rbx
14089ea14:     	popq	%rdi
14089ea15:     	popq	%rsi
14089ea16:     	popq	%r12
14089ea18:     	popq	%r13
14089ea1a:     	popq	%r14
14089ea1c:     	popq	%r15
14089ea1e:     	popq	%rbp
14089ea1f:     	retq
14089ea20:     	movq	%rdx, 0x10(%rsp)
14089ea25:     	pushq	%rbp
14089ea26:     	pushq	%r15
14089ea28:     	pushq	%r14
14089ea2a:     	pushq	%r13
14089ea2c:     	pushq	%r12
14089ea2e:     	pushq	%rsi
14089ea2f:     	pushq	%rdi
14089ea30:     	pushq	%rbx
14089ea31:     	subq	$0x68, %rsp
14089ea35:     	leaq	0x80(%rdx), %rbp
14089ea3c:     	movaps	%xmm9, 0x20(%rsp)
14089ea42:     	movaps	%xmm8, 0x30(%rsp)
14089ea48:     	movaps	%xmm7, 0x40(%rsp)
14089ea4d:     	movaps	%xmm6, 0x50(%rsp)
14089ea52:     	movq	0x178(%rbp), %rcx
14089ea59:     	movq	0x170(%rbp), %rax
14089ea60:     	movq	%rax, %rdx
14089ea63:     	testq	%rax, %rax
14089ea66:     	sete	%al
14089ea69:     	orb	0x187(%rbp), %al
14089ea6f:     	testb	$0x1, %al
14089ea71:     	jne	0x14089ea86 <.text+0x89da86>
14089ea73:     	shlq	$0x2, %rdx
14089ea77:     	leaq	(%rdx,%rdx,2), %rdx
14089ea7b:     	movl	$0x4, %r8d
14089ea81:     	callq	0x140613c20 <.text+0x612c20>
14089ea86:     	cmpq	$0x0, 0x168(%rbp)
14089ea8e:     	je	0x14089eab1 <.text+0x89dab1>
14089ea90:     	movq	0x168(%rbp), %rax
14089ea97:     	shlq	$0x2, %rax
14089ea9b:     	leaq	(%rax,%rax,2), %rdx
14089ea9f:     	movl	$0x4, %r8d
14089eaa5:     	movq	0xb8(%rbp), %rcx
14089eaac:     	callq	0x140613c20 <.text+0x612c20>
14089eab1:     	movaps	0x50(%rsp), %xmm6
14089eab6:     	movaps	0x40(%rsp), %xmm7
14089eabb:     	movaps	0x30(%rsp), %xmm8
14089eac1:     	movaps	0x20(%rsp), %xmm9
14089eac7:     	addq	$0x68, %rsp
14089eacb:     	popq	%rbx
14089eacc:     	popq	%rdi
14089eacd:     	popq	%rsi
14089eace:     	popq	%r12
14089ead0:     	popq	%r13
14089ead2:     	popq	%r14
14089ead4:     	popq	%r15
14089ead6:     	popq	%rbp
14089ead7:     	retq
14089ead8:     	nopl	(%rax,%rax)
14089eae0:     	movq	%rdx, 0x10(%rsp)
14089eae5:     	pushq	%rbp
14089eae6:     	pushq	%r15
14089eae8:     	pushq	%r14
14089eaea:     	pushq	%r13
14089eaec:     	pushq	%r12
14089eaee:     	pushq	%rsi
14089eaef:     	pushq	%rdi
14089eaf0:     	pushq	%rbx
14089eaf1:     	subq	$0x68, %rsp
14089eaf5:     	leaq	0x80(%rdx), %rbp
14089eafc:     	movaps	%xmm9, 0x20(%rsp)
14089eb02:     	movaps	%xmm8, 0x30(%rsp)
14089eb08:     	movaps	%xmm7, 0x40(%rsp)
14089eb0d:     	movaps	%xmm6, 0x50(%rsp)
14089eb12:     	movq	0x168(%rbp), %rcx
14089eb19:     	movq	0x170(%rbp), %rdx
14089eb20:     	callq	0x14002e900 <.text+0x2d900>
14089eb25:     	movaps	0x50(%rsp), %xmm6
14089eb2a:     	movaps	0x40(%rsp), %xmm7
14089eb2f:     	movaps	0x30(%rsp), %xmm8
14089eb35:     	movaps	0x20(%rsp), %xmm9
14089eb3b:     	addq	$0x68, %rsp
14089eb3f:     	popq	%rbx
14089eb40:     	popq	%rdi
14089eb41:     	popq	%rsi
14089eb42:     	popq	%r12
14089eb44:     	popq	%r13
14089eb46:     	popq	%r14
14089eb48:     	popq	%r15
14089eb4a:     	popq	%rbp
14089eb4b:     	retq
14089eb4c:     	nopl	(%rax)
14089eb50:     	movq	%rdx, 0x10(%rsp)
14089eb55:     	pushq	%rbp
14089eb56:     	pushq	%r15
14089eb58:     	pushq	%r14
14089eb5a:     	pushq	%r13
14089eb5c:     	pushq	%r12
14089eb5e:     	pushq	%rsi
14089eb5f:     	pushq	%rdi
14089eb60:     	pushq	%rbx
14089eb61:     	subq	$0x68, %rsp
14089eb65:     	leaq	0x80(%rdx), %rbp
14089eb6c:     	movaps	%xmm9, 0x20(%rsp)
14089eb72:     	movaps	%xmm8, 0x30(%rsp)
14089eb78:     	movaps	%xmm7, 0x40(%rsp)
14089eb7d:     	movaps	%xmm6, 0x50(%rsp)
14089eb82:     	movq	0xc0(%rbp), %rax
14089eb89:     	testq	%rax, %rax
14089eb8c:     	je	0x14089eba8 <.text+0x89dba8>
14089eb8e:     	shlq	$0x2, %rax
14089eb92:     	leaq	(%rax,%rax,2), %rdx
14089eb96:     	movl	$0x4, %r8d
14089eb9c:     	movq	0x160(%rbp), %rcx
14089eba3:     	callq	0x140613c20 <.text+0x612c20>
14089eba8:     	movaps	0x50(%rsp), %xmm6
14089ebad:     	movaps	0x40(%rsp), %xmm7
14089ebb2:     	movaps	0x30(%rsp), %xmm8
14089ebb8:     	movaps	0x20(%rsp), %xmm9
14089ebbe:     	addq	$0x68, %rsp
14089ebc2:     	popq	%rbx
14089ebc3:     	popq	%rdi
14089ebc4:     	popq	%rsi
14089ebc5:     	popq	%r12
14089ebc7:     	popq	%r13
14089ebc9:     	popq	%r14
14089ebcb:     	popq	%r15
14089ebcd:     	popq	%rbp
14089ebce:     	retq
14089ebcf:     	nop
14089ebd0:     	movq	%rdx, 0x10(%rsp)
14089ebd5:     	pushq	%rbp
14089ebd6:     	pushq	%r15
14089ebd8:     	pushq	%r14
14089ebda:     	pushq	%r13
14089ebdc:     	pushq	%r12
14089ebde:     	pushq	%rsi
14089ebdf:     	pushq	%rdi
14089ebe0:     	pushq	%rbx
14089ebe1:     	subq	$0x68, %rsp
14089ebe5:     	leaq	0x80(%rdx), %rbp
14089ebec:     	movaps	%xmm9, 0x20(%rsp)
14089ebf2:     	movaps	%xmm8, 0x30(%rsp)
14089ebf8:     	movaps	%xmm7, 0x40(%rsp)
14089ebfd:     	movaps	%xmm6, 0x50(%rsp)
14089ec02:     	movq	0x170(%rbp), %rax
14089ec09:     	movq	0x8(%rax), %rdx
14089ec0d:     	testq	%rdx, %rdx
14089ec10:     	je	0x14089ec29 <.text+0x89dc29>
14089ec12:     	movq	0x170(%rbp), %rax
14089ec19:     	movq	0x10(%rax), %r8
14089ec1d:     	movq	0x168(%rbp), %rcx
14089ec24:     	callq	0x140613c20 <.text+0x612c20>
14089ec29:     	movaps	0x50(%rsp), %xmm6
14089ec2e:     	movaps	0x40(%rsp), %xmm7
14089ec33:     	movaps	0x30(%rsp), %xmm8
14089ec39:     	movaps	0x20(%rsp), %xmm9
14089ec3f:     	addq	$0x68, %rsp
14089ec43:     	popq	%rbx
14089ec44:     	popq	%rdi
14089ec45:     	popq	%rsi
14089ec46:     	popq	%r12
14089ec48:     	popq	%r13
14089ec4a:     	popq	%r14
14089ec4c:     	popq	%r15
14089ec4e:     	popq	%rbp
14089ec4f:     	retq
