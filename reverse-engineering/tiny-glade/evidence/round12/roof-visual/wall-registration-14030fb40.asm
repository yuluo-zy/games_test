
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14030fb40:     	pushq	%rbp
14030fb41:     	pushq	%rsi
14030fb42:     	pushq	%rdi
14030fb43:     	pushq	%rbx
14030fb44:     	subq	$0x4e8, %rsp            # imm = 0x4E8
14030fb4b:     	leaq	0x80(%rsp), %rbp
14030fb53:     	movq	$-0x2, 0x460(%rbp)
14030fb5e:     	movq	%rcx, %rsi
14030fb61:     	movzbl	0x33250a1(%rip), %eax   # 0x143634c09
14030fb68:     	movl	$0x230, %ecx            # imm = 0x230
14030fb6d:     	movl	$0x8, %edx
14030fb72:     	callq	0x140613c10 <.text+0x612c10>
14030fb77:     	testq	%rax, %rax
14030fb7a:     	je	0x140310297 <.text+0x30f297>
14030fb80:     	movq	%rax, 0x458(%rbp)
14030fb87:     	xorps	%xmm0, %xmm0
14030fb8a:     	movups	%xmm0, 0x188(%rbp)
14030fb91:     	movups	%xmm0, 0x1a0(%rbp)
14030fb98:     	movups	%xmm0, 0x1b8(%rbp)
14030fb9f:     	movups	%xmm0, 0x1d0(%rbp)
14030fba6:     	movups	%xmm0, 0x1e8(%rbp)
14030fbad:     	movl	$0x0, 0x1f8(%rbp)
14030fbb7:     	movups	%xmm0, 0x220(%rbp)
14030fbbe:     	movups	%xmm0, 0x238(%rbp)
14030fbc5:     	movups	%xmm0, 0x250(%rbp)
14030fbcc:     	movups	%xmm0, 0x268(%rbp)
14030fbd3:     	movups	%xmm0, 0x280(%rbp)
14030fbda:     	movl	$0x0, 0x290(%rbp)
14030fbe4:     	movq	$0x0, 0x168(%rbp)
14030fbef:     	movq	$0x8, 0x170(%rbp)
14030fbfa:     	movq	$0x0, 0x178(%rbp)
14030fc05:     	movq	$0x10, 0x180(%rbp)
14030fc10:     	movq	$0x10, 0x198(%rbp)
14030fc1b:     	movq	$0x10, 0x1b0(%rbp)
14030fc26:     	movq	$0x10, 0x1c8(%rbp)
14030fc31:     	movq	$0x10, 0x1e0(%rbp)
14030fc3c:     	movabsq	$-0x8000000000000000, %rbx # imm = 0x8000000000000000
14030fc46:     	movq	%rbx, 0x200(%rbp)
14030fc4d:     	leaq	0x26aa001(%rip), %rax   # 0x1429b9c55
14030fc54:     	movq	%rax, 0x208(%rbp)
14030fc5b:     	movq	$0x57, 0x210(%rbp)
14030fc66:     	movq	$0x10, 0x218(%rbp)
14030fc71:     	movq	$0x10, 0x230(%rbp)
14030fc7c:     	movq	$0x10, 0x248(%rbp)
14030fc87:     	movq	$0x10, 0x260(%rbp)
14030fc92:     	movq	$0x10, 0x278(%rbp)
14030fc9d:     	movl	$0x0, 0x298(%rbp)
14030fca7:     	movw	$0x1, 0x29c(%rbp)
14030fcb0:     	movq	%rbx, 0x398(%rbp)
14030fcb7:     	movl	$0x0, 0x450(%rbp)
14030fcc1:     	movzbl	0x3324f41(%rip), %eax   # 0x143634c09
14030fcc8:     	movl	$0x2f0, %ecx            # imm = 0x2F0
14030fccd:     	movl	$0x8, %edx
14030fcd2:     	callq	0x140613c10 <.text+0x612c10>
14030fcd7:     	testq	%rax, %rax
14030fcda:     	je	0x1403102a6 <.text+0x30f2a6>
14030fce0:     	movq	%rax, %rdi
14030fce3:     	leaq	0x168(%rbp), %rdx
14030fcea:     	movl	$0x2f0, %r8d            # imm = 0x2F0
14030fcf0:     	movq	%rax, %rcx
14030fcf3:     	callq	0x142923c00 <.text+0x2922c00>
14030fcf8:     	leaq	0x26a9bb1(%rip), %r8    # 0x1429b98b0
14030fcff:     	leaq	0x88(%rbp), %rcx
14030fd06:     	movq	%rdi, %rdx
14030fd09:     	callq	0x140ce4b10 <.text+0xce3b10>
14030fd0e:     	xorps	%xmm0, %xmm0
14030fd11:     	movups	%xmm0, 0x1f0(%rbp)
14030fd18:     	movups	%xmm0, 0x208(%rbp)
14030fd1f:     	movups	%xmm0, 0x220(%rbp)
14030fd26:     	movups	%xmm0, 0x238(%rbp)
14030fd2d:     	movups	%xmm0, 0x250(%rbp)
14030fd34:     	movl	$0x0, 0x260(%rbp)
14030fd3e:     	movups	%xmm0, 0x288(%rbp)
14030fd45:     	movups	%xmm0, 0x2a0(%rbp)
14030fd4c:     	movups	%xmm0, 0x2b8(%rbp)
14030fd53:     	movups	%xmm0, 0x2d0(%rbp)
14030fd5a:     	movups	%xmm0, 0x2e8(%rbp)
14030fd61:     	movl	$0x0, 0x2f8(%rbp)
14030fd6b:     	movq	$0x0, 0x168(%rbp)
14030fd76:     	movq	$0x0, 0x1d0(%rbp)
14030fd81:     	movq	$0x8, 0x1d8(%rbp)
14030fd8c:     	movq	$0x0, 0x1e0(%rbp)
14030fd97:     	movq	$0x10, 0x1e8(%rbp)
14030fda2:     	movq	$0x10, 0x200(%rbp)
14030fdad:     	movq	$0x10, 0x218(%rbp)
14030fdb8:     	movq	$0x10, 0x230(%rbp)
14030fdc3:     	movq	$0x10, 0x248(%rbp)
14030fdce:     	movq	%rbx, 0x268(%rbp)
14030fdd5:     	leaq	0x26a9ed0(%rip), %rax   # 0x1429b9cac
14030fddc:     	movq	%rax, 0x270(%rbp)
14030fde3:     	movq	$0x50, 0x278(%rbp)
14030fdee:     	movq	$0x10, 0x280(%rbp)
14030fdf9:     	movq	$0x10, 0x298(%rbp)
14030fe04:     	movq	$0x10, 0x2b0(%rbp)
14030fe0f:     	movq	$0x10, 0x2c8(%rbp)
14030fe1a:     	movq	$0x10, 0x2e0(%rbp)
14030fe25:     	movl	$0x0, 0x300(%rbp)
14030fe2f:     	movw	$0x1, 0x304(%rbp)
14030fe38:     	movl	$0x0, 0x308(%rbp)
14030fe42:     	movzbl	0x3324dc0(%rip), %eax   # 0x143634c09
14030fe49:     	movl	$0x1a8, %ecx            # imm = 0x1A8
14030fe4e:     	movl	$0x8, %edx
14030fe53:     	callq	0x140613c10 <.text+0x612c10>
14030fe58:     	testq	%rax, %rax
14030fe5b:     	je	0x1403102b7 <.text+0x30f2b7>
14030fe61:     	movq	%rax, %rdi
14030fe64:     	leaq	0x168(%rbp), %rdx
14030fe6b:     	movl	$0x1a8, %r8d            # imm = 0x1A8
14030fe71:     	movq	%rax, %rcx
14030fe74:     	callq	0x142923c00 <.text+0x2922c00>
14030fe79:     	leaq	0x26a9978(%rip), %r8    # 0x1429b97f8
14030fe80:     	leaq	0x18(%rbp), %rcx
14030fe84:     	movq	%rdi, %rdx
14030fe87:     	callq	0x140ce4b10 <.text+0xce3b10>
14030fe8c:     	xorps	%xmm0, %xmm0
14030fe8f:     	movups	%xmm0, 0x1e8(%rbp)
14030fe96:     	movups	%xmm0, 0x200(%rbp)
14030fe9d:     	movups	%xmm0, 0x218(%rbp)
14030fea4:     	movups	%xmm0, 0x230(%rbp)
14030feab:     	movups	%xmm0, 0x248(%rbp)
14030feb2:     	movl	$0x0, 0x258(%rbp)
14030febc:     	movups	%xmm0, 0x280(%rbp)
14030fec3:     	movups	%xmm0, 0x298(%rbp)
14030feca:     	movups	%xmm0, 0x2b0(%rbp)
14030fed1:     	movups	%xmm0, 0x2c8(%rbp)
14030fed8:     	movups	%xmm0, 0x2e0(%rbp)
14030fedf:     	movl	$0x0, 0x2f0(%rbp)
14030fee9:     	movq	$0x0, 0x168(%rbp)
14030fef4:     	movq	$0x0, 0x1c8(%rbp)
14030feff:     	movq	$0x8, 0x1d0(%rbp)
14030ff0a:     	movq	$0x0, 0x1d8(%rbp)
14030ff15:     	movq	$0x10, 0x1e0(%rbp)
14030ff20:     	movq	$0x10, 0x1f8(%rbp)
14030ff2b:     	movq	$0x10, 0x210(%rbp)
14030ff36:     	movq	$0x10, 0x228(%rbp)
14030ff41:     	movq	$0x10, 0x240(%rbp)
14030ff4c:     	movq	%rbx, 0x260(%rbp)
14030ff53:     	leaq	0x26a9da2(%rip), %rax   # 0x1429b9cfc
14030ff5a:     	movq	%rax, 0x268(%rbp)
14030ff61:     	movq	$0x52, 0x270(%rbp)
14030ff6c:     	movq	$0x10, 0x278(%rbp)
14030ff77:     	movq	$0x10, 0x290(%rbp)
14030ff82:     	movq	$0x10, 0x2a8(%rbp)
14030ff8d:     	movq	$0x10, 0x2c0(%rbp)
14030ff98:     	movq	$0x10, 0x2d8(%rbp)
14030ffa3:     	movl	$0x0, 0x2f8(%rbp)
14030ffad:     	movw	$0x1, 0x2fc(%rbp)
14030ffb6:     	movl	$0x0, 0x300(%rbp)
14030ffc0:     	movzbl	0x3324c42(%rip), %eax   # 0x143634c09
14030ffc7:     	movl	$0x1a0, %ecx            # imm = 0x1A0
14030ffcc:     	movl	$0x8, %edx
14030ffd1:     	callq	0x140613c10 <.text+0x612c10>
14030ffd6:     	testq	%rax, %rax
14030ffd9:     	je	0x1403102c8 <.text+0x30f2c8>
14030ffdf:     	movq	%rax, %rdi
14030ffe2:     	leaq	0x168(%rbp), %rdx
14030ffe9:     	movl	$0x1a0, %r8d            # imm = 0x1A0
14030ffef:     	movq	%rax, %rcx
14030fff2:     	callq	0x142923c00 <.text+0x2922c00>
14030fff7:     	leaq	0x26a9ada(%rip), %r8    # 0x1429b9ad8
14030fffe:     	leaq	-0x58(%rbp), %rcx
140310002:     	movq	%rdi, %rdx
140310005:     	callq	0x140ce4b10 <.text+0xce3b10>
14031000a:     	leaq	0xf8(%rbp), %rcx
140310011:     	callq	0x14033c140 <.text+0x33b140>
140310016:     	leaq	0x168(%rbp), %rcx
14031001d:     	callq	0x140254d90 <.text+0x253d90>
140310022:     	movups	0xe8(%rbp), %xmm0
140310029:     	movq	0x458(%rbp), %rax
140310030:     	movups	%xmm0, 0x60(%rax)
140310034:     	movups	0xd8(%rbp), %xmm0
14031003b:     	movups	%xmm0, 0x50(%rax)
14031003f:     	movups	0xc8(%rbp), %xmm0
140310046:     	movups	%xmm0, 0x40(%rax)
14031004a:     	movups	0x88(%rbp), %xmm0
140310051:     	movups	0x98(%rbp), %xmm1
140310058:     	movups	0xa8(%rbp), %xmm2
14031005f:     	movups	0xb8(%rbp), %xmm3
140310066:     	movups	%xmm3, 0x30(%rax)
14031006a:     	movups	%xmm2, 0x20(%rax)
14031006e:     	movups	%xmm1, 0x10(%rax)
140310072:     	movups	%xmm0, (%rax)
140310075:     	movups	0x18(%rbp), %xmm0
140310079:     	movups	0x28(%rbp), %xmm1
14031007d:     	movups	0x38(%rbp), %xmm2
140310081:     	movups	0x48(%rbp), %xmm3
140310085:     	movups	%xmm0, 0x70(%rax)
140310089:     	movups	%xmm1, 0x80(%rax)
140310090:     	movups	%xmm2, 0x90(%rax)
140310097:     	movups	%xmm3, 0xa0(%rax)
14031009e:     	movups	0x58(%rbp), %xmm0
1403100a2:     	movups	%xmm0, 0xb0(%rax)
1403100a9:     	movups	0x68(%rbp), %xmm0
1403100ad:     	movups	%xmm0, 0xc0(%rax)
1403100b4:     	movups	0x78(%rbp), %xmm0
1403100b8:     	movups	%xmm0, 0xd0(%rax)
1403100bf:     	movups	0x8(%rbp), %xmm0
1403100c3:     	movups	%xmm0, 0x140(%rax)
1403100ca:     	movups	-0x8(%rbp), %xmm0
1403100ce:     	movups	%xmm0, 0x130(%rax)
1403100d5:     	movups	-0x18(%rbp), %xmm0
1403100d9:     	movups	%xmm0, 0x120(%rax)
1403100e0:     	movups	-0x58(%rbp), %xmm0
1403100e4:     	movups	-0x48(%rbp), %xmm1
1403100e8:     	movups	-0x38(%rbp), %xmm2
1403100ec:     	movups	-0x28(%rbp), %xmm3
1403100f0:     	movups	%xmm3, 0x110(%rax)
1403100f7:     	movups	%xmm2, 0x100(%rax)
1403100fe:     	movups	%xmm1, 0xf0(%rax)
140310105:     	movups	%xmm0, 0xe0(%rax)
14031010c:     	movups	0xf8(%rbp), %xmm0
140310113:     	movups	0x108(%rbp), %xmm1
14031011a:     	movups	0x118(%rbp), %xmm2
140310121:     	movups	0x128(%rbp), %xmm3
140310128:     	movups	%xmm0, 0x150(%rax)
14031012f:     	movups	%xmm1, 0x160(%rax)
140310136:     	movups	%xmm2, 0x170(%rax)
14031013d:     	movups	%xmm3, 0x180(%rax)
140310144:     	movups	0x138(%rbp), %xmm0
14031014b:     	movups	%xmm0, 0x190(%rax)
140310152:     	movups	0x148(%rbp), %xmm0
140310159:     	movups	%xmm0, 0x1a0(%rax)
140310160:     	movups	0x158(%rbp), %xmm0
140310167:     	movups	%xmm0, 0x1b0(%rax)
14031016e:     	movups	0x168(%rbp), %xmm0
140310175:     	movups	0x178(%rbp), %xmm1
14031017c:     	movups	0x188(%rbp), %xmm2
140310183:     	movups	0x198(%rbp), %xmm3
14031018a:     	movups	%xmm0, 0x1c0(%rax)
140310191:     	movups	%xmm1, 0x1d0(%rax)
140310198:     	movups	%xmm2, 0x1e0(%rax)
14031019f:     	movups	%xmm3, 0x1f0(%rax)
1403101a6:     	movups	0x1a8(%rbp), %xmm0
1403101ad:     	movups	%xmm0, 0x200(%rax)
1403101b4:     	movups	0x1c8(%rbp), %xmm0
1403101bb:     	movups	%xmm0, 0x220(%rax)
1403101c2:     	movups	0x1b8(%rbp), %xmm0
1403101c9:     	movups	%xmm0, 0x210(%rax)
1403101d0:     	movq	$0x5, 0x170(%rbp)
1403101db:     	movq	%rax, 0x178(%rbp)
1403101e2:     	movq	$0x5, 0x180(%rbp)
1403101ed:     	movq	$0x0, 0x188(%rbp)
1403101f8:     	movq	$0x8, 0x190(%rbp)
140310203:     	xorps	%xmm0, %xmm0
140310206:     	movups	%xmm0, 0x198(%rbp)
14031020d:     	movq	0x108(%rbp), %rax
140310214:     	movq	%rax, 0x1b8(%rbp)
14031021b:     	movups	0xf8(%rbp), %xmm0
140310222:     	movups	%xmm0, 0x1a8(%rbp)
140310229:     	movq	%rbx, 0x168(%rbp)
140310230:     	leaq	0x1a0(%rbp), %rcx
140310237:     	callq	0x1401705d0 <.text+0x16f5d0>
14031023c:     	movups	0x1c8(%rbp), %xmm0
140310243:     	movups	%xmm0, 0x60(%rsi)
140310247:     	movups	0x1b8(%rbp), %xmm0
14031024e:     	movups	%xmm0, 0x50(%rsi)
140310252:     	movups	0x1a8(%rbp), %xmm0
140310259:     	movups	%xmm0, 0x40(%rsi)
14031025d:     	movups	0x168(%rbp), %xmm0
140310264:     	movups	0x178(%rbp), %xmm1
14031026b:     	movups	0x188(%rbp), %xmm2
140310272:     	movups	0x198(%rbp), %xmm3
140310279:     	movups	%xmm3, 0x30(%rsi)
14031027d:     	movups	%xmm2, 0x20(%rsi)
140310281:     	movups	%xmm1, 0x10(%rsi)
140310285:     	movups	%xmm0, (%rsi)
140310288:     	movq	%rsi, %rax
14031028b:     	addq	$0x4e8, %rsp            # imm = 0x4E8
140310292:     	popq	%rbx
140310293:     	popq	%rdi
140310294:     	popq	%rsi
140310295:     	popq	%rbp
140310296:     	retq
140310297:     	movl	$0x8, %ecx
14031029c:     	movl	$0x230, %edx            # imm = 0x230
1403102a1:     	callq	0x1428d9000 <.text+0x28d8000>
1403102a6:     	movl	$0x8, %ecx
1403102ab:     	movl	$0x2f0, %edx            # imm = 0x2F0
1403102b0:     	callq	0x1428d9000 <.text+0x28d8000>
1403102b5:     	jmp	0x1403102d7 <.text+0x30f2d7>
1403102b7:     	movl	$0x8, %ecx
1403102bc:     	movl	$0x1a8, %edx            # imm = 0x1A8
1403102c1:     	callq	0x1428d9000 <.text+0x28d8000>
1403102c6:     	jmp	0x1403102d7 <.text+0x30f2d7>
1403102c8:     	movl	$0x8, %ecx
1403102cd:     	movl	$0x1a0, %edx            # imm = 0x1A0
1403102d2:     	callq	0x1428d9000 <.text+0x28d8000>
1403102d7:     	ud2
1403102d9:     	nopl	(%rax)
1403102e0:     	movq	%rdx, 0x10(%rsp)
1403102e5:     	pushq	%rbp
1403102e6:     	pushq	%rsi
1403102e7:     	pushq	%rdi
1403102e8:     	pushq	%rbx
1403102e9:     	subq	$0x28, %rsp
1403102ed:     	leaq	0x80(%rdx), %rbp
1403102f4:     	leaq	0x168(%rbp), %rcx
1403102fb:     	callq	0x1405cf3b0 <.text+0x5ce3b0>
140310300:     	nop
140310301:     	addq	$0x28, %rsp
140310305:     	popq	%rbx
140310306:     	popq	%rdi
140310307:     	popq	%rsi
140310308:     	popq	%rbp
140310309:     	retq
14031030a:     	nopw	(%rax,%rax)
140310310:     	movq	%rdx, 0x10(%rsp)
140310315:     	pushq	%rbp
140310316:     	pushq	%rsi
140310317:     	pushq	%rdi
140310318:     	pushq	%rbx
140310319:     	subq	$0x28, %rsp
14031031d:     	leaq	0x80(%rdx), %rbp
140310324:     	movl	$0x230, %edx            # imm = 0x230
140310329:     	movl	$0x8, %r8d
14031032f:     	movq	0x458(%rbp), %rcx
140310336:     	callq	0x140613c20 <.text+0x612c20>
14031033b:     	nop
14031033c:     	addq	$0x28, %rsp
140310340:     	popq	%rbx
140310341:     	popq	%rdi
140310342:     	popq	%rsi
140310343:     	popq	%rbp
140310344:     	retq
140310345:     	nopw	%cs:(%rax,%rax)
140310350:     	movq	%rdx, 0x10(%rsp)
140310355:     	pushq	%rbp
140310356:     	pushq	%rsi
140310357:     	pushq	%rdi
140310358:     	pushq	%rbx
140310359:     	subq	$0x28, %rsp
14031035d:     	leaq	0x80(%rdx), %rbp
140310364:     	leaq	0x168(%rbp), %rcx
14031036b:     	callq	0x1405dc7b0 <.text+0x5db7b0>
140310370:     	nop
140310371:     	addq	$0x28, %rsp
140310375:     	popq	%rbx
140310376:     	popq	%rdi
140310377:     	popq	%rsi
140310378:     	popq	%rbp
140310379:     	retq
14031037a:     	nopw	(%rax,%rax)
140310380:     	movq	%rdx, 0x10(%rsp)
140310385:     	pushq	%rbp
140310386:     	pushq	%rsi
140310387:     	pushq	%rdi
140310388:     	pushq	%rbx
140310389:     	subq	$0x28, %rsp
14031038d:     	leaq	0x80(%rdx), %rbp
140310394:     	leaq	0x88(%rbp), %rcx
14031039b:     	callq	0x140029880 <.text+0x28880>
1403103a0:     	nop
1403103a1:     	addq	$0x28, %rsp
1403103a5:     	popq	%rbx
1403103a6:     	popq	%rdi
1403103a7:     	popq	%rsi
1403103a8:     	popq	%rbp
1403103a9:     	retq
1403103aa:     	nopw	(%rax,%rax)
1403103b0:     	movq	%rdx, 0x10(%rsp)
1403103b5:     	pushq	%rbp
1403103b6:     	pushq	%rsi
1403103b7:     	pushq	%rdi
1403103b8:     	pushq	%rbx
1403103b9:     	subq	$0x28, %rsp
1403103bd:     	leaq	0x80(%rdx), %rbp
1403103c4:     	leaq	0x168(%rbp), %rcx
1403103cb:     	callq	0x1405d2c30 <.text+0x5d1c30>
1403103d0:     	nop
1403103d1:     	addq	$0x28, %rsp
1403103d5:     	popq	%rbx
1403103d6:     	popq	%rdi
1403103d7:     	popq	%rsi
1403103d8:     	popq	%rbp
1403103d9:     	retq
1403103da:     	nopw	(%rax,%rax)
1403103e0:     	movq	%rdx, 0x10(%rsp)
1403103e5:     	pushq	%rbp
1403103e6:     	pushq	%rsi
1403103e7:     	pushq	%rdi
1403103e8:     	pushq	%rbx
1403103e9:     	subq	$0x28, %rsp
1403103ed:     	leaq	0x80(%rdx), %rbp
1403103f4:     	leaq	0x18(%rbp), %rcx
1403103f8:     	callq	0x140029880 <.text+0x28880>
1403103fd:     	nop
1403103fe:     	addq	$0x28, %rsp
140310402:     	popq	%rbx
140310403:     	popq	%rdi
140310404:     	popq	%rsi
140310405:     	popq	%rbp
140310406:     	retq
140310407:     	nopw	(%rax,%rax)
140310410:     	movq	%rdx, 0x10(%rsp)
140310415:     	pushq	%rbp
140310416:     	pushq	%rsi
140310417:     	pushq	%rdi
140310418:     	pushq	%rbx
140310419:     	subq	$0x28, %rsp
14031041d:     	leaq	0x80(%rdx), %rbp
140310424:     	leaq	-0x58(%rbp), %rcx
140310428:     	callq	0x140029880 <.text+0x28880>
14031042d:     	nop
14031042e:     	addq	$0x28, %rsp
140310432:     	popq	%rbx
140310433:     	popq	%rdi
140310434:     	popq	%rsi
140310435:     	popq	%rbp
140310436:     	retq
140310437:     	nopw	(%rax,%rax)
140310440:     	movq	%rdx, 0x10(%rsp)
140310445:     	pushq	%rbp
140310446:     	pushq	%rsi
140310447:     	pushq	%rdi
140310448:     	pushq	%rbx
140310449:     	subq	$0x28, %rsp
14031044d:     	leaq	0x80(%rdx), %rbp
140310454:     	leaq	0xf8(%rbp), %rcx
14031045b:     	callq	0x140029880 <.text+0x28880>
140310460:     	nop
140310461:     	addq	$0x28, %rsp
140310465:     	popq	%rbx
140310466:     	popq	%rdi
140310467:     	popq	%rsi
140310468:     	popq	%rbp
140310469:     	retq
14031046a:     	nopw	(%rax,%rax)
140310470:     	movq	%rdx, 0x10(%rsp)
140310475:     	pushq	%rbp
140310476:     	pushq	%rsi
140310477:     	pushq	%rdi
140310478:     	pushq	%rbx
140310479:     	subq	$0x28, %rsp
14031047d:     	leaq	0x80(%rdx), %rbp
140310484:     	leaq	0x168(%rbp), %rcx
14031048b:     	callq	0x140029880 <.text+0x28880>
140310490:     	nop
140310491:     	addq	$0x28, %rsp
140310495:     	popq	%rbx
140310496:     	popq	%rdi
140310497:     	popq	%rsi
140310498:     	popq	%rbp
140310499:     	retq
