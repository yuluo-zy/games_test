
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14228dd10:     	pushq	%rbp
14228dd11:     	pushq	%r15
14228dd13:     	pushq	%r14
14228dd15:     	pushq	%r13
14228dd17:     	pushq	%r12
14228dd19:     	pushq	%rsi
14228dd1a:     	pushq	%rdi
14228dd1b:     	pushq	%rbx
14228dd1c:     	subq	$0xf8, %rsp
14228dd23:     	leaq	0x80(%rsp), %rbp
14228dd2b:     	movq	$-0x2, 0x70(%rbp)
14228dd33:     	movq	%rdx, 0x68(%rbp)
14228dd37:     	movq	0x10(%rdx), %r15
14228dd3b:     	movq	0x8(%rcx), %r12
14228dd3f:     	movq	0x18(%rcx), %r13
14228dd43:     	leaq	-0x8(%rbp), %rdi
14228dd47:     	leaq	-0x58(%rbp), %rbx
14228dd4b:     	leaq	0x10(%rbp), %r14
14228dd4f:     	movabsq	$-0x8000000000000000, %rsi # imm = 0x8000000000000000
14228dd59:     	movq	%rcx, 0x60(%rbp)
14228dd5d:     	jmp	0x14228dda4 <.text+0x228cda4>
14228dd5f:     	nop
14228dd60:     	movq	0x68(%rbp), %r8
14228dd64:     	movq	0x8(%r8), %rax
14228dd68:     	leaq	(%rax,%rax,2), %rcx
14228dd6c:     	shlq	$0x4, %rcx
14228dd70:     	movl	$0xc5, (%r15,%rcx)
14228dd78:     	movq	%rsi, 0x8(%r15,%rcx)
14228dd7d:     	movaps	0x10(%rbp), %xmm0
14228dd81:     	movups	%xmm0, 0x10(%r15,%rcx)
14228dd87:     	movq	0x20(%rbp), %rdx
14228dd8b:     	movq	%rdx, 0x20(%r15,%rcx)
14228dd90:     	movl	$0x42800000, 0x28(%r15,%rcx) # imm = 0x42800000
14228dd99:     	incq	%rax
14228dd9c:     	movq	%rax, 0x8(%r8)
14228dda0:     	movq	0x60(%rbp), %rcx
14228dda4:     	cmpq	%r13, %r12
14228dda7:     	je	0x14228de82 <.text+0x228ce82>
14228ddad:     	movups	(%r12), %xmm0
14228ddb2:     	movups	0x10(%r12), %xmm1
14228ddb8:     	movaps	%xmm1, -0x30(%rbp)
14228ddbc:     	movaps	%xmm0, -0x40(%rbp)
14228ddc0:     	addq	$0x20, %r12
14228ddc4:     	movq	%r12, 0x8(%rcx)
14228ddc8:     	movaps	-0x40(%rbp), %xmm0
14228ddcc:     	movaps	-0x30(%rbp), %xmm1
14228ddd0:     	movaps	%xmm1, 0x50(%rbp)
14228ddd4:     	movaps	%xmm0, 0x40(%rbp)
14228ddd8:     	leaq	0x40(%rbp), %rcx
14228dddc:     	callq	0x140d76ed0 <.text+0xd75ed0>
14228dde1:     	movq	%rdx, %r8
14228dde4:     	movq	%rdi, %rcx
14228dde7:     	movq	%rax, %rdx
14228ddea:     	callq	0x140d77340 <.text+0xd76340>
14228ddef:     	movq	%rdi, -0x18(%rbp)
14228ddf3:     	leaq	-0x225232a(%rip), %rax  # 0x14003bad0 <.text+0x3aad0>
14228ddfa:     	movq	%rax, -0x10(%rbp)
14228ddfe:     	leaq	0xdacc6b(%rip), %rax    # 0x14303aa70
14228de05:     	movq	%rax, 0x10(%rbp)
14228de09:     	movq	$0x1, 0x18(%rbp)
14228de11:     	leaq	-0x18(%rbp), %rax
14228de15:     	movq	%rax, 0x20(%rbp)
14228de19:     	movq	$0x1, 0x28(%rbp)
14228de21:     	movq	$0x0, 0x30(%rbp)
14228de29:     	movq	%rbx, %rcx
14228de2c:     	movq	%r14, %rdx
14228de2f:     	callq	0x140d79f90 <.text+0xd78f90>
14228de34:     	movq	-0x8(%rbp), %rdx
14228de38:     	movq	%rdx, %rax
14228de3b:     	negq	%rax
14228de3e:     	jo	0x14228de51 <.text+0x228ce51>
14228de40:     	jae	0x14228de51 <.text+0x228ce51>
14228de42:     	movq	(%rbp), %rcx
14228de46:     	movl	$0x1, %r8d
14228de4c:     	callq	0x140613c20 <.text+0x612c20>
14228de51:     	movups	-0x58(%rbp), %xmm0
14228de55:     	movaps	%xmm0, 0x10(%rbp)
14228de59:     	movq	-0x48(%rbp), %rax
14228de5d:     	movq	%rax, 0x20(%rbp)
14228de61:     	movq	0x40(%rbp), %rdx
14228de65:     	testq	%rdx, %rdx
14228de68:     	je	0x14228dd60 <.text+0x228cd60>
14228de6e:     	movq	0x48(%rbp), %rcx
14228de72:     	movl	$0x1, %r8d
14228de78:     	callq	0x140613c20 <.text+0x612c20>
14228de7d:     	jmp	0x14228dd60 <.text+0x228cd60>
14228de82:     	movq	0x68(%rbp), %rdx
14228de86:     	movq	(%rdx), %rax
14228de89:     	movq	0x8(%rdx), %rdx
14228de8d:     	movq	%rdx, (%rax)
14228de90:     	movq	0x10(%rcx), %rdx
14228de94:     	testq	%rdx, %rdx
14228de97:     	je	0x14228deab <.text+0x228ceab>
14228de99:     	movq	(%rcx), %rcx
14228de9c:     	shlq	$0x5, %rdx
14228dea0:     	movl	$0x8, %r8d
14228dea6:     	callq	0x140613c20 <.text+0x612c20>
14228deab:     	nop
14228deac:     	addq	$0xf8, %rsp
14228deb3:     	popq	%rbx
14228deb4:     	popq	%rdi
14228deb5:     	popq	%rsi
14228deb6:     	popq	%r12
14228deb8:     	popq	%r13
14228deba:     	popq	%r14
14228debc:     	popq	%r15
14228debe:     	popq	%rbp
14228debf:     	retq
14228dec0:     	movq	%rdx, 0x10(%rsp)
14228dec5:     	pushq	%rbp
14228dec6:     	pushq	%r15
14228dec8:     	pushq	%r14
14228deca:     	pushq	%r13
14228decc:     	pushq	%r12
14228dece:     	pushq	%rsi
14228decf:     	pushq	%rdi
14228ded0:     	pushq	%rbx
14228ded1:     	subq	$0x28, %rsp
14228ded5:     	leaq	0x80(%rdx), %rbp
14228dedc:     	movq	0x40(%rbp), %rdx
14228dee0:     	testq	%rdx, %rdx
14228dee3:     	je	0x14228def4 <.text+0x228cef4>
14228dee5:     	movq	0x48(%rbp), %rcx
14228dee9:     	movl	$0x1, %r8d
14228deef:     	callq	0x140613c20 <.text+0x612c20>
14228def4:     	movq	0x68(%rbp), %rcx
14228def8:     	movq	(%rcx), %rax
14228defb:     	movq	0x8(%rcx), %rcx
14228deff:     	movq	%rcx, (%rax)
14228df02:     	movq	0x60(%rbp), %rcx
14228df06:     	callq	0x1405dd0b0 <.text+0x5dc0b0>
14228df0b:     	nop
14228df0c:     	addq	$0x28, %rsp
14228df10:     	popq	%rbx
14228df11:     	popq	%rdi
14228df12:     	popq	%rsi
14228df13:     	popq	%r12
14228df15:     	popq	%r13
14228df17:     	popq	%r14
14228df19:     	popq	%r15
14228df1b:     	popq	%rbp
14228df1c:     	retq
14228df1d:     	nopl	(%rax)
14228df20:     	movq	%rdx, 0x10(%rsp)
14228df25:     	pushq	%rbp
14228df26:     	pushq	%r15
14228df28:     	pushq	%r14
14228df2a:     	pushq	%r13
14228df2c:     	pushq	%r12
14228df2e:     	pushq	%rsi
14228df2f:     	pushq	%rdi
14228df30:     	pushq	%rbx
14228df31:     	subq	$0x28, %rsp
14228df35:     	leaq	0x80(%rdx), %rbp
14228df3c:     	movq	-0x8(%rbp), %rdx
14228df40:     	leaq	(,%rdx,2), %rax
14228df48:     	testq	%rax, %rax
14228df4b:     	je	0x14228df5c <.text+0x228cf5c>
14228df4d:     	movq	(%rbp), %rcx
14228df51:     	movl	$0x1, %r8d
14228df57:     	callq	0x140613c20 <.text+0x612c20>
14228df5c:     	nop
14228df5d:     	addq	$0x28, %rsp
14228df61:     	popq	%rbx
14228df62:     	popq	%rdi
14228df63:     	popq	%rsi
14228df64:     	popq	%r12
14228df66:     	popq	%r13
14228df68:     	popq	%r14
14228df6a:     	popq	%r15
14228df6c:     	popq	%rbp
14228df6d:     	retq
14228df6e:     	int3
14228df6f:     	int3
14228df70:     	pushq	%r15
14228df72:     	pushq	%r14
14228df74:     	pushq	%rsi
14228df75:     	pushq	%rdi
14228df76:     	pushq	%rbx
14228df77:     	subq	$0x70, %rsp
14228df7b:     	leaq	0xdacb9e(%rip), %rax    # 0x14303ab20
14228df82:     	movq	%rax, 0x30(%rsp)
14228df87:     	movq	$0x47, 0x38(%rsp)
14228df90:     	cmpb	$0x0, 0x58(%rcx)
14228df94:     	jne	0x14228e0e4 <.text+0x228d0e4>
14228df9a:     	movq	%rcx, %rsi
14228df9d:     	movq	0x18(%rcx), %rax
14228dfa1:     	cmpq	$0x0, (%rax)
14228dfa5:     	je	0x14228e08c <.text+0x228d08c>
14228dfab:     	movabsq	$-0x7343dc20d8bf724a, %rcx # imm = 0x8CBC23DF27408DB6
14228dfb5:     	cmpq	%rcx, 0x10(%rax)
14228dfb9:     	jne	0x14228e08c <.text+0x228d08c>
14228dfbf:     	movabsq	$-0x7885331d747b1d61, %rcx # imm = 0x877ACCE28B84E29F
14228dfc9:     	cmpq	%rcx, 0x8(%rax)
14228dfcd:     	jne	0x14228e08c <.text+0x228d08c>
14228dfd3:     	movq	(%rsi), %rdi
14228dfd6:     	movq	0x8(%rsi), %rcx
14228dfda:     	movq	0x48(%rsi), %r14
14228dfde:     	movq	0x10(%rcx), %rax
14228dfe2:     	movq	0x20(%rax), %rbx
14228dfe6:     	andq	$-0x8, %rbx
14228dfea:     	movq	%rbx, %r8
14228dfed:     	subq	(%rax), %r8
14228dff0:     	jb	0x14228e065 <.text+0x228d065>
14228dff2:     	cmpq	$0x8, %r8
14228dff6:     	jb	0x14228e065 <.text+0x228d065>
14228dff8:     	addq	$-0x8, %rbx
14228dffc:     	movq	%rbx, 0x20(%rax)
14228e000:     	je	0x14228e065 <.text+0x228d065>
14228e002:     	movq	%rdx, (%rbx)
14228e005:     	movq	0x10(%rdi), %r15
14228e009:     	cmpq	(%rdi), %r15
14228e00c:     	jne	0x14228e01d <.text+0x228d01d>
14228e00e:     	leaq	0xdacadb(%rip), %rdx    # 0x14303aaf0
14228e015:     	movq	%rdi, %rcx
14228e018:     	callq	0x140b81df0 <.text+0xb80df0>
14228e01d:     	movq	0x8(%rdi), %rax
14228e021:     	movq	%r14, (%rax,%r15,8)
14228e025:     	incq	%r15
14228e028:     	movq	%r15, 0x10(%rdi)
14228e02c:     	movq	0x28(%rdi), %r14
14228e030:     	cmpq	0x18(%rdi), %r14
14228e034:     	jne	0x14228e046 <.text+0x228d046>
14228e036:     	leaq	0x18(%rdi), %rcx
14228e03a:     	leaq	0xdacac7(%rip), %rdx    # 0x14303ab08
14228e041:     	callq	0x140b81df0 <.text+0xb80df0>
14228e046:     	movq	0x20(%rdi), %rax
14228e04a:     	movq	%rbx, (%rax,%r14,8)
14228e04e:     	incq	%r14
14228e051:     	movq	%r14, 0x28(%rdi)
14228e055:     	movb	$0x1, 0x58(%rsi)
14228e059:     	addq	$0x70, %rsp
14228e05d:     	popq	%rbx
14228e05e:     	popq	%rdi
14228e05f:     	popq	%rsi
14228e060:     	popq	%r14
14228e062:     	popq	%r15
14228e064:     	retq
14228e065:     	movq	%rdx, %rbx
14228e068:     	movl	$0x8, %edx
14228e06d:     	movl	$0x8, %r8d
14228e073:     	callq	0x1428d7780 <.text+0x28d6780>
14228e078:     	movq	%rbx, %rdx
14228e07b:     	movq	%rax, %rbx
14228e07e:     	testq	%rax, %rax
14228e081:     	jne	0x14228e002 <.text+0x228d002>
14228e087:     	callq	0x1428d79d0 <.text+0x28d69d0>
14228e08c:     	leaq	0x30(%rsp), %rax
14228e091:     	movq	%rax, 0x20(%rsp)
14228e096:     	leaq	-0x15432bd(%rip), %rax  # 0x140d4ade0 <.text+0xd49de0>
14228e09d:     	movq	%rax, 0x28(%rsp)
14228e0a2:     	leaq	0xdacaff(%rip), %rax    # 0x14303aba8
14228e0a9:     	movq	%rax, 0x40(%rsp)
14228e0ae:     	movq	$0x2, 0x48(%rsp)
14228e0b7:     	movq	$0x0, 0x60(%rsp)
14228e0c0:     	leaq	0x20(%rsp), %rax
14228e0c5:     	movq	%rax, 0x50(%rsp)
14228e0ca:     	movq	$0x1, 0x58(%rsp)
14228e0d3:     	leaq	0xdacaee(%rip), %rdx    # 0x14303abc8
14228e0da:     	leaq	0x40(%rsp), %rcx
14228e0df:     	callq	0x1428d9390 <.text+0x28d8390>
14228e0e4:     	leaq	0x30(%rsp), %rax
14228e0e9:     	movq	%rax, 0x20(%rsp)
14228e0ee:     	leaq	-0x1543315(%rip), %rax  # 0x140d4ade0 <.text+0xd49de0>
14228e0f5:     	movq	%rax, 0x28(%rsp)
14228e0fa:     	leaq	0xdacb0f(%rip), %rax    # 0x14303ac10
14228e101:     	movq	%rax, 0x40(%rsp)
14228e106:     	movq	$0x2, 0x48(%rsp)
14228e10f:     	movq	$0x0, 0x60(%rsp)
14228e118:     	leaq	0x20(%rsp), %rax
14228e11d:     	movq	%rax, 0x50(%rsp)
14228e122:     	movq	$0x1, 0x58(%rsp)
14228e12b:     	leaq	0xdacafe(%rip), %rdx    # 0x14303ac30
14228e132:     	leaq	0x40(%rsp), %rcx
14228e137:     	callq	0x1428d9390 <.text+0x28d8390>
14228e13c:     	int3
14228e13d:     	int3
14228e13e:     	int3
14228e13f:     	int3
14228e140:     	pushq	%rbp
14228e141:     	pushq	%r15
14228e143:     	pushq	%r14
14228e145:     	pushq	%r13
14228e147:     	pushq	%r12
14228e149:     	pushq	%rsi
14228e14a:     	pushq	%rdi
14228e14b:     	pushq	%rbx
14228e14c:     	subq	$0x48, %rsp
