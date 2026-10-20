
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14233ba40: 41 57                       	pushq	%r15
14233ba42: 41 56                       	pushq	%r14
14233ba44: 41 55                       	pushq	%r13
14233ba46: 41 54                       	pushq	%r12
14233ba48: 56                          	pushq	%rsi
14233ba49: 57                          	pushq	%rdi
14233ba4a: 55                          	pushq	%rbp
14233ba4b: 53                          	pushq	%rbx
14233ba4c: 48 81 ec 78 01 00 00        	subq	$0x178, %rsp            # imm = 0x178
14233ba53: 44 0f 29 8c 24 60 01 00 00  	movaps	%xmm9, 0x160(%rsp)
14233ba5c: 44 0f 29 84 24 50 01 00 00  	movaps	%xmm8, 0x150(%rsp)
14233ba65: 0f 29 bc 24 40 01 00 00     	movaps	%xmm7, 0x140(%rsp)
14233ba6d: 0f 29 b4 24 30 01 00 00     	movaps	%xmm6, 0x130(%rsp)
14233ba75: 48 8b 19                    	movq	(%rcx), %rbx
14233ba78: 4c 8b 71 08                 	movq	0x8(%rcx), %r14
14233ba7c: 44 8b 49 10                 	movl	0x10(%rcx), %r9d
14233ba80: 8b 41 14                    	movl	0x14(%rcx), %eax
14233ba83: 49 8d b6 a0 01 00 00        	leaq	0x1a0(%r14), %rsi
14233ba8a: 49 8d be f8 00 00 00        	leaq	0xf8(%r14), %rdi
14233ba91: 48 8d 8c 24 90 00 00 00     	leaq	0x90(%rsp), %rcx
14233ba99: 89 44 24 20                 	movl	%eax, 0x20(%rsp)
14233ba9d: 4c 89 f2                    	movq	%r14, %rdx
14233baa0: 49 89 d8                    	movq	%rbx, %r8
14233baa3: e8 08 79 2e 00              	callq	0x1426233b0 <.text+0x26223b0>
14233baa8: 4c 89 74 24 50              	movq	%r14, 0x50(%rsp)
14233baad: 4c 89 74 24 70              	movq	%r14, 0x70(%rsp)
14233bab2: 48 89 74 24 78              	movq	%rsi, 0x78(%rsp)
14233bab7: 48 89 bc 24 80 00 00 00     	movq	%rdi, 0x80(%rsp)
14233babf: 48 89 5c 24 68              	movq	%rbx, 0x68(%rsp)
14233bac4: 48 89 9c 24 88 00 00 00     	movq	%rbx, 0x88(%rsp)
14233bacc: 4c 8b b4 24 90 00 00 00     	movq	0x90(%rsp), %r14
14233bad4: 4c 8b bc 24 98 00 00 00     	movq	0x98(%rsp), %r15
14233badc: 48 8b 84 24 b0 00 00 00     	movq	0xb0(%rsp), %rax
14233bae4: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
14233bae9: 48 8b 9c 24 c0 00 00 00     	movq	0xc0(%rsp), %rbx
14233baf1: 48 8b bc 24 c8 00 00 00     	movq	0xc8(%rsp), %rdi
14233baf9: 0f b6 84 24 d8 00 00 00     	movzbl	0xd8(%rsp), %eax
14233bb01: 89 44 24 3c                 	movl	%eax, 0x3c(%rsp)
14233bb05: 0f b6 84 24 e8 00 00 00     	movzbl	0xe8(%rsp), %eax
14233bb0d: 89 44 24 38                 	movl	%eax, 0x38(%rsp)
14233bb11: 48 8b 84 24 f0 00 00 00     	movq	0xf0(%rsp), %rax
14233bb19: 48 89 44 24 40              	movq	%rax, 0x40(%rsp)
14233bb1e: 48 8b ac 24 f8 00 00 00     	movq	0xf8(%rsp), %rbp
14233bb26: 4c 8b a4 24 08 01 00 00     	movq	0x108(%rsp), %r12
14233bb2e: 8b 84 24 14 01 00 00        	movl	0x114(%rsp), %eax
14233bb35: 89 44 24 5c                 	movl	%eax, 0x5c(%rsp)
14233bb39: 48 8b 84 24 18 01 00 00     	movq	0x118(%rsp), %rax
14233bb41: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14233bb46: 0f b6 84 24 20 01 00 00     	movzbl	0x120(%rsp), %eax
14233bb4e: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
14233bb52: 0f b6 84 24 28 01 00 00     	movzbl	0x128(%rsp), %eax
14233bb5a: 88 44 24 37                 	movb	%al, 0x37(%rsp)
14233bb5e: f3 0f 10 35 c2 15 69 00     	movss	0x6915c2(%rip), %xmm6   # 0x1429cd128
14233bb66: f3 0f 10 3d 62 9d 5e 00     	movss	0x5e9d62(%rip), %xmm7   # 0x1429258d0
14233bb6e: f3 44 0f 10 05 8d 9d 5e 00  	movss	0x5e9d8d(%rip), %xmm8   # 0x142925904
14233bb77: eb 41                       	jmp	0x14233bbba <.text+0x233abba>
14233bb79: 0f 1f 80 00 00 00 00        	nopl	(%rax)
14233bb80: 48 ff c7                    	incq	%rdi
14233bb83: f3 0f 10 48 04              	movss	0x4(%rax), %xmm1
14233bb88: 48 8d 0d f9 21 d1 00        	leaq	0xd121f9(%rip), %rcx    # 0x14304dd88
14233bb8f: e8 5c a3 95 fe              	callq	0x140c95ef0 <.text+0xc94ef0>
14233bb94: 41 0f 28 c8                 	movaps	%xmm8, %xmm1
14233bb98: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14233bb9c: f3 44 0f 59 c8              	mulss	%xmm0, %xmm9
14233bba1: f3 44 0f 58 c9              	addss	%xmm1, %xmm9
14233bba6: 8b 44 24 5c                 	movl	0x5c(%rsp), %eax
14233bbaa: 41 89 45 00                 	movl	%eax, (%r13)
14233bbae: f3 44 0f 59 4e 10           	mulss	0x10(%rsi), %xmm9
14233bbb4: f3 44 0f 11 4e 10           	movss	%xmm9, 0x10(%rsi)
14233bbba: f6 44 24 37 01              	testb	$0x1, 0x37(%rsp)
14233bbbf: 74 4f                       	je	0x14233bc10 <.text+0x233ac10>
14233bbc1: 48 39 df                    	cmpq	%rbx, %rdi
14233bbc4: 0f 85 66 02 00 00           	jne	0x14233be30 <.text+0x233ae30>
14233bbca: 4d 39 fe                    	cmpq	%r15, %r14
14233bbcd: 4c 8b 4c 24 68              	movq	0x68(%rsp), %r9
14233bbd2: 0f 84 04 04 00 00           	je	0x14233bfdc <.text+0x233afdc>
14233bbd8: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
14233bbdd: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
14233bbe4: 66 66 66 2e 0f 1f 84 00 00 00 00 00 	nopw	%cs:(%rax,%rax)
14233bbf0: 41 8b 0e                    	movl	(%r14), %ecx
14233bbf3: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14233bbf7: 48 8b 5c c8 10              	movq	0x10(%rax,%rcx,8), %rbx
14233bbfc: 48 85 db                    	testq	%rbx, %rbx
14233bbff: 75 66                       	jne	0x14233bc67 <.text+0x233ac67>
14233bc01: 49 83 c6 04                 	addq	$0x4, %r14
14233bc05: 4d 39 fe                    	cmpq	%r15, %r14
14233bc08: 75 e6                       	jne	0x14233bbf0 <.text+0x233abf0>
14233bc0a: e9 cd 03 00 00              	jmp	0x14233bfdc <.text+0x233afdc>
14233bc0f: 90                          	nop
14233bc10: 48 39 df                    	cmpq	%rbx, %rdi
14233bc13: 0f 85 16 03 00 00           	jne	0x14233bf2f <.text+0x233af2f>
14233bc19: 4d 39 fe                    	cmpq	%r15, %r14
14233bc1c: 48 8b 74 24 68              	movq	0x68(%rsp), %rsi
14233bc21: 0f 84 b5 03 00 00           	je	0x14233bfdc <.text+0x233afdc>
14233bc27: 48 8b 44 24 50              	movq	0x50(%rsp), %rax
14233bc2c: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
14233bc33: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233bc40: 41 8b 0e                    	movl	(%r14), %ecx
14233bc43: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233bc47: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233bc4b: 48 8b 5c 08 58              	movq	0x58(%rax,%rcx), %rbx
14233bc50: 48 85 db                    	testq	%rbx, %rbx
14233bc53: 0f 85 8b 00 00 00           	jne	0x14233bce4 <.text+0x233ace4>
14233bc59: 49 83 c6 04                 	addq	$0x4, %r14
14233bc5d: 4d 39 fe                    	cmpq	%r15, %r14
14233bc60: 75 de                       	jne	0x14233bc40 <.text+0x233ac40>
14233bc62: e9 75 03 00 00              	jmp	0x14233bfdc <.text+0x233afdc>
14233bc67: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
14233bc6b: 4d 8b 81 10 01 00 00        	movq	0x110(%r9), %r8
14233bc72: 48 8b 41 38                 	movq	0x38(%rcx), %rax
14233bc76: 48 8b 51 40                 	movq	0x40(%rcx), %rdx
14233bc7a: 4c 39 c2                    	cmpq	%r8, %rdx
14233bc7d: 0f 86 0a 01 00 00           	jbe	0x14233bd8d <.text+0x233ad8d>
14233bc83: 4e 8b 04 c0                 	movq	(%rax,%r8,8), %r8
14233bc87: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14233bc8b: 4d 85 c0                    	testq	%r8, %r8
14233bc8e: 0f 84 2a 03 00 00           	je	0x14233bfbe <.text+0x233afbe>
14233bc94: 49 f7 d0                    	notq	%r8
14233bc97: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14233bc9b: 49 c1 e0 04                 	shlq	$0x4, %r8
14233bc9f: 4e 8b 44 01 10              	movq	0x10(%rcx,%r8), %r8
14233bca4: 4c 89 44 24 40              	movq	%r8, 0x40(%rsp)
14233bca9: 4d 8b 81 20 01 00 00        	movq	0x120(%r9), %r8
14233bcb0: 4c 39 c2                    	cmpq	%r8, %rdx
14233bcb3: 0f 87 f1 00 00 00           	ja	0x14233bdaa <.text+0x233adaa>
14233bcb9: c7 44 24 3c 00 00 00 00     	movl	$0x0, 0x3c(%rsp)
14233bcc1: 4d 8b 81 28 01 00 00        	movq	0x128(%r9), %r8
14233bcc8: 4c 39 c2                    	cmpq	%r8, %rdx
14233bccb: 0f 86 f7 00 00 00           	jbe	0x14233bdc8 <.text+0x233adc8>
14233bcd1: 4a 83 3c c0 00              	cmpq	$0x0, (%rax,%r8,8)
14233bcd6: 41 0f 95 c0                 	setne	%r8b
14233bcda: 44 89 44 24 38              	movl	%r8d, 0x38(%rsp)
14233bcdf: e9 ec 00 00 00              	jmp	0x14233bdd0 <.text+0x233add0>
14233bce4: 48 01 c8                    	addq	%rcx, %rax
14233bce7: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14233bced: 48 8b 54 24 50              	movq	0x50(%rsp), %rdx
14233bcf2: 4c 8b 82 a8 01 00 00        	movq	0x1a8(%rdx), %r8
14233bcf9: 4c 8d 14 c9                 	leaq	(%rcx,%rcx,8), %r10
14233bcfd: 4b 8d 14 d0                 	leaq	(%r8,%r10,8), %rdx
14233bd01: 4c 8b 8e 10 01 00 00        	movq	0x110(%rsi), %r9
14233bd08: 4b 8b 4c d0 38              	movq	0x38(%r8,%r10,8), %rcx
14233bd0d: 4f 8b 44 d0 40              	movq	0x40(%r8,%r10,8), %r8
14233bd12: 4d 39 c8                    	cmpq	%r9, %r8
14233bd15: 0f 86 2d 01 00 00           	jbe	0x14233be48 <.text+0x233ae48>
14233bd1b: 4e 8b 0c c9                 	movq	(%rcx,%r9,8), %r9
14233bd1f: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14233bd23: 4d 85 c9                    	testq	%r9, %r9
14233bd26: 0f 84 20 01 00 00           	je	0x14233be4c <.text+0x233ae4c>
14233bd2c: 49 f7 d1                    	notq	%r9
14233bd2f: 4f 8d 0c 49                 	leaq	(%r9,%r9,2), %r9
14233bd33: 49 c1 e1 04                 	shlq	$0x4, %r9
14233bd37: 4e 8b 4c 0a 10              	movq	0x10(%rdx,%r9), %r9
14233bd3c: 4c 89 4c 24 40              	movq	%r9, 0x40(%rsp)
14233bd41: 4c 8b 96 20 01 00 00        	movq	0x120(%rsi), %r10
14233bd48: 4c 8b 88 88 00 00 00        	movq	0x88(%rax), %r9
14233bd4f: 4d 39 d1                    	cmpq	%r10, %r9
14233bd52: 0f 87 14 01 00 00           	ja	0x14233be6c <.text+0x233ae6c>
14233bd58: c7 44 24 3c 00 00 00 00     	movl	$0x0, 0x3c(%rsp)
14233bd60: 4c 8b 96 28 01 00 00        	movq	0x128(%rsi), %r10
14233bd67: 4d 39 d1                    	cmpq	%r10, %r9
14233bd6a: 0f 87 21 01 00 00           	ja	0x14233be91 <.text+0x233ae91>
14233bd70: c7 44 24 38 00 00 00 00     	movl	$0x0, 0x38(%rsp)
14233bd78: 4c 8b 96 30 01 00 00        	movq	0x130(%rsi), %r10
14233bd7f: 4d 39 d1                    	cmpq	%r10, %r9
14233bd82: 0f 87 2a 01 00 00           	ja	0x14233beb2 <.text+0x233aeb2>
14233bd88: e9 60 01 00 00              	jmp	0x14233beed <.text+0x233aeed>
14233bd8d: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14233bd91: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
14233bd9a: 4d 8b 81 20 01 00 00        	movq	0x120(%r9), %r8
14233bda1: 4c 39 c2                    	cmpq	%r8, %rdx
14233bda4: 0f 86 0f ff ff ff           	jbe	0x14233bcb9 <.text+0x233acb9>
14233bdaa: 4a 83 3c c0 00              	cmpq	$0x0, (%rax,%r8,8)
14233bdaf: 41 0f 95 c0                 	setne	%r8b
14233bdb3: 44 89 44 24 3c              	movl	%r8d, 0x3c(%rsp)
14233bdb8: 4d 8b 81 28 01 00 00        	movq	0x128(%r9), %r8
14233bdbf: 4c 39 c2                    	cmpq	%r8, %rdx
14233bdc2: 0f 87 09 ff ff ff           	ja	0x14233bcd1 <.text+0x233acd1>
14233bdc8: c7 44 24 38 00 00 00 00     	movl	$0x0, 0x38(%rsp)
14233bdd0: 49 83 c6 04                 	addq	$0x4, %r14
14233bdd4: 4d 8b 81 18 01 00 00        	movq	0x118(%r9), %r8
14233bddb: 4e 8b 04 c0                 	movq	(%rax,%r8,8), %r8
14233bddf: 49 f7 d0                    	notq	%r8
14233bde2: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14233bde6: 49 c1 e0 04                 	shlq	$0x4, %r8
14233bdea: 4a 8b 6c 01 10              	movq	0x10(%rcx,%r8), %rbp
14233bdef: 4e 8b 64 01 28              	movq	0x28(%rcx,%r8), %r12
14233bdf4: 4d 8b 81 30 01 00 00        	movq	0x130(%r9), %r8
14233bdfb: 4c 39 c2                    	cmpq	%r8, %rdx
14233bdfe: 76 37                       	jbe	0x14233be37 <.text+0x233ae37>
14233be00: 4a 8b 04 c0                 	movq	(%rax,%r8,8), %rax
14233be04: 48 85 c0                    	testq	%rax, %rax
14233be07: 74 2e                       	je	0x14233be37 <.text+0x233ae37>
14233be09: 48 f7 d0                    	notq	%rax
14233be0c: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14233be10: 48 c1 e0 04                 	shlq	$0x4, %rax
14233be14: 48 8b 44 01 10              	movq	0x10(%rcx,%rax), %rax
14233be19: 48 89 44 24 48              	movq	%rax, 0x48(%rsp)
14233be1e: b0 01                       	movb	$0x1, %al
14233be20: 89 44 24 30                 	movl	%eax, 0x30(%rsp)
14233be24: 31 ff                       	xorl	%edi, %edi
14233be26: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14233be30: 89 f9                       	movl	%edi, %ecx
14233be32: e9 08 01 00 00              	jmp	0x14233bf3f <.text+0x233af3f>
14233be37: 31 ff                       	xorl	%edi, %edi
14233be39: c7 44 24 30 00 00 00 00     	movl	$0x0, 0x30(%rsp)
14233be41: 89 f9                       	movl	%edi, %ecx
14233be43: e9 f7 00 00 00              	jmp	0x14233bf3f <.text+0x233af3f>
14233be48: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14233be4c: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
14233be55: 4c 8b 96 20 01 00 00        	movq	0x120(%rsi), %r10
14233be5c: 4c 8b 88 88 00 00 00        	movq	0x88(%rax), %r9
14233be63: 4d 39 d1                    	cmpq	%r10, %r9
14233be66: 0f 86 ec fe ff ff           	jbe	0x14233bd58 <.text+0x233ad58>
14233be6c: 4c 8b 98 80 00 00 00        	movq	0x80(%rax), %r11
14233be73: 4b 83 3c d3 00              	cmpq	$0x0, (%r11,%r10,8)
14233be78: 41 0f 95 c2                 	setne	%r10b
14233be7c: 44 89 54 24 3c              	movl	%r10d, 0x3c(%rsp)
14233be81: 4c 8b 96 28 01 00 00        	movq	0x128(%rsi), %r10
14233be88: 4d 39 d1                    	cmpq	%r10, %r9
14233be8b: 0f 86 df fe ff ff           	jbe	0x14233bd70 <.text+0x233ad70>
14233be91: 4c 8b 98 80 00 00 00        	movq	0x80(%rax), %r11
14233be98: 4b 83 3c d3 00              	cmpq	$0x0, (%r11,%r10,8)
14233be9d: 41 0f 95 c2                 	setne	%r10b
14233bea1: 44 89 54 24 38              	movl	%r10d, 0x38(%rsp)
14233bea6: 4c 8b 96 30 01 00 00        	movq	0x130(%rsi), %r10
14233bead: 4d 39 d1                    	cmpq	%r10, %r9
14233beb0: 76 3b                       	jbe	0x14233beed <.text+0x233aeed>
14233beb2: 4c 8b 88 80 00 00 00        	movq	0x80(%rax), %r9
14233beb9: 4b 83 3c d1 00              	cmpq	$0x0, (%r9,%r10,8)
14233bebe: 74 2d                       	je	0x14233beed <.text+0x233aeed>
14233bec0: 41 b1 01                    	movb	$0x1, %r9b
14233bec3: 44 89 4c 24 30              	movl	%r9d, 0x30(%rsp)
14233bec8: 4d 39 d0                    	cmpq	%r10, %r8
14233becb: 76 2a                       	jbe	0x14233bef7 <.text+0x233aef7>
14233becd: 4e 8b 04 d1                 	movq	(%rcx,%r10,8), %r8
14233bed1: 4d 85 c0                    	testq	%r8, %r8
14233bed4: 74 21                       	je	0x14233bef7 <.text+0x233aef7>
14233bed6: 49 f7 d0                    	notq	%r8
14233bed9: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14233bedd: 49 c1 e0 04                 	shlq	$0x4, %r8
14233bee1: 4e 8b 44 02 10              	movq	0x10(%rdx,%r8), %r8
14233bee6: 4c 89 44 24 48              	movq	%r8, 0x48(%rsp)
14233beeb: eb 13                       	jmp	0x14233bf00 <.text+0x233af00>
14233beed: c7 44 24 30 00 00 00 00     	movl	$0x0, 0x30(%rsp)
14233bef5: eb 09                       	jmp	0x14233bf00 <.text+0x233af00>
14233bef7: 48 c7 44 24 48 00 00 00 00  	movq	$0x0, 0x48(%rsp)
14233bf00: 49 83 c6 04                 	addq	$0x4, %r14
14233bf04: 4c 8b 86 18 01 00 00        	movq	0x118(%rsi), %r8
14233bf0b: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
14233bf0f: 48 f7 d1                    	notq	%rcx
14233bf12: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233bf16: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233bf1a: 48 8b 6c 0a 10              	movq	0x10(%rdx,%rcx), %rbp
14233bf1f: 4c 8b 64 0a 28              	movq	0x28(%rdx,%rcx), %r12
14233bf24: 48 8b 40 50                 	movq	0x50(%rax), %rax
14233bf28: 48 89 44 24 60              	movq	%rax, 0x60(%rsp)
14233bf2d: 31 ff                       	xorl	%edi, %edi
14233bf2f: 48 89 f8                    	movq	%rdi, %rax
14233bf32: 48 c1 e0 04                 	shlq	$0x4, %rax
14233bf36: 48 8b 4c 24 60              	movq	0x60(%rsp), %rcx
14233bf3b: 8b 4c 01 08                 	movl	0x8(%rcx,%rax), %ecx
14233bf3f: 48 89 c8                    	movq	%rcx, %rax
14233bf42: 48 c1 e0 04                 	shlq	$0x4, %rax
14233bf46: 48 03 44 24 40              	addq	0x40(%rsp), %rax
14233bf4b: 48 8d 14 89                 	leaq	(%rcx,%rcx,4), %rdx
14233bf4f: 48 8d 34 95 00 00 00 00     	leaq	(,%rdx,4), %rsi
14233bf57: 48 01 ee                    	addq	%rbp, %rsi
14233bf5a: 4d 8d 2c 8c                 	leaq	(%r12,%rcx,4), %r13
14233bf5e: f6 44 24 30 01              	testb	$0x1, 0x30(%rsp)
14233bf63: 74 3b                       	je	0x14233bfa0 <.text+0x233afa0>
14233bf65: 48 8b 54 24 48              	movq	0x48(%rsp), %rdx
14233bf6a: 48 8d 0c ca                 	leaq	(%rdx,%rcx,8), %rcx
14233bf6e: 8b 54 24 38                 	movl	0x38(%rsp), %edx
14233bf72: 0a 54 24 3c                 	orb	0x3c(%rsp), %dl
14233bf76: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
14233bf7a: f6 c2 01                    	testb	$0x1, %dl
14233bf7d: 75 04                       	jne	0x14233bf83 <.text+0x233af83>
14233bf7f: 44 0f 28 cf                 	movaps	%xmm7, %xmm9
14233bf83: 48 85 c9                    	testq	%rcx, %rcx
14233bf86: 0f 84 f4 fb ff ff           	je	0x14233bb80 <.text+0x233ab80>
14233bf8c: 83 39 01                    	cmpl	$0x1, (%rcx)
14233bf8f: 0f 85 eb fb ff ff           	jne	0x14233bb80 <.text+0x233ab80>
14233bf95: f3 44 0f 10 49 04           	movss	0x4(%rcx), %xmm9
14233bf9b: e9 e0 fb ff ff              	jmp	0x14233bb80 <.text+0x233ab80>
14233bfa0: 8b 4c 24 38                 	movl	0x38(%rsp), %ecx
14233bfa4: 0a 4c 24 3c                 	orb	0x3c(%rsp), %cl
14233bfa8: 44 0f 28 ce                 	movaps	%xmm6, %xmm9
14233bfac: f6 c1 01                    	testb	$0x1, %cl
14233bfaf: 0f 85 cb fb ff ff           	jne	0x14233bb80 <.text+0x233ab80>
14233bfb5: 44 0f 28 cf                 	movaps	%xmm7, %xmm9
14233bfb9: e9 c2 fb ff ff              	jmp	0x14233bb80 <.text+0x233ab80>
14233bfbe: 48 c7 44 24 40 00 00 00 00  	movq	$0x0, 0x40(%rsp)
14233bfc7: 4d 8b 81 20 01 00 00        	movq	0x120(%r9), %r8
14233bfce: 4c 39 c2                    	cmpq	%r8, %rdx
14233bfd1: 0f 87 d3 fd ff ff           	ja	0x14233bdaa <.text+0x233adaa>
14233bfd7: e9 dd fc ff ff              	jmp	0x14233bcb9 <.text+0x233acb9>
14233bfdc: 0f 28 b4 24 30 01 00 00     	movaps	0x130(%rsp), %xmm6
14233bfe4: 0f 28 bc 24 40 01 00 00     	movaps	0x140(%rsp), %xmm7
14233bfec: 44 0f 28 84 24 50 01 00 00  	movaps	0x150(%rsp), %xmm8
14233bff5: 44 0f 28 8c 24 60 01 00 00  	movaps	0x160(%rsp), %xmm9
14233bffe: 48 81 c4 78 01 00 00        	addq	$0x178, %rsp            # imm = 0x178
14233c005: 5b                          	popq	%rbx
14233c006: 5d                          	popq	%rbp
14233c007: 5f                          	popq	%rdi
14233c008: 5e                          	popq	%rsi
14233c009: 41 5c                       	popq	%r12
14233c00b: 41 5d                       	popq	%r13
14233c00d: 41 5e                       	popq	%r14
14233c00f: 41 5f                       	popq	%r15
14233c011: c3                          	retq
14233c012: cc                          	int3
14233c013: cc                          	int3
14233c014: cc                          	int3
14233c015: cc                          	int3
14233c016: cc                          	int3
14233c017: cc                          	int3
14233c018: cc                          	int3
14233c019: cc                          	int3
14233c01a: cc                          	int3
14233c01b: cc                          	int3
14233c01c: cc                          	int3
14233c01d: cc                          	int3
14233c01e: cc                          	int3
14233c01f: cc                          	int3
14233c020: 41 57                       	pushq	%r15
14233c022: 41 56                       	pushq	%r14
14233c024: 41 55                       	pushq	%r13
14233c026: 41 54                       	pushq	%r12
14233c028: 56                          	pushq	%rsi
14233c029: 57                          	pushq	%rdi
14233c02a: 55                          	pushq	%rbp
14233c02b: 53                          	pushq	%rbx
14233c02c: 48 83 ec 68                 	subq	$0x68, %rsp
14233c030: 0f 29 7c 24 50              	movaps	%xmm7, 0x50(%rsp)
14233c035: 0f 29 74 24 40              	movaps	%xmm6, 0x40(%rsp)
14233c03a: 48 8b 11                    	movq	(%rcx), %rdx
14233c03d: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14233c041: 48 89 44 24 30              	movq	%rax, 0x30(%rsp)
14233c046: 8b 41 14                    	movl	0x14(%rcx), %eax
14233c049: 89 44 24 24                 	movl	%eax, 0x24(%rsp)
14233c04d: 4c 8b b2 f8 00 00 00        	movq	0xf8(%rdx), %r14
14233c054: 48 8b 82 00 01 00 00        	movq	0x100(%rdx), %rax
14233c05b: 4d 8d 3c 86                 	leaq	(%r14,%rax,4), %r15
14233c05f: 48 89 54 24 28              	movq	%rdx, 0x28(%rsp)
14233c064: 80 ba 08 01 00 00 00        	cmpb	$0x0, 0x108(%rdx)
14233c06b: 74 1a                       	je	0x14233c087 <.text+0x233b087>
14233c06d: 31 db                       	xorl	%ebx, %ebx
14233c06f: f3 0f 10 35 8d 98 5e 00     	movss	0x5e988d(%rip), %xmm6   # 0x142925904
14233c077: 0f 57 ff                    	xorps	%xmm7, %xmm7
14233c07a: 45 31 ed                    	xorl	%r13d, %r13d
14233c07d: 45 31 e4                    	xorl	%r12d, %r12d
14233c080: 31 ff                       	xorl	%edi, %edi
14233c082: e9 97 00 00 00              	jmp	0x14233c11e <.text+0x233b11e>
14233c087: bb 08 00 00 00              	movl	$0x8, %ebx
14233c08c: 31 f6                       	xorl	%esi, %esi
14233c08e: f3 0f 10 35 6e 98 5e 00     	movss	0x5e986e(%rip), %xmm6   # 0x142925904
14233c096: 0f 57 ff                    	xorps	%xmm7, %xmm7
14233c099: 45 31 ed                    	xorl	%r13d, %r13d
14233c09c: 45 31 e4                    	xorl	%r12d, %r12d
14233c09f: 31 ff                       	xorl	%edi, %edi
14233c0a1: e9 93 01 00 00              	jmp	0x14233c239 <.text+0x233b239>
14233c0a6: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14233c0aa: 45 31 ed                    	xorl	%r13d, %r13d
14233c0ad: 48 8b 54 24 28              	movq	0x28(%rsp), %rdx
14233c0b2: 48 8b 92 18 01 00 00        	movq	0x118(%rdx), %rdx
14233c0b9: 49 83 c6 04                 	addq	$0x4, %r14
14233c0bd: 48 8b 04 d0                 	movq	(%rax,%rdx,8), %rax
14233c0c1: 48 f7 d0                    	notq	%rax
14233c0c4: 48 8d 04 40                 	leaq	(%rax,%rax,2), %rax
14233c0c8: 48 c1 e0 04                 	shlq	$0x4, %rax
14233c0cc: 48 8b 5c 01 10              	movq	0x10(%rcx,%rax), %rbx
14233c0d1: 48 8b 6c 01 28              	movq	0x28(%rcx,%rax), %rbp
14233c0d6: 31 ff                       	xorl	%edi, %edi
14233c0d8: 89 fe                       	movl	%edi, %esi
14233c0da: 48 89 f0                    	movq	%rsi, %rax
14233c0dd: 48 c1 e0 04                 	shlq	$0x4, %rax
14233c0e1: 48 ff c7                    	incq	%rdi
14233c0e4: f3 41 0f 10 4c 05 00        	movss	(%r13,%rax), %xmm1
14233c0eb: 48 8d 0d 96 1c d1 00        	leaq	0xd11c96(%rip), %rcx    # 0x14304dd88
14233c0f2: e8 f9 9d 95 fe              	callq	0x140c95ef0 <.text+0xc94ef0>
14233c0f7: 0f 28 ce                    	movaps	%xmm6, %xmm1
14233c0fa: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14233c0fe: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
14233c102: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14233c106: 8b 44 24 24                 	movl	0x24(%rsp), %eax
14233c10a: 89 44 b5 00                 	movl	%eax, (%rbp,%rsi,4)
14233c10e: 48 8d 04 b6                 	leaq	(%rsi,%rsi,4), %rax
14233c112: f3 0f 59 4c 83 10           	mulss	0x10(%rbx,%rax,4), %xmm1
14233c118: f3 0f 11 4c 83 10           	movss	%xmm1, 0x10(%rbx,%rax,4)
14233c11e: 4c 39 e7                    	cmpq	%r12, %rdi
14233c121: 75 b5                       	jne	0x14233c0d8 <.text+0x233b0d8>
14233c123: 4d 39 fe                    	cmpq	%r15, %r14
14233c126: 0f 84 b3 01 00 00           	je	0x14233c2df <.text+0x233b2df>
14233c12c: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14233c131: 48 8b 80 a8 01 00 00        	movq	0x1a8(%rax), %rax
14233c138: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14233c140: 41 8b 0e                    	movl	(%r14), %ecx
14233c143: 48 8d 0c c9                 	leaq	(%rcx,%rcx,8), %rcx
14233c147: 4c 8b 64 c8 10              	movq	0x10(%rax,%rcx,8), %r12
14233c14c: 4d 85 e4                    	testq	%r12, %r12
14233c14f: 75 0f                       	jne	0x14233c160 <.text+0x233b160>
14233c151: 49 83 c6 04                 	addq	$0x4, %r14
14233c155: 4d 39 fe                    	cmpq	%r15, %r14
14233c158: 75 e6                       	jne	0x14233c140 <.text+0x233b140>
14233c15a: e9 80 01 00 00              	jmp	0x14233c2df <.text+0x233b2df>
14233c15f: 90                          	nop
14233c160: 48 8d 0c c8                 	leaq	(%rax,%rcx,8), %rcx
14233c164: 48 8b 44 24 28              	movq	0x28(%rsp), %rax
14233c169: 48 8b 90 10 01 00 00        	movq	0x110(%rax), %rdx
14233c170: 48 8b 41 38                 	movq	0x38(%rcx), %rax
14233c174: 48 39 51 40                 	cmpq	%rdx, 0x40(%rcx)
14233c178: 0f 86 28 ff ff ff           	jbe	0x14233c0a6 <.text+0x233b0a6>
14233c17e: 48 8b 14 d0                 	movq	(%rax,%rdx,8), %rdx
14233c182: 48 8b 49 18                 	movq	0x18(%rcx), %rcx
14233c186: 48 85 d2                    	testq	%rdx, %rdx
14233c189: 0f 84 1b ff ff ff           	je	0x14233c0aa <.text+0x233b0aa>
14233c18f: 48 f7 d2                    	notq	%rdx
14233c192: 48 8d 14 52                 	leaq	(%rdx,%rdx,2), %rdx
14233c196: 48 c1 e2 04                 	shlq	$0x4, %rdx
14233c19a: 4c 8b 6c 11 10              	movq	0x10(%rcx,%rdx), %r13
14233c19f: e9 09 ff ff ff              	jmp	0x14233c0ad <.text+0x233b0ad>
14233c1a4: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14233c1a8: 45 31 ed                    	xorl	%r13d, %r13d
14233c1ab: 4c 8b 44 24 28              	movq	0x28(%rsp), %r8
14233c1b0: 4d 8b 80 18 01 00 00        	movq	0x118(%r8), %r8
14233c1b7: 49 83 c6 04                 	addq	$0x4, %r14
14233c1bb: 4a 8b 0c c1                 	movq	(%rcx,%r8,8), %rcx
14233c1bf: 48 f7 d1                    	notq	%rcx
14233c1c2: 48 8d 0c 49                 	leaq	(%rcx,%rcx,2), %rcx
14233c1c6: 48 c1 e1 04                 	shlq	$0x4, %rcx
14233c1ca: 48 8b 74 0a 10              	movq	0x10(%rdx,%rcx), %rsi
14233c1cf: 48 8b 4c 0a 28              	movq	0x28(%rdx,%rcx), %rcx
14233c1d4: 48 89 4c 24 38              	movq	%rcx, 0x38(%rsp)
14233c1d9: 48 8b 58 50                 	movq	0x50(%rax), %rbx
14233c1dd: 31 ff                       	xorl	%edi, %edi
14233c1df: 48 89 f8                    	movq	%rdi, %rax
14233c1e2: 48 c1 e0 04                 	shlq	$0x4, %rax
14233c1e6: 8b 6c 03 08                 	movl	0x8(%rbx,%rax), %ebp
14233c1ea: 48 89 e8                    	movq	%rbp, %rax
14233c1ed: 48 c1 e0 04                 	shlq	$0x4, %rax
14233c1f1: 48 ff c7                    	incq	%rdi
14233c1f4: f3 41 0f 10 4c 05 00        	movss	(%r13,%rax), %xmm1
14233c1fb: 48 8d 0d 86 1b d1 00        	leaq	0xd11b86(%rip), %rcx    # 0x14304dd88
14233c202: e8 e9 9c 95 fe              	callq	0x140c95ef0 <.text+0xc94ef0>
14233c207: 0f 28 ce                    	movaps	%xmm6, %xmm1
14233c20a: f3 0f 5c c8                 	subss	%xmm0, %xmm1
14233c20e: f3 0f 59 cf                 	mulss	%xmm7, %xmm1
14233c212: f3 0f 58 c8                 	addss	%xmm0, %xmm1
14233c216: 8b 44 24 24                 	movl	0x24(%rsp), %eax
14233c21a: 48 8b 4c 24 38              	movq	0x38(%rsp), %rcx
14233c21f: 89 04 a9                    	movl	%eax, (%rcx,%rbp,4)
14233c222: 48 8d 04 ad 00 00 00 00     	leaq	(,%rbp,4), %rax
14233c22a: 48 01 e8                    	addq	%rbp, %rax
14233c22d: f3 0f 59 4c 86 10           	mulss	0x10(%rsi,%rax,4), %xmm1
14233c233: f3 0f 11 4c 86 10           	movss	%xmm1, 0x10(%rsi,%rax,4)
14233c239: 4c 39 e7                    	cmpq	%r12, %rdi
14233c23c: 75 a1                       	jne	0x14233c1df <.text+0x233b1df>
14233c23e: 4d 39 fe                    	cmpq	%r15, %r14
14233c241: 0f 84 98 00 00 00           	je	0x14233c2df <.text+0x233b2df>
14233c247: 48 8b 44 24 30              	movq	0x30(%rsp), %rax
14233c24c: 48 8b 80 00 01 00 00        	movq	0x100(%rax), %rax
14233c253: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
14233c260: 41 8b 0e                    	movl	(%r14), %ecx
14233c263: 48 8d 0c 89                 	leaq	(%rcx,%rcx,4), %rcx
14233c267: 48 c1 e1 05                 	shlq	$0x5, %rcx
14233c26b: 4c 8b 64 08 58              	movq	0x58(%rax,%rcx), %r12
14233c270: 4d 85 e4                    	testq	%r12, %r12
14233c273: 75 0b                       	jne	0x14233c280 <.text+0x233b280>
14233c275: 49 83 c6 04                 	addq	$0x4, %r14
14233c279: 4d 39 fe                    	cmpq	%r15, %r14
14233c27c: 75 e2                       	jne	0x14233c260 <.text+0x233b260>
14233c27e: eb 5f                       	jmp	0x14233c2df <.text+0x233b2df>
14233c280: 48 01 c8                    	addq	%rcx, %rax
14233c283: 8b 88 94 00 00 00           	movl	0x94(%rax), %ecx
14233c289: 48 8b 54 24 30              	movq	0x30(%rsp), %rdx
14233c28e: 4c 8b 8a a8 01 00 00        	movq	0x1a8(%rdx), %r9
14233c295: 4c 8d 14 c9                 	leaq	(%rcx,%rcx,8), %r10
14233c299: 4b 8d 14 d1                 	leaq	(%r9,%r10,8), %rdx
14233c29d: 48 8b 4c 24 28              	movq	0x28(%rsp), %rcx
14233c2a2: 4c 8b 81 10 01 00 00        	movq	0x110(%rcx), %r8
14233c2a9: 4b 8b 4c d1 38              	movq	0x38(%r9,%r10,8), %rcx
14233c2ae: 4f 39 44 d1 40              	cmpq	%r8, 0x40(%r9,%r10,8)
14233c2b3: 0f 86 eb fe ff ff           	jbe	0x14233c1a4 <.text+0x233b1a4>
14233c2b9: 4e 8b 04 c1                 	movq	(%rcx,%r8,8), %r8
14233c2bd: 48 8b 52 18                 	movq	0x18(%rdx), %rdx
14233c2c1: 4d 85 c0                    	testq	%r8, %r8
14233c2c4: 0f 84 de fe ff ff           	je	0x14233c1a8 <.text+0x233b1a8>
14233c2ca: 49 f7 d0                    	notq	%r8
14233c2cd: 4f 8d 04 40                 	leaq	(%r8,%r8,2), %r8
14233c2d1: 49 c1 e0 04                 	shlq	$0x4, %r8
14233c2d5: 4e 8b 6c 02 10              	movq	0x10(%rdx,%r8), %r13
14233c2da: e9 cc fe ff ff              	jmp	0x14233c1ab <.text+0x233b1ab>
14233c2df: 0f 28 74 24 40              	movaps	0x40(%rsp), %xmm6
14233c2e4: 0f 28 7c 24 50              	movaps	0x50(%rsp), %xmm7
14233c2e9: 48 83 c4 68                 	addq	$0x68, %rsp
14233c2ed: 5b                          	popq	%rbx
14233c2ee: 5d                          	popq	%rbp
14233c2ef: 5f                          	popq	%rdi
14233c2f0: 5e                          	popq	%rsi
14233c2f1: 41 5c                       	popq	%r12
14233c2f3: 41 5d                       	popq	%r13
14233c2f5: 41 5e                       	popq	%r14
14233c2f7: 41 5f                       	popq	%r15
14233c2f9: c3                          	retq
