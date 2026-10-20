
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14048db70: 55                          	pushq	%rbp
14048db71: 56                          	pushq	%rsi
14048db72: 57                          	pushq	%rdi
14048db73: 53                          	pushq	%rbx
14048db74: 48 83 ec 58                 	subq	$0x58, %rsp
14048db78: 48 8d 6c 24 50              	leaq	0x50(%rsp), %rbp
14048db7d: 48 c7 45 00 fe ff ff ff     	movq	$-0x2, (%rbp)
14048db85: 4c 8b 59 08                 	movq	0x8(%rcx), %r11
14048db89: 48 8b 41 18                 	movq	0x18(%rcx), %rax
14048db8d: 4c 8b 02                    	movq	(%rdx), %r8
14048db90: 4c 8b 4a 08                 	movq	0x8(%rdx), %r9
14048db94: 49 39 c3                    	cmpq	%rax, %r11
14048db97: 74 6b                       	je	0x14048dc04 <.text+0x48cc04>
14048db99: 4d 69 d1 98 00 00 00        	imulq	$0x98, %r9, %r10
14048dba0: 4c 03 52 10                 	addq	0x10(%rdx), %r10
14048dba4: 48 ba 00 00 00 00 03 00 00 00       	movabsq	$0x300000000, %rdx # imm = 0x300000000
14048dbae: 66 90                       	nop
14048dbb0: 49 8d 73 50                 	leaq	0x50(%r11), %rsi
14048dbb4: 41 0f 10 03                 	movups	(%r11), %xmm0
14048dbb8: 41 0f 10 4b 10              	movups	0x10(%r11), %xmm1
14048dbbd: 41 0f 10 53 20              	movups	0x20(%r11), %xmm2
14048dbc2: 41 0f 10 5b 30              	movups	0x30(%r11), %xmm3
14048dbc7: 41 0f 10 63 40              	movups	0x40(%r11), %xmm4
14048dbcc: 41 0f 11 62 40              	movups	%xmm4, 0x40(%r10)
14048dbd1: 41 0f 11 5a 30              	movups	%xmm3, 0x30(%r10)
14048dbd6: 41 0f 11 52 20              	movups	%xmm2, 0x20(%r10)
14048dbdb: 41 0f 11 4a 10              	movups	%xmm1, 0x10(%r10)
14048dbe0: 41 0f 11 02                 	movups	%xmm0, (%r10)
14048dbe4: 49 89 52 50                 	movq	%rdx, 0x50(%r10)
14048dbe8: 41 c6 82 96 00 00 00 06     	movb	$0x6, 0x96(%r10)
14048dbf0: 49 ff c1                    	incq	%r9
14048dbf3: 49 81 c2 98 00 00 00        	addq	$0x98, %r10
14048dbfa: 49 89 f3                    	movq	%rsi, %r11
14048dbfd: 48 39 c6                    	cmpq	%rax, %rsi
14048dc00: 75 ae                       	jne	0x14048dbb0 <.text+0x48cbb0>
14048dc02: eb 03                       	jmp	0x14048dc07 <.text+0x48cc07>
14048dc04: 4c 89 de                    	movq	%r11, %rsi
14048dc07: 48 8b 11                    	movq	(%rcx), %rdx
14048dc0a: 48 89 55 e8                 	movq	%rdx, -0x18(%rbp)
14048dc0e: 48 8b 49 10                 	movq	0x10(%rcx), %rcx
14048dc12: 48 89 4d f8                 	movq	%rcx, -0x8(%rbp)
14048dc16: 4d 89 08                    	movq	%r9, (%r8)
14048dc19: 48 29 f0                    	subq	%rsi, %rax
14048dc1c: 48 c1 e8 04                 	shrq	$0x4, %rax
14048dc20: 48 b9 cd cc cc cc cc cc cc cc       	movabsq	$-0x3333333333333333, %rcx # imm = 0xCCCCCCCCCCCCCCCD
14048dc2a: 48 0f af c8                 	imulq	%rax, %rcx
14048dc2e: 48 89 4d e0                 	movq	%rcx, -0x20(%rbp)
14048dc32: 48 c7 45 f0 00 00 00 00     	movq	$0x0, -0x10(%rbp)
14048dc3a: 48 89 75 d8                 	movq	%rsi, -0x28(%rbp)
14048dc3e: 48 89 f1                    	movq	%rsi, %rcx
14048dc41: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14048dc50: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
14048dc54: 48 39 45 e0                 	cmpq	%rax, -0x20(%rbp)
14048dc58: 74 15                       	je	0x14048dc6f <.text+0x48cc6f>
14048dc5a: 48 ff c0                    	incq	%rax
14048dc5d: 48 89 45 f0                 	movq	%rax, -0x10(%rbp)
14048dc61: 48 8d 71 50                 	leaq	0x50(%rcx), %rsi
14048dc65: e8 76 9c 16 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14048dc6a: 48 89 f1                    	movq	%rsi, %rcx
14048dc6d: eb e1                       	jmp	0x14048dc50 <.text+0x48cc50>
14048dc6f: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14048dc73: 48 85 c0                    	testq	%rax, %rax
14048dc76: 74 1f                       	je	0x14048dc97 <.text+0x48cc97>
14048dc78: 48 c1 e0 04                 	shlq	$0x4, %rax
14048dc7c: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
14048dc80: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14048dc86: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14048dc8a: 48 83 c4 58                 	addq	$0x58, %rsp
14048dc8e: 5b                          	popq	%rbx
14048dc8f: 5f                          	popq	%rdi
14048dc90: 5e                          	popq	%rsi
14048dc91: 5d                          	popq	%rbp
14048dc92: e9 89 5f 18 00              	jmp	0x140613c20 <.text+0x612c20>
14048dc97: 90                          	nop
14048dc98: 48 83 c4 58                 	addq	$0x58, %rsp
14048dc9c: 5b                          	popq	%rbx
14048dc9d: 5f                          	popq	%rdi
14048dc9e: 5e                          	popq	%rsi
14048dc9f: 5d                          	popq	%rbp
14048dca0: c3                          	retq
14048dca1: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14048dcb0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14048dcb5: 55                          	pushq	%rbp
14048dcb6: 56                          	pushq	%rsi
14048dcb7: 57                          	pushq	%rdi
14048dcb8: 53                          	pushq	%rbx
14048dcb9: 48 83 ec 28                 	subq	$0x28, %rsp
14048dcbd: 48 8d 6a 50                 	leaq	0x50(%rdx), %rbp
14048dcc1: 48 83 7d f8 00              	cmpq	$0x0, -0x8(%rbp)
14048dcc6: 74 1b                       	je	0x14048dce3 <.text+0x48cce3>
14048dcc8: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
14048dccc: 48 c1 e0 04                 	shlq	$0x4, %rax
14048dcd0: 48 8d 14 80                 	leaq	(%rax,%rax,4), %rdx
14048dcd4: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14048dcda: 48 8b 4d e8                 	movq	-0x18(%rbp), %rcx
14048dcde: e8 3d 5f 18 00              	callq	0x140613c20 <.text+0x612c20>
14048dce3: 90                          	nop
14048dce4: 48 83 c4 28                 	addq	$0x28, %rsp
14048dce8: 5b                          	popq	%rbx
14048dce9: 5f                          	popq	%rdi
14048dcea: 5e                          	popq	%rsi
14048dceb: 5d                          	popq	%rbp
14048dcec: c3                          	retq
14048dced: 0f 1f 00                    	nopl	(%rax)
14048dcf0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
14048dcf5: 55                          	pushq	%rbp
14048dcf6: 56                          	pushq	%rsi
14048dcf7: 57                          	pushq	%rdi
14048dcf8: 53                          	pushq	%rbx
14048dcf9: 48 83 ec 28                 	subq	$0x28, %rsp
14048dcfd: 48 8d 6a 50                 	leaq	0x50(%rdx), %rbp
14048dd01: 48 8b 7d e0                 	movq	-0x20(%rbp), %rdi
14048dd05: 48 8b 5d f0                 	movq	-0x10(%rbp), %rbx
14048dd09: 48 39 df                    	cmpq	%rbx, %rdi
14048dd0c: 48 8b 75 d8                 	movq	-0x28(%rbp), %rsi
14048dd10: 74 26                       	je	0x14048dd38 <.text+0x48cd38>
14048dd12: 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00   	nopw	%cs:(%rax,%rax)
14048dd20: 48 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %rcx
14048dd24: 48 c1 e1 04                 	shlq	$0x4, %rcx
14048dd28: 48 01 f1                    	addq	%rsi, %rcx
14048dd2b: e8 b0 9b 16 00              	callq	0x1405f78e0 <.text+0x5f68e0>
14048dd30: 48 ff c3                    	incq	%rbx
14048dd33: 48 39 fb                    	cmpq	%rdi, %rbx
14048dd36: 75 e8                       	jne	0x14048dd20 <.text+0x48cd20>
14048dd38: 48 83 c4 28                 	addq	$0x28, %rsp
14048dd3c: 5b                          	popq	%rbx
14048dd3d: 5f                          	popq	%rdi
14048dd3e: 5e                          	popq	%rsi
14048dd3f: 5d                          	popq	%rbp
14048dd40: c3                          	retq
14048dd41: cc                          	int3
14048dd42: cc                          	int3
14048dd43: cc                          	int3
14048dd44: cc                          	int3
14048dd45: cc                          	int3
14048dd46: cc                          	int3
14048dd47: cc                          	int3
14048dd48: cc                          	int3
14048dd49: cc                          	int3
14048dd4a: cc                          	int3
14048dd4b: cc                          	int3
14048dd4c: cc                          	int3
14048dd4d: cc                          	int3
14048dd4e: cc                          	int3
14048dd4f: cc                          	int3
14048dd50: 55                          	pushq	%rbp
14048dd51: 41 57                       	pushq	%r15
14048dd53: 41 56                       	pushq	%r14
14048dd55: 41 55                       	pushq	%r13
14048dd57: 41 54                       	pushq	%r12
14048dd59: 56                          	pushq	%rsi
14048dd5a: 57                          	pushq	%rdi
14048dd5b: 53                          	pushq	%rbx
14048dd5c: 48 81 ec b8 00 00 00        	subq	$0xb8, %rsp
14048dd63: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14048dd6b: 44 0f 29 55 20              	movaps	%xmm10, 0x20(%rbp)
14048dd70: 44 0f 29 4d 10              	movaps	%xmm9, 0x10(%rbp)
14048dd75: 44 0f 29 45 00              	movaps	%xmm8, (%rbp)
14048dd7a: 0f 29 7d f0                 	movaps	%xmm7, -0x10(%rbp)
14048dd7e: 0f 29 75 e0                 	movaps	%xmm6, -0x20(%rbp)
14048dd82: 48 c7 45 d8 fe ff ff ff     	movq	$-0x2, -0x28(%rbp)
14048dd8a: 48 8b 39                    	movq	(%rcx), %rdi
14048dd8d: 48 8b 41 08                 	movq	0x8(%rcx), %rax
14048dd91: 4c 8b 02                    	movq	(%rdx), %r8
14048dd94: 4c 89 45 c8                 	movq	%r8, -0x38(%rbp)
14048dd98: 4c 8b 42 08                 	movq	0x8(%rdx), %r8
14048dd9c: 4c 89 45 d0                 	movq	%r8, -0x30(%rbp)
14048dda0: 48 39 c7                    	cmpq	%rax, %rdi
14048dda3: 0f 84 db 01 00 00           	je	0x14048df84 <.text+0x48cf84>
14048dda9: 48 8b 59 10                 	movq	0x10(%rcx), %rbx
14048ddad: 4c 8b 41 18                 	movq	0x18(%rcx), %r8
14048ddb1: 4c 89 45 c0                 	movq	%r8, -0x40(%rbp)
14048ddb5: 4c 8b 41 20                 	movq	0x20(%rcx), %r8
14048ddb9: 4c 89 45 b8                 	movq	%r8, -0x48(%rbp)
14048ddbd: 4c 8b 41 28                 	movq	0x28(%rcx), %r8
14048ddc1: 4c 89 45 b0                 	movq	%r8, -0x50(%rbp)
14048ddc5: 4c 8b 41 30                 	movq	0x30(%rcx), %r8
14048ddc9: 4c 89 45 a8                 	movq	%r8, -0x58(%rbp)
14048ddcd: 48 8b 71 38                 	movq	0x38(%rcx), %rsi
14048ddd1: 4c 8b 71 40                 	movq	0x40(%rcx), %r14
14048ddd5: 4c 8b 7a 10                 	movq	0x10(%rdx), %r15
14048ddd9: 48 29 f8                    	subq	%rdi, %rax
14048dddc: 48 c1 e8 02                 	shrq	$0x2, %rax
14048dde0: 49 bc ab aa aa aa aa aa aa aa       	movabsq	$-0x5555555555555555, %r12 # imm = 0xAAAAAAAAAAAAAAAB
14048ddea: 4c 0f af e0                 	imulq	%rax, %r12
14048ddee: 45 31 ed                    	xorl	%r13d, %r13d
