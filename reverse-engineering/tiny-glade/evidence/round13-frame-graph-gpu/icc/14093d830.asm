
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14093d830:     	pushq	%rbp
14093d831:     	pushq	%r15
14093d833:     	pushq	%r14
14093d835:     	pushq	%r13
14093d837:     	pushq	%r12
14093d839:     	pushq	%rsi
14093d83a:     	pushq	%rdi
14093d83b:     	pushq	%rbx
14093d83c:     	subq	$0x218, %rsp            # imm = 0x218
14093d843:     	leaq	0x80(%rsp), %rbp
14093d84b:     	movq	$-0x2, 0x190(%rbp)
14093d856:     	movq	(%r8), %rax
14093d859:     	movq	0x8(%r8), %r8
14093d85d:     	movq	(%rax), %r10
14093d860:     	xorl	%r9d, %r9d
14093d863:     	movq	%r10, %rsi
14093d866:     	subq	0x18(%r8), %rsi
14093d86a:     	cmovbq	%r9, %rsi
14093d86e:     	subq	0x38(%r8), %r10
14093d872:     	cmovbq	%r9, %r10
14093d876:     	leaq	(%rsi,%rsi,4), %rbx
14093d87a:     	shlq	$0x3, %rbx
14093d87e:     	addq	0x8(%r8), %rbx
14093d882:     	leaq	(%r10,%r10,4), %r11
14093d886:     	shlq	$0x3, %r11
14093d88a:     	addq	0x28(%r8), %r11
14093d88e:     	movq	0x10(%r8), %r14
14093d892:     	subq	%rsi, %r14
14093d895:     	cmovbq	%r9, %r14
14093d899:     	movq	0x30(%r8), %rdi
14093d89d:     	movl	$0x8, %esi
14093d8a2:     	cmovbq	%rsi, %rbx
14093d8a6:     	leaq	(%r14,%r14,4), %r15
14093d8aa:     	setbe	%r12b
14093d8ae:     	leaq	-0x28(%rbx,%r15,8), %r15
14093d8b3:     	cmovbeq	%r9, %r15
14093d8b7:     	subq	%r10, %rdi
14093d8ba:     	cmovbq	%r9, %rdi
14093d8be:     	cmovbq	%rsi, %r11
14093d8c2:     	movq	%rcx, %rbx
14093d8c5:     	leaq	(%rdi,%rdi,4), %rcx
14093d8c9:     	setbe	%r10b
14093d8cd:     	leaq	-0x28(%r11,%rcx,8), %rsi
14093d8d2:     	cmovbeq	%r15, %rsi
14093d8d6:     	addq	%r14, %rdi
14093d8d9:     	testb	%r10b, %r12b
14093d8dc:     	cmoveq	%r9, %rdi
14093d8e0:     	movq	0x40(%r8), %rcx
14093d8e4:     	subq	%rdi, %rcx
14093d8e7:     	movq	%rcx, (%rax)
14093d8ea:     	testb	%r10b, %r12b
14093d8ed:     	jne	0x14093dea7 <.text+0x93cea7>
14093d8f3:     	movl	0x1c(%rdx), %eax
14093d8f6:     	movq	0x10(%rdx), %r15
14093d8fa:     	movl	%eax, 0x160(%rbp)
14093d900:     	movl	%eax, (%r15)
14093d903:     	movq	(%rdx), %rcx
14093d906:     	xorl	%eax, %eax
14093d908:     	cmpq	(%rcx), %rax
14093d90b:     	movq	%rcx, %rax
14093d90e:     	movabsq	$-0x8000000000000000, %rdi # imm = 0x8000000000000000
14093d918:     	movq	%rcx, 0x178(%rbp)
14093d91f:     	jo	0x14093d962 <.text+0x93c962>
14093d921:     	leaq	0x18(%rax), %rcx
14093d925:     	movq	%rcx, 0x188(%rbp)
14093d92c:     	callq	0x14003bb80 <.text+0x3ab80>
14093d931:     	movq	0x188(%rbp), %rcx
14093d938:     	callq	0x140be24b0 <.text+0xbe14b0>
14093d93d:     	movq	0x178(%rbp), %rax
14093d944:     	movq	(%rax), %rdx
14093d947:     	testq	%rdx, %rdx
14093d94a:     	je	0x14093d962 <.text+0x93c962>
14093d94c:     	movq	0x8(%rax), %rcx
14093d950:     	movl	$0x1, %r8d
14093d956:     	callq	0x140613c20 <.text+0x612c20>
14093d95b:     	movq	0x178(%rbp), %rax
14093d962:     	movq	%rdi, (%rax)
14093d965:     	movq	%rsi, %rcx
14093d968:     	callq	0x140d76ed0 <.text+0xd75ed0>
14093d96d:     	movq	%rax, %rsi
14093d970:     	movq	%rdx, %rdi
14093d973:     	movq	$0x0, 0x108(%rbp)
14093d97e:     	movq	$0x1, 0x110(%rbp)
14093d989:     	movq	$0x0, 0x118(%rbp)
14093d994:     	movl	$0x0, (%rbp)
14093d99b:     	xorps	%xmm0, %xmm0
14093d99e:     	movups	%xmm0, 0x8(%rbp)
14093d9a2:     	movl	$0x7, 0x18(%rbp)
14093d9a9:     	movq	$0x0, 0x1c(%rbp)
14093d9b1:     	movw	$0x0, 0x24(%rbp)
14093d9b7:     	movb	$0x1, 0x20(%rbp)
14093d9bb:     	movq	%rbp, %rcx
14093d9be:     	movq	%rax, %rdx
14093d9c1:     	movq	%rdi, %r8
14093d9c4:     	callq	0x140d59dc0 <.text+0xd58dc0>
14093d9c9:     	testq	%rax, %rax
14093d9cc:     	jne	0x14093dd74 <.text+0x93cd74>
14093d9d2:     	movq	%rdx, 0xe0(%rbp)
14093d9d9:     	leaq	0xe0(%rbp), %rcx
14093d9e0:     	leaq	0x108(%rbp), %rdx
14093d9e7:     	callq	0x140d59c30 <.text+0xd58c30>
14093d9ec:     	testq	%rax, %rax
14093d9ef:     	jne	0x14093dd61 <.text+0x93cd61>
14093d9f5:     	movq	0xe0(%rbp), %rcx
14093d9fc:     	callq	*0x28549ee(%rip)        # 0x1431923f0
14093da02:     	movq	0x118(%rbp), %r8
14093da09:     	testq	%r8, %r8
14093da0c:     	je	0x14093dd9d <.text+0x93cd9d>
14093da12:     	movq	0x110(%rbp), %rdx
14093da19:     	xorl	%ecx, %ecx
14093da1b:     	callq	0x14168e0b0 <.text+0x168d0b0>
14093da20:     	movq	%rax, %r14
14093da23:     	movq	0x108(%rbp), %rdx
14093da2a:     	testq	%rdx, %rdx
14093da2d:     	je	0x14093da41 <.text+0x93ca41>
14093da2f:     	movq	0x110(%rbp), %rcx
14093da36:     	movl	$0x1, %r8d
14093da3c:     	callq	0x140613c20 <.text+0x612c20>
14093da41:     	testq	%r14, %r14
14093da44:     	je	0x14093ddbb <.text+0x93cdbb>
14093da4a:     	xorl	%ecx, %ecx
14093da4c:     	callq	0x141739890 <.text+0x1738890>
14093da51:     	testq	%rax, %rax
14093da54:     	movq	%r14, 0x188(%rbp)
14093da5b:     	je	0x14093dffc <.text+0x93cffc>
14093da61:     	movl	$0x0, 0x30(%rsp)
14093da69:     	movl	$0x0, 0x28(%rsp)
14093da71:     	movl	$0x4009a, 0x20(%rsp)    # imm = 0x4009A
14093da79:     	xorl	%ecx, %ecx
14093da7b:     	movq	%rax, 0x180(%rbp)
14093da82:     	movq	%rax, %rdx
14093da85:     	movl	$0x4009a, %r8d          # imm = 0x4009A
14093da8b:     	movq	%r14, %r9
14093da8e:     	callq	0x14167e9d0 <.text+0x167d9d0>
14093da93:     	testq	%rax, %rax
14093da96:     	je	0x14093dcb1 <.text+0x93ccb1>
14093da9c:     	movq	%rax, 0x168(%rbp)
14093daa3:     	movq	$0x1, (%rbp)
14093daab:     	movq	$0x0, 0x8(%rbp)
14093dab3:     	movq	$0x20, 0x10(%rbp)
14093dabb:     	movq	$0x2, 0x18(%rbp)
14093dac3:     	movq	$0x2, 0x78(%rbp)
14093dacb:     	leaq	0x2173a5e(%rip), %r8    # 0x142ab1530
14093dad2:     	leaq	0x108(%rbp), %rcx
14093dad9:     	movq	%rbp, %rdx
14093dadc:     	callq	0x1413036c0 <.text+0x13026c0>
14093dae1:     	movq	0x118(%rbp), %r14
14093dae8:     	movl	$0xffffffff, %eax       # imm = 0xFFFFFFFF
14093daed:     	cmpq	%rax, %r14
14093daf0:     	jae	0x14093dfdf <.text+0x93cfdf>
14093daf6:     	movq	%r15, 0x158(%rbp)
14093dafd:     	movq	0x110(%rbp), %r15
14093db04:     	movq	0x168(%rbp), %r13
14093db0b:     	movq	%r13, %rcx
14093db0e:     	movq	%r15, %rdx
14093db11:     	movq	%r15, %r8
14093db14:     	movl	%r14d, %r9d
14093db17:     	callq	0x14167dd60 <.text+0x167cd60>
14093db1c:     	movq	0x108(%rbp), %r12
14093db23:     	movq	%r13, %rcx
14093db26:     	callq	0x14167dcb0 <.text+0x167ccb0>
14093db2b:     	movq	0x180(%rbp), %rcx
14093db32:     	callq	0x14168df40 <.text+0x168cf40>
14093db37:     	movq	0x188(%rbp), %rcx
14093db3e:     	callq	0x14168df40 <.text+0x168cf40>
14093db43:     	movq	%r12, %rax
14093db46:     	negq	%rax
14093db49:     	jo	0x14093ddd0 <.text+0x93cdd0>
14093db4f:     	movq	%r12, 0x180(%rbp)
14093db56:     	movq	%r15, 0x188(%rbp)
14093db5d:     	movq	(%rbx), %rax
14093db60:     	movq	(%rax), %rbx
14093db63:     	movabsq	$0x2000000020, %rax     # imm = 0x2000000020
14093db6d:     	movq	%rax, 0x108(%rbp)
14093db74:     	movl	$0x20, 0x110(%rbp)
14093db7e:     	movq	%rbp, %rcx
14093db81:     	leaq	0x108(%rbp), %rdx
14093db88:     	movl	$0x5b, %r8d
14093db8e:     	callq	0x140bd5310 <.text+0xbd4310>
14093db93:     	leaq	0xe0(%rbp), %r15
14093db9a:     	movq	%rbp, %r12
14093db9d:     	movq	%r15, %rcx
14093dba0:     	movq	%r12, %rdx
14093dba3:     	callq	0x140bd5370 <.text+0xbd4370>
14093dba8:     	addq	$0x10, %rbx
14093dbac:     	shlq	$0x3, %r14
14093dbb0:     	movq	0x188(%rbp), %rax
14093dbb7:     	movq	%rax, 0x8(%rbp)
14093dbbb:     	movq	%r14, 0x10(%rbp)
14093dbbf:     	movq	$0x3, (%rbp)
14093dbc7:     	movq	%r12, 0x28(%rsp)
14093dbcc:     	movq	%r15, 0x20(%rsp)
14093dbd1:     	leaq	0x217450e(%rip), %r8    # 0x142ab20e6
14093dbd8:     	leaq	0x108(%rbp), %rcx
14093dbdf:     	movl	$0x3, %r9d
14093dbe5:     	movq	%rbx, %rdx
14093dbe8:     	callq	0x1416a71f0 <.text+0x16a61f0>
14093dbed:     	cmpl	$0xb, 0x108(%rbp)
14093dbf4:     	jne	0x14093e02c <.text+0x93d02c>
14093dbfa:     	movq	0x140(%rbp), %rax
14093dc01:     	movq	%rax, 0x30(%rbp)
14093dc05:     	movups	0x110(%rbp), %xmm0
14093dc0c:     	movups	0x120(%rbp), %xmm1
14093dc13:     	movups	0x130(%rbp), %xmm2
14093dc1a:     	movaps	%xmm2, 0x20(%rbp)
14093dc1e:     	movaps	%xmm1, 0x10(%rbp)
14093dc22:     	movaps	%xmm0, (%rbp)
14093dc26:     	movq	%rsi, %rcx
14093dc29:     	movq	%rdi, %rdx
14093dc2c:     	callq	0x140d61e00 <.text+0xd60e00>
14093dc31:     	testq	%rax, %rax
14093dc34:     	je	0x14093dcd8 <.text+0x93ccd8>
14093dc3a:     	movq	%rdx, %r8
14093dc3d:     	leaq	0x108(%rbp), %rcx
14093dc44:     	movq	%rax, %rdx
14093dc47:     	callq	0x140d77340 <.text+0xd76340>
14093dc4c:     	movq	0x108(%rbp), %rdi
14093dc53:     	movq	0x110(%rbp), %r12
14093dc5a:     	movq	%rdi, %rax
14093dc5d:     	negq	%rax
14093dc60:     	movq	0x118(%rbp), %r15
14093dc67:     	jno	0x14093dd4e <.text+0x93cd4e>
14093dc6d:     	testq	%r15, %r15
14093dc70:     	movq	0x188(%rbp), %rbx
14093dc77:     	movq	0x180(%rbp), %r14
14093dc7e:     	js	0x14093e08c <.text+0x93d08c>
14093dc84:     	je	0x14093debc <.text+0x93cebc>
14093dc8a:     	movzbl	0x2cf6f78(%rip), %eax   # 0x143634c09
14093dc91:     	movl	$0x1, %edi
14093dc96:     	movl	$0x1, %edx
14093dc9b:     	movq	%r15, %rcx
14093dc9e:     	callq	0x140613c10 <.text+0x612c10>
14093dca3:     	testq	%rax, %rax
14093dca6:     	jne	0x14093dd46 <.text+0x93cd46>
14093dcac:     	jmp	0x14093e08e <.text+0x93d08e>
14093dcb1:     	xorl	%ecx, %ecx
14093dcb3:     	callq	0x1428a9650 <.text+0x28a8650>
14093dcb8:     	movq	%rax, %r15
14093dcbb:     	movq	0x180(%rbp), %rcx
14093dcc2:     	callq	0x14168df40 <.text+0x168cf40>
14093dcc7:     	movq	0x188(%rbp), %rcx
14093dcce:     	callq	0x14168df40 <.text+0x168cf40>
14093dcd3:     	jmp	0x14093ddd0 <.text+0x93cdd0>
14093dcd8:     	leaq	0x108(%rbp), %rcx
14093dcdf:     	movq	%rsi, %rdx
14093dce2:     	movq	%rdi, %r8
14093dce5:     	callq	0x140d77340 <.text+0xd76340>
14093dcea:     	movq	0x108(%rbp), %rdi
14093dcf1:     	movq	0x110(%rbp), %r12
14093dcf8:     	movq	%rdi, %rax
14093dcfb:     	negq	%rax
14093dcfe:     	movq	0x118(%rbp), %r15
14093dd05:     	jno	0x14093dd4e <.text+0x93cd4e>
14093dd07:     	testq	%r15, %r15
14093dd0a:     	movq	0x188(%rbp), %rbx
14093dd11:     	movq	0x180(%rbp), %r14
14093dd18:     	js	0x14093e0a2 <.text+0x93d0a2>
14093dd1e:     	je	0x14093debc <.text+0x93cebc>
14093dd24:     	movzbl	0x2cf6ede(%rip), %eax   # 0x143634c09
14093dd2b:     	movl	$0x1, %edi
14093dd30:     	movl	$0x1, %edx
14093dd35:     	movq	%r15, %rcx
14093dd38:     	callq	0x140613c10 <.text+0x612c10>
14093dd3d:     	testq	%rax, %rax
14093dd40:     	je	0x14093e0a4 <.text+0x93d0a4>
14093dd46:     	movq	%rax, %rsi
14093dd49:     	jmp	0x14093dec1 <.text+0x93cec1>
14093dd4e:     	movq	0x188(%rbp), %rbx
14093dd55:     	movq	0x180(%rbp), %r14
14093dd5c:     	jmp	0x14093ded5 <.text+0x93ced5>
14093dd61:     	movq	0xe0(%rbp), %rcx
14093dd68:     	movq	%rdx, %rsi
14093dd6b:     	callq	*0x285467f(%rip)        # 0x1431923f0
14093dd71:     	movq	%rsi, %rdx
14093dd74:     	movq	0x108(%rbp), %rax
14093dd7b:     	testq	%rax, %rax
14093dd7e:     	je	0x14093ddc5 <.text+0x93cdc5>
14093dd80:     	movq	0x110(%rbp), %rcx
14093dd87:     	movl	$0x1, %r8d
14093dd8d:     	movq	%rdx, %rsi
14093dd90:     	movq	%rax, %rdx
14093dd93:     	callq	0x140613c20 <.text+0x612c20>
14093dd98:     	movq	%rsi, %rdx
14093dd9b:     	jmp	0x14093ddc5 <.text+0x93cdc5>
14093dd9d:     	movq	0x108(%rbp), %rdx
14093dda4:     	testq	%rdx, %rdx
14093dda7:     	je	0x14093ddbb <.text+0x93cdbb>
14093dda9:     	movq	0x110(%rbp), %rcx
14093ddb0:     	movl	$0x1, %r8d
14093ddb6:     	callq	0x140613c20 <.text+0x612c20>
14093ddbb:     	movabsq	$0x2800000003, %rdx     # imm = 0x2800000003
14093ddc5:     	movq	%rdx, %rcx
14093ddc8:     	callq	0x1428a9590 <.text+0x28a8590>
14093ddcd:     	movq	%rax, %r15
14093ddd0:     	movq	%r15, 0xe0(%rbp)
14093ddd7:     	movq	0x2cee5ca(%rip), %rax   # 0x14362c3a8
14093ddde:     	movq	(%rax), %rax
14093dde1:     	cmpq	$0x2, %rax
14093dde5:     	jb	0x14093de9b <.text+0x93ce9b>
14093ddeb:     	leaq	0xe0(%rbp), %rax
14093ddf2:     	movq	%rax, 0x108(%rbp)
14093ddf9:     	leaq	0x40ee20(%rip), %rax    # 0x140d4cc20 <.text+0xd4bc20>
14093de00:     	movq	%rax, 0x110(%rbp)
14093de07:     	leaq	0x2174332(%rip), %rcx   # 0x142ab2140
14093de0e:     	callq	0x14067dd80 <.text+0x67cd80>
14093de13:     	movups	(%rax), %xmm0
14093de16:     	movl	0x10(%rax), %eax
14093de19:     	movq	$0x2, 0x30(%rbp)
14093de21:     	leaq	0x2174330(%rip), %rcx   # 0x142ab2158
14093de28:     	movq	%rcx, 0x38(%rbp)
14093de2c:     	movq	$0x2a, 0x40(%rbp)
14093de34:     	leaq	0x21742f5(%rip), %rdx   # 0x142ab2130
14093de3b:     	movq	%rdx, 0x50(%rbp)
14093de3f:     	movq	$0x1, 0x58(%rbp)
14093de47:     	leaq	0x108(%rbp), %rdx
14093de4e:     	movq	%rdx, 0x60(%rbp)
14093de52:     	movq	$0x1, 0x68(%rbp)
14093de5a:     	movq	$0x0, 0x70(%rbp)
14093de62:     	movq	$0x0, (%rbp)
14093de6a:     	movq	%rcx, 0x8(%rbp)
14093de6e:     	movq	$0x2a, 0x10(%rbp)
14093de76:     	movq	$0x0, 0x18(%rbp)
14093de7e:     	movups	%xmm0, 0x20(%rbp)
14093de82:     	movl	$0x1, 0x48(%rbp)
14093de89:     	movl	%eax, 0x4c(%rbp)
14093de8c:     	leaq	0x177(%rbp), %rcx
14093de93:     	movq	%rbp, %rdx
14093de96:     	callq	0x140d4a330 <.text+0xd49330>
14093de9b:     	leaq	0xe0(%rbp), %rcx
14093dea2:     	callq	0x140d4cc30 <.text+0xd4bc30>
14093dea7:     	nop
14093dea8:     	addq	$0x218, %rsp            # imm = 0x218
14093deaf:     	popq	%rbx
14093deb0:     	popq	%rdi
14093deb1:     	popq	%rsi
14093deb2:     	popq	%r12
14093deb4:     	popq	%r13
14093deb6:     	popq	%r14
14093deb8:     	popq	%r15
14093deba:     	popq	%rbp
14093debb:     	retq
14093debc:     	movl	$0x1, %esi
14093dec1:     	movq	%rsi, %rcx
14093dec4:     	movq	%r12, %rdx
14093dec7:     	movq	%r15, %r8
14093deca:     	callq	0x142923c00 <.text+0x2922c00>
14093decf:     	movq	%r15, %rdi
14093ded2:     	movq	%rsi, %r12
14093ded5:     	movq	0x30(%rbp), %rax
14093ded9:     	movq	%rax, -0x10(%rbp)
14093dedd:     	movaps	(%rbp), %xmm0
14093dee1:     	movaps	0x10(%rbp), %xmm1
14093dee5:     	movaps	0x20(%rbp), %xmm2
14093dee9:     	movaps	%xmm2, -0x20(%rbp)
14093deed:     	movaps	%xmm1, -0x30(%rbp)
14093def1:     	movaps	%xmm0, -0x40(%rbp)
14093def5:     	movl	0x160(%rbp), %eax
14093defb:     	movq	0x158(%rbp), %rcx
14093df02:     	movl	%eax, (%rcx)
14093df04:     	xorl	%eax, %eax
14093df06:     	movq	0x178(%rbp), %rcx
14093df0d:     	cmpq	(%rcx), %rax
14093df10:     	movq	%rcx, %rax
14093df13:     	jo	0x14093df8e <.text+0x93cf8e>
14093df15:     	movq	%rdi, 0x100(%rbp)
14093df1c:     	movq	%r12, 0x158(%rbp)
14093df23:     	movq	%r15, 0x168(%rbp)
14093df2a:     	leaq	0x18(%rax), %rcx
14093df2e:     	movq	%rcx, 0x160(%rbp)
14093df35:     	callq	0x14003bb80 <.text+0x3ab80>
14093df3a:     	movq	0x160(%rbp), %rcx
14093df41:     	callq	0x140be24b0 <.text+0xbe14b0>
14093df46:     	movq	0x178(%rbp), %rax
14093df4d:     	movq	(%rax), %rdx
14093df50:     	testq	%rdx, %rdx
14093df53:     	movq	0x188(%rbp), %rbx
14093df5a:     	movq	0x180(%rbp), %r14
14093df61:     	movq	0x168(%rbp), %r15
14093df68:     	movq	0x158(%rbp), %r12
14093df6f:     	movq	0x100(%rbp), %rdi
14093df76:     	je	0x14093df8e <.text+0x93cf8e>
14093df78:     	movq	0x8(%rax), %rcx
14093df7c:     	movl	$0x1, %r8d
14093df82:     	callq	0x140613c20 <.text+0x612c20>
14093df87:     	movq	0x178(%rbp), %rax
14093df8e:     	movq	%rdi, (%rax)
14093df91:     	movq	%r12, 0x8(%rax)
14093df95:     	movq	%r15, 0x10(%rax)
14093df99:     	movaps	-0x40(%rbp), %xmm0
14093df9d:     	movaps	-0x30(%rbp), %xmm1
14093dfa1:     	movaps	-0x20(%rbp), %xmm2
14093dfa5:     	movups	%xmm0, 0x18(%rax)
14093dfa9:     	movups	%xmm1, 0x28(%rax)
14093dfad:     	movups	%xmm2, 0x38(%rax)
14093dfb1:     	movq	%rax, %rcx
14093dfb4:     	movq	-0x10(%rbp), %rax
14093dfb8:     	movq	%rax, 0x48(%rcx)
14093dfbc:     	testq	%r14, %r14
14093dfbf:     	je	0x14093dea7 <.text+0x93cea7>
14093dfc5:     	shlq	$0x3, %r14
14093dfc9:     	movl	$0x2, %r8d
14093dfcf:     	movq	%rbx, %rcx
14093dfd2:     	movq	%r14, %rdx
14093dfd5:     	callq	0x140613c20 <.text+0x612c20>
14093dfda:     	jmp	0x14093dea7 <.text+0x93cea7>
14093dfdf:     	leaq	0x217373a(%rip), %rcx   # 0x142ab1720
14093dfe6:     	leaq	0x21737c3(%rip), %r8    # 0x142ab17b0
14093dfed:     	movl	$0x2f, %edx
14093dff2:     	callq	0x1428d9430 <.text+0x28d8430>
14093dff7:     	jmp	0x14093e0b6 <.text+0x93d0b6>
14093dffc:     	movb	$0x0, (%rbp)
14093e000:     	leaq	0x2173701(%rip), %rax   # 0x142ab1708
14093e007:     	movq	%rax, 0x20(%rsp)
14093e00c:     	leaq	0x2173575(%rip), %rcx   # 0x142ab1588
14093e013:     	leaq	0x217354e(%rip), %r9    # 0x142ab1568
14093e01a:     	movq	%rbp, %r8
14093e01d:     	movl	$0x2b, %edx
14093e022:     	callq	0x1428d9760 <.text+0x28d8760>
14093e027:     	jmp	0x14093e0b6 <.text+0x93d0b6>
14093e02c:     	movups	0x148(%rbp), %xmm0
14093e033:     	movaps	%xmm0, 0x40(%rbp)
14093e037:     	movups	0x108(%rbp), %xmm0
14093e03e:     	movups	0x118(%rbp), %xmm1
14093e045:     	movups	0x128(%rbp), %xmm2
14093e04c:     	movups	0x138(%rbp), %xmm3
14093e053:     	movaps	%xmm3, 0x30(%rbp)
14093e057:     	movaps	%xmm2, 0x20(%rbp)
14093e05b:     	movaps	%xmm1, 0x10(%rbp)
14093e05f:     	movaps	%xmm0, (%rbp)
14093e063:     	leaq	0x217408e(%rip), %rax   # 0x142ab20f8
14093e06a:     	movq	%rax, 0x20(%rsp)
14093e06f:     	leaq	0x2174073(%rip), %rcx   # 0x142ab20e9
14093e076:     	leaq	0x21734cb(%rip), %r9    # 0x142ab1548
14093e07d:     	movq	%rbp, %r8
14093e080:     	movl	$0xf, %edx
14093e085:     	callq	0x1428d9760 <.text+0x28d8760>
14093e08a:     	jmp	0x14093e0b6 <.text+0x93d0b6>
14093e08c:     	xorl	%edi, %edi
14093e08e:     	leaq	0x23030a3(%rip), %r8    # 0x142c41138
14093e095:     	movq	%rdi, %rcx
14093e098:     	movq	%r15, %rdx
14093e09b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14093e0a0:     	jmp	0x14093e0b6 <.text+0x93d0b6>
14093e0a2:     	xorl	%edi, %edi
14093e0a4:     	leaq	0x230308d(%rip), %r8    # 0x142c41138
14093e0ab:     	movq	%rdi, %rcx
14093e0ae:     	movq	%r15, %rdx
14093e0b1:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
14093e0b6:     	ud2
