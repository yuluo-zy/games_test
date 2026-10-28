
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1401584d0: 55                          	pushq	%rbp
1401584d1: 56                          	pushq	%rsi
1401584d2: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1401584d9: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1401584e1: 48 c7 45 10 fe ff ff ff     	movq	$-0x2, 0x10(%rbp)
1401584e9: 48 89 ce                    	movq	%rcx, %rsi
1401584ec: 48 c7 45 f0 00 00 00 00     	movq	$0x0, -0x10(%rbp)
1401584f4: 48 c7 45 f8 01 00 00 00     	movq	$0x1, -0x8(%rbp)
1401584fc: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
140158504: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
14015850c: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
140158514: 48 c7 45 d0 20 00 00 00     	movq	$0x20, -0x30(%rbp)
14015851c: c6 45 d8 03                 	movb	$0x3, -0x28(%rbp)
140158520: 48 8d 45 f0                 	leaq	-0x10(%rbp), %rax
140158524: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140158528: 48 8d 05 e1 33 80 02        	leaq	0x28033e1(%rip), %rax   # 0x14295b910
14015852f: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140158533: 48 8b 02                    	movq	(%rdx), %rax
140158536: 0f b6 00                    	movzbl	(%rax), %eax
140158539: 85 c0                       	testl	%eax, %eax
14015853b: 74 18                       	je	0x140158555 <.text+0x157555>
14015853d: 83 f8 01                    	cmpl	$0x1, %eax
140158540: 75 26                       	jne	0x140158568 <.text+0x157568>
140158542: 48 8d 0d c3 90 84 02        	leaq	0x28490c3(%rip), %rcx   # 0x1429a160c
140158549: ba 1e 00 00 00              	movl	$0x1e, %edx
14015854e: e8 2d 4f a4 00              	callq	0x140b9d480 <.text+0xb9c480>
140158553: eb 24                       	jmp	0x140158579 <.text+0x157579>
140158555: 48 8d 0d 94 90 84 02        	leaq	0x2849094(%rip), %rcx   # 0x1429a15f0
14015855c: ba 1c 00 00 00              	movl	$0x1c, %edx
140158561: e8 1a 4f a4 00              	callq	0x140b9d480 <.text+0xb9c480>
140158566: eb 11                       	jmp	0x140158579 <.text+0x157579>
140158568: 48 8d 0d bb 90 84 02        	leaq	0x28490bb(%rip), %rcx   # 0x1429a162a
14015856f: ba 1e 00 00 00              	movl	$0x1e, %edx
140158574: e8 07 4f a4 00              	callq	0x140b9d480 <.text+0xb9c480>
140158579: 49 89 d0                    	movq	%rdx, %r8
14015857c: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140158580: 48 89 c2                    	movq	%rax, %rdx
140158583: e8 08 e6 c2 00              	callq	0x140d86b90 <.text+0xd85b90>
140158588: 84 c0                       	testb	%al, %al
14015858a: 75 1c                       	jne	0x1401585a8 <.text+0x1575a8>
14015858c: 48 8b 45 00                 	movq	(%rbp), %rax
140158590: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140158594: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
140158598: 0f 11 06                    	movups	%xmm0, (%rsi)
14015859b: 48 89 f0                    	movq	%rsi, %rax
14015859e: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1401585a5: 5e                          	popq	%rsi
1401585a6: 5d                          	popq	%rbp
1401585a7: c3                          	retq
1401585a8: 48 8d 05 c9 33 80 02        	leaq	0x28033c9(%rip), %rax   # 0x14295b978
1401585af: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1401585b4: 48 8d 0d 85 33 80 02        	leaq	0x2803385(%rip), %rcx   # 0x14295b940
1401585bb: 4c 8d 0d ce 33 80 02        	leaq	0x28033ce(%rip), %r9    # 0x14295b990
1401585c2: 4c 8d 45 0f                 	leaq	0xf(%rbp), %r8
1401585c6: ba 37 00 00 00              	movl	$0x37, %edx
1401585cb: e8 90 11 78 02              	callq	0x1428d9760 <.text+0x28d8760>
1401585d0: 0f 0b                       	ud2
1401585d2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1401585e0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1401585e5: 55                          	pushq	%rbp
1401585e6: 56                          	pushq	%rsi
1401585e7: 48 83 ec 28                 	subq	$0x28, %rsp
1401585eb: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1401585f2: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
1401585f6: 48 85 d2                    	testq	%rdx, %rdx
1401585f9: 74 0f                       	je	0x14015860a <.text+0x15760a>
1401585fb: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1401585ff: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140158605: e8 16 b6 4b 00              	callq	0x140613c20 <.text+0x612c20>
14015860a: 90                          	nop
14015860b: 48 83 c4 28                 	addq	$0x28, %rsp
14015860f: 5e                          	popq	%rsi
140158610: 5d                          	popq	%rbp
140158611: c3                          	retq
140158612: cc                          	int3
140158613: cc                          	int3
140158614: cc                          	int3
140158615: cc                          	int3
140158616: cc                          	int3
140158617: cc                          	int3
140158618: cc                          	int3
140158619: cc                          	int3
14015861a: cc                          	int3
14015861b: cc                          	int3
14015861c: cc                          	int3
14015861d: cc                          	int3
14015861e: cc                          	int3
14015861f: cc                          	int3
140158620: 55                          	pushq	%rbp
140158621: 56                          	pushq	%rsi
140158622: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140158629: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140158631: 48 c7 45 10 fe ff ff ff     	movq	$-0x2, 0x10(%rbp)
140158639: 48 89 d0                    	movq	%rdx, %rax
14015863c: 48 89 ce                    	movq	%rcx, %rsi
14015863f: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140158643: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140158647: 48 8b 09                    	movq	(%rcx), %rcx
14015864a: 48 39 d1                    	cmpq	%rdx, %rcx
14015864d: 0f 83 a3 00 00 00           	jae	0x1401586f6 <.text+0x1576f6>
140158653: 48 8b 00                    	movq	(%rax), %rax
140158656: 48 c7 45 f0 00 00 00 00     	movq	$0x0, -0x10(%rbp)
14015865e: 48 c7 45 f8 01 00 00 00     	movq	$0x1, -0x8(%rbp)
140158666: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
14015866e: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140158676: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
14015867e: 48 c7 45 d0 20 00 00 00     	movq	$0x20, -0x30(%rbp)
140158686: c6 45 d8 03                 	movb	$0x3, -0x28(%rbp)
14015868a: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
14015868e: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
140158692: 48 8d 15 77 32 80 02        	leaq	0x2803277(%rip), %rdx   # 0x14295b910
140158699: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
14015869d: f6 04 c8 01                 	testb	$0x1, (%rax,%rcx,8)
1401586a1: 74 22                       	je	0x1401586c5 <.text+0x1576c5>
1401586a3: 48 8d 0d 26 8f 84 02        	leaq	0x2848f26(%rip), %rcx   # 0x1429a15d0
1401586aa: ba 20 00 00 00              	movl	$0x20, %edx
1401586af: e8 cc 4d a4 00              	callq	0x140b9d480 <.text+0xb9c480>
1401586b4: 49 89 d0                    	movq	%rdx, %r8
1401586b7: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
1401586bb: 48 89 c2                    	movq	%rax, %rdx
1401586be: e8 cd e4 c2 00              	callq	0x140d86b90 <.text+0xd85b90>
1401586c3: eb 11                       	jmp	0x1401586d6 <.text+0x1576d6>
1401586c5: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
1401586c9: 48 83 c1 04                 	addq	$0x4, %rcx
1401586cd: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
1401586d1: e8 8a 4e c3 00              	callq	0x140d8d560 <.text+0xd8c560>
1401586d6: 84 c0                       	testb	%al, %al
1401586d8: 75 28                       	jne	0x140158702 <.text+0x157702>
1401586da: 48 8b 45 00                 	movq	(%rbp), %rax
1401586de: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
1401586e2: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
1401586e6: 0f 11 06                    	movups	%xmm0, (%rsi)
1401586e9: 48 89 f0                    	movq	%rsi, %rax
1401586ec: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
1401586f3: 5e                          	popq	%rsi
1401586f4: 5d                          	popq	%rbp
1401586f5: c3                          	retq
1401586f6: 4c 8d 05 13 30 80 02        	leaq	0x2803013(%rip), %r8    # 0x14295b710
1401586fd: e8 16 0e 78 02              	callq	0x1428d9518 <.text+0x28d8518>
140158702: 48 8d 05 6f 32 80 02        	leaq	0x280326f(%rip), %rax   # 0x14295b978
140158709: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14015870e: 48 8d 0d 2b 32 80 02        	leaq	0x280322b(%rip), %rcx   # 0x14295b940
140158715: 4c 8d 0d 74 32 80 02        	leaq	0x2803274(%rip), %r9    # 0x14295b990
14015871c: 4c 8d 45 0f                 	leaq	0xf(%rbp), %r8
140158720: ba 37 00 00 00              	movl	$0x37, %edx
140158725: e8 36 10 78 02              	callq	0x1428d9760 <.text+0x28d8760>
14015872a: 0f 0b                       	ud2
14015872c: 0f 1f 40 00                 	nopl	(%rax)
140158730: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140158735: 55                          	pushq	%rbp
140158736: 56                          	pushq	%rsi
140158737: 48 83 ec 28                 	subq	$0x28, %rsp
14015873b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140158742: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
140158746: 48 85 d2                    	testq	%rdx, %rdx
140158749: 74 0f                       	je	0x14015875a <.text+0x15775a>
14015874b: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
14015874f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140158755: e8 c6 b4 4b 00              	callq	0x140613c20 <.text+0x612c20>
14015875a: 90                          	nop
14015875b: 48 83 c4 28                 	addq	$0x28, %rsp
14015875f: 5e                          	popq	%rsi
140158760: 5d                          	popq	%rbp
140158761: c3                          	retq
140158762: cc                          	int3
140158763: cc                          	int3
140158764: cc                          	int3
140158765: cc                          	int3
140158766: cc                          	int3
140158767: cc                          	int3
140158768: cc                          	int3
140158769: cc                          	int3
14015876a: cc                          	int3
14015876b: cc                          	int3
14015876c: cc                          	int3
14015876d: cc                          	int3
14015876e: cc                          	int3
14015876f: cc                          	int3
140158770: 55                          	pushq	%rbp
140158771: 56                          	pushq	%rsi
140158772: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
140158779: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140158781: 48 c7 45 10 fe ff ff ff     	movq	$-0x2, 0x10(%rbp)
140158789: 48 89 d0                    	movq	%rdx, %rax
14015878c: 48 89 ce                    	movq	%rcx, %rsi
14015878f: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140158793: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140158797: 48 8b 09                    	movq	(%rcx), %rcx
14015879a: 48 39 d1                    	cmpq	%rdx, %rcx
14015879d: 0f 83 9a 00 00 00           	jae	0x14015883d <.text+0x15783d>
1401587a3: 48 8b 00                    	movq	(%rax), %rax
1401587a6: 48 8d 14 c8                 	leaq	(%rax,%rcx,8), %rdx
1401587aa: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
1401587b2: 48 c7 45 00 01 00 00 00     	movq	$0x1, (%rbp)
1401587ba: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
1401587c2: 48 8d 44 c8 04              	leaq	0x4(%rax,%rcx,8), %rax
1401587c7: 48 89 55 d8                 	movq	%rdx, -0x28(%rbp)
1401587cb: 48 8d 0d 8e 4d c3 00        	leaq	0xc34d8e(%rip), %rcx    # 0x140d8d560 <.text+0xd8c560>
1401587d2: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
1401587d6: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
1401587da: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
1401587de: 48 8d 05 a3 8d 84 02        	leaq	0x2848da3(%rip), %rax   # 0x1429a1588
1401587e5: 48 89 45 a8                 	movq	%rax, -0x58(%rbp)
1401587e9: 48 c7 45 b0 02 00 00 00     	movq	$0x2, -0x50(%rbp)
1401587f1: 48 8d 45 d8                 	leaq	-0x28(%rbp), %rax
1401587f5: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
1401587f9: 48 c7 45 c0 02 00 00 00     	movq	$0x2, -0x40(%rbp)
140158801: 48 c7 45 c8 00 00 00 00     	movq	$0x0, -0x38(%rbp)
140158809: 48 8d 15 00 31 80 02        	leaq	0x2803100(%rip), %rdx   # 0x14295b910
140158810: 48 8d 4d f8                 	leaq	-0x8(%rbp), %rcx
140158814: 4c 8d 45 a8                 	leaq	-0x58(%rbp), %r8
140158818: e8 b3 d2 c2 00              	callq	0x140d85ad0 <.text+0xd84ad0>
14015881d: 84 c0                       	testb	%al, %al
14015881f: 75 28                       	jne	0x140158849 <.text+0x157849>
140158821: 48 8b 45 08                 	movq	0x8(%rbp), %rax
140158825: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
140158829: 0f 10 45 f8                 	movups	-0x8(%rbp), %xmm0
14015882d: 0f 11 06                    	movups	%xmm0, (%rsi)
140158830: 48 89 f0                    	movq	%rsi, %rax
140158833: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14015883a: 5e                          	popq	%rsi
14015883b: 5d                          	popq	%rbp
14015883c: c3                          	retq
14015883d: 4c 8d 05 cc 2e 80 02        	leaq	0x2802ecc(%rip), %r8    # 0x14295b710
140158844: e8 cf 0c 78 02              	callq	0x1428d9518 <.text+0x28d8518>
140158849: 48 8d 05 28 31 80 02        	leaq	0x2803128(%rip), %rax   # 0x14295b978
140158850: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140158855: 48 8d 0d e4 30 80 02        	leaq	0x28030e4(%rip), %rcx   # 0x14295b940
14015885c: 4c 8d 0d 2d 31 80 02        	leaq	0x280312d(%rip), %r9    # 0x14295b990
140158863: 4c 8d 45 a8                 	leaq	-0x58(%rbp), %r8
140158867: ba 37 00 00 00              	movl	$0x37, %edx
14015886c: e8 ef 0e 78 02              	callq	0x1428d9760 <.text+0x28d8760>
140158871: 0f 0b                       	ud2
140158873: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140158880: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140158885: 55                          	pushq	%rbp
140158886: 56                          	pushq	%rsi
140158887: 48 83 ec 28                 	subq	$0x28, %rsp
14015888b: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140158892: 48 8b 55 f8                 	movq	-0x8(%rbp), %rdx
140158896: 48 85 d2                    	testq	%rdx, %rdx
140158899: 74 0f                       	je	0x1401588aa <.text+0x1578aa>
14015889b: 48 8b 4d 00                 	movq	(%rbp), %rcx
14015889f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1401588a5: e8 76 b3 4b 00              	callq	0x140613c20 <.text+0x612c20>
1401588aa: 90                          	nop
1401588ab: 48 83 c4 28                 	addq	$0x28, %rsp
1401588af: 5e                          	popq	%rsi
1401588b0: 5d                          	popq	%rbp
1401588b1: c3                          	retq
1401588b2: cc                          	int3
1401588b3: cc                          	int3
1401588b4: cc                          	int3
1401588b5: cc                          	int3
1401588b6: cc                          	int3
1401588b7: cc                          	int3
1401588b8: cc                          	int3
1401588b9: cc                          	int3
1401588ba: cc                          	int3
1401588bb: cc                          	int3
1401588bc: cc                          	int3
1401588bd: cc                          	int3
1401588be: cc                          	int3
1401588bf: cc                          	int3
1401588c0: 55                          	pushq	%rbp
1401588c1: 56                          	pushq	%rsi
1401588c2: 48 81 ec 98 00 00 00        	subq	$0x98, %rsp
1401588c9: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1401588d1: 48 c7 45 10 fe ff ff ff     	movq	$-0x2, 0x10(%rbp)
1401588d9: 48 89 d0                    	movq	%rdx, %rax
1401588dc: 48 89 ce                    	movq	%rcx, %rsi
1401588df: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
1401588e3: 48 8b 48 10                 	movq	0x10(%rax), %rcx
1401588e7: 48 8b 09                    	movq	(%rcx), %rcx
1401588ea: 48 39 d1                    	cmpq	%rdx, %rcx
1401588ed: 73 7f                       	jae	0x14015896e <.text+0x15796e>
1401588ef: 48 8b 00                    	movq	(%rax), %rax
1401588f2: 48 c7 45 f0 00 00 00 00     	movq	$0x0, -0x10(%rbp)
1401588fa: 48 c7 45 f8 01 00 00 00     	movq	$0x1, -0x8(%rbp)
140158902: 48 c7 45 00 00 00 00 00     	movq	$0x0, (%rbp)
14015890a: 48 c7 45 b0 00 00 00 00     	movq	$0x0, -0x50(%rbp)
140158912: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
14015891a: 48 c7 45 d0 20 00 00 00     	movq	$0x20, -0x30(%rbp)
140158922: c6 45 d8 03                 	movb	$0x3, -0x28(%rbp)
140158926: 48 8d 55 f0                 	leaq	-0x10(%rbp), %rdx
14015892a: 48 89 55 e0                 	movq	%rdx, -0x20(%rbp)
14015892e: 48 8d 15 db 2f 80 02        	leaq	0x2802fdb(%rip), %rdx   # 0x14295b910
140158935: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
140158939: 48 8b 04 c8                 	movq	(%rax,%rcx,8), %rax
14015893d: 48 8b 48 08                 	movq	0x8(%rax), %rcx
140158941: 48 8b 50 10                 	movq	0x10(%rax), %rdx
140158945: 4c 8d 45 b0                 	leaq	-0x50(%rbp), %r8
140158949: e8 a2 fd c2 00              	callq	0x140d886f0 <.text+0xd876f0>
14015894e: 84 c0                       	testb	%al, %al
140158950: 75 28                       	jne	0x14015897a <.text+0x15797a>
140158952: 48 8b 45 00                 	movq	(%rbp), %rax
140158956: 48 89 46 10                 	movq	%rax, 0x10(%rsi)
14015895a: 0f 10 45 f0                 	movups	-0x10(%rbp), %xmm0
14015895e: 0f 11 06                    	movups	%xmm0, (%rsi)
140158961: 48 89 f0                    	movq	%rsi, %rax
140158964: 48 81 c4 98 00 00 00        	addq	$0x98, %rsp
14015896b: 5e                          	popq	%rsi
14015896c: 5d                          	popq	%rbp
14015896d: c3                          	retq
14015896e: 4c 8d 05 9b 2d 80 02        	leaq	0x2802d9b(%rip), %r8    # 0x14295b710
140158975: e8 9e 0b 78 02              	callq	0x1428d9518 <.text+0x28d8518>
14015897a: 48 8d 05 f7 2f 80 02        	leaq	0x2802ff7(%rip), %rax   # 0x14295b978
140158981: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140158986: 48 8d 0d b3 2f 80 02        	leaq	0x2802fb3(%rip), %rcx   # 0x14295b940
14015898d: 4c 8d 0d fc 2f 80 02        	leaq	0x2802ffc(%rip), %r9    # 0x14295b990
140158994: 4c 8d 45 0f                 	leaq	0xf(%rbp), %r8
140158998: ba 37 00 00 00              	movl	$0x37, %edx
14015899d: e8 be 0d 78 02              	callq	0x1428d9760 <.text+0x28d8760>
1401589a2: 0f 0b                       	ud2
1401589a4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1401589b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1401589b5: 55                          	pushq	%rbp
1401589b6: 56                          	pushq	%rsi
1401589b7: 48 83 ec 28                 	subq	$0x28, %rsp
1401589bb: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1401589c2: 48 8b 55 f0                 	movq	-0x10(%rbp), %rdx
1401589c6: 48 85 d2                    	testq	%rdx, %rdx
1401589c9: 74 0f                       	je	0x1401589da <.text+0x1579da>
1401589cb: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
1401589cf: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1401589d5: e8 46 b2 4b 00              	callq	0x140613c20 <.text+0x612c20>
1401589da: 90                          	nop
1401589db: 48 83 c4 28                 	addq	$0x28, %rsp
1401589df: 5e                          	popq	%rsi
1401589e0: 5d                          	popq	%rbp
1401589e1: c3                          	retq
1401589e2: cc                          	int3
1401589e3: cc                          	int3
1401589e4: cc                          	int3
1401589e5: cc                          	int3
1401589e6: cc                          	int3
1401589e7: cc                          	int3
1401589e8: cc                          	int3
1401589e9: cc                          	int3
1401589ea: cc                          	int3
1401589eb: cc                          	int3
1401589ec: cc                          	int3
1401589ed: cc                          	int3
1401589ee: cc                          	int3
1401589ef: cc                          	int3
1401589f0: 55                          	pushq	%rbp
1401589f1: 56                          	pushq	%rsi
1401589f2: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1401589f9: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140158a01: 48 c7 45 40 fe ff ff ff     	movq	$-0x2, 0x40(%rbp)
140158a09: 48 89 d0                    	movq	%rdx, %rax
140158a0c: 48 89 ce                    	movq	%rcx, %rsi
140158a0f: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
140158a13: 48 8b 48 10                 	movq	0x10(%rax), %rcx
140158a17: 48 8b 09                    	movq	(%rcx), %rcx
140158a1a: 48 39 d1                    	cmpq	%rdx, %rcx
140158a1d: 0f 83 de 00 00 00           	jae	0x140158b01 <.text+0x157b01>
140158a23: 48 8b 00                    	movq	(%rax), %rax
140158a26: 48 c7 45 20 00 00 00 00     	movq	$0x0, 0x20(%rbp)
140158a2e: 48 c7 45 28 01 00 00 00     	movq	$0x1, 0x28(%rbp)
140158a36: 48 c7 45 30 00 00 00 00     	movq	$0x0, 0x30(%rbp)
140158a3e: 8b 04 88                    	movl	(%rax,%rcx,4), %eax
140158a41: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
140158a46: f3 0f 5e 05 e2 2b 80 02     	divss	0x2802be2(%rip), %xmm0  # 0x14295b630
140158a4e: f3 0f 11 45 3c              	movss	%xmm0, 0x3c(%rbp)
140158a53: 48 8d 45 3c                 	leaq	0x3c(%rbp), %rax
140158a57: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140158a5b: 48 8d 05 ae 3e c3 00        	leaq	0xc33eae(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
140158a62: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140158a66: 48 c7 45 d8 00 00 00 00     	movq	$0x0, -0x28(%rbp)
140158a6e: 48 c7 45 e0 02 00 00 00     	movq	$0x2, -0x20(%rbp)
140158a76: 48 c7 45 e8 00 00 00 00     	movq	$0x0, -0x18(%rbp)
140158a7e: 48 c7 45 f0 02 00 00 00     	movq	$0x2, -0x10(%rbp)
140158a86: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
140158a8e: 48 c7 45 00 20 00 00 00     	movq	$0x20, (%rbp)
