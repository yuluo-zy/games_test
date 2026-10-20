
D:\game\ljxsj_92385\Tiny Glade\tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
14157b150: 41 57                       	pushq	%r15
14157b152: 41 56                       	pushq	%r14
14157b154: 41 55                       	pushq	%r13
14157b156: 41 54                       	pushq	%r12
14157b158: 56                          	pushq	%rsi
14157b159: 57                          	pushq	%rdi
14157b15a: 55                          	pushq	%rbp
14157b15b: 53                          	pushq	%rbx
14157b15c: 48 81 ec f8 02 00 00        	subq	$0x2f8, %rsp            # imm = 0x2F8
14157b163: 49 89 cf                    	movq	%rcx, %r15
14157b166: 0f 10 02                    	movups	(%rdx), %xmm0
14157b169: 0f 29 84 24 90 00 00 00     	movaps	%xmm0, 0x90(%rsp)
14157b171: 48 8b 42 10                 	movq	0x10(%rdx), %rax
14157b175: 48 89 84 24 a0 00 00 00     	movq	%rax, 0xa0(%rsp)
14157b17d: 4c 8b a4 24 90 00 00 00     	movq	0x90(%rsp), %r12
14157b185: 48 8b 84 24 98 00 00 00     	movq	0x98(%rsp), %rax
14157b18d: 48 89 44 24 20              	movq	%rax, 0x20(%rsp)
14157b192: 49 39 c4                    	cmpq	%rax, %r12
14157b195: 0f 84 14 02 00 00           	je	0x14157b3af <.text+0x157a3af>
14157b19b: 4c 89 bc 24 00 01 00 00     	movq	%r15, 0x100(%rsp)
14157b1a3: 49 8d 6c 24 20              	leaq	0x20(%r12), %rbp
14157b1a8: 48 89 ac 24 90 00 00 00     	movq	%rbp, 0x90(%rsp)
14157b1b0: 48 8b bc 24 a0 00 00 00     	movq	0xa0(%rsp), %rdi
14157b1b8: 45 8b 34 24                 	movl	(%r12), %r14d
14157b1bc: 45 31 ed                    	xorl	%r13d, %r13d
14157b1bf: b8 08 00 00 00              	movl	$0x8, %eax
14157b1c4: 41 bf 00 00 00 00           	movl	$0x0, %r15d
14157b1ca: 41 83 fe 01                 	cmpl	$0x1, %r14d
14157b1ce: 75 19                       	jne	0x14157b1e9 <.text+0x157a1e9>
14157b1d0: 41 0f 10 44 24 04           	movups	0x4(%r12), %xmm0
14157b1d6: 0f 29 84 24 f0 00 00 00     	movaps	%xmm0, 0xf0(%rsp)
14157b1de: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14157b1e4: b8 18 00 00 00              	movl	$0x18, %eax
14157b1e9: 45 85 f6                    	testl	%r14d, %r14d
14157b1ec: 41 0f 94 c0                 	sete	%r8b
14157b1f0: 41 c0 e0 03                 	shlb	$0x3, %r8b
14157b1f4: 41 80 c0 2e                 	addb	$0x2e, %r8b
14157b1f8: 49 8b 34 04                 	movq	(%r12,%rax), %rsi
14157b1fc: 8b 1e                       	movl	(%rsi), %ebx
14157b1fe: 48 89 f9                    	movq	%rdi, %rcx
14157b201: 89 da                       	movl	%ebx, %edx
14157b203: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14157b209: e8 82 18 46 ff              	callq	0x1409dca90 <.text+0x9dba90>
14157b20e: c7 84 24 b4 00 00 00 00 00 00 00    	movl	$0x0, 0xb4(%rsp)
14157b219: c7 84 24 bc 00 00 00 00 00 00 00    	movl	$0x0, 0xbc(%rsp)
14157b224: c7 84 24 c4 00 00 00 00 00 00 00    	movl	$0x0, 0xc4(%rsp)
14157b22f: c7 84 24 cc 00 00 00 00 00 00 00    	movl	$0x0, 0xcc(%rsp)
14157b23a: c7 84 24 d4 00 00 00 00 00 00 00    	movl	$0x0, 0xd4(%rsp)
14157b245: c7 84 24 dc 00 00 00 00 00 00 00    	movl	$0x0, 0xdc(%rsp)
14157b250: 48 c7 84 24 e4 00 00 00 00 00 00 00 	movq	$0x0, 0xe4(%rsp)
14157b25c: c6 84 24 ec 00 00 00 00     	movb	$0x0, 0xec(%rsp)
14157b264: 48 8d 4c 24 2c              	leaq	0x2c(%rsp), %rcx
14157b269: 48 8d 94 24 b4 00 00 00     	leaq	0xb4(%rsp), %rdx
14157b271: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14157b277: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14157b27d: e8 9e a6 65 ff              	callq	0x140bd5920 <.text+0xbd4920>
14157b282: 8b 46 0c                    	movl	0xc(%rsi), %eax
14157b285: 0f b6 4e 21                 	movzbl	0x21(%rsi), %ecx
14157b289: 89 5c 24 7c                 	movl	%ebx, 0x7c(%rsp)
14157b28d: 89 84 24 80 00 00 00        	movl	%eax, 0x80(%rsp)
14157b294: 88 8c 24 88 00 00 00        	movb	%cl, 0x88(%rsp)
14157b29b: 44 89 7c 24 68              	movl	%r15d, 0x68(%rsp)
14157b2a0: 0f 28 84 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm0
14157b2a8: 0f 11 44 24 6c              	movups	%xmm0, 0x6c(%rsp)
14157b2ad: 44 89 b4 24 84 00 00 00     	movl	%r14d, 0x84(%rsp)
14157b2b5: 83 7c 24 2c 02              	cmpl	$0x2, 0x2c(%rsp)
14157b2ba: 4c 8b bc 24 00 01 00 00     	movq	0x100(%rsp), %r15
14157b2c2: 0f 84 4d 05 00 00           	je	0x14157b815 <.text+0x157a815>
14157b2c8: 8b 44 24 7c                 	movl	0x7c(%rsp), %eax
14157b2cc: 89 84 24 60 01 00 00        	movl	%eax, 0x160(%rsp)
14157b2d3: 8b 84 24 80 00 00 00        	movl	0x80(%rsp), %eax
14157b2da: 89 84 24 64 01 00 00        	movl	%eax, 0x164(%rsp)
14157b2e1: 8b 84 24 84 00 00 00        	movl	0x84(%rsp), %eax
14157b2e8: 89 84 24 68 01 00 00        	movl	%eax, 0x168(%rsp)
14157b2ef: 0f b6 84 24 88 00 00 00     	movzbl	0x88(%rsp), %eax
14157b2f7: 88 84 24 6c 01 00 00        	movb	%al, 0x16c(%rsp)
14157b2fe: 0f b7 84 24 89 00 00 00     	movzwl	0x89(%rsp), %eax
14157b306: 66 89 84 24 6d 01 00 00     	movw	%ax, 0x16d(%rsp)
14157b30e: 0f b6 84 24 8b 00 00 00     	movzbl	0x8b(%rsp), %eax
14157b316: 88 84 24 6f 01 00 00        	movb	%al, 0x16f(%rsp)
14157b31d: 0f 10 44 24 6c              	movups	0x6c(%rsp), %xmm0
14157b322: 0f 29 84 24 50 01 00 00     	movaps	%xmm0, 0x150(%rsp)
14157b32a: 0f 10 44 24 2c              	movups	0x2c(%rsp), %xmm0
14157b32f: 0f 10 4c 24 3c              	movups	0x3c(%rsp), %xmm1
14157b334: 0f 10 54 24 4c              	movups	0x4c(%rsp), %xmm2
14157b339: 0f 10 5c 24 5c              	movups	0x5c(%rsp), %xmm3
14157b33e: 0f 29 9c 24 40 01 00 00     	movaps	%xmm3, 0x140(%rsp)
14157b346: 0f 29 94 24 30 01 00 00     	movaps	%xmm2, 0x130(%rsp)
14157b34e: 0f 29 8c 24 20 01 00 00     	movaps	%xmm1, 0x120(%rsp)
14157b356: 0f 29 84 24 10 01 00 00     	movaps	%xmm0, 0x110(%rsp)
14157b35e: 41 bd 01 00 00 00           	movl	$0x1, %r13d
14157b364: 48 3b 6c 24 20              	cmpq	0x20(%rsp), %rbp
14157b369: 0f 84 a6 04 00 00           	je	0x14157b815 <.text+0x157a815>
14157b36f: 49 8d 44 24 40              	leaq	0x40(%r12), %rax
14157b374: 48 89 84 24 90 00 00 00     	movq	%rax, 0x90(%rsp)
14157b37c: 45 8b 74 24 20              	movl	0x20(%r12), %r14d
14157b381: 41 83 fe 01                 	cmpl	$0x1, %r14d
14157b385: 48 89 84 24 08 01 00 00     	movq	%rax, 0x108(%rsp)
14157b38d: 75 28                       	jne	0x14157b3b7 <.text+0x157a3b7>
14157b38f: 41 0f 10 44 24 24           	movups	0x24(%r12), %xmm0
14157b395: 0f 29 84 24 f0 00 00 00     	movaps	%xmm0, 0xf0(%rsp)
14157b39d: c7 84 24 8c 00 00 00 01 00 00 00    	movl	$0x1, 0x8c(%rsp)
14157b3a8: b8 18 00 00 00              	movl	$0x18, %eax
14157b3ad: eb 18                       	jmp	0x14157b3c7 <.text+0x157a3c7>
14157b3af: 45 31 ed                    	xorl	%r13d, %r13d
14157b3b2: e9 5e 04 00 00              	jmp	0x14157b815 <.text+0x157a815>
14157b3b7: c7 84 24 8c 00 00 00 00 00 00 00    	movl	$0x0, 0x8c(%rsp)
14157b3c2: b8 08 00 00 00              	movl	$0x8, %eax
14157b3c7: 45 85 f6                    	testl	%r14d, %r14d
14157b3ca: 41 0f 94 c0                 	sete	%r8b
14157b3ce: 41 c0 e0 03                 	shlb	$0x3, %r8b
14157b3d2: 41 80 c0 2e                 	addb	$0x2e, %r8b
14157b3d6: 48 8b 74 05 00              	movq	(%rbp,%rax), %rsi
14157b3db: 8b 1e                       	movl	(%rsi), %ebx
14157b3dd: 41 bd 01 00 00 00           	movl	$0x1, %r13d
14157b3e3: 48 89 f9                    	movq	%rdi, %rcx
14157b3e6: 89 da                       	movl	%ebx, %edx
14157b3e8: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14157b3ee: e8 9d 16 46 ff              	callq	0x1409dca90 <.text+0x9dba90>
14157b3f3: c7 84 24 b4 00 00 00 00 00 00 00    	movl	$0x0, 0xb4(%rsp)
14157b3fe: c7 84 24 bc 00 00 00 00 00 00 00    	movl	$0x0, 0xbc(%rsp)
14157b409: c7 84 24 c4 00 00 00 00 00 00 00    	movl	$0x0, 0xc4(%rsp)
14157b414: c7 84 24 cc 00 00 00 00 00 00 00    	movl	$0x0, 0xcc(%rsp)
14157b41f: c7 84 24 d4 00 00 00 00 00 00 00    	movl	$0x0, 0xd4(%rsp)
14157b42a: c7 84 24 dc 00 00 00 00 00 00 00    	movl	$0x0, 0xdc(%rsp)
14157b435: 48 c7 84 24 e4 00 00 00 00 00 00 00 	movq	$0x0, 0xe4(%rsp)
14157b441: c6 84 24 ec 00 00 00 00     	movb	$0x0, 0xec(%rsp)
14157b449: 48 8d 4c 24 2c              	leaq	0x2c(%rsp), %rcx
14157b44e: 48 8d 94 24 b4 00 00 00     	leaq	0xb4(%rsp), %rdx
14157b456: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14157b45c: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14157b462: e8 b9 a4 65 ff              	callq	0x140bd5920 <.text+0xbd4920>
14157b467: 8b 46 0c                    	movl	0xc(%rsi), %eax
14157b46a: 0f b6 4e 21                 	movzbl	0x21(%rsi), %ecx
14157b46e: 89 5c 24 7c                 	movl	%ebx, 0x7c(%rsp)
14157b472: 89 84 24 80 00 00 00        	movl	%eax, 0x80(%rsp)
14157b479: 88 8c 24 88 00 00 00        	movb	%cl, 0x88(%rsp)
14157b480: 8b 84 24 8c 00 00 00        	movl	0x8c(%rsp), %eax
14157b487: 89 44 24 68                 	movl	%eax, 0x68(%rsp)
14157b48b: 0f 28 84 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm0
14157b493: 48 8d 44 24 6c              	leaq	0x6c(%rsp), %rax
14157b498: 0f 11 00                    	movups	%xmm0, (%rax)
14157b49b: 44 89 b4 24 84 00 00 00     	movl	%r14d, 0x84(%rsp)
14157b4a3: 83 7c 24 2c 02              	cmpl	$0x2, 0x2c(%rsp)
14157b4a8: 0f 84 67 03 00 00           	je	0x14157b815 <.text+0x157a815>
14157b4ae: 48 8d 84 24 70 01 00 00     	leaq	0x170(%rsp), %rax
14157b4b6: 8b 4c 24 7c                 	movl	0x7c(%rsp), %ecx
14157b4ba: 89 48 50                    	movl	%ecx, 0x50(%rax)
14157b4bd: 8b 8c 24 80 00 00 00        	movl	0x80(%rsp), %ecx
14157b4c4: 89 48 54                    	movl	%ecx, 0x54(%rax)
14157b4c7: 8b 8c 24 84 00 00 00        	movl	0x84(%rsp), %ecx
14157b4ce: 89 48 58                    	movl	%ecx, 0x58(%rax)
14157b4d1: 0f b6 8c 24 88 00 00 00     	movzbl	0x88(%rsp), %ecx
14157b4d9: 88 48 5c                    	movb	%cl, 0x5c(%rax)
14157b4dc: 0f b7 8c 24 89 00 00 00     	movzwl	0x89(%rsp), %ecx
14157b4e4: 66 89 48 5d                 	movw	%cx, 0x5d(%rax)
14157b4e8: 0f b6 8c 24 8b 00 00 00     	movzbl	0x8b(%rsp), %ecx
14157b4f0: 88 48 5f                    	movb	%cl, 0x5f(%rax)
14157b4f3: 0f 10 44 24 6c              	movups	0x6c(%rsp), %xmm0
14157b4f8: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14157b4fc: 0f 10 44 24 2c              	movups	0x2c(%rsp), %xmm0
14157b501: 0f 10 4c 24 3c              	movups	0x3c(%rsp), %xmm1
14157b506: 0f 10 54 24 4c              	movups	0x4c(%rsp), %xmm2
14157b50b: 0f 10 5c 24 5c              	movups	0x5c(%rsp), %xmm3
14157b510: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14157b514: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14157b518: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14157b51c: 0f 11 00                    	movups	%xmm0, (%rax)
14157b51f: 41 bd 02 00 00 00           	movl	$0x2, %r13d
14157b525: 48 8b 8c 24 08 01 00 00     	movq	0x108(%rsp), %rcx
14157b52d: 48 3b 4c 24 20              	cmpq	0x20(%rsp), %rcx
14157b532: 0f 84 dd 02 00 00           	je	0x14157b815 <.text+0x157a815>
14157b538: 49 8d 5c 24 60              	leaq	0x60(%r12), %rbx
14157b53d: 48 89 9c 24 90 00 00 00     	movq	%rbx, 0x90(%rsp)
14157b545: 45 8b 74 24 40              	movl	0x40(%r12), %r14d
14157b54a: 41 83 fe 01                 	cmpl	$0x1, %r14d
14157b54e: 75 1b                       	jne	0x14157b56b <.text+0x157a56b>
14157b550: 41 0f 10 44 24 44           	movups	0x44(%r12), %xmm0
14157b556: 0f 29 84 24 f0 00 00 00     	movaps	%xmm0, 0xf0(%rsp)
14157b55e: 41 bf 01 00 00 00           	movl	$0x1, %r15d
14157b564: b8 18 00 00 00              	movl	$0x18, %eax
14157b569: eb 08                       	jmp	0x14157b573 <.text+0x157a573>
14157b56b: 45 31 ff                    	xorl	%r15d, %r15d
14157b56e: b8 08 00 00 00              	movl	$0x8, %eax
14157b573: 45 85 f6                    	testl	%r14d, %r14d
14157b576: 41 0f 94 c0                 	sete	%r8b
14157b57a: 41 c0 e0 03                 	shlb	$0x3, %r8b
14157b57e: 41 80 c0 2e                 	addb	$0x2e, %r8b
14157b582: 48 8b 34 01                 	movq	(%rcx,%rax), %rsi
14157b586: 8b 2e                       	movl	(%rsi), %ebp
14157b588: 48 89 f9                    	movq	%rdi, %rcx
14157b58b: 89 ea                       	movl	%ebp, %edx
14157b58d: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14157b593: e8 f8 14 46 ff              	callq	0x1409dca90 <.text+0x9dba90>
14157b598: c7 84 24 b4 00 00 00 00 00 00 00    	movl	$0x0, 0xb4(%rsp)
14157b5a3: c7 84 24 bc 00 00 00 00 00 00 00    	movl	$0x0, 0xbc(%rsp)
14157b5ae: c7 84 24 c4 00 00 00 00 00 00 00    	movl	$0x0, 0xc4(%rsp)
14157b5b9: c7 84 24 cc 00 00 00 00 00 00 00    	movl	$0x0, 0xcc(%rsp)
14157b5c4: c7 84 24 d4 00 00 00 00 00 00 00    	movl	$0x0, 0xd4(%rsp)
14157b5cf: c7 84 24 dc 00 00 00 00 00 00 00    	movl	$0x0, 0xdc(%rsp)
14157b5da: 48 c7 84 24 e4 00 00 00 00 00 00 00 	movq	$0x0, 0xe4(%rsp)
14157b5e6: c6 84 24 ec 00 00 00 00     	movb	$0x0, 0xec(%rsp)
14157b5ee: 48 8d 4c 24 2c              	leaq	0x2c(%rsp), %rcx
14157b5f3: 48 8d 94 24 b4 00 00 00     	leaq	0xb4(%rsp), %rdx
14157b5fb: 41 b8 01 00 00 00           	movl	$0x1, %r8d
14157b601: 41 b9 01 00 00 00           	movl	$0x1, %r9d
14157b607: e8 14 a3 65 ff              	callq	0x140bd5920 <.text+0xbd4920>
14157b60c: 8b 46 0c                    	movl	0xc(%rsi), %eax
14157b60f: 0f b6 4e 21                 	movzbl	0x21(%rsi), %ecx
14157b613: 89 6c 24 7c                 	movl	%ebp, 0x7c(%rsp)
14157b617: 89 84 24 80 00 00 00        	movl	%eax, 0x80(%rsp)
14157b61e: 88 8c 24 88 00 00 00        	movb	%cl, 0x88(%rsp)
14157b625: 44 89 7c 24 68              	movl	%r15d, 0x68(%rsp)
14157b62a: 0f 28 84 24 f0 00 00 00     	movaps	0xf0(%rsp), %xmm0
14157b632: 48 8d 44 24 6c              	leaq	0x6c(%rsp), %rax
14157b637: 0f 11 00                    	movups	%xmm0, (%rax)
14157b63a: 44 89 b4 24 84 00 00 00     	movl	%r14d, 0x84(%rsp)
14157b642: 83 7c 24 2c 02              	cmpl	$0x2, 0x2c(%rsp)
14157b647: 4c 8b bc 24 00 01 00 00     	movq	0x100(%rsp), %r15
14157b64f: 0f 84 c0 01 00 00           	je	0x14157b815 <.text+0x157a815>
14157b655: 48 8d 84 24 d0 01 00 00     	leaq	0x1d0(%rsp), %rax
14157b65d: 8b 4c 24 7c                 	movl	0x7c(%rsp), %ecx
14157b661: 89 48 50                    	movl	%ecx, 0x50(%rax)
14157b664: 8b 8c 24 80 00 00 00        	movl	0x80(%rsp), %ecx
14157b66b: 89 48 54                    	movl	%ecx, 0x54(%rax)
14157b66e: 8b 8c 24 84 00 00 00        	movl	0x84(%rsp), %ecx
14157b675: 89 48 58                    	movl	%ecx, 0x58(%rax)
14157b678: 0f b6 8c 24 88 00 00 00     	movzbl	0x88(%rsp), %ecx
14157b680: 88 48 5c                    	movb	%cl, 0x5c(%rax)
14157b683: 0f b7 8c 24 89 00 00 00     	movzwl	0x89(%rsp), %ecx
14157b68b: 66 89 48 5d                 	movw	%cx, 0x5d(%rax)
14157b68f: 0f b6 8c 24 8b 00 00 00     	movzbl	0x8b(%rsp), %ecx
14157b697: 88 48 5f                    	movb	%cl, 0x5f(%rax)
14157b69a: 0f 10 44 24 6c              	movups	0x6c(%rsp), %xmm0
14157b69f: 0f 11 40 40                 	movups	%xmm0, 0x40(%rax)
14157b6a3: 0f 10 44 24 2c              	movups	0x2c(%rsp), %xmm0
14157b6a8: 0f 10 4c 24 3c              	movups	0x3c(%rsp), %xmm1
14157b6ad: 0f 10 54 24 4c              	movups	0x4c(%rsp), %xmm2
14157b6b2: 0f 10 5c 24 5c              	movups	0x5c(%rsp), %xmm3
14157b6b7: 0f 11 58 30                 	movups	%xmm3, 0x30(%rax)
14157b6bb: 0f 11 50 20                 	movups	%xmm2, 0x20(%rax)
14157b6bf: 0f 11 48 10                 	movups	%xmm1, 0x10(%rax)
14157b6c3: 0f 11 00                    	movups	%xmm0, (%rax)
14157b6c6: 41 bd 03 00 00 00           	movl	$0x3, %r13d
14157b6cc: 48 3b 5c 24 20              	cmpq	0x20(%rsp), %rbx
14157b6d1: 0f 84 3e 01 00 00           	je	0x14157b815 <.text+0x157a815>
14157b6d7: 48 8d bc 24 a0 00 00 00     	leaq	0xa0(%rsp), %rdi
14157b6df: 4d 8d b4 24 80 00 00 00     	leaq	0x80(%r12), %r14
14157b6e7: 4c 89 b4 24 90 00 00 00     	movq	%r14, 0x90(%rsp)
14157b6ef: 48 8d 4c 24 2c              	leaq	0x2c(%rsp), %rcx
14157b6f4: 48 89 fa                    	movq	%rdi, %rdx
14157b6f7: 49 89 d8                    	movq	%rbx, %r8
14157b6fa: e8 01 92 5e ff              	callq	0x140b64900 <.text+0xb63900>
14157b6ff: 83 7c 24 2c 02              	cmpl	$0x2, 0x2c(%rsp)
14157b704: 0f 84 0b 01 00 00           	je	0x14157b815 <.text+0x157a815>
14157b70a: 0f 10 44 24 7c              	movups	0x7c(%rsp), %xmm0
14157b70f: 0f 29 84 24 80 02 00 00     	movaps	%xmm0, 0x280(%rsp)
14157b717: 0f 10 44 24 6c              	movups	0x6c(%rsp), %xmm0
14157b71c: 0f 29 84 24 70 02 00 00     	movaps	%xmm0, 0x270(%rsp)
14157b724: 0f 10 44 24 2c              	movups	0x2c(%rsp), %xmm0
14157b729: 0f 10 4c 24 3c              	movups	0x3c(%rsp), %xmm1
14157b72e: 0f 10 54 24 4c              	movups	0x4c(%rsp), %xmm2
14157b733: 0f 10 5c 24 5c              	movups	0x5c(%rsp), %xmm3
14157b738: 0f 29 9c 24 60 02 00 00     	movaps	%xmm3, 0x260(%rsp)
14157b740: 0f 29 94 24 50 02 00 00     	movaps	%xmm2, 0x250(%rsp)
14157b748: 0f 29 8c 24 40 02 00 00     	movaps	%xmm1, 0x240(%rsp)
14157b750: 0f 29 84 24 30 02 00 00     	movaps	%xmm0, 0x230(%rsp)
14157b758: 41 bd 04 00 00 00           	movl	$0x4, %r13d
14157b75e: 4c 3b 74 24 20              	cmpq	0x20(%rsp), %r14
14157b763: 0f 84 ac 00 00 00           	je	0x14157b815 <.text+0x157a815>
14157b769: 49 8d 9c 24 a0 00 00 00     	leaq	0xa0(%r12), %rbx
14157b771: 48 89 9c 24 90 00 00 00     	movq	%rbx, 0x90(%rsp)
14157b779: 48 8d 4c 24 2c              	leaq	0x2c(%rsp), %rcx
14157b77e: 48 89 fa                    	movq	%rdi, %rdx
14157b781: 4d 89 f0                    	movq	%r14, %r8
14157b784: e8 77 91 5e ff              	callq	0x140b64900 <.text+0xb63900>
14157b789: 83 7c 24 2c 02              	cmpl	$0x2, 0x2c(%rsp)
14157b78e: 0f 84 81 00 00 00           	je	0x14157b815 <.text+0x157a815>
14157b794: 0f 10 44 24 7c              	movups	0x7c(%rsp), %xmm0
14157b799: 0f 29 84 24 e0 02 00 00     	movaps	%xmm0, 0x2e0(%rsp)
14157b7a1: 0f 10 44 24 6c              	movups	0x6c(%rsp), %xmm0
14157b7a6: 0f 29 84 24 d0 02 00 00     	movaps	%xmm0, 0x2d0(%rsp)
14157b7ae: 0f 10 44 24 2c              	movups	0x2c(%rsp), %xmm0
14157b7b3: 0f 10 4c 24 3c              	movups	0x3c(%rsp), %xmm1
14157b7b8: 0f 10 54 24 4c              	movups	0x4c(%rsp), %xmm2
14157b7bd: 0f 10 5c 24 5c              	movups	0x5c(%rsp), %xmm3
14157b7c2: 0f 29 9c 24 c0 02 00 00     	movaps	%xmm3, 0x2c0(%rsp)
14157b7ca: 0f 29 94 24 b0 02 00 00     	movaps	%xmm2, 0x2b0(%rsp)
14157b7d2: 0f 29 8c 24 a0 02 00 00     	movaps	%xmm1, 0x2a0(%rsp)
14157b7da: 0f 29 84 24 90 02 00 00     	movaps	%xmm0, 0x290(%rsp)
14157b7e2: 41 bd 05 00 00 00           	movl	$0x5, %r13d
14157b7e8: 48 3b 5c 24 20              	cmpq	0x20(%rsp), %rbx
14157b7ed: 74 26                       	je	0x14157b815 <.text+0x157a815>
14157b7ef: 49 81 c4 c0 00 00 00        	addq	$0xc0, %r12
14157b7f6: 4c 89 a4 24 90 00 00 00     	movq	%r12, 0x90(%rsp)
14157b7fe: 48 8d 4c 24 2c              	leaq	0x2c(%rsp), %rcx
14157b803: 48 89 fa                    	movq	%rdi, %rdx
14157b806: 49 89 d8                    	movq	%rbx, %r8
14157b809: e8 f2 90 5e ff              	callq	0x140b64900 <.text+0xb63900>
14157b80e: 83 7c 24 2c 02              	cmpl	$0x2, 0x2c(%rsp)
14157b813: 75 35                       	jne	0x14157b84a <.text+0x157a84a>
14157b815: 44 89 ac 24 f0 02 00 00     	movl	%r13d, 0x2f0(%rsp)
14157b81d: 48 8d 94 24 10 01 00 00     	leaq	0x110(%rsp), %rdx
14157b825: 41 b8 e4 01 00 00           	movl	$0x1e4, %r8d            # imm = 0x1E4
14157b82b: 4c 89 f9                    	movq	%r15, %rcx
14157b82e: e8 cd 83 3a 01              	callq	0x142923c00 <.text+0x2922c00>
14157b833: 4c 89 f8                    	movq	%r15, %rax
14157b836: 48 81 c4 f8 02 00 00        	addq	$0x2f8, %rsp            # imm = 0x2F8
14157b83d: 5b                          	popq	%rbx
14157b83e: 5d                          	popq	%rbp
14157b83f: 5f                          	popq	%rdi
14157b840: 5e                          	popq	%rsi
14157b841: 41 5c                       	popq	%r12
14157b843: 41 5d                       	popq	%r13
14157b845: 41 5e                       	popq	%r14
14157b847: 41 5f                       	popq	%r15
14157b849: c3                          	retq
14157b84a: 48 8d b4 24 f0 02 00 00     	leaq	0x2f0(%rsp), %rsi
14157b852: 48 8d 0d ff ba 6f 01        	leaq	0x16fbaff(%rip), %rcx   # 0x142c77358
14157b859: e8 32 e8 37 01              	callq	0x1428fa090 <.text+0x28f9090>
14157b85e: 0f 10 44 24 7c              	movups	0x7c(%rsp), %xmm0
14157b863: 0f 11 46 50                 	movups	%xmm0, 0x50(%rsi)
14157b867: 0f 10 44 24 6c              	movups	0x6c(%rsp), %xmm0
14157b86c: 0f 11 46 40                 	movups	%xmm0, 0x40(%rsi)
14157b870: 0f 10 44 24 2c              	movups	0x2c(%rsp), %xmm0
14157b875: 0f 10 4c 24 3c              	movups	0x3c(%rsp), %xmm1
14157b87a: 0f 10 54 24 4c              	movups	0x4c(%rsp), %xmm2
14157b87f: 0f 10 5c 24 5c              	movups	0x5c(%rsp), %xmm3
14157b884: 0f 11 5e 30                 	movups	%xmm3, 0x30(%rsi)
14157b888: 0f 11 56 20                 	movups	%xmm2, 0x20(%rsi)
14157b88c: 0f 11 4e 10                 	movups	%xmm1, 0x10(%rsi)
14157b890: 0f 11 06                    	movups	%xmm0, (%rsi)
14157b893: 4c 3b 64 24 20              	cmpq	0x20(%rsp), %r12
14157b898: 0f 84 86 00 00 00           	je	0x14157b924 <.text+0x157a924>
14157b89e: 41 bd 06 00 00 00           	movl	$0x6, %r13d
14157b8a4: 4c 8d b4 24 50 03 00 00     	leaq	0x350(%rsp), %r14
14157b8ac: 48 8d 5c 24 2c              	leaq	0x2c(%rsp), %rbx
14157b8b1: 4d 89 e0                    	movq	%r12, %r8
14157b8b4: 49 83 c4 20                 	addq	$0x20, %r12
14157b8b8: 4c 89 a4 24 90 00 00 00     	movq	%r12, 0x90(%rsp)
14157b8c0: 48 89 d9                    	movq	%rbx, %rcx
14157b8c3: 48 89 fa                    	movq	%rdi, %rdx
14157b8c6: e8 35 90 5e ff              	callq	0x140b64900 <.text+0xb63900>
14157b8cb: 83 7c 24 2c 02              	cmpl	$0x2, 0x2c(%rsp)
14157b8d0: 0f 84 3f ff ff ff           	je	0x14157b815 <.text+0x157a815>
14157b8d6: 0f 10 44 24 7c              	movups	0x7c(%rsp), %xmm0
14157b8db: 41 0f 11 46 50              	movups	%xmm0, 0x50(%r14)
14157b8e0: 0f 10 44 24 6c              	movups	0x6c(%rsp), %xmm0
14157b8e5: 41 0f 11 46 40              	movups	%xmm0, 0x40(%r14)
14157b8ea: 0f 10 44 24 2c              	movups	0x2c(%rsp), %xmm0
14157b8ef: 0f 10 4c 24 3c              	movups	0x3c(%rsp), %xmm1
14157b8f4: 0f 10 54 24 4c              	movups	0x4c(%rsp), %xmm2
14157b8f9: 0f 10 5c 24 5c              	movups	0x5c(%rsp), %xmm3
14157b8fe: 41 0f 11 5e 30              	movups	%xmm3, 0x30(%r14)
14157b903: 41 0f 11 56 20              	movups	%xmm2, 0x20(%r14)
14157b908: 41 0f 11 4e 10              	movups	%xmm1, 0x10(%r14)
14157b90d: 41 0f 11 06                 	movups	%xmm0, (%r14)
14157b911: 49 ff c5                    	incq	%r13
14157b914: 49 83 c6 60                 	addq	$0x60, %r14
14157b918: 4c 3b 64 24 20              	cmpq	0x20(%rsp), %r12
14157b91d: 75 92                       	jne	0x14157b8b1 <.text+0x157a8b1>
14157b91f: e9 f1 fe ff ff              	jmp	0x14157b815 <.text+0x157a815>
14157b924: 41 bd 06 00 00 00           	movl	$0x6, %r13d
14157b92a: e9 e6 fe ff ff              	jmp	0x14157b815 <.text+0x157a815>
