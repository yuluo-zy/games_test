
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a92f80: 55                          	pushq	%rbp
140a92f81: 41 57                       	pushq	%r15
140a92f83: 41 56                       	pushq	%r14
140a92f85: 41 55                       	pushq	%r13
140a92f87: 41 54                       	pushq	%r12
140a92f89: 56                          	pushq	%rsi
140a92f8a: 57                          	pushq	%rdi
140a92f8b: 53                          	pushq	%rbx
140a92f8c: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140a92f93: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a92f9b: 0f 29 7d 70                 	movaps	%xmm7, 0x70(%rbp)
140a92f9f: 0f 29 75 60                 	movaps	%xmm6, 0x60(%rbp)
140a92fa3: 48 c7 45 58 fe ff ff ff     	movq	$-0x2, 0x58(%rbp)
140a92fab: 4c 89 c7                    	movq	%r8, %rdi
140a92fae: 49 89 d6                    	movq	%rdx, %r14
140a92fb1: 49 89 cf                    	movq	%rcx, %r15
140a92fb4: e8 47 3a 18 00              	callq	0x140c16a00 <.text+0xc15a00>
140a92fb9: 88 45 57                    	movb	%al, 0x57(%rbp)
140a92fbc: 84 c0                       	testb	%al, %al
140a92fbe: 0f 84 1b 02 00 00           	je	0x140a931df <.text+0xa921df>
140a92fc4: 4c 8d 25 30 70 04 02        	leaq	0x2047030(%rip), %r12   # 0x142ad9ffb
140a92fcb: be 43 00 00 00              	movl	$0x43, %esi
140a92fd0: ba 43 00 00 00              	movl	$0x43, %edx
140a92fd5: 4c 89 e1                    	movq	%r12, %rcx
140a92fd8: e8 e3 e9 ff ff              	callq	0x140a919c0 <.text+0xa909c0>
140a92fdd: 48 83 f8 01                 	cmpq	$0x1, %rax
140a92fe1: 0f 85 aa 00 00 00           	jne	0x140a93091 <.text+0xa92091>
140a92fe7: 49 89 d1                    	movq	%rdx, %r9
140a92fea: 48 85 d2                    	testq	%rdx, %rdx
140a92fed: 74 36                       	je	0x140a93025 <.text+0xa92025>
140a92fef: 49 83 f9 43                 	cmpq	$0x43, %r9
140a92ff3: 73 2e                       	jae	0x140a93023 <.text+0xa92023>
140a92ff5: 48 8d 05 ff 6f 04 02        	leaq	0x2046fff(%rip), %rax   # 0x142ad9ffb
140a92ffc: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
140a93001: 7f 22                       	jg	0x140a93025 <.text+0xa92025>
140a93003: 48 8d 05 0e 71 04 02        	leaq	0x204710e(%rip), %rax   # 0x142ada118
140a9300a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a9300f: 48 8d 0d e5 6f 04 02        	leaq	0x2046fe5(%rip), %rcx   # 0x142ad9ffb
140a93016: ba 43 00 00 00              	movl	$0x43, %edx
140a9301b: 45 31 c0                    	xorl	%r8d, %r8d
140a9301e: e8 1d 69 e4 01              	callq	0x1428d9940 <.text+0x28d8940>
140a93023: 75 de                       	jne	0x140a93003 <.text+0xa92003>
140a93025: 4c 8d 25 cf 6f 04 02        	leaq	0x2046fcf(%rip), %r12   # 0x142ad9ffb
140a9302c: 4c 89 e1                    	movq	%r12, %rcx
140a9302f: 4c 89 ca                    	movq	%r9, %rdx
140a93032: e8 89 e9 ff ff              	callq	0x140a919c0 <.text+0xa909c0>
140a93037: 48 83 f8 01                 	cmpq	$0x1, %rax
140a9303b: 75 54                       	jne	0x140a93091 <.text+0xa92091>
140a9303d: 49 89 d0                    	movq	%rdx, %r8
140a93040: 49 83 c0 02                 	addq	$0x2, %r8
140a93044: 74 39                       	je	0x140a9307f <.text+0xa9207f>
140a93046: 49 83 f8 43                 	cmpq	$0x43, %r8
140a9304a: 73 31                       	jae	0x140a9307d <.text+0xa9207d>
140a9304c: 48 8d 05 a8 6f 04 02        	leaq	0x2046fa8(%rip), %rax   # 0x142ad9ffb
140a93053: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
140a93058: 7f 25                       	jg	0x140a9307f <.text+0xa9207f>
140a9305a: 48 8d 05 cf 70 04 02        	leaq	0x20470cf(%rip), %rax   # 0x142ada130
140a93061: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a93066: 48 8d 0d 8e 6f 04 02        	leaq	0x2046f8e(%rip), %rcx   # 0x142ad9ffb
140a9306d: ba 43 00 00 00              	movl	$0x43, %edx
140a93072: 41 b9 43 00 00 00           	movl	$0x43, %r9d
140a93078: e8 c3 68 e4 01              	callq	0x1428d9940 <.text+0x28d8940>
140a9307d: 75 db                       	jne	0x140a9305a <.text+0xa9205a>
140a9307f: be 41 00 00 00              	movl	$0x41, %esi
140a93084: 48 29 d6                    	subq	%rdx, %rsi
140a93087: 4c 8d 25 6d 6f 04 02        	leaq	0x2046f6d(%rip), %r12   # 0x142ad9ffb
140a9308e: 4d 01 c4                    	addq	%r8, %r12
140a93091: 48 8d 05 ad 72 04 02        	leaq	0x20472ad(%rip), %rax   # 0x142ada345
140a93098: 4c 8d 2d 6e 72 04 02        	leaq	0x204726e(%rip), %r13   # 0x142ada30d
140a9309f: 90                          	nop
140a930a0: 0f be 48 ff                 	movsbl	-0x1(%rax), %ecx
140a930a4: 85 c9                       	testl	%ecx, %ecx
140a930a6: 78 18                       	js	0x140a930c0 <.text+0xa920c0>
140a930a8: 48 ff c8                    	decq	%rax
140a930ab: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a930ae: 75 69                       	jne	0x140a93119 <.text+0xa92119>
140a930b0: eb 7c                       	jmp	0x140a9312e <.text+0xa9212e>
140a930b2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a930c0: 0f b6 50 fe                 	movzbl	-0x2(%rax), %edx
140a930c4: 80 fa c0                    	cmpb	$-0x40, %dl
140a930c7: 7d 25                       	jge	0x140a930ee <.text+0xa920ee>
140a930c9: 44 0f b6 40 fd              	movzbl	-0x3(%rax), %r8d
140a930ce: 41 80 f8 c0                 	cmpb	$-0x40, %r8b
140a930d2: 7d 23                       	jge	0x140a930f7 <.text+0xa920f7>
140a930d4: 44 0f b6 48 fc              	movzbl	-0x4(%rax), %r9d
140a930d9: 48 83 c0 fc                 	addq	$-0x4, %rax
140a930dd: 41 83 e1 07                 	andl	$0x7, %r9d
140a930e1: 41 c1 e1 06                 	shll	$0x6, %r9d
140a930e5: 41 83 e0 3f                 	andl	$0x3f, %r8d
140a930e9: 45 09 c8                    	orl	%r9d, %r8d
140a930ec: eb 11                       	jmp	0x140a930ff <.text+0xa920ff>
140a930ee: 48 83 c0 fe                 	addq	$-0x2, %rax
140a930f2: 83 e2 1f                    	andl	$0x1f, %edx
140a930f5: eb 12                       	jmp	0x140a93109 <.text+0xa92109>
140a930f7: 48 83 c0 fd                 	addq	$-0x3, %rax
140a930fb: 41 83 e0 0f                 	andl	$0xf, %r8d
140a930ff: 41 c1 e0 06                 	shll	$0x6, %r8d
140a93103: 83 e2 3f                    	andl	$0x3f, %edx
140a93106: 44 09 c2                    	orl	%r8d, %edx
140a93109: c1 e2 06                    	shll	$0x6, %edx
140a9310c: 80 e1 3f                    	andb	$0x3f, %cl
140a9310f: 0f b6 c9                    	movzbl	%cl, %ecx
140a93112: 09 d1                       	orl	%edx, %ecx
140a93114: 83 f9 5c                    	cmpl	$0x5c, %ecx
140a93117: 74 15                       	je	0x140a9312e <.text+0xa9212e>
140a93119: 83 f9 2f                    	cmpl	$0x2f, %ecx
140a9311c: 74 10                       	je	0x140a9312e <.text+0xa9212e>
140a9311e: 4c 39 e8                    	cmpq	%r13, %rax
140a93121: 0f 85 79 ff ff ff           	jne	0x140a930a0 <.text+0xa920a0>
140a93127: bb 38 00 00 00              	movl	$0x38, %ebx
140a9312c: eb 4f                       	jmp	0x140a9317d <.text+0xa9217d>
140a9312e: 4c 8d 2d d8 71 04 02        	leaq	0x20471d8(%rip), %r13   # 0x142ada30d
140a93135: 4c 29 e8                    	subq	%r13, %rax
140a93138: 49 89 c0                    	movq	%rax, %r8
140a9313b: 49 ff c0                    	incq	%r8
140a9313e: 74 32                       	je	0x140a93172 <.text+0xa92172>
140a93140: 49 83 f8 38                 	cmpq	$0x38, %r8
140a93144: 73 2a                       	jae	0x140a93170 <.text+0xa92170>
140a93146: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
140a9314b: 7f 25                       	jg	0x140a93172 <.text+0xa92172>
140a9314d: 48 8d 05 a4 6f 04 02        	leaq	0x2046fa4(%rip), %rax   # 0x142ada0f8
140a93154: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a93159: 48 8d 0d ad 71 04 02        	leaq	0x20471ad(%rip), %rcx   # 0x142ada30d
140a93160: ba 38 00 00 00              	movl	$0x38, %edx
140a93165: 41 b9 38 00 00 00           	movl	$0x38, %r9d
140a9316b: e8 d0 67 e4 01              	callq	0x1428d9940 <.text+0x28d8940>
140a93170: 75 db                       	jne	0x140a9314d <.text+0xa9214d>
140a93172: bb 37 00 00 00              	movl	$0x37, %ebx
140a93177: 48 29 c3                    	subq	%rax, %rbx
140a9317a: 4d 01 c5                    	addq	%r8, %r13
140a9317d: e8 ae 4f 18 00              	callq	0x140c18130 <.text+0xc17130>
140a93182: 48 89 c1                    	movq	%rax, %rcx
140a93185: 48 8b 00                    	movq	(%rax), %rax
140a93188: 48 83 f8 01                 	cmpq	$0x1, %rax
140a9318c: 0f 85 0c 05 00 00           	jne	0x140a9369e <.text+0xa9269e>
140a93192: 48 83 c1 08                 	addq	$0x8, %rcx
140a93196: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a9319a: 0f 85 42 05 00 00           	jne	0x140a936e2 <.text+0xa926e2>
140a931a0: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a931a7: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
140a931ab: 48 83 c1 08                 	addq	$0x8, %rcx
140a931af: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
140a931b4: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
140a931bd: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
140a931c6: 4c 89 e2                    	movq	%r12, %rdx
140a931c9: 49 89 f0                    	movq	%rsi, %r8
140a931cc: 4d 89 e9                    	movq	%r13, %r9
140a931cf: e8 6c 3b 18 00              	callq	0x140c16d40 <.text+0xc15d40>
140a931d4: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
140a931d8: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140a931dc: 48 ff 00                    	incq	(%rax)
140a931df: 49 8b 0f                    	movq	(%r15), %rcx
140a931e2: 49 8b 47 08                 	movq	0x8(%r15), %rax
140a931e6: 48 89 45 28                 	movq	%rax, 0x28(%rbp)
140a931ea: 45 8b 7f 14                 	movl	0x14(%r15), %r15d
140a931ee: 4c 8b 81 f8 00 00 00        	movq	0xf8(%rcx), %r8
140a931f5: 48 8b 81 00 01 00 00        	movq	0x100(%rcx), %rax
140a931fc: 4d 8d 0c 80                 	leaq	(%r8,%rax,4), %r9
140a93200: 48 89 4d 08                 	movq	%rcx, 0x8(%rbp)
140a93204: 0f b6 81 08 01 00 00        	movzbl	0x108(%rcx), %eax
140a9320b: 49 8b 0e                    	movq	(%r14), %rcx
140a9320e: 48 89 4d f0                 	movq	%rcx, -0x10(%rbp)
140a93212: 49 8b 4e 08                 	movq	0x8(%r14), %rcx
140a93216: 48 89 4d c0                 	movq	%rcx, -0x40(%rbp)
140a9321a: 41 89 c6                    	movl	%eax, %r14d
140a9321d: 48 8b 07                    	movq	(%rdi), %rax
140a93220: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
140a93224: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140a93228: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
140a9322c: f3 0f 10 05 c4 84 f5 01     	movss	0x1f584c4(%rip), %xmm0  # 0x1429eb6f8
140a93234: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a93237: 0f 28 15 42 b7 e9 01        	movaps	0x1e9b742(%rip), %xmm2  # 0x14292e980
140a9323e: f3 0f 10 1d 8a a0 f3 01     	movss	0x1f3a08a(%rip), %xmm3  # 0x1429cd2d0
140a93246: f3 0f 10 25 b6 26 e9 01     	movss	0x1e926b6(%rip), %xmm4  # 0x142925904
140a9324e: 48 c7 45 38 00 00 00 00     	movq	$0x0, 0x38(%rbp)
140a93256: b8 08 00 00 00              	movl	$0x8, %eax
140a9325b: 48 89 45 00                 	movq	%rax, (%rbp)
140a9325f: 45 31 e4                    	xorl	%r12d, %r12d
140a93262: 45 31 d2                    	xorl	%r10d, %r10d
140a93265: 45 31 ed                    	xorl	%r13d, %r13d
140a93268: 31 d2                       	xorl	%edx, %edx
140a9326a: 4c 89 4d c8                 	movq	%r9, -0x38(%rbp)
140a9326e: 44 88 75 56                 	movb	%r14b, 0x56(%rbp)
140a93272: 45 84 f6                    	testb	%r14b, %r14b
140a93275: 74 49                       	je	0x140a932c0 <.text+0xa922c0>
140a93277: 4d 39 e2                    	cmpq	%r12, %r10
140a9327a: 0f 85 90 00 00 00           	jne	0x140a93310 <.text+0xa92310>
140a93280: 4d 39 c8                    	cmpq	%r9, %r8
140a93283: 0f 84 b9 03 00 00           	je	0x140a93642 <.text+0xa92642>
140a93289: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140a9328d: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
140a93294: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140a932a0: 41 8b 08                    	movl	(%r8), %ecx
140a932a3: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140a932a7: 4c 8b 64 c8 10              	movq	0x10(%rax,%rcx,8), %r12
140a932ac: 4d 85 e4                    	testq	%r12, %r12
140a932af: 75 77                       	jne	0x140a93328 <.text+0xa92328>
140a932b1: 49 83 c0 04                 	addq	$0x4, %r8
140a932b5: 4d 39 c8                    	cmpq	%r9, %r8
140a932b8: 75 e6                       	jne	0x140a932a0 <.text+0xa922a0>
140a932ba: e9 83 03 00 00              	jmp	0x140a93642 <.text+0xa92642>
140a932bf: 90                          	nop
140a932c0: 4d 39 e2                    	cmpq	%r12, %r10
140a932c3: 75 5b                       	jne	0x140a93320 <.text+0xa92320>
140a932c5: 4d 39 c8                    	cmpq	%r9, %r8
140a932c8: 0f 84 74 03 00 00           	je	0x140a93642 <.text+0xa92642>
140a932ce: 48 8b 45 28                 	movq	0x28(%rbp), %rax
140a932d2: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
140a932d9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a932e0: 41 8b 08                    	movl	(%r8), %ecx
140a932e3: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
140a932e7: 48 c1 e1 05                 	shlq	$0x5, %rcx
140a932eb: 4c 8b 64 08 58              	movq	0x58(%rax,%rcx), %r12
140a932f0: 4d 85 e4                    	testq	%r12, %r12
140a932f3: 75 7a                       	jne	0x140a9336f <.text+0xa9236f>
140a932f5: 49 83 c0 04                 	addq	$0x4, %r8
140a932f9: 4d 39 c8                    	cmpq	%r9, %r8
140a932fc: 75 e2                       	jne	0x140a932e0 <.text+0xa922e0>
140a932fe: e9 3f 03 00 00              	jmp	0x140a93642 <.text+0xa92642>
140a93303: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a93310: 89 55 50                    	movl	%edx, 0x50(%rbp)
140a93313: eb 55                       	jmp	0x140a9336a <.text+0xa9236a>
140a93315: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a93320: 89 55 50                    	movl	%edx, 0x50(%rbp)
140a93323: e9 a7 00 00 00              	jmp	0x140a933cf <.text+0xa923cf>
140a93328: 89 55 50                    	movl	%edx, 0x50(%rbp)
140a9332b: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
140a9332f: 48 8b 4d 08                 	movq	0x8(%rbp), %rcx
140a93333: 48 8b 89 18 01 00 00        	movq	0x118(%rcx), %rcx
140a9333a: 48 8b 50 18                 	movq	0x18(%rax), %rdx
140a9333e: 48 8b 40 38                 	movq	0x38(%rax), %rax
140a93342: 48 8b 04 c8                 	movq	(%rax,%rcx,8), %rax
140a93346: 48 f7 d0                    	notq	%rax
140a93349: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
140a9334d: 48 c1 e0 04                 	shlq	$0x4, %rax
140a93351: 48 8b 4c 02 10              	movq	0x10(%rdx,%rax), %rcx
140a93356: 48 89 4d 38                 	movq	%rcx, 0x38(%rbp)
140a9335a: 48 8b 44 02 28              	movq	0x28(%rdx,%rax), %rax
140a9335f: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
140a93363: 49 83 c0 04                 	addq	$0x4, %r8
140a93367: 45 31 d2                    	xorl	%r10d, %r10d
140a9336a: 44 89 d0                    	movl	%r10d, %eax
140a9336d: eb 6f                       	jmp	0x140a933de <.text+0xa923de>
140a9336f: 89 55 50                    	movl	%edx, 0x50(%rbp)
140a93372: 48 01 c8                    	addq	%rcx, %rax
140a93375: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
140a9337b: 48 8b 55 28                 	movq	0x28(%rbp), %rdx
140a9337f: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
140a93386: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
140a9338a: 4c 8b 55 08                 	movq	0x8(%rbp), %r10
140a9338e: 4d 8b 92 18 01 00 00        	movq	0x118(%r10), %r10
140a93395: 4c 8b 5c ca 18              	movq	0x18(%rdx,%rcx,8), %r11
140a9339a: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
140a9339f: 4a 8b 0c d1                 	movq	(%rcx,%r10,8), %rcx
140a933a3: 48 f7 d1                    	notq	%rcx
140a933a6: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
140a933aa: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a933ae: 49 8b 54 0b 10              	movq	0x10(%r11,%rcx), %rdx
140a933b3: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
140a933b7: 49 8b 4c 0b 28              	movq	0x28(%r11,%rcx), %rcx
140a933bc: 48 89 4d 20                 	movq	%rcx, 0x20(%rbp)
140a933c0: 48 8b 40 50                 	movq	0x50(%rax), %rax
140a933c4: 48 89 45 00                 	movq	%rax, (%rbp)
140a933c8: 49 83 c0 04                 	addq	$0x4, %r8
140a933cc: 45 31 d2                    	xorl	%r10d, %r10d
140a933cf: 4c 89 d0                    	movq	%r10, %rax
140a933d2: 48 c1 e0 04                 	shlq	$0x4, %rax
140a933d6: 48 8b 4d 00                 	movq	(%rbp), %rcx
140a933da: 8b 44 01 08                 	movl	0x8(%rcx,%rax), %eax
140a933de: 48 8b 4d 38                 	movq	0x38(%rbp), %rcx
140a933e2: 48 8d 0c c1                 	leaq	(%rcx,%rax,8), %rcx
140a933e6: 48 89 4d 40                 	movq	%rcx, 0x40(%rbp)
140a933ea: 48 8b 4d 20                 	movq	0x20(%rbp), %rcx
140a933ee: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
140a933f2: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140a933f6: 49 ff c2                    	incq	%r10
140a933f9: 4c 89 55 48                 	movq	%r10, 0x48(%rbp)
140a933fd: 48 8b 75 f0                 	movq	-0x10(%rbp), %rsi
140a93401: 48 8b 06                    	movq	(%rsi), %rax
140a93404: 48 89 c1                    	movq	%rax, %rcx
140a93407: 48 8b 7d c0                 	movq	-0x40(%rbp), %rdi
140a9340b: 48 2b 4f 18                 	subq	0x18(%rdi), %rcx
140a9340f: bb 00 00 00 00              	movl	$0x0, %ebx
140a93414: 48 0f 42 cb                 	cmovbq	%rbx, %rcx
140a93418: 48 2b 47 38                 	subq	0x38(%rdi), %rax
140a9341c: 48 0f 42 c3                 	cmovbq	%rbx, %rax
140a93420: 48 8b 57 30                 	movq	0x30(%rdi), %rdx
140a93424: 48 29 c2                    	subq	%rax, %rdx
140a93427: 48 0f 42 d3                 	cmovbq	%rbx, %rdx
140a9342b: 41 0f 97 c2                 	seta	%r10b
140a9342f: 4c 8b 5f 10                 	movq	0x10(%rdi), %r11
140a93433: 49 29 cb                    	subq	%rcx, %r11
140a93436: 4c 0f 42 db                 	cmovbq	%rbx, %r11
140a9343a: 48 8b 47 40                 	movq	0x40(%rdi), %rax
140a9343e: 0f 97 c1                    	seta	%cl
140a93441: 49 01 d3                    	addq	%rdx, %r11
140a93444: 48 89 c2                    	movq	%rax, %rdx
140a93447: 4c 29 da                    	subq	%r11, %rdx
140a9344a: 48 89 16                    	movq	%rdx, (%rsi)
140a9344d: 44 08 d1                    	orb	%r10b, %cl
140a93450: 0f 84 3a 01 00 00           	je	0x140a93590 <.text+0xa92590>
140a93456: 48 8b 4d f0                 	movq	-0x10(%rbp), %rcx
140a9345a: 48 89 01                    	movq	%rax, (%rcx)
140a9345d: 4c 8b 4d f8                 	movq	-0x8(%rbp), %r9
140a93461: 4d 8b 11                    	movq	(%r9), %r10
140a93464: 4c 89 d3                    	movq	%r10, %rbx
140a93467: 44 89 6d 1c                 	movl	%r13d, 0x1c(%rbp)
140a9346b: 4c 8b 6d d0                 	movq	-0x30(%rbp), %r13
140a9346f: 49 2b 5d 18                 	subq	0x18(%r13), %rbx
140a93473: b9 00 00 00 00              	movl	$0x0, %ecx
140a93478: 48 0f 42 d9                 	cmovbq	%rcx, %rbx
140a9347c: 4d 2b 55 38                 	subq	0x38(%r13), %r10
140a93480: 4c 0f 42 d1                 	cmovbq	%rcx, %r10
140a93484: 49 8b 7d 08                 	movq	0x8(%r13), %rdi
140a93488: 49 8b 75 30                 	movq	0x30(%r13), %rsi
140a9348c: 48 89 75 d8                 	movq	%rsi, -0x28(%rbp)
140a93490: 4c 29 d6                    	subq	%r10, %rsi
140a93493: 48 0f 42 f1                 	cmovbq	%rcx, %rsi
140a93497: 49 8b 45 10                 	movq	0x10(%r13), %rax
140a9349b: 41 0f 96 c6                 	setbe	%r14b
140a9349f: 48 89 45 e8                 	movq	%rax, -0x18(%rbp)
140a934a3: 48 29 d8                    	subq	%rbx, %rax
140a934a6: 48 0f 42 c1                 	cmovbq	%rcx, %rax
140a934aa: 49 8b 4d 40                 	movq	0x40(%r13), %rcx
140a934ae: 45 89 fb                    	movl	%r15d, %r11d
140a934b1: 41 0f 96 c7                 	setbe	%r15b
140a934b5: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
140a934b9: 48 29 f1                    	subq	%rsi, %rcx
140a934bc: 48 29 c1                    	subq	%rax, %rcx
140a934bf: 49 8b 55 28                 	movq	0x28(%r13), %rdx
140a934c3: 44 8b 6d 1c                 	movl	0x1c(%rbp), %r13d
140a934c7: 49 89 09                    	movq	%rcx, (%r9)
140a934ca: 4c 8b 4d c8                 	movq	-0x38(%rbp), %r9
140a934ce: 45 84 f7                    	testb	%r14b, %r15b
140a934d1: 44 0f b6 75 56              	movzbl	0x56(%rbp), %r14d
140a934d6: 45 89 df                    	movl	%r11d, %r15d
140a934d9: 0f 85 b1 00 00 00           	jne	0x140a93590 <.text+0xa92590>
140a934df: 4c 89 d1                    	movq	%r10, %rcx
140a934e2: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a934e6: 48 01 ca                    	addq	%rcx, %rdx
140a934e9: 48 c1 e6 04                 	shlq	$0x4, %rsi
140a934ed: 48 89 d9                    	movq	%rbx, %rcx
140a934f0: 48 c1 e1 04                 	shlq	$0x4, %rcx
140a934f4: 48 01 cf                    	addq	%rcx, %rdi
140a934f7: 48 c1 e0 04                 	shlq	$0x4, %rax
140a934fb: 48 39 5d e8                 	cmpq	%rbx, -0x18(%rbp)
140a934ff: 41 bb 08 00 00 00           	movl	$0x8, %r11d
140a93505: 49 0f 42 fb                 	cmovbq	%r11, %rdi
140a93509: 48 8d 44 07 f0              	leaq	-0x10(%rdi,%rax), %rax
140a9350e: b9 00 00 00 00              	movl	$0x0, %ecx
140a93513: 48 0f 47 c8                 	cmovaq	%rax, %rcx
140a93517: 4c 39 55 d8                 	cmpq	%r10, -0x28(%rbp)
140a9351b: 49 0f 42 d3                 	cmovbq	%r11, %rdx
140a9351f: 48 8d 44 32 f0              	leaq	-0x10(%rdx,%rsi), %rax
140a93524: 48 0f 46 c1                 	cmovbeq	%rcx, %rax
140a93528: 48 8b 4d f8                 	movq	-0x8(%rbp), %rcx
140a9352c: 48 8b 55 e0                 	movq	-0x20(%rbp), %rdx
140a93530: 48 89 11                    	movq	%rdx, (%rcx)
140a93533: f2 0f 10 68 08              	movsd	0x8(%rax), %xmm5
140a93538: 0f 28 f5                    	movaps	%xmm5, %xmm6
140a9353b: 0f 59 f5                    	mulps	%xmm5, %xmm6
140a9353e: f3 0f 16 fe                 	movshdup	%xmm6, %xmm7            # xmm7 = xmm6[1,1,3,3]
140a93542: f3 0f 58 fe                 	addss	%xmm6, %xmm7
140a93546: 0f 57 f6                    	xorps	%xmm6, %xmm6
140a93549: f3 0f 51 f7                 	sqrtss	%xmm7, %xmm6
140a9354d: 0f 2e f1                    	ucomiss	%xmm1, %xmm6
140a93550: 76 3e                       	jbe	0x140a93590 <.text+0xa92590>
140a93552: 0f 28 f5                    	movaps	%xmm5, %xmm6
140a93555: 0f 54 f2                    	andps	%xmm2, %xmm6
140a93558: 66 0f 70 fe f5              	pshufd	$0xf5, %xmm6, %xmm7     # xmm7 = xmm6[1,1,3,3]
140a9355d: 0f 2e fe                    	ucomiss	%xmm6, %xmm7
140a93560: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140a93564: 44 89 39                    	movl	%r15d, (%rcx)
140a93567: 0f 86 9d 00 00 00           	jbe	0x140a9360a <.text+0xa9260a>
140a9356d: f3 0f 16 ed                 	movshdup	%xmm5, %xmm5            # xmm5 = xmm5[1,1,3,3]
140a93571: f3 0f 59 eb                 	mulss	%xmm3, %xmm5
140a93575: 0f 54 ea                    	andps	%xmm2, %xmm5
140a93578: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a9357c: f3 0f 58 68 04              	addss	0x4(%rax), %xmm5
140a93581: f3 0f 5d ec                 	minss	%xmm4, %xmm5
140a93585: f3 0f 11 68 04              	movss	%xmm5, 0x4(%rax)
140a9358a: 41 b5 01                    	movb	$0x1, %r13b
140a9358d: eb 51                       	jmp	0x140a935e0 <.text+0xa925e0>
140a9358f: 90                          	nop
140a93590: 41 f6 c5 01                 	testb	$0x1, %r13b
140a93594: 74 2a                       	je	0x140a935c0 <.text+0xa925c0>
140a93596: 41 b5 01                    	movb	$0x1, %r13b
140a93599: f6 45 50 01                 	testb	$0x1, 0x50(%rbp)
140a9359d: b2 01                       	movb	$0x1, %dl
140a9359f: 4c 8b 55 48                 	movq	0x48(%rbp), %r10
140a935a3: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a935a7: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140a935ab: 0f 85 c1 fc ff ff           	jne	0x140a93272 <.text+0xa92272>
140a935b1: eb 3d                       	jmp	0x140a935f0 <.text+0xa925f0>
140a935b3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a935c0: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
140a935c4: 44 89 39                    	movl	%r15d, (%rcx)
140a935c7: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a935cb: f3 0f 10 68 04              	movss	0x4(%rax), %xmm5
140a935d0: f3 0f 58 e8                 	addss	%xmm0, %xmm5
140a935d4: f3 0f 5f e9                 	maxss	%xmm1, %xmm5
140a935d8: f3 0f 11 68 04              	movss	%xmm5, 0x4(%rax)
140a935dd: 45 31 ed                    	xorl	%r13d, %r13d
140a935e0: f6 45 50 01                 	testb	$0x1, 0x50(%rbp)
140a935e4: b2 01                       	movb	$0x1, %dl
140a935e6: 4c 8b 55 48                 	movq	0x48(%rbp), %r10
140a935ea: 0f 85 82 fc ff ff           	jne	0x140a93272 <.text+0xa92272>
140a935f0: 44 89 39                    	movl	%r15d, (%rcx)
140a935f3: 31 d2                       	xorl	%edx, %edx
140a935f5: f3 0f 10 28                 	movss	(%rax), %xmm5
140a935f9: f3 0f 58 e8                 	addss	%xmm0, %xmm5
140a935fd: f3 0f 5f e9                 	maxss	%xmm1, %xmm5
140a93601: f3 0f 11 28                 	movss	%xmm5, (%rax)
140a93605: e9 68 fc ff ff              	jmp	0x140a93272 <.text+0xa92272>
140a9360a: f3 0f 59 eb                 	mulss	%xmm3, %xmm5
140a9360e: 0f 54 ea                    	andps	%xmm2, %xmm5
140a93611: 48 8b 45 40                 	movq	0x40(%rbp), %rax
140a93615: f3 0f 58 28                 	addss	(%rax), %xmm5
140a93619: f3 0f 5d ec                 	minss	%xmm4, %xmm5
140a9361d: f3 0f 11 28                 	movss	%xmm5, (%rax)
140a93621: b2 01                       	movb	$0x1, %dl
140a93623: 41 f6 c5 01                 	testb	$0x1, %r13b
140a93627: 41 b5 01                    	movb	$0x1, %r13b
140a9362a: 4c 8b 55 48                 	movq	0x48(%rbp), %r10
140a9362e: 0f 85 3e fc ff ff           	jne	0x140a93272 <.text+0xa92272>
140a93634: 44 89 39                    	movl	%r15d, (%rcx)
140a93637: 48 83 c0 04                 	addq	$0x4, %rax
140a9363b: b2 01                       	movb	$0x1, %dl
140a9363d: 45 31 ed                    	xorl	%r13d, %r13d
140a93640: eb b3                       	jmp	0x140a935f5 <.text+0xa925f5>
140a93642: 80 7d 57 00                 	cmpb	$0x0, 0x57(%rbp)
140a93646: 74 3a                       	je	0x140a93682 <.text+0xa92682>
140a93648: e8 e3 4a 18 00              	callq	0x140c18130 <.text+0xc17130>
140a9364d: 48 89 c1                    	movq	%rax, %rcx
140a93650: 48 8b 00                    	movq	(%rax), %rax
140a93653: 48 83 f8 01                 	cmpq	$0x1, %rax
140a93657: 75 5f                       	jne	0x140a936b8 <.text+0xa926b8>
140a93659: 48 83 c1 08                 	addq	$0x8, %rcx
140a9365d: 48 8b 55 10                 	movq	0x10(%rbp), %rdx
140a93661: 48 83 39 00                 	cmpq	$0x0, (%rcx)
140a93665: 75 7b                       	jne	0x140a936e2 <.text+0xa926e2>
140a93667: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
140a9366e: 48 89 4d 48                 	movq	%rcx, 0x48(%rbp)
140a93672: 48 83 c1 08                 	addq	$0x8, %rcx
140a93676: e8 75 3a 18 00              	callq	0x140c170f0 <.text+0xc160f0>
140a9367b: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140a9367f: 48 ff 00                    	incq	(%rax)
140a93682: 0f 28 75 60                 	movaps	0x60(%rbp), %xmm6
140a93686: 0f 28 7d 70                 	movaps	0x70(%rbp), %xmm7
140a9368a: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
140a93691: 5b                          	popq	%rbx
140a93692: 5f                          	popq	%rdi
140a93693: 5e                          	popq	%rsi
140a93694: 41 5c                       	popq	%r12
140a93696: 41 5d                       	popq	%r13
140a93698: 41 5e                       	popq	%r14
140a9369a: 41 5f                       	popq	%r15
140a9369c: 5d                          	popq	%rbp
140a9369d: c3                          	retq
140a9369e: 48 85 c0                    	testq	%rax, %rax
140a936a1: 75 33                       	jne	0x140a936d6 <.text+0xa926d6>
140a936a3: 31 d2                       	xorl	%edx, %edx
140a936a5: e8 26 51 e6 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a936aa: 48 89 c1                    	movq	%rax, %rcx
140a936ad: 48 85 c0                    	testq	%rax, %rax
140a936b0: 0f 85 e0 fa ff ff           	jne	0x140a93196 <.text+0xa92196>
140a936b6: eb 1e                       	jmp	0x140a936d6 <.text+0xa926d6>
140a936b8: 48 85 c0                    	testq	%rax, %rax
140a936bb: 48 8b 45 10                 	movq	0x10(%rbp), %rax
140a936bf: 75 15                       	jne	0x140a936d6 <.text+0xa926d6>
140a936c1: 48 89 c6                    	movq	%rax, %rsi
140a936c4: 31 d2                       	xorl	%edx, %edx
140a936c6: e8 05 51 e6 01              	callq	0x1428f87d0 <.text+0x28f77d0>
140a936cb: 48 89 f2                    	movq	%rsi, %rdx
140a936ce: 48 89 c1                    	movq	%rax, %rcx
140a936d1: 48 85 c0                    	testq	%rax, %rax
140a936d4: 75 8b                       	jne	0x140a93661 <.text+0xa92661>
140a936d6: 48 8d 0d c3 73 1a 02        	leaq	0x21a73c3(%rip), %rcx   # 0x142c3aaa0
140a936dd: e8 ee 49 e4 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a936e2: 48 8d 0d 87 74 1a 02        	leaq	0x21a7487(%rip), %rcx   # 0x142c3ab70
140a936e9: e8 62 5b e4 01              	callq	0x1428d9250 <.text+0x28d8250>
140a936ee: cc                          	int3
140a936ef: 90                          	nop
140a936f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a936f5: 55                          	pushq	%rbp
140a936f6: 41 57                       	pushq	%r15
140a936f8: 41 56                       	pushq	%r14
140a936fa: 41 55                       	pushq	%r13
140a936fc: 41 54                       	pushq	%r12
140a936fe: 56                          	pushq	%rsi
140a936ff: 57                          	pushq	%rdi
140a93700: 53                          	pushq	%rbx
140a93701: 48 83 ec 58                 	subq	$0x58, %rsp
140a93705: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a9370c: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140a93711: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
140a93716: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140a9371a: 48 ff 00                    	incq	(%rax)
140a9371d: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140a93722: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140a93727: 48 83 c4 58                 	addq	$0x58, %rsp
140a9372b: 5b                          	popq	%rbx
140a9372c: 5f                          	popq	%rdi
140a9372d: 5e                          	popq	%rsi
140a9372e: 41 5c                       	popq	%r12
140a93730: 41 5d                       	popq	%r13
140a93732: 41 5e                       	popq	%r14
140a93734: 41 5f                       	popq	%r15
140a93736: 5d                          	popq	%rbp
140a93737: c3                          	retq
140a93738: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
140a93740: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a93745: 55                          	pushq	%rbp
140a93746: 41 57                       	pushq	%r15
140a93748: 41 56                       	pushq	%r14
140a9374a: 41 55                       	pushq	%r13
140a9374c: 41 54                       	pushq	%r12
140a9374e: 56                          	pushq	%rsi
140a9374f: 57                          	pushq	%rdi
140a93750: 53                          	pushq	%rbx
140a93751: 48 83 ec 58                 	subq	$0x58, %rsp
140a93755: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a9375c: 0f 29 7c 24 30              	movaps	%xmm7, 0x30(%rsp)
140a93761: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
140a93766: 48 8b 45 48                 	movq	0x48(%rbp), %rax
140a9376a: 48 ff 00                    	incq	(%rax)
140a9376d: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
140a93772: 0f 28 7c 24 30              	movaps	0x30(%rsp), %xmm7
140a93777: 48 83 c4 58                 	addq	$0x58, %rsp
140a9377b: 5b                          	popq	%rbx
140a9377c: 5f                          	popq	%rdi
140a9377d: 5e                          	popq	%rsi
140a9377e: 41 5c                       	popq	%r12
140a93780: 41 5d                       	popq	%r13
140a93782: 41 5e                       	popq	%r14
140a93784: 41 5f                       	popq	%r15
140a93786: 5d                          	popq	%rbp
140a93787: c3                          	retq
