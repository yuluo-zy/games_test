
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14172b280: 55                          	pushq	%rbp
14172b281: 41 57                       	pushq	%r15
14172b283: 41 56                       	pushq	%r14
14172b285: 41 55                       	pushq	%r13
14172b287: 41 54                       	pushq	%r12
14172b289: 56                          	pushq	%rsi
14172b28a: 57                          	pushq	%rdi
14172b28b: 53                          	pushq	%rbx
14172b28c: 48 81 ec 18 02 00 00        	subq	$0x218, %rsp            # imm = 0x218
14172b293: 48 8d ac 24 80 00 00 00     	leaq	0x80(%rsp), %rbp
14172b29b: 66 0f 7f b5 80 01 00 00     	movdqa	%xmm6, 0x180(%rbp)
14172b2a3: 48 c7 85 78 01 00 00 fe ff ff ff    	movq	$-0x2, 0x178(%rbp)
14172b2ae: 4c 89 c7                    	movq	%r8, %rdi
14172b2b1: 48 89 95 a0 00 00 00        	movq	%rdx, 0xa0(%rbp)
14172b2b8: 48 89 ce                    	movq	%rcx, %rsi
14172b2bb: 48 8b 85 00 02 00 00        	movq	0x200(%rbp), %rax
14172b2c2: 4c 89 4d f0                 	movq	%r9, -0x10(%rbp)
14172b2c6: 48 89 45 f8                 	movq	%rax, -0x8(%rbp)
14172b2ca: c6 85 73 01 00 00 01        	movb	$0x1, 0x173(%rbp)
14172b2d1: c6 85 72 01 00 00 01        	movb	$0x1, 0x172(%rbp)
14172b2d8: 48 89 4d e8                 	movq	%rcx, -0x18(%rbp)
14172b2dc: 4c 89 45 e0                 	movq	%r8, -0x20(%rbp)
14172b2e0: e8 fb 82 63 ff              	callq	0x140d635e0 <.text+0xd625e0>
14172b2e5: 48 89 45 a0                 	movq	%rax, -0x60(%rbp)
14172b2e9: 89 55 a8                    	movl	%edx, -0x58(%rbp)
14172b2ec: f3 0f 6f 07                 	movdqu	(%rdi), %xmm0
14172b2f0: f3 0f 6f 4f 10              	movdqu	0x10(%rdi), %xmm1
14172b2f5: f3 0f 6f 57 20              	movdqu	0x20(%rdi), %xmm2
14172b2fa: 66 0f 7f 55 d0              	movdqa	%xmm2, -0x30(%rbp)
14172b2ff: 66 0f 7f 4d c0              	movdqa	%xmm1, -0x40(%rbp)
14172b304: 66 0f 7f 45 b0              	movdqa	%xmm0, -0x50(%rbp)
14172b309: 48 8b 0e                    	movq	(%rsi), %rcx
14172b30c: 48 8b 46 08                 	movq	0x8(%rsi), %rax
14172b310: 4c 8b 66 10                 	movq	0x10(%rsi), %r12
14172b314: 49 c1 e4 05                 	shlq	$0x5, %r12
14172b318: 49 01 c4                    	addq	%rax, %r12
14172b31b: 48 89 85 a8 00 00 00        	movq	%rax, 0xa8(%rbp)
14172b322: 48 89 85 b0 00 00 00        	movq	%rax, 0xb0(%rbp)
14172b329: 48 89 8d b8 00 00 00        	movq	%rcx, 0xb8(%rbp)
14172b330: 4c 89 a5 c0 00 00 00        	movq	%r12, 0xc0(%rbp)
14172b337: 4c 8d 6d 78                 	leaq	0x78(%rbp), %r13
14172b33b: 48 8d b5 d8 00 00 00        	leaq	0xd8(%rbp), %rsi
14172b342: 66 0f ef f6                 	pxor	%xmm6, %xmm6
14172b346: 48 8d 7d 70                 	leaq	0x70(%rbp), %rdi
14172b34a: 48 8d 9d d0 00 00 00        	leaq	0xd0(%rbp), %rbx
14172b351: 4c 8d 7d f0                 	leaq	-0x10(%rbp), %r15
14172b355: 4c 39 e0                    	cmpq	%r12, %rax
14172b358: 0f 84 c1 04 00 00           	je	0x14172b81f <.text+0x172a81f>
14172b35e: 66 90                       	nop
14172b360: 4c 8d 70 20                 	leaq	0x20(%rax), %r14
14172b364: 4c 89 b5 b0 00 00 00        	movq	%r14, 0xb0(%rbp)
14172b36b: 48 8b 08                    	movq	(%rax), %rcx
14172b36e: 48 8b 50 18                 	movq	0x18(%rax), %rdx
14172b372: 48 89 55 10                 	movq	%rdx, 0x10(%rbp)
14172b376: 0f 10 40 08                 	movups	0x8(%rax), %xmm0
14172b37a: 0f 29 45 00                 	movaps	%xmm0, (%rbp)
14172b37e: 48 89 c8                    	movq	%rcx, %rax
14172b381: 48 f7 d8                    	negq	%rax
14172b384: 0f 80 98 04 00 00           	jo	0x14172b822 <.text+0x172a822>
14172b38a: 48 89 4d 70                 	movq	%rcx, 0x70(%rbp)
14172b38e: 48 8b 45 10                 	movq	0x10(%rbp), %rax
14172b392: 49 89 45 10                 	movq	%rax, 0x10(%r13)
14172b396: 0f 28 45 00                 	movaps	(%rbp), %xmm0
14172b39a: 41 0f 11 45 00              	movups	%xmm0, (%r13)
14172b39f: c7 85 d0 00 00 00 00 00 00 00       	movl	$0x0, 0xd0(%rbp)
14172b3a9: f3 0f 7f 36                 	movdqu	%xmm6, (%rsi)
14172b3ad: c7 85 e8 00 00 00 07 00 00 00       	movl	$0x7, 0xe8(%rbp)
14172b3b7: 66 c7 46 1c 00 00           	movw	$0x0, 0x1c(%rsi)
14172b3bd: 48 c7 46 14 00 00 00 00     	movq	$0x0, 0x14(%rsi)
14172b3c5: c6 85 f0 00 00 00 01        	movb	$0x1, 0xf0(%rbp)
14172b3cc: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
14172b3d3: 48 89 f9                    	movq	%rdi, %rcx
14172b3d6: e8 f5 ba 64 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14172b3db: 49 89 d0                    	movq	%rdx, %r8
14172b3de: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
14172b3e5: 48 89 d9                    	movq	%rbx, %rcx
14172b3e8: 48 89 c2                    	movq	%rax, %rdx
14172b3eb: e8 d0 e9 62 ff              	callq	0x140d59dc0 <.text+0xd58dc0>
14172b3f0: 48 85 c0                    	testq	%rax, %rax
14172b3f3: 0f 84 97 01 00 00           	je	0x14172b590 <.text+0x172a590>
14172b3f9: 48 89 95 90 00 00 00        	movq	%rdx, 0x90(%rbp)
14172b400: 48 8b 05 a1 0f f0 01        	movq	0x1f00fa1(%rip), %rax   # 0x14362c3a8
14172b407: 48 8b 00                    	movq	(%rax), %rax
14172b40a: 48 85 c0                    	testq	%rax, %rax
14172b40d: 0f 84 06 01 00 00           	je	0x14172b519 <.text+0x172a519>
14172b413: 48 89 f9                    	movq	%rdi, %rcx
14172b416: e8 b5 ba 64 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14172b41b: 48 89 c1                    	movq	%rax, %rcx
14172b41e: e8 dd 69 63 ff              	callq	0x140d61e00 <.text+0xd60e00>
14172b423: 48 89 45 20                 	movq	%rax, 0x20(%rbp)
14172b427: 48 89 55 28                 	movq	%rdx, 0x28(%rbp)
14172b42b: 48 8d 45 20                 	leaq	0x20(%rbp), %rax
14172b42f: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14172b433: 48 8d 05 46 38 ff ff        	leaq	-0xc7ba(%rip), %rax     # 0x14171ec80 <.text+0x171dc80>
14172b43a: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14172b43e: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
14172b445: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14172b449: 48 8d 05 90 f8 62 ff        	leaq	-0x9d0770(%rip), %rax   # 0x140d5ace0 <.text+0xd59ce0>
14172b450: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14172b454: 48 8d 0d a5 fc 59 01        	leaq	0x159fca5(%rip), %rcx   # 0x142ccb100
14172b45b: e8 20 29 f5 fe              	callq	0x14067dd80 <.text+0x67cd80>
14172b460: 0f 10 00                    	movups	(%rax), %xmm0
14172b463: 8b 40 10                    	movl	0x10(%rax), %eax
14172b466: 48 c7 85 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rbp)
14172b471: 48 8d 0d e8 f0 59 01        	leaq	0x159f0e8(%rip), %rcx   # 0x142cca560
14172b478: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14172b47f: 48 c7 85 10 01 00 00 28 00 00 00    	movq	$0x28, 0x110(%rbp)
14172b48a: 48 8d 15 4f fc 59 01        	leaq	0x159fc4f(%rip), %rdx   # 0x142ccb0e0
14172b491: 48 89 95 20 01 00 00        	movq	%rdx, 0x120(%rbp)
14172b498: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
14172b4a3: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
14172b4a7: 48 89 95 30 01 00 00        	movq	%rdx, 0x130(%rbp)
14172b4ae: 48 c7 85 38 01 00 00 02 00 00 00    	movq	$0x2, 0x138(%rbp)
14172b4b9: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
14172b4c4: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
14172b4cf: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14172b4d6: 48 c7 85 e0 00 00 00 28 00 00 00    	movq	$0x28, 0xe0(%rbp)
14172b4e1: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
14172b4ec: 0f 11 85 f0 00 00 00        	movups	%xmm0, 0xf0(%rbp)
14172b4f3: c7 85 18 01 00 00 01 00 00 00       	movl	$0x1, 0x118(%rbp)
14172b4fd: 89 85 1c 01 00 00           	movl	%eax, 0x11c(%rbp)
14172b503: 48 8d 8d 74 01 00 00        	leaq	0x174(%rbp), %rcx
14172b50a: 48 89 da                    	movq	%rbx, %rdx
14172b50d: e8 1e ee 61 ff              	callq	0x140d4a330 <.text+0xd49330>
14172b512: 48 8b 95 90 00 00 00        	movq	0x90(%rbp), %rdx
14172b519: 89 d0                       	movl	%edx, %eax
14172b51b: 83 e0 03                    	andl	$0x3, %eax
14172b51e: 83 f8 01                    	cmpl	$0x1, %eax
14172b521: 0f 85 9e 02 00 00           	jne	0x14172b7c5 <.text+0x172a7c5>
14172b527: 48 8d 42 ff                 	leaq	-0x1(%rdx), %rax
14172b52b: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14172b532: 48 8b 42 ff                 	movq	-0x1(%rdx), %rax
14172b536: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
14172b53d: 48 8b 42 07                 	movq	0x7(%rdx), %rax
14172b541: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14172b548: 48 8b 00                    	movq	(%rax), %rax
14172b54b: 48 85 c0                    	testq	%rax, %rax
14172b54e: 74 09                       	je	0x14172b559 <.text+0x172a559>
14172b550: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14172b557: ff d0                       	callq	*%rax
14172b559: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14172b560: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14172b567: 48 8b 50 08                 	movq	0x8(%rax), %rdx
14172b56b: 48 85 d2                    	testq	%rdx, %rdx
14172b56e: 74 09                       	je	0x14172b579 <.text+0x172a579>
14172b570: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14172b574: e8 a7 86 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172b579: ba 18 00 00 00              	movl	$0x18, %edx
14172b57e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14172b584: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
14172b58b: e9 30 02 00 00              	jmp	0x14172b7c0 <.text+0x172a7c0>
14172b590: 49 89 d6                    	movq	%rdx, %r14
14172b593: 0f b6 05 6f 96 f0 01        	movzbl	0x1f0966f(%rip), %eax   # 0x143634c09
14172b59a: b9 00 20 00 00              	movl	$0x2000, %ecx           # imm = 0x2000
14172b59f: ba 01 00 00 00              	movl	$0x1, %edx
14172b5a4: e8 67 86 ee fe              	callq	0x140613c10 <.text+0x612c10>
14172b5a9: 48 85 c0                    	testq	%rax, %rax
14172b5ac: 0f 84 8c 0c 00 00           	je	0x14172c23e <.text+0x172b23e>
14172b5b2: 48 89 85 d0 00 00 00        	movq	%rax, 0xd0(%rbp)
14172b5b9: 48 c7 85 d8 00 00 00 00 20 00 00    	movq	$0x2000, 0xd8(%rbp) # imm = 0x2000
14172b5c4: f3 0f 7f 76 08              	movdqu	%xmm6, 0x8(%rsi)
14172b5c9: 48 c7 46 18 00 00 00 00     	movq	$0x0, 0x18(%rsi)
14172b5d1: 4c 89 b5 f8 00 00 00        	movq	%r14, 0xf8(%rbp)
14172b5d8: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
14172b5df: 48 8d 4d 20                 	leaq	0x20(%rbp), %rcx
14172b5e3: 48 89 da                    	movq	%rbx, %rdx
14172b5e6: e8 45 b1 b5 ff              	callq	0x141286730 <.text+0x1285730>
14172b5eb: 48 8b 45 20                 	movq	0x20(%rbp), %rax
14172b5ef: 48 89 c1                    	movq	%rax, %rcx
14172b5f2: 48 f7 d9                    	negq	%rcx
14172b5f5: 0f 81 6a 01 00 00           	jno	0x14172b765 <.text+0x172a765>
14172b5fb: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
14172b5ff: 48 89 8d 58 01 00 00        	movq	%rcx, 0x158(%rbp)
14172b606: 48 8b 05 9b 0d f0 01        	movq	0x1f00d9b(%rip), %rax   # 0x14362c3a8
14172b60d: 48 8b 00                    	movq	(%rax), %rax
14172b610: 48 85 c0                    	testq	%rax, %rax
14172b613: 0f 84 0f 01 00 00           	je	0x14172b728 <.text+0x172a728>
14172b619: 48 89 f9                    	movq	%rdi, %rcx
14172b61c: e8 af b8 64 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14172b621: 48 89 c1                    	movq	%rax, %rcx
14172b624: e8 d7 67 63 ff              	callq	0x140d61e00 <.text+0xd60e00>
14172b629: 48 89 85 90 00 00 00        	movq	%rax, 0x90(%rbp)
14172b630: 48 89 95 98 00 00 00        	movq	%rdx, 0x98(%rbp)
14172b637: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
14172b63e: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14172b642: 48 8d 05 37 36 ff ff        	leaq	-0xc9c9(%rip), %rax     # 0x14171ec80 <.text+0x171dc80>
14172b649: 48 89 45 48                 	movq	%rax, 0x48(%rbp)
14172b64d: 48 8d 85 58 01 00 00        	leaq	0x158(%rbp), %rax
14172b654: 48 89 45 50                 	movq	%rax, 0x50(%rbp)
14172b658: 48 8d 05 a1 ba 45 ff        	leaq	-0xba455f(%rip), %rax   # 0x140b87100 <.text+0xb86100>
14172b65f: 48 89 45 58                 	movq	%rax, 0x58(%rbp)
14172b663: 48 8d 0d 3e fa 59 01        	leaq	0x159fa3e(%rip), %rcx   # 0x142ccb0a8
14172b66a: e8 11 27 f5 fe              	callq	0x14067dd80 <.text+0x67cd80>
14172b66f: 0f 10 00                    	movups	(%rax), %xmm0
14172b672: 8b 40 10                    	movl	0x10(%rax), %eax
14172b675: 48 c7 85 00 01 00 00 01 00 00 00    	movq	$0x1, 0x100(%rbp)
14172b680: 48 8d 0d d9 ee 59 01        	leaq	0x159eed9(%rip), %rcx   # 0x142cca560
14172b687: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14172b68e: 48 c7 85 10 01 00 00 28 00 00 00    	movq	$0x28, 0x110(%rbp)
14172b699: 48 8d 15 e8 f9 59 01        	leaq	0x159f9e8(%rip), %rdx   # 0x142ccb088
14172b6a0: 48 89 95 20 01 00 00        	movq	%rdx, 0x120(%rbp)
14172b6a7: 48 c7 85 28 01 00 00 02 00 00 00    	movq	$0x2, 0x128(%rbp)
14172b6b2: 48 8d 55 40                 	leaq	0x40(%rbp), %rdx
14172b6b6: 48 89 95 30 01 00 00        	movq	%rdx, 0x130(%rbp)
14172b6bd: 48 c7 85 38 01 00 00 02 00 00 00    	movq	$0x2, 0x138(%rbp)
14172b6c8: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
14172b6d3: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
14172b6de: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14172b6e5: 48 c7 85 e0 00 00 00 28 00 00 00    	movq	$0x28, 0xe0(%rbp)
14172b6f0: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
14172b6fb: 0f 11 85 f0 00 00 00        	movups	%xmm0, 0xf0(%rbp)
14172b702: c7 85 18 01 00 00 01 00 00 00       	movl	$0x1, 0x118(%rbp)
14172b70c: 89 85 1c 01 00 00           	movl	%eax, 0x11c(%rbp)
14172b712: 48 8d 8d 74 01 00 00        	leaq	0x174(%rbp), %rcx
14172b719: 48 89 da                    	movq	%rbx, %rdx
14172b71c: e8 0f ec 61 ff              	callq	0x140d4a330 <.text+0xd49330>
14172b721: 48 8b 8d 58 01 00 00        	movq	0x158(%rbp), %rcx
14172b728: 48 8b 01                    	movq	(%rcx), %rax
14172b72b: 48 83 f8 01                 	cmpq	$0x1, %rax
14172b72f: 48 89 8d 68 01 00 00        	movq	%rcx, 0x168(%rbp)
14172b736: 74 6d                       	je	0x14172b7a5 <.text+0x172a7a5>
14172b738: 48 85 c0                    	testq	%rax, %rax
14172b73b: 75 71                       	jne	0x14172b7ae <.text+0x172a7ae>
14172b73d: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14172b744: 48 8b 50 10                 	movq	0x10(%rax), %rdx
14172b748: 48 85 d2                    	testq	%rdx, %rdx
14172b74b: 74 61                       	je	0x14172b7ae <.text+0x172a7ae>
14172b74d: 48 8b 85 68 01 00 00        	movq	0x168(%rbp), %rax
14172b754: 48 8b 48 08                 	movq	0x8(%rax), %rcx
14172b758: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172b75e: e8 bd 84 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172b763: eb 49                       	jmp	0x14172b7ae <.text+0x172a7ae>
14172b765: 48 8b 4d 28                 	movq	0x28(%rbp), %rcx
14172b769: 48 8b 55 30                 	movq	0x30(%rbp), %rdx
14172b76d: 48 c1 e2 04                 	shlq	$0x4, %rdx
14172b771: 48 01 ca                    	addq	%rcx, %rdx
14172b774: 48 89 8d d0 00 00 00        	movq	%rcx, 0xd0(%rbp)
14172b77b: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14172b782: 48 89 85 e0 00 00 00        	movq	%rax, 0xe0(%rbp)
14172b789: 48 89 95 e8 00 00 00        	movq	%rdx, 0xe8(%rbp)
14172b790: c6 85 77 01 00 00 01        	movb	$0x1, 0x177(%rbp)
14172b797: 48 8d 4d b0                 	leaq	-0x50(%rbp), %rcx
14172b79b: 48 89 da                    	movq	%rbx, %rdx
14172b79e: e8 dd 7e df ff              	callq	0x141523680 <.text+0x1522680>
14172b7a3: eb 20                       	jmp	0x14172b7c5 <.text+0x172a7c5>
14172b7a5: 48 83 c1 08                 	addq	$0x8, %rcx
14172b7a9: e8 52 c2 eb fe              	callq	0x1405e7a00 <.text+0x5e6a00>
14172b7ae: ba 28 00 00 00              	movl	$0x28, %edx
14172b7b3: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14172b7b9: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14172b7c0: e8 5b 84 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172b7c5: f3 0f 6f 45 70              	movdqu	0x70(%rbp), %xmm0
14172b7ca: f3 0f 6f 8d 80 00 00 00     	movdqu	0x80(%rbp), %xmm1
14172b7d2: 66 0f 7f 8d e0 00 00 00     	movdqa	%xmm1, 0xe0(%rbp)
14172b7da: 66 0f 7f 85 d0 00 00 00     	movdqa	%xmm0, 0xd0(%rbp)
14172b7e2: c6 85 77 01 00 00 00        	movb	$0x0, 0x177(%rbp)
14172b7e9: 48 8d 4d 40                 	leaq	0x40(%rbp), %rcx
14172b7ed: 4c 89 fa                    	movq	%r15, %rdx
14172b7f0: 49 89 d8                    	movq	%rbx, %r8
14172b7f3: e8 48 e8 1a ff              	callq	0x1408da040 <.text+0x8d9040>
14172b7f8: 48 8b 55 40                 	movq	0x40(%rbp), %rdx
14172b7fc: 48 89 d0                    	movq	%rdx, %rax
14172b7ff: 48 f7 d8                    	negq	%rax
14172b802: 0f 81 14 08 00 00           	jno	0x14172c01c <.text+0x172b01c>
14172b808: 48 8b 85 b0 00 00 00        	movq	0xb0(%rbp), %rax
14172b80f: 4c 8b a5 c0 00 00 00        	movq	0xc0(%rbp), %r12
14172b816: 4c 39 e0                    	cmpq	%r12, %rax
14172b819: 0f 85 41 fb ff ff           	jne	0x14172b360 <.text+0x172a360>
14172b81f: 49 89 c6                    	movq	%rax, %r14
14172b822: 4d 29 f4                    	subq	%r14, %r12
14172b825: 74 3b                       	je	0x14172b862 <.text+0x172a862>
14172b827: 49 c1 ec 05                 	shrq	$0x5, %r12
14172b82b: 49 83 c6 08                 	addq	$0x8, %r14
14172b82f: eb 18                       	jmp	0x14172b849 <.text+0x172a849>
14172b831: 66 66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00	nopw	%cs:(%rax,%rax)
14172b840: 49 83 c6 20                 	addq	$0x20, %r14
14172b844: 49 ff cc                    	decq	%r12
14172b847: 74 19                       	je	0x14172b862 <.text+0x172a862>
14172b849: 49 8b 56 f8                 	movq	-0x8(%r14), %rdx
14172b84d: 48 85 d2                    	testq	%rdx, %rdx
14172b850: 74 ee                       	je	0x14172b840 <.text+0x172a840>
14172b852: 49 8b 0e                    	movq	(%r14), %rcx
14172b855: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172b85b: e8 c0 83 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172b860: eb de                       	jmp	0x14172b840 <.text+0x172a840>
14172b862: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14172b869: 48 85 d2                    	testq	%rdx, %rdx
14172b86c: 74 16                       	je	0x14172b884 <.text+0x172a884>
14172b86e: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14172b875: 48 c1 e2 05                 	shlq	$0x5, %rdx
14172b879: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14172b87f: e8 9c 83 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172b884: 48 8b 8d a0 00 00 00        	movq	0xa0(%rbp), %rcx
14172b88b: 48 8b 01                    	movq	(%rcx), %rax
14172b88e: 48 8b 71 08                 	movq	0x8(%rcx), %rsi
14172b892: 4c 8b 79 10                 	movq	0x10(%rcx), %r15
14172b896: 66 0f 6f 45 b0              	movdqa	-0x50(%rbp), %xmm0
14172b89b: 66 0f 6f 4d c0              	movdqa	-0x40(%rbp), %xmm1
14172b8a0: 66 0f 6f 55 d0              	movdqa	-0x30(%rbp), %xmm2
14172b8a5: 66 0f 7f 55 60              	movdqa	%xmm2, 0x60(%rbp)
14172b8aa: 66 0f 7f 4d 50              	movdqa	%xmm1, 0x50(%rbp)
14172b8af: 66 0f 7f 45 40              	movdqa	%xmm0, 0x40(%rbp)
14172b8b4: 49 c1 e7 05                 	shlq	$0x5, %r15
14172b8b8: 49 01 f7                    	addq	%rsi, %r15
14172b8bb: 48 89 b5 a8 00 00 00        	movq	%rsi, 0xa8(%rbp)
14172b8c2: 48 89 b5 b0 00 00 00        	movq	%rsi, 0xb0(%rbp)
14172b8c9: 48 89 85 b8 00 00 00        	movq	%rax, 0xb8(%rbp)
14172b8d0: 4c 89 bd c0 00 00 00        	movq	%r15, 0xc0(%rbp)
14172b8d7: 48 8d 7d 70                 	leaq	0x70(%rbp), %rdi
14172b8db: 4c 8d b5 d0 00 00 00        	leaq	0xd0(%rbp), %r14
14172b8e2: 66 0f 76 f6                 	pcmpeqd	%xmm6, %xmm6
14172b8e6: eb 0b                       	jmp	0x14172b8f3 <.text+0x172a8f3>
14172b8e8: 0f 1f 84 00 00 00 00 00     	nopl	(%rax,%rax)
14172b8f0: 4c 89 e6                    	movq	%r12, %rsi
14172b8f3: 4c 39 fe                    	cmpq	%r15, %rsi
14172b8f6: 0f 84 6a 05 00 00           	je	0x14172be66 <.text+0x172ae66>
14172b8fc: 4c 8d 66 20                 	leaq	0x20(%rsi), %r12
14172b900: 4c 89 a5 b0 00 00 00        	movq	%r12, 0xb0(%rbp)
14172b907: 48 8b 06                    	movq	(%rsi), %rax
14172b90a: 48 8b 4e 18                 	movq	0x18(%rsi), %rcx
14172b90e: 48 89 4d 30                 	movq	%rcx, 0x30(%rbp)
14172b912: 0f 10 46 08                 	movups	0x8(%rsi), %xmm0
14172b916: 0f 29 45 20                 	movaps	%xmm0, 0x20(%rbp)
14172b91a: 48 89 c1                    	movq	%rax, %rcx
14172b91d: 48 f7 d9                    	negq	%rcx
14172b920: 0f 80 3d 05 00 00           	jo	0x14172be63 <.text+0x172ae63>
14172b926: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14172b92a: 48 8b 45 30                 	movq	0x30(%rbp), %rax
14172b92e: 49 89 45 10                 	movq	%rax, 0x10(%r13)
14172b932: 66 0f 6f 45 20              	movdqa	0x20(%rbp), %xmm0
14172b937: f3 41 0f 7f 45 00           	movdqu	%xmm0, (%r13)
14172b93d: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172b944: 48 89 f9                    	movq	%rdi, %rcx
14172b947: e8 84 b5 64 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14172b94c: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172b953: 48 89 c1                    	movq	%rax, %rcx
14172b956: e8 45 6e 63 ff              	callq	0x140d627a0 <.text+0xd617a0>
14172b95b: 48 85 c0                    	testq	%rax, %rax
14172b95e: 74 3e                       	je	0x14172b99e <.text+0x172a99e>
14172b960: 49 89 d0                    	movq	%rdx, %r8
14172b963: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172b96a: 4c 89 f1                    	movq	%r14, %rcx
14172b96d: 48 89 c2                    	movq	%rax, %rdx
14172b970: e8 fb d4 62 ff              	callq	0x140d58e70 <.text+0xd57e70>
14172b975: f6 85 d0 00 00 00 01        	testb	$0x1, 0xd0(%rbp)
14172b97c: 75 20                       	jne	0x14172b99e <.text+0x172a99e>
14172b97e: 48 83 bd e0 00 00 00 08     	cmpq	$0x8, 0xe0(%rbp)
14172b986: 75 16                       	jne	0x14172b99e <.text+0x172a99e>
14172b988: 48 8b 85 d8 00 00 00        	movq	0xd8(%rbp), %rax
14172b98f: 48 b9 73 6e 61 70 73 68 6f 74       	movabsq	$0x746f687370616e73, %rcx # imm = 0x746F687370616E73
14172b999: 48 39 08                    	cmpq	%rcx, (%rax)
14172b99c: 74 21                       	je	0x14172b9bf <.text+0x172a9bf>
14172b99e: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
14172b9a2: 48 85 d2                    	testq	%rdx, %rdx
14172b9a5: 0f 84 45 ff ff ff           	je	0x14172b8f0 <.text+0x172a8f0>
14172b9ab: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14172b9af: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172b9b5: e8 66 82 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172b9ba: e9 31 ff ff ff              	jmp	0x14172b8f0 <.text+0x172a8f0>
14172b9bf: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172b9c6: 48 89 f9                    	movq	%rdi, %rcx
14172b9c9: e8 02 b5 64 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14172b9ce: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172b9d5: 48 89 c1                    	movq	%rax, %rcx
14172b9d8: e8 23 6d 63 ff              	callq	0x140d62700 <.text+0xd61700>
14172b9dd: 48 85 c0                    	testq	%rax, %rax
14172b9e0: 74 1e                       	je	0x14172ba00 <.text+0x172aa00>
14172b9e2: 49 89 d0                    	movq	%rdx, %r8
14172b9e5: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172b9ec: 4c 89 f1                    	movq	%r14, %rcx
14172b9ef: 48 89 c2                    	movq	%rax, %rdx
14172b9f2: e8 79 d4 62 ff              	callq	0x140d58e70 <.text+0xd57e70>
14172b9f7: f6 85 d0 00 00 00 01        	testb	$0x1, 0xd0(%rbp)
14172b9fe: 74 20                       	je	0x14172ba20 <.text+0x172aa20>
14172ba00: 48 8b 55 70                 	movq	0x70(%rbp), %rdx
14172ba04: 48 85 d2                    	testq	%rdx, %rdx
14172ba07: 74 0f                       	je	0x14172ba18 <.text+0x172aa18>
14172ba09: 48 8b 4d 78                 	movq	0x78(%rbp), %rcx
14172ba0d: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172ba13: e8 08 82 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172ba18: 4c 89 e6                    	movq	%r12, %rsi
14172ba1b: e9 d3 fe ff ff              	jmp	0x14172b8f3 <.text+0x172a8f3>
14172ba20: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
14172ba27: 4c 8b 85 e0 00 00 00        	movq	0xe0(%rbp), %r8
14172ba2e: 48 89 8d 90 00 00 00        	movq	%rcx, 0x90(%rbp)
14172ba35: 4c 89 85 98 00 00 00        	movq	%r8, 0x98(%rbp)
14172ba3c: 4d 85 c0                    	testq	%r8, %r8
14172ba3f: 0f 84 4f 01 00 00           	je	0x14172bb94 <.text+0x172ab94>
14172ba45: 49 83 f8 01                 	cmpq	$0x1, %r8
14172ba49: 75 1d                       	jne	0x14172ba68 <.text+0x172aa68>
14172ba4b: 0f b6 01                    	movzbl	(%rcx), %eax
14172ba4e: 83 f8 2b                    	cmpl	$0x2b, %eax
14172ba51: 0f 84 3d 01 00 00           	je	0x14172bb94 <.text+0x172ab94>
14172ba57: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172ba5d: 83 f8 2d                    	cmpl	$0x2d, %eax
14172ba60: 0f 84 2e 01 00 00           	je	0x14172bb94 <.text+0x172ab94>
14172ba66: eb 60                       	jmp	0x14172bac8 <.text+0x172aac8>
14172ba68: 80 39 2b                    	cmpb	$0x2b, (%rcx)
14172ba6b: 75 55                       	jne	0x14172bac2 <.text+0x172aac2>
14172ba6d: 48 ff c1                    	incq	%rcx
14172ba70: 49 8d 40 ff                 	leaq	-0x1(%r8), %rax
14172ba74: 49 83 f8 12                 	cmpq	$0x12, %r8
14172ba78: 49 89 c0                    	movq	%rax, %r8
14172ba7b: 72 4b                       	jb	0x14172bac8 <.text+0x172aac8>
14172ba7d: 45 31 c9                    	xorl	%r9d, %r9d
14172ba80: 31 db                       	xorl	%ebx, %ebx
14172ba82: 4d 39 c8                    	cmpq	%r9, %r8
14172ba85: 74 67                       	je	0x14172baee <.text+0x172aaee>
14172ba87: 48 89 d8                    	movq	%rbx, %rax
14172ba8a: ba 0a 00 00 00              	movl	$0xa, %edx
14172ba8f: 48 f7 e2                    	mulq	%rdx
14172ba92: 41 0f 90 c2                 	seto	%r10b
14172ba96: 42 0f b6 14 09              	movzbl	(%rcx,%r9), %edx
14172ba9b: 83 c2 d0                    	addl	$-0x30, %edx
14172ba9e: 83 fa 09                    	cmpl	$0x9, %edx
14172baa1: 0f 87 ed 00 00 00           	ja	0x14172bb94 <.text+0x172ab94>
14172baa7: 45 84 d2                    	testb	%r10b, %r10b
14172baaa: 0f 85 e4 00 00 00           	jne	0x14172bb94 <.text+0x172ab94>
14172bab0: 48 89 c3                    	movq	%rax, %rbx
14172bab3: 89 d0                       	movl	%edx, %eax
14172bab5: 49 ff c1                    	incq	%r9
14172bab8: 48 01 c3                    	addq	%rax, %rbx
14172babb: 73 c5                       	jae	0x14172ba82 <.text+0x172aa82>
14172babd: e9 d2 00 00 00              	jmp	0x14172bb94 <.text+0x172ab94>
14172bac2: 49 83 f8 11                 	cmpq	$0x11, %r8
14172bac6: 73 b5                       	jae	0x14172ba7d <.text+0x172aa7d>
14172bac8: 31 c0                       	xorl	%eax, %eax
14172baca: 31 db                       	xorl	%ebx, %ebx
14172bacc: 0f b6 14 01                 	movzbl	(%rcx,%rax), %edx
14172bad0: 83 c2 d0                    	addl	$-0x30, %edx
14172bad3: 83 fa 09                    	cmpl	$0x9, %edx
14172bad6: 0f 87 b8 00 00 00           	ja	0x14172bb94 <.text+0x172ab94>
14172badc: 4c 8d 0c 9b                 	leaq	(%rbx,%rbx,4), %r9
14172bae0: 89 d2                       	movl	%edx, %edx
14172bae2: 4a 8d 1c 4a                 	leaq	(%rdx,%r9,2), %rbx
14172bae6: 48 ff c0                    	incq	%rax
14172bae9: 49 39 c0                    	cmpq	%rax, %r8
14172baec: 75 de                       	jne	0x14172bacc <.text+0x172aacc>
14172baee: 48 89 9d d0 00 00 00        	movq	%rbx, 0xd0(%rbp)
14172baf5: 48 83 7d 58 00              	cmpq	$0x0, 0x58(%rbp)
14172bafa: 0f 84 94 00 00 00           	je	0x14172bb94 <.text+0x172ab94>
14172bb00: 48 8d 4d 60                 	leaq	0x60(%rbp), %rcx
14172bb04: 4c 89 f2                    	movq	%r14, %rdx
14172bb07: e8 64 52 e8 fe              	callq	0x1405b0d70 <.text+0x5afd70>
14172bb0c: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14172bb10: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
14172bb14: 49 89 d0                    	movq	%rdx, %r8
14172bb17: 49 21 c0                    	andq	%rax, %r8
14172bb1a: 48 c1 e8 39                 	shrq	$0x39, %rax
14172bb1e: 66 0f 6e c0                 	movd	%eax, %xmm0
14172bb22: 66 0f 60 c0                 	punpcklbw	%xmm0, %xmm0    # xmm0 = xmm0[0,0,1,1,2,2,3,3,4,4,5,5,6,6,7,7]
14172bb26: f2 0f 70 c0 00              	pshuflw	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0,4,5,6,7]
14172bb2b: 66 0f 70 c0 00              	pshufd	$0x0, %xmm0, %xmm0      # xmm0 = xmm0[0,0,0,0]
14172bb30: 31 c0                       	xorl	%eax, %eax
14172bb32: f3 42 0f 6f 0c 01           	movdqu	(%rcx,%r8), %xmm1
14172bb38: 66 0f 6f d1                 	movdqa	%xmm1, %xmm2
14172bb3c: 66 0f 74 d0                 	pcmpeqb	%xmm0, %xmm2
14172bb40: 66 44 0f d7 ca              	pmovmskb	%xmm2, %r9d
14172bb45: 45 85 c9                    	testl	%r9d, %r9d
14172bb48: 74 2c                       	je	0x14172bb76 <.text+0x172ab76>
14172bb4a: f3 45 0f bc d1              	tzcntl	%r9d, %r10d
14172bb4f: 4d 01 c2                    	addq	%r8, %r10
14172bb52: 49 21 d2                    	andq	%rdx, %r10
14172bb55: 49 c1 e2 03                 	shlq	$0x3, %r10
14172bb59: 49 89 cb                    	movq	%rcx, %r11
14172bb5c: 4d 29 d3                    	subq	%r10, %r11
14172bb5f: 49 3b 5b f8                 	cmpq	-0x8(%r11), %rbx
14172bb63: 0f 84 97 fe ff ff           	je	0x14172ba00 <.text+0x172aa00>
14172bb69: 45 8d 51 ff                 	leal	-0x1(%r9), %r10d
14172bb6d: 66 45 21 ca                 	andw	%r9w, %r10w
14172bb71: 45 89 d1                    	movl	%r10d, %r9d
14172bb74: 75 d4                       	jne	0x14172bb4a <.text+0x172ab4a>
14172bb76: 66 0f 74 ce                 	pcmpeqb	%xmm6, %xmm1
14172bb7a: 66 44 0f d7 c9              	pmovmskb	%xmm1, %r9d
14172bb7f: 45 85 c9                    	testl	%r9d, %r9d
14172bb82: 75 10                       	jne	0x14172bb94 <.text+0x172ab94>
14172bb84: 49 01 c0                    	addq	%rax, %r8
14172bb87: 49 83 c0 10                 	addq	$0x10, %r8
14172bb8b: 48 83 c0 10                 	addq	$0x10, %rax
14172bb8f: 49 21 d0                    	andq	%rdx, %r8
14172bb92: eb 9e                       	jmp	0x14172bb32 <.text+0x172ab32>
14172bb94: 48 8b 05 0d 08 f0 01        	movq	0x1f0080d(%rip), %rax   # 0x14362c3a8
14172bb9b: 48 8b 00                    	movq	(%rax), %rax
14172bb9e: 48 83 f8 04                 	cmpq	$0x4, %rax
14172bba2: 0f 82 e3 00 00 00           	jb	0x14172bc8b <.text+0x172ac8b>
14172bba8: 48 8d 85 90 00 00 00        	leaq	0x90(%rbp), %rax
14172bbaf: 48 89 45 00                 	movq	%rax, (%rbp)
14172bbb3: 48 8d 05 e6 7c e0 fe        	leaq	-0x11f831a(%rip), %rax  # 0x1405338a0 <.text+0x5328a0>
14172bbba: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14172bbbe: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172bbc5: 48 8d 0d f4 f5 59 01        	leaq	0x159f5f4(%rip), %rcx   # 0x142ccb1c0
14172bbcc: e8 af 21 f5 fe              	callq	0x14067dd80 <.text+0x67cd80>
14172bbd1: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14172bbd5: 8b 40 10                    	movl	0x10(%rax), %eax
14172bbd8: 48 c7 85 00 01 00 00 04 00 00 00    	movq	$0x4, 0x100(%rbp)
14172bbe3: 48 8d 0d 76 e9 59 01        	leaq	0x159e976(%rip), %rcx   # 0x142cca560
14172bbea: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14172bbf1: 48 c7 85 10 01 00 00 28 00 00 00    	movq	$0x28, 0x110(%rbp)
14172bbfc: 48 8d 15 ad f5 59 01        	leaq	0x159f5ad(%rip), %rdx   # 0x142ccb1b0
14172bc03: 48 89 95 20 01 00 00        	movq	%rdx, 0x120(%rbp)
14172bc0a: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
14172bc15: 48 89 ea                    	movq	%rbp, %rdx
14172bc18: 48 89 95 30 01 00 00        	movq	%rdx, 0x130(%rbp)
14172bc1f: 48 c7 85 38 01 00 00 01 00 00 00    	movq	$0x1, 0x138(%rbp)
14172bc2a: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
14172bc35: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
14172bc40: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14172bc47: 48 c7 85 e0 00 00 00 28 00 00 00    	movq	$0x28, 0xe0(%rbp)
14172bc52: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
14172bc5d: f3 0f 7f 85 f0 00 00 00     	movdqu	%xmm0, 0xf0(%rbp)
14172bc65: c7 85 18 01 00 00 01 00 00 00       	movl	$0x1, 0x118(%rbp)
14172bc6f: 89 85 1c 01 00 00           	movl	%eax, 0x11c(%rbp)
14172bc75: c6 85 76 01 00 00 01        	movb	$0x1, 0x176(%rbp)
14172bc7c: 48 8d 8d 74 01 00 00        	leaq	0x174(%rbp), %rcx
14172bc83: 4c 89 f2                    	movq	%r14, %rdx
14172bc86: e8 a5 e6 61 ff              	callq	0x140d4a330 <.text+0xd49330>
14172bc8b: f3 0f 6f 45 70              	movdqu	0x70(%rbp), %xmm0
14172bc90: f3 0f 6f 8d 80 00 00 00     	movdqu	0x80(%rbp), %xmm1
14172bc98: 66 0f 7f 8d e0 00 00 00     	movdqa	%xmm1, 0xe0(%rbp)
14172bca0: 66 0f 7f 85 d0 00 00 00     	movdqa	%xmm0, 0xd0(%rbp)
14172bca8: 4c 89 f1                    	movq	%r14, %rcx
14172bcab: e8 20 b2 64 ff              	callq	0x140d76ed0 <.text+0xd75ed0>
14172bcb0: 48 89 c1                    	movq	%rax, %rcx
14172bcb3: e8 d8 d8 63 ff              	callq	0x140d69590 <.text+0xd68590>
14172bcb8: 48 89 c3                    	movq	%rax, %rbx
14172bcbb: 48 8b 95 d0 00 00 00        	movq	0xd0(%rbp), %rdx
14172bcc2: 48 85 d2                    	testq	%rdx, %rdx
14172bcc5: 74 12                       	je	0x14172bcd9 <.text+0x172acd9>
14172bcc7: 48 8b 8d d8 00 00 00        	movq	0xd8(%rbp), %rcx
14172bcce: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172bcd4: e8 47 7f ee fe              	callq	0x140613c20 <.text+0x612c20>
14172bcd9: 48 85 db                    	testq	%rbx, %rbx
14172bcdc: 0f 84 36 fd ff ff           	je	0x14172ba18 <.text+0x172aa18>
14172bce2: 48 89 9d 58 01 00 00        	movq	%rbx, 0x158(%rbp)
14172bce9: 48 8b 05 b8 06 f0 01        	movq	0x1f006b8(%rip), %rax   # 0x14362c3a8
14172bcf0: 48 8b 00                    	movq	(%rax), %rax
14172bcf3: 48 83 f8 02                 	cmpq	$0x2, %rax
14172bcf7: 0f 82 dc 00 00 00           	jb	0x14172bdd9 <.text+0x172add9>
14172bcfd: 48 8d 85 58 01 00 00        	leaq	0x158(%rbp), %rax
14172bd04: 48 89 45 00                 	movq	%rax, (%rbp)
14172bd08: 48 8d 05 d1 ef 62 ff        	leaq	-0x9d102f(%rip), %rax   # 0x140d5ace0 <.text+0xd59ce0>
14172bd0f: 48 89 45 08                 	movq	%rax, 0x8(%rbp)
14172bd13: 48 8d 0d f6 f4 59 01        	leaq	0x159f4f6(%rip), %rcx   # 0x142ccb210
14172bd1a: e8 61 20 f5 fe              	callq	0x14067dd80 <.text+0x67cd80>
14172bd1f: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14172bd23: 8b 40 10                    	movl	0x10(%rax), %eax
14172bd26: 48 c7 85 00 01 00 00 02 00 00 00    	movq	$0x2, 0x100(%rbp)
14172bd31: 48 8d 0d 28 e8 59 01        	leaq	0x159e828(%rip), %rcx   # 0x142cca560
14172bd38: 48 89 8d 08 01 00 00        	movq	%rcx, 0x108(%rbp)
14172bd3f: 48 c7 85 10 01 00 00 28 00 00 00    	movq	$0x28, 0x110(%rbp)
14172bd4a: 48 8d 15 af f4 59 01        	leaq	0x159f4af(%rip), %rdx   # 0x142ccb200
14172bd51: 48 89 95 20 01 00 00        	movq	%rdx, 0x120(%rbp)
14172bd58: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
14172bd63: 48 89 ea                    	movq	%rbp, %rdx
14172bd66: 48 89 95 30 01 00 00        	movq	%rdx, 0x130(%rbp)
14172bd6d: 48 c7 85 38 01 00 00 01 00 00 00    	movq	$0x1, 0x138(%rbp)
14172bd78: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
14172bd83: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
14172bd8e: 48 89 8d d8 00 00 00        	movq	%rcx, 0xd8(%rbp)
14172bd95: 48 c7 85 e0 00 00 00 28 00 00 00    	movq	$0x28, 0xe0(%rbp)
14172bda0: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
14172bdab: f3 0f 7f 85 f0 00 00 00     	movdqu	%xmm0, 0xf0(%rbp)
14172bdb3: c7 85 18 01 00 00 01 00 00 00       	movl	$0x1, 0x118(%rbp)
14172bdbd: 89 85 1c 01 00 00           	movl	%eax, 0x11c(%rbp)
14172bdc3: 48 8d 8d 74 01 00 00        	leaq	0x174(%rbp), %rcx
14172bdca: 4c 89 f2                    	movq	%r14, %rdx
14172bdcd: e8 5e e5 61 ff              	callq	0x140d4a330 <.text+0xd49330>
14172bdd2: 48 8b 9d 58 01 00 00        	movq	0x158(%rbp), %rbx
14172bdd9: 89 d8                       	movl	%ebx, %eax
14172bddb: 83 e0 03                    	andl	$0x3, %eax
14172bdde: 83 f8 01                    	cmpl	$0x1, %eax
14172bde1: 0f 85 31 fc ff ff           	jne	0x14172ba18 <.text+0x172aa18>
14172bde7: 48 8d 43 ff                 	leaq	-0x1(%rbx), %rax
14172bdeb: 48 89 85 c8 00 00 00        	movq	%rax, 0xc8(%rbp)
14172bdf2: 48 8b 43 ff                 	movq	-0x1(%rbx), %rax
14172bdf6: 48 89 85 68 01 00 00        	movq	%rax, 0x168(%rbp)
14172bdfd: 48 8b 43 07                 	movq	0x7(%rbx), %rax
14172be01: 48 89 85 60 01 00 00        	movq	%rax, 0x160(%rbp)
14172be08: 48 8b 00                    	movq	(%rax), %rax
14172be0b: 48 85 c0                    	testq	%rax, %rax
14172be0e: 74 09                       	je	0x14172be19 <.text+0x172ae19>
14172be10: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14172be17: ff d0                       	callq	*%rax
14172be19: 48 8b 8d 68 01 00 00        	movq	0x168(%rbp), %rcx
14172be20: 48 8b 85 60 01 00 00        	movq	0x160(%rbp), %rax
14172be27: 48 8b 50 08                 	movq	0x8(%rax), %rdx
14172be2b: 48 85 d2                    	testq	%rdx, %rdx
14172be2e: 74 09                       	je	0x14172be39 <.text+0x172ae39>
14172be30: 4c 8b 40 10                 	movq	0x10(%rax), %r8
14172be34: e8 e7 7d ee fe              	callq	0x140613c20 <.text+0x612c20>
14172be39: ba 18 00 00 00              	movl	$0x18, %edx
14172be3e: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14172be44: 48 8b 8d c8 00 00 00        	movq	0xc8(%rbp), %rcx
14172be4b: e8 d0 7d ee fe              	callq	0x140613c20 <.text+0x612c20>
14172be50: 48 8b b5 b0 00 00 00        	movq	0xb0(%rbp), %rsi
14172be57: 4c 8b bd c0 00 00 00        	movq	0xc0(%rbp), %r15
14172be5e: e9 90 fa ff ff              	jmp	0x14172b8f3 <.text+0x172a8f3>
14172be63: 4c 89 e6                    	movq	%r12, %rsi
14172be66: 49 29 f7                    	subq	%rsi, %r15
14172be69: 74 37                       	je	0x14172bea2 <.text+0x172aea2>
14172be6b: 49 c1 ef 05                 	shrq	$0x5, %r15
14172be6f: 48 83 c6 08                 	addq	$0x8, %rsi
14172be73: eb 14                       	jmp	0x14172be89 <.text+0x172ae89>
14172be75: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14172be80: 48 83 c6 20                 	addq	$0x20, %rsi
14172be84: 49 ff cf                    	decq	%r15
14172be87: 74 19                       	je	0x14172bea2 <.text+0x172aea2>
14172be89: 48 8b 56 f8                 	movq	-0x8(%rsi), %rdx
14172be8d: 48 85 d2                    	testq	%rdx, %rdx
14172be90: 74 ee                       	je	0x14172be80 <.text+0x172ae80>
14172be92: 48 8b 0e                    	movq	(%rsi), %rcx
14172be95: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172be9b: e8 80 7d ee fe              	callq	0x140613c20 <.text+0x612c20>
14172bea0: eb de                       	jmp	0x14172be80 <.text+0x172ae80>
14172bea2: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14172bea9: 48 85 d2                    	testq	%rdx, %rdx
14172beac: 74 16                       	je	0x14172bec4 <.text+0x172aec4>
14172beae: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14172beb5: 48 c1 e2 05                 	shlq	$0x5, %rdx
14172beb9: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14172bebf: e8 5c 7d ee fe              	callq	0x140613c20 <.text+0x612c20>
14172bec4: 48 8b 55 48                 	movq	0x48(%rbp), %rdx
14172bec8: 48 85 d2                    	testq	%rdx, %rdx
14172becb: 74 27                       	je	0x14172bef4 <.text+0x172aef4>
14172becd: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
14172bed5: 48 83 e0 f0                 	andq	$-0x10, %rax
14172bed9: 48 01 c2                    	addq	%rax, %rdx
14172bedc: 48 83 c2 11                 	addq	$0x11, %rdx
14172bee0: 74 12                       	je	0x14172bef4 <.text+0x172aef4>
14172bee2: 48 8b 4d 40                 	movq	0x40(%rbp), %rcx
14172bee6: 48 29 c1                    	subq	%rax, %rcx
14172bee9: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14172beef: e8 2c 7d ee fe              	callq	0x140613c20 <.text+0x612c20>
14172bef4: 48 8b 05 ad 04 f0 01        	movq	0x1f004ad(%rip), %rax   # 0x14362c3a8
14172befb: 48 8b 00                    	movq	(%rax), %rax
14172befe: 48 83 f8 03                 	cmpq	$0x3, %rax
14172bf02: 0f 82 f8 00 00 00           	jb	0x14172c000 <.text+0x172b000>
14172bf08: c6 85 75 01 00 00 00        	movb	$0x0, 0x175(%rbp)
14172bf0f: 48 8d 4d a0                 	leaq	-0x60(%rbp), %rcx
14172bf13: e8 f8 79 63 ff              	callq	0x140d63910 <.text+0xd62910>
14172bf18: 48 89 45 40                 	movq	%rax, 0x40(%rbp)
14172bf1c: 89 55 48                    	movl	%edx, 0x48(%rbp)
14172bf1f: 48 8d 45 40                 	leaq	0x40(%rbp), %rax
14172bf23: 48 89 45 70                 	movq	%rax, 0x70(%rbp)
14172bf27: 48 8d 05 22 e6 65 ff        	leaq	-0x9a19de(%rip), %rax   # 0x140d8a550 <.text+0xd89550>
14172bf2e: 48 89 45 78                 	movq	%rax, 0x78(%rbp)
14172bf32: c6 85 75 01 00 00 00        	movb	$0x0, 0x175(%rbp)
14172bf39: 48 8d 0d 00 f1 59 01        	leaq	0x159f100(%rip), %rcx   # 0x142ccb040
14172bf40: e8 3b 1e f5 fe              	callq	0x14067dd80 <.text+0x67cd80>
14172bf45: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14172bf49: 8b 40 10                    	movl	0x10(%rax), %eax
14172bf4c: 48 c7 85 00 01 00 00 03 00 00 00    	movq	$0x3, 0x100(%rbp)
14172bf57: 48 8d 15 02 e6 59 01        	leaq	0x159e602(%rip), %rdx   # 0x142cca560
14172bf5e: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
14172bf65: 48 c7 85 10 01 00 00 28 00 00 00    	movq	$0x28, 0x110(%rbp)
14172bf70: 48 8d 0d b9 f0 59 01        	leaq	0x159f0b9(%rip), %rcx   # 0x142ccb030
14172bf77: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
14172bf7e: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
14172bf89: 48 89 bd 30 01 00 00        	movq	%rdi, 0x130(%rbp)
14172bf90: 48 c7 85 38 01 00 00 01 00 00 00    	movq	$0x1, 0x138(%rbp)
14172bf9b: 48 c7 85 40 01 00 00 00 00 00 00    	movq	$0x0, 0x140(%rbp)
14172bfa6: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
14172bfb1: 48 89 95 d8 00 00 00        	movq	%rdx, 0xd8(%rbp)
14172bfb8: 48 c7 85 e0 00 00 00 28 00 00 00    	movq	$0x28, 0xe0(%rbp)
14172bfc3: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
14172bfce: f3 0f 7f 85 f0 00 00 00     	movdqu	%xmm0, 0xf0(%rbp)
14172bfd6: c7 85 18 01 00 00 01 00 00 00       	movl	$0x1, 0x118(%rbp)
14172bfe0: 89 85 1c 01 00 00           	movl	%eax, 0x11c(%rbp)
14172bfe6: c6 85 75 01 00 00 00        	movb	$0x0, 0x175(%rbp)
14172bfed: 48 8d 8d 74 01 00 00        	leaq	0x174(%rbp), %rcx
14172bff4: 48 8d 95 d0 00 00 00        	leaq	0xd0(%rbp), %rdx
14172bffb: e8 30 e3 61 ff              	callq	0x140d4a330 <.text+0xd49330>
14172c000: c6 85 71 01 00 00 00        	movb	$0x0, 0x171(%rbp)
14172c007: c6 85 70 01 00 00 00        	movb	$0x0, 0x170(%rbp)
14172c00e: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
14172c012: e8 a9 b8 14 ff              	callq	0x1408778c0 <.text+0x8768c0>
14172c017: e9 07 02 00 00              	jmp	0x14172c223 <.text+0x172b223>
14172c01c: 73 0f                       	jae	0x14172c02d <.text+0x172b02d>
14172c01e: 48 8b 4d 48                 	movq	0x48(%rbp), %rcx
14172c022: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172c028: e8 f3 7b ee fe              	callq	0x140613c20 <.text+0x612c20>
14172c02d: 48 8b 05 74 03 f0 01        	movq	0x1f00374(%rip), %rax   # 0x14362c3a8
14172c034: 48 8b 00                    	movq	(%rax), %rax
14172c037: 48 83 f8 03                 	cmpq	$0x3, %rax
14172c03b: 0f 82 c8 00 00 00           	jb	0x14172c109 <.text+0x172b109>
14172c041: c6 85 77 01 00 00 00        	movb	$0x0, 0x177(%rbp)
14172c048: 48 8d 0d 29 f1 59 01        	leaq	0x159f129(%rip), %rcx   # 0x142ccb178
14172c04f: e8 2c 1d f5 fe              	callq	0x14067dd80 <.text+0x67cd80>
14172c054: f3 0f 6f 00                 	movdqu	(%rax), %xmm0
14172c058: 8b 40 10                    	movl	0x10(%rax), %eax
14172c05b: 48 c7 85 00 01 00 00 03 00 00 00    	movq	$0x3, 0x100(%rbp)
14172c066: 48 8d 15 f3 e4 59 01        	leaq	0x159e4f3(%rip), %rdx   # 0x142cca560
14172c06d: 48 89 95 08 01 00 00        	movq	%rdx, 0x108(%rbp)
14172c074: 48 c7 85 10 01 00 00 28 00 00 00    	movq	$0x28, 0x110(%rbp)
14172c07f: 48 8d 0d e2 f0 59 01        	leaq	0x159f0e2(%rip), %rcx   # 0x142ccb168
14172c086: 48 89 8d 20 01 00 00        	movq	%rcx, 0x120(%rbp)
14172c08d: 48 c7 85 28 01 00 00 01 00 00 00    	movq	$0x1, 0x128(%rbp)
14172c098: 48 c7 85 30 01 00 00 08 00 00 00    	movq	$0x8, 0x130(%rbp)
14172c0a3: 48 c7 85 d0 00 00 00 00 00 00 00    	movq	$0x0, 0xd0(%rbp)
14172c0ae: 66 0f ef c9                 	pxor	%xmm1, %xmm1
14172c0b2: f3 0f 7f 8d 38 01 00 00     	movdqu	%xmm1, 0x138(%rbp)
14172c0ba: 48 89 95 d8 00 00 00        	movq	%rdx, 0xd8(%rbp)
14172c0c1: 48 c7 85 e0 00 00 00 28 00 00 00    	movq	$0x28, 0xe0(%rbp)
14172c0cc: 48 c7 85 e8 00 00 00 00 00 00 00    	movq	$0x0, 0xe8(%rbp)
14172c0d7: f3 0f 7f 85 f0 00 00 00     	movdqu	%xmm0, 0xf0(%rbp)
14172c0df: c7 85 18 01 00 00 01 00 00 00       	movl	$0x1, 0x118(%rbp)
14172c0e9: 89 85 1c 01 00 00           	movl	%eax, 0x11c(%rbp)
14172c0ef: c6 85 77 01 00 00 00        	movb	$0x0, 0x177(%rbp)
14172c0f6: 48 8d 8d 74 01 00 00        	leaq	0x174(%rbp), %rcx
14172c0fd: 48 8d 95 d0 00 00 00        	leaq	0xd0(%rbp), %rdx
14172c104: e8 27 e2 61 ff              	callq	0x140d4a330 <.text+0xd49330>
14172c109: 48 8b b5 b0 00 00 00        	movq	0xb0(%rbp), %rsi
14172c110: 48 8b bd c0 00 00 00        	movq	0xc0(%rbp), %rdi
14172c117: 48 29 f7                    	subq	%rsi, %rdi
14172c11a: 74 36                       	je	0x14172c152 <.text+0x172b152>
14172c11c: 48 c1 ef 05                 	shrq	$0x5, %rdi
14172c120: 48 83 c6 08                 	addq	$0x8, %rsi
14172c124: eb 13                       	jmp	0x14172c139 <.text+0x172b139>
14172c126: 66 2e 0f 1f 84 00 00 00 00 00       	nopw	%cs:(%rax,%rax)
14172c130: 48 83 c6 20                 	addq	$0x20, %rsi
14172c134: 48 ff cf                    	decq	%rdi
14172c137: 74 19                       	je	0x14172c152 <.text+0x172b152>
14172c139: 48 8b 56 f8                 	movq	-0x8(%rsi), %rdx
14172c13d: 48 85 d2                    	testq	%rdx, %rdx
14172c140: 74 ee                       	je	0x14172c130 <.text+0x172b130>
14172c142: 48 8b 0e                    	movq	(%rsi), %rcx
14172c145: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172c14b: e8 d0 7a ee fe              	callq	0x140613c20 <.text+0x612c20>
14172c150: eb de                       	jmp	0x14172c130 <.text+0x172b130>
14172c152: 48 8b 95 b8 00 00 00        	movq	0xb8(%rbp), %rdx
14172c159: 48 85 d2                    	testq	%rdx, %rdx
14172c15c: 74 16                       	je	0x14172c174 <.text+0x172b174>
14172c15e: 48 8b 8d a8 00 00 00        	movq	0xa8(%rbp), %rcx
14172c165: 48 c1 e2 05                 	shlq	$0x5, %rdx
14172c169: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14172c16f: e8 ac 7a ee fe              	callq	0x140613c20 <.text+0x612c20>
14172c174: 48 8b 55 b8                 	movq	-0x48(%rbp), %rdx
14172c178: 48 85 d2                    	testq	%rdx, %rdx
14172c17b: 74 27                       	je	0x14172c1a4 <.text+0x172b1a4>
14172c17d: 48 8d 04 d5 17 00 00 00     	leaq	0x17(,%rdx,8), %rax
14172c185: 48 83 e0 f0                 	andq	$-0x10, %rax
14172c189: 48 01 c2                    	addq	%rax, %rdx
14172c18c: 48 83 c2 11                 	addq	$0x11, %rdx
14172c190: 74 12                       	je	0x14172c1a4 <.text+0x172b1a4>
14172c192: 48 8b 4d b0                 	movq	-0x50(%rbp), %rcx
14172c196: 48 29 c1                    	subq	%rax, %rcx
14172c199: 41 b8 10 00 00 00           	movl	$0x10, %r8d
14172c19f: e8 7c 7a ee fe              	callq	0x140613c20 <.text+0x612c20>
14172c1a4: c6 85 71 01 00 00 01        	movb	$0x1, 0x171(%rbp)
14172c1ab: c6 85 70 01 00 00 00        	movb	$0x0, 0x170(%rbp)
14172c1b2: 48 8d 4d f0                 	leaq	-0x10(%rbp), %rcx
14172c1b6: e8 05 b7 14 ff              	callq	0x1408778c0 <.text+0x8768c0>
14172c1bb: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14172c1c2: 48 8b 70 08                 	movq	0x8(%rax), %rsi
14172c1c6: 48 8b 78 10                 	movq	0x10(%rax), %rdi
14172c1ca: 48 85 ff                    	testq	%rdi, %rdi
14172c1cd: 74 33                       	je	0x14172c202 <.text+0x172b202>
14172c1cf: 48 8d 5e 08                 	leaq	0x8(%rsi), %rbx
14172c1d3: eb 14                       	jmp	0x14172c1e9 <.text+0x172b1e9>
14172c1d5: 66 66 2e 0f 1f 84 00 00 00 00 00    	nopw	%cs:(%rax,%rax)
14172c1e0: 48 83 c3 20                 	addq	$0x20, %rbx
14172c1e4: 48 ff cf                    	decq	%rdi
14172c1e7: 74 19                       	je	0x14172c202 <.text+0x172b202>
14172c1e9: 48 8b 53 f8                 	movq	-0x8(%rbx), %rdx
14172c1ed: 48 85 d2                    	testq	%rdx, %rdx
14172c1f0: 74 ee                       	je	0x14172c1e0 <.text+0x172b1e0>
14172c1f2: 48 8b 0b                    	movq	(%rbx), %rcx
14172c1f5: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14172c1fb: e8 20 7a ee fe              	callq	0x140613c20 <.text+0x612c20>
14172c200: eb de                       	jmp	0x14172c1e0 <.text+0x172b1e0>
14172c202: 48 8b 85 a0 00 00 00        	movq	0xa0(%rbp), %rax
14172c209: 48 8b 10                    	movq	(%rax), %rdx
14172c20c: 48 85 d2                    	testq	%rdx, %rdx
14172c20f: 74 12                       	je	0x14172c223 <.text+0x172b223>
14172c211: 48 c1 e2 05                 	shlq	$0x5, %rdx
14172c215: 41 b8 08 00 00 00           	movl	$0x8, %r8d
14172c21b: 48 89 f1                    	movq	%rsi, %rcx
14172c21e: e8 fd 79 ee fe              	callq	0x140613c20 <.text+0x612c20>
14172c223: 0f 28 b5 80 01 00 00        	movaps	0x180(%rbp), %xmm6
14172c22a: 48 81 c4 18 02 00 00        	addq	$0x218, %rsp            # imm = 0x218
14172c231: 5b                          	popq	%rbx
14172c232: 5f                          	popq	%rdi
14172c233: 5e                          	popq	%rsi
14172c234: 41 5c                       	popq	%r12
14172c236: 41 5d                       	popq	%r13
14172c238: 41 5e                       	popq	%r14
14172c23a: 41 5f                       	popq	%r15
14172c23c: 5d                          	popq	%rbp
14172c23d: c3                          	retq
14172c23e: 4c 89 b5 68 01 00 00        	movq	%r14, 0x168(%rbp)
14172c245: 4c 8d 05 7c ad 55 01        	leaq	0x155ad7c(%rip), %r8    # 0x142c86fc8
14172c24c: b9 01 00 00 00              	movl	$0x1, %ecx
14172c251: ba 00 20 00 00              	movl	$0x2000, %edx           # imm = 0x2000
14172c256: e8 88 cd 1a 01              	callq	0x1428d8fe3 <.text+0x28d7fe3>
14172c25b: 0f 0b                       	ud2
