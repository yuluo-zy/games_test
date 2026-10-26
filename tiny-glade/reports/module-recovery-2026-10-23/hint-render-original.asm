
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
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
141c52a09:     	leaq	0x40(%rsp), %rax
141c52a0e:     	movq	%rax, 0xf0(%rsp)
141c52a16:     	movq	$0x4, 0xf8(%rsp)
141c52a22:     	movl	$0x41900000, 0x3c(%rsp) # imm = 0x41900000
141c52a2a:     	movq	%rbx, 0x100(%rsp)
141c52a32:     	movq	%r14, 0x108(%rsp)
141c52a3a:     	leaq	0x3c(%rsp), %rbx
141c52a3f:     	movq	%rbx, 0x110(%rsp)
141c52a47:     	leaq	0xc0(%rsp), %rdi
141c52a4f:     	leaq	0x100(%rsp), %rsi
141c52a57:     	movl	$0x7, %r8d
141c52a5d:     	movq	%rdi, %rcx
141c52a60:     	movq	%rsi, %r9
141c52a63:     	callq	0x14228b3d0 <.text+0x228a3d0>
141c52a68:     	movq	0x60(%rsp), %rdx
141c52a6d:     	movq	0x68(%rsp), %r8
141c52a72:     	movups	0x70(%rsp), %xmm0
141c52a77:     	movups	%xmm0, 0x100(%rsp)
141c52a7f:     	movq	%rbx, 0x110(%rsp)
141c52a87:     	movq	%rdi, %rcx
141c52a8a:     	movq	%rsi, %r9
141c52a8d:     	callq	0x14228b3d0 <.text+0x228a3d0>
141c52a92:     	movq	0x80(%rsp), %rdx
141c52a9a:     	movq	0x88(%rsp), %r8
141c52aa2:     	movups	0x90(%rsp), %xmm0
141c52aaa:     	movups	%xmm0, 0x100(%rsp)
141c52ab2:     	movq	%rbx, 0x110(%rsp)
141c52aba:     	movq	%rdi, %rcx
141c52abd:     	movq	%rsi, %r9
141c52ac0:     	callq	0x14228b3d0 <.text+0x228a3d0>
141c52ac5:     	movq	0xa0(%rsp), %rdx
141c52acd:     	movq	0xa8(%rsp), %r8
141c52ad5:     	movups	0xb0(%rsp), %xmm0
141c52add:     	movups	%xmm0, 0x100(%rsp)
141c52ae5:     	movq	%rbx, 0x110(%rsp)
141c52aed:     	movq	%rdi, %rcx
141c52af0:     	movq	%rsi, %r9
141c52af3:     	callq	0x14228b3d0 <.text+0x228a3d0>
141c52af8:     	jmp	0x141c53088 <.text+0x1c52088>
141c52afd:     	movq	0x8(%rdi), %r14
141c52b01:     	leaq	0x12c1be8(%rip), %rax   # 0x142f146f0
141c52b08:     	movq	%rax, 0xc0(%rsp)
141c52b10:     	movq	$0x4, 0xc8(%rsp)
141c52b1c:     	leaq	0xc0(%rsp), %rbx
141c52b24:     	movq	%rbx, %rcx
141c52b27:     	callq	0x140683c10 <.text+0x682c10>
141c52b2c:     	movq	%rax, 0x40(%rsp)
141c52b31:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
141c52b3b:     	movq	%rax, 0x48(%rsp)
141c52b40:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
141c52b4a:     	movq	%rax, 0x50(%rsp)
141c52b4f:     	leaq	0x40(%rsp), %rcx
141c52b54:     	callq	0x14228d080 <.text+0x228c080>
141c52b59:     	movq	%r14, 0x40(%rsp)
141c52b5e:     	movq	%rax, 0x48(%rsp)
141c52b63:     	leaq	0x40(%rsp), %rcx
141c52b68:     	callq	0x14228d1f0 <.text+0x228c1f0>
141c52b6d:     	leaq	0x40(%rsp), %r14
141c52b72:     	movq	%r14, %rcx
141c52b75:     	movq	%rdi, %rdx
141c52b78:     	movq	%rax, %r8
141c52b7b:     	callq	0x1406861b0 <.text+0x6851b0>
141c52b80:     	leaq	0x12c1b6d(%rip), %r8    # 0x142f146f4
141c52b87:     	leaq	0x128(%rsp), %rcx
141c52b8f:     	movl	$0x12, %r9d
141c52b95:     	movq	%r14, %rdx
141c52b98:     	callq	0x14068bee0 <.text+0x68aee0>
141c52b9d:     	movl	0x4(%rsi), %eax
141c52ba0:     	cmpl	$0xc3, %eax
141c52ba5:     	jne	0x141c52ef6 <.text+0x1c51ef6>
141c52bab:     	leaq	0x12c1bfd(%rip), %rcx   # 0x142f147af
141c52bb2:     	movl	$0x1, %eax
141c52bb7:     	jmp	0x141c52f24 <.text+0x1c51f24>
141c52bbc:     	addq	$0x4, %rsi
141c52bc0:     	movq	0x8(%rdi), %rbx
141c52bc4:     	leaq	0x12c1b25(%rip), %rax   # 0x142f146f0
141c52bcb:     	movq	%rax, 0xc0(%rsp)
141c52bd3:     	movq	$0x4, 0xc8(%rsp)
141c52bdf:     	leaq	0xc0(%rsp), %rcx
141c52be7:     	callq	0x140683c10 <.text+0x682c10>
141c52bec:     	movq	%rax, 0x40(%rsp)
141c52bf1:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
141c52bfb:     	movq	%rax, 0x48(%rsp)
141c52c00:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
141c52c0a:     	movq	%rax, 0x50(%rsp)
141c52c0f:     	leaq	0x40(%rsp), %rcx
141c52c14:     	callq	0x14228d080 <.text+0x228c080>
141c52c19:     	movq	%rbx, 0x40(%rsp)
141c52c1e:     	movq	%rax, 0x48(%rsp)
141c52c23:     	leaq	0x40(%rsp), %rcx
141c52c28:     	callq	0x14228d1f0 <.text+0x228c1f0>
141c52c2d:     	leaq	0xc0(%rsp), %rbx
141c52c35:     	movq	%rbx, %rcx
141c52c38:     	movq	%rdi, %rdx
141c52c3b:     	movq	%rax, %r8
141c52c3e:     	callq	0x1406861b0 <.text+0x6851b0>
141c52c43:     	leaq	0x12c1aaa(%rip), %r8    # 0x142f146f4
141c52c4a:     	leaq	0x40(%rsp), %rdi
141c52c4f:     	movl	$0x12, %r9d
141c52c55:     	movq	%rdi, %rcx
141c52c58:     	movq	%rbx, %rdx
141c52c5b:     	callq	0x14068bee0 <.text+0x68aee0>
141c52c60:     	leaq	0x12c1a9f(%rip), %rdx   # 0x142f14706
141c52c67:     	movl	$0x3, %r8d
141c52c6d:     	movq	%rdi, %rcx
141c52c70:     	movq	%rsi, %r9
141c52c73:     	callq	0x14228bdb0 <.text+0x228adb0>
141c52c78:     	jmp	0x141c53088 <.text+0x1c52088>
141c52c7d:     	addq	$0x8, %rsi
141c52c81:     	movq	0x8(%rdi), %r15
141c52c85:     	movq	$0x0, 0xc0(%rsp)
141c52c91:     	leaq	0xc0(%rsp), %rcx
141c52c99:     	movq	%rdx, %r12
141c52c9c:     	callq	0x1400885d0 <.text+0x875d0>
141c52ca1:     	movq	%rax, 0x40(%rsp)
141c52ca6:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
141c52cb0:     	movq	%rax, 0x48(%rsp)
141c52cb5:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
141c52cbf:     	movq	%rax, 0x50(%rsp)
141c52cc4:     	leaq	0x40(%rsp), %rcx
141c52cc9:     	callq	0x14228d080 <.text+0x228c080>
141c52cce:     	movq	%r15, 0x40(%rsp)
141c52cd3:     	movq	%rax, 0x48(%rsp)
141c52cd8:     	leaq	0x40(%rsp), %rcx
141c52cdd:     	callq	0x14228d1f0 <.text+0x228c1f0>
141c52ce2:     	leaq	0xc0(%rsp), %r15
141c52cea:     	movq	%r15, %rcx
141c52ced:     	movq	%rdi, %rdx
141c52cf0:     	movq	%rax, %r8
141c52cf3:     	callq	0x1406861b0 <.text+0x6851b0>
141c52cf8:     	leaq	0x40(%rsp), %rdi
141c52cfd:     	movq	%rdi, %rcx
141c52d00:     	movq	%r15, %rdx
141c52d03:     	movq	%rbx, %r8
141c52d06:     	movq	%r14, %r9
141c52d09:     	callq	0x14068bee0 <.text+0x68aee0>
141c52d0e:     	movq	%rsi, 0xc0(%rsp)
141c52d16:     	movq	%r12, 0xc8(%rsp)
141c52d1e:     	leaq	0x118(%rsp), %rax
141c52d26:     	movq	%rax, 0xd0(%rsp)
141c52d2e:     	leaq	0x12c1a73(%rip), %rdx   # 0x142f147a8
141c52d35:     	leaq	0xc0(%rsp), %r9
141c52d3d:     	movl	$0x7, %r8d
141c52d43:     	movq	%rdi, %rcx
141c52d46:     	callq	0x14228b6d0 <.text+0x228a6d0>
141c52d4b:     	jmp	0x141c53088 <.text+0x1c52088>
141c52d50:     	addq	$0x8, %rsi
141c52d54:     	movq	0x8(%rdi), %r15
141c52d58:     	movq	$0x0, 0xc0(%rsp)
141c52d64:     	leaq	0xc0(%rsp), %rcx
141c52d6c:     	movq	%rdx, %r12
141c52d6f:     	callq	0x1400885d0 <.text+0x875d0>
141c52d74:     	movq	%rax, 0x40(%rsp)
141c52d79:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
141c52d83:     	movq	%rax, 0x48(%rsp)
141c52d88:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
141c52d92:     	movq	%rax, 0x50(%rsp)
141c52d97:     	leaq	0x40(%rsp), %rcx
141c52d9c:     	callq	0x14228d080 <.text+0x228c080>
141c52da1:     	movq	%r15, 0x40(%rsp)
141c52da6:     	movq	%rax, 0x48(%rsp)
141c52dab:     	leaq	0x40(%rsp), %rcx
141c52db0:     	callq	0x14228d1f0 <.text+0x228c1f0>
141c52db5:     	leaq	0xc0(%rsp), %r15
141c52dbd:     	movq	%r15, %rcx
141c52dc0:     	movq	%rdi, %rdx
141c52dc3:     	movq	%rax, %r8
141c52dc6:     	callq	0x1406861b0 <.text+0x6851b0>
141c52dcb:     	leaq	0x40(%rsp), %rdi
141c52dd0:     	movq	%rdi, %rcx
141c52dd3:     	movq	%r15, %rdx
141c52dd6:     	movq	%rbx, %r8
141c52dd9:     	movq	%r14, %r9
141c52ddc:     	callq	0x14068bee0 <.text+0x68aee0>
141c52de1:     	movq	%rsi, 0xc0(%rsp)
141c52de9:     	movq	%r12, 0xc8(%rsp)
141c52df1:     	leaq	0x118(%rsp), %rax
141c52df9:     	movq	%rax, 0xd0(%rsp)
141c52e01:     	leaq	0x12c19a0(%rip), %rdx   # 0x142f147a8
141c52e08:     	leaq	0xc0(%rsp), %r9
141c52e10:     	movl	$0x7, %r8d
141c52e16:     	movq	%rdi, %rcx
141c52e19:     	callq	0x14228ae20 <.text+0x2289e20>
141c52e1e:     	jmp	0x141c53088 <.text+0x1c52088>
141c52e23:     	addq	$0x8, %rsi
141c52e27:     	movq	0x8(%rdi), %r15
141c52e2b:     	movq	$0x0, 0xc0(%rsp)
141c52e37:     	leaq	0xc0(%rsp), %rcx
141c52e3f:     	movq	%rdx, %r12
141c52e42:     	callq	0x1400885d0 <.text+0x875d0>
141c52e47:     	movq	%rax, 0x40(%rsp)
141c52e4c:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
141c52e56:     	movq	%rax, 0x48(%rsp)
141c52e5b:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
141c52e65:     	movq	%rax, 0x50(%rsp)
141c52e6a:     	leaq	0x40(%rsp), %rcx
141c52e6f:     	callq	0x14228d080 <.text+0x228c080>
141c52e74:     	movq	%r15, 0x40(%rsp)
141c52e79:     	movq	%rax, 0x48(%rsp)
141c52e7e:     	leaq	0x40(%rsp), %rcx
141c52e83:     	callq	0x14228d1f0 <.text+0x228c1f0>
141c52e88:     	leaq	0xc0(%rsp), %r15
141c52e90:     	movq	%r15, %rcx
141c52e93:     	movq	%rdi, %rdx
141c52e96:     	movq	%rax, %r8
141c52e99:     	callq	0x1406861b0 <.text+0x6851b0>
141c52e9e:     	leaq	0x40(%rsp), %rdi
141c52ea3:     	movq	%rdi, %rcx
141c52ea6:     	movq	%r15, %rdx
141c52ea9:     	movq	%rbx, %r8
141c52eac:     	movq	%r14, %r9
141c52eaf:     	callq	0x14068bee0 <.text+0x68aee0>
141c52eb4:     	movq	%rsi, 0xc0(%rsp)
141c52ebc:     	movq	%r12, 0xc8(%rsp)
141c52ec4:     	leaq	0x118(%rsp), %rax
141c52ecc:     	movq	%rax, 0xd0(%rsp)
141c52ed4:     	leaq	0x12c18cd(%rip), %rdx   # 0x142f147a8
141c52edb:     	leaq	0xc0(%rsp), %r9
141c52ee3:     	movl	$0x7, %r8d
141c52ee9:     	movq	%rdi, %rcx
141c52eec:     	callq	0x14228b040 <.text+0x228a040>
141c52ef1:     	jmp	0x141c53088 <.text+0x1c52088>
141c52ef6:     	movq	0x8(%rsi), %rcx
141c52efa:     	movl	%eax, 0x40(%rsp)
141c52efe:     	movq	%rcx, 0x44(%rsp)
141c52f03:     	leaq	0x40(%rsp), %rcx
141c52f08:     	callq	0x14089d490 <.text+0x89c490>
141c52f0d:     	testq	%rax, %rax
141c52f10:     	leaq	0x12c1899(%rip), %rcx   # 0x142f147b0
141c52f17:     	cmovneq	%rax, %rcx
141c52f1b:     	movl	$0x3, %eax
141c52f20:     	cmovneq	%rdx, %rax
141c52f24:     	leaq	0x12c17db(%rip), %rdx   # 0x142f14706
141c52f2b:     	movq	%rdx, 0xc0(%rsp)
141c52f33:     	movq	$0x3, 0xc8(%rsp)
141c52f3f:     	movq	%rcx, 0xd0(%rsp)
141c52f47:     	movq	%rax, 0xd8(%rsp)
141c52f4f:     	movups	0x128(%rsp), %xmm0
141c52f57:     	movups	0x138(%rsp), %xmm1
141c52f5f:     	movaps	%xmm0, 0x40(%rsp)
141c52f64:     	movaps	%xmm1, 0x50(%rsp)
141c52f69:     	movq	0x148(%rsp), %r8
141c52f71:     	movq	%r8, 0x60(%rsp)
141c52f76:     	movl	$0x42000000, 0x68(%rsp) # imm = 0x42000000
141c52f7e:     	movq	%rbx, 0x70(%rsp)
141c52f83:     	movq	$0x1, 0x78(%rsp)
141c52f8c:     	movl	$0x42000000, 0x3c(%rsp) # imm = 0x42000000
141c52f94:     	movq	%rcx, 0x100(%rsp)
141c52f9c:     	movq	%rax, 0x108(%rsp)
141c52fa4:     	leaq	0x3c(%rsp), %rax
141c52fa9:     	movq	%rax, 0x110(%rsp)
141c52fb1:     	leaq	0x40(%rsp), %rcx
141c52fb6:     	leaq	0x100(%rsp), %r9
141c52fbe:     	movl	$0x3, %r8d
141c52fc4:     	callq	0x14228b3d0 <.text+0x228a3d0>
141c52fc9:     	jmp	0x141c53088 <.text+0x1c52088>
141c52fce:     	movq	(%rax), %rbx
141c52fd1:     	testq	%rbx, %rbx
141c52fd4:     	je	0x141c530d4 <.text+0x1c520d4>
141c52fda:     	movss	0x8(%rdx), %xmm0
141c52fdf:     	divss	0x20(%rsi), %xmm0
141c52fe4:     	callq	0x142923ee0 <.text+0x2922ee0>
141c52fe9:     	cvttss2si	%xmm0, %rax
141c52fee:     	movq	%rax, %rcx
141c52ff1:     	sarq	$0x3f, %rcx
141c52ff5:     	movaps	%xmm0, %xmm1
141c52ff8:     	subss	0xd070dc(%rip), %xmm1   # 0x14295a0dc
141c53000:     	cvttss2si	%xmm1, %rdx
141c53005:     	andq	%rcx, %rdx
141c53008:     	orq	%rax, %rdx
141c5300b:     	xorl	%ecx, %ecx
141c5300d:     	xorps	%xmm1, %xmm1
141c53010:     	ucomiss	%xmm1, %xmm0
141c53013:     	cmovaeq	%rdx, %rcx
141c53017:     	ucomiss	0xd070c2(%rip), %xmm0   # 0x14295a0e0
141c5301e:     	movq	$-0x1, %rax
141c53025:     	cmovbeq	%rcx, %rax
141c53029:     	movq	%rax, %rcx
141c5302c:     	orq	%rbx, %rcx
141c5302f:     	shrq	$0x20, %rcx
141c53033:     	je	0x141c5303c <.text+0x1c5203c>
141c53035:     	xorl	%edx, %edx
141c53037:     	divq	%rbx
141c5303a:     	jmp	0x141c53040 <.text+0x1c52040>
141c5303c:     	xorl	%edx, %edx
141c5303e:     	divl	%ebx
141c53040:     	movq	0x10(%rsi), %rcx
141c53044:     	shlq	$0x4, %rdx
141c53048:     	leaq	(%rcx,%rdx), %rax
141c5304c:     	addq	%rdx, %rcx
141c5304f:     	addq	$0x8, %rcx
141c53053:     	movq	(%rax), %rax
141c53056:     	movq	(%rcx), %rcx
141c53059:     	movss	0x28(%rsi), %xmm0
141c5305e:     	movss	%xmm0, 0x30(%rsp)
141c53064:     	movq	%rcx, 0x28(%rsp)
141c53069:     	movq	%rax, 0x20(%rsp)
141c5306e:     	leaq	0x12c161a(%rip), %r8    # 0x142f1468f
141c53075:     	leaq	0x40(%rsp), %rcx
141c5307a:     	movl	$0x4, %r9d
141c53080:     	movq	%rdi, %rdx
141c53083:     	callq	0x14228a4e0 <.text+0x22894e0>
141c53088:     	nop
141c53089:     	addq	$0x178, %rsp            # imm = 0x178
141c53090:     	popq	%rbx
141c53091:     	popq	%rbp
141c53092:     	popq	%rdi
141c53093:     	popq	%rsi
141c53094:     	popq	%r12
141c53096:     	popq	%r13
141c53098:     	popq	%r14
141c5309a:     	popq	%r15
141c5309c:     	retq
141c5309d:     	leaq	0x12c16dc(%rip), %rax   # 0x142f14780
141c530a4:     	movq	%rax, 0x40(%rsp)
141c530a9:     	movq	$0x1, 0x48(%rsp)
141c530b2:     	movq	$0x8, 0x50(%rsp)
141c530bb:     	xorps	%xmm0, %xmm0
141c530be:     	movups	%xmm0, 0x58(%rsp)
141c530c3:     	leaq	0x12c16c6(%rip), %rdx   # 0x142f14790
141c530ca:     	leaq	0x40(%rsp), %rcx
141c530cf:     	callq	0x1428d9390 <.text+0x28d8390>
141c530d4:     	leaq	0x12c15fd(%rip), %rcx   # 0x142f146d8
141c530db:     	callq	0x1428d9a20 <.text+0x28d8a20>
141c530e0:     	int3
141c530e1:     	int3
141c530e2:     	int3
141c530e3:     	int3
141c530e4:     	int3
141c530e5:     	int3
141c530e6:     	int3
141c530e7:     	int3
141c530e8:     	int3
141c530e9:     	int3
141c530ea:     	int3
141c530eb:     	int3
141c530ec:     	int3
141c530ed:     	int3
141c530ee:     	int3
141c530ef:     	int3
