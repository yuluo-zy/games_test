
reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
142298d30:     	pushq	%r15
142298d32:     	pushq	%r14
142298d34:     	pushq	%r13
142298d36:     	pushq	%r12
142298d38:     	pushq	%rsi
142298d39:     	pushq	%rdi
142298d3a:     	pushq	%rbp
142298d3b:     	pushq	%rbx
142298d3c:     	subq	$0x78, %rsp
142298d40:     	movaps	%xmm7, 0x60(%rsp)
142298d45:     	movaps	%xmm6, 0x50(%rsp)
142298d4a:     	movq	%r9, 0x38(%rsp)
142298d4f:     	movq	(%rcx), %rsi
142298d52:     	movq	0x8(%rcx), %rdi
142298d56:     	movq	(%rsi), %rax
142298d59:     	xorl	%r15d, %r15d
142298d5c:     	movq	%rax, %r10
142298d5f:     	subq	0x18(%rdi), %r10
142298d63:     	cmovbq	%r15, %r10
142298d67:     	subq	0x38(%rdi), %rax
142298d6b:     	cmovbq	%r15, %rax
142298d6f:     	movq	0x10(%rdi), %rbx
142298d73:     	movq	0x30(%rdi), %r9
142298d77:     	movq	%r9, %rcx
142298d7a:     	subq	%rax, %rcx
142298d7d:     	cmovbq	%r15, %rcx
142298d81:     	setbe	%bpl
142298d85:     	movq	%rbx, %r14
142298d88:     	subq	%r10, %r14
142298d8b:     	cmovbq	%r15, %r14
142298d8f:     	setbe	%r12b
142298d93:     	leaq	(%rcx,%r14), %r13
142298d97:     	testb	%bpl, %r12b
142298d9a:     	cmoveq	%r15, %r13
142298d9e:     	movq	0x40(%rdi), %r11
142298da2:     	subq	%r13, %r11
142298da5:     	movq	0x8(%rdi), %r15
142298da9:     	movq	0x28(%rdi), %rdi
142298dad:     	movq	%r11, (%rsi)
142298db0:     	movq	0xe8(%rsp), %r13
142298db8:     	testb	%bpl, %r12b
142298dbb:     	jne	0x142298e06 <.text+0x2297e06>
142298dbd:     	xorl	%r11d, %r11d
142298dc0:     	cmpq	%r10, %rbx
142298dc3:     	leaq	(%r10,%r10,4), %r10
142298dc7:     	leaq	(%r15,%r10,8), %r10
142298dcb:     	movl	$0x8, %esi
142298dd0:     	cmovbq	%rsi, %r10
142298dd4:     	leaq	(%r14,%r14,4), %rbx
142298dd8:     	leaq	-0x28(%r10,%rbx,8), %r10
142298ddd:     	cmovaq	%r10, %r11
142298de1:     	cmpq	%rax, %r9
142298de4:     	leaq	(%rax,%rax,4), %rax
142298de8:     	leaq	(%rdi,%rax,8), %rax
142298dec:     	cmovbq	%rsi, %rax
142298df0:     	leaq	(%rcx,%rcx,4), %rcx
142298df4:     	leaq	-0x28(%rax,%rcx,8), %rax
142298df9:     	cmovbeq	%r11, %rax
142298dfd:     	cmpl	$0xc, (%rax)
142298e00:     	jne	0x142298fef <.text+0x2297fef>
142298e06:     	movq	(%rdx), %rax
142298e09:     	movq	0x8(%rdx), %rcx
142298e0d:     	movq	(%rax), %rdx
142298e10:     	xorl	%r9d, %r9d
142298e13:     	movq	%rdx, %r10
142298e16:     	subq	0x18(%rcx), %r10
142298e1a:     	cmovbq	%r9, %r10
142298e1e:     	subq	0x38(%rcx), %rdx
142298e22:     	cmovbq	%r9, %rdx
142298e26:     	movq	0x30(%rcx), %r11
142298e2a:     	subq	%rdx, %r11
142298e2d:     	cmovbq	%r9, %r11
142298e31:     	setbe	%dl
142298e34:     	movq	0x10(%rcx), %rsi
142298e38:     	subq	%r10, %rsi
142298e3b:     	cmovbq	%r9, %rsi
142298e3f:     	movq	0x40(%rcx), %rcx
142298e43:     	setbe	%r9b
142298e47:     	addq	%r11, %rsi
142298e4a:     	movq	%rcx, %r10
142298e4d:     	subq	%rsi, %r10
142298e50:     	movq	%r10, (%rax)
142298e53:     	testb	%dl, %r9b
142298e56:     	je	0x142298fec <.text+0x2297fec>
142298e5c:     	movq	0xe0(%rsp), %rax
142298e64:     	movq	(%r8), %rdx
142298e67:     	movq	0x8(%r8), %r15
142298e6b:     	movq	0xf8(%rdx), %r12
142298e72:     	movq	0x100(%rdx), %rcx
142298e79:     	leaq	(%r12,%rcx,4), %r13
142298e7d:     	movq	0x38(%rsp), %rcx
142298e82:     	movq	(%rcx), %rcx
142298e85:     	movq	%rcx, 0x38(%rsp)
142298e8a:     	movq	(%rax), %rax
142298e8d:     	movq	%rax, 0x48(%rsp)
142298e92:     	movq	%rdx, 0x40(%rsp)
142298e97:     	cmpb	$0x0, 0x108(%rdx)
142298e9e:     	je	0x142299055 <.text+0x2298055>
142298ea4:     	xorl	%esi, %esi
142298ea6:     	movsd	0xda2e02(%rip), %xmm6   # 0x14303bcb0
142298eae:     	leaq	0x2c(%rsp), %r14
142298eb3:     	movss	0x69b6ed(%rip), %xmm7   # 0x1429345a8
142298ebb:     	xorl	%edi, %edi
142298ebd:     	xorl	%ebx, %ebx
142298ebf:     	cmpq	%rdi, %rbx
142298ec2:     	jne	0x142298f33 <.text+0x2297f33>
142298ec4:     	nopw	%cs:(%rax,%rax)
142298ed0:     	cmpq	%r13, %r12
142298ed3:     	je	0x14229903a <.text+0x229803a>
142298ed9:     	movl	(%r12), %ecx
142298edd:     	addq	$0x4, %r12
142298ee1:     	movq	0x1a8(%r15), %rax
142298ee8:     	leaq	(%rcx,%rcx,8), %rcx
142298eec:     	movq	0x10(%rax,%rcx,8), %rdi
142298ef1:     	testq	%rdi, %rdi
142298ef4:     	je	0x142298ed0 <.text+0x2297ed0>
142298ef6:     	leaq	(%rax,%rcx,8), %rax
142298efa:     	movq	0x40(%rsp), %rcx
142298eff:     	movq	0x110(%rcx), %rcx
142298f06:     	cmpq	%rcx, 0x40(%rax)
142298f0a:     	jbe	0x142298f2f <.text+0x2297f2f>
142298f0c:     	movq	0x38(%rax), %rdx
142298f10:     	movq	(%rdx,%rcx,8), %rcx
142298f14:     	testq	%rcx, %rcx
142298f17:     	je	0x142298f2f <.text+0x2297f2f>
142298f19:     	movq	0x18(%rax), %rax
142298f1d:     	notq	%rcx
142298f20:     	leaq	(%rcx,%rcx,2), %rcx
142298f24:     	shlq	$0x4, %rcx
142298f28:     	movq	0x10(%rax,%rcx), %rsi
142298f2d:     	jmp	0x142298f31 <.text+0x2297f31>
142298f2f:     	xorl	%esi, %esi
142298f31:     	xorl	%ebx, %ebx
142298f33:     	movl	%ebx, %eax
142298f35:     	leaq	(%rax,%rax,4), %rbp
142298f39:     	shlq	$0x5, %rbp
142298f3d:     	movq	0x38(%rsp), %rax
142298f42:     	movsd	(%rax), %xmm0
142298f46:     	movsd	0x50(%rsi,%rbp), %xmm1
142298f4c:     	subps	%xmm1, %xmm0
142298f4f:     	movsd	0x8c(%rsi,%rbp), %xmm1
142298f58:     	mulps	%xmm0, %xmm0
142298f5b:     	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
142298f5f:     	addss	%xmm0, %xmm2
142298f63:     	xorps	%xmm0, %xmm0
142298f66:     	sqrtss	%xmm2, %xmm0
142298f6a:     	mulps	%xmm6, %xmm1
142298f6d:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
142298f71:     	addss	%xmm1, %xmm2
142298f75:     	ucomiss	%xmm2, %xmm0
142298f78:     	ja	0x1422991bc <.text+0x22981bc>
142298f7e:     	addq	%rsi, %rbp
142298f81:     	incq	%rbx
142298f84:     	movq	%r14, %rcx
142298f87:     	movq	0x48(%rsp), %rdx
142298f8c:     	callq	0x140911dd0 <.text+0x910dd0>
142298f91:     	movss	0x30(%rsp), %xmm0
142298f97:     	subss	0x98(%rbp), %xmm0
142298f9f:     	mulss	%xmm0, %xmm0
142298fa3:     	movss	0x34(%rsp), %xmm1
142298fa9:     	movss	0x2c(%rsp), %xmm2
142298faf:     	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
142298fb2:     	movss	0x94(%rbp), %xmm1
142298fba:     	movss	0x9c(%rbp), %xmm3
142298fc2:     	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
142298fc5:     	subps	%xmm1, %xmm2
142298fc8:     	mulps	%xmm2, %xmm2
142298fcb:     	addss	%xmm2, %xmm0
142298fcf:     	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
142298fd3:     	addss	%xmm0, %xmm1
142298fd7:     	xorps	%xmm0, %xmm0
142298fda:     	sqrtss	%xmm1, %xmm0
142298fde:     	ucomiss	%xmm7, %xmm0
142298fe1:     	jbe	0x142298ebf <.text+0x2297ebf>
142298fe7:     	jmp	0x1422991bc <.text+0x22981bc>
142298fec:     	movq	%rcx, (%rax)
142298fef:     	movl	0x1c(%r13), %eax
142298ff3:     	movq	0x10(%r13), %rcx
142298ff7:     	movl	%eax, (%rcx)
142298ff9:     	movq	(%r13), %rsi
142298ffd:     	movq	0x30(%rsi), %rdi
142299001:     	movq	0x40(%rsi), %rbx
142299005:     	cmpq	0x20(%rsi), %rdi
142299009:     	jne	0x14229901b <.text+0x229801b>
14229900b:     	leaq	0x20(%rsi), %rcx
14229900f:     	leaq	0xda22b2(%rip), %rdx    # 0x14303b2c8
142299016:     	callq	0x140562630 <.text+0x561630>
14229901b:     	movq	0x28(%rsi), %rax
14229901f:     	movq	%rdi, %rcx
142299022:     	shlq	$0x4, %rcx
142299026:     	movq	%rbx, (%rax,%rcx)
14229902a:     	movb	$0x1, 0x8(%rax,%rcx)
14229902f:     	incq	%rdi
142299032:     	movq	%rdi, 0x30(%rsi)
142299036:     	incq	0x40(%rsi)
14229903a:     	movaps	0x50(%rsp), %xmm6
14229903f:     	movaps	0x60(%rsp), %xmm7
142299044:     	addq	$0x78, %rsp
142299048:     	popq	%rbx
142299049:     	popq	%rbp
14229904a:     	popq	%rdi
14229904b:     	popq	%rsi
14229904c:     	popq	%r12
14229904e:     	popq	%r13
142299050:     	popq	%r14
142299052:     	popq	%r15
142299054:     	retq
142299055:     	movl	$0x8, %esi
14229905a:     	xorl	%ebx, %ebx
14229905c:     	movsd	0xda2c4c(%rip), %xmm6   # 0x14303bcb0
142299064:     	movss	0x69b53c(%rip), %xmm7   # 0x1429345a8
14229906c:     	xorl	%edi, %edi
14229906e:     	xorl	%ebp, %ebp
142299070:     	cmpq	%rdi, %rbp
142299073:     	jne	0x1422990fc <.text+0x22980fc>
142299079:     	nopl	(%rax)
142299080:     	cmpq	%r13, %r12
142299083:     	je	0x14229903a <.text+0x229803a>
142299085:     	movl	(%r12), %ecx
142299089:     	addq	$0x4, %r12
14229908d:     	movq	0x100(%r15), %rax
142299094:     	leaq	(%rcx,%rcx,4), %rcx
142299098:     	shlq	$0x5, %rcx
14229909c:     	movq	0x58(%rax,%rcx), %rdi
1422990a1:     	testq	%rdi, %rdi
1422990a4:     	je	0x142299080 <.text+0x2298080>
1422990a6:     	addq	%rcx, %rax
1422990a9:     	movl	0x94(%rax), %ecx
1422990af:     	movq	0x1a8(%r15), %rdx
1422990b6:     	leaq	(%rcx,%rcx,8), %r8
1422990ba:     	movq	0x40(%rsp), %rcx
1422990bf:     	movq	0x110(%rcx), %rcx
1422990c6:     	cmpq	%rcx, 0x40(%rdx,%r8,8)
1422990cb:     	jbe	0x1422990f4 <.text+0x22980f4>
1422990cd:     	leaq	(%rdx,%r8,8), %rdx
1422990d1:     	movq	0x38(%rdx), %r8
1422990d5:     	movq	(%r8,%rcx,8), %rcx
1422990d9:     	testq	%rcx, %rcx
1422990dc:     	je	0x1422990f4 <.text+0x22980f4>
1422990de:     	movq	0x18(%rdx), %rdx
1422990e2:     	notq	%rcx
1422990e5:     	leaq	(%rcx,%rcx,2), %rcx
1422990e9:     	shlq	$0x4, %rcx
1422990ed:     	movq	0x10(%rdx,%rcx), %rbx
1422990f2:     	jmp	0x1422990f6 <.text+0x22980f6>
1422990f4:     	xorl	%ebx, %ebx
1422990f6:     	movq	0x50(%rax), %rsi
1422990fa:     	xorl	%ebp, %ebp
1422990fc:     	movq	%rbp, %rax
1422990ff:     	shlq	$0x4, %rax
142299103:     	movl	0x8(%rsi,%rax), %eax
142299107:     	leaq	(%rax,%rax,4), %r14
14229910b:     	shlq	$0x5, %r14
14229910f:     	movq	0x38(%rsp), %rax
142299114:     	movsd	(%rax), %xmm0
142299118:     	movsd	0x50(%rbx,%r14), %xmm1
14229911f:     	subps	%xmm1, %xmm0
142299122:     	movsd	0x8c(%rbx,%r14), %xmm1
14229912c:     	mulps	%xmm0, %xmm0
14229912f:     	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
142299133:     	addss	%xmm0, %xmm2
142299137:     	xorps	%xmm0, %xmm0
14229913a:     	sqrtss	%xmm2, %xmm0
14229913e:     	mulps	%xmm6, %xmm1
142299141:     	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
142299145:     	addss	%xmm1, %xmm2
142299149:     	ucomiss	%xmm2, %xmm0
14229914c:     	ja	0x1422991bc <.text+0x22981bc>
14229914e:     	addq	%rbx, %r14
142299151:     	incq	%rbp
142299154:     	leaq	0x2c(%rsp), %rcx
142299159:     	movq	0x48(%rsp), %rdx
14229915e:     	callq	0x140911dd0 <.text+0x910dd0>
142299163:     	movss	0x30(%rsp), %xmm0
142299169:     	subss	0x98(%r14), %xmm0
142299172:     	mulss	%xmm0, %xmm0
142299176:     	movss	0x34(%rsp), %xmm1
14229917c:     	movss	0x2c(%rsp), %xmm2
142299182:     	unpcklps	%xmm1, %xmm2            # xmm2 = xmm2[0],xmm1[0],xmm2[1],xmm1[1]
142299185:     	movss	0x94(%r14), %xmm1
14229918e:     	movss	0x9c(%r14), %xmm3
142299197:     	unpcklps	%xmm3, %xmm1            # xmm1 = xmm1[0],xmm3[0],xmm1[1],xmm3[1]
14229919a:     	subps	%xmm1, %xmm2
14229919d:     	mulps	%xmm2, %xmm2
1422991a0:     	addss	%xmm2, %xmm0
1422991a4:     	movshdup	%xmm2, %xmm1            # xmm1 = xmm2[1,1,3,3]
1422991a8:     	addss	%xmm0, %xmm1
1422991ac:     	xorps	%xmm0, %xmm0
1422991af:     	sqrtss	%xmm1, %xmm0
1422991b3:     	ucomiss	%xmm7, %xmm0
1422991b6:     	jbe	0x142299070 <.text+0x2298070>
1422991bc:     	movq	0xe8(%rsp), %rdx
1422991c4:     	movl	0x1c(%rdx), %eax
1422991c7:     	movq	0x10(%rdx), %rcx
1422991cb:     	movl	%eax, (%rcx)
1422991cd:     	movq	(%rdx), %rsi
1422991d0:     	movq	0x30(%rsi), %rdi
1422991d4:     	movq	0x40(%rsi), %rbx
1422991d8:     	cmpq	0x20(%rsi), %rdi
1422991dc:     	je	0x14229900b <.text+0x229800b>
1422991e2:     	jmp	0x14229901b <.text+0x229801b>
