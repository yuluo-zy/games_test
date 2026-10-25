
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1414547d0: 55                          	pushq	%rbp
1414547d1: 41 57                       	pushq	%r15
1414547d3: 41 56                       	pushq	%r14
1414547d5: 41 55                       	pushq	%r13
1414547d7: 41 54                       	pushq	%r12
1414547d9: 56                          	pushq	%rsi
1414547da: 57                          	pushq	%rdi
1414547db: 53                          	pushq	%rbx
1414547dc: 48 81 ec 68 01 00 00        	subq	$0x168, %rsp            # imm = 0x168
1414547e3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1414547eb: 0f 29 b5 d0 00 00 00        	movaps	%xmm6, 0xd0(%rbp)
1414547f2: 48 c7 85 c8 00 00 00 fe ff ff ff    	movq	$-0x2, 0xc8(%rbp)
1414547fd: 4d 89 ce                    	movq	%r9, %r14
141454800: 4c 89 c6                    	movq	%r8, %rsi
141454803: 48 89 d3                    	movq	%rdx, %rbx
141454806: 49 89 cf                    	movq	%rcx, %r15
141454809: 48 8d 15 c8 cd 80 01        	leaq	0x180cdc8(%rip), %rdx   # 0x142c615d8
141454810: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
141454814: 41 b8 1d 00 00 00           	movl	$0x1d, %r8d
14145481a: e8 91 a6 6b ff              	callq	0x140b0eeb0 <.text+0xb0deb0>
14145481f: 31 c0                       	xorl	%eax, %eax
141454821: 48 3b 45 50                 	cmpq	0x50(%rbp), %rax
141454825: 0f 80 64 04 00 00           	jo	0x141454c8f <.text+0x1453c8f>
14145482b: 4c 8b a5 58 01 00 00        	movq	0x158(%rbp), %r12
141454832: 48 8b bd 50 01 00 00        	movq	0x150(%rbp), %rdi
141454839: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141454840: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141454844: 0f 10 85 b0 00 00 00        	movups	0xb0(%rbp), %xmm0
14145484b: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
14145484f: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
141454856: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
14145485a: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
141454861: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
141454865: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
141454869: 0f 10 4d 60                 	movups	0x60(%rbp), %xmm1
14145486d: 0f 10 55 70                 	movups	0x70(%rbp), %xmm2
141454871: 0f 10 9d 80 00 00 00        	movups	0x80(%rbp), %xmm3
141454878: 0f 29 5d 00                 	movaps	%xmm3, (%rbp)
14145487c: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
141454880: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
141454884: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
141454888: 4c 8b 0e                    	movq	(%rsi), %r9
14145488b: 48 8b 03                    	movq	(%rbx), %rax
14145488e: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
141454892: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141454897: 48 8d 15 a2 cd 80 01        	leaq	0x180cda2(%rip), %rdx   # 0x142c61640
14145489e: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
1414548a2: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1414548a8: 4c 89 4d b8                 	movq	%r9, -0x48(%rbp)
1414548ac: e8 ef 63 6c ff              	callq	0x140b1aca0 <.text+0xb19ca0>
1414548b1: 48 89 c3                    	movq	%rax, %rbx
1414548b4: 49 8b 37                    	movq	(%r15), %rsi
1414548b7: 48 8b 0e                    	movq	(%rsi), %rcx
1414548ba: 48 83 c1 10                 	addq	$0x10, %rcx
1414548be: 48 8b 07                    	movq	(%rdi), %rax
1414548c1: 4d 8b 2e                    	movq	(%r14), %r13
1414548c4: 4c 89 6c 24 30              	movq	%r13, 0x30(%rsp)
1414548c9: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
1414548cd: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1414548d2: 48 c7 44 24 20 1b 00 00 00  	movq	$0x1b, 0x20(%rsp)
1414548db: 48 8d 15 68 cd 80 01        	leaq	0x180cd68(%rip), %rdx   # 0x142c6164a
1414548e2: 4c 8d 0d 82 cd 80 01        	leaq	0x180cd82(%rip), %r9    # 0x142c6166b
1414548e9: 41 b8 21 00 00 00           	movl	$0x21, %r8d
1414548ef: e8 4c b5 6e ff              	callq	0x140b3fe40 <.text+0xb3ee40>
1414548f4: 49 89 c6                    	movq	%rax, %r14
1414548f7: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
1414548fb: 4c 89 e2                    	movq	%r12, %rdx
1414548fe: e8 9d fd 8b ff              	callq	0x140d146a0 <.text+0xd136a0>
141454903: 4c 8b 7d 58                 	movq	0x58(%rbp), %r15
141454907: 4d 85 ff                    	testq	%r15, %r15
14145490a: 75 04                       	jne	0x141454910 <.text+0x1453910>
14145490c: 4c 8b 7d 60                 	movq	0x60(%rbp), %r15
141454910: 48 8b 7d 70                 	movq	0x70(%rbp), %rdi
141454914: 49 8b 07                    	movq	(%r15), %rax
141454917: 4d 8b 67 10                 	movq	0x10(%r15), %r12
14145491b: 4c 29 e0                    	subq	%r12, %rax
14145491e: 48 83 f8 77                 	cmpq	$0x77, %rax
141454922: 0f 86 1d 03 00 00           	jbe	0x141454c45 <.text+0x1453c45>
141454928: 49 8b 47 08                 	movq	0x8(%r15), %rax
14145492c: 48 8d 0d 2d 8b e8 ff        	leaq	-0x1774d3(%rip), %rcx   # 0x1412dd460 <.text+0x12dc460>
141454933: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
141454937: 0f 28 35 82 97 4e 01        	movaps	0x14e9782(%rip), %xmm6  # 0x14293e0c0
14145493e: 42 0f 11 74 20 08           	movups	%xmm6, 0x8(%rax,%r12)
141454944: 4a c7 44 20 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r12)
14145494d: 42 c7 44 20 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r12)
141454956: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
141454960: 4a 89 4c 20 24              	movq	%rcx, 0x24(%rax,%r12)
141454965: 42 c7 44 20 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r12) # imm = 0x3F800000
14145496e: 4a 89 5c 20 38              	movq	%rbx, 0x38(%rax,%r12)
141454973: 4e 89 74 20 40              	movq	%r14, 0x40(%rax,%r12)
141454978: 42 c6 44 20 48 00           	movb	$0x0, 0x48(%rax,%r12)
14145497e: 4a 89 7c 20 58              	movq	%rdi, 0x58(%rax,%r12)
141454983: 48 8d 0d a6 03 11 00        	leaq	0x1103a6(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
14145498a: 4a 89 4c 20 68              	movq	%rcx, 0x68(%rax,%r12)
14145498f: 49 83 c4 78                 	addq	$0x78, %r12
141454993: 4d 89 67 10                 	movq	%r12, 0x10(%r15)
141454997: 48 8d 15 e8 cc 80 01        	leaq	0x180cce8(%rip), %rdx   # 0x142c61686
14145499e: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
1414549a2: 41 b8 24 00 00 00           	movl	$0x24, %r8d
1414549a8: e8 03 a5 6b ff              	callq	0x140b0eeb0 <.text+0xb0deb0>
1414549ad: 31 c0                       	xorl	%eax, %eax
1414549af: 48 3b 45 50                 	cmpq	0x50(%rbp), %rax
1414549b3: 0f 80 0b 03 00 00           	jo	0x141454cc4 <.text+0x1453cc4>
1414549b9: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
1414549c0: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
1414549c4: 0f 10 85 b0 00 00 00        	movups	0xb0(%rbp), %xmm0
1414549cb: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
1414549cf: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1414549d6: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
1414549da: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
1414549e1: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
1414549e5: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
1414549e9: 0f 10 4d 60                 	movups	0x60(%rbp), %xmm1
1414549ed: 0f 10 55 70                 	movups	0x70(%rbp), %xmm2
1414549f1: 0f 10 9d 80 00 00 00        	movups	0x80(%rbp), %xmm3
1414549f8: 0f 29 5d 00                 	movaps	%xmm3, (%rbp)
1414549fc: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
141454a00: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
141454a04: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
141454a08: 48 8b 45 c8                 	movq	-0x38(%rbp), %rax
141454a0c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141454a11: 48 8d 15 b0 cc 80 01        	leaq	0x180ccb0(%rip), %rdx   # 0x142c616c8
141454a18: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141454a1c: 41 b8 11 00 00 00           	movl	$0x11, %r8d
141454a22: 4c 8b 65 b8                 	movq	-0x48(%rbp), %r12
141454a26: 4d 89 e1                    	movq	%r12, %r9
141454a29: e8 72 62 6c ff              	callq	0x140b1aca0 <.text+0xb19ca0>
141454a2e: 48 89 c3                    	movq	%rax, %rbx
141454a31: 48 8b 0e                    	movq	(%rsi), %rcx
141454a34: 48 83 c1 10                 	addq	$0x10, %rcx
141454a38: 4c 89 6c 24 30              	movq	%r13, 0x30(%rsp)
141454a3d: 48 8b 45 c0                 	movq	-0x40(%rbp), %rax
141454a41: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
141454a46: 48 c7 44 24 20 18 00 00 00  	movq	$0x18, 0x20(%rsp)
141454a4f: 48 8d 15 83 cc 80 01        	leaq	0x180cc83(%rip), %rdx   # 0x142c616d9
141454a56: 4c 8d 0d 9d cc 80 01        	leaq	0x180cc9d(%rip), %r9    # 0x142c616fa
141454a5d: 41 b8 21 00 00 00           	movl	$0x21, %r8d
141454a63: e8 d8 b3 6e ff              	callq	0x140b3fe40 <.text+0xb3ee40>
141454a68: 49 89 c6                    	movq	%rax, %r14
141454a6b: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
141454a6f: 48 8b 95 58 01 00 00        	movq	0x158(%rbp), %rdx
141454a76: e8 25 fc 8b ff              	callq	0x140d146a0 <.text+0xd136a0>
141454a7b: 48 8b 7d 58                 	movq	0x58(%rbp), %rdi
141454a7f: 48 85 ff                    	testq	%rdi, %rdi
141454a82: 75 04                       	jne	0x141454a88 <.text+0x1453a88>
141454a84: 48 8b 7d 60                 	movq	0x60(%rbp), %rdi
141454a88: 48 8b 75 70                 	movq	0x70(%rbp), %rsi
141454a8c: 48 8b 07                    	movq	(%rdi), %rax
141454a8f: 4c 8b 7f 10                 	movq	0x10(%rdi), %r15
141454a93: 4c 29 f8                    	subq	%r15, %rax
141454a96: 48 83 f8 77                 	cmpq	$0x77, %rax
141454a9a: 0f 86 ca 01 00 00           	jbe	0x141454c6a <.text+0x1453c6a>
141454aa0: 48 8b 47 08                 	movq	0x8(%rdi), %rax
141454aa4: 48 8d 0d b5 89 e8 ff        	leaq	-0x17764b(%rip), %rcx   # 0x1412dd460 <.text+0x12dc460>
141454aab: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
141454aaf: 42 0f 11 74 38 08           	movups	%xmm6, 0x8(%rax,%r15)
141454ab5: 4a c7 44 38 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r15)
141454abe: 42 c7 44 38 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r15)
141454ac7: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
141454ad1: 4a 89 4c 38 24              	movq	%rcx, 0x24(%rax,%r15)
141454ad6: 42 c7 44 38 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r15) # imm = 0x3F800000
141454adf: 4a 89 5c 38 38              	movq	%rbx, 0x38(%rax,%r15)
141454ae4: 4e 89 74 38 40              	movq	%r14, 0x40(%rax,%r15)
141454ae9: 42 c6 44 38 48 00           	movb	$0x0, 0x48(%rax,%r15)
141454aef: 4a 89 74 38 58              	movq	%rsi, 0x58(%rax,%r15)
141454af4: 48 8d 0d 35 02 11 00        	leaq	0x110235(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
141454afb: 4a 89 4c 38 68              	movq	%rcx, 0x68(%rax,%r15)
141454b00: 49 83 c7 78                 	addq	$0x78, %r15
141454b04: 4c 89 7f 10                 	movq	%r15, 0x10(%rdi)
141454b08: 48 8d 15 03 cc 80 01        	leaq	0x180cc03(%rip), %rdx   # 0x142c61712
141454b0f: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
141454b13: 41 b8 1e 00 00 00           	movl	$0x1e, %r8d
141454b19: e8 92 a3 6b ff              	callq	0x140b0eeb0 <.text+0xb0deb0>
141454b1e: 31 c0                       	xorl	%eax, %eax
141454b20: 48 3b 45 50                 	cmpq	0x50(%rbp), %rax
141454b24: 0f 80 cc 01 00 00           	jo	0x141454cf6 <.text+0x1453cf6>
141454b2a: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141454b31: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141454b35: 0f 10 85 b0 00 00 00        	movups	0xb0(%rbp), %xmm0
141454b3c: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
141454b40: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
141454b47: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
141454b4b: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
141454b52: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
141454b56: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
141454b5a: 0f 10 4d 60                 	movups	0x60(%rbp), %xmm1
141454b5e: 0f 10 55 70                 	movups	0x70(%rbp), %xmm2
141454b62: 0f 10 9d 80 00 00 00        	movups	0x80(%rbp), %xmm3
141454b69: 0f 29 5d 00                 	movaps	%xmm3, (%rbp)
141454b6d: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
141454b71: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
141454b75: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
141454b79: 48 8b 75 c8                 	movq	-0x38(%rbp), %rsi
141454b7d: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
141454b82: 48 8d 15 bf cb 80 01        	leaq	0x180cbbf(%rip), %rdx   # 0x142c61748
141454b89: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141454b8d: 41 b8 0b 00 00 00           	movl	$0xb, %r8d
141454b93: 4d 89 e1                    	movq	%r12, %r9
141454b96: e8 05 61 6c ff              	callq	0x140b1aca0 <.text+0xb19ca0>
141454b9b: 48 8d 15 b1 cb 80 01        	leaq	0x180cbb1(%rip), %rdx   # 0x142c61753
141454ba2: 48 8d 4d 50                 	leaq	0x50(%rbp), %rcx
141454ba6: 41 b8 27 00 00 00           	movl	$0x27, %r8d
141454bac: e8 ff a2 6b ff              	callq	0x140b0eeb0 <.text+0xb0deb0>
141454bb1: 31 c0                       	xorl	%eax, %eax
141454bb3: 48 3b 45 50                 	cmpq	0x50(%rbp), %rax
141454bb7: 0f 80 6b 01 00 00           	jo	0x141454d28 <.text+0x1453d28>
141454bbd: 48 8b 85 c0 00 00 00        	movq	0xc0(%rbp), %rax
141454bc4: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
141454bc8: 0f 10 85 b0 00 00 00        	movups	0xb0(%rbp), %xmm0
141454bcf: 0f 29 45 30                 	movaps	%xmm0, 0x30(%rbp)
141454bd3: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
141454bda: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
141454bde: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
141454be5: 0f 29 45 10                 	movaps	%xmm0, 0x10(%rbp)
141454be9: 0f 10 45 50                 	movups	0x50(%rbp), %xmm0
141454bed: 0f 10 4d 60                 	movups	0x60(%rbp), %xmm1
141454bf1: 0f 10 55 70                 	movups	0x70(%rbp), %xmm2
141454bf5: 0f 10 9d 80 00 00 00        	movups	0x80(%rbp), %xmm3
141454bfc: 0f 29 5d 00                 	movaps	%xmm3, (%rbp)
141454c00: 0f 29 55 f0                 	movaps	%xmm2, -0x10(%rbp)
141454c04: 0f 29 4d e0                 	movaps	%xmm1, -0x20(%rbp)
141454c08: 0f 29 45 d0                 	movaps	%xmm0, -0x30(%rbp)
141454c0c: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
141454c11: 48 8d 15 80 cb 80 01        	leaq	0x180cb80(%rip), %rdx   # 0x142c61798
141454c18: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141454c1c: 41 b8 14 00 00 00           	movl	$0x14, %r8d
141454c22: 4d 89 e1                    	movq	%r12, %r9
141454c25: e8 76 60 6c ff              	callq	0x140b1aca0 <.text+0xb19ca0>
141454c2a: 0f 28 b5 d0 00 00 00        	movaps	0xd0(%rbp), %xmm6
141454c31: 48 81 c4 68 01 00 00        	addq	$0x168, %rsp            # imm = 0x168
141454c38: 5b                          	popq	%rbx
141454c39: 5f                          	popq	%rdi
141454c3a: 5e                          	popq	%rsi
141454c3b: 41 5c                       	popq	%r12
141454c3d: 41 5d                       	popq	%r13
141454c3f: 41 5e                       	popq	%r14
141454c41: 41 5f                       	popq	%r15
141454c43: 5d                          	popq	%rbp
141454c44: c3                          	retq
141454c45: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141454c4e: 41 b8 78 00 00 00           	movl	$0x78, %r8d
141454c54: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141454c5a: 4c 89 f9                    	movq	%r15, %rcx
141454c5d: 4c 89 e2                    	movq	%r12, %rdx
141454c60: e8 fb 4d 45 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141454c65: e9 be fc ff ff              	jmp	0x141454928 <.text+0x1453928>
141454c6a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
141454c73: 41 b8 78 00 00 00           	movl	$0x78, %r8d
141454c79: 41 b9 01 00 00 00           	movl	$0x1, %r9d
141454c7f: 48 89 f9                    	movq	%rdi, %rcx
141454c82: 4c 89 fa                    	movq	%r15, %rdx
141454c85: e8 d6 4d 45 01              	callq	0x1428a9a60 <.text+0x28a8a60>
141454c8a: e9 11 fe ff ff              	jmp	0x141454aa0 <.text+0x1453aa0>
141454c8f: 48 8b 45 58                 	movq	0x58(%rbp), %rax
141454c93: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
141454c97: 48 8d 05 8a c9 80 01        	leaq	0x180c98a(%rip), %rax   # 0x142c61628
141454c9e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141454ca3: 48 8d 0d 76 c7 80 01        	leaq	0x180c776(%rip), %rcx   # 0x142c61420
141454caa: 4c 8d 0d 4f c7 80 01        	leaq	0x180c74f(%rip), %r9    # 0x142c61400
141454cb1: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
141454cb5: ba 2b 00 00 00              	movl	$0x2b, %edx
141454cba: e8 a1 4a 48 01              	callq	0x1428d9760 <.text+0x28d8760>
141454cbf: e9 94 00 00 00              	jmp	0x141454d58 <.text+0x1453d58>
141454cc4: 48 8b 45 58                 	movq	0x58(%rbp), %rax
141454cc8: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
141454ccc: 48 8d 05 dd c9 80 01        	leaq	0x180c9dd(%rip), %rax   # 0x142c616b0
141454cd3: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141454cd8: 48 8d 0d 41 c7 80 01        	leaq	0x180c741(%rip), %rcx   # 0x142c61420
141454cdf: 4c 8d 0d 1a c7 80 01        	leaq	0x180c71a(%rip), %r9    # 0x142c61400
141454ce6: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
141454cea: ba 2b 00 00 00              	movl	$0x2b, %edx
141454cef: e8 6c 4a 48 01              	callq	0x1428d9760 <.text+0x28d8760>
141454cf4: eb 62                       	jmp	0x141454d58 <.text+0x1453d58>
141454cf6: 48 8b 45 58                 	movq	0x58(%rbp), %rax
141454cfa: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
141454cfe: 48 8d 05 2b ca 80 01        	leaq	0x180ca2b(%rip), %rax   # 0x142c61730
141454d05: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141454d0a: 48 8d 0d 0f c7 80 01        	leaq	0x180c70f(%rip), %rcx   # 0x142c61420
141454d11: 4c 8d 0d e8 c6 80 01        	leaq	0x180c6e8(%rip), %r9    # 0x142c61400
141454d18: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
141454d1c: ba 2b 00 00 00              	movl	$0x2b, %edx
141454d21: e8 3a 4a 48 01              	callq	0x1428d9760 <.text+0x28d8760>
141454d26: eb 30                       	jmp	0x141454d58 <.text+0x1453d58>
141454d28: 48 8b 45 58                 	movq	0x58(%rbp), %rax
141454d2c: 48 89 45 d0                 	movq	%rax, -0x30(%rbp)
141454d30: 48 8d 05 49 ca 80 01        	leaq	0x180ca49(%rip), %rax   # 0x142c61780
141454d37: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
141454d3c: 48 8d 0d dd c6 80 01        	leaq	0x180c6dd(%rip), %rcx   # 0x142c61420
141454d43: 4c 8d 0d b6 c6 80 01        	leaq	0x180c6b6(%rip), %r9    # 0x142c61400
141454d4a: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
141454d4e: ba 2b 00 00 00              	movl	$0x2b, %edx
141454d53: e8 08 4a 48 01              	callq	0x1428d9760 <.text+0x28d8760>
141454d58: 0f 0b                       	ud2
141454d5a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141454d60: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141454d65: 55                          	pushq	%rbp
141454d66: 41 57                       	pushq	%r15
141454d68: 41 56                       	pushq	%r14
141454d6a: 41 55                       	pushq	%r13
141454d6c: 41 54                       	pushq	%r12
141454d6e: 56                          	pushq	%rsi
141454d6f: 57                          	pushq	%rdi
141454d70: 53                          	pushq	%rbx
141454d71: 48 83 ec 48                 	subq	$0x48, %rsp
141454d75: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141454d7c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
141454d81: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141454d85: e8 a6 7e 8f ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141454d8a: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
141454d8f: 48 83 c4 48                 	addq	$0x48, %rsp
141454d93: 5b                          	popq	%rbx
141454d94: 5f                          	popq	%rdi
141454d95: 5e                          	popq	%rsi
141454d96: 41 5c                       	popq	%r12
141454d98: 41 5d                       	popq	%r13
141454d9a: 41 5e                       	popq	%r14
141454d9c: 41 5f                       	popq	%r15
141454d9e: 5d                          	popq	%rbp
141454d9f: c3                          	retq
141454da0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141454da5: 55                          	pushq	%rbp
141454da6: 41 57                       	pushq	%r15
141454da8: 41 56                       	pushq	%r14
141454daa: 41 55                       	pushq	%r13
141454dac: 41 54                       	pushq	%r12
141454dae: 56                          	pushq	%rsi
141454daf: 57                          	pushq	%rdi
141454db0: 53                          	pushq	%rbx
141454db1: 48 83 ec 48                 	subq	$0x48, %rsp
141454db5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141454dbc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
141454dc1: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141454dc5: e8 66 7e 8f ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141454dca: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
141454dcf: 48 83 c4 48                 	addq	$0x48, %rsp
141454dd3: 5b                          	popq	%rbx
141454dd4: 5f                          	popq	%rdi
141454dd5: 5e                          	popq	%rsi
141454dd6: 41 5c                       	popq	%r12
141454dd8: 41 5d                       	popq	%r13
141454dda: 41 5e                       	popq	%r14
141454ddc: 41 5f                       	popq	%r15
141454dde: 5d                          	popq	%rbp
141454ddf: c3                          	retq
141454de0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141454de5: 55                          	pushq	%rbp
141454de6: 41 57                       	pushq	%r15
141454de8: 41 56                       	pushq	%r14
141454dea: 41 55                       	pushq	%r13
141454dec: 41 54                       	pushq	%r12
141454dee: 56                          	pushq	%rsi
141454def: 57                          	pushq	%rdi
141454df0: 53                          	pushq	%rbx
141454df1: 48 83 ec 48                 	subq	$0x48, %rsp
141454df5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141454dfc: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
141454e01: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141454e05: e8 26 7e 8f ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141454e0a: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
141454e0f: 48 83 c4 48                 	addq	$0x48, %rsp
141454e13: 5b                          	popq	%rbx
141454e14: 5f                          	popq	%rdi
141454e15: 5e                          	popq	%rsi
141454e16: 41 5c                       	popq	%r12
141454e18: 41 5d                       	popq	%r13
141454e1a: 41 5e                       	popq	%r14
141454e1c: 41 5f                       	popq	%r15
141454e1e: 5d                          	popq	%rbp
141454e1f: c3                          	retq
141454e20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
141454e25: 55                          	pushq	%rbp
141454e26: 41 57                       	pushq	%r15
141454e28: 41 56                       	pushq	%r14
141454e2a: 41 55                       	pushq	%r13
141454e2c: 41 54                       	pushq	%r12
141454e2e: 56                          	pushq	%rsi
141454e2f: 57                          	pushq	%rdi
141454e30: 53                          	pushq	%rbx
141454e31: 48 83 ec 48                 	subq	$0x48, %rsp
141454e35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
141454e3c: 0f 29 74 24 30              	movaps	%xmm6, 0x30(%rsp)
141454e41: 48 8d 4d d0                 	leaq	-0x30(%rbp), %rcx
141454e45: e8 e6 7d 8f ff              	callq	0x140d4cc30 <.text+0xd4bc30>
141454e4a: 0f 28 74 24 30              	movaps	0x30(%rsp), %xmm6
141454e4f: 48 83 c4 48                 	addq	$0x48, %rsp
141454e53: 5b                          	popq	%rbx
141454e54: 5f                          	popq	%rdi
141454e55: 5e                          	popq	%rsi
141454e56: 41 5c                       	popq	%r12
141454e58: 41 5d                       	popq	%r13
141454e5a: 41 5e                       	popq	%r14
141454e5c: 41 5f                       	popq	%r15
141454e5e: 5d                          	popq	%rbp
141454e5f: c3                          	retq
