
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140864430: 55                          	pushq	%rbp
140864431: 41 57                       	pushq	%r15
140864433: 41 56                       	pushq	%r14
140864435: 41 54                       	pushq	%r12
140864437: 56                          	pushq	%rsi
140864438: 57                          	pushq	%rdi
140864439: 53                          	pushq	%rbx
14086443a: 48 81 ec 50 01 00 00        	subq	$0x150, %rsp            # imm = 0x150
140864441: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140864449: 0f 29 b5 c0 00 00 00        	movaps	%xmm6, 0xc0(%rbp)
140864450: 48 c7 85 b8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xb8(%rbp)
14086445b: 4c 89 cb                    	movq	%r9, %rbx
14086445e: 4d 89 c7                    	movq	%r8, %r15
140864461: 48 89 d6                    	movq	%rdx, %rsi
140864464: 49 89 ce                    	movq	%rcx, %r14
140864467: 48 8d 15 42 1f 23 02        	leaq	0x2231f42(%rip), %rdx   # 0x142a963b0
14086446e: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
140864472: 41 b8 1a 00 00 00           	movl	$0x1a, %r8d
140864478: e8 33 aa 2a 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
14086447d: 31 c0                       	xorl	%eax, %eax
14086447f: 48 3b 45 40                 	cmpq	0x40(%rbp), %rax
140864483: 0f 80 af 01 00 00           	jo	0x140864638 <.text+0x863638>
140864489: 48 8b bd 38 01 00 00        	movq	0x138(%rbp), %rdi
140864490: 4c 8b a5 30 01 00 00        	movq	0x130(%rbp), %r12
140864497: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14086449e: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
1408644a2: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1408644a9: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
1408644ad: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
1408644b4: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1408644b8: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1408644bf: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
1408644c3: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1408644c7: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1408644cb: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1408644cf: 0f 10 5d 70                 	movups	0x70(%rbp), %xmm3
1408644d3: 0f 29 5d f0                 	movaps	%xmm3, -0x10(%rbp)
1408644d7: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1408644db: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1408644df: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1408644e3: 4d 8b 0f                    	movq	(%r15), %r9
1408644e6: 48 8b 06                    	movq	(%rsi), %rax
1408644e9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1408644ee: 48 8d 15 23 1f 23 02        	leaq	0x2231f23(%rip), %rdx   # 0x142a96418
1408644f5: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1408644f9: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1408644ff: e8 9c 67 2b 00              	callq	0x140b1aca0 <.text+0xb19ca0>
140864504: 48 89 c6                    	movq	%rax, %rsi
140864507: 49 8b 06                    	movq	(%r14), %rax
14086450a: 48 8b 08                    	movq	(%rax), %rcx
14086450d: 48 83 c1 10                 	addq	$0x10, %rcx
140864511: 49 8b 04 24                 	movq	(%r12), %rax
140864515: 48 8b 13                    	movq	(%rbx), %rdx
140864518: 48 89 54 24 30              	movq	%rdx, 0x30(%rsp)
14086451d: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
140864522: 48 c7 44 24 20 0e 00 00 00  	movq	$0xe, 0x20(%rsp)
14086452b: 48 8d 15 ed 1e 23 02        	leaq	0x2231eed(%rip), %rdx   # 0x142a9641f
140864532: 4c 8d 0d fa 1e 23 02        	leaq	0x2231efa(%rip), %r9    # 0x142a96433
140864539: 41 b8 14 00 00 00           	movl	$0x14, %r8d
14086453f: e8 fc b8 2d 00              	callq	0x140b3fe40 <.text+0xb3ee40>
140864544: 48 89 c3                    	movq	%rax, %rbx
140864547: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14086454b: e8 30 8c 43 00              	callq	0x140c9d180 <.text+0xc9c180>
140864550: 0f 28 75 40                 	movaps	0x40(%rbp), %xmm6
140864554: 48 8b 45 5c                 	movq	0x5c(%rbp), %rax
140864558: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
14086455c: 8b 45 64                    	movl	0x64(%rbp), %eax
14086455f: 89 45 c8                    	movl	%eax, -0x38(%rbp)
140864562: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
140864566: 48 89 fa                    	movq	%rdi, %rdx
140864569: e8 32 01 4b 00              	callq	0x140d146a0 <.text+0xd136a0>
14086456e: 48 8b 7d 48                 	movq	0x48(%rbp), %rdi
140864572: 48 85 ff                    	testq	%rdi, %rdi
140864575: 75 04                       	jne	0x14086457b <.text+0x86357b>
140864577: 48 8b 7d 50                 	movq	0x50(%rbp), %rdi
14086457b: 4c 8b 7d 60                 	movq	0x60(%rbp), %r15
14086457f: 48 8b 07                    	movq	(%rdi), %rax
140864582: 4c 8b 77 10                 	movq	0x10(%rdi), %r14
140864586: 4c 29 f0                    	subq	%r14, %rax
140864589: 48 83 f8 77                 	cmpq	$0x77, %rax
14086458d: 0f 86 80 00 00 00           	jbe	0x140864613 <.text+0x863613>
140864593: 48 8b 47 08                 	movq	0x8(%rdi), %rax
140864597: 48 8d 0d c2 8e a7 00        	leaq	0xa78ec2(%rip), %rcx    # 0x1412dd460 <.text+0x12dc460>
14086459e: 4a 89 0c 30                 	movq	%rcx, (%rax,%r14)
1408645a2: 42 0f 11 74 30 08           	movups	%xmm6, 0x8(%rax,%r14)
1408645a8: f2 0f 10 05 10 18 23 02     	movsd	0x2231810(%rip), %xmm0  # 0x142a95dc0
1408645b0: f2 42 0f 11 44 30 18        	movsd	%xmm0, 0x18(%rax,%r14)
1408645b7: 42 c7 44 30 20 00 00 00 80  	movl	$0x80000000, 0x20(%rax,%r14) # imm = 0x80000000
1408645c0: 48 8b 4d c0                 	movq	-0x40(%rbp), %rcx
1408645c4: 4a 89 4c 30 24              	movq	%rcx, 0x24(%rax,%r14)
1408645c9: 8b 4d c8                    	movl	-0x38(%rbp), %ecx
1408645cc: 42 89 4c 30 2c              	movl	%ecx, 0x2c(%rax,%r14)
1408645d1: 4a 89 74 30 38              	movq	%rsi, 0x38(%rax,%r14)
1408645d6: 4a 89 5c 30 40              	movq	%rbx, 0x40(%rax,%r14)
1408645db: 42 c6 44 30 48 00           	movb	$0x0, 0x48(%rax,%r14)
1408645e1: 4e 89 7c 30 58              	movq	%r15, 0x58(%rax,%r14)
1408645e6: 48 8d 0d 43 07 d0 00        	leaq	0xd00743(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1408645ed: 4a 89 4c 30 68              	movq	%rcx, 0x68(%rax,%r14)
1408645f2: 49 83 c6 78                 	addq	$0x78, %r14
1408645f6: 4c 89 77 10                 	movq	%r14, 0x10(%rdi)
1408645fa: 0f 28 b5 c0 00 00 00        	movaps	0xc0(%rbp), %xmm6
140864601: 48 81 c4 50 01 00 00        	addq	$0x150, %rsp            # imm = 0x150
140864608: 5b                          	popq	%rbx
140864609: 5f                          	popq	%rdi
14086460a: 5e                          	popq	%rsi
14086460b: 41 5c                       	popq	%r12
14086460d: 41 5e                       	popq	%r14
14086460f: 41 5f                       	popq	%r15
140864611: 5d                          	popq	%rbp
140864612: c3                          	retq
140864613: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
14086461c: 41 b8 78 00 00 00           	movl	$0x78, %r8d
140864622: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140864628: 48 89 f9                    	movq	%rdi, %rcx
14086462b: 4c 89 f2                    	movq	%r14, %rdx
14086462e: e8 2d 54 04 02              	callq	0x1428a9a60 <.text+0x28a8a60>
140864633: e9 5b ff ff ff              	jmp	0x140864593 <.text+0x863593>
140864638: 48 8b 45 48                 	movq	0x48(%rbp), %rax
14086463c: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140864640: 48 8d 05 b9 1d 23 02        	leaq	0x2231db9(%rip), %rax   # 0x142a96400
140864647: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14086464c: 48 8d 0d ad 18 23 02        	leaq	0x22318ad(%rip), %rcx   # 0x142a95f00
140864653: 4c 8d 0d 86 18 23 02        	leaq	0x2231886(%rip), %r9    # 0x142a95ee0
14086465a: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
14086465e: ba 2b 00 00 00              	movl	$0x2b, %edx
140864663: e8 f8 50 07 02              	callq	0x1428d9760 <.text+0x28d8760>
140864668: 0f 0b                       	ud2
14086466a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140864670: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140864675: 55                          	pushq	%rbp
140864676: 41 57                       	pushq	%r15
140864678: 41 56                       	pushq	%r14
14086467a: 41 54                       	pushq	%r12
14086467c: 56                          	pushq	%rsi
14086467d: 57                          	pushq	%rdi
14086467e: 53                          	pushq	%rbx
14086467f: 48 83 ec 50                 	subq	$0x50, %rsp
140864683: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
14086468a: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
14086468f: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140864693: e8 98 85 4e 00              	callq	0x140d4cc30 <.text+0xd4bc30>
140864698: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
14086469d: 48 83 c4 50                 	addq	$0x50, %rsp
1408646a1: 5b                          	popq	%rbx
1408646a2: 5f                          	popq	%rdi
1408646a3: 5e                          	popq	%rsi
1408646a4: 41 5c                       	popq	%r12
1408646a6: 41 5e                       	popq	%r14
1408646a8: 41 5f                       	popq	%r15
1408646aa: 5d                          	popq	%rbp
1408646ab: c3                          	retq
