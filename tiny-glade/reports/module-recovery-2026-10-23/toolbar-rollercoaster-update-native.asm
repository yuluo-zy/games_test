
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1403b7f20: 55                          	pushq	%rbp
1403b7f21: 41 57                       	pushq	%r15
1403b7f23: 41 56                       	pushq	%r14
1403b7f25: 41 55                       	pushq	%r13
1403b7f27: 41 54                       	pushq	%r12
1403b7f29: 56                          	pushq	%rsi
1403b7f2a: 57                          	pushq	%rdi
1403b7f2b: 53                          	pushq	%rbx
1403b7f2c: 48 81 ec c8 08 00 00        	subq	$0x8c8, %rsp            # imm = 0x8C8
1403b7f33: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1403b7f3b: 44 0f 29 bd 30 08 00 00     	movaps	%xmm15, 0x830(%rbp)
1403b7f43: 44 0f 29 b5 20 08 00 00     	movaps	%xmm14, 0x820(%rbp)
1403b7f4b: 44 0f 29 ad 10 08 00 00     	movaps	%xmm13, 0x810(%rbp)
1403b7f53: 44 0f 29 a5 00 08 00 00     	movaps	%xmm12, 0x800(%rbp)
1403b7f5b: 44 0f 29 9d f0 07 00 00     	movaps	%xmm11, 0x7f0(%rbp)
1403b7f63: 44 0f 29 95 e0 07 00 00     	movaps	%xmm10, 0x7e0(%rbp)
1403b7f6b: 44 0f 29 8d d0 07 00 00     	movaps	%xmm9, 0x7d0(%rbp)
1403b7f73: 44 0f 29 85 c0 07 00 00     	movaps	%xmm8, 0x7c0(%rbp)
1403b7f7b: 0f 29 bd b0 07 00 00        	movaps	%xmm7, 0x7b0(%rbp)
1403b7f82: 0f 29 b5 a0 07 00 00        	movaps	%xmm6, 0x7a0(%rbp)
1403b7f89: 48 c7 85 98 07 00 00 fe ff ff ff    	movq	$-0x2, 0x798(%rbp)
1403b7f94: 4d 89 cf                    	movq	%r9, %r15
1403b7f97: 4c 89 85 50 03 00 00        	movq	%r8, 0x350(%rbp)
1403b7f9e: 48 89 95 20 06 00 00        	movq	%rdx, 0x620(%rbp)
1403b7fa5: 48 89 8d 80 07 00 00        	movq	%rcx, 0x780(%rbp)
1403b7fac: 48 8b 85 c0 08 00 00        	movq	0x8c0(%rbp), %rax
1403b7fb3: 0f 10 00                    	movups	(%rax), %xmm0
1403b7fb6: f3 0f 6f 48 10              	movdqu	0x10(%rax), %xmm1
1403b7fbb: 0f 10 50 20                 	movups	0x20(%rax), %xmm2
1403b7fbf: 0f 10 58 30                 	movups	0x30(%rax), %xmm3
1403b7fc3: 0f 29 5d f0                 	movaps	%xmm3, -0x10(%rbp)
1403b7fc7: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1403b7fcb: 66 0f 7f 4d d0              	movdqa	%xmm1, -0x30(%rbp)
1403b7fd0: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1403b7fd4: 4c 8b 70 40                 	movq	0x40(%rax), %r14
1403b7fd8: 48 8b 48 60                 	movq	0x60(%rax), %rcx
1403b7fdc: 48 89 8d b8 02 00 00        	movq	%rcx, 0x2b8(%rbp)
1403b7fe3: 48 8b 88 80 00 00 00        	movq	0x80(%rax), %rcx
1403b7fea: 48 89 8d 30 07 00 00        	movq	%rcx, 0x730(%rbp)
1403b7ff1: 48 8b 88 a0 00 00 00        	movq	0xa0(%rax), %rcx
1403b7ff8: 48 89 8d 60 03 00 00        	movq	%rcx, 0x360(%rbp)
1403b7fff: 48 8b 88 c0 00 00 00        	movq	0xc0(%rax), %rcx
1403b8006: 48 89 8d 28 06 00 00        	movq	%rcx, 0x628(%rbp)
1403b800d: 48 8b 88 e0 00 00 00        	movq	0xe0(%rax), %rcx
1403b8014: 48 89 8d c8 02 00 00        	movq	%rcx, 0x2c8(%rbp)
1403b801b: 48 8b 88 00 01 00 00        	movq	0x100(%rax), %rcx
1403b8022: 48 89 8d 48 03 00 00        	movq	%rcx, 0x348(%rbp)
1403b8029: 48 8b 80 20 01 00 00        	movq	0x120(%rax), %rax
1403b8030: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
1403b8037: e8 c4 e9 85 00              	callq	0x140c16a00 <.text+0xc15a00>
1403b803c: 84 c0                       	testb	%al, %al
1403b803e: 4c 89 bd c0 02 00 00        	movq	%r15, 0x2c0(%rbp)
1403b8045: 74 5c                       	je	0x1403b80a3 <.text+0x3b70a3>
1403b8047: be 2f 00 00 00              	movl	$0x2f, %esi
1403b804c: b9 2f 00 00 00              	movl	$0x2f, %ecx
1403b8051: e8 5a e2 ff ff              	callq	0x1403b62b0 <.text+0x3b52b0>
1403b8056: 48 8d 3d a6 54 61 02        	leaq	0x26154a6(%rip), %rdi   # 0x1429cd503
1403b805d: 48 83 f8 01                 	cmpq	$0x1, %rax
1403b8061: 0f 85 a7 00 00 00           	jne	0x1403b810e <.text+0x3b710e>
1403b8067: 49 89 d1                    	movq	%rdx, %r9
1403b806a: 48 85 d2                    	testq	%rdx, %rdx
1403b806d: 74 3d                       	je	0x1403b80ac <.text+0x3b70ac>
1403b806f: 49 83 f9 2f                 	cmpq	$0x2f, %r9
1403b8073: 73 35                       	jae	0x1403b80aa <.text+0x3b70aa>
1403b8075: 48 8d 05 87 54 61 02        	leaq	0x2615487(%rip), %rax   # 0x1429cd503
1403b807c: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1403b8081: 7f 29                       	jg	0x1403b80ac <.text+0x3b70ac>
1403b8083: 48 8d 05 2e 55 61 02        	leaq	0x261552e(%rip), %rax   # 0x1429cd5b8
1403b808a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403b808f: 48 8d 0d 6d 54 61 02        	leaq	0x261546d(%rip), %rcx   # 0x1429cd503
1403b8096: ba 2f 00 00 00              	movl	$0x2f, %edx
1403b809b: 45 31 c0                    	xorl	%r8d, %r8d
1403b809e: e8 9d 18 52 02              	callq	0x1428d9940 <.text+0x28d8940>
1403b80a3: 31 c0                       	xorl	%eax, %eax
1403b80a5: e9 b9 01 00 00              	jmp	0x1403b8263 <.text+0x3b7263>
1403b80aa: 75 d7                       	jne	0x1403b8083 <.text+0x3b7083>
1403b80ac: 4c 89 c9                    	movq	%r9, %rcx
1403b80af: e8 fc e1 ff ff              	callq	0x1403b62b0 <.text+0x3b52b0>
1403b80b4: 48 83 f8 01                 	cmpq	$0x1, %rax
1403b80b8: 75 54                       	jne	0x1403b810e <.text+0x3b710e>
1403b80ba: 49 89 d0                    	movq	%rdx, %r8
1403b80bd: 49 83 c0 02                 	addq	$0x2, %r8
1403b80c1: 74 39                       	je	0x1403b80fc <.text+0x3b70fc>
1403b80c3: 49 83 f8 2f                 	cmpq	$0x2f, %r8
1403b80c7: 73 31                       	jae	0x1403b80fa <.text+0x3b70fa>
1403b80c9: 48 8d 05 33 54 61 02        	leaq	0x2615433(%rip), %rax   # 0x1429cd503
1403b80d0: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1403b80d5: 7f 25                       	jg	0x1403b80fc <.text+0x3b70fc>
1403b80d7: 48 8d 05 f2 54 61 02        	leaq	0x26154f2(%rip), %rax   # 0x1429cd5d0
1403b80de: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403b80e3: 48 8d 0d 19 54 61 02        	leaq	0x2615419(%rip), %rcx   # 0x1429cd503
1403b80ea: ba 2f 00 00 00              	movl	$0x2f, %edx
1403b80ef: 41 b9 2f 00 00 00           	movl	$0x2f, %r9d
1403b80f5: e8 46 18 52 02              	callq	0x1428d9940 <.text+0x28d8940>
1403b80fa: 75 db                       	jne	0x1403b80d7 <.text+0x3b70d7>
1403b80fc: be 2d 00 00 00              	movl	$0x2d, %esi
1403b8101: 48 29 d6                    	subq	%rdx, %rsi
1403b8104: 48 8d 3d f8 53 61 02        	leaq	0x26153f8(%rip), %rdi   # 0x1429cd503
1403b810b: 4c 01 c7                    	addq	%r8, %rdi
1403b810e: 4c 8d 05 e1 55 61 02        	leaq	0x26155e1(%rip), %r8    # 0x1429cd6f6
1403b8115: 48 8d 1d b4 55 61 02        	leaq	0x26155b4(%rip), %rbx   # 0x1429cd6d0
1403b811c: 0f 1f 40 00                 	nopl	(%rax)
1403b8120: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1403b8125: 85 c0                       	testl	%eax, %eax
1403b8127: 78 17                       	js	0x1403b8140 <.text+0x3b7140>
1403b8129: 49 ff c8                    	decq	%r8
1403b812c: 83 f8 5c                    	cmpl	$0x5c, %eax
1403b812f: 75 63                       	jne	0x1403b8194 <.text+0x3b7194>
1403b8131: eb 73                       	jmp	0x1403b81a6 <.text+0x3b71a6>
1403b8133: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1403b8140: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1403b8145: 80 f9 c0                    	cmpb	$-0x40, %cl
1403b8148: 7d 23                       	jge	0x1403b816d <.text+0x3b716d>
1403b814a: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1403b814f: 80 fa c0                    	cmpb	$-0x40, %dl
1403b8152: 7d 22                       	jge	0x1403b8176 <.text+0x3b7176>
1403b8154: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1403b8159: 49 83 c0 fc                 	addq	$-0x4, %r8
1403b815d: 41 83 e1 07                 	andl	$0x7, %r9d
1403b8161: 41 c1 e1 06                 	shll	$0x6, %r9d
1403b8165: 83 e2 3f                    	andl	$0x3f, %edx
1403b8168: 44 09 ca                    	orl	%r9d, %edx
1403b816b: eb 10                       	jmp	0x1403b817d <.text+0x3b717d>
1403b816d: 49 83 c0 fe                 	addq	$-0x2, %r8
1403b8171: 83 e1 1f                    	andl	$0x1f, %ecx
1403b8174: eb 0f                       	jmp	0x1403b8185 <.text+0x3b7185>
1403b8176: 49 83 c0 fd                 	addq	$-0x3, %r8
1403b817a: 83 e2 0f                    	andl	$0xf, %edx
1403b817d: c1 e2 06                    	shll	$0x6, %edx
1403b8180: 83 e1 3f                    	andl	$0x3f, %ecx
1403b8183: 09 d1                       	orl	%edx, %ecx
1403b8185: c1 e1 06                    	shll	$0x6, %ecx
1403b8188: 24 3f                       	andb	$0x3f, %al
1403b818a: 0f b6 c0                    	movzbl	%al, %eax
1403b818d: 09 c8                       	orl	%ecx, %eax
1403b818f: 83 f8 5c                    	cmpl	$0x5c, %eax
1403b8192: 74 12                       	je	0x1403b81a6 <.text+0x3b71a6>
1403b8194: 83 f8 2f                    	cmpl	$0x2f, %eax
1403b8197: 74 0d                       	je	0x1403b81a6 <.text+0x3b71a6>
1403b8199: 49 39 d8                    	cmpq	%rbx, %r8
1403b819c: 75 82                       	jne	0x1403b8120 <.text+0x3b7120>
1403b819e: 41 bf 26 00 00 00           	movl	$0x26, %r15d
1403b81a4: eb 4d                       	jmp	0x1403b81f3 <.text+0x3b71f3>
1403b81a6: 48 8d 1d 23 55 61 02        	leaq	0x2615523(%rip), %rbx   # 0x1429cd6d0
1403b81ad: 49 29 d8                    	subq	%rbx, %r8
1403b81b0: 49 ff c0                    	incq	%r8
1403b81b3: 74 32                       	je	0x1403b81e7 <.text+0x3b71e7>
1403b81b5: 49 83 f8 26                 	cmpq	$0x26, %r8
1403b81b9: 73 2a                       	jae	0x1403b81e5 <.text+0x3b71e5>
1403b81bb: 41 80 3c 18 bf              	cmpb	$-0x41, (%r8,%rbx)
1403b81c0: 7f 25                       	jg	0x1403b81e7 <.text+0x3b71e7>
1403b81c2: 48 8d 05 cf 53 61 02        	leaq	0x26153cf(%rip), %rax   # 0x1429cd598
1403b81c9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403b81ce: 48 8d 0d fb 54 61 02        	leaq	0x26154fb(%rip), %rcx   # 0x1429cd6d0
1403b81d5: ba 26 00 00 00              	movl	$0x26, %edx
1403b81da: 41 b9 26 00 00 00           	movl	$0x26, %r9d
1403b81e0: e8 5b 17 52 02              	callq	0x1428d9940 <.text+0x28d8940>
1403b81e5: 75 db                       	jne	0x1403b81c2 <.text+0x3b71c2>
1403b81e7: 41 bf 26 00 00 00           	movl	$0x26, %r15d
1403b81ed: 4d 29 c7                    	subq	%r8, %r15
1403b81f0: 4c 01 c3                    	addq	%r8, %rbx
1403b81f3: e8 38 ff 85 00              	callq	0x140c18130 <.text+0xc17130>
1403b81f8: 48 89 c1                    	movq	%rax, %rcx
1403b81fb: 48 8b 00                    	movq	(%rax), %rax
1403b81fe: 48 83 f8 01                 	cmpq	$0x1, %rax
1403b8202: 0f 85 2e 2e 00 00           	jne	0x1403bb036 <.text+0x3ba036>
1403b8208: 48 83 c1 08                 	addq	$0x8, %rcx
1403b820c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1403b8210: 0f 85 a8 3d 00 00           	jne	0x1403bbfbe <.text+0x3bafbe>
1403b8216: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1403b821d: 48 89 8d 88 07 00 00        	movq	%rcx, 0x788(%rbp)
1403b8224: 48 83 c1 08                 	addq	$0x8, %rcx
1403b8228: 4c 89 7c 24 20              	movq	%r15, 0x20(%rsp)
1403b822d: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1403b8236: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1403b823f: 48 89 fa                    	movq	%rdi, %rdx
1403b8242: 49 89 f0                    	movq	%rsi, %r8
1403b8245: 49 89 d9                    	movq	%rbx, %r9
1403b8248: e8 f3 ea 85 00              	callq	0x140c16d40 <.text+0xc15d40>
1403b824d: 48 8b 8d 88 07 00 00        	movq	0x788(%rbp), %rcx
1403b8254: 48 ff 01                    	incq	(%rcx)
1403b8257: 48 89 85 f0 05 00 00        	movq	%rax, 0x5f0(%rbp)
1403b825e: b8 01 00 00 00              	movl	$0x1, %eax
1403b8263: 48 8b 8d b8 08 00 00        	movq	0x8b8(%rbp), %rcx
1403b826a: 48 89 85 e8 05 00 00        	movq	%rax, 0x5e8(%rbp)
1403b8271: 48 8b 09                    	movq	(%rcx), %rcx
1403b8274: e8 f7 ff 4e 00              	callq	0x1408a8270 <.text+0x8a7270>
1403b8279: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403b827c: f3 0f 11 85 24 07 00 00     	movss	%xmm0, 0x724(%rbp)
1403b8284: 84 c0                       	testb	%al, %al
1403b8286: 75 0e                       	jne	0x1403b8296 <.text+0x3b7296>
1403b8288: f3 41 0f 10 46 28           	movss	0x28(%r14), %xmm0
1403b828e: f3 0f 11 85 24 07 00 00     	movss	%xmm0, 0x724(%rbp)
1403b8296: 31 c0                       	xorl	%eax, %eax
1403b8298: 4c 8b b5 80 07 00 00        	movq	0x780(%rbp), %r14
1403b829f: 49 3b 06                    	cmpq	(%r14), %rax
1403b82a2: 0f 80 8f 2a 00 00           	jo	0x1403bad37 <.text+0x3b9d37>
1403b82a8: 48 8b 85 20 06 00 00        	movq	0x620(%rbp), %rax
1403b82af: 8b 48 1c                    	movl	0x1c(%rax), %ecx
1403b82b2: 48 8b 50 10                 	movq	0x10(%rax), %rdx
1403b82b6: 89 8d dc 06 00 00           	movl	%ecx, 0x6dc(%rbp)
1403b82bc: 48 89 95 58 03 00 00        	movq	%rdx, 0x358(%rbp)
1403b82c3: 89 0a                       	movl	%ecx, (%rdx)
1403b82c5: 48 8b 00                    	movq	(%rax), %rax
1403b82c8: 48 89 85 f8 06 00 00        	movq	%rax, 0x6f8(%rbp)
1403b82cf: c6 80 44 03 00 00 01        	movb	$0x1, 0x344(%rax)
1403b82d6: f3 0f 10 35 56 4d 61 02     	movss	0x2614d56(%rip), %xmm6  # 0x1429cd034
1403b82de: f3 0f 59 b5 24 07 00 00     	mulss	0x724(%rbp), %xmm6
1403b82e6: 48 8b 85 b0 08 00 00        	movq	0x8b0(%rbp), %rax
1403b82ed: 48 8b 38                    	movq	(%rax), %rdi
1403b82f0: f3 0f 10 bf 50 01 00 00     	movss	0x150(%rdi), %xmm7
1403b82f8: f3 0f 5e fe                 	divss	%xmm6, %xmm7
1403b82fc: 66 0f 7e f8                 	movd	%xmm7, %eax
1403b8300: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
1403b8305: 3d 00 00 80 7f              	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
1403b830a: 7c 03                       	jl	0x1403b830f <.text+0x3b730f>
1403b830c: 0f 57 ff                    	xorps	%xmm7, %xmm7
1403b830f: f3 44 0f 10 87 54 01 00 00  	movss	0x154(%rdi), %xmm8
1403b8318: 49 8b 76 10                 	movq	0x10(%r14), %rsi
1403b831c: 48 85 f6                    	testq	%rsi, %rsi
1403b831f: 48 89 bd 68 03 00 00        	movq	%rdi, 0x368(%rbp)
1403b8326: 74 39                       	je	0x1403b8361 <.text+0x3b7361>
1403b8328: 0f b6 05 da c8 27 03        	movzbl	0x327c8da(%rip), %eax   # 0x143634c09
1403b832f: ba 01 00 00 00              	movl	$0x1, %edx
1403b8334: 48 89 f1                    	movq	%rsi, %rcx
1403b8337: e8 04 b9 25 00              	callq	0x140613c40 <.text+0x612c40>
1403b833c: 48 89 85 18 07 00 00        	movq	%rax, 0x718(%rbp)
1403b8343: 48 85 c0                    	testq	%rax, %rax
1403b8346: 75 25                       	jne	0x1403b836d <.text+0x3b736d>
1403b8348: 4c 8d 05 f1 55 61 02        	leaq	0x26155f1(%rip), %r8    # 0x1429cd940
1403b834f: b9 01 00 00 00              	movl	$0x1, %ecx
1403b8354: 48 89 f2                    	movq	%rsi, %rdx
1403b8357: e8 87 0c 52 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403b835c: e9 b6 45 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403b8361: b8 01 00 00 00              	movl	$0x1, %eax
1403b8366: 48 89 85 18 07 00 00        	movq	%rax, 0x718(%rbp)
1403b836d: 48 89 b5 38 07 00 00        	movq	%rsi, 0x738(%rbp)
1403b8374: 48 8b b5 00 09 00 00        	movq	0x900(%rbp), %rsi
1403b837b: 48 8b bd f8 08 00 00        	movq	0x8f8(%rbp), %rdi
1403b8382: 48 8b 9d d8 08 00 00        	movq	0x8d8(%rbp), %rbx
1403b8389: f3 0f 10 05 53 1d 5a 02     	movss	0x25a1d53(%rip), %xmm0  # 0x14295a0e4
1403b8391: f3 0f 5f c7                 	maxss	%xmm7, %xmm0
1403b8395: f3 44 0f 10 0d 66 d5 56 02  	movss	0x256d566(%rip), %xmm9  # 0x142925904
1403b839e: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1403b83a2: f3 0f 5d c8                 	minss	%xmm0, %xmm1
1403b83a6: 0f 29 8d a0 00 00 00        	movaps	%xmm1, 0xa0(%rbp)
1403b83ad: 0f 28 3d 3c d5 56 02        	movaps	0x256d53c(%rip), %xmm7  # 0x1429258f0
1403b83b4: 44 0f 57 c7                 	xorps	%xmm7, %xmm8
1403b83b8: f3 44 0f 5e c6              	divss	%xmm6, %xmm8
1403b83bd: 66 44 0f 7e c0              	movd	%xmm8, %eax
1403b83c2: 25 ff ff ff 7f              	andl	$0x7fffffff, %eax       # imm = 0x7FFFFFFF
1403b83c7: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
1403b83cb: 3d 00 00 80 7f              	cmpl	$0x7f800000, %eax       # imm = 0x7F800000
1403b83d0: 7c 04                       	jl	0x1403b83d6 <.text+0x3b73d6>
1403b83d2: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
1403b83d6: f3 0f 10 05 06 1d 5a 02     	movss	0x25a1d06(%rip), %xmm0  # 0x14295a0e4
1403b83de: f3 41 0f 5f c0              	maxss	%xmm8, %xmm0
1403b83e3: 41 0f 28 f1                 	movaps	%xmm9, %xmm6
1403b83e7: f3 0f 5d f0                 	minss	%xmm0, %xmm6
1403b83eb: f3 41 0f 10 46 48           	movss	0x48(%r14), %xmm0
1403b83f1: f3 0f 58 85 24 07 00 00     	addss	0x724(%rbp), %xmm0
1403b83f9: f3 41 0f 11 46 48           	movss	%xmm0, 0x48(%r14)
1403b83ff: 49 8d 46 18                 	leaq	0x18(%r14), %rax
1403b8403: 48 89 85 60 07 00 00        	movq	%rax, 0x760(%rbp)
1403b840a: 0f 28 c6                    	movaps	%xmm6, %xmm0
1403b840d: f3 41 0f 5f c5              	maxss	%xmm13, %xmm0
1403b8412: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
1403b8416: 0f 57 f7                    	xorps	%xmm7, %xmm6
1403b8419: f3 41 0f 5f f5              	maxss	%xmm13, %xmm6
1403b841e: 0f 28 c6                    	movaps	%xmm6, %xmm0
1403b8421: f3 0f 58 c6                 	addss	%xmm6, %xmm0
1403b8425: f3 0f 58 05 47 82 5a 02     	addss	0x25a8247(%rip), %xmm0  # 0x142960674
1403b842d: f3 0f 59 05 03 4c 61 02     	mulss	0x2614c03(%rip), %xmm0  # 0x1429cd038
1403b8435: e8 86 ba 56 02              	callq	0x142923ec0 <.text+0x2922ec0>
1403b843a: f3 0f 11 85 e4 05 00 00     	movss	%xmm0, 0x5e4(%rbp)
1403b8442: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1403b8446: f3 0f 5c ce                 	subss	%xmm6, %xmm1
1403b844a: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1403b844e: 48 8b 85 28 06 00 00        	movq	0x628(%rbp), %rax
1403b8455: 48 83 c0 18                 	addq	$0x18, %rax
1403b8459: 48 89 85 10 03 00 00        	movq	%rax, 0x310(%rbp)
1403b8460: 48 8b 85 d0 08 00 00        	movq	0x8d0(%rbp), %rax
1403b8467: 48 8b 08                    	movq	(%rax), %rcx
1403b846a: 48 89 8d 38 03 00 00        	movq	%rcx, 0x338(%rbp)
1403b8471: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1403b8475: 48 89 8d 20 03 00 00        	movq	%rcx, 0x320(%rbp)
1403b847c: 0f 28 85 a0 00 00 00        	movaps	0xa0(%rbp), %xmm0
1403b8483: 0f 54 f8                    	andps	%xmm0, %xmm7
1403b8486: 0f 28 d0                    	movaps	%xmm0, %xmm2
1403b8489: 0f 14 d7                    	unpcklps	%xmm7, %xmm2            # xmm2 = xmm2[0],xmm7[0],xmm2[1],xmm7[1]
1403b848c: 0f 59 15 ad 4b 61 02        	mulps	0x2614bad(%rip), %xmm2  # 0x1429cd040
1403b8493: 8b 40 14                    	movl	0x14(%rax), %eax
1403b8496: 89 85 d4 05 00 00           	movl	%eax, 0x5d4(%rbp)
1403b849c: 0f 59 15 ad 4b 61 02        	mulps	0x2614bad(%rip), %xmm2  # 0x1429cd050
1403b84a3: 0f 29 55 20                 	movaps	%xmm2, 0x20(%rbp)
1403b84a7: f3 0f 16 d2                 	movshdup	%xmm2, %xmm2            # xmm2 = xmm2[1,1,3,3]
1403b84ab: 0f 29 55 30                 	movaps	%xmm2, 0x30(%rbp)
1403b84af: 48 8b 03                    	movq	(%rbx), %rax
1403b84b2: 48 89 85 18 06 00 00        	movq	%rax, 0x618(%rbp)
1403b84b9: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403b84bd: 48 89 85 08 06 00 00        	movq	%rax, 0x608(%rbp)
1403b84c4: 48 8b 85 c8 08 00 00        	movq	0x8c8(%rbp), %rax
1403b84cb: 48 8b 08                    	movq	(%rax), %rcx
1403b84ce: 48 89 8d 88 07 00 00        	movq	%rcx, 0x788(%rbp)
1403b84d5: 8b 4f 1c                    	movl	0x1c(%rdi), %ecx
1403b84d8: 89 8d d8 05 00 00           	movl	%ecx, 0x5d8(%rbp)
1403b84de: 48 8b 0f                    	movq	(%rdi), %rcx
1403b84e1: 48 8b 57 10                 	movq	0x10(%rdi), %rdx
1403b84e5: 48 89 95 90 02 00 00        	movq	%rdx, 0x290(%rbp)
1403b84ec: 8b 56 1c                    	movl	0x1c(%rsi), %edx
1403b84ef: 89 95 dc 05 00 00           	movl	%edx, 0x5dc(%rbp)
1403b84f5: 48 8b 16                    	movq	(%rsi), %rdx
1403b84f8: 4c 8b 46 10                 	movq	0x10(%rsi), %r8
1403b84fc: 4c 89 85 98 02 00 00        	movq	%r8, 0x298(%rbp)
1403b8503: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1403b8507: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
1403b850b: 4c 8b 68 08                 	movq	0x8(%rax), %r13
1403b850f: 8b 40 14                    	movl	0x14(%rax), %eax
1403b8512: 89 85 28 07 00 00           	movl	%eax, 0x728(%rbp)
1403b8518: 48 89 8d 28 03 00 00        	movq	%rcx, 0x328(%rbp)
1403b851f: 48 8d 41 20                 	leaq	0x20(%rcx), %rax
1403b8523: 48 89 85 a0 02 00 00        	movq	%rax, 0x2a0(%rbp)
1403b852a: 48 89 95 30 03 00 00        	movq	%rdx, 0x330(%rbp)
1403b8531: 48 8d 42 20                 	leaq	0x20(%rdx), %rax
1403b8535: 48 89 85 a8 02 00 00        	movq	%rax, 0x2a8(%rbp)
1403b853c: 48 c7 85 40 03 00 00 00 00 00 00    	movq	$0x0, 0x340(%rbp)
1403b8547: 48 c7 85 10 06 00 00 00 00 00 00    	movq	$0x0, 0x610(%rbp)
1403b8552: 44 0f 28 25 56 61 57 02     	movaps	0x2576156(%rip), %xmm12 # 0x14292e6b0
1403b855a: f3 0f 11 8d e0 05 00 00     	movss	%xmm1, 0x5e0(%rbp)
1403b8562: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1403b8570: 48 81 bd 40 03 00 00 e8 03 00 00    	cmpq	$0x3e8, 0x340(%rbp) # imm = 0x3E8
1403b857b: 0f 83 1e 15 00 00           	jae	0x1403b9a9f <.text+0x3b8a9f>
1403b8581: 4c 8b 85 80 07 00 00        	movq	0x780(%rbp), %r8
1403b8588: f3 41 0f 10 40 48           	movss	0x48(%r8), %xmm0
1403b858e: f3 0f 10 15 ca 4a 61 02     	movss	0x2614aca(%rip), %xmm2  # 0x1429cd060
1403b8596: 0f 2e d0                    	ucomiss	%xmm0, %xmm2
1403b8599: 4d 8b 60 38                 	movq	0x38(%r8), %r12
1403b859d: 0f 87 07 15 00 00           	ja	0x1403b9aaa <.text+0x3b8aaa>
1403b85a3: f3 0f 58 05 8d 4a 61 02     	addss	0x2614a8d(%rip), %xmm0  # 0x1429cd038
1403b85ab: f3 41 0f 11 40 48           	movss	%xmm0, 0x48(%r8)
1403b85b1: 4d 85 e4                    	testq	%r12, %r12
1403b85b4: 78 0a                       	js	0x1403b85c0 <.text+0x3b75c0>
1403b85b6: 0f 57 d2                    	xorps	%xmm2, %xmm2
1403b85b9: f3 49 0f 2a d4              	cvtsi2ss	%r12, %xmm2
1403b85be: eb 19                       	jmp	0x1403b85d9 <.text+0x3b75d9>
1403b85c0: 4c 89 e0                    	movq	%r12, %rax
1403b85c3: 48 d1 e8                    	shrq	%rax
1403b85c6: 41 83 e4 01                 	andl	$0x1, %r12d
1403b85ca: 49 09 c4                    	orq	%rax, %r12
1403b85cd: 0f 57 d2                    	xorps	%xmm2, %xmm2
1403b85d0: f3 49 0f 2a d4              	cvtsi2ss	%r12, %xmm2
1403b85d5: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1403b85d9: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b85e0: f3 0f 10 40 40              	movss	0x40(%rax), %xmm0
1403b85e5: f3 0f 10 48 44              	movss	0x44(%rax), %xmm1
1403b85ea: 0f 29 8d 70 03 00 00        	movaps	%xmm1, 0x370(%rbp)
1403b85f1: f3 0f 59 15 6b 4a 61 02     	mulss	0x2614a6b(%rip), %xmm2  # 0x1429cd064
1403b85f9: f3 0f 11 85 00 06 00 00     	movss	%xmm0, 0x600(%rbp)
1403b8601: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1403b8605: 4c 8d a5 80 03 00 00        	leaq	0x380(%rbp), %r12
1403b860c: 4c 89 e1                    	movq	%r12, %rcx
1403b860f: 48 8b 95 60 07 00 00        	movq	0x760(%rbp), %rdx
1403b8616: e8 e5 c1 8d 00              	callq	0x140c94800 <.text+0xc93800>
1403b861b: f3 0f 10 bd 84 03 00 00     	movss	0x384(%rbp), %xmm7
1403b8623: 0f 28 f7                    	movaps	%xmm7, %xmm6
1403b8626: f3 0f 59 35 0a 4a 61 02     	mulss	0x2614a0a(%rip), %xmm6  # 0x1429cd038
1403b862e: f3 0f 59 35 72 bf 57 02     	mulss	0x257bf72(%rip), %xmm6  # 0x1429345a8
1403b8636: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b863d: f3 0f 58 70 44              	addss	0x44(%rax), %xmm6
1403b8642: 0f 28 c6                    	movaps	%xmm6, %xmm0
1403b8645: f3 0f 10 0d a3 60 57 02     	movss	0x25760a3(%rip), %xmm1  # 0x14292e6f0
1403b864d: f3 0f 5f c1                 	maxss	%xmm1, %xmm0
1403b8651: f3 0f 11 40 44              	movss	%xmm0, 0x44(%rax)
1403b8656: 48 8b 8d b0 02 00 00        	movq	0x2b0(%rbp), %rcx
1403b865d: e8 1e d0 50 00              	callq	0x1408c5680 <.text+0x8c4680>
1403b8662: 0f b6 c0                    	movzbl	%al, %eax
1403b8665: f3 0f 2a d8                 	cvtsi2ss	%eax, %xmm3
1403b8669: f3 0f 58 5d 50              	addss	0x50(%rbp), %xmm3
1403b866e: 41 0f 2e dd                 	ucomiss	%xmm13, %xmm3
1403b8672: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b8679: f3 0f 10 40 44              	movss	0x44(%rax), %xmm0
1403b867e: 76 30                       	jbe	0x1403b86b0 <.text+0x3b76b0>
1403b8680: f3 0f 10 0d e4 49 61 02     	movss	0x26149e4(%rip), %xmm1  # 0x1429cd06c
1403b8688: f3 0f 10 15 d0 49 61 02     	movss	0x26149d0(%rip), %xmm2  # 0x1429cd060
1403b8690: e8 4b 7e 8d 00              	callq	0x140c904e0 <.text+0xc8f4e0>
1403b8695: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b869c: f3 0f 11 40 44              	movss	%xmm0, 0x44(%rax)
1403b86a1: eb 14                       	jmp	0x1403b86b7 <.text+0x3b76b7>
1403b86a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1403b86b0: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b86b7: f3 0f 10 95 e0 05 00 00     	movss	0x5e0(%rbp), %xmm2
1403b86bf: f3 44 0f 10 1d a8 49 61 02  	movss	0x26149a8(%rip), %xmm11 # 0x1429cd070
1403b86c8: 44 0f 2e da                 	ucomiss	%xmm2, %xmm11
1403b86cc: f3 0f 59 85 e4 05 00 00     	mulss	0x5e4(%rbp), %xmm0
1403b86d4: f3 0f 11 40 44              	movss	%xmm0, 0x44(%rax)
1403b86d9: 0f 82 2a 2b 00 00           	jb	0x1403bb209 <.text+0x3ba209>
1403b86df: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
1403b86e3: f3 44 0f 5d da              	minss	%xmm2, %xmm11
1403b86e8: f3 44 0f 11 58 44           	movss	%xmm11, 0x44(%rax)
1403b86ee: 48 8b 40 10                 	movq	0x10(%rax), %rax
1403b86f2: 48 ff c8                    	decq	%rax
1403b86f5: 78 19                       	js	0x1403b8710 <.text+0x3b7710>
1403b86f7: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403b86fa: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
1403b86ff: eb 27                       	jmp	0x1403b8728 <.text+0x3b7728>
1403b8701: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1403b8710: 48 89 c1                    	movq	%rax, %rcx
1403b8713: 48 d1 e9                    	shrq	%rcx
1403b8716: 83 e0 01                    	andl	$0x1, %eax
1403b8719: 48 09 c8                    	orq	%rcx, %rax
1403b871c: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403b871f: f3 48 0f 2a c0              	cvtsi2ss	%rax, %xmm0
1403b8724: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1403b8728: f3 0f 59 05 34 49 61 02     	mulss	0x2614934(%rip), %xmm0  # 0x1429cd064
1403b8730: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b8737: f3 0f 58 40 40              	addss	0x40(%rax), %xmm0
1403b873c: f3 0f 10 48 34              	movss	0x34(%rax), %xmm1
1403b8741: 0f 28 d1                    	movaps	%xmm1, %xmm2
1403b8744: f3 0f 58 15 28 49 61 02     	addss	0x2614928(%rip), %xmm2  # 0x1429cd074
1403b874c: f3 0f 5c c2                 	subss	%xmm2, %xmm0
1403b8750: f3 0f 5c ca                 	subss	%xmm2, %xmm1
1403b8754: f3 0f 5e c1                 	divss	%xmm1, %xmm0
1403b8758: 0f 57 c9                    	xorps	%xmm1, %xmm1
1403b875b: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1403b875f: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1403b8763: f3 0f 5d c1                 	minss	%xmm1, %xmm0
1403b8767: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403b876a: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1403b876e: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1403b8772: f3 0f 10 15 fe 48 61 02     	movss	0x26148fe(%rip), %xmm2  # 0x1429cd078
1403b877a: f3 0f 5c d0                 	subss	%xmm0, %xmm2
1403b877e: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
1403b8782: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1403b8786: f3 0f 10 0d 62 5f 57 02     	movss	0x2575f62(%rip), %xmm1  # 0x14292e6f0
1403b878e: e8 3d 7d 8d 00              	callq	0x140c904d0 <.text+0xc8f4d0>
1403b8793: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403b8796: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b879d: f3 0f 10 58 44              	movss	0x44(%rax), %xmm3
1403b87a2: f3 0f 58 1d f2 be 57 02     	addss	0x257bef2(%rip), %xmm3  # 0x14293469c
1403b87aa: f3 41 0f 5f d9              	maxss	%xmm9, %xmm3
1403b87af: f3 0f 59 1d b9 d0 56 02     	mulss	0x256d0b9(%rip), %xmm3  # 0x142925870
1403b87b7: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1403b87bb: f3 0f 10 15 9d 48 61 02     	movss	0x261489d(%rip), %xmm2  # 0x1429cd060
1403b87c3: e8 18 7d 8d 00              	callq	0x140c904e0 <.text+0xc8f4e0>
1403b87c8: 0f 28 d0                    	movaps	%xmm0, %xmm2
1403b87cb: 48 8b 8d 80 07 00 00        	movq	0x780(%rbp), %rcx
1403b87d2: f3 0f 11 41 44              	movss	%xmm0, 0x44(%rcx)
1403b87d7: f3 0f 10 41 30              	movss	0x30(%rcx), %xmm0
1403b87dc: f3 0f 10 49 34              	movss	0x34(%rcx), %xmm1
1403b87e1: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1403b87e4: 0f 82 3b 2a 00 00           	jb	0x1403bb225 <.text+0x3ba225>
1403b87ea: 48 ff 85 40 03 00 00        	incq	0x340(%rbp)
1403b87f1: f3 0f 10 1d 6f 48 61 02     	movss	0x261486f(%rip), %xmm3  # 0x1429cd068
1403b87f9: 0f c2 df 01                 	cmpltps	%xmm7, %xmm3
1403b87fd: f3 0f 10 25 eb 5e 57 02     	movss	0x2575eeb(%rip), %xmm4  # 0x14292e6f0
1403b8805: 0f c2 f4 01                 	cmpltps	%xmm4, %xmm6
1403b8809: 0f 54 f3                    	andps	%xmm3, %xmm6
1403b880c: 66 0f 7e f0                 	movd	%xmm6, %eax
1403b8810: 48 8b 95 10 06 00 00        	movq	0x610(%rbp), %rdx
1403b8817: 08 c2                       	orb	%al, %dl
1403b8819: 48 89 95 10 06 00 00        	movq	%rdx, 0x610(%rbp)
1403b8820: f3 0f 59 15 38 48 61 02     	mulss	0x2614838(%rip), %xmm2  # 0x1429cd060
1403b8828: f3 0f 58 51 40              	addss	0x40(%rcx), %xmm2
1403b882d: f3 0f 5f c2                 	maxss	%xmm2, %xmm0
1403b8831: f3 0f 5d c8                 	minss	%xmm0, %xmm1
1403b8835: f3 0f 11 49 40              	movss	%xmm1, 0x40(%rcx)
1403b883a: 48 69 41 10 b0 01 00 00     	imulq	$0x1b0, 0x10(%rcx), %rax # imm = 0x1B0
1403b8842: 4c 8b 79 08                 	movq	0x8(%rcx), %r15
1403b8846: 4c 01 f8                    	addq	%r15, %rax
1403b8849: 48 89 85 f8 05 00 00        	movq	%rax, 0x5f8(%rbp)
1403b8850: f3 0f 12 85 70 03 00 00     	movsldup	0x370(%rbp), %xmm0      # xmm0 = mem[0,0,2,2]
1403b8858: 0f 29 85 d0 02 00 00        	movaps	%xmm0, 0x2d0(%rbp)
1403b885f: 31 c0                       	xorl	%eax, %eax
1403b8861: eb 2e                       	jmp	0x1403b8891 <.text+0x3b7891>
1403b8863: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1403b8870: 4c 8b b5 58 07 00 00        	movq	0x758(%rbp), %r14
1403b8877: 49 89 f7                    	movq	%rsi, %r15
1403b887a: f3 44 0f 10 0d 81 d0 56 02  	movss	0x256d081(%rip), %xmm9  # 0x142925904
1403b8883: 4c 8d a5 80 03 00 00        	leaq	0x380(%rbp), %r12
1403b888a: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403b8891: 4c 3b bd f8 05 00 00        	cmpq	0x5f8(%rbp), %r15
1403b8898: 0f 84 d2 fc ff ff           	je	0x1403b8570 <.text+0x3b7570>
1403b889e: 48 89 85 70 07 00 00        	movq	%rax, 0x770(%rbp)
1403b88a5: 41 8b b7 78 01 00 00        	movl	0x178(%r15), %esi
1403b88ac: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
1403b88b3: e8 c8 54 2c 00              	callq	0x14067dd80 <.text+0x67cd80>
1403b88b8: 48 89 c3                    	movq	%rax, %rbx
1403b88bb: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1403b88c0: 0f 84 fa 01 00 00           	je	0x1403b8ac0 <.text+0x3b7ac0>
1403b88c6: f3 0f 6f 43 20              	movdqu	0x20(%rbx), %xmm0
1403b88cb: 66 0f 70 c8 44              	pshufd	$0x44, %xmm0, %xmm1     # xmm1 = xmm0[0,1,0,1]
1403b88d0: 66 0f ef 0d a8 47 61 02     	pxor	0x26147a8(%rip), %xmm1  # 0x1429cd080
1403b88d8: 66 0f 7f 8d 80 03 00 00     	movdqa	%xmm1, 0x380(%rbp)
1403b88e0: 66 0f 70 c8 ee              	pshufd	$0xee, %xmm0, %xmm1     # xmm1 = xmm0[2,3,2,3]
1403b88e5: 66 0f ef 0d a3 47 61 02     	pxor	0x26147a3(%rip), %xmm1  # 0x1429cd090
1403b88ed: 66 0f 7f 8d 90 03 00 00     	movdqa	%xmm1, 0x390(%rbp)
1403b88f5: 66 0f 7f 85 a0 03 00 00     	movdqa	%xmm0, 0x3a0(%rbp)
1403b88fd: 48 8d 85 b0 03 00 00        	leaq	0x3b0(%rbp), %rax
1403b8904: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1403b8908: 66 0f 7f 00                 	movdqa	%xmm0, (%rax)
1403b890c: 48 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%rax)
1403b8914: 89 b5 b0 00 00 00           	movl	%esi, 0xb0(%rbp)
1403b891a: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403b8920: 4c 89 e1                    	movq	%r12, %rcx
1403b8923: 48 8d 95 b0 00 00 00        	leaq	0xb0(%rbp), %rdx
1403b892a: e8 31 d8 d4 ff              	callq	0x140106160 <.text+0x105160>
1403b892f: 4c 8b 8d b0 03 00 00        	movq	0x3b0(%rbp), %r9
1403b8936: 49 c1 e1 38                 	shlq	$0x38, %r9
1403b893a: 4c 0b 8d b8 03 00 00        	orq	0x3b8(%rbp), %r9
1403b8941: 48 8b 8d 90 03 00 00        	movq	0x390(%rbp), %rcx
1403b8948: 48 8b 85 98 03 00 00        	movq	0x398(%rbp), %rax
1403b894f: 4c 31 c8                    	xorq	%r9, %rax
1403b8952: 4c 8b 85 80 03 00 00        	movq	0x380(%rbp), %r8
1403b8959: 49 01 c8                    	addq	%rcx, %r8
1403b895c: 48 8b 95 88 03 00 00        	movq	0x388(%rbp), %rdx
1403b8963: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1403b8967: 48 01 c2                    	addq	%rax, %rdx
1403b896a: 4c 31 c1                    	xorq	%r8, %rcx
1403b896d: 48 c1 c0 10                 	rolq	$0x10, %rax
1403b8971: 48 31 d0                    	xorq	%rdx, %rax
1403b8974: 49 c1 c0 20                 	rolq	$0x20, %r8
1403b8978: 48 01 ca                    	addq	%rcx, %rdx
1403b897b: 49 01 c0                    	addq	%rax, %r8
1403b897e: 48 c1 c1 11                 	rolq	$0x11, %rcx
1403b8982: 48 31 d1                    	xorq	%rdx, %rcx
1403b8985: 48 c1 c0 15                 	rolq	$0x15, %rax
1403b8989: 48 c1 c2 20                 	rolq	$0x20, %rdx
1403b898d: 4c 31 c0                    	xorq	%r8, %rax
1403b8990: 4d 31 c8                    	xorq	%r9, %r8
1403b8993: 48 81 f2 ff 00 00 00        	xorq	$0xff, %rdx
1403b899a: 49 01 c8                    	addq	%rcx, %r8
1403b899d: 48 01 c2                    	addq	%rax, %rdx
1403b89a0: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1403b89a4: 4c 31 c1                    	xorq	%r8, %rcx
1403b89a7: 48 c1 c0 10                 	rolq	$0x10, %rax
1403b89ab: 49 c1 c0 20                 	rolq	$0x20, %r8
1403b89af: 48 31 d0                    	xorq	%rdx, %rax
1403b89b2: 48 01 ca                    	addq	%rcx, %rdx
1403b89b5: 48 c1 c1 11                 	rolq	$0x11, %rcx
1403b89b9: 49 01 c0                    	addq	%rax, %r8
1403b89bc: 48 31 d1                    	xorq	%rdx, %rcx
1403b89bf: 48 c1 c0 15                 	rolq	$0x15, %rax
1403b89c3: 4c 31 c0                    	xorq	%r8, %rax
1403b89c6: 48 c1 c2 20                 	rolq	$0x20, %rdx
1403b89ca: 49 01 c8                    	addq	%rcx, %r8
1403b89cd: 48 01 c2                    	addq	%rax, %rdx
1403b89d0: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1403b89d4: 4c 31 c1                    	xorq	%r8, %rcx
1403b89d7: 48 c1 c0 10                 	rolq	$0x10, %rax
1403b89db: 49 c1 c0 20                 	rolq	$0x20, %r8
1403b89df: 48 31 d0                    	xorq	%rdx, %rax
1403b89e2: 48 01 ca                    	addq	%rcx, %rdx
1403b89e5: 48 c1 c1 11                 	rolq	$0x11, %rcx
1403b89e9: 49 01 c0                    	addq	%rax, %r8
1403b89ec: 48 31 d1                    	xorq	%rdx, %rcx
1403b89ef: 48 c1 c0 15                 	rolq	$0x15, %rax
1403b89f3: 4c 31 c0                    	xorq	%r8, %rax
1403b89f6: 48 c1 c2 20                 	rolq	$0x20, %rdx
1403b89fa: 49 01 c8                    	addq	%rcx, %r8
1403b89fd: 48 01 c2                    	addq	%rax, %rdx
1403b8a00: 48 c1 c1 0d                 	rolq	$0xd, %rcx
1403b8a04: 4c 31 c1                    	xorq	%r8, %rcx
1403b8a07: 48 c1 c0 10                 	rolq	$0x10, %rax
1403b8a0b: 48 31 d0                    	xorq	%rdx, %rax
1403b8a0e: 48 01 ca                    	addq	%rcx, %rdx
1403b8a11: 48 c1 c1 11                 	rolq	$0x11, %rcx
1403b8a15: 48 c1 c0 15                 	rolq	$0x15, %rax
1403b8a19: 49 89 d0                    	movq	%rdx, %r8
1403b8a1c: 49 c1 c0 20                 	rolq	$0x20, %r8
1403b8a20: 49 31 c8                    	xorq	%rcx, %r8
1403b8a23: 49 31 c0                    	xorq	%rax, %r8
1403b8a26: 49 31 d0                    	xorq	%rdx, %r8
1403b8a29: 4c 89 c2                    	movq	%r8, %rdx
1403b8a2c: 48 c1 ea 39                 	shrq	$0x39, %rdx
1403b8a30: 48 8b 03                    	movq	(%rbx), %rax
1403b8a33: 48 8b 4b 08                 	movq	0x8(%rbx), %rcx
1403b8a37: 66 0f 6e c2                 	movd	%edx, %xmm0
1403b8a3b: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1403b8a3f: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1403b8a44: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1403b8a49: 48 8d 50 d8                 	leaq	-0x28(%rax), %rdx
1403b8a4d: 45 31 c9                    	xorl	%r9d, %r9d
1403b8a50: 49 21 c8                    	andq	%rcx, %r8
1403b8a53: f3 42 0f 6f 0c 00           	movdqu	(%rax,%r8), %xmm1
1403b8a59: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1403b8a5d: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1403b8a61: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1403b8a66: 45 85 d2                    	testl	%r10d, %r10d
1403b8a69: 74 25                       	je	0x1403b8a90 <.text+0x3b7a90>
1403b8a6b: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1403b8a70: 4d 01 c3                    	addq	%r8, %r11
1403b8a73: 49 21 cb                    	andq	%rcx, %r11
1403b8a76: 49 f7 db                    	negq	%r11
1403b8a79: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
1403b8a7d: 42 3b 34 da                 	cmpl	(%rdx,%r11,8), %esi
1403b8a81: 74 2d                       	je	0x1403b8ab0 <.text+0x3b7ab0>
1403b8a83: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1403b8a87: 66 45 21 d3                 	andw	%r10w, %r11w
1403b8a8b: 45 89 da                    	movl	%r11d, %r10d
1403b8a8e: 75 db                       	jne	0x1403b8a6b <.text+0x3b7a6b>
1403b8a90: 66 0f 74 0d f8 ce 56 02     	pcmpeqb	0x256cef8(%rip), %xmm1  # 0x142925990
1403b8a98: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
1403b8a9d: 45 85 d2                    	testl	%r10d, %r10d
1403b8aa0: 75 2e                       	jne	0x1403b8ad0 <.text+0x3b7ad0>
1403b8aa2: 4d 01 c8                    	addq	%r9, %r8
1403b8aa5: 49 83 c0 10                 	addq	$0x10, %r8
1403b8aa9: 49 83 c1 10                 	addq	$0x10, %r9
1403b8aad: eb a1                       	jmp	0x1403b8a50 <.text+0x3b7a50>
1403b8aaf: 90                          	nop
1403b8ab0: 4a 8d 0c d8                 	leaq	(%rax,%r11,8), %rcx
1403b8ab4: eb 1c                       	jmp	0x1403b8ad2 <.text+0x3b7ad2>
1403b8ab6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1403b8ac0: 31 c0                       	xorl	%eax, %eax
1403b8ac2: eb 19                       	jmp	0x1403b8add <.text+0x3b7add>
1403b8ac4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1403b8ad0: 31 c9                       	xorl	%ecx, %ecx
1403b8ad2: 48 8d 41 d8                 	leaq	-0x28(%rcx), %rax
1403b8ad6: 48 85 c9                    	testq	%rcx, %rcx
1403b8ad9: 48 0f 44 c1                 	cmoveq	%rcx, %rax
1403b8add: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
1403b8ae1: 48 85 c0                    	testq	%rax, %rax
1403b8ae4: 48 0f 44 c8                 	cmoveq	%rax, %rcx
1403b8ae8: 48 c7 85 80 03 00 00 01 00 00 00    	movq	$0x1, 0x380(%rbp)
1403b8af3: 48 89 8d 88 03 00 00        	movq	%rcx, 0x388(%rbp)
1403b8afa: 48 c7 85 90 03 00 00 00 00 00 00    	movq	$0x0, 0x390(%rbp)
1403b8b05: 48 c7 85 a0 03 00 00 00 00 00 00    	movq	$0x0, 0x3a0(%rbp)
1403b8b10: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1403b8b17: 4c 89 e2                    	movq	%r12, %rdx
1403b8b1a: 4c 8d 05 17 8d 64 02        	leaq	0x2648d17(%rip), %r8    # 0x142a01838
1403b8b21: e8 5a 76 f0 ff              	callq	0x1402c0180 <.text+0x2bf180>
1403b8b26: 49 8b 9f 70 01 00 00        	movq	0x170(%r15), %rbx
1403b8b2d: 48 89 d8                    	movq	%rbx, %rax
1403b8b30: 48 c1 e8 20                 	shrq	$0x20, %rax
1403b8b34: 48 8b b5 70 07 00 00        	movq	0x770(%rbp), %rsi
1403b8b3b: 75 43                       	jne	0x1403b8b80 <.text+0x3b7b80>
1403b8b3d: 4c 89 f3                    	movq	%r14, %rbx
1403b8b40: 48 c7 85 68 07 00 00 00 00 00 00    	movq	$0x0, 0x768(%rbp)
1403b8b4b: 4c 8b b5 80 07 00 00        	movq	0x780(%rbp), %r14
1403b8b52: 48 8b bd 60 07 00 00        	movq	0x760(%rbp), %rdi
1403b8b59: 48 89 9d 58 07 00 00        	movq	%rbx, 0x758(%rbp)
1403b8b60: 48 85 f6                    	testq	%rsi, %rsi
1403b8b63: 0f 88 fc 00 00 00           	js	0x1403b8c65 <.text+0x3b7c65>
1403b8b69: 45 0f 57 db                 	xorps	%xmm11, %xmm11
1403b8b6d: f3 4c 0f 2a de              	cvtsi2ss	%rsi, %xmm11
1403b8b72: e9 0a 01 00 00              	jmp	0x1403b8c81 <.text+0x3b7c81>
1403b8b77: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1403b8b80: 89 d9                       	movl	%ebx, %ecx
1403b8b82: 48 8b 95 20 03 00 00        	movq	0x320(%rbp), %rdx
1403b8b89: 48 3b 4a 10                 	cmpq	0x10(%rdx), %rcx
1403b8b8d: 48 8b bd 60 07 00 00        	movq	0x760(%rbp), %rdi
1403b8b94: 0f 83 a6 00 00 00           	jae	0x1403b8c40 <.text+0x3b7c40>
1403b8b9a: 48 8b 52 08                 	movq	0x8(%rdx), %rdx
1403b8b9e: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1403b8ba2: 39 04 8a                    	cmpl	%eax, (%rdx,%rcx,4)
1403b8ba5: 0f 85 95 00 00 00           	jne	0x1403b8c40 <.text+0x3b7c40>
1403b8bab: 48 8d 04 8a                 	leaq	(%rdx,%rcx,4), %rax
1403b8baf: 8b 48 04                    	movl	0x4(%rax), %ecx
1403b8bb2: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
1403b8bb7: 48 39 d1                    	cmpq	%rdx, %rcx
1403b8bba: 0f 84 80 00 00 00           	je	0x1403b8c40 <.text+0x3b7c40>
1403b8bc0: 4c 8b 9d 38 03 00 00        	movq	0x338(%rbp), %r11
1403b8bc7: 49 39 4b 38                 	cmpq	%rcx, 0x38(%r11)
1403b8bcb: 76 73                       	jbe	0x1403b8c40 <.text+0x3b7c40>
1403b8bcd: 49 8b 53 28                 	movq	0x28(%r11), %rdx
1403b8bd1: 41 89 c8                    	movl	%ecx, %r8d
1403b8bd4: 41 c1 e8 06                 	shrl	$0x6, %r8d
1403b8bd8: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
1403b8bdc: 48 0f a3 ca                 	btq	%rcx, %rdx
1403b8be0: 73 5e                       	jae	0x1403b8c40 <.text+0x3b7c40>
1403b8be2: 8b 48 0c                    	movl	0xc(%rax), %ecx
1403b8be5: 48 8b 95 20 03 00 00        	movq	0x320(%rbp), %rdx
1403b8bec: 4c 8b 8a a8 01 00 00        	movq	0x1a8(%rdx), %r9
1403b8bf3: 4c 8d 14 c9                 	leaq	(%rcx,%rcx,8), %r10
1403b8bf7: 4d 8b 83 18 01 00 00        	movq	0x118(%r11), %r8
1403b8bfe: 4b 8b 4c d1 18              	movq	0x18(%r9,%r10,8), %rcx
1403b8c03: 4b 8b 54 d1 38              	movq	0x38(%r9,%r10,8), %rdx
1403b8c08: 4f 39 44 d1 40              	cmpq	%r8, 0x40(%r9,%r10,8)
1403b8c0d: 0f 86 16 0e 00 00           	jbe	0x1403b9a29 <.text+0x3b8a29>
1403b8c13: 4e 8b 04 c2                 	movq	(%rdx,%r8,8), %r8
1403b8c17: 4d 85 c0                    	testq	%r8, %r8
1403b8c1a: 4c 8b b5 80 07 00 00        	movq	0x780(%rbp), %r14
1403b8c21: 0f 84 0e 0e 00 00           	je	0x1403b9a35 <.text+0x3b8a35>
1403b8c27: 49 f7 d0                    	notq	%r8
1403b8c2a: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
1403b8c2e: 49 c1 e0 04                 	shlq	$0x4, %r8
1403b8c32: 4e 8b 44 01 10              	movq	0x10(%rcx,%r8), %r8
1403b8c37: e9 fc 0d 00 00              	jmp	0x1403b9a38 <.text+0x3b8a38>
1403b8c3c: 0f 1f 40 00                 	nopl	(%rax)
1403b8c40: 4c 89 f3                    	movq	%r14, %rbx
1403b8c43: 48 c7 85 68 07 00 00 00 00 00 00    	movq	$0x0, 0x768(%rbp)
1403b8c4e: 4c 8b b5 80 07 00 00        	movq	0x780(%rbp), %r14
1403b8c55: 48 89 9d 58 07 00 00        	movq	%rbx, 0x758(%rbp)
1403b8c5c: 48 85 f6                    	testq	%rsi, %rsi
1403b8c5f: 0f 89 04 ff ff ff           	jns	0x1403b8b69 <.text+0x3b7b69>
1403b8c65: 48 89 f0                    	movq	%rsi, %rax
1403b8c68: 48 d1 e8                    	shrq	%rax
1403b8c6b: 89 f1                       	movl	%esi, %ecx
1403b8c6d: 83 e1 01                    	andl	$0x1, %ecx
1403b8c70: 48 09 c1                    	orq	%rax, %rcx
1403b8c73: 45 0f 57 db                 	xorps	%xmm11, %xmm11
1403b8c77: f3 4c 0f 2a d9              	cvtsi2ss	%rcx, %xmm11
1403b8c7c: f3 45 0f 58 db              	addss	%xmm11, %xmm11
1403b8c81: 49 8b 76 38                 	movq	0x38(%r14), %rsi
1403b8c85: f3 44 0f 59 1d d6 43 61 02  	mulss	0x26143d6(%rip), %xmm11 # 0x1429cd064
1403b8c8e: f3 45 0f 10 66 40           	movss	0x40(%r14), %xmm12
1403b8c94: f3 45 0f 58 e3              	addss	%xmm11, %xmm12
1403b8c99: 48 8d 8d 80 06 00 00        	leaq	0x680(%rbp), %rcx
1403b8ca0: 48 89 fa                    	movq	%rdi, %rdx
1403b8ca3: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1403b8ca7: e8 f4 b9 8d 00              	callq	0x140c946a0 <.text+0xc936a0>
1403b8cac: 48 8d 8d 3c 02 00 00        	leaq	0x23c(%rbp), %rcx
1403b8cb3: 48 89 fa                    	movq	%rdi, %rdx
1403b8cb6: 41 0f 28 d4                 	movaps	%xmm12, %xmm2
1403b8cba: e8 41 bb 8d 00              	callq	0x140c94800 <.text+0xc93800>
1403b8cbf: f3 44 0f 58 9d 00 06 00 00  	addss	0x600(%rbp), %xmm11
1403b8cc8: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
1403b8ccf: 48 89 fa                    	movq	%rdi, %rdx
1403b8cd2: 41 0f 28 d3                 	movaps	%xmm11, %xmm2
1403b8cd6: e8 25 bb 8d 00              	callq	0x140c94800 <.text+0xc93800>
1403b8cdb: f3 44 0f 10 a5 30 02 00 00  	movss	0x230(%rbp), %xmm12
1403b8ce4: f3 44 0f 59 a5 70 03 00 00  	mulss	0x370(%rbp), %xmm12
1403b8ced: f3 0f 10 9d 3c 02 00 00     	movss	0x23c(%rbp), %xmm3
1403b8cf5: f2 44 0f 10 b5 34 02 00 00  	movsd	0x234(%rbp), %xmm14
1403b8cfe: f2 0f 10 95 40 02 00 00     	movsd	0x240(%rbp), %xmm2
1403b8d06: 0f 28 c3                    	movaps	%xmm3, %xmm0
1403b8d09: f3 0f 59 c3                 	mulss	%xmm3, %xmm0
1403b8d0d: 0f 28 ca                    	movaps	%xmm2, %xmm1
1403b8d10: 0f 59 ca                    	mulps	%xmm2, %xmm1
1403b8d13: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1403b8d17: f3 0f 16 c9                 	movshdup	%xmm1, %xmm1            # xmm1 = xmm1[1,1,3,3]
1403b8d1b: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1403b8d1f: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403b8d22: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1403b8d26: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1403b8d2a: f3 0f 5e c8                 	divss	%xmm0, %xmm1
1403b8d2e: 0f 29 9d 00 07 00 00        	movaps	%xmm3, 0x700(%rbp)
1403b8d35: 0f 28 c3                    	movaps	%xmm3, %xmm0
1403b8d38: f3 0f 59 c1                 	mulss	%xmm1, %xmm0
1403b8d3c: f3 0f 11 85 80 03 00 00     	movss	%xmm0, 0x380(%rbp)
1403b8d44: f3 0f 12 c1                 	movsldup	%xmm1, %xmm0            # xmm0 = xmm1[0,0,2,2]
1403b8d48: 0f 29 95 40 07 00 00        	movaps	%xmm2, 0x740(%rbp)
1403b8d4f: 0f 59 c2                    	mulps	%xmm2, %xmm0
1403b8d52: 0f 13 85 84 03 00 00        	movlps	%xmm0, 0x384(%rbp)
1403b8d59: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403b8d60: 4c 89 e2                    	movq	%r12, %rdx
1403b8d63: e8 18 c0 f7 ff              	callq	0x140334d80 <.text+0x333d80>
1403b8d68: 44 0f 59 b5 d0 02 00 00     	mulps	0x2d0(%rbp), %xmm14
1403b8d70: f3 41 0f 10 4e 44           	movss	0x44(%r14), %xmm1
1403b8d76: 0f 28 9d 40 07 00 00        	movaps	0x740(%rbp), %xmm3
1403b8d7d: f3 0f 16 d3                 	movshdup	%xmm3, %xmm2            # xmm2 = xmm3[1,1,3,3]
1403b8d81: f3 0f 59 d1                 	mulss	%xmm1, %xmm2
1403b8d85: f3 41 0f 16 c6              	movshdup	%xmm14, %xmm0           # xmm0 = xmm14[1,1,3,3]
1403b8d8a: f3 0f 5c c2                 	subss	%xmm2, %xmm0
1403b8d8e: f3 0f 59 05 0a 43 61 02     	mulss	0x261430a(%rip), %xmm0  # 0x1429cd0a0
1403b8d96: f3 0f 10 35 c2 42 61 02     	movss	0x26142c2(%rip), %xmm6  # 0x1429cd060
1403b8d9e: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
1403b8da2: f3 0f 12 d3                 	movsldup	%xmm3, %xmm2            # xmm2 = xmm3[0,0,2,2]
1403b8da6: 0f 28 9d 00 07 00 00        	movaps	0x700(%rbp), %xmm3
1403b8dad: f3 0f 10 d3                 	movss	%xmm3, %xmm2            # xmm2 = xmm3[0],xmm2[1,2,3]
1403b8db1: f3 0f 12 c9                 	movsldup	%xmm1, %xmm1            # xmm1 = xmm1[0,0,2,2]
1403b8db5: 0f 59 ca                    	mulps	%xmm2, %xmm1
1403b8db8: f3 41 0f 12 d6              	movsldup	%xmm14, %xmm2           # xmm2 = xmm14[0,0,2,2]
1403b8dbd: f3 41 0f 10 d4              	movss	%xmm12, %xmm2           # xmm2 = xmm12[0],xmm2[1,2,3]
1403b8dc2: 0f 5c d1                    	subps	%xmm1, %xmm2
1403b8dc5: 0f 59 15 e4 42 61 02        	mulps	0x26142e4(%rip), %xmm2  # 0x1429cd0b0
1403b8dcc: 0f 28 3d 6d 42 61 02        	movaps	0x261426d(%rip), %xmm7  # 0x1429cd040
1403b8dd3: 0f 59 d7                    	mulps	%xmm7, %xmm2
1403b8dd6: f2 41 0f 10 8f 7c 01 00 00  	movsd	0x17c(%r15), %xmm1
1403b8ddf: 0f 58 ca                    	addps	%xmm2, %xmm1
1403b8de2: 41 0f 13 8f 7c 01 00 00     	movlps	%xmm1, 0x17c(%r15)
1403b8dea: f3 41 0f 58 87 84 01 00 00  	addss	0x184(%r15), %xmm0
1403b8df3: f3 41 0f 11 87 84 01 00 00  	movss	%xmm0, 0x184(%r15)
1403b8dfc: 48 39 b5 70 07 00 00        	cmpq	%rsi, 0x770(%rbp)
1403b8e03: 44 0f 28 1d 75 5b 57 02     	movaps	0x2575b75(%rip), %xmm11 # 0x14292e980
1403b8e0b: 0f 85 c5 00 00 00           	jne	0x1403b8ed6 <.text+0x3b7ed6>
1403b8e11: 0f 28 95 30 05 00 00        	movaps	0x530(%rbp), %xmm2
1403b8e18: 0f 28 e2                    	movaps	%xmm2, %xmm4
1403b8e1b: 0f 59 e2                    	mulps	%xmm2, %xmm4
1403b8e1e: f3 0f 16 dc                 	movshdup	%xmm4, %xmm3            # xmm3 = xmm4[1,1,3,3]
1403b8e22: f3 0f 58 dc                 	addss	%xmm4, %xmm3
1403b8e26: 0f 28 ec                    	movaps	%xmm4, %xmm5
1403b8e29: 66 0f 15 ec                 	unpckhpd	%xmm4, %xmm5            # xmm5 = xmm5[1],xmm4[1]
1403b8e2d: f3 0f 58 eb                 	addss	%xmm3, %xmm5
1403b8e31: 0f c6 ed 00                 	shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
1403b8e35: 0f c6 e4 ff                 	shufps	$0xff, %xmm4, %xmm4     # xmm4 = xmm4[3,3,3,3]
1403b8e39: 0f 5c e5                    	subps	%xmm5, %xmm4
1403b8e3c: f3 0f 10 2d c0 ca 56 02     	movss	0x256cac0(%rip), %xmm5  # 0x142925904
1403b8e44: 0f 59 e5                    	mulps	%xmm5, %xmm4
1403b8e47: 0f 28 da                    	movaps	%xmm2, %xmm3
1403b8e4a: 0f 59 dd                    	mulps	%xmm5, %xmm3
1403b8e4d: f3 0f 16 eb                 	movshdup	%xmm3, %xmm5            # xmm5 = xmm3[1,1,3,3]
1403b8e51: f3 0f 58 eb                 	addss	%xmm3, %xmm5
1403b8e55: 0f 12 db                    	movhlps	%xmm3, %xmm3            # xmm3 = xmm3[1,1]
1403b8e58: f3 0f 58 dd                 	addss	%xmm5, %xmm3
1403b8e5c: f3 0f 58 db                 	addss	%xmm3, %xmm3
1403b8e60: 0f c6 db 00                 	shufps	$0x0, %xmm3, %xmm3      # xmm3 = xmm3[0,0,0,0]
1403b8e64: 0f 59 da                    	mulps	%xmm2, %xmm3
1403b8e67: 0f 58 dc                    	addps	%xmm4, %xmm3
1403b8e6a: 0f 28 e2                    	movaps	%xmm2, %xmm4
1403b8e6d: 0f c6 e2 d2                 	shufps	$0xd2, %xmm2, %xmm4     # xmm4 = xmm4[2,0],xmm2[1,3]
1403b8e71: 0f 59 25 48 42 61 02        	mulps	0x2614248(%rip), %xmm4  # 0x1429cd0c0
1403b8e78: 0f 28 ea                    	movaps	%xmm2, %xmm5
1403b8e7b: 0f c6 ea c9                 	shufps	$0xc9, %xmm2, %xmm5     # xmm5 = xmm5[1,2],xmm2[0,3]
1403b8e7f: 0f 59 2d 2a b7 57 02        	mulps	0x257b72a(%rip), %xmm5  # 0x1429345b0
1403b8e86: 0f 5c ec                    	subps	%xmm4, %xmm5
1403b8e89: 0f 58 d2                    	addps	%xmm2, %xmm2
1403b8e8c: 0f c6 d2 ff                 	shufps	$0xff, %xmm2, %xmm2     # xmm2 = xmm2[3,3,3,3]
1403b8e90: 0f 59 d5                    	mulps	%xmm5, %xmm2
1403b8e93: 0f 58 d3                    	addps	%xmm3, %xmm2
1403b8e96: 0f 28 5d 40                 	movaps	0x40(%rbp), %xmm3
1403b8e9a: 0f 59 da                    	mulps	%xmm2, %xmm3
1403b8e9d: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1403b8ea0: f3 0f 59 95 a0 00 00 00     	mulss	0xa0(%rbp), %xmm2
1403b8ea8: f3 0f 59 d6                 	mulss	%xmm6, %xmm2
1403b8eac: f3 0f 59 15 dc b7 57 02     	mulss	0x257b7dc(%rip), %xmm2  # 0x142934690
1403b8eb4: 0f 59 df                    	mulps	%xmm7, %xmm3
1403b8eb7: 0f 59 1d 12 42 61 02        	mulps	0x2614212(%rip), %xmm3  # 0x1429cd0d0
1403b8ebe: 0f 58 cb                    	addps	%xmm3, %xmm1
1403b8ec1: 41 0f 13 8f 7c 01 00 00     	movlps	%xmm1, 0x17c(%r15)
1403b8ec9: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1403b8ecd: f3 41 0f 11 87 84 01 00 00  	movss	%xmm0, 0x184(%r15)
1403b8ed6: 44 0f 28 0d d2 57 57 02     	movaps	0x25757d2(%rip), %xmm9  # 0x14292e6b0
1403b8ede: 0f 57 ff                    	xorps	%xmm7, %xmm7
1403b8ee1: 4c 8b 85 68 07 00 00        	movq	0x768(%rbp), %r8
1403b8ee8: 0f 59 0d f1 41 61 02        	mulps	0x26141f1(%rip), %xmm1  # 0x1429cd0e0
1403b8eef: 41 0f 13 8f 7c 01 00 00     	movlps	%xmm1, 0x17c(%r15)
1403b8ef7: f3 0f 59 05 f1 41 61 02     	mulss	0x26141f1(%rip), %xmm0  # 0x1429cd0f0
1403b8eff: f3 41 0f 11 87 84 01 00 00  	movss	%xmm0, 0x184(%r15)
1403b8f08: f3 41 0f 10 87 88 01 00 00  	movss	0x188(%r15), %xmm0
1403b8f11: 0f 28 d0                    	movaps	%xmm0, %xmm2
1403b8f14: 41 0f 54 d3                 	andps	%xmm11, %xmm2
1403b8f18: 0f 2e 15 51 7e 59 02        	ucomiss	0x2597e51(%rip), %xmm2  # 0x142950d70
1403b8f1f: 73 0f                       	jae	0x1403b8f30 <.text+0x3b7f30>
1403b8f21: 31 c0                       	xorl	%eax, %eax
1403b8f23: eb 1e                       	jmp	0x1403b8f43 <.text+0x3b7f43>
1403b8f25: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403b8f30: f3 41 0f 10 8f a0 01 00 00  	movss	0x1a0(%r15), %xmm1
1403b8f39: 0f 2e 0d b4 41 61 02        	ucomiss	0x26141b4(%rip), %xmm1  # 0x1429cd0f4
1403b8f40: 0f 97 c0                    	seta	%al
1403b8f43: 48 8b 95 38 07 00 00        	movq	0x738(%rbp), %rdx
1403b8f4a: f3 0f 10 0d a6 41 61 02     	movss	0x26141a6(%rip), %xmm1  # 0x1429cd0f8
1403b8f52: 0f 28 d9                    	movaps	%xmm1, %xmm3
1403b8f55: 4d 85 c0                    	testq	%r8, %r8
1403b8f58: 74 26                       	je	0x1403b8f80 <.text+0x3b7f80>
1403b8f5a: 48 8b 8d 18 03 00 00        	movq	0x318(%rbp), %rcx
1403b8f61: f3 0f 10 59 3c              	movss	0x3c(%rcx), %xmm3
1403b8f66: f3 0f 58 db                 	addss	%xmm3, %xmm3
1403b8f6a: f3 0f 59 59 40              	mulss	0x40(%rcx), %xmm3
1403b8f6f: f3 0f 59 59 44              	mulss	0x44(%rcx), %xmm3
1403b8f74: f3 0f 5e 1d 80 41 61 02     	divss	0x2614180(%rip), %xmm3  # 0x1429cd0fc
1403b8f7c: f3 0f 59 d9                 	mulss	%xmm1, %xmm3
1403b8f80: f3 41 0f 10 af 8c 01 00 00  	movss	0x18c(%r15), %xmm5
1403b8f89: f3 41 0f 10 8f 90 01 00 00  	movss	0x190(%r15), %xmm1
1403b8f92: 0f 14 c5                    	unpcklps	%xmm5, %xmm0            # xmm0 = xmm0[0],xmm5[0],xmm0[1],xmm5[1]
1403b8f95: 4c 8b 8d 70 07 00 00        	movq	0x770(%rbp), %r9
1403b8f9c: 49 39 f1                    	cmpq	%rsi, %r9
1403b8f9f: 75 7f                       	jne	0x1403b9020 <.text+0x3b8020>
1403b8fa1: 41 0f 54 eb                 	andps	%xmm11, %xmm5
1403b8fa5: 0f 28 f1                    	movaps	%xmm1, %xmm6
1403b8fa8: 41 0f 54 f3                 	andps	%xmm11, %xmm6
1403b8fac: f3 44 0f 10 05 4f c9 56 02  	movss	0x256c94f(%rip), %xmm8  # 0x142925904
1403b8fb5: 41 0f 28 e0                 	movaps	%xmm8, %xmm4
1403b8fb9: f3 0f 5c e6                 	subss	%xmm6, %xmm4
1403b8fbd: f3 0f 5f e7                 	maxss	%xmm7, %xmm4
1403b8fc1: f3 41 0f 5d e0              	minss	%xmm8, %xmm4
1403b8fc6: f3 0f 59 65 30              	mulss	0x30(%rbp), %xmm4
1403b8fcb: 0f 14 d5                    	unpcklps	%xmm5, %xmm2            # xmm2 = xmm2[0],xmm5[0],xmm2[1],xmm5[1]
1403b8fce: 41 0f 28 e9                 	movaps	%xmm9, %xmm5
1403b8fd2: 0f 5c ea                    	subps	%xmm2, %xmm5
1403b8fd5: 0f 57 d2                    	xorps	%xmm2, %xmm2
1403b8fd8: 0f 5f d5                    	maxps	%xmm5, %xmm2
1403b8fdb: 0f c2 ed 07                 	cmpordps	%xmm5, %xmm5
1403b8fdf: 0f 54 ea                    	andps	%xmm2, %xmm5
1403b8fe2: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
1403b8fe6: 0f 5d d5                    	minps	%xmm5, %xmm2
1403b8fe9: 0f c2 ed 03                 	cmpunordps	%xmm5, %xmm5
1403b8fed: 41 0f 28 f1                 	movaps	%xmm9, %xmm6
1403b8ff1: 0f 54 f5                    	andps	%xmm5, %xmm6
1403b8ff4: 0f 55 ea                    	andnps	%xmm2, %xmm5
1403b8ff7: 0f 56 ee                    	orps	%xmm6, %xmm5
1403b8ffa: 0f 59 6d 20                 	mulps	0x20(%rbp), %xmm5
1403b8ffe: 0f 58 c5                    	addps	%xmm5, %xmm0
1403b9001: 41 0f 13 87 88 01 00 00     	movlps	%xmm0, 0x188(%r15)
1403b9009: f3 0f 58 cc                 	addss	%xmm4, %xmm1
1403b900d: f3 41 0f 11 8f 90 01 00 00  	movss	%xmm1, 0x190(%r15)
1403b9016: 0f 28 d0                    	movaps	%xmm0, %xmm2
1403b9019: 41 0f 54 d3                 	andps	%xmm11, %xmm2
1403b901d: eb 0a                       	jmp	0x1403b9029 <.text+0x3b8029>
1403b901f: 90                          	nop
1403b9020: f3 44 0f 10 05 db c8 56 02  	movss	0x256c8db(%rip), %xmm8  # 0x142925904
1403b9029: 44 0f 28 ad 30 05 00 00     	movaps	0x530(%rbp), %xmm13
1403b9031: 41 0f 28 fd                 	movaps	%xmm13, %xmm7
1403b9035: 0f 57 3d c4 40 61 02        	xorps	0x26140c4(%rip), %xmm7  # 0x1429cd100
1403b903c: f3 41 0f 10 6e 44           	movss	0x44(%r14), %xmm5
1403b9042: 0f 28 b5 00 07 00 00        	movaps	0x700(%rbp), %xmm6
1403b9049: f3 0f 59 f5                 	mulss	%xmm5, %xmm6
1403b904d: 41 0f 28 e4                 	movaps	%xmm12, %xmm4
1403b9051: f3 0f 5c e6                 	subss	%xmm6, %xmm4
1403b9055: f3 0f 59 e3                 	mulss	%xmm3, %xmm4
1403b9059: f3 0f 59 25 ff 3f 61 02     	mulss	0x2613fff(%rip), %xmm4  # 0x1429cd060
1403b9061: f3 0f 12 ed                 	movsldup	%xmm5, %xmm5            # xmm5 = xmm5[0,0,2,2]
1403b9065: 0f 59 ad 40 07 00 00        	mulps	0x740(%rbp), %xmm5
1403b906c: 41 0f 28 f6                 	movaps	%xmm14, %xmm6
1403b9070: 0f 5c f5                    	subps	%xmm5, %xmm6
1403b9073: f3 0f 12 eb                 	movsldup	%xmm3, %xmm5            # xmm5 = xmm3[0,0,2,2]
1403b9077: 0f 59 ee                    	mulps	%xmm6, %xmm5
1403b907a: 0f 59 2d bf 3f 61 02        	mulps	0x2613fbf(%rip), %xmm5  # 0x1429cd040
1403b9081: 0f 16 e5                    	movlhps	%xmm5, %xmm4            # xmm4 = xmm4[0],xmm5[0]
1403b9084: 0f 28 dc                    	movaps	%xmm4, %xmm3
1403b9087: 0f c6 dd d8                 	shufps	$0xd8, %xmm5, %xmm3     # xmm3 = xmm3[0,2],xmm5[1,3]
1403b908b: 0f c6 e5 58                 	shufps	$0x58, %xmm5, %xmm4     # xmm4 = xmm4[0,2],xmm5[1,1]
1403b908f: 0f 28 f7                    	movaps	%xmm7, %xmm6
1403b9092: 0f 59 f7                    	mulps	%xmm7, %xmm6
1403b9095: f3 0f 16 ee                 	movshdup	%xmm6, %xmm5            # xmm5 = xmm6[1,1,3,3]
1403b9099: f3 0f 58 ee                 	addss	%xmm6, %xmm5
1403b909d: 44 0f 28 d6                 	movaps	%xmm6, %xmm10
1403b90a1: 66 44 0f 15 d6              	unpckhpd	%xmm6, %xmm10           # xmm10 = xmm10[1],xmm6[1]
1403b90a6: f3 44 0f 58 d5              	addss	%xmm5, %xmm10
1403b90ab: 45 0f c6 d2 00              	shufps	$0x0, %xmm10, %xmm10    # xmm10 = xmm10[0,0,0,0]
1403b90b0: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1403b90b4: 41 0f 5c f2                 	subps	%xmm10, %xmm6
1403b90b8: 0f 59 e6                    	mulps	%xmm6, %xmm4
1403b90bb: 0f 28 eb                    	movaps	%xmm3, %xmm5
1403b90be: 0f 59 ef                    	mulps	%xmm7, %xmm5
1403b90c1: f3 44 0f 16 d5              	movshdup	%xmm5, %xmm10           # xmm10 = xmm5[1,1,3,3]
1403b90c6: f3 44 0f 58 d5              	addss	%xmm5, %xmm10
1403b90cb: 0f 12 ed                    	movhlps	%xmm5, %xmm5            # xmm5 = xmm5[1,1]
1403b90ce: f3 41 0f 58 ea              	addss	%xmm10, %xmm5
1403b90d3: f3 0f 58 ed                 	addss	%xmm5, %xmm5
1403b90d7: 0f c6 ed 00                 	shufps	$0x0, %xmm5, %xmm5      # xmm5 = xmm5[0,0,0,0]
1403b90db: 0f 59 ef                    	mulps	%xmm7, %xmm5
1403b90de: 0f 58 ec                    	addps	%xmm4, %xmm5
1403b90e1: 66 44 0f 70 ff d2           	pshufd	$0xd2, %xmm7, %xmm15    # xmm15 = xmm7[2,0,1,3]
1403b90e7: 66 41 0f 6f e7              	movdqa	%xmm15, %xmm4
1403b90ec: 0f 59 e3                    	mulps	%xmm3, %xmm4
1403b90ef: 0f c6 db d2                 	shufps	$0xd2, %xmm3, %xmm3     # xmm3 = xmm3[2,0,1,3]
1403b90f3: 0f 59 df                    	mulps	%xmm7, %xmm3
1403b90f6: 0f 5c e3                    	subps	%xmm3, %xmm4
1403b90f9: 0f c6 e4 d2                 	shufps	$0xd2, %xmm4, %xmm4     # xmm4 = xmm4[2,0,1,3]
1403b90fd: 45 0f 28 d5                 	movaps	%xmm13, %xmm10
1403b9101: 45 0f 58 d5                 	addps	%xmm13, %xmm10
1403b9105: 45 0f c6 d2 ff              	shufps	$0xff, %xmm10, %xmm10   # xmm10 = xmm10[3,3,3,3]
1403b910a: 41 0f 59 e2                 	mulps	%xmm10, %xmm4
1403b910e: 0f 58 e5                    	addps	%xmm5, %xmm4
1403b9111: f3 0f 16 d8                 	movshdup	%xmm0, %xmm3            # xmm3 = xmm0[1,1,3,3]
1403b9115: 41 0f 54 db                 	andps	%xmm11, %xmm3
1403b9119: 0f 14 d3                    	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
1403b911c: 0f 28 d9                    	movaps	%xmm1, %xmm3
1403b911f: 41 0f 54 db                 	andps	%xmm11, %xmm3
1403b9123: 41 0f 28 e8                 	movaps	%xmm8, %xmm5
1403b9127: f3 0f 5c eb                 	subss	%xmm3, %xmm5
1403b912b: 41 0f 28 d9                 	movaps	%xmm9, %xmm3
1403b912f: 0f 5c da                    	subps	%xmm2, %xmm3
1403b9132: 0f 57 d2                    	xorps	%xmm2, %xmm2
1403b9135: 0f 5f d3                    	maxps	%xmm3, %xmm2
1403b9138: 0f c2 db 07                 	cmpordps	%xmm3, %xmm3
1403b913c: 0f 54 da                    	andps	%xmm2, %xmm3
1403b913f: 41 0f 28 d1                 	movaps	%xmm9, %xmm2
1403b9143: 0f 5d d3                    	minps	%xmm3, %xmm2
1403b9146: 0f c2 db 03                 	cmpunordps	%xmm3, %xmm3
1403b914a: 45 0f 28 d9                 	movaps	%xmm9, %xmm11
1403b914e: 44 0f 54 db                 	andps	%xmm3, %xmm11
1403b9152: 0f 55 da                    	andnps	%xmm2, %xmm3
1403b9155: 41 0f 56 db                 	orps	%xmm11, %xmm3
1403b9159: 44 0f 28 1d 1f 58 57 02     	movaps	0x257581f(%rip), %xmm11 # 0x14292e980
1403b9161: f3 0f 5f 2d 27 75 5a 02     	maxss	0x25a7527(%rip), %xmm5  # 0x142960690
1403b9169: f3 41 0f 5d e8              	minss	%xmm8, %xmm5
1403b916e: 0f 59 dc                    	mulps	%xmm4, %xmm3
1403b9171: 0f 12 e4                    	movhlps	%xmm4, %xmm4            # xmm4 = xmm4[1,1]
1403b9174: f3 0f 59 e5                 	mulss	%xmm5, %xmm4
1403b9178: f3 0f 58 e1                 	addss	%xmm1, %xmm4
1403b917c: f3 0f 5f 25 60 0f 5a 02     	maxss	0x25a0f60(%rip), %xmm4  # 0x14295a0e4
1403b9184: f3 41 0f 5d e0              	minss	%xmm8, %xmm4
1403b9189: f3 41 0f 11 a7 90 01 00 00  	movss	%xmm4, 0x190(%r15)
1403b9192: 0f 58 d8                    	addps	%xmm0, %xmm3
1403b9195: 0f 28 0d 34 55 57 02        	movaps	0x2575534(%rip), %xmm1  # 0x14292e6d0
1403b919c: 0f 28 c1                    	movaps	%xmm1, %xmm0
1403b919f: 0f 5f c3                    	maxps	%xmm3, %xmm0
1403b91a2: 0f c2 db 03                 	cmpunordps	%xmm3, %xmm3
1403b91a6: 0f 54 cb                    	andps	%xmm3, %xmm1
1403b91a9: 0f 55 d8                    	andnps	%xmm0, %xmm3
1403b91ac: 0f 56 d9                    	orps	%xmm1, %xmm3
1403b91af: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1403b91b3: 0f 5d c3                    	minps	%xmm3, %xmm0
1403b91b6: 0f c2 db 03                 	cmpunordps	%xmm3, %xmm3
1403b91ba: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1403b91be: 0f 54 cb                    	andps	%xmm3, %xmm1
1403b91c1: 0f 55 d8                    	andnps	%xmm0, %xmm3
1403b91c4: 0f 56 d9                    	orps	%xmm1, %xmm3
1403b91c7: 41 0f 28 c3                 	movaps	%xmm11, %xmm0
1403b91cb: 0f 55 c3                    	andnps	%xmm3, %xmm0
1403b91ce: 45 0f 54 cb                 	andps	%xmm11, %xmm9
1403b91d2: 44 0f 56 c8                 	orps	%xmm0, %xmm9
1403b91d6: 0f 28 c3                    	movaps	%xmm3, %xmm0
1403b91d9: 41 0f 54 c3                 	andps	%xmm11, %xmm0
1403b91dd: 0f 28 25 2c 3f 61 02        	movaps	0x2613f2c(%rip), %xmm4  # 0x1429cd110
1403b91e4: 0f 28 d4                    	movaps	%xmm4, %xmm2
1403b91e7: 0f 5d d0                    	minps	%xmm0, %xmm2
1403b91ea: 0f c2 c0 03                 	cmpunordps	%xmm0, %xmm0
1403b91ee: 0f 54 e0                    	andps	%xmm0, %xmm4
1403b91f1: 0f 55 c2                    	andnps	%xmm2, %xmm0
1403b91f4: 0f 56 c4                    	orps	%xmm4, %xmm0
1403b91f7: 41 0f 59 c1                 	mulps	%xmm9, %xmm0
1403b91fb: 0f 5c d8                    	subps	%xmm0, %xmm3
1403b91fe: 41 0f 13 9f 88 01 00 00     	movlps	%xmm3, 0x188(%r15)
1403b9206: f3 41 0f 10 87 a0 01 00 00  	movss	0x1a0(%r15), %xmm0
1403b920f: 41 0f 54 db                 	andps	%xmm11, %xmm3
1403b9213: 0f 28 cb                    	movaps	%xmm3, %xmm1
1403b9216: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1403b921a: f3 0f c2 c0 03              	cmpunordss	%xmm0, %xmm0
1403b921f: 0f 28 d0                    	movaps	%xmm0, %xmm2
1403b9222: 0f 55 c1                    	andnps	%xmm1, %xmm0
1403b9225: 0f 54 d3                    	andps	%xmm3, %xmm2
1403b9228: 0f 56 c2                    	orps	%xmm2, %xmm0
1403b922b: f3 0f 59 05 bd 3e 61 02     	mulss	0x2613ebd(%rip), %xmm0  # 0x1429cd0f0
1403b9233: f3 41 0f 11 87 a0 01 00 00  	movss	%xmm0, 0x1a0(%r15)
1403b923c: 49 39 d1                    	cmpq	%rdx, %r9
1403b923f: 0f 83 f1 1f 00 00           	jae	0x1403bb236 <.text+0x3ba236>
1403b9245: f3 0f 10 05 23 7b 59 02     	movss	0x2597b23(%rip), %xmm0  # 0x142950d70
1403b924d: 0f 2e c3                    	ucomiss	%xmm3, %xmm0
1403b9250: 0f 97 c1                    	seta	%cl
1403b9253: 20 c8                       	andb	%cl, %al
1403b9255: 48 8b 8d 18 07 00 00        	movq	0x718(%rbp), %rcx
1403b925c: 42 08 04 09                 	orb	%al, (%rcx,%r9)
1403b9260: 4d 85 c0                    	testq	%r8, %r8
1403b9263: f3 44 0f 10 05 98 c6 56 02  	movss	0x256c698(%rip), %xmm8  # 0x142925904
1403b926c: 0f 84 3e 02 00 00           	je	0x1403b94b0 <.text+0x3b84b0>
1403b9272: f3 41 0f 10 87 7c 01 00 00  	movss	0x17c(%r15), %xmm0
1403b927b: f3 0f 59 c0                 	mulss	%xmm0, %xmm0
1403b927f: f2 41 0f 10 8f 80 01 00 00  	movsd	0x180(%r15), %xmm1
1403b9288: 0f 59 c9                    	mulps	%xmm1, %xmm1
1403b928b: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1403b928f: f3 0f 16 c9                 	movshdup	%xmm1, %xmm1            # xmm1 = xmm1[1,1,3,3]
1403b9293: f3 0f 58 c8                 	addss	%xmm0, %xmm1
1403b9297: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403b929a: f3 0f 51 c1                 	sqrtss	%xmm1, %xmm0
1403b929e: 0f 2e 05 7b 3e 61 02        	ucomiss	0x2613e7b(%rip), %xmm0  # 0x1429cd120
1403b92a5: 0f 86 05 02 00 00           	jbe	0x1403b94b0 <.text+0x3b84b0>
1403b92ab: e8 10 61 8f 00              	callq	0x140caf3c0 <.text+0xcae3c0>
1403b92b0: 0f 2e 05 6d 3e 61 02        	ucomiss	0x2613e6d(%rip), %xmm0  # 0x1429cd124
1403b92b7: 0f 86 f3 01 00 00           	jbe	0x1403b94b0 <.text+0x3b84b0>
1403b92bd: 48 8b 85 18 03 00 00        	movq	0x318(%rbp), %rax
1403b92c4: 48 8b 78 20                 	movq	0x20(%rax), %rdi
1403b92c8: 8b 85 d8 05 00 00           	movl	0x5d8(%rbp), %eax
1403b92ce: 48 8b 8d 90 02 00 00        	movq	0x290(%rbp), %rcx
1403b92d5: 89 01                       	movl	%eax, (%rcx)
1403b92d7: 48 8b 85 28 03 00 00        	movq	0x328(%rbp), %rax
1403b92de: 48 8b 70 30                 	movq	0x30(%rax), %rsi
1403b92e2: 48 8b 58 40                 	movq	0x40(%rax), %rbx
1403b92e6: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
1403b92ea: 75 13                       	jne	0x1403b92ff <.text+0x3b82ff>
1403b92ec: 48 8b 8d a0 02 00 00        	movq	0x2a0(%rbp), %rcx
1403b92f3: 48 8d 15 d6 41 5a 02        	leaq	0x25a41d6(%rip), %rdx   # 0x14295d4d0
1403b92fa: e8 31 93 1a 00              	callq	0x140562630 <.text+0x561630>
1403b92ff: 48 8b 95 28 03 00 00        	movq	0x328(%rbp), %rdx
1403b9306: 48 8b 42 28                 	movq	0x28(%rdx), %rax
1403b930a: 48 89 f1                    	movq	%rsi, %rcx
1403b930d: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403b9311: 48 89 1c 08                 	movq	%rbx, (%rax,%rcx)
1403b9315: 48 89 7c 08 08              	movq	%rdi, 0x8(%rax,%rcx)
1403b931a: 48 ff c6                    	incq	%rsi
1403b931d: 48 89 72 30                 	movq	%rsi, 0x30(%rdx)
1403b9321: 48 ff 42 40                 	incq	0x40(%rdx)
1403b9325: 48 8b 95 58 07 00 00        	movq	0x758(%rbp), %rdx
1403b932c: 89 d0                       	movl	%edx, %eax
1403b932e: 45 0f 28 d8                 	movaps	%xmm8, %xmm11
1403b9332: 48 8b 8d 08 06 00 00        	movq	0x608(%rbp), %rcx
1403b9339: 48 3b 41 10                 	cmpq	0x10(%rcx), %rax
1403b933d: 0f 83 e6 00 00 00           	jae	0x1403b9429 <.text+0x3b8429>
1403b9343: 48 8b 8d 08 06 00 00        	movq	0x608(%rbp), %rcx
1403b934a: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1403b934e: 48 c1 ea 20                 	shrq	$0x20, %rdx
1403b9352: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1403b9356: 45 0f 28 d8                 	movaps	%xmm8, %xmm11
1403b935a: 39 14 81                    	cmpl	%edx, (%rcx,%rax,4)
1403b935d: 0f 85 c6 00 00 00           	jne	0x1403b9429 <.text+0x3b8429>
1403b9363: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1403b9367: 8b 48 04                    	movl	0x4(%rax), %ecx
1403b936a: 45 0f 28 d8                 	movaps	%xmm8, %xmm11
1403b936e: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
1403b9373: 48 39 d1                    	cmpq	%rdx, %rcx
1403b9376: 0f 84 ad 00 00 00           	je	0x1403b9429 <.text+0x3b8429>
1403b937c: 45 0f 28 d8                 	movaps	%xmm8, %xmm11
1403b9380: 48 8b 95 18 06 00 00        	movq	0x618(%rbp), %rdx
1403b9387: 48 39 4a 38                 	cmpq	%rcx, 0x38(%rdx)
1403b938b: 0f 86 98 00 00 00           	jbe	0x1403b9429 <.text+0x3b8429>
1403b9391: 48 8b 95 18 06 00 00        	movq	0x618(%rbp), %rdx
1403b9398: 48 8b 52 28                 	movq	0x28(%rdx), %rdx
1403b939c: 41 89 c8                    	movl	%ecx, %r8d
1403b939f: 41 c1 e8 06                 	shrl	$0x6, %r8d
1403b93a3: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
1403b93a7: 48 0f a3 ca                 	btq	%rcx, %rdx
1403b93ab: 45 0f 28 d8                 	movaps	%xmm8, %xmm11
1403b93af: 73 78                       	jae	0x1403b9429 <.text+0x3b8429>
1403b93b1: 8b 48 0c                    	movl	0xc(%rax), %ecx
1403b93b4: 8b 40 10                    	movl	0x10(%rax), %eax
1403b93b7: 48 8b 95 08 06 00 00        	movq	0x608(%rbp), %rdx
1403b93be: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
1403b93c5: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1403b93c9: 4c 8b 85 18 06 00 00        	movq	0x618(%rbp), %r8
1403b93d0: 4d 8b 80 10 01 00 00        	movq	0x110(%r8), %r8
1403b93d7: 4c 8b 4c ca 18              	movq	0x18(%rdx,%rcx,8), %r9
1403b93dc: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
1403b93e1: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
1403b93e5: 48 f7 d1                    	notq	%rcx
1403b93e8: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1403b93ec: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403b93f0: 49 8b 4c 09 10              	movq	0x10(%r9,%rcx), %rcx
1403b93f5: 48 6b c0 38                 	imulq	$0x38, %rax, %rax
1403b93f9: f3 44 0f 10 4c 01 30        	movss	0x30(%rcx,%rax), %xmm9
1403b9400: e8 bb 5f 8f 00              	callq	0x140caf3c0 <.text+0xcae3c0>
1403b9405: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
1403b9409: f3 44 0f 58 0d c6 c4 56 02  	addss	0x256c4c6(%rip), %xmm9  # 0x1429258d8
1403b9412: f3 44 0f 59 1d 55 c5 56 02  	mulss	0x256c555(%rip), %xmm11 # 0x142925970
1403b941b: f3 45 0f 58 d9              	addss	%xmm9, %xmm11
1403b9420: f3 44 0f 5d 1d ff 3c 61 02  	minss	0x2613cff(%rip), %xmm11 # 0x1429cd128
1403b9429: 8b 85 dc 05 00 00           	movl	0x5dc(%rbp), %eax
1403b942f: 48 8b 8d 98 02 00 00        	movq	0x298(%rbp), %rcx
1403b9436: 89 01                       	movl	%eax, (%rcx)
1403b9438: 48 8b 85 30 03 00 00        	movq	0x330(%rbp), %rax
1403b943f: 48 8b 70 30                 	movq	0x30(%rax), %rsi
1403b9443: 48 8b 78 40                 	movq	0x40(%rax), %rdi
1403b9447: 48 3b 70 20                 	cmpq	0x20(%rax), %rsi
1403b944b: 75 13                       	jne	0x1403b9460 <.text+0x3b8460>
1403b944d: 48 8b 8d a8 02 00 00        	movq	0x2a8(%rbp), %rcx
1403b9454: 48 8d 15 75 40 5a 02        	leaq	0x25a4075(%rip), %rdx   # 0x14295d4d0
1403b945b: e8 d0 97 1a 00              	callq	0x140562c30 <.text+0x561c30>
1403b9460: 4c 8b 85 30 03 00 00        	movq	0x330(%rbp), %r8
1403b9467: 49 8b 40 28                 	movq	0x28(%r8), %rax
1403b946b: 48 89 f1                    	movq	%rsi, %rcx
1403b946e: 48 c1 e1 05                 	shlq	$0x5, %rcx
1403b9472: 48 89 3c 08                 	movq	%rdi, (%rax,%rcx)
1403b9476: 48 8b 95 80 06 00 00        	movq	0x680(%rbp), %rdx
1403b947d: 48 89 54 08 08              	movq	%rdx, 0x8(%rax,%rcx)
1403b9482: 8b 95 88 06 00 00           	movl	0x688(%rbp), %edx
1403b9488: 89 54 08 10                 	movl	%edx, 0x10(%rax,%rcx)
1403b948c: f3 44 0f 11 5c 08 14        	movss	%xmm11, 0x14(%rax,%rcx)
1403b9493: c6 44 08 18 00              	movb	$0x0, 0x18(%rax,%rcx)
1403b9498: 48 ff c6                    	incq	%rsi
1403b949b: 49 89 70 30                 	movq	%rsi, 0x30(%r8)
1403b949f: 49 ff 40 40                 	incq	0x40(%r8)
1403b94a3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1403b94b0: f3 41 0f 10 46 44           	movss	0x44(%r14), %xmm0
1403b94b6: 0f 28 8d 00 07 00 00        	movaps	0x700(%rbp), %xmm1
1403b94bd: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1403b94c1: f3 44 0f 5c e1              	subss	%xmm1, %xmm12
1403b94c6: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1403b94ca: 0f 28 8d 40 07 00 00        	movaps	0x740(%rbp), %xmm1
1403b94d1: 0f 59 c8                    	mulps	%xmm0, %xmm1
1403b94d4: f3 44 0f 59 25 9f c3 56 02  	mulss	0x256c39f(%rip), %xmm12 # 0x14292587c
1403b94dd: f3 44 0f 59 25 7a 3b 61 02  	mulss	0x2613b7a(%rip), %xmm12 # 0x1429cd060
1403b94e6: 44 0f 5c f1                 	subps	%xmm1, %xmm14
1403b94ea: 44 0f 59 35 3e 3c 61 02     	mulps	0x2613c3e(%rip), %xmm14 # 0x1429cd130
1403b94f2: 44 0f 59 35 46 3b 61 02     	mulps	0x2613b46(%rip), %xmm14 # 0x1429cd040
1403b94fa: 45 0f 16 e6                 	movlhps	%xmm14, %xmm12          # xmm12 = xmm12[0],xmm14[0]
1403b94fe: 41 0f 28 c4                 	movaps	%xmm12, %xmm0
1403b9502: 41 0f c6 c6 d8              	shufps	$0xd8, %xmm14, %xmm0    # xmm0 = xmm0[0,2],xmm14[1,3]
1403b9507: 45 0f c6 e6 58              	shufps	$0x58, %xmm14, %xmm12   # xmm12 = xmm12[0,2],xmm14[1,1]
1403b950c: 41 0f 59 f4                 	mulps	%xmm12, %xmm6
1403b9510: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403b9513: 0f 59 cf                    	mulps	%xmm7, %xmm1
1403b9516: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1403b951a: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1403b951e: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1403b9521: f3 0f 58 ca                 	addss	%xmm2, %xmm1
1403b9525: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1403b9529: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1403b952d: 0f 59 cf                    	mulps	%xmm7, %xmm1
1403b9530: 0f 58 ce                    	addps	%xmm6, %xmm1
1403b9533: 44 0f 59 f8                 	mulps	%xmm0, %xmm15
1403b9537: 0f c6 c0 d2                 	shufps	$0xd2, %xmm0, %xmm0     # xmm0 = xmm0[2,0,1,3]
1403b953b: 0f 59 c7                    	mulps	%xmm7, %xmm0
1403b953e: 44 0f 5c f8                 	subps	%xmm0, %xmm15
1403b9542: 45 0f c6 ff d2              	shufps	$0xd2, %xmm15, %xmm15   # xmm15 = xmm15[2,0,1,3]
1403b9547: 45 0f 59 d7                 	mulps	%xmm15, %xmm10
1403b954b: 44 0f 58 d1                 	addps	%xmm1, %xmm10
1403b954f: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1403b9553: 66 41 0f 15 c2              	unpckhpd	%xmm10, %xmm0           # xmm0 = xmm0[1],xmm10[1]
1403b9558: f3 0f 5f 05 84 0b 5a 02     	maxss	0x25a0b84(%rip), %xmm0  # 0x14295a0e4
1403b9560: f3 41 0f 5d c0              	minss	%xmm8, %xmm0
1403b9565: f3 41 0f 58 87 9c 01 00 00  	addss	0x19c(%r15), %xmm0
1403b956e: 0f 28 15 5b 51 57 02        	movaps	0x257515b(%rip), %xmm2  # 0x14292e6d0
1403b9575: 0f 28 ca                    	movaps	%xmm2, %xmm1
1403b9578: 41 0f 5f ca                 	maxps	%xmm10, %xmm1
1403b957c: 45 0f c2 d2 03              	cmpunordps	%xmm10, %xmm10
1403b9581: 41 0f 54 d2                 	andps	%xmm10, %xmm2
1403b9585: 44 0f 55 d1                 	andnps	%xmm1, %xmm10
1403b9589: 44 0f 56 d2                 	orps	%xmm2, %xmm10
1403b958d: 0f 28 15 1c 51 57 02        	movaps	0x257511c(%rip), %xmm2  # 0x14292e6b0
1403b9594: 0f 28 ca                    	movaps	%xmm2, %xmm1
1403b9597: 41 0f 5d ca                 	minps	%xmm10, %xmm1
1403b959b: 45 0f c2 d2 03              	cmpunordps	%xmm10, %xmm10
1403b95a0: 41 0f 54 d2                 	andps	%xmm10, %xmm2
1403b95a4: 44 0f 55 d1                 	andnps	%xmm1, %xmm10
1403b95a8: 44 0f 56 d2                 	orps	%xmm2, %xmm10
1403b95ac: f2 41 0f 10 8f 94 01 00 00  	movsd	0x194(%r15), %xmm1
1403b95b5: 41 0f 58 ca                 	addps	%xmm10, %xmm1
1403b95b9: 0f 59 0d 80 3b 61 02        	mulps	0x2613b80(%rip), %xmm1  # 0x1429cd140
1403b95c0: 41 0f 13 8f 94 01 00 00     	movlps	%xmm1, 0x194(%r15)
1403b95c8: f3 0f 59 05 80 3b 61 02     	mulss	0x2613b80(%rip), %xmm0  # 0x1429cd150
1403b95d0: f3 41 0f 11 87 9c 01 00 00  	movss	%xmm0, 0x19c(%r15)
1403b95d9: f2 41 0f 10 87 88 01 00 00  	movsd	0x188(%r15), %xmm0
1403b95e2: 0f 58 05 77 3b 61 02        	addps	0x2613b77(%rip), %xmm0  # 0x1429cd160
1403b95e9: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403b95ec: 0f 59 c8                    	mulps	%xmm0, %xmm1
1403b95ef: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1403b95f3: f3 0f 58 d1                 	addss	%xmm1, %xmm2
1403b95f7: 0f 57 c9                    	xorps	%xmm1, %xmm1
1403b95fa: f3 0f 51 ca                 	sqrtss	%xmm2, %xmm1
1403b95fe: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1403b9602: f3 0f 5e d1                 	divss	%xmm1, %xmm2
1403b9606: f3 0f 12 ca                 	movsldup	%xmm2, %xmm1            # xmm1 = xmm2[0,0,2,2]
1403b960a: 0f 59 c8                    	mulps	%xmm0, %xmm1
1403b960d: 0f 13 8d 90 06 00 00        	movlps	%xmm1, 0x690(%rbp)
1403b9614: f3 0f 59 15 74 70 5a 02     	mulss	0x25a7074(%rip), %xmm2  # 0x142960690
1403b961c: f3 0f 11 95 98 06 00 00     	movss	%xmm2, 0x698(%rbp)
1403b9624: 48 b8 00 00 00 00 00 00 80 3f       	movabsq	$0x3f80000000000000, %rax # imm = 0x3F80000000000000
1403b962e: 48 89 85 30 06 00 00        	movq	%rax, 0x630(%rbp)
1403b9635: c7 85 38 06 00 00 00 00 00 00       	movl	$0x0, 0x638(%rbp)
1403b963f: 4c 89 e1                    	movq	%r12, %rcx
1403b9642: 48 8d 95 30 06 00 00        	leaq	0x630(%rbp), %rdx
1403b9649: 4c 8d 85 90 06 00 00        	leaq	0x690(%rbp), %r8
1403b9650: e8 db 17 99 00              	callq	0x140d4ae30 <.text+0xd49e30>
1403b9655: 49 8d b7 b0 01 00 00        	leaq	0x1b0(%r15), %rsi
1403b965c: 48 ff 85 70 07 00 00        	incq	0x770(%rbp)
1403b9663: 0f 28 b5 80 03 00 00        	movaps	0x380(%rbp), %xmm6
1403b966a: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1403b966e: 41 0f c6 c5 00              	shufps	$0x0, %xmm13, %xmm0     # xmm0 = xmm0[0,0],xmm13[0,0]
1403b9673: 41 0f 28 cd                 	movaps	%xmm13, %xmm1
1403b9677: 41 0f c6 cd 55              	shufps	$0x55, %xmm13, %xmm1    # xmm1 = xmm1[1,1],xmm13[1,1]
1403b967c: 41 0f 28 d5                 	movaps	%xmm13, %xmm2
1403b9680: 41 0f c6 d5 aa              	shufps	$0xaa, %xmm13, %xmm2    # xmm2 = xmm2[2,2],xmm13[2,2]
1403b9685: 45 0f c6 ed ff              	shufps	$0xff, %xmm13, %xmm13   # xmm13 = xmm13[3,3,3,3]
1403b968a: 44 0f 59 ee                 	mulps	%xmm6, %xmm13
1403b968e: 0f 28 de                    	movaps	%xmm6, %xmm3
1403b9691: 0f c6 de 1b                 	shufps	$0x1b, %xmm6, %xmm3     # xmm3 = xmm3[3,2],xmm6[1,0]
1403b9695: 0f 59 c3                    	mulps	%xmm3, %xmm0
1403b9698: 0f c6 db b1                 	shufps	$0xb1, %xmm3, %xmm3     # xmm3 = xmm3[1,0,3,2]
1403b969c: 0f 59 d9                    	mulps	%xmm1, %xmm3
1403b969f: 0f 28 25 ca 3a 61 02        	movaps	0x2613aca(%rip), %xmm4  # 0x1429cd170
1403b96a6: 0f 59 c4                    	mulps	%xmm4, %xmm0
1403b96a9: 41 0f 58 c5                 	addps	%xmm13, %xmm0
1403b96ad: 0f c6 f6 b1                 	shufps	$0xb1, %xmm6, %xmm6     # xmm6 = xmm6[1,0,3,2]
1403b96b1: 0f 59 f2                    	mulps	%xmm2, %xmm6
1403b96b4: 0f 28 15 c5 3a 61 02        	movaps	0x2613ac5(%rip), %xmm2  # 0x1429cd180
1403b96bb: 0f 59 da                    	mulps	%xmm2, %xmm3
1403b96be: 0f 28 2d cb 3a 61 02        	movaps	0x2613acb(%rip), %xmm5  # 0x1429cd190
1403b96c5: 0f 59 f5                    	mulps	%xmm5, %xmm6
1403b96c8: 0f 58 f3                    	addps	%xmm3, %xmm6
1403b96cb: 0f 58 f0                    	addps	%xmm0, %xmm6
1403b96ce: 0f 29 b5 30 05 00 00        	movaps	%xmm6, 0x530(%rbp)
1403b96d5: 48 8b bd b0 00 00 00        	movq	0xb0(%rbp), %rdi
1403b96dc: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
1403b96e3: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1403b96ea: 4c 8d 24 c8                 	leaq	(%rax,%rcx,8), %r12
1403b96ee: 4d 8b bf 68 01 00 00        	movq	0x168(%r15), %r15
1403b96f5: 44 0f 28 c6                 	movaps	%xmm6, %xmm8
1403b96f9: 44 0f c6 c6 00              	shufps	$0x0, %xmm6, %xmm8      # xmm8 = xmm8[0,0],xmm6[0,0]
1403b96fe: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
1403b9702: 44 0f c6 ce 55              	shufps	$0x55, %xmm6, %xmm9     # xmm9 = xmm9[1,1],xmm6[1,1]
1403b9707: 0f 28 fe                    	movaps	%xmm6, %xmm7
1403b970a: 0f c6 fe aa                 	shufps	$0xaa, %xmm6, %xmm7     # xmm7 = xmm7[2,2],xmm6[2,2]
1403b970e: 44 0f 28 d6                 	movaps	%xmm6, %xmm10
1403b9712: 44 0f c6 d6 ff              	shufps	$0xff, %xmm6, %xmm10    # xmm10 = xmm10[3,3],xmm6[3,3]
1403b9717: 41 0f 28 c2                 	movaps	%xmm10, %xmm0
1403b971b: 0f 59 05 7e 3a 61 02        	mulps	0x2613a7e(%rip), %xmm0  # 0x1429cd1a0
1403b9722: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1403b9726: 0f 59 0d 83 3a 61 02        	mulps	0x2613a83(%rip), %xmm1  # 0x1429cd1b0
1403b972d: 0f 59 cc                    	mulps	%xmm4, %xmm1
1403b9730: 0f 58 c8                    	addps	%xmm0, %xmm1
1403b9733: 41 0f 28 c1                 	movaps	%xmm9, %xmm0
1403b9737: 0f 59 05 82 3a 61 02        	mulps	0x2613a82(%rip), %xmm0  # 0x1429cd1c0
1403b973e: 0f 59 c2                    	mulps	%xmm2, %xmm0
1403b9741: 44 0f 28 df                 	movaps	%xmm7, %xmm11
1403b9745: 44 0f 59 1d 83 3a 61 02     	mulps	0x2613a83(%rip), %xmm11 # 0x1429cd1d0
1403b974d: 44 0f 59 dd                 	mulps	%xmm5, %xmm11
1403b9751: 44 0f 58 d8                 	addps	%xmm0, %xmm11
1403b9755: 44 0f 58 d9                 	addps	%xmm1, %xmm11
1403b9759: 48 8d 1c fd 00 00 00 00     	leaq	(,%rdi,8), %rbx
1403b9761: 48 89 c1                    	movq	%rax, %rcx
1403b9764: 44 0f 28 25 44 4f 57 02     	movaps	0x2574f44(%rip), %xmm12 # 0x14292e6b0
1403b976c: 45 0f 57 ed                 	xorps	%xmm13, %xmm13
1403b9770: 48 85 c9                    	testq	%rcx, %rcx
1403b9773: 74 3b                       	je	0x1403b97b0 <.text+0x3b87b0>
1403b9775: 4d 89 e6                    	movq	%r12, %r14
1403b9778: 4c 39 e0                    	cmpq	%r12, %rax
1403b977b: 74 13                       	je	0x1403b9790 <.text+0x3b8790>
1403b977d: 4c 8b 00                    	movq	(%rax), %r8
1403b9780: 48 83 c0 08                 	addq	$0x8, %rax
1403b9784: 4c 89 c2                    	movq	%r8, %rdx
1403b9787: 48 c1 ea 20                 	shrq	$0x20, %rdx
1403b978b: 75 4c                       	jne	0x1403b97d9 <.text+0x3b87d9>
1403b978d: 49 89 c6                    	movq	%rax, %r14
1403b9790: 48 85 ff                    	testq	%rdi, %rdi
1403b9793: 74 1e                       	je	0x1403b97b3 <.text+0x3b87b3>
1403b9795: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403b979b: 48 89 da                    	movq	%rbx, %rdx
1403b979e: e8 7d a4 25 00              	callq	0x140613c20 <.text+0x612c20>
1403b97a3: eb 0e                       	jmp	0x1403b97b3 <.text+0x3b87b3>
1403b97a5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403b97b0: 49 89 c6                    	movq	%rax, %r14
1403b97b3: 4c 89 f8                    	movq	%r15, %rax
1403b97b6: 48 c1 e8 20                 	shrq	$0x20, %rax
1403b97ba: 0f 84 10 01 00 00           	je	0x1403b98d0 <.text+0x3b88d0>
1403b97c0: 45 89 f9                    	movl	%r15d, %r9d
1403b97c3: 4c 89 f0                    	movq	%r14, %rax
1403b97c6: 31 c9                       	xorl	%ecx, %ecx
1403b97c8: 4c 89 ca                    	movq	%r9, %rdx
1403b97cb: 4d 89 f8                    	movq	%r15, %r8
1403b97ce: 4d 89 cf                    	movq	%r9, %r15
1403b97d1: 4d 3b 4d 10                 	cmpq	0x10(%r13), %r9
1403b97d5: 73 99                       	jae	0x1403b9770 <.text+0x3b8770>
1403b97d7: eb 0c                       	jmp	0x1403b97e5 <.text+0x3b87e5>
1403b97d9: 45 89 c1                    	movl	%r8d, %r9d
1403b97dc: 4c 89 fa                    	movq	%r15, %rdx
1403b97df: 4d 3b 4d 10                 	cmpq	0x10(%r13), %r9
1403b97e3: 73 8b                       	jae	0x1403b9770 <.text+0x3b8770>
1403b97e5: 4d 8b 55 08                 	movq	0x8(%r13), %r10
1403b97e9: 49 c1 e8 20                 	shrq	$0x20, %r8
1403b97ed: 4f 8d 0c 89                 	leaq	(%r9,%r9,4), %r9
1403b97f1: 49 89 d7                    	movq	%rdx, %r15
1403b97f4: 47 39 04 8a                 	cmpl	%r8d, (%r10,%r9,4)
1403b97f8: 0f 85 72 ff ff ff           	jne	0x1403b9770 <.text+0x3b8770>
1403b97fe: 4f 8d 04 8a                 	leaq	(%r10,%r9,4), %r8
1403b9802: 45 8b 48 04                 	movl	0x4(%r8), %r9d
1403b9806: 49 89 d7                    	movq	%rdx, %r15
1403b9809: 41 ba ff ff ff ff           	movl	$0xffffffff, %r10d      # imm = 0xFFFFFFFF
1403b980f: 4d 39 d1                    	cmpq	%r10, %r9
1403b9812: 0f 84 58 ff ff ff           	je	0x1403b9770 <.text+0x3b8770>
1403b9818: 49 89 d7                    	movq	%rdx, %r15
1403b981b: 4c 8b 95 88 07 00 00        	movq	0x788(%rbp), %r10
1403b9822: 4d 39 4a 38                 	cmpq	%r9, 0x38(%r10)
1403b9826: 0f 86 44 ff ff ff           	jbe	0x1403b9770 <.text+0x3b8770>
1403b982c: 4c 8b 95 88 07 00 00        	movq	0x788(%rbp), %r10
1403b9833: 4d 8b 52 28                 	movq	0x28(%r10), %r10
1403b9837: 45 89 cb                    	movl	%r9d, %r11d
1403b983a: 41 c1 eb 06                 	shrl	$0x6, %r11d
1403b983e: 4f 8b 14 da                 	movq	(%r10,%r11,8), %r10
1403b9842: 4d 0f a3 ca                 	btq	%r9, %r10
1403b9846: 49 89 d7                    	movq	%rdx, %r15
1403b9849: 0f 83 21 ff ff ff           	jae	0x1403b9770 <.text+0x3b8770>
1403b984f: 45 8b 48 0c                 	movl	0xc(%r8), %r9d
1403b9853: 45 8b 40 10                 	movl	0x10(%r8), %r8d
1403b9857: 4d 8b 95 a8 01 00 00        	movq	0x1a8(%r13), %r10
1403b985e: 4f 8d 0c c9                 	leaq	(%r9,%r9,8), %r9
1403b9862: 4c 8b 9d 88 07 00 00        	movq	0x788(%rbp), %r11
1403b9869: 4d 8b 9b 10 01 00 00        	movq	0x110(%r11), %r11
1403b9870: 4f 8b 74 ca 18              	movq	0x18(%r10,%r9,8), %r14
1403b9875: 4f 8b 4c ca 38              	movq	0x38(%r10,%r9,8), %r9
1403b987a: 4f 8b 0c d9                 	movq	(%r9,%r11,8), %r9
1403b987e: 49 f7 d1                    	notq	%r9
1403b9881: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
1403b9885: 49 c1 e1 04                 	shlq	$0x4, %r9
1403b9889: 4f 8b 54 0e 10              	movq	0x10(%r14,%r9), %r10
1403b988e: 4f 8b 4c 0e 28              	movq	0x28(%r14,%r9), %r9
1403b9893: 4f 8d 1c 40                 	leaq	(%r8,%r8,2), %r11
1403b9897: 49 c1 e3 04                 	shlq	$0x4, %r11
1403b989b: 44 8b b5 88 06 00 00        	movl	0x688(%rbp), %r14d
1403b98a2: 47 89 74 1a 18              	movl	%r14d, 0x18(%r10,%r11)
1403b98a7: 4c 8b b5 80 06 00 00        	movq	0x680(%rbp), %r14
1403b98ae: 4f 89 74 1a 10              	movq	%r14, 0x10(%r10,%r11)
1403b98b3: 44 8b b5 28 07 00 00        	movl	0x728(%rbp), %r14d
1403b98ba: 47 89 34 81                 	movl	%r14d, (%r9,%r8,4)
1403b98be: 47 0f 29 1c 1a              	movaps	%xmm11, (%r10,%r11)
1403b98c3: 49 89 d7                    	movq	%rdx, %r15
1403b98c6: e9 a5 fe ff ff              	jmp	0x1403b9770 <.text+0x3b8770>
1403b98cb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1403b98d0: 48 8b 95 68 07 00 00        	movq	0x768(%rbp), %rdx
1403b98d7: 48 85 d2                    	testq	%rdx, %rdx
1403b98da: 0f 84 90 ef ff ff           	je	0x1403b8870 <.text+0x3b7870>
1403b98e0: 0f 28 ce                    	movaps	%xmm6, %xmm1
1403b98e3: 0f 59 ce                    	mulps	%xmm6, %xmm1
1403b98e6: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1403b98ea: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1403b98ee: 0f 28 d1                    	movaps	%xmm1, %xmm2
1403b98f1: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1403b98f5: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1403b98f9: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1403b98fd: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1403b9901: 0f 5c ca                    	subps	%xmm2, %xmm1
1403b9904: 0f 28 15 d5 38 61 02        	movaps	0x26138d5(%rip), %xmm2  # 0x1429cd1e0
1403b990b: 0f 59 ca                    	mulps	%xmm2, %xmm1
1403b990e: 0f 28 c6                    	movaps	%xmm6, %xmm0
1403b9911: 0f 59 c2                    	mulps	%xmm2, %xmm0
1403b9914: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1403b9918: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1403b991c: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1403b991f: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1403b9923: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1403b9927: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1403b992b: 0f 59 c6                    	mulps	%xmm6, %xmm0
1403b992e: 0f 58 c1                    	addps	%xmm1, %xmm0
1403b9931: 0f 28 ce                    	movaps	%xmm6, %xmm1
1403b9934: 0f c6 ce d2                 	shufps	$0xd2, %xmm6, %xmm1     # xmm1 = xmm1[2,0],xmm6[1,3]
1403b9938: 0f 59 0d b1 38 61 02        	mulps	0x26138b1(%rip), %xmm1  # 0x1429cd1f0
1403b993f: 0f 28 d6                    	movaps	%xmm6, %xmm2
1403b9942: 0f c6 d6 c9                 	shufps	$0xc9, %xmm6, %xmm2     # xmm2 = xmm2[1,2],xmm6[0,3]
1403b9946: 0f 59 15 b3 38 61 02        	mulps	0x26138b3(%rip), %xmm2  # 0x1429cd200
1403b994d: 0f 5c d1                    	subps	%xmm1, %xmm2
1403b9950: 0f 58 f6                    	addps	%xmm6, %xmm6
1403b9953: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1403b9957: 0f 59 f2                    	mulps	%xmm2, %xmm6
1403b995a: 0f 58 f0                    	addps	%xmm0, %xmm6
1403b995d: 0f 28 c6                    	movaps	%xmm6, %xmm0
1403b9960: 66 0f 15 c6                 	unpckhpd	%xmm6, %xmm0            # xmm0 = xmm0[1],xmm6[1]
1403b9964: f3 0f 58 85 88 06 00 00     	addss	0x688(%rbp), %xmm0
1403b996c: f2 0f 10 8d 80 06 00 00     	movsd	0x680(%rbp), %xmm1
1403b9974: 0f 58 ce                    	addps	%xmm6, %xmm1
1403b9977: 0f 13 4a 10                 	movlps	%xmm1, 0x10(%rdx)
1403b997b: f3 0f 11 42 18              	movss	%xmm0, 0x18(%rdx)
1403b9980: 44 0f 59 15 88 38 61 02     	mulps	0x2613888(%rip), %xmm10 # 0x1429cd210
1403b9988: 44 0f 59 05 90 38 61 02     	mulps	0x2613890(%rip), %xmm8  # 0x1429cd220
1403b9990: 0f 28 1d d9 37 61 02        	movaps	0x26137d9(%rip), %xmm3  # 0x1429cd170
1403b9997: 44 0f 59 c3                 	mulps	%xmm3, %xmm8
1403b999b: 45 0f 58 c2                 	addps	%xmm10, %xmm8
1403b999f: 44 0f 59 0d 89 38 61 02     	mulps	0x2613889(%rip), %xmm9  # 0x1429cd230
1403b99a7: 0f 28 25 d2 37 61 02        	movaps	0x26137d2(%rip), %xmm4  # 0x1429cd180
1403b99ae: 44 0f 59 cc                 	mulps	%xmm4, %xmm9
1403b99b2: 0f 59 3d 87 38 61 02        	mulps	0x2613887(%rip), %xmm7  # 0x1429cd240
1403b99b9: 0f 28 2d d0 37 61 02        	movaps	0x26137d0(%rip), %xmm5  # 0x1429cd190
1403b99c0: 0f 59 fd                    	mulps	%xmm5, %xmm7
1403b99c3: 41 0f 58 f9                 	addps	%xmm9, %xmm7
1403b99c7: 41 0f 58 f8                 	addps	%xmm8, %xmm7
1403b99cb: 0f 28 c7                    	movaps	%xmm7, %xmm0
1403b99ce: 0f c6 c7 00                 	shufps	$0x0, %xmm7, %xmm0      # xmm0 = xmm0[0,0],xmm7[0,0]
1403b99d2: 0f 28 cf                    	movaps	%xmm7, %xmm1
1403b99d5: 0f 28 d7                    	movaps	%xmm7, %xmm2
1403b99d8: 0f c6 ff ff                 	shufps	$0xff, %xmm7, %xmm7     # xmm7 = xmm7[3,3,3,3]
1403b99dc: 0f 59 3d 6d 38 61 02        	mulps	0x261386d(%rip), %xmm7  # 0x1429cd250
1403b99e3: 0f 59 05 76 38 61 02        	mulps	0x2613876(%rip), %xmm0  # 0x1429cd260
1403b99ea: 0f 59 c3                    	mulps	%xmm3, %xmm0
1403b99ed: 0f 58 c7                    	addps	%xmm7, %xmm0
1403b99f0: 0f c6 c9 55                 	shufps	$0x55, %xmm1, %xmm1     # xmm1 = xmm1[1,1,1,1]
1403b99f4: 0f c6 d2 aa                 	shufps	$0xaa, %xmm2, %xmm2     # xmm2 = xmm2[2,2,2,2]
1403b99f8: 0f 59 0d 71 38 61 02        	mulps	0x2613871(%rip), %xmm1  # 0x1429cd270
1403b99ff: 0f 59 cc                    	mulps	%xmm4, %xmm1
1403b9a02: 0f 59 15 77 38 61 02        	mulps	0x2613877(%rip), %xmm2  # 0x1429cd280
1403b9a09: 0f 59 d5                    	mulps	%xmm5, %xmm2
1403b9a0c: 0f 58 d1                    	addps	%xmm1, %xmm2
1403b9a0f: 0f 58 d0                    	addps	%xmm0, %xmm2
1403b9a12: 8b 85 d4 05 00 00           	movl	0x5d4(%rbp), %eax
1403b9a18: 48 8b 8d 88 02 00 00        	movq	0x288(%rbp), %rcx
1403b9a1f: 89 01                       	movl	%eax, (%rcx)
1403b9a21: 0f 29 12                    	movaps	%xmm2, (%rdx)
1403b9a24: e9 47 ee ff ff              	jmp	0x1403b8870 <.text+0x3b7870>
1403b9a29: 45 31 c0                    	xorl	%r8d, %r8d
1403b9a2c: 4c 8b b5 80 07 00 00        	movq	0x780(%rbp), %r14
1403b9a33: eb 03                       	jmp	0x1403b9a38 <.text+0x3b8a38>
1403b9a35: 45 31 c0                    	xorl	%r8d, %r8d
1403b9a38: 8b 40 10                    	movl	0x10(%rax), %eax
1403b9a3b: 4c 8b 8d 38 03 00 00        	movq	0x338(%rbp), %r9
1403b9a42: 4d 8b 89 10 01 00 00        	movq	0x110(%r9), %r9
1403b9a49: 4a 8b 14 ca                 	movq	(%rdx,%r9,8), %rdx
1403b9a4d: 48 f7 d2                    	notq	%rdx
1403b9a50: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1403b9a54: 48 c1 e2 04                 	shlq	$0x4, %rdx
1403b9a58: 4c 8d 0c 40                 	leaq	(%rax,%rax,2), %r9
1403b9a5c: 49 c1 e1 04                 	shlq	$0x4, %r9
1403b9a60: 4c 03 4c 11 10              	addq	0x10(%rcx,%rdx), %r9
1403b9a65: 4c 89 8d 68 07 00 00        	movq	%r9, 0x768(%rbp)
1403b9a6c: 48 8b 4c 11 28              	movq	0x28(%rcx,%rdx), %rcx
1403b9a71: 48 8d 0c 81                 	leaq	(%rcx,%rax,4), %rcx
1403b9a75: 48 89 8d 88 02 00 00        	movq	%rcx, 0x288(%rbp)
1403b9a7c: 48 6b c0 68                 	imulq	$0x68, %rax, %rax
1403b9a80: 4c 01 c0                    	addq	%r8, %rax
1403b9a83: 48 89 85 18 03 00 00        	movq	%rax, 0x318(%rbp)
1403b9a8a: 48 89 9d 58 07 00 00        	movq	%rbx, 0x758(%rbp)
1403b9a91: 48 85 f6                    	testq	%rsi, %rsi
1403b9a94: 0f 89 cf f0 ff ff           	jns	0x1403b8b69 <.text+0x3b7b69>
1403b9a9a: e9 c6 f1 ff ff              	jmp	0x1403b8c65 <.text+0x3b7c65>
1403b9a9f: 4c 8b 85 80 07 00 00        	movq	0x780(%rbp), %r8
1403b9aa6: 4d 8b 60 38                 	movq	0x38(%r8), %r12
1403b9aaa: 49 8b 50 10                 	movq	0x10(%r8), %rdx
1403b9aae: 49 39 d4                    	cmpq	%rdx, %r12
1403b9ab1: 0f 83 23 2d 00 00           	jae	0x1403bc7da <.text+0x3bb7da>
1403b9ab7: 4d 85 e4                    	testq	%r12, %r12
1403b9aba: 78 0a                       	js	0x1403b9ac6 <.text+0x3b8ac6>
1403b9abc: 0f 57 ff                    	xorps	%xmm7, %xmm7
1403b9abf: f3 49 0f 2a fc              	cvtsi2ss	%r12, %xmm7
1403b9ac4: eb 1b                       	jmp	0x1403b9ae1 <.text+0x3b8ae1>
1403b9ac6: 4c 89 e0                    	movq	%r12, %rax
1403b9ac9: 48 d1 e8                    	shrq	%rax
1403b9acc: 44 89 e1                    	movl	%r12d, %ecx
1403b9acf: 83 e1 01                    	andl	$0x1, %ecx
1403b9ad2: 48 09 c1                    	orq	%rax, %rcx
1403b9ad5: 0f 57 ff                    	xorps	%xmm7, %xmm7
1403b9ad8: f3 48 0f 2a f9              	cvtsi2ss	%rcx, %xmm7
1403b9add: f3 0f 58 ff                 	addss	%xmm7, %xmm7
1403b9ae1: 49 8b 70 08                 	movq	0x8(%r8), %rsi
1403b9ae5: f3 0f 59 3d 77 35 61 02     	mulss	0x2613577(%rip), %xmm7  # 0x1429cd064
1403b9aed: f3 41 0f 58 78 40           	addss	0x40(%r8), %xmm7
1403b9af3: 48 8d 8d 78 02 00 00        	leaq	0x278(%rbp), %rcx
1403b9afa: 48 8b 95 60 07 00 00        	movq	0x760(%rbp), %rdx
1403b9b01: 0f 28 d7                    	movaps	%xmm7, %xmm2
1403b9b04: e8 97 ab 8d 00              	callq	0x140c946a0 <.text+0xc936a0>
1403b9b09: 48 8d 8d 68 02 00 00        	leaq	0x268(%rbp), %rcx
1403b9b10: 48 8b 95 60 07 00 00        	movq	0x760(%rbp), %rdx
1403b9b17: 0f 28 d7                    	movaps	%xmm7, %xmm2
1403b9b1a: e8 e1 ac 8d 00              	callq	0x140c94800 <.text+0xc93800>
1403b9b1f: f3 0f 10 85 70 02 00 00     	movss	0x270(%rbp), %xmm0
1403b9b27: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403b9b2a: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1403b9b2e: f2 0f 10 95 68 02 00 00     	movsd	0x268(%rbp), %xmm2
1403b9b36: 0f 28 da                    	movaps	%xmm2, %xmm3
1403b9b39: 0f 59 da                    	mulps	%xmm2, %xmm3
1403b9b3c: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
1403b9b40: f3 0f 58 e3                 	addss	%xmm3, %xmm4
1403b9b44: f3 0f 58 e1                 	addss	%xmm1, %xmm4
1403b9b48: 0f 57 c9                    	xorps	%xmm1, %xmm1
1403b9b4b: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
1403b9b4f: f3 0f 10 3d ad bd 56 02     	movss	0x256bdad(%rip), %xmm7  # 0x142925904
1403b9b57: 0f 28 df                    	movaps	%xmm7, %xmm3
1403b9b5a: f3 0f 5e d9                 	divss	%xmm1, %xmm3
1403b9b5e: f3 0f 12 cb                 	movsldup	%xmm3, %xmm1            # xmm1 = xmm3[0,0,2,2]
1403b9b62: 0f 59 ca                    	mulps	%xmm2, %xmm1
1403b9b65: f3 0f 59 d8                 	mulss	%xmm0, %xmm3
1403b9b69: 0f 13 8d b0 00 00 00        	movlps	%xmm1, 0xb0(%rbp)
1403b9b70: f3 0f 11 9d b8 00 00 00     	movss	%xmm3, 0xb8(%rbp)
1403b9b78: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403b9b7f: 48 8d 95 b0 00 00 00        	leaq	0xb0(%rbp), %rdx
1403b9b86: e8 f5 b1 f7 ff              	callq	0x140334d80 <.text+0x333d80>
1403b9b8b: 0f 28 b5 80 03 00 00        	movaps	0x380(%rbp), %xmm6
1403b9b92: 48 8b 85 f8 06 00 00        	movq	0x6f8(%rbp), %rax
1403b9b99: 48 8d 90 f0 02 00 00        	leaq	0x2f0(%rax), %rdx
1403b9ba0: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403b9ba7: 48 89 95 d0 02 00 00        	movq	%rdx, 0x2d0(%rbp)
1403b9bae: e8 5d a7 5d 00              	callq	0x140994310 <.text+0x993310>
1403b9bb3: 0f 28 85 80 03 00 00        	movaps	0x380(%rbp), %xmm0
1403b9bba: 0f 28 ce                    	movaps	%xmm6, %xmm1
1403b9bbd: 0f c6 ce 00                 	shufps	$0x0, %xmm6, %xmm1      # xmm1 = xmm1[0,0],xmm6[0,0]
1403b9bc1: 0f 28 d6                    	movaps	%xmm6, %xmm2
1403b9bc4: 0f c6 d6 55                 	shufps	$0x55, %xmm6, %xmm2     # xmm2 = xmm2[1,1],xmm6[1,1]
1403b9bc8: 0f 28 de                    	movaps	%xmm6, %xmm3
1403b9bcb: 0f c6 de aa                 	shufps	$0xaa, %xmm6, %xmm3     # xmm3 = xmm3[2,2],xmm6[2,2]
1403b9bcf: 0f 28 e6                    	movaps	%xmm6, %xmm4
1403b9bd2: 0f c6 e6 ff                 	shufps	$0xff, %xmm6, %xmm4     # xmm4 = xmm4[3,3],xmm6[3,3]
1403b9bd6: 0f 59 e0                    	mulps	%xmm0, %xmm4
1403b9bd9: 0f 28 e8                    	movaps	%xmm0, %xmm5
1403b9bdc: 0f c6 e8 1b                 	shufps	$0x1b, %xmm0, %xmm5     # xmm5 = xmm5[3,2],xmm0[1,0]
1403b9be0: 0f 59 cd                    	mulps	%xmm5, %xmm1
1403b9be3: 0f c6 ed b1                 	shufps	$0xb1, %xmm5, %xmm5     # xmm5 = xmm5[1,0,3,2]
1403b9be7: 0f 59 ea                    	mulps	%xmm2, %xmm5
1403b9bea: 0f 59 0d 7f 35 61 02        	mulps	0x261357f(%rip), %xmm1  # 0x1429cd170
1403b9bf1: 0f c6 c0 b1                 	shufps	$0xb1, %xmm0, %xmm0     # xmm0 = xmm0[1,0,3,2]
1403b9bf5: 0f 59 c3                    	mulps	%xmm3, %xmm0
1403b9bf8: 0f 59 2d 81 35 61 02        	mulps	0x2613581(%rip), %xmm5  # 0x1429cd180
1403b9bff: 0f 59 05 8a 35 61 02        	mulps	0x261358a(%rip), %xmm0  # 0x1429cd190
1403b9c06: 0f 58 cc                    	addps	%xmm4, %xmm1
1403b9c09: 0f 58 c5                    	addps	%xmm5, %xmm0
1403b9c0c: 0f 28 de                    	movaps	%xmm6, %xmm3
1403b9c0f: 0f 59 de                    	mulps	%xmm6, %xmm3
1403b9c12: f3 0f 16 d3                 	movshdup	%xmm3, %xmm2            # xmm2 = xmm3[1,1,3,3]
1403b9c16: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1403b9c1a: 0f 28 e3                    	movaps	%xmm3, %xmm4
1403b9c1d: 66 0f 15 e3                 	unpckhpd	%xmm3, %xmm4            # xmm4 = xmm4[1],xmm3[1]
1403b9c21: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1403b9c25: 0f c6 e4 00                 	shufps	$0x0, %xmm4, %xmm4      # xmm4 = xmm4[0,0,0,0]
1403b9c29: 0f c6 db ff                 	shufps	$0xff, %xmm3, %xmm3     # xmm3 = xmm3[3,3,3,3]
1403b9c2d: 0f 5c dc                    	subps	%xmm4, %xmm3
1403b9c30: 0f 28 15 59 36 61 02        	movaps	0x2613659(%rip), %xmm2  # 0x1429cd290
1403b9c37: 0f 59 da                    	mulps	%xmm2, %xmm3
1403b9c3a: 0f 59 d6                    	mulps	%xmm6, %xmm2
1403b9c3d: f3 0f 16 e2                 	movshdup	%xmm2, %xmm4            # xmm4 = xmm2[1,1,3,3]
1403b9c41: f3 0f 58 e2                 	addss	%xmm2, %xmm4
1403b9c45: 0f 12 d2                    	movhlps	%xmm2, %xmm2            # xmm2 = xmm2[1,1]
1403b9c48: f3 0f 58 d4                 	addss	%xmm4, %xmm2
1403b9c4c: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1403b9c50: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1403b9c54: 0f 59 d6                    	mulps	%xmm6, %xmm2
1403b9c57: 0f 58 d3                    	addps	%xmm3, %xmm2
1403b9c5a: 0f 28 de                    	movaps	%xmm6, %xmm3
1403b9c5d: 0f c6 de d2                 	shufps	$0xd2, %xmm6, %xmm3     # xmm3 = xmm3[2,0],xmm6[1,3]
1403b9c61: 0f 59 1d 38 36 61 02        	mulps	0x2613638(%rip), %xmm3  # 0x1429cd2a0
1403b9c68: 0f 28 e6                    	movaps	%xmm6, %xmm4
1403b9c6b: 0f c6 e6 c9                 	shufps	$0xc9, %xmm6, %xmm4     # xmm4 = xmm4[1,2],xmm6[0,3]
1403b9c6f: 0f 59 25 3a 36 61 02        	mulps	0x261363a(%rip), %xmm4  # 0x1429cd2b0
1403b9c76: 0f 5c e3                    	subps	%xmm3, %xmm4
1403b9c79: 0f 58 f6                    	addps	%xmm6, %xmm6
1403b9c7c: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1403b9c80: 0f 59 f4                    	mulps	%xmm4, %xmm6
1403b9c83: 0f 58 f2                    	addps	%xmm2, %xmm6
1403b9c86: 0f 28 de                    	movaps	%xmm6, %xmm3
1403b9c89: 66 0f 15 de                 	unpckhpd	%xmm6, %xmm3            # xmm3 = xmm3[1],xmm6[1]
1403b9c8d: f3 0f 58 9d 80 02 00 00     	addss	0x280(%rbp), %xmm3
1403b9c95: 49 69 c4 b0 01 00 00        	imulq	$0x1b0, %r12, %rax      # imm = 0x1B0
1403b9c9c: f3 0f 10 94 06 84 01 00 00  	movss	0x184(%rsi,%rax), %xmm2
1403b9ca5: f3 0f 5f 15 37 04 5a 02     	maxss	0x25a0437(%rip), %xmm2  # 0x14295a0e4
1403b9cad: f3 0f 5d 15 4f bc 56 02     	minss	0x256bc4f(%rip), %xmm2  # 0x142925904
1403b9cb5: f3 0f 59 15 b3 bc 56 02     	mulss	0x256bcb3(%rip), %xmm2  # 0x142925970
1403b9cbd: 0f 58 c1                    	addps	%xmm1, %xmm0
1403b9cc0: f3 0f 58 d3                 	addss	%xmm3, %xmm2
1403b9cc4: f2 0f 10 8d 78 02 00 00     	movsd	0x278(%rbp), %xmm1
1403b9ccc: 0f 58 ce                    	addps	%xmm6, %xmm1
1403b9ccf: f2 0f 10 9c 06 7c 01 00 00  	movsd	0x17c(%rsi,%rax), %xmm3
1403b9cd8: 0f 28 2d f1 49 57 02        	movaps	0x25749f1(%rip), %xmm5  # 0x14292e6d0
1403b9cdf: 0f 28 e5                    	movaps	%xmm5, %xmm4
1403b9ce2: 0f 5f e3                    	maxps	%xmm3, %xmm4
1403b9ce5: 0f c2 db 03                 	cmpunordps	%xmm3, %xmm3
1403b9ce9: 0f 54 eb                    	andps	%xmm3, %xmm5
1403b9cec: 0f 55 dc                    	andnps	%xmm4, %xmm3
1403b9cef: 0f 56 dd                    	orps	%xmm5, %xmm3
1403b9cf2: 41 0f 28 e4                 	movaps	%xmm12, %xmm4
1403b9cf6: 0f 5d e3                    	minps	%xmm3, %xmm4
1403b9cf9: 0f c2 db 03                 	cmpunordps	%xmm3, %xmm3
1403b9cfd: 44 0f 54 e3                 	andps	%xmm3, %xmm12
1403b9d01: 0f 55 dc                    	andnps	%xmm4, %xmm3
1403b9d04: 41 0f 56 dc                 	orps	%xmm12, %xmm3
1403b9d08: 0f 59 1d b1 35 61 02        	mulps	0x26135b1(%rip), %xmm3  # 0x1429cd2c0
1403b9d0f: 0f 58 d9                    	addps	%xmm1, %xmm3
1403b9d12: 0f 13 9d b0 05 00 00        	movlps	%xmm3, 0x5b0(%rbp)
1403b9d19: f3 0f 11 95 b8 05 00 00     	movss	%xmm2, 0x5b8(%rbp)
1403b9d21: 0f 29 85 c0 05 00 00        	movaps	%xmm0, 0x5c0(%rbp)
1403b9d28: 48 8b 85 f8 06 00 00        	movq	0x6f8(%rbp), %rax
1403b9d2f: 48 8d 90 00 01 00 00        	leaq	0x100(%rax), %rdx
1403b9d36: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403b9d3d: e8 8e ac 55 00              	callq	0x1409149d0 <.text+0x9139d0>
1403b9d42: 48 8b 85 a0 06 00 00        	movq	0x6a0(%rbp), %rax
1403b9d49: 48 8b 8d f8 06 00 00        	movq	0x6f8(%rbp), %rcx
1403b9d50: f3 0f 10 81 d8 04 00 00     	movss	0x4d8(%rcx), %xmm0
1403b9d58: 48 8b 48 20                 	movq	0x20(%rax), %rcx
1403b9d5c: 8b 50 28                    	movl	0x28(%rax), %edx
1403b9d5f: 0f 10 08                    	movups	(%rax), %xmm1
1403b9d62: 0f 10 50 10                 	movups	0x10(%rax), %xmm2
1403b9d66: 0f 11 8d 34 06 00 00        	movups	%xmm1, 0x634(%rbp)
1403b9d6d: 89 95 5c 06 00 00           	movl	%edx, 0x65c(%rbp)
1403b9d73: 48 89 8d 54 06 00 00        	movq	%rcx, 0x654(%rbp)
1403b9d7a: 0f 11 95 44 06 00 00        	movups	%xmm2, 0x644(%rbp)
1403b9d81: f3 0f 11 85 30 06 00 00     	movss	%xmm0, 0x630(%rbp)
1403b9d89: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403b9d90: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
1403b9d97: e8 d4 61 8d 00              	callq	0x140c8ff70 <.text+0xc8ef70>
1403b9d9c: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1403b9da3: 48 8d 95 30 06 00 00        	leaq	0x630(%rbp), %rdx
1403b9daa: e8 e1 d5 8d 00              	callq	0x140c97390 <.text+0xc96390>
1403b9daf: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1403b9db6: 0f 28 8d c0 00 00 00        	movaps	0xc0(%rbp), %xmm1
1403b9dbd: 0f 28 95 d0 00 00 00        	movaps	0xd0(%rbp), %xmm2
1403b9dc4: 0f 28 9d e0 00 00 00        	movaps	0xe0(%rbp), %xmm3
1403b9dcb: 0f 29 9d b0 03 00 00        	movaps	%xmm3, 0x3b0(%rbp)
1403b9dd2: 0f 29 95 a0 03 00 00        	movaps	%xmm2, 0x3a0(%rbp)
1403b9dd9: 0f 29 8d 90 03 00 00        	movaps	%xmm1, 0x390(%rbp)
1403b9de0: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
1403b9de7: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1403b9dee: 0f 29 85 c0 03 00 00        	movaps	%xmm0, 0x3c0(%rbp)
1403b9df5: 0f 28 85 00 01 00 00        	movaps	0x100(%rbp), %xmm0
1403b9dfc: 0f 29 85 d0 03 00 00        	movaps	%xmm0, 0x3d0(%rbp)
1403b9e03: 0f 28 85 10 01 00 00        	movaps	0x110(%rbp), %xmm0
1403b9e0a: 0f 29 85 e0 03 00 00        	movaps	%xmm0, 0x3e0(%rbp)
1403b9e11: 0f 28 85 20 01 00 00        	movaps	0x120(%rbp), %xmm0
1403b9e18: 0f 29 85 f0 03 00 00        	movaps	%xmm0, 0x3f0(%rbp)
1403b9e1f: 0f 28 85 30 05 00 00        	movaps	0x530(%rbp), %xmm0
1403b9e26: 0f 28 8d 40 05 00 00        	movaps	0x540(%rbp), %xmm1
1403b9e2d: 0f 28 95 50 05 00 00        	movaps	0x550(%rbp), %xmm2
1403b9e34: 0f 28 9d 60 05 00 00        	movaps	0x560(%rbp), %xmm3
1403b9e3b: 0f 29 85 00 04 00 00        	movaps	%xmm0, 0x400(%rbp)
1403b9e42: 0f 29 8d 10 04 00 00        	movaps	%xmm1, 0x410(%rbp)
1403b9e49: 0f 29 95 20 04 00 00        	movaps	%xmm2, 0x420(%rbp)
1403b9e50: 0f 29 9d 30 04 00 00        	movaps	%xmm3, 0x430(%rbp)
1403b9e57: 0f 28 85 70 05 00 00        	movaps	0x570(%rbp), %xmm0
1403b9e5e: 0f 28 8d 80 05 00 00        	movaps	0x580(%rbp), %xmm1
1403b9e65: 0f 28 95 90 05 00 00        	movaps	0x590(%rbp), %xmm2
1403b9e6c: 0f 28 9d a0 05 00 00        	movaps	0x5a0(%rbp), %xmm3
1403b9e73: 0f 29 9d 70 04 00 00        	movaps	%xmm3, 0x470(%rbp)
1403b9e7a: 0f 29 95 60 04 00 00        	movaps	%xmm2, 0x460(%rbp)
1403b9e81: 0f 29 8d 50 04 00 00        	movaps	%xmm1, 0x450(%rbp)
1403b9e88: 0f 29 85 40 04 00 00        	movaps	%xmm0, 0x440(%rbp)
1403b9e8f: 8b 85 dc 06 00 00           	movl	0x6dc(%rbp), %eax
1403b9e95: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
1403b9e9c: 89 01                       	movl	%eax, (%rcx)
1403b9e9e: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403b9ea5: 41 b8 00 01 00 00           	movl	$0x100, %r8d            # imm = 0x100
1403b9eab: 48 8b 8d f8 06 00 00        	movq	0x6f8(%rbp), %rcx
1403b9eb2: e8 49 9d 56 02              	callq	0x142923c00 <.text+0x2922c00>
1403b9eb7: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403b9ebe: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1403b9ec2: 48 69 40 10 b0 01 00 00     	imulq	$0x1b0, 0x10(%rax), %rax # imm = 0x1B0
1403b9eca: 48 89 8d 70 03 00 00        	movq	%rcx, 0x370(%rbp)
1403b9ed1: 48 01 c8                    	addq	%rcx, %rax
1403b9ed4: 48 89 85 00 06 00 00        	movq	%rax, 0x600(%rbp)
1403b9edb: f3 0f 10 35 f1 33 61 02     	movss	0x26133f1(%rip), %xmm6  # 0x1429cd2d4
1403b9ee3: f3 44 0f 10 05 a8 a7 57 02  	movss	0x257a7a8(%rip), %xmm8  # 0x142934694
1403b9eec: 45 0f 57 c9                 	xorps	%xmm9, %xmm9
1403b9ef0: f3 44 0f 10 1d 57 ba 56 02  	movss	0x256ba57(%rip), %xmm11 # 0x142925950
1403b9ef9: f3 44 0f 10 25 fe b9 56 02  	movss	0x256b9fe(%rip), %xmm12 # 0x142925900
1403b9f02: f3 44 0f 10 2d c5 33 61 02  	movss	0x26133c5(%rip), %xmm13 # 0x1429cd2d0
1403b9f0b: f3 44 0f 10 35 5c 6e 57 02  	movss	0x2576e5c(%rip), %xmm14 # 0x142930d70
1403b9f14: 48 c7 85 40 07 00 00 00 00 00 00    	movq	$0x0, 0x740(%rbp)
1403b9f1f: 90                          	nop
1403b9f20: 48 8b 85 70 03 00 00        	movq	0x370(%rbp), %rax
1403b9f27: 48 3b 85 00 06 00 00        	cmpq	0x600(%rbp), %rax
1403b9f2e: 0f 84 0b 06 00 00           	je	0x1403ba53f <.text+0x3b953f>
1403b9f34: 48 8b 8d 40 07 00 00        	movq	0x740(%rbp), %rcx
1403b9f3b: 48 85 c9                    	testq	%rcx, %rcx
1403b9f3e: 4c 8b 85 80 07 00 00        	movq	0x780(%rbp), %r8
1403b9f45: 48 89 85 70 07 00 00        	movq	%rax, 0x770(%rbp)
1403b9f4c: 78 12                       	js	0x1403b9f60 <.text+0x3b8f60>
1403b9f4e: 0f 57 d2                    	xorps	%xmm2, %xmm2
1403b9f51: f3 48 0f 2a d1              	cvtsi2ss	%rcx, %xmm2
1403b9f56: eb 20                       	jmp	0x1403b9f78 <.text+0x3b8f78>
1403b9f58: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1403b9f60: 48 89 c8                    	movq	%rcx, %rax
1403b9f63: 48 d1 e8                    	shrq	%rax
1403b9f66: 83 e1 01                    	andl	$0x1, %ecx
1403b9f69: 48 09 c1                    	orq	%rax, %rcx
1403b9f6c: 0f 57 d2                    	xorps	%xmm2, %xmm2
1403b9f6f: f3 48 0f 2a d1              	cvtsi2ss	%rcx, %xmm2
1403b9f74: f3 0f 58 d2                 	addss	%xmm2, %xmm2
1403b9f78: 48 8b 95 60 07 00 00        	movq	0x760(%rbp), %rdx
1403b9f7f: f3 0f 59 15 dd 30 61 02     	mulss	0x26130dd(%rip), %xmm2  # 0x1429cd064
1403b9f87: f3 41 0f 58 50 40           	addss	0x40(%r8), %xmm2
1403b9f8d: 48 8d 8d 74 06 00 00        	leaq	0x674(%rbp), %rcx
1403b9f94: e8 07 a7 8d 00              	callq	0x140c946a0 <.text+0xc936a0>
1403b9f99: 48 8b 8d 40 07 00 00        	movq	0x740(%rbp), %rcx
1403b9fa0: 48 3b 8d 38 07 00 00        	cmpq	0x738(%rbp), %rcx
1403b9fa7: 0f 83 f7 20 00 00           	jae	0x1403bc0a4 <.text+0x3bb0a4>
1403b9fad: 48 8b 85 18 07 00 00        	movq	0x718(%rbp), %rax
1403b9fb4: 0f b6 04 08                 	movzbl	(%rax,%rcx), %eax
1403b9fb8: 88 85 f8 05 00 00           	movb	%al, 0x5f8(%rbp)
1403b9fbe: 48 ff c1                    	incq	%rcx
1403b9fc1: 48 89 8d 40 07 00 00        	movq	%rcx, 0x740(%rbp)
1403b9fc8: 4c 8b 95 70 07 00 00        	movq	0x770(%rbp), %r10
1403b9fcf: 41 83 7a 68 02              	cmpl	$0x2, 0x68(%r10)
1403b9fd4: 49 8d 42 68                 	leaq	0x68(%r10), %rax
1403b9fd8: 48 89 85 00 07 00 00        	movq	%rax, 0x700(%rbp)
1403b9fdf: 41 b9 00 00 00 00           	movl	$0x0, %r9d
1403b9fe5: 49 0f 44 c1                 	cmoveq	%r9, %rax
1403b9fe9: 49 8d 8a b0 01 00 00        	leaq	0x1b0(%r10), %rcx
1403b9ff0: 48 89 8d 70 03 00 00        	movq	%rcx, 0x370(%rbp)
1403b9ff7: 49 8d 8a a0 00 00 00        	leaq	0xa0(%r10), %rcx
1403b9ffe: 41 83 ba a0 00 00 00 02     	cmpl	$0x2, 0xa0(%r10)
1403ba006: 49 0f 44 c9                 	cmoveq	%r9, %rcx
1403ba00a: 41 83 7a 30 02              	cmpl	$0x2, 0x30(%r10)
1403ba00f: 49 8d 52 30                 	leaq	0x30(%r10), %rdx
1403ba013: 48 89 95 68 07 00 00        	movq	%rdx, 0x768(%rbp)
1403ba01a: 49 0f 44 d1                 	cmoveq	%r9, %rdx
1403ba01e: 4d 8d 82 d8 00 00 00        	leaq	0xd8(%r10), %r8
1403ba025: 41 83 ba d8 00 00 00 02     	cmpl	$0x2, 0xd8(%r10)
1403ba02d: 4c 89 85 58 07 00 00        	movq	%r8, 0x758(%rbp)
1403ba034: 4d 0f 44 c1                 	cmoveq	%r9, %r8
1403ba038: 4d 3b 8a 40 01 00 00        	cmpq	0x140(%r10), %r9
1403ba03f: 49 8d 9a 40 01 00 00        	leaq	0x140(%r10), %rbx
1403ba046: 48 c7 85 80 03 00 00 01 00 00 00    	movq	$0x1, 0x380(%rbp)
1403ba051: 48 89 85 88 03 00 00        	movq	%rax, 0x388(%rbp)
1403ba058: 48 89 de                    	movq	%rbx, %rsi
1403ba05b: 49 0f 40 f1                 	cmovoq	%r9, %rsi
1403ba05f: 48 89 8d 90 03 00 00        	movq	%rcx, 0x390(%rbp)
1403ba066: 48 89 95 98 03 00 00        	movq	%rdx, 0x398(%rbp)
1403ba06d: 4c 89 85 a0 03 00 00        	movq	%r8, 0x3a0(%rbp)
1403ba074: 48 c7 85 a8 03 00 00 00 00 00 00    	movq	$0x0, 0x3a8(%rbp)
1403ba07f: 48 c7 85 b0 03 00 00 04 00 00 00    	movq	$0x4, 0x3b0(%rbp)
1403ba08a: 48 c7 85 b8 03 00 00 00 00 00 00    	movq	$0x0, 0x3b8(%rbp)
1403ba095: 48 c7 85 c8 03 00 00 00 00 00 00    	movq	$0x0, 0x3c8(%rbp)
1403ba0a0: 48 c7 85 d8 03 00 00 01 00 00 00    	movq	$0x1, 0x3d8(%rbp)
1403ba0ab: 48 89 b5 e0 03 00 00        	movq	%rsi, 0x3e0(%rbp)
1403ba0b2: 48 c7 85 e8 03 00 00 00 00 00 00    	movq	$0x0, 0x3e8(%rbp)
1403ba0bd: 48 c7 85 f8 03 00 00 00 00 00 00    	movq	$0x0, 0x3f8(%rbp)
1403ba0c8: 41 bc 01 00 00 00           	movl	$0x1, %r12d
1403ba0ce: 31 ff                       	xorl	%edi, %edi
1403ba0d0: 45 31 f6                    	xorl	%r14d, %r14d
1403ba0d3: eb 53                       	jmp	0x1403ba128 <.text+0x3b9128>
1403ba0d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403ba0e0: 48 89 f9                    	movq	%rdi, %rcx
1403ba0e3: 48 83 c7 38                 	addq	$0x38, %rdi
1403ba0e7: 48 89 85 f0 03 00 00        	movq	%rax, 0x3f0(%rbp)
1403ba0ee: 48 89 bd e8 03 00 00        	movq	%rdi, 0x3e8(%rbp)
1403ba0f5: 48 89 b5 e0 03 00 00        	movq	%rsi, 0x3e0(%rbp)
1403ba0fc: 41 bc 02 00 00 00           	movl	$0x2, %r12d
1403ba102: 8b 85 7c 06 00 00           	movl	0x67c(%rbp), %eax
1403ba108: 89 85 b8 00 00 00           	movl	%eax, 0xb8(%rbp)
1403ba10e: 48 8b 85 74 06 00 00        	movq	0x674(%rbp), %rax
1403ba115: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1403ba11c: 48 8d 95 b0 00 00 00        	leaq	0xb0(%rbp), %rdx
1403ba123: e8 78 dc 69 00              	callq	0x140a57da0 <.text+0xa56da0>
1403ba128: 49 83 fc 02                 	cmpq	$0x2, %r12
1403ba12c: 74 49                       	je	0x1403ba177 <.text+0x3b9177>
1403ba12e: 41 f6 c4 01                 	testb	$0x1, %r12b
1403ba132: 74 31                       	je	0x1403ba165 <.text+0x3b9165>
1403ba134: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1403ba140: 49 83 fe 04                 	cmpq	$0x4, %r14
1403ba144: 74 19                       	je	0x1403ba15f <.text+0x3b915f>
1403ba146: 4a 8b 8c f5 88 03 00 00     	movq	0x388(%rbp,%r14,8), %rcx
1403ba14e: 49 ff c6                    	incq	%r14
1403ba151: 48 85 c9                    	testq	%rcx, %rcx
1403ba154: 74 ea                       	je	0x1403ba140 <.text+0x3b9140>
1403ba156: 4c 89 b5 a8 03 00 00        	movq	%r14, 0x3a8(%rbp)
1403ba15d: eb a3                       	jmp	0x1403ba102 <.text+0x3b9102>
1403ba15f: 41 be 04 00 00 00           	movl	$0x4, %r14d
1403ba165: 4c 89 b5 a8 03 00 00        	movq	%r14, 0x3a8(%rbp)
1403ba16c: 48 c7 85 80 03 00 00 02 00 00 00    	movq	$0x2, 0x380(%rbp)
1403ba177: 48 8b 85 f0 03 00 00        	movq	0x3f0(%rbp), %rax
1403ba17e: 48 85 ff                    	testq	%rdi, %rdi
1403ba181: 0f 94 c1                    	sete	%cl
1403ba184: 48 39 c7                    	cmpq	%rax, %rdi
1403ba187: 0f 94 c2                    	sete	%dl
1403ba18a: 08 ca                       	orb	%cl, %dl
1403ba18c: 80 fa 01                    	cmpb	$0x1, %dl
1403ba18f: 0f 85 4b ff ff ff           	jne	0x1403ba0e0 <.text+0x3b90e0>
1403ba195: 48 85 f6                    	testq	%rsi, %rsi
1403ba198: 74 26                       	je	0x1403ba1c0 <.text+0x3b91c0>
1403ba19a: 48 8b 46 10                 	movq	0x10(%rsi), %rax
1403ba19e: 48 85 c0                    	testq	%rax, %rax
1403ba1a1: 74 1d                       	je	0x1403ba1c0 <.text+0x3b91c0>
1403ba1a3: 48 8b 7e 08                 	movq	0x8(%rsi), %rdi
1403ba1a7: 48 6b c0 38                 	imulq	$0x38, %rax, %rax
1403ba1ab: 48 01 f8                    	addq	%rdi, %rax
1403ba1ae: 31 f6                       	xorl	%esi, %esi
1403ba1b0: e9 2b ff ff ff              	jmp	0x1403ba0e0 <.text+0x3b90e0>
1403ba1b5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403ba1c0: f3 44 0f 10 95 78 06 00 00  	movss	0x678(%rbp), %xmm10
1403ba1c9: 48 8b 85 00 07 00 00        	movq	0x700(%rbp), %rax
1403ba1d0: 83 38 02                    	cmpl	$0x2, (%rax)
1403ba1d3: 74 72                       	je	0x1403ba247 <.text+0x3b9247>
1403ba1d5: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403ba1dc: f3 0f 10 40 44              	movss	0x44(%rax), %xmm0
1403ba1e1: f3 41 0f 59 c5              	mulss	%xmm13, %xmm0
1403ba1e6: 0f 57 d2                    	xorps	%xmm2, %xmm2
1403ba1e9: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
1403ba1ed: 0f 28 cf                    	movaps	%xmm7, %xmm1
1403ba1f0: f3 0f 5d ca                 	minss	%xmm2, %xmm1
1403ba1f4: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403ba1fb: f3 0f 59 88 98 00 00 00     	mulss	0x98(%rax), %xmm1
1403ba203: 48 8b 8d 00 07 00 00        	movq	0x700(%rbp), %rcx
1403ba20a: e8 91 d3 69 00              	callq	0x140a575a0 <.text+0xa565a0>
1403ba20f: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403ba216: f3 0f 10 40 44              	movss	0x44(%rax), %xmm0
1403ba21b: f3 41 0f 59 c6              	mulss	%xmm14, %xmm0
1403ba220: 0f 57 c9                    	xorps	%xmm1, %xmm1
1403ba223: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1403ba227: 0f 28 df                    	movaps	%xmm7, %xmm3
1403ba22a: f3 0f 5d d9                 	minss	%xmm1, %xmm3
1403ba22e: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1403ba234: 48 8b 8d 00 07 00 00        	movq	0x700(%rbp), %rcx
1403ba23b: 48 8d 15 51 35 61 02        	leaq	0x2613551(%rip), %rdx   # 0x1429cd793
1403ba242: e8 29 d4 69 00              	callq	0x140a57670 <.text+0xa56670>
1403ba247: 80 bd f8 05 00 00 00        	cmpb	$0x0, 0x5f8(%rbp)
1403ba24e: 0f 84 9c 00 00 00           	je	0x1403ba2f0 <.text+0x3b92f0>
1403ba254: 31 c0                       	xorl	%eax, %eax
1403ba256: 48 3b 03                    	cmpq	(%rbx), %rax
1403ba259: 48 0f 40 d8                 	cmovoq	%rax, %rbx
1403ba25d: 31 c9                       	xorl	%ecx, %ecx
1403ba25f: 90                          	nop
1403ba260: 48 85 c9                    	testq	%rcx, %rcx
1403ba263: 74 0b                       	je	0x1403ba270 <.text+0x3b9270>
1403ba265: 4c 39 f9                    	cmpq	%r15, %rcx
1403ba268: 74 06                       	je	0x1403ba270 <.text+0x3b9270>
1403ba26a: 49 89 ce                    	movq	%rcx, %r14
1403ba26d: eb 1c                       	jmp	0x1403ba28b <.text+0x3b928b>
1403ba26f: 90                          	nop
1403ba270: 48 85 db                    	testq	%rbx, %rbx
1403ba273: 74 7b                       	je	0x1403ba2f0 <.text+0x3b92f0>
1403ba275: 4c 8b 73 08                 	movq	0x8(%rbx), %r14
1403ba279: 48 8b 43 10                 	movq	0x10(%rbx), %rax
1403ba27d: 4c 6b f8 38                 	imulq	$0x38, %rax, %r15
1403ba281: 4d 01 f7                    	addq	%r14, %r15
1403ba284: 48 85 c0                    	testq	%rax, %rax
1403ba287: 74 67                       	je	0x1403ba2f0 <.text+0x3b92f0>
1403ba289: 31 db                       	xorl	%ebx, %ebx
1403ba28b: 4c 89 f1                    	movq	%r14, %rcx
1403ba28e: e8 bd d2 69 00              	callq	0x140a57550 <.text+0xa56550>
1403ba293: 49 8d 4e 38                 	leaq	0x38(%r14), %rcx
1403ba297: 84 c0                       	testb	%al, %al
1403ba299: 75 c5                       	jne	0x1403ba260 <.text+0x3b9260>
1403ba29b: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403ba2a2: f3 0f 10 80 a0 01 00 00     	movss	0x1a0(%rax), %xmm0
1403ba2aa: f3 0f 59 c6                 	mulss	%xmm6, %xmm0
1403ba2ae: f3 0f 10 15 ba b5 56 02     	movss	0x256b5ba(%rip), %xmm2  # 0x142925870
1403ba2b6: f3 0f 5f d0                 	maxss	%xmm0, %xmm2
1403ba2ba: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
1403ba2be: f3 0f 5d ca                 	minss	%xmm2, %xmm1
1403ba2c2: f3 41 0f 59 4e 30           	mulss	0x30(%r14), %xmm1
1403ba2c8: 4c 89 f1                    	movq	%r14, %rcx
1403ba2cb: e8 d0 d2 69 00              	callq	0x140a575a0 <.text+0xa565a0>
1403ba2d0: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403ba2d7: c7 80 a0 01 00 00 00 00 00 00       	movl	$0x0, 0x1a0(%rax)
1403ba2e1: 4c 89 f1                    	movq	%r14, %rcx
1403ba2e4: e8 17 d0 69 00              	callq	0x140a57300 <.text+0xa56300>
1403ba2e9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403ba2f0: 48 8b 85 58 07 00 00        	movq	0x758(%rbp), %rax
1403ba2f7: 83 38 02                    	cmpl	$0x2, (%rax)
1403ba2fa: 74 20                       	je	0x1403ba31c <.text+0x3b931c>
1403ba2fc: 45 0f 2e e2                 	ucomiss	%xmm10, %xmm12
1403ba300: 76 0e                       	jbe	0x1403ba310 <.text+0x3b9310>
1403ba302: 48 8b 8d 58 07 00 00        	movq	0x758(%rbp), %rcx
1403ba309: e8 f2 cf 69 00              	callq	0x140a57300 <.text+0xa56300>
1403ba30e: eb 0c                       	jmp	0x1403ba31c <.text+0x3b931c>
1403ba310: 48 8b 8d 58 07 00 00        	movq	0x758(%rbp), %rcx
1403ba317: e8 54 d1 69 00              	callq	0x140a57470 <.text+0xa56470>
1403ba31c: 4c 8b 85 70 07 00 00        	movq	0x770(%rbp), %r8
1403ba323: f3 41 0f 10 80 60 01 00 00  	movss	0x160(%r8), %xmm0
1403ba32c: 44 0f 2e c8                 	ucomiss	%xmm0, %xmm9
1403ba330: 0f 93 c0                    	setae	%al
1403ba333: 84 85 10 06 00 00           	testb	%al, 0x610(%rbp)
1403ba339: 0f 84 51 01 00 00           	je	0x1403ba490 <.text+0x3b9490>
1403ba33f: 31 c0                       	xorl	%eax, %eax
1403ba341: 49 3b 80 10 01 00 00        	cmpq	0x110(%r8), %rax
1403ba348: 70 71                       	jo	0x1403ba3bb <.text+0x3b93bb>
1403ba34a: 49 8b 90 20 01 00 00        	movq	0x120(%r8), %rdx
1403ba351: 49 8b 88 58 01 00 00        	movq	0x158(%r8), %rcx
1403ba358: 48 39 d1                    	cmpq	%rdx, %rcx
1403ba35b: 0f 83 46 24 00 00           	jae	0x1403bc7a7 <.text+0x3bb7a7>
1403ba361: 49 8b 80 18 01 00 00        	movq	0x118(%r8), %rax
1403ba368: 48 6b c9 38                 	imulq	$0x38, %rcx, %rcx
1403ba36c: 4c 8d 34 08                 	leaq	(%rax,%rcx), %r14
1403ba370: f3 0f 10 4c 08 30           	movss	0x30(%rax,%rcx), %xmm1
1403ba376: f3 41 0f 59 cb              	mulss	%xmm11, %xmm1
1403ba37b: 4c 89 f1                    	movq	%r14, %rcx
1403ba37e: e8 1d d2 69 00              	callq	0x140a575a0 <.text+0xa565a0>
1403ba383: 4c 89 f1                    	movq	%r14, %rcx
1403ba386: e8 75 cf 69 00              	callq	0x140a57300 <.text+0xa56300>
1403ba38b: 8b 85 7c 06 00 00           	movl	0x67c(%rbp), %eax
1403ba391: 89 85 88 03 00 00           	movl	%eax, 0x388(%rbp)
1403ba397: 48 8b 85 74 06 00 00        	movq	0x674(%rbp), %rax
1403ba39e: 48 89 85 80 03 00 00        	movq	%rax, 0x380(%rbp)
1403ba3a5: 4c 89 f1                    	movq	%r14, %rcx
1403ba3a8: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403ba3af: e8 ec d9 69 00              	callq	0x140a57da0 <.text+0xa56da0>
1403ba3b4: 4c 8b 85 70 07 00 00        	movq	0x770(%rbp), %r8
1403ba3bb: 31 c0                       	xorl	%eax, %eax
1403ba3bd: 49 3b 80 28 01 00 00        	cmpq	0x128(%r8), %rax
1403ba3c4: 70 57                       	jo	0x1403ba41d <.text+0x3b941d>
1403ba3c6: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403ba3cd: 48 8b 90 38 01 00 00        	movq	0x138(%rax), %rdx
1403ba3d4: 48 8b 88 58 01 00 00        	movq	0x158(%rax), %rcx
1403ba3db: 48 39 d1                    	cmpq	%rdx, %rcx
1403ba3de: 0f 83 d4 23 00 00           	jae	0x1403bc7b8 <.text+0x3bb7b8>
1403ba3e4: 48 8b 80 30 01 00 00        	movq	0x130(%rax), %rax
1403ba3eb: 48 6b c9 38                 	imulq	$0x38, %rcx, %rcx
1403ba3ef: 4c 8d 34 08                 	leaq	(%rax,%rcx), %r14
1403ba3f3: f3 0f 10 4c 08 30           	movss	0x30(%rax,%rcx), %xmm1
1403ba3f9: f3 41 0f 59 cb              	mulss	%xmm11, %xmm1
1403ba3fe: 4c 89 f1                    	movq	%r14, %rcx
1403ba401: e8 9a d1 69 00              	callq	0x140a575a0 <.text+0xa565a0>
1403ba406: 4c 89 f1                    	movq	%r14, %rcx
1403ba409: e8 f2 ce 69 00              	callq	0x140a57300 <.text+0xa56300>
1403ba40e: 4c 89 f1                    	movq	%r14, %rcx
1403ba411: 48 8d 95 74 06 00 00        	leaq	0x674(%rbp), %rdx
1403ba418: e8 83 d9 69 00              	callq	0x140a57da0 <.text+0xa56da0>
1403ba41d: 4c 8b 85 70 07 00 00        	movq	0x770(%rbp), %r8
1403ba424: 49 8b 80 58 01 00 00        	movq	0x158(%r8), %rax
1403ba42b: 48 ff c0                    	incq	%rax
1403ba42e: 49 89 80 58 01 00 00        	movq	%rax, 0x158(%r8)
1403ba435: 41 c7 80 60 01 00 00 00 00 80 3e    	movl	$0x3e800000, 0x160(%r8) # imm = 0x3E800000
1403ba440: 31 c9                       	xorl	%ecx, %ecx
1403ba442: 49 3b 88 10 01 00 00        	cmpq	0x110(%r8), %rcx
1403ba449: 70 56                       	jo	0x1403ba4a1 <.text+0x3b94a1>
1403ba44b: 49 8b 88 20 01 00 00        	movq	0x120(%r8), %rcx
1403ba452: 48 85 c9                    	testq	%rcx, %rcx
1403ba455: 0f 84 6e 23 00 00           	je	0x1403bc7c9 <.text+0x3bb7c9>
1403ba45b: 48 89 c2                    	movq	%rax, %rdx
1403ba45e: 48 09 ca                    	orq	%rcx, %rdx
1403ba461: 48 c1 ea 20                 	shrq	$0x20, %rdx
1403ba465: 0f 84 b4 00 00 00           	je	0x1403ba51f <.text+0x3b951f>
1403ba46b: 31 d2                       	xorl	%edx, %edx
1403ba46d: 48 f7 f1                    	divq	%rcx
1403ba470: 49 89 90 58 01 00 00        	movq	%rdx, 0x158(%r8)
1403ba477: 48 8b 85 68 07 00 00        	movq	0x768(%rbp), %rax
1403ba47e: 83 38 02                    	cmpl	$0x2, (%rax)
1403ba481: 0f 84 99 fa ff ff           	je	0x1403b9f20 <.text+0x3b8f20>
1403ba487: eb 28                       	jmp	0x1403ba4b1 <.text+0x3b94b1>
1403ba489: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403ba490: f3 0f 5c 85 24 07 00 00     	subss	0x724(%rbp), %xmm0
1403ba498: f3 41 0f 11 80 60 01 00 00  	movss	%xmm0, 0x160(%r8)
1403ba4a1: 48 8b 85 68 07 00 00        	movq	0x768(%rbp), %rax
1403ba4a8: 83 38 02                    	cmpl	$0x2, (%rax)
1403ba4ab: 0f 84 6f fa ff ff           	je	0x1403b9f20 <.text+0x3b8f20>
1403ba4b1: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403ba4b8: f3 44 0f 10 90 94 01 00 00  	movss	0x194(%rax), %xmm10
1403ba4c1: 41 b8 0d 00 00 00           	movl	$0xd, %r8d
1403ba4c7: 48 8b 8d 68 07 00 00        	movq	0x768(%rbp), %rcx
1403ba4ce: 48 8d 15 be 32 61 02        	leaq	0x26132be(%rip), %rdx   # 0x1429cd793
1403ba4d5: 0f 28 df                    	movaps	%xmm7, %xmm3
1403ba4d8: e8 93 d1 69 00              	callq	0x140a57670 <.text+0xa56670>
1403ba4dd: 44 0f 54 15 9b 44 57 02     	andps	0x257449b(%rip), %xmm10 # 0x14292e980
1403ba4e5: f3 0f 10 0d 03 42 57 02     	movss	0x2574203(%rip), %xmm1  # 0x14292e6f0
1403ba4ed: f3 41 0f 5d ca              	minss	%xmm10, %xmm1
1403ba4f2: f3 0f 59 0d 76 b4 56 02     	mulss	0x256b476(%rip), %xmm1  # 0x142925970
1403ba4fa: f3 0f 59 0d 6e b3 56 02     	mulss	0x256b36e(%rip), %xmm1  # 0x142925870
1403ba502: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403ba509: f3 0f 59 48 60              	mulss	0x60(%rax), %xmm1
1403ba50e: 48 8b 8d 68 07 00 00        	movq	0x768(%rbp), %rcx
1403ba515: e8 86 d0 69 00              	callq	0x140a575a0 <.text+0xa565a0>
1403ba51a: e9 01 fa ff ff              	jmp	0x1403b9f20 <.text+0x3b8f20>
1403ba51f: 31 d2                       	xorl	%edx, %edx
1403ba521: f7 f1                       	divl	%ecx
1403ba523: 49 89 90 58 01 00 00        	movq	%rdx, 0x158(%r8)
1403ba52a: 48 8b 85 68 07 00 00        	movq	0x768(%rbp), %rax
1403ba531: 83 38 02                    	cmpl	$0x2, (%rax)
1403ba534: 0f 84 e6 f9 ff ff           	je	0x1403b9f20 <.text+0x3b8f20>
1403ba53a: e9 72 ff ff ff              	jmp	0x1403ba4b1 <.text+0x3b94b1>
1403ba53f: 48 8b 95 80 07 00 00        	movq	0x780(%rbp), %rdx
1403ba546: 48 8b 42 10                 	movq	0x10(%rdx), %rax
1403ba54a: 48 ff c8                    	decq	%rax
1403ba54d: 78 0a                       	js	0x1403ba559 <.text+0x3b9559>
1403ba54f: 0f 57 f6                    	xorps	%xmm6, %xmm6
1403ba552: f3 48 0f 2a f0              	cvtsi2ss	%rax, %xmm6
1403ba557: eb 18                       	jmp	0x1403ba571 <.text+0x3b9571>
1403ba559: 48 89 c1                    	movq	%rax, %rcx
1403ba55c: 48 d1 e9                    	shrq	%rcx
1403ba55f: 83 e0 01                    	andl	$0x1, %eax
1403ba562: 48 09 c8                    	orq	%rcx, %rax
1403ba565: 0f 57 f6                    	xorps	%xmm6, %xmm6
1403ba568: f3 48 0f 2a f0              	cvtsi2ss	%rax, %xmm6
1403ba56d: f3 0f 58 f6                 	addss	%xmm6, %xmm6
1403ba571: 48 8b 85 68 03 00 00        	movq	0x368(%rbp), %rax
1403ba578: f3 0f 59 35 e4 2a 61 02     	mulss	0x2612ae4(%rip), %xmm6  # 0x1429cd064
1403ba580: f3 0f 58 72 40              	addss	0x40(%rdx), %xmm6
1403ba585: 8b 88 90 00 00 00           	movl	0x90(%rax), %ecx
1403ba58b: 48 89 c6                    	movq	%rax, %rsi
1403ba58e: f3 0f 10 88 94 00 00 00     	movss	0x94(%rax), %xmm1
1403ba596: e8 f5 54 58 00              	callq	0x14093fa90 <.text+0x93ea90>
1403ba59b: 88 c3                       	movb	%al, %bl
1403ba59d: 8b 8e 98 00 00 00           	movl	0x98(%rsi), %ecx
1403ba5a3: 66 0f 6e 8e 9c 00 00 00     	movd	0x9c(%rsi), %xmm1
1403ba5ab: e8 e0 54 58 00              	callq	0x14093fa90 <.text+0x93ea90>
1403ba5b0: 08 c3                       	orb	%al, %bl
1403ba5b2: 4c 8b 85 80 07 00 00        	movq	0x780(%rbp), %r8
1403ba5b9: 75 2e                       	jne	0x1403ba5e9 <.text+0x3b95e9>
1403ba5bb: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
1403ba5c2: 83 38 13                    	cmpl	$0x13, (%rax)
1403ba5c5: 75 22                       	jne	0x1403ba5e9 <.text+0x3b95e9>
1403ba5c7: 48 8b 8d f8 06 00 00        	movq	0x6f8(%rbp), %rcx
1403ba5ce: e8 5d 6c 55 00              	callq	0x140911230 <.text+0x910230>
1403ba5d3: 3c 03                       	cmpb	$0x3, %al
1403ba5d5: 4c 8b 85 80 07 00 00        	movq	0x780(%rbp), %r8
1403ba5dc: 75 0b                       	jne	0x1403ba5e9 <.text+0x3b95e9>
1403ba5de: 41 0f 2e 70 34              	ucomiss	0x34(%r8), %xmm6
1403ba5e3: 0f 82 30 07 00 00           	jb	0x1403bad19 <.text+0x3b9d19>
1403ba5e9: 49 8b 50 08                 	movq	0x8(%r8), %rdx
1403ba5ed: 49 8b 48 10                 	movq	0x10(%r8), %rcx
1403ba5f1: 49 c7 40 10 00 00 00 00     	movq	$0x0, 0x10(%r8)
1403ba5f9: 48 69 c1 b0 01 00 00        	imulq	$0x1b0, %rcx, %rax      # imm = 0x1B0
1403ba600: 48 01 d0                    	addq	%rdx, %rax
1403ba603: 48 89 95 90 06 00 00        	movq	%rdx, 0x690(%rbp)
1403ba60a: 48 89 85 98 06 00 00        	movq	%rax, 0x698(%rbp)
1403ba611: 4c 89 85 a0 06 00 00        	movq	%r8, 0x6a0(%rbp)
1403ba618: 48 89 8d a8 06 00 00        	movq	%rcx, 0x6a8(%rbp)
1403ba61f: 48 c7 85 b0 06 00 00 00 00 00 00    	movq	$0x0, 0x6b0(%rbp)
1403ba62a: 48 8d 9d b0 00 00 00        	leaq	0xb0(%rbp), %rbx
1403ba631: 4c 8d a5 30 05 00 00        	leaq	0x530(%rbp), %r12
1403ba638: 4c 8d 35 49 33 61 02        	leaq	0x2613349(%rip), %r14   # 0x1429cd988
1403ba63f: be ff ff ff ff              	movl	$0xffffffff, %esi       # imm = 0xFFFFFFFF
1403ba644: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1403ba648: 48 39 c2                    	cmpq	%rax, %rdx
1403ba64b: 0f 84 22 04 00 00           	je	0x1403baa73 <.text+0x3b9a73>
1403ba651: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1403ba660: 48 8d 82 b0 01 00 00        	leaq	0x1b0(%rdx), %rax
1403ba667: 48 89 85 90 06 00 00        	movq	%rax, 0x690(%rbp)
1403ba66e: 0f 28 02                    	movaps	(%rdx), %xmm0
1403ba671: 0f 28 4a 10                 	movaps	0x10(%rdx), %xmm1
1403ba675: 0f 28 52 20                 	movaps	0x20(%rdx), %xmm2
1403ba679: 0f 29 85 30 06 00 00        	movaps	%xmm0, 0x630(%rbp)
1403ba680: 0f 29 8d 40 06 00 00        	movaps	%xmm1, 0x640(%rbp)
1403ba687: 0f 29 95 50 06 00 00        	movaps	%xmm2, 0x650(%rbp)
1403ba68e: 48 8b 7a 30                 	movq	0x30(%rdx), %rdi
1403ba692: 48 83 c2 38                 	addq	$0x38, %rdx
1403ba696: 41 b8 78 01 00 00           	movl	$0x178, %r8d            # imm = 0x178
1403ba69c: 48 89 d9                    	movq	%rbx, %rcx
1403ba69f: e8 5c 95 56 02              	callq	0x142923c00 <.text+0x2922c00>
1403ba6a4: 48 83 ff 03                 	cmpq	$0x3, %rdi
1403ba6a8: 0f 84 c5 03 00 00           	je	0x1403baa73 <.text+0x3b9a73>
1403ba6ae: 66 0f 6f 85 30 06 00 00     	movdqa	0x630(%rbp), %xmm0
1403ba6b6: 66 0f 6f 8d 40 06 00 00     	movdqa	0x640(%rbp), %xmm1
1403ba6be: 66 0f 6f 95 50 06 00 00     	movdqa	0x650(%rbp), %xmm2
1403ba6c6: 66 0f 7f 95 a0 03 00 00     	movdqa	%xmm2, 0x3a0(%rbp)
1403ba6ce: 66 0f 7f 8d 90 03 00 00     	movdqa	%xmm1, 0x390(%rbp)
1403ba6d6: 66 0f 7f 85 80 03 00 00     	movdqa	%xmm0, 0x380(%rbp)
1403ba6de: 48 89 bd b0 03 00 00        	movq	%rdi, 0x3b0(%rbp)
1403ba6e5: 41 b8 78 01 00 00           	movl	$0x178, %r8d            # imm = 0x178
1403ba6eb: 48 8d 8d b8 03 00 00        	leaq	0x3b8(%rbp), %rcx
1403ba6f2: 48 89 da                    	movq	%rbx, %rdx
1403ba6f5: e8 06 95 56 02              	callq	0x142923c00 <.text+0x2922c00>
1403ba6fa: 44 8b bd f8 04 00 00        	movl	0x4f8(%rbp), %r15d
1403ba701: 44 89 bd c0 06 00 00        	movl	%r15d, 0x6c0(%rbp)
1403ba708: 48 8b 8d 10 03 00 00        	movq	0x310(%rbp), %rcx
1403ba70f: e8 6c 36 2c 00              	callq	0x14067dd80 <.text+0x67cd80>
1403ba714: 48 89 c7                    	movq	%rax, %rdi
1403ba717: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1403ba71c: 0f 84 ae 00 00 00           	je	0x1403ba7d0 <.text+0x3b97d0>
1403ba722: 48 89 f9                    	movq	%rdi, %rcx
1403ba725: 48 83 c1 20                 	addq	$0x20, %rcx
1403ba729: 48 8d 95 c0 06 00 00        	leaq	0x6c0(%rbp), %rdx
1403ba730: e8 9b 67 1f 00              	callq	0x1405b0ed0 <.text+0x5afed0>
1403ba735: 48 8b 0f                    	movq	(%rdi), %rcx
1403ba738: 48 8b 57 08                 	movq	0x8(%rdi), %rdx
1403ba73c: 49 89 d0                    	movq	%rdx, %r8
1403ba73f: 49 21 c0                    	andq	%rax, %r8
1403ba742: 48 c1 e8 39                 	shrq	$0x39, %rax
1403ba746: 66 0f 6e c0                 	movd	%eax, %xmm0
1403ba74a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1403ba74e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1403ba753: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1403ba758: 48 8d 41 d8                 	leaq	-0x28(%rcx), %rax
1403ba75c: 45 31 c9                    	xorl	%r9d, %r9d
1403ba75f: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
1403ba765: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1403ba769: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1403ba76d: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1403ba772: 45 85 d2                    	testl	%r10d, %r10d
1403ba775: 74 29                       	je	0x1403ba7a0 <.text+0x3b97a0>
1403ba777: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1403ba77c: 4d 01 c3                    	addq	%r8, %r11
1403ba77f: 49 21 d3                    	andq	%rdx, %r11
1403ba782: 49 f7 db                    	negq	%r11
1403ba785: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
1403ba789: 46 3b 3c d8                 	cmpl	(%rax,%r11,8), %r15d
1403ba78d: 74 31                       	je	0x1403ba7c0 <.text+0x3b97c0>
1403ba78f: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1403ba793: 66 45 21 d3                 	andw	%r10w, %r11w
1403ba797: 45 89 da                    	movl	%r11d, %r10d
1403ba79a: 75 db                       	jne	0x1403ba777 <.text+0x3b9777>
1403ba79c: 0f 1f 40 00                 	nopl	(%rax)
1403ba7a0: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
1403ba7a4: 66 44 0f d7 d1              	pmovmskb	%xmm1, %r10d
1403ba7a9: 45 85 d2                    	testl	%r10d, %r10d
1403ba7ac: 75 32                       	jne	0x1403ba7e0 <.text+0x3b97e0>
1403ba7ae: 4d 01 c8                    	addq	%r9, %r8
1403ba7b1: 49 83 c0 10                 	addq	$0x10, %r8
1403ba7b5: 49 83 c1 10                 	addq	$0x10, %r9
1403ba7b9: 49 21 d0                    	andq	%rdx, %r8
1403ba7bc: eb a1                       	jmp	0x1403ba75f <.text+0x3b975f>
1403ba7be: 66 90                       	nop
1403ba7c0: 4a 8d 0c d9                 	leaq	(%rcx,%r11,8), %rcx
1403ba7c4: eb 1c                       	jmp	0x1403ba7e2 <.text+0x3b97e2>
1403ba7c6: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1403ba7d0: 31 c0                       	xorl	%eax, %eax
1403ba7d2: eb 19                       	jmp	0x1403ba7ed <.text+0x3b97ed>
1403ba7d4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1403ba7e0: 31 c9                       	xorl	%ecx, %ecx
1403ba7e2: 48 8d 41 d8                 	leaq	-0x28(%rcx), %rax
1403ba7e6: 48 85 c9                    	testq	%rcx, %rcx
1403ba7e9: 48 0f 44 c1                 	cmoveq	%rcx, %rax
1403ba7ed: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
1403ba7f1: 48 85 c0                    	testq	%rax, %rax
1403ba7f4: 48 0f 44 c8                 	cmoveq	%rax, %rcx
1403ba7f8: 48 c7 85 30 05 00 00 01 00 00 00    	movq	$0x1, 0x530(%rbp)
1403ba803: 48 89 8d 38 05 00 00        	movq	%rcx, 0x538(%rbp)
1403ba80a: 48 c7 85 40 05 00 00 00 00 00 00    	movq	$0x0, 0x540(%rbp)
1403ba815: 48 c7 85 50 05 00 00 00 00 00 00    	movq	$0x0, 0x550(%rbp)
1403ba820: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
1403ba827: 4c 89 e2                    	movq	%r12, %rdx
1403ba82a: 4c 8d 05 07 70 64 02        	leaq	0x2647007(%rip), %r8    # 0x142a01838
1403ba831: e8 4a 59 f0 ff              	callq	0x1402c0180 <.text+0x2bf180>
1403ba836: 48 8b 95 b0 05 00 00        	movq	0x5b0(%rbp), %rdx
1403ba83d: 48 8b 8d b8 05 00 00        	movq	0x5b8(%rbp), %rcx
1403ba844: 48 8b 85 c0 05 00 00        	movq	0x5c0(%rbp), %rax
1403ba84b: 48 85 c0                    	testq	%rax, %rax
1403ba84e: 0f 84 2c 01 00 00           	je	0x1403ba980 <.text+0x3b9980>
1403ba854: 48 c1 e0 03                 	shlq	$0x3, %rax
1403ba858: 45 31 c0                    	xorl	%r8d, %r8d
1403ba85b: eb 10                       	jmp	0x1403ba86d <.text+0x3b986d>
1403ba85d: 0f 1f 00                    	nopl	(%rax)
1403ba860: 49 83 c0 08                 	addq	$0x8, %r8
1403ba864: 4c 39 c0                    	cmpq	%r8, %rax
1403ba867: 0f 84 13 01 00 00           	je	0x1403ba980 <.text+0x3b9980>
1403ba86d: 4e 8b 14 01                 	movq	(%rcx,%r8), %r10
1403ba871: 4d 89 d1                    	movq	%r10, %r9
1403ba874: 49 c1 e9 20                 	shrq	$0x20, %r9
1403ba878: 0f 84 02 01 00 00           	je	0x1403ba980 <.text+0x3b9980>
1403ba87e: 45 89 d2                    	movl	%r10d, %r10d
1403ba881: 4d 3b 55 10                 	cmpq	0x10(%r13), %r10
1403ba885: 73 d9                       	jae	0x1403ba860 <.text+0x3b9860>
1403ba887: 4d 8b 5d 08                 	movq	0x8(%r13), %r11
1403ba88b: 4f 8d 14 92                 	leaq	(%r10,%r10,4), %r10
1403ba88f: 47 39 0c 93                 	cmpl	%r9d, (%r11,%r10,4)
1403ba893: 75 cb                       	jne	0x1403ba860 <.text+0x3b9860>
1403ba895: 4f 8d 0c 93                 	leaq	(%r11,%r10,4), %r9
1403ba899: 45 8b 51 04                 	movl	0x4(%r9), %r10d
1403ba89d: 49 39 f2                    	cmpq	%rsi, %r10
1403ba8a0: 74 be                       	je	0x1403ba860 <.text+0x3b9860>
1403ba8a2: 4c 8b 9d 88 07 00 00        	movq	0x788(%rbp), %r11
1403ba8a9: 4d 39 53 38                 	cmpq	%r10, 0x38(%r11)
1403ba8ad: 76 b1                       	jbe	0x1403ba860 <.text+0x3b9860>
1403ba8af: 4c 8b 9d 88 07 00 00        	movq	0x788(%rbp), %r11
1403ba8b6: 4d 8b 5b 28                 	movq	0x28(%r11), %r11
1403ba8ba: 44 89 d7                    	movl	%r10d, %edi
1403ba8bd: c1 ef 06                    	shrl	$0x6, %edi
1403ba8c0: 4d 8b 1c fb                 	movq	(%r11,%rdi,8), %r11
1403ba8c4: 4d 0f a3 d3                 	btq	%r10, %r11
1403ba8c8: 73 96                       	jae	0x1403ba860 <.text+0x3b9860>
1403ba8ca: 45 8b 51 0c                 	movl	0xc(%r9), %r10d
1403ba8ce: 45 8b 49 10                 	movl	0x10(%r9), %r9d
1403ba8d2: 4d 8b 9d a8 01 00 00        	movq	0x1a8(%r13), %r11
1403ba8d9: 4f 8d 14 d2                 	leaq	(%r10,%r10,8), %r10
1403ba8dd: 48 8b bd 88 07 00 00        	movq	0x788(%rbp), %rdi
1403ba8e4: 48 8b bf 10 01 00 00        	movq	0x110(%rdi), %rdi
1403ba8eb: 4f 8b 7c d3 18              	movq	0x18(%r11,%r10,8), %r15
1403ba8f0: 4f 8b 54 d3 38              	movq	0x38(%r11,%r10,8), %r10
1403ba8f5: 4d 8b 14 fa                 	movq	(%r10,%rdi,8), %r10
1403ba8f9: 49 f7 d2                    	notq	%r10
1403ba8fc: 4f 8d 14 52                 	leaq	(%r10,%r10,2), %r10
1403ba900: 49 c1 e2 04                 	shlq	$0x4, %r10
1403ba904: 4f 8b 5c 17 10              	movq	0x10(%r15,%r10), %r11
1403ba909: 4f 8b 54 17 28              	movq	0x28(%r15,%r10), %r10
1403ba90e: 4b 8d 3c 49                 	leaq	(%r9,%r9,2), %rdi
1403ba912: 48 c1 e7 04                 	shlq	$0x4, %rdi
1403ba916: 0f 28 85 80 03 00 00        	movaps	0x380(%rbp), %xmm0
1403ba91d: 0f 28 8d 90 03 00 00        	movaps	0x390(%rbp), %xmm1
1403ba924: 0f 28 95 a0 03 00 00        	movaps	0x3a0(%rbp), %xmm2
1403ba92b: 0f 29 95 50 05 00 00        	movaps	%xmm2, 0x550(%rbp)
1403ba932: 0f 29 8d 40 05 00 00        	movaps	%xmm1, 0x540(%rbp)
1403ba939: 0f 29 85 30 05 00 00        	movaps	%xmm0, 0x530(%rbp)
1403ba940: 44 8b bd 28 07 00 00        	movl	0x728(%rbp), %r15d
1403ba947: 47 89 3c 8a                 	movl	%r15d, (%r10,%r9,4)
1403ba94b: 66 0f 6f 85 30 05 00 00     	movdqa	0x530(%rbp), %xmm0
1403ba953: 66 0f 6f 8d 40 05 00 00     	movdqa	0x540(%rbp), %xmm1
1403ba95b: 66 0f 6f 95 50 05 00 00     	movdqa	0x550(%rbp), %xmm2
1403ba963: 66 41 0f 7f 54 3b 20        	movdqa	%xmm2, 0x20(%r11,%rdi)
1403ba96a: 66 41 0f 7f 4c 3b 10        	movdqa	%xmm1, 0x10(%r11,%rdi)
1403ba971: 66 41 0f 7f 04 3b           	movdqa	%xmm0, (%r11,%rdi)
1403ba977: e9 e4 fe ff ff              	jmp	0x1403ba860 <.text+0x3b9860>
1403ba97c: 0f 1f 40 00                 	nopl	(%rax)
1403ba980: 48 85 d2                    	testq	%rdx, %rdx
1403ba983: 74 0f                       	je	0x1403ba994 <.text+0x3b9994>
1403ba985: 48 c1 e2 03                 	shlq	$0x3, %rdx
1403ba989: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403ba98f: e8 8c 92 25 00              	callq	0x140613c20 <.text+0x612c20>
1403ba994: 48 8b 85 e8 04 00 00        	movq	0x4e8(%rbp), %rax
1403ba99b: 48 8b 8d f0 08 00 00        	movq	0x8f0(%rbp), %rcx
1403ba9a2: f3 0f 6f 01                 	movdqu	(%rcx), %xmm0
1403ba9a6: f3 0f 6f 49 10              	movdqu	0x10(%rcx), %xmm1
1403ba9ab: 48 89 85 50 05 00 00        	movq	%rax, 0x550(%rbp)
1403ba9b2: f3 0f 7f 85 30 05 00 00     	movdqu	%xmm0, 0x530(%rbp)
1403ba9ba: f3 0f 7f 8d 40 05 00 00     	movdqu	%xmm1, 0x540(%rbp)
1403ba9c2: 4c 89 e1                    	movq	%r12, %rcx
1403ba9c5: 4c 89 f2                    	movq	%r14, %rdx
1403ba9c8: e8 a3 9d 95 00              	callq	0x140d14770 <.text+0xd13770>
1403ba9cd: 48 c7 85 30 05 00 00 01 00 00 00    	movq	$0x1, 0x530(%rbp)
1403ba9d8: 48 8d 85 b0 03 00 00        	leaq	0x3b0(%rbp), %rax
1403ba9df: 48 89 85 38 05 00 00        	movq	%rax, 0x538(%rbp)
1403ba9e6: 48 8d 85 e8 03 00 00        	leaq	0x3e8(%rbp), %rax
1403ba9ed: 48 89 85 40 05 00 00        	movq	%rax, 0x540(%rbp)
1403ba9f4: 48 8d 85 20 04 00 00        	leaq	0x420(%rbp), %rax
1403ba9fb: 48 89 85 48 05 00 00        	movq	%rax, 0x548(%rbp)
1403baa02: 48 8d 85 58 04 00 00        	leaq	0x458(%rbp), %rax
1403baa09: 48 89 85 50 05 00 00        	movq	%rax, 0x550(%rbp)
1403baa10: 31 ff                       	xorl	%edi, %edi
1403baa12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
1403baa20: 48 83 ff 04                 	cmpq	$0x4, %rdi
1403baa24: 74 2a                       	je	0x1403baa50 <.text+0x3b9a50>
1403baa26: 48 8b 8c fd 38 05 00 00     	movq	0x538(%rbp,%rdi,8), %rcx
1403baa2e: 48 ff c7                    	incq	%rdi
1403baa31: 83 39 02                    	cmpl	$0x2, (%rcx)
1403baa34: 74 ea                       	je	0x1403baa20 <.text+0x3b9a20>
1403baa36: 48 89 bd 58 05 00 00        	movq	%rdi, 0x558(%rbp)
1403baa3d: e8 2e ca 69 00              	callq	0x140a57470 <.text+0xa56470>
1403baa42: eb dc                       	jmp	0x1403baa20 <.text+0x3b9a20>
1403baa44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1403baa50: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1403baa57: e8 a4 b4 23 00              	callq	0x1405f5f00 <.text+0x5f4f00>
1403baa5c: 48 8b 95 90 06 00 00        	movq	0x690(%rbp), %rdx
1403baa63: 48 8b 85 98 06 00 00        	movq	0x698(%rbp), %rax
1403baa6a: 48 39 c2                    	cmpq	%rax, %rdx
1403baa6d: 0f 85 ed fb ff ff           	jne	0x1403ba660 <.text+0x3b9660>
1403baa73: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403baa7a: e8 a1 f2 24 00              	callq	0x140609d20 <.text+0x608d20>
1403baa7f: 48 8d 4d 64                 	leaq	0x64(%rbp), %rcx
1403baa83: 48 8b 95 f8 06 00 00        	movq	0x6f8(%rbp), %rdx
1403baa8a: e8 b1 c2 8d 00              	callq	0x140c96d40 <.text+0xc95d40>
1403baa8f: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403baa96: 48 8b 95 f8 06 00 00        	movq	0x6f8(%rbp), %rdx
1403baa9d: e8 de c2 8d 00              	callq	0x140c96d80 <.text+0xc95d80>
1403baaa2: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1403baaa9: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403baab0: e8 7b 55 8d 00              	callq	0x140c90030 <.text+0xc8f030>
1403baab5: f3 0f 10 85 b8 00 00 00     	movss	0xb8(%rbp), %xmm0
1403baabd: 0f 28 c8                    	movaps	%xmm0, %xmm1
1403baac0: f3 0f 59 c8                 	mulss	%xmm0, %xmm1
1403baac4: f2 0f 10 95 b0 00 00 00     	movsd	0xb0(%rbp), %xmm2
1403baacc: 0f 28 da                    	movaps	%xmm2, %xmm3
1403baacf: 0f 59 da                    	mulps	%xmm2, %xmm3
1403baad2: f3 0f 16 e3                 	movshdup	%xmm3, %xmm4            # xmm4 = xmm3[1,1,3,3]
1403baad6: f3 0f 58 e3                 	addss	%xmm3, %xmm4
1403baada: f3 0f 58 e1                 	addss	%xmm1, %xmm4
1403baade: 0f 57 c9                    	xorps	%xmm1, %xmm1
1403baae1: f3 0f 51 cc                 	sqrtss	%xmm4, %xmm1
1403baae5: f3 0f 5e f9                 	divss	%xmm1, %xmm7
1403baae9: f3 0f 12 cf                 	movsldup	%xmm7, %xmm1            # xmm1 = xmm7[0,0,2,2]
1403baaed: 0f 59 ca                    	mulps	%xmm2, %xmm1
1403baaf0: f3 0f 59 f8                 	mulss	%xmm0, %xmm7
1403baaf4: 0f 13 8d 58 02 00 00        	movlps	%xmm1, 0x258(%rbp)
1403baafb: f3 0f 11 bd 60 02 00 00     	movss	%xmm7, 0x260(%rbp)
1403bab03: 8b 85 dc 06 00 00           	movl	0x6dc(%rbp), %eax
1403bab09: 48 8b 8d 58 03 00 00        	movq	0x358(%rbp), %rcx
1403bab10: 89 01                       	movl	%eax, (%rcx)
1403bab12: 48 8d 55 64                 	leaq	0x64(%rbp), %rdx
1403bab16: 4c 8d 85 58 02 00 00        	leaq	0x258(%rbp), %r8
1403bab1d: 48 8b 8d d0 02 00 00        	movq	0x2d0(%rbp), %rcx
1403bab24: e8 07 96 5d 00              	callq	0x140994130 <.text+0x993130>
1403bab29: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
1403bab30: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1403bab33: 48 8b 0a                    	movq	(%rdx), %rcx
1403bab36: 48 8b 52 10                 	movq	0x10(%rdx), %rdx
1403bab3a: c6 81 d8 00 00 00 00        	movb	$0x0, 0xd8(%rcx)
1403bab41: 89 02                       	movl	%eax, (%rdx)
1403bab43: c7 81 d4 00 00 00 00 00 00 00       	movl	$0x0, 0xd4(%rcx)
1403bab4d: 48 8b 8d e8 08 00 00        	movq	0x8e8(%rbp), %rcx
1403bab54: 48 8b 01                    	movq	(%rcx), %rax
1403bab57: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
1403bab5b: 4c 8b b0 f8 00 00 00        	movq	0xf8(%rax), %r14
1403bab62: 48 8b 88 00 01 00 00        	movq	0x100(%rax), %rcx
1403bab69: 4d 8d 3c 8e                 	leaq	(%r14,%rcx,4), %r15
1403bab6d: 0f b6 80 08 01 00 00        	movzbl	0x108(%rax), %eax
1403bab74: 88 85 88 07 00 00           	movb	%al, 0x788(%rbp)
1403bab7a: b8 08 00 00 00              	movl	$0x8, %eax
1403bab7f: 48 89 85 40 07 00 00        	movq	%rax, 0x740(%rbp)
1403bab86: 45 31 ed                    	xorl	%r13d, %r13d
1403bab89: 48 89 85 00 07 00 00        	movq	%rax, 0x700(%rbp)
1403bab90: 31 db                       	xorl	%ebx, %ebx
1403bab92: 48 89 95 70 07 00 00        	movq	%rdx, 0x770(%rbp)
1403bab99: 80 bd 88 07 00 00 00        	cmpb	$0x0, 0x788(%rbp)
1403baba0: 75 5e                       	jne	0x1403bac00 <.text+0x3b9c00>
1403baba2: 4c 39 eb                    	cmpq	%r13, %rbx
1403baba5: 75 42                       	jne	0x1403babe9 <.text+0x3b9be9>
1403baba7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1403babb0: 4d 39 fe                    	cmpq	%r15, %r14
1403babb3: 0f 84 32 01 00 00           	je	0x1403baceb <.text+0x3b9ceb>
1403babb9: 41 8b 0e                    	movl	(%r14), %ecx
1403babbc: 49 83 c6 04                 	addq	$0x4, %r14
1403babc0: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
1403babc7: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
1403babcb: 48 c1 e1 05                 	shlq	$0x5, %rcx
1403babcf: 4c 8b 6c 08 58              	movq	0x58(%rax,%rcx), %r13
1403babd4: 4d 85 ed                    	testq	%r13, %r13
1403babd7: 74 d7                       	je	0x1403babb0 <.text+0x3b9bb0>
1403babd9: 48 01 c8                    	addq	%rcx, %rax
1403babdc: 48 8b 40 50                 	movq	0x50(%rax), %rax
1403babe0: 48 89 85 00 07 00 00        	movq	%rax, 0x700(%rbp)
1403babe7: 31 db                       	xorl	%ebx, %ebx
1403babe9: 48 89 d8                    	movq	%rbx, %rax
1403babec: 48 c1 e0 04                 	shlq	$0x4, %rax
1403babf0: 48 03 85 00 07 00 00        	addq	0x700(%rbp), %rax
1403babf7: eb 58                       	jmp	0x1403bac51 <.text+0x3b9c51>
1403babf9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403bac00: 4c 39 eb                    	cmpq	%r13, %rbx
1403bac03: 75 41                       	jne	0x1403bac46 <.text+0x3b9c46>
1403bac05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403bac10: 4d 39 fe                    	cmpq	%r15, %r14
1403bac13: 0f 84 d2 00 00 00           	je	0x1403baceb <.text+0x3b9ceb>
1403bac19: 41 8b 0e                    	movl	(%r14), %ecx
1403bac1c: 49 83 c6 04                 	addq	$0x4, %r14
1403bac20: 48 8b 82 a8 01 00 00        	movq	0x1a8(%rdx), %rax
1403bac27: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1403bac2b: 4c 8b 6c c8 10              	movq	0x10(%rax,%rcx,8), %r13
1403bac30: 4d 85 ed                    	testq	%r13, %r13
1403bac33: 74 db                       	je	0x1403bac10 <.text+0x3b9c10>
1403bac35: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1403bac39: 48 8b 40 08                 	movq	0x8(%rax), %rax
1403bac3d: 48 89 85 40 07 00 00        	movq	%rax, 0x740(%rbp)
1403bac44: 31 db                       	xorl	%ebx, %ebx
1403bac46: 48 8b 85 40 07 00 00        	movq	0x740(%rbp), %rax
1403bac4d: 48 8d 04 d8                 	leaq	(%rax,%rbx,8), %rax
1403bac51: 4c 8b 20                    	movq	(%rax), %r12
1403bac54: 4c 89 e0                    	movq	%r12, %rax
1403bac57: 48 c1 e8 20                 	shrq	$0x20, %rax
1403bac5b: 0f 84 8a 00 00 00           	je	0x1403baceb <.text+0x3b9ceb>
1403bac61: 48 8b 85 f0 08 00 00        	movq	0x8f0(%rbp), %rax
1403bac68: 48 8b 70 08                 	movq	0x8(%rax), %rsi
1403bac6c: 48 85 f6                    	testq	%rsi, %rsi
1403bac6f: 75 04                       	jne	0x1403bac75 <.text+0x3b9c75>
1403bac71: 48 8b 70 10                 	movq	0x10(%rax), %rsi
1403bac75: 48 8b 06                    	movq	(%rsi), %rax
1403bac78: 48 8b 7e 10                 	movq	0x10(%rsi), %rdi
1403bac7c: 48 29 f8                    	subq	%rdi, %rax
1403bac7f: 48 83 f8 17                 	cmpq	$0x17, %rax
1403bac83: 76 44                       	jbe	0x1403bacc9 <.text+0x3b9cc9>
1403bac85: 48 ff c3                    	incq	%rbx
1403bac88: 48 8b 46 08                 	movq	0x8(%rsi), %rax
1403bac8c: 48 8d 0d bd 2a d9 ff        	leaq	-0x26d543(%rip), %rcx   # 0x14014d750 <.text+0x14c750>
1403bac93: 48 89 0c 38                 	movq	%rcx, (%rax,%rdi)
1403bac97: 48 8d 0d b2 2d d9 ff        	leaq	-0x26d24e(%rip), %rcx   # 0x14014da50 <.text+0x14ca50>
1403bac9e: 48 89 4c 38 08              	movq	%rcx, 0x8(%rax,%rdi)
1403baca3: 4c 89 64 38 10              	movq	%r12, 0x10(%rax,%rdi)
1403baca8: 48 83 c7 18                 	addq	$0x18, %rdi
1403bacac: 48 89 7e 10                 	movq	%rdi, 0x10(%rsi)
1403bacb0: 48 8b 95 70 07 00 00        	movq	0x770(%rbp), %rdx
1403bacb7: 80 bd 88 07 00 00 00        	cmpb	$0x0, 0x788(%rbp)
1403bacbe: 0f 84 de fe ff ff           	je	0x1403baba2 <.text+0x3b9ba2>
1403bacc4: e9 37 ff ff ff              	jmp	0x1403bac00 <.text+0x3b9c00>
1403bacc9: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1403bacd2: 41 b8 18 00 00 00           	movl	$0x18, %r8d
1403bacd8: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403bacde: 48 89 f1                    	movq	%rsi, %rcx
1403bace1: 48 89 fa                    	movq	%rdi, %rdx
1403bace4: e8 f7 7c 4e 02              	callq	0x1428a29e0 <.text+0x28a19e0>
1403bace9: eb 9a                       	jmp	0x1403bac85 <.text+0x3b9c85>
1403baceb: 31 c0                       	xorl	%eax, %eax
1403baced: 48 8b 8d 80 07 00 00        	movq	0x780(%rbp), %rcx
1403bacf4: 48 3b 01                    	cmpq	(%rcx), %rax
1403bacf7: 70 0c                       	jo	0x1403bad05 <.text+0x3b9d05>
1403bacf9: 48 8b 8d 80 07 00 00        	movq	0x780(%rbp), %rcx
1403bad00: e8 ab f2 23 00              	callq	0x1405f9fb0 <.text+0x5f8fb0>
1403bad05: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403bad0f: 48 8b 8d 80 07 00 00        	movq	0x780(%rbp), %rcx
1403bad16: 48 89 01                    	movq	%rax, (%rcx)
1403bad19: 48 8b 95 38 07 00 00        	movq	0x738(%rbp), %rdx
1403bad20: 48 85 d2                    	testq	%rdx, %rdx
1403bad23: 74 12                       	je	0x1403bad37 <.text+0x3b9d37>
1403bad25: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403bad2b: 48 8b 8d 18 07 00 00        	movq	0x718(%rbp), %rcx
1403bad32: e8 e9 8e 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bad37: 4c 8b 2d ea 12 27 03        	movq	0x32712ea(%rip), %r13   # 0x14362c028
1403bad3e: 41 c6 45 00 00              	movb	$0x0, (%r13)
1403bad43: 48 8b 85 48 03 00 00        	movq	0x348(%rbp), %rax
1403bad4a: 83 38 13                    	cmpl	$0x13, (%rax)
1403bad4d: 0f 85 b6 00 00 00           	jne	0x1403bae09 <.text+0x3b9e09>
1403bad53: 48 8b 85 20 06 00 00        	movq	0x620(%rbp), %rax
1403bad5a: 48 8b 30                    	movq	(%rax), %rsi
1403bad5d: 48 89 f1                    	movq	%rsi, %rcx
1403bad60: e8 cb 64 55 00              	callq	0x140911230 <.text+0x910230>
1403bad65: 3c 03                       	cmpb	$0x3, %al
1403bad67: 0f 85 9c 00 00 00           	jne	0x1403bae09 <.text+0x3b9e09>
1403bad6d: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403bad74: 48 8b 00                    	movq	(%rax), %rax
1403bad77: 48 f7 d8                    	negq	%rax
1403bad7a: 0f 81 89 00 00 00           	jno	0x1403bae09 <.text+0x3b9e09>
1403bad80: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403bad87: 48 89 f2                    	movq	%rsi, %rdx
1403bad8a: e8 41 70 55 00              	callq	0x140911dd0 <.text+0x910dd0>
1403bad8f: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bad96: 48 89 f2                    	movq	%rsi, %rdx
1403bad99: e8 52 70 55 00              	callq	0x140911df0 <.text+0x910df0>
1403bad9e: 8b 85 38 05 00 00           	movl	0x538(%rbp), %eax
1403bada4: 89 85 b8 00 00 00           	movl	%eax, 0xb8(%rbp)
1403badaa: 48 8b 85 30 05 00 00        	movq	0x530(%rbp), %rax
1403badb1: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1403badb8: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1403badbf: 48 89 85 bc 00 00 00        	movq	%rax, 0xbc(%rbp)
1403badc6: 8b 85 88 03 00 00           	movl	0x388(%rbp), %eax
1403badcc: 89 85 c4 00 00 00           	movl	%eax, 0xc4(%rbp)
1403badd2: c7 44 24 20 01 01 00 00     	movl	$0x101, 0x20(%rsp)      # imm = 0x101
1403badda: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bade1: 48 8d 55 c0                 	leaq	-0x40(%rbp), %rdx
1403bade5: 48 8d bd b0 00 00 00        	leaq	0xb0(%rbp), %rdi
1403badec: f3 0f 10 1d a0 98 57 02     	movss	0x25798a0(%rip), %xmm3  # 0x142934694
1403badf4: 49 89 f8                    	movq	%rdi, %r8
1403badf7: e8 e4 14 51 00              	callq	0x1408cc2e0 <.text+0x8cb2e0>
1403badfc: 83 bd b4 03 00 00 02        	cmpl	$0x2, 0x3b4(%rbp)
1403bae03: 0f 85 b7 00 00 00           	jne	0x1403baec0 <.text+0x3b9ec0>
1403bae09: 48 83 bd e8 05 00 00 00     	cmpq	$0x0, 0x5e8(%rbp)
1403bae11: 74 4b                       	je	0x1403bae5e <.text+0x3b9e5e>
1403bae13: e8 18 d3 85 00              	callq	0x140c18130 <.text+0xc17130>
1403bae18: 48 89 c1                    	movq	%rax, %rcx
1403bae1b: 48 8b 00                    	movq	(%rax), %rax
1403bae1e: 48 83 f8 01                 	cmpq	$0x1, %rax
1403bae22: 0f 85 f4 01 00 00           	jne	0x1403bb01c <.text+0x3ba01c>
1403bae28: 48 83 c1 08                 	addq	$0x8, %rcx
1403bae2c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1403bae30: 0f 85 88 11 00 00           	jne	0x1403bbfbe <.text+0x3bafbe>
1403bae36: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1403bae3d: 48 89 8d 88 07 00 00        	movq	%rcx, 0x788(%rbp)
1403bae44: 48 83 c1 08                 	addq	$0x8, %rcx
1403bae48: 48 8b 95 f0 05 00 00        	movq	0x5f0(%rbp), %rdx
1403bae4f: e8 9c c2 85 00              	callq	0x140c170f0 <.text+0xc160f0>
1403bae54: 48 8b 85 88 07 00 00        	movq	0x788(%rbp), %rax
1403bae5b: 48 ff 00                    	incq	(%rax)
1403bae5e: 0f 28 b5 a0 07 00 00        	movaps	0x7a0(%rbp), %xmm6
1403bae65: 0f 28 bd b0 07 00 00        	movaps	0x7b0(%rbp), %xmm7
1403bae6c: 44 0f 28 85 c0 07 00 00     	movaps	0x7c0(%rbp), %xmm8
1403bae74: 44 0f 28 8d d0 07 00 00     	movaps	0x7d0(%rbp), %xmm9
1403bae7c: 44 0f 28 95 e0 07 00 00     	movaps	0x7e0(%rbp), %xmm10
1403bae84: 44 0f 28 9d f0 07 00 00     	movaps	0x7f0(%rbp), %xmm11
1403bae8c: 44 0f 28 a5 00 08 00 00     	movaps	0x800(%rbp), %xmm12
1403bae94: 44 0f 28 ad 10 08 00 00     	movaps	0x810(%rbp), %xmm13
1403bae9c: 44 0f 28 b5 20 08 00 00     	movaps	0x820(%rbp), %xmm14
1403baea4: 44 0f 28 bd 30 08 00 00     	movaps	0x830(%rbp), %xmm15
1403baeac: 48 81 c4 c8 08 00 00        	addq	$0x8c8, %rsp            # imm = 0x8C8
1403baeb3: 5b                          	popq	%rbx
1403baeb4: 5f                          	popq	%rdi
1403baeb5: 5e                          	popq	%rsi
1403baeb6: 41 5c                       	popq	%r12
1403baeb8: 41 5d                       	popq	%r13
1403baeba: 41 5e                       	popq	%r14
1403baebc: 41 5f                       	popq	%r15
1403baebe: 5d                          	popq	%rbp
1403baebf: c3                          	retq
1403baec0: 80 bd 80 03 00 00 04        	cmpb	$0x4, 0x380(%rbp)
1403baec7: 0f 85 3c ff ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403baecd: 48 83 bd 88 03 00 00 02     	cmpq	$0x2, 0x388(%rbp)
1403baed5: 0f 85 2e ff ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403baedb: 44 8b b5 84 03 00 00        	movl	0x384(%rbp), %r14d
1403baee2: 44 89 b5 30 05 00 00        	movl	%r14d, 0x530(%rbp)
1403baee9: 48 8b 8d 60 03 00 00        	movq	0x360(%rbp), %rcx
1403baef0: 44 89 f2                    	movl	%r14d, %edx
1403baef3: e8 38 1a 66 00              	callq	0x140a1c930 <.text+0xa1b930>
1403baef8: 48 89 c3                    	movq	%rax, %rbx
1403baefb: 48 85 c0                    	testq	%rax, %rax
1403baefe: 0f 84 05 ff ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403baf04: 48 8d 15 dd 2b 61 02        	leaq	0x2612bdd(%rip), %rdx   # 0x1429cdae8
1403baf0b: 48 89 d9                    	movq	%rbx, %rcx
1403baf0e: e8 5d 85 47 00              	callq	0x140833470 <.text+0x832470>
1403baf13: 0f b6 5b 46                 	movzbl	0x46(%rbx), %ebx
1403baf17: 8d 4b c8                    	leal	-0x38(%rbx), %ecx
1403baf1a: 80 f9 08                    	cmpb	$0x8, %cl
1403baf1d: 0f 92 c2                    	setb	%dl
1403baf20: 80 f9 06                    	cmpb	$0x6, %cl
1403baf23: 0f 95 c1                    	setne	%cl
1403baf26: 84 ca                       	testb	%cl, %dl
1403baf28: 0f 85 db fe ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403baf2e: 0f b6 cb                    	movzbl	%bl, %ecx
1403baf31: 83 f9 36                    	cmpl	$0x36, %ecx
1403baf34: 0f 87 cf fe ff ff           	ja	0x1403bae09 <.text+0x3b9e09>
1403baf3a: 89 c9                       	movl	%ecx, %ecx
1403baf3c: 48 ba 00 00 80 01 00 01 40 00       	movabsq	$0x40010001800000, %rdx # imm = 0x40010001800000
1403baf46: 48 0f a3 ca                 	btq	%rcx, %rdx
1403baf4a: 0f 83 b9 fe ff ff           	jae	0x1403bae09 <.text+0x3b9e09>
1403baf50: 44 8b 38                    	movl	(%rax), %r15d
1403baf53: 8b 40 04                    	movl	0x4(%rax), %eax
1403baf56: 89 85 88 07 00 00           	movl	%eax, 0x788(%rbp)
1403baf5c: 48 8b 85 28 06 00 00        	movq	0x628(%rbp), %rax
1403baf63: 48 8d 48 18                 	leaq	0x18(%rax), %rcx
1403baf67: e8 14 2e 2c 00              	callq	0x14067dd80 <.text+0x67cd80>
1403baf6c: 48 83 78 18 00              	cmpq	$0x0, 0x18(%rax)
1403baf71: 0f 84 92 fe ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403baf77: 48 89 c1                    	movq	%rax, %rcx
1403baf7a: 48 83 c1 20                 	addq	$0x20, %rcx
1403baf7e: 48 8d 95 30 05 00 00        	leaq	0x530(%rbp), %rdx
1403baf85: 49 89 c4                    	movq	%rax, %r12
1403baf88: e8 43 5f 1f 00              	callq	0x1405b0ed0 <.text+0x5afed0>
1403baf8d: 49 8b 0c 24                 	movq	(%r12), %rcx
1403baf91: 49 8b 54 24 08              	movq	0x8(%r12), %rdx
1403baf96: 49 89 d0                    	movq	%rdx, %r8
1403baf99: 49 21 c0                    	andq	%rax, %r8
1403baf9c: 48 c1 e8 39                 	shrq	$0x39, %rax
1403bafa0: 66 0f 6e c0                 	movd	%eax, %xmm0
1403bafa4: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1403bafa8: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1403bafad: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1403bafb2: 48 8d 41 d8                 	leaq	-0x28(%rcx), %rax
1403bafb6: 45 31 c9                    	xorl	%r9d, %r9d
1403bafb9: 66 0f 76 c9                 	pcmpeqd	%xmm1, %xmm1
1403bafbd: f3 42 0f 6f 14 01           	movdqu	(%rcx,%r8), %xmm2
1403bafc3: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1403bafc7: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1403bafcb: 66 44 0f d7 d3              	pmovmskb	%xmm3, %r10d
1403bafd0: 45 85 d2                    	testl	%r10d, %r10d
1403bafd3: 74 25                       	je	0x1403baffa <.text+0x3b9ffa>
1403bafd5: f3 45 0f bc da              	tzcntl	%r10d, %r11d
1403bafda: 4d 01 c3                    	addq	%r8, %r11
1403bafdd: 49 21 d3                    	andq	%rdx, %r11
1403bafe0: 49 f7 db                    	negq	%r11
1403bafe3: 4f 8d 1c 9b                 	leaq	(%r11,%r11,4), %r11
1403bafe7: 46 3b 34 d8                 	cmpl	(%rax,%r11,8), %r14d
1403bafeb: 74 6d                       	je	0x1403bb05a <.text+0x3ba05a>
1403bafed: 45 8d 5a ff                 	leal	-0x1(%r10), %r11d
1403baff1: 66 45 21 d3                 	andw	%r10w, %r11w
1403baff5: 45 89 da                    	movl	%r11d, %r10d
1403baff8: 75 db                       	jne	0x1403bafd5 <.text+0x3b9fd5>
1403baffa: 66 0f 74 d1                 	pcmpeqb	%xmm1, %xmm2
1403baffe: 66 44 0f d7 d2              	pmovmskb	%xmm2, %r10d
1403bb003: 45 85 d2                    	testl	%r10d, %r10d
1403bb006: 0f 85 fd fd ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403bb00c: 4d 01 c8                    	addq	%r9, %r8
1403bb00f: 49 83 c0 10                 	addq	$0x10, %r8
1403bb013: 49 83 c1 10                 	addq	$0x10, %r9
1403bb017: 49 21 d0                    	andq	%rdx, %r8
1403bb01a: eb a1                       	jmp	0x1403bafbd <.text+0x3b9fbd>
1403bb01c: 48 85 c0                    	testq	%rax, %rax
1403bb01f: 75 2d                       	jne	0x1403bb04e <.text+0x3ba04e>
1403bb021: 31 d2                       	xorl	%edx, %edx
1403bb023: e8 18 66 4e 02              	callq	0x1428a1640 <.text+0x28a0640>
1403bb028: 48 89 c1                    	movq	%rax, %rcx
1403bb02b: 48 85 c0                    	testq	%rax, %rax
1403bb02e: 0f 85 f8 fd ff ff           	jne	0x1403bae2c <.text+0x3b9e2c>
1403bb034: eb 18                       	jmp	0x1403bb04e <.text+0x3ba04e>
1403bb036: 48 85 c0                    	testq	%rax, %rax
1403bb039: 75 13                       	jne	0x1403bb04e <.text+0x3ba04e>
1403bb03b: 31 d2                       	xorl	%edx, %edx
1403bb03d: e8 fe 65 4e 02              	callq	0x1428a1640 <.text+0x28a0640>
1403bb042: 48 89 c1                    	movq	%rax, %rcx
1403bb045: 48 85 c0                    	testq	%rax, %rax
1403bb048: 0f 85 be d1 ff ff           	jne	0x1403b820c <.text+0x3b720c>
1403bb04e: 48 8d 0d bb 79 57 02        	leaq	0x25779bb(%rip), %rcx   # 0x142932a10
1403bb055: e8 76 d0 51 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1403bb05a: 4a 8d 0c d9                 	leaq	(%rcx,%r11,8), %rcx
1403bb05e: 48 83 c1 e0                 	addq	$-0x20, %rcx
1403bb062: e8 69 69 45 00              	callq	0x1408119d0 <.text+0x8109d0>
1403bb067: 48 85 c0                    	testq	%rax, %rax
1403bb06a: 0f 94 c1                    	sete	%cl
1403bb06d: 48 85 d2                    	testq	%rdx, %rdx
1403bb070: 0f 94 c2                    	sete	%dl
1403bb073: 08 ca                       	orb	%cl, %dl
1403bb075: 0f 85 8e fd ff ff           	jne	0x1403bae09 <.text+0x3b9e09>
1403bb07b: 48 8b 00                    	movq	(%rax), %rax
1403bb07e: 48 89 c1                    	movq	%rax, %rcx
1403bb081: 48 c1 e9 20                 	shrq	$0x20, %rcx
1403bb085: 0f 84 7e fd ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403bb08b: 80 fb 38                    	cmpb	$0x38, %bl
1403bb08e: 0f 84 75 fd ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403bb094: 89 85 f8 02 00 00           	movl	%eax, 0x2f8(%rbp)
1403bb09a: 89 8d fc 02 00 00           	movl	%ecx, 0x2fc(%rbp)
1403bb0a0: 44 89 bd 00 03 00 00        	movl	%r15d, 0x300(%rbp)
1403bb0a7: 8b 85 88 07 00 00           	movl	0x788(%rbp), %eax
1403bb0ad: 89 85 04 03 00 00           	movl	%eax, 0x304(%rbp)
1403bb0b3: 44 89 b5 08 03 00 00        	movl	%r14d, 0x308(%rbp)
1403bb0ba: 88 9d 0c 03 00 00           	movb	%bl, 0x30c(%rbp)
1403bb0c0: 41 c6 45 00 01              	movb	$0x1, (%r13)
1403bb0c5: 48 8b 85 b0 08 00 00        	movq	0x8b0(%rbp), %rax
1403bb0cc: 48 8b 00                    	movq	(%rax), %rax
1403bb0cf: 8b 88 98 00 00 00           	movl	0x98(%rax), %ecx
1403bb0d5: 66 0f 6e 88 9c 00 00 00     	movd	0x9c(%rax), %xmm1
1403bb0dd: e8 ae 49 58 00              	callq	0x14093fa90 <.text+0x93ea90>
1403bb0e2: 84 c0                       	testb	%al, %al
1403bb0e4: 0f 84 1f fd ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403bb0ea: 48 8b 85 f8 02 00 00        	movq	0x2f8(%rbp), %rax
1403bb0f1: 48 8b 8d c8 08 00 00        	movq	0x8c8(%rbp), %rcx
1403bb0f8: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1403bb0fc: 41 89 c0                    	movl	%eax, %r8d
1403bb0ff: 4c 3b 41 10                 	cmpq	0x10(%rcx), %r8
1403bb103: 0f 83 79 17 00 00           	jae	0x1403bc882 <.text+0x3bb882>
1403bb109: 48 8b 51 08                 	movq	0x8(%rcx), %rdx
1403bb10d: 49 89 c2                    	movq	%rax, %r10
1403bb110: 49 c1 ea 20                 	shrq	$0x20, %r10
1403bb114: 4f 8d 0c 80                 	leaq	(%r8,%r8,4), %r9
1403bb118: 46 39 14 8a                 	cmpl	%r10d, (%rdx,%r9,4)
1403bb11c: 0f 85 67 17 00 00           	jne	0x1403bc889 <.text+0x3bb889>
1403bb122: 4e 8d 0c 8a                 	leaq	(%rdx,%r9,4), %r9
1403bb126: 41 8b 51 04                 	movl	0x4(%r9), %edx
1403bb12a: 41 bb ff ff ff ff           	movl	$0xffffffff, %r11d      # imm = 0xFFFFFFFF
1403bb130: 4c 39 da                    	cmpq	%r11, %rdx
1403bb133: 0f 84 50 17 00 00           	je	0x1403bc889 <.text+0x3bb889>
1403bb139: 4c 8b 85 c8 08 00 00        	movq	0x8c8(%rbp), %r8
1403bb140: 4d 8b 10                    	movq	(%r8), %r10
1403bb143: 45 31 c0                    	xorl	%r8d, %r8d
1403bb146: 49 39 52 38                 	cmpq	%rdx, 0x38(%r10)
1403bb14a: 0f 86 49 17 00 00           	jbe	0x1403bc899 <.text+0x3bb899>
1403bb150: 4d 8b 5a 28                 	movq	0x28(%r10), %r11
1403bb154: 89 d3                       	movl	%edx, %ebx
1403bb156: c1 eb 06                    	shrl	$0x6, %ebx
1403bb159: 4d 8b 1c db                 	movq	(%r11,%rbx,8), %r11
1403bb15d: 49 0f a3 d3                 	btq	%rdx, %r11
1403bb161: 0f 83 32 17 00 00           	jae	0x1403bc899 <.text+0x3bb899>
1403bb167: 41 8b 41 0c                 	movl	0xc(%r9), %eax
1403bb16b: 41 8b 51 10                 	movl	0x10(%r9), %edx
1403bb16f: 48 8b 89 a8 01 00 00        	movq	0x1a8(%rcx), %rcx
1403bb176: 48 8d 04 c0                 	leaq	(%rax,%rax,8), %rax
1403bb17a: 4d 8b 82 10 01 00 00        	movq	0x110(%r10), %r8
1403bb181: 4c 8b 4c c1 18              	movq	0x18(%rcx,%rax,8), %r9
1403bb186: 48 8b 44 c1 38              	movq	0x38(%rcx,%rax,8), %rax
1403bb18b: 4a 8b 04 c0                 	movq	(%rax,%r8,8), %rax
1403bb18f: 48 f7 d0                    	notq	%rax
1403bb192: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1403bb196: 48 c1 e0 04                 	shlq	$0x4, %rax
1403bb19a: 49 8b 44 01 10              	movq	0x10(%r9,%rax), %rax
1403bb19f: 48 8d 0c 52                 	leaq	(%rdx,%rdx,2), %rcx
1403bb1a3: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403bb1a7: 0f 28 34 08                 	movaps	(%rax,%rcx), %xmm6
1403bb1ab: 44 89 bd 80 03 00 00        	movl	%r15d, 0x380(%rbp)
1403bb1b2: 8b 85 88 07 00 00           	movl	0x788(%rbp), %eax
1403bb1b8: 89 85 84 03 00 00           	movl	%eax, 0x384(%rbp)
1403bb1be: 48 8b 85 30 07 00 00        	movq	0x730(%rbp), %rax
1403bb1c5: 48 8b 98 00 01 00 00        	movq	0x100(%rax), %rbx
1403bb1cc: 48 85 db                    	testq	%rbx, %rbx
1403bb1cf: 0f 84 b3 01 00 00           	je	0x1403bb388 <.text+0x3ba388>
1403bb1d5: 48 83 fb 01                 	cmpq	$0x1, %rbx
1403bb1d9: 75 6f                       	jne	0x1403bb24a <.text+0x3ba24a>
1403bb1db: 48 8b 80 f8 00 00 00        	movq	0xf8(%rax), %rax
1403bb1e2: 44 39 78 18                 	cmpl	%r15d, 0x18(%rax)
1403bb1e6: 0f 85 9c 01 00 00           	jne	0x1403bb388 <.text+0x3ba388>
1403bb1ec: 8b 8d 88 07 00 00           	movl	0x788(%rbp), %ecx
1403bb1f2: 39 48 1c                    	cmpl	%ecx, 0x1c(%rax)
1403bb1f5: 0f 85 8d 01 00 00           	jne	0x1403bb388 <.text+0x3ba388>
1403bb1fb: 44 89 b5 18 07 00 00        	movl	%r14d, 0x718(%rbp)
1403bb202: 31 d2                       	xorl	%edx, %edx
1403bb204: e9 a8 00 00 00              	jmp	0x1403bb2b1 <.text+0x3ba2b1>
1403bb209: 4c 8d 05 e0 21 61 02        	leaq	0x26121e0(%rip), %r8    # 0x1429cd3f0
1403bb210: f3 0f 10 0d 58 1e 61 02     	movss	0x2611e58(%rip), %xmm1  # 0x1429cd070
1403bb218: 0f 28 c2                    	movaps	%xmm2, %xmm0
1403bb21b: e8 60 02 9d 00              	callq	0x140d8b480 <.text+0xd8a480>
1403bb220: e9 f2 16 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bb225: 4c 8d 05 c4 21 61 02        	leaq	0x26121c4(%rip), %r8    # 0x1429cd3f0
1403bb22c: e8 4f 02 9d 00              	callq	0x140d8b480 <.text+0xd8a480>
1403bb231: e9 e1 16 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bb236: 4c 8d 05 1b 27 61 02        	leaq	0x261271b(%rip), %r8    # 0x1429cd958
1403bb23d: 4c 89 c9                    	movq	%r9, %rcx
1403bb240: e8 d3 e2 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bb245: e9 cd 16 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bb24a: 4c 8d a8 f0 00 00 00        	leaq	0xf0(%rax), %r13
1403bb251: 48 8b 88 28 01 00 00        	movq	0x128(%rax), %rcx
1403bb258: 48 8b 90 30 01 00 00        	movq	0x130(%rax), %rdx
1403bb25f: 45 89 f8                    	movl	%r15d, %r8d
1403bb262: 44 8b 8d 88 07 00 00        	movl	0x788(%rbp), %r9d
1403bb269: e8 d2 b5 d4 ff              	callq	0x140106840 <.text+0x105840>
1403bb26e: 4c 8d 85 80 03 00 00        	leaq	0x380(%rbp), %r8
1403bb275: 4c 89 e9                    	movq	%r13, %rcx
1403bb278: 48 89 c2                    	movq	%rax, %rdx
1403bb27b: e8 20 00 ff ff              	callq	0x1403ab2a0 <.text+0x3aa2a0>
1403bb280: 48 83 f8 01                 	cmpq	$0x1, %rax
1403bb284: 48 8b 85 30 07 00 00        	movq	0x730(%rbp), %rax
1403bb28b: 0f 85 f7 00 00 00           	jne	0x1403bb388 <.text+0x3ba388>
1403bb291: 44 89 b5 18 07 00 00        	movl	%r14d, 0x718(%rbp)
1403bb298: 48 39 da                    	cmpq	%rbx, %rdx
1403bb29b: 0f 83 53 16 00 00           	jae	0x1403bc8f4 <.text+0x3bb8f4>
1403bb2a1: 48 8b 80 f8 00 00 00        	movq	0xf8(%rax), %rax
1403bb2a8: 48 85 c0                    	testq	%rax, %rax
1403bb2ab: 0f 84 d7 00 00 00           	je	0x1403bb388 <.text+0x3ba388>
1403bb2b1: 0f 28 ce                    	movaps	%xmm6, %xmm1
1403bb2b4: 0f 59 ce                    	mulps	%xmm6, %xmm1
1403bb2b7: f3 0f 16 c1                 	movshdup	%xmm1, %xmm0            # xmm0 = xmm1[1,1,3,3]
1403bb2bb: f3 0f 58 c1                 	addss	%xmm1, %xmm0
1403bb2bf: 0f 28 d1                    	movaps	%xmm1, %xmm2
1403bb2c2: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1403bb2c6: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1403bb2ca: 0f c6 d2 00                 	shufps	$0x0, %xmm2, %xmm2      # xmm2 = xmm2[0,0,0,0]
1403bb2ce: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1403bb2d2: 0f 5c ca                    	subps	%xmm2, %xmm1
1403bb2d5: 0f 59 0d 04 20 61 02        	mulps	0x2612004(%rip), %xmm1  # 0x1429cd2e0
1403bb2dc: 0f 28 05 bd dd 5e 02        	movaps	0x25eddbd(%rip), %xmm0  # 0x1429a90a0
1403bb2e3: 0f 59 c6                    	mulps	%xmm6, %xmm0
1403bb2e6: f3 0f 16 d0                 	movshdup	%xmm0, %xmm2            # xmm2 = xmm0[1,1,3,3]
1403bb2ea: f3 0f 58 d0                 	addss	%xmm0, %xmm2
1403bb2ee: 0f 12 c0                    	movhlps	%xmm0, %xmm0            # xmm0 = xmm0[1,1]
1403bb2f1: f3 0f 58 c2                 	addss	%xmm2, %xmm0
1403bb2f5: f3 0f 58 c0                 	addss	%xmm0, %xmm0
1403bb2f9: 0f c6 c0 00                 	shufps	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1403bb2fd: 0f 59 c6                    	mulps	%xmm6, %xmm0
1403bb300: 0f 58 c1                    	addps	%xmm1, %xmm0
1403bb303: 0f 28 ce                    	movaps	%xmm6, %xmm1
1403bb306: 0f c6 ce d6                 	shufps	$0xd6, %xmm6, %xmm1     # xmm1 = xmm1[2,1],xmm6[1,3]
1403bb30a: 0f 59 0d df 1f 61 02        	mulps	0x2611fdf(%rip), %xmm1  # 0x1429cd2f0
1403bb311: 0f 28 d6                    	movaps	%xmm6, %xmm2
1403bb314: 0f c6 d6 c5                 	shufps	$0xc5, %xmm6, %xmm2     # xmm2 = xmm2[1,1],xmm6[0,3]
1403bb318: 0f 59 15 e1 1d 61 02        	mulps	0x2611de1(%rip), %xmm2  # 0x1429cd100
1403bb31f: 0f 5c d1                    	subps	%xmm1, %xmm2
1403bb322: 0f 58 f6                    	addps	%xmm6, %xmm6
1403bb325: 0f c6 f6 ff                 	shufps	$0xff, %xmm6, %xmm6     # xmm6 = xmm6[3,3,3,3]
1403bb329: 0f 59 f2                    	mulps	%xmm2, %xmm6
1403bb32c: 0f 58 f0                    	addps	%xmm0, %xmm6
1403bb32f: 48 c1 e2 05                 	shlq	$0x5, %rdx
1403bb333: f2 0f 10 44 10 08           	movsd	0x8(%rax,%rdx), %xmm0
1403bb339: 0f c6 f6 e8                 	shufps	$0xe8, %xmm6, %xmm6     # xmm6 = xmm6[0,2,2,3]
1403bb33d: 0f 59 f0                    	mulps	%xmm0, %xmm6
1403bb340: f3 0f 16 fe                 	movshdup	%xmm6, %xmm7            # xmm7 = xmm6[1,1,3,3]
1403bb344: f3 0f 58 fe                 	addss	%xmm6, %xmm7
1403bb348: 0f b6 05 ba 98 27 03        	movzbl	0x32798ba(%rip), %eax   # 0x143634c09
1403bb34f: b9 08 00 00 00              	movl	$0x8, %ecx
1403bb354: ba 04 00 00 00              	movl	$0x4, %edx
1403bb359: e8 b2 88 25 00              	callq	0x140613c10 <.text+0x612c10>
1403bb35e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403bb361: 0f 2e f8                    	ucomiss	%xmm0, %xmm7
1403bb364: 76 63                       	jbe	0x1403bb3c9 <.text+0x3ba3c9>
1403bb366: 44 89 f9                    	movl	%r15d, %ecx
1403bb369: 8b 95 88 07 00 00           	movl	0x788(%rbp), %edx
1403bb36f: 48 85 c0                    	testq	%rax, %rax
1403bb372: 75 67                       	jne	0x1403bb3db <.text+0x3ba3db>
1403bb374: b9 04 00 00 00              	movl	$0x4, %ecx
1403bb379: ba 08 00 00 00              	movl	$0x8, %edx
1403bb37e: e8 7d dc 51 02              	callq	0x1428d9000 <.text+0x28d8000>
1403bb383: e9 8f 15 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bb388: 48 8d 05 a1 25 61 02        	leaq	0x26125a1(%rip), %rax   # 0x1429cd930
1403bb38f: 48 89 85 80 03 00 00        	movq	%rax, 0x380(%rbp)
1403bb396: 48 c7 85 88 03 00 00 01 00 00 00    	movq	$0x1, 0x388(%rbp)
1403bb3a1: 48 c7 85 90 03 00 00 08 00 00 00    	movq	$0x8, 0x390(%rbp)
1403bb3ac: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1403bb3b0: f3 0f 7f 85 98 03 00 00     	movdqu	%xmm0, 0x398(%rbp)
1403bb3b8: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bb3bf: e8 5c 6c 4e 02              	callq	0x1428a2020 <.text+0x28a1020>
1403bb3c4: e9 a2 0a 00 00              	jmp	0x1403bbe6b <.text+0x3bae6b>
1403bb3c9: 8b 8d 88 07 00 00           	movl	0x788(%rbp), %ecx
1403bb3cf: 44 89 fa                    	movl	%r15d, %edx
1403bb3d2: 48 85 c0                    	testq	%rax, %rax
1403bb3d5: 0f 84 2d 15 00 00           	je	0x1403bc908 <.text+0x3bb908>
1403bb3db: 89 08                       	movl	%ecx, (%rax)
1403bb3dd: 89 50 04                    	movl	%edx, 0x4(%rax)
1403bb3e0: 48 c7 85 e0 06 00 00 02 00 00 00    	movq	$0x2, 0x6e0(%rbp)
1403bb3eb: 48 89 85 e8 06 00 00        	movq	%rax, 0x6e8(%rbp)
1403bb3f2: 48 c7 85 f0 06 00 00 02 00 00 00    	movq	$0x2, 0x6f0(%rbp)
1403bb3fd: e8 3e 23 99 00              	callq	0x140d4d740 <.text+0xd4c740>
1403bb402: 48 89 c3                    	movq	%rax, %rbx
1403bb405: f6 00 01                    	testb	$0x1, (%rax)
1403bb408: 74 11                       	je	0x1403bb41b <.text+0x3ba41b>
1403bb40a: 48 89 d9                    	movq	%rbx, %rcx
1403bb40d: 48 83 c1 08                 	addq	$0x8, %rcx
1403bb411: 48 8b 43 08                 	movq	0x8(%rbx), %rax
1403bb415: 48 8b 53 10                 	movq	0x10(%rbx), %rdx
1403bb419: eb 1b                       	jmp	0x1403bb436 <.text+0x3ba436>
1403bb41b: e8 e0 99 9a 00              	callq	0x140d64e00 <.text+0xd63e00>
1403bb420: 48 89 d9                    	movq	%rbx, %rcx
1403bb423: 48 83 c1 08                 	addq	$0x8, %rcx
1403bb427: 48 c7 03 01 00 00 00        	movq	$0x1, (%rbx)
1403bb42e: 48 89 43 08                 	movq	%rax, 0x8(%rbx)
1403bb432: 48 89 53 10                 	movq	%rdx, 0x10(%rbx)
1403bb436: 4c 8d 40 01                 	leaq	0x1(%rax), %r8
1403bb43a: 4c 89 01                    	movq	%r8, (%rcx)
1403bb43d: 0f 10 05 4c 1f 61 02        	movups	0x2611f4c(%rip), %xmm0  # 0x1429cd390
1403bb444: 0f 29 85 90 03 00 00        	movaps	%xmm0, 0x390(%rbp)
1403bb44b: 0f 10 05 2e 1f 61 02        	movups	0x2611f2e(%rip), %xmm0  # 0x1429cd380
1403bb452: 0f 29 85 80 03 00 00        	movaps	%xmm0, 0x380(%rbp)
1403bb459: 4c 8d 85 a0 03 00 00        	leaq	0x3a0(%rbp), %r8
1403bb460: 48 89 85 a0 03 00 00        	movq	%rax, 0x3a0(%rbp)
1403bb467: 48 89 95 a8 03 00 00        	movq	%rdx, 0x3a8(%rbp)
1403bb46e: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bb475: ba 01 00 00 00              	movl	$0x1, %edx
1403bb47a: 41 b1 01                    	movb	$0x1, %r9b
1403bb47d: e8 5e 06 50 02              	callq	0x1428bbae0 <.text+0x28baae0>
1403bb482: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bb489: 44 89 fa                    	movl	%r15d, %edx
1403bb48c: 44 8b 85 88 07 00 00        	movl	0x788(%rbp), %r8d
1403bb493: e8 a8 d1 f0 ff              	callq	0x1402c8640 <.text+0x2c7640>
1403bb498: 66 0f 6f 85 80 03 00 00     	movdqa	0x380(%rbp), %xmm0
1403bb4a0: 66 0f 6f 8d 90 03 00 00     	movdqa	0x390(%rbp), %xmm1
1403bb4a8: 66 0f 6f 95 a0 03 00 00     	movdqa	0x3a0(%rbp), %xmm2
1403bb4b0: 66 0f 7f 95 b0 06 00 00     	movdqa	%xmm2, 0x6b0(%rbp)
1403bb4b8: 66 0f 7f 8d a0 06 00 00     	movdqa	%xmm1, 0x6a0(%rbp)
1403bb4c0: 66 0f 7f 85 90 06 00 00     	movdqa	%xmm0, 0x690(%rbp)
1403bb4c8: 48 8b 9d f0 06 00 00        	movq	0x6f0(%rbp), %rbx
1403bb4cf: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
1403bb4d3: eb 15                       	jmp	0x1403bb4ea <.text+0x3ba4ea>
1403bb4d5: 48 8b 85 e8 06 00 00        	movq	0x6e8(%rbp), %rax
1403bb4dc: 44 89 24 98                 	movl	%r12d, (%rax,%rbx,4)
1403bb4e0: 48 ff c3                    	incq	%rbx
1403bb4e3: 48 89 9d f0 06 00 00        	movq	%rbx, 0x6f0(%rbp)
1403bb4ea: 48 85 db                    	testq	%rbx, %rbx
1403bb4ed: 0f 84 fb 12 00 00           	je	0x1403bc7ee <.text+0x3bb7ee>
1403bb4f3: 48 8b 85 e8 06 00 00        	movq	0x6e8(%rbp), %rax
1403bb4fa: 48 8d 04 98                 	leaq	(%rax,%rbx,4), %rax
1403bb4fe: 48 83 c0 fc                 	addq	$-0x4, %rax
1403bb502: 48 85 c0                    	testq	%rax, %rax
1403bb505: 0f 84 e3 12 00 00           	je	0x1403bc7ee <.text+0x3bb7ee>
1403bb50b: 8b 00                       	movl	(%rax), %eax
1403bb50d: 89 85 88 07 00 00           	movl	%eax, 0x788(%rbp)
1403bb513: 48 8b 8d b8 02 00 00        	movq	0x2b8(%rbp), %rcx
1403bb51a: e8 91 33 54 00              	callq	0x1408fe8b0 <.text+0x8fd8b0>
1403bb51f: 49 89 c4                    	movq	%rax, %r12
1403bb522: 8b 85 88 07 00 00           	movl	0x788(%rbp), %eax
1403bb528: 89 85 80 03 00 00           	movl	%eax, 0x380(%rbp)
1403bb52e: 4d 8b 7c 24 10              	movq	0x10(%r12), %r15
1403bb533: 41 bd 04 00 00 00           	movl	$0x4, %r13d
1403bb539: 4d 85 ff                    	testq	%r15, %r15
1403bb53c: 74 7b                       	je	0x1403bb5b9 <.text+0x3ba5b9>
1403bb53e: 49 83 ff 01                 	cmpq	$0x1, %r15
1403bb542: 75 1a                       	jne	0x1403bb55e <.text+0x3ba55e>
1403bb544: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1403bb549: 41 bf 04 00 00 00           	movl	$0x4, %r15d
1403bb54f: 8b 8d 88 07 00 00           	movl	0x788(%rbp), %ecx
1403bb555: 39 48 20                    	cmpl	%ecx, 0x20(%rax)
1403bb558: 75 65                       	jne	0x1403bb5bf <.text+0x3ba5bf>
1403bb55a: 31 d2                       	xorl	%edx, %edx
1403bb55c: eb 47                       	jmp	0x1403bb5a5 <.text+0x3ba5a5>
1403bb55e: 49 8b 4c 24 38              	movq	0x38(%r12), %rcx
1403bb563: 49 8b 54 24 40              	movq	0x40(%r12), %rdx
1403bb568: 44 8b 85 88 07 00 00        	movl	0x788(%rbp), %r8d
1403bb56f: e8 fc b5 d4 ff              	callq	0x140106b70 <.text+0x105b70>
1403bb574: 4c 89 e1                    	movq	%r12, %rcx
1403bb577: 48 89 c2                    	movq	%rax, %rdx
1403bb57a: 4c 8d 85 80 03 00 00        	leaq	0x380(%rbp), %r8
1403bb581: e8 0a fe fe ff              	callq	0x1403ab390 <.text+0x3aa390>
1403bb586: 48 83 f8 01                 	cmpq	$0x1, %rax
1403bb58a: 75 2d                       	jne	0x1403bb5b9 <.text+0x3ba5b9>
1403bb58c: 4c 39 fa                    	cmpq	%r15, %rdx
1403bb58f: 0f 83 9a 12 00 00           	jae	0x1403bc82f <.text+0x3bb82f>
1403bb595: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1403bb59a: 41 bf 04 00 00 00           	movl	$0x4, %r15d
1403bb5a0: 48 85 c0                    	testq	%rax, %rax
1403bb5a3: 74 1a                       	je	0x1403bb5bf <.text+0x3ba5bf>
1403bb5a5: 48 8d 0c 92                 	leaq	(%rdx,%rdx,4), %rcx
1403bb5a9: 4c 8b 7c c8 08              	movq	0x8(%rax,%rcx,8), %r15
1403bb5ae: 48 8b 44 c8 10              	movq	0x10(%rax,%rcx,8), %rax
1403bb5b3: 4d 8d 2c c7                 	leaq	(%r15,%rax,8), %r13
1403bb5b7: eb 06                       	jmp	0x1403bb5bf <.text+0x3ba5bf>
1403bb5b9: 41 bf 04 00 00 00           	movl	$0x4, %r15d
1403bb5bf: 48 83 bd a8 06 00 00 00     	cmpq	$0x0, 0x6a8(%rbp)
1403bb5c7: 0f 84 f2 00 00 00           	je	0x1403bb6bf <.text+0x3ba6bf>
1403bb5cd: 4c 8b 95 90 06 00 00        	movq	0x690(%rbp), %r10
1403bb5d4: 4c 8b 9d 98 06 00 00        	movq	0x698(%rbp), %r11
1403bb5db: eb 77                       	jmp	0x1403bb654 <.text+0x3ba654>
1403bb5dd: 4c 21 f0                    	andq	%r14, %rax
1403bb5e0: f3 0f 6f 14 03              	movdqu	(%rbx,%rax), %xmm2
1403bb5e5: 66 0f 6f da                 	movdqa	%xmm2, %xmm3
1403bb5e9: 66 0f 74 d8                 	pcmpeqb	%xmm0, %xmm3
1403bb5ed: 66 0f d7 d3                 	pmovmskb	%xmm3, %edx
1403bb5f1: 85 d2                       	testl	%edx, %edx
1403bb5f3: 74 42                       	je	0x1403bb637 <.text+0x3ba637>
1403bb5f5: 4d 89 f3                    	movq	%r14, %r11
1403bb5f8: 49 89 da                    	movq	%rbx, %r10
1403bb5fb: f3 44 0f bc c2              	tzcntl	%edx, %r8d
1403bb600: 49 01 c0                    	addq	%rax, %r8
1403bb603: 4d 21 d8                    	andq	%r11, %r8
1403bb606: 49 c1 e0 03                 	shlq	$0x3, %r8
1403bb60a: 4d 89 d1                    	movq	%r10, %r9
1403bb60d: 4d 29 c1                    	subq	%r8, %r9
1403bb610: f3 41 0f 7e 59 f8           	movq	-0x8(%r9), %xmm3
1403bb616: 66 0f 76 d9                 	pcmpeqd	%xmm1, %xmm3
1403bb61a: 66 0f 70 db 50              	pshufd	$0x50, %xmm3, %xmm3     # xmm3 = xmm3[0,0,1,1]
1403bb61f: 66 44 0f 50 c3              	movmskpd	%xmm3, %r8d
1403bb624: 41 83 f8 03                 	cmpl	$0x3, %r8d
1403bb628: 74 2a                       	je	0x1403bb654 <.text+0x3ba654>
1403bb62a: 44 8d 42 ff                 	leal	-0x1(%rdx), %r8d
1403bb62e: 66 41 21 d0                 	andw	%dx, %r8w
1403bb632: 44 89 c2                    	movl	%r8d, %edx
1403bb635: 75 c4                       	jne	0x1403bb5fb <.text+0x3ba5fb>
1403bb637: 66 0f 74 d6                 	pcmpeqb	%xmm6, %xmm2
1403bb63b: 66 0f d7 d2                 	pmovmskb	%xmm2, %edx
1403bb63f: 85 d2                       	testl	%edx, %edx
1403bb641: 0f 85 8e 00 00 00           	jne	0x1403bb6d5 <.text+0x3ba6d5>
1403bb647: 48 01 c8                    	addq	%rcx, %rax
1403bb64a: 48 83 c0 10                 	addq	$0x10, %rax
1403bb64e: 48 83 c1 10                 	addq	$0x10, %rcx
1403bb652: eb 89                       	jmp	0x1403bb5dd <.text+0x3ba5dd>
1403bb654: 4d 39 ef                    	cmpq	%r13, %r15
1403bb657: 0f 84 cd 00 00 00           	je	0x1403bb72a <.text+0x3ba72a>
1403bb65d: 4d 89 de                    	movq	%r11, %r14
1403bb660: 4c 89 d3                    	movq	%r10, %rbx
1403bb663: 45 8b 27                    	movl	(%r15), %r12d
1403bb666: 8b 8d 88 07 00 00           	movl	0x788(%rbp), %ecx
1403bb66c: 44 89 e2                    	movl	%r12d, %edx
1403bb66f: e8 ec 5b 54 00              	callq	0x140901260 <.text+0x900260>
1403bb674: 89 85 80 03 00 00           	movl	%eax, 0x380(%rbp)
1403bb67a: 89 95 84 03 00 00           	movl	%edx, 0x384(%rbp)
1403bb680: 48 8d 8d b0 06 00 00        	leaq	0x6b0(%rbp), %rcx
1403bb687: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403bb68e: e8 4d 65 1f 00              	callq	0x1405b1be0 <.text+0x5b0be0>
1403bb693: 49 83 c7 08                 	addq	$0x8, %r15
1403bb697: 48 89 c1                    	movq	%rax, %rcx
1403bb69a: 48 c1 e9 39                 	shrq	$0x39, %rcx
1403bb69e: 66 0f 6e c1                 	movd	%ecx, %xmm0
1403bb6a2: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1403bb6a6: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1403bb6ab: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1403bb6b0: f3 0f 7e 8d 80 03 00 00     	movq	0x380(%rbp), %xmm1
1403bb6b8: 31 c9                       	xorl	%ecx, %ecx
1403bb6ba: e9 1e ff ff ff              	jmp	0x1403bb5dd <.text+0x3ba5dd>
1403bb6bf: 4d 39 ef                    	cmpq	%r13, %r15
1403bb6c2: 74 66                       	je	0x1403bb72a <.text+0x3ba72a>
1403bb6c4: 45 8b 27                    	movl	(%r15), %r12d
1403bb6c7: 8b 8d 88 07 00 00           	movl	0x788(%rbp), %ecx
1403bb6cd: 44 89 e2                    	movl	%r12d, %edx
1403bb6d0: e8 8b 5b 54 00              	callq	0x140901260 <.text+0x900260>
1403bb6d5: b8 01 00 00 00              	movl	$0x1, %eax
1403bb6da: 83 f8 01                    	cmpl	$0x1, %eax
1403bb6dd: 75 52                       	jne	0x1403bb731 <.text+0x3ba731>
1403bb6df: 8b 8d 88 07 00 00           	movl	0x788(%rbp), %ecx
1403bb6e5: 44 89 e2                    	movl	%r12d, %edx
1403bb6e8: e8 73 5b 54 00              	callq	0x140901260 <.text+0x900260>
1403bb6ed: 41 89 d0                    	movl	%edx, %r8d
1403bb6f0: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403bb6f7: 89 c2                       	movl	%eax, %edx
1403bb6f9: e8 42 cf f0 ff              	callq	0x1402c8640 <.text+0x2c7640>
1403bb6fe: 48 8b 9d f0 06 00 00        	movq	0x6f0(%rbp), %rbx
1403bb705: 48 3b 9d e0 06 00 00        	cmpq	0x6e0(%rbp), %rbx
1403bb70c: 0f 85 c3 fd ff ff           	jne	0x1403bb4d5 <.text+0x3ba4d5>
1403bb712: 48 8d 8d e0 06 00 00        	leaq	0x6e0(%rbp), %rcx
1403bb719: 48 8d 15 98 21 61 02        	leaq	0x2612198(%rip), %rdx   # 0x1429cd8b8
1403bb720: e8 4b 41 7a 00              	callq	0x140b5f870 <.text+0xb5e870>
1403bb725: e9 ab fd ff ff              	jmp	0x1403bb4d5 <.text+0x3ba4d5>
1403bb72a: 31 c0                       	xorl	%eax, %eax
1403bb72c: 83 f8 01                    	cmpl	$0x1, %eax
1403bb72f: 74 ae                       	je	0x1403bb6df <.text+0x3ba6df>
1403bb731: 48 c7 85 c0 06 00 00 00 00 00 00    	movq	$0x0, 0x6c0(%rbp)
1403bb73c: 48 c7 85 c8 06 00 00 04 00 00 00    	movq	$0x4, 0x6c8(%rbp)
1403bb747: 48 c7 85 d0 06 00 00 00 00 00 00    	movq	$0x0, 0x6d0(%rbp)
1403bb752: 48 8b 9d e8 06 00 00        	movq	0x6e8(%rbp), %rbx
1403bb759: 48 8b 85 f0 06 00 00        	movq	0x6f0(%rbp), %rax
1403bb760: 4c 8d 2c 83                 	leaq	(%rbx,%rax,4), %r13
1403bb764: 48 8b 85 30 07 00 00        	movq	0x730(%rbp), %rax
1403bb76b: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1403bb76f: 48 69 50 10 50 01 00 00     	imulq	$0x150, 0x10(%rax), %rdx # imm = 0x150
1403bb777: 48 89 8d 58 07 00 00        	movq	%rcx, 0x758(%rbp)
1403bb77e: 48 01 ca                    	addq	%rcx, %rdx
1403bb781: 48 89 95 88 07 00 00        	movq	%rdx, 0x788(%rbp)
1403bb788: 48 83 c0 60                 	addq	$0x60, %rax
1403bb78c: 48 89 85 60 07 00 00        	movq	%rax, 0x760(%rbp)
1403bb793: b0 01                       	movb	$0x1, %al
1403bb795: 0f 28 35 e4 31 57 02        	movaps	0x25731e4(%rip), %xmm6  # 0x14292e980
1403bb79c: 0f 28 3d 5d 1b 61 02        	movaps	0x2611b5d(%rip), %xmm7  # 0x1429cd300
1403bb7a3: f3 44 0f 10 05 c8 4e 5a 02  	movss	0x25a4ec8(%rip), %xmm8  # 0x142960674
1403bb7ac: 4c 89 ad 38 07 00 00        	movq	%r13, 0x738(%rbp)
1403bb7b3: eb 09                       	jmp	0x1403bb7be <.text+0x3ba7be>
1403bb7b5: 31 c0                       	xorl	%eax, %eax
1403bb7b7: 48 8b 9d 70 07 00 00        	movq	0x770(%rbp), %rbx
1403bb7be: 4c 39 eb                    	cmpq	%r13, %rbx
1403bb7c1: 0f 84 5d 04 00 00           	je	0x1403bbc24 <.text+0x3bac24>
1403bb7c7: 48 8d 4b 04                 	leaq	0x4(%rbx), %rcx
1403bb7cb: a8 01                       	testb	$0x1, %al
1403bb7cd: 74 12                       	je	0x1403bb7e1 <.text+0x3ba7e1>
1403bb7cf: 4c 39 e9                    	cmpq	%r13, %rcx
1403bb7d2: 0f 84 4c 04 00 00           	je	0x1403bbc24 <.text+0x3bac24>
1403bb7d8: 48 8d 43 08                 	leaq	0x8(%rbx), %rax
1403bb7dc: 48 89 da                    	movq	%rbx, %rdx
1403bb7df: eb 0d                       	jmp	0x1403bb7ee <.text+0x3ba7ee>
1403bb7e1: 48 89 c8                    	movq	%rcx, %rax
1403bb7e4: 48 8b 95 00 07 00 00        	movq	0x700(%rbp), %rdx
1403bb7eb: 48 89 d9                    	movq	%rbx, %rcx
1403bb7ee: 48 89 85 70 07 00 00        	movq	%rax, 0x770(%rbp)
1403bb7f5: 8b 1a                       	movl	(%rdx), %ebx
1403bb7f7: 48 89 8d 00 07 00 00        	movq	%rcx, 0x700(%rbp)
1403bb7fe: 44 8b 21                    	movl	(%rcx), %r12d
1403bb801: 48 8b 8d 58 07 00 00        	movq	0x758(%rbp), %rcx
1403bb808: 48 3b 8d 88 07 00 00        	cmpq	0x788(%rbp), %rcx
1403bb80f: 74 a4                       	je	0x1403bb7b5 <.text+0x3ba7b5>
1403bb811: 49 89 cf                    	movq	%rcx, %r15
1403bb814: c6 85 97 07 00 00 01        	movb	$0x1, 0x797(%rbp)
1403bb81b: 89 d9                       	movl	%ebx, %ecx
1403bb81d: 44 89 e2                    	movl	%r12d, %edx
1403bb820: e8 3b 5a 54 00              	callq	0x140901260 <.text+0x900260>
1403bb825: 49 8d 8f 50 01 00 00        	leaq	0x150(%r15), %rcx
1403bb82c: 41 39 87 28 01 00 00        	cmpl	%eax, 0x128(%r15)
1403bb833: 75 d3                       	jne	0x1403bb808 <.text+0x3ba808>
1403bb835: 41 39 97 2c 01 00 00        	cmpl	%edx, 0x12c(%r15)
1403bb83c: 75 ca                       	jne	0x1403bb808 <.text+0x3ba808>
1403bb83e: b8 04 00 00 00              	movl	$0x4, %eax
1403bb843: 48 89 85 40 07 00 00        	movq	%rax, 0x740(%rbp)
1403bb84a: 31 c0                       	xorl	%eax, %eax
1403bb84c: 49 3b 87 98 00 00 00        	cmpq	0x98(%r15), %rax
1403bb853: b8 00 00 00 00              	movl	$0x0, %eax
1403bb858: 48 89 85 68 07 00 00        	movq	%rax, 0x768(%rbp)
1403bb85f: 41 bd 00 00 00 00           	movl	$0x0, %r13d
1403bb865: 0f 80 05 01 00 00           	jo	0x1403bb970 <.text+0x3ba970>
1403bb86b: 49 8b 87 a8 00 00 00        	movq	0xa8(%r15), %rax
1403bb872: 48 89 85 28 07 00 00        	movq	%rax, 0x728(%rbp)
1403bb879: b9 0c 00 00 00              	movl	$0xc, %ecx
1403bb87e: 48 f7 e1                    	mulq	%rcx
1403bb881: 49 89 c6                    	movq	%rax, %r14
1403bb884: 0f 80 50 10 00 00           	jo	0x1403bc8da <.text+0x3bb8da>
1403bb88a: b9 00 00 00 00              	movl	$0x0, %ecx
1403bb88f: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
1403bb899: 49 39 c6                    	cmpq	%rax, %r14
1403bb89c: 0f 87 3a 10 00 00           	ja	0x1403bc8dc <.text+0x3bb8dc>
1403bb8a2: 4d 8b af a0 00 00 00        	movq	0xa0(%r15), %r13
1403bb8a9: 4d 85 f6                    	testq	%r14, %r14
1403bb8ac: 74 39                       	je	0x1403bb8e7 <.text+0x3ba8e7>
1403bb8ae: 0f b6 05 54 93 27 03        	movzbl	0x3279354(%rip), %eax   # 0x143634c09
1403bb8b5: ba 04 00 00 00              	movl	$0x4, %edx
1403bb8ba: 4c 89 f1                    	movq	%r14, %rcx
1403bb8bd: e8 4e 83 25 00              	callq	0x140613c10 <.text+0x612c10>
1403bb8c2: b9 04 00 00 00              	movl	$0x4, %ecx
1403bb8c7: 48 89 85 40 07 00 00        	movq	%rax, 0x740(%rbp)
1403bb8ce: 48 85 c0                    	testq	%rax, %rax
1403bb8d1: 0f 84 05 10 00 00           	je	0x1403bc8dc <.text+0x3bb8dc>
1403bb8d7: 48 8b 85 28 07 00 00        	movq	0x728(%rbp), %rax
1403bb8de: 48 89 85 68 07 00 00        	movq	%rax, 0x768(%rbp)
1403bb8e5: eb 17                       	jmp	0x1403bb8fe <.text+0x3ba8fe>
1403bb8e7: b8 04 00 00 00              	movl	$0x4, %eax
1403bb8ec: 48 89 85 40 07 00 00        	movq	%rax, 0x740(%rbp)
1403bb8f3: 48 c7 85 68 07 00 00 00 00 00 00    	movq	$0x0, 0x768(%rbp)
1403bb8fe: 48 8b 8d 40 07 00 00        	movq	0x740(%rbp), %rcx
1403bb905: 4c 89 ea                    	movq	%r13, %rdx
1403bb908: 4d 89 f0                    	movq	%r14, %r8
1403bb90b: e8 f0 82 56 02              	callq	0x142923c00 <.text+0x2922c00>
1403bb910: 45 3b a7 28 01 00 00        	cmpl	0x128(%r15), %r12d
1403bb917: 75 50                       	jne	0x1403bb969 <.text+0x3ba969>
1403bb919: 4c 8b ad 28 07 00 00        	movq	0x728(%rbp), %r13
1403bb920: 49 83 fd 02                 	cmpq	$0x2, %r13
1403bb924: 72 4a                       	jb	0x1403bb970 <.text+0x3ba970>
1403bb926: 4c 89 e8                    	movq	%r13, %rax
1403bb929: 48 d1 e8                    	shrq	%rax
1403bb92c: 48 8b 95 40 07 00 00        	movq	0x740(%rbp), %rdx
1403bb933: 48 8d 4a 08                 	leaq	0x8(%rdx), %rcx
1403bb937: 4c 01 f2                    	addq	%r14, %rdx
1403bb93a: 48 83 c2 fc                 	addq	$-0x4, %rdx
1403bb93e: 4c 8b 41 f8                 	movq	-0x8(%rcx), %r8
1403bb942: 4c 8b 4a f8                 	movq	-0x8(%rdx), %r9
1403bb946: 4c 89 49 f8                 	movq	%r9, -0x8(%rcx)
1403bb94a: 4c 89 42 f8                 	movq	%r8, -0x8(%rdx)
1403bb94e: 44 8b 01                    	movl	(%rcx), %r8d
1403bb951: 44 8b 0a                    	movl	(%rdx), %r9d
1403bb954: 44 89 09                    	movl	%r9d, (%rcx)
1403bb957: 44 89 02                    	movl	%r8d, (%rdx)
1403bb95a: 48 83 c1 0c                 	addq	$0xc, %rcx
1403bb95e: 48 83 c2 f4                 	addq	$-0xc, %rdx
1403bb962: 48 ff c8                    	decq	%rax
1403bb965: 75 d7                       	jne	0x1403bb93e <.text+0x3ba93e>
1403bb967: eb 07                       	jmp	0x1403bb970 <.text+0x3ba970>
1403bb969: 4c 8b ad 28 07 00 00        	movq	0x728(%rbp), %r13
1403bb970: 89 9d 80 03 00 00           	movl	%ebx, 0x380(%rbp)
1403bb976: 44 89 a5 84 03 00 00        	movl	%r12d, 0x384(%rbp)
1403bb97d: 48 8b 85 30 07 00 00        	movq	0x730(%rbp), %rax
1403bb984: 4c 8b 78 70                 	movq	0x70(%rax), %r15
1403bb988: 49 83 ff 01                 	cmpq	$0x1, %r15
1403bb98c: 74 5f                       	je	0x1403bb9ed <.text+0x3ba9ed>
1403bb98e: 4d 85 ff                    	testq	%r15, %r15
1403bb991: 0f 84 68 0e 00 00           	je	0x1403bc7ff <.text+0x3bb7ff>
1403bb997: 48 8b 88 98 00 00 00        	movq	0x98(%rax), %rcx
1403bb99e: 48 8b 90 a0 00 00 00        	movq	0xa0(%rax), %rdx
1403bb9a5: 41 89 d8                    	movl	%ebx, %r8d
1403bb9a8: 45 89 e1                    	movl	%r12d, %r9d
1403bb9ab: e8 90 ae d4 ff              	callq	0x140106840 <.text+0x105840>
1403bb9b0: c6 85 96 07 00 00 01        	movb	$0x1, 0x796(%rbp)
1403bb9b7: 48 8b 8d 60 07 00 00        	movq	0x760(%rbp), %rcx
1403bb9be: 48 89 c2                    	movq	%rax, %rdx
1403bb9c1: 4c 8d 85 80 03 00 00        	leaq	0x380(%rbp), %r8
1403bb9c8: e8 f3 f7 fe ff              	callq	0x1403ab1c0 <.text+0x3aa1c0>
1403bb9cd: 48 83 f8 01                 	cmpq	$0x1, %rax
1403bb9d1: 48 8b 85 30 07 00 00        	movq	0x730(%rbp), %rax
1403bb9d8: 0f 85 21 0e 00 00           	jne	0x1403bc7ff <.text+0x3bb7ff>
1403bb9de: 4c 39 fa                    	cmpq	%r15, %rdx
1403bb9e1: 0f 83 5f 0e 00 00           	jae	0x1403bc846 <.text+0x3bb846>
1403bb9e7: 48 8b 40 68                 	movq	0x68(%rax), %rax
1403bb9eb: eb 19                       	jmp	0x1403bba06 <.text+0x3baa06>
1403bb9ed: 48 8b 40 68                 	movq	0x68(%rax), %rax
1403bb9f1: 39 58 08                    	cmpl	%ebx, 0x8(%rax)
1403bb9f4: 0f 85 05 0e 00 00           	jne	0x1403bc7ff <.text+0x3bb7ff>
1403bb9fa: 44 39 60 0c                 	cmpl	%r12d, 0xc(%rax)
1403bb9fe: 0f 85 fb 0d 00 00           	jne	0x1403bc7ff <.text+0x3bb7ff>
1403bba04: 31 d2                       	xorl	%edx, %edx
1403bba06: 48 8d 0c 92                 	leaq	(%rdx,%rdx,4), %rcx
1403bba0a: 80 7c c8 10 00              	cmpb	$0x0, 0x10(%rax,%rcx,8)
1403bba0f: 75 3a                       	jne	0x1403bba4b <.text+0x3baa4b>
1403bba11: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1403bba15: 48 83 c0 10                 	addq	$0x10, %rax
1403bba19: 0f b6 48 02                 	movzbl	0x2(%rax), %ecx
1403bba1d: 0a 48 01                    	orb	0x1(%rax), %cl
1403bba20: 74 29                       	je	0x1403bba4b <.text+0x3baa4b>
1403bba22: 4d 85 ed                    	testq	%r13, %r13
1403bba25: 74 21                       	je	0x1403bba48 <.text+0x3baa48>
1403bba27: 48 8b 8d 40 07 00 00        	movq	0x740(%rbp), %rcx
1403bba2e: 48 8d 51 0c                 	leaq	0xc(%rcx), %rdx
1403bba32: 4a 8d 04 ad fc ff ff ff     	leaq	-0x4(,%r13,4), %rax
1403bba3a: 49 ff cd                    	decq	%r13
1403bba3d: 4c 8d 04 40                 	leaq	(%rax,%rax,2), %r8
1403bba41: e8 ca 81 56 02              	callq	0x142923c10 <.text+0x2922c10>
1403bba46: eb 03                       	jmp	0x1403bba4b <.text+0x3baa4b>
1403bba48: 45 31 ed                    	xorl	%r13d, %r13d
1403bba4b: 48 8b 85 30 07 00 00        	movq	0x730(%rbp), %rax
1403bba52: 44 89 a5 80 03 00 00        	movl	%r12d, 0x380(%rbp)
1403bba59: 89 9d 84 03 00 00           	movl	%ebx, 0x384(%rbp)
1403bba5f: 4c 8b 78 70                 	movq	0x70(%rax), %r15
1403bba63: 49 83 ff 01                 	cmpq	$0x1, %r15
1403bba67: 74 5f                       	je	0x1403bbac8 <.text+0x3baac8>
1403bba69: 4d 85 ff                    	testq	%r15, %r15
1403bba6c: 0f 84 a5 0d 00 00           	je	0x1403bc817 <.text+0x3bb817>
1403bba72: 48 8b 88 98 00 00 00        	movq	0x98(%rax), %rcx
1403bba79: 48 8b 90 a0 00 00 00        	movq	0xa0(%rax), %rdx
1403bba80: 45 89 e0                    	movl	%r12d, %r8d
1403bba83: 41 89 d9                    	movl	%ebx, %r9d
1403bba86: e8 b5 ad d4 ff              	callq	0x140106840 <.text+0x105840>
1403bba8b: c6 85 96 07 00 00 01        	movb	$0x1, 0x796(%rbp)
1403bba92: 48 8b 8d 60 07 00 00        	movq	0x760(%rbp), %rcx
1403bba99: 48 89 c2                    	movq	%rax, %rdx
1403bba9c: 4c 8d 85 80 03 00 00        	leaq	0x380(%rbp), %r8
1403bbaa3: e8 18 f7 fe ff              	callq	0x1403ab1c0 <.text+0x3aa1c0>
1403bbaa8: 48 83 f8 01                 	cmpq	$0x1, %rax
1403bbaac: 48 8b 85 30 07 00 00        	movq	0x730(%rbp), %rax
1403bbab3: 0f 85 5e 0d 00 00           	jne	0x1403bc817 <.text+0x3bb817>
1403bbab9: 4c 39 fa                    	cmpq	%r15, %rdx
1403bbabc: 0f 83 a2 0d 00 00           	jae	0x1403bc864 <.text+0x3bb864>
1403bbac2: 48 8b 40 68                 	movq	0x68(%rax), %rax
1403bbac6: eb 19                       	jmp	0x1403bbae1 <.text+0x3baae1>
1403bbac8: 48 8b 40 68                 	movq	0x68(%rax), %rax
1403bbacc: 44 39 60 08                 	cmpl	%r12d, 0x8(%rax)
1403bbad0: 0f 85 41 0d 00 00           	jne	0x1403bc817 <.text+0x3bb817>
1403bbad6: 39 58 0c                    	cmpl	%ebx, 0xc(%rax)
1403bbad9: 0f 85 38 0d 00 00           	jne	0x1403bc817 <.text+0x3bb817>
1403bbadf: 31 d2                       	xorl	%edx, %edx
1403bbae1: 48 8d 0c 92                 	leaq	(%rdx,%rdx,4), %rcx
1403bbae5: 80 7c c8 10 00              	cmpb	$0x0, 0x10(%rax,%rcx,8)
1403bbaea: 75 20                       	jne	0x1403bbb0c <.text+0x3bab0c>
1403bbaec: 48 8d 04 c8                 	leaq	(%rax,%rcx,8), %rax
1403bbaf0: 48 83 c0 10                 	addq	$0x10, %rax
1403bbaf4: 80 78 01 00                 	cmpb	$0x0, 0x1(%rax)
1403bbaf8: 75 06                       	jne	0x1403bbb00 <.text+0x3bab00>
1403bbafa: 80 78 02 01                 	cmpb	$0x1, 0x2(%rax)
1403bbafe: 75 0c                       	jne	0x1403bbb0c <.text+0x3bab0c>
1403bbb00: 4d 85 ed                    	testq	%r13, %r13
1403bbb03: 0f 84 da 00 00 00           	je	0x1403bbbe3 <.text+0x3babe3>
1403bbb09: 49 ff cd                    	decq	%r13
1403bbb0c: 4d 85 ed                    	testq	%r13, %r13
1403bbb0f: 0f 84 ce 00 00 00           	je	0x1403bbbe3 <.text+0x3babe3>
1403bbb15: 4a 8d 04 6d 00 00 00 00     	leaq	(,%r13,2), %rax
1403bbb1d: 4c 01 e8                    	addq	%r13, %rax
1403bbb20: 4c 8b a5 40 07 00 00        	movq	0x740(%rbp), %r12
1403bbb27: 49 8d 1c 84                 	leaq	(%r12,%rax,4), %rbx
1403bbb2b: 4c 8b bd d0 06 00 00        	movq	0x6d0(%rbp), %r15
1403bbb32: eb 2a                       	jmp	0x1403bbb5e <.text+0x3bab5e>
1403bbb34: 48 8b 85 c8 06 00 00        	movq	0x6c8(%rbp), %rax
1403bbb3b: 46 0f 13 0c a8              	movlps	%xmm9, (%rax,%r13,4)
1403bbb40: f3 46 0f 11 54 a8 08        	movss	%xmm10, 0x8(%rax,%r13,4)
1403bbb47: 49 ff c7                    	incq	%r15
1403bbb4a: 4c 89 bd d0 06 00 00        	movq	%r15, 0x6d0(%rbp)
1403bbb51: 49 83 c4 0c                 	addq	$0xc, %r12
1403bbb55: 49 39 dc                    	cmpq	%rbx, %r12
1403bbb58: 0f 84 85 00 00 00           	je	0x1403bbbe3 <.text+0x3babe3>
1403bbb5e: f2 45 0f 10 0c 24           	movsd	(%r12), %xmm9
1403bbb64: f3 45 0f 10 54 24 08        	movss	0x8(%r12), %xmm10
1403bbb6b: 48 8b 85 c8 06 00 00        	movq	0x6c8(%rbp), %rax
1403bbb72: 4d 85 ff                    	testq	%r15, %r15
1403bbb75: 41 0f 94 c0                 	sete	%r8b
1403bbb79: 4f 8d 2c 7f                 	leaq	(%r15,%r15,2), %r13
1403bbb7d: 4a 8d 0c a8                 	leaq	(%rax,%r13,4), %rcx
1403bbb81: 48 89 ca                    	movq	%rcx, %rdx
1403bbb84: 48 83 c2 f4                 	addq	$-0xc, %rdx
1403bbb88: 41 0f 94 c1                 	sete	%r9b
1403bbb8c: 45 08 c1                    	orb	%r8b, %r9b
1403bbb8f: 75 2d                       	jne	0x1403bbbbe <.text+0x3babbe>
1403bbb91: f2 0f 10 02                 	movsd	(%rdx), %xmm0
1403bbb95: 41 0f 5c c1                 	subps	%xmm9, %xmm0
1403bbb99: 0f 54 c6                    	andps	%xmm6, %xmm0
1403bbb9c: 0f c2 c7 06                 	cmpnleps	%xmm7, %xmm0
1403bbba0: 0f 14 c0                    	unpcklps	%xmm0, %xmm0            # xmm0 = xmm0[0,0,1,1]
1403bbba3: 66 0f 50 d0                 	movmskpd	%xmm0, %edx
1403bbba7: 85 d2                       	testl	%edx, %edx
1403bbba9: 75 13                       	jne	0x1403bbbbe <.text+0x3babbe>
1403bbbab: f3 0f 10 41 fc              	movss	-0x4(%rcx), %xmm0
1403bbbb0: f3 41 0f 5c c2              	subss	%xmm10, %xmm0
1403bbbb5: 0f 54 c6                    	andps	%xmm6, %xmm0
1403bbbb8: 44 0f 2e c0                 	ucomiss	%xmm0, %xmm8
1403bbbbc: 73 93                       	jae	0x1403bbb51 <.text+0x3bab51>
1403bbbbe: 4c 3b bd c0 06 00 00        	cmpq	0x6c0(%rbp), %r15
1403bbbc5: 0f 85 70 ff ff ff           	jne	0x1403bbb3b <.text+0x3bab3b>
1403bbbcb: 48 8d 8d c0 06 00 00        	leaq	0x6c0(%rbp), %rcx
1403bbbd2: 48 8d 15 27 1d 61 02        	leaq	0x2611d27(%rip), %rdx   # 0x1429cd900
1403bbbd9: e8 a2 35 7e 00              	callq	0x140b9f180 <.text+0xb9e180>
1403bbbde: e9 51 ff ff ff              	jmp	0x1403bbb34 <.text+0x3bab34>
1403bbbe3: 31 c0                       	xorl	%eax, %eax
1403bbbe5: 48 8b 8d 68 07 00 00        	movq	0x768(%rbp), %rcx
1403bbbec: 48 85 c9                    	testq	%rcx, %rcx
1403bbbef: 4c 8b ad 38 07 00 00        	movq	0x738(%rbp), %r13
1403bbbf6: 48 8b 9d 70 07 00 00        	movq	0x770(%rbp), %rbx
1403bbbfd: 0f 84 bb fb ff ff           	je	0x1403bb7be <.text+0x3ba7be>
1403bbc03: 48 c1 e1 02                 	shlq	$0x2, %rcx
1403bbc07: 48 8d 14 49                 	leaq	(%rcx,%rcx,2), %rdx
1403bbc0b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bbc11: 48 8b 8d 40 07 00 00        	movq	0x740(%rbp), %rcx
1403bbc18: e8 03 80 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bbc1d: 31 c0                       	xorl	%eax, %eax
1403bbc1f: e9 9a fb ff ff              	jmp	0x1403bb7be <.text+0x3ba7be>
1403bbc24: 48 8b 85 d0 06 00 00        	movq	0x6d0(%rbp), %rax
1403bbc2b: 48 85 c0                    	testq	%rax, %rax
1403bbc2e: 0f 84 cc 00 00 00           	je	0x1403bbd00 <.text+0x3bad00>
1403bbc34: 48 8b 8d c8 06 00 00        	movq	0x6c8(%rbp), %rcx
1403bbc3b: 4c 8d 04 40                 	leaq	(%rax,%rax,2), %r8
1403bbc3f: 4a 8d 04 85 f4 ff ff ff     	leaq	-0xc(,%r8,4), %rax
1403bbc47: 49 89 c9                    	movq	%rcx, %r9
1403bbc4a: 48 83 f8 2f                 	cmpq	$0x2f, %rax
1403bbc4e: 0f 86 84 00 00 00           	jbe	0x1403bbcd8 <.text+0x3bacd8>
1403bbc54: 48 ba ab aa aa aa aa aa aa aa       	movabsq	$-0x5555555555555555, %rdx # imm = 0xAAAAAAAAAAAAAAAB
1403bbc5e: 48 f7 e2                    	mulq	%rdx
1403bbc61: 48 c1 ea 03                 	shrq	$0x3, %rdx
1403bbc65: 48 ff c2                    	incq	%rdx
1403bbc68: 89 d0                       	movl	%edx, %eax
1403bbc6a: 83 e0 03                    	andl	$0x3, %eax
1403bbc6d: 41 b9 04 00 00 00           	movl	$0x4, %r9d
1403bbc73: 4c 0f 45 c8                 	cmovneq	%rax, %r9
1403bbc77: 4c 29 ca                    	subq	%r9, %rdx
1403bbc7a: 48 8d 04 52                 	leaq	(%rdx,%rdx,2), %rax
1403bbc7e: 4c 8d 0c 81                 	leaq	(%rcx,%rax,4), %r9
1403bbc82: 48 8d 41 28                 	leaq	0x28(%rcx), %rax
1403bbc86: 0f 28 05 83 16 61 02        	movaps	0x2611683(%rip), %xmm0  # 0x1429cd310
1403bbc8d: f3 0f 10 48 dc              	movss	-0x24(%rax), %xmm1
1403bbc92: f3 0f 10 50 e8              	movss	-0x18(%rax), %xmm2
1403bbc97: 0f 14 ca                    	unpcklps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0],xmm1[1],xmm2[1]
1403bbc9a: f3 0f 10 50 f4              	movss	-0xc(%rax), %xmm2
1403bbc9f: f3 0f 10 18                 	movss	(%rax), %xmm3
1403bbca3: 0f 14 d3                    	unpcklps	%xmm3, %xmm2            # xmm2 = xmm2[0],xmm3[0],xmm2[1],xmm3[1]
1403bbca6: 0f 16 ca                    	movlhps	%xmm2, %xmm1            # xmm1 = xmm1[0],xmm2[0]
1403bbca9: 0f 5f c8                    	maxps	%xmm0, %xmm1
1403bbcac: f3 0f 11 48 dc              	movss	%xmm1, -0x24(%rax)
1403bbcb1: f3 0f 16 d1                 	movshdup	%xmm1, %xmm2            # xmm2 = xmm1[1,1,3,3]
1403bbcb5: f3 0f 11 50 e8              	movss	%xmm2, -0x18(%rax)
1403bbcba: 0f 28 d1                    	movaps	%xmm1, %xmm2
1403bbcbd: 66 0f 15 d1                 	unpckhpd	%xmm1, %xmm2            # xmm2 = xmm2[1],xmm1[1]
1403bbcc1: f3 0f 11 50 f4              	movss	%xmm2, -0xc(%rax)
1403bbcc6: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1403bbcca: f3 0f 11 08                 	movss	%xmm1, (%rax)
1403bbcce: 48 83 c0 30                 	addq	$0x30, %rax
1403bbcd2: 48 83 c2 fc                 	addq	$-0x4, %rdx
1403bbcd6: 75 b5                       	jne	0x1403bbc8d <.text+0x3bac8d>
1403bbcd8: 4a 8d 04 81                 	leaq	(%rcx,%r8,4), %rax
1403bbcdc: f3 0f 10 05 3c 16 61 02     	movss	0x261163c(%rip), %xmm0  # 0x1429cd320
1403bbce4: f3 41 0f 10 49 04           	movss	0x4(%r9), %xmm1
1403bbcea: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
1403bbcee: f3 41 0f 11 49 04           	movss	%xmm1, 0x4(%r9)
1403bbcf4: 49 8d 49 0c                 	leaq	0xc(%r9), %rcx
1403bbcf8: 49 89 c9                    	movq	%rcx, %r9
1403bbcfb: 48 39 c1                    	cmpq	%rax, %rcx
1403bbcfe: 75 e4                       	jne	0x1403bbce4 <.text+0x3bace4>
1403bbd00: 48 8b 85 d0 06 00 00        	movq	0x6d0(%rbp), %rax
1403bbd07: 48 89 85 c0 05 00 00        	movq	%rax, 0x5c0(%rbp)
1403bbd0e: f3 0f 6f 85 c0 06 00 00     	movdqu	0x6c0(%rbp), %xmm0
1403bbd16: 66 0f 7f 85 b0 05 00 00     	movdqa	%xmm0, 0x5b0(%rbp)
1403bbd1e: c6 85 97 07 00 00 00        	movb	$0x0, 0x797(%rbp)
1403bbd25: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bbd2c: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
1403bbd33: 45 31 c0                    	xorl	%r8d, %r8d
1403bbd36: e8 05 79 d9 ff              	callq	0x140153640 <.text+0x152640>
1403bbd3b: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1403bbd42: 48 89 c1                    	movq	%rax, %rcx
1403bbd45: 48 f7 d9                    	negq	%rcx
1403bbd48: f3 0f 6f 85 88 03 00 00     	movdqu	0x388(%rbp), %xmm0
1403bbd50: 66 0f 7f 85 80 00 00 00     	movdqa	%xmm0, 0x80(%rbp)
1403bbd58: 71 1b                       	jno	0x1403bbd75 <.text+0x3bad75>
1403bbd5a: c6 85 97 07 00 00 00        	movb	$0x0, 0x797(%rbp)
1403bbd61: 48 8d 8d 80 00 00 00        	leaq	0x80(%rbp), %rcx
1403bbd68: e8 23 83 4e 02              	callq	0x1428a4090 <.text+0x28a3090>
1403bbd6d: 48 89 c6                    	movq	%rax, %rsi
1403bbd70: e9 9b 00 00 00              	jmp	0x1403bbe10 <.text+0x3bae10>
1403bbd75: 0f 10 85 98 03 00 00        	movups	0x398(%rbp), %xmm0
1403bbd7c: f3 0f 6f 8d a8 03 00 00     	movdqu	0x3a8(%rbp), %xmm1
1403bbd84: f3 0f 7f 8d d8 00 00 00     	movdqu	%xmm1, 0xd8(%rbp)
1403bbd8c: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
1403bbd93: 66 0f 6f 85 80 00 00 00     	movdqa	0x80(%rbp), %xmm0
1403bbd9b: f3 0f 7f 85 b8 00 00 00     	movdqu	%xmm0, 0xb8(%rbp)
1403bbda3: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1403bbdaa: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403bbdb1: 48 8d 95 b0 00 00 00        	leaq	0xb0(%rbp), %rdx
1403bbdb8: 66 0f 6e 15 44 9b 56 02     	movd	0x2569b44(%rip), %xmm2  # 0x142925904
1403bbdc0: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1403bbdc6: e8 35 6d d9 ff              	callq	0x140152b00 <.text+0x151b00>
1403bbdcb: 48 8b 85 30 05 00 00        	movq	0x530(%rbp), %rax
1403bbdd2: 48 89 c1                    	movq	%rax, %rcx
1403bbdd5: 48 f7 d9                    	negq	%rcx
1403bbdd8: f3 0f 6f 85 38 05 00 00     	movdqu	0x538(%rbp), %xmm0
1403bbde0: 66 0f 7f 85 90 00 00 00     	movdqa	%xmm0, 0x90(%rbp)
1403bbde8: 0f 81 dc 01 00 00           	jno	0x1403bbfca <.text+0x3bafca>
1403bbdee: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1403bbdf5: e8 96 82 4e 02              	callq	0x1428a4090 <.text+0x28a3090>
1403bbdfa: 48 89 c6                    	movq	%rax, %rsi
1403bbdfd: c6 85 97 07 00 00 00        	movb	$0x0, 0x797(%rbp)
1403bbe04: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1403bbe0b: e8 10 fc d8 ff              	callq	0x14014ba20 <.text+0x14aa20>
1403bbe10: 48 8b 95 98 06 00 00        	movq	0x698(%rbp), %rdx
1403bbe17: 48 85 d2                    	testq	%rdx, %rdx
1403bbe1a: 74 2a                       	je	0x1403bbe46 <.text+0x3bae46>
1403bbe1c: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
1403bbe24: 48 83 e0 f0                 	andq	$-0x10, %rax
1403bbe28: 48 01 c2                    	addq	%rax, %rdx
1403bbe2b: 48 83 c2 11                 	addq	$0x11, %rdx
1403bbe2f: 74 15                       	je	0x1403bbe46 <.text+0x3bae46>
1403bbe31: 48 8b 8d 90 06 00 00        	movq	0x690(%rbp), %rcx
1403bbe38: 48 29 c1                    	subq	%rax, %rcx
1403bbe3b: 41 b8 10 00 00 00           	movl	$0x10, %r8d
1403bbe41: e8 da 7d 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bbe46: 48 8b 95 e0 06 00 00        	movq	0x6e0(%rbp), %rdx
1403bbe4d: 48 85 d2                    	testq	%rdx, %rdx
1403bbe50: 74 16                       	je	0x1403bbe68 <.text+0x3bae68>
1403bbe52: 48 8b 8d e8 06 00 00        	movq	0x6e8(%rbp), %rcx
1403bbe59: 48 c1 e2 02                 	shlq	$0x2, %rdx
1403bbe5d: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bbe63: e8 b8 7d 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bbe68: 48 89 f0                    	movq	%rsi, %rax
1403bbe6b: 48 89 85 30 05 00 00        	movq	%rax, 0x530(%rbp)
1403bbe72: 48 8b 05 2f 05 27 03        	movq	0x327052f(%rip), %rax   # 0x14362c3a8
1403bbe79: 48 8b 00                    	movq	(%rax), %rax
1403bbe7c: 48 83 f8 02                 	cmpq	$0x2, %rax
1403bbe80: 0f 82 dc 00 00 00           	jb	0x1403bbf62 <.text+0x3baf62>
1403bbe86: 48 8d 85 30 05 00 00        	leaq	0x530(%rbp), %rax
1403bbe8d: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1403bbe94: 48 8d 05 85 0d 99 00        	leaq	0x990d85(%rip), %rax    # 0x140d4cc20 <.text+0xd4bc20>
1403bbe9b: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
1403bbea2: 48 8d 0d c7 1b 61 02        	leaq	0x2611bc7(%rip), %rcx   # 0x1429cda70
1403bbea9: e8 d2 1e 2c 00              	callq	0x14067dd80 <.text+0x67cd80>
1403bbeae: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
1403bbeb2: 8b 40 10                    	movl	0x10(%rax), %eax
1403bbeb5: 48 c7 85 b0 03 00 00 02 00 00 00    	movq	$0x2, 0x3b0(%rbp)
1403bbec0: 48 8d 0d 79 18 61 02        	leaq	0x2611879(%rip), %rcx   # 0x1429cd740
1403bbec7: 48 89 8d b8 03 00 00        	movq	%rcx, 0x3b8(%rbp)
1403bbece: 48 c7 85 c0 03 00 00 19 00 00 00    	movq	$0x19, 0x3c0(%rbp)
1403bbed9: 48 8d 15 80 1b 61 02        	leaq	0x2611b80(%rip), %rdx   # 0x1429cda60
1403bbee0: 48 89 95 d0 03 00 00        	movq	%rdx, 0x3d0(%rbp)
1403bbee7: 48 c7 85 d8 03 00 00 01 00 00 00    	movq	$0x1, 0x3d8(%rbp)
1403bbef2: 48 89 bd e0 03 00 00        	movq	%rdi, 0x3e0(%rbp)
1403bbef9: 48 c7 85 e8 03 00 00 01 00 00 00    	movq	$0x1, 0x3e8(%rbp)
1403bbf04: 48 c7 85 f0 03 00 00 00 00 00 00    	movq	$0x0, 0x3f0(%rbp)
1403bbf0f: 48 c7 85 80 03 00 00 00 00 00 00    	movq	$0x0, 0x380(%rbp)
1403bbf1a: 48 89 8d 88 03 00 00        	movq	%rcx, 0x388(%rbp)
1403bbf21: 48 c7 85 90 03 00 00 19 00 00 00    	movq	$0x19, 0x390(%rbp)
1403bbf2c: 48 c7 85 98 03 00 00 00 00 00 00    	movq	$0x0, 0x398(%rbp)
1403bbf37: f3 0f 7f 85 a0 03 00 00     	movdqu	%xmm0, 0x3a0(%rbp)
1403bbf3f: c7 85 c8 03 00 00 01 00 00 00       	movl	$0x1, 0x3c8(%rbp)
1403bbf49: 89 85 cc 03 00 00           	movl	%eax, 0x3cc(%rbp)
1403bbf4f: 48 8d 8d 30 06 00 00        	leaq	0x630(%rbp), %rcx
1403bbf56: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403bbf5d: e8 ce e3 98 00              	callq	0x140d4a330 <.text+0xd49330>
1403bbf62: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403bbf69: e8 c2 0c 99 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1403bbf6e: 48 83 bd e8 05 00 00 00     	cmpq	$0x0, 0x5e8(%rbp)
1403bbf76: 0f 84 e2 ee ff ff           	je	0x1403bae5e <.text+0x3b9e5e>
1403bbf7c: e8 af c1 85 00              	callq	0x140c18130 <.text+0xc17130>
1403bbf81: 48 89 c1                    	movq	%rax, %rcx
1403bbf84: 48 8b 00                    	movq	(%rax), %rax
1403bbf87: 48 83 f8 01                 	cmpq	$0x1, %rax
1403bbf8b: 0f 85 2b 01 00 00           	jne	0x1403bc0bc <.text+0x3bb0bc>
1403bbf91: 48 83 c1 08                 	addq	$0x8, %rcx
1403bbf95: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1403bbf99: 75 23                       	jne	0x1403bbfbe <.text+0x3bafbe>
1403bbf9b: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1403bbfa2: 48 89 8d 88 07 00 00        	movq	%rcx, 0x788(%rbp)
1403bbfa9: 48 83 c1 08                 	addq	$0x8, %rcx
1403bbfad: 48 8b 95 f0 05 00 00        	movq	0x5f0(%rbp), %rdx
1403bbfb4: e8 37 b1 85 00              	callq	0x140c170f0 <.text+0xc160f0>
1403bbfb9: e9 96 ee ff ff              	jmp	0x1403bae54 <.text+0x3b9e54>
1403bbfbe: 48 8d 0d f3 6a 57 02        	leaq	0x2576af3(%rip), %rcx   # 0x142932ab8
1403bbfc5: e8 86 d2 51 02              	callq	0x1428d9250 <.text+0x28d8250>
1403bbfca: 0f 10 85 48 05 00 00        	movups	0x548(%rbp), %xmm0
1403bbfd1: f3 0f 6f 8d 58 05 00 00     	movdqu	0x558(%rbp), %xmm1
1403bbfd9: f3 0f 7f 8d 58 06 00 00     	movdqu	%xmm1, 0x658(%rbp)
1403bbfe1: 0f 11 85 48 06 00 00        	movups	%xmm0, 0x648(%rbp)
1403bbfe8: 66 0f 6f 85 90 00 00 00     	movdqa	0x90(%rbp), %xmm0
1403bbff0: f3 0f 7f 85 38 06 00 00     	movdqu	%xmm0, 0x638(%rbp)
1403bbff8: 48 89 85 30 06 00 00        	movq	%rax, 0x630(%rbp)
1403bbfff: c6 85 95 07 00 00 01        	movb	$0x1, 0x795(%rbp)
1403bc006: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1403bc00d: e8 0e fa d8 ff              	callq	0x14014ba20 <.text+0x14aa20>
1403bc012: 48 8b 85 60 06 00 00        	movq	0x660(%rbp), %rax
1403bc019: 48 89 85 b0 03 00 00        	movq	%rax, 0x3b0(%rbp)
1403bc020: f3 0f 6f 85 30 06 00 00     	movdqu	0x630(%rbp), %xmm0
1403bc028: f3 0f 6f 8d 40 06 00 00     	movdqu	0x640(%rbp), %xmm1
1403bc030: f3 0f 6f 95 50 06 00 00     	movdqu	0x650(%rbp), %xmm2
1403bc038: 66 0f 7f 95 a0 03 00 00     	movdqa	%xmm2, 0x3a0(%rbp)
1403bc040: 66 0f 7f 8d 90 03 00 00     	movdqa	%xmm1, 0x390(%rbp)
1403bc048: 66 0f 7f 85 80 03 00 00     	movdqa	%xmm0, 0x380(%rbp)
1403bc050: c6 85 95 07 00 00 00        	movb	$0x0, 0x795(%rbp)
1403bc057: 48 8d 9d b0 00 00 00        	leaq	0xb0(%rbp), %rbx
1403bc05e: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403bc065: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bc06b: 48 89 d9                    	movq	%rbx, %rcx
1403bc06e: e8 1d 84 d9 ff              	callq	0x140154490 <.text+0x153490>
1403bc073: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1403bc07a: 48 89 c1                    	movq	%rax, %rcx
1403bc07d: 48 f7 d9                    	negq	%rcx
1403bc080: f3 0f 6f 85 b8 00 00 00     	movdqu	0xb8(%rbp), %xmm0
1403bc088: 66 0f 7f 45 70              	movdqa	%xmm0, 0x70(%rbp)
1403bc08d: 71 4e                       	jno	0x1403bc0dd <.text+0x3bb0dd>
1403bc08f: c6 85 95 07 00 00 00        	movb	$0x0, 0x795(%rbp)
1403bc096: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
1403bc09a: e8 f1 7f 4e 02              	callq	0x1428a4090 <.text+0x28a3090>
1403bc09f: e9 c9 fc ff ff              	jmp	0x1403bbd6d <.text+0x3bad6d>
1403bc0a4: 4c 8d 05 f5 18 61 02        	leaq	0x26118f5(%rip), %r8    # 0x1429cd9a0
1403bc0ab: 48 8b 95 38 07 00 00        	movq	0x738(%rbp), %rdx
1403bc0b2: e8 61 d4 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc0b7: e9 5b 08 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc0bc: 48 85 c0                    	testq	%rax, %rax
1403bc0bf: 0f 85 89 ef ff ff           	jne	0x1403bb04e <.text+0x3ba04e>
1403bc0c5: 31 d2                       	xorl	%edx, %edx
1403bc0c7: e8 74 55 4e 02              	callq	0x1428a1640 <.text+0x28a0640>
1403bc0cc: 48 89 c1                    	movq	%rax, %rcx
1403bc0cf: 48 85 c0                    	testq	%rax, %rax
1403bc0d2: 0f 85 bd fe ff ff           	jne	0x1403bbf95 <.text+0x3baf95>
1403bc0d8: e9 71 ef ff ff              	jmp	0x1403bb04e <.text+0x3ba04e>
1403bc0dd: 0f 10 85 c8 00 00 00        	movups	0xc8(%rbp), %xmm0
1403bc0e4: f3 0f 6f 8d d8 00 00 00     	movdqu	0xd8(%rbp), %xmm1
1403bc0ec: f3 0f 7f 8d 58 05 00 00     	movdqu	%xmm1, 0x558(%rbp)
1403bc0f4: 0f 11 85 48 05 00 00        	movups	%xmm0, 0x548(%rbp)
1403bc0fb: 66 0f 6f 45 70              	movdqa	0x70(%rbp), %xmm0
1403bc100: f3 0f 7f 85 38 05 00 00     	movdqu	%xmm0, 0x538(%rbp)
1403bc108: 48 89 85 30 05 00 00        	movq	%rax, 0x530(%rbp)
1403bc10f: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
1403bc116: e8 95 e2 98 00              	callq	0x140d4a3b0 <.text+0xd493b0>
1403bc11b: 4c 8b 85 38 05 00 00        	movq	0x538(%rbp), %r8
1403bc122: 4c 8b 8d 40 05 00 00        	movq	0x540(%rbp), %r9
1403bc129: c7 85 e8 02 00 00 00 00 00 00       	movl	$0x0, 0x2e8(%rbp)
1403bc133: 8b 85 e8 02 00 00           	movl	0x2e8(%rbp), %eax
1403bc139: 89 85 b0 00 00 00           	movl	%eax, 0xb0(%rbp)
1403bc13f: 48 8b 85 ec 02 00 00        	movq	0x2ec(%rbp), %rax
1403bc146: 48 89 85 b4 00 00 00        	movq	%rax, 0xb4(%rbp)
1403bc14d: 8b 85 f4 02 00 00           	movl	0x2f4(%rbp), %eax
1403bc153: 89 85 bc 00 00 00           	movl	%eax, 0xbc(%rbp)
1403bc159: 48 8d 85 e8 02 00 00        	leaq	0x2e8(%rbp), %rax
1403bc160: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1403bc165: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1403bc16a: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bc171: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
1403bc178: e8 f3 77 8d 00              	callq	0x140c93970 <.text+0xc92970>
1403bc17d: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1403bc184: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403bc18b: e8 b0 87 8d 00              	callq	0x140c94940 <.text+0xc93940>
1403bc190: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
1403bc197: e8 84 a4 22 00              	callq	0x1405e6620 <.text+0x5e5620>
1403bc19c: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1403bc1a3: 48 89 85 88 07 00 00        	movq	%rax, 0x788(%rbp)
1403bc1aa: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
1403bc1b1: 48 89 85 70 07 00 00        	movq	%rax, 0x770(%rbp)
1403bc1b8: f3 0f 6f 85 c0 00 00 00     	movdqu	0xc0(%rbp), %xmm0
1403bc1c0: 66 0f 7f 45 00              	movdqa	%xmm0, (%rbp)
1403bc1c5: c6 85 95 07 00 00 00        	movb	$0x0, 0x795(%rbp)
1403bc1cc: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403bc1d3: e8 48 f8 d8 ff              	callq	0x14014ba20 <.text+0x14aa20>
1403bc1d8: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403bc1df: e8 6c dd d8 ff              	callq	0x140149f50 <.text+0x148f50>
1403bc1e4: 48 8b 95 e0 06 00 00        	movq	0x6e0(%rbp), %rdx
1403bc1eb: 48 85 d2                    	testq	%rdx, %rdx
1403bc1ee: 74 16                       	je	0x1403bc206 <.text+0x3bb206>
1403bc1f0: 48 8b 8d e8 06 00 00        	movq	0x6e8(%rbp), %rcx
1403bc1f7: 48 c1 e2 02                 	shlq	$0x2, %rdx
1403bc1fb: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bc201: e8 1a 7a 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bc206: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
1403bc210: 48 39 85 88 07 00 00        	cmpq	%rax, 0x788(%rbp)
1403bc217: 75 0c                       	jne	0x1403bc225 <.text+0x3bb225>
1403bc219: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403bc220: e9 46 fc ff ff              	jmp	0x1403bbe6b <.text+0x3bae6b>
1403bc225: 0f 28 45 00                 	movaps	(%rbp), %xmm0
1403bc229: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1403bc22d: c6 85 7f 07 00 00 01        	movb	$0x1, 0x77f(%rbp)
1403bc234: 48 8b 85 f0 08 00 00        	movq	0x8f0(%rbp), %rax
1403bc23b: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
1403bc240: 48 8d 85 f8 02 00 00        	leaq	0x2f8(%rbp), %rax
1403bc247: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
1403bc24c: 48 8b 85 c8 08 00 00        	movq	0x8c8(%rbp), %rax
1403bc253: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1403bc258: 48 8b 85 28 06 00 00        	movq	0x628(%rbp), %rax
1403bc25f: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403bc264: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403bc26b: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
1403bc26f: 48 8b 95 c8 02 00 00        	movq	0x2c8(%rbp), %rdx
1403bc276: 4c 8b 8d 60 03 00 00        	movq	0x360(%rbp), %r9
1403bc27d: e8 1e 1f 00 00              	callq	0x1403be1a0 <.text+0x3bd1a0>
1403bc282: 48 8b 85 a0 06 00 00        	movq	0x6a0(%rbp), %rax
1403bc289: 48 85 c0                    	testq	%rax, %rax
1403bc28c: 8b 95 18 07 00 00           	movl	0x718(%rbp), %edx
1403bc292: 74 33                       	je	0x1403bc2c7 <.text+0x3bb2c7>
1403bc294: 4c 8b a5 98 06 00 00        	movq	0x698(%rbp), %r12
1403bc29b: 48 69 c0 b0 01 00 00        	imulq	$0x1b0, %rax, %rax      # imm = 0x1B0
1403bc2a2: 4d 8d 2c 04                 	leaq	(%r12,%rax), %r13
1403bc2a6: 41 bf 01 00 00 00           	movl	$0x1, %r15d
1403bc2ac: 31 c9                       	xorl	%ecx, %ecx
1403bc2ae: 41 3b 94 0c 78 01 00 00     	cmpl	0x178(%r12,%rcx), %edx
1403bc2b6: 74 27                       	je	0x1403bc2df <.text+0x3bb2df>
1403bc2b8: 49 ff c7                    	incq	%r15
1403bc2bb: 48 81 c1 b0 01 00 00        	addq	$0x1b0, %rcx            # imm = 0x1B0
1403bc2c2: 48 39 c8                    	cmpq	%rcx, %rax
1403bc2c5: 75 e7                       	jne	0x1403bc2ae <.text+0x3bb2ae>
1403bc2c7: c6 85 7e 07 00 00 01        	movb	$0x1, 0x77e(%rbp)
1403bc2ce: 48 8d 0d 43 17 61 02        	leaq	0x2611743(%rip), %rcx   # 0x1429cda18
1403bc2d5: e8 36 d0 51 02              	callq	0x1428d9310 <.text+0x28d8310>
1403bc2da: e9 38 06 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc2df: 48 8b 8d d0 08 00 00        	movq	0x8d0(%rbp), %rcx
1403bc2e6: 48 8b 01                    	movq	(%rcx), %rax
1403bc2e9: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1403bc2ed: 48 8d 91 a0 01 00 00        	leaq	0x1a0(%rcx), %rdx
1403bc2f4: 4c 8b 80 f8 00 00 00        	movq	0xf8(%rax), %r8
1403bc2fb: 4c 8b 88 00 01 00 00        	movq	0x100(%rax), %r9
1403bc302: 4f 8d 0c 88                 	leaq	(%r8,%r9,4), %r9
1403bc306: 44 0f b6 90 08 01 00 00     	movzbl	0x108(%rax), %r10d
1403bc30e: 0f 57 c0                    	xorps	%xmm0, %xmm0
1403bc311: 0f 11 85 d8 03 00 00        	movups	%xmm0, 0x3d8(%rbp)
1403bc318: 0f 11 85 c8 03 00 00        	movups	%xmm0, 0x3c8(%rbp)
1403bc31f: 48 c7 85 e8 03 00 00 00 00 00 00    	movq	$0x0, 0x3e8(%rbp)
1403bc32a: 48 89 8d 80 03 00 00        	movq	%rcx, 0x380(%rbp)
1403bc331: 48 81 c1 f8 00 00 00        	addq	$0xf8, %rcx
1403bc338: 48 89 95 88 03 00 00        	movq	%rdx, 0x388(%rbp)
1403bc33f: 48 89 8d 90 03 00 00        	movq	%rcx, 0x390(%rbp)
1403bc346: 48 89 85 98 03 00 00        	movq	%rax, 0x398(%rbp)
1403bc34d: 4c 89 85 a0 03 00 00        	movq	%r8, 0x3a0(%rbp)
1403bc354: 4c 89 8d a8 03 00 00        	movq	%r9, 0x3a8(%rbp)
1403bc35b: 48 c7 85 b0 03 00 00 08 00 00 00    	movq	$0x8, 0x3b0(%rbp)
1403bc366: 48 c7 85 b8 03 00 00 00 00 00 00    	movq	$0x0, 0x3b8(%rbp)
1403bc371: 48 c7 85 c0 03 00 00 08 00 00 00    	movq	$0x8, 0x3c0(%rbp)
1403bc37c: 44 88 95 f0 03 00 00        	movb	%r10b, 0x3f0(%rbp)
1403bc383: c6 85 7e 07 00 00 01        	movb	$0x1, 0x77e(%rbp)
1403bc38a: 4c 8d 05 f7 1c 62 02        	leaq	0x2621cf7(%rip), %r8    # 0x1429de088
1403bc391: 48 8d 8d 30 06 00 00        	leaq	0x630(%rbp), %rcx
1403bc398: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403bc39f: e8 7c 36 f0 ff              	callq	0x1402bfa20 <.text+0x2bea20>
1403bc3a4: 48 8b 85 38 06 00 00        	movq	0x638(%rbp), %rax
1403bc3ab: 48 8b 9d 40 06 00 00        	movq	0x640(%rbp), %rbx
1403bc3b2: 48 89 85 00 07 00 00        	movq	%rax, 0x700(%rbp)
1403bc3b9: 48 89 85 80 03 00 00        	movq	%rax, 0x380(%rbp)
1403bc3c0: 48 89 9d 88 03 00 00        	movq	%rbx, 0x388(%rbp)
1403bc3c7: c6 85 94 07 00 00 01        	movb	$0x1, 0x794(%rbp)
1403bc3ce: 48 8d 0d f3 12 61 02        	leaq	0x26112f3(%rip), %rcx   # 0x1429cd6c8
1403bc3d5: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403bc3dc: e8 7f c8 ce ff              	callq	0x1400a8c60 <.text+0xa7c60>
1403bc3e1: 49 8d 47 ff                 	leaq	-0x1(%r15), %rax
1403bc3e5: 48 89 85 68 07 00 00        	movq	%rax, 0x768(%rbp)
1403bc3ec: 48 8b 85 e0 08 00 00        	movq	0x8e0(%rbp), %rax
1403bc3f3: 48 8b 08                    	movq	(%rax), %rcx
1403bc3f6: 48 89 8d 60 07 00 00        	movq	%rcx, 0x760(%rbp)
1403bc3fd: 48 8b 40 08                 	movq	0x8(%rax), %rax
1403bc401: 48 89 85 58 07 00 00        	movq	%rax, 0x758(%rbp)
1403bc408: 48 8b 85 30 06 00 00        	movq	0x630(%rbp), %rax
1403bc40f: 48 89 85 40 07 00 00        	movq	%rax, 0x740(%rbp)
1403bc416: 45 31 f6                    	xorl	%r14d, %r14d
1403bc419: 48 89 d8                    	movq	%rbx, %rax
1403bc41c: 4d 39 ec                    	cmpq	%r13, %r12
1403bc41f: 0f 84 be 01 00 00           	je	0x1403bc5e3 <.text+0x3bb5e3>
1403bc425: 49 81 c4 b0 01 00 00        	addq	$0x1b0, %r12            # imm = 0x1B0
1403bc42c: 49 ff c6                    	incq	%r14
1403bc42f: 4d 39 f7                    	cmpq	%r14, %r15
1403bc432: 74 e8                       	je	0x1403bc41c <.text+0x3bb41c>
1403bc434: bb 00 00 00 00              	movl	$0x0, %ebx
1403bc439: 48 85 c0                    	testq	%rax, %rax
1403bc43c: 74 db                       	je	0x1403bc419 <.text+0x3bb419>
1403bc43e: 48 8d 58 ff                 	leaq	-0x1(%rax), %rbx
1403bc442: 48 89 9d 40 06 00 00        	movq	%rbx, 0x640(%rbp)
1403bc449: 48 8b 8d 00 07 00 00        	movq	0x700(%rbp), %rcx
1403bc450: 48 8b 4c c1 f8              	movq	-0x8(%rcx,%rax,8), %rcx
1403bc455: 48 89 8d 28 07 00 00        	movq	%rcx, 0x728(%rbp)
1403bc45c: 48 c1 e9 20                 	shrq	$0x20, %rcx
1403bc460: 48 89 8d 38 07 00 00        	movq	%rcx, 0x738(%rbp)
1403bc467: 74 b0                       	je	0x1403bc419 <.text+0x3bb419>
1403bc469: 8b 85 28 07 00 00           	movl	0x728(%rbp), %eax
1403bc46f: 48 8b 8d 58 07 00 00        	movq	0x758(%rbp), %rcx
1403bc476: 48 3b 41 10                 	cmpq	0x10(%rcx), %rax
1403bc47a: 0f 83 46 01 00 00           	jae	0x1403bc5c6 <.text+0x3bb5c6>
1403bc480: 48 8b 8d 58 07 00 00        	movq	0x758(%rbp), %rcx
1403bc487: 48 8b 49 08                 	movq	0x8(%rcx), %rcx
1403bc48b: 48 8d 04 80                 	leaq	(%rax,%rax,4), %rax
1403bc48f: 48 8b 95 38 07 00 00        	movq	0x738(%rbp), %rdx
1403bc496: 39 14 81                    	cmpl	%edx, (%rcx,%rax,4)
1403bc499: 0f 85 27 01 00 00           	jne	0x1403bc5c6 <.text+0x3bb5c6>
1403bc49f: 48 8d 04 81                 	leaq	(%rcx,%rax,4), %rax
1403bc4a3: 8b 48 04                    	movl	0x4(%rax), %ecx
1403bc4a6: ba ff ff ff ff              	movl	$0xffffffff, %edx       # imm = 0xFFFFFFFF
1403bc4ab: 48 39 d1                    	cmpq	%rdx, %rcx
1403bc4ae: 0f 84 12 01 00 00           	je	0x1403bc5c6 <.text+0x3bb5c6>
1403bc4b4: 48 8b 95 60 07 00 00        	movq	0x760(%rbp), %rdx
1403bc4bb: 48 39 4a 38                 	cmpq	%rcx, 0x38(%rdx)
1403bc4bf: 0f 86 01 01 00 00           	jbe	0x1403bc5c6 <.text+0x3bb5c6>
1403bc4c5: 48 8b 95 60 07 00 00        	movq	0x760(%rbp), %rdx
1403bc4cc: 48 8b 52 28                 	movq	0x28(%rdx), %rdx
1403bc4d0: 41 89 c8                    	movl	%ecx, %r8d
1403bc4d3: 41 c1 e8 06                 	shrl	$0x6, %r8d
1403bc4d7: 4a 8b 14 c2                 	movq	(%rdx,%r8,8), %rdx
1403bc4db: 48 0f a3 ca                 	btq	%rcx, %rdx
1403bc4df: 0f 83 e1 00 00 00           	jae	0x1403bc5c6 <.text+0x3bb5c6>
1403bc4e5: 8b 48 0c                    	movl	0xc(%rax), %ecx
1403bc4e8: 8b 40 10                    	movl	0x10(%rax), %eax
1403bc4eb: 48 8b 95 58 07 00 00        	movq	0x758(%rbp), %rdx
1403bc4f2: 48 8b 92 a8 01 00 00        	movq	0x1a8(%rdx), %rdx
1403bc4f9: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
1403bc4fd: 4c 8b 8d 60 07 00 00        	movq	0x760(%rbp), %r9
1403bc504: 4d 8b 81 10 01 00 00        	movq	0x110(%r9), %r8
1403bc50b: 4d 8b 89 18 01 00 00        	movq	0x118(%r9), %r9
1403bc512: 4c 8b 54 ca 18              	movq	0x18(%rdx,%rcx,8), %r10
1403bc517: 48 8b 4c ca 38              	movq	0x38(%rdx,%rcx,8), %rcx
1403bc51c: 4a 8b 14 c1                 	movq	(%rcx,%r8,8), %rdx
1403bc520: 48 f7 d2                    	notq	%rdx
1403bc523: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
1403bc527: 48 c1 e2 04                 	shlq	$0x4, %rdx
1403bc52b: 49 8b 54 12 10              	movq	0x10(%r10,%rdx), %rdx
1403bc530: 4a 8b 0c c9                 	movq	(%rcx,%r9,8), %rcx
1403bc534: 48 f7 d1                    	notq	%rcx
1403bc537: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
1403bc53b: 48 c1 e1 04                 	shlq	$0x4, %rcx
1403bc53f: 49 8b 4c 0a 10              	movq	0x10(%r10,%rcx), %rcx
1403bc544: 48 c1 e0 03                 	shlq	$0x3, %rax
1403bc548: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
1403bc54c: 48 8b 54 02 10              	movq	0x10(%rdx,%rax), %rdx
1403bc551: 48 8b 44 01 10              	movq	0x10(%rcx,%rax), %rax
1403bc556: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
1403bc55d: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
1403bc564: 31 ff                       	xorl	%edi, %edi
1403bc566: 48 83 ff 10                 	cmpq	$0x10, %rdi
1403bc56a: 74 5a                       	je	0x1403bc5c6 <.text+0x3bb5c6>
1403bc56c: 48 89 f8                    	movq	%rdi, %rax
1403bc56f: 48 83 c7 08                 	addq	$0x8, %rdi
1403bc573: 83 bc 05 b4 00 00 00 00     	cmpl	$0x0, 0xb4(%rbp,%rax)
1403bc57b: 74 e9                       	je	0x1403bc566 <.text+0x3bb566>
1403bc57d: 48 8b 84 05 b0 00 00 00     	movq	0xb0(%rbp,%rax), %rax
1403bc585: 48 8b 8d f0 08 00 00        	movq	0x8f0(%rbp), %rcx
1403bc58c: 0f 10 01                    	movups	(%rcx), %xmm0
1403bc58f: f3 0f 6f 49 10              	movdqu	0x10(%rcx), %xmm1
1403bc594: 48 89 85 a0 03 00 00        	movq	%rax, 0x3a0(%rbp)
1403bc59b: 0f 11 85 80 03 00 00        	movups	%xmm0, 0x380(%rbp)
1403bc5a2: f3 0f 7f 8d 90 03 00 00     	movdqu	%xmm1, 0x390(%rbp)
1403bc5aa: c6 85 94 07 00 00 01        	movb	$0x1, 0x794(%rbp)
1403bc5b1: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bc5b8: 48 8d 15 71 14 61 02        	leaq	0x2611471(%rip), %rdx   # 0x1429cda30
1403bc5bf: e8 dc 16 d9 ff              	callq	0x14014dca0 <.text+0x14cca0>
1403bc5c4: eb a0                       	jmp	0x1403bc566 <.text+0x3bb566>
1403bc5c6: 48 8b 85 28 07 00 00        	movq	0x728(%rbp), %rax
1403bc5cd: 41 89 44 24 c0              	movl	%eax, -0x40(%r12)
1403bc5d2: 48 8b 85 38 07 00 00        	movq	0x738(%rbp), %rax
1403bc5d9: 41 89 44 24 c4              	movl	%eax, -0x3c(%r12)
1403bc5de: e9 36 fe ff ff              	jmp	0x1403bc419 <.text+0x3bb419>
1403bc5e3: 48 8b 95 50 03 00 00        	movq	0x350(%rbp), %rdx
1403bc5ea: 8b 42 1c                    	movl	0x1c(%rdx), %eax
1403bc5ed: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1403bc5f1: 89 01                       	movl	%eax, (%rcx)
1403bc5f3: 48 8b 02                    	movq	(%rdx), %rax
1403bc5f6: c6 80 d8 00 00 00 01        	movb	$0x1, 0xd8(%rax)
1403bc5fd: 48 8b 8d 20 06 00 00        	movq	0x620(%rbp), %rcx
1403bc604: 8b 41 1c                    	movl	0x1c(%rcx), %eax
1403bc607: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
1403bc60b: 89 01                       	movl	%eax, (%rcx)
1403bc60d: f2 0f 10 05 1b 0d 61 02     	movsd	0x2610d1b(%rip), %xmm0  # 0x1429cd330
1403bc615: f2 0f 11 85 b0 00 00 00     	movsd	%xmm0, 0xb0(%rbp)
1403bc61d: c7 85 b8 00 00 00 00 00 80 bf       	movl	$0xbf800000, 0xb8(%rbp) # imm = 0xBF800000
1403bc627: 48 81 c6 f0 02 00 00        	addq	$0x2f0, %rsi            # imm = 0x2F0
1403bc62e: c7 85 88 03 00 00 00 00 00 00       	movl	$0x0, 0x388(%rbp)
1403bc638: 48 c7 85 80 03 00 00 00 00 00 00    	movq	$0x0, 0x380(%rbp)
1403bc643: c6 85 94 07 00 00 01        	movb	$0x1, 0x794(%rbp)
1403bc64a: 48 8d 95 80 03 00 00        	leaq	0x380(%rbp), %rdx
1403bc651: 4c 8d 85 b0 00 00 00        	leaq	0xb0(%rbp), %r8
1403bc658: 48 89 f1                    	movq	%rsi, %rcx
1403bc65b: e8 d0 7a 5d 00              	callq	0x140994130 <.text+0x993130>
1403bc660: 48 8b 85 88 07 00 00        	movq	0x788(%rbp), %rax
1403bc667: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1403bc66e: 48 8b 85 70 07 00 00        	movq	0x770(%rbp), %rax
1403bc675: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
1403bc67c: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
1403bc680: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
1403bc687: 48 8b 85 a0 06 00 00        	movq	0x6a0(%rbp), %rax
1403bc68e: 48 89 85 40 05 00 00        	movq	%rax, 0x540(%rbp)
1403bc695: 0f 10 85 90 06 00 00        	movups	0x690(%rbp), %xmm0
1403bc69c: 0f 29 85 30 05 00 00        	movaps	%xmm0, 0x530(%rbp)
1403bc6a3: 48 8b 85 c0 02 00 00        	movq	0x2c0(%rbp), %rax
1403bc6aa: 48 8b 00                    	movq	(%rax), %rax
1403bc6ad: c6 85 94 07 00 00 00        	movb	$0x0, 0x794(%rbp)
1403bc6b4: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403bc6b9: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bc6c0: 48 8d 95 b0 00 00 00        	leaq	0xb0(%rbp), %rdx
1403bc6c7: 4c 8d 85 30 05 00 00        	leaq	0x530(%rbp), %r8
1403bc6ce: 4c 8b 8d 68 07 00 00        	movq	0x768(%rbp), %r9
1403bc6d5: e8 66 a4 ff ff              	callq	0x1403b6b40 <.text+0x3b5b40>
1403bc6da: 0f 10 85 80 03 00 00        	movups	0x380(%rbp), %xmm0
1403bc6e1: 0f 10 8d 90 03 00 00        	movups	0x390(%rbp), %xmm1
1403bc6e8: 0f 10 95 a0 03 00 00        	movups	0x3a0(%rbp), %xmm2
1403bc6ef: 0f 10 9d b0 03 00 00        	movups	0x3b0(%rbp), %xmm3
1403bc6f6: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
1403bc6fd: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1403bc704: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1403bc70b: 0f 29 9d e0 00 00 00        	movaps	%xmm3, 0xe0(%rbp)
1403bc712: 0f 10 85 c0 03 00 00        	movups	0x3c0(%rbp), %xmm0
1403bc719: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1403bc720: 31 c0                       	xorl	%eax, %eax
1403bc722: 48 8b 8d 80 07 00 00        	movq	0x780(%rbp), %rcx
1403bc729: 48 3b 01                    	cmpq	(%rcx), %rax
1403bc72c: 70 0c                       	jo	0x1403bc73a <.text+0x3bb73a>
1403bc72e: 48 8b 8d 80 07 00 00        	movq	0x780(%rbp), %rcx
1403bc735: e8 76 d8 23 00              	callq	0x1405f9fb0 <.text+0x5f8fb0>
1403bc73a: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1403bc741: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403bc748: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1403bc74c: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1403bc753: 0f 28 8d c0 00 00 00        	movaps	0xc0(%rbp), %xmm1
1403bc75a: 0f 28 95 d0 00 00 00        	movaps	0xd0(%rbp), %xmm2
1403bc761: 0f 28 9d e0 00 00 00        	movaps	0xe0(%rbp), %xmm3
1403bc768: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
1403bc76c: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
1403bc770: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
1403bc774: 0f 11 00                    	movups	%xmm0, (%rax)
1403bc777: 48 83 bd 40 07 00 00 00     	cmpq	$0x0, 0x740(%rbp)
1403bc77f: 0f 84 84 e6 ff ff           	je	0x1403bae09 <.text+0x3b9e09>
1403bc785: 48 8b 95 40 07 00 00        	movq	0x740(%rbp), %rdx
1403bc78c: 48 c1 e2 03                 	shlq	$0x3, %rdx
1403bc790: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403bc796: 48 8b 8d 00 07 00 00        	movq	0x700(%rbp), %rcx
1403bc79d: e8 7e 74 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bc7a2: e9 62 e6 ff ff              	jmp	0x1403bae09 <.text+0x3b9e09>
1403bc7a7: 4c 8d 05 0a 12 61 02        	leaq	0x261120a(%rip), %r8    # 0x1429cd9b8
1403bc7ae: e8 65 cd 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc7b3: e9 5f 01 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc7b8: 4c 8d 05 11 12 61 02        	leaq	0x2611211(%rip), %r8    # 0x1429cd9d0
1403bc7bf: e8 54 cd 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc7c4: e9 4e 01 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc7c9: 48 8d 0d 18 12 61 02        	leaq	0x2611218(%rip), %rcx   # 0x1429cd9e8
1403bc7d0: e8 4b d2 51 02              	callq	0x1428d9a20 <.text+0x28d8a20>
1403bc7d5: e9 3d 01 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc7da: 4c 8d 05 8f 11 61 02        	leaq	0x261118f(%rip), %r8    # 0x1429cd970
1403bc7e1: 4c 89 e1                    	movq	%r12, %rcx
1403bc7e4: e8 2f cd 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc7e9: e9 29 01 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc7ee: 48 8d 0d ab 10 61 02        	leaq	0x26110ab(%rip), %rcx   # 0x1429cd8a0
1403bc7f5: e8 16 cb 51 02              	callq	0x1428d9310 <.text+0x28d8310>
1403bc7fa: e9 18 01 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc7ff: c6 85 96 07 00 00 01        	movb	$0x1, 0x796(%rbp)
1403bc806: 48 8d 0d c3 10 61 02        	leaq	0x26110c3(%rip), %rcx   # 0x1429cd8d0
1403bc80d: e8 fe ca 51 02              	callq	0x1428d9310 <.text+0x28d8310>
1403bc812: e9 00 01 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc817: c6 85 96 07 00 00 01        	movb	$0x1, 0x796(%rbp)
1403bc81e: 48 8d 0d c3 10 61 02        	leaq	0x26110c3(%rip), %rcx   # 0x1429cd8e8
1403bc825: e8 e6 ca 51 02              	callq	0x1428d9310 <.text+0x28d8310>
1403bc82a: e9 e8 00 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc82f: 4c 8d 05 4a 1b 59 02        	leaq	0x2591b4a(%rip), %r8    # 0x14294e380
1403bc836: 48 89 d1                    	movq	%rdx, %rcx
1403bc839: 4c 89 fa                    	movq	%r15, %rdx
1403bc83c: e8 d7 cc 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc841: e9 d1 00 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc846: c6 85 96 07 00 00 01        	movb	$0x1, 0x796(%rbp)
1403bc84d: 4c 8d 05 2c 1b 59 02        	leaq	0x2591b2c(%rip), %r8    # 0x14294e380
1403bc854: 48 89 d1                    	movq	%rdx, %rcx
1403bc857: 4c 89 fa                    	movq	%r15, %rdx
1403bc85a: e8 b9 cc 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc85f: e9 b3 00 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc864: c6 85 96 07 00 00 01        	movb	$0x1, 0x796(%rbp)
1403bc86b: 4c 8d 05 0e 1b 59 02        	leaq	0x2591b0e(%rip), %r8    # 0x14294e380
1403bc872: 48 89 d1                    	movq	%rdx, %rcx
1403bc875: 4c 89 fa                    	movq	%r15, %rdx
1403bc878: e8 9b cc 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc87d: e9 95 00 00 00              	jmp	0x1403bc917 <.text+0x3bb917>
1403bc882: 48 c1 e8 20                 	shrq	$0x20, %rax
1403bc886: 49 89 c2                    	movq	%rax, %r10
1403bc889: 49 c1 e2 20                 	shlq	$0x20, %r10
1403bc88d: 4d 09 c2                    	orq	%r8, %r10
1403bc890: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403bc896: 4c 89 d0                    	movq	%r10, %rax
1403bc899: 44 89 85 80 03 00 00        	movl	%r8d, 0x380(%rbp)
1403bc8a0: 89 95 84 03 00 00           	movl	%edx, 0x384(%rbp)
1403bc8a6: 48 89 85 88 03 00 00        	movq	%rax, 0x388(%rbp)
1403bc8ad: 48 8d 05 4c 11 61 02        	leaq	0x261114c(%rip), %rax   # 0x1429cda00
1403bc8b4: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1403bc8b9: 48 8d 0d 18 0c 61 02        	leaq	0x2610c18(%rip), %rcx   # 0x1429cd4d8
1403bc8c0: 4c 8d 0d f1 0b 61 02        	leaq	0x2610bf1(%rip), %r9    # 0x1429cd4b8
1403bc8c7: 4c 8d 85 80 03 00 00        	leaq	0x380(%rbp), %r8
1403bc8ce: ba 2b 00 00 00              	movl	$0x2b, %edx
1403bc8d3: e8 88 ce 51 02              	callq	0x1428d9760 <.text+0x28d8760>
1403bc8d8: eb 3d                       	jmp	0x1403bc917 <.text+0x3bb917>
1403bc8da: 31 c9                       	xorl	%ecx, %ecx
1403bc8dc: c6 85 97 07 00 00 01        	movb	$0x1, 0x797(%rbp)
1403bc8e3: 4c 8d 05 4e 1b 5f 02        	leaq	0x25f1b4e(%rip), %r8    # 0x1429ae438
1403bc8ea: 4c 89 f2                    	movq	%r14, %rdx
1403bc8ed: e8 f1 c6 51 02              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1403bc8f2: eb 23                       	jmp	0x1403bc917 <.text+0x3bb917>
1403bc8f4: 4c 8d 05 85 1a 59 02        	leaq	0x2591a85(%rip), %r8    # 0x14294e380
1403bc8fb: 48 89 d1                    	movq	%rdx, %rcx
1403bc8fe: 48 89 da                    	movq	%rbx, %rdx
1403bc901: e8 12 cc 51 02              	callq	0x1428d9518 <.text+0x28d8518>
1403bc906: eb 0f                       	jmp	0x1403bc917 <.text+0x3bb917>
1403bc908: b9 04 00 00 00              	movl	$0x4, %ecx
1403bc90d: ba 08 00 00 00              	movl	$0x8, %edx
1403bc912: e8 e9 c6 51 02              	callq	0x1428d9000 <.text+0x28d8000>
1403bc917: 0f 0b                       	ud2
1403bc919: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403bc920: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bc925: 55                          	pushq	%rbp
1403bc926: 41 57                       	pushq	%r15
1403bc928: 41 56                       	pushq	%r14
1403bc92a: 41 55                       	pushq	%r13
1403bc92c: 41 54                       	pushq	%r12
1403bc92e: 56                          	pushq	%rsi
1403bc92f: 57                          	pushq	%rdi
1403bc930: 53                          	pushq	%rbx
1403bc931: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bc938: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bc93f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bc945: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bc94b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bc951: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bc957: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bc960: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bc969: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bc972: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bc97b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bc983: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bc98b: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
1403bc992: 48 8b 85 80 07 00 00        	movq	0x780(%rbp), %rax
1403bc999: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
1403bc99d: 0f 28 85 b0 00 00 00        	movaps	0xb0(%rbp), %xmm0
1403bc9a4: 66 0f 6f 8d c0 00 00 00     	movdqa	0xc0(%rbp), %xmm1
1403bc9ac: 66 0f 6f 95 d0 00 00 00     	movdqa	0xd0(%rbp), %xmm2
1403bc9b4: 66 0f 6f 9d e0 00 00 00     	movdqa	0xe0(%rbp), %xmm3
1403bc9bc: f3 0f 7f 58 30              	movdqu	%xmm3, 0x30(%rax)
1403bc9c1: f3 0f 7f 50 20              	movdqu	%xmm2, 0x20(%rax)
1403bc9c6: f3 0f 7f 48 10              	movdqu	%xmm1, 0x10(%rax)
1403bc9cb: 0f 11 00                    	movups	%xmm0, (%rax)
1403bc9ce: c6 85 94 07 00 00 00        	movb	$0x0, 0x794(%rbp)
1403bc9d5: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bc9dd: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bc9e5: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bc9ee: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bc9f7: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bca00: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bca09: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bca0f: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bca15: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bca1b: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bca21: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bca28: 5b                          	popq	%rbx
1403bca29: 5f                          	popq	%rdi
1403bca2a: 5e                          	popq	%rsi
1403bca2b: 41 5c                       	popq	%r12
1403bca2d: 41 5d                       	popq	%r13
1403bca2f: 41 5e                       	popq	%r14
1403bca31: 41 5f                       	popq	%r15
1403bca33: 5d                          	popq	%rbp
1403bca34: c3                          	retq
1403bca35: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403bca40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bca45: 55                          	pushq	%rbp
1403bca46: 41 57                       	pushq	%r15
1403bca48: 41 56                       	pushq	%r14
1403bca4a: 41 55                       	pushq	%r13
1403bca4c: 41 54                       	pushq	%r12
1403bca4e: 56                          	pushq	%rsi
1403bca4f: 57                          	pushq	%rdi
1403bca50: 53                          	pushq	%rbx
1403bca51: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bca58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bca5f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bca65: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bca6b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bca71: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bca77: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bca80: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bca89: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bca92: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bca9b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bcaa3: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bcaab: 0f b6 9d 94 07 00 00        	movzbl	0x794(%rbp), %ebx
1403bcab2: 48 8b 95 30 06 00 00        	movq	0x630(%rbp), %rdx
1403bcab9: 48 85 d2                    	testq	%rdx, %rdx
1403bcabc: 74 16                       	je	0x1403bcad4 <.text+0x3bbad4>
1403bcabe: 48 c1 e2 03                 	shlq	$0x3, %rdx
1403bcac2: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403bcac8: 48 8b 8d 00 07 00 00        	movq	0x700(%rbp), %rcx
1403bcacf: e8 4c 71 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bcad4: 88 9d 7e 07 00 00           	movb	%bl, 0x77e(%rbp)
1403bcada: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bcae2: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bcaea: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bcaf3: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bcafc: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bcb05: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bcb0e: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bcb14: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bcb1a: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bcb20: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bcb26: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bcb2d: 5b                          	popq	%rbx
1403bcb2e: 5f                          	popq	%rdi
1403bcb2f: 5e                          	popq	%rsi
1403bcb30: 41 5c                       	popq	%r12
1403bcb32: 41 5d                       	popq	%r13
1403bcb34: 41 5e                       	popq	%r14
1403bcb36: 41 5f                       	popq	%r15
1403bcb38: 5d                          	popq	%rbp
1403bcb39: c3                          	retq
1403bcb3a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1403bcb40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bcb45: 55                          	pushq	%rbp
1403bcb46: 41 57                       	pushq	%r15
1403bcb48: 41 56                       	pushq	%r14
1403bcb4a: 41 55                       	pushq	%r13
1403bcb4c: 41 54                       	pushq	%r12
1403bcb4e: 56                          	pushq	%rsi
1403bcb4f: 57                          	pushq	%rdi
1403bcb50: 53                          	pushq	%rbx
1403bcb51: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bcb58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bcb5f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bcb65: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bcb6b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bcb71: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bcb77: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bcb80: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bcb89: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bcb92: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bcb9b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bcba3: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bcbab: 48 8d 8d b0 05 00 00        	leaq	0x5b0(%rbp), %rcx
1403bcbb2: e8 69 9a 22 00              	callq	0x1405e6620 <.text+0x5e5620>
1403bcbb7: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bcbbf: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bcbc7: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bcbd0: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bcbd9: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bcbe2: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bcbeb: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bcbf1: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bcbf7: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bcbfd: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bcc03: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bcc0a: 5b                          	popq	%rbx
1403bcc0b: 5f                          	popq	%rdi
1403bcc0c: 5e                          	popq	%rsi
1403bcc0d: 41 5c                       	popq	%r12
1403bcc0f: 41 5d                       	popq	%r13
1403bcc11: 41 5e                       	popq	%r14
1403bcc13: 41 5f                       	popq	%r15
1403bcc15: 5d                          	popq	%rbp
1403bcc16: c3                          	retq
1403bcc17: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1403bcc20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bcc25: 55                          	pushq	%rbp
1403bcc26: 41 57                       	pushq	%r15
1403bcc28: 41 56                       	pushq	%r14
1403bcc2a: 41 55                       	pushq	%r13
1403bcc2c: 41 54                       	pushq	%r12
1403bcc2e: 56                          	pushq	%rsi
1403bcc2f: 57                          	pushq	%rdi
1403bcc30: 53                          	pushq	%rbx
1403bcc31: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bcc38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bcc3f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bcc45: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bcc4b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bcc51: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bcc57: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bcc60: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bcc69: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bcc72: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bcc7b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bcc83: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bcc8b: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403bcc92: e8 89 ed d8 ff              	callq	0x14014ba20 <.text+0x14aa20>
1403bcc97: c6 85 95 07 00 00 00        	movb	$0x0, 0x795(%rbp)
1403bcc9e: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bcca6: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bccae: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bccb7: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bccc0: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bccc9: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bccd2: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bccd8: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bccde: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bcce4: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bccea: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bccf1: 5b                          	popq	%rbx
1403bccf2: 5f                          	popq	%rdi
1403bccf3: 5e                          	popq	%rsi
1403bccf4: 41 5c                       	popq	%r12
1403bccf6: 41 5d                       	popq	%r13
1403bccf8: 41 5e                       	popq	%r14
1403bccfa: 41 5f                       	popq	%r15
1403bccfc: 5d                          	popq	%rbp
1403bccfd: c3                          	retq
1403bccfe: 66 90                       	nop
1403bcd00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bcd05: 55                          	pushq	%rbp
1403bcd06: 41 57                       	pushq	%r15
1403bcd08: 41 56                       	pushq	%r14
1403bcd0a: 41 55                       	pushq	%r13
1403bcd0c: 41 54                       	pushq	%r12
1403bcd0e: 56                          	pushq	%rsi
1403bcd0f: 57                          	pushq	%rdi
1403bcd10: 53                          	pushq	%rbx
1403bcd11: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bcd18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bcd1f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bcd25: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bcd2b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bcd31: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bcd37: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bcd40: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bcd49: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bcd52: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bcd5b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bcd63: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bcd6b: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1403bcd72: e8 a9 ec d8 ff              	callq	0x14014ba20 <.text+0x14aa20>
1403bcd77: c6 85 97 07 00 00 00        	movb	$0x0, 0x797(%rbp)
1403bcd7e: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bcd86: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bcd8e: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bcd97: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bcda0: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bcda9: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bcdb2: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bcdb8: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bcdbe: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bcdc4: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bcdca: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bcdd1: 5b                          	popq	%rbx
1403bcdd2: 5f                          	popq	%rdi
1403bcdd3: 5e                          	popq	%rsi
1403bcdd4: 41 5c                       	popq	%r12
1403bcdd6: 41 5d                       	popq	%r13
1403bcdd8: 41 5e                       	popq	%r14
1403bcdda: 41 5f                       	popq	%r15
1403bcddc: 5d                          	popq	%rbp
1403bcddd: c3                          	retq
1403bcdde: 66 90                       	nop
1403bcde0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bcde5: 55                          	pushq	%rbp
1403bcde6: 41 57                       	pushq	%r15
1403bcde8: 41 56                       	pushq	%r14
1403bcdea: 41 55                       	pushq	%r13
1403bcdec: 41 54                       	pushq	%r12
1403bcdee: 56                          	pushq	%rsi
1403bcdef: 57                          	pushq	%rdi
1403bcdf0: 53                          	pushq	%rbx
1403bcdf1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bcdf8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bcdff: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bce05: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bce0b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bce11: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bce17: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bce20: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bce29: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bce32: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bce3b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bce43: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bce4b: 80 bd 95 07 00 00 00        	cmpb	$0x0, 0x795(%rbp)
1403bce52: 74 73                       	je	0x1403bcec7 <.text+0x3bbec7>
1403bce54: 48 8d 8d 30 06 00 00        	leaq	0x630(%rbp), %rcx
1403bce5b: e8 c0 eb d8 ff              	callq	0x14014ba20 <.text+0x14aa20>
1403bce60: c6 85 97 07 00 00 00        	movb	$0x0, 0x797(%rbp)
1403bce67: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bce6f: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bce77: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bce80: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bce89: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bce92: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bce9b: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bcea1: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bcea7: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bcead: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bceb3: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bceba: 5b                          	popq	%rbx
1403bcebb: 5f                          	popq	%rdi
1403bcebc: 5e                          	popq	%rsi
1403bcebd: 41 5c                       	popq	%r12
1403bcebf: 41 5d                       	popq	%r13
1403bcec1: 41 5e                       	popq	%r14
1403bcec3: 41 5f                       	popq	%r15
1403bcec5: 5d                          	popq	%rbp
1403bcec6: c3                          	retq
1403bcec7: c6 85 97 07 00 00 00        	movb	$0x0, 0x797(%rbp)
1403bcece: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bced6: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bcede: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bcee7: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bcef0: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bcef9: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bcf02: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bcf08: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bcf0e: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bcf14: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bcf1a: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bcf21: 5b                          	popq	%rbx
1403bcf22: 5f                          	popq	%rdi
1403bcf23: 5e                          	popq	%rsi
1403bcf24: 41 5c                       	popq	%r12
1403bcf26: 41 5d                       	popq	%r13
1403bcf28: 41 5e                       	popq	%r14
1403bcf2a: 41 5f                       	popq	%r15
1403bcf2c: 5d                          	popq	%rbp
1403bcf2d: c3                          	retq
1403bcf2e: 66 90                       	nop
1403bcf30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bcf35: 55                          	pushq	%rbp
1403bcf36: 41 57                       	pushq	%r15
1403bcf38: 41 56                       	pushq	%r14
1403bcf3a: 41 55                       	pushq	%r13
1403bcf3c: 41 54                       	pushq	%r12
1403bcf3e: 56                          	pushq	%rsi
1403bcf3f: 57                          	pushq	%rdi
1403bcf40: 53                          	pushq	%rbx
1403bcf41: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bcf48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bcf4f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bcf55: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bcf5b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bcf61: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bcf67: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bcf70: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bcf79: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bcf82: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bcf8b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bcf93: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bcf9b: 48 8d 8d 80 03 00 00        	leaq	0x380(%rbp), %rcx
1403bcfa2: e8 a9 cf d8 ff              	callq	0x140149f50 <.text+0x148f50>
1403bcfa7: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bcfaf: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bcfb7: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bcfc0: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bcfc9: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bcfd2: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bcfdb: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bcfe1: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bcfe7: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bcfed: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bcff3: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bcffa: 5b                          	popq	%rbx
1403bcffb: 5f                          	popq	%rdi
1403bcffc: 5e                          	popq	%rsi
1403bcffd: 41 5c                       	popq	%r12
1403bcfff: 41 5d                       	popq	%r13
1403bd001: 41 5e                       	popq	%r14
1403bd003: 41 5f                       	popq	%r15
1403bd005: 5d                          	popq	%rbp
1403bd006: c3                          	retq
1403bd007: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1403bd010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd015: 55                          	pushq	%rbp
1403bd016: 41 57                       	pushq	%r15
1403bd018: 41 56                       	pushq	%r14
1403bd01a: 41 55                       	pushq	%r13
1403bd01c: 41 54                       	pushq	%r12
1403bd01e: 56                          	pushq	%rsi
1403bd01f: 57                          	pushq	%rdi
1403bd020: 53                          	pushq	%rbx
1403bd021: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd028: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd02f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd035: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd03b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd041: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd047: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd050: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd059: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd062: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd06b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd073: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd07b: 48 8b 85 88 07 00 00        	movq	0x788(%rbp), %rax
1403bd082: 48 ff 00                    	incq	(%rax)
1403bd085: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd08d: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd095: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd09e: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd0a7: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd0b0: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd0b9: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd0bf: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd0c5: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd0cb: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd0d1: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd0d8: 5b                          	popq	%rbx
1403bd0d9: 5f                          	popq	%rdi
1403bd0da: 5e                          	popq	%rsi
1403bd0db: 41 5c                       	popq	%r12
1403bd0dd: 41 5d                       	popq	%r13
1403bd0df: 41 5e                       	popq	%r14
1403bd0e1: 41 5f                       	popq	%r15
1403bd0e3: 5d                          	popq	%rbp
1403bd0e4: c3                          	retq
1403bd0e5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403bd0f0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd0f5: 55                          	pushq	%rbp
1403bd0f6: 41 57                       	pushq	%r15
1403bd0f8: 41 56                       	pushq	%r14
1403bd0fa: 41 55                       	pushq	%r13
1403bd0fc: 41 54                       	pushq	%r12
1403bd0fe: 56                          	pushq	%rsi
1403bd0ff: 57                          	pushq	%rdi
1403bd100: 53                          	pushq	%rbx
1403bd101: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd108: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd10f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd115: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd11b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd121: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd127: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd130: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd139: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd142: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd14b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd153: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd15b: 0f b6 9d 7e 07 00 00        	movzbl	0x77e(%rbp), %ebx
1403bd162: f6 c3 01                    	testb	$0x1, %bl
1403bd165: 74 72                       	je	0x1403bd1d9 <.text+0x3bc1d9>
1403bd167: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403bd16e: e8 8d 6f 24 00              	callq	0x140604100 <.text+0x603100>
1403bd173: 88 9d 7f 07 00 00           	movb	%bl, 0x77f(%rbp)
1403bd179: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd181: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd189: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd192: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd19b: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd1a4: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd1ad: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd1b3: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd1b9: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd1bf: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd1c5: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd1cc: 5b                          	popq	%rbx
1403bd1cd: 5f                          	popq	%rdi
1403bd1ce: 5e                          	popq	%rsi
1403bd1cf: 41 5c                       	popq	%r12
1403bd1d1: 41 5d                       	popq	%r13
1403bd1d3: 41 5e                       	popq	%r14
1403bd1d5: 41 5f                       	popq	%r15
1403bd1d7: 5d                          	popq	%rbp
1403bd1d8: c3                          	retq
1403bd1d9: 88 9d 7f 07 00 00           	movb	%bl, 0x77f(%rbp)
1403bd1df: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd1e7: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd1ef: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd1f8: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd201: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd20a: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd213: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd219: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd21f: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd225: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd22b: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd232: 5b                          	popq	%rbx
1403bd233: 5f                          	popq	%rdi
1403bd234: 5e                          	popq	%rsi
1403bd235: 41 5c                       	popq	%r12
1403bd237: 41 5d                       	popq	%r13
1403bd239: 41 5e                       	popq	%r14
1403bd23b: 41 5f                       	popq	%r15
1403bd23d: 5d                          	popq	%rbp
1403bd23e: c3                          	retq
1403bd23f: 90                          	nop
1403bd240: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd245: 55                          	pushq	%rbp
1403bd246: 41 57                       	pushq	%r15
1403bd248: 41 56                       	pushq	%r14
1403bd24a: 41 55                       	pushq	%r13
1403bd24c: 41 54                       	pushq	%r12
1403bd24e: 56                          	pushq	%rsi
1403bd24f: 57                          	pushq	%rdi
1403bd250: 53                          	pushq	%rbx
1403bd251: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd258: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd25f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd265: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd26b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd271: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd277: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd280: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd289: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd292: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd29b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd2a3: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd2ab: f6 85 7f 07 00 00 01        	testb	$0x1, 0x77f(%rbp)
1403bd2b2: 0f 84 91 00 00 00           	je	0x1403bd349 <.text+0x3bc349>
1403bd2b8: 48 8b 85 88 07 00 00        	movq	0x788(%rbp), %rax
1403bd2bf: 48 8d 04 45 00 00 00 00     	leaq	(,%rax,2), %rax
1403bd2c7: 48 85 c0                    	testq	%rax, %rax
1403bd2ca: 74 1d                       	je	0x1403bd2e9 <.text+0x3bc2e9>
1403bd2cc: 48 8b 95 88 07 00 00        	movq	0x788(%rbp), %rdx
1403bd2d3: 48 c1 e2 06                 	shlq	$0x6, %rdx
1403bd2d7: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bd2dd: 48 8b 8d 70 07 00 00        	movq	0x770(%rbp), %rcx
1403bd2e4: e8 37 69 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bd2e9: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd2f1: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd2f9: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd302: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd30b: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd314: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd31d: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd323: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd329: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd32f: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd335: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd33c: 5b                          	popq	%rbx
1403bd33d: 5f                          	popq	%rdi
1403bd33e: 5e                          	popq	%rsi
1403bd33f: 41 5c                       	popq	%r12
1403bd341: 41 5d                       	popq	%r13
1403bd343: 41 5e                       	popq	%r14
1403bd345: 41 5f                       	popq	%r15
1403bd347: 5d                          	popq	%rbp
1403bd348: c3                          	retq
1403bd349: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd351: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd359: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd362: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd36b: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd374: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd37d: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd383: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd389: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd38f: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd395: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd39c: 5b                          	popq	%rbx
1403bd39d: 5f                          	popq	%rdi
1403bd39e: 5e                          	popq	%rsi
1403bd39f: 41 5c                       	popq	%r12
1403bd3a1: 41 5d                       	popq	%r13
1403bd3a3: 41 5e                       	popq	%r14
1403bd3a5: 41 5f                       	popq	%r15
1403bd3a7: 5d                          	popq	%rbp
1403bd3a8: c3                          	retq
1403bd3a9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403bd3b0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd3b5: 55                          	pushq	%rbp
1403bd3b6: 41 57                       	pushq	%r15
1403bd3b8: 41 56                       	pushq	%r14
1403bd3ba: 41 55                       	pushq	%r13
1403bd3bc: 41 54                       	pushq	%r12
1403bd3be: 56                          	pushq	%rsi
1403bd3bf: 57                          	pushq	%rdi
1403bd3c0: 53                          	pushq	%rbx
1403bd3c1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd3c8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd3cf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd3d5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd3db: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd3e1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd3e7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd3f0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd3f9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd402: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd40b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd413: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd41b: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
1403bd422: e8 09 f8 98 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1403bd427: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd42f: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd437: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd440: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd449: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd452: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd45b: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd461: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd467: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd46d: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd473: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd47a: 5b                          	popq	%rbx
1403bd47b: 5f                          	popq	%rdi
1403bd47c: 5e                          	popq	%rsi
1403bd47d: 41 5c                       	popq	%r12
1403bd47f: 41 5d                       	popq	%r13
1403bd481: 41 5e                       	popq	%r14
1403bd483: 41 5f                       	popq	%r15
1403bd485: 5d                          	popq	%rbp
1403bd486: c3                          	retq
1403bd487: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1403bd490: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd495: 55                          	pushq	%rbp
1403bd496: 41 57                       	pushq	%r15
1403bd498: 41 56                       	pushq	%r14
1403bd49a: 41 55                       	pushq	%r13
1403bd49c: 41 54                       	pushq	%r12
1403bd49e: 56                          	pushq	%rsi
1403bd49f: 57                          	pushq	%rdi
1403bd4a0: 53                          	pushq	%rbx
1403bd4a1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd4a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd4af: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd4b5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd4bb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd4c1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd4c7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd4d0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd4d9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd4e2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd4eb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd4f3: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd4fb: 48 83 bd 68 07 00 00 00     	cmpq	$0x0, 0x768(%rbp)
1403bd503: 74 25                       	je	0x1403bd52a <.text+0x3bc52a>
1403bd505: 48 8b 85 68 07 00 00        	movq	0x768(%rbp), %rax
1403bd50c: 48 8d 04 85 00 00 00 00     	leaq	(,%rax,4), %rax
1403bd514: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1403bd518: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bd51e: 48 8b 8d 40 07 00 00        	movq	0x740(%rbp), %rcx
1403bd525: e8 f6 66 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bd52a: c6 85 96 07 00 00 00        	movb	$0x0, 0x796(%rbp)
1403bd531: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd539: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd541: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd54a: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd553: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd55c: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd565: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd56b: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd571: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd577: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd57d: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd584: 5b                          	popq	%rbx
1403bd585: 5f                          	popq	%rdi
1403bd586: 5e                          	popq	%rsi
1403bd587: 41 5c                       	popq	%r12
1403bd589: 41 5d                       	popq	%r13
1403bd58b: 41 5e                       	popq	%r14
1403bd58d: 41 5f                       	popq	%r15
1403bd58f: 5d                          	popq	%rbp
1403bd590: c3                          	retq
1403bd591: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1403bd5a0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd5a5: 55                          	pushq	%rbp
1403bd5a6: 41 57                       	pushq	%r15
1403bd5a8: 41 56                       	pushq	%r14
1403bd5aa: 41 55                       	pushq	%r13
1403bd5ac: 41 54                       	pushq	%r12
1403bd5ae: 56                          	pushq	%rsi
1403bd5af: 57                          	pushq	%rdi
1403bd5b0: 53                          	pushq	%rbx
1403bd5b1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd5b8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd5bf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd5c5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd5cb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd5d1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd5d7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd5e0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd5e9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd5f2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd5fb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd603: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd60b: 80 bd 96 07 00 00 00        	cmpb	$0x0, 0x796(%rbp)
1403bd612: 0f 84 92 00 00 00           	je	0x1403bd6aa <.text+0x3bc6aa>
1403bd618: 48 83 bd 68 07 00 00 00     	cmpq	$0x0, 0x768(%rbp)
1403bd620: 74 21                       	je	0x1403bd643 <.text+0x3bc643>
1403bd622: 48 8b 85 68 07 00 00        	movq	0x768(%rbp), %rax
1403bd629: 48 c1 e0 02                 	shlq	$0x2, %rax
1403bd62d: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1403bd631: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bd637: 48 8b 8d 40 07 00 00        	movq	0x740(%rbp), %rcx
1403bd63e: e8 dd 65 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bd643: c6 85 97 07 00 00 01        	movb	$0x1, 0x797(%rbp)
1403bd64a: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd652: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd65a: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd663: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd66c: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd675: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd67e: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd684: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd68a: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd690: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd696: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd69d: 5b                          	popq	%rbx
1403bd69e: 5f                          	popq	%rdi
1403bd69f: 5e                          	popq	%rsi
1403bd6a0: 41 5c                       	popq	%r12
1403bd6a2: 41 5d                       	popq	%r13
1403bd6a4: 41 5e                       	popq	%r14
1403bd6a6: 41 5f                       	popq	%r15
1403bd6a8: 5d                          	popq	%rbp
1403bd6a9: c3                          	retq
1403bd6aa: c6 85 97 07 00 00 01        	movb	$0x1, 0x797(%rbp)
1403bd6b1: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd6b9: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd6c1: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd6ca: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd6d3: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd6dc: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd6e5: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd6eb: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd6f1: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd6f7: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd6fd: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd704: 5b                          	popq	%rbx
1403bd705: 5f                          	popq	%rdi
1403bd706: 5e                          	popq	%rsi
1403bd707: 41 5c                       	popq	%r12
1403bd709: 41 5d                       	popq	%r13
1403bd70b: 41 5e                       	popq	%r14
1403bd70d: 41 5f                       	popq	%r15
1403bd70f: 5d                          	popq	%rbp
1403bd710: c3                          	retq
1403bd711: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1403bd720: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd725: 55                          	pushq	%rbp
1403bd726: 41 57                       	pushq	%r15
1403bd728: 41 56                       	pushq	%r14
1403bd72a: 41 55                       	pushq	%r13
1403bd72c: 41 54                       	pushq	%r12
1403bd72e: 56                          	pushq	%rsi
1403bd72f: 57                          	pushq	%rdi
1403bd730: 53                          	pushq	%rbx
1403bd731: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd738: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd73f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd745: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd74b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd751: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd757: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd760: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd769: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd772: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd77b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd783: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd78b: 80 bd 97 07 00 00 00        	cmpb	$0x0, 0x797(%rbp)
1403bd792: 0f 84 87 00 00 00           	je	0x1403bd81f <.text+0x3bc81f>
1403bd798: 48 8b 85 c0 06 00 00        	movq	0x6c0(%rbp), %rax
1403bd79f: 48 85 c0                    	testq	%rax, %rax
1403bd7a2: 74 1a                       	je	0x1403bd7be <.text+0x3bc7be>
1403bd7a4: 48 8b 8d c8 06 00 00        	movq	0x6c8(%rbp), %rcx
1403bd7ab: 48 c1 e0 02                 	shlq	$0x2, %rax
1403bd7af: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1403bd7b3: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bd7b9: e8 62 64 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bd7be: 66 0f 6f b4 24 d0 00 00 00  	movdqa	0xd0(%rsp), %xmm6
1403bd7c7: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd7cf: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd7d8: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd7e1: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd7ea: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd7f3: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd7f9: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd7ff: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd805: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd80b: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd812: 5b                          	popq	%rbx
1403bd813: 5f                          	popq	%rdi
1403bd814: 5e                          	popq	%rsi
1403bd815: 41 5c                       	popq	%r12
1403bd817: 41 5d                       	popq	%r13
1403bd819: 41 5e                       	popq	%r14
1403bd81b: 41 5f                       	popq	%r15
1403bd81d: 5d                          	popq	%rbp
1403bd81e: c3                          	retq
1403bd81f: 66 0f 6f b4 24 d0 00 00 00  	movdqa	0xd0(%rsp), %xmm6
1403bd828: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd830: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd839: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd842: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd84b: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd854: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd85a: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd860: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd866: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd86c: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd873: 5b                          	popq	%rbx
1403bd874: 5f                          	popq	%rdi
1403bd875: 5e                          	popq	%rsi
1403bd876: 41 5c                       	popq	%r12
1403bd878: 41 5d                       	popq	%r13
1403bd87a: 41 5e                       	popq	%r14
1403bd87c: 41 5f                       	popq	%r15
1403bd87e: 5d                          	popq	%rbp
1403bd87f: c3                          	retq
1403bd880: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd885: 55                          	pushq	%rbp
1403bd886: 41 57                       	pushq	%r15
1403bd888: 41 56                       	pushq	%r14
1403bd88a: 41 55                       	pushq	%r13
1403bd88c: 41 54                       	pushq	%r12
1403bd88e: 56                          	pushq	%rsi
1403bd88f: 57                          	pushq	%rdi
1403bd890: 53                          	pushq	%rbx
1403bd891: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd898: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd89f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd8a5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd8ab: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd8b1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd8b7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd8c0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd8c9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd8d2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd8db: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd8e3: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1403bd8ec: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403bd8f3: e8 58 c6 d8 ff              	callq	0x140149f50 <.text+0x148f50>
1403bd8f8: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd900: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd908: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bd911: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bd91a: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bd923: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bd92c: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bd932: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bd938: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bd93e: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bd944: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bd94b: 5b                          	popq	%rbx
1403bd94c: 5f                          	popq	%rdi
1403bd94d: 5e                          	popq	%rsi
1403bd94e: 41 5c                       	popq	%r12
1403bd950: 41 5d                       	popq	%r13
1403bd952: 41 5e                       	popq	%r14
1403bd954: 41 5f                       	popq	%r15
1403bd956: 5d                          	popq	%rbp
1403bd957: c3                          	retq
1403bd958: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1403bd960: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bd965: 55                          	pushq	%rbp
1403bd966: 41 57                       	pushq	%r15
1403bd968: 41 56                       	pushq	%r14
1403bd96a: 41 55                       	pushq	%r13
1403bd96c: 41 54                       	pushq	%r12
1403bd96e: 56                          	pushq	%rsi
1403bd96f: 57                          	pushq	%rdi
1403bd970: 53                          	pushq	%rbx
1403bd971: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bd978: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bd97f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bd985: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bd98b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bd991: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bd997: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bd9a0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bd9a9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bd9b2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bd9bb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bd9c3: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bd9cb: 48 8b 95 e0 06 00 00        	movq	0x6e0(%rbp), %rdx
1403bd9d2: 48 85 d2                    	testq	%rdx, %rdx
1403bd9d5: 74 16                       	je	0x1403bd9ed <.text+0x3bc9ed>
1403bd9d7: 48 8b 8d e8 06 00 00        	movq	0x6e8(%rbp), %rcx
1403bd9de: 48 c1 e2 02                 	shlq	$0x2, %rdx
1403bd9e2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1403bd9e8: e8 33 62 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bd9ed: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bd9f5: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bd9fd: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bda06: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bda0f: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bda18: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bda21: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bda27: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bda2d: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bda33: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bda39: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bda40: 5b                          	popq	%rbx
1403bda41: 5f                          	popq	%rdi
1403bda42: 5e                          	popq	%rsi
1403bda43: 41 5c                       	popq	%r12
1403bda45: 41 5d                       	popq	%r13
1403bda47: 41 5e                       	popq	%r14
1403bda49: 41 5f                       	popq	%r15
1403bda4b: 5d                          	popq	%rbp
1403bda4c: c3                          	retq
1403bda4d: 0f 1f 00                    	nopl	(%rax)
1403bda50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bda55: 55                          	pushq	%rbp
1403bda56: 41 57                       	pushq	%r15
1403bda58: 41 56                       	pushq	%r14
1403bda5a: 41 55                       	pushq	%r13
1403bda5c: 41 54                       	pushq	%r12
1403bda5e: 56                          	pushq	%rsi
1403bda5f: 57                          	pushq	%rdi
1403bda60: 53                          	pushq	%rbx
1403bda61: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bda68: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bda6f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bda75: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bda7b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bda81: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bda87: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bda90: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bda99: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bdaa2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bdaab: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bdab3: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1403bdabc: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1403bdac6: 48 8b 8d 80 07 00 00        	movq	0x780(%rbp), %rcx
1403bdacd: 48 89 01                    	movq	%rax, (%rcx)
1403bdad0: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bdad8: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bdae0: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bdae9: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bdaf2: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bdafb: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bdb04: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bdb0a: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bdb10: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bdb16: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bdb1c: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bdb23: 5b                          	popq	%rbx
1403bdb24: 5f                          	popq	%rdi
1403bdb25: 5e                          	popq	%rsi
1403bdb26: 41 5c                       	popq	%r12
1403bdb28: 41 5d                       	popq	%r13
1403bdb2a: 41 5e                       	popq	%r14
1403bdb2c: 41 5f                       	popq	%r15
1403bdb2e: 5d                          	popq	%rbp
1403bdb2f: c3                          	retq
1403bdb30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bdb35: 55                          	pushq	%rbp
1403bdb36: 41 57                       	pushq	%r15
1403bdb38: 41 56                       	pushq	%r14
1403bdb3a: 41 55                       	pushq	%r13
1403bdb3c: 41 54                       	pushq	%r12
1403bdb3e: 56                          	pushq	%rsi
1403bdb3f: 57                          	pushq	%rdi
1403bdb40: 53                          	pushq	%rbx
1403bdb41: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bdb48: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bdb4f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bdb55: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bdb5b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bdb61: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bdb67: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bdb70: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bdb79: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bdb82: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bdb8b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bdb93: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bdb9b: 48 8b 85 88 07 00 00        	movq	0x788(%rbp), %rax
1403bdba2: 48 ff 00                    	incq	(%rax)
1403bdba5: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bdbad: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bdbb5: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bdbbe: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bdbc7: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bdbd0: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bdbd9: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bdbdf: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bdbe5: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bdbeb: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bdbf1: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bdbf8: 5b                          	popq	%rbx
1403bdbf9: 5f                          	popq	%rdi
1403bdbfa: 5e                          	popq	%rsi
1403bdbfb: 41 5c                       	popq	%r12
1403bdbfd: 41 5d                       	popq	%r13
1403bdbff: 41 5e                       	popq	%r14
1403bdc01: 41 5f                       	popq	%r15
1403bdc03: 5d                          	popq	%rbp
1403bdc04: c3                          	retq
1403bdc05: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403bdc10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bdc15: 55                          	pushq	%rbp
1403bdc16: 41 57                       	pushq	%r15
1403bdc18: 41 56                       	pushq	%r14
1403bdc1a: 41 55                       	pushq	%r13
1403bdc1c: 41 54                       	pushq	%r12
1403bdc1e: 56                          	pushq	%rsi
1403bdc1f: 57                          	pushq	%rdi
1403bdc20: 53                          	pushq	%rbx
1403bdc21: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bdc28: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bdc2f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bdc35: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bdc3b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bdc41: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bdc47: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bdc50: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bdc59: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bdc62: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bdc6b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bdc73: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bdc7b: 48 8b 85 88 07 00 00        	movq	0x788(%rbp), %rax
1403bdc82: 48 ff 00                    	incq	(%rax)
1403bdc85: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bdc8d: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bdc95: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bdc9e: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bdca7: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bdcb0: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bdcb9: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bdcbf: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bdcc5: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bdccb: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bdcd1: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bdcd8: 5b                          	popq	%rbx
1403bdcd9: 5f                          	popq	%rdi
1403bdcda: 5e                          	popq	%rsi
1403bdcdb: 41 5c                       	popq	%r12
1403bdcdd: 41 5d                       	popq	%r13
1403bdcdf: 41 5e                       	popq	%r14
1403bdce1: 41 5f                       	popq	%r15
1403bdce3: 5d                          	popq	%rbp
1403bdce4: c3                          	retq
1403bdce5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1403bdcf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bdcf5: 55                          	pushq	%rbp
1403bdcf6: 41 57                       	pushq	%r15
1403bdcf8: 41 56                       	pushq	%r14
1403bdcfa: 41 55                       	pushq	%r13
1403bdcfc: 41 54                       	pushq	%r12
1403bdcfe: 56                          	pushq	%rsi
1403bdcff: 57                          	pushq	%rdi
1403bdd00: 53                          	pushq	%rbx
1403bdd01: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bdd08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bdd0f: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bdd15: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bdd1b: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bdd21: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bdd27: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bdd30: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bdd39: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bdd42: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bdd4b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bdd53: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1403bdd5c: 48 8d 8d b0 03 00 00        	leaq	0x3b0(%rbp), %rcx
1403bdd63: e8 98 81 23 00              	callq	0x1405f5f00 <.text+0x5f4f00>
1403bdd68: 66 0f 6f b4 24 d0 00 00 00  	movdqa	0xd0(%rsp), %xmm6
1403bdd71: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bdd79: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bdd82: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bdd8b: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bdd94: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bdd9d: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bdda3: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bdda9: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bddaf: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bddb5: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bddbc: 5b                          	popq	%rbx
1403bddbd: 5f                          	popq	%rdi
1403bddbe: 5e                          	popq	%rsi
1403bddbf: 41 5c                       	popq	%r12
1403bddc1: 41 5d                       	popq	%r13
1403bddc3: 41 5e                       	popq	%r14
1403bddc5: 41 5f                       	popq	%r15
1403bddc7: 5d                          	popq	%rbp
1403bddc8: c3                          	retq
1403bddc9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403bddd0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bddd5: 55                          	pushq	%rbp
1403bddd6: 41 57                       	pushq	%r15
1403bddd8: 41 56                       	pushq	%r14
1403bddda: 41 55                       	pushq	%r13
1403bdddc: 41 54                       	pushq	%r12
1403bddde: 56                          	pushq	%rsi
1403bdddf: 57                          	pushq	%rdi
1403bdde0: 53                          	pushq	%rbx
1403bdde1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bdde8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bddef: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bddf5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bddfb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bde01: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bde07: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bde10: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bde19: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bde22: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bde2b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bde33: 66 0f 7f b4 24 d0 00 00 00  	movdqa	%xmm6, 0xd0(%rsp)
1403bde3c: 48 8d 8d 90 06 00 00        	leaq	0x690(%rbp), %rcx
1403bde43: e8 d8 be 24 00              	callq	0x140609d20 <.text+0x608d20>
1403bde48: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bde50: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bde58: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bde61: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bde6a: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bde73: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bde7c: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bde82: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bde88: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bde8e: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bde94: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bde9b: 5b                          	popq	%rbx
1403bde9c: 5f                          	popq	%rdi
1403bde9d: 5e                          	popq	%rsi
1403bde9e: 41 5c                       	popq	%r12
1403bdea0: 41 5d                       	popq	%r13
1403bdea2: 41 5e                       	popq	%r14
1403bdea4: 41 5f                       	popq	%r15
1403bdea6: 5d                          	popq	%rbp
1403bdea7: c3                          	retq
1403bdea8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1403bdeb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bdeb5: 55                          	pushq	%rbp
1403bdeb6: 41 57                       	pushq	%r15
1403bdeb8: 41 56                       	pushq	%r14
1403bdeba: 41 55                       	pushq	%r13
1403bdebc: 41 54                       	pushq	%r12
1403bdebe: 56                          	pushq	%rsi
1403bdebf: 57                          	pushq	%rdi
1403bdec0: 53                          	pushq	%rbx
1403bdec1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bdec8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bdecf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bded5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bdedb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bdee1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bdee7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bdef0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bdef9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bdf02: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bdf0b: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403bdf13: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403bdf1b: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1403bdf22: 48 85 d2                    	testq	%rdx, %rdx
1403bdf25: 74 16                       	je	0x1403bdf3d <.text+0x3bcf3d>
1403bdf27: 48 8b 8d b8 00 00 00        	movq	0xb8(%rbp), %rcx
1403bdf2e: 48 c1 e2 03                 	shlq	$0x3, %rdx
1403bdf32: 41 b8 08 00 00 00           	movl	$0x8, %r8d
1403bdf38: e8 e3 5c 25 00              	callq	0x140613c20 <.text+0x612c20>
1403bdf3d: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403bdf45: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403bdf4d: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403bdf56: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403bdf5f: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403bdf68: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403bdf71: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403bdf77: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403bdf7d: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403bdf83: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403bdf89: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403bdf90: 5b                          	popq	%rbx
1403bdf91: 5f                          	popq	%rdi
1403bdf92: 5e                          	popq	%rsi
1403bdf93: 41 5c                       	popq	%r12
1403bdf95: 41 5d                       	popq	%r13
1403bdf97: 41 5e                       	popq	%r14
1403bdf99: 41 5f                       	popq	%r15
1403bdf9b: 5d                          	popq	%rbp
1403bdf9c: c3                          	retq
1403bdf9d: 0f 1f 00                    	nopl	(%rax)
1403bdfa0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403bdfa5: 55                          	pushq	%rbp
1403bdfa6: 41 57                       	pushq	%r15
1403bdfa8: 41 56                       	pushq	%r14
1403bdfaa: 41 55                       	pushq	%r13
1403bdfac: 41 54                       	pushq	%r12
1403bdfae: 56                          	pushq	%rsi
1403bdfaf: 57                          	pushq	%rdi
1403bdfb0: 53                          	pushq	%rbx
1403bdfb1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403bdfb8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403bdfbf: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403bdfc5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403bdfcb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403bdfd1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403bdfd7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403bdfe0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403bdfe9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403bdff2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403bdffb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403be003: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403be00b: 48 8b 95 38 07 00 00        	movq	0x738(%rbp), %rdx
1403be012: 48 85 d2                    	testq	%rdx, %rdx
1403be015: 74 12                       	je	0x1403be029 <.text+0x3bd029>
1403be017: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1403be01d: 48 8b 8d 18 07 00 00        	movq	0x718(%rbp), %rcx
1403be024: e8 f7 5b 25 00              	callq	0x140613c20 <.text+0x612c20>
1403be029: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403be031: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403be039: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403be042: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403be04b: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403be054: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403be05d: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403be063: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403be069: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403be06f: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403be075: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403be07c: 5b                          	popq	%rbx
1403be07d: 5f                          	popq	%rdi
1403be07e: 5e                          	popq	%rsi
1403be07f: 41 5c                       	popq	%r12
1403be081: 41 5d                       	popq	%r13
1403be083: 41 5e                       	popq	%r14
1403be085: 41 5f                       	popq	%r15
1403be087: 5d                          	popq	%rbp
1403be088: c3                          	retq
1403be089: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1403be090: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1403be095: 55                          	pushq	%rbp
1403be096: 41 57                       	pushq	%r15
1403be098: 41 56                       	pushq	%r14
1403be09a: 41 55                       	pushq	%r13
1403be09c: 41 54                       	pushq	%r12
1403be09e: 56                          	pushq	%rsi
1403be09f: 57                          	pushq	%rdi
1403be0a0: 53                          	pushq	%rbx
1403be0a1: 48 81 ec e8 00 00 00        	subq	$0xe8, %rsp
1403be0a8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1403be0af: 44 0f 29 7c 24 40           	movaps	%xmm15, 0x40(%rsp)
1403be0b5: 44 0f 29 74 24 50           	movaps	%xmm14, 0x50(%rsp)
1403be0bb: 44 0f 29 6c 24 60           	movaps	%xmm13, 0x60(%rsp)
1403be0c1: 44 0f 29 64 24 70           	movaps	%xmm12, 0x70(%rsp)
1403be0c7: 44 0f 29 9c 24 80 00 00 00  	movaps	%xmm11, 0x80(%rsp)
1403be0d0: 44 0f 29 94 24 90 00 00 00  	movaps	%xmm10, 0x90(%rsp)
1403be0d9: 44 0f 29 8c 24 a0 00 00 00  	movaps	%xmm9, 0xa0(%rsp)
1403be0e2: 44 0f 29 84 24 b0 00 00 00  	movaps	%xmm8, 0xb0(%rsp)
1403be0eb: 0f 29 bc 24 c0 00 00 00     	movaps	%xmm7, 0xc0(%rsp)
1403be0f3: 0f 29 b4 24 d0 00 00 00     	movaps	%xmm6, 0xd0(%rsp)
1403be0fb: 48 83 bd e8 05 00 00 00     	cmpq	$0x0, 0x5e8(%rbp)
1403be103: 74 2f                       	je	0x1403be134 <.text+0x3bd134>
1403be105: 48 8d 85 f0 05 00 00        	leaq	0x5f0(%rbp), %rax
1403be10c: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1403be113: 48 8d 85 50 02 00 00        	leaq	0x250(%rbp), %rax
1403be11a: 48 89 85 48 02 00 00        	movq	%rax, 0x248(%rbp)
1403be121: 48 8d 0d 10 f4 60 02        	leaq	0x260f410(%rip), %rcx   # 0x1429cd538
1403be128: 48 8d 95 48 02 00 00        	leaq	0x248(%rbp), %rdx
1403be12f: e8 4c 44 ce ff              	callq	0x1400a2580 <.text+0xa1580>
1403be134: 0f 28 b4 24 d0 00 00 00     	movaps	0xd0(%rsp), %xmm6
1403be13c: 0f 28 bc 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm7
1403be144: 44 0f 28 84 24 b0 00 00 00  	movaps	0xb0(%rsp), %xmm8
1403be14d: 44 0f 28 8c 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm9
1403be156: 44 0f 28 94 24 90 00 00 00  	movaps	0x90(%rsp), %xmm10
1403be15f: 44 0f 28 9c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm11
1403be168: 44 0f 28 64 24 70           	movaps	0x70(%rsp), %xmm12
1403be16e: 44 0f 28 6c 24 60           	movaps	0x60(%rsp), %xmm13
1403be174: 44 0f 28 74 24 50           	movaps	0x50(%rsp), %xmm14
1403be17a: 44 0f 28 7c 24 40           	movaps	0x40(%rsp), %xmm15
1403be180: 48 81 c4 e8 00 00 00        	addq	$0xe8, %rsp
1403be187: 5b                          	popq	%rbx
1403be188: 5f                          	popq	%rdi
1403be189: 5e                          	popq	%rsi
1403be18a: 41 5c                       	popq	%r12
1403be18c: 41 5d                       	popq	%r13
1403be18e: 41 5e                       	popq	%r14
1403be190: 41 5f                       	popq	%r15
1403be192: 5d                          	popq	%rbp
1403be193: c3                          	retq
