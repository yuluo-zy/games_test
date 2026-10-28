
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14228ca20:     	pushq	%r15
14228ca22:     	pushq	%r14
14228ca24:     	pushq	%r13
14228ca26:     	pushq	%r12
14228ca28:     	pushq	%rsi
14228ca29:     	pushq	%rdi
14228ca2a:     	pushq	%rbp
14228ca2b:     	pushq	%rbx
14228ca2c:     	subq	$0x88, %rsp
14228ca33:     	movaps	%xmm6, 0x70(%rsp)
14228ca38:     	movq	(%rcx), %rax
14228ca3b:     	movq	0x10(%rax), %r15
14228ca3f:     	testq	%r15, %r15
14228ca42:     	je	0x14228cbe2 <.text+0x228bbe2>
14228ca48:     	movq	%rdx, %rbp
14228ca4b:     	movq	0x8(%rax), %rbx
14228ca4f:     	movq	0x8(%rcx), %rdi
14228ca53:     	movq	0x10(%rcx), %rsi
14228ca57:     	movq	0x8(%rdx), %r14
14228ca5b:     	movq	$0x0, 0x40(%rsp)
14228ca64:     	leaq	0x40(%rsp), %rcx
14228ca69:     	callq	0x1400885d0 <.text+0x875d0>
14228ca6e:     	movq	%rax, 0x28(%rsp)
14228ca73:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
14228ca7d:     	movq	%rax, 0x30(%rsp)
14228ca82:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14228ca8c:     	movq	%rax, 0x38(%rsp)
14228ca91:     	leaq	0x28(%rsp), %rcx
14228ca96:     	callq	0x14228d080 <.text+0x228c080>
14228ca9b:     	movq	%r14, 0x28(%rsp)
14228caa0:     	movq	%rax, 0x30(%rsp)
14228caa5:     	leaq	0x28(%rsp), %rcx
14228caaa:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228caaf:     	leaq	0x58(%rsp), %r14
14228cab4:     	movq	%r14, %rcx
14228cab7:     	movq	%rbp, %rdx
14228caba:     	movq	%rax, %r8
14228cabd:     	callq	0x1406861b0 <.text+0x6851b0>
14228cac2:     	movq	(%rsi), %r9
14228cac5:     	incq	%r9
14228cac8:     	movq	%rbx, %rcx
14228cacb:     	movq	%rdi, %rdx
14228cace:     	movq	%r14, %r8
14228cad1:     	callq	0x141c52700 <.text+0x1c51700>
14228cad6:     	cmpq	$0x1, %r15
14228cada:     	je	0x14228cbe2 <.text+0x228bbe2>
14228cae0:     	shlq	$0x4, %r15
14228cae4:     	leaq	(%r15,%r15,2), %r13
14228cae8:     	addq	$0x30, %rbx
14228caec:     	addq	$-0x30, %r13
14228caf0:     	movl	$0xf4241, %r14d         # imm = 0xF4241
14228caf6:     	movss	0x761106(%rip), %xmm6   # 0x1429edc04
14228cafe:     	nop
14228cb00:     	movq	%rbx, 0x48(%rsp)
14228cb05:     	movq	%rsi, %r15
14228cb08:     	movq	%rdi, %rsi
14228cb0b:     	leaq	-0xf4240(%r14), %rdi
14228cb12:     	leaq	0x30(%rbx), %rax
14228cb16:     	movq	%rax, 0x50(%rsp)
14228cb1b:     	movq	$0x1, 0x20(%rsp)
14228cb24:     	leaq	0x28(%rsp), %rbx
14228cb29:     	movq	%rbx, %rcx
14228cb2c:     	movq	%rbp, %rdx
14228cb2f:     	movq	%r14, %r8
14228cb32:     	leaq	0xdadc1f(%rip), %r9     # 0x14303a758
14228cb39:     	callq	0x14228a0e0 <.text+0x22890e0>
14228cb3e:     	movq	%rbx, %rcx
14228cb41:     	callq	0x140681d30 <.text+0x680d30>
14228cb46:     	movq	%rax, %rcx
14228cb49:     	movaps	%xmm6, %xmm1
14228cb4c:     	callq	0x1425f0ac0 <.text+0x25efac0>
14228cb51:     	movq	0x8(%rbp), %r12
14228cb55:     	movq	%rdi, 0x40(%rsp)
14228cb5a:     	movq	%rsi, %rdi
14228cb5d:     	movq	%r15, %rsi
14228cb60:     	leaq	0x40(%rsp), %rcx
14228cb65:     	callq	0x1400885d0 <.text+0x875d0>
14228cb6a:     	movq	%rax, 0x28(%rsp)
14228cb6f:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
14228cb79:     	movq	%rax, 0x30(%rsp)
14228cb7e:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14228cb88:     	movq	%rax, 0x38(%rsp)
14228cb8d:     	movq	%rbx, %rcx
14228cb90:     	callq	0x14228d080 <.text+0x228c080>
14228cb95:     	movq	%r12, 0x28(%rsp)
14228cb9a:     	movq	%rax, 0x30(%rsp)
14228cb9f:     	movq	%rbx, %rcx
14228cba2:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228cba7:     	leaq	0x58(%rsp), %rbx
14228cbac:     	movq	%rbx, %rcx
14228cbaf:     	movq	%rbp, %rdx
14228cbb2:     	movq	%rax, %r8
14228cbb5:     	callq	0x1406861b0 <.text+0x6851b0>
14228cbba:     	movq	(%r15), %r9
14228cbbd:     	incq	%r9
14228cbc0:     	movq	0x48(%rsp), %rcx
14228cbc5:     	movq	%rdi, %rdx
14228cbc8:     	movq	%rbx, %r8
14228cbcb:     	callq	0x141c52700 <.text+0x1c51700>
14228cbd0:     	incq	%r14
14228cbd3:     	movq	0x50(%rsp), %rbx
14228cbd8:     	addq	$-0x30, %r13
14228cbdc:     	jne	0x14228cb00 <.text+0x228bb00>
14228cbe2:     	movaps	0x70(%rsp), %xmm6
14228cbe7:     	addq	$0x88, %rsp
14228cbee:     	popq	%rbx
14228cbef:     	popq	%rbp
14228cbf0:     	popq	%rdi
14228cbf1:     	popq	%rsi
14228cbf2:     	popq	%r12
14228cbf4:     	popq	%r13
14228cbf6:     	popq	%r14
14228cbf8:     	popq	%r15
14228cbfa:     	retq
14228cbfb:     	int3
14228cbfc:     	int3
14228cbfd:     	int3
14228cbfe:     	int3
14228cbff:     	int3
14228cc00:     	pushq	%r15
14228cc02:     	pushq	%r14
14228cc04:     	pushq	%r13
14228cc06:     	pushq	%r12
14228cc08:     	pushq	%rsi
14228cc09:     	pushq	%rdi
14228cc0a:     	pushq	%rbp
14228cc0b:     	pushq	%rbx
14228cc0c:     	subq	$0x88, %rsp
14228cc13:     	movaps	%xmm6, 0x70(%rsp)
14228cc18:     	movq	(%rcx), %rax
14228cc1b:     	movq	0x10(%rax), %r15
14228cc1f:     	testq	%r15, %r15
14228cc22:     	je	0x14228cdc2 <.text+0x228bdc2>
14228cc28:     	movq	%rdx, %rbp
14228cc2b:     	movq	0x8(%rax), %rbx
14228cc2f:     	movq	0x8(%rcx), %rdi
14228cc33:     	movq	0x10(%rcx), %rsi
14228cc37:     	movq	0x8(%rdx), %r14
14228cc3b:     	movq	$0x0, 0x40(%rsp)
14228cc44:     	leaq	0x40(%rsp), %rcx
14228cc49:     	callq	0x1400885d0 <.text+0x875d0>
14228cc4e:     	movq	%rax, 0x28(%rsp)
14228cc53:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
14228cc5d:     	movq	%rax, 0x30(%rsp)
14228cc62:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14228cc6c:     	movq	%rax, 0x38(%rsp)
14228cc71:     	leaq	0x28(%rsp), %rcx
14228cc76:     	callq	0x14228d080 <.text+0x228c080>
14228cc7b:     	movq	%r14, 0x28(%rsp)
14228cc80:     	movq	%rax, 0x30(%rsp)
14228cc85:     	leaq	0x28(%rsp), %rcx
14228cc8a:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228cc8f:     	leaq	0x58(%rsp), %r14
14228cc94:     	movq	%r14, %rcx
14228cc97:     	movq	%rbp, %rdx
14228cc9a:     	movq	%rax, %r8
14228cc9d:     	callq	0x1406861b0 <.text+0x6851b0>
14228cca2:     	movq	(%rsi), %r9
14228cca5:     	incq	%r9
14228cca8:     	movq	%rbx, %rcx
14228ccab:     	movq	%rdi, %rdx
14228ccae:     	movq	%r14, %r8
14228ccb1:     	callq	0x141c52700 <.text+0x1c51700>
14228ccb6:     	cmpq	$0x1, %r15
14228ccba:     	je	0x14228cdc2 <.text+0x228bdc2>
14228ccc0:     	shlq	$0x4, %r15
14228ccc4:     	leaq	(%r15,%r15,2), %r13
14228ccc8:     	addq	$0x30, %rbx
14228cccc:     	addq	$-0x30, %r13
14228ccd0:     	movl	$0xf4241, %r14d         # imm = 0xF4241
14228ccd6:     	movss	0x760f26(%rip), %xmm6   # 0x1429edc04
14228ccde:     	nop
14228cce0:     	movq	%rbx, 0x48(%rsp)
14228cce5:     	movq	%rsi, %r15
14228cce8:     	movq	%rdi, %rsi
14228cceb:     	leaq	-0xf4240(%r14), %rdi
14228ccf2:     	leaq	0x30(%rbx), %rax
14228ccf6:     	movq	%rax, 0x50(%rsp)
14228ccfb:     	movq	$0x1, 0x20(%rsp)
14228cd04:     	leaq	0x28(%rsp), %rbx
14228cd09:     	movq	%rbx, %rcx
14228cd0c:     	movq	%rbp, %rdx
14228cd0f:     	movq	%r14, %r8
14228cd12:     	leaq	0xdada40(%rip), %r9     # 0x14303a759
14228cd19:     	callq	0x14228a0e0 <.text+0x22890e0>
14228cd1e:     	movq	%rbx, %rcx
14228cd21:     	callq	0x140681d30 <.text+0x680d30>
14228cd26:     	movq	%rax, %rcx
14228cd29:     	movaps	%xmm6, %xmm1
14228cd2c:     	callq	0x1425f0ac0 <.text+0x25efac0>
14228cd31:     	movq	0x8(%rbp), %r12
14228cd35:     	movq	%rdi, 0x40(%rsp)
14228cd3a:     	movq	%rsi, %rdi
14228cd3d:     	movq	%r15, %rsi
14228cd40:     	leaq	0x40(%rsp), %rcx
14228cd45:     	callq	0x1400885d0 <.text+0x875d0>
14228cd4a:     	movq	%rax, 0x28(%rsp)
14228cd4f:     	movabsq	$0x41223169ff28813b, %rax # imm = 0x41223169FF28813B
14228cd59:     	movq	%rax, 0x30(%rsp)
14228cd5e:     	movabsq	$-0x58648d975d569727, %rax # imm = 0xA79B7268A2A968D9
14228cd68:     	movq	%rax, 0x38(%rsp)
14228cd6d:     	movq	%rbx, %rcx
14228cd70:     	callq	0x14228d080 <.text+0x228c080>
14228cd75:     	movq	%r12, 0x28(%rsp)
14228cd7a:     	movq	%rax, 0x30(%rsp)
14228cd7f:     	movq	%rbx, %rcx
14228cd82:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228cd87:     	leaq	0x58(%rsp), %rbx
14228cd8c:     	movq	%rbx, %rcx
14228cd8f:     	movq	%rbp, %rdx
14228cd92:     	movq	%rax, %r8
14228cd95:     	callq	0x1406861b0 <.text+0x6851b0>
14228cd9a:     	movq	(%r15), %r9
14228cd9d:     	incq	%r9
14228cda0:     	movq	0x48(%rsp), %rcx
14228cda5:     	movq	%rdi, %rdx
14228cda8:     	movq	%rbx, %r8
14228cdab:     	callq	0x141c52700 <.text+0x1c51700>
14228cdb0:     	incq	%r14
14228cdb3:     	movq	0x50(%rsp), %rbx
14228cdb8:     	addq	$-0x30, %r13
14228cdbc:     	jne	0x14228cce0 <.text+0x228bce0>
14228cdc2:     	movaps	0x70(%rsp), %xmm6
14228cdc7:     	addq	$0x88, %rsp
14228cdce:     	popq	%rbx
14228cdcf:     	popq	%rbp
14228cdd0:     	popq	%rdi
14228cdd1:     	popq	%rsi
14228cdd2:     	popq	%r12
14228cdd4:     	popq	%r13
14228cdd6:     	popq	%r14
14228cdd8:     	popq	%r15
14228cdda:     	retq
14228cddb:     	int3
14228cddc:     	int3
14228cddd:     	int3
14228cdde:     	int3
14228cddf:     	int3
