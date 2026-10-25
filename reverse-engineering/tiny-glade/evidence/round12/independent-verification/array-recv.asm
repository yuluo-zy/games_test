
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
141679d70: 55                          	pushq	%rbp
141679d71: 41 57                       	pushq	%r15
141679d73: 41 56                       	pushq	%r14
141679d75: 41 55                       	pushq	%r13
141679d77: 41 54                       	pushq	%r12
141679d79: 56                          	pushq	%rsi
141679d7a: 57                          	pushq	%rdi
141679d7b: 53                          	pushq	%rbx
141679d7c: 48 81 ec 88 00 00 00        	subq	$0x88, %rsp
141679d83: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
141679d8b: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
141679d93: 48 89 ce                    	movq	%rcx, %rsi
141679d96: 48 89 55 c8                 	movq	%rdx, -0x38(%rbp)
141679d9a: 44 89 45 d0                 	movl	%r8d, -0x30(%rbp)
141679d9e: 0f 57 c0                    	xorps	%xmm0, %xmm0
141679da1: 0f 29 45 b0                 	movaps	%xmm0, -0x50(%rbp)
141679da5: 0f 29 45 a0                 	movaps	%xmm0, -0x60(%rbp)
141679da9: 48 c7 45 c0 00 00 00 00     	movq	$0x0, -0x40(%rbp)
141679db1: 4c 8d 65 a0                 	leaq	-0x60(%rbp), %r12
141679db5: 4c 8d 6d c8                 	leaq	-0x38(%rbp), %r13
141679db9: 48 8d 7d d8                 	leaq	-0x28(%rbp), %rdi
141679dbd: 48 8d 5d f0                 	leaq	-0x10(%rbp), %rbx
141679dc1: 31 c9                       	xorl	%ecx, %ecx
141679dc3: eb 11                       	jmp	0x141679dd6 <.text+0x1678dd6>
141679dc5: e8 a6 bb 6d ff              	callq	0x140d55970 <.text+0xd54970>
141679dca: 66 0f 1f 44 00 00           	nopw	(%rax,%rax)
141679dd0: 41 ff c6                    	incl	%r14d
141679dd3: 44 89 f1                    	movl	%r14d, %ecx
141679dd6: 41 89 ce                    	movl	%ecx, %r14d
141679dd9: 48 8b 0e                    	movq	(%rsi), %rcx
141679ddc: 48 8b 96 90 01 00 00        	movq	0x190(%rsi), %rdx
141679de3: 4c 8b 86 98 01 00 00        	movq	0x198(%rsi), %r8
141679dea: 48 ff ca                    	decq	%rdx
141679ded: 31 c0                       	xorl	%eax, %eax
141679def: 48 2b 86 88 01 00 00        	subq	0x188(%rsi), %rax
141679df6: 48 21 ca                    	andq	%rcx, %rdx
141679df9: 4d 8b 0c d0                 	movq	(%r8,%rdx,8), %r9
141679dfd: 4c 8d 51 01                 	leaq	0x1(%rcx), %r10
141679e01: 4d 39 ca                    	cmpq	%r9, %r10
141679e04: 0f 85 86 00 00 00           	jne	0x141679e90 <.text+0x1678e90>
141679e0a: 4c 8d 52 01                 	leaq	0x1(%rdx), %r10
141679e0e: 4c 3b 96 80 01 00 00        	cmpq	0x180(%rsi), %r10
141679e15: 72 0d                       	jb	0x141679e24 <.text+0x1678e24>
141679e17: 48 21 c8                    	andq	%rcx, %rax
141679e1a: 48 03 86 88 01 00 00        	addq	0x188(%rsi), %rax
141679e21: 49 89 c1                    	movq	%rax, %r9
141679e24: 48 89 c8                    	movq	%rcx, %rax
141679e27: f0                          	lock
141679e28: 4c 0f b1 0e                 	cmpxchgq	%r9, (%rsi)
141679e2c: 0f 84 7d 02 00 00           	je	0x14167a0af <.text+0x16790af>
141679e32: 41 83 fe 06                 	cmpl	$0x6, %r14d
141679e36: b8 06 00 00 00              	movl	$0x6, %eax
141679e3b: 41 0f 42 c6                 	cmovbl	%r14d, %eax
141679e3f: b9 01 00 00 00              	movl	$0x1, %ecx
141679e44: 45 85 f6                    	testl	%r14d, %r14d
141679e47: 74 8d                       	je	0x141679dd6 <.text+0x1678dd6>
141679e49: 0f af c0                    	imull	%eax, %eax
141679e4c: 83 f8 08                    	cmpl	$0x8, %eax
141679e4f: 72 24                       	jb	0x141679e75 <.text+0x1678e75>
141679e51: 89 c1                       	movl	%eax, %ecx
141679e53: 83 e1 38                    	andl	$0x38, %ecx
141679e56: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141679e60: f3 90                       	pause
141679e62: f3 90                       	pause
141679e64: f3 90                       	pause
141679e66: f3 90                       	pause
141679e68: f3 90                       	pause
141679e6a: f3 90                       	pause
141679e6c: f3 90                       	pause
141679e6e: f3 90                       	pause
141679e70: 83 c1 f8                    	addl	$-0x8, %ecx
141679e73: 75 eb                       	jne	0x141679e60 <.text+0x1678e60>
141679e75: 83 e0 05                    	andl	$0x5, %eax
141679e78: 0f 84 52 ff ff ff           	je	0x141679dd0 <.text+0x1678dd0>
141679e7e: 66 90                       	nop
141679e80: f3 90                       	pause
141679e82: ff c8                       	decl	%eax
141679e84: 75 fa                       	jne	0x141679e80 <.text+0x1678e80>
141679e86: e9 45 ff ff ff              	jmp	0x141679dd0 <.text+0x1678dd0>
141679e8b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141679e90: 49 39 c9                    	cmpq	%rcx, %r9
141679e93: 75 7b                       	jne	0x141679f10 <.text+0x1678f10>
141679e95: 0f ae f0                    	mfence
141679e98: 48 8b 86 80 00 00 00        	movq	0x80(%rsi), %rax
141679e9f: 48 8b 96 90 01 00 00        	movq	0x190(%rsi), %rdx
141679ea6: 49 89 d0                    	movq	%rdx, %r8
141679ea9: 49 f7 d0                    	notq	%r8
141679eac: 49 21 c0                    	andq	%rax, %r8
141679eaf: 49 39 c8                    	cmpq	%rcx, %r8
141679eb2: 0f 84 b8 00 00 00           	je	0x141679f70 <.text+0x1678f70>
141679eb8: 41 83 fe 06                 	cmpl	$0x6, %r14d
141679ebc: b8 06 00 00 00              	movl	$0x6, %eax
141679ec1: 41 0f 42 c6                 	cmovbl	%r14d, %eax
141679ec5: b9 01 00 00 00              	movl	$0x1, %ecx
141679eca: 45 85 f6                    	testl	%r14d, %r14d
141679ecd: 0f 84 03 ff ff ff           	je	0x141679dd6 <.text+0x1678dd6>
141679ed3: 0f af c0                    	imull	%eax, %eax
141679ed6: 83 f8 08                    	cmpl	$0x8, %eax
141679ed9: 72 1a                       	jb	0x141679ef5 <.text+0x1678ef5>
141679edb: 89 c1                       	movl	%eax, %ecx
141679edd: 83 e1 38                    	andl	$0x38, %ecx
141679ee0: f3 90                       	pause
141679ee2: f3 90                       	pause
141679ee4: f3 90                       	pause
141679ee6: f3 90                       	pause
141679ee8: f3 90                       	pause
141679eea: f3 90                       	pause
141679eec: f3 90                       	pause
141679eee: f3 90                       	pause
141679ef0: 83 c1 f8                    	addl	$-0x8, %ecx
141679ef3: 75 eb                       	jne	0x141679ee0 <.text+0x1678ee0>
141679ef5: 83 e0 05                    	andl	$0x5, %eax
141679ef8: 0f 84 d2 fe ff ff           	je	0x141679dd0 <.text+0x1678dd0>
141679efe: 66 90                       	nop
141679f00: f3 90                       	pause
141679f02: ff c8                       	decl	%eax
141679f04: 75 fa                       	jne	0x141679f00 <.text+0x1678f00>
141679f06: e9 c5 fe ff ff              	jmp	0x141679dd0 <.text+0x1678dd0>
141679f0b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141679f10: 41 83 fe 07                 	cmpl	$0x7, %r14d
141679f14: 0f 83 ab fe ff ff           	jae	0x141679dc5 <.text+0x1678dc5>
141679f1a: 45 85 f6                    	testl	%r14d, %r14d
141679f1d: 0f 84 ad fe ff ff           	je	0x141679dd0 <.text+0x1678dd0>
141679f23: 44 89 f0                    	movl	%r14d, %eax
141679f26: 0f af c0                    	imull	%eax, %eax
141679f29: 8d 48 ff                    	leal	-0x1(%rax), %ecx
141679f2c: 83 f9 07                    	cmpl	$0x7, %ecx
141679f2f: 72 24                       	jb	0x141679f55 <.text+0x1678f55>
141679f31: 89 c1                       	movl	%eax, %ecx
141679f33: 83 e1 f8                    	andl	$-0x8, %ecx
141679f36: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
141679f40: f3 90                       	pause
141679f42: f3 90                       	pause
141679f44: f3 90                       	pause
141679f46: f3 90                       	pause
141679f48: f3 90                       	pause
141679f4a: f3 90                       	pause
141679f4c: f3 90                       	pause
141679f4e: f3 90                       	pause
141679f50: 83 c1 f8                    	addl	$-0x8, %ecx
141679f53: 75 eb                       	jne	0x141679f40 <.text+0x1678f40>
141679f55: 83 e0 05                    	andl	$0x5, %eax
141679f58: 0f 84 72 fe ff ff           	je	0x141679dd0 <.text+0x1678dd0>
141679f5e: 66 90                       	nop
141679f60: f3 90                       	pause
141679f62: ff c8                       	decl	%eax
141679f64: 75 fa                       	jne	0x141679f60 <.text+0x1678f60>
141679f66: e9 65 fe ff ff              	jmp	0x141679dd0 <.text+0x1678dd0>
141679f6b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
141679f70: 48 85 c2                    	testq	%rax, %rdx
141679f73: 0f 85 72 01 00 00           	jne	0x14167a0eb <.text+0x16790eb>
141679f79: 44 8b 75 d0                 	movl	-0x30(%rbp), %r14d
141679f7d: 41 81 fe 00 ca 9a 3b        	cmpl	$0x3b9aca00, %r14d      # imm = 0x3B9ACA00
141679f84: 74 1e                       	je	0x141679fa4 <.text+0x1678fa4>
141679f86: 4c 8b 7d c8                 	movq	-0x38(%rbp), %r15
141679f8a: e8 51 96 6e ff              	callq	0x140d635e0 <.text+0xd625e0>
141679f8f: 4c 39 f8                    	cmpq	%r15, %rax
141679f92: 75 0a                       	jne	0x141679f9e <.text+0x1678f9e>
141679f94: 44 39 f2                    	cmpl	%r14d, %edx
141679f97: 72 0b                       	jb	0x141679fa4 <.text+0x1678fa4>
141679f99: e9 51 01 00 00              	jmp	0x14167a0ef <.text+0x16790ef>
141679f9e: 0f 87 4b 01 00 00           	ja	0x14167a0ef <.text+0x16790ef>
141679fa4: e8 d7 37 6d ff              	callq	0x140d4d780 <.text+0xd4c780>
141679fa9: 49 89 c6                    	movq	%rax, %r14
141679fac: 48 8b 00                    	movq	(%rax), %rax
141679faf: 48 83 f8 01                 	cmpq	$0x1, %rax
141679fb3: 75 78                       	jne	0x14167a02d <.text+0x167902d>
141679fb5: 49 83 c6 08                 	addq	$0x8, %r14
141679fb9: 4d 8b 3e                    	movq	(%r14), %r15
141679fbc: 49 c7 06 00 00 00 00        	movq	$0x0, (%r14)
141679fc3: 4d 85 ff                    	testq	%r15, %r15
141679fc6: 0f 84 86 00 00 00           	je	0x14167a052 <.text+0x1679052>
141679fcc: 4c 89 7d f0                 	movq	%r15, -0x10(%rbp)
141679fd0: 49 c7 47 18 00 00 00 00     	movq	$0x0, 0x18(%r15)
141679fd8: 49 c7 47 20 00 00 00 00     	movq	$0x0, 0x20(%r15)
141679fe0: 4c 89 65 d8                 	movq	%r12, -0x28(%rbp)
141679fe4: 48 89 75 e0                 	movq	%rsi, -0x20(%rbp)
141679fe8: 4c 89 6d e8                 	movq	%r13, -0x18(%rbp)
141679fec: c6 45 ff 01                 	movb	$0x1, -0x1(%rbp)
141679ff0: 48 89 f9                    	movq	%rdi, %rcx
141679ff3: 4c 89 fa                    	movq	%r15, %rdx
141679ff6: e8 15 07 00 00              	callq	0x14167a710 <.text+0x1679710>
141679ffb: 49 8b 06                    	movq	(%r14), %rax
141679ffe: 48 89 45 d8                 	movq	%rax, -0x28(%rbp)
14167a002: 4d 89 3e                    	movq	%r15, (%r14)
14167a005: 48 85 c0                    	testq	%rax, %rax
14167a008: 0f 84 b3 fd ff ff           	je	0x141679dc1 <.text+0x1678dc1>
14167a00e: 48 8b 45 d8                 	movq	-0x28(%rbp), %rax
14167a012: f0                          	lock
14167a013: 48 ff 08                    	decq	(%rax)
14167a016: 0f 85 a5 fd ff ff           	jne	0x141679dc1 <.text+0x1678dc1>
14167a01c: c6 45 ff 00                 	movb	$0x0, -0x1(%rbp)
14167a020: 48 89 f9                    	movq	%rdi, %rcx
14167a023: e8 f8 9b 6d ff              	callq	0x140d53c20 <.text+0xd52c20>
14167a028: e9 94 fd ff ff              	jmp	0x141679dc1 <.text+0x1678dc1>
14167a02d: 48 85 c0                    	testq	%rax, %rax
14167a030: 75 42                       	jne	0x14167a074 <.text+0x1679074>
14167a032: 4c 89 f1                    	movq	%r14, %rcx
14167a035: 31 d2                       	xorl	%edx, %edx
14167a037: e8 84 e5 27 01              	callq	0x1428f85c0 <.text+0x28f75c0>
14167a03c: 49 89 c6                    	movq	%rax, %r14
14167a03f: 4d 8b 3e                    	movq	(%r14), %r15
14167a042: 49 c7 06 00 00 00 00        	movq	$0x0, (%r14)
14167a049: 4d 85 ff                    	testq	%r15, %r15
14167a04c: 0f 85 7a ff ff ff           	jne	0x141679fcc <.text+0x1678fcc>
14167a052: e8 89 e2 25 01              	callq	0x1428d82e0 <.text+0x28d72e0>
14167a057: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14167a05b: 4c 89 65 d8                 	movq	%r12, -0x28(%rbp)
14167a05f: 48 89 75 e0                 	movq	%rsi, -0x20(%rbp)
14167a063: 4c 89 6d e8                 	movq	%r13, -0x18(%rbp)
14167a067: 48 89 f9                    	movq	%rdi, %rcx
14167a06a: 48 89 c2                    	movq	%rax, %rdx
14167a06d: e8 9e 06 00 00              	callq	0x14167a710 <.text+0x1679710>
14167a072: eb 20                       	jmp	0x14167a094 <.text+0x1679094>
14167a074: e8 67 e2 25 01              	callq	0x1428d82e0 <.text+0x28d72e0>
14167a079: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14167a07d: 4c 89 65 d8                 	movq	%r12, -0x28(%rbp)
14167a081: 48 89 75 e0                 	movq	%rsi, -0x20(%rbp)
14167a085: 4c 89 6d e8                 	movq	%r13, -0x18(%rbp)
14167a089: 48 89 f9                    	movq	%rdi, %rcx
14167a08c: 48 89 c2                    	movq	%rax, %rdx
14167a08f: e8 7c 06 00 00              	callq	0x14167a710 <.text+0x1679710>
14167a094: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14167a098: f0                          	lock
14167a099: 48 ff 08                    	decq	(%rax)
14167a09c: 0f 85 1f fd ff ff           	jne	0x141679dc1 <.text+0x1678dc1>
14167a0a2: 48 89 d9                    	movq	%rbx, %rcx
14167a0a5: e8 76 9b 6d ff              	callq	0x140d53c20 <.text+0xd52c20>
14167a0aa: e9 12 fd ff ff              	jmp	0x141679dc1 <.text+0x1678dc1>
14167a0af: 49 8d 04 d0                 	leaq	(%r8,%rdx,8), %rax
14167a0b3: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
14167a0b7: 48 03 8e 88 01 00 00        	addq	0x188(%rsi), %rcx
14167a0be: 48 89 4d a8                 	movq	%rcx, -0x58(%rbp)
14167a0c2: 49 89 0c d0                 	movq	%rcx, (%r8,%rdx,8)
14167a0c6: 48 81 c6 00 01 00 00        	addq	$0x100, %rsi            # imm = 0x100
14167a0cd: 48 89 f1                    	movq	%rsi, %rcx
14167a0d0: e8 9b 32 00 00              	callq	0x14167d370 <.text+0x167c370>
14167a0d5: b0 02                       	movb	$0x2, %al
14167a0d7: 48 81 c4 88 00 00 00        	addq	$0x88, %rsp
14167a0de: 5b                          	popq	%rbx
14167a0df: 5f                          	popq	%rdi
14167a0e0: 5e                          	popq	%rsi
14167a0e1: 41 5c                       	popq	%r12
14167a0e3: 41 5d                       	popq	%r13
14167a0e5: 41 5e                       	popq	%r14
14167a0e7: 41 5f                       	popq	%r15
14167a0e9: 5d                          	popq	%rbp
14167a0ea: c3                          	retq
14167a0eb: b0 01                       	movb	$0x1, %al
14167a0ed: eb e8                       	jmp	0x14167a0d7 <.text+0x16790d7>
14167a0ef: 31 c0                       	xorl	%eax, %eax
14167a0f1: eb e4                       	jmp	0x14167a0d7 <.text+0x16790d7>
14167a0f3: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
