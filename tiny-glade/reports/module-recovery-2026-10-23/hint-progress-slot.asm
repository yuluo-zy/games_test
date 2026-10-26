
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14228bfd0:     	pushq	%r15
14228bfd2:     	pushq	%r14
14228bfd4:     	pushq	%r13
14228bfd6:     	pushq	%r12
14228bfd8:     	pushq	%rsi
14228bfd9:     	pushq	%rdi
14228bfda:     	pushq	%rbp
14228bfdb:     	pushq	%rbx
14228bfdc:     	subq	$0xf8, %rsp
14228bfe3:     	movq	%r9, %rsi
14228bfe6:     	movq	%rcx, %rbx
14228bfe9:     	movq	%rdx, 0x68(%rsp)
14228bfee:     	movq	%r8, 0x70(%rsp)
14228bff3:     	leaq	0x10(%rcx), %rdi
14228bff7:     	movq	%rdi, %rcx
14228bffa:     	callq	0x1406868b0 <.text+0x6858b0>
14228bfff:     	movq	%rax, %r12
14228c002:     	leaq	0x68(%rsp), %r15
14228c007:     	movq	%r15, %rcx
14228c00a:     	callq	0x140683c10 <.text+0x682c10>
14228c00f:     	movq	%rax, 0x48(%rsp)
14228c014:     	leaq	0x48(%rsp), %r14
14228c019:     	movq	%r14, %rcx
14228c01c:     	callq	0x1400885d0 <.text+0x875d0>
14228c021:     	movq	%rax, 0x78(%rsp)
14228c026:     	movabsq	$0x41223169ff28813b, %r13 # imm = 0x41223169FF28813B
14228c030:     	movq	%r13, 0x80(%rsp)
14228c038:     	movabsq	$-0x58648d975d569727, %rbp # imm = 0xA79B7268A2A968D9
14228c042:     	movq	%rbp, 0x88(%rsp)
14228c04a:     	leaq	0x78(%rsp), %rcx
14228c04f:     	callq	0x14228d080 <.text+0x228c080>
14228c054:     	movq	%r12, 0x78(%rsp)
14228c059:     	movq	%rax, 0x80(%rsp)
14228c061:     	leaq	0x78(%rsp), %rcx
14228c066:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228c06b:     	movq	%rax, %r12
14228c06e:     	movq	%rdi, %rcx
14228c071:     	movq	%rax, %rdx
14228c074:     	callq	0x140686450 <.text+0x685450>
14228c079:     	testb	%al, %al
14228c07b:     	je	0x14228c092 <.text+0x228b092>
14228c07d:     	leaq	0x48(%rsp), %rcx
14228c082:     	movq	%rdi, %rdx
14228c085:     	movq	%r12, %r8
14228c088:     	callq	0x140686530 <.text+0x685530>
14228c08d:     	jmp	0x14228c1e4 <.text+0x228b1e4>
14228c092:     	movq	0x13a030f(%rip), %rax   # 0x14362c3a8
14228c099:     	movq	(%rax), %rax
14228c09c:     	testq	%rax, %rax
14228c09f:     	je	0x14228c186 <.text+0x228b186>
14228c0a5:     	movq	%rbx, 0x48(%rsp)
14228c0aa:     	leaq	-0x1d58811(%rip), %rax  # 0x1405338a0 <.text+0x5328a0>
14228c0b1:     	movq	%rax, 0x50(%rsp)
14228c0b6:     	movq	%r15, 0x58(%rsp)
14228c0bb:     	movq	%rax, 0x60(%rsp)
14228c0c0:     	leaq	0xdae559(%rip), %rcx    # 0x14303a620
14228c0c7:     	callq	0x14067dd80 <.text+0x67cd80>
14228c0cc:     	movups	(%rax), %xmm0
14228c0cf:     	movl	0x10(%rax), %eax
14228c0d2:     	movq	$0x1, 0xa8(%rsp)
14228c0de:     	leaq	0xdae553(%rip), %rcx    # 0x14303a638
14228c0e5:     	movq	%rcx, 0xb0(%rsp)
14228c0ed:     	movq	$0x14, 0xb8(%rsp)
14228c0f9:     	leaq	0xdae4a8(%rip), %rdx    # 0x14303a5a8
14228c100:     	movq	%rdx, 0xc8(%rsp)
14228c108:     	movq	$0x2, 0xd0(%rsp)
14228c114:     	movq	%r14, 0xd8(%rsp)
14228c11c:     	movq	$0x2, 0xe0(%rsp)
14228c128:     	movq	$0x0, 0xe8(%rsp)
14228c134:     	movq	$0x0, 0x78(%rsp)
14228c13d:     	movq	%rcx, 0x80(%rsp)
14228c145:     	movq	$0x14, 0x88(%rsp)
14228c151:     	movq	$0x0, 0x90(%rsp)
14228c15d:     	movups	%xmm0, 0x98(%rsp)
14228c165:     	movl	$0x1, 0xc0(%rsp)
14228c170:     	movl	%eax, 0xc4(%rsp)
14228c177:     	leaq	0x47(%rsp), %rcx
14228c17c:     	leaq	0x78(%rsp), %rdx
14228c181:     	callq	0x140d4a330 <.text+0xd49330>
14228c186:     	movups	0x68(%rsp), %xmm0
14228c18b:     	movq	0x18(%rbx), %rbx
14228c18f:     	movups	%xmm0, 0x48(%rsp)
14228c194:     	leaq	0x48(%rsp), %rcx
14228c199:     	callq	0x140683c10 <.text+0x682c10>
14228c19e:     	movq	%rax, 0x78(%rsp)
14228c1a3:     	movq	%r13, 0x80(%rsp)
14228c1ab:     	movq	%rbp, 0x88(%rsp)
14228c1b3:     	leaq	0x78(%rsp), %rcx
14228c1b8:     	callq	0x14228d080 <.text+0x228c080>
14228c1bd:     	movq	%rbx, 0x78(%rsp)
14228c1c2:     	movq	%rax, 0x80(%rsp)
14228c1ca:     	leaq	0x78(%rsp), %rcx
14228c1cf:     	callq	0x14228d1f0 <.text+0x228c1f0>
14228c1d4:     	leaq	0x48(%rsp), %rcx
14228c1d9:     	movq	%rdi, %rdx
14228c1dc:     	movq	%rax, %r8
14228c1df:     	callq	0x1406861b0 <.text+0x6851b0>
14228c1e4:     	movq	(%rsi), %rcx
14228c1e7:     	cmpq	$0x4, %rcx
14228c1eb:     	jae	0x14228c243 <.text+0x228b243>
14228c1ed:     	shlq	$0x4, %rcx
14228c1f1:     	leaq	0xdae4a8(%rip), %rax    # 0x14303a6a0
14228c1f8:     	movq	(%rcx,%rax), %rax
14228c1fc:     	movq	%rax, 0x20(%rsp)
14228c201:     	movl	$0x41880000, 0x30(%rsp) # imm = 0x41880000
14228c209:     	movq	$0xf, 0x28(%rsp)
14228c212:     	leaq	0xdae517(%rip), %r8     # 0x14303a730
14228c219:     	leaq	0x78(%rsp), %rcx
14228c21e:     	leaq	0x48(%rsp), %rdx
14228c223:     	movl	$0xc, %r9d
14228c229:     	callq	0x14228a4e0 <.text+0x22894e0>
14228c22e:     	nop
14228c22f:     	addq	$0xf8, %rsp
14228c236:     	popq	%rbx
14228c237:     	popq	%rbp
14228c238:     	popq	%rdi
14228c239:     	popq	%rsi
14228c23a:     	popq	%r12
14228c23c:     	popq	%r13
14228c23e:     	popq	%r14
14228c240:     	popq	%r15
14228c242:     	retq
14228c243:     	leaq	0xdae4ce(%rip), %r8     # 0x14303a718
14228c24a:     	movl	$0x4, %edx
14228c24f:     	callq	0x1428d9518 <.text+0x28d8518>
14228c254:     	int3
14228c255:     	int3
14228c256:     	int3
14228c257:     	int3
14228c258:     	int3
14228c259:     	int3
14228c25a:     	int3
14228c25b:     	int3
14228c25c:     	int3
14228c25d:     	int3
14228c25e:     	int3
14228c25f:     	int3
