
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1421c36d0: 56                          	pushq	%rsi
1421c36d1: 57                          	pushq	%rdi
1421c36d2: 53                          	pushq	%rbx
1421c36d3: 48 83 ec 20                 	subq	$0x20, %rsp
1421c36d7: 4c 89 ce                    	movq	%r9, %rsi
1421c36da: 4c 89 c3                    	movq	%r8, %rbx
1421c36dd: 48 89 cf                    	movq	%rcx, %rdi
1421c36e0: 89 d0                       	movl	%edx, %eax
1421c36e2: 48 8d 0d 6f 72 e3 00        	leaq	0xe3726f(%rip), %rcx    # 0x142ffa958
1421c36e9: 48 63 04 81                 	movslq	(%rcx,%rax,4), %rax
1421c36ed: 48 01 c8                    	addq	%rcx, %rax
1421c36f0: ff e0                       	jmpq	*%rax
1421c36f2: 48 c7 07 00 00 00 00        	movq	$0x0, (%rdi)
1421c36f9: 48 c7 47 08 08 00 00 00     	movq	$0x8, 0x8(%rdi)
1421c3701: 48 c7 47 10 00 00 00 00     	movq	$0x0, 0x10(%rdi)
1421c3709: e9 db 00 00 00              	jmp	0x1421c37e9 <.text+0x21c27e9>
1421c370e: 0f b6 05 f4 14 47 01        	movzbl	0x14714f4(%rip), %eax   # 0x143634c09
1421c3715: b9 20 00 00 00              	movl	$0x20, %ecx
1421c371a: ba 08 00 00 00              	movl	$0x8, %edx
1421c371f: e8 ec 04 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c3724: 48 85 c0                    	testq	%rax, %rax
1421c3727: 0f 84 db 00 00 00           	je	0x1421c3808 <.text+0x21c2808>
1421c372d: 0f 10 05 8c 6f e3 00        	movups	0xe36f8c(%rip), %xmm0   # 0x142ffa6c0
1421c3734: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1421c3738: 0f 10 05 71 6f e3 00        	movups	0xe36f71(%rip), %xmm0   # 0x142ffa6b0
1421c373f: e9 8f 00 00 00              	jmp	0x1421c37d3 <.text+0x21c27d3>
1421c3744: 0f b6 05 be 14 47 01        	movzbl	0x14714be(%rip), %eax   # 0x143634c09
1421c374b: b9 20 00 00 00              	movl	$0x20, %ecx
1421c3750: ba 08 00 00 00              	movl	$0x8, %edx
1421c3755: e8 b6 04 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c375a: 48 85 c0                    	testq	%rax, %rax
1421c375d: 0f 84 a5 00 00 00           	je	0x1421c3808 <.text+0x21c2808>
1421c3763: 0f 10 05 2e 6e e3 00        	movups	0xe36e2e(%rip), %xmm0   # 0x142ffa598
1421c376a: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1421c376e: 0f 10 05 13 6e e3 00        	movups	0xe36e13(%rip), %xmm0   # 0x142ffa588
1421c3775: eb 5c                       	jmp	0x1421c37d3 <.text+0x21c27d3>
1421c3777: 0f b6 05 8b 14 47 01        	movzbl	0x147148b(%rip), %eax   # 0x143634c09
1421c377e: b9 20 00 00 00              	movl	$0x20, %ecx
1421c3783: ba 08 00 00 00              	movl	$0x8, %edx
1421c3788: e8 83 04 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c378d: 48 85 c0                    	testq	%rax, %rax
1421c3790: 74 76                       	je	0x1421c3808 <.text+0x21c2808>
1421c3792: 0f 10 05 bf 6e e3 00        	movups	0xe36ebf(%rip), %xmm0   # 0x142ffa658
1421c3799: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1421c379d: 0f 10 05 a4 6e e3 00        	movups	0xe36ea4(%rip), %xmm0   # 0x142ffa648
1421c37a4: eb 2d                       	jmp	0x1421c37d3 <.text+0x21c27d3>
1421c37a6: 0f b6 05 5c 14 47 01        	movzbl	0x147145c(%rip), %eax   # 0x143634c09
1421c37ad: b9 20 00 00 00              	movl	$0x20, %ecx
1421c37b2: ba 08 00 00 00              	movl	$0x8, %edx
1421c37b7: e8 54 04 45 fe              	callq	0x140613c10 <.text+0x612c10>
1421c37bc: 48 85 c0                    	testq	%rax, %rax
1421c37bf: 74 47                       	je	0x1421c3808 <.text+0x21c2808>
1421c37c1: 0f 10 05 88 6f e3 00        	movups	0xe36f88(%rip), %xmm0   # 0x142ffa750
1421c37c8: 0f 11 40 10                 	movups	%xmm0, 0x10(%rax)
1421c37cc: 0f 10 05 6d 6f e3 00        	movups	0xe36f6d(%rip), %xmm0   # 0x142ffa740
1421c37d3: 0f 11 00                    	movups	%xmm0, (%rax)
1421c37d6: 48 c7 07 01 00 00 00        	movq	$0x1, (%rdi)
1421c37dd: 48 89 47 08                 	movq	%rax, 0x8(%rdi)
1421c37e1: 48 c7 47 10 01 00 00 00     	movq	$0x1, 0x10(%rdi)
1421c37e9: 48 89 5f 18                 	movq	%rbx, 0x18(%rdi)
1421c37ed: 48 89 77 20                 	movq	%rsi, 0x20(%rdi)
1421c37f1: 48 83 c4 20                 	addq	$0x20, %rsp
1421c37f5: 5b                          	popq	%rbx
1421c37f6: 5f                          	popq	%rdi
1421c37f7: 5e                          	popq	%rsi
1421c37f8: c3                          	retq
1421c37f9: 48 b8 00 00 00 00 00 00 00 80       	movabsq	$-0x8000000000000000, %rax # imm = 0x8000000000000000
1421c3803: 48 89 07                    	movq	%rax, (%rdi)
1421c3806: eb e9                       	jmp	0x1421c37f1 <.text+0x21c27f1>
1421c3808: 4c 8d 05 21 68 e3 00        	leaq	0xe36821(%rip), %r8     # 0x142ffa030
1421c380f: b9 08 00 00 00              	movl	$0x8, %ecx
1421c3814: ba 20 00 00 00              	movl	$0x20, %edx
1421c3819: e8 c5 57 71 00              	callq	0x1428d8fe3 <.text+0x28d7fe3>
1421c381e: cc                          	int3
1421c381f: cc                          	int3
