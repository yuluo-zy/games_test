
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a647e0: 55                          	pushq	%rbp
140a647e1: 41 57                       	pushq	%r15
140a647e3: 41 56                       	pushq	%r14
140a647e5: 56                          	pushq	%rsi
140a647e6: 57                          	pushq	%rdi
140a647e7: 53                          	pushq	%rbx
140a647e8: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140a647ef: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a647f7: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
140a64802: 4c 89 cf                    	movq	%r9, %rdi
140a64805: 4c 89 c3                    	movq	%r8, %rbx
140a64808: 49 89 d6                    	movq	%rdx, %r14
140a6480b: 48 89 ce                    	movq	%rcx, %rsi
140a6480e: 0f b6 05 f4 03 bd 02        	movzbl	0x2bd03f4(%rip), %eax   # 0x143634c09
140a64815: b9 08 00 00 00              	movl	$0x8, %ecx
140a6481a: ba 08 00 00 00              	movl	$0x8, %edx
140a6481f: e8 ec f3 ba ff              	callq	0x140613c10 <.text+0x612c10>
140a64824: 48 85 c0                    	testq	%rax, %rax
140a64827: 0f 84 61 01 00 00           	je	0x140a6498e <.text+0xa6398e>
140a6482d: 49 89 c7                    	movq	%rax, %r15
140a64830: 0f b6 05 d2 03 bd 02        	movzbl	0x2bd03d2(%rip), %eax   # 0x143634c09
140a64837: b9 0b 00 00 00              	movl	$0xb, %ecx
140a6483c: ba 01 00 00 00              	movl	$0x1, %edx
140a64841: e8 ca f3 ba ff              	callq	0x140613c10 <.text+0x612c10>
140a64846: 48 85 c0                    	testq	%rax, %rax
140a64849: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
140a6484d: 0f 84 4a 01 00 00           	je	0x140a6499d <.text+0xa6399d>
140a64853: 48 b9 74 65 6d 70 5f 73 70 68       	movabsq	$0x6870735f706d6574, %rcx # imm = 0x6870735F706D6574
140a6485d: 48 89 08                    	movq	%rcx, (%rax)
140a64860: c7 40 07 68 65 72 65        	movl	$0x65726568, 0x7(%rax)  # imm = 0x65726568
140a64867: 48 c7 45 50 0b 00 00 00     	movq	$0xb, 0x50(%rbp)
140a6486f: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140a64873: 48 c7 45 60 0b 00 00 00     	movq	$0xb, 0x60(%rbp)
140a6487b: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a64885: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140a64889: 0f b6 05 79 03 bd 02        	movzbl	0x2bd0379(%rip), %eax   # 0x143634c09
140a64890: b9 13 00 00 00              	movl	$0x13, %ecx
140a64895: ba 01 00 00 00              	movl	$0x1, %edx
140a6489a: e8 71 f3 ba ff              	callq	0x140613c10 <.text+0x612c10>
140a6489f: 48 85 c0                    	testq	%rax, %rax
140a648a2: 0f 84 0d 01 00 00           	je	0x140a649b5 <.text+0xa639b5>
140a648a8: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140a648af: 8b 15 e6 4b 03 02           	movl	0x2034be6(%rip), %edx   # 0x142a9949b
140a648b5: 89 50 0f                    	movl	%edx, 0xf(%rax)
140a648b8: 0f 10 05 cd 4b 03 02        	movups	0x2034bcd(%rip), %xmm0  # 0x142a9948c
140a648bf: 0f 11 00                    	movups	%xmm0, (%rax)
140a648c2: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
140a648c6: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
140a648ca: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
140a648ce: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140a648d2: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
140a648d6: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
140a648da: f3 41 0f 10 46 0c           	movss	0xc(%r14), %xmm0
140a648e0: 0f 28 0d d9 97 ed 01        	movaps	0x1ed97d9(%rip), %xmm1  # 0x14293e0c0
140a648e7: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140a648eb: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
140a648f1: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
140a648fb: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
140a648ff: c7 45 00 01 00 00 00        	movl	$0x1, (%rbp)
140a64906: f3 0f 11 45 04              	movss	%xmm0, 0x4(%rbp)
140a6490b: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
140a64912: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
140a6491a: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140a6491e: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
140a64926: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
140a6492d: 49 8b 06                    	movq	(%r14), %rax
140a64930: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
140a64934: 41 8b 46 08                 	movl	0x8(%r14), %eax
140a64938: 89 45 34                    	movl	%eax, 0x34(%rbp)
140a6493b: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
140a64941: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
140a64945: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140a6494a: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140a6494e: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140a64952: 49 89 d8                    	movq	%rbx, %r8
140a64955: 49 89 f9                    	movq	%rdi, %r9
140a64958: e8 f3 93 e3 ff              	callq	0x14089dd50 <.text+0x89cd50>
140a6495d: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140a64961: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140a64965: 48 89 01                    	movq	%rax, (%rcx)
140a64968: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140a6496f: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
140a64973: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
140a6497b: 48 89 f0                    	movq	%rsi, %rax
140a6497e: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
140a64985: 5b                          	popq	%rbx
140a64986: 5f                          	popq	%rdi
140a64987: 5e                          	popq	%rsi
140a64988: 41 5e                       	popq	%r14
140a6498a: 41 5f                       	popq	%r15
140a6498c: 5d                          	popq	%rbp
140a6498d: c3                          	retq
140a6498e: b9 08 00 00 00              	movl	$0x8, %ecx
140a64993: ba 08 00 00 00              	movl	$0x8, %edx
140a64998: e8 63 46 e7 01              	callq	0x1428d9000 <.text+0x28d8000>
140a6499d: 4c 8d 05 d4 f1 06 02        	leaq	0x206f1d4(%rip), %r8    # 0x142ad3b78
140a649a4: b9 01 00 00 00              	movl	$0x1, %ecx
140a649a9: ba 0b 00 00 00              	movl	$0xb, %edx
140a649ae: e8 30 46 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a649b3: eb 16                       	jmp	0x140a649cb <.text+0xa639cb>
140a649b5: 4c 8d 05 64 45 03 02        	leaq	0x2034564(%rip), %r8    # 0x142a98f20
140a649bc: b9 01 00 00 00              	movl	$0x1, %ecx
140a649c1: ba 13 00 00 00              	movl	$0x13, %edx
140a649c6: e8 18 46 e7 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a649cb: 0f 0b                       	ud2
140a649cd: 0f 1f 00                    	nopl	(%rax)
140a649d0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a649d5: 55                          	pushq	%rbp
140a649d6: 41 57                       	pushq	%r15
140a649d8: 41 56                       	pushq	%r14
140a649da: 56                          	pushq	%rsi
140a649db: 57                          	pushq	%rdi
140a649dc: 53                          	pushq	%rbx
140a649dd: 48 83 ec 28                 	subq	$0x28, %rsp
140a649e1: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a649e8: ba 08 00 00 00              	movl	$0x8, %edx
140a649ed: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140a649f3: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140a649f7: e8 24 f2 ba ff              	callq	0x140613c20 <.text+0x612c20>
140a649fc: 90                          	nop
140a649fd: 48 83 c4 28                 	addq	$0x28, %rsp
140a64a01: 5b                          	popq	%rbx
140a64a02: 5f                          	popq	%rdi
140a64a03: 5e                          	popq	%rsi
140a64a04: 41 5e                       	popq	%r14
140a64a06: 41 5f                       	popq	%r15
140a64a08: 5d                          	popq	%rbp
140a64a09: c3                          	retq
140a64a0a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
140a64a10: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a64a15: 55                          	pushq	%rbp
140a64a16: 41 57                       	pushq	%r15
140a64a18: 41 56                       	pushq	%r14
140a64a1a: 56                          	pushq	%rsi
140a64a1b: 57                          	pushq	%rdi
140a64a1c: 53                          	pushq	%rbx
140a64a1d: 48 83 ec 28                 	subq	$0x28, %rsp
140a64a21: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a64a28: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140a64a2c: e8 6f 63 e3 ff              	callq	0x14089ada0 <.text+0x899da0>
140a64a31: 90                          	nop
140a64a32: 48 83 c4 28                 	addq	$0x28, %rsp
140a64a36: 5b                          	popq	%rbx
140a64a37: 5f                          	popq	%rdi
140a64a38: 5e                          	popq	%rsi
140a64a39: 41 5e                       	popq	%r14
140a64a3b: 41 5f                       	popq	%r15
140a64a3d: 5d                          	popq	%rbp
140a64a3e: c3                          	retq
