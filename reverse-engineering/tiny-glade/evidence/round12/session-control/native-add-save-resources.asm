
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409447e0:     	pushq	%rbp
1409447e1:     	pushq	%r15
1409447e3:     	pushq	%r14
1409447e5:     	pushq	%rsi
1409447e6:     	pushq	%rdi
1409447e7:     	pushq	%rbx
1409447e8:     	subq	$0x348, %rsp            # imm = 0x348
1409447ef:     	leaq	0x80(%rsp), %rbp
1409447f7:     	movaps	%xmm6, 0x2b0(%rbp)
1409447fe:     	andq	$-0x80, %rsp
140944802:     	movq	%rsp, %rbx
140944805:     	movq	%rbp, 0x320(%rbx)
14094480c:     	movq	$-0x2, 0x2a8(%rbp)
140944817:     	movq	%rcx, %rsi
14094481a:     	movq	$0x1, 0x280(%rbx)
140944825:     	movq	$0x1, 0x288(%rbx)
140944830:     	movb	$0x0, 0x290(%rbx)
140944837:     	xorps	%xmm6, %xmm6
14094483a:     	movaps	%xmm6, 0x100(%rbx)
140944841:     	movaps	%xmm6, 0x180(%rbx)
140944848:     	movw	$0x0, 0x200(%rbx)
140944851:     	movq	$0x0, 0x208(%rbx)
14094485c:     	movq	$0x8, 0x210(%rbx)
140944867:     	movups	%xmm6, 0x218(%rbx)
14094486e:     	movq	$0x8, 0x228(%rbx)
140944879:     	movq	$0x0, 0x230(%rbx)
140944884:     	movb	$0x1, 0x238(%rbx)
14094488b:     	movzbl	0x2cf0377(%rip), %eax   # 0x143634c09
140944892:     	movl	$0x200, %ecx            # imm = 0x200
140944897:     	movl	$0x80, %edx
14094489c:     	callq	0x140613c10 <.text+0x612c10>
1409448a1:     	testq	%rax, %rax
1409448a4:     	je	0x140944e19 <.text+0x943e19>
1409448aa:     	movq	%rax, %rdi
1409448ad:     	leaq	0x100(%rbx), %rdx
1409448b4:     	movl	$0x200, %r8d            # imm = 0x200
1409448ba:     	movq	%rax, %rcx
1409448bd:     	callq	0x142923c00 <.text+0x2922c00>
1409448c2:     	movq	$0x1, 0x90(%rbx)
1409448cd:     	movq	%rdi, 0x98(%rbx)
1409448d4:     	movq	$0x1, 0xa0(%rbx)
1409448df:     	movq	%rdi, 0xa8(%rbx)
1409448e6:     	movq	$0x1, 0x280(%rbx)
1409448f1:     	movq	$0x1, 0x288(%rbx)
1409448fc:     	movb	$0x0, 0x290(%rbx)
140944903:     	movaps	%xmm6, 0x100(%rbx)
14094490a:     	movaps	%xmm6, 0x180(%rbx)
140944911:     	movw	$0x0, 0x200(%rbx)
14094491a:     	movq	$0x0, 0x208(%rbx)
140944925:     	movq	$0x8, 0x210(%rbx)
140944930:     	movups	%xmm6, 0x218(%rbx)
140944937:     	movq	$0x8, 0x228(%rbx)
140944942:     	movq	$0x0, 0x230(%rbx)
14094494d:     	movb	$0x1, 0x238(%rbx)
140944954:     	movzbl	0x2cf02ae(%rip), %eax   # 0x143634c09
14094495b:     	movl	$0x200, %ecx            # imm = 0x200
140944960:     	movl	$0x80, %edx
140944965:     	callq	0x140613c10 <.text+0x612c10>
14094496a:     	testq	%rax, %rax
14094496d:     	je	0x140944e2d <.text+0x943e2d>
140944973:     	movq	%rax, %r14
140944976:     	leaq	0x100(%rbx), %rdx
14094497d:     	movl	$0x200, %r8d            # imm = 0x200
140944983:     	movq	%rax, %rcx
140944986:     	callq	0x142923c00 <.text+0x2922c00>
14094498b:     	movq	$0x1, 0xb0(%rbx)
140944996:     	movq	%r14, 0xb8(%rbx)
14094499d:     	movq	$0x1, 0xc0(%rbx)
1409449a8:     	movq	%r14, 0xc8(%rbx)
1409449af:     	movzbl	0x2cf0253(%rip), %eax   # 0x143634c09
1409449b6:     	movl	$0x30, %ecx
1409449bb:     	movl	$0x8, %edx
1409449c0:     	callq	0x140613c10 <.text+0x612c10>
1409449c5:     	testq	%rax, %rax
1409449c8:     	je	0x140944e3e <.text+0x943e3e>
1409449ce:     	movq	$0x1, (%rax)
1409449d5:     	movq	$0x1, 0x8(%rax)
1409449dd:     	movb	$0x0, 0x10(%rax)
1409449e1:     	movq	$0x0, 0x18(%rax)
1409449e9:     	movq	$0x8, 0x20(%rax)
1409449f1:     	movq	$0x0, 0x28(%rax)
1409449f9:     	movq	%rax, 0x48(%rbx)
1409449fd:     	lock
1409449fe:     	incq	(%rax)
140944a01:     	jle	0x140944eb6 <.text+0x943eb6>
140944a07:     	movq	%rax, 0x40(%rbx)
140944a0b:     	movq	%rax, 0x68(%rbx)
140944a0f:     	movabsq	$-0x8000000000000000, %r15 # imm = 0x8000000000000000
140944a19:     	movq	%r15, 0x110(%rbx)
140944a20:     	movq	$0x0, 0x100(%rbx)
140944a2b:     	movb	$0x0, 0x128(%rbx)
140944a32:     	movzbl	0x2cf01d0(%rip), %eax   # 0x143634c09
140944a39:     	movl	$0xa, %ecx
140944a3e:     	movl	$0x1, %edx
140944a43:     	callq	0x140613c10 <.text+0x612c10>
140944a48:     	testq	%rax, %rax
140944a4b:     	movq	%rax, 0x38(%rbx)
140944a4f:     	je	0x140944e57 <.text+0x943e57>
140944a55:     	movabsq	$0x617320636e797341, %rcx # imm = 0x617320636E797341
140944a5f:     	movq	%rcx, (%rax)
140944a62:     	movw	$0x6576, 0x8(%rax)      # imm = 0x6576
140944a68:     	movq	$0xa, 0x50(%rbx)
140944a70:     	movq	%rax, 0x58(%rbx)
140944a74:     	movq	$0xa, 0x60(%rbx)
140944a7c:     	leaq	0xd0(%rbx), %rcx
140944a83:     	leaq	0x100(%rbx), %rdx
140944a8a:     	leaq	0x50(%rbx), %r8
140944a8e:     	callq	0x140d558f0 <.text+0xd548f0>
140944a93:     	movq	$0x1, 0x100(%rbx)
140944a9e:     	movq	%rdi, 0x108(%rbx)
140944aa5:     	movq	$0x1, 0x110(%rbx)
140944ab0:     	movq	%r14, 0x118(%rbx)
140944ab7:     	movq	0x40(%rbx), %rax
140944abb:     	movq	%rax, 0x120(%rbx)
140944ac2:     	movb	$0x1, 0x31(%rbx)
140944ac6:     	movb	$0x0, 0x32(%rbx)
140944aca:     	leaq	0x50(%rbx), %rcx
140944ace:     	leaq	0xd0(%rbx), %rdx
140944ad5:     	leaq	0x100(%rbx), %r8
140944adc:     	callq	0x140a6d8b0 <.text+0xa6c8b0>
140944ae1:     	cmpq	$0x0, 0x50(%rbx)
140944ae6:     	je	0x140944e6f <.text+0x943e6f>
140944aec:     	movq	0x60(%rbx), %rax
140944af0:     	movq	%rax, 0x80(%rbx)
140944af7:     	movups	0x50(%rbx), %xmm0
140944afb:     	movaps	%xmm0, 0x70(%rbx)
140944aff:     	leaq	0x38(%rsi), %rcx
140944b03:     	movb	$0x1, 0x2f(%rbx)
140944b07:     	movb	$0x1, 0x30(%rbx)
140944b0b:     	movabsq	$0x4ab328be485143c6, %rdx # imm = 0x4AB328BE485143C6
140944b15:     	movabsq	$-0x837924aa97582f9, %r8 # imm = 0xF7C86DB5568A7D07
140944b1f:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140944b24:     	testq	%rax, %rax
140944b27:     	je	0x140944b5c <.text+0x943b5c>
140944b29:     	cmpq	%rdx, 0x218(%rsi)
140944b30:     	jbe	0x140944b5c <.text+0x943b5c>
140944b32:     	movq	0x210(%rsi), %rax
140944b39:     	movq	(%rax,%rdx,8), %rax
140944b3d:     	testq	%rax, %rax
140944b40:     	je	0x140944b5c <.text+0x943b5c>
140944b42:     	movq	0x1e0(%rsi), %rcx
140944b49:     	notq	%rax
140944b4c:     	leaq	(%rax,%rax,2), %rax
140944b50:     	shlq	$0x5, %rax
140944b54:     	cmpq	$0x0, 0x20(%rcx,%rax)
140944b5a:     	jne	0x140944b6c <.text+0x943b6c>
140944b5c:     	movb	$0x1, 0x2f(%rbx)
140944b60:     	movb	$0x1, 0x30(%rbx)
140944b64:     	movq	%rsi, %rcx
140944b67:     	callq	0x141521240 <.text+0x1520240>
140944b6c:     	movq	%r15, 0xd0(%rbx)
140944b73:     	movzbl	0x2cf008f(%rip), %eax   # 0x143634c09
140944b7a:     	movl	$0x20, %ecx
140944b7f:     	movl	$0x8, %edx
140944b84:     	callq	0x140613c10 <.text+0x612c10>
140944b89:     	testq	%rax, %rax
140944b8c:     	je	0x140944ea7 <.text+0x943ea7>
140944b92:     	movl	$0x0, 0x8(%rax)
140944b99:     	movq	%r15, (%rax)
140944b9c:     	movups	0xd0(%rbx), %xmm0
140944ba3:     	movups	0xe0(%rbx), %xmm1
140944baa:     	movups	0xf0(%rbx), %xmm2
140944bb1:     	movups	%xmm2, 0x138(%rbx)
140944bb8:     	movups	%xmm1, 0x128(%rbx)
140944bbf:     	movups	%xmm0, 0x118(%rbx)
140944bc6:     	movb	$0x2, 0x148(%rbx)
140944bcd:     	movq	$0x1, 0x100(%rbx)
140944bd8:     	movq	%rax, 0x108(%rbx)
140944bdf:     	movq	$0x1, 0x110(%rbx)
140944bea:     	movb	$0x1, 0x2f(%rbx)
140944bee:     	movb	$0x1, 0x30(%rbx)
140944bf2:     	leaq	0x100(%rbx), %rdx
140944bf9:     	movq	%rsi, %rcx
140944bfc:     	callq	0x141541420 <.text+0x1540420>
140944c01:     	movq	0x80(%rbx), %rcx
140944c08:     	movq	%rcx, 0x210(%rbx)
140944c0f:     	movaps	0x70(%rbx), %xmm0
140944c13:     	movups	%xmm0, 0x200(%rbx)
140944c1a:     	movq	$0x0, 0x220(%rbx)
140944c25:     	movq	$0x1, 0x100(%rbx)
140944c30:     	movq	%rdi, 0x108(%rbx)
140944c37:     	movb	$0x0, 0x228(%rbx)
140944c3e:     	movq	$0x1, 0x230(%rbx)
140944c49:     	movq	%r14, 0x238(%rbx)
140944c50:     	movq	$0x0, 0x110(%rbx)
140944c5b:     	movq	$0x8, 0x118(%rbx)
140944c66:     	movq	$0x0, 0x120(%rbx)
140944c71:     	movq	%r15, 0x128(%rbx)
140944c78:     	movl	$0x0, 0x240(%rbx)
140944c82:     	movsd	0x216d726(%rip), %xmm0  # 0x142ab23b0
140944c8a:     	movsd	%xmm0, 0x244(%rbx)
140944c92:     	movq	0x40(%rbx), %rcx
140944c96:     	movq	%rcx, 0x218(%rbx)
140944c9d:     	movb	$0x0, 0x2f(%rbx)
140944ca1:     	movb	$0x0, 0x30(%rbx)
140944ca5:     	leaq	0x100(%rbx), %rdx
140944cac:     	movq	%rax, %rcx
140944caf:     	callq	0x1415496b0 <.text+0x15486b0>
140944cb4:     	movq	%rax, %rsi
140944cb7:     	leaq	0x38(%rax), %rdi
140944cbb:     	movb	$0x0, 0x2f(%rbx)
140944cbf:     	movb	$0x0, 0x30(%rbx)
140944cc3:     	movabsq	$0x4d00324fb817a8a1, %rdx # imm = 0x4D00324FB817A8A1
140944ccd:     	movabsq	$0x5846f8d25a034525, %r8 # imm = 0x5846F8D25A034525
140944cd7:     	movq	%rdi, %rcx
140944cda:     	callq	0x1400dd6f0 <.text+0xdc6f0>
140944cdf:     	testq	%rax, %rax
140944ce2:     	je	0x140944d17 <.text+0x943d17>
140944ce4:     	cmpq	%rdx, 0x218(%rsi)
140944ceb:     	jbe	0x140944d17 <.text+0x943d17>
140944ced:     	movq	0x210(%rsi), %rax
140944cf4:     	movq	(%rax,%rdx,8), %rax
140944cf8:     	testq	%rax, %rax
140944cfb:     	je	0x140944d17 <.text+0x943d17>
140944cfd:     	movq	0x1e0(%rsi), %rcx
