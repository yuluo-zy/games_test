
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1400b5c30:     	pushq	%rbp
1400b5c31:     	pushq	%rsi
1400b5c32:     	pushq	%rdi
1400b5c33:     	pushq	%rbx
1400b5c34:     	subq	$0x348, %rsp            # imm = 0x348
1400b5c3b:     	leaq	0x80(%rsp), %rbp
1400b5c43:     	movq	$-0x2, 0x2c0(%rbp)
1400b5c4e:     	movq	%rdx, %rsi
1400b5c51:     	movq	%rcx, %rdi
1400b5c54:     	movq	(%rcx), %rbx
1400b5c57:     	movzbl	0x357efab(%rip), %eax   # 0x143634c09
1400b5c5e:     	movl	$0x8, %ecx
1400b5c63:     	movl	$0x8, %edx
1400b5c68:     	callq	0x140613c10 <.text+0x612c10>
1400b5c6d:     	testq	%rax, %rax
1400b5c70:     	je	0x1400b61a4 <.text+0xb51a4>
1400b5c76:     	movq	%rbx, (%rax)
1400b5c79:     	leaq	0x287ed10(%rip), %rcx   # 0x142934990
1400b5c80:     	movq	%rcx, 0x20(%rsp)
1400b5c85:     	leaq	0x160(%rbp), %rcx
1400b5c8c:     	movq	%rsi, %rdx
1400b5c8f:     	xorl	%r8d, %r8d
1400b5c92:     	movq	%rax, %r9
1400b5c95:     	callq	0x1400c4230 <.text+0xc3230>
1400b5c9a:     	movq	0x170(%rbp), %rax
1400b5ca1:     	lock
1400b5ca2:     	decq	(%rax)
1400b5ca5:     	jne	0x1400b5cb3 <.text+0xb4cb3>
1400b5ca7:     	leaq	0x170(%rbp), %rcx
1400b5cae:     	callq	0x140c41170 <.text+0xc40170>
1400b5cb3:     	movq	0x10(%rdi), %rbx
1400b5cb7:     	movzbl	0x357ef4b(%rip), %eax   # 0x143634c09
1400b5cbe:     	movl	$0x8, %ecx
1400b5cc3:     	movl	$0x8, %edx
1400b5cc8:     	callq	0x140613c10 <.text+0x612c10>
1400b5ccd:     	testq	%rax, %rax
1400b5cd0:     	je	0x1400b61a4 <.text+0xb51a4>
1400b5cd6:     	movq	%rbx, (%rax)
1400b5cd9:     	leaq	0x287ecd0(%rip), %rcx   # 0x1429349b0
1400b5ce0:     	movq	%rcx, 0x20(%rsp)
1400b5ce5:     	leaq	0x160(%rbp), %rcx
1400b5cec:     	movq	%rsi, %rdx
1400b5cef:     	xorl	%r8d, %r8d
1400b5cf2:     	movq	%rax, %r9
1400b5cf5:     	callq	0x1400c4230 <.text+0xc3230>
1400b5cfa:     	movq	0x170(%rbp), %rax
1400b5d01:     	lock
1400b5d02:     	decq	(%rax)
1400b5d05:     	jne	0x1400b5d13 <.text+0xb4d13>
1400b5d07:     	leaq	0x170(%rbp), %rcx
1400b5d0e:     	callq	0x140c41170 <.text+0xc40170>
1400b5d13:     	movq	0x8(%rdi), %rdi
1400b5d17:     	movzbl	0x357eeeb(%rip), %eax   # 0x143634c09
1400b5d1e:     	movl	$0x8, %ecx
1400b5d23:     	movl	$0x8, %edx
1400b5d28:     	callq	0x140613c10 <.text+0x612c10>
1400b5d2d:     	testq	%rax, %rax
1400b5d30:     	je	0x1400b61a4 <.text+0xb51a4>
1400b5d36:     	movq	%rax, 0x2b8(%rbp)
1400b5d3d:     	movq	%rdi, (%rax)
1400b5d40:     	leaq	0xf0(%rbp), %rcx
1400b5d47:     	callq	0x140c34750 <.text+0xc33750>
1400b5d4c:     	movq	$0x0, 0x200(%rbp)
1400b5d57:     	movq	$0x1, 0x208(%rbp)
1400b5d62:     	xorps	%xmm0, %xmm0
1400b5d65:     	movups	%xmm0, 0x210(%rbp)
1400b5d6c:     	movq	$0x1, 0x220(%rbp)
1400b5d77:     	movq	$0x0, 0x228(%rbp)
1400b5d82:     	movups	0xf0(%rbp), %xmm1
1400b5d89:     	movups	0x100(%rbp), %xmm2
1400b5d90:     	movups	0x110(%rbp), %xmm3
1400b5d97:     	movups	0x120(%rbp), %xmm4
1400b5d9e:     	movups	%xmm1, -0x50(%rbp)
1400b5da2:     	movups	%xmm2, -0x40(%rbp)
1400b5da6:     	movups	%xmm3, -0x30(%rbp)
1400b5daa:     	movups	%xmm4, -0x20(%rbp)
1400b5dae:     	movups	0x130(%rbp), %xmm1
1400b5db5:     	movups	%xmm1, -0x10(%rbp)
1400b5db9:     	movups	0x140(%rbp), %xmm1
1400b5dc0:     	movups	%xmm1, (%rbp)
1400b5dc4:     	movups	0x150(%rbp), %xmm1
1400b5dcb:     	movups	%xmm1, 0x10(%rbp)
1400b5dcf:     	movups	%xmm0, 0x278(%rbp)
1400b5dd6:     	movq	$0x0, 0x160(%rbp)
1400b5de1:     	movq	$0x0, 0x170(%rbp)
1400b5dec:     	movq	$0x0, 0x180(%rbp)
1400b5df7:     	movups	-0x58(%rbp), %xmm1
1400b5dfb:     	movups	-0x48(%rbp), %xmm2
1400b5dff:     	movups	-0x38(%rbp), %xmm3
1400b5e03:     	movups	-0x28(%rbp), %xmm4
1400b5e07:     	movups	%xmm1, 0x188(%rbp)
1400b5e0e:     	movups	%xmm2, 0x198(%rbp)
1400b5e15:     	movups	%xmm3, 0x1a8(%rbp)
1400b5e1c:     	movups	%xmm4, 0x1b8(%rbp)
1400b5e23:     	movups	-0x18(%rbp), %xmm1
1400b5e27:     	movups	%xmm1, 0x1c8(%rbp)
1400b5e2e:     	movups	-0x8(%rbp), %xmm1
1400b5e32:     	movups	%xmm1, 0x1d8(%rbp)
1400b5e39:     	movups	0x8(%rbp), %xmm1
1400b5e3d:     	movups	%xmm1, 0x1e8(%rbp)
1400b5e44:     	movq	0x18(%rbp), %rax
1400b5e48:     	movq	%rax, 0x1f8(%rbp)
1400b5e4f:     	movl	$0x2, 0x230(%rbp)
1400b5e59:     	movhps	0x287e7c8(%rip), %xmm0  # xmm0 = xmm0[0,1],mem[0,1]
                                                # 0x142934628
