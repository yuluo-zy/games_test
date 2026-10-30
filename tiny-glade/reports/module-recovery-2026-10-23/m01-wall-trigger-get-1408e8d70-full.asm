
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1408e8d70: 55                          	pushq	%rbp
1408e8d71: 41 57                       	pushq	%r15
1408e8d73: 41 56                       	pushq	%r14
1408e8d75: 41 55                       	pushq	%r13
1408e8d77: 41 54                       	pushq	%r12
1408e8d79: 56                          	pushq	%rsi
1408e8d7a: 57                          	pushq	%rdi
1408e8d7b: 53                          	pushq	%rbx
1408e8d7c: 48 81 ec 38 03 00 00        	subq	$0x338, %rsp            # imm = 0x338
1408e8d83: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1408e8d8b: 44 0f 29 bd a0 02 00 00     	movaps	%xmm15, 0x2a0(%rbp)
1408e8d93: 44 0f 29 b5 90 02 00 00     	movaps	%xmm14, 0x290(%rbp)
1408e8d9b: 44 0f 29 ad 80 02 00 00     	movaps	%xmm13, 0x280(%rbp)
1408e8da3: 44 0f 29 a5 70 02 00 00     	movaps	%xmm12, 0x270(%rbp)
1408e8dab: 44 0f 29 9d 60 02 00 00     	movaps	%xmm11, 0x260(%rbp)
1408e8db3: 44 0f 29 95 50 02 00 00     	movaps	%xmm10, 0x250(%rbp)
1408e8dbb: 44 0f 29 8d 40 02 00 00     	movaps	%xmm9, 0x240(%rbp)
1408e8dc3: 44 0f 29 85 30 02 00 00     	movaps	%xmm8, 0x230(%rbp)
1408e8dcb: 0f 29 bd 20 02 00 00        	movaps	%xmm7, 0x220(%rbp)
1408e8dd2: 0f 29 b5 10 02 00 00        	movaps	%xmm6, 0x210(%rbp)
1408e8dd9: 48 c7 85 08 02 00 00 fe ff ff ff    	movq	$-0x2, 0x208(%rbp)
1408e8de4: 4c 8b 72 10                 	movq	0x10(%rdx), %r14
1408e8de8: 4d 85 f6                    	testq	%r14, %r14
1408e8deb: 0f 84 63 0d 00 00           	je	0x1408e9b54 <.text+0x8e8b54>
1408e8df1: 4c 89 c6                    	movq	%r8, %rsi
1408e8df4: 48 89 d7                    	movq	%rdx, %rdi
1408e8df7: 48 89 4d 58                 	movq	%rcx, 0x58(%rbp)
1408e8dfb: 4a 8d 04 f5 00 00 00 00     	leaq	(,%r14,8), %rax
1408e8e03: 48 8d 1c c0                 	leaq	(%rax,%rax,8), %rbx
1408e8e07: 0f b6 05 fb bd d4 02        	movzbl	0x2d4bdfb(%rip), %eax   # 0x143634c09
1408e8e0e: ba 04 00 00 00              	movl	$0x4, %edx
1408e8e13: 48 89 d9                    	movq	%rbx, %rcx
1408e8e16: e8 f5 ad d2 ff              	callq	0x140613c10 <.text+0x612c10>
1408e8e1b: 48 85 c0                    	testq	%rax, %rax
1408e8e1e: 0f 84 96 0d 00 00           	je	0x1408e9bba <.text+0x8e8bba>
1408e8e24: 4f 8d 3c 36                 	leaq	(%r14,%r14), %r15
1408e8e28: 4c 89 bd e8 01 00 00        	movq	%r15, 0x1e8(%rbp)
1408e8e2f: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
1408e8e36: 48 c7 85 f8 01 00 00 00 00 00 00    	movq	$0x0, 0x1f8(%rbp)
1408e8e41: 0f b6 05 c1 bd d4 02        	movzbl	0x2d4bdc1(%rip), %eax   # 0x143634c09
1408e8e48: ba 04 00 00 00              	movl	$0x4, %edx
1408e8e4d: 48 89 d9                    	movq	%rbx, %rcx
1408e8e50: e8 bb ad d2 ff              	callq	0x140613c10 <.text+0x612c10>
1408e8e55: 48 85 c0                    	testq	%rax, %rax
1408e8e58: 0f 84 70 0d 00 00           	je	0x1408e9bce <.text+0x8e8bce>
1408e8e5e: 4c 89 bd b8 01 00 00        	movq	%r15, 0x1b8(%rbp)
1408e8e65: 48 89 85 c0 01 00 00        	movq	%rax, 0x1c0(%rbp)
1408e8e6c: 48 c7 85 c8 01 00 00 00 00 00 00    	movq	$0x0, 0x1c8(%rbp)
1408e8e77: 4c 89 f0                    	movq	%r14, %rax
1408e8e7a: 48 c1 e0 04                 	shlq	$0x4, %rax
1408e8e7e: 48 8d 1c 40                 	leaq	(%rax,%rax,2), %rbx
1408e8e82: 0f b6 05 80 bd d4 02        	movzbl	0x2d4bd80(%rip), %eax   # 0x143634c09
1408e8e89: ba 04 00 00 00              	movl	$0x4, %edx
1408e8e8e: 48 89 d9                    	movq	%rbx, %rcx
1408e8e91: e8 7a ad d2 ff              	callq	0x140613c10 <.text+0x612c10>
1408e8e96: 48 85 c0                    	testq	%rax, %rax
1408e8e99: 0f 84 45 0d 00 00           	je	0x1408e9be4 <.text+0x8e8be4>
1408e8e9f: 4c 89 bd a0 01 00 00        	movq	%r15, 0x1a0(%rbp)
1408e8ea6: 48 89 85 a8 01 00 00        	movq	%rax, 0x1a8(%rbp)
1408e8ead: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
1408e8eb8: 4c 8b 7f 08                 	movq	0x8(%rdi), %r15
1408e8ebc: 49 6b c6 78                 	imulq	$0x78, %r14, %rax
1408e8ec0: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
1408e8ec7: f6 46 0c 01                 	testb	$0x1, 0xc(%rsi)
1408e8ecb: 75 05                       	jne	0x1408e8ed2 <.text+0x8e7ed2>
1408e8ecd: 0f 57 c0                    	xorps	%xmm0, %xmm0
1408e8ed0: eb 05                       	jmp	0x1408e8ed7 <.text+0x8e7ed7>
1408e8ed2: f3 0f 10 46 10              	movss	0x10(%rsi), %xmm0
1408e8ed7: f3 0f 10 4e 04              	movss	0x4(%rsi), %xmm1
1408e8edc: f3 0f 11 8d d0 01 00 00     	movss	%xmm1, 0x1d0(%rbp)
1408e8ee4: f3 0f 10 4e 08              	movss	0x8(%rsi), %xmm1
1408e8ee9: f3 0f 11 8d d4 01 00 00     	movss	%xmm1, 0x1d4(%rbp)
1408e8ef1: 4c 01 bd a8 00 00 00        	addq	%r15, 0xa8(%rbp)
1408e8ef8: 0f b6 06                    	movzbl	(%rsi), %eax
1408e8efb: 88 85 06 02 00 00           	movb	%al, 0x206(%rbp)
1408e8f01: 0f b6 46 1c                 	movzbl	0x1c(%rsi), %eax
1408e8f05: 88 85 05 02 00 00           	movb	%al, 0x205(%rbp)
1408e8f0b: 0f b6 47 60                 	movzbl	0x60(%rdi), %eax
1408e8f0f: 88 85 07 02 00 00           	movb	%al, 0x207(%rbp)
1408e8f15: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1408e8f19: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
1408e8f1d: f3 0f 10 46 14              	movss	0x14(%rsi), %xmm0
1408e8f22: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1408e8f26: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
1408e8f2a: f3 0f 10 46 18              	movss	0x18(%rsi), %xmm0
1408e8f2f: f3 0f 12 c0                 	movsldup	%xmm0, %xmm0            # xmm0 = xmm0[0,0,2,2]
1408e8f33: 0f 29 45 e0                 	movaps	%xmm0, -0x20(%rbp)
1408e8f37: 48 8d 9d b8 01 00 00        	leaq	0x1b8(%rbp), %rbx
1408e8f3e: 4c 8d 35 fb d6 1b 02        	leaq	0x21bd6fb(%rip), %r14   # 0x142aa6640
1408e8f45: 48 8d bd a0 01 00 00        	leaq	0x1a0(%rbp), %rdi
1408e8f4c: eb 23                       	jmp	0x1408e8f71 <.text+0x8e7f71>
1408e8f4e: 66 90                       	nop
1408e8f50: 41 0f 2e d2                 	ucomiss	%xmm10, %xmm2
1408e8f54: 0f 85 56 04 00 00           	jne	0x1408e93b0 <.text+0x8e83b0>
1408e8f5a: 0f 8a 50 04 00 00           	jp	0x1408e93b0 <.text+0x8e83b0>
1408e8f60: 49 83 c7 78                 	addq	$0x78, %r15
1408e8f64: 4c 3b bd a8 00 00 00        	cmpq	0xa8(%rbp), %r15
1408e8f6b: 0f 84 ff 09 00 00           	je	0x1408e9970 <.text+0x8e8970>
1408e8f71: 80 bd 05 02 00 00 00        	cmpb	$0x0, 0x205(%rbp)
1408e8f78: 0f 84 c2 00 00 00           	je	0x1408e9040 <.text+0x8e8040>
1408e8f7e: f3 45 0f 10 47 50           	movss	0x50(%r15), %xmm8
1408e8f84: f3 45 0f 10 4f 54           	movss	0x54(%r15), %xmm9
1408e8f8a: 41 0f 28 d0                 	movaps	%xmm8, %xmm2
1408e8f8e: 45 0f 28 d1                 	movaps	%xmm9, %xmm10
1408e8f92: f6 85 06 02 00 00 01        	testb	$0x1, 0x206(%rbp)
1408e8f99: 74 b5                       	je	0x1408e8f50 <.text+0x8e7f50>
1408e8f9b: f3 0f 10 85 d0 01 00 00     	movss	0x1d0(%rbp), %xmm0
1408e8fa3: f3 0f 10 8d d4 01 00 00     	movss	0x1d4(%rbp), %xmm1
1408e8fab: 0f 2e c8                    	ucomiss	%xmm0, %xmm1
1408e8fae: 0f 82 8d 0a 00 00           	jb	0x1408e9a41 <.text+0x8e8a41>
1408e8fb4: f3 41 0f 5f c0              	maxss	%xmm8, %xmm0
1408e8fb9: 0f 28 d1                    	movaps	%xmm1, %xmm2
1408e8fbc: f3 0f 5d d0                 	minss	%xmm0, %xmm2
1408e8fc0: f3 0f 11 55 00              	movss	%xmm2, (%rbp)
1408e8fc5: 0f 2e 15 c4 76 07 02        	ucomiss	0x20776c4(%rip), %xmm2  # 0x142960690
1408e8fcc: 0f 82 80 0a 00 00           	jb	0x1408e9a52 <.text+0x8e8a52>
1408e8fd2: f3 0f 10 05 2a c9 03 02     	movss	0x203c92a(%rip), %xmm0  # 0x142925904
1408e8fda: 0f 2e c2                    	ucomiss	%xmm2, %xmm0
1408e8fdd: 0f 82 6f 0a 00 00           	jb	0x1408e9a52 <.text+0x8e8a52>
1408e8fe3: 0f 2e d2                    	ucomiss	%xmm2, %xmm2
1408e8fe6: 0f 8a ca 0a 00 00           	jp	0x1408e9ab6 <.text+0x8e8ab6>
1408e8fec: f3 0f 10 85 d0 01 00 00     	movss	0x1d0(%rbp), %xmm0
1408e8ff4: f3 41 0f 5f c1              	maxss	%xmm9, %xmm0
1408e8ff9: f3 44 0f 10 95 d4 01 00 00  	movss	0x1d4(%rbp), %xmm10
1408e9002: f3 44 0f 5d d0              	minss	%xmm0, %xmm10
1408e9007: f3 44 0f 11 55 00           	movss	%xmm10, (%rbp)
1408e900d: 44 0f 2e 15 7b 76 07 02     	ucomiss	0x207767b(%rip), %xmm10 # 0x142960690
1408e9015: 0f 82 b8 0a 00 00           	jb	0x1408e9ad3 <.text+0x8e8ad3>
1408e901b: f3 0f 10 05 e1 c8 03 02     	movss	0x203c8e1(%rip), %xmm0  # 0x142925904
1408e9023: 41 0f 2e c2                 	ucomiss	%xmm10, %xmm0
1408e9027: 0f 82 a6 0a 00 00           	jb	0x1408e9ad3 <.text+0x8e8ad3>
1408e902d: 45 0f 2e d2                 	ucomiss	%xmm10, %xmm10
1408e9031: 0f 8b 19 ff ff ff           	jnp	0x1408e8f50 <.text+0x8e7f50>
1408e9037: e9 fb 0a 00 00              	jmp	0x1408e9b37 <.text+0x8e8b37>
1408e903c: 0f 1f 40 00                 	nopl	(%rax)
1408e9040: 41 8b 47 28                 	movl	0x28(%r15), %eax
1408e9044: 89 85 88 01 00 00           	movl	%eax, 0x188(%rbp)
1408e904a: 49 8b 47 20                 	movq	0x20(%r15), %rax
1408e904e: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
1408e9055: 41 8b 47 34                 	movl	0x34(%r15), %eax
1408e9059: 89 45 68                    	movl	%eax, 0x68(%rbp)
1408e905c: 49 8b 47 2c                 	movq	0x2c(%r15), %rax
1408e9060: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1408e9064: 41 8b 47 40                 	movl	0x40(%r15), %eax
1408e9068: 89 85 b8 00 00 00           	movl	%eax, 0xb8(%rbp)
1408e906e: 49 8b 47 38                 	movq	0x38(%r15), %rax
1408e9072: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408e9079: 48 89 e9                    	movq	%rbp, %rcx
1408e907c: 48 8d 95 80 01 00 00        	leaq	0x180(%rbp), %rdx
1408e9083: 4c 8d 45 60                 	leaq	0x60(%rbp), %r8
1408e9087: 4c 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %r9
1408e908e: e8 dd 1c 95 00              	callq	0x14123ad70 <.text+0x1239d70>
1408e9093: 49 8d 4f 2c                 	leaq	0x2c(%r15), %rcx
1408e9097: 49 8d 47 38                 	leaq	0x38(%r15), %rax
1408e909b: 41 8b 57 60                 	movl	0x60(%r15), %edx
1408e909f: 89 95 b8 00 00 00           	movl	%edx, 0xb8(%rbp)
1408e90a5: 49 8b 57 58                 	movq	0x58(%r15), %rdx
1408e90a9: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
1408e90b0: 41 8b 57 6c                 	movl	0x6c(%r15), %edx
1408e90b4: 4c 8d 85 bc 00 00 00        	leaq	0xbc(%rbp), %r8
1408e90bb: 41 89 50 08                 	movl	%edx, 0x8(%r8)
1408e90bf: 49 8b 57 64                 	movq	0x64(%r15), %rdx
1408e90c3: 49 89 10                    	movq	%rdx, (%r8)
1408e90c6: 41 8b 57 60                 	movl	0x60(%r15), %edx
1408e90ca: 41 89 50 14                 	movl	%edx, 0x14(%r8)
1408e90ce: 49 8b 57 58                 	movq	0x58(%r15), %rdx
1408e90d2: 49 89 50 0c                 	movq	%rdx, 0xc(%r8)
1408e90d6: 41 0f 10 3f                 	movups	(%r15), %xmm7
1408e90da: f2 41 0f 10 77 10           	movsd	0x10(%r15), %xmm6
1408e90e0: 8b 95 d0 00 00 00           	movl	0xd0(%rbp), %edx
1408e90e6: 4c 8d 55 24                 	leaq	0x24(%rbp), %r10
1408e90ea: 41 89 52 20                 	movl	%edx, 0x20(%r10)
1408e90ee: 48 8b 95 b0 00 00 00        	movq	0xb0(%rbp), %rdx
1408e90f5: 44 8b 85 b8 00 00 00        	movl	0xb8(%rbp), %r8d
1408e90fc: 44 8b 8d bc 00 00 00        	movl	0xbc(%rbp), %r9d
1408e9103: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1408e910a: 41 0f 11 42 10              	movups	%xmm0, 0x10(%r10)
1408e910f: 49 89 12                    	movq	%rdx, (%r10)
1408e9112: 45 89 42 08                 	movl	%r8d, 0x8(%r10)
1408e9116: 45 89 4a 0c                 	movl	%r9d, 0xc(%r10)
1408e911a: 8b 51 08                    	movl	0x8(%rcx), %edx
1408e911d: 89 95 e0 01 00 00           	movl	%edx, 0x1e0(%rbp)
1408e9123: 48 8b 09                    	movq	(%rcx), %rcx
1408e9126: 48 89 8d d8 01 00 00        	movq	%rcx, 0x1d8(%rbp)
1408e912d: 41 8b 4f 4c                 	movl	0x4c(%r15), %ecx
1408e9131: 89 8d 88 01 00 00           	movl	%ecx, 0x188(%rbp)
1408e9137: 49 8b 4f 44                 	movq	0x44(%r15), %rcx
1408e913b: 48 89 8d 80 01 00 00        	movq	%rcx, 0x180(%rbp)
1408e9142: 8b 48 08                    	movl	0x8(%rax), %ecx
1408e9145: 89 8d b8 00 00 00           	movl	%ecx, 0xb8(%rbp)
1408e914b: 48 8b 00                    	movq	(%rax), %rax
1408e914e: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408e9155: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
1408e9159: 48 8d 95 d8 01 00 00        	leaq	0x1d8(%rbp), %rdx
1408e9160: 4c 8d 85 80 01 00 00        	leaq	0x180(%rbp), %r8
1408e9167: 4c 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %r9
1408e916e: e8 fd 1b 95 00              	callq	0x14123ad70 <.text+0x1239d70>
1408e9173: 49 8d 47 58                 	leaq	0x58(%r15), %rax
1408e9177: 49 8d 4f 64                 	leaq	0x64(%r15), %rcx
1408e917b: 8b 51 08                    	movl	0x8(%rcx), %edx
1408e917e: 89 95 b8 00 00 00           	movl	%edx, 0xb8(%rbp)
1408e9184: 48 8b 11                    	movq	(%rcx), %rdx
1408e9187: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
1408e918e: 8b 51 08                    	movl	0x8(%rcx), %edx
1408e9191: 4c 8d 85 bc 00 00 00        	leaq	0xbc(%rbp), %r8
1408e9198: 41 89 50 08                 	movl	%edx, 0x8(%r8)
1408e919c: 48 8b 09                    	movq	(%rcx), %rcx
1408e919f: 49 89 08                    	movq	%rcx, (%r8)
1408e91a2: 8b 48 08                    	movl	0x8(%rax), %ecx
1408e91a5: 41 89 48 14                 	movl	%ecx, 0x14(%r8)
1408e91a9: 48 8b 00                    	movq	(%rax), %rax
1408e91ac: 49 89 40 0c                 	movq	%rax, 0xc(%r8)
1408e91b0: 8b 85 d0 00 00 00           	movl	0xd0(%rbp), %eax
1408e91b6: 4c 8d 8d 84 00 00 00        	leaq	0x84(%rbp), %r9
1408e91bd: 41 89 41 20                 	movl	%eax, 0x20(%r9)
1408e91c1: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1408e91c8: 8b 8d b8 00 00 00           	movl	0xb8(%rbp), %ecx
1408e91ce: 8b 95 bc 00 00 00           	movl	0xbc(%rbp), %edx
1408e91d4: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1408e91db: 41 0f 11 41 10              	movups	%xmm0, 0x10(%r9)
1408e91e0: 49 89 01                    	movq	%rax, (%r9)
1408e91e3: 41 89 49 08                 	movl	%ecx, 0x8(%r9)
1408e91e7: 41 89 51 0c                 	movl	%edx, 0xc(%r9)
1408e91eb: 48 8b 45 40                 	movq	0x40(%rbp), %rax
1408e91ef: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1408e91f6: 0f 10 45 00                 	movups	(%rbp), %xmm0
1408e91fa: 0f 10 4d 10                 	movups	0x10(%rbp), %xmm1
1408e91fe: 0f 10 55 20                 	movups	0x20(%rbp), %xmm2
1408e9202: 0f 10 5d 30                 	movups	0x30(%rbp), %xmm3
1408e9206: 0f 29 9d e0 00 00 00        	movaps	%xmm3, 0xe0(%rbp)
1408e920d: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1408e9214: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1408e921b: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
1408e9222: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
1408e9229: 49 89 80 94 00 00 00        	movq	%rax, 0x94(%r8)
1408e9230: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1408e9234: 0f 10 4d 70                 	movups	0x70(%rbp), %xmm1
1408e9238: 0f 10 95 80 00 00 00        	movups	0x80(%rbp), %xmm2
1408e923f: 0f 10 9d 90 00 00 00        	movups	0x90(%rbp), %xmm3
1408e9246: 41 0f 11 98 84 00 00 00     	movups	%xmm3, 0x84(%r8)
1408e924e: 41 0f 11 50 74              	movups	%xmm2, 0x74(%r8)
1408e9253: 41 0f 11 48 64              	movups	%xmm1, 0x64(%r8)
1408e9258: 41 0f 11 40 54              	movups	%xmm0, 0x54(%r8)
1408e925d: 0f 11 bd f8 00 00 00        	movups	%xmm7, 0xf8(%rbp)
1408e9264: 0f 13 b5 08 01 00 00        	movlps	%xmm6, 0x108(%rbp)
1408e926b: 0f 17 bd 58 01 00 00        	movhps	%xmm7, 0x158(%rbp)
1408e9272: f2 41 0f 10 47 18           	movsd	0x18(%r15), %xmm0
1408e9278: 0f 16 c6                    	movlhps	%xmm6, %xmm0            # xmm0 = xmm0[0],xmm6[0]
1408e927b: 0f 29 85 60 01 00 00        	movaps	%xmm0, 0x160(%rbp)
1408e9282: 45 31 e4                    	xorl	%r12d, %r12d
1408e9285: eb 35                       	jmp	0x1408e92bc <.text+0x8e82bc>
1408e9287: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
1408e9290: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1408e9297: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
1408e929b: 49 8b 54 24 58              	movq	0x58(%r12), %rdx
1408e92a0: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
1408e92a5: 41 0f 10 44 24 48           	movups	0x48(%r12), %xmm0
1408e92ab: 0f 11 04 c8                 	movups	%xmm0, (%rax,%rcx,8)
1408e92af: 48 ff c6                    	incq	%rsi
1408e92b2: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
1408e92b9: 4d 89 ec                    	movq	%r13, %r12
1408e92bc: 49 83 fc 02                 	cmpq	$0x2, %r12
1408e92c0: 0f 84 9a fc ff ff           	je	0x1408e8f60 <.text+0x8e7f60>
1408e92c6: 4d 8d 6c 24 01              	leaq	0x1(%r12), %r13
1408e92cb: 4c 89 ad 70 01 00 00        	movq	%r13, 0x170(%rbp)
1408e92d2: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1408e92d9: 48 3b b5 e8 01 00 00        	cmpq	0x1e8(%rbp), %rsi
1408e92e0: 75 13                       	jne	0x1408e92f5 <.text+0x8e82f5>
1408e92e2: 48 8d 8d e8 01 00 00        	leaq	0x1e8(%rbp), %rcx
1408e92e9: 48 8d 15 20 d3 1b 02        	leaq	0x21bd320(%rip), %rdx   # 0x142aa6610
1408e92f0: e8 3b 69 27 00              	callq	0x140b5fc30 <.text+0xb5ec30>
1408e92f5: 4b 8d 04 64                 	leaq	(%r12,%r12,2), %rax
1408e92f9: 48 c1 e0 05                 	shlq	$0x5, %rax
1408e92fd: 4c 8d 24 28                 	leaq	(%rax,%rbp), %r12
1408e9301: 49 81 c4 b0 00 00 00        	addq	$0xb0, %r12
1408e9308: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1408e930f: 48 8d 0c f6                 	leaq	(%rsi,%rsi,8), %rcx
1408e9313: 41 8b 54 24 20              	movl	0x20(%r12), %edx
1408e9318: 89 54 88 20                 	movl	%edx, 0x20(%rax,%rcx,4)
1408e931c: 41 0f 10 04 24              	movups	(%r12), %xmm0
1408e9321: 41 0f 10 4c 24 10           	movups	0x10(%r12), %xmm1
1408e9327: 0f 11 4c 88 10              	movups	%xmm1, 0x10(%rax,%rcx,4)
1408e932c: 0f 11 04 88                 	movups	%xmm0, (%rax,%rcx,4)
1408e9330: 48 ff c6                    	incq	%rsi
1408e9333: 48 89 b5 f8 01 00 00        	movq	%rsi, 0x1f8(%rbp)
1408e933a: 48 8b b5 c8 01 00 00        	movq	0x1c8(%rbp), %rsi
1408e9341: 48 3b b5 b8 01 00 00        	cmpq	0x1b8(%rbp), %rsi
1408e9348: 75 0f                       	jne	0x1408e9359 <.text+0x8e8359>
1408e934a: 48 89 d9                    	movq	%rbx, %rcx
1408e934d: 48 8d 15 d4 d2 1b 02        	leaq	0x21bd2d4(%rip), %rdx   # 0x142aa6628
1408e9354: e8 d7 68 27 00              	callq	0x140b5fc30 <.text+0xb5ec30>
1408e9359: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1408e9360: 48 8d 0c f6                 	leaq	(%rsi,%rsi,8), %rcx
1408e9364: 41 8b 54 24 44              	movl	0x44(%r12), %edx
1408e9369: 89 54 88 20                 	movl	%edx, 0x20(%rax,%rcx,4)
1408e936d: 41 0f 10 44 24 24           	movups	0x24(%r12), %xmm0
1408e9373: 41 0f 10 4c 24 34           	movups	0x34(%r12), %xmm1
1408e9379: 0f 11 4c 88 10              	movups	%xmm1, 0x10(%rax,%rcx,4)
1408e937e: 0f 11 04 88                 	movups	%xmm0, (%rax,%rcx,4)
1408e9382: 48 ff c6                    	incq	%rsi
1408e9385: 48 89 b5 c8 01 00 00        	movq	%rsi, 0x1c8(%rbp)
1408e938c: 48 8b b5 b0 01 00 00        	movq	0x1b0(%rbp), %rsi
1408e9393: 48 3b b5 a0 01 00 00        	cmpq	0x1a0(%rbp), %rsi
1408e939a: 0f 85 f0 fe ff ff           	jne	0x1408e9290 <.text+0x8e8290>
1408e93a0: 48 89 f9                    	movq	%rdi, %rcx
1408e93a3: 4c 89 f2                    	movq	%r14, %rdx
1408e93a6: e8 d5 3f d9 ff              	callq	0x14067d380 <.text+0x67c380>
1408e93ab: e9 e0 fe ff ff              	jmp	0x1408e9290 <.text+0x8e8290>
1408e93b0: f2 41 0f 10 5f 20           	movsd	0x20(%r15), %xmm3
1408e93b6: f2 41 0f 10 4f 2c           	movsd	0x2c(%r15), %xmm1
1408e93bc: f2 41 0f 10 47 58           	movsd	0x58(%r15), %xmm0
1408e93c2: 0f 59 45 f0                 	mulps	-0x10(%rbp), %xmm0
1408e93c6: f2 45 0f 10 67 70           	movsd	0x70(%r15), %xmm12
1408e93cc: 41 0f 28 ec                 	movaps	%xmm12, %xmm5
1408e93d0: 45 0f 57 f6                 	xorps	%xmm14, %xmm14
1408e93d4: 41 0f 59 ee                 	mulps	%xmm14, %xmm5
1408e93d8: f3 41 0f 10 67 34           	movss	0x34(%r15), %xmm4
1408e93de: f3 45 0f 10 7f 28           	movss	0x28(%r15), %xmm15
1408e93e4: 44 0f 14 fc                 	unpcklps	%xmm4, %xmm15           # xmm15 = xmm15[0],xmm4[0],xmm15[1],xmm4[1]
1408e93e8: 44 0f 5c fd                 	subps	%xmm5, %xmm15
1408e93ec: 44 0f 28 5d d0              	movaps	-0x30(%rbp), %xmm11
1408e93f1: 41 0f 28 e3                 	movaps	%xmm11, %xmm4
1408e93f5: 41 0f 5f e4                 	maxps	%xmm12, %xmm4
1408e93f9: 44 0f 2e 25 8f 72 07 02     	ucomiss	0x207728f(%rip), %xmm12 # 0x142960690
1408e9401: 41 0f 28 f4                 	movaps	%xmm12, %xmm6
1408e9405: 0f c6 f5 11                 	shufps	$0x11, %xmm5, %xmm6     # xmm6 = xmm6[1,0],xmm5[1,0]
1408e9409: 0f c6 f5 e2                 	shufps	$0xe2, %xmm5, %xmm6     # xmm6 = xmm6[2,0],xmm5[2,3]
1408e940d: 0f 28 fd                    	movaps	%xmm5, %xmm7
1408e9410: 41 0f 14 fc                 	unpcklps	%xmm12, %xmm7           # xmm7 = xmm7[0],xmm12[0],xmm7[1],xmm12[1]
1408e9414: f3 41 0f 12 ec              	movsldup	%xmm12, %xmm5           # xmm5 = xmm12[0,0,2,2]
1408e9419: 45 0f c2 e4 03              	cmpunordps	%xmm12, %xmm12
1408e941e: 45 0f 54 dc                 	andps	%xmm12, %xmm11
1408e9422: 44 0f 55 e4                 	andnps	%xmm4, %xmm12
1408e9426: 0f 97 c0                    	seta	%al
1408e9429: 45 0f 56 e3                 	orps	%xmm11, %xmm12
1408e942d: 22 85 07 02 00 00           	andb	0x207(%rbp), %al
1408e9433: f3 45 0f 10 5f 6c           	movss	0x6c(%r15), %xmm11
1408e9439: f3 41 0f 10 67 60           	movss	0x60(%r15), %xmm4
1408e943f: 41 0f 14 e3                 	unpcklps	%xmm11, %xmm4           # xmm4 = xmm4[0],xmm11[0],xmm4[1],xmm11[1]
1408e9443: 45 0f 28 dc                 	movaps	%xmm12, %xmm11
1408e9447: 45 0f 59 de                 	mulps	%xmm14, %xmm11
1408e944b: 45 0f 58 fb                 	addps	%xmm11, %xmm15
1408e944f: 0f 5c df                    	subps	%xmm7, %xmm3
1408e9452: 0f 5c ce                    	subps	%xmm6, %xmm1
1408e9455: 45 0f 28 eb                 	movaps	%xmm11, %xmm13
1408e9459: 45 0f 14 ec                 	unpcklps	%xmm12, %xmm13          # xmm13 = xmm13[0],xmm12[0],xmm13[1],xmm12[1]
1408e945d: 44 0f 58 eb                 	addps	%xmm3, %xmm13
1408e9461: 44 0f 58 e8                 	addps	%xmm0, %xmm13
1408e9465: 45 0f c6 e3 11              	shufps	$0x11, %xmm11, %xmm12   # xmm12 = xmm12[1,0],xmm11[1,0]
1408e946a: 45 0f c6 e3 e2              	shufps	$0xe2, %xmm11, %xmm12   # xmm12 = xmm12[2,0],xmm11[2,3]
1408e946f: 44 0f 58 e1                 	addps	%xmm1, %xmm12
1408e9473: f2 41 0f 10 5f 38           	movsd	0x38(%r15), %xmm3
1408e9479: f2 41 0f 10 4f 44           	movsd	0x44(%r15), %xmm1
1408e947f: 0f 28 f1                    	movaps	%xmm1, %xmm6
1408e9482: 0f c6 f3 11                 	shufps	$0x11, %xmm3, %xmm6     # xmm6 = xmm6[1,0],xmm3[1,0]
1408e9486: 0f c6 f3 e2                 	shufps	$0xe2, %xmm3, %xmm6     # xmm6 = xmm6[2,0],xmm3[2,3]
1408e948a: 0f 5c ee                    	subps	%xmm6, %xmm5
1408e948d: 0f b6 c0                    	movzbl	%al, %eax
1408e9490: 66 0f 6e f0                 	movd	%eax, %xmm6
1408e9494: 66 0f 70 fe 50              	pshufd	$0x50, %xmm6, %xmm7     # xmm7 = xmm6[0,0,1,1]
1408e9499: 66 0f 72 f7 1f              	pslld	$0x1f, %xmm7
1408e949e: 66 0f 72 e7 1f              	psrad	$0x1f, %xmm7
1408e94a3: 0f 54 ef                    	andps	%xmm7, %xmm5
1408e94a6: 0f 55 7d e0                 	andnps	-0x20(%rbp), %xmm7
1408e94aa: 0f 56 fd                    	orps	%xmm5, %xmm7
1408e94ad: 0f 28 ef                    	movaps	%xmm7, %xmm5
1408e94b0: 41 0f 59 ee                 	mulps	%xmm14, %xmm5
1408e94b4: 0f 28 f5                    	movaps	%xmm5, %xmm6
1408e94b7: 0f 14 f7                    	unpcklps	%xmm7, %xmm6            # xmm6 = xmm6[0],xmm7[0],xmm6[1],xmm7[1]
1408e94ba: 0f 58 f3                    	addps	%xmm3, %xmm6
1408e94bd: 0f 58 f0                    	addps	%xmm0, %xmm6
1408e94c0: f3 41 0f 10 47 4c           	movss	0x4c(%r15), %xmm0
1408e94c6: f3 45 0f 10 77 40           	movss	0x40(%r15), %xmm14
1408e94cc: 44 0f 14 f0                 	unpcklps	%xmm0, %xmm14           # xmm14 = xmm14[0],xmm0[0],xmm14[1],xmm0[1]
1408e94d0: 44 0f 58 f5                 	addps	%xmm5, %xmm14
1408e94d4: 0f c6 fd 11                 	shufps	$0x11, %xmm5, %xmm7     # xmm7 = xmm7[1,0],xmm5[1,0]
1408e94d8: 0f c6 fd e2                 	shufps	$0xe2, %xmm5, %xmm7     # xmm7 = xmm7[2,0],xmm5[2,3]
1408e94dc: f2 41 0f 10 47 64           	movsd	0x64(%r15), %xmm0
1408e94e2: 0f 28 5d f0                 	movaps	-0x10(%rbp), %xmm3
1408e94e6: 0f 59 c3                    	mulps	%xmm3, %xmm0
1408e94e9: 0f 59 e3                    	mulps	%xmm3, %xmm4
1408e94ec: 44 0f 58 e0                 	addps	%xmm0, %xmm12
1408e94f0: 44 0f 58 fc                 	addps	%xmm4, %xmm15
1408e94f4: 0f 58 f9                    	addps	%xmm1, %xmm7
1408e94f7: 0f 58 f8                    	addps	%xmm0, %xmm7
1408e94fa: 44 0f 58 f4                 	addps	%xmm4, %xmm14
1408e94fe: 49 8b 47 68                 	movq	0x68(%r15), %rax
1408e9502: 48 89 85 90 01 00 00        	movq	%rax, 0x190(%rbp)
1408e9509: 41 0f 10 47 58              	movups	0x58(%r15), %xmm0
1408e950e: 0f 29 85 80 01 00 00        	movaps	%xmm0, 0x180(%rbp)
1408e9515: 41 0f 10 07                 	movups	(%r15), %xmm0
1408e9519: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
1408e951d: f2 41 0f 10 47 10           	movsd	0x10(%r15), %xmm0
1408e9523: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
1408e9527: f2 41 0f 10 47 18           	movsd	0x18(%r15), %xmm0
1408e952d: 0f 29 45 c0                 	movaps	%xmm0, -0x40(%rbp)
1408e9531: 41 0f 2e d0                 	ucomiss	%xmm8, %xmm2
1408e9535: 75 19                       	jne	0x1408e9550 <.text+0x8e8550>
1408e9537: 7a 17                       	jp	0x1408e9550 <.text+0x8e8550>
1408e9539: 45 0f 2e d1                 	ucomiss	%xmm9, %xmm10
1408e953d: 75 11                       	jne	0x1408e9550 <.text+0x8e8550>
1408e953f: 7a 0f                       	jp	0x1408e9550 <.text+0x8e8550>
1408e9541: f3 45 0f 16 c7              	movshdup	%xmm15, %xmm8           # xmm8 = xmm15[1,1,3,3]
1408e9546: f3 45 0f 16 ce              	movshdup	%xmm14, %xmm9           # xmm9 = xmm14[1,1,3,3]
1408e954b: e9 cf 00 00 00              	jmp	0x1408e961f <.text+0x8e861f>
1408e9550: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1408e9554: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1408e9558: e8 43 6f 3a 00              	callq	0x140c904a0 <.text+0xc8f4a0>
1408e955d: 44 0f 28 d8                 	movaps	%xmm0, %xmm11
1408e9561: 41 0f 28 c0                 	movaps	%xmm8, %xmm0
1408e9565: 41 0f 28 c9                 	movaps	%xmm9, %xmm1
1408e9569: 41 0f 28 d2                 	movaps	%xmm10, %xmm2
1408e956d: e8 2e 6f 3a 00              	callq	0x140c904a0 <.text+0xc8f4a0>
1408e9572: f3 44 0f 10 05 89 c3 03 02  	movss	0x203c389(%rip), %xmm8  # 0x142925904
1408e957b: 41 0f 28 d8                 	movaps	%xmm8, %xmm3
1408e957f: f3 41 0f 5c db              	subss	%xmm11, %xmm3
1408e9584: f3 44 0f 12 cb              	movsldup	%xmm3, %xmm9            # xmm9 = xmm3[0,0,2,2]
1408e9589: 41 0f 28 e5                 	movaps	%xmm13, %xmm4
1408e958d: 41 0f 59 e1                 	mulps	%xmm9, %xmm4
1408e9591: f3 41 0f 12 d3              	movsldup	%xmm11, %xmm2           # xmm2 = xmm11[0,0,2,2]
1408e9596: 41 0f 28 cc                 	movaps	%xmm12, %xmm1
1408e959a: 0f 59 ca                    	mulps	%xmm2, %xmm1
1408e959d: 0f 58 cc                    	addps	%xmm4, %xmm1
1408e95a0: 41 0f 14 db                 	unpcklps	%xmm11, %xmm3           # xmm3 = xmm3[0],xmm11[0],xmm3[1],xmm11[1]
1408e95a4: 41 0f 28 ef                 	movaps	%xmm15, %xmm5
1408e95a8: 0f 59 eb                    	mulps	%xmm3, %xmm5
1408e95ab: f3 0f 16 e5                 	movshdup	%xmm5, %xmm4            # xmm4 = xmm5[1,1,3,3]
1408e95af: f3 0f 58 e5                 	addss	%xmm5, %xmm4
1408e95b3: 41 0f 28 e8                 	movaps	%xmm8, %xmm5
1408e95b7: f3 0f 5c e8                 	subss	%xmm0, %xmm5
1408e95bb: f3 44 0f 12 d5              	movsldup	%xmm5, %xmm10           # xmm10 = xmm5[0,0,2,2]
1408e95c0: 45 0f 59 ea                 	mulps	%xmm10, %xmm13
1408e95c4: f3 44 0f 12 d8              	movsldup	%xmm0, %xmm11           # xmm11 = xmm0[0,0,2,2]
1408e95c9: 45 0f 59 e3                 	mulps	%xmm11, %xmm12
1408e95cd: 45 0f 58 e5                 	addps	%xmm13, %xmm12
1408e95d1: 0f 14 e8                    	unpcklps	%xmm0, %xmm5            # xmm5 = xmm5[0],xmm0[0],xmm5[1],xmm0[1]
1408e95d4: 44 0f 59 fd                 	mulps	%xmm5, %xmm15
1408e95d8: f3 45 0f 16 c7              	movshdup	%xmm15, %xmm8           # xmm8 = xmm15[1,1,3,3]
1408e95dd: f3 45 0f 58 c7              	addss	%xmm15, %xmm8
1408e95e2: 44 0f 59 ce                 	mulps	%xmm6, %xmm9
1408e95e6: 0f 59 d7                    	mulps	%xmm7, %xmm2
1408e95e9: 41 0f 58 d1                 	addps	%xmm9, %xmm2
1408e95ed: 41 0f 59 de                 	mulps	%xmm14, %xmm3
1408e95f1: 41 0f 59 ee                 	mulps	%xmm14, %xmm5
1408e95f5: f3 44 0f 16 f3              	movshdup	%xmm3, %xmm14           # xmm14 = xmm3[1,1,3,3]
1408e95fa: f3 44 0f 58 f3              	addss	%xmm3, %xmm14
1408e95ff: 41 0f 59 f2                 	mulps	%xmm10, %xmm6
1408e9603: 41 0f 59 fb                 	mulps	%xmm11, %xmm7
1408e9607: 0f 58 fe                    	addps	%xmm6, %xmm7
1408e960a: f3 44 0f 16 cd              	movshdup	%xmm5, %xmm9            # xmm9 = xmm5[1,1,3,3]
1408e960f: f3 44 0f 58 cd              	addss	%xmm5, %xmm9
1408e9614: 44 0f 28 fc                 	movaps	%xmm4, %xmm15
1408e9618: 0f 28 f2                    	movaps	%xmm2, %xmm6
1408e961b: 44 0f 28 e9                 	movaps	%xmm1, %xmm13
1408e961f: 44 0f 13 ad d8 01 00 00     	movlps	%xmm13, 0x1d8(%rbp)
1408e9627: f3 44 0f 11 bd e0 01 00 00  	movss	%xmm15, 0x1e0(%rbp)
1408e9630: 44 0f 13 65 60              	movlps	%xmm12, 0x60(%rbp)
1408e9635: f3 44 0f 11 45 68           	movss	%xmm8, 0x68(%rbp)
1408e963b: 0f 13 b5 b0 00 00 00        	movlps	%xmm6, 0xb0(%rbp)
1408e9642: f3 44 0f 11 b5 b8 00 00 00  	movss	%xmm14, 0xb8(%rbp)
1408e964b: 48 89 e9                    	movq	%rbp, %rcx
1408e964e: 48 8d 95 d8 01 00 00        	leaq	0x1d8(%rbp), %rdx
1408e9655: 4c 8d 45 60                 	leaq	0x60(%rbp), %r8
1408e9659: 4c 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %r9
1408e9660: e8 0b 17 95 00              	callq	0x14123ad70 <.text+0x1239d70>
1408e9665: 8b 85 88 01 00 00           	movl	0x188(%rbp), %eax
1408e966b: 89 85 b8 00 00 00           	movl	%eax, 0xb8(%rbp)
1408e9671: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1408e9678: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408e967f: 48 8d 95 8c 01 00 00        	leaq	0x18c(%rbp), %rdx
1408e9686: 8b 42 08                    	movl	0x8(%rdx), %eax
1408e9689: 48 8d 8d bc 00 00 00        	leaq	0xbc(%rbp), %rcx
1408e9690: 89 41 08                    	movl	%eax, 0x8(%rcx)
1408e9693: 48 8b 02                    	movq	(%rdx), %rax
1408e9696: 48 89 01                    	movq	%rax, (%rcx)
1408e9699: 8b 85 88 01 00 00           	movl	0x188(%rbp), %eax
1408e969f: 89 41 14                    	movl	%eax, 0x14(%rcx)
1408e96a2: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1408e96a9: 48 89 41 0c                 	movq	%rax, 0xc(%rcx)
1408e96ad: 8b 85 d0 00 00 00           	movl	0xd0(%rbp), %eax
1408e96b3: 4c 8d 45 24                 	leaq	0x24(%rbp), %r8
1408e96b7: 41 89 40 20                 	movl	%eax, 0x20(%r8)
1408e96bb: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1408e96c2: 8b 8d b8 00 00 00           	movl	0xb8(%rbp), %ecx
1408e96c8: 8b 95 bc 00 00 00           	movl	0xbc(%rbp), %edx
1408e96ce: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1408e96d5: 41 0f 11 40 10              	movups	%xmm0, 0x10(%r8)
1408e96da: 49 89 00                    	movq	%rax, (%r8)
1408e96dd: 41 89 48 08                 	movl	%ecx, 0x8(%r8)
1408e96e1: 41 89 50 0c                 	movl	%edx, 0xc(%r8)
1408e96e5: 44 0f 13 65 48              	movlps	%xmm12, 0x48(%rbp)
1408e96ea: f3 44 0f 11 45 50           	movss	%xmm8, 0x50(%rbp)
1408e96f0: 0f 13 bd d8 01 00 00        	movlps	%xmm7, 0x1d8(%rbp)
1408e96f7: f3 44 0f 11 8d e0 01 00 00  	movss	%xmm9, 0x1e0(%rbp)
1408e9700: 0f 13 b5 b0 00 00 00        	movlps	%xmm6, 0xb0(%rbp)
1408e9707: f3 44 0f 11 b5 b8 00 00 00  	movss	%xmm14, 0xb8(%rbp)
1408e9710: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
1408e9714: 48 8d 55 48                 	leaq	0x48(%rbp), %rdx
1408e9718: 4c 8d 85 d8 01 00 00        	leaq	0x1d8(%rbp), %r8
1408e971f: 4c 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %r9
1408e9726: e8 45 16 95 00              	callq	0x14123ad70 <.text+0x1239d70>
1408e972b: 48 8d 8d 8c 01 00 00        	leaq	0x18c(%rbp), %rcx
1408e9732: 8b 41 08                    	movl	0x8(%rcx), %eax
1408e9735: 89 85 b8 00 00 00           	movl	%eax, 0xb8(%rbp)
1408e973b: 48 8b 01                    	movq	(%rcx), %rax
1408e973e: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408e9745: 8b 41 08                    	movl	0x8(%rcx), %eax
1408e9748: 4c 8d 85 bc 00 00 00        	leaq	0xbc(%rbp), %r8
1408e974f: 41 89 40 08                 	movl	%eax, 0x8(%r8)
1408e9753: 48 8b 01                    	movq	(%rcx), %rax
1408e9756: 49 89 00                    	movq	%rax, (%r8)
1408e9759: 8b 85 88 01 00 00           	movl	0x188(%rbp), %eax
1408e975f: 41 89 40 14                 	movl	%eax, 0x14(%r8)
1408e9763: 48 8b 85 80 01 00 00        	movq	0x180(%rbp), %rax
1408e976a: 49 89 40 0c                 	movq	%rax, 0xc(%r8)
1408e976e: 8b 85 d0 00 00 00           	movl	0xd0(%rbp), %eax
1408e9774: 4c 8d 8d 84 00 00 00        	leaq	0x84(%rbp), %r9
1408e977b: 41 89 41 20                 	movl	%eax, 0x20(%r9)
1408e977f: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1408e9786: 8b 8d b8 00 00 00           	movl	0xb8(%rbp), %ecx
1408e978c: 8b 95 bc 00 00 00           	movl	0xbc(%rbp), %edx
1408e9792: 0f 10 85 c0 00 00 00        	movups	0xc0(%rbp), %xmm0
1408e9799: 41 0f 11 41 10              	movups	%xmm0, 0x10(%r9)
1408e979e: 49 89 01                    	movq	%rax, (%r9)
1408e97a1: 41 89 49 08                 	movl	%ecx, 0x8(%r9)
1408e97a5: 41 89 51 0c                 	movl	%edx, 0xc(%r9)
1408e97a9: 48 8b 45 40                 	movq	0x40(%rbp), %rax
1408e97ad: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1408e97b4: 0f 10 45 00                 	movups	(%rbp), %xmm0
1408e97b8: 0f 10 4d 10                 	movups	0x10(%rbp), %xmm1
1408e97bc: 0f 10 55 20                 	movups	0x20(%rbp), %xmm2
1408e97c0: 0f 10 5d 30                 	movups	0x30(%rbp), %xmm3
1408e97c4: 0f 29 9d e0 00 00 00        	movaps	%xmm3, 0xe0(%rbp)
1408e97cb: 0f 29 95 d0 00 00 00        	movaps	%xmm2, 0xd0(%rbp)
1408e97d2: 0f 29 8d c0 00 00 00        	movaps	%xmm1, 0xc0(%rbp)
1408e97d9: 0f 29 85 b0 00 00 00        	movaps	%xmm0, 0xb0(%rbp)
1408e97e0: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
1408e97e7: 49 89 80 94 00 00 00        	movq	%rax, 0x94(%r8)
1408e97ee: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
1408e97f2: 0f 10 4d 70                 	movups	0x70(%rbp), %xmm1
1408e97f6: 0f 10 95 80 00 00 00        	movups	0x80(%rbp), %xmm2
1408e97fd: 0f 10 9d 90 00 00 00        	movups	0x90(%rbp), %xmm3
1408e9804: 41 0f 11 98 84 00 00 00     	movups	%xmm3, 0x84(%r8)
1408e980c: 41 0f 11 50 74              	movups	%xmm2, 0x74(%r8)
1408e9811: 41 0f 11 48 64              	movups	%xmm1, 0x64(%r8)
1408e9816: 41 0f 11 40 54              	movups	%xmm0, 0x54(%r8)
1408e981b: 66 0f 28 4d b0              	movapd	-0x50(%rbp), %xmm1
1408e9820: 66 0f 11 8d f8 00 00 00     	movupd	%xmm1, 0xf8(%rbp)
1408e9828: 0f 28 45 a0                 	movaps	-0x60(%rbp), %xmm0
1408e982c: 0f 13 85 08 01 00 00        	movlps	%xmm0, 0x108(%rbp)
1408e9833: 66 0f c6 4d c0 01           	shufpd	$0x1, -0x40(%rbp), %xmm1 # xmm1 = xmm1[1],mem[0]
1408e9839: 66 0f 11 8d 58 01 00 00     	movupd	%xmm1, 0x158(%rbp)
1408e9841: 0f 13 85 68 01 00 00        	movlps	%xmm0, 0x168(%rbp)
1408e9848: 45 31 e4                    	xorl	%r12d, %r12d
1408e984b: eb 2f                       	jmp	0x1408e987c <.text+0x8e887c>
1408e984d: 0f 1f 00                    	nopl	(%rax)
1408e9850: 48 8b 85 a8 01 00 00        	movq	0x1a8(%rbp), %rax
1408e9857: 48 8d 0c 76                 	leaq	(%rsi,%rsi,2), %rcx
1408e985b: 49 8b 54 24 58              	movq	0x58(%r12), %rdx
1408e9860: 48 89 54 c8 10              	movq	%rdx, 0x10(%rax,%rcx,8)
1408e9865: 41 0f 10 44 24 48           	movups	0x48(%r12), %xmm0
1408e986b: 0f 11 04 c8                 	movups	%xmm0, (%rax,%rcx,8)
1408e986f: 48 ff c6                    	incq	%rsi
1408e9872: 48 89 b5 b0 01 00 00        	movq	%rsi, 0x1b0(%rbp)
1408e9879: 4d 89 ec                    	movq	%r13, %r12
1408e987c: 49 83 fc 02                 	cmpq	$0x2, %r12
1408e9880: 0f 84 da f6 ff ff           	je	0x1408e8f60 <.text+0x8e7f60>
1408e9886: 4d 8d 6c 24 01              	leaq	0x1(%r12), %r13
1408e988b: 4c 89 ad 70 01 00 00        	movq	%r13, 0x170(%rbp)
1408e9892: 48 8b b5 f8 01 00 00        	movq	0x1f8(%rbp), %rsi
1408e9899: 48 3b b5 e8 01 00 00        	cmpq	0x1e8(%rbp), %rsi
1408e98a0: 75 13                       	jne	0x1408e98b5 <.text+0x8e88b5>
1408e98a2: 48 8d 8d e8 01 00 00        	leaq	0x1e8(%rbp), %rcx
1408e98a9: 48 8d 15 60 cd 1b 02        	leaq	0x21bcd60(%rip), %rdx   # 0x142aa6610
1408e98b0: e8 7b 63 27 00              	callq	0x140b5fc30 <.text+0xb5ec30>
1408e98b5: 4b 8d 04 64                 	leaq	(%r12,%r12,2), %rax
1408e98b9: 48 c1 e0 05                 	shlq	$0x5, %rax
1408e98bd: 4c 8d 24 28                 	leaq	(%rax,%rbp), %r12
1408e98c1: 49 81 c4 b0 00 00 00        	addq	$0xb0, %r12
1408e98c8: 48 8b 85 f0 01 00 00        	movq	0x1f0(%rbp), %rax
1408e98cf: 48 8d 0c f6                 	leaq	(%rsi,%rsi,8), %rcx
1408e98d3: 41 8b 54 24 20              	movl	0x20(%r12), %edx
1408e98d8: 89 54 88 20                 	movl	%edx, 0x20(%rax,%rcx,4)
1408e98dc: 41 0f 10 04 24              	movups	(%r12), %xmm0
1408e98e1: 41 0f 10 4c 24 10           	movups	0x10(%r12), %xmm1
1408e98e7: 0f 11 4c 88 10              	movups	%xmm1, 0x10(%rax,%rcx,4)
1408e98ec: 0f 11 04 88                 	movups	%xmm0, (%rax,%rcx,4)
1408e98f0: 48 ff c6                    	incq	%rsi
1408e98f3: 48 89 b5 f8 01 00 00        	movq	%rsi, 0x1f8(%rbp)
1408e98fa: 48 8b b5 c8 01 00 00        	movq	0x1c8(%rbp), %rsi
1408e9901: 48 3b b5 b8 01 00 00        	cmpq	0x1b8(%rbp), %rsi
1408e9908: 75 0f                       	jne	0x1408e9919 <.text+0x8e8919>
1408e990a: 48 89 d9                    	movq	%rbx, %rcx
1408e990d: 48 8d 15 14 cd 1b 02        	leaq	0x21bcd14(%rip), %rdx   # 0x142aa6628
1408e9914: e8 17 63 27 00              	callq	0x140b5fc30 <.text+0xb5ec30>
1408e9919: 48 8b 85 c0 01 00 00        	movq	0x1c0(%rbp), %rax
1408e9920: 48 8d 0c f6                 	leaq	(%rsi,%rsi,8), %rcx
1408e9924: 41 8b 54 24 44              	movl	0x44(%r12), %edx
1408e9929: 89 54 88 20                 	movl	%edx, 0x20(%rax,%rcx,4)
1408e992d: 41 0f 10 44 24 24           	movups	0x24(%r12), %xmm0
1408e9933: 41 0f 10 4c 24 34           	movups	0x34(%r12), %xmm1
1408e9939: 0f 11 4c 88 10              	movups	%xmm1, 0x10(%rax,%rcx,4)
1408e993e: 0f 11 04 88                 	movups	%xmm0, (%rax,%rcx,4)
1408e9942: 48 ff c6                    	incq	%rsi
1408e9945: 48 89 b5 c8 01 00 00        	movq	%rsi, 0x1c8(%rbp)
1408e994c: 48 8b b5 b0 01 00 00        	movq	0x1b0(%rbp), %rsi
1408e9953: 48 3b b5 a0 01 00 00        	cmpq	0x1a0(%rbp), %rsi
1408e995a: 0f 85 f0 fe ff ff           	jne	0x1408e9850 <.text+0x8e8850>
1408e9960: 48 89 f9                    	movq	%rdi, %rcx
1408e9963: 4c 89 f2                    	movq	%r14, %rdx
1408e9966: e8 15 3a d9 ff              	callq	0x14067d380 <.text+0x67c380>
1408e996b: e9 e0 fe ff ff              	jmp	0x1408e9850 <.text+0x8e8850>
1408e9970: 48 83 bd f8 01 00 00 00     	cmpq	$0x0, 0x1f8(%rbp)
1408e9978: 0f 84 ee 01 00 00           	je	0x1408e9b6c <.text+0x8e8b6c>
1408e997e: 48 83 bd c8 01 00 00 00     	cmpq	$0x0, 0x1c8(%rbp)
1408e9986: 0f 84 fa 01 00 00           	je	0x1408e9b86 <.text+0x8e8b86>
1408e998c: 48 83 bd b0 01 00 00 00     	cmpq	$0x0, 0x1b0(%rbp)
1408e9994: 0f 84 06 02 00 00           	je	0x1408e9ba0 <.text+0x8e8ba0>
1408e999a: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
1408e99a1: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1408e99a5: 48 89 48 10                 	movq	%rcx, 0x10(%rax)
1408e99a9: 0f 10 85 e8 01 00 00        	movups	0x1e8(%rbp), %xmm0
1408e99b0: 0f 11 00                    	movups	%xmm0, (%rax)
1408e99b3: 0f 10 85 b8 01 00 00        	movups	0x1b8(%rbp), %xmm0
1408e99ba: 0f 11 40 18                 	movups	%xmm0, 0x18(%rax)
1408e99be: 48 8b 8d c8 01 00 00        	movq	0x1c8(%rbp), %rcx
1408e99c5: 48 89 48 28                 	movq	%rcx, 0x28(%rax)
1408e99c9: 0f 10 85 a0 01 00 00        	movups	0x1a0(%rbp), %xmm0
1408e99d0: 0f 11 40 30                 	movups	%xmm0, 0x30(%rax)
1408e99d4: 48 8b 8d b0 01 00 00        	movq	0x1b0(%rbp), %rcx
1408e99db: 48 89 48 40                 	movq	%rcx, 0x40(%rax)
1408e99df: 0f 28 b5 10 02 00 00        	movaps	0x210(%rbp), %xmm6
1408e99e6: 0f 28 bd 20 02 00 00        	movaps	0x220(%rbp), %xmm7
1408e99ed: 44 0f 28 85 30 02 00 00     	movaps	0x230(%rbp), %xmm8
1408e99f5: 44 0f 28 8d 40 02 00 00     	movaps	0x240(%rbp), %xmm9
1408e99fd: 44 0f 28 95 50 02 00 00     	movaps	0x250(%rbp), %xmm10
1408e9a05: 44 0f 28 9d 60 02 00 00     	movaps	0x260(%rbp), %xmm11
1408e9a0d: 44 0f 28 a5 70 02 00 00     	movaps	0x270(%rbp), %xmm12
1408e9a15: 44 0f 28 ad 80 02 00 00     	movaps	0x280(%rbp), %xmm13
1408e9a1d: 44 0f 28 b5 90 02 00 00     	movaps	0x290(%rbp), %xmm14
1408e9a25: 44 0f 28 bd a0 02 00 00     	movaps	0x2a0(%rbp), %xmm15
1408e9a2d: 48 81 c4 38 03 00 00        	addq	$0x338, %rsp            # imm = 0x338
1408e9a34: 5b                          	popq	%rbx
1408e9a35: 5f                          	popq	%rdi
1408e9a36: 5e                          	popq	%rsi
1408e9a37: 41 5c                       	popq	%r12
1408e9a39: 41 5d                       	popq	%r13
1408e9a3b: 41 5e                       	popq	%r14
1408e9a3d: 41 5f                       	popq	%r15
1408e9a3f: 5d                          	popq	%rbp
1408e9a40: c3                          	retq
1408e9a41: 4c 8d 05 48 b7 1b 02        	leaq	0x21bb748(%rip), %r8    # 0x142aa5190
1408e9a48: e8 33 1a 4a 00              	callq	0x140d8b480 <.text+0xd8a480>
1408e9a4d: e9 a6 01 00 00              	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9a52: 48 89 e8                    	movq	%rbp, %rax
1408e9a55: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1408e9a59: 48 8d 05 b0 2e 4a 00        	leaq	0x4a2eb0(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
1408e9a60: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1408e9a64: 48 8d 05 55 b9 1b 02        	leaq	0x21bb955(%rip), %rax   # 0x142aa53c0
1408e9a6b: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408e9a72: 48 c7 85 b8 00 00 00 01 00 00 00    	movq	$0x1, 0xb8(%rbp)
1408e9a7d: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
1408e9a88: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
1408e9a8c: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1408e9a93: 48 c7 85 c8 00 00 00 01 00 00 00    	movq	$0x1, 0xc8(%rbp)
1408e9a9e: 48 8d 15 db b9 1b 02        	leaq	0x21bb9db(%rip), %rdx   # 0x142aa5480
1408e9aa5: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1408e9aac: e8 df f8 fe 01              	callq	0x1428d9390 <.text+0x28d8390>
1408e9ab1: e9 42 01 00 00              	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9ab6: 48 8d 0d 13 b9 1b 02        	leaq	0x21bb913(%rip), %rcx   # 0x142aa53d0
1408e9abd: 4c 8d 05 bc b9 1b 02        	leaq	0x21bb9bc(%rip), %r8    # 0x142aa5480
1408e9ac4: ba 1d 00 00 00              	movl	$0x1d, %edx
1408e9ac9: e8 62 f9 fe 01              	callq	0x1428d9430 <.text+0x28d8430>
1408e9ace: e9 25 01 00 00              	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9ad3: 48 89 e8                    	movq	%rbp, %rax
1408e9ad6: 48 89 45 60                 	movq	%rax, 0x60(%rbp)
1408e9ada: 48 8d 05 2f 2e 4a 00        	leaq	0x4a2e2f(%rip), %rax    # 0x140d8c910 <.text+0xd8b910>
1408e9ae1: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
1408e9ae5: 48 8d 05 d4 b8 1b 02        	leaq	0x21bb8d4(%rip), %rax   # 0x142aa53c0
1408e9aec: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
1408e9af3: 48 c7 85 b8 00 00 00 01 00 00 00    	movq	$0x1, 0xb8(%rbp)
1408e9afe: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
1408e9b09: 48 8d 45 60                 	leaq	0x60(%rbp), %rax
1408e9b0d: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
1408e9b14: 48 c7 85 c8 00 00 00 01 00 00 00    	movq	$0x1, 0xc8(%rbp)
1408e9b1f: 48 8d 15 5a b9 1b 02        	leaq	0x21bb95a(%rip), %rdx   # 0x142aa5480
1408e9b26: 48 8d 8d b0 00 00 00        	leaq	0xb0(%rbp), %rcx
1408e9b2d: e8 5e f8 fe 01              	callq	0x1428d9390 <.text+0x28d8390>
1408e9b32: e9 c1 00 00 00              	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9b37: 48 8d 0d 92 b8 1b 02        	leaq	0x21bb892(%rip), %rcx   # 0x142aa53d0
1408e9b3e: 4c 8d 05 3b b9 1b 02        	leaq	0x21bb93b(%rip), %r8    # 0x142aa5480
1408e9b45: ba 1d 00 00 00              	movl	$0x1d, %edx
1408e9b4a: e8 e1 f8 fe 01              	callq	0x1428d9430 <.text+0x28d8430>
1408e9b4f: e9 a4 00 00 00              	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9b54: 48 8d 0d 65 c9 1b 02        	leaq	0x21bc965(%rip), %rcx   # 0x142aa64c0
1408e9b5b: 4c 8d 05 96 ca 1b 02        	leaq	0x21bca96(%rip), %r8    # 0x142aa65f8
1408e9b62: ba 28 00 00 00              	movl	$0x28, %edx
1408e9b67: e8 c4 f8 fe 01              	callq	0x1428d9430 <.text+0x28d8430>
1408e9b6c: 48 8d 0d 0d c9 1b 02        	leaq	0x21bc90d(%rip), %rcx   # 0x142aa6480
1408e9b73: 4c 8d 05 66 ca 1b 02        	leaq	0x21bca66(%rip), %r8    # 0x142aa65e0
1408e9b7a: ba 27 00 00 00              	movl	$0x27, %edx
1408e9b7f: e8 ac f8 fe 01              	callq	0x1428d9430 <.text+0x28d8430>
1408e9b84: eb 72                       	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9b86: 48 8d 0d 13 ca 1b 02        	leaq	0x21bca13(%rip), %rcx   # 0x142aa65a0
1408e9b8d: 4c 8d 05 34 ca 1b 02        	leaq	0x21bca34(%rip), %r8    # 0x142aa65c8
1408e9b94: ba 25 00 00 00              	movl	$0x25, %edx
1408e9b99: e8 92 f8 fe 01              	callq	0x1428d9430 <.text+0x28d8430>
1408e9b9e: eb 58                       	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9ba0: 48 8d 0d b9 c9 1b 02        	leaq	0x21bc9b9(%rip), %rcx   # 0x142aa6560
1408e9ba7: 4c 8d 05 da c9 1b 02        	leaq	0x21bc9da(%rip), %r8    # 0x142aa6588
1408e9bae: ba 26 00 00 00              	movl	$0x26, %edx
1408e9bb3: e8 78 f8 fe 01              	callq	0x1428d9430 <.text+0x28d8430>
1408e9bb8: eb 3e                       	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9bba: 4c 8d 05 57 c9 1b 02        	leaq	0x21bc957(%rip), %r8    # 0x142aa6518
1408e9bc1: b9 04 00 00 00              	movl	$0x4, %ecx
1408e9bc6: 48 89 da                    	movq	%rbx, %rdx
1408e9bc9: e8 15 f4 fe 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408e9bce: 4c 8d 05 5b c9 1b 02        	leaq	0x21bc95b(%rip), %r8    # 0x142aa6530
1408e9bd5: b9 04 00 00 00              	movl	$0x4, %ecx
1408e9bda: 48 89 da                    	movq	%rbx, %rdx
1408e9bdd: e8 01 f4 fe 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408e9be2: eb 14                       	jmp	0x1408e9bf8 <.text+0x8e8bf8>
1408e9be4: 4c 8d 05 5d c9 1b 02        	leaq	0x21bc95d(%rip), %r8    # 0x142aa6548
1408e9beb: b9 04 00 00 00              	movl	$0x4, %ecx
1408e9bf0: 48 89 da                    	movq	%rbx, %rdx
1408e9bf3: e8 eb f3 fe 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1408e9bf8: 0f 0b                       	ud2
1408e9bfa: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1408e9c00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408e9c05: 55                          	pushq	%rbp
1408e9c06: 41 57                       	pushq	%r15
1408e9c08: 41 56                       	pushq	%r14
1408e9c0a: 41 55                       	pushq	%r13
1408e9c0c: 41 54                       	pushq	%r12
1408e9c0e: 56                          	pushq	%rsi
1408e9c0f: 57                          	pushq	%rdi
1408e9c10: 53                          	pushq	%rbx
1408e9c11: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1408e9c18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408e9c1f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1408e9c25: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1408e9c2b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1408e9c31: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1408e9c37: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1408e9c3d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1408e9c43: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1408e9c4c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1408e9c55: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1408e9c5d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1408e9c65: 48 8b 85 e8 01 00 00        	movq	0x1e8(%rbp), %rax
1408e9c6c: 48 85 c0                    	testq	%rax, %rax
1408e9c6f: 74 1a                       	je	0x1408e9c8b <.text+0x8e8c8b>
1408e9c71: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
1408e9c78: 48 c1 e0 02                 	shlq	$0x2, %rax
1408e9c7c: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1408e9c80: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1408e9c86: e8 95 9f d2 ff              	callq	0x140613c20 <.text+0x612c20>
1408e9c8b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1408e9c93: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1408e9c9b: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1408e9ca4: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1408e9cad: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1408e9cb3: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1408e9cb9: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1408e9cbf: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1408e9cc5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1408e9ccb: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1408e9cd1: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1408e9cd8: 5b                          	popq	%rbx
1408e9cd9: 5f                          	popq	%rdi
1408e9cda: 5e                          	popq	%rsi
1408e9cdb: 41 5c                       	popq	%r12
1408e9cdd: 41 5d                       	popq	%r13
1408e9cdf: 41 5e                       	popq	%r14
1408e9ce1: 41 5f                       	popq	%r15
1408e9ce3: 5d                          	popq	%rbp
1408e9ce4: c3                          	retq
1408e9ce5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1408e9cf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408e9cf5: 55                          	pushq	%rbp
1408e9cf6: 41 57                       	pushq	%r15
1408e9cf8: 41 56                       	pushq	%r14
1408e9cfa: 41 55                       	pushq	%r13
1408e9cfc: 41 54                       	pushq	%r12
1408e9cfe: 56                          	pushq	%rsi
1408e9cff: 57                          	pushq	%rdi
1408e9d00: 53                          	pushq	%rbx
1408e9d01: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1408e9d08: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408e9d0f: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1408e9d15: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1408e9d1b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1408e9d21: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1408e9d27: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1408e9d2d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1408e9d33: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1408e9d3c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1408e9d45: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1408e9d4d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1408e9d55: 48 8b 85 b8 01 00 00        	movq	0x1b8(%rbp), %rax
1408e9d5c: 48 85 c0                    	testq	%rax, %rax
1408e9d5f: 74 1a                       	je	0x1408e9d7b <.text+0x8e8d7b>
1408e9d61: 48 8b 8d c0 01 00 00        	movq	0x1c0(%rbp), %rcx
1408e9d68: 48 c1 e0 02                 	shlq	$0x2, %rax
1408e9d6c: 48 8d 14 c0                 	leaq	(%rax,%rax,8), %rdx
1408e9d70: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1408e9d76: e8 a5 9e d2 ff              	callq	0x140613c20 <.text+0x612c20>
1408e9d7b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1408e9d83: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1408e9d8b: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1408e9d94: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1408e9d9d: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1408e9da3: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1408e9da9: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1408e9daf: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1408e9db5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1408e9dbb: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1408e9dc1: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1408e9dc8: 5b                          	popq	%rbx
1408e9dc9: 5f                          	popq	%rdi
1408e9dca: 5e                          	popq	%rsi
1408e9dcb: 41 5c                       	popq	%r12
1408e9dcd: 41 5d                       	popq	%r13
1408e9dcf: 41 5e                       	popq	%r14
1408e9dd1: 41 5f                       	popq	%r15
1408e9dd3: 5d                          	popq	%rbp
1408e9dd4: c3                          	retq
1408e9dd5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
1408e9de0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1408e9de5: 55                          	pushq	%rbp
1408e9de6: 41 57                       	pushq	%r15
1408e9de8: 41 56                       	pushq	%r14
1408e9dea: 41 55                       	pushq	%r13
1408e9dec: 41 54                       	pushq	%r12
1408e9dee: 56                          	pushq	%rsi
1408e9def: 57                          	pushq	%rdi
1408e9df0: 53                          	pushq	%rbx
1408e9df1: 48 81 ec c8 00 00 00        	subq	$0xc8, %rsp
1408e9df8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1408e9dff: 44 0f 29 7c 24 20           	movaps	%xmm15, 0x20(%rsp)
1408e9e05: 44 0f 29 74 24 30           	movaps	%xmm14, 0x30(%rsp)
1408e9e0b: 44 0f 29 6c 24 40           	movaps	%xmm13, 0x40(%rsp)
1408e9e11: 44 0f 29 64 24 50           	movaps	%xmm12, 0x50(%rsp)
1408e9e17: 44 0f 29 5c 24 60           	movaps	%xmm11, 0x60(%rsp)
1408e9e1d: 44 0f 29 54 24 70           	movaps	%xmm10, 0x70(%rsp)
1408e9e23: 44 0f 29 8c 24 80 00 00 00  	movaps	%xmm9, 0x80(%rsp)
1408e9e2c: 44 0f 29 84 24 90 00 00 00  	movaps	%xmm8, 0x90(%rsp)
1408e9e35: 0f 29 bc 24 a0 00 00 00     	movaps	%xmm7, 0xa0(%rsp)
1408e9e3d: 0f 29 b4 24 b0 00 00 00     	movaps	%xmm6, 0xb0(%rsp)
1408e9e45: 48 8b 85 a0 01 00 00        	movq	0x1a0(%rbp), %rax
1408e9e4c: 48 85 c0                    	testq	%rax, %rax
1408e9e4f: 74 1a                       	je	0x1408e9e6b <.text+0x8e8e6b>
1408e9e51: 48 8b 8d a8 01 00 00        	movq	0x1a8(%rbp), %rcx
1408e9e58: 48 c1 e0 03                 	shlq	$0x3, %rax
1408e9e5c: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
1408e9e60: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1408e9e66: e8 b5 9d d2 ff              	callq	0x140613c20 <.text+0x612c20>
1408e9e6b: 0f 28 b4 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm6
1408e9e73: 0f 28 bc 24 a0 00 00 00     	movaps	0xa0(%rsp), %xmm7
1408e9e7b: 44 0f 28 84 24 90 00 00 00  	movaps	0x90(%rsp), %xmm8
1408e9e84: 44 0f 28 8c 24 80 00 00 00  	movaps	0x80(%rsp), %xmm9
1408e9e8d: 44 0f 28 54 24 70           	movaps	0x70(%rsp), %xmm10
1408e9e93: 44 0f 28 5c 24 60           	movaps	0x60(%rsp), %xmm11
1408e9e99: 44 0f 28 64 24 50           	movaps	0x50(%rsp), %xmm12
1408e9e9f: 44 0f 28 6c 24 40           	movaps	0x40(%rsp), %xmm13
1408e9ea5: 44 0f 28 74 24 30           	movaps	0x30(%rsp), %xmm14
1408e9eab: 44 0f 28 7c 24 20           	movaps	0x20(%rsp), %xmm15
1408e9eb1: 48 81 c4 c8 00 00 00        	addq	$0xc8, %rsp
1408e9eb8: 5b                          	popq	%rbx
1408e9eb9: 5f                          	popq	%rdi
1408e9eba: 5e                          	popq	%rsi
1408e9ebb: 41 5c                       	popq	%r12
1408e9ebd: 41 5d                       	popq	%r13
1408e9ebf: 41 5e                       	popq	%r14
1408e9ec1: 41 5f                       	popq	%r15
1408e9ec3: 5d                          	popq	%rbp
1408e9ec4: c3                          	retq
