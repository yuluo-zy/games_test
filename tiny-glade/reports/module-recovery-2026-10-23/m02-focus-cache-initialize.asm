
B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141752c50: 41 57                       	pushq	%r15
141752c52: 41 56                       	pushq	%r14
141752c54: 41 55                       	pushq	%r13
141752c56: 41 54                       	pushq	%r12
141752c58: 56                          	pushq	%rsi
141752c59: 57                          	pushq	%rdi
141752c5a: 55                          	pushq	%rbp
141752c5b: 53                          	pushq	%rbx
141752c5c: 48 81 ec 08 01 00 00        	subq	$0x108, %rsp            # imm = 0x108
141752c63: 48 89 d7                    	movq	%rdx, %rdi
141752c66: 48 89 ce                    	movq	%rcx, %rsi
141752c69: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
141752c73: 48 39 81 38 01 00 00        	cmpq	%rax, 0x138(%rcx)
141752c7a: 0f 85 d2 02 00 00           	jne	0x141752f52 <.text+0x1751f52>
141752c80: 48 89 f9                    	movq	%rdi, %rcx
141752c83: 48 89 f2                    	movq	%rsi, %rdx
141752c86: e8 45 68 c1 ff              	callq	0x1413694d0 <.text+0x13684d0>
141752c8b: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
141752c93: 48 89 f9                    	movq	%rdi, %rcx
141752c96: 48 89 f2                    	movq	%rsi, %rdx
141752c99: e8 82 55 c2 ff              	callq	0x141378220 <.text+0x1377220>
141752c9e: 48 89 84 24 98 00 00 00     	movq	%rax, 0x98(%rsp)
141752ca6: 48 89 f9                    	movq	%rdi, %rcx
141752ca9: 48 89 f2                    	movq	%rsi, %rdx
141752cac: e8 2f 9f c0 ff              	callq	0x14135cbe0 <.text+0x135bbe0>
141752cb1: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
141752cb9: 48 89 f9                    	movq	%rdi, %rcx
141752cbc: 48 89 f2                    	movq	%rsi, %rdx
141752cbf: e8 2c 15 bf ff              	callq	0x1413441f0 <.text+0x13431f0>
141752cc4: 48 89 84 24 88 00 00 00     	movq	%rax, 0x88(%rsp)
141752ccc: 0f 57 c0                    	xorps	%xmm0, %xmm0
141752ccf: 0f 11 84 24 c0 00 00 00     	movups	%xmm0, 0xc0(%rsp)
141752cd7: 48 c7 84 24 d0 00 00 00 00 00 00 00 	movq	$0x0, 0xd0(%rsp)
141752ce3: 48 89 f9                    	movq	%rdi, %rcx
141752ce6: 48 89 f2                    	movq	%rsi, %rdx
141752ce9: e8 92 be c1 ff              	callq	0x14136eb80 <.text+0x136db80>
141752cee: 48 89 84 24 80 00 00 00     	movq	%rax, 0x80(%rsp)
141752cf6: 48 89 f9                    	movq	%rdi, %rcx
141752cf9: 48 89 f2                    	movq	%rsi, %rdx
141752cfc: e8 1f 93 c6 ff              	callq	0x1413bc020 <.text+0x13bb020>
141752d01: 48 89 44 24 78              	movq	%rax, 0x78(%rsp)
141752d06: 48 89 f9                    	movq	%rdi, %rcx
141752d09: 48 89 f2                    	movq	%rsi, %rdx
141752d0c: e8 af 14 c3 ff              	callq	0x1413841c0 <.text+0x13831c0>
141752d11: 48 89 44 24 70              	movq	%rax, 0x70(%rsp)
141752d16: 48 89 f9                    	movq	%rdi, %rcx
141752d19: 48 89 f2                    	movq	%rsi, %rdx
141752d1c: e8 df 74 c2 ff              	callq	0x14137a200 <.text+0x1379200>
141752d21: 48 89 44 24 68              	movq	%rax, 0x68(%rsp)
141752d26: 48 89 f9                    	movq	%rdi, %rcx
141752d29: 48 89 f2                    	movq	%rsi, %rdx
141752d2c: e8 ff c3 c0 ff              	callq	0x14135f130 <.text+0x135e130>
141752d31: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
141752d36: 48 89 f9                    	movq	%rdi, %rcx
141752d39: 48 89 f2                    	movq	%rsi, %rdx
141752d3c: e8 ef be c6 ff              	callq	0x1413bec30 <.text+0x13bdc30>
141752d41: 48 89 44 24 58              	movq	%rax, 0x58(%rsp)
141752d46: 48 89 f9                    	movq	%rdi, %rcx
141752d49: 48 89 f2                    	movq	%rsi, %rdx
141752d4c: e8 6f eb c4 ff              	callq	0x1413a18c0 <.text+0x13a08c0>
141752d51: 48 89 44 24 50              	movq	%rax, 0x50(%rsp)
141752d56: 48 89 f9                    	movq	%rdi, %rcx
141752d59: 48 89 f2                    	movq	%rsi, %rdx
141752d5c: e8 8f 67 c4 ff              	callq	0x1413994f0 <.text+0x13984f0>
141752d61: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
141752d66: 48 89 f9                    	movq	%rdi, %rcx
141752d69: 48 89 f2                    	movq	%rsi, %rdx
141752d6c: e8 5f d9 c0 ff              	callq	0x1413606d0 <.text+0x135f6d0>
141752d71: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
141752d76: 48 89 f9                    	movq	%rdi, %rcx
141752d79: 48 89 f2                    	movq	%rsi, %rdx
141752d7c: e8 af 9a c6 ff              	callq	0x1413bc830 <.text+0x13bb830>
141752d81: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
141752d86: 48 89 f9                    	movq	%rdi, %rcx
141752d89: 48 89 f2                    	movq	%rsi, %rdx
141752d8c: e8 ff 70 bf ff              	callq	0x141349e90 <.text+0x1348e90>
141752d91: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
141752d96: 48 89 f9                    	movq	%rdi, %rcx
141752d99: 48 89 f2                    	movq	%rsi, %rdx
141752d9c: e8 6f 3a c1 ff              	callq	0x141366810 <.text+0x1365810>
141752da1: 49 89 c4                    	movq	%rax, %r12
141752da4: 48 89 f9                    	movq	%rdi, %rcx
141752da7: 48 89 f2                    	movq	%rsi, %rdx
141752daa: e8 c1 8f bf ff              	callq	0x14134bd70 <.text+0x134ad70>
141752daf: 49 89 c5                    	movq	%rax, %r13
141752db2: 48 89 f9                    	movq	%rdi, %rcx
141752db5: 48 89 f2                    	movq	%rsi, %rdx
141752db8: e8 93 9c c1 ff              	callq	0x14136ca50 <.text+0x136ba50>
141752dbd: 48 89 c5                    	movq	%rax, %rbp
141752dc0: 48 89 f9                    	movq	%rdi, %rcx
141752dc3: 48 89 f2                    	movq	%rsi, %rdx
141752dc6: e8 45 82 c1 ff              	callq	0x14136b010 <.text+0x136a010>
141752dcb: 48 89 c3                    	movq	%rax, %rbx
141752dce: 48 89 f9                    	movq	%rdi, %rcx
141752dd1: 48 89 f2                    	movq	%rsi, %rdx
141752dd4: e8 47 c2 bf ff              	callq	0x14134f020 <.text+0x134e020>
141752dd9: 49 89 c6                    	movq	%rax, %r14
141752ddc: 4c 8b bf a0 05 00 00        	movq	0x5a0(%rdi), %r15
141752de3: 48 8b 86 38 01 00 00        	movq	0x138(%rsi), %rax
141752dea: 48 b9 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rcx # imm = 0x8000000000000001
141752df4: 48 39 c8                    	cmpq	%rcx, %rax
141752df7: 74 24                       	je	0x141752e1d <.text+0x1751e1d>
141752df9: 48 89 c1                    	movq	%rax, %rcx
141752dfc: 48 f7 d9                    	negq	%rcx
141752dff: 70 1c                       	jo	0x141752e1d <.text+0x1751e1d>
141752e01: 73 1a                       	jae	0x141752e1d <.text+0x1751e1d>
141752e03: 48 8b 8e 40 01 00 00        	movq	0x140(%rsi), %rcx
141752e0a: 48 c1 e0 02                 	shlq	$0x2, %rax
141752e0e: 48 8d 14 40                 	leaq	(%rax,%rax,2), %rdx
141752e12: 41 b8 04 00 00 00           	movl	$0x4, %r8d
141752e18: e8 03 0e ec fe              	callq	0x140613c20 <.text+0x612c20>
141752e1d: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
141752e27: 48 89 86 38 01 00 00        	movq	%rax, 0x138(%rsi)
141752e2e: c7 86 50 01 00 00 00 00 00 00       	movl	$0x0, 0x150(%rsi)
141752e38: 8b 84 24 d4 00 00 00        	movl	0xd4(%rsp), %eax
141752e3f: 89 86 74 01 00 00           	movl	%eax, 0x174(%rsi)
141752e45: 0f 10 84 24 b4 00 00 00     	movups	0xb4(%rsp), %xmm0
141752e4d: 0f 10 8c 24 c4 00 00 00     	movups	0xc4(%rsp), %xmm1
141752e55: 0f 11 86 54 01 00 00        	movups	%xmm0, 0x154(%rsi)
141752e5c: 0f 11 8e 64 01 00 00        	movups	%xmm1, 0x164(%rsi)
141752e63: 48 8b 84 24 a0 00 00 00     	movq	0xa0(%rsp), %rax
141752e6b: 48 89 86 78 01 00 00        	movq	%rax, 0x178(%rsi)
141752e72: 48 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %rax
141752e7a: 48 89 86 80 01 00 00        	movq	%rax, 0x180(%rsi)
141752e81: 48 8b 84 24 90 00 00 00     	movq	0x90(%rsp), %rax
141752e89: 48 89 86 88 01 00 00        	movq	%rax, 0x188(%rsi)
141752e90: 48 8b 84 24 88 00 00 00     	movq	0x88(%rsp), %rax
141752e98: 48 89 86 90 01 00 00        	movq	%rax, 0x190(%rsi)
141752e9f: 48 8b 84 24 80 00 00 00     	movq	0x80(%rsp), %rax
141752ea7: 48 89 86 98 01 00 00        	movq	%rax, 0x198(%rsi)
141752eae: 48 8b 44 24 78              	movq	0x78(%rsp), %rax
141752eb3: 48 89 86 a0 01 00 00        	movq	%rax, 0x1a0(%rsi)
141752eba: 48 8b 44 24 70              	movq	0x70(%rsp), %rax
141752ebf: 48 89 86 a8 01 00 00        	movq	%rax, 0x1a8(%rsi)
141752ec6: 48 8b 44 24 68              	movq	0x68(%rsp), %rax
141752ecb: 48 89 86 b0 01 00 00        	movq	%rax, 0x1b0(%rsi)
141752ed2: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
141752ed7: 48 89 86 b8 01 00 00        	movq	%rax, 0x1b8(%rsi)
141752ede: 48 8b 44 24 58              	movq	0x58(%rsp), %rax
141752ee3: 48 89 86 c0 01 00 00        	movq	%rax, 0x1c0(%rsi)
141752eea: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
141752eef: 48 89 86 c8 01 00 00        	movq	%rax, 0x1c8(%rsi)
141752ef6: 48 8b 44 24 48              	movq	0x48(%rsp), %rax
141752efb: 48 89 86 d0 01 00 00        	movq	%rax, 0x1d0(%rsi)
141752f02: 48 8b 44 24 40              	movq	0x40(%rsp), %rax
141752f07: 48 89 86 d8 01 00 00        	movq	%rax, 0x1d8(%rsi)
141752f0e: 48 8b 44 24 38              	movq	0x38(%rsp), %rax
141752f13: 48 89 86 e0 01 00 00        	movq	%rax, 0x1e0(%rsi)
141752f1a: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
141752f1f: 48 89 86 e8 01 00 00        	movq	%rax, 0x1e8(%rsi)
141752f26: 4c 89 a6 f0 01 00 00        	movq	%r12, 0x1f0(%rsi)
141752f2d: 4c 89 ae f8 01 00 00        	movq	%r13, 0x1f8(%rsi)
141752f34: 48 89 ae 00 02 00 00        	movq	%rbp, 0x200(%rsi)
141752f3b: 48 89 9e 08 02 00 00        	movq	%rbx, 0x208(%rsi)
141752f42: 4c 89 b6 10 02 00 00        	movq	%r14, 0x210(%rsi)
141752f49: 4c 89 be 18 02 00 00        	movq	%r15, 0x218(%rsi)
141752f50: eb 18                       	jmp	0x141752f6a <.text+0x1751f6a>
