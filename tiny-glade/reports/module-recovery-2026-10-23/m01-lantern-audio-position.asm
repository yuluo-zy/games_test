
/Users/yuluo/Desktop/game/B版本内容/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a57da0: 55                          	pushq	%rbp
140a57da1: 41 57                       	pushq	%r15
140a57da3: 41 56                       	pushq	%r14
140a57da5: 56                          	pushq	%rsi
140a57da6: 57                          	pushq	%rdi
140a57da7: 53                          	pushq	%rbx
140a57da8: 48 81 ec b8 01 00 00        	subq	$0x1b8, %rsp            # imm = 0x1B8
140a57daf: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
140a57db7: 48 c7 85 30 01 00 00 fe ff ff ff    	movq	$-0x2, 0x130(%rbp)
140a57dc2: 83 39 01                    	cmpl	$0x1, (%rcx)
140a57dc5: 0f 85 f9 03 00 00           	jne	0x140a581c4 <.text+0xa571c4>
140a57dcb: 48 89 cf                    	movq	%rcx, %rdi
140a57dce: 48 8d 41 08                 	leaq	0x8(%rcx), %rax
140a57dd2: f3 0f 10 42 08              	movss	0x8(%rdx), %xmm0
140a57dd7: f2 0f 10 0a                 	movsd	(%rdx), %xmm1
140a57ddb: f2 0f 11 4d 60              	movsd	%xmm1, 0x60(%rbp)
140a57de0: f3 0f 11 45 68              	movss	%xmm0, 0x68(%rbp)
140a57de5: 0f 28 05 74 86 07 02        	movaps	0x2078674(%rip), %xmm0  # 0x142ad0460
140a57dec: 0f 11 45 6c                 	movups	%xmm0, 0x6c(%rbp)
140a57df0: 0f 28 05 79 86 07 02        	movaps	0x2078679(%rip), %xmm0  # 0x142ad0470
140a57df7: 0f 11 45 7c                 	movups	%xmm0, 0x7c(%rbp)
140a57dfb: c7 85 8c 00 00 00 00 00 00 00       	movl	$0x0, 0x8c(%rbp)
140a57e05: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
140a57e09: 48 8d 75 60                 	leaq	0x60(%rbp), %rsi
140a57e0d: 48 89 c2                    	movq	%rax, %rdx
140a57e10: 49 89 f0                    	movq	%rsi, %r8
140a57e13: e8 38 f0 9d 00              	callq	0x141436e50 <.text+0x1435e50>
140a57e18: 48 b8 04 00 00 00 00 00 00 80       	movabsq	$-0x7ffffffffffffffc, %rax # imm = 0x8000000000000004
140a57e22: 48 39 45 b0                 	cmpq	%rax, -0x50(%rbp)
140a57e26: 0f 84 98 03 00 00           	je	0x140a581c4 <.text+0xa571c4>
140a57e2c: 48 8b 45 e0                 	movq	-0x20(%rbp), %rax
140a57e30: 48 89 45 30                 	movq	%rax, 0x30(%rbp)
140a57e34: 0f 10 45 b0                 	movups	-0x50(%rbp), %xmm0
140a57e38: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
140a57e3c: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
140a57e40: 0f 29 55 20                 	movaps	%xmm2, 0x20(%rbp)
140a57e44: 0f 29 4d 10                 	movaps	%xmm1, 0x10(%rbp)
140a57e48: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
140a57e4c: 48 83 c7 10                 	addq	$0x10, %rdi
140a57e50: 48 89 7d 40                 	movq	%rdi, 0x40(%rbp)
140a57e54: 48 8d 05 95 55 68 ff        	leaq	-0x97aa6b(%rip), %rax   # 0x1400dd3f0 <.text+0xdc3f0>
140a57e5b: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
140a57e5f: 48 89 e8                    	movq	%rbp, %rax
140a57e62: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140a57e66: 48 8d 05 83 ac ff ff        	leaq	-0x537d(%rip), %rax     # 0x140a52af0 <.text+0xa51af0>
140a57e6d: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
140a57e71: 48 c7 45 60 02 00 00 00     	movq	$0x2, 0x60(%rbp)
140a57e79: 48 c7 45 68 02 00 00 00     	movq	$0x2, 0x68(%rbp)
140a57e81: 48 c7 45 70 02 00 00 00     	movq	$0x2, 0x70(%rbp)
140a57e89: 48 c7 85 80 00 00 00 00 00 00 00    	movq	$0x0, 0x80(%rbp)
140a57e94: 48 c7 85 88 00 00 00 20 00 00 00    	movq	$0x20, 0x88(%rbp)
140a57e9f: c6 85 90 00 00 00 03        	movb	$0x3, 0x90(%rbp)
140a57ea6: 48 c7 85 98 00 00 00 02 00 00 00    	movq	$0x2, 0x98(%rbp)
140a57eb1: 48 c7 85 a0 00 00 00 02 00 00 00    	movq	$0x2, 0xa0(%rbp)
140a57ebc: 48 c7 85 a8 00 00 00 02 00 00 00    	movq	$0x2, 0xa8(%rbp)
140a57ec7: 48 c7 85 b8 00 00 00 01 00 00 00    	movq	$0x1, 0xb8(%rbp)
140a57ed2: 48 b8 20 00 00 00 04 00 00 00       	movabsq	$0x400000020, %rax # imm = 0x400000020
140a57edc: 48 89 85 c0 00 00 00        	movq	%rax, 0xc0(%rbp)
140a57ee3: c6 85 c8 00 00 00 03        	movb	$0x3, 0xc8(%rbp)
140a57eea: 48 8d 05 4f ae 07 02        	leaq	0x207ae4f(%rip), %rax   # 0x142ad2d40
140a57ef1: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
140a57ef8: 48 c7 85 f8 00 00 00 02 00 00 00    	movq	$0x2, 0xf8(%rbp)
140a57f03: 4c 8d 7d 40                 	leaq	0x40(%rbp), %r15
140a57f07: 4c 89 bd 00 01 00 00        	movq	%r15, 0x100(%rbp)
140a57f0e: 48 c7 85 08 01 00 00 02 00 00 00    	movq	$0x2, 0x108(%rbp)
140a57f19: 48 89 b5 10 01 00 00        	movq	%rsi, 0x110(%rbp)
140a57f20: 48 c7 85 18 01 00 00 02 00 00 00    	movq	$0x2, 0x118(%rbp)
140a57f2b: 48 8d 4d e8                 	leaq	-0x18(%rbp), %rcx
140a57f2f: 48 8d b5 f0 00 00 00        	leaq	0xf0(%rbp), %rsi
140a57f36: 48 89 f2                    	movq	%rsi, %rdx
140a57f39: e8 52 20 32 00              	callq	0x140d79f90 <.text+0xd78f90>
140a57f3e: 0f 10 45 e8                 	movups	-0x18(%rbp), %xmm0
140a57f42: 0f 29 45 40                 	movaps	%xmm0, 0x40(%rbp)
140a57f46: 48 8b 45 f8                 	movq	-0x8(%rbp), %rax
140a57f4a: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
140a57f4e: 8b 05 2c ce bd 02           	movl	0x2bdce2c(%rip), %eax   # 0x143634d80
140a57f54: 83 f8 03                    	cmpl	$0x3, %eax
140a57f57: 0f 85 78 02 00 00           	jne	0x140a581d5 <.text+0xa571d5>
140a57f5d: 48 8b 15 14 ce bd 02        	movq	0x2bdce14(%rip), %rdx   # 0x143634d78
140a57f64: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
140a57f68: e8 23 a9 ae ff              	callq	0x140542890 <.text+0x541890>
140a57f6d: 83 7d 60 01                 	cmpl	$0x1, 0x60(%rbp)
140a57f71: 0f 84 e3 02 00 00           	je	0x140a5825a <.text+0xa5725a>
140a57f77: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140a57f7b: 48 89 85 e8 00 00 00        	movq	%rax, 0xe8(%rbp)
140a57f82: 0f b6 45 70                 	movzbl	0x70(%rbp), %eax
140a57f86: 88 85 2f 01 00 00           	movb	%al, 0x12f(%rbp)
140a57f8c: 0f b6 05 76 cc bd 02        	movzbl	0x2bdcc76(%rip), %eax   # 0x143634c09
140a57f93: b9 0f 00 00 00              	movl	$0xf, %ecx
140a57f98: ba 01 00 00 00              	movl	$0x1, %edx
140a57f9d: e8 6e bc bb ff              	callq	0x140613c10 <.text+0x612c10>
140a57fa2: 48 85 c0                    	testq	%rax, %rax
140a57fa5: 0f 84 f1 02 00 00           	je	0x140a5829c <.text+0xa5729c>
140a57fab: 48 b9 70 61 74 68 21 20 28 29       	movabsq	$0x2928202168746170, %rcx # imm = 0x2928202168746170
140a57fb5: 48 89 48 07                 	movq	%rcx, 0x7(%rax)
140a57fb9: 48 b9 6d 6f 64 75 6c 65 5f 70       	movabsq	$0x705f656c75646f6d, %rcx # imm = 0x705F656C75646F6D
140a57fc3: 48 89 08                    	movq	%rcx, (%rax)
140a57fc6: 48 c7 45 60 0f 00 00 00     	movq	$0xf, 0x60(%rbp)
140a57fce: 48 89 45 68                 	movq	%rax, 0x68(%rbp)
140a57fd2: 48 c7 45 70 0f 00 00 00     	movq	$0xf, 0x70(%rbp)
140a57fda: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a57fe3: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
140a57fe7: ba 0f 00 00 00              	movl	$0xf, %edx
140a57fec: 41 b8 17 00 00 00           	movl	$0x17, %r8d
140a57ff2: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a57ff8: e8 63 1a e5 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a57ffd: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140a58001: 48 8b 7d 70                 	movq	0x70(%rbp), %rdi
140a58005: 0f 10 05 4f aa 07 02        	movups	0x207aa4f(%rip), %xmm0  # 0x142ad2a5b
140a5800c: 0f 11 04 38                 	movups	%xmm0, (%rax,%rdi)
140a58010: 48 b9 20 3a 3a 20 57 61 72 6e       	movabsq	$0x6e726157203a3a20, %rcx # imm = 0x6E726157203A3A20
140a5801a: 48 89 4c 38 0f              	movq	%rcx, 0xf(%rax,%rdi)
140a5801f: 48 83 c7 17                 	addq	$0x17, %rdi
140a58023: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
140a5802a: 0f 10 45 60                 	movups	0x60(%rbp), %xmm0
140a5802e: 0f 29 85 f0 00 00 00        	movaps	%xmm0, 0xf0(%rbp)
140a58035: 4c 8b 75 48                 	movq	0x48(%rbp), %r14
140a58039: 48 8b 5d 50                 	movq	0x50(%rbp), %rbx
140a5803d: 48 8b 85 f0 00 00 00        	movq	0xf0(%rbp), %rax
140a58044: 48 29 f8                    	subq	%rdi, %rax
140a58047: 48 39 d8                    	cmpq	%rbx, %rax
140a5804a: 0f 82 ba 01 00 00           	jb	0x140a5820a <.text+0xa5720a>
140a58050: 48 8b 8d f8 00 00 00        	movq	0xf8(%rbp), %rcx
140a58057: 48 01 f9                    	addq	%rdi, %rcx
140a5805a: 4c 89 f2                    	movq	%r14, %rdx
140a5805d: 49 89 d8                    	movq	%rbx, %r8
140a58060: e8 9b bb ec 01              	callq	0x142923c00 <.text+0x2922c00>
140a58065: 48 01 df                    	addq	%rbx, %rdi
140a58068: 48 89 bd 00 01 00 00        	movq	%rdi, 0x100(%rbp)
140a5806f: 48 89 7d 70                 	movq	%rdi, 0x70(%rbp)
140a58073: 0f 28 85 f0 00 00 00        	movaps	0xf0(%rbp), %xmm0
140a5807a: 0f 29 45 60                 	movaps	%xmm0, 0x60(%rbp)
140a5807e: 48 8b 85 e8 00 00 00        	movq	0xe8(%rbp), %rax
140a58085: 48 8d 48 08                 	leaq	0x8(%rax), %rcx
140a58089: 48 8d 55 60                 	leaq	0x60(%rbp), %rdx
140a5808d: e8 1e 38 9e 00              	callq	0x14143b8b0 <.text+0x143a8b0>
140a58092: 84 c0                       	testb	%al, %al
140a58094: 0f 85 d8 00 00 00           	jne	0x140a58172 <.text+0xa57172>
140a5809a: 48 8b 05 07 43 bd 02        	movq	0x2bd4307(%rip), %rax   # 0x14362c3a8
140a580a1: 48 8b 00                    	movq	(%rax), %rax
140a580a4: 48 83 f8 02                 	cmpq	$0x2, %rax
140a580a8: 0f 82 c4 00 00 00           	jb	0x140a58172 <.text+0xa57172>
140a580ae: 4c 89 bd f0 00 00 00        	movq	%r15, 0xf0(%rbp)
140a580b5: 48 8d 05 14 3a 5e ff        	leaq	-0xa1c5ec(%rip), %rax   # 0x14003bad0 <.text+0x3aad0>
140a580bc: 48 89 85 f8 00 00 00        	movq	%rax, 0xf8(%rbp)
140a580c3: 48 8d 0d 96 ac 07 02        	leaq	0x207ac96(%rip), %rcx   # 0x142ad2d60
140a580ca: e8 b1 5c c2 ff              	callq	0x14067dd80 <.text+0x67cd80>
140a580cf: 0f 10 00                    	movups	(%rax), %xmm0
140a580d2: 8b 40 10                    	movl	0x10(%rax), %eax
140a580d5: 48 c7 85 90 00 00 00 02 00 00 00    	movq	$0x2, 0x90(%rbp)
140a580e0: 48 8d 0d 69 a4 07 02        	leaq	0x207a469(%rip), %rcx   # 0x142ad2550
140a580e7: 48 89 8d 98 00 00 00        	movq	%rcx, 0x98(%rbp)
140a580ee: 48 c7 85 a0 00 00 00 28 00 00 00    	movq	$0x28, 0xa0(%rbp)
140a580f9: 48 8d 15 78 a9 07 02        	leaq	0x207a978(%rip), %rdx   # 0x142ad2a78
140a58100: 48 89 95 b0 00 00 00        	movq	%rdx, 0xb0(%rbp)
140a58107: 48 c7 85 b8 00 00 00 01 00 00 00    	movq	$0x1, 0xb8(%rbp)
140a58112: 48 89 b5 c0 00 00 00        	movq	%rsi, 0xc0(%rbp)
140a58119: 48 c7 85 c8 00 00 00 01 00 00 00    	movq	$0x1, 0xc8(%rbp)
140a58124: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
140a5812f: 48 c7 45 60 00 00 00 00     	movq	$0x0, 0x60(%rbp)
140a58137: 48 89 4d 68                 	movq	%rcx, 0x68(%rbp)
140a5813b: 48 c7 45 70 28 00 00 00     	movq	$0x28, 0x70(%rbp)
140a58143: 48 c7 45 78 00 00 00 00     	movq	$0x0, 0x78(%rbp)
140a5814b: 0f 11 85 80 00 00 00        	movups	%xmm0, 0x80(%rbp)
140a58152: c7 85 a8 00 00 00 01 00 00 00       	movl	$0x1, 0xa8(%rbp)
140a5815c: 89 85 ac 00 00 00           	movl	%eax, 0xac(%rbp)
140a58162: 48 8d 8d 2e 01 00 00        	leaq	0x12e(%rbp), %rcx
140a58169: 48 8d 55 60                 	leaq	0x60(%rbp), %rdx
140a5816d: e8 be 21 2f 00              	callq	0x140d4a330 <.text+0xd49330>
140a58172: 80 bd 2f 01 00 00 00        	cmpb	$0x0, 0x12f(%rbp)
140a58179: 48 8b b5 e8 00 00 00        	movq	0xe8(%rbp), %rsi
140a58180: 75 16                       	jne	0x140a58198 <.text+0xa57198>
140a58182: 48 8b 05 97 42 bd 02        	movq	0x2bd4297(%rip), %rax   # 0x14362c420
140a58189: 48 8b 00                    	movq	(%rax), %rax
140a5818c: 48 d1 e0                    	shlq	%rax
140a5818f: 48 85 c0                    	testq	%rax, %rax
140a58192: 0f 85 ac 00 00 00           	jne	0x140a58244 <.text+0xa57244>
140a58198: 31 c0                       	xorl	%eax, %eax
140a5819a: 86 06                       	xchgb	%al, (%rsi)
140a5819c: 3c 02                       	cmpb	$0x2, %al
140a5819e: 0f 84 93 00 00 00           	je	0x140a58237 <.text+0xa57237>
140a581a4: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
140a581a8: 48 85 d2                    	testq	%rdx, %rdx
140a581ab: 74 0f                       	je	0x140a581bc <.text+0xa571bc>
140a581ad: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
140a581b1: 41 b8 01 00 00 00           	movl	$0x1, %r8d
140a581b7: e8 64 ba bb ff              	callq	0x140613c20 <.text+0x612c20>
140a581bc: 48 89 e9                    	movq	%rbp, %rcx
140a581bf: e8 ec a4 ff ff              	callq	0x140a526b0 <.text+0xa516b0>
140a581c4: 90                          	nop
140a581c5: 48 81 c4 b8 01 00 00        	addq	$0x1b8, %rsp            # imm = 0x1B8
140a581cc: 5b                          	popq	%rbx
140a581cd: 5f                          	popq	%rdi
140a581ce: 5e                          	popq	%rsi
140a581cf: 41 5e                       	popq	%r14
140a581d1: 41 5f                       	popq	%r15
140a581d3: 5d                          	popq	%rbp
140a581d4: c3                          	retq
140a581d5: c6 85 f0 00 00 00 01        	movb	$0x1, 0xf0(%rbp)
140a581dc: 48 89 75 60                 	movq	%rsi, 0x60(%rbp)
140a581e0: 48 8d 05 79 ab 07 02        	leaq	0x207ab79(%rip), %rax   # 0x142ad2d60
140a581e7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a581ec: 48 8d 0d 8d cb bd 02        	leaq	0x2bdcb8d(%rip), %rcx   # 0x143634d80
140a581f3: 4c 8d 0d be 82 07 02        	leaq	0x20782be(%rip), %r9    # 0x142ad04b8
140a581fa: 4c 8d 45 60                 	leaq	0x60(%rbp), %r8
140a581fe: 31 d2                       	xorl	%edx, %edx
140a58200: e8 1b 06 e8 01              	callq	0x1428d8820 <.text+0x28d7820>
140a58205: e9 53 fd ff ff              	jmp	0x140a57f5d <.text+0xa56f5d>
140a5820a: 48 c7 44 24 20 01 00 00 00  	movq	$0x1, 0x20(%rsp)
140a58213: 48 8d 8d f0 00 00 00        	leaq	0xf0(%rbp), %rcx
140a5821a: 41 b9 01 00 00 00           	movl	$0x1, %r9d
140a58220: 48 89 fa                    	movq	%rdi, %rdx
140a58223: 49 89 d8                    	movq	%rbx, %r8
140a58226: e8 35 18 e5 01              	callq	0x1428a9a60 <.text+0x28a8a60>
140a5822b: 48 8b bd 00 01 00 00        	movq	0x100(%rbp), %rdi
140a58232: e9 19 fe ff ff              	jmp	0x140a58050 <.text+0xa57050>
140a58237: 48 89 f1                    	movq	%rsi, %rcx
140a5823a: e8 d1 05 e8 01              	callq	0x1428d8810 <.text+0x28d7810>
140a5823f: e9 60 ff ff ff              	jmp	0x140a581a4 <.text+0xa571a4>
140a58244: e8 67 04 e8 01              	callq	0x1428d86b0 <.text+0x28d76b0>
140a58249: 84 c0                       	testb	%al, %al
140a5824b: 0f 85 47 ff ff ff           	jne	0x140a58198 <.text+0xa57198>
140a58251: c6 46 01 01                 	movb	$0x1, 0x1(%rsi)
140a58255: e9 3e ff ff ff              	jmp	0x140a58198 <.text+0xa57198>
140a5825a: 48 8b 45 68                 	movq	0x68(%rbp), %rax
140a5825e: 0f b6 4d 70                 	movzbl	0x70(%rbp), %ecx
140a58262: 48 89 85 f0 00 00 00        	movq	%rax, 0xf0(%rbp)
140a58269: 88 8d f8 00 00 00           	movb	%cl, 0xf8(%rbp)
140a5826f: 48 8d 05 ea aa 07 02        	leaq	0x207aaea(%rip), %rax   # 0x142ad2d60
140a58276: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a5827b: 48 8d 0d c6 a7 07 02        	leaq	0x207a7c6(%rip), %rcx   # 0x142ad2a48
140a58282: 4c 8d 0d 17 83 07 02        	leaq	0x2078317(%rip), %r9    # 0x142ad05a0
140a58289: 4c 8d 85 f0 00 00 00        	leaq	0xf0(%rbp), %r8
140a58290: ba 13 00 00 00              	movl	$0x13, %edx
140a58295: e8 c6 14 e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a5829a: eb 16                       	jmp	0x140a582b2 <.text+0xa572b2>
140a5829c: 4c 8d 05 0d 85 07 02        	leaq	0x207850d(%rip), %r8    # 0x142ad07b0
140a582a3: b9 01 00 00 00              	movl	$0x1, %ecx
140a582a8: ba 0f 00 00 00              	movl	$0xf, %edx
140a582ad: e8 31 0d e8 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
