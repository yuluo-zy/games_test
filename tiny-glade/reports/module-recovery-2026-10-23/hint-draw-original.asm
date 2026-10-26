
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141c51a60:     	pushq	%r15
141c51a62:     	pushq	%r14
141c51a64:     	pushq	%r13
141c51a66:     	pushq	%r12
141c51a68:     	pushq	%rsi
141c51a69:     	pushq	%rdi
141c51a6a:     	pushq	%rbp
141c51a6b:     	pushq	%rbx
141c51a6c:     	subq	$0x298, %rsp            # imm = 0x298
141c51a73:     	movaps	%xmm10, 0x280(%rsp)
141c51a7c:     	movaps	%xmm9, 0x270(%rsp)
141c51a85:     	movaps	%xmm8, 0x260(%rsp)
141c51a8e:     	movaps	%xmm7, 0x250(%rsp)
141c51a96:     	movaps	%xmm6, 0x240(%rsp)
141c51a9e:     	movq	%r9, 0x100(%rsp)
141c51aa6:     	movq	%rdx, %rsi
141c51aa9:     	movq	%rcx, %rdi
141c51aac:     	movq	0x308(%rsp), %rbx
141c51ab4:     	movq	0x310(%rsp), %r14
141c51abc:     	movq	(%r8), %rcx
141c51abf:     	callq	0x140958f30 <.text+0x957f30>
141c51ac4:     	movb	%al, 0x67(%rsp)
141c51ac8:     	movq	(%r14), %rax
141c51acb:     	movq	0x8(%r14), %rcx
141c51acf:     	movq	(%rax), %rdx
141c51ad2:     	xorl	%r8d, %r8d
141c51ad5:     	movq	%rdx, %r9
141c51ad8:     	subq	0x18(%rcx), %r9
141c51adc:     	cmovbq	%r8, %r9
141c51ae0:     	subq	0x38(%rcx), %rdx
141c51ae4:     	cmovbq	%r8, %rdx
141c51ae8:     	movq	0x30(%rcx), %r10
141c51aec:     	subq	%rdx, %r10
141c51aef:     	cmovbq	%r8, %r10
141c51af3:     	seta	%dl
141c51af6:     	movq	0x10(%rcx), %r11
141c51afa:     	subq	%r9, %r11
141c51afd:     	cmovbq	%r8, %r11
141c51b01:     	movq	0x40(%rcx), %rcx
141c51b05:     	seta	%r8b
141c51b09:     	addq	%r10, %r11
141c51b0c:     	movq	%rcx, %r9
141c51b0f:     	subq	%r11, %r9
141c51b12:     	movq	%r9, (%rax)
141c51b15:     	orb	%dl, %r8b
141c51b18:     	jne	0x141c51b4b <.text+0x1c50b4b>
141c51b1a:     	movq	(%rbx), %rax
141c51b1d:     	movq	%rax, 0xb8(%rsp)
141c51b25:     	movl	0x1c(%rdi), %eax
141c51b28:     	movl	%eax, 0x90(%rsp)
141c51b2f:     	movq	(%rdi), %rax
141c51b32:     	movq	%rax, 0xa8(%rsp)
141c51b3a:     	movq	0x10(%rdi), %rax
141c51b3e:     	movq	%rax, 0xb0(%rsp)
141c51b46:     	jmp	0x141c51ccd <.text+0x1c50ccd>
141c51b4b:     	movq	0x318(%rsp), %r14
141c51b53:     	movq	%rcx, (%rax)
141c51b56:     	movq	(%rbx), %rcx
141c51b59:     	movq	%rcx, 0xb8(%rsp)
141c51b61:     	callq	0x140949c10 <.text+0x948c10>
141c51b66:     	movaps	%xmm1, %xmm6
141c51b69:     	movq	(%r14), %rax
141c51b6c:     	movss	(%rax), %xmm8
141c51b71:     	movss	0x4(%rax), %xmm7
141c51b76:     	movl	0x1c(%rdi), %eax
141c51b79:     	movq	0x10(%rdi), %rcx
141c51b7d:     	movl	%eax, 0x90(%rsp)
141c51b84:     	movq	%rcx, 0xb0(%rsp)
141c51b8c:     	movl	%eax, (%rcx)
141c51b8e:     	movq	(%rdi), %r14
141c51b91:     	leaq	0x12c2ad8(%rip), %rax   # 0x142f14670
141c51b98:     	movq	%rax, 0x218(%rsp)
141c51ba0:     	movq	$0x11, 0x220(%rsp)
141c51bac:     	leaq	0x218(%rsp), %rcx
141c51bb4:     	callq	0x140683c10 <.text+0x682c10>
141c51bb9:     	movabsq	$-0x4e96acdfffb04bbd, %rdi # imm = 0xB1695320004FB443
141c51bc3:     	xorq	%rax, %rdi
141c51bc6:     	movq	%r14, %rcx
141c51bc9:     	movq	%rdi, %rdx
141c51bcc:     	callq	0x140683ec0 <.text+0x682ec0>
141c51bd1:     	movb	$0x0, 0x5d(%rax)
141c51bd5:     	testb	$0x1, %dl
141c51bd8:     	jne	0x141c51c4a <.text+0x1c50c4a>
141c51bda:     	divss	0xd09a4e(%rip), %xmm6   # 0x14295b630
141c51be2:     	divss	%xmm6, %xmm8
141c51be7:     	addss	0xd0aaa8(%rip), %xmm8   # 0x14295c698
141c51bf0:     	divss	%xmm6, %xmm7
141c51bf4:     	movq	0x10(%r14), %rbx
141c51bf8:     	cmpq	(%r14), %rbx
141c51bfb:     	jne	0x141c51c0c <.text+0x1c50c0c>
141c51bfd:     	leaq	0x13e87dc(%rip), %rdx   # 0x14303a3e0
141c51c04:     	movq	%r14, %rcx
141c51c07:     	callq	0x1406319d0 <.text+0x6309d0>
141c51c0c:     	movq	0x8(%r14), %rax
141c51c10:     	imulq	$0x38, %rbx, %rcx
141c51c14:     	movl	$0x4, (%rax,%rcx)
141c51c1b:     	movss	%xmm8, 0x4(%rax,%rcx)
141c51c22:     	movss	%xmm7, 0x8(%rax,%rcx)
141c51c28:     	movsd	0xcdcab0(%rip), %xmm0   # 0x14292e6e0
141c51c30:     	movsd	%xmm0, 0xc(%rax,%rcx)
141c51c36:     	movl	$0x0, 0x14(%rax,%rcx)
141c51c3e:     	movq	%rdi, 0x30(%rax,%rcx)
141c51c43:     	incq	%rbx
141c51c46:     	movq	%rbx, 0x10(%r14)
141c51c4a:     	movq	%r14, 0xa8(%rsp)
141c51c52:     	movq	%r14, 0x140(%rsp)
141c51c5a:     	movq	%rdi, 0x148(%rsp)
141c51c62:     	movq	$0x0, 0x150(%rsp)
141c51c6e:     	leaq	0x218(%rsp), %rdi
141c51c76:     	leaq	0x140(%rsp), %rdx
141c51c7e:     	movq	%rdi, %rcx
141c51c81:     	callq	0x140686690 <.text+0x685690>
141c51c86:     	leaq	0x12c29f4(%rip), %r8    # 0x142f14681
141c51c8d:     	leaq	0x140(%rsp), %r14
141c51c95:     	movl	$0xa, %r9d
141c51c9b:     	movq	%r14, %rcx
141c51c9e:     	movq	%rdi, %rdx
141c51ca1:     	callq	0x14068bee0 <.text+0x68aee0>
141c51ca6:     	movq	%r14, %rcx
141c51ca9:     	callq	0x14068ea60 <.text+0x68da60>
141c51cae:     	movl	$0x3f800000, 0xb0(%rax) # imm = 0x3F800000
141c51cb8:     	leaq	0x12c29cc(%rip), %rdx   # 0x142f1468b
141c51cbf:     	movl	$0x4, %r8d
141c51cc5:     	movq	%r14, %rcx
141c51cc8:     	callq	0x14228b8f0 <.text+0x228a8f0>
141c51ccd:     	movq	(%rsi), %rdi
141c51cd0:     	movq	0x8(%rsi), %r15
141c51cd4:     	movl	0x10(%rsi), %r9d
141c51cd8:     	movl	0x14(%rsi), %eax
141c51cdb:     	leaq	0x160(%rsp), %rcx
141c51ce3:     	movl	%eax, 0x20(%rsp)
141c51ce7:     	movq	%r15, %rdx
141c51cea:     	movq	%rdi, %r8
141c51ced:     	callq	0x14228cde0 <.text+0x228bde0>
141c51cf2:     	movq	0x160(%rsp), %r12
141c51cfa:     	movq	0x168(%rsp), %rbp
141c51d02:     	movq	0x180(%rsp), %rax
141c51d0a:     	movq	%rax, 0xc0(%rsp)
141c51d12:     	movq	0x190(%rsp), %rbx
141c51d1a:     	movq	0x198(%rsp), %r8
141c51d22:     	movq	0x1a0(%rsp), %r10
141c51d2a:     	movzbl	0x1a8(%rsp), %eax
141c51d32:     	movl	%eax, 0x5c(%rsp)
141c51d36:     	movq	0x1b0(%rsp), %rax
141c51d3e:     	movq	%rax, 0x80(%rsp)
141c51d46:     	movzbl	0x1b8(%rsp), %eax
141c51d4e:     	movl	%eax, 0x58(%rsp)
141c51d52:     	movq	0x1c0(%rsp), %rax
141c51d5a:     	movq	%rax, 0x78(%rsp)
141c51d5f:     	movzbl	0x1c8(%rsp), %eax
141c51d67:     	movl	%eax, 0x54(%rsp)
141c51d6b:     	movq	0x1d0(%rsp), %rax
141c51d73:     	movq	%rax, 0xa0(%rsp)
141c51d7b:     	movq	0x1e0(%rsp), %rax
141c51d83:     	movq	%rax, 0x98(%rsp)
141c51d8b:     	movl	0x1ec(%rsp), %eax
141c51d92:     	movl	%eax, 0x94(%rsp)
141c51d99:     	movq	0x1f0(%rsp), %rax
141c51da1:     	movq	%rax, 0x70(%rsp)
141c51da6:     	xorps	%xmm7, %xmm7
141c51da9:     	movss	0xd0987e(%rip), %xmm8   # 0x14295b630
141c51db2:     	movss	0xd08329(%rip), %xmm9   # 0x14295a0e4
141c51dbb:     	movzbl	0x210(%rsp), %r9d
141c51dc4:     	movq	0x1f8(%rsp), %rax
141c51dcc:     	movq	%rax, 0x68(%rsp)
141c51dd1:     	movq	0x200(%rsp), %rax
141c51dd9:     	movq	%rax, 0x88(%rsp)
141c51de1:     	movzbl	0x208(%rsp), %eax
141c51de9:     	movl	%eax, 0x60(%rsp)
141c51ded:     	movq	$0x0, 0xe0(%rsp)
141c51df9:     	movq	%rdi, 0xc8(%rsp)
141c51e01:     	movb	%r9b, 0x53(%rsp)
141c51e06:     	movq	%rbp, 0xf8(%rsp)
141c51e0e:     	movq	%r15, 0xf0(%rsp)
141c51e16:     	testb	$0x1, %r9b
141c51e1a:     	jne	0x141c52120 <.text+0x1c51120>
141c51e20:     	cmpq	%rbx, %r8
141c51e23:     	jne	0x141c520f9 <.text+0x1c510f9>
141c51e29:     	nopl	(%rax)
141c51e30:     	cmpq	%rbp, %r12
141c51e33:     	je	0x141c526bd <.text+0x1c516bd>
141c51e39:     	movl	(%r12), %ecx
141c51e3d:     	addq	$0x4, %r12
141c51e41:     	movq	0x100(%r15), %rax
141c51e48:     	leaq	(%rcx,%rcx,4), %rcx
141c51e4c:     	shlq	$0x5, %rcx
141c51e50:     	movq	0x58(%rax,%rcx), %rbx
141c51e55:     	testq	%rbx, %rbx
141c51e58:     	je	0x141c51e30 <.text+0x1c50e30>
141c51e5a:     	addq	%rcx, %rax
141c51e5d:     	movl	0x94(%rax), %ecx
141c51e63:     	movq	0x1a8(%r15), %rdx
141c51e6a:     	leaq	(%rcx,%rcx,8), %r9
141c51e6e:     	movq	0x40(%rdx,%r9,8), %r8
141c51e73:     	movq	0x18(%rdx,%r9,8), %rcx
141c51e78:     	movq	0x38(%rdx,%r9,8), %rdx
141c51e7d:     	movq	0x118(%rdi), %r9
141c51e84:     	cmpq	%r9, %r8
141c51e87:     	jbe	0x141c51ebf <.text+0x1c50ebf>
141c51e89:     	movq	(%rdx,%r9,8), %r9
141c51e8d:     	testq	%r9, %r9
141c51e90:     	je	0x141c51ebf <.text+0x1c50ebf>
141c51e92:     	movq	%r10, %rsi
141c51e95:     	notq	%r9
141c51e98:     	leaq	(%r9,%r9,2), %r9
141c51e9c:     	shlq	$0x4, %r9
141c51ea0:     	movq	0x10(%rcx,%r9), %r9
141c51ea5:     	movq	%r9, 0x70(%rsp)
141c51eaa:     	movq	0x120(%rdi), %r10
141c51eb1:     	movq	0x88(%rax), %r9
141c51eb8:     	cmpq	%r10, %r9
141c51ebb:     	ja	0x141c51ede <.text+0x1c50ede>
141c51ebd:     	jmp	0x141c51f20 <.text+0x1c50f20>
141c51ebf:     	movq	%r10, %rsi
141c51ec2:     	movq	$0x0, 0x70(%rsp)
141c51ecb:     	movq	0x120(%rdi), %r10
141c51ed2:     	movq	0x88(%rax), %r9
141c51ed9:     	cmpq	%r10, %r9
141c51edc:     	jbe	0x141c51f20 <.text+0x1c50f20>
141c51ede:     	movq	0x80(%rax), %r11
141c51ee5:     	cmpq	$0x0, (%r11,%r10,8)
141c51eea:     	je	0x141c51f20 <.text+0x1c50f20>
141c51eec:     	movb	$0x1, %r11b
141c51eef:     	movl	%r11d, 0x5c(%rsp)
141c51ef4:     	cmpq	%r10, %r8
141c51ef7:     	jbe	0x141c51f36 <.text+0x1c50f36>
141c51ef9:     	movq	(%rdx,%r10,8), %r10
141c51efd:     	testq	%r10, %r10
141c51f00:     	je	0x141c51f36 <.text+0x1c50f36>
141c51f02:     	notq	%r10
141c51f05:     	leaq	(%r10,%r10,2), %r10
141c51f09:     	shlq	$0x4, %r10
141c51f0d:     	movq	0x10(%rcx,%r10), %rsi
141c51f12:     	movq	0x128(%rdi), %r10
141c51f19:     	cmpq	%r10, %r8
141c51f1c:     	ja	0x141c51f44 <.text+0x1c50f44>
141c51f1e:     	jmp	0x141c51f70 <.text+0x1c50f70>
141c51f20:     	movl	$0x0, 0x5c(%rsp)
141c51f28:     	movq	0x128(%rdi), %r10
141c51f2f:     	cmpq	%r10, %r8
141c51f32:     	ja	0x141c51f44 <.text+0x1c50f44>
141c51f34:     	jmp	0x141c51f70 <.text+0x1c50f70>
141c51f36:     	xorl	%esi, %esi
141c51f38:     	movq	0x128(%rdi), %r10
141c51f3f:     	cmpq	%r10, %r8
141c51f42:     	jbe	0x141c51f70 <.text+0x1c50f70>
141c51f44:     	movq	(%rdx,%r10,8), %r10
141c51f48:     	testq	%r10, %r10
141c51f4b:     	je	0x141c51f70 <.text+0x1c50f70>
141c51f4d:     	notq	%r10
141c51f50:     	leaq	(%r10,%r10,2), %r10
141c51f54:     	shlq	$0x4, %r10
141c51f58:     	movq	0x10(%rcx,%r10), %r10
141c51f5d:     	movq	%r10, 0x68(%rsp)
141c51f62:     	movq	0x130(%rdi), %r10
141c51f69:     	cmpq	%r10, %r9
141c51f6c:     	jbe	0x141c51fcf <.text+0x1c50fcf>
141c51f6e:     	jmp	0x141c51f85 <.text+0x1c50f85>
141c51f70:     	movq	$0x0, 0x68(%rsp)
141c51f79:     	movq	0x130(%rdi), %r10
141c51f80:     	cmpq	%r10, %r9
141c51f83:     	jbe	0x141c51fcf <.text+0x1c50fcf>
141c51f85:     	movq	0x80(%rax), %r11
141c51f8c:     	cmpq	$0x0, (%r11,%r10,8)
141c51f91:     	je	0x141c51fcf <.text+0x1c50fcf>
141c51f93:     	movb	$0x1, %r11b
141c51f96:     	movl	%r11d, 0x58(%rsp)
141c51f9b:     	cmpq	%r10, %r8
141c51f9e:     	jbe	0x141c51fe5 <.text+0x1c50fe5>
141c51fa0:     	movq	(%rdx,%r10,8), %r10
141c51fa4:     	testq	%r10, %r10
141c51fa7:     	je	0x141c51fe5 <.text+0x1c50fe5>
141c51fa9:     	notq	%r10
141c51fac:     	leaq	(%r10,%r10,2), %r10
141c51fb0:     	shlq	$0x4, %r10
141c51fb4:     	movq	0x10(%rcx,%r10), %r10
141c51fb9:     	movq	%r10, 0x80(%rsp)
141c51fc1:     	movq	0x138(%rdi), %r10
141c51fc8:     	cmpq	%r10, %r9
141c51fcb:     	ja	0x141c51ffd <.text+0x1c50ffd>
141c51fcd:     	jmp	0x141c52038 <.text+0x1c51038>
141c51fcf:     	movl	$0x0, 0x58(%rsp)
141c51fd7:     	movq	0x138(%rdi), %r10
141c51fde:     	cmpq	%r10, %r9
141c51fe1:     	jbe	0x141c52038 <.text+0x1c51038>
141c51fe3:     	jmp	0x141c51ffd <.text+0x1c50ffd>
141c51fe5:     	movq	$0x0, 0x80(%rsp)
141c51ff1:     	movq	0x138(%rdi), %r10
141c51ff8:     	cmpq	%r10, %r9
141c51ffb:     	jbe	0x141c52038 <.text+0x1c51038>
141c51ffd:     	movq	0x80(%rax), %r11
141c52004:     	cmpq	$0x0, (%r11,%r10,8)
141c52009:     	je	0x141c52038 <.text+0x1c51038>
141c5200b:     	movb	$0x1, %r11b
141c5200e:     	movl	%r11d, 0x54(%rsp)
141c52013:     	cmpq	%r10, %r8
141c52016:     	jbe	0x141c52042 <.text+0x1c51042>
141c52018:     	movq	(%rdx,%r10,8), %r10
141c5201c:     	testq	%r10, %r10
141c5201f:     	je	0x141c52042 <.text+0x1c51042>
141c52021:     	notq	%r10
141c52024:     	leaq	(%r10,%r10,2), %r10
141c52028:     	shlq	$0x4, %r10
141c5202c:     	movq	0x10(%rcx,%r10), %r10
141c52031:     	movq	%r10, 0x78(%rsp)
141c52036:     	jmp	0x141c5204b <.text+0x1c5104b>
141c52038:     	movl	$0x0, 0x54(%rsp)
141c52040:     	jmp	0x141c5204b <.text+0x1c5104b>
141c52042:     	movq	$0x0, 0x78(%rsp)
141c5204b:     	movq	0x110(%rdi), %r10
141c52052:     	movq	(%rdx,%r10,8), %r10
141c52056:     	notq	%r10
141c52059:     	leaq	(%r10,%r10,2), %r10
141c5205d:     	shlq	$0x4, %r10
141c52061:     	movq	0x10(%rcx,%r10), %r14
141c52066:     	movq	%r14, 0xa0(%rsp)
141c5206e:     	movq	0x28(%rcx,%r10), %r10
141c52073:     	movq	%r10, 0x98(%rsp)
141c5207b:     	movq	0x140(%rdi), %r10
141c52082:     	cmpq	%r10, %r9
141c52085:     	jbe	0x141c520cb <.text+0x1c510cb>
141c52087:     	movq	0x80(%rax), %r9
141c5208e:     	cmpq	$0x0, (%r9,%r10,8)
141c52093:     	je	0x141c520cb <.text+0x1c510cb>
141c52095:     	movb	$0x1, %r9b
141c52098:     	movl	%r9d, 0x60(%rsp)
141c5209d:     	cmpq	%r10, %r8
141c520a0:     	movzbl	0x53(%rsp), %r9d
141c520a6:     	jbe	0x141c520db <.text+0x1c510db>
141c520a8:     	movq	(%rdx,%r10,8), %rdx
141c520ac:     	testq	%rdx, %rdx
141c520af:     	je	0x141c520db <.text+0x1c510db>
141c520b1:     	notq	%rdx
141c520b4:     	leaq	(%rdx,%rdx,2), %rdx
141c520b8:     	shlq	$0x4, %rdx
141c520bc:     	movq	0x10(%rcx,%rdx), %rcx
141c520c1:     	movq	%rcx, 0x88(%rsp)
141c520c9:     	jmp	0x141c520e7 <.text+0x1c510e7>
141c520cb:     	movl	$0x0, 0x60(%rsp)
141c520d3:     	movzbl	0x53(%rsp), %r9d
141c520d9:     	jmp	0x141c520e7 <.text+0x1c510e7>
141c520db:     	movq	$0x0, 0x88(%rsp)
141c520e7:     	movq	%rsi, %r10
141c520ea:     	movq	0x50(%rax), %rax
141c520ee:     	movq	%rax, 0xc0(%rsp)
141c520f6:     	xorl	%r8d, %r8d
141c520f9:     	movq	%r8, %rax
141c520fc:     	shlq	$0x4, %rax
141c52100:     	movq	0xc0(%rsp), %rcx
141c52108:     	movl	0x8(%rcx,%rax), %ecx
141c5210c:     	jmp	0x141c52328 <.text+0x1c51328>
141c52111:     	nopw	%cs:(%rax,%rax)
141c52120:     	cmpq	%rbx, %r8
141c52123:     	jne	0x141c52325 <.text+0x1c51325>
141c52129:     	nopl	(%rax)
141c52130:     	cmpq	%rbp, %r12
141c52133:     	je	0x141c526bd <.text+0x1c516bd>
141c52139:     	movl	(%r12), %ecx
141c5213d:     	addq	$0x4, %r12
141c52141:     	movq	0x1a8(%r15), %rax
141c52148:     	leaq	(%rcx,%rcx,8), %rcx
141c5214c:     	movq	0x10(%rax,%rcx,8), %rbx
141c52151:     	testq	%rbx, %rbx
141c52154:     	je	0x141c52130 <.text+0x1c51130>
141c52156:     	leaq	(%rax,%rcx,8), %rcx
141c5215a:     	movq	0x40(%rcx), %rdx
141c5215e:     	movq	0x18(%rcx), %rax
141c52162:     	movq	0x38(%rcx), %rcx
141c52166:     	movq	0x118(%rdi), %r8
141c5216d:     	cmpq	%r8, %rdx
141c52170:     	jbe	0x141c5219e <.text+0x1c5119e>
141c52172:     	movq	(%rcx,%r8,8), %r8
141c52176:     	testq	%r8, %r8
141c52179:     	je	0x141c5219e <.text+0x1c5119e>
141c5217b:     	notq	%r8
141c5217e:     	leaq	(%r8,%r8,2), %r8
141c52182:     	shlq	$0x4, %r8
141c52186:     	movq	0x10(%rax,%r8), %r8
141c5218b:     	movq	%r8, 0x70(%rsp)
141c52190:     	movq	0x120(%rdi), %r8
141c52197:     	cmpq	%r8, %rdx
141c5219a:     	ja	0x141c521b3 <.text+0x1c511b3>
141c5219c:     	jmp	0x141c521e2 <.text+0x1c511e2>
141c5219e:     	movq	$0x0, 0x70(%rsp)
141c521a7:     	movq	0x120(%rdi), %r8
141c521ae:     	cmpq	%r8, %rdx
141c521b1:     	jbe	0x141c521e2 <.text+0x1c511e2>
141c521b3:     	movq	(%rcx,%r8,8), %r8
141c521b7:     	testq	%r8, %r8
141c521ba:     	je	0x141c521e2 <.text+0x1c511e2>
141c521bc:     	notq	%r8
141c521bf:     	leaq	(%r8,%r8,2), %r8
141c521c3:     	shlq	$0x4, %r8
141c521c7:     	movq	0x10(%rax,%r8), %r10
141c521cc:     	movb	$0x1, %r8b
141c521cf:     	movl	%r8d, 0x5c(%rsp)
141c521d4:     	movq	0x128(%rdi), %r8
141c521db:     	cmpq	%r8, %rdx
141c521de:     	jbe	0x141c52222 <.text+0x1c51222>
141c521e0:     	jmp	0x141c521f6 <.text+0x1c511f6>
141c521e2:     	movl	$0x0, 0x5c(%rsp)
141c521ea:     	movq	0x128(%rdi), %r8
141c521f1:     	cmpq	%r8, %rdx
141c521f4:     	jbe	0x141c52222 <.text+0x1c51222>
141c521f6:     	movq	(%rcx,%r8,8), %r8
141c521fa:     	testq	%r8, %r8
141c521fd:     	je	0x141c52222 <.text+0x1c51222>
141c521ff:     	notq	%r8
141c52202:     	leaq	(%r8,%r8,2), %r8
141c52206:     	shlq	$0x4, %r8
141c5220a:     	movq	0x10(%rax,%r8), %r8
141c5220f:     	movq	%r8, 0x68(%rsp)
141c52214:     	movq	0x130(%rdi), %r8
141c5221b:     	cmpq	%r8, %rdx
141c5221e:     	jbe	0x141c5226e <.text+0x1c5126e>
141c52220:     	jmp	0x141c52237 <.text+0x1c51237>
141c52222:     	movq	$0x0, 0x68(%rsp)
141c5222b:     	movq	0x130(%rdi), %r8
141c52232:     	cmpq	%r8, %rdx
141c52235:     	jbe	0x141c5226e <.text+0x1c5126e>
141c52237:     	movq	(%rcx,%r8,8), %r8
141c5223b:     	testq	%r8, %r8
141c5223e:     	je	0x141c5226e <.text+0x1c5126e>
141c52240:     	notq	%r8
141c52243:     	leaq	(%r8,%r8,2), %r8
141c52247:     	shlq	$0x4, %r8
141c5224b:     	movq	0x10(%rax,%r8), %r8
141c52250:     	movq	%r8, 0x80(%rsp)
141c52258:     	movb	$0x1, %r8b
141c5225b:     	movl	%r8d, 0x58(%rsp)
141c52260:     	movq	0x138(%rdi), %r8
141c52267:     	cmpq	%r8, %rdx
141c5226a:     	jbe	0x141c522aa <.text+0x1c512aa>
141c5226c:     	jmp	0x141c52282 <.text+0x1c51282>
141c5226e:     	movl	$0x0, 0x58(%rsp)
141c52276:     	movq	0x138(%rdi), %r8
141c5227d:     	cmpq	%r8, %rdx
141c52280:     	jbe	0x141c522aa <.text+0x1c512aa>
141c52282:     	movq	(%rcx,%r8,8), %r8
141c52286:     	testq	%r8, %r8
141c52289:     	je	0x141c522aa <.text+0x1c512aa>
141c5228b:     	notq	%r8
141c5228e:     	leaq	(%r8,%r8,2), %r8
141c52292:     	shlq	$0x4, %r8
141c52296:     	movq	0x10(%rax,%r8), %r8
141c5229b:     	movq	%r8, 0x78(%rsp)
141c522a0:     	movb	$0x1, %r8b
141c522a3:     	movl	%r8d, 0x54(%rsp)
141c522a8:     	jmp	0x141c522b2 <.text+0x1c512b2>
141c522aa:     	movl	$0x0, 0x54(%rsp)
141c522b2:     	movq	0x110(%rdi), %r8
141c522b9:     	movq	(%rcx,%r8,8), %r8
141c522bd:     	notq	%r8
141c522c0:     	leaq	(%r8,%r8,2), %r8
141c522c4:     	shlq	$0x4, %r8
141c522c8:     	movq	0x10(%rax,%r8), %rsi
141c522cd:     	movq	%rsi, 0xa0(%rsp)
141c522d5:     	movq	0x28(%rax,%r8), %r8
141c522da:     	movq	%r8, 0x98(%rsp)
141c522e2:     	movq	0x140(%rdi), %r8
141c522e9:     	cmpq	%r8, %rdx
141c522ec:     	jbe	0x141c5231a <.text+0x1c5131a>
141c522ee:     	movq	(%rcx,%r8,8), %rcx
141c522f2:     	testq	%rcx, %rcx
141c522f5:     	je	0x141c5231a <.text+0x1c5131a>
141c522f7:     	notq	%rcx
141c522fa:     	leaq	(%rcx,%rcx,2), %rcx
141c522fe:     	shlq	$0x4, %rcx
141c52302:     	movq	0x10(%rax,%rcx), %rax
141c52307:     	movq	%rax, 0x88(%rsp)
141c5230f:     	movb	$0x1, %al
141c52311:     	movl	%eax, 0x60(%rsp)
141c52315:     	xorl	%r8d, %r8d
141c52318:     	jmp	0x141c52325 <.text+0x1c51325>
141c5231a:     	xorl	%r8d, %r8d
141c5231d:     	movl	$0x0, 0x60(%rsp)
141c52325:     	movl	%r8d, %ecx
141c52328:     	leaq	(,%rcx,8), %rax
141c52330:     	leaq	(%rax,%rax,2), %rsi
141c52334:     	movq	0xa0(%rsp), %rax
141c5233c:     	addq	%rsi, %rax
141c5233f:     	movq	%rax, 0xd8(%rsp)
141c52347:     	movq	0x98(%rsp), %rax
141c5234f:     	leaq	(%rax,%rcx,4), %rax
141c52353:     	movq	%rax, 0xd0(%rsp)
141c5235b:     	leaq	(%rcx,%rcx,2), %rax
141c5235f:     	movq	0x70(%rsp), %rdx
141c52364:     	leaq	(%rdx,%rax,4), %r13
141c52368:     	shlq	$0x4, %rax
141c5236c:     	addq	%r10, %rax
141c5236f:     	testb	$0x1, 0x5c(%rsp)
141c52374:     	movl	$0x0, %r11d
141c5237a:     	cmoveq	%r11, %rax
141c5237e:     	leaq	(%rcx,%rcx,4), %rdx
141c52382:     	movq	0x68(%rsp), %rdi
141c52387:     	leaq	(%rdi,%rdx,4), %r14
141c5238b:     	addq	0x80(%rsp), %rsi
141c52393:     	testb	$0x1, 0x58(%rsp)
141c52398:     	cmoveq	%r11, %rsi
141c5239c:     	movq	0x78(%rsp), %rdx
141c523a1:     	leaq	(%rdx,%rcx,8), %rdi
141c523a5:     	testb	$0x1, 0x54(%rsp)
141c523aa:     	cmoveq	%r11, %rdi
141c523ae:     	testb	$0x1, 0x60(%rsp)
141c523b3:     	movl	$0x0, %ecx
141c523b8:     	cmovneq	0x88(%rsp), %rcx
141c523c1:     	incq	%r8
141c523c4:     	movq	0xe0(%rsp), %rdx
141c523cc:     	movq	%rdx, 0x118(%rsp)
141c523d4:     	incq	%rdx
141c523d7:     	movq	%rdx, 0xe0(%rsp)
141c523df:     	movq	%rax, 0x120(%rsp)
141c523e7:     	movq	%rcx, 0xe8(%rsp)
141c523ef:     	ucomiss	0x4(%r13), %xmm7
141c523f4:     	jae	0x141c525a0 <.text+0x1c515a0>
141c523fa:     	movq	%r8, 0x108(%rsp)
141c52402:     	movq	%r10, 0x110(%rsp)
141c5240a:     	movq	0xb8(%rsp), %rcx
141c52412:     	callq	0x140949c10 <.text+0x948c10>
141c52417:     	movaps	%xmm1, %xmm6
141c5241a:     	divss	%xmm8, %xmm6
141c5241f:     	movsd	0x8(%r14), %xmm10
141c52425:     	movl	0x90(%rsp), %eax
141c5242c:     	movq	0xb0(%rsp), %rcx
141c52434:     	movl	%eax, (%rcx)
141c52436:     	leaq	0x118(%rsp), %rax
141c5243e:     	movq	%rax, 0x128(%rsp)
141c52446:     	leaq	-0xec4d8d(%rip), %rax   # 0x140d8d6c0 <.text+0xd8c6c0>
141c5244d:     	movq	%rax, 0x130(%rsp)
141c52455:     	leaq	0x12c223c(%rip), %rax   # 0x142f14698
141c5245c:     	movq	%rax, 0x140(%rsp)
141c52464:     	movq	$0x1, 0x148(%rsp)
141c52470:     	leaq	0x128(%rsp), %rax
141c52478:     	movq	%rax, 0x150(%rsp)
141c52480:     	movq	$0x1, 0x158(%rsp)
141c5248c:     	movq	$0x0, 0x160(%rsp)
141c52498:     	leaq	0x218(%rsp), %rcx
141c524a0:     	leaq	0x140(%rsp), %rdx
141c524a8:     	callq	0x140d79f90 <.text+0xd78f90>
141c524ad:     	movl	$0x0, 0x128(%rsp)
141c524b8:     	cmpq	$0x0, 0xe8(%rsp)
141c524c1:     	movaps	%xmm9, %xmm0
141c524c5:     	je	0x141c524ca <.text+0x1c514ca>
141c524c7:     	xorps	%xmm0, %xmm0
141c524ca:     	movshdup	%xmm10, %xmm1           # xmm1 = xmm10[1,1,3,3]
141c524cf:     	movaps	%xmm10, %xmm3
141c524d3:     	divss	%xmm6, %xmm3
141c524d7:     	divss	%xmm6, %xmm1
141c524db:     	movss	%xmm0, 0x38(%rsp)
141c524e1:     	movss	%xmm0, 0x30(%rsp)
141c524e7:     	leaq	0x128(%rsp), %rbp
141c524ef:     	movq	%rbp, 0x28(%rsp)
141c524f4:     	movss	%xmm1, 0x20(%rsp)
141c524fa:     	movb	$0x0, 0x40(%rsp)
141c524ff:     	leaq	0x140(%rsp), %r15
141c52507:     	movq	%r15, %rcx
141c5250a:     	movq	0xa8(%rsp), %rdx
141c52512:     	leaq	0x218(%rsp), %r14
141c5251a:     	movq	%r14, %r8
141c5251d:     	callq	0x142289f80 <.text+0x2288f80>
141c52522:     	movq	%rbp, %rcx
141c52525:     	movq	%r15, %rdx
141c52528:     	callq	0x140686690 <.text+0x685690>
141c5252d:     	movl	$0xa, %r9d
141c52533:     	movq	%r14, %rcx
141c52536:     	movq	%rbp, %rdx
141c52539:     	leaq	0x12c2141(%rip), %r8    # 0x142f14681
141c52540:     	callq	0x14068bee0 <.text+0x68aee0>
141c52545:     	movq	%r14, %rcx
141c52548:     	callq	0x14068ea60 <.text+0x68da60>
141c5254d:     	movss	0x4(%r13), %xmm0
141c52553:     	movss	%xmm0, 0xb0(%rax)
141c5255b:     	movl	0x94(%rsp), %ecx
141c52562:     	movq	0xd0(%rsp), %rbp
141c5256a:     	movl	%ecx, (%rbp)
141c5256d:     	movq	0xd8(%rsp), %r15
141c52575:     	movsd	%xmm10, (%r15)
141c5257a:     	movq	%rax, %rcx
141c5257d:     	callq	0x140683cb0 <.text+0x682cb0>
141c52582:     	testq	%rax, %rax
141c52585:     	je	0x141c525c0 <.text+0x1c515c0>
141c52587:     	movsd	(%rax), %xmm0
141c5258b:     	movsd	0x8(%rax), %xmm10
141c52591:     	movsldup	%xmm6, %xmm1            # xmm1 = xmm6[0,0,2,2]
141c52595:     	mulps	%xmm1, %xmm10
141c52599:     	mulps	%xmm1, %xmm0
141c5259c:     	jmp	0x141c525c3 <.text+0x1c515c3>
141c5259e:     	nop
141c525a0:     	movq	0xc8(%rsp), %rdi
141c525a8:     	testb	$0x1, %r9b
141c525ac:     	jne	0x141c52120 <.text+0x1c51120>
141c525b2:     	jmp	0x141c51e20 <.text+0x1c50e20>
141c525b7:     	nopw	(%rax,%rax)
141c525c0:     	xorps	%xmm0, %xmm0
141c525c3:     	movlps	%xmm10, 0x8(%r15)
141c525c8:     	movl	0x94(%rsp), %eax
141c525cf:     	movl	%eax, (%rbp)
141c525d2:     	movlps	%xmm0, 0x10(%r15)
141c525d7:     	testq	%rsi, %rsi
141c525da:     	movq	0xf8(%rsp), %rbp
141c525e2:     	movq	0xf0(%rsp), %r15
141c525ea:     	je	0x141c52611 <.text+0x1c51611>
141c525ec:     	leaq	0xe8(%rsp), %rax
141c525f4:     	movq	%rax, 0x20(%rsp)
141c525f9:     	movl	$0x4, %r8d
141c525ff:     	movq	%r14, %rcx
141c52602:     	leaq	0x12c2082(%rip), %rdx   # 0x142f1468b
141c52609:     	movq	%rsi, %r9
141c5260c:     	callq	0x14228ab40 <.text+0x2289b40>
141c52611:     	leaq	0x120(%rsp), %rax
141c52619:     	movq	%rax, 0x140(%rsp)
141c52621:     	leaq	0x67(%rsp), %rax
141c52626:     	movq	%rax, 0x148(%rsp)
141c5262e:     	movq	0x300(%rsp), %rax
141c52636:     	movq	%rax, 0x150(%rsp)
141c5263e:     	movq	0x100(%rsp), %rax
141c52646:     	movq	%rax, 0x158(%rsp)
141c5264e:     	movq	%r13, 0x160(%rsp)
141c52656:     	movl	$0x5, %r8d
141c5265c:     	movq	%r14, %rcx
141c5265f:     	leaq	0x12c2042(%rip), %rdx   # 0x142f146a8
141c52666:     	leaq	0x140(%rsp), %r9
141c5266e:     	callq	0x14228bb90 <.text+0x228ab90>
141c52673:     	testq	%rdi, %rdi
141c52676:     	je	0x141c52690 <.text+0x1c51690>
141c52678:     	movl	$0x4, %r8d
141c5267e:     	movq	%r14, %rcx
141c52681:     	leaq	0x12c2025(%rip), %rdx   # 0x142f146ad
141c52688:     	movq	%rdi, %r9
141c5268b:     	callq	0x14228bfd0 <.text+0x228afd0>
141c52690:     	movq	0xc8(%rsp), %rdi
141c52698:     	movzbl	0x53(%rsp), %r9d
141c5269e:     	movq	0x110(%rsp), %r10
141c526a6:     	movq	0x108(%rsp), %r8
141c526ae:     	testb	$0x1, %r9b
141c526b2:     	je	0x141c51e20 <.text+0x1c50e20>
141c526b8:     	jmp	0x141c52120 <.text+0x1c51120>
141c526bd:     	movaps	0x240(%rsp), %xmm6
141c526c5:     	movaps	0x250(%rsp), %xmm7
141c526cd:     	movaps	0x260(%rsp), %xmm8
141c526d6:     	movaps	0x270(%rsp), %xmm9
141c526df:     	movaps	0x280(%rsp), %xmm10
141c526e8:     	addq	$0x298, %rsp            # imm = 0x298
141c526ef:     	popq	%rbx
141c526f0:     	popq	%rbp
141c526f1:     	popq	%rdi
141c526f2:     	popq	%rsi
141c526f3:     	popq	%r12
141c526f5:     	popq	%r13
141c526f7:     	popq	%r14
141c526f9:     	popq	%r15
141c526fb:     	retq
141c526fc:     	int3
141c526fd:     	int3
141c526fe:     	int3
141c526ff:     	int3
141c52700:     	pushq	%r15
141c52702:     	pushq	%r14
141c52704:     	pushq	%r13
141c52706:     	pushq	%r12
141c52708:     	pushq	%rsi
141c52709:     	pushq	%rdi
141c5270a:     	pushq	%rbp
141c5270b:     	pushq	%rbx
141c5270c:     	subq	$0x178, %rsp            # imm = 0x178
141c52713:     	movq	%r8, %rdi
141c52716:     	movq	%rcx, %rsi
141c52719:     	movq	%r9, 0x118(%rsp)
141c52721:     	xorl	%eax, %eax
141c52723:     	testq	%r9, %r9
141c52726:     	sete	%al
141c52729:     	leaq	0x12c1f81(%rip), %rcx   # 0x142f146b1
141c52730:     	leaq	0x12c1f8c(%rip), %rbx   # 0x142f146c3
141c52737:     	cmoveq	%rcx, %rbx
141c5273b:     	movl	(%rsi), %r13d
141c5273e:     	leal	-0xc4(%r13), %ecx
141c52745:     	cmpl	$0x9, %ecx
141c52748:     	movl	$0x4, %r8d
141c5274e:     	cmovbl	%ecx, %r8d
141c52752:     	leaq	0xe(,%rax,4), %r14
141c5275a:     	leaq	0x12c2113(%rip), %rax   # 0x142f14874
141c52761:     	movslq	(%rax,%r8,4), %rcx
141c52765:     	addq	%rax, %rcx
141c52768:     	jmpq	*%rcx
141c5276a:     	leaq	0x18(%rsi), %rax
141c5276e:     	xorl	%ecx, %ecx
141c52770:     	cmpq	0x8(%rsi), %rcx
141c52774:     	jno	0x141c52fce <.text+0x1c51fce>
141c5277a:     	leaq	0x20(%rsi), %rcx
141c5277e:     	jmp	0x141c53053 <.text+0x1c52053>
141c52783:     	movq	0x8(%rdi), %rbx
141c52787:     	leaq	0x12c1f62(%rip), %rax   # 0x142f146f0
141c5278e:     	movq	%rax, 0xc0(%rsp)
141c52796:     	movq	$0x4, 0xc8(%rsp)
141c527a2:     	leaq	0xc0(%rsp), %rcx
141c527aa:     	callq	0x140683c10 <.text+0x682c10>
141c527af:     	movq	%rax, 0x40(%rsp)
141c527b4:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
141c527be:     	movq	%rax, 0x48(%rsp)
141c527c3:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
141c527cd:     	movq	%rax, 0x50(%rsp)
141c527d2:     	leaq	0x40(%rsp), %rcx
141c527d7:     	callq	0x14228d080 <.text+0x228c080>
141c527dc:     	movq	%rbx, 0x40(%rsp)
141c527e1:     	movq	%rax, 0x48(%rsp)
141c527e6:     	leaq	0x40(%rsp), %rcx
141c527eb:     	callq	0x14228d1f0 <.text+0x228c1f0>
141c527f0:     	leaq	0x40(%rsp), %rbx
141c527f5:     	movq	%rbx, %rcx
141c527f8:     	movq	%rdi, %rdx
141c527fb:     	movq	%rax, %r8
141c527fe:     	callq	0x1406861b0 <.text+0x6851b0>
141c52803:     	leaq	0x12c1eff(%rip), %r8    # 0x142f14709
141c5280a:     	leaq	0x150(%rsp), %rcx
141c52812:     	movl	$0xf, %r9d
141c52818:     	movq	%rbx, %rdx
141c5281b:     	callq	0x14068bee0 <.text+0x68aee0>
141c52820:     	leaq	0x12c1f88(%rip), %rdi   # 0x142f147af
141c52827:     	movl	$0x1, %r15d
141c5282d:     	movl	$0x1, %r14d
141c52833:     	movq	%rdi, %rbx
141c52836:     	cmpl	$0xc3, %r13d
141c5283d:     	je	0x141c5286f <.text+0x1c5186f>
141c5283f:     	movq	0x4(%rsi), %rax
141c52843:     	movl	%r13d, 0x40(%rsp)
141c52848:     	movq	%rax, 0x44(%rsp)
141c5284d:     	leaq	0x40(%rsp), %rcx
141c52852:     	callq	0x14089d490 <.text+0x89c490>
141c52857:     	testq	%rax, %rax
141c5285a:     	leaq	0x12c1f4f(%rip), %rbx   # 0x142f147b0
141c52861:     	cmovneq	%rax, %rbx
141c52865:     	movl	$0x3, %r14d
141c5286b:     	cmovneq	%rdx, %r14
141c5286f:     	movl	0xc(%rsi), %eax
141c52872:     	movq	%rdi, %r13
141c52875:     	cmpl	$0xc3, %eax
141c5287a:     	je	0x141c528ab <.text+0x1c518ab>
141c5287c:     	movq	0x10(%rsi), %rcx
141c52880:     	movl	%eax, 0x40(%rsp)
141c52884:     	movq	%rcx, 0x44(%rsp)
141c52889:     	leaq	0x40(%rsp), %rcx
141c5288e:     	callq	0x14089d490 <.text+0x89c490>
141c52893:     	testq	%rax, %rax
141c52896:     	leaq	0x12c1f13(%rip), %r13   # 0x142f147b0
141c5289d:     	cmovneq	%rax, %r13
141c528a1:     	movl	$0x3, %r15d
141c528a7:     	cmovneq	%rdx, %r15
141c528ab:     	movq	%r15, 0x120(%rsp)
141c528b3:     	movl	0x18(%rsi), %eax
141c528b6:     	movl	$0x1, %ebp
141c528bb:     	movl	$0x1, %r12d
141c528c1:     	movq	%rdi, %r15
141c528c4:     	cmpl	$0xc3, %eax
141c528c9:     	je	0x141c528fa <.text+0x1c518fa>
141c528cb:     	movq	0x1c(%rsi), %rcx
141c528cf:     	movl	%eax, 0x40(%rsp)
141c528d3:     	movq	%rcx, 0x44(%rsp)
141c528d8:     	leaq	0x40(%rsp), %rcx
141c528dd:     	callq	0x14089d490 <.text+0x89c490>
141c528e2:     	testq	%rax, %rax
141c528e5:     	leaq	0x12c1ec4(%rip), %r15   # 0x142f147b0
141c528ec:     	cmovneq	%rax, %r15
141c528f0:     	movl	$0x3, %r12d
141c528f6:     	cmovneq	%rdx, %r12
141c528fa:     	movl	0x24(%rsi), %eax
141c528fd:     	cmpl	$0xc3, %eax
141c52902:     	je	0x141c52932 <.text+0x1c51932>
141c52904:     	movq	0x28(%rsi), %rcx
141c52908:     	movl	%eax, 0x40(%rsp)
141c5290c:     	movq	%rcx, 0x44(%rsp)
141c52911:     	leaq	0x40(%rsp), %rcx
141c52916:     	callq	0x14089d490 <.text+0x89c490>
141c5291b:     	testq	%rax, %rax
141c5291e:     	leaq	0x12c1e8b(%rip), %rdi   # 0x142f147b0
141c52925:     	cmovneq	%rax, %rdi
141c52929:     	movl	$0x3, %ebp
141c5292e:     	cmovneq	%rdx, %rbp
141c52932:     	leaq	0x12c1ddf(%rip), %rdx   # 0x142f14718
141c52939:     	movq	%rdx, 0x40(%rsp)
141c5293e:     	movq	$0x7, 0x48(%rsp)
141c52947:     	movq	%rbx, 0x50(%rsp)
141c5294c:     	movq	%r14, 0x58(%rsp)
141c52951:     	leaq	0x12c1dc7(%rip), %rax   # 0x142f1471f
141c52958:     	movq	%rax, 0x60(%rsp)
141c5295d:     	movq	$0x4, 0x68(%rsp)
141c52966:     	movq	%r13, 0x70(%rsp)
141c5296b:     	movq	0x120(%rsp), %rax
141c52973:     	movq	%rax, 0x78(%rsp)
141c52978:     	leaq	0x12c1da4(%rip), %rax   # 0x142f14723
141c5297f:     	movq	%rax, 0x80(%rsp)
141c52987:     	movq	$0x4, 0x88(%rsp)
141c52993:     	movq	%r15, 0x90(%rsp)
141c5299b:     	movq	%r12, 0x98(%rsp)
141c529a3:     	leaq	0x12c1d7d(%rip), %rax   # 0x142f14727
141c529aa:     	movq	%rax, 0xa0(%rsp)
141c529b2:     	movq	$0x5, 0xa8(%rsp)
141c529be:     	movq	%rdi, 0xb0(%rsp)
141c529c6:     	movq	%rbp, 0xb8(%rsp)
141c529ce:     	movups	0x150(%rsp), %xmm0
141c529d6:     	movups	0x160(%rsp), %xmm1
141c529de:     	movaps	%xmm0, 0xc0(%rsp)
141c529e6:     	movaps	%xmm1, 0xd0(%rsp)
141c529ee:     	movq	0x170(%rsp), %rax
141c529f6:     	movq	%rax, 0xe0(%rsp)
141c529fe:     	movl	$0x41900000, 0xe8(%rsp) # imm = 0x41900000
