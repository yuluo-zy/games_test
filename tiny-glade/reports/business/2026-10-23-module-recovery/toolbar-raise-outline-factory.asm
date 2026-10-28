
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409550a0: 55                          	pushq	%rbp
1409550a1: 41 57                       	pushq	%r15
1409550a3: 41 56                       	pushq	%r14
1409550a5: 41 55                       	pushq	%r13
1409550a7: 41 54                       	pushq	%r12
1409550a9: 56                          	pushq	%rsi
1409550aa: 57                          	pushq	%rdi
1409550ab: 53                          	pushq	%rbx
1409550ac: 48 81 ec 28 01 00 00        	subq	$0x128, %rsp            # imm = 0x128
1409550b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409550bb: 0f 29 b5 90 00 00 00        	movaps	%xmm6, 0x90(%rbp)
1409550c2: 48 c7 85 88 00 00 00 fe ff ff ff    	movq	$-0x2, 0x88(%rbp)
1409550cd: 4c 89 cb                    	movq	%r9, %rbx
1409550d0: 4d 89 c6                    	movq	%r8, %r14
1409550d3: 48 89 d7                    	movq	%rdx, %rdi
1409550d6: 48 89 ce                    	movq	%rcx, %rsi
1409550d9: 0f b6 05 29 fb cd 02        	movzbl	0x2cdfb29(%rip), %eax   # 0x143634c09
1409550e0: b9 08 00 00 00              	movl	$0x8, %ecx
1409550e5: ba 08 00 00 00              	movl	$0x8, %edx
1409550ea: e8 21 eb cb ff              	callq	0x140613c10 <.text+0x612c10>
1409550ef: 48 85 c0                    	testq	%rax, %rax
1409550f2: 0f 84 3d 02 00 00           	je	0x140955335 <.text+0x954335>
1409550f8: 4c 89 75 78                 	movq	%r14, 0x78(%rbp)
1409550fc: 48 89 5d 70                 	movq	%rbx, 0x70(%rbp)
140955100: 48 89 85 80 00 00 00        	movq	%rax, 0x80(%rbp)
140955107: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
14095510b: 31 db                       	xorl	%ebx, %ebx
14095510d: b9 0c 00 00 00              	movl	$0xc, %ecx
140955112: 4c 89 f8                    	movq	%r15, %rax
140955115: 48 f7 e1                    	mulq	%rcx
140955118: 49 89 c4                    	movq	%rax, %r12
14095511b: 0f 80 00 02 00 00           	jo	0x140955321 <.text+0x954321>
140955121: 48 b8 fc ff ff ff ff ff ff 7f       	movabsq	$0x7ffffffffffffffc, %rax # imm = 0x7FFFFFFFFFFFFFFC
14095512b: 49 39 c4                    	cmpq	%rax, %r12
14095512e: 0f 87 ed 01 00 00           	ja	0x140955321 <.text+0x954321>
140955134: 4c 8b 6f 08                 	movq	0x8(%rdi), %r13
140955138: 4d 85 e4                    	testq	%r12, %r12
14095513b: 74 2a                       	je	0x140955167 <.text+0x954167>
14095513d: 0f b6 05 c5 fa cd 02        	movzbl	0x2cdfac5(%rip), %eax   # 0x143634c09
140955144: bb 04 00 00 00              	movl	$0x4, %ebx
140955149: ba 04 00 00 00              	movl	$0x4, %edx
14095514e: 4c 89 e1                    	movq	%r12, %rcx
140955151: e8 ba ea cb ff              	callq	0x140613c10 <.text+0x612c10>
140955156: 48 85 c0                    	testq	%rax, %rax
140955159: 0f 84 c2 01 00 00           	je	0x140955321 <.text+0x954321>
14095515f: 49 89 c6                    	movq	%rax, %r14
140955162: 4c 89 fb                    	movq	%r15, %rbx
140955165: eb 08                       	jmp	0x14095516f <.text+0x95416f>
140955167: 41 be 04 00 00 00           	movl	$0x4, %r14d
14095516d: 31 db                       	xorl	%ebx, %ebx
14095516f: 4c 89 f1                    	movq	%r14, %rcx
140955172: 4c 89 ea                    	movq	%r13, %rdx
140955175: 4d 89 e0                    	movq	%r12, %r8
140955178: e8 83 ea fc 01              	callq	0x142923c00 <.text+0x2922c00>
14095517d: 48 89 5d 48                 	movq	%rbx, 0x48(%rbp)
140955181: 4c 89 75 50                 	movq	%r14, 0x50(%rbp)
140955185: 4c 89 7d 58                 	movq	%r15, 0x58(%rbp)
140955189: 48 b8 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rax # imm = 0x8000000000000002
140955193: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
140955197: 0f b6 05 6b fa cd 02        	movzbl	0x2cdfa6b(%rip), %eax   # 0x143634c09
14095519e: b9 13 00 00 00              	movl	$0x13, %ecx
1409551a3: ba 01 00 00 00              	movl	$0x1, %edx
1409551a8: e8 63 ea cb ff              	callq	0x140613c10 <.text+0x612c10>
1409551ad: 48 85 c0                    	testq	%rax, %rax
1409551b0: 0f 84 8e 01 00 00           	je	0x140955344 <.text+0x954344>
1409551b6: 48 8b 8d 10 01 00 00        	movq	0x110(%rbp), %rcx
1409551bd: 8b 15 d8 42 14 02           	movl	0x21442d8(%rip), %edx   # 0x142a9949b
1409551c3: 89 50 0f                    	movl	%edx, 0xf(%rax)
1409551c6: 0f 10 05 bf 42 14 02        	movups	0x21442bf(%rip), %xmm0  # 0x142a9948c
1409551cd: 0f 11 00                    	movups	%xmm0, (%rax)
1409551d0: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409551d4: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409551d8: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409551dc: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1409551e0: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
1409551e4: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
1409551e8: 48 c7 45 2c 00 00 00 00     	movq	$0x0, 0x2c(%rbp)
1409551f0: 48 c7 45 31 00 00 00 00     	movq	$0x0, 0x31(%rbp)
1409551f8: 0f 28 05 c1 8e fe 01        	movaps	0x1fe8ec1(%rip), %xmm0  # 0x14293e0c0
1409551ff: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140955203: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
140955209: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
140955213: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
140955217: c7 45 00 00 00 00 00        	movl	$0x0, (%rbp)
14095521e: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
140955225: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
14095522d: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140955231: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
140955239: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
140955240: 66 c7 45 39 00 00           	movw	$0x0, 0x39(%rbp)
140955246: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
14095524b: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14095524f: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140955253: 4c 8b 45 78                 	movq	0x78(%rbp), %r8
140955257: 4c 8b 4d 70                 	movq	0x70(%rbp), %r9
14095525b: e8 f0 8a f4 ff              	callq	0x14089dd50 <.text+0x89cd50>
140955260: 48 8b 5d 48                 	movq	0x48(%rbp), %rbx
140955264: 48 85 db                    	testq	%rbx, %rbx
140955267: 75 04                       	jne	0x14095526d <.text+0x95426d>
140955269: 48 8b 5d 50                 	movq	0x50(%rbp), %rbx
14095526d: f3 0f 10 77 18              	movss	0x18(%rdi), %xmm6
140955272: 4c 8b 75 60                 	movq	0x60(%rbp), %r14
140955276: 48 8b 03                    	movq	(%rbx), %rax
140955279: 48 8b 7b 10                 	movq	0x10(%rbx), %rdi
14095527d: 48 29 f8                    	subq	%rdi, %rax
140955280: 48 83 f8 27                 	cmpq	$0x27, %rax
140955284: 76 76                       	jbe	0x1409552fc <.text+0x9542fc>
140955286: 48 8b 43 08                 	movq	0x8(%rbx), %rax
14095528a: 48 8d 0d 5f 76 98 00        	leaq	0x98765f(%rip), %rcx    # 0x1412dc8f0 <.text+0x12db8f0>
140955291: 48 89 0c 38                 	movq	%rcx, (%rax,%rdi)
140955295: 4c 89 74 38 08              	movq	%r14, 0x8(%rax,%rdi)
14095529a: c7 44 38 10 01 00 00 00     	movl	$0x1, 0x10(%rax,%rdi)
1409552a2: f3 0f 11 74 38 14           	movss	%xmm6, 0x14(%rax,%rdi)
1409552a8: c6 44 38 18 00              	movb	$0x0, 0x18(%rax,%rdi)
1409552ad: 48 8d 0d 7c fa c0 00        	leaq	0xc0fa7c(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409552b4: 48 89 4c 38 20              	movq	%rcx, 0x20(%rax,%rdi)
1409552b9: 48 83 c7 28                 	addq	$0x28, %rdi
1409552bd: 48 89 7b 10                 	movq	%rdi, 0x10(%rbx)
1409552c1: 48 8b 85 80 00 00 00        	movq	0x80(%rbp), %rax
1409552c8: 4c 89 30                    	movq	%r14, (%rax)
1409552cb: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
1409552d2: 48 89 46 08                 	movq	%rax, 0x8(%rsi)
1409552d6: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
1409552de: 48 89 f0                    	movq	%rsi, %rax
1409552e1: 0f 28 b5 90 00 00 00        	movaps	0x90(%rbp), %xmm6
1409552e8: 48 81 c4 28 01 00 00        	addq	$0x128, %rsp            # imm = 0x128
1409552ef: 5b                          	popq	%rbx
1409552f0: 5f                          	popq	%rdi
1409552f1: 5e                          	popq	%rsi
1409552f2: 41 5c                       	popq	%r12
1409552f4: 41 5d                       	popq	%r13
1409552f6: 41 5e                       	popq	%r14
1409552f8: 41 5f                       	popq	%r15
1409552fa: 5d                          	popq	%rbp
1409552fb: c3                          	retq
1409552fc: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140955305: 41 b8 28 00 00 00           	movl	$0x28, %r8d
14095530b: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140955311: 48 89 d9                    	movq	%rbx, %rcx
140955314: 48 89 fa                    	movq	%rdi, %rdx
140955317: e8 44 47 f5 01              	callq	0x1428a9a60 <.text+0x28a8a60>
14095531c: e9 65 ff ff ff              	jmp	0x140955286 <.text+0x954286>
140955321: 4c 8d 05 50 38 2e 02        	leaq	0x22e3850(%rip), %r8    # 0x142c38b78
140955328: 48 89 d9                    	movq	%rbx, %rcx
14095532b: 4c 89 e2                    	movq	%r12, %rdx
14095532e: e8 b0 3c f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140955333: eb 25                       	jmp	0x14095535a <.text+0x95435a>
140955335: b9 08 00 00 00              	movl	$0x8, %ecx
14095533a: ba 08 00 00 00              	movl	$0x8, %edx
14095533f: e8 bc 3c f8 01              	callq	0x1428d9000 <.text+0x28d8000>
140955344: 4c 8d 05 d5 3b 14 02        	leaq	0x2143bd5(%rip), %r8    # 0x142a98f20
14095534b: b9 01 00 00 00              	movl	$0x1, %ecx
140955350: ba 13 00 00 00              	movl	$0x13, %edx
140955355: e8 89 3c f8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
