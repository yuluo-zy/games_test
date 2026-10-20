
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140873de0:     	pushq	%rbp
140873de1:     	pushq	%r15
140873de3:     	pushq	%r14
140873de5:     	pushq	%r13
140873de7:     	pushq	%r12
140873de9:     	pushq	%rsi
140873dea:     	pushq	%rdi
140873deb:     	pushq	%rbx
140873dec:     	subq	$0xb8, %rsp
140873df3:     	leaq	0x80(%rsp), %rbp
140873dfb:     	movq	$-0x2, 0x30(%rbp)
140873e03:     	movq	%rdx, %rsi
140873e06:     	movq	%rcx, %rdi
140873e09:     	callq	0x140c16a00 <.text+0xc15a00>
140873e0e:     	testb	%al, %al
140873e10:     	je	0x140873e71 <.text+0x872e71>
140873e12:     	leaq	0x22240df(%rip), %r14   # 0x142a97ef8
140873e19:     	movl	$0x35, %ebx
140873e1e:     	movl	$0x35, %edx
140873e23:     	movq	%r14, %rcx
140873e26:     	callq	0x140872150 <.text+0x871150>
140873e2b:     	cmpq	$0x1, %rax
140873e2f:     	jne	0x140873ee6 <.text+0x872ee6>
140873e35:     	movq	%rdx, %r9
140873e38:     	testq	%rdx, %rdx
140873e3b:     	je	0x140873e7a <.text+0x872e7a>
140873e3d:     	cmpq	$0x35, %r9
140873e41:     	jae	0x140873e78 <.text+0x872e78>
140873e43:     	leaq	0x22240ae(%rip), %rax   # 0x142a97ef8
140873e4a:     	cmpb	$-0x41, (%r9,%rax)
140873e4f:     	jg	0x140873e7a <.text+0x872e7a>
140873e51:     	leaq	0x2224198(%rip), %rax   # 0x142a97ff0
140873e58:     	movq	%rax, 0x20(%rsp)
140873e5d:     	leaq	0x2224094(%rip), %rcx   # 0x142a97ef8
140873e64:     	movl	$0x35, %edx
140873e69:     	xorl	%r8d, %r8d
140873e6c:     	callq	0x1428d9940 <.text+0x28d8940>
140873e71:     	xorl	%eax, %eax
140873e73:     	jmp	0x140874046 <.text+0x873046>
140873e78:     	jne	0x140873e51 <.text+0x872e51>
140873e7a:     	leaq	0x2224077(%rip), %r14   # 0x142a97ef8
140873e81:     	movq	%r14, %rcx
140873e84:     	movq	%r9, %rdx
140873e87:     	callq	0x140872150 <.text+0x871150>
140873e8c:     	cmpq	$0x1, %rax
140873e90:     	jne	0x140873ee6 <.text+0x872ee6>
140873e92:     	movq	%rdx, %r8
140873e95:     	addq	$0x2, %r8
140873e99:     	je	0x140873ed4 <.text+0x872ed4>
140873e9b:     	cmpq	$0x35, %r8
140873e9f:     	jae	0x140873ed2 <.text+0x872ed2>
140873ea1:     	leaq	0x2224050(%rip), %rax   # 0x142a97ef8
140873ea8:     	cmpb	$-0x41, (%r8,%rax)
140873ead:     	jg	0x140873ed4 <.text+0x872ed4>
140873eaf:     	leaq	0x2224152(%rip), %rax   # 0x142a98008
140873eb6:     	movq	%rax, 0x20(%rsp)
140873ebb:     	leaq	0x2224036(%rip), %rcx   # 0x142a97ef8
140873ec2:     	movl	$0x35, %edx
140873ec7:     	movl	$0x35, %r9d
140873ecd:     	callq	0x1428d9940 <.text+0x28d8940>
140873ed2:     	jne	0x140873eaf <.text+0x872eaf>
140873ed4:     	movl	$0x33, %ebx
140873ed9:     	subq	%rdx, %rbx
140873edc:     	leaq	0x2224015(%rip), %r14   # 0x142a97ef8
140873ee3:     	addq	%r8, %r14
140873ee6:     	leaq	0x222434c(%rip), %rax   # 0x142a98239
140873eed:     	leaq	0x2224314(%rip), %r15   # 0x142a98208
140873ef4:     	nopw	%cs:(%rax,%rax)
140873f00:     	movsbl	-0x1(%rax), %ecx
140873f04:     	testl	%ecx, %ecx
140873f06:     	js	0x140873f20 <.text+0x872f20>
140873f08:     	decq	%rax
140873f0b:     	cmpl	$0x5c, %ecx
140873f0e:     	jne	0x140873f79 <.text+0x872f79>
140873f10:     	jmp	0x140873f8f <.text+0x872f8f>
140873f12:     	nopw	%cs:(%rax,%rax)
140873f20:     	movzbl	-0x2(%rax), %edx
140873f24:     	cmpb	$-0x40, %dl
140873f27:     	jge	0x140873f4e <.text+0x872f4e>
140873f29:     	movzbl	-0x3(%rax), %r8d
140873f2e:     	cmpb	$-0x40, %r8b
140873f32:     	jge	0x140873f57 <.text+0x872f57>
140873f34:     	movzbl	-0x4(%rax), %r9d
140873f39:     	addq	$-0x4, %rax
140873f3d:     	andl	$0x7, %r9d
140873f41:     	shll	$0x6, %r9d
140873f45:     	andl	$0x3f, %r8d
140873f49:     	orl	%r9d, %r8d
140873f4c:     	jmp	0x140873f5f <.text+0x872f5f>
140873f4e:     	addq	$-0x2, %rax
140873f52:     	andl	$0x1f, %edx
140873f55:     	jmp	0x140873f69 <.text+0x872f69>
140873f57:     	addq	$-0x3, %rax
140873f5b:     	andl	$0xf, %r8d
140873f5f:     	shll	$0x6, %r8d
140873f63:     	andl	$0x3f, %edx
140873f66:     	orl	%r8d, %edx
140873f69:     	shll	$0x6, %edx
140873f6c:     	andb	$0x3f, %cl
140873f6f:     	movzbl	%cl, %ecx
140873f72:     	orl	%edx, %ecx
140873f74:     	cmpl	$0x5c, %ecx
140873f77:     	je	0x140873f8f <.text+0x872f8f>
140873f79:     	cmpl	$0x2f, %ecx
140873f7c:     	je	0x140873f8f <.text+0x872f8f>
140873f7e:     	cmpq	%r15, %rax
140873f81:     	jne	0x140873f00 <.text+0x872f00>
140873f87:     	movl	$0x31, %r12d
140873f8d:     	jmp	0x140873fdf <.text+0x872fdf>
140873f8f:     	leaq	0x2224272(%rip), %r15   # 0x142a98208
140873f96:     	subq	%r15, %rax
140873f99:     	movq	%rax, %r8
140873f9c:     	incq	%r8
140873f9f:     	je	0x140873fd3 <.text+0x872fd3>
140873fa1:     	cmpq	$0x31, %r8
140873fa5:     	jae	0x140873fd1 <.text+0x872fd1>
140873fa7:     	cmpb	$-0x41, (%r8,%r15)
140873fac:     	jg	0x140873fd3 <.text+0x872fd3>
140873fae:     	leaq	0x222401b(%rip), %rax   # 0x142a97fd0
140873fb5:     	movq	%rax, 0x20(%rsp)
140873fba:     	leaq	0x2224247(%rip), %rcx   # 0x142a98208
140873fc1:     	movl	$0x31, %edx
140873fc6:     	movl	$0x31, %r9d
140873fcc:     	callq	0x1428d9940 <.text+0x28d8940>
140873fd1:     	jne	0x140873fae <.text+0x872fae>
140873fd3:     	movl	$0x30, %r12d
140873fd9:     	subq	%rax, %r12
140873fdc:     	addq	%r8, %r15
140873fdf:     	callq	0x140c18130 <.text+0xc17130>
140873fe4:     	movq	%rax, %rcx
140873fe7:     	movq	(%rax), %rax
140873fea:     	cmpq	$0x1, %rax
140873fee:     	jne	0x14087429c <.text+0x87329c>
140873ff4:     	addq	$0x8, %rcx
140873ff8:     	cmpq	$0x0, (%rcx)
140873ffc:     	jne	0x1408742c0 <.text+0x8732c0>
140874002:     	movq	$-0x1, (%rcx)
140874009:     	movq	%rcx, 0x28(%rbp)
14087400d:     	addq	$0x8, %rcx
140874011:     	movq	%r12, 0x20(%rsp)
140874016:     	movq	$0x0, 0x30(%rsp)
14087401f:     	movq	$0x1, 0x28(%rsp)
140874028:     	movq	%r14, %rdx
14087402b:     	movq	%rbx, %r8
14087402e:     	movq	%r15, %r9
140874031:     	callq	0x140c16d40 <.text+0xc15d40>
140874036:     	movq	0x28(%rbp), %rcx
14087403a:     	incq	(%rcx)
14087403d:     	movq	%rax, -0x8(%rbp)
140874041:     	movl	$0x1, %eax
140874046:     	movq	%rax, -0x10(%rbp)
14087404a:     	movq	(%rdi), %rcx
14087404d:     	movq	0x8(%rdi), %rax
140874051:     	movq	%rax, 0x20(%rbp)
140874055:     	movl	0x14(%rdi), %eax
140874058:     	movl	%eax, 0x14(%rbp)
14087405b:     	movq	0xf8(%rcx), %rbx
140874062:     	movq	0x100(%rcx), %rax
140874069:     	leaq	(%rbx,%rax,4), %r12
14087406d:     	movq	%rcx, 0x8(%rbp)
140874071:     	movzbl	0x108(%rcx), %eax
140874078:     	movb	%al, 0x28(%rbp)
14087407b:     	movq	(%rsi), %rax
14087407e:     	movq	%rax, -0x18(%rbp)
140874082:     	movl	$0x8, %eax
140874087:     	movq	%rax, (%rbp)
14087408b:     	xorl	%esi, %esi
14087408d:     	leaq	-0x44(%rbp), %r15
140874091:     	xorl	%r13d, %r13d
140874094:     	xorl	%edi, %edi
140874096:     	cmpb	$0x0, 0x28(%rbp)
14087409a:     	je	0x1408740e0 <.text+0x8730e0>
14087409c:     	cmpq	%r13, %rdi
14087409f:     	jne	0x14087416a <.text+0x87316a>
1408740a5:     	cmpq	%r12, %rbx
1408740a8:     	je	0x140874231 <.text+0x873231>
1408740ae:     	movq	0x20(%rbp), %rax
1408740b2:     	movq	0x1a8(%rax), %rax
1408740b9:     	nopl	(%rax)
1408740c0:     	movl	(%rbx), %ecx
1408740c2:     	leaq	(%rcx,%rcx,8), %rcx
1408740c6:     	movq	0x10(%rax,%rcx,8), %r13
1408740cb:     	testq	%r13, %r13
1408740ce:     	jne	0x140874130 <.text+0x873130>
1408740d0:     	addq	$0x4, %rbx
1408740d4:     	cmpq	%r12, %rbx
1408740d7:     	jne	0x1408740c0 <.text+0x8730c0>
1408740d9:     	jmp	0x140874231 <.text+0x873231>
1408740de:     	nop
1408740e0:     	cmpq	%r13, %rdi
1408740e3:     	jne	0x1408741c8 <.text+0x8731c8>
1408740e9:     	cmpq	%r12, %rbx
1408740ec:     	je	0x140874231 <.text+0x873231>
1408740f2:     	movq	0x20(%rbp), %rax
1408740f6:     	movq	0x100(%rax), %rax
1408740fd:     	nopl	(%rax)
140874100:     	movl	(%rbx), %ecx
140874102:     	leaq	(%rcx,%rcx,4), %rcx
140874106:     	shlq	$0x5, %rcx
14087410a:     	movq	0x58(%rax,%rcx), %r13
14087410f:     	testq	%r13, %r13
140874112:     	jne	0x140874170 <.text+0x873170>
140874114:     	addq	$0x4, %rbx
140874118:     	cmpq	%r12, %rbx
14087411b:     	jne	0x140874100 <.text+0x873100>
14087411d:     	jmp	0x140874231 <.text+0x873231>
140874122:     	nopw	%cs:(%rax,%rax)
140874130:     	leaq	(%rax,%rcx,8), %rax
140874134:     	movq	0x8(%rbp), %rcx
140874138:     	movq	0x110(%rcx), %rcx
14087413f:     	movq	0x18(%rax), %rdx
140874143:     	movq	0x38(%rax), %rax
140874147:     	movq	(%rax,%rcx,8), %rax
14087414b:     	notq	%rax
14087414e:     	leaq	(%rax,%rax,2), %rax
140874152:     	shlq	$0x4, %rax
140874156:     	movq	0x10(%rdx,%rax), %rsi
14087415b:     	movq	0x28(%rdx,%rax), %rax
140874160:     	movq	%rax, 0x18(%rbp)
140874164:     	addq	$0x4, %rbx
140874168:     	xorl	%edi, %edi
14087416a:     	movl	%edi, %eax
14087416c:     	jmp	0x1408741d7 <.text+0x8731d7>
14087416e:     	nop
140874170:     	addq	%rcx, %rax
140874173:     	movl	0x94(%rax), %ecx
140874179:     	movq	0x20(%rbp), %rdx
14087417d:     	movq	0x1a8(%rdx), %rdx
140874184:     	leaq	(%rcx,%rcx,8), %rcx
140874188:     	movq	0x8(%rbp), %r8
14087418c:     	movq	0x110(%r8), %r8
140874193:     	movq	0x18(%rdx,%rcx,8), %r9
140874198:     	movq	0x38(%rdx,%rcx,8), %rcx
14087419d:     	movq	(%rcx,%r8,8), %rcx
1408741a1:     	notq	%rcx
1408741a4:     	leaq	(%rcx,%rcx,2), %rcx
1408741a8:     	shlq	$0x4, %rcx
1408741ac:     	movq	0x10(%r9,%rcx), %rsi
1408741b1:     	movq	0x28(%r9,%rcx), %rcx
1408741b6:     	movq	%rcx, 0x18(%rbp)
1408741ba:     	movq	0x50(%rax), %rax
1408741be:     	movq	%rax, (%rbp)
1408741c2:     	addq	$0x4, %rbx
1408741c6:     	xorl	%edi, %edi
1408741c8:     	movq	%rdi, %rax
1408741cb:     	shlq	$0x4, %rax
1408741cf:     	movq	(%rbp), %rcx
1408741d3:     	movl	0x8(%rcx,%rax), %eax
1408741d7:     	leaq	(%rax,%rax,2), %r14
1408741db:     	shlq	$0x4, %r14
1408741df:     	addq	%rsi, %r14
1408741e2:     	movq	0x18(%rbp), %rcx
1408741e6:     	leaq	(%rcx,%rax,4), %rax
1408741ea:     	movl	0x14(%rbp), %ecx
1408741ed:     	movl	%ecx, (%rax)
1408741ef:     	movq	%r15, %rcx
1408741f2:     	movq	-0x18(%rbp), %rdx
1408741f6:     	callq	0x140c96d40 <.text+0xc95d40>
1408741fb:     	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
140874205:     	movq	%rax, -0x28(%rbp)
140874209:     	movl	$0x0, -0x20(%rbp)
140874210:     	movq	%r14, %rcx
140874213:     	movq	%r15, %rdx
140874216:     	leaq	-0x28(%rbp), %r8
14087421a:     	callq	0x140c9ce00 <.text+0xc9be00>
14087421f:     	incq	%rdi
140874222:     	cmpb	$0x0, 0x28(%rbp)
140874226:     	je	0x1408740e0 <.text+0x8730e0>
14087422c:     	jmp	0x14087409c <.text+0x87309c>
140874231:     	cmpq	$0x0, -0x10(%rbp)
140874236:     	je	0x140874272 <.text+0x873272>
140874238:     	callq	0x140c18130 <.text+0xc17130>
14087423d:     	movq	%rax, %rcx
140874240:     	movq	(%rax), %rax
140874243:     	cmpq	$0x1, %rax
140874247:     	jne	0x140874286 <.text+0x873286>
140874249:     	addq	$0x8, %rcx
14087424d:     	cmpq	$0x0, (%rcx)
140874251:     	jne	0x1408742c0 <.text+0x8732c0>
140874253:     	movq	$-0x1, (%rcx)
14087425a:     	movq	%rcx, 0x28(%rbp)
14087425e:     	addq	$0x8, %rcx
140874262:     	movq	-0x8(%rbp), %rdx
140874266:     	callq	0x140c170f0 <.text+0xc160f0>
14087426b:     	movq	0x28(%rbp), %rax
14087426f:     	incq	(%rax)
140874272:     	addq	$0xb8, %rsp
140874279:     	popq	%rbx
14087427a:     	popq	%rdi
14087427b:     	popq	%rsi
14087427c:     	popq	%r12
14087427e:     	popq	%r13
140874280:     	popq	%r14
140874282:     	popq	%r15
140874284:     	popq	%rbp
140874285:     	retq
140874286:     	testq	%rax, %rax
140874289:     	jne	0x1408742b4 <.text+0x8732b4>
14087428b:     	xorl	%edx, %edx
14087428d:     	callq	0x1428f87d0 <.text+0x28f77d0>
140874292:     	movq	%rax, %rcx
140874295:     	testq	%rax, %rax
140874298:     	jne	0x14087424d <.text+0x87324d>
14087429a:     	jmp	0x1408742b4 <.text+0x8732b4>
14087429c:     	testq	%rax, %rax
14087429f:     	jne	0x1408742b4 <.text+0x8732b4>
1408742a1:     	xorl	%edx, %edx
1408742a3:     	callq	0x1428f87d0 <.text+0x28f77d0>
1408742a8:     	movq	%rax, %rcx
1408742ab:     	testq	%rax, %rax
1408742ae:     	jne	0x140873ff8 <.text+0x872ff8>
1408742b4:     	leaq	0x23c67e5(%rip), %rcx   # 0x142c3aaa0
1408742bb:     	callq	0x1428d80d0 <.text+0x28d70d0>
1408742c0:     	leaq	0x23c68a9(%rip), %rcx   # 0x142c3ab70
1408742c7:     	callq	0x1428d9250 <.text+0x28d8250>
1408742cc:     	int3
1408742cd:     	nopl	(%rax)
1408742d0:     	movq	%rdx, 0x10(%rsp)
1408742d5:     	pushq	%rbp
1408742d6:     	pushq	%r15
1408742d8:     	pushq	%r14
1408742da:     	pushq	%r13
1408742dc:     	pushq	%r12
1408742de:     	pushq	%rsi
1408742df:     	pushq	%rdi
1408742e0:     	pushq	%rbx
1408742e1:     	subq	$0x38, %rsp
1408742e5:     	leaq	0x80(%rdx), %rbp
1408742ec:     	movq	0x28(%rbp), %rax
1408742f0:     	incq	(%rax)
1408742f3:     	addq	$0x38, %rsp
1408742f7:     	popq	%rbx
1408742f8:     	popq	%rdi
1408742f9:     	popq	%rsi
1408742fa:     	popq	%r12
1408742fc:     	popq	%r13
1408742fe:     	popq	%r14
140874300:     	popq	%r15
140874302:     	popq	%rbp
140874303:     	retq
140874304:     	nopw	%cs:(%rax,%rax)
140874310:     	movq	%rdx, 0x10(%rsp)
140874315:     	pushq	%rbp
140874316:     	pushq	%r15
140874318:     	pushq	%r14
14087431a:     	pushq	%r13
14087431c:     	pushq	%r12
14087431e:     	pushq	%rsi
14087431f:     	pushq	%rdi
140874320:     	pushq	%rbx
140874321:     	subq	$0x38, %rsp
140874325:     	leaq	0x80(%rdx), %rbp
14087432c:     	cmpq	$0x0, -0x10(%rbp)
140874331:     	je	0x140874353 <.text+0x873353>
140874333:     	leaq	-0x8(%rbp), %rax
140874337:     	movq	%rax, -0x30(%rbp)
14087433b:     	leaq	-0x30(%rbp), %rax
14087433f:     	movq	%rax, -0x38(%rbp)
140874343:     	leaq	0x2223c26(%rip), %rcx   # 0x142a97f70
14087434a:     	leaq	-0x38(%rbp), %rdx
14087434e:     	callq	0x14132f2d0 <.text+0x132e2d0>
140874353:     	nop
140874354:     	addq	$0x38, %rsp
140874358:     	popq	%rbx
140874359:     	popq	%rdi
14087435a:     	popq	%rsi
14087435b:     	popq	%r12
14087435d:     	popq	%r13
14087435f:     	popq	%r14
140874361:     	popq	%r15
140874363:     	popq	%rbp
140874364:     	retq
140874365:     	nopw	%cs:(%rax,%rax)
140874370:     	movq	%rdx, 0x10(%rsp)
140874375:     	pushq	%rbp
140874376:     	pushq	%r15
140874378:     	pushq	%r14
14087437a:     	pushq	%r13
14087437c:     	pushq	%r12
14087437e:     	pushq	%rsi
14087437f:     	pushq	%rdi
140874380:     	pushq	%rbx
140874381:     	subq	$0x38, %rsp
140874385:     	leaq	0x80(%rdx), %rbp
14087438c:     	movq	0x28(%rbp), %rax
140874390:     	incq	(%rax)
140874393:     	addq	$0x38, %rsp
140874397:     	popq	%rbx
140874398:     	popq	%rdi
140874399:     	popq	%rsi
14087439a:     	popq	%r12
14087439c:     	popq	%r13
14087439e:     	popq	%r14
1408743a0:     	popq	%r15
1408743a2:     	popq	%rbp
1408743a3:     	retq
