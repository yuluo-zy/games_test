
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1407c0910: 55                          	pushq	%rbp
1407c0911: 41 57                       	pushq	%r15
1407c0913: 41 56                       	pushq	%r14
1407c0915: 41 55                       	pushq	%r13
1407c0917: 41 54                       	pushq	%r12
1407c0919: 56                          	pushq	%rsi
1407c091a: 57                          	pushq	%rdi
1407c091b: 53                          	pushq	%rbx
1407c091c: 48 81 ec 98 03 00 00        	subq	$0x398, %rsp            # imm = 0x398
1407c0923: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1407c092b: 44 0f 29 bd 00 03 00 00     	movaps	%xmm15, 0x300(%rbp)
1407c0933: 44 0f 29 b5 f0 02 00 00     	movaps	%xmm14, 0x2f0(%rbp)
1407c093b: 44 0f 29 ad e0 02 00 00     	movaps	%xmm13, 0x2e0(%rbp)
1407c0943: 44 0f 29 a5 d0 02 00 00     	movaps	%xmm12, 0x2d0(%rbp)
1407c094b: 44 0f 29 9d c0 02 00 00     	movaps	%xmm11, 0x2c0(%rbp)
1407c0953: 44 0f 29 95 b0 02 00 00     	movaps	%xmm10, 0x2b0(%rbp)
1407c095b: 44 0f 29 8d a0 02 00 00     	movaps	%xmm9, 0x2a0(%rbp)
1407c0963: 44 0f 29 85 90 02 00 00     	movaps	%xmm8, 0x290(%rbp)
1407c096b: 0f 29 bd 80 02 00 00        	movaps	%xmm7, 0x280(%rbp)
1407c0972: 0f 29 b5 70 02 00 00        	movaps	%xmm6, 0x270(%rbp)
1407c0979: 48 c7 85 68 02 00 00 fe ff ff ff    	movq	$-0x2, 0x268(%rbp)
1407c0984: 4c 89 cb                    	movq	%r9, %rbx
1407c0987: 4c 89 c6                    	movq	%r8, %rsi
1407c098a: 48 89 d7                    	movq	%rdx, %rdi
1407c098d: 49 89 ce                    	movq	%rcx, %r14
1407c0990: e8 6b 60 45 00              	callq	0x140c16a00 <.text+0xc15a00>
1407c0995: 84 c0                       	testb	%al, %al
1407c0997: 74 5d                       	je	0x1407c09f6 <.text+0x7bf9f6>
1407c0999: 41 bf 48 00 00 00           	movl	$0x48, %r15d
1407c099f: b9 48 00 00 00              	movl	$0x48, %ecx
1407c09a4: e8 37 fb ff ff              	callq	0x1407c04e0 <.text+0x7bf4e0>
1407c09a9: 4c 8d 25 20 57 2c 02        	leaq	0x22c5720(%rip), %r12   # 0x142a860d0
1407c09b0: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c09b4: 0f 85 c1 00 00 00           	jne	0x1407c0a7b <.text+0x7bfa7b>
1407c09ba: 49 89 d1                    	movq	%rdx, %r9
1407c09bd: 48 85 d2                    	testq	%rdx, %rdx
1407c09c0: 74 56                       	je	0x1407c0a18 <.text+0x7bfa18>
1407c09c2: 49 83 f9 48                 	cmpq	$0x48, %r9
1407c09c6: 73 4e                       	jae	0x1407c0a16 <.text+0x7bfa16>
1407c09c8: 48 8d 05 01 57 2c 02        	leaq	0x22c5701(%rip), %rax   # 0x142a860d0
1407c09cf: 41 80 3c 01 bf              	cmpb	$-0x41, (%r9,%rax)
1407c09d4: 7f 42                       	jg	0x1407c0a18 <.text+0x7bfa18>
1407c09d6: 48 8d 05 c3 57 2c 02        	leaq	0x22c57c3(%rip), %rax   # 0x142a861a0
1407c09dd: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c09e2: 48 8d 0d e7 56 2c 02        	leaq	0x22c56e7(%rip), %rcx   # 0x142a860d0
1407c09e9: ba 48 00 00 00              	movl	$0x48, %edx
1407c09ee: 45 31 c0                    	xorl	%r8d, %r8d
1407c09f1: e8 4a 8f 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c09f6: 48 c7 85 08 02 00 00 00 00 00 00    	movq	$0x0, 0x208(%rbp)
1407c0a01: 48 8b 05 90 b6 e6 02        	movq	0x2e6b690(%rip), %rax   # 0x14362c098
1407c0a08: 80 38 00                    	cmpb	$0x0, (%rax)
1407c0a0b: 0f 85 42 02 00 00           	jne	0x1407c0c53 <.text+0x7bfc53>
1407c0a11: e9 9f 02 00 00              	jmp	0x1407c0cb5 <.text+0x7bfcb5>
1407c0a16: 75 be                       	jne	0x1407c09d6 <.text+0x7bf9d6>
1407c0a18: 4c 89 c9                    	movq	%r9, %rcx
1407c0a1b: e8 c0 fa ff ff              	callq	0x1407c04e0 <.text+0x7bf4e0>
1407c0a20: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c0a24: 75 55                       	jne	0x1407c0a7b <.text+0x7bfa7b>
1407c0a26: 49 89 d0                    	movq	%rdx, %r8
1407c0a29: 49 83 c0 02                 	addq	$0x2, %r8
1407c0a2d: 74 39                       	je	0x1407c0a68 <.text+0x7bfa68>
1407c0a2f: 49 83 f8 48                 	cmpq	$0x48, %r8
1407c0a33: 73 31                       	jae	0x1407c0a66 <.text+0x7bfa66>
1407c0a35: 48 8d 05 94 56 2c 02        	leaq	0x22c5694(%rip), %rax   # 0x142a860d0
1407c0a3c: 41 80 3c 00 bf              	cmpb	$-0x41, (%r8,%rax)
1407c0a41: 7f 25                       	jg	0x1407c0a68 <.text+0x7bfa68>
1407c0a43: 48 8d 05 6e 57 2c 02        	leaq	0x22c576e(%rip), %rax   # 0x142a861b8
1407c0a4a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c0a4f: 48 8d 0d 7a 56 2c 02        	leaq	0x22c567a(%rip), %rcx   # 0x142a860d0
1407c0a56: ba 48 00 00 00              	movl	$0x48, %edx
1407c0a5b: 41 b9 48 00 00 00           	movl	$0x48, %r9d
1407c0a61: e8 da 8e 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c0a66: 75 db                       	jne	0x1407c0a43 <.text+0x7bfa43>
1407c0a68: 41 bf 46 00 00 00           	movl	$0x46, %r15d
1407c0a6e: 49 29 d7                    	subq	%rdx, %r15
1407c0a71: 4c 8d 25 58 56 2c 02        	leaq	0x22c5658(%rip), %r12   # 0x142a860d0
1407c0a78: 4d 01 c4                    	addq	%r8, %r12
1407c0a7b: 4c 8d 05 a3 57 2c 02        	leaq	0x22c57a3(%rip), %r8    # 0x142a86225
1407c0a82: 4c 8d 2d 5f 57 2c 02        	leaq	0x22c575f(%rip), %r13   # 0x142a861e8
1407c0a89: 4c 89 b5 58 02 00 00        	movq	%r14, 0x258(%rbp)
1407c0a90: 41 0f be 40 ff              	movsbl	-0x1(%r8), %eax
1407c0a95: 85 c0                       	testl	%eax, %eax
1407c0a97: 78 17                       	js	0x1407c0ab0 <.text+0x7bfab0>
1407c0a99: 49 ff c8                    	decq	%r8
1407c0a9c: 83 f8 5c                    	cmpl	$0x5c, %eax
1407c0a9f: 75 63                       	jne	0x1407c0b04 <.text+0x7bfb04>
1407c0aa1: eb 7b                       	jmp	0x1407c0b1e <.text+0x7bfb1e>
1407c0aa3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1407c0ab0: 41 0f b6 48 fe              	movzbl	-0x2(%r8), %ecx
1407c0ab5: 80 f9 c0                    	cmpb	$-0x40, %cl
1407c0ab8: 7d 23                       	jge	0x1407c0add <.text+0x7bfadd>
1407c0aba: 41 0f b6 50 fd              	movzbl	-0x3(%r8), %edx
1407c0abf: 80 fa c0                    	cmpb	$-0x40, %dl
1407c0ac2: 7d 22                       	jge	0x1407c0ae6 <.text+0x7bfae6>
1407c0ac4: 45 0f b6 48 fc              	movzbl	-0x4(%r8), %r9d
1407c0ac9: 49 83 c0 fc                 	addq	$-0x4, %r8
1407c0acd: 41 83 e1 07                 	andl	$0x7, %r9d
1407c0ad1: 41 c1 e1 06                 	shll	$0x6, %r9d
1407c0ad5: 83 e2 3f                    	andl	$0x3f, %edx
1407c0ad8: 44 09 ca                    	orl	%r9d, %edx
1407c0adb: eb 10                       	jmp	0x1407c0aed <.text+0x7bfaed>
1407c0add: 49 83 c0 fe                 	addq	$-0x2, %r8
1407c0ae1: 83 e1 1f                    	andl	$0x1f, %ecx
1407c0ae4: eb 0f                       	jmp	0x1407c0af5 <.text+0x7bfaf5>
1407c0ae6: 49 83 c0 fd                 	addq	$-0x3, %r8
1407c0aea: 83 e2 0f                    	andl	$0xf, %edx
1407c0aed: c1 e2 06                    	shll	$0x6, %edx
1407c0af0: 83 e1 3f                    	andl	$0x3f, %ecx
1407c0af3: 09 d1                       	orl	%edx, %ecx
1407c0af5: c1 e1 06                    	shll	$0x6, %ecx
1407c0af8: 24 3f                       	andb	$0x3f, %al
1407c0afa: 0f b6 c0                    	movzbl	%al, %eax
1407c0afd: 09 c8                       	orl	%ecx, %eax
1407c0aff: 83 f8 5c                    	cmpl	$0x5c, %eax
1407c0b02: 74 1a                       	je	0x1407c0b1e <.text+0x7bfb1e>
1407c0b04: 83 f8 2f                    	cmpl	$0x2f, %eax
1407c0b07: 74 15                       	je	0x1407c0b1e <.text+0x7bfb1e>
1407c0b09: 4d 39 e8                    	cmpq	%r13, %r8
1407c0b0c: 75 82                       	jne	0x1407c0a90 <.text+0x7bfa90>
1407c0b0e: 49 89 fe                    	movq	%rdi, %r14
1407c0b11: 48 89 f7                    	movq	%rsi, %rdi
1407c0b14: 48 89 de                    	movq	%rbx, %rsi
1407c0b17: bb 3d 00 00 00              	movl	$0x3d, %ebx
1407c0b1c: eb 55                       	jmp	0x1407c0b73 <.text+0x7bfb73>
1407c0b1e: 4c 8d 2d c3 56 2c 02        	leaq	0x22c56c3(%rip), %r13   # 0x142a861e8
1407c0b25: 4d 29 e8                    	subq	%r13, %r8
1407c0b28: 49 ff c0                    	incq	%r8
1407c0b2b: 74 32                       	je	0x1407c0b5f <.text+0x7bfb5f>
1407c0b2d: 49 83 f8 3d                 	cmpq	$0x3d, %r8
1407c0b31: 73 2a                       	jae	0x1407c0b5d <.text+0x7bfb5d>
1407c0b33: 43 80 3c 28 bf              	cmpb	$-0x41, (%r8,%r13)
1407c0b38: 7f 25                       	jg	0x1407c0b5f <.text+0x7bfb5f>
1407c0b3a: 48 8d 05 3f 56 2c 02        	leaq	0x22c563f(%rip), %rax   # 0x142a86180
1407c0b41: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c0b46: 48 8d 0d 9b 56 2c 02        	leaq	0x22c569b(%rip), %rcx   # 0x142a861e8
1407c0b4d: ba 3d 00 00 00              	movl	$0x3d, %edx
1407c0b52: 41 b9 3d 00 00 00           	movl	$0x3d, %r9d
1407c0b58: e8 e3 8d 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c0b5d: 75 db                       	jne	0x1407c0b3a <.text+0x7bfb3a>
1407c0b5f: 49 89 fe                    	movq	%rdi, %r14
1407c0b62: 48 89 f7                    	movq	%rsi, %rdi
1407c0b65: 48 89 de                    	movq	%rbx, %rsi
1407c0b68: bb 3d 00 00 00              	movl	$0x3d, %ebx
1407c0b6d: 4c 29 c3                    	subq	%r8, %rbx
1407c0b70: 4d 01 c5                    	addq	%r8, %r13
1407c0b73: e8 b8 75 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c0b78: 48 89 c1                    	movq	%rax, %rcx
1407c0b7b: 48 8b 00                    	movq	(%rax), %rax
1407c0b7e: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c0b82: 0f 85 79 0c 00 00           	jne	0x1407c1801 <.text+0x7c0801>
1407c0b88: 48 83 c1 08                 	addq	$0x8, %rcx
1407c0b8c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c0b90: 0f 85 0f 0d 00 00           	jne	0x1407c18a5 <.text+0x7c08a5>
1407c0b96: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c0b9d: 48 89 8d 38 02 00 00        	movq	%rcx, 0x238(%rbp)
1407c0ba4: 48 83 c1 08                 	addq	$0x8, %rcx
1407c0ba8: 48 89 5c 24 20              	movq	%rbx, 0x20(%rsp)
1407c0bad: 48 c7 44 24 30 00 00 00 00  	movq	$0x0, 0x30(%rsp)
1407c0bb6: 48 c7 44 24 28 01 00 00 00  	movq	$0x1, 0x28(%rsp)
1407c0bbf: 4c 89 e2                    	movq	%r12, %rdx
1407c0bc2: 4d 89 f8                    	movq	%r15, %r8
1407c0bc5: 4d 89 e9                    	movq	%r13, %r9
1407c0bc8: e8 73 61 45 00              	callq	0x140c16d40 <.text+0xc15d40>
1407c0bcd: 49 89 c7                    	movq	%rax, %r15
1407c0bd0: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
1407c0bd7: 48 ff 00                    	incq	(%rax)
1407c0bda: 4c 89 bd 10 02 00 00        	movq	%r15, 0x210(%rbp)
1407c0be1: 48 c7 85 08 02 00 00 01 00 00 00    	movq	$0x1, 0x208(%rbp)
1407c0bec: 48 8b 05 a5 b4 e6 02        	movq	0x2e6b4a5(%rip), %rax   # 0x14362c098
1407c0bf3: 80 38 00                    	cmpb	$0x0, (%rax)
1407c0bf6: 48 89 f3                    	movq	%rsi, %rbx
1407c0bf9: 48 89 fe                    	movq	%rdi, %rsi
1407c0bfc: 4c 89 f7                    	movq	%r14, %rdi
1407c0bff: 4c 8b b5 58 02 00 00        	movq	0x258(%rbp), %r14
1407c0c06: 0f 84 a9 00 00 00           	je	0x1407c0cb5 <.text+0x7bfcb5>
1407c0c0c: e8 1f 75 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c0c11: 48 89 c1                    	movq	%rax, %rcx
1407c0c14: 48 8b 00                    	movq	(%rax), %rax
1407c0c17: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c0c1b: 0f 85 fe 0b 00 00           	jne	0x1407c181f <.text+0x7c081f>
1407c0c21: 48 83 c1 08                 	addq	$0x8, %rcx
1407c0c25: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c0c29: 0f 85 76 0c 00 00           	jne	0x1407c18a5 <.text+0x7c08a5>
1407c0c2f: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c0c36: 48 89 8d 58 02 00 00        	movq	%rcx, 0x258(%rbp)
1407c0c3d: 48 83 c1 08                 	addq	$0x8, %rcx
1407c0c41: 4c 89 fa                    	movq	%r15, %rdx
1407c0c44: e8 a7 64 45 00              	callq	0x140c170f0 <.text+0xc160f0>
1407c0c49: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1407c0c50: 48 ff 00                    	incq	(%rax)
1407c0c53: 0f 28 b5 70 02 00 00        	movaps	0x270(%rbp), %xmm6
1407c0c5a: 0f 28 bd 80 02 00 00        	movaps	0x280(%rbp), %xmm7
1407c0c61: 44 0f 28 85 90 02 00 00     	movaps	0x290(%rbp), %xmm8
1407c0c69: 44 0f 28 8d a0 02 00 00     	movaps	0x2a0(%rbp), %xmm9
1407c0c71: 44 0f 28 95 b0 02 00 00     	movaps	0x2b0(%rbp), %xmm10
1407c0c79: 44 0f 28 9d c0 02 00 00     	movaps	0x2c0(%rbp), %xmm11
1407c0c81: 44 0f 28 a5 d0 02 00 00     	movaps	0x2d0(%rbp), %xmm12
1407c0c89: 44 0f 28 ad e0 02 00 00     	movaps	0x2e0(%rbp), %xmm13
1407c0c91: 44 0f 28 b5 f0 02 00 00     	movaps	0x2f0(%rbp), %xmm14
1407c0c99: 44 0f 28 bd 00 03 00 00     	movaps	0x300(%rbp), %xmm15
1407c0ca1: 48 81 c4 98 03 00 00        	addq	$0x398, %rsp            # imm = 0x398
1407c0ca8: 5b                          	popq	%rbx
1407c0ca9: 5f                          	popq	%rdi
1407c0caa: 5e                          	popq	%rsi
1407c0cab: 41 5c                       	popq	%r12
1407c0cad: 41 5d                       	popq	%r13
1407c0caf: 41 5e                       	popq	%r14
1407c0cb1: 41 5f                       	popq	%r15
1407c0cb3: 5d                          	popq	%rbp
1407c0cb4: c3                          	retq
1407c0cb5: 4c 89 f1                    	movq	%r14, %rcx
1407c0cb8: e8 b3 5d 36 00              	callq	0x140b26a70 <.text+0xb25a70>
1407c0cbd: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
1407c0cc4: 48 89 95 98 00 00 00        	movq	%rdx, 0x98(%rbp)
1407c0ccb: 48 8b 17                    	movq	(%rdi), %rdx
1407c0cce: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407c0cd5: e8 d6 d8 25 00              	callq	0x140a1e5b0 <.text+0xa1d5b0>
1407c0cda: 48 8b 85 80 03 00 00        	movq	0x380(%rbp), %rax
1407c0ce1: 0f b6 8d 40 01 00 00        	movzbl	0x140(%rbp), %ecx
1407c0ce8: 88 8d 65 02 00 00           	movb	%cl, 0x265(%rbp)
1407c0cee: 48 8b 8d 48 01 00 00        	movq	0x148(%rbp), %rcx
1407c0cf5: 48 89 8d f0 01 00 00        	movq	%rcx, 0x1f0(%rbp)
1407c0cfc: 4c 8b 8d 50 01 00 00        	movq	0x150(%rbp), %r9
1407c0d03: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
1407c0d0a: 48 89 8d 40 02 00 00        	movq	%rcx, 0x240(%rbp)
1407c0d11: 48 8b 8d 60 01 00 00        	movq	0x160(%rbp), %rcx
1407c0d18: 48 89 8d 48 02 00 00        	movq	%rcx, 0x248(%rbp)
1407c0d1f: 48 8b 95 68 01 00 00        	movq	0x168(%rbp), %rdx
1407c0d26: 44 0f b7 95 78 01 00 00     	movzwl	0x178(%rbp), %r10d
1407c0d2e: 4c 8b 85 80 01 00 00        	movq	0x180(%rbp), %r8
1407c0d35: 4c 89 85 20 02 00 00        	movq	%r8, 0x220(%rbp)
1407c0d3c: 4c 8b 06                    	movq	(%rsi), %r8
1407c0d3f: 4c 89 85 f8 01 00 00        	movq	%r8, 0x1f8(%rbp)
1407c0d46: 44 8b 40 1c                 	movl	0x1c(%rax), %r8d
1407c0d4a: 44 89 85 18 02 00 00        	movl	%r8d, 0x218(%rbp)
1407c0d51: 4c 8b 00                    	movq	(%rax), %r8
1407c0d54: 4c 89 85 c8 00 00 00        	movq	%r8, 0xc8(%rbp)
1407c0d5b: 48 8b 40 10                 	movq	0x10(%rax), %rax
1407c0d5f: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
1407c0d66: 48 8b 03                    	movq	(%rbx), %rax
1407c0d69: 48 89 85 d8 00 00 00        	movq	%rax, 0xd8(%rbp)
1407c0d70: f3 44 0f 10 05 ff c2 20 02  	movss	0x220c2ff(%rip), %xmm8  # 0x1429cd078
1407c0d79: f3 44 0f 10 25 82 4b 16 02  	movss	0x2164b82(%rip), %xmm12 # 0x142925904
1407c0d82: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407c0d89: 48 89 95 50 02 00 00        	movq	%rdx, 0x250(%rbp)
1407c0d90: 49 83 f9 02                 	cmpq	$0x2, %r9
1407c0d94: 0f 84 a6 01 00 00           	je	0x1407c0f40 <.text+0x7bff40>
1407c0d9a: 4c 89 8d 28 02 00 00        	movq	%r9, 0x228(%rbp)
1407c0da1: 48 8b 95 48 02 00 00        	movq	0x248(%rbp), %rdx
1407c0da8: 48 85 d2                    	testq	%rdx, %rdx
1407c0dab: 0f 84 d6 00 00 00           	je	0x1407c0e87 <.text+0x7bfe87>
1407c0db1: 48 83 bd 20 02 00 00 00     	cmpq	$0x0, 0x220(%rbp)
1407c0db9: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1407c0dc0: 0f 84 25 01 00 00           	je	0x1407c0eeb <.text+0x7bfeeb>
1407c0dc6: 66 45 85 d2                 	testw	%r10w, %r10w
1407c0dca: 74 14                       	je	0x1407c0de0 <.text+0x7bfde0>
1407c0dcc: 41 8d 42 ff                 	leal	-0x1(%r10), %eax
1407c0dd0: 44 21 d0                    	andl	%r10d, %eax
1407c0dd3: 89 85 60 02 00 00           	movl	%eax, 0x260(%rbp)
1407c0dd9: eb 3f                       	jmp	0x1407c0e1a <.text+0x7bfe1a>
1407c0ddb: 0f 1f 44 00 00              	nopl	(%rax,%rax)
1407c0de0: 66 0f 6f 00                 	movdqa	(%rax), %xmm0
1407c0de4: 66 44 0f d7 d0              	pmovmskb	%xmm0, %r10d
1407c0de9: 48 81 c2 80 f6 ff ff        	addq	$-0x980, %rdx           # imm = 0xF680
1407c0df0: 48 83 c0 10                 	addq	$0x10, %rax
1407c0df4: 41 81 fa ff ff 00 00        	cmpl	$0xffff, %r10d          # imm = 0xFFFF
1407c0dfb: 74 e3                       	je	0x1407c0de0 <.text+0x7bfde0>
1407c0dfd: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1407c0e04: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407c0e09: 44 29 d0                    	subl	%r10d, %eax
1407c0e0c: 41 f7 d2                    	notl	%r10d
1407c0e0f: 44 89 d1                    	movl	%r10d, %ecx
1407c0e12: 21 c1                       	andl	%eax, %ecx
1407c0e14: 89 8d 60 02 00 00           	movl	%ecx, 0x260(%rbp)
1407c0e1a: f3 41 0f bc c2              	tzcntl	%r10d, %eax
1407c0e1f: 48 f7 d8                    	negq	%rax
1407c0e22: 48 69 c0 98 00 00 00        	imulq	$0x98, %rax, %rax
1407c0e29: 48 89 c6                    	movq	%rax, %rsi
1407c0e2c: 48 89 95 48 02 00 00        	movq	%rdx, 0x248(%rbp)
1407c0e33: 48 8d 0c 02                 	leaq	(%rdx,%rax), %rcx
1407c0e37: 48 81 c1 68 ff ff ff        	addq	$-0x98, %rcx
1407c0e3e: e8 5d b4 25 00              	callq	0x140a1c2a0 <.text+0xa1b2a0>
1407c0e43: 48 ff 8d 20 02 00 00        	decq	0x220(%rbp)
1407c0e4a: 48 83 f8 03                 	cmpq	$0x3, %rax
1407c0e4e: 0f 85 c9 00 00 00           	jne	0x1407c0f1d <.text+0x7bff1d>
1407c0e54: 8b 85 60 02 00 00           	movl	0x260(%rbp), %eax
1407c0e5a: 41 89 c2                    	movl	%eax, %r10d
1407c0e5d: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1407c0e64: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1407c0e6b: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407c0e72: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1407c0e79: f6 85 28 02 00 00 01        	testb	$0x1, 0x228(%rbp)
1407c0e80: 75 23                       	jne	0x1407c0ea5 <.text+0x7bfea5>
1407c0e82: e9 86 00 00 00              	jmp	0x1407c0f0d <.text+0x7bff0d>
1407c0e87: 48 8b 85 50 02 00 00        	movq	0x250(%rbp), %rax
1407c0e8e: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1407c0e95: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1407c0e9c: f6 85 28 02 00 00 01        	testb	$0x1, 0x228(%rbp)
1407c0ea3: 74 68                       	je	0x1407c0f0d <.text+0x7bff0d>
1407c0ea5: b8 01 00 00 00              	movl	$0x1, %eax
1407c0eaa: 41 b8 00 00 00 00           	movl	$0x0, %r8d
1407c0eb0: 4c 89 85 48 02 00 00        	movq	%r8, 0x248(%rbp)
1407c0eb7: 41 b8 00 00 00 00           	movl	$0x0, %r8d
1407c0ebd: 48 85 d2                    	testq	%rdx, %rdx
1407c0ec0: 0f 84 81 00 00 00           	je	0x1407c0f47 <.text+0x7bff47>
1407c0ec6: 45 89 d0                    	movl	%r10d, %r8d
1407c0ec9: 44 89 85 60 02 00 00        	movl	%r8d, 0x260(%rbp)
1407c0ed0: 48 c7 85 48 02 00 00 00 00 00 00    	movq	$0x0, 0x248(%rbp)
1407c0edb: 48 c7 85 40 02 00 00 00 00 00 00    	movq	$0x0, 0x240(%rbp)
1407c0ee6: e9 a1 00 00 00              	jmp	0x1407c0f8c <.text+0x7bff8c>
1407c0eeb: 48 c7 85 20 02 00 00 00 00 00 00    	movq	$0x0, 0x220(%rbp)
1407c0ef6: 48 8b 95 40 02 00 00        	movq	0x240(%rbp), %rdx
1407c0efd: 48 89 85 50 02 00 00        	movq	%rax, 0x250(%rbp)
1407c0f04: f6 85 28 02 00 00 01        	testb	$0x1, 0x228(%rbp)
1407c0f0b: 75 98                       	jne	0x1407c0ea5 <.text+0x7bfea5>
1407c0f0d: 48 c7 85 48 02 00 00 00 00 00 00    	movq	$0x0, 0x248(%rbp)
1407c0f18: 49 89 d0                    	movq	%rdx, %r8
1407c0f1b: eb 2a                       	jmp	0x1407c0f47 <.text+0x7bff47>
1407c0f1d: 48 89 d1                    	movq	%rdx, %rcx
1407c0f20: 48 89 f2                    	movq	%rsi, %rdx
1407c0f23: 48 03 95 48 02 00 00        	addq	0x248(%rbp), %rdx
1407c0f2a: 48 81 c2 70 ff ff ff        	addq	$-0x90, %rdx
1407c0f31: 49 89 c8                    	movq	%rcx, %r8
1407c0f34: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407c0f3b: eb 56                       	jmp	0x1407c0f93 <.text+0x7bff93>
1407c0f3d: 0f 1f 00                    	nopl	(%rax)
1407c0f40: 4c 8b 85 40 02 00 00        	movq	0x240(%rbp), %r8
1407c0f47: 48 8b 95 f0 01 00 00        	movq	0x1f0(%rbp), %rdx
1407c0f4e: 48 85 d2                    	testq	%rdx, %rdx
1407c0f51: 0f 95 c0                    	setne	%al
1407c0f54: 84 85 65 02 00 00           	testb	%al, 0x265(%rbp)
1407c0f5a: 0f 84 d9 08 00 00           	je	0x1407c1839 <.text+0x7c0839>
1407c0f60: b8 02 00 00 00              	movl	$0x2, %eax
1407c0f65: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
1407c0f6c: 44 89 d0                    	movl	%r10d, %eax
1407c0f6f: 89 85 60 02 00 00           	movl	%eax, 0x260(%rbp)
1407c0f75: 4c 89 85 40 02 00 00        	movq	%r8, 0x240(%rbp)
1407c0f7c: 48 c7 85 f0 01 00 00 00 00 00 00    	movq	$0x0, 0x1f0(%rbp)
1407c0f87: b8 02 00 00 00              	movl	$0x2, %eax
1407c0f8c: 4c 8b 85 00 02 00 00        	movq	0x200(%rbp), %r8
1407c0f93: 48 89 85 30 01 00 00        	movq	%rax, 0x130(%rbp)
1407c0f9a: 4c 89 85 00 02 00 00        	movq	%r8, 0x200(%rbp)
1407c0fa1: 4c 89 85 38 01 00 00        	movq	%r8, 0x138(%rbp)
1407c0fa8: e8 c3 a0 25 00              	callq	0x140a1b070 <.text+0xa1a070>
1407c0fad: 48 8b b5 40 01 00 00        	movq	0x140(%rbp), %rsi
1407c0fb4: 4c 8b a5 48 01 00 00        	movq	0x148(%rbp), %r12
1407c0fbb: 0f b7 8d 58 01 00 00        	movzwl	0x158(%rbp), %ecx
1407c0fc2: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
1407c0fc9: 48 89 85 58 02 00 00        	movq	%rax, 0x258(%rbp)
1407c0fd0: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
1407c0fd7: 48 89 85 38 02 00 00        	movq	%rax, 0x238(%rbp)
1407c0fde: 48 83 c0 50                 	addq	$0x50, %rax
1407c0fe2: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1407c0fe9: 48 83 bd 58 02 00 00 00     	cmpq	$0x0, 0x258(%rbp)
1407c0ff1: 0f 84 c5 07 00 00           	je	0x1407c17bc <.text+0x7c07bc>
1407c0ff7: 66 85 c9                    	testw	%cx, %cx
1407c0ffa: 74 14                       	je	0x1407c1010 <.text+0x7c0010>
1407c0ffc: 44 8d 71 ff                 	leal	-0x1(%rcx), %r14d
1407c1000: 41 21 ce                    	andl	%ecx, %r14d
1407c1003: 48 85 f6                    	testq	%rsi, %rsi
1407c1006: 75 31                       	jne	0x1407c1039 <.text+0x7c0039>
1407c1008: e9 af 07 00 00              	jmp	0x1407c17bc <.text+0x7c07bc>
1407c100d: 0f 1f 00                    	nopl	(%rax)
1407c1010: 66 41 0f 6f 04 24           	movdqa	(%r12), %xmm0
1407c1016: 66 0f d7 c8                 	pmovmskb	%xmm0, %ecx
1407c101a: 48 83 c6 c0                 	addq	$-0x40, %rsi
1407c101e: 49 83 c4 10                 	addq	$0x10, %r12
1407c1022: 81 f9 ff ff 00 00           	cmpl	$0xffff, %ecx           # imm = 0xFFFF
1407c1028: 74 e6                       	je	0x1407c1010 <.text+0x7c0010>
1407c102a: b8 fe ff ff ff              	movl	$0xfffffffe, %eax       # imm = 0xFFFFFFFE
1407c102f: 29 c8                       	subl	%ecx, %eax
1407c1031: f7 d1                       	notl	%ecx
1407c1033: 41 89 ce                    	movl	%ecx, %r14d
1407c1036: 41 21 c6                    	andl	%eax, %r14d
1407c1039: f3 0f bc c1                 	tzcntl	%ecx, %eax
1407c103d: c1 e0 02                    	shll	$0x2, %eax
1407c1040: 48 89 f1                    	movq	%rsi, %rcx
1407c1043: 48 29 c1                    	subq	%rax, %rcx
1407c1046: 44 8b 79 fc                 	movl	-0x4(%rcx), %r15d
1407c104a: 44 89 bd 40 01 00 00        	movl	%r15d, 0x140(%rbp)
1407c1051: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
1407c1058: 48 83 78 48 00              	cmpq	$0x0, 0x48(%rax)
1407c105d: 0f 84 8d 07 00 00           	je	0x1407c17f0 <.text+0x7c07f0>
1407c1063: 48 8b 8d c0 00 00 00        	movq	0xc0(%rbp), %rcx
1407c106a: 48 8d 95 40 01 00 00        	leaq	0x140(%rbp), %rdx
1407c1071: e8 5a fe de ff              	callq	0x1405b0ed0 <.text+0x5afed0>
1407c1076: 48 ff 8d 58 02 00 00        	decq	0x258(%rbp)
1407c107d: 48 8b 8d 38 02 00 00        	movq	0x238(%rbp), %rcx
1407c1084: 48 8b 79 30                 	movq	0x30(%rcx), %rdi
1407c1088: 48 8b 49 38                 	movq	0x38(%rcx), %rcx
1407c108c: 48 89 ca                    	movq	%rcx, %rdx
1407c108f: 48 21 c2                    	andq	%rax, %rdx
1407c1092: 48 c1 e8 39                 	shrq	$0x39, %rax
1407c1096: 66 0f 6e c0                 	movd	%eax, %xmm0
1407c109a: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
1407c109e: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
1407c10a3: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
1407c10a8: 48 8d 47 b0                 	leaq	-0x50(%rdi), %rax
1407c10ac: 45 31 c0                    	xorl	%r8d, %r8d
1407c10af: f3 0f 6f 0c 17              	movdqu	(%rdi,%rdx), %xmm1
1407c10b4: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
1407c10b8: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
1407c10bc: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
1407c10c1: 45 85 c9                    	testl	%r9d, %r9d
1407c10c4: 74 2a                       	je	0x1407c10f0 <.text+0x7c00f0>
1407c10c6: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
1407c10cb: 49 01 d2                    	addq	%rdx, %r10
1407c10ce: 49 21 ca                    	andq	%rcx, %r10
1407c10d1: 49 f7 da                    	negq	%r10
1407c10d4: 4b 8d 1c 92                 	leaq	(%r10,%r10,4), %rbx
1407c10d8: 48 c1 e3 04                 	shlq	$0x4, %rbx
1407c10dc: 44 3b 3c 18                 	cmpl	(%rax,%rbx), %r15d
1407c10e0: 74 3e                       	je	0x1407c1120 <.text+0x7c0120>
1407c10e2: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
1407c10e6: 66 45 21 ca                 	andw	%r9w, %r10w
1407c10ea: 45 89 d1                    	movl	%r10d, %r9d
1407c10ed: 75 d7                       	jne	0x1407c10c6 <.text+0x7c00c6>
1407c10ef: 90                          	nop
1407c10f0: 66 0f 74 0d 98 48 16 02     	pcmpeqb	0x2164898(%rip), %xmm1  # 0x142925990
1407c10f8: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
1407c10fd: 45 85 c9                    	testl	%r9d, %r9d
1407c1100: 0f 85 ea 06 00 00           	jne	0x1407c17f0 <.text+0x7c07f0>
1407c1106: 4c 01 c2                    	addq	%r8, %rdx
1407c1109: 48 83 c2 10                 	addq	$0x10, %rdx
1407c110d: 49 83 c0 10                 	addq	$0x10, %r8
1407c1111: 48 21 ca                    	andq	%rcx, %rdx
1407c1114: eb 99                       	jmp	0x1407c10af <.text+0x7c00af>
1407c1116: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407c1120: 48 8d 8d 90 00 00 00        	leaq	0x90(%rbp), %rcx
1407c1127: 44 89 fa                    	movl	%r15d, %edx
1407c112a: e8 41 58 36 00              	callq	0x140b26970 <.text+0xb25970>
1407c112f: 44 89 f1                    	movl	%r14d, %ecx
1407c1132: 3c 15                       	cmpb	$0x15, %al
1407c1134: 0f 85 af fe ff ff           	jne	0x1407c0fe9 <.text+0x7bffe9>
1407c113a: 48 01 df                    	addq	%rbx, %rdi
1407c113d: 0f b6 47 ff                 	movzbl	-0x1(%rdi), %eax
1407c1141: 88 85 67 02 00 00           	movb	%al, 0x267(%rbp)
1407c1147: 48 8d 8d e0 00 00 00        	leaq	0xe0(%rbp), %rcx
1407c114e: 48 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %rdx
1407c1155: 45 89 f8                    	movl	%r15d, %r8d
1407c1158: 4c 8d 0d c9 50 2c 02        	leaq	0x22c50c9(%rip), %r9    # 0x142a86228
1407c115f: e8 bc 98 25 00              	callq	0x140a1aa20 <.text+0xa19a20>
1407c1164: 8b 47 e0                    	movl	-0x20(%rdi), %eax
1407c1167: 89 85 1c 02 00 00           	movl	%eax, 0x21c(%rbp)
1407c116d: 8b 85 18 02 00 00           	movl	0x218(%rbp), %eax
1407c1173: 48 8b 8d d0 00 00 00        	movq	0xd0(%rbp), %rcx
1407c117a: 89 01                       	movl	%eax, (%rcx)
1407c117c: 4c 8b 85 30 01 00 00        	movq	0x130(%rbp), %r8
1407c1183: 4c 8b 8d 38 01 00 00        	movq	0x138(%rbp), %r9
1407c118a: 0f b6 85 67 02 00 00        	movzbl	0x267(%rbp), %eax
1407c1191: 88 44 24 20                 	movb	%al, 0x20(%rsp)
1407c1195: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
1407c119c: 44 89 fa                    	movl	%r15d, %edx
1407c119f: e8 0c 47 06 00              	callq	0x1408258b0 <.text+0x8248b0>
1407c11a4: 49 89 c5                    	movq	%rax, %r13
1407c11a7: 48 8b 00                    	movq	(%rax), %rax
1407c11aa: 49 0b 45 08                 	orq	0x8(%r13), %rax
1407c11ae: 0f 85 fd 06 00 00           	jne	0x1407c18b1 <.text+0x7c08b1>
1407c11b4: 49 c7 45 08 00 00 00 00     	movq	$0x0, 0x8(%r13)
1407c11bc: 49 c7 45 00 01 00 00 00     	movq	$0x1, (%r13)
1407c11c4: 66 0f 6f 85 e0 00 00 00     	movdqa	0xe0(%rbp), %xmm0
1407c11cc: 66 0f 6f 8d f0 00 00 00     	movdqa	0xf0(%rbp), %xmm1
1407c11d4: 66 0f 6f 95 00 01 00 00     	movdqa	0x100(%rbp), %xmm2
1407c11dc: 66 41 0f 7f 45 10           	movdqa	%xmm0, 0x10(%r13)
1407c11e2: 66 41 0f 7f 4d 20           	movdqa	%xmm1, 0x20(%r13)
1407c11e8: 66 41 0f 7f 55 30           	movdqa	%xmm2, 0x30(%r13)
1407c11ee: 48 8d 8d 30 01 00 00        	leaq	0x130(%rbp), %rcx
1407c11f5: e8 46 b0 25 00              	callq	0x140a1c240 <.text+0xa1b240>
1407c11fa: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
1407c1201: 48 89 c2                    	movq	%rax, %rdx
1407c1204: 4c 8d 05 35 50 2c 02        	leaq	0x22c5035(%rip), %r8    # 0x142a86240
1407c120b: e8 e0 0c 2d 00              	callq	0x140a91ef0 <.text+0xa90ef0>
1407c1210: 48 89 c3                    	movq	%rax, %rbx
1407c1213: 48 85 c0                    	testq	%rax, %rax
1407c1216: 0f 84 fc 06 00 00           	je	0x1407c1918 <.text+0x7c0918>
1407c121c: 48 83 c7 e8                 	addq	$-0x18, %rdi
1407c1220: 48 89 f9                    	movq	%rdi, %rcx
1407c1223: 48 8d 15 46 50 2c 02        	leaq	0x22c5046(%rip), %rdx   # 0x142a86270
1407c122a: e8 a1 80 07 00              	callq	0x1408392d0 <.text+0x8382d0>
1407c122f: f3 0f 10 38                 	movss	(%rax), %xmm7
1407c1233: f3 41 0f 58 f8              	addss	%xmm8, %xmm7
1407c1238: 8b 8d f0 00 00 00           	movl	0xf0(%rbp), %ecx
1407c123e: 89 8d b0 00 00 00           	movl	%ecx, 0xb0(%rbp)
1407c1244: 0f 28 85 e0 00 00 00        	movaps	0xe0(%rbp), %xmm0
1407c124b: 0f 29 85 a0 00 00 00        	movaps	%xmm0, 0xa0(%rbp)
1407c1252: f3 44 0f 10 ad f4 00 00 00  	movss	0xf4(%rbp), %xmm13
1407c125b: 48 8d 95 f8 00 00 00        	leaq	0xf8(%rbp), %rdx
1407c1262: 48 8b 4a 10                 	movq	0x10(%rdx), %rcx
1407c1266: 48 89 8d e0 01 00 00        	movq	%rcx, 0x1e0(%rbp)
1407c126d: f3 0f 6f 02                 	movdqu	(%rdx), %xmm0
1407c1271: 66 0f 7f 85 d0 01 00 00     	movdqa	%xmm0, 0x1d0(%rbp)
1407c1279: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407c1280: c7 41 fb 00 00 00 00        	movl	$0x0, -0x5(%rcx)
1407c1287: 48 c7 41 f3 00 00 00 00     	movq	$0x0, -0xd(%rcx)
1407c128f: 0f b6 40 04                 	movzbl	0x4(%rax), %eax
1407c1293: 88 85 66 02 00 00           	movb	%al, 0x266(%rbp)
1407c1299: 48 8b 85 40 01 00 00        	movq	0x140(%rbp), %rax
1407c12a0: 48 8b 8d 47 01 00 00        	movq	0x147(%rbp), %rcx
1407c12a7: 48 89 8d 17 01 00 00        	movq	%rcx, 0x117(%rbp)
1407c12ae: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
1407c12b5: 49 8b bd c0 00 00 00        	movq	0xc0(%r13), %rdi
1407c12bc: 49 3b bd b0 00 00 00        	cmpq	0xb0(%r13), %rdi
1407c12c3: 75 16                       	jne	0x1407c12db <.text+0x7c02db>
1407c12c5: 4c 89 e9                    	movq	%r13, %rcx
1407c12c8: 48 81 c1 b0 00 00 00        	addq	$0xb0, %rcx
1407c12cf: 48 8d 15 b2 4f 2c 02        	leaq	0x22c4fb2(%rip), %rdx   # 0x142a86288
1407c12d6: e8 d5 a0 06 00              	callq	0x14082b3b0 <.text+0x82a3b0>
1407c12db: 0f 28 c7                    	movaps	%xmm7, %xmm0
1407c12de: f3 41 0f 58 c5              	addss	%xmm13, %xmm0
1407c12e3: 49 8b 85 b8 00 00 00        	movq	0xb8(%r13), %rax
1407c12ea: 48 69 cf 60 01 00 00        	imulq	$0x160, %rdi, %rcx      # imm = 0x160
1407c12f1: 48 c7 04 08 00 00 00 00     	movq	$0x0, (%rax,%rcx)
1407c12f9: 48 c7 44 08 10 00 00 00 00  	movq	$0x0, 0x10(%rax,%rcx)
1407c1302: 48 c7 44 08 20 00 00 00 00  	movq	$0x0, 0x20(%rax,%rcx)
1407c130b: c7 44 08 30 00 00 00 00     	movl	$0x0, 0x30(%rax,%rcx)
1407c1313: 48 8b 55 54                 	movq	0x54(%rbp), %rdx
1407c1317: 48 89 54 08 34              	movq	%rdx, 0x34(%rax,%rcx)
1407c131c: 8b 55 5c                    	movl	0x5c(%rbp), %edx
1407c131f: 89 54 08 3c                 	movl	%edx, 0x3c(%rax,%rcx)
1407c1323: c7 44 08 40 00 00 00 00     	movl	$0x0, 0x40(%rax,%rcx)
1407c132b: 0f 10 8d 40 01 00 00        	movups	0x140(%rbp), %xmm1
1407c1332: f3 0f 6f 95 50 01 00 00     	movdqu	0x150(%rbp), %xmm2
1407c133a: 0f 10 9d 60 01 00 00        	movups	0x160(%rbp), %xmm3
1407c1341: 0f 10 a5 6c 01 00 00        	movups	0x16c(%rbp), %xmm4
1407c1348: 0f 11 4c 08 44              	movups	%xmm1, 0x44(%rax,%rcx)
1407c134d: f3 0f 7f 54 08 54           	movdqu	%xmm2, 0x54(%rax,%rcx)
1407c1353: 0f 11 5c 08 64              	movups	%xmm3, 0x64(%rax,%rcx)
1407c1358: 0f 11 64 08 70              	movups	%xmm4, 0x70(%rax,%rcx)
1407c135d: c7 84 08 80 00 00 00 00 00 00 00    	movl	$0x0, 0x80(%rax,%rcx)
1407c1368: 48 8b 55 60                 	movq	0x60(%rbp), %rdx
1407c136c: 48 89 94 08 84 00 00 00     	movq	%rdx, 0x84(%rax,%rcx)
1407c1374: 8b 55 68                    	movl	0x68(%rbp), %edx
1407c1377: 89 94 08 8c 00 00 00        	movl	%edx, 0x8c(%rax,%rcx)
1407c137e: c7 84 08 90 00 00 00 00 00 00 00    	movl	$0x0, 0x90(%rax,%rcx)
1407c1389: 48 8b 55 6c                 	movq	0x6c(%rbp), %rdx
1407c138d: 48 89 94 08 94 00 00 00     	movq	%rdx, 0x94(%rax,%rcx)
1407c1395: 8b 55 74                    	movl	0x74(%rbp), %edx
1407c1398: 89 94 08 9c 00 00 00        	movl	%edx, 0x9c(%rax,%rcx)
1407c139f: 48 c7 84 08 a0 00 00 00 01 00 00 00 	movq	$0x1, 0xa0(%rax,%rcx)
1407c13ab: 48 c7 84 08 a8 00 00 00 00 00 00 00 	movq	$0x0, 0xa8(%rax,%rcx)
1407c13b7: 48 8d 15 66 fe 45 02        	leaq	0x245fe66(%rip), %rdx   # 0x142c21224
1407c13be: 48 89 94 08 b0 00 00 00     	movq	%rdx, 0xb0(%rax,%rcx)
1407c13c6: 48 c7 84 08 b8 00 00 00 0a 00 00 00 	movq	$0xa, 0xb8(%rax,%rcx)
1407c13d2: 8b 95 b0 00 00 00           	movl	0xb0(%rbp), %edx
1407c13d8: 89 94 08 d0 00 00 00        	movl	%edx, 0xd0(%rax,%rcx)
1407c13df: 0f 28 8d a0 00 00 00        	movaps	0xa0(%rbp), %xmm1
1407c13e6: 0f 29 8c 08 c0 00 00 00     	movaps	%xmm1, 0xc0(%rax,%rcx)
1407c13ee: f3 0f 11 84 08 d4 00 00 00  	movss	%xmm0, 0xd4(%rax,%rcx)
1407c13f7: 48 8b 95 e0 01 00 00        	movq	0x1e0(%rbp), %rdx
1407c13fe: 48 89 94 08 e8 00 00 00     	movq	%rdx, 0xe8(%rax,%rcx)
1407c1406: 0f 28 85 d0 01 00 00        	movaps	0x1d0(%rbp), %xmm0
1407c140d: 0f 11 84 08 d8 00 00 00     	movups	%xmm0, 0xd8(%rax,%rcx)
1407c1415: c7 84 08 f0 00 00 00 00 00 00 00    	movl	$0x0, 0xf0(%rax,%rcx)
1407c1420: c7 84 08 f8 00 00 00 00 00 00 00    	movl	$0x0, 0xf8(%rax,%rcx)
1407c142b: 48 8b 95 88 00 00 00        	movq	0x88(%rbp), %rdx
1407c1432: 48 89 94 08 0c 01 00 00     	movq	%rdx, 0x10c(%rax,%rcx)
1407c143a: 0f 10 45 78                 	movups	0x78(%rbp), %xmm0
1407c143e: 0f 11 84 08 fc 00 00 00     	movups	%xmm0, 0xfc(%rax,%rcx)
1407c1446: c6 84 08 14 01 00 00 02     	movb	$0x2, 0x114(%rax,%rcx)
1407c144e: 0f b6 95 37 02 00 00        	movzbl	0x237(%rbp), %edx
1407c1455: 88 94 08 17 01 00 00        	movb	%dl, 0x117(%rax,%rcx)
1407c145c: 0f b7 95 35 02 00 00        	movzwl	0x235(%rbp), %edx
1407c1463: 66 89 94 08 15 01 00 00     	movw	%dx, 0x115(%rax,%rcx)
1407c146b: c7 84 08 18 01 00 00 00 00 00 00    	movl	$0x0, 0x118(%rax,%rcx)
1407c1476: 0f 10 45 c0                 	movups	-0x40(%rbp), %xmm0
1407c147a: f3 0f 6f 4d d0              	movdqu	-0x30(%rbp), %xmm1
1407c147f: f3 0f 7f 8c 08 2c 01 00 00  	movdqu	%xmm1, 0x12c(%rax,%rcx)
1407c1488: 0f 11 84 08 1c 01 00 00     	movups	%xmm0, 0x11c(%rax,%rcx)
1407c1490: c6 84 08 3c 01 00 00 02     	movb	$0x2, 0x13c(%rax,%rcx)
1407c1498: 48 8b 95 10 01 00 00        	movq	0x110(%rbp), %rdx
1407c149f: 4c 8b 85 17 01 00 00        	movq	0x117(%rbp), %r8
1407c14a6: 4c 89 84 08 44 01 00 00     	movq	%r8, 0x144(%rax,%rcx)
1407c14ae: 48 89 94 08 3d 01 00 00     	movq	%rdx, 0x13d(%rax,%rcx)
1407c14b6: c6 84 08 4c 01 00 00 00     	movb	$0x0, 0x14c(%rax,%rcx)
1407c14be: 0f b6 95 66 02 00 00        	movzbl	0x266(%rbp), %edx
1407c14c5: 88 94 08 4d 01 00 00        	movb	%dl, 0x14d(%rax,%rcx)
1407c14cc: 0f b6 95 67 02 00 00        	movzbl	0x267(%rbp), %edx
1407c14d3: 88 94 08 4e 01 00 00        	movb	%dl, 0x14e(%rax,%rcx)
1407c14da: 66 c7 84 08 4f 01 00 00 00 09       	movw	$0x900, 0x14f(%rax,%rcx) # imm = 0x900
1407c14e4: 48 8b 95 20 01 00 00        	movq	0x120(%rbp), %rdx
1407c14eb: 4c 8b 85 27 01 00 00        	movq	0x127(%rbp), %r8
1407c14f2: 48 89 94 08 51 01 00 00     	movq	%rdx, 0x151(%rax,%rcx)
1407c14fa: 4c 89 84 08 58 01 00 00     	movq	%r8, 0x158(%rax,%rcx)
1407c1502: 48 ff c7                    	incq	%rdi
1407c1505: 49 89 bd c0 00 00 00        	movq	%rdi, 0xc0(%r13)
1407c150c: 44 0f 28 f7                 	movaps	%xmm7, %xmm14
1407c1510: f3 44 0f 59 35 57 43 16 02  	mulss	0x2164357(%rip), %xmm14 # 0x142925870
1407c1519: f3 45 0f 58 ee              	addss	%xmm14, %xmm13
1407c151e: f3 44 0f 11 ad f4 00 00 00  	movss	%xmm13, 0xf4(%rbp)
1407c1527: f3 44 0f 10 8d f0 00 00 00  	movss	0xf0(%rbp), %xmm9
1407c1530: f3 44 0f 10 95 f8 00 00 00  	movss	0xf8(%rbp), %xmm10
1407c1539: 44 0f 28 bd e0 00 00 00     	movaps	0xe0(%rbp), %xmm15
1407c1541: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407c1548: 48 8b 95 f8 01 00 00        	movq	0x1f8(%rbp), %rdx
1407c154f: 45 89 f8                    	movl	%r15d, %r8d
1407c1552: 4c 8d 0d 47 4d 2c 02        	leaq	0x22c4d47(%rip), %r9    # 0x142a862a0
1407c1559: e8 c2 94 25 00              	callq	0x140a1aa20 <.text+0xa19a20>
1407c155e: f3 44 0f 10 9d 54 01 00 00  	movss	0x154(%rbp), %xmm11
1407c1567: 48 89 d9                    	movq	%rbx, %rcx
1407c156a: e8 b1 90 36 00              	callq	0x140b2a620 <.text+0xb29620>
1407c156f: 0f 28 f0                    	movaps	%xmm0, %xmm6
1407c1572: c7 85 d0 01 00 00 9a 99 59 3f       	movl	$0x3f59999a, 0x1d0(%rbp) # imm = 0x3F59999A
1407c157c: f3 0f 11 bd d4 01 00 00     	movss	%xmm7, 0x1d4(%rbp)
1407c1584: c7 85 d8 01 00 00 9a 99 59 3f       	movl	$0x3f59999a, 0x1d8(%rbp) # imm = 0x3F59999A
1407c158e: c7 85 48 01 00 00 00 00 00 00       	movl	$0x0, 0x148(%rbp)
1407c1598: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
1407c15a3: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
1407c15a7: 48 8d 95 d0 01 00 00        	leaq	0x1d0(%rbp), %rdx
1407c15ae: 4c 8d 85 40 01 00 00        	leaq	0x140(%rbp), %r8
1407c15b5: e8 36 ea 10 00              	callq	0x1408cfff0 <.text+0x8ceff0>
1407c15ba: 48 c7 85 40 01 00 00 01 00 00 00    	movq	$0x1, 0x140(%rbp)
1407c15c5: 48 c7 85 48 01 00 00 01 00 00 00    	movq	$0x1, 0x148(%rbp)
1407c15d0: 0f 28 45 30                 	movaps	0x30(%rbp), %xmm0
1407c15d4: 48 8d 85 50 01 00 00        	leaq	0x150(%rbp), %rax
1407c15db: 0f 29 40 70                 	movaps	%xmm0, 0x70(%rax)
1407c15df: 0f 28 45 20                 	movaps	0x20(%rbp), %xmm0
1407c15e3: 0f 29 40 60                 	movaps	%xmm0, 0x60(%rax)
1407c15e7: 0f 28 45 10                 	movaps	0x10(%rbp), %xmm0
1407c15eb: 0f 29 40 50                 	movaps	%xmm0, 0x50(%rax)
1407c15ef: 0f 28 45 00                 	movaps	(%rbp), %xmm0
1407c15f3: 0f 29 40 40                 	movaps	%xmm0, 0x40(%rax)
1407c15f7: 66 0f 6f 45 c0              	movdqa	-0x40(%rbp), %xmm0
1407c15fc: 66 0f 6f 4d d0              	movdqa	-0x30(%rbp), %xmm1
1407c1601: 66 0f 6f 55 e0              	movdqa	-0x20(%rbp), %xmm2
1407c1606: 0f 28 5d f0                 	movaps	-0x10(%rbp), %xmm3
1407c160a: 0f 29 58 30                 	movaps	%xmm3, 0x30(%rax)
1407c160e: 66 0f 7f 50 20              	movdqa	%xmm2, 0x20(%rax)
1407c1613: 66 0f 7f 48 10              	movdqa	%xmm1, 0x10(%rax)
1407c1618: 66 0f 7f 00                 	movdqa	%xmm0, (%rax)
1407c161c: 0f b6 05 e6 35 e7 02        	movzbl	0x2e735e6(%rip), %eax   # 0x143634c09
1407c1623: b9 90 00 00 00              	movl	$0x90, %ecx
1407c1628: ba 10 00 00 00              	movl	$0x10, %edx
1407c162d: e8 de 25 e5 ff              	callq	0x140613c10 <.text+0x612c10>
1407c1632: 48 85 c0                    	testq	%rax, %rax
1407c1635: 0f 84 cc 02 00 00           	je	0x1407c1907 <.text+0x7c0907>
1407c163b: 48 89 c7                    	movq	%rax, %rdi
1407c163e: 41 b8 90 00 00 00           	movl	$0x90, %r8d
1407c1644: 48 89 c1                    	movq	%rax, %rcx
1407c1647: 48 8d 9d 40 01 00 00        	leaq	0x140(%rbp), %rbx
1407c164e: 48 89 da                    	movq	%rbx, %rdx
1407c1651: e8 aa 25 16 02              	callq	0x142923c00 <.text+0x2922c00>
1407c1656: f0                          	lock
1407c1657: 48 ff 07                    	incq	(%rdi)
1407c165a: 0f 8e c4 02 00 00           	jle	0x1407c1924 <.text+0x7c0924>
1407c1660: f3 44 0f 11 8d 40 01 00 00  	movss	%xmm9, 0x140(%rbp)
1407c1669: f3 44 0f 11 ad 44 01 00 00  	movss	%xmm13, 0x144(%rbp)
1407c1672: f3 44 0f 11 95 48 01 00 00  	movss	%xmm10, 0x148(%rbp)
1407c167b: 44 0f 29 7d 40              	movaps	%xmm15, 0x40(%rbp)
1407c1680: 48 8d 45 40                 	leaq	0x40(%rbp), %rax
1407c1684: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c1689: 4c 89 e9                    	movq	%r13, %rcx
1407c168c: 48 89 fa                    	movq	%rdi, %rdx
1407c168f: 49 89 f8                    	movq	%rdi, %r8
1407c1692: 49 89 d9                    	movq	%rbx, %r9
1407c1695: e8 16 3c 06 00              	callq	0x1408252b0 <.text+0x8242b0>
1407c169a: 0f b6 05 68 35 e7 02        	movzbl	0x2e73568(%rip), %eax   # 0x143634c09
1407c16a1: b9 28 00 00 00              	movl	$0x28, %ecx
1407c16a6: ba 04 00 00 00              	movl	$0x4, %edx
1407c16ab: e8 60 25 e5 ff              	callq	0x140613c10 <.text+0x612c10>
1407c16b0: 48 85 c0                    	testq	%rax, %rax
1407c16b3: 0f 84 3d 02 00 00           	je	0x1407c18f6 <.text+0x7c08f6>
1407c16b9: 48 89 c7                    	movq	%rax, %rdi
1407c16bc: f3 44 0f 5c de              	subss	%xmm6, %xmm11
1407c16c1: 41 0f 28 c5                 	movaps	%xmm13, %xmm0
1407c16c5: f3 41 0f 5c c6              	subss	%xmm14, %xmm0
1407c16ca: c7 00 02 00 00 00           	movl	$0x2, (%rax)
1407c16d0: f3 44 0f 11 58 04           	movss	%xmm11, 0x4(%rax)
1407c16d6: f3 44 0f 11 48 08           	movss	%xmm9, 0x8(%rax)
1407c16dc: f3 44 0f 11 50 0c           	movss	%xmm10, 0xc(%rax)
1407c16e2: f3 0f 11 40 10              	movss	%xmm0, 0x10(%rax)
1407c16e7: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
1407c16ec: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
1407c16f0: 41 0f 58 cf                 	addps	%xmm15, %xmm1
1407c16f4: 0f c6 c9 ff                 	shufps	$0xff, %xmm1, %xmm1     # xmm1 = xmm1[3,3,3,3]
1407c16f8: 41 0f 28 d7                 	movaps	%xmm15, %xmm2
1407c16fc: 41 0f c6 d7 c5              	shufps	$0xc5, %xmm15, %xmm2    # xmm2 = xmm2[1,1],xmm15[0,3]
1407c1701: 0f 59 15 98 42 16 02        	mulps	0x2164298(%rip), %xmm2  # 0x1429259a0
1407c1708: 41 0f 28 df                 	movaps	%xmm15, %xmm3
1407c170c: 41 0f c6 df d6              	shufps	$0xd6, %xmm15, %xmm3    # xmm3 = xmm3[2,1],xmm15[1,3]
1407c1711: 0f 59 1d e8 0c 2c 02        	mulps	0x22c0ce8(%rip), %xmm3  # 0x142a82400
1407c1718: 0f 5c d3                    	subps	%xmm3, %xmm2
1407c171b: 0f 59 d1                    	mulps	%xmm1, %xmm2
1407c171e: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
1407c1722: 41 0f 59 cf                 	mulps	%xmm15, %xmm1
1407c1726: 0f 28 d9                    	movaps	%xmm1, %xmm3
1407c1729: 0f c6 d9 ff                 	shufps	$0xff, %xmm1, %xmm3     # xmm3 = xmm3[3,3],xmm1[3,3]
1407c172d: f3 0f 16 e1                 	movshdup	%xmm1, %xmm4            # xmm4 = xmm1[1,1,3,3]
1407c1731: f3 0f 58 e1                 	addss	%xmm1, %xmm4
1407c1735: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407c1738: f3 0f 58 cc                 	addss	%xmm4, %xmm1
1407c173c: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407c1740: 0f 5c d9                    	subps	%xmm1, %xmm3
1407c1743: 41 0f 59 dc                 	mulps	%xmm12, %xmm3
1407c1747: 41 0f 28 cf                 	movaps	%xmm15, %xmm1
1407c174b: 41 0f 59 cc                 	mulps	%xmm12, %xmm1
1407c174f: f3 0f 16 e1                 	movshdup	%xmm1, %xmm4            # xmm4 = xmm1[1,1,3,3]
1407c1753: f3 0f 58 e1                 	addss	%xmm1, %xmm4
1407c1757: 0f 12 c9                    	movhlps	%xmm1, %xmm1            # xmm1 = xmm1[1,1]
1407c175a: f3 0f 58 cc                 	addss	%xmm4, %xmm1
1407c175e: f3 0f 58 c9                 	addss	%xmm1, %xmm1
1407c1762: 0f c6 c9 00                 	shufps	$0x0, %xmm1, %xmm1      # xmm1 = xmm1[0,0,0,0]
1407c1766: 41 0f 59 cf                 	mulps	%xmm15, %xmm1
1407c176a: 0f 58 cb                    	addps	%xmm3, %xmm1
1407c176d: 0f 58 ca                    	addps	%xmm2, %xmm1
1407c1770: f3 45 0f 58 f5              	addss	%xmm13, %xmm14
1407c1775: f3 44 0f 11 70 14           	movss	%xmm14, 0x14(%rax)
1407c177b: 0f c6 c9 e8                 	shufps	$0xe8, %xmm1, %xmm1     # xmm1 = xmm1[0,2,2,3]
1407c177f: 0f 13 48 18                 	movlps	%xmm1, 0x18(%rax)
1407c1783: 8b 85 1c 02 00 00           	movl	0x21c(%rbp), %eax
1407c1789: 89 47 20                    	movl	%eax, 0x20(%rdi)
1407c178c: f3 0f 11 47 24              	movss	%xmm0, 0x24(%rdi)
1407c1791: 49 8b 8d 18 01 00 00        	movq	0x118(%r13), %rcx
1407c1798: 48 85 c9                    	testq	%rcx, %rcx
1407c179b: 74 10                       	je	0x1407c17ad <.text+0x7c07ad>
1407c179d: ba 28 00 00 00              	movl	$0x28, %edx
1407c17a2: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1407c17a8: e8 73 24 e5 ff              	callq	0x140613c20 <.text+0x612c20>
1407c17ad: 49 89 bd 18 01 00 00        	movq	%rdi, 0x118(%r13)
1407c17b4: 44 89 f1                    	movl	%r14d, %ecx
1407c17b7: e9 2d f8 ff ff              	jmp	0x1407c0fe9 <.text+0x7bffe9>
1407c17bc: 8b 85 60 02 00 00           	movl	0x260(%rbp), %eax
1407c17c2: 41 89 c2                    	movl	%eax, %r10d
1407c17c5: 48 8b 95 50 02 00 00        	movq	0x250(%rbp), %rdx
1407c17cc: 4c 8b 8d 28 02 00 00        	movq	0x228(%rbp), %r9
1407c17d3: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407c17da: 48 89 95 50 02 00 00        	movq	%rdx, 0x250(%rbp)
1407c17e1: 49 83 f9 02                 	cmpq	$0x2, %r9
1407c17e5: 0f 85 af f5 ff ff           	jne	0x1407c0d9a <.text+0x7bfd9a>
1407c17eb: e9 50 f7 ff ff              	jmp	0x1407c0f40 <.text+0x7bff40>
1407c17f0: 48 8d 0d f1 fa 45 02        	leaq	0x245faf1(%rip), %rcx   # 0x142c212e8
1407c17f7: e8 14 7b 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c17fc: e9 23 01 00 00              	jmp	0x1407c1924 <.text+0x7c0924>
1407c1801: 48 85 c0                    	testq	%rax, %rax
1407c1804: 0f 85 8f 00 00 00           	jne	0x1407c1899 <.text+0x7c0899>
1407c180a: 31 d2                       	xorl	%edx, %edx
1407c180c: e8 6f 0d 13 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c1811: 48 89 c1                    	movq	%rax, %rcx
1407c1814: 48 85 c0                    	testq	%rax, %rax
1407c1817: 0f 85 6f f3 ff ff           	jne	0x1407c0b8c <.text+0x7bfb8c>
1407c181d: eb 7a                       	jmp	0x1407c1899 <.text+0x7c0899>
1407c181f: 48 85 c0                    	testq	%rax, %rax
1407c1822: 75 75                       	jne	0x1407c1899 <.text+0x7c0899>
1407c1824: 31 d2                       	xorl	%edx, %edx
1407c1826: e8 55 0d 13 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c182b: 48 89 c1                    	movq	%rax, %rcx
1407c182e: 48 85 c0                    	testq	%rax, %rax
1407c1831: 0f 85 ee f3 ff ff           	jne	0x1407c0c25 <.text+0x7bfc25>
1407c1837: eb 60                       	jmp	0x1407c1899 <.text+0x7c0899>
1407c1839: 48 83 bd 08 02 00 00 00     	cmpq	$0x0, 0x208(%rbp)
1407c1841: 0f 84 0c f4 ff ff           	je	0x1407c0c53 <.text+0x7bfc53>
1407c1847: e8 e4 68 45 00              	callq	0x140c18130 <.text+0xc17130>
1407c184c: 48 89 c1                    	movq	%rax, %rcx
1407c184f: 48 8b 00                    	movq	(%rax), %rax
1407c1852: 48 83 f8 01                 	cmpq	$0x1, %rax
1407c1856: 75 2d                       	jne	0x1407c1885 <.text+0x7c0885>
1407c1858: 48 83 c1 08                 	addq	$0x8, %rcx
1407c185c: 48 83 39 00                 	cmpq	$0x0, (%rcx)
1407c1860: 75 43                       	jne	0x1407c18a5 <.text+0x7c08a5>
1407c1862: 48 c7 01 ff ff ff ff        	movq	$-0x1, (%rcx)
1407c1869: 48 89 8d 58 02 00 00        	movq	%rcx, 0x258(%rbp)
1407c1870: 48 83 c1 08                 	addq	$0x8, %rcx
1407c1874: 48 8b 95 10 02 00 00        	movq	0x210(%rbp), %rdx
1407c187b: e8 70 58 45 00              	callq	0x140c170f0 <.text+0xc160f0>
1407c1880: e9 c4 f3 ff ff              	jmp	0x1407c0c49 <.text+0x7bfc49>
1407c1885: 48 85 c0                    	testq	%rax, %rax
1407c1888: 75 0f                       	jne	0x1407c1899 <.text+0x7c0899>
1407c188a: 31 d2                       	xorl	%edx, %edx
1407c188c: e8 ef 0c 13 02              	callq	0x1428f2580 <.text+0x28f1580>
1407c1891: 48 89 c1                    	movq	%rax, %rcx
1407c1894: 48 85 c0                    	testq	%rax, %rax
1407c1897: 75 c3                       	jne	0x1407c185c <.text+0x7c085c>
1407c1899: 48 8d 0d 28 fb 45 02        	leaq	0x245fb28(%rip), %rcx   # 0x142c213c8
1407c18a0: e8 2b 68 11 02              	callq	0x1428d80d0 <.text+0x28d70d0>
1407c18a5: 48 8d 0d 8c fb 45 02        	leaq	0x245fb8c(%rip), %rcx   # 0x142c21438
1407c18ac: e8 9f 79 11 02              	callq	0x1428d9250 <.text+0x28d8250>
1407c18b1: 48 8d 05 70 fe 2c 02        	leaq	0x22cfe70(%rip), %rax   # 0x142a91728
1407c18b8: 48 89 85 40 01 00 00        	movq	%rax, 0x140(%rbp)
1407c18bf: 48 c7 85 48 01 00 00 01 00 00 00    	movq	$0x1, 0x148(%rbp)
1407c18ca: 48 c7 85 50 01 00 00 08 00 00 00    	movq	$0x8, 0x150(%rbp)
1407c18d5: 66 0f ef c0                 	pxor	%xmm0, %xmm0
1407c18d9: f3 0f 7f 85 58 01 00 00     	movdqu	%xmm0, 0x158(%rbp)
1407c18e1: 48 8d 15 50 fe 2c 02        	leaq	0x22cfe50(%rip), %rdx   # 0x142a91738
1407c18e8: 48 8d 8d 40 01 00 00        	leaq	0x140(%rbp), %rcx
1407c18ef: e8 9c 7a 11 02              	callq	0x1428d9390 <.text+0x28d8390>
1407c18f4: eb 2e                       	jmp	0x1407c1924 <.text+0x7c0924>
1407c18f6: b9 04 00 00 00              	movl	$0x4, %ecx
1407c18fb: ba 28 00 00 00              	movl	$0x28, %edx
1407c1900: e8 fb 76 11 02              	callq	0x1428d9000 <.text+0x28d8000>
1407c1905: eb 1d                       	jmp	0x1407c1924 <.text+0x7c0924>
1407c1907: b9 10 00 00 00              	movl	$0x10, %ecx
1407c190c: ba 90 00 00 00              	movl	$0x90, %edx
1407c1911: e8 ea 76 11 02              	callq	0x1428d9000 <.text+0x28d8000>
1407c1916: eb 0c                       	jmp	0x1407c1924 <.text+0x7c0924>
1407c1918: 48 8d 0d 39 49 2c 02        	leaq	0x22c4939(%rip), %rcx   # 0x142a86258
1407c191f: e8 ec 79 11 02              	callq	0x1428d9310 <.text+0x28d8310>
1407c1924: 0f 0b                       	ud2
1407c1926: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
1407c1930: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c1935: 55                          	pushq	%rbp
1407c1936: 41 57                       	pushq	%r15
1407c1938: 41 56                       	pushq	%r14
1407c193a: 41 55                       	pushq	%r13
1407c193c: 41 54                       	pushq	%r12
1407c193e: 56                          	pushq	%rsi
1407c193f: 57                          	pushq	%rdi
1407c1940: 53                          	pushq	%rbx
1407c1941: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c1948: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c194f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c1955: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c195b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c1961: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c1967: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c196d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c1976: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c197f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c1988: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c1990: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c1998: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1407c199f: 48 ff 00                    	incq	(%rax)
1407c19a2: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c19aa: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c19b2: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c19bb: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c19c4: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c19cd: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c19d3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c19d9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c19df: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c19e5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c19eb: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c19f2: 5b                          	popq	%rbx
1407c19f3: 5f                          	popq	%rdi
1407c19f4: 5e                          	popq	%rsi
1407c19f5: 41 5c                       	popq	%r12
1407c19f7: 41 5d                       	popq	%r13
1407c19f9: 41 5e                       	popq	%r14
1407c19fb: 41 5f                       	popq	%r15
1407c19fd: 5d                          	popq	%rbp
1407c19fe: c3                          	retq
1407c19ff: 90                          	nop
1407c1a00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c1a05: 55                          	pushq	%rbp
1407c1a06: 41 57                       	pushq	%r15
1407c1a08: 41 56                       	pushq	%r14
1407c1a0a: 41 55                       	pushq	%r13
1407c1a0c: 41 54                       	pushq	%r12
1407c1a0e: 56                          	pushq	%rsi
1407c1a0f: 57                          	pushq	%rdi
1407c1a10: 53                          	pushq	%rbx
1407c1a11: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c1a18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c1a1f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c1a25: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c1a2b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c1a31: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c1a37: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c1a3d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c1a46: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c1a4f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c1a58: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c1a60: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c1a68: 48 8b 85 58 02 00 00        	movq	0x258(%rbp), %rax
1407c1a6f: 48 ff 00                    	incq	(%rax)
1407c1a72: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c1a7a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c1a82: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c1a8b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c1a94: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c1a9d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c1aa3: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c1aa9: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c1aaf: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c1ab5: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c1abb: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c1ac2: 5b                          	popq	%rbx
1407c1ac3: 5f                          	popq	%rdi
1407c1ac4: 5e                          	popq	%rsi
1407c1ac5: 41 5c                       	popq	%r12
1407c1ac7: 41 5d                       	popq	%r13
1407c1ac9: 41 5e                       	popq	%r14
1407c1acb: 41 5f                       	popq	%r15
1407c1acd: 5d                          	popq	%rbp
1407c1ace: c3                          	retq
1407c1acf: 90                          	nop
1407c1ad0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c1ad5: 55                          	pushq	%rbp
1407c1ad6: 41 57                       	pushq	%r15
1407c1ad8: 41 56                       	pushq	%r14
1407c1ada: 41 55                       	pushq	%r13
1407c1adc: 41 54                       	pushq	%r12
1407c1ade: 56                          	pushq	%rsi
1407c1adf: 57                          	pushq	%rdi
1407c1ae0: 53                          	pushq	%rbx
1407c1ae1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c1ae8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c1aef: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c1af5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c1afb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c1b01: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c1b07: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c1b0d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c1b16: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c1b1f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c1b28: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c1b30: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c1b38: 48 8d 8d 50 01 00 00        	leaq	0x150(%rbp), %rcx
1407c1b3f: e8 9c 00 e4 ff              	callq	0x140601be0 <.text+0x600be0>
1407c1b44: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c1b4c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c1b54: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c1b5d: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c1b66: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c1b6f: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c1b75: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c1b7b: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c1b81: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c1b87: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c1b8d: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c1b94: 5b                          	popq	%rbx
1407c1b95: 5f                          	popq	%rdi
1407c1b96: 5e                          	popq	%rsi
1407c1b97: 41 5c                       	popq	%r12
1407c1b99: 41 5d                       	popq	%r13
1407c1b9b: 41 5e                       	popq	%r14
1407c1b9d: 41 5f                       	popq	%r15
1407c1b9f: 5d                          	popq	%rbp
1407c1ba0: c3                          	retq
1407c1ba1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1407c1bb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c1bb5: 55                          	pushq	%rbp
1407c1bb6: 41 57                       	pushq	%r15
1407c1bb8: 41 56                       	pushq	%r14
1407c1bba: 41 55                       	pushq	%r13
1407c1bbc: 41 54                       	pushq	%r12
1407c1bbe: 56                          	pushq	%rsi
1407c1bbf: 57                          	pushq	%rdi
1407c1bc0: 53                          	pushq	%rbx
1407c1bc1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c1bc8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c1bcf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c1bd5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c1bdb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c1be1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c1be7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c1bed: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c1bf6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c1bff: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c1c08: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c1c10: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c1c18: 48 8b 85 38 02 00 00        	movq	0x238(%rbp), %rax
1407c1c1f: 48 ff 00                    	incq	(%rax)
1407c1c22: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c1c2a: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c1c32: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c1c3b: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c1c44: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c1c4d: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c1c53: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c1c59: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c1c5f: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c1c65: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c1c6b: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c1c72: 5b                          	popq	%rbx
1407c1c73: 5f                          	popq	%rdi
1407c1c74: 5e                          	popq	%rsi
1407c1c75: 41 5c                       	popq	%r12
1407c1c77: 41 5d                       	popq	%r13
1407c1c79: 41 5e                       	popq	%r14
1407c1c7b: 41 5f                       	popq	%r15
1407c1c7d: 5d                          	popq	%rbp
1407c1c7e: c3                          	retq
1407c1c7f: 90                          	nop
1407c1c80: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1407c1c85: 55                          	pushq	%rbp
1407c1c86: 41 57                       	pushq	%r15
1407c1c88: 41 56                       	pushq	%r14
1407c1c8a: 41 55                       	pushq	%r13
1407c1c8c: 41 54                       	pushq	%r12
1407c1c8e: 56                          	pushq	%rsi
1407c1c8f: 57                          	pushq	%rdi
1407c1c90: 53                          	pushq	%rbx
1407c1c91: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1407c1c98: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1407c1c9f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1407c1ca5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1407c1cab: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1407c1cb1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1407c1cb7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1407c1cbd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1407c1cc6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1407c1ccf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1407c1cd8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1407c1ce0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1407c1ce8: 48 83 bd 08 02 00 00 00     	cmpq	$0x0, 0x208(%rbp)
1407c1cf0: 74 29                       	je	0x1407c1d1b <.text+0x7c0d1b>
1407c1cf2: 48 8d 85 10 02 00 00        	leaq	0x210(%rbp), %rax
1407c1cf9: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
1407c1d00: 48 8d 85 20 01 00 00        	leaq	0x120(%rbp), %rax
1407c1d07: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
1407c1d0b: 48 8d 0d 0e 44 2c 02        	leaq	0x22c440e(%rip), %rcx   # 0x142a86120
1407c1d12: 48 8d 55 78                 	leaq	0x78(%rbp), %rdx
1407c1d16: e8 a5 5e a9 00              	callq	0x141257bc0 <.text+0x1256bc0>
1407c1d1b: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1407c1d23: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1407c1d2b: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1407c1d34: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1407c1d3d: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1407c1d46: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1407c1d4c: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1407c1d52: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1407c1d58: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1407c1d5e: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1407c1d64: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1407c1d6b: 5b                          	popq	%rbx
1407c1d6c: 5f                          	popq	%rdi
1407c1d6d: 5e                          	popq	%rsi
1407c1d6e: 41 5c                       	popq	%r12
1407c1d70: 41 5d                       	popq	%r13
1407c1d72: 41 5e                       	popq	%r14
1407c1d74: 41 5f                       	popq	%r15
1407c1d76: 5d                          	popq	%rbp
1407c1d77: c3                          	retq
1407c1d78: cc                          	int3
1407c1d79: cc                          	int3
1407c1d7a: cc                          	int3
1407c1d7b: cc                          	int3
1407c1d7c: cc                          	int3
1407c1d7d: cc                          	int3
1407c1d7e: cc                          	int3
1407c1d7f: cc                          	int3
1407c1d80: 56                          	pushq	%rsi
1407c1d81: 57                          	pushq	%rdi
1407c1d82: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
1407c1d89: 49 89 c8                    	movq	%rcx, %r8
1407c1d8c: 48 c7 44 24 20 02 00 00 00  	movq	$0x2, 0x20(%rsp)
1407c1d95: 48 8d 15 bc 46 2c 02        	leaq	0x22c46bc(%rip), %rdx   # 0x142a86458
1407c1d9c: 4c 8d 0d 7d 47 2c 02        	leaq	0x22c477d(%rip), %r9    # 0x142a86520
1407c1da3: 48 8d 4c 24 50              	leaq	0x50(%rsp), %rcx
1407c1da8: e8 93 7b 5c 00              	callq	0x140d89940 <.text+0xd88940>
1407c1dad: f6 44 24 50 01              	testb	$0x1, 0x50(%rsp)
1407c1db2: 74 48                       	je	0x1407c1dfc <.text+0x7c0dfc>
1407c1db4: 48 8d 54 24 58              	leaq	0x58(%rsp), %rdx
1407c1db9: 48 83 bc 24 88 00 00 00 ff  	cmpq	$-0x1, 0x88(%rsp)
1407c1dc2: 4c 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %r8
1407c1dca: 4c 8b 8c 24 a0 00 00 00     	movq	0xa0(%rsp), %r9
1407c1dd2: 48 8b 84 24 a8 00 00 00     	movq	0xa8(%rsp), %rax
1407c1dda: 48 8b 8c 24 b0 00 00 00     	movq	0xb0(%rsp), %rcx
1407c1de2: 0f 84 2b 01 00 00           	je	0x1407c1f13 <.text+0x7c0f13>
1407c1de8: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1407c1ded: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c1df2: c6 44 24 30 00              	movb	$0x0, 0x30(%rsp)
1407c1df7: e9 26 01 00 00              	jmp	0x1407c1f22 <.text+0x7c0f22>
1407c1dfc: 80 7c 24 6a 00              	cmpb	$0x0, 0x6a(%rsp)
1407c1e01: 74 08                       	je	0x1407c1e0b <.text+0x7c0e0b>
1407c1e03: 45 31 c9                    	xorl	%r9d, %r9d
1407c1e06: e9 37 01 00 00              	jmp	0x1407c1f42 <.text+0x7c0f42>
1407c1e0b: 4c 8b 4c 24 60              	movq	0x60(%rsp), %r9
1407c1e10: 44 0f b6 44 24 69           	movzbl	0x69(%rsp), %r8d
1407c1e16: 4d 85 c9                    	testq	%r9, %r9
1407c1e19: 0f 84 0f 01 00 00           	je	0x1407c1f2e <.text+0x7c0f2e>
1407c1e1f: 48 8b 8c 24 98 00 00 00     	movq	0x98(%rsp), %rcx
1407c1e27: 48 8b 94 24 a0 00 00 00     	movq	0xa0(%rsp), %rdx
1407c1e2f: 44 89 c0                    	movl	%r8d, %eax
1407c1e32: eb 25                       	jmp	0x1407c1e59 <.text+0x7c0e59>
1407c1e34: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
1407c1e40: 41 89 c2                    	movl	%eax, %r10d
1407c1e43: 41 80 f2 01                 	xorb	$0x1, %r10b
1407c1e47: 3c 01                       	cmpb	$0x1, %al
1407c1e49: 41 0f 95 c0                 	setne	%r8b
1407c1e4d: 44 89 d0                    	movl	%r10d, %eax
1407c1e50: 4d 01 d9                    	addq	%r11, %r9
1407c1e53: 0f 84 d5 00 00 00           	je	0x1407c1f2e <.text+0x7c0f2e>
1407c1e59: 49 39 d1                    	cmpq	%rdx, %r9
1407c1e5c: 73 12                       	jae	0x1407c1e70 <.text+0x7c0e70>
1407c1e5e: 42 80 3c 09 bf              	cmpb	$-0x41, (%rcx,%r9)
1407c1e63: 7f 11                       	jg	0x1407c1e76 <.text+0x7c0e76>
1407c1e65: e9 f1 00 00 00              	jmp	0x1407c1f5b <.text+0x7c0f5b>
1407c1e6a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1407c1e70: 0f 85 e5 00 00 00           	jne	0x1407c1f5b <.text+0x7c0f5b>
1407c1e76: 46 0f be 54 09 ff           	movsbl	-0x1(%rcx,%r9), %r10d
1407c1e7c: 45 85 d2                    	testl	%r10d, %r10d
1407c1e7f: 79 4d                       	jns	0x1407c1ece <.text+0x7c0ece>
1407c1e81: 46 0f b6 5c 09 fe           	movzbl	-0x2(%rcx,%r9), %r11d
1407c1e87: 41 80 fb c0                 	cmpb	$-0x40, %r11b
1407c1e8b: 7d 1f                       	jge	0x1407c1eac <.text+0x7c0eac>
1407c1e8d: 42 0f b6 74 09 fd           	movzbl	-0x3(%rcx,%r9), %esi
1407c1e93: 40 80 fe c0                 	cmpb	$-0x40, %sil
1407c1e97: 7d 19                       	jge	0x1407c1eb2 <.text+0x7c0eb2>
1407c1e99: 42 0f b6 7c 09 fc           	movzbl	-0x4(%rcx,%r9), %edi
1407c1e9f: 83 e7 07                    	andl	$0x7, %edi
1407c1ea2: c1 e7 06                    	shll	$0x6, %edi
1407c1ea5: 83 e6 3f                    	andl	$0x3f, %esi
1407c1ea8: 09 fe                       	orl	%edi, %esi
1407c1eaa: eb 09                       	jmp	0x1407c1eb5 <.text+0x7c0eb5>
1407c1eac: 41 83 e3 1f                 	andl	$0x1f, %r11d
1407c1eb0: eb 0d                       	jmp	0x1407c1ebf <.text+0x7c0ebf>
1407c1eb2: 83 e6 0f                    	andl	$0xf, %esi
1407c1eb5: c1 e6 06                    	shll	$0x6, %esi
1407c1eb8: 41 83 e3 3f                 	andl	$0x3f, %r11d
1407c1ebc: 41 09 f3                    	orl	%esi, %r11d
1407c1ebf: 41 c1 e3 06                 	shll	$0x6, %r11d
1407c1ec3: 41 80 e2 3f                 	andb	$0x3f, %r10b
1407c1ec7: 45 0f b6 d2                 	movzbl	%r10b, %r10d
1407c1ecb: 45 09 da                    	orl	%r11d, %r10d
1407c1ece: 41 f6 c0 01                 	testb	$0x1, %r8b
1407c1ed2: 75 63                       	jne	0x1407c1f37 <.text+0x7c0f37>
1407c1ed4: 49 c7 c3 ff ff ff ff        	movq	$-0x1, %r11
1407c1edb: 41 81 fa 80 00 00 00        	cmpl	$0x80, %r10d
1407c1ee2: 0f 82 58 ff ff ff           	jb	0x1407c1e40 <.text+0x7c0e40>
1407c1ee8: 49 c7 c3 fe ff ff ff        	movq	$-0x2, %r11
1407c1eef: 41 81 fa 00 08 00 00        	cmpl	$0x800, %r10d           # imm = 0x800
1407c1ef6: 0f 82 44 ff ff ff           	jb	0x1407c1e40 <.text+0x7c0e40>
1407c1efc: 45 31 db                    	xorl	%r11d, %r11d
1407c1eff: 41 81 fa 00 00 01 00        	cmpl	$0x10000, %r10d         # imm = 0x10000
1407c1f06: 41 0f 92 c3                 	setb	%r11b
1407c1f0a: 49 83 cb fc                 	orq	$-0x4, %r11
1407c1f0e: e9 2d ff ff ff              	jmp	0x1407c1e40 <.text+0x7c0e40>
1407c1f13: 48 89 4c 24 28              	movq	%rcx, 0x28(%rsp)
1407c1f18: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c1f1d: c6 44 24 30 01              	movb	$0x1, 0x30(%rsp)
1407c1f22: 48 8d 4c 24 38              	leaq	0x38(%rsp), %rcx
1407c1f27: e8 44 00 00 00              	callq	0x1407c1f70 <.text+0x7c0f70>
1407c1f2c: eb 19                       	jmp	0x1407c1f47 <.text+0x7c0f47>
1407c1f2e: 45 31 c9                    	xorl	%r9d, %r9d
1407c1f31: 41 f6 c0 01                 	testb	$0x1, %r8b
1407c1f35: 74 0b                       	je	0x1407c1f42 <.text+0x7c0f42>
1407c1f37: 4c 89 4c 24 40              	movq	%r9, 0x40(%rsp)
1407c1f3c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1407c1f42: 4c 89 4c 24 38              	movq	%r9, 0x38(%rsp)
1407c1f47: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
1407c1f4c: 48 8b 54 24 40              	movq	0x40(%rsp), %rdx
1407c1f51: 48 81 c4 b8 00 00 00        	addq	$0xb8, %rsp
1407c1f58: 5f                          	popq	%rdi
1407c1f59: 5e                          	popq	%rsi
1407c1f5a: c3                          	retq
1407c1f5b: 48 8d 05 f6 45 2c 02        	leaq	0x22c45f6(%rip), %rax   # 0x142a86558
1407c1f62: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1407c1f67: 45 31 c0                    	xorl	%r8d, %r8d
1407c1f6a: e8 d1 79 11 02              	callq	0x1428d9940 <.text+0x28d8940>
1407c1f6f: cc                          	int3
1407c1f70: 41 57                       	pushq	%r15
1407c1f72: 41 56                       	pushq	%r14
1407c1f74: 41 55                       	pushq	%r13
1407c1f76: 41 54                       	pushq	%r12
1407c1f78: 56                          	pushq	%rsi
1407c1f79: 57                          	pushq	%rdi
1407c1f7a: 55                          	pushq	%rbp
1407c1f7b: 53                          	pushq	%rbx
1407c1f7c: 48 83 ec 48                 	subq	$0x48, %rsp
1407c1f80: 48 89 d0                    	movq	%rdx, %rax
1407c1f83: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
1407c1f88: 48 8b 94 24 b8 00 00 00     	movq	0xb8(%rsp), %rdx
1407c1f90: 48 8b 68 28                 	movq	0x28(%rax), %rbp
1407c1f94: 49 89 ea                    	movq	%rbp, %r10
1407c1f97: 49 29 d2                    	subq	%rdx, %r10
1407c1f9a: 4d 39 ca                    	cmpq	%r9, %r10
1407c1f9d: 0f 83 6f 01 00 00           	jae	0x1407c2112 <.text+0x7c1112>
1407c1fa3: 44 0f b6 b4 24 c0 00 00 00  	movzbl	0xc0(%rsp), %r14d
1407c1fac: 48 8b bc 24 b0 00 00 00     	movq	0xb0(%rsp), %rdi
1407c1fb4: 48 8b 58 18                 	movq	0x18(%rax), %rbx
1407c1fb8: 48 8b 48 08                 	movq	0x8(%rax), %rcx
1407c1fbc: 4c 8b 58 10                 	movq	0x10(%rax), %r11
1407c1fc0: 4c 89 5c 24 38              	movq	%r11, 0x38(%rsp)
1407c1fc5: 4c 8b 68 38                 	movq	0x38(%rax), %r13
1407c1fc9: 4c 8d 5f ff                 	leaq	-0x1(%rdi), %r11
1407c1fcd: 4c 89 5c 24 40              	movq	%r11, 0x40(%rsp)
1407c1fd2: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1407c1fd7: eb 1d                       	jmp	0x1407c1ff6 <.text+0x7c0ff6>
1407c1fd9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407c1fe0: 48 89 70 38                 	movq	%rsi, 0x38(%rax)
1407c1fe4: 49 89 f5                    	movq	%rsi, %r13
1407c1fe7: 4c 89 d5                    	movq	%r10, %rbp
1407c1fea: 49 29 d2                    	subq	%rdx, %r10
1407c1fed: 4d 39 ca                    	cmpq	%r9, %r10
1407c1ff0: 0f 83 1c 01 00 00           	jae	0x1407c2112 <.text+0x7c1112>
1407c1ff6: 47 0f b6 1c 10              	movzbl	(%r8,%r10), %r11d
1407c1ffb: 4c 0f a3 db                 	btq	%r11, %rbx
1407c1fff: 73 7f                       	jae	0x1407c2080 <.text+0x7c1080>
1407c2001: 49 39 cd                    	cmpq	%rcx, %r13
1407c2004: 49 89 cb                    	movq	%rcx, %r11
1407c2007: 4d 0f 42 dd                 	cmovbq	%r13, %r11
1407c200b: 44 89 f3                    	movl	%r14d, %ebx
1407c200e: 45 84 f6                    	testb	%r14b, %r14b
1407c2011: 4c 0f 45 d9                 	cmovneq	%rcx, %r11
1407c2015: 49 8d 73 ff                 	leaq	-0x1(%r11), %rsi
1407c2019: 48 39 d6                    	cmpq	%rdx, %rsi
1407c201c: 0f 83 7e 00 00 00           	jae	0x1407c20a0 <.text+0x7c10a0>
1407c2022: 4d 89 dc                    	movq	%r11, %r12
1407c2025: 49 f7 dc                    	negq	%r12
1407c2028: 48 8b 74 24 40              	movq	0x40(%rsp), %rsi
1407c202d: 4e 8d 3c 1e                 	leaq	(%rsi,%r11), %r15
1407c2031: 4b 8d 34 13                 	leaq	(%r11,%r10), %rsi
1407c2035: 48 ff ce                    	decq	%rsi
1407c2038: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
1407c2040: 4d 85 e4                    	testq	%r12, %r12
1407c2043: 74 64                       	je	0x1407c20a9 <.text+0x7c10a9>
1407c2045: 4c 39 ce                    	cmpq	%r9, %rsi
1407c2048: 0f 83 36 01 00 00           	jae	0x1407c2184 <.text+0x7c1184>
1407c204e: 45 0f b6 1f                 	movzbl	(%r15), %r11d
1407c2052: 49 ff c4                    	incq	%r12
1407c2055: 49 ff cf                    	decq	%r15
1407c2058: 4c 8d 76 ff                 	leaq	-0x1(%rsi), %r14
1407c205c: 45 3a 1c 30                 	cmpb	(%r8,%rsi), %r11b
1407c2060: 4c 89 f6                    	movq	%r14, %rsi
1407c2063: 74 db                       	je	0x1407c2040 <.text+0x7c1040>
1407c2065: 48 29 cd                    	subq	%rcx, %rbp
1407c2068: 4c 29 e5                    	subq	%r12, %rbp
1407c206b: 48 89 68 28                 	movq	%rbp, 0x28(%rax)
1407c206f: 48 89 d6                    	movq	%rdx, %rsi
1407c2072: e9 83 00 00 00              	jmp	0x1407c20fa <.text+0x7c10fa>
1407c2077: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1407c2080: 4c 89 50 28                 	movq	%r10, 0x28(%rax)
1407c2084: 48 89 d6                    	movq	%rdx, %rsi
1407c2087: 45 84 f6                    	testb	%r14b, %r14b
1407c208a: 0f 84 50 ff ff ff           	je	0x1407c1fe0 <.text+0x7c0fe0>
1407c2090: e9 52 ff ff ff              	jmp	0x1407c1fe7 <.text+0x7c0fe7>
1407c2095: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1407c20a0: 4d 85 db                    	testq	%r11, %r11
1407c20a3: 0f 85 ed 00 00 00           	jne	0x1407c2196 <.text+0x7c1196>
1407c20a9: 84 db                       	testb	%bl, %bl
1407c20ab: 4c 89 ee                    	movq	%r13, %rsi
1407c20ae: 48 0f 45 f2                 	cmovneq	%rdx, %rsi
1407c20b2: 4f 8d 3c 10                 	leaq	(%r8,%r10), %r15
1407c20b6: 49 89 cc                    	movq	%rcx, %r12
1407c20b9: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1407c20c0: 49 39 f4                    	cmpq	%rsi, %r12
1407c20c3: 73 5e                       	jae	0x1407c2123 <.text+0x7c1123>
1407c20c5: 49 39 d4                    	cmpq	%rdx, %r12
1407c20c8: 0f 83 87 00 00 00           	jae	0x1407c2155 <.text+0x7c1155>
1407c20ce: 4f 8d 1c 22                 	leaq	(%r10,%r12), %r11
1407c20d2: 4d 39 cb                    	cmpq	%r9, %r11
1407c20d5: 0f 83 8d 00 00 00           	jae	0x1407c2168 <.text+0x7c1168>
1407c20db: 46 0f b6 1c 27              	movzbl	(%rdi,%r12), %r11d
1407c20e0: 4d 8d 74 24 01              	leaq	0x1(%r12), %r14
1407c20e5: 47 3a 1c 27                 	cmpb	(%r15,%r12), %r11b
1407c20e9: 4d 89 f4                    	movq	%r14, %r12
1407c20ec: 74 d2                       	je	0x1407c20c0 <.text+0x7c10c0>
1407c20ee: 48 8b 74 24 38              	movq	0x38(%rsp), %rsi
1407c20f3: 48 29 f5                    	subq	%rsi, %rbp
1407c20f6: 48 89 68 28                 	movq	%rbp, 0x28(%rax)
1407c20fa: 49 89 ea                    	movq	%rbp, %r10
1407c20fd: 41 89 de                    	movl	%ebx, %r14d
1407c2100: 84 db                       	testb	%bl, %bl
1407c2102: 48 8b 5c 24 28              	movq	0x28(%rsp), %rbx
1407c2107: 0f 84 d3 fe ff ff           	je	0x1407c1fe0 <.text+0x7c0fe0>
1407c210d: e9 d5 fe ff ff              	jmp	0x1407c1fe7 <.text+0x7c0fe7>
1407c2112: 48 c7 40 28 00 00 00 00     	movq	$0x0, 0x28(%rax)
1407c211a: 31 c0                       	xorl	%eax, %eax
1407c211c: 48 8b 4c 24 30              	movq	0x30(%rsp), %rcx
1407c2121: eb 1e                       	jmp	0x1407c2141 <.text+0x7c1141>
1407c2123: 4c 89 50 28                 	movq	%r10, 0x28(%rax)
1407c2127: 84 db                       	testb	%bl, %bl
1407c2129: 75 04                       	jne	0x1407c212f <.text+0x7c112f>
1407c212b: 48 89 50 38                 	movq	%rdx, 0x38(%rax)
1407c212f: 48 8b 4c 24 30              	movq	0x30(%rsp), %rcx
1407c2134: 4c 89 51 08                 	movq	%r10, 0x8(%rcx)
1407c2138: 48 89 69 10                 	movq	%rbp, 0x10(%rcx)
1407c213c: b8 01 00 00 00              	movl	$0x1, %eax
1407c2141: 48 89 01                    	movq	%rax, (%rcx)
1407c2144: 48 83 c4 48                 	addq	$0x48, %rsp
1407c2148: 5b                          	popq	%rbx
1407c2149: 5d                          	popq	%rbp
1407c214a: 5f                          	popq	%rdi
1407c214b: 5e                          	popq	%rsi
1407c214c: 41 5c                       	popq	%r12
1407c214e: 41 5d                       	popq	%r13
1407c2150: 41 5e                       	popq	%r14
1407c2152: 41 5f                       	popq	%r15
1407c2154: c3                          	retq
1407c2155: 48 39 d1                    	cmpq	%rdx, %rcx
1407c2158: 48 0f 46 ca                 	cmovbeq	%rdx, %rcx
1407c215c: 4c 8d 05 25 42 2c 02        	leaq	0x22c4225(%rip), %r8    # 0x142a86388
1407c2163: e8 b0 73 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c2168: 49 01 ca                    	addq	%rcx, %r10
1407c216b: 4d 39 d1                    	cmpq	%r10, %r9
1407c216e: 4d 0f 47 d1                 	cmovaq	%r9, %r10
1407c2172: 4c 8d 05 27 42 2c 02        	leaq	0x22c4227(%rip), %r8    # 0x142a863a0
1407c2179: 4c 89 d1                    	movq	%r10, %rcx
1407c217c: 4c 89 ca                    	movq	%r9, %rdx
1407c217f: e8 94 73 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c2184: 4c 8d 05 45 42 2c 02        	leaq	0x22c4245(%rip), %r8    # 0x142a863d0
1407c218b: 48 89 f1                    	movq	%rsi, %rcx
1407c218e: 4c 89 ca                    	movq	%r9, %rdx
1407c2191: e8 82 73 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c2196: 4c 8d 05 1b 42 2c 02        	leaq	0x22c421b(%rip), %r8    # 0x142a863b8
1407c219d: 48 89 f1                    	movq	%rsi, %rcx
1407c21a0: e8 73 73 11 02              	callq	0x1428d9518 <.text+0x28d8518>
1407c21a5: cc                          	int3
1407c21a6: cc                          	int3
1407c21a7: cc                          	int3
1407c21a8: cc                          	int3
1407c21a9: cc                          	int3
1407c21aa: cc                          	int3
1407c21ab: cc                          	int3
1407c21ac: cc                          	int3
1407c21ad: cc                          	int3
1407c21ae: cc                          	int3
1407c21af: cc                          	int3
