
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14076d9f0:     	pushq	%rbp
14076d9f1:     	pushq	%r15
14076d9f3:     	pushq	%r14
14076d9f5:     	pushq	%r13
14076d9f7:     	pushq	%r12
14076d9f9:     	pushq	%rsi
14076d9fa:     	pushq	%rdi
14076d9fb:     	pushq	%rbx
14076d9fc:     	subq	$0x388, %rsp            # imm = 0x388
14076da03:     	leaq	0x80(%rsp), %rbp
14076da0b:     	movaps	%xmm15, 0x2f0(%rbp)
14076da13:     	movaps	%xmm14, 0x2e0(%rbp)
14076da1b:     	movaps	%xmm13, 0x2d0(%rbp)
14076da23:     	movaps	%xmm12, 0x2c0(%rbp)
14076da2b:     	movaps	%xmm11, 0x2b0(%rbp)
14076da33:     	movaps	%xmm10, 0x2a0(%rbp)
14076da3b:     	movaps	%xmm9, 0x290(%rbp)
14076da43:     	movaps	%xmm8, 0x280(%rbp)
14076da4b:     	movaps	%xmm7, 0x270(%rbp)
14076da52:     	movdqa	%xmm6, 0x260(%rbp)
14076da5a:     	movq	$-0x2, 0x258(%rbp)
14076da65:     	movq	%r9, 0x248(%rbp)
14076da6c:     	movq	%r8, %r12
14076da6f:     	movq	%rdx, %rbx
14076da72:     	movq	%rcx, %rdi
14076da75:     	callq	0x140c16a00 <.text+0xc15a00>
14076da7a:     	movb	%al, 0x256(%rbp)
14076da80:     	testb	%al, %al
14076da82:     	je	0x14076dc9e <.text+0x76cc9e>
14076da88:     	movl	$0x3d, %esi
14076da8d:     	movl	$0x3d, %ecx
14076da92:     	callq	0x14076d5c0 <.text+0x76c5c0>
14076da97:     	leaq	0x2310dfa(%rip), %r14   # 0x142a7e898
14076da9e:     	cmpq	$0x1, %rax
14076daa2:     	jne	0x14076db48 <.text+0x76cb48>
14076daa8:     	movq	%rdx, %r9
14076daab:     	testq	%rdx, %rdx
14076daae:     	je	0x14076dae6 <.text+0x76cae6>
14076dab0:     	cmpq	$0x3d, %r9
14076dab4:     	jae	0x14076dae4 <.text+0x76cae4>
14076dab6:     	leaq	0x2310ddb(%rip), %rax   # 0x142a7e898
14076dabd:     	cmpb	$-0x41, (%r9,%rax)
14076dac2:     	jg	0x14076dae6 <.text+0x76cae6>
14076dac4:     	leaq	0x2310e85(%rip), %rax   # 0x142a7e950
14076dacb:     	movq	%rax, 0x20(%rsp)
14076dad0:     	leaq	0x2310dc1(%rip), %rcx   # 0x142a7e898
14076dad7:     	movl	$0x3d, %edx
14076dadc:     	xorl	%r8d, %r8d
14076dadf:     	callq	0x1428d9940 <.text+0x28d8940>
14076dae4:     	jne	0x14076dac4 <.text+0x76cac4>
14076dae6:     	movq	%r9, %rcx
14076dae9:     	callq	0x14076d5c0 <.text+0x76c5c0>
14076daee:     	cmpq	$0x1, %rax
14076daf2:     	jne	0x14076db48 <.text+0x76cb48>
14076daf4:     	movq	%rdx, %r8
14076daf7:     	addq	$0x2, %r8
14076dafb:     	je	0x14076db36 <.text+0x76cb36>
14076dafd:     	cmpq	$0x3d, %r8
14076db01:     	jae	0x14076db34 <.text+0x76cb34>
14076db03:     	leaq	0x2310d8e(%rip), %rax   # 0x142a7e898
14076db0a:     	cmpb	$-0x41, (%r8,%rax)
14076db0f:     	jg	0x14076db36 <.text+0x76cb36>
14076db11:     	leaq	0x2310e50(%rip), %rax   # 0x142a7e968
14076db18:     	movq	%rax, 0x20(%rsp)
14076db1d:     	leaq	0x2310d74(%rip), %rcx   # 0x142a7e898
14076db24:     	movl	$0x3d, %edx
14076db29:     	movl	$0x3d, %r9d
14076db2f:     	callq	0x1428d9940 <.text+0x28d8940>
14076db34:     	jne	0x14076db11 <.text+0x76cb11>
14076db36:     	movl	$0x3b, %esi
14076db3b:     	subq	%rdx, %rsi
14076db3e:     	leaq	0x2310d53(%rip), %r14   # 0x142a7e898
14076db45:     	addq	%r8, %r14
14076db48:     	leaq	0x2310e85(%rip), %r8    # 0x142a7e9d4
14076db4f:     	leaq	0x2310e42(%rip), %r13   # 0x142a7e998
14076db56:     	nopw	%cs:(%rax,%rax)
14076db60:     	movsbl	-0x1(%r8), %eax
14076db65:     	testl	%eax, %eax
14076db67:     	js	0x14076db80 <.text+0x76cb80>
14076db69:     	decq	%r8
14076db6c:     	cmpl	$0x5c, %eax
14076db6f:     	jne	0x14076dbd4 <.text+0x76cbd4>
14076db71:     	jmp	0x14076dbe6 <.text+0x76cbe6>
14076db73:     	nopw	%cs:(%rax,%rax)
14076db80:     	movzbl	-0x2(%r8), %ecx
14076db85:     	cmpb	$-0x40, %cl
14076db88:     	jge	0x14076dbad <.text+0x76cbad>
14076db8a:     	movzbl	-0x3(%r8), %edx
14076db8f:     	cmpb	$-0x40, %dl
14076db92:     	jge	0x14076dbb6 <.text+0x76cbb6>
14076db94:     	movzbl	-0x4(%r8), %r9d
14076db99:     	addq	$-0x4, %r8
14076db9d:     	andl	$0x7, %r9d
14076dba1:     	shll	$0x6, %r9d
14076dba5:     	andl	$0x3f, %edx
14076dba8:     	orl	%r9d, %edx
14076dbab:     	jmp	0x14076dbbd <.text+0x76cbbd>
14076dbad:     	addq	$-0x2, %r8
14076dbb1:     	andl	$0x1f, %ecx
14076dbb4:     	jmp	0x14076dbc5 <.text+0x76cbc5>
14076dbb6:     	addq	$-0x3, %r8
14076dbba:     	andl	$0xf, %edx
14076dbbd:     	shll	$0x6, %edx
14076dbc0:     	andl	$0x3f, %ecx
14076dbc3:     	orl	%edx, %ecx
14076dbc5:     	shll	$0x6, %ecx
14076dbc8:     	andb	$0x3f, %al
14076dbca:     	movzbl	%al, %eax
14076dbcd:     	orl	%ecx, %eax
14076dbcf:     	cmpl	$0x5c, %eax
14076dbd2:     	je	0x14076dbe6 <.text+0x76cbe6>
14076dbd4:     	cmpl	$0x2f, %eax
14076dbd7:     	je	0x14076dbe6 <.text+0x76cbe6>
14076dbd9:     	cmpq	%r13, %r8
14076dbdc:     	jne	0x14076db60 <.text+0x76cb60>
14076dbde:     	movl	$0x3c, %r15d
14076dbe4:     	jmp	0x14076dc33 <.text+0x76cc33>
14076dbe6:     	leaq	0x2310dab(%rip), %r13   # 0x142a7e998
14076dbed:     	subq	%r13, %r8
14076dbf0:     	incq	%r8
14076dbf3:     	je	0x14076dc27 <.text+0x76cc27>
14076dbf5:     	cmpq	$0x3c, %r8
14076dbf9:     	jae	0x14076dc25 <.text+0x76cc25>
14076dbfb:     	cmpb	$-0x41, (%r8,%r13)
14076dc00:     	jg	0x14076dc27 <.text+0x76cc27>
14076dc02:     	leaq	0x2310d27(%rip), %rax   # 0x142a7e930
14076dc09:     	movq	%rax, 0x20(%rsp)
14076dc0e:     	leaq	0x2310d83(%rip), %rcx   # 0x142a7e998
14076dc15:     	movl	$0x3c, %edx
14076dc1a:     	movl	$0x3c, %r9d
14076dc20:     	callq	0x1428d9940 <.text+0x28d8940>
14076dc25:     	jne	0x14076dc02 <.text+0x76cc02>
14076dc27:     	movl	$0x3c, %r15d
14076dc2d:     	subq	%r8, %r15
14076dc30:     	addq	%r8, %r13
14076dc33:     	callq	0x140c18130 <.text+0xc17130>
14076dc38:     	movq	%rax, %rcx
14076dc3b:     	movq	(%rax), %rax
14076dc3e:     	cmpq	$0x1, %rax
14076dc42:     	jne	0x14076f5a7 <.text+0x76e5a7>
14076dc48:     	addq	$0x8, %rcx
14076dc4c:     	cmpq	$0x0, (%rcx)
14076dc50:     	jne	0x14076f6ae <.text+0x76e6ae>
14076dc56:     	movq	$-0x1, (%rcx)
14076dc5d:     	movq	%rcx, 0x240(%rbp)
14076dc64:     	addq	$0x8, %rcx
14076dc68:     	movq	%r15, 0x20(%rsp)
14076dc6d:     	movq	$0x0, 0x30(%rsp)
14076dc76:     	movq	$0x1, 0x28(%rsp)
14076dc7f:     	movq	%r14, %rdx
14076dc82:     	movq	%rsi, %r8
14076dc85:     	movq	%r13, %r9
14076dc88:     	callq	0x140c16d40 <.text+0xc15d40>
14076dc8d:     	movq	%rax, 0x170(%rbp)
14076dc94:     	movq	0x240(%rbp), %rax
14076dc9b:     	incq	(%rax)
14076dc9e:     	movq	0x370(%rbp), %rax
14076dca5:     	movq	(%rax), %rcx
14076dca8:     	leaq	0x24acb21(%rip), %rdx   # 0x142c1a7d0
14076dcaf:     	callq	0x141209c40 <.text+0x1208c40>
14076dcb4:     	movq	%rax, %rsi
14076dcb7:     	testq	%rax, %rax
14076dcba:     	je	0x14076f852 <.text+0x76e852>
14076dcc0:     	leaq	0x24acca1(%rip), %rax   # 0x142c1a968
14076dcc7:     	movq	%rax, (%rbp)
14076dccb:     	movq	$0x3d, 0x8(%rbp)
14076dcd3:     	movq	%rsi, %r14
14076dcd6:     	addq	$0x10, %r14
14076dcda:     	cmpq	$0x3d, 0x18(%rsi)
14076dcdf:     	jne	0x14076f5ec <.text+0x76e5ec>
14076dce5:     	movq	(%r14), %rdx
14076dce8:     	leaq	0x24acc79(%rip), %rcx   # 0x142c1a968
14076dcef:     	movl	$0x3d, %r8d
14076dcf5:     	callq	0x142923bf0 <.text+0x2922bf0>
14076dcfa:     	testl	%eax, %eax
14076dcfc:     	jne	0x14076f5ec <.text+0x76e5ec>
14076dd02:     	movq	(%rsi), %rcx
14076dd05:     	movq	0x8(%rsi), %rax
14076dd09:     	callq	*0x18(%rax)
14076dd0c:     	movq	%rax, %r14
14076dd0f:     	testq	%rax, %rax
14076dd12:     	je	0x14076f852 <.text+0x76e852>
14076dd18:     	movb	$0x0, 0x255(%rbp)
14076dd1f:     	movq	(%rdi), %rax
14076dd22:     	movq	0x8(%rdi), %rdx
14076dd26:     	movq	(%rax), %r10
14076dd29:     	xorl	%r9d, %r9d
14076dd2c:     	movq	%r10, %r11
14076dd2f:     	subq	0x18(%rdx), %r11
14076dd33:     	cmovbq	%r9, %r11
14076dd37:     	subq	0x38(%rdx), %r10
14076dd3b:     	cmovbq	%r9, %r10
14076dd3f:     	movq	%r11, %rcx
14076dd42:     	shlq	$0x4, %rcx
14076dd46:     	addq	0x8(%rdx), %rcx
14076dd4a:     	movq	0x10(%rdx), %r8
14076dd4e:     	subq	%r11, %r8
14076dd51:     	cmovbq	%r9, %r8
14076dd55:     	movl	$0x8, %esi
14076dd5a:     	cmovbq	%rsi, %rcx
14076dd5e:     	movq	%r10, %r11
14076dd61:     	shlq	$0x4, %r11
14076dd65:     	addq	0x28(%rdx), %r11
14076dd69:     	movq	0x30(%rdx), %rdi
14076dd6d:     	subq	%r10, %rdi
14076dd70:     	cmovbq	%r9, %rdi
14076dd74:     	cmovbq	%rsi, %r11
14076dd78:     	leaq	(%rdi,%r8), %r9
14076dd7c:     	movq	0x40(%rdx), %rdx
14076dd80:     	subq	%r9, %rdx
14076dd83:     	movq	%rdx, (%rax)
14076dd86:     	shlq	$0x4, %r8
14076dd8a:     	addq	%rcx, %r8
14076dd8d:     	shlq	$0x4, %rdi
14076dd91:     	addq	%r11, %rdi
14076dd94:     	movq	%rax, 0x180(%rbp)
14076dd9b:     	movq	%rcx, 0x188(%rbp)
14076dda2:     	movq	%r8, 0x190(%rbp)
14076dda9:     	movq	%r11, 0x198(%rbp)
14076ddb0:     	movq	%rdi, 0x1a0(%rbp)
14076ddb7:     	movq	%r9, 0x1a8(%rbp)
14076ddbe:     	leaq	0x2310abb(%rip), %r8    # 0x142a7e880
14076ddc5:     	leaq	0x158(%rbp), %rcx
14076ddcc:     	leaq	0x180(%rbp), %rdx
14076ddd3:     	callq	0x1412036d0 <.text+0x12026d0>
14076ddd8:     	movq	%r12, 0x240(%rbp)
14076dddf:     	movq	%r14, 0xa8(%rbp)
14076dde6:     	movq	(%rbx), %r13
14076dde9:     	movq	0x8(%rbx), %rdx
14076dded:     	movq	(%r13), %r9
14076ddf1:     	xorl	%r8d, %r8d
14076ddf4:     	movq	%r9, %rcx
14076ddf7:     	subq	0x18(%rdx), %rcx
14076ddfb:     	cmovbq	%r8, %rcx
14076ddff:     	subq	0x38(%rdx), %r9
14076de03:     	cmovbq	%r8, %r9
14076de07:     	movq	%rcx, %rax
14076de0a:     	shlq	$0x6, %rax
14076de0e:     	addq	0x8(%rdx), %rax
14076de12:     	movq	0x10(%rdx), %r15
14076de16:     	subq	%rcx, %r15
14076de19:     	cmovbq	%r8, %r15
14076de1d:     	cmovbq	%rsi, %rax
14076de21:     	movq	%r9, %rcx
14076de24:     	shlq	$0x6, %rcx
14076de28:     	addq	0x28(%rdx), %rcx
14076de2c:     	movq	0x30(%rdx), %r12
14076de30:     	subq	%r9, %r12
14076de33:     	cmovbq	%r8, %r12
14076de37:     	cmovbq	%rsi, %rcx
14076de3b:     	movq	0x40(%rdx), %rdx
14076de3f:     	subq	%r12, %rdx
14076de42:     	subq	%r15, %rdx
14076de45:     	movq	%rdx, (%r13)
14076de49:     	shlq	$0x6, %r15
14076de4d:     	addq	%rax, %r15
14076de50:     	shlq	$0x6, %r12
14076de54:     	addq	%rcx, %r12
14076de57:     	jmp	0x14076dea8 <.text+0x76cea8>
14076de59:     	testq	%rdi, %rdi
14076de5c:     	je	0x14076df1a <.text+0x76cf1a>
14076de62:     	movq	%r12, %r8
14076de65:     	subq	%rdi, %r8
14076de68:     	shrq	$0x6, %r8
14076de6c:     	incq	%r8
14076de6f:     	movq	$0x8, 0x20(%rsp)
14076de78:     	movl	$0x8, %r9d
14076de7e:     	leaq	0x158(%rbp), %rcx
14076de85:     	movq	%rbx, %rdx
14076de88:     	callq	0x1428a7de0 <.text+0x28a6de0>
14076de8d:     	movq	0x160(%rbp), %rax
14076de94:     	movq	%rsi, (%rax,%rbx,8)
14076de98:     	incq	%rbx
14076de9b:     	movq	%rbx, 0x168(%rbp)
14076dea2:     	movq	%rdi, %rcx
14076dea5:     	movq	%r14, %rax
14076dea8:     	testq	%rax, %rax
14076deab:     	je	0x14076dec0 <.text+0x76cec0>
14076dead:     	cmpq	%r15, %rax
14076deb0:     	je	0x14076dec0 <.text+0x76cec0>
14076deb2:     	leaq	0x40(%rax), %r14
14076deb6:     	movq	%rcx, %rdi
14076deb9:     	jmp	0x14076ded4 <.text+0x76ced4>
14076debb:     	nopl	(%rax,%rax)
14076dec0:     	testq	%rcx, %rcx
14076dec3:     	je	0x14076df22 <.text+0x76cf22>
14076dec5:     	cmpq	%r12, %rcx
14076dec8:     	je	0x14076df22 <.text+0x76cf22>
14076deca:     	leaq	0x40(%rcx), %rdi
14076dece:     	xorl	%r14d, %r14d
14076ded1:     	movq	%rcx, %rax
14076ded4:     	incq	(%r13)
14076ded8:     	movq	0x8(%rax), %rsi
14076dedc:     	movq	0x168(%rbp), %rbx
14076dee3:     	cmpq	0x158(%rbp), %rbx
14076deea:     	jne	0x14076de8d <.text+0x76ce8d>
14076deec:     	testq	%r14, %r14
14076deef:     	je	0x14076de59 <.text+0x76ce59>
14076def5:     	movq	%r15, %r8
14076def8:     	subq	%r14, %r8
14076defb:     	shrq	$0x6, %r8
14076deff:     	testq	%rdi, %rdi
14076df02:     	je	0x14076de6c <.text+0x76ce6c>
14076df08:     	movq	%r12, %rax
14076df0b:     	subq	%rdi, %rax
14076df0e:     	shrq	$0x6, %rax
14076df12:     	addq	%rax, %r8
14076df15:     	jmp	0x14076de6c <.text+0x76ce6c>
14076df1a:     	xorl	%r8d, %r8d
14076df1d:     	jmp	0x14076de6c <.text+0x76ce6c>
14076df22:     	movq	0x160(%rbp), %r12
14076df29:     	movq	0x168(%rbp), %rsi
14076df30:     	cmpq	$0x1, %rsi
14076df34:     	ja	0x14076f61c <.text+0x76e61c>
14076df3a:     	leaq	(%r12,%rsi,8), %r13
14076df3e:     	movq	0x240(%rbp), %rdx
14076df45:     	movq	(%rdx), %r15
14076df48:     	movq	0x10(%rdx), %rax
14076df4c:     	movq	%rax, 0x58(%rbp)
14076df50:     	leaq	0x20(%r15), %rax
14076df54:     	movq	%rax, 0xa0(%rbp)
14076df5b:     	movq	0x248(%rbp), %rcx
14076df62:     	movl	0x1c(%rcx), %eax
14076df65:     	movl	%eax, 0x104(%rbp)
14076df6b:     	movq	(%rcx), %rax
14076df6e:     	movq	0x10(%rcx), %rcx
14076df72:     	movq	%rcx, 0x68(%rbp)
14076df76:     	movq	%rax, 0x60(%rbp)
14076df7a:     	leaq	0x20(%rax), %rax
14076df7e:     	movq	%rax, 0x70(%rbp)
14076df82:     	movl	0x1c(%rdx), %eax
14076df85:     	movl	%eax, 0x100(%rbp)
14076df8b:     	movq	0x380(%rbp), %rcx
14076df92:     	movl	0x1c(%rcx), %eax
14076df95:     	movl	%eax, 0x108(%rbp)
14076df9b:     	movq	(%rcx), %rax
14076df9e:     	movq	%rax, 0x78(%rbp)
14076dfa2:     	movq	0x10(%rcx), %rax
14076dfa6:     	movq	%rax, 0x80(%rbp)
14076dfad:     	pcmpeqd	%xmm6, %xmm6
14076dfb1:     	movss	0x22711b2(%rip), %xmm11 # 0x1429df16c
14076dfba:     	xorps	%xmm12, %xmm12
14076dfbe:     	xorps	%xmm13, %xmm13
14076dfc2:     	movaps	0x2310796(%rip), %xmm14 # 0x142a7e760
14076dfca:     	movss	0x23107a1(%rip), %xmm15 # 0x142a7e774
14076dfd3:     	cmpq	%r13, %r12
14076dfd6:     	je	0x14076f440 <.text+0x76e440>
14076dfdc:     	movq	(%r12), %rsi
14076dfe0:     	movq	%rsi, 0x228(%rbp)
14076dfe7:     	cmpq	$0x0, 0x18(%r15)
14076dfec:     	je	0x14076f3e7 <.text+0x76e3e7>
14076dff2:     	addq	$0x8, %r12
14076dff6:     	movq	0xa0(%rbp), %rcx
14076dffd:     	leaq	0x228(%rbp), %rdx
14076e004:     	callq	0x1405b0d70 <.text+0x5afd70>
14076e009:     	movq	(%r15), %rbx
14076e00c:     	movq	0x8(%r15), %rcx
14076e010:     	movq	%rcx, %rdx
14076e013:     	andq	%rax, %rdx
14076e016:     	shrq	$0x39, %rax
14076e01a:     	movd	%eax, %xmm0
14076e01e:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14076e022:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14076e027:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14076e02c:     	leaq	-0x2a8(%rbx), %rax
14076e033:     	xorl	%r8d, %r8d
14076e036:     	movdqu	(%rbx,%rdx), %xmm1
14076e03b:     	movdqa	%xmm1, %xmm2
14076e03f:     	pcmpeqb	%xmm0, %xmm2
14076e043:     	pmovmskb	%xmm2, %r9d
14076e048:     	testl	%r9d, %r9d
14076e04b:     	je	0x14076e080 <.text+0x76d080>
14076e04d:     	tzcntl	%r9d, %r10d
14076e052:     	addq	%rdx, %r10
14076e055:     	andq	%rcx, %r10
14076e058:     	negq	%r10
14076e05b:     	imulq	$0x2a8, %r10, %rdi      # imm = 0x2A8
14076e062:     	cmpq	(%rax,%rdi), %rsi
14076e066:     	je	0x14076e0b0 <.text+0x76d0b0>
14076e068:     	leal	-0x1(%r9), %r10d
14076e06c:     	andw	%r9w, %r10w
14076e070:     	movl	%r10d, %r9d
14076e073:     	jne	0x14076e04d <.text+0x76d04d>
14076e075:     	nopw	%cs:(%rax,%rax)
14076e080:     	pcmpeqb	%xmm6, %xmm1
14076e084:     	pmovmskb	%xmm1, %r9d
14076e089:     	testl	%r9d, %r9d
14076e08c:     	jne	0x14076f3e7 <.text+0x76e3e7>
14076e092:     	addq	%r8, %rdx
14076e095:     	addq	$0x10, %rdx
14076e099:     	addq	$0x10, %r8
14076e09d:     	andq	%rcx, %rdx
14076e0a0:     	jmp	0x14076e036 <.text+0x76d036>
14076e0a2:     	nopw	%cs:(%rax,%rax)
14076e0b0:     	movq	-0x170(%rbx,%rdi), %r14
14076e0b8:     	testq	%r14, %r14
14076e0bb:     	je	0x14076f863 <.text+0x76e863>
14076e0c1:     	leaq	0x18(%r14), %rcx
14076e0c5:     	xorl	%eax, %eax
14076e0c7:     	movb	$0x1, %dl
14076e0c9:     	lock
14076e0ca:     	cmpxchgb	%dl, 0x18(%r14)
14076e0cf:     	jne	0x14076f3d1 <.text+0x76e3d1>
14076e0d5:     	callq	0x140c81c70 <.text+0xc80c70>
14076e0da:     	movq	0x10(%r14), %rax
14076e0de:     	testq	%rax, %rax
14076e0e1:     	sete	%cl
14076e0e4:     	shlq	$0x2, %rax
14076e0e8:     	addq	0x8(%r14), %rax
14076e0ec:     	addq	$-0x4, %rax
14076e0f0:     	sete	%dl
14076e0f3:     	orb	%cl, %dl
14076e0f5:     	je	0x14076e110 <.text+0x76d110>
14076e0f7:     	movq	0xa8(%rbp), %r8
14076e0fe:     	movq	0x28(%r8), %rax
14076e102:     	testq	%rax, %rax
14076e105:     	je	0x14076fa35 <.text+0x76ea35>
14076e10b:     	leal	-0x1(%rax), %ecx
14076e10e:     	jmp	0x14076e11d <.text+0x76d11d>
14076e110:     	movl	(%rax), %ecx
14076e112:     	movq	0xa8(%rbp), %r8
14076e119:     	movq	0x28(%r8), %rax
14076e11d:     	leaq	(%rbx,%rdi), %r9
14076e121:     	movq	0x20(%r8), %rdx
14076e125:     	movups	0x30(%r8), %xmm0
14076e12a:     	movups	0x40(%r8), %xmm1
14076e12f:     	movups	0x50(%r8), %xmm2
14076e134:     	movl	%ecx, 0x50(%rbp)
14076e137:     	movq	%rdx, (%rbp)
14076e13b:     	movq	%rax, 0x8(%rbp)
14076e13f:     	movups	%xmm0, 0x10(%rbp)
14076e143:     	movq	%r14, 0x20(%rbp)
14076e147:     	movups	%xmm1, 0x28(%rbp)
14076e14b:     	movq	%r8, 0x38(%rbp)
14076e14f:     	movups	%xmm2, 0x40(%rbp)
14076e153:     	movq	%rbp, %rax
14076e156:     	movq	%rax, -0x10(%rbp)
14076e15a:     	leaq	0x255(%rbp), %rax
14076e161:     	movq	%rax, -0x8(%rbp)
14076e165:     	movq	$0x0, 0x140(%rbp)
14076e170:     	movq	$0x8, 0x148(%rbp)
14076e17b:     	movq	$0x0, 0x150(%rbp)
14076e186:     	xorps	%xmm0, %xmm0
14076e189:     	movss	%xmm0, 0x23c(%rbp)
14076e191:     	movq	%r9, 0x240(%rbp)
14076e198:     	cmpb	$0x0, -0xa(%r9)
14076e19d:     	je	0x14076f0d0 <.text+0x76e0d0>
14076e1a3:     	movl	0x104(%rbp), %eax
14076e1a9:     	movq	0x68(%rbp), %rcx
14076e1ad:     	movl	%eax, (%rcx)
14076e1af:     	movq	0x60(%rbp), %rsi
14076e1b3:     	cmpq	$0x0, 0x18(%rsi)
14076e1b8:     	je	0x14076f428 <.text+0x76e428>
14076e1be:     	leaq	(%rbx,%rdi), %rax
14076e1c2:     	addq	$-0x2a0, %rax           # imm = 0xFD60
14076e1c8:     	movq	%rax, 0x1e8(%rbp)
14076e1cf:     	movq	0x70(%rbp), %rcx
14076e1d3:     	leaq	0x228(%rbp), %rdx
14076e1da:     	callq	0x1405b0d70 <.text+0x5afd70>
14076e1df:     	movq	(%rsi), %rcx
14076e1e2:     	movq	0x8(%rsi), %rdx
14076e1e6:     	movq	%rdx, %r8
14076e1e9:     	andq	%rax, %r8
14076e1ec:     	shrq	$0x39, %rax
14076e1f0:     	movd	%eax, %xmm0
14076e1f4:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14076e1f8:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14076e1fd:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14076e202:     	leaq	-0x20(%rcx), %rax
14076e206:     	movq	0x228(%rbp), %r9
14076e20d:     	xorl	%r10d, %r10d
14076e210:     	movdqu	(%rcx,%r8), %xmm1
14076e216:     	movdqa	%xmm1, %xmm2
14076e21a:     	pcmpeqb	%xmm0, %xmm2
14076e21e:     	pmovmskb	%xmm2, %r11d
14076e223:     	testl	%r11d, %r11d
14076e226:     	je	0x14076e252 <.text+0x76d252>
14076e228:     	tzcntl	%r11d, %esi
14076e22d:     	addq	%r8, %rsi
14076e230:     	andq	%rdx, %rsi
14076e233:     	movq	%rsi, %rdi
14076e236:     	shlq	$0x5, %rdi
14076e23a:     	movq	%rax, %rbx
14076e23d:     	subq	%rdi, %rbx
14076e240:     	cmpq	(%rbx), %r9
14076e243:     	je	0x14076e274 <.text+0x76d274>
14076e245:     	leal	-0x1(%r11), %esi
14076e249:     	andw	%r11w, %si
14076e24d:     	movl	%esi, %r11d
14076e250:     	jne	0x14076e228 <.text+0x76d228>
14076e252:     	pcmpeqb	%xmm6, %xmm1
14076e256:     	pmovmskb	%xmm1, %r11d
14076e25b:     	testl	%r11d, %r11d
14076e25e:     	jne	0x14076f428 <.text+0x76e428>
14076e264:     	addq	%r10, %r8
14076e267:     	addq	$0x10, %r8
14076e26b:     	addq	$0x10, %r10
14076e26f:     	andq	%rdx, %r8
14076e272:     	jmp	0x14076e210 <.text+0x76d210>
14076e274:     	movq	%r15, 0x118(%rbp)
14076e27b:     	negq	%rsi
14076e27e:     	shlq	$0x5, %rsi
14076e282:     	movq	-0x10(%rcx,%rsi), %rdx
14076e287:     	movq	-0x8(%rcx,%rsi), %rax
14076e28c:     	movq	%rax, %rcx
14076e28f:     	shlq	$0x7, %rcx
14076e293:     	leaq	(%rcx,%rax,8), %rax
14076e297:     	addq	%rdx, %rax
14076e29a:     	pxor	%xmm0, %xmm0
14076e29e:     	movd	%xmm0, 0x23c(%rbp)
14076e2a6:     	movq	%r12, 0x120(%rbp)
14076e2ad:     	movq	%r13, 0x128(%rbp)
14076e2b4:     	movq	%rax, 0x98(%rbp)
14076e2bb:     	cmpq	%rax, %rdx
14076e2be:     	je	0x14076f0bc <.text+0x76e0bc>
14076e2c4:     	movq	%rdx, 0x248(%rbp)
14076e2cb:     	movq	0x150(%rbp), %rax
14076e2d2:     	movq	%rax, 0x230(%rbp)
14076e2d9:     	movq	0x240(%rbp), %rax
14076e2e0:     	movq	-0x1d8(%rax), %rcx
14076e2e7:     	movq	%rcx, 0x130(%rbp)
14076e2ee:     	movq	-0x1d0(%rax), %rcx
14076e2f5:     	movq	%rcx, 0x138(%rbp)
14076e2fc:     	movq	-0x1c0(%rax), %rdi
14076e303:     	movq	-0x1b8(%rax), %r15
14076e30a:     	movq	-0x210(%rax), %r12
14076e311:     	movq	-0x208(%rax), %r13
14076e318:     	movq	-0x1f8(%rax), %r14
14076e31f:     	movq	-0x1f0(%rax), %rsi
14076e326:     	movl	-0x48(%rax), %ebx
14076e329:     	movb	$0x1, 0x257(%rbp)
14076e330:     	movq	0x1e8(%rbp), %rcx
14076e337:     	callq	0x140b2a770 <.text+0xb29770>
14076e33c:     	movl	%eax, 0x180(%rbp)
14076e342:     	movd	%xmm0, 0x184(%rbp)
14076e34a:     	movb	$0x1, 0x257(%rbp)
14076e351:     	cmpl	$0x2, %ebx
14076e354:     	setb	0x50(%rsp)
14076e359:     	leaq	0x180(%rbp), %rax
14076e360:     	movq	%rax, 0x58(%rsp)
14076e365:     	movq	%rsi, 0x48(%rsp)
14076e36a:     	movq	%r14, 0x40(%rsp)
14076e36f:     	movq	%r13, 0x38(%rsp)
14076e374:     	movq	%r12, 0x30(%rsp)
14076e379:     	movq	%r15, 0x28(%rsp)
14076e37e:     	movq	%rdi, 0x20(%rsp)
14076e383:     	leaq	0xe8(%rbp), %rcx
14076e38a:     	movq	0x248(%rbp), %rdx
14076e391:     	movq	0x130(%rbp), %r8
14076e398:     	movq	0x138(%rbp), %r9
14076e39f:     	callq	0x14122ff90 <.text+0x122ef90>
14076e3a4:     	movq	0xf0(%rbp), %rax
14076e3ab:     	movq	0xf8(%rbp), %r8
14076e3b2:     	movq	$0x0, 0xf8(%rbp)
14076e3bd:     	imulq	$0x68, %r8, %rcx
14076e3c1:     	addq	%rax, %rcx
14076e3c4:     	movq	%rax, 0xc0(%rbp)
14076e3cb:     	movq	%rcx, 0xc8(%rbp)
14076e3d2:     	leaq	0xe8(%rbp), %rax
14076e3d9:     	movq	%rax, 0xd0(%rbp)
14076e3e0:     	movq	%r8, 0xd8(%rbp)
14076e3e7:     	movq	$0x0, 0xe0(%rbp)
14076e3f2:     	movq	0x140(%rbp), %rax
14076e3f9:     	movq	0x230(%rbp), %rdx
14076e400:     	subq	%rdx, %rax
14076e403:     	movq	%rdx, %rsi
14076e406:     	cmpq	%r8, %rax
14076e409:     	jb	0x14076f08e <.text+0x76e08e>
14076e40f:     	movq	0x148(%rbp), %rbx
14076e416:     	movq	0xc0(%rbp), %rax
14076e41d:     	movq	0xc8(%rbp), %rcx
14076e424:     	movq	0xd0(%rbp), %rdx
14076e42b:     	movq	0xd8(%rbp), %r8
14076e432:     	movq	%rax, 0x1f0(%rbp)
14076e439:     	movq	%rcx, 0x1f8(%rbp)
14076e440:     	movq	0xe0(%rbp), %rax
14076e447:     	movq	%rax, 0x210(%rbp)
14076e44e:     	movq	%rdx, 0x200(%rbp)
14076e455:     	movq	%r8, 0x208(%rbp)
14076e45c:     	movq	0x1f0(%rbp), %r9
14076e463:     	movq	0x1f8(%rbp), %rax
14076e46a:     	cmpq	%rax, %r9
14076e46d:     	je	0x14076e559 <.text+0x76d559>
14076e473:     	imulq	$0x68, %rsi, %rcx
14076e477:     	addq	%rbx, %rcx
14076e47a:     	addq	$0x8, %rcx
14076e47e:     	addq	$0x68, %r9
14076e482:     	nopw	%cs:(%rax,%rax)
14076e490:     	movq	%r9, %rdx
14076e493:     	movq	-0x68(%r9), %r9
14076e497:     	movq	%r9, %r8
14076e49a:     	negq	%r8
14076e49d:     	jo	0x14076e552 <.text+0x76d552>
14076e4a3:     	leaq	-0x68(%rdx), %r8
14076e4a7:     	movups	0x58(%r8), %xmm0
14076e4ac:     	movaps	%xmm0, 0x1d0(%rbp)
14076e4b3:     	movups	0x48(%r8), %xmm0
14076e4b8:     	movaps	%xmm0, 0x1c0(%rbp)
14076e4bf:     	movups	0x8(%r8), %xmm0
14076e4c4:     	movups	0x18(%r8), %xmm1
14076e4c9:     	movups	0x28(%r8), %xmm2
14076e4ce:     	movups	0x38(%r8), %xmm3
14076e4d3:     	movaps	%xmm3, 0x1b0(%rbp)
14076e4da:     	movaps	%xmm2, 0x1a0(%rbp)
14076e4e1:     	movaps	%xmm1, 0x190(%rbp)
14076e4e8:     	movaps	%xmm0, 0x180(%rbp)
14076e4ef:     	movq	%r9, -0x8(%rcx)
14076e4f3:     	movaps	0x180(%rbp), %xmm0
14076e4fa:     	movdqa	0x190(%rbp), %xmm1
14076e502:     	movdqa	0x1a0(%rbp), %xmm2
14076e50a:     	movaps	0x1b0(%rbp), %xmm3
14076e511:     	movups	%xmm0, (%rcx)
14076e514:     	movdqu	%xmm1, 0x10(%rcx)
14076e519:     	movdqu	%xmm2, 0x20(%rcx)
14076e51e:     	movups	%xmm3, 0x30(%rcx)
14076e522:     	movaps	0x1c0(%rbp), %xmm0
14076e529:     	movups	%xmm0, 0x40(%rcx)
14076e52d:     	movdqa	0x1d0(%rbp), %xmm0
14076e535:     	movdqu	%xmm0, 0x50(%rcx)
14076e53a:     	incq	%rsi
14076e53d:     	addq	$0x68, %rcx
14076e541:     	leaq	0x68(%rdx), %r9
14076e545:     	addq	$0x68, %r8
14076e549:     	cmpq	%rax, %r8
14076e54c:     	jne	0x14076e490 <.text+0x76d490>
14076e552:     	movq	%rdx, 0x1f0(%rbp)
14076e559:     	movq	%rsi, 0x150(%rbp)
14076e560:     	movb	$0x0, 0x254(%rbp)
14076e567:     	leaq	0x1f0(%rbp), %rcx
14076e56e:     	callq	0x1411fd610 <.text+0x11fc610>
14076e573:     	movb	$0x1, 0x257(%rbp)
14076e57a:     	leaq	0xe8(%rbp), %rcx
14076e581:     	callq	0x1405d3750 <.text+0x5d2750>
14076e586:     	movq	0x230(%rbp), %rdx
14076e58d:     	cmpq	%rsi, %rdx
14076e590:     	ja	0x14076f7ae <.text+0x76e7ae>
14076e596:     	movq	0x248(%rbp), %rax
14076e59d:     	leaq	0x88(%rax), %rcx
14076e5a4:     	movq	%rcx, 0x90(%rbp)
14076e5ab:     	movss	0x23c(%rbp), %xmm0
14076e5b3:     	addss	0x84(%rax), %xmm0
14076e5bb:     	movss	%xmm0, 0x23c(%rbp)
14076e5c3:     	imulq	$0x68, %rdx, %r12
14076e5c7:     	addq	%rbx, %r12
14076e5ca:     	imulq	$0x68, %rsi, %r14
14076e5ce:     	addq	%rbx, %r14
14076e5d1:     	xorl	%r8d, %r8d
14076e5d4:     	movq	%r12, %r15
14076e5d7:     	movq	%r14, 0x130(%rbp)
14076e5de:     	cmpq	%r14, %r15
14076e5e1:     	je	0x14076ea8f <.text+0x76da8f>
14076e5e7:     	movq	0x10(%r15), %rax
14076e5eb:     	testq	%rax, %rax
14076e5ee:     	je	0x14076f8e9 <.text+0x76e8e9>
14076e5f4:     	movq	0x28(%r15), %r14
14076e5f8:     	testq	%r14, %r14
14076e5fb:     	je	0x14076f8cd <.text+0x76e8cd>
14076e601:     	leaq	-0x1(%rax), %rcx
14076e605:     	movabsq	$0x3fffffffffffffff, %rdx # imm = 0x3FFFFFFFFFFFFFFF
14076e60f:     	andq	%rdx, %rcx
14076e612:     	cmpq	%rcx, %r14
14076e615:     	jbe	0x14076f9fc <.text+0x76e9fc>
14076e61b:     	movq	%r12, %rdi
14076e61e:     	leaq	0x68(%r15), %rcx
14076e622:     	movq	%rcx, 0x88(%rbp)
14076e629:     	movq	%r8, 0x138(%rbp)
14076e630:     	leal	0x1(%r8), %ecx
14076e634:     	movl	%ecx, 0x10c(%rbp)
14076e63a:     	movq	0x240(%rbp), %rcx
14076e641:     	movss	-0x1c(%rcx), %xmm8
14076e647:     	movss	-0x18(%rcx), %xmm9
14076e64d:     	movq	0x8(%r15), %rsi
14076e651:     	movq	0x20(%r15), %rdx
14076e655:     	leaq	0x4(%rdx), %rcx
14076e659:     	movq	%rdx, %r13
14076e65c:     	movss	0x4(%rdx), %xmm7
14076e661:     	movss	(%rsi), %xmm0
14076e665:     	addss	%xmm7, %xmm0
14076e669:     	shlq	$0x2, %rax
14076e66d:     	xorl	%edx, %edx
14076e66f:     	nop
14076e670:     	movss	(%rcx), %xmm1
14076e674:     	movss	(%rsi,%rdx), %xmm2
14076e679:     	addss	%xmm1, %xmm2
14076e67d:     	movaps	%xmm0, %xmm3
14076e680:     	cmpltss	%xmm2, %xmm3
14076e685:     	andps	%xmm3, %xmm1
14076e688:     	andnps	%xmm7, %xmm3
14076e68b:     	orps	%xmm1, %xmm3
14076e68e:     	movaps	%xmm3, %xmm7
14076e691:     	maxss	%xmm0, %xmm2
14076e695:     	addq	$0xc, %rcx
14076e699:     	addq	$0x4, %rdx
14076e69d:     	movaps	%xmm2, %xmm0
14076e6a0:     	cmpq	%rdx, %rax
14076e6a3:     	jne	0x14076e670 <.text+0x76d670>
14076e6a5:     	addss	0x21b71c2(%rip), %xmm8  # 0x142925870
14076e6ae:     	leaq	(,%r14,4), %r12
14076e6b6:     	movzbl	0x2ec654c(%rip), %eax   # 0x143634c09
14076e6bd:     	movl	$0x4, %edx
14076e6c2:     	movq	%r12, %rcx
14076e6c5:     	callq	0x140613c10 <.text+0x612c10>
14076e6ca:     	testq	%rax, %rax
14076e6cd:     	je	0x14076f8ad <.text+0x76e8ad>
14076e6d3:     	movq	%rax, %rbx
14076e6d6:     	movaps	%xmm9, %xmm0
14076e6da:     	cmpunordss	%xmm9, %xmm0
14076e6e0:     	movaps	%xmm0, %xmm1
14076e6e3:     	andps	%xmm8, %xmm1
14076e6e7:     	maxss	%xmm9, %xmm8
14076e6ec:     	andnps	%xmm8, %xmm0
14076e6f0:     	orps	%xmm1, %xmm0
14076e6f3:     	cmpq	$0x3, %r14
14076e6f7:     	ja	0x14076e710 <.text+0x76d710>
14076e6f9:     	xorl	%eax, %eax
14076e6fb:     	movq	%rdi, %r12
14076e6fe:     	jmp	0x14076e7b0 <.text+0x76d7b0>
14076e703:     	nopw	%cs:(%rax,%rax)
14076e710:     	leaq	(%rbx,%r14,4), %rax
14076e714:     	leaq	0x4(%r13), %rcx
14076e718:     	leaq	(%r14,%r14,2), %rdx
14076e71c:     	leaq	-0x4(,%rdx,4), %rdx
14076e724:     	addq	%r13, %rdx
14076e727:     	cmpq	%rdx, %rbx
14076e72a:     	setb	%dl
14076e72d:     	cmpq	%rax, %rcx
14076e730:     	setb	%al
14076e733:     	testb	%al, %dl
14076e735:     	movq	%rdi, %r12
14076e738:     	je	0x14076e73e <.text+0x76d73e>
14076e73a:     	xorl	%eax, %eax
14076e73c:     	jmp	0x14076e7b0 <.text+0x76d7b0>
14076e73e:     	movq	%r14, %rax
14076e741:     	andq	$-0x4, %rax
14076e745:     	movaps	%xmm7, %xmm1
14076e748:     	shufps	$0x0, %xmm7, %xmm1      # xmm1 = xmm1[0,0],xmm7[0,0]
14076e74c:     	movaps	%xmm0, %xmm2
14076e74f:     	shufps	$0x0, %xmm0, %xmm2      # xmm2 = xmm2[0,0],xmm0[0,0]
14076e753:     	leaq	0x28(%r13), %rcx
14076e757:     	xorl	%edx, %edx
14076e759:     	nopl	(%rax)
14076e760:     	movss	-0x24(%rcx), %xmm3
14076e765:     	movss	-0x18(%rcx), %xmm4
14076e76a:     	unpcklps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0],xmm3[1],xmm4[1]
14076e76d:     	movss	-0xc(%rcx), %xmm4
14076e772:     	movss	(%rcx), %xmm5
14076e776:     	unpcklps	%xmm5, %xmm4            # xmm4 = xmm4[0],xmm5[0],xmm4[1],xmm5[1]
14076e779:     	movlhps	%xmm4, %xmm3            # xmm3 = xmm3[0],xmm4[0]
14076e77c:     	movaps	%xmm1, %xmm4
14076e77f:     	subps	%xmm3, %xmm4
14076e782:     	addps	%xmm14, %xmm4
14076e786:     	maxps	%xmm13, %xmm4
14076e78a:     	addps	%xmm2, %xmm4
14076e78d:     	movups	%xmm4, (%rbx,%rdx,4)
14076e791:     	addq	$0x4, %rdx
14076e795:     	addq	$0x30, %rcx
14076e799:     	cmpq	%rdx, %rax
14076e79c:     	jne	0x14076e760 <.text+0x76d760>
14076e79e:     	cmpq	%rax, %r14
14076e7a1:     	je	0x14076e7e6 <.text+0x76d7e6>
14076e7a3:     	nopw	%cs:(%rax,%rax)
14076e7b0:     	leaq	(%rax,%rax,2), %rcx
14076e7b4:     	leaq	0x4(,%rcx,4), %rcx
14076e7bc:     	addq	%r13, %rcx
14076e7bf:     	nop
14076e7c0:     	movaps	%xmm7, %xmm1
14076e7c3:     	subss	(%rcx), %xmm1
14076e7c7:     	addss	%xmm11, %xmm1
14076e7cc:     	maxss	%xmm12, %xmm1
14076e7d1:     	addss	%xmm0, %xmm1
14076e7d5:     	movss	%xmm1, (%rbx,%rax,4)
14076e7da:     	incq	%rax
14076e7dd:     	addq	$0xc, %rcx
14076e7e1:     	cmpq	%rax, %r14
14076e7e4:     	jne	0x14076e7c0 <.text+0x76d7c0>
14076e7e6:     	movq	(%r15), %rdx
14076e7e9:     	testq	%rdx, %rdx
14076e7ec:     	je	0x14076e800 <.text+0x76d800>
14076e7ee:     	shlq	$0x2, %rdx
14076e7f2:     	movl	$0x4, %r8d
14076e7f8:     	movq	%rsi, %rcx
14076e7fb:     	callq	0x140613c20 <.text+0x612c20>
14076e800:     	movq	%r14, (%r15)
14076e803:     	movq	%rbx, 0x8(%r15)
14076e807:     	movq	%r14, 0x10(%r15)
14076e80b:     	movl	0x60(%r15), %eax
14076e80f:     	leaq	0x188(%rbp), %rcx
14076e816:     	movl	%eax, 0x8(%rcx)
14076e819:     	movq	0x58(%r15), %rax
14076e81d:     	movq	%rax, (%rcx)
14076e820:     	leaq	(%rbx,%r14,4), %rax
14076e824:     	movups	%xmm13, 0x20(%rcx)
14076e829:     	movq	$0x0, 0x30(%rcx)
14076e831:     	movq	0x1e8(%rbp), %rcx
14076e838:     	movq	%rcx, 0x180(%rbp)
14076e83f:     	movq	%rbx, 0x198(%rbp)
14076e846:     	movq	%rax, 0x1a0(%rbp)
14076e84d:     	movb	$0x1, 0x257(%rbp)
14076e854:     	leaq	0xc0(%rbp), %rcx
14076e85b:     	leaq	0x180(%rbp), %rdx
14076e862:     	leaq	0x2310017(%rip), %r8    # 0x142a7e880
14076e869:     	callq	0x141204400 <.text+0x1203400>
14076e86e:     	movq	0x130(%rbp), %r14
14076e875:     	movq	0x10(%r15), %rdi
14076e879:     	testq	%rdi, %rdi
14076e87c:     	movq	%rdi, 0x230(%rbp)
14076e883:     	je	0x14076f88d <.text+0x76e88d>
14076e889:     	movzbl	0x2ec6379(%rip), %eax   # 0x143634c09
14076e890:     	movl	$0x1, %edx
14076e895:     	movq	%rdi, %rcx
14076e898:     	callq	0x140613c40 <.text+0x612c40>
14076e89d:     	testq	%rax, %rax
14076e8a0:     	je	0x14076f874 <.text+0x76e874>
14076e8a6:     	movq	%rax, %r13
14076e8a9:     	movb	$0x8, (%rax)
14076e8ac:     	movq	0x10(%r15), %rcx
14076e8b0:     	decq	%rcx
14076e8b3:     	cmpq	%rdi, %rcx
14076e8b6:     	jae	0x14076fa1a <.text+0x76ea1a>
14076e8bc:     	movb	$0x8, (%r13,%rcx)
14076e8c2:     	movq	0x10(%r15), %rax
14076e8c6:     	movq	$0x0, 0x110(%rbp)
14076e8d1:     	movq	%rax, 0xe8(%rbp)
14076e8d8:     	cmpq	$0x1, %rax
14076e8dc:     	je	0x14076f3f8 <.text+0x76e3f8>
14076e8e2:     	testq	%rax, %rax
14076e8e5:     	je	0x14076f6ba <.text+0x76e6ba>
14076e8eb:     	movq	0xd0(%rbp), %rsi
14076e8f2:     	testq	%rsi, %rsi
14076e8f5:     	je	0x14076e95e <.text+0x76d95e>
14076e8f7:     	leaq	(,%rsi,8), %rbx
14076e8ff:     	movzbl	0x2ec6303(%rip), %eax   # 0x143634c09
14076e906:     	movl	$0x4, %edx
14076e90b:     	movq	%rbx, %rcx
14076e90e:     	callq	0x140613c10 <.text+0x612c10>
14076e913:     	testq	%rax, %rax
14076e916:     	je	0x14076fa7c <.text+0x76ea7c>
14076e91c:     	movl	$0x1, %ecx
14076e921:     	movq	%rcx, 0x178(%rbp)
14076e928:     	movq	%rax, %rcx
14076e92b:     	cmpq	$0x1, %rsi
14076e92f:     	movq	%rax, %rbx
14076e932:     	je	0x14076e955 <.text+0x76d955>
14076e934:     	leaq	-0x8(,%rsi,8), %r8
14076e93c:     	movq	%rbx, %rcx
14076e93f:     	xorl	%edx, %edx
14076e941:     	callq	0x142923c20 <.text+0x2922c20>
14076e946:     	leaq	(%rbx,%rsi,8), %rcx
14076e94a:     	addq	$-0x8, %rcx
14076e94e:     	movq	%rsi, 0x178(%rbp)
14076e955:     	movq	$0x0, (%rcx)
14076e95c:     	jmp	0x14076e96e <.text+0x76d96e>
14076e95e:     	movl	$0x4, %ebx
14076e963:     	movq	$0x0, 0x178(%rbp)
14076e96e:     	movq	0x10(%r15), %rax
14076e972:     	movq	%rax, %rcx
14076e975:     	movq	%rax, 0xe8(%rbp)
14076e97c:     	subq	$0x1, %rcx
14076e980:     	movq	%rcx, 0x110(%rbp)
14076e987:     	movq	%r13, 0xb0(%rbp)
14076e98e:     	movq	%rbx, 0xb8(%rbp)
14076e995:     	jb	0x14076f7cc <.text+0x76e7cc>
14076e99b:     	je	0x14076fa6e <.text+0x76ea6e>
14076e9a1:     	cmpq	%rsi, %rcx
14076e9a4:     	jae	0x14076fa5d <.text+0x76ea5d>
14076e9aa:     	addl	$-0x2, %eax
14076e9ad:     	movl	%eax, (%rbx,%rcx,8)
14076e9b0:     	movl	$0x3f800000, 0x4(%rbx,%rcx,8) # imm = 0x3F800000
14076e9b8:     	movq	0x8(%r15), %r8
14076e9bc:     	movq	0x10(%r15), %r9
14076e9c0:     	movups	0x20(%r15), %xmm0
14076e9c5:     	movq	%rbx, %rdx
14076e9c8:     	movq	0xc8(%rbp), %rbx
14076e9cf:     	movq	0x228(%rbp), %rcx
14076e9d6:     	movq	0x138(%rbp), %rax
14076e9dd:     	movl	%eax, 0x68(%rsp)
14076e9e1:     	movq	%rsi, 0x60(%rsp)
14076e9e6:     	movq	%rdx, 0x58(%rsp)
14076e9eb:     	movq	%rdi, 0x50(%rsp)
14076e9f0:     	movq	%r13, 0x48(%rsp)
14076e9f5:     	leaq	-0x10(%rbp), %rax
14076e9f9:     	movq	%rax, 0x40(%rsp)
14076e9fe:     	movq	%rsi, 0x38(%rsp)
14076ea03:     	movq	%rbx, 0x30(%rsp)
14076ea08:     	movups	%xmm0, 0x20(%rsp)
14076ea0d:     	movq	0x1e8(%rbp), %rdx
14076ea14:     	callq	0x141242530 <.text+0x1241530>
14076ea19:     	movq	0x178(%rbp), %rdx
14076ea20:     	testq	%rdx, %rdx
14076ea23:     	je	0x14076ea3b <.text+0x76da3b>
14076ea25:     	shlq	$0x3, %rdx
14076ea29:     	movl	$0x4, %r8d
14076ea2f:     	movq	0xb8(%rbp), %rcx
14076ea36:     	callq	0x140613c20 <.text+0x612c20>
14076ea3b:     	movl	$0x1, %r8d
14076ea41:     	movq	0xb0(%rbp), %rcx
14076ea48:     	movq	0x230(%rbp), %rdx
14076ea4f:     	callq	0x140613c20 <.text+0x612c20>
14076ea54:     	movq	0xc0(%rbp), %rax
14076ea5b:     	testq	%rax, %rax
14076ea5e:     	je	0x14076ea76 <.text+0x76da76>
14076ea60:     	shlq	$0x2, %rax
14076ea64:     	leaq	(%rax,%rax,2), %rdx
14076ea68:     	movl	$0x4, %r8d
14076ea6e:     	movq	%rbx, %rcx
14076ea71:     	callq	0x140613c20 <.text+0x612c20>
14076ea76:     	movq	0x88(%rbp), %r15
14076ea7d:     	movl	0x10c(%rbp), %eax
14076ea83:     	movl	%eax, %r8d
14076ea86:     	cmpq	%r14, %r15
14076ea89:     	jne	0x14076e5e7 <.text+0x76d5e7>
14076ea8f:     	movq	0x248(%rbp), %r8
14076ea96:     	leaq	0x30(%r8), %rbx
14076ea9a:     	leaq	0x60(%r8), %r15
14076ea9e:     	leaq	0x48(%r8), %r13
14076eaa2:     	pcmpeqd	%xmm6, %xmm6
14076eaa6:     	cmpq	%r14, %r12
14076eaa9:     	jne	0x14076eb03 <.text+0x76db03>
14076eaab:     	movq	0x90(%rbp), %rdx
14076eab2:     	movq	0x120(%rbp), %r12
14076eab9:     	movq	0x128(%rbp), %r13
14076eac0:     	movq	0x98(%rbp), %rax
14076eac7:     	cmpq	%rax, %rdx
14076eaca:     	jne	0x14076e2c4 <.text+0x76d2c4>
14076ead0:     	jmp	0x14076f0bc <.text+0x76e0bc>
14076ead5:     	nopw	%cs:(%rax,%rax)
14076eae0:     	movq	0x248(%rbp), %r8
14076eae7:     	movq	0x68(%r8), %rax
14076eaeb:     	movq	%rsi, (%rax,%rdi,8)
14076eaef:     	incq	%rdi
14076eaf2:     	movq	%rdi, 0x70(%r8)
14076eaf6:     	addq	$0x68, %r12
14076eafa:     	pcmpeqd	%xmm6, %xmm6
14076eafe:     	cmpq	%r14, %r12
14076eb01:     	je	0x14076eaab <.text+0x76daab>
14076eb03:     	movq	0x28(%r12), %rax
14076eb08:     	testq	%rax, %rax
14076eb0b:     	je	0x14076f9e0 <.text+0x76e9e0>
14076eb11:     	cmpq	$0x0, 0x10(%r12)
14076eb17:     	je	0x14076f9c4 <.text+0x76e9c4>
14076eb1d:     	movq	0x8(%r12), %rcx
14076eb22:     	movss	(%rcx), %xmm7
14076eb26:     	addss	0x230fc42(%rip), %xmm7  # 0x142a7e770
14076eb2e:     	cmpq	$0x1, %rax
14076eb32:     	je	0x14076f9a2 <.text+0x76e9a2>
14076eb38:     	cmpq	$0x0, 0x40(%r12)
14076eb3e:     	je	0x14076f986 <.text+0x76e986>
14076eb44:     	movq	0x48(%r12), %rsi
14076eb49:     	movq	0x20(%r12), %rax
14076eb4e:     	movss	0xc(%rax), %xmm0
14076eb53:     	subss	(%rax), %xmm0
14076eb57:     	movq	0x38(%r12), %rcx
14076eb5c:     	movaps	%xmm0, %xmm1
14076eb5f:     	mulss	%xmm0, %xmm1
14076eb63:     	movss	0x14(%rax), %xmm10
14076eb69:     	subss	0x8(%rax), %xmm10
14076eb6f:     	movaps	%xmm10, %xmm2
14076eb73:     	mulss	%xmm10, %xmm2
14076eb78:     	addss	%xmm1, %xmm2
14076eb7c:     	xorps	%xmm1, %xmm1
14076eb7f:     	sqrtss	%xmm2, %xmm1
14076eb83:     	movss	0x21b6d78(%rip), %xmm9  # 0x142925904
14076eb8c:     	divss	%xmm1, %xmm9
14076eb91:     	mulss	%xmm9, %xmm10
14076eb96:     	movss	(%rcx), %xmm8
14076eb9b:     	movl	0x8(%rax), %ecx
14076eb9e:     	movl	%ecx, 0x188(%rbp)
14076eba4:     	movq	(%rax), %rax
14076eba7:     	movq	%rax, 0x180(%rbp)
14076ebae:     	movq	0x70(%r8), %rax
14076ebb2:     	testq	%rax, %rax
14076ebb5:     	sete	%cl
14076ebb8:     	shlq	$0x3, %rax
14076ebbc:     	addq	0x68(%r8), %rax
14076ebc0:     	mulss	%xmm0, %xmm9
14076ebc5:     	addq	$-0x8, %rax
14076ebc9:     	sete	%dl
14076ebcc:     	orb	%cl, %dl
14076ebce:     	jne	0x14076ec90 <.text+0x76dc90>
14076ebd4:     	cmpq	%rsi, (%rax)
14076ebd7:     	jne	0x14076ec90 <.text+0x76dc90>
14076ebdd:     	movq	0x58(%r8), %rax
14076ebe1:     	testq	%rax, %rax
14076ebe4:     	je	0x14076faae <.text+0x76eaae>
14076ebea:     	movq	0x50(%r8), %rcx
14076ebee:     	leaq	(%rcx,%rax,4), %rax
14076ebf2:     	addq	$-0x4, %rax
14076ebf6:     	testq	%rax, %rax
14076ebf9:     	je	0x14076faae <.text+0x76eaae>
14076ebff:     	movss	(%rax), %xmm0
14076ec03:     	movaps	%xmm7, %xmm1
14076ec06:     	minss	%xmm0, %xmm1
14076ec0a:     	cmpunordss	%xmm0, %xmm0
14076ec0f:     	movaps	%xmm0, %xmm2
14076ec12:     	andps	%xmm7, %xmm2
14076ec15:     	andnps	%xmm1, %xmm0
14076ec18:     	orps	%xmm2, %xmm0
14076ec1b:     	movss	%xmm0, (%rax)
14076ec1f:     	movq	0x40(%r8), %rcx
14076ec23:     	testq	%rcx, %rcx
14076ec26:     	je	0x14076fa99 <.text+0x76ea99>
14076ec2c:     	movq	0x38(%r8), %rax
14076ec30:     	leaq	(%rcx,%rcx,8), %rdx
14076ec34:     	leaq	(%rdx,%rdx,2), %rdx
14076ec38:     	addq	%rcx, %rdx
14076ec3b:     	leaq	(%rax,%rdx), %rcx
14076ec3f:     	addq	$-0x1c, %rcx
14076ec43:     	testq	%rcx, %rcx
14076ec46:     	je	0x14076fa99 <.text+0x76ea99>
14076ec4c:     	addq	%rdx, %rax
14076ec4f:     	ucomiss	-0x8(%rax), %xmm7
14076ec53:     	jbe	0x14076ed6a <.text+0x76dd6a>
14076ec59:     	movss	%xmm9, -0x1c(%rax)
14076ec5f:     	movss	%xmm10, -0x18(%rax)
14076ec65:     	movq	0x180(%rbp), %rcx
14076ec6c:     	movq	%rcx, -0x14(%rax)
14076ec70:     	movl	0x188(%rbp), %ecx
14076ec76:     	movl	%ecx, -0xc(%rax)
14076ec79:     	movss	%xmm7, -0x8(%rax)
14076ec7e:     	movss	%xmm8, -0x4(%rax)
14076ec84:     	jmp	0x14076ed6a <.text+0x76dd6a>
14076ec89:     	nopl	(%rax)
14076ec90:     	movq	0x58(%r8), %rdi
14076ec94:     	cmpq	0x48(%r8), %rdi
14076ec98:     	jne	0x14076ecb0 <.text+0x76dcb0>
14076ec9a:     	movb	$0x1, 0x257(%rbp)
14076eca1:     	movq	%r13, %rcx
14076eca4:     	leaq	0x230d7cd(%rip), %rdx   # 0x142a7c478
14076ecab:     	callq	0x140650050 <.text+0x64f050>
14076ecb0:     	movq	0x248(%rbp), %rcx
14076ecb7:     	movq	0x50(%rcx), %rax
14076ecbb:     	movss	%xmm7, (%rax,%rdi,4)
14076ecc0:     	incq	%rdi
14076ecc3:     	movq	%rdi, 0x58(%rcx)
14076ecc7:     	movq	0x40(%rcx), %rdi
14076eccb:     	cmpq	0x30(%rcx), %rdi
14076eccf:     	jne	0x14076ece7 <.text+0x76dce7>
14076ecd1:     	movb	$0x1, 0x257(%rbp)
14076ecd8:     	movq	%rbx, %rcx
14076ecdb:     	leaq	0x230d7ae(%rip), %rdx   # 0x142a7c490
14076ece2:     	callq	0x14078cb80 <.text+0x78bb80>
14076ece7:     	movq	0x248(%rbp), %r8
14076ecee:     	movq	0x38(%r8), %rax
14076ecf2:     	leaq	(%rdi,%rdi,8), %rcx
14076ecf6:     	leaq	(%rcx,%rcx,2), %rcx
14076ecfa:     	addq	%rdi, %rcx
14076ecfd:     	movss	%xmm9, (%rax,%rcx)
14076ed03:     	movss	%xmm10, 0x4(%rax,%rcx)
14076ed0a:     	movq	0x180(%rbp), %rdx
14076ed11:     	movq	%rdx, 0x8(%rax,%rcx)
14076ed16:     	movl	0x188(%rbp), %edx
14076ed1c:     	movl	%edx, 0x10(%rax,%rcx)
14076ed20:     	movss	%xmm7, 0x14(%rax,%rcx)
14076ed26:     	movss	%xmm8, 0x18(%rax,%rcx)
14076ed2d:     	incq	%rdi
14076ed30:     	movq	%rdi, 0x40(%r8)
14076ed34:     	movq	0x70(%r8), %rdi
14076ed38:     	cmpq	0x60(%r8), %rdi
14076ed3c:     	jne	0x14076ed54 <.text+0x76dd54>
14076ed3e:     	movb	$0x1, 0x257(%rbp)
14076ed45:     	movq	%r15, %rcx
14076ed48:     	leaq	0x230d759(%rip), %rdx   # 0x142a7c4a8
14076ed4f:     	callq	0x140b81df0 <.text+0xb80df0>
14076ed54:     	movq	0x248(%rbp), %rcx
14076ed5b:     	movq	0x68(%rcx), %rax
14076ed5f:     	movq	%rsi, (%rax,%rdi,8)
14076ed63:     	incq	%rdi
14076ed66:     	movq	%rdi, 0x70(%rcx)
14076ed6a:     	movq	0x50(%r12), %rsi
14076ed6f:     	movb	$0x1, 0x257(%rbp)
14076ed76:     	movq	%r12, %rcx
14076ed79:     	callq	0x141234970 <.text+0x1233970>
14076ed7e:     	decq	%rax
14076ed81:     	movq	0x28(%r12), %rdx
14076ed86:     	cmpq	%rdx, %rax
14076ed89:     	jae	0x14076f905 <.text+0x76e905>
14076ed8f:     	movq	0x20(%r12), %rcx
14076ed94:     	leaq	(%rax,%rax,2), %rax
14076ed98:     	movl	0x8(%rcx,%rax,4), %edx
14076ed9c:     	movl	%edx, 0x188(%rbp)
14076eda2:     	movq	(%rcx,%rax,4), %rax
14076eda6:     	movq	%rax, 0x180(%rbp)
14076edad:     	movb	$0x1, 0x257(%rbp)
14076edb4:     	movq	%r12, %rcx
14076edb7:     	callq	0x141234970 <.text+0x1233970>
14076edbc:     	leaq	-0x1(%rax), %rcx
14076edc0:     	movq	0x10(%r12), %rdx
14076edc5:     	cmpq	%rdx, %rcx
14076edc8:     	jae	0x14076f920 <.text+0x76e920>
14076edce:     	movq	0x8(%r12), %rcx
14076edd3:     	movss	-0x4(%rcx,%rax,4), %xmm10
14076edda:     	addss	0x230f98d(%rip), %xmm10 # 0x142a7e770
14076ede3:     	movb	$0x1, 0x257(%rbp)
14076edea:     	movq	%r12, %rcx
14076eded:     	callq	0x141234970 <.text+0x1233970>
14076edf2:     	decq	%rax
14076edf5:     	movq	0x28(%r12), %rdx
14076edfa:     	cmpq	%rdx, %rax
14076edfd:     	jae	0x14076f938 <.text+0x76e938>
14076ee03:     	movq	0x20(%r12), %rcx
14076ee08:     	leaq	(%rax,%rax,2), %rax
14076ee0c:     	movss	(%rcx,%rax,4), %xmm7
14076ee11:     	movss	0x8(%rcx,%rax,4), %xmm6
14076ee17:     	movb	$0x1, 0x257(%rbp)
14076ee1e:     	movq	%r12, %rcx
14076ee21:     	callq	0x141234970 <.text+0x1233970>
14076ee26:     	addq	$-0x2, %rax
14076ee2a:     	movq	0x28(%r12), %rdx
14076ee2f:     	cmpq	%rdx, %rax
14076ee32:     	jae	0x14076f953 <.text+0x76e953>
14076ee38:     	movq	0x20(%r12), %rcx
14076ee3d:     	leaq	(%rax,%rax,2), %rax
14076ee41:     	subss	(%rcx,%rax,4), %xmm7
14076ee46:     	subss	0x8(%rcx,%rax,4), %xmm6
14076ee4c:     	movb	$0x1, 0x257(%rbp)
14076ee53:     	movq	%r12, %rcx
14076ee56:     	callq	0x141234970 <.text+0x1233970>
14076ee5b:     	leaq	-0x1(%rax), %rcx
14076ee5f:     	movq	0x40(%r12), %rdx
14076ee64:     	cmpq	%rdx, %rcx
14076ee67:     	jae	0x14076f96e <.text+0x76e96e>
14076ee6d:     	decq	%rsi
14076ee70:     	movaps	%xmm7, %xmm0
14076ee73:     	mulss	%xmm7, %xmm0
14076ee77:     	movaps	%xmm6, %xmm1
14076ee7a:     	mulss	%xmm6, %xmm1
14076ee7e:     	addss	%xmm0, %xmm1
14076ee82:     	xorps	%xmm0, %xmm0
14076ee85:     	sqrtss	%xmm1, %xmm0
14076ee89:     	movss	0x21b6a72(%rip), %xmm9  # 0x142925904
14076ee92:     	divss	%xmm0, %xmm9
14076ee97:     	mulss	%xmm9, %xmm7
14076ee9c:     	movq	0x38(%r12), %rcx
14076eea1:     	movss	-0x4(%rcx,%rax,4), %xmm8
14076eea8:     	movl	0x188(%rbp), %eax
14076eeae:     	movl	%eax, 0x1f8(%rbp)
14076eeb4:     	movq	0x180(%rbp), %rax
14076eebb:     	movq	%rax, 0x1f0(%rbp)
14076eec2:     	movq	0x248(%rbp), %r8
14076eec9:     	movq	0x70(%r8), %rax
14076eecd:     	testq	%rax, %rax
14076eed0:     	sete	%cl
14076eed3:     	shlq	$0x3, %rax
14076eed7:     	addq	0x68(%r8), %rax
14076eedb:     	mulss	%xmm6, %xmm9
14076eee0:     	addq	$-0x8, %rax
14076eee4:     	sete	%dl
14076eee7:     	orb	%cl, %dl
14076eee9:     	jne	0x14076efc0 <.text+0x76dfc0>
14076eeef:     	cmpq	%rsi, (%rax)
14076eef2:     	jne	0x14076efc0 <.text+0x76dfc0>
14076eef8:     	movq	0x58(%r8), %rax
14076eefc:     	testq	%rax, %rax
14076eeff:     	je	0x14076fad8 <.text+0x76ead8>
14076ef05:     	movq	0x50(%r8), %rcx
14076ef09:     	leaq	(%rcx,%rax,4), %rax
14076ef0d:     	addq	$-0x4, %rax
14076ef11:     	testq	%rax, %rax
14076ef14:     	je	0x14076fad8 <.text+0x76ead8>
14076ef1a:     	movss	(%rax), %xmm0
14076ef1e:     	movaps	%xmm10, %xmm1
14076ef22:     	minss	%xmm0, %xmm1
14076ef26:     	cmpunordss	%xmm0, %xmm0
14076ef2b:     	movaps	%xmm0, %xmm2
14076ef2e:     	andps	%xmm10, %xmm2
14076ef32:     	andnps	%xmm1, %xmm0
14076ef35:     	orps	%xmm2, %xmm0
14076ef38:     	movss	%xmm0, (%rax)
14076ef3c:     	movq	0x40(%r8), %rcx
14076ef40:     	testq	%rcx, %rcx
14076ef43:     	je	0x14076fac3 <.text+0x76eac3>
14076ef49:     	movq	0x38(%r8), %rax
14076ef4d:     	leaq	(%rcx,%rcx,8), %rdx
14076ef51:     	leaq	(%rdx,%rdx,2), %rdx
14076ef55:     	addq	%rcx, %rdx
14076ef58:     	leaq	(%rax,%rdx), %rcx
14076ef5c:     	addq	$-0x1c, %rcx
14076ef60:     	testq	%rcx, %rcx
14076ef63:     	je	0x14076fac3 <.text+0x76eac3>
14076ef69:     	addq	%rdx, %rax
14076ef6c:     	ucomiss	-0x8(%rax), %xmm10
14076ef71:     	jbe	0x14076ef9e <.text+0x76df9e>
14076ef73:     	movss	%xmm7, -0x1c(%rax)
14076ef78:     	movss	%xmm9, -0x18(%rax)
14076ef7e:     	movq	0x1f0(%rbp), %rcx
14076ef85:     	movq	%rcx, -0x14(%rax)
14076ef89:     	movl	0x1f8(%rbp), %ecx
14076ef8f:     	movl	%ecx, -0xc(%rax)
14076ef92:     	movss	%xmm10, -0x8(%rax)
14076ef98:     	movss	%xmm8, -0x4(%rax)
14076ef9e:     	addq	$0x68, %r12
14076efa2:     	pcmpeqd	%xmm6, %xmm6
14076efa6:     	cmpq	%r14, %r12
14076efa9:     	jne	0x14076eb03 <.text+0x76db03>
14076efaf:     	jmp	0x14076eaab <.text+0x76daab>
14076efb4:     	nopw	%cs:(%rax,%rax)
14076efc0:     	movq	0x58(%r8), %rdi
14076efc4:     	cmpq	0x48(%r8), %rdi
14076efc8:     	jne	0x14076efe0 <.text+0x76dfe0>
14076efca:     	movb	$0x1, 0x257(%rbp)
14076efd1:     	movq	%r13, %rcx
14076efd4:     	leaq	0x230d49d(%rip), %rdx   # 0x142a7c478
14076efdb:     	callq	0x140650050 <.text+0x64f050>
14076efe0:     	movq	0x248(%rbp), %rcx
14076efe7:     	movq	0x50(%rcx), %rax
14076efeb:     	movss	%xmm10, (%rax,%rdi,4)
14076eff1:     	incq	%rdi
14076eff4:     	movq	%rdi, 0x58(%rcx)
14076eff8:     	movq	0x40(%rcx), %rdi
14076effc:     	cmpq	0x30(%rcx), %rdi
14076f000:     	jne	0x14076f018 <.text+0x76e018>
14076f002:     	movb	$0x1, 0x257(%rbp)
14076f009:     	movq	%rbx, %rcx
14076f00c:     	leaq	0x230d47d(%rip), %rdx   # 0x142a7c490
14076f013:     	callq	0x14078cb80 <.text+0x78bb80>
14076f018:     	movq	0x248(%rbp), %r8
14076f01f:     	movq	0x38(%r8), %rax
14076f023:     	leaq	(%rdi,%rdi,8), %rcx
14076f027:     	leaq	(%rcx,%rcx,2), %rcx
14076f02b:     	addq	%rdi, %rcx
14076f02e:     	movss	%xmm7, (%rax,%rcx)
14076f033:     	movss	%xmm9, 0x4(%rax,%rcx)
14076f03a:     	movq	0x1f0(%rbp), %rdx
14076f041:     	movq	%rdx, 0x8(%rax,%rcx)
14076f046:     	movl	0x1f8(%rbp), %edx
14076f04c:     	movl	%edx, 0x10(%rax,%rcx)
14076f050:     	movss	%xmm10, 0x14(%rax,%rcx)
14076f057:     	movss	%xmm8, 0x18(%rax,%rcx)
14076f05e:     	incq	%rdi
14076f061:     	movq	%rdi, 0x40(%r8)
14076f065:     	movq	0x70(%r8), %rdi
14076f069:     	cmpq	0x60(%r8), %rdi
14076f06d:     	jne	0x14076eae0 <.text+0x76dae0>
14076f073:     	movb	$0x1, 0x257(%rbp)
14076f07a:     	movq	%r15, %rcx
14076f07d:     	leaq	0x230d424(%rip), %rdx   # 0x142a7c4a8
14076f084:     	callq	0x140b81df0 <.text+0xb80df0>
14076f089:     	jmp	0x14076eae0 <.text+0x76dae0>
14076f08e:     	movb	$0x1, 0x254(%rbp)
14076f095:     	movq	$0x68, 0x20(%rsp)
14076f09e:     	movl	$0x8, %r9d
14076f0a4:     	leaq	0x140(%rbp), %rcx
14076f0ab:     	callq	0x1428a7de0 <.text+0x28a6de0>
14076f0b0:     	movq	0x150(%rbp), %rsi
14076f0b7:     	jmp	0x14076e40f <.text+0x76d40f>
14076f0bc:     	movq	0x118(%rbp), %r15
14076f0c3:     	nopw	%cs:(%rax,%rax)
14076f0d0:     	movq	0x58(%rbp), %rax
14076f0d4:     	movl	0x100(%rbp), %ecx
14076f0da:     	movl	%ecx, (%rax)
14076f0dc:     	cmpq	$0x0, 0x18(%r15)
14076f0e1:     	je	0x14076f410 <.text+0x76e410>
14076f0e7:     	movq	%r15, %rsi
14076f0ea:     	movq	0xa0(%rbp), %rcx
14076f0f1:     	leaq	0x228(%rbp), %rdx
14076f0f8:     	callq	0x1405b0d70 <.text+0x5afd70>
14076f0fd:     	movq	(%r15), %r15
14076f100:     	movq	%rsi, 0x118(%rbp)
14076f107:     	movq	0x8(%rsi), %rcx
14076f10b:     	movq	%rcx, %rdx
14076f10e:     	andq	%rax, %rdx
14076f111:     	shrq	$0x39, %rax
14076f115:     	movd	%eax, %xmm0
14076f119:     	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14076f11d:     	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14076f122:     	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14076f127:     	leaq	-0x2a8(%r15), %rax
14076f12e:     	movq	0x228(%rbp), %r11
14076f135:     	xorl	%r8d, %r8d
14076f138:     	movdqu	(%r15,%rdx), %xmm1
14076f13e:     	movdqa	%xmm1, %xmm2
14076f142:     	pcmpeqb	%xmm0, %xmm2
14076f146:     	pmovmskb	%xmm2, %r9d
14076f14b:     	testl	%r9d, %r9d
14076f14e:     	je	0x14076f178 <.text+0x76e178>
14076f150:     	tzcntl	%r9d, %r10d
14076f155:     	addq	%rdx, %r10
14076f158:     	andq	%rcx, %r10
14076f15b:     	negq	%r10
14076f15e:     	imulq	$0x2a8, %r10, %r10      # imm = 0x2A8
14076f165:     	cmpq	(%rax,%r10), %r11
14076f169:     	je	0x14076f19a <.text+0x76e19a>
14076f16b:     	leal	-0x1(%r9), %r10d
14076f16f:     	andw	%r9w, %r10w
14076f173:     	movl	%r10d, %r9d
14076f176:     	jne	0x14076f150 <.text+0x76e150>
14076f178:     	pcmpeqb	%xmm6, %xmm1
14076f17c:     	pmovmskb	%xmm1, %r9d
14076f181:     	testl	%r9d, %r9d
14076f184:     	jne	0x14076f410 <.text+0x76e410>
14076f18a:     	addq	%r8, %rdx
14076f18d:     	addq	$0x10, %rdx
14076f191:     	addq	$0x10, %r8
14076f195:     	andq	%rcx, %rdx
14076f198:     	jmp	0x14076f138 <.text+0x76e138>
14076f19a:     	movq	%r11, 0x230(%rbp)
14076f1a1:     	movq	%r13, 0x128(%rbp)
14076f1a8:     	movq	%r12, 0x120(%rbp)
14076f1af:     	addq	%r10, %r15
14076f1b2:     	movq	0x148(%rbp), %rdi
14076f1b9:     	movq	0x150(%rbp), %rax
14076f1c0:     	movq	%rax, 0x1e8(%rbp)
14076f1c7:     	imulq	$0x68, %rax, %rax
14076f1cb:     	addq	%rdi, %rax
14076f1ce:     	movq	%rax, 0x240(%rbp)
14076f1d5:     	jmp	0x14076f1e7 <.text+0x76e1e7>
14076f1d7:     	nopw	(%rax,%rax)
14076f1e0:     	movq	0x248(%rbp), %rdi
14076f1e7:     	cmpq	0x240(%rbp), %rdi
14076f1ee:     	je	0x14076f2e7 <.text+0x76e2e7>
14076f1f4:     	leaq	0x68(%rdi), %rax
14076f1f8:     	movq	%rax, 0x248(%rbp)
14076f1ff:     	leaq	0x58(%rdi), %rbx
14076f203:     	movl	0x60(%rdi), %eax
14076f206:     	movl	%eax, 0x1f8(%rbp)
14076f20c:     	movq	0x58(%rdi), %rax
14076f210:     	movq	%rax, 0x1f0(%rbp)
14076f217:     	nopw	(%rax,%rax)
14076f220:     	movb	$0x1, 0x257(%rbp)
14076f227:     	leaq	0x180(%rbp), %rcx
14076f22e:     	leaq	0x1f0(%rbp), %rdx
14076f235:     	callq	0x1412002f0 <.text+0x11ff2f0>
14076f23a:     	testb	$0x1, 0x180(%rbp)
14076f241:     	je	0x14076f1e0 <.text+0x76e1e0>
14076f243:     	movl	0x184(%rbp), %r14d
14076f24a:     	movl	0x188(%rbp), %esi
14076f250:     	movb	$0x1, 0x257(%rbp)
14076f257:     	movl	%r14d, %ecx
14076f25a:     	movl	%esi, %edx
14076f25c:     	callq	0x141200280 <.text+0x11ff280>
14076f261:     	movq	%rax, %r12
14076f264:     	movq	-0x1a0(%r15), %rdx
14076f26b:     	cmpq	%rdx, %rax
14076f26e:     	jae	0x14076f762 <.text+0x76e762>
14076f274:     	movq	-0x1a8(%r15), %r13
14076f27b:     	movss	(%r13,%r12,4), %xmm6
14076f282:     	movb	$0x1, 0x257(%rbp)
14076f289:     	movq	%rbx, %rcx
14076f28c:     	callq	0x141200370 <.text+0x11ff370>
14076f291:     	movl	%edx, %r9d
14076f294:     	movb	$0x1, 0x257(%rbp)
14076f29b:     	movl	%r14d, %ecx
14076f29e:     	movl	%esi, %edx
14076f2a0:     	movl	%eax, %r8d
14076f2a3:     	callq	0x1412002e0 <.text+0x11ff2e0>
14076f2a8:     	movq	0x10(%rdi), %rdx
14076f2ac:     	cmpq	%rdx, %rax
14076f2af:     	jae	0x14076f747 <.text+0x76e747>
14076f2b5:     	movq	0x8(%rdi), %rcx
14076f2b9:     	movss	(%rcx,%rax,4), %xmm0
14076f2be:     	addss	%xmm15, %xmm0
14076f2c3:     	movaps	%xmm6, %xmm1
14076f2c6:     	cmpunordss	%xmm6, %xmm1
14076f2cb:     	movaps	%xmm1, %xmm2
14076f2ce:     	andps	%xmm0, %xmm2
14076f2d1:     	maxss	%xmm6, %xmm0
14076f2d5:     	andnps	%xmm0, %xmm1
14076f2d8:     	orps	%xmm2, %xmm1
14076f2db:     	movss	%xmm1, (%r13,%r12,4)
14076f2e2:     	jmp	0x14076f220 <.text+0x76e220>
14076f2e7:     	addq	$-0x118, %r15           # imm = 0xFEE8
14076f2ee:     	movb	$0x1, 0x257(%rbp)
14076f2f5:     	movq	%r15, %rcx
14076f2f8:     	movq	0x1e8(%rbp), %rdx
14076f2ff:     	callq	0x140a885e0 <.text+0xa875e0>
14076f304:     	movb	$0x1, 0x257(%rbp)
14076f30b:     	movq	%r15, %rcx
14076f30e:     	movd	0x23c(%rbp), %xmm1
14076f316:     	callq	0x140a88450 <.text+0xa87450>
14076f31b:     	movl	0x108(%rbp), %eax
14076f321:     	movq	0x80(%rbp), %rcx
14076f328:     	movl	%eax, (%rcx)
14076f32a:     	movq	0x150(%rbp), %rax
14076f331:     	movq	%rax, 0x190(%rbp)
14076f338:     	movdqu	0x140(%rbp), %xmm0
14076f340:     	movdqa	%xmm0, 0x180(%rbp)
14076f348:     	movb	$0x0, 0x257(%rbp)
14076f34f:     	leaq	0x1f0(%rbp), %rcx
14076f356:     	movq	0x78(%rbp), %rdx
14076f35a:     	movq	0x230(%rbp), %r8
14076f361:     	leaq	0x180(%rbp), %r9
14076f368:     	callq	0x1411fbd70 <.text+0x11fad70>
14076f36d:     	xorl	%eax, %eax
14076f36f:     	cmpq	0x1f0(%rbp), %rax
14076f376:     	jo	0x14076f38b <.text+0x76e38b>
14076f378:     	movb	$0x0, 0x257(%rbp)
14076f37f:     	leaq	0x1f0(%rbp), %rcx
14076f386:     	callq	0x1405d3750 <.text+0x5d2750>
14076f38b:     	movq	0x20(%rbp), %rsi
14076f38f:     	addq	$0x18, %rsi
14076f393:     	movq	%rsi, %rcx
14076f396:     	callq	0x140c81e10 <.text+0xc80e10>
14076f39b:     	movq	0x120(%rbp), %r12
14076f3a2:     	movq	0x128(%rbp), %r13
14076f3a9:     	movq	0x118(%rbp), %r15
14076f3b0:     	pcmpeqd	%xmm6, %xmm6
14076f3b4:     	movb	$0x1, %al
14076f3b6:     	xorl	%ecx, %ecx
14076f3b8:     	lock
14076f3b9:     	cmpxchgb	%cl, (%rsi)
14076f3bc:     	je	0x14076dfd3 <.text+0x76cfd3>
14076f3c2:     	movq	%rsi, %rcx
14076f3c5:     	xorl	%edx, %edx
14076f3c7:     	callq	0x1428cdb90 <.text+0x28ccb90>
14076f3cc:     	jmp	0x14076dfd3 <.text+0x76cfd3>
14076f3d1:     	movq	%rcx, %rsi
14076f3d4:     	movl	$0x3b9aca00, %r8d       # imm = 0x3B9ACA00
14076f3da:     	callq	0x1428cd210 <.text+0x28cc210>
14076f3df:     	movq	%rsi, %rcx
14076f3e2:     	jmp	0x14076e0d5 <.text+0x76d0d5>
14076f3e7:     	leaq	0x230f68a(%rip), %rcx   # 0x142a7ea78
14076f3ee:     	callq	0x1428d9310 <.text+0x28d8310>
14076f3f3:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f3f8:     	movq	%r13, 0x220(%rbp)
14076f3ff:     	leaq	0x24a9f42(%rip), %rcx   # 0x142c19348
14076f406:     	callq	0x1428d9310 <.text+0x28d8310>
14076f40b:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f410:     	movb	$0x1, 0x257(%rbp)
14076f417:     	leaq	0x230f7f2(%rip), %rcx   # 0x142a7ec10
14076f41e:     	callq	0x1428d9310 <.text+0x28d8310>
14076f423:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f428:     	movb	$0x1, 0x257(%rbp)
14076f42f:     	leaq	0x230f65a(%rip), %rcx   # 0x142a7ea90
14076f436:     	callq	0x1428d9310 <.text+0x28d8310>
14076f43b:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f440:     	cmpb	$0x0, 0x255(%rbp)
14076f447:     	je	0x14076f4cf <.text+0x76e4cf>
14076f44d:     	movq	0x2ebcf54(%rip), %rax   # 0x14362c3a8
14076f454:     	movq	(%rax), %rax
14076f457:     	testq	%rax, %rax
14076f45a:     	je	0x14076f4cf <.text+0x76e4cf>
14076f45c:     	leaq	0x230f5bd(%rip), %rax   # 0x142a7ea20
14076f463:     	movq	%rax, 0x180(%rbp)
14076f46a:     	movq	$0x1, 0x188(%rbp)
14076f475:     	movq	$0x8, 0x190(%rbp)
14076f480:     	pxor	%xmm0, %xmm0
14076f484:     	movdqu	%xmm0, 0x198(%rbp)
14076f48c:     	leaq	0x230f59d(%rip), %rcx   # 0x142a7ea30
14076f493:     	callq	0x14067dd80 <.text+0x67cd80>
14076f498:     	leaq	0x230f5a9(%rip), %rcx   # 0x142a7ea48
14076f49f:     	movq	%rcx, (%rbp)
14076f4a3:     	movq	$0x2a, 0x8(%rbp)
14076f4ab:     	movq	%rcx, 0x10(%rbp)
14076f4af:     	movq	$0x2a, 0x18(%rbp)
14076f4b7:     	movq	%rax, 0x20(%rbp)
14076f4bb:     	leaq	0x180(%rbp), %rcx
14076f4c2:     	movq	%rbp, %r8
14076f4c5:     	movl	$0x1, %edx
14076f4ca:     	callq	0x1404b1430 <.text+0x4b0430>
14076f4cf:     	movq	0x158(%rbp), %rdx
14076f4d6:     	testq	%rdx, %rdx
14076f4d9:     	je	0x14076f4f1 <.text+0x76e4f1>
14076f4db:     	movq	0x160(%rbp), %rcx
14076f4e2:     	shlq	$0x3, %rdx
14076f4e6:     	movl	$0x8, %r8d
14076f4ec:     	callq	0x140613c20 <.text+0x612c20>
14076f4f1:     	cmpb	$0x0, 0x256(%rbp)
14076f4f8:     	je	0x14076f545 <.text+0x76e545>
14076f4fa:     	callq	0x140c18130 <.text+0xc17130>
14076f4ff:     	movq	%rax, %rcx
14076f502:     	movq	(%rax), %rax
14076f505:     	cmpq	$0x1, %rax
14076f509:     	jne	0x14076f5c1 <.text+0x76e5c1>
14076f50f:     	addq	$0x8, %rcx
14076f513:     	movq	0x170(%rbp), %rdx
14076f51a:     	cmpq	$0x0, (%rcx)
14076f51e:     	jne	0x14076f6ae <.text+0x76e6ae>
14076f524:     	movq	$-0x1, (%rcx)
14076f52b:     	movq	%rcx, 0x248(%rbp)
14076f532:     	addq	$0x8, %rcx
14076f536:     	callq	0x140c170f0 <.text+0xc160f0>
14076f53b:     	movq	0x248(%rbp), %rax
14076f542:     	incq	(%rax)
14076f545:     	movaps	0x260(%rbp), %xmm6
14076f54c:     	movaps	0x270(%rbp), %xmm7
14076f553:     	movaps	0x280(%rbp), %xmm8
14076f55b:     	movaps	0x290(%rbp), %xmm9
14076f563:     	movaps	0x2a0(%rbp), %xmm10
14076f56b:     	movaps	0x2b0(%rbp), %xmm11
14076f573:     	movaps	0x2c0(%rbp), %xmm12
14076f57b:     	movaps	0x2d0(%rbp), %xmm13
14076f583:     	movaps	0x2e0(%rbp), %xmm14
14076f58b:     	movaps	0x2f0(%rbp), %xmm15
14076f593:     	addq	$0x388, %rsp            # imm = 0x388
14076f59a:     	popq	%rbx
14076f59b:     	popq	%rdi
14076f59c:     	popq	%rsi
14076f59d:     	popq	%r12
14076f59f:     	popq	%r13
14076f5a1:     	popq	%r14
14076f5a3:     	popq	%r15
14076f5a5:     	popq	%rbp
14076f5a6:     	retq
14076f5a7:     	testq	%rax, %rax
14076f5aa:     	jne	0x14076f5e0 <.text+0x76e5e0>
14076f5ac:     	xorl	%edx, %edx
14076f5ae:     	callq	0x1428f15c0 <.text+0x28f05c0>
14076f5b3:     	movq	%rax, %rcx
14076f5b6:     	testq	%rax, %rax
14076f5b9:     	jne	0x14076dc4c <.text+0x76cc4c>
14076f5bf:     	jmp	0x14076f5e0 <.text+0x76e5e0>
14076f5c1:     	testq	%rax, %rax
14076f5c4:     	jne	0x14076f5e0 <.text+0x76e5e0>
14076f5c6:     	xorl	%edx, %edx
14076f5c8:     	callq	0x1428f15c0 <.text+0x28f05c0>
14076f5cd:     	movq	0x170(%rbp), %rdx
14076f5d4:     	movq	%rax, %rcx
14076f5d7:     	testq	%rax, %rax
14076f5da:     	jne	0x14076f51a <.text+0x76e51a>
14076f5e0:     	leaq	0x24a9ec1(%rip), %rcx   # 0x142c194a8
14076f5e7:     	callq	0x1428d80d0 <.text+0x28d70d0>
14076f5ec:     	movq	$0x0, 0x180(%rbp)
14076f5f7:     	leaq	0x24ab262(%rip), %rax   # 0x142c1a860
14076f5fe:     	movq	%rax, 0x20(%rsp)
14076f603:     	movq	%rbp, %rdx
14076f606:     	leaq	0x180(%rbp), %r9
14076f60d:     	xorl	%ecx, %ecx
14076f60f:     	movq	%r14, %r8
14076f612:     	callq	0x1428cab41 <.text+0x28c9b41>
14076f617:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f61c:     	cmpq	$0x15, %rsi
14076f620:     	jae	0x14076fa46 <.text+0x76ea46>
14076f626:     	leaq	(%r12,%rsi,8), %rax
14076f62a:     	leaq	0x8(%r12), %rcx
14076f62f:     	movl	$0x8, %edx
14076f634:     	jmp	0x14076f649 <.text+0x76e649>
14076f636:     	movq	%r12, %r9
14076f639:     	movq	%r8, (%r9)
14076f63c:     	addq	$0x8, %rcx
14076f640:     	addq	$0x8, %rdx
14076f644:     	cmpq	%rax, %rcx
14076f647:     	je	0x14076f675 <.text+0x76e675>
14076f649:     	movq	-0x8(%rcx), %r10
14076f64d:     	movq	(%rcx), %r8
14076f650:     	cmpq	%r10, %r8
14076f653:     	jae	0x14076f63c <.text+0x76e63c>
14076f655:     	movq	%rdx, %r9
14076f658:     	movq	%r10, (%r12,%r9)
14076f65c:     	cmpq	$0x8, %r9
14076f660:     	je	0x14076f636 <.text+0x76e636>
14076f662:     	movq	-0x10(%r12,%r9), %r10
14076f667:     	addq	$-0x8, %r9
14076f66b:     	cmpq	%r10, %r8
14076f66e:     	jb	0x14076f658 <.text+0x76e658>
14076f670:     	addq	%r12, %r9
14076f673:     	jmp	0x14076f639 <.text+0x76e639>
14076f675:     	movq	0x160(%rbp), %r12
14076f67c:     	movq	0x168(%rbp), %rsi
14076f683:     	cmpq	$0x2, %rsi
14076f687:     	jb	0x14076df3a <.text+0x76cf3a>
14076f68d:     	movl	$0x1, %eax
14076f692:     	movq	(%r12,%rax,8), %rcx
14076f696:     	cmpq	-0x8(%r12,%rax,8), %rcx
14076f69b:     	je	0x14076f77d <.text+0x76e77d>
14076f6a1:     	incq	%rax
14076f6a4:     	cmpq	%rax, %rsi
14076f6a7:     	jne	0x14076f692 <.text+0x76e692>
14076f6a9:     	jmp	0x14076df3a <.text+0x76cf3a>
14076f6ae:     	leaq	0x24a9e63(%rip), %rcx   # 0x142c19518
14076f6b5:     	callq	0x1428d9250 <.text+0x28d8250>
14076f6ba:     	leaq	0x110(%rbp), %rax
14076f6c1:     	movq	%rax, 0x1f0(%rbp)
14076f6c8:     	leaq	0x61dff1(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
14076f6cf:     	movq	%rax, 0x1f8(%rbp)
14076f6d6:     	leaq	0xe8(%rbp), %rcx
14076f6dd:     	movq	%rcx, 0x200(%rbp)
14076f6e4:     	movq	%rax, 0x208(%rbp)
14076f6eb:     	leaq	0x24a9c06(%rip), %rax   # 0x142c192f8
14076f6f2:     	movq	%rax, 0x180(%rbp)
14076f6f9:     	movq	$0x2, 0x188(%rbp)
14076f704:     	movq	$0x0, 0x1a0(%rbp)
14076f70f:     	leaq	0x1f0(%rbp), %rax
14076f716:     	movq	%rax, 0x190(%rbp)
14076f71d:     	movq	$0x2, 0x198(%rbp)
14076f728:     	movq	%r13, 0x220(%rbp)
14076f72f:     	leaq	0x24a9be2(%rip), %rdx   # 0x142c19318
14076f736:     	leaq	0x180(%rbp), %rcx
14076f73d:     	callq	0x1428d9390 <.text+0x28d8390>
14076f742:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f747:     	movb	$0x1, 0x257(%rbp)
14076f74e:     	leaq	0x230f4eb(%rip), %r8    # 0x142a7ec40
14076f755:     	movq	%rax, %rcx
14076f758:     	callq	0x1428d9518 <.text+0x28d8518>
14076f75d:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f762:     	movb	$0x1, 0x257(%rbp)
14076f769:     	leaq	0x230f4b8(%rip), %r8    # 0x142a7ec28
14076f770:     	movq	%r12, %rcx
14076f773:     	callq	0x1428d9518 <.text+0x28d8518>
14076f778:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f77d:     	leaq	0x1(%rax), %rcx
14076f781:     	jmp	0x14076f786 <.text+0x76e786>
14076f783:     	incq	%rcx
14076f786:     	cmpq	%rsi, %rcx
14076f789:     	jae	0x14076f79f <.text+0x76e79f>
14076f78b:     	movq	(%r12,%rcx,8), %rdx
14076f78f:     	cmpq	-0x8(%r12,%rax,8), %rdx
14076f794:     	je	0x14076f783 <.text+0x76e783>
14076f796:     	movq	%rdx, (%r12,%rax,8)
14076f79a:     	incq	%rax
14076f79d:     	jmp	0x14076f783 <.text+0x76e783>
14076f79f:     	movq	%rax, 0x168(%rbp)
14076f7a6:     	movq	%rax, %rsi
14076f7a9:     	jmp	0x14076df3a <.text+0x76cf3a>
14076f7ae:     	movb	$0x1, 0x257(%rbp)
14076f7b5:     	leaq	0x230f2ec(%rip), %r8    # 0x142a7eaa8
14076f7bc:     	movq	%rdx, %rcx
14076f7bf:     	movq	%rsi, %rdx
14076f7c2:     	callq	0x1428d9830 <.text+0x28d8830>
14076f7c7:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f7cc:     	leaq	0x110(%rbp), %rax
14076f7d3:     	movq	%rax, 0x1f0(%rbp)
14076f7da:     	leaq	0x61dedf(%rip), %rax    # 0x140d8d6c0 <.text+0xd8c6c0>
14076f7e1:     	movq	%rax, 0x1f8(%rbp)
14076f7e8:     	leaq	0xe8(%rbp), %rcx
14076f7ef:     	movq	%rcx, 0x200(%rbp)
14076f7f6:     	movq	%rax, 0x208(%rbp)
14076f7fd:     	leaq	0x24a9af4(%rip), %rax   # 0x142c192f8
14076f804:     	movq	%rax, 0x180(%rbp)
14076f80b:     	movq	$0x2, 0x188(%rbp)
14076f816:     	movq	$0x0, 0x1a0(%rbp)
14076f821:     	leaq	0x1f0(%rbp), %rax
14076f828:     	movq	%rax, 0x190(%rbp)
14076f82f:     	movq	$0x2, 0x198(%rbp)
14076f83a:     	leaq	0x24a9ad7(%rip), %rdx   # 0x142c19318
14076f841:     	leaq	0x180(%rbp), %rcx
14076f848:     	callq	0x1428d9390 <.text+0x28d8390>
14076f84d:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f852:     	leaq	0x230f17f(%rip), %rcx   # 0x142a7e9d8
14076f859:     	callq	0x1428d9310 <.text+0x28d8310>
14076f85e:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f863:     	leaq	0x24ab23e(%rip), %rcx   # 0x142c1aaa8
14076f86a:     	callq	0x1428d9310 <.text+0x28d8310>
14076f86f:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f874:     	leaq	0x230f31d(%rip), %r8    # 0x142a7eb98
14076f87b:     	movl	$0x1, %ecx
14076f880:     	movq	%rdi, %rdx
14076f883:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14076f888:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f88d:     	movq	$0x1, 0x220(%rbp)
14076f898:     	leaq	0x230f311(%rip), %r8    # 0x142a7ebb0
14076f89f:     	xorl	%ecx, %ecx
14076f8a1:     	xorl	%edx, %edx
14076f8a3:     	callq	0x1428d9518 <.text+0x28d8518>
14076f8a8:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f8ad:     	movb	$0x1, 0x257(%rbp)
14076f8b4:     	leaq	0x24b021d(%rip), %r8    # 0x142c1fad8
14076f8bb:     	movl	$0x4, %ecx
14076f8c0:     	movq	%r12, %rdx
14076f8c3:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14076f8c8:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f8cd:     	movb	$0x1, 0x257(%rbp)
14076f8d4:     	leaq	0x24b026d(%rip), %r8    # 0x142c1fb48
14076f8db:     	xorl	%ecx, %ecx
14076f8dd:     	xorl	%edx, %edx
14076f8df:     	callq	0x1428d9518 <.text+0x28d8518>
14076f8e4:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f8e9:     	movb	$0x1, 0x257(%rbp)
14076f8f0:     	leaq	0x24b0239(%rip), %r8    # 0x142c1fb30
14076f8f7:     	xorl	%ecx, %ecx
14076f8f9:     	xorl	%edx, %edx
14076f8fb:     	callq	0x1428d9518 <.text+0x28d8518>
14076f900:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f905:     	movb	$0x1, 0x257(%rbp)
14076f90c:     	leaq	0x230f20d(%rip), %r8    # 0x142a7eb20
14076f913:     	movq	%rax, %rcx
14076f916:     	callq	0x1428d9518 <.text+0x28d8518>
14076f91b:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f920:     	movb	$0x1, 0x257(%rbp)
14076f927:     	leaq	0x230f20a(%rip), %r8    # 0x142a7eb38
14076f92e:     	callq	0x1428d9518 <.text+0x28d8518>
14076f933:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f938:     	movb	$0x1, 0x257(%rbp)
14076f93f:     	leaq	0x230f20a(%rip), %r8    # 0x142a7eb50
14076f946:     	movq	%rax, %rcx
14076f949:     	callq	0x1428d9518 <.text+0x28d8518>
14076f94e:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f953:     	movb	$0x1, 0x257(%rbp)
14076f95a:     	leaq	0x230f207(%rip), %r8    # 0x142a7eb68
14076f961:     	movq	%rax, %rcx
14076f964:     	callq	0x1428d9518 <.text+0x28d8518>
14076f969:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f96e:     	movb	$0x1, 0x257(%rbp)
14076f975:     	leaq	0x230f204(%rip), %r8    # 0x142a7eb80
14076f97c:     	callq	0x1428d9518 <.text+0x28d8518>
14076f981:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f986:     	movb	$0x1, 0x257(%rbp)
14076f98d:     	leaq	0x230f174(%rip), %r8    # 0x142a7eb08
14076f994:     	xorl	%ecx, %ecx
14076f996:     	xorl	%edx, %edx
14076f998:     	callq	0x1428d9518 <.text+0x28d8518>
14076f99d:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f9a2:     	movb	$0x1, 0x257(%rbp)
14076f9a9:     	leaq	0x230f140(%rip), %r8    # 0x142a7eaf0
14076f9b0:     	movl	$0x1, %ecx
14076f9b5:     	movl	$0x1, %edx
14076f9ba:     	callq	0x1428d9518 <.text+0x28d8518>
14076f9bf:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f9c4:     	movb	$0x1, 0x257(%rbp)
14076f9cb:     	leaq	0x230f106(%rip), %r8    # 0x142a7ead8
14076f9d2:     	xorl	%ecx, %ecx
14076f9d4:     	xorl	%edx, %edx
14076f9d6:     	callq	0x1428d9518 <.text+0x28d8518>
14076f9db:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f9e0:     	movb	$0x1, 0x257(%rbp)
14076f9e7:     	leaq	0x230f0d2(%rip), %r8    # 0x142a7eac0
14076f9ee:     	xorl	%ecx, %ecx
14076f9f0:     	xorl	%edx, %edx
14076f9f2:     	callq	0x1428d9518 <.text+0x28d8518>
14076f9f7:     	jmp	0x14076faeb <.text+0x76eaeb>
14076f9fc:     	movb	$0x1, 0x257(%rbp)
14076fa03:     	leaq	0x24b0156(%rip), %r8    # 0x142c1fb60
14076fa0a:     	movq	%r14, %rcx
14076fa0d:     	movq	%r14, %rdx
14076fa10:     	callq	0x1428d9518 <.text+0x28d8518>
14076fa15:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fa1a:     	movq	%r13, 0x220(%rbp)
14076fa21:     	leaq	0x230f1a0(%rip), %r8    # 0x142a7ebc8
14076fa28:     	movq	%rdi, %rdx
14076fa2b:     	callq	0x1428d9518 <.text+0x28d8518>
14076fa30:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fa35:     	leaq	0x24ab084(%rip), %rcx   # 0x142c1aac0
14076fa3c:     	callq	0x1428d9310 <.text+0x28d8310>
14076fa41:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fa46:     	leaq	0x180(%rbp), %r8
14076fa4d:     	movq	%r12, %rcx
14076fa50:     	movq	%rsi, %rdx
14076fa53:     	callq	0x140958d60 <.text+0x957d60>
14076fa58:     	jmp	0x14076f68d <.text+0x76e68d>
14076fa5d:     	leaq	0x230f194(%rip), %r8    # 0x142a7ebf8
14076fa64:     	movq	%rsi, %rdx
14076fa67:     	callq	0x1428d9518 <.text+0x28d8518>
14076fa6c:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fa6e:     	leaq	0x24a98d3(%rip), %rcx   # 0x142c19348
14076fa75:     	callq	0x1428d9310 <.text+0x28d8310>
14076fa7a:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fa7c:     	movq	%r13, 0x220(%rbp)
14076fa83:     	leaq	0x230f156(%rip), %r8    # 0x142a7ebe0
14076fa8a:     	movl	$0x4, %ecx
14076fa8f:     	movq	%rbx, %rdx
14076fa92:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14076fa97:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fa99:     	movb	$0x1, 0x257(%rbp)
14076faa0:     	leaq	0x230ca31(%rip), %rcx   # 0x142a7c4d8
14076faa7:     	callq	0x1428d9310 <.text+0x28d8310>
14076faac:     	jmp	0x14076faeb <.text+0x76eaeb>
14076faae:     	movb	$0x1, 0x257(%rbp)
14076fab5:     	leaq	0x230ca04(%rip), %rcx   # 0x142a7c4c0
14076fabc:     	callq	0x1428d9310 <.text+0x28d8310>
14076fac1:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fac3:     	movb	$0x1, 0x257(%rbp)
14076faca:     	leaq	0x230ca07(%rip), %rcx   # 0x142a7c4d8
14076fad1:     	callq	0x1428d9310 <.text+0x28d8310>
14076fad6:     	jmp	0x14076faeb <.text+0x76eaeb>
14076fad8:     	movb	$0x1, 0x257(%rbp)
14076fadf:     	leaq	0x230c9da(%rip), %rcx   # 0x142a7c4c0
14076fae6:     	callq	0x1428d9310 <.text+0x28d8310>
14076faeb:     	ud2
14076faed:     	nopl	(%rax)
14076faf0:     	movq	%rdx, 0x10(%rsp)
14076faf5:     	pushq	%rbp
14076faf6:     	pushq	%r15
14076faf8:     	pushq	%r14
14076fafa:     	pushq	%r13
14076fafc:     	pushq	%r12
14076fafe:     	pushq	%rsi
14076faff:     	pushq	%rdi
14076fb00:     	pushq	%rbx
14076fb01:     	subq	$0x118, %rsp            # imm = 0x118
14076fb08:     	leaq	0x80(%rdx), %rbp
14076fb0f:     	movaps	%xmm15, 0x70(%rsp)
14076fb15:     	movaps	%xmm14, 0x80(%rsp)
14076fb1e:     	movaps	%xmm13, 0x90(%rsp)
14076fb27:     	movaps	%xmm12, 0xa0(%rsp)
14076fb30:     	movaps	%xmm11, 0xb0(%rsp)
14076fb39:     	movaps	%xmm10, 0xc0(%rsp)
14076fb42:     	movaps	%xmm9, 0xd0(%rsp)
14076fb4b:     	movaps	%xmm8, 0xe0(%rsp)
14076fb54:     	movaps	%xmm7, 0xf0(%rsp)
14076fb5c:     	movdqa	%xmm6, 0x100(%rsp)
14076fb65:     	movq	0x240(%rbp), %rax
14076fb6c:     	incq	(%rax)
14076fb6f:     	movaps	0x100(%rsp), %xmm6
14076fb77:     	movaps	0xf0(%rsp), %xmm7
14076fb7f:     	movaps	0xe0(%rsp), %xmm8
14076fb88:     	movaps	0xd0(%rsp), %xmm9
14076fb91:     	movaps	0xc0(%rsp), %xmm10
14076fb9a:     	movaps	0xb0(%rsp), %xmm11
14076fba3:     	movaps	0xa0(%rsp), %xmm12
14076fbac:     	movaps	0x90(%rsp), %xmm13
14076fbb5:     	movaps	0x80(%rsp), %xmm14
14076fbbe:     	movaps	0x70(%rsp), %xmm15
14076fbc4:     	addq	$0x118, %rsp            # imm = 0x118
14076fbcb:     	popq	%rbx
14076fbcc:     	popq	%rdi
14076fbcd:     	popq	%rsi
14076fbce:     	popq	%r12
14076fbd0:     	popq	%r13
14076fbd2:     	popq	%r14
14076fbd4:     	popq	%r15
14076fbd6:     	popq	%rbp
14076fbd7:     	retq
14076fbd8:     	nopl	(%rax,%rax)
14076fbe0:     	movq	%rdx, 0x10(%rsp)
14076fbe5:     	pushq	%rbp
14076fbe6:     	pushq	%r15
14076fbe8:     	pushq	%r14
14076fbea:     	pushq	%r13
14076fbec:     	pushq	%r12
14076fbee:     	pushq	%rsi
14076fbef:     	pushq	%rdi
14076fbf0:     	pushq	%rbx
14076fbf1:     	subq	$0x118, %rsp            # imm = 0x118
14076fbf8:     	leaq	0x80(%rdx), %rbp
14076fbff:     	movaps	%xmm15, 0x70(%rsp)
14076fc05:     	movaps	%xmm14, 0x80(%rsp)
14076fc0e:     	movaps	%xmm13, 0x90(%rsp)
14076fc17:     	movaps	%xmm12, 0xa0(%rsp)
14076fc20:     	movaps	%xmm11, 0xb0(%rsp)
14076fc29:     	movaps	%xmm10, 0xc0(%rsp)
14076fc32:     	movaps	%xmm9, 0xd0(%rsp)
14076fc3b:     	movaps	%xmm8, 0xe0(%rsp)
14076fc44:     	movaps	%xmm7, 0xf0(%rsp)
14076fc4c:     	movdqa	%xmm6, 0x100(%rsp)
14076fc55:     	cmpb	$0x0, 0x256(%rbp)
14076fc5c:     	je	0x14076fc9b <.text+0x76ec9b>
14076fc5e:     	callq	0x140c18130 <.text+0xc17130>
14076fc63:     	movq	%rax, %rsi
14076fc66:     	movq	(%rax), %rax
14076fc69:     	cmpq	$0x1, %rax
14076fc6d:     	jne	0x14076fd04 <.text+0x76ed04>
14076fc73:     	addq	$0x8, %rsi
14076fc77:     	movq	0x170(%rbp), %rdx
14076fc7e:     	cmpq	$0x0, (%rsi)
14076fc82:     	jne	0x14076fd32 <.text+0x76ed32>
14076fc88:     	movq	$-0x1, (%rsi)
14076fc8f:     	leaq	0x8(%rsi), %rcx
14076fc93:     	callq	0x140c170f0 <.text+0xc160f0>
14076fc98:     	incq	(%rsi)
14076fc9b:     	movaps	0x100(%rsp), %xmm6
14076fca3:     	movaps	0xf0(%rsp), %xmm7
14076fcab:     	movaps	0xe0(%rsp), %xmm8
14076fcb4:     	movaps	0xd0(%rsp), %xmm9
14076fcbd:     	movaps	0xc0(%rsp), %xmm10
14076fcc6:     	movaps	0xb0(%rsp), %xmm11
14076fccf:     	movaps	0xa0(%rsp), %xmm12
14076fcd8:     	movaps	0x90(%rsp), %xmm13
14076fce1:     	movaps	0x80(%rsp), %xmm14
14076fcea:     	movaps	0x70(%rsp), %xmm15
14076fcf0:     	addq	$0x118, %rsp            # imm = 0x118
14076fcf7:     	popq	%rbx
14076fcf8:     	popq	%rdi
14076fcf9:     	popq	%rsi
14076fcfa:     	popq	%r12
14076fcfc:     	popq	%r13
14076fcfe:     	popq	%r14
14076fd00:     	popq	%r15
14076fd02:     	popq	%rbp
14076fd03:     	retq
14076fd04:     	testq	%rax, %rax
14076fd07:     	jne	0x14076fd26 <.text+0x76ed26>
14076fd09:     	movq	%rsi, %rcx
14076fd0c:     	xorl	%edx, %edx
14076fd0e:     	callq	0x1428f15c0 <.text+0x28f05c0>
14076fd13:     	movq	0x170(%rbp), %rdx
14076fd1a:     	movq	%rax, %rsi
14076fd1d:     	testq	%rax, %rax
14076fd20:     	jne	0x14076fc7e <.text+0x76ec7e>
14076fd26:     	leaq	0x24a977b(%rip), %rcx   # 0x142c194a8
14076fd2d:     	callq	0x1428d80d0 <.text+0x28d70d0>
14076fd32:     	leaq	0x24a97df(%rip), %rcx   # 0x142c19518
14076fd39:     	callq	0x1428d9250 <.text+0x28d8250>
14076fd3e:     	int3
14076fd3f:     	nop
14076fd40:     	movq	%rdx, 0x10(%rsp)
14076fd45:     	pushq	%rbp
14076fd46:     	pushq	%r15
14076fd48:     	pushq	%r14
14076fd4a:     	pushq	%r13
14076fd4c:     	pushq	%r12
14076fd4e:     	pushq	%rsi
14076fd4f:     	pushq	%rdi
14076fd50:     	pushq	%rbx
14076fd51:     	subq	$0x118, %rsp            # imm = 0x118
14076fd58:     	leaq	0x80(%rdx), %rbp
14076fd5f:     	movaps	%xmm15, 0x70(%rsp)
14076fd65:     	movaps	%xmm14, 0x80(%rsp)
14076fd6e:     	movaps	%xmm13, 0x90(%rsp)
14076fd77:     	movaps	%xmm12, 0xa0(%rsp)
14076fd80:     	movaps	%xmm11, 0xb0(%rsp)
14076fd89:     	movaps	%xmm10, 0xc0(%rsp)
14076fd92:     	movaps	%xmm9, 0xd0(%rsp)
14076fd9b:     	movaps	%xmm8, 0xe0(%rsp)
14076fda4:     	movaps	%xmm7, 0xf0(%rsp)
14076fdac:     	movdqa	%xmm6, 0x100(%rsp)
14076fdb5:     	movq	0x158(%rbp), %rdx
14076fdbc:     	testq	%rdx, %rdx
14076fdbf:     	je	0x14076fdd7 <.text+0x76edd7>
14076fdc1:     	movq	0x160(%rbp), %rcx
14076fdc8:     	shlq	$0x3, %rdx
14076fdcc:     	movl	$0x8, %r8d
14076fdd2:     	callq	0x140613c20 <.text+0x612c20>
14076fdd7:     	movdqa	0x100(%rsp), %xmm6
14076fde0:     	movaps	0xf0(%rsp), %xmm7
14076fde8:     	movaps	0xe0(%rsp), %xmm8
14076fdf1:     	movaps	0xd0(%rsp), %xmm9
14076fdfa:     	movaps	0xc0(%rsp), %xmm10
14076fe03:     	movaps	0xb0(%rsp), %xmm11
14076fe0c:     	movaps	0xa0(%rsp), %xmm12
14076fe15:     	movaps	0x90(%rsp), %xmm13
14076fe1e:     	movaps	0x80(%rsp), %xmm14
14076fe27:     	movaps	0x70(%rsp), %xmm15
14076fe2d:     	addq	$0x118, %rsp            # imm = 0x118
14076fe34:     	popq	%rbx
14076fe35:     	popq	%rdi
14076fe36:     	popq	%rsi
14076fe37:     	popq	%r12
14076fe39:     	popq	%r13
14076fe3b:     	popq	%r14
14076fe3d:     	popq	%r15
14076fe3f:     	popq	%rbp
14076fe40:     	retq
14076fe41:     	nopw	%cs:(%rax,%rax)
14076fe50:     	movq	%rdx, 0x10(%rsp)
14076fe55:     	pushq	%rbp
14076fe56:     	pushq	%r15
14076fe58:     	pushq	%r14
14076fe5a:     	pushq	%r13
14076fe5c:     	pushq	%r12
14076fe5e:     	pushq	%rsi
14076fe5f:     	pushq	%rdi
14076fe60:     	pushq	%rbx
14076fe61:     	subq	$0x118, %rsp            # imm = 0x118
14076fe68:     	leaq	0x80(%rdx), %rbp
14076fe6f:     	movaps	%xmm15, 0x70(%rsp)
14076fe75:     	movaps	%xmm14, 0x80(%rsp)
14076fe7e:     	movaps	%xmm13, 0x90(%rsp)
14076fe87:     	movaps	%xmm12, 0xa0(%rsp)
14076fe90:     	movaps	%xmm11, 0xb0(%rsp)
14076fe99:     	movaps	%xmm10, 0xc0(%rsp)
14076fea2:     	movaps	%xmm9, 0xd0(%rsp)
14076feab:     	movaps	%xmm8, 0xe0(%rsp)
14076feb4:     	movaps	%xmm7, 0xf0(%rsp)
14076febc:     	movdqa	%xmm6, 0x100(%rsp)
14076fec5:     	movq	0x248(%rbp), %rax
14076fecc:     	incq	(%rax)
14076fecf:     	movaps	0x100(%rsp), %xmm6
14076fed7:     	movaps	0xf0(%rsp), %xmm7
14076fedf:     	movaps	0xe0(%rsp), %xmm8
14076fee8:     	movaps	0xd0(%rsp), %xmm9
14076fef1:     	movaps	0xc0(%rsp), %xmm10
14076fefa:     	movaps	0xb0(%rsp), %xmm11
14076ff03:     	movaps	0xa0(%rsp), %xmm12
14076ff0c:     	movaps	0x90(%rsp), %xmm13
14076ff15:     	movaps	0x80(%rsp), %xmm14
14076ff1e:     	movaps	0x70(%rsp), %xmm15
14076ff24:     	addq	$0x118, %rsp            # imm = 0x118
14076ff2b:     	popq	%rbx
14076ff2c:     	popq	%rdi
14076ff2d:     	popq	%rsi
14076ff2e:     	popq	%r12
14076ff30:     	popq	%r13
14076ff32:     	popq	%r14
14076ff34:     	popq	%r15
14076ff36:     	popq	%rbp
14076ff37:     	retq
14076ff38:     	nopl	(%rax,%rax)
14076ff40:     	movq	%rdx, 0x10(%rsp)
14076ff45:     	pushq	%rbp
14076ff46:     	pushq	%r15
14076ff48:     	pushq	%r14
14076ff4a:     	pushq	%r13
14076ff4c:     	pushq	%r12
14076ff4e:     	pushq	%rsi
14076ff4f:     	pushq	%rdi
14076ff50:     	pushq	%rbx
14076ff51:     	subq	$0x118, %rsp            # imm = 0x118
14076ff58:     	leaq	0x80(%rdx), %rbp
14076ff5f:     	movaps	%xmm15, 0x70(%rsp)
14076ff65:     	movaps	%xmm14, 0x80(%rsp)
14076ff6e:     	movaps	%xmm13, 0x90(%rsp)
14076ff77:     	movaps	%xmm12, 0xa0(%rsp)
14076ff80:     	movaps	%xmm11, 0xb0(%rsp)
14076ff89:     	movaps	%xmm10, 0xc0(%rsp)
14076ff92:     	movaps	%xmm9, 0xd0(%rsp)
14076ff9b:     	movaps	%xmm8, 0xe0(%rsp)
14076ffa4:     	movaps	%xmm7, 0xf0(%rsp)
14076ffac:     	movaps	%xmm6, 0x100(%rsp)
14076ffb4:     	movq	0x20(%rbp), %rdi
14076ffb8:     	leaq	0x18(%rdi), %rsi
14076ffbc:     	movq	%rsi, %rcx
14076ffbf:     	callq	0x140c81e10 <.text+0xc80e10>
14076ffc4:     	xorl	%ecx, %ecx
14076ffc6:     	movb	$0x1, %al
14076ffc8:     	lock
14076ffc9:     	cmpxchgb	%cl, 0x18(%rdi)
14076ffcd:     	jne	0x140770038 <.text+0x76f038>
14076ffcf:     	movaps	0x100(%rsp), %xmm6
14076ffd7:     	movaps	0xf0(%rsp), %xmm7
14076ffdf:     	movaps	0xe0(%rsp), %xmm8
14076ffe8:     	movaps	0xd0(%rsp), %xmm9
14076fff1:     	movaps	0xc0(%rsp), %xmm10
14076fffa:     	movaps	0xb0(%rsp), %xmm11
140770003:     	movaps	0xa0(%rsp), %xmm12
14077000c:     	movaps	0x90(%rsp), %xmm13
140770015:     	movaps	0x80(%rsp), %xmm14
14077001e:     	movaps	0x70(%rsp), %xmm15
140770024:     	addq	$0x118, %rsp            # imm = 0x118
14077002b:     	popq	%rbx
14077002c:     	popq	%rdi
14077002d:     	popq	%rsi
14077002e:     	popq	%r12
140770030:     	popq	%r13
140770032:     	popq	%r14
140770034:     	popq	%r15
140770036:     	popq	%rbp
140770037:     	retq
140770038:     	movq	%rsi, %rcx
14077003b:     	xorl	%edx, %edx
14077003d:     	callq	0x1428cdb90 <.text+0x28ccb90>
140770042:     	jmp	0x14076ffcf <.text+0x76efcf>
140770044:     	nopw	%cs:(%rax,%rax)
140770050:     	movq	%rdx, 0x10(%rsp)
140770055:     	pushq	%rbp
140770056:     	pushq	%r15
140770058:     	pushq	%r14
14077005a:     	pushq	%r13
14077005c:     	pushq	%r12
14077005e:     	pushq	%rsi
14077005f:     	pushq	%rdi
140770060:     	pushq	%rbx
140770061:     	subq	$0x118, %rsp            # imm = 0x118
140770068:     	leaq	0x80(%rdx), %rbp
14077006f:     	movaps	%xmm15, 0x70(%rsp)
140770075:     	movaps	%xmm14, 0x80(%rsp)
14077007e:     	movaps	%xmm13, 0x90(%rsp)
140770087:     	movaps	%xmm12, 0xa0(%rsp)
140770090:     	movaps	%xmm11, 0xb0(%rsp)
140770099:     	movaps	%xmm10, 0xc0(%rsp)
1407700a2:     	movaps	%xmm9, 0xd0(%rsp)
1407700ab:     	movaps	%xmm8, 0xe0(%rsp)
1407700b4:     	movaps	%xmm7, 0xf0(%rsp)
1407700bc:     	movdqa	%xmm6, 0x100(%rsp)
1407700c5:     	cmpb	$0x0, 0x254(%rbp)
1407700cc:     	je	0x1407700da <.text+0x76f0da>
1407700ce:     	leaq	0xc0(%rbp), %rcx
1407700d5:     	callq	0x1411fd610 <.text+0x11fc610>
1407700da:     	movaps	0x100(%rsp), %xmm6
1407700e2:     	movaps	0xf0(%rsp), %xmm7
1407700ea:     	movaps	0xe0(%rsp), %xmm8
1407700f3:     	movaps	0xd0(%rsp), %xmm9
1407700fc:     	movaps	0xc0(%rsp), %xmm10
140770105:     	movaps	0xb0(%rsp), %xmm11
14077010e:     	movaps	0xa0(%rsp), %xmm12
140770117:     	movaps	0x90(%rsp), %xmm13
140770120:     	movaps	0x80(%rsp), %xmm14
140770129:     	movaps	0x70(%rsp), %xmm15
14077012f:     	addq	$0x118, %rsp            # imm = 0x118
140770136:     	popq	%rbx
140770137:     	popq	%rdi
140770138:     	popq	%rsi
140770139:     	popq	%r12
14077013b:     	popq	%r13
14077013d:     	popq	%r14
14077013f:     	popq	%r15
140770141:     	popq	%rbp
140770142:     	retq
140770143:     	nopw	%cs:(%rax,%rax)
140770150:     	movq	%rdx, 0x10(%rsp)
140770155:     	pushq	%rbp
140770156:     	pushq	%r15
140770158:     	pushq	%r14
14077015a:     	pushq	%r13
14077015c:     	pushq	%r12
14077015e:     	pushq	%rsi
14077015f:     	pushq	%rdi
140770160:     	pushq	%rbx
140770161:     	subq	$0x118, %rsp            # imm = 0x118
140770168:     	leaq	0x80(%rdx), %rbp
14077016f:     	movaps	%xmm15, 0x70(%rsp)
140770175:     	movaps	%xmm14, 0x80(%rsp)
14077017e:     	movaps	%xmm13, 0x90(%rsp)
140770187:     	movaps	%xmm12, 0xa0(%rsp)
140770190:     	movaps	%xmm11, 0xb0(%rsp)
140770199:     	movaps	%xmm10, 0xc0(%rsp)
1407701a2:     	movaps	%xmm9, 0xd0(%rsp)
1407701ab:     	movaps	%xmm8, 0xe0(%rsp)
1407701b4:     	movaps	%xmm7, 0xf0(%rsp)
1407701bc:     	movdqa	%xmm6, 0x100(%rsp)
1407701c5:     	movq	0xc0(%rbp), %rax
1407701cc:     	testq	%rax, %rax
1407701cf:     	je	0x1407701eb <.text+0x76f1eb>
1407701d1:     	movq	0xc8(%rbp), %rcx
1407701d8:     	shlq	$0x2, %rax
1407701dc:     	leaq	(%rax,%rax,2), %rdx
1407701e0:     	movl	$0x4, %r8d
1407701e6:     	callq	0x140613c20 <.text+0x612c20>
1407701eb:     	movb	$0x1, 0x257(%rbp)
1407701f2:     	movdqa	0x100(%rsp), %xmm6
1407701fb:     	movaps	0xf0(%rsp), %xmm7
140770203:     	movaps	0xe0(%rsp), %xmm8
14077020c:     	movaps	0xd0(%rsp), %xmm9
140770215:     	movaps	0xc0(%rsp), %xmm10
14077021e:     	movaps	0xb0(%rsp), %xmm11
140770227:     	movaps	0xa0(%rsp), %xmm12
140770230:     	movaps	0x90(%rsp), %xmm13
140770239:     	movaps	0x80(%rsp), %xmm14
140770242:     	movaps	0x70(%rsp), %xmm15
140770248:     	addq	$0x118, %rsp            # imm = 0x118
14077024f:     	popq	%rbx
140770250:     	popq	%rdi
140770251:     	popq	%rsi
140770252:     	popq	%r12
140770254:     	popq	%r13
140770256:     	popq	%r14
140770258:     	popq	%r15
14077025a:     	popq	%rbp
14077025b:     	retq
14077025c:     	nopl	(%rax)
140770260:     	movq	%rdx, 0x10(%rsp)
140770265:     	pushq	%rbp
140770266:     	pushq	%r15
140770268:     	pushq	%r14
14077026a:     	pushq	%r13
14077026c:     	pushq	%r12
14077026e:     	pushq	%rsi
14077026f:     	pushq	%rdi
140770270:     	pushq	%rbx
140770271:     	subq	$0x118, %rsp            # imm = 0x118
140770278:     	leaq	0x80(%rdx), %rbp
14077027f:     	movaps	%xmm15, 0x70(%rsp)
140770285:     	movaps	%xmm14, 0x80(%rsp)
14077028e:     	movaps	%xmm13, 0x90(%rsp)
140770297:     	movaps	%xmm12, 0xa0(%rsp)
1407702a0:     	movaps	%xmm11, 0xb0(%rsp)
1407702a9:     	movaps	%xmm10, 0xc0(%rsp)
1407702b2:     	movaps	%xmm9, 0xd0(%rsp)
1407702bb:     	movaps	%xmm8, 0xe0(%rsp)
1407702c4:     	movaps	%xmm7, 0xf0(%rsp)
1407702cc:     	movdqa	%xmm6, 0x100(%rsp)
1407702d5:     	movq	0x230(%rbp), %rdx
1407702dc:     	testq	%rdx, %rdx
1407702df:     	je	0x1407702f3 <.text+0x76f2f3>
1407702e1:     	movq	0x220(%rbp), %rcx
1407702e8:     	movl	$0x1, %r8d
1407702ee:     	callq	0x140613c20 <.text+0x612c20>
1407702f3:     	movdqa	0x100(%rsp), %xmm6
1407702fc:     	movaps	0xf0(%rsp), %xmm7
140770304:     	movaps	0xe0(%rsp), %xmm8
14077030d:     	movaps	0xd0(%rsp), %xmm9
140770316:     	movaps	0xc0(%rsp), %xmm10
14077031f:     	movaps	0xb0(%rsp), %xmm11
140770328:     	movaps	0xa0(%rsp), %xmm12
140770331:     	movaps	0x90(%rsp), %xmm13
14077033a:     	movaps	0x80(%rsp), %xmm14
140770343:     	movaps	0x70(%rsp), %xmm15
140770349:     	addq	$0x118, %rsp            # imm = 0x118
140770350:     	popq	%rbx
140770351:     	popq	%rdi
140770352:     	popq	%rsi
140770353:     	popq	%r12
140770355:     	popq	%r13
140770357:     	popq	%r14
140770359:     	popq	%r15
14077035b:     	popq	%rbp
14077035c:     	retq
14077035d:     	nopl	(%rax)
140770360:     	movq	%rdx, 0x10(%rsp)
140770365:     	pushq	%rbp
140770366:     	pushq	%r15
140770368:     	pushq	%r14
14077036a:     	pushq	%r13
14077036c:     	pushq	%r12
14077036e:     	pushq	%rsi
14077036f:     	pushq	%rdi
140770370:     	pushq	%rbx
140770371:     	subq	$0x118, %rsp            # imm = 0x118
140770378:     	leaq	0x80(%rdx), %rbp
14077037f:     	movaps	%xmm15, 0x70(%rsp)
140770385:     	movaps	%xmm14, 0x80(%rsp)
14077038e:     	movaps	%xmm13, 0x90(%rsp)
140770397:     	movaps	%xmm12, 0xa0(%rsp)
1407703a0:     	movaps	%xmm11, 0xb0(%rsp)
1407703a9:     	movaps	%xmm10, 0xc0(%rsp)
1407703b2:     	movaps	%xmm9, 0xd0(%rsp)
1407703bb:     	movaps	%xmm8, 0xe0(%rsp)
1407703c4:     	movaps	%xmm7, 0xf0(%rsp)
1407703cc:     	movdqa	%xmm6, 0x100(%rsp)
1407703d5:     	movq	0x178(%rbp), %rdx
1407703dc:     	testq	%rdx, %rdx
1407703df:     	je	0x1407703f7 <.text+0x76f3f7>
1407703e1:     	shlq	$0x3, %rdx
1407703e5:     	movl	$0x4, %r8d
1407703eb:     	movq	0xb8(%rbp), %rcx
1407703f2:     	callq	0x140613c20 <.text+0x612c20>
1407703f7:     	movq	0xb0(%rbp), %rax
1407703fe:     	movq	%rax, 0x220(%rbp)
140770405:     	movdqa	0x100(%rsp), %xmm6
14077040e:     	movaps	0xf0(%rsp), %xmm7
140770416:     	movaps	0xe0(%rsp), %xmm8
14077041f:     	movaps	0xd0(%rsp), %xmm9
140770428:     	movaps	0xc0(%rsp), %xmm10
140770431:     	movaps	0xb0(%rsp), %xmm11
14077043a:     	movaps	0xa0(%rsp), %xmm12
140770443:     	movaps	0x90(%rsp), %xmm13
14077044c:     	movaps	0x80(%rsp), %xmm14
140770455:     	movaps	0x70(%rsp), %xmm15
14077045b:     	addq	$0x118, %rsp            # imm = 0x118
140770462:     	popq	%rbx
140770463:     	popq	%rdi
140770464:     	popq	%rsi
140770465:     	popq	%r12
140770467:     	popq	%r13
140770469:     	popq	%r14
14077046b:     	popq	%r15
14077046d:     	popq	%rbp
14077046e:     	retq
14077046f:     	nop
140770470:     	movq	%rdx, 0x10(%rsp)
140770475:     	pushq	%rbp
140770476:     	pushq	%r15
140770478:     	pushq	%r14
14077047a:     	pushq	%r13
14077047c:     	pushq	%r12
14077047e:     	pushq	%rsi
14077047f:     	pushq	%rdi
140770480:     	pushq	%rbx
140770481:     	subq	$0x118, %rsp            # imm = 0x118
140770488:     	leaq	0x80(%rdx), %rbp
14077048f:     	movaps	%xmm15, 0x70(%rsp)
140770495:     	movaps	%xmm14, 0x80(%rsp)
14077049e:     	movaps	%xmm13, 0x90(%rsp)
1407704a7:     	movaps	%xmm12, 0xa0(%rsp)
1407704b0:     	movaps	%xmm11, 0xb0(%rsp)
1407704b9:     	movaps	%xmm10, 0xc0(%rsp)
1407704c2:     	movaps	%xmm9, 0xd0(%rsp)
1407704cb:     	movaps	%xmm8, 0xe0(%rsp)
1407704d4:     	movaps	%xmm7, 0xf0(%rsp)
1407704dc:     	movdqa	%xmm6, 0x100(%rsp)
1407704e5:     	cmpb	$0x0, 0x257(%rbp)
1407704ec:     	je	0x140770563 <.text+0x76f563>
1407704ee:     	leaq	0x140(%rbp), %rcx
1407704f5:     	callq	0x1405d3750 <.text+0x5d2750>
1407704fa:     	movaps	0x100(%rsp), %xmm6
140770502:     	movaps	0xf0(%rsp), %xmm7
14077050a:     	movaps	0xe0(%rsp), %xmm8
140770513:     	movaps	0xd0(%rsp), %xmm9
14077051c:     	movaps	0xc0(%rsp), %xmm10
140770525:     	movaps	0xb0(%rsp), %xmm11
14077052e:     	movaps	0xa0(%rsp), %xmm12
140770537:     	movaps	0x90(%rsp), %xmm13
140770540:     	movaps	0x80(%rsp), %xmm14
140770549:     	movaps	0x70(%rsp), %xmm15
14077054f:     	addq	$0x118, %rsp            # imm = 0x118
140770556:     	popq	%rbx
140770557:     	popq	%rdi
140770558:     	popq	%rsi
140770559:     	popq	%r12
14077055b:     	popq	%r13
14077055d:     	popq	%r14
14077055f:     	popq	%r15
140770561:     	popq	%rbp
140770562:     	retq
140770563:     	movaps	0x100(%rsp), %xmm6
14077056b:     	movaps	0xf0(%rsp), %xmm7
140770573:     	movaps	0xe0(%rsp), %xmm8
14077057c:     	movaps	0xd0(%rsp), %xmm9
140770585:     	movaps	0xc0(%rsp), %xmm10
14077058e:     	movaps	0xb0(%rsp), %xmm11
140770597:     	movaps	0xa0(%rsp), %xmm12
1407705a0:     	movaps	0x90(%rsp), %xmm13
1407705a9:     	movaps	0x80(%rsp), %xmm14
1407705b2:     	movaps	0x70(%rsp), %xmm15
1407705b8:     	addq	$0x118, %rsp            # imm = 0x118
1407705bf:     	popq	%rbx
1407705c0:     	popq	%rdi
1407705c1:     	popq	%rsi
1407705c2:     	popq	%r12
1407705c4:     	popq	%r13
1407705c6:     	popq	%r14
1407705c8:     	popq	%r15
1407705ca:     	popq	%rbp
1407705cb:     	retq
1407705cc:     	nopl	(%rax)
1407705d0:     	movq	%rdx, 0x10(%rsp)
1407705d5:     	pushq	%rbp
1407705d6:     	pushq	%r15
1407705d8:     	pushq	%r14
1407705da:     	pushq	%r13
1407705dc:     	pushq	%r12
1407705de:     	pushq	%rsi
1407705df:     	pushq	%rdi
1407705e0:     	pushq	%rbx
1407705e1:     	subq	$0x118, %rsp            # imm = 0x118
1407705e8:     	leaq	0x80(%rdx), %rbp
1407705ef:     	movaps	%xmm15, 0x70(%rsp)
1407705f5:     	movaps	%xmm14, 0x80(%rsp)
1407705fe:     	movaps	%xmm13, 0x90(%rsp)
140770607:     	movaps	%xmm12, 0xa0(%rsp)
140770610:     	movaps	%xmm11, 0xb0(%rsp)
140770619:     	movaps	%xmm10, 0xc0(%rsp)
140770622:     	movaps	%xmm9, 0xd0(%rsp)
14077062b:     	movaps	%xmm8, 0xe0(%rsp)
140770634:     	movaps	%xmm7, 0xf0(%rsp)
14077063c:     	movaps	%xmm6, 0x100(%rsp)
140770644:     	leaq	0xe8(%rbp), %rcx
14077064b:     	callq	0x1405d3750 <.text+0x5d2750>
140770650:     	movb	$0x1, 0x257(%rbp)
140770657:     	movdqa	0x100(%rsp), %xmm6
140770660:     	movaps	0xf0(%rsp), %xmm7
140770668:     	movaps	0xe0(%rsp), %xmm8
140770671:     	movaps	0xd0(%rsp), %xmm9
14077067a:     	movaps	0xc0(%rsp), %xmm10
140770683:     	movaps	0xb0(%rsp), %xmm11
14077068c:     	movaps	0xa0(%rsp), %xmm12
140770695:     	movaps	0x90(%rsp), %xmm13
14077069e:     	movaps	0x80(%rsp), %xmm14
1407706a7:     	movaps	0x70(%rsp), %xmm15
1407706ad:     	addq	$0x118, %rsp            # imm = 0x118
1407706b4:     	popq	%rbx
1407706b5:     	popq	%rdi
1407706b6:     	popq	%rsi
1407706b7:     	popq	%r12
1407706b9:     	popq	%r13
1407706bb:     	popq	%r14
1407706bd:     	popq	%r15
1407706bf:     	popq	%rbp
1407706c0:     	retq
