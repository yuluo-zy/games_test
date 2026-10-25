
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140df3e70: 41 57                       	pushq	%r15
140df3e72: 41 56                       	pushq	%r14
140df3e74: 41 54                       	pushq	%r12
140df3e76: 56                          	pushq	%rsi
140df3e77: 57                          	pushq	%rdi
140df3e78: 53                          	pushq	%rbx
140df3e79: 48 83 ec 68                 	subq	$0x68, %rsp
140df3e7d: 48 89 ce                    	movq	%rcx, %rsi
140df3e80: 49 8b 00                    	movq	(%r8), %rax
140df3e83: 48 8d 0d c6 52 d8 01        	leaq	0x1d852c6(%rip), %rcx   # 0x142b79150
140df3e8a: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
140df3e8e: 48 01 c8                    	addq	%rcx, %rax
140df3e91: ff e0                       	jmpq	*%rax
140df3e93: 49 8b 40 18                 	movq	0x18(%r8), %rax
140df3e97: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140df3e9c: 41 0f 10 40 08              	movups	0x8(%r8), %xmm0
140df3ea1: 0f 11 44 24 28              	movups	%xmm0, 0x28(%rsp)
140df3ea6: 48 c7 44 24 20 00 00 00 00  	movq	$0x0, 0x20(%rsp)
140df3eaf: e9 f6 00 00 00              	jmp	0x140df3faa <.text+0xdf2faa>
140df3eb4: 49 8b 40 08                 	movq	0x8(%r8), %rax
140df3eb8: 80 74 02 60 01              	xorb	$0x1, 0x60(%rdx,%rax)
140df3ebd: e9 3f 01 00 00              	jmp	0x140df4001 <.text+0xdf3001>
140df3ec2: 48 8d 42 18                 	leaq	0x18(%rdx), %rax
140df3ec6: 8b 5a 30                    	movl	0x30(%rdx), %ebx
140df3ec9: 4c 8d 05 10 4f d8 01        	leaq	0x1d84f10(%rip), %r8    # 0x142b78de0
140df3ed0: 48 8d 4c 24 20              	leaq	0x20(%rsp), %rcx
140df3ed5: 48 89 d7                    	movq	%rdx, %rdi
140df3ed8: 48 89 c2                    	movq	%rax, %rdx
140df3edb: e8 50 c6 c5 00              	callq	0x141a50530 <.text+0x1a4f530>
140df3ee0: 48 89 f8                    	movq	%rdi, %rax
140df3ee3: 89 5c 24 38                 	movl	%ebx, 0x38(%rsp)
140df3ee7: 8b 4f 60                    	movl	0x60(%rdi), %ecx
140df3eea: 89 4c 24 60                 	movl	%ecx, 0x60(%rsp)
140df3eee: 0f b7 4f 64                 	movzwl	0x64(%rdi), %ecx
140df3ef2: 66 89 4c 24 64              	movw	%cx, 0x64(%rsp)
140df3ef7: 0f 10 44 24 20              	movups	0x20(%rsp), %xmm0
140df3efc: 0f 29 44 24 40              	movaps	%xmm0, 0x40(%rsp)
140df3f01: 48 8b 4c 24 30              	movq	0x30(%rsp), %rcx
140df3f06: 48 89 4c 24 50              	movq	%rcx, 0x50(%rsp)
140df3f0b: 8b 4c 24 38                 	movl	0x38(%rsp), %ecx
140df3f0f: 89 4c 24 58                 	movl	%ecx, 0x58(%rsp)
140df3f13: 8b 4c 24 3c                 	movl	0x3c(%rsp), %ecx
140df3f17: 89 4c 24 5c                 	movl	%ecx, 0x5c(%rsp)
140df3f1b: 4c 8d 77 38                 	leaq	0x38(%rdi), %r14
140df3f1f: 48 8b 7f 38                 	movq	0x38(%rdi), %rdi
140df3f23: 48 b9 02 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffe, %rcx # imm = 0x8000000000000002
140df3f2d: 48 39 cf                    	cmpq	%rcx, %rdi
140df3f30: 0f 8c a0 00 00 00           	jl	0x140df3fd6 <.text+0xdf2fd6>
140df3f36: 48 8b 58 40                 	movq	0x40(%rax), %rbx
140df3f3a: 4c 8b 78 48                 	movq	0x48(%rax), %r15
140df3f3e: 4d 85 ff                    	testq	%r15, %r15
140df3f41: 74 79                       	je	0x140df3fbc <.text+0xdf2fbc>
140df3f43: 4c 8d 63 10                 	leaq	0x10(%rbx), %r12
140df3f47: eb 10                       	jmp	0x140df3f59 <.text+0xdf2f59>
140df3f49: 0f 1f 80 00 00 00 00        	nopl	(%rax)
140df3f50: 49 83 c4 20                 	addq	$0x20, %r12
140df3f54: 49 ff cf                    	decq	%r15
140df3f57: 74 63                       	je	0x140df3fbc <.text+0xdf2fbc>
140df3f59: 49 8b 54 24 f8              	movq	-0x8(%r12), %rdx
140df3f5e: 48 85 d2                    	testq	%rdx, %rdx
140df3f61: 74 ed                       	je	0x140df3f50 <.text+0xdf2f50>
140df3f63: 49 8b 0c 24                 	movq	(%r12), %rcx
140df3f67: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140df3f6d: e8 ae fc 81 ff              	callq	0x140613c20 <.text+0x612c20>
140df3f72: eb dc                       	jmp	0x140df3f50 <.text+0xdf2f50>
140df3f74: 48 b8 01 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffff, %rax # imm = 0x8000000000000001
140df3f7e: 48 89 06                    	movq	%rax, (%rsi)
140df3f81: 48 c7 46 08 00 00 00 00     	movq	$0x0, 0x8(%rsi)
140df3f89: e9 80 00 00 00              	jmp	0x140df400e <.text+0xdf300e>
140df3f8e: 49 8b 40 18                 	movq	0x18(%r8), %rax
140df3f92: 48 89 44 24 38              	movq	%rax, 0x38(%rsp)
140df3f97: 41 0f 10 40 08              	movups	0x8(%r8), %xmm0
140df3f9c: 0f 11 44 24 28              	movups	%xmm0, 0x28(%rsp)
140df3fa1: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140df3faa: 48 8d 44 24 20              	leaq	0x20(%rsp), %rax
140df3faf: 48 89 d1                    	movq	%rdx, %rcx
140df3fb2: 48 89 c2                    	movq	%rax, %rdx
140df3fb5: e8 56 fd ff ff              	callq	0x140df3d10 <.text+0xdf2d10>
140df3fba: eb 45                       	jmp	0x140df4001 <.text+0xdf3001>
140df3fbc: 48 85 ff                    	testq	%rdi, %rdi
140df3fbf: 74 15                       	je	0x140df3fd6 <.text+0xdf2fd6>
140df3fc1: 48 c1 e7 05                 	shlq	$0x5, %rdi
140df3fc5: 41 b8 08 00 00 00           	movl	$0x8, %r8d
140df3fcb: 48 89 d9                    	movq	%rbx, %rcx
140df3fce: 48 89 fa                    	movq	%rdi, %rdx
140df3fd1: e8 4a fc 81 ff              	callq	0x140613c20 <.text+0x612c20>
140df3fd6: 48 8b 44 24 60              	movq	0x60(%rsp), %rax
140df3fdb: 49 89 46 20                 	movq	%rax, 0x20(%r14)
140df3fdf: 0f 28 44 24 40              	movaps	0x40(%rsp), %xmm0
140df3fe4: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
140df3fe9: 49 89 46 10                 	movq	%rax, 0x10(%r14)
140df3fed: 8b 44 24 58                 	movl	0x58(%rsp), %eax
140df3ff1: 41 89 46 18                 	movl	%eax, 0x18(%r14)
140df3ff5: 8b 44 24 5c                 	movl	0x5c(%rsp), %eax
140df3ff9: 41 89 46 1c                 	movl	%eax, 0x1c(%r14)
140df3ffd: 41 0f 11 06                 	movups	%xmm0, (%r14)
140df4001: 48 b8 1e 00 00 00 00 00 00 80       	movabsq	$-0x7fffffffffffffe2, %rax # imm = 0x800000000000001E
140df400b: 48 89 06                    	movq	%rax, (%rsi)
140df400e: 48 89 f0                    	movq	%rsi, %rax
140df4011: 48 83 c4 68                 	addq	$0x68, %rsp
140df4015: 5b                          	popq	%rbx
140df4016: 5f                          	popq	%rdi
140df4017: 5e                          	popq	%rsi
140df4018: 41 5c                       	popq	%r12
140df401a: 41 5e                       	popq	%r14
140df401c: 41 5f                       	popq	%r15
140df401e: c3                          	retq
