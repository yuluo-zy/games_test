
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a3fbf0: c1 b0 01 89 85 b0           	<unknown>
140a3fbf6: 02 00                       	addb	(%rax), %al
140a3fbf8: 00 f3                       	addb	%dh, %bl
140a3fbfa: 0f 10 05 cb 1c f9 01        	movups	0x1f91ccb(%rip), %xmm0  # 0x1429d18cc
140a3fc01: 0f 2e 87 48 01 00 00        	ucomiss	0x148(%rdi), %xmm0
140a3fc08: 48 89 95 a0 02 00 00        	movq	%rdx, 0x2a0(%rbp)
140a3fc0f: 0f 86 23 01 00 00           	jbe	0x140a3fd38 <.text+0xa3ed38>
140a3fc15: f3 41 0f 10 44 24 28        	movss	0x28(%r12), %xmm0
140a3fc1c: 0f 57 c9                    	xorps	%xmm1, %xmm1
140a3fc1f: 0f 2e c1                    	ucomiss	%xmm1, %xmm0
140a3fc22: 0f 82 d9 14 00 00           	jb	0x140a41101 <.text+0xa40101>
140a3fc28: f3 0f 58 87 44 01 00 00     	addss	0x144(%rdi), %xmm0
140a3fc30: f3 0f 11 87 44 01 00 00     	movss	%xmm0, 0x144(%rdi)
140a3fc38: 84 c9                       	testb	%cl, %cl
140a3fc3a: 0f 84 f8 00 00 00           	je	0x140a3fd38 <.text+0xa3ed38>
140a3fc40: c7 85 b8 02 00 00 02 00 00 00       	movl	$0x2, 0x2b8(%rbp)
140a3fc4a: f3 0f 5f c8                 	maxss	%xmm0, %xmm1
140a3fc4e: f3 0f 10 05 ae 49 ef 01     	movss	0x1ef49ae(%rip), %xmm0  # 0x142934604
140a3fc56: f3 0f 5d c1                 	minss	%xmm1, %xmm0
140a3fc5a: f3 0f 11 87 44 01 00 00     	movss	%xmm0, 0x144(%rdi)
140a3fc62: 0f 2e 05 9b 5c ee 01        	ucomiss	0x1ee5c9b(%rip), %xmm0  # 0x142925904
140a3fc69: 0f 82 d3 00 00 00           	jb	0x140a3fd42 <.text+0xa3ed42>
140a3fc6f: 44 8b a7 40 01 00 00        	movl	0x140(%rdi), %r12d
140a3fc76: ba c5 b3 a2 91              	movl	$0x91a2b3c5, %edx       # imm = 0x91A2B3C5
140a3fc7b: f3 0f 10 0d 61 a4 f1 01     	movss	0x1f1a461(%rip), %xmm1  # 0x14295a0e4
140a3fc83: 4c 8d 05 ca f6 05 02        	leaq	0x205f6ca(%rip), %r8    # 0x142a9f354
140a3fc8a: 4c 8d 15 bf f6 05 02        	leaq	0x205f6bf(%rip), %r10   # 0x142a9f350
140a3fc91: 4c 8d 1d b4 f6 05 02        	leaq	0x205f6b4(%rip), %r11   # 0x142a9f34c
140a3fc98: 48 8d 05 a9 f6 05 02        	leaq	0x205f6a9(%rip), %rax   # 0x142a9f348
140a3fc9f: f3 0f 10 15 5d 5c ee 01     	movss	0x1ee5c5d(%rip), %xmm2  # 0x142925904
140a3fca7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a3fcb0: 45 89 e1                    	movl	%r12d, %r9d
140a3fcb3: 4c 89 ce                    	movq	%r9, %rsi
140a3fcb6: 48 0f af f2                 	imulq	%rdx, %rsi
140a3fcba: 48 89 f7                    	movq	%rsi, %rdi
140a3fcbd: 48 c1 ef 2c                 	shrq	$0x2c, %rdi
140a3fcc1: 69 ff 20 1c 00 00           	imull	$0x1c20, %edi, %edi     # imm = 0x1C20
140a3fcc7: f3 0f 58 c1                 	addss	%xmm1, %xmm0
140a3fccb: 44 39 e7                    	cmpl	%r12d, %edi
140a3fcce: 0f 84 37 13 00 00           	je	0x140a4100b <.text+0xa4000b>
140a3fcd4: 48 c1 ee 2b                 	shrq	$0x2b, %rsi
140a3fcd8: 69 f6 10 0e 00 00           	imull	$0xe10, %esi, %esi      # imm = 0xE10
140a3fcde: 44 39 e6                    	cmpl	%r12d, %esi
140a3fce1: 48 8b bd 48 03 00 00        	movq	0x348(%rbp), %rdi
140a3fce8: 0f 84 2c 13 00 00           	je	0x140a4101a <.text+0xa4001a>
140a3fcee: 4d 69 c9 b5 81 4e 1b        	imulq	$0x1b4e81b5, %r9, %r9   # imm = 0x1B4E81B5
140a3fcf5: 4c 89 ce                    	movq	%r9, %rsi
140a3fcf8: 48 c1 ee 26                 	shrq	$0x26, %rsi
140a3fcfc: 69 f6 58 02 00 00           	imull	$0x258, %esi, %esi      # imm = 0x258
140a3fd02: 44 39 e6                    	cmpl	%r12d, %esi
140a3fd05: 0f 84 1a 13 00 00           	je	0x140a41025 <.text+0xa40025>
140a3fd0b: 49 c1 e9 25                 	shrq	$0x25, %r9
140a3fd0f: 45 69 c9 2c 01 00 00        	imull	$0x12c, %r9d, %r9d      # imm = 0x12C
140a3fd16: 45 39 e1                    	cmpl	%r12d, %r9d
140a3fd19: 0f 84 11 13 00 00           	je	0x140a41030 <.text+0xa40030>
140a3fd1f: 41 ff c4                    	incl	%r12d
140a3fd22: 0f 2e c2                    	ucomiss	%xmm2, %xmm0
140a3fd25: 73 89                       	jae	0x140a3fcb0 <.text+0xa3ecb0>
140a3fd27: f3 0f 11 87 44 01 00 00     	movss	%xmm0, 0x144(%rdi)
140a3fd2f: 44 89 a7 40 01 00 00        	movl	%r12d, 0x140(%rdi)
140a3fd36: eb 0a                       	jmp	0x140a3fd42 <.text+0xa3ed42>
140a3fd38: c7 85 b8 02 00 00 02 00 00 00       	movl	$0x2, 0x2b8(%rbp)
