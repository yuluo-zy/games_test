
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a2ce90: 55                          	pushq	%rbp
140a2ce91: 41 57                       	pushq	%r15
140a2ce93: 41 56                       	pushq	%r14
140a2ce95: 41 55                       	pushq	%r13
140a2ce97: 41 54                       	pushq	%r12
140a2ce99: 56                          	pushq	%rsi
140a2ce9a: 57                          	pushq	%rdi
140a2ce9b: 53                          	pushq	%rbx
140a2ce9c: 48 81 ec 68 03 00 00        	subq	$0x368, %rsp            # imm = 0x368
140a2cea3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a2ceab: 66 44 0f 7f 85 d0 02 00 00  	movdqa	%xmm8, 0x2d0(%rbp)
140a2ceb4: 66 0f 7f bd c0 02 00 00     	movdqa	%xmm7, 0x2c0(%rbp)
140a2cebc: 66 0f 7f b5 b0 02 00 00     	movdqa	%xmm6, 0x2b0(%rbp)
140a2cec4: 48 c7 85 a8 02 00 00 fe ff ff ff    	movq	$-0x2, 0x2a8(%rbp)
140a2cecf: 48 8b 85 58 03 00 00        	movq	0x358(%rbp), %rax
140a2ced6: 48 8b 8d 50 03 00 00        	movq	0x350(%rbp), %rcx
140a2cedd: 0f 10 02                    	movups	(%rdx), %xmm0
140a2cee0: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140a2cee4: 0f 29 4d 00                 	movaps	%xmm1, (%rbp)
140a2cee8: 0f 29 45 f0                 	movaps	%xmm0, -0x10(%rbp)
140a2ceec: 41 0f 10 00                 	movups	(%r8), %xmm0
140a2cef0: 41 0f 10 48 10              	movups	0x10(%r8), %xmm1
140a2cef5: 0f 29 4d 20                 	movaps	%xmm1, 0x20(%rbp)
140a2cef9: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
140a2cefd: 41 0f 10 01                 	movups	(%r9), %xmm0
140a2cf01: 41 0f 10 49 10              	movups	0x10(%r9), %xmm1
140a2cf06: 0f 29 4d 40                 	movaps	%xmm1, 0x40(%rbp)
140a2cf0a: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
140a2cf0e: 0f 10 01                    	movups	(%rcx), %xmm0
140a2cf11: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
140a2cf15: 0f 29 45 50                 	movaps	%xmm0, 0x50(%rbp)
140a2cf19: 0f 29 4d 60                 	movaps	%xmm1, 0x60(%rbp)
140a2cf1d: 0f 10 00                    	movups	(%rax), %xmm0
140a2cf20: 0f 10 48 10                 	movups	0x10(%rax), %xmm1
140a2cf24: 0f 29 45 70                 	movaps	%xmm0, 0x70(%rbp)
140a2cf28: 0f 29 8d 80 00 00 00        	movaps	%xmm1, 0x80(%rbp)
140a2cf2f: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a2cf33: 48 8b 10                    	movq	(%rax), %rdx
140a2cf36: 48 83 c2 10                 	addq	$0x10, %rdx
140a2cf3a: 8b 45 4c                    	movl	0x4c(%rbp), %eax
140a2cf3d: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
140a2cf41: 89 01                       	movl	%eax, (%rcx)
140a2cf43: 48 8b 45 30                 	movq	0x30(%rbp), %rax
140a2cf47: 48 89 85 f0 01 00 00        	movq	%rax, 0x1f0(%rbp)
140a2cf4e: 4c 8d bd 60 01 00 00        	leaq	0x160(%rbp), %r15
140a2cf55: 4c 89 f9                    	movq	%r15, %rcx
140a2cf58: 48 89 95 78 02 00 00        	movq	%rdx, 0x278(%rbp)
140a2cf5f: e8 8c 4e 0d 00              	callq	0x140b01df0 <.text+0xb00df0>
140a2cf64: 48 8b 95 70 01 00 00        	movq	0x170(%rbp), %rdx
140a2cf6b: 48 89 95 90 02 00 00        	movq	%rdx, 0x290(%rbp)
140a2cf72: f3 0f 6f 85 60 01 00 00     	movdqu	0x160(%rbp), %xmm0
140a2cf7a: 66 0f 7f 85 80 02 00 00     	movdqa	%xmm0, 0x280(%rbp)
140a2cf82: 4c 8b 85 78 01 00 00        	movq	0x178(%rbp), %r8
140a2cf89: 48 8b 8d 80 01 00 00        	movq	0x180(%rbp), %rcx
140a2cf90: 4c 6b 8d 88 01 00 00 38     	imulq	$0x38, 0x188(%rbp), %r9
140a2cf98: 48 8b 85 88 02 00 00        	movq	0x288(%rbp), %rax
140a2cf9f: 48 8d 14 92                 	leaq	(%rdx,%rdx,4), %rdx
140a2cfa3: 48 8d 14 d0                 	leaq	(%rax,%rdx,8), %rdx
140a2cfa7: 49 01 c9                    	addq	%rcx, %r9
140a2cfaa: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140a2cfae: f3 0f 7f 85 40 01 00 00     	movdqu	%xmm0, 0x140(%rbp)
140a2cfb6: 48 c7 85 50 01 00 00 00 00 00 00    	movq	$0x0, 0x150(%rbp)
140a2cfc1: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140a2cfc8: 48 89 95 f8 01 00 00        	movq	%rdx, 0x1f8(%rbp)
140a2cfcf: 48 89 95 18 01 00 00        	movq	%rdx, 0x118(%rbp)
140a2cfd6: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
140a2cfdd: 48 89 8d 28 01 00 00        	movq	%rcx, 0x128(%rbp)
140a2cfe4: 4c 89 85 30 01 00 00        	movq	%r8, 0x130(%rbp)
140a2cfeb: 4c 89 8d 00 02 00 00        	movq	%r9, 0x200(%rbp)
140a2cff2: 4c 89 8d 38 01 00 00        	movq	%r9, 0x138(%rbp)
140a2cff9: 48 8d b5 10 02 00 00        	leaq	0x210(%rbp), %rsi
140a2d000: bf 01 00 00 00              	movl	$0x1, %edi
140a2d005: eb 0f                       	jmp	0x140a2d016 <.text+0xa2c016>
140a2d007: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a2d010: 4c 89 f1                    	movq	%r14, %rcx
140a2d013: 4c 89 e0                    	movq	%r12, %rax
140a2d016: 48 3b 85 f8 01 00 00        	cmpq	0x1f8(%rbp), %rax
140a2d01d: 0f 84 36 02 00 00           	je	0x140a2d259 <.text+0xa2c259>
140a2d023: 4c 8d 60 28                 	leaq	0x28(%rax), %r12
140a2d027: 4c 89 a5 10 01 00 00        	movq	%r12, 0x110(%rbp)
140a2d02e: 48 3b 8d 00 02 00 00        	cmpq	0x200(%rbp), %rcx
140a2d035: 0f 84 1e 02 00 00           	je	0x140a2d259 <.text+0xa2c259>
140a2d03b: 4c 8d 71 38                 	leaq	0x38(%rcx), %r14
140a2d03f: 4c 89 b5 28 01 00 00        	movq	%r14, 0x128(%rbp)
140a2d046: 0f b6 51 36                 	movzbl	0x36(%rcx), %edx
140a2d04a: 80 fa 07                    	cmpb	$0x7, %dl
140a2d04d: 0f 84 06 02 00 00           	je	0x140a2d259 <.text+0xa2c259>
140a2d053: 44 0f b6 41 37              	movzbl	0x37(%rcx), %r8d
140a2d058: 4c 8b 49 2e                 	movq	0x2e(%rcx), %r9
140a2d05c: 4c 89 4d de                 	movq	%r9, -0x22(%rbp)
140a2d060: 0f 10 01                    	movups	(%rcx), %xmm0
140a2d063: 0f 10 49 10                 	movups	0x10(%rcx), %xmm1
140a2d067: 0f 10 51 20                 	movups	0x20(%rcx), %xmm2
140a2d06b: 0f 29 55 d0                 	movaps	%xmm2, -0x30(%rbp)
140a2d06f: 0f 29 4d c0                 	movaps	%xmm1, -0x40(%rbp)
140a2d073: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
140a2d077: 48 8b 4d de                 	movq	-0x22(%rbp), %rcx
140a2d07b: 49 89 4f 46                 	movq	%rcx, 0x46(%r15)
140a2d07f: 0f 28 55 d0                 	movaps	-0x30(%rbp), %xmm2
140a2d083: 41 0f 11 57 38              	movups	%xmm2, 0x38(%r15)
140a2d088: 41 0f 11 4f 28              	movups	%xmm1, 0x28(%r15)
140a2d08d: 41 0f 11 47 18              	movups	%xmm0, 0x18(%r15)
140a2d092: 88 95 ae 01 00 00           	movb	%dl, 0x1ae(%rbp)
140a2d098: 44 88 85 af 01 00 00        	movb	%r8b, 0x1af(%rbp)
140a2d09f: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
140a2d0a9: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
140a2d0b0: 0f 10 40 08                 	movups	0x8(%rax), %xmm0
140a2d0b4: 0f 11 85 98 00 00 00        	movups	%xmm0, 0x98(%rbp)
140a2d0bb: 48 8d 85 98 00 00 00        	leaq	0x98(%rbp), %rax
140a2d0c2: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
140a2d0c9: 48 8d 05 c0 cd 6d ff        	leaq	-0x923240(%rip), %rax   # 0x140109e90 <.text+0x108e90>
140a2d0d0: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
140a2d0d7: 48 8d 05 ca 9b 0b 02        	leaq	0x20b9bca(%rip), %rax   # 0x142ae6ca8
140a2d0de: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
140a2d0e5: 48 c7 85 18 02 00 00 01 00 00 00    	movq	$0x1, 0x218(%rbp)
140a2d0f0: 48 8d 85 a8 00 00 00        	leaq	0xa8(%rbp), %rax
140a2d0f7: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140a2d0fe: 48 c7 85 28 02 00 00 01 00 00 00    	movq	$0x1, 0x228(%rbp)
140a2d109: 48 c7 85 30 02 00 00 00 00 00 00    	movq	$0x0, 0x230(%rbp)
140a2d114: 48 8d 8d b8 00 00 00        	leaq	0xb8(%rbp), %rcx
140a2d11b: 48 89 f2                    	movq	%rsi, %rdx
140a2d11e: e8 6d ce 34 00              	callq	0x140d79f90 <.text+0xd78f90>
140a2d123: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
140a2d12a: 48 89 85 70 02 00 00        	movq	%rax, 0x270(%rbp)
140a2d131: 4c 8b ad c8 00 00 00        	movq	0xc8(%rbp), %r13
140a2d138: 4d 85 ed                    	testq	%r13, %r13
140a2d13b: 0f 88 57 09 00 00           	js	0x140a2da98 <.text+0xa2ca98>
140a2d141: 74 2d                       	je	0x140a2d170 <.text+0xa2c170>
140a2d143: 0f b6 05 bf 7a c0 02        	movzbl	0x2c07abf(%rip), %eax   # 0x143634c09
140a2d14a: ba 01 00 00 00              	movl	$0x1, %edx
140a2d14f: 4c 89 e9                    	movq	%r13, %rcx
140a2d152: e8 b9 6a be ff              	callq	0x140613c10 <.text+0x612c10>
140a2d157: 48 85 c0                    	testq	%rax, %rax
140a2d15a: 0f 84 3a 09 00 00           	je	0x140a2da9a <.text+0xa2ca9a>
140a2d160: 48 89 c3                    	movq	%rax, %rbx
140a2d163: eb 10                       	jmp	0x140a2d175 <.text+0xa2c175>
140a2d165: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a2d170: bb 01 00 00 00              	movl	$0x1, %ebx
140a2d175: 48 89 d9                    	movq	%rbx, %rcx
140a2d178: 48 8b 95 70 02 00 00        	movq	0x270(%rbp), %rdx
140a2d17f: 4d 89 e8                    	movq	%r13, %r8
140a2d182: e8 79 6a ef 01              	callq	0x142923c00 <.text+0x2922c00>
140a2d187: 48 8b 95 60 01 00 00        	movq	0x160(%rbp), %rdx
140a2d18e: 48 89 d0                    	movq	%rdx, %rax
140a2d191: 48 f7 d8                    	negq	%rax
140a2d194: 70 14                       	jo	0x140a2d1aa <.text+0xa2c1aa>
140a2d196: 73 12                       	jae	0x140a2d1aa <.text+0xa2c1aa>
140a2d198: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
140a2d19f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a2d1a5: e8 76 6a be ff              	callq	0x140613c20 <.text+0x612c20>
140a2d1aa: 4c 89 ad 60 01 00 00        	movq	%r13, 0x160(%rbp)
140a2d1b1: 48 89 9d 68 01 00 00        	movq	%rbx, 0x168(%rbp)
140a2d1b8: 4c 89 ad 70 01 00 00        	movq	%r13, 0x170(%rbp)
140a2d1bf: f3 0f 6f 85 80 01 00 00     	movdqu	0x180(%rbp), %xmm0
140a2d1c7: 0f 10 8d 90 01 00 00        	movups	0x190(%rbp), %xmm1
140a2d1ce: 0f 10 95 a0 01 00 00        	movups	0x1a0(%rbp), %xmm2
140a2d1d5: 66 0f 7f 85 30 02 00 00     	movdqa	%xmm0, 0x230(%rbp)
140a2d1dd: 0f 29 8d 40 02 00 00        	movaps	%xmm1, 0x240(%rbp)
140a2d1e4: 0f 29 95 50 02 00 00        	movaps	%xmm2, 0x250(%rbp)
140a2d1eb: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
140a2d1f2: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
140a2d1f9: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
140a2d200: 48 89 85 18 02 00 00        	movq	%rax, 0x218(%rbp)
140a2d207: 48 8b 85 70 01 00 00        	movq	0x170(%rbp), %rax
140a2d20e: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140a2d215: 48 8b 85 78 01 00 00        	movq	0x178(%rbp), %rax
140a2d21c: 48 89 85 28 02 00 00        	movq	%rax, 0x228(%rbp)
140a2d223: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
140a2d22a: 48 89 f2                    	movq	%rsi, %rdx
140a2d22d: e8 ce ef f0 ff              	callq	0x14093c200 <.text+0x93b200>
140a2d232: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
140a2d239: 48 85 d2                    	testq	%rdx, %rdx
140a2d23c: 0f 84 ce fd ff ff           	je	0x140a2d010 <.text+0xa2c010>
140a2d242: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a2d248: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
140a2d24f: e8 cc 69 be ff              	callq	0x140613c20 <.text+0x612c20>
140a2d254: e9 b7 fd ff ff              	jmp	0x140a2d010 <.text+0xa2c010>
140a2d259: c6 85 a6 02 00 00 01        	movb	$0x1, 0x2a6(%rbp)
140a2d260: 4c 8d bd 10 01 00 00        	leaq	0x110(%rbp), %r15
140a2d267: 4c 89 f9                    	movq	%r15, %rcx
140a2d26a: e8 11 59 e5 ff              	callq	0x140882b80 <.text+0x881b80>
140a2d26f: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
140a2d276: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140a2d27d: 66 0f 6f 85 80 02 00 00     	movdqa	0x280(%rbp), %xmm0
140a2d285: 66 0f 7f 85 60 01 00 00     	movdqa	%xmm0, 0x160(%rbp)
140a2d28d: c6 85 a6 02 00 00 00        	movb	$0x0, 0x2a6(%rbp)
140a2d294: 4c 8d 05 4d e5 09 02        	leaq	0x209e54d(%rip), %r8    # 0x142acb7e8
140a2d29b: 4c 8d a5 60 01 00 00        	leaq	0x160(%rbp), %r12
140a2d2a2: 48 8d 4d 70                 	leaq	0x70(%rbp), %rcx
140a2d2a6: 4c 89 e2                    	movq	%r12, %rdx
140a2d2a9: e8 02 40 8b 00              	callq	0x1412e12b0 <.text+0x12e02b0>
140a2d2ae: 8b 05 94 7a c0 02           	movl	0x2c07a94(%rip), %eax   # 0x143634d48
140a2d2b4: 83 f8 03                    	cmpl	$0x3, %eax
140a2d2b7: 0f 85 af 07 00 00           	jne	0x140a2da6c <.text+0xa2ca6c>
140a2d2bd: 48 8b 35 3c 7a c0 02        	movq	0x2c07a3c(%rip), %rsi   # 0x143634d00
140a2d2c4: e8 67 84 31 00              	callq	0x140d45730 <.text+0xd44730>
140a2d2c9: 48 89 c3                    	movq	%rax, %rbx
140a2d2cc: 31 c0                       	xorl	%eax, %eax
140a2d2ce: 48 39 f0                    	cmpq	%rsi, %rax
140a2d2d1: 0f 81 7a 02 00 00           	jno	0x140a2d551 <.text+0xa2c551>
140a2d2d7: 48 c7 85 80 02 00 00 04 00 00 00    	movq	$0x4, 0x280(%rbp)
140a2d2e2: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
140a2d2e9: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
140a2d2f0: 48 8b 35 19 7a c0 02        	movq	0x2c07a19(%rip), %rsi   # 0x143634d10
140a2d2f7: 48 8b 3d 1a 7a c0 02        	movq	0x2c07a1a(%rip), %rdi   # 0x143634d18
140a2d2fe: 48 8d 85 b8 00 00 00        	leaq	0xb8(%rbp), %rax
140a2d305: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140a2d30c: 31 c9                       	xorl	%ecx, %ecx
140a2d30e: e8 2d e5 ee ff              	callq	0x14091b840 <.text+0x91a840>
140a2d313: 48 85 c0                    	testq	%rax, %rax
140a2d316: 0f 84 a9 07 00 00           	je	0x140a2dac5 <.text+0xa2cac5>
140a2d31c: 48 89 b5 60 01 00 00        	movq	%rsi, 0x160(%rbp)
140a2d323: 48 89 bd 68 01 00 00        	movq	%rdi, 0x168(%rbp)
140a2d32a: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
140a2d331: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
140a2d338: 4c 89 bd 78 01 00 00        	movq	%r15, 0x178(%rbp)
140a2d33f: 4c 8b 08                    	movq	(%rax), %r9
140a2d342: 49 83 c1 10                 	addq	$0x10, %r9
140a2d346: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140a2d34b: 4c 89 4c 24 20              	movq	%r9, 0x20(%rsp)
140a2d350: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
140a2d357: 48 89 da                    	movq	%rbx, %rdx
140a2d35a: 41 b0 01                    	movb	$0x1, %r8b
140a2d35d: e8 ce 26 e8 ff              	callq	0x1408afa30 <.text+0x8aea30>
140a2d362: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
140a2d369: 48 89 c1                    	movq	%rax, %rcx
140a2d36c: 48 f7 d9                    	negq	%rcx
140a2d36f: 0f 80 50 07 00 00           	jo	0x140a2dac5 <.text+0xa2cac5>
140a2d375: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
140a2d37c: 48 8b 95 20 02 00 00        	movq	0x220(%rbp), %rdx
140a2d383: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140a2d387: 48 8d 14 d1                 	leaq	(%rcx,%rdx,8), %rdx
140a2d38b: 4c 8b 05 7e 79 c0 02        	movq	0x2c0797e(%rip), %r8    # 0x143634d10
140a2d392: 4c 8b 0d 7f 79 c0 02        	movq	0x2c0797f(%rip), %r9    # 0x143634d18
140a2d399: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
140a2d39d: 4f 8d 0c c8                 	leaq	(%r8,%r9,8), %r9
140a2d3a1: 66 0f ef c0                 	pxor	%xmm0, %xmm0
140a2d3a5: f3 0f 7f 85 d0 01 00 00     	movdqu	%xmm0, 0x1d0(%rbp)
140a2d3ad: 48 c7 85 e0 01 00 00 00 00 00 00    	movq	$0x0, 0x1e0(%rbp)
140a2d3b8: 4c 89 85 60 01 00 00        	movq	%r8, 0x160(%rbp)
140a2d3bf: 4c 89 8d 68 01 00 00        	movq	%r9, 0x168(%rbp)
140a2d3c6: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
140a2d3cd: 48 89 8d 78 01 00 00        	movq	%rcx, 0x178(%rbp)
140a2d3d4: 48 89 85 80 01 00 00        	movq	%rax, 0x180(%rbp)
140a2d3db: 48 89 95 88 01 00 00        	movq	%rdx, 0x188(%rbp)
140a2d3e2: 48 c7 85 90 01 00 00 00 00 00 00    	movq	$0x0, 0x190(%rbp)
140a2d3ed: 48 c7 85 b0 01 00 00 00 00 00 00    	movq	$0x0, 0x1b0(%rbp)
140a2d3f8: 48 8d bd 28 02 00 00        	leaq	0x228(%rbp), %rdi
140a2d3ff: 48 8d b5 b8 00 00 00        	leaq	0xb8(%rbp), %rsi
140a2d406: 48 8d 9d 60 01 00 00        	leaq	0x160(%rbp), %rbx
140a2d40d: 41 be 01 00 00 00           	movl	$0x1, %r14d
140a2d413: e9 a0 00 00 00              	jmp	0x140a2d4b8 <.text+0xa2c4b8>
140a2d418: 41 bd 01 00 00 00           	movl	$0x1, %r13d
140a2d41e: 4c 89 e9                    	movq	%r13, %rcx
140a2d421: 4c 89 e2                    	movq	%r12, %rdx
140a2d424: 4d 89 f8                    	movq	%r15, %r8
140a2d427: e8 d4 67 ef 01              	callq	0x142923c00 <.text+0x2922c00>
140a2d42c: 4c 89 bd 10 02 00 00        	movq	%r15, 0x210(%rbp)
140a2d433: 4c 89 ad 18 02 00 00        	movq	%r13, 0x218(%rbp)
140a2d43a: 4c 89 bd 20 02 00 00        	movq	%r15, 0x220(%rbp)
140a2d441: f3 0f 6f 85 30 02 00 00     	movdqu	0x230(%rbp), %xmm0
140a2d449: 0f 10 8d 40 02 00 00        	movups	0x240(%rbp), %xmm1
140a2d450: 0f 10 95 50 02 00 00        	movups	0x250(%rbp), %xmm2
140a2d457: 66 0f 7f 85 30 01 00 00     	movdqa	%xmm0, 0x130(%rbp)
140a2d45f: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140a2d466: 0f 29 95 50 01 00 00        	movaps	%xmm2, 0x150(%rbp)
140a2d46d: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
140a2d474: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140a2d47b: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
140a2d482: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140a2d489: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
140a2d490: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140a2d497: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
140a2d49e: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140a2d4a5: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
140a2d4ac: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
140a2d4b3: e8 48 ed f0 ff              	callq	0x14093c200 <.text+0x93b200>
140a2d4b8: 48 89 f1                    	movq	%rsi, %rcx
140a2d4bb: 48 89 da                    	movq	%rbx, %rdx
140a2d4be: e8 5d cd ff ff              	callq	0x140a2a220 <.text+0xa29220>
140a2d4c3: 0f b6 85 f6 00 00 00        	movzbl	0xf6(%rbp), %eax
140a2d4ca: 83 f8 07                    	cmpl	$0x7, %eax
140a2d4cd: 74 e9                       	je	0x140a2d4b8 <.text+0xa2c4b8>
140a2d4cf: 83 f8 08                    	cmpl	$0x8, %eax
140a2d4d2: 0f 84 5e 05 00 00           	je	0x140a2da36 <.text+0xa2ca36>
140a2d4d8: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140a2d4df: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
140a2d4e6: 48 8b 4a 30                 	movq	0x30(%rdx), %rcx
140a2d4ea: 48 89 4f 30                 	movq	%rcx, 0x30(%rdi)
140a2d4ee: 0f 10 02                    	movups	(%rdx), %xmm0
140a2d4f1: 0f 10 4a 10                 	movups	0x10(%rdx), %xmm1
140a2d4f5: 0f 10 52 20                 	movups	0x20(%rdx), %xmm2
140a2d4f9: 0f 11 57 20                 	movups	%xmm2, 0x20(%rdi)
140a2d4fd: 0f 11 4f 10                 	movups	%xmm1, 0x10(%rdi)
140a2d501: 0f 11 07                    	movups	%xmm0, (%rdi)
140a2d504: 48 b9 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rcx # imm = 0x8000000000000000
140a2d50e: 48 89 8d 10 02 00 00        	movq	%rcx, 0x210(%rbp)
140a2d515: 4c 8b 78 10                 	movq	0x10(%rax), %r15
140a2d519: 4d 85 ff                    	testq	%r15, %r15
140a2d51c: 0f 88 5f 05 00 00           	js	0x140a2da81 <.text+0xa2ca81>
140a2d522: 4c 8b 60 08                 	movq	0x8(%rax), %r12
140a2d526: 0f 84 ec fe ff ff           	je	0x140a2d418 <.text+0xa2c418>
140a2d52c: 0f b6 05 d6 76 c0 02        	movzbl	0x2c076d6(%rip), %eax   # 0x143634c09
140a2d533: ba 01 00 00 00              	movl	$0x1, %edx
140a2d538: 4c 89 f9                    	movq	%r15, %rcx
140a2d53b: e8 d0 66 be ff              	callq	0x140613c10 <.text+0x612c10>
140a2d540: 48 85 c0                    	testq	%rax, %rax
140a2d543: 0f 84 3b 05 00 00           	je	0x140a2da84 <.text+0xa2ca84>
140a2d549: 49 89 c5                    	movq	%rax, %r13
140a2d54c: e9 cd fe ff ff              	jmp	0x140a2d41e <.text+0xa2c41e>
140a2d551: 48 c7 85 80 02 00 00 04 00 00 00    	movq	$0x4, 0x280(%rbp)
140a2d55c: 48 8b 85 78 02 00 00        	movq	0x278(%rbp), %rax
140a2d563: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
140a2d56a: 48 8b 35 97 77 c0 02        	movq	0x2c07797(%rip), %rsi   # 0x143634d08
140a2d571: 48 8b 3d 98 77 c0 02        	movq	0x2c07798(%rip), %rdi   # 0x143634d10
140a2d578: 48 8d 85 b8 00 00 00        	leaq	0xb8(%rbp), %rax
140a2d57f: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140a2d586: 31 c9                       	xorl	%ecx, %ecx
140a2d588: e8 b3 e2 ee ff              	callq	0x14091b840 <.text+0x91a840>
140a2d58d: 48 85 c0                    	testq	%rax, %rax
140a2d590: 0f 84 2f 05 00 00           	je	0x140a2dac5 <.text+0xa2cac5>
140a2d596: 48 89 b5 60 01 00 00        	movq	%rsi, 0x160(%rbp)
140a2d59d: 48 89 bd 68 01 00 00        	movq	%rdi, 0x168(%rbp)
140a2d5a4: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
140a2d5ab: 48 89 8d 70 01 00 00        	movq	%rcx, 0x170(%rbp)
140a2d5b2: 4c 89 bd 78 01 00 00        	movq	%r15, 0x178(%rbp)
140a2d5b9: 4c 8b 08                    	movq	(%rax), %r9
140a2d5bc: 49 83 c1 10                 	addq	$0x10, %r9
140a2d5c0: 4c 89 64 24 28              	movq	%r12, 0x28(%rsp)
140a2d5c5: 4c 89 4c 24 20              	movq	%r9, 0x20(%rsp)
140a2d5ca: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
140a2d5d1: 48 89 da                    	movq	%rbx, %rdx
140a2d5d4: 41 b0 01                    	movb	$0x1, %r8b
140a2d5d7: e8 f4 19 e8 ff              	callq	0x1408aefd0 <.text+0x8adfd0>
140a2d5dc: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
140a2d5e3: 48 89 c1                    	movq	%rax, %rcx
140a2d5e6: 48 f7 d9                    	negq	%rcx
140a2d5e9: 0f 80 d6 04 00 00           	jo	0x140a2dac5 <.text+0xa2cac5>
140a2d5ef: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
140a2d5f6: 48 8b 95 20 02 00 00        	movq	0x220(%rbp), %rdx
140a2d5fd: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
140a2d601: 48 8d 14 d1                 	leaq	(%rcx,%rdx,8), %rdx
140a2d605: 48 89 8d 60 01 00 00        	movq	%rcx, 0x160(%rbp)
140a2d60c: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
140a2d613: 48 89 85 70 01 00 00        	movq	%rax, 0x170(%rbp)
140a2d61a: 48 89 95 78 01 00 00        	movq	%rdx, 0x178(%rbp)
140a2d621: 48 c7 85 80 01 00 00 00 00 00 00    	movq	$0x0, 0x180(%rbp)
140a2d62c: 48 c7 85 a0 01 00 00 00 00 00 00    	movq	$0x0, 0x1a0(%rbp)
140a2d637: 8b 45 6c                    	movl	0x6c(%rbp), %eax
140a2d63a: 89 85 70 02 00 00           	movl	%eax, 0x270(%rbp)
140a2d640: 48 8b 45 50                 	movq	0x50(%rbp), %rax
140a2d644: 48 89 85 00 02 00 00        	movq	%rax, 0x200(%rbp)
140a2d64b: 48 8b 45 60                 	movq	0x60(%rbp), %rax
140a2d64f: 48 89 85 f8 01 00 00        	movq	%rax, 0x1f8(%rbp)
140a2d656: 48 8d bd b8 00 00 00        	leaq	0xb8(%rbp), %rdi
140a2d65d: 48 8d 9d 60 01 00 00        	leaq	0x160(%rbp), %rbx
140a2d664: 49 be 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r14 # imm = 0x8000000000000001
140a2d66e: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
140a2d672: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a2d680: 48 89 f9                    	movq	%rdi, %rcx
140a2d683: 48 89 da                    	movq	%rbx, %rdx
140a2d686: e8 b5 cd ff ff              	callq	0x140a2a440 <.text+0xa29440>
140a2d68b: 48 8b 85 b8 00 00 00        	movq	0xb8(%rbp), %rax
140a2d692: 4c 39 f0                    	cmpq	%r14, %rax
140a2d695: 0f 84 8d 03 00 00           	je	0x140a2da28 <.text+0xa2ca28>
140a2d69b: 48 89 c1                    	movq	%rax, %rcx
140a2d69e: 48 f7 d9                    	negq	%rcx
140a2d6a1: 70 dd                       	jo	0x140a2d680 <.text+0xa2c680>
140a2d6a3: 48 89 85 80 02 00 00        	movq	%rax, 0x280(%rbp)
140a2d6aa: 48 8d 95 c0 00 00 00        	leaq	0xc0(%rbp), %rdx
140a2d6b1: 0f 10 02                    	movups	(%rdx), %xmm0
140a2d6b4: 48 8d 85 88 02 00 00        	leaq	0x288(%rbp), %rax
140a2d6bb: 0f 11 00                    	movups	%xmm0, (%rax)
140a2d6be: 48 8b 42 40                 	movq	0x40(%rdx), %rax
140a2d6c2: 48 8d 8d 28 02 00 00        	leaq	0x228(%rbp), %rcx
140a2d6c9: 48 89 41 30                 	movq	%rax, 0x30(%rcx)
140a2d6cd: 0f 10 42 10                 	movups	0x10(%rdx), %xmm0
140a2d6d1: 0f 10 4a 20                 	movups	0x20(%rdx), %xmm1
140a2d6d5: 0f 10 52 30                 	movups	0x30(%rdx), %xmm2
140a2d6d9: 0f 11 51 20                 	movups	%xmm2, 0x20(%rcx)
140a2d6dd: 0f 11 49 10                 	movups	%xmm1, 0x10(%rcx)
140a2d6e1: 0f 11 01                    	movups	%xmm0, (%rcx)
140a2d6e4: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
140a2d6ee: 48 89 85 10 02 00 00        	movq	%rax, 0x210(%rbp)
140a2d6f5: 48 8b bd 90 02 00 00        	movq	0x290(%rbp), %rdi
140a2d6fc: 48 85 ff                    	testq	%rdi, %rdi
140a2d6ff: 0f 88 a9 03 00 00           	js	0x140a2daae <.text+0xa2caae>
140a2d705: 4c 8b a5 88 02 00 00        	movq	0x288(%rbp), %r12
140a2d70c: 74 28                       	je	0x140a2d736 <.text+0xa2c736>
140a2d70e: 0f b6 05 f4 74 c0 02        	movzbl	0x2c074f4(%rip), %eax   # 0x143634c09
140a2d715: 41 bd 01 00 00 00           	movl	$0x1, %r13d
140a2d71b: ba 01 00 00 00              	movl	$0x1, %edx
140a2d720: 48 89 f9                    	movq	%rdi, %rcx
140a2d723: e8 e8 64 be ff              	callq	0x140613c10 <.text+0x612c10>
140a2d728: 48 85 c0                    	testq	%rax, %rax
140a2d72b: 0f 84 80 03 00 00           	je	0x140a2dab1 <.text+0xa2cab1>
140a2d731: 49 89 c7                    	movq	%rax, %r15
140a2d734: eb 06                       	jmp	0x140a2d73c <.text+0xa2c73c>
140a2d736: 41 bf 01 00 00 00           	movl	$0x1, %r15d
140a2d73c: 4c 89 f9                    	movq	%r15, %rcx
140a2d73f: 4c 89 e2                    	movq	%r12, %rdx
140a2d742: 49 89 f8                    	movq	%rdi, %r8
140a2d745: e8 b6 64 ef 01              	callq	0x142923c00 <.text+0x2922c00>
140a2d74a: 48 89 bd 10 02 00 00        	movq	%rdi, 0x210(%rbp)
140a2d751: 4c 89 bd 18 02 00 00        	movq	%r15, 0x218(%rbp)
140a2d758: 48 89 bd 20 02 00 00        	movq	%rdi, 0x220(%rbp)
140a2d75f: f3 0f 6f 85 30 02 00 00     	movdqu	0x230(%rbp), %xmm0
140a2d767: 0f 10 8d 40 02 00 00        	movups	0x240(%rbp), %xmm1
140a2d76e: 0f 10 95 50 02 00 00        	movups	0x250(%rbp), %xmm2
140a2d775: 66 0f 7f 85 30 01 00 00     	movdqa	%xmm0, 0x130(%rbp)
140a2d77d: 0f 29 8d 40 01 00 00        	movaps	%xmm1, 0x140(%rbp)
140a2d784: 0f 29 95 50 01 00 00        	movaps	%xmm2, 0x150(%rbp)
140a2d78b: 48 8b 85 10 02 00 00        	movq	0x210(%rbp), %rax
140a2d792: 48 89 85 10 01 00 00        	movq	%rax, 0x110(%rbp)
140a2d799: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
140a2d7a0: 48 89 85 18 01 00 00        	movq	%rax, 0x118(%rbp)
140a2d7a7: 48 8b 85 20 02 00 00        	movq	0x220(%rbp), %rax
140a2d7ae: 48 89 85 20 01 00 00        	movq	%rax, 0x120(%rbp)
140a2d7b5: 48 8b 85 28 02 00 00        	movq	0x228(%rbp), %rax
140a2d7bc: 48 89 85 28 01 00 00        	movq	%rax, 0x128(%rbp)
140a2d7c3: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2d7ca: 48 8b 8d f0 01 00 00        	movq	0x1f0(%rbp), %rcx
140a2d7d1: 48 8d 95 10 01 00 00        	leaq	0x110(%rbp), %rdx
140a2d7d8: e8 23 ea f0 ff              	callq	0x14093c200 <.text+0x93b200>
140a2d7dd: 48 83 3d 4b 75 c0 02 00     	cmpq	$0x0, 0x2c0754b(%rip)   # 0x143634d30
140a2d7e5: 0f 84 85 01 00 00           	je	0x140a2d970 <.text+0xa2c970>
140a2d7eb: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2d7f2: 48 8d 0d 3f 75 c0 02        	leaq	0x2c0753f(%rip), %rcx   # 0x143634d38
140a2d7f9: 48 8d 95 80 02 00 00        	leaq	0x280(%rbp), %rdx
140a2d800: e8 eb ad 8b 00              	callq	0x1412e85f0 <.text+0x12e75f0>
140a2d805: 48 8b 3d 14 75 c0 02        	movq	0x2c07514(%rip), %rdi   # 0x143634d20
140a2d80c: 49 89 ff                    	movq	%rdi, %r15
140a2d80f: 49 21 c7                    	andq	%rax, %r15
140a2d812: 48 c1 e8 39                 	shrq	$0x39, %rax
140a2d816: 4c 8b 25 fb 74 c0 02        	movq	0x2c074fb(%rip), %r12   # 0x143634d18
140a2d81d: 66 0f 6e c0                 	movd	%eax, %xmm0
140a2d821: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
140a2d825: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
140a2d82a: 66 0f 70 f8 00              	pshufd	$0x0, %xmm0, %xmm7      # xmm7 = xmm0[0,0,0,0]
140a2d82f: 4d 8d 6c 24 90              	leaq	-0x70(%r12), %r13
140a2d834: 31 f6                       	xorl	%esi, %esi
140a2d836: f3 47 0f 6f 04 3c           	movdqu	(%r12,%r15), %xmm8
140a2d83c: 66 41 0f 6f c0              	movdqa	%xmm8, %xmm0
140a2d841: 66 0f 74 c7                 	pcmpeqb	%xmm7, %xmm0
140a2d845: 66 0f d7 d8                 	pmovmskb	%xmm0, %ebx
140a2d849: 85 db                       	testl	%ebx, %ebx
140a2d84b: 74 43                       	je	0x140a2d890 <.text+0xa2c890>
140a2d84d: f3 0f bc c3                 	tzcntl	%ebx, %eax
140a2d851: 4c 01 f8                    	addq	%r15, %rax
140a2d854: 48 21 f8                    	andq	%rdi, %rax
140a2d857: 48 f7 d8                    	negq	%rax
140a2d85a: 4c 6b f0 70                 	imulq	$0x70, %rax, %r14
140a2d85e: 4b 8d 14 2e                 	leaq	(%r14,%r13), %rdx
140a2d862: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2d869: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
140a2d870: e8 7b e1 59 00              	callq	0x140fcb9f0 <.text+0xfca9f0>
140a2d875: 84 c0                       	testb	%al, %al
140a2d877: 75 39                       	jne	0x140a2d8b2 <.text+0xa2c8b2>
140a2d879: 8d 43 ff                    	leal	-0x1(%rbx), %eax
140a2d87c: 66 21 d8                    	andw	%bx, %ax
140a2d87f: 89 c3                       	movl	%eax, %ebx
140a2d881: 75 ca                       	jne	0x140a2d84d <.text+0xa2c84d>
140a2d883: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a2d890: 66 44 0f 74 c6              	pcmpeqb	%xmm6, %xmm8
140a2d895: 66 41 0f d7 c0              	pmovmskb	%xmm8, %eax
140a2d89a: 85 c0                       	testl	%eax, %eax
140a2d89c: 0f 85 ce 00 00 00           	jne	0x140a2d970 <.text+0xa2c970>
140a2d8a2: 49 01 f7                    	addq	%rsi, %r15
140a2d8a5: 49 83 c7 10                 	addq	$0x10, %r15
140a2d8a9: 48 83 c6 10                 	addq	$0x10, %rsi
140a2d8ad: 49 21 ff                    	andq	%rdi, %r15
140a2d8b0: eb 84                       	jmp	0x140a2d836 <.text+0xa2c836>
140a2d8b2: 4b 8d 3c 34                 	leaq	(%r12,%r14), %rdi
140a2d8b6: 48 83 c7 a8                 	addq	$-0x58, %rdi
140a2d8ba: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2d8c1: 48 89 f9                    	movq	%rdi, %rcx
140a2d8c4: e8 07 96 34 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140a2d8c9: 49 89 d0                    	movq	%rdx, %r8
140a2d8cc: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2d8d3: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
140a2d8da: 48 89 c2                    	movq	%rax, %rdx
140a2d8dd: e8 2e ce 33 00              	callq	0x140d6a710 <.text+0xd69710>
140a2d8e2: 83 bd 10 02 00 00 02        	cmpl	$0x2, 0x210(%rbp)
140a2d8e9: 0f 85 c0 00 00 00           	jne	0x140a2d9af <.text+0xa2c9af>
140a2d8ef: 48 8b 85 18 02 00 00        	movq	0x218(%rbp), %rax
140a2d8f6: 89 c1                       	movl	%eax, %ecx
140a2d8f8: 83 e1 03                    	andl	$0x3, %ecx
140a2d8fb: 83 f9 01                    	cmpl	$0x1, %ecx
140a2d8fe: 75 70                       	jne	0x140a2d970 <.text+0xa2c970>
140a2d900: 48 8d 48 ff                 	leaq	-0x1(%rax), %rcx
140a2d904: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
140a2d90b: 48 8b 48 ff                 	movq	-0x1(%rax), %rcx
140a2d90f: 48 89 8d 78 02 00 00        	movq	%rcx, 0x278(%rbp)
140a2d916: 48 8b 40 07                 	movq	0x7(%rax), %rax
140a2d91a: 48 89 85 08 02 00 00        	movq	%rax, 0x208(%rbp)
140a2d921: 48 8b 00                    	movq	(%rax), %rax
140a2d924: 48 85 c0                    	testq	%rax, %rax
140a2d927: 74 09                       	je	0x140a2d932 <.text+0xa2c932>
140a2d929: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
140a2d930: ff d0                       	callq	*%rax
140a2d932: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
140a2d939: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140a2d940: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140a2d944: 48 85 d2                    	testq	%rdx, %rdx
140a2d947: 74 09                       	je	0x140a2d952 <.text+0xa2c952>
140a2d949: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140a2d94d: e8 ce 62 be ff              	callq	0x140613c20 <.text+0x612c20>
140a2d952: ba 18 00 00 00              	movl	$0x18, %edx
140a2d957: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140a2d95d: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
140a2d964: e8 b7 62 be ff              	callq	0x140613c20 <.text+0x612c20>
140a2d969: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140a2d970: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
140a2d977: 48 85 d2                    	testq	%rdx, %rdx
140a2d97a: 48 8d bd b8 00 00 00        	leaq	0xb8(%rbp), %rdi
140a2d981: 48 8d 9d 60 01 00 00        	leaq	0x160(%rbp), %rbx
140a2d988: 49 be 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r14 # imm = 0x8000000000000001
140a2d992: 0f 84 e8 fc ff ff           	je	0x140a2d680 <.text+0xa2c680>
140a2d998: 48 8b 8d 88 02 00 00        	movq	0x288(%rbp), %rcx
140a2d99f: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a2d9a5: e8 76 62 be ff              	callq	0x140613c20 <.text+0x612c20>
140a2d9aa: e9 d1 fc ff ff              	jmp	0x140a2d680 <.text+0xa2c680>
140a2d9af: 8b 85 70 02 00 00           	movl	0x270(%rbp), %eax
140a2d9b5: 48 8b 8d f8 01 00 00        	movq	0x1f8(%rbp), %rcx
140a2d9bc: 89 01                       	movl	%eax, (%rcx)
140a2d9be: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
140a2d9c5: 48 89 85 20 02 00 00        	movq	%rax, 0x220(%rbp)
140a2d9cc: f3 0f 6f 85 80 02 00 00     	movdqu	0x280(%rbp), %xmm0
140a2d9d4: 66 0f 7f 85 10 02 00 00     	movdqa	%xmm0, 0x210(%rbp)
140a2d9dc: c6 85 a5 02 00 00 01        	movb	$0x1, 0x2a5(%rbp)
140a2d9e3: 48 89 f9                    	movq	%rdi, %rcx
140a2d9e6: e8 e5 94 34 00              	callq	0x140d76ed0 <.text+0xd75ed0>
140a2d9eb: 48 8d bd b8 00 00 00        	leaq	0xb8(%rbp), %rdi
140a2d9f2: 48 8d 9d 60 01 00 00        	leaq	0x160(%rbp), %rbx
140a2d9f9: 49 be 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %r14 # imm = 0x8000000000000001
140a2da03: 49 89 d1                    	movq	%rdx, %r9
140a2da06: c6 85 a5 02 00 00 00        	movb	$0x0, 0x2a5(%rbp)
140a2da0d: 48 8b 8d 00 02 00 00        	movq	0x200(%rbp), %rcx
140a2da14: 48 8d 95 10 02 00 00        	leaq	0x210(%rbp), %rdx
140a2da1b: 49 89 c0                    	movq	%rax, %r8
140a2da1e: e8 4d d3 e8 ff              	callq	0x1408bad70 <.text+0x8b9d70>
140a2da23: e9 58 fc ff ff              	jmp	0x140a2d680 <.text+0xa2c680>
140a2da28: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140a2da2f: e8 ac 5a e5 ff              	callq	0x1408834e0 <.text+0x8824e0>
140a2da34: eb 0c                       	jmp	0x140a2da42 <.text+0xa2ca42>
140a2da36: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
140a2da3d: e8 be 43 e5 ff              	callq	0x140881e00 <.text+0x880e00>
140a2da42: 0f 28 b5 b0 02 00 00        	movaps	0x2b0(%rbp), %xmm6
140a2da49: 0f 28 bd c0 02 00 00        	movaps	0x2c0(%rbp), %xmm7
140a2da50: 44 0f 28 85 d0 02 00 00     	movaps	0x2d0(%rbp), %xmm8
140a2da58: 48 81 c4 68 03 00 00        	addq	$0x368, %rsp            # imm = 0x368
140a2da5f: 5b                          	popq	%rbx
140a2da60: 5f                          	popq	%rdi
140a2da61: 5e                          	popq	%rsi
140a2da62: 41 5c                       	popq	%r12
140a2da64: 41 5d                       	popq	%r13
140a2da66: 41 5e                       	popq	%r14
140a2da68: 41 5f                       	popq	%r15
140a2da6a: 5d                          	popq	%rbp
140a2da6b: c3                          	retq
140a2da6c: 48 8d 55 10                 	leaq	0x10(%rbp), %rdx
140a2da70: 48 8d 0d 89 72 c0 02        	leaq	0x2c07289(%rip), %rcx   # 0x143634d00
140a2da77: e8 dd c0 e7 01              	callq	0x1428a9b59 <.text+0x28a8b59>
140a2da7c: e9 3c f8 ff ff              	jmp	0x140a2d2bd <.text+0xa2c2bd>
140a2da81: 45 31 f6                    	xorl	%r14d, %r14d
140a2da84: 4c 8d 05 2d 15 20 02        	leaq	0x220152d(%rip), %r8    # 0x142c2efb8
140a2da8b: 4c 89 f1                    	movq	%r14, %rcx
140a2da8e: 4c 89 fa                    	movq	%r15, %rdx
140a2da91: e8 4d b5 ea 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a2da96: eb 2b                       	jmp	0x140a2dac3 <.text+0xa2cac3>
140a2da98: 31 ff                       	xorl	%edi, %edi
140a2da9a: 4c 8d 05 17 15 20 02        	leaq	0x2201517(%rip), %r8    # 0x142c2efb8
140a2daa1: 48 89 f9                    	movq	%rdi, %rcx
140a2daa4: 4c 89 ea                    	movq	%r13, %rdx
140a2daa7: e8 37 b5 ea 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a2daac: eb 15                       	jmp	0x140a2dac3 <.text+0xa2cac3>
140a2daae: 45 31 ed                    	xorl	%r13d, %r13d
140a2dab1: 4c 8d 05 00 15 20 02        	leaq	0x2201500(%rip), %r8    # 0x142c2efb8
140a2dab8: 4c 89 e9                    	movq	%r13, %rcx
140a2dabb: 48 89 fa                    	movq	%rdi, %rdx
140a2dabe: e8 20 b5 ea 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
140a2dac3: 0f 0b                       	ud2
140a2dac5: 48 8d 0d d4 cf 20 02        	leaq	0x220cfd4(%rip), %rcx   # 0x142c3aaa0
140a2dacc: e8 ff a5 ea 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a2dad1: cc                          	int3
140a2dad2: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
140a2dae0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2dae5: 55                          	pushq	%rbp
140a2dae6: 41 57                       	pushq	%r15
140a2dae8: 41 56                       	pushq	%r14
140a2daea: 41 55                       	pushq	%r13
140a2daec: 41 54                       	pushq	%r12
140a2daee: 56                          	pushq	%rsi
140a2daef: 57                          	pushq	%rdi
140a2daf0: 53                          	pushq	%rbx
140a2daf1: 48 83 ec 68                 	subq	$0x68, %rsp
140a2daf5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2dafc: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2db03: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2db09: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2db0f: 48 8d 8d 10 01 00 00        	leaq	0x110(%rbp), %rcx
140a2db16: e8 65 50 e5 ff              	callq	0x140882b80 <.text+0x881b80>
140a2db1b: c6 85 a6 02 00 00 01        	movb	$0x1, 0x2a6(%rbp)
140a2db22: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
140a2db28: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a2db2e: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a2db35: 48 83 c4 68                 	addq	$0x68, %rsp
140a2db39: 5b                          	popq	%rbx
140a2db3a: 5f                          	popq	%rdi
140a2db3b: 5e                          	popq	%rsi
140a2db3c: 41 5c                       	popq	%r12
140a2db3e: 41 5d                       	popq	%r13
140a2db40: 41 5e                       	popq	%r14
140a2db42: 41 5f                       	popq	%r15
140a2db44: 5d                          	popq	%rbp
140a2db45: c3                          	retq
140a2db46: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a2db50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2db55: 55                          	pushq	%rbp
140a2db56: 41 57                       	pushq	%r15
140a2db58: 41 56                       	pushq	%r14
140a2db5a: 41 55                       	pushq	%r13
140a2db5c: 41 54                       	pushq	%r12
140a2db5e: 56                          	pushq	%rsi
140a2db5f: 57                          	pushq	%rdi
140a2db60: 53                          	pushq	%rbx
140a2db61: 48 83 ec 68                 	subq	$0x68, %rsp
140a2db65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2db6c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2db73: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2db79: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2db7f: 48 8b 8d 78 02 00 00        	movq	0x278(%rbp), %rcx
140a2db86: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140a2db8d: 48 8b 50 08                 	movq	0x8(%rax), %rdx
140a2db91: 48 85 d2                    	testq	%rdx, %rdx
140a2db94: 74 10                       	je	0x140a2dba6 <.text+0xa2cba6>
140a2db96: 48 8b 85 08 02 00 00        	movq	0x208(%rbp), %rax
140a2db9d: 4c 8b 40 10                 	movq	0x10(%rax), %r8
140a2dba1: e8 7a 60 be ff              	callq	0x140613c20 <.text+0x612c20>
140a2dba6: ba 18 00 00 00              	movl	$0x18, %edx
140a2dbab: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140a2dbb1: 48 8b 8d 08 01 00 00        	movq	0x108(%rbp), %rcx
140a2dbb8: e8 63 60 be ff              	callq	0x140613c20 <.text+0x612c20>
140a2dbbd: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2dbc4: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a2dbc9: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a2dbce: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a2dbd4: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dbd8: 5b                          	popq	%rbx
140a2dbd9: 5f                          	popq	%rdi
140a2dbda: 5e                          	popq	%rsi
140a2dbdb: 41 5c                       	popq	%r12
140a2dbdd: 41 5d                       	popq	%r13
140a2dbdf: 41 5e                       	popq	%r14
140a2dbe1: 41 5f                       	popq	%r15
140a2dbe3: 5d                          	popq	%rbp
140a2dbe4: c3                          	retq
140a2dbe5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
140a2dbf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2dbf5: 55                          	pushq	%rbp
140a2dbf6: 41 57                       	pushq	%r15
140a2dbf8: 41 56                       	pushq	%r14
140a2dbfa: 41 55                       	pushq	%r13
140a2dbfc: 41 54                       	pushq	%r12
140a2dbfe: 56                          	pushq	%rsi
140a2dbff: 57                          	pushq	%rdi
140a2dc00: 53                          	pushq	%rbx
140a2dc01: 48 83 ec 68                 	subq	$0x68, %rsp
140a2dc05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2dc0c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2dc13: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2dc19: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2dc1f: 80 bd a5 02 00 00 00        	cmpb	$0x0, 0x2a5(%rbp)
140a2dc26: 74 46                       	je	0x140a2dc6e <.text+0xa2cc6e>
140a2dc28: 48 8b 95 10 02 00 00        	movq	0x210(%rbp), %rdx
140a2dc2f: 48 85 d2                    	testq	%rdx, %rdx
140a2dc32: 74 12                       	je	0x140a2dc46 <.text+0xa2cc46>
140a2dc34: 48 8b 8d 18 02 00 00        	movq	0x218(%rbp), %rcx
140a2dc3b: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a2dc41: e8 da 5f be ff              	callq	0x140613c20 <.text+0x612c20>
140a2dc46: c6 85 a7 02 00 00 00        	movb	$0x0, 0x2a7(%rbp)
140a2dc4d: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a2dc52: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a2dc57: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a2dc5d: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dc61: 5b                          	popq	%rbx
140a2dc62: 5f                          	popq	%rdi
140a2dc63: 5e                          	popq	%rsi
140a2dc64: 41 5c                       	popq	%r12
140a2dc66: 41 5d                       	popq	%r13
140a2dc68: 41 5e                       	popq	%r14
140a2dc6a: 41 5f                       	popq	%r15
140a2dc6c: 5d                          	popq	%rbp
140a2dc6d: c3                          	retq
140a2dc6e: c6 85 a7 02 00 00 00        	movb	$0x0, 0x2a7(%rbp)
140a2dc75: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a2dc7a: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a2dc7f: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a2dc85: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dc89: 5b                          	popq	%rbx
140a2dc8a: 5f                          	popq	%rdi
140a2dc8b: 5e                          	popq	%rsi
140a2dc8c: 41 5c                       	popq	%r12
140a2dc8e: 41 5d                       	popq	%r13
140a2dc90: 41 5e                       	popq	%r14
140a2dc92: 41 5f                       	popq	%r15
140a2dc94: 5d                          	popq	%rbp
140a2dc95: c3                          	retq
140a2dc96: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
140a2dca0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2dca5: 55                          	pushq	%rbp
140a2dca6: 41 57                       	pushq	%r15
140a2dca8: 41 56                       	pushq	%r14
140a2dcaa: 41 55                       	pushq	%r13
140a2dcac: 41 54                       	pushq	%r12
140a2dcae: 56                          	pushq	%rsi
140a2dcaf: 57                          	pushq	%rdi
140a2dcb0: 53                          	pushq	%rbx
140a2dcb1: 48 83 ec 68                 	subq	$0x68, %rsp
140a2dcb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2dcbc: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2dcc3: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2dcc9: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2dccf: 80 bd a7 02 00 00 00        	cmpb	$0x0, 0x2a7(%rbp)
140a2dcd6: 74 42                       	je	0x140a2dd1a <.text+0xa2cd1a>
140a2dcd8: 48 8b 95 80 02 00 00        	movq	0x280(%rbp), %rdx
140a2dcdf: 48 85 d2                    	testq	%rdx, %rdx
140a2dce2: 74 12                       	je	0x140a2dcf6 <.text+0xa2ccf6>
140a2dce4: 48 8b 8d 88 02 00 00        	movq	0x288(%rbp), %rcx
140a2dceb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a2dcf1: e8 2a 5f be ff              	callq	0x140613c20 <.text+0x612c20>
140a2dcf6: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
140a2dcfc: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a2dd02: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a2dd09: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dd0d: 5b                          	popq	%rbx
140a2dd0e: 5f                          	popq	%rdi
140a2dd0f: 5e                          	popq	%rsi
140a2dd10: 41 5c                       	popq	%r12
140a2dd12: 41 5d                       	popq	%r13
140a2dd14: 41 5e                       	popq	%r14
140a2dd16: 41 5f                       	popq	%r15
140a2dd18: 5d                          	popq	%rbp
140a2dd19: c3                          	retq
140a2dd1a: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
140a2dd20: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a2dd26: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a2dd2d: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dd31: 5b                          	popq	%rbx
140a2dd32: 5f                          	popq	%rdi
140a2dd33: 5e                          	popq	%rsi
140a2dd34: 41 5c                       	popq	%r12
140a2dd36: 41 5d                       	popq	%r13
140a2dd38: 41 5e                       	popq	%r14
140a2dd3a: 41 5f                       	popq	%r15
140a2dd3c: 5d                          	popq	%rbp
140a2dd3d: c3                          	retq
140a2dd3e: 66 90                       	nop
140a2dd40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2dd45: 55                          	pushq	%rbp
140a2dd46: 41 57                       	pushq	%r15
140a2dd48: 41 56                       	pushq	%r14
140a2dd4a: 41 55                       	pushq	%r13
140a2dd4c: 41 54                       	pushq	%r12
140a2dd4e: 56                          	pushq	%rsi
140a2dd4f: 57                          	pushq	%rdi
140a2dd50: 53                          	pushq	%rbx
140a2dd51: 48 83 ec 68                 	subq	$0x68, %rsp
140a2dd55: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2dd5c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2dd63: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2dd69: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2dd6f: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140a2dd76: e8 f5 ad e6 ff              	callq	0x140898b70 <.text+0x897b70>
140a2dd7b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a2dd80: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a2dd85: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a2dd8b: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dd8f: 5b                          	popq	%rbx
140a2dd90: 5f                          	popq	%rdi
140a2dd91: 5e                          	popq	%rsi
140a2dd92: 41 5c                       	popq	%r12
140a2dd94: 41 5d                       	popq	%r13
140a2dd96: 41 5e                       	popq	%r14
140a2dd98: 41 5f                       	popq	%r15
140a2dd9a: 5d                          	popq	%rbp
140a2dd9b: c3                          	retq
140a2dd9c: 0f 1f 40 00                 	nopl	(%rax)
140a2dda0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2dda5: 55                          	pushq	%rbp
140a2dda6: 41 57                       	pushq	%r15
140a2dda8: 41 56                       	pushq	%r14
140a2ddaa: 41 55                       	pushq	%r13
140a2ddac: 41 54                       	pushq	%r12
140a2ddae: 56                          	pushq	%rsi
140a2ddaf: 57                          	pushq	%rdi
140a2ddb0: 53                          	pushq	%rbx
140a2ddb1: 48 83 ec 68                 	subq	$0x68, %rsp
140a2ddb5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2ddbc: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2ddc3: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2ddc9: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2ddcf: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
140a2ddd6: 48 85 d2                    	testq	%rdx, %rdx
140a2ddd9: 74 12                       	je	0x140a2dded <.text+0xa2cded>
140a2dddb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a2dde1: 48 8b 8d 70 02 00 00        	movq	0x270(%rbp), %rcx
140a2dde8: e8 33 5e be ff              	callq	0x140613c20 <.text+0x612c20>
140a2dded: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
140a2ddf3: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a2ddf9: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a2de00: 48 83 c4 68                 	addq	$0x68, %rsp
140a2de04: 5b                          	popq	%rbx
140a2de05: 5f                          	popq	%rdi
140a2de06: 5e                          	popq	%rsi
140a2de07: 41 5c                       	popq	%r12
140a2de09: 41 5d                       	popq	%r13
140a2de0b: 41 5e                       	popq	%r14
140a2de0d: 41 5f                       	popq	%r15
140a2de0f: 5d                          	popq	%rbp
140a2de10: c3                          	retq
140a2de11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
140a2de20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2de25: 55                          	pushq	%rbp
140a2de26: 41 57                       	pushq	%r15
140a2de28: 41 56                       	pushq	%r14
140a2de2a: 41 55                       	pushq	%r13
140a2de2c: 41 54                       	pushq	%r12
140a2de2e: 56                          	pushq	%rsi
140a2de2f: 57                          	pushq	%rdi
140a2de30: 53                          	pushq	%rbx
140a2de31: 48 83 ec 68                 	subq	$0x68, %rsp
140a2de35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2de3c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2de43: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2de49: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2de4f: 80 bd a6 02 00 00 00        	cmpb	$0x0, 0x2a6(%rbp)
140a2de56: 75 21                       	jne	0x140a2de79 <.text+0xa2ce79>
140a2de58: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a2de5d: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a2de62: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a2de68: 48 83 c4 68                 	addq	$0x68, %rsp
140a2de6c: 5b                          	popq	%rbx
140a2de6d: 5f                          	popq	%rdi
140a2de6e: 5e                          	popq	%rsi
140a2de6f: 41 5c                       	popq	%r12
140a2de71: 41 5d                       	popq	%r13
140a2de73: 41 5e                       	popq	%r14
140a2de75: 41 5f                       	popq	%r15
140a2de77: 5d                          	popq	%rbp
140a2de78: c3                          	retq
140a2de79: 48 8d 8d 80 02 00 00        	leaq	0x280(%rbp), %rcx
140a2de80: e8 db c9 71 ff              	callq	0x14014a860 <.text+0x149860>
140a2de85: eb d1                       	jmp	0x140a2de58 <.text+0xa2ce58>
140a2de87: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a2de90: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2de95: 55                          	pushq	%rbp
140a2de96: 41 57                       	pushq	%r15
140a2de98: 41 56                       	pushq	%r14
140a2de9a: 41 55                       	pushq	%r13
140a2de9c: 41 54                       	pushq	%r12
140a2de9e: 56                          	pushq	%rsi
140a2de9f: 57                          	pushq	%rdi
140a2dea0: 53                          	pushq	%rbx
140a2dea1: 48 83 ec 68                 	subq	$0x68, %rsp
140a2dea5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2deac: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2deb3: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2deb9: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2debf: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140a2dec6: e8 15 56 e5 ff              	callq	0x1408834e0 <.text+0x8824e0>
140a2decb: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a2ded0: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a2ded5: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a2dedb: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dedf: 5b                          	popq	%rbx
140a2dee0: 5f                          	popq	%rdi
140a2dee1: 5e                          	popq	%rsi
140a2dee2: 41 5c                       	popq	%r12
140a2dee4: 41 5d                       	popq	%r13
140a2dee6: 41 5e                       	popq	%r14
140a2dee8: 41 5f                       	popq	%r15
140a2deea: 5d                          	popq	%rbp
140a2deeb: c3                          	retq
140a2deec: 0f 1f 40 00                 	nopl	(%rax)
140a2def0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2def5: 55                          	pushq	%rbp
140a2def6: 41 57                       	pushq	%r15
140a2def8: 41 56                       	pushq	%r14
140a2defa: 41 55                       	pushq	%r13
140a2defc: 41 54                       	pushq	%r12
140a2defe: 56                          	pushq	%rsi
140a2deff: 57                          	pushq	%rdi
140a2df00: 53                          	pushq	%rbx
140a2df01: 48 83 ec 68                 	subq	$0x68, %rsp
140a2df05: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2df0c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2df13: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2df19: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2df1f: 48 8d 8d 70 01 00 00        	leaq	0x170(%rbp), %rcx
140a2df26: e8 d5 3e e5 ff              	callq	0x140881e00 <.text+0x880e00>
140a2df2b: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a2df30: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a2df35: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a2df3b: 48 83 c4 68                 	addq	$0x68, %rsp
140a2df3f: 5b                          	popq	%rbx
140a2df40: 5f                          	popq	%rdi
140a2df41: 5e                          	popq	%rsi
140a2df42: 41 5c                       	popq	%r12
140a2df44: 41 5d                       	popq	%r13
140a2df46: 41 5e                       	popq	%r14
140a2df48: 41 5f                       	popq	%r15
140a2df4a: 5d                          	popq	%rbp
140a2df4b: c3                          	retq
140a2df4c: 0f 1f 40 00                 	nopl	(%rax)
140a2df50: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2df55: 55                          	pushq	%rbp
140a2df56: 41 57                       	pushq	%r15
140a2df58: 41 56                       	pushq	%r14
140a2df5a: 41 55                       	pushq	%r13
140a2df5c: 41 54                       	pushq	%r12
140a2df5e: 56                          	pushq	%rsi
140a2df5f: 57                          	pushq	%rdi
140a2df60: 53                          	pushq	%rbx
140a2df61: 48 83 ec 68                 	subq	$0x68, %rsp
140a2df65: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2df6c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2df73: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2df79: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2df7f: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
140a2df86: e8 e5 ab e6 ff              	callq	0x140898b70 <.text+0x897b70>
140a2df8b: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
140a2df91: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a2df97: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a2df9e: 48 83 c4 68                 	addq	$0x68, %rsp
140a2dfa2: 5b                          	popq	%rbx
140a2dfa3: 5f                          	popq	%rdi
140a2dfa4: 5e                          	popq	%rsi
140a2dfa5: 41 5c                       	popq	%r12
140a2dfa7: 41 5d                       	popq	%r13
140a2dfa9: 41 5e                       	popq	%r14
140a2dfab: 41 5f                       	popq	%r15
140a2dfad: 5d                          	popq	%rbp
140a2dfae: c3                          	retq
140a2dfaf: 90                          	nop
140a2dfb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2dfb5: 55                          	pushq	%rbp
140a2dfb6: 41 57                       	pushq	%r15
140a2dfb8: 41 56                       	pushq	%r14
140a2dfba: 41 55                       	pushq	%r13
140a2dfbc: 41 54                       	pushq	%r12
140a2dfbe: 56                          	pushq	%rsi
140a2dfbf: 57                          	pushq	%rdi
140a2dfc0: 53                          	pushq	%rbx
140a2dfc1: 48 83 ec 68                 	subq	$0x68, %rsp
140a2dfc5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2dfcc: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2dfd3: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2dfd9: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2dfdf: 48 8d 8d 60 01 00 00        	leaq	0x160(%rbp), %rcx
140a2dfe6: e8 85 ab e6 ff              	callq	0x140898b70 <.text+0x897b70>
140a2dfeb: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
140a2dff1: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a2dff7: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a2dffe: 48 83 c4 68                 	addq	$0x68, %rsp
140a2e002: 5b                          	popq	%rbx
140a2e003: 5f                          	popq	%rdi
140a2e004: 5e                          	popq	%rsi
140a2e005: 41 5c                       	popq	%r12
140a2e007: 41 5d                       	popq	%r13
140a2e009: 41 5e                       	popq	%r14
140a2e00b: 41 5f                       	popq	%r15
140a2e00d: 5d                          	popq	%rbp
140a2e00e: c3                          	retq
140a2e00f: 90                          	nop
140a2e010: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a2e015: 55                          	pushq	%rbp
140a2e016: 41 57                       	pushq	%r15
140a2e018: 41 56                       	pushq	%r14
140a2e01a: 41 55                       	pushq	%r13
140a2e01c: 41 54                       	pushq	%r12
140a2e01e: 56                          	pushq	%rsi
140a2e01f: 57                          	pushq	%rdi
140a2e020: 53                          	pushq	%rbx
140a2e021: 48 83 ec 68                 	subq	$0x68, %rsp
140a2e025: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a2e02c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a2e033: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a2e039: 66 0f 7f 74 24 50           	movdqa	%xmm6, 0x50(%rsp)
140a2e03f: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2e046: 48 8d 8d 10 02 00 00        	leaq	0x210(%rbp), %rcx
140a2e04d: e8 1e ab e6 ff              	callq	0x140898b70 <.text+0x897b70>
140a2e052: c6 85 a7 02 00 00 01        	movb	$0x1, 0x2a7(%rbp)
140a2e059: 66 0f 6f 74 24 50           	movdqa	0x50(%rsp), %xmm6
140a2e05f: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a2e065: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a2e06c: 48 83 c4 68                 	addq	$0x68, %rsp
140a2e070: 5b                          	popq	%rbx
140a2e071: 5f                          	popq	%rdi
140a2e072: 5e                          	popq	%rsi
140a2e073: 41 5c                       	popq	%r12
140a2e075: 41 5d                       	popq	%r13
140a2e077: 41 5e                       	popq	%r14
140a2e079: 41 5f                       	popq	%r15
140a2e07b: 5d                          	popq	%rbp
140a2e07c: c3                          	retq