1400b5e60:     	movups	%xmm0, 0x238(%rbp)
1400b5e67:     	movb	$0x0, 0x248(%rbp)
1400b5e6e:     	movaps	0x287e75b(%rip), %xmm0  # 0x1429345d0
1400b5e75:     	movups	%xmm0, 0x250(%rbp)
1400b5e7c:     	movb	$0x0, 0x260(%rbp)
1400b5e83:     	movq	0x2b8(%rbp), %rax
1400b5e8a:     	movq	%rax, 0x268(%rbp)
1400b5e91:     	leaq	0x294c200(%rip), %rax   # 0x142a02098
1400b5e98:     	movq	%rax, 0x270(%rbp)
1400b5e9f:     	movq	$0x0, 0x290(%rbp)
1400b5eaa:     	movl	$0x10101, 0x2a0(%rbp)   # imm = 0x10101
1400b5eb4:     	movb	$0x2, 0x2a4(%rbp)
1400b5ebb:     	movzbl	0x357ed47(%rip), %eax   # 0x143634c09
1400b5ec2:     	movl	$0x8, %ecx
1400b5ec7:     	movl	$0x1, %edx
1400b5ecc:     	callq	0x140613c10 <.text+0x612c10>
1400b5ed1:     	testq	%rax, %rax
1400b5ed4:     	je	0x1400b61b3 <.text+0xb51b3>
1400b5eda:     	movabsq	$0x656475746974614c, %rcx # imm = 0x656475746974614C
1400b5ee4:     	movq	%rcx, (%rax)
1400b5ee7:     	movl	$0x0, 0x2b4(%rbp)
1400b5ef1:     	movl	$0x0, 0x2b1(%rbp)
1400b5efb:     	movq	0x190(%rbp), %rdi
1400b5f02:     	movl	%edi, %ecx
1400b5f04:     	andl	$-0x2, %ecx
1400b5f07:     	leaq	-0x3(%rdi), %rdx
1400b5f0b:     	xorl	%r8d, %r8d
1400b5f0e:     	cmpl	$0x4, %ecx
1400b5f11:     	cmoveq	%rdx, %r8
1400b5f15:     	testq	%r8, %r8
1400b5f18:     	movq	%rax, 0x2b8(%rbp)
1400b5f1f:     	je	0x1400b5f5a <.text+0xb4f5a>
1400b5f21:     	cmpq	$0x1, %r8
1400b5f25:     	jne	0x1400b5fc8 <.text+0xb4fc8>
1400b5f2b:     	movq	0x198(%rbp), %rdx
1400b5f32:     	testq	%rdx, %rdx
1400b5f35:     	je	0x1400b5f49 <.text+0xb4f49>
1400b5f37:     	movq	0x1a0(%rbp), %rcx
1400b5f3e:     	movl	$0x1, %r8d
1400b5f44:     	callq	0x140613c20 <.text+0x612c20>
1400b5f49:     	leaq	0x1b0(%rbp), %rcx
1400b5f50:     	callq	0x140608f80 <.text+0x607f80>
1400b5f55:     	jmp	0x1400b5fe1 <.text+0xb4fe1>
1400b5f5a:     	movq	0x1c0(%rbp), %rdx
1400b5f61:     	testq	%rdx, %rdx
1400b5f64:     	je	0x1400b5f78 <.text+0xb4f78>
1400b5f66:     	movq	0x1c8(%rbp), %rcx
1400b5f6d:     	movl	$0x1, %r8d
1400b5f73:     	callq	0x140613c20 <.text+0x612c20>
1400b5f78:     	cmpl	$0x3, %edi
1400b5f7b:     	je	0x1400b5f9b <.text+0xb4f9b>
1400b5f7d:     	cmpl	$0x2, %edi
1400b5f80:     	jb	0x1400b5f9b <.text+0xb4f9b>
1400b5f82:     	movq	0x198(%rbp), %rax
1400b5f89:     	lock
1400b5f8a:     	decq	(%rax)
1400b5f8d:     	jne	0x1400b5f9b <.text+0xb4f9b>
1400b5f8f:     	leaq	0x198(%rbp), %rcx
1400b5f96:     	callq	0x14064eb30 <.text+0x64db30>
1400b5f9b:     	movq	0x1a8(%rbp), %rax
1400b5fa2:     	cmpq	$0x6, %rax
1400b5fa6:     	je	0x1400b5fe1 <.text+0xb4fe1>
1400b5fa8:     	cmpl	$0x5, %eax
1400b5fab:     	jb	0x1400b5fe1 <.text+0xb4fe1>
1400b5fad:     	movq	0x1b0(%rbp), %rax
1400b5fb4:     	lock
1400b5fb5:     	decq	(%rax)
1400b5fb8:     	jne	0x1400b5fe1 <.text+0xb4fe1>
1400b5fba:     	leaq	0x1b0(%rbp), %rcx
1400b5fc1:     	callq	0x14064eb30 <.text+0x64db30>
1400b5fc6:     	jmp	0x1400b5fe1 <.text+0xb4fe1>
1400b5fc8:     	movq	0x198(%rbp), %rax
1400b5fcf:     	lock
1400b5fd0:     	decq	(%rax)
1400b5fd3:     	jne	0x1400b5fe1 <.text+0xb4fe1>
1400b5fd5:     	leaq	0x198(%rbp), %rcx
1400b5fdc:     	callq	0x140c7de90 <.text+0xc7ce90>
1400b5fe1:     	movq	$0x3, 0x190(%rbp)
1400b5fec:     	movups	-0x58(%rbp), %xmm0
1400b5ff0:     	movups	%xmm0, 0x198(%rbp)
1400b5ff7:     	movq	$0x6, 0x1a8(%rbp)
1400b6002:     	movups	0xf0(%rbp), %xmm0
1400b6009:     	movups	%xmm0, 0x1b0(%rbp)
1400b6010:     	movq	$0x8, 0x1c0(%rbp)
1400b601b:     	movq	0x2b8(%rbp), %rax
1400b6022:     	movq	%rax, 0x1c8(%rbp)
1400b6029:     	movq	$0x8, 0x1d0(%rbp)
1400b6034:     	movl	$0x0, 0x1d8(%rbp)
1400b603e:     	movl	$0x0, 0x1e0(%rbp)
1400b6048:     	movq	$0x0, 0x1e8(%rbp)
1400b6053:     	movb	$0x0, 0x1f0(%rbp)
1400b605a:     	movq	0x2ad(%rbp), %rax
1400b6061:     	movq	%rax, 0x1f1(%rbp)
1400b6068:     	movl	0x2b4(%rbp), %eax
1400b606e:     	movl	%eax, 0x1f8(%rbp)
1400b6074:     	leaq	-0x58(%rbp), %rdi
1400b6078:     	leaq	0x160(%rbp), %rdx
1400b607f:     	movl	$0x148, %r8d            # imm = 0x148
1400b6085:     	movq	%rdi, %rcx
1400b6088:     	callq	0x142923c00 <.text+0x2922c00>
1400b608d:     	leaq	0x160(%rbp), %rcx
1400b6094:     	movq	%rdi, %rdx
1400b6097:     	movq	%rsi, %r8
1400b609a:     	callq	0x140c77160 <.text+0xc76160>
1400b609f:     	movq	0x170(%rbp), %rax
1400b60a6:     	lock
1400b60a7:     	decq	(%rax)
1400b60aa:     	jne	0x1400b60b8 <.text+0xb50b8>
1400b60ac:     	leaq	0x170(%rbp), %rcx
1400b60b3:     	callq	0x140c41170 <.text+0xc40170>
1400b60b8:     	movzbl	0x357eb4a(%rip), %eax   # 0x143634c09
1400b60bf:     	movl	$0x15, %ecx
1400b60c4:     	movl	$0x1, %edx
1400b60c9:     	callq	0x140613c10 <.text+0x612c10>
1400b60ce:     	testq	%rax, %rax
1400b60d1:     	je	0x1400b61cb <.text+0xb51cb>
1400b60d7:     	movups	0x287f122(%rip), %xmm0  # 0x142935200
1400b60de:     	movups	%xmm0, (%rax)
1400b60e1:     	movabsq	$0x7765697620676e69, %rcx # imm = 0x7765697620676E69
1400b60eb:     	movq	%rcx, 0xd(%rax)
1400b60ef:     	movq	$0x15, 0x190(%rbp)
1400b60fa:     	movq	%rax, 0x198(%rbp)
1400b6101:     	movq	$0x15, 0x1a0(%rbp)
1400b610c:     	movl	$0x0, 0x1a8(%rbp)
1400b6116:     	movq	$0x0, 0x1b8(%rbp)
1400b6121:     	movl	$0x0, 0x1b0(%rbp)
1400b612b:     	movq	$0x3, 0x160(%rbp)
1400b6136:     	movq	$0x6, 0x178(%rbp)
1400b6141:     	movb	$0x0, 0x1c0(%rbp)
1400b6148:     	movl	$0x0, 0x1c5(%rbp)
1400b6152:     	movl	$0x0, 0x1c8(%rbp)
1400b615c:     	leaq	0x357ea2d(%rip), %rax   # 0x143634b90
1400b6163:     	movq	%rax, 0x1d0(%rbp)
1400b616a:     	movb	$0x0, 0x1d8(%rbp)
1400b6171:     	leaq	-0x58(%rbp), %rcx
1400b6175:     	leaq	0x160(%rbp), %rdx
1400b617c:     	movq	%rsi, %r8
1400b617f:     	callq	0x140c48930 <.text+0xc47930>
1400b6184:     	movq	-0x48(%rbp), %rax
1400b6188:     	lock
1400b6189:     	decq	(%rax)
1400b618c:     	jne	0x1400b6197 <.text+0xb5197>
1400b618e:     	leaq	-0x48(%rbp), %rcx
1400b6192:     	callq	0x140c41170 <.text+0xc40170>
1400b6197:     	nop
1400b6198:     	addq	$0x348, %rsp            # imm = 0x348
1400b619f:     	popq	%rbx
1400b61a0:     	popq	%rdi
1400b61a1:     	popq	%rsi
1400b61a2:     	popq	%rbp
1400b61a3:     	retq
1400b61a4:     	movl	$0x8, %ecx
1400b61a9:     	movl	$0x8, %edx
1400b61ae:     	callq	0x1428d9000 <.text+0x28d8000>
1400b61b3:     	leaq	0x287eb8e(%rip), %r8    # 0x142934d48
1400b61ba:     	movl	$0x1, %ecx
1400b61bf:     	movl	$0x8, %edx
1400b61c4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400b61c9:     	ud2
1400b61cb:     	leaq	0x287eb76(%rip), %r8    # 0x142934d48
1400b61d2:     	movl	$0x1, %ecx
1400b61d7:     	movl	$0x15, %edx
1400b61dc:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1400b61e1:     	int3
1400b61e2:     	nopw	%cs:(%rax,%rax)
1400b61f0:     	movq	%rdx, 0x10(%rsp)
1400b61f5:     	pushq	%rbp
1400b61f6:     	pushq	%rsi
1400b61f7:     	pushq	%rdi
1400b61f8:     	pushq	%rbx
1400b61f9:     	subq	$0x28, %rsp
1400b61fd:     	leaq	0x80(%rdx), %rbp
1400b6204:     	leaq	0x160(%rbp), %rcx
1400b620b:     	callq	0x1405ec430 <.text+0x5eb430>
1400b6210:     	nop
1400b6211:     	addq	$0x28, %rsp
1400b6215:     	popq	%rbx
1400b6216:     	popq	%rdi
1400b6217:     	popq	%rsi
1400b6218:     	popq	%rbp
1400b6219:     	retq
1400b621a:     	nopw	(%rax,%rax)
1400b6220:     	movq	%rdx, 0x10(%rsp)
1400b6225:     	pushq	%rbp
1400b6226:     	pushq	%rsi
1400b6227:     	pushq	%rdi
1400b6228:     	pushq	%rbx
1400b6229:     	subq	$0x28, %rsp
1400b622d:     	leaq	0x80(%rdx), %rbp
1400b6234:     	leaq	0x294be5d(%rip), %rdx   # 0x142a02098
1400b623b:     	movq	0x2b8(%rbp), %rcx
1400b6242:     	callq	0x14002e900 <.text+0x2d900>
1400b6247:     	nop
1400b6248:     	addq	$0x28, %rsp
1400b624c:     	popq	%rbx
1400b624d:     	popq	%rdi
1400b624e:     	popq	%rsi
1400b624f:     	popq	%rbp
1400b6250:     	retq
1400b6251:     	nopw	%cs:(%rax,%rax)
1400b6260:     	movq	%rdx, 0x10(%rsp)
1400b6265:     	pushq	%rbp
1400b6266:     	pushq	%rsi
1400b6267:     	pushq	%rdi
1400b6268:     	pushq	%rbx
1400b6269:     	subq	$0x28, %rsp
1400b626d:     	leaq	0x80(%rdx), %rbp
1400b6274:     	leaq	0x1a8(%rbp), %rcx
1400b627b:     	callq	0x1405fdb50 <.text+0x5fcb50>
1400b6280:     	nop
1400b6281:     	addq	$0x28, %rsp
1400b6285:     	popq	%rbx
1400b6286:     	popq	%rdi
1400b6287:     	popq	%rsi
1400b6288:     	popq	%rbp
1400b6289:     	retq
1400b628a:     	nopw	(%rax,%rax)
1400b6290:     	movq	%rdx, 0x10(%rsp)
1400b6295:     	pushq	%rbp
1400b6296:     	pushq	%rsi
1400b6297:     	pushq	%rdi
1400b6298:     	pushq	%rbx
1400b6299:     	subq	$0x28, %rsp
1400b629d:     	leaq	0x80(%rdx), %rbp
1400b62a4:     	movq	$0x3, 0x190(%rbp)
1400b62af:     	movups	-0x58(%rbp), %xmm0
1400b62b3:     	movups	%xmm0, 0x198(%rbp)
1400b62ba:     	movq	$0x6, 0x1a8(%rbp)
1400b62c5:     	movups	0xf0(%rbp), %xmm0
1400b62cc:     	movups	%xmm0, 0x1b0(%rbp)
1400b62d3:     	movq	$0x8, 0x1c0(%rbp)
1400b62de:     	movq	0x2b8(%rbp), %rax
1400b62e5:     	movq	%rax, 0x1c8(%rbp)
1400b62ec:     	movq	$0x8, 0x1d0(%rbp)
1400b62f7:     	movl	$0x0, 0x1d8(%rbp)
1400b6301:     	movl	$0x0, 0x1e0(%rbp)
1400b630b:     	movq	$0x0, 0x1e8(%rbp)
1400b6316:     	movb	$0x0, 0x1f0(%rbp)
1400b631d:     	movq	0x2ad(%rbp), %rax
1400b6324:     	movq	%rax, 0x1f1(%rbp)
1400b632b:     	movl	0x2b4(%rbp), %eax
1400b6331:     	movl	%eax, 0x1f8(%rbp)
1400b6337:     	addq	$0x28, %rsp
1400b633b:     	popq	%rbx
1400b633c:     	popq	%rdi
1400b633d:     	popq	%rsi
1400b633e:     	popq	%rbp
1400b633f:     	retq
