
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140f9dba0:     	pushq	%rbp
140f9dba1:     	pushq	%r14
140f9dba3:     	pushq	%rsi
140f9dba4:     	pushq	%rdi
140f9dba5:     	pushq	%rbx
140f9dba6:     	subq	$0x4e0, %rsp            # imm = 0x4E0
140f9dbad:     	leaq	0x80(%rsp), %rbp
140f9dbb5:     	movq	$-0x2, 0x458(%rbp)
140f9dbc0:     	movq	(%rcx), %rdi
140f9dbc3:     	cmpq	$0x0, 0x1b8(%rdi)
140f9dbcb:     	je	0x140f9dca5 <.text+0xf9cca5>
140f9dbd1:     	movl	%edx, %ebx
140f9dbd3:     	movq	%rcx, %rsi
140f9dbd6:     	leaq	0x8(%rcx), %rdx
140f9dbda:     	leaq	0x1c0(%rdi), %rcx
140f9dbe1:     	callq	0x1405b0d70 <.text+0x5afd70>
140f9dbe6:     	movq	0x1a0(%rdi), %rcx
140f9dbed:     	movq	0x1a8(%rdi), %rdx
140f9dbf4:     	movq	%rdx, %r8
140f9dbf7:     	andq	%rax, %r8
140f9dbfa:     	shrq	$0x39, %rax
140f9dbfe:     	movd	%eax, %xmm0
140f9dc02:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140f9dc06:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140f9dc0b:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
140f9dc10:     	leaq	-0xc0(%rcx), %rax
140f9dc17:     	movq	0x8(%rsi), %r9
140f9dc1b:     	xorl	%r10d, %r10d
140f9dc1e:     	pcmpeqd	%xmm1, %xmm1
140f9dc22:     	movdqu	(%rcx,%r8), %xmm2
140f9dc28:     	movdqa	%xmm2, %xmm3
140f9dc2c:     	pcmpeqb	%xmm0, %xmm3
140f9dc30:     	pmovmskb	%xmm3, %r11d
140f9dc35:     	testl	%r11d, %r11d
140f9dc38:     	je	0x140f9dc70 <.text+0xf9cc70>
140f9dc3a:     	tzcntl	%r11d, %esi
140f9dc3f:     	addq	%r8, %rsi
140f9dc42:     	andq	%rdx, %rsi
140f9dc45:     	negq	%rsi
140f9dc48:     	leaq	(%rsi,%rsi,2), %rsi
140f9dc4c:     	shlq	$0x6, %rsi
140f9dc50:     	cmpq	(%rax,%rsi), %r9
140f9dc54:     	je	0x140f9dc8e <.text+0xf9cc8e>
140f9dc56:     	leal	-0x1(%r11), %esi
140f9dc5a:     	andw	%r11w, %si
140f9dc5e:     	movl	%esi, %r11d
140f9dc61:     	jne	0x140f9dc3a <.text+0xf9cc3a>
140f9dc63:     	nopw	%cs:(%rax,%rax)
140f9dc70:     	pcmpeqb	%xmm1, %xmm2
140f9dc74:     	pmovmskb	%xmm2, %r11d
140f9dc79:     	testl	%r11d, %r11d
140f9dc7c:     	jne	0x140f9dca5 <.text+0xf9cca5>
140f9dc7e:     	addq	%r10, %r8
140f9dc81:     	addq	$0x10, %r8
140f9dc85:     	addq	$0x10, %r10
140f9dc89:     	andq	%rdx, %r8
140f9dc8c:     	jmp	0x140f9dc22 <.text+0xf9cc22>
140f9dc8e:     	leaq	(%rcx,%rsi), %r14
140f9dc92:     	movq	-0x38(%rcx,%rsi), %rcx
140f9dc97:     	testq	%rcx, %rcx
140f9dc9a:     	je	0x140f9dcbd <.text+0xf9ccbd>
140f9dc9c:     	movq	-0x30(%r14), %rax
140f9dca0:     	jmp	0x140f9dd6b <.text+0xf9cd6b>
140f9dca5:     	leaq	0x1c11394(%rip), %rcx   # 0x142baf040
140f9dcac:     	leaq	0x1c1139d(%rip), %r8    # 0x142baf050
140f9dcb3:     	movl	$0xb, %edx
140f9dcb8:     	callq	0x1428d9330 <.text+0x28d8330>
140f9dcbd:     	testb	%bl, %bl
140f9dcbf:     	jne	0x140f9ddab <.text+0xf9cdab>
140f9dcc5:     	movq	0x260(%rdi), %r9
140f9dccc:     	lock
140f9dccd:     	incq	(%r9)
140f9dcd0:     	jle	0x140f9de15 <.text+0xf9ce15>
140f9dcd6:     	leaq	-0x60(%rbp), %rsi
140f9dcda:     	movd	0x1a7915e(%rip), %xmm1  # 0x142a16e40
140f9dce2:     	movq	%rsi, %rcx
140f9dce5:     	movb	$0x1, %r8b
140f9dce8:     	callq	0x140fac1f0 <.text+0xfab1f0>
140f9dced:     	movq	%rsi, %rcx
140f9dcf0:     	movw	$0x2bc, %dx             # imm = 0x2BC
140f9dcf4:     	callq	0x140b90ea0 <.text+0xb8fea0>
140f9dcf9:     	leaq	0x1f8(%rbp), %rcx
140f9dd00:     	leaq	-0x60(%rbp), %rdx
140f9dd04:     	movl	$0x258, %r8d            # imm = 0x258
140f9dd0a:     	callq	0x142923c00 <.text+0x2922c00>
140f9dd0f:     	movl	$0xffffffa0, 0x450(%rbp) # imm = 0xFFFFFFA0
140f9dd19:     	movw	$0x2800, 0x454(%rbp)    # imm = 0x2800
140f9dd22:     	movzbl	0x2696ee0(%rip), %eax   # 0x143634c09
140f9dd29:     	movl	$0x260, %ecx            # imm = 0x260
140f9dd2e:     	movl	$0x8, %edx
140f9dd33:     	callq	0x140613c10 <.text+0x612c10>
140f9dd38:     	testq	%rax, %rax
140f9dd3b:     	je	0x140f9de06 <.text+0xf9ce06>
140f9dd41:     	leaq	0x1f8(%rbp), %rdx
140f9dd48:     	movl	$0x260, %r8d            # imm = 0x260
140f9dd4e:     	movq	%rax, %rsi
140f9dd51:     	movq	%rax, %rcx
140f9dd54:     	callq	0x142923c00 <.text+0x2922c00>
140f9dd59:     	movq	%rsi, %rcx
140f9dd5c:     	movq	%rsi, -0x38(%r14)
140f9dd60:     	leaq	0x1c11969(%rip), %rax   # 0x142baf6d0
140f9dd67:     	movq	%rax, -0x30(%r14)
140f9dd6b:     	callq	*0x28(%rax)
140f9dd6e:     	movq	%rax, %rsi
140f9dd71:     	movq	%rax, %rcx
140f9dd74:     	callq	*0x18(%rdx)
140f9dd77:     	testq	%rsi, %rsi
140f9dd7a:     	je	0x140f9ddee <.text+0xf9cdee>
140f9dd7c:     	movabsq	$0x64db2c3b564bd8b3, %r8 # imm = 0x64DB2C3B564BD8B3
140f9dd86:     	cmpq	%r8, %rax
140f9dd89:     	jne	0x140f9ddee <.text+0xf9cdee>
140f9dd8b:     	movabsq	$0x615bfcfe4597ab8f, %rcx # imm = 0x615BFCFE4597AB8F
140f9dd95:     	cmpq	%rcx, %rdx
140f9dd98:     	jne	0x140f9ddee <.text+0xf9cdee>
140f9dd9a:     	movq	%rsi, %rax
140f9dd9d:     	addq	$0x4e0, %rsp            # imm = 0x4E0
140f9dda4:     	popq	%rbx
140f9dda5:     	popq	%rdi
140f9dda6:     	popq	%rsi
140f9dda7:     	popq	%r14
140f9dda9:     	popq	%rbp
140f9ddaa:     	retq
140f9ddab:     	leaq	0x1c113c6(%rip), %rax   # 0x142baf178
140f9ddb2:     	movq	%rax, 0x1f8(%rbp)
140f9ddb9:     	movq	$0x1, 0x200(%rbp)
140f9ddc4:     	movq	$0x8, 0x208(%rbp)
140f9ddcf:     	pxor	%xmm0, %xmm0
140f9ddd3:     	movdqu	%xmm0, 0x210(%rbp)
140f9dddb:     	leaq	0x1c113a6(%rip), %rdx   # 0x142baf188
140f9dde2:     	leaq	0x1f8(%rbp), %rcx
140f9dde9:     	callq	0x1428d9390 <.text+0x28d8390>
140f9ddee:     	leaq	0x1c11273(%rip), %rcx   # 0x142baf068
140f9ddf5:     	leaq	0x1c11274(%rip), %r8    # 0x142baf070
140f9ddfc:     	movl	$0x8, %edx
140f9de01:     	callq	0x1428d9330 <.text+0x28d8330>
140f9de06:     	movl	$0x8, %ecx
140f9de0b:     	movl	$0x260, %edx            # imm = 0x260
140f9de10:     	callq	0x1428d9000 <.text+0x28d8000>
140f9de15:     	ud2
140f9de17:     	nopw	(%rax,%rax)
140f9de20:     	movq	%rdx, 0x10(%rsp)
140f9de25:     	pushq	%rbp
140f9de26:     	pushq	%r14
140f9de28:     	pushq	%rsi
140f9de29:     	pushq	%rdi
140f9de2a:     	pushq	%rbx
140f9de2b:     	subq	$0x20, %rsp
140f9de2f:     	leaq	0x80(%rdx), %rbp
140f9de36:     	leaq	0x1f8(%rbp), %rcx
140f9de3d:     	callq	0x140fa47e0 <.text+0xfa37e0>
140f9de42:     	nop
140f9de43:     	addq	$0x20, %rsp
140f9de47:     	popq	%rbx
140f9de48:     	popq	%rdi
140f9de49:     	popq	%rsi
140f9de4a:     	popq	%r14
140f9de4c:     	popq	%rbp
140f9de4d:     	retq
140f9de4e:     	nop
140f9de50:     	movq	%rdx, 0x10(%rsp)
140f9de55:     	pushq	%rbp
140f9de56:     	pushq	%r14
140f9de58:     	pushq	%rsi
140f9de59:     	pushq	%rdi
140f9de5a:     	pushq	%rbx
140f9de5b:     	subq	$0x20, %rsp
140f9de5f:     	leaq	0x80(%rdx), %rbp
140f9de66:     	leaq	-0x60(%rbp), %rcx
140f9de6a:     	callq	0x140fa47e0 <.text+0xfa37e0>
140f9de6f:     	nop
140f9de70:     	addq	$0x20, %rsp
140f9de74:     	popq	%rbx
140f9de75:     	popq	%rdi
140f9de76:     	popq	%rsi
140f9de77:     	popq	%r14
140f9de79:     	popq	%rbp
140f9de7a:     	retq
