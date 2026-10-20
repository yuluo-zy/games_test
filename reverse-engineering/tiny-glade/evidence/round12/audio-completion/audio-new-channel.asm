
D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140a556b0: 48 8d 8d 60 02 00 00        	leaq	0x260(%rbp), %rcx
140a556b7: e8 54 e2 30 00              	callq	0x140d63910 <.text+0xd62910>
140a556bc: 48 89 85 30 05 00 00        	movq	%rax, 0x530(%rbp)
140a556c3: 89 95 38 05 00 00           	movl	%edx, 0x538(%rbp)
140a556c9: 48 8d 85 30 05 00 00        	leaq	0x530(%rbp), %rax
140a556d0: 48 89 85 20 05 00 00        	movq	%rax, 0x520(%rbp)
140a556d7: 48 8d 05 72 4e 33 00        	leaq	0x334e72(%rip), %rax    # 0x140d8a550 <.text+0xd89550>
140a556de: 48 89 85 28 05 00 00        	movq	%rax, 0x528(%rbp)
140a556e5: 48 8d 05 2c d1 07 02        	leaq	0x207d12c(%rip), %rax   # 0x142ad2818
140a556ec: 48 89 85 a0 02 00 00        	movq	%rax, 0x2a0(%rbp)
140a556f3: 48 c7 85 a8 02 00 00 01 00 00 00    	movq	$0x1, 0x2a8(%rbp)
140a556fe: 48 c7 85 c0 02 00 00 00 00 00 00    	movq	$0x0, 0x2c0(%rbp)
140a55709: 48 8d 85 20 05 00 00        	leaq	0x520(%rbp), %rax
140a55710: 48 89 85 b0 02 00 00        	movq	%rax, 0x2b0(%rbp)
140a55717: 48 c7 85 b8 02 00 00 01 00 00 00    	movq	$0x1, 0x2b8(%rbp)
140a55722: 48 8d 0d ff d0 07 02        	leaq	0x207d0ff(%rip), %rcx   # 0x142ad2828
140a55729: e8 52 86 c2 ff              	callq	0x14067dd80 <.text+0x67cd80>
140a5572e: 48 8d 0d 1b ce 07 02        	leaq	0x207ce1b(%rip), %rcx   # 0x142ad2550
140a55735: 48 89 4d c0                 	movq	%rcx, -0x40(%rbp)
140a55739: 48 c7 45 c8 28 00 00 00     	movq	$0x28, -0x38(%rbp)
140a55741: 48 89 4d d0                 	movq	%rcx, -0x30(%rbp)
140a55745: 48 c7 45 d8 28 00 00 00     	movq	$0x28, -0x28(%rbp)
140a5574d: 48 89 45 e0                 	movq	%rax, -0x20(%rbp)
140a55751: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140a55758: 4c 8d 45 c0                 	leaq	-0x40(%rbp), %r8
140a5575c: ba 04 00 00 00              	movl	$0x4, %edx
140a55761: e8 ca bc a5 ff              	callq	0x1404b1430 <.text+0x4b0430>
140a55766: 48 8b 85 90 05 00 00        	movq	0x590(%rbp), %rax
140a5576d: 48 8d 95 b0 05 00 00        	leaq	0x5b0(%rbp), %rdx
140a55774: 48 8b 4a 08                 	movq	0x8(%rdx), %rcx
140a55778: f3 0f 6f 42 e8              	movdqu	-0x18(%rdx), %xmm0
140a5577d: 0f 10 4a f8                 	movups	-0x8(%rdx), %xmm1
140a55781: 66 0f 7f 85 30 02 00 00     	movdqa	%xmm0, 0x230(%rbp)
140a55789: 0f 29 8d 40 02 00 00        	movaps	%xmm1, 0x240(%rbp)
140a55790: 48 89 8d 50 02 00 00        	movq	%rcx, 0x250(%rbp)
140a55797: 48 85 c0                    	testq	%rax, %rax
140a5579a: 0f 84 23 f8 ff ff           	je	0x140a54fc3 <.text+0xa53fc3>
140a557a0: 48 8b 8d 50 02 00 00        	movq	0x250(%rbp), %rcx
140a557a7: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
140a557ab: 66 0f 6f 85 30 02 00 00     	movdqa	0x230(%rbp), %xmm0
140a557b3: 0f 28 8d 40 02 00 00        	movaps	0x240(%rbp), %xmm1
140a557ba: 0f 11 4d d8                 	movups	%xmm1, -0x28(%rbp)
140a557be: f3 0f 7f 45 c8              	movdqu	%xmm0, -0x38(%rbp)
140a557c3: 48 89 45 c0                 	movq	%rax, -0x40(%rbp)
140a557c7: 48 89 75 f0                 	movq	%rsi, -0x10(%rbp)
140a557cb: c6 85 d6 05 00 00 01        	movb	$0x1, 0x5d6(%rbp)
140a557d2: 48 8d 8d a0 02 00 00        	leaq	0x2a0(%rbp), %rcx
140a557d9: ba 01 00 00 00              	movl	$0x1, %edx
140a557de: e8 4d 45 e8 ff              	callq	0x1408d9d30 <.text+0x8d8d30>
140a557e3: f3 0f 6f 85 a0 02 00 00     	movdqu	0x2a0(%rbp), %xmm0
140a557eb: 48 8b 9d b0 02 00 00        	movq	0x2b0(%rbp), %rbx
140a557f2: 4c 8b b5 b8 02 00 00        	movq	0x2b8(%rbp), %r14
140a557f9: 48 89 9d 30 05 00 00        	movq	%rbx, 0x530(%rbp)
140a55800: 4c 89 b5 38 05 00 00        	movq	%r14, 0x538(%rbp)
140a55807: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a5580b: 48 89 85 e0 02 00 00        	movq	%rax, 0x2e0(%rbp)
140a55812: 0f 10 4d c0                 	movups	-0x40(%rbp), %xmm1
140a55816: 0f 10 55 d0                 	movups	-0x30(%rbp), %xmm2
140a5581a: 0f 10 5d e0                 	movups	-0x20(%rbp), %xmm3
140a5581e: 0f 11 9d d0 02 00 00        	movups	%xmm3, 0x2d0(%rbp)
140a55825: 0f 11 95 c0 02 00 00        	movups	%xmm2, 0x2c0(%rbp)
140a5582c: 0f 11 8d b0 02 00 00        	movups	%xmm1, 0x2b0(%rbp)
140a55833: f3 0f 7f 85 a0 02 00 00     	movdqu	%xmm0, 0x2a0(%rbp)
140a5583b: 0f b6 05 c7 f3 bd 02        	movzbl	0x2bdf3c7(%rip), %eax   # 0x143634c09
140a55842: b9 48 00 00 00              	movl	$0x48, %ecx
140a55847: ba 08 00 00 00              	movl	$0x8, %edx
140a5584c: e8 bf e3 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a55851: 48 85 c0                    	testq	%rax, %rax
140a55854: 0f 84 a8 01 00 00           	je	0x140a55a02 <.text+0xa54a02>
140a5585a: 48 89 c7                    	movq	%rax, %rdi
140a5585d: 48 8b 85 e0 02 00 00        	movq	0x2e0(%rbp), %rax
140a55864: 48 89 47 40                 	movq	%rax, 0x40(%rdi)
140a55868: f3 0f 6f 85 a0 02 00 00     	movdqu	0x2a0(%rbp), %xmm0
140a55870: 0f 10 8d b0 02 00 00        	movups	0x2b0(%rbp), %xmm1
140a55877: 0f 10 95 c0 02 00 00        	movups	0x2c0(%rbp), %xmm2
140a5587e: 0f 10 9d d0 02 00 00        	movups	0x2d0(%rbp), %xmm3
140a55885: 0f 11 5f 30                 	movups	%xmm3, 0x30(%rdi)
140a55889: 0f 11 57 20                 	movups	%xmm2, 0x20(%rdi)
140a5588d: 0f 11 4f 10                 	movups	%xmm1, 0x10(%rdi)
140a55891: f3 0f 7f 07                 	movdqu	%xmm0, (%rdi)
140a55895: 0f b6 05 6d f3 bd 02        	movzbl	0x2bdf36d(%rip), %eax   # 0x143634c09
140a5589c: b9 20 00 00 00              	movl	$0x20, %ecx
140a558a1: ba 08 00 00 00              	movl	$0x8, %edx
140a558a6: e8 65 e3 bb ff              	callq	0x140613c10 <.text+0x612c10>
140a558ab: 48 85 c0                    	testq	%rax, %rax
140a558ae: 0f 84 5f 01 00 00           	je	0x140a55a13 <.text+0xa54a13>
140a558b4: 48 c7 00 01 00 00 00        	movq	$0x1, (%rax)
140a558bb: 48 c7 40 08 01 00 00 00     	movq	$0x1, 0x8(%rax)
140a558c3: 48 89 70 10                 	movq	%rsi, 0x10(%rax)
140a558c7: 48 8b 8d 70 05 00 00        	movq	0x570(%rbp), %rcx
140a558ce: 48 89 48 18                 	movq	%rcx, 0x18(%rax)
140a558d2: 49 89 47 10                 	movq	%rax, 0x10(%r15)
140a558d6: 49 89 3f                    	movq	%rdi, (%r15)
140a558d9: 48 8d 05 28 d2 07 02        	leaq	0x207d228(%rip), %rax   # 0x142ad2b08
140a558e0: 49 89 47 08                 	movq	%rax, 0x8(%r15)
140a558e4: 49 c7 47 18 01 00 00 00     	movq	$0x1, 0x18(%r15)
140a558ec: 66 41 c7 47 20 00 00        	movw	$0x0, 0x20(%r15)
140a558f3: 49 89 5f 28                 	movq	%rbx, 0x28(%r15)
140a558f7: 4d 89 77 30                 	movq	%r14, 0x30(%r15)
140a558fb: e9 25 f8 ff ff              	jmp	0x140a55125 <.text+0xa54125>
140a55900: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a55904: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
140a5590b: f3 0f 6f 45 c0              	movdqu	-0x40(%rbp), %xmm0
140a55910: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
140a55914: 0f 10 55 e0                 	movups	-0x20(%rbp), %xmm2
140a55918: 0f 29 95 c0 02 00 00        	movaps	%xmm2, 0x2c0(%rbp)
140a5591f: 0f 29 8d b0 02 00 00        	movaps	%xmm1, 0x2b0(%rbp)
140a55926: 66 0f 7f 85 a0 02 00 00     	movdqa	%xmm0, 0x2a0(%rbp)
140a5592e: 48 8d 05 33 cd 07 02        	leaq	0x207cd33(%rip), %rax   # 0x142ad2668
140a55935: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a5593a: 48 8d 0d bf ac 07 02        	leaq	0x207acbf(%rip), %rcx   # 0x142ad0600
140a55941: 4c 8d 0d 78 ac 07 02        	leaq	0x207ac78(%rip), %r9    # 0x142ad05c0
140a55948: 4c 8d 85 a0 02 00 00        	leaq	0x2a0(%rbp), %r8
140a5594f: ba 2b 00 00 00              	movl	$0x2b, %edx
140a55954: e8 07 3e e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a55959: e9 cb 00 00 00              	jmp	0x140a55a29 <.text+0xa54a29>
140a5595e: 48 8b 45 f0                 	movq	-0x10(%rbp), %rax
140a55962: 48 89 85 d0 02 00 00        	movq	%rax, 0x2d0(%rbp)
140a55969: f3 0f 6f 45 c0              	movdqu	-0x40(%rbp), %xmm0
140a5596e: 0f 10 4d d0                 	movups	-0x30(%rbp), %xmm1
140a55972: 0f 10 55 e0                 	movups	-0x20(%rbp), %xmm2
140a55976: 0f 29 95 c0 02 00 00        	movaps	%xmm2, 0x2c0(%rbp)
140a5597d: 0f 29 8d b0 02 00 00        	movaps	%xmm1, 0x2b0(%rbp)
140a55984: 66 0f 7f 85 a0 02 00 00     	movdqa	%xmm0, 0x2a0(%rbp)
140a5598c: 48 8d 05 ed cc 07 02        	leaq	0x207cced(%rip), %rax   # 0x142ad2680
140a55993: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a55998: 48 8d 0d 61 ac 07 02        	leaq	0x207ac61(%rip), %rcx   # 0x142ad0600
140a5599f: 4c 8d 0d 1a ac 07 02        	leaq	0x207ac1a(%rip), %r9    # 0x142ad05c0
140a559a6: 4c 8d 85 a0 02 00 00        	leaq	0x2a0(%rbp), %r8
140a559ad: ba 2b 00 00 00              	movl	$0x2b, %edx
140a559b2: e8 a9 3d e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a559b7: eb 70                       	jmp	0x140a55a29 <.text+0xa54a29>
140a559b9: 48 89 b5 a0 02 00 00        	movq	%rsi, 0x2a0(%rbp)
140a559c0: 48 8d 05 d9 cd 07 02        	leaq	0x207cdd9(%rip), %rax   # 0x142ad27a0
140a559c7: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
140a559cc: 48 8d 0d b3 cd 07 02        	leaq	0x207cdb3(%rip), %rcx   # 0x142ad2786
140a559d3: 4c 8d 0d 06 ac 07 02        	leaq	0x207ac06(%rip), %r9    # 0x142ad05e0
140a559da: 4c 8d 85 a0 02 00 00        	leaq	0x2a0(%rbp), %r8
140a559e1: ba 14 00 00 00              	movl	$0x14, %edx
140a559e6: e8 75 3d e8 01              	callq	0x1428d9760 <.text+0x28d8760>
140a559eb: eb 3c                       	jmp	0x140a55a29 <.text+0xa54a29>
140a559ed: c6 85 d7 05 00 00 01        	movb	$0x1, 0x5d7(%rbp)
140a559f4: 48 8d 0d a5 50 1e 02        	leaq	0x21e50a5(%rip), %rcx   # 0x142c3aaa0
140a559fb: e8 d0 26 e8 01              	callq	0x1428d80d0 <.text+0x28d70d0>
140a55a00: eb 27                       	jmp	0x140a55a29 <.text+0xa54a29>
140a55a02: b9 08 00 00 00              	movl	$0x8, %ecx
140a55a07: ba 48 00 00 00              	movl	$0x48, %edx
140a55a0c: e8 ef 35 e8 01              	callq	0x1428d9000 <.text+0x28d8000>
140a55a11: eb 16                       	jmp	0x140a55a29 <.text+0xa54a29>
140a55a13: 48 89 bd 70 05 00 00        	movq	%rdi, 0x570(%rbp)
140a55a1a: b9 08 00 00 00              	movl	$0x8, %ecx
140a55a1f: ba 20 00 00 00              	movl	$0x20, %edx
140a55a24: e8 d7 35 e8 01              	callq	0x1428d9000 <.text+0x28d8000>
140a55a29: 0f 0b                       	ud2
140a55a2b: 0f 1f 44 00 00              	nopl	(%rax,%rax)
140a55a30: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a55a35: 55                          	pushq	%rbp
140a55a36: 41 57                       	pushq	%r15
140a55a38: 41 56                       	pushq	%r14
140a55a3a: 41 55                       	pushq	%r13
140a55a3c: 41 54                       	pushq	%r12
140a55a3e: 56                          	pushq	%rsi
140a55a3f: 57                          	pushq	%rdi
140a55a40: 53                          	pushq	%rbx
140a55a41: 48 83 ec 68                 	subq	$0x68, %rsp
140a55a45: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a55a4c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a55a53: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a55a59: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a55a5e: 48 8d 8d 30 05 00 00        	leaq	0x530(%rbp), %rcx
140a55a65: 41 b8 48 00 00 00           	movl	$0x48, %r8d
140a55a6b: 41 b9 10 00 00 00           	movl	$0x10, %r9d
140a55a71: 48 8d 95 50 05 00 00        	leaq	0x550(%rbp), %rdx
140a55a78: e8 e3 f9 11 00              	callq	0x140b75460 <.text+0xb74460>
140a55a7d: c6 85 d7 05 00 00 00        	movb	$0x0, 0x5d7(%rbp)
140a55a84: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a55a89: 66 0f 6f 7c 24 40           	movdqa	0x40(%rsp), %xmm7
140a55a8f: 66 44 0f 6f 44 24 30        	movdqa	0x30(%rsp), %xmm8
140a55a96: 48 83 c4 68                 	addq	$0x68, %rsp
140a55a9a: 5b                          	popq	%rbx
140a55a9b: 5f                          	popq	%rdi
140a55a9c: 5e                          	popq	%rsi
140a55a9d: 41 5c                       	popq	%r12
140a55a9f: 41 5d                       	popq	%r13
140a55aa1: 41 5e                       	popq	%r14
140a55aa3: 41 5f                       	popq	%r15
140a55aa5: 5d                          	popq	%rbp
140a55aa6: c3                          	retq
140a55aa7: 66 0f 1f 84 00 00 00 00 00  	nopw	(%rax,%rax)
140a55ab0: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a55ab5: 55                          	pushq	%rbp
140a55ab6: 41 57                       	pushq	%r15
140a55ab8: 41 56                       	pushq	%r14
140a55aba: 41 55                       	pushq	%r13
140a55abc: 41 54                       	pushq	%r12
140a55abe: 56                          	pushq	%rsi
140a55abf: 57                          	pushq	%rdi
140a55ac0: 53                          	pushq	%rbx
140a55ac1: 48 83 ec 68                 	subq	$0x68, %rsp
140a55ac5: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a55acc: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
140a55ad3: 66 0f 7f 7c 24 40           	movdqa	%xmm7, 0x40(%rsp)
140a55ad9: 0f 29 74 24 50              	movaps	%xmm6, 0x50(%rsp)
140a55ade: 80 bd d7 05 00 00 00        	cmpb	$0x0, 0x5d7(%rbp)
140a55ae5: 75 21                       	jne	0x140a55b08 <.text+0xa54b08>
140a55ae7: 0f 28 74 24 50              	movaps	0x50(%rsp), %xmm6
140a55aec: 0f 28 7c 24 40              	movaps	0x40(%rsp), %xmm7
140a55af1: 44 0f 28 44 24 30           	movaps	0x30(%rsp), %xmm8
140a55af7: 48 83 c4 68                 	addq	$0x68, %rsp
140a55afb: 5b                          	popq	%rbx
140a55afc: 5f                          	popq	%rdi
140a55afd: 5e                          	popq	%rsi
140a55afe: 41 5c                       	popq	%r12
140a55b00: 41 5d                       	popq	%r13
140a55b02: 41 5e                       	popq	%r14
140a55b04: 41 5f                       	popq	%r15
140a55b06: 5d                          	popq	%rbp
140a55b07: c3                          	retq
140a55b08: 48 8d 4d c0                 	leaq	-0x40(%rbp), %rcx
140a55b0c: e8 df 0e b9 ff              	callq	0x1405e69f0 <.text+0x5e59f0>
140a55b11: eb d4                       	jmp	0x140a55ae7 <.text+0xa54ae7>
140a55b13: 66 66 66 66 2e 0f 1f 84 00 00 00 00 00      	nopw	%cs:(%rax,%rax)
140a55b20: 48 89 54 24 10              	movq	%rdx, 0x10(%rsp)
140a55b25: 55                          	pushq	%rbp
140a55b26: 41 57                       	pushq	%r15
140a55b28: 41 56                       	pushq	%r14
140a55b2a: 41 55                       	pushq	%r13
140a55b2c: 41 54                       	pushq	%r12
140a55b2e: 56                          	pushq	%rsi
140a55b2f: 57                          	pushq	%rdi
140a55b30: 53                          	pushq	%rbx
140a55b31: 48 83 ec 68                 	subq	$0x68, %rsp
140a55b35: 48 8d aa 80 00 00 00        	leaq	0x80(%rdx), %rbp
140a55b3c: 66 44 0f 7f 44 24 30        	movdqa	%xmm8, 0x30(%rsp)
