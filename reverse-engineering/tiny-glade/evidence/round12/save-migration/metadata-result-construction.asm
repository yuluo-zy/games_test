
D:\game\reverse-engineering\tiny-glade\workspace\input\tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140d6a960: 55                          	pushq	%rbp
140d6a961: 41 57                       	pushq	%r15
140d6a963: 41 56                       	pushq	%r14
140d6a965: 41 55                       	pushq	%r13
140d6a967: 41 54                       	pushq	%r12
140d6a969: 56                          	pushq	%rsi
140d6a96a: 57                          	pushq	%rdi
140d6a96b: 53                          	pushq	%rbx
140d6a96c: 48 81 ec f8 02 00 00        	subq	$0x2f8, %rsp            # imm = 0x2F8
140d6a973: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140d6a97b: 48 c7 85 70 02 00 00 fe ff ff ff    	movq	$-0x2, 0x270(%rbp)
140d6a986: 44 89 cf                    	movl	%r9d, %edi
140d6a989: 4d 89 c6                    	movq	%r8, %r14
140d6a98c: 49 89 d7                    	movq	%rdx, %r15
140d6a98f: 48 89 ce                    	movq	%rcx, %rsi
140d6a992: 48 b8 00 00 00 00 07 00 00 00       	movabsq	$0x700000000, %rax # imm = 0x700000000
140d6a99c: 48 89 85 1c 02 00 00        	movq	%rax, 0x21c(%rbp)
140d6a9a3: 66 c7 85 2c 02 00 00 00 00  	movw	$0x0, 0x22c(%rbp)
140d6a9ac: 48 c7 85 24 02 00 00 00 00 00 00    	movq	$0x0, 0x224(%rbp)
140d6a9b7: 48 c7 85 10 02 00 00 00 00 00 00    	movq	$0x0, 0x210(%rbp)
140d6a9c2: 48 c7 85 08 02 00 00 01 00 00 00    	movq	$0x1, 0x208(%rbp)
140d6a9cd: 44 89 c8                    	movl	%r9d, %eax
140d6a9d0: 0d 00 00 00 02              	orl	$0x2000000, %eax        # imm = 0x2000000
140d6a9d5: 89 85 18 02 00 00           	movl	%eax, 0x218(%rbp)
140d6a9db: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
140d6a9e2: e8 19 a6 00 00              	callq	0x140d75000 <.text+0xd74000>
140d6a9e7: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
140d6a9ee: 4c 8b a5 58 02 00 00        	movq	0x258(%rbp), %r12
140d6a9f5: 48 89 c1                    	movq	%rax, %rcx
140d6a9f8: 48 f7 d9                    	negq	%rcx
140d6a9fb: 70 40                       	jo	0x140d6aa3d <.text+0xd69a3d>
140d6a9fd: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140d6aa04: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140d6aa08: 4c 89 65 c0                 	movq	%r12, -0x40(%rbp)
140d6aa0c: 48 89 4d c8                 	movq	%rcx, -0x38(%rbp)
140d6aa10: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
140d6aa17: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140d6aa1b: 41 b0 01                    	movb	$0x1, %r8b
140d6aa1e: e8 2d d2 00 00              	callq	0x140d77c50 <.text+0xd76c50>
140d6aa23: 4c 8b ad 30 02 00 00        	movq	0x230(%rbp), %r13
140d6aa2a: 4c 8b a5 38 02 00 00        	movq	0x238(%rbp), %r12
140d6aa31: 4c 89 e8                    	movq	%r13, %rax
140d6aa34: 48 f7 d8                    	negq	%rax
140d6aa37: 0f 81 9c 00 00 00           	jno	0x140d6aad9 <.text+0xd69ad9>
140d6aa3d: 4c 89 e3                    	movq	%r12, %rbx
140d6aa40: 89 d8                       	movl	%ebx, %eax
140d6aa42: 83 e0 03                    	andl	$0x3, %eax
140d6aa45: 83 f8 02                    	cmpl	$0x2, %eax
140d6aa48: 0f 85 d8 00 00 00           	jne	0x140d6ab26 <.text+0xd69b26>
140d6aa4e: 48 89 d8                    	movq	%rbx, %rax
140d6aa51: 48 c1 e8 20                 	shrq	$0x20, %rax
140d6aa55: 48 83 f8 05                 	cmpq	$0x5, %rax
140d6aa59: 74 09                       	je	0x140d6aa64 <.text+0xd69a64>
140d6aa5b: 83 f8 20                    	cmpl	$0x20, %eax
140d6aa5e: 0f 85 c2 00 00 00           	jne	0x140d6ab26 <.text+0xd69b26>
140d6aa64: 48 89 9d 48 02 00 00        	movq	%rbx, 0x248(%rbp)
140d6aa6b: 48 8d 8d 50 02 00 00        	leaq	0x250(%rbp), %rcx
140d6aa72: 4c 89 fa                    	movq	%r15, %rdx
140d6aa75: 4d 89 f0                    	movq	%r14, %r8
140d6aa78: e8 83 a5 00 00              	callq	0x140d75000 <.text+0xd74000>
140d6aa7d: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
140d6aa84: 4c 8b b5 58 02 00 00        	movq	0x258(%rbp), %r14
140d6aa8b: 48 89 c1                    	movq	%rax, %rcx
140d6aa8e: 48 f7 d9                    	negq	%rcx
140d6aa91: 70 40                       	jo	0x140d6aad3 <.text+0xd69ad3>
140d6aa93: 48 8b 8d 60 02 00 00        	movq	0x260(%rbp), %rcx
140d6aa9a: 48 89 45 b8                 	movq	%rax, -0x48(%rbp)
140d6aa9e: 4c 89 75 c0                 	movq	%r14, -0x40(%rbp)
140d6aaa2: 48 89 4d c8                 	movq	%rcx, -0x38(%rbp)
140d6aaa6: 48 8d 8d 30 02 00 00        	leaq	0x230(%rbp), %rcx
140d6aaad: 48 8d 55 b8                 	leaq	-0x48(%rbp), %rdx
140d6aab1: 41 b0 01                    	movb	$0x1, %r8b
140d6aab4: e8 97 d1 00 00              	callq	0x140d77c50 <.text+0xd76c50>
140d6aab9: 4c 8b bd 30 02 00 00        	movq	0x230(%rbp), %r15
140d6aac0: 4c 8b b5 38 02 00 00        	movq	0x238(%rbp), %r14
140d6aac7: 4c 89 f8                    	movq	%r15, %rax
140d6aaca: 48 f7 d8                    	negq	%rax
140d6aacd: 0f 81 88 00 00 00           	jno	0x140d6ab5b <.text+0xd69b5b>
140d6aad3: 4c 89 76 08                 	movq	%r14, 0x8(%rsi)
140d6aad7: eb 51                       	jmp	0x140d6ab2a <.text+0xd69b2a>
140d6aad9: 89 bd 6c 02 00 00           	movl	%edi, 0x26c(%rbp)
140d6aadf: 48 8d 95 08 02 00 00        	leaq	0x208(%rbp), %rdx
140d6aae6: 4c 89 e1                    	movq	%r12, %rcx
140d6aae9: e8 f2 db ff ff              	callq	0x140d686e0 <.text+0xd676e0>
140d6aaee: 48 89 c7                    	movq	%rax, %rdi
140d6aaf1: 48 89 d3                    	movq	%rdx, %rbx
140d6aaf4: 4d 85 ed                    	testq	%r13, %r13
140d6aaf7: 74 14                       	je	0x140d6ab0d <.text+0xd69b0d>
140d6aaf9: 4d 01 ed                    	addq	%r13, %r13
140d6aafc: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140d6ab02: 4c 89 e1                    	movq	%r12, %rcx
140d6ab05: 4c 89 ea                    	movq	%r13, %rdx
140d6ab08: e8 13 91 8a ff              	callq	0x140613c20 <.text+0x612c20>
140d6ab0d: 48 85 ff                    	testq	%rdi, %rdi
140d6ab10: 74 33                       	je	0x140d6ab45 <.text+0xd69b45>
140d6ab12: 8b bd 6c 02 00 00           	movl	0x26c(%rbp), %edi
140d6ab18: 89 d8                       	movl	%ebx, %eax
140d6ab1a: 83 e0 03                    	andl	$0x3, %eax
140d6ab1d: 83 f8 02                    	cmpl	$0x2, %eax
140d6ab20: 0f 84 28 ff ff ff           	je	0x140d6aa4e <.text+0xd69a4e>
140d6ab26: 48 89 5e 08                 	movq	%rbx, 0x8(%rsi)
140d6ab2a: 48 c7 06 02 00 00 00        	movq	$0x2, (%rsi)
140d6ab31: 48 81 c4 f8 02 00 00        	addq	$0x2f8, %rsp            # imm = 0x2F8
140d6ab38: 5b                          	popq	%rbx
140d6ab39: 5f                          	popq	%rdi
140d6ab3a: 5e                          	popq	%rsi
140d6ab3b: 41 5c                       	popq	%r12
140d6ab3d: 41 5d                       	popq	%r13
140d6ab3f: 41 5e                       	popq	%r14
140d6ab41: 41 5f                       	popq	%r15
140d6ab43: 5d                          	popq	%rbp
140d6ab44: c3                          	retq
140d6ab45: 48 89 f1                    	movq	%rsi, %rcx
140d6ab48: 48 89 da                    	movq	%rbx, %rdx
140d6ab4b: e8 80 dd ff ff              	callq	0x140d688d0 <.text+0xd678d0>
140d6ab50: 48 89 d9                    	movq	%rbx, %rcx
140d6ab53: ff 15 97 78 42 02           	callq	*0x2427897(%rip)        # 0x1431923f0
140d6ab59: eb d6                       	jmp	0x140d6ab31 <.text+0xd69b31>
140d6ab5b: 41 89 fc                    	movl	%edi, %r12d
140d6ab5e: 48 8d 7d b8                 	leaq	-0x48(%rbp), %rdi
140d6ab62: 41 b8 50 02 00 00           	movl	$0x250, %r8d            # imm = 0x250
140d6ab68: 48 89 f9                    	movq	%rdi, %rcx
140d6ab6b: 31 d2                       	xorl	%edx, %edx
140d6ab6d: e8 ae 90 bb 01              	callq	0x142923c20 <.text+0x2922c20>
140d6ab72: c7 44 24 28 00 00 00 00     	movl	$0x0, 0x28(%rsp)
140d6ab7a: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
140d6ab83: 4c 89 f1                    	movq	%r14, %rcx
140d6ab86: ba 01 00 00 00              	movl	$0x1, %edx
140d6ab8b: 49 89 f8                    	movq	%rdi, %r8
140d6ab8e: 45 31 c9                    	xorl	%r9d, %r9d
140d6ab91: ff 15 f1 78 42 02           	callq	*0x24278f1(%rip)        # 0x143192488
140d6ab97: 48 83 f8 ff                 	cmpq	$-0x1, %rax
140d6ab9b: 74 43                       	je	0x140d6abe0 <.text+0xd69be0>
140d6ab9d: 48 89 c1                    	movq	%rax, %rcx
140d6aba0: ff 15 da 78 42 02           	callq	*0x24278da(%rip)        # 0x143192480
140d6aba6: 8b 45 b8                    	movl	-0x48(%rbp), %eax
140d6aba9: 0f 10 45 bc                 	movups	-0x44(%rbp), %xmm0
140d6abad: f2 0f 10 4d cc              	movsd	-0x34(%rbp), %xmm1
140d6abb2: 44 8b 45 d8                 	movl	-0x28(%rbp), %r8d
140d6abb6: 8b 55 d4                    	movl	-0x2c(%rbp), %edx
140d6abb9: 89 c1                       	movl	%eax, %ecx
140d6abbb: c1 e1 15                    	shll	$0x15, %ecx
140d6abbe: c1 f9 1f                    	sarl	$0x1f, %ecx
140d6abc1: 23 4d dc                    	andl	-0x24(%rbp), %ecx
140d6abc4: 45 85 e4                    	testl	%r12d, %r12d
140d6abc7: 75 24                       	jne	0x140d6abed <.text+0xd69bed>
140d6abc9: 41 89 c1                    	movl	%eax, %r9d
140d6abcc: 41 c1 e9 0a                 	shrl	$0xa, %r9d
140d6abd0: 41 89 ca                    	movl	%ecx, %r10d
140d6abd3: 41 c1 ea 1d                 	shrl	$0x1d, %r10d
140d6abd7: 45 21 ca                    	andl	%r9d, %r10d
140d6abda: 41 f6 c2 01                 	testb	$0x1, %r10b
140d6abde: 74 0d                       	je	0x140d6abed <.text+0xd69bed>
140d6abe0: 48 89 5e 08                 	movq	%rbx, 0x8(%rsi)
140d6abe4: 48 c7 06 02 00 00 00        	movq	$0x2, (%rsi)
140d6abeb: eb 35                       	jmp	0x140d6ac22 <.text+0xd69c22>
140d6abed: 48 c1 e2 20                 	shlq	$0x20, %rdx
140d6abf1: 4c 09 c2                    	orq	%r8, %rdx
140d6abf4: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
140d6abfb: c7 46 10 00 00 00 00        	movl	$0x0, 0x10(%rsi)
140d6ac02: c7 46 18 00 00 00 00        	movl	$0x0, 0x18(%rsi)
140d6ac09: 0f 11 46 20                 	movups	%xmm0, 0x20(%rsi)
140d6ac0d: 0f 13 4e 30                 	movlps	%xmm1, 0x30(%rsi)
140d6ac11: 48 89 56 38                 	movq	%rdx, 0x38(%rsi)
140d6ac15: c7 46 40 00 00 00 00        	movl	$0x0, 0x40(%rsi)
140d6ac1c: 89 46 4c                    	movl	%eax, 0x4c(%rsi)
140d6ac1f: 89 4e 50                    	movl	%ecx, 0x50(%rsi)
140d6ac22: 4d 85 ff                    	testq	%r15, %r15
140d6ac25: 0f 84 06 ff ff ff           	je	0x140d6ab31 <.text+0xd69b31>
140d6ac2b: 4d 01 ff                    	addq	%r15, %r15
140d6ac2e: 41 b8 02 00 00 00           	movl	$0x2, %r8d
140d6ac34: 4c 89 f1                    	movq	%r14, %rcx
140d6ac37: 4c 89 fa                    	movq	%r15, %rdx
140d6ac3a: e8 e1 8f 8a ff              	callq	0x140613c20 <.text+0x612c20>
140d6ac3f: e9 ed fe ff ff              	jmp	0x140d6ab31 <.text+0xd69b31>
140d6ac44: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
140d6ac50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140d6ac55: 55                          	pushq	%rbp
140d6ac56: 41 57                       	pushq	%r15
140d6ac58: 41 56                       	pushq	%r14
140d6ac5a: 41 55                       	pushq	%r13
140d6ac5c: 41 54                       	pushq	%r12
140d6ac5e: 56                          	pushq	%rsi
140d6ac5f: 57                          	pushq	%rdi
140d6ac60: 53                          	pushq	%rbx
140d6ac61: 48 83 ec 38                 	subq	$0x38, %rsp
140d6ac65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140d6ac6c: 48 8d 8d 48 02 00 00        	leaq	0x248(%rbp), %rcx
140d6ac73: e8 88 cd 87 ff              	callq	0x1405e7a00 <.text+0x5e6a00>
140d6ac78: 90                          	nop
140d6ac79: 48 83 c4 38                 	addq	$0x38, %rsp
140d6ac7d: 5b                          	popq	%rbx
140d6ac7e: 5f                          	popq	%rdi
140d6ac7f: 5e                          	popq	%rsi
140d6ac80: 41 5c                       	popq	%r12
140d6ac82: 41 5d                       	popq	%r13
140d6ac84: 41 5e                       	popq	%r14
140d6ac86: 41 5f                       	popq	%r15
140d6ac88: 5d                          	popq	%rbp
140d6ac89: c3                          	retq
