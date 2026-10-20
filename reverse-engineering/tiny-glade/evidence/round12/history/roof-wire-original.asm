
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140259e30:     	pushq	%r14
140259e32:     	pushq	%rsi
140259e33:     	pushq	%rdi
140259e34:     	pushq	%rbx
140259e35:     	subq	$0x38, %rsp
140259e39:     	movq	%rdx, %rdi
140259e3c:     	movq	%rcx, %rsi
140259e3f:     	movzbl	(%rcx), %eax
140259e42:     	leaq	0x274b1df(%rip), %rcx   # 0x1429a5028
140259e49:     	movslq	(%rcx,%rax,4), %rax
140259e4d:     	addq	%rcx, %rax
140259e50:     	jmpq	*%rax
140259e52:     	leaq	0x8(%rdi), %rdx
140259e56:     	movq	(%rdi), %rbx
140259e59:     	movq	0x10(%rbx), %rax
140259e5d:     	cmpq	%rax, (%rbx)
140259e60:     	je	0x14025a48f <.text+0x25948f>
140259e66:     	movq	0x8(%rbx), %rcx
140259e6a:     	movb	$0x7b, (%rcx,%rax)
140259e6e:     	incq	%rax
140259e71:     	movq	%rax, 0x10(%rbx)
140259e75:     	leaq	0x274ae28(%rip), %r8    # 0x1429a4ca4
140259e7c:     	jmp	0x14025a2a7 <.text+0x2592a7>
140259e81:     	leaq	0x8(%rdi), %rdx
140259e85:     	movq	(%rdi), %rbx
140259e88:     	movq	0x10(%rbx), %rax
140259e8c:     	cmpq	%rax, (%rbx)
140259e8f:     	je	0x14025a402 <.text+0x259402>
140259e95:     	movq	0x8(%rbx), %rcx
140259e99:     	movb	$0x7b, (%rcx,%rax)
140259e9d:     	incq	%rax
140259ea0:     	movq	%rax, 0x10(%rbx)
140259ea4:     	leaq	0x274aeb5(%rip), %r8    # 0x1429a4d60
140259eab:     	movl	$0x16, %r9d
140259eb1:     	movq	%rdi, %rcx
140259eb4:     	callq	0x1404b6100 <.text+0x4b5100>
140259eb9:     	testq	%rax, %rax
140259ebc:     	jne	0x14025a3a3 <.text+0x2593a3>
140259ec2:     	movq	(%rdi), %rbx
140259ec5:     	movq	0x10(%rbx), %rdx
140259ec9:     	cmpq	%rdx, (%rbx)
140259ecc:     	je	0x14025a4ed <.text+0x2594ed>
140259ed2:     	movq	0x8(%rbx), %rax
140259ed6:     	movb	$0x3a, (%rax,%rdx)
140259eda:     	incq	%rdx
140259edd:     	movq	%rdx, 0x10(%rbx)
140259ee1:     	movq	(%rdi), %rbx
140259ee4:     	movq	0x10(%rbx), %rdx
140259ee8:     	cmpq	%rdx, (%rbx)
140259eeb:     	je	0x14025a513 <.text+0x259513>
140259ef1:     	movq	0x8(%rbx), %rax
140259ef5:     	movb	$0x7b, (%rax,%rdx)
140259ef9:     	incq	%rdx
140259efc:     	movq	%rdx, 0x10(%rbx)
140259f00:     	leaq	0x8(%rsi), %r9
140259f04:     	movq	%rdi, 0x28(%rsp)
140259f09:     	movb	$0x1, 0x30(%rsp)
140259f0e:     	leaq	0x274ad99(%rip), %rdx   # 0x1429a4cae
140259f15:     	leaq	0x28(%rsp), %rcx
140259f1a:     	movl	$0xa, %r8d
140259f20:     	callq	0x1404c11a0 <.text+0x4c01a0>
140259f25:     	testq	%rax, %rax
140259f28:     	jne	0x14025a3ab <.text+0x2593ab>
140259f2e:     	leaq	0x1(%rsi), %r9
140259f32:     	leaq	0x274ae3d(%rip), %rdx   # 0x1429a4d76
140259f39:     	leaq	0x28(%rsp), %rcx
140259f3e:     	movl	$0x6, %r8d
140259f44:     	callq	0x1404be720 <.text+0x4bd720>
140259f49:     	testq	%rax, %rax
140259f4c:     	jne	0x14025a3ab <.text+0x2593ab>
140259f52:     	addq	$0x2, %rsi
140259f56:     	leaq	0x274ae1f(%rip), %rdx   # 0x1429a4d7c
140259f5d:     	leaq	0x28(%rsp), %rcx
140259f62:     	movl	$0x5, %r8d
140259f68:     	movq	%rsi, %r9
140259f6b:     	callq	0x1404be720 <.text+0x4bd720>
140259f70:     	jmp	0x14025a388 <.text+0x259388>
140259f75:     	leaq	0x8(%rdi), %rdx
140259f79:     	movq	(%rdi), %rbx
140259f7c:     	movq	0x10(%rbx), %rax
140259f80:     	cmpq	%rax, (%rbx)
140259f83:     	je	0x14025a431 <.text+0x259431>
140259f89:     	movq	0x8(%rbx), %rcx
140259f8d:     	movb	$0x7b, (%rcx,%rax)
140259f91:     	incq	%rax
140259f94:     	movq	%rax, 0x10(%rbx)
140259f98:     	leaq	0x274ad32(%rip), %r8    # 0x1429a4cd1
140259f9f:     	movl	$0xa, %r9d
140259fa5:     	movq	%rdi, %rcx
140259fa8:     	callq	0x1404b6100 <.text+0x4b5100>
140259fad:     	testq	%rax, %rax
140259fb0:     	jne	0x14025a3a3 <.text+0x2593a3>
140259fb6:     	movq	(%rdi), %rbx
140259fb9:     	movq	0x10(%rbx), %rdx
140259fbd:     	cmpq	%rdx, (%rbx)
140259fc0:     	je	0x14025a539 <.text+0x259539>
140259fc6:     	movq	0x8(%rbx), %rax
140259fca:     	movb	$0x3a, (%rax,%rdx)
140259fce:     	incq	%rdx
140259fd1:     	movq	%rdx, 0x10(%rbx)
140259fd5:     	movq	(%rdi), %rbx
140259fd8:     	movq	0x10(%rbx), %rdx
140259fdc:     	cmpq	%rdx, (%rbx)
140259fdf:     	je	0x14025a55f <.text+0x25955f>
140259fe5:     	movq	0x8(%rbx), %rax
140259fe9:     	movb	$0x7b, (%rax,%rdx)
140259fed:     	incq	%rdx
140259ff0:     	movq	%rdx, 0x10(%rbx)
140259ff4:     	leaq	0x28(%rsi), %r9
140259ff8:     	movq	%rdi, 0x28(%rsp)
140259ffd:     	movb	$0x1, 0x30(%rsp)
14025a002:     	leaq	0x274aca5(%rip), %rdx   # 0x1429a4cae
14025a009:     	leaq	0x28(%rsp), %rcx
14025a00e:     	movl	$0xa, %r8d
14025a014:     	callq	0x1404c11a0 <.text+0x4c01a0>
14025a019:     	testq	%rax, %rax
14025a01c:     	jne	0x14025a3ab <.text+0x2593ab>
14025a022:     	leaq	0x4(%rsi), %r9
14025a026:     	leaq	0x274acae(%rip), %rdx   # 0x1429a4cdb
14025a02d:     	leaq	0x28(%rsp), %rcx
14025a032:     	movl	$0xb, %r8d
14025a038:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a03d:     	testq	%rax, %rax
14025a040:     	jne	0x14025a3ab <.text+0x2593ab>
14025a046:     	leaq	0x8(%rsi), %r9
14025a04a:     	leaq	0x274ac95(%rip), %rdx   # 0x1429a4ce6
14025a051:     	leaq	0x28(%rsp), %rcx
14025a056:     	movl	$0xc, %r8d
14025a05c:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a061:     	testq	%rax, %rax
14025a064:     	jne	0x14025a3ab <.text+0x2593ab>
14025a06a:     	leaq	0x30(%rsi), %r9
14025a06e:     	leaq	0x274ac7d(%rip), %rdx   # 0x1429a4cf2
14025a075:     	leaq	0x28(%rsp), %rcx
14025a07a:     	movl	$0xf, %r8d
14025a080:     	callq	0x1404b98d0 <.text+0x4b88d0>
14025a085:     	testq	%rax, %rax
14025a088:     	jne	0x14025a3ab <.text+0x2593ab>
14025a08e:     	leaq	0xc(%rsi), %r9
14025a092:     	leaq	0x274ac68(%rip), %rdx   # 0x1429a4d01
14025a099:     	leaq	0x28(%rsp), %rcx
14025a09e:     	movl	$0xa, %r8d
14025a0a4:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a0a9:     	testq	%rax, %rax
14025a0ac:     	jne	0x14025a3ab <.text+0x2593ab>
14025a0b2:     	leaq	0x10(%rsi), %r9
14025a0b6:     	leaq	0x274ac4e(%rip), %rdx   # 0x1429a4d0b
14025a0bd:     	leaq	0x28(%rsp), %rcx
14025a0c2:     	movl	$0x9, %r8d
14025a0c8:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a0cd:     	testq	%rax, %rax
14025a0d0:     	jne	0x14025a3ab <.text+0x2593ab>
14025a0d6:     	leaq	0x14(%rsi), %r9
14025a0da:     	leaq	0x274ac33(%rip), %rdx   # 0x1429a4d14
14025a0e1:     	leaq	0x28(%rsp), %rcx
14025a0e6:     	movl	$0xa, %r8d
14025a0ec:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a0f1:     	testq	%rax, %rax
14025a0f4:     	jne	0x14025a3ab <.text+0x2593ab>
14025a0fa:     	leaq	0x18(%rsi), %r9
14025a0fe:     	leaq	0x274ac19(%rip), %rdx   # 0x1429a4d1e
14025a105:     	leaq	0x28(%rsp), %rcx
14025a10a:     	movl	$0xb, %r8d
14025a110:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a115:     	testq	%rax, %rax
14025a118:     	jne	0x14025a3ab <.text+0x2593ab>
14025a11e:     	leaq	0x38(%rsi), %r9
14025a122:     	leaq	0x274ac00(%rip), %rdx   # 0x1429a4d29
14025a129:     	leaq	0x28(%rsp), %rcx
14025a12e:     	movl	$0xe, %r8d
14025a134:     	callq	0x1404b98d0 <.text+0x4b88d0>
14025a139:     	testq	%rax, %rax
14025a13c:     	jne	0x14025a3ab <.text+0x2593ab>
14025a142:     	leaq	0x1c(%rsi), %r9
14025a146:     	leaq	0x274abea(%rip), %rdx   # 0x1429a4d37
14025a14d:     	leaq	0x28(%rsp), %rcx
14025a152:     	movl	$0x9, %r8d
14025a158:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a15d:     	testq	%rax, %rax
14025a160:     	jne	0x14025a3ab <.text+0x2593ab>
14025a166:     	addq	$0x20, %rsi
14025a16a:     	leaq	0x274abcf(%rip), %rdx   # 0x1429a4d40
14025a171:     	leaq	0x28(%rsp), %rcx
14025a176:     	movl	$0x8, %r8d
14025a17c:     	movq	%rsi, %r9
14025a17f:     	callq	0x1404bdc80 <.text+0x4bcc80>
14025a184:     	jmp	0x14025a388 <.text+0x259388>
14025a189:     	leaq	0x8(%rdi), %rdx
14025a18d:     	movq	(%rdi), %rbx
14025a190:     	movq	0x10(%rbx), %rax
14025a194:     	cmpq	%rax, (%rbx)
14025a197:     	je	0x14025a460 <.text+0x259460>
14025a19d:     	movq	0x8(%rbx), %rcx
14025a1a1:     	movb	$0x7b, (%rcx,%rax)
14025a1a5:     	incq	%rax
14025a1a8:     	movq	%rax, 0x10(%rbx)
14025a1ac:     	leaq	0x274ab95(%rip), %r8    # 0x1429a4d48
14025a1b3:     	movl	$0x12, %r9d
14025a1b9:     	movq	%rdi, %rcx
14025a1bc:     	callq	0x1404b6100 <.text+0x4b5100>
14025a1c1:     	testq	%rax, %rax
14025a1c4:     	jne	0x14025a3a3 <.text+0x2593a3>
14025a1ca:     	movq	(%rdi), %rbx
14025a1cd:     	movq	0x10(%rbx), %rdx
14025a1d1:     	cmpq	%rdx, (%rbx)
14025a1d4:     	je	0x14025a585 <.text+0x259585>
14025a1da:     	movq	0x8(%rbx), %rax
14025a1de:     	movb	$0x3a, (%rax,%rdx)
14025a1e2:     	incq	%rdx
14025a1e5:     	movq	%rdx, 0x10(%rbx)
14025a1e9:     	movq	(%rdi), %rbx
14025a1ec:     	movq	0x10(%rbx), %rdx
14025a1f0:     	cmpq	%rdx, (%rbx)
14025a1f3:     	je	0x14025a5ab <.text+0x2595ab>
14025a1f9:     	movq	0x8(%rbx), %rax
14025a1fd:     	movb	$0x7b, (%rax,%rdx)
14025a201:     	incq	%rdx
14025a204:     	movq	%rdx, 0x10(%rbx)
14025a208:     	leaq	0x8(%rsi), %r9
14025a20c:     	movq	%rdi, 0x28(%rsp)
14025a211:     	movb	$0x1, 0x30(%rsp)
14025a216:     	leaq	0x274aa91(%rip), %rdx   # 0x1429a4cae
14025a21d:     	leaq	0x28(%rsp), %rcx
14025a222:     	movl	$0xa, %r8d
14025a228:     	callq	0x1404c11a0 <.text+0x4c01a0>
14025a22d:     	testq	%rax, %rax
14025a230:     	jne	0x14025a3ab <.text+0x2593ab>
14025a236:     	leaq	0x1(%rsi), %r9
14025a23a:     	leaq	0x274ab19(%rip), %rdx   # 0x1429a4d5a
14025a241:     	leaq	0x28(%rsp), %rcx
14025a246:     	movl	$0x4, %r8d
14025a24c:     	callq	0x1404ba9b0 <.text+0x4b99b0>
14025a251:     	testq	%rax, %rax
14025a254:     	jne	0x14025a3ab <.text+0x2593ab>
14025a25a:     	addq	$0x2, %rsi
14025a25e:     	leaq	0x274aaf9(%rip), %rdx   # 0x1429a4d5e
14025a265:     	leaq	0x28(%rsp), %rcx
14025a26a:     	movl	$0x2, %r8d
14025a270:     	movq	%rsi, %r9
14025a273:     	callq	0x1404ba9b0 <.text+0x4b99b0>
14025a278:     	jmp	0x14025a388 <.text+0x259388>
14025a27d:     	leaq	0x8(%rdi), %rdx
14025a281:     	movq	(%rdi), %rbx
14025a284:     	movq	0x10(%rbx), %rax
14025a288:     	cmpq	%rax, (%rbx)
14025a28b:     	je	0x14025a4be <.text+0x2594be>
14025a291:     	movq	0x8(%rbx), %rcx
14025a295:     	movb	$0x7b, (%rcx,%rax)
14025a299:     	incq	%rax
14025a29c:     	movq	%rax, 0x10(%rbx)
14025a2a0:     	leaq	0x274aa20(%rip), %r8    # 0x1429a4cc7
14025a2a7:     	movl	$0xa, %r9d
14025a2ad:     	movq	%rdi, %rcx
14025a2b0:     	callq	0x1404b6100 <.text+0x4b5100>
14025a2b5:     	testq	%rax, %rax
14025a2b8:     	jne	0x14025a3a3 <.text+0x2593a3>
14025a2be:     	movq	(%rdi), %rbx
14025a2c1:     	movq	0x10(%rbx), %rdx
14025a2c5:     	cmpq	%rdx, (%rbx)
14025a2c8:     	je	0x14025a3b6 <.text+0x2593b6>
14025a2ce:     	movq	0x8(%rbx), %rax
14025a2d2:     	movb	$0x3a, (%rax,%rdx)
14025a2d6:     	incq	%rdx
14025a2d9:     	movq	%rdx, 0x10(%rbx)
14025a2dd:     	movq	(%rdi), %rbx
14025a2e0:     	movq	0x10(%rbx), %rdx
14025a2e4:     	cmpq	%rdx, (%rbx)
14025a2e7:     	je	0x14025a3dc <.text+0x2593dc>
14025a2ed:     	movq	0x8(%rbx), %rax
14025a2f1:     	movb	$0x7b, (%rax,%rdx)
14025a2f5:     	incq	%rdx
14025a2f8:     	movq	%rdx, 0x10(%rbx)
14025a2fc:     	leaq	0x30(%rsi), %r9
14025a300:     	movq	%rdi, 0x28(%rsp)
14025a305:     	movb	$0x1, 0x30(%rsp)
14025a30a:     	leaq	0x274a99d(%rip), %rdx   # 0x1429a4cae
14025a311:     	leaq	0x28(%rsp), %rcx
14025a316:     	movl	$0xa, %r8d
14025a31c:     	callq	0x1404c11a0 <.text+0x4c01a0>
14025a321:     	testq	%rax, %rax
14025a324:     	jne	0x14025a3ab <.text+0x2593ab>
14025a32a:     	leaq	0x14(%rsi), %r9
14025a32e:     	leaq	0x274a88d(%rip), %rdx   # 0x1429a4bc2
14025a335:     	leaq	0x28(%rsp), %rcx
14025a33a:     	movl	$0x6, %r8d
14025a340:     	callq	0x1404bb130 <.text+0x4ba130>
14025a345:     	testq	%rax, %rax
14025a348:     	jne	0x14025a3ab <.text+0x2593ab>
14025a34a:     	leaq	0x4(%rsi), %r9
14025a34e:     	leaq	0x274a8a3(%rip), %rdx   # 0x1429a4bf8
14025a355:     	leaq	0x28(%rsp), %rcx
14025a35a:     	movl	$0x8, %r8d
14025a360:     	callq	0x1404baab0 <.text+0x4b9ab0>
14025a365:     	testq	%rax, %rax
14025a368:     	jne	0x14025a3ab <.text+0x2593ab>
14025a36a:     	addq	$0x8, %rsi
14025a36e:     	leaq	0x274a943(%rip), %rdx   # 0x1429a4cb8
14025a375:     	leaq	0x28(%rsp), %rcx
14025a37a:     	movl	$0xf, %r8d
14025a380:     	movq	%rsi, %r9
14025a383:     	callq	0x1404be300 <.text+0x4bd300>
14025a388:     	testq	%rax, %rax
14025a38b:     	jne	0x14025a3ab <.text+0x2593ab>
14025a38d:     	movq	0x28(%rsp), %rax
14025a392:     	movzbl	0x30(%rsp), %edx
14025a397:     	movq	(%rax), %rcx
14025a39a:     	callq	0x14003e0a0 <.text+0x3d0a0>
14025a39f:     	xorl	%eax, %eax
14025a3a1:     	jmp	0x14025a3ab <.text+0x2593ab>
14025a3a3:     	movq	%rax, %rcx
14025a3a6:     	callq	0x1428c98a0 <.text+0x28c88a0>
14025a3ab:     	nop
14025a3ac:     	addq	$0x38, %rsp
14025a3b0:     	popq	%rbx
14025a3b1:     	popq	%rdi
14025a3b2:     	popq	%rsi
14025a3b3:     	popq	%r14
14025a3b5:     	retq
14025a3b6:     	movq	$0x1, 0x20(%rsp)
14025a3bf:     	movl	$0x1, %r8d
14025a3c5:     	movl	$0x1, %r9d
14025a3cb:     	movq	%rbx, %rcx
14025a3ce:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a3d3:     	movq	0x10(%rbx), %rdx
14025a3d7:     	jmp	0x14025a2ce <.text+0x2592ce>
14025a3dc:     	movq	$0x1, 0x20(%rsp)
14025a3e5:     	movl	$0x1, %r8d
14025a3eb:     	movl	$0x1, %r9d
14025a3f1:     	movq	%rbx, %rcx
14025a3f4:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a3f9:     	movq	0x10(%rbx), %rdx
14025a3fd:     	jmp	0x14025a2ed <.text+0x2592ed>
14025a402:     	movq	$0x1, 0x20(%rsp)
14025a40b:     	movl	$0x1, %r8d
14025a411:     	movl	$0x1, %r9d
14025a417:     	movq	%rbx, %rcx
14025a41a:     	movq	%rdx, %r14
14025a41d:     	movq	%rax, %rdx
14025a420:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a425:     	movq	%r14, %rdx
14025a428:     	movq	0x10(%rbx), %rax
14025a42c:     	jmp	0x140259e95 <.text+0x258e95>
14025a431:     	movq	$0x1, 0x20(%rsp)
14025a43a:     	movl	$0x1, %r8d
14025a440:     	movl	$0x1, %r9d
14025a446:     	movq	%rbx, %rcx
14025a449:     	movq	%rdx, %r14
14025a44c:     	movq	%rax, %rdx
14025a44f:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a454:     	movq	%r14, %rdx
14025a457:     	movq	0x10(%rbx), %rax
14025a45b:     	jmp	0x140259f89 <.text+0x258f89>
14025a460:     	movq	$0x1, 0x20(%rsp)
14025a469:     	movl	$0x1, %r8d
14025a46f:     	movl	$0x1, %r9d
14025a475:     	movq	%rbx, %rcx
14025a478:     	movq	%rdx, %r14
14025a47b:     	movq	%rax, %rdx
14025a47e:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a483:     	movq	%r14, %rdx
14025a486:     	movq	0x10(%rbx), %rax
14025a48a:     	jmp	0x14025a19d <.text+0x25919d>
14025a48f:     	movq	$0x1, 0x20(%rsp)
14025a498:     	movl	$0x1, %r8d
14025a49e:     	movl	$0x1, %r9d
14025a4a4:     	movq	%rbx, %rcx
14025a4a7:     	movq	%rdx, %r14
14025a4aa:     	movq	%rax, %rdx
14025a4ad:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a4b2:     	movq	%r14, %rdx
14025a4b5:     	movq	0x10(%rbx), %rax
14025a4b9:     	jmp	0x140259e66 <.text+0x258e66>
14025a4be:     	movq	$0x1, 0x20(%rsp)
14025a4c7:     	movl	$0x1, %r8d
14025a4cd:     	movl	$0x1, %r9d
14025a4d3:     	movq	%rbx, %rcx
14025a4d6:     	movq	%rdx, %r14
14025a4d9:     	movq	%rax, %rdx
14025a4dc:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a4e1:     	movq	%r14, %rdx
14025a4e4:     	movq	0x10(%rbx), %rax
14025a4e8:     	jmp	0x14025a291 <.text+0x259291>
14025a4ed:     	movq	$0x1, 0x20(%rsp)
14025a4f6:     	movl	$0x1, %r8d
14025a4fc:     	movl	$0x1, %r9d
14025a502:     	movq	%rbx, %rcx
14025a505:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a50a:     	movq	0x10(%rbx), %rdx
14025a50e:     	jmp	0x140259ed2 <.text+0x258ed2>
14025a513:     	movq	$0x1, 0x20(%rsp)
14025a51c:     	movl	$0x1, %r8d
14025a522:     	movl	$0x1, %r9d
14025a528:     	movq	%rbx, %rcx
14025a52b:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a530:     	movq	0x10(%rbx), %rdx
14025a534:     	jmp	0x140259ef1 <.text+0x258ef1>
14025a539:     	movq	$0x1, 0x20(%rsp)
14025a542:     	movl	$0x1, %r8d
14025a548:     	movl	$0x1, %r9d
14025a54e:     	movq	%rbx, %rcx
14025a551:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a556:     	movq	0x10(%rbx), %rdx
14025a55a:     	jmp	0x140259fc6 <.text+0x258fc6>
14025a55f:     	movq	$0x1, 0x20(%rsp)
14025a568:     	movl	$0x1, %r8d
14025a56e:     	movl	$0x1, %r9d
14025a574:     	movq	%rbx, %rcx
14025a577:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a57c:     	movq	0x10(%rbx), %rdx
14025a580:     	jmp	0x140259fe5 <.text+0x258fe5>
14025a585:     	movq	$0x1, 0x20(%rsp)
14025a58e:     	movl	$0x1, %r8d
14025a594:     	movl	$0x1, %r9d
14025a59a:     	movq	%rbx, %rcx
14025a59d:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a5a2:     	movq	0x10(%rbx), %rdx
14025a5a6:     	jmp	0x14025a1da <.text+0x2591da>
14025a5ab:     	movq	$0x1, 0x20(%rsp)
14025a5b4:     	movl	$0x1, %r8d
14025a5ba:     	movl	$0x1, %r9d
14025a5c0:     	movq	%rbx, %rcx
14025a5c3:     	callq	0x1428a29e0 <.text+0x28a19e0>
14025a5c8:     	movq	0x10(%rbx), %rdx
14025a5cc:     	jmp	0x14025a1f9 <.text+0x2591f9>
