
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14090d000:     	pushq	%rbp
14090d001:     	pushq	%r15
14090d003:     	pushq	%r14
14090d005:     	pushq	%r13
14090d007:     	pushq	%r12
14090d009:     	pushq	%rsi
14090d00a:     	pushq	%rdi
14090d00b:     	pushq	%rbx
14090d00c:     	subq	$0x1c8, %rsp            # imm = 0x1C8
14090d013:     	leaq	0x80(%rsp), %rbp
14090d01b:     	movaps	%xmm7, 0x130(%rbp)
14090d022:     	movaps	%xmm6, 0x120(%rbp)
14090d029:     	movq	$-0x2, 0x118(%rbp)
14090d034:     	movq	%r9, %r14
14090d037:     	movq	%r8, %rdi
14090d03a:     	movq	%rdx, 0xe8(%rbp)
14090d041:     	movq	%rcx, %rsi
14090d044:     	leaq	0x38(%r8), %r12
14090d048:     	movb	$0x1, 0x116(%rbp)
14090d04f:     	movabsq	$0x7e3bf54835a54748, %rdx # imm = 0x7E3BF54835A54748
14090d059:     	movabsq	$0x56b9895f6e41fabb, %r8 # imm = 0x56B9895F6E41FABB
14090d063:     	movq	%r12, %rcx
14090d066:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14090d06b:     	testq	%rax, %rax
14090d06e:     	je	0x14090d5fa <.text+0x90c5fa>
14090d074:     	cmpq	%rdx, 0x218(%rdi)
14090d07b:     	jbe	0x14090d5fa <.text+0x90c5fa>
14090d081:     	movq	0x210(%rdi), %rax
14090d088:     	movq	(%rax,%rdx,8), %rcx
14090d08c:     	testq	%rcx, %rcx
14090d08f:     	je	0x14090d5fa <.text+0x90c5fa>
14090d095:     	movq	0x1e0(%rdi), %rax
14090d09c:     	notq	%rcx
14090d09f:     	leaq	(%rcx,%rcx,2), %rcx
14090d0a3:     	shlq	$0x5, %rcx
14090d0a7:     	cmpq	$0x0, 0x20(%rax,%rcx)
14090d0ad:     	je	0x14090d5fa <.text+0x90c5fa>
14090d0b3:     	addq	%rcx, %rax
14090d0b6:     	movq	0x10(%rax), %rax
14090d0ba:     	movq	%rax, 0xa8(%rbp)
14090d0c1:     	movb	$0x1, 0x116(%rbp)
14090d0c8:     	movabsq	$0x4a142e09642375b9, %rdx # imm = 0x4A142E09642375B9
14090d0d2:     	movabsq	$0x43f6154cfa8c847f, %r8 # imm = 0x43F6154CFA8C847F
14090d0dc:     	movq	%r12, %rcx
14090d0df:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14090d0e4:     	testq	%rax, %rax
14090d0e7:     	je	0x14090d612 <.text+0x90c612>
14090d0ed:     	cmpq	%rdx, 0x218(%rdi)
14090d0f4:     	jbe	0x14090d612 <.text+0x90c612>
14090d0fa:     	movq	0x210(%rdi), %rax
14090d101:     	movq	(%rax,%rdx,8), %rcx
14090d105:     	testq	%rcx, %rcx
14090d108:     	je	0x14090d612 <.text+0x90c612>
14090d10e:     	movq	0x1e0(%rdi), %rax
14090d115:     	notq	%rcx
14090d118:     	leaq	(%rcx,%rcx,2), %rcx
14090d11c:     	shlq	$0x5, %rcx
14090d120:     	cmpq	$0x0, 0x20(%rax,%rcx)
14090d126:     	je	0x14090d612 <.text+0x90c612>
14090d12c:     	addq	%rcx, %rax
14090d12f:     	movq	0x10(%rax), %rax
14090d133:     	movzbl	0xc0(%rax), %ecx
14090d13a:     	shll	$0x6, %ecx
14090d13d:     	leaq	(%rax,%rcx), %r15
14090d141:     	cmpb	$0x0, 0x28(%rax,%rcx)
14090d146:     	je	0x14090d190 <.text+0x90c190>
14090d148:     	movzbl	0x2d27aba(%rip), %eax   # 0x143634c09
14090d14f:     	movl	$0x7, %ecx
14090d154:     	movl	$0x1, %edx
14090d159:     	callq	0x140613c10 <.text+0x612c10>
14090d15e:     	testq	%rax, %rax
14090d161:     	je	0x14090d666 <.text+0x90c666>
14090d167:     	movl	0x21d9a85(%rip), %ecx   # 0x142ae6bf2
14090d16d:     	movl	%ecx, 0x3(%rax)
14090d170:     	movl	0x21d9a79(%rip), %ecx   # 0x142ae6bef
14090d176:     	movl	%ecx, (%rax)
14090d178:     	movq	$0x7, 0x78(%rbp)
14090d180:     	movq	%rax, 0x80(%rbp)
14090d187:     	movaps	0x2021522(%rip), %xmm6  # 0x14292e6b0
14090d18e:     	jmp	0x14090d1a9 <.text+0x90c1a9>
14090d190:     	movb	$0x1, 0x116(%rbp)
14090d197:     	leaq	0x78(%rbp), %rcx
14090d19b:     	movq	%r15, %rdx
14090d19e:     	callq	0x140d7bd90 <.text+0xd7ad90>
14090d1a3:     	movsd	0x20(%r15), %xmm6
14090d1a9:     	movb	$0x1, 0x115(%rbp)
14090d1b0:     	movabsq	$-0x3d4716be36e59bd8, %rdx # imm = 0xC2B8E941C91A6428
14090d1ba:     	movabsq	$0x59f3c6d1d3da43a0, %r8 # imm = 0x59F3C6D1D3DA43A0
14090d1c4:     	movq	%r12, %rcx
14090d1c7:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14090d1cc:     	testq	%rax, %rax
14090d1cf:     	je	0x14090d627 <.text+0x90c627>
14090d1d5:     	cmpq	%rdx, 0x218(%rdi)
14090d1dc:     	jbe	0x14090d627 <.text+0x90c627>
14090d1e2:     	movq	0x210(%rdi), %rax
14090d1e9:     	movq	(%rax,%rdx,8), %rcx
14090d1ed:     	testq	%rcx, %rcx
14090d1f0:     	je	0x14090d627 <.text+0x90c627>
14090d1f6:     	movq	0x1e0(%rdi), %rax
14090d1fd:     	notq	%rcx
14090d200:     	leaq	(%rcx,%rcx,2), %rcx
14090d204:     	shlq	$0x5, %rcx
14090d208:     	cmpq	$0x0, 0x20(%rax,%rcx)
14090d20e:     	je	0x14090d627 <.text+0x90c627>
14090d214:     	addq	%rcx, %rax
14090d217:     	movq	0x10(%rax), %rax
14090d21b:     	movl	(%rax), %r8d
14090d21e:     	movq	0x1c8(%rbp), %rcx
14090d225:     	movq	(%rcx), %rax
14090d228:     	movq	%rax, 0x100(%rbp)
14090d22f:     	negq	%rax
14090d232:     	seto	%r13b
14090d236:     	movq	%r15, 0xb0(%rbp)
14090d23d:     	movb	%r13b, 0x114(%rbp)
14090d244:     	movl	%r8d, 0xfc(%rbp)
14090d24b:     	jno	0x14090d265 <.text+0x90c265>
14090d24d:     	xorps	%xmm7, %xmm7
14090d250:     	cmpb	$0x1, 0x8(%rcx)
14090d254:     	jne	0x14090d296 <.text+0x90c296>
14090d256:     	movl	$0x1, 0x110(%rbp)
14090d260:     	jmp	0x14090d304 <.text+0x90c304>
14090d265:     	movq	0x8(%rcx), %rax
14090d269:     	movq	%rax, 0xf0(%rbp)
14090d270:     	movq	0x10(%rcx), %rax
14090d274:     	movq	%rax, 0xb8(%rbp)
14090d27b:     	movss	0x18(%rcx), %xmm7
14090d280:     	movzbl	0x1c(%rcx), %eax
14090d284:     	movl	%eax, 0x10c(%rbp)
14090d28a:     	movl	$0x0, 0x110(%rbp)
14090d294:     	jmp	0x14090d30e <.text+0x90c30e>
14090d296:     	movzbl	0x2d2796c(%rip), %eax   # 0x143634c09
14090d29d:     	movl	$0x7, %ecx
14090d2a2:     	movl	$0x1, %edx
14090d2a7:     	callq	0x140613c10 <.text+0x612c10>
14090d2ac:     	testq	%rax, %rax
14090d2af:     	je	0x14090d685 <.text+0x90c685>
14090d2b5:     	movq	%rax, %rbx
14090d2b8:     	movl	0x21d9934(%rip), %eax   # 0x142ae6bf2
14090d2be:     	movl	%eax, 0x3(%rbx)
14090d2c1:     	movl	0x21d9928(%rip), %eax   # 0x142ae6bef
14090d2c7:     	movl	%eax, (%rbx)
14090d2c9:     	movq	0x78(%rbp), %rdx
14090d2cd:     	testq	%rdx, %rdx
14090d2d0:     	je	0x14090d2e4 <.text+0x90c2e4>
14090d2d2:     	movq	0x80(%rbp), %rcx
14090d2d9:     	movl	$0x1, %r8d
14090d2df:     	callq	0x140613c20 <.text+0x612c20>
14090d2e4:     	movq	$0x7, 0x78(%rbp)
14090d2ec:     	movq	%rbx, 0x80(%rbp)
14090d2f3:     	movaps	0x20213b6(%rip), %xmm6  # 0x14292e6b0
14090d2fa:     	movl	$0x3, 0x110(%rbp)
14090d304:     	movl	$0x0, 0x10c(%rbp)
14090d30e:     	movb	$0x1, 0x117(%rbp)
14090d315:     	movabsq	$0x5f06b298510fb768, %rdx # imm = 0x5F06B298510FB768
14090d31f:     	movabsq	$0x54e14bacecb858d, %r8 # imm = 0x54E14BACECB858D
14090d329:     	movq	%r12, %rcx
14090d32c:     	callq	0x1400dd6f0 <.text+0xdc6f0>
14090d331:     	testq	%rax, %rax
14090d334:     	je	0x14090d63c <.text+0x90c63c>
14090d33a:     	cmpq	%rdx, 0x218(%rdi)
14090d341:     	jbe	0x14090d63c <.text+0x90c63c>
14090d347:     	movq	0x210(%rdi), %rax
14090d34e:     	movq	(%rax,%rdx,8), %rcx
14090d352:     	testq	%rcx, %rcx
14090d355:     	je	0x14090d63c <.text+0x90c63c>
14090d35b:     	movq	0x1e0(%rdi), %rax
14090d362:     	notq	%rcx
14090d365:     	leaq	(%rcx,%rcx,2), %rcx
14090d369:     	shlq	$0x5, %rcx
14090d36d:     	cmpq	$0x0, 0x20(%rax,%rcx)
14090d373:     	je	0x14090d63c <.text+0x90c63c>
14090d379:     	addq	%rcx, %rax
14090d37c:     	movq	0x10(%rax), %r15
14090d380:     	movq	(%r15), %rax
14090d383:     	xorl	%ecx, %ecx
14090d385:     	cmpq	%rax, %rcx
14090d388:     	cmovoq	%rcx, %r15
14090d38c:     	addq	$0x18, %r15
14090d390:     	cmpq	%rax, %rcx
14090d393:     	cmovoq	%rcx, %r15
14090d397:     	movb	$0x1, 0x117(%rbp)
14090d39e:     	leaq	-0x40(%rbp), %rcx
14090d3a2:     	movq	%r14, %rdx
14090d3a5:     	movl	$0x2b, %r8d
14090d3ab:     	callq	0x140bd52a0 <.text+0xbd42a0>
14090d3b0:     	movb	$0x1, 0x117(%rbp)
14090d3b7:     	leaq	0x58(%rbp), %r12
14090d3bb:     	leaq	-0x40(%rbp), %rdx
14090d3bf:     	movq	%r12, %rcx
14090d3c2:     	movb	$0x4, %r8b
14090d3c5:     	callq	0x140bd55a0 <.text+0xbd45a0>
14090d3ca:     	movups	0x58(%rbp), %xmm0
14090d3ce:     	movups	0x68(%rbp), %xmm1
14090d3d2:     	movaps	%xmm1, 0xd0(%rbp)
14090d3d9:     	movaps	%xmm0, 0xc0(%rbp)
14090d3e0:     	leaq	-0x30(%rbp), %rcx
14090d3e4:     	movb	$0x1, 0x117(%rbp)
14090d3eb:     	leaq	0xc0(%rbp), %rbx
14090d3f2:     	movq	%rbx, %rdx
14090d3f5:     	callq	0x140bd6ce0 <.text+0xbd5ce0>
14090d3fa:     	leaq	0x219dcdf(%rip), %rax   # 0x142aab0e0
14090d401:     	movq	%rax, -0x40(%rbp)
14090d405:     	movq	$0xe, -0x38(%rbp)
14090d40d:     	movb	$0x1, 0x117(%rbp)
14090d414:     	leaq	-0x40(%rbp), %r13
14090d418:     	movq	0xe8(%rbp), %rcx
14090d41f:     	movq	%r13, %rdx
14090d422:     	callq	0x140be94d0 <.text+0xbe84d0>
14090d427:     	movups	0x58(%rbp), %xmm0
14090d42b:     	movups	0x68(%rbp), %xmm1
14090d42f:     	movups	%xmm0, 0xc4(%rbp)
