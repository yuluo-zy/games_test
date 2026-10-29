
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141664380: 08 01                       	orb	%al, (%rcx)
141664382: 00 00                       	addb	%al, (%rax)
141664384: 48 89 ca                    	movq	%rcx, %rdx
141664387: 48 f7 d2                    	notq	%rdx
14166438a: 48 21 c2                    	andq	%rax, %rdx
14166438d: 4c 39 c2                    	cmpq	%r8, %rdx
141664390: 0f 85 4f ff ff ff           	jne	0x1416642e5 <.text+0x16632e5>
141664396: 48 85 c1                    	testq	%rax, %rcx
141664399: 74 1f                       	je	0x1416643ba <.text+0x16633ba>
14166439b: c6 46 01 01                 	movb	$0x1, 0x1(%rsi)
14166439f: eb 1d                       	jmp	0x1416643be <.text+0x16633be>
1416643a1: 4c 01 ca                    	addq	%r9, %rdx
1416643a4: 4c 03 87 00 01 00 00        	addq	0x100(%rdi), %r8
1416643ab: 48 8b 42 08                 	movq	0x8(%rdx), %rax
1416643af: 4c 89 02                    	movq	%r8, (%rdx)
1416643b2: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1416643b6: 31 c0                       	xorl	%eax, %eax
1416643b8: eb 06                       	jmp	0x1416643c0 <.text+0x16633c0>
1416643ba: c6 46 01 00                 	movb	$0x0, 0x1(%rsi)
1416643be: b0 01                       	movb	$0x1, %al
1416643c0: 88 06                       	movb	%al, (%rsi)
1416643c2: 48 89 f0                    	movq	%rsi, %rax
1416643c5: 48 83 c4 30                 	addq	$0x30, %rsp
1416643c9: 5b                          	popq	%rbx
1416643ca: 5f                          	popq	%rdi
1416643cb: 5e                          	popq	%rsi
1416643cc: c3                          	retq
1416643cd: 4c 8d 05 dc ea 61 01        	leaq	0x161eadc(%rip), %r8    # 0x142c82eb0
1416643d4: e8 3f 51 27 01              	callq	0x1428d9518 <.text+0x28d8518>
1416643d9: cc                          	int3
1416643da: cc                          	int3
1416643db: cc                          	int3
1416643dc: cc                          	int3
1416643dd: cc                          	int3
1416643de: cc                          	int3
1416643df: cc                          	int3
1416643e0: 55                          	pushq	%rbp
1416643e1: 56                          	pushq	%rsi
1416643e2: 57                          	pushq	%rdi
1416643e3: 48 81 ec c0 01 00 00        	subq	$0x1c0, %rsp            # imm = 0x1C0
1416643ea: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416643f2: 48 c7 85 38 01 00 00 fe ff ff ff    	movq	$-0x2, 0x138(%rbp)
1416643fd: 48 89 ce                    	movq	%rcx, %rsi
141664400: 0f 57 c0                    	xorps	%xmm0, %xmm0
141664403: 0f 11 45 18                 	movups	%xmm0, 0x18(%rbp)
141664407: 0f 11 45 30                 	movups	%xmm0, 0x30(%rbp)
14166440b: 0f 11 45 48                 	movups	%xmm0, 0x48(%rbp)
14166440f: 0f 11 45 60                 	movups	%xmm0, 0x60(%rbp)
141664413: 0f 11 45 78                 	movups	%xmm0, 0x78(%rbp)
141664417: c7 85 88 00 00 00 00 00 00 00       	movl	$0x0, 0x88(%rbp)
141664421: 0f 11 85 b0 00 00 00        	movups	%xmm0, 0xb0(%rbp)
141664428: 0f 11 85 c8 00 00 00        	movups	%xmm0, 0xc8(%rbp)
14166442f: 0f 11 85 e0 00 00 00        	movups	%xmm0, 0xe0(%rbp)
141664436: 0f 11 85 f8 00 00 00        	movups	%xmm0, 0xf8(%rbp)
14166443d: 0f 11 85 10 01 00 00        	movups	%xmm0, 0x110(%rbp)
141664444: c7 85 20 01 00 00 00 00 00 00       	movl	$0x0, 0x120(%rbp)
14166444e: 48 c7 45 a0 00 00 00 00     	movq	$0x0, -0x60(%rbp)
141664456: 48 c7 45 f8 00 00 00 00     	movq	$0x0, -0x8(%rbp)
14166445e: 48 c7 45 00 08 00 00 00     	movq	$0x8, (%rbp)
141664466: 48 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%rbp)
14166446e: 48 c7 45 10 10 00 00 00     	movq	$0x10, 0x10(%rbp)
141664476: 48 c7 45 28 10 00 00 00     	movq	$0x10, 0x28(%rbp)
14166447e: 48 c7 45 40 10 00 00 00     	movq	$0x10, 0x40(%rbp)
141664486: 48 c7 45 58 10 00 00 00     	movq	$0x10, 0x58(%rbp)
14166448e: 48 c7 45 70 10 00 00 00     	movq	$0x10, 0x70(%rbp)
141664496: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1416644a0: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1416644a7: 48 8d 05 87 ee 61 01        	leaq	0x161ee87(%rip), %rax   # 0x142c83335
1416644ae: 48 89 85 98 00 00 00        	movq	%rax, 0x98(%rbp)
1416644b5: 48 c7 85 a0 00 00 00 32 00 00 00    	movq	$0x32, 0xa0(%rbp)
1416644c0: 48 c7 85 a8 00 00 00 10 00 00 00    	movq	$0x10, 0xa8(%rbp)
1416644cb: 48 c7 85 c0 00 00 00 10 00 00 00    	movq	$0x10, 0xc0(%rbp)
1416644d6: 48 c7 85 d8 00 00 00 10 00 00 00    	movq	$0x10, 0xd8(%rbp)
1416644e1: 48 c7 85 f0 00 00 00 10 00 00 00    	movq	$0x10, 0xf0(%rbp)
1416644ec: 48 c7 85 08 01 00 00 10 00 00 00    	movq	$0x10, 0x108(%rbp)
1416644f7: c7 85 28 01 00 00 00 00 00 00       	movl	$0x0, 0x128(%rbp)
141664501: 66 c7 85 2c 01 00 00 01 00  	movw	$0x1, 0x12c(%rbp)
14166450a: c7 85 30 01 00 00 00 00 00 00       	movl	$0x0, 0x130(%rbp)
141664514: 0f b6 05 ee 06 fd 01        	movzbl	0x1fd06ee(%rip), %eax   # 0x143634c09
14166451b: b9 98 01 00 00              	movl	$0x198, %ecx            # imm = 0x198
141664520: ba 08 00 00 00              	movl	$0x8, %edx
141664525: e8 e6 f6 fa fe              	callq	0x140613c10 <.text+0x612c10>
14166452a: 48 85 c0                    	testq	%rax, %rax
14166452d: 74 35                       	je	0x141664564 <.text+0x1663564>
14166452f: 48 89 c7                    	movq	%rax, %rdi
141664532: 48 8d 55 a0                 	leaq	-0x60(%rbp), %rdx
141664536: 41 b8 98 01 00 00           	movl	$0x198, %r8d            # imm = 0x198
14166453c: 48 89 c1                    	movq	%rax, %rcx
14166453f: e8 bc f6 2b 01              	callq	0x142923c00 <.text+0x2922c00>
141664544: 4c 8d 05 7d e9 61 01        	leaq	0x161e97d(%rip), %r8    # 0x142c82ec8
14166454b: 48 89 f1                    	movq	%rsi, %rcx
14166454e: 48 89 fa                    	movq	%rdi, %rdx
141664551: e8 ba 05 68 ff              	callq	0x140ce4b10 <.text+0xce3b10>
141664556: 48 89 f0                    	movq	%rsi, %rax
141664559: 48 81 c4 c0 01 00 00        	addq	$0x1c0, %rsp            # imm = 0x1C0
141664560: 5f                          	popq	%rdi
141664561: 5e                          	popq	%rsi
141664562: 5d                          	popq	%rbp
141664563: c3                          	retq
141664564: b9 08 00 00 00              	movl	$0x8, %ecx
141664569: ba 98 01 00 00              	movl	$0x198, %edx            # imm = 0x198
14166456e: e8 8d 4a 27 01              	callq	0x1428d9000 <.text+0x28d8000>
141664573: 0f 0b                       	ud2
141664575: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
141664580: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141664585: 55                          	pushq	%rbp
141664586: 56                          	pushq	%rsi
141664587: 57                          	pushq	%rdi
141664588: 48 83 ec 20                 	subq	$0x20, %rsp
14166458c: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141664593: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
141664597: e8 74 c9 f6 fe              	callq	0x1405d0f10 <.text+0x5cff10>
14166459c: 90                          	nop
14166459d: 48 83 c4 20                 	addq	$0x20, %rsp
1416645a1: 5f                          	popq	%rdi
1416645a2: 5e                          	popq	%rsi
1416645a3: 5d                          	popq	%rbp
1416645a4: c3                          	retq
1416645a5: cc                          	int3
1416645a6: cc                          	int3
1416645a7: cc                          	int3
1416645a8: cc                          	int3
1416645a9: cc                          	int3
1416645aa: cc                          	int3
1416645ab: cc                          	int3
1416645ac: cc                          	int3
1416645ad: cc                          	int3
1416645ae: cc                          	int3
1416645af: cc                          	int3
1416645b0: 55                          	pushq	%rbp
1416645b1: 56                          	pushq	%rsi
1416645b2: 57                          	pushq	%rdi
1416645b3: 48 81 ec a0 01 00 00        	subq	$0x1a0, %rsp            # imm = 0x1A0
1416645ba: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1416645c2: 48 c7 85 18 01 00 00 fe ff ff ff    	movq	$-0x2, 0x118(%rbp)
1416645cd: 48 89 ce                    	movq	%rcx, %rsi
1416645d0: 0f 57 c0                    	xorps	%xmm0, %xmm0
1416645d3: 0f 11 45 f8                 	movups	%xmm0, -0x8(%rbp)
1416645d7: 0f 11 45 10                 	movups	%xmm0, 0x10(%rbp)
1416645db: 0f 11 45 28                 	movups	%xmm0, 0x28(%rbp)
1416645df: 0f 11 45 40                 	movups	%xmm0, 0x40(%rbp)
1416645e3: 0f 11 45 58                 	movups	%xmm0, 0x58(%rbp)
1416645e7: c7 45 68 00 00 00 00        	movl	$0x0, 0x68(%rbp)
1416645ee: 0f 11 85 90 00 00 00        	movups	%xmm0, 0x90(%rbp)
1416645f5: 0f 11 85 a8 00 00 00        	movups	%xmm0, 0xa8(%rbp)
1416645fc: 0f 11 85 c0 00 00 00        	movups	%xmm0, 0xc0(%rbp)
