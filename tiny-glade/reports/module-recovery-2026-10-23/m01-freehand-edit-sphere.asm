
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140acc850: 55                          	pushq	%rbp
140acc851: 41 57                       	pushq	%r15
140acc853: 41 56                       	pushq	%r14
140acc855: 56                          	pushq	%rsi
140acc856: 57                          	pushq	%rdi
140acc857: 53                          	pushq	%rbx
140acc858: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
140acc85f: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140acc867: 48 c7 85 80 00 00 00 fe ff ff ff    	movq	$-0x2, 0x80(%rbp)
140acc872: 4c 89 cf                    	movq	%r9, %rdi
140acc875: 4c 89 c3                    	movq	%r8, %rbx
140acc878: 49 89 d6                    	movq	%rdx, %r14
140acc87b: 48 89 ce                    	movq	%rcx, %rsi
140acc87e: 0f b6 05 84 83 b6 02        	movzbl	0x2b68384(%rip), %eax   # 0x143634c09
140acc885: b9 08 00 00 00              	movl	$0x8, %ecx
140acc88a: ba 08 00 00 00              	movl	$0x8, %edx
140acc88f: e8 7c 73 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acc894: 48 85 c0                    	testq	%rax, %rax
140acc897: 0f 84 61 01 00 00           	je	0x140acc9fe <.text+0xacb9fe>
140acc89d: 49 89 c7                    	movq	%rax, %r15
140acc8a0: 0f b6 05 62 83 b6 02        	movzbl	0x2b68362(%rip), %eax   # 0x143634c09
140acc8a7: b9 0b 00 00 00              	movl	$0xb, %ecx
140acc8ac: ba 01 00 00 00              	movl	$0x1, %edx
140acc8b1: e8 5a 73 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acc8b6: 48 85 c0                    	testq	%rax, %rax
140acc8b9: 4c 89 7d 78                 	movq	%r15, 0x78(%rbp)
140acc8bd: 0f 84 4a 01 00 00           	je	0x140acca0d <.text+0xacba0d>
140acc8c3: 48 b9 74 65 6d 70 5f 73 70 68       	movabsq	$0x6870735f706d6574, %rcx # imm = 0x6870735F706D6574
140acc8cd: 48 89 08                    	movq	%rcx, (%rax)
140acc8d0: c7 40 07 68 65 72 65        	movl	$0x65726568, 0x7(%rax)  # imm = 0x65726568
140acc8d7: 48 c7 45 50 0b 00 00 00     	movq	$0xb, 0x50(%rbp)
140acc8df: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140acc8e3: 48 c7 45 60 0b 00 00 00     	movq	$0xb, 0x60(%rbp)
140acc8eb: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140acc8f5: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140acc8f9: 0f b6 05 09 83 b6 02        	movzbl	0x2b68309(%rip), %eax   # 0x143634c09
140acc900: b9 13 00 00 00              	movl	$0x13, %ecx
140acc905: ba 01 00 00 00              	movl	$0x1, %edx
140acc90a: e8 01 73 b4 ff              	callq	0x140613c10 <.text+0x612c10>
140acc90f: 48 85 c0                    	testq	%rax, %rax
140acc912: 0f 84 0d 01 00 00           	je	0x140acca25 <.text+0xacba25>
140acc918: 48 8b 8d e0 00 00 00        	movq	0xe0(%rbp), %rcx
140acc91f: 8b 15 76 cb fc 01           	movl	0x1fccb76(%rip), %edx   # 0x142a9949b
140acc925: 89 50 0f                    	movl	%edx, 0xf(%rax)
140acc928: 0f 10 05 5d cb fc 01        	movups	0x1fccb5d(%rip), %xmm0  # 0x142a9948c
140acc92f: 0f 11 00                    	movups	%xmm0, (%rax)
140acc932: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
140acc936: 0f 10 4d 58                 	movups	0x58(%rbp), %xmm1
140acc93a: 0f 10 55 68                 	movups	0x68(%rbp), %xmm2
140acc93e: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
140acc942: 0f 29 4d d0                 	movaps	%xmm1, -0x30(%rbp)
140acc946: 0f 29 55 e0                 	movaps	%xmm2, -0x20(%rbp)
140acc94a: f3 41 0f 10 46 0c           	movss	0xc(%r14), %xmm0
140acc950: 0f 28 0d 69 17 e7 01        	movaps	0x1e71769(%rip), %xmm1  # 0x14293e0c0
140acc957: 0f 29 4d b0                 	movaps	%xmm1, -0x50(%rbp)
140acc95b: 66 c7 45 f0 00 01           	movw	$0x100, -0x10(%rbp)     # imm = 0x100
140acc961: 48 ba 00 00 80 3f 01 00 00 00       	movabsq	$0x13f800000, %rdx # imm = 0x13F800000
140acc96b: 48 89 55 f4                 	movq	%rdx, -0xc(%rbp)
140acc96f: c7 45 00 01 00 00 00        	movl	$0x1, (%rbp)
140acc976: f3 0f 11 45 04              	movss	%xmm0, 0x4(%rbp)
140acc97b: c7 45 08 00 00 00 00        	movl	$0x0, 0x8(%rbp)
140acc982: 48 c7 45 10 13 00 00 00     	movq	$0x13, 0x10(%rbp)
140acc98a: 48 89 45 18                 	movq	%rax, 0x18(%rbp)
140acc98e: 48 c7 45 20 13 00 00 00     	movq	$0x13, 0x20(%rbp)
140acc996: c7 45 28 00 00 00 00        	movl	$0x0, 0x28(%rbp)
140acc99d: 49 8b 06                    	movq	(%r14), %rax
140acc9a0: 48 89 45 2c                 	movq	%rax, 0x2c(%rbp)
140acc9a4: 41 8b 46 08                 	movl	0x8(%r14), %eax
140acc9a8: 89 45 34                    	movl	%eax, 0x34(%rbp)
140acc9ab: 66 c7 45 38 00 00           	movw	$0x0, 0x38(%rbp)
140acc9b1: c6 45 3a 00                 	movb	$0x0, 0x3a(%rbp)
140acc9b5: 48 89 4c 24 20              	movq	%rcx, 0x20(%rsp)
140acc9ba: 48 8d 4d 48                 	leaq	0x48(%rbp), %rcx
140acc9be: 48 8d 55 b0                 	leaq	-0x50(%rbp), %rdx
140acc9c2: 49 89 d8                    	movq	%rbx, %r8
140acc9c5: 49 89 f9                    	movq	%rdi, %r9
140acc9c8: e8 83 13 dd ff              	callq	0x14089dd50 <.text+0x89cd50>
140acc9cd: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140acc9d1: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
140acc9d5: 48 89 01                    	movq	%rax, (%rcx)
140acc9d8: 48 c7 06 01 00 00 00        	movq	$0x1, (%rsi)
140acc9df: 48 89 4e 08                 	movq	%rcx, 0x8(%rsi)
140acc9e3: 48 c7 46 10 01 00 00 00     	movq	$0x1, 0x10(%rsi)
140acc9eb: 48 89 f0                    	movq	%rsi, %rax
140acc9ee: 48 81 c4 08 01 00 00        	addq	$0x108, %rsp            # imm = 0x108
140acc9f5: 5b                          	popq	%rbx
140acc9f6: 5f                          	popq	%rdi
140acc9f7: 5e                          	popq	%rsi
140acc9f8: 41 5e                       	popq	%r14
140acc9fa: 41 5f                       	popq	%r15
140acc9fc: 5d                          	popq	%rbp
140acc9fd: c3                          	retq
140acc9fe: b9 08 00 00 00              	movl	$0x8, %ecx
140acca03: ba 08 00 00 00              	movl	$0x8, %edx
140acca08: e8 f3 c5 e0 01              	callq	0x1428d9000 <.text+0x28d8000>
140acca0d: 4c 8d 05 ac 3c 01 02        	leaq	0x2013cac(%rip), %r8    # 0x142ae06c0
140acca14: b9 01 00 00 00              	movl	$0x1, %ecx
140acca19: ba 0b 00 00 00              	movl	$0xb, %edx
140acca1e: e8 c0 c5 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140acca23: eb 16                       	jmp	0x140acca3b <.text+0xacba3b>
140acca25: 4c 8d 05 f4 c4 fc 01        	leaq	0x1fcc4f4(%rip), %r8    # 0x142a98f20
140acca2c: b9 01 00 00 00              	movl	$0x1, %ecx
140acca31: ba 13 00 00 00              	movl	$0x13, %edx
140acca36: e8 a8 c5 e0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
