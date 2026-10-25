
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1409d49a0: 55                          	pushq	%rbp
1409d49a1: 41 57                       	pushq	%r15
1409d49a3: 41 56                       	pushq	%r14
1409d49a5: 41 55                       	pushq	%r13
1409d49a7: 41 54                       	pushq	%r12
1409d49a9: 56                          	pushq	%rsi
1409d49aa: 57                          	pushq	%rdi
1409d49ab: 53                          	pushq	%rbx
1409d49ac: 48 81 ec a8 02 00 00        	subq	$0x2a8, %rsp            # imm = 0x2A8
1409d49b3: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
1409d49bb: 44 0f 29 bd 10 02 00 00     	movaps	%xmm15, 0x210(%rbp)
1409d49c3: 44 0f 29 b5 00 02 00 00     	movaps	%xmm14, 0x200(%rbp)
1409d49cb: 44 0f 29 ad f0 01 00 00     	movaps	%xmm13, 0x1f0(%rbp)
1409d49d3: 44 0f 29 a5 e0 01 00 00     	movaps	%xmm12, 0x1e0(%rbp)
1409d49db: 44 0f 29 9d d0 01 00 00     	movaps	%xmm11, 0x1d0(%rbp)
1409d49e3: 44 0f 29 95 c0 01 00 00     	movaps	%xmm10, 0x1c0(%rbp)
1409d49eb: 44 0f 29 8d b0 01 00 00     	movaps	%xmm9, 0x1b0(%rbp)
1409d49f3: 44 0f 29 85 a0 01 00 00     	movaps	%xmm8, 0x1a0(%rbp)
1409d49fb: 0f 29 bd 90 01 00 00        	movaps	%xmm7, 0x190(%rbp)
1409d4a02: 0f 29 b5 80 01 00 00        	movaps	%xmm6, 0x180(%rbp)
1409d4a09: 48 c7 85 78 01 00 00 fe ff ff ff    	movq	$-0x2, 0x178(%rbp)
1409d4a14: 4c 89 4d 28                 	movq	%r9, 0x28(%rbp)
1409d4a18: 4c 89 45 20                 	movq	%r8, 0x20(%rbp)
1409d4a1c: 49 89 d7                    	movq	%rdx, %r15
1409d4a1f: 48 8b 85 a8 02 00 00        	movq	0x2a8(%rbp), %rax
1409d4a26: 48 8b 09                    	movq	(%rcx), %rcx
1409d4a29: 4c 8b 29                    	movq	(%rcx), %r13
1409d4a2c: 49 83 c5 10                 	addq	$0x10, %r13
1409d4a30: 48 8b 00                    	movq	(%rax), %rax
1409d4a33: 80 78 1f 01                 	cmpb	$0x1, 0x1f(%rax)
1409d4a37: 4c 89 6d 30                 	movq	%r13, 0x30(%rbp)
1409d4a3b: 48 89 55 38                 	movq	%rdx, 0x38(%rbp)
1409d4a3f: 0f 85 3d 04 00 00           	jne	0x1409d4e82 <.text+0x9d3e82>
1409d4a45: 48 8b 85 98 02 00 00        	movq	0x298(%rbp), %rax
1409d4a4c: 48 c7 45 b8 29 a7 02 00     	movq	$0x2a729, -0x48(%rbp)   # imm = 0x2A729
1409d4a54: 48 8b 30                    	movq	(%rax), %rsi
1409d4a57: 4c 8d bd 0c 01 00 00        	leaq	0x10c(%rbp), %r15
1409d4a5e: 4c 8d b5 9c 00 00 00        	leaq	0x9c(%rbp), %r14
1409d4a65: bf 18 00 00 00              	movl	$0x18, %edi
1409d4a6a: 4c 8d 65 b8                 	leaq	-0x48(%rbp), %r12
1409d4a6e: f3 44 0f 10 05 71 56 f8 01  	movss	0x1f85671(%rip), %xmm8  # 0x14295a0e8
1409d4a77: f3 44 0f 10 0d 08 bd fc 01  	movss	0x1fcbd08(%rip), %xmm9  # 0x1429a0788
1409d4a80: 4c 8d ad f0 00 00 00        	leaq	0xf0(%rbp), %r13
1409d4a87: f3 44 0f 10 15 a4 85 ff 01  	movss	0x1ff85a4(%rip), %xmm10 # 0x1429cd034
1409d4a90: f3 44 0f 10 1d 6b 0e f5 01  	movss	0x1f50e6b(%rip), %xmm11 # 0x142925904
1409d4a99: 45 0f 57 e4                 	xorps	%xmm12, %xmm12
1409d4a9d: 48 8d 5d 40                 	leaq	0x40(%rbp), %rbx
1409d4aa1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409d4ab0: 48 8b 16                    	movq	(%rsi), %rdx
1409d4ab3: 48 85 d2                    	testq	%rdx, %rdx
1409d4ab6: 74 28                       	je	0x1409d4ae0 <.text+0x9d3ae0>
1409d4ab8: 48 83 c2 10                 	addq	$0x10, %rdx
1409d4abc: 41 b9 1b 00 00 00           	movl	$0x1b, %r9d
1409d4ac2: 48 89 d9                    	movq	%rbx, %rcx
1409d4ac5: 4c 8d 05 1d d8 0e 02        	leaq	0x20ed81d(%rip), %r8    # 0x142ac22e9
1409d4acc: e8 6f f9 07 00              	callq	0x140a54440 <.text+0xa53440>
1409d4ad1: 48 83 7d 40 02              	cmpq	$0x2, 0x40(%rbp)
1409d4ad6: 75 5c                       	jne	0x1409d4b34 <.text+0x9d3b34>
1409d4ad8: e9 13 0c 00 00              	jmp	0x1409d56f0 <.text+0x9d46f0>
1409d4add: 0f 1f 00                    	nopl	(%rax)
1409d4ae0: 0f b6 05 22 01 c6 02        	movzbl	0x2c60122(%rip), %eax   # 0x143634c09
1409d4ae7: b9 1b 00 00 00              	movl	$0x1b, %ecx
1409d4aec: ba 01 00 00 00              	movl	$0x1, %edx
1409d4af1: e8 1a f1 c3 ff              	callq	0x140613c10 <.text+0x612c10>
1409d4af6: 48 85 c0                    	testq	%rax, %rax
1409d4af9: 0f 84 2c 0c 00 00           	je	0x1409d572b <.text+0x9d472b>
1409d4aff: 0f 10 05 ee d7 0e 02        	movups	0x20ed7ee(%rip), %xmm0  # 0x142ac22f4
1409d4b06: 0f 11 40 0b                 	movups	%xmm0, 0xb(%rax)
1409d4b0a: 0f 10 05 d8 d7 0e 02        	movups	0x20ed7d8(%rip), %xmm0  # 0x142ac22e9
1409d4b11: 0f 11 00                    	movups	%xmm0, (%rax)
1409d4b14: 48 c7 45 40 00 00 00 00     	movq	$0x0, 0x40(%rbp)
1409d4b1c: 48 c7 45 50 1b 00 00 00     	movq	$0x1b, 0x50(%rbp)
1409d4b24: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
1409d4b28: 48 c7 45 60 1b 00 00 00     	movq	$0x1b, 0x60(%rbp)
1409d4b30: c6 45 68 02                 	movb	$0x2, 0x68(%rbp)
1409d4b34: 48 8b 45 40                 	movq	0x40(%rbp), %rax
1409d4b38: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
1409d4b3c: 48 8b 55 50                 	movq	0x50(%rbp), %rdx
1409d4b40: 4c 8b 45 58                 	movq	0x58(%rbp), %r8
1409d4b44: 4c 8b 4d 60                 	movq	0x60(%rbp), %r9
1409d4b48: 4c 89 4d 00                 	movq	%r9, (%rbp)
1409d4b4c: 44 0f b6 4d 68              	movzbl	0x68(%rbp), %r9d
1409d4b51: 44 88 4d 08                 	movb	%r9b, 0x8(%rbp)
1409d4b55: 44 8b 4d 69                 	movl	0x69(%rbp), %r9d
1409d4b59: 44 89 4d 09                 	movl	%r9d, 0x9(%rbp)
1409d4b5d: 44 0f b7 4d 6d              	movzwl	0x6d(%rbp), %r9d
1409d4b62: 66 44 89 4d 0d              	movw	%r9w, 0xd(%rbp)
1409d4b67: 44 0f b6 4d 6f              	movzbl	0x6f(%rbp), %r9d
1409d4b6c: 44 88 4d 0f                 	movb	%r9b, 0xf(%rbp)
1409d4b70: 48 89 55 f0                 	movq	%rdx, -0x10(%rbp)
1409d4b74: 4c 89 45 f8                 	movq	%r8, -0x8(%rbp)
1409d4b78: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1409d4b7c: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
1409d4b80: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
1409d4b87: 4c 89 e1                    	movq	%r12, %rcx
1409d4b8a: e8 f1 a7 2d 00              	callq	0x140caf380 <.text+0xcae380>
1409d4b8f: 0f 28 f0                    	movaps	%xmm0, %xmm6
1409d4b92: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
1409d4b99: 4c 89 e1                    	movq	%r12, %rcx
1409d4b9c: e8 df a7 2d 00              	callq	0x140caf380 <.text+0xcae380>
1409d4ba1: 0f 28 f8                    	movaps	%xmm0, %xmm7
1409d4ba4: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
1409d4bab: 4c 89 e9                    	movq	%r13, %rcx
1409d4bae: e8 cd 85 2c 00              	callq	0x140c9d180 <.text+0xc9c180>
1409d4bb3: 44 0f 28 ad f0 00 00 00     	movaps	0xf0(%rbp), %xmm13
1409d4bbb: 49 8b 07                    	movq	(%r15), %rax
1409d4bbe: 48 89 45 10                 	movq	%rax, 0x10(%rbp)
1409d4bc2: 41 8b 47 08                 	movl	0x8(%r15), %eax
1409d4bc6: 89 45 18                    	movl	%eax, 0x18(%rbp)
1409d4bc9: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
1409d4bd0: 4c 89 e1                    	movq	%r12, %rcx
1409d4bd3: e8 a8 a7 2d 00              	callq	0x140caf380 <.text+0xcae380>
1409d4bd8: f3 41 0f 58 f0              	addss	%xmm8, %xmm6
1409d4bdd: f3 41 0f 59 f1              	mulss	%xmm9, %xmm6
1409d4be2: f3 41 0f 58 f8              	addss	%xmm8, %xmm7
1409d4be7: f3 41 0f 59 f9              	mulss	%xmm9, %xmm7
1409d4bec: f3 41 0f 59 c2              	mulss	%xmm10, %xmm0
1409d4bf1: f3 41 0f 58 c3              	addss	%xmm11, %xmm0
1409d4bf6: 0f 28 4d e0                 	movaps	-0x20(%rbp), %xmm1
1409d4bfa: 0f 28 55 f0                 	movaps	-0x10(%rbp), %xmm2
1409d4bfe: 0f 28 5d 00                 	movaps	(%rbp), %xmm3
1409d4c02: 0f 29 5d 60                 	movaps	%xmm3, 0x60(%rbp)
1409d4c06: 0f 29 55 50                 	movaps	%xmm2, 0x50(%rbp)
1409d4c0a: 0f 29 4d 40                 	movaps	%xmm1, 0x40(%rbp)
1409d4c0e: 44 0f 29 ad 80 00 00 00     	movaps	%xmm13, 0x80(%rbp)
1409d4c16: f3 0f 11 b5 90 00 00 00     	movss	%xmm6, 0x90(%rbp)
1409d4c1e: c7 85 94 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0x94(%rbp) # imm = 0x3F800000
1409d4c28: f3 0f 11 bd 98 00 00 00     	movss	%xmm7, 0x98(%rbp)
1409d4c30: 8b 45 18                    	movl	0x18(%rbp), %eax
1409d4c33: 41 89 46 08                 	movl	%eax, 0x8(%r14)
1409d4c37: 48 8b 45 10                 	movq	0x10(%rbp), %rax
1409d4c3b: 49 89 06                    	movq	%rax, (%r14)
1409d4c3e: c7 85 c0 00 00 00 02 00 00 00       	movl	$0x2, 0xc0(%rbp)
1409d4c48: f3 0f 11 85 c4 00 00 00     	movss	%xmm0, 0xc4(%rbp)
1409d4c50: 45 0f 11 66 2c              	movups	%xmm12, 0x2c(%r14)
1409d4c55: c7 85 d8 00 00 00 03 00 00 00       	movl	$0x3, 0xd8(%rbp)
1409d4c5f: c7 45 70 00 00 00 00        	movl	$0x0, 0x70(%rbp)
1409d4c66: 41 c7 46 1c 00 00 00 00     	movl	$0x0, 0x1c(%r14)
1409d4c6e: 49 c7 46 14 00 00 00 00     	movq	$0x0, 0x14(%r14)
1409d4c76: c7 85 bc 00 00 00 cd cc 4c 3d       	movl	$0x3d4ccccd, 0xbc(%rbp) # imm = 0x3D4CCCCD
1409d4c80: 41 c7 46 4c 00 00 00 00     	movl	$0x0, 0x4c(%r14)
1409d4c88: 49 c7 46 44 00 00 00 00     	movq	$0x0, 0x44(%r14)
1409d4c90: c7 85 ec 00 00 00 00 00 80 3f       	movl	$0x3f800000, 0xec(%rbp) # imm = 0x3F800000
1409d4c9a: c6 85 77 01 00 00 00        	movb	$0x0, 0x177(%rbp)
1409d4ca1: 4c 89 e9                    	movq	%r13, %rcx
1409d4ca4: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
1409d4cab: 49 89 d8                    	movq	%rbx, %r8
1409d4cae: 4c 8d 0d 6b d6 0e 02        	leaq	0x20ed66b(%rip), %r9    # 0x142ac2320
1409d4cb5: e8 66 d2 90 00              	callq	0x1412e1f20 <.text+0x12e0f20>
1409d4cba: 48 ff cf                    	decq	%rdi
1409d4cbd: 0f 85 ed fd ff ff           	jne	0x1409d4ab0 <.text+0x9d3ab0>
1409d4cc3: 48 8d 15 db d3 0e 02        	leaq	0x20ed3db(%rip), %rdx   # 0x142ac20a5
1409d4cca: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d4cce: 41 b8 1c 00 00 00           	movl	$0x1c, %r8d
1409d4cd4: e8 d7 a1 13 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
1409d4cd9: 31 c0                       	xorl	%eax, %eax
1409d4cdb: 48 3b 45 40                 	cmpq	0x40(%rbp), %rax
1409d4cdf: 0f 80 4d 0b 00 00           	jo	0x1409d5832 <.text+0x9d4832>
1409d4ce5: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1409d4cec: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1409d4cf3: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1409d4cfa: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1409d4d01: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
1409d4d08: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1409d4d0f: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1409d4d16: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1409d4d1d: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409d4d21: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409d4d25: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409d4d29: 0f 10 5d 70                 	movups	0x70(%rbp), %xmm3
1409d4d2d: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1409d4d34: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1409d4d3b: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1409d4d42: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409d4d49: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
1409d4d50: 4c 8b 08                    	movq	(%rax), %r9
1409d4d53: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1409d4d57: 48 8b 00                    	movq	(%rax), %rax
1409d4d5a: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d4d5f: 48 8d 15 aa d3 0e 02        	leaq	0x20ed3aa(%rip), %rdx   # 0x142ac2110
1409d4d66: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d4d6d: 41 b8 09 00 00 00           	movl	$0x9, %r8d
1409d4d73: e8 28 5f 14 00              	callq	0x140b1aca0 <.text+0xb19ca0>
1409d4d78: 48 89 c7                    	movq	%rax, %rdi
1409d4d7b: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1409d4d7f: 48 8b 00                    	movq	(%rax), %rax
1409d4d82: 4c 8b 7d 38                 	movq	0x38(%rbp), %r15
1409d4d86: 49 8b 0f                    	movq	(%r15), %rcx
1409d4d89: 48 89 4c 24 30              	movq	%rcx, 0x30(%rsp)
1409d4d8e: 48 89 44 24 28              	movq	%rax, 0x28(%rsp)
1409d4d93: 48 c7 44 24 20 10 00 00 00  	movq	$0x10, 0x20(%rsp)
1409d4d9c: 48 8d 15 76 d3 0e 02        	leaq	0x20ed376(%rip), %rdx   # 0x142ac2119
1409d4da3: 4c 8d 0d 8e d3 0e 02        	leaq	0x20ed38e(%rip), %r9    # 0x142ac2138
1409d4daa: 41 b8 1f 00 00 00           	movl	$0x1f, %r8d
1409d4db0: 4c 8b 6d 30                 	movq	0x30(%rbp), %r13
1409d4db4: 4c 89 e9                    	movq	%r13, %rcx
1409d4db7: e8 84 b0 16 00              	callq	0x140b3fe40 <.text+0xb3ee40>
1409d4dbc: 48 89 c3                    	movq	%rax, %rbx
1409d4dbf: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d4dc3: e8 b8 83 2c 00              	callq	0x140c9d180 <.text+0xc9c180>
1409d4dc8: 8b 45 58                    	movl	0x58(%rbp), %eax
1409d4dcb: 89 85 f8 00 00 00           	movl	%eax, 0xf8(%rbp)
1409d4dd1: 48 8b 45 50                 	movq	0x50(%rbp), %rax
1409d4dd5: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409d4ddc: 0f 28 75 40                 	movaps	0x40(%rbp), %xmm6
1409d4de0: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d4de4: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
1409d4deb: e8 b0 f8 33 00              	callq	0x140d146a0 <.text+0xd136a0>
1409d4df0: 4c 8b 75 48                 	movq	0x48(%rbp), %r14
1409d4df4: 4d 85 f6                    	testq	%r14, %r14
1409d4df7: 75 04                       	jne	0x1409d4dfd <.text+0x9d3dfd>
1409d4df9: 4c 8b 75 50                 	movq	0x50(%rbp), %r14
1409d4dfd: 48 8b 75 60                 	movq	0x60(%rbp), %rsi
1409d4e01: 49 8b 06                    	movq	(%r14), %rax
1409d4e04: 4d 8b 66 10                 	movq	0x10(%r14), %r12
1409d4e08: 4c 29 e0                    	subq	%r12, %rax
1409d4e0b: 48 83 f8 77                 	cmpq	$0x77, %rax
1409d4e0f: 0f 86 b6 08 00 00           	jbe	0x1409d56cb <.text+0x9d46cb>
1409d4e15: 49 8b 46 08                 	movq	0x8(%r14), %rax
1409d4e19: 48 8d 0d 40 86 90 00        	leaq	0x908640(%rip), %rcx    # 0x1412dd460 <.text+0x12dc460>
1409d4e20: 4a 89 0c 20                 	movq	%rcx, (%rax,%r12)
1409d4e24: 42 0f 11 74 20 08           	movups	%xmm6, 0x8(%rax,%r12)
1409d4e2a: 48 8b 8d f0 00 00 00        	movq	0xf0(%rbp), %rcx
1409d4e31: 4a 89 4c 20 18              	movq	%rcx, 0x18(%rax,%r12)
1409d4e36: 8b 8d f8 00 00 00           	movl	0xf8(%rbp), %ecx
1409d4e3c: 42 89 4c 20 20              	movl	%ecx, 0x20(%rax,%r12)
1409d4e41: f2 0f 10 05 d7 77 0b 02     	movsd	0x20b77d7(%rip), %xmm0  # 0x142a8c620
1409d4e49: f2 42 0f 11 44 20 24        	movsd	%xmm0, 0x24(%rax,%r12)
1409d4e50: 42 c7 44 20 2c 9a 99 19 3f  	movl	$0x3f19999a, 0x2c(%rax,%r12) # imm = 0x3F19999A
1409d4e59: 4a 89 7c 20 38              	movq	%rdi, 0x38(%rax,%r12)
1409d4e5e: 4a 89 5c 20 40              	movq	%rbx, 0x40(%rax,%r12)
1409d4e63: 42 c6 44 20 48 00           	movb	$0x0, 0x48(%rax,%r12)
1409d4e69: 4a 89 74 20 58              	movq	%rsi, 0x58(%rax,%r12)
1409d4e6e: 48 8d 0d bb fe b8 00        	leaq	0xb8febb(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409d4e75: 4a 89 4c 20 68              	movq	%rcx, 0x68(%rax,%r12)
1409d4e7a: 49 83 c4 78                 	addq	$0x78, %r12
1409d4e7e: 4d 89 66 10                 	movq	%r12, 0x10(%r14)
1409d4e82: 48 8b 85 b0 02 00 00        	movq	0x2b0(%rbp), %rax
1409d4e89: 48 8b 00                    	movq	(%rax), %rax
1409d4e8c: 80 78 58 04                 	cmpb	$0x4, 0x58(%rax)
1409d4e90: 0f 84 1b 05 00 00           	je	0x1409d53b1 <.text+0x9d43b1>
1409d4e96: 48 c7 45 10 9a 02 00 00     	movq	$0x29a, 0x10(%rbp)      # imm = 0x29A
1409d4e9e: 48 8d b5 f8 00 00 00        	leaq	0xf8(%rbp), %rsi
1409d4ea5: 31 ff                       	xorl	%edi, %edi
1409d4ea7: 48 8d 5d 10                 	leaq	0x10(%rbp), %rbx
1409d4eab: f3 0f 10 35 51 0a f5 01     	movss	0x1f50a51(%rip), %xmm6  # 0x142925904
1409d4eb3: f3 44 0f 10 1d 6c ca 0e 02  	movss	0x20eca6c(%rip), %xmm11 # 0x142ac1928
1409d4ebc: f3 44 0f 10 25 67 ca 0e 02  	movss	0x20eca67(%rip), %xmm12 # 0x142ac192c
1409d4ec5: f3 44 0f 10 2d 62 ca 0e 02  	movss	0x20eca62(%rip), %xmm13 # 0x142ac1930
1409d4ece: f3 44 0f 10 35 3d e9 0a 02  	movss	0x20ae93d(%rip), %xmm14 # 0x142a83814
1409d4ed7: 4c 8d 75 40                 	leaq	0x40(%rbp), %r14
1409d4edb: 45 0f 57 ff                 	xorps	%xmm15, %xmm15
1409d4edf: 90                          	nop
1409d4ee0: 89 f9                       	movl	%edi, %ecx
1409d4ee2: ba 02 00 00 00              	movl	$0x2, %edx
1409d4ee7: e8 b4 ba 2b 00              	callq	0x140c909a0 <.text+0xc8f9a0>
1409d4eec: 0f 28 f8                    	movaps	%xmm0, %xmm7
1409d4eef: 48 89 d9                    	movq	%rbx, %rcx
1409d4ef2: e8 89 a4 2d 00              	callq	0x140caf380 <.text+0xcae380>
1409d4ef7: f3 0f 59 05 51 0a f5 01     	mulss	0x1f50a51(%rip), %xmm0  # 0x142925950
1409d4eff: f3 0f 58 c7                 	addss	%xmm7, %xmm0
1409d4f03: 0f 28 ce                    	movaps	%xmm6, %xmm1
1409d4f06: e8 05 f0 f4 01              	callq	0x142923f10 <.text+0x2922f10>
1409d4f0b: 0f 28 f8                    	movaps	%xmm0, %xmm7
1409d4f0e: 45 0f 57 c0                 	xorps	%xmm8, %xmm8
1409d4f12: f3 4c 0f 2a c7              	cvtsi2ss	%rdi, %xmm8
1409d4f17: 48 89 d9                    	movq	%rbx, %rcx
1409d4f1a: e8 61 a4 2d 00              	callq	0x140caf380 <.text+0xcae380>
1409d4f1f: f3 41 0f 58 c0              	addss	%xmm8, %xmm0
1409d4f24: f3 41 0f 5e c3              	divss	%xmm11, %xmm0
1409d4f29: f3 41 0f 59 fc              	mulss	%xmm12, %xmm7
1409d4f2e: f3 41 0f 58 fd              	addss	%xmm13, %xmm7
1409d4f33: 66 0f 7e f9                 	movd	%xmm7, %ecx
1409d4f37: 66 0f 7e c0                 	movd	%xmm0, %eax
1409d4f3b: 48 c1 e0 20                 	shlq	$0x20, %rax
1409d4f3f: 48 09 c1                    	orq	%rax, %rcx
1409d4f42: e8 b9 b7 2b 00              	callq	0x140c90700 <.text+0xc8f700>
1409d4f47: 44 0f 28 c0                 	movaps	%xmm0, %xmm8
1409d4f4b: 0f 28 f9                    	movaps	%xmm1, %xmm7
1409d4f4e: 48 89 d9                    	movq	%rbx, %rcx
1409d4f51: e8 2a a4 2d 00              	callq	0x140caf380 <.text+0xcae380>
1409d4f56: 44 0f 28 c8                 	movaps	%xmm0, %xmm9
1409d4f5a: 4c 89 f1                    	movq	%r14, %rcx
1409d4f5d: e8 1e 82 2c 00              	callq	0x140c9d180 <.text+0xc9c180>
1409d4f62: 44 0f 28 55 40              	movaps	0x40(%rbp), %xmm10
1409d4f67: 48 8d 4d 5c                 	leaq	0x5c(%rbp), %rcx
1409d4f6b: 48 8b 01                    	movq	(%rcx), %rax
1409d4f6e: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1409d4f72: 8b 41 08                    	movl	0x8(%rcx), %eax
1409d4f75: 89 45 e8                    	movl	%eax, -0x18(%rbp)
1409d4f78: c7 46 08 00 00 00 00        	movl	$0x0, 0x8(%rsi)
1409d4f7f: 48 c7 06 00 00 00 00        	movq	$0x0, (%rsi)
1409d4f86: 4c 89 f1                    	movq	%r14, %rcx
1409d4f89: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
1409d4f90: e8 0b f7 33 00              	callq	0x140d146a0 <.text+0xd136a0>
1409d4f95: 4c 8b 65 48                 	movq	0x48(%rbp), %r12
1409d4f99: 4d 85 e4                    	testq	%r12, %r12
1409d4f9c: 75 04                       	jne	0x1409d4fa2 <.text+0x9d3fa2>
1409d4f9e: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
1409d4fa2: 4c 8b 7d 60                 	movq	0x60(%rbp), %r15
1409d4fa6: 49 8b 04 24                 	movq	(%r12), %rax
1409d4faa: 4d 8b 6c 24 10              	movq	0x10(%r12), %r13
1409d4faf: 4c 29 e8                    	subq	%r13, %rax
1409d4fb2: 48 3d a7 00 00 00           	cmpq	$0xa7, %rax
1409d4fb8: 0f 86 c2 00 00 00           	jbe	0x1409d5080 <.text+0x9d4080>
1409d4fbe: 48 ff c7                    	incq	%rdi
1409d4fc1: f3 45 0f 59 c6              	mulss	%xmm14, %xmm8
1409d4fc6: f3 44 0f 58 ce              	addss	%xmm6, %xmm9
1409d4fcb: f3 41 0f 59 fe              	mulss	%xmm14, %xmm7
1409d4fd0: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1409d4fd5: 48 8d 0d 84 2e 90 00        	leaq	0x902e84(%rip), %rcx    # 0x1412d7e60 <.text+0x12d6e60>
1409d4fdc: 4a 89 0c 28                 	movq	%rcx, (%rax,%r13)
1409d4fe0: 42 c7 44 28 08 00 00 00 00  	movl	$0x0, 0x8(%rax,%r13)
1409d4fe9: 46 0f 11 54 28 18           	movups	%xmm10, 0x18(%rax,%r13)
1409d4fef: f3 46 0f 11 44 28 28        	movss	%xmm8, 0x28(%rax,%r13)
1409d4ff6: f3 46 0f 11 4c 28 2c        	movss	%xmm9, 0x2c(%rax,%r13)
1409d4ffd: f3 42 0f 11 7c 28 30        	movss	%xmm7, 0x30(%rax,%r13)
1409d5004: 48 8b 4d e0                 	movq	-0x20(%rbp), %rcx
1409d5008: 4a 89 4c 28 34              	movq	%rcx, 0x34(%rax,%r13)
1409d500d: 8b 4d e8                    	movl	-0x18(%rbp), %ecx
1409d5010: 42 89 4c 28 3c              	movl	%ecx, 0x3c(%rax,%r13)
1409d5015: 0f 10 85 f0 00 00 00        	movups	0xf0(%rbp), %xmm0
1409d501c: 42 0f 11 44 28 40           	movups	%xmm0, 0x40(%rax,%r13)
1409d5022: 8b 8d 00 01 00 00           	movl	0x100(%rbp), %ecx
1409d5028: 42 89 4c 28 50              	movl	%ecx, 0x50(%rax,%r13)
1409d502d: 48 b9 cd cc 4c 3d 03 00 00 00       	movabsq	$0x33d4ccccd, %rcx # imm = 0x33D4CCCCD
1409d5037: 4a 89 4c 28 54              	movq	%rcx, 0x54(%rax,%r13)
1409d503c: 46 0f 11 7c 28 60           	movups	%xmm15, 0x60(%rax,%r13)
1409d5042: 42 c7 44 28 70 00 00 80 3f  	movl	$0x3f800000, 0x70(%rax,%r13) # imm = 0x3F800000
1409d504b: 42 c6 44 28 78 00           	movb	$0x0, 0x78(%rax,%r13)
1409d5051: 4e 89 bc 28 88 00 00 00     	movq	%r15, 0x88(%rax,%r13)
1409d5059: 48 8d 0d d0 fc b8 00        	leaq	0xb8fcd0(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409d5060: 4a 89 8c 28 98 00 00 00     	movq	%rcx, 0x98(%rax,%r13)
1409d5068: 49 81 c5 a8 00 00 00        	addq	$0xa8, %r13
1409d506f: 4d 89 6c 24 10              	movq	%r13, 0x10(%r12)
1409d5074: 48 83 ff 60                 	cmpq	$0x60, %rdi
1409d5078: 0f 85 62 fe ff ff           	jne	0x1409d4ee0 <.text+0x9d3ee0>
1409d507e: eb 25                       	jmp	0x1409d50a5 <.text+0x9d40a5>
1409d5080: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409d5089: 41 b8 a8 00 00 00           	movl	$0xa8, %r8d
1409d508f: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409d5095: 4c 89 e1                    	movq	%r12, %rcx
1409d5098: 4c 89 ea                    	movq	%r13, %rdx
1409d509b: e8 c0 49 ed 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1409d50a0: e9 19 ff ff ff              	jmp	0x1409d4fbe <.text+0x9d3fbe>
1409d50a5: 31 f6                       	xorl	%esi, %esi
1409d50a7: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d50ab: f3 0f 10 15 51 08 f5 01     	movss	0x1f50851(%rip), %xmm2  # 0x142925904
1409d50b3: 31 d2                       	xorl	%edx, %edx
1409d50b5: e8 46 1a 03 00              	callq	0x140a06b00 <.text+0xa05b00>
1409d50ba: 48 3b 75 40                 	cmpq	0x40(%rbp), %rsi
1409d50be: 0f 80 2b 07 00 00           	jo	0x1409d57ef <.text+0x9d47ef>
1409d50c4: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1409d50cb: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1409d50d2: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1409d50d9: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1409d50e0: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
1409d50e7: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1409d50ee: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1409d50f5: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1409d50fc: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409d5100: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409d5104: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409d5108: 0f 10 5d 70                 	movups	0x70(%rbp), %xmm3
1409d510c: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1409d5113: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1409d511a: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1409d5121: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409d5128: f3 0f 10 05 d4 07 f5 01     	movss	0x1f507d4(%rip), %xmm0  # 0x142925904
1409d5130: 0f 13 45 d0                 	movlps	%xmm0, -0x30(%rbp)
1409d5134: c7 45 d8 00 00 80 3f        	movl	$0x3f800000, -0x28(%rbp) # imm = 0x3F800000
1409d513b: 48 8b 45 d0                 	movq	-0x30(%rbp), %rax
1409d513f: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
1409d5143: c7 45 e8 00 00 80 3f        	movl	$0x3f800000, -0x18(%rbp) # imm = 0x3F800000
1409d514a: 48 8d 5d 40                 	leaq	0x40(%rbp), %rbx
1409d514e: 48 8d 95 f0 00 00 00        	leaq	0xf0(%rbp), %rdx
1409d5155: 4c 8d 45 e0                 	leaq	-0x20(%rbp), %r8
1409d5159: 48 89 d9                    	movq	%rbx, %rcx
1409d515c: e8 1f 25 03 00              	callq	0x140a07680 <.text+0xa06680>
1409d5161: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
1409d5168: 4c 8b 08                    	movq	(%rax), %r9
1409d516b: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1409d516f: 48 8b 30                    	movq	(%rax), %rsi
1409d5172: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409d5177: 48 8d 15 e2 cf 0e 02        	leaq	0x20ecfe2(%rip), %rdx   # 0x142ac2160
1409d517e: 41 b8 07 00 00 00           	movl	$0x7, %r8d
1409d5184: 48 89 d9                    	movq	%rbx, %rcx
1409d5187: 4c 89 4d c0                 	movq	%r9, -0x40(%rbp)
1409d518b: e8 10 5b 14 00              	callq	0x140b1aca0 <.text+0xb19ca0>
1409d5190: 48 89 45 c8                 	movq	%rax, -0x38(%rbp)
1409d5194: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1409d5198: 4c 8b 38                    	movq	(%rax), %r15
1409d519b: 48 8b 45 38                 	movq	0x38(%rbp), %rax
1409d519f: 48 8b 38                    	movq	(%rax), %rdi
1409d51a2: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1409d51a7: 4c 89 7c 24 28              	movq	%r15, 0x28(%rsp)
1409d51ac: 48 c7 44 24 20 0e 00 00 00  	movq	$0xe, 0x20(%rsp)
1409d51b5: 48 8d 15 ab cf 0e 02        	leaq	0x20ecfab(%rip), %rdx   # 0x142ac2167
1409d51bc: 4c 8d 0d c1 cf 0e 02        	leaq	0x20ecfc1(%rip), %r9    # 0x142ac2184
1409d51c3: 41 b8 1d 00 00 00           	movl	$0x1d, %r8d
1409d51c9: 48 8b 4d 30                 	movq	0x30(%rbp), %rcx
1409d51cd: e8 6e ac 16 00              	callq	0x140b3fe40 <.text+0xb3ee40>
1409d51d2: 49 89 c6                    	movq	%rax, %r14
1409d51d5: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d51d9: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
1409d51e0: e8 bb f4 33 00              	callq	0x140d146a0 <.text+0xd136a0>
1409d51e5: 4c 8b 65 48                 	movq	0x48(%rbp), %r12
1409d51e9: 4d 85 e4                    	testq	%r12, %r12
1409d51ec: 75 04                       	jne	0x1409d51f2 <.text+0x9d41f2>
1409d51ee: 4c 8b 65 50                 	movq	0x50(%rbp), %r12
1409d51f2: 48 8b 5d 60                 	movq	0x60(%rbp), %rbx
1409d51f6: 49 8b 04 24                 	movq	(%r12), %rax
1409d51fa: 4d 8b 6c 24 10              	movq	0x10(%r12), %r13
1409d51ff: 4c 29 e8                    	subq	%r13, %rax
1409d5202: 48 83 f8 77                 	cmpq	$0x77, %rax
1409d5206: 0f 86 75 04 00 00           	jbe	0x1409d5681 <.text+0x9d4681>
1409d520c: 49 8b 44 24 08              	movq	0x8(%r12), %rax
1409d5211: 48 8d 0d c8 4b 90 00        	leaq	0x904bc8(%rip), %rcx    # 0x1412d9de0 <.text+0x12d8de0>
1409d5218: 4a 89 0c 28                 	movq	%rcx, (%rax,%r13)
1409d521c: 0f 28 35 9d 8e f6 01        	movaps	0x1f68e9d(%rip), %xmm6  # 0x14293e0c0
1409d5223: 42 0f 11 74 28 08           	movups	%xmm6, 0x8(%rax,%r13)
1409d5229: 4a c7 44 28 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r13)
1409d5232: 42 c7 44 28 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r13)
1409d523b: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1409d5245: 4a 89 4c 28 24              	movq	%rcx, 0x24(%rax,%r13)
1409d524a: 42 c7 44 28 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r13) # imm = 0x3F800000
1409d5253: 48 8b 4d c8                 	movq	-0x38(%rbp), %rcx
1409d5257: 4a 89 4c 28 38              	movq	%rcx, 0x38(%rax,%r13)
1409d525c: 4e 89 74 28 40              	movq	%r14, 0x40(%rax,%r13)
1409d5261: 42 c6 44 28 48 00           	movb	$0x0, 0x48(%rax,%r13)
1409d5267: 4a 89 5c 28 58              	movq	%rbx, 0x58(%rax,%r13)
1409d526c: 48 8d 0d bd fa b8 00        	leaq	0xb8fabd(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409d5273: 4a 89 4c 28 68              	movq	%rcx, 0x68(%rax,%r13)
1409d5278: 49 83 c5 78                 	addq	$0x78, %r13
1409d527c: 4d 89 6c 24 10              	movq	%r13, 0x10(%r12)
1409d5281: 48 8d 5d 40                 	leaq	0x40(%rbp), %rbx
1409d5285: f3 0f 10 0d 77 06 f5 01     	movss	0x1f50677(%rip), %xmm1  # 0x142925904
1409d528d: 48 89 d9                    	movq	%rbx, %rcx
1409d5290: e8 db 10 03 00              	callq	0x140a06370 <.text+0xa05370>
1409d5295: 4c 8d b5 f0 00 00 00        	leaq	0xf0(%rbp), %r14
1409d529c: 4c 8d 45 d0                 	leaq	-0x30(%rbp), %r8
1409d52a0: 4c 89 f1                    	movq	%r14, %rcx
1409d52a3: 48 89 da                    	movq	%rbx, %rdx
1409d52a6: e8 d5 23 03 00              	callq	0x140a07680 <.text+0xa06680>
1409d52ab: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409d52b0: 48 8d 15 db ce 0e 02        	leaq	0x20ecedb(%rip), %rdx   # 0x142ac2192
1409d52b7: 41 b8 0c 00 00 00           	movl	$0xc, %r8d
1409d52bd: 4c 89 f1                    	movq	%r14, %rcx
1409d52c0: 4c 8b 4d c0                 	movq	-0x40(%rbp), %r9
1409d52c4: e8 d7 59 14 00              	callq	0x140b1aca0 <.text+0xb19ca0>
1409d52c9: 48 89 c6                    	movq	%rax, %rsi
1409d52cc: 48 89 7c 24 30              	movq	%rdi, 0x30(%rsp)
1409d52d1: 4c 89 7c 24 28              	movq	%r15, 0x28(%rsp)
1409d52d6: 48 c7 44 24 20 13 00 00 00  	movq	$0x13, 0x20(%rsp)
1409d52df: 48 8d 15 b8 ce 0e 02        	leaq	0x20eceb8(%rip), %rdx   # 0x142ac219e
1409d52e6: 4c 8d 0d d3 ce 0e 02        	leaq	0x20eced3(%rip), %r9    # 0x142ac21c0
1409d52ed: 41 b8 22 00 00 00           	movl	$0x22, %r8d
1409d52f3: 4c 8b 6d 30                 	movq	0x30(%rbp), %r13
1409d52f7: 4c 89 e9                    	movq	%r13, %rcx
1409d52fa: e8 41 ab 16 00              	callq	0x140b3fe40 <.text+0xb3ee40>
1409d52ff: 48 89 c3                    	movq	%rax, %rbx
1409d5302: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d5306: 48 8b 95 a0 02 00 00        	movq	0x2a0(%rbp), %rdx
1409d530d: e8 8e f3 33 00              	callq	0x140d146a0 <.text+0xd136a0>
1409d5312: 4c 8b 75 48                 	movq	0x48(%rbp), %r14
1409d5316: 4d 85 f6                    	testq	%r14, %r14
1409d5319: 75 04                       	jne	0x1409d531f <.text+0x9d431f>
1409d531b: 4c 8b 75 50                 	movq	0x50(%rbp), %r14
1409d531f: 48 8b 7d 60                 	movq	0x60(%rbp), %rdi
1409d5323: 49 8b 06                    	movq	(%r14), %rax
1409d5326: 4d 8b 7e 10                 	movq	0x10(%r14), %r15
1409d532a: 4c 29 f8                    	subq	%r15, %rax
1409d532d: 48 3d 87 00 00 00           	cmpq	$0x87, %rax
1409d5333: 0f 86 6d 03 00 00           	jbe	0x1409d56a6 <.text+0x9d46a6>
1409d5339: 49 8b 46 08                 	movq	0x8(%r14), %rax
1409d533d: 48 8d 0d bc 61 90 00        	leaq	0x9061bc(%rip), %rcx    # 0x1412db500 <.text+0x12da500>
1409d5344: 4a 89 0c 38                 	movq	%rcx, (%rax,%r15)
1409d5348: 42 0f 11 74 38 08           	movups	%xmm6, 0x8(%rax,%r15)
1409d534e: 4a c7 44 38 18 00 00 00 00  	movq	$0x0, 0x18(%rax,%r15)
1409d5357: 42 c7 44 38 20 00 00 00 00  	movl	$0x0, 0x20(%rax,%r15)
1409d5360: 48 b9 00 00 80 3f 00 00 80 3f       	movabsq	$0x3f8000003f800000, %rcx # imm = 0x3F8000003F800000
1409d536a: 4a 89 4c 38 24              	movq	%rcx, 0x24(%rax,%r15)
1409d536f: 42 c7 44 38 2c 00 00 80 3f  	movl	$0x3f800000, 0x2c(%rax,%r15) # imm = 0x3F800000
1409d5378: 4a 89 74 38 38              	movq	%rsi, 0x38(%rax,%r15)
1409d537d: 4a 89 5c 38 40              	movq	%rbx, 0x40(%rax,%r15)
1409d5382: 42 c7 44 38 48 11 00 00 00  	movl	$0x11, 0x48(%rax,%r15)
1409d538b: 42 c6 44 38 58 00           	movb	$0x0, 0x58(%rax,%r15)
1409d5391: 4a 89 7c 38 68              	movq	%rdi, 0x68(%rax,%r15)
1409d5396: 48 8d 0d 93 f9 b8 00        	leaq	0xb8f993(%rip), %rcx    # 0x141564d30 <.text+0x1563d30>
1409d539d: 4a 89 4c 38 78              	movq	%rcx, 0x78(%rax,%r15)
1409d53a2: 49 81 c7 88 00 00 00        	addq	$0x88, %r15
1409d53a9: 4d 89 7e 10                 	movq	%r15, 0x10(%r14)
1409d53ad: 4c 8b 7d 38                 	movq	0x38(%rbp), %r15
1409d53b1: 48 8d 15 1b ce 0e 02        	leaq	0x20ece1b(%rip), %rdx   # 0x142ac21d3
1409d53b8: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d53bc: 41 b8 1e 00 00 00           	movl	$0x1e, %r8d
1409d53c2: e8 e9 9a 13 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
1409d53c7: 31 c0                       	xorl	%eax, %eax
1409d53c9: 48 3b 45 40                 	cmpq	0x40(%rbp), %rax
1409d53cd: 0f 80 6e 03 00 00           	jo	0x1409d5741 <.text+0x9d4741>
1409d53d3: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1409d53da: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1409d53e1: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1409d53e8: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1409d53ef: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
1409d53f6: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1409d53fd: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1409d5404: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1409d540b: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409d540f: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409d5413: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409d5417: 0f 10 5d 70                 	movups	0x70(%rbp), %xmm3
1409d541b: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1409d5422: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1409d5429: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1409d5430: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409d5437: 48 8b 85 90 02 00 00        	movq	0x290(%rbp), %rax
1409d543e: 48 8b 38                    	movq	(%rax), %rdi
1409d5441: 48 8b 45 28                 	movq	0x28(%rbp), %rax
1409d5445: 48 8b 30                    	movq	(%rax), %rsi
1409d5448: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409d544d: 48 8d 15 bc cd 0e 02        	leaq	0x20ecdbc(%rip), %rdx   # 0x142ac2210
1409d5454: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d545b: 41 b8 04 00 00 00           	movl	$0x4, %r8d
1409d5461: 49 89 f9                    	movq	%rdi, %r9
1409d5464: e8 37 58 14 00              	callq	0x140b1aca0 <.text+0xb19ca0>
1409d5469: 48 8b 45 20                 	movq	0x20(%rbp), %rax
1409d546d: 48 8b 18                    	movq	(%rax), %rbx
1409d5470: 4d 8b 37                    	movq	(%r15), %r14
1409d5473: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1409d5478: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1409d547d: 48 c7 44 24 20 0b 00 00 00  	movq	$0xb, 0x20(%rsp)
1409d5486: 48 8d 15 87 cd 0e 02        	leaq	0x20ecd87(%rip), %rdx   # 0x142ac2214
1409d548d: 4c 8d 0d a3 cd 0e 02        	leaq	0x20ecda3(%rip), %r9    # 0x142ac2237
1409d5494: 41 b8 23 00 00 00           	movl	$0x23, %r8d
1409d549a: 4c 89 e9                    	movq	%r13, %rcx
1409d549d: e8 9e a9 16 00              	callq	0x140b3fe40 <.text+0xb3ee40>
1409d54a2: 48 8d 15 99 cd 0e 02        	leaq	0x20ecd99(%rip), %rdx   # 0x142ac2242
1409d54a9: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d54ad: 41 b8 1d 00 00 00           	movl	$0x1d, %r8d
1409d54b3: e8 f8 99 13 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
1409d54b8: 31 c0                       	xorl	%eax, %eax
1409d54ba: 48 3b 45 40                 	cmpq	0x40(%rbp), %rax
1409d54be: 0f 80 b8 02 00 00           	jo	0x1409d577c <.text+0x9d477c>
1409d54c4: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1409d54cb: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1409d54d2: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1409d54d9: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1409d54e0: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
1409d54e7: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1409d54ee: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1409d54f5: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1409d54fc: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409d5500: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409d5504: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409d5508: 0f 10 5d 70                 	movups	0x70(%rbp), %xmm3
1409d550c: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1409d5513: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1409d551a: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1409d5521: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409d5528: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409d552d: 48 8d 15 44 cd 0e 02        	leaq	0x20ecd44(%rip), %rdx   # 0x142ac2278
1409d5534: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d553b: 41 b8 0a 00 00 00           	movl	$0xa, %r8d
1409d5541: 49 89 f9                    	movq	%rdi, %r9
1409d5544: e8 57 57 14 00              	callq	0x140b1aca0 <.text+0xb19ca0>
1409d5549: 48 8d 15 32 cd 0e 02        	leaq	0x20ecd32(%rip), %rdx   # 0x142ac2282
1409d5550: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
1409d5554: 41 b8 19 00 00 00           	movl	$0x19, %r8d
1409d555a: e8 51 99 13 00              	callq	0x140b0eeb0 <.text+0xb0deb0>
1409d555f: 31 c0                       	xorl	%eax, %eax
1409d5561: 48 3b 45 40                 	cmpq	0x40(%rbp), %rax
1409d5565: 0f 80 4c 02 00 00           	jo	0x1409d57b7 <.text+0x9d47b7>
1409d556b: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
1409d5572: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
1409d5579: 0f 10 85 a0 00 00 00        	movups	0xa0(%rbp), %xmm0
1409d5580: 0f 29 85 50 01 00 00        	movaps	%xmm0, 0x150(%rbp)
1409d5587: 0f 10 85 90 00 00 00        	movups	0x90(%rbp), %xmm0
1409d558e: 0f 29 85 40 01 00 00        	movaps	%xmm0, 0x140(%rbp)
1409d5595: 0f 10 85 80 00 00 00        	movups	0x80(%rbp), %xmm0
1409d559c: 0f 29 85 30 01 00 00        	movaps	%xmm0, 0x130(%rbp)
1409d55a3: 0f 10 45 40                 	movups	0x40(%rbp), %xmm0
1409d55a7: 0f 10 4d 50                 	movups	0x50(%rbp), %xmm1
1409d55ab: 0f 10 55 60                 	movups	0x60(%rbp), %xmm2
1409d55af: 0f 10 5d 70                 	movups	0x70(%rbp), %xmm3
1409d55b3: 0f 29 9d 20 01 00 00        	movaps	%xmm3, 0x120(%rbp)
1409d55ba: 0f 29 95 10 01 00 00        	movaps	%xmm2, 0x110(%rbp)
1409d55c1: 0f 29 8d 00 01 00 00        	movaps	%xmm1, 0x100(%rbp)
1409d55c8: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409d55cf: 48 89 74 24 20              	movq	%rsi, 0x20(%rsp)
1409d55d4: 48 8d 15 dd cc 0e 02        	leaq	0x20eccdd(%rip), %rdx   # 0x142ac22b8
1409d55db: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d55e2: 41 b8 06 00 00 00           	movl	$0x6, %r8d
1409d55e8: 49 89 f9                    	movq	%rdi, %r9
1409d55eb: e8 b0 56 14 00              	callq	0x140b1aca0 <.text+0xb19ca0>
1409d55f0: 4c 89 74 24 30              	movq	%r14, 0x30(%rsp)
1409d55f5: 48 89 5c 24 28              	movq	%rbx, 0x28(%rsp)
1409d55fa: 48 c7 44 24 20 0f 00 00 00  	movq	$0xf, 0x20(%rsp)
1409d5603: 48 8d 15 b4 cc 0e 02        	leaq	0x20eccb4(%rip), %rdx   # 0x142ac22be
1409d560a: 4c 8d 0d c9 cc 0e 02        	leaq	0x20eccc9(%rip), %r9    # 0x142ac22da
1409d5611: 41 b8 1c 00 00 00           	movl	$0x1c, %r8d
1409d5617: 4c 89 e9                    	movq	%r13, %rcx
1409d561a: e8 21 a8 16 00              	callq	0x140b3fe40 <.text+0xb3ee40>
1409d561f: 0f 28 b5 80 01 00 00        	movaps	0x180(%rbp), %xmm6
1409d5626: 0f 28 bd 90 01 00 00        	movaps	0x190(%rbp), %xmm7
1409d562d: 44 0f 28 85 a0 01 00 00     	movaps	0x1a0(%rbp), %xmm8
1409d5635: 44 0f 28 8d b0 01 00 00     	movaps	0x1b0(%rbp), %xmm9
1409d563d: 44 0f 28 95 c0 01 00 00     	movaps	0x1c0(%rbp), %xmm10
1409d5645: 44 0f 28 9d d0 01 00 00     	movaps	0x1d0(%rbp), %xmm11
1409d564d: 44 0f 28 a5 e0 01 00 00     	movaps	0x1e0(%rbp), %xmm12
1409d5655: 44 0f 28 ad f0 01 00 00     	movaps	0x1f0(%rbp), %xmm13
1409d565d: 44 0f 28 b5 00 02 00 00     	movaps	0x200(%rbp), %xmm14
1409d5665: 44 0f 28 bd 10 02 00 00     	movaps	0x210(%rbp), %xmm15
1409d566d: 48 81 c4 a8 02 00 00        	addq	$0x2a8, %rsp            # imm = 0x2A8
1409d5674: 5b                          	popq	%rbx
1409d5675: 5f                          	popq	%rdi
1409d5676: 5e                          	popq	%rsi
1409d5677: 41 5c                       	popq	%r12
1409d5679: 41 5d                       	popq	%r13
1409d567b: 41 5e                       	popq	%r14
1409d567d: 41 5f                       	popq	%r15
1409d567f: 5d                          	popq	%rbp
1409d5680: c3                          	retq
1409d5681: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409d568a: 41 b8 78 00 00 00           	movl	$0x78, %r8d
1409d5690: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409d5696: 4c 89 e1                    	movq	%r12, %rcx
1409d5699: 4c 89 ea                    	movq	%r13, %rdx
1409d569c: e8 bf 43 ed 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1409d56a1: e9 66 fb ff ff              	jmp	0x1409d520c <.text+0x9d420c>
1409d56a6: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409d56af: 41 b8 88 00 00 00           	movl	$0x88, %r8d
1409d56b5: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409d56bb: 4c 89 f1                    	movq	%r14, %rcx
1409d56be: 4c 89 fa                    	movq	%r15, %rdx
1409d56c1: e8 9a 43 ed 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1409d56c6: e9 6e fc ff ff              	jmp	0x1409d5339 <.text+0x9d4339>
1409d56cb: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
1409d56d4: 41 b8 78 00 00 00           	movl	$0x78, %r8d
1409d56da: 41 b9 01 00 00 00           	movl	$0x1, %r9d
1409d56e0: 4c 89 f1                    	movq	%r14, %rcx
1409d56e3: 4c 89 e2                    	movq	%r12, %rdx
1409d56e6: e8 75 43 ed 01              	callq	0x1428a9a60 <.text+0x28a8a60>
1409d56eb: e9 25 f7 ff ff              	jmp	0x1409d4e15 <.text+0x9d3e15>
1409d56f0: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1409d56f4: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409d56fb: 48 8d 05 06 cc 0e 02        	leaq	0x20ecc06(%rip), %rax   # 0x142ac2308
1409d5702: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d5707: 48 8d 0d 6a c7 0e 02        	leaq	0x20ec76a(%rip), %rcx   # 0x142ac1e78
1409d570e: 4c 8d 0d 43 c7 0e 02        	leaq	0x20ec743(%rip), %r9    # 0x142ac1e58
1409d5715: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409d571c: ba 2b 00 00 00              	movl	$0x2b, %edx
1409d5721: e8 3a 40 f0 01              	callq	0x1428d9760 <.text+0x28d8760>
1409d5726: e9 3d 01 00 00              	jmp	0x1409d5868 <.text+0x9d4868>
1409d572b: 4c 8d 05 7e b0 0f 02        	leaq	0x20fb07e(%rip), %r8    # 0x142ad07b0
1409d5732: b9 01 00 00 00              	movl	$0x1, %ecx
1409d5737: ba 1b 00 00 00              	movl	$0x1b, %edx
1409d573c: e8 a2 38 f0 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1409d5741: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1409d5745: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409d574c: 48 8d 05 a5 ca 0e 02        	leaq	0x20ecaa5(%rip), %rax   # 0x142ac21f8
1409d5753: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d5758: 48 8d 0d 19 c7 0e 02        	leaq	0x20ec719(%rip), %rcx   # 0x142ac1e78
1409d575f: 4c 8d 0d f2 c6 0e 02        	leaq	0x20ec6f2(%rip), %r9    # 0x142ac1e58
1409d5766: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409d576d: ba 2b 00 00 00              	movl	$0x2b, %edx
1409d5772: e8 e9 3f f0 01              	callq	0x1428d9760 <.text+0x28d8760>
1409d5777: e9 ec 00 00 00              	jmp	0x1409d5868 <.text+0x9d4868>
1409d577c: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1409d5780: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409d5787: 48 8d 05 d2 ca 0e 02        	leaq	0x20ecad2(%rip), %rax   # 0x142ac2260
1409d578e: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d5793: 48 8d 0d de c6 0e 02        	leaq	0x20ec6de(%rip), %rcx   # 0x142ac1e78
1409d579a: 4c 8d 0d b7 c6 0e 02        	leaq	0x20ec6b7(%rip), %r9    # 0x142ac1e58
1409d57a1: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409d57a8: ba 2b 00 00 00              	movl	$0x2b, %edx
1409d57ad: e8 ae 3f f0 01              	callq	0x1428d9760 <.text+0x28d8760>
1409d57b2: e9 b1 00 00 00              	jmp	0x1409d5868 <.text+0x9d4868>
1409d57b7: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1409d57bb: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409d57c2: 48 8d 05 d7 ca 0e 02        	leaq	0x20ecad7(%rip), %rax   # 0x142ac22a0
1409d57c9: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d57ce: 48 8d 0d a3 c6 0e 02        	leaq	0x20ec6a3(%rip), %rcx   # 0x142ac1e78
1409d57d5: 4c 8d 0d 7c c6 0e 02        	leaq	0x20ec67c(%rip), %r9    # 0x142ac1e58
1409d57dc: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409d57e3: ba 2b 00 00 00              	movl	$0x2b, %edx
1409d57e8: e8 73 3f f0 01              	callq	0x1428d9760 <.text+0x28d8760>
1409d57ed: eb 79                       	jmp	0x1409d5868 <.text+0x9d4868>
1409d57ef: 48 8b 45 58                 	movq	0x58(%rbp), %rax
1409d57f3: 48 89 85 00 01 00 00        	movq	%rax, 0x100(%rbp)
1409d57fa: 0f 10 45 48                 	movups	0x48(%rbp), %xmm0
1409d57fe: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
1409d5805: 48 8d 05 3c c9 0e 02        	leaq	0x20ec93c(%rip), %rax   # 0x142ac2148
1409d580c: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d5811: 48 8d 0d 60 c6 0e 02        	leaq	0x20ec660(%rip), %rcx   # 0x142ac1e78
1409d5818: 4c 8d 0d 89 c6 0e 02        	leaq	0x20ec689(%rip), %r9    # 0x142ac1ea8
1409d581f: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409d5826: ba 2b 00 00 00              	movl	$0x2b, %edx
1409d582b: e8 30 3f f0 01              	callq	0x1428d9760 <.text+0x28d8760>
1409d5830: eb 36                       	jmp	0x1409d5868 <.text+0x9d4868>
1409d5832: 48 8b 45 48                 	movq	0x48(%rbp), %rax
1409d5836: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
1409d583d: 48 8d 05 b4 c8 0e 02        	leaq	0x20ec8b4(%rip), %rax   # 0x142ac20f8
1409d5844: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
1409d5849: 48 8d 0d 28 c6 0e 02        	leaq	0x20ec628(%rip), %rcx   # 0x142ac1e78
1409d5850: 4c 8d 0d 01 c6 0e 02        	leaq	0x20ec601(%rip), %r9    # 0x142ac1e58
1409d5857: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
1409d585e: ba 2b 00 00 00              	movl	$0x2b, %edx
1409d5863: e8 f8 3e f0 01              	callq	0x1428d9760 <.text+0x28d8760>
1409d5868: 0f 0b                       	ud2
1409d586a: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
1409d5870: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d5875: 55                          	pushq	%rbp
1409d5876: 41 57                       	pushq	%r15
1409d5878: 41 56                       	pushq	%r14
1409d587a: 41 55                       	pushq	%r13
1409d587c: 41 54                       	pushq	%r12
1409d587e: 56                          	pushq	%rsi
1409d587f: 57                          	pushq	%rdi
1409d5880: 53                          	pushq	%rbx
1409d5881: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409d5888: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d588f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409d5895: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409d589b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409d58a1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409d58a7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409d58ad: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409d58b6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409d58bf: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409d58c8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409d58d0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409d58d8: 48 8b 95 f0 00 00 00        	movq	0xf0(%rbp), %rdx
1409d58df: 48 85 d2                    	testq	%rdx, %rdx
1409d58e2: 74 12                       	je	0x1409d58f6 <.text+0x9d48f6>
1409d58e4: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
1409d58eb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
1409d58f1: e8 2a e3 c3 ff              	callq	0x140613c20 <.text+0x612c20>
1409d58f6: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409d58fe: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409d5906: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409d590f: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409d5918: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409d5921: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409d5927: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409d592d: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409d5933: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409d5939: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409d593f: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409d5946: 5b                          	popq	%rbx
1409d5947: 5f                          	popq	%rdi
1409d5948: 5e                          	popq	%rsi
1409d5949: 41 5c                       	popq	%r12
1409d594b: 41 5d                       	popq	%r13
1409d594d: 41 5e                       	popq	%r14
1409d594f: 41 5f                       	popq	%r15
1409d5951: 5d                          	popq	%rbp
1409d5952: c3                          	retq
1409d5953: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
1409d5960: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d5965: 55                          	pushq	%rbp
1409d5966: 41 57                       	pushq	%r15
1409d5968: 41 56                       	pushq	%r14
1409d596a: 41 55                       	pushq	%r13
1409d596c: 41 54                       	pushq	%r12
1409d596e: 56                          	pushq	%rsi
1409d596f: 57                          	pushq	%rdi
1409d5970: 53                          	pushq	%rbx
1409d5971: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409d5978: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d597f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409d5985: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409d598b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409d5991: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409d5997: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409d599d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409d59a6: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409d59af: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409d59b8: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409d59c0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409d59c8: 80 bd 77 01 00 00 00        	cmpb	$0x0, 0x177(%rbp)
1409d59cf: 75 5d                       	jne	0x1409d5a2e <.text+0x9d4a2e>
1409d59d1: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409d59d9: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409d59e1: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409d59ea: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409d59f3: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409d59fc: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409d5a02: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409d5a08: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409d5a0e: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409d5a14: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409d5a1a: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409d5a21: 5b                          	popq	%rbx
1409d5a22: 5f                          	popq	%rdi
1409d5a23: 5e                          	popq	%rsi
1409d5a24: 41 5c                       	popq	%r12
1409d5a26: 41 5d                       	popq	%r13
1409d5a28: 41 5e                       	popq	%r14
1409d5a2a: 41 5f                       	popq	%r15
1409d5a2c: 5d                          	popq	%rbp
1409d5a2d: c3                          	retq
1409d5a2e: 48 8d 4d e0                 	leaq	-0x20(%rbp), %rcx
1409d5a32: e8 49 4c 77 ff              	callq	0x14014a680 <.text+0x149680>
1409d5a37: eb 98                       	jmp	0x1409d59d1 <.text+0x9d49d1>
1409d5a39: 0f 1f 80 00 00 00 00        	nopl	(%rax)
1409d5a40: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d5a45: 55                          	pushq	%rbp
1409d5a46: 41 57                       	pushq	%r15
1409d5a48: 41 56                       	pushq	%r14
1409d5a4a: 41 55                       	pushq	%r13
1409d5a4c: 41 54                       	pushq	%r12
1409d5a4e: 56                          	pushq	%rsi
1409d5a4f: 57                          	pushq	%rdi
1409d5a50: 53                          	pushq	%rbx
1409d5a51: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409d5a58: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d5a5f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409d5a65: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409d5a6b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409d5a71: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409d5a77: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409d5a7d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409d5a86: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409d5a8f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409d5a98: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409d5aa0: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409d5aa8: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d5aaf: e8 7c 71 37 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1409d5ab4: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409d5abc: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409d5ac4: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409d5acd: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409d5ad6: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409d5adf: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409d5ae5: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409d5aeb: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409d5af1: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409d5af7: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409d5afd: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409d5b04: 5b                          	popq	%rbx
1409d5b05: 5f                          	popq	%rdi
1409d5b06: 5e                          	popq	%rsi
1409d5b07: 41 5c                       	popq	%r12
1409d5b09: 41 5d                       	popq	%r13
1409d5b0b: 41 5e                       	popq	%r14
1409d5b0d: 41 5f                       	popq	%r15
1409d5b0f: 5d                          	popq	%rbp
1409d5b10: c3                          	retq
1409d5b11: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409d5b20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d5b25: 55                          	pushq	%rbp
1409d5b26: 41 57                       	pushq	%r15
1409d5b28: 41 56                       	pushq	%r14
1409d5b2a: 41 55                       	pushq	%r13
1409d5b2c: 41 54                       	pushq	%r12
1409d5b2e: 56                          	pushq	%rsi
1409d5b2f: 57                          	pushq	%rdi
1409d5b30: 53                          	pushq	%rbx
1409d5b31: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409d5b38: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d5b3f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409d5b45: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409d5b4b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409d5b51: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409d5b57: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409d5b5d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409d5b66: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409d5b6f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409d5b78: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409d5b80: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409d5b88: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d5b8f: e8 9c 70 37 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1409d5b94: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409d5b9c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409d5ba4: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409d5bad: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409d5bb6: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409d5bbf: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409d5bc5: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409d5bcb: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409d5bd1: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409d5bd7: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409d5bdd: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409d5be4: 5b                          	popq	%rbx
1409d5be5: 5f                          	popq	%rdi
1409d5be6: 5e                          	popq	%rsi
1409d5be7: 41 5c                       	popq	%r12
1409d5be9: 41 5d                       	popq	%r13
1409d5beb: 41 5e                       	popq	%r14
1409d5bed: 41 5f                       	popq	%r15
1409d5bef: 5d                          	popq	%rbp
1409d5bf0: c3                          	retq
1409d5bf1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409d5c00: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d5c05: 55                          	pushq	%rbp
1409d5c06: 41 57                       	pushq	%r15
1409d5c08: 41 56                       	pushq	%r14
1409d5c0a: 41 55                       	pushq	%r13
1409d5c0c: 41 54                       	pushq	%r12
1409d5c0e: 56                          	pushq	%rsi
1409d5c0f: 57                          	pushq	%rdi
1409d5c10: 53                          	pushq	%rbx
1409d5c11: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409d5c18: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d5c1f: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409d5c25: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409d5c2b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409d5c31: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409d5c37: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409d5c3d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409d5c46: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409d5c4f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409d5c58: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409d5c60: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409d5c68: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d5c6f: e8 bc 6f 37 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1409d5c74: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409d5c7c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409d5c84: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409d5c8d: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409d5c96: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409d5c9f: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409d5ca5: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409d5cab: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409d5cb1: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409d5cb7: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409d5cbd: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409d5cc4: 5b                          	popq	%rbx
1409d5cc5: 5f                          	popq	%rdi
1409d5cc6: 5e                          	popq	%rsi
1409d5cc7: 41 5c                       	popq	%r12
1409d5cc9: 41 5d                       	popq	%r13
1409d5ccb: 41 5e                       	popq	%r14
1409d5ccd: 41 5f                       	popq	%r15
1409d5ccf: 5d                          	popq	%rbp
1409d5cd0: c3                          	retq
1409d5cd1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409d5ce0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d5ce5: 55                          	pushq	%rbp
1409d5ce6: 41 57                       	pushq	%r15
1409d5ce8: 41 56                       	pushq	%r14
1409d5cea: 41 55                       	pushq	%r13
1409d5cec: 41 54                       	pushq	%r12
1409d5cee: 56                          	pushq	%rsi
1409d5cef: 57                          	pushq	%rdi
1409d5cf0: 53                          	pushq	%rbx
1409d5cf1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409d5cf8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d5cff: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409d5d05: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409d5d0b: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409d5d11: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409d5d17: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409d5d1d: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409d5d26: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409d5d2f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409d5d38: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409d5d40: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409d5d48: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d5d4f: e8 dc 6e 37 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1409d5d54: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409d5d5c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409d5d64: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409d5d6d: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409d5d76: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409d5d7f: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409d5d85: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409d5d8b: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409d5d91: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409d5d97: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409d5d9d: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409d5da4: 5b                          	popq	%rbx
1409d5da5: 5f                          	popq	%rdi
1409d5da6: 5e                          	popq	%rsi
1409d5da7: 41 5c                       	popq	%r12
1409d5da9: 41 5d                       	popq	%r13
1409d5dab: 41 5e                       	popq	%r14
1409d5dad: 41 5f                       	popq	%r15
1409d5daf: 5d                          	popq	%rbp
1409d5db0: c3                          	retq
1409d5db1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
1409d5dc0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
1409d5dc5: 55                          	pushq	%rbp
1409d5dc6: 41 57                       	pushq	%r15
1409d5dc8: 41 56                       	pushq	%r14
1409d5dca: 41 55                       	pushq	%r13
1409d5dcc: 41 54                       	pushq	%r12
1409d5dce: 56                          	pushq	%rsi
1409d5dcf: 57                          	pushq	%rdi
1409d5dd0: 53                          	pushq	%rbx
1409d5dd1: 48 81 ec d8 00 00 00        	subq	$0xd8, %rsp
1409d5dd8: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
1409d5ddf: 44 0f 29 7c 24 30           	movaps	%xmm15, 0x30(%rsp)
1409d5de5: 44 0f 29 74 24 40           	movaps	%xmm14, 0x40(%rsp)
1409d5deb: 44 0f 29 6c 24 50           	movaps	%xmm13, 0x50(%rsp)
1409d5df1: 44 0f 29 64 24 60           	movaps	%xmm12, 0x60(%rsp)
1409d5df7: 44 0f 29 5c 24 70           	movaps	%xmm11, 0x70(%rsp)
1409d5dfd: 44 0f 29 94 24 80 00 00 00  	movaps	%xmm10, 0x80(%rsp)
1409d5e06: 44 0f 29 8c 24 90 00 00 00  	movaps	%xmm9, 0x90(%rsp)
1409d5e0f: 44 0f 29 84 24 a0 00 00 00  	movaps	%xmm8, 0xa0(%rsp)
1409d5e18: 0f 29 bc 24 b0 00 00 00     	movaps	%xmm7, 0xb0(%rsp)
1409d5e20: 0f 29 b4 24 c0 00 00 00     	movaps	%xmm6, 0xc0(%rsp)
1409d5e28: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
1409d5e2f: e8 fc 6d 37 00              	callq	0x140d4cc30 <.text+0xd4bc30>
1409d5e34: 0f 28 b4 24 c0 00 00 00     	movaps	0xc0(%rsp), %xmm6
1409d5e3c: 0f 28 bc 24 b0 00 00 00     	movaps	0xb0(%rsp), %xmm7
1409d5e44: 44 0f 28 84 24 a0 00 00 00  	movaps	0xa0(%rsp), %xmm8
1409d5e4d: 44 0f 28 8c 24 90 00 00 00  	movaps	0x90(%rsp), %xmm9
1409d5e56: 44 0f 28 94 24 80 00 00 00  	movaps	0x80(%rsp), %xmm10
1409d5e5f: 44 0f 28 5c 24 70           	movaps	0x70(%rsp), %xmm11
1409d5e65: 44 0f 28 64 24 60           	movaps	0x60(%rsp), %xmm12
1409d5e6b: 44 0f 28 6c 24 50           	movaps	0x50(%rsp), %xmm13
1409d5e71: 44 0f 28 74 24 40           	movaps	0x40(%rsp), %xmm14
1409d5e77: 44 0f 28 7c 24 30           	movaps	0x30(%rsp), %xmm15
1409d5e7d: 48 81 c4 d8 00 00 00        	addq	$0xd8, %rsp
1409d5e84: 5b                          	popq	%rbx
1409d5e85: 5f                          	popq	%rdi
1409d5e86: 5e                          	popq	%rsi
1409d5e87: 41 5c                       	popq	%r12
1409d5e89: 41 5d                       	popq	%r13
1409d5e8b: 41 5e                       	popq	%r14
1409d5e8d: 41 5f                       	popq	%r15
1409d5e8f: 5d                          	popq	%rbp
1409d5e90: c3                          	retq
